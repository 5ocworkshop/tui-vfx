# Claude style.glitch review

## Original user task
Move v3.1 recipe effects through compositor-owned IR, remove wrong-layer player/backend effect logic, no fallbacks, commit at feature seams.

## Final prompt sent to Claude CLI
```text
Review this Rust diff for tui-vfx. Task: migrate v3.1 style.glitch out of backend/player style-stage emulation and into compositor-owned IR by lowering to SpatialShaderType::GlitchLines ShaderLayerSpec. Requirements: preserve recipe settings (seed, intensity, italicStart, italicEnd), preserve V2-visible cyan foreground and italic-window styling, no fallback, remove wrong-layer backend style code, and keep V3 edge-distortion conversions from dropping new fields. Focus on correctness, layering violations, silent semantic gaps, missing tests, and compile/clippy hazards. Return blockers first, then non-blocking suggestions.

Diff follows:
diff --git a/crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs b/crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs
index 70d6273..813d5ee 100644
--- a/crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs
+++ b/crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs
@@ -1,7 +1,8 @@
 // <FILE>crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs</FILE> - <DESC>Lower player render requests into compositor CompositionSpec modes</DESC>
 // <VERS>VERSION: 0.41.0</VERS>
 // <WCTX>Native compositor lowering: map bounded v3.1 recipe graph effects into native CompositionSpec and source-stage content/style/filter work with honest fallback diagnostics.</WCTX>
-// <CLOG>0.42.0: MINOR — lower style.neonFlicker into compositor NeonFlickerShader layers and remove backend style-stage emulation.
+// <CLOG>0.43.0: MINOR — lower style.glitch into compositor GlitchLinesShader layers and remove backend style-stage emulation.
+// 0.42.0: MINOR — lower style.neonFlicker into compositor NeonFlickerShader layers and remove backend style-stage emulation.
 // 0.41.0: MINOR — lower focused-row-gradient style.spatial into compositor ShaderLayerSpec and remove backend style-stage emulation.
 // 0.40.0: MINOR — lower shader.diffusion into compositor ShaderLayerSpec and remove backend style-stage emulation.
 // 0.39.0: MINOR — lower shader.barberPole into compositor ShaderLayerSpec and remove backend style-stage emulation.
@@ -54,10 +55,11 @@ use tui_vfx_style::models::{
     ApplyToColor, BarberPoleApplyTo, BarberPoleShader, BindableU16, BorderSweepShader, ColorConfig,
     ColorSpace, DiffusionApplyTo, DiffusionShader, DiffusionSource, FocusFieldApplyTo,
     FocusFieldShader, FocusFieldShape, FocusedRowGradientShader, GlistenApplyTo, GlistenBandShader,
-    GlistenDirection, Gradient, HighlighterApplyTo, HighlighterDirection, HighlighterMode,
-    HighlighterRowMask, HighlighterShader, LinearGradientApplyTo, LinearGradientShader,
-    NeonFlickerShader, RadarShader, RevealWipeShader, SegmentMode, SpatialShaderType, StyleRegion,
-    TextContrast, WayfindingNode, WayfindingNodeApplyTo, WayfindingNodeShader,
+    GlistenDirection, GlitchLinesShader, Gradient, HighlighterApplyTo, HighlighterDirection,
+    HighlighterMode, HighlighterRowMask, HighlighterShader, LinearGradientApplyTo,
+    LinearGradientShader, NeonFlickerShader, RadarShader, RevealWipeShader, SegmentMode,
+    SpatialShaderType, StyleRegion, TextContrast, WayfindingNode, WayfindingNodeApplyTo,
+    WayfindingNodeShader,
 };
 
 const SUPPORTED_WIPE_DIRECTIONS: &[&str] = &[
@@ -175,13 +177,6 @@ pub enum NativeStyleStage {
     },
     /// Apply V2-compatible rainbow foreground cycling.
     Rainbow { rotation_speed: f64 },
-    /// Apply V2-compatible glitch foreground/italic styling.
-    Glitch {
-        seed: usize,
-        intensity: f64,
-        italic_start: f64,
-        italic_end: f64,
-    },
     /// Apply player-compatible color fade styling to existing foreground/background channels.
     ColorFade { target: String, color_space: String },
     /// Apply player-compatible HSL color shift styling to existing channels.
@@ -561,7 +556,7 @@ fn lower_node_into_spec(
         "style.moduloColumns" => lower_style_modulo_columns(node, style_stages, request, warnings),
         "style.neonFlicker" => lower_style_neon_flicker(node, spec, request, warnings),
         "style.rainbow" => lower_style_rainbow(node, style_stages, request, warnings),
-        "style.glitch" => lower_style_glitch(node, style_stages, request, warnings),
+        "style.glitch" => lower_style_glitch(node, spec, request, warnings),
         "style.spatial" => lower_style_spatial(node, spec, style_stages, request, warnings),
         other => NodeLoweringOutcome::Unsupported {
             reason: format!("Effect `{other}` is not yet supported by compositor-native lowering."),
@@ -1878,7 +1873,7 @@ fn lower_style_rainbow(
 
 fn lower_style_glitch(
     node: &NodeSpec,
-    style_stages: &mut Vec<NativeStyleStage>,
+    spec: &mut CompositionSpec,
     request: &PlayerRenderBackendRequest,
     warnings: Vec<PlayerRenderBackendDiagnostic>,
 ) -> NodeLoweringOutcome {
@@ -1891,12 +1886,32 @@ fn lower_style_glitch(
         return NodeLoweringOutcome::Unsupported { reason };
     }
 
-    let italic_start = number_input(node, request, "italicStart", 0.0).clamp(0.0, 1.0);
-    style_stages.push(NativeStyleStage::Glitch {
-        seed: integer_input(node, request, "seed", 0).max(0) as usize,
-        intensity: number_input(node, request, "intensity", 0.5).clamp(0.0, 1.0),
-        italic_start,
-        italic_end: number_input(node, request, "italicEnd", 1.0).clamp(italic_start, 1.0),
+    let italic_start = resolved_number_input(node, request, "italicStart", 0.0).clamp(0.0, 1.0);
+    spec.shader_layers.push(ShaderLayerSpec {
+        shader: SpatialShaderType::GlitchLines(GlitchLinesShader {
+            seed: resolved_integer_input(node, request, "seed", 0).max(0) as u64,
+            intensity: resolved_number_input(node, request, "intensity", 0.5).clamp(0.0, 1.0)
+                as f32,
+            max_lines: request.source_ir.height.max(1).min(u16::MAX as usize) as u16,
+            speed: 1.0,
+            flash_chance: 0.0,
+            pulse_color: None,
+            base_color: Some(ColorConfig::Rgb {
+                r: 0,
+                g: 255,
+                b: 255,
+            }),
+            italic_start: Some(italic_start as f32),
+            italic_end: Some(
+                resolved_number_input(node, request, "italicEnd", 1.0).clamp(italic_start, 1.0)
+                    as f32,
+            ),
+            pulse_speed: 0.5,
+            italic_on_flash: false,
+            flash_hold: 1,
+            noise_type: Default::default(),
+        }),
+        region: StyleRegion::All,
     });
     NodeLoweringOutcome::Lowered { warnings }
 }
diff --git a/crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs b/crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs
index 4ce6016..55e01a8 100644
--- a/crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs
+++ b/crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs
@@ -276,14 +276,6 @@ fn scene_ir_with_native_content_stages(
             NativeStyleStage::Rainbow { rotation_speed } => {
                 apply_rainbow_style_stage(&mut staged, *rotation_speed)
             }
-            NativeStyleStage::Glitch {
-                seed,
-                intensity,
-                italic_start,
-                italic_end,
-            } => {
-                apply_glitch_style_stage(&mut staged, *seed, *intensity, *italic_start, *italic_end)
-            }
             NativeStyleStage::ColorFade {
                 target,
                 color_space,
@@ -756,30 +748,6 @@ fn apply_rainbow_style_stage(report: &mut PlayerRenderIrReport, _rotation_speed:
     }
 }
 
-fn apply_glitch_style_stage(
-    report: &mut PlayerRenderIrReport,
-    _seed: usize,
-    _intensity: f64,
-    italic_start: f64,
-    italic_end: f64,
-) {
-    let width = report_width(report);
-    let height = report_height(report);
-    let italic = (italic_start..=italic_end).contains(&report.phase_t);
-    for y in 0..height {
-        for x in 0..width {
-            set_report_cell_style(
-                report,
-                x,
-                y,
-                Some(rgba_label(0, 255, 255, 255).as_str()),
-                None,
-                italic.then_some("italic"),
-            );
-        }
-    }
-}
-
 fn apply_color_fade_style_stage(
     report: &mut PlayerRenderIrReport,
     target: &str,
diff --git a/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs b/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
index 55b8a1a..9d9ffd2 100644
--- a/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
+++ b/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
@@ -147,6 +147,53 @@ fn test_fnc_cli_capture_cells_writes_procedural_recipe_metadata() {
     assert_eq!(run_schema, "v3.1.player.cellCapture.sqlite.1");
 }
 
+#[test]
+fn test_fnc_cli_lowers_glitch_style_to_compositor_shader_layer_not_style_stage_json() {
+    let report = player_cli_json(
+        vec![
+            str_arg("render-backend"),
+            str_arg("--recipe"),
+            recipe_path("styles/style_glitch.json"),
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
+            str_arg("0.35"),
+        ],
+        "render-backend native glitch style compositor shader layer player cli",
+    );
+
+    assert_eq!(report["backend"], "compositor");
+    assert_eq!(report["recipeId"], "debugStyleGlitchV2Oracle");
+    assert_eq!(report["compositionMode"], "native");
+    assert_eq!(report["fallbackUsed"], false);
+    assert_eq!(report["nativeLoweringSucceeded"], true);
+    assert_eq!(report["compositionSpecSummary"]["shaderLayers"], 1);
+    assert_eq!(report["compositionSpecSummary"]["styleStages"], 0);
+    assert_eq!(
+        report["loweredEffectIds"],
+        serde_json::json!(["style.glitch"])
+    );
+    assert_eq!(
+        report["styledCells"][0]["foreground"],
+        "rgba(0,255,255,255)"
+    );
+    assert!(
+        report["styledCells"][0]["modifiers"]
+            .as_array()
+            .expect("modifiers array")
+            .contains(&serde_json::json!("italic"))
+    );
+}
+
 #[test]
 fn test_fnc_cli_lowers_neon_flicker_style_to_compositor_shader_layer_not_style_stage_json() {
     let report = player_cli_json(
diff --git a/crates/tui-vfx-style/src/models/cls_glitch_lines_shader.rs b/crates/tui-vfx-style/src/models/cls_glitch_lines_shader.rs
index 0b3f4c9..b009011 100644
--- a/crates/tui-vfx-style/src/models/cls_glitch_lines_shader.rs
+++ b/crates/tui-vfx-style/src/models/cls_glitch_lines_shader.rs
@@ -1,7 +1,8 @@
 // <FILE>tui-vfx-style/src/models/cls_glitch_lines_shader.rs</FILE> - <DESC>Spatial glitch with random interference lines</DESC>
-// <VERS>VERSION: 1.8.0</VERS>
-// <WCTX>Adding screen coordinate context to shaders</WCTX>
-// <CLOG>Updated to use ShaderContext for screen-space effects</CLOG>
+// <VERS>VERSION: 1.9.0</VERS>
+// <WCTX>Compositor-owned v3.1 style.glitch lowering needs the edge-distortion shader to carry legacy style color and italic-window knobs without backend emulation.</WCTX>
+// <CLOG>1.9.0: add optional base-color and italic-window styling so v3.1 glitch style recipes can lower into compositor shader layers.
+// Updated to use ShaderContext for screen-space effects</CLOG>
 
 use crate::models::{ColorConfig, NoiseType};
 
@@ -20,7 +21,7 @@ use crate::models::ColorSpace;
 use crate::traits::{ShaderContext, StyleShader};
 use crate::utils::blend_colors;
 use serde::{Deserialize, Serialize};
-use tui_vfx_types::{Color, Style};
+use tui_vfx_types::{Color, Modifiers, Style};
 
 /// Spatial glitch shader that creates random horizontal interference lines.
 ///
@@ -50,6 +51,18 @@ pub struct GlitchLinesShader {
     /// Optional color to pulse towards (e.g., white for cyan→white→cyan)
     #[serde(default)]
     pub pulse_color: Option<ColorConfig>,
+    /// Optional foreground color to install before glitch styling.
+    #[serde(default, skip_serializing_if = "Option::is_none")]
+    #[config(opaque)]
+    pub base_color: Option<ColorConfig>,
+    /// Optional normalized time at which italic styling begins.
+    #[serde(default, skip_serializing_if = "Option::is_none")]
+    #[config(opaque)]
+    pub italic_start: Option<f32>,
+    /// Optional normalized time at which italic styling ends.
+    #[serde(default, skip_serializing_if = "Option::is_none")]
+    #[config(opaque)]
+    pub italic_end: Option<f32>,
     /// Speed of the color pulse cycle (cycles per animation)
     #[serde(default = "default_pulse_speed")]
     #[config(default = 0.5)]
@@ -78,6 +91,9 @@ impl Default for GlitchLinesShader {
             speed: 1.0,
             flash_chance: 0.0,
             pulse_color: None,
+            base_color: None,
+            italic_start: None,
+            italic_end: None,
             pulse_speed: 0.5,
             italic_on_flash: false,
             flash_hold: 1,
@@ -175,6 +191,14 @@ impl StyleShader for GlitchLinesShader {
         let t = ctx.t as f32;
         let (y, height) = (ctx.local_y, ctx.height);
         let mut result = base;
+        if let Some(color) = self.base_color {
+            result.fg = Color::from(color);
+        }
+        if let (Some(start), Some(end)) = (self.italic_start, self.italic_end)
+            && (start..=end).contains(&t)
+        {
+            result.mods = result.mods.combine(Modifiers::italic());
+        }
 
         // Apply color pulse if configured
         if let Some(ref pulse_cfg) = self.pulse_color {
@@ -228,6 +252,9 @@ mod tests {
             speed: 1.0,
             flash_chance: 0.0,
             pulse_color: None,
+            base_color: None,
+            italic_start: None,
+            italic_end: None,
             pulse_speed: 0.5,
             italic_on_flash: false,
             flash_hold: 1,
@@ -261,6 +288,9 @@ mod tests {
             speed: 1.0,
             flash_chance: 0.0,
             pulse_color: None,
+            base_color: None,
+            italic_start: None,
+            italic_end: None,
             pulse_speed: 0.5,
             italic_on_flash: false,
             flash_hold: 1,
@@ -275,7 +305,23 @@ mod tests {
             );
         }
     }
+
+    #[test]
+    fn test_base_color_and_italic_window_are_applied_before_glitch_lines() {
+        let shader = GlitchLinesShader {
+            intensity: 0.0,
+            base_color: Some(ColorConfig::Cyan),
+            italic_start: Some(0.3),
+            italic_end: Some(0.7),
+            ..GlitchLinesShader::default()
+        };
+        let ctx = ShaderContext::new(0, 0, 8, 4, 0, 0, 0.35, None, None);
+        let out = shader.style_at(&ctx, Style::default());
+
+        assert_eq!(out.fg, Color::from(ColorConfig::Cyan));
+        assert!(out.mods.italic);
+    }
 }
 
 // <FILE>tui-vfx-style/src/models/cls_glitch_lines_shader.rs</FILE> - <DESC>Spatial glitch with random interference lines</DESC>
-// <VERS>END OF VERSION: 1.8.0</VERS>
+// <VERS>END OF VERSION: 1.9.0</VERS>
diff --git a/crates/tui-vfx-style/src/models/v3/cls_vfx_edge_distortion_shader.rs b/crates/tui-vfx-style/src/models/v3/cls_vfx_edge_distortion_shader.rs
index 3b363fa..0a1f116 100644
--- a/crates/tui-vfx-style/src/models/v3/cls_vfx_edge_distortion_shader.rs
+++ b/crates/tui-vfx-style/src/models/v3/cls_vfx_edge_distortion_shader.rs
@@ -48,6 +48,9 @@ impl From<&GlitchLinesShader> for VfxEdgeDistortionShader {
                 speed: shader.speed,
                 flash_chance: shader.flash_chance,
                 pulse_color: shader.pulse_color,
+                base_color: shader.base_color,
+                italic_start: shader.italic_start,
+                italic_end: shader.italic_end,
                 pulse_speed: shader.pulse_speed,
                 italic_on_flash: shader.italic_on_flash,
                 flash_hold: shader.flash_hold,
diff --git a/crates/tui-vfx-style/src/models/v3/enum_vfx_edge_distortion_behavior.rs b/crates/tui-vfx-style/src/models/v3/enum_vfx_edge_distortion_behavior.rs
index 8d38ec1..1661313 100644
--- a/crates/tui-vfx-style/src/models/v3/enum_vfx_edge_distortion_behavior.rs
+++ b/crates/tui-vfx-style/src/models/v3/enum_vfx_edge_distortion_behavior.rs
@@ -51,6 +51,18 @@ pub enum VfxEdgeDistortionBehavior {
         /// Optional pulse color.
         #[serde(default, skip_serializing_if = "Option::is_none")]
         pulse_color: Option<ColorConfig>,
+        /// Optional foreground color installed before glitch styling.
+        #[serde(default, skip_serializing_if = "Option::is_none")]
+        #[config(opaque)]
+        base_color: Option<ColorConfig>,
+        /// Optional normalized time at which italic styling begins.
+        #[serde(default, skip_serializing_if = "Option::is_none")]
+        #[config(opaque)]
+        italic_start: Option<f32>,
+        /// Optional normalized time at which italic styling ends.
+        #[serde(default, skip_serializing_if = "Option::is_none")]
+        #[config(opaque)]
+        italic_end: Option<f32>,
         /// Pulse speed.
         #[config(default = 0.5)]
         pulse_speed: f32,
diff --git a/crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs b/crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs
index ca88879..27df713 100644
--- a/crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs
+++ b/crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs
@@ -539,6 +539,9 @@ impl From<&crate::models::v3::VfxEdgeDistortionShader> for SpatialShaderType {
                 speed,
                 flash_chance,
                 pulse_color,
+                base_color,
+                italic_start,
+                italic_end,
                 pulse_speed,
                 italic_on_flash,
                 flash_hold,
@@ -550,6 +553,9 @@ impl From<&crate::models::v3::VfxEdgeDistortionShader> for SpatialShaderType {
                 speed: *speed,
                 flash_chance: *flash_chance,
                 pulse_color: *pulse_color,
+                base_color: *base_color,
+                italic_start: *italic_start,
+                italic_end: *italic_end,
                 pulse_speed: *pulse_speed,
                 italic_on_flash: *italic_on_flash,
                 flash_hold: *flash_hold,
diff --git a/crates/tui-vfx-style/src/models/v3/test_vfx_edge_distortion_shader.rs b/crates/tui-vfx-style/src/models/v3/test_vfx_edge_distortion_shader.rs
index bcb7b1f..e763a82 100644
--- a/crates/tui-vfx-style/src/models/v3/test_vfx_edge_distortion_shader.rs
+++ b/crates/tui-vfx-style/src/models/v3/test_vfx_edge_distortion_shader.rs
@@ -18,6 +18,9 @@ fn converts_glitch_lines_into_v3_edge_distortion_surface() {
         speed: 1.5,
         flash_chance: 0.2,
         pulse_color: Some(ColorConfig::White),
+        base_color: Some(ColorConfig::Cyan),
+        italic_start: Some(0.3),
+        italic_end: Some(0.7),
         pulse_speed: 0.75,
         italic_on_flash: true,
         flash_hold: 3,
@@ -34,6 +37,9 @@ fn converts_glitch_lines_into_v3_edge_distortion_surface() {
             speed: 1.5,
             flash_chance: 0.2,
             pulse_color: Some(ColorConfig::White),
+            base_color: Some(ColorConfig::Cyan),
+            italic_start: Some(0.3),
+            italic_end: Some(0.7),
             pulse_speed: 0.75,
             italic_on_flash: true,
             flash_hold: 3,
@@ -42,6 +48,36 @@ fn converts_glitch_lines_into_v3_edge_distortion_surface() {
     );
 }
 
+#[test]
+fn lowers_v3_glitch_lines_style_fields_back_to_legacy_shader() {
+    let grouped = VfxEdgeDistortionShader {
+        behavior: VfxEdgeDistortionBehavior::GlitchLines {
+            seed: 7,
+            intensity: 0.8,
+            max_lines: 9,
+            speed: 1.5,
+            flash_chance: 0.2,
+            pulse_color: Some(ColorConfig::White),
+            base_color: Some(ColorConfig::Cyan),
+            italic_start: Some(0.3),
+            italic_end: Some(0.7),
+            pulse_speed: 0.75,
+            italic_on_flash: true,
+            flash_hold: 3,
+            noise_type: NoiseType::Gaussian,
+        },
+    };
+
+    let lowered = SpatialShaderType::from(&grouped);
+    let SpatialShaderType::GlitchLines(shader) = lowered else {
+        panic!("expected GlitchLines shader");
+    };
+
+    assert_eq!(shader.base_color, Some(ColorConfig::Cyan));
+    assert_eq!(shader.italic_start, Some(0.3));
+    assert_eq!(shader.italic_end, Some(0.7));
+}
+
 #[test]
 fn converts_chromatic_edge_into_v3_edge_distortion_surface() {
     let legacy = ChromaticEdgeShader {
```

