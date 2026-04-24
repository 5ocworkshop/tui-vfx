# Work Packet DAG

Purpose: capture execution order, dependencies, shared-write collisions, and safe parallel lanes for the current pre-authored work packets.

## Governing principle
The packet quality bar matters because packet quality sets the ceiling on subagent output quality. Packets should be:
- detailed enough for a junior-but-capable engineer to stay accurate
- narrow enough that the subagent is still doing real work rather than just replaying the leader's implementation
- explicit about repo boundaries, in-scope files, out-of-scope files, verification, and handoff conditions

## Node list
- `WP01` — V3 schema/docs freshness
- `WP02` — briefing experiment integration
- `WP03` — reusable task-packet template
- `WP04` — validator output-stage schedule truth
- `WP05` — debug-recipes QC for V3
- `WP06` — V3 docs source-of-truth audit
- `WP07` — Madeira flag parity audit
- `WP08` — Madeira next-slice plan
- `WP09` — scene-layer/native bridge parity
- `WP10` — debug-recipe corpus normalization audit
- `WP11` — motion-disabled demo UX
- `WP12` — native replay hot-path audit

## Dependency graph

### Direct dependencies
- `WP02` depends on the briefing experiment results being complete.
- `WP03` depends on the briefing experiment results being complete.
- `WP08` depends on `WP07`.
- `WP09` is strongly informed by `WP07`; treat `WP07 -> WP09` as preferred sequencing unless a concrete scene-bearing gap is already independently known.

### Soft dependencies / recommended ordering
- `WP01` should precede `WP06` if docs freshness is currently red.
- `WP04` should precede `WP05` when QC/reporting depends on truthful validator stage messaging.
- `WP07` should precede any Madeira implementation lane.
- `WP10` should precede any broad debug-recipe cleanup tranche.
- `WP12` should follow major correctness work in the same seam, not precede it.

## DAG edges summary
- `ExperimentComplete -> WP02`
- `ExperimentComplete -> WP03`
- `WP01 -> WP06` (soft)
- `WP04 -> WP05` (soft)
- `WP07 -> WP08`
- `WP07 -> WP09` (soft/preferred)
- `WP10 -> future debug-recipe cleanup tranches`
- `Correctness on native replay -> WP12` (already mostly satisfied by recent timing work)

## Safe parallel groups

### Group A — post-experiment docs/process
Can run in parallel after experiment convergence if write scopes are separated carefully:
- `WP02` — ORCHESTRATION/shared briefing integration
- `WP03` — reusable packet template

Collision warning:
- both may touch nearby steering files if not carefully separated
- safest approach is sequential unless the template lives in its own new file

### Group B — validator/tooling
Potentially parallel if files are kept disjoint:
- `WP01` — V3 schema/docs freshness
- `WP04` — validator output-stage schedule truth
- `WP05` — V3 debug-recipes QC
- `WP06` — docs source-of-truth audit (read-heavy)

Collision warning:
- `WP01`, `WP04`, and `WP05` all live in `/usr/projects/tui-vfx-recipes`
- `WP04` and `WP05` may collide in validator files
- `WP01` and `WP06` may collide around generated docs and export helpers
- safest split:
  - run `WP06` as read-only first
  - then run `WP01`
  - then `WP04`
  - then `WP05`

### Group C — Madeira/scene path
Preferred sequence rather than parallel:
- `WP07` audit first
- `WP08` planning next
- `WP09` implementation/parity follow-up after the audit identifies the seam

Reason:
- these packets are tightly coupled and are likely to collide in the same scene/compile/render files

### Group D — independent audits
Usually safe in parallel with most other lanes:
- `WP10` — debug-recipe corpus normalization audit
- `WP11` — motion-disabled demo UX
- `WP12` — native replay hot-path audit

Collision warning:
- `WP11` may touch example/demo files in `tui-vfx-recipes` while other recipe-tooling lanes are active; still mostly independent
- `WP12` overlaps with compiled/native replay seams and should not run in parallel with any implementation lane changing the same replay files

## Write-scope collision map

### Highest collision risk
- `WP01` with `WP06`
  - both can touch `src/v3/*` docs/schema surfaces and generated docs
- `WP04` with `WP05`
  - both can touch validator files and tests
- `WP07`, `WP08`, `WP09`
  - all related to Madeira/scene compile/render seams
- `WP12` with any native replay implementation lane
  - should not overlap with replay-code changes

### Medium collision risk
- `WP02` with `WP03`
  - both may affect steering/packet structure docs
- `WP10` with future corpus cleanup lanes
  - audit should ideally finish before cleanup begins

### Low collision risk
- `WP11` with most non-demo lanes
- `WP06` with runtime code lanes if kept strictly read-only

