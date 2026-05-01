# Claude review: style.moduloColumns compositor lowering

Started: 2026-05-01T02:48:27Z

```diff
diff --git a/crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs b/crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs
index b589116..628d6bd 100644
--- a/crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs
+++ b/crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs
@@ -1,7 +1,8 @@
 // <FILE>crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs</FILE> - <DESC>Lower player render requests into compositor CompositionSpec modes</DESC>
 // <VERS>VERSION: 0.41.0</VERS>
 // <WCTX>Native compositor lowering: map bounded v3.1 recipe graph effects into native CompositionSpec and source-stage content/style/filter work with honest fallback diagnostics.</WCTX>
-// <CLOG>0.45.0: MINOR — lower style.italicWindow into compositor ModifierWindowShader layers and remove backend style-stage emulation.
+// <CLOG>0.46.0: MINOR — lower style.moduloColumns into compositor LinearGradientShader layers scoped by StyleRegion::Modulo and remove backend style-stage emulation.
+// 0.45.0: MINOR — lower style.italicWindow into compositor ModifierWindowShader layers and remove backend style-stage emulation.
 // 0.44.0: MINOR — lower style.pulse into compositor PulseWaveShader layers and remove backend style-stage emulation.
 // 0.43.0: MINOR — lower style.glitch into compositor GlitchLinesShader layers and remove backend style-stage emulation.
 // 0.42.0: MINOR — lower style.neonFlicker into compositor NeonFlickerShader layers and remove backend style-stage emulation.
@@ -59,9 +60,9 @@ use tui_vfx_style::models::{
     FocusFieldShader, FocusFieldShape, FocusedRowGradientShader, GlistenApplyTo, GlistenBandShader,
     GlistenDirection, GlitchLinesShader, Gradient, HighlighterApplyTo, HighlighterDirection,
     HighlighterMode, HighlighterRowMask, HighlighterShader, LinearGradientApplyTo,
-    LinearGradientShader, ModifierWindowShader, NeonFlickerShader, PulseWaveShader, RadarShader,
-    RevealWipeShader, SegmentMode, SpatialShaderType, StyleRegion, TextContrast, WaveDirection,
-    WayfindingNode, WayfindingNodeApplyTo, WayfindingNodeShader,
+    LinearGradientShader, ModifierWindowShader, ModuloAxis, NeonFlickerShader, PulseWaveShader,
+    RadarShader, RevealWipeShader, SegmentMode, SpatialShaderType, StyleRegion, TextContrast,
+    WaveDirection, WayfindingNode, WayfindingNodeApplyTo, WayfindingNodeShader,
 };
 
 const SUPPORTED_WIPE_DIRECTIONS: &[&str] = &[
@@ -170,13 +171,6 @@ pub enum NativeContentStage {
 /// Native style transform stage owned by the compositor backend adapter.
 #[derive(Clone, Debug, PartialEq)]
 pub enum NativeStyleStage {
-    /// Apply foreground/background colors to modulo-selected columns.
-    ModuloColumns {
-        modulus: usize,
-        remainder: usize,
-        foreground: String,
-        background: String,
-    },
     /// Apply V2-compatible rainbow foreground cycling.
     Rainbow { rotation_speed: f64 },
     /// Apply player-compatible color fade styling to existing foreground/background channels.
@@ -547,7 +541,7 @@ fn lower_node_into_spec(
         "style.fadeIn" | "style.fadeOut" => lower_style_fade(node, spec, request, warnings),
         "style.pulse" => lower_style_pulse(node, spec, request, warnings),
         "style.italicWindow" => lower_style_italic_window(node, spec, request, warnings),
-        "style.moduloColumns" => lower_style_modulo_columns(node, style_stages, request, warnings),
+        "style.moduloColumns" => lower_style_modulo_columns(node, spec, request, warnings),
         "style.neonFlicker" => lower_style_neon_flicker(node, spec, request, warnings),
         "style.rainbow" => lower_style_rainbow(node, style_stages, request, warnings),
         "style.glitch" => lower_style_glitch(node, spec, request, warnings),
@@ -1759,7 +1753,7 @@ fn eased_style_progress(node: &NodeSpec, request: &PlayerRenderBackendRequest) -
 
 fn lower_style_modulo_columns(
     node: &NodeSpec,
-    style_stages: &mut Vec<NativeStyleStage>,
+    spec: &mut CompositionSpec,
     request: &PlayerRenderBackendRequest,
     warnings: Vec<PlayerRenderBackendDiagnostic>,
 ) -> NodeLoweringOutcome {
@@ -1778,23 +1772,52 @@ fn lower_style_modulo_columns(
             reason: "Effect `style.moduloColumns` requires a modulo-columns scope for compositor-native style-stage lowering.".to_string(),
         };
     };
-    style_stages.push(NativeStyleStage::ModuloColumns {
-        modulus,
-        remainder,
-        foreground: color_label_from_config(
-            color_input(node, request, "foreground").unwrap_or(ColorConfig::Cyan),
-        ),
-        background: color_label_from_config(color_input(node, request, "background").unwrap_or(
-            ColorConfig::Rgb {
-                r: 15,
-                g: 40,
-                b: 55,
-            },
-        )),
-    });
+    let region = StyleRegion::Modulo {
+        axis: ModuloAxis::Vertical,
+        modulus: BindableU16::from(modulus as u16),
+        remainder: BindableU16::from(remainder as u16),
+    };
+    let foreground = resolved_color_input(node, request, "foreground").unwrap_or(ColorConfig::Cyan);
+    let background =
+        resolved_color_input(node, request, "background").unwrap_or(ColorConfig::Rgb {
+            r: 15,
+            g: 40,
+            b: 55,
+        });
+
+    spec.shader_layers.push(solid_color_shader_layer(
+        foreground,
+        LinearGradientApplyTo::Foreground,
+        region.clone(),
+    ));
+    spec.shader_layers.push(solid_color_shader_layer(
+        background,
+        LinearGradientApplyTo::Background,
+        region,
+    ));
     NodeLoweringOutcome::Lowered { warnings }
 }
 
+fn solid_color_shader_layer(
+    color: ColorConfig,
+    apply_to: LinearGradientApplyTo,
+    region: StyleRegion,
+) -> ShaderLayerSpec {
+    let color = tui_vfx_types::Color::from(color);
+    ShaderLayerSpec {
+        shader: SpatialShaderType::LinearGradient(LinearGradientShader {
+            gradient: Gradient {
+                stops: vec![(0.0, color), (1.0, color)],
+                space: ColorSpace::Rgb,
+            },
+            angle_deg: 0.0,
+            apply_to,
+            intensity: 1.0,
+        }),
+        region,
+    }
+}
+
 fn lower_style_neon_flicker(
     node: &NodeSpec,
     spec: &mut CompositionSpec,
diff --git a/crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs b/crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs
index 2441acf..17e8c74 100644
--- a/crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs
+++ b/crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs
@@ -261,18 +261,6 @@ fn scene_ir_with_native_content_stages(
     }
     for stage in &lowered_spec.style_stages {
         match stage {
-            NativeStyleStage::ModuloColumns {
-                modulus,
-                remainder,
-                foreground,
-                background,
-            } => apply_modulo_columns_style_stage(
-                &mut staged,
-                *modulus,
-                *remainder,
-                foreground,
-                background,
-            ),
             NativeStyleStage::Rainbow { rotation_speed } => {
                 apply_rainbow_style_stage(&mut staged, *rotation_speed)
             }
@@ -704,25 +692,6 @@ fn apply_slide_shift_content_stage(
     sync_styled_cells_to_rows(report);
 }
 
-fn apply_modulo_columns_style_stage(
-    report: &mut PlayerRenderIrReport,
-    modulus: usize,
-    remainder: usize,
-    foreground: &str,
-    background: &str,
-) {
-    let width = report_width(report);
-    let height = report_height(report);
-    let modulus = modulus.max(1);
-    for y in 0..height {
-        for x in 0..width {
-            if x % modulus == remainder {
-                set_report_cell_style(report, x, y, Some(foreground), Some(background), None);
-            }
-        }
-    }
-}
-
 fn apply_rainbow_style_stage(report: &mut PlayerRenderIrReport, _rotation_speed: f64) {
     let width = report_width(report).max(1);
     let height = report_height(report);
diff --git a/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs b/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
index 35ff883..1953d48 100644
--- a/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
+++ b/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
@@ -4879,6 +4879,12 @@ fn test_fnc_cli_renders_compositor_backend_native_migrated_filter_parity_recipes
             report["compositionSpecSummary"][summary_key], expected_stage_count,
             "{recipe}"
         );
+        if effect_id == "style.moduloColumns" {
+            assert_eq!(
+                report["compositionSpecSummary"]["styleStages"], 0,
+                "style.moduloColumns must not leave effect logic in backend style stages: {recipe}"
+            );
+        }
         assert!(
             report["loweredEffectIds"]
                 .as_array()
@@ -5021,15 +5027,15 @@ fn test_fnc_cli_renders_compositor_backend_native_residual_style_content_blocker
             "styles/style_modulo_columns_period.json",
             "debugStyleModuloColumnsPeriod",
             "style.moduloColumns",
-            "styleStages",
-            1,
+            "shaderLayers",
+            2,
         ),
         (
             "styles/style_modulo_vertical_every_fourth_column_offset.json",
             "debugStyleModuloVerticalEveryFourthColumnOffset",
             "style.moduloColumns",
-            "styleStages",
-            1,
+            "shaderLayers",
+            2,
         ),
         (
             "content/content_dissolve.json",
@@ -5108,6 +5114,12 @@ fn test_fnc_cli_renders_compositor_backend_native_residual_style_content_blocker
             report["compositionSpecSummary"][summary_key], expected_stage_count,
             "{recipe}"
         );
+        if effect_id == "style.moduloColumns" {
+            assert_eq!(
+                report["compositionSpecSummary"]["styleStages"], 0,
+                "style.moduloColumns must not leave effect logic in backend style stages: {recipe}"
+            );
+        }
         assert!(
             report["loweredEffectIds"]
                 .as_array()
```

