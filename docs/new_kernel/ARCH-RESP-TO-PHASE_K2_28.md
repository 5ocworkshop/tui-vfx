<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_28.md</FILE> - <DESC>Self-generated v3.1 one-off content/filter native blocker closure packet</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Ralph continuation after residual style/content blocker closure: attack current top one-off content/filter strict-native fallbacks.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — define the one-off content/filter native blocker closure tranche.</CLOG> -->

# One-off content/filter native blocker closure packet

## Task statement

Continue the v3.1 player/studio completion loop by converting the current top non-deprecated `debug_recipes` compositor-native fallbacks into strict native passes where authored fields can be represented without semantic loss.

Current audit after residual style/content closure:

```text
recipes=144 nativePasses=109 fallbacks=35 hardErrors=0
topUnsupported=content.slideShift:1, filter.bracketEmphasis:1, filter.dotIndicator:1, filter.edgeGrow:1, filter.hoverBar:1, filter.matrixRain:1, filter.subPixelBar:1, filter.underlineWipe:1
```

This packet targets those exact top one-off blockers.

## Desired outcome

- The listed content/filter effects lower and render through strict native compositor mode for their current debug fixtures.
- `content.slideShift` remains a source-only native content stage.
- Filter effects either lower to exact compositor filter representations or to backend-owned source style stages when that is the smallest honest fit for player-compatible styled-cell output.
- Unsupported shapes remain explicit fallbacks with actionable diagnostics.
- Documentation and review/de-slop evidence are updated only for impacted files.

## Target recipe set

- `content/content_slide_shift.json`
- `filters/filter_bracket_emphasis.json`
- `filters/filter_dot_indicator.json`
- `filters/filter_edge_grow_left.json`
- `filters/filter_hover_bar.json`
- `filters/filter_matrix_rain_speed_profile.json`
- `filters/filter_sub_pixel_bar.json`
- `filters/filter_underline_wipe.json`

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

Primary native lowering/render files:

- `crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs`
- `crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs`
- `crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs`

Semantic references:

- `crates/tui-vfx-player/src/fnc_apply_content_primitive.rs`
- `crates/tui-vfx-player/src/fnc_apply_filter_primitive.rs`
- `crates/tui-vfx-player/src/fnc_collect_handled_primitive_inputs.rs`

Verification references:

- `scripts/k222_native_coverage_audit.sh`
- `scripts/k221_source_isolated_native_demo.sh`

## Red / green / refactor plan

1. Add strict-native CLI regressions for the eight target recipes and unsupported-shape guard cases.
2. Include native-vs-`irResolved` `rows`/`styledCells` parity for the target set.
3. Run targeted nextest to observe RED.
4. Implement the minimum source-stage/style-stage native representation needed for current fixture fields.
5. Run targeted nextest until GREEN.
6. Run native coverage audit and verify movement from 109 native passes.
7. Run format/check/clippy, docs/API/rustdoc gates if impacted, formal review, AI de-slop, and post-de-slop regression verification.

## Acceptance criteria

- Targeted strict-native CLI tests pass.
- Native-vs-`irResolved` parity holds for target `rows` and `styledCells`.
- Native coverage audit shows the eight target recipes no longer fall back.
- No hard errors are introduced.
- Native metadata remains honest: `fallbackUsed=false`, `sourceRenderMode=sourceOnly`, `nativeSourceIsolated=true` for newly supported recipes.
- Diagnostics remain actionable for unsupported graph outputs, unsupported fields, and unsupported scopes.
- Results documentation records baseline, final counters, exact blockers closed, and remaining blockers with a concrete next action.
