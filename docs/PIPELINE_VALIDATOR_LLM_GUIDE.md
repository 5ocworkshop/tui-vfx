<!-- <FILE>docs/PIPELINE_VALIDATOR_LLM_GUIDE.md</FILE> - <DESC>How an LLM should use pipeline-validator to debug recipe rendering</DESC> -->
<!-- <VERS>VERSION: 1.2.0</VERS> -->
<!-- <WCTX>Clarify validator ownership versus Preview and probe, and document the upstream-native debug-recipes QC flow plus concrete export acceptance</WCTX> -->
<!-- <CLOG>MINOR: Add an authority split for Preview / validator / probe, document --debug-recipes-qc, and explain that concrete GTD bridge exports are valid upstream recipe inputs after token resolution</CLOG> -->

# Pipeline Validator: An LLM's Guide to Inspecting Recipe Output

This document is for LLMs (and humans) who need to **see what a recipe is actually rendering** without launching an interactive demo. The `pipeline-validator` binary lives in the sibling `tui-vfx-recipes` repo at `tools/pipeline-validator/` and exposes the engine's full render path through a CLI you can grep, diff, and reason about.

It exists because, when a recipe behaves unexpectedly, the question "what does the user actually see at frame N?" is the only question that matters — and you can't answer it from the recipe JSON alone.

## When to use it

| Situation | Tool |
|---|---|
| Need the canonical recipe-authoring verdict (parse/rules/stages) | `pipeline-validator <file>` |
| Need a machine-readable upstream QC pass over debug recipes or resolved concrete exports | `pipeline-validator --debug-recipes-qc --format json <path-or-dir>` |
| Recipe parses? Schema valid? | `pipeline-validator <file>` (default) |
| Stage-by-stage cell modification counts | `--stages -vvv` |
| What does the buffer literally look like at a specific time? | `--dump --stage output --sample-t T --phase X -vvv` |
| Is shader X touching cells it shouldn't? | `--stages -vvv` (look at SHADER cell counts vs your expectation) |
| Wrong colors / wrong glyphs at specific positions | grid-map and per-row dumps under `--dump --stage output -vvv` |

You should reach for it **before** asking the user "what do you see?" — it is faster than waiting for a human to launch a demo and describe an animation in words.

## Authority split: Preview vs validator vs probe

- **Preview / demo browser** is the canonical **recipe player** for human-eye sign-off and visual acceptance.
- **`pipeline-validator`** is the canonical **recipe-authoring validation** surface. It owns parse/rules/profile/stage truth plus upstream-native debug-recipes QC.
- **`recipe-probe` / `pipeline-validator --probe`** are the canonical **structured recipe evidence** surfaces when you need JSON frame/timeline/diff/debug output from the same recipe input.
- **`pipeline-probe`** is the canonical **direct engine-scene** probe when you already have a `ProbeSceneSpec` or want engine-only observability without recipe parsing.

Those surfaces are complementary:
- validator answers **“is this recipe authoring input valid and structurally healthy upstream?”**
- probe answers **“what structured frame/timeline evidence do we have for this recipe or scene?”**
- Preview answers **“does the canonical player look correct to a human?”**

Do not move GTD display-truth semantics into validator acceptance. Upstream validator/probe accept **resolved concrete recipe payloads** as normal recipe JSON, but GTD token resolution remains GTD-owned.

## The flags that matter

- **`--rules --stages`** — the standard pre-flight: parses, validates against rules, runs the profile/render/shader/output stages. Use this first.
- **`--phase entering|dwelling|exiting`** — selects which animation phase to simulate. Critical: each phase has its own `t` clock, and an LLM that confuses dwell time with overall lifecycle time will get fooled (see "Time math" below).
- **`--sample-t T1 [T2 T3 …]`** — sample at specific phase progress values (0.0–1.0 within the chosen phase). `--sample-t 0.5 --phase dwelling` means "halfway through dwell".
- **`--dump --stage output`** — dump the rendered buffer state. `-vvv` (triple verbose) is required to actually see the cell-level grid output.
- **`--stages -vvv`** — print per-stage cell modification counts (sampler, mask, shader, filter), plus a sample of cell-by-cell modifications. Use this to localize *which* stage is making the change you didn't expect.
- **`--canvas RRGGBB`** — pre-fill the simulation buffer with this RGB color before each render, so you can see how a recipe composites over a non-default canvas (gt-design beige, dashboard navy, etc.). Hex, no alpha.
- **`--canvas-content MODE`** — what glyphs to paint into the canvas along with the color. `empty` (default) = spaces, `sentinel` = repeating `+` grid (useful for the early color-only tests), `lorem` = lorem ipsum text wrapped across the buffer. Combine with `--canvas` for a full content+color simulation of a widget overlaying a document. Use `lorem` when debugging **glyph bleed-through** bugs — any canvas character visible inside the widget rectangle after the overlay renders is a bug.
- **`--debug-recipes-qc`** — run the upstream-native QC bundle: rules/stages validation, recipe-side structured probe evidence, future-capture artifact hints, family-aware checks for debug fixtures, and GTD-agnostic acceptance of concrete exported recipe JSON.
- **`--trace`** — even more verbose pipeline tracing; usually too noisy. Try `--stages -vvv` first.

