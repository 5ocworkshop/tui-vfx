# Common Execution Rules For Work Packets

These rules apply to every packet in this directory unless the packet explicitly narrows or overrides something.

## Workspace / copy discipline — applies to every packet
A leader-provided repository checkout or git worktree is already the assigned workspace. Subagents must use that workspace directly. They must not create nested clones, nested git worktrees, project copies, or crate copies as a substitute for scope control.

This is a general rule for all work packets, not only compost work. If a packet needs a new isolated workspace, the leader creates it before dispatch and names it explicitly. The packet still gives exact write-scope files/modules inside that workspace.

## Role and quality bar
Assume the assigned subagent is a junior-but-capable engineer:
- be explicit
- do not expect them to infer repo boundaries correctly from vibe alone
- do not assume they will naturally stay narrow without written out-of-scope bullets
- require concrete verification and reporting
- require proof that grounding docs were read and followed, not merely listed

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
- expect the leader to rerun verification, review semantics, review the touched-file list every time, and make small
  integration fixes before accepting the work
- one mid-flight status check after localized compile failures is useful; repeated
  interruptions are unnecessary when the agent is clearly converging

## Shared grounding handoff
`/usr/projects/tui-vfx/steering/SUBAGENT-GROUNDING.md` owns the global grounding
read list, OFPF practice pass, and `READY FOR WORK PACKET` report. Do not copy
that read list into packets.

Packet execution starts from one of two states:
- accepted grounding exists for this subagent session: confirm it briefly, do not
  repeat it, then read only packet-specific docs;
- accepted grounding cannot be confirmed: stop and report that the leader must
  run or refresh grounding before packet work continues.

## OFPF rules
- Use OFPF reads first for packet-specific inspection: orientation, hotspots,
  inspect, focus, around.
- Keep reads surgical before broad file reads.
- Do not drift into broad cleanup while on a blocker packet.
- For tui-vfx-compost lanes, the active target is the existing `crates/tui-vfx-compost` tree. A leader-provided git worktree is already the isolated checkout. Do not tell agents to copy the repo, copy `tui-vfx-compositor`, copy any abandoned copied-crate path, create nested clones, or create nested/per-slice worktrees. Migrate vertically in the assigned worktree/files only; historical copied-crate work is reference/recovery material, not a reusable packet instruction.

## TDD / regression discipline
- If behavior is subtle or easy to regress, prefer adding or tightening a focused regression test before broad edits.
- If tests already exist for the seam, extend the narrowest relevant one instead of adding broad snapshot churn.
- Preserve V2 fallback/oracle paths until owner-approved removal. For v3.1
  migration work, treat current recipe artifacts as reference/exploratory
  evidence unless a vertical slice explicitly owns them.

## Reporting contract
Every final report should include:
- shared grounding confirmation (`READY FOR WORK PACKET` previously accepted, or
  fresh grounding completed for this packet)
- packet-specific docs read confirmation
- 3 short reflection bullets
- exact task-scope paths used for grounding
- changed files (full paths)
- every file created or edited by the packet, with origin/action marked as `edited-existing`, `new-authored`, `copied`, `moved`, or `generated`
- exact commands run
- pass/fail outcome per command
- blocker or handoff notes
- performance risks noticed

## Touched-file list review
Every subagent final report must include a touched-file list. For each path created or edited during the packet, state whether the file was `edited-existing`, `new-authored`, `copied`, `moved`, or `generated`. If no files were changed, say `Touched files: none`. This applies even when a copied/generated file is later edited heavily. The leader must review this list every time before acceptance or integration; any unexpected or inappropriate file, copied source, generated artifact, broad-scope edit, or unexplained origin is a blocker until corrected.

## File metadata discipline
When touching files that carry file-level metadata headers:
- keep `<CLOG>` / `// <CLOG>` entries to 1-2 short lines
- summarize only the latest/current file change
- do not append a running history; git is the history
- keep `<WCTX>` focused on durable work context unless the file's role changes

## Required file-tree briefing
Every implementation/refactor packet must include the intended file tree for the assigned slice and an approximate expected file-name breakdown before work begins. The breakdown should name likely modules/files, note which files are expected edits vs new files, and state that deviations must be reported before broad edits. This is required because file layout is part of the architecture, not an implementation afterthought.

## Scope split rule
Every packet should make a clear distinction between:
- `task-scope paths` = the files/areas the assignee must ground on
- `write scope` = the smallest justified set of files they may actually edit

Do not rely on repo names alone when the packet can support exact path strings.
Exact write scope is enforced inside the assigned workspace; it is never a request to create another workspace or copy.
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
7. expected file tree / approximate file-name breakdown
8. explicit out-of-scope items
9. packet-specific extra docs
10. repo-boundary guardrails
11. first steps / grounding instructions that confirm prior shared grounding
    instead of repeating it
12. exact verification commands
13. reporting contract
14. closing task reminder

## Performance checklist
If the packet touches runtime/render paths, explicitly check for:
- repeated per-cell allocations
- repeated context rebuilding inside nested loops
- recomputation that can be hoisted
- normalized-time vs elapsed-time confusion in cadence-bearing code
- file-size/OFPF pressure: target roughly 300 LOC where practical; files above
  500 LOC need a split plan or written cohesion justification

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
