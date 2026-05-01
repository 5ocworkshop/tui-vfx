# Claude style.pulse review

## Original user task
Move v3.1 recipe effects through compositor-owned IR, remove wrong-layer player/backend effect logic, no fallbacks, commit at feature seams.

## Final prompt sent to Claude CLI
```text
Review this Rust diff for tui-vfx. Task: migrate v3.1 style.pulse out of backend/player style-stage emulation and into compositor-owned IR by lowering to SpatialShaderType::PulseWave ShaderLayerSpec. Requirements: preserve recipe settings (pulseColor/color, frequency including binding id, applyTo), preserve V2-visible uniform channel-pulse output, no fallback, remove wrong-layer backend style code, and keep V3 motion-field conversions from dropping new fields. Focus on correctness, layering violations, silent semantic gaps, missing tests, and compile/clippy hazards. Return blockers first, then non-blocking suggestions.

Diff follows:
diff --git a/crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs b/crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs
index 51dcf40..8258725 100644
--- a/crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs
+++ b/crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs
@@ -1,7 +1,8 @@
 // <FILE>crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs</FILE> - <DESC>Lower player render requests into compositor CompositionSpec modes</DESC>
 // <VERS>VERSION: 0.41.0</VERS>
 // <WCTX>Native compositor lowering: map bounded v3.1 recipe graph effects into native CompositionSpec and source-stage content/style/filter work with honest fallback diagnostics.</WCTX>
-// <CLOG>0.43.0: MINOR — lower style.glitch into compositor GlitchLinesShader layers and remove backend style-stage emulation.
+// <CLOG>0.44.0: MINOR — lower style.pulse into compositor PulseWaveShader layers and remove backend style-stage emulation.
+// 0.43.0: MINOR — lower style.glitch into compositor GlitchLinesShader layers and remove backend style-stage emulation.
 // 0.42.0: MINOR — lower style.neonFlicker into compositor NeonFlickerShader layers and remove backend style-stage emulation.
 // 0.41.0: MINOR — lower focused-row-gradient style.spatial into compositor ShaderLayerSpec and remove backend style-stage emulation.
 // 0.40.0: MINOR — lower shader.diffusion into compositor ShaderLayerSpec and remove backend style-stage emulation.
@@ -57,9 +58,9 @@ use tui_vfx_style::models::{
     FocusFieldShader, FocusFieldShape, FocusedRowGradientShader, GlistenApplyTo, GlistenBandShader,
     GlistenDirection, GlitchLinesShader, Gradient, HighlighterApplyTo, HighlighterDirection,
     HighlighterMode, HighlighterRowMask, HighlighterShader, LinearGradientApplyTo,
-    LinearGradientShader, NeonFlickerShader, RadarShader, RevealWipeShader, SegmentMode,
-    SpatialShaderType, StyleRegion, TextContrast, WayfindingNode, WayfindingNodeApplyTo,
-    WayfindingNodeShader,
+    LinearGradientShader, NeonFlickerShader, PulseWaveShader, RadarShader, RevealWipeShader,
+    SegmentMode, SpatialShaderType, StyleRegion, TextContrast, WaveDirection, WayfindingNode,
+    WayfindingNodeApplyTo, WayfindingNodeShader,
 };
 
 const SUPPORTED_WIPE_DIRECTIONS: &[&str] = &[
@@ -185,12 +186,6 @@ pub enum NativeStyleStage {
         saturation_shift: f64,
         lightness_shift: f64,
     },
-    /// Apply player-compatible pulse styling to existing channels.
-    Pulse {
-        color: String,
-        frequency: f64,
-        apply_to: String,
-    },
     /// Apply player-compatible italic-window styling.
     ItalicWindow { start: f64, end: f64 },
 }
@@ -551,7 +546,7 @@ fn lower_node_into_spec(
         "style.colorFade" => lower_style_color_fade(node, style_stages, request, warnings),
         "style.colorShift" => lower_style_color_shift(node, style_stages, request, warnings),
         "style.fadeIn" | "style.fadeOut" => lower_style_fade(node, spec, request, warnings),
-        "style.pulse" => lower_style_pulse(node, style_stages, request, warnings),
+        "style.pulse" => lower_style_pulse(node, spec, request, warnings),
         "style.italicWindow" => lower_style_italic_window(node, style_stages, request, warnings),
         "style.moduloColumns" => lower_style_modulo_columns(node, style_stages, request, warnings),
         "style.neonFlicker" => lower_style_neon_flicker(node, spec, request, warnings),
@@ -2159,7 +2154,7 @@ fn lower_style_color_shift(
 
 fn lower_style_pulse(
     node: &NodeSpec,
-    style_stages: &mut Vec<NativeStyleStage>,
+    spec: &mut CompositionSpec,
     request: &PlayerRenderBackendRequest,
     warnings: Vec<PlayerRenderBackendDiagnostic>,
 ) -> NodeLoweringOutcome {
@@ -2172,15 +2167,24 @@ fn lower_style_pulse(
         return NodeLoweringOutcome::Unsupported { reason };
     }
 
-    style_stages.push(NativeStyleStage::Pulse {
-        color: color_label_from_config(color_alias_input(
-            node,
-            request,
-            &["pulseColor", "color"],
-            (255, 100, 100),
-        )),
-        frequency: number_input(node, request, "frequency", 1.0).max(0.0),
-        apply_to: enum_label_input(node, request, "applyTo", "foreground"),
+    spec.shader_layers.push(ShaderLayerSpec {
+        shader: SpatialShaderType::PulseWave(PulseWaveShader {
+            frequency: resolved_number_input(node, request, "frequency", 1.0).max(0.0) as f32,
+            frequency_binding: signal_source_id(node, "frequency")
+                .map(|id| id.as_str().to_string()),
+            speed: 1.0,
+            color: resolved_color_alias_input(
+                node,
+                request,
+                &["pulseColor", "color"],
+                (255, 100, 100),
+            ),
+            apply_to: pulse_apply_to_input(node, request),
+            uniform: true,
+            direction: WaveDirection::Horizontal,
+            wavelength: PulseWaveShader::default().wavelength,
+        }),
+        region: StyleRegion::All,
     });
     NodeLoweringOutcome::Lowered { warnings }
 }
@@ -3608,6 +3612,29 @@ fn color_alias_input(
         })
 }
 
+fn resolved_color_alias_input(
+    node: &NodeSpec,
+    request: &PlayerRenderBackendRequest,
+    keys: &[&str],
+    default_rgb: (u8, u8, u8),
+) -> ColorConfig {
+    keys.iter()
+        .find_map(|key| resolved_color_input(node, request, key))
+        .unwrap_or(ColorConfig::Rgb {
+            r: default_rgb.0,
+            g: default_rgb.1,
+            b: default_rgb.2,
+        })
+}
+
+fn pulse_apply_to_input(node: &NodeSpec, request: &PlayerRenderBackendRequest) -> ApplyToColor {
+    match enum_input(node, request, "applyTo") {
+        Some("foreground" | "fg") => ApplyToColor::Foreground,
+        Some("background" | "bg") => ApplyToColor::Background,
+        _ => ApplyToColor::Both,
+    }
+}
+
 fn color_config_from_hex(value: &str) -> Option<ColorConfig> {
     let hex = value.strip_prefix('#')?;
     if hex.len() != 6 {
diff --git a/crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs b/crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs
index 55e01a8..895ed99 100644
--- a/crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs
+++ b/crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs
@@ -290,11 +290,6 @@ fn scene_ir_with_native_content_stages(
                 *saturation_shift,
                 *lightness_shift,
             ),
-            NativeStyleStage::Pulse {
-                color,
-                frequency,
-                apply_to,
-            } => apply_pulse_style_stage(&mut staged, color, *frequency, apply_to),
             NativeStyleStage::ItalicWindow { start, end } => {
                 apply_italic_window_style_stage(&mut staged, *start, *end)
             }
@@ -822,39 +817,6 @@ fn apply_color_shift_style_stage(
     }
 }
 
-fn apply_pulse_style_stage(
-    report: &mut PlayerRenderIrReport,
-    color: &str,
-    frequency: f64,
-    apply_to: &str,
-) {
-    let width = report_width(report);
-    let height = report_height(report);
-    let clock = report.loop_t.unwrap_or(report.phase_t);
-    let strength = (clock * frequency.max(0.0) * std::f64::consts::TAU).sin() * 0.5 + 0.5;
-    for y in 0..height {
-        for x in 0..width {
-            let (existing_foreground, existing_background) = report
-                .styled_cells
-                .iter()
-                .find(|cell| cell.x == x && cell.y == y)
-                .map(|cell| (cell.foreground.clone(), cell.background.clone()))
-                .unwrap_or_else(|| (DEFAULT_FOREGROUND.to_string(), TRANSPARENT_RGBA.to_string()));
-            let foreground = if matches!(apply_to, "foreground" | "both") {
-                pulse_lerp_rgba_label(&existing_foreground, color, strength)
-            } else {
-                existing_foreground
-            };
-            let background = if matches!(apply_to, "background" | "both") {
-                pulse_lerp_rgba_label(&existing_background, color, strength)
-            } else {
-                existing_background
-            };
-            set_report_cell_style(report, x, y, Some(&foreground), Some(&background), None);
-        }
-    }
-}
-
 fn apply_italic_window_style_stage(report: &mut PlayerRenderIrReport, start: f64, end: f64) {
     if !(start..=end).contains(&report.phase_t) {
         return;
@@ -1074,27 +1036,6 @@ fn dissolve_threshold(x: usize, y: usize, width: usize, seed: usize, direction:
 
 const DEFAULT_FOREGROUND: &str = "defaultForeground";
 const TRANSPARENT_RGBA: &str = "transparent";
-fn pulse_lerp_rgba_label(from: &str, to: &str, t: f64) -> String {
-    let Some((from_r, from_g, from_b, from_a)) = parse_rgba_label(from) else {
-        return from.to_string();
-    };
-    let Some((to_r, to_g, to_b, to_a)) = parse_rgba_label(to) else {
-        return from.to_string();
-    };
-    let t = t.clamp(0.0, 1.0);
-    let inv_t = 1.0 - t;
-    rgba_label(
-        pulse_lerp_channel(from_r, to_r, inv_t, t),
-        pulse_lerp_channel(from_g, to_g, inv_t, t),
-        pulse_lerp_channel(from_b, to_b, inv_t, t),
-        pulse_lerp_channel(from_a, to_a, inv_t, t),
-    )
-}
-
-fn pulse_lerp_channel(start: u8, end: u8, inv_t: f64, t: f64) -> u8 {
-    (start as f64 * inv_t + end as f64 * t) as u8
-}
-
 fn rgba_label(r: u8, g: u8, b: u8, a: u8) -> String {
     format!("rgba({r},{g},{b},{a})")
 }
diff --git a/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs b/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
index 41b1429..151475d 100644
--- a/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
+++ b/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
@@ -2584,7 +2584,8 @@ fn test_fnc_cli_native_style_pulse_matches_v2_deprecated_channel_oracle_json() {
 
     assert_eq!(report["compositionMode"], "native");
     assert_eq!(report["fallbackUsed"], false);
-    assert_eq!(report["compositionSpecSummary"]["styleStages"], 1);
+    assert_eq!(report["compositionSpecSummary"]["shaderLayers"], 1);
+    assert_eq!(report["compositionSpecSummary"]["styleStages"], 0);
     assert_eq!(report["rows"][0], "╭─────────────────────────────────╮");
     assert_eq!(report["rows"][1], "│STYLE TEST: Pulse Effect         │");
     assert_eq!(
diff --git a/crates/tui-vfx-style/src/models/cls_pulse_wave_shader.rs b/crates/tui-vfx-style/src/models/cls_pulse_wave_shader.rs
index 372f67b..c60e329 100644
--- a/crates/tui-vfx-style/src/models/cls_pulse_wave_shader.rs
+++ b/crates/tui-vfx-style/src/models/cls_pulse_wave_shader.rs
@@ -1,10 +1,11 @@
 // <FILE>tui-vfx-style/src/models/cls_pulse_wave_shader.rs</FILE> - <DESC>Spatial pulse wave with rippling color</DESC>
-// <VERS>VERSION: 1.5.0</VERS>
-// <WCTX>Phase 0 P0.3 follow-on — refactor representative position/radius field math onto the shared mixed-signals spatial coordinate substrate now that those leaves exist.</WCTX>
-// <CLOG>1.5.0: use mixed-signals spatial coordinate leaves for horizontal/vertical/radial/diagonal wave phase derivation instead of open-coding normalized coordinate math in the shader.
+// <VERS>VERSION: 1.6.0</VERS>
+// <WCTX>Compositor-owned v3.1 style.pulse lowering needs PulseWaveShader to support uniform channel pulses without backend emulation.</WCTX>
+// <CLOG>1.6.0: add uniform pulse and channel-target controls so v3.1 pulse style recipes can lower into compositor shader layers.
+// 1.5.0: use mixed-signals spatial coordinate leaves for horizontal/vertical/radial/diagonal wave phase derivation instead of open-coding normalized coordinate math in the shader.
 // Add frequency_binding: Option<String> and thread an explicit frequency parameter through blend_at so style_at can resolve the binding once per frame</CLOG>
 
-use crate::models::{ColorConfig, ColorSpace};
+use crate::models::{ApplyToColor, ColorConfig, ColorSpace};
 use crate::traits::{ShaderContext, StyleShader};
 use crate::utils::blend_colors;
 use mixed_signals::prelude::{Signal, SignalContext, SignalExt, Sine, SpatialCoordinateSignal};
@@ -32,6 +33,13 @@ pub struct PulseWaveShader {
     pub speed: f32,
     /// Color to pulse towards
     pub color: ColorConfig,
+    /// Target channel(s) affected by the pulse.
+    #[serde(default)]
+    pub apply_to: ApplyToColor,
+    /// Whether all cells share the same temporal pulse factor.
+    #[serde(default)]
+    #[config(default = false)]
+    pub uniform: bool,
     /// Wave direction
     #[config(default = "horizontal")]
     pub direction: WaveDirection,
@@ -64,6 +72,8 @@ impl Default for PulseWaveShader {
             frequency_binding: None,
             speed: 1.0,
             color: ColorConfig::Magenta,
+            apply_to: ApplyToColor::Both,
+            uniform: false,
             direction: WaveDirection::Horizontal,
             wavelength: 8.0,
         }
@@ -126,22 +136,37 @@ impl PulseWaveShader {
 impl StyleShader for PulseWaveShader {
     fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
         let frequency = self.effective_frequency(ctx);
-        let blend_factor = self.blend_at(
-            ctx.local_x,
-            ctx.local_y,
-            ctx.width,
-            ctx.height,
-            ctx.t as f32,
-            frequency,
-        );
+        let blend_factor = if self.uniform {
+            let signal = Sine::new(1.0 / std::f32::consts::TAU, 1.0, 0.0, 0.0).normalized();
+            let wave_input = ctx.t as f32 * self.speed * frequency * std::f32::consts::TAU;
+            signal.sample(wave_input.into())
+        } else {
+            self.blend_at(
+                ctx.local_x,
+                ctx.local_y,
+                ctx.width,
+                ctx.height,
+                ctx.t as f32,
+                frequency,
+            )
+        };
         let pulse_color: Color = self.color.into();
 
         let mut result = base;
-        if base.fg != Color::TRANSPARENT {
+        if matches!(self.apply_to, ApplyToColor::Foreground | ApplyToColor::Both)
+            && base.fg != Color::TRANSPARENT
+        {
             result.fg = blend_colors(base.fg, pulse_color, blend_factor, ColorSpace::Rgb);
         }
-        if base.bg != Color::TRANSPARENT {
-            result.bg = blend_colors(base.bg, pulse_color, blend_factor * 0.3, ColorSpace::Rgb);
+        if matches!(self.apply_to, ApplyToColor::Background | ApplyToColor::Both)
+            && base.bg != Color::TRANSPARENT
+        {
+            let background_blend = if self.uniform {
+                blend_factor
+            } else {
+                blend_factor * 0.3
+            };
+            result.bg = blend_colors(base.bg, pulse_color, background_blend, ColorSpace::Rgb);
         }
         result
     }
@@ -165,6 +190,8 @@ mod tests {
             frequency_binding: None,
             speed: 1.0,
             color: ColorConfig::Red,
+            apply_to: ApplyToColor::Both,
+            uniform: false,
             direction: WaveDirection::Horizontal,
             wavelength: 4.0,
         };
@@ -191,6 +218,8 @@ mod tests {
             frequency_binding: None,
             speed: 1.0,
             color: ColorConfig::Cyan,
+            apply_to: ApplyToColor::Both,
+            uniform: false,
             direction: WaveDirection::Radial,
             wavelength: 10.0,
         };
@@ -266,6 +295,8 @@ mod tests {
             frequency_binding: Some("freq".to_string()),
             speed: 1.0,
             color: ColorConfig::Red,
+            apply_to: ApplyToColor::Both,
+            uniform: false,
             direction: WaveDirection::Horizontal,
             wavelength: 4.0,
         };
@@ -292,7 +323,33 @@ mod tests {
         // should differ at this fixed position.
         assert_ne!(out_a, out_b);
     }
+
+    #[test]
+    fn uniform_pulse_targets_both_channels_with_same_blend_factor() {
+        let shader = PulseWaveShader {
+            frequency: 2.0,
+            speed: 1.0,
+            color: ColorConfig::Rgb {
+                r: 255,
+                g: 100,
+                b: 100,
+            },
+            apply_to: ApplyToColor::Both,
+            uniform: true,
+            ..PulseWaveShader::default()
+        };
+        let ctx = ShaderContext::new(0, 0, 35, 3, 0, 0, 0.25, None, None);
+        let base = Style {
+            fg: Color::rgb(204, 0, 0),
+            bg: Color::rgb(50, 10, 10),
+            mods: Default::default(),
+        };
+        let out = shader.style_at(&ctx, base);
+
+        assert_eq!(out.fg, Color::rgb(229, 50, 50));
+        assert_eq!(out.bg, Color::rgb(152, 55, 55));
+    }
 }
 
 // <FILE>tui-vfx-style/src/models/cls_pulse_wave_shader.rs</FILE> - <DESC>Spatial pulse wave with rippling color</DESC>
-// <VERS>END OF VERSION: 1.4.0</VERS>
+// <VERS>END OF VERSION: 1.6.0</VERS>
diff --git a/crates/tui-vfx-style/src/models/v3/cls_vfx_motion_field_shader.rs b/crates/tui-vfx-style/src/models/v3/cls_vfx_motion_field_shader.rs
index 02a3147..6d2f52a 100644
--- a/crates/tui-vfx-style/src/models/v3/cls_vfx_motion_field_shader.rs
+++ b/crates/tui-vfx-style/src/models/v3/cls_vfx_motion_field_shader.rs
@@ -49,6 +49,8 @@ impl From<&PulseWaveShader> for VfxMotionFieldShader {
                 frequency_binding: shader.frequency_binding.clone(),
                 speed: shader.speed,
                 color: shader.color,
+                apply_to: shader.apply_to,
+                uniform: shader.uniform,
                 direction: shader.direction.into(),
                 wavelength: shader.wavelength,
             },
diff --git a/crates/tui-vfx-style/src/models/v3/enum_vfx_motion_field_behavior.rs b/crates/tui-vfx-style/src/models/v3/enum_vfx_motion_field_behavior.rs
index 961c78f..d20a9c5 100644
--- a/crates/tui-vfx-style/src/models/v3/enum_vfx_motion_field_behavior.rs
+++ b/crates/tui-vfx-style/src/models/v3/enum_vfx_motion_field_behavior.rs
@@ -46,6 +46,13 @@ pub enum VfxMotionFieldBehavior {
         speed: f32,
         /// Target pulse color.
         color: ColorConfig,
+        /// Target channel(s) affected by the pulse.
+        #[serde(default)]
+        apply_to: crate::models::ApplyToColor,
+        /// Whether all cells share one temporal pulse factor.
+        #[serde(default)]
+        #[config(default = false)]
+        uniform: bool,
         /// Field direction.
         #[serde(default)]
         direction: VfxMotionFieldDirection,
diff --git a/crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs b/crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs
index ec36639..d0ce908 100644
--- a/crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs
+++ b/crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs
@@ -476,6 +476,8 @@ impl From<&crate::models::v3::VfxMotionFieldShader> for SpatialShaderType {
                 frequency_binding,
                 speed,
                 color,
+                apply_to,
+                uniform,
                 direction,
                 wavelength,
             } => SpatialShaderType::PulseWave(PulseWaveShader {
@@ -483,6 +485,8 @@ impl From<&crate::models::v3::VfxMotionFieldShader> for SpatialShaderType {
                 frequency_binding: frequency_binding.clone(),
                 speed: *speed,
                 color: *color,
+                apply_to: *apply_to,
+                uniform: *uniform,
                 direction: (*direction).into(),
                 wavelength: *wavelength,
             }),
diff --git a/crates/tui-vfx-style/src/models/v3/test_vfx_motion_field_shader.rs b/crates/tui-vfx-style/src/models/v3/test_vfx_motion_field_shader.rs
index 483809b..5fc96f1 100644
--- a/crates/tui-vfx-style/src/models/v3/test_vfx_motion_field_shader.rs
+++ b/crates/tui-vfx-style/src/models/v3/test_vfx_motion_field_shader.rs
@@ -4,7 +4,9 @@
 // <CLOG>Extract focused conversion tests for VfxMotionFieldShader into a dedicated sibling file.</CLOG>
 
 use super::{VfxMotionFieldBehavior, VfxMotionFieldDirection, VfxMotionFieldShader};
-use crate::models::{ColorConfig, OrbitShader, PulseWaveShader, RadarShader, SpatialShaderType};
+use crate::models::{
+    ApplyToColor, ColorConfig, OrbitShader, PulseWaveShader, RadarShader, SpatialShaderType,
+};
 
 #[test]
 fn converts_pulse_wave_into_v3_motion_field_surface() {
@@ -13,6 +15,8 @@ fn converts_pulse_wave_into_v3_motion_field_surface() {
         frequency_binding: Some("freq".to_string()),
         speed: 1.25,
         color: ColorConfig::Magenta,
+        apply_to: ApplyToColor::Background,
+        uniform: true,
         direction: crate::models::WaveDirection::Radial,
         wavelength: 10.0,
     };
@@ -25,12 +29,38 @@ fn converts_pulse_wave_into_v3_motion_field_surface() {
             frequency_binding: Some("freq".to_string()),
             speed: 1.25,
             color: ColorConfig::Magenta,
+            apply_to: ApplyToColor::Background,
+            uniform: true,
             direction: VfxMotionFieldDirection::Radial,
             wavelength: 10.0,
         }
     );
 }
 
+#[test]
+fn lowers_v3_pulse_wave_channel_fields_back_to_legacy_shader() {
+    let grouped = VfxMotionFieldShader {
+        behavior: VfxMotionFieldBehavior::PulseWave {
+            frequency: 2.0,
+            frequency_binding: None,
+            speed: 1.0,
+            color: ColorConfig::Red,
+            apply_to: ApplyToColor::Both,
+            uniform: true,
+            direction: VfxMotionFieldDirection::Horizontal,
+            wavelength: 8.0,
+        },
+    };
+
+    let lowered = SpatialShaderType::from(&grouped);
+    let SpatialShaderType::PulseWave(shader) = lowered else {
+        panic!("expected PulseWave shader");
+    };
+
+    assert_eq!(shader.apply_to, ApplyToColor::Both);
+    assert!(shader.uniform);
+}
+
 #[test]
 fn converts_radar_into_v3_motion_field_surface() {
     let legacy = RadarShader {
```

