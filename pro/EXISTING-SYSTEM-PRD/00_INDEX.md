<!-- <FILE>pro/EXISTING-SYSTEM-PRD/00_INDEX.md</FILE> - <DESC>Index for the evidence-backed Existing-System PRD of tui-vfx — links to the 14 chapters, audit metadata, and inspection scope.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Initial scaffold; chapter 01 + 02 produced under US-001. Chapters 03-14 pending.</WCTX> -->
<!-- <CLOG>0.1.0: scaffold + audit metadata + chapter index.</CLOG> -->

# Existing-System PRD — tui-vfx

This is the index for an evidence-backed factual baseline of the tui-vfx workspace as of the audit date below. It is produced under the rules of `pro/REVERSE-PRD.md`. It does not propose a new architecture; every claim is cited from the workspace itself.

## Audit metadata

| Field | Value | Evidence |
|---|---|---|
| Audit date | 2026-04-28 | (machine date at audit start) |
| Workspace root | `/usr/projects/tui-vfx` | filesystem |
| Git branch | `master` | `git rev-parse --abbrev-ref HEAD` |
| Git commit SHA | `519a6c67540214117436c81f67a8bcf50ea0f5aa` | `git rev-parse HEAD` at audit start |
| Working tree | clean | `git status` at audit start |
| Workspace version | `0.11.0` (pre-1.0) | `Cargo.toml:31` (`workspace.package.version = "0.11.0"`) |

## Inspection scope

- **In scope.** All 13 workspace members declared in `Cargo.toml:14-28` (`crates/tui-vfx`, `crates/tui-vfx-compositor`, `crates/tui-vfx-content`, `crates/tui-vfx-core`, `crates/tui-vfx-core-macros`, `crates/tui-vfx-debug`, `crates/tui-vfx-geometry`, `crates/tui-vfx-next`, `crates/tui-vfx-probe`, `crates/tui-vfx-shadow`, `crates/tui-vfx-style`, `crates/tui-vfx-types`, `xtask`). Workspace-level configuration (`Cargo.toml`, `clippy.toml`, `.cargo/config.toml`, `justfile`, `README.md`, `CHANGELOG.md`, `examples/`). Note: `crates/tui-vfx-next` is a clean-room V3.1 surface-contract spike crate (639 LOC, 7 source files); it landed mid-audit and is included in scope.
- **Out of scope.** Sibling repositories that share the path-dep boundary (`../mixed-signals`, `../tui-vfx-recipes`, `../gt-design`, `../rocketsplash`). They are referenced where the in-tree code imports from them, but their internals are not catalogued here.
- **Skipped.** `target/` (build artifacts), `.git/`, `recyclebin/` (per-project archive convention; indexed by the librarian but not part of live source). The `pro/` working directory is the input/output workspace for this audit and is not part of the system under audit.

## Primary evidence collector

The `ofpf-*` semantic suite (a thin alias layer over `librarian-cli` against a daemon-loaded index) is the primary collector. At audit start the daemon reported 960 files, 7005 definitions, 2566 edges across 543 Rust files; graph age 21 s; not stale (`ofpf-status`).

## Chapter index

| # | File | Section |
|---|---|---|
| 01 | [01_executive_summary.md](01_executive_summary.md) | Executive summary |
| 02 | [02_workspace_inventory.md](02_workspace_inventory.md) | Repository and workspace inventory |
| 03 | [03_feature_inventory.md](03_feature_inventory.md) | Product feature inventory |
| 04 | [04_functional_requirements.md](04_functional_requirements.md) | Functional requirements extracted from code |
| 05 | [05_options_and_configuration.md](05_options_and_configuration.md) | Options and configuration catalog |
| 06 | [06_public_interfaces.md](06_public_interfaces.md) | Public interfaces |
| 07 | [07_data_model_and_persistence.md](07_data_model_and_persistence.md) | Data model and persistence |
| 08 | [08_runtime_behavior.md](08_runtime_behavior.md) | Runtime behavior |
| 09 | [09_security_permissions_secrets.md](09_security_permissions_secrets.md) | Security, permissions, secrets |
| 10 | [10_tests_and_verified_behaviors.md](10_tests_and_verified_behaviors.md) | Tests and verified behaviors |
| 11 | [11_architecture_observations.md](11_architecture_observations.md) | Architecture-relevant observations |
| 12 | [12_open_questions.md](12_open_questions.md) | Open questions and unknowns |
| 13 | [13_evidence_ledger.md](13_evidence_ledger.md) | Evidence ledger |
| 14 | [14_coverage_report.md](14_coverage_report.md) | Coverage report |

## Status of this PRD

| Chapter | Status |
|---|---|
| 00 — Index | populated (US-001) |
| 01 — Executive summary | **populated** (US-001 — passes: true) |
| 02 — Workspace inventory | **populated** (US-001 — passes: true) |
| 03 — Feature inventory | **deepened** (US-002 + US-003 + deepening pass — passes: true); 49 features incl. F015a + F044a + F048 + F049 — F001/F003/F004/F005/F006/F008/F009/F010/F011/F012/F028 substantially escalated to High confidence with end-to-end evidence |
| 04 — Functional requirements | **populated** (US-004 — passes: true); 63 REQ rows covering all features (60 original + REQ-061..063 for tui-vfx-next) |
| 05 — Options and configuration | **populated** (US-005 — passes: true); 41 OPT rows + 8 subsections |
| 06 — Public interfaces | **populated** (US-006 — passes: true); CLI + 12 Rust APIs + file formats + schemas + plugin surfaces |
| 07 — Data model and persistence | **populated** (US-007 — passes: true); domain types + file formats + SQLite persistence + state machines |
| 08 — Runtime behavior | **populated** (US-008 — passes: true); fully synchronous workspace; no async runtime, no tracing ecosystem, no metrics |
| 09 — Security, permissions, secrets | **populated** (US-009 — passes: true); 5 unsafe lines (1 file, test fixture); no auth / TLS / network / credential code |
| 10 — Tests and verified behaviors | **deepened** (US-010 + deepening-pass agent #5 — passes: true); 2854 test defs, 75 integration-test files, **assertion-body-derived behaviors-verified** for 14 feature groups; only the linker-only sub-suites (style models, cursor sub-classes, transformer sub-classes, easing, paths) remain Medium |
| 11 — Architecture observations | **deepened** (US-011 + deepening pass — passes: true); 20 observations (15 original + 11.15a-c, 11.16-11.20 from sub-agent reports) |
| 12 — Open questions | **deepened** (US-011 + deepening pass — passes: true); 11 of 13 questions resolved (only §12.5 recipe-validator status and §12.12 dep-rationale-block consistency remain genuinely open) |
| 13 — Evidence ledger | **populated** (US-012 — passes: true); 50+ E### entries across 11 categories |
| 14 — Coverage report | **populated** (US-012 — passes: true); honest record of what was read and what wasn't |

Each chapter is reviewed against its acceptance criteria in `.omc/prd.json` before its US-### story is marked `passes: true`.

<!-- <FILE>pro/EXISTING-SYSTEM-PRD/00_INDEX.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
