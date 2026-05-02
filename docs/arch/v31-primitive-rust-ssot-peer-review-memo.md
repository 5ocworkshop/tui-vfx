<!-- <FILE>docs/arch/v31-primitive-rust-ssot-peer-review-memo.md</FILE> - <DESC>Peer-review memo on the v3.1 primitive Rust-SSOT plan, current compost substrate, legacy compositor reference, schema generation, and remaining blockers</DESC> -->
<!-- <VERS>VERSION: 0.2.1</VERS> -->
<!-- <WCTX>Add a source-alignment note to the author response so the review artifact points readers from early CellFilter/Sampler labels to the final current-contract FrameFilter/CoordinateSampler runtime names.</WCTX> -->
<!-- <CLOG>0.2.1: PATCH — note that final production docs align trait names to current EffectDomain vocabulary (`FrameFilterRuntime`, `CoordinateSamplerRuntime`) while preserving this memo as review history.
0.2.0: MINOR — add author response and disposition section closing the review loop.
0.1.0: INIT — write peer-review memo with original findings and addendum on hard blockers versus sequencing decisions.</CLOG> -->

# Peer-review memo: v3.1 primitive Rust SSOT plan

## Scope reviewed

Repository: `/usr/projects/tui-vfx`

Discovery used:

```bash
/usr/local/bin/ofpf-load --root /usr/projects/tui-vfx
ofpf-* discovery/read/search commands
```

Reviewed areas:

- `docs/arch/v31-primitive-rust-ssot.md`
- `docs/arch/v31-primitive-rust-ssot-implementation-plan.md`
- `docs/arch/primitive-workbench-schema-driven-workflow.md`
- current clean-sheet compositor crate: `crates/tui-vfx-compost`
- legacy reference compositor crate: `crates/tui-vfx-compositor`
- v3.1 contract schema DTOs and checked generated schema fixtures under `schemas/v3.1/contract`
- current provisional descriptor pack: `descriptors/v3.1/packs/primitive.json`
- current `xtask` command surface and generation behavior

## Original answer / critical review

### Executive summary

I would **not start bulk primitive porting yet**. The direction is promising, but the plan currently has a few load-bearing mismatches:

1. It says “schema is SSOT” while the Rust-SSOT proposal says “Rust primitive is the only SSOT.” This needs a precise taxonomy.
2. The proposed `EffectRuntime::apply(inputs, CellView, EffectCtx)` fits filters/shaders, but not masks, samplers, content transforms, or sources.
3. `primitive.json` and schemas are generated artifacts/provisional references, so the plan must not treat current descriptor content as semantic truth.
4. `xtask descriptors gen` does **not exist today**. Current schema generation is test-driven via `UPDATE_SCHEMAS=1`; current `xtask docs` generates legacy `effect_schemas.json`, not v3.1 primitive descriptors.
5. Replacing `primitive.json` with a one-effect Phase 1 pack would break today’s descriptor consumers unless handled as a sidecar/partial registry strategy.
6. The current clean compositor still depends on `tui-vfx-style` for `shader.linearGradient`, which conflicts with the doc’s “no legacy imports” wording unless “legacy” only means `tui-vfx-compositor`.

---

## Critical feedback

### 1. Resolve “schema SSOT” vs “Rust SSOT” before implementation

The workbench doc says the schema/descriptor is the source of truth and tooling derives implementation surfaces from it. The Rust-SSOT doc inverts that: Rust primitive declaration is the only edit point, and `primitive.json` is regenerated from it.

Evidence:

- `docs/arch/primitive-workbench-schema-driven-workflow.md:50-120` — schema/descriptor-first block diagram.
- `docs/arch/primitive-workbench-schema-driven-workflow.md:122-266` — explicitly frames Rust-SSOT as inverted peer-review direction.
- `docs/arch/v31-primitive-rust-ssot.md:181-194` — says Rust edit is the only way to change descriptor content.

Recommendation: define two SSOT layers:

- **Contract schema SSOT:** Rust DTOs in `tui-vfx-contract` generate JSON Schemas under `schemas/v3.1/contract`.
- **Primitive catalog instance SSOT:** either Rust primitive declarations or descriptor JSON entries generate `primitive.json`.

If the project principle is “schema is our SSOT,” then Rust-SSOT should be reframed as **schema-constrained Rust authoring for primitive descriptor instances**, not “Rust is the SSOT” in the same sense.

---

### 2. The runtime trait is too filter-shaped for the full primitive set

The proposed trait:

```rust
fn apply(inputs, cell: &mut CellView<'_, Self>, ctx: &EffectCtx<'_>);
```

is viable for cell filters/shaders, but legacy primitives have different runtime shapes:

- Filter mutates a `Cell`: `tui-vfx-compositor/src/traits/filter.rs:68-93`
- Mask returns visibility: `tui-vfx-compositor/src/traits/mask.rs:9-18`
- Sampler returns source coord + displacement delta: `tui-vfx-compositor/src/traits/sampler.rs:30-106`
- Sources materialize surfaces in current compost: `crates/tui-vfx-compost/src/source/*`
- Content transforms are not simple per-cell style mutations.

Current compost also has domain-specific execution assumptions:

- `crates/tui-vfx-compost/src/render/fnc_apply_effect_stack.rs:39-48` rejects active content/style/sampler/mask/filter stages today.
- `crates/tui-vfx-compost/src/validation/orc_validate_render_contract.rs:62-82` only accepts `shader.linearGradient`.
- `crates/tui-vfx-compost/src/shaders/cls_linear_gradient_node.rs:16-48` is a shader-specific wrapper.

Recommendation: replace one universal `EffectRuntime` with domain-specific runtimes, e.g.:

- `CellShaderRuntime`
- `CellFilterRuntime`
- `MaskRuntime`
- `SamplerRuntime`
- `ContentTransformRuntime`
- `SourceRuntime`

or use an `EffectDomainRuntime` enum/adapter where each domain has an explicit call shape.

---

### 3. Do not use current `primitive.json` as truth; use it only as a provisional inventory

`primitive.json` is supposed to become an output, and current schemas/descriptor content may be wrong.

Evidence:

- Current pack has 120 effects and 7 source descriptors, but that is provisional.
- The Rust-SSOT doc itself lists observed descriptor drift: `docs/arch/v31-primitive-rust-ssot.md:14-27`.
- The workbench doc says generated validation should prove generated and human-owned layers agree: `docs/arch/primitive-workbench-schema-driven-workflow.md:542-553`.

Recommendation: in the plan, call current `primitive.json` a **migration seed / provisional catalog**, not “the primitive corpus truth.” Each port should derive truth from:

1. v3.1 contract schema shape,
2. current alias/canonicalization behavior,
3. legacy implementation semantics where present,
4. corpus fixtures,
5. explicit author decision where no legacy ancestor exists.

---

### 4. Xtask reality differs from the proposed plan

Today, `xtask` has only `Audit`, `Docs`, and `Recipes` command groups.

Evidence:

- `xtask/src/main.rs:23-40` — current top-level commands.
- `xtask/src/docs/gen_effect_schemas.rs:12-16` — generated `docs/generated/effect_schemas.json` pulls from legacy/current non-v3.1 surfaces: `tui_vfx_compositor`, `tui_vfx_style`, `tui_vfx_content`, etc.
- `crates/tui-vfx-contract/tests/test_schema_generation.rs:47-197` — v3.1 contract schemas are generated from `tui-vfx-contract` DTOs.
- `crates/tui-vfx-contract/tests/test_schema_generation.rs:607-614` — schemas update only when tests run with `UPDATE_SCHEMAS`.

Recommendation: add a real generator plan section for:

```bash
cargo xtask schemas gen/check       # or document UPDATE_SCHEMAS if kept
cargo xtask descriptors gen/check   # new primitive.json generator
```

Without this, “schema is generated” and “descriptor is generated” are handled by unrelated mechanisms.

