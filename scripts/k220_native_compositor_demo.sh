#!/usr/bin/env bash
# <FILE>scripts/k220_native_compositor_demo.sh</FILE> - <DESC>K2.20 native compositor demo and evidence harness</DESC>
# <VERS>VERSION: 0.1.0</VERS>
# <WCTX>Native compositor lowering: generate pass/fail evidence for native CompositionSpec lowering and studio controls.</WCTX>
# <CLOG>0.1.0: INIT — render required native debug recipes, timelines, ANSI output, and studio before/after artifacts.</CLOG>

set -euo pipefail

REPO_ROOT=${REPO_ROOT:-/usr/projects/tui-vfx}
RECIPE_REPO=${RECIPE_REPO:-/usr/projects/tui-vfx-recipes}
RECIPE_ROOT=${RECIPE_ROOT:-$RECIPE_REPO/recipes/v3.1/debug_recipes}
OUT=${OUT:-/tmp/k220-native-results}
export OUT
DESCRIPTOR_PACK=${DESCRIPTOR_PACK:-$REPO_ROOT/descriptors/v3.1/packs/primitive.json}
CLI="$REPO_ROOT/target/debug/tui-vfx-player-cli"
UI="$REPO_ROOT/target/debug/tui-vfx-player-ui"

mkdir -p "$OUT"
cd "$REPO_ROOT"
cargo build -q -p tui-vfx-player-cli -p tui-vfx-player-ui

render_native_json() {
  local recipe=$1
  local artifact=$2
  "$CLI" render-backend \
    --descriptor-pack "$DESCRIPTOR_PACK" \
    --backend compositor \
    --composition-mode native \
    --fail-on-fallback \
    --format json \
    "$RECIPE_ROOT/$recipe" > "$OUT/$artifact"
}

render_native_ansi() {
  local recipe=$1
  local artifact=$2
  "$CLI" render-backend \
    --descriptor-pack "$DESCRIPTOR_PACK" \
    --backend compositor \
    --composition-mode native \
    --fail-on-fallback \
    --format ansi \
    "$RECIPE_ROOT/$recipe" > "$OUT/$artifact"
}

render_native_timeline() {
  local recipe=$1
  local artifact=$2
  "$CLI" render-backend-timeline \
    --descriptor-pack "$DESCRIPTOR_PACK" \
    --backend compositor \
    --composition-mode native \
    --fail-on-fallback \
    --format json \
    --samples 5 \
    "$RECIPE_ROOT/$recipe" > "$OUT/$artifact"
}

render_studio_text() {
  local recipe=$1
  local artifact=$2
  local script=${3:-render}
  "$UI" \
    --descriptor-pack "$DESCRIPTOR_PACK" \
    --recipes-root "$RECIPE_ROOT" \
    --recipe "$RECIPE_ROOT/$recipe" \
    --backend compositor \
    --composition-mode native \
    --fail-on-fallback \
    --studio \
    --script "$script" \
    --no-clear > "$OUT/$artifact"
}

render_studio_json() {
  local recipe=$1
  local artifact=$2
  local assignment=$3
  "$CLI" studio-snapshot \
    --descriptor-pack "$DESCRIPTOR_PACK" \
    --backend compositor \
    --composition-mode native \
    --fail-on-fallback \
    --set "$assignment" \
    "$RECIPE_ROOT/$recipe" > "$OUT/$artifact"
}

recipes=(
  "baseline.json:native_baseline.json:baseline"
  "filters/filter_tint.json:native_filter_tint.json:filter.tint"
  "filters/filter_dim.json:native_filter_dim.json:filter.dim"
  "masks/mask_wipe.json:native_mask_wipe.json:mask.wipe"
  "masks/mask_checkers.json:native_mask_checkers.json:mask.checkers"
  "samplers/sampler_sinewave.json:native_sampler_sinewave.json:sampler.sineWave"
  "samplers/sampler_ripple.json:native_sampler_ripple.json:sampler.ripple"
  "shaders/primitives/shader_linear_gradient_apply_to_both.json:native_linear_gradient_apply_to_both.json:shader.linearGradient"
  "shaders/primitives/shader_linear_gradient_diagonal.json:native_linear_gradient_diagonal.json:shader.linearGradient"
  "shaders/compositions/shader_border_sweep_position_binding.json:native_border_sweep.json:shader.borderSweep"
  "styles/style_fade_in.json:native_style_fade_in.json:style.fadeIn"
  "styles/style_fade_out.json:native_style_fade_out.json:style.fadeOut"
  "filters/filter_pill_button_progress_binding.json:native_pill_button.json:filter.pillButton"
)

: > "$OUT/native_pass_fail_table.txt"
printf '%-68s | %-24s | %-6s | %-8s | %-34s | %s\n' "recipe" "effects lowered" "native" "fallback" "artifact" "result" >> "$OUT/native_pass_fail_table.txt"
printf '%-68s-+-%-24s-+-%-6s-+-%-8s-+-%-34s-+-%s\n' "--------------------------------------------------------------------" "------------------------" "------" "--------" "----------------------------------" "------" >> "$OUT/native_pass_fail_table.txt"

for entry in "${recipes[@]}"; do
  IFS=: read -r recipe artifact effect <<< "$entry"
  render_native_json "$recipe" "$artifact"
  jq -e '.backend == "compositor" and .compositionMode == "native" and .fallbackUsed == false and .nativeLoweringAttempted == true and .nativeLoweringSucceeded == true' "$OUT/$artifact" >/dev/null
  if [[ "$recipe" != "baseline.json" ]]; then
    jq -e '.compositionSpecNonEmpty == true and .loweredNodeCount > 0 and (.loweredEffectIds | length > 0)' "$OUT/$artifact" >/dev/null
    jq -e --arg effect "$effect" '.loweredEffectIds | index($effect)' "$OUT/$artifact" >/dev/null
  fi
  printf '%-68s | %-24s | %-6s | %-8s | %-34s | PASS\n' "$recipe" "$effect" "yes" "no" "$artifact" >> "$OUT/native_pass_fail_table.txt"
