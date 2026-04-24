<!-- <FILE>docs/design/tui-vfx-v3-outstanding-master-list.md</FILE> - <DESC>Master outstanding work list for completing tui-vfx V3 and retiring V2 only at the final stability gate.</DESC> -->
<!-- <VERS>VERSION: 0.7.0</VERS> -->
<!-- <WCTX>Track the live V3 punch list, separated into owner decisions and autonomous execution, with accepted naming, phase-scoping, migration outcome, release-gate, scope, composition, and capability-governance decisions moved from decision-needed to execution lanes.</WCTX> -->
<!-- <CLOG>0.7.0: record accepted capability governance from tui-vfx-v3-capability-governance-decision.md, including promotion ladder and rule-of-three factory-internal promotion. 0.6.0: record accepted scope/composition decision from tui-vfx-v3-scope-composition-decision.md, including intersect/replace scope modes, deferred union, and explicit normalized combine/merge semantics. 0.5.0: record accepted release-gate policy from tui-vfx-v3-release-gate-policy.md, including outcome states, whitelist ownership, GTD owner approval for product-visible drift, and no automatic fixture recapture. 0.4.0: record accepted provisional migration outcome policy from tui-vfx-v3-migration-outcome-policy.md, including owner-audit caveat and no legacy recipe removal. 0.3.0: record accepted phase-scoping decision from tui-vfx-v3-phase-scoping-decision.md and move Q#3/Q#13 out of owner-decision status. 0.2.0: record accepted naming decisions from tui-vfx-v3-naming-normalization-decisions.md, move preview/player and routing naming out of owner-decision status, and point rename work at execution. 0.1.0: initial master outstanding list with edge tooling lanes, migration-equivalence work, recipe migration, docs/schema gates, and final V2-removal policy.</CLOG> -->

# V3 outstanding master punch list

This is the working master punch list for remaining V3 work. It is intentionally
higher-level than per-lane PRDs. Use it to decide what remains before V3 is
considered stable and before any V2 fallback/removal work is considered.

## Live status

