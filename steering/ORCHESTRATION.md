<!-- <FILE>steering/ORCHESTRATION.md</FILE> - <DESC>Leader-facing orchestration protocol for routing subagents, preparing bounded work packets, and preserving OFPF steering discipline across long-running tui-vfx work.</DESC> -->
<!-- <VERS>VERSION: 0.1.1</VERS> -->
<!-- <WCTX>Clarify one-time subagent grounding so follow-on packet prompts do not trigger repeated global orientation.</WCTX> -->
<!-- <CLOG>0.1.1: PATCH — clarify that accepted `READY FOR WORK PACKET` grounding satisfies later packet grounding requirements.</CLOG> -->

# Orchestration Protocol

Purpose: persistent instructions for how the leader agent should manage subagents and work packets during long-running versioned recipe/compositor migration work, especially after context compaction or reset.

## 1. Leader vs subagent split

The leader agent owns:
- architecture
- sequencing
- integration
- verification interpretation
- commits / final reporting
- steering alignment

Subagents own bounded lanes only:
- tightly scoped implementation slices
- focused audits
- doc cleanup lanes
- recipe cleanup lanes
- targeted validation/research lanes

Do not outsource core architectural judgment or final verification.

## 2. Mandatory grounding before packet assignment

Never assign a work packet to a newly launched subagent as its first task.
Launch the subagent with a single grounding assignment:

```text
Read and follow /usr/projects/tui-vfx/steering/SUBAGENT-GROUNDING.md.
Complete the grounding report and stop with READY FOR WORK PACKET.
Do not begin implementation or packet work until the leader assigns a packet.
```

Grounding is one-time per subagent session. If the same subagent has already
completed `/usr/projects/tui-vfx/steering/SUBAGENT-GROUNDING.md`, reported
`READY FOR WORK PACKET`, and the leader accepted that report, later packet
prompts must treat shared grounding as satisfied. Do not ask that agent to rerun
the global grounding pass. The packet should instead tell the agent to confirm
prior grounding, read only packet-specific docs, and proceed with targeted OFPF
inspection for the assigned lane. If the agent cannot confirm prior grounding, or
if context loss makes the prior report unavailable, the leader should refresh or
rerun grounding before assigning packet work.

This removes duplicate must-read lists from the process. The grounding file is
the single source for project goals, philosophy, coding standards, OFPF practice,
and vocabulary gating before packet work. Work packets then carry only
packet-specific scope, docs, verification, and reporting instructions.

The leader must wait for the grounding report and check that it is substantive:
the agent should name the documents read, summarize applicable constraints,
show high-value OFPF practice commands, and report any unreadable document as a
blocker or verification gap. A pro-forma "read the docs" acknowledgement is not
enough.

Document-flow model:

```text
┌─────────────────────────────────────────────────────────────────────┐
│ LEADER / ORCHESTRATION VIEW                                         │
│ steering/ORCHESTRATION.md                                           │
│                                                                     │
│ Defines: new subagents ground first; grounded subagents do not       │
│ repeat global grounding; packets carry task-specific context only.   │
└───────────────────────────────┬─────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────┐
│ NEW SUBAGENT?                                                       │
└───────────────┬───────────────────────────────────────┬─────────────┘
                │ yes                                   │ no / reused
                ▼                                       ▼
┌───────────────────────────────────────┐     ┌──────────────────────────────┐
│ GROUNDING ASSIGNMENT ONLY             │     │ CHECK PRIOR GROUNDING         │
│ steering/SUBAGENT-GROUNDING.md        │     │                              │
│                                       │     │ Has this subagent already:    │
│ Agent reads global project context:   │     │ - completed grounding         │
│ - project goals and philosophy        │     │ - reported READY FOR WORK     │
│ - coding / TDD / OFPF rules           │     │   PACKET                     │
│ - repository boundaries               │     │ - had leader acceptance       │
│ - vocabulary guidance                 │     └──────────────┬───────────────┘
│                                       │                    │
│ Agent stops with:                     │                    ▼
│ READY FOR WORK PACKET                 │       ┌────────────────────────────┐
└───────────────────┬───────────────────┘       │ If yes: do not repeat      │
                    │                           │ global grounding. Continue │
                    ▼                           │ to packet-specific docs.   │
┌───────────────────────────────────────┐       └──────────────┬─────────────┘
│ LEADER ACCEPTS GROUNDING REPORT       │                      │
│                                       │                      │
│ Checks report is substantive, not a   │                      │
│ ceremonial acknowledgement.           │                      │
└───────────────────┬───────────────────┘                      │
                    │                                          │
                    └──────────────────┬───────────────────────┘
                                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│ WORK PACKET DISPATCH                                                │
│ steering/work-packets/<packet>.md                                   │
│ or steering/TASK_PACKET_TEMPLATE.md-derived packet                  │
│                                                                     │
│ Packet says: confirm accepted grounding; do not repeat it; read     │
│ packet-specific docs only; inspect task-scope paths with OFPF; stay │
│ inside write scope; run exact verification commands.                │
└───────────────────────────────┬─────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────┐
│ COMMON EXECUTION RULES                                              │
│ steering/work-packets/COMMON_EXECUTION_RULES.md                     │
│                                                                     │
│ Applies reusable packet rails: grounding status reporting, OFPF      │
│ first, scope split, exact verification, hot-path and handoff rules. │
└───────────────────────────────┬─────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────┐
│ SUBAGENT EXECUTES PACKET                                            │
│                                                                     │
│ Final report includes grounding confirmation, packet-specific docs, │
│ changed files, commands run, pass/fail, blockers, and risks.        │
└───────────────────────────────┬─────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────┐
│ LEADER REVIEW / VERIFY / ACCEPT                                     │
│                                                                     │
│ Leader owns integration, final verification, semantic review,        │
│ commit, packet archival, and agent closure.                         │
└─────────────────────────────────────────────────────────────────────┘
```

