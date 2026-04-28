<!-- <FILE>pro/EXISTING-SYSTEM-PRD/05_options_and_configuration.md</FILE> - <DESC>Chapter 5 of the evidence-backed Existing-System PRD: every option that alters behavior — CLI, environment, config-file, Cargo features, compile-time cfgs, constants, API parameters, and workflow recipes.</DESC> -->
<!-- <VERS>VERSION: 0.1.1</VERS> -->
<!-- <WCTX>Mid-audit citation refresh after the 13th workspace member (crates/tui-vfx-next) was added; profile.dev cites now point at lines 72-73 / 75-76.</WCTX> -->
<!-- <CLOG>0.1.1: PATCH — re-anchor OPT-021 / OPT-022 citations after Cargo.toml line shift. 0.1.0: initial population.</CLOG> -->

# 5. Options and Configuration Catalog

## 5.1 Master options table

Categories per `pro/REVERSE-PRD.md` §"Phase 3": CLI / environment / config file / Cargo feature / compile-time cfg / constant / API parameter / database setting / unknown.

The workspace has **no config-file-reading code path** — clippy.toml, .cargo/config.toml, and Cargo.toml are build-time inputs to Cargo and rustc, not runtime inputs to any binary. The only runtime-read environment variable is `CARGO_MANIFEST_DIR` (one site, in `xtask`). The bulk of the option surface is therefore CLI flags (xtask + pipeline-probe), Cargo features, and a small set of public constants.

