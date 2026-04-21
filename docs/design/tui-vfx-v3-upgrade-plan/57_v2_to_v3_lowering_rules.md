<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/57_v2_to_v3_lowering_rules.md</FILE> - <DESC>Chapter 57 — canonical lowering rules from V2 to V3. Defines how existing V2 surfaces map into the stabilized V3 tree, what normal forms are preferred, and which cases require human classification rather than blind automatic migration.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>This chapter follows the schema-hardening and capability-catalog phases. It turns migration behavior into an explicit contract so loaders, scripts, validators, and human re-authoring passes do not silently embed different policy.</WCTX> -->
<!-- <CLOG>1.0.0: initial chapter. Defines the lowering contract categories, the default migration posture, canonical mappings for the highest-value V2 surfaces, and the distinction between automatic lowering, automatic normalization, and human-classification-required cases.</CLOG> -->

# 57 — V2 → V3 Lowering Rules

This chapter answers a specific implementation question:

> Given the now-stabilized V3 structure, how should existing V2 surfaces be lowered into that structure without every tool, loader, or human migration pass inventing its own policy?

The goal is to make lowering behavior a **documented contract** rather than an accidental byproduct of implementation order.

---

## 10 — Lowering categories

Every V2→V3 mapping should fall into one of four categories.

### 1. Direct structural carry-forward
A V2 field already has a straightforward V3 home.

### 2. Canonical normalization
A V2 concept survives, but its representation is normalized in V3.

### 3. Structural lift
A V2 concept moves out of a family-specific payload and into a shared structural home in V3.

### 4. Human classification required
The V2 concept cannot be lowered safely without deciding whether it is:
- primitive
- composed primitive
- wrapper
- hybrid template
- policy variant

---

## 20 — Default migration posture

When in doubt, use these rules:

1. **Lift cross-cutting semantics upward.**
   - if the concept is really scope / timing / interaction / placement, move it into the V3 structural home
2. **Normalize repeated legacy shapes to one V3 normal form.**
   - do not preserve V2's parallel shape diversity when V3 already picked one preferred form
3. **Do not auto-promote family-specific payload details into new schema surface.**
   - if something is family-internal unless proven otherwise, keep it family-internal
4. **Require human classification before flattening deep families.**
   - especially for split-flap, typewriter+cursor, wrappers, hybrid templates, and rule-engine families

---

## 30 — High-value canonical lowering rules

### 30.1 Recipe envelope

| V2 surface | V3 home | Lowering rule |
|---|---|---|
| `theme` | top-level `theme` | carry forward directly |
| `message` | `config.message` | carry forward directly |
| `layout` | `config.layout` | carry forward directly |
| `lifecycle.auto_dismiss_ms` | `config.lifecycle.auto_dismiss_ms` | carry forward directly |
| `border` | `config.border` | carry forward directly |
| `shadow` | `config.shadow` | carry forward directly |
| `scene` | `config.scene` | carry forward directly, then normalize per-layer fields |
| `requires_primitives` | top-level `requires_primitives` | carry forward as contract/discovery hint |
| `time` | `config.clock` | normalize name and shape |

### 30.2 Pools / presets / asset pools

| V2 surface | V3 home | Lowering rule |
|---|---|---|
| `text_pool` | authoring-scale layer above concrete tree | do **not** lower into concrete tree |
| `effect_pool` | authoring-scale layer above concrete tree | do **not** lower into concrete tree |
| `preset_pool` | authoring-scale layer above concrete tree | do **not** lower into concrete tree |
| `image_pool` / `font_pool` | authoring-scale layer above concrete tree | do **not** lower into concrete tree |

Rule:
- these are family/template/content-source authoring mechanisms, not concrete execution-tree nodes
- lowering should emit concrete realized recipes after pool/preset selection, not force pools into the runtime tree

### 30.3 Scope / targeting

| V2 surface | V3 home | Lowering rule |
|---|---|---|
| `apply_to: foreground/background` | `scope.kind = channel` | structural lift |
| `apply_to: both` | `scope.kind = all` by default | structural lift |
| `BorderOnly` | `scope.kind = border` | direct mapping |
| `Rows` / `RowRange` | `scope.kind = rows/row_range` | direct mapping |
| `Columns` / `ColumnRange` | `scope.kind = columns/column_range` | direct mapping |
| `Cell` / `Cells` | `scope.kind = cell/cells` | direct mapping |
| large explicit `Cells` runs | `cell_run` / `cell_runs` / `region_ref` when possible | normalization/compression |

Rule:
- channel targeting belongs in scope unless a family truly has a second independent render-channel concept
- region compression helpers are preferred when the authored concept is contiguous or reusable

### 30.4 Style normalization

