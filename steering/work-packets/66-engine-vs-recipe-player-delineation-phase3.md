<!-- <FILE>steering/work-packets/66-engine-vs-recipe-player-delineation-phase3.md</FILE> - <DESC>Phase 3 of recipe-signal facade revised completion plan: delineate engine API vs recipe player in code-level rustdoc, examples, and steering; add Intention 44 as durable counter-force</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Phase 1 (commit d480d32) and Phase 2 (commit 5a0ea2b) are committed; the Phase 2 packet shipped a mid-flight redesign that collapsed the parallel-enum + 45 transparent wrappers into a thin newtype around mixed_signals::SignalSpec with a catalog-checked custom Deserialize. Phase 3 docs the as-built reality, not the proposal's original §8.3 sketch.</WCTX> -->
<!-- <CLOG>0.2.0: refresh against verified state via ofpf — record the Phase 2 newtype redesign so docs and Intention 44 describe what shipped. Reduce US-3.2 scope (most of signals/mod.rs rustdoc landed in the Phase 2 deslop pass). Correct US-3.3 to require explicit Cargo.toml example registration (examples/ paths in this workspace are not auto-discovered). Update US-3.5 Intention 44 text to reference the newtype + catalog-gated Deserialize, not wrappers + dispatch arms. Update US-3.4 closeout language to record divergence between the proposal §9.3 plan and what shipped.
0.1.0: initial packet — five stories US-3.1..US-3.5 covering tui-vfx lib.rs rustdoc delineation, recipe facade rustdoc, direct-API signal example, proposal doc closeout, and Intention 44.</CLOG> -->

# 66 — engine-vs-recipe-player delineation (Phase 3)

## Status as of 2026-04-28 (verified via ofpf-* against /usr/projects/tui-vfx and /usr/projects/tui-vfx-recipes)

Phase gates are satisfied; this packet documents the as-built post-Phase-2 reality.

- **Phase 1 (packet 64) committed** as `d480d32` — "Phase 1: close 15-variant gap in VfxRecipeSignalSpec facade". Catalog grew 43 → 58.
- **Phase 2 (packet 65) committed** as `5a0ea2b` — "Collapse VfxRecipeSignalSpec into a catalog-gated newtype around mixed_signals::SignalSpec". Mid-flight redesign: the parallel facade enum + 45 `#[serde(transparent)]` wrappers in `src/signals/{composition,envelopes,oscillators,physics,processing,random,spatial}/` collapsed to a thin newtype `pub struct VfxRecipeSignalSpec(pub mixed_signals::SignalSpec)` with a custom `Deserialize` that buffers the JSON, reads the `"type"` discriminant, and rejects any value not in `vfx_recipe_signal_catalog()`. The 45 wrapper files moved to `recyclebin/src/signals/`.
- **Catalog ⊆ SignalSpec.** Post-redesign the catalog gates `Deserialize` rather than constraining the type system. Adding a recipe-author-reachable variant takes 3 things, not 5: (1) the variant exists in `mixed_signals::SignalSpec` with stable wire format, (2) a catalog entry in `vfx_recipe_signal_catalog`, (3) a round-trip serde test in `test_signals.rs`. The "wrapper struct + dispatch arm" steps are gone.
- **`signals/mod.rs` rustdoc already covers most of US-3.2** (the Phase 2 deslop pass landed it). Surviving public surface: `VfxRecipeSignalSpec`, `VfxIntoRecipeSignal`, `VfxRecipeSignalMeta`, `vfx_recipe_signal_catalog`, `deserialize_signal_or_float_with_facade`. US-3.2 in this packet narrows accordingly.
- **`tui-vfx::lib.rs` is at VERS 0.7.0** with comprehensive Architecture / Shadow / Framework Adapters / Prelude rustdoc that does not yet mention the engine-vs-recipe-player split — US-3.1 still applies in full.
- **`tui-vfx-recipes::lib.rs` is at VERS 0.6.0** with utilitarian rustdoc that does not mention direct-API consumers — US-3.2 (the lib.rs half) applies in full.
- **`MARKETING.md`** has no references to facade / recipe-side / direct-API / engine API; nothing to reconcile.
- **INTENTIONS.md is at v0.7.1.** Last numbered intention is `## 43`; **Intention 44 is the next free number**.
- **The proposal doc** lives at `/usr/projects/tui-vfx/docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md` v0.4.0. §9.3 documents Phase 2 as a wrapper-and-dispatch consolidation; that plan diverged. US-3.4 must record the divergence and the actual ship shape, not just bump the version.
- **Examples are not auto-discovered** in this workspace — `crates/tui-vfx/Cargo.toml` registers `pipeline_effects_showcase` with `path = "../../examples/pipeline_effects_showcase.rs"`. The new `direct_api_signal_strength` example must add a parallel `[[example]]` block. US-3.3 corrected.

