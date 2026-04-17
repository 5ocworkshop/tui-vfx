# Examples

The `tui-vfx` library ships without inline examples. Instead, 400+ effect recipes
and a full interactive demo browser live in the companion `tui-vfx-recipes` crate.

```bash
cargo install tui-vfx-recipes
cargo run --example demo --release
```

Recipes are kept in a separate crate so that recipe bundles can be versioned,
curated, and distributed independently of the core engine.

There is one source-level example in this repo for the newer low-level
pipeline primitives:

```bash
cargo run -p tui-vfx --example pipeline_effects_showcase
```

It prints representative snapshots for:
- `Materialize` (mask)
- `EdgeGrow` (filter)
- `GlyphCascade` (content effect)
