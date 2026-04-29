<!-- <FILE>docs/new_kernel/K2_6_RECIPE_MIGRATION_LOOP_PRD.md</FILE> - <DESC>K2.6 PRD for the v3.1 recipe-by-recipe migration mapping loop</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Define the repeatable v3.1-only recipe migration mapping loop without mutating recipe corpora.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — align loop with K2.6 coverage/timeline/diff commands and reviewed tooling inspirations.</CLOG> -->

# K2.6 Recipe Migration Mapping Loop PRD

## Purpose

Define the repeatable recipe-by-recipe migration loop for mapping legacy debug
recipes into canonical v3.1 recipe fixtures.

The loop is a process and evidence contract. It tells future agents how to pick
a recipe or family, inspect source evidence, propose a v3.1 mapping, classify
unmapped capability, and decide whether migration can proceed now. It does not
loosen the v3.1 contract and does not make legacy recipes authoritative.

## Authority boundaries

| Surface | Role in the loop | Authority level |
| --- | --- | --- |
| `schemas/v3.1/contract/` | Stable contract schema evidence. | Schema authority. |
| `crates/tui-vfx-contract/` | Rust source of truth for stable contract wire shape. | Schema authority. |
| `schemas/v3.1/next/` | Proof-pipeline concepts when the candidate uses not-yet-promoted primitive pipeline fields. | Explicitly scoped proof authority. |
| `crates/tui-vfx-next/` | Rust proof source for not-yet-promoted pipeline concepts. | Explicitly scoped proof authority. |
| `descriptors/v3.1/packs/primitive.json` | Current primitive descriptor pack for source/effect/input availability. | Descriptor authority. |
| `descriptors/v3.1/` | Descriptor artifacts and pack structure. | Descriptor authority. |
| `../tui-vfx-recipes/recipes/v3.1/debug_recipes/` | Canonical migrated fixture corpus. | Canonical fixture evidence. |
| `../tui-vfx-recipes/recipes/debug_recipes/` | Legacy recipe behavior and field evidence. | Read-only source evidence only. |
| `tui-vfx-contract-cli` | Structural validation. | Automation authority for contract validity. |
| `tui-vfx-player-cli` | Player reports, frame evidence, coverage, gap reports. | Automation authority for player/render evidence. |

The loop must not use non-`v3.1` schema directories, legacy runtime crates, or
legacy recipe tooling as schema authority. Legacy tooling can inspire workflow
shape only when already source-reviewed in K2 tooling docs.

## Non-goals

- No recipe corpus mutation by the mapping agent unless a later packet gives
  explicit migration-write authority.
- No aliases or compatibility shortcuts that make old recipe JSON validate as
  v3.1.
- No dependency from `tui-vfx-player` or `tui-vfx-player-cli` to legacy
  `tui-vfx-recipes` runtime crates.
- No visual parity claim from a successful player render. Visual parity remains
  a separate evidence gate.
- No schema, descriptor, value-kind, scope-kind, or write-policy addition during
  a mapping pass. The mapping pass recommends additions with evidence.

## Required loop inputs

Each batch must start with an explicit input block in the batch memo:

| Input | Required content |
| --- | --- |
| Legacy source recipe path(s) | One recipe file or a small family from `../tui-vfx-recipes/recipes/debug_recipes/`; read-only. |
| Current v3.1 fixture references | Similar fixtures from `../tui-vfx-recipes/recipes/v3.1/debug_recipes/`, if any. |
| Descriptor pack(s) | Usually `descriptors/v3.1/packs/primitive.json`; include exact path and digest if the batch script records one. |
| Schema/contract references | The specific contract or proof schema/Rust paths consulted. Do not cite generic `v3` authority. |
| Player/validator reports | Current report files or command outputs used to establish baseline support. |
| Selection rationale | Why this recipe/family was selected now. |
| Acceptance target | `canonicalReady`, or the exact blocker status the batch expects to resolve later. |

Use portable path variables in commands where possible:

```bash
export RECIPE_REPO="${RECIPE_REPO:-../tui-vfx-recipes}"
export TMPDIR="${TMPDIR:-/tmp}"
```

## Selection and prioritization

Pick the next recipe or family by applying these filters in order:

1. **Safety filter** — exclude recipes whose legacy intent is unclear enough
   that a mapping would invent behavior. Classify those as
   `blockedByUnknownLegacyIntent` until source notes or human review clarify the
   intent.
2. **Representation gap** — prefer families reported by `migration-gap` as
   unrepresented, then partially represented families with high confidence
   legacy evidence.
3. **Descriptor readiness** — prefer recipes whose sources/effects and input
   shapes are already present in the v3.1 descriptor pack.
4. **Player readiness** — prefer recipes whose primitive effects can be rendered
   or whose adapter gaps are already classified by `primitive-adapter-gap` and
   field-coverage reports.
5. **Base-reference value** — prefer simple single-primitive reference recipes
   before compositions. A primitive baseline makes later composition failures
   diagnosable.
6. **Family coherence** — when several recipes share one format, migrate a small
   family slice together only if the shared structure is stable and the variants
   differ by clear parameters such as color, direction, phase, or state.
7. **Visual review cost** — defer high-parity, high-motion, or ambiguous visual
   recipes until timeline/diff evidence is sufficient to make review efficient.

Batch size should be small: one recipe, or one tightly related family slice.
The batch memo must say why the size is safe.

## Baseline commands

A migration agent must run or consume fresh equivalent reports before deciding
that a recipe maps cleanly. Use the exact command set available in the current
workspace; if a command is unavailable, record the failure and use the narrowest
fallback evidence.

```bash
export RECIPE_REPO="${RECIPE_REPO:-../tui-vfx-recipes}"
export TMPDIR="${TMPDIR:-/tmp}"

cargo run -q -p tui-vfx-player-cli -- migration-gap \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --json \
  > "$TMPDIR/tui-vfx-migration-gap.json"

cargo run -q -p tui-vfx-player-cli -- primitive-adapter-gap \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive --json "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  > "$TMPDIR/tui-vfx-primitive-adapter-gap.json"

cargo run -q -p tui-vfx-player-cli -- inventory-recipes \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive --json "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  > "$TMPDIR/tui-vfx-inventory-report.json"

cargo run -q -p tui-vfx-player-cli -- render-frame \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive --json "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  > "$TMPDIR/tui-vfx-visual-frame-report.json"
```

When the field-coverage command exists, the loop must also consume its report.
The batch memo should name the schema version and summarize
`usedButUnhandledInputFields`, declared-but-unused descriptor fields, and any
per-primitive missing field handling that affects the candidate mapping.

After a candidate v3.1 fixture exists in a future write-authorized lane, the
agent must run structural validation and focused player reports on that fixture
before claiming `canonicalReady`.

## Mapping decision workflow

For each source recipe or family:

1. **Read source evidence** — inspect only the selected legacy recipe path(s)
   and any immediately referenced assets or family companions needed to
   understand intent. Keep legacy files read-only.
2. **Inventory authored fields** — list source, effect, timing, scope, signal,
   asset, style, and composition fields that carry behavior. Mark decorative or
   metadata-only fields separately.
3. **Find v3.1 homes** — map each behavior-carrying field to one of:
   descriptor input, source descriptor, effect descriptor, value kind, scope
   kind, write policy, lifecycle/time contract, scene layer, player adapter, or
   migration rule.
4. **Compare current authority** — verify that the proposed home exists in the
   v3.1 schemas, Rust contract surfaces, descriptor pack, and player reports.
5. **Classify gaps** — use exactly one primary status and any supporting
   recommendation types from this PRD.
6. **Draft canonical mapping** — provide the smallest v3.1 recipe shape that
   preserves intent. For doc-only mapping passes, keep it as a proposed JSON
   excerpt or field table, not a corpus mutation.
7. **Define acceptance evidence** — list the validator/player/timeline/diff or
   visual review artifacts needed before accepting a real migrated fixture.
8. **Write the batch memo** — make the conclusion machine-checkable enough for
   the next agent to continue without re-reading unrelated history.

## Stable statuses

Use one primary status per recipe or family. Supporting notes may explain
secondary blockers, but dashboards and batch summaries should aggregate the
primary status.

