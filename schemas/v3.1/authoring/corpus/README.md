# Authoring corpus

Paired recipe corpus for the V3.1 shorthand-design exercise (M2 / M3 of the
authoring-shorthand work plan).

- `canonical/` — post-relaxation V3.1 canonical recipes.
- `shorthand/` — the same intent in proposed authoring shorthand.

Each pair is evidence behind the alias and expansion tables that will land in
`schemas/v3.1/authoring/{transition,sampler,...}/`. Patterns surfaced by diffing
the pairs are summarised in `M4_PATTERNS.md` (delivered with the same drop).

This corpus is a design exercise, not production. Recipes are not wired into the
runtime; they exist so that diffing canonical-vs-shorthand surfaces the rules
the canonicalizer must implement. Rule-of-three applies: a shorthand convention
only earns its place if three or more pairs need it.

## Pairs

10 pairs covering distinct patterns. The remaining 29 read recipes from the
selection (see `M4_PATTERNS.md` for the list) are pattern-confirming evidence
not written as separate pairs.

| Pair | Pattern |
|---|---|
| `baseline` | Minimum-valid recipe (no effects) |
| `filter_dim` | Multi-phase filter with channel-scoped exit variant |
| `mask_blinds` | Multi-phase mask — transition-eligible |
| `sampler_radial_twist` | Single sampler with structured params, transparent bg |
| `style_fade_in_from_canvas` | Tagged-union `from`, canvas-aware fade |
| `wave_with_envelope_signal` | Composed signal expression `multiply(sine, adsr)` |
| `bool_binding_demo` | Event-driven dwell — binding gates phase exit |
| `motion_carrier_orbit_helix` | Motion route with named alias and route params |
| `shadow_bottom_centered_inset` | Shadow attachment on a card |
| `scene_layer_full_stack` | Multi-layer scene: shadow + channel scope + sibling motion |
