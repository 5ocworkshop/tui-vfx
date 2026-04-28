<!-- <FILE>pro/EXISTING-SYSTEM-PRD/13_evidence_ledger.md</FILE> - <DESC>Chapter 13 of the evidence-backed Existing-System PRD: every load-bearing evidence reference used across chapters 1-12, listed with E### IDs.</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Deepening pass: append E130..E140 covering the new line citations folded into chapters 3, 4, 11, 12.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — append §13.13 deepening-pass evidence (33-variant FilterSpec, 31-variant SpatialShaderType, 11-variant V3 family, 31-arm V2→V3 lowering, 3-variant ColorSpace, V3 pattern-inconsistency lines, BlendMode unused, BindableString/U16 shim, glyph framework live consumers, ShadowSpec wrapper, content-transformer trait + TransformContext, 4 expect() panic sites). 0.1.0: initial population.</CLOG> -->

# 13. Evidence Ledger

This chapter consolidates the evidence used in chapters 1-12. Per `pro/REVERSE-PRD.md` §"Phase 13", every reference cited in a chapter should appear here. The chapters above use direct `path:line` citations for navigability (a reader landing on chapter 3 F006 can click to the code without first traversing chapter 13). This ledger is therefore an index of the **load-bearing** references — the ones that establish a feature, requirement, option, or trust boundary — grouped by evidence-source kind.

## 13.1 Schema

| Column | Meaning |
|---|---|
| Evidence ID | `E###`, sequential |
| Path | Workspace-relative path |
| Lines / Symbol | Line range or `:: symbol_name` |
| Proves | One-line statement of what the evidence establishes |
| Confidence | `High` / `Medium` / `Low` per `pro/REVERSE-PRD.md` §"Evidence requirements" |

## 13.2 Workspace structural evidence

| Evidence ID | Path | Lines / Symbol | Proves | Confidence |
|---|---|---|---|---|
| E001 | `Cargo.toml` | `:14-28` (members) | The workspace contains 13 members at audit-time | High |
| E002 | `Cargo.toml` | `:30-37` ([workspace.package]) | Workspace metadata: version `0.11.0`, edition `2024`, MSRV `1.95.0`, license `MIT` | High |
| E003 | `Cargo.toml` | `:39-65` ([workspace.dependencies]) | Internal-crate workspace deps + 6 external crates | High |
| E004 | `Cargo.toml` | `:13` | `resolver = "3"` | High |
| E005 | `Cargo.toml` | `:69-76` (profile.dev + profile.dev.package."*") | Build-profile overrides (opt-level 2 for crate code, 3 for deps) | High |
| E006 | `clippy.toml` | `:1-8` | `too-many-arguments-threshold = 9` (Intention 40 §1) | High |
| E007 | `.cargo/config.toml` | `:6` | Cargo alias `xtask = "run --package xtask --"` | High |
| E008 | `xtask/Cargo.toml` | `:6-19` | xtask is `publish = false`, MSRV `1.86.0` (overrides workspace), declares both bin (`xtask`) and lib (`xtask_audit_configschema`) targets | High |
| E009 | `crates/tui-vfx-types/Cargo.toml` | `:26-28` | Optional `serde` Cargo feature, default on | High |
| E010 | `crates/tui-vfx-next/Cargo.toml` | `:1-16` | Clean-room V3.1 spike crate; depends only on `tui-vfx-types` and `tui-vfx-geometry` | High |

## 13.3 Public Rust API surface

