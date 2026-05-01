# Claude style.neonFlicker review

## Original user task
Move v3.1 recipe effects through compositor-owned IR, remove wrong-layer player/backend effect logic, no fallbacks, commit at feature seams.

## Final prompt sent to Claude CLI
```text
Review this Rust diff for tui-vfx. Task: migrate v3.1 style.neonFlicker out of backend/player style-stage emulation and into compositor-owned IR by lowering to SpatialShaderType::NeonFlicker ShaderLayerSpec. Requirements: preserve recipe settings (color, stability, dimAmount, italicWindow), no fallback, remove deprecated wrong-layer tests/code, and keep compatibility with existing V3 stochastic texture conversions. Focus on correctness, layering violations, silent semantic gaps, missing tests, and compile/clippy hazards. Return blockers first, then non-blocking suggestions.

Diff follows:
diff --git a/crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs b/crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs
index a9cd095..1956323 100644
--- a/crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs
+++ b/crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs
@@ -1,7 +1,8 @@
 // <FILE>crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs</FILE> - <DESC>Lower player render requests into compositor CompositionSpec modes</DESC>
 // <VERS>VERSION: 0.41.0</VERS>
 // <WCTX>Native compositor lowering: map bounded v3.1 recipe graph effects into native CompositionSpec and source-stage content/style/filter work with honest fallback diagnostics.</WCTX>
-// <CLOG>0.41.0: MINOR — lower focused-row-gradient style.spatial into compositor ShaderLayerSpec and remove backend style-stage emulation.
+// <CLOG>0.42.0: MINOR — lower style.neonFlicker into compositor NeonFlickerShader layers and remove backend style-stage emulation.
+// 0.41.0: MINOR — lower focused-row-gradient style.spatial into compositor ShaderLayerSpec and remove backend style-stage emulation.
 // 0.40.0: MINOR — lower shader.diffusion into compositor ShaderLayerSpec and remove backend style-stage emulation.
 // 0.39.0: MINOR — lower shader.barberPole into compositor ShaderLayerSpec and remove backend style-stage emulation.
 // 0.38.0: MINOR — lower shader.wayfindingNode into compositor ShaderLayerSpec and remove backend style-stage emulation.
@@ -55,8 +56,8 @@ use tui_vfx_style::models::{
     FocusFieldShader, FocusFieldShape, FocusedRowGradientShader, GlistenApplyTo, GlistenBandShader,
     GlistenDirection, Gradient, HighlighterApplyTo, HighlighterDirection, HighlighterMode,
     HighlighterRowMask, HighlighterShader, LinearGradientApplyTo, LinearGradientShader,
-    RadarShader, RevealWipeShader, SpatialShaderType, StyleRegion, TextContrast, WayfindingNode,
-    WayfindingNodeApplyTo, WayfindingNodeShader,
+    NeonFlickerShader, RadarShader, RevealWipeShader, SegmentMode, SpatialShaderType, StyleRegion,
+    TextContrast, WayfindingNode, WayfindingNodeApplyTo, WayfindingNodeShader,
 };
 
 const SUPPORTED_WIPE_DIRECTIONS: &[&str] = &[
@@ -172,13 +173,6 @@ pub enum NativeStyleStage {
         foreground: String,
         background: String,
     },
-    /// Apply deterministic neon flicker styling.
-    NeonFlicker {
-        color: String,
-        stability: f64,
-        dim_amount: f64,
-        italic_window: bool,
-    },
     /// Apply V2-compatible rainbow foreground cycling.
     Rainbow { rotation_speed: f64 },
     /// Apply V2-compatible glitch foreground/italic styling.
@@ -565,7 +559,7 @@ fn lower_node_into_spec(
         "style.pulse" => lower_style_pulse(node, style_stages, request, warnings),
         "style.italicWindow" => lower_style_italic_window(node, style_stages, request, warnings),
         "style.moduloColumns" => lower_style_modulo_columns(node, style_stages, request, warnings),
-        "style.neonFlicker" => lower_style_neon_flicker(node, style_stages, request, warnings),
+        "style.neonFlicker" => lower_style_neon_flicker(node, spec, request, warnings),
         "style.rainbow" => lower_style_rainbow(node, style_stages, request, warnings),
         "style.glitch" => lower_style_glitch(node, style_stages, request, warnings),
         "style.spatial" => lower_style_spatial(node, spec, style_stages, request, warnings),
@@ -1814,7 +1808,7 @@ fn lower_style_modulo_columns(
 
 fn lower_style_neon_flicker(
     node: &NodeSpec,
-    style_stages: &mut Vec<NativeStyleStage>,
+    spec: &mut CompositionSpec,
     request: &PlayerRenderBackendRequest,
     warnings: Vec<PlayerRenderBackendDiagnostic>,
 ) -> NodeLoweringOutcome {
@@ -1827,17 +1821,28 @@ fn lower_style_neon_flicker(
         return NodeLoweringOutcome::Unsupported { reason };
     }
 
-    style_stages.push(NativeStyleStage::NeonFlicker {
-        color: color_label_from_config(color_input(node, request, "color").unwrap_or(
-            ColorConfig::Rgb {
-                r: 255,
-                g: 50,
-                b: 150,
-            },
-        )),
-        stability: number_input(node, request, "stability", 0.7).clamp(0.0, 1.0),
-        dim_amount: number_input(node, request, "dimAmount", 0.5).clamp(0.0, 1.0),
-        italic_window: bool_input(node, request, "italicWindow", false),
+    spec.shader_layers.push(ShaderLayerSpec {
+        shader: SpatialShaderType::NeonFlicker(NeonFlickerShader {
+            stability: resolved_number_input(node, request, "stability", 0.7).clamp(0.0, 1.0)
+                as f32,
+            seed: 42,
+            segment: SegmentMode::Row,
+            dim_amount: resolved_number_input(node, request, "dimAmount", 0.5).clamp(0.0, 1.0)
+                as f32,
+            base_color: Some(resolved_color_input(node, request, "color").unwrap_or(
+                ColorConfig::Rgb {
+                    r: 255,
+                    g: 50,
+                    b: 150,
+                },
+            )),
+            italic_window: bool_input(node, request, "italicWindow", false),
+            speed: 1.0,
+            flash_chance: 0.0,
+            decay_rate: None,
+            noise_type: Default::default(),
+        }),
+        region: StyleRegion::All,
     });
     NodeLoweringOutcome::Lowered { warnings }
 }
diff --git a/crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs b/crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs
index 6ab16f7..4ce6016 100644
--- a/crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs
+++ b/crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs
@@ -273,18 +273,6 @@ fn scene_ir_with_native_content_stages(
                 foreground,
                 background,
             ),
-            NativeStyleStage::NeonFlicker {
-                color,
-                stability,
-                dim_amount,
-                italic_window,
-            } => apply_neon_flicker_style_stage(
-                &mut staged,
-                color,
-                *stability,
-                *dim_amount,
-                *italic_window,
-            ),
             NativeStyleStage::Rainbow { rotation_speed } => {
                 apply_rainbow_style_stage(&mut staged, *rotation_speed)
             }
@@ -751,37 +739,6 @@ fn apply_modulo_columns_style_stage(
     }
 }
 
-fn apply_neon_flicker_style_stage(
-    report: &mut PlayerRenderIrReport,
-    color: &str,
-    stability: f64,
-    dim_amount: f64,
-    italic_window: bool,
-) {
-    let width = report_width(report);
-    let height = report_height(report);
-    let clock = report.loop_t.unwrap_or(report.phase_t);
-    let flicker = (clock * 37.0).sin() * 0.5 + 0.5;
-    let active_strength = if flicker <= stability {
-        1.0
-    } else {
-        1.0 - dim_amount
-    };
-    let foreground = dimmed_rgba_label(color, active_strength);
-    for y in 0..height {
-        for x in 0..width {
-            set_report_cell_style(
-                report,
-                x,
-                y,
-                Some(foreground.as_str()),
-                None,
-                italic_window.then_some("italic"),
-            );
-        }
-    }
-}
-
 fn apply_rainbow_style_stage(report: &mut PlayerRenderIrReport, _rotation_speed: f64) {
     let width = report_width(report).max(1);
     let height = report_height(report);
@@ -1174,20 +1131,6 @@ fn rgba_label(r: u8, g: u8, b: u8, a: u8) -> String {
     format!("rgba({r},{g},{b},{a})")
 }
 
-fn dimmed_rgba_label(label: &str, strength: f64) -> String {
-    let Some((r, g, b, a)) = parse_rgba_label(label) else {
-        return label.to_string();
-    };
-    let strength = strength.clamp(0.0, 1.0);
-    format!(
-        "rgba({},{},{},{})",
-        (r as f64 * strength).round() as u8,
-        (g as f64 * strength).round() as u8,
-        (b as f64 * strength).round() as u8,
-        a
-    )
-}
-
 fn parse_rgba_label(label: &str) -> Option<(u8, u8, u8, u8)> {
     let inner = label.strip_prefix("rgba(")?.strip_suffix(')')?;
     let mut parts = inner.split(',').map(str::trim);
diff --git a/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs b/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
index 56fb70a..3e08192 100644
--- a/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
+++ b/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
@@ -147,6 +147,82 @@ fn test_fnc_cli_capture_cells_writes_procedural_recipe_metadata() {
     assert_eq!(run_schema, "v3.1.player.cellCapture.sqlite.1");
 }
 
+#[test]
+fn test_fnc_cli_lowers_neon_flicker_style_to_compositor_shader_layer_not_style_stage_json() {
+    let report = player_cli_json(
+        vec![
+            str_arg("render-backend"),
+            str_arg("--recipe"),
+            recipe_path("styles/style_neon_flicker.json"),
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
+            str_arg("dwell"),
+            str_arg("--phase-t"),
+            str_arg("0.35"),
+        ],
+        "render-backend native neon flicker style compositor shader layer player cli",
+    );
+
+    assert_eq!(report["backend"], "compositor");
+    assert_eq!(report["recipeId"], "debugStyleNeonFlicker");
+    assert_eq!(report["compositionMode"], "native");
+    assert_eq!(report["fallbackUsed"], false);
+    assert_eq!(report["nativeLoweringSucceeded"], true);
+    assert_eq!(report["compositionSpecSummary"]["shaderLayers"], 1);
+    assert_eq!(report["compositionSpecSummary"]["styleStages"], 0);
+    assert_eq!(
+        report["loweredEffectIds"],
+        serde_json::json!(["style.neonFlicker"])
+    );
+}
+
+#[test]
+fn test_fnc_cli_lowers_neon_flicker_modifier_style_to_compositor_shader_layer_json() {
+    let report = player_cli_json(
+        vec![
+            str_arg("render-backend"),
+            str_arg("--recipe"),
+            recipe_path("styles/style_neon_flicker_modifier.json"),
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
+            str_arg("dwell"),
+            str_arg("--phase-t"),
+            str_arg("0.35"),
+        ],
+        "render-backend native neon flicker modifier compositor shader layer player cli",
+    );
+
+    assert_eq!(report["recipeId"], "debugStyleNeonFlickerModifier");
+    assert_eq!(report["fallbackUsed"], false);
+    assert_eq!(report["nativeLoweringSucceeded"], true);
+    assert_eq!(report["compositionSpecSummary"]["shaderLayers"], 1);
+    assert_eq!(report["compositionSpecSummary"]["styleStages"], 0);
+    assert_eq!(
+        report["styledCells"][0]["foreground"],
+        "rgba(80,255,220,255)"
+    );
+    assert_eq!(
+        report["styledCells"][0]["modifiers"],
+        serde_json::json!(["italic"])
+    );
+}
+
 #[test]
 fn test_fnc_cli_lowers_focused_row_gradient_spatial_style_to_compositor_shader_layer_json() {
     let report = player_cli_json(
@@ -4729,20 +4805,6 @@ fn test_fnc_cli_renders_compositor_backend_native_residual_style_content_blocker
             "styleStages",
             1,
         ),
-        (
-            "styles/style_neon_flicker.json",
-            "debugStyleNeonFlicker",
-            "style.neonFlicker",
-            "styleStages",
-            1,
-        ),
-        (
-            "styles/style_neon_flicker_modifier.json",
-            "debugStyleNeonFlickerModifier",
-            "style.neonFlicker",
-            "styleStages",
-            1,
-        ),
         (
             "content/content_dissolve.json",
             "debugContentDissolve",
diff --git a/crates/tui-vfx-style/src/models/cls_neon_flicker_shader.rs b/crates/tui-vfx-style/src/models/cls_neon_flicker_shader.rs
index a8d5ba1..ccb576b 100644
--- a/crates/tui-vfx-style/src/models/cls_neon_flicker_shader.rs
+++ b/crates/tui-vfx-style/src/models/cls_neon_flicker_shader.rs
@@ -1,15 +1,16 @@
 // <FILE>tui-vfx-style/src/models/cls_neon_flicker_shader.rs</FILE> - <DESC>Spatial neon flicker with independent segments</DESC>
 // <VERS>VERSION: 1.6.0</VERS>
 // <WCTX>Adding screen coordinate context to shaders</WCTX>
-// <CLOG>Updated to use ShaderContext for screen-space effects</CLOG>
+// <CLOG>1.7.0: add optional base-color and italic-window styling so v3.1 neon style recipes can lower into compositor shader layers.
+// Updated to use ShaderContext for screen-space effects</CLOG>
 
-use crate::models::NoiseType;
+use crate::models::{ColorConfig, NoiseType};
 use crate::traits::{ShaderContext, StyleShader};
 use crate::utils::darken;
 use mixed_signals::envelopes::Impact;
 use mixed_signals::traits::Signal;
 use serde::{Deserialize, Serialize};
-use tui_vfx_types::{Color, Style};
+use tui_vfx_types::{Color, Modifiers, Style};
 
 fn default_speed() -> f32 {
     1.0
@@ -39,6 +40,17 @@ pub struct NeonFlickerShader {
     /// How much the flicker dims the color (0.0 - 1.0)
     #[config(default = 0.8)]
     pub dim_amount: f32,
+    /// Optional foreground color to install before flicker dimming.
+    ///
+    /// When absent, the shader preserves the incoming foreground color and only
+    /// applies the flicker treatment.
+    #[serde(default, skip_serializing_if = "Option::is_none")]
+    #[config(opaque)]
+    pub base_color: Option<ColorConfig>,
+    /// Whether the shader adds italic styling to the affected cells.
+    #[serde(default)]
+    #[config(default = false)]
+    pub italic_window: bool,
     /// Speed multiplier (lower = slower flicker)
     #[serde(default = "default_speed")]
     #[config(default = 1.0)]
@@ -82,6 +94,8 @@ impl Default for NeonFlickerShader {
             seed: 42,
             segment: SegmentMode::Row,
             dim_amount: 0.8,
+            base_color: None,
+            italic_window: false,
             speed: 1.0,
             flash_chance: 0.0,
             decay_rate: None,
@@ -149,9 +163,15 @@ impl StyleShader for NeonFlickerShader {
         let t = ctx.t as f32;
         let segment_id = self.segment_id(ctx.local_x, ctx.local_y);
         let (dim, is_flash) = self.flicker_state(segment_id, t);
+        let mut result = base;
+        if let Some(color) = self.base_color {
+            result.fg = Color::from(color);
+        }
+        if self.italic_window {
+            result.mods = result.mods.combine(Modifiers::italic());
+        }
 
         if dim > 0.0 {
-            let mut result = base;
             if is_flash {
                 // White flash - briefly go bright white
                 result.fg = Color::WHITE;
@@ -169,7 +189,7 @@ impl StyleShader for NeonFlickerShader {
             }
             result
         } else {
-            base
+            result
         }
     }
 }
@@ -192,6 +212,8 @@ mod tests {
             seed: 123,
             segment: SegmentMode::Row,
             dim_amount: 0.8,
+            base_color: None,
+            italic_window: false,
             speed: 1.0,
             flash_chance: 0.0,
             decay_rate: None,
@@ -222,6 +244,8 @@ mod tests {
             seed: 42,
             segment: SegmentMode::Cell,
             dim_amount: 0.8,
+            base_color: None,
+            italic_window: false,
             speed: 1.0,
             flash_chance: 0.0,
             decay_rate: None,
@@ -241,6 +265,25 @@ mod tests {
             "High stability should mean rare flickers"
         );
     }
+
+    #[test]
+    fn test_base_color_and_italic_window_are_applied_before_flicker() {
+        let shader = NeonFlickerShader {
+            stability: 1.0,
+            base_color: Some(ColorConfig::Rgb {
+                r: 255,
+                g: 50,
+                b: 150,
+            }),
+            italic_window: true,
+            ..NeonFlickerShader::default()
+        };
+        let ctx = ShaderContext::new(0, 0, 8, 4, 0, 0, 0.35, None, None);
+        let out = shader.style_at(&ctx, Style::default());
+
+        assert_eq!(out.fg, Color::rgb(255, 50, 150));
+        assert!(out.mods.italic);
+    }
 }
 
 // <FILE>tui-vfx-style/src/models/cls_neon_flicker_shader.rs</FILE> - <DESC>Spatial neon flicker with independent segments</DESC>
diff --git a/crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs b/crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs
index aec6471..3b4dc55 100644
--- a/crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs
+++ b/crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs
@@ -626,6 +626,8 @@ impl From<&crate::models::v3::VfxStochasticTextureShader> for SpatialShaderType
                 seed: *seed,
                 segment: (*segment).into(),
                 dim_amount: *dim_amount,
+                base_color: None,
+                italic_window: false,
                 speed: *speed,
                 flash_chance: *flash_chance,
                 decay_rate: *decay_rate,
diff --git a/crates/tui-vfx-style/src/models/v3/test_vfx_stochastic_texture_shader.rs b/crates/tui-vfx-style/src/models/v3/test_vfx_stochastic_texture_shader.rs
index 07ee5a9..758856a 100644
--- a/crates/tui-vfx-style/src/models/v3/test_vfx_stochastic_texture_shader.rs
+++ b/crates/tui-vfx-style/src/models/v3/test_vfx_stochastic_texture_shader.rs
@@ -16,6 +16,8 @@ fn converts_neon_flicker_into_v3_stochastic_texture_surface() {
         seed: 7,
         segment: crate::models::SegmentMode::Column,
         dim_amount: 0.9,
+        base_color: None,
+        italic_window: false,
         speed: 1.4,
         flash_chance: 0.1,
         decay_rate: Some(2.0),
diff --git a/crates/tui-vfx-style/tests/models/test_cls_neon_flicker_shader.rs b/crates/tui-vfx-style/tests/models/test_cls_neon_flicker_shader.rs
index b87c791..4784c6e 100644
--- a/crates/tui-vfx-style/tests/models/test_cls_neon_flicker_shader.rs
+++ b/crates/tui-vfx-style/tests/models/test_cls_neon_flicker_shader.rs
@@ -30,6 +30,8 @@ fn test_cell_segment_id_pairing() {
         seed: 7,
         segment: SegmentMode::Cell,
         dim_amount: 0.8,
+        base_color: None,
+        italic_window: false,
         speed: 1.0,
         flash_chance: 0.0,
         decay_rate: None,
```

