<!-- <FILE>steering/work-packets/66-engine-vs-recipe-player-delineation-phase3.md</FILE> - <DESC>Phase 3 of recipe-signal facade revised completion plan: delineate engine API vs recipe player in code-level rustdoc, examples, and steering; add Intention 44 as durable counter-force</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Recipe-signal facade Phase 3 — close the silent-delineation gap surfaced 2026-04-27. Engine API is direct-consumable; recipe player is a peer authoring layer; mixed-signals is the substrate. Two surfaces are intentional and meet at SignalOrFloat.</WCTX> -->
<!-- <CLOG>0.1.0: initial packet — five stories US-3.1..US-3.5 covering tui-vfx lib.rs rustdoc delineation, recipe facade rustdoc, direct-API signal example, proposal doc closeout, and Intention 44.</CLOG> -->

# 66 — engine-vs-recipe-player delineation (Phase 3)

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
- `/usr/projects/tui-vfx/crates/tui-vfx/examples/pipeline_effects_showcase.rs` (existing direct-API example pattern; the new example mirrors its style)
- `/usr/projects/tui-vfx/examples/pipeline_effects_showcase.rs` (workspace-level example referenced from `cargo run -p tui-vfx --example`)
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
- `/usr/projects/tui-vfx/Cargo.toml` if needed to register the new example (verify before editing; `cargo run -p tui-vfx --example direct_api_signal_strength` should auto-discover from `examples/`).
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
- The two surfaces meet at `SignalOrFloat`-typed engine fields (and `BindableValue::Signal(SignalOrFloat)` per Decision 2A).

Style: per Intention writing-style. No marketing voice. No grandiose framing. One idea per sentence. Be specific about types.

Bump VERS minor; one-line CLOG.

Acceptance:

- `cargo doc -p tui-vfx --no-deps` clean.
- New rustdoc reads coherently as the entry point for a developer arriving cold at the engine.
- The delineation appears in the rendered docs alongside the existing crate-architecture table.

### US-3.2 — `tui-vfx-recipes::lib.rs` and `signals/mod.rs` rustdoc

Update `tui-vfx-recipes/src/lib.rs` module-level rustdoc to state:

- This crate is the recipe authoring + deserialization layer. It parses recipe JSON and produces engine-native types (`tui_vfx::*`) for `render_pipeline()` to render.
- Direct-API consumers (those constructing engine specs in Rust) do not need this crate; they should depend on `tui-vfx` and `mixed_signals` directly.
- The crate's public surface is the loader, validator, probe/trace tooling, and the canonical playback-item builder.

Update `tui-vfx-recipes/src/signals/mod.rs` module-level rustdoc to state:

- The `signals` module is a recipe-JSON deserialization seam. `VfxRecipeSignalSpec` is a curated subset of `mixed_signals::*` primitives reachable from recipe JSON.
- Direct-API consumers should `use mixed_signals::*` directly. The facade does not exist for them.
- The facade's curation policy: each variant is a deliberate decision; new mixed-signals primitives do not auto-expose. Adding a variant requires the enum arm, the wrapper, the dispatch arm, the catalog entry, and a round-trip test.
- Production effect code (filters, masks, samplers, shaders) keeps importing `mixed_signals::Signal` and calling `.sample_with_context(...)`. They do not know the facade exists.

Bump VERS minor on both files; one-line CLOG each.

Acceptance:

- `cargo doc -p tui-vfx-recipes --no-deps` clean.
- A reader of `signals/mod.rs` rustdoc understands the recipe-only scope and the curation contract.

### US-3.3 — Direct-API signal example

Add `/usr/projects/tui-vfx/examples/direct_api_signal_strength.rs`. Pattern: mirror `pipeline_effects_showcase.rs`. Demonstrate:

- Constructing a `FilterSpec` with a `SignalOrFloat::Signal(...)` parameter (e.g. `FilterSpec::Vignette { strength: SignalOrFloat::Signal(SignalSpec::Sine { frequency: 0.5, amplitude: 0.3, offset: 0.5, phase: 0.0 }), radius: SignalOrFloat::Static(0.6) }`).
- Calling `render_pipeline()` directly with a constructed `CompositionOptions` and reading a few frames at different `t` values.
- Printing each frame so the example output is human-readable like `pipeline_effects_showcase`.

