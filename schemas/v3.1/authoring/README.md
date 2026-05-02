<!-- COMMENT <FILE>schemas/v3.1/authoring/README.md</FILE> - <DESC>Orientation for the V3.1 authoring shorthand alias structure</DESC> -->
<!-- COMMENT <VERS>VERSION: 0.2.0</VERS> -->
<!-- COMMENT <WCTX>Alias structure synced with the contract gaps closed in commit f7e9d5b</WCTX> -->
<!-- COMMENT <CLOG>Unblocked rows/columns scope entries, added nodeFieldLifts (writeChannels) and shadowFieldRenames sections, refreshed gap-status section</CLOG> -->

# V3.1 authoring shorthand — alias structure

This directory holds the data tables the canonicalize function consults when
turning author-friendly shorthand JSON into the strict V3.1 `RecipeDocument`.
It is a **data-only** layer — no Rust code lives here. The canonicalize
function lives in `crates/tui-vfx-contract/src/canonicalize/` and reads these
tables.

## Pipeline

```
shorthand JSON  ──►  [canonicalize]  ──►  RecipeDocument  ──►  [LoadedRecipe::load]  ──►  runtime
                          │
                          ├── reads common/canonicalization-rules.json
                          ├── reads <axis>/aliases.json     (per axis)
                          └── reads <axis>/expansion.json   (per axis, where presets live)
```

The canonicalize function is the only consumer of these tables at runtime.
Docs generators (`cargo xtask docs generate`) read them to produce the
authoring reference.

## Layout

```
schemas/v3.1/authoring/
├── README.md                    (this file)
├── corpus/                      180-recipe survey + 47 paired files + DECISIONS.md
├── meta/                        JSON Schema meta-schemas — describe the data shape
│   ├── alias-table.schema.json
│   ├── expansion-table.schema.json
│   └── canonicalization-rules.schema.json
├── common/                      universal lookups (one file, all axes)
│   └── canonicalization-rules.json
├── transition/
│   ├── aliases.json
│   └── expansion.json           presets — the high-traffic table for transitions
├── filter/
│   └── aliases.json
├── shader/
│   └── aliases.json
├── sampler/
│   └── aliases.json
├── style/
│   └── aliases.json
└── mask/
    └── aliases.json             mask-as-effect form (Q3); mask-as-transition lives in transition/expansion.json
```

## Three table types

### 1. Common canonicalization rules (`common/canonicalization-rules.json`)

Universal transformations that apply across every axis:

- **Named colors.** 17 names per Q6 — ANSI-16 plus light-variants, `light_gray`, `dark_gray`, `transparent`, `reset`.
- **Phase aliases.** `"all"` expands to `["enter", "dwell", "exit"]`; bare phase strings lift to single-element arrays.
- **Scope shape map.** Author-side scope shorthands (`{ rect: [...] }`, `{ rowRange: [...] }`, `{ rows: [...] }`, etc.) mapped to canonical `ScopeSpec` discriminators.
- **Node field lifts.** Structural rules that move information out of an author-facing position into a sibling `NodeSpec` field. The big one: `{ scope: { channel: "foreground" } }` lifts to `NodeSpec.writeChannels: ["foreground"]` (plural array, sibling of `scope`). Legacy V2/V3 `applyTo` follows the same lift.
- **Shadow field renames.** Shadow-block author-side names mapped to canonical `ShadowSpec` field names. Currently: `edgeCrossing` → `edgeCrossingPolicy`.
- **Value envelope catalog.** Author-side scalar shapes mapped to `{ kind, value }` envelopes (integer, number, boolean, text, color, duration, enum).
- **Duration string forms.** `"300ms"`, `"1.5s"`, `"2m"` recognized.
- **Sigils.** `$bind:<id>[?<fallback>]` and `$asset:<id>` resolution rules.

### 2. Per-axis alias tables (`<axis>/aliases.json`)

Validated against `meta/alias-table.schema.json`. Each entry maps an
author-side spelling to a canonical effect identifier plus parameter envelope
hints. Params not listed in `paramMapping` pass through unchanged with
type-inferred envelopes.

```json
{
  "from": "dim",
  "canonicalEffect": "filter.dim",
  "paramMapping": {
    "factor":  { "to": "factor",  "envelope": "literal-number", "default": 0.5 },
    "phase":   { "to": "activePhases", "envelope": "phases-list" },
    "scope":   { "to": "scope",        "envelope": "scope-spec" }
  }
}
```

### 3. Per-axis expansion tables (`<axis>/expansion.json`)

Validated against `meta/expansion-table.schema.json`. Each entry maps a
preset name (e.g., `"iris"`, `"crossfade"`, `"push"`) to the canonical
tracks (kind=`transition`) or graph nodes (kind=`effect-stack`) it expands
into. Preset names become `TransitionIntent::Preset { preset }` provenance
in the canonical output for kind=transition.

```json
{
  "preset": "iris",
  "kind": "transition",
  "params": [
    { "name": "shape",    "type": "enum", "values": ["circle", "diamond", "square"], "default": "circle" },
    { "name": "softEdge", "type": "boolean", "default": false },
    { "name": "duration", "type": "duration", "default": "500ms" }
  ],
  "tracks": [{ "kind": "visibility.iris" }]
}
```

## Versionless extension

Adding entries to any of these tables is a **versionless change**. The V3.1
canonical schema (`schemas/v3.1/recipe.schema.json`) does not reference
these tables; only the canonicalize function does. Constraints:

| Change | V3.1 schema bump? | Contract change? |
|---|---|---|
| Add a new spelling in `<axis>/aliases.json` | No | No |
| Add a new preset in `<axis>/expansion.json` (composing existing primitives) | No | No |
| Add a new named color to `common/canonicalization-rules.json` | No | No |
| Loosen the alias-table format (new fields on entries) | Bump alias-table meta-schema | No |
| Add a new canonical primitive (new shader/filter descriptor) | Yes | Yes — descriptor pack + maybe contract |
| Add a new canonical track kind (`relation.*`) | Yes | Yes |

The five schema-faith presets (`crossfade`, `push`, `morph`, `stippled`,
`braille`) are the latter case — they reference canonical track kinds that
do not yet exist in `tui-vfx-contract`. The expansion entries are present so
the docs generator can render them and corpus pairs can validate as-soon-as
the canonical primitives land. The canonicalizer should refuse them with a
"pending contract primitive" message until then.

## Schema-gap status — closed

All three gaps from the [schema gaps memo](corpus/SCHEMA_GAPS_MEMO.md) landed
in commit `f7e9d5b`:

- **Gap 1 — Channel scoping.** Resolved via Option B with a wrinkle: plural
  `NodeSpec.writeChannels: CellChannel[]` rather than singular `Option<T>`.
  Empty array means "no further restriction beyond the descriptor's declared
  writable channels"; non-empty must be a subset of the descriptor's
  `cellAccess.writes`. Author-side `{ scope: { channel: "foreground" } }`
  lifts *out of* scope into the sibling `writeChannels` field — see
  `nodeFieldLifts` in `common/canonicalization-rules.json`.
- **Gap 2 — Row/column index sets.** `ScopeSpec::Rows { indices: [...] }`
  and `ScopeSpec::Columns { indices: [...] }` exist; both have non-empty +
  uniqueItems schema constraints. `scopeShapeMap` rows/columns entries are
  unblocked.
- **Gap 3 — Shadow edge-crossing policy.** `ShadowSpec.edgeCrossingPolicy:
  "default" | "fade" | "preserve"` is optional. Author-side
  `shadow.edgeCrossing` canonicalizes to `shadow.edgeCrossingPolicy`; values
  pass through unchanged. See `shadowFieldRenames`.

`CellChannel` enum values (camelCase, from `cls_cell_channel.rs`):
`glyph | foreground | background | modifiers | modifierAlpha | role`. Author
shorthand accepts the same casing.

Corpus regression coverage for the channel-scoping fix is the gate the
implementer named: shorthand → canonicalize → load → render with
`writeChannels: ["foreground"]` must leave the background channel
unchanged from the recipe-base color, and vice versa. The five recipes
named in the [response memo](corpus/SCHEMA_GAPS_MEMO_RESPONSE.md)
(`scene_layer_full_stack`, `complex_diamond_highlight`,
`B10_grimoire_incantation_bar_palette`, `L06_hygge_lantern_diffusion`,
`M11_blueprint_circuit_pulse_info`) are the canonical fixtures for that
gate.

## Format

JSON. Hand-written. Validated against the meta-schemas in `meta/`. TOML was
considered and rejected (M4 / DECISIONS): the recursive shape of expansion
entries is a poor fit for TOML's flat tables, and JSON Schema infrastructure
already exists for canonical recipe validation.

## What is NOT here

- **Recipes.** Authoring recipes live wherever they live in the consumer
  project (`/usr/projects/gt-design/recipes/`, the tui-vfx debug recipes,
  etc.). This directory is the alias data the canonicalizer reads.
- **Canonical schemas.** Those live at `schemas/v3.1/recipe.schema.json`
  and the per-type schemas alongside.
- **Code.** The canonicalize function lives in
  `crates/tui-vfx-contract/src/canonicalize/`.
- **Templates.** `extends:` template files are co-located with the recipe
  family they serve (Q11), e.g., `recipes/wargames/themes/wopr_green.json`.

## Status

- Meta-schemas drafted (3 files).
- Common canonicalization rules drafted (1 file). All three contract gaps
  closed; `nodeFieldLifts` and `shadowFieldRenames` sections document the
  channel-scoping and shadow-edge-crossing rules.
- Seed alias tables drafted for transition / filter / shader / sampler /
  style / mask. Coverage is representative, not exhaustive — the canonicalize
  function's first round-trip suite drives which entries grow next.
- Five schema-faith transition presets present in `transition/expansion.json`
  pending their canonical track primitives (relation.crossfade, relation.push,
  relation.morph, visibility.stippled, visibility.braille).

### Corpus paired-file caveat

The `corpus/canonical/*.json` files were authored before the Gap 1 fix, so
they still show channel scoping as `scope: { kind: "channel", value: "foreground" }`
(the rejected Option A shape). Once the canonicalize function lands and the
round-trip suite runs against the corpus, those canonical sides need
regeneration to use the actual landed shape — `scope: { kind: "all" }` plus
`writeChannels: ["foreground"]` as a NodeSpec sibling. This is paperwork at
regeneration time, not a structural rework. The shorthand sides of the
pairs are unaffected.

<!-- COMMENT <FILE>schemas/v3.1/authoring/README.md</FILE> - <DESC>Orientation for the V3.1 authoring shorthand alias structure</DESC> -->
<!-- COMMENT <VERS>END OF VERSION: 0.2.0</VERS> -->
