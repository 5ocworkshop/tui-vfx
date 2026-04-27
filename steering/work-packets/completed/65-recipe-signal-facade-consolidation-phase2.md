<!-- <FILE>steering/work-packets/65-recipe-signal-facade-consolidation-phase2.md</FILE> - <DESC>Phase 2 of recipe-signal facade revised completion plan: route every recipe-JSON signal-expression deserialization site through VfxRecipeSignalSpec; engine field types stay engine-native; Decision 2A for BindableValue</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>Recipe-signal facade Phase 2 — mid-execution architectural redesign: VfxRecipeSignalSpec collapses from a parallel enum + 45 transparent-wrapper structs to a thin newtype around mixed_signals::SignalSpec with a catalog-checked custom Deserialize. Single wire format, single type at the seam, single substrate; the catalog stays as the curation gate.</WCTX> -->
<!-- <CLOG>0.3.0: record the mid-packet redesign — VfxRecipeSignalSpec is now `pub struct VfxRecipeSignalSpec(pub mixed_signals::SignalSpec)` with catalog-gated Deserialize; 45 transparent wrappers moved to recyclebin/. Phase γ's wire-format-parity assumption (parallel enum + struct wrappers) failed against the recipe corpus on Keyframes shape mismatch and missing optional-field defaults; the redesign restores wire-format identity with the engine substrate.
0.2.0: gate-satisfaction header + dated status; audit US-2.2 pre-resolved (only V3LoopbackValue is JSON-deserialized; rest are code-constructed); strict-contracts gate (US-2.5) re-framed as positive curation check now that mixed-signals/recipe catalogs are 1:1 isomorphic.
0.1.0: initial packet — seven stories US-2.1..US-2.7 covering V3LoopbackValue migration, recipe-side SignalOrFloat audit + adapter, strict-contracts validator gate, cross-repo audit, and verification.</CLOG> -->

# 65 — recipe-signal facade consolidation (Phase 2)

## Mid-execution redesign (2026-04-27, recorded post hoc)

Phase γ's design — a parallel facade enum (`pub enum VfxRecipeSignalSpec { Sine(VfxRecipeSineSpec), ... }`) plus 45 `#[serde(transparent)]` wrapper structs over `mixed_signals::*` types — was the original deserialization seam this packet was scoped against. While executing US-2.1 the corpus regression test surfaced two structural problems that no per-recipe edit could fix:

1. The transparent wrappers wrap upstream **structs** (`pub struct VfxRecipePerlinSpec(pub mixed_signals::noise::PerlinNoise)`), but the per-field `#[serde(default = "...")]` annotations live on `mixed_signals::SignalSpec`'s **enum arms**, not on the structs. Recipes that omit optional fields (e.g. `offset` on `perlin`) parse cleanly through `SignalSpec` but fail through the wrappers.
2. Even when all fields are provided, `VfxRecipeSignalSpec` and `mixed_signals::SignalSpec` are not 1:1 wire-format equivalent for every variant. `SignalSpec::Keyframes::keyframes: Vec<(f32, f32)>` ≠ `mixed_signals::generators::Keyframes::keyframes: Vec<Keyframe { time, value }>`. Round-trip lowering through JSON breaks on this and possibly other variants.

Decision (with packet-owner agreement): collapse the facade to a thin newtype with a catalog-checked custom `Deserialize`:

```rust
pub struct VfxRecipeSignalSpec(pub mixed_signals::SignalSpec);

impl<'de> Deserialize<'de> for VfxRecipeSignalSpec {
    // buffer the value, read "type" discriminant, reject if not in
    // vfx_recipe_signal_catalog(), else delegate to SignalSpec's own serde.
}
```

Effects:

