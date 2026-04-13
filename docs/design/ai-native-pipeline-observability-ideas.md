# AI-Native Pipeline Observability — Running Design Notes

Captured in context while auditing 100 debug recipes in `tui-vfx-recipes`.
Each entry: the friction, the idea, priority. Moved into `tui-vfx` because
the observability layer these ideas describe belongs at the engine level,
not inside a recipe-specific tool — anyone consuming `tui-vfx` should get
the same debuggability surface, not just recipe authors.

## Core vision

A CLI + library that lets an AI see the **entire pipeline playback** —
every cell, every modification, every stage, every frame — in a format
optimized for machine parsing and progressive disclosure. Replaces the
current `pipeline-validator --dump` (grid map + sample cells, built for
humans glancing at output) with a layered query model where broad
summaries drill down to per-cell deltas on demand. The same tool works
whether the input is a recipe (via the adapter in `tui-vfx-recipes`) or
a direct engine `CompositionOptions` (via the programmatic API).

## Ideas log

### 1. Structured output mode (JSON / NDJSON)
**Friction:** I spent 30% of the audit time grep-ing prose dump output for
"fg=Rgb(...)" patterns. Every answer required a regex. Regex-fragile, slow,
error-prone when a filter's prose mentions "Rgb" in its description.
**Idea:** `--format json` flag that emits the full result as structured data.
NDJSON preferred so streaming over many frames is cheap. Every scalar typed.
**Priority:** P0 — without this nothing else I list is useful.

### 2. Full-buffer cell dump (not just first-N)
**Friction:** The "first non-empty cells" list is capped at 10 (now 32), and
iterates row-major, so it exhausts on the top border row and never reaches
the interior where most shader effects live. I had to grep for the orbit
color globally to even confirm the 3 dots existed — and found zero because
the dump simply didn't emit them.
**Idea:** `--cells all` or `--cells rect=x,y,w,h` mode that emits *every* cell
in the requested region. Each cell entry: (x, y, ch, fg, bg, modifiers,
last_touched_by_stage, last_touched_by_effect).
**Priority:** P0.

### 3. Per-stage causation trace per cell
**Friction:** When I see an "anomalous" cell I have no way to know which
stage/effect put it there. For pattern_fill, the validator reported 0 mods
until I fixed the counter to include char changes — but even then I had no
way to ask "what exactly did the filter do to cell (27,8)?".
**Idea:** Per-cell causation record: for each modification, store
`{ stage: "filter", effect: "PatternFill", before: {...}, after: {...}, t: 0.5, phase: "Dwelling" }`.
Query by cell position to get the full causation chain.
**Priority:** P0 — this is the single biggest leverage gain.

### 4. Frame timeline query
**Friction:** To understand a looping animation I had to run the validator
7-10 times at different `--sample-t` values, concatenate output, and diff by
eye. Every invocation re-parses the recipe, re-runs the pipeline, re-prints
~100 lines of boilerplate.
**Idea:** `--timeline phase=dwelling --frames 10 --format json` produces a
single array of 10 frame states. One parse, one pipeline setup, N renders.
Same for entering/exiting. Combined mode `--timeline all --frames 30`.
**Priority:** P0.

### 5. Frame-to-frame diff mode
**Friction:** When comparing orbit dots at t=0.3 vs t=0.5, I need to mentally
diff two grid maps. For 100-cell widgets this is unreliable.
**Idea:** `--diff from=0.3 to=0.5` emits only the (x, y, before, after)
tuples of cells that changed. For pure animation recipes this collapses
the output from 200 cells × 10 frames to a dozen deltas per step.
**Priority:** P0.

