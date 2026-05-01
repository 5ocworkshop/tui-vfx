# Claude review: style.colorFade compositor lowering

Started: 2026-05-01T03:09:14Z

## Claude response

## Findings — hard blockers / correctness risks

**1. V2 parity break for `colorSpace: "hct"` (BLOCKER if oracle parity required).**
- Legacy `legacy_color_fade_label` was a 2-way switch — `eq_ignore_ascii_case("hsl")` → HSL lerp, *everything else* (including `"hct"`) → RGB lerp.
- New `color_space_from_str` (`fnc_lower_recipe_graph_to_composition_spec.rs:3704-3710`) maps `"hct"` → `ColorSpace::Hct` → perceptually-uniform CAM16 blend in `blend_style_to_color_in_space`.
- The shipped recipe `style_color_fade.json` declares `allowedValues: ["rgb","hct","hsl"]` and tags `v2-deprecated-oracle`. The recipe's exit phase uses `"hsl"`; no recipe currently exercises `"hct"`, but any consumer that does will silently change output.
- Also legacy was case-insensitive (`"HSL"` worked); new path depends on `enum_input` returning lowercase. Worth verifying.

**2. HSL parity is unverified (BLOCKER for "deterministic V2 parity").**
- Legacy `legacy_hsl_rgba` open-coded shortest-path hue lerp + per-channel alpha lerp.
- New path delegates to `blend_style_to_color_in_space`. Whether that function (a) chooses the same shortest-path direction, (b) lerps alpha, and (c) handles achromatic endpoints identically is not asserted by any test in this diff. The new CLI test only covers `enter` at `t=0.5` with `rgb`; the recipe's `exit` phase uses HSL and is untested.
- Add an HSL CLI assertion (e.g. exit phase, t=0.5) before claiming oracle parity.

**3. `StyleRegion::All` ≠ legacy "every cell in the report" semantics.**
- Legacy `apply_color_fade_style_stage` iterated `0..width × 0..height` and wrote a result for **every** grid cell, falling back to `DEFAULT_FOREGROUND` / `TRANSPARENT_RGBA` for cells with no prior styled entry.
- New path emits one shader layer with `StyleRegion::All`. If the compositor only invokes the shader on cells that already exist in the styled-cell stream, empty cells that previously got a "fade DEFAULT_FOREGROUND→target on transparent" result will now be absent. The shipped test recipe is fully populated by `source.card`, so the CLI test won't catch this; debug recipes that previously over-wrote into empty grid will diverge. Worth either (a) confirming the compositor's `StyleRegion::All` implementation fills empties, or (b) accepting and documenting the behavior change explicitly.

**4. Missing `enum_vfx_spatial_composed_primitive.rs` version bump.**
- File `crates/tui-vfx-style/src/models/v3/enum_vfx_spatial_composed_primitive.rs` adds a new public enum variant `ColorFade(VfxColorFadeShader)` and a new `family_label` arm, but VERS stays at `0.2.0` and CLOG is unchanged. Per project convention this is a new public surface and warrants 0.3.0 + new CLOG line. Same audit for `cls_spatial_shader_type.rs`, `fnc_lower_legacy_spatial_shader.rs`, `fnc_try_lower_v3_spatial_shader_family.rs`, and `mod.rs` re-exports — confirm each got its own VERS bump.

**5. V3 hierarchy naming is consistent with precedent.**
- `VfxColorFadeShader = ColorFadeShader` alias mirrors the `VfxRainbowCycleShader` pattern; placement under `VfxSpatialComposedPrimitive::ColorFade` with `family_label = "color_fade"` matches the snake_case tag convention. No naming objection.

**6. Round-trip test uses `ColorFadeShader::default()`.**
- `test_try_lower_v3_spatial_shader_family.rs` uses `Default` for the variant assertion. Default is `target = rgb(255,200,50), color_space = Rgb`. The round-trip is structurally fine, but it doesn't exercise distinct field values, so a mis-wired clone in `try_lower_composed` (e.g. dropping `color_space`) wouldn't be caught. Consider a non-default fixture (e.g. `Hsl` + alpha-bearing target) to harden the assertion — same gap exists for some sibling families, so this is a "while you're here" rather than a blocker.

**Recommended gates before merging:**
- Add HSL CLI assertion against the recipe's exit phase.
- Decide and document HCT semantics (intentional V3 upgrade vs. parity preservation), and either map `"hct"` → `ColorSpace::Rgb` in this lowering or update the v2-deprecated-oracle tag.
- Verify `StyleRegion::All` shader-layer paint covers empty cells, or annotate the behavior delta.
- Bump VERS / append CLOG on every file that gained a public surface in this slice.

Claude exit: 0 at 2026-05-01T03:11:21Z

## Follow-up actions after review

- Preserved V2 oracle behavior for declared `hct` colorSpace by mapping only literal `hsl` to HSL and every other value to RGB in style.colorFade lowering.
- Added exit-phase HSL CLI coverage with exact foreground/background assertions.
- Added full-grid styled cell count assertion (`35x3 = 105`) to verify `StyleRegion::All` shader execution covers the whole card surface for this fixture.
- Bumped public-surface CLOG/VERS for `VfxSpatialComposedPrimitive` and `SpatialShaderType` additions.
- Kept the V3 alias and non-new-dependency approach; no fallback branch added.
