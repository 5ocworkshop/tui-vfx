<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_29.md</FILE> - <DESC>Self-generated v3.1 remaining vignette/mask native blocker closure packet</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Ralph continuation after one-off content/filter blocker closure: attack current top vignette and mask strict-native fallbacks.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — define the remaining vignette/mask native blocker closure tranche.</CLOG> -->

# Remaining vignette/mask native blocker closure packet

## Task statement

Continue the v3.1 player/studio completion loop by converting the current top non-deprecated `debug_recipes` compositor-native fallbacks into strict native passes where authored fields can be represented without semantic loss.

Current audit after one-off content/filter closure:

```text
recipes=144 nativePasses=117 fallbacks=27 hardErrors=0
topUnsupported=filter.vignette:1, mask.blinds:1, mask.cellular:1, mask.diamond:1, mask.dissolve:1, mask.iris:1, mask.none:1, mask.pathReveal:1
```

This packet targets those exact top blockers.

## Desired outcome

- The listed vignette/mask effects lower and render through strict native compositor mode for their current debug fixtures.
- `filter.vignette` preserves authored player-visible color/applyTo semantics; use a backend-owned style stage if direct `FilterSpec::Vignette` remains non-isomorphic for the fixture fields.
- Mask effects lower to compositor `MaskSpec` variants only where current fields are represented without semantic loss.
- Unsupported shapes remain explicit fallbacks with actionable diagnostics.
- Documentation and review/de-slop evidence are updated only for impacted files.

## Target recipe set

- `filters/filter_vignette.json`
- `masks/mask_blinds.json`
- `masks/mask_cellular.json`
- `masks/mask_diamond.json`
- `masks/mask_dissolve.json`
- `masks/mask_iris.json`
- `masks/mask_none.json`
- `masks/mask_path_reveal.json`

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

- `crates/tui-vfx-player/src/fnc_apply_filter_primitive.rs`
- `crates/tui-vfx-player/src/fnc_collect_handled_primitive_inputs.rs`
- `crates/tui-vfx-compositor/src/types/cls_mask_spec.rs`
- `crates/tui-vfx-compositor/src/pipeline/cls_prepared_mask.rs`

Verification references:

- `scripts/k222_native_coverage_audit.sh`
- `scripts/k221_source_isolated_native_demo.sh`

## Red / green / refactor plan

1. Add strict-native CLI regressions for the eight target recipes and unsupported-shape guard cases.
2. Include native-vs-`irResolved` `rows`/`styledCells` parity for the vignette path when implemented as a source style stage.
3. Run targeted nextest to observe RED.
4. Implement the minimum honest native representation needed for current fixture fields.
5. Run targeted nextest until GREEN.
6. Run native coverage audit and verify movement from 117 native passes.
7. Run format/check/clippy, docs/API/rustdoc gates if impacted, formal review, AI de-slop, and post-de-slop regression verification.

## Acceptance criteria

- Targeted strict-native CLI tests pass.
- Native coverage audit shows the eight target recipes no longer fall back.
- No hard errors are introduced.
- Native metadata remains honest: `fallbackUsed=false`, `sourceRenderMode=sourceOnly`, `nativeSourceIsolated=true` for newly supported recipes.
- Diagnostics remain actionable for unsupported graph outputs, unsupported fields, and unsupported scopes.
- Results documentation records baseline, final counters, exact blockers closed, and remaining blockers with a concrete next action.
