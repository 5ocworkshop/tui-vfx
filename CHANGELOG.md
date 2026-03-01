<!-- <FILE>CHANGELOG.md</FILE> - <DESC>Release history for tui-vfx</DESC> -->
<!-- <VERS>VERSION: 1.2.0</VERS> -->
<!-- <WCTX>Add 0.2.1 patch release entry</WCTX> -->
<!-- <CLOG>Document HalfBlock right-edge shadow convention fix</CLOG> -->

# Changelog

All notable changes to this project will be documented in this file.

This project follows [Semantic Versioning](https://semver.org/).

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
<!-- <VERS>END OF VERSION: 1.2.0</VERS> -->