## Claude output (raw)
```text
I checked the recipe (`debugStyleGlitchV2Oracle` with `intensity: 0.6`, `seed: 42`, italic window 0.3–0.7), the new `style_at` flow, and helper signatures. Below.

## Blockers

### 1. New path adds `underline` on glitch rows; V2 oracle never did
The old `apply_glitch_style_stage` took `_seed: usize, _intensity: f64` (both ignored) and produced cyan + italic-window only. The new shader's `style_at` (cls_glitch_lines_shader.rs:226) does:
```rust
if self.row_has_line(y, height, t) {
    result = result.underline();
}
```
At the recipe's `intensity: 0.6`, `seed: 42`, `max_lines: source_ir.height = 3`, `row_has_line` will mark rows as underlined. That is a visible modifier the V2 oracle does not emit. The task said "preserve V2-visible cyan foreground and italic-window styling" — this path adds a third modifier on a randomized row schedule.

Either:
- pin `intensity: 0.0` in the lowering and accept that the recipe `intensity` input is honored shape-wise but not visually (V2 emulator did exactly this), **and** add a regression assertion that no cell carries `underline`; or
- gate `row_has_line` behind a flag (e.g., a new `lines_enabled: bool` defaulting true, set false in this lowering path) so the legacy oracle parity is preserved while the shader stays available for non-V2 callers.

The current code claims V2 parity in the CLOG but produces a divergent grid.

### 2. The new CLI test cannot detect the regression in (1)
`test_fnc_cli_lowers_glitch_style_to_compositor_shader_layer_not_style_stage_json` only asserts:
- `styledCells[0]["foreground"] == "rgba(0,255,255,255)"`
- `styledCells[0]["modifiers"].contains("italic")`

It does not assert absence of `underline`, does not check any cell beyond index 0, and `contains` (not equality) means italic-only and italic+underline both pass. With `max_lines = 3` and a 3-row recipe at intensity 0.6, ~one row will currently render with `underline`, but the test won't catch it. Add at minimum:
- assert no styledCell has `underline` in modifiers (V2 parity check);
- a second invocation at `--phase-t 0.0` (or 0.9) asserting italic is absent — the diff adds italic-window code in the shader but no test exercises the bound.

### 3. `max_lines` derived from `request.source_ir.height`
```rust
max_lines: request.source_ir.height.max(1).min(u16::MAX as usize) as u16,
```
The recipe has no `maxLines` input, so a default is needed, but coupling it to the source IR grid height changes shader output as a function of element size, which the V2 emulator never did. Combined with (1), this is what makes the regression visible-grid-shaped. If you keep glitch lines on, pick a stable constant default consistent with `GlitchLinesShader::default().max_lines` (=3 from the file's existing default) so behavior is at least deterministic across grid sizes.

### 4. `#[config(opaque)]` inconsistency on the V3 group enum
In `enum_vfx_edge_distortion_behavior.rs`, the existing `pulse_color: Option<ColorConfig>` is *not* `#[config(opaque)]`, but the three new fields are. Either `pulse_color` is wrong upstream and should also be opaque, or the new fields should match its pattern. Whichever is right, the round-trip diff currently mixes conventions for semantically identical `Option<ColorConfig>` fields. Pick one and apply uniformly so the autogen capability manifest doesn't render the variant inconsistently.

