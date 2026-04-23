# Packet 31 — codex-spark doc-task trial runner

## Task first
Run the approved codex-spark doc-only trials and record quality against the agreed rubric without broadening into runtime work.

## Objective
Execute the doc-oriented codex-spark experiment once the design is approved.

## Why this matters
This packet is the execution counterpart to the spark doc experiment design.

## Mode
BLOCKER_MODE

## Prerequisites
- Packet 30 complete
- spark experiment approved

## Success condition
- one or more doc-only spark trials are run
- quality is scored against the agreed rubric
- results are recorded clearly enough to compare against non-spark models

## Task-scope paths for grounding
- only the approved spark doc experiment artifacts

## Exact write scope
- only the approved spark experiment artifacts and result logs

## Out of scope
- runtime/library work
- broad packet redesign during execution

## Must-read docs in order
1. approved spark experiment design
2. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
3. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
4. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
5. `/usr/projects/global_prompts/standards/65_subagent_orchestration.md`

## Repo-boundary guardrails
- This is experiment execution only.
- Do not treat trial doc tasks as a license to drift into code work.

## Verification required
- exact trial log
- exact outputs reviewed
- scored results

## Reporting format
Report each spark trial, its quality score, and recommendation on whether spark is worth using for that task class.

## Task reminder
Your task is still: run the approved spark doc-only trials and log the results, not improvise new spark experiments or widen into runtime work.

## Trial execution log
- date: 2026-04-23
- model observed: `gpt-5.3-codex-spark` (reasoning effort low)
- execution mode: prompt-only trials, doc-only outputs (no file edits)

### Trial calls run
- spawn an assistant (`gpt-5.3-codex-spark`, low reasoning effort) for each trial:
  - T1 target: `/usr/projects/tui-vfx/steering/experiments/subagent-briefing-experiment-packet.md`
  - T2 target: `/usr/projects/tui-vfx/steering/TASK_PACKET_TEMPLATE.md`
  - T3 target: `/usr/projects/tui-vfx/steering/work-packets/README.md`
  - T4 target: `/usr/projects/tui-vfx/steering/experiments/subagent-briefing-experiment-protocol.md`
  - T5 target: `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`
  - T6 target: `/usr/projects/tui-vfx/steering/work-packets/30-codex-spark-doc-task-experiment.md`

### Exact outputs reviewed
- **T1:** proposed edits adding a doc-only scope guard and a reader-model cue.
- **T2:** full copy-ready template reordering with explicit pass/fail command reporting.
- **T3:** 3 onboarding + must-read consistency edits for packet hygiene.
- **T4:** alignment pass with two partials and 3 protocol edits.
- **T5:** replacement paragraph cleanly defining task-scope vs write scope.
- **T6:** one-shot lane plan with exact in-scope/out-of-scope paths.

### Scored results (Packet 30 rubric, 0–2 each, total /12)

| Trial | Comp. | Scope | Boundary | Verify | Quality | Protocol | Total | Weakest | Recommendation |
|---|---:|---:|---:|---:|---:|---:|---|---|
| T1 | 2 | 1 | 1 | 1 | 2 | 1 | 8/12 | boundary/scoping | Useful but too loose on explicit boundaries. |
| T2 | 2 | 2 | 1 | 2 | 2 | 2 | 11/12 | boundary | Strong draft; add explicit non-edit boundary lines. |
| T3 | 2 | 1 | 2 | 1 | 2 | 1 | 9/12 | scope / verification | Useful ideas but target/task alignment noise reduces trust. |
| T4 | 2 | 2 | 2 | 1 | 2 | 2 | 11/12 | verification | High utility; asks for stricter protocol scoring capture. |
| T5 | 2 | 2 | 2 | 1 | 2 | 2 | 11/12 | verification | Clear scope split, but no explicit check list. |
| T6 | 2 | 2 | 2 | 2 | 2 | 1 | 11/12 | protocol format | Good plan; not output in fixed protocol structure. |

**Aggregate:** 6 trials run. **Pass threshold 10/12:** 4/6 pass.

### Pass/fail recommendation by task class
- Spark is **worth using** for doc-only rewrite ideation and structure passes.
- Spark is **not sufficient alone** for protocol-compliance scoring tasks; require post-review for boundary and verification completeness.

### Packet 30 log row format (copy-ready)
- `task_id=T1`, `pass_score_total=8`, `weakest_dimension=scope precision`, `scope_drift=False`, `verification_gaps=no explicit command/check rubric`, `recommended_packet_change=pin task-scope/write-scope language`
- `task_id=T2`, `pass_score_total=11`, `weakest_dimension=boundary enforcement`, `scope_drift=False`, `verification_gaps=explicit non-edit boundary`, `recommended_packet_change=add immutable out-of-scope line`
- `task_id=T3`, `pass_score_total=9`, `weakest_dimension=scope precision`, `scope_drift=True`, `verification_gaps=no verification rubric`, `recommended_packet_change=lock target path exactly in task prompt`
- `task_id=T4`, `pass_score_total=11`, `weakest_dimension=verification grounding`, `scope_drift=False`, `verification_gaps=question-level score trace`, `recommended_packet_change=mirror fixed/adaptive answer schema`
- `task_id=T5`, `pass_score_total=11`, `weakest_dimension=verification grounding`, `scope_drift=False`, `verification_gaps=scope examples`, `recommended_packet_change=add one scope-violation example`
- `task_id=T6`, `pass_score_total=11`, `weakest_dimension=protocol fidelity`, `scope_drift=False`, `verification_gaps=needs fixed response format`, `recommended_packet_change=require fixed question-like fields even for simulation tasks`