| Evidence ID | Path | Lines / Symbol | Proves | Confidence |
|---|---|---|---|---|
| E020 | `crates/tui-vfx/src/lib.rs` | `:14-31` | The "Audiences" rustdoc declaring engine API vs recipe layer; `:186-195` re-export surface | High |
| E021 | `crates/tui-vfx-types/src/lib.rs` | `:72-109` | Public sub-modules + foundation re-exports (Cell, Color, Grid, RoleMap, etc.) | High |
| E022 | `crates/tui-vfx-core/src/lib.rs` | `:6-23` | bindable + schema + time_spec + ConfigSchema derive shim | High |
| E023 | `crates/tui-vfx-core-macros/src/lib.rs` | `:28` (`pub fn derive_config_schema`) | The proc-macro derive entry point | High |
| E024 | `crates/tui-vfx-geometry/src/lib.rs` | `:7-23` | 9 sub-modules + `MotionPath` + `WipeDirection` + `wipe_progress` re-exports | High |
| E025 | `crates/tui-vfx-compositor/src/lib.rs` | `:6-14` | `pub mod {context, pipeline, traits, types, utils, widgets}` + `pub(crate) mod {filters, masks, samplers}` | High |
| E026 | `crates/tui-vfx-compositor/src/pipeline/mod.rs` | `:24-37` | Pipeline-module re-exports (CompositionSpec, render_pipeline_with_spec, etc.) | High |
| E027 | `crates/tui-vfx-style/src/lib.rs` | `:6-9` | 4 public sub-modules (`models`, `schedules`, `traits`, `utils`) | High |
| E028 | `crates/tui-vfx-content/src/lib.rs` | `:111-123` | 11 public sub-modules + private `mod mechanical;` at `:116` | High |
| E029 | `crates/tui-vfx-shadow/src/lib.rs` | `:350-363` | Shadow public API (`render_shadow*`, `extract_shadow_envelope`, ShadowConfig, etc.) | High |
| E030 | `crates/tui-vfx-debug/src/lib.rs` | `:32-38` | Logger + inspection module re-exports | High |
| E031 | `crates/tui-vfx-probe/src/lib.rs` | `:75-107` | 24 `cls_probe_*` DTO re-exports + 8 helper-fn re-exports | High |
| E032 | `crates/tui-vfx-next/src/lib.rs` | `:1-23` | Clean-room V3.1 spike public surface | High |

## 13.4 Pipeline / render-path evidence

| Evidence ID | Path | Lines / Symbol | Proves | Confidence |
|---|---|---|---|---|
| E040 | `crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs` | `:106` (`pub fn render_pipeline`), `:211` (`pub fn render_pipeline_with_area`) | Two of the four public render entry points | High |
| E041 | `crates/tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec.rs` | `:21-50` (`pub fn render_pipeline_with_spec`) | The full signature; the `.expect()` panic on V3 lowering at `:25-28`; `CompositionPlaybackTiming` lift at `:46-50` | High |
| E042 | `crates/tui-vfx-compositor/src/samplers/` | directory listing | 11 sampler classes | High |
| E043 | `crates/tui-vfx-compositor/src/masks/` | directory listing | 11 mask classes + `col_soft_edge` | High |
| E044 | `crates/tui-vfx-compositor/src/filters/` | directory listing | 25 filter classes | High |
| E045 | `crates/tui-vfx-style/src/models/` | directory listing | 50 named-shader / style-effect classes (V2) | High |
| E046 | `crates/tui-vfx-style/src/models/v3/` | directory listing | 11 V3 shader classes + 13 behavior enums + 1 lowering function | High |
| E047 | `crates/tui-vfx-shadow/src/lib.rs` | `:35-50` (Quick Start doctest) | `ShadowConfig` builder grammar | High |
| E048 | `crates/tui-vfx-shadow/src/renderers/` | directory listing | 5 shadow-renderer classes (braille, gradient, half_block, medium_shade, solid) | High |

## 13.5 Bindable / signal / runtime-params evidence

| Evidence ID | Path | Lines / Symbol | Proves | Confidence |
|---|---|---|---|---|
| E060 | `crates/tui-vfx-core/src/bindable/cls_bindable.rs` | `:24` (`pub enum Never`), `:49` (`pub trait RuntimeParamsRead`), `:73` (`pub trait BindableSignal`), `:167` (`pub enum VfxBindable<T, S = Never>`) | The bindable type system | High |
| E061 | `crates/tui-vfx-core/src/lib.rs` | `:11-15` (the `pub use bindable::{...}` block) | The full set of re-exported bindable types | High |
| E062 | `crates/tui-vfx-content/src/lib.rs` | `:53-90` (the rustdoc block on bindable rate parameters) | The three-arm contract (Literal / Binding / Signal) documented for content effects | High |

