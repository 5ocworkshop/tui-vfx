# Packet 30 — codex-spark doc-task experiment design

## Task first
Design and stage a separate doc-only Codex-spark experiment for evaluating doc-oriented task quality, then record the candidate tasks, scoring rubric, and expected failure modes for future trial execution.

## Why this matters
Recent cycles converged on packet structure quality for mixed execution lanes, but the same structure has not been proven against **doc-only** workloads where codex-spark is often the likely model. This packet isolates that model-fit question before broad application.

## Success condition
By the end of this packet, there is a complete experiment design artifact that:
- is fully doc-oriented (no code execution)
- defines a fixed candidate-task set with exact paths
- defines scoring and failure-mode checks that are specific enough to compare spark results against other models
- is explicit that this is a design task only (no trial execution in this packet)

## Mode
BLOCKER_MODE

## Objective
Create a clean experiment design for `codex-spark` on doc tasks using a narrow, reusable protocol. Candidate tasks should be sourced from steering, experiment, and packet surfaces so results reflect documentation comprehension and execution-safety quality, not domain implementation skill.

## Experimental structure

### 1) Scope of this experiment
- **Scope:** packet and protocol quality for doc-only subagent dispatch, grounded on files already in steering/experiment documentation.
- **Exclusion:** no implementation, no runtime validation, no product code, no broad orchestration rewrites.

### 2) Candidate doc tasks (for future trial execution)
The following 6 tasks are the default candidate set for codex-spark in this experiment:

1. **Doc structure critique + rewrite proposal**
   - Design target: `/usr/projects/tui-vfx/steering/experiments/subagent-briefing-experiment-packet.md`
   - Allowed action: propose a minimal, concrete structure edit (no file execution required for this packet; execution by runner later).

2. **Packet template clarity pass**
   - Design target: `/usr/projects/tui-vfx/steering/TASK_PACKET_TEMPLATE.md`
   - Allowed action: produce a cleaned candidate packet section order and explicit verification language.

3. **Packet library hygiene audit**
   - Design target: `/usr/projects/tui-vfx/steering/work-packets/README.md`
   - Allowed action: identify and tighten doc-only onboarding and required must-read consistency language.

4. **Experiment-readme consistency pass**
   - Design target: `/usr/projects/tui-vfx/steering/experiments/subagent-briefing-experiment-protocol.md`
   - Allowed action: verify that the experimental protocol and packet template language align with the protocol’s fixed question set and scoring expectations.

5. **Scope-boundary checklist pass**
   - Design target: `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`
   - Allowed action: rewrite one boundary paragraph to make scope split (`task-scope` vs `write scope`) explicit.

6. **Candidate-lane simulation prompt**
   - Design target: `/usr/projects/tui-vfx/steering/work-packets/30-codex-spark-doc-task-experiment.md`
   - Allowed action: simulate the next lane with a one-shot doc-oriented rewrite plan, constrained to `tui-vfx/steering` docs only.

### 3) Fixed protocol (required in every cycle)
1. Read required grounding set in the defined order before any design recommendation.
2. State task-first boundary in one sentence.
3. Name exact in-scope files and exact out-of-scope boundaries.
4. Include concrete repo-boundary guardrails.
5. Include at least one runnable command checklist if and only if the task naturally names a command.
6. End with a closing task reminder that prevents widening.

### 4) Scoring rubric (0–2 each)
Evaluate each candidate task result against these dimensions:

| Dimension | 0 (Fail) | 1 (Partial) | 2 (Strong) |
|---|---|---|---|
| **Task comprehension** | misses the assignment or changes domain | captures assignment but blurs lane or doc-only constraint | states exact assignment and preserves constraints |
| **Scope precision** | wrong/missing file paths | partial paths or inferred paths | exact path list in scope statement |
| **Boundary enforcement** | broadens to runtime/prod work | mentions bounds but still drifts | strictly enforces explicit out-of-scope lines |
| **Verification grounding** | no validation plan | generic checks without path/command basis | concrete command/file checks or explicitly justified "not applicable" |
| **Output quality** | vague, generic prose | somewhat structured but noisy or ambiguous | concise, copy-ready, and unambiguous |
| **Protocol fidelity** | violates required order/format | follows some but not all required steps | follows full required order and packet format |

- **Pass threshold per task:** 10/12 or above.
- **Experimental pass threshold:** at least 4/6 tasks at pass threshold **and** no more than one task below 6/12.

### 5) Expected failure modes and mitigations
- **Mode drift into execution:** result starts proposing code edits in `/usr/projects/tui-vfx-recipes/src` or other runtime areas.
  - *Mitigation:* include an automatic boundary review line and score cap if runtime paths appear.
- **Path guessing:** helper infers file scope not supported by packet intent.
  - *Mitigation:* award at most 1 on scope precision unless exact paths are enumerated.
- **Verification vagueness:** generic “run tests” without concrete checks.
  - *Mitigation:* require explicit commands or explicit “not applicable” with reason.
- **Read-order loss:** required docs read in order not respected.
  - *Mitigation:* include mandatory read-order confirmation section.
- **Task-completion confusion:** design claims a full trial execution instead of design artifact.
  - *Mitigation:* hard gate: design-only status must be stated in Task first and reporting section.

### 6) Failure capture format
For each cycle, append one short audit row under:
- `task_id`: one of `T1..T6`
- `pass_score_total`
- `weakest_dimension`
- `scope_drift`
- `verification_gaps`
- `recommended_packet_change`

Use exact paths in that row when failures are tied to file scope.

## Exact write scope
- Only this file:
  - `/usr/projects/tui-vfx/steering/work-packets/30-codex-spark-doc-task-experiment.md`

## Out of scope
- running the spark trial itself
- creating or changing non-doc protocol artifacts in this packet
- any product/runtime code changes
- any broad orchestration rewrites (including `ORCHESTRATION.md`) from this packet
- any task outside the doc-oriented set listed above unless a follow-up packet redefines scope

## Must-read docs in order
1. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
2. experiment learnings in `/usr/projects/tui-vfx/steering/experiments/` (especially existing convergence artifacts)
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. `/usr/projects/tui-vfx/steering/TASK_PACKET_TEMPLATE.md`
5. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`
6. `/usr/projects/tui-vfx/steering/experiments/subagent-briefing-experiment-protocol.md`
7. `/usr/projects/global_prompts/standards/65_subagent_orchestration.md`

## Repo-boundary guardrails
- **Doc surfaces only**: `/usr/projects/tui-vfx/steering/**` and `/usr/projects/tui-vfx/steering/experiments/**`
- **No runtime code**: especially no changes in `/usr/projects/tui-vfx/recipes/**`, `/usr/projects/tui-vfx/src/**`, `/usr/projects/tui-vfx-recipes/**`, `/usr/projects/mixed-signals/**`

## Verification required
This packet is design-only; verify by static check only:
- `sed -n '1,260p' /usr/projects/tui-vfx/steering/work-packets/30-codex-spark-doc-task-experiment.md`
- Confirm all sections are present and paths are explicit
- Confirm no non-doc files are named for editing in write scope

## Reporting contract for design artifact review
Final review from this packet must include:
- complete experiment design and candidate set
- scoring rubric and thresholds
- failure-mode list and mitigation
- exact write scope confirmation
- exact file path confirmation for every candidate task

## Closing task reminder
Your task is still: complete this **doc-only codex-spark experiment design** in this file and keep it intentionally separate from running the trial.
