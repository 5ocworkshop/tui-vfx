# Packet 14 — probe/validator consistency audit

## Objective
Audit whether the probe-facing and validator-facing V3 surfaces tell the same truth about the current compiled bridge and timing/model constraints.

## Why this matters
If probe and validator disagree, users get conflicting diagnostics about what V3 can actually do.

## Mode
BLOCKER_MODE

## Success condition
- identify concrete inconsistencies or confirm the surfaces are aligned
- produce one next corrective seam if misalignment exists
- no broad tool rewrite

## In scope
- `/usr/projects/tui-vfx-recipes/src/probe/`
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/`
- relevant V3 compile/bridge helper call sites for evidence only

## Out of scope
- implementing every mismatch found
- runtime render changes unless absolutely required to prove the inconsistency
- recipe corpus edits

## Recommended first steps
1. Identify one representative compiled V3 recipe.
2. Compare what probe reports versus what validator stages report.
3. Note any mismatches in:
   - supported stages
   - fixed-sample caveats
   - reported families
   - rule/render/output claims

## Verification required
- exact commands for one probe run and one validator run on the same recipe
- evidence-backed mismatch list or explicit confirmation of alignment

## Reporting format
Report:
- representative recipe used
- exact commands
- aligned truths
- mismatches
- one recommended next fix lane if needed

## Task reminder
Your task is still: audit consistency between probe and validator surfaces, not rewrite either system wholesale.
