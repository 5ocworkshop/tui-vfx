<!-- <FILE>CLAUDE.md</FILE> - <DESC>Project-root orientation for Claude (and other coding agents) entering this repository. Points at the steering docs, the active design work, sibling projects, tooling entry points, and the load-bearing don'ts. Kept short so it's read before editing.</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Add a hard "Before your first response" checklist at the top. v0.1.0's "read the steering docs" wording was soft enough that sessions rationalized skipping it on summary-style questions. The checklist makes orientation an explicit precondition of the first reply, not a background suggestion.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — prepend imperative orientation checklist; replace soft "read the steering docs" opening with a numbered precondition list covering ofpf-status, MARKETING.md, INTENTIONS.md, and ofpf-orientation. No structural changes below that section.</CLOG> -->
<!-- <CLOG>0.1.0: initial project-root orientation file.</CLOG> -->

# tui-vfx — Claude orientation

This repo is a visual effects library for terminal UIs.

## Before your first response — do all four

This is a precondition, not a suggestion. "Simple" questions (summaries, quick lookups, one-line answers) are not exempt. If you think orientation is overkill for this particular question, that is the failure mode — do it anyway.

1. **Run `ofpf-status`** — confirms the librarian daemon is healthy. If it errors, surface that before answering; most repo questions are unreliable without it.
2. **Read `steering/MARKETING.md`** — what tui-vfx is, feature hierarchy, competitive positioning vs tachyonfx, current status. The 90-second form is canonical; shorter forms are compressions of it.
3. **Read `steering/INTENTIONS.md`** — 29 durable engineering and architectural intentions. The **Writing style** section at the top governs all docs, commits, rustdoc, and review prose — no marketing voice, no grandiose framing, one idea per sentence.
4. **Run `ofpf-orientation`** — architecture graph. Required before reading code; optional but useful before answering design questions.

The global `~/.claude/CLAUDE.md` covers the OFPF methodology and `ofpf-*` tooling that's the primary interface for this repo — read it if the concepts above (librarian, `ofpf-*`, OFPF file prefixes) are unfamiliar.

## Active design work

The V3 recipe-schema redesign is in planning. Home: **`docs/design/tui-vfx-v3-upgrade-plan/00_INDEX.md`**. Eight structural decisions, 23 open questions, six release gates. Features annotated `(V3)` in MARKETING.md depend on this cutover.

Companion: `docs/design/tui-vfx-v3-schema-draft.json` is the specification-by-example of the draft schema (strip `#`-prefixed lines for valid JSON).

## Sibling projects

- **`/usr/projects/mixed-signals`** — canonical home for signal primitives (`Sine`, `Triangle`, `Ramp`, `Keyframes`, `ADSR`, `DampedSpring`, spatial noise, operators). V3 consumes signals; V3 does not invent signals (Intention 9). When a capability is missing, extend mixed-signals upstream.
- **`/usr/projects/tui-vfx-recipes`** — recipe schema, validator, canonical playback-item builder, trace/probe tooling. Recipe-authoring truth lives here; downstream consumers wrap rather than reinterpret (Intention 3).
- **`/usr/projects/gt-design`** — first production consumer. Owns theme resolution, surface identity, factory composition, render-truth routing. Meaning lives low (tui-vfx); policy lives high (gt-design) — Principle 5.
- **`/usr/projects/rocketsplash`** — authoring tool for the asset formats tui-vfx consumes (`.rsi` cell-coarse images, `.rsb` braille-supersampled images, `.rsf` font atlases).

## Tooling entry points

- **`just --list`** — all project recipes (docs generation, recipe validation, API doc workflows).
- **`cargo xtask --help`** — subcommands for docs generation, recipe validation, freshness checks. `cargo xtask docs generate` regenerates the capability manifest from rustdoc + `docs/templates/capabilities.toml`.
- **`cargo test -p <crate>`** — per-crate tests. Linux is the official release-blocking validation target (Intention 16).

## Code conventions (summary)

Full spec lives in global `~/.claude/CLAUDE.md` and `~/.claude/rules/ofpf.md`. Project-specific notes:

- **OFPF file prefixes.** `fnc_`, `orc_`, `cls_`, `ui_`, `test_`, with per-prefix size limits. Metadata envelopes on every source, test, and markdown file.
- **Wire-format prefix.** Public types crossing crate boundaries use the `Vfx*` prefix (Intention 8). V3 renames the legacy `Ra*` prefix to `Vfx*` (Decision 4); both exist in-tree until the cutover.
- **Tests pass = done.** TDD red→green is the default. Audit gate at phase end (Intentions 14, 15).
- **Commits.** No Co-Authored-By lines (per user's global instructions). Commit-message style: `Work Context:` + `Changes:` blocks — see `git log` for the house pattern.

## Load-bearing don'ts

- **Don't import internal crate types from consumer examples.** Use the public surface; expand the surface if needed (Intention 2).
- **Don't build signal primitives inside tui-vfx.** Extend `mixed-signals` upstream (Intention 9).
- **Don't break individual-unit addressability when consolidating.** Debug/preview recipes stay as individual files; `template + variants` and `$use` fragment forms must ship with tooling that preserves per-item addressability (Intention 26 / Principle 4).
- **Don't leak ratatui types into the compositor.** tui-vfx is grid-first; ratatui is a consumer, not the consumer. Adapters translate at L4 (Intention 1).

<!-- <FILE>CLAUDE.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