## Claude response

## Claude response (second attempt with inline diff)


## Claude response (second attempt with inline diff)

The diff is empty — nothing was attached after `DIFF:`. Paste the patch (or give me a commit/PR ref or file paths to read) and I'll review against the stated goal: moving `style.moduloColumns` out of `NativeStyleStage` into compositor IR as two `LinearGradientShader` layers scoped by `StyleRegion::Modulo`, no fallbacks.

To make the review useful, I'll specifically look for:
- **Layer boundary leaks** — any modulo-specific logic remaining in `NativeStyleStage` or player/backend after the move; ratatui types reaching the compositor (Intention 1).
- **IR shape** — whether the two gradient layers actually carry solid-color stops (degenerate gradient) vs. introducing a new shader variant; FG/BG ordering and z-order determinism.
- **Region scoping** — `StyleRegion::Modulo` semantics: column predicate evaluation point, interaction with other regions, idempotence under re-render.
- **Determinism parity** — pixel-for-pixel match against the pre-migration path (golden/snapshot tests covering the modulo case); no float drift from gradient interpolation when stops are equal.
- **Bindings/loopback** — if modulo params are now bindable, every `requires_bindings` entry must have effective loopback (per memory).
- **No fallbacks** — confirm there's no "if no compositor, do it the old way" branch left behind, and the legacy `NativeStyleStage` modulo path is fully removed (not just `#[allow(dead_code)]`).
- **Schema/wire** — `Vfx*` prefix on any new public types; rustdoc updated on touched public items; capability manifest regen if needed.
- **Warnings/clippy** — clean build, no per-site `#[allow]`.

