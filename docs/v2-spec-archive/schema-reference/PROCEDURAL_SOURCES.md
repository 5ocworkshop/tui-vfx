<!-- <FILE>docs/scene/PROCEDURAL_SOURCES.md</FILE> - <DESC>Authoring guide for stock procedural scene sources</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Sub-plan B Phase B.3 — document the stock procedural source catalog plus the determinism, tiny-rect, and no-interior-mutability contracts.</WCTX> -->
<!-- <CLOG>0.1.0: initial procedural source authoring guide.</CLOG> -->

# Procedural Sources

`scene.procedural` sources are deterministic recipe-owned content generators.
Each source paints directly into an `OwnedGrid` + `RoleMap` pair through the
`ProceduralSource` trait.

## Contract

- **Deterministic:** the same inputs must produce the same output.
- **No interior mutability:** stock sources must not use `OnceCell`, `Mutex`,
  `RwLock`, or `Atomic*`.
- **Tiny-rect safe:** `1×1` output must never panic.
- **Role tagging:** stock sources tag every painted cell as `RoleTag::Procedural`.
- **Reduce-motion freeze:** the explicit first-frame freeze behavior is deferred
  to Sub-plan B Phase B.4b, where manager wiring lands.

## Stock source ids

| Source id | Behavior | Notes |
| --- | --- | --- |
| `braille_spinner` | Four-frame braille spinner | Compact procedural motion |
| `dots_spinner` | Four-frame dots spinner | Uses braille-dot glyphs |
| `line_spinner` | Four-frame `-\\|/` spinner | ASCII-safe fallback |
| `breathe` | Ambient brightness ramp | Reads the active normalized clock |
| `pulse` | Triangular highlight pulse | Peaks at mid-cycle |
| `fallback_procedural` | Unknown-id fallback glyph | Registry-owned; not user-authored |

## Registration

Hosts can register custom sources with:

```rust
use std::sync::Arc;
use tui_vfx_recipes::scene::procedural::{ProceduralRegistry, ProceduralSource};

let mut registry = ProceduralRegistry::with_stock_defaults();
registry.register(Arc::new(MySource::default()));
```

Custom sources should follow the same determinism and tiny-rect rules as the
stock catalog so trace output remains reproducible.

<!-- <FILE>docs/scene/PROCEDURAL_SOURCES.md</FILE> - <DESC>Authoring guide for stock procedural scene sources</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
