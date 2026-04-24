<!-- <FILE>docs/design/tui-vfx-v3-outstanding-master-list.md</FILE> - <DESC>Master outstanding work list for completing tui-vfx V3 and retiring V2 only at the final stability gate.</DESC> -->
<!-- <VERS>VERSION: 0.12.1</VERS> -->
<!-- <WCTX>Keep the V3 master punch list aligned with active work and explicitly deferred post-release capability specs.</WCTX> -->
<!-- <CLOG>0.12.1: mark V3-PHASE01 complete after PhaseSet implementation.</CLOG> -->

# V3 outstanding master punch list

This is the working master punch list for remaining V3 work. It is intentionally
higher-level than per-lane PRDs. Use it to decide what remains before V3 is
considered stable and before any V2 fallback/removal work is considered.

## Live status

| ID | Lane | Status | Notes / next action |
|---|---|---|---|
| V3-T01 | Tooling docs hub | Complete initial / ongoing | Canonical command map now covers validation, probe, diff/database, preview/player, resize, edge ingestion, command capture, docs generation, and release-gate evidence; expand only as new tooling lands. |
| V3-M01 | VC-09 migration-equivalence harness | In progress / mixed evidence | BSOD V2↔V3 canary is exact; `ease_linear` and `wargames_defcon` now have truthful owner-review-required output+probe mismatch evidence; continue expanding critical-pair coverage while keeping legacy recipes in place pending owner audit. |
| V3-M02 | Kept-recipe migration/rewrite | Deferred on owner audit | Owner needs time to audit recipes. Work around with provisional classifications only; do not remove legacy recipes. |
| V3-VC01/03 | Validator/canonicalization follow-ons | Outstanding | Finish stricter authoring schema diagnostics and style normalization validation; VC-10 review queue is already seeded. |
| V3-C01/C02 | Canonical normalized IR + canonicalization tooling | Complete initial / broaden later | Normalized IR dump contract exists and curated payload alias equivalence tests prove first canonicalization pairs; broader property/corpus coverage remains follow-up. |
| V3-CI02 | Release-gate evidence capture / compare | Outstanding | The release-gate manifest exists; next work is pass/fail/whitelist-needed evidence capture using existing probe/render/trace tooling, starting with the command-backed `probe_alarm_lighthouse` smoke record. |
| V3-EDGE01 | Motion/shadow/vanishing-edge integration | Outstanding | Implement/prove host-bound motion envelope, transparent shadow behavior, and directional edge-crossing semantics together. |
| V3-SPATIAL01 | Spatial field substrate follow-ons | Outstanding | Surface/frame-space signal basis and richer field/showcase consumers remain follow-up beyond the landed cell-space field-hint proofs. |
| V3-SHOW01 | Madeira / showcase parity | Outstanding | Asset-agnostic Madeira works in the first slice; richer showcase parity and demo-grade reference recipe remain follow-up. |
| V3-REGION01 | Region compression follow-up | Outstanding | Resolve larger-corpus pressure beyond current region refs/runs. |
| V3-NAME01/PREVIEW01 | V3 naming cutover work | Outstanding / decided | Execute the accepted naming slate in `tui-vfx-v3-naming-normalization-decisions.md`: `Vfx*`, `PlaybackPlan`, `PlaybackController`, `V3FrameSnapshot`, `tui-vfx-player`, etc. |
| V3-VIEW01 | Normalized IR viewer/explorer | Outstanding | Scope around normalized execution graph after IR contract is stable. |
| V3-F01 | Celebratory particles/fireworks | Owner decision | Conceptual home exists; priority/fidelity decision needed. |
| V3-TOOL01 | Chapter 100 tooling/CI cutover checklist | Outstanding | V3 schema dispatch/cutover, doc generators, debug QC, trace/probe parity, demo V3 corpus loading, and CI gates must go green. |
| V3-D01 | Rustdoc/schema/generated docs gate | Ongoing | Required for every public/schema-bearing V3 change. |
| V3-D02 | Hand-authored capabilities/authoring docs | Ongoing | Keep author-facing docs aligned with as-built behavior. |
| V3-DOCS01 | V3 docs lifecycle/elevation plan | Complete initial / follow-up queued | `tui-vfx-v3-docs-lifecycle-plan.md` classifies core docs, active implementation plans, retained design records, stale-status cleanup, and archive/merge candidates. Next follow-up is V3-QDOC01 stale checklist reconciliation. |
| V3-Q01 | Chapter 80 open-question closure | Outstanding | Several questions have strong leans but need either owner decision or implementation-backed closure. |
| V3-BRAILLE01 | Braille dotfield strategy | Post-release strategy captured | Strategy is documented in `post-release/braille-dotfield-toolkit-plan.md#16-post-release-strategy`; implementation remains explicitly post-release. |
| V3-GLYPHACTOR01 | Glyph actor procedural | Post-release spec captured | `docs/design/post-release/glyph-actor-procedural-spec.md`; implementation waits until core V3 release/migration stability. |
| V3-WEATHER01 | Weather ambient fields | Post-release spec captured | `docs/design/post-release/weather-ambient-field-spec.md`; rain/snow/wind/fog/lightning/time-of-day ingredients are deferred until after V3 core stability. |
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
| V3-Q21 | Recipe metadata policy | Metadata is optional and non-rendering. Keep `description`; support optional `intent_hints`, `expected_visual`, `visual_tags`, `mood`, `related_themes`, `maturity_era`, `authoring_notes`, and `last_reviewed`. `intent_hints` are discovery hints, not routing authority. |
| V3-Q23 | Timer model | Do not add a first-class universal `Timer` primitive for V3. Keep distributed timing: lifecycle/pipeline timing, `mixed-signals` temporal signals, and effect-local timing where owned by the effect. |

