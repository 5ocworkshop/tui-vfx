<!-- <FILE>docs/design/tui-vfx-binding-loopback-implementation-plan.md</FILE> - <DESC>Implementation plan for the binding loopback design (companion to tui-vfx-binding-loopback.md). Phase-by-phase file lists, TDD outlines, commit boundaries, and explicit deferrals so a single end-to-end push can land L1–L5 without re-deciding architecture mid-flight.</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>L1 + L2 shipped. Update plan to reflect: L2 used the existing root-level `requires_bindings` block (not a new `config.bindings` block), Intention 37 landed mid-implementation and forced the "loopback required for every declaration" rule (no production-only carve-out), all five BindingKinds (U16/F32/Bool/String/Color) ship in L2 with non-numeric kinds carrying literal-only loopback in v1.</WCTX> -->
<!-- <CLOG>0.2.0: mark L1 and L2 as shipped with commit pointers. L2 deviations from initial plan: (a) authoring layered onto root-level `requires_bindings` rather than a new `config.bindings` block; (b) per Intention 37, `loopback` is effectively required (the validator gates `effective_loopback().is_some()` per declaration); (c) all five BindingKinds shipped in L2 (the v0.1 plan scoped only U16/F32, but the existing corpus's bool/string/color bindings forced the broader scope); (d) `loopback_declarations` lives compile-time-derivable from the contracts JSON (via `compile_loopback_declarations`) rather than as a stored field on `CompiledRecipePlan`. L3 / L4 / L5 plans unchanged.
0.1.0: Initial implementation plan covering L1 (engine fallback layer / merge function), L2 (`bindings:` block authoring + strict-contracts gate), L3 (visibility badge), L4 (strictness modes), L5 (probe + browser + demo recipes + docs).</CLOG> -->

# Binding loopback — implementation plan

Status: implementation plan. Companion to the design proposal at
[`tui-vfx-binding-loopback.md`](tui-vfx-binding-loopback.md) v0.3.0
(commit `3443102`). The design doc is the source of truth for the
WHAT and WHY; this doc is the HOW.

This plan is intentionally concrete. It is the contract for a single
end-to-end push that lands all five phases (L1 → L5) without
re-deciding architecture between commits.

## 0. Plan-the-plan ground rules

These ground rules apply to every phase below. They exist because
the loopback layer cuts across engine + recipes + schema + validator
+ docs, and each phase has temptations to over-build or under-test:

1. **TDD is mandatory.** Every new function gets a peer test file
   (`test_*.rs`) and the test is written first. If it's logic, the
   test exists before the code. No exceptions.
2. **One commit per phase.** Each phase ends with a green workspace
   test run and one well-formed commit (engine-side phases commit
   to `tui-vfx`, recipes-side phases commit to `tui-vfx-recipes`).
   No multi-phase commits, no half-finished commits.
3. **Build only what the phase needs, defer everything else.** Each
   phase has a "Deferred" section listing things that look like they
   belong in the phase but actually belong in a later one. Treat
   that section as a hard fence.
4. **Use OFPF tools for codebase exploration.** `ofpf-inspect`
   before modifying any file; `ofpf-search-content` /
   `ofpf-search-defs` before adding new symbols (avoid name
   collisions with existing types).
5. **The other dev's mechanical/odometer work is in flight.** Do
   not stage their files. Verify `git status` before each commit.
   Their files in `tui-vfx-content/src/mechanical/` and the
   `Odometer.mechanical` field are untouchable.
6. **Going slow is part of the plan.** Per owner: "Carefully,
   thoroughly, taking the time to get it right." Re-read your own
   diff before committing. Run the workspace test suite, not just
   the targeted file's tests. Fix existing CLOG entries that
   exceed 1-2 lines as you encounter them.

## 1. Phase L1 — engine fallback layer (Rust API only) — **SHIPPED**

> Status: shipped at commit `ee15dc4` (recipes). Module:
> `tui-vfx-recipes/src/loopback/`. 26 unit tests passing. Extended at L2
> commit (see section 2) to cover non-numeric kinds via the new
> `LoopbackValue` enum.

### Goal

A pure merge function that takes (host-supplied
`ShaderRuntimeParams`, recipe-declared `LoopbackDeclarations`,
frame context) and returns a merged `ShaderRuntimeParams` where:

- Every key the host supplied passes through unchanged.
- Every key the host did NOT supply but the recipe declared a
  loopback for is filled with the loopback's evaluated value.
- Keys that have neither a host value nor a loopback are absent
  from the merged map (the existing missing-binding contract
  fires downstream).

No JSON authoring yet. No badge yet. No strictness modes yet. Just
the lookup-precedence plumbing and a Rust-level API.

### Why this lives in `tui-vfx-recipes`, not `tui-vfx-style`

- The host params come from the per-recipe-item state
  (`RenderPlan.runtime_params`), populated by recipes-side code.
- The loopback declarations come from the compiled recipe.
- The merge is a recipe-construction concern, not a style/engine
  concern. The engine downstream (`BindableU16::evaluate`,
  `BindableValue::evaluate`) consumes the merged map without
  knowing or caring how it was built.
- This keeps the engine crates unchanged and avoids touching the
  serialization/Default surface of `ShaderRuntimeParams`.

### Files

New:
- `crates/tui-vfx-recipes/src/loopback/mod.rs` — module declaration.
- `crates/tui-vfx-recipes/src/loopback/cls_loopback_declaration.rs` —
  `LoopbackDeclaration { kind: BindingKind, signal: SignalOrFloat | u16-literal | f32-literal }`. Carries the per-binding contract.
- `crates/tui-vfx-recipes/src/loopback/cls_loopback_declarations.rs` —
  `LoopbackDeclarations(BTreeMap<String, LoopbackDeclaration>)`.
  Container with `iter()`, `len()`, `is_empty()`, `insert()`.
- `crates/tui-vfx-recipes/src/loopback/enum_binding_kind.rs` —
  `BindingKind { U16, F32 }`. (Future: `String`, `Seed`, `Color`.
  v1 is `U16` + `F32` only — the current bindable types.)
- `crates/tui-vfx-recipes/src/loopback/fnc_evaluate_loopback.rs` —
  `evaluate_loopback(decl, t, signal_ctx) -> Option<ShaderRuntimeParamValue>`.
  Pure per-declaration evaluator. `f32` decls evaluate to
  `ShaderRuntimeParamValue::Number`; `u16` decls evaluate the same
  signal then `round + clamp` to a `Number` storing the integer
  (the existing `as_u16` accessor handles the conversion at lookup
  time, so no new variant is needed).
- `crates/tui-vfx-recipes/src/loopback/fnc_merge_loopback_params.rs` —
  `merge_loopback_params(host: &ShaderRuntimeParams, decls: &LoopbackDeclarations, t: f64, signal_ctx: &SignalContext) -> ShaderRuntimeParams`.
  Clones host, then for each declared key NOT already in the host
  map, evaluates the declaration and inserts. Single pass, no
  re-evaluation per lookup.
- Peer tests for each `fnc_*` and `cls_*` file.

Modified:
- `crates/tui-vfx-recipes/src/lib.rs` — `pub mod loopback;` plus
  the public re-exports for `LoopbackDeclaration`,
  `LoopbackDeclarations`, `BindingKind`, `merge_loopback_params`.

### TDD test outline (write these first)

| Test | What it proves |
| --- | --- |
| `merge_returns_host_unchanged_when_no_loopback_decls` | The empty-decls case is a clean clone — no surprises. |
| `merge_inserts_loopback_value_for_missing_key` | Loopback fallback fires when host hasn't supplied. |
| `merge_does_not_overwrite_host_supplied_value` | Host wins; loopback is fallback only. |
| `merge_skips_loopback_when_signal_evaluation_returns_none` | Edge case: a malformed signal expression doesn't insert garbage. |
| `merge_evaluates_u16_loopback_via_round_and_clamp` | Float signal → u16 boundary conversion is correct (test t at 0.0, 0.5, 1.0 against a `ramp(0, 100)`). |
| `merge_evaluates_f32_loopback_directly` | f32 signals pass through without conversion. |
| `merge_evaluates_static_literal_loopback_independent_of_t` | A `static` SignalOrFloat returns the same value at every t. |
| `merge_evaluates_signal_loopback_per_frame` | Two calls at different t produce different values for a `ramp` declaration. |
| `loopback_declaration_kind_drives_evaluation_path` | A `BindingKind::U16` declaration runs through round+clamp; a `BindingKind::F32` declaration does not. |

### Commit

- Single commit on `tui-vfx-recipes` (the engine workspace is not
  touched).
- Title: `Add binding loopback merge layer (Phase L1)`
- Body: Phase-by-phase per the project's commit format. References
  the design doc and this plan.
- Verify: `cd /usr/projects/tui-vfx-recipes && cargo test --workspace --no-fail-fast` is green.

### Deferred (do NOT do in L1)

- JSON authoring of `bindings:` block — that's L2.
- Strict-contracts integration — that's L2.
- Wiring the merge into `RenderPlan` construction — that's L2.
  (L1 ships the merge function; L2 calls it.)
- Visibility badge — L3.
- Strictness modes — L4.

## 2. Phase L2 — `bindings:` block authoring + strict-contracts — **SHIPPED**

> Status: shipped (commit pending — this commit). Authoring shape lives
> on the existing root-level `requires_bindings` block (not a new
> `config.bindings` block as v0.1 of this plan proposed). All five
> BindingKinds (U16/F32/Bool/String/Color) ship in L2; non-numeric kinds
> carry literal-only loopback in v1 via the legacy `default:` field.
> Intention 37 enforces `effective_loopback().is_some()` per declaration
> at the strict-contracts gate. Render-plan wiring uses
> `CompiledV3RuntimeOverrides::with_loopback_applied` invoked at the two
> compiled-V3 render entry points.
>
> The deviations below are noted; the rest of the section is preserved
> as the as-planned text for historical reference. See the "Behavioural
> notes" block at the top of the design doc for the as-shipped contract.

### Goal

Recipe authors can declare bindings in JSON. The recipe parses,
normalizes, and compiles the block. The compiled loopback flows
into the L1 merge function during render-plan construction. The
strict-contracts validator gates that every `{"binding": "name"}`
referenced in the pipeline has a matching declaration.

### Files

New:
- `crates/tui-vfx-recipes/src/v3/authoring/cls_v3_binding_declaration.rs` —
  `V3BindingDeclaration { kind: BindingKind, description: Option<String>, loopback: Option<V3LoopbackValue> }`.
- `crates/tui-vfx-recipes/src/v3/authoring/enum_v3_loopback_value.rs` —
  `V3LoopbackValue` enum that mirrors the lenient JSON shapes
  (literal integer / literal float / `{"static": …}` / `{"signal": …}` /
  full `SignalOrFloat`). Lenient deserialize like `BindableU16Repr`.
- Peer tests.

Modified (recipes side):
- `crates/tui-vfx-recipes/src/v3/authoring/cls_v3_recipe_document.rs` —
  add `pub bindings: BTreeMap<String, V3BindingDeclaration>` field
  on the V3 config struct, `#[serde(default)]` so existing recipes
  parse unchanged.
- `crates/tui-vfx-recipes/src/v3/normalize/` — pass-through:
  add `pub bindings: BTreeMap<String, NormalizedBindingDeclaration>`
  on `NormalizedRecipe`. New normalize fn
  `fnc_normalize_binding_declarations.rs`.
- `crates/tui-vfx-recipes/src/v3/normalized/cls_normalized_recipe.rs` —
  the field above.
- `crates/tui-vfx-recipes/src/v3/compile/` — new
  `fnc_compile_loopback_declarations.rs` that converts the
  normalized declarations into L1's `LoopbackDeclarations`.
- `crates/tui-vfx-recipes/src/v3/compile/cls_compiled_recipe_plan.rs` —
  add `pub loopback_declarations: LoopbackDeclarations`.
- `crates/tui-vfx-recipes/src/rendering/fnc_render_animated_with_theme.rs`
  (or wherever `RenderPlan.runtime_params` is finalized) — call
  `merge_loopback_params(host, &compiled.loopback_declarations, t, &signal_ctx)`
  before handing off to the engine.
- `crates/tui-vfx-recipes/src/v3/validate/enum_validate_error.rs` —
  add `BindingDeclarationMissing { binding: String }` variant for
  "pipeline references `{"binding": name}` but the recipe did not
  declare it" and `BindingDeclarationInvalid { binding: String, reason: String }`
  for "declaration's loopback signal/literal is malformed."
- `crates/tui-vfx-recipes/src/v3/validate/col_validate_contracts.rs`
  (or new sibling `col_validate_binding_declarations.rs`) — strict
  contracts gate that walks every step's payload, collects every
  `{"binding": "name"}` reference, and verifies a matching
  declaration exists. Mirrors the existing
  `UndeclaredTemplateContract` machinery.
- `crates/tui-vfx-recipes/src/v3/validate/fnc_validate_normalized_recipe_strict_contracts.rs` —
  invokes the new gate.

### TDD test outline

| Test | What it proves |
| --- | --- |
| `parses_bindings_block_with_kind_description_loopback` | Full happy-path JSON parses into `V3BindingDeclaration`. |
| `parses_bindings_block_with_only_loopback_omitted` | `loopback: None` is allowed — declaration without a fallback. |
| `parses_bindings_block_with_only_description_omitted` | `description: None` is allowed but discouraged. |
| `parses_loopback_value_as_bare_integer` | Lenient back-compat for `loopback: 5`. |
| `parses_loopback_value_as_signal_ramp` | Signal-driven loopback parses. |
| `rejects_unknown_binding_kind` | `kind: "color"` (not yet supported) fails parse with a clear error. |
| `normalize_passes_bindings_through_unchanged` | Normalize layer is a pure pass-through for now. |
| `compile_produces_loopback_declarations` | Compile maps the V3 shape to L1's `LoopbackDeclarations`. |
| `merge_loopback_params_runs_in_render_plan` | End-to-end: a recipe with one declared binding and no host params produces a `runtime_params` populated by the loopback. |
| `strict_contracts_fails_on_undeclared_binding` | A pipeline `{"binding": "missing"}` with no declaration fails `--rules --strict-contracts` with `BindingDeclarationMissing`. |
| `strict_contracts_passes_when_binding_declared` | Same recipe with a matching declaration passes. |
| `strict_contracts_fails_on_invalid_loopback_signal` | A declaration with a malformed signal expression fails `BindingDeclarationInvalid`. |
| `host_supplied_value_takes_precedence_over_loopback_end_to_end` | Recipe + host param key → host wins (regression guard for L1). |

### Commit

- Single commit on `tui-vfx-recipes`.
- Title: `Add bindings: block authoring and strict-contracts gate (Phase L2)`
- Verify: workspace test green; pipeline-validator
  `--rules --strict-contracts` passes a bindings-using recipe and
  fails an undeclared-binding fixture.

### Deferred

- Visibility badge — L3.
- Strictness modes — L4.
- Probe diagnostic emit — L5.
- Demo recipes that exercise the feature end-to-end — L5.
- Hand-maintained doc updates (API_HAND, CAPABILITIES_REFERENCE,
  vocabulary) — L5.

## 3. Phase L3 — visibility badge

### Goal

Engine-side composition of the `LB` badge per the design doc
section 5. Triggered by any loopback firing during the current
frame. Solid orange, soft-edge fade, 4-cell ASCII variant
(`!LB ` or `[LB]`) by default; Nerd Font 5-cell variant
(` LB `) selectable via host config. Pulse-then-settle deferred
to v1.1 (see "Deferred" below).

### Why this lives in the engine, not the recipe browser

- Probe captures should carry the badge so probe reports show the
  same visual state as live playback.
- The compositor already owns post-pipeline overlay mechanics.
- Putting the badge in the player means every player has to
  re-implement it; engine-side is one place.

### Files

New:
- `crates/tui-vfx-compositor/src/overlays/cls_loopback_badge_overlay.rs` —
  the overlay implementation. Renders the badge into the top-right
  area of the surface. Pure given an enum (which glyph variant) and
  a flag (badge active this frame).
- `crates/tui-vfx-compositor/src/overlays/enum_loopback_badge_style.rs` —
  `LoopbackBadgeStyle { Auto, NerdFont, Ascii }` (Auto defaults to
  NerdFont for v1; in v2 Auto detects).
- `crates/tui-vfx-compositor/src/overlays/mod.rs` — module declaration.
- Peer tests.

Modified:
- `crates/tui-vfx-compositor/src/lib.rs` — `pub mod overlays;` plus
  re-exports.
- The render pipeline composes the overlay as the last step before
  returning the rendered scene. The pipeline already has a hook for
  post-render filters; use that. Find via
  `ofpf-inspect crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs`.
- `crates/tui-vfx-recipes/src/loopback/cls_loopback_declarations.rs`
  (modified from L1) — add a `frame_fired_keys: Vec<String>`
  hook so the merge function records which loopback keys fired
  during the current merge call. Engine reads this from the recipe
  side (via a small `LoopbackBadgeState` struct on `RenderPlan`)
  and decides whether to draw the badge.

### TDD test outline

| Test | What it proves |
| --- | --- |
| `badge_does_not_render_when_no_loopback_fired` | Default state is invisible. |
| `badge_renders_in_top_right_when_any_loopback_fired` | Trigger logic works. |
| `badge_glyph_variant_ascii_uses_4_cells` | Cell-count contract. |
| `badge_glyph_variant_nerd_font_uses_5_cells` | Cell-count contract. |
| `badge_orange_foreground_matches_warning_severity_color` | Color spec. |
| `badge_soft_fade_alpha_ramps_at_outer_edges` | 1-cell alpha falloff present. |
| `badge_overlay_does_not_modify_underlying_cells_when_inactive` | Inactive overlay is a no-op. |
| `merge_loopback_params_records_fired_keys` | The L1 merge function tracks fires for the badge layer. |

### Commit

- Single commit on `tui-vfx` (engine).
- Title: `Add LB visibility badge overlay for loopback fires (Phase L3)`
- Verify: workspace test green. Visual inspection deferred to L5
  via demo recipes.

### Deferred to v1.1

- Pulse-then-settle animation. Requires per-recipe-play-start
  reference for the cadence; the simplest implementation needs a
  small temporal-state field on the overlay that resets on
  recipe-start. The MVP badge is solid orange (still loud,
  still visible).
- Auto detection of Nerd Font availability. v1 Auto = NerdFont
  unconditionally; document this and let users force `Ascii` if
  they're in a non-Nerd-Font environment.

## 4. Phase L4 — strictness modes

### Goal

`LoopbackStrictness` enum on the host config:
- `Permissive` (default in browser/demo player/probe): loopback
  fires, badge appears, render continues.
- `Warn`: same as permissive plus a structured event each frame any
  loopback fires.
- `Strict`: loopback layer disabled. A missing host value falls
  through to the existing missing-binding contract. Badge **always**
  appears whenever the L1 merge would have inserted a value in
  permissive mode (so missing host wiring is loud even though the
  recipe still renders).
- `Error`: loopback disabled, missing host value is a hard render
  error.

### Files

New:
- `crates/tui-vfx-recipes/src/loopback/enum_loopback_strictness.rs` —
  the enum.
- Peer tests.

Modified:
- `crates/tui-vfx-recipes/src/loopback/fnc_merge_loopback_params.rs`
  (from L1) — accept a `strictness: LoopbackStrictness` parameter.
  In `Strict` and `Error` modes, do NOT insert loopback values, but
  DO still record `frame_fired_keys` (so the badge knows what
  WOULD have been inserted).
- `crates/tui-vfx-recipes/src/manager/` (or wherever the
  AnimationManager / renderer config lives) — add
  `with_loopback_strictness(...)` and `with_loopback_observer(...)`
  builder methods.
- `crates/tui-vfx-recipes/src/manager/orc_render_plan.rs` (or
  equivalent) — propagate the configured strictness into the merge
  call.
- `crates/tui-vfx-recipes/src/rendering/types.rs` — add the
  strictness field to `RenderPlan` (default `Permissive`).
- The engine's `LoopbackBadgeOverlay` (from L3) — read strictness
  hint from the recipe side and force-show the badge in `Strict`/`Error`
  modes regardless of whether the merge actually fired.

### TDD test outline

| Test | What it proves |
| --- | --- |
| `permissive_mode_inserts_loopback_value` | Default behavior unchanged. |
| `warn_mode_inserts_loopback_and_emits_observer_event` | Observer callback fires. |
| `strict_mode_does_not_insert_loopback_value` | Loopback layer disabled. |
| `strict_mode_records_would_have_fired_keys` | Badge layer still knows. |
| `error_mode_returns_render_error_on_missing_host_value` | Hard error on missing wiring. |
| `error_mode_passes_when_host_supplies_all_keys` | No error when wiring is correct. |
| `strictness_default_is_permissive` | Default behavior is the safe one. |

### Commit

- Single combined commit on `tui-vfx-recipes` (engine doesn't need
  changes if the badge force-show hint is just a flag on the
  overlay-input struct).
- Title: `Add LoopbackStrictness modes (Phase L4)`

### Deferred

- Per-binding strictness ("strict for key X, permissive for key Y").
  YAGNI for v1 per the design doc's open question 4.

## 5. Phase L5 — probe + browser integration + demo recipes + docs

### Goal

The mechanism is complete and visible end-to-end:

- `pipeline-validator --probe` records a `Warning` diagnostic per
  loopback fire.
- The recipe browser surfaces the binding contract for any recipe
  with a `bindings:` block (kind/description/loopback presence per
  entry).
- Three demo recipes use the mechanism to prove it works for the
  shapes Phase 3b enabled (RowRange / ColumnRange / Modulo with
  bound endpoints).
- Hand-maintained docs (API_HAND, CAPABILITIES_REFERENCE,
  vocabulary) describe the `bindings:` block authoring shape and
  the loopback contract.

### Files

Modified (probe):
- `crates/tui-vfx-probe/src/types/cls_probe_diagnostic.rs` (or
  equivalent) — add `LoopbackFire { binding: String }` diagnostic
  shape, classified `Warning`. Aggregated per-recipe-per-key
  (one diagnostic per binding key per probe run, not per frame).
- `crates/tui-vfx-probe/src/operational/` — emit the diagnostic
  when the loopback layer reports any `frame_fired_keys` during
  probe execution.
- Peer tests.

Modified (recipe browser):
- `tui-vfx-recipes/src/preview/` (whichever module surfaces the
  per-recipe summary) — add a `bindings_summary()` method on
  preview items that returns
  `Vec<(name: String, kind: BindingKind, description: Option<String>, has_loopback: bool)>`.
- Browser UI hook: out of scope for this repo; document the
  surface so downstream players can render the summary.

New (demo recipes):
- `recipes/debug_recipes/bindings/binding_row_range_synth_grid_expand.json` —
  RowRange.end driven by a ramp loopback; visualises a SynthGrid
  expand animation.
- `recipes/debug_recipes/bindings/binding_column_range_scan_reveal.json` —
  ColumnRange.start driven by a ramp loopback; visualises a
  scan-down/across reveal.
- `recipes/debug_recipes/bindings/binding_modulo_animated_stripe_density.json` —
  Modulo.modulus driven by a slow ramp loopback (period 2..6 over
  time); visualises stripe density change.

Modified (docs):
- `docs/API_HAND.md` — add a "Bindings and loopback" section under
  the StyleRegion neighborhood. Describe the `bindings:` block
  shape, loopback contract, and the strictness modes.
- `docs/CAPABILITIES_REFERENCE.md` — same coverage at a higher
  level. Cross-link to the design doc.
- `docs/design/tui-vfx-v3-recipe-vocabulary.md` — promote the
  "Runtime-binding support today" subsection from "in flux" to
  "shipped at vX.Y.Z" once L1–L4 are merged.

### TDD test outline

| Test | What it proves |
| --- | --- |
| `probe_emits_loopback_fire_warning_diagnostic` | Probe sees the fire. |
| `probe_aggregates_loopback_fires_per_key_not_per_frame` | One diagnostic per key over a probe run, not N (where N = frame count). |
| `binding_row_range_synth_grid_expand_passes_strict_contracts` | Demo recipe is well-formed. |
| `binding_row_range_synth_grid_expand_probes_clean` | Demo recipe plays without errors. |
| `binding_column_range_scan_reveal_passes_strict_contracts` | Demo recipe is well-formed. |
| `binding_modulo_animated_stripe_density_passes_strict_contracts` | Demo recipe is well-formed. |
| `bindings_summary_lists_all_declared_bindings` | Browser surface returns the contract. |

### Commit

- Two commits: one on `tui-vfx` (probe diagnostic), one on
  `tui-vfx-recipes` (browser summary, demo recipes, doc updates).
- Engine commit title: `Add LoopbackFire probe Warning diagnostic (Phase L5 engine)`
- Recipes commit title: `Add binding loopback demo recipes and docs (Phase L5 recipes)`

### Deferred

- Per-frame diagnostic emit (the v1 aggregation is per-key-per-run).
  v2 may want per-frame for hosts that care about which frame
  fired which loopback.
- Recipe browser UI rendering. The summary surface is the
  contract; the UI is a downstream responsibility.

## 6. Final acceptance test list

The work is done when ALL of the following are true:

1. `cargo test --workspace --no-fail-fast` is green on both
   `tui-vfx` and `tui-vfx-recipes` after every phase commit.
2. `pipeline-validator --rules --strict-contracts` on a
   bindings-using recipe passes; on a recipe with an undeclared
   binding fails with `BindingDeclarationMissing`.
3. `pipeline-validator --probe` on a bindings-using recipe with
   no host params reports `success` overall AND a `Warning`
   diagnostic per declared binding key.
4. The three demo recipes
   (`binding_row_range_synth_grid_expand.json`,
   `binding_column_range_scan_reveal.json`,
   `binding_modulo_animated_stripe_density.json`) play
   standalone in the demo player and visually exercise the
   bindable variants Phase 3b enabled.
5. The `LB` badge renders in the top-right of any rendered
   surface where at least one loopback fired this frame.
6. `LoopbackStrictness::Strict` disables the loopback layer and
   forces the badge to show whenever a loopback would have fired.
7. `LoopbackStrictness::Error` raises a render error when the
   host fails to supply a bound value.
8. The hand-maintained docs (API_HAND, CAPABILITIES_REFERENCE,
   vocabulary) describe the new authoring shape and contract.
9. The design doc
   (`tui-vfx-binding-loopback.md`) is bumped to v0.4.0 with the
   "Status:" line changed from "Design — not yet implemented" to
   "Status: shipped at vX.Y.Z".

## 7. Pre-flight checklist

Before starting Phase L1:

- [ ] `cd /usr/projects/tui-vfx && git status` is clean except for
  the other dev's mechanical/odometer files (verify those are
  exactly what they were before this work started).