| V2 surface | V3 home | Lowering rule |
|---|---|---|
| singular `style` | tree-normal form | normalize |
| plural `styles[]` | tree-normal form | normalize |
| `base_style` on a scoped style lane | `style_effect(type = base_style_override, style = ...)` | normalize |
| `enter_effect` / `dwell_effect` / `exit_effect` | `style_effect` leaves with explicit phase | normalize |
| style-layer `clock` | per-step `clock` override | structural lift |
| `spatial_shader` | `style_effect(type = spatial, shader = ...)` **or** sibling `shader` step | human-guided but rule-bound |

Rule:
- the normal form is a tree of `style_effect`, optional `base_style_override`, and optional sibling `shader` steps
- singular/plural V2 style shapes must not survive as distinct V3 execution shapes

### 30.5 Interaction

| V2 surface | V3 home | Lowering rule |
|---|---|---|
| `interaction_states` + `interaction_config` | Step-level `interaction` | structural lift |

Rule:
- interaction stays attached to the relevant step/lane, not scattered across the recipe envelope

### 30.6 Motion / timing

| V2 surface | V3 home | Lowering rule |
|---|---|---|
| `pipeline.<phase>.duration_ms` | `pipeline.timing.enter_ms/exit_ms` | normalize |
| `pipeline.<phase>.easing` | `pipeline.timing.enter_ease/exit_ease` | normalize |
| `motion_path` | `pipeline.timing.enter_path/exit_path` | structural lift |
| `from` / `to` offscreen | `pipeline.timing.enter_from/exit_to` | structural lift |
| `snapping` | `pipeline.timing.enter_snap/exit_snap` | structural lift |
| style/content loop clocks | `config.clock` or per-step `clock` | normalize by scope |
| legacy `continuous` | `phase = all` + clocked step/renderer | normalize, not preserve as separate mode |

Rule:
- motion/timing metadata stays on the recipe/step envelope, not inside arbitrary payloads unless the family's actual behavior is inherently family-local

### 30.7 Content renderer trees

| V2 surface | V3 home | Lowering rule |
|---|---|---|
| `content.effect.typewriter*` variants | `content.effect = typewriter_renderer` with nested cursor policy | human classification + canonical subtree |
| `content.effect.split_flap*` variants | `content.effect = split_flap_renderer` with policy bundles | human classification + canonical subtree |
| simple content transforms (`mirror`, `numeric`, `redact`) | direct carry-forward inside `content.effect` | direct |
| hybrid content recipes with shell polish | keep behavior in `content.effect`, move shell polish to pipeline | normalize |

Rule:
- do not explode nested cursor or split-flap policy variants into fake top-level independent families

### 30.8 Filter-family lowering rules

| V2 surface | V3 home | Lowering rule |
|---|---|---|
| progress/indicator families | one `progress_emphasis` subtree | human classification + canonical subtree |
| sweep families | one `traveling_band` subtree with lane-specific wrappers | human classification |
| pattern/procedural texture families | split into `pattern_treatment` and `procedural_texture` siblings | human classification |
| field-rendering wrappers | wrapper category over upstream source fields | human classification |
| vignette variants | one `falloff_treatment` subtree with policy axes | human classification |
| `glyph_style` | rule-engine family on existing `filter` lane | governance category |

### 30.9 Scene-layer lowering rules

| V2 / example surface | V3 home | Lowering rule |
|---|---|---|
| layer-local source | `scene.layers[*].source` | direct |
| sibling-relative placement | `scene.layers[*].placement` | direct |
| layer-local styling | `scene.layers[*].surface` | direct |
| layer-local operations | `scene.layers[*].pipeline` | direct |
| procedural generator payloads | `source_id` + `schema_ref` + `params` | normalized source contract |

Rule:
- scene layers are not a fallback hack; they are a first-class renderer-tree surface

---

## 40 — Cases that require human classification

These are the classes that should **not** be blindly auto-lowered:

1. families that may collapse into deeper subtrees
2. wrapper/router families
3. hybrid transition templates
4. split-flap subtree variants
5. typewriter+cursor subtree variants
6. rule-engine payloads
7. field-rendering wrappers
8. same-name families across lanes where semantics may diverge

If a migration tool cannot determine these safely, it should emit a review-needed artifact rather than guess.

---

## 50 — Lowering-map execution companion

The live execution companion for this phase is:

- `docs/design/tui-vfx-v3-lowering-map.md`

That file records concrete family-by-family lowering decisions as they are executed.

---

## 60 — Definition of done for this phase

This phase is done only when:

- the high-value V2 surfaces have explicit lowering rules
- the major ambiguous family classes have a documented migration stance
- normalization rules are stable enough for tooling implementation
- the lowering map covers the currently supported corpus
- the validator/canonicalizer can be implemented from these docs without guessing at policy

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/57_v2_to_v3_lowering_rules.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
