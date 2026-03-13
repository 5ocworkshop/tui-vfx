# tui-vfx justfile
# Run `just --list` to see all available commands

# Default recipe: show available commands
default:
    @just --list

# ═══════════════════════════════════════════════════════════════════════════════
# DOCUMENTATION GENERATION
# ═══════════════════════════════════════════════════════════════════════════════
#
# The documentation pipeline merges two sources:
#   1. Rustdoc comments in source code (technical details)
#   2. docs/templates/capabilities.toml (editorial/semantic details)
#
# These combine to generate:
#   - docs/generated/CAPABILITIES.md (human-readable reference)
#   - docs/generated/ai-context.md (condensed AI prompt)
#   - docs/generated/capabilities.json (machine-readable)
#   - docs/generated/effect_schemas.json (full ConfigSchema metadata)
#
# See docs/design/CAPABILITY_MANIFEST_*.md for architecture details.
# ═══════════════════════════════════════════════════════════════════════════════

# Generate all documentation from rustdoc + TOML sources
docs-generate:
    @echo "Generating documentation from rustdoc + templates/capabilities.toml..."
    cargo xtask docs generate

# Check that generated docs are up-to-date (for CI)
# Fails if regenerating would change any files
docs-check:
    @echo "Checking documentation freshness..."
    cargo xtask docs check

# Generate only the AI context prompt (condensed ~50 line version)
docs-ai-context:
    @echo "Generating AI context prompt..."
    cargo xtask docs ai-context

# Generate only CAPABILITIES.md
docs-markdown:
    @echo "Generating CAPABILITIES.md..."
    cargo xtask docs markdown

# Extract rustdoc JSON (requires nightly)
# This is called internally by docs-generate, but can be run standalone for debugging
docs-rustdoc-json:
    @echo "Extracting rustdoc JSON (requires nightly)..."
    cargo +nightly rustdoc \
        -p tui-vfx-compositor \
        -p tui-vfx-style \
        -p tui-vfx-content \
        -p tui-vfx-shadow \
        -- -Z unstable-options --output-format json
    @echo "JSON output in target/doc/*.json"

# Validate capabilities.toml against code (ensures all variants documented)
docs-validate:
    @echo "Validating templates/capabilities.toml coverage..."
    cargo xtask docs validate

# Generate TOML stubs for undocumented effects (prints to stdout)
docs-scaffold:
    @echo "Scaffolding TOML stubs for undocumented effects..."
    cargo xtask docs scaffold

# Generate TOML stubs and write directly to capabilities.toml
docs-scaffold-write:
    @echo "Writing TOML stubs to templates/capabilities.toml..."
    cargo xtask docs scaffold --write

# Validate recipes against capabilities.json (pass --recipes-dir)
recipes-validate recipes_dir:
    @echo "Validating recipes..."
    cargo xtask recipes validate --recipes-dir {{recipes_dir}}

# ═══════════════════════════════════════════════════════════════════════════════
# API DOCUMENTATION GENERATION
# ═══════════════════════════════════════════════════════════════════════════════
#
# The API documentation pipeline merges two sources:
#   1. Code metadata (ConfigSchema, syn parsing, runtime introspection)
#   2. docs/templates/api_docs.toml (editorial: structure, examples, usage notes)
#
# This generates docs/generated/API.md (complete technical API reference).
#
# QA baseline: docs/API_HAND.md (original hand-maintained version)
# See docs/design/API_DOC_GENERATION_PLAN.md for architecture details.
# ═══════════════════════════════════════════════════════════════════════════════

# Generate API.md from code + api_docs.toml
docs-api:
    @echo "Generating API.md from code + templates/api_docs.toml..."
    cargo xtask docs api

# Check that API.md is up-to-date (for CI)
docs-api-check:
    @echo "Checking API.md freshness..."
    cargo xtask docs api-check

# Validate api_docs.toml against code (ensures all public types documented)
docs-api-validate:
    @echo "Validating templates/api_docs.toml coverage..."
    cargo xtask docs api-validate

# Generate TOML stubs for undocumented API types (prints to stdout)
docs-api-scaffold:
    @echo "Scaffolding templates/api_docs.toml stubs for undocumented types..."
    cargo xtask docs api-scaffold