## Claude output (raw)
```text
Reviewed the diff against the working tree. Found a notable mismatch between the submitted diff and what's actually on disk — flagging both up front.

## Blockers

**B1. Submitted diff is incomplete.** The working tree carries two more modified files that the diff omits, and they are load-bearing for this change:
- `crates/tui-vfx-style/src/models/v3/enum_vfx_stochastic_texture_behavior.rs` — adds `base_color: Option<ColorConfig>` and `italic_window: bool` to the `VfxStochasticTextureBehavior::NeonFlicker` variant.
- `crates/tui-vfx-style/src/models/v3/cls_vfx_stochastic_texture_shader.rs` — forwards both fields in `From<&NeonFlickerShader> for VfxStochasticTextureShader`.

Without those, the diff as written does not compile (the `cls_vfx_stochastic_texture_shader.rs` constructor would be missing two required fields once `NeonFlickerShader` gains them, and the v3 reverse-direction destructure has nothing to bind against). Re-emit the diff with all four files in `git status` matching the v3 work.

**B2. `From<&VfxStochasticTextureShader> for SpatialShaderType` hunk disagrees with the working tree, and the diff version is silently inert.** The diff text adds:
```
+ base_color: None,
+ italic_window: false,
```
but the actual file at `crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs:631-632` reads `base_color: *base_color, italic_window: *italic_window`, and the destructure adds the two new bindings — which is the correct behavior. The diff-text version would be a textbook "parse-and-inert schema field" violation: a V3 stochastic-texture recipe authored with `base_color`/`italic_window` would lose them when lowered into `SpatialShaderType::NeonFlicker`. Confirm the on-disk version is the intended ship state and re-export the diff so the review reflects it.

**B3. Metadata version not bumped on `cls_neon_flicker_shader.rs`.** Header is `<VERS>VERSION: 1.6.0</VERS>` and footer is `<VERS>END OF VERSION: 1.6.0</VERS>`, but the new `<CLOG>` entry leads with `1.7.0:`. Bump both VERS markers to 1.7.0. WCTX is also stale ("Adding screen coordinate context to shaders") — refresh to reflect the v3.1 style-recipe lowering context.

## Non-blocking

**N1. Visual behavior shift — uniform flicker → per-row independent flicker.** The deleted `apply_neon_flicker_style_stage` did one global `(clock * 37.0).sin() * 0.5 + 0.5` test against `stability`, dimming every cell uniformly. The new path is per-row noise-driven with a white-flash chance and Impact decay envelope. Intended, but anyone with snapshots anchored to the prior emulation will see a regression. Worth one line in CLOG/WCTX or release notes.

**N2. Brittle foreground assertion in the new modifier test.** `test_fnc_cli_lowers_neon_flicker_modifier_style_to_compositor_shader_layer_json` asserts `styledCells[0]["foreground"] == "rgba(80,255,220,255)"` against a recipe with `stability: 0.35` (i.e., row-0 flickers ~65% of sampled times). The test passes only because the noise sample at this specific `phase_t` happens to land on a non-flicker outcome for row 0. Any change to the noise routine, time-base, or default seed silently flips the assertion. Two safer options: set `stability: 1.0` in the fixture (deterministic no-flicker, fg always equals base color), or restrict the assertion to shader-layer count + region and cover cell rendering with a snapshot in the style crate.

**N3. Hardcoded `seed/segment/speed/flash_chance/decay_rate/noise_type` in `lower_style_neon_flicker`.** Matches the v3.1 schema's four-knob surface as scoped, but permanently pins v3.1 `style.neonFlicker` to `SegmentMode::Row` and `seed: 42`. Any future "cell" or "column" v3.1 request will need V3 stochastic-texture-shader, not v3.1 `style.neonFlicker`. Add a one-line rustdoc to the lowering function recording the deliberate constraint.

**N4. Asymmetric input resolution — `bool_input` vs `resolved_*_input`.** `stability`, `dimAmount`, `color` use the binding-aware `resolved_*_input` family; `italicWindow` uses `bool_input`. Today there is no `resolved_bool_input`, so this is the only choice, but the asymmetry means three of four `style.neonFlicker` inputs accept bindings/parameter refs and one does not — silently falling back to default if a binding is supplied. Either add `resolved_bool_input` and route through it, or have `unsupported_style_stage_reason` reject binding-shaped values on `italicWindow` so the gap is loud rather than silent. (Per your "loopback is required for bindings" rule — strict-contracts mode should reject un-previewable inputs by class.)

**N5. Missing reverse-direction round-trip test.** `test_vfx_stochastic_texture_shader.rs:converts_neon_flicker_into_v3_stochastic_texture_surface` only exercises `NeonFlickerShader → VfxStochasticTextureShader` with `base_color: None, italic_window: false` — i.e., default values. There is no test that drives a non-default `base_color`/`italic_window` through the reverse direction (`VfxStochasticTextureShader → SpatialShaderType::NeonFlicker`). That's exactly the path implicated in B2; a single round-trip assertion with `Some(Rgb {...})` + `italic_window: true` would have caught the wrong hunk and is cheap to add.
```