## Debug-recipes QC

Use the QC mode when you want one upstream-owned command surface for recipe-authoring proof:

```bash
cargo run -q -p pipeline-validator -- \
  --debug-recipes-qc \
  --format json \
  recipes/debug_recipes/shaders/shader_glisten_band_speed_binding.json
```

Or against a resolved concrete export produced by bridge tooling:

```bash
cargo run -q -p pipeline-validator -- \
  --debug-recipes-qc \
  --format json \
  /path/to/exported/concrete_recipe.json
```

The QC report is intentionally GTD-agnostic. It records:
- validation stages
- frame / lifecycle / timeline / diff evidence
- family tags and binding/canvas checks where relevant
- deterministic artifact hints and a stable fingerprint for future golden capture

It does **not** claim GTD display truth or GTD token-resolution correctness.

## Reading the dump

When you run `pipeline-validator --dump --stage output --sample-t 0.5 --phase dwelling -vvv <recipe.json>`, the most important sections are:

1. **`item 0: area=(X,Y WxH), phase=…, t=…`** — confirms where the widget is being rendered in the frame buffer and what the engine thinks the current phase progress is. **If `t` doesn't match what you asked for, your time math is wrong** (see below).

2. **`Sample cells:`** — eight named positions (corners, center, content area). Quick sanity check that the right region is being inspected.

3. **`First non-empty cells:`** — first ~10 cells in row-major order with non-whitespace symbols. Useful for finding "what's at the start of the widget".

4. **`Grid map (T=trace bg, #=dot+trace, D=dot, B=border, t=text, .=bg, _=empty):`** — a per-cell ASCII classification of the entire widget area, one char per cell. This is the single most useful view — it lets you see the *shape* of what's lit. Each row is labeled with `y= 0` through `y= height-1` in widget-local coordinates.

5. **`Per-row bg colors (R/G/B nibble):`** — a coarser per-cell view that ignores symbols and only classifies background color: `C` = cyan-ish (something bright), `b` = base/dim, `.` = unknown, `_` = no RGB color set. Use this when you want to know "where is the trace shader actually painting cells" without being distracted by glyph content.

For shader debugging, **the per-row bg dump is what you want**. The classification thresholds are loose intentionally — if you need exact colors, drop into the cell sample list above.

## Time math (the part that fooled me for hours)

Phase sample timing is **not** the global lifecycle clock. The validator's `--phase dwelling --sample-t 0.5` means "render the widget at the wall-clock moment that corresponds to 50% of the way through the *dwell* phase", which is `enter_duration_ms + 0.5 * dwell_duration_ms` after item creation, where `dwell_duration_ms = auto_dismiss_ms - enter_ms - exit_ms`.

The validator computes this for you and ticks the lifecycle state machine at the enter→dwell boundary first (so `dwell_start` lands at the real boundary instead of jumping to the target time). But: the engine's `phase_progress` for the dwell phase divides by `auto_dismiss_ms`, *not* `dwell_ms`. So the `t` value the **shader** sees is **not** the same as the `--sample-t` you asked for — it's `(elapsed_since_dwell_start) / auto_dismiss_ms`.

Concretely: with `enter=1100`, `exit=700`, `auto_dismiss=6500`:
- `dwell_ms = 6500 - 1100 - 700 = 4700`
- `--sample-t 0.5 --phase dwelling` ticks at `1100 + 0.5*4700 = 3450 ms`
- The shader sees `plan.t = (3450 - 1100) / 6500 = 0.36`

If you want to predict what the trace shader is doing, **always compute the shader's t using `auto_dismiss_ms` as the denominator**, not `dwell_ms`. The `t` printed in `item 0:` is the engine's value — trust it over your own arithmetic.

## A concrete debugging recipe

The workflow that actually solved a real bug:

1. **Sanity:** `pipeline-validator --rules --stages <recipe>` — confirms parse and structure.
2. **Stage counts:** `pipeline-validator --stages -vvv --sample-t 0.4 0.5 0.6 --phase dwelling <recipe>` — look at the SHADER and FILTER cell modification counts. If a count is suspiciously high, that stage is the suspect.
3. **Visual map:** `pipeline-validator --dump --stage output --sample-t 0.5 --phase dwelling -vvv <recipe>` — read the grid map. Compare what you see against what your mental model of the recipe says should be there. **Any cell that's lit and shouldn't be is a bug** — either in the recipe, or in the engine.
4. **Cross-reference t values:** repeat step 3 at multiple `--sample-t` values to see the animation's progression. A bug that "appears at 50%" should be visible somewhere between t=0.45 and t=0.55.
5. **Read the engine source:** when the visual diverges from the recipe's intent, the divergence happens in the engine. Find the shader/filter implementation and trace through the math for one specific cell coordinate. (See `crates/tui-vfx-style/src/models/cls_*_shader.rs` for spatial shaders; `crates/tui-vfx-compositor/src/filters/cls_*.rs` for filters.)

## Real example: the phantom projection bug

We had a `TracePath` recipe that lit 19 cells in a horizontal line on row y=7, even though only the 2 cells of the actual segment should have been on at that moment. The grid map showed:

```
y= 7: bCCCCCCCCCCCCCCCCCCCbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
```

19 C cells from x=1 to x=19 — but path A's bottom horizontal segment was authored as `(18,7)→(34,7)`, so cells x=1..17 had no business being on. We knew the bug had to be in the shader, not the recipe, because the only way the recipe's geometry could produce extra cells is if the engine was misclassifying them as on-route.

Tracing through `cls_trace_common.rs::project_onto_polyline` for a specific phantom cell (x=5, y=7) revealed the bug: the distance metric for a horizontal segment was just `|cell_y - segment_y|`, which is 0 for any cell on the same row regardless of x. `clamp_x` then snapped the cell's x to the segment endpoint, giving `progress_on_segment = 0`, and the cell was treated as sitting *on the segment endpoint with distance 0*. The thickness filter (`distance > thickness - 1`) didn't reject it because distance was already 0.

Fix: use true point-to-segment Euclidean distance, including the parallel offset when the cell is outside the segment's range. Five regression tests in `cls_trace_common.rs` cover the cases. v1.0.0 → v1.1.0.

**Lesson for LLMs:** when a shader bug *can* manifest as "every cell at the same y/x as a segment lights up at full intensity at the moment the head crosses an endpoint", look for distance metrics that ignore one dimension. The phantom always projects to either the segment **start** (cells outside on one side) or the segment **end** (cells outside on the other side), so the buggy cells form a perfect line at the segment's y (or x) level — that's a fingerprint.

## Limitations

- **Single recipe per invocation** for `--dump` (or rather, the dump output is hard to read across multiple files). Pass one path at a time.
- **Buffer is reused across sample-t values** within a single invocation, so when you pass multiple `--sample-t`, the dump shows the *last* sample's state. To compare frames, run separate invocations and diff the outputs.
- **The grid map's classification is thresholded**, not exact. If a cell is faintly modified (e.g., a fading tail), it may be classified as `b` (base) rather than `C` (cyan). Drop down to the per-cell sample list when you need precision.
- **Animation type matters.** `animation_type: none` skips the entering phase entirely (`lifecycle.rs:29`), so `--phase entering` will produce empty output. `animation_type: fade` runs the phases without positional motion, which is usually what you want for shader debugging.
- **Looped time confuses the mask stage.** When `time.loop = true`, `options.t` is plumbed from `loop_t` instead of phase progress (`fnc_render_animated_with_theme.rs:416`). This means masks see cycling time, not enter/exit progress, which can make a wipe mask look like it "leaps". If a mask isn't running cleanly across its phase, **disable looping** and use the shader's own `speed` multiplier to cycle the trace internally.

## See also

- `docs/CAPABILITIES_REFERENCE.md` — full inventory of shaders, filters, masks, samplers
- `docs/TERMINAL_MOTION_HEURISTICS.md` — *read this first* before authoring any recipe
- `crates/tui-vfx-style/src/models/cls_trace_common.rs` — the projection function and its regression tests
- `tools/pipeline-validator/src/stages/functions/fnc_validate_output.rs` (in tui-vfx-recipes) — the validator's output stage, including the dwell-time and render-plan-aware sampling logic
- `tools/pipeline-validator/src/stages/functions/fnc_sample_buffer_cells.rs` (in tui-vfx-recipes) — the grid-map and per-row dump implementations

<!-- <FILE>docs/PIPELINE_VALIDATOR_LLM_GUIDE.md</FILE> - <DESC>How an LLM should use pipeline-validator to debug recipe rendering</DESC> -->
<!-- <VERS>END OF VERSION: 1.2.0</VERS> -->
