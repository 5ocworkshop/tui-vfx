<!-- <FILE>CHANGELOG.md</FILE> - <DESC>Release history for tui-vfx</DESC> -->
<!-- <VERS>VERSION: 1.5.0</VERS> -->
<!-- <WCTX>Color-inert glyph detection for shadow grading replacement</WCTX> -->
<!-- <CLOG>Document color-inert glyph detection and replacement_char field</CLOG> -->

# Changelog

All notable changes to this project will be documented in this file.

This project follows [Semantic Versioning](https://semver.org/).

## 0.2.4 — 2026-03-13

### Added
- **tui-vfx-shadow:** Added `ShadowCompositeMode` enum with `GlyphOverlay` (default, backward-compatible) and `GradeUnderlying` (destination-preserving color grading) variants.
- **tui-vfx-shadow:** Added `ShadowGradeConfig` struct with per-channel dim, desaturate, and tint strength controls for fine-tuned grade-underlying shadows.
- **tui-vfx-shadow:** Added `ShadowConfig::with_composite_mode()`, `.with_grade()`, and `.with_dramatic_grade()` builder methods.
- **tui-vfx-compositor:** Added `fnc_grade_shadow_cell` — implements the grade-underlying algorithm (desaturate → dim → tint) that preserves destination glyphs and modifiers while applying color grading scaled by shadow coverage.
- **tui-vfx-compositor:** Pipeline branches on `ShadowCompositeMode`: `GlyphOverlay` uses the existing `blend_shadow_cell`, `GradeUnderlying` uses the new `grade_shadow_cell`.
- **tui-vfx (prelude):** Re-exported `ShadowCompositeMode` and `ShadowGradeConfig` from the prelude.
- **tui-vfx-types:** Added `color_inert` module with `is_color_inert_glyph()` — detects emoji, PUA/nerd-font icons, variation selectors, and ZWJ that ignore ANSI fg color in terminal emulators.
- **tui-vfx-shadow:** Added `ShadowGradeConfig::replacement_char` field (`Option<char>`) — when set, color-inert glyphs are replaced with the given character during grade-underlying compositing. `Default` is `None` (backward compatible); `dramatic()` sets `Some('·')`.
- **tui-vfx-compositor:** `grade_shadow_cell` now conditionally replaces color-inert glyphs with the configured placeholder, preventing bright bitmap artifacts in dimmed shadow regions.

### Fixed
- **tui-vfx-compositor:** Fixed `test_shadow_extends_render_area` assertion that checked `bg` instead of `fg` for half-block soft-edge shadow cells (shadow color is carried in `fg` for `RIGHT_HALF` characters).

### Changed
- **docs:** Updated `HOWTO_SHADOWS.md` with Shadow Compositing Modes section, dramatic example, and custom grade parameters example.
- **docs:** Updated `API_HAND.md` and `api_docs.toml` to document new types and builder methods.
- **xtask/docs:** Updated `api_metadata.rs` and `gen_api.rs` so generated `API.md` includes `ShadowCompositeMode` and `ShadowGradeConfig` sections and the updated `ShadowConfig` struct.

## 0.2.3 — 2026-03-13

### Added
- **tui-vfx-shadow:** Added `ShadowStyle::MediumShade`, a textured full-cell shadow style that renders with the Unicode medium shade character (`▒`).
- **tui-vfx-shadow:** Added `MediumShadeRenderer` and wired it through `render_shadow(...)` style dispatch.

### Changed
- **tui-vfx-shadow:** Updated crate docs and renderer exports to include the new medium-shade style and renderer.
- **xtask/docs:** Included `MediumShade` in extracted shadow effect metadata so generated docs/schema output reflects the new style.

## 0.2.2 — 2026-03-08

### Fixed
- **tui-vfx-style:** All positional-sweep shaders (`GlistenBandShader`, `BorderSweepShader`, `RadarShader`, `ReflectShader`, `OrbitShader`) multiplied `t * self.speed` internally, but the compositor clamps `t` (via `shader_t`) to `[0, 1]`. With `speed < 1.0`, the sweep was truncated to `speed%` of the full range (e.g. `speed: 0.3` → band only reached 30% of widget width). Fix: removed `self.speed` from the positional computation in all 5 shaders. Sweep rate is now controlled exclusively by the caller via `loop_t`, which is the correct architectural boundary — the compositor owns timing, shaders own spatial mapping. The `speed` field remains on each struct for serde compatibility but is no longer used in rendering. Upstream consumers (e.g. `normalise_shader_timing` in gooey-ratatui) that worked around this bug can now be simplified.

## 0.2.1 — 2026-03-01

### Fixed
- **tui-vfx-shadow:** Normalized HalfBlock right-edge shadow to use `fg=shadow, bg=surface` convention, consistent with all other edges. Previously, the right-edge first column and corner used `fg=surface, bg=shadow` (inverted), which caused the compositor's transparent-portion resolution to land in `fg` instead of `bg`. Downstream `apply_vfx_cell_to_rat` would then preserve the destination cell's existing `fg` (often `Color::Reset` = white) rather than the intended background, producing visible white artifacts on right-edge shadows. Replaced `LEFT_THREE_QUARTERS` (▊) with `RIGHT_HALF` (▐) for the right-edge soft gradient.

## 0.2.0 — 2026-02-18

Initial public release.

### Added
- Compositing pipeline with configurable effect chains via `CompositionOptions`
- 10 mask/transition types: Dissolve, Wipe, Iris, Blinds, Checkers, Diamond, Cellular, Radial, PathReveal, NoiseDither
- 8 filter types: Dim, Brighten, Tint, Invert, Vignette, PatternFill, Greyscale, RigidShake
- 6 sampler types: Ripple, SineWave, CRT, CRTJitter, FaultLine, Shredder
- 14 style shader types for procedural color and style generation
- 12 content transformer types: Typewriter, Scramble, Morph, and more
- Shadow rendering with Braille, HalfBlock, and Solid styles
- Framework-agnostic `Grid` trait for integration with any terminal rendering backend
- Data-driven configuration via serde-compatible effect specs (JSON/TOML)
- `ConfigSchema` derive macro for runtime introspection of effect parameters
- `xtask` documentation pipeline: auto-generated API reference, capabilities inventory, effect schemas, and AI context prompt
- Recipe validation tooling for JSON effect configurations

<!-- <FILE>CHANGELOG.md</FILE> - <DESC>Release history for tui-vfx</DESC> -->
<!-- <VERS>END OF VERSION: 1.5.0</VERS> -->