Send the diff and I'll flag hard blockers and correctness risks only.

## Claude response (third attempt: repository path review)


## Claude response (stdin diff attempt)

## Review — `style.moduloColumns` → compositor IR migration

**Hard blockers / correctness risks:**

1. **Layer-vs-stage ordering regression with un-migrated style effects.** Compositor `shader_layers` evaluate before backend `style_stages`. If a recipe places `moduloColumns` *after* `style.rainbow` (still on `NativeStyleStage`), pre-migration order was `rainbow → modulo` (modulo wins on selected cells). Post-migration it becomes `modulo (compositor) → rainbow (stage)` — rainbow now overwrites the modulo foreground everywhere, so selected cells lose their `cyan` fg and only the bg layer remains visible. The same hazard applies to any still-staged successor (`Rainbow`, `ColorFade`, plus any content-stage that runs after compositor). This is the migration's load-bearing parity risk; the test in `test_fnc_render_recipe_cli.rs:5021` only counts spec entries — no rendered-output golden compares the staged path's output to the new path. Add a parity snapshot before declaring done.

2. **`resolved_color_input` vs old `color_input` is a quiet behavior change.** Old path read raw `Value::Color`; bound color inputs would have fallen through to `unwrap_or` defaults. New path resolves bindings, so a recipe with `{"binding": "..."}` on `foreground`/`background` now picks up the bound value where it previously rendered the default. Likely the intended improvement, but it should be called out — and any baseline that relied on the old fall-through behavior will diverge.