- Wire format identical to `mixed_signals::SignalSpec` by construction. No parity debt.
- Curation gate moves from the type system (variant set) to `Deserialize` (catalog membership). Same outcome — uncatalogued discriminants are rejected at the JSON boundary.
- `into_signal_or_float()` and `into_recipe_signal()` become one-liners.
- 45 transparent-wrapper struct files moved to `recyclebin/src/signals/`.
- Catalog (`vfx_recipe_signal_catalog`) is unchanged and stays load-bearing for: docs (RECIPE_SIGNALS_REFERENCE.md from packet 01), the strict-contracts validator gate, and the new `Deserialize` gate.
- Stories US-2.1 (V3LoopbackValue migration), US-2.3 (signal_or_float adapter), and US-2.5 (strict-contracts gate) all completed under the redesign with the same end-state acceptance criteria. US-2.2 audit (only `V3LoopbackValue` is a JSON-deser site) and US-2.6 cross-repo audit are unaffected.

Single-path framing: recipes have one wire format (catalog-curated subset of `SignalSpec`), one type at the seam (`VfxRecipeSignalSpec`), one substrate (`mixed_signals::SignalSpec`), one curation gate. The facade is now a thin gate, not a parallel reality.

## Status as of 2026-04-27

Both upstream gates are satisfied; this packet is unblocked.

- `mixed_signals::types::signal_spec.rs` v2.7.0 (commit `c551ad0` — "Close SignalSpec envelope/physics coverage gap") closes the substrate-side coverage gap, exposing 7 physics primitives (Spring, Bounce, Pendulum, Projectile, Orbit, Decay, Attractor) and 2 absolute-time decay envelopes (LinearDecay, ExponentialDecay) through `Serialize + Deserialize + Signal`. The recipe-side facade no longer carries variants the upstream substrate cannot represent.
- `tui-vfx-recipes` Phase 1 (commit `d480d32` — "Phase 1: close 15-variant gap in VfxRecipeSignalSpec facade") brings `VfxRecipeSignalSpec` to full 58-variant coverage in 1:1 correspondence with `mixed_signals::SignalSpec`. The catalog (`vfx_recipe_signal_catalog()`) and dispatch (`fnc_into_recipe_signal.rs` v0.3.0) match the facade enum exactly; the `catalog_completeness` test enforces this invariant.

Audit findings (pre-resolved for US-2.2 — see scope adjustments below):

- `src/v3/authoring/enum_v3_loopback_value.rs` is the only JSON-deserialized signal expression site that still imports `mixed_signals::SignalSpec` directly. **The single material migration target.**
- `src/v3/compile/fnc_build_composition_spec_from_compiled_plan.rs:634` already routes through `VfxRecipeSignalSpec` (Phase γ).
- `src/preview/fnc_derive_cursor_paint_ops_from_progress.rs`, `src/manager/fnc_populate_effects.rs`, `src/loopback/cls_loopback_declaration.rs`, `src/loopback/enum_loopback_value.rs`, `src/v3/authoring/cls_v3_binding_declaration.rs`, `src/v3/compile/fnc_compile_loopback_declarations.rs` all use `SignalOrFloat` only as a code-constructed value (literal `default:` lifting, `SignalOrFloat::Static(...)` initialisers, lowering helpers). None of them deserialize a signal expression from JSON. **Not migration targets.**

Strict-contracts framing change: with mixed-signals and the recipe catalog now 1:1 isomorphic, the validator's job is no longer "reject upstream-only discriminants that the facade has not exposed yet" (the empty set, post-Phase-1). It is the positive curation gate: every `{"signal": {"type": "<discriminant>"}}` shape in a normalized recipe must carry a discriminant present in `vfx_recipe_signal_catalog()`. This catches typos, fabricated discriminants, and any future drift if mixed-signals adds a new SignalSpec variant before the recipe catalog is updated.

## Task first

Make `tui_vfx_recipes::signals::VfxRecipeSignalSpec` the single deserialization seam for every recipe-JSON signal expression. Engine field types stay engine-native. Direct-API consumers do not change.

## Why this matters

