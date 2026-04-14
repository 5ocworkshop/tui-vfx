<!-- <FILE>docs/RECIPE_AUTHORING_WORKFLOW.md</FILE> - <DESC>Canonical staged workflow for authoring and flattening complex tui-vfx recipes</DESC> -->
<!-- <VERS>VERSION: 1.2.0</VERS> -->
<!-- <WCTX>Recipe workflow plus focused-cell, motion-analysis, and text-integrity diagnostics</WCTX> -->
<!-- <CLOG>MINOR: Update the canonical authoring workflow to include focused widget-cell debugging, timeline motion analysis, and partial-match text diagnostics as first-class tools in the staged recipe loop</CLOG> -->

# Recipe Authoring Workflow

Complex terminal recipes are easiest to get right when they are authored in
stages, verified incrementally, and only flattened into a single final recipe
after the composition is proven.

This recommendation came directly from auditing the probe-validation corpus:
recipes can be **structurally valid but visually wrong** when too many effects
are combined at once.

Typical failure modes:
- borders get sampled or filtered into mush
- text clips or becomes unreadable
- a late-stage filter overwrites the effect you actually cared about

## Recommended build order

Draft complex recipes in staged files, validating each stage before adding the
next one. Once the composition is working and signed off, flatten it back down
to a single final recipe file for normal browsing and use.

1. **`01_base`**
   - layout
   - message
   - border
   - base style only
   - confirm text fits and border looks correct

2. **`02_content`**
   - add content effect only
   - confirm legibility and timing

3. **`03_mask`**
   - add mask only
   - confirm reveal geometry does not chew the border or clip the text

4. **`04_sampler`**
   - add sampler only
   - confirm motion distortion helps rather than destroys the frame

5. **`05_style`**
   - add pulse/fade/color-shift/shader layers
   - confirm hierarchy is still readable

6. **`06_filter`**
   - add the most destructive layer last
   - confirm it does not cover the message or break the border

7. **`final`**
   - run validator
   - run probe timeline/diff
   - run manual visual QA
   - flatten the proven composition back to one final file

## Effective debugging loop

Use the structured tooling at every stage:

```bash
cargo run -q -p pipeline-validator -- --rules --stages path/to/stage.json
cargo run -q -p recipe-probe -- --with-causation path/to/stage.json
```

When a single cell looks wrong, use focused-cell mode instead of scanning a
full frame dump:

```bash
cargo run -q -p recipe-probe -- \
  path/to/stage.json \
  --phase dwelling \
  --sample-t 1.0 \
  --with-causation \
  --widget-cell 0,0
```

For time-varying stages:

```bash
cargo run -q -p recipe-probe -- \
  --with-causation \
  --diff-to 0.66 \
  --sqlite-query "select count(*) as changed_cells from probe_diff_cells" \
  path/to/stage.json
```

For animated effects, use timeline motion analysis early rather than waiting for
human visual doubt:

```bash
cargo run -q -p recipe-probe -- \
  path/to/stage.json \
  --phase dwelling \
  --frames 5 \
  --with-causation \
  --sqlite-query "select effect, span_x, span_y, status from probe_motion_effects order by effect"
```

Recipe-side tooling also now has a recipe-aware diagnostics layer on top of the
probe output. That makes it possible to flag some failures automatically, such as:

- expected message missing at a dwell sample
- expected message partially matching the target text with a similarity score
- text leaking onto border rows
- underline contamination on the bottom border

## Important limitation

The probe database and JSON reports are a **force multiplier**, but not a full
replacement for visual QA.

They are excellent for:
- proving that stages ran
- identifying which cells changed
- explaining when and where changes occurred

They are not always sufficient for:
- subtle border corruption
- text offset/spacing defects
- visually bad but structurally valid layer combinations
- motion continuity/sweep-boundary defects unless a specific diagnostic exists

That is why the staged workflow and a final human-eye QA pass are both required.

## Heuristic

If a recipe needs more than one sampler/filter/shader-like layer to feel alive,
prefer to:
- prove each layer in isolation first
- keep screenshots or row-level probe output for comparison
- collapse to a single final recipe only after the staged versions all behave
- whenever you discover a new repeatable visual failure class, try to encode it
  into probe diagnostics or regression tests so future audits catch it automatically

## Related docs

- [PIPELINE_PROBE_LLM_GUIDE.md](PIPELINE_PROBE_LLM_GUIDE.md)
- [PIPELINE_VALIDATOR_LLM_GUIDE.md](PIPELINE_VALIDATOR_LLM_GUIDE.md)
- [RECIPE_VISUAL_QA.md](RECIPE_VISUAL_QA.md)
- [PIPELINE_PROBE_WISHLIST.md](PIPELINE_PROBE_WISHLIST.md)

<!-- <FILE>docs/RECIPE_AUTHORING_WORKFLOW.md</FILE> - <DESC>Canonical staged workflow for authoring and flattening complex tui-vfx recipes</DESC> -->
<!-- <VERS>END OF VERSION: 1.2.0</VERS> -->
