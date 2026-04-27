<!-- <FILE>docs/design/tui-vfx-v3-lowering-map.md</FILE> - <DESC>Live lowering map from important V2 surfaces/families into the V3 schema and capability catalog. This is the execution companion for Chapter 57.</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Extended through the first ambiguity-resolution batch so the lowering map now covers not only the straightforward mappings but also the classes that originally required explicit human review.</WCTX> -->
<!-- <CLOG>0.2.0: resolve LM-12 human-review-required cases by writing explicit lowering rules for wrappers, hybrid templates, same-name cross-lane families, rule-engine families, field-rendering wrappers, and celebratory particle generators.
0.1.0: initial lowering map. Covers envelope/time/style/interaction/spatial-shader/apply_to/content-tree normalizations and marks the highest-risk family classes as human-review-required.</CLOG> -->

# tui-vfx V3 lowering map

This file is the execution artifact for the lowering phase.

It records the current lowering contract from important V2 surfaces into the V3 schema and capability catalog.

---

## 1. Status tracker

| ID | Lowering area | Status | Notes |
|---|---|---|---|
| LM-01 | Envelope + top-level config fields | RESOLVED | direct homes chosen |
| LM-02 | `time` / clocks / continuous | RESOLVED | normalized home chosen |
| LM-03 | `apply_to` + channel targeting | RESOLVED | structural lift to scope |
| LM-04 | `style` / `styles[]` normalization | RESOLVED | one tree normal form |
| LM-05 | `spatial_shader` migration rule | RESOLVED | wrapper vs sibling shader rule |
| LM-06 | interaction metadata | RESOLVED | step-level home chosen |
| LM-07 | content pools / presets / asset pools | RESOLVED | above-the-tree authoring layer |
| LM-08 | typewriter + cursor variants | RESOLVED | lower into one renderer tree |
| LM-09 | split-flap variants | RESOLVED | lower into one renderer tree |
| LM-10 | filter-side subtree families | RESOLVED | lower against capability catalog |
| LM-11 | scene layers + procedural sources | RESOLVED | direct layer homes + schema_ref |
| LM-12 | edge cases requiring human review | RESOLVED | explicit lowering rules added |

---

## 2. Resolved lowering rules

### LM-01 — Envelope + top-level config fields

- `message` → `config.message`
- `layout` → `config.layout`
- `lifecycle` → `config.lifecycle`
- `border` → `config.border`
- `theme` → top-level `theme`
- `shadow` → `config.shadow`
- `requires_primitives` → top-level `requires_primitives`
- `scene` → `config.scene`

### LM-02 — `time` / clocks / continuous

- `time.loop` + `time.loop_period_ms` → `config.clock`
- style-layer `clock` → per-step `clock`
- legacy `continuous` → `phase = all` + clocked step/renderer, not a separate V3 mode

### LM-03 — `apply_to` + channel targeting

- `foreground` / `background` → `scope.kind = channel`
- `both` → `scope.kind = all` by default
- payload-level `apply_to` survives only when the family genuinely needs a second independent channel/render concept

### LM-04 — `style` / `styles[]` normalization

- both singular and plural forms normalize to the same tree idiom:
  - `style_effect`
  - optional `base_style_override`
  - optional sibling `shader`
- per-phase style effects become explicit phase-tagged style leaves

### LM-05 — `spatial_shader` migration rule

Use one of two targets:

1. `style_effect(type = spatial, shader = ...)`
   - when the semantics are style-local
2. sibling `shader` step
   - when the semantics are better understood as a general pipeline operation

### LM-06 — interaction metadata

- `interaction_states` + `interaction_config` → Step-level `interaction`

### LM-07 — content pools / presets / asset pools

Do **not** lower them into the concrete execution tree.

Treat them as:
- template/family/content-source authoring mechanisms above the concrete tree

### LM-08 — typewriter + cursor variants

Lower all of the following into one `typewriter_renderer` subtree with nested cursor policy:

- caret/full cursor variants
- grow-in variants
- wake variants
- scan variants
- braille glyph variants

### LM-09 — split-flap variants

Lower all of the following into one `split_flap_renderer` subtree with policy bundles:

- board/display variants
- charset variants
- source/target variants
- physical rolling variants
- authenticity/timing policy variants

### LM-10 — filter-side subtree families

Lower by capability-catalog family rather than by one-name-per-file legacy identity:

- progress emphasis
- traveling-band / sweep
- pattern treatment
- procedural texture
- motion-treatment
- field-rendering wrappers
- falloff treatment
- rule-engine family

### LM-11 — scene layers + procedural sources

- layers keep their own `source`, `placement`, `surface`, `pipeline`
- procedural sources normalize to `source_id` + `schema_ref` + `params`

---

## 3. Resolved ambiguity-heavy lowering rules (LM-12)

### LM-12.1 — Wrapper / router families

Examples:
- style-native spatial wrappers
- style-side hosts of other capability families

Lowering rule:
- keep them on their native lane
- represent the hosted family inside payload structure
- do **not** lower them into fake new top-level StepKinds

### LM-12.2 — Hybrid templates

Examples:
- wipe + fade transition templates
- materialize variants
- field-rendering wrappers with upstream source shaders

Lowering rule:
- lower to explicit `parallel` / `sequence` tree forms composed of already-known leaf kinds
- do not force them into one flattened primitive payload
- preserve them as reusable composition templates in docs/catalog/governance where useful

### LM-12.3 — Same-name cross-lane families

Examples:
- `crt` filter vs `crt` sampler
- `sub_cell_shake` shader vs `sub_cell_shake` filter

Lowering rule:
- treat same-name cross-lane families as distinct unless implementation/code evidence proves they are the same substrate with wrappers
- name collisions are documentation/governance problems first, not automatic collapse signals

### LM-12.4 — Field-rendering wrappers

Examples:
- `subcell_light_*`

Lowering rule:
- lower as a wrapper category over an upstream field source
- preserve both:
  - the renderer payload
  - and the upstream source-producing sibling subtree
- do not collapse them into standalone simple filter leaves

### LM-12.5 — Rule-engine families

Examples:
- `glyph_style`

Lowering rule:
- keep them on the existing operational lane
- classify them as rule-engine payload families in docs/catalog/governance
- normalize repeated phase payloads where possible, but do not pretend they are ordinary scalar payloads

### LM-12.6 — Celebratory particle generators

Examples:
- Madeira fireworks

Lowering rule:
- do not force a fake lowering onto existing sweep/texture families
- classify as a future procedural generator subtree
- keep omitted from current concrete migration targets unless and until the new generator family exists

---

## 4. Next execution target

Next lowering work should focus on:

- validating that lowering rules are sufficient for normalized IR construction
- identifying any implementation spikes that still force schema revision rather than lowering/normalization
- keeping this map current as real implementation begins

<!-- <FILE>docs/design/tui-vfx-v3-lowering-map.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
