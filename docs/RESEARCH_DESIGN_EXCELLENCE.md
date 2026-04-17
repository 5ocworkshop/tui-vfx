<!-- <FILE>docs/RESEARCH_DESIGN_EXCELLENCE.md</FILE> - <DESC>Summary of the design-excellence research findings and recommendations</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Consolidate cross-domain design research into a durable narrative artifact in the docs tree</WCTX> -->
<!-- <CLOG>NEW: Add a concise research summary covering the evidence basis, terminal-medium filter, recommendation shortlist, and rationale for the top two new shader families</CLOG> -->

# Design Excellence Research Summary

This document summarizes the broad design research used to identify new,
subtle, professional capability families for `tui-vfx` and GT Design.

## Research lanes

The research drew from:
- product design systems and application UI craft
- motion and fatigue guidance
- transport and wayfinding systems
- architecture and lighting design
- automotive/cockpit interaction patterns
- furniture and industrial design
- mid-century modern and Scandinavian design language

## What repeated evidence suggested

Across these domains, the strongest common themes were:
- hierarchy should often be felt more than seen
- good polish reduces friction before it creates delight
- support chrome should recede
- warmth often comes from material response and restrained contrast
- occasional surprise is welcome, but frequent unnecessary motion creates fatigue

## Terminal-medium filter

All recommendations were filtered through the actual medium:
- truecolor terminal
- cell grid
- one glyph per cell
- fg/bg/modifiers only
- typical terminal aspect ratio
- 60 FPS useful for continuity, but not an excuse for broad choreography

This heavily favors:
- static-first effects
- shell-owned cells over dense text
- color/contrast/depth modulation over glyph churn

## Recommendation shortlist

### Primary
1. `ConcealedLightShader`
2. `DiffusionShader`

### Secondary
3. `AffordanceWakeShader`
4. `WayfindingNodeShader`

### Deferred
- `MaterialContrastShader`
- structured-irregularity / cadence helper

### Policy, not primitive
- rare delight
- fatigue-aware cooldowns
- probabilistic optional surprise
- milestone gating

## Why the top two won

### ConcealedLightShader
A hidden-source architectural light primitive for thresholds, seams, shell
depth, and structural hierarchy.

### DiffusionShader
A material-light primitive for paper, textile, frosted, and lantern-like
surfaces that can carry warmth and softness without noisy motion.

These two were the least duplicative, most terminal-appropriate, and most
cross-theme-useful recommendations.

## Suggested next implementation order
1. `ConcealedLightShader`
2. `DiffusionShader`
3. dogfood each in a tiny number of GT Design recipes
4. keep only the uses that survive the “felt, not seen” test

<!-- <FILE>docs/RESEARCH_DESIGN_EXCELLENCE.md</FILE> - <DESC>Summary of the design-excellence research findings and recommendations</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
