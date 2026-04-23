# Packet 32 — post-experiment delegation strategy refresh

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

## In scope
- orchestration strategy docs only
- packet library references as needed

## Out of scope
- running new experiments
- runtime code

## Verification required
- evidence trace from experiment results to strategy updates

## Reporting format
Report the recommended model/task-class matrix and any changes needed to the permanent orchestration docs.