---

### 5. Phase 1 “one-effect primitive.json” is risky

The implementation plan says Phase 1 emits `primitive.json` containing exactly `filter.dim` plus existing source descriptors.

Evidence:

- `docs/arch/v31-primitive-rust-ssot-implementation-plan.md:67-73`
- Current compost tests read `descriptors/v3.1/packs/primitive.json`: `crates/tui-vfx-compost/tests/test_corpus_load_authoring_shorthand.rs:22-30`, `crates/tui-vfx-compost/tests/direct_recipe/support.rs:20-28`.

That would be a major behavioral change for tests/tools that expect the current descriptor inventory.

Recommendation: Phase 1 should not overwrite the full pack with a one-effect pack. Safer alternatives:

- generate a **sidecar** pack first, e.g. `primitive.generated.partial.json`;
- or emit full `primitive.json` where unported effects are carried forward as descriptor-only bootstrap entries;
- or keep checked-in `primitive.json` unchanged until enough registry coverage exists, then switch the round-trip lock to full-pack mode.

---

### 6. “No legacy imports” needs sharper wording

The Rust-SSOT doc prohibits imports from `tui-vfx-compositor`, `tui-vfx-style`, and `tui-vfx-content`.

Evidence:

- `docs/arch/v31-primitive-rust-ssot.md:556-565`
- Current `tui-vfx-compost` depends on `tui-vfx-style`: `crates/tui-vfx-compost/Cargo.toml:18-24`
- Current linear gradient implementation uses style types: `crates/tui-vfx-compost/src/shaders/fnc_linear_gradient_style.rs:6-12`

If the clean-sheet rule is “do not depend on legacy compositor,” then the doc is too strict. If the rule is “pure v3.1 means no style/content legacy runtime dependencies,” then Phase 0 must include extracting/reimplementing shader/style math into compost-native helpers.

Recommendation: explicitly distinguish:

- forbidden: `tui-vfx-compositor` legacy runtime DTOs/traits;
- temporarily allowed or forbidden-by-date: `tui-vfx-style`, `tui-vfx-content`, `tui-vfx-shadow`.

---

### 7. `Bindable<T>` should be scheduled by value-source semantics, not only wrapper type

The proposal treats `Literal<T>` vs `Bindable<T>` as the descriptor `bindable` flag. That is good for descriptor generation, but insufficient for runtime scheduling.

Evidence:

- `ValueSource` supports literals, parameters, signals, graph values, maps, sampled fields, signal expressions, phase progress, and clocks: `crates/tui-vfx-contract/src/cls_value_source.rs:22-88`.
- Current compost resolves value sources dynamically through `RuntimeContext`: `crates/tui-vfx-compost/src/runtime/fnc_resolve_value_source.rs:13-58`.
- Current `RuntimeMutability` vocabulary is `compileTime`, `phaseStart`, `resetOnly`, `runtime`: `crates/tui-vfx-contract/src/cls_runtime_mutability.rs:11-20`.

Recommendation:

- Keep `Bindable<T>` as authoring/descriptor bindability.
- Add an explicit resolution policy derived from source kind:
  - literal/parameter default: load/element/sample cacheable;
  - clock/phase progress: sample-time;
  - sampled field: per-cell;
  - graph value: topology-dependent, maybe per-node/per-cell.
- Align doc wording: it says `perFrame`, but actual enum is `runtime`.

---

### 8. Outputs should not block early phases

Current descriptors support outputs structurally, but current primitive pack has no effect outputs, and current compost rejects `NodeOutputSource::EffectOutput`.

Evidence:

- `crates/tui-vfx-contract/src/cls_effect_descriptor.rs:39-43`
- `crates/tui-vfx-contract/src/cls_effect_output_spec.rs:8-18`
- `crates/tui-vfx-compost/src/render/fnc_publish_node_outputs.rs:18-31` supports input re-emission and rejects effect output capture.
- `crates/tui-vfx-compost/src/validation/orc_validate_render_contract.rs:87-110` rejects `EffectOutput`.

