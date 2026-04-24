<!-- <FILE>docs/design/tui-vfx-v3-timing-and-metadata-decision.md</FILE> - <DESC>Accepted V3 decisions for distributed timing and optional recipe metadata.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Record owner-approved closure for V3 Q23 timer model and Q21 recipe metadata policy, including debug-recipe visual expectation guidance and the distinction between intrinsic recipe metadata and host/manifest routing policy.</WCTX> -->
<!-- <CLOG>0.1.0: Initial accepted decision record for distributed timing, optional metadata fields, debug recipe expected-visual/body-text requirements, and non-authoritative intent hints.</CLOG> -->

# V3 timing and metadata decision

Status: accepted.

This closes:

- Q23 — timer model
- Q21 — recipe metadata fields

## Timer model

V3 does not add a first-class universal `Timer` primitive.

Timing remains distributed by responsibility:

- Recipe and pipeline lifecycle timing owns whole-recipe or whole-layer phase
  envelopes such as enter, dwell, exit, offsets, and phase durations.
- `mixed-signals` owns continuous temporal signal generation, easing, ADSR,
  keyframes, springs, physics, and reusable time-varying math.
- Effect-local timing remains inside an effect when the effect owns the
  animation semantics and exposing a generic timer would add ceremony without
  reuse.

A generic timer remains deferred until repeated recipe pressure proves that a
single public abstraction would reduce author burden without blurring these
responsibility boundaries.

Implementation guidance:

- Prefer clear field names over a new timer object.
- Keep normalized phase progress and elapsed/absolute time distinct.
- Cadence-bearing effects should use elapsed/absolute time where discontinuity
  would be visible.
- Do not introduce recipe-period hacks to hide timing discontinuities.
- If later corpus pressure requires a timer, propose it as a separate schema
  addition with migration examples.

## Recipe metadata policy

Recipes keep the existing top-level identity fields:

- `id`
- `title`
- `description`
- `version`
- `last_updated`
- `schema_version`

`description` stays. It is the public short summary of what the recipe is and
what it broadly does.

V3 also supports an optional top-level `metadata` block for discovery,
curation, QA, and authoring context. Metadata does not participate in rendering
or host routing.

Recommended optional fields:

| Field | Purpose |
|---|---|
| `intent_hints: [string]` | Non-authoritative discovery hints for likely fit, such as `notification`, `success_feedback`, `debug_preview`, or `primitive_reference`. Hosts must not treat this as routing authority. |
| `expected_visual: string` | Plain-language statement of what a viewer/reviewer should see on screen. Strongly recommended for debug recipes and visual reference fixtures. |
| `visual_tags: [string]` | Visual, motion, family, or technique tags used for search and authoring reference. Replaces the narrower `aesthetic_tags` wording for new docs, but old drafts may still use `aesthetic_tags` during migration. |
| `mood: string` | Optional emotional tone, such as `calm`, `urgent`, `playful`, `technical`, or `optimistic`. |
| `related_themes: [string]` | Optional theme affinity or `theme-neutral` for recipes that are not theme-bound. |
| `maturity_era: string` | Optional audit/migration history marker, such as `basic`, `mature`, `professional`, or `experimental`. |
| `authoring_notes: string` | Concrete authoring rationale, caveats, substitution hints, or warnings for future editors. |
| `last_reviewed: string` | Optional ISO date for audit freshness. |

All fields are optional at the core schema level. Validators may warn when
important discovery or visual-QA fields are absent from shipped recipe corpora,
but absence is not a V3 core parse error.

Example:

```json
{
  "description": "Eichler success notification with diamond reveal and teal/coral styling.",
  "metadata": {
    "intent_hints": ["notification", "success_feedback"],
    "expected_visual": "A compact success toast enters from the bottom with a diamond reveal and soft bounce.",
    "visual_tags": ["eichler", "atomic", "diamond", "teal", "midcentury"],
    "mood": "optimistic",
    "related_themes": ["eichler"],
    "authoring_notes": "Uses diamond masking to carry the atomic motif. Keep the canvas compact so it remains usable as a notification treatment.",
    "maturity_era": "professional",
    "last_reviewed": "2026-04-24"
  }
}
```

## Debug recipe requirements

Debug recipes and visual reference fixtures have a stricter authoring bar than
ordinary recipes because they are regression baselines and first-quality
examples for humans and AI authors.

For debug recipes:

- `description` must explain what the viewer should expect to see.
- `metadata.expected_visual` is warning-level strongly recommended.
- Body/message text is fixture presentation, not filler.
- Body/message text should usually follow:

```text
<Family>: <Human Name>
<Concise behavior cue>
```

Example:

```text
Mask: Iris Effect
Expands from center with soft edge
```

The first line tells the viewer what primitive or composition is being tested.
The second line gives a concise behavior cue. Avoid filler such as `Watch`
when it does not add information.

Debug recipe layout must comfortably contain this text. If the label and cue do
not fit, resize or retune the fixture rather than leaving clipped or ambiguous
reference content.

## Host and manifest routing boundary

`intent_hints` are discovery hints only. They are not routing authority.

The host, app, manifest, theme, or recipe collection decides how a recipe is
used. For example, a downstream theme manifest can bind a recipe to a modal,
toast, splash, drawer, or exit screen without the recipe itself claiming that
role as a requirement.

This keeps portable recipes reusable while still giving authors and tools enough
context to find likely matches.

<!-- <FILE>docs/design/tui-vfx-v3-timing-and-metadata-decision.md</FILE> - <DESC>Accepted V3 decisions for distributed timing and optional recipe metadata.</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
