# Packet 41 — Madeira visual vetting protocol

## Task first
Define a repeatable Madeira visual-vetting protocol so “fully operational and vetted” includes an explicit visual quality bar.

## Objective
Create a protocol artifact that standardizes what visual evidence is required for Madeira and clearly separates correctness checks from aesthetic preference checks.

## Why this matters
Passing structured tooling alone is not enough for a showcase recipe. Madeira needs a deterministic visual contract plus explicit human review guardrails.

## Scope and inputs
- Madeira-specific visual check scope: `/usr/projects/tui-vfx-recipes/recipes/madeira_flag/madeira_flag.json`
- Existing Madeira-related audit trail and baseline outputs (not re-run in this lane)
- `RECIPE_VISUAL_QA` and `RECIPE_AUTHORING_WORKFLOW` as source of truth for tooling roles and staging philosophy
- Packet constraints: doc-only, no runtime execution, no aesthetic retuning

## Why these outputs are reused
This protocol reuses the existing upstream visual QA convention chain (source of truth):
- `/usr/projects/tui-vfx/docs/RECIPE_VISUAL_QA.md`

## Madeira visual-vetting protocol (minimum artifact)

### A. Correctness checks (required, pass/fail)
Run in order; all must pass for “visuals correct.”

1. **Recipe truth gates (structure and stages)**
   - `pipeline-validator --rules --stages /usr/projects/tui-vfx-recipes/recipes/madeira_flag/madeira_flag.json`
   - `pipeline-validator --probe --probe-causation --probe-frames 3 /usr/projects/tui-vfx-recipes/recipes/madeira_flag/madeira_flag.json`

2. **Deterministic frame sampling matrix (required screenshots/frames)**
   - Capture at least three phase samples for the same deterministic seed/context:
     - **Enter sample**: `--phase entering --sample-t 0.5`
     - **Dwell sample**: `--phase dwelling --sample-t 0.5`
     - **Exit sample**: `--phase exiting --sample-t 0.5`
   - For each sample, capture one frame at canonical viewport size used by the recipe baseline (`80x24`) and record:
     - frame timestamp/sample coordinate
     - notable anchors (firework region, flag region, text stack)

3. **Temporal variation check**
   - `recipe-probe --with-causation --frames 3 --diff-to 0.66 ...`
   - Require non-zero changed-cell deltas and readable monotonic event progression in `probe_motion_effects` for moving layers.

4. **Scene-layer integrity checks (baseline-level correctness)**
   - Verify three named regions remain visually and structurally present across dwell:
     - Backdrop region (base fill + fade state)
     - Fireworks cloud region (active spawn and decay over time)
     - Flag/text region (message remains legible, border/text relationship stable)
   - Reject if any region is missing for a full dwell frame or if expected moving regions alias into others.

5. **Reference anchor check against existing Madeira audit outputs**
   - Confirm the above outputs are compared against existing packet-tracked Madeira outputs (no new capture format is introduced in this lane).

### B. Preference checks (human review required, non-blocking by default unless release signoff)
Reviewers judge these using the preview/demo surface only.

1. **Showcase identity**
   - Does Madeira read as a cohesive New-Year flagship scene (not a disconnected composite of layers)?

2. **Motion/readability balance**
   - Flag motion reads intentional (not random/noisy); text stack remains cleanly readable during dwell and not visually drowned by effects.

3. **Color and tone consistency**
   - Palette and glow should feel festive without flattening into uniform noise.

4. **Temporal rhythm**
   - Fireworks cadence and flag motion should feel lively but not chaotic; transitions should not appear abrupt.

### C. Correctness vs preference decision rule
- **Correctness** is binary: must pass for operational readiness artifacts.
- **Preference** is directional: documented as `PASS / PASS_WITH_MINOR_DEVIATION / FAIL` and fed into packet `43/44` risk notes.
- `correctness` failures block “fully operational and vetted”; preference misses are non-blocking unless release signoff requires zero deviations.

## Madeira checklist (for copy/paste execution)

### Must capture today
1. `pipeline-validator --rules --stages recipes/madeira_flag/madeira_flag.json`
2. `pipeline-validator --probe --probe-causation --probe-frames 3 recipes/madeira_flag/madeira_flag.json`
3. `cargo run -q -p recipe-probe -- --with-causation --phase entering --sample-t 0.5 recipes/madeira_flag/madeira_flag.json`
4. `cargo run -q -p recipe-probe -- --with-causation --phase dwelling --sample-t 0.5 recipes/madeira_flag/madeira_flag.json`
5. `cargo run -q -p recipe-probe -- --with-causation --phase exiting --sample-t 0.5 recipes/madeira_flag/madeira_flag.json`
6. `cargo run -q -p recipe-probe -- --with-causation --diff-to 0.66 recipes/madeira_flag/madeira_flag.json`
7. 1-frame and 3-frame human preview passes of the same recipe in the canonical player.

## Reporting contract for this protocol
- Artifact must log:
  - checks run and timestamp,
  - pass/fail per correctness check,
  - preference judgment summary,
  - whether results match prior Madeira baseline outputs.

## Out of scope
- changing animation semantics,
- broad cosmetic retuning,
- broad new tooling or screenshot infra additions.