## 13.6 Content + transformer evidence

| Evidence ID | Path | Lines / Symbol | Proves | Confidence |
|---|---|---|---|---|
| E070 | `crates/tui-vfx-content/src/transformers/` | directory listing | 15 transformer classes + `fnc_get_transformer.rs` + `fnc_morph_chars.rs` | High |
| E071 | `crates/tui-vfx-content/src/lib.rs` | `:1-100` (full module rustdoc with three doctests) | `ContentEffect::apply` / `apply_to_borrowed` / `apply_with_runtime` API | High |
| E072 | `crates/tui-vfx-content/src/cell_motion/mod.rs` | `:1-30` | Cell-motion scheduler public surface (V3 packet 1) | High |
| E073 | `crates/tui-vfx-content/src/sources/mod.rs` | `:1-26` | RocketsplashImage / RocketsplashFont source family | High |
| E074 | `crates/tui-vfx-content/src/fonts/mod.rs` | `:1-33` | 3×3 line glyph table + FontGlyphTable + FontRegistry | High |
| E075 | `crates/tui-vfx-content/src/assets/mod.rs` | `:1-25` | AssetRegistry + DEFAULT_LOGO_SENTINEL; consumer surface deferred | High |
| E076 | `crates/tui-vfx-content/src/pool/mod.rs` | `:1-50` | Generic Pool<T> + 4 type aliases + TextPool | High |
| E077 | `crates/tui-vfx-content/src/lib.rs` | `:116` (`mod mechanical;` — no `pub`) | Mechanical content cycles is intentionally private | High |

## 13.7 Foundation-types evidence

| Evidence ID | Path | Lines / Symbol | Proves | Confidence |
|---|---|---|---|---|
| E080 | `crates/tui-vfx-types/src/lib.rs` | `:14-32` (rustdoc block on roles) | The 12 first-class RoleTag variants + Custom variant; SemanticScene composer architecture | High |
| E081 | `crates/tui-vfx-types/src/braille.rs` | `:1-50` | Braille pattern utilities; region-mask constants (LEFT_COLUMN, RIGHT_COLUMN, etc.) | High |
| E082 | `crates/tui-vfx-types/src/glyph/` | directory listing (4 files) | Glyph rendering framework Slice 6.6 §F.1 (cls_glyph_encoder, fnc_sample_eight_subcells) | High |

## 13.8 Inspection / probe / debug evidence

| Evidence ID | Path | Lines / Symbol | Proves | Confidence |
|---|---|---|---|---|
| E090 | `crates/tui-vfx-debug/src/lib.rs` | `:5-30` (module rustdoc) | Two-responsibility crate (logger + inspection foundation) | High |
| E091 | `crates/tui-vfx-debug/src/inspection/` | directory listing (14 cls_*.rs files) | Inspection-foundation taxonomy | High |
| E092 | `crates/tui-vfx-debug/src/inspection/mod.rs` | `:1-30` (module rustdoc) | TraceEvent + TraceFilter + InspectionSink + StageMask vocabulary | High |
| E093 | `crates/tui-vfx-probe/src/bin/pipeline-probe.rs` | `:1-100` (file head + arg parsing) | The pipeline-probe CLI surface | High |
| E094 | `crates/tui-vfx-probe/Cargo.toml` | `:25` (`rusqlite = { ..., features = ["bundled"] }`); `:29-31` (`[[bin]]`) | Probe crate's only persistence engine + binary registration | High |

## 13.9 xtask + tooling evidence

