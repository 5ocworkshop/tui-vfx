# ARCH-RESP-TO-PHASE_K2_2

## Acceptance

**Accepted.**

K2.2 lands the missing evidence layer cleanly. The key architectural win is that the player now exposes a stable frame-shaped artifact without pretending it has a full compositor-backed visual surface.

The explicit provenance fields are especially important:

```text
substrate=textGrid
cellSource=rows
styleKnown=false
```

That prevents row-derived cells from being mistaken for true styled/role-aware render output.

## What K2.2 now gives us

We now have four distinct evidence surfaces:

```text
render-recipe       → existing K0 text-row player regression authority
inventory-recipes   → descriptor/effect coverage inventory
migration-gap       → legacy-vs-v3.1 corpus planning report
render-frame        → stable visual-frame JSON substrate
```

Current canonical corpus status remains:

```text
total=16
rendered=10
unsupported=6
errors=0
```

That is the right result for K2.2. It is not a failure that six remain unsupported; it is now a measurable and reportable gap.

## Acceptance notes

K2.2 correctly does **not**:

```text
replace render-recipe
wire the compositor
modify K1
modify recipes
claim visual parity
hide style/role limitations
```

The new `render-frame` report is therefore safe to build on.

The most important limitation to carry forward is:

```text
styleKnown=false
```

This means K2.3 must be careful. Some unsupported primitives may be reducible in a text-grid player, but some may require a true styled-cell substrate before they can honestly become “rendered.”

## Current stack position

Relative to the K-track, we now have:

```text
K0    CLI player established
K1    ratatui GUI path additive, not replacement
K2.0  canonical fixture render/inventory reports
K2.1  migration-gap planning report
K2.2  stable visual-frame evidence report
```

The next phase should not broaden migration yet. It should use K2.2’s evidence to reduce or classify the remaining unsupported primitive adapters.

# Next assignment

Proceed to:

```text
Phase K2.3 — Primitive Adapter Burn-down + Styled-Cell Substrate Decision
```

## Objective

Use the K2.2 visual-frame substrate to reduce the six unsupported canonical primitive fixtures where honest support is possible, and explicitly classify any remaining unsupported primitives that require a styled-cell or compositor-backed substrate.

The goal is not simply to make the unsupported count go to zero. The goal is to avoid false positives.

A primitive should only move from `unsupported` to `rendered` when the player can produce meaningful frame evidence for it.

## Expected starting unsupported set

The implementation should verify this from the existing reports rather than hard-coding it, but the likely unsupported primitive families are:

```text
mask.dissolve
sampler.ripple
style.colorFade
style.baseStyleOverride
shader.linearGradient
shader.borderSweep
```

The source of truth should be the current `inventory-recipes` / `render-frame` unsupported ids.

## Key decision for K2.3

K2.3 should distinguish two adapter classes:

```text
text-grid adapters
styled-cell adapters
```

Text-grid adapters can affect `rows[]` and sparse glyph cells without requiring true style data.

Styled-cell adapters require meaningful foreground/background/modifier/role output. They should not be marked rendered if the player still only has placeholder style fields.

## Required behavior

K2.3 should produce one of these outcomes for each unsupported primitive id:

```text
rendered
  The player can produce deterministic, meaningful frame evidence.

stillUnsupported
  The player cannot honestly render this primitive yet.

blockedByStyledCellSubstrate
  The primitive is inherently style/role/color-oriented and should wait for
  a styled-cell player substrate instead of being faked in text rows.

blockedBySemanticDecision
  The primitive needs a contract/schema/descriptor decision before support.
```

The report should make this explicit.

## Implementation constraints

Do not broaden migration.