Recommendation: defer `PrimitiveOutputs` derive until after substrate supports effect output capture. For now, model outputs as an explicit unsupported/future capability.

---

### 9. Move commonality review earlier

The workbench doc is right that commonality should be discovered before broad generation/porting.

Evidence:

- `docs/arch/primitive-workbench-schema-driven-workflow.md:322-327`
- `docs/arch/primitive-workbench-schema-driven-workflow.md:431-460`

Recommendation: add a Phase 0.5 or Phase 1 task: extract shared helpers for:

- apply-to foreground/background/both,
- color blending,
- phase/time normalization,
- coordinate normalization,
- mask visibility sampling,
- sampler displacement,
- enum normalization,
- unsupported input diagnostics.

Otherwise bulk ports will bake in 120 variations of the same plumbing.

---

## Recommended revised gating before work starts

Before implementation, I would ask the plan author to update the plan with these decisions:

1. **SSOT taxonomy:** contract schema SSOT vs primitive catalog instance SSOT.
2. **Runtime trait split:** one trait per execution domain or equivalent domain-specific dispatch.
3. **Generator ownership:** add or document `xtask schemas` and `xtask descriptors`.
4. **Partial-pack strategy:** do not replace full `primitive.json` with only `filter.dim`.
5. **Legacy dependency policy:** clarify whether `tui-vfx-style`/`content` are allowed references or must be eliminated.
6. **Bindable scheduling model:** source-kind-aware resolution cadence.
7. **Source descriptors:** first-class `SourceRuntime`/`SourcePrimitive` path, not an afterthought.
8. **Outputs:** defer derive until effect-output capture exists.
9. **Common helper extraction:** do before bulk porting.

---

## Addendum after reviewing the blocker recap

The blocker recap is mostly directionally correct, but I would revise the blocker boundary. Items 1–3 are real Phase 0 substrate decisions, but they are not the only decisions that can cause broad rework.

### A. Items 1–3 are necessary, but not sufficient, Phase 0 gates

The recap correctly identifies these as trait-shaping substrate decisions:

1. `CellView<'_, P>` enforcement strategy.
2. `Bindable<T>` resolution cadence.
3. v3.1 scalar wrapper type home.

However, the trait surface also depends on **runtime domain shape**. If we choose a single per-cell `EffectRuntime::apply` now, later mask/sampler/content/source ports will either contort around it or force a trait split. That is comparable in churn risk to the three listed blockers.

**Recommended addition to hard preconditions:**

4. **Effect runtime domain model.** Decide whether there is one universal runtime trait or separate runtime traits for cell filters/shaders, masks, samplers, content transforms, and sources.

My recommendation: domain-specific runtime traits from the start, with a shared descriptor trait. This preserves one descriptor/registry story while avoiding a false universal per-cell apply ABI.

### B. Source descriptors should be promoted from peer-review item to early architecture gate

The recap lists source descriptors as architectural decision 4 and says it can be made before/during its phase. I think this is riskier than that because current recipes and compost source materialization already rely on sources.

Current `primitive.json` contains source descriptors alongside effects, and current compost has a `source` module. If the registry is meant to become the runtime/catalog source, sources need a parallel registry path before loader rewiring. Otherwise Phase 4 will have split truth: effect descriptors from registry, source descriptors still from JSON or hand-curated code.

**Recommended decision:** create a `SourceDescriptorPrimitive` / `SourceRuntime` family, but keep it smaller than effect primitives. Sources have a different shape: assets, output contract, lifecycle, source inputs, and materialization rather than cell mutation.

### C. `PrimitiveEnum` second derive is acceptable, but not urgent for Phase 1

I agree the cleanest long-term route is a second derive or trait for enum labels. But Phase 1 can avoid this by manually writing enum descriptor specs for `filter.dim` and the first few ports. The key is to not let enum reflection block substrate pinning.

**Recommendation:** accept `PrimitiveEnum`, but schedule it with the `PrimitiveInputs` derive in Phase 2, not Phase 0/1.

