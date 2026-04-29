<!-- <FILE>docs/new_kernel/PHASE_K2_13_REVIEW_AND_DESLOP_REPORT.md</FILE> - <DESC>Review and anti-slop closure report for K2.13 schema decision burn-down</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.13 schema decision burn-down: record formal review, deslop findings, fixes, and fresh verification.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document third-party review and anti-slop closure for K2.13.</CLOG> -->

# K2.13 review and anti-slop closure report

## Scope

This report covers the K2.13 v3.1 `debug_recipes/` schema-readiness work executed from `ARCH-RESP-TO-PHASE_K2_12.md`, including the later `ARCH-RESP-TO-PHASE_K2_13.md` extension memo folded into the same packet.

The active source corpus remained `/usr/projects/tui-vfx-recipes/recipes/debug_recipes`; `_DEPRECATED_` records were treated as raw oracle-only evidence rather than active migration blockers.

## Formal review findings

### Code review

The post-fix code review initially requested changes:

- Workspace-wide `cargo check --workspace` failed in `tui-vfx-next` after `ScopeEvalInput` and `ValueSource::SampledField` expanded.
- Grouped blocker rows still exposed generic blocking buckets even though the summary had disposition-based readiness fields.
- Schema-readiness accepted rows carried legacy `defer...Decision` recommendation text.

Fixes applied:

- Updated `tui-vfx-next` proof code and tests for the expanded `ScopeEvalInput` and `ValueSource::SampledField` shapes.
- Reused the same offender classification for grouped blockers so final blocker rows no longer report generic `ownerAudit`, `unknown`, or deprecated owner-policy buckets as readiness blockers.
- Changed offender `recommendedDisposition` to mirror final dispositions instead of legacy defer vocabulary.

### Anti-slop review

The anti-slop pass requested:

- Validate `sampledField.field` instead of accepting arbitrary strings.
- Preserve non-target channels for shader `applyTo` instead of resetting them to defaults.
- Clarify the later K2.13 memo's provisional vocabulary against final report dispositions.

Fixes applied:

- Added explicit sampled-field name validation and tests.
- Updated linear-gradient style application to preserve existing foreground/background on non-target channels.
- Added a terminology note to `ARCH-RESP-TO-PHASE_K2_13.md` that maps provisional memo terms to final report dispositions.

## Fresh verification evidence

- `cargo check --workspace` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo nextest run --workspace --no-fail-fast` — 2821 passed, 0 failed.
- `cargo xtask docs check` — passed with the three existing generated-doc warnings for missing TOML/ai-hint coverage.
- `schema-readiness --include-offenders` over the active debug corpus:
  - `totalLegacyRecords`: 603
  - `offenders`: 383
  - `canDeclareSchemaReady`: true
  - `unresolvedSchemaBlockers`: 0
  - `remainingOwnerDecisionCount`: 0
  - grouped blocking blockers: 0
  - generic grouped blockers (`ownerAudit`, `unknown`): 0
  - contradictory `acceptedSchema`/`defer...Decision` rows: 0

## Remaining risks

- Descriptor and adapter backlog remains real work; schema readiness now classifies it explicitly rather than pretending migration is visually complete.
- Backend renderer and GUI/human-review holdbacks are accepted holdbacks, not implemented renderer capability.
- `source.ansi`, `source.image`, and `source.procedural` descriptors are schema/descriptor commitments; full runtime source execution remains future work.
- `sampledField` is intentionally narrow (`surfaceAngleFrom`) until additional sampled fields are explicitly accepted.
