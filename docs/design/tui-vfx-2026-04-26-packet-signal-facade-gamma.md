<!-- <FILE>docs/design/tui-vfx-2026-04-26-packet-signal-facade-gamma.md</FILE> - <DESC>Implementation packet for signal-facade phase γ — build the `tui_vfx_recipes::signals` module that exposes a curated, normalized recipe-author signal surface in front of `mixed_signals::*`. Self-contained execution brief: pre-flight, current-state audit of every signal-deserialization site in the recipes crate, audit of the parallel motion-physics channel that the facade collapses, open architectural questions with recommended defaults, step-by-step plan with OFPF prefixes, code snippets for `VfxRecipeSignalSpec` shape, test plan, acceptance criteria, verification commands, rollback plan.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Capture phase γ of the signal-facade proposal as a junior-ready packet — Option A (in-crate module) accepted 2026-04-26; γ becomes executable now that 1.2.A `VfxBindable<T, S = Never>` shipped at commit 77d1636 and the symmetric Bindable family phase δ depends on is cheap.</WCTX> -->
<!-- <CLOG>1.0.0: initial packet — pre-flight, current-state audit (recipes-side signal sites, parallel motion-physics channel, mixed-signals public surface, signals.toml absence), four open questions with recommended defaults, four-phase implementation plan, code snippets for `VfxRecipeSignalSpec` enum + transparent-wrapper variant + per-family modules, test plan, acceptance criteria, verification commands, rollback plan, sequencing note.</CLOG> -->

# Packet — signal-facade phase γ (`tui_vfx_recipes::signals`)

> **Source proposal.** `docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md` v0.3.0, §8 and the **Decision (2026-04-26)** section at the end. Option A accepted: in-crate facade at `tui_vfx_recipes::signals::*`; production code keeps importing `mixed_signals::*` directly.
>
> **Status.** Queued. Phase γ unblocked because 1.2.A `VfxBindable<T, S = Never>` shipped at commit 77d1636 in `tui-vfx-core`, so the symmetric Bindable family that signal-facade phase δ depends on is now a one-line type alias per Bindable.
>
> **Sequencing.** Phases α and β (autogen `SIGNALS_REFERENCE.md` + curated Core 12 cheatsheet) are doc-only and green-lit. They can land before γ, in parallel with γ, or after — γ does not depend on them. γ does **not** unblock δ on its own; δ's Bindable signal-form variants point at `VfxRecipeSignalSpec`, so γ is δ's hard gate.
>
> **Risk tier.** M — new module subtree in `tui-vfx-recipes`, redirect of two existing signal-deserialization sites, collapse of one parallel motion-physics channel. No cross-crate ripple beyond recipes (mixed-signals untouched, production `tui-vfx` consumers untouched). No upstream changes to `mixed-signals`.

---

## Goal & motivation

The recipes crate today reaches `mixed-signals` through two structurally different channels:

1. **Direct `SignalSpec` deserialization** at `crates/tui-vfx-recipes/src/v3/compile/fnc_build_composition_spec_from_compiled_plan.rs:633` — `serde_json::from_value::<SignalSpec>(value.clone()).ok()?` followed by `.build()`. Every signal field in the recipe schema lands here.
2. **Parallel motion-physics channel** at `crates/tui-vfx-recipes/src/v3/compile/cls_v3_motion_envelope.rs:60` — `enum V3MotionDynamicSpec { Spring { ... }, Bounce { ... }, Pendulum { ... }, Projectile { ... }, Orbit { ... }, Friction { ... }, ... }` lowered via `lower_composed_path` into `PathType` variants. This is **not** a signal channel — it's a motion-spec channel — but it carries the same physics primitives (spring, bounce, pendulum, projectile, orbit) that the facade's catalog covers, and it is the second of "the four observed gaps" the proposal calls out.

A recipe author writing `{"signal": {"type": "spring", ...}}` today gets a deserialization error: `SignalSpec` does not contain a Spring variant. To get spring behavior the author must reach for `dynamics: [{"type": "spring", ...}]` inside a motion spec. The two routes work, but they require two mental models for one capability.

Phase γ builds `tui_vfx_recipes::signals` — a curated, normalized recipe-author surface that:

- Wraps the `SignalSpec` discriminants the recipes crate elects to expose (Core 12 + advanced).
- Adds physics variants that mixed-signals already implements but `SignalSpec` does not expose, so `{"signal": {"type": "spring", ...}}` deserializes successfully.
- Becomes the single public type that recipe-side deserialization paths target — `serde_json::from_value::<VfxRecipeSignalSpec>(...)` replaces `serde_json::from_value::<SignalSpec>(...)` at every recipe-side site.
- Hosts the autogen catalog phases α/β consume.

The facade is **recipe-deserialization-only**. Production code in `tui-vfx`, `tui-vfx-style`, `tui-vfx-compositor`, `tui-vfx-content`, and `gt-design` keeps importing `mixed_signals::*` directly. The facade is unreachable from any binary that doesn't already depend on `tui-vfx-recipes`.

