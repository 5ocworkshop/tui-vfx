# Claude style.italicWindow review

## Original user task
Move v3.1 recipe effects through compositor-owned IR, remove wrong-layer player/backend effect logic, no fallbacks, commit at feature seams.

## Final prompt sent to Claude CLI
```text
Review this Rust diff for tui-vfx. Task: migrate v3.1 style.italicWindow out of backend/player style-stage emulation and into compositor-owned IR by lowering to a ModifierWindow ShaderLayerSpec. Requirements: preserve start/end settings, preserve V2-visible italic modifier behavior inside the window and no italic outside, no fallback, remove wrong-layer backend style code, and keep V3 spatial family conversions exhaustive without dropping fields. Focus on correctness, layering violations, silent semantic gaps, missing tests, and compile/clippy hazards. Return blockers first, then non-blocking suggestions.

Diff follows:
diff --git a/crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs b/crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs
index cc0078e..b589116 100644
--- a/crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs
+++ b/crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs
@@ -1,7 +1,8 @@
 // <FILE>crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs</FILE> - <DESC>Lower player render requests into compositor CompositionSpec modes</DESC>
 // <VERS>VERSION: 0.41.0</VERS>
 // <WCTX>Native compositor lowering: map bounded v3.1 recipe graph effects into native CompositionSpec and source-stage content/style/filter work with honest fallback diagnostics.</WCTX>
-// <CLOG>0.44.0: MINOR — lower style.pulse into compositor PulseWaveShader layers and remove backend style-stage emulation.
+// <CLOG>0.45.0: MINOR — lower style.italicWindow into compositor ModifierWindowShader layers and remove backend style-stage emulation.
+// 0.44.0: MINOR — lower style.pulse into compositor PulseWaveShader layers and remove backend style-stage emulation.
 // 0.43.0: MINOR — lower style.glitch into compositor GlitchLinesShader layers and remove backend style-stage emulation.
 // 0.42.0: MINOR — lower style.neonFlicker into compositor NeonFlickerShader layers and remove backend style-stage emulation.
 // 0.41.0: MINOR — lower focused-row-gradient style.spatial into compositor ShaderLayerSpec and remove backend style-stage emulation.
@@ -58,9 +59,9 @@ use tui_vfx_style::models::{
     FocusFieldShader, FocusFieldShape, FocusedRowGradientShader, GlistenApplyTo, GlistenBandShader,
     GlistenDirection, GlitchLinesShader, Gradient, HighlighterApplyTo, HighlighterDirection,
     HighlighterMode, HighlighterRowMask, HighlighterShader, LinearGradientApplyTo,
-    LinearGradientShader, NeonFlickerShader, PulseWaveShader, RadarShader, RevealWipeShader,
-    SegmentMode, SpatialShaderType, StyleRegion, TextContrast, WaveDirection, WayfindingNode,
-    WayfindingNodeApplyTo, WayfindingNodeShader,
+    LinearGradientShader, ModifierWindowShader, NeonFlickerShader, PulseWaveShader, RadarShader,
+    RevealWipeShader, SegmentMode, SpatialShaderType, StyleRegion, TextContrast, WaveDirection,
+    WayfindingNode, WayfindingNodeApplyTo, WayfindingNodeShader,
 };
 
 const SUPPORTED_WIPE_DIRECTIONS: &[&str] = &[
@@ -186,8 +187,6 @@ pub enum NativeStyleStage {
         saturation_shift: f64,
         lightness_shift: f64,
     },
-    /// Apply player-compatible italic-window styling.
-    ItalicWindow { start: f64, end: f64 },
 }
 
 /// Cursor wake behavior for native typewriter content.
@@ -547,7 +546,7 @@ fn lower_node_into_spec(
         "style.colorShift" => lower_style_color_shift(node, style_stages, request, warnings),
         "style.fadeIn" | "style.fadeOut" => lower_style_fade(node, spec, request, warnings),
         "style.pulse" => lower_style_pulse(node, spec, request, warnings),
-        "style.italicWindow" => lower_style_italic_window(node, style_stages, request, warnings),
+        "style.italicWindow" => lower_style_italic_window(node, spec, request, warnings),
         "style.moduloColumns" => lower_style_modulo_columns(node, style_stages, request, warnings),
         "style.neonFlicker" => lower_style_neon_flicker(node, spec, request, warnings),
         "style.rainbow" => lower_style_rainbow(node, style_stages, request, warnings),
@@ -2194,7 +2193,7 @@ fn lower_style_pulse(
 
 fn lower_style_italic_window(
     node: &NodeSpec,
-    style_stages: &mut Vec<NativeStyleStage>,
+    spec: &mut CompositionSpec,
     request: &PlayerRenderBackendRequest,
     warnings: Vec<PlayerRenderBackendDiagnostic>,
 ) -> NodeLoweringOutcome {
@@ -2207,9 +2206,16 @@ fn lower_style_italic_window(
         return NodeLoweringOutcome::Unsupported { reason };
     }
 
-    let start = number_input(node, request, "start", 0.0).clamp(0.0, 1.0);
-    let end = number_input(node, request, "end", 1.0).clamp(start, 1.0);
-    style_stages.push(NativeStyleStage::ItalicWindow { start, end });
+    let start = resolved_number_input(node, request, "start", 0.0).clamp(0.0, 1.0);
+    let end = resolved_number_input(node, request, "end", 1.0).clamp(start, 1.0);
+    spec.shader_layers.push(ShaderLayerSpec {
+        shader: SpatialShaderType::ModifierWindow(ModifierWindowShader {
+            start: start as f32,
+            end: end as f32,
+            italic: true,
+        }),
+        region: StyleRegion::All,
+    });
     NodeLoweringOutcome::Lowered { warnings }
 }
 
diff --git a/crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs b/crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs
index 895ed99..2441acf 100644
--- a/crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs
+++ b/crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs
@@ -290,9 +290,6 @@ fn scene_ir_with_native_content_stages(
                 *saturation_shift,
                 *lightness_shift,
             ),
-            NativeStyleStage::ItalicWindow { start, end } => {
-                apply_italic_window_style_stage(&mut staged, *start, *end)
-            }
         }
     }
     staged
@@ -817,19 +814,6 @@ fn apply_color_shift_style_stage(
     }
 }
 
-fn apply_italic_window_style_stage(report: &mut PlayerRenderIrReport, start: f64, end: f64) {
-    if !(start..=end).contains(&report.phase_t) {
-        return;
-    }
-    let width = report_width(report);
-    let height = report_height(report);
-    for y in 0..height {
-        for x in 0..width {
-            set_report_cell_style(report, x, y, None, None, Some("italic"));
-        }
-    }
-}
-
 fn set_report_cell_style(
     report: &mut PlayerRenderIrReport,
     x: usize,
diff --git a/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs b/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
index fd477bb..35ff883 100644
--- a/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
+++ b/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
@@ -2542,7 +2542,8 @@ fn test_fnc_cli_native_style_italic_window_matches_v2_deprecated_modifier_oracle
 
     assert_eq!(report["compositionMode"], "native");
     assert_eq!(report["fallbackUsed"], false);
-    assert_eq!(report["compositionSpecSummary"]["styleStages"], 1);
+    assert_eq!(report["compositionSpecSummary"]["shaderLayers"], 1);
+    assert_eq!(report["compositionSpecSummary"]["styleStages"], 0);
     assert_eq!(report["rows"][0], "╭─────────────────────────────────╮");
     assert_eq!(report["rows"][1], "│STYLE TEST: ItalicWindow         │");
     assert_eq!(
@@ -2554,6 +2555,40 @@ fn test_fnc_cli_native_style_italic_window_matches_v2_deprecated_modifier_oracle
         ),
         V2_ITALIC_WINDOW_CARD_CELL_COUNT
     );
+
+    let inactive_report = player_cli_json(
+        vec![
+            str_arg("render-backend"),
+            str_arg("--recipe"),
+            recipe_path("styles/style_italic_window.json"),
+            str_arg("--descriptor-pack"),
+            descriptor_pack_path(),
+            str_arg("--backend"),
+            str_arg("compositor"),
+            str_arg("--composition-mode"),
+            str_arg("native"),
+            str_arg("--fail-on-fallback"),
+            str_arg("--format"),
+            str_arg("json"),
+            str_arg("--phase"),
+            str_arg("enter"),
+            str_arg("--phase-t"),
+            str_arg("0.9"),
+        ],
+        "render-backend native italic-window outside window player cli",
+    );
+    assert_eq!(inactive_report["fallbackUsed"], false);
+    assert_eq!(inactive_report["compositionSpecSummary"]["shaderLayers"], 1);
+    assert_eq!(inactive_report["compositionSpecSummary"]["styleStages"], 0);
+    assert_eq!(
+        styled_cell_modifier_count(
+            &inactive_report,
+            "rgba(255,255,0,255)",
+            "rgba(50,40,20,255)",
+            "italic"
+        ),
+        0
+    );
 }
 
 #[test]
diff --git a/crates/tui-vfx-style/src/models/cls_spatial_shader_type.rs b/crates/tui-vfx-style/src/models/cls_spatial_shader_type.rs
index c71cfcc..191f989 100644
--- a/crates/tui-vfx-style/src/models/cls_spatial_shader_type.rs
+++ b/crates/tui-vfx-style/src/models/cls_spatial_shader_type.rs
@@ -114,10 +114,10 @@ use crate::models::{
     cls_focused_row_gradient_shader::FocusedRowGradientShader,
     cls_glisten_band_shader::GlistenBandShader, cls_glitch_lines_shader::GlitchLinesShader,
     cls_glow_shader::GlowShader, cls_highlighter_shader::HighlighterShader,
-    cls_neon_flicker_shader::NeonFlickerShader, cls_orbit_shader::OrbitShader,
-    cls_pulse_wave_shader::PulseWaveShader, cls_radar_shader::RadarShader,
-    cls_radial_spiral_shader::RadialSpiralShader, cls_reflect_shader::ReflectShader,
-    cls_reveal_wipe_shader::RevealWipeShader,
+    cls_modifier_window_shader::ModifierWindowShader, cls_neon_flicker_shader::NeonFlickerShader,
+    cls_orbit_shader::OrbitShader, cls_pulse_wave_shader::PulseWaveShader,
+    cls_radar_shader::RadarShader, cls_radial_spiral_shader::RadialSpiralShader,
+    cls_reflect_shader::ReflectShader, cls_reveal_wipe_shader::RevealWipeShader,
     cls_stochastic_sparkle_shader::StochasticSparkleShader,
     cls_sub_cell_shake_shader::SubCellShakeShader, cls_terminal_fire_shader::TerminalFireShader,
     cls_terminal_water_shader::TerminalWaterShader, cls_trace_path_shader::TracePathShader,
@@ -188,6 +188,9 @@ pub enum SpatialShaderType {
     /// Rippling color wave emanating from position (attention).
     PulseWave(PulseWaveShader),
 
+    /// Time-windowed text modifier application.
+    ModifierWindow(ModifierWindowShader),
+
     /// Layered water/ocean field with ripples, wakes, foam, and glint.
     TerminalWater(TerminalWaterShader),
 
@@ -265,6 +268,7 @@ impl StyleShader for SpatialShaderType {
             SpatialShaderType::GlitchLines(s) => s.style_at(ctx, base),
             SpatialShaderType::NeonFlicker(s) => s.style_at(ctx, base),
             SpatialShaderType::PulseWave(s) => s.style_at(ctx, base),
+            SpatialShaderType::ModifierWindow(s) => s.style_at(ctx, base),
             SpatialShaderType::TerminalWater(s) => s.style_at(ctx, base),
             SpatialShaderType::TerminalFire(s) => s.style_at(ctx, base),
             SpatialShaderType::RadialSpiral(s) => s.style_at(ctx, base),
@@ -413,6 +417,7 @@ impl SpatialShaderType {
             SpatialShaderType::GlitchLines(_) => "GlitchLines",
             SpatialShaderType::NeonFlicker(_) => "NeonFlicker",
             SpatialShaderType::PulseWave(_) => "PulseWave",
+            SpatialShaderType::ModifierWindow(_) => "ModifierWindow",
             SpatialShaderType::TerminalWater(_) => "TerminalWater",
             SpatialShaderType::TerminalFire(_) => "TerminalFire",
             SpatialShaderType::RadialSpiral(_) => "RadialSpiral",
@@ -468,6 +473,7 @@ impl SpatialShaderType {
                 "Flickering neon sign effect with independent segments"
             }
             SpatialShaderType::PulseWave(_) => "Rippling color wave emanating from position",
+            SpatialShaderType::ModifierWindow(_) => "Text modifier during a time window",
             SpatialShaderType::TerminalFire(_) => {
                 "Emissive procedural flame/smoke field with rising turbulence, blue core, and sparks"
             }
@@ -617,6 +623,11 @@ impl SpatialShaderType {
                 ("wavelength", format!("{} cells", s.wavelength)),
                 ("color", format!("{:?}", s.color)),
             ],
+            SpatialShaderType::ModifierWindow(s) => vec![
+                ("start", format!("{}", s.start)),
+                ("end", format!("{}", s.end)),
+                ("italic", format!("{}", s.italic)),
+            ],
             SpatialShaderType::TerminalWater(s) => vec![
                 ("mode", format!("{:?}", s.mode)),
                 ("layers", format!("{}", s.layers)),
diff --git a/crates/tui-vfx-style/src/models/mod.rs b/crates/tui-vfx-style/src/models/mod.rs
index d93ec3c..51fc206 100644
--- a/crates/tui-vfx-style/src/models/mod.rs
+++ b/crates/tui-vfx-style/src/models/mod.rs
@@ -52,6 +52,7 @@ pub mod cls_gradient;
 pub mod cls_gradient_lut;
 pub mod cls_highlighter_shader;
 pub mod cls_linear_gradient_shader;
+pub mod cls_modifier_window_shader;
 pub mod cls_neon_flicker_shader;
 pub mod cls_noise_type;
 pub mod cls_orbit_shader;
@@ -124,6 +125,7 @@ pub use cls_highlighter_shader::{
     HighlighterShader, TextContrast,
 };
 pub use cls_linear_gradient_shader::{LinearGradientApplyTo, LinearGradientShader};
+pub use cls_modifier_window_shader::ModifierWindowShader;
 pub use cls_neon_flicker_shader::{NeonFlickerShader, SegmentMode};
 pub use cls_noise_type::NoiseType;
 pub use cls_orbit_shader::OrbitShader;
diff --git a/crates/tui-vfx-style/src/models/v3/enum_vfx_spatial_composed_primitive.rs b/crates/tui-vfx-style/src/models/v3/enum_vfx_spatial_composed_primitive.rs
index 270f22b..d434b78 100644
--- a/crates/tui-vfx-style/src/models/v3/enum_vfx_spatial_composed_primitive.rs
+++ b/crates/tui-vfx-style/src/models/v3/enum_vfx_spatial_composed_primitive.rs
@@ -11,9 +11,12 @@
 //! authored surfaces that the plan decided should not be treated as true
 //! primitives.
 
-use crate::models::v3::{
-    VfxCursorShader, VfxGuidanceCueShader, VfxMaterialLightShader, VfxProgressEmphasisShader,
-    VfxStochasticTextureShader, VfxStripeMotionShader, VfxTravelingBandShader,
+use crate::models::{
+    ModifierWindowShader,
+    v3::{
+        VfxCursorShader, VfxGuidanceCueShader, VfxMaterialLightShader, VfxProgressEmphasisShader,
+        VfxStochasticTextureShader, VfxStripeMotionShader, VfxTravelingBandShader,
+    },
 };
 use serde::{Deserialize, Serialize};
 
@@ -35,6 +38,8 @@ pub enum VfxSpatialComposedPrimitive {
     Cursor(VfxCursorShader),
     /// Stripe-motion composed family.
     StripeMotion(VfxStripeMotionShader),
+    /// Time-windowed text modifier composed family.
+    ModifierWindow(ModifierWindowShader),
 }
 
 impl VfxSpatialComposedPrimitive {
@@ -48,6 +53,7 @@ impl VfxSpatialComposedPrimitive {
             Self::StochasticTexture(_) => "stochastic_texture",
             Self::Cursor(_) => "cursor",
             Self::StripeMotion(_) => "stripe_motion",
+            Self::ModifierWindow(_) => "modifier_window",
         }
     }
 }
diff --git a/crates/tui-vfx-style/src/models/v3/fnc_lower_legacy_spatial_shader.rs b/crates/tui-vfx-style/src/models/v3/fnc_lower_legacy_spatial_shader.rs
index 7f23eff..be778a4 100644
--- a/crates/tui-vfx-style/src/models/v3/fnc_lower_legacy_spatial_shader.rs
+++ b/crates/tui-vfx-style/src/models/v3/fnc_lower_legacy_spatial_shader.rs
@@ -55,6 +55,9 @@ pub fn lower_legacy_spatial_shader(shader: &SpatialShaderType) -> VfxSpatialShad
         SpatialShaderType::PulseWave(shader) => {
             VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::MotionField(shader.into()))
         }
+        SpatialShaderType::ModifierWindow(shader) => VfxSpatialShaderFamily::ComposedPrimitive(
+            VfxSpatialComposedPrimitive::ModifierWindow(shader.clone()),
+        ),
         SpatialShaderType::RadialSpiral(shader) => {
             VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::MotionField(shader.into()))
         }
diff --git a/crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs b/crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs
index d0ce908..58e8e55 100644
--- a/crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs
+++ b/crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs
@@ -85,6 +85,9 @@ fn try_lower_composed(
         }
         VfxSpatialComposedPrimitive::Cursor(shader) => Ok(SpatialShaderType::from(shader)),
         VfxSpatialComposedPrimitive::StripeMotion(shader) => Ok(SpatialShaderType::from(shader)),
+        VfxSpatialComposedPrimitive::ModifierWindow(shader) => {
+            Ok(SpatialShaderType::ModifierWindow(shader.clone()))
+        }
     }
 }
 
```

