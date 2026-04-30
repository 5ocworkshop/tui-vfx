<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_21.md</FILE> - <DESC>Self-generated next packet after K2.20 native compositor closure</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Native player/studio completion: remove false-native source risk and expand operational studio controls.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — define the source-isolated native compositor and descriptor-driven studio control packet.</CLOG> -->

# Phase K2.21 — Source-Isolated Native Playback + Descriptor-Driven Studio Controls

## Review verdict from K2.20

**PROCEED WITH MOMENTUM.**

K2.20 produced the first bounded, real compositor-native path:

```text
RecipeDocument v3.1
  -> PlayerRenderBackendRequest
  -> native CompositionSpec lowering
  -> render_pipeline_with_spec
  -> backend evidence and live studio before/after hashes
```

The K2.20 harness passed for 13 native recipes, 12 non-empty native `CompositionSpec` outputs, zero fallback, and two live studio control mutations.

But it also deliberately left one high-level blocker that can create false confidence if we keep piling effects on top of it:

```text
Native mode still uses the player-rendered IR as the source scene.
That IR may already contain post-effect player adaptations.
```

This packet must remove that ambiguity before expanding the player/studio surface further.

---

## Executive goal

Make native compositor playback honest by isolating the pre-effect source substrate, then use that source-isolated request path to expand the studio into a more operational, descriptor-driven control surface.

Target shape:

```text
RecipeDocument v3.1
  -> source-only player IR
  -> PlayerRenderBackendRequest
  -> native CompositionSpec lowering from recipe graph/effect nodes
  -> compositor render_pipeline_with_spec
  -> CLI playback + ratatui/studio preview
  -> descriptor-driven controls mutate runtime values and visibly change output
```

The packet is complete only when native mode can prove:

```text
1. Its source grid is pre-effect/source-only, not player-post-effect IR.
2. Native success evidence distinguishes source isolation from fallback rendering.
3. Dynamic studio controls are descriptor-derived where descriptor metadata exists, not only signal-name derived.
4. User-runnable commands show animated native playback and before/after studio mutation with color.
```

---

## Non-negotiable rules

- v3.1 pathway only.
- Debug recipes only: `/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes`.
- Exclude deprecated recipes.
- Do not bump schema version; v3.1 is pre-release and not locked.
- Do not use transient packet names in durable code fields, variables, schema values, CLI flags, diagnostics, or public vocabulary.
- Update docs/rustdocs/OFPF metadata/VOCABULARY/checklists only when impacted.
- Use `cargo nextest` for test runs.
- Use `/usr/projects/tui-vfx-recipes/examples/demo.rs` as an oracle/inspiration for a functioning animated browser/player shell, but build around this repo's v3.1 data model and do not copy its old schema structure.

---

## Required outcome A — Source-only render substrate

Add a player-owned render path that renders scenes and sources without applying recipe-level graph effects.

Required properties:

- The source-only path must preserve scene placement, layers, visibility, source adapters, source-local element pipeline behavior only when that behavior is intentionally source-local.
- Recipe-level graph effects must not be applied to the source-only IR.
- The existing post-effect `render_recipe_ir` behavior must remain available for `irResolved` mode and compatibility.
- Native and auto modes must use source-only IR as the compositor source substrate.
- `irResolved` mode must continue to use post-effect player IR.

Required evidence fields or diagnostics:

```text
sourceRenderMode=sourceOnly or equivalent durable wording
nativeSourceIsolated=true for native/auto native attempts
playerIrAlreadyResolved only for irResolved fallback/compatibility paths
```

Use durable, logical names. Do not include packet labels in field names.

---

## Required outcome B — Native no-false-success gate

Update the K2.20 harness or add a successor harness:

```bash
./scripts/k221_source_isolated_native_demo.sh
```

Required artifacts:

```text
/tmp/k221-source-native-results/README.md
/tmp/k221-source-native-results/source_isolation_summary.json
/tmp/k221-source-native-results/native_pass_fail_table.txt
/tmp/k221-source-native-results/native_timeline_hashes.json
/tmp/k221-source-native-results/studio_control_mutations.json
/tmp/k221-source-native-results/user_commands.txt
```

The harness must fail if:

- native mode uses post-effect IR evidence;
- native fallback occurs under `--fail-on-fallback`;
- required native recipes do not report source isolation;
- at least four animated/control recipes do not change backend hashes across time or mutation samples.

---

## Required outcome C — Descriptor-driven studio controls

Expand studio controls beyond signal-name discovery.

The studio must derive controls from descriptor input metadata when available:

- numbers/ranges;
- integers;
- booleans;
- enums/allowed values;
- colors;
- gradients or a documented, user-visible editable subset;
- signal-backed values;
- literal values that can be promoted into runtime overrides without mutating recipe files.

Required user-visible evidence:

- Controls panel shows control id, label, value kind, current value, source effect/input, and whether it is signal-backed, parameter-backed, or runtime override-backed.
- Script mode can mutate at least one number, one enum, one boolean, and one color/control-family value where matching debug recipes exist.
- Mutations rerender and change backend output or produce an explicit no-visual-change diagnostic when the value is semantically accepted but visually neutral.

---

## Required outcome D — Animation/live playback proof

Ensure the CLI/UI commands a user can paste show actual animation, not just single-frame snapshots.

Required commands must be documented in a result file and final memo:

- Animated CLI timeline command for native compositor mode.
- Interactive UI player command using `--backend compositor --composition-mode native --studio`.
- Scripted UI command that mutates controls and proves hash/cell changes.
- A color ANSI render command.

The player should draw from the working patterns in `/usr/projects/tui-vfx-recipes/examples/demo.rs`: browser/preview split, stable ticking, pause/resume, restart/reload, help, and clear focus semantics. Adapt only what fits this repo's current player/studio model.

---

## Required outcome E — Expand native lane coverage only after source isolation

After source isolation is proved, expand native lowering where the compositor API already has a direct mapping. Do not invent fake mappings.

Prioritize remaining debug-recipes effect lanes that already have player adapters or compositor equivalents, then report unsupported effects with exact missing API/model reason.

Required result shape:

```text
effect id
recipe path
native supported? yes/no
reason if no
source-isolated? yes/no
fallback used? yes/no
hash changed across time/control? yes/no
```

---

## Required verification

Minimum gates:

```bash
cargo fmt --package tui-vfx-player --package tui-vfx-player-backend-compositor --package tui-vfx-player-cli --package tui-vfx-player-ui -- --check
cargo check -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui
cargo clippy -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui --all-targets -- -D warnings
cargo nextest run -p tui-vfx-player -p tui-vfx-player-backend-compositor -p tui-vfx-player-cli -p tui-vfx-player-ui --no-fail-fast
./scripts/k221_source_isolated_native_demo.sh
cargo nextest run --workspace --no-fail-fast
```

If a gate is too broad for a rapid loop, run targeted nextest first, then finish with the full gate before closure.

---

## Required final memo shape

Final memo must start with successful user-visible results:

1. Successful source-isolated native playback results.
2. User-runnable player/studio commands.
3. Source isolation proof.
4. Studio control mutation proof.
5. Native effect coverage table.
6. What remains unsupported and why.
7. Verification matrix.
8. Files/crates touched.
9. Review and de-slop results.
10. Next recommended packet or explicit statement that player/studio are now public-demo ready.

Do not start with process. Start with what works.

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_21.md</FILE> - <DESC>Self-generated next packet after K2.20 native compositor closure</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
