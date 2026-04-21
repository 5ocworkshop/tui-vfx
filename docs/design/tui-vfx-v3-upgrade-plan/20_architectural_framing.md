<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/20_architectural_framing.md</FILE> - <DESC>Chapter 20 — architectural framing: layer model (L1→L5), the ecosystem-agnostic seam rationale, mixed-signals as canonical upstream home for signal primitives, and the two-level chaining distinction. Where V3 decisions live in the stack and why those layer boundaries matter.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Extracted from the monolithic plan (v0.16.0) "Architectural framing" section. Includes the L1-L5 layer diagram from the monolith (preserved verbatim).</WCTX> -->
<!-- <CLOG>1.0.0: initial extraction from the monolith.</CLOG> -->

# 20 — Architectural framing

## 10 — The ecosystem-agnostic layer earns its place

`tui-vfx` deliberately renders to a grid and maps to ratatui at the final stage rather than being ratatui-native. Earlier in plan development this looked like optionality that might never cash in. It does cash in — the movie-composer concept (see `90_deferred_design.md`) is a concrete use case where the grid-first architecture is the enabler, not overhead.

The mental model: **ratatui is *a* consumer of tui-vfx, not *the* consumer.** Future sibling consumers become possible without rewriting the compositor:

```text
 L5 consumers:    ratatui app  │  movie player  │  static renderer  │  wasm embed
                       ↓               ↓                 ↓                 ↓
 L4 adapters:    gtd-ratatui   │  gtd-movie*    │  gtd-static*      │  gtd-wasm*
                       ↓               ↓                 ↓                 ↓
 L2/L3:                  tui-vfx-recipes + tui-vfx-compositor (grid-first)
                                            ↓
 L1:                   tui-vfx-types, mixed-signals, mcu-terminal-color
```

(*Siblings beyond `gtd-ratatui` are hypothetical; just mapping the territory.*)

V3 design must preserve this. A decision that introduces ratatui-specific types into the pipeline vocabulary (e.g., a step binding to `ratatui::layout::Rect`) is a regression. Grid-first is honored; adapters translate at L4.

**Why this costs but earns its place even when we only have one L5 consumer today:**

The grid-first architecture has real costs: a `tui-vfx-types::Cell` that parallels `ratatui::buffer::Cell` (requiring conversion at the boundary), compositor work that can't use ratatui's native cell types directly, extra indirection in hot paths. For gt-design alone — our only current production consumer — this is overhead that doesn't visibly pay back. Earlier in plan development this looked like optionality that might never cash in.

The movie-composer use case is the concrete cash-in: a movie player doesn't need ratatui's widget/layout/event-loop machinery. With grid-first architecture, a movie-player binary can render directly to crossterm / stdout / a file / a wasm buffer without pulling ratatui into its dependency tree. Ship static binaries at a few hundred KB instead of MB. Enable adjacent uses: terminal recordings for docs, CI visual regression via grid diffs, wasm-embedded demos, SVG/PNG/SIXEL static export, training movies, documentation hero assets.

Three secondary benefits the agnostic layer also provides:

- **Clean test surface.** Compositor unit tests have no ratatui dependency. Grid assertions are easier to reason about than ratatui-buffer assertions with their mod-state.
- **Forced layer discipline.** Intention 40 is about using foundation libraries instead of rolling your own. Part of why we *can* honor that cleanly is that tui-vfx's internals don't presume a ratatui world.
- **Optionality.** If gt-design or any future product ever wants to render to something other than ratatui (web terminal renderer, custom compositor, embedded display), we have a seam.

The decision to pay the cost isn't abstract optionality anymore — it's validated by a concrete second consumer (movie-composer) that's plausibly imminent and architecturally natural. V3 must not regress this — every decision is evaluated for whether it leaks ratatui-specific types into pipeline vocabulary.

## 20 — mixed-signals is the upstream home for signal primitives

`/usr/projects/mixed-signals` was created deliberately near the start of the project as the canonical home for signal primitives. It has been stable for 4+ months and already carries the 1D composition / processing / noise / physics / envelope catalog (`Sine`, `Triangle`, `Ramp`, `Keyframes`, `Add`, `Multiply`, `Mix`, `Normalize`, `Remap`, `Clamp`, `ADSR`, `DampedSpring`, `SpatialNoise`, etc.).

**V3 consumes signals; V3 does not invent signals.** When V3 needs a capability that doesn't exist in `mixed-signals` today (the `SpatialSignalSpec` 2D-aware signal graph for the flag-animation PRD is the current driver), the correct response is to extend `mixed-signals` upstream, not to build a parallel signal surface inside `tui-vfx`.

This flips the preference stated in the flag-animation PRD (v0.3.0) where "Path A" (local `SpatialSignalSpec` in tui-vfx-compositor) was primary and "Path B" (upstream `Signal2d` trait in mixed-signals) was an opt-in follow-up. The V3 direction is that **Path B is primary**; Path A is a fallback only if upstream velocity genuinely blocks V3 delivery. This aligns with gt-design Intention 40: *"when a foundation library is missing a capability GTD needs, the correct response is to extend the foundation library — not to work around it with inline code."*

## 30 — Two levels of chaining live at different layers

Pipe-culture chain-ability (Principle 2) applies at more than one level, and V3 must keep them distinct:

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│   Level 1: SIGNAL-GRAPH composition   (lives in mixed-signals)  │
│   ─────────────────────────────────                             │
│                                                                 │
│       Sine ─┐                                                   │
│             ├─▶ Add ─▶ Multiply ─▶ (signal value)               │
│       Ramp ─┘            ▲                                      │
│                    Envelope                                     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
                           │
                           │  signals flow in as one kind of
                           │  bindable value between steps
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│   Level 2: PIPELINE-STEP chaining     (lives in V3 schema)      │
│   ─────────────────────────────────                             │
│                                                                 │
│    ┌────────┐   displacement   ┌─────────────────┐              │
│    │Sampler ├──────hint───────▶│DisplacementShade│              │
│    └────────┘                  └─────────────────┘              │
│                                                                 │
│    (HintRef<T> + ParamValue<T> — see Decisions 6 and 7)         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

1. **Signal-graph composition** — combining signals into composite waveforms (`Add(Sine, Ramp)`, `Multiply(wave, envelope)`). Lives in `mixed-signals`. Already works in 1D; V3 wants 2D-aware extensions upstream (see above).
2. **Pipeline-step chaining** — one V3 step's output feeding another V3 step's input (`DisplacementShade` reading a sampler's offset hint). Lives in the V3 schema as a first-class primitive. Treats signals as *one kind* of bindable value that can flow between steps, alongside other hints like displacement offsets, cell-density maps, and sampled colors.

Mixed-signals must not grow a "pipeline step" concept, and V3 must not duplicate signal-composition logic. The layering is clean; the plan must honor it.

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/20_architectural_framing.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