## Recommended execution order

### Phase 0 — wait for experiment results
Do not run `WP02` or `WP03` until the experiment is done.

### Phase 1 — process integration after experiment convergence
1. `WP02` — briefing experiment integration
2. `WP03` — reusable task-packet template

### Phase 2 — tooling/docs truth surfaces
3. `WP06` — docs source-of-truth audit (read-only)
4. `WP01` — schema/docs freshness
5. `WP04` — validator output-stage schedule truth
6. `WP05` — V3 debug-recipes QC

### Phase 3 — Madeira path clarification
7. `WP07` — Madeira parity audit
8. `WP08` — Madeira next-slice plan
9. `WP09` — scene-layer/native bridge parity (or the exact first Madeira-linked implementation seam)

### Phase 4 — supporting audits and UX
10. `WP10` — debug-recipe corpus normalization audit
11. `WP11` — motion-disabled demo UX
12. `WP12` — native replay hot-path audit

## Suggested dispatch strategy

### Conservative / minimal collision strategy
Run mostly sequentially, with only one or two read-heavy lanes in parallel.

### Moderate parallel strategy
- After experiment convergence:
  - run `WP02` and `WP06` in parallel only if `WP06` stays read-only
- then:
  - run `WP01`
  - run `WP04`
  - run `WP05`
- then:
  - run `WP07`
  - run `WP10` in parallel if purely audit
- then:
  - use `WP08` to choose the next implementation seam

## Stop conditions / resequencing triggers
Resequence if:
- a packet uncovers a missing prerequisite
- a packet widens into a shared file cluster unexpectedly
- freshness or validator gates fail in a way that invalidates later audit assumptions
- the experiment results suggest a materially different packet structure that should be applied before further delegation

## Final note
This DAG should be updated whenever:
- a packet is completed
- a packet scope changes materially
- a newly discovered blocker inserts a new prerequisite edge


## Additional packet nodes
- `WP13` — V3 rules-stage coverage expansion
- `WP14` — probe/validator consistency audit
- `WP15` — generated docs freshness gate hardening
- `WP16` — recipe_schema validator boundary audit
- `WP17` — centralized loader dispatch audit
- `WP18` — preview/probe scheduling parity audit
- `WP19` — scene procedural determinism audit
- `WP20` — Madeira scene semantics audit
- `WP21` — filter family native-only fixtures audit
- `WP22` — work-packet library maintenance

## Additional dependency notes
- `WP13` depends softly on `WP04`/`bf08c44` because it extends the restored V3 rules stage.
- `WP14` depends softly on `WP04`, `WP05`, and `WP13` because validator truth should stabilize before cross-surface consistency auditing.
- `WP15` depends softly on `WP01`; run `WP01` first if freshness is currently red.
- `WP16` is independent of most runtime lanes and can run as a read-heavy blocker audit.
- `WP17` is a read-heavy audit and can run in parallel with most non-loader implementation lanes.
- `WP18` depends softly on the recent timing normalization work and should not overlap with new timing implementation in the same files.
- `WP19` is mostly independent but should not overlap with scene/procedural implementation lanes touching the same tests.
- `WP20` depends softly on `WP07`; it is a deeper Madeira follow-on audit.
- `WP21` depends softly on `WP10`; use `WP10` first if a broader corpus-priority view is still missing.
- `WP22` depends on the briefing experiment converging and on `WP03` if a reusable template is introduced.

## Additional collision notes
- `WP13` collides with `WP04` and partially with `WP05` in validator tool files.
- `WP14` is mostly audit-only but may inspect the same files as `WP04`/`WP05`; safest as read-only while those are active.
- `WP15` collides with `WP01` around docs generator/gate files.
- `WP17` may collide with future loader refactor lanes in `src/recipe/` and `src/v3/fnc_load_*`.
- `WP18` may collide with preview/probe timing changes.
- `WP19` and `WP20` can collide with scene-bearing implementation lanes.
- `WP22` collides with any broad packet-library rewrite and should follow the experiment winner.


## Further-ahead packet nodes
- `WP23` — Madeira first implementation slice
- `WP24` — Madeira second-slice template
- `WP25` — V3 debug-recipes filter-family cleanup tranche
- `WP26` — debug-recipes content-family cleanup tranche
- `WP27` — validator JSON shape hardening
- `WP28` — V3 tooling command reference
- `WP29` — V3 handoffs and operator guides
- `WP30` — codex-spark doc-task experiment design
- `WP31` — codex-spark doc-task trial runner
- `WP32` — post-experiment delegation strategy refresh
- `WP33` — V3 end-to-end readiness audit
- `WP34` — pre-Madeira implementation checklist

