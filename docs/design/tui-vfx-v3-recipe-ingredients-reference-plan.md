<!-- <FILE>docs/design/tui-vfx-v3-recipe-ingredients-reference-plan.md</FILE> - <DESC>Plan and template for a standardized V3 recipe ingredients reference</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Define the author-facing ingredients catalog needed by staged recipe authoring prompts and future AI/human recipe authors.</WCTX> -->
<!-- <CLOG>0.1.0: add the recipe ingredients reference template and rollout plan.</CLOG> -->

# V3 Recipe Ingredients Reference Plan

We do not yet have a good enough single reference for every author-facing
recipe ingredient. The information exists, but it is scattered across the
authoring guide, schema docs, capabilities reference, debug recipes, procedural
source docs, and implementation-specific examples.

This document defines the shape of the reference we need.

## 1. Goal

Create a standardized **recipe ingredients reference**: a catalog of the things
authors combine to make V3 recipes.

The reference should help humans and AI authors answer:

- What ingredients are available?
- What does each ingredient feel like?
- What is it good for?
- What should it be paired with?
- What should it not be used for?
- Which fields/bindings/inputs matter?
- Where is a working debug recipe?

This is different from development tooling documentation. Validators, probes,
trace CLIs, and preview players are **development tools**. Content transforms,
masks, shaders, filters, motion routes, procedurals, bindings, timers, progress
treatments, and I/O chains are **recipe ingredients**.

## 2. Standard ingredient entry format

Every ingredient entry should use this format:

```text
## Ingredient Name

Kind: content | motion_route | easing | mask | shader | filter | sampler |
      procedural_source | binding | progress_timer | asset | io_chain |
      host_edge_affordance

One-line concept:
  ...

What it is good for:
  - ...

What it feels like:
  ...

Inputs and knobs:
  - field / binding / token:
  - timing / clock:
  - asset / source:

Pairs well with:
  - ingredient:
    why:

Avoid when:
  - ...

Accessibility / restraint notes:
  - reduced motion:
  - readability:
  - attention level:

Theme-fit prompts:
  - What theme signal could this express?
  - What user moment does it naturally serve?

Reference recipes:
  - path:
    what to inspect:

Validation notes:
  - accepted schema spelling:
  - known bridge/probe caveats:
```

## 3. Recommended catalog sections

Initial catalog sections:

1. Content transforms
2. Motion routes and dynamics
3. Easings
4. Masks
5. Shaders
6. Filters
7. Samplers
8. Procedural sources
9. Progress and timer ingredients
10. Runtime bindings, tokens, and assets
11. I/O chain ingredients
12. Host-edge affordances

Each section should start with a short chooser table:

```text
If you want...        Consider...           Avoid...
subtle focus          FocusedRowGradient    GlitchLines
ambient progress      SubPixelBar + Pulse   fireworks
warning prominence    EdgeGrow + tint       chaotic shake
```

## 4. Quality bar

A good ingredient description is:

- **author-facing**, not implementation-first
- **specific about feel**, not just field names
- **honest about limits**
- **paired with working examples**
- **consistent about canonical vocabulary**
- **useful for combination brainstorming**

The reference should help an author form combinations before writing JSON, for
example:

```text
domestic timer dial + radial mask + numeric/odometer + sub-cell progress
focused row + area-rug metaphor + FocusedRowGradient + selected_row binding
sun patch + GlistenBand + concealed light + slow cubic_out settle
```

## 5. Rollout plan

1. Create the catalog skeleton with the sections above.
2. Populate 4-6 high-value ingredient families first:
   - motion routes/easings
   - masks
   - focused-row/selection shaders
   - progress/timer ingredients
   - procedural sources
   - I/O chain patterns
3. Link each ingredient to at least one debug recipe or probe fixture.
4. Add validation notes where schema spelling or probe bridge behavior is easy
   to get wrong.
5. Use the staged theme-authoring A/B tests to identify missing descriptions.
6. Promote the catalog to core V3 documentation when the first pass is useful.

## 6. Open questions

- Should the first catalog be hand-maintained, generated from rustdoc/schema, or
  hybrid?
- Which fields can be auto-generated without losing the author-facing “feel”
  description?
- Should recipe examples link to debug recipes only, or also curated theme
  evaluation recipes after review?
- How should host-edge affordances be represented while adapter support remains
  boundary-specific?

# <FILE>docs/design/tui-vfx-v3-recipe-ingredients-reference-plan.md</FILE> - <DESC>Plan and template for a standardized V3 recipe ingredients reference</DESC>
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