done

render_native_ansi "shaders/primitives/shader_linear_gradient_apply_to_both.json" "native_linear_gradient.ansi"
cp "$OUT/native_linear_gradient_apply_to_both.json" "$OUT/native_linear_gradient.json"
render_native_timeline "masks/mask_wipe.json" "native_mask_wipe_timeline.json"
render_native_timeline "shaders/compositions/shader_border_sweep_position_binding.json" "native_border_sweep_timeline.json"
render_native_timeline "samplers/sampler_sinewave.json" "native_sampler_sinewave_timeline.json"

jq -e '(.samples | length) == 5 and ([.samples[].backendHash] | unique | length > 1) and all(.samples[]; .fallbackUsed == false)' "$OUT/native_mask_wipe_timeline.json" >/dev/null
jq -e '(.samples | length) == 5 and ([.samples[].backendHash] | unique | length > 1) and all(.samples[]; .fallbackUsed == false)' "$OUT/native_border_sweep_timeline.json" >/dev/null
jq -e '(.samples | length) == 5 and ([.samples[].backendHash] | unique | length > 1) and all(.samples[]; .fallbackUsed == false)' "$OUT/native_sampler_sinewave_timeline.json" >/dev/null

render_studio_text "shaders/compositions/shader_border_sweep_position_binding.json" "studio_border_sweep_before.txt" "render"
render_studio_text "shaders/compositions/shader_border_sweep_position_binding.json" "studio_border_sweep_after.txt" "set position=0.75; render"
cp "$OUT/studio_border_sweep_after.txt" "$OUT/studio_live_border_sweep.txt"
render_studio_json "shaders/compositions/shader_border_sweep_position_binding.json" "studio_border_sweep_before_after.json" "position=0.75"
cp "$OUT/studio_border_sweep_before_after.json" "$OUT/studio_before_after.json"

render_studio_text "filters/filter_pill_button_progress_binding.json" "studio_live_pill_button.txt" "set progress=0.25; render"
render_studio_json "filters/filter_pill_button_progress_binding.json" "studio_pill_button_before_after.json" "progress=0.25"
render_studio_text "shaders/primitives/shader_linear_gradient_apply_to_both.json" "studio_gradient_controls.txt" "render"

jq -e '.beforeBackendHash != .afterBackendHash and .changedCells > 0 and .before.fallbackUsed == false and .after.fallbackUsed == false' "$OUT/studio_border_sweep_before_after.json" >/dev/null
jq -e '.beforeBackendHash != .afterBackendHash and .changedCells > 0 and .before.fallbackUsed == false and .after.fallbackUsed == false' "$OUT/studio_pill_button_before_after.json" >/dev/null
grep -q "sweepPosition" "$OUT/studio_live_border_sweep.txt"
grep -q "Pillprogress" "$OUT/studio_live_pill_button.txt"

required_json_files=(
  "$OUT/native_baseline.json"
  "$OUT/native_filter_tint.json"
  "$OUT/native_filter_dim.json"
  "$OUT/native_mask_wipe.json"
  "$OUT/native_mask_checkers.json"
  "$OUT/native_sampler_sinewave.json"
  "$OUT/native_sampler_ripple.json"
  "$OUT/native_linear_gradient_apply_to_both.json"
  "$OUT/native_linear_gradient_diagonal.json"
  "$OUT/native_border_sweep.json"
  "$OUT/native_style_fade_in.json"
  "$OUT/native_style_fade_out.json"
  "$OUT/native_pill_button.json"
)
jq -s '{
  schemaVersion: "v3.1.k220.nativeSummary.1",
  nativeRecipeCount: length,
  nativeNonEmptySpecCount: map(select(.compositionSpecNonEmpty == true)) | length,
  fallbackCount: map(select(.fallbackUsed == true)) | length,
  nativeLoweringSucceededCount: map(select(.nativeLoweringSucceeded == true)) | length,
  loweredEffectIds: (map(.loweredEffectIds[]?) | unique),
  artifactsRoot: env.OUT
}' "${required_json_files[@]}" > "$OUT/native_summary.json"
cp "$OUT/native_summary.json" "$OUT/native_required_summary.json"

cat > "$OUT/README.md" <<README
# K2.20 native compositor results

Generated by \`scripts/k220_native_compositor_demo.sh\`.

- Native summary: \`native_summary.json\`
- Pass/fail table: \`native_pass_fail_table.txt\`
- Linear gradient ANSI proof: \`native_linear_gradient.ansi\`
- Timeline proofs: \`native_mask_wipe_timeline.json\`, \`native_border_sweep_timeline.json\`, \`native_sampler_sinewave_timeline.json\`
- Studio proofs: \`studio_border_sweep_before_after.json\`, \`studio_pill_button_before_after.json\`, \`studio_live_border_sweep.txt\`, \`studio_live_pill_button.txt\`
README

jq -e '.nativeRecipeCount >= 12 and .nativeNonEmptySpecCount >= 8 and .fallbackCount == 0' "$OUT/native_summary.json" >/dev/null
printf 'K2.20 native compositor demo PASS. Results: %s\n' "$OUT"

# <FILE>scripts/k220_native_compositor_demo.sh</FILE> - <DESC>K2.20 native compositor demo and evidence harness</DESC>
# <VERS>END OF VERSION: 0.1.0</VERS>
