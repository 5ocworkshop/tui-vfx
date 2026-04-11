<!-- <FILE>CHANGELOG.md</FILE> - <DESC>Release history for tui-vfx</DESC> -->
<!-- <VERS>VERSION: 1.9.0</VERS> -->
<!-- <WCTX>feat/content-ergonomics: document 0.3.0 ergonomic additions to tui-vfx-content</WCTX> -->
<!-- <CLOG>Add 0.3.0 section covering ContentEffect::apply and TypewriterCursor presets</CLOG> -->

# Changelog

All notable changes to this project will be documented in this file.

This project follows [Semantic Versioning](https://semver.org/).

## 0.3.0 — 2026-04-11

### Added
- **tui-vfx-content:** Added `ContentEffect::apply(target, progress) -> String` — a one-call ergonomic entry point that hides the `get_transformer` dispatcher, the `SignalContext`, and the `Cow` unwrap for the common static-progress case. Collapses three lines of boilerplate into one.
- **tui-vfx-content:** Added `ContentEffect::apply_to_borrowed(target, progress) -> Cow<'_, str>` preserving the zero-allocation fast path that the underlying `TextTransformer::transform` method provides. Use this when you care about avoiding `Cow::into_owned` in the no-op case (e.g. Typewriter at progress `1.0` returning the full target as a borrowed slice).
- **tui-vfx-content:** Added `ContentEffect::apply_with_context(target, progress, &ctx) -> Cow<'_, str>` — the advanced-use entry point for signal-driven pacing with a caller-supplied `SignalContext`.
- **tui-vfx-content:** Added `TypewriterCursor::simple(glyph)` and the convenience presets `block()` (█), `underscore()` (_), `pipe()` (|), and `caret()` (▌). Each preset is a thin wrapper over `Self::default()` that swaps the character field, so the rest of the `SignalOrFloat` shape is preserved exactly.
- **tui-vfx-content:** Crate-level rustdoc gained Quick start, Static vs signal-driven parameters, and Cursor presets sections explaining when to reach for `SignalOrFloat::Static(n)` vs the dynamic variants.

### Changed
- **workspace:** Bumped workspace version from `0.2.6` to `0.3.0` to reflect the additive `tui-vfx-content` public API additions. All other crates are republished at the new version unchanged.

### Notes
- Every change in this release is **additive** to the public API. The existing `get_transformer` + `TextTransformer::transform` + `SignalContext` path is unchanged and remains the canonical advanced API. No call sites need to migrate.

## 0.2.6 — 2026-03-17

### Added
- **tui-vfx-compositor:** Added `CharsetNoise` filter — non-converging time-varying character replacement operating at the compositor level. Replaces cell characters from a position-aware charset gradient that changes over time. Lives in the filter pipeline alongside braille_dust, tint, dim, etc., so every consumer (factory, recipe preview, any future consumer) gets it automatically. Supports vertical gradient of charsets, per-cell jitter, and desynchronized timing. JSON: `{ "type": "charset_noise", "hz": 8.0, "seed": 42, "jitter": 0.15, "gradient": [...] }`.

### Changed
- **tui-vfx-content:** **BREAKING:** Removed `CharsetNoise` variant from `ContentEffect` enum. CharsetNoise is now a compositor filter (`FilterSpec::CharsetNoise`), not a content transformer. Recipes should use `pipeline.filter.dwell` instead of `content.effect`. The old `cls_charset_noise.rs` and `cls_charset_noise_config.rs` moved to recyclebin.

### Fixed
- **tui-vfx-compositor:** Fixed `CharsetNoise` filter time_step calculation — was dividing by 1000 (leftover from content transformer which received milliseconds). Compositor filters receive normalized t (0–1). Matches braille_dust's pattern.

## 0.2.5 — 2026-03-17

### Added
- **tui-vfx-content:** Added `CharsetNoise` content transformer — non-converging, time-varying character replacement with vertical gradient support. Unlike Scramble (which resolves toward target text), CharsetNoise cycles indefinitely, replacing characters from a configurable charset at a given hz rate. Supports position-aware charset gradients (sparse characters at top, dense at bottom) and per-cell jitter for organic variation. Designed for living textures: fire, rain, smoke, static noise. Including empty characters (like `⠀`) in sparse pools creates flickering shape boundaries.
- **tui-vfx-content:** Added `GradientStop` type for charset gradient configuration and `AffectMode` enum (`all` / `non_empty`) controlling which cells are replaced.
- **tui-vfx-content:** Added `CharsetNoise` variant to `ContentEffect` enum with full serde support (`"type": "charset_noise"` in JSON).
- **tui-vfx-compositor:** Added `Gravity` sampler — parabolic acceleration displacement (`0.5 * a * t²`) with terminal velocity cap. Positive acceleration = fall down/right, negative = rise up/left. Useful for falling text, rising smoke, drop-in entrances, and debris effects. JSON: `{ "type": "gravity", "axis": "y", "acceleration": 6.0, "terminal_velocity": 12.0 }`.
- **tui-vfx-compositor:** Added `drift` field to `BrailleDust` filter — shifts particle hash query position over each lifecycle step, faking gravity (positive = fall) or buoyancy (negative = rise) without per-particle state. JSON: `"drift": 2.0`.

### Fixed
- **tui-vfx-compositor:** `BrailleDust` filter now recognizes empty braille `⠀` (U+2800) as empty alongside whitespace, so dust particles correctly appear in braille art content that uses `⠀` for empty space.
- **tui-vfx-compositor:** `BrailleDust` particles now appear at staggered times — per-cell time offset desynchronizes step transitions so particles don't all flash in unison.
- **tui-vfx-compositor:** `BrailleDust` particles now fade in/out smoothly — foreground color is dimmed by a `sin(π * progress)` bell curve envelope over each lifecycle step, replacing the previous binary snap on/off.

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
- **tui-vfx-shadow:** All five renderers (HalfBlock, Solid, MediumShade, Braille, Gradient) now apply a +1 inset on right-edge `start_y` and bottom-edge `start_x`, so shadows start 1 cell further from the element corner. This improves grade-underlying visual weight by preventing the shadow from crowding the element boundary.
- **tui-vfx-shadow:** `ShadowGradeConfig::dramatic()` preset bumped `fg_dim_strength` from 0.28 → 0.40 and `fg_desaturate_strength` from 0.22 → 0.30, making bright underlying text (e.g. white) more visibly subdued in shadow regions.
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