# Generate TOML stubs and write directly to api_docs.toml
docs-api-scaffold-write:
    @echo "Writing TOML stubs to templates/api_docs.toml..."
    cargo xtask docs api-scaffold --write

# Diff generated API.md against hand-maintained baseline (QA check)
docs-api-diff:
    @echo "Comparing generated API.md against API_HAND.md..."
    @diff -u docs/API_HAND.md docs/generated/API.md || echo "Files differ (expected during development)"

# ═══════════════════════════════════════════════════════════════════════════════
# ALL DOCUMENTATION (COMBINED)
# ═══════════════════════════════════════════════════════════════════════════════

# Generate all documentation (CAPABILITIES.md + API.md + ai-context.md + JSON)
docs-all:
    @echo "Generating all documentation..."
    cargo xtask docs generate
    @echo "✓ All documentation generated"

# Check all documentation is up-to-date (for CI)
docs-all-check:
    @echo "Checking all documentation freshness..."
    cargo xtask docs check
    @echo "✓ All documentation up-to-date"

# Validate all TOML manifests against code
docs-all-validate:
    @echo "Validating all documentation manifests..."
    cargo xtask docs validate
    cargo xtask docs api-validate
    @echo "✓ All manifests valid"

# ═══════════════════════════════════════════════════════════════════════════════
# STANDARD DEVELOPMENT
# ═══════════════════════════════════════════════════════════════════════════════

# Build all crates
build:
    cargo build --workspace

# Build in release mode
build-release:
    cargo build --workspace --release

# Run all tests
test:
    cargo test --workspace

# Run tests with output shown
test-verbose:
    cargo test --workspace -- --nocapture

# Run clippy lints
lint:
    cargo clippy --workspace --all-targets -- -D warnings

# Format code
fmt:
    cargo fmt --all

# Check formatting without modifying
fmt-check:
    cargo fmt --all -- --check

# Run all checks (fmt, lint, test, docs-all-check)
check-all: fmt-check lint test docs-all-check
    @echo "All checks passed!"

# ═══════════════════════════════════════════════════════════════════════════════
# DOCUMENTATION (STANDARD RUSTDOC)
# ═══════════════════════════════════════════════════════════════════════════════

# Generate rustdoc HTML documentation
doc:
    cargo doc --workspace --no-deps

# Generate and open rustdoc in browser
doc-open:
    cargo doc --workspace --no-deps --open

# ═══════════════════════════════════════════════════════════════════════════════
# EXAMPLES
# ═══════════════════════════════════════════════════════════════════════════════

# Run a specific example (usage: just example <name>)
example name:
    cargo run --example {{name}}

# List available examples
examples:
    @echo "Available examples:"
    @find examples -name "*.rs" -exec basename {} .rs \; 2>/dev/null || echo "No examples found"

# ═══════════════════════════════════════════════════════════════════════════════
# MAINTENANCE
# ═══════════════════════════════════════════════════════════════════════════════

# Clean build artifacts
clean:
    cargo clean

# Update dependencies
update:
    cargo update

# Show outdated dependencies
outdated:
    cargo outdated

# ═══════════════════════════════════════════════════════════════════════════════
# XTASK (BUILD TOOLING)
# ═══════════════════════════════════════════════════════════════════════════════

# Run xtask with arbitrary arguments (usage: just xtask <args>)
xtask *args:
    cargo xtask {{args}}

# ═══════════════════════════════════════════════════════════════════════════════
# DRAMATIC COLOR-SHADOW ROLLOUT
# ═══════════════════════════════════════════════════════════════════════════════
#
# Phase-specific validation recipes for the dramatic color-graded shadow feature.
# See gt-design/plans/tui-vfx-v1-dramatic-color-shadow-plan.md for the master plan.
# ═══════════════════════════════════════════════════════════════════════════════

# Phase 0 red: shadow contract tests (should fail before implementation)
dramatic-shadow-phase0-red:
    cargo test -p tui-vfx-shadow shadow_config_grade_underlying_serde_round_trip -- --nocapture
    cargo test -p tui-vfx-shadow shadow_grade_config_dramatic_defaults_are_visible -- --nocapture
    cargo test -p tui-vfx-compositor shadow_spec_preserves_grade_underlying_config -- --nocapture