Do not mutate:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/**/*
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/**/*
```

unless a very small fixture change is absolutely necessary and separately justified. Prefer using the existing 16 canonical fixtures.

Do not change the default behavior of:

```text
render-recipe
inventory-recipes
migration-gap
render-frame
```

except that counts may improve if primitives become honestly renderable.

Do not import the legacy recipe runtime.

Do not wire the full compositor in K2.3 unless the implementation can do so without collapsing the clean K0/K2 player boundary. The preferred K2.3 approach is still player-local adapter work and capability classification.

Do not claim visual parity.

## Suggested command additions

Keep existing commands, but add a focused adapter gap report if useful:

```bash
cargo run -p tui-vfx-player-cli -- primitive-adapter-gap \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

Recommended schema label:

```text
v3.1.player.primitiveAdapterGap.1
```

If the implementer can expose the classification through existing `inventory-recipes` or `render-frame` without adding a command, that is acceptable, but a dedicated command may make K2.3 review easier.

## Required verification

Run the existing checks:

```bash
cargo fmt --package tui-vfx-player -- --check
cargo fmt --package tui-vfx-player-cli -- --check
cargo clippy -p tui-vfx-player -p tui-vfx-player-cli --all-targets -- -D warnings
cargo test -p tui-vfx-player
cargo test -p tui-vfx-player-cli
```

Re-run K2 reports:

```bash
cargo run -q -p tui-vfx-player-cli -- render-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
  > /tmp/tui-vfx-k23-render-report.json

cargo run -q -p tui-vfx-player-cli -- inventory-recipes \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
  > /tmp/tui-vfx-k23-inventory-report.json

cargo run -q -p tui-vfx-player-cli -- render-frame \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
  > /tmp/tui-vfx-k23-visual-frame-report.json

cargo run -q -p tui-vfx-player-cli -- migration-gap \
  --legacy-root /usr/projects/tui-vfx-recipes/recipes/debug_recipes \
  --v31-root /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  > /tmp/tui-vfx-k23-migration-gap-report.json
```

Then:

```bash
cargo test --workspace
git diff --check
git -C /usr/projects/tui-vfx-recipes status --short -- recipes/debug_recipes recipes/v3.1/debug_recipes
```

Expected recipe status:

```text
# no output
```

## Acceptance criteria

K2.3 is acceptable if it does all of the following:

```text
1. Identifies the current unsupported primitive ids from player evidence.
2. Adds honest adapter support for any primitives that can be represented now.
3. Keeps style/color/role-only primitives unsupported or blocked unless real styled-cell data is produced.
4. Preserves render-recipe behavior.
5. Preserves K2.1 migration-gap behavior.
6. Preserves K2.2 visual-frame report shape.
7. Produces a clear adapter gap/status artifact.
8. Does not mutate recipe corpora.
```

## Deliverables

Expected files or equivalents:

```text
docs/new_kernel/K2_3_PRIMITIVE_ADAPTER_GAP_EVIDENCE.md
docs/new_kernel/PHASE_K2_3_PRIMITIVE_ADAPTER_BURNDOWN_STATUS_MEMO_TO_ARCHITECT.md
docs/VOCABULARY.md
```

The status memo should report:

```text
starting unsupported ids
ending unsupported ids
rendered/unsupported/error counts before and after
which ids were added as adapters
which ids remain blocked
why each blocked id remains blocked
captured report paths
verification commands and results
recipe-root modification check
```

## Draft implementer prompt

