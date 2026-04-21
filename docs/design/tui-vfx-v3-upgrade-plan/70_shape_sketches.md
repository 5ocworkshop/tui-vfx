<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/70_shape_sketches.md</FILE> - <DESC>Chapter 70 — shape sketches: small illustrative flat-vs-tree JSON comparisons for three representative cases (simple fade-in toast, ember-felt three-layered dwell, ambient-halo four-per-edge not-expressible-in-flat). Concrete before/after to anchor the abstract tree-schema decision.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Extracted from the monolithic plan (v0.16.0) "Shape sketches" section. Verbatim. Broader translation study is deferred to Workflow C in Chapter 110.</WCTX> -->
<!-- <CLOG>1.0.0: initial extraction from the monolith.</CLOG> -->

# 70 — Shape sketches

Small illustrative examples. A broader translation study across ~12 diverse recipes lives in Workflow C (see `110_appendix_audits.md`) and its sibling appendix file.

## 10 — Simple fade-in toast

**Flat (today):**

```json
{
  "pipeline": {
    "enter": {"duration_ms": 300, "easing": "quad_out"},
    "exit":  {"duration_ms": 200, "easing": "quad_in"},
    "mask": {"enter": {"type": "none"}, "exit": {"type": "none"}},
    "sampler": {"enter": {"type": "none"}, "exit": {"type": "none"}},
    "filter": {"dwell": []},
    "styles": [
      {"region": "All", "base_style": {...},
       "enter_effect": {"type": "fade_in"}}
    ]
  }
}
```

**Tree (proposed):**

```json
{
  "pipeline": {
    "timing": {"enter_ms": 300, "exit_ms": 200, "enter_ease": "quad_out", "exit_ease": "quad_in"},
    "step": {
      "kind": "style_effect",
      "phase": "enter",
      "payload": {"type": "fade_in"}
    }
  }
}
```

Simple recipe is visibly simpler. No ceremony for masks, samplers, filters the recipe doesn't use.

## 20 — Ember-felt (three layered dwell operations)

**Flat (today):** scattered across `pipeline.styles[1].spatial_shader`, `pipeline.styles[2].spatial_shader`, and `pipeline.filter.dwell[0]`. Semantic grouping ("these three all happen during dwell on the background") is invisible in the document structure.

**Tree (proposed):**

```json
{
  "step": {
    "kind": "parallel",
    "phase": "dwell",
    "scope": {"kind": "channel", "value": "background"},
    "children": [
      {"kind": "shader", "payload": {"type": "diffusion", "source": "top_right", "color": {...}, "mode": "warm_drift"}},
      {"kind": "shader", "payload": {"type": "concealed_light", "source": "left", "color": {...}}},
      {"kind": "filter",  "payload": {"type": "vignette", "sides": ["bottom", "right"]}}
    ]
  }
}
```

Scope propagates to all three children. The three-operation structure reads at a glance. Named factories (`diffusion`, `concealed_light`, `vignette`) remain the JSON surface; internally they load to the decomposed model.

## 30 — Ambient halo (not expressible in flat schema)

Four per-edge diffusion instances scoped to the recessed canvas, bound to runtime-sampled colors:

```json
{
  "step": {
    "kind": "parallel",
    "phase": "all",
    "scope": {
      "kind": "and",
      "children": [
        {"kind": "channel", "value": "background"},
        {"kind": "rect_exclude", "source": "focus_rect"}
      ]
    },
    "children": [
      {"kind": "shader", "payload": {"type": "colored_overlay", "pattern": {"kind": "radial_from_edge", "edge": "top"}, "color": {"kind": "sampled", "source": "focus_edge_top"}}},
      {"kind": "shader", "payload": {"type": "colored_overlay", "pattern": {"kind": "radial_from_edge", "edge": "bottom"}, "color": {"kind": "sampled", "source": "focus_edge_bottom"}}},
      {"kind": "shader", "payload": {"type": "colored_overlay", "pattern": {"kind": "radial_from_edge", "edge": "left"}, "color": {"kind": "sampled", "source": "focus_edge_left"}}},
      {"kind": "shader", "payload": {"type": "colored_overlay", "pattern": {"kind": "radial_from_edge", "edge": "right"}, "color": {"kind": "sampled", "source": "focus_edge_right"}}}
    ]
  }
}
```

This composition is impossible in the flat schema today because: filters can't scope to non-rect regions; shaders can't bind to runtime-sampled colors; and there's no way to declare "four instances of the same operation, each with per-edge parameters." The tree schema + unified scope + Pattern-axis + runtime color binding make it natural.

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/70_shape_sketches.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
