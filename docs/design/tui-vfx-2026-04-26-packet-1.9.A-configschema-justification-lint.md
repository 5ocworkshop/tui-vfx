<!-- <FILE>docs/design/tui-vfx-2026-04-26-packet-1.9.A-configschema-justification-lint.md</FILE> - <DESC>Implementation packet for buy-once sweep finding 1.9.A — a hand-written ConfigSchema audit lint. Self-contained brief: pre-flight, current-state audit of every existing `impl ConfigSchema for X` in tree, the justification-comment format spec, open architectural questions with recommended defaults, step-by-step plan, code snippets for the xtask subcommand and the baseline allowlist file format, test plan, acceptance criteria, verification commands, rollback plan. Junior-ready half-session packet: pure validation infrastructure, no production-code touch.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Capture sweep finding 1.9.A as a junior-ready packet. Lint design + existing-impl audit + grandfather plan. No execution.</WCTX> -->
<!-- <CLOG>1.0.0: initial packet (pre-flight, current-state audit of 12 in-tree hand-written impls + 1 macro-defined family, justification-comment format spec, three open questions with defaults, five-phase plan, code snippets, test plan, acceptance criteria).</CLOG> -->

# Packet 1.9.A — Hand-written `impl ConfigSchema` justification lint

> **Status: QUEUED.** Pure validation-infrastructure work. Half-session, single-author. No production-code path is modified by Phase 1–3 of this packet; Phase 5 (the audit-as-followup) is split out as a sibling packet so the lint can land standalone.

> **Source findings.** `docs/design/tui-vfx-buy-once-architecture-sweep.md` v1.4.0 §1.9.A (lines 442–476) and §6.5 Option A (lines 701–719). Handoff queue: `docs/design/tui-vfx-2026-04-26-handoff-outstanding.md` v1.3.0 §8.4 row "1.9.A" — flagged as the lowest-cost move in the queue ("cheap quick win, ~1 hour of xtask code, recommended for a slow-day session").
>
> **Risk tier (per sweep).** M (audit) → S (lint mechanic). The lint itself is mechanical; the audit-as-followup carries the design judgment.
>
> **Sequencing.** Independent. Does not block or unblock any other queued packet. Can land any time.

---

## Goal & motivation

Hand-written `impl ConfigSchema for X` proliferates parallel to the `#[derive(ConfigSchema)]` macro. Each existing hand-write is technically valid. Each one is also a one-off the derive macro could probably handle if either the macro or the type shape were adjusted. Per Intention 12A: "V3 schema-bearing types must support `ConfigSchema` derivation or have an explicit reason not to." That rule is currently informal — there is no mechanical check that the "explicit reason" is recorded.

Without a friction gate, the pattern grows silently. A new hand-write costs nothing to add today and there is no convention that asks the author to justify it. Per sweep §6.5 the recommended action is Option A (hard gate with justification comment): every hand-written `impl ConfigSchema for X` must be preceded by a `// CONFIGSCHEMA-JUSTIFICATION: <reason>` comment, enforced by `cargo xtask`. Per Intention 25 rule 2 (mechanize drift classes already seen) the drift exists today; per Intention 25 rule 5 the smallest mechanization is a grep-and-baseline lint, not a differ. Per Intention 25 rule 7 the failure message names the playbook ("add a justification comment, or migrate to `#[derive(ConfigSchema)]`").

**How the lint stays sober.**

- **No false positives on legitimate hand-writes.** The justification-comment format names a small enum of canonical exception kinds (`derive-cannot-handle-foreign-type`, `derive-cannot-handle-generic-T`, `derive-cannot-handle-untagged-enum`, `intentional-divergence-from-derive-output`, `Other("...")`). A legitimate hand-write satisfies the gate by adding one line.
- **No friction for the macro itself.** Impls produced by the `#[derive(ConfigSchema)]` macro at `crates/tui-vfx-core-macros/src/fnc_impl_config_schema.rs` are emitted from `quote!` — they never appear textually in repo source — so the lint cannot see them and cannot misfire on them.
- **No friction for `macro_rules!`-emitted impls.** The two `impl ConfigSchema for $t` lines inside the `impl_primitive_schema!` / `impl_int_schema!` macros at `crates/tui-vfx-core/src/schema/mod.rs:28,40` are inside the macro definition and are not impls themselves. The lint must skip them by detecting macro-body context (or by allowlisting that one file).
- **No flag-day audit.** Existing impls are grandfathered via a baseline allowlist of `(file, type)` tuples. The lint fails only on impls not in the baseline. Every existing impl can earn its justification later, in a sibling packet, without blocking the lint from landing.

## Scope

**In scope.**

- A new xtask subcommand (recommended `cargo xtask audit configschema`) that scans the workspace for `impl ConfigSchema for X`, checks each match against the baseline allowlist, and verifies that any new match is preceded by a recognized `// CONFIGSCHEMA-JUSTIFICATION: ...` comment.
- The justification-comment format specification: required form, required content, the canonical exception-kind enum, and the `Other("...")` escape hatch (which emits a CI warning to encourage canonical kinds).
- A baseline allowlist file (`xtask/data/configschema_baseline.toml` or similar) seeded by enumerating every existing in-tree `impl ConfigSchema for X`. Baseline entries exempt the impl from the justification requirement; the followup audit packet either adds a justification comment to each entry and removes it from the baseline, or flags it as derive-migratable.
- A short docs page (`docs/CONFIGSCHEMA_JUSTIFICATION.md`) explaining the rule, the canonical exception kinds, and when to expect derive to grow to handle a new case.
- Tests for the lint itself: fixture-based (a new unjustified impl fails; a new impl with the canonical justification passes; an existing impl in the baseline passes regardless; an impl removed from the baseline does not false-positive).
- Wiring the lint into the existing CI gate (justfile recipe + the same call site that runs `cargo xtask docs check`).