- [ ] `cd /usr/projects/tui-vfx-recipes && git status` is clean.
- [ ] `cargo test --workspace` passes on both repos as a baseline
  (so any failure during the work is unambiguously caused by the
  loopback work, not pre-existing).
- [ ] `ofpf-load --root /usr/projects/tui-vfx` and
  `ofpf-load --root /usr/projects/tui-vfx-recipes` are both
  current.
- [ ] Re-read the design doc
  (`tui-vfx-binding-loopback.md` v0.3.0) end-to-end. The plan must
  not silently disagree with the design.

## 8. References

- [`tui-vfx-binding-loopback.md`](tui-vfx-binding-loopback.md) v0.3.0
  — the design proposal this plan implements.
- `crates/tui-vfx-style/src/traits/cls_shader_context.rs` —
  `ShaderRuntimeParams` definition (the substrate the loopback
  layer pre-fills).
- `crates/tui-vfx-style/src/models/cls_bindable_u16.rs` —
  `BindableU16` consumer (no changes; just consumes the
  loopback-augmented runtime_params).
- `crates/tui-vfx-compositor/src/types/cls_bindable_value.rs` —
  `BindableValue` consumer.
- `crates/tui-vfx-recipes/src/v3/validate/enum_validate_error.rs`
  — existing validator error variants; L2 adds two new variants
  alongside.
- `crates/tui-vfx-recipes/src/v3/validate/col_validate_contracts.rs`
  — existing strict-contracts machinery; L2 extends it.
- `crates/tui-vfx-recipes/src/rendering/fnc_render_animated_with_theme.rs`
  — the render-plan finalization point where L2 calls
  `merge_loopback_params`.
- `docs/design/tui-vfx-v3-outstanding-master-list.md` —
  V3-PROCRUNTIME01 entry that tracks the procedural pathway
  follow-on (release-blocking, not in this plan).

<!-- <FILE>docs/design/tui-vfx-binding-loopback-implementation-plan.md</FILE> - <DESC>Implementation plan for binding loopback</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