## Task first

Make the engine-API-vs-recipe-player split explicit in code-level rustdoc, examples, and steering. Add Intention 44 as the durable counter-force against future drift.

## Why this matters

The codebase already supports two consumers cleanly: direct-API consumers using `tui-vfx`'s engine surface (`render_pipeline`, `CompositionSpec`, `MaskSpec`, etc.) with `mixed_signals::*` directly, and recipe-JSON authors using `tui-vfx-recipes` and the curated `VfxRecipeSignalSpec` facade. The architecture is sound — but the delineation is undocumented. A reader of `tui_vfx::lib.rs` cannot tell, from the prelude alone, whether `render_pipeline()` is a public direct-consumption entry point or only an internal target of the recipe player. Direct-API examples don't show signal usage. There is no statement that the two surfaces are intentional and meet at `SignalOrFloat`.

Phase 3 closes that gap with five docs/examples-only stories. No code shape changes. Intention 44 captures the durable rule.

Companion: `docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md` v0.4.0 §9.4.

## Success condition

By the end of this packet:

- `crates/tui-vfx/src/lib.rs` rustdoc states the engine API is a public direct-consumption surface; `tui-vfx-recipes` is a peer authoring layer; `mixed_signals` is the substrate.
- `crates/tui-vfx-recipes/src/lib.rs` and `src/signals/mod.rs` rustdoc state the facade is a recipe-JSON deserialization seam; direct-API consumers should use `mixed_signals::*` instead.
- A new direct-API example, `examples/direct_api_signal_strength.rs`, exercises a signal expression constructed in Rust and rendered through `render_pipeline()`.
- `docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md` updated to v0.5.0 reflecting Phase 1 + 2 + 3 ship state.
- `steering/INTENTIONS.md` adds Intention 44 (or next free number) covering the engine-vs-recipe-player rule.
- `cargo doc --workspace --no-deps` clean.
- `cargo run -p tui-vfx --example direct_api_signal_strength` runs and prints expected output.
- Cross-repo `cargo build --workspace` clean (no behavior change risk; verification is a sanity check).

## Mode

BLOCKER_MODE. Docs-only packet but verification still required (Intention 12 — documentation is a first-class engineering contract).

## Gate

Phases 1 (packet 64) and 2 (packet 65) **green and committed**. Phase 3 documents the post-consolidation state; running it before Phase 2 produces docs that contradict the as-built code.

Phase 3 may overlap with Phase 2 if and only if the writer is documenting Phase 2's intended end state. If Phase 2 changes course mid-flight, Phase 3 docs must be updated accordingly.

## Task-scope paths for grounding

- `/usr/projects/tui-vfx/docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md` (v0.4.0 §9.4)
- `/usr/projects/tui-vfx/crates/tui-vfx/src/lib.rs` (the engine umbrella)
- `/usr/projects/tui-vfx/crates/tui-vfx-compositor/src/lib.rs`
- `/usr/projects/tui-vfx/examples/pipeline_effects_showcase.rs` (existing direct-API example pattern; the new example mirrors its style; registered in `crates/tui-vfx/Cargo.toml` as `[[example]] path = "../../examples/pipeline_effects_showcase.rs"` — examples in this workspace live at the repo-level `examples/` dir, not inside `crates/tui-vfx/`)
- `/usr/projects/tui-vfx-recipes/src/lib.rs`
- `/usr/projects/tui-vfx-recipes/src/signals/mod.rs`
- `/usr/projects/tui-vfx-recipes/src/signals/cls_vfx_recipe_signal_spec.rs`
- `/usr/projects/tui-vfx/steering/INTENTIONS.md` (where Intention 44 lands)
- `/usr/projects/tui-vfx/steering/MARKETING.md` (cross-reference if it claims a delineation that contradicts this work)

## Exact write scope

Only edit these paths:

- `/usr/projects/tui-vfx/crates/tui-vfx/src/lib.rs` (US-3.1 — module-level rustdoc)
- `/usr/projects/tui-vfx-recipes/src/lib.rs` (US-3.2 — module-level rustdoc)
- `/usr/projects/tui-vfx-recipes/src/signals/mod.rs` (US-3.2 — module-level rustdoc)
- New file: `/usr/projects/tui-vfx/examples/direct_api_signal_strength.rs` (US-3.3)
- `/usr/projects/tui-vfx/crates/tui-vfx/Cargo.toml` — required: add a new `[[example]]` block for `direct_api_signal_strength` mirroring the existing `pipeline_effects_showcase` registration. Examples are NOT auto-discovered from the repo-level `examples/` dir in this workspace.
- `/usr/projects/tui-vfx/docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md` (US-3.4 — bump to 0.5.0; closeout)
- `/usr/projects/tui-vfx/steering/INTENTIONS.md` (US-3.5 — Intention 44; bump VERS minor; one-line CLOG)
- progress.txt at packet close.

## Explicit out of scope

Do not widen into:

- Any engine code shape change. This is a docs/examples-only packet.
- Any recipe deserialization shape change. Phase 2 owns that.
- New facade variants. Phase 1 owns that.
- New `mixed_signals` primitives.
- `CAPABILITIES_REFERENCE.md`, `SIGNALS_REFERENCE.md`, or other autogen outputs (they regenerate from source).
- gt-design documentation (their concern).
- Any `MARKETING.md` rewrite. Touch it only if a sentence directly contradicts the new delineation; otherwise leave alone.

## Must-read docs in order

