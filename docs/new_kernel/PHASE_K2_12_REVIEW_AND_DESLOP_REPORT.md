<!-- <FILE>docs/new_kernel/PHASE_K2_12_REVIEW_AND_DESLOP_REPORT.md</FILE> - <DESC>K2.12 formal review and AI de-slop report</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.12 schema lock: record briefing-first review and de-slop findings plus fixes.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture formal review, AI de-slop, applied fixes, and residual risks.</CLOG> -->

# Phase K2.12 Review and De-Slop Report

## Scope

Review and de-slop were scoped to the K2.12 touched surfaces only:

- offender-ledger player/CLI code,
- source.text handled-input and external v3.1 fixture work,
- contract/player CLI tests whose fixture counts changed,
- K2.12 new-kernel docs, index, and vocabulary updates.

Both review agents were explicitly instructed to read `.omx/context/k212-subagent-briefing.md` before task details and not to read or rely on `steering/ORCHESTRATION.md`.

## Third-party review findings

Reviewer summary:

- Directionally solid: `schema-readiness --include-offenders` emits 386 classified offender rows, the source.text fixture validates, and the legacy root remains untouched.
- Requested changes before closeout:
  1. Add this missing review/de-slop artifact because it was referenced by the index and status memo.
  2. Clarify that top-level `summary.ownerAuditRecords` and `summary.unknownRecords` are raw migration-status counters; the K2.12 retirement claim applies to offender rows.
  3. Replace stale K2.11/K2.12 next-packet labels with durable forward-looking decision lanes.
  4. Ensure the external source.text fixture is included in version control.

Applied fixes:

- Added this report artifact.
- Clarified summary-versus-offender wording in K2.12 docs.
- Updated offender `recommendedDisposition` values for runtime, motion, lifecycle, scene, and schema-model rows.
- Updated `recommendedNextPacket` labels to durable decision packets instead of backward-looking phase labels.
- Kept the legacy recipe root clean and tracked the new external v3.1 fixture for commit.

## AI de-slop findings

De-slop summary:

- Remove transient packet vocabulary from fixture metadata.
- Tighten tests to prove exact offender counts, zero generic offender rows, all five former unknown-style paths, and field blockers.
- Avoid order-brittle assertions for source ids.
- Document why offender-level `schemaReadinessBlocking` differs from raw grouped summary semantics.

Applied fixes:

- Changed fixture author metadata to `tui-vfx maintainers`.
- Added exact offender-kind count assertions.
- Added assertions that no offender row uses `ownerAudit` or `unknown`.
- Added checks for all five former unknown-style paths as `contentDescriptor`.
- Added checks for all four field-coverage blocker rows and their exact unsupported fields.
- Replaced the brittle `requiredSourceIds[1]` check with an array-contains helper.
- Added a code comment explaining that offender rows are a schema-lock decision board and keep GUI/backend/descriptor rows blocking until owner holdback/signoff.

## Remaining risks

- `summary.ownerAuditRecords` and `summary.unknownRecords` remain raw migration-status counters for backward continuity. K2.12 normalized the offender rows, not the raw migration summary fields.
- Backend renderer, GUI human-review, descriptor expansion, source/content, runtime dynamism, scene-local pipeline, scope vocabulary, and field-coverage decisions still require owner signoff or follow-up packets before schema readiness can be approved.
- The unrelated untracked `/usr/projects/tui-vfx-recipes/pro/` path is outside this packet and was not touched.

<!-- <FILE>docs/new_kernel/PHASE_K2_12_REVIEW_AND_DESLOP_REPORT.md</FILE> - <DESC>K2.12 formal review and AI de-slop report</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