Today the facade governs exactly one deserialization site (`v3/compile/fnc_build_composition_spec_from_compiled_plan.rs:635`). Every other recipe-authored field that accepts a signal still goes through upstream `mixed_signals::SignalSpec` directly — `V3LoopbackValue::Signal(SignalSpec)`, recipe-side `SignalOrFloat::Signal(SignalSpec)` fields, and so on. That means the facade's curation policy (deliberate-add per Q3) is unenforced across ~95% of the recipe-authored signal surface. The headline maintenance lever from §8.2 — locally-named/scoped interface point for swaps, plug-ins, exposure-limiting, rename/remap — applies to one field.

Phase 2 fixes that without touching engine field types. The recipe layer deserializes through the facade and lowers to `SignalOrFloat` / `Box<dyn Signal>` at the seam. Engine code never sees the facade. Direct-API consumers never see the facade. Only recipe-side deserialization moves.

Companion: `docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md` v0.4.0 §9.3.

## Success condition

By the end of this packet:

- Every recipe-JSON deserialization site that accepts a signal expression in `tui-vfx-recipes` routes through `VfxRecipeSignalSpec`.
- `V3LoopbackValue::Signal(SignalSpec)` is `V3LoopbackValue::Signal(VfxRecipeSignalSpec)`; lowering at `to_signal_or_float()` produces an equivalent `SignalOrFloat`.
- A reusable recipe-deserialization adapter accepts `<number>` or `{"signal": <VfxRecipeSignalSpec>}` and lowers to engine-native `SignalOrFloat`.
- The strict-contracts validator rejects a recipe that authors an upstream-only `SignalSpec` discriminant (e.g. one not yet exposed by the facade).
- `BindableValue::Signal(SignalOrFloat)` remains engine-native (Decision 2A); recipe-side construction lowers at the seam.
- Cross-repo `cargo build --workspace` clean for tui-vfx, tui-vfx-recipes, mixed-signals, gt-design.
- gt-design integration tests pass; `pipeline_effects_showcase` example output is unchanged.
- Recipe corpus validates clean under strict contracts.
- progress.txt updated with audit evidence per Intention 41.

## Mode

BLOCKER_MODE.

## Gate

Phase 1 (packet 64) must be **green and committed** before this packet starts. Without the full 58-variant facade, this consolidation regresses recipe-author capability.

**Status (2026-04-27):** Gate satisfied. `tui-vfx-recipes` commit `d480d32` lands the 15-variant gap close (catalog 43→58); the upstream substrate is also in place via `mixed-signals` commit `c551ad0` (SignalSpec coverage normalization Phase A).

## Task-scope paths for grounding

- `/usr/projects/tui-vfx/docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md` (v0.4.0 §9.3)
- `/usr/projects/tui-vfx-recipes/src/v3/authoring/enum_v3_loopback_value.rs` (the migration target)
- `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_build_composition_spec_from_compiled_plan.rs` (existing facade caller; unchanged)
- `/usr/projects/tui-vfx-recipes/src/loopback/cls_loopback_declaration.rs` (recipe-side `SignalOrFloat` consumer)
- `/usr/projects/tui-vfx-recipes/src/loopback/enum_loopback_value.rs`
- `/usr/projects/tui-vfx-recipes/src/loopback/fnc_evaluate_loopback.rs`
- `/usr/projects/tui-vfx-recipes/src/loopback/fnc_merge_loopback_params.rs`
- `/usr/projects/tui-vfx-recipes/src/preview/fnc_derive_cursor_paint_ops_from_progress.rs` (recipe-side `SignalOrFloat`)
- `/usr/projects/tui-vfx-recipes/src/manager/fnc_populate_effects.rs` (recipe-side `SignalOrFloat`)
- `/usr/projects/tui-vfx-recipes/src/v3/authoring/cls_v3_binding_declaration.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_compile_loopback_declarations.rs`
- `/usr/projects/tui-vfx-recipes/src/signals/` (the facade)
- `/usr/projects/mixed-signals/src/types/signal_or_float.rs` (engine substrate; do not modify)
- The strict-contracts validator entrypoint (locate via `ofpf-defs` for `validate_strict_contracts` or similar — Phase 2 owner identifies before US-2.5).

