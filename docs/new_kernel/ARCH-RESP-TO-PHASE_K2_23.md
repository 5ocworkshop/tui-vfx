<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_23.md</FILE> - <DESC>K2.23 self-generated public-demo player and studio packet</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Post-K2.22 continuation: move visible player/studio toward public-demo completeness while native effect-lane expansion continues.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — define lifecycle playback, help/status, interactive studio, and content-transform strategy gates.</CLOG> -->

# K2.23 public-demo player and studio packet

## Intent

Make the player and studio feel operational in the way the working `/usr/projects/tui-vfx-recipes/examples/demo.rs` oracle does, while staying wired to this repo's v3.1 `RecipeDocument` data model and source-isolated compositor backend.

## Required results

1. Add demo-grade playback foundations: lifecycle-aware phase timing or a documented first increment toward it, stable status messages, and reload-from-disk behavior for the active recipe.
2. Add help/status foundations: help overlay must intercept input, status must expose backend/composition/source/fallback evidence, and user actions/errors must not be overwritten every tick.
3. Start real interactive studio support: when `--studio` is active, controls must be visible and navigable/editable in the ratatui UI, not only scriptable.
4. Continue effect-lane momentum with a content-transform strategy: audit the existing player content adapters and decide the correct native backend stage for `content.*` families before implementing broad content lowering.
5. Produce user-runnable commands and tests proving the UI/player behavior.

## Constraints

- v3.1 only; do not bump schema version.
- Debug recipes only; exclude deprecated recipes.
- Do not use transient packet shorthand in public names or schema/report fields.
- Use `/usr/projects/tui-vfx-recipes/examples/demo.rs` as an oracle for behavior and ergonomics, not as an input-schema source.
- Use `cargo nextest` for tests.
- Subagents must read `.omx/context/k223-subagent-briefing-latest.md` first and must not read or receive `steering/ORCHESTRATION.md`.

## Acceptance gates

- Regressions prove help overlay interception, active recipe reload from disk, and at least one interactive studio control mutation path.
- UI status/render output includes native/source/fallback evidence when compositor backend is selected.
- Existing K2.21/K2.22 harnesses continue to pass.
- Formal review and AI de-slop are documented before closure.

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_23.md</FILE> - <DESC>K2.23 self-generated public-demo player and studio packet</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
