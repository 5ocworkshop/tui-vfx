# Packet 43 — Madeira release-readiness checklist

## Evidence bundle used
- Packet 39: Madeira reference fixtures and baselines (`c1e8a0f`)
- Packet 40: Madeira end-to-end operational check
- Packet 41: Madeira visual vetting protocol (`fbcfe5f`)
- Packet 42: Madeira performance and 60 FPS audit
- Harvested status note: `steering/work-packets/STATUS.md`

## Current readiness read
**Not ready yet.** The current bundle still carries two open release blockers:
- packet 40’s stale diagnostic-truth / diagnostic-example surface
- packet 42’s sustained 60 FPS risk from direct-preview structural overhead

## Must-have checklist

| Gate | Exact proof item | Pass condition | Current status |
|---|---|---|---|
| End-to-end operational path | Run the packet-40 Madeira matrix: load/parse, normalize/validate/compile, validator output truth, deterministic render/probe, and any Madeira-specific baseline tests. | Every stage is PASS with fresh evidence tied to `recipes/madeira_flag/madeira_flag.json`. | PARTIAL — packet 40 says the path is mostly green, but the stale diagnostic-truth / diagnostic-example surface remains the next blocker. |
| Regression anchors | Confirm the packet-39 Madeira baseline artifacts are present and referenced by the release checklist. | Baselines exist, are trustworthy, and would catch a Madeira regression. | PASS — packet 39 landed (`c1e8a0f`). |
| Visual correctness protocol | Execute packet-41 correctness commands: `pipeline-validator --rules --stages /usr/projects/tui-vfx-recipes/recipes/madeira_flag/madeira_flag.json`, `pipeline-validator --probe --probe-causation --probe-frames 3 /usr/projects/tui-vfx-recipes/recipes/madeira_flag/madeira_flag.json`, the three `recipe-probe` phase samples (`--phase entering`, `--phase dwelling`, `--phase exiting` at `--sample-t 0.5`), `recipe-probe --diff-to 0.66 /usr/projects/tui-vfx-recipes/recipes/madeira_flag/madeira_flag.json`, plus 1-frame and 3-frame human preview passes. | All correctness checks PASS; any failed or unproven step blocks release. | PASS as a defined protocol artifact; UNPROVEN as a fresh execution result in this bundle. |
| Human visual gate | Apply packet-41 preference review on the canonical player preview. | Reviewer records PASS / PASS_WITH_MINOR_DEVIATION / FAIL for showcase identity, motion/readability, color/tone, and temporal rhythm. | UNPROVEN — no fresh human review result is present in the current evidence bundle. |
| Diagnostic truth | Verify the packet-40 diagnostic-truth / stale example surface is resolved or explicitly exempted. | No stale Madeira diagnostics remain in the readiness path. | FAIL — explicitly called out as the next blocker. |
| Performance budget | Review packet-42 measurements against the 16.7 ms / 60 FPS target on the Madeira execution path. | Sustained frame time stays within budget with no hot path at risk. | FAIL / AT RISK — packet 42 flags direct-preview structural overhead as a sustained 60 FPS risk. |

## Nice-to-have checklist

| Gate | Exact proof item | Pass condition | Current status |
|---|---|---|---|
| Showcase identity | Human reviewer checks whether Madeira reads as a cohesive New-Year flagship scene rather than a disconnected composite. | PASS / PASS_WITH_MINOR_DEVIATION / FAIL. | UNPROVEN. |
| Motion/readability balance | Human reviewer checks that motion feels intentional and text remains cleanly readable during dwell. | PASS / PASS_WITH_MINOR_DEVIATION / FAIL. | UNPROVEN. |
| Color and tone consistency | Human reviewer checks that palette and glow feel festive without collapsing into uniform noise. | PASS / PASS_WITH_MINOR_DEVIATION / FAIL. | UNPROVEN. |
| Temporal rhythm | Human reviewer checks that fireworks cadence and flag motion feel lively but not chaotic. | PASS / PASS_WITH_MINOR_DEVIATION / FAIL. | UNPROVEN. |

## Release rule
- Must-haves are binary release gates.
- Any `FAIL`, `AT RISK`, or `UNPROVEN` must-have blocks final release signoff.
- Nice-to-haves inform polish only and do not block signoff unless a later packet raises the bar explicitly.
