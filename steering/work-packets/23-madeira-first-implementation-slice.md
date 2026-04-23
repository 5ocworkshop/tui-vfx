# Packet 23 — Madeira first implementation slice

## Task first
Implement exactly the first Madeira slice chosen by the audit/plan work, and nothing broader.

## Objective
Implement the first concrete Madeira-facing gap identified by the Madeira audit/plan packets once the audit work is complete.

## Why this matters
The Madeira work only becomes real once we start closing one gap at a time. This packet is the first execution lane after the audit/planning packets establish the exact seam.

## Mode
BLOCKER_MODE

## Prerequisites
Do not dispatch this packet until:
- Packet 07 (Madeira parity audit) is complete
- Packet 08 (Madeira next-slice plan) is complete
- the chosen slice is explicitly named

## Success condition
- one Madeira gap is closed
- exact tests/validator/probe evidence are green for that slice
- no broad Madeira redesign

## Task-scope paths for grounding
- only the exact files named by Packet 08
- only the chosen first slice

## Exact write scope
- only the exact files named by Packet 08
- only the narrowest proof/test surfaces tied to that slice

## Out of scope
- all other Madeira gaps
- broad effect tuning
- unrelated scene/runtime cleanup

## Must-read docs in order
1. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
2. relevant Madeira audit results
3. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
4. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
5. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
6. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`

## Repo-boundary guardrails
- Do not broaden from the chosen slice into “general Madeira improvement.”
- Keep unrelated fireworks/text/scene gaps out unless they are explicitly part of the chosen slice.

## Verification required
Use the exact commands identified by Packet 08 and also re-run:
- `cargo test -p tui-vfx-recipes load_v3_compiled_loads_madeira_flag_recipe -- --nocapture`
- `cargo test -p tui-vfx-recipes load_v3_document_reads_madeira_flag_recipe -- --nocapture`
- one representative validator/probe command if the slice affects those surfaces

## Reporting format
Report:
- exact gap closed
- exact files changed
- exact proof commands
- remaining Madeira gaps left untouched

## Task reminder
Your task is still: implement one Madeira slice, not “finish Madeira” in one jump.