Leader note:
- this ORCHESTRATION file is primarily leader-facing
- subagents should not normally be told to read it directly
- instead, the leader should distill the relevant orchestration rules into
  `SUBAGENT-GROUNDING.md`, the active briefing, and the concrete task packet

## 3. Orientation first

`SUBAGENT-GROUNDING.md` owns the initial OFPF practice pass. Follow-on packet
prompts own only packet-specific context. After a grounded agent receives a work
packet, use OFPF tools first for packet-specific exploration and targeted reads.
Fall back to narrow `rg` / `sed` / `jq` only when OFPF is unavailable or
insufficient.

## 4. Work packet format

Each subagent packet should include, in this preferred order:
1. task first (one-sentence assignment)
2. why this matters
3. success condition
4. mode (`BLOCKER_MODE` / `FAMILY_MODE` when relevant)
5. exact task-scope paths used for grounding
6. exact write scope
7. explicit out-of-scope items
8. packet-specific extra docs only; do not repeat global grounding docs
9. repo-boundary guardrails
10. first steps / grounding instructions
11. verification expected (exact shell-ready commands when possible)
12. reporting contract
13. closing task reminder

Default current dispatch stance:
- choose a role-specialized subagent surface that matches the task shape instead
  of suppressing roles to protect a model choice
- OMX default agent profiles now align with the desired `gpt-5.5` lanes, so rely
  on role routing plus the requested reasoning effort rather than suppressing the
  role field or adding stale explicit model overrides
- omit explicit model overrides unless the owner explicitly asks for one or the
  current repo/runtime profile contract requires it
- use `reasoning_effort: low` for tightly bounded doc, lookup, and small
  mechanical lanes; `medium` for normal implementation/test lanes; and `high`
  for architecture, safety, or ambiguity-heavy lanes
- keep the task packet narrower than a normal frontier packet whenever the lane
  is implementation-heavy but concrete: exact write scope, exact non-goals,
  exact tests, and exact edge-case probes are required
- expect good in-scope execution and honest reporting, but leader review must
  still check semantic edge cases, file-size/OFPF pressure, and whether tests
  landed in the intended harness
- prefer sending one mid-flight status check after the first compile/test failure
  report; do not repeatedly interrupt if the agent is making localized progress
- after the agent returns, run the verification locally and do at least one
  leader-owned edge-case review before accepting the lane
- include one canonical JSON/fixture shape when the packet touches parser,
  schema, or DTO propagation; otherwise the agent may pass the concept but miss a
  plan-required default or boundary assertion
- keep up to 5 useful lanes busy when independent work exists

Work packet quality rule:
- write packets for a junior-but-capable engineer who is not deeply familiar
  with the project
- packet quality is the first lever; improve packet specificity before jumping
  to a bigger model
- the experiment showed that exact path strings and exact verification wording
  are what keep bounded lanes stable, so name them whenever the docs support
  them
- include enough detail, map, guardrails, and verification direction that the
  assignee can stay accurate without guessing at repo boundaries or widening
  scope
