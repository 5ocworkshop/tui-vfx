# Work Packets

Purpose: pre-authored, handoff-ready packets for bounded subagent lanes once the briefing experiment converges and we are ready to dispatch optimized agents again.

## How to use these packets
- These are not generic notes; they are intended to be copied or adapted into subagent launches.
- Pair every packet with the current winning briefing structure from the experiment before dispatch.
- Keep one agent per packet unless the packet explicitly says otherwise.
- Keep the agent list clean: once a packet is accepted/committed or rejected/abandoned, close that agent.

## Shared must-read order for subagents
1. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. `/usr/projects/mixed-signals/steering/INTENTIONS.md` when lower math/signal substrate is in scope
4. the active shared briefing in `/usr/projects/gt-design/.omx/context/`
5. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-recipe-vocabulary.md`
6. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
7. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
8. `/usr/projects/global_prompts/standards/60_file_centric_execution.md`
9. `/usr/projects/global_prompts/standards/65_subagent_orchestration.md`
10. the packet-specific file in this directory


## Packet quality reminder
- The quality and detail of these packets will strongly influence the quality and detail of subagent output.
- Packets should assume the assignee is a junior-but-capable engineer who is not deeply familiar with the repo.
- Include enough map/support/direction that the assignee can stay accurate without broadening scope or guessing at boundaries.
- Do not over-specify to the point that the packet is doing the work itself.

## Shared execution expectations
- Start with `ofpf-orientation` on the repos in scope.
- Prefer OFPF tools before broad grep/sed sweeps.
- Keep `task-scope paths` separate from `write scope`.
- Prefer exact path strings and shell-ready verification commands whenever the
  packet is concrete enough to support them.
- Respect repo boundaries:
  - `mixed-signals` = reusable signal/math substrate
  - `tui-vfx` = effect/render semantics
  - `tui-vfx-recipes` = recipe authoring truth, tooling, validator, preview, compiled seams, generated V3 docs
- Use BLOCKER_MODE unless the packet explicitly says FAMILY_MODE.
- Do not widen the assignment to the whole V3 migration.
- If you touch a hot path, call out performance risks against the 16.7 ms / 60 FPS budget.
- Use full paths in reports.

## Packet template
- `/usr/projects/tui-vfx/steering/TASK_PACKET_TEMPLATE.md`

## Packet index
1. `01-v3-schema-docs-freshness.md`
2. `02-briefing-experiment-integration.md`
3. `03-task-packet-template-integration.md`
4. `04-validator-output-stage-schedule-truth.md`
5. `05-debug-recipes-qc-v3.md`
6. `06-v3-docs-source-of-truth-audit.md`
7. `07-madeira-flag-parity-audit.md`
8. `08-madeira-next-slice-plan.md`
9. `09-scene-layer-native-bridge-parity.md`
10. `10-debug-recipe-corpus-normalization-audit.md`
11. `11-motion-disabled-demo-ux.md`
12. `12-native-replay-hot-path-audit.md`
- `13-v3-rules-stage-coverage-expansion.md`
- `14-probe-validator-consistency-audit.md`
- `15-generated-docs-freshness-gate-hardening.md`
- `16-recipe-schema-validator-boundary-audit.md`
- `17-centralized-loader-dispatch-audit.md`
- `18-preview-probe-scheduling-parity-audit.md`
- `19-scene-procedural-determinism-audit.md`
- `20-madeira-scene-semantics-audit.md`
- `21-filter-family-native-only-fixtures-audit.md`
- `22-work-packet-library-maintenance.md`
- `23-madeira-first-implementation-slice.md`
- `24-madeira-second-slice-template.md`
- `25-v3-debug-recipes-filter-family-tranche.md`
- `26-debug-recipes-content-family-tranche.md`
- `27-validator-json-shape-hardening.md`
- `28-v3-tooling-command-reference.md`
- `29-v3-handoffs-and-operator-guides.md`
- `30-codex-spark-doc-task-experiment.md`
- `31-codex-spark-doc-task-runner.md`
- `32-post-experiment-delegation-strategy-refresh.md`
- `33-v3-end-to-end-readiness-audit.md`
- `34-pre-madeira-implementation-checklist.md`
- `35-madeira-scene-semantics-implementation-tranche.md`
- `36-madeira-fireworks-effect-parity-audit.md`
- `37-madeira-fireworks-effect-implementation-tranche.md`
- `38-madeira-validator-probe-truth-hardening.md`
- `39-madeira-reference-fixtures-and-baselines.md`
- `40-madeira-end-to-end-operational-check.md`
- `41-madeira-visual-vetting-protocol.md`
- `42-madeira-performance-and-60fps-audit.md`
- `43-madeira-release-readiness-checklist.md`
- `44-madeira-fully-operational-and-vetted-signoff.md`