The example doubles as documentation: a developer who reads it learns the direct-API signal usage pattern they could not learn from existing examples.

Acceptance:

- `cargo run -p tui-vfx --example direct_api_signal_strength` runs and prints frames showing visible signal-driven variation across `t`.
- `cargo build --workspace` in tui-vfx clean.
- Example file carries OFPF metadata envelope.

### US-3.4 — Closeout pass on the proposal doc

Update `docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md` to v0.5.0:

- Bump VERS, update WCTX, append one-line CLOG.
- Update the "Status snapshot" table to mark Phase γ ✅ Complete (after Phase 1) and δ-renamed-to-Phase-2 ✅ Complete (after Phase 2).
- Update §9 to reflect ship state. The plan stays for historical reference but the status changes from "planned" to "shipped" with the relevant packet IDs (64/65/66) recorded.
- Cross-link Intention 44.

Acceptance:

- Doc reads as a coherent record: shipped status, packet history, current architecture.
- Markdown lints clean (existing lint config).

### US-3.5 — Intention 44 in steering/INTENTIONS.md

Add Intention 44 (or next free number after audit; verify by reading the existing file end). Suggested content:

> **44. Recipe-JSON signal authoring goes through `VfxRecipeSignalSpec`; engine direct-API consumers use `mixed_signals::*` directly.**
>
> The two surfaces are intentional and meet at `SignalOrFloat`-typed engine fields. The facade at `tui_vfx_recipes::signals::VfxRecipeSignalSpec` is the curated, recipe-JSON deserialization seam in front of `mixed_signals::*`. Direct-API consumers — those constructing `FilterSpec`, `MaskSpec`, `SamplerSpec` in Rust — depend on `tui-vfx` and `mixed_signals` directly. The facade does not exist for them.
>
> Rules:
>
> 1. **Recipe deserialization seams use the facade.** Every JSON-deserialized signal expression in `tui-vfx-recipes` routes through `VfxRecipeSignalSpec`. Engine-native types (`SignalOrFloat`, `Box<dyn Signal>`) are produced at the seam, not directly from JSON.
> 2. **Engine field types stay engine-native.** `FilterSpec.factor: SignalOrFloat`, `BindableValue::Signal(SignalOrFloat)`, and similar engine field types remain. The facade lives one layer above the engine.
> 3. **Adding a recipe-author variant is a deliberate decision.** New `mixed_signals` primitives do not auto-expose. Each variant requires an enum arm, a wrapper or inline struct, a dispatch arm, a catalog entry, a round-trip test, and consideration of whether it earns its place per Intention 24.
> 4. **Strict-contracts validator enforces the consolidation.** A recipe authoring an upstream-only `SignalSpec` discriminant (one not exposed by the facade) fails strict validation. Mechanical drift-prevention per Intention 25.
> 5. **Direct-API examples use `mixed_signals::*` directly.** Examples in `tui-vfx/examples/` show signal usage through `SignalOrFloat::Signal(SignalSpec::...)`, not through the facade.
> 6. **`Binding(String)` is orthogonal.** Host-supplied runtime values flow through `RuntimeBindings`. Both authoring paths can use them; this rule does not constrain bindings.
>
> Why: a single recipe-author surface lets us swap, plug-in, exposure-limit, rename, or remap signals without touching recipes or examples; lets us attach authoring metadata once; and lets the validator enforce the curation invariant. The two-surface design is intentional because the audiences (Rust developers writing direct-API code; recipe authors writing JSON) have different ergonomic needs (type safety vs themability/AI-authoring/visibility).

Bump `INTENTIONS.md` VERS; one-line CLOG. Update the top-of-file "Top-of-mind intentions" paragraph if Intention 44 is load-bearing enough to belong there.

Acceptance:

- INTENTIONS.md reads coherently with Intention 44 inserted in numerical order.
- The new intention follows the existing voice and structure (rules block + Why block + What this is *not* saying — optional but matches existing pattern).
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
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
