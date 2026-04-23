# Packet 32 — post-experiment delegation strategy refresh

## Task first
Refresh the delegation strategy using experiment evidence, not hunches.

## Objective
Refresh the overall delegation strategy after the briefing/model experiments complete, using the evidence from all experiment lanes and the packet library.

## Why this matters
Once the experiments finish, we should update the real deployment strategy for which models/packet styles are used for which task classes.

## Mode
BLOCKER_MODE

## Prerequisites
- main briefing experiment complete
- model-comparison experiment(s) complete
- any spark-doc experiment results available if run

## Success condition
- one updated delegation strategy exists
- task classes are matched to model/controller/helper choices
- packet-library usage rules are updated if needed

## Task-scope paths for grounding
- orchestration strategy docs only
- packet library references as needed

## Exact write scope
- the smallest orchestration strategy doc surface needed to capture the new matrix
- packet-library references only if directly needed

## Out of scope
- running new experiments
- runtime code

## Must-read docs in order
1. completed experiment result files
2. `/usr/projects/tui-vfx/steering/ORCHESTRATION.md`
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. `/usr/projects/tui-vfx/steering/TASK_PACKET_TEMPLATE.md`
5. `/usr/projects/global_prompts/standards/65_subagent_orchestration.md`

## Repo-boundary guardrails
- This is orchestration/process work only.
- Do not rerun experiments from this packet.

## Verification required
- evidence trace from experiment results to strategy updates

## Reporting format
Report the recommended model/task-class matrix and any changes needed to the permanent orchestration docs.

## Task reminder
Your task is still: refresh the delegation strategy from evidence, not to invent a new orchestration philosophy.
