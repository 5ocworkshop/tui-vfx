<!-- <FILE>pro/EXISTING-SYSTEM-PRD/01_executive_summary.md</FILE> - <DESC>Chapter 1 of the evidence-backed Existing-System PRD: executive summary of tui-vfx — only verified facts about primary capabilities, major entry points, major crates, and major external dependencies.</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Mid-audit: a 13th workspace member (crates/tui-vfx-next) was added by the maintainer; this chapter and 02 are re-anchored against the new line numbers.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — adopt the new 13-crate workspace shape; tui-vfx-next added; every Cargo.toml line citation past line 16 shifts by +1; package.version is now :31 (was :30); workspace.dependencies block is now :39-65 (was :38-64). 0.1.1: PATCH — line citations verified. 0.1.0: initial population.</CLOG> -->

# 1. Executive Summary

This chapter records only what the workspace itself demonstrates. Behavioral claims are deferred to chapter 3; this chapter is a structural orientation.

## What the workspace contains

The workspace `tui-vfx` is a Cargo workspace of 13 members. Members are declared at `Cargo.toml:14-28` (the array list itself spans `15-27` plus the opening / closing brackets on `14` and `28`). Twelve members live under `crates/` and one (`xtask`) lives at the workspace root. The `[workspace.package]` block is at `Cargo.toml:30-37` and declares package version `0.11.0` (`:31`), edition `2024` (`:32`), MSRV `1.95.0` (`:33`), license `MIT` (`:34`), repository `https://github.com/5ocworkshop/tui-vfx` (`:35`), keywords `["tui", "terminal", "effects", "animation"]` (`:36`), and categories `["command-line-interface", "graphics"]` (`:37`). The 13th member, `crates/tui-vfx-next`, is a clean-room V3.1 surface-contract spike that landed during the audit.