- do not over-specify to the point that the packet is doing the work itself
- explicitly separate:
  - `task-scope paths` = the files/areas the agent grounded on to understand the
    problem
  - `write scope` = the smallest justified set of files they may actually edit
- require exact path strings rather than repo-name-only summaries whenever the
  packet can support that level of specificity
- require shell-ready verification commands rather than conceptual “run tests”
  summaries whenever the packet can support that level of specificity
- if the packet cannot justify an exact path or command yet, say that it is not
  concrete enough instead of inventing one
- if the packet names exact verification commands, the assignee should run or
  report those exact commands rather than swapping in broader substitutes
- when in doubt, add:
  - clearer in-scope / out-of-scope bullets
  - explicit repo-boundary reminders
  - explicit first steps
  - exact verification commands
  - a closing task reminder
- packets that authorize edits to metadata-bearing files must include the file
  metadata rule: `<CLOG>` / `// <CLOG>` is only 1-2 short lines about the latest
  file change, not a running history; git is the history
- for `gpt-5.5` low implementation packets, add explicit self-checks for:
  - non-obvious coordinate-frame or timing semantics named by the plan
  - OFPF soft/hard line pressure before final report
  - whether test filters actually ran the new tests
  - any pre-existing dirty files so the helper does not claim ownership of
    unrelated changes
  - whether parser/schema tests cover both acceptance and defaults, not just
    propagation through later phases

Post-experiment evidence trace:
- `/usr/projects/tui-vfx/steering/experiments/subagent-briefing-experiment-results.md`
  showed packet quality, exact pathing, and exact verification wording were the
  main drivers of stable bounded-lane performance
- `/usr/projects/tui-vfx/steering/experiments/model-compare/mini-medium-results.md`
  showed the smaller helper was unreliable on underspecified packets and became
  stable once the lane, scope, and command contract were made concrete
- `/usr/projects/tui-vfx/steering/experiments/model-compare/gpt54mini-medium-results.md`
  showed the corrected mini-medium setup is the best default for concrete
  bounded lanes, including blocker-scoped validator/tooling work
- `/usr/projects/tui-vfx/steering/experiments/model-compare/gpt54-medium-results.md`
  and `/usr/projects/tui-vfx/steering/experiments/model-compare/frontier-low-results.md`
  showed frontier helpers stay strong on harder boundary judgment but tend to
  need more prompting pressure to stay exact on paths, commands, and final
  recommendation shape
- codex-spark operational learnings from the doc-task experiment family remain
  stable: use spark for low-context doc surfaces only, not for runtime lanes or
  broad repo-context work
- first `gpt-5.5` low Packet 1 for V3 per-cell motion was strong enough to use
  again: it stayed in scope, read the required docs, produced passing tests, and
  reported honestly. Leader review still found a semantic coordinate-frame issue
  (`local_frame` origin vs returned scene-local coordinates) and OFPF/test
  placement cleanup. Lesson: this lane is effective when the packet spells out
  tricky semantics as direct tests, but final acceptance remains a leader-owned
  integration/review step.
- second `gpt-5.5` low Packet 2 reinforced the pattern: it cleanly carried typed
  cell-motion data through recipes DTO/normalization/compile and ran the right
  docs check, but leader review added the missing parser-default assertion. For
  schema packets, tasking should include a canonical JSON snippet plus a checklist
  for accept/reject/default/propagation tests.
- third `gpt-5.5` low Packet 3 showed that implementation lanes can find the
  real runtime seam when tasking permits narrow repo-boundary adjustment: root
  runtime integration lived in `tui-vfx-recipes`, not the compositor crate. For
  runtime packets, name likely preview/player/probe surfaces and explicitly ask
  for reduced-motion source discovery plus remaining tooling touchpoints, rather
  than assuming the engine repo owns the whole path.

Current role/task routing after the OMX profile refresh:
- use `explore` or an equivalent fast lookup role for read-only file, symbol,
  relationship, or codebase-map questions
- use `executor` for concrete implementation, cleanup, and refactor lanes with
  clear file ownership
- use `test-engineer` for test-first design, coverage gaps, fixtures, and flaky
  verification hardening
- use `debugger` for regression isolation and root-cause analysis before edits
- use `architect` for multi-system tradeoffs, contract boundaries, and durable
  API/schema decisions
- use `critic` or `code-reviewer` for challenge/review lanes where independent
  skepticism materially reduces risk
- use `writer` for documentation, migration notes, status memos, and handoff
  artifacts