## Claude output (raw)
```text
# Review: style.italicWindow → ModifierWindow ShaderLayerSpec migration

## Verified facts
- `cls_modifier_window_shader.rs` exists (untracked in diff), implements `StyleShader` with the correct semantic: `(self.start..=self.end).contains(&(ctx.t as f32))` short-circuits outside the window — identical inclusive bounds to the old `apply_italic_window_style_stage`.
- `spec.t` is set from `input.phase_t` at lowering (line 221), so `ctx.t` reaching the shader matches the `phase_t` the old code gated on.
- Recipe `style_italic_window.json` uses start=0.2, end=0.8. New test at phase_t=0.9 is therefore strictly outside; existing test at 0.5 is inside.
- Both V3 round-trip arms are wired: `lower_legacy_spatial_shader` produces `ComposedPrimitive::ModifierWindow`; `try_lower_composed` rebuilds `SpatialShaderType::ModifierWindow`. The compositor's `try_from_v3_shader_family` `.expect(...)` would otherwise panic on a missing arm — exhaustiveness is preserved.

## Blockers
None with high confidence. The migration preserves behavior, removes the wrong-layer renderer code, and round-trips cleanly. Two items at the edge of "blocker" depending on house policy:

1. **Wire-format prefix regression (Intention 8).** Every other sibling in `VfxSpatialComposedPrimitive` wraps with a `Vfx*`-prefixed type (`VfxCursorShader`, `VfxStripeMotionShader`, etc.). The new variant embeds the legacy `ModifierWindowShader` directly: `ModifierWindow(ModifierWindowShader)`. Since this struct is a brand-new public type (0.1.0) crossing the V3 wire boundary, this is the right moment to introduce it as `VfxModifierWindowShader` (or alias) rather than retroactively. If the V3 cutover plan treats this enum as the wire surface, the unprefixed name will need to be renamed later — easier now, before consumers latch on.

2. **Silent semantic widening: `number_input` → `resolved_number_input`.** This change isn't called out in the CLOG. Behaviorally, `start`/`end` now resolve through `runtime_input_overrides`, signals, and `graph_values` — they did not before. That's a real recipe-behavior change (positive: bindings to start/end now work). It deserves an explicit CLOG line so it's not buried under "lower into ShaderLayerSpec." Confirm whether v3.1 already documented start/end as binding-resolvable — if so, this was a pre-existing bug fix and should be called out as such.

## Non-blocking suggestions

3. **Header version drift in `fnc_render_compositor_backend.rs`.** Still at 0.38.0; CLOG ends at focused-row-gradient. Pulse, glitch, neonFlicker, and now italicWindow have all shed code from this file without a VERS bump. Per the user's metadata-header memory, VERS is per-file edit count — this file is 4 edits behind. Same gap exists in 5deb84b and earlier, so this is a pattern issue, not specific to this commit; flag once and keep going.

4. **Round-trip unit test missing.** The new V3 ComposedPrimitive arm is only exercised by the integration CLI test. A focused test in `cls_spatial_shader_type` (or alongside `lower_legacy_spatial_shader` / `try_lower_composed`) round-tripping a `ModifierWindow` would catch a future regression in either arm directly, without needing the player CLI harness.

5. **Boundary coverage in the CLI test.** New test covers 0.9 (outside). Existing test covers 0.5 (inside). Neither pins inclusive boundary at exactly start=0.2 or end=0.8. Given the inclusive `..=` is preserved, a single boundary assertion would lock in the contract cheaply.

6. **`italic: false` is a degenerate config.** With only `italic` as the shader's effect, `italic=false` makes the shader a no-op — the `is_empty()` branch returns `base` for every cell. That's fine as a defensive default, but if the schema is meant to keep meaning, consider rejecting `italic=false` at validation, or add a `// Why:` comment that this is intentionally tolerated as the future-extension shape (when more modifiers are added).

