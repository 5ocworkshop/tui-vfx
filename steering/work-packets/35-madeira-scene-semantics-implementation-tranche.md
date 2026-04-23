# Packet 35 — Madeira scene semantics implementation tranche

## Objective
Implement one bounded Madeira scene-semantics gap after the Madeira audits identify the highest-priority missing or degraded scene behavior.

## Why this matters
Madeira will not be truly operational until scene placement/composition semantics match the intended authored recipe behavior, not just the validator bridge happy path.

## Mode
BLOCKER_MODE

## Prerequisites
- Packet 07 complete
- Packet 08 complete
- Packet 20 complete if it identified a more precise scene-semantic seam
- the chosen scene-semantic tranche is explicitly named before dispatch

## Success condition
- one concrete scene-semantic gap is closed
- exact affected Madeira behavior is proven with focused evidence
- no broad scene-system rewrite

## In scope
- only the exact scene/compile/render files named by the prior Madeira audit packet
- only the one tranche explicitly chosen before dispatch
- the narrowest tests/probes needed to prove the fix

## Out of scope
- fireworks/effect tuning outside the selected scene seam
- unrelated scene-layer cleanup
- broad Madeira implementation
- debug recipe corpus work unless directly required as proof

## Recommended first steps
1. Restate the exact chosen scene-semantic gap.
2. Identify the minimum file set that controls that seam.
3. Find the narrowest existing deterministic or probe tests that should lock the behavior.
4. Patch the seam only after the proof shape is clear.

## Verification required
Use the exact commands named by the prerequisite audit/plan packets and also re-run the relevant Madeira smoke proofs, for example:
- `cargo test -p tui-vfx-recipes load_v3_compiled_loads_madeira_flag_recipe -- --nocapture`
- `cargo test -p tui-vfx-recipes load_v3_document_reads_madeira_flag_recipe -- --nocapture`
- one focused deterministic/probe/validator proof command tied to the fixed scene-semantic gap

## Performance note
Scene-bearing fixes can introduce hidden grid-copy or allocation costs. Call out any per-frame/per-layer cost increase explicitly.

## Reporting format
Report:
- exact scene-semantic gap fixed
- exact files changed
- exact proof commands
- any remaining Madeira scene-semantic gaps left untouched

## Task reminder
Your task is still: close one specific Madeira scene-semantic gap, not “make Madeira complete.”
