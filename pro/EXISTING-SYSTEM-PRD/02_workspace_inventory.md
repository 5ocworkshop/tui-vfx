<!-- <FILE>pro/EXISTING-SYSTEM-PRD/02_workspace_inventory.md</FILE> - <DESC>Chapter 2 of the evidence-backed Existing-System PRD: deterministic crate inventory — package metadata, targets, Cargo features, dependency relationships, and build-script presence for every workspace member.</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Mid-audit: a 13th workspace member (crates/tui-vfx-next) was added; chapter re-anchored against the new line numbers and the new crate is added to the inventory.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — adopt 13-crate workspace shape; tui-vfx-next added to crate table and dep matrix; every Cargo.toml line citation past line 16 shifts by +1 (resolver still :13; members block now :14-28; [workspace.package] :30-37; [workspace.dependencies] header :39; mixed-signals :52; rocketsplash-rt :65; profile.dev :72-73; profile.dev.package."*" :75-76). 0.1.1: PATCH — line citations rebuilt from a programmatic scan. 0.1.0: initial.</CLOG> -->

# 2. Repository and Workspace Inventory

This chapter is deterministic by construction. Crates are ordered lexicographically by package name. Within each crate, evidence is cited as `path:line` ranges against the file at the audit-time SHA (`519a6c67`) recorded in chapter 0.

## 2.1 Workspace declaration

The workspace is declared at `Cargo.toml:12-28` (block at `:12`, `resolver` at `:13`, `members` array at `:14-28` with the closing `]` on `:28`). Members are listed at `Cargo.toml:15-27` in the order recorded by the maintainer:

```
crates/tui-vfx              (line 15)
crates/tui-vfx-types        (line 16)
crates/tui-vfx-next         (line 17)   ← V3.1 surface-contract spike, added mid-audit
crates/tui-vfx-core         (line 18)
crates/tui-vfx-core-macros  (line 19)
crates/tui-vfx-geometry     (line 20)
crates/tui-vfx-compositor   (line 21)
crates/tui-vfx-style        (line 22)
crates/tui-vfx-content      (line 23)
crates/tui-vfx-shadow       (line 24)
crates/tui-vfx-debug        (line 25)
crates/tui-vfx-probe        (line 26)
xtask                       (line 27)
```

Workspace member count: 13 — verified against `cargo metadata --no-deps --format-version=1 | jq '.workspace_members | length'` which reports `13`.

`[workspace.package]` block at `Cargo.toml:30-37`:

| Key | Value | Line |
|---|---|---|
| `version` | `0.11.0` | `:31` |
| `edition` | `2024` | `:32` |
| `rust-version` | `1.95.0` | `:33` |
| `license` | `MIT` | `:34` |
| `repository` | `https://github.com/5ocworkshop/tui-vfx` | `:35` |
| `keywords` | `["tui", "terminal", "effects", "animation"]` | `:36` |
| `categories` | `["command-line-interface", "graphics"]` | `:37` |

`[workspace.dependencies]` block header at `Cargo.toml:39`. Internal-crate path entries at `:41-49` (each pinned to `version = "0.11.0"` and `path = "crates/<name>"`). External crates at `:52` (`mixed-signals`), `:58-59` (`mcu-hct`, `mcu-utils`), `:60-62` (`serde`, `serde_json`, `smallvec`), `:65` (`rocketsplash-rt`). Note: `tui-vfx-next` is **not** lifted into `[workspace.dependencies]` at audit-time; only its own `[package]` references siblings via `tui-vfx-types.workspace = true` and `tui-vfx-geometry.workspace = true`.

`resolver = "3"` at `Cargo.toml:13`.

## 2.2 Crate table

Listed in lexicographic order. Columns: package name; path; type (lib / bin / proc-macro / build helper); whether each crate ships `src/lib.rs`, a `src/main.rs`, a `src/bin/` directory, an `examples/` directory, a `tests/` directory, or a `benches/` directory; whether a `build.rs` is present; the count of registered `[[bin]]`, `[[example]]`, and `[[bench]]` blocks declared in the crate's `Cargo.toml`. Counts of `tests/*.rs` and `benches/*.rs` reflect filesystem presence at audit-time.