## Needs project-owner decision / input

These items are blocked on intent, scope, or acceptance judgment. Do not make
irreversible decisions on these without project-owner input.

| ID | Decision needed | Why owner input matters |
|---|---|---|
| V3-Q07 | GTD `RecipeSceneCanvas` sequencing relative to upstream V3 | Upstream can keep moving, but GTD adoption order is a downstream product/workflow decision. |
| V3-Q08 | Relative Light explorations: V3-only, lab-only V2, or feature-gated V2 | Decides whether new work enters the migration corpus. Current plan leans lab-only until V3. |
| V3-Q20 | Confirm `RecipeSceneCanvas` remains neutral substrate only | Current plan strongly leans yes, with gt-design wrapping it in family-specific surfaces. |
| V3-M02 | Which recipes are being kept, migrated, rewritten, or dropped | Deferred until owner recipe audit later today. Work around with provisional classifications only. |
| V3-E04 | Whether ANSI/command capture should support live capture later or remain offline-only | Current guidance is offline-only for runtime determinism. A live mode would be a higher-level host/tool policy decision. |
| V3-F01 | Celebratory particles / fireworks priority | Schema has a conceptual home, but implementation priority and required fidelity are product/capability choices. |
| V3-PKG01 | Recipe/theme distribution and packaging source model | Chapter 90 defers registry/archive/embedded/remote source design; V3 should preserve byte-source abstractions but not choose packaging without owner input. |
| V3-DYN01 | Dynamic recipe formalization | Chapter 90 defers formal treatment of generated/runtime-dependent recipes beyond current substitutions/runtime bindings. |
| V3-R99 | Final V2 retirement approval | V2 removal is explicitly final-only after migration/rewrite, stability, downstream adaptation, and owner approval. |

## Autonomous execution queue

Use the [V3 execution DAG](tui-vfx-v3-execution-dag.md) to choose work that can run in parallel without cross-track blockers.

These items have enough guidance in `steering/INTENTIONS.md`, the V3 plan, and
the tooling/authoring docs for agents to keep moving without more owner input.