| ID | Lane | Status | Notes / next action |
|---|---|---|---|
| V3-P0 | V2 retention policy | Done / policy captured | V2 removal is final-only and requires explicit approval after kept recipes migrate or rewrite. |
| V3-T01 | Tooling docs hub | In progress | Seeded under `docs/tooling/`; expand as tooling lanes land. |
| V3-T02 | Probe database / frame diff map | Complete initial | Existing probe diff + SQLite xray surfaces are documented as the reuse target; no duplicate diff system. |
| V3-T03 | Thin V3 preview/player | Complete initial | Existing thin surfaces are mapped; canonical future small CLI/tool name is `tui-vfx-player`; full scripted movie composer is `gtd-movie` and remains deferred. |
| V3-T04 | Grid-resize adapter contract | Complete initial | Docs/evidence prove host-owned resize: host supplies new grid; V3 rerenders with preserved time/runtime state. |
| V3-E01 | ANSI source ingestion | Complete initial | ANSI-styled text normalizes into grid/source data and feeds downstream V3 chains; no terminal lifecycle in runtime. |
| V3-E02 | Offline command-output capture | Complete initial | `recipe-source-capture` captures offline command output into artifacts; runtime recipe execution does not spawn commands. |
| V3-E03 | Grapheme/wide-cell review | Complete initial | As-built Unicode/wide-cell stance is documented; deeper storage changes are deferred. |
| V3-M01 | VC-09 migration-equivalence harness | Outstanding / unblocked provisionally | Use accepted `equivalent` / `replacement` / `retired` tracks while owner audit is pending; do not remove legacy recipes. |
| V3-M02 | Kept-recipe migration/rewrite | Deferred on owner audit | Owner needs time to audit recipes. Work around with provisional classifications only. |
| V3-VC01/03/10 | Validator/canonicalization follow-ons | Outstanding | Finish stricter schema/style diagnostics and human-review-needed queue. |
| V3-C01/C02 | Canonical normalized IR + canonicalization tooling | Outstanding | Treat normalized IR as validator/viewer/equivalence target and prove canonicalized equivalence for curated forms. |
| V3-RG01/RG02 | Release-gate fixtures/evidence | Outstanding / policy decided | Build Chapter 60 manifests using accepted outcome states: `pass`, `fail`, `accepted_change`, `stale_fixture`, `not_applicable`; fixture recapture is explicit only. |
| V3-MOTION01/02 | Motion/offscreen migration support | Outstanding | Complete compatibility table and validate offscreen/slide fixtures. |
| V3-EDGE01 | Motion/shadow/vanishing-edge integration | Outstanding | Implement/prove host-bound motion envelope, transparent shadow behavior, and directional edge-crossing semantics together. |
| V3-SPATIAL01 | Spatial field substrate follow-ons | Outstanding | Surface/frame-space signal basis and richer field/showcase consumers remain follow-up beyond the landed cell-space field-hint proofs. |
| V3-SHOW01 | Madeira / showcase parity | Outstanding | Asset-agnostic Madeira works in the first slice; richer showcase parity and demo-grade reference recipe remain follow-up. |
| V3-REGION01 | Region compression follow-up | Outstanding | Resolve larger-corpus pressure beyond current region refs/runs. |
| V3-NAME01/PREVIEW01 | V3 naming cutover work | Outstanding / decided | Execute the accepted naming slate in `tui-vfx-v3-naming-normalization-decisions.md`: `Vfx*`, `PlaybackPlan`, `PlaybackController`, `V3FrameSnapshot`, `tui-vfx-player`, etc. |
| V3-VIEW01 | Normalized IR viewer/explorer | Outstanding | Scope around normalized execution graph after IR contract is stable. |
| V3-F01 | Celebratory particles/fireworks | Owner decision | Conceptual home exists; priority/fidelity decision needed. |
| V3-TOOL01 | Chapter 100 tooling/CI cutover checklist | Outstanding | V3 schema dispatch/cutover, doc generators, debug QC, trace/probe parity, demo V3 corpus loading, and CI gates must go green. |
| V3-MIG01 | Authoritative recipe inventory manifest | Outstanding | Chapter 50 requires checked-in classification of all recipe files before curation/migration scope is reliable. |
| V3-D01 | Rustdoc/schema/generated docs gate | Ongoing | Required for every public/schema-bearing V3 change. |
| V3-D02 | Hand-authored capabilities/authoring docs | Ongoing | Keep author-facing docs aligned with as-built behavior. |
| V3-RG01 | Six V3 release gates | Outstanding | Chapter 60 gates need fixture/evidence ownership: shadow, offscreen, probe, trace, GTD integration, role-aware lowering. |
| V3-Q01 | Chapter 80 open-question closure | Outstanding | Several questions have strong leans but need either owner decision or implementation-backed closure. |
| V3-R99 | Final V2 retirement plan | Blocked by design | Only after migration, stability, downstream adaptation, and owner approval. |

## Resolved owner decisions

These decisions are no longer blocked on owner input. They should be treated as
accepted direction and moved into implementation/doc cutover work.