| Package | Path | Type | `lib.rs` | `main.rs` | `src/bin/` | `examples/` | `tests/*.rs` | `benches/*.rs` | `build.rs` | `[[bin]]` | `[[example]]` | `[[bench]]` |
|---|---|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| `tui-vfx` | `crates/tui-vfx` | lib + examples (meta-crate) | Y | N | 0 | 0¹ | 2 | 0 | N | 0 | 2² | 0 |
| `tui-vfx-compositor` | `crates/tui-vfx-compositor` | lib | Y | N | 0 | 0 | 13 | 0 | N | 0 | 0 | 0 |
| `tui-vfx-content` | `crates/tui-vfx-content` | lib | Y | N | 0 | 0 | 7 | 0 | N | 0 | 0 | 0 |
| `tui-vfx-core` | `crates/tui-vfx-core` | lib | Y | N | 0 | 0 | 7 | 0 | N | 0 | 0 | 0 |
| `tui-vfx-core-macros` | `crates/tui-vfx-core-macros` | proc-macro | Y³ | N | 0 | 0 | 0 | 0 | N | 0 | 0 | 0 |
| `tui-vfx-debug` | `crates/tui-vfx-debug` | lib | Y | N | 0 | 0 | 8 | 2 | N | 0 | 0 | 2⁴ |
| `tui-vfx-geometry` | `crates/tui-vfx-geometry` | lib | Y | N | 0 | 0 | 15 | 1 | N | 0 | 0 | 1⁵ |
| `tui-vfx-next` | `crates/tui-vfx-next` | lib | Y | N | 0 | 0 | 1⁹ | 0 | N | 0 | 0 | 0 |
| `tui-vfx-probe` | `crates/tui-vfx-probe` | lib + bin | Y | N | 1⁶ | 0 | 6 | 0 | N | 1⁶ | 0 | 0 |
| `tui-vfx-shadow` | `crates/tui-vfx-shadow` | lib | Y | N | 0 | 0 | 3 | 0 | N | 0 | 0 | 0 |
| `tui-vfx-style` | `crates/tui-vfx-style` | lib | Y | N | 0 | 0 | 3 | 0 | N | 0 | 0 | 0 |
| `tui-vfx-types` | `crates/tui-vfx-types` | lib | Y | N | 0 | 0 | 9 | 0 | N | 0 | 0 | 0 |
| `xtask` | `xtask` | bin + lib (`publish = false`) | Y⁷ | Y | 0 | 0 | 1 | 0 | N | 1⁸ | 0 | 0 |

Footnotes:

¹ The meta-crate has no `examples/` directory of its own; the two registered `[[example]]` blocks point at workspace-root `examples/`.
² `[[example]]` blocks at `crates/tui-vfx/Cargo.toml:32-38`: `pipeline_effects_showcase` (block at `:32-34`, path `../../examples/pipeline_effects_showcase.rs` at `:34`) and `direct_api_signal_strength` (block at `:36-38`, path `../../examples/direct_api_signal_strength.rs` at `:38`).
³ `crates/tui-vfx-core-macros/Cargo.toml:17-19` declares `[lib]` (`:17`) with `proc-macro = true` (`:19`). The crate has no `tests/` or `benches/` directory at audit-time.
⁴ `[[bench]]` blocks at `crates/tui-vfx-debug/Cargo.toml:30-36`: `bench_emit_overhead` (`:30-32`, `harness = false` at `:32`) and `bench_full_trace_60fps` (`:34-36`, `harness = false` at `:36`).
⁵ `[[bench]]` block at `crates/tui-vfx-geometry/Cargo.toml:35-37`: `easing` (`:35-37`, `harness = false` at `:37`).
⁶ `[[bin]]` block at `crates/tui-vfx-probe/Cargo.toml:29-31`: `pipeline-probe` (path `src/bin/pipeline-probe.rs`).
⁷ `xtask/Cargo.toml:17-19` declares `[lib] name = "xtask_audit_configschema"` (`:18`, path `src/lib.rs` at `:19`).
⁸ `xtask/Cargo.toml:13-15` declares `[[bin]] name = "xtask"` (`:14`, path `src/main.rs` at `:15`).
⁹ `crates/tui-vfx-next/tests/surface_contract.rs` is the only test file in the new spike crate.

