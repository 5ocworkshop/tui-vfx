# Packet 27 — validator JSON shape hardening

## Task first
Stabilize one concrete JSON-shape weakness in validator/probe outputs so downstream tooling can rely on it more safely.

## Objective
Audit and tighten the machine-readable JSON output shape for validator/probe surfaces so downstream tooling and experiments can rely on it more safely.

## Why this matters
As more of our workflow depends on machine-readable validator/probe results, shape drift or inconsistent field semantics becomes costly.

## Mode
BLOCKER_MODE

## Success condition
- one clear JSON-shape inconsistency or weakness is fixed
- tests prove the intended stable shape
- no broad redesign of the whole CLI

## Task-scope paths for grounding
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/`
- `/usr/projects/tui-vfx-recipes/tools/recipe-probe/` when directly relevant
- JSON-focused tests

## Exact write scope
- the smallest validator/probe JSON-shape seam involved
- the narrowest related test file(s)

## Out of scope
- recipe content
- non-JSON text-output polishing
- unrelated validator stage logic unless needed to stabilize the JSON contract

## Must-read docs in order
1. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
2. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
3. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
4. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
5. `/usr/projects/global_prompts/standards/60_file_centric_execution.md`

## Repo-boundary guardrails
- Keep this lane focused on JSON-shape stability.
- Do not broaden into unrelated CLI UX or stage redesign.

## Verification required
- targeted JSON-shape tests
- one representative CLI JSON run showing the intended shape

## Reporting format
Report the old ambiguous shape, the new stabilized shape, and exact affected consumers if known.