1. `/usr/projects/tui-vfx/steering/INTENTIONS.md` (Intentions 1, 2, 3, 11, 12, 25, 29, 35; existing patterns for Intention numbering)
2. `/usr/projects/tui-vfx/steering/MARKETING.md` (writing style top section; specific framing of engine vs recipe layer)
3. `/usr/projects/tui-vfx/docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md` (v0.4.0 §9.1 architectural framing — the source for the delineation language)
4. `/usr/projects/tui-vfx/steering/work-packets/64-recipe-signal-facade-completion-phase1.md` (closeout report)
5. `/usr/projects/tui-vfx/steering/work-packets/65-recipe-signal-facade-consolidation-phase2.md` (closeout report)
6. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`

## Repo-boundary guardrails

- `tui-vfx` lib.rs rustdoc describes engine surface; do not promise recipe behavior here.
- `tui-vfx-recipes` lib.rs rustdoc describes the recipe-authoring surface; do not describe engine internals.
- `mixed-signals` documentation is owned by mixed-signals; this packet does not touch it. Reference it from rustdoc with intra-doc links.
- Intention 44 is project-level; it lands in `tui-vfx/steering/INTENTIONS.md`. Do not author a parallel intention in tui-vfx-recipes.

## Pipeline-touch definition of done

Not pipeline-touch. Same exclusion as Phase 1.

## Stories

### US-3.1 — `tui-vfx::lib.rs` rustdoc delineation

Update `crates/tui-vfx/src/lib.rs` module-level rustdoc to state:

- The engine API is a public, fully-supported direct-consumption surface. The crate re-exports `render_pipeline`, `CompositionSpec`, `MaskSpec`, `FilterSpec`, `SamplerSpec`, `ShadowSpec` and related primitives for direct construction in Rust.
- Recipes are a peer authoring layer in `tui-vfx-recipes`, optional for direct-API consumers. Recipes parse JSON and produce engine-native types; the engine does not depend on recipes.
- `mixed_signals` is the signal substrate. Engine field types like `factor: SignalOrFloat`, `strength: SignalOrFloat` accept full upstream `SignalSpec` shapes constructed in Rust.
- Both audiences are intentional. Direct-API consumers get type safety, IDE completion, lower per-frame cost (no JSON parse), and embedding in custom widgets. Recipe-JSON consumers get themability, hot-reload, AI-authoring, validation, and probe/trace visibility.
- The two surfaces meet at `SignalOrFloat`-typed engine fields (and `VfxBindableValue::Signal(SignalOrFloat)` per Decision 2A — note: the type was renamed from `BindableValue` to `VfxBindableValue` per Intention 8's `Vfx*` prefix rule for cross-crate types).

Style: per Intention writing-style. No marketing voice. No grandiose framing. One idea per sentence. Be specific about types.

Bump VERS minor; one-line CLOG.

Acceptance:

- `cargo doc -p tui-vfx --no-deps` clean.
- New rustdoc reads coherently as the entry point for a developer arriving cold at the engine.
- The delineation appears in the rendered docs alongside the existing crate-architecture table.

### US-3.2 — `tui-vfx-recipes::lib.rs` rustdoc (signals/mod.rs is mostly already done)

**lib.rs (write in full):** Update `tui-vfx-recipes/src/lib.rs` module-level rustdoc to state:

- This crate is the recipe authoring + deserialization layer. It parses recipe JSON and produces engine-native types (`tui_vfx::*`) for `render_pipeline()` to render.
- Direct-API consumers (those constructing engine specs in Rust) do not need this crate; they should depend on `tui-vfx` and `mixed_signals` directly.
- The crate's public surface is the loader, validator, probe/trace tooling, and the canonical playback-item builder.

Bump VERS minor on `lib.rs`; one-line CLOG.

**signals/mod.rs (verify only, with one cross-link):** As of `signals/mod.rs` v1.0.0 (Phase 2 deslop pass), the module-level rustdoc already states the recipe-only scope, the "production code keeps importing `mixed_signals::*` directly" rule, and the curation policy. Reread before editing — most of the work this story originally called for is already in place.

Two refinements remain:

1. The curation-policy block lists 3 steps (variant exists in `SignalSpec`, catalog entry, round-trip test). Confirm wording is post-redesign-correct: no mention of "wrapper struct" or "dispatch arm" since neither exists anymore.
2. After Intention 44 lands (US-3.5), add a one-line cross-link from `signals/mod.rs` to it.

Acceptance:

- `cargo doc -p tui-vfx-recipes --no-deps` clean.
- A reader of `tui-vfx-recipes/src/lib.rs` rustdoc understands the crate's role and the direct-API alternative.
- A reader of `signals/mod.rs` rustdoc understands the recipe-only scope and the post-redesign 3-step curation contract.
- `signals/mod.rs` cross-links Intention 44 once it exists.

### US-3.3 — Direct-API signal example

Add `/usr/projects/tui-vfx/examples/direct_api_signal_strength.rs`. Pattern: mirror `examples/pipeline_effects_showcase.rs` (the existing direct-API example). Demonstrate:

- Constructing an effect spec with a `SignalOrFloat::Signal(...)` parameter using `mixed_signals::SignalSpec` constructed in Rust (e.g. a Vignette or comparable filter whose actual field shape matches the as-built engine API — verify via `ofpf-defs FilterSpec` against the tui-vfx workspace before drafting the example, then pick a filter with a `SignalOrFloat`-typed strength/factor field).
- Calling `render_pipeline()` (or `render_pipeline_with_spec`, whichever the existing showcase uses) with a constructed `CompositionOptions` and reading a few frames at different `t` values.
- Printing each frame so the example output is human-readable like `pipeline_effects_showcase`.

The example doubles as documentation: a developer who reads it learns the direct-API signal usage pattern they could not learn from existing examples.

**Cargo.toml registration is required.** `crates/tui-vfx/Cargo.toml` already has `[[example]] name = "pipeline_effects_showcase"; path = "../../examples/pipeline_effects_showcase.rs"` — add a parallel block for `direct_api_signal_strength`. Examples in this workspace are not auto-discovered; without the `[[example]]` block, `cargo run -p tui-vfx --example direct_api_signal_strength` will fail with "no example target named …".

Acceptance:

- `cargo run -p tui-vfx --example direct_api_signal_strength` runs and prints frames showing visible signal-driven variation across `t`.
- `cargo build --workspace` in tui-vfx clean.
- Example file carries OFPF metadata envelope.
- `crates/tui-vfx/Cargo.toml` carries a new `[[example]]` block; bump that crate's VERS PATCH.

### US-3.4 — Closeout pass on the proposal doc

Update `docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md` to v0.5.0:

- Bump VERS, update WCTX, append one-line CLOG.
- Update the "Status snapshot" table to mark Phase γ ✅ Complete (commit `4cd6b8e`), Phase 1 ✅ Complete (commit `d480d32`), Phase 2 ✅ Complete (commit `5a0ea2b`).
- **§9.3 Phase 2 plan diverged from what shipped.** The original plan described "consolidate recipe-side signal access onto the facade" by routing `V3LoopbackValue::Signal` through `VfxRecipeSignalSpec` and adding a recipe-deserialization adapter that lowers to `SignalOrFloat`. That intent shipped, but the *facade type* itself was redesigned mid-flight: from a parallel enum `pub enum VfxRecipeSignalSpec { Sine(VfxRecipeSineSpec), Triangle(VfxRecipeTriangleSpec), … }` plus 45 transparent-wrapper structs — to a thin newtype `pub struct VfxRecipeSignalSpec(pub mixed_signals::SignalSpec)` with a custom `Deserialize` that gates the catalog. The redesign was forced by two wire-format-parity failures the original design had unintentionally introduced: (1) per-field `#[serde(default)]` annotations live on `SignalSpec`'s enum arms rather than the underlying structs, so transparent wrappers required fields the SignalSpec-shaped JSON omits; (2) `SignalSpec::Keyframes::keyframes: Vec<(f32, f32)>` and `mixed_signals::generators::Keyframes::keyframes: Vec<Keyframe { time, value }>` are not wire-format equivalent. Add a `§9.3a — what actually shipped` subsection capturing this; do not erase §9.3.
- Update §9.4 (Phase 3) to mark "in flight under packet 66".
- Cross-link Intention 44 once US-3.5 lands.

