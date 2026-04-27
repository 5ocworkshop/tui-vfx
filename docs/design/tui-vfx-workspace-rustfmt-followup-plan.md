<!-- <FILE>docs/design/tui-vfx-workspace-rustfmt-followup-plan.md</FILE> - <DESC>Stub plan for the workspace-wide rustfmt cleanup that surfaced during configschema gate followup US-009. `cargo fmt --all` reformats ~40 files of pre-existing rustfmt drift across xtask/src/docs, xtask/src/audit, and xtask/src/main.rs that are unrelated to the configschema gate but block `just check-all` from going green. Out of scope for the configschema followup; needs its own packet before warn-only → hard-fail flip on 2026-07-01.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>2026-04-28: discovered during configschema gate followup US-009 verification. The umbrella `just check-all` and `just ci` recipes now invoke `audit-all` correctly, but they are red on `fmt-check` because of pre-existing workspace-wide rustfmt drift unrelated to the audit gate.</WCTX> -->
<!-- <CLOG>1.0.0: initial stub plan. Quantified scope: 54 files reformatted by `cargo fmt --all`, of which ~14 were edited by packet 1.9.A.followup (now committed clean) and ~40 are sibling pre-existing drift.</CLOG> -->

# Plan stub — workspace-wide rustfmt cleanup followup

> **Why this stub exists.** `cargo fmt --all` was run during configschema gate followup US-009 to validate the new umbrella wiring. It surfaced ~40 files of pre-existing rustfmt drift across `xtask/src/docs/*`, `xtask/src/main.rs`, `xtask/src/audit/fnc_audit_configschema.rs`, and several others — files unrelated to the configschema gate but blocking `just check-all` from going green.

## Verified scope (2026-04-28)

`cargo fmt --all` reformats 54 files: 1230 lines deleted, 513 lines inserted (mostly line-wrap normalizations). After excluding the 14 files edited by packet 1.9.A.followup (which were re-formatted in their respective commits), the remaining drift covers approximately 40 sibling files.

**Suspected drift origin:** rustfmt config or formatter version evolved past the last full-workspace fmt pass. Most diffs are line-wrap reflows (e.g. multi-line function calls collapsing to single line; or imports ordering).

**Indicator categories observed:**

- `xtask/src/docs/extract_signals_rustdoc.rs` (87 lines diff)
- `xtask/src/docs/merge_signals.rs` (42 lines)
- `xtask/src/docs/mod.rs` (30 lines)
- `xtask/src/docs/gen_signals_markdown.rs` (26 lines)
- `xtask/src/docs/parse_signals_toml.rs` (19 lines)
- `xtask/src/audit/fnc_audit_configschema.rs` (20 lines, despite this file being touched by 1.9.A — these reflows are not in scope for the audit gate's behavior)
- `xtask/src/main.rs` (4 lines)
- … and ~32 more files across the workspace

## Goal

Run `cargo fmt --all`, verify `cargo test --workspace` and `cargo clippy --workspace --all-targets -- -D warnings` still pass, commit as one atomic packet titled `chore: workspace-wide rustfmt --all`. After this packet, `just check-all` and `just ci` go green and the configschema audit gate's warn-only → hard-fail flip on 2026-07-01 is unblocked from the umbrella-recipe perspective.

## Phases

### Phase 0 — Snapshot

1. Capture `cargo test --workspace` pass count.
2. Capture `cargo clippy --workspace --all-targets -- -D warnings` (must be clean before fmt run).

### Phase 1 — Fmt run

1. `cargo fmt --all`.
2. `git status --short` — confirm scope matches the ~40-file expectation (no surprise extra files).
3. `git diff --stat` — capture line counts.

### Phase 2 — Re-verify gates

1. `cargo build --workspace` — clean.
2. `cargo test --workspace` — same pass count as Phase 0 snapshot.
3. `cargo clippy --workspace --all-targets -- -D warnings` — clean.
4. `cargo xtask audit configschema` — clean.
5. `cargo xtask docs check` — clean.
6. `just check-all` — **NOW GREEN** (the goal of this packet).

### Phase 3 — Commit + close

1. Single commit titled `chore: workspace-wide rustfmt --all` with a short summary referencing this plan.
2. Move this plan stub to `docs/design/completed/`.

## Estimated effort

~30 minutes. The fmt run is automated; the bulk is verification + commit.

## Why this is a separate packet

- **Scope-keeping:** packet 1.9.A.followup is about the configschema audit gate. Bundling 40 unrelated rustfmt fixes into it would violate the project's commit-discipline that scope and intent stay aligned.
- **Risk:** rustfmt reflows are mechanical and almost never break behavior, but they touch many files. A separate atomic commit makes a rollback (if any subtle issue surfaces) trivial.
- **Timing:** must land before 2026-07-01 (the configschema warn-only → hard-fail flip) so `just ci` actually exercises the audit gate. Until then, contributors run `just audit-all` directly.

## Pointer

Cross-referenced from:
- `docs/design/tui-vfx-configschema-audit-gate-plan.md` v3.0.0 §Out of scope (will need an addendum once this packet lands)
- `.omc/progress.txt` US-009 follow-up note

<!-- <FILE>docs/design/tui-vfx-workspace-rustfmt-followup-plan.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