## Further-ahead dependency notes
- `WP23` depends on `WP07` and `WP08`.
- `WP24` depends on `WP23`.
- `WP25` depends softly on `WP10` or `WP21` to identify the correct tranche.
- `WP26` depends on a content-family shortlist/audit.
- `WP27` depends softly on validator stage stabilization (`WP04`, `WP13`).
- `WP28` depends softly on current command surfaces stabilizing; best after `WP01`, `WP04`, `WP05`, and `WP13`.
- `WP29` depends softly on the current blocker landscape being accurate; best after major tooling blockers settle.
- `WP30` should follow the main briefing experiment convergence.
- `WP31` depends on `WP30`.
- `WP32` depends on the experiment family results (`WP02`, `WP03`, and any spark trial results).
- `WP33` depends softly on the major tooling blockers being updated so the readiness matrix is meaningful.
- `WP34` depends on `WP07`, `WP08`, and the broader V3 readiness evidence in `WP33`.

## Further-ahead collision notes
- `WP23`/`WP24`/future Madeira slices will likely collide heavily with scene-bearing compile/render files.
- `WP25` and `WP26` may collide with future debug-recipe QC or corpus cleanup lanes.
- `WP27` collides with validator/probe JSON-focused work.
- `WP28` and `WP29` are docs/handoff oriented and can often run in parallel with code lanes if they stay read-heavy or docs-only.
- `WP30`/`WP31`/`WP32` belong to the orchestration/experimentation track and should be coordinated with the current briefing experiment.
- `WP33` is mostly audit-only and can often run in parallel with bounded implementation lanes if the evidence sources are stable.
- `WP34` is planning/checklist work and should follow the relevant audits rather than compete with implementation on the same files.


## Madeira-completion packet nodes
- `WP35` — Madeira scene semantics implementation tranche
- `WP36` — Madeira fireworks/effect parity audit
- `WP37` — Madeira fireworks/effect implementation tranche
- `WP38` — Madeira validator/probe truth hardening
- `WP39` — Madeira reference fixtures and baselines
- `WP40` — Madeira end-to-end operational check
- `WP41` — Madeira visual vetting protocol
- `WP42` — Madeira performance and 60 FPS audit
- `WP43` — Madeira release readiness checklist
- `WP44` — Madeira fully operational and vetted signoff

## Madeira-completion dependency notes
- `WP35` depends on `WP07`/`WP08` and any deeper scene-semantic audit such as `WP20`.
- `WP36` should follow the baseline Madeira parity audit and can run after `WP07` if scene-vs-effect separation is still unclear.
- `WP37` depends on `WP36`.
- `WP38` should follow at least one Madeira implementation tranche so diagnostic truth can be tested against real improvements.
- `WP39` should follow at least one Madeira implementation tranche and ideally after `WP38` if the truth surfaces changed.
- `WP40` depends on enough implementation/baseline truth to make an end-to-end check meaningful.
- `WP41` should follow once Madeira is visually substantial enough to review coherently.
- `WP42` depends on enough operational behavior being present to make performance auditing meaningful.
- `WP43` depends on `WP40`, `WP41`, and `WP42`.
- `WP44` depends on `WP43` and the must-have checklist being executable.

## Madeira-completion collision notes
- `WP35` and `WP37` may collide with overlapping scene/effect seams in Madeira-adjacent runtime files; do not run them in parallel if they touch the same compile/render surfaces.
- `WP38` can overlap with validator/probe files and should not run in parallel with other diagnostic-surface changes in the same files.
- `WP39` may touch baseline tests and artifacts that later operational checks depend on.
- `WP40`, `WP41`, `WP42`, `WP43`, and `WP44` are mostly audit/checklist/signoff oriented and should generally follow implementation slices rather than compete with them.

## How far the packet library now extends
The packet library now reaches all the way from immediate V3 tooling blockers through:
- validator/probe/docs/schema hardening
- debug-recipe audits and cleanup tranches
- Madeira auditing and staged implementation
- Madeira operational, visual, and performance vetting
- final Madeira release-readiness and signoff


## Session-harvested follow-on nodes
- `WP45` — probe/validator ordered-preview truth fix
- `WP46` — procedural-layer frame-equivalence regression
- `WP47` — tui-vfx-trace SignalContext compile fix
- `WP48` — Madeira ballistic fireworks procedural support
- `WP49` — scene-layer compile-normalize proof fix
- `WP50` — demo info-panel overflow fix

### Dependency notes
- `WP45` follows the probe/validator consistency audit.
- `WP46` follows the procedural determinism audit.
- `WP47` follows the procedural determinism audit and only repairs the stale trace proof seam.
- `WP48` follows the Madeira fireworks/effect audit and should precede broader fireworks parity work.
- `WP49` follows the Madeira scene-semantics audit where the stale compile-normalize proof was identified.
- `WP50` follows the demo UX/control work and addresses the next discovered UX defect in the same surface.


