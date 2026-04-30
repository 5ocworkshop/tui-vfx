<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_31.md</FILE> - <DESC>v3.1 CRT sampler native blocker closure plan</DESC> -->
<!-- <VERS>VERSION: 0.1.1</VERS> -->
<!-- <WCTX>After radial/wipe-corner closure: close the current CRT sampler strict-native fallbacks.</WCTX> -->
<!-- <CLOG>0.1.1: PATCH — replace process-heavy wording with durable closure-plan vocabulary.
0.1.0: INIT — define the CRT sampler native blocker closure plan.</CLOG> -->

# CRT sampler native blocker closure plan

## Task statement

Continue the v3.1 player/studio completion loop by converting the current `sampler.crt` and `sampler.crtJitter` non-deprecated `debug_recipes` native fallbacks into strict native passes without semantic loss.

Current audit after radial/wipe-corner closure:

```text
recipes=144 nativePasses=127 fallbacks=17 hardErrors=0
topUnsupported=sampler.crt:1, sampler.crtJitter:1, shader.barberPole:1, shader.diffusion:1, shader.focusField:1, shader.glistenBand:1, shader.highlighter:1, shader.radar:1
```

## Desired outcome

- `sampler.crt` and `sampler.crtJitter` render in strict native compositor mode for their current v3.1 debug fixtures.
- Output remains player-visible parity locked against `irResolved` rows and styled cells.
- Unsupported fields, graph outputs, and unsupported scopes remain explicit fallbacks with actionable diagnostics.
- Documentation and review/de-slop evidence are updated only for impacted files.

## Target recipe set

- `samplers/sampler_crt.json`
- `samplers/sampler_crt_jitter.json`

## Constraints

- Scope is v3.1 only.
- Scope is non-deprecated `debug_recipes` only.
- Do not bump the schema version; v3.1 is pre-release and not locked.
- Do not use transient packet shorthand in durable code names, field names, or report vocabulary.
- Use `/usr/projects/tui-vfx-recipes/examples/demo.rs` only as an operational player/UI/compositor oracle; do not copy its input schema.
- Use nextest for test runs.
- Keep docs, OFPF metadata, rustdocs, and vocabulary synchronized only when impacted.
- Do not read or provide `steering/ORCHESTRATION.md` to subagents.

## Implementation touchpoints

Primary native lowering/render/test files:

- `crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs`
- `crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs`
- `crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs`

Semantic references:

- `crates/tui-vfx-player/src/fnc_apply_distortion_sampler_primitives.rs`
- `crates/tui-vfx-compositor/src/types/cls_sampler_spec.rs`

Verification references:

- `scripts/k222_native_coverage_audit.sh`

## Red / green / refactor plan

1. Add strict-native CLI regressions for the two target recipes.
2. Assert native-vs-`irResolved` `rows` and `styledCells` parity at `phase_t=0.35`.
3. Add unsupported-shape rejection cases for target-specific fields, graph outputs, and non-all scopes.
4. Run targeted nextest to observe RED.
5. Implement the minimum honest source-owned/native representation needed for current fixture fields.
6. Run targeted nextest until GREEN.
7. Run native coverage audit and verify movement from 127 native passes.
8. Run format/check/clippy, full nextest, docs/API/rustdoc gates, formal review, AI de-slop, and post-de-slop regression verification.

## Acceptance criteria

- Targeted strict-native CLI tests pass.
- Native coverage audit shows the two target recipes no longer fall back.
- No hard errors are introduced.
- Native metadata remains honest: `fallbackUsed=false`, `sourceRenderMode=sourceOnly`, `nativeSourceIsolated=true` for newly supported recipes.
- Diagnostics remain actionable for unsupported graph outputs, unsupported fields, and unsupported scopes.
- Results documentation records baseline, final counters, exact blockers closed, and remaining blockers with a concrete next action.
