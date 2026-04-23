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
