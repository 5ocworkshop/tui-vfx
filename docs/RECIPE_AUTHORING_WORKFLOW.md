<!-- <FILE>docs/RECIPE_AUTHORING_WORKFLOW.md</FILE> - <DESC>Canonical staged workflow for authoring and flattening complex tui-vfx recipes</DESC> -->
<!-- <VERS>VERSION: 1.5.1</VERS> -->
<!-- <WCTX>Add primitive-first debug recipe guidance so authors prove the simplest visual form before advanced chaining/showcase recipes, now with explicit EIRA-07 resize-contract evidence references.</WCTX> -->
<!-- <CLOG>1.5.1: Add concrete resize-contract evidence references (`resize_preserve_phase_chain.json` and `diag_resize_preserve_phase`) so the authoring guide states the as-built host-owned resize model directly. 1.5.0: Add primitive-first debug recipe guidance: every primitive/feature/pathway starts with the simplest visual confirmation, with complex I/O showcases as secondary fixtures. 1.4.0: Add grid-size/adaptive-layout guidance: tui-vfx renders to caller-provided grids, host apps own resize events, and authors should choose fullscreen/relative/bound positions intentionally. 1.3.0: Add a “which tool proves what” split, including upstream-native debug-recipes QC and Preview as the canonical player for final visual sign-off</CLOG> -->

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

## Which tool proves what?

| Question | Canonical surface |
| --- | --- |
| “Is this recipe structurally valid upstream?” | `pipeline-validator` |
| “Can upstream emit deterministic structured evidence for this recipe?” | `recipe-probe` or `pipeline-validator --probe` |
| “Can upstream run a repeatable QC bundle over debug fixtures or concrete exports?” | `pipeline-validator --debug-recipes-qc` |
| “Does the canonical player actually look right to a human?” | Preview / demo browser |
| “What happens for a direct engine scene with no recipe parsing?” | `pipeline-probe` |

Preview is the canonical **player**.
Validator is the canonical **recipe-authoring acceptance** surface.
Probe outputs are the canonical **structured evidence** surfaces.

## Primitive-first debug recipes

For `debug_recipes/`, the first fixture for any primitive, feature, or pathway
should be the simplest presentation that makes the behavior immediately visible.
This baseline is the visual truth source for authors and reviewers.

Guidelines:

- Strip away anything not required to see the primitive: no decorative border,
  card background, extra shader, mask, or filter unless that element is the
  primitive under test or makes the behavior legible.
- Put advanced combinations, cross-family chains, and “full potential” showcases
  in secondary complex/composition fixtures after the baseline exists.
- Name and describe the baseline in human terms so the demo browser tells the
  viewer exactly what should move, tint, reveal, distort, or emit.
- When a combined recipe looks wrong, compare it against the primitive baseline
  first; do not make the complex chain the only proof that a primitive works.

Example: a `radial_twist` sampler baseline should move just the message text
through the radial pathway. A later complex fixture may add a shader or I/O
chain, but the text-only pathway proof remains the starting reference.

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
   - run upstream-native QC when stabilizing a reusable debug fixture or validating a resolved concrete export
   - run probe timeline/diff
   - run manual visual QA
   - flatten the proven composition back to one final file


## Grid size and adaptive authoring

`tui-vfx` renders to the grid/`Rect` supplied by its caller. It does not need
to know that a terminal resize happened; the host app, Ratatui integration,
preview runner, or validator supplies a new target grid and asks the recipe to
render again.

The concrete EIRA-07 evidence fixture for this contract is
`tui-vfx-recipes/recipes/debug_recipes/complex/resize_preserve_phase_chain.json`.
Its paired smoke example,
`cargo run -q --example diag_resize_preserve_phase -- recipes/debug_recipes/complex/resize_preserve_phase_chain.json`,
renders the same dwell sample at two host sizes plus a restarted control render
so authors can see the current behavior instead of inferring it from prose.

Authoring implications:

- Use `layout.mode: "fullscreen"` when the recipe should fill whatever grid the
  host provides. In V3, this is the natural adaptive path: resize changes the
  target grid, not the recipe identity.
- Use fixed `width`/`height` deliberately when the recipe is designed to stay a
  fixed-size object inside a larger grid. Fixed recipes may be centered, anchored,
  clipped, or padded by the host/layout rules rather than magically reflowing.
  `recipes/wargames/themes/wopr_cyan.json` and `recipes/wargames/themes/wopr_green.json`
  are the current fixed-size V3 reference shapes.
- Treat absolute cell positions as widget-local defaults. If a position must track
  a changing host layout, expose it through runtime bindings or derive it from
  grid-aware layout/source rules instead of baking one terminal size into the
  recipe.
- Test important recipes at more than one grid size. Preserve phase/sample time
  when comparing sizes so you are testing layout adaptation rather than animation
  restart behavior.

The library core should remain area-in/grid-out. Terminal event polling, resize
events, and decisions like "restart vs preserve phase" belong to host adapters
and preview tools.

## Effective debugging loop

Use the structured tooling at every stage:

```bash
cargo run -q -p pipeline-validator -- --rules --stages path/to/stage.json
cargo run -q -p recipe-probe -- --with-causation path/to/stage.json
```

For upstream-owned recipe QC over debug fixtures or GTD-resolved concrete exports:

```bash
cargo run -q -p pipeline-validator -- \
  --debug-recipes-qc \
  --format json \
  path/to/concrete_recipe.json
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
<!-- <VERS>END OF VERSION: 1.5.0</VERS> -->