**Maintainability:**

3. **Solid color encoded as a degenerate 2-stop gradient.** `solid_color_shader_layer` constructs `LinearGradientShader { stops: [(0.0, c), (1.0, c)], angle_deg: 0.0, intensity: 1.0 }`. Nothing in the helper or its callsite explains *why* that shape — a future reader will assume an animated/positional gradient was intended and the second stop got copy-paste-broken. Either drop a one-line `// degenerate gradient = solid fill; no SolidColor variant in SpatialShaderType` rustdoc on the helper, or (cleaner) introduce a `SpatialShaderType::SolidColor` variant so the IR carries intent. The helper is now reachable from a real lowering site so the rustdoc obligation kicks in (per project convention).

4. **`modulus as u16` truncation.** `modulus: usize` casts to `u16` silently. If schema validation upstream caps it ≤ `u16::MAX`, fine — but worth a one-line `debug_assert!` or comment pointing at the validator, since a stray oversized value would now silently produce a wrong stripe pattern instead of a clear error.

**Non-issues verified:**
- `ModuloAxis::Vertical` matches the old `x % modulus == remainder` (vertical = column axis, per `cls_style_region.rs:44`).
- Color baking parity: old `color_label_from_config` already routed through `Color::from(ColorConfig)` and emitted `rgba(...)` strings, so named-color → RGB happened in both paths. No theme-color regression.
- Layer-pair ordering (fg layer then bg layer with disjoint `apply_to`) is independent — order between the two pushes doesn't matter.
## Hard blockers / correctness risks

**1. ModuloAxis::Vertical semantics — parity-critical**
Old code selected columns: `if x % modulus == remainder`. New code uses `ModuloAxis::Vertical`. If `Vertical` means "stripes oriented vertically" (i.e., modulus over **x** → columns), parity holds. If it means "modulus computed on the vertical/y axis" (→ rows), this silently inverts orientation. The two test fixtures named `..._period` and `..._vertical_every_fourth_column_offset` only assert layer **counts**, not pixel parity — they will not catch an axis flip. Verify `StyleRegion::Modulo` evaluator and add a render-equivalence assertion (or freeze a golden frame) before landing.

**2. Lost `modulus.max(1)` guard**
Backend stage clamped `modulus` to ≥1 to avoid div-by-zero. New path passes raw `u16` (including 0) into `BindableU16`. Confirm the compositor's `StyleRegion::Modulo` rejects/normalizes `modulus == 0`; if not, you've moved a defensive clamp into a panic path or no-op-by-luck. Recipe-schema validation alone isn't enough unless it's strict-contracts gated.

**3. `intensity: 1.0` ≠ unconditional overwrite (potentially)**
Old path called `set_report_cell_style(..., Some(fg), Some(bg), None)` which **replaces** the channel. `LinearGradientShader { intensity: 1.0 }` typically **blends** at full weight — semantically different if any preceding layer also wrote to the same cell channel under that region. Confirm the LinearGradient evaluator's intensity=1.0 contract is "replace, not blend" for solid stops, or you'll see drift when modulo composes with other style stages.

**4. u16 rejection is a new failure class**
Old `usize` accepted any modulus/remainder; new path returns `Unsupported` above `u16::MAX`. Reasonable for grid coordinates, but: (a) confirm the recipe schema bounds modulus/remainder ≤ 65535 (otherwise validation passes recipes the lowerer rejects — a determinism break), and (b) the diagnostic message is fine but no test exercises the boundary.