# Phase 0 green: shadow contract + lib tests
dramatic-shadow-phase0-green:
    cargo test -p tui-vfx-shadow --lib -- --nocapture
    cargo test -p tui-vfx-compositor --lib -- --nocapture

# Phase 0 docs: clippy + rustdoc + format
dramatic-shadow-phase0-docs:
    cargo clippy -p tui-vfx-shadow --lib --no-deps -- -D warnings
    RUSTDOCFLAGS="-D warnings" cargo doc -p tui-vfx-shadow --no-deps
    cargo clippy -p tui-vfx-compositor --lib --no-deps -- -D warnings
    RUSTDOCFLAGS="-D warnings" cargo doc -p tui-vfx-compositor --no-deps
    cargo fmt --all -- --check

# Phase 1 red: grade-underlying integration tests (should fail before implementation)
dramatic-shadow-phase1-red:
    cargo test -p tui-vfx-compositor --test test_pipeline test_shadow_grade_underlying_preserves_destination_glyphs -- --nocapture
    cargo test -p tui-vfx-compositor --test test_pipeline test_shadow_grade_underlying_preserves_destination_modifiers -- --nocapture
    cargo test -p tui-vfx-compositor --test test_pipeline test_shadow_grade_underlying_is_visibly_dramatic -- --nocapture

# Phase 1 green: full compositor test suite
dramatic-shadow-phase1-green:
    cargo test -p tui-vfx-compositor --test test_pipeline -- --nocapture
    cargo test -p tui-vfx-compositor --test test_pipeline test_render_pipeline_with_spec -- --nocapture
    cargo test -p tui-vfx-shadow --lib -- --nocapture

# Phase 1 docs: clippy + rustdoc
dramatic-shadow-phase1-docs:
    cargo clippy -p tui-vfx-compositor --lib --no-deps -- -D warnings
    RUSTDOCFLAGS="-D warnings" cargo doc -p tui-vfx-compositor --no-deps

# Phase 2 red: animation and gradient tests (should fail before implementation)
dramatic-shadow-phase2-red:
    cargo test -p tui-vfx-compositor --test test_pipeline test_shadow_grade_underlying_progress_controls_visibility -- --nocapture
    cargo test -p tui-vfx-compositor --test test_pipeline test_shadow_grade_underlying_gradient_softens_penumbra -- --nocapture

# Phase 2 green: full test suite + spec equivalence (skip pre-existing test_shadow_extends_render_area)
dramatic-shadow-phase2-green:
    cargo test -p tui-vfx-compositor --test test_pipeline -- --nocapture --skip test_shadow_extends_render_area
    cargo test -p tui-vfx-compositor --test test_pipeline test_render_pipeline_with_spec -- --nocapture

# Phase 2 docs: both crates clippy + rustdoc
dramatic-shadow-phase2-docs:
    cargo clippy -p tui-vfx-shadow --lib --no-deps -- -D warnings
    RUSTDOCFLAGS="-D warnings" cargo doc -p tui-vfx-shadow --no-deps
    cargo clippy -p tui-vfx-compositor --lib --no-deps -- -D warnings
    RUSTDOCFLAGS="-D warnings" cargo doc -p tui-vfx-compositor --no-deps

# Full quality gate: all phases green + docs + format + workspace test
dramatic-shadow-full-quality:
    cargo fmt --all -- --check
    cargo clippy -p tui-vfx-shadow --lib --no-deps -- -D warnings
    cargo clippy -p tui-vfx-compositor --lib --no-deps -- -D warnings
    RUSTDOCFLAGS="-D warnings" cargo doc -p tui-vfx-shadow --no-deps
    RUSTDOCFLAGS="-D warnings" cargo doc -p tui-vfx-compositor --no-deps
    cargo test -p tui-vfx-shadow --lib -- --nocapture
    cargo test -p tui-vfx-compositor --lib -- --nocapture
    cargo test -p tui-vfx-compositor --test test_pipeline -- --nocapture --skip test_shadow_extends_render_area

# ═══════════════════════════════════════════════════════════════════════════════
# CI SIMULATION
# ═══════════════════════════════════════════════════════════════════════════════

# Run the full CI pipeline locally
ci: fmt-check lint test docs-all-check
    @echo ""
    @echo "══════════════════════════════════════════"
    @echo "  CI simulation passed!"
    @echo "══════════════════════════════════════════"
