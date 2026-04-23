# Packet 24 — Madeira second-slice template

## Task first
Select the next single Madeira slice after Packet 23, using the current landed-state evidence and the latest audit follow-ons.

## Objective
Define the immediate next bounded Madeira execution lane after Packet 23 has landed, without broadening into roadmap planning.

## Why this matters
Packet 23 proved one slice is green; Packet 24 names the next bounded follow-up so implementation can continue without ambiguity.

## Mode
BLOCKER_MODE

## Prerequisites
- Packet 23 complete
- Packet 23 verification green
- latest post-Madeira implementation packets landed and indexed in status:
  - `/usr/projects/tui-vfx/steering/work-packets/STATUS.md`
  - `/usr/projects/tui-vfx/steering/work-packets/README.md`

## Success condition
- one next follow-up slice is selected and framed clearly
- exact files and proof commands are named
- chosen slice is explicitly the next logical step after the latest landed Madeira-adjacent work

## Task-scope paths for grounding
- packet 23 output and verification references
- status ledger for completed post-Madeira work
- most recent Madeira-focused follow-on outcomes

## Exact write scope
- none by default; this is a planning packet
- if you create a small planning note, keep it narrow and explain why

## Out of scope
- broad restructuring
- multiple slices at once

## Must-read docs in order
1. relevant Madeira implementation/audit outputs
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
5. `/usr/projects/tui-vfx/steering/work-packets/61-madeira-reference-repo-faithfulness-audit.md`
6. `/usr/projects/tui-vfx/steering/work-packets/STATUS.md`

## Repo-boundary guardrails
- Choose one next slice only.
- Do not turn this into a broad Madeira roadmap rewrite.

## Verification required
- exact commands named
- evidence that the next slice depends on what landed since Packet 23
- explicit, current reference to one concrete next packet path

## Reporting format
Report:
- exact next slice name and number
- short rationale tied to latest completed/created packets
- proof commands/evidence references that justify the dependency chain

## Task note (post-Madeira refresh)
Current next slice is:

- **`61-madeira-reference-repo-faithfulness-audit.md`** (`/usr/projects/tui-vfx/steering/work-packets/61-madeira-reference-repo-faithfulness-audit.md`)

Rationale: Packet 23 opened the Madeira execution lane, packets 48 and 52 completed the highest-priority effect/performance follow-ups identified in-session, and packet 61 is the next bounded audit that directly validates whether accumulated Madeira work now matches the reference behavior before any further broad Madeira slice is dispatched.

## Suggested evidence commands
1. `sed -n '1,260p' /usr/projects/tui-vfx/steering/work-packets/STATUS.md`
2. `sed -n '1,260p' /usr/projects/tui-vfx/steering/work-packets/DAG.md`
3. `sed -n '1,260p' /usr/projects/tui-vfx/steering/work-packets/61-madeira-reference-repo-faithfulness-audit.md`
4. `sed -n '1,260p' /usr/projects/gt-design/docs/superpowers/handoffs/2026-04-23-v3-session-audit-synthesis.md`
