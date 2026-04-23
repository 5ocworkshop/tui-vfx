<!-- <FILE>docs/audits/2026-04-24-rust-security-audit.md</FILE> - <DESC>Rust repository audit (security, stability, maintainability, performance) covering tui-vfx and sibling tui-vfx-recipes on 2026-04-24. Records methodology, findings resolved, evidence-of-coverage non-findings, findings not resolved with rationale, post-fix verification, and Cargo.lock handling notes.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Initial audit report produced against the rust-security-audit prompt (AUDIT_PROMPT.md) at the user's request. Scope: contract-preserving fixes only; contract-changing findings flagged and deferred.</WCTX> -->
<!-- <CLOG>0.1.0: initial audit report.</CLOG> -->

# Rust security, stability, maintainability, performance audit — 2026-04-24

**Auditor protocol:** `/usr/projects/rust-security-audit/AUDIT_PROMPT.md`
**Scope:** `/usr/projects/tui-vfx` (workspace, 12 crates) and sibling `/usr/projects/tui-vfx-recipes` (workspace, 5 crates).
**Constraint:** no change may alter a public function's signature, return type, or observable behavior. Findings that require a contract change are flagged and left in "not resolved."

---

## 1. Methodology

### Tool versions
- `rustc 1.95.0 (59807616e 2026-04-14)` (upgraded from 1.91.1 mid-audit at user direction).
- `cargo 1.91.1 (ea2d97820 2025-10-10)`.
- `cargo-audit 0.22.0`, advisory DB with 1053 advisories loaded (`/home/jac/.cargo/advisory-db`).
- `ripgrep 13+`, `git`, `jq`.

### Workspace facts
- **tui-vfx**: edition 2024, resolver 3, MSRV bumped 1.86.0 → **1.95.0** in this audit. Members: `tui-vfx`, `-compositor`, `-content`, `-core`, `-core-macros`, `-debug`, `-geometry`, `-probe`, `-shadow`, `-style`, `-types`, `xtask` (12 total). Library; `Cargo.lock` tracked.
- **tui-vfx-recipes**: edition 2024, resolver 3, MSRV bumped 1.86.0 → **1.95.0** in this audit. Members: `tui-vfx-recipes`, `pipeline-validator`, `recipe-probe`, `recipe-validator`, `tui-vfx-trace` (5 total). Library; `Cargo.lock` tracked.

### Baseline working-tree state (captured pre-audit)
Both repos had substantial uncommitted changes from an in-flight V3 refactor. Specifically:
- tui-vfx: ~30 modified `.rs` files in `tui-vfx-compositor`, `-content`, `-geometry`, `-shadow`, `-types`, and the umbrella `tui-vfx` crate. Design docs also modified.
- tui-vfx-recipes: ~30 modified `.rs` files under `src/` plus `Cargo.lock`, including an API surface rename that leaves `pipeline-validator` **failing to compile** (unresolved `tui_vfx_recipes::v3::validate_v3_compiled_recipe` and missing `build_compiled_v3_qc_report`).

The audit was run against this working tree (no stashing, no reset). Any findings tied to the in-flight refactor are attributed to it and **out of audit scope**. The post-fix verification gate is compared against the pre-fix **same** working tree to detect regressions introduced by audit changes only.

### Commands run (Phase 1–4)

Phase 2 (automated sweeps) in both repos:
```bash
cargo audit
cargo clippy --workspace --release --all-targets
cargo test --workspace --release --no-fail-fast
```

Phase 3 (grep sweeps) in both repos, limited to `src/` trees, excluding test files:
```bash
rg '\.unwrap\(\)|\.expect\(|panic!\(|todo!\(|unimplemented!\('
rg 'println!|eprintln!|dbg!\('
rg 'TODO|FIXME|XXX|HACK'
rg 'transmute|MaybeUninit|from_raw|into_raw|mem::uninitialized'
rg 'f32 == |f64 == | == 0\.0|!= 0\.0'
rg '#\[deprecated|#\[allow\(deprecated'
rg 'unsafe '
rg '\.sqrt\(\)|\.log\(|\.ln\(|\.exp\(|\.tan\(|\.acos\(|\.asin\('
```

Phase 4 (manual `unsafe` review): zero `unsafe` sites in either repo (the one `unsafe ` grep hit was a comment token inside `tui-vfx-types/src/braille.rs`). Four-point checklist therefore vacuously satisfied.

Phase 5 (integer arithmetic) and Phase 6 (NaN/non-finite): see "Findings that came back clean" below.