| Status | Use when | Required next action |
| --- | --- | --- |
| `canonicalReady` | The candidate can be represented with current v3.1 schema, descriptors, and player adapter support, and validation/player evidence can prove it. | Create or approve the migrated fixture in a write-authorized lane; keep evidence artifacts. |
| `descriptorExpansionNeeded` | The schema can represent the behavior, but the descriptor pack lacks a source/effect/input/output entry or allowed value. | Propose descriptor changes with source evidence and tests. |
| `schemaDecisionNeeded` | The behavior needs a new value kind, scope kind, write policy, lifecycle/timing construct, recipe field, or other contract-visible shape. | Escalate a schema decision with alternatives and migration impact. |
| `adapterNeeded` | The v3.1 contract and descriptors can represent the behavior, but the player cannot render/sample the primitive honestly. | Add or schedule a player adapter and report tests. |
| `sourceDecisionNeeded` | Legacy evidence is inconsistent, duplicated, deprecated, or conflicts with current authoring direction. | Ask for source/authoring policy before mapping. |
| `semanticReviewNeeded` | The behavior maps mechanically but the meaning may change, collapse distinct concepts, or introduce a public ingredient too casually. | Request design/architecture review before descriptor or schema changes. |
| `visualParityPending` | The mapping is structurally and mechanically plausible, but acceptance depends on visual comparison across frame/timeline/diff evidence. | Produce visual parity evidence before acceptance. |
| `blockedByUnknownLegacyIntent` | The source recipe does not make expected behavior clear enough to map without guessing. | Gather source notes, screenshots, human review, or a simpler representative fixture. |

## Recommendation types

Use these exact recommendation values in batch memos and future structured
reports:

| Recommendation | Meaning |
| --- | --- |
| `addDescriptorInput` | Add or refine an input on an existing descriptor. |
| `addSourceInput` | Add or refine an input on an existing source descriptor. |
| `addEffectDescriptor` | Add a missing effect descriptor. |
| `addSourceDescriptor` | Add a missing source descriptor. |
| `addValueKind` | Add a new value/value-source kind or accepted value shape. |
| `addScopeKind` | Add a new scope or selection kind. |
| `addWritePolicy` | Add a new write/composition policy. |
| `addPlayerAdapter` | Add player support for a represented primitive. |
| `addMigrationRule` | Add a deterministic source-to-v3.1 rewrite rule that does not change schema semantics. |
| `manualRewriteRecommended` | Do not automate; a human or design-aware agent should rewrite the recipe against v3.1 ingredients. |
| `deferUntilSemanticDecision` | Stop migration until a semantic/schema/descriptor decision is made. |

A recommendation is not approval to change the schema or descriptors. It is a
classified request with evidence.

## Pass/fail gates

A batch passes only when all applicable gates are satisfied.

| Gate | Pass condition | Fail classification |
| --- | --- | --- |
| Source safety | Legacy source paths were read only; no corpus file was changed. | Batch invalid. Restore changes before reporting. |
| V3.1 authority | All mapping claims cite v3.1 schemas, contract Rust, descriptors, or player reports. | `schemaDecisionNeeded` or evidence gap. |
| Field accounting | Every behavior-carrying legacy field is mapped, classified as intentionally dropped, or attached to a recommendation. | `descriptorExpansionNeeded`, `schemaDecisionNeeded`, `sourceDecisionNeeded`, or `semanticReviewNeeded`. |
| Descriptor fit | Required source/effect/input/output descriptors exist and match the intended values. | `descriptorExpansionNeeded`. |
| Schema fit | Required value kind, scope kind, write policy, lifecycle/time field, and recipe shape exist. | `schemaDecisionNeeded`. |
| Player fit | Current player can render/sample represented primitives honestly, including styled-cell provenance when style/color/role evidence matters. | `adapterNeeded`. |
| Automation evidence | Relevant CLI reports are attached or named with paths and summary counters. | Evidence gap; do not mark ready. |
| Visual acceptance | Visual parity is either not required for the batch or has frame/timeline/diff/human evidence. | `visualParityPending`. |
| Review hygiene | A third-party review and AI de-slop pass have checked the batch memo or produced fixture changes. | Batch incomplete. |

## Evidence artifacts

Each batch should produce a memo under a future batch-specific path chosen by the
packet owner. The memo must include these sections:

```text
# Recipe Migration Batch Memo: <portable recipe/family name>

## Inputs
- Legacy source paths:
- Candidate v3.1 fixture paths:
- Descriptor packs:
- Schema/contract paths consulted:
- CLI report paths:

## Selection rationale

## Authored legacy behavior inventory

## Proposed v3.1 mapping

## Unmapped fields and classified gaps

## Status and recommendations
- Primary status:
- Recommendations:

## Required additions before migration
- Descriptor additions:
- Schema decisions:
- Player adapters:
- Migration rules:

## Acceptance gates
- Structural validation:
- Player report evidence:
- Field coverage evidence:
- Timeline/diff evidence:
- Visual parity evidence:

## Review and de-slop
- Third-party review:
- AI de-slop changes:

## Final decision
- Can migration proceed now?
- If not, who owns the next decision?
```

Machine-readable report artifacts should remain CLI-owned. The migration memo
may summarize JSON with `jq`, but it should link to or name the actual report
files rather than retyping large outputs.

## Legacy debug recipes as read-only source evidence

Legacy debug recipes are evidence of authored intent and migration pressure.
They are not v3.1 fixtures, not schema examples, and not validation targets for
`v3.1` contract correctness.

Rules:

1. Read legacy recipe files narrowly: the selected file, immediate family peers,
   and referenced assets only when needed.
2. Preserve paths exactly in the memo so future agents can reproduce the source
   read.
3. Do not edit, normalize, reformat, rename, or move legacy source files.
4. Do not add compatibility aliases so old fields validate under v3.1.
5. When legacy behavior is unclear, classify the blocker instead of guessing.
6. When legacy behavior is undesirable for v3.1, recommend
   `manualRewriteRecommended`, `sourceDecisionNeeded`, or
   `deferUntilSemanticDecision` rather than preserving it literally.

## CLI automation authority

The CLI remains the automation authority for migration evidence. Humans and GUI
surfaces may help inspect results, but batch acceptance is based on reportable
commands.

Current authority split:

```text
tui-vfx-contract-cli validate-recipe   -> structural v3.1 validity
tui-vfx-player-cli render-recipe       -> player run/frame status
tui-vfx-player-cli render-frame        -> visual-frame rows/cells/provenance
tui-vfx-player-cli inventory-recipes   -> corpus descriptor/source/effect inventory
tui-vfx-player-cli migration-gap       -> legacy/v3.1 family representation gaps
tui-vfx-player-cli primitive-adapter-gap -> primitive support blocker classes
tui-vfx-player-cli primitive-field-coverage -> authored input handling evidence
tui-vfx-player-cli render-timeline     -> temporal multi-frame evidence
tui-vfx-player-cli render-frame-diff   -> changed-cell evidence
```

Any GUI player workflow must call into or consume these authorities. It may not
replace them with private state, screenshots alone, or legacy runtime behavior.

## Reviewed tooling inspiration sources

The migration loop reuses hard-earned workflow lessons from reviewed legacy tools without treating them as v3.1 authority:

| Source | Adopt for integration planning | Boundary |
| --- | --- | --- |
| `../tui-vfx-recipes/examples/demo.rs` | Human playback flow, reload/pause/scrub, diagnostics visibility, and preview ergonomics. | UX inspiration only; no legacy runtime dependency. |
| `../tui-vfx-recipes/tools/pipeline-validator` | Mode-based CLI and staged reports for repeatable gates. | Do not preserve legacy pipeline semantics as canonical. |
| `../tui-vfx-recipes/tools/recipe-probe` | Timeline, diff, focus-cell, and causation concepts for iterative recipe debugging. | Clean-room reports own the new schemas. |
| `../tui-vfx-recipes/tools/tui-vfx-trace` | Selectors, stage masks, and NDJSON streams as later trace/evidence ideas. | Trace storage/stage machinery is later work. |
| `../tui-vfx-recipes/tools/tui-vfx-horseman` | Compact corpus summaries for batch triage. | Summary labels must map to v3.1 vocabulary before adoption. |
| `../tui-vfx-recipes/tools/recipe-source-capture` | Reproducible generated-source artifacts for migration audit trails. | Captured legacy source is evidence, not canonical input. |
| `../tui-vfx-recipes/tools/recipe-signals-doc` | Generated-doc drift checks for descriptor/signal surfaces. | Old docs do not define v3.1 signal schema. |
| `../tui-vfx-recipes/tools/recipe-validator` | Historical warning categories and migration clues. | Deprecated; must not become new authority. |