Acceptance:

- Doc reads as a coherent record: shipped status, packet history (including the Phase 2 redesign), current architecture.
- §9.3 stays as the original plan-of-record; §9.3a documents what actually shipped and why.
- Status snapshot rows carry the actual commit hashes.
- Markdown lints clean (existing lint config).

### US-3.5 — Intention 44 in steering/INTENTIONS.md

Add Intention 44. Verified via ofpf against INTENTIONS.md v0.7.1: last numbered intention is `## 43`, so `44` is the next free number. Suggested content (rewritten to describe the as-built newtype facade, not the original wrapper sketch):

> **44. Recipe-JSON signal authoring goes through `VfxRecipeSignalSpec`; engine direct-API consumers use `mixed_signals::*` directly.**
>
> The two surfaces are intentional and meet at `SignalOrFloat`-typed engine fields. The facade at `tui_vfx_recipes::signals::VfxRecipeSignalSpec` is a thin newtype around `mixed_signals::SignalSpec` with a custom `Deserialize` that gates the recipe-author catalog. Direct-API consumers — those constructing `FilterSpec`, `MaskSpec`, `SamplerSpec` in Rust — depend on `tui-vfx` and `mixed_signals` directly. The facade does not exist for them.
>
> Rules:
>
> 1. **Recipe deserialization seams use the facade.** Every JSON-deserialized signal expression in `tui-vfx-recipes` routes through `VfxRecipeSignalSpec`. The newtype's `Deserialize` rejects any `"type"` discriminant not in `vfx_recipe_signal_catalog()`; the engine substrate `mixed_signals::SignalSpec` is produced one layer below.
> 2. **Engine field types stay engine-native.** `FilterSpec.factor: SignalOrFloat`, `VfxBindableValue::Signal(SignalOrFloat)` (= `VfxBindable<f32, SignalOrFloat>::Signal(_)`), and similar engine field types remain. The facade lives one layer above the engine; lowering is a one-liner because the newtype wraps the substrate directly.
> 3. **One wire format.** `VfxRecipeSignalSpec` and `mixed_signals::SignalSpec` share the JSON shape by construction. Recipes that worked through `mixed_signals::SignalSpec` continue to work through the facade. The catalog is the only thing that diverges — and only as a *subset* of `SignalSpec` discriminants, never a parallel encoding.
> 4. **Adding a recipe-author variant is a deliberate decision.** New `mixed_signals` primitives do not auto-expose. Adding a variant requires three things: (a) the variant exists in `mixed_signals::SignalSpec` with stable wire format, (b) a `VfxRecipeSignalMeta` entry in `vfx_recipe_signal_catalog`, (c) a round-trip serde test in `test_signals.rs`. No wrapper struct, no dispatch arm — the newtype's `SignalSpec::build` delegation handles construction.
> 5. **Strict-contracts validator enforces curation.** `validate_normalized_recipe_strict_contracts` walks the recipe's raw `Value` payloads (filter / shader / sampler payloads stay as `serde_json::Value`, not typed) and rejects any `{"signal": {"type": "<x>"}}` whose `<x>` is not in `vfx_recipe_signal_catalog()`. This catches authorings that bypass the typed `Deserialize` boundary. Drift-prevention per Intention 25.
> 6. **Direct-API examples use `mixed_signals::*` directly.** Examples in `tui-vfx/examples/` show signal usage through `SignalOrFloat::Signal(SignalSpec::...)`, not through the facade.
> 7. **`Binding(String)` is orthogonal.** Host-supplied runtime values flow through `RuntimeBindings`. Both authoring paths can use them; this rule does not constrain bindings.
>
> Why: a single recipe-author entrypoint lets us swap, plug-in, exposure-limit, rename, or remap signals without touching recipes or examples; lets us attach authoring metadata once (the catalog drives `RECIPE_SIGNALS_REFERENCE.md` via the doc generator); and lets the validator enforce the curation invariant. The two-surface design is intentional because the audiences (Rust developers writing direct-API code; recipe authors writing JSON) have different ergonomic needs (type safety vs themability / AI-authoring / hot-reload / probe visibility).
>
> What this is *not* saying: it does not say the facade introduces a parallel type system or rename layer over `mixed_signals`. The facade is a *thin* newtype gate — wire format is identical to the substrate — not a re-encoding. Phase γ explored a parallel-enum + per-variant wrappers design and Phase 2 redesigned away from it because the parallel encoding leaked wire-format-parity gaps into the recipe corpus. Future contributors: do not reintroduce wrappers without weighing the parity-debt cost.