| Option ID | Name | Category | Type | Default | Required | Scope | Behavior Affected | Evidence | Confidence |
|---|---|---|---|---|---|---|---|---|---|
| OPT-001 | `xtask audit configschema` | CLI subcommand | enum branch | n/a | as-invoked | binary `xtask` | Run the ConfigSchema-justification lint | `xtask/src/main.rs:42-52,127-145` | High |
| OPT-002 | `xtask docs <action>` | CLI subcommand | enum branch (13 actions) | n/a | as-invoked | binary `xtask` | Generate/check/scaffold capability + API + signals docs | `xtask/src/main.rs:54-110,147-162` | High |
| OPT-003 | `xtask docs scaffold --write` | CLI flag | bool | `false` | optional | binary `xtask` | Write capability stubs to `docs/templates/capabilities.toml` instead of stdout | `xtask/src/main.rs:80-83` | High |
| OPT-004 | `xtask docs api-scaffold --write` | CLI flag | bool | `false` | optional | binary `xtask` | Write API stubs to `docs/templates/api_docs.toml` instead of stdout | `xtask/src/main.rs:99-102` | High |
| OPT-005 | `xtask recipes validate --recipes-dir <path>` | CLI flag | string | (no default — required) | required | binary `xtask` | Path to recipe JSON directory to validate | `xtask/src/main.rs:114-115` | High |
| OPT-006 | `xtask recipes validate --output-dir <path>` | CLI flag | string | `docs/generated` | optional | binary `xtask` | Path to write per-recipe validation reports | `xtask/src/main.rs:117-119` | High |
| OPT-007 | `pipeline-probe --input <path>` | CLI flag | string | (none — first non-flag positional resolves through manual parser) | required | binary `pipeline-probe` | Path to a `ProbeSceneSpec` JSON document | `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:43,46` | High |
| OPT-008 | `pipeline-probe --format <fmt>` | CLI flag | string | `json` | optional | binary `pipeline-probe` | Output format selector (literal default `"json"`) | `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:35,47` | High |
| OPT-009 | `pipeline-probe --phase <entering|dwelling|exiting>` | CLI flag | enum | `dwelling` | optional | binary `pipeline-probe` | Lifecycle phase to probe | `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:36,48-55` | High |
| OPT-010 | `pipeline-probe --sample-t <f64>` | CLI flag | f64 | `0.5` | optional | binary `pipeline-probe` | Phase-local time at which to sample | `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:37,56-61` | High |
| OPT-011 | `pipeline-probe --cells <all|non-empty|modified>` | CLI flag | enum | `all` | optional | binary `pipeline-probe` | Cell-selection predicate for the report | `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:38,62-69` | High |
| OPT-012 | `pipeline-probe --with-causation` | CLI flag | bool | `false` | optional | binary `pipeline-probe` | Include per-cell root-cause analysis | `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:39,70` | High |
| OPT-013 | `pipeline-probe --frames <usize>` | CLI flag | usize | `None` (single frame) | optional | binary `pipeline-probe` | Sample N evenly-spaced frames across the phase (timeline mode) | `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:40,71-77` | High |
| OPT-014 | `pipeline-probe --diff-to <f64>` | CLI flag | f64 | `None` (no diff) | optional | binary `pipeline-probe` | Diff against a probe at a different phase-local time | `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:41,78-83` | High |
| OPT-015 | `pipeline-probe --widget-cell <x,y>` | CLI flag | comma-separated u16 pair (parsed by `parse_widget_cell` at `:221-226`) | `None` | optional, single-frame-only (mutually exclusive with `--frames`, `--diff-to`, `--sqlite-query`) | binary `pipeline-probe` | Return one widget cell + its root cause; output emitted as `focus_cell` field on the JSON envelope at `:213-217` | `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:84, :92-94, :174-179, :213-217, :221-226` | High |
| OPT-016 | `--sqlite-query <sql>` | CLI flag | string | `None` | optional | binary `pipeline-probe` | Run a SQL query against the persisted SQLite store | `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:41,74` | High |
| OPT-017 | `CARGO_MANIFEST_DIR` | environment variable | string | (unset → `.`) | optional | binary `xtask` | Resolve the workspace root for the audit subcommand | `xtask/src/main.rs:128-138` | High |
| OPT-018 | `tui-vfx-types::serde` | Cargo feature | bool | `default = ["serde"]` (on) | optional | crate `tui-vfx-types` | Enable serde derives on every foundation type; gates one `#[cfg(feature = "serde")]` impl in `interned_string.rs` | `crates/tui-vfx-types/Cargo.toml:26-28`; `crates/tui-vfx-types/src/interned_string.rs:114,121` | High |
| OPT-019 | `tui-vfx-geometry::default` | Cargo feature | bool | `default = []` (no extras) | optional | crate `tui-vfx-geometry` | Reserved feature slot — no behavioral effect at audit-time | `crates/tui-vfx-geometry/Cargo.toml:32-33` | High |
| OPT-020 | `tui-vfx-content::default` | Cargo feature | bool | `default = []` (no extras) | optional | crate `tui-vfx-content` | Reserved feature slot — no behavioral effect at audit-time | `crates/tui-vfx-content/Cargo.toml:36-37` | High |
| OPT-021 | `[profile.dev].opt-level` | Cargo profile | u8 | `2` | n/a | workspace | Build-time optimization for the workspace's own code | `Cargo.toml:72-73` | High |
| OPT-022 | `[profile.dev.package."*"].opt-level` | Cargo profile | u8 | `3` | n/a | workspace | Build-time optimization for all dependencies | `Cargo.toml:75-76` | High |
| OPT-023 | `clippy.toml::too-many-arguments-threshold` | Lint config | u32 | `9` | n/a | workspace | Raises clippy's `too_many_arguments` threshold; rationale comment cites Intention 40 §1 | `clippy.toml:1-8` | High |
| OPT-024 | `.cargo/config.toml::alias.xtask` | Cargo alias | string | `"run --package xtask --"` | n/a | workspace | Routes `cargo xtask` to the `xtask` binary | `.cargo/config.toml:6` | High |
| OPT-025 | `DEFAULT_FONT_SENTINEL` | Public constant | `&'static str` | `"default_font"` | n/a | crate `tui-vfx-content` | The reserved name that `FontRegistry::resolve` routes to the registered default | `crates/tui-vfx-content/src/fonts/cls_font_registry.rs:31` | High |
| OPT-026 | `DEFAULT_LOGO_SENTINEL` | Public constant | `&'static str` | `"default_logo"` | n/a | crate `tui-vfx-content` | The reserved name that `AssetRegistry` routes to the registered default | `crates/tui-vfx-content/src/assets/cls_asset_registry.rs:29` | High |
| OPT-027 | `BRAILLE_LEFT_COL` / `BRAILLE_RIGHT_COL` | Public constant | `&'static str` | `"⡇"` / `"⣸"` | n/a | crate `tui-vfx-content` | Cell glyphs used by the morph-chars helper for partial column reveals | `crates/tui-vfx-content/src/transformers/fnc_morph_chars.rs:112,119` | High |
| OPT-028 | `tui_vfx_types::braille::{LEFT_COLUMN, RIGHT_COLUMN, TOP_ROW, BOTTOM_ROW, UPPER_HALF, LOWER_HALF, ALL_DOTS}` | Public constants | `u8` (region masks) | varied bitmasks | n/a | crate `tui-vfx-types` | 2×4 dot-region bitmasks consumed by braille effects | `crates/tui-vfx-types/src/braille.rs:29,32,35,38,41,44,47` | High |
| OPT-029 | `tui_vfx_types::braille::{PATTERNS_1, PATTERNS_2}` | Public constants | `[u8; 8]` and `[u8; 28]` | hardcoded patterns | n/a | crate `tui-vfx-types` | Pre-computed braille bit patterns for one-dot and two-dot subsets | `crates/tui-vfx-types/src/braille.rs:138,141` | High |
| OPT-030 | `tui_vfx_types::glyph::SUBCELL_OFFSETS` | Public constant | `[(f32, f32); 8]` | the eight 2×4 sub-cell coordinates | n/a | crate `tui-vfx-types` | Sub-cell sample positions used by `fnc_sample_eight_subcells` | `crates/tui-vfx-types/src/glyph/fnc_sample_eight_subcells.rs:35` | High |
| OPT-031 | `VISUAL_CENTER_PERCENT` | Module-private constant | `u16` | `45` | n/a | `tui-vfx-geometry::anchors` | Y-coordinate of middle anchors as a percent of frame height (45% rather than 50% for legacy-notification parity) | `crates/tui-vfx-geometry/src/anchors/mod.rs:11-13` | High |
| OPT-032 | `CompositionSpec`'s `t`, `loop_t`, `phase` parameters | API parameter | `f64`, `f64`, lifecycle phase | per-spec construction | required | `tui-vfx-compositor` | Lifted by `render_pipeline_with_spec` into `CompositionPlaybackTiming::new(t, loop_t, phase)` | `crates/tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec.rs:46-50` | High |
| OPT-033 | `CompositionSpec.preserve_unfilled` | API parameter | `bool` | per-spec | optional | `tui-vfx-compositor` | Keep destination cells the source did not write to | `crates/tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec.rs:39` | High |
| OPT-034 | `CompositionOptions.shadow` and `.shadow_element_rect` | API parameter | `Option<ShadowSpec>` and `Option<Rect>` | `None` / `None` | optional | `tui-vfx-compositor` | Shadow stage opt-in + element rect for shadow extrusion | `crates/tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec.rs:37-38` | High |
| OPT-035 | `BoundaryMode` (Grid trait) | API parameter | enum | per-call | required | `tui-vfx-types` | Out-of-bounds read policy for `Grid` | `crates/tui-vfx-types/src/lib.rs:97` | High (presence) / Medium (variant list — needs read of `grid.rs`) |
| OPT-036 | `RoleTag::Custom(InternedRoleName)` | API parameter | enum variant | per-construction | optional | `tui-vfx-types` | Carries an `Arc<str>` custom role name beyond the 12 first-class roles | `crates/tui-vfx-types/src/lib.rs:24-28,107` | High |
| OPT-037 | `LogLevel`, `ModuleConfig` | API parameter | enum / struct | per-module | optional | `tui-vfx-debug` | Per-module debug-log verbosity | `crates/tui-vfx-debug/src/lib.rs:37` | High |
| OPT-038 | `TraceFilter`, `TraceSelector`, `StageMask` | API parameter | structs / bit mask | per-sink | optional | `tui-vfx-debug::inspection` | Sink-time filtering of `TraceEvent` by selector / stage / frame range / time range | `crates/tui-vfx-debug/src/inspection/mod.rs:11-19` | High |
| OPT-039 | `ProbeRequest` (constructed from CLI flags) | API parameter | struct | per-invocation | required (binary path) | `tui-vfx-probe` | Probe phase + sample-t + cell selector + with-causation, etc. | `crates/tui-vfx-probe/src/lib.rs:90`; `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:42-90` | High |
| OPT-040 | `RuntimeBindings` / `ShaderRuntimeParams` (host-supplied per-frame values) | API parameter | map of name → value | per-frame | optional | `tui-vfx-style::traits` (where `ShaderRuntimeParams` lives) | The `Binding(name)` arm of `VfxBindableValue` resolves names against this map | `crates/tui-vfx-content/src/lib.rs:53-90` (rustdoc); `crates/tui-vfx-style/src/traits/cls_shader_context.rs:215+` (per archived 69-A evidence) | High |
| OPT-041 | `RoleMap` (source roles passed to render_pipeline) | API parameter | dense per-cell role map | per-render | required | `tui-vfx-compositor` | Scope-by-role targeting in samplers/masks/shaders/filters | `crates/tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec.rs:22`; `crates/tui-vfx-types/src/lib.rs:105` | High |