- use `verifier` for completion evidence and claim validation after integration
- select reasoning effort by risk and ambiguity; improve packet specificity
  before escalating effort
- never suppress role selection merely to preserve `gpt-5.5`; role profiles
  are now the supported way to reach the correct model/effort lane

Grounding-first rule:
- all newly launched subagents complete `SUBAGENT-GROUNDING.md` first and stop
  with `READY FOR WORK PACKET`
- once the leader accepts that report, later packet prompts treat shared
  grounding as complete; do not make the same subagent repeat the global pass
- if the packet is testing comprehension or prompt quality, ask questions only
  after that stop point
- do not score or rely on answers from a helper that skipped the grounding stop
  point


Version precision:
- Treat version labels as scope. `V3`, `v3.1`, and V2 are different surfaces.
- If the active packet is v3.1, subagent prompts and reports must say `v3.1` / `V3.1`, not generic `V3 pipeline` or `V3 migration`, unless explicitly discussing historical V3 artifacts.
- If an inherited steering doc uses older generic V3 wording, the leader must translate it into the active packet's version-specific scope instead of copying the generic wording into the briefing.

## 5. Path discipline

The orchestration root is `/usr/projects/gt-design`, but active implementation may be primarily in:
- `/usr/projects/tui-vfx`
- `/usr/projects/tui-vfx-recipes`
- `/usr/projects/mixed-signals`

Use full paths in briefing docs, task packets, and reports whenever ambiguity is possible.

## 6. Performance rule

Always watch for non-performant patterns in touched scopes.
Target budget: 16.7 ms per frame for 60 FPS.

Flag or avoid:
- unnecessary nested loops on hot paths
- repeated allocations in per-cell/per-frame work
- recomputation that can be hoisted or cached
- avoidable string/Vec churn in render-time code
- unnecessary delays or synchronization on active playback paths

Subagents should call out hot-path risks even in audit lanes.

## 6A. Key-binding safety rule

When a task touches keyboard bindings:
- first inspect the current key map / handlers / help text before assigning a
  new binding
- do this even if the task instructions suggest a specific key
- if the requested key is already in use, do not silently override it
- choose an alternate key that minimizes collisions and report:
  - the collision you found
  - the alternate key you chose
  - why that alternate was safer
- update help text and any on-screen binding references together with the code
- treat keybinding collisions as a correctness issue, not a cosmetic one

## 7. Debug-recipe rules

When touching debug recipes:
- keep them polished and professional
- ensure the effect is clearly visible
- ensure message text fits the rect cleanly
- include description text explaining what the viewer should see
- use the standard body-text pattern:
  - line 1: category + human-readable effect name
  - line 2: concise behavioral cue
- do not create separate recipe categories just because a recipe is runtime-bound or procedural
- directory structure should follow what the recipe demonstrates, not merely how a value is driven

When updating a shader/filter/mask/sampler/style/content effect or other
primitive/effect family:
- update the corresponding debug recipe(s) in the same tranche whenever the
  visual semantics, naming, timing, authoring vocabulary, or parameter story
  are affected
- normalize the family in one pass when possible:
  - shared math substrate
  - timing semantics
  - vocabulary/comments/naming
  - rustdocs/public docs
  - generated-doc/schema metadata and drift checks when public or
    schema-bearing surfaces change
  - matching debug recipes/descriptions/layouts
- when repeated reusable math patterns are discovered, apply the boundary rule:
  - if the math is signal/math substrate
  - and it is reusable across 3+ real use cases
  - and it is not inherently tied to one renderer/effect semantic
  - then move it into `mixed-signals` instead of re-rolling it locally
- if the pattern is effect/render semantics, keep it in `tui-vfx` /
  `tui-vfx-recipes` and only extract the lower reusable math
- ensure those debug recipes remain high-quality references with:
  - clear descriptions
  - correct body text
  - adequate layout/contrast
  - representative timing/variation
- treat this as part of the definition of done for that family update, not as
  optional follow-up polish

## 8. Timing / architecture rules currently in force

- cadence-driven motion should use monotonic elapsed time, not reset-on-loop normalized time
- keep normalized phase/loop progress available separately
- do not use recipe-period hacks as architectural fixes
- move reusable math/signal substrate lower into `mixed-signals`
- keep effect semantics in `tui-vfx` / `tui-vfx-recipes`
- when a truly different geometric model appears, add an explicit new basis rather than mutating old primitive semantics
- treat missing rustdocs, missing generated-doc inputs, stale debug recipes, or
  unvalidated schema drift as incomplete pipeline-family work