## Non-blocking

- **Field naming.** `base_color` collides conceptually with the `base: Style` parameter in `style_at`. Prefer `fg_color` / `foreground_color`. Also: the rustdoc says "installed before glitch styling," but it's also installed before `pulse_color` blending; rephrase to "installed as the starting foreground before any glitch-line, pulse, or flash effects."
- **Layering.** Putting "install a foreground color" and an "italic time window" on a *spatial shader* mixes presentation policy into a procedural primitive. It works for the migration, but if other recipe lowerings need similar pre-shader styling, you'll be tempted to repeat it. Consider whether a small "pre-style" layer alongside the shader layer is a cleaner structural home — out of scope for this packet, but worth a follow-up note.
- **Rustdoc.** Per the project rule about improving rustdocs on every edit: the three new fields each get one terse sentence. Mention the units (normalized phase-t in 0..=1), the both-or-neither contract for italic_start/italic_end (the impl silently drops italic when only one is set), and that `base_color` overrides whatever `base.fg` the upstream stages set.
- **CLOG style.** Per your memory rule, CLOG should be a single most-recent-change line, not a growing changelog. The file already accumulates history; this packet appends another. Consider replacing the block with one line and letting `git log` carry the rest.
- **Cast clarity.** `request.source_ir.height.max(1).min(u16::MAX as usize) as u16` reads cleaner as `u16::try_from(request.source_ir.height.max(1)).unwrap_or(u16::MAX)`. Minor.
- **Defaulted shader fields are noise.** `pulse_speed: 0.5`, `pulse_color: None`, `flash_chance: 0.0`, `flash_hold: 1`, `noise_type: Default::default()`, `italic_on_flash: false` are all defaults; could use `..GlitchLinesShader::default()` for the inactive knobs to keep intent (seed/intensity/colors/italic-window) visible.
- **Edge-distortion test.** Good that you added a round-trip test for the three new fields; consider also asserting `pulse_color`/`pulse_speed` survive the round trip in the same test so a future field reorder doesn't silently drop one.