7. **Rustdoc on the new public type is light.** Per the user's "improve rustdocs whenever editing a file" memory: `ModifierWindowShader` is a fresh public surface and is the V3 wire payload for italicWindow. The current doc comments are one-liners. A short paragraph on the field semantics (inclusive bounds, normalized phase_t, modifier OR-combine vs. overwrite) would carry weight in autogen schema docs.

## Compile/clippy
No hazards visible. Both match arms (`StyleShader` impl, `name`/`description`/`decorate`) are added; `lower_legacy_spatial_shader` and `try_lower_composed` are exhaustive after the additions. The `as f32` casts on already-clamped `[0.0, 1.0]` f64 values are precision-safe.
```

## Follow-up actions after Claude review

Claude found no hard correctness blockers. I addressed the policy/quality edge items before commit:

- Added a V3-prefixed `VfxModifierWindowShader` surface (type alias over the runtime `ModifierWindowShader`) and changed `VfxSpatialComposedPrimitive::ModifierWindow` to use the Vfx-prefixed name.
- Documented `ModifierWindowShader` semantics: inclusive normalized bounds and OR-combined modifiers.
- Added exact start/end boundary coverage to the shader unit test.
- Added a focused V3 family round-trip regression for `SpatialShaderType::ModifierWindow`.
- Kept the CLI coverage proving inside-window italic parity and outside-window no-italic behavior, both with shaderLayers=1/styleStages=0.
- Noted that `start`/`end` now use resolved inputs as part of the compositor-native lowering; this preserves literal recipes and allows binding-compatible lowering instead of backend-stage emulation.

Post-fix verification: cargo check passed, targeted tui-vfx-style and tui-vfx-player-cli nextest runs passed, strict render-backend showed fallbackUsed=false/nativeLoweringSucceeded=true/shaderLayers=1/styleStages=0/italicCount=315, clippy --tests -D warnings passed, fmt --check passed, and git diff --check passed.
