# Packet 59 — V3 rustdoc gap closure for schema-bearing APIs

## Task first
Close the highest-value rustdoc gaps on the current V3 schema-bearing API surfaces so generated V3 docs stop carrying obvious `No rustdoc summary` holes.

## Why this matters
The V3 docs/autogen path exists and is live, but `docs/generated/V3_API.md` still exposes multiple `No rustdoc summary` entries on important compile/runtime/schema-bearing items. That weakens the whole doc-generation contract the project says is release-gating.

## Success condition
- one bounded tranche of high-value V3 schema-bearing items gains meaningful rustdoc
- regenerated V3 API docs show those gaps closed
- no unrelated behavior changes are smuggled into the doc pass

## Mode
FAMILY_MODE

## Task-scope paths for grounding
- `/usr/projects/tui-vfx/steering/INTENTIONS.md`
- `/usr/projects/tui-vfx-recipes/docs/generated/V3_API.md`
- `/usr/projects/tui-vfx-recipes/tools/fnc_generate_v3_docs.py`
- `/usr/projects/tui-vfx-recipes/src/v3/`
- the exact high-value schema-bearing items currently lacking rustdoc summaries

## Exact write scope
- the chosen V3 schema-bearing source files missing rustdoc
- regenerated V3 API docs if needed
- the smallest generator-touching seam only if required for freshness/checking

## Out of scope
- broad prose rewrite across the whole workspace
- unrelated runtime logic changes
- non-V3 public API cleanup outside the chosen tranche

## Must-read docs in order
1. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
2. `/usr/projects/tui-vfx-recipes/docs/generated/V3_API.md`
3. `/usr/projects/tui-vfx-recipes/tools/fnc_generate_v3_docs.py`
4. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
5. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
6. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
7. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`

## Verification required
- `python3 tools/fnc_generate_v3_docs.py --write` or `--check`
- a before/after statement showing the chosen `No rustdoc summary` gaps are closed
- focused build/tests if touched files require them

## Task reminder
Your task is still: close one bounded tranche of V3 rustdoc gaps, not rewrite every doc comment in the workspace.