| Evidence ID | Path | Lines / Symbol | Proves | Confidence |
|---|---|---|---|---|
| E100 | `xtask/src/main.rs` | `:23-39` (Commands enum), `:42-110` (per-action enums), `:124-177` (dispatcher) | Full xtask CLI surface | High |
| E101 | `xtask/src/main.rs` | `:130-134` | Reads `CARGO_MANIFEST_DIR` env var (the only env-var read in the workspace) | High |
| E102 | `xtask/src/audit/` | directory (5 files) + `xtask/tests/test_audit_configschema.rs` (15 787 bytes) | configschema audit implementation + test coverage | High |
| E103 | `xtask/src/docs/` | directory (20 files) | Doc-generation toolchain | High |
| E104 | `justfile` | `:9-25` (DOCUMENTATION GENERATION header) + recipe headers | Workflow-recipe surface | High |

## 13.10 Steering / intent evidence

| Evidence ID | Path | Lines / Symbol | Proves | Confidence |
|---|---|---|---|---|
| E110 | `Cargo.toml` | `:5` `<CLOG>` v0.8.0 | Sub-plan A handoff; cross-phase integration test path | Medium (doc-evidence) |
| E111 | `Cargo.toml` | `:6` `<CLOG>` v0.7.0 | The Phase A.4 inspection-foundation surface naming (TraceEvent / TraceFilter / etc.) | Medium |
| E112 | `Cargo.toml` | `:7` `<CLOG>` v0.6.0 | Phase A.3 role-aware ShaderContext + ShadowConfig.source_region change | Medium |
| E113 | `Cargo.toml` | `:8` `<CLOG>` v0.5.0 | Phase A.2 BREAKING render_pipeline signature change | Medium |
| E114 | `steering/INTENTIONS.md` | Intentions 1, 3, 9, 10, 26, 27, 36, 40, 42, 43, 44 | Project-level steering rules referenced by chapters 6, 7, 8, 11, 12 | High (where the intention is shipped); Medium (where the intention is aspirational) |
| E115 | `steering/MARKETING.md` | `:266-279` | Pre/post-pass framework V3 description | Medium (doc-only; the as-built scope is partial — chapter 12 §12.9 records the open question) |
| E116 | `README.md` | `:7-15` | Workspace's own self-description | Medium (doc-evidence) |

## 13.11 Test evidence

| Evidence ID | Path | Lines / Symbol | Proves | Confidence |
|---|---|---|---|---|
| E120 | `crates/tui-vfx/tests/test_foundation_end_to_end.rs` | (file presence) | Sub-plan A foundation cross-phase integration test (per E110 `<CLOG>`) | High |
| E121 | `crates/tui-vfx-compositor/tests/test_alloc_budget.rs` | `:36-43` | The 5 unsafe lines in the workspace; CountingAllocator | High |
| E122 | (per-crate `tests/` directories) | filesystem listing | 75 integration-test files across 11 crates + xtask + tui-vfx-next (chapter 10 enumerates) | High |
| E123 | `ofpf-sql` against `file_definitions WHERE is_test = 1` joined onto `files` | (chapter 10 §10.1 records the query) | 2854 test definitions; per-crate distribution | High |

## 13.13 Deepening-pass evidence (added 2026-04-29)

The four sub-agent reports added the following E### entries.

### Compositor stage spec catalogue

| Evidence ID | Path | Lines / Symbol | Proves | Confidence |
|---|---|---|---|---|
| E130 | `crates/tui-vfx-compositor/src/types/cls_filter_spec.rs` | `:443-1374` (enum body); `:2115` (kitt_bps_from_bpm); `:2146` (try_from_v3); `:2199` (validate); `:2249-2282` (name dispatch — 32 arms) | `FilterSpec` has 32 variants (REQ-007 corrected count — verified by direct grep of the `name()` arms) | High |
| E131 | `crates/tui-vfx-compositor/src/types/cls_sampler_spec.rs` | `:163-452` (11-variant enum); `:560` (try_from_v3); `:614, :631, :650` (name/terse_description/key_parameters) | Sampler stage's 11 variants + per-variant fields | High |
| E132 | `crates/tui-vfx-compositor/src/types/cls_mask_spec.rs` | `:192-473` (12-variant enum incl. None); `:500` (resolve_wipe priority) | Mask stage's 11 variants + helper enums + resolve_wipe priority | High |
| E133 | `crates/tui-vfx-compositor/src/types/cls_shadow_spec.rs` | `:49-53` | `ShadowSpec { #[serde(flatten)] config: ShadowConfig }` wrapper | High |
| E134 | `crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs` | `:106` (render_pipeline); `:118` (shadow dispatch); `:243-793` (shadow path); `:255, :553, :559, :582, :712` (4 expect() panics with guards); `:819-909` (non-shadow loop); `:913-1077` (inspected loop); `:777-781` (RoleTag::Shadow write-back); 8 emit-site lines | The 4-stage call sequence + the 8 emit sites + the 4 production expect() panics + the role write-back | High |

