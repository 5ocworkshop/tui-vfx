# Kickoff prompt: AI-native pipeline observability tooling

> **How to use this file:** paste the contents below (everything under
> "PROMPT START" through "PROMPT END") into a fresh Claude Code session.
> The primary working directory should be `/usr/projects/tui-vfx` (the
> engine repo) because the core observability library lives there; the
> session will also reach into `/usr/projects/tui-vfx-recipes` for the
> recipe-adapter CLI and the existing recipe corpus. It's a self-contained
> briefing that gets the agent oriented without re-deriving context from
> scratch.

---

## PROMPT START

### Project orientation

You are working across two sibling Rust workspaces:

- **`/usr/projects/tui-vfx`** (engine, primary workspace for this work):
  compositor pipeline (sampler → mask → shader → filter → style),
  per-cell primitives (`Cell`, `Color`, `Style`), the effect taxonomy
  (`FilterSpec`, `MaskSpec`, `SamplerSpec`, `SpatialShaderType`,
  `StyleEffect`, `ContentEffect`), and the
  `tui_vfx_compositor::traits::pipeline_inspector::CompositorInspector` +
  `tui_vfx_recipes::inspector::PipelineInspector` traits that expose
  per-cell transformation hooks. **The new observability library and its
  CLI entry point belong here.**
- **`/usr/projects/tui-vfx-recipes`** (recipes + adapter): the recipe
  schema surface (V3 direction: `VfxRecipeConfig`; legacy code may still
  expose `RaRecipeConfig` during cutover), preview manager, rendering
  integration with ratatui, the
  existing `pipeline-validator` CLI, the `StageInspector` wrapper, and
  a 101-recipe debug + demo corpus under `recipes/debug_recipes/` and
  `recipes/gt-design-codex/`. **This is where the recipe adapter CLI
  lives** — it parses recipe JSON, builds an engine config, and hands
  off to the engine-side observability library.

**You must follow the operating constraints in
`/home/jac/.claude/CLAUDE.md`**: OFPF file naming (`fnc_`, `orc_`,
`cls_`, `ui_`, `test_`), metadata headers + version bumps on every file
you touch, TDD for non-trivial logic, commit messages in the
Work Context / Changes format with no `Co-Authored-By` line, and
specific-file `git add` (never `-A`). Treat these as hard rules. The
rules apply equally to both repos.

### Why we're here

The previous session audited ~100 debug recipes with the existing
`pipeline-validator` CLI and repeatedly hit friction that showed the
tool was built for a human glancing at prose dumps, not for an AI
programmatically interrogating recipe state. Every debugging question
turned into 5–10 lines of bash + grep against ad-hoc prose output, and
subtle issues (silent field drift, phase-tick desync between validator
and engine, char-only filter writes not counted as modifications) went
undetected for long stretches because the observation surface was
sparse and human-shaped.

The user elevated "make the tooling AI-native" to a **first-class
project goal**: build a CLI that lets an AI see the entire recipe
playback — every cell, every modification, every stage, every frame —
in a format optimized for machine parsing and progressive disclosure.
The intuition is that this turns recipe creation, testing, debugging,
and refinement from a friction-dominated activity into a fluent one,
which is force-multiplier for any AI authoring recipes.

### Read this first

`/usr/projects/tui-vfx/docs/design/ai-native-pipeline-observability-ideas.md`
is the canonical source of specific ideas. It has 27 friction→idea
pairs captured in-the-moment while fighting through the audit of the
`tui-vfx-recipes` debug-recipe corpus, a proposed canonical command
shape, a priority order (P0–P2), and a suggested first-pass
implementation order. **Read that file completely before proposing
any design.** Do not re-derive it from scratch; treat the friction
there as evidence, not opinion.

### Philosophical framing (read this too, it matters)

Current `pipeline-validator` is "show the human enough to spot the
obvious". The AI-native tool is "emit the full state; let the caller
filter." The design shift:

- **Structured over prose.** Prose output (grid maps, per-row
  classifiers, sample-cell lists) is optimized for pattern-matching by
  a human eye. An AI parses it via regex, and every regex is a fragile
  translation layer. JSON/NDJSON first-class output replaces that
  translation layer with a schema.
- **Progressive disclosure, not implicit sampling.** Current dump
  caps at 10 (now 32) "first non-empty cells" in row-major order. That
  almost always means "top border row only" on wide widgets. Replace
  implicit sampling with explicit queries: "give me all cells" / "give
  me cells in rect" / "give me cells that changed between t=0.3 and
  t=0.5" / "give me cell (x,y)". The caller asks for exactly the
  granularity it wants.