### D. Outputs modeling should explicitly defer until effect-output capture exists

The recap says outputs can be parallel derived or hand-authored. Current evidence says outputs are structurally in the contract but not used by the current pack, and compost rejects `EffectOutput` capture. So output modeling should not be a peer-review blocker for the first primitive ports.

**Recommendation:** keep `PrimitiveOutputs = NoOutputs` as the only supported Phase 1/2 path. Add a `descriptor-only outputs allowed, runtime capture unsupported` note only if a current descriptor needs it.

### E. V3.1-only effects need a triage artifact before bulk porting, not before filter.dim

I agree these should not block Phase 1. They should block the start of bulk Phase 3 only if they are still unclassified.

**Recommended artifact:** `docs/arch/v31-primitive-no-legacy-ancestor-triage.md` or a section in the implementation plan with dispositions:

- author from corpus + alias intent,
- descriptor-only until exercised,
- remove from generated `primitive.json`,
- split into source/transition/non-effect category.

### F. Phase 2 timing: after 3 hand ports is still best

I agree with the current plan’s conservative timing: land derive after 3 hand-ported primitives. But choose the first three to stress different axes of the input model, not merely three similar filters.

Better candidates than only `dim/greyscale/tint`:

1. `filter.dim` — small enum/apply-to, simple channel writes.
2. `filter.tint` or `filter.pillButton` — color, number/integer, boolean, bindable progress, spatial context.
3. one mask or sampler stub/descriptor-only shape — to test the domain split early.

If the runtime-domain model is not resolved, three filters will falsely validate a filter-only trait.

### G. Phase 4 entry criterion should be semantic, not just coverage percentage

The recap asks whether Phase 4 needs 100% corpus load coverage. I would avoid a pure percentage gate. The loader can switch once these invariants are true:

- every descriptor required by corpus load exists in the registry descriptor view,
- source descriptors are available through the same registry/catalog mechanism or explicitly bridged,
- JSON-derived and registry-derived `DescriptorCatalog` compare equal for the supported surface,
- external tools still consume generated `primitive.json`,
- runtime can produce structured `EffectNotImplemented` instead of `UnknownEffect` for descriptor-only primitives.

In practice this likely means near-100% corpus descriptor coverage, but the criterion should be “no semantic delta in load validation,” not only count.

### H. Workbench doc should coexist as authoring workflow, not be simply superseded

If Rust-SSOT is accepted, the workbench doc still has durable value: commonality review, generated validation manifests, migration mapping discipline, fixture generation, and docs/control metadata. I would not deprecate it wholesale.

**Recommendation:** rename/reframe the relationship:

- Rust primitive declaration owns primitive descriptor instances for substrate-linked primitives.
- Workbench remains the workflow layer that can scaffold Rust declarations, migration maps, fixtures, validation, and docs.
- External JSON-first tools can still author proposals, but accepted changes land by updating Rust and regenerating JSON.

### I. The missing generator path is a hidden blocker

The recap did not include this, but it is a practical start blocker for any SSOT lock. Today there is no `xtask descriptors gen/check`, and schema generation lives in contract tests behind `UPDATE_SCHEMAS=1`.

**Recommended hard precondition or Phase 1 gate:** explicitly define generator command ownership before writing the round-trip lock. Otherwise the plan has no stable way to enact “the fix is always rerun codegen.”

### J. Revised blocker classification

I would classify the work as follows:

#### Must decide before primitive runtime ports

1. `CellView<'_, P>` enforcement strategy.
2. `Bindable<T>` resolution/scheduling policy, at least enough for the first ports.
3. v3.1 scalar/wrapper type home.
4. Runtime domain trait split / dispatch ABI.
5. Legacy dependency policy for `tui-vfx-style`, `tui-vfx-content`, and `tui-vfx-compositor`.

#### Must decide before descriptor/codegen lock

