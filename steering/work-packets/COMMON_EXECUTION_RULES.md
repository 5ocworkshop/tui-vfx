# Common Execution Rules For Work Packets

These rules apply to every packet in this directory unless the packet explicitly narrows or overrides something.

## Role and quality bar
Assume the assigned subagent is a junior-but-capable engineer:
- be explicit
- do not expect them to infer repo boundaries correctly from vibe alone
- do not assume they will naturally stay narrow without written out-of-scope bullets
- require concrete verification and reporting

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
- changed files (full paths)
- exact commands run
- pass/fail outcome per command
- blocker or handoff notes
- performance risks noticed

## Performance checklist
If the packet touches runtime/render paths, explicitly check for:
- repeated per-cell allocations
- repeated context rebuilding inside nested loops
- recomputation that can be hoisted
- normalized-time vs elapsed-time confusion in cadence-bearing code

## Debug recipe rules
If a packet explicitly includes debug recipes, enforce:
- clear description text
- body text that is concise and useful
- adequate size/contrast/layout to show the effect
- no filler wording
- no misleading effect naming

## Commit/handoff expectation
The subagent does not commit directly to mainline history. The leader reviews, integrates, commits, and then closes the agent.
