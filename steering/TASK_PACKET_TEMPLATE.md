# Task Packet Template

Purpose: reusable template for bounded subagent packets and follow-on tranches.

Use this template when dispatching a real subagent lane. Fill in the bracketed fields and delete any sections that truly do not apply. If the lane is not concrete enough to name exact paths or commands, say so instead of guessing.

---

# [Packet name]

## Task first
[State the assignment in one sentence. Keep it concrete and bounded.]

## Why this matters
[Explain why this specific lane is worth doing now and what larger goal it supports.]

## Success condition
By the end of this packet:
- [specific outcome 1]
- [specific outcome 2]
- [specific outcome 3]

## Mode
[BLOCKER_MODE by default; FAMILY_MODE only when the packet is explicitly family-wide]

## Agent lane / model constraints
[If the owner requested an unroled model lane, state it here. Example:
`Use unroled gpt-5.5 low. Do not assign a specialist role because that may
change the model/effort profile.` Delete this section when not relevant.]

## Task-scope paths for grounding
These are the exact files/areas you should use to understand the problem before you decide what the write scope is:
- `[full path]`
- `[full path]`
- `[full path]`

## Exact write scope
Only edit these paths unless the packet explicitly authorizes a nearby extension.
If the packet cannot justify exact paths yet, say that the write scope is not concrete enough instead of inventing paths:
- `[full path]`
- `[full path]`
- `[full path]`

## Explicit out of scope
Do not widen into:
- `steering/ORCHESTRATION.md` or any leader-only orchestration policy; subagents should not read or use it
- `[out-of-scope item]`
- `[out-of-scope item]`
- `[out-of-scope item]`
- whole V3 migration
- unrelated orchestration policy or experiment-protocol redesign
- runtime/library code unless this packet explicitly owns it

## Packet-specific extra docs
This packet assumes the agent has already completed
`/usr/projects/tui-vfx/steering/SUBAGENT-GROUNDING.md`, reported
`READY FOR WORK PACKET`, and had that grounding accepted by the leader. That
satisfies the shared grounding requirement for follow-on packets in the same
subagent session. Do not ask the agent to repeat the global grounding pass or
re-read global grounding docs here. List only active briefing files and
packet-specific architecture, schema, design, or evidence docs needed for this
packet:

1. `[active shared briefing file if this packet uses one]`
2. `[packet-specific extra doc if needed]`

## Repo-boundary guardrails
- `mixed-signals` owns reusable signal/math substrate only.
- `tui-vfx` owns renderer/effect semantics.
- `tui-vfx-recipes` owns recipe truth, tooling, validator/probe/preview, compiled seams, and generated v3.1 schema/docs surfaces.
- Do not read or revise `ORCHESTRATION.md`; it is leader-only orchestration context and will confuse bounded worker lanes.
- Do not revise experiment protocols unless this packet explicitly owns that surface.
- [any lane-specific boundary note]

## Pipeline-touch definition of done
If this packet touches a shader, filter, mask, sampler, style, content effect,
motion route, shadow, scope, binding, or adjacent v3.1 pipeline file:
- move primitive reusable math/signal substrate to `mixed-signals` when it is
  renderer-agnostic and useful to 3+ real callers
- keep renderer/effect semantics in `tui-vfx` / `tui-vfx-recipes`
- normalize timing language and implementation (`elapsed`/absolute time for
  cadence, normalized phase/loop progress for phase)
- update rustdocs for public/schema-bearing items and generated-doc inputs when
  schema/API surfaces change
- align comments/docs/rustdocs/fixtures with canonical v3.1 vocabulary
- update primitive-first debug/reference recipes when visual semantics, timing,
  names, or parameters change
- preserve V2 fallback and oracle paths until owner-approved removal
- prefer adding validator/probe/test coverage for any drift class discovered
- keep `<CLOG>` entries to the latest one- or two-line summary and update
  relevant `INDEX.md` files when docs move or become canonical

## Test-shape requirements
If this packet touches schema/parser behavior, include or point to one canonical
authored JSON fixture and require focused coverage for:
- accepted minimal form
- rejected unknown nested fields
- defaulted omitted fields
- validation boundary errors
- typed propagation into downstream IR/compiled structures