| ID | Work to complete | Definition of done |
|---|---|---|
| V3-I01 | Keep the V3 docs index current | `docs/design/tui-vfx-v3-INDEX.md` is the single start page for V3 work and links every major plan/tooling/migration doc. |
| V3-VC01 | Finish authoring schema validation diagnostics | `pipeline-validator` surfaces stricter authoring-shape diagnostics and schema reports without breaking compatible recipes unexpectedly. |
| V3-VC03 | Finish style normalization validation | Validator proves no dual style forms survive normalized IR. |
| V3-M01 | Continue VC-09 migration-equivalence harness | Use provisional `equivalent` / `replacement` / `retired` tracks; keep all legacy files in place pending owner audit. |
| V3-CI02 | Capture / compare release-gate evidence | Gates produce pass/fail/whitelist-needed output with render/probe/trace evidence. |
| V3-C01 | Canonical normalized IR as explicit artifact | Normalized IR is treated as the validator/viewer/equivalence target; `pipeline-validator --dump-normalized --format json` now advertises the `tui_vfx.pipeline_validator.normalized_ir_dump.v1` envelope contract in `docs/contracts/normalized_ir_dump.v1.schema.json`. |
| V3-C02-FOLLOW | Broaden canonicalization/property-test tooling | Extend beyond the first curated payload alias equivalence tests into named-factory/compositional pairs as the corpus demands. |
| V3-VIEW01 | Normalized IR viewer/explorer backlog | Viewer work is scoped around normalized execution graph, not raw authoring syntax. |
| V3-REGION01 | Region compression follow-up | `cell_run`, `cell_runs`, `region_ref`, and any larger-corpus compression pressure are implemented or deferred with evidence. |
| V3-NAME01 | `Ra*` → `Vfx*` inventory execution | Rename-bearing buckets are worked down methodically while preserving compatibility/deprecation guidance during V3 cutover. |
| V3-PREVIEW01 | Preview seam naming migration plan | Execute `Preview*` -> `Playback*`/chosen name with re-exports and docs after inventory/risk ordering is complete. |
| V3-STYLE01 | Runtime-facing V3 style family consumers | Continue wiring real V3-side family surfaces into runtime consumers; avoid deleting legacy V2 surfaces until final removal. |
| V3-SCHED01 | Scheduler/batching final strategy | Keep semantic proofs green, preserve `Sequence` feed-forward and `Parallel` snapshot isolation, and only optimize when render-hash drift guards prove safety. |
| V3-BIND01 | Broader runtime binding evaluation | Extend runtime binding support beyond currently proven shader/procedural/scene-visibility seams where the corpus needs it. |
| V3-HINT01 | Hint graph rules hardening | Duplicate producer, visibility, lifetime, and value-kind errors are enforced consistently in validator and runtime. |
| V3-SCOPE01 | Implement accepted scope inheritance rule | Validator/canonicalizer support `scope_mode: intersect|replace`, reject empty static intersections and invalid replace-without-child-scope, and emit authored/effective scopes. |
| V3-COMBINE01 | Implement accepted combine defaults | Normalizer emits explicit effective combine/merge semantics for sequence, parallel, masks, filters, shaders, samplers, and overlap classes. |
| V3-GOV01 | Apply capability promotion ladder in authoring docs | Authoring docs and capability catalog use base primitive / variant / earned-name composition / factory-internal / deferred categories consistently. |
| V3-GOV02 | Add factory-internal promotion review hooks | Validator/docs process can flag repeated factory-internal conventions for rule-of-three review without making them public prematurely. |
| V3-SPATIAL01 | Add/prove surface-frame spatial basis if still missing | Mixed-signals has cell-space leaves; docs call out continuous surface/frame geometry leaves as the next additive substrate for optical falloff/spotlight-style consumers. |
| V3-SPATIAL02 | Restore richer Madeira/showcase parity | Re-create the Madeira demo as a first-class V3 showcase using field generation, typed hints, displacement, field-correlated shading, and scene-layer composition. |
| V3-SHADOW01 | Implement/prove transparent shadow and host-bound shadow model | Shadow docs require explicit transparent shadow, host attachment, and motion-edge integration rather than accidental underlay behavior. |
| V3-EDGE01 | Implement/prove directional vanishing-edge behavior | Motion/offscreen edge crossing must drive border vanish/preserve and shadow fade/clip/preserve based on the active clipped edge. |
| V3-TOOL01 | Work down Chapter 100 release checklist | V3 support for schema dispatch/cutover, `pipeline-validator`, `recipe-probe`, `tui-vfx-trace`, debug QC, docs generation, authoring docs, demo, and CI. |
| V3-DOCGEN01 | V3-shape generated docs pipeline | Generated-doc landing page now advertises recipe-side V3 entry points; broader schema/capabilities/AI-context generation and drift checks remain follow-up. |
| V3-TRACE01 | Trace/probe parity for broader V3 subset | Recipe-probe and tui-vfx-trace already accept supported compiled V3 subsets; scene parity, lifecycle-analysis parity, and broader trace semantics remain. |
| V3-DEMO01 | Full native animated V3 demo/browser path | Demo/play_recipe render supported direct V3 bridge subset; full browser experience for migrated V3 corpus remains outstanding. |
| V3-AUTH01-FOLLOW | V3 authoring-doc sibling follow-ons | Scene authoring guide is complete-initial; reconcile schema reference, procedural sources, pipeline-validator LLM guide, and authoring workflow docs around the same V3 ladder and vocabulary. |
| V3-QDOC01 | Reconcile stale status docs | First-slice and validator checklist rows now reflect as-built state with evidence; completed-initial and kept here as a historical breadcrumb. |
| V3-D01 | Keep rustdocs/schema/generated docs current | Public/schema-bearing changes include rustdocs and generated docs validation where applicable. |
| V3-D02 | Keep hand-maintained docs current | Capabilities, authoring, V3 index, and punch list match the implemented system. |
| V3-DOCS01 | Create docs lifecycle/elevation plan | Classify transient V3 planning docs versus durable core docs; decide what gets elevated, merged, kept as design record, or archived. Use developer voice: clear, concise, honest, with occasional fun but no marketing fog machine. |

