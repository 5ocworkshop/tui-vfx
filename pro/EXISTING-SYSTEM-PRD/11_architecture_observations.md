<!-- <FILE>pro/EXISTING-SYSTEM-PRD/11_architecture_observations.md</FILE> - <DESC>Chapter 11 of the evidence-backed Existing-System PRD: factual observations relevant to future architecture discussions. No proposals; no value judgements.</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Deepening pass surfaced five additional load-bearing observations.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — append §§11.16-11.20 from the deepening-pass sub-agent reports (V3 alias translation seam in SpatialShaderType::try_from_v3_payload, BlendMode is unused public surface, ColorSpace Oklch doc drift, V3 shader pattern inconsistency, BindableString/U16 aliases unused). 0.1.0: initial population.</CLOG> -->

# 11. Architecture-Relevant Observations

This chapter records facts that may matter for future architecture discussions. Per `pro/REVERSE-PRD.md` §"Phase 11 / Allowed", the allowed forms are factual statements about implementation, duplication, type sharing, and runtime dependencies; this chapter does **not** propose replacements or evaluate quality.

## 11.1 The compositor pipeline is a closed four-stage shape, with shadow as a paired pre-stage

The `render_pipeline_with_spec` driver (chapter 3 F001) lowers a `CompositionSpec` into a `CompositionOptions` and runs Sampler → Mask → Shader → Filter, with shadow rendered before the element it shadows (`crates/tui-vfx-shadow/src/lib.rs:18-30`). The four stages are exposed as separate spec types in `crates/tui-vfx-compositor/src/types/` (`SamplerSpec`, `MaskSpec`, `ShaderLayerSpec`, `FilterSpec`) and four parallel internal `pub(crate)` directories (`samplers/`, `masks/`, `filters/` plus shaders defined externally in `tui-vfx-style`). Pre/post-pass framework as catalogued in `steering/MARKETING.md:266-279` is a V3-planned generalization; chapter 12 records its as-built status.

## 11.2 The shader stage straddles two crates by design

`ShaderLayerSpec` lives in `tui-vfx-compositor::pipeline` (`crates/tui-vfx-compositor/src/pipeline/cls_shader_layer_spec.rs`), but the shader implementations live in `tui-vfx-style::models` (50 files) and `tui-vfx-style::models::v3` (11 V3 `cls_vfx_*` files). The V3 lowering seam at `crates/tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec.rs:25-28` (`ShaderWithRegion::try_from_v3_shader_family`) is the inter-crate bridge.

## 11.3 V2 and V3 surfaces coexist in `tui-vfx-style::models/`

`crates/tui-vfx-style/src/models/` contains 50 files at the top level (V2 named-shader catalog) plus a `v3/` sub-directory with 11 V3 `cls_vfx_*_shader.rs` shaders, 13 V3 `enum_vfx_*` behavior enums, two V3-lowering error enums (`enum_try_lower_v3_spatial_shader_error.rs`, `enum_try_lower_v3_style_effect_error.rs`), and one V2→V3 lowering function (`fnc_lower_legacy_spatial_shader.rs`). The `<CLOG>` blocks across the workspace (`Cargo.toml:5-10`) describe the cutover as in progress.

## 11.4 The grid-first contract is enforced by crate dependency direction

`tui-vfx-types` (the foundation) does not depend on any sibling crate (it depends only on `mixed-signals` (workspace), optional `serde`, and `tui-vfx-core` per `crates/tui-vfx-types/Cargo.toml:18-21`). The grid-first commitment in Intention 1 is structural: `Grid`, `OwnedGrid`, `Cell`, `Color`, `SemanticScene`, `RoleMap` all live in `tui-vfx-types`, and ratatui does not appear in any `Cargo.toml` of any workspace member at audit-time (a workspace-wide `grep "ratatui"` against `Cargo.toml` files returns no matches).

