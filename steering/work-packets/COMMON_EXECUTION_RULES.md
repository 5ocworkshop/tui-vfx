# Common Execution Rules For Work Packets

These rules apply to every packet in this directory unless the packet explicitly narrows or overrides something.

## Role and quality bar
Assume the assigned subagent is a junior-but-capable engineer:
- be explicit
- do not expect them to infer repo boundaries correctly from vibe alone
- do not assume they will naturally stay narrow without written out-of-scope bullets
- require concrete verification and reporting

## gpt-5.5 low lane lessons
When dispatching an unroled `gpt-5.5` low agent:
- keep the packet concrete and narrower than a normal frontier packet
- do not assign a role if the owner requested the raw model/effort lane
- include exact write scope and explicit non-goals; this lane can execute well
  but should not be asked to infer architecture boundaries from broad plans
- include one canonical input fixture or JSON snippet for schema/parser packets
  so the agent does not need to infer the intended authored shape
- name tricky semantic edge cases as tests, especially coordinate-frame,
  timing, cache, and schema-default behavior
- for schema/parser packets, require separate tests for acceptance, unknown-field
  rejection, default application, validation boundaries, and downstream typed
  propagation
- for runtime packets, name likely player/preview/probe entrypoints and ask the
  lane to prove they do not silently drop the new feature
- when reduced-motion, timing, or cache policy is relevant, require the lane to
  identify the existing source of truth or explicitly report that no runtime
  option exists yet
- require a final self-check for OFPF line pressure, test placement, and whether
  filtered test commands actually exercised the new tests
- expect the leader to rerun verification, review semantics, and make small
  integration fixes before accepting the work
- one mid-flight status check after localized compile failures is useful; repeated
  interruptions are unnecessary when the agent is clearly converging

## OFPF rules
- Start with `ofpf-orientation --root <repo>` for each repo in scope.
- Use OFPF reads first: orientation, hotspots, inspect, focus, around.
- Keep reads surgical before broad file reads.
- Do not drift into broad cleanup while on a blocker packet.

## TDD / regression discipline
- If behavior is subtle or easy to regress, prefer adding or tightening a focused regression test before broad edits.
- If tests already exist for the seam, extend the narrowest relevant one instead of adding broad snapshot churn.

## Reporting contract
Every final report should include:
- docs read confirmation
- 3 short reflection bullets
- exact task-scope paths used for grounding
- changed files (full paths)
- exact commands run
- pass/fail outcome per command
- blocker or handoff notes
- performance risks noticed

## File metadata discipline
When touching files that carry file-level metadata headers:
- keep `<CLOG>` / `// <CLOG>` entries to 1-2 short lines
- summarize only the latest/current file change
- do not append a running history; git is the history
- keep `<WCTX>` focused on durable work context unless the file's role changes

## Scope split rule
Every packet should make a clear distinction between:
- `task-scope paths` = the files/areas the assignee must ground on
- `write scope` = the smallest justified set of files they may actually edit

Do not rely on repo names alone when the packet can support exact path strings.
Do not rely on conceptual “run tests” language when the packet can support exact
shell-ready verification commands.

## Preferred packet shape
The preferred packet structure is:
1. task first
2. why this matters
3. success condition
4. mode
5. task-scope paths for grounding
6. exact write scope
7. explicit out-of-scope items
8. must-read docs
9. repo-boundary guardrails
10. first steps / grounding instructions
11. exact verification commands
12. reporting contract
13. closing task reminder

## Performance checklist
If the packet touches runtime/render paths, explicitly check for:
- repeated per-cell allocations
- repeated context rebuilding inside nested loops
- recomputation that can be hoisted
- normalized-time vs elapsed-time confusion in cadence-bearing code

## Key-binding checklist
If the packet touches keyboard bindings:
- inspect the current binding/handler/help surfaces first
- do not assume a requested key is free just because the task says to use it
- if there is a collision, choose an alternate key and report:
  - the collision
  - the alternate key
  - why the alternate is safer
- update help text and visible keybinding references with the code change

## Debug recipe rules
If a packet explicitly includes debug recipes, enforce:
- clear description text
- body text that is concise and useful
- adequate size/contrast/layout to show the effect
- no filler wording
- no misleading effect naming

## Commit/handoff expectation
The subagent does not commit directly to mainline history. The leader reviews, integrates, commits, and then closes the agent.
