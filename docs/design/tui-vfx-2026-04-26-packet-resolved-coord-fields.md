<!-- <FILE>docs/design/tui-vfx-2026-04-26-packet-resolved-coord-fields.md</FILE> - <DESC>Junior-ready execution packet for the first §5 follow-on move from the accepted Model B effect-composition decision: add resolved_x / resolved_y to VfxCellContext so downstream stages can react to per-cell sampler displacement. Smallest demonstration that the Phase F bundle is the right place to grow per-cell state.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>2026-04-26 evening: §8.1 effect-composition decision accepted (Model B locked); this packet captures the smallest of the three §5 follow-on moves so it's ready to execute the moment 1.2.A's Phase 6 verification clears tui-vfx-types.</WCTX> -->
<!-- <CLOG>1.0.0: initial packet — four open architectural questions resolved with recommended defaults, sampler-trait return-shape change, file-by-file plan, TDD red→green, acceptance criteria.</CLOG> -->

# Packet — Resolved-coord fields on `VfxCellContext`

## 1. Goal & motivation

The Phase F bundle (`VfxCellContext`) carries seven per-cell fields shared across `Filter` / `Mask` / `Sampler` / `StyleShader`: `local_x`, `local_y`, `width`, `height`, `screen_x`, `screen_y`, `t`. The Sampler stage can displace where in the source it reads per cell — that's how ripple, wave, and swimming-text effects work. After a sampler runs, downstream stages still see only `(local_x, local_y)`; they cannot tell whether the cell was displaced or by how much. Three real downstream effect classes are blocked by this gap:

- A shader that fades brightness based on the magnitude of sampler displacement (a ripple's brightness falls off from the wave crest).
- A mask that follows the displacement (a "wave-only" mask that hides cells away from the displacement source).
- A filter that intensifies near the original sampling source (post-process effects keyed to the un-displaced position).

**The fix.** Add `resolved_x` and `resolved_y` fields to `VfxCellContext`. The Sampler stage writes them; downstream stages read them. When no sampler ran (or a sampler chose not to displace), the values default to `local_x` / `local_y`, so existing recipes are unchanged.

This is the **first** of three §5 follow-on moves accepted in `tui-vfx-effect-composition-model.md` v0.2.0 §10. It earns its place by:
1. Closing a real gap (the three effect classes above).
2. Demonstrating that the Phase F bundle is the right structural place to grow per-stage output state — the same pattern unblocks the filter-discard bit (next slice).
3. Doing so without any trait-signature breakage (per Q4 below).

## 2. Scope

**In scope.**
- Add two fields to `VfxCellContext` in `crates/tui-vfx-types/src/cls_vfx_cell_context.rs`.
- Decide and implement the writeback mechanism per Q4 below.
- Update the `Sampler` trait return shape to communicate displacement.
- Update the compositor orchestrator to thread updated context to downstream stages.
- Update all 11 Sampler impls to opt into the new return shape (most return "no displacement" trivially).
- Add peer test coverage; update existing tests that construct contexts via the canonical constructor.
- Rustdoc on new fields, accessors, and the Sampler return type.

**Out of scope.**
- Filter-discard bit (separate §8.3 follow-on packet).
- Composite-effect templates in V3 schema (separate §8.3 follow-on packet).
- Recipe-level exposure of resolved coords. This is a runtime contract between stages; recipe authors do not configure it.
- Observer-bus event emission for displacement (a `CellTransformed { cause: SamplerOutput { delta } }` event would be a natural follow-on once the observability bus reaches its event-emission phase, but it's not required for this packet's acceptance).

## 3. Pre-work checklist

```bash
# Confirm graph health
ofpf-status

# Confirm 1.2.A Phase 6 verification has landed and tui-vfx-types is settled
git -C /usr/projects/tui-vfx log --oneline -10 -- crates/tui-vfx-types/

# Inspect the current bundle
ofpf-inspect crates/tui-vfx-types/src/cls_vfx_cell_context.rs

# Find the Sampler trait + impls
ofpf-defs Sampler
ofpf-inspect crates/tui-vfx-compositor/src/samplers/  # adjust path per ofpf-defs output

# Find the compositor orchestrator that runs sampler→mask→shader→filter in order
ofpf-defs render_pipeline
ofpf-inspect crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs  # path TBD

# Audit dependents on VfxCellContext (Phase F made this hub)
ofpf-blast crates/tui-vfx-types/src/cls_vfx_cell_context.rs

# Read the accepted decision the packet executes against
cat docs/design/tui-vfx-effect-composition-model.md  # §10 + §5
```

## 4. Current-state audit

Executor MUST run `ofpf-inspect` on each file before editing and capture output here. The packet's pre-resolved values below are the working assumptions that drove the design; if `ofpf-inspect` reveals different shape, stop and re-derive Q1–Q4 against the actual surface.

| File | Working assumption (verify with ofpf-inspect) |
|---|---|
| `crates/tui-vfx-types/src/cls_vfx_cell_context.rs` | 164 LOC, 7 fields, `Copy` + `new()` constructor, four accessor methods, peer test module. Adding 2 fields takes it to ~190 LOC, still under the `cls_` 200 hard limit. |
| Sampler trait file | TBD — find via `ofpf-defs Sampler`. Phase F.4 migrated 11 impls to take `&VfxCellContext`; verify the trait method signature and return type. |
| 11 Sampler impls | Per Phase F.4 commit `b628abd`. Most are "non-displacing" (color filters dressed as samplers); only ~3-4 actually displace. Find them with `ofpf-content "fn sample"` against `tui-vfx-style/src/models/` and `tui-vfx-compositor/src/samplers/`. |
| Compositor orchestrator | TBD. Run `ofpf-defs render_pipeline` to find the function that calls Sampler → Mask → Shader → Filter in order with the `VfxCellContext` bundle threaded through. The orchestrator is the only place that needs to change to thread the updated bundle to downstream stages. |

## 5. Open architectural questions — resolved with defaults

Each question carries a recommended default and a one-line rationale. The executor applies the defaults unless the audit in §4 reveals a contradicting fact.

### Q1 — Field type: signed `i32` or unsigned `u16`?

**Recommended default:** `i32`.

**Rationale.** Samplers can compute displacements that conceptually point outside the layer (read from "above" row 0, or from a tiled source). `u16` would require samplers to clamp/saturate at write time, losing intent — a downstream shader that wants "displacement magnitude" can't tell saturation from a real edge case. `i32` lets samplers express the truthful resolved coordinate; readers clamp at use time if they need a bounded index. Cost: 4 bytes per coord vs 2; 8 bytes total per bundle, well under any cache-line or hot-path budget.

### Q2 — Default semantics: `Option<i32>` or always-set defaulting to local?

**Recommended default:** always-set, defaulted to `local_x as i32` / `local_y as i32` at bundle construction.

**Rationale.** `Option` distinguishes "no sampler ran" from "sampler ran and chose not to displace." The distinction is behaviorally indistinguishable for any monotonic distance-based read — a shader computing `|resolved - local|` produces 0 in both cases. `Option` adds a discriminant byte and forces every reader through `unwrap_or(local)`. Always-set is simpler and the default carries the right semantics for free. If a future use case genuinely needs "did a sampler ever run on this cell," that's a separate event-stream concern (observability bus territory), not bundle state.

### Q3 — Multi-sampler chains: accumulator or independent?

**Recommended default:** accumulator. Sampler N reads `resolved_x` / `resolved_y` from the bundle (which carries sampler N-1's output, or local if N=0) and writes back its own delta added to the prior resolved value.

**Rationale.** The compositor's actual source-read happens once, downstream of all samplers. Conceptually only the final resolved coordinate matters for the read; intermediate sampler outputs are inputs to the next sampler. The accumulator semantics give downstream readers the truthful "total displacement" they want. Independent semantics (each sampler computes from local) would mean multi-sampler chains could see only the LAST sampler's contribution, hiding the rest — useless for the three motivating use cases.

Concrete example: a wave sampler displaces y by `+sin(x)`; a vertical-scroll sampler displaces y by `+scroll_offset`. With accumulator, downstream sees `resolved_y = local_y + sin(x) + scroll_offset` — true total displacement. With independent, downstream sees only `+scroll_offset` (or only `+sin(x)`, depending on order) — false.

### Q4 — Mutation mechanism: `&mut` bundle, return new bundle, or side-channel?

**Recommended default:** Sampler trait returns a `SamplerOutput` struct that includes the resolved-coord delta. Orchestrator constructs the next-stage bundle by combining the prior bundle with the sampler's output. Bundle stays `Copy` and immutable at trait surfaces.

**Rationale.** Three options were considered:

- **A: Change Phase F's trait surfaces from `&VfxCellContext` to `&mut VfxCellContext`.** Sampler writes resolved into the bundle; downstream sees updated bundle. Breaks Phase F's just-shipped immutable contract that was a deliberate design choice per the F.5 commit message (commits `b628abd`, `5535b0e`, `7821815`). Touches 30 Filter + 11 Mask + 11 Sampler + N Shader call sites that just settled. Rejected.

- **B: Sampler returns a new `VfxCellContext` bundle that downstream stages receive.** Conceptually clean. Forces the orchestrator to construct a new bundle per sampler application — Copy type so allocation-friendly, but the orchestrator owns the construction. Bundle stays immutable at trait surfaces. Acceptable, but the return type forces every Sampler to return a full bundle (most don't change anything).

- **C (recommended): Sampler returns a `SamplerOutput` struct carrying just the per-stage outputs (initially: resolved-coord delta).** Orchestrator combines the input bundle with the sampler's output to build the next-stage bundle. Bundle stays `Copy` + immutable at trait surfaces. Future per-stage outputs (filter-discard bit next slice; whatever comes after) extend `SamplerOutput` / `FilterOutput` rather than the bundle. The bundle carries shared per-cell state; output structs carry stage-specific deltas. Clear separation.

Option C honors Phase F's intention (bundle is the cross-stage shared state) AND opens a clean path for the next slice (FilterOutput grows the discard bit). The cost is one new public struct per stage that has outputs; today only Sampler needs one.

## 6. Step-by-step implementation plan

File-centric per OFPF. TDD red→green per file.

### Phase 1 — Extend `VfxCellContext`

**File:** `crates/tui-vfx-types/src/cls_vfx_cell_context.rs`

Bump `<VERS>` to `1.1.0` (additive field bump). Update `<WCTX>` and `<CLOG>` per the OFPF metadata convention.

Add fields after `t`:

```rust
pub struct VfxCellContext {
    pub local_x: u16,
    pub local_y: u16,
    pub width: u16,
    pub height: u16,
    pub screen_x: u16,
    pub screen_y: u16,
    pub t: f64,

    /// Sampler-resolved x coordinate (i32 to allow negative offsets).
    /// Defaults to `local_x as i32` at construction. Samplers in a chain
    /// accumulate displacement: each sampler writes `prior_resolved_x + its_delta`.
    /// Downstream stages (mask, shader, filter) read this to react to displacement.
    pub resolved_x: i32,

    /// Sampler-resolved y coordinate (i32 to allow negative offsets).
    /// Same accumulator semantics as `resolved_x`.
    pub resolved_y: i32,
}
```

Update `new()` to default `resolved_x` and `resolved_y` from `local_x` / `local_y`:

```rust
#[inline]
pub fn new(
    local_x: u16, local_y: u16,
    width: u16, height: u16,
    screen_x: u16, screen_y: u16,
    t: f64,
) -> Self {
    Self {
        local_x, local_y, width, height, screen_x, screen_y, t,
        resolved_x: local_x as i32,
        resolved_y: local_y as i32,
    }
}
```

Add a builder for downstream bundle construction (orchestrator uses this):

```rust
/// Construct a downstream context by applying a sampler's resolved-coord
/// delta to a prior context. All other fields preserved verbatim.
#[inline]
pub fn with_sampler_resolution(self, delta_x: i32, delta_y: i32) -> Self {
    Self {
        resolved_x: self.resolved_x.saturating_add(delta_x),
        resolved_y: self.resolved_y.saturating_add(delta_y),
        ..self
    }
}

/// Displacement vector (resolved minus local). Useful for distance-based
/// shaders / masks / filters.
#[inline]
pub fn displacement(&self) -> (i32, i32) {
    (
        self.resolved_x.saturating_sub(self.local_x as i32),
        self.resolved_y.saturating_sub(self.local_y as i32),
    )
}

/// Magnitude of displacement (Euclidean). Convenience for brightness fades.
#[inline]
pub fn displacement_magnitude(&self) -> f32 {
    let (dx, dy) = self.displacement();
    ((dx * dx + dy * dy) as f32).sqrt()
}
```

Update `test_default()` if needed (the existing one calls `new()` with all zeros, which now correctly defaults resolved to `(0, 0)` — no change required).

Add tests in the existing `mod tests` block (TDD: write these before adding the fields):

```rust
#[test]
fn resolved_defaults_to_local_at_construction() {
    let ctx = VfxCellContext::new(3, 5, 10, 10, 0, 0, 0.0);
    assert_eq!(ctx.resolved_x, 3);
    assert_eq!(ctx.resolved_y, 5);
    assert_eq!(ctx.displacement(), (0, 0));
    assert_eq!(ctx.displacement_magnitude(), 0.0);
}

#[test]
fn with_sampler_resolution_accumulates() {
    let ctx = VfxCellContext::new(3, 5, 10, 10, 0, 0, 0.0)
        .with_sampler_resolution(2, -1)
        .with_sampler_resolution(1, 4);
    assert_eq!(ctx.resolved_x, 6);  // 3 + 2 + 1
    assert_eq!(ctx.resolved_y, 8);  // 5 + (-1) + 4
    assert_eq!(ctx.displacement(), (3, 3));
}

#[test]
fn displacement_magnitude_is_euclidean() {
    let ctx = VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0)
        .with_sampler_resolution(3, 4);
    assert!((ctx.displacement_magnitude() - 5.0).abs() < f32::EPSILON);
}

#[test]
fn with_sampler_resolution_saturates_at_i32_bounds() {
    let ctx = VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0)
        .with_sampler_resolution(i32::MAX, i32::MAX)
        .with_sampler_resolution(1, 1);
    assert_eq!(ctx.resolved_x, i32::MAX);  // saturating
    assert_eq!(ctx.resolved_y, i32::MAX);
}

#[test]
fn negative_resolved_coords_supported() {
    let ctx = VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0)
        .with_sampler_resolution(-3, -7);
    assert_eq!(ctx.resolved_x, -3);
    assert_eq!(ctx.resolved_y, -7);
}
```

Run `cargo test -p tui-vfx-types` — RED then GREEN.

### Phase 2 — Add `SamplerOutput` to the Sampler trait

**File:** location TBD — find via `ofpf-defs Sampler` in pre-work. Likely `crates/tui-vfx-compositor/src/samplers/mod.rs` or `crates/tui-vfx-compositor/src/traits.rs`.

Define the output struct in the same module as the trait:

```rust
/// Per-cell output from a Sampler stage application.
///
/// At present this carries only the resolved-coord delta. Future per-stage
/// outputs (e.g., a "swap source layer" hint) extend this struct rather
/// than the cross-stage `VfxCellContext` bundle.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SamplerOutput {
    /// Sampler's contribution to the resolved-coord chain. Added to the
    /// prior `VfxCellContext::resolved_x` by the orchestrator before the
    /// downstream stage runs.
    pub delta_x: i32,
    /// Same for y.
    pub delta_y: i32,
}

impl SamplerOutput {
    /// No-displacement output. Use for color-filter samplers and any
    /// sampler that does not move the source-read coordinate.
    #[inline]
    pub fn no_displacement() -> Self {
        Self { delta_x: 0, delta_y: 0 }
    }

    /// Sampler displaces the read coordinate by `(delta_x, delta_y)`.
    #[inline]
    pub fn displaced(delta_x: i32, delta_y: i32) -> Self {
        Self { delta_x, delta_y }
    }
}
```

Change the Sampler trait method to return `SamplerOutput`. Audit the existing return type first; the working assumption is the trait returns `()` or a `Cell` today. If it returns `Cell`, change to `(Cell, SamplerOutput)` or fold both into a richer return struct — pick whichever minimizes call-site churn during the audit.

Bump trait file `<VERS>` to a MINOR if the change is additive (e.g., the existing return becomes part of a richer struct) or MAJOR if it's a breaking shape change.

### Phase 3 — Update the 11 Sampler impls

Per Phase F.4 commit `b628abd`. Most samplers are non-displacing (color filters dressed as samplers); they return `SamplerOutput::no_displacement()`. Identify the displacing ones via `ofpf-content "fn sample"` against the sampler models and update each to return `SamplerOutput::displaced(dx, dy)` from the displacement they already compute internally.

For each sampler file:
1. Bump `<VERS>` (PATCH if non-displacing, MINOR if displacement is now exposed).
2. Add or update the peer test that asserts `SamplerOutput` shape for at least one input.
3. Run `cargo test -p tui-vfx-style` (or whichever crate hosts the impl).

Expected per-impl LOC change: ~5-10 lines.

### Phase 4 — Update the orchestrator

**File:** TBD — `ofpf-defs render_pipeline` finds it. The function runs the four stages per cell.

Where it calls the Sampler today, capture the `SamplerOutput`, then construct the next stage's bundle:

```rust
// Before sampler:
let mut ctx = VfxCellContext::new(local_x, local_y, w, h, sx, sy, t);

// Run sampler chain (could be 0..N samplers):
for sampler in sampler_chain {
    let output = sampler.sample(source_cell, &ctx, /* other args */);
    ctx = ctx.with_sampler_resolution(output.delta_x, output.delta_y);
}

// Downstream stages see the accumulated resolved coords:
mask.apply(&ctx, ...);
shader.apply(&ctx, ...);
filter.apply(&ctx, ...);
```

Bump orchestrator file `<VERS>` to MINOR. Add a peer test asserting the accumulator semantics: two displacing samplers in sequence produce `resolved = local + delta1 + delta2` in the downstream bundle.

### Phase 5 — Workspace verification

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo doc --no-deps -p tui-vfx-types -p tui-vfx-compositor
cargo xtask docs generate  # if VfxCellContext appears in the capability manifest
```

Any new clippy warning is a fail per `feedback_clean_build_no_warnings`. Pre-existing warnings count.

## 7. Test plan

| Test | Where | Asserts | Phase |
|---|---|---|---|
| `resolved_defaults_to_local_at_construction` | `cls_vfx_cell_context.rs` `mod tests` | new() initializes resolved = local | 1 |
| `with_sampler_resolution_accumulates` | same | accumulator semantics for chained calls | 1 |
| `displacement_magnitude_is_euclidean` | same | helper math | 1 |
| `with_sampler_resolution_saturates_at_i32_bounds` | same | no overflow panic | 1 |
| `negative_resolved_coords_supported` | same | i32 over u16 choice honored | 1 |
| `sampler_output_no_displacement_is_zero` | sampler trait file `mod tests` | builder sanity | 2 |
| `sampler_output_displaced_carries_deltas` | same | builder sanity | 2 |
| For each displacing Sampler impl: peer test asserts `SamplerOutput` shape for a known input | per-impl `test_*.rs` | impl correctness | 3 |
| `orchestrator_accumulates_resolved_across_sampler_chain` | orchestrator peer test | end-to-end: two samplers, downstream context shows accumulated resolved | 4 |
| `downstream_stages_see_resolved_when_sampler_displaces` | orchestrator peer test | mask / shader / filter receive `ctx.resolved_x != ctx.local_x` after a displacing sampler | 4 |

Phase F's existing tests for `VfxCellContext` (4 tests in the existing `mod tests`) continue to pass unchanged because the additions are additive.

Per-phase commands:

```bash
cargo test -p tui-vfx-types cls_vfx_cell_context              # Phase 1
cargo test -p tui-vfx-compositor samplers                     # Phase 2 + 3 (path TBD)
cargo test -p tui-vfx-style                                   # Phase 3 (sampler models)
cargo test -p tui-vfx-compositor pipeline                     # Phase 4 (orchestrator)
cargo test --workspace                                        # Phase 5
```

## 8. Acceptance criteria

Binary pass/fail. All must hold at commit time.

- [ ] `cargo build --workspace` clean.
- [ ] `cargo test --workspace` all green.
- [ ] `cargo clippy --workspace -- -D warnings` clean. No new `#[allow]` suppressions added (per `feedback_no_landmines`).
- [ ] No pre-existing warnings either (per `feedback_clean_build_no_warnings`).
- [ ] `VfxCellContext` carries `resolved_x: i32` and `resolved_y: i32` fields, defaulted to local at construction.
- [ ] `VfxCellContext::with_sampler_resolution(delta_x, delta_y)` builder ships with rustdoc and accumulator semantics test.
- [ ] `VfxCellContext::displacement()` and `displacement_magnitude()` accessors ship with rustdoc and tests.
- [ ] `SamplerOutput` struct ships with rustdoc, builders, and peer tests.
- [ ] Sampler trait return shape updated; all 11 impls migrated; each has a peer test asserting its `SamplerOutput`.
- [ ] Orchestrator threads `SamplerOutput` into downstream bundle via `with_sampler_resolution`; peer test asserts the accumulator end-to-end.
- [ ] No public API churn beyond the additive field bumps and the Sampler return-shape change (per Intention 23 rule 4: never breaking churn during consolidation; field additions and return-type extensions are additive).
- [ ] Rustdoc audited on every public item touched (per `feedback_rustdoc_when_editing`). New fields have doc comments. New methods have doc comments + at least one example in the rustdoc when non-trivial.
- [ ] Metadata envelopes updated on every touched file (DESC if role changed; VERS bumped per OFPF rules; WCTX one-line context; CLOG one-line note for this change).
- [ ] No parse-and-inert fields (per `feedback_no_inert_schema`). `resolved_x` / `resolved_y` are written by samplers AND read by at least one downstream stage's test.
- [ ] `cargo doc --no-deps -p tui-vfx-types -p tui-vfx-compositor` clean.
- [ ] `cargo xtask docs generate` clean if `VfxCellContext` appears in the capability manifest.

## 9. Verification commands

Copy-paste the executor runs at the end:

```bash
cd /usr/projects/tui-vfx
cargo build --workspace 2>&1 | tail -20
cargo test --workspace 2>&1 | tail -40
cargo clippy --workspace -- -D warnings 2>&1 | tail -20
cargo doc --no-deps -p tui-vfx-types -p tui-vfx-compositor 2>&1 | tail -10
cargo xtask docs generate 2>&1 | tail -10

# OFPF-side audit: verify no new warnings or dead code
ofpf-status
ofpf-inspect crates/tui-vfx-types/src/cls_vfx_cell_context.rs

# If observability bus has shipped by now: assert the resolved-coord write
# emits a CellTransformed { cause: SamplerOutput { delta } } event.
# If not, defer this assertion to the observability follow-up.
```

## 10. Rollback plan

If the audit in §4 reveals that the Sampler trait return shape cannot be extended additively (the trait signature is part of a heavily-used downstream API outside Phase F's just-touched surface), stop before Phase 2. The rollback is mechanical:

1. Revert any in-progress Phase 2 / 3 edits.
2. Keep Phase 1 (the `VfxCellContext` field additions are pure-additive and harmless even if no sampler ever writes to them).
3. Open an architectural decision: route the Sampler return through Option B (sampler returns a new bundle) instead of Option C, and re-derive Phases 2-4.

If the audit reveals that the orchestrator's per-cell sampler-call site is in a hot benchmark path, run the existing `bench_full_trace_60fps` criterion bench before and after Phase 4. The accumulator's `with_sampler_resolution` is a Copy + i32 add + saturating_add — sub-nanosecond per cell — but verify against the bench gate before signing off.

## 11. Risks & gotchas

- **Sampler trait return-shape change touches 11 impls.** Phase F.4 already proved this surface is well-understood; the migration follows the same pattern. The `SamplerOutput::no_displacement()` builder makes most impls a one-line change.
- **`i32` arithmetic on `u16` inputs needs `as` casts.** `resolved_x = local_x as i32` at construction is intentional. Downstream readers that want a `usize` index for source lookup need to clamp: `let src_x = ctx.resolved_x.clamp(0, ctx.width as i32 - 1) as usize`. Document this pattern in the rustdoc on `resolved_x`.
- **The `displacement()` accessor uses `saturating_sub` on `i32 - i32`.** This is fine for the i32 range (`i32::MIN..=i32::MAX`). The saturating call prevents panic on extreme inputs that an out-of-spec sampler might produce; in practice samplers stay within layer-rect bounds.
- **`feedback_no_inert_schema` enforcement.** The packet adds two public fields. They MUST be wired end-to-end before commit: at least one displacing Sampler impl writes them via the orchestrator's threading, and at least one downstream stage's test reads them and asserts behavior. Acceptance criterion §8 covers this; do not skip the assertion.
- **Bench-gate awareness.** `bench_full_trace_60fps` is the release-gate criterion bench (Intention release-gate). The accumulator adds two i32 writes per sampler-stage transition. At 80×24 cells × 4 layers × 1-2 samplers/layer × 60fps ≈ 0.6-1.2M extra writes/sec, well under any meaningful bench impact, but run the bench post-Phase-4 to confirm.
- **Multi-sampler order matters under accumulator semantics.** A wave-then-scroll chain produces `resolved = local + wave + scroll`; scroll-then-wave produces the same final number commutatively, but if a future sampler has order-sensitive output (e.g., a sampler that reads ITS OWN resolved coord to decide its delta), the order is load-bearing. None today have that property; flag in the rustdoc on `with_sampler_resolution` that "samplers are applied in pipeline order."

## 12. Sequencing note

**Hard gate:** wait until 1.2.A's Phase 6 verification has landed and `tui-vfx-types` is settled on master. Both this packet and 1.2.A touch the same crate; landing in parallel risks mechanical merge friction.

**Soft coordination with the observability bus** (parallel session, currently US-003 of US-010): when this packet lands, samplers that write resolved-coord deltas become candidates for `CellTransformed { cause: SamplerOutput { delta_x, delta_y } }` event emission. The observability spec v0.2.0 doesn't require this for its v0.1.0 acceptance, but the event variant is a natural follow-on and worth flagging to the observability author. No blocking dependency in either direction.

**Unblocks:** the second §8.3 follow-on slice (filter-discard bit) follows the same `FilterOutput` pattern this packet establishes for `SamplerOutput`. The third §8.3 follow-on (composite-effect templates in V3 schema) is independent of this work.

**Future use:** the eventual gt-design V3 stack migration (§8.5 packet, in flight as a parallel sub-agent) will need a recipe-author UX for "expose the resolved coord to a shader." That's V3 schema design, not this packet.

<!-- <FILE>docs/design/tui-vfx-2026-04-26-packet-resolved-coord-fields.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
