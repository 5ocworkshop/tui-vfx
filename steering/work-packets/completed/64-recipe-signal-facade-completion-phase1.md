<!-- <FILE>steering/work-packets/64-recipe-signal-facade-completion-phase1.md</FILE> - <DESC>Phase 1 of recipe-signal facade revised completion plan: add the 15 missing mixed-signals primitives to tui_vfx_recipes::signals so the facade reaches every Serialize/Deserialize-able upstream type</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Recipe-signal facade Phase 1 — close the 15-variant gap surfaced by the 2026-04-27 audit. Catalog grows 43→58. No call-site migration; that is Phase 2.</WCTX> -->
<!-- <CLOG>0.1.0: initial packet — eight stories US-1.1..US-1.8 covering 9 random/RNG noise variants, 3 envelopes, 3 composition operators, plus dispatch/catalog/tests/rustdoc/audit.</CLOG> -->

# 64 — recipe-signal facade completion (Phase 1)

## Task first

Add the 15 missing mixed-signals primitives to `tui_vfx_recipes::signals::VfxRecipeSignalSpec` so the facade reaches every Serialize/Deserialize-able upstream type. Catalog grows 43 → 58.

## Why this matters

The facade exists to be the curated, project-controlled, recipe-JSON deserialization seam in front of `mixed_signals::*`. It currently exposes 43 of 58 reachable primitives. Until the gap closes, Phase 2 (consolidating recipe-side `SignalSpec` access onto the facade) regresses recipe-author capability — authors who today write `{"type": "gaussian_noise", ...}` through the legacy `SignalSpec` path would lose access. Phase 1 is the additive prerequisite that unblocks Phase 2.

Companion: `docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md` v0.4.0 §9.2.

## Success condition

By the end of this packet:

- `VfxRecipeSignalSpec` has 58 variants (15 new); discriminants match upstream `SignalSpec` snake_case names verbatim.
- `VfxIntoRecipeSignal::into_recipe_signal()` has an arm for every new variant; produces a working `Box<dyn Signal>`.
- `vfx_recipe_signal_catalog()` returns 58 entries; `catalog_completeness` test asserts 58.
- 15 round-trip serde tests pass.
- `cargo build --workspace` clean across all four repos (tui-vfx, tui-vfx-recipes, mixed-signals, gt-design).
- `cargo clippy --workspace --all-targets -- -D warnings` clean.
- Zero new `#[allow]` suppressions.
- Per-file metadata envelopes complete; CLOG entries one-line.
- Cross-repo audit recorded in `.omc/progress.txt` per Intention 41.

## Mode

BLOCKER_MODE.

## Task-scope paths for grounding

Read first to ground the work:

- `/usr/projects/tui-vfx/docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md` (especially the 2026-04-27 status snapshot and §9.2)
- `/usr/projects/tui-vfx-recipes/src/signals/cls_vfx_recipe_signal_spec.rs` (the enum to extend)
- `/usr/projects/tui-vfx-recipes/src/signals/fnc_into_recipe_signal.rs` (dispatch to extend)
- `/usr/projects/tui-vfx-recipes/src/signals/fnc_vfx_recipe_signal_catalog.rs` (catalog to extend)
- `/usr/projects/tui-vfx-recipes/src/signals/test_signals.rs` (round-trip tests + completeness assertion)
- `/usr/projects/tui-vfx-recipes/src/signals/oscillators/cls_vfx_recipe_sine_spec.rs` (canonical transparent-wrapper pattern)
- `/usr/projects/tui-vfx-recipes/src/signals/composition/cls_vfx_recipe_add_spec.rs` (canonical inline-struct-with-recursion pattern)
- `/usr/projects/mixed-signals/src/types/signal_spec.rs` (upstream wire format — discriminant names + field shapes)
- `/usr/projects/mixed-signals/src/types/signal_spec/orc_signal_spec_build.rs` (upstream construction patterns to mirror)
- `/usr/projects/mixed-signals/src/random/{cls_seeded_random,cls_spatial_noise,cls_gaussian_noise,cls_poisson_noise,cls_correlated_noise,cls_pink_noise,cls_per_character_noise,cls_student_t_noise,cls_impulse_noise}.rs`
- `/usr/projects/mixed-signals/src/envelopes/{cls_linear,cls_linear_decay,cls_exponential_decay}.rs`
- `/usr/projects/mixed-signals/src/composition/cls_vca_centered.rs`
- `/usr/projects/mixed-signals/src/generators/{cls_phase_accumulator,cls_phase_sine}.rs`