## Post-Madeira-diagnostics packet nodes
- `WP51` — Madeira diagnostic example PreviewItem Option fix
- `WP52` — Madeira direct-preview allocation/cache slice

## Post-Madeira-diagnostics dependency notes
- `WP51` depends on Packet 40 because it repairs the exact bounded diagnostic-surface blocker identified there.
- `WP52` depends on Packet 42 because it follows the performance audit's recommended next optimization seam.
- `WP43` should follow `WP41` and `WP42` now that visual protocol and performance audit outputs exist.
- `WP44` still depends on `WP40`, `WP41`, `WP42`, and `WP43`.

## Post-Madeira-diagnostics collision notes
- `WP51` collides with preview/diagnostic example surfaces only and should stay out of runtime redesign.
- `WP52` collides with direct-preview/render-path optimization work and should not overlap with other preview-path implementation lanes.


## Post-V3-program-status packet nodes
- `WP53` — V3 family-models critical cutover
- `WP54` — V3 mixed-signals signal-graph and time alignment
- `WP55` — V3 spatial leaves and field-hint threading
- `WP56` — V3 motion-path and offscreen-origin support
- `WP57` — V3 live naming and vocabulary cleanup
- `WP58` — V3 Ra→Vfx public-surface rename tranche
- `WP59` — V3 rustdoc gap closure for schema-bearing APIs
- `WP60` — V3 doc-autogen and authoring-guide cutover

## Post-V3-program-status dependency notes
- `WP53` follows the remaining Chapter 100 family-model blocker in `tui-vfx-style/src/models/` and should coordinate with any still-open family cleanup lanes such as `WP05` and `WP26`.
- `WP54` follows the migration-log major gap around signal-graph JSON shape and should precede or explicitly frame downstream mixed-signals/V3 field work.
- `WP55` follows `WP54` and the spatial-field-hint plan; it is the first execution-facing tranche for mixed-signals spatial leaves and recipes-side threading.
- `WP56` follows the motion spec and migration-log major gap for `motion_path` plus offscreen `from/to`.
- `WP57` follows the rename inventory's "live docs/comments/prompts first" rule and should precede the larger Rust public-surface rename.
- `WP58` follows `WP57` and the rename inventory's public-surface rename order.
- `WP59` follows the current V3 docs/autogen gap evidence in `docs/generated/V3_API.md`.
- `WP60` follows Chapter 100's unresolved doc-generator / guide / CI blockers and should coordinate with `WP59` when both touch generated artifacts.

## Post-V3-program-status collision notes
- `WP53` can collide with style-model / capability-doc / doc-generator work and should not overlap with other broad `tui-vfx-style` restructuring.
- `WP54` and `WP55` can collide across `mixed-signals` and recipes/runtime signal consumers; sequence them unless the write scopes are explicitly disjoint.
- `WP56` collides with motion/runtime/schema files and should not overlap with other motion-path or preview-path refactors.
- `WP57` and `WP58` both touch naming/vocabulary surfaces; keep the live-doc cleanup tranche separate from the public Rust rename tranche.
- `WP59` and `WP60` both affect docs/rustdoc/generation surfaces; avoid running them in parallel unless the write scopes are clearly separated.


## Madeira-reference audit packet node
- `WP61` — Madeira reference-repo faithfulness audit

## Madeira-reference dependency notes
- `WP23` is now effectively superseded by the later Madeira audit/corrective chain (`WP61` -> `WP62`); keep it as the historical first-slice entry point, not the active execution lane.
- `WP61` should follow the current Madeira operational, visual, and readiness evidence (`WP40`, `WP41`, `WP43`) because it compares the live V3 recipe against `/usr/projects/madeira-flag` using the latest proof surfaces.
- `WP61` can inform `WP44` signoff and any future Madeira corrective implementation slice.

## Madeira-reference collision notes
- `WP61` is primarily audit-only and can run in parallel with non-Madeira implementation lanes, but should avoid overlapping with another packet that is actively retuning Madeira visuals at the same time.


## Madeira-direct-output corrective packet node
- `WP62` — Madeira direct-scene output fidelity fix

## Madeira-direct-output dependency notes
- `WP62` follows `WP61` because it implements the single next corrective slice identified by the faithfulness audit.
- `WP62` should precede any deeper visual parity tuning or final Madeira signoff reassessment.

## Madeira-direct-output collision notes
- `WP62` collides with any packet actively retuning Madeira scene rendering or preview behavior and should not overlap with broader Madeira implementation lanes in the same files.
