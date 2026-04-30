<!-- <FILE>docs/new_kernel/V31_PARALLEL_MIGRATION_AGENT_BRIEFING.md</FILE> - <DESC>Reusable shared briefing for parallel V2 deprecated to v3.1 debug recipe migration agents</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Parallelize v3.1 vertical migration while preserving V2 oracle parity, one recipe at a time, with strict evidence and boundary discipline.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — define parallel migration lane briefing, validation steps, write-scope rules, and reporting contract.</CLOG> -->

# v3.1 Parallel Debug Recipe Migration Agent Briefing

## Non-negotiable context

- Work only in `/usr/projects/tui-vfx` and `/usr/projects/tui-vfx-recipes`.
- Do **not** read, quote, or send `steering/ORCHESTRATION.md`.
- Do **not** touch `/usr/projects/tui-vfx-recipes/pro/`.
- v3.1 is pre-release. Do **not** bump schema version.
- V2 `_DEPRECATED_` debug recipes are the canonical oracle. Ignore V3 if it conflicts.
- Use durable human-readable names. Do not put transient packet/phase shorthand in public names, fields, schema values, variables, or report vocabulary.
- Report successful results first. Supporting detail follows.

## Must-read documents before action

Read these first, in this order:

1. `.omx/context/v31-vertical-migration-briefing-latest.md`
2. `docs/new_kernel/V31_PARALLEL_MIGRATION_AGENT_BRIEFING.md` (this file)
3. `docs/new_kernel/V31_VERTICAL_MIGRATION_VALIDATION_PROCESS.md`
4. `docs/new_kernel/V31_RENDERING_BOUNDARY_RULES.md`
5. `steering/INTENTIONS.md`
6. `steering/OFPF-TOOLS.md`
7. `steering/TASK_PACKET_TEMPLATE.md`
8. `steering/work-packets/COMMON_EXECUTION_RULES.md`
9. Applicable `../global_prompts/standards/*.md`, especially tooling preamble, OFPF, TDD, file-centric execution, subagent orchestration, metadata headers, and recycle-bin rules.

## Role of a parallel migration agent

You own a disjoint recipe group. Your job is to convert V2 `_DEPRECATED_` debug recipes into v3.1 debug recipes with visual/detail parity and evidence.

You are **not** the global architect. Do not redesign the schema, player, compositor, or studio. If you find a code/backend gap, document it precisely and continue with other recipes in your group.

## Per-recipe loop

For each assigned recipe:

```text
1. Inspect V2 oracle JSON.
2. Capture V2 evidence with recipe-probe at meaningful phase/sample points.
3. Inspect existing v3.1 target recipe if present.
4. Decide whether the target is:
   - migrated and validated,
   - needs recipe-only correction,
   - blocked by descriptor/backend/player support,
   - intentionally deferred because it is complex/outside current primitive lane.
5. If recipe-only correction is safe, edit only the assigned v3.1 recipe file.
6. Validate through the real v3.1 player/compositor backend.
7. Record exact commands and evidence.
8. Move to the next recipe.
```

Do not horizontally complete a whole layer before proving individual vertical recipe evidence.

## Required V2 oracle evidence

Use `/usr/projects/tui-vfx-recipes` tooling. Start with:

```bash
cd /usr/projects/tui-vfx-recipes && cargo run -q -p recipe-probe -- recipes/debug_recipes/<dir>/_DEPRECATED_<name>.json --phase <entering|dwelling|exiting> --sample-t <t> --cells all --with-causation --format json
```

Capture enough evidence to compare deterministically:

- title, description, message text;
- width, height, border type, border trim;
- lifecycle enter/dwell/exit durations;
- base foreground/background and any expected channel changes;
- active phase(s);
- visible rows or styled-cell/color/glyph evidence;
- letter-cell counts and foreground/background class counts for styles/filters/shaders;
- visible-cell counts for masks;
- row displacement for samplers/content motion.

## Required v3.1 validation evidence

Use strict native compositor mode and fail on fallback:

```bash
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-cli -- render-backend --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/<relative>.json --descriptor-pack descriptors/v3.1/packs/primitive.json --backend compositor --composition-mode native --fail-on-fallback --format json --phase <enter|dwell|exit> --phase-t <t>
```

Check:

- `errors=[]`
- `fallbackUsed=false`
- `nativeLoweringSucceeded=true`
- no `unsupportedNativeEffect`
- expected `compositionSpecSummary` counts
- rows/styled cells match V2 oracle at the sampled point

If strict-native fails because backend support is missing, do **not** fake success. Record:

```text
BLOCKED: <recipe>
- Unsupported effect/input/scope/output: ...
- Exact command and diagnostic
- V2 oracle evidence
- Proposed owner-level patch, if clear
```

## Boundary rules

Follow `docs/new_kernel/V31_RENDERING_BOUNDARY_RULES.md`:

- Contract/recipe owns durable authored intent.
- Player owns sampled lifecycle/source/value resolution.
- Backend lowerer owns backend-specific translation and honest rejection.
- Compositor owns reusable render operations.
- UI/CLI/studio present evidence and controls; they must not reimplement effect semantics.

Do not add schema fields for backend convenience. Do not silently drop authored semantics.

## Parallel write-scope discipline

Your assignment will name exact recipe files or directories. Stay inside them.

Allowed by default:

- Assigned v3.1 recipe files in `/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/...`.
- A lane-specific report file if explicitly assigned, normally under `.omx/reports/` or `docs/new_kernel/` with a unique lane name.

Not allowed unless explicitly assigned:

- Shared Rust source files.
- Shared Rust test files.
- Descriptor pack edits.
- Schema files.
- Index files.
- Any file in `/usr/projects/tui-vfx-recipes/pro/`.

If a shared code/test/descriptor edit is required, report the blocker to the leader instead of freelancing.

## Testing expectations

For recipe-only changes, run at least:

```bash
python3 -m json.tool <changed-recipe>.json >/tmp/<recipe>.json.ok
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-cli -- render-backend ... --composition-mode native --fail-on-fallback ...
git -C /usr/projects/tui-vfx-recipes diff --check -- <changed-recipe>.json
```

If you touch Rust with explicit permission, use TDD: failing regression first, green implementation, refactor/de-slop, nextest, clippy, fmt, diff-check.

## Output/reporting contract

Return results, not vague summaries. Start with successful results.

Required final structure:

```text
## Successful results
- recipe A migrated/validated: evidence summary
- recipe B already matched: evidence summary

## Blocked recipes
- recipe C: exact blocker and next action

## Files changed
- path: change summary

## Commands and evidence
- exact command: PASS/FAIL + key output

## Recommendations
- concrete next leader action(s)
```

If no recipe in your group can be safely migrated, the successful result is a precise blocker map with V2 evidence and next actions.

<!-- <FILE>docs/new_kernel/V31_PARALLEL_MIGRATION_AGENT_BRIEFING.md</FILE> - <DESC>Reusable shared briefing for parallel V2 deprecated to v3.1 debug recipe migration agents</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