## Claude output (raw)
```text
## Review: `style.pulse` → compositor PulseWave lowering

I verified the V2 oracle math, the test recipe, the `ApplyToColor` defaults, and that nothing else still calls the removed backend code. Findings below.

### Blockers

**B1. `applyTo` default silently changes from foreground to both.** `fnc_lower_recipe_graph_to_composition_spec.rs:3630-3636`:
```rust
fn pulse_apply_to_input(...) -> ApplyToColor {
    match enum_input(node, request, "applyTo") {
        Some("foreground" | "fg") => ApplyToColor::Foreground,
        Some("background" | "bg") => ApplyToColor::Background,
        _ => ApplyToColor::Both,           // <-- was "foreground"
    }
}
```
Previous lowering used `enum_label_input(..., "applyTo", "foreground")`, and `apply_pulse_style_stage` only blended fg unless `apply_to` was `"background"|"both"` — so a recipe omitting `applyTo` (or with a typo) got fg-only. The new fallback paints both channels, and unrecognized values land in `Both` instead of being rejected. Both shipped pulse recipes set `applyTo: "both"` so the V2-oracle CLI test still passes — but this is a real contract change for any author relying on the documented default. It also disagrees with `ApplyToColor::default() == Foreground` (defined at `cls_focused_row_gradient_shader.rs:121`), so the V3 PulseWave serde default and the recipe lowering default now diverge. Either preserve `Foreground` as the fallback, or use `strict_enum_input` and reject the unrecognized-value case explicitly. Don't silently re-route to `Both`.

**B2. No coverage for the lowering contract itself.** The CLI test only asserts pixel output; nothing pins `ShaderLayerSpec.shader == PulseWave { uniform: true, apply_to: Both, frequency_binding: Some(...), .. }`. Combined with B1, a future regression that flipped a default could pass the existing test if the recipe still happened to set `applyTo` explicitly. Add a focused unit/integration test against `lower_style_pulse` that verifies (a) `uniform: true`, (b) `apply_to` round-trips per recipe value, (c) `frequency_binding` is preserved when `frequency` is bound. The recipe is named `style_pulse_runtime_frequency.json` but ships a literal `2.0` — the binding-preservation path is unexercised.

### Non-blocking

**N1. Clever-not-clear Sine construction in the uniform branch** (`cls_pulse_wave_shader.rs:139-143`):
```rust
let signal = Sine::new(1.0 / std::f32::consts::TAU, 1.0, 0.0, 0.0).normalized();
let wave_input = ctx.t as f32 * self.speed * frequency * std::f32::consts::TAU;
signal.sample(wave_input.into())
```
Sine evaluates `sin(2π·(freq·t + phase))`. The `1/TAU` frequency exactly cancels the `*TAU` on the input — the whole expression reduces to `sin(ctx.t · speed · freq · 2π) · 0.5 + 0.5`. Idiomatic form: `Sine::new(self.speed * frequency, 1.0, 0.0, 0.0).normalized().sample(ctx.t.into())` — let Sine apply its own TAU. Constructing the Sine per cell call is also pointless; it can be hoisted or inlined as a scalar.

**N2. Lowering hardcodes inert fields** (`speed: 1.0`, `direction: Horizontal`, `wavelength: default().wavelength`). Fine while `uniform: true` makes them dead, but if the recipe schema later admits `speed` or `wavelength` they'll be silently dropped. One-line comment next to those hardcodes, or pull them through `resolved_number_input` with the V2 default.

**N3. New `apply_to` gating in `style_at` is only tested with `Both, uniform=true`.** Add a `(Foreground, uniform=false)` case proving bg is untouched, and a `(Background, uniform=true)` case proving fg is untouched. Without these, the matches! guards and the `blend_factor` vs `blend_factor * 0.3` branch on bg are partially covered.

**N4. CLOG drift** — both `cls_pulse_wave_shader.rs` and `fnc_lower_recipe_graph_to_composition_spec.rs` keep accumulating multi-line CLOGs. Project convention (per memory) is one-line CLOG describing only the most recent change; `git log` holds the running history. Not introduced here but the new entries continue the drift. Footer in the shader file also skipped `1.5.0` (was `1.4.0`, now `1.6.0`) — this catches up but worth a note in the team's housekeeping.

**N5. Removed code is fully reachable-from-nowhere — confirmed.** No remaining references to `NativeStyleStage::Pulse`, `apply_pulse_style_stage`, or `pulse_lerp_rgba_label` in the workspace. (`pulse_lerp_channel` exists in `tui-vfx-player/src/fnc_apply_style_primitive.rs` but is an independent helper for that crate, not a leftover from the deleted code.)

**N6. V2 oracle parity verified.** With recipe input `fg=rgb(205,0,0)`, `bg=rgb(50,10,10)`, `pulseColor=rgb(255,100,100)`, `frequency=2.0`, `phase_t=0.25`: strength = `sin(0.25·2·2π)·0.5 + 0.5 = 0.5`, fg = `(205+255)/2 = 230`, bg = `(50+255)/2 = 152.5 → 152`. Matches the asserted `rgba(230,50,50,255)` / `rgba(152,55,55,255)`. The shader unit test's `229` differs only because it feeds `204` instead of `205` — internally consistent.

### Recommendation

Fix B1 (preserve foreground default or reject unknown values strictly) and add B2's lowering-contract test before merge. The rest are quality polish.
```

## Follow-up actions after Claude review

Claude's blockers were addressed in the final on-disk state before commit:

- Fixed `style.pulse` applyTo handling so omitted `applyTo` defaults to foreground only, matching the previous recipe adapter contract and `ApplyToColor::default()`.
- Changed unknown `applyTo` values from silent `Both` fallback to an unsupported-native lowering rejection.
- Added CLI regressions for omitted `applyTo` (foreground-only/background unchanged) and invalid `applyTo` rejection.
- Kept the explicit V2 oracle recipe coverage for `applyTo: both`, proving `shaderLayers=1`, `styleStages=0`, and full card color parity.
- Kept V3 motion-field round-trip coverage for `apply_to` and `uniform`, plus existing runtime frequency resolution coverage in the shader unit tests.

Post-fix verification: cargo check passed, targeted tui-vfx-style and tui-vfx-player-cli nextest runs passed, strict render-backend showed fallbackUsed=false/nativeLoweringSucceeded=true/shaderLayers=1/styleStages=0/colorCount=105, clippy --tests -D warnings passed, fmt --check passed, and git diff --check passed.
