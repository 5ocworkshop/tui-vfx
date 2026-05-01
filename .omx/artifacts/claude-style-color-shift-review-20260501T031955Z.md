# Claude review: style.colorShift compositor lowering

Started: 2026-05-01T03:19:55Z

## Claude response

## Review

**No hard blockers.** The seam is correct; V2 parity holds.

**Verified**
- `crates/tui-vfx-style/src/utils/fnc_style_hsl.rs:16` `shift_style_hsl` reproduces the deleted backend math (rem_euclid hue, clamped sat/light, transparent passthrough). `cls_color_shift_shader.rs:42` multiplies by `ctx.t.clamp(0,1)` — same semantics as the removed `apply_color_shift_style_stage`.
- V2 letter-cell oracle (`test_fnc_cli_native_style_color_shift_matches_v2_deprecated_letter_cell_oracle_json`) still asserts identical `rgba(0,57,230,255) / rgba(26,18,106,255)` cell counts → byte-exact parity preserved despite f64→f32 cast at lowering.
- V3 round-trip extended (`ColorShift` legacy↔composed) at `test_try_lower_v3_spatial_shader_family.rs:264-269`. `VfxColorShiftShader` is a type alias mirroring `VfxColorFadeShader` — naming consistent with sibling families.
- `NativeStyleStage` has zero remaining variants and zero remaining call sites outside the lowering file. Workspace builds clean (no dead-code warnings — the empty enum + Vec are still referenced through summary plumbing).

**Risks / cleanup smell (not blockers)**
1. `NativeStyleStage` is now uninhabited (`enum {}`) but `style_stages: Vec<NativeStyleStage>`, the `&mut style_stages` plumbing, and the `styleStages` evidence counter remain. It's permanently dead infrastructure — the counter always reports 0 and `native_stage_non_empty`'s style branch is unreachable. Per your "no landmines / fix root causes" intention, this should be removed in this packet rather than left as the next sibling's chore. `fnc_lower_recipe_graph_to_composition_spec.rs:97,177,237,252,277,316,352,368,375,403,1947`.
2. `StyleRegion::All` + per-cell phase-multiplied shifts: confirm the compositor evaluates shader layers for every grid cell (not just cells already in `styled_cells`) — the legacy code synthesized cells from `rows` glyphs when no styled cell existed. Test asserts 105 cells which matches, so this looks correct, but it's the one behavior worth a sentence in the WCTX since prior color-style migrations (colorFade, rainbow) shared the same risk.

Suggest folding the `NativeStyleStage` removal into this commit.

Claude exit: 0 at 2026-05-01T03:23:26Z

## Follow-up actions after review

- Removed the empty `NativeStyleStage` enum, `style_stages` field, lowering plumbing, and renderer metadata dependency. `styleStages` remains only as compatibility evidence and is hard-coded to `0` in summaries.
- Kept `nativeStyleStages` backend metadata as `0` for compatibility with existing report consumers while removing the execution layer.
- Added/updated tests so colorShift and the affected sub-pixel-bar oracle assert compositor-owned primitives (`shaderLayers`/`filters`) with `styleStages = 0`.
- Verified `StyleRegion::All` shader execution covers the full 35x3 card surface via `styledCells == 105` in colorShift/colorFade tests.