If this packet touches runtime behavior, name likely player/preview/probe
entrypoints and require focused coverage that those surfaces do not silently
drop the new feature. If reduced-motion, timing, or cache policy matters, require
the packet to identify the existing source of truth or report the missing seam
as a handoff.

## First steps / grounding instructions
1. Confirm that prior shared grounding was accepted: the agent completed
   `/usr/projects/tui-vfx/steering/SUBAGENT-GROUNDING.md` and reported
   `READY FOR WORK PACKET`. If that is already true in this subagent session,
   do not repeat it.
2. If prior accepted grounding cannot be confirmed, stop and report that the
   leader must run or refresh shared grounding before packet work continues.
3. Read the packet-specific extra docs before broader file reads or edits.
4. Restate briefly:
   - that shared grounding is already complete, or that it is blocked/missing
   - what belongs in which repo
   - what counts as done
   - what the biggest scope risk is
5. Do the narrowest repo inspection needed before editing.
6. **Cross-repo audit (Intention 41).** If the packet touches any public
   surface that downstream consumers might construct, import, or reference
   — struct fields, public types, exported constants, public function
   signatures — run the appropriate `rg` / `ofpf-search` / `ofpf-content`
   query across **all four repos**:
   - `/usr/projects/tui-vfx`
   - `/usr/projects/tui-vfx-recipes`
   - `/usr/projects/mixed-signals`
   - `/usr/projects/gt-design`

   Report the per-repo hit counts in the final report. Two-repo audits are
   the failure mode the SignalContext lift hit (struct-literal sites in
   tui-vfx-recipes and gt-design were missed). If the packet is purely
   internal to one crate, say so explicitly and skip.
6. If this is a `gpt-5.5` low lane, explicitly list the tricky semantics you
   will test before implementation (coordinate frame, timing, cache, schema
   defaults, or other packet-specific risks).
7. For schema/parser `gpt-5.5` low lanes, confirm the canonical JSON fixture
   shape before editing and name the default/boundary assertions you will add.

## Hot-path watchpoints
- [lane-specific hot-path risk]
- [lane-specific hot-path risk]
- If this is not a hot-path-sensitive lane, say so explicitly.

## Verification required
Run exactly. Only list commands the packet/docs actually justify:
- `[shell-ready command]`
- `[shell-ready command]`
- `[shell-ready command]`
Prefer documented validator/test commands over invented alternatives.

If a command fails, report that exact failure before considering any follow-up command, then classify it as:
- in-scope failure
- expected downstream fallout
- or blocker

## Pre-commit write-scope guard (Intention 40 §5)
Before running `git commit`, verify your stage matches the declared write
scope exactly:

```bash
git diff --cached --name-only
```

The output must list **only** files in this packet's "Exact write scope"
section. If any other file appears (sibling agents' in-progress edits, files
created during build, recyclebin moves, etc.), unstage them with
`git restore --staged <path>` before committing. Sweeping up unrelated
changes contaminates the commit identity even when the swept-up content is
correct work — future archaeology can't tell which files belong to this
packet.

Report the `git diff --cached --name-only` output in your final report so
the leader can verify the scope matched. Likewise: do not run
`git add -A` or `git add .` — stage by explicit path only.

## No-landmines pre-commit check (Intention 40)
Run before commit:

```bash
git diff --cached | rg '^\+.*#\[allow|^\+.*#!\[allow' || echo "no new #[allow] suppressions"
```

If this surfaces new `#[allow]` lines, justify each in the commit message or
remove them. Per-site `#[allow]` for clippy is a landmine; if a strict gate
flags real code-style debt, fix the root cause or set explicit project-level
policy in `clippy.toml`.

## Reporting contract
Your final report must include:
- docs read confirmation
- 3 reflection bullets
- exact task-scope paths used for grounding
- exact changed files (full paths)
- exact commands run
- pass/fail outcome per command
- blocker or handoff notes
- performance risks noticed
- for `gpt-5.5` low lanes: OFPF line-pressure self-check, test-placement
  self-check, and any pre-existing dirty files noticed

## File metadata discipline
If you touch files with header metadata:
- keep `<CLOG>` / `// <CLOG>` entries to 1-2 short lines
- summarize only the latest change made to that file
- do not preserve or append a running history; git already provides history
- keep `<WCTX>` as stable work context unless the file's role changed

## Closing task reminder
Your task is still: [repeat the one-sentence assignment here].
Do not widen into [specific common drift areas].