**5. BindableU16 + loopback contract**
`modulus`/`remainder` are now `BindableU16`, so bindings can target them. Per repo rule, every `requires_bindings` entry must yield a loopback. If `style.moduloColumns` doesn't declare or default-lift loopbacks for these two scope fields, strict-contracts will reject any animated recipe. Audit the effect's binding manifest alongside this change.

**6. `color_input` → `resolved_color_input` swap**
Diff silently changes the helper used to read `foreground`/`background`. If `resolved_color_input` honors bindings/themes that `color_input` did not (or vice versa), default-color fixtures may pass while bound-color fixtures regress. Worth one paragraph in the CLOG and a binding-color test.

**7. Test coverage is count-only**
The CLI test suite now asserts `shaderLayers == 2` and `styleStages == 0`. That proves routing, not behavior. Without a frame-level parity test against the pre-cutover output (or at minimum, an IR snapshot of the produced `StyleRegion::Modulo` with the right axis/modulus/remainder values), risks 1–3 above are undetectable in CI.

Suggest: gate the merge on (a) an axis-semantics assertion, (b) a render-parity golden for at least one of the two existing fixtures, and (c) explicit verification of intensity=1.0 overwrite semantics for `LinearGradientShader` with identical stops.

Claude exit: 0 at 2026-05-01T02:51:40Z
## Review — `style.moduloColumns` migration to compositor-owned IR

**Parity verified end-to-end.** The two-layer lowering produces the same observable effect as the removed `NativeStyleStage::ModuloColumns`:
- `StyleRegion::Modulo { axis: Vertical, modulus, remainder }` → `should_style` evaluates `x % modulus == remainder` (`fnc_style_region_should_style.rs:94-98`), matching the original predicate.
- `bounding_rect` for `Modulo` returns `None` → renderer iterates the full widget area, matching the original `0..height × 0..width` scan.
- Two-stop equal-color `LinearGradientShader` at `intensity: 1.0` → `blend_colors` short-circuits to `c2` at `t >= 1.0` (`fnc_blend_colors.rs:15-17`) → channel fully replaced, matching `set_report_cell_style`.
- Variant removal is clean: zero orphan references to `NativeStyleStage::ModuloColumns` or `apply_modulo_columns_style_stage`.

### Issues — actionable

**1. Dead-code guard in the wrong test** — `test_fnc_render_recipe_cli.rs:4882` (the `migrated_filter_parity_recipes_json` test). The loop's tuple table iterates `filter.*` effects only; the new `if effect_id == "style.moduloColumns"` can never fire here. The same guard at line 5111 in the residual-style test is the one that actually exercises moduloColumns recipes. Drop the duplicate at 4882 — keeping it implies coverage that doesn't exist and will mislead future readers.

**2. Silent `usize → u16` truncation at the lowering site** — `fnc_lower_recipe_graph_to_composition_spec.rs:1775-1776`: `BindableU16::from(modulus as u16)` and the remainder cast. `modulo_columns_scope` (line 3192) only enforces `*modulus > 0`, not a `u16::MAX` ceiling. Original path carried `usize` end-to-end with `.max(1)`. Realistically out of range, but it's a quiet contract narrowing — either bound-check at the lowering site and emit a diagnostic, or tighten the upstream scope parser. Don't leave it as a silent wrap.

### Maintainability nit (not blocking)

**3. `solid_color_shader_layer` is file-local.** It's the third+ migration (italic-window, pulse, glitch, neon-flicker, focused-row, now modulo) using the "two-stop equal-color LinearGradient stands in for force-paint" idiom. If the next seam reaches for the same pattern, lift this helper to a shared compositor-lowering util before drift sets in (stop count, intensity default, color space). Not urgent given the helper currently has only one caller.

### Other notes
- LOC-neutral net change in the lowering file; backend file shrinks by ~31 lines (style stage emulation removed). No size-budget concerns.
- Doc edit (`dynamic-light-shadow-primitive-spec.md`) is unrelated to this seam — metadata cross-reference only. Fine in the same commit but separable.
## Review — `style.moduloColumns` migration to compositor IR