The shape of the migration (remove style-stage variant, lower into ShaderLayerSpec, plumb the new fields through V3 round-trip) is correct. The blocker is the V2-parity gap created by leaving `row_has_line` and `intensity` active in a path advertised as V2-equivalent, plus a test that doesn't notice.
```

## Follow-up actions after Claude review

Claude's blockers were addressed in the final on-disk state before commit:

- Added `lines_enabled` to `GlitchLinesShader` and the V3 edge-distortion family so the compositor shader can preserve V2 style.glitch parity without adding underline interference lines. The v3.1 `style.glitch` lowering sets `lines_enabled: false` while still carrying seed/intensity in the shader payload.
- Replaced grid-height-derived `max_lines` with the shader default to avoid element-size-shaped behavior.
- Strengthened the CLI regression to assert no styled cell receives `underline`, and added a second render outside the italic window to prove italic ends after `italicEnd`.
- Made the V3 edge-distortion conversion carry `lines_enabled` in both directions and extended round-trip assertions.
- Aligned the new V3 option fields with the existing `pulse_color` schema pattern by removing opaque annotations.
- Fixed `pulse_color` blending to use the current `result.fg`, so an explicit base color is the pulse source rather than being bypassed by the pre-style step.

Post-fix verification: cargo check passed, targeted tui-vfx-style and tui-vfx-player-cli nextest runs passed, strict render-backend showed fallbackUsed=false/nativeLoweringSucceeded=true/shaderLayers=1/styleStages=0/underlineCount=0, clippy --tests -D warnings passed, fmt --check passed, and git diff --check passed.