## 2.3 Build scripts

A repo-wide search (`find . -maxdepth 4 -name build.rs -not -path './target/*'`) returned **no `build.rs` files**. No workspace member uses a build script.

## 2.4 Dependency relationships

### 2.4.1 Internal (within-workspace) edges

Per the `[dependencies]` and `[dev-dependencies]` blocks across all twelve manifests. `D` denotes a runtime `[dependencies]` edge; `d` denotes a `[dev-dependencies]` edge; `—` denotes no edge.

| from \\ to | `compositor` | `content` | `core` | `core-macros` | `debug` | `geometry` | `next` | `probe` | `shadow` | `style` | `types` |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| `tui-vfx` (meta) | D | D | D | — | d | D | — | — | D | D | D |
| `tui-vfx-compositor` | — | d | D | — | D | D | — | — | D | D | D |
| `tui-vfx-content` | — | — | D | — | — | D | — | — | — | D | D |
| `tui-vfx-core` | — | — | — | D | — | — | — | — | — | — | — |
| `tui-vfx-core-macros` | — | — | — | — | — | — | — | — | — | — | — |
| `tui-vfx-debug` | — | — | — | — | — | — | — | — | — | — | D |
| `tui-vfx-geometry` | — | — | D | — | — | — | — | — | — | — | D |
| `tui-vfx-next` | — | — | — | — | — | D | — | — | — | — | D |
| `tui-vfx-probe` | D | — | — | — | — | — | — | — | — | D | D |
| `tui-vfx-shadow` | — | — | D | — | — | — | — | — | — | — | D |
| `tui-vfx-style` | — | — | D | — | — | D | — | — | — | — | D |
| `tui-vfx-types` | — | — | D | — | — | — | — | — | — | — | — |
| `xtask` | D | D | D | — | — | — | — | — | D | D | D |

Sources for each row (the `[dependencies]` and where present `[dev-dependencies]` blocks of the named crate):

- `crates/tui-vfx/Cargo.toml:19-30` ([dependencies] `:19-26`, [dev-dependencies] `:28-30`)
- `crates/tui-vfx-compositor/Cargo.toml:19-47` ([dependencies] `:19-39`, [dev-dependencies] `:41-47`)
- `crates/tui-vfx-content/Cargo.toml:19-34` ([dependencies] `:19-31`, [dev-dependencies] `:33-34`)
- `crates/tui-vfx-core/Cargo.toml:18-22` ([dependencies] only)
- `crates/tui-vfx-core-macros/Cargo.toml:21-24` ([dependencies] only — no internal-crate deps)
- `crates/tui-vfx-debug/Cargo.toml:19-28` ([dependencies] `:19-25`, [dev-dependencies] `:27-28`)
- `crates/tui-vfx-geometry/Cargo.toml:18-30` ([dependencies] `:18-25`, [dev-dependencies] `:27-30`)
- `crates/tui-vfx-next/Cargo.toml:14-16` ([dependencies] only — two internal-crate workspace deps: `tui-vfx-types` at `:15`, `tui-vfx-geometry` at `:16`)
- `crates/tui-vfx-probe/Cargo.toml:18-27` ([dependencies] `:18-25`, [dev-dependencies] header at `:27` is empty)
- `crates/tui-vfx-shadow/Cargo.toml:18-25` ([dependencies] `:18-22`, [dev-dependencies] `:24-25`)
- `crates/tui-vfx-style/Cargo.toml:18-38` ([dependencies] `:18-34`, [dev-dependencies] `:36-38`)
- `crates/tui-vfx-types/Cargo.toml:18-24` ([dependencies] `:18-21`, [dev-dependencies] `:23-24`)
- `xtask/Cargo.toml:21-51` ([dev-dependencies] `:21-22`, [dependencies] `:24-51`)