## 11.5 The compositor crate's `filters/`, `masks/`, `samplers/` are `pub(crate)`

The implementation classes are private (`crates/tui-vfx-compositor/src/lib.rs:7-9`); the public surface is the `*Spec` types in `crates/tui-vfx-compositor/src/types/`. Consumers cannot reach the per-`cls_*` filter / mask / sampler files directly. This is recorded by REQ-004.

## 11.6 The shadow rendering has a published Quick Start example in its rustdoc

`crates/tui-vfx-shadow/src/lib.rs:35-50` ships a complete usage example as a doctest; the shadow crate (chapter 3 F006) is the only one of the V3-era crates with a top-level Quick Start in its module rustdoc. Other crates ship doctests inside per-feature blocks (e.g., `crates/tui-vfx-content/src/lib.rs:33-46, 69-83, 96-104` ship three doctests for the Typewriter / Scramble / cursor-presets paths).

## 11.7 The recipe-authoring layer is an out-of-workspace crate

The `tui-vfx-recipes` crate (referenced from `crates/tui-vfx/src/lib.rs:14-31` rustdoc and from the `xtask recipes validate` subcommand) is a sibling repository. The workspace under audit consumes recipe JSON only as input to the `xtask recipes validate` tooling and to one style-crate test fixture; it does not own recipe parsing, substitution, validation, or canonical playback-item construction. (Intention 3 records this as a deliberate architectural boundary.)

## 11.8 `tui-vfx-debug` carries two distinct responsibilities

`crates/tui-vfx-debug/src/lib.rs:5-30` documents the crate as carrying **two complementary responsibilities**: the debug logger (chapter 3 F035) and the inspection foundation (F036). The two surfaces are independent — `mod config; mod logger;` is private; `pub mod inspection` is public. The inspection module gains 14 `cls_*.rs` files; the logger surface ships two re-export lines.

## 11.9 The `tui-vfx-probe` crate is the only consumer of `tui-vfx-compositor` in `[dependencies]`

Per the dep matrix in chapter 2 §2.4.1, `tui-vfx-probe` is the only crate that depends on `tui-vfx-compositor` at runtime. The meta-crate `tui-vfx` re-exports `tui_vfx_compositor` (`crates/tui-vfx/src/lib.rs:186`) but its direct consumers (the two example targets and the `tests/` files) are the only consumers that import the compositor through the meta-crate's umbrella.

## 11.10 The clean-room V3.1 spike is intentionally narrow

`tui-vfx-next` (chapter 3 F048) depends only on `tui-vfx-types` and `tui-vfx-geometry` (`crates/tui-vfx-next/Cargo.toml:14-16`). Its module-level rustdoc at `crates/tui-vfx-next/src/lib.rs:3-9` says it "proves the Phase A semantic surface rules without depending on the legacy compositor, style, content, or shadow implementation crates." The crate is **not** imported by any other workspace member at audit-time, including the meta-crate `tui-vfx`. (Chapter 12 records the open question of its planned trajectory.)

## 11.11 `xtask` reproduces the production crates as path dependencies, plus a separate `mixed-signals` path

Per `xtask/Cargo.toml:45-51`, `xtask` declares its workspace-internal deps via explicit `path = "../crates/<name>"` rather than `workspace = true` (chapter 2 §2.4.1). The `mixed-signals` reference uses `path = "../../mixed-signals"` (`:51`) — a two-level relative path. This stylistic divergence from the rest of the workspace is the only path-vs-workspace deviation in the manifest.

## 11.12 Two identical pool aliases for two different asset families