## 5.2 Subsections

### 5.2.1 CLI options

The workspace has two CLI binaries: `xtask` and `pipeline-probe`. Their full flag surfaces are catalogued in the master table above (OPT-001..OPT-016). Notes:

- `xtask` uses `clap 4 derive` (`xtask/Cargo.toml:26`), so flag parsing is auto-generated from the `Cli` / `Commands` / `*Action` enums.
- `pipeline-probe` uses **manual `std::env::args` parsing** (`crates/tui-vfx-probe/src/bin/pipeline-probe.rs:35-90`), not `clap`. Default values are literal `let` initializers; flag handling is a `while let Some(arg) = args.next()` match. Error messages for missing values are in-source string literals (e.g., `"missing value for --format"`).
- `cargo xtask <...>` is the canonical invocation form per the alias at `.cargo/config.toml:6`. The `cargo xtask` entry point routes through `cargo run --package xtask --`.

### 5.2.2 Environment variables

| Variable | Read at | Purpose |
|---|---|---|
| `CARGO_MANIFEST_DIR` | `xtask/src/main.rs:130-134` | Resolves the workspace root (`std::env::var(...).ok().as_deref().unwrap_or(".")`); falls through to `.` when unset |

A workspace-wide search for `env::var` and `env::var_os` returned only the one xtask call site. No other production code path reads environment variables at audit-time.