The root `README.md:7-15` describes the workspace as `"gt-design-first scene rendering and VFX infrastructure for terminal UIs. Open, extensible, grid-first."` and as `"the Rust scene renderer, VFX compositor, and recipe-driven animation runtime behind gt-design's palette, theme, motion, animation, and visual-effects surfaces."` This README claim is recorded as documentation evidence (Medium confidence — consistent with the package descriptions in each crate's `Cargo.toml`, but not independently verified by behavioral tests in this chapter).

## Major crates (lexicographic order)

The package descriptions below are taken directly from each crate's `[package].description` field. They are doc-evidence of the maintainer's stated role for the crate; chapter 2 catalogues the targets and dependencies, and chapter 3 verifies the behaviors against code paths.

| Crate | `description` (verbatim from `Cargo.toml`) | Evidence |
|---|---|---|
| `tui-vfx` | "Cell-based visual effects for terminal UIs" | `crates/tui-vfx/Cargo.toml:9` |
| `tui-vfx-compositor` | "Framework-agnostic cell compositor for tui-vfx" | `crates/tui-vfx-compositor/Cargo.toml:9` |
| `tui-vfx-content` | "Text manipulation primitives for tui-vfx" | `crates/tui-vfx-content/Cargo.toml:9` |
| `tui-vfx-core` | "Schema and introspection primitives for the tui-vfx ecosystem" | `crates/tui-vfx-core/Cargo.toml:8` |
| `tui-vfx-core-macros` | "Proc-macro derives for tui-vfx ConfigSchema" | `crates/tui-vfx-core-macros/Cargo.toml:8` |
| `tui-vfx-debug` | "Centralized debug logger + unified inspection foundation for tui-vfx ecosystem" | `crates/tui-vfx-debug/Cargo.toml:9` |
| `tui-vfx-geometry` | "Pure math, layout, and motion primitives for tui-vfx" | `crates/tui-vfx-geometry/Cargo.toml:8` |
| `tui-vfx-next` | "Clean-room v3.1 surface contract spike for tui-vfx" | `crates/tui-vfx-next/Cargo.toml:5` |
| `tui-vfx-probe` | "Engine-owned structured pipeline observability for tui-vfx" | `crates/tui-vfx-probe/Cargo.toml:8` |
| `tui-vfx-shadow` | "Shadow rendering effects for TUI applications" | `crates/tui-vfx-shadow/Cargo.toml:8` |
| `tui-vfx-style` | "Color interpolation and style effects for tui-vfx" | `crates/tui-vfx-style/Cargo.toml:8` |
| `tui-vfx-types` | "Framework-agnostic foundation types for the tui-vfx ecosystem" | `crates/tui-vfx-types/Cargo.toml:8` |
| `xtask` | (no `[package].description` populated; the file's `<DESC>` header at line 1 reads "Build tooling crate for documentation generation and CI tasks") | `xtask/Cargo.toml:1` (header), `xtask/Cargo.toml:6-11` ([package] block — confirms no `description = ` line) |

## Major entry points

- **Library entry points.** Every workspace member except `xtask` declares a `src/lib.rs` (verified by direct filesystem check on each `crates/*/src/`). `xtask` declares both a `src/main.rs` binary and a `src/lib.rs` library named `xtask_audit_configschema` (`xtask/Cargo.toml:13-19`). The new `tui-vfx-next` crate has `src/lib.rs` plus six sibling source files (`diagnostic.rs`, `effect.rs`, `engine.rs`, `scope.rs`, `surface.rs`, `write.rs`).
- **Binary entry points.**
  - `xtask`: bin `xtask` at `xtask/src/main.rs` (`xtask/Cargo.toml:13-15`).
  - `tui-vfx-probe`: bin `pipeline-probe` at `crates/tui-vfx-probe/src/bin/pipeline-probe.rs` (`crates/tui-vfx-probe/Cargo.toml:29-31`).
  - No other workspace member declares a `[[bin]]` target or carries a `src/main.rs`.
- **Example targets.** Two examples are registered against the meta-crate `tui-vfx`, both pointing at `examples/` at the workspace root: `pipeline_effects_showcase` (`crates/tui-vfx/Cargo.toml:32-34`) and `direct_api_signal_strength` (`crates/tui-vfx/Cargo.toml:36-38`). The directory listing of `examples/` shows both `.rs` files plus a `README.md`.
- **Bench targets.** Three criterion benches are declared in two crates: `tui-vfx-geometry::easing` (`crates/tui-vfx-geometry/Cargo.toml:35-37`), `tui-vfx-debug::bench_emit_overhead` (`crates/tui-vfx-debug/Cargo.toml:30-32`), and `tui-vfx-debug::bench_full_trace_60fps` (`crates/tui-vfx-debug/Cargo.toml:34-36`).
- **CLI surface.** A workspace-level cargo alias `xtask = "run --package xtask --"` is declared at `.cargo/config.toml:6`. It exposes the `xtask` binary as `cargo xtask <command>`. The full xtask subcommand surface is enumerated in chapter 5.

## Major external dependencies

The `[workspace.dependencies]` table at `Cargo.toml:39-65` aggregates the externally hosted crates the workspace depends on. Per-crate `[dependencies]` and `[dev-dependencies]` add a small number of additional crates not lifted to workspace scope (chapter 2 enumerates the per-crate matrix). The externally hosted dependencies surfaced at the workspace level are:

| Dependency | Version | Source | Purpose (per the in-tree comment) | Evidence |
|---|---|---|---|---|
| `mixed-signals` | `0.3.0` | path `../mixed-signals` | Sibling repo — signal primitives consumed by every crate that needs `Signal`/`SignalContext`. | `Cargo.toml:52` |
| `mcu-hct` | `0.2.0` | crates.io | "HCT (Hue, Chroma, Tone) perceptual color space" used by `tui-vfx-style`. | `Cargo.toml:53-58` (block-comment + dep line) |
| `mcu-utils` | `0.2.0` | crates.io | Companion to `mcu-hct`. | `Cargo.toml:59` |
| `serde` | `1.0` | crates.io | Serialization, with `derive` feature on. | `Cargo.toml:60` |
| `serde_json` | `1.0` | crates.io | JSON. | `Cargo.toml:61` |
| `smallvec` | `1.11` | crates.io | Inline allocation for ≤N effects (per the inline comment). | `Cargo.toml:62` |
| `rocketsplash-rt` | `0.2.2` | crates.io (sister project — see `Cargo.toml:64` comment pointing to `docs/internal/plans/splash-library-and-vfx-integration.md`) | Splash/image format consumer used by `tui-vfx-content`. | `Cargo.toml:64-65` |

Per-crate-only dependencies (not lifted to workspace scope) include: `bitflags 2.4` (`tui-vfx-shadow`), `colored 2.1` and `chrono 0.4` and `lazy_static 1.4` (`tui-vfx-debug`; `lazy_static` also in `tui-vfx-compositor`), `rusqlite 0.32` with `bundled` feature (`tui-vfx-probe`), `unicode-segmentation 1.10` (`tui-vfx-content`), `proc-macro2 1` / `quote 1` / `syn 2` (`tui-vfx-core-macros`), and the dev-only crates `approx 0.5`, `criterion 0.5`, `rstest 0.18`, `tempfile 3`. The `xtask` binary additionally pulls in `clap 4`, `toml 0.8`, `walkdir 2`, `anyhow 1`, `owo-colors 4`. Chapter 2 records the full per-crate dep matrix with citations.

## Workspace-wide configuration

- **Edition + MSRV.** Edition `2024` (`Cargo.toml:32`), MSRV `1.95.0` (`Cargo.toml:33`). The `xtask` package overrides the workspace MSRV to `1.86.0` and pins its own version to `0.1.0` with `publish = false` (`xtask/Cargo.toml:8-11`).
- **Resolver.** `resolver = "3"` (`Cargo.toml:13`).
- **Build profiles.** `[profile.dev]` at `Cargo.toml:72-73` sets `opt-level = 2`, and `[profile.dev.package."*"]` at `Cargo.toml:75-76` sets `opt-level = 3`. The inline comment at `Cargo.toml:69-71` states the rationale: "Example/demo workloads need meaningful optimization in dev too; otherwise local cargo run / cargo test significantly under-represents real runtime performance."
- **Clippy.** `clippy.toml:8` sets `too-many-arguments-threshold = 9` with a comment block at `clippy.toml:1-7` citing Intention 40 §1; the file is 8 lines total.
- **Cargo aliases.** `.cargo/config.toml:6` defines `xtask = "run --package xtask --"`. The file is 6 lines total.
- **No `rust-toolchain.toml`** at the workspace root (filesystem check returned absent).
- **No `rustfmt.toml`** at the workspace root (filesystem check returned absent).
- **No `.github/workflows/`** directory (filesystem check returned absent — i.e., no GitHub Actions CI configuration is checked in).
- **No `build.rs`** in any crate (`find . -maxdepth 4 -name build.rs -not -path './target/*'` returned no matches).
- **`justfile`** at the workspace root is 330 lines (`wc -l justfile`); chapter 5 enumerates its recipes.

## Primary capabilities

The workspace's externally meaningful capabilities are catalogued in detail in chapter 3. At this orientation level, the structural evidence for capability families is:

1. **A grid-first cell compositor.** `tui-vfx-compositor` (32 624 LOC across 128 files per `ofpf-sql`'s `files.lines` aggregation; see chapter 14) declares a public surface in `crates/tui-vfx-compositor/src/lib.rs` and depends on `tui-vfx-types`, `tui-vfx-style`, `tui-vfx-shadow`, `mixed-signals`, `smallvec`, `serde_json`, `lazy_static`, `tui-vfx-debug`, `tui-vfx-core`, and `tui-vfx-geometry` (`crates/tui-vfx-compositor/Cargo.toml:19-39`).
2. **Foundation cell/color/style primitives.** `tui-vfx-types` carries the cross-crate types (`tui-vfx-types/lib.rs` is the workspace's highest-fan-in file at 151 incoming logic edges per `ofpf-orientation`). Optional `serde` integration is gated by the `serde` Cargo feature, on by default (`crates/tui-vfx-types/Cargo.toml:26-28`).
3. **Schema and ConfigSchema introspection.** `tui-vfx-core` provides schema primitives consumed by every effect-bearing crate; `tui-vfx-core-macros` is a `proc-macro = true` crate (`crates/tui-vfx-core-macros/Cargo.toml:18-19`) that emits `ConfigSchema` derives.
4. **Style/color effects.** `tui-vfx-style` depends on `mcu-hct` + `mcu-utils` for the HCT perceptual color space (`crates/tui-vfx-style/Cargo.toml:33-34`).
5. **Geometry / motion / easing.** `tui-vfx-geometry` consumes `mixed-signals` for "oscillating paths" (the inline comment at `crates/tui-vfx-geometry/Cargo.toml:24-25`) and ships a criterion bench `easing` (`crates/tui-vfx-geometry/Cargo.toml:35-37`).
6. **Content transformers (text / braille image / fonts).** `tui-vfx-content` depends on `unicode-segmentation`, `rocketsplash-rt`, `mixed-signals`, and the workspace style/geometry/types crates (`crates/tui-vfx-content/Cargo.toml:19-31`).
7. **Shadow rendering.** `tui-vfx-shadow` ships as its own crate, depending only on `tui-vfx-types`, `tui-vfx-core`, `serde`, and `bitflags` (`crates/tui-vfx-shadow/Cargo.toml:18-22`).
8. **Centralized debug logging + inspection foundation.** `tui-vfx-debug` (`crates/tui-vfx-debug/Cargo.toml:9`) ships two criterion benches that target a "60 fps" budget (`bench_full_trace_60fps` at `crates/tui-vfx-debug/Cargo.toml:34-36`).
9. **Pipeline observability binary.** `tui-vfx-probe` produces the `pipeline-probe` binary (`crates/tui-vfx-probe/Cargo.toml:29-31`) and is the only workspace member that depends on `rusqlite` (`crates/tui-vfx-probe/Cargo.toml:25`).
10. **Build/doc tooling.** `xtask` is `publish = false` and uses `clap` + `toml` + `walkdir` + `anyhow` + `owo-colors`; it depends on six effect crates plus `mixed-signals` "for metadata extraction" (`xtask/Cargo.toml:24-51`, with the per-crate paths at `:45-51`).

## External integration boundaries (path dependencies)

Two path dependencies cross out of the workspace:

- `mixed-signals` at `../mixed-signals`, version `0.3.0` (`Cargo.toml:52`). Consumed by `tui-vfx-types`, `tui-vfx-core`, `tui-vfx-geometry`, `tui-vfx-compositor`, `tui-vfx-style`, `tui-vfx-content`, `tui-vfx-probe`, and `xtask` (per their respective `[dependencies]` blocks). `tui-vfx-next` does **not** depend on `mixed-signals` at audit-time.
- The `xtask` crate's `mixed-signals` reference uses an explicit relative `path = "../../mixed-signals"` literal (`xtask/Cargo.toml:51`) rather than `workspace = true`. The other consumers use `workspace = true`.

The `[workspace.dependencies]` `rocketsplash-rt = "0.2.2"` entry (`Cargo.toml:65`) targets crates.io rather than a sibling path; its only consumer is `tui-vfx-content` (`crates/tui-vfx-content/Cargo.toml:31`).

## Confidence

Overall confidence for this chapter is **High** for structural facts (crate count, package names and versions, target presence, dependency declarations, MSRV/edition/license/repository) — they are taken directly from the workspace manifests and verified against `cargo metadata --no-deps`. Confidence is **Medium** for capability-family claims, which use the package `description` fields and dependency shapes as evidence; chapter 3 escalates per-feature confidence to High where code paths and tests both demonstrate the behavior.

<!-- <FILE>pro/EXISTING-SYSTEM-PRD/01_executive_summary.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.1</VERS> -->