Notable structural facts about the dep graph:

- `tui-vfx-core-macros` has no internal-crate dependencies (`crates/tui-vfx-core-macros/Cargo.toml:21-24` lists only `proc-macro2` `:22`, `quote` `:23`, `syn` `:24`).
- `tui-vfx-types` carries one internal-crate path dep: `tui-vfx-core` (`crates/tui-vfx-types/Cargo.toml:21`); the rest of its `[dependencies]` block is `mixed-signals` (`:19`) and optional `serde` (`:20`).
- `tui-vfx-core` depends only on `tui-vfx-core-macros` among the workspace siblings (`crates/tui-vfx-core/Cargo.toml:19`); the rest is `mixed-signals` `:20`, `serde` `:21`, `serde_json` `:22`.
- `tui-vfx-next` is intentionally narrow: it depends only on `tui-vfx-types` and `tui-vfx-geometry` (`crates/tui-vfx-next/Cargo.toml:15-16`). The crate's own `<DESC>` block at `:1` flags it as a "Clean-room v3.1 surface contract spike crate", and its module-level rustdoc at `crates/tui-vfx-next/src/lib.rs:3-9` explicitly states the crate "proves the Phase A semantic surface rules without depending on the legacy compositor, style, content, or shadow implementation crates."
- `tui-vfx` (meta) is the only crate that depends on every effect-bearing sibling at runtime (`crates/tui-vfx/Cargo.toml:19-26`). It does not depend on `tui-vfx-debug` at runtime (dev-dep at `:29`); it also does not depend on `tui-vfx-next` (the spike crate is not yet wired into the meta-crate's umbrella).
- `tui-vfx-probe` is the only crate that depends on `tui-vfx-compositor` (`crates/tui-vfx-probe/Cargo.toml:19`).
- `tui-vfx-compositor` depends on `tui-vfx-debug` at runtime (`crates/tui-vfx-compositor/Cargo.toml:38`), so debug-logging emit sites are part of the compositor's public render path.

### 2.4.2 Internal cycles

`ofpf-cycles --exclude-roles re_export` reports cycles within file granularity, not at crate granularity. The crate-level dep graph has no cycles by construction (Cargo enforces an acyclic crate graph at build time; `cargo metadata --no-deps` succeeded at audit start without reporting a cycle).

### 2.4.3 External (out-of-workspace) edges

Workspace-lifted deps live in `[workspace.dependencies]` (`Cargo.toml:39-65`). Per-crate-only externals live in each crate's `[dependencies]` and `[dev-dependencies]` blocks. The full per-crate matrix:

| Crate | Runtime-only externals (lex order) | Dev-only externals (lex order) | Evidence |
|---|---|---|---|
| `tui-vfx` | `tui-vfx-*` workspace siblings only | `mixed-signals` (workspace), `tui-vfx-debug` (workspace) | `crates/tui-vfx/Cargo.toml:19-30` |
| `tui-vfx-compositor` | `lazy_static 1.4`, `mixed-signals` (workspace), `serde_json 1.0` (per-crate), `serde` (workspace), `smallvec` (workspace), `tui-vfx-{types, core, style, geometry, shadow, debug}` (workspace) | `mixed-signals` (workspace), `tui-vfx-content` (workspace) | `crates/tui-vfx-compositor/Cargo.toml:19-47` |
| `tui-vfx-content` | `mixed-signals` (workspace), `rocketsplash-rt` (workspace), `serde` (workspace), `tui-vfx-{core, style, types, geometry}` (workspace), `unicode-segmentation 1.10` | `serde_json` (workspace) | `crates/tui-vfx-content/Cargo.toml:19-34` |
| `tui-vfx-core` | `mixed-signals` (workspace), `serde` (workspace), `serde_json` (workspace), `tui-vfx-core-macros` (workspace) | (none) | `crates/tui-vfx-core/Cargo.toml:18-22` |
| `tui-vfx-core-macros` | `proc-macro2 1`, `quote 1`, `syn 2 features=["full","extra-traits"]` | (none) | `crates/tui-vfx-core-macros/Cargo.toml:21-24` |
| `tui-vfx-debug` | `chrono 0.4`, `colored 2.1`, `lazy_static 1.4`, `serde 1.0 features=["derive"]` (per-crate, not workspace), `serde_json 1.0` (per-crate), `tui-vfx-types` (workspace) | `criterion 0.5 features=["html_reports"]` | `crates/tui-vfx-debug/Cargo.toml:19-28` |
| `tui-vfx-geometry` | `mixed-signals` (workspace), `serde` (workspace), `tui-vfx-{core, types}` (workspace) | `approx 0.5`, `criterion 0.5 features=["html_reports"]`, `serde_json 1.0` | `crates/tui-vfx-geometry/Cargo.toml:18-30` |
| `tui-vfx-next` | `tui-vfx-{geometry, types}` (workspace) — only two internal deps; **no** `mixed-signals`, **no** `serde`, **no** `tui-vfx-core` | (none) | `crates/tui-vfx-next/Cargo.toml:14-16` |
| `tui-vfx-probe` | `mixed-signals` (workspace), `rusqlite 0.32 features=["bundled"]`, `serde` (workspace), `serde_json` (workspace), `tui-vfx-{compositor, style, types}` (workspace) | (none) | `crates/tui-vfx-probe/Cargo.toml:18-27` |
| `tui-vfx-shadow` | `bitflags 2.4 features=["serde"]`, `serde` (workspace), `tui-vfx-{core, types}` (workspace) | `serde_json` (workspace) | `crates/tui-vfx-shadow/Cargo.toml:18-25` |
| `tui-vfx-style` | `mcu-hct` (workspace), `mcu-utils` (workspace), `mixed-signals` (workspace), `serde` (workspace), `serde_json` (workspace), `tui-vfx-{core, geometry, types}` (workspace) | `rstest 0.18` | `crates/tui-vfx-style/Cargo.toml:18-38` |
| `tui-vfx-types` | `mixed-signals` (workspace), `serde 1.0 features=["derive"] optional=true`, `tui-vfx-core` (path) | `serde_json 1.0` | `crates/tui-vfx-types/Cargo.toml:18-24` |
| `xtask` | `anyhow 1`, `clap 4 features=["derive"]`, `mixed-signals` (path `../../mixed-signals`), `owo-colors 4`, `serde 1.0 features=["derive"]`, `serde_json 1.0`, `toml 0.8`, `tui-vfx-{compositor, content, core, shadow, style, types}` (path), `walkdir 2` | `tempfile 3` | `xtask/Cargo.toml:21-51` |

Two notes on this table:

- `tui-vfx-debug`'s `serde 1.0 features=["derive"]` and `serde_json 1.0` are pinned **per-crate** at `crates/tui-vfx-debug/Cargo.toml:23-24`, not via `serde.workspace = true`. Confidence: High (read directly).
- `xtask`'s deps use explicit `path = "..."` rather than `workspace = true` (`xtask/Cargo.toml:45-51`) — including `mixed-signals = { path = "../../mixed-signals" }` at `:51`. Confidence: High.

## 2.5 Cargo features

Three crates declare a `[features]` block. Nine crates do not.

| Crate | `[features]` block | Default features | Defined features | Evidence |
|---|---|---|---|---|
| `tui-vfx-types` | yes | `["serde"]` | `serde = ["dep:serde"]` | `crates/tui-vfx-types/Cargo.toml:26-28` |
| `tui-vfx-geometry` | yes | `[]` | (none beyond `default = []`) | `crates/tui-vfx-geometry/Cargo.toml:32-33` |
| `tui-vfx-content` | yes | `[]` | (none beyond `default = []`) | `crates/tui-vfx-content/Cargo.toml:36-37` |
| `tui-vfx`, `tui-vfx-compositor`, `tui-vfx-core`, `tui-vfx-core-macros`, `tui-vfx-debug`, `tui-vfx-next`, `tui-vfx-probe`, `tui-vfx-shadow`, `tui-vfx-style`, `xtask` | absent | n/a | n/a | filesystem inspection of each `Cargo.toml` (no `[features]` heading present) |

Only one feature gate has a behavioral effect: `tui-vfx-types`'s optional `dep:serde`. The other two `[features]` blocks declare `default = []` with no additional named features (status: present-but-empty).

Cargo features are catalogued separately from product features; product features appear in chapter 3.

## 2.6 Build profiles

`Cargo.toml:69-76` defines two profile overrides:

| Profile | Setting | Value | Lines | Comment evidence |
|---|---|---|---|---|
| `[profile.dev]` | `opt-level` | `2` | block `:72-73`, value `:73` | Inline comment at `Cargo.toml:69-71` states the rationale: "Example/demo workloads need meaningful optimization in dev too; otherwise local cargo run / cargo test significantly under-represents real runtime performance." |
| `[profile.dev.package."*"]` | `opt-level` | `3` | block `:75-76`, value `:76` | (no inline comment) |

No `[profile.release]`, `[profile.test]`, or `[profile.bench]` overrides are declared at the workspace root.

## 2.7 Toolchain configuration

| File | Present? | Effective fact | Evidence |
|---|:-:|---|---|
| `rust-toolchain.toml` | N | The workspace does not pin a toolchain. CI/local toolchain selection is not constrained beyond MSRV `1.95.0` for the workspace and `1.86.0` for `xtask`. | `Cargo.toml:33`, `xtask/Cargo.toml:10` |
| `rustfmt.toml` | N | The workspace does not override rustfmt defaults. | filesystem absence |
| `clippy.toml` | Y (8 lines) | Sets `too-many-arguments-threshold = 9` with a comment block at lines 1-7 citing project Intention 40 §1. | `clippy.toml:1-8` |
| `.cargo/config.toml` | Y (6 lines) | Defines a single alias: `xtask = "run --package xtask --"`. | `.cargo/config.toml:1-6` (the `[alias]` heading + the single line `xtask = ...` at `:6`) |

## 2.8 Workspace-root assets

Files at the workspace root that are not Cargo manifests, and whose presence is structurally meaningful for chapters 3-14:

| File | Lines | Role (per its own `<DESC>` header) | Evidence |
|---|---:|---|---|
| `README.md` | 253 | "Project overview and usage guide" | `README.md:1` |
| `CHANGELOG.md` | (not counted) | "Release history for tui-vfx" | `CHANGELOG.md:1` |
| `justfile` | 330 | Workflow recipes (chapter 5 enumerates) | `justfile:1-4` (the `# tui-vfx justfile` comment block) |
| `LICENSE` | (not inspected) | License text — implied by `[workspace.package].license = "MIT"` | `Cargo.toml:34` |
| `examples/pipeline_effects_showcase.rs` | (not counted in this chapter) | Registered example target | `crates/tui-vfx/Cargo.toml:32-34` |
| `examples/direct_api_signal_strength.rs` | (not counted in this chapter) | Registered example target | `crates/tui-vfx/Cargo.toml:36-38` |
| `examples/README.md` | (not counted) | Examples directory readme | filesystem listing |

The `pro/` directory (input `REVERSE-PRD.md` and the chaptered output of this audit) and the `steering/` and `docs/` directories are referenced where applicable in later chapters; they are not workspace members.

## 2.9 CI configuration

`.github/workflows/` is not present at the workspace root (filesystem check returned absent). No other CI configuration directory (`.gitlab-ci.yml`, `.circleci/`, `.azure-pipelines.yml`, `cloudbuild.yaml`) was observed at the workspace root either. The `justfile` (330 lines) and `cargo xtask` subcommands (chapter 5) are the workflow-task surface; no automated CI configuration is checked in at audit-time.

## 2.10 Confidence

Every row in this chapter is **High** confidence: it is taken from the workspace manifests, `cargo metadata --no-deps`, or a direct filesystem check at the audit-time SHA. No claim in this chapter is inferred from a name without manifest evidence. Line citations were rebuilt from a programmatic line-number scan after a 0.1.0 first-pass discovered systematic off-by-many errors.

<!-- <FILE>pro/EXISTING-SYSTEM-PRD/02_workspace_inventory.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.1</VERS> -->
