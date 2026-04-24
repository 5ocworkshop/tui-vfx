<!-- <FILE>URGENT_TODO.md</FILE> - <DESC>Immediate next-step checklist for finishing full direct-V3 playback support after the Madeira braille-dotfield breakthrough.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Tonight proved the hard part: a recipe-owned braille-dotfield scene source can animate with shared wave-driven displacement and correlated shading. The remaining blocker is broader direct-V3 playback parity, especially fullscreen/opaque-canvas ownership and the edges where demo/play_recipe still fall back or fail. This file is the shortest practical restart point for the next session.</WCTX> -->
<!-- <CLOG>0.1.0: initial urgent todo capturing what landed, what is still missing for full direct-V3 support, and the exact files/seams to inspect next.</CLOG> -->

# URGENT TODO — finish direct V3 playback support

## What is already achieved

These are real as-built capabilities, not plans:

1. **Recipe-owned braille-dotfield scene source exists**
   - `tui-vfx-recipes/src/scene/procedural/sources/cls_braille_dotfield.rs`
   - `tui-vfx-recipes/src/scene/procedural/sources/cls_braille_flag_field.rs`
   - `recipes/madeira_flag/madeira_flag.json`
2. **The flag is no longer an image-like fallback path**
   - the hidden `madeira_flag_rsb` synthetic asset fallback was removed
   - the flag now lives on a procedural source path
3. **Shared wave-driven lighting works**
   - one internal wave drives both displacement and correlated shading
   - the user visually confirmed the lighting is dynamic and wave-driven
4. **Animated scene procedural with lighting is working**
   - this was the hard part
   - it proves the V3 scene/procedural path can carry meaningful animated authored visuals now
5. **Transparent-cell rules are better understood now**
   - transparent procedural cells should reveal the underlay by default
   - some recipes, like Madeira and BSOD-style fullscreen ownership, should deliberately install a full recipe-owned underlay instead
6. **A V3 fullscreen BSOD copy exists**
   - `recipes/bsod_crash_v3.json`
   - use it as the canary for fullscreen-owned background semantics on the direct-V3 path

## The real remaining problem

The schema is not the blocker.

- `layout.mode = "fullscreen"` already exists in V3
- V3 fullscreen recipes already parse and validate

The blocker is:

> **the direct-V3 playback/render path is still not fully semantically equivalent to the legacy fullscreen/item path**

In practice this means:

- some V3 recipes are structurally valid
- but `demo.rs` / `play_recipe.rs` still reject or fall back because the current direct-V3 subset is incomplete
- fullscreen-owned background semantics are still fragile
- bounded transparent-overlay semantics and fullscreen opaque-ownership semantics are not fully unified yet

## Immediate symptoms to fix

### 1. Direct V3 still fails for some valid fullscreen recipes

Observed symptom:
- demo/playback can say some variant of
  - `legacy playback is not available for v3 recipes`

Meaning:
- direct V3 construction failed
- then fallback tried the legacy runtime path
- but there was no paired `_DEPRECATED_...json` recipe to bridge through

This is **not** a schema failure.
It is a **direct-V3 support gap**.

### 2. Opaque background ownership is still unreliable

There are two different desired behaviors and the renderer must support both:

#### A. Transparent overlay mode
- flag/fireworks/decorative procedurals reveal the existing canvas
- blank transparent cells should not stamp black

#### B. Full-canvas ownership mode
- recipe intentionally owns the entire viewport/canvas
- blank cells with opaque background must still overwrite the underlay
- `bsod_crash.json` is the legacy proof of this behavior
- `bsod_crash_v3.json` is the V3 canary for this behavior

Right now the direct-V3 snapshot path is still too easy to get wrong around:
- blank glyph + opaque bg
- transparent glyph + transparent bg
- preserving or replacing underlay correctly

### 3. The direct-V3 subset boundary is still implicit

`preview_from_recipe_path_with_cutover_fallback(...)` in demo/play_recipe does:
1. try direct V3
2. if that fails, try legacy runtime fallback

What is missing is a cleaner explicit answer to:
- *why* did direct V3 fail for this recipe?
- which exact capability gate rejected it?
- can we expose that as a diagnostic instead of generic fallback behavior?

## Highest-value next actions

### A. Add a failing regression for direct V3 fullscreen ownership

Add a focused test that proves whether a V3 fullscreen recipe can reach:
- `DirectV3PreviewState::from_compiled(...)`
- `preview_from_recipe_path_with_cutover_fallback(...)`
- `demo.rs` direct-V3 path

Start with:
- `recipes/bsod_crash_v3.json`

Expected behavior:
- direct V3 path should succeed
- it should **not** fall back to legacy
- fullscreen blue background should be recipe-owned in the direct snapshot path

Best locations:
- `tui-vfx-recipes/src/preview/fnc_preview_from_recipe_path.rs`
- `tui-vfx-recipes/src/preview/cls_direct_v3_preview_state.rs`
- maybe a new focused demo/playback regression if needed

