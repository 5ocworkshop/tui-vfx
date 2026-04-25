<!-- <FILE>docs/design/tui-vfx-v3-outstanding-master-list.md</FILE> - <DESC>Master outstanding work list for completing tui-vfx V3 and retiring V2 only at the final stability gate.</DESC> -->
<!-- <VERS>VERSION: 0.14.25</VERS> -->
<!-- <WCTX>Keep the V3 master punch list aligned with active work, question-closure status, and explicitly deferred post-release specs.</WCTX> -->
<!-- <CLOG>0.14.25: close V3-STYLE01 after runtime-facing style family consumer hardening landed.</CLOG> -->

# V3 outstanding master punch list

This is the working master punch list for remaining V3 work. It is intentionally
higher-level than per-lane PRDs. Use it to decide what remains before V3 is
considered stable and before any V2 fallback/removal work is considered.

## Live status

| ID | Lane | Status | Notes / next action |
|---|---|---|---|
| V3-PIPE01 | Core V3 pipeline hardening | Complete initial / matrix audit continues | See `tui-vfx-v3-core-pipeline-readiness-matrix.md` for the current lane-by-lane proof set. Latest matrix pass keeps shader/filter/mask/sampler/style/content/shadow/binding lanes on deterministic fixture evidence and adds a direct `scene_layer_surface_shadow_pipeline` proof so the smaller scene-layer shadow+filter lane is no longer implicit. Continue shrinking the per-family matrix only where new fixtures expose gaps. |
| V3-T01 | Tooling docs hub | Complete initial / ongoing | Canonical command map now covers validation, probe, diff/database, preview/player, resize, edge ingestion, command capture, docs generation, and release-gate evidence; expand only as new tooling lands. |
| V3-M01 | VC-09 migration-equivalence harness | Complete initial / owner-audit blocked for complete-final | `tui-vfx-recipes` commit `d2fe1e4` carries the current 10-pair critical evidence group plus machine-readable provisional buckets and `OWNER-RECIPE-AUDIT`. Continue only curated critical evidence until the owner recipe audit resolves keep/rewrite/retire classifications; keep legacy files in place. |
| V3-M02 | Kept-recipe migration/rewrite | Deferred on owner audit | Owner needs time to audit recipes. Work around with provisional classifications only; do not remove legacy recipes. |
| V3-VC01/03 | Validator/canonicalization follow-ons | Complete initial / broaden later | Metadata shape diagnostics now include `related_themes`; style normalization validation has a first pass. Broaden remaining optional metadata/schema-report coverage as corpus pressure appears. |
| V3-C01/C02 | Canonical normalized IR + canonicalization tooling | Complete initial / broaden later | Normalized IR dump contract exists, curated payload alias equivalence tests prove first canonicalization pairs, and normalized IR now emits explicit phase/scope/combine metadata. Broader property/corpus coverage remains follow-up. |
| V3-CI02 | Release-gate evidence capture / compare | Blocked on visual/GTD approval and GTD probe follow-up | Headless evidence is current for non-GTD fixtures that can run without GUI capture. The current inventory has 19 records: 11 pass and 8 blocked/failing records. Remaining blockers are owner-approved `render_capture_png` captures for offscreen/role-scope fixtures, owner-approved GTD representative captures, and GTD candidate probe failures if those provisional fixtures remain selected. No X11/Zutty/Xvfb capture is allowed without explicit owner approval. |
| V3-EDGE01 | Motion/shadow/vanishing-edge integration | Complete initial / broaden later | Scene-layer motion now feeds edge-crossing border and attached-shadow policy; shared helper coverage proves diagonal/two-edge blanking directly. 2026-04-25 follow-up fixed the edge-crossing shadow artifact where a viewport-clipped side could cast a fake vertical shadow leg during exit. Broaden full scene-layer corpus fixtures as needed. |
| V3-BORDER01 | Border parity and schema hardening | Complete initial / broaden only on corpus pressure | `single` now parses as the line-drawn plain border, V3 trim precedence has tests for omitted trim and explicit `trim: none`, and `tui-vfx-recipes` commit `d80bb7d` makes root V3 `custom_chars` and `frame` render into border-role source cells. Follow-up decision is closed: role-scoped border/text targeting is a native V3 source/scene contract; legacy V2 recipes stay on the legacy renderer until migrated rather than receiving synthetic V3 role maps. |
| V3-NAME01/PREVIEW01 | V3 naming cutover work | Complete initial / compatibility shims retained | Scene/continuous/clock, main `config.rs`, parser wrappers, recipe internals, public preview-boundary aliases, examples, tests, and tooling now use canonical `Vfx*`/`Playback*`/frame names where safe while preserving compatibility aliases and legacy module paths during cutover. Future work is only broader deprecation/module-path cleanup if needed. |
| V3-F01 | Celebratory particles/fireworks | Owner decision | Conceptual home exists; priority/fidelity decision needed. |
| V3-TOOL01 | Chapter 100 tooling/CI cutover checklist | Outstanding / Horseman complete-initial | V3 schema dispatch/cutover, doc generators, debug QC, trace/probe parity, demo V3 corpus loading, and CI gates must go green. Packaged `tui-vfx-horseman` now exists in `tui-vfx-recipes` with text/JSON summary modes over existing preview/cutover APIs; `just v3-headless-smoke` now rehearses the headless validator/probe/trace/docs slice with a legacy fallback probe and the `probe_alarm_lighthouse` release-gate probe smoke, but the broader command/docs/CI cutover still needs GitHub Actions and release-gate adoption. |
| V3-D01 | Rustdoc/schema/generated docs gate | Ongoing / recipes docs gate green | 2026-04-25 recipes-side audit: `just docs-v3-generate` produced no generated-output drift, `just docs-v3-check` passed, and `RUSTDOCFLAGS="-D warnings" CARGO_TARGET_DIR=/tmp/tui-vfx-recipes-doc-target cargo doc -p tui-vfx-recipes --no-deps` completed. Use the throwaway target dir when shared cargo target locks are busy. |
| V3-D02 | Hand-authored capabilities/authoring docs | Ongoing | Keep author-facing docs aligned with as-built behavior. |
| V3-Q01 | Chapter 80 open-question closure | Complete initial / owner decisions remain | Chapter 80 now has a current closure ledger separating accepted/implemented questions, active implementation-shaped questions, and owner/downstream decisions. Remaining owner decisions stay listed below rather than blocking upstream V3 execution. |
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
| V3-T05 | Thin player and future movie-layer naming | `tui-vfx-horseman` for the small recipe player/tool; `gtd-movie` for future scripted movie/timeline composition. |
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
| V3-I01 | Keep the V3 docs index current | `docs/design/tui-vfx-v3-INDEX.md` is the single start page for V3 work and classifies canonical core docs, active work surfaces, retained history, and post-release/deferred specs. Keep the lifecycle map current as docs change. |
| V3-VC01 | Finish authoring schema validation diagnostics | `pipeline-validator` surfaces stricter authoring-shape diagnostics and schema reports without breaking compatible recipes unexpectedly. |
| V3-VC03 | Finish style normalization validation | Validator proves no dual style forms survive normalized IR. |
| V3-M01 | Continue VC-09 migration-equivalence harness | Use provisional `equivalent` / `replacement` / `retired` tracks; keep all legacy files in place pending owner audit. |
| V3-CI02 | Capture / compare release-gate evidence | Remaining work is owner visual-capture/GTD-fixture approval, any owner-approved capture reruns, and follow-up on provisional GTD candidate probe failures if those fixtures remain selected; known non-GTD headless probe blocker is resolved. |
| V3-C01 | Canonical normalized IR as explicit artifact | Normalized IR is treated as the validator/viewer/equivalence target; `pipeline-validator --dump-normalized --format json` now advertises the `tui_vfx.pipeline_validator.normalized_ir_dump.v1` envelope contract in `docs/contracts/normalized_ir_dump.v1.schema.json`. |
| V3-C02-FOLLOW | Broaden canonicalization/property-test tooling | Extend beyond the first curated payload alias equivalence tests into named-factory/compositional pairs as the corpus demands. |
| V3-SCHED01 | Scheduler/batching final strategy | Keep semantic proofs green, preserve `Sequence` feed-forward and `Parallel` snapshot isolation, and only optimize when render-hash drift guards prove safety. |
| V3-BIND01 | Broader runtime binding evaluation | FadeToCanvas canvas-color binding is now probe/QC-proven for exit-phase compiled V3 filters. Continue extending runtime binding support only where the corpus exposes a concrete missing seam. |
| V3-SPATIAL02 | Restore richer Madeira/showcase parity | Complete code-side / showcase tuning only if owner requests | `tui-vfx-recipes` commit `83add2a` preserves authored spatial-field payloads for visual bridge consumers, samples only non-spatial style hint scalars at active replay time, adds runtime override handling for non-spatial style replay, and keeps Madeira strict-contract validation passing. |
| V3-EDGE01 | Implement/prove directional vanishing-edge behavior | Motion/offscreen edge crossing must drive border vanish/preserve and shadow fade/clip/preserve based on the active clipped edge. Latest clipped-shadow-side fix is landed; remaining work is broader scene/corpus proof if new recipes expose gaps. |
| V3-TOOL01 | Work down Chapter 100 release checklist | `tui-vfx-horseman` is the packaged headless summary/corpus command, and debug QC now accepts multi-sampler, none-mask, and exit-phase FadeToCanvas fixtures. Remaining work is CI adoption, broader trace/probe parity, and release-gate policy closure beyond the first headless probe smoke. |
| V3-TRACE01 | Trace/probe parity for broader V3 subset | `tui-vfx-recipes` commit `d2fe1e4` adds lifecycle-sampled compiled-V3 scene-composition trace events. The next slice now mirrors lifecycle transitions and broader stage-summary semantics; continue toward broader trace semantics and do not mark complete-final yet. |
| V3-D01 | Keep rustdocs/schema/generated docs current | Public/schema-bearing changes include rustdocs and generated docs validation where applicable. Latest recipes-side audit on 2026-04-25: `just docs-v3-generate`, `just docs-v3-check`, and `RUSTDOCFLAGS="-D warnings" CARGO_TARGET_DIR=/tmp/tui-vfx-recipes-doc-target cargo doc -p tui-vfx-recipes --no-deps` are green. |
| V3-D02 | Keep hand-maintained docs current | Capabilities, authoring, V3 index, and punch list match the implemented system. |