**Out of scope.**

- Any rewrite of existing hand-written `impl ConfigSchema` blocks. The grandfather rule means the lint can land without touching production code. The audit-as-followup packet (Phase 5) walks the baseline, adds the justification comments, and removes entries from the baseline as they are validated.
- Teaching the `#[derive(ConfigSchema)]` macro to handle additional cases (generic bounds, foreign types, untagged enums). Each derive-macro extension is its own packet, motivated by what the followup audit finds. This packet only mechanizes the policy; it does not change what the policy decides.
- A differ between hand-written impls and what the derive macro would have emitted (sweep §6.5 Option C). Per the sweep, the differ is moderate effort and the simpler grep-and-baseline gate is the recommended path. The differ can be a future packet if the followup audit reveals enough cases to justify it.
- Any migration of existing hand-written impls to derive form. That is the followup packet's call, made per impl.

**Crates touched.**

- **`xtask/`:** new subcommand module + the baseline allowlist file under `xtask/data/`.
- **`docs/`:** new `CONFIGSCHEMA_JUSTIFICATION.md`.
- **`justfile`:** new `audit-configschema` recipe + add the call to whichever recipe is the canonical CI entry (currently `docs-check`; adapt as the project's CI gate evolves).

No source crate is touched in Phases 1–4. Phase 5 (the followup audit) is a separate packet.

## Pre-work checklist

```bash
# Confirm orientation per CLAUDE.md.
ofpf-status
cat /usr/projects/tui-vfx/CLAUDE.md
cat /usr/projects/tui-vfx/steering/INTENTIONS.md     # esp. 12A, 14, 15, 24, 25

# Re-ground the audit numbers — they may have moved between packet write and execution.
ofpf-content "impl ConfigSchema for" --glob "**/*.rs"

# Inspect the derive macro and the xtask entrypoint.
ofpf-inspect /usr/projects/tui-vfx/crates/tui-vfx-core-macros/src/fnc_impl_config_schema.rs
ofpf-inspect /usr/projects/tui-vfx/xtask/src/main.rs

# Confirm the existing CI gate path. There is no .github/workflows/ directory in tree;
# the project relies on `just` recipes that wrap `cargo xtask`. Verify before wiring.
ls /usr/projects/tui-vfx/.github 2>&1 || echo "no .github (expected as of 2026-04-26)"
grep -nE "^[a-z-]+:" /usr/projects/tui-vfx/justfile

# Source citations.
grep -n "1\.9\.A" /usr/projects/tui-vfx/docs/design/tui-vfx-buy-once-architecture-sweep.md
grep -n "1\.9\.A" /usr/projects/tui-vfx/docs/design/tui-vfx-2026-04-26-handoff-outstanding.md
```

If any `ofpf-*` tool errors three times, run `ofpf-bug` and surface to the user. Do not proceed with hand-rolled `grep`/`find` substitutes — the lint depends on the canonical `ofpf-content` output as a sanity check.

## Current-state audit

### Every existing `impl ConfigSchema for X` in the workspace

Per `ofpf-content "impl ConfigSchema for" --glob "**/*.rs"` on 2026-04-26 (excluding `recyclebin/`):

| # | File | Line | Type | Form | Justification-comment present? | Likely category |
|---|---|---|---|---|---|---|
| 1 | `crates/tui-vfx-types/src/color.rs` | 27 | `Color` | hand-written | No | wire-format primitive (foreign-shape constraint — the schema declares the RGBA range explicitly per channel) |
| 2 | `crates/tui-vfx-types/src/role_tag.rs` | 124 | `RoleTag` | hand-written | No | derive-could-handle (looks like a tagged unit-enum with `Custom(InternedRoleName)` payload — derive may or may not handle the interned wrapper) |
| 3 | `crates/tui-vfx-core/src/schema/mod.rs` | 28 (inside `impl_primitive_schema!`) | `$t` | macro-emitted (`macro_rules!` body) | N/A | **lint must skip — not a real impl** |
| 4 | `crates/tui-vfx-core/src/schema/mod.rs` | 40 (inside `impl_int_schema!`) | `$t` | macro-emitted (`macro_rules!` body) | N/A | **lint must skip — not a real impl** |
| 5 | `crates/tui-vfx-core/src/schema/mod.rs` | 69 | `String` | hand-written (cannot be macro-folded — String has no `MIN`/`MAX`) | No | foreign type / primitive bridge |
| 6 | `crates/tui-vfx-core/src/schema/mod.rs` | 77 | `&str` | hand-written | No | foreign type / primitive bridge |
| 7 | `crates/tui-vfx-core/src/mixed_signals_schema.rs` | 32 | `SignalOrFloat` | hand-written | No | foreign type (lives in `mixed-signals`, derive cannot reach across crates) |
| 8 | `crates/tui-vfx-core/src/mixed_signals_schema.rs` | 82 | `SignalSpec` | hand-written | No | foreign type (lives in `mixed-signals`) |
| 9 | `crates/tui-vfx-core/src/mixed_signals_schema.rs` | 467 | `EasingType` | hand-written | No | foreign type (lives in `mixed-signals`) |
| 10 | `crates/tui-vfx-core/src/bindable/cls_bindable.rs` | 85 | `Never` | hand-written | partial — file rustdoc explains the uninhabited-type rationale, no inline `// CONFIGSCHEMA-JUSTIFICATION:` line | intentional-divergence (uninhabited type — derive would emit nothing useful) |
| 11 | `crates/tui-vfx-style/src/models/fnc_style_region_schema.rs` | 21 | `StyleRegion` | hand-written | partial — file `<DESC>` envelope explains the cross-crate constraint ("Role(RoleTag) variant requires manual schema because RoleTag lives in tui-vfx-types which doesn't depend on tui-vfx-core"), no inline `// CONFIGSCHEMA-JUSTIFICATION:` line | derive-cannot-handle-cross-crate-trait-dep |
| 12 | `crates/tui-vfx-geometry/src/types/path_type.rs` | 201 | `PathType` | hand-written | No | derive-could-handle (looks like a tagged enum with struct variants — strong derive-migration candidate) |
| 13 | `crates/tui-vfx-types/src/color.rs` (second pass — note: the audit table above already lists Color once) | — | — | — | — | **see #1** |

The two recyclebin matches (`recyclebin/crates/tui-vfx-style/src/models/cls_bindable_u16.rs:107`, `cls_bindable_string.rs:138`) are out of scope per the recyclebin protocol and the lint must skip the entire `recyclebin/` tree.

**In-tree count: 12 hand-written impls + 2 macro-emitted lines (skipped). Of the 12, ~3 are unambiguously must-be-hand-written (foreign-type bridges to `mixed-signals`: SignalOrFloat, SignalSpec, EasingType), ~3 more are hand-written for sound structural reasons (Color, String, &str — primitive bridges with explicit range/structural shape; Never — uninhabited), and the remaining ~3–6 (RoleTag, PathType, StyleRegion plus the Bindable family in recyclebin) are plausible derive-migration candidates.** The followup audit packet adjudicates each.

### The derive macro at `crates/tui-vfx-core-macros/src/fnc_impl_config_schema.rs`

Per `ofpf-inspect`, the macro is 46 LOC and dispatches to `derive_struct_schema` and `derive_enum_schema`. It does NOT propagate generic bounds (i.e. it does not emit `where T: ConfigSchema` for generic types). It does NOT handle foreign types (impls inside the crate that owns the trait are required by orphan rules). It does NOT handle untagged enums (no support for `serde(untagged)` shape). These three limitations are the canonical recognized exceptions and the justification-comment format names them explicitly.

### The existing xtask CI gates at `xtask/src/`

Per `ofpf-inspect /usr/projects/tui-vfx/xtask/src/main.rs`, the xtask currently exposes two top-level subcommand groups:

- `cargo xtask docs <action>` — generation, validation, and freshness checks for `CAPABILITIES.md`, `API.md`, the AI context prompt, and the capability schemas.
- `cargo xtask recipes <action>` — recipe validation against `capabilities.json`.

Both are wrapped by `just` recipes (`docs-generate`, `docs-check`, `docs-validate`, etc.). There is no separate audit subcommand group today. **Per Q1 below, the recommended placement is a new `audit` group: `cargo xtask audit configschema`. This keeps the namespace clean for future audit additions (e.g. a `cargo xtask audit deslop` or `cargo xtask audit recyclebin-orphans` would slot in alongside).**

There is no `.github/workflows/` directory; CI runs through the `justfile` recipes. Wiring the lint means adding a `audit-configschema` recipe to the justfile and either: (a) calling it from whichever recipe is the canonical CI entrypoint, or (b) adding it to the developer's local "before push" checklist if there is no such recipe yet. Either way, the wiring is one line.

## The justification-comment format specification

**Required form.** Either:

1. A `///` doc comment block immediately above the `impl ConfigSchema for X` line, containing a line that starts with `CONFIGSCHEMA-JUSTIFICATION:`. The doc comment may contain other rustdoc on adjacent lines.
2. A `// CONFIGSCHEMA-JUSTIFICATION: <kind>: <reason>` line directly preceding the `impl` line (no blank line between). This form is for inline `impl` blocks where a doc comment would attach to the wrong item.

The lint accepts either form. The doc-comment form is preferred because it surfaces in `cargo doc` output.

**Required content.** The justification names a recognized exception kind, optionally followed by `:` and free-form prose explaining the specific case:

```text
// CONFIGSCHEMA-JUSTIFICATION: derive-cannot-handle-foreign-type: SignalOrFloat lives in the mixed-signals crate; orphan rules forbid #[derive].
```

**Canonical exception kinds.** A small enum, defined inline in the lint subcommand:

| Kind | When to use |
|---|---|
| `derive-cannot-handle-foreign-type` | Type lives in another crate; orphan rules forbid `#[derive]`. (Examples: `SignalOrFloat`, `SignalSpec`, `EasingType`.) |
| `derive-cannot-handle-generic-T` | Generic type with `ConfigSchema` bounds the macro does not currently emit. (Example anticipated: `VfxBindable<T, S>` if it had been chosen for derive form.) |
| `derive-cannot-handle-untagged-enum` | Enum uses `serde(untagged)` and the macro does not emit untagged-walk schemas. |
| `derive-cannot-handle-cross-crate-trait-dep` | The type's variants reference another crate's types in a way that breaks the derive's import resolution. (Example: `StyleRegion::Role(RoleTag)` — RoleTag lives in `tui-vfx-types` which does not depend on `tui-vfx-core`.) |
| `intentional-divergence-from-derive-output` | The hand-written impl produces a schema that differs from what derive would emit, on purpose. Reason follows. (Example: `Color` declares per-channel ranges that derive does not synthesize.) |
| `primitive-bridge` | The type IS a primitive or a thin wrapper, and the schema is a `SchemaNode::Primitive` literal. (Example: `String`, `&str`.) |
| `uninhabited-type` | The type has no constructable values; the schema is an empty `SchemaNode::Enum`. (Example: `Never`.) |
| `Other("<reason>")` | Catch-all. Triggers a CI **warning** (not a failure) urging the author to add a canonical kind. The lint passes; reviewers see the warning and either accept the `Other` case or push for canonicalization. |

The kinds are versioned with the lint. Adding a new kind is a one-line patch to the lint plus a docs update; the lint refuses unrecognized kinds (other than the explicit `Other("...")` form).

## Open architectural questions

### Q1 — Where the lint lives

**Options.**

- **A.** A new top-level xtask subcommand group: `cargo xtask audit configschema`. Future audits (e.g. a deslop checker, a recyclebin-orphan checker) slot in as `cargo xtask audit <thing>`.
- **B.** An extension of the existing `cargo xtask docs` subcommand: `cargo xtask docs audit-configschema`. Matches the rule that schema/docs concerns live under `docs`.
- **C.** A standalone CI script outside xtask (a bash or Python script in `scripts/`). Fastest to land; loses the cross-platform Rust-only invariant.

**Recommended: Option A.** Per Intention 25 rule 5 (the smallest intervention that delivers full coverage) the audit group is one new subcommand line and one new module file. Per Intention 24 (abstractions earn their place) the `audit` namespace is genuinely useful for future audits — it does not exist for its own sake. Option B conflates "doc generation" with "policy enforcement" semantically. Option C loses the workspace's cross-platform Cargo invariant (the project is Linux-blessed but xtask runs on macOS and Windows for contributors).

**Stop-and-ask trigger.** If the user prefers Option B for terseness, that is a one-line difference in the subcommand wiring; the rest of the packet is identical.

### Q2 — Lint enforcement timing

**Options.**

- **A.** Fail at the CI gate immediately (any new unjustified impl breaks the build).
- **B.** Warn-only for one release cycle, then promote to fail. Documented deprecation pattern.
- **C.** Warn-only indefinitely (soft enforcement; rely on review to catch).

**Recommended: Option B.** Per Intention 25 rule 7 (the failure message documents the playbook) the warn-only period gives contributors time to learn the format without blocking PRs. Per Intention 14 (audit gates) the promotion to hard-fail must be scheduled and visible — the docs page commits to a date, the lint emits a "this will fail starting <date>" message during the warn period, and the promotion is a one-line change to the lint. Option A risks blocking unrelated PRs in the first week. Option C never delivers the gate; rule 6 of Intention 25 ("retire the gate when the drift class is gone" — but the drift class never goes away if the gate is permanently soft).

**Stop-and-ask trigger.** If the user wants Option A from day one, the lint is identical — only the exit code on warning changes (`exit 1` vs `exit 0` with a printed warning).

### Q3 — Existing-impl handling

**Options.**

- **A.** Grandfather all existing impls. Snapshot a baseline allowlist of `(file, type)` tuples; CI fails only on new entries (impls not in the baseline that lack a justification comment). The followup audit packet walks the baseline and either adds a justification comment (then removes the baseline entry) or flags the impl for derive migration.
- **B.** Fix-them-all-now. Audit each existing impl in this packet; add the justification comment retroactively. No baseline allowlist needed.

**Recommended: Option A (grandfather + audit-as-followup).** Per Intention 25 rule 5 (smallest intervention) the lint can land standalone without the multi-day audit blocking it. Per the handoff doc §8.4 (this packet is flagged as a "cheap quick win, ~1 hour of xtask code") Option B turns it into a multi-session packet that depends on per-impl design judgment (RoleTag's `Custom(InternedRoleName)` arm: derive-migratable or not? requires reading the macro). The grandfather mechanic preserves the friction-gate behavior for new impls (the actual goal) while deferring the per-impl judgment to a followup packet that can take its time. Option B blends two concerns: (1) install the gate, (2) audit the existing population. They have different cadences.

**Stop-and-ask trigger.** If the user wants Option B, the followup-audit packet (Phase 5 here) collapses into Phase 1, Phase 2 still works the same way, and the baseline-allowlist file ships empty. Total work roughly doubles but lands as one slice.

## Step-by-step implementation plan

### Phase 1 — Audit + enumerate the baseline

Generate the baseline allowlist by running `ofpf-content "impl ConfigSchema for" --glob "**/*.rs"` (excluding `recyclebin/`), filtering out `macro_rules!`-body matches, and writing the result to `xtask/data/configschema_baseline.toml`. The file is checked in. New entries cannot be added without a code review.

Format (TOML, one entry per impl):

```toml
# xtask/data/configschema_baseline.toml
#
# Baseline allowlist of pre-existing hand-written `impl ConfigSchema for X`
# blocks. Entries here are exempt from the justification-comment requirement.
# The followup audit packet (1.9.A.followup) walks each entry and either
# adds a justification comment + removes the entry, or migrates the impl to
# `#[derive(ConfigSchema)]` + removes the entry. New impls cannot be added
# to this file — they must satisfy the lint by carrying a justification
# comment instead.

schema_version = 1

[[entry]]
file = "crates/tui-vfx-types/src/color.rs"
type = "Color"
note = "wire-format primitive — declares per-channel range explicitly"

[[entry]]
file = "crates/tui-vfx-types/src/role_tag.rs"
type = "RoleTag"
note = "candidate for derive migration — see followup packet"

# … 10 more entries, one per in-tree match …
```

The `note` field is informational only — the lint does not parse it.

**Phase 1 deliverable.** `xtask/data/configschema_baseline.toml` checked in with one entry per pre-existing in-tree impl. No source-crate changes.

### Phase 2 — Implement the lint subcommand

Add `xtask/src/audit/mod.rs` and `xtask/src/audit/fnc_audit_configschema.rs`. Wire the new subcommand group into `xtask/src/main.rs`:

```rust
// In xtask/src/main.rs Commands enum:
#[derive(Subcommand)]
enum Commands {
    Docs { /* existing */ },
    Recipes { /* existing */ },
    /// Audit gates (validation infrastructure).
    Audit {
        #[command(subcommand)]
        action: AuditAction,
    },
}

#[derive(Subcommand)]
enum AuditAction {
    /// Verify every hand-written `impl ConfigSchema for X` has a justification
    /// comment, or is in the baseline allowlist.
    Configschema,
}
```

The lint scans every `**/*.rs` file under the workspace root (excluding `recyclebin/`, `target/`, and `xtask/data/`). For each file, it detects `impl ConfigSchema for X` matches at file scope (skipping `macro_rules!` bodies — see Risks & gotchas for the detection rule). For each match it checks: (a) is this `(file, type)` in the baseline? (b) if not, does the line above contain a recognized `// CONFIGSCHEMA-JUSTIFICATION:` comment, or does a `///` doc-comment block immediately above contain one? If neither, emit a failure (or warning per Q2's warn-only mode).

**Phase 2 deliverable.** `cargo xtask audit configschema` runs and exits 0 on the current tree (every existing impl is in the baseline). Adding a new `impl ConfigSchema for X` to a fixture without the comment fails the lint; adding the comment passes.

### Phase 3 — Wire into CI

Add to the `justfile`:

```text
# Audit hand-written ConfigSchema impls per Intention 12A.
audit-configschema:
    @echo "Auditing hand-written impl ConfigSchema for X blocks..."
    cargo xtask audit configschema

# Run all audits (used by CI).
audit-all: audit-configschema
    @echo "All audits passed."
```

If the project gains a top-level CI recipe (currently `docs-check` is the closest analogue), add `audit-all` as a dependency. As of 2026-04-26 there is no `.github/workflows/` directory; CI is invoked by external scripts that call `just`. The packet author should ask the user whether the new recipe should slot into an existing CI recipe or wait for the canonical CI entrypoint to be defined.

**Phase 3 deliverable.** `just audit-configschema` works locally and is wired into whatever CI gate the user names.

### Phase 4 — Documentation

Write `docs/CONFIGSCHEMA_JUSTIFICATION.md` (~80 lines):

- The rule (Intention 12A: derivable or justified).
- The justification-comment format (both forms, the canonical kinds enum, the `Other("...")` escape hatch).
- Example: a hand-written impl with the justification comment.
- The baseline allowlist file: what it is, why it exists, why entries cannot be added without code review.
- The promotion schedule per Q2 (warn-only until <date>, then fail).
- When to expect derive to grow to handle a new case (link to the derive macro file; note that extending the macro is its own packet, motivated by repeated `derive-cannot-handle-X` justifications across the codebase — i.e. when the same exception kind appears N times, that is the signal to extend the macro instead of adding the N+1th hand-write).

Add a one-line link from `steering/INTENTIONS.md` Intention 12A to the new docs page.

**Phase 4 deliverable.** `docs/CONFIGSCHEMA_JUSTIFICATION.md` published; Intention 12A links to it.

### Phase 5 — Audit-as-followup (sibling packet)

Out of scope for this packet but tracked. The followup packet walks each entry in `xtask/data/configschema_baseline.toml` and:

1. Reads the impl. Determines whether it is a legitimate hand-write or a derive-migration candidate.
2. If legitimate: adds a `// CONFIGSCHEMA-JUSTIFICATION: <kind>: <reason>` comment and removes the entry from the baseline.
3. If derive-migratable: either migrates to `#[derive(ConfigSchema)]` (if the macro handles the case as-is) or files a sub-finding to extend the derive macro (if it does not), and removes the entry from the baseline.
4. Verifies `cargo build --workspace` and `cargo test --workspace` after each impl.
5. When the baseline is empty, the file is deleted and the lint becomes universal.

This packet should NOT execute Phase 5. The followup packet is filed separately so a junior can land Phases 1–4 in one half-session. **The baseline allowlist file is deliberately not minimized in this packet** — every existing impl gets an entry, and the followup packet adjudicates each. Pre-judging here would couple the lint's landing to design decisions that belong in the followup.

## Code snippets

### One existing hand-written impl with the justification comment added

This shows what an impl looks like once the followup audit has visited it. **The followup packet edits this file; this packet does not.**

```rust
// In crates/tui-vfx-core/src/mixed_signals_schema.rs:

// CONFIGSCHEMA-JUSTIFICATION: derive-cannot-handle-foreign-type: SignalOrFloat
// is defined in the mixed-signals crate; orphan rules forbid #[derive(ConfigSchema)]
// outside the trait's home crate (tui-vfx-core).
impl ConfigSchema for SignalOrFloat {
    fn schema() -> SchemaNode {
        // … existing body …
    }
}
```

The doc-comment form is also accepted:

```rust
/// Hand-written ConfigSchema for the SignalOrFloat foreign type.
///
/// CONFIGSCHEMA-JUSTIFICATION: derive-cannot-handle-foreign-type
impl ConfigSchema for SignalOrFloat {
    fn schema() -> SchemaNode {
        // … existing body …
    }
}
```

### The lint subcommand's main scan loop

Sketch (not final code; the implementation should respect OFPF prefixes — `fnc_audit_configschema.rs` for the entrypoint, `fnc_scan_file_for_impls.rs` for the per-file scan, etc.):

```rust
// xtask/src/audit/fnc_audit_configschema.rs
//
// Public entrypoint for `cargo xtask audit configschema`.

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

const BASELINE_PATH: &str = "xtask/data/configschema_baseline.toml";
const JUSTIFICATION_MARKER: &str = "CONFIGSCHEMA-JUSTIFICATION:";

pub fn audit_configschema() -> Result<()> {
    let workspace_root = workspace_root()?;
    let baseline = load_baseline(&workspace_root.join(BASELINE_PATH))?;
    let rust_files = collect_rust_files(&workspace_root);

    let mut failures: Vec<Failure> = Vec::new();
    let mut warnings: Vec<Warning> = Vec::new();

    for path in rust_files {
        let source = fs::read_to_string(&path)?;
        for hit in scan_file_for_impls(&path, &source) {
            // Skip macro-body matches (see Risks & gotchas).
            if hit.is_inside_macro_body { continue; }

            let in_baseline = baseline.contains(&path, &hit.type_name);
            let justification = find_justification_above(&source, hit.line);

            match (in_baseline, justification) {
                (true, _) => continue, // grandfathered
                (false, Some(j)) if j.is_canonical() => continue, // properly justified
                (false, Some(j)) => warnings.push(Warning::other_kind(&path, hit, j)),
                (false, None) => failures.push(Failure::missing(&path, hit)),
            }
        }
    }

    report(&failures, &warnings)?;
    if !failures.is_empty() {
        anyhow::bail!("{} unjustified impl(s) found", failures.len());
    }
    Ok(())
}
```

Helper modules `fnc_load_baseline.rs`, `fnc_scan_file_for_impls.rs`, `fnc_find_justification_above.rs` per OFPF. Each ~30–50 LOC.

### The baseline allowlist file format

Already shown under Phase 1 above. The format is deliberately simple — one TOML table per entry with `file` and `type` fields, plus an optional `note`. The lint loads it once at startup and treats it as a `HashSet<(PathBuf, String)>`.

## Test plan

The lint is itself a testable artifact. Tests live at `xtask/tests/test_audit_configschema.rs` (xtask is a binary crate; integration tests under `tests/` are the right home).

**Test 1: a new unjustified impl in a fixture fails the lint.**

```rust
#[test]
fn unjustified_impl_in_new_file_fails() {
    let fixture = make_fixture_workspace(&[
        ("crates/foo/src/lib.rs", r#"
            use tui_vfx_core::{ConfigSchema, SchemaNode};
            pub struct NewType;
            impl ConfigSchema for NewType {
                fn schema() -> SchemaNode { unimplemented!() }
            }
        "#),
    ]);
    let result = run_audit(&fixture);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("NewType"));
}
```

**Test 2: a new impl with the canonical justification passes.**

```rust
#[test]
fn justified_impl_passes() {
    let fixture = make_fixture_workspace(&[
        ("crates/foo/src/lib.rs", r#"
            use tui_vfx_core::{ConfigSchema, SchemaNode};
            pub struct NewType;
            // CONFIGSCHEMA-JUSTIFICATION: derive-cannot-handle-untagged-enum: NewType serializes via serde(untagged).
            impl ConfigSchema for NewType {
                fn schema() -> SchemaNode { unimplemented!() }
            }
        "#),
    ]);
    let result = run_audit(&fixture);
    assert!(result.is_ok());
}
```

**Test 3: an existing impl in the baseline allowlist passes regardless of justification.**

```rust
#[test]
fn baselined_impl_passes_without_justification() {
    let fixture = make_fixture_workspace(&[
        ("crates/foo/src/lib.rs", r#"
            use tui_vfx_core::{ConfigSchema, SchemaNode};
            pub struct LegacyType;
            impl ConfigSchema for LegacyType {
                fn schema() -> SchemaNode { unimplemented!() }
            }
        "#),
        ("xtask/data/configschema_baseline.toml", r#"
            schema_version = 1
            [[entry]]
            file = "crates/foo/src/lib.rs"
            type = "LegacyType"
        "#),
    ]);
    let result = run_audit(&fixture);
    assert!(result.is_ok());
}
```

**Test 4: an impl removed from the baseline allowlist (file deleted) does not false-positive.**

```rust
#[test]
fn baseline_entry_for_deleted_file_does_not_panic() {
    let fixture = make_fixture_workspace(&[
        // No source files. The baseline references a file that doesn't exist.
        ("xtask/data/configschema_baseline.toml", r#"
            schema_version = 1
            [[entry]]
            file = "crates/deleted/src/lib.rs"
            type = "GoneType"
        "#),
    ]);
    let result = run_audit(&fixture);
    // The lint should pass — there is nothing to flag.
    // Optional: emit an info-level message that the baseline has stale entries
    // (the followup-audit packet is the right place to clean those up).
    assert!(result.is_ok());
}
```

**Test 5: macro-body matches are skipped.**

```rust
#[test]
fn macro_body_matches_are_skipped() {
    let fixture = make_fixture_workspace(&[
        ("crates/foo/src/lib.rs", r#"
            macro_rules! impl_thing {
                ($t:ty) => {
                    impl ConfigSchema for $t {
                        fn schema() -> SchemaNode { unimplemented!() }
                    }
                };
            }
            // No real impl outside the macro body.
        "#),
    ]);
    let result = run_audit(&fixture);
    assert!(result.is_ok());
}
```

**Test 6: `Other("...")` justification passes with a warning.**

```rust
#[test]
fn other_justification_passes_with_warning() {
    let fixture = make_fixture_workspace(&[
        ("crates/foo/src/lib.rs", r#"
            // CONFIGSCHEMA-JUSTIFICATION: Other("custom one-off reason")
            impl ConfigSchema for NewType { fn schema() -> SchemaNode { unimplemented!() } }
        "#),
    ]);
    let (result, captured_stderr) = run_audit_capturing_output(&fixture);
    assert!(result.is_ok());
    assert!(captured_stderr.contains("warning"));
    assert!(captured_stderr.contains("Other"));
}
```

**Test 7: unrecognized kind is rejected.**

```rust
#[test]
fn unrecognized_kind_is_rejected() {
    let fixture = make_fixture_workspace(&[
        ("crates/foo/src/lib.rs", r#"
            // CONFIGSCHEMA-JUSTIFICATION: derive-cannot-handle-magic
            impl ConfigSchema for NewType { fn schema() -> SchemaNode { unimplemented!() } }
        "#),
    ]);
    let result = run_audit(&fixture);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("unrecognized kind"));
}
```

**TDD red→green.** Write all seven tests first, run them (all should fail with "audit_configschema not implemented"), then implement the lint until they pass.

## Acceptance criteria

- [ ] `xtask/src/audit/mod.rs` and `xtask/src/audit/fnc_audit_configschema.rs` exist and follow OFPF prefixes + metadata envelopes.
- [ ] `cargo xtask audit configschema` exits 0 on the current tree.
- [ ] `xtask/data/configschema_baseline.toml` checked in with exactly one entry per pre-existing in-tree `impl ConfigSchema for X` (excluding `macro_rules!`-body matches and `recyclebin/`).
- [ ] All seven lint tests pass: `cargo test -p xtask`.
- [ ] `just audit-configschema` recipe added to the justfile and works locally.
- [ ] `docs/CONFIGSCHEMA_JUSTIFICATION.md` published with the format spec, the canonical kinds enum, the promotion schedule, and the link from Intention 12A.
- [ ] `cargo build --workspace` succeeds with zero new warnings.
- [ ] `cargo test --workspace` green.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` clean.
- [ ] No `#[allow]` suppressions added.
- [ ] No inert schema fields added (this packet adds none — pure xtask code).
- [ ] No source crate is touched by Phases 1–4 (verify with `git diff --stat` — only `xtask/`, `docs/`, `justfile`, `steering/INTENTIONS.md`).
- [ ] Lint mode is warn-only per Q2 default. The promotion-to-fail date is named in `docs/CONFIGSCHEMA_JUSTIFICATION.md` and emitted in the warn message.
- [ ] Rustdoc improved on every public item touched per `feedback_rustdoc_when_editing`.
- [ ] `cargo doc --no-deps` succeeds with no broken intra-doc links.
- [ ] **Followup-audit packet filed** — a sibling design doc (`docs/design/tui-vfx-2026-04-26-packet-1.9.A.followup-configschema-audit.md`) exists with the per-impl walk plan. The followup is OUT OF SCOPE for this packet but the file exists so it does not get lost.

## Verification commands

```bash
# Build clean across the workspace.
cargo build --workspace

# Lint runs and exits 0 on the current tree.
cargo xtask audit configschema
echo "exit code: $?"  # Expect: 0

# Lint tests pass.
cargo test -p xtask

# Verify no source crate was touched.
git diff --stat HEAD~5..HEAD -- 'crates/**'
# Expect: empty (or only changes incidental to packet write, e.g. rustdoc audits per feedback_rustdoc_when_editing).

# Verify the baseline file enumerates every in-tree match.
ofpf-content "impl ConfigSchema for" --glob "**/*.rs" 2>&1 | grep -v "recyclebin\|macro_rules\|\$t " | wc -l
grep -c "^\[\[entry\]\]" /usr/projects/tui-vfx/xtask/data/configschema_baseline.toml
# Two numbers should match.

# Workspace clippy clean.
cargo clippy --workspace --all-targets -- -D warnings

# Rustdoc clean.
cargo doc --no-deps

# Manual smoke test — add an unjustified impl and confirm the lint catches it.
# (Don't commit this; it's a manual verification.)
cat >> /tmp/smoke_test.rs << 'EOF'
use tui_vfx_core::{ConfigSchema, SchemaNode};
struct SmokeType;
impl ConfigSchema for SmokeType { fn schema() -> SchemaNode { todo!() } }
EOF
# Place under crates/tui-vfx-core/src/, run `cargo xtask audit configschema`,
# expect failure naming SmokeType. Then delete the test file.
```

## Rollback plan

The packet is structured as four interim commits (one per phase). All are additive — no source crate is touched.

1. If Phase 2 (the lint itself) reveals a deal-breaker (false positives on a real impl, false negatives on a fixture, parser ambiguity in the macro-body detection): stop, do not promote to Phase 3.
2. `git revert <phase-commit-hash>` to back out the lint subcommand. The baseline file from Phase 1 stays — it is informational and harmless on its own.
3. If the lint generates too many false positives at landing (for example, a contributor writes an impl whose justification comment the lint misparses): downgrade to warn-only per Q2 default. The promotion to fail can be deferred. This is the rollback by configuration, not by revert.
4. If the deal-breaker is in Phase 1 (the baseline file format is wrong somehow): edit the format and regenerate. The TOML format is informational; no consumer except the lint itself reads it.
5. `cargo build --workspace` to confirm the restored state compiles.
6. File a finding in the sweep doc capturing what blocked the lint, then surface to the user.

## Risks & gotchas

- **`macro_rules!`-body matches must be skipped.** The two `impl ConfigSchema for $t` lines at `crates/tui-vfx-core/src/schema/mod.rs:28,40` are inside `impl_primitive_schema!` and `impl_int_schema!`. A naïve text grep treats them as 14 separate impls (one per macro invocation: `bool`, `char`, `f32`, `f64`, `i8`, …, `usize`). The detection rule: look for the enclosing `macro_rules! NAME { … }` block. The simplest implementation: if the matched line contains `$t` (or any `$ident`), it is inside a macro body and skipped. A more rigorous implementation parses the file with `syn` and walks the AST, but that is overkill for this gate — the `$t` heuristic catches every case in the current tree and any future case follows the same pattern.

- **Proc-macro-generated impls are invisible to the lint and that is correct.** `#[derive(ConfigSchema)]` expands at compile time to an `impl ConfigSchema for X` block, but the expansion is never written to source files. The lint scans source, so it never sees derive-emitted impls and cannot misfire on them. This is a feature, not a bug — the lint's job is to gate hand-written impls, and derive impls are by definition not hand-written.

- **Generic impls (`impl<T> ConfigSchema for Wrapper<T>`).** The current tree has no top-level generic impls outside the macro at `cls_bindable.rs` (which is `impl ConfigSchema for Never` — no generic on the impl line, even though the wider module is generic). The lint should detect generic impls as a sub-class and route them to the `derive-cannot-handle-generic-T` justification kind; the regex/parser must match `impl ConfigSchema for X` AND `impl<T> ConfigSchema for X<T>` AND `impl<T, S> ConfigSchema for X<T, S>`. The simplest match: `^\s*impl(?:<[^>]+>)?\s+ConfigSchema\s+for\s+`.

- **Impls inside `mod tests` and `#[cfg(test)]` blocks.** The current tree has none, but a future contributor might write a test fixture that needs a `ConfigSchema` impl. The lint should skip impls inside `#[cfg(test)]` blocks AND inside `mod tests { … }` modules. The simplest implementation: walk the file with `syn`, check the surrounding attributes/module context. The fallback heuristic: skip files whose path matches `**/tests/**` or `**/test_*.rs`. This needs the user's call — the heuristic is good enough for now and the strict form can come later.

- **The lint's regex must NOT match `impl<T: ConfigSchema> Foo for T` or `impl ConfigSchema for X { … } // commented out`.** Use a precise regex anchored to the `impl ... ConfigSchema for ...` line, not a substring match. Rust comments are nontrivial — a `///` doc comment containing the literal text `impl ConfigSchema for FakeType` would false-positive a substring match. Pre-strip rustdoc and `//` line comments from the source before scanning, OR use the `syn` parser for full correctness.

- **Path normalization in the baseline file.** The baseline stores paths relative to the workspace root (`crates/tui-vfx-core/src/...`). The lint must normalize OS-specific separators (Windows `\` vs Unix `/`) before comparing. The simplest fix: store and compare paths using `Path::components()`. Do not rely on string equality.

- **The baseline becoming a place to hide regressions.** A contributor could add a new impl AND a baseline entry in the same PR. The grandfather mechanism would silently allow it. **Mitigation:** the baseline file's CODEOWNERS-equivalent should be the schema reviewer (the user, presumably). Any baseline change should require explicit reviewer attention. The lint can also emit a info-level warning whenever the baseline has more than its expected count, so additions surface in CI logs.

- **The followup audit may discover the canonical-kinds enum is wrong.** The current enum (`derive-cannot-handle-foreign-type`, `derive-cannot-handle-generic-T`, etc.) is derived from the existing 12 impls. The followup audit may find a kind not in the enum (e.g., `derive-cannot-handle-trait-object` or `derive-output-too-verbose`). Adding a kind is a one-line patch — the lint should print "unrecognized kind: <X>; please file a packet to extend the enum or use Other(...) as a temporary measure."

- **Pre-existing OFPF noise: this packet adds new files, all of which need the metadata envelope.** Per `~/.claude/CLAUDE.md` the envelope is mandatory. The xtask audit module is `mod_audit.rs` (or `audit/mod.rs`), the entrypoint is `fnc_audit_configschema.rs`, helpers are `fnc_load_baseline.rs`, `fnc_scan_file_for_impls.rs`, `fnc_find_justification_above.rs`. Each carries a header/footer envelope; each has a peer test file under `xtask/tests/` per the OFPF rule "every fnc_ has a test_fnc_".

## Sequencing note

- This packet is **independent**. It does not block or unblock any other queued packet in `docs/design/tui-vfx-2026-04-26-handoff-outstanding.md` §8.4.
- The followup-audit packet (Phase 5) **depends on this packet** but is filed separately. Land this packet first; the followup picks up after.
- Per the handoff doc, this is flagged as the lowest-cost move in the queue. A junior should be able to land Phases 1–4 in one focused half-session.
- No interaction with the V3 schema redesign (`docs/design/tui-vfx-v3-upgrade-plan/`). The lint applies equally to V3 types — it is a workspace-wide friction gate, not a V2-specific or V3-specific check.

<!-- <FILE>docs/design/tui-vfx-2026-04-26-packet-1.9.A-configschema-justification-lint.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
