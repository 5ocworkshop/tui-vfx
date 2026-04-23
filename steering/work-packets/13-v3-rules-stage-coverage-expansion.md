# Packet 13 — V3 rules-stage coverage expansion

## Objective
Expand V3 `--rules` coverage in `pipeline-validator` beyond the first restored supported path so the rules stage gives useful stage-specific evidence across more representative V3 cases.

## Why this matters
We restored the existence of a real V3 rules stage. The next step is to prove it behaves correctly across more than one happy-path compiled recipe and one parse-failure case.

## Mode
BLOCKER_MODE

## Success condition
- add focused rules-stage coverage for at least a small representative set of V3 cases
- keep tests narrow and stage-specific
- do not widen into output/render stage work

## In scope
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/fnc_validate_v3_compiled_recipe.rs`
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/tests/test_v3_compiled_bridge.rs`
- if needed, one nearby validator support file in the same tool only

## Out of scope
- output-stage changes
- runtime/preview behavior changes
- generated docs/schema export
- debug recipe corpus edits

## Recommended first steps
1. Inspect the existing V3 rules-stage tests.
2. Identify 2–4 additional representative categories, for example:
   - supported compiled V3 fixture with scene layers
   - supported compiled V3 fixture with non-trivial scope/use of normalized validation
   - V3 input that loads/normalizes but fails explicit normalized validation
3. Add only the narrowest tests needed.

## OFPF guidance
- `ofpf-orientation --root /usr/projects/tui-vfx-recipes`
- focus validator tool files first
- avoid broad repo reads

## Verification required
- exact targeted `pipeline-validator` tests you add/touch
- one representative CLI `--rules` JSON run if it proves stage shape

## Reporting format
Report:
- which rule-stage scenarios are now covered
- exact files changed
- exact commands run
- remaining uncovered rules-stage classes worth a future packet

## Task reminder
Your task is still: expand V3 rules-stage confidence with narrow validator tests, not redesign validator staging.