## Exact write scope

Audit-driven; refine after US-2.2 lands the per-site list. Initial scope:

- `/usr/projects/tui-vfx-recipes/src/v3/authoring/enum_v3_loopback_value.rs` (US-2.1)
- New file: `/usr/projects/tui-vfx-recipes/src/signals/fnc_signal_or_float_adapter.rs` (US-2.3) — recipe-deserialization adapter producing `SignalOrFloat` from `<number>` or `{"signal": <VfxRecipeSignalSpec>}`.
- `/usr/projects/tui-vfx-recipes/src/signals/mod.rs` (re-export the adapter; CLOG bump)
- Per-site updates from US-2.2's audit (likely subset of):
  - `/usr/projects/tui-vfx-recipes/src/loopback/cls_loopback_declaration.rs`
  - `/usr/projects/tui-vfx-recipes/src/loopback/enum_loopback_value.rs`
  - `/usr/projects/tui-vfx-recipes/src/preview/fnc_derive_cursor_paint_ops_from_progress.rs`
  - `/usr/projects/tui-vfx-recipes/src/manager/fnc_populate_effects.rs`
  - `/usr/projects/tui-vfx-recipes/src/v3/authoring/cls_v3_binding_declaration.rs`
  - `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_compile_loopback_declarations.rs`
- Strict-contracts validator entry point identified in US-2.5 (path filled in by the executor after `ofpf-defs` lookup).
- New tests under `/usr/projects/tui-vfx-recipes/tests/` or alongside touched files (test_*.rs OFPF style).
- progress.txt at packet close.

If US-2.2 surfaces additional sites, expand this list in the packet itself before editing — do not silently widen.

## Explicit out of scope

Do not widen into:

- Any engine crate (`crates/tui-vfx-*`). Engine field types stay engine-native (`SignalOrFloat`, `mixed_signals::*`).
- `BindableValue` source (`crates/tui-vfx-core/src/.../cls_vfx_bindable.rs` or equivalent). Decision 2A: keep engine-native; lower at the recipe seam. If a recipe-side construction site is currently building `BindableValue::Signal(SignalOrFloat)` from JSON via raw `SignalSpec`, replace the JSON path with `VfxRecipeSignalSpec` and lower at the seam — the `BindableValue` type does not change shape.
- Adding new variants to `VfxRecipeSignalSpec` (Phase 1 owns that; gated done before this packet starts).
- `WeightedMix` exposure (still deferred).
- Any `mixed_signals` change.
- `pipeline_effects_showcase.rs` and other direct-API examples.
- Recipe corpus content (existing `.json` recipes). The schema change is wire-format-compatible: the discriminants and field shapes match upstream `SignalSpec` for the migrated variants. Validate the corpus; do not rewrite recipes.
- gt-design source modifications.

## Must-read docs in order

1. `/usr/projects/tui-vfx/steering/INTENTIONS.md` (Intentions 1, 2, 3, 9, 24, 25, 26, 41)
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md` (if present)
3. `/usr/projects/tui-vfx/steering/OFPF-TOOLS.md`
4. `/usr/projects/tui-vfx/docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md` (v0.4.0 §9.1, §9.3)
5. `/usr/projects/tui-vfx/steering/work-packets/64-recipe-signal-facade-completion-phase1.md` (Phase 1 closeout report — confirms the 58-variant baseline)
6. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`

## Repo-boundary guardrails