| ID | Decision | Canonical target |
|---|---|---|
| V3-Q15 | Vocabulary refresh for the accepted naming slate | `duration_ms`, `enter_offset_ms`, `exit_offset_ms`, `phase: "all"` plus `clock`/`timing`, `placement.anchor`, `motion.route`, `motion.dynamics[]`, `edge_crossing`. |
| V3-Q18 | Routing/hosting hint vocabulary | `StepIntent` for per-step routing/work intent; `SurfaceIntent` for recipe host/container intent. |
| V3-Q19 | Canonical playback seam naming | `PlaybackPlan`, `PlaybackController`, `src/playback/`, `V3FrameSnapshot`, `render_v3_frame_to_buffer`. |
| V3-T05 | Thin player and future movie-layer naming | `tui-vfx-player` for the small recipe player/tool; `gtd-movie` for future scripted movie/timeline composition. |
| V3-NAME-POLICY | Public/wire-format V3 prefix | `Vfx*`; keep `Ra*` only as hidden/deprecated cutover aliases where required. |
| V3-Q03/Q13 | Phase scoping and partial phase spans | Steps and containers may both declare `phase`; effective phase is inherited by intersection; default is `all`; normalized IR emits explicit `PhaseSet`. |
| V3-M03 | Migration outcome tracks | Use `equivalent`, `replacement`, and `retired` tracks. Classification is subject to owner recipe audit, and legacy recipe files must not be removed. |
| V3-RG02 | Release-gate tolerance / whitelist ownership | Use structured manifests. Project-visible visual drift and GTD fixture selection require owner approval; library/tooling failures can be classified/fixed by implementation/tooling owners. |
| V3-Q04 | Composition combine defaults | Authoring uses per-kind defaults; normalized IR emits explicit combine/merge semantics; `sequence` is feed-forward and `parallel` is snapshot-isolated with post-join merge. |
| V3-Q28 | Scope composition precedence | Scope inheritance defaults to `intersect`; explicit `replace` is allowed; `union` is deferred until real recipes prove it is needed. |
| V3-Q25 | Primitive catalog governance | Use the accepted promotion ladder: base primitive, variant, earned-name composition, factory-internal convention, or deferred. Public promotions are sticky. |
| V3-Q27 | Factory-internal promotion process | Use rule-of-three review trigger: three factories, or two factories plus flagship recipe, or repeated author demand. |

## Needs project-owner decision / input

These items are blocked on intent, scope, or acceptance judgment. Do not make
irreversible decisions on these without project-owner input.

| ID | Decision needed | Why owner input matters |
|---|---|---|
| V3-Q07 | GTD `RecipeSceneCanvas` sequencing relative to upstream V3 | Upstream can keep moving, but GTD adoption order is a downstream product/workflow decision. |
| V3-Q08 | Relative Light explorations: V3-only, lab-only V2, or feature-gated V2 | Decides whether new work enters the migration corpus. Current plan leans lab-only until V3. |
| V3-Q20 | Confirm `RecipeSceneCanvas` remains neutral substrate only | Current plan strongly leans yes, with gt-design wrapping it in family-specific surfaces. |
| V3-Q21 | Recipe metadata required/optional policy | `use_cases` may become required; other metadata likely optional. This affects authoring burden and discovery quality. |
| V3-Q23 | Timer model: keep distributed timing or introduce first-class Timer | A first-class timer would reshape step authoring and content-effect timing. Current plan leans defer/status quo with clear docs. |
| V3-M02 | Which recipes are being kept, migrated, rewritten, or dropped | Deferred until owner recipe audit later today. Work around with provisional classifications only. |
| V3-E04 | Whether ANSI/command capture should support live capture later or remain offline-only | Current guidance is offline-only for runtime determinism. A live mode would be a higher-level host/tool policy decision. |
| V3-F01 | Celebratory particles / fireworks priority | Schema has a conceptual home, but implementation priority and required fidelity are product/capability choices. |
| V3-PKG01 | Recipe/theme distribution and packaging source model | Chapter 90 defers registry/archive/embedded/remote source design; V3 should preserve byte-source abstractions but not choose packaging without owner input. |
| V3-DYN01 | Dynamic recipe formalization | Chapter 90 defers formal treatment of generated/runtime-dependent recipes beyond current substitutions/runtime bindings. |
| V3-R99 | Final V2 retirement approval | V2 removal is explicitly final-only after migration/rewrite, stability, downstream adaptation, and owner approval. |

## Autonomous execution queue

These items have enough guidance in `steering/INTENTIONS.md`, the V3 plan, and
the tooling/authoring docs for agents to keep moving without more owner input.

