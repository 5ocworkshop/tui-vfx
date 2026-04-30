<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_22.md</FILE> - <DESC>K2.22 self-generated native coverage and studio completeness packet</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Post-K2.21 continuation: drive player/studio toward public-demo completeness by auditing all non-deprecated v3.1 debug recipes under source-isolated native compositor mode, then implement the highest-impact missing effect/control lanes.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — define source-local native coverage audit, next lowerer expansion, and dynamic studio widget acceptance gates.</CLOG> -->

# K2.22 native coverage and studio completeness packet

## Intent

Move from bounded source-isolated native proof to broad v3.1 debug recipe playback coverage. The target corpus is exclusively non-deprecated recipes under:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

Use `/usr/projects/tui-vfx-recipes/examples/demo.rs` as a working GUI-player oracle for playback loop, focus handling, status messages, and user-facing ergonomics, while keeping this repo's player/studio wired around the v3.1 `RecipeDocument` data model and the compositor backend request boundary.

## Required results

1. Produce an audit artifact that attempts native/source-only compositor rendering for every non-deprecated v3.1 debug recipe.
2. Separate results into successful native passes, unsupported native effects, source-local fidelity issues, studio-control gaps, and hard errors.
3. Implement the highest-impact native lowerer expansion that materially reduces unsupported debug recipe count.
4. Keep Studio controls descriptor-driven. Any touched controls must expose current/default/range/allowed/mutability metadata and must support runtime mutation without recipe JSON edits.
5. Produce user-runnable commands for animated color playback and studio interaction.
6. Update impacted docs only: results, status memo, review/de-slop report, `docs/new_kernel/INDEX.md`, and vocabulary/checklist entries only if new durable terms or gates are introduced.

## Constraints

- v3.1 only; do not bump schema version.
- Debug recipes only; exclude deprecated recipes.
- Do not use transient packet shorthand in public variable names, field names, schema values, or report vocabulary.
- Use `cargo nextest` for tests.
- Preserve K2.21 gates: native mode must prove `sourceRenderMode=sourceOnly`, `nativeSourceIsolated=true`, `fallbackUsed=false` under `--fail-on-fallback`; `irResolved` must remain post-effect compatible.
- Subagents must read `.omx/context/k222-subagent-briefing-latest.md` first and must not read or receive `steering/ORCHESTRATION.md`.

## Acceptance gates

- A script or command produces a native pass/fail table for all non-deprecated v3.1 debug recipes.
- At least one previously unsupported effect family is lowered natively with regression tests and harness evidence, unless the audit proves another blocker is higher-impact and documents why.
- Studio/UI still supports the K2.21 number/color/integer/boolean/enum mutation evidence.
- Fresh verification passes: fmt, check, clippy, targeted nextest, K2.21 harness, K2.22 harness, docs gates, and workspace nextest when practical.
- Formal review and AI de-slop pass are documented before closure.

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_22.md</FILE> - <DESC>K2.22 self-generated native coverage and studio completeness packet</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