6. `xtask descriptors gen/check` command shape.
7. Partial-pack/full-pack/sidecar strategy for generated `primitive.json`.
8. Registry representation for source descriptors, even if runtime source materialization remains separate.
9. SSOT taxonomy wording: schema DTOs vs primitive catalog instances.

#### Can wait until Phase 2+

10. `PrimitiveInputs` derive timing.
11. `PrimitiveEnum` derive details.
12. `PrimitiveOutputs` derive/modeling.
13. V3.1-only effect dispositions, as long as there is a triage artifact before bulk porting.
14. Workbench doc final supersession/coexistence wording.


---

## Author response and disposition

The plan author reviewed this memo and agreed with the main direction, with a few amendments. This section records the closed-loop disposition so future readers do not have to reconstruct which review points were accepted into the next doc revision.

### Accepted without amendment

1. **SSOT taxonomy.** Accepted. The architecture docs should distinguish:
   - contract schema SSOT: Rust DTOs in `tui-vfx-contract` generate `schemas/v3.1/contract/*`;
   - primitive catalog instance SSOT: accepted Rust primitive declarations generate `primitive.json`.

2. **Domain-specific runtime traits.** Accepted. The single `EffectRuntime::apply` shape will be replaced by one descriptor trait plus domain-specific runtime traits:
   - `CellShaderRuntime`
   - `CellFilterRuntime`
   - `MaskRuntime`
   - `SamplerRuntime`
   - `ContentTransformRuntime`
   - `SourceRuntime`

   The registry should keep one descriptor view and separate runtime dispatch maps by domain. The production docs refine the early review labels to match the current `EffectDomain` enum: `CellFilterRuntime` becomes `FrameFilterRuntime`, and `SamplerRuntime` becomes `CoordinateSamplerRuntime`.

3. **`primitive.json` as migration seed, not truth.** Accepted. Current descriptor content is a provisional migration seed. Each primitive port should derive truth from contract schema, alias/canonicalize behavior, legacy semantics where present, corpus evidence, and explicit authorial decisions.

4. **`xtask` reality.** Accepted. `cargo xtask descriptors gen` / `--check` does not exist today and must become an explicit Phase 1 deliverable. Schema generation currently happens through the `UPDATE_SCHEMAS=1` contract-test path; the plan should decide whether to leave that alone or unify schema generation under `xtask` for discoverability.

5. **`Bindable<T>` semantics.** Accepted. `Literal<T>` vs `Bindable<T>` is descriptor metadata only. Runtime cadence must be `ValueSource`-kind-driven, and doc wording should use the actual `RuntimeMutability` vocabulary: `compileTime`, `phaseStart`, `resetOnly`, `runtime`.

6. **Outputs.** Accepted. `PrimitiveOutputs = NoOutputs` should be the only supported Phase 1/2 path. Output modeling and derives should wait until effect-output capture exists in the runtime substrate.

7. **Commonality extraction.** Accepted. Add Phase 0.5 before bulk Phase 3 work to extract repeated helpers such as apply-to channel routing, color blending, phase/time normalization, coordinate normalization, mask visibility helpers, sampler displacement helpers, enum normalization, and unsupported-input diagnostics.

### Accepted with amendment

1. **Phase 1 pack strategy.** The review warned against overwriting the full `primitive.json` with a one-effect pack. The author accepts the risk and prefers a concrete carry-forward strategy:

   - add `descriptors/v3.1/packs/primitive.bootstrap.json` containing unported descriptors;
   - `EffectRegistry` accepts both Rust-derived descriptor entries and bootstrap-carryforward entries;
   - codegen emits one unified `descriptors/v3.1/packs/primitive.json` for external consumers;
   - the round-trip lock compares the unified output;
   - every Phase 3 port moves one entry from bootstrap JSON into Rust;
   - completion is when the bootstrap file is empty.

   This preserves a single external artifact while making the burndown explicit.