### 6. Row/column anomaly detector (DONE for bg/fg)
**Done:** Added in fnc_sample_buffer_cells.rs v1.3.0. Per-row mode comparison
highlights outliers even when brightness buckets can't distinguish them.
Caught orbit's 3 dots immediately.
**Next iteration:** Extend to column anomaly (for vertical effects like
focused_row_gradient) and full-widget 2D anomaly (mark cells that differ
from the widget's mode).
**Priority:** P1.

### 7. Recipe expectation DSL / assert mode
**Friction:** Half the audit was "does observed match the recipe
description?". I had to hand-translate the English description into a
mental model, then hand-compare. Subjective, slow, unreliable.
**Idea:** Let the recipe author embed expected-state assertions alongside
the recipe: `"assert": { "at": "dwelling@0.5", "cells": [{"x":8,"y":6,"fg":"orbit_color"}], "mod_count": {"shader": ">=3"} }`.
Validator verifies. `--assert-strict` fails on any mismatch.
**Priority:** P1 — high payoff but requires schema design and buy-in.

### 8. Effect attribution by region
**Friction:** For a shader like TracePath with 2 snake heads, I need to know
"is the head at the expected position?". The validator gives me the full
modification list but not a spatial summary.
**Idea:** `--effect-bounds` emits the bounding box of each effect's modifications
per frame. E.g., `TracePath: bbox=(1,7)..(18,7), cells=17, head_at=(18,7)`.
Lets me verify path geometry without parsing every cell.
**Priority:** P1.

### 9. Uniform color-delta classification (finer than K/D/M/L/W)
**Friction:** Orbit cells Rgb(100,220,255) and base Rgb(200,200,220) both
classify as `L` in the brightness bucket. Had to add an anomaly map to
distinguish them. Works but is a hack; real solution is a deltaE-based
classifier.
**Idea:** Replace the 5-bucket classifier with a "distance from base_style"
classifier that uses perceptual color distance. Output per-cell delta as
`0-9` gradations instead of 5 fixed buckets. Preserves readability while
giving finer resolution.
**Priority:** P2 — current anomaly map is good enough for 95% of cases.

### 10. Cell query by absolute coord
**Friction:** I'd often want to ask "what's at frame (36, 12)?" without
dumping everything. Currently impossible without grep.
**Idea:** `--cell x,y` flag that prints exactly one cell's full state plus
its causation chain. Repeatable. `--cell 36,12 --cell 37,12 --cell 40,10`.
**Priority:** P1.

### 11. Time-series trace of a single cell
**Friction:** To verify an animation, I want "what happens at cell (8,6)
across t=0.0, 0.1, 0.2, ..., 1.0 during dwell?". Currently requires 10 runs.
**Idea:** `--cell-trace 8,6 --phase dwelling --frames 20` produces a single
row of the cell's state at each frame, showing color/char evolution.
**Priority:** P1.

### 12. Effect list with invocation counts
**Friction:** I can't easily see "which effects actually ran at all" without
reading stage logs. For debugging "is my style effect even being applied?"
this is the first thing I want.
**Idea:** `--effects-ran` prints a compact list: `filter:Dim(105 cells),
shader:Orbit(3 cells), style:Pulse(60 cells)`. One line, tells me what
touched pixels.
**Priority:** P0 — trivial to add, huge value.

### 13. Parse-failure sweep helper
**Friction:** I wrote a bash loop to find parse failures across 100 files.
Should be a built-in.
**Idea:** `--batch recipes/debug_recipes/**/*.json --failures-only` prints
only the failing ones and their errors. Default exit code non-zero on any
failure.
**Priority:** P1.

### 14. "Why is this cell not modified?" diagnostic
**Friction:** When pattern_fill showed 0 modifications, I had no way to
ask "why is cell (27,8) unchanged by this filter?". Took me 20 min of
log reading to discover the counter was broken, not the filter.
**Idea:** `--why-unchanged 27,8` asks the pipeline to explain: was the
cell visited by each stage, what did the stage do, was the result equal
to the input? Distinguishes "stage skipped" from "stage no-op" from
"stage wrote same value".
**Priority:** P2 — nice-to-have diagnostic.

### 15. Recipe self-description prose extraction
**Friction:** To compare observed vs expected I kept opening the recipe
JSON and manually reading the description field. Should be auto-surfaced
alongside the dump.
**Idea:** Every output format includes `recipe_description` at the top
so the expectation is right next to the observation.
**Priority:** P1 — tiny change, big ergonomic win.

### 16. Coverage-gap sentinel in test_filter_recipe_coverage.rs style
**Already exists.** Extend to shaders, samplers, masks, styles so the test
suite catches missing recipes across the entire effect taxonomy.
**Priority:** P1.

### 17. Enter/dwell/exit phase completeness summary
**Friction:** Several filter recipes (Dim, Invert, Tint) put their filter
in enter/exit but NOT dwell, so a default `--phase dwelling` audit reports
zero. I wasted time investigating these before realizing they were
phase-shifted correctly.
**Idea:** `--phases-used` flag reports which phases actually contain
non-none effects per layer. `phases_used: {enter: [filter:Dim], dwell: [],
exit: [filter:Dim]}`. Lets me pre-filter which phases to sample.
**Priority:** P1.

### 18. Compressed grid map legend inline
**Friction:** Current grid map legend is repeated verbosely in every dump.
For machine consumption this is noise.
**Idea:** In `--format json` omit the legend (machines don't need it).
In prose mode, print legend once at the top, not per-map.
**Priority:** P2 — cosmetic.


### 19. Stage counter coverage gaps for style + content
**Friction:** 27 of 100 recipes (14 content, 12 style, 1 baseline) report
zero modifications across every stage counter because the StageInspector
only counts sampler/mask/shader/filter — style effects (FadeIn, Pulse,
Rainbow, Glitch, etc.) and content effects (Typewriter, Marquee,
Dissolve, etc.) run in separate pipeline stages with no counter. I
cannot use `--stages` to detect whether a style or content recipe is
actually doing anything. I have to fall back to cell-diffing across
sample_t values, which is slow and noisy.
**Idea:** Extend StageInspector with on_style_effect_applied() and
on_content_effect_applied() hooks that record per-cell transformations
the same way filter/shader do. Add STYLE: N/M and CONTENT: N/M lines
to the stages report.
**Priority:** P0 — without this, 27% of recipes are untestable by the
primary audit tool.

### 20. Widget area offset in outputs
**Friction:** The `Sample cells` list shows cells with frame-absolute
coords (e.g., (27,8)) but the grid map is in widget-local coords.
Translating between them is constant mental overhead when sanity-checking
a position.
**Idea:** Every cell coordinate in the output carries both absolute and
widget-local coords: `(27,8 / widget 5,1)`. JSON mode: `{abs:[27,8], wl:[5,1]}`.
**Priority:** P1.


### 21. Phase-boundary tick verification / accurate sample_t math
**Friction:** When I asked the validator for `--sample-t 0.5 --phase exiting`
on style_fade_out, the render plan reported back `phase=Dwelling, t=0.40` —
the lifecycle tick did not advance into the exit phase. Either the
validator's phase-math is off or the effect sits in a phase that doesn't
get ticked correctly when sampled explicitly. Either way, I have no
confidence that `--phase exiting` produces the rendering I expect without
inspecting the item's reported phase afterward.
**Idea:** Assert: requested phase == reported phase, or error loudly.
Expose the computed tick time explicitly in every output so the AI can
confirm the simulation is where it thinks it is.
**Priority:** P0 — silent phase drift destroys trust in every other
observation.

### 22. Named-color handling in color grep
**Friction:** My time-diff script grep'd `fg=Rgb\(...\)` which misses
cells whose fg is a named color (Cyan, Red, White). Debugging why a
recipe with `"foreground": {"type": "cyan"}` never matched my filter
cost me 10 minutes. A tool designed for machines should never force me
to reason about color representation variance.
**Idea:** JSON mode normalizes every color to a canonical form:
`{space: "rgb", r, g, b}` or `{space: "named", name: "cyan"}`. Either
way the structure is predictable.
**Priority:** P0 — this is just "structured output" applied correctly.

### 23. Silent unknown-field rejection on effect structs
**Friction:** I fixed three style recipes whose field names drifted from
the engine schema (target→target_color, color→pulse_color, speed→rotation_speed).
But I also found `style_fade_out.json` uses `"ease": "QuadIn"` when the
schema expects `"easing"` — and the recipe passes parse because serde
silently dropped the unknown field. The fade_out runs with default easing
(linear), which is... fine, except the author's intent is lost without
warning.
**Idea:** `--strict-schema` flag that treats unknown fields as errors.
Or a `--warn-unknown` that reports them. Let the AI catch silent drifts
without needing to know the exact field set.
**Priority:** P1 — gives authors a safety net and me an audit tool.

### 24. Quick "is this effect running?" check
**Friction:** For style_fade_out I had to cross-reference `exit_effect: Some`
with observed bg staying static across sample_t. That requires two separate
validator invocations and mental correlation. I want one command that says
"style effect FadeOut is configured but produced 0 observable changes across
exit phase — probable bug".
**Idea:** `--effect-health` mode runs each configured effect at multiple
sample_t and reports which effects produce zero observable changes.
Output: `FadeOut @ exit_phase: 0 cells changed across 10 samples (EXPECTED_CHANGE_NOT_OBSERVED)`.
**Priority:** P0 — this is the single question I want to ask most often.


---

## Audit observations that shaped this doc

- **24 friction points in ~4 hours of audit.** Every single one is a
  "this query should be one command, not 10 lines of bash + grep".
- **Friction Category A: representation.** Dumps output prose with
  embedded colors/chars that require regex to extract. `--format json`
  fixes 80% of this instantly.
- **Friction Category B: coverage.** Dump samples top-N cells in row-major,
  which means the top border dominates every output and interior effects
  are invisible unless you know where to look.
- **Friction Category C: counter gaps.** Style and content stages have
  no mod counter, so 27% of recipes register as "zero activity" in the
  main health check.
- **Friction Category D: phase math.** `--phase exiting` silently does
  not reach the exit phase on some recipes. Cannot build a trustworthy
  audit on an untrustworthy clock.
- **Friction Category E: causation loss.** I can observe a cell's final
  state, but I cannot trace *why* it has that state — which stage wrote
  it, which effect owned the write, what it was before. The inspector
  captures this internally but doesn't expose it per-query.

## Proposed canonical command shape

```
recipe-inspect <recipe.json> [--format json|prose] \
    --frames N                      # timeline of N samples across phase(s)
    --phase enter|dwell|exit|all    # which phase(s) to simulate
    --region full|rect=x,y,w,h      # cells to include
    --cells all|non_empty|modified  # which cells to emit
    --with-causation                # include per-cell stage/effect history
    --effects-only                  # compact "what ran" list
    --diff from=<t> to=<t>          # only emit cells that changed
    --cell-trace x,y                # single cell across frames
    --why-unchanged x,y             # diagnostic for null-mod cells
    --assert-strict                 # fail on any phase/schema/count anomaly
```

Everything else is progressive disclosure on top of that core.

## Suggested first-pass implementation order

1. `--format json` structured output (P0) — unblocks all AI-grade querying.
2. Style + content stage counters (P0, #19) — closes the 27% coverage gap.
3. Phase-tick correctness + reporting (P0, #21) — builds trust.
4. `--effect-health` / `--effects-only` (P0, #24, #12) — one-command "did
   everything run?" question.
5. Per-cell causation trace (P0, #3) — required for non-trivial debugging.
6. Timeline + diff modes (P0, #4, #5) — required for animation verification.
7. Cell query + cell-trace (P1, #10, #11) — required for targeted probes.
8. `--why-unchanged` diagnostic (P2, #14) — polish.

Everything earlier than P0 in the canonical command shape comes "for free"
once the first four land.


### 25. Lifecycle timing asserts / self-consistency check
**Friction:** The validator and the engine had different mental models
of `auto_dismiss_ms`. Validator computed `dwell_ms = auto_dismiss - enter - exit`;
engine used `dwell_ms = auto_dismiss` directly. The mismatch was silent
— every recipe still rendered *something*, just not at the time the
validator thought it was asking for. I only caught it because fade_out
looked static and I had time to dig. A silent timing desync corrupts
every observation downstream.
**Idea:** The validator should compute a full timeline (tick_0 = create,
tick_1 = enter_start, tick_2 = dwell_start, tick_3 = exit_start,
tick_4 = finished) BEFORE it starts sampling, and print the timeline
at the top of every dump. Assert that the engine's reported phase at
the tick time matches the requested phase; fail loudly if not.
**Priority:** P0 — timing drift destroyed trust in half of my audit
observations for style recipes.

### 26. Silent field-drift detection across recipes
**Friction:** style_fade_in and style_fade_out had `ease: "QuadOut/QuadIn"`
when the engine expected `easing`. serde silently dropped the unknown
field, so the authored easing was never applied but no one noticed for
months. The symptom was "effect runs with default easing" — invisible
unless you squint at color curves. Three similar silent drifts in the
same audit (target, color, speed, ease).
**Idea:** `--lint` mode walks the recipe, compares every field name
against the deserialized struct, and reports unknown fields as
warnings. Or a stricter `--deny-unknown-fields` that fails. Either way
the AI can sweep the corpus with one command and find every silent
drift.
**Priority:** P0 — the single biggest source of "works-in-parse,
broken-at-runtime" recipes in this corpus.

### 27. Effect source / sink trace
**Friction:** To find Rainbow's speed formula I had to grep through
tui-vfx-style, find StyleInterpolator::calculate, read the match arm,
compute `hue = t * speed * 360`, and reverse-engineer which
rotation_speed value gives visible cycling. A tool that exposed this
formula per-effect would save 20 minutes per tuning session.
**Idea:** `--effect-info Rainbow` prints the effect's formula, input
parameters, default values, and a sanity-check table showing
output-at-t for a few sample inputs. E.g.,
`Rainbow.speed=1.0 → t=0.00:0°, 0.25:90°, 0.50:180°, 0.75:270°, 1.00:360°`.
Lets me pick a speed value without guessing.
**Priority:** P2 — nice but can be substituted by reading source.