## Follow-up actions after Claude review

Claude reviewed an initial stale diff while the working tree was still moving. I treated the review as a blocker checklist and addressed the substantive points in the final on-disk state:

- Re-exported/kept the load-bearing V3 stochastic texture files in the committed diff: `enum_vfx_stochastic_texture_behavior.rs` and `cls_vfx_stochastic_texture_shader.rs` now carry `base_color` and `italic_window`.
- Confirmed and preserved the correct reverse lowering: `VfxStochasticTextureShader -> SpatialShaderType::NeonFlicker` now carries `base_color` and `italic_window` instead of dropping them.
- Bumped `cls_neon_flicker_shader.rs` version markers to 1.7.0 and refreshed WCTX.
- Removed the brittle CLI foreground assertion for the stochastic modifier recipe while keeping shader-layer/no-style-stage and italic modifier proof; base-color rendering remains covered by the deterministic style-shader unit test.
- Added a reverse-direction V3 stochastic texture regression with non-default `base_color` and `italic_window`.
- Added a loud rejection for non-literal `italicWindow` sources because there is no resolved bool binding path in this adapter.
- Documented that v3.1 `style.neonFlicker` intentionally exposes only the legacy four-knob surface; seed/segment/speed/flash/decay live on the V3 stochastic-texture shader surface.

Post-fix verification: cargo check passed, targeted tui-vfx-style and tui-vfx-player-cli nextest runs passed, strict render-backend showed fallbackUsed=false/nativeLoweringSucceeded=true/shaderLayers=1/styleStages=0, clippy --tests -D warnings passed, fmt --check passed, and git diff --check passed.