---

## 2. Findings resolved

| # | Class | Finding | Fix applied | Commit |
|---|---|---|---|---|
| 1 | Security | RUSTSEC-2026-0009 (medium, 6.8) — DoS via stack exhaustion in `time 0.3.45`, transitive from `ratatui-widgets` → `ratatui` in tui-vfx-recipes | Bumped tui-vfx-recipes MSRV to 1.95.0 and ran `cargo update -p time`. Locked to `time 0.3.47` (the fix version, which requires rustc ≥ 1.88). Picked up `num-conv 0.1.0 → 0.2.1` and `time-core 0.1.7 → 0.1.8` as accompaniments. | tui-vfx-recipes `14a7ce5` |
| 2 | Maintainability | Workspace MSRV (1.86.0) was lower than the local toolchain and blocked taking security-fix versions of dependencies | Raised MSRV to **1.95.0** in both workspaces (`rust-version = "1.95.0"` in each root `Cargo.toml` plus each tool crate under `tui-vfx-recipes/tools/`). | tui-vfx `ea45edd`, tui-vfx-recipes `14a7ce5` |

### Notes on the time fix path considered and rejected

An alternative was to feature-gate `ratatui`'s `widget-calendar` (the only dep edge pulling `time` into the tui-vfx-recipes graph; zero `Calendar` usage across both repos). This would have removed `time` from the dep graph entirely while leaving MSRV at 1.86.0. User direction ("for security we'll go up to 1.95.0") preferred the MSRV bump so every fix lane is open, not just this one.

---

## 3. Findings that came back clean — evidence of coverage

These checks ran and turned up **zero findings**. Each zero is evidence that the category was covered, not filler.

| Category | tui-vfx | tui-vfx-recipes |
|---|---|---|
| `unsafe` blocks or `unsafe fn` | **0** | **0** |
| `transmute` / `MaybeUninit` / `from_raw` / `into_raw` / `mem::uninitialized` | 0 | 0 |
| `TODO` / `FIXME` / `XXX` / `HACK` markers in `src/` | 0 | 0 |
| `#[deprecated]` on items / `#[allow(deprecated)]` suppressions | 0 | 0 |
| Genuine `f32 ==` / `f64 ==` comparisons | 0 | 0 |
| Production-code `unwrap` / `expect` / `panic!` / `todo!` / `unimplemented!` with a reachable failure path | **0** (see detail below) | 0 |
| `.sqrt()` on unconstrained input | 0 | 0 |
| `.log()` / `.ln()` / `.exp()` / `.tan()` / `.acos()` / `.asin()` on unconstrained input | 0 | 0 |
| Debug prints (`println!` / `eprintln!` / `dbg!`) on reachable production paths | 0 | 0 |

### Float equality detail

The `f32 == | f64 == | == 0.0 | != 0.0` sweep returned six hits across tui-vfx (`cls_slide_shift.rs`, `fnc_blend_colors.rs`, `cls_bevel_shader.rs`, `cls_prepared_filter.rs`, `fnc_grade_shadow_cell.rs`, `color.rs`). Every hit was `== 0.0` guarding a subsequent division or early-return — the correct use of the idiom, not a float-comparison bug.

### NaN-producing math detail

The `sqrt/log/ln/exp/tan/acos/asin` sweep returned ~38 hits in tui-vfx (zero in tui-vfx-recipes). Every `.sqrt()` argument was a sum of squares (`dx*dx + dy*dy`, `.powi(2) + .powi(2)`) or explicitly clamped non-negative. Every `.exp()` argument was a negated non-negative producing bounded `[0, 1]`. `cos` is total. No `log`/`acos`/`asin` hits with unbounded input. Domain-safe.

### Production unwrap detail

A naive grep across `crates/*/src/**/*.rs` (excluding `tests/` and `test*.rs`) returned 256 hits in tui-vfx, but the vast majority were inside inline `#[cfg(test)]` modules. Filtering each file at the first `#[cfg(test)]` line brought the production count to ~32 sites across 11 files. Each was manually reviewed:

| File | Count | Classification |
|---|---|---|
| `tui-vfx-debug/src/logger.rs` | 8 | `Mutex::lock().unwrap()` — dev-tooling poison fatal |
| `tui-vfx-style/src/models/cls_gradient.rs` | 5 | `first/last().unwrap()` guarded by `if self.stops.is_empty() { return ...; }` at line 86 |
| `tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs` | 4 | `expect()` with documented invariant ("shadow coverage implies a shadow cell") |
| `tui-vfx-types/src/braille.rs` | 1 | `char::from_u32(0x2800 + u8)` — range-bounded, infallible by construction |
| `tui-vfx-debug/src/inspection/cls_trace_sink.rs` | 3 | Mutex poison expect — dev-tooling |
| `tui-vfx-debug/src/inspection/cls_trace_emitter.rs` | 2 | RwLock poison expect — dev-tooling |
| `tui-vfx-core-macros/src/lib.rs` | 2 | `to_lowercase/uppercase().next().unwrap()` on ASCII-only proc-macro identifier chars |
| `tui-vfx-compositor/src/context/cls_compositor_ctx.rs` | 2 | `scratchpad.as_ref/mut().unwrap()` — guaranteed `Some` by the assignment immediately above each site |
| `tui-vfx-core/src/schema/fnc_node_to_json_schema.rs` | 1 | `schema.as_object_mut().unwrap()` — the type of `schema` at that point is a known JSON object |
| `tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec.rs` | 1 | `expect("spec shader layers should lower through the grouped V3 runtime seam")` — documented invariant |
| `tui-vfx-compositor/src/pipeline/cls_prepared_mask.rs` | 1 | `spec.resolve_wipe().unwrap()` inside a `MaskSpec::Wipe { .. }` match arm; `resolve_wipe()` returns `Some(_)` for every `Wipe` branch and `None` only for `_ => ...` (non-Wipe), so the `unwrap` is infallible at that call site |

Every production unwrap is either a poisoning expect (mutex/rwlock) in dev-tooling crates, or an invariant documented in code, or infallible by construction. No contract-preserving fix is needed.

### Debug print detail

Four hits in tui-vfx, zero in tui-vfx-recipes:
- `crates/tui-vfx-probe/src/bin/pipeline-probe.rs:24,233,234` — `eprintln!`/`println!` are intentional CLI output of the `pipeline-probe` binary.
- `crates/tui-vfx-debug/src/logger.rs:100` — `println!` inside the logger's own `DefaultLogger::log` path (the logger printing is its job).

---

## 4. Findings not resolved

### 4a. Upstream-pending advisories

Both remain after the audit; both require upstream action plus in-repo contract change to resolve.

| ID | Crate | Class | Dependency path | Resolution path |
|---|---|---|---|---|
| RUSTSEC-2026-0097 | `rand 0.8.5` | Unsoundness warning | Transitive: `mixed-signals 0.2.3` → `rand 0.8.5` and `rand_distr 0.4.3` → `rand 0.8.5`. Also directly from `tui-vfx-compositor` (which takes `rand 0.8`). | Upgrade `rand` to 0.9.x in the `mixed-signals` sibling (`/usr/projects/mixed-signals`) and in the `tui-vfx-compositor` direct dep. `rand 0.9` is a breaking upgrade; observable API flow-through. Per Intention 9 (signal primitives live upstream in mixed-signals), the upstream bump must come first. |
| RUSTSEC-2026-0002 | `lru 0.12.5` | Unsoundness warning (Stacked-Borrows violation in `IterMut`) | Direct dep of `tui-vfx-recipes`. | Upgrade to `lru 0.17.0+`. 0.12 → 0.17 is four major bumps; likely API churn. Requires code review of every `lru::LruCache` call site in `tui-vfx-recipes`. |

### 4b. Findings deliberately deferred (user direction)

- **Clippy cleanup**: default-profile clippy in `tui-vfx-style` (45 warnings, 44 autofix-able) and `tui-vfx-recipes` (43 warnings, 17 autofix-able). All observable as stylistic (clones on `Copy`, collapsible `if let`, one unused import `CellMask`, `starts_with` idiom, `iter().any()` vs `.contains()` preference). User direction: "2 and 3 leave alone for now" — not applied.

### 4c. Out-of-audit preexisting issues

- **pipeline-validator compile errors** in tui-vfx-recipes (unresolved `validate_v3_compiled_recipe`, missing `build_compiled_v3_qc_report`). Caused by the in-flight V3 authoring refactor, not by any audit change. Fixing it belongs to that refactor, not to this audit.
- **15+ test failures in tui-vfx-recipes** across 7 integration targets — same root cause as the compile errors above. Unchanged by audit.

---

## 5. Verification

Gate: `cargo build --release`, `cargo clippy --release --lib` (warning count), `cargo test --release` (failure count), `cargo audit` (advisory count).

