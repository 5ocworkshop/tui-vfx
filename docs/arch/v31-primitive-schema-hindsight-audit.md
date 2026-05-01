<!-- <FILE>docs/arch/v31-primitive-schema-hindsight-audit.md</FILE> - <DESC>Bounded hindsight audit of v3.1 primitive descriptor fields before compositor-next workbench generation</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Compositor-next Phase 3.5: classify repeated descriptor/schema concepts before broad primitive scaffolding.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add bounded primitive-field commonality audit for compositor-next and Primitive Workbench startup.</CLOG> -->

# v3.1 Primitive Schema Hindsight Audit

## Purpose

This audit is the bounded pre-generation pass required by the compositor-next vertical plan. It looks for repeated primitive descriptor concepts before Primitive Workbench scaffolding turns accidental duplication into generated code.

This is not a broad v3.1 redesign. It classifies common fields, records accepted common contracts, and names owner-decision items. Unrelated vertical primitive slices may proceed when their local field contracts are clear.

Input evidence:

- Descriptor pack: `descriptors/v3.1/packs/primitive.json`
- Generated report: `.omx/reports/v31-primitive-field-commonality-report.json`
- Human summary: `.omx/reports/v31-primitive-field-commonality-report.md`
- Architecture constraints: `docs/arch/compositor-next-vertical-implementation-plan.md`, `docs/arch/v31-schema-boundary-north-star.md`, and `docs/arch/primitive-workbench-schema-driven-workflow.md`

## Corpus snapshot

The current primitive descriptor pack contains 127 descriptors and 642 input fields. The most repeated field names are `color` (32), `applyTo` (27), `speed` (24), `seed` (23), `intensity` (15), `direction` (14), `progress` (12), `width` (10), `softEdge` (9), `amplitude` (8), `frequency` (6), `axis` (6), and `mode` (6).

The sparse explicit `semantic` annotations are themselves an audit finding: repeated field names often have `semantic: null`, while only a small set of semantics repeat (`foreground-color`, `background-color`, `blend-ratio`, `edge-position`, `random-seed`). Primitive Workbench should not infer meaning from names alone without a descriptor-side semantic or a family-local generated helper decision.

## Classification table

| Concept | Classification | Decision | Rationale |
| --- | --- | --- | --- |
| `color` / accent colors | `same-name-same-semantics` with local exceptions | `candidate-common-contract` | Repeats across 32 descriptors as color input. Workbench can generate common color extraction, default handling, bindability checks, and diagnostics. Local docs must still distinguish accent/ink/fill roles. |
| `foreground` / `background` | `different-name-same-semantics` | `candidate-common-contract` | Existing explicit semantics identify `foreground-color` and `background-color`; source and style primitives should share color-channel accessors. |
| `applyTo` | mixed `same-name-same-semantics` and `same-name-different-semantics` | `owner-decision-needed` | Repeats 27 times but kind varies (`enum`/`string`) and semantics vary (`channel-target`, `style-channel`, or null). Workbench may generate helpers only after a descriptor fragment or family-specific projection names allowed targets. |
| `progress` / `position` | `different-name-same-semantics` plus legacy ambiguity | `candidate-common-contract` for bindable progress; `legacy-alias-only` for incompatible names | `progress` appears directly while `position` sometimes carries semantic `progress` and sometimes edge position. Workbench must not blindly alias `position` to progress. |
| `direction`, `axis`, `origin`, `position` | `family-specific-semantic` | `candidate-common-contract` for geometry vocabulary with per-family enums | These are repeated geometry controls, but allowed values differ by primitive family. Generate enum extraction and diagnostics from descriptor values rather than one global enum. |
| `speed`, `frequency`, `amplitude`, `wavelength` | `same-name-same-semantics` within motion families | `candidate-common-contract` | Common numeric motion controls. Generated accessors should preserve units/ranges and bindability. Do not collapse into presentation `fps`. |
| `seed` | `same-name-same-semantics` | `candidate-common-contract` | Repeats 23 times, sometimes with explicit `random-seed`/`seed` semantics. Generate seeded-random extraction helper and deterministic test fixture hooks. |
| `density`, `threshold`, `intensity`, `strength` | mixed | `owner-decision-needed` | Names repeat, but semantics differ (opacity, displacement, desaturation, blend ratio). Require per-field semantic before common generated helpers beyond raw number extraction. |
| `radius`, `falloff`, `feather`, `softEdge` | `different-name-same-semantics` family cluster | `candidate-common-contract` | Common edge/gradient softness controls. Generate family-local helpers after first three real usages prove the exact distance model. |
| `width`, `height`, `bandWidth`, `edgeWidth`, `thickness` | mixed | `keep-distinct` initially | These may refer to source dimensions, band widths, edge widths, or stroke thickness. Same numeric kind does not imply shared semantics. |
| `glyph`, `charset`, `pattern`, `fillChar` | `family-specific-semantic` | `candidate-common-contract` for glyph/charset extraction | Repeated glyph selection should share validation and Unicode diagnostics, but visual semantics remain primitive-owned. |
| Legacy aliases and V2 naming | `legacy-alias-only` | migration-only | Migration mappings may accept legacy vocabulary, but canonical v3.1 descriptor fields should not grow permissive aliases merely to validate old JSON. |

