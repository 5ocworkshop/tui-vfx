<!-- <FILE>docs/design/tui-vfx-v3-core-pipeline-readiness-matrix.md</FILE> - <DESC>Fact-based readiness matrix for the current V3 shader/filter/mask/sampler/style/content/shadow/binding lanes.</DESC> -->
<!-- <VERS>VERSION: 0.1.1</VERS> -->
<!-- <WCTX>Keep the lane-by-lane V3 core pipeline status grounded in concrete fixtures, validator commands, and deterministic tests rather than broad summary claims.</WCTX> -->
<!-- <CLOG>0.1.1: correct the scene-layer shadow+filter evidence to cite layer-local filter hash-diff proof.</CLOG> -->

# V3 core pipeline readiness matrix

This is the current as-built matrix for the core V3 pipeline lanes named in
`V3-PIPE01`. It is intentionally narrow:

- repos in scope: `tui-vfx`, `tui-vfx-recipes`
- evidence shape: fixture + exact validator/test command + observed result
- goal: show what is already proven now, and identify the next blocker without
  re-reading the full punch list

## Matrix

| Lane | Current status | Primary fixture(s) | Evidence | What is proven now |
|---|---|---|---|---|
| Shader | Proven on direct compiled V3 path | `recipes/debug_recipes/complex/v3_cross_family_sequence_disjoint.json`, `recipes/debug_recipes/content/content_typewriter_io_filter_shader.json` | E1, E2, E3 | Shader steps lower and render after upstream sampler/filter/content work. |
| Filter | Proven on direct compiled V3 path | `recipes/debug_recipes/complex/v3_cross_family_sequence_disjoint.json`, `recipes/debug_recipes/content/content_typewriter_io_filter_shader.json`, `recipes/debug_recipes/scene/scene_layer_surface_shadow_pipeline.json` | E1, E2, E3, E6 | Filters render in root and scene-layer pipelines, including a scene-layer shadow-bearing surface. |
| Mask | Proven on direct compiled V3 path | `recipes/debug_recipes/complex/v3_cross_family_sequence_disjoint.json` | E1, E2 | Mask lowering and execution work in the ordered cross-family sequence after a filter-sourced output. |
| Sampler | Proven on direct compiled V3 path | `recipes/debug_recipes/complex/v3_cross_family_sequence_disjoint.json` | E1, E2 | Sampler lowering/execution works in the same authored sequence as downstream filter/mask/shader/style lanes. |
| Style | Proven on direct compiled V3 path | `recipes/debug_recipes/complex/v3_cross_family_sequence_disjoint.json`, `recipes/debug_recipes/scene/scene_layer_full_stack.json` | E1, E2, E4 | Role-scoped style work composes after spatial lanes; scene-layer base styling survives on the scene path. |
| Content | Proven before downstream pipeline replay | `recipes/debug_recipes/content/content_typewriter_io_filter_shader.json` | E3 | Content effects resolve source text first, then feed downstream filter/shader execution without a separate glue path. |
| Shadow | Proven on scene-layer/direct-V3 path | `recipes/debug_recipes/scene/scene_layer_full_stack.json`, `recipes/debug_recipes/scene/scene_layer_surface_shadow_pipeline.json` | E4, E6 | Attached shadow survives compiled scene rendering, and the smaller shadow+filter fixture now has direct deterministic proof. |
| Binding | Proven for runtime-driven scene-layer motion | `recipes/debug_recipes/scene/scene_layer_full_stack.json` | E5 | Runtime binding overrides change compiled V3 output through `enter.phase_offset_ms` without changing authored structure. |

## Evidence commands

Run from the repo named on the first line of each item.

### E1 — normalized cross-family topology

Repo: `tui-vfx-recipes`

```bash
cargo run -q -p pipeline-validator -- --explore-normalized recipes/debug_recipes/complex/v3_cross_family_sequence_disjoint.json
```

Observed result:

- exits `0`
- normalized step tree shows one authored `Sequence`
- sequence includes sampler → sampler → filter → mask → shader → style_effect

### E2 — deterministic cross-family render proof

Repo: `tui-vfx-recipes`

```bash
cargo test -q --lib deterministic_render_resolves_cross_family_sequence_disjoint_fixture
```

Observed result:

- exits `0`
- asserts `non_empty_cells == 166`
- asserts `render_hash == 6316889231219963374`
- asserts one mask, one filter, one sampler, and one shader layer on the compiled spec

### E3 — deterministic content-to-pipeline proof

Repo: `tui-vfx-recipes`

```bash
cargo test -q --lib deterministic_render_resolves_content_typewriter_io_filter_shader_fixture
```

Observed result:

- exits `0`
- asserts `non_empty_cells == 140`
- asserts `render_hash == 16235098437429853043`

### E4 — deterministic scene-layer full-stack proof

Repo: `tui-vfx-recipes`

```bash
cargo test -q --lib scene_layer_full_stack_fixture_hash_stays_stable
```

Observed result:

- exits `0`
- asserts `non_empty_cells == 480`
- asserts `render_hash == 9247869877837138478`

### E5 — runtime binding changes compiled V3 output

Repo: `tui-vfx-recipes`

```bash
cargo test -q --lib scene_layer_full_stack_card_phase_offset_binding_changes_entering_render
```

Observed result:

- exits `0`
- default entering render and override render produce different hashes
- binding exercised: `card_phase_offset_ms`

### E6 — direct shadow+filter scene-layer proof

Repo: `tui-vfx-recipes`

```bash
cargo test -q --lib deterministic_render_resolves_scene_layer_surface_shadow_pipeline_fixture
```

Observed result:

- exits `0`
- compiled `card` scene layer carries a layer-local filter leaf
- rendered roles include shadow-tagged cells
- removing the layer-local filter changes the render hash
- fixture path: `recipes/debug_recipes/scene/scene_layer_surface_shadow_pipeline.json`

## Current gap after this matrix pass

The highest-priority remaining core-pipeline blocker is no longer a missing
proof for the named lanes above; it is broader release-gate evidence coverage
for blocked fixtures under `V3-CI02`, especially owner-approved
`render_capture_png` captures for offscreen/role-scope fixtures and GTD
representatives.

<!-- <FILE>docs/design/tui-vfx-v3-core-pipeline-readiness-matrix.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.1</VERS> -->