- **Causation over state.** Observing a cell's final state rarely
  tells you what's wrong — you need to know which stage wrote it,
  which effect owned the write, and what the cell was before. The
  inspector traits already capture this internally; the tool has to
  expose it per-query.
- **Trustworthy clocks.** The validator's lifecycle tick had silent
  math desynchronization with the engine that corrupted exit-phase
  observations for every recipe with a style effect in exit. Every
  output must self-report its timing assumptions so the AI can detect
  drift instead of being fooled by it. See ideas #21 and #25.
- **Loud unknowns.** Silent serde field drops were responsible for
  5 of the broken recipes the previous session found. The tool should
  treat unknown fields as signals, not noise. See ideas #23 and #26.

Every feature in the ideas doc is grounded in a specific friction
from the audit. If you disagree with an idea's priority or framing,
read the audit entry it came from — there's probably context you'd
otherwise miss.

### Architecture (decided — do not re-litigate)

The observability layer is split across the two repos along a clean
seam:

**Engine side (`tui-vfx`) — NEW core observability library.** This is
where the P0 work lives.
- Structured per-cell emission with a typed JSON schema.
- Causation tracking (which stage wrote each cell, before/after, phase,
  `t`) layered on top of the existing `CompositorInspector` hooks.
- Timeline sampling, diff computation, region queries, cell queries,
  effect-health checks — all pure engine concepts that operate on a
  `CompositionOptions` or equivalent direct engine config.
- A Rust API so tests, benchmarks, and other engine consumers can
  embed the observability surface, plus a thin binary (working name:
  `pipeline-probe`) that accepts direct engine configs.
- **Crucial:** nothing in this crate depends on `tui-vfx-recipes`. The
  engine must not import recipes types — that would create a reverse
  dependency and lock the debuggability layer behind recipe authoring.

**Recipes side (`tui-vfx-recipes`) — recipe adapter.** The existing
`pipeline-validator` stays, thins out, and delegates.
- Keeps the recipe-specific stages: Parse, Rules, Profile. Those are
  about schema correctness and live naturally next to the schema.
- Replaces its Render/Shader/Output/Stages pipeline with a call into
  the engine-side observability library once the engine work lands.
- Its job shrinks to: parse recipe JSON → build engine config → hand
  off → print results. Everything else (structured output, causation,
  timeline, diff) comes from the engine library transparently.
- The 101 debug recipes stay here. They're the integration test
  corpus for the recipe adapter and a fixture set for the engine
  library via the adapter.

**Two wins from this split:**
1. No reverse dependency. `tui-vfx` stays at the bottom of the stack.
2. Reusable debuggability surface. Engine authors, alternative recipe
   formats, programmatic consumers, and effect implementers all get
   the same observability layer for free instead of having it locked
   inside a recipe-validator binary.