- `tui-vfx-recipes` owns recipe truth — this packet operates entirely inside it (plus cross-repo build verification).
- `tui-vfx` engine field types do not change.
- `mixed_signals::SignalOrFloat` continues to be the engine substrate; recipe-side adapter produces it.
- The strict-contracts validator stays in tui-vfx-recipes (per Intention 5: load → substitute → resolve → build is the loader's contract; validation belongs at the load boundary).

## Pipeline-touch definition of done

Not pipeline-touch. Same exclusion applies as Phase 1.

Required hygiene:

- Update rustdocs for every public item that changes shape (Intention 12).
- Align with canonical V3 vocabulary (Intention 32).
- One-line `<CLOG>` entries.

## Stories

### US-2.1 — Migrate `V3LoopbackValue::Signal(SignalSpec)` to facade

Update `enum_v3_loopback_value.rs`:

- Change `Signal(SignalSpec)` → `Signal(VfxRecipeSignalSpec)` (both the public enum and the lenient-deserialize wire enum).
- Update `to_signal_or_float()` to lower via `into_recipe_signal()` and wrap in `SignalOrFloat::from(...)` or equivalent. Document the lowering pattern in the function rustdoc.
- Update existing tests in the file (lines around 134, 161, 179) to use `VfxRecipeSignalSpec` constructors.
- Bump VERS minor; one-line CLOG.

Acceptance:

- `enum_v3_loopback_value.rs` compiles.
- All existing tests in the file pass.
- New round-trip test: a JSON `{"signal": {"type": "spring", ...}}` deserializes through `V3LoopbackValue::Signal(VfxRecipeSignalSpec::Spring(...))` and lowers to a working `SignalOrFloat`.
- New round-trip test: a JSON `{"signal": {"type": "gaussian_noise", ...}}` (a Phase 1 addition) deserializes correctly.

### US-2.2 — Audit recipe-side `SignalOrFloat`-typed fields

Use `ofpf-content` and `ofpf-refs` to enumerate every site in `tui-vfx-recipes` where a `SignalOrFloat` field is populated by JSON deserialization (vs constructed in code).

Inputs to gather per site:

- File path + line.
- Field type and shape (`SignalOrFloat`, `BindableValue`, etc.).
- Whether the field is recipe-deserialized (read from JSON) or constructed in code.
- Whether the deserializer is direct serde or a custom adapter.

Output: a table in progress.txt with per-site classification. Sites populated by JSON deserialization are migration targets for US-2.4. Sites constructed in code are not migration targets.

Acceptance:

- Audit table recorded in progress.txt.
- Each migration target named explicitly with file:line.
- Each non-target named with the reason it is excluded.

### US-2.3 — Implement the recipe-deserialization adapter

Add `signals/fnc_signal_or_float_adapter.rs` with a public function (or struct + Deserialize impl) that:

- Accepts JSON shapes `<number>` (lenient bare-literal) or `{"signal": <VfxRecipeSignalSpec>}` or `{"static": <number>}` (matching V3LoopbackValue's lenient pattern).
- Lowers to engine-native `mixed_signals::SignalOrFloat`:
  - bare number / `{"static": N}` → `SignalOrFloat::Static(N)`
  - `{"signal": <VfxRecipeSignalSpec>}` → `into_recipe_signal()` → `SignalOrFloat::from(Box<dyn Signal>)` (or whatever the upstream conversion is).
- Documented signature accepting a `serde_json::Value` or implementing a `Deserialize` adapter pattern reusable across sites.

Per Intention 24, ensure the adapter earns its place: it must clearly reduce per-site code over hand-written serde at each migration target. Name and signature reviewed before US-2.4 applies it.

Acceptance:

- Adapter file compiles.
- Adapter has unit tests covering: bare number, `{"signal": ...}` with each of three representative variants (one per family, including a Phase 1 addition like `gaussian_noise`), `{"static": N}`, and rejection of malformed shapes.
- Re-exported from `signals/mod.rs`.

### US-2.4 — Apply the adapter at every recipe-deserialization site

For each migration target in US-2.2's audit:

- Replace the existing `SignalOrFloat` deserializer with the US-2.3 adapter.
- Verify the wire format does not change for existing recipe corpus content (JSON shapes preserved).
- Update file rustdoc / comments to point at the adapter.

Acceptance:

- Each migration target uses the US-2.3 adapter.
- `cargo test -p tui-vfx-recipes` passes.
- Recipe corpus deserialization is wire-format-compatible (verified in US-2.7).

### US-2.5 — Strict-contracts validator gate

Add a check in the strict-contracts validator (entry point: `src/v3/validate/fnc_validate_normalized_recipe.rs::validate_normalized_recipe_strict_contracts` — confirmed via `ofpf-content "strict_contracts"`):

- A recipe authoring `{"signal": {"type": "<discriminant>"}}` where `<discriminant>` is **not** a member of `vfx_recipe_signal_catalog()` fails strict validation with a clear error message.
- Error message names the offending discriminant and points at the catalog.
- Non-strict (lenient) mode tolerates unknown discriminants per existing behavior; only strict mode rejects.

Per Intention 25 (hunt for infrastructure wins), this is the mechanical drift-prevention gate that keeps the consolidation invariant once it lands. Without this check, a future contributor could introduce a fresh `SignalSpec`-typed field, bypass the facade, and break the consolidation silently.

**Framing note (Phase 2 scope refresh, 2026-04-27):** post-Phase-1, `vfx_recipe_signal_catalog()` and `mixed_signals::SignalSpec` are 1:1 isomorphic, so the "upstream-only discriminant" failure mode this gate originally targeted is the empty set today. The gate's actual job is forward-looking: catch typos, fabricated discriminants, and any future drift between the upstream substrate and the recipe catalog. Implement positively — "every signal-shaped object's `type` must be in the catalog" — not negatively.

Acceptance:

- Validator unit test: a recipe with a known facade discriminant validates clean.
- Validator unit test: a recipe with a fabricated unknown discriminant fails strict validation with the documented error message.
- Validator unit test: same fabricated discriminant validates clean in non-strict mode.

### US-2.6 — Cross-repo audit (Intention 41)

Per Intention 41:

```bash
for repo in /usr/projects/tui-vfx /usr/projects/tui-vfx-recipes /usr/projects/mixed-signals /usr/projects/gt-design; do
  for sym in V3LoopbackValue VfxRecipeSignalSpec into_recipe_signal vfx_recipe_signal_catalog; do
    count=$(rg -l "$sym" "$repo" 2>/dev/null | wc -l)
    echo "$repo $sym: $count"
  done
done
```

Per-repo expectations:

- tui-vfx: 0 (the facade lives in recipes; engine never imports it).
- tui-vfx-recipes: many (the change-set repo).
- mixed-signals: 0.
- gt-design: 0 direct usage of the facade. gt-design's recipe consumption goes through tui-vfx-recipes; the migration is invisible.

Verify each external impl (if any) of `V3LoopbackValue` still compiles. Run `cargo build --workspace` in each repo.

Acceptance:

- Audit table recorded in progress.txt.
- All four repos `cargo build --workspace` clean.
- gt-design integration tests under `gtd-factory/tests/` pass.

### US-2.7 — Verification

Run in this order:

1. `cargo test -p tui-vfx-recipes` (unit + integration).
2. `cargo build --workspace` in tui-vfx-recipes.
3. Recipe corpus validation: run the strict-contracts validator over `recipes/` and `debug_recipes/`. Confirm no recipes regress.
4. `cargo build --workspace` in tui-vfx, gt-design, mixed-signals.
5. Run gt-design's `gtd-factory` integration tests.
6. Run the `pipeline_effects_showcase` example (`cargo run -p tui-vfx --example pipeline_effects_showcase`); diff output against pre-migration baseline. Expect byte-identical (no behavior change for direct-API consumers).
7. `cargo clippy --workspace --all-targets -- -D warnings` clean.

Acceptance:

- All commands pass.
- Showcase example output unchanged.
- progress.txt updated with timestamps + outcomes.

## Test-shape requirements

This packet touches schema/parser behavior. Required coverage per OFPF + COMMON_EXECUTION_RULES:

- **Accepted minimal form** — `{"signal": {"type": "sine"}}` works through every migrated site.
- **Rejected unknown nested fields** — verify `deny_unknown_fields` behavior at the facade layer; document outcomes in test comments.
- **Defaulted omitted fields** — adapter accepts bare numbers as `Static`; verify default behavior.
- **Validation boundary errors** — strict-contracts validator rejects unknown discriminants (US-2.5).
- **Typed propagation into compiled IR** — a recipe loaded through the migrated path produces an equivalent `SignalOrFloat` to the pre-migration path; verify with a round-trip diff.

This packet also touches runtime behavior:

- Confirm preview/probe entrypoints do not silently drop signal expressions through the migrated sites. Grep `pipeline-validator --probe`, `recipe-probe`, and `preview` paths; if any of them now-break, that is a blocker.

## Hot-path watchpoints

- The adapter (US-2.3) runs once per recipe load, not per frame. Allocation cost is acceptable.
- The strict-contracts validator (US-2.5) runs at load time only.
- The migrated `V3LoopbackValue` lowering runs once per binding declaration; not on a hot path.

If a profile shows the migrated path on a hot frame loop, escalate to leader before optimizing — the adapter is supposed to be load-time only.

## Verification required

```bash
# In tui-vfx-recipes:
cd /usr/projects/tui-vfx-recipes
cargo build -p tui-vfx-recipes
cargo test -p tui-vfx-recipes
cargo build --workspace
cargo clippy --workspace --all-targets -- -D warnings

# Recipe corpus validation (locate the strict-contracts entry point):
# cargo run -p pipeline-validator -- --rules --strict-contracts (adjust per as-built CLI)

# Cross-repo build (Intention 41):
cd /usr/projects/tui-vfx && cargo build --workspace
cd /usr/projects/gt-design && cargo build --workspace
cd /usr/projects/mixed-signals && cargo build --workspace

# gt-design integration test:
cd /usr/projects/gt-design && cargo test -p gtd-factory

# Direct-API behavior unchanged:
cd /usr/projects/tui-vfx && cargo run -p tui-vfx --example pipeline_effects_showcase > /tmp/showcase-after.txt
# diff against pre-migration baseline; expect byte-identical.

# No-landmines pre-commit:
git diff --cached | rg '^\+.*#\[allow|^\+.*#!\[allow' || echo "no new #[allow] suppressions"
```

If any command fails, report the exact failure, classify it (in-scope failure / expected downstream fallout / blocker), and stop until the leader confirms the fix path.

## Pre-commit write-scope guard (Intention 40 §5)

```bash
git diff --cached --name-only
```

Output must be in the "Exact write scope" section above. Unstage sweep-up with `git restore --staged <path>`. Stage by explicit path.

## No-landmines pre-commit check (Intention 40)

```bash
git diff --cached | rg '^\+.*#\[allow|^\+.*#!\[allow' || echo "no new #[allow] suppressions"
```

## Reporting contract

Final report includes:

- Docs-read confirmation
- 3 reflection bullets
- Audit table from US-2.2 (per-site classification)
- Cross-repo audit table from US-2.6
- Files changed (full paths)
- Commands run + pass/fail
- progress.txt update summary
- Showcase-example diff outcome (expected: empty)
- Blockers / handoff notes for Phase 3

## File metadata discipline

- `<CLOG>` one or two short lines per file.
- `<WCTX>` updated only if the file's role changed.
- `<VERS>` bumped: PATCH for internal edits, MINOR for shape changes.

## Closing task reminder

**Task:** every recipe-JSON deserialization site that accepts a signal expression routes through `VfxRecipeSignalSpec`. Engine field types stay engine-native. Decision 2A: BindableValue stays engine-native; lower at the recipe seam.

**Do not widen into:** engine crates, `BindableValue` source, new facade variants, mixed-signals, recipe corpus content, gt-design source.

<!-- <FILE>steering/work-packets/65-recipe-signal-facade-consolidation-phase2.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.3.0</VERS> -->