## 9. Repo-state expectations

During active library migration:
- GTD may be temporarily non-compiling
- do not treat GTD compile failures as blocking if they are downstream fallout from in-progress upstream library changes
- prioritize making the libraries and recipe pipeline correct first

## 10. Reporting expectations

Subagent final reports must include:
- explicit note that `SUBAGENT-GROUNDING.md` was completed before packet work,
  plus any packet-specific docs read
- changed files
- concrete evidence / commands run
- blockers or recommended next handoff
- any performance risks noticed

Leader summaries should stay short, concrete, and evidence-based.

## 10A. Merge/commit discipline

When agent work is accepted into the main working tree:

- commit that accepted merge promptly rather than letting many accepted merges
  accumulate uncommitted
- use one commit per accepted workset when practical so recovery/review stays
  clean
- include a brief user-facing summary of what was achieved/completed when the
  merge is accepted
- if work is not yet acceptable, send it back for refinement instead of mixing
  it into the main line prematurely
- once accepted work is committed, close the finished agent to free the slot

Work-packet lifecycle rule:
- do not treat a packet as complete just because an agent response looked good
- a packet becomes complete only after the related work is actually committed on
  `master`
- once the related commit is visible on `master`, remove or archive the packet
  from the active work-packet queue so the live backlog reflects real remaining
  work

## 11. Refresh rule after context reset

If context is compacted or partially lost, reread:
1. `/usr/projects/tui-vfx/steering/ORCHESTRATION.md`
2. current active shared briefing doc
3. relevant steering/design docs for the active lane

Then continue from the latest verified state rather than restarting discovery from scratch.


## 12. Queue discipline

The leader should keep useful agent lanes queued so agent slots stay productive.

Rules:
- always have the next bounded task packet ready before an agent finishes when possible
- prioritize reviewing and responding to agent completions over continuing long local work
- do direct work in the gaps between agent reviews, not at the expense of leaving completed agents idle
- if local work is more than a step or two from a natural pause, stop and switch focus when agents finish so the queue keeps moving
- review completed agent work promptly
- accept and integrate good work quickly
- send work back for refinement when it does not yet meet steering or verification standards
- once a workset is accepted and committed (or clearly no longer needed), close the agent to free the slot
- prefer keeping up to 5 useful bounded lanes busy rather than letting completed agents linger idle

## 13. Recursive self-improvement loop

The leader should periodically improve the orchestration approach itself rather
than treating prompts/briefings as fixed.

Cadence:
- after roughly every 5 completed work packets, pause for a short retrospective
- also run the retrospective sooner if there is obvious prompt drift, repeated
  cleanup, repeated scope creep, or repeated verification gaps

What to review:
- which task packets came back cleanly and why
- which packets needed correction and why
- whether the task scope was too broad or too vague
- whether architectural boundary rules were too buried
- whether performance guidance was too generic for the hot path
- whether the agent needed more explicit out-of-scope instructions
- whether the verification contract was specific enough
- whether the ordering of prompt sections helped or hurt

What to adjust:
- wording
- ordering
- level of detail
- definition-of-done bullets
- out-of-scope bullets
- verification checklist
- explicit “if you see X, do Y” rails
- prompt templates for recurring task families

Preferred task-packet ordering for bounded lanes:
1. task first
2. why this matters
3. success condition
4. mode
5. task-scope grounding paths
6. exact write scope
7. explicit out-of-scope items
8. packet-specific extra docs
9. repo-boundary reminders
10. first steps / grounding instructions
11. hot-path/performance reminders
12. required verification
13. reporting contract
14. closing task reminder

Institutionalization rule:
- when a retrospective yields a durable lesson, update this file and, if
  currently relevant, the active shared briefing document
- do not keep repeating the same orchestration mistake across cycles if the
  lesson can be captured once here

Quality rubric for reviewing medium-agent output:
- correctness of the main claim
- adherence to write scope
- respect for architecture boundaries
- verification quality
- performance awareness
- reporting clarity

Escalation rule:
- if the same type of issue appears in 2 or more recent packets, treat it as a
  briefing/prompt design failure and fix the orchestration materials before
  dispatching more similar work

<!-- <FILE>steering/ORCHESTRATION.md</FILE> - <DESC>Leader-facing orchestration protocol for routing subagents, preparing bounded work packets, and preserving OFPF steering discipline across long-running tui-vfx work.</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.1</VERS> -->