**Known complication — staged rollout.** The existing
`pipeline-validator` has working code for Render / Shader / Output /
Stages that currently uses `StageInspector` (a `tui-vfx-recipes` wrapper
over the engine's inspector traits). During the rollout:
- Phase 1: build the engine-side library and its standalone binary.
  Leave the existing validator alone so the recipe corpus keeps
  passing.
- Phase 2: once the engine library can reproduce the current
  validator's output for a representative recipe, add a feature flag
  to the validator that delegates to it. Keep the old path behind the
  flag until parity is confirmed.
- Phase 3: retire the duplicate inspection path in the validator,
  leaving only the recipe-specific schema checks + delegation.

Do not rush phase 2. The recipe corpus is the safety net — if
delegation breaks a recipe's output, the library has a gap.

### Current state of the repos (as of this kickoff)

All of this landed during the previous session:

- **101/101 debug recipes pass `--rules --stages`** — this is the
  baseline. Any change you make must preserve it.
- **`cargo test --workspace` is fully green** — 28 main tests, 5
  canvas-compositing regression tests, plus all validator and content
  tests. Do not let this regress.
- **Tooling enhancements already in place:**
  - `src/inspector/impls/cls_stage_inspector.rs` v1.5.0 — captures
    before/after symbol on every filter and shader hook, so char-only
    modifications now count properly.
  - `tools/pipeline-validator/src/stages/functions/fnc_sample_buffer_cells.rs`
    v1.3.0 — per-row fg brightness dump + per-row fg/bg anomaly maps
    that highlight outlier cells vs row mode (lets you see a 3-dot
    Orbit shader immediately).
  - `tools/pipeline-validator/src/stages/functions/fnc_validate_output.rs`
    v1.3.0 — fixes two interrelated lifecycle-timing bugs:
    (1) `--phase exiting` now honors `--sample-t`, and
    (2) `dwell_ms = auto_dismiss_ms` (previously subtracted enter+exit,
    which silently pushed exit samples back into dwell).
  - `tests/test_filter_recipe_coverage.rs` — snapshot test asserting
    every `FilterSpec` variant has a matching debug recipe. Extend to
    the other effect taxonomies (shaders/masks/samplers/styles) when
    you get there.
- **Recipe fixes:** 3 parse failures fixed (`target→target_color`,
  `color→pulse_color`, `speed→rotation_speed`), 2 silent drifts fixed
  (`ease→easing` on both fade_in and fade_out), 1 tuning fix
  (`style_rainbow` rotation_speed 360→1.0).
- **Recent commits (most recent first) — read `git log` for details:**
  - `4ee7a7a` docs(design): +3 ideas from fade_out debug session (25-27)
  - `8c3e8e8` fix(validator+recipes): exit-phase sampling, dwell_ms math, ease→easing drift
  - `1c258ee` docs(design): capture AI-native recipe observability ideas from audit
  - `ca1e6b2` fix(audit): repair 3 parse-failing style recipes + enhance validator tooling
  - `15acd96` fix(debug-recipes): boost KittScanner visibility
  - `eb55301` docs(plans)+test(coverage): correct filter plan field lists and add FilterSpec coverage guard
  - `8882df3` / `a20965c` / `93a48f9` — 3 tiers of filter debug recipes (18 total)

- **Pre-existing unstaged files** (were there at session start,
  unrelated to this work — **do not touch**):
  - `.gitignore`
  - `recipes/gt-design-codex/blueprint_inspection_gate_modal.json`
  - `recipes/gt-design-codex/eichler_sunburst_ping_info.json`
- **Upstream docs aligned** in `/usr/projects/tui-vfx/docs/` commit
  `0f58618` — the hand-maintained filter tables in `CAPABILITIES_REFERENCE.md`
  and `API_HAND.md` were out of sync with `cls_filter_spec.rs` v3.4.0
  and have been corrected. The generated docs under `docs/generated/`
  are already in sync via the TOML template.

### Suggested first-pass scope (do not treat as fixed)

The ideas doc has a suggested implementation order. A minimal-viable
sequence that gives the AI real leverage fast:

1. **Design doc first** (~30–60 min): crystallize the crate layout
   inside `tui-vfx`, the command shape for `pipeline-probe`, the JSON
   schema, and the progressive-disclosure hierarchy. Circulate with
   the user, iterate until the shape is right, THEN implement. Do not
   start coding without alignment on the schema — the schema is the
   contract and it's expensive to change later. Put the design doc at
   `/usr/projects/tui-vfx/docs/design/pipeline-probe-design.md` (or
   similar) alongside the ideas doc.
2. **Crate scaffolding in `tui-vfx`.** A new crate (working name
   `tui-vfx-probe` or similar) under `tui-vfx/crates/` that depends
   only on other `tui-vfx-*` crates. Expose both a library entry
   point and a `pipeline-probe` binary. No recipes dependency.
3. **`--format json` core** — the single enabler that unblocks every
   other item. Start with a dump that accepts a direct
   `CompositionOptions` (or the engine equivalent) and emits every
   cell in the widget area as typed JSON with metadata + timing.
   Prove it against a minimal test fixture before wiring to recipes.
4. **Style + content stage counters** (idea #19) — extend the engine
   inspector traits with `on_style_effect_applied` and
   `on_content_effect_applied` hooks. Without this, 27% of recipes
   register as zero-activity in the main health check, and any audit
   tool is blind to them. These hooks are in `tui-vfx`, which is why
   the work goes engine-side.
5. **Per-cell causation trace** (idea #3) — store per-cell stage /
   effect history in a new inspector implementation in the engine
   and expose it as a queryable map. This is the single
   highest-leverage feature for non-trivial debugging.
6. **Frame timeline + diff modes** (ideas #4, #5) — required for
   verifying animation correctness without N separate invocations.
7. **Recipe adapter delegation (phase 2 of the rollout).** Once
   the engine library can reproduce the current validator's output
   for a sample recipe, add a feature flag to `pipeline-validator`
   that delegates to it. Keep the old path until parity holds
   across the full 101-recipe corpus.
8. Whatever else the ideas doc surfaces as P0 that you haven't
   covered yet.

### Work mode

- **Discuss before building.** For anything beyond trivial fixes,
  surface the plan and get alignment. This work is about shaping an
  interface, not writing the most Rust code possible. Brainstorm
  liberally but mark brainstorms as brainstorms, not decisions.
- **TDD where the logic warrants it.** JSON schema serialization,
  causation-chain reconstruction, diff computation, phase-tick
  correctness — all warrant tests first. Pure plumbing changes (e.g.,
  adding a new CLI flag that delegates to existing code) don't need
  TDD ceremony.
- **Run `cargo test --workspace` at every meaningful checkpoint in
  both repos.** The previous session's test suite is the safety net —
  if it regresses, you've almost certainly broken something
  load-bearing. Run it in whichever repo you touched.
- **Run the full recipe validation corpus when touching the adapter.**
  From `/usr/projects/tui-vfx-recipes`:
  `cargo run -q -p pipeline-validator -- --rules --stages recipes/debug_recipes/**/*.json`
  should continue to show 101/101 passing throughout the rollout. Any
  regression there is a blocker, even during phase 2 delegation.
- **Keep appending to the ideas doc** as you hit new friction. The
  file is a living document; treat it like a lab notebook. Each entry
  is cheap to write and valuable later.

### Where the friction was loudest (so you can feel it yourself)

Run these commands in a throwaway terminal to experience the current
state before you touch anything. This is the baseline you're improving
against:

```bash
# The existing "full dump" is actually ~32 cells of the top border
cargo run -q -p pipeline-validator -- --dump --stage output \
    --sample-t 0.5 --phase dwelling -vvv \
    recipes/debug_recipes/shaders/shader_orbit.json

# The per-row anomaly map (just added) makes Orbit's 3 dots visible,
# but only because of a retrofit. In JSON mode this should be trivial.
cargo run -q -p pipeline-validator -- --dump --stage output \
    --sample-t 0.5 --phase dwelling -vvv \
    recipes/debug_recipes/shaders/shader_orbit.json | grep -A 12 "anomaly"

# Stage counts work now, but you cannot query WHICH cells the filter
# modified, which stage wrote the "after" state, or what the "before"
# was. All the data is inside the StageInspector; none of it is exposed.
cargo run -q -p pipeline-validator -- --stages -vvv \
    --sample-t 0.5 --phase dwelling \
    recipes/debug_recipes/filters/filter_pattern_fill.json
```

Notice: every one of those outputs is prose you'd have to regex to
consume. That's the shape we're replacing.

### First thing to do when you start

1. `git log --oneline -10` in `/usr/projects/tui-vfx` and
   `git log --oneline -15` in `/usr/projects/tui-vfx-recipes` — see
   recent history on both sides.
2. `cat docs/design/ai-native-pipeline-observability-ideas.md` (from
   the `tui-vfx` working directory) — read the ideas doc in full.
3. `cd /usr/projects/tui-vfx-recipes` and run the three friction
   commands from the section above to feel the current recipe-side
   output format that the engine-side library is going to replace.
4. Skim the engine-side surfaces the new library will build on:
   - `/usr/projects/tui-vfx/crates/tui-vfx-compositor/src/traits/pipeline_inspector.rs`
     — the existing inspector trait you're going to extend or wrap.
   - `/usr/projects/tui-vfx/crates/tui-vfx-compositor/src/types/` —
     effect spec definitions.
5. Come back to the user with: "I've read the context. Here's how I'd
   lay out the new `tui-vfx` crate, here's the JSON schema sketch for
   the smallest useful output (a single-frame cell dump), and here's
   the question I'd like to settle before writing any code." Ask
   whether the crate should live in `tui-vfx/crates/tui-vfx-probe/`
   or somewhere else, whether the binary name is `pipeline-probe` or
   something else, and what the first fixture should be (a minimal
   programmatic config or a recipe-loaded one via a test helper).

Do not start writing code in the first response. The schema is a
contract; get alignment before committing to it. The architectural
split is already decided (see the "Architecture" section above) —
don't re-litigate that, build on it.

## PROMPT END

---

## Notes for the human paste-er

- This file is self-contained context; you shouldn't need to add
  anything unless your goals have shifted.
- If you want the agent to skip the "read first" orientation and
  jump into coding, delete the "First thing to do when you start"
  section — but I don't recommend it; the orientation is cheap and
  the ideas doc has context the agent can't recover from source.
- If the work you want is narrower than the full AI-native rebuild
  (e.g., just JSON output, or just causation traces), say so at the
  top of your first message. The prompt is a maximal briefing, not
  a minimal one.
- If you move or rename `docs/design/ai-native-pipeline-observability-ideas.md`,
  update the path references in this prompt.
