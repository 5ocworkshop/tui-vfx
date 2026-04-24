<!-- <FILE>docs/design/post-release/INDEX.md</FILE> - <DESC>Index of post-release V3 capability ideas that are intentionally deferred until the core V3 release is stable.</DESC> -->
<!-- <VERS>VERSION: 0.1.2</VERS> -->
<!-- <WCTX>Collect deferred capability specs without mixing post-release creative work into the active V3 release gate.</WCTX> -->
<!-- <CLOG>0.1.2: add weather ambient field spec.</CLOG> -->

# Post-release capability specs

These specs capture useful ideas that should not distract from the active V3
release gate. They are written now so the idea is not lost; implementation waits
until V3 playback, validation, docs, and recipe migration are stable.

## Specs

- [Braille dotfield toolkit plan](braille-dotfield-toolkit-plan.md) — reusable
  2×4 subcell dotfield sources, transforms, and procedural consumers.
- [Glyph actor procedural spec](glyph-actor-procedural-spec.md) — ASCII/Unicode
  stick-figure actors that can move, pose, perch, jump, and compose with V3
  motion routes and effects.
- [Weather ambient field spec](weather-ambient-field-spec.md) — rain, snow,
  wind, fog, lightning, and time-of-day lighting as composable V3 ambient
  ingredients.

## Rules

- Mark each spec as post-release at the top.
- Keep the active V3 master punch list authoritative for release-blocking work.
- Promote a post-release spec into active work only after an explicit owner
  decision or a later planning cycle.
- Reusable signal/math substrate still belongs in `mixed-signals`; terminal
  glyph rendering and effect semantics stay in `tui-vfx` / `tui-vfx-recipes`.

<!-- <FILE>docs/design/post-release/INDEX.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.2</VERS> -->