The headline maintenance lever the facade provides (per the user's 2026-04-26 articulation): a locally-named, locally-scoped interface point to drive recipe inputs. Future swaps, plug-ins, exposure-limiting, and rename/remap stay in one place.

## Scope

**In scope.**

- New module subtree at `crates/tui-vfx-recipes/src/signals/` reachable as `tui_vfx_recipes::signals::*`. Exactly four public exports: `VfxRecipeSignalSpec`, `VfxIntoRecipeSignal`, `VfxRecipeSignalMeta`, `vfx_recipe_signal_catalog()`.
- One `VfxRecipeSignalSpec` enum with variants split across per-family modules (oscillators, physics, spatial, composition) for OFPF size compliance. The mod-root re-exports a single flat enum.
- Transparent-wrapper variants (`#[serde(transparent)]`) over `mixed_signals::*` types where the recipe wire shape exactly matches the upstream serde shape. Unit variants for nullary spatial leaves (`SampleNormX`, `SampleNormY`, `SampleRadius`).
- `VfxIntoRecipeSignal` trait converting a `VfxRecipeSignalSpec` into `Box<dyn mixed_signals::traits::Signal>` (the type the recipes machinery already consumes).
- Migration of the existing recipe-side signal-deserialization sites to use the facade. Two sites identified in §Current-state audit.
- Collapse of the parallel motion-physics channel: variants `Spring`, `Bounce`, `Pendulum`, `Projectile`, `Orbit`, `Decay`, `Attractor` land as facade variants. The recipe-author can write `{"signal": {"type": "spring", ...}}` and have it deserialize. **The existing `V3MotionDynamicSpec` route is preserved unchanged** — recipes that today use motion-spec dynamics keep working. The collapse is from the author's perspective (one mental model: "every signal-producing primitive is a `{"signal": {"type": "X"}}`"), not a physical removal of the motion-spec path.
- Catalog metadata (`VfxRecipeSignalMeta`) defined in code, iterable for the autogen target. One entry per discriminant.

**Out of scope.**

- Any change to production code's direct `mixed_signals::*` imports. Filters, masks, samplers, shaders continue importing `mixed_signals::Signal` and calling `.sample_with_context(&signal_ctx)`. They don't know the facade exists.
- Any change to `mixed-signals` upstream. The crate stays at its current public surface (Intention 9: extend mixed-signals when a new primitive is needed, not when reorganizing recipe-facing exposure).
- Any change to the recipe schema's field shapes. The wire format authors write today (e.g. `{"type": "ramp", "start": 0, "end": 8, "duration": 5.0}`) deserializes through the facade with the identical bytes. Backward compatibility is the load-bearing constraint.
- Symmetric `VfxBindableF32` / `VfxBindableColor` family with signal-form variants (signal-facade phase δ). That packet follows γ; it adds `Signal(VfxRecipeSignalSpec)` arms to `VfxBindableU16` / `VfxBindableString` and introduces the two new named Bindables. Not this packet.
- Any removal of the parallel `V3MotionDynamicSpec` channel. Removing it ripples through `lower_composed_path`, every recipe that uses motion-spec dynamics, and the runtime path-evaluation code. Out of scope for γ.
- Promotion to the proposal's Option B sub-crate (`tui-vfx-recipes-signals` under a workspace conversion). Stays on the table as a mechanical conversion if the module ever outgrows reasonable size.
- Phases α (`SIGNALS_REFERENCE.md` autogen) and β (Core 12 cheatsheet). Doc-only, green-lit, separately scoped.

**Crates touched.**

- **`tui-vfx-recipes`:** new `signals/` module subtree; two existing files updated to deserialize through `VfxRecipeSignalSpec` instead of `SignalSpec` directly.
- No other crate touched.

## Pre-work checklist

```bash
# Daemon health.
ofpf-status
ofpf-stats
ofpf-overview

# Load the recipes repo (this packet's primary work surface).
ofpf-load --root /usr/projects/tui-vfx-recipes

# Load mixed-signals (audit target — read-only). Path-verify first; the
# canonical location is /usr/projects/mixed-signals.
ls /usr/projects/mixed-signals/Cargo.toml
ofpf-load --root /usr/projects/mixed-signals

# Re-confirm the signals.toml authoring overlay does not yet exist
# (phase α is doc-only and unfinished).
find /usr/projects/tui-vfx/docs/templates -name "signals*" -print

# Read the source proposal §8 + Decision section.
sed -n '393,572p' /usr/projects/tui-vfx/docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md

# Read 1.2.A — the prerequisite that landed.
sed -n '1,40p' /usr/projects/tui-vfx/docs/design/tui-vfx-2026-04-26-packet-1.2.A-bindable-generic.md

# Confirm 1.2.A's VfxBindable lives in tui-vfx-core::bindable as expected.
ofpf-defs VfxBindable

# Inspect every recipe-side signal-deserialization site.
ofpf-inspect crates/tui-vfx-recipes/src/v3/compile/fnc_build_composition_spec_from_compiled_plan.rs
ofpf-inspect crates/tui-vfx-recipes/src/manager/fnc_populate_effects.rs
ofpf-inspect crates/tui-vfx-recipes/src/loopback/fnc_evaluate_loopback.rs

# Inspect the parallel motion-physics channel.
ofpf-inspect crates/tui-vfx-recipes/src/v3/compile/cls_v3_motion_envelope.rs

# Inspect mixed-signals' public surface (read-only).
ofpf-defs Signal --root /usr/projects/mixed-signals
ofpf-defs SignalSpec --root /usr/projects/mixed-signals
ofpf-defs SignalContext --root /usr/projects/mixed-signals
ofpf-extract /usr/projects/mixed-signals/src/types/signal_spec.rs SignalSpec

# Find every recipe-side use of SignalSpec (the migration surface).
ofpf-content "SignalSpec" --root /usr/projects/tui-vfx-recipes

# Find every recipe-side use of mixed_signals (broader audit).
ofpf-content "use mixed_signals" --root /usr/projects/tui-vfx-recipes

# Find recipe-side physics references (the parallel-channel inventory).
ofpf-content "physics\|spring\|bounce\|pendulum" --root /usr/projects/tui-vfx-recipes

# Confirm the recipes crate's prelude / public surface.
sed -n '1,80p' /usr/projects/tui-vfx-recipes/src/lib.rs
sed -n '1,50p' /usr/projects/tui-vfx-recipes/src/prelude.rs
```

If any `ofpf-*` command errors three times in a row, run `ofpf-bug` to file a librarian bug and continue with manual reads (`Read` / `grep`) — never abandon the audit.

## Current-state audit

Captured 2026-04-26 from the librarian.

### A. Recipe-side signal-deserialization sites

The two sites where the recipes crate hands JSON to `mixed_signals::SignalSpec`:

| Path | LOC | Role | Signal contact point |
|---|---|---|---|
| `crates/tui-vfx-recipes/src/v3/compile/fnc_build_composition_spec_from_compiled_plan.rs` | 1617 | unit (large; V3 compile pipeline) | line 633: `serde_json::from_value::<SignalSpec>(value.clone()).ok()?` inside `fn sample_signal_value(value, sample_t, loop_t) -> Option<f32>`. Imports `use mixed_signals::prelude::{SignalContext, SignalSpec};` at line 10. |
| `crates/tui-vfx-recipes/src/manager/fnc_populate_effects.rs` | 261 | unit | imports mixed-signals types in the manager's effect-population path; one of the two recipe-side sites that hands JSON to `mixed_signals` for signal evaluation. |

Both sites do the same three-step dance: take a `serde_json::Value`, deserialize to a mixed-signals type, call `.build()` (or `.sample_with_context(...)`). The migration replaces the type parameter — `from_value::<SignalSpec>` becomes `from_value::<VfxRecipeSignalSpec>` and the per-site evaluation uses `VfxIntoRecipeSignal::into_recipe_signal`.

`fnc_populate_effects.rs` (261 LOC) is the smaller, more contained migration; do it first to verify the facade's evaluation contract before touching the 1617-LOC compile-pipeline file.

A third file — `crates/tui-vfx-recipes/src/loopback/fnc_evaluate_loopback.rs` (208 LOC) — uses `mixed_signals::SignalOrFloat` (per `ofpf-content "SignalSpec\|SignalOrFloat"`). `SignalOrFloat` is a separate wire-format type used inside `BindableValue` (now `VfxBindableValue` per 1.2.A); it is not a `SignalSpec` site. **Out of scope for γ** — `SignalOrFloat`'s recipe-side wrapping is handled by signal-facade phase δ when the symmetric Bindable family lands. Confirm with `ofpf-content "SignalSpec" --root /usr/projects/tui-vfx-recipes` that the only sites importing `SignalSpec` directly are the two named above.

### B. Parallel motion-physics channel

`crates/tui-vfx-recipes/src/v3/compile/cls_v3_motion_envelope.rs` (570 LOC, line 60):

```rust
pub enum V3MotionDynamicSpec {
    Spring { stiffness: f32, damping: f32 },
    Bounce { bounces: u8, decay: f32 },
    Friction { drag: f32 },
    Pendulum { amplitude: f32, oscillations: f32, damping: f32 },
    Hover,
    Squash,
    Step { steps: u8 },
    Projectile { arc_height: f32, gravity: f32 },
    Orbit { /* ... */ },
    Swirl { /* ... */ },
    CarrierOrbit { /* ... */ },
    FigureEight { /* ... */ },
    // ... and others
}
```

Lowered via `pub fn lower_composed_path(route: &PathType, dynamics: &[V3MotionDynamicSpec]) -> PathType` (line 279) into the runtime's `PathType` enum.

This is **a different channel from `SignalSpec`**. `V3MotionDynamicSpec` carries motion-path dynamics that compose into a `PathType` for trajectory evaluation. `SignalSpec` carries scalar signal sources for parameter evaluation. Both happen to expose physics primitives (Spring, Bounce, Pendulum, Projectile, Orbit), which is the bifurcation the proposal §1.4 names.

**The collapse γ performs is one-directional: physics primitives become reachable via the facade as signals (`{"signal": {"type": "spring", ...}}`) so authors have one mental model.** The motion-spec channel stays untouched. Removing it is out of scope (would ripple through `lower_composed_path`, every recipe using motion dynamics, and the runtime path evaluator).

Variants the facade adds for physics: `Spring`, `Bounce`, `Pendulum`, `Projectile`, `Orbit`, `Decay`, `Attractor`. Mixed-signals provides all seven via `physics/` (per its `lib.rs:46`). Each is `Serialize + Deserialize`-able as a Rust struct.

### C. Mixed-signals public surface (audit-only — no edits)

`/usr/projects/mixed-signals/src/lib.rs:46` lists the public categories. The full `SignalSpec` enum at `/usr/projects/mixed-signals/src/types/signal_spec.rs` (read with `ofpf-extract /usr/projects/mixed-signals/src/types/signal_spec.rs SignalSpec`) confirms 40+ variants spanning oscillators, noise, spatial coordinates, envelopes, composition, processing.

| Category | Confirmed in `SignalSpec` | Confirmed in `physics/` (NOT in `SignalSpec`) |
|---|---|---|
| Oscillators (Sine, Triangle, Square, Sawtooth, Constant, Ramp, Step, Pulse) | ✓ | — |
| Noise (WhiteNoise, Perlin, SeededRandom, SpatialNoise, Gaussian, Poisson, Correlated, Pink, PerCharacter, StudentT, Impulse) | ✓ | — |
| Spatial leaves (SampleNormX/Y, SampleCellX/Y, SampleRadius, SampleSurface*From) | ✓ | — |
| Envelopes (Adsr, Impact, LinearEnvelope; LinearDecay/ExponentialDecay missing per proposal §1.4) | partial | — |
| Composition (Add, Multiply, Mix, FrequencyMod, VcaCentered, PhaseAccumulator, PhaseSine, Keyframes) | ✓ | — |
| Processing (Clamp, Quantize, Remap, Invert, Abs) | ✓ | — |
| Physics (DampedSpring, BouncingDrop, FrictionDecay, Pendulum, Orbit, Projectile, Attractor) | ✗ | ✓ |

The facade exposes the curated subset — not every variant `SignalSpec` carries. Per Intention 24 ("library changes earn their place") and the §8.7 Q3 default ("each variant earns its place; not auto-exposed"), the facade's variant set is a deliberate decision rather than a mechanical re-export of every upstream primitive.

The Core 12 (proposal §2.2) is the floor: `sine`, `triangle`, `ramp`, `keyframes`, `adsr`, `impact`, `spring`, `bounce`, `sample_norm_x`, `sample_radius`, `add`, `multiply`, `perlin`. Add the additional curated discriminants identified during phases α/β content review. Variants not in the catalog stay reachable through `SignalSpec` for production code — recipe authors cannot reach them through the facade.

### D. `signals.toml` overlay file

`find /usr/projects/tui-vfx/docs/templates -name "signals*"` returns nothing as of 2026-04-26. Phase α (autogen `SIGNALS_REFERENCE.md` from `SignalSpec` rustdoc + `signals.toml` overlay) has not been executed. **γ does not require α to ship first.** The catalog metadata γ defines (`VfxRecipeSignalMeta` in code) is the data structure α/β consume; α can land before γ (using the `SignalSpec` rustdoc surface), in parallel with γ (using `vfx_recipe_signal_catalog()` once γ ships), or after γ (also using the catalog). γ should not block on α.

If α has shipped before γ starts, the `signals.toml` overlay becomes a useful reference for the metadata strings the catalog should carry. If α has not shipped, the catalog metadata strings are written in the `VfxRecipeSignalMeta` constructor calls inline.

### E. Crate dependency graph

`crates/tui-vfx-recipes/Cargo.toml` already depends on `mixed-signals = { path = "../mixed-signals", version = "0.3.0" }`. No new dependency edges are required for γ. The facade's only external dep beyond what recipes already pulls is `serde` (already present).

### F. 1.2.A `VfxBindable<T, S = Never>` reality

Confirm with `ofpf-defs VfxBindable` that the type lives at `tui_vfx_core::bindable::VfxBindable`. γ does not directly depend on `VfxBindable` — it produces the `VfxRecipeSignalSpec` type that δ will use as `S` for the symmetric Bindable family. γ should compile and ship without δ existing.

## Open architectural questions

These are the §8.7 questions specific to the facade plus one packet-introduced naming decision. Each carries a recommended default a junior can apply if no other guidance arrives. **Stop-and-ask trigger: surface to the user before committing to Q4 (the facade enum's name) — Intention 8 is non-negotiable but the exact spelling is the leader's call.**

### Q1 — Module structure

Single `signals/` module with `mod.rs` carrying the whole enum, versus split-by-family (oscillators, physics, spatial, composition) with `mod.rs` re-exporting?

| Option | Trade-off |
|---|---|
| A — Single `mod.rs` enum | Smallest module; one file to read. Will exceed `cls_` hard limit (200 LOC) once every variant has rustdoc. |
| B — Split by family | One file per family (`cls_oscillator_variants.rs`, `cls_physics_variants.rs`, `cls_spatial_variants.rs`, `cls_composition_variants.rs`). Each file under 200 LOC. `mod.rs` re-exports a single flat enum. |

**Recommended default: Option B.** Per proposal §8.4 #2 the enum file would be large because of `Serialize/Deserialize` derives + per-variant doc comments. Split by family at the file level; the public surface stays a single flat `VfxRecipeSignalSpec` enum the author writes one JSON shape for. Implementation note: the variants must live in one `enum` definition (Rust does not support split enum bodies); the per-family files house the metadata-overlay constructors and the per-variant `VfxIntoRecipeSignal` arm conversions, while the enum body itself is in `mod.rs` or a single `cls_vfx_recipe_signal_spec.rs`. If `cls_vfx_recipe_signal_spec.rs` exceeds 200 LOC after rustdoc, accept the soft-limit overrun and document the rationale in the file's CLOG (the consolidation is intrinsic to the enum's responsibility — splitting would harm clarity).

### Q2 — Direct passthrough vs re-wrapping

The proposal §8.3 sketch wraps `Sine(mixed_signals::Sine)`. Alternative: define each variant's payload as a transparent wrapper using `#[serde(transparent)]` so deserialization passes through to `mixed_signals::Sine`'s own serde derive, but the wrapper has zero behavioral cost and gives the facade a place to attach recipe-only authoring metadata.

| Option | Trade-off |
|---|---|
| A — Direct wrap (`Sine(mixed_signals::Sine)`) | Cheapest. Zero new types. The variant carries the upstream type as-is. |
| B — Transparent wrapper (`Sine(VfxRecipeSineSpec)` where `pub struct VfxRecipeSineSpec(#[serde(transparent)] mixed_signals::Sine)`) | Per-variant wrapper struct. Slot for per-variant metadata. Upstream rename absorbed without recipe break. |

**Recommended default: Option B (transparent wrapper).** Per proposal §8.4 #3 the wrapper has zero behavioral cost and is the maintenance lever the facade exists to provide — if mixed-signals renames `Sine` to `SineWave`, recipes don't break, the wrapper absorbs it. Per Intention 24 rule 2 ("library changes earn their place"), the wrapper earns its place via the rename-absorption seam. The cost is 7–12 one-line wrapper structs (one per non-unit variant); negligible.

If Option A is chosen instead, the facade is a thin re-export and the rename-absorption capability is lost — any upstream rename in `mixed-signals` breaks every recipe. Document the trade-off in the file's CLOG if Option A is selected.

### Q3 — Versioning relationship to mixed-signals

When `mixed-signals` adds a primitive (e.g. a new noise type, a new envelope shape), does the facade auto-expose it?

**Recommended default: deliberate-add policy (per proposal §8.7 #4).** Each `VfxRecipeSignalSpec` variant is a deliberate decision driven by recipe-author need, not by mechanical mirror of upstream additions. Rationale per Intention 24 rule 6 ("watch for the rationalization chain"): the facade's value is the curation; auto-exposing every upstream addition turns the facade into a mirror, which has zero curation value.

Concrete enforcement: the catalog-completeness test (§Test plan) asserts every variant in `vfx_recipe_signal_catalog()` has a matching enum arm and vice versa. There is no test that asserts every `mixed_signals::*` primitive has a facade variant — that asymmetry is intentional. If a future contributor wants to add a primitive to the facade, they add (a) the enum variant, (b) the wrapper struct, (c) the `VfxIntoRecipeSignal` arm, (d) the catalog metadata entry, (e) the round-trip serde test. Five-point checklist; the per-variant deliberate-add cost is the curation gate.

### Q4 — Naming for the facade enum

`RecipeSignalSpec` versus `VfxRecipeSignalSpec`?

Per Intention 8 the `Vfx*` prefix tests are: (a) wire-format data, (b) errors from public APIs, (c) contract-producing traits. The facade's enum is wire-format data crossing crate boundaries (downstream consumers — gt-design, the docs autogen, anything that re-exports `tui_vfx_recipes::signals::*` — will import it). Test (a) is satisfied unambiguously.

**Recommended default: `VfxRecipeSignalSpec`.** Per Intention 8 the prefix applies; per proposal §8.7 (Naming — DECIDED 2026-04-26) the user's direction was "the longer name keeps the recipe-only scope explicit at every import site and discourages production code from reaching for it." All four public exports get the prefix: `VfxRecipeSignalSpec`, `VfxIntoRecipeSignal`, `VfxRecipeSignalMeta`, `vfx_recipe_signal_catalog()`. The free-standing helper function uses `vfx_*` (snake-case mirror) per the Vfx prefix's snake-case form for functions.

**Stop-and-ask trigger:** if the leader prefers `RecipeSignalSpec` (no Vfx prefix) on the rationale that the type lives inside `tui_vfx_recipes::signals` — making the prefix redundant — surface this to the user. Counter-argument: downstream `use tui_vfx_recipes::signals::RecipeSignalSpec;` does not carry the `Vfx` provenance, so a reader of the consumer file has to chase the import to learn the type's origin. The prefix is a one-time cost (typing `Vfx`) for permanent clarity at every import site.

### Q5 — Catalog metadata storage

`VfxRecipeSignalMeta` entries inline in code (one constructor call per variant) versus loaded from `signals.toml` at startup?

| Option | Trade-off |
|---|---|
| A — Inline in code | One source of truth per variant; rustdoc + metadata stay coherent on every edit (per `feedback_rustdoc_when_editing`). No runtime parse cost. No dep on `toml`. |
| B — `signals.toml` overlay | Authoring metadata edits don't require a Rust recompile. New crate dep (`toml`). Two sources of truth (variant rustdoc + toml entry) — drift risk. |

**Recommended default: Option A (inline).** Per proposal §8.1 ("Lives in code (not toml) so rustdoc + the metadata stay coherent on every edit"). The catalog is iterable through `vfx_recipe_signal_catalog() -> &'static [VfxRecipeSignalMeta]`. Phase α's autogen pipeline reads this iterator. If phase α elects to use a `signals.toml` overlay for cosmetic strings (one-line summary, recommended usage), that's α's decision; the catalog provides the structural data either way.

## Step-by-step implementation plan

OFPF discipline: edit one file at a time, write tests first (red), implement (green), confirm clippy clean, commit interim work between phases.

### Phase 1 — Define `VfxRecipeSignalSpec` + per-family modules

**Step 1.1.** Pre-edit: `ofpf-inspect crates/tui-vfx-recipes/src/lib.rs`. Confirm where to add `pub mod signals;`. `ofpf-inspect crates/tui-vfx-recipes/src/prelude.rs` — confirm whether the prelude should re-export `signals::*` (recommended: yes, for consumer ergonomics).

**Step 1.2.** Write the failing tests first. New file `crates/tui-vfx-recipes/src/signals/test_signals.rs` (peer test per OFPF). Cover:

- Round-trip serde for each variant: `{"type": "sine", "frequency": 0.5}` → `VfxRecipeSignalSpec::Sine(VfxRecipeSineSpec(...))` → `{"type": "sine", "frequency": 0.5}`. One test per family.
- Catalog completeness: assert `vfx_recipe_signal_catalog().len()` matches the variant count. Assert every variant has a catalog entry by discriminant matching.
- `VfxIntoRecipeSignal::into_recipe_signal` produces a `Box<dyn Signal>` that `.sample_with_context(...)` returns a finite `f32`.
- Backward-compat: every JSON shape that the old `SignalSpec`-based deserialization accepted continues to deserialize via the facade for the curated variants.
- Physics specifically: `{"type": "spring", "stiffness": 8.0, "damping": 0.6, "target": 1.0}` deserializes successfully (this is the case `SignalSpec` rejected — collapsing the parallel channel).

Run `cargo test -p tui-vfx-recipes signals` — fails (module does not exist).

**Step 1.3.** Create the module skeleton:

```
crates/tui-vfx-recipes/src/signals/
  mod.rs                              # pub use, module docs, the enum body
  cls_vfx_recipe_signal_spec.rs       # the enum definition (or in mod.rs if Q1=A)
  fnc_into_recipe_signal.rs           # the trait + impls
  cls_vfx_recipe_signal_meta.rs       # autogen metadata struct
  fnc_vfx_recipe_signal_catalog.rs    # the static catalog
  oscillators/
    mod.rs
    cls_vfx_recipe_sine_spec.rs       # transparent wrapper structs
    cls_vfx_recipe_triangle_spec.rs
    # ... one per oscillator variant
  physics/
    mod.rs
    cls_vfx_recipe_spring_spec.rs
    cls_vfx_recipe_bounce_spec.rs
    cls_vfx_recipe_pendulum_spec.rs
    cls_vfx_recipe_projectile_spec.rs
    cls_vfx_recipe_orbit_spec.rs
    cls_vfx_recipe_decay_spec.rs
    cls_vfx_recipe_attractor_spec.rs
  spatial/
    mod.rs                            # unit-variant cohort; small file
  composition/
    mod.rs
    cls_vfx_recipe_add_spec.rs
    cls_vfx_recipe_multiply_spec.rs
    cls_vfx_recipe_mix_spec.rs
  test_signals.rs                     # peer tests for the module
```

OFPF prefixes apply (`cls_` for wrapper structs, `fnc_` for free functions, `mod.rs` for re-exports). The per-family `mod.rs` files are pure re-export surfaces and can go below the soft size limit.

Metadata envelope template for every new file:

- `<DESC>` — file-specific, one line. Examples:
  - `cls_vfx_recipe_signal_spec.rs`: "Recipe-facing signal expression — tagged enum that mirrors the curated subset of mixed-signals primitives recipes can reach."
  - `cls_vfx_recipe_spring_spec.rs`: "Transparent serde wrapper over mixed_signals::DampedSpring for recipe-side deserialization."
  - `fnc_into_recipe_signal.rs`: "Convert a VfxRecipeSignalSpec into a Box<dyn mixed_signals::Signal> for the recipe runtime."
- `<VERS>VERSION: 0.1.0</VERS>`
- `<WCTX>Signal-facade phase γ — build the tui_vfx_recipes::signals module per the 2026-04-26 Option A acceptance. Curated, normalized recipe-author surface in front of mixed_signals::*; production code keeps importing mixed_signals::* directly.</WCTX>`
- `<CLOG>0.1.0: initial implementation — &lt;file-specific change&gt;.</CLOG>`

**Step 1.4.** Implement the enum + wrapper structs + trait + catalog per §Code snippets. One file at a time; run `cargo test -p tui-vfx-recipes signals` after each per-family group lands.

**Step 1.5.** Update `crates/tui-vfx-recipes/src/lib.rs`: add `pub mod signals;`. Update `crates/tui-vfx-recipes/src/prelude.rs`: add `pub use crate::signals::{VfxRecipeSignalSpec, VfxIntoRecipeSignal, VfxRecipeSignalMeta, vfx_recipe_signal_catalog};`.

**Step 1.6.** Run `cargo test -p tui-vfx-recipes signals` — green. Run `cargo build --workspace` — confirm no consumer breaks.

**Step 1.7.** Commit interim: `Add tui_vfx_recipes::signals module — VfxRecipeSignalSpec + catalog (γ phase 1)`.

### Phase 2 — Migrate `fnc_populate_effects.rs` to the facade

**Step 2.1.** Pre-edit: `ofpf-inspect crates/tui-vfx-recipes/src/manager/fnc_populate_effects.rs`. Identify every `mixed_signals::SignalSpec` call site (per Phase 1 audit, this file is the smaller of the two and a safer first migration).

**Step 2.2.** Replace `SignalSpec` deserialization with `VfxRecipeSignalSpec` deserialization. Replace `.build()` calls with `VfxIntoRecipeSignal::into_recipe_signal()`. Bump file VERS to next minor; CLOG entry: `0.X.0: route signal deserialization through tui_vfx_recipes::signals::VfxRecipeSignalSpec instead of mixed_signals::SignalSpec — signal-facade phase γ migration. Recipe wire format unchanged.`

**Step 2.3.** Run `cargo test -p tui-vfx-recipes manager` — confirm every existing test passes. The wire format is preserved, so every recipe that loaded before continues to load.

**Step 2.4.** Run `cargo test -p tui-vfx-recipes` — full crate green.

**Step 2.5.** Commit interim: `Route fnc_populate_effects through VfxRecipeSignalSpec (γ phase 2)`.

### Phase 3 — Migrate `fnc_build_composition_spec_from_compiled_plan.rs` to the facade

**Step 3.1.** Pre-edit: `ofpf-inspect crates/tui-vfx-recipes/src/v3/compile/fnc_build_composition_spec_from_compiled_plan.rs`. Identify the `sample_signal_value` helper at line 627–639 and any other `SignalSpec` references (`grep -n "SignalSpec" crates/tui-vfx-recipes/src/v3/compile/fnc_build_composition_spec_from_compiled_plan.rs`).

**Step 3.2.** Replace the type parameter at line 633: `serde_json::from_value::<SignalSpec>(value.clone()).ok()?` becomes `serde_json::from_value::<VfxRecipeSignalSpec>(value.clone()).ok()?`. Replace `spec.build()` with `spec.into_recipe_signal()`. Update the `use` at line 10 — drop `SignalSpec` import; `SignalContext` stays (it's a runtime carry type, not a wire-format type).

**Step 3.3.** **Verify the parallel motion-physics channel collapse from the author's perspective.** Add a test (in `crates/tui-vfx-recipes/src/signals/test_signals.rs` or in the V3 compile test surface): a recipe with `{"signal": {"type": "spring", "stiffness": 8.0, "damping": 0.6, "target": 1.0}}` deserializes through the facade and produces a sampleable signal. Before γ this would have failed (`SignalSpec` rejects `spring`); after γ it succeeds. This is the headline collapse-evidence test.

**Step 3.4.** Run `cargo test -p tui-vfx-recipes v3` — confirm every existing test passes.

**Step 3.5.** Run `cargo test -p tui-vfx-recipes` — full crate green.

**Step 3.6.** Commit interim: `Route fnc_build_composition_spec through VfxRecipeSignalSpec (γ phase 3)`.

### Phase 4 — Workspace verification + documentation

**Step 4.1.** Run the full §Verification commands block.

**Step 4.2.** Update `docs/design/tui-vfx-2026-04-26-handoff-outstanding.md` (if it carries a γ-tracking entry) to mark γ as done.

**Step 4.3.** If `VfxRecipeSignalSpec` should appear in `docs/templates/capabilities.toml` or `docs/templates/api_docs.toml`, add the entry. Run `cargo xtask docs generate` and commit the regenerated `docs/CAPABILITIES_REFERENCE.md` / `docs/API_DOCS.md`.

**Step 4.4.** Audit rustdoc on every public item in the new module per `feedback_rustdoc_when_editing`. Every `pub` item — the enum, every variant, every wrapper struct, the trait, the meta struct, the catalog function — gets at minimum a one-line `///` doc. Variants that wrap a `mixed_signals::*` type cite the upstream type by full path in the doc.

**Step 4.5.** Final commit: `Phase 4: workspace clean (γ complete — VfxRecipeSignalSpec is the recipe-side signal seam)`.

## Code snippets

### Module root (`signals/mod.rs`)

```rust
//! # Recipe-facing signal surface
//!
//! `tui_vfx_recipes::signals::*` is the deserialization seam between recipe
//! JSON and the runtime signal value the effect machinery consumes. This
//! module is recipe-deserialization-only — production code in `tui-vfx`,
//! `tui-vfx-style`, `tui-vfx-compositor`, `tui-vfx-content`, and
//! `gt-design` keeps importing `mixed_signals::*` directly.
//!
//! ## What this module does
//!
//! 1. Defines [`VfxRecipeSignalSpec`] — the curated, normalized enum a
//!    recipe author writes JSON against.
//! 2. Provides [`VfxIntoRecipeSignal`] for converting that enum into a
//!    `Box<dyn mixed_signals::traits::Signal>` for runtime evaluation.
//! 3. Hosts [`vfx_recipe_signal_catalog`] — the iterable metadata table
//!    `cargo xtask docs generate` reads to emit `SIGNALS_REFERENCE.md`.
//!
//! ## What this module does NOT do
//!
//! - Re-export `mixed_signals::*` types. Recipe authors write JSON; the
//!   facade absorbs the upstream type behind a transparent wrapper.
//! - Intercept production code paths. Filters, masks, samplers, shaders
//!   continue importing `mixed_signals::Signal` directly.
//! - Auto-expose every upstream primitive. Each variant is a deliberate
//!   curation decision driven by recipe-author need.

mod cls_vfx_recipe_signal_spec;
mod cls_vfx_recipe_signal_meta;
mod fnc_into_recipe_signal;
mod fnc_vfx_recipe_signal_catalog;

pub mod oscillators;
pub mod physics;
pub mod spatial;
pub mod composition;

pub use cls_vfx_recipe_signal_spec::VfxRecipeSignalSpec;
pub use cls_vfx_recipe_signal_meta::VfxRecipeSignalMeta;
pub use fnc_into_recipe_signal::VfxIntoRecipeSignal;
pub use fnc_vfx_recipe_signal_catalog::vfx_recipe_signal_catalog;

#[cfg(test)]
mod test_signals;
```

### The enum (`cls_vfx_recipe_signal_spec.rs`)

Per Q2 default (transparent wrapper) and Q1 default (split-by-family):

```rust
use serde::{Deserialize, Serialize};

use crate::signals::oscillators::{
    VfxRecipeSineSpec, VfxRecipeTriangleSpec, VfxRecipeRampSpec,
    VfxRecipeKeyframesSpec, VfxRecipeAdsrSpec, VfxRecipeImpactSpec,
    VfxRecipePerlinSpec,
};
use crate::signals::physics::{
    VfxRecipeSpringSpec, VfxRecipeBounceSpec, VfxRecipePendulumSpec,
    VfxRecipeProjectileSpec, VfxRecipeOrbitSpec, VfxRecipeDecaySpec,
    VfxRecipeAttractorSpec,
};
use crate::signals::composition::{
    VfxRecipeAddSpec, VfxRecipeMultiplySpec, VfxRecipeMixSpec,
};

/// Recipe-facing signal expression. Tagged enum mirroring the curated
/// surface a recipe author can reach for. Wraps mixed-signals primitives
/// behind transparent serde wrappers so upstream renames do not break
/// recipes.
///
/// # Wire format
///
/// `#[serde(tag = "type", rename_all = "snake_case")]` — every variant is
/// a JSON object with a `"type"` discriminant and the upstream type's
/// fields inlined. Backward-compatible with the JSON shapes recipe
/// authors wrote against `mixed_signals::SignalSpec` for the variants
/// the facade exposes.
///
/// # Curation policy
///
/// Each variant is a deliberate decision. Adding a variant requires:
/// 1. The enum arm here.
/// 2. The transparent wrapper struct in the per-family module.
/// 3. The arm in [`crate::signals::VfxIntoRecipeSignal`].
/// 4. The catalog entry in [`crate::signals::vfx_recipe_signal_catalog`].
/// 5. A round-trip serde test in `test_signals.rs`.
///
/// New `mixed_signals::*` primitives do NOT auto-expose through the
/// facade.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum VfxRecipeSignalSpec {
    // --- Oscillators ---
    /// Single-frequency sine wave. Wraps [`mixed_signals::Sine`].
    Sine(VfxRecipeSineSpec),
    /// Triangle wave. Wraps `mixed_signals::Triangle`.
    Triangle(VfxRecipeTriangleSpec),
    /// Linear interpolation between two values over a duration.
    Ramp(VfxRecipeRampSpec),
    /// Time-keyed waypoints. Wraps `mixed_signals::Keyframes`.
    Keyframes(VfxRecipeKeyframesSpec),

    // --- Envelopes ---
    /// ADSR envelope. Wraps `mixed_signals::Adsr`.
    Adsr(VfxRecipeAdsrSpec),
    /// Impact envelope (sharp attack, exponential decay).
    Impact(VfxRecipeImpactSpec),

    // --- Noise ---
    /// Perlin noise. Wraps `mixed_signals::Perlin`.
    Perlin(VfxRecipePerlinSpec),

    // --- Spatial leaves (nullary) ---
    /// Per-cell normalized X coordinate from `SignalContext`.
    SampleNormX,
    /// Per-cell normalized Y coordinate from `SignalContext`.
    SampleNormY,
    /// Per-cell radial distance from center.
    SampleRadius,

    // --- Physics (collapse the parallel motion-spec channel from the author's POV) ---
    /// Damped spring. Wraps `mixed_signals::DampedSpring`. Reachable
    /// today only through the motion-spec dynamics channel; the facade
    /// makes it reachable as a signal source too.
    Spring(VfxRecipeSpringSpec),
    /// Bouncing drop. Wraps `mixed_signals::BouncingDrop`.
    Bounce(VfxRecipeBounceSpec),
    /// Pendulum. Wraps `mixed_signals::Pendulum`.
    Pendulum(VfxRecipePendulumSpec),
    /// Ballistic projectile arc. Wraps `mixed_signals::Projectile`.
    Projectile(VfxRecipeProjectileSpec),
    /// Orbital motion. Wraps `mixed_signals::Orbit`.
    Orbit(VfxRecipeOrbitSpec),
    /// Friction decay. Wraps `mixed_signals::FrictionDecay`.
    Decay(VfxRecipeDecaySpec),
    /// Attractor. Wraps `mixed_signals::Attractor`.
    Attractor(VfxRecipeAttractorSpec),

    // --- Composition (recursively contain VfxRecipeSignalSpec) ---
    /// Sum of two signal expressions.
    Add(VfxRecipeAddSpec),
    /// Product of two signal expressions.
    Multiply(VfxRecipeMultiplySpec),
    /// Weighted blend of two signal expressions.
    Mix(VfxRecipeMixSpec),
}
```

### One example per family (transparent wrappers)

**Oscillator** (`oscillators/cls_vfx_recipe_sine_spec.rs`):

```rust
use serde::{Deserialize, Serialize};

/// Recipe-side wrapper over `mixed_signals::Sine`. Transparent serde
/// passthrough — the JSON shape matches the upstream type exactly.
/// Exists as a wrapper so an upstream rename (e.g. `Sine` →
/// `SineWave`) is absorbed by editing this one file rather than every
/// recipe that mentions sine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VfxRecipeSineSpec(pub mixed_signals::Sine);
```

**Physics** (`physics/cls_vfx_recipe_spring_spec.rs`):

```rust
use serde::{Deserialize, Serialize};

/// Recipe-side wrapper over `mixed_signals::DampedSpring`. Note the
/// upstream type name carries `Damped`; the facade exposes it as `Spring`
/// for the recipe-author surface (per the Core 12 cheatsheet).
///
/// # Wire format
///
/// ```jsonc
/// {"type": "spring", "stiffness": 8.0, "damping": 0.6, "target": 1.0}
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VfxRecipeSpringSpec(pub mixed_signals::DampedSpring);
```

**Spatial** (`spatial/mod.rs` — unit variants live in the parent enum directly; no wrapper needed):

```rust
//! Unit variants for nullary spatial leaves. The `SampleNormX`,
//! `SampleNormY`, `SampleRadius` discriminants live as bare arms on
//! [`crate::signals::VfxRecipeSignalSpec`] — no wrapper struct because
//! there's no payload to wrap. This module is documentation-only.
```

**Composition** (`composition/cls_vfx_recipe_mix_spec.rs`):

```rust
use serde::{Deserialize, Serialize};

use crate::signals::VfxRecipeSignalSpec;

/// Weighted blend of two recipe-side signal expressions. Recurses into
/// `VfxRecipeSignalSpec`, NOT `mixed_signals::SignalSpec` — the recipe
/// author stays in the facade's surface.
///
/// # Wire format
///
/// ```jsonc
/// {"type": "mix",
///  "a": {"type": "sine", "frequency": 1.0},
///  "b": {"type": "perlin", "scale": 0.3},
///  "weight": {"type": "ramp", "start": 0, "end": 1, "duration": 2.0}}
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VfxRecipeMixSpec {
    pub a: Box<VfxRecipeSignalSpec>,
    pub b: Box<VfxRecipeSignalSpec>,
    pub weight: Box<VfxRecipeSignalSpec>,
}
```

### The trait (`fnc_into_recipe_signal.rs`)

```rust
use mixed_signals::traits::Signal;

use crate::signals::VfxRecipeSignalSpec;

/// Convert a recipe-side signal expression into a runtime signal value.
/// The recipe-loading path calls this immediately after deserialization;
/// the resulting `Box<dyn Signal>` flows into the same evaluation
/// machinery as signals constructed from `mixed_signals::SignalSpec` in
/// production code.
pub trait VfxIntoRecipeSignal {
    /// Build the runtime signal. Returns a boxed trait object so the
    /// downstream code (filter / mask / sampler / shader machinery) can
    /// hold a heterogeneous collection of signal sources without caring
    /// about the concrete type.
    fn into_recipe_signal(self) -> Box<dyn Signal>;
}

impl VfxIntoRecipeSignal for VfxRecipeSignalSpec {
    fn into_recipe_signal(self) -> Box<dyn Signal> {
        match self {
            VfxRecipeSignalSpec::Sine(spec) => Box::new(spec.0),
            VfxRecipeSignalSpec::Triangle(spec) => Box::new(spec.0),
            VfxRecipeSignalSpec::Ramp(spec) => Box::new(spec.0),
            VfxRecipeSignalSpec::Keyframes(spec) => Box::new(spec.0),
            VfxRecipeSignalSpec::Adsr(spec) => Box::new(spec.0),
            VfxRecipeSignalSpec::Impact(spec) => Box::new(spec.0),
            VfxRecipeSignalSpec::Perlin(spec) => Box::new(spec.0),
            VfxRecipeSignalSpec::SampleNormX => {
                // Reach mixed-signals' nullary spatial constructor.
                // Exact constructor depends on mixed-signals' API surface;
                // verify with `ofpf-defs SpatialCoordinate` against
                // /usr/projects/mixed-signals before settling on the call.
                Box::new(mixed_signals::generators::SpatialCoordinateSignal::norm_x())
            }
            VfxRecipeSignalSpec::SampleNormY => {
                Box::new(mixed_signals::generators::SpatialCoordinateSignal::norm_y())
            }
            VfxRecipeSignalSpec::SampleRadius => {
                Box::new(mixed_signals::generators::SpatialCoordinateSignal::radius())
            }
            VfxRecipeSignalSpec::Spring(spec) => Box::new(spec.0),
            VfxRecipeSignalSpec::Bounce(spec) => Box::new(spec.0),
            VfxRecipeSignalSpec::Pendulum(spec) => Box::new(spec.0),
            VfxRecipeSignalSpec::Projectile(spec) => Box::new(spec.0),
            VfxRecipeSignalSpec::Orbit(spec) => Box::new(spec.0),
            VfxRecipeSignalSpec::Decay(spec) => Box::new(spec.0),
            VfxRecipeSignalSpec::Attractor(spec) => Box::new(spec.0),
            VfxRecipeSignalSpec::Add(spec) => {
                let a = spec.a.into_recipe_signal();
                let b = spec.b.into_recipe_signal();
                Box::new(mixed_signals::composition::Add::new(a, b))
            }
            VfxRecipeSignalSpec::Multiply(spec) => {
                let a = spec.a.into_recipe_signal();
                let b = spec.b.into_recipe_signal();
                Box::new(mixed_signals::composition::Multiply::new(a, b))
            }
            VfxRecipeSignalSpec::Mix(spec) => {
                let a = spec.a.into_recipe_signal();
                let b = spec.b.into_recipe_signal();
                let weight = spec.weight.into_recipe_signal();
                Box::new(mixed_signals::composition::Mix::new(a, b, weight))
            }
        }
    }
}
```

The exact mixed-signals constructor calls in the spatial / composition arms are placeholders — verify with `ofpf-defs SpatialCoordinateSignal` and `ofpf-defs Add` (against `--root /usr/projects/mixed-signals`) and adjust to the actual upstream API before committing. The pattern is the same: the facade recurses on its own enum and hands `Box<dyn Signal>` payloads to the upstream composition constructors.

### Catalog metadata (`fnc_vfx_recipe_signal_catalog.rs`)

```rust
use crate::signals::cls_vfx_recipe_signal_meta::VfxRecipeSignalMeta;

/// Iterable catalog of every variant in [`crate::signals::VfxRecipeSignalSpec`]
/// with authoring metadata. Read by `cargo xtask docs generate` to emit
/// `docs/SIGNALS_REFERENCE.md` (signal-facade phases α + β).
///
/// # Curation invariant
///
/// Length matches `VfxRecipeSignalSpec`'s variant count exactly. The
/// `catalog_completeness` test in `test_signals.rs` enforces this.
pub fn vfx_recipe_signal_catalog() -> &'static [VfxRecipeSignalMeta] {
    static CATALOG: &[VfxRecipeSignalMeta] = &[
        VfxRecipeSignalMeta {
            discriminant: "sine",
            family: "oscillator",
            one_line_summary: "Single-frequency sine wave",
            recommended_use: "Smooth periodic motion; opacity pulse; soft oscillation",
            wraps_upstream: Some("mixed_signals::Sine"),
            in_core_12: true,
        },
        // ... one entry per variant ...
        VfxRecipeSignalMeta {
            discriminant: "spring",
            family: "physics",
            one_line_summary: "Damped spring oscillation",
            recommended_use: "Tactile bounce-into-place; UI snap; settle-after-impact",
            wraps_upstream: Some("mixed_signals::DampedSpring"),
            in_core_12: true,
        },
        // ...
    ];
    CATALOG
}
```

### Recipe-side migration shape (Phase 3 site)

Before (`crates/tui-vfx-recipes/src/v3/compile/fnc_build_composition_spec_from_compiled_plan.rs:633`):

```rust
use mixed_signals::prelude::{SignalContext, SignalSpec};

fn sample_signal_value(
    value: &serde_json::Value,
    sample_t: f64,
    loop_t: Option<f64>,
) -> Option<f32> {
    let spec = serde_json::from_value::<SignalSpec>(value.clone()).ok()?;
    let signal = spec.build().ok()?;
    let t = loop_t.unwrap_or(sample_t).clamp(0.0, 1.0);
    let ctx = SignalContext::new(0, 0);
    Some(signal.sample_with_context(t, &ctx))
}
```

After:

```rust
use mixed_signals::prelude::SignalContext;
use crate::signals::{VfxIntoRecipeSignal, VfxRecipeSignalSpec};

fn sample_signal_value(
    value: &serde_json::Value,
    sample_t: f64,
    loop_t: Option<f64>,
) -> Option<f32> {
    let spec = serde_json::from_value::<VfxRecipeSignalSpec>(value.clone()).ok()?;
    let signal = spec.into_recipe_signal();
    let t = loop_t.unwrap_or(sample_t).clamp(0.0, 1.0);
    let ctx = SignalContext::new(0, 0);
    Some(signal.sample_with_context(t, &ctx))
}
```

The diff is one type and one method call. Recipe wire format unchanged for every variant the facade exposes; the `{"type": "spring", ...}` shape that the old version rejected now succeeds.

## Test plan

### Existing tests that must keep passing unchanged

- `cargo test -p tui-vfx-recipes` — full crate green. Zero recipe-loading regressions.
- `cargo test -p tui-vfx-recipes manager` — every `fnc_populate_effects` test (Phase 2 verification).
- `cargo test -p tui-vfx-recipes v3` — every V3 compile test (Phase 3 verification).
- The recipe-fixture tests under `tests/` and `recipes/` — every `.json` recipe that exercises a signal field deserializes successfully through the facade.

### New tests in `crates/tui-vfx-recipes/src/signals/test_signals.rs`

Coverage:

- **Round-trip serde per variant.** One test per variant: serialize `VfxRecipeSignalSpec::Sine(VfxRecipeSineSpec(Sine::default()))`, parse the result, assert equality. Covers every arm of the enum.
- **Catalog completeness.** Assert `vfx_recipe_signal_catalog().len()` equals the variant count of `VfxRecipeSignalSpec` (use `std::mem::variant_count` if stable, otherwise hardcode the count and update on every addition — flag in CLOG). Assert each catalog entry's `discriminant` matches a variant's `#[serde(rename_all = "snake_case")]`-derived name.
- **Backward-compat for every curated variant.** For each variant, deserialize the JSON shape an existing recipe uses (audit `recipes/**/*.json` via `grep -rln "\"type\": \"X\"" recipes/` for representative examples) and assert it parses through the facade.
- **Physics collapse evidence.** Deserialize `{"type": "spring", "stiffness": 8.0, "damping": 0.6, "target": 1.0}` through `VfxRecipeSignalSpec`. Build via `into_recipe_signal()`. Sample at t=0.5 and assert a finite value. Before γ, the same JSON would have failed `SignalSpec` deserialization; after γ it succeeds.
- **Composition recursion.** Deserialize a nested `mix(sine, perlin, weight=ramp)` shape and assert each recursive arm is a `VfxRecipeSignalSpec` variant (not a `mixed_signals::SignalSpec`). Catches regressions where a contributor accidentally recurses into the upstream type.
- **No production import leak.** Compile-time check: `assert!(std::any::type_name::<VfxRecipeSignalSpec>().starts_with("tui_vfx_recipes::signals"));` verifies the type lives where it should.

### Additional test in the V3 compile surface

- **End-to-end recipe with a spring signal.** Add a fixture recipe under `tests/fixtures/` (or wherever existing V3 fixtures live; verify with `find tests -name "*.json"`) that uses `{"signal": {"type": "spring", ...}}` for a filter parameter. Load through the recipe pipeline and assert it compiles to a runnable composition. This is the integration test for the parallel-channel collapse.

### TDD red→green per phase

1. Phase 1 red: `cargo test -p tui-vfx-recipes signals` fails (module does not exist).
2. Phase 1 green: implement the module per §Code snippets; tests pass.
3. Phase 2 red: add the spring-deserializes-through-facade test before migrating; assert it fails against the still-`SignalSpec`-routed code.
4. Phase 2 green: migrate `fnc_populate_effects.rs`; the new test passes; existing tests stay green.
5. Phase 3 red: same — physics collapse evidence test asserts the V3 compile path accepts spring signals; fails before migration.
6. Phase 3 green: migrate `fnc_build_composition_spec_from_compiled_plan.rs`; all tests green.

### Per-phase test commands

```bash
# Phase 1
cargo test -p tui-vfx-recipes signals

# Phase 2
cargo test -p tui-vfx-recipes manager
cargo test -p tui-vfx-recipes signals
cargo build --workspace

# Phase 3
cargo test -p tui-vfx-recipes v3
cargo test -p tui-vfx-recipes signals
cargo test -p tui-vfx-recipes
cargo build --workspace

# Phase 4 (final)
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo doc --no-deps
```

## Acceptance criteria

- [ ] `crates/tui-vfx-recipes/src/signals/` module subtree exists per Phase 1 layout.
- [ ] Exactly four public exports from `tui_vfx_recipes::signals`: `VfxRecipeSignalSpec`, `VfxIntoRecipeSignal`, `VfxRecipeSignalMeta`, `vfx_recipe_signal_catalog`.
- [ ] `VfxRecipeSignalSpec` carries the curated Core 12 + advanced variants per the §Audit table; physics variants (`Spring`, `Bounce`, `Pendulum`, `Projectile`, `Orbit`, `Decay`, `Attractor`) included.
- [ ] Per-variant transparent wrapper structs in per-family modules (`oscillators/`, `physics/`, `composition/`); spatial unit variants live on the enum directly. (Q1 + Q2 defaults applied.)
- [ ] `VfxIntoRecipeSignal::into_recipe_signal` produces a working `Box<dyn Signal>` for every variant.
- [ ] `vfx_recipe_signal_catalog()` returns one entry per variant; the `catalog_completeness` test passes.
- [ ] **Recipe-side signal deserialization migrated:**
    - [ ] `crates/tui-vfx-recipes/src/manager/fnc_populate_effects.rs` deserializes through `VfxRecipeSignalSpec` (Phase 2).
    - [ ] `crates/tui-vfx-recipes/src/v3/compile/fnc_build_composition_spec_from_compiled_plan.rs` deserializes through `VfxRecipeSignalSpec` (Phase 3).
- [ ] **Physics collapse evidence:** a recipe with `{"signal": {"type": "spring", ...}}` deserializes successfully and samples to a finite value. Test landed in `signals/test_signals.rs` and (separately) in the V3 compile test surface.
- [ ] **Recipe wire format preserved** — every existing recipe under `recipes/`, `designer_recipes/`, and the `AA_*` design-recipe directories continues to load. No author edits required.
- [ ] **Production code untouched** — `crates/tui-vfx*/src/**/*.rs` (excluding `tui-vfx-recipes`) carries zero new imports of `tui_vfx_recipes::signals::*`. The facade is recipe-deserialization-only per the Option A acceptance.
- [ ] **mixed-signals untouched** — `git status` against `/usr/projects/mixed-signals` shows no diffs (Intention 9).
- [ ] `cargo build --workspace` succeeds with zero new warnings.
- [ ] `cargo test --workspace` green.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` clean.
- [ ] No `#[allow]` suppressions added per `feedback_no_landmines`.
- [ ] No inert schema fields per `feedback_no_inert_schema` — every variant of `VfxRecipeSignalSpec` is reachable via `into_recipe_signal()` and exercised in the round-trip test.
- [ ] **Loopback-safe** per `feedback_loopback_required` — no `requires_bindings` entries introduced by γ; the facade is a deserialization seam, not a bindings producer.
- [ ] `cargo doc --no-deps` succeeds with no broken intra-doc links. Every public item in `tui_vfx_recipes::signals` carries at minimum a one-line `///` doc; variants that wrap a `mixed_signals::*` type cite the upstream type by full path.
- [ ] Rustdoc improved on every public item touched per `feedback_rustdoc_when_editing`.
- [ ] If `VfxRecipeSignalSpec` appears in `docs/templates/capabilities.toml` or `docs/templates/api_docs.toml`, `cargo xtask docs generate` regenerates the relevant manifest and the regen is committed.
- [ ] **Vfx*-prefix decision (Q4) recorded in the enum file's CLOG.** If the user accepted `VfxRecipeSignalSpec`, the file's CLOG names Intention 8 explicitly.
- [ ] `crates/tui-vfx-recipes/Cargo.toml` gains zero new external dependencies (already has `mixed-signals`, `serde`).

## Verification commands

```bash
# Build clean across the workspace.
cargo build --workspace

# Per-crate tests.
cargo test -p tui-vfx-recipes
cargo test --workspace

# Targeted module tests.
cargo test -p tui-vfx-recipes signals

# Clippy with denied warnings.
cargo clippy --workspace --all-targets -- -D warnings

# Rustdoc clean.
cargo doc --no-deps

# Capability manifest regen (if applicable).
cargo xtask docs generate

# Confirm production code does not import the facade.
grep -rln "use tui_vfx_recipes::signals" /usr/projects/tui-vfx/crates/ \
    | grep -v "tui-vfx-recipes/" \
    || echo "OK: no production import leaks into the facade"

# Confirm mixed-signals is untouched.
( cd /usr/projects/mixed-signals && git status --porcelain )
# Expect empty output.

# Confirm the parallel motion-physics channel still exists (γ does NOT remove it).
grep -n "V3MotionDynamicSpec" /usr/projects/tui-vfx-recipes/src/v3/compile/cls_v3_motion_envelope.rs | head -3

# Confirm the two migrated sites no longer import SignalSpec directly.
grep -n "SignalSpec" /usr/projects/tui-vfx-recipes/src/manager/fnc_populate_effects.rs || echo "OK: SignalSpec no longer imported"
grep -n "SignalSpec" /usr/projects/tui-vfx-recipes/src/v3/compile/fnc_build_composition_spec_from_compiled_plan.rs || echo "OK: SignalSpec no longer imported"
```

## Rollback plan

The packet is structured as four interim commits (one per phase). If any phase reveals a deal-breaker:

1. Stop. Do not continue to the next phase.
2. `git revert <phase-commit-hash>` on the most recent phase. Earlier phases stay landed (they are additive — Phase 1 just adds a new module, Phases 2–3 redirect existing call sites).
3. If the deal-breaker is in Phase 1 (the facade itself), `git revert` Phase 1 too. Move the new module to `recyclebin/crates/tui-vfx-recipes/src/signals/` per the recyclebin protocol.
4. `cargo build --workspace` and `cargo test --workspace` to confirm the restored state compiles and tests pass.
5. File a finding in the proposal doc capturing what blocked the consolidation, then surface to the user. Common blockers to anticipate:
   - **Serde wire-shape mismatch.** A `mixed_signals::*` type's serde derive emits a JSON shape the recipe wire format does not match exactly (e.g. field renaming, default-value handling). Fix: replace the transparent wrapper with a hand-rolled `#[serde(from = ..., into = ...)]` shim. Add a regression test for the specific shape.
   - **Spatial leaf constructor mismatch.** The mixed-signals constructors for `SpatialCoordinateSignal::norm_x()` / `norm_y()` / `radius()` do not exist with those names; the actual constructors take parameters. Fix: verify with `ofpf-defs SpatialCoordinateSignal` against `/usr/projects/mixed-signals`, adjust the arm bodies. The facade arms can carry the parameter literals if upstream requires them.
   - **`Box<dyn Signal>` trait-object incompatibility.** A specific `mixed_signals::*` type does not implement `Signal` directly (e.g. requires a builder step). Fix: call the builder in the `into_recipe_signal` arm before boxing.
   - **Recipe-fixture regression.** A specific recipe under `recipes/` or `designer_recipes/` deserializes against `SignalSpec` today but fails against `VfxRecipeSignalSpec` because the recipe uses a variant the facade doesn't expose. Fix: either add the variant to the facade (curation decision) or document the variant as deliberately excluded and migrate the recipe.

The recyclebin protocol from `~/.claude/CLAUDE.md` mandates moves over deletes — every retired file goes to `recyclebin/` mirroring its original path, never `rm`.

## Risks & gotchas

- **Serde shape preservation is the load-bearing constraint.** Every existing recipe's signal field must continue to deserialize byte-for-byte. The `#[serde(transparent)]` wrappers make this automatic for variants where the upstream type's serde derive produces the same JSON the recipe wrote against `SignalSpec`. Verify the tagged-discriminant interaction: `VfxRecipeSignalSpec` uses `#[serde(tag = "type", rename_all = "snake_case")]`, so each variant becomes `{"type": "<snake_name>", <upstream_fields>}`. If `mixed_signals::Sine`'s serde derive uses different field names than `SignalSpec`'s `Sine` variant did, the facade's wire format diverges from the legacy. **Mitigation:** the round-trip serde test must compare against the literal JSON byte-strings authors write today. Run the test against representative recipe fixtures (audit with `grep -rln "\"type\": \"sine\"" /usr/projects/tui-vfx-recipes/recipes/`).

- **The parallel motion-physics channel collapse is one-directional.** The facade adds physics primitives as signal sources; the existing `V3MotionDynamicSpec` channel stays untouched. Authors who write `dynamics: [{"type": "spring", ...}]` inside a motion spec keep getting motion-spec spring behavior. Authors who write `{"signal": {"type": "spring", ...}}` get signal-source spring behavior. The two are evaluated through different runtime paths (`PathType` for motion-spec, `Box<dyn Signal>` for signal-source). **Document this in the facade's mod-level rustdoc** — a recipe author should know that "spring" in a `dynamics:` array and "spring" in a `signal:` field are conceptually similar but evaluated independently. A future packet can unify them; γ does not.

- **The "each variant earns its place" curation policy is enforced by checklist, not by code.** Adding a variant requires five updates (enum arm, wrapper struct, trait arm, catalog entry, round-trip test). A contributor who skips any of the five breaks the catalog-completeness test or the round-trip test. **Document the five-point checklist in the facade's mod-level rustdoc** so the policy is visible at the natural touch point.

- **Mixed-signals constructor calls in the spatial / composition arms are placeholders.** The §Code snippets show `mixed_signals::generators::SpatialCoordinateSignal::norm_x()` and `mixed_signals::composition::Add::new(a, b)` as illustrative — the actual upstream API may use different module paths or constructor names. Verify with `ofpf-defs SpatialCoordinateSignal` and `ofpf-defs Add` against `/usr/projects/mixed-signals` before settling. If the upstream API requires a builder pattern (`Add::builder().a(...).b(...).build()`), the arm bodies adjust accordingly.

- **The facade's `Box<dyn Signal>` output couples the facade to the `Signal` trait object's stability.** If `mixed_signals::traits::Signal` adds a method, every implementer must update; the boxed trait object's method table must match. This is the standard `dyn Trait` versioning constraint and is not specific to γ. **Mitigation:** keep the `tui-vfx-recipes` Cargo.toml's `mixed-signals` dependency pinned to a specific minor version (already at `version = "0.3.0"` per Cargo.toml inspection) so a `mixed-signals` minor bump is a deliberate decision.

- **Phase α/β interaction.** If phase α ships a `signals.toml` overlay before γ, the catalog metadata strings in `vfx_recipe_signal_catalog()` should reference the same authoring hints α uses. Two sources of truth (toml + code) is the drift risk per Q5. **Recommended:** if α has shipped, audit the toml strings against the inline catalog metadata at γ's Phase 4 — flag any drift to the user and reconcile in code.

- **The `VfxRecipeSignalSpec` enum file size.** With ~17 variants, each with a one-paragraph rustdoc, the file will likely exceed the `cls_` soft limit (150 LOC) and may hit the hard limit (200 LOC). Per Q1 default this is acceptable — splitting an enum body across files is not supported by Rust. Document the soft-limit overrun in the file's CLOG with the rationale: "the consolidation is intrinsic to the enum's responsibility."

## Sequencing note

- This packet **depends on** 1.2.A `VfxBindable<T, S = Never>` being landed (✓ — shipped at commit 77d1636 in `tui-vfx-core`).
- Phases α (autogen `SIGNALS_REFERENCE.md`) and β (Core 12 cheatsheet) are doc-only and green-lit. They can land before γ, in parallel with γ, or after γ. γ does not block on them.
- This packet **unblocks** signal-facade phase δ (symmetric `VfxBindableF32` / `VfxBindableColor` family with signal-form variants pointing at `VfxRecipeSignalSpec`). δ is hard-gated on γ — until `VfxRecipeSignalSpec` exists and the recipe-side deserialization sites route through it, δ has nothing to point its `Signal` arm at.
- This packet **does not** sweep production code. The facade is recipe-deserialization-only per the Option A acceptance. Production binaries continue importing `mixed_signals::*` directly.
- This packet **does not** modify `mixed-signals`. Intention 9 stays clean — mixed-signals is extended when a new primitive is needed, not when reorganizing recipe-facing exposure.
- The handoff doc `docs/design/tui-vfx-2026-04-26-handoff-outstanding.md` should be updated to mark γ as done in the same commit that lands Phase 4.

<!-- <FILE>docs/design/tui-vfx-2026-04-26-packet-signal-facade-gamma.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