Per `crates/tui-vfx-content/src/pool/mod.rs:19-26`, `ImagePool = Pool<String>` and `FontPool = Pool<String>` — both are aliases of the same generic over the same item type. The distinction is purely semantic (asset names that the host's `AssetMap` resolves to `.rss` bytes vs. `.rsf` bytes). Documented inline at `mod.rs:28-39` as the asset-naming convention.

## 11.13 The compositor depends on `tui-vfx-debug` at runtime

Per `crates/tui-vfx-compositor/Cargo.toml:38`, `tui-vfx-debug` is a runtime `[dependencies]` entry, not a `[dev-dependencies]`. The meta-crate `tui-vfx` only declares `tui-vfx-debug` as a `[dev-dependencies]` (`crates/tui-vfx/Cargo.toml:29`). This means production consumers of the compositor pull in the logger and inspection crate; consumers of the meta-crate without the dev-deps do not see `tui-vfx-debug` in their public-API surface.

## 11.14 `tui-vfx-types`'s optional `serde` is the only behaviorally-meaningful Cargo feature

Three crates declare `[features]` blocks (chapter 2 §2.5); only one gates real behavior (`tui-vfx-types::serde`, on by default). The other two (`tui-vfx-geometry::default = []`, `tui-vfx-content::default = []`) are present-but-empty. The optional-serde gate produces exactly two `#[cfg(feature = "serde")]` impls in `crates/tui-vfx-types/src/interned_string.rs:114,121` (chapter 5 §5.2.5) — these are the only non-test `cfg` gates in the workspace.

## 11.15a Content-transformer field-visibility asymmetry

The 15 transformer classes under `crates/tui-vfx-content/src/transformers/` use inconsistent field-visibility patterns. Only `Typewriter.speed_variance`, the 17 fields of `SplitFlap`, and `WrapIndicator.{prefix, suffix}` are `pub`. The other 12 transformers carry private fields with no accessor methods — recipe-construction therefore goes through `new(...)` / `new_mechanical(...)` / preset-style constructors. Three of those (SplitFlap, Odometer, SlideShift, WrapIndicator) carry no `Default` impl. `Numeric::new` accepts `&str` (clones internally) while all other constructors that take strings accept owned `String`. Each transformer's specific evidence is in chapter 3 F012's per-transformer table.

## 11.15b Eight transformers ignore the runtime context

8 of the 15 content transformers (GlyphCascade, Mirror, Morph, Numeric, Redact, Dissolve, SlideShift, WrapIndicator) ignore `ctx: &TransformContext` in their `transform` body — they are deterministic functions of `progress + seed`. Only the remaining 7 (Typewriter, Scramble, GlitchShift, ScrambleGlitchShift, SplitFlap, Marquee, Odometer) consume `ctx.signal_ctx` and / or `ctx.runtime_params`. This split is observable in F012's per-transformer table.

## 11.15c Three production `expect()` panics in the shadow path of `orc_render_pipeline`

`crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs` contains exactly three production-path `.expect()` calls beyond the V3-lowering one captured by REQ-002:

1. `:255` — `let shadow_spec = options.shadow.as_ref().expect("shadow_spec must be Some");` Caller-asserted: only reachable after the `is_some()` dispatch at `:118`.
2. `:559` — `shadow_cell.expect("shadow region candidate must have shadow coverage")`. Guarded by the `shadow_has_coverage` boolean at `:553`.
3. `:582, :712` — `shadow_cell.expect("shadow coverage implies a shadow cell")`. Same `shadow_has_coverage` guard.

All three guards are local to the surrounding code; if the guards are correct, the panics are unreachable. Recorded in F001's "Errors and edge cases" with the panic table.

## 11.16 V3 alias-translation seam in `SpatialShaderType::try_from_v3_payload`

`crates/tui-vfx-style/src/models/cls_spatial_shader_type.rs:304+` mutates the JSON payload in place before deserializing. V3-authored type names like `fractional_stripe_overlay`, `gradient_overlay`, `colored_overlay` are rewritten to V2 type names so `serde` can match them. This is a quiet alias-translation layer that affects how recipe-loader errors surface; consumers that introspect raw JSON bytes vs. the deserialised `SpatialShaderType` see different shapes. No SSOT violation per se; documented here so future readers understand the rewrite.

## 11.17 `ColorSpace` documentation drift (Oklch)

`crates/tui-vfx-style/src/models/cls_style_effect.rs:218` rustdoc for `StyleEffect::ColorFade` lists `Oklch` as one of the available color spaces. The actual `ColorSpace` enum at `cls_color_space.rs:31` has only three variants — `Rgb`, `Hsl`, `Hct` — and authoring `"oklch"` against the deserialiser fails. A drift documented; not load-bearing for runtime, but load-bearing for the doc-generation toolchain.

## 11.18 V3 shader struct pattern inconsistency

8 of 11 V3 shaders (`VfxEdgeDistortionShader`, `VfxGradientRevealShader`, `VfxGuidanceCueShader`, `VfxMaterialLightShader`, `VfxMotionFieldShader`, `VfxStochasticTextureShader`, `VfxStripeMotionShader`, `VfxSurfaceDepthShader`) wrap a single `behavior: VfxXBehavior` field. The other 3 lift fields out of the behavior:

- `VfxCursorShader` (`cls_vfx_cursor_shader.rs:23`) — fully flat layout mirroring V2 `CursorShader`.
- `VfxProgressEmphasisShader` (`cls_vfx_progress_emphasis_shader.rs:26`) — fully flat layout, 13 sibling fields.
- `VfxTravelingBandShader` (`cls_vfx_traveling_band_shader.rs:27`) — mixed: `speed` and `color` lifted out + `behavior:` field.

Whether this is an in-flight V3-design artefact or a deliberate two-tier convention is not documented in steering. Per Intention 23 (Rule of Three) the inconsistency is at the threshold where the family rule should be settled — if 3 of 11 shaders need the flat layout, that pattern itself is the third occurrence and should be either lifted to a shared shape or explicitly documented as the exception.

## 11.19 `BindableString` / `BindableU16` aliases have no in-tree consumer

`crates/tui-vfx-style/src/models/cls_bindable_string.rs:14` and `cls_bindable_u16.rs:14` are 17-line re-export shims (`pub use tui_vfx_core::bindable::VfxBindable{String,U16} as Bindable{String,U16}`). The original 322-LOC and 250-LOC bodies were retired to `recyclebin/` per the 1.2.A consolidation. **No in-tree shader struct actually consumes the typed aliases** — bindings are still raw `Option<String>` keys (e.g. `speed_binding: Option<String>`, `direction_binding: Option<String>`) throughout the V2 shader catalogue and the V3 `VfxProgressEmphasisShader`. The typed-alias surface is therefore tooling-facing only at audit-time.

## 11.20 `BlendMode` is unused public surface

`crates/tui-vfx-style/src/models/cls_blend_mode.rs:18` declares `pub enum BlendMode { Normal, Additive, Multiply, Screen, Overlay, Mix }` (default `Normal`) plus `BlendMode::blend(base, overlay, strength)`. **No shader struct field references `BlendMode`** at audit-time (workspace-wide grep confirms zero in-tree consumers). It is therefore present-but-unused public surface — the kind of accumulation Intention 24 ("Library Changes Must Earn Their Place") explicitly guards against. Not a defect (it is a small enum), but worth a steering review.

## 11.15 Sibling-repo coupling shape

The workspace path-depends on one sibling (`mixed-signals` at `../mixed-signals`, version `0.3.0`, `Cargo.toml:52`) and crates.io-depends on another sibling (`rocketsplash-rt 0.2.2`, `Cargo.toml:65`). Three further sibling repos are referenced in steering docs / rustdoc but not in `[dependencies]` at audit-time: `tui-vfx-recipes` (the recipe-authoring crate), `gt-design` (the first production consumer), and `rocketsplash` (the asset-authoring tool).

<!-- <FILE>pro/EXISTING-SYSTEM-PRD/11_architecture_observations.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
