<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_27.md</FILE> - <DESC>Self-generated v3.1 native residual blocker closure packet</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Ralph continuation after native shader/filter/mask/sampler blocker closure: attack current top style and content strict-native fallbacks.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — define the residual style/content native blocker closure tranche.</CLOG> -->

# Residual style and content native blocker closure packet

## Task statement

Continue the v3.1 player/studio completion loop by converting the current top non-deprecated `debug_recipes` compositor-native fallbacks into strict native passes where the authored fields can be represented without semantic loss.

Current audit after the previous tranche:

```text
recipes=144 nativePasses=99 fallbacks=45 hardErrors=0
topUnsupported=style.moduloColumns:2, style.neonFlicker:2, content.dissolve:1, content.glitchShift:1, content.mirror:1, content.numeric:1, content.redact:1, content.scrambleGlitchShift:1
```

This packet targets those exact top blockers.

## Desired outcome

- `style.moduloColumns`, `style.neonFlicker`, and the listed content transforms lower and render through strict native compositor mode for their current debug fixtures.
- Content effects remain source-only native content stages; do not regress into IR-resolved fallback.
- Style effects either lower to existing compositor filters/shader layers/region semantics or add a small dedicated style-stage representation only if that is the smallest honest fit.
- Unsupported shapes remain explicit fallbacks with actionable diagnostics.
- Documentation and review/de-slop evidence are updated only for impacted files.

## Target recipe set

- `styles/style_modulo_columns_period.json`
- `styles/style_modulo_vertical_every_fourth_column_offset.json`
- `styles/style_neon_flicker.json`
- `styles/style_neon_flicker_modifier.json`
- `content/content_dissolve.json`
- `content/content_glitch_shift.json`
- `content/content_mirror.json`
- `content/content_numeric.json`
- `content/content_redact.json`
- `content/content_scramble_glitch_shift.json`

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
- `crates/tui-vfx-player/src/fnc_apply_style_primitive.rs`
- `crates/tui-vfx-player/src/fnc_collect_handled_primitive_inputs.rs`

Verification references:

- `scripts/k222_native_coverage_audit.sh`
- `scripts/k221_source_isolated_native_demo.sh`

## Red / green / refactor plan

1. Add strict-native CLI regressions for the ten target recipes and unsupported-shape guard cases.
2. Run targeted nextest to observe RED.
3. Implement the minimum source-stage/style-stage native representation needed for the current fixture fields.
4. Run targeted nextest until GREEN.
5. Run native coverage audit and verify movement from 99 native passes.
6. Run format/check/clippy, docs/API gates if impacted, formal review, AI de-slop, and post-de-slop regression verification.

## Acceptance criteria

- Targeted strict-native CLI tests pass.
- Native coverage audit shows the ten target recipes no longer fall back.
- No hard errors are introduced.
- Native metadata remains honest: `fallbackUsed=false`, `sourceRenderMode=sourceOnly`, `nativeSourceIsolated=true` for newly supported recipes.
- Diagnostics remain actionable for unsupported graph outputs, non-all scopes, and unrepresentable fields.
- Results documentation records baseline, final counters, exact blockers closed, and remaining blockers with a concrete next action.