## Exact write scope

Only edit these paths:

- `/usr/projects/tui-vfx-recipes/src/signals/cls_vfx_recipe_signal_spec.rs` (enum extension + CLOG)
- `/usr/projects/tui-vfx-recipes/src/signals/fnc_into_recipe_signal.rs` (15 new arms + CLOG)
- `/usr/projects/tui-vfx-recipes/src/signals/fnc_vfx_recipe_signal_catalog.rs` (15 new entries + CLOG; catalog count comment 43→58)
- `/usr/projects/tui-vfx-recipes/src/signals/mod.rs` (add `pub mod random;` and `pub mod envelopes;`; re-exports + CLOG)
- `/usr/projects/tui-vfx-recipes/src/signals/test_signals.rs` (15 round-trip tests + bump catalog assertion 43→58 + CLOG)
- New family directories:
  - `/usr/projects/tui-vfx-recipes/src/signals/random/mod.rs`
  - `/usr/projects/tui-vfx-recipes/src/signals/random/cls_vfx_recipe_seeded_random_spec.rs`
  - `/usr/projects/tui-vfx-recipes/src/signals/random/cls_vfx_recipe_spatial_noise_spec.rs`
  - `/usr/projects/tui-vfx-recipes/src/signals/random/cls_vfx_recipe_gaussian_noise_spec.rs`
  - `/usr/projects/tui-vfx-recipes/src/signals/random/cls_vfx_recipe_poisson_noise_spec.rs`
  - `/usr/projects/tui-vfx-recipes/src/signals/random/cls_vfx_recipe_correlated_noise_spec.rs`
  - `/usr/projects/tui-vfx-recipes/src/signals/random/cls_vfx_recipe_pink_noise_spec.rs`
  - `/usr/projects/tui-vfx-recipes/src/signals/random/cls_vfx_recipe_per_character_noise_spec.rs`
  - `/usr/projects/tui-vfx-recipes/src/signals/random/cls_vfx_recipe_student_t_noise_spec.rs`
  - `/usr/projects/tui-vfx-recipes/src/signals/random/cls_vfx_recipe_impulse_noise_spec.rs`
- New under existing `signals/envelopes/` (create directory + mod.rs):
  - `/usr/projects/tui-vfx-recipes/src/signals/envelopes/mod.rs`
  - `/usr/projects/tui-vfx-recipes/src/signals/envelopes/cls_vfx_recipe_linear_envelope_spec.rs`
  - `/usr/projects/tui-vfx-recipes/src/signals/envelopes/cls_vfx_recipe_linear_decay_spec.rs`
  - `/usr/projects/tui-vfx-recipes/src/signals/envelopes/cls_vfx_recipe_exponential_decay_spec.rs`
- New under existing `signals/composition/`:
  - `/usr/projects/tui-vfx-recipes/src/signals/composition/cls_vfx_recipe_vca_centered_spec.rs`
  - `/usr/projects/tui-vfx-recipes/src/signals/composition/cls_vfx_recipe_phase_accumulator_spec.rs`
  - `/usr/projects/tui-vfx-recipes/src/signals/composition/cls_vfx_recipe_phase_sine_spec.rs`
  - update `/usr/projects/tui-vfx-recipes/src/signals/composition/mod.rs` (re-exports + CLOG)
- Note: existing `oscillators/cls_vfx_recipe_adsr_spec.rs` and `oscillators/cls_vfx_recipe_impact_spec.rs` will move to `envelopes/` only as a follow-up cleanup if scope permits; not required for this packet. Leave them in `oscillators/` for now to keep blast radius bounded.
- `/usr/projects/tui-vfx-recipes/.omc/progress.txt` (or `progress.txt` at repo root if present) — record audit evidence at packet close.