## Completed / complete-initial work

Completed items stay here rather than disappearing so future agents can see what
was already accepted and avoid rediscovery. `Complete initial` means the first
usable slice landed, but docs/tests may still evolve as related lanes continue.

| ID | Completed state | Evidence |
|---|---|---|
| V3-PIPE01-SAMPLER | Complete initial | Ordered multi-sampler chains now execute through `CompositionSpec::effective_samplers`, compiled V3 lowers repeated sampler leaves instead of failing `MultipleSamplers`, and probe/QC reports configured sampler chains with `configured_count: 2`. |
| V3-PIPE01-SHADER-BAND | Complete initial | V3 traveling-band lowering now preserves reflect `gap`/`width` plus `head_tail` color policy for border, reflect, trace propagation, and trace path while keeping legacy solid-color defaults backward-compatible. |
| V3-PIPE01-FILTER-BIND | Complete initial | Exit-phase `fade_to_canvas.canvas_color` bindings lower into runtime-compatible filter payloads and pass recipe-probe/debug-QC with `FadeToCanvas#1` observed. |
| V3-STYLE01 | Closed | `tui-vfx-recipes` commit `83add2a` audits the V3 style family surface and hardens direct runtime replay so spatial style effects continue through compositor shader families while every non-spatial family (`StyleFade`, `StyleModulation`, `TypographyWindow`, `StyleInstability`, `PairedCapability`) is collected for the style pass with active-time scalar hint/runtime-binding sampling. |
| V3-PIPE01-MASK-NONE | Complete initial | `mask_none.json` exists as the primitive-first no-clipping baseline; debug QC treats explicit V3 none-mask fixtures as valid inactive-mask baselines. |
| V3-PIPE01-CONTENT-SCENE | Complete initial | Scene text-source `content_effect` is regression-proven through compiled scene source construction and direct preview-area render hash changes. |
| V3-P0 | Policy captured | V2 removal is final-only and requires explicit approval after kept recipes migrate/rewrite, V3 stabilizes, and downstream consumers adapt. |
| V3-DAG01 | Complete initial | `docs/design/tui-vfx-v3-execution-dag.md`; commit `1492914`. |
| V3-T02 | Complete initial | Existing probe diff + SQLite xray surfaces documented as the reuse target; no duplicate diff system. |
| V3-T03 | Complete initial | Existing thin preview/player surfaces mapped; canonical future small CLI/tool name is `tui-vfx-horseman`; full scripted movie composer is `gtd-movie` and remains deferred. |
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
| V3-SHADOW01 | Complete initial / broaden later | `tui-vfx-recipes` headless proof now threads recipe-owned shadow through preview/probe/render and the direct-V3 underlay path: `preview_from_recipe_config`, `build_probe_spec_from_preview`, `direct_v3_snapshot_preserves_underlay_background_for_transparent_cells`, and `direct_v3_snapshot_shadow_blends_against_real_underlay` cover transparent shadow plus host-bound composition; release-gate fixtures `shadow_surface_base`, `shadow_surface_pipeline`, and `shadow_edge_crossing` all pass probe/trace. Broaden later only if a new consumer needs additional composite-mode variants. |
| V3-SHADOW02 | Complete | `tui-vfx` commit `5c754ea` makes the default shadow style the preferred transparent full-cell alpha shadow; `tui-vfx-recipes` commit `0239173` suppresses fake clipped-edge shadow legs during edge-crossing exits and adds `recipes/debug_recipes/shadows/` coverage for solid/full-cell, grade-underlying, medium-shade, braille, gradient, and explicit half-block styles. |
| V3-SPATIAL02-CODE | Complete initial | `tui-vfx-recipes` commit `83add2a`: spatial-field payloads are preserved for bridge/showcase consumers while non-spatial style hints are sampled at replay time; runtime overrides and style-family replay tests landed, and Madeira strict-contract validation passed. |
| V3-BORDER01-FOLLOW | Complete decision | Do not retrofit synthetic V3 role maps onto legacy V2-rendered sources. V2 recipes remain on the legacy renderer until migrated; schema notes now state that role-scoped border/title/frame/custom-char targeting is guaranteed on the native V3 path only. |
| V3-BORDER01-SOURCE | Complete initial | `tui-vfx-recipes` commit `d80bb7d`: root V3 source rendering now honors `border.type: "single"`, `custom_chars`, direct `frame` payloads, and border titles with `RoleTag::Border` coverage; targeted source-surface tests and cargo check passed. |
| V3-VC03 | Complete initial | `tui-vfx-recipes` commit `2a7b216` and `tui-vfx` commit `4c19f0c`: validator rejects dual legacy/canonical style forms in normalized leaf payloads. |
| V3-VC01 | Complete initial | `tui-vfx-recipes` commit `dac3a67`: optional authoring metadata shape validation for `intent_hints`, `visual_tags`, and `expected_visual`. |
| V3-DOCGEN01A | Complete initial | `tui-vfx-recipes` commit `1a32b56`: generated docs landing page names `just docs-v3-generate`, `just docs-v3-check`, `V3_API.md`, and `v3_api.json`. |
| V3-DOCGEN01 | Complete initial | `tui-vfx-recipes` generated-doc gate now freshness-checks `docs/generated/V3_API.md`, `docs/generated/v3_api.json`, and `docs/generated/README.md` with schema-versioned public item, field, enum-variant, and type-alias shape extracted from `src/v3`. Broader unified `tui-vfx` capabilities/AI-context generation remains tracked under V3-TOOL01 / Chapter 100. |
| V3-T01 | Complete initial | `tui-vfx` commit `421b6e7`: `docs/tooling/INDEX.md` is now the canonical V3 tooling command map and links release-gate evidence records. |
| V3-GOV01 | Complete initial | `docs/design/tui-vfx-v3-capability-governance-decision.md`, `docs/design/tui-vfx-v3-recipe-vocabulary.md`, `docs/scene/AUTHORING_GUIDE.md`, and `docs/scene/THEME_RECIPE_AUTHORING_PROMPT.md` now use the full ladder terms consistently: base primitive, variant, earned-name composition, factory-internal convention, deferred. |
| V3-GOV02 | Complete initial | `docs/scene/AUTHORING_GUIDE.md#90-authoring-workflow-and-debugging-tools`, `docs/scene/AUTHORING_GUIDE.md#100-appendix-debug-recipe-standards`, and `docs/scene/evaluations/README.md` now record review-hook guidance for repeated factory-internal conventions without adding a new registry. |
| V3-BRAILLE01 | Complete strategy | `tui-vfx` commit `421b6e7`: post-release dotfield strategy captures bgraph/rocketsplash inspiration, ownership boundaries, creative procedural lanes, ANSI diagrams, and phased follow-up work. |
| V3-LANG01 | Complete initial | `tui-vfx` commit `b4ba71c` and `tui-vfx-recipes` commit `7572f86`: schema-field vocabulary is now a steering rule and authoring docs use `motion_routes` for `motion.route` fixtures. |
| V3-EASING01 | Complete initial | `tui-vfx-recipes` commit `7572f86`: all 29 legacy easing source names have matching V3 debug fixtures under `recipes/debug_recipes/easings/` with `metadata.expected_visual` and `EASING:` body labels. |
| V3-MOTIONROUTE01 | Complete initial | `tui-vfx-recipes` commit `7572f86` and `tui-vfx` commit `2ac8cc8`: route/path fixtures moved to `recipes/debug_recipes/motion_routes/`, tests and release-gate references updated. |
| V3-TRACE01A | Complete initial | `tui-vfx-recipes` commit `50c3547`: direct compiled-V3 trace runs emit trace-local lifecycle phase markers and tooling docs no longer treat the surface as blocked. |
| V3-C02A | Complete initial | `tui-vfx-recipes` commit `50c3547`: curated payload alias tests prove `pulse_color`→`color` and `rotation_speed`→`speed` canonicalize equivalently. |
| V3-VC09-EASE01 | Complete evidence checkpoint | `tui-vfx-recipes` commit `9cc4497`: `ease_linear` is tracked as owner-review-required output+probe mismatch evidence. |
| V3-VC09-WARGAMES01 | Complete evidence checkpoint | `tui-vfx-recipes` commit `9cc4497`: `wargames_defcon` is tracked as owner-review-required output+probe mismatch evidence for the next critical-pair slice. |
| V3-VC09-COMPLEX01 | Complete evidence checkpoint | `tui-vfx-recipes` commit `d68ce00`: `complex_full_pipeline` is tracked as owner-review-required output+probe mismatch replacement evidence. |
| V3-VC09-MULTISAMPLER01 | Complete evidence checkpoint | `tui-vfx-recipes` commit `bf2ee98`: `complex_multi_sampler` is tracked as partial-match replacement evidence with output-only owner review. |
| V3-VC09-LAYEREDSHADERS01 | Complete evidence checkpoint | `tui-vfx-recipes` commit `9630bc1`: `complex_layered_shaders` is tracked as replacement evidence with output+probe mismatch owner review. |
| V3-VC09-MASKIRIS01 | Complete evidence checkpoint | `tui-vfx-recipes` commit `9630bc1`: `mask_iris` is tracked as replacement evidence with output+probe mismatch owner review. |
| V3-VC09-BASELINE01 | Complete evidence checkpoint | `tui-vfx-recipes` commit `58b34be`: `baseline` is tracked as partial-match replacement evidence with output-only owner review. |
| V3-BEZIER01 | Complete | `tui-vfx-recipes` commit `752dd5e`: custom Bezier easing fixture now uses a distinct overshoot cubic-bezier curve and validates through parser, validator, and probe paths. |
| V3-PHASE01 | Complete initial | `tui-vfx-recipes` commit `39aae98`: single phase and phase arrays parse, inherited phase intersections normalize into explicit `PhaseSet`, empty effective phase sets fail validation, compiled leaves carry PhaseSet, and generated V3 docs are current. |
| V3-HINT01 | Complete initial | `tui-vfx-recipes` commit `ec25b34`: build and ordered runtime paths share hard failures for duplicate producers, missing producers, and declared hint kind mismatches. |
| V3-SCOPE01 | Complete initial | `tui-vfx-recipes` commit `ec25b34`: authoring/normalization/generated docs support `scope_mode: intersect|replace`, normalized IR emits authored/effective scopes, and validation rejects empty static intersections plus replace-without-authored-scope. |
| V3-COMBINE01 | Complete initial | `tui-vfx-recipes` commit `bf2ee98`: normalized IR emits explicit sequence/parallel combine, merge, leaf-family defaults, and conservative parallel overlap classifications. |
| V3-NAME01-INV | Complete initial | `tui-vfx` commit `c765fa8`: accepted naming slate has concrete repo/file/symbol buckets, compatibility/re-export guidance, risk order, and next rename slice. |
| V3-CI02-D6-REFRESH | Complete evidence refresh | `tui-vfx` commit `c765fa8`: stale D6 sidecars replaced with fresh truthful failing evidence for offscreen diagnostics and missing shadow render-capture gates. |
| V3-D2-SURFACEMAP | Complete initial | `tui-vfx` commit `abfb615`: `v3_play_recipe`, validator output dump, probe/diff surfaces, render dump responsibilities, and packaged `tui-vfx-horseman` are documented as the current Horseman/headless summary substrate. |
| V3-C3C4-SCENE-EDGE | Complete initial | `tui-vfx-recipes` commits `c5190f1`, `86abb54`, and `5164596`: scene-layer motion host context feeds edge-crossing border and attached-shadow policies; shared helper coverage proves diagonal/two-edge blanking directly. |
| V3-SPATIAL01-SURFACE-RADIUS | Complete initial | `tui-vfx-recipes` commit `c5190f1`: existing mixed-signals `sample_surface_radius` leaf parses through V3 recipe payloads and affects native filter rendering. |
| V3-D6-OFFSCREEN-FIXTURES | Complete evidence refresh | `tui-vfx-recipes` commit `e6e9da5` and `tui-vfx` evidence update: `content_slide_shift` and `scene_layer_follow_lag` now probe clean with empty diagnostics. |
| V3-NAME01-SCHEMA-A | Complete initial | `tui-vfx-recipes` commit `a9d92e3`: scene/continuous/clock schema types use canonical `Vfx*` definitions with hidden `Ra*` compatibility aliases. |
| V3-NAME01-SCHEMA-CONFIG | Complete initial | `tui-vfx-recipes` commit `ebc3eb0`: main `config.rs` schema definitions use canonical `Vfx*` names with hidden `Ra*` compatibility aliases and unchanged serde/wire fields. |
| V3-NAME01-PARSER | Complete initial | `tui-vfx-recipes` commit `eca1967`: parser wrappers and recipe internals use canonical `Vfx*` names while retaining hidden `Ra*` aliases. |
| V3-SPATIAL01-ORIGIN-LEAVES | Complete initial | `tui-vfx-recipes` commit `7b2246f`: `sample_surface_radius_from` and `sample_surface_angle_from` both affect native V3 filter rendering through authored recipe payloads. |
| V3-D6-SHADOW-CAPTURES | Complete evidence refresh | Shadow release-gate sidecars now include required `render_capture_png` artifacts with checked SHA-256 values for base, edge-crossing, and pipeline fixtures. |
| V3-VC01-RELATED | Complete initial | `tui-vfx-recipes` commit `ec25b34`: `pipeline-validator --rules` surfaces malformed `metadata.related_themes` with path-style parse diagnostics. |
| V3-AUTH01 | Complete initial | `tui-vfx-recipes` commit `c0aca72`: scene guide became a ten-section V3 authoring ladder from toast quickstart through scenes, pipelines, I/O, procedural sources, tooling, and debug recipe standards. |
| V3-AUTH01-FOLLOW | Complete initial | Sibling authoring docs now point from the ten-section scene guide to canonical vocabulary, schema/generated API references, procedural source catalog, tooling/validator guidance, and debug fixture standards. Future lifecycle/elevation work can still promote deeper generated ingredient catalogs. |
| V3-PREVIEW01-ALIASES | Complete initial | `tui-vfx-recipes` commit `e52a510`: public preview boundary and prelude now expose `PlaybackPlan`, `PlaybackController`, `PlaybackRecipeBridge`, `V3FrameSnapshot`, and `render_v3_frame_to_buffer` compatibility aliases while preserving legacy `Preview*` names. |
| V3-SPATIAL01-DEBUG | Complete initial | `tui-vfx-recipes` commit `e52a510`: primitive-first debug recipes now demonstrate `sample_surface_radius`, `sample_surface_radius_from`, and `sample_surface_angle_from` through V3 filter dimming with validator/probe/debug-QC evidence. |
| V3-VC09-CONTENTSLIDE01 | Complete evidence checkpoint | `tui-vfx-recipes` commit `e52a510`: `content_slide_shift` is tracked as replacement evidence with output+probe mismatch owner-review status. |
| V3-VC09-CONTENTGLITCH01 | Complete evidence checkpoint | `tui-vfx-recipes` commit `4b939fe`: `content_glitch_shift` is tracked as replacement evidence with output+probe mismatch owner-review status. |
| V3-CI02-INVENTORY01 | Complete inventory checkpoint | Release-gate inventory found 9 complete-pass records, 2 explicit fails, 2 stale/incomplete records, and 6 missing records; remaining offscreen/GTD/role-scope captures stay active under V3-CI02. |
| V3-CI02-OFFSCREEN-CONTENT01 | Complete evidence refresh | `offscreen_content_slide_shift.evidence.record.json` now includes a `render_capture_png` entry with SHA-256 `edd3a3544a6e6fc92a015edc72628cbafbade599d13502fc89c4b85234968fb2`; `offscreen_follow_lag` remains the stale offscreen capture. |
| V3-QDOC01-LINKS01 | Complete initial | Broken historical V3 doc links in the upgrade audit workflow and tooling/appendix chapters were repaired or converted to plain-text historical references; broader stale-status reconciliation remains under V3-DOCS01. |
| V3-TOOL01-THINPLAYER-SCOPE | Complete assessment | Headless summary gap is packaging/surface consolidation, not engine work: first implementation slice should add `tools/tui-vfx-horseman` around existing `play_recipe`/`v3_play_recipe` behavior while avoiding `gtd-movie` timeline/composer semantics. |
| V3-VIEW01-SCOPE | Complete assessment | Normalized IR viewer gap is presentation, not schema work: first implementation slice should add an offline `pipeline-validator` normalized execution-graph explorer over existing normalized IR loader/dump APIs. |
| V3-DOCS01-QDOC01-COMPLETE | Complete initial | Focused V3 docs lifecycle/status reconciliation updated the V3 index, lifecycle plan, and upgrade-plan index; local Markdown link audit over `docs/design` and `docs/tooling` checked 70 files with 0 broken local links. |
| V3-DOCS01-LIFECYCLE02 | Complete clarification | Current reconciliation pass made the V3 index expose lifecycle buckets directly and updated the lifecycle plan to state remaining docs-lifecycle maintenance as active follow-up rather than complete-final. |
| V3-CI02-HEADLESS01 | Complete evidence checkpoint | Headless release-gate records now document passing probe/lowering/trace evidence where available and classify remaining `render_capture_png` gaps as blocked-on-explicit-owner-visual-capture; `probe_midnight_switchboard` remains the technical blocker to fix. |
| V3-TOOL01-THINPLAYER-PACKAGE | Complete initial | `tui-vfx-recipes` commit `c4401ff`: packaged `tui-vfx-horseman` workspace package provides text and `--json` recipe playback summaries through existing preview/cutover APIs while preserving legacy fallback. |
| V3-VIEW01-EXPLORER | Complete initial | `tui-vfx-recipes` commit `a2ef9f3`: `pipeline-validator --explore-normalized` prints identity, contracts, scene layers, and pipeline step-tree summaries without changing normalized IR schema or runtime paths. |
| V3-REGION01-COMPLETE-INITIAL | Complete initial | `tui-vfx-recipes` commit `a2ef9f3`: region-compression support is proven from authoring/schema through normalized/compiled/output bridge, with `shader_region_compression_scope.json` as the debug fixture. |
| V3-CI02-HEADLESS02 | Complete inventory refresh | Headless release-gate refresh on 2026-04-24 records 19 evidence sidecars: 11 passing records and 8 blocked/failing records (`offscreen_follow_lag`, `offscreen_scene_full_stack`, `role_scope_border_style`, `role_scope_scene_pipeline`, and four provisional GTD records). No GUI/X11/Zutty/Xvfb capture was run. |
| V3-CI02-MIDNIGHTSWITCHBOARD01 | Complete technical blocker fix | `tui-vfx-recipes` commit `8a2eca7` plus `docs/tooling/probe_midnight_switchboard.evidence.record.json`: `midnight_switchboard` now reports successful combined/lifecycle probe analysis with configured style/shader effects observed. |
| V3-PREVIEW01-IMPORTS | Complete initial | `tui-vfx-recipes` commit `8a2eca7`: examples, tests, and tooling imports/use sites now use canonical `PlaybackPlan`, `PlaybackController`, `PlaybackRecipeBridge`, `V3FrameSnapshot`, and `render_v3_frame_to_buffer` where safe while retaining `Preview*` compatibility seams. |
| V3-SHOW01-MADEIRA-HEADLESS | Complete initial | `tui-vfx-recipes` commit `8a2eca7`: Madeira has diagnostic-clean headless strict/probe/player evidence and `docs/scene/MADEIRA_HEADLESS_PARITY.md`; visual owner review remains separate. |
| V3-DEMO01-HEADLESS-PLAYER | Complete initial | `tui-vfx-recipes` commit `bffe815`: `tui-vfx-horseman --corpus <dir> --json` checks recursive V3 recipe discovery/loading through the demo/player seam, warning on fallback or load errors; demo tests cover disk refresh/reload behavior without GUI playback. |
| V3-M01-CRITICAL-GROUP | Complete initial / owner-audit blocked | `tui-vfx-recipes` commit `d2fe1e4`: VC-09 report includes the current 10-pair critical evidence group, provisional owner-audit buckets, and explicit `OWNER-RECIPE-AUDIT` blocker for complete-final corpus claims. |
| V3-TRACE01-SCENE-LIFECYCLE | Complete slice | `tui-vfx-recipes` commit `d2fe1e4`: compiled V3 trace runs sample enter/dwell/exit frames and emit scene-composition events that respect lifecycle-gated layer visibility. Broader trace parity remains active. |
| V3-Q01-CH80-LEDGER | Complete initial | Chapter 80 open questions now include a current closure ledger for accepted/implemented, active implementation-shaped, and owner/downstream decision questions so agents do not rediscover settled decisions. |

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
<!-- <VERS>END OF VERSION: 0.14.25</VERS> -->