| ID | Work to complete | Definition of done |
|---|---|---|
| V3-I01 | Keep the V3 docs index current | `docs/design/tui-vfx-v3-INDEX.md` is the single start page for V3 work and links every major plan/tooling/migration doc. |
| V3-T01 | Expand the tooling docs hub | `docs/tooling/` maps validation, probe, diff/database, preview/player, resize, edge ingestion, and capture surfaces with links to concrete commands. |
| V3-T02 | Document existing probe database / frame diff tooling | Complete initial; keep docs current as probe tables/flags evolve. |
| V3-T03 | Tighten the thin V3 preview/player example surface | Complete initial; future rename/cutover should align with `tui-vfx-player`, `PlaybackPlan`, `V3FrameSnapshot`. |
| V3-T04 | Document grid-resize behavior and add evidence if useful | Complete initial; smoke/example and debug fixture prove preserved phase across grid sizes. |
| V3-E01 | Wire ANSI source ingestion | Complete initial; maintain bounded ANSI parser docs and avoid claiming full terminal emulation. |
| V3-E02 | Wire offline command-output capture | Complete initial; maintain offline-only security boundary and capture artifact format. |
| V3-E03 | Complete grapheme/wide-cell review | Complete initial; deeper storage model remains deferred until a later explicit redesign. |
| V3-VC01 | Finish authoring schema validation diagnostics | `pipeline-validator` surfaces stricter authoring-shape diagnostics and schema reports without breaking compatible recipes unexpectedly. |
| V3-VC03 | Finish style normalization validation | Validator proves no dual style forms survive normalized IR. |
| V3-M01 | Continue VC-09 migration-equivalence harness | Use provisional `equivalent` / `replacement` / `retired` tracks; keep all legacy files in place pending owner audit. |
| V3-M03 | Implement migration outcome reporting | Inventory/equivalence tooling can report track, rationale, evidence status, and owner-audit-needed flags. |
| V3-VC10 | Add human-review-needed report | Lowering/migration classes that need manual review become a machine-readable queue. |
| V3-CI01 | Build release-gate fixture manifests | Chapter 60 critical sets are represented as manifests for shadow, offscreen, probe, trace, GTD integration, and role-aware lowering. |
| V3-CI02 | Capture / compare release-gate evidence | Gates produce pass/fail/whitelist-needed output with render/probe/trace evidence. |
| V3-C01 | Canonical normalized IR as explicit artifact | Normalized IR is treated as the validator/viewer/equivalence target, with serializable output where tooling needs it. |
| V3-C02 | Canonicalization/property-test tooling | Named-factory and compositional forms can canonicalize to the same normalized form for curated pairs. |
| V3-VIEW01 | Normalized IR viewer/explorer backlog | Viewer work is scoped around normalized execution graph, not raw authoring syntax. |
| V3-MOTION01 | Motion/offscreen compatibility table | Current `PathType` variants, `route`, `dynamics[]`, offscreen `from/to`, and edge-crossing behavior have a concrete V2→V3 mapping. |
| V3-MOTION02 | Offscreen/slide fixture support | Representative slide/offscreen recipes can be expressed and validated in V3. |
| V3-REGION01 | Region compression follow-up | `cell_run`, `cell_runs`, `region_ref`, and any larger-corpus compression pressure are implemented or deferred with evidence. |
| V3-NAME01 | `Ra*` → `Vfx*` inventory execution | Rename-bearing buckets are worked down methodically while preserving compatibility/deprecation guidance during V3 cutover. |
| V3-PREVIEW01 | Preview seam naming migration plan | If owner confirms the rename, execute `Preview*` -> `Playback*`/chosen name with re-exports and docs. |
| V3-STYLE01 | Runtime-facing V3 style family consumers | Continue wiring real V3-side family surfaces into runtime consumers; avoid deleting legacy V2 surfaces until final removal. |
| V3-SCHED01 | Scheduler/batching final strategy | Keep semantic proofs green, preserve `Sequence` feed-forward and `Parallel` snapshot isolation, and only optimize when render-hash drift guards prove safety. |
| V3-BIND01 | Broader runtime binding evaluation | Extend runtime binding support beyond currently proven shader/procedural/scene-visibility seams where the corpus needs it. |
| V3-HINT01 | Hint graph rules hardening | Duplicate producer, visibility, lifetime, and value-kind errors are enforced consistently in validator and runtime. |
| V3-PHASE01 | Implement accepted phase-scoping rule | Validator/canonicalizer accept single phase and phase arrays, apply inherited intersection, reject empty effective phase, and emit explicit normalized `PhaseSet`. |
| V3-SCOPE01 | Implement accepted scope inheritance rule | Validator/canonicalizer support `scope_mode: intersect|replace`, reject empty static intersections and invalid replace-without-child-scope, and emit authored/effective scopes. |
| V3-COMBINE01 | Implement accepted combine defaults | Normalizer emits explicit effective combine/merge semantics for sequence, parallel, masks, filters, shaders, samplers, and overlap classes. |
| V3-GOV01 | Apply capability promotion ladder in authoring docs | Authoring docs and capability catalog use base primitive / variant / earned-name composition / factory-internal / deferred categories consistently. |
| V3-GOV02 | Add factory-internal promotion review hooks | Validator/docs process can flag repeated factory-internal conventions for rule-of-three review without making them public prematurely. |
| V3-SPATIAL01 | Add/prove surface-frame spatial basis if still missing | Mixed-signals has cell-space leaves; docs call out continuous surface/frame geometry leaves as the next additive substrate for optical falloff/spotlight-style consumers. |
| V3-SPATIAL02 | Restore richer Madeira/showcase parity | Re-create the Madeira demo as a first-class V3 showcase using field generation, typed hints, displacement, field-correlated shading, and scene-layer composition. |
| V3-SHADOW01 | Implement/prove transparent shadow and host-bound shadow model | Shadow docs require explicit transparent shadow, host attachment, and motion-edge integration rather than accidental underlay behavior. |
| V3-EDGE01 | Implement/prove directional vanishing-edge behavior | Motion/offscreen edge crossing must drive border vanish/preserve and shadow fade/clip/preserve based on the active clipped edge. |
| V3-TOOL01 | Work down Chapter 100 release checklist | V3 support for schema dispatch/cutover, `pipeline-validator`, `recipe-probe`, `tui-vfx-trace`, debug QC, docs generation, authoring docs, demo, and CI. |
| V3-DOCGEN01 | V3-shape generated docs pipeline | `xtask docs generate` and recipe-side docs-v3 generator/check path produce V3-shaped schemas/capabilities/AI context. |
| V3-TRACE01 | Trace/probe parity for broader V3 subset | Recipe-probe and tui-vfx-trace already accept supported compiled V3 subsets; scene parity, lifecycle-analysis parity, and broader trace semantics remain. |
| V3-DEMO01 | Full native animated V3 demo/browser path | Demo/play_recipe render supported direct V3 bridge subset; full browser experience for migrated V3 corpus remains outstanding. |
| V3-MIG01 | Checked-in recipe inventory manifest | Inventory all recipes as candidate/debug/probe/test/deprecated/generated before curation and migration scheduling. |
| V3-AUTH01 | V3 authoring-guide rewrite | Rewrite authoring workflow, schema reference, scene guide, procedural sources, and pipeline-validator LLM guide for V3 tree schema and primitive/default model. |
| V3-QDOC01 | Reconcile stale status docs | Some older first-slice/status docs still say `IN_PROGRESS`; update or annotate after verifying current as-built state. |
| V3-D01 | Keep rustdocs/schema/generated docs current | Public/schema-bearing changes include rustdocs and generated docs validation where applicable. |
| V3-D02 | Keep hand-maintained docs current | Capabilities, authoring, V3 index, and punch list match the implemented system. |