### tui-vfx
| Gate | Pre-audit | Post-audit | Status |
|---|---|---|---|
| `cargo build --release --workspace` | green | **green** (1m 32s on rustc 1.95) | ✅ |
| `cargo test --workspace --release --no-fail-fast` | 800+ pass, 0 fail, 30 ignored | **800+ pass, 0 fail, 30 ignored** | ✅ no regression |
| `cargo audit` | 1 allowed warning (rand 0.8.5) | **1 allowed warning (rand 0.8.5)** | ✅ no new advisories |
| Clippy warnings | ~50 (style-heavy; no new errors) | **~50, unchanged** | ✅ no regression |

### tui-vfx-recipes
| Gate | Pre-audit | Post-audit | Status |
|---|---|---|---|
| `cargo build --release` | green (library crate builds; pipeline-validator fails — preexisting) | **green (library crate builds; pipeline-validator still fails — preexisting)** | ✅ no regression from audit |
| `cargo test --workspace --release --no-fail-fast` | 15 failed tests across 7 suites (preexisting, in-flight API refactor) | **same set of failures; no net-new** | ✅ no regression |
| `cargo audit` | 1 vulnerability (time 0.3.45), 2 allowed warnings (lru, rand) | **0 vulnerabilities, 2 allowed warnings (lru, rand)** | ✅ CVE cleared |
| Clippy warnings | 43 lib + pipeline-validator compile errors | **43 lib + pipeline-validator compile errors (preexisting)** | ✅ no regression |

---

## 6. `Cargo.lock` notes

Both repos are **libraries** and both currently track `Cargo.lock` in git (neither has it in `.gitignore`). That means:

- The `time 0.3.45 → 0.3.47` bump is fully committed in tui-vfx-recipes `14a7ce5` and ships to every downstream on its next build.
- Downstream library consumers that do **not** track their own `Cargo.lock` will resolve `time` fresh against the advisory DB and pick 0.3.47 (or higher) automatically once they target rustc ≥ 1.88. Downstream binary consumers (such as `gt-design`) will pick up the fix on their next `cargo update`.

The surgical lock-file strategy used in the tui-vfx-recipes commit:
1. Saved the working-tree `Cargo.lock` (containing in-flight path-dep reconciliations) to a tempfile.
2. Reset `Cargo.lock` to HEAD with `git checkout -- Cargo.lock`.
3. Ran `cargo update -p time` against HEAD state, producing a diff that contains only the `time`-related bumps plus the path-dep reconciliations that the current `Cargo.toml` already required (`mixed-signals 0.2.3` as a path dep; new `tui-vfx-core` edge on `tui-vfx-types`).
4. Verified the resulting lock file byte-identical to the saved in-flight version (`diff` exit 0) — the reconciliation converged deterministically and nothing from the user's in-flight state was dropped.
5. Committed.

---

## 7. Out-of-scope items (per the audit prompt)

Things this audit explicitly does not cover:
- Fuzz testing (no `cargo-fuzz` run).
- `miri` unsoundness analysis of the non-existent `unsafe` code.
- Timing side channels, cryptographic weakness, or supply-chain attack surface beyond what `cargo audit` reports.
- License compliance (`cargo-deny` not run).
- Whether test coverage is *adequate* for the code that exists (only that tests pass).
- Performance profiling — no `cargo bench` or `perf` run in this pass. The project has a standing 60 fps / 16.7 ms frame-budget gate (`bench_full_trace_60fps`) enforced by CI; that remains the authoritative performance check.

---

## 8. Recommended follow-up (not applied)

1. **Upstream mixed-signals rand 0.8 → 0.9 bump** to clear RUSTSEC-2026-0097 — requires work in `/usr/projects/mixed-signals` first, then a `mixed-signals` version bump in both tui-vfx and tui-vfx-recipes, then removing the direct `rand 0.8` dep from `tui-vfx-compositor`.
2. **Direct lru 0.12 → 0.17 bump in tui-vfx-recipes** to clear RUSTSEC-2026-0002 — multi-major upgrade; needs a review of every `LruCache` call site.
3. **Clippy cleanup pass** on `tui-vfx-style` (45 warnings) and `tui-vfx-recipes` (43 warnings) — deferred at user direction; low risk, autofix-able.
4. **Close the pipeline-validator API drift** in tui-vfx-recipes — part of the in-flight V3 refactor, not this audit's scope.

<!-- <FILE>docs/audits/2026-04-24-rust-security-audit.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