Structurally consistent with the in-flight migration pattern (italicWindow, pulse, glitch, neonFlicker, focused-row). Two real risks worth flagging.

### Hard blockers / correctness risks

**1. `modulus = 0` semantic divergence (real, narrow).**
- Old (`fnc_render_compositor_backend.rs:707`): `let modulus = modulus.max(1)` → modulus 0 silently becomes 1, *entire grid* gets the foreground/background.
- New (`fnc_style_region_should_style.rs:88-93`): if `modulus == 0` returns `false` → *no cells* styled.
Parity flips. If the recipe validator rejects modulus 0 upstream this is moot — but the lowering function does not validate modulus (only checks the scope's existence at line 1772-1774). Either add a `modulus == 0` rejection at lower time or document that the validator owns this. Not a blocker if validator coverage is confirmed.

**2. fg/bg now carried as literal `Color`, not string label.**
- Old path stamped `Some("Cyan")` strings into `PlayerRenderIrReport` cells.
- New path stamps `tui_vfx_types::Color::from(ColorConfig::Cyan)` (resolved RGB) via `LinearGradientShader`.
Terminal output should be identical, but any downstream stage that pattern-matches on label strings (or any golden snapshot serializing cell style as `"fg":"Cyan"` vs `"fg":{"r":...}`) will diverge. The CLI test you updated only counts layers; no rendered-output parity assertion. Same gap exists for the prior four migrations, so this is consistency rather than regression — but it's not *verified* parity.

### Region semantics ✓
`StyleRegion::Modulo { axis: ModuloAxis::Vertical, modulus, remainder }` evaluates `x % modulus == remainder` (`fnc_style_region_should_style.rs:94-97`), matching old `if x % modulus == remainder`. Correct axis choice — `Vertical` = column selection.

### Layer composition ✓
Two layers, fg-only + bg-only, intensity 1.0 with two-identical-stop gradient at angle 0 → `gradient.sample(t)` returns the constant color for any `t` (`cls_linear_gradient_shader.rs:169`); `apply_to: Foreground/Background` fully replaces target channel only (lines 173-184). Order between the two pushed layers doesn't matter (disjoint channels). Order vs *other* shader_layers in multi-effect recipes is unverified by these tests.

### Tests
- `shaderLayers == 2` + new explicit `styleStages == 0` assertion is the right structural gate. ✓
- No render-output parity check across modulus/remainder permutations or against the legacy backend output. Acceptable only if you trust the four prior migrations' pattern.
- Suggest adding a single parity case where modulus is set in a multi-effect recipe (e.g., moduloColumns + rainbow) to catch interleave regressions before V3 cutover.

### Minor
- `lower_style_modulo_columns` signature drops `style_stages: &mut Vec<NativeStyleStage>` — good, one less foot-gun.
- CLOG/VERS bump to 0.46.0 MINOR is correct (additive lowering path; backend emulator removal is internal).
- `solid_color_shader_layer` is a clean, reusable helper; if a sixth migration needs the same idiom, lift it earlier in the file with the other shader-construction helpers.

**Verdict:** Ship-ready *if* (a) modulus=0 is validator-rejected upstream, and (b) the team has accepted the no-render-parity-test risk for this migration cohort.

## Follow-up actions after review

- Added explicit u16 bounds rejection before converting modulo scope values into `StyleRegion::Modulo`, avoiding silent truncation.
- Kept the existing `modulo_columns_scope` `modulus > 0` gate, so zero-modulus inputs remain unsupported rather than reaching compositor evaluation.
- Added rendered-frame assertions for both modulo fixtures: selected `x % modulus == 1` columns must receive the expected foreground/background colors, and non-selected columns must not receive that color pair.
- Documented why `solid_color_shader_layer` intentionally uses a degenerate two-stop `LinearGradientShader`: current `SpatialShaderType` has no dedicated solid-color shader.
- Verified `LinearGradientShader` intensity `1.0` is the existing replacement-strength path for target channels; strict renders show exact expected `rgba(...)` foreground/background values.
- Noted ordering risk while `Rainbow`, `ColorFade`, and `ColorShift` still remain backend style stages; this task is continuing immediately to remove those remaining stages so mixed style recipes do not depend on cross-layer ordering.