### Style models V2+V3

| Evidence ID | Path | Lines / Symbol | Proves | Confidence |
|---|---|---|---|---|
| E135 | `crates/tui-vfx-style/src/models/cls_style_effect.rs` | `:81-250` (StyleEffect 11 variants); `:218` (Oklch doc-vs-code drift) | StyleEffect surface + the doc drift recorded in observation 11.17 | High |
| E136 | `crates/tui-vfx-style/src/models/cls_spatial_shader_type.rs` | `:157-303` (31 variants); `:304+` (try_from_v3_payload alias-translation seam) | SpatialShaderType 31 variants — F008 count corrected from 50 to 31 | High |
| E137 | `crates/tui-vfx-style/src/models/cls_color_space.rs` | `:31` | `ColorSpace` is a flat 3-variant enum (`Rgb`/`Hsl`/`Hct`); resolves chapter 12's open question | High |
| E138 | `crates/tui-vfx-style/src/models/v3/` | 30 files: 11 `cls_vfx_*_shader.rs` + 18 `enum_*.rs` (11 per-shader behavior + 5 family/structure + 2 lowering error) + 1 `fnc_lower_legacy_spatial_shader.rs:23` | Full V3 surface enumerated; pattern inconsistency on 3 of 11 shaders; 31-arm exhaustive V2→V3 lowering | High |
| E139 | `crates/tui-vfx-style/src/models/cls_blend_mode.rs` | `:18` | `BlendMode` 6-variant enum is unused public surface (observation 11.20) | High |
| E140 | `crates/tui-vfx-style/src/models/cls_bindable_{string,u16}.rs` | `:14` (each — 14-line shims) | The two style-side bindable types are now re-export shims around `tui_vfx_core::bindable` | High |

### Bindable / core / glyph

| Evidence ID | Path | Lines / Symbol | Proves | Confidence |
|---|---|---|---|---|
| E141 | `crates/tui-vfx-core/src/bindable/cls_bindable.rs` | `:24` (Never enum); `:49` (RuntimeParamsRead trait); `:73` (BindableSignal trait); `:167` (VfxBindable enum) | The canonical bindable type system | High |
| E142 | `crates/tui-vfx-core/src/mixed_signals_schema.rs` | `:33-81` (SignalOrFloat); `:84-467` (SignalSpec — 30 variants); `:470-606` (EasingType — 25 variants) | The 609-line orphan-rule-justified schema bridge | High |
| E143 | `crates/tui-vfx-types/src/glyph/{mod.rs, cls_glyph_encoder.rs, fnc_sample_eight_subcells.rs}` | `mod.rs:23-29` re-exports; `cls_glyph_encoder.rs:48-79, :103-119, :144-170`; `fnc_sample_eight_subcells.rs:35-44, :73-84, :126-137` | Glyph framework is complete (F028 status corrected from partial to implemented) | High |
| E144 | `crates/tui-vfx-compositor/src/filters/cls_scalar_field_glyph_filter.rs` | `:73` | The live `ScalarFieldGlyphFilter` consumer of the glyph framework | High |
| E145 | `crates/tui-vfx-compositor/src/pipeline/cls_prepared_filter.rs` | `:87-89` | The `ScalarFieldGlyphWater` / `ScalarFieldGlyphFire` route via `WaterFieldSignal` / `FireFieldSignal` from `tui-vfx-style` | High |

