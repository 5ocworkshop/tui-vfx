<!-- <FILE>steering/work-packets/67-engine-api-signals-doc-relabel.md</FILE> - <DESC>Rename docs/generated/SIGNALS_REFERENCE.md → API_SIGNALS_REFERENCE.md and rescope it to the engine / direct-API audience; purge post-Phase-A stale content (the "physics primitives are not reachable through SignalSpec" callout and per-row "(parallel channel)" annotations); add the engine-side "wrong door" preamble cross-linking the recipe-side reference.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>2026-04-27 post-Phase-A audit + tui-vfx-recipes Packet 01 split: the existing SIGNALS_REFERENCE.md is preambled "Recipe-author cheatsheet" but cannot reach the recipe-side catalog due to dep direction. With the recipe-side generator landing in tui-vfx-recipes (Packet 01), this packet narrows the engine-side doc to its real audience (direct-API consumers) and drops content that Phase A invalidated.</WCTX> -->
<!-- <CLOG>0.1.0: initial packet — four stories US-1.1..US-1.4 covering rename, preamble + heading rescope, stale-content purge (physics callout + parallel-channel annotations + is_parallel_channel flag), signals.toml rescope to engine-only, regenerate + commit + verification.</CLOG> -->

# 67 — Engine / API signals doc relabel + post-Phase-A purge

## Task first

Rename `docs/generated/SIGNALS_REFERENCE.md` to `docs/generated/API_SIGNALS_REFERENCE.md`. Rescope it to the engine / direct-API audience with a "wrong door" preamble cross-linking the recipe-side reference. Drop the post-Phase-A stale claims (physics-primitives "parallel channel only" callout + per-row annotations + the `is_parallel_channel` extractor flag where it conflates "not in SignalSpec today" with "physics"). Re-scope `docs/templates/signals.toml` to engine-API content (move recipe-author copy out — the recipe repo now owns that).

This is the engine-side half of a paired packet. The recipe-side half is `tui-vfx-recipes/steering/work-packets/01-recipe-signals-doc-generator.md`.

## Why this matters

Today's `SIGNALS_REFERENCE.md`:
- Is preambled "Recipe-author cheatsheet" but is generated from a tui-vfx xtask that cannot see the `tui-vfx-recipes` recipe-side catalog (dep direction: `tui-vfx-recipes → tui-vfx`, not the reverse).
- Contains content that is stale post-mixed-signals Phase A: a top-of-file callout asserts physics primitives "are not reachable through SignalSpec JSON today"; the Core 12 table annotates row 8 "spring (parallel channel)"; the extractor sets `is_parallel_channel: bool = true` on physics types. All three are now false — `mixed_signals::types::SignalSpec` covers `Spring`, `Bounce`, `Pendulum`, `Projectile`, `Orbit`, `Decay`, `Attractor`, `LinearDecay`, `ExponentialDecay`.
- Mixes engine-API content with recipe-author overlay copy from `docs/templates/signals.toml`.

After Packet 01 lands the recipe-side generator (`docs/generated/RECIPE_SIGNALS_REFERENCE.md` in `tui-vfx-recipes`), this doc's natural audience is engine / API-direct consumers (`tui-vfx-content`, `tui-vfx-style`, `gt-design` motion runtime, escape-hatch users). Renaming + rescoping makes the boundary obvious and removes the stale content in one pass.

## Success condition

By the end of this packet:

- `docs/generated/SIGNALS_REFERENCE.md` is renamed to `docs/generated/API_SIGNALS_REFERENCE.md`.
- The H1 heading reads `Engine / Direct-API Signals Reference` (no longer "tui-vfx Signals Reference").
- The engine-side "wrong door" preamble (text below) appears at the top of the file, before the H1.
- The "Note on physics signals … parallel motion-spec channel" callout is removed.
- The Core 12 table no longer shows `(parallel channel)` annotations on physics rows; the wire format is the actual SignalSpec form.
- The extractor's `is_parallel_channel: bool` flag is either removed or repurposed (see US-1.2 — the flag conflated "not yet in SignalSpec" with "physics", and Phase A invalidated the conflation).
- `docs/templates/signals.toml` is rescoped to engine-API-focused content; recipe-author copy migrated out (Packet 01 brings its own equivalent in `tui-vfx-recipes/docs/templates/signals.toml`).
- `cargo xtask docs signals` regenerates cleanly; output is the rescoped doc with no stale claims.
- `cargo build --workspace` clean. `cargo clippy --workspace --all-targets -- -D warnings` clean.
- Per-file metadata envelopes complete; CLOG entries one-line.
- No-landmines pre-commit check passes (no new `#[allow]`).

## Mode

BLOCKER_MODE.

## Task-scope paths for grounding

Read first:

- `/usr/projects/tui-vfx/steering/INTENTIONS.md` (esp. Intentions 1, 8, 9, 26, 41).
- `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`.
- `/usr/projects/tui-vfx/docs/generated/SIGNALS_REFERENCE.md` (current state — the file being renamed).
- `/usr/projects/tui-vfx/docs/templates/signals.toml` (overlay being rescoped).
- `/usr/projects/tui-vfx/xtask/src/docs/extract_signals_rustdoc.rs` (where `is_parallel_channel` lives — see SignalDoc struct + per-family flag-set logic).
- `/usr/projects/tui-vfx/xtask/src/docs/parse_signals_toml.rs`.
- `/usr/projects/tui-vfx/xtask/src/docs/merge_signals.rs`.
- `/usr/projects/tui-vfx/xtask/src/docs/gen_signals_markdown.rs` (heading + preamble emission lives here).
- `/usr/projects/tui-vfx/xtask/src/docs/validate_signals.rs`.
- `/usr/projects/mixed-signals/docs/plans/signalspec-coverage-normalization-plan.md` (Phase A — the substrate change that invalidated the stale content).
- `/usr/projects/mixed-signals/src/types/signal_spec.rs` (lines 311–379 — the new physics + decay variants the existing doc claims don't exist).
- `/usr/projects/tui-vfx-recipes/steering/work-packets/01-recipe-signals-doc-generator.md` (the paired recipe-side packet — read for symmetric preamble text and split rationale).

## Exact write scope

Only edit / create / move these paths:

- `/usr/projects/tui-vfx/docs/generated/SIGNALS_REFERENCE.md` → **moved to** `/usr/projects/tui-vfx/docs/generated/API_SIGNALS_REFERENCE.md` (use `git mv`; do not delete + recreate).
- `/usr/projects/tui-vfx/xtask/src/docs/gen_signals_markdown.rs` (preamble const + heading + OUTPUT_PATH update).
- `/usr/projects/tui-vfx/xtask/src/docs/extract_signals_rustdoc.rs` (drop or rename `is_parallel_channel`; update per-family logic).
- `/usr/projects/tui-vfx/xtask/src/docs/parse_signals_toml.rs` (only if the rescope of signals.toml requires schema changes; otherwise no change).
- `/usr/projects/tui-vfx/xtask/src/docs/merge_signals.rs` (drop the stale callout-emission logic if hardcoded here).
- `/usr/projects/tui-vfx/xtask/src/docs/validate_signals.rs` (update OUTPUT_PATH reference).
- `/usr/projects/tui-vfx/docs/templates/signals.toml` (rescope to engine-API content).
- `/usr/projects/tui-vfx/docs/INDEX.md` (rename pointer; one-line note about recipe-author readers belonging in tui-vfx-recipes — but no link the other way; keep the boundary clean).
- `/usr/projects/tui-vfx/.omc/progress.txt` (record outcomes at packet close).

## Explicit out of scope

Do not widen into:

- `tui-vfx-recipes/**` — Packet 01 owns the recipe-side generator. This packet does not touch that repo.
- `mixed_signals/**` — Phase A is done. No upstream edits in this packet.
- Any engine crate (`crates/tui-vfx-*`) — pure docs + xtask change.
- The recipe corpus (`recipes/**` in either repo) — no recipe JSON changes.
- The `gt-design` repo — no changes; recipe-author boundary lives in tui-vfx-recipes.
- `cargo doc` rustdoc emission — markdown reference only.

## Must-read docs in order

1. `/usr/projects/tui-vfx/steering/INTENTIONS.md`.
2. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`.
3. `/usr/projects/tui-vfx-recipes/steering/work-packets/01-recipe-signals-doc-generator.md` (paired packet — preamble symmetry).
4. `/usr/projects/mixed-signals/docs/plans/signalspec-coverage-normalization-plan.md` (Phase A context).
5. `/usr/projects/tui-vfx/xtask/src/docs/gen_signals_markdown.rs` (where the H1 + OUTPUT_PATH live today).
6. `/usr/projects/tui-vfx/xtask/src/docs/extract_signals_rustdoc.rs` (where `is_parallel_channel` is set).

## Repo-boundary guardrails

- `mixed-signals` is the substrate — read-only here.
- `tui-vfx-recipes` is downstream — do **not** edit.
- This packet's writes are confined to `tui-vfx/docs/**` + `tui-vfx/xtask/src/docs/**`.

## Pipeline-touch definition of done

Not pipeline-touch. Standard non-pipeline-touch hygiene:
- Per-file metadata envelopes.
- One-line CLOGs.
- Zero new `#[allow]` suppressions.

## Stories

### US-1.1 — Rename + relabel

Use `git mv`:

```bash
git mv docs/generated/SIGNALS_REFERENCE.md docs/generated/API_SIGNALS_REFERENCE.md
```

Edit `xtask/src/docs/gen_signals_markdown.rs`:
- Update `const OUTPUT_PATH: &str = "docs/generated/API_SIGNALS_REFERENCE.md";`
- Update the H1 emission from `# tui-vfx Signals Reference` to `# Engine / Direct-API Signals Reference`.
- Add `const PREAMBLE: &str = "..."` containing the engine-side "wrong door" text (verbatim below).
- Emit `PREAMBLE` before the H1.

Edit `xtask/src/docs/validate_signals.rs`:
- Update any path constant referencing `SIGNALS_REFERENCE.md`.

Edit `docs/INDEX.md`:
- Rename pointer.
- Add a one-line note: "If you are writing recipe JSON, see the recipe signals reference in tui-vfx-recipes." Single sentence, not a link tree.

Acceptance:
- File rename preserves git history (verify with `git log --follow docs/generated/API_SIGNALS_REFERENCE.md`).
- `cargo xtask docs signals` writes to the new path; the old path no longer exists after generation.
- INDEX.md change reviewed for clarity.

### US-1.2 — Purge post-Phase-A stale content

Three sites:

**A. The "Note on physics signals" callout.**
Locate the source of the callout in either:
- `xtask/src/docs/gen_signals_markdown.rs` (if hardcoded in the markdown emitter), OR
- `docs/templates/signals.toml` (if injected via overlay), OR
- `xtask/src/docs/extract_signals_rustdoc.rs` (if assembled from upstream rustdoc + a flag).

Investigate and remove. The callout currently reads (paraphrased): "DampedSpring and the other physics primitives are in the parallel motion-spec channel — they are not reachable through SignalSpec JSON today. Phase γ of the signal-facade roadmap will collapse this gap." Phase A made this false.

**B. The Core 12 table's `(parallel channel)` annotation on the spring row.**
Locate in `gen_signals_markdown.rs` Core 12 emission logic. The annotation should be removed; the wire format for row 8 should read `spring` (the actual SignalSpec discriminant), not `spring (parallel channel)`.

**C. The `is_parallel_channel: bool` flag in `extract_signals_rustdoc.rs`.**
The `SignalDoc` struct has this field, set during extraction for physics types. The flag conflated two different things:
- "Not yet in `SignalSpec`" (was true for the 9 Phase-A primitives).
- "Belongs to the parallel motion-spec channel" (orthogonal — a runtime-routing concern, not a deserialization-reachability one).

Phase A made the first interpretation false. The second interpretation is true but irrelevant to this doc (which documents primitives, not motion routing).

Two options:
1. **Remove the flag entirely.** Simplest. Re-emission of the per-row annotation drops out automatically.
2. **Repurpose the flag** to "is also reachable as a `V3MotionDynamicSpec` `dynamics:` element" — a true statement, but a different one. If repurposed, document the new meaning in the field docstring and in any rendered annotation.

Recommendation: option 1 (remove). The motion-spec parallelism belongs in motion-route docs, not in the signals reference. Less code, no semantic drift.

Acceptance:
- The callout no longer appears in the generated `API_SIGNALS_REFERENCE.md`.
- Core 12 row 8 reads `spring` with no parenthetical.
- `is_parallel_channel` removed (or repurposed with explicit docstring).
- All physics primitives (`Spring`, `Bounce`, `Pendulum`, `Projectile`, `Orbit`, `Decay`, `Attractor`) and decay envelopes (`LinearDecay`, `ExponentialDecay`) are listed in the appropriate per-family sections with their actual SignalSpec wire format.

### US-1.3 — Rescope `docs/templates/signals.toml` to engine-API content

Audit `docs/templates/signals.toml` row by row:
- Engine-API content stays (e.g. constructor argument descriptions, direct-API usage hints).
- Recipe-author copy moves out (Packet 01 lifts these into `tui-vfx-recipes/docs/templates/signals.toml`).

Coordinate with Packet 01: identify which rows are recipe-author overlay (`recipe_hint`, `use_cases`, wire-format examples in author terms) vs engine-API overlay (constructor signatures, direct-call patterns, escape-hatch notes).

If both audiences need similar copy: keep two copies — the small duplication is cheaper than coupling the generators.

Acceptance:
- Engine-only content remains.
- The rescoped overlay produces a coherent `API_SIGNALS_REFERENCE.md` with engine-relevant hints, not recipe-author hints.
- No content lost (anything moved out is staged as input for Packet 01's overlay; coordinate via cross-repo notes in `.omc/progress.txt`).

### US-1.4 — Regenerate + verify

Run:

```bash
cd /usr/projects/tui-vfx
cargo xtask docs signals
git status   # only API_SIGNALS_REFERENCE.md and the moved-out path expected
git diff docs/generated/API_SIGNALS_REFERENCE.md  # spot-check rendered output
cargo build --workspace
cargo clippy --workspace --all-targets -- -D warnings
git diff --cached | rg '^\+.*#\[allow|^\+.*#!\[allow' || echo "no new #[allow] suppressions"
```

Manual spot-checks:
- Open the regenerated doc; verify preamble, H1, no stale physics callout, Core 12 table clean, every physics primitive present with real wire format.
- Verify `git log --follow docs/generated/API_SIGNALS_REFERENCE.md` traces back to the original SIGNALS_REFERENCE.md history.

Acceptance:
- All commands pass.
- Spot-checks confirm content is engine-scoped + post-Phase-A correct.
- `.omc/progress.txt` updated with command outcomes + spot-check notes.

## Preambles

### Top of `docs/generated/API_SIGNALS_REFERENCE.md` (verbatim)

```markdown
> **⚠ Not for recipe authors.**
>
> This reference documents `mixed_signals::*` primitives as reached by
> **direct API construction** in engine code (`tui-vfx-content`, `tui-vfx-style`,
> `gt-design` motion runtime, and other `mixed_signals::*` consumers).
>
> If you are writing a recipe JSON file (anything with `"type": "sine"`,
> `"type": "spring"`, `"signal": {...}`, etc.), **stop reading this document**
> and go to:
>
>   **→ `tui-vfx-recipes/docs/generated/RECIPE_SIGNALS_REFERENCE.md`**
>
> The recipe-author surface is `tui_vfx_recipes::signals::VfxRecipeSignalSpec`,
> a curated, wire-format-stable facade over `mixed_signals::*`. Field shapes,
> defaults, recommended use, and Core 12 guidance are documented there in
> recipe-author terms. This document does not cover them.
```

Emit as `const PREAMBLE: &str = "..."` at the top of `gen_signals_markdown.rs` so it's versioned with the generator and re-emitted on every regeneration.

## Test-shape requirements

- Generator round-trip: `cargo xtask docs signals` is idempotent.
- The validator (`validate_signals`) still passes against the new path + heading + preamble.
- A deliberate hand-edit to `API_SIGNALS_REFERENCE.md` triggers the existing freshness gate (if one is configured) on next CI run.

## Hot-path watchpoints

Doc generation only. No runtime or CI-blocking change other than the freshness gate noticing the rename (which is the intended behavior on first CI run after merge).

## Verification required

```bash
cd /usr/projects/tui-vfx
cargo xtask docs signals
cargo build --workspace
cargo clippy --workspace --all-targets -- -D warnings
git log --follow docs/generated/API_SIGNALS_REFERENCE.md | head -20  # confirm history preserved
git diff --cached | rg '^\+.*#\[allow|^\+.*#!\[allow' || echo "no new #[allow] suppressions"
```

If any command fails, report the exact failure, classify it (in-scope / expected fallout / blocker), and stop until the leader confirms the fix path.

## Pre-commit write-scope guard

```bash
git diff --cached --name-only
```

Output must list only paths in "Exact write scope" above (note: the rename appears as a delete + add, or as `R` if `git mv` was used). Stage by explicit path; never `git add -A`.

## Reporting contract

Final report must include:

- Docs-read confirmation (must-read docs in order).
- 3 reflection bullets (what worked, what surprised, coordination notes for Packet 01).
- Exact files changed (full paths) including rename source/target.
- Exact commands run + pass/fail per command.
- Sample output: first 60 lines of regenerated `API_SIGNALS_REFERENCE.md`.
- progress.txt update summary.
- Any blockers / handoff notes for Packet 01 (especially: which signals.toml rows moved to recipe-side overlay).

## File metadata discipline

Every touched file:

- `<CLOG>` entry one or two short lines.
- `<WCTX>` updated if the file's role changed (the renamed markdown file's role explicitly changed — engine-only now).
- `<VERS>` PATCH bump for internal edits, MINOR for renames.

## Closing task reminder

**Task:** rename `SIGNALS_REFERENCE.md` → `API_SIGNALS_REFERENCE.md`; rescope to engine / direct-API; purge post-Phase-A stale content (callout + Core 12 annotation + `is_parallel_channel` flag); rescope `docs/templates/signals.toml`; regenerate + verify.

**Do not widen into:** `tui-vfx-recipes/**`, `mixed_signals/**`, any engine crate, the recipe corpus.

**Companion packet:** `tui-vfx-recipes/steering/work-packets/01-recipe-signals-doc-generator.md` (the recipe-side generator).

<!-- <FILE>steering/work-packets/67-engine-api-signals-doc-relabel.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