2. **Legacy import policy.** The author accepts the need for sharper wording and proposes a tiered table:

   | Dependency | Policy for new primitive ports |
   | --- | --- |
   | `tui-vfx-compositor` | Permanently forbidden under `crates/tui-vfx-compost/src/primitives/`; legacy reference only. |
   | `tui-vfx-style`, `tui-vfx-content`, `tui-vfx-shadow` | Forbidden going forward, with existing compost usage grandfathered temporarily. The known `shader.linearGradient` style dependency needs a Phase 0 cleanup ticket. |
   | `tui-vfx-types`, `tui-vfx-contract` | Allowed as low-level shared types and v3.1 contract surface. |

### Clarified / pushed back

1. **First three primitive ports.** The review recommended exercising more than filter-shaped code before deriving macros. The author agrees with the intent but proposes specific ports:

   1. `filter.dim` — smallest cell filter.
   2. `mask.dissolve` — smallest mask, exercises visibility-returning runtime.
   3. `sampler.gravity` — smallest sampler, exercises source-coordinate/displacement runtime.

   This validates three runtime trait shapes before Phase 2 macro work.

2. **Blocker classification.** The author accepts the reviewer’s expanded Phase 0 gate:

   Must decide before primitive runtime ports:

   - `CellView<'_, P>` enforcement strategy.
   - `Bindable<T>` resolution/scheduling policy.
   - v3.1 scalar/wrapper type home.
   - runtime domain trait split / dispatch ABI.
   - legacy dependency policy.

   Must decide before descriptor/codegen lock:

   - `xtask descriptors gen/check` command shape.
   - partial/full/bootstrap pack strategy.
   - registry representation for source descriptors.
   - SSOT taxonomy wording.

   Can wait until Phase 2+:

   - `PrimitiveInputs` derive timing.
   - `PrimitiveEnum` details.
   - `PrimitiveOutputs` modeling.
   - v3.1-only effect triage dispositions.
   - final workbench doc coexistence/supersession wording.

### Proposed follow-up doc revisions

1. **`docs/arch/v31-primitive-rust-ssot.md` → bump to `0.3.0`:**
   - rewrite Status/Goal/SSOT sections with two-layer taxonomy;
   - replace single `EffectRuntime` with domain-specific runtime traits and registry dispatch maps;
   - add scheduling policy mapping `ValueSource` kinds to resolution cadence;
   - replace blanket legacy-import prohibition with the tiered dependency-policy table;
   - mark `PrimitiveOutputs` as deferred;
   - add bootstrap-carryforward strategy to codegen/round-trip-lock section.

2. **`docs/arch/v31-primitive-rust-ssot-implementation-plan.md` → bump to `0.2.0`:**
   - add Phase 0 decisions for runtime domain split and legacy dependency policy;
   - add Phase 0.5 commonality extraction;
   - rework Phase 1 deliverables around bootstrap-carryforward codegen;
   - port `filter.dim`, `mask.dissolve`, and `sampler.gravity` before Phase 2;
   - add Phase 1 pre-merge checklist for codegen-lock decisions;
   - specify `cargo xtask descriptors gen` / `--check` and decide whether schema generation also moves under `xtask`.

3. **This memo remains the review artifact of record.** This section records author disposition; the architecture and implementation-plan docs should carry the final design, not duplicate the full debate.

## Bottom-line recommendation

I would amend the implementation plan before starting code:

1. Add a Phase 0 decision for **domain-specific runtime traits**.
2. Add a Phase 0/1 decision for **generator command ownership**.
3. Add a Phase 1 safety rule: **do not shrink checked-in `primitive.json` to one effect**.
4. Reframe SSOT as: **contract schemas are generated from `tui-vfx-contract`; primitive descriptor instances are generated from accepted Rust primitive declarations**.
5. Keep `primitive-workbench-schema-driven-workflow.md` as the workflow/scaffolding/validation layer rather than superseding it entirely.

<!-- <FILE>docs/arch/v31-primitive-rust-ssot-peer-review-memo.md</FILE> - <DESC>Peer-review memo on the v3.1 primitive Rust-SSOT plan, current compost substrate, legacy compositor reference, schema generation, and remaining blockers</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.1</VERS> -->