## Completed / complete-initial work

Completed items stay here rather than disappearing so future agents can see what
was already accepted and avoid rediscovery. `Complete initial` means the first
usable slice landed, but docs/tests may still evolve as related lanes continue.

| ID | Completed state | Evidence |
|---|---|---|
| V3-P0 | Policy captured | V2 removal is final-only and requires explicit approval after kept recipes migrate/rewrite, V3 stabilizes, and downstream consumers adapt. |
| V3-DAG01 | Complete initial | `docs/design/tui-vfx-v3-execution-dag.md`; commit `1492914`. |
| V3-T02 | Complete initial | Existing probe diff + SQLite xray surfaces documented as the reuse target; no duplicate diff system. |
| V3-T03 | Complete initial | Existing thin preview/player surfaces mapped; canonical future small CLI/tool name is `tui-vfx-player`; full scripted movie composer is `gtd-movie` and remains deferred. |
| V3-T04 | Complete initial | Docs/evidence prove host-owned resize: host supplies new grid; V3 rerenders with preserved time/runtime state. |
| V3-E01 | Complete initial | ANSI-styled text normalizes into grid/source data and feeds downstream V3 chains; runtime remains terminal-lifecycle agnostic. |
| V3-E02 | Complete initial | `recipe-source-capture` captures offline command output into artifacts; runtime recipe execution does not spawn commands. |
| V3-E03 | Complete initial | As-built Unicode/wide-cell stance is documented; deeper storage changes are deferred. |
| V3-MIG01 | Complete initial | `tui-vfx-recipes/docs/v3_recipe_inventory_manifest.md`; commit `0554afe`. |
| V3-VC10 | Complete initial | `pipeline-validator --lowering-report --format json` includes `human_review_needed`; commits `11f01c8` and `ed542fb`. |
| V3-CI01 | Complete initial | `docs/design/tui-vfx-v3-release-gate-manifest.md` and `.seed.json`; commit `245525a`. |
| V3-MOTION01 | Complete | `docs/design/tui-vfx-v3-motion-compatibility-table.md`; commit `3ea0d35`. |
| V3-META01 | Complete initial | Debug QC warns when fixtures lack `metadata.expected_visual`; schema/docs aligned to accepted optional metadata policy; commits `fd4da75` and `2fb4a37`. |
| V3-TIME01 | Complete initial | Schema/hand docs explain distributed timing and no universal V3 Timer primitive; commit `2fb4a37`. |
| V3-VC09-BSOD | Complete initial | `tui-vfx-recipes` commit `fe55beb`: BSOD canary reports exact output and probe match through `--migration-equivalence-report`. |
| V3-M03 | Complete initial | `tui-vfx-recipes` commit `fe55beb`: migration report emits `track`, `rationale`, `evidence_status`, owner-review flags, and retired-row handling. |
| V3-MOTION02 | Complete initial | `tui-vfx-recipes` commit `fe55beb` plus `tui-vfx` commit `50481b8`: diagonal offscreen fixture, lowering/probe tests, and compiled top-left/bottom-right two-edge parity. |
| V3-VC03 | Complete initial | `tui-vfx-recipes` commit `2a7b216` and `tui-vfx` commit `4c19f0c`: validator rejects dual legacy/canonical style forms in normalized leaf payloads. |
| V3-VC01 | Complete initial | `tui-vfx-recipes` commit `dac3a67`: optional authoring metadata shape validation for `intent_hints`, `visual_tags`, and `expected_visual`. |
| V3-DOCGEN01A | Complete initial | `tui-vfx-recipes` commit `1a32b56`: generated docs landing page names `just docs-v3-generate`, `just docs-v3-check`, `V3_API.md`, and `v3_api.json`. |
| V3-T01 | Complete initial | `tui-vfx` commit `421b6e7`: `docs/tooling/INDEX.md` is now the canonical V3 tooling command map and links release-gate evidence records. |
| V3-BRAILLE01 | Complete strategy | `tui-vfx` commit `421b6e7`: post-release dotfield strategy captures bgraph/rocketsplash inspiration, ownership boundaries, creative procedural lanes, ANSI diagrams, and phased follow-up work. |
| V3-LANG01 | Complete initial | `tui-vfx` commit `b4ba71c` and `tui-vfx-recipes` commit `7572f86`: schema-field vocabulary is now a steering rule and authoring docs use `motion_routes` for `motion.route` fixtures. |
| V3-EASING01 | Complete initial | `tui-vfx-recipes` commit `7572f86`: all 29 legacy easing source names have matching V3 debug fixtures under `recipes/debug_recipes/easings/` with `metadata.expected_visual` and `EASING:` body labels. |
| V3-MOTIONROUTE01 | Complete initial | `tui-vfx-recipes` commit `7572f86` and `tui-vfx` commit `2ac8cc8`: route/path fixtures moved to `recipes/debug_recipes/motion_routes/`, tests and release-gate references updated. |
| V3-TRACE01A | Complete initial | `tui-vfx-recipes` commit `50c3547`: direct compiled-V3 trace runs emit trace-local lifecycle phase markers and tooling docs no longer treat the surface as blocked. |
| V3-C02A | Complete initial | `tui-vfx-recipes` commit `50c3547`: curated payload alias tests prove `pulse_color`→`color` and `rotation_speed`→`speed` canonicalize equivalently. |
| V3-VC09-EASE01 | Complete evidence checkpoint | `tui-vfx-recipes` commit `9cc4497`: `ease_linear` is tracked as owner-review-required output+probe mismatch evidence. |
| V3-VC09-WARGAMES01 | Complete evidence checkpoint | `tui-vfx-recipes` commit `9cc4497`: `wargames_defcon` is tracked as owner-review-required output+probe mismatch evidence for the next critical-pair slice. |
| V3-BEZIER01 | Complete | `tui-vfx-recipes` commit `752dd5e`: custom Bezier easing fixture now uses a distinct overshoot cubic-bezier curve and validates through parser, validator, and probe paths. |
| V3-PHASE01 | Complete initial | `tui-vfx-recipes` commit `39aae98`: single phase and phase arrays parse, inherited phase intersections normalize into explicit `PhaseSet`, empty effective phase sets fail validation, compiled leaves carry PhaseSet, and generated V3 docs are current. |
| V3-AUTH01 | Complete initial | `tui-vfx-recipes` commit `c0aca72`: scene guide became a ten-section V3 authoring ladder from toast quickstart through scenes, pipelines, I/O, procedural sources, tooling, and debug recipe standards. |

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

- **Tooling docs hub** — maintain a first-class `docs/tooling/` area that maps
  validation, probing, diff/database, preview/player, resize, edge-ingestion,
  capture, and docs-generation workflows.
- **Thin V3 preview/player surface** — keep this smaller than the deferred movie
  composer. It should load a recipe, advance/render frames, and emit digestible
  text/json/grid/probe-compatible output through the canonical V3 path.
- **Frame diff/database docs** — reuse the existing probe diff and SQLite xray
  surfaces. Do not build a duplicate diff format unless the existing probe shape
  cannot represent the evidence.
- **Grid-resize docs/evidence** — keep documenting the as-built contract: hosts
  own resize events; V3 renders to the current supplied grid; phase/time is
  preserved when the host preserves elapsed/runtime state.
- **Grapheme/wide-cell review** — current stance is documented; adopt deeper
  storage changes only if evidence shows a correctness gap.

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
<!-- <VERS>END OF VERSION: 0.12.1</VERS> -->