## Explicit out of scope

Do not widen into:

- `steering/ORCHESTRATION.md` or any leader-only orchestration policy
- `crates/tui-vfx-core/`, `crates/tui-vfx-compositor/`, `crates/tui-vfx-types/` or any engine-side type — engine field types stay engine-native
- `src/v3/authoring/enum_v3_loopback_value.rs` — Phase 2 owns this migration
- `src/v3/compile/fnc_build_composition_spec_from_compiled_plan.rs` — already on facade; no change
- `BindableValue` — Phase 2 owns this; Decision 2A keeps it engine-native
- Any `mixed_signals` change — Intention 9; not this packet
- `WeightedMix` exposure — non-Serialize upstream type; explicitly deferred
- Recipe corpus migration — purely additive packet; existing recipes already work
- `docs/generated/SIGNALS_REFERENCE.md` — autogen target; will regenerate from rustdoc + signals.toml when the autogen runs (not part of this packet's hand-written scope)
- Adding a `signals.toml` overlay entry beyond what is strictly required for the catalog completeness; the autogen pipeline picks up new variants from rustdoc + the catalog; targeted toml entries are a follow-up if reference doc readability requires them

## Must-read docs in order

1. `/usr/projects/tui-vfx/steering/INTENTIONS.md` (especially Intentions 8, 9, 23, 24, 25, 40, 41, 42, 43)
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md` (if present)
3. `/usr/projects/tui-vfx/steering/OFPF-TOOLS.md`
4. `/usr/projects/tui-vfx/docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md` (the v0.4.0 status snapshot and §9.2)
5. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`

## Repo-boundary guardrails

- `mixed-signals` owns the primitive types. This packet does not change them.
- `tui-vfx` owns engine semantics. This packet does not touch any engine crate.
- `tui-vfx-recipes` owns recipe truth. This packet adds module content inside `tui-vfx-recipes::signals`.
- Per Intention 9: if any of the 15 primitives is missing a needed shape (it is not, per audit), the answer is to extend `mixed-signals` upstream first — not to invent it locally.

## Pipeline-touch definition of done

This packet is **not** pipeline-touch. The facade lives at the recipe deserialization layer and produces engine-native types at the seam; no shader, filter, mask, sampler, motion route, shadow, scope, or binding code is touched. The pipeline-touch checklist therefore does not apply except for:

- Update rustdocs on every public item added (Intention 12).
- Align comments and rustdoc with canonical V3 vocabulary (Intention 32).
- Keep `<CLOG>` entries one-line per touched file (per memory rule).

## Stories

### US-1.1 — Random / RNG noise: 8 transparent wrappers

Add 8 new files under `signals/random/`. Each is a `#[serde(transparent)]` wrapper over the upstream type, mirroring the canonical pattern in `oscillators/cls_vfx_recipe_sine_spec.rs`.

Variants: `seeded_random`, `spatial_noise`, `gaussian_noise`, `poisson_noise`, `correlated_noise`, `pink_noise`, `per_character_noise`, `student_t_noise`.

Acceptance:

- 8 `.rs` files with `pub struct VfxRecipe<Name>Spec(pub mixed_signals::random::<Name>);` + Serialize/Deserialize + Debug + Clone + Copy + PartialEq.
- Each file carries a metadata envelope.
- `random/mod.rs` re-exports all 8 with rustdoc.
- Wire format matches the upstream `SignalSpec` shape (verified by reading upstream struct fields against `SignalSpec` declaration in `mixed-signals/src/types/signal_spec.rs`).
- Serde derive bypasses upstream `::new()` validation. This is intentional and matches the upstream `Deserialize` behavior; do not add custom validation.

### US-1.2 — `impulse_noise`: inline struct (wire-format mismatch mitigation)

`mixed_signals::random::ImpulseNoise` has 6 fields; `SignalSpec::ImpulseNoise` exposes only 3 (`{seed, rate_hz, impulse_width}`). A transparent wrapper would expose `bucket_size`, `amplitude`, `offset` and break SignalSpec-compatible JSON.

Implement as an inline struct mirroring the SignalSpec shape:

```rust
pub struct VfxRecipeImpulseNoiseSpec {
    pub seed: u64,
    pub rate_hz: f32,
    pub impulse_width: f32,
}
```

Build via `ImpulseNoise::with_width(rate_hz, seed, impulse_width)` in the dispatch arm.

Acceptance:

- File carries metadata envelope, full rustdoc explaining why the wire-format mismatch warranted an inline shape (link to this packet).
- Wire format: `{"type": "impulse_noise", "seed": <u64>, "rate_hz": <f32>, "impulse_width": <f32>}`.
- `into_recipe_signal()` arm calls `ImpulseNoise::with_width(spec.rate_hz, spec.seed, spec.impulse_width)`.

### US-1.3 — Envelopes: 3 transparent wrappers

Create new `signals/envelopes/` family directory. Add 3 transparent wrappers over `mixed_signals::envelopes::{LinearEnvelope, LinearDecay, ExponentialDecay}`.

Note: `SignalSpec` uses `LinearEnvelope { attack, release, peak }` but the upstream `LinearEnvelope` struct has the same three public fields, so the transparent wrapper preserves SignalSpec wire-format compatibility for `linear_envelope`. `linear_decay` and `exponential_decay` have no SignalSpec wire format (not yet in `SignalSpec`); the facade's wire format is the upstream struct's natural serde shape.

Acceptance:

- `envelopes/mod.rs` with rustdoc + re-exports + metadata envelope.
- 3 wrapper files; metadata envelopes; one-line rustdoc.
- `signals/mod.rs` adds `pub mod envelopes;` re-exporting the three wrappers.

### US-1.4 — Composition: 3 inline structs with `Box<VfxRecipeSignalSpec>` recursion

`VcaCentered`, `PhaseAccumulator`, `PhaseSine` are generic over their child `Signal` type; transparent wrapping is not possible. Mirror the existing `Add` / `Mix` pattern in `composition/cls_vfx_recipe_add_spec.rs`:

```rust
pub struct VfxRecipeVcaCenteredSpec {
    pub carrier: Box<VfxRecipeSignalSpec>,
    pub amplitude: Box<VfxRecipeSignalSpec>,
}

pub struct VfxRecipePhaseAccumulatorSpec {
    pub frequency: Box<VfxRecipeSignalSpec>,
    #[serde(default)]
    pub initial_phase: f32,
}

pub struct VfxRecipePhaseSineSpec {
    pub phase: Box<VfxRecipeSignalSpec>,
}
```

`into_recipe_signal()` arms recurse the children via `.into_recipe_signal()` and pass the resulting `Box<dyn Signal>` to the upstream constructor.

Acceptance:

- 3 files in `composition/`; metadata envelopes; rustdoc explaining the recursion pattern.
- `composition/mod.rs` re-exports them; metadata envelope and CLOG bumped.
- Wire format matches `SignalSpec::VcaCentered`, `SignalSpec::PhaseAccumulator`, `SignalSpec::PhaseSine`.

### US-1.5 — Wire enum + dispatch + catalog

In `cls_vfx_recipe_signal_spec.rs`:

- Add 15 new enum arms in the appropriate sections (Random/RNG noise, Envelopes, Composition).
- Each arm wraps the per-family spec struct.
- Bump VERS minor; update CLOG to one line.

In `fnc_into_recipe_signal.rs`:

- Add 15 new match arms producing `Box<dyn Signal>`.
- Use the patterns from US-1.1..US-1.4.
- Bump VERS minor; update CLOG to one line.

In `fnc_vfx_recipe_signal_catalog.rs`:

- Add 15 new `VfxRecipeSignalMeta` entries.
- Family fields: `"noise"` (random/RNG go in noise family for catalog; rationale: that is how recipe authors find them — they search by behavioral category, not by mixed-signals module), `"envelope"`, `"composition"`.
- `in_core_12: false` for all 15.
- Update the catalog-count comment from 43 → 58.
- Bump VERS minor; update CLOG to one line.

In `signals/mod.rs`:

- `pub mod random;` + `pub mod envelopes;` declarations.
- Re-exports through the facade's public surface.
- Bump VERS minor; update CLOG to one line.

Acceptance:

- `cargo build -p tui-vfx-recipes` compiles cleanly.
- All re-exports reachable from `tui_vfx_recipes::signals::*`.

### US-1.6 — Round-trip tests + bump completeness assertion

In `test_signals.rs`:

- Add 15 new `#[test]` functions, one per new variant. Each round-trips a representative JSON payload through `serde_json::from_str::<VfxRecipeSignalSpec>` and `serde_json::to_string`. Assert structural equality.
- For composition variants (US-1.4), include a nested-child case (e.g. `VcaCentered { carrier: Sine, amplitude: Ramp }`).
- Bump `catalog_completeness` assertion from 43 to 58 (and the literal-count comment).
- Add a `behavior_smoke` test for at least one new variant per family that constructs the signal via `into_recipe_signal()` and calls `.sample(0.5)` — confirms dispatch produces a working `Box<dyn Signal>`.

Acceptance:

- `cargo test -p tui-vfx-recipes signals::` passes 15 new tests.
- `catalog_completeness` test passes with 58.

### US-1.7 — Rustdoc + metadata-envelope hygiene

For every new file and every modified file:

- Top-of-file metadata envelope (`<FILE>` / `<DESC>` / `<VERS>` / `<WCTX>` / `<CLOG>`).
- Bottom-of-file `END OF VERSION` envelope.
- `<CLOG>` is **one or two short lines** describing the latest change only — git holds running history.
- Rustdoc on every public item: type, every variant, every field. Brief, specific. No marketing voice (Intention writing-style).
- Cross-link new variants to upstream `mixed_signals::*` types via intra-doc links where feasible.

Acceptance:

- `cargo doc -p tui-vfx-recipes --no-deps` succeeds with zero warnings on the touched files.
- A spot-check on three new files (one per family) confirms the rustdoc naming + cross-links.

### US-1.8 — Verification + cross-repo audit (Intention 41)

Run in this order:

1. `cargo build -p tui-vfx-recipes` (in `/usr/projects/tui-vfx-recipes`).
2. `cargo test -p tui-vfx-recipes` (in `/usr/projects/tui-vfx-recipes`).
3. `cargo build --workspace` (in `/usr/projects/tui-vfx-recipes`).
4. `cargo clippy --workspace --all-targets -- -D warnings` (in `/usr/projects/tui-vfx-recipes`).
5. `cargo build --workspace` (in `/usr/projects/tui-vfx`).
6. `cargo build --workspace` (in `/usr/projects/gt-design`).
7. `cargo build --workspace` (in `/usr/projects/mixed-signals`).
8. Cross-repo discriminant audit per Intention 41:

```bash
for repo in /usr/projects/tui-vfx /usr/projects/tui-vfx-recipes /usr/projects/mixed-signals /usr/projects/gt-design; do
  for sig in seeded_random spatial_noise gaussian_noise poisson_noise correlated_noise pink_noise per_character_noise student_t_noise impulse_noise linear_envelope linear_decay exponential_decay vca_centered phase_accumulator phase_sine; do
    count=$(rg -l "\"type\": \"$sig\"" "$repo" 2>/dev/null | wc -l)
    echo "$repo $sig: $count"
  done
done
```

9. Record the audit + build outputs in progress.txt. Note any unexpected hits (per audit pre-condition, expected hits: 0 across all 4 repos for 14 of 15 discriminants; 1 hit for `spatial_noise` in `tui-vfx-recipes`).

Acceptance:

- All 7 build/test commands pass.
- Cross-repo audit shows expected hit counts.
- progress.txt updated with timestamps + per-command outcomes.
- No `#[allow]` suppressions added (verified by `rg -n '#\[allow|#!\[allow' src/signals/`).

## Test-shape requirements

Schema/parser behavior changes; canonical fixture coverage required:

- **Accepted minimal form** — bare-minimum JSON for each new variant deserializes correctly.
- **Rejected unknown nested fields** — verify behavior (likely lenient at root via the existing facade pattern; document outcome in test comments).
- **Defaulted omitted fields** — for variants with `#[serde(default)]`, omit the field and assert default is applied. Particularly relevant for `phase_accumulator` (`initial_phase` default).
- **Validation boundary errors** — inappropriate where upstream types do not validate at deserialize time (most cases). Document this explicitly in test comments referencing this packet.
- **Typed propagation into downstream IR** — `into_recipe_signal()` produces a `Box<dyn Signal>` whose `.sample(0.5)` returns a finite value (smoke test from US-1.6).

## Hot-path watchpoints

- The new variant arms in `into_recipe_signal()` are not on a hot path; they run once per recipe load. Box allocation is acceptable.
- Composition variants (US-1.4) recurse during construction only, not during evaluation. The recursion depth is bounded by recipe author intent; document `mix(mix(mix(...)))` performance considerations in proposal §6 question 3 (cap on signal-expression depth) — out of scope for this packet, just note it.

## Verification required

```bash
# In tui-vfx-recipes:
cd /usr/projects/tui-vfx-recipes
cargo build -p tui-vfx-recipes
cargo test -p tui-vfx-recipes
cargo build --workspace
cargo clippy --workspace --all-targets -- -D warnings

# Cross-repo build (Intention 41):
cd /usr/projects/tui-vfx && cargo build --workspace
cd /usr/projects/gt-design && cargo build --workspace
cd /usr/projects/mixed-signals && cargo build --workspace

# No-landmines pre-commit check:
cd /usr/projects/tui-vfx-recipes
git diff --cached | rg '^\+.*#\[allow|^\+.*#!\[allow' || echo "no new #[allow] suppressions"

# Discriminant audit (script in US-1.8 above)
```

If any command fails, report the exact failure, classify it (in-scope failure / expected downstream fallout / blocker), and stop until the leader confirms the fix path.

## Pre-commit write-scope guard (Intention 40 §5)

Before `git commit`:

```bash
git diff --cached --name-only
```

The output must list **only** the paths in "Exact write scope" above. Unstage any sweep-up with `git restore --staged <path>`. Stage by explicit path; never `git add -A`.

## No-landmines pre-commit check (Intention 40)

```bash
git diff --cached | rg '^\+.*#\[allow|^\+.*#!\[allow' || echo "no new #[allow] suppressions"
```

If any new `#[allow]` lines surface, fix the root cause or set explicit project-level policy in `clippy.toml`. Per-site `#[allow]` for clippy is a landmine.

## Reporting contract

Final report must include:

- Docs-read confirmation (the must-read docs)
- 3 reflection bullets (what worked, what surprised, what to watch in Phase 2)
- Exact task-scope paths read
- Exact files changed (full paths)
- Exact commands run + pass/fail per command
- Cross-repo audit table (4 repos × 15 discriminants per US-1.8)
- progress.txt update summary
- Any blockers / handoff notes for Phase 2

## File metadata discipline

Every touched file:

- `<CLOG>` entry one or two short lines covering the latest change only.
- `<WCTX>` updated only if the file's role changed; otherwise stable.
- `<VERS>` bumped per OFPF policy: PATCH for internal edits, MINOR for new exports/non-breaking additions.

## Closing task reminder

**Task:** add 15 missing mixed-signals primitives to `tui_vfx_recipes::signals::VfxRecipeSignalSpec`; catalog 43 → 58; no engine changes; no call-site migration.

**Do not widen into:** engine crates, `V3LoopbackValue`, `BindableValue`, recipe corpus migration, mixed-signals upstream, or `WeightedMix`.

<!-- <FILE>steering/work-packets/64-recipe-signal-facade-completion-phase1.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
