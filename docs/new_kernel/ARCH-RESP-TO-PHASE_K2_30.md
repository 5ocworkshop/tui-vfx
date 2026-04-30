<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_30.md</FILE> - <DESC>Self-generated v3.1 radial/wipe-corner native blocker closure packet</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Ralph continuation after vignette/mask blocker closure: attack the remaining mask strict-native fallbacks before sampler/shader work.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — define the radial and wipe-corner native blocker closure tranche.</CLOG> -->

# Radial and wipe-corner native blocker closure packet

## Task statement

Continue the v3.1 player/studio completion loop by converting the current remaining non-deprecated `debug_recipes` mask native fallbacks into strict native passes without semantic loss.

Current audit after vignette/mask closure:

```text
recipes=144 nativePasses=125 fallbacks=19 hardErrors=0
topUnsupported=mask.radial:1, mask.wipeCorner:1, sampler.crt:1, sampler.crtJitter:1, shader.barberPole:1, shader.diffusion:1, shader.focusField:1, shader.glistenBand:1
```

This packet targets the two remaining mask blockers only.

## Desired outcome

- `mask.radial` and `mask.wipeCorner` render in strict native compositor mode for their current v3.1 debug fixtures.
- Output remains player-visible parity locked against `irResolved` rows and styled cells.
- Unsupported fields, graph outputs, scopes, and unsupported enum values remain explicit fallbacks with actionable diagnostics.
- Documentation and review/de-slop evidence are updated only for impacted files.

## Target recipe set

- `masks/mask_radial.json`
- `masks/mask_wipe_corner_out_from_top_left.json`

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

- `crates/tui-vfx-player/src/fnc_apply_simple_mask_primitives.rs`
- `crates/tui-vfx-player/src/fnc_apply_mask_wipe.rs`

Verification references:

- `scripts/k222_native_coverage_audit.sh`

## Red / green / refactor plan

1. Add strict-native CLI regressions for the two target recipes.
2. Assert native-vs-`irResolved` `rows` and `styledCells` parity at `phase_t=0.35`.
3. Add unsupported-shape and invalid-enum rejection cases for target-specific fields.
4. Run targeted nextest to observe RED.
5. Implement the minimum honest source-owned/native representation needed for current fixture fields.
6. Run targeted nextest until GREEN.
7. Run native coverage audit and verify movement from 125 native passes.
8. Run format/check/clippy, full nextest, docs/API/rustdoc gates, formal review, AI de-slop, and post-de-slop regression verification.

## Acceptance criteria

- Targeted strict-native CLI tests pass.
- Native coverage audit shows the two target recipes no longer fall back.
- No hard errors are introduced.
- Native metadata remains honest: `fallbackUsed=false`, `sourceRenderMode=sourceOnly`, `nativeSourceIsolated=true` for newly supported recipes.
- Diagnostics remain actionable for unsupported graph outputs, unsupported fields, unsupported scopes, and unsupported enum values.
- Results documentation records baseline, final counters, exact blockers closed, and remaining blockers with a concrete next action.
