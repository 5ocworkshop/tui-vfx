<!-- <FILE>docs/new_kernel/V31_VERTICAL_MIGRATION_VALIDATION_PROCESS.md</FILE> - <DESC>Junior-agent procedure for V2 deprecated to v3.1 vertical recipe migration validation</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>Repeatable v3.1 primitive migration and validation process using V2 deprecated recipes as the oracle.</WCTX> -->
<!-- <CLOG>0.3.0: MINOR — record color-channel oracle requirements for style primitives.
0.2.0: MINOR — require styled-cell glyph evidence for glyph-changing primitives.
0.1.0: INIT — document the vertical migration evidence loop for junior agents.</CLOG> -->

# v3.1 Vertical Migration Validation Process

Use this process for one primitive recipe at a time. Do not migrate a family horizontally before one vertical slice is proven end-to-end.

## 1. Required briefing and scope

1. Read `.omx/context/v31-vertical-migration-briefing-latest.md` first.
2. Read `steering/INTENTIONS.md`, `steering/OFPF-TOOLS.md`, `steering/TASK_PACKET_TEMPLATE.md`, and `steering/work-packets/COMMON_EXECUTION_RULES.md`.
3. Read applicable `../global_prompts/standards/*.md`, especially OFPF, TDD, file-centric execution, sub-agent orchestration, metadata headers, and recycle-bin rules.
4. Do not read `steering/ORCHESTRATION.md` unless the leader explicitly authorizes leader-only context.
5. Work only in `/usr/projects/tui-vfx` and `/usr/projects/tui-vfx-recipes`.

## 2. Pick exactly one recipe

Start with a simple V2 `_DEPRECATED_` debug recipe that isolates one primitive. Good early candidates are masks, simple filters, styles, and samplers with one clear visible behavior.

Record:
- V2 source recipe path.
- V3.1 target recipe path.
- Effect id.
- Active lifecycle phase or phases.
- Viewer-facing title, description, expected visual notes, and message text.
- Layout, foreground/background colors, border, timing, and payload fields.

## 3. Capture the V2 oracle evidence

Use the mature V2 tooling in `/usr/projects/tui-vfx-recipes` before changing v3.1 files.

Capture enough evidence to make the expected behavior deterministic:
- Key sampled phase times, usually enter midpoint, enter near-complete, dwell, and exit if the recipe has exit behavior.
- Visible rows or cell/frame database output when available.
- Letter/glyph counts when useful.
- Foreground/background colors.
- Whether blank rows, borders, and message text are expected at that sample.

Do not rely only on visual memory. Paste or save the command and the observed evidence.

## 4. Migrate the v3.1 recipe directly from V2

Preserve V2 behavior unless the leader records an intentional deviation.

Required mapping checks:
- Metadata title and description are present and useful.
- Expected visual text explains what the viewer should see.
- Source/card message, width, height, foreground, background, border style, and border trim match the V2 presentation intent.
- Lifecycle phase durations and active phases match V2 timing semantics.
- Graph node inputs preserve all V2 primitive payload fields using descriptor-backed v3.1 inputs.
- Names are durable and human-readable; do not use transient work-packet labels in fields or symbols.

## 5. Prove v3.1 through the real player path

Use the compositor backend in native mode and fail on fallback:

```bash
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-cli -- render-backend --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/<relative>.json --descriptor-pack descriptors/v3.1/packs/primitive.json --backend compositor --composition-mode native --fail-on-fallback --format json --phase <enter|dwell|exit> --phase-t <sample>
```

Check every sampled point:
- `fallbackUsed` is `false`.
- `nativeLoweringSucceeded` is `true`.
- `diagnostics` do not contain `unsupportedNativeEffect`.
- `compositionSpecSummary` includes the expected native stage/filter/mask count for the active phase.
- `letterCellEvidence` matches the V2 oracle when the primitive affects visible text.
- `letterCellEvidence.foregroundBackgroundClassCounts` matches the V2 oracle when a style primitive changes foreground and background color channels without changing glyphs.
- `styledCells` glyph/foreground/background evidence matches the V2 oracle when the primitive changes non-alphanumeric glyphs, bullets, borders, or indicator cells.
- `rows` show the same presentation behavior at the chosen sample.

Phase matters. If a recipe is gated to `enter`, do not validate it at default dwell and call empty output a failure or success without checking the lifecycle.

## 6. Add or extend focused regression coverage

Prefer extending the narrowest existing test rather than adding broad snapshots.

For each proven primitive, add deterministic assertions for:
- Native mode succeeds without fallback.
- The active phase lowers the expected primitive.
- V2-oracle letter-cell counts or color-class counts match at key samples.
- V2-oracle foreground/background class counts match for color-only style primitives such as color fade.
- V2-oracle styled-cell glyph counts match for glyph-changing primitives such as dot indicators.
- Invalid enums and unsupported fields still fail when the node is active.

Use `cargo nextest` for test runs.

## 7. Report results first

A junior-agent report must start with successful results, then evidence.

Include:
- Recipes migrated.
- Exact V2 oracle samples captured.
- Exact v3.1 CLI commands and pass/fail results.
- Files changed.
- Tests run.
- Remaining blockers as exact file paths and required next actions.

Do not submit a vague summary. The leader needs auditable proof that the vertical slice works.

<!-- <FILE>docs/new_kernel/V31_VERTICAL_MIGRATION_VALIDATION_PROCESS.md</FILE> - <DESC>Junior-agent procedure for V2 deprecated to v3.1 vertical recipe migration validation</DESC> -->
<!-- <VERS>END OF VERSION: 0.3.0</VERS> -->