### B. Reconcile direct snapshot composition with fullscreen ownership

Inspect:
- `tui-vfx-recipes/src/preview/fnc_render_direct_v3_snapshot.rs`
- `tui-vfx-recipes/src/scene/layers/cls_procedural_layer.rs`
- `tui-vfx-recipes/src/compat.rs`

Need one clean rule:

- blank + transparent background ⇒ preserve underlay
- blank + opaque background ⇒ overwrite underlay background
- visible glyph + transparent background ⇒ preserve underlay background but replace glyph/fg
- visible glyph + opaque background ⇒ full overwrite

The old V2 fullscreen path already behaves correctly for BSOD-style recipes.
The direct-V3 path must match that behavior where appropriate.

### C. Make the direct-V3 support boundary explicit in code/docs

Document and/or encode exactly what the direct-V3 preview path supports today.

At minimum, the following should be made explicit:
- root-only direct path vs scene-bearing path
- fullscreen-owned recipes
- transparent bounded recipes
- procedural scene layers
- layer-local pipelines
- scene-layer background/base-style ownership semantics

Best places:
- `tui-vfx/docs/design/tui-vfx-v3-compiled-execution-plan.md`
- `tui-vfx/docs/design/tui-vfx-v3-upgrade-plan/100_tooling_ci_migration.md`
- `tui-vfx-recipes/docs/scene/PROCEDURAL_SOURCES.md`
- possibly a new small note in `preview_from_recipe_path_with_cutover_fallback` rustdoc

## Architectural caveats to keep in mind

These are important but not the first blocker tomorrow:

1. **Braille-dotfield toolkit extraction is only half done**
   - primitives exist
   - displacement/shading/overscan logic still lives too much inside `braille_flag_field`
2. **Flag geometry is still not fully recipe-authored**
   - colors/wave/shading are in recipe params
   - stripe/cross geometry is still hardcoded in Rust
3. **Overscan is currently modeled through preferred area inflation**
   - this works
   - but a more honest future seam is source render extent / bleed rather than nominal placement size inflation

These should be fixed, but **after** the direct-V3 playback path itself is trustworthy for fullscreen and bounded ownership semantics.

## Files to inspect first tomorrow

### Playback / fallback entry points
- `tui-vfx-recipes/src/preview/fnc_preview_from_recipe_path.rs`
- `tui-vfx-recipes/examples/demo.rs`
- `tui-vfx-recipes/examples/play_recipe.rs`

### Direct V3 preview/render path
- `tui-vfx-recipes/src/preview/cls_direct_v3_preview_state.rs`
- `tui-vfx-recipes/src/preview/fnc_render_direct_v3_snapshot.rs`
- `tui-vfx-recipes/src/v3/compile/fnc_render_compiled_plan_deterministically.rs`
- `tui-vfx-recipes/src/v3/compile/cls_v3_playback_timing.rs`

### Scene/procedural ownership path
- `tui-vfx-recipes/src/scene/layers/cls_procedural_layer.rs`
- `tui-vfx-recipes/src/scene/procedural/sources/cls_braille_dotfield.rs`
- `tui-vfx-recipes/src/scene/procedural/sources/cls_braille_flag_field.rs`
- `tui-vfx-recipes/src/scene/procedural/sources/cls_ballistic_fireworks.rs`

### Semantic reference recipes
- `tui-vfx-recipes/recipes/madeira_flag/madeira_flag.json`
- `tui-vfx-recipes/recipes/bsod_crash_v3.json`
- `tui-vfx-recipes/recipes/bsod_crash.json`
- `tui-vfx-recipes/recipes/wargames/themes/new_wopr_fullscreen_cyan.json`

## Tomorrow’s shortest path

1. Prove whether `bsod_crash_v3.json` reaches direct V3 or not
2. If not, capture the exact failure with a regression test
3. Fix fullscreen-owned background semantics in the direct-V3 path
4. Re-test `bsod_crash_v3.json` in demo/play_recipe
5. Re-test `madeira_flag.json`
6. Only then return to the broader toolkit cleanup

## Bottom line

The hardest new capability is already real:

> **animated braille-dotfield scene source with shared wave-driven lighting**

What remains is mostly **playback semantics and support-boundary cleanup**, not invention of a whole new visual system.

That is good news.

<!-- <FILE>URGENT_TODO.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->

## Added note — file-backed braille source support still missing

One more missing capability to carry tomorrow:

- the current `braille_flag_field` source **does not read the base braille flag from a file**
- it currently draws the base flag geometry procedurally in Rust
- there is **not yet** a generic path to:
  - load a braille-dotfield source from disk
  - decode it into the dotfield representation
  - run it through the same displacement / shading / emission path

Why this matters:
- the whole point of the new path is that future recipes should be able to
  swap the authored braille source without rewriting Rust
- Madeira is working as a first consumer, but the reusable path is still only
  half done until file-backed braille-dotfield input exists too

When you resume, treat this as a follow-on after the direct-V3 fullscreen /
background-ownership semantics are trustworthy.
