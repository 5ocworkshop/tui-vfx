# Packet 23 — Madeira first implementation slice

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

## In scope
- only the exact files named by Packet 08
- only the chosen first slice

## Out of scope
- all other Madeira gaps
- broad effect tuning
- unrelated scene/runtime cleanup

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