## Timing and cadence findings

Do not collapse timing into `fps`.

Required boundary concepts remain distinct:

1. **Presentation cadence** — how often a player/backend attempts to draw frames.
2. **Semantic update cadence** — optional fixed-step recomputation cadence for primitives/sources that need deterministic state updates.
3. **Sample time** — the actual `phaseT`, optional `loopT`, and absolute elapsed/sample milliseconds used for deterministic sampling.

The Madeira flag procedural source remains the reference case: flag wave/fireworks motion needs absolute elapsed time even when normalized `phaseT`/`loopT` are fixed. Preview loopback ramps also use elapsed time to honor authored duration. Primitive Workbench generated inputs should expose sample-time requirements explicitly when a primitive depends on elapsed time.

Accepted common timing concepts:

- `sampleTime` / runtime sample context as a generated accessor contract.
- `speed` / `frequency` as primitive numeric inputs, not presentation cadence.
- Optional future `updateCadence` only for primitives/sources with fixed-step semantics.

Rejected collapse:

- `fps` as a primitive semantic field. Presentation cadence is a player/backend control unless a future descriptor explicitly accepts a presentation hint.

## First vertical slice: `shader.highlighter`

The first existing shader candidate remains `shader.highlighter`.

Current descriptor inputs from `descriptors/v3.1/packs/primitive.json`:

| Field | Current kind | Audit classification |
| --- | --- | --- |
| `color` | `color` | candidate common color input |
| `bandWidth` | `number` | primitive-local width/band control |
| `blendStrength` | `number` | candidate blend-ratio helper only after semantic annotation |
| `textContrast` | `number` | owner-decision-needed; runtime model is enum-like, descriptor kind should be reviewed before generated scaffold hardens it |
| `mode` | `enum` | primitive-local enum |
| `softEdge` | `boolean` | candidate softness/edge helper |
| `direction` | `enum` | family geometry enum |
| `rowMask` | `integer` | owner-decision-needed; runtime model is structured/enum-like, descriptor kind should be reviewed before scaffold hardens it |
| `applyTo` | `enum` | channel/style target helper after allowed values are read from descriptor |

Highlighter pre-slice owner-decision items:

1. Confirm whether `textContrast` should remain a `number` descriptor input or become an enum/structured descriptor matching runtime `TextContrast`.
2. Confirm whether `rowMask` should remain `integer` or become an enum/structured descriptor matching runtime `HighlighterRowMask`.
3. Attach semantics for `blendStrength`, `applyTo`, and `color` if generated workbench helpers are expected to classify them globally.

These decisions should be made before generating checked-in highlighter scaffold. They do not block the baseline compositor-next copy or old-vs-next parity proof.

## Workbench inputs accepted now

Primitive Workbench MVP may safely generate these helper families for the first slice:

- typed color extraction and color diagnostics;
- numeric extraction with descriptor range/default/bindability checks;
- enum extraction from descriptor `allowedValues`;
- boolean extraction;
- unsupported-field diagnostics keyed to descriptor input set;
- field coverage manifest that reports authored fields missing from runtime handling;
- sample-time accessor contract that distinguishes `phaseT`, `loopT`, and absolute elapsed milliseconds.

Primitive Workbench must not yet generate a global `applyTo` helper or global geometry enum helper unless it is parameterized by descriptor allowed values and family-local semantics.

## Rejected collapses

- Do not merge `intensity`, `strength`, and `blendStrength` into one global concept. They overlap as numbers but represent opacity, displacement, desaturation, and blend ratio in different contexts.
- Do not merge `width`, `height`, `bandWidth`, `edgeWidth`, and `thickness` into one global size field.
- Do not treat `position` as progress without checking semantic annotation; it also means edge position.
- Do not add compatibility aliases to canonical v3.1 descriptors solely to make V2 JSON validate.
- Do not model `source.indexedField` as an effect descriptor; it is reserved as a source descriptor after existing primitive migration workflow is proven.

## Owner-decision-needed queue

| Item | Why it needs owner/design decision | Unblocks |
| --- | --- | --- |
| Highlighter `textContrast` descriptor kind | Current descriptor says `number`; runtime shape is enum/structured. Generated scaffold would harden the mismatch. | `shader.highlighter` generated input model |
| Highlighter `rowMask` descriptor kind | Current descriptor says `integer`; runtime shape is enum/structured. Generated scaffold would harden the mismatch. | `shader.highlighter` field coverage and unsupported diagnostics |
| Global `applyTo` semantics | Repeated field with family-specific target sets. | shared workbench channel-routing helper |
| Timing `updateCadence` descriptor concept | Needed only if fixed-step semantics appear; should not be invented for every primitive. | future procedural/time-sensitive primitive work |

## Acceptance outcome

This audit satisfies the compositor-next Phase 3.5 gate for baseline and first-slice planning:

- Common fields were classified before broad scaffold generation.
- Accepted common concepts are documented.
- Rejected collapses include rationale.
- Owner-decision items are explicit and scoped.
- No compatibility aliases were introduced.

<!-- <FILE>docs/arch/v31-primitive-schema-hindsight-audit.md</FILE> - <DESC>Bounded hindsight audit of v3.1 primitive descriptor fields before compositor-next workbench generation</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