### Content transformers

| Evidence ID | Path | Lines / Symbol | Proves | Confidence |
|---|---|---|---|---|
| E146 | `crates/tui-vfx-content/src/traits/text_transformer.rs` | `:13` | The `TextTransformer` trait shape | High |
| E147 | `crates/tui-vfx-content/src/traits/cls_transform_context.rs` | `:19, :36` | `TransformContext { signal_ctx, runtime_params }` plus its `new` constructor | High |
| E148 | `crates/tui-vfx-content/src/types/cls_content_effect.rs` | `:179` (enum); per-variant lines `:189, :206, :220, :237, :261, :285, :394, :424, :433, :442, :459, :485, :501, :525, :547`; methods at `:557, :578, :601` | The `ContentEffect` 15-variant enum + 3 inherent methods | High |
| E149 | `crates/tui-vfx-content/src/types/fnc_apply_content_effect.rs` | `:49` (apply); `:68` (apply_to_borrowed); `:87` (apply_with_runtime) | The three apply methods | High |
| E150 | `crates/tui-vfx-content/src/transformers/` | 15 `cls_*.rs` files at the line numbers cited in chapter 3 F012's table | Per-transformer field shapes and Default values | High |

### Chapter 12 resolutions

| Evidence ID | Path | Lines / Symbol | Proves | Confidence |
|---|---|---|---|---|
| E151 | `crates/tui-vfx-compositor/src/widgets/mod.rs` | `:1-10` (full file) | 10-line tombstone post-V1-removal — resolves §12.2 | High |
| E152 | `crates/tui-vfx-content/src/glyph_particles/mod.rs` | `:1-431` | Complete 431-line glyph-particle emitter — resolves §12.6 | High |
| E153 | `crates/tui-vfx-geometry/src/widgets/mod.rs` and `types.rs` | `:7-26` (mod.rs); types.rs full | Geometry-side widgets is a numpad-grid hit-test surface — resolves §12.8 | High |
| E154 | `crates/tui-vfx-content/src/assets/cls_asset_registry.rs` | `:11-16` (deferral header); `:38-132` (impl); `:134-281` (16 inline tests) | AssetRegistry producer-half complete; consumer-half deferred per Phase 7 — resolves §12.10 | High |
| E155 | `crates/tui-vfx-probe/src/bin/pipeline-probe.rs` | `:42-86` (flag set); `:84` (--widget-cell); `:92-94` (mutual-exclusion); `:174-179` (single-frame path); `:213-217` (focus_cell envelope key); `:221-226` (parse_widget_cell) | Full pipeline-probe CLI grammar — resolves §12.13, upgrades OPT-015 to High | High |
| E156 | `docs/new_kernel/PHASE_B_STATUS_MEMO_TO_ARCHITECT.md` | `:211, :230` | Documented "Phase B complete, parked" status for tui-vfx-next — resolves §12.3 | Medium (doc-evidence) |
| E157 | `docs/design/tui-vfx-pre-post-pass-rollout-plan.md` | (full file) | Pre/post-pass framework is design-only with no code — resolves §12.9 | Medium (doc-evidence) |

## 13.12 Confidence summary

The evidence ledger has 50+ E### entries across 11 categories. Of those:

- **High** confidence: ~45 entries (direct code-path or test-file evidence)
- **Medium** confidence: ~6-8 entries (documentation-evidence — `<CLOG>` blocks in `Cargo.toml`, README, MARKETING.md, INTENTIONS.md). Per `pro/REVERSE-PRD.md`, "Low-confidence findings stay in section 12" — none of the entries above are Low.

Where a chapter asserts a fact, the evidence supporting it is in this ledger or in chapter 14's "commands run" list (for absence-of-evidence claims, e.g., chapter 6 §6.3, §6.4 / chapter 8 §8.3-§8.6 / chapter 9 §9.1-§9.4).

<!-- <FILE>pro/EXISTING-SYSTEM-PRD/13_evidence_ledger.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