Bump `INTENTIONS.md` VERS minor; one-line CLOG. Consider the top-of-file "Top-of-mind intentions" paragraph: Intention 44 is structurally important (defines a recurring decision boundary) — author may decide to surface it there.

Acceptance:

- INTENTIONS.md reads coherently with Intention 44 inserted in numerical order at the appropriate file location (verify Part I/II split before placement).
- The new intention follows the existing voice and structure (rules block + Why block + What this is *not* saying).
- Rule 4's "three things" matches the as-built reality, not the obsolete five-point checklist from `signals/mod.rs` v0.2.0 era.
- VERS / WCTX / CLOG hygiene maintained.

## Test-shape requirements

This packet does not touch schema/parser/runtime behavior. The only "test" is the example running cleanly:

- `cargo run -p tui-vfx --example direct_api_signal_strength` produces non-empty, human-readable output with visible signal variation across frames.
- `cargo doc --workspace --no-deps` clean.

## Hot-path watchpoints

Not applicable. Docs-only packet.

## Verification required

```bash
cd /usr/projects/tui-vfx
cargo doc --workspace --no-deps
cargo run -p tui-vfx --example direct_api_signal_strength
cargo build --workspace

cd /usr/projects/tui-vfx-recipes
cargo doc --workspace --no-deps
cargo build --workspace

# No-landmines pre-commit (docs files cannot have #[allow], but the example might):
git diff --cached | rg '^\+.*#\[allow|^\+.*#!\[allow' || echo "no new #[allow] suppressions"
```

If any command fails, classify (in-scope failure / expected fallout / blocker) and stop until leader confirms.

## Pre-commit write-scope guard (Intention 40 §5)

```bash
git diff --cached --name-only
```

Output must be in the "Exact write scope" section above. Unstage any sweep-up; stage by explicit path.

## No-landmines pre-commit check (Intention 40)

```bash
git diff --cached | rg '^\+.*#\[allow|^\+.*#!\[allow' || echo "no new #[allow] suppressions"
```

## Reporting contract

Final report includes:

- Docs-read confirmation
- 3 reflection bullets (clarity wins, drift risks remaining, suggested follow-up)
- Files changed (full paths)
- `cargo doc` and example-run outputs
- Any blockers / handoff notes (e.g. Intention 44 numbering conflict if discovered during US-3.5)

## File metadata discipline

- `<CLOG>` one or two short lines.
- `<WCTX>` updated only if the file's role changed.
- `<VERS>` bumped: PATCH for internal edits, MINOR for new exports / new sections.

## Closing task reminder

**Task:** delineate engine vs recipe-player in code rustdoc, examples, and steering. Add Intention 44 as durable counter-force.

**Do not widen into:** code shape changes, MARKETING.md rewrite, autogen output edits, gt-design docs, mixed-signals docs.

<!-- <FILE>steering/work-packets/66-engine-vs-recipe-player-delineation-phase3.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
