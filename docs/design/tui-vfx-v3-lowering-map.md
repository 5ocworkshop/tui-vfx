<!-- <FILE>docs/design/tui-vfx-v3-lowering-map.md</FILE> - <DESC>Live lowering map from important V2 surfaces/families into the V3 schema and capability catalog. This is the execution companion for Chapter 57.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Seed the lowering phase with the highest-value mappings so implementation can start from explicit migration contracts rather than ad hoc policy.</WCTX> -->
<!-- <CLOG>0.1.0: initial lowering map. Covers envelope/time/style/interaction/spatial-shader/apply_to/content-tree normalizations and marks the highest-risk family classes as human-review-required.</CLOG> -->

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
| LM-12 | edge cases requiring human review | OPEN | living list |

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

## 3. Human-review-required cases

These should not be blindly auto-lowered:

- wrapper/router families
- hybrid templates
- same-name cross-lane families
- field-rendering wrappers
- rule-engine families with large rule tables
- any future celebratory particle family

---

## 4. Next execution target

Next lowering work should focus on:

- populating LM-12 with concrete remaining edge cases as implementation begins
- validating that lowering rules are sufficient for normalized IR construction
- checking whether any family still forces schema changes rather than simple lowering/normalization

<!-- <FILE>docs/design/tui-vfx-v3-lowering-map.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