## Reusable migration-agent prompt template

```text
You are a local recipe migration mapping agent for tui-vfx v3.1.

Given:
- legacy source recipe path(s): <paths under $RECIPE_REPO/recipes/debug_recipes>
- canonical v3.1 descriptor pack(s): <paths, usually descriptors/v3.1/packs/primitive.json>
- current v3.1 fixture corpus: <$RECIPE_REPO/recipes/v3.1/debug_recipes>
- player/validator reports: <paths to migration-gap, inventory, render-frame,
  primitive-adapter-gap, field-coverage, timeline/diff if available>

Rules:
- Treat legacy debug recipes as read-only source evidence only.
- Use v3.1 schemas, tui-vfx-contract Rust, v3.1 descriptors, and player reports
  as authority.
- Do not mutate recipe corpus files.
- Do not add dependencies or compatibility aliases.
- Prefer portable paths such as $RECIPE_REPO and $TMPDIR.

Produce:
1. Proposed canonical v3.1 mapping for each selected recipe or family.
2. Inventory of behavior-carrying legacy fields and where each maps in v3.1.
3. Unmapped fields, intentionally dropped fields, and uncertainty notes.
4. Required descriptor, schema, player-adapter, or migration-rule additions.
5. One primary status from:
   canonicalReady, descriptorExpansionNeeded, schemaDecisionNeeded,
   adapterNeeded, sourceDecisionNeeded, semanticReviewNeeded,
   visualParityPending, blockedByUnknownLegacyIntent.
6. Recommendation values from:
   addDescriptorInput, addSourceInput, addEffectDescriptor,
   addSourceDescriptor, addValueKind, addScopeKind, addWritePolicy,
   addPlayerAdapter, addMigrationRule, manualRewriteRecommended,
   deferUntilSemanticDecision.
7. Whether migration can proceed now.
8. Whether visual parity evidence is required before acceptance.
9. Evidence artifact paths and report summary counters.
```

## Example decision table

| Observation | Status | Recommendation |
| --- | --- | --- |
| Legacy recipe uses a known effect id, all inputs exist in the descriptor, and current player evidence renders the primitive honestly. | `canonicalReady` | `addMigrationRule` if an automated rewrite is useful; otherwise none. |
| Legacy recipe uses a known effect but one authored option is absent from the descriptor input enum. | `descriptorExpansionNeeded` | `addDescriptorInput`. |
| Legacy recipe needs a new scope selection shape. | `schemaDecisionNeeded` | `addScopeKind`, `deferUntilSemanticDecision`. |
| Legacy recipe can be represented, but player reports the effect unsupported or style evidence cannot be expressed. | `adapterNeeded` | `addPlayerAdapter`. |
| Legacy family contains conflicting variants where current docs do not identify the intended one. | `sourceDecisionNeeded` | `manualRewriteRecommended` or `deferUntilSemanticDecision`. |
| Legacy behavior would promote a one-off combination into a public primitive without rule-of-three support. | `semanticReviewNeeded` | `deferUntilSemanticDecision`. |
| Candidate renders and validates, but acceptance depends on motion or color parity against legacy output. | `visualParityPending` | `manualRewriteRecommended` until timeline/diff evidence is reviewed. |

## Batch completion checklist

Before reporting a migration batch complete, confirm:

- Required grounding docs and OFPF orientation were attempted as the packet asks.
- Legacy source recipes were read only.
- No recipe corpus paths were mutated.
- No dependencies were added.
- All behavior-carrying fields were accounted for.
- One primary status was assigned per recipe or family.
- Recommendation values use the stable vocabulary in this PRD.
- CLI report artifacts were produced or unavailable commands were recorded.
- Visual parity was explicitly accepted, deferred, or marked pending.
- Third-party review and AI de-slop pass were completed for touched artifacts.

<!-- <FILE>docs/new_kernel/K2_6_RECIPE_MIGRATION_LOOP_PRD.md</FILE> - <DESC>K2.6 PRD for the v3.1 recipe-by-recipe migration mapping loop</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
