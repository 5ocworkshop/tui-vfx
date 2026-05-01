# M4 — Round 6 (schema-faith demonstrators for the 5 missing transition presets)

After 5 corpus rounds the missing transition presets (`Crossfade`, `Push`,
`Morph`, `Stippled`, `Braille`) have zero corpus witnesses — they are upcoming
primitives, not absences. This addendum drafts their alias + expansion table
entries based on:

- The `TransitionPreset` enum (`cls_transition_preset.rs:20`).
- The canonical track families in `v31-native-transition-model.md` §3 ("Track
  families"): `visibility.*`, `opacity.*`, `motion.*`, `relation.*`,
  `content.*`, `style.*`.
- The descriptive sentences attached to each preset in the contract:

  | Preset | Contract description |
  |---|---|
  | `Crossfade` | "Between-surface blend from prior to next surface." |
  | `Push` | "Coordinated from/to surface displacement." |
  | `Morph` | "Between-surface content correspondence transform." |
  | `Stippled` | "Stipple-pattern visibility reveal." |
  | `Braille` | "Braille-pattern visibility reveal." |

Five paired files written: `transition_{crossfade,push,morph,stippled,braille}.json`
in canonical/ and shorthand/. All five are clearly labeled
`tags: [..., "schema-faith"]` so they don't get mistaken for corpus-derived
evidence.

## Family split

The five missing presets split cleanly along the track-family axis:

### Single-subject (visibility.*) — same shape as iris/dissolve/blinds

| Preset | Track | Subject parameter | Witnesses for shape |
|---|---|---|---|
| `Stippled` | `visibility.stippled` | `subject: "to"` | iris (6), dissolve (6), blinds (5) |
| `Braille` | `visibility.braille` | `subject: "to"` | same family |

Both fit the existing single-subject visibility pattern. Shorthand expansion
is mechanical: same as the other visibility presets, with new parameters
(`pattern`/`density`/`seed` for stippled; `subcellOrder`/`seed` for braille).
**No new schema shape needed.**

### Two-subject (relation.*) — structurally new for the corpus

| Preset | Track | Subjects | Subject populated by |
|---|---|---|---|
| `Crossfade` | `relation.crossfade` | `from` + `to` | canonicalizer |
| `Push` | `relation.push` | `from` + `to` | canonicalizer |
| `Morph` | `relation.morph` | `from` + `to` | canonicalizer |

These are **architecturally distinct from anything corpus-witnessed.** Every
prior transition track in the corpus operates on a single subject (the
incoming surface for enter, the outgoing surface for exit). Relation
transitions coordinate **two surfaces simultaneously** — the prior surface
fades/displaces/morphs into the next.

This raises a fresh design question.

## Q16. How does shorthand express two-subject transitions?

The corpus shorthand to date has been single-subject:

```json
"transitions": { "enter": { "preset": "iris", "shape": "circle" } }
```

The recipe author writes "iris reveal" and the canonicalizer fills in
`subject: "to"` because that's the only sensible choice for an enter mask.

For relation transitions, the canonicalizer needs to identify **two**
subjects. Options:

**A. Implicit-from-context.** The shorthand stays single-line:
```json
"transitions": { "enter": { "preset": "crossfade", "duration": "300ms" } }
```
The canonicalizer fills in:
- `from`: a synthetic `{ kind: "previous" }` reference (whatever was rendered
  before this recipe loaded — the host's prior frame, or another recipe).
- `to`: `{ kind: "scene", "id": "mainScene" }` (this recipe's main scene).

This is the simplest authoring surface. It works for the typical case
(crossfading between two notifications, or between recipe A and recipe B).

**B. Explicit subjects when needed.** The shorthand allows override:
```json
"transitions": {
  "enter": {
    "preset": "crossfade",
    "from": "@previousRecipe",
    "to": "@mainScene",
    "duration": "300ms"
  }
}
```

Needed when the recipe wants to crossfade with something other than the
default `previous`. Possibly rare but real.

**C. Both.** Default to implicit-from-context; allow explicit override when
specified.

**Recommendation:** option C. Default behavior matches the simple case (no
boilerplate for the typical crossfade); explicit `from`/`to` references
preserved for the cases where authors need control.

The five paired files I wrote use option A's shorthand form, with the
canonicalizer expanding `subjects: { from: { kind: "previous" }, to: { kind:
"scene", id: ... } }` from context.

## Alias and expansion table draft entries

Once the contract types land, the alias and expansion tables get these rows
verbatim. Format follows the meta-schema sketched in M4_PATTERNS.md §M5.

### Alias table (`schemas/v3.1/authoring/transition/transition-aliases.json`)

```json
{
  "crossfade": "Crossfade",
  "cross-fade": "Crossfade",
  "fade-between": "Crossfade",

  "push": "Push",
  "push-ltr": { "preset": "Push", "direction": "leftToRight" },
  "push-rtl": { "preset": "Push", "direction": "rightToLeft" },
  "push-up": { "preset": "Push", "direction": "bottomToTop" },
  "push-down": { "preset": "Push", "direction": "topToBottom" },

  "morph": "Morph",
  "content-morph": "Morph",
  "glyph-morph": { "preset": "Morph", "match": "glyph" },

  "stippled": "Stippled",
  "stipple": "Stippled",
  "dither-reveal": "Stippled",

  "braille": "Braille",
  "braille-reveal": "Braille",
  "subcell-reveal": "Braille"
}
```

### Expansion table (`schemas/v3.1/authoring/transition/transition-preset-expansion.json`)

```json
{
  "Crossfade": {
    "subjects": { "from": "@previous", "to": "@scene" },
    "tracks": [
      { "kind": "relation.crossfade", "from": "from", "to": "to" }
    ]
  },
  "Push": {
    "subjects": { "from": "@previous", "to": "@scene" },
    "params": { "direction": "leftToRight" },
    "tracks": [
      { "kind": "relation.push", "from": "from", "to": "to", "direction": "$direction" }
    ]
  },
  "Morph": {
    "subjects": { "from": "@previous", "to": "@scene" },
    "params": { "match": "glyph", "unmatchedPolicy": "fade" },
    "tracks": [
      { "kind": "relation.morph", "from": "from", "to": "to", "match": "$match", "unmatchedPolicy": "$unmatchedPolicy" }
    ],
    "reducedMotion": { "policy": "substitute", "transition": "@crossfade" }
  },
  "Stippled": {
    "subjects": { "to": "@scene" },
    "params": { "pattern": "ordered", "density": 0.5 },
    "tracks": [
      { "kind": "visibility.stippled", "subject": "to", "pattern": "$pattern", "density": "$density", "seed": "$seed?" }
    ]
  },
  "Braille": {
    "subjects": { "to": "@scene" },
    "params": { "subcellOrder": "raster" },
    "tracks": [
      { "kind": "visibility.braille", "subject": "to", "subcellOrder": "$subcellOrder", "seed": "$seed?" }
    ],
    "reducedMotion": { "policy": "substitute", "transition": "@stippled" }
  }
}
```

Sigil notes for the meta-schema:
- `@previous` — synthetic subject reference resolving to "whatever was
  rendered before this recipe loaded."
- `@scene` — synthetic subject reference resolving to "this recipe's main
  scene."
- `@crossfade`/`@stippled` — references to other transition presets in the
  same table (used by `reducedMotion.transition`).
- `$param` — author-supplied parameter substitution. `$param?` is optional.

The graceful-degradation chain matters for accessibility per the architecture
doc's reduced-motion rule (must terminate, must not cycle):
- `Morph` → `Crossfade` → `none` (instant)
- `Braille` → `Stippled` → `none`

## Updated transition-preset coverage

After round 6:

| Preset | Witnesses | Status |
|---|---|---|
| `Wipe` | 15+ | corpus-promoted |
| `fade` | 12+ | corpus-promoted |
| `Iris` | 6 | corpus-promoted |
| `Dissolve` | 6 | corpus-promoted |
| `Blinds` | 5 | corpus-promoted |
| `Diamond` | 6 | corpus-promoted |
| `Cellular` | 5 | corpus-promoted |
| `Radial` | 3 | corpus-promoted |
| `Checkers` | 3 | corpus-promoted |
| `Path_reveal` (with spiral) | 3 | corpus-promoted |
| `Noise_dither` | 3 | corpus-promoted |
| **`Crossfade`** | 0 (schema-faith pair) | **schema-promoted, design-Q16 raised** |
| **`Push`** | 0 (schema-faith pair) | **schema-promoted** |
| **`Morph`** | 0 (schema-faith pair) | **schema-promoted, distinct from content.morph** |
| **`Stippled`** | 0 (schema-faith pair) | **schema-promoted** |
| **`Braille`** | 0 (schema-faith pair) | **schema-promoted** |

All 16 presets now have either corpus witnesses or schema-faith demonstrators
plus alias/expansion entries. **The transition-preset surface is complete.**

## Final corpus state after round 6

- **174 recipes read + 5 schema-faith demonstrators authored** = 179 design
  evidence units.
- **42 paired files** (37 corpus-derived + 5 schema-faith).
- **6 M4 docs** (`M4_PATTERNS.md` + `_v2`–`_v6`).
- **16 open design questions** (15 from prior + Q16 from this round).
- **Pattern catalog complete enough for M5.**

## Recommendation unchanged

Move to M5. The corpus is closed evidence; remaining decisions are design
calls (Q1–Q16), and the schema-faith demonstrators give the alias and
expansion tables their first 5 rows for the structurally-new family.