### 5.2.3 Config file keys

No runtime configuration file is read by any binary. The three configuration files at the workspace root are build-time inputs:

- `Cargo.toml` — Cargo manifest (workspace + per-crate). 75 lines.
- `clippy.toml` — Clippy configuration. 8 lines. Sets `too-many-arguments-threshold = 9` (OPT-023).
- `.cargo/config.toml` — Cargo configuration. 6 lines. Defines the `xtask` alias (OPT-024).

No per-crate or per-binary config-file path was observed in the workspace. Recipe JSON (and any future asset bytes) are accepted as command-line inputs (`pipeline-probe --input`) or as in-process byte sources (Intention 27).

### 5.2.4 Cargo features

Three `[features]` blocks in the workspace; only one carries behavioral effect.

| Crate | Block | Effect |
|---|---|---|
| `tui-vfx-types` | `default = ["serde"]` / `serde = ["dep:serde"]` | Gates serde derives via `optional = true` on `serde` (`crates/tui-vfx-types/Cargo.toml:20`) and one `#[cfg(feature = "serde")]` block in `interned_string.rs:114,121`. |
| `tui-vfx-geometry` | `default = []` | No effect at audit-time. The block is reserved. |
| `tui-vfx-content` | `default = []` | No effect at audit-time. The block is reserved. |

(See chapter 2 §2.5 for the Cargo features inventory.)

### 5.2.5 Compile-time `cfg` gates

A repo-wide search for `#[cfg(...)]` outside `cfg(test)` returned only two lines, both in the foundation types crate:

- `crates/tui-vfx-types/src/interned_string.rs:114` and `:121` — `#[cfg(feature = "serde")]` gates two impl blocks.

No `cfg(target_os = ...)`, `cfg(target_arch = ...)`, `cfg(unix)`, `cfg(windows)`, or other platform-specific cfg gates were observed in production code at audit-time.

### 5.2.6 Public constants and tunables

Catalogued individually as OPT-025..OPT-031. Summary:

- `DEFAULT_FONT_SENTINEL` and `DEFAULT_LOGO_SENTINEL` — sentinel name routing for the font and asset registries.
- `BRAILLE_LEFT_COL`, `BRAILLE_RIGHT_COL` — cell glyphs for column reveals.
- `tui_vfx_types::braille::{LEFT_COLUMN, RIGHT_COLUMN, TOP_ROW, BOTTOM_ROW, UPPER_HALF, LOWER_HALF, ALL_DOTS, PATTERNS_1, PATTERNS_2}` — region masks and pre-computed pattern arrays.
- `SUBCELL_OFFSETS` — eight sub-cell coordinates for sub-cell sampling.
- `VISUAL_CENTER_PERCENT = 45` (module-private) — middle-anchor y-coordinate as a percent of frame height. Module-private but load-bearing for the legacy notification parity contract.

The clippy threshold (`too-many-arguments-threshold = 9`) is the only build-time lint tunable.

### 5.2.7 API parameters (high-level)

The chapter 3 feature subsections enumerate every public-API entry point. The configuration-relevant API parameters surface in OPT-032..OPT-041 above. Summary:

- The render-pipeline entry points (F001) take a closed set of arguments: `&dyn Grid`, `&RoleMap`, `&mut SemanticScene`, render rect, spec/options, optional inspector.
- `CompositionSpec` (F001) carries lifecycle timing (`t`, `loop_t`, `phase`) and shadow / preserve-unfilled / runtime-params toggles.
- `RuntimeBindings` / `ShaderRuntimeParams` (F011, F040) is the host-supplied per-frame option surface. Bindings resolved against this map populate the `Binding(name)` arm of `VfxBindableValue`.
- `RoleMap` + `RoleTag::Custom(...)` (F027) is the per-cell role surface that scope-by-role effects target.

### 5.2.8 Workflow recipes (`justfile`)

The `justfile` (330 lines) is the workspace's task runner. `just --list` enumerates every recipe. The audit-time recipe set:

| Recipe | Rough purpose | Source |
|---|---|---|
| `default` | Show the recipe list | `justfile:5` |
| `docs-generate` | `cargo xtask docs generate` | `justfile:26` |
| `docs-check` | `cargo xtask docs check` (CI) | `justfile:32` |
| `docs-ai-context` | `cargo xtask docs ai-context` | `justfile:37` |
| `docs-markdown` | `cargo xtask docs markdown` | `justfile:42` |
| `docs-rustdoc-json` | Generate rustdoc JSON | `justfile:48` |
| `docs-validate` | `cargo xtask docs validate` | `justfile:59` |
| `docs-scaffold[-write]` | `cargo xtask docs scaffold[-write]` | `justfile:64,69` |
| `docs-api[-...]` | `cargo xtask docs api[-...]` (six recipes: generate, check, validate, scaffold, scaffold-write, diff) | `justfile:93-118` |
| `docs-all[-check\|-validate]` | Run all doc subcommands | `justfile:127,133,139` |
| `build`, `build-release` | Cargo build wrappers | `justfile:150,154` |
| `test`, `test-verbose` | Cargo test wrappers | `justfile:158,162` |
| `lint` | Cargo clippy | `justfile:166` |
| `fmt`, `fmt-check` | Cargo fmt | `justfile:170,174` |
| `check-all` | `fmt-check lint test docs-all-check audit-all` | `justfile:178` |
| `doc`, `doc-open` | Cargo doc | `justfile:186,190` |
| `examples`, `example-pipeline-effects` | Run examples | `justfile:202,207` |
| `clean`, `update`, `outdated` | Cargo cleanup / dep updates | `justfile:215-223` |
| `dramatic-shadow-full-quality` | A specific demo invocation | `justfile:296` |
| `audit-configschema`, `audit-all` | `cargo xtask audit configschema` and aggregator | `justfile:313,318` |
| `ci` | `fmt-check lint test docs-all-check audit-all` | `justfile:326` |

The `justfile` is a thin delegation surface over `cargo xtask` and `cargo` directly; it does not introduce its own option flags.

## 5.3 Confidence

**High** for every catalogued option whose source is the workspace itself. The previously Medium-confidence OPT-015 (`pipeline-probe --widget-cell`'s grammar) was upgraded to High after the chapter-12 deepening pass read `pipeline-probe.rs:80-188` end-to-end (chapter 12 §12.13 records the resolution).

<!-- <FILE>pro/EXISTING-SYSTEM-PRD/05_options_and_configuration.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