```text
Implement Phase K2.3 — Primitive Adapter Burn-down + Styled-Cell Substrate Decision.

Context:
- Implementation repo: /usr/projects/tui-vfx
- Recipe repo: /usr/projects/tui-vfx-recipes
- Canonical fixture root: /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
- Legacy fixture root, evidence only: /usr/projects/tui-vfx-recipes/recipes/debug_recipes
- Descriptor pack: /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json

Goal:
Use the K2.2 render-frame substrate to identify and reduce the remaining unsupported canonical primitive ids where honest player support is possible. Do not fake support for style/color/role primitives while styleKnown=false. Classify blockers explicitly.

Start by running:
  cargo run -q -p tui-vfx-player-cli -- inventory-recipes \
    --descriptor-pack descriptors/v3.1/packs/primitive.json \
    --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes

  cargo run -q -p tui-vfx-player-cli -- render-frame \
    --descriptor-pack descriptors/v3.1/packs/primitive.json \
    --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes

Likely unsupported ids to inspect:
  mask.dissolve
  sampler.ripple
  style.colorFade
  style.baseStyleOverride
  shader.linearGradient
  shader.borderSweep

Do not hard-code that list; confirm it from the report.

For each unsupported id, choose one outcome:
  rendered
  stillUnsupported
  blockedByStyledCellSubstrate
  blockedBySemanticDecision

Rules:
- Reuse existing K0/K2 player paths.
- Preserve render-recipe behavior.
- Preserve inventory-recipes behavior except for improved counts if support is added.
- Preserve migration-gap behavior.
- Preserve render-frame schema v3.1.player.visualFrameReport.1.
- Do not mutate /usr/projects/tui-vfx-recipes/recipes/debug_recipes.
- Do not mutate /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes unless a tiny fixture change is explicitly justified.
- Do not import the legacy recipe runtime.
- Do not claim visual parity.
- Do not mark style/color/role-only primitives as rendered unless the player frame contains real style/role evidence.

If useful, add a new command:
  primitive-adapter-gap

with schema:
  v3.1.player.primitiveAdapterGap.1

Otherwise, add equivalent classification fields to existing evidence output.

Required verification:
  cargo fmt --package tui-vfx-player -- --check
  cargo fmt --package tui-vfx-player-cli -- --check
  cargo clippy -p tui-vfx-player -p tui-vfx-player-cli --all-targets -- -D warnings
  cargo test -p tui-vfx-player
  cargo test -p tui-vfx-player-cli

  cargo run -q -p tui-vfx-player-cli -- render-recipe \
    --descriptor-pack descriptors/v3.1/packs/primitive.json \
    --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
    > /tmp/tui-vfx-k23-render-report.json

  cargo run -q -p tui-vfx-player-cli -- inventory-recipes \
    --descriptor-pack descriptors/v3.1/packs/primitive.json \
    --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
    > /tmp/tui-vfx-k23-inventory-report.json

  cargo run -q -p tui-vfx-player-cli -- render-frame \
    --descriptor-pack descriptors/v3.1/packs/primitive.json \
    --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
    > /tmp/tui-vfx-k23-visual-frame-report.json

  cargo run -q -p tui-vfx-player-cli -- migration-gap \
    --legacy-root /usr/projects/tui-vfx-recipes/recipes/debug_recipes \
    --v31-root /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
    --descriptor-pack descriptors/v3.1/packs/primitive.json \
    > /tmp/tui-vfx-k23-migration-gap-report.json

  cargo test --workspace
  git diff --check
  git -C /usr/projects/tui-vfx-recipes status --short -- recipes/debug_recipes recipes/v3.1/debug_recipes

Deliver:
- code changes in tui-vfx-player / tui-vfx-player-cli as needed
- tests for unsupported-id classification and any newly supported adapter
- docs/VOCABULARY.md update if new terms are introduced
- docs/new_kernel/K2_3_PRIMITIVE_ADAPTER_GAP_EVIDENCE.md
- docs/new_kernel/PHASE_K2_3_PRIMITIVE_ADAPTER_BURNDOWN_STATUS_MEMO_TO_ARCHITECT.md
```

## Why this is the right next step

The migration-gap report says there is broad corpus pressure, but K2.2 says our current player still cannot represent all 16 canonical fixtures. The right move is therefore:

```text
do not migrate wider yet
make current canonical fixtures more truthfully inspectable first
```

After K2.3, we can decide between:

```text
K2.4 — styled-cell player substrate
K2.4 — minimal complex canonical fixture
K2.4 — content-family pilot
K2.4 — ratatui GUI consumption of render-frame evidence
```

The choice should be based on how many of the six unsupported primitives are honestly reducible without a styled-cell substrate.
