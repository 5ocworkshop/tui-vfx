# Schema / API / Docs Gate

Reproducible commands:

```bash
cargo test -p tui-vfx-contract --test test_schema_generation
UPDATE_SCHEMAS=1 cargo test -p tui-vfx-contract --test test_schema_generation checked_in_contract_schemas_are_current -- --exact
cargo xtask docs generate
cargo xtask docs check
cargo xtask docs api
cargo xtask docs api-check
cargo xtask docs api-validate
cargo xtask audit configschema
```

Canonical recipe validation gate:

```bash
export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}
cargo run -q -p tui-vfx-contract-cli -- validate-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"
```

Template boundary remains: templates are mandatory compile-time composition inputs; runtime/player receives expanded canonical v3.1 recipes with no unresolved template references.