## Hard policy: V2 removal is last

Do not remove V2, V2 recipe support, or V2 fallback paths until the very end.

V2 stays in place until all of these are true:

1. every recipe we are keeping has either been migrated to V3 or intentionally
   rewritten as a V3 recipe,
2. critical recipes have migration-equivalence or accepted replacement evidence,
3. V3 playback, probe, validator, demo, and recipe-authoring tooling are stable,
4. downstream consumers have had time to adapt,
5. the project owner explicitly agrees the V2 safety net is no longer needed.

Expected sequence: finish V3 capability and tooling work first, migrate or
rewrite recipes second, stabilize with both paths present third, and only then
plan V2 removal as its own final cleanup lane.

## Chapter 63 edge tooling lane

The Cellophane/Whoa-inspired edge lane is tracked in
`docs/design/tui-vfx-v3-upgrade-plan/63_edge_ingestion_runtime_adapters.md`.
The effects/path primitives are effectively complete; the remaining work is
mostly tooling, docs, and source/adapter surfaces.

### Outstanding / in progress

- **Tooling docs hub** — create a first-class `docs/tooling/` area that maps
  validation, probing, diff/database, preview/player, resize, and edge-ingestion
  workflows.
- **ANSI source ingestion** — allow ANSI-styled text to become grid/source data
  for V3 recipes without terminal lifecycle coupling.
