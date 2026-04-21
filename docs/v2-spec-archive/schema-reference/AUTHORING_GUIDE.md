<!-- <FILE>docs/scene/AUTHORING_GUIDE.md</FILE> - <DESC>Authoring guide for the recipe scene composer</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Sub-plan B Phase B.2 — document the new scene composer workflow, layer choices, and the canonical Border-driven card shadow recipe pattern.</WCTX> -->
<!-- <CLOG>0.1.0: initial scene authoring guide.</CLOG> -->

# Scene Authoring Guide

## When to use the scene composer

Use `scene` when a recipe needs semantic source layers instead of a single widget-rendered source surface.
The scene composer is the front-stage path for:

- layered text + image compositions
- border-aware cards whose shadows should extrude from `RoleTag::Border`
- authored fallback/procedural content
- custom host composers installed through `SceneComposer`

Recipes that omit `scene` keep the existing widget-first behavior.

## Stock layers

- `text` — authored strings, optional `ContentEffect`, alignment, clip/hide/wrap overflow
- `image` — resolved `SemanticScene` assets; missing assets fall back deterministically
- `card` — fill + border + padded text with role tags split across `Background`, `Border`, and `Text`
- `procedural` — placeholder/fallback path in B.2; the full registry lands in B.3

## Canonical card-shadow pattern

1. Build a `card` layer and let it tag its border cells as `RoleTag::Border`.
2. Feed the composed `SemanticScene` into the normal pipeline.
3. Configure the shadow stage with `ShadowConfig::with_source_region(RoleTag::Border)`.
4. The destination `RoleMap` will receive `RoleTag::Shadow` only where the extruded shadow lands.

This is the core motivating example for the B.2 runtime.

## Custom composer escape hatch

Hosts can provide any `Arc<dyn SceneComposer>` implementation.
The stock composer exists for the common recipe layer set, but the trait is object-safe so a host can replace the front-stage composition strategy entirely.

<!-- <FILE>docs/scene/AUTHORING_GUIDE.md</FILE> - <DESC>Authoring guide for the recipe scene composer</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