- **Offline command-output capture** — provide authoring/tooling capture into a
  fixture/source artifact. Runtime recipe playback must not spawn commands.
- **Thin V3 preview/player surface** — keep this smaller than the deferred movie
  composer. It should load a recipe, advance/render frames, and emit digestible
  text/json/grid/probe-compatible output through the canonical V3 path.
- **Frame diff/database docs** — reuse the existing probe diff and SQLite xray
  surfaces. Do not build a duplicate diff format unless the existing probe shape
  cannot represent the evidence.
- **Grid-resize docs/evidence** — document the as-built contract: hosts own
  resize events; V3 renders to the current supplied grid; phase/time is preserved
  when the host preserves elapsed/runtime state. No new core resize machinery is
  expected unless a concrete bug is found.
- **Grapheme/wide-cell review** — document the current Unicode/wide-cell stance
  before broad ANSI ingestion. Adopt deeper storage changes only if evidence
  shows a correctness gap.

## V3 recipe migration and equivalence

- Continue VC-09 migration-equivalence harness work until the designated critical
  recipes have useful equivalence or accepted replacement evidence.
- Keep V2 and V3 playback available side by side during migration.
- Keep the demo/player capable of showing both paths while recipes are being
  rewritten or migrated.
- Use equivalence where behavior should match V2; use explicit replacement notes
  where V3 intentionally improves or changes the authored effect.

## V3 I/O and pipeline stability

- Maintain full input/output support across shader/filter/mask/sampler/style/
  effect chains.
- Keep debug recipes for each meaningful feature/pathway, starting with the
  simplest primitive-first presentation before advanced combinations.
- Validate chained examples through pipeline-validator/probe, not only by visual
  playback.
- Keep scheduler semantics documented: `sequence` feed-forward, `parallel`
  snapshot isolation, and explicit hint producer/consumer contracts.

## Authoring docs and generated docs

- Keep rustdocs complete for every public or schema-bearing V3 type/field.
- Keep hand-maintained docs aligned with the as-built system.
- Refresh generated schema/API docs when schema-bearing Rust surfaces change.
- Keep `CAPABILITIES_REFERENCE.md`, recipe authoring guidance, and debug recipe
  standards current as new capabilities land.

## Downstream readiness

- Keep tui-vfx, tui-vfx-recipes, mixed-signals, and gt-design boundaries clear:
  reusable signal/math substrate belongs in mixed-signals; effect/render
  semantics belong in tui-vfx/tui-vfx-recipes; downstream display policy belongs
  in consumers.
- Preserve public-surface discipline so gt-design and other consumers wrap the
  canonical recipe/playback outputs rather than reinterpreting internals.
- Use the thin player/tooling work to support future movie/static/wasm/CI visual
  consumers without pulling ratatui into core semantics.

## Final V2 retirement gate

When everything above is stable, create a separate V2-retirement plan. That plan
must inventory remaining V2-only recipes, fallback code paths, docs, examples,
and downstream consumers before any deletion happens.

Until that dedicated plan exists and is approved, V2 removal is out of scope.

<!-- <FILE>docs/design/tui-vfx-v3-outstanding-master-list.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
