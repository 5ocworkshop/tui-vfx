#!/usr/bin/env bash
# <FILE>scripts/k221_source_isolated_native_demo.sh</FILE> - <DESC>Source-isolated native compositor demo and verification harness</DESC>
# <VERS>VERSION: 0.1.0</VERS>
# <WCTX>Source-isolated native playback: prove native mode uses source-only IR and descriptor-driven studio controls mutate output.</WCTX>
# <CLOG>0.1.0: INIT — emit K2.21 source-isolation, timeline, and studio-control artifacts.</CLOG>
set -euo pipefail

ROOT="${ROOT:-/usr/projects/tui-vfx}"
RECIPES_ROOT="${RECIPES_ROOT:-/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes}"
OUT="${OUT:-/tmp/k221-source-native-results}"
PACK="$ROOT/descriptors/v3.1/packs/primitive.json"
CLI=(cargo run -q -p tui-vfx-player-cli --)
UI=(cargo run -q -p tui-vfx-player-ui --)

cd "$ROOT"
rm -rf "$OUT"
mkdir -p "$OUT"

recipe() { printf '%s/%s' "$RECIPES_ROOT" "$1"; }

render_native_json() {
  local relative="$1" out_file="$2"
  "${CLI[@]}" render-backend \
    --recipe "$(recipe "$relative")" \
    --descriptor-pack "$PACK" \
    --backend compositor \
    --composition-mode native \
    --fail-on-fallback \
    --format json > "$out_file"
}

timeline_native_json() {
  local relative="$1" out_file="$2"
  "${CLI[@]}" render-backend-timeline \
    --recipe "$(recipe "$relative")" \
    --descriptor-pack "$PACK" \
    --backend compositor \
    --composition-mode native \
    --fail-on-fallback \
    --format json \
    --samples 5 > "$out_file"
}

assert_native_source_isolated() {
  local file="$1"
  jq -e '
    .compositionMode == "native" and
    .fallbackUsed == false and
    .nativeLoweringSucceeded == true and
    .sourceRenderMode == "sourceOnly" and
    .nativeSourceIsolated == true and
    ([.diagnostics[].code] | index("playerIrAlreadyResolved") | not)
  ' "$file" >/dev/null
}

assert_timeline_changes() {
  local file="$1"
  jq -e '
    (.samples | length) >= 2 and
    ([.samples[].sourceRenderMode] | all(. == "sourceOnly")) and
    ([.samples[].nativeSourceIsolated] | all(. == true)) and
    ([.samples[].fallbackUsed] | all(. == false)) and
    ([.samples[].backendHash] | unique | length) > 1
  ' "$file" >/dev/null
}

render_native_json "filters/filter_tint.json" "$OUT/native_filter_tint.json"
render_native_json "shaders/primitives/shader_linear_gradient_apply_to_both.json" "$OUT/native_linear_gradient.json"
render_native_json "masks/mask_wipe.json" "$OUT/native_mask_wipe.json"
render_native_json "samplers/sampler_sinewave.json" "$OUT/native_sampler_sinewave.json"
render_native_json "shaders/compositions/shader_border_sweep_position_binding.json" "$OUT/native_border_sweep.json"

for file in "$OUT"/native_*.json; do
  assert_native_source_isolated "$file"
done

"${CLI[@]}" render-backend \
  --recipe "$(recipe "shaders/primitives/shader_linear_gradient_apply_to_both.json")" \
  --descriptor-pack "$PACK" \
  --backend compositor \
  --composition-mode ir-resolved \
  --format json > "$OUT/ir_resolved_compat.json"
jq -e '.compositionMode == "irResolved" and .sourceRenderMode == "postEffectIr" and .nativeSourceIsolated == false and ([.diagnostics[].code] | index("playerIrAlreadyResolved"))' "$OUT/ir_resolved_compat.json" >/dev/null

timeline_native_json "masks/mask_wipe.json" "$OUT/native_mask_wipe_timeline.json"
timeline_native_json "samplers/sampler_sinewave.json" "$OUT/native_sampler_sinewave_timeline.json"
timeline_native_json "shaders/compositions/shader_border_sweep_position_binding.json" "$OUT/native_border_sweep_timeline.json"
for file in "$OUT"/*_timeline.json; do
  assert_timeline_changes "$file"
done

"${CLI[@]}" studio-snapshot \
  --recipe "$(recipe "filters/filter_pill_button_progress_binding.json")" \
  --descriptor-pack "$PACK" \
  --backend compositor \
  --composition-mode native \
  --fail-on-fallback \
  --set progress=0.25 \
  --json > "$OUT/studio_signal_mutation.json"
jq -e '.changedCells > 0 and .before.sourceRenderMode == "sourceOnly" and .after.sourceRenderMode == "sourceOnly" and (.mutations[] | select(.targetKind == "signal" and .signalId == "pillProgress"))' "$OUT/studio_signal_mutation.json" >/dev/null

"${CLI[@]}" studio-snapshot \
  --recipe "$(recipe "filters/filter_pill_button_progress_binding.json")" \
  --descriptor-pack "$PACK" \
  --backend compositor \
  --composition-mode native \
  --fail-on-fallback \
  --set 'effect:filter.pillButton:effectNode:activeColor=#ff0000' \
  --json > "$OUT/studio_descriptor_color_mutation.json"
jq -e '.changedCells > 0 and .after.sourceRenderMode == "sourceOnly" and .after.nativeSourceIsolated == true and (.mutations[] | select(.targetKind == "runtimeInputOverride" and .runtimeInput == "effect:filter.pillButton:effectNode:activeColor")) and (.controls[] | select(.id == "effect:filter.pillButton:effectNode:activeColor" and .controlKind == "colorPicker"))' "$OUT/studio_descriptor_color_mutation.json" >/dev/null

"${CLI[@]}" studio-snapshot \
  --recipe "$(recipe "baseline.json")" \
  --descriptor-pack "$PACK" \
  --backend compositor \
  --composition-mode native \
  --fail-on-fallback \
  --set 'source:source.card:mainCard:width=20' \
  --json > "$OUT/studio_source_integer_mutation.json"
jq -e '.changedCells > 0 and .after.sourceRenderMode == "sourceOnly" and (.mutations[] | select(.targetKind == "runtimeInputOverride" and .runtimeInput == "source:source.card:mainCard:width"))' "$OUT/studio_source_integer_mutation.json" >/dev/null

"${CLI[@]}" studio-snapshot \
  --recipe "$(recipe "scene/scene_layer_visibility_binding_io.json")" \
  --descriptor-pack "$PACK" \
  --backend compositor \
  --composition-mode native \
  --fail-on-fallback \
  --set showPrimaryLayer=false \
  --json > "$OUT/studio_boolean_mutation.json"
jq -e '.changedCells > 0 and .after.sourceRenderMode == "sourceOnly" and (.mutations[] | select(.targetKind == "signal" and .signalId == "showPrimaryLayer" and .value.value == false))' "$OUT/studio_boolean_mutation.json" >/dev/null

"${CLI[@]}" studio-snapshot \
  --recipe "$(recipe "masks/mask_wipe.json")" \
  --descriptor-pack "$PACK" \
  --backend compositor \
  --composition-mode native \
  --fail-on-fallback \
  --set maskWipeEnter.direction=rightToLeft \
  --json > "$OUT/studio_enum_mutation_no_visual_change.json"
jq -e '.changedCells == 0 and (.mutations[] | select(.targetKind == "runtimeInputOverride" and .runtimeInput == "effect:mask.wipe:maskWipeEnter:direction")) and (.studioDiagnostics[] | select(.code == "studioMutationNoVisualChange"))' "$OUT/studio_enum_mutation_no_visual_change.json" >/dev/null

"${UI[@]}" \
  --descriptor-pack "$PACK" \
  --recipes-root "$RECIPES_ROOT" \
  --recipe "$(recipe "filters/filter_pill_button_progress_binding.json")" \
  --backend compositor \
  --composition-mode native \
  --fail-on-fallback \
  --studio \
  --script 'set effect:filter.pillButton:effectNode:activeColor=#ff0000; render; quit' \
  --no-clear > "$OUT/studio_descriptor_control_ui.txt"
grep -q 'control: colorPicker' "$OUT/studio_descriptor_control_ui.txt"
grep -q 'target: runtimeInputOverride' "$OUT/studio_descriptor_control_ui.txt"
grep -q $'\033\[38;2;' "$OUT/studio_descriptor_control_ui.txt"

"${CLI[@]}" render-backend-timeline \
  --recipe "$(recipe "shaders/compositions/shader_border_sweep_position_binding.json")" \
  --descriptor-pack "$PACK" \
  --backend compositor \
  --composition-mode native \
  --fail-on-fallback \
  --format ansi \
  --samples 3 \
  --no-clear > "$OUT/native_border_sweep_timeline.ansi"
grep -q 'backend_hash=' "$OUT/native_border_sweep_timeline.ansi"
grep -q $'\033\[38;2;' "$OUT/native_border_sweep_timeline.ansi"

jq -n \
  --slurpfile tint "$OUT/native_filter_tint.json" \
  --slurpfile gradient "$OUT/native_linear_gradient.json" \
  --slurpfile wipe "$OUT/native_mask_wipe.json" \
  --slurpfile sine "$OUT/native_sampler_sinewave.json" \
  --slurpfile border "$OUT/native_border_sweep.json" \
  --slurpfile signal "$OUT/studio_signal_mutation.json" \
  --slurpfile color "$OUT/studio_descriptor_color_mutation.json" \
  --slurpfile integer "$OUT/studio_source_integer_mutation.json" \
  --slurpfile boolean "$OUT/studio_boolean_mutation.json" \
  --slurpfile enum "$OUT/studio_enum_mutation_no_visual_change.json" \
  '{
    schemaVersion: "v3.1.player.sourceIsolatedNativeHarness.1",
    nativeRecipeCount: 5,
    fallbackCount: ([ $tint[0], $gradient[0], $wipe[0], $sine[0], $border[0] ] | map(select(.fallbackUsed == true)) | length),
    sourceIsolatedCount: ([ $tint[0], $gradient[0], $wipe[0], $sine[0], $border[0] ] | map(select(.sourceRenderMode == "sourceOnly" and .nativeSourceIsolated == true)) | length),
    changedEvidenceCount: 7,
    acceptedNoVisualChangeCount: ([ $enum[0] ] | map(select((.studioDiagnostics // []) | any(.code == "studioMutationNoVisualChange"))) | length),
    studioMutations: [$signal[0].mutations[], $color[0].mutations[], $integer[0].mutations[], $boolean[0].mutations[], $enum[0].mutations[]]
  }' > "$OUT/source_isolation_summary.json"

cat > "$OUT/native_pass_fail_table.txt" <<TABLE
recipe_or_artifact | gate | result
filters/filter_tint.json | native source-isolated render | PASS
shaders/primitives/shader_linear_gradient_apply_to_both.json | native source-isolated render | PASS
masks/mask_wipe.json | native source-isolated render + timeline hash drift | PASS
samplers/sampler_sinewave.json | native source-isolated render + timeline hash drift | PASS
shaders/compositions/shader_border_sweep_position_binding.json | native source-isolated render + timeline hash drift | PASS
studio_signal_mutation.json | signal-backed control mutates output | PASS
studio_descriptor_color_mutation.json | descriptor runtime color control mutates output | PASS
studio_source_integer_mutation.json | source runtime integer control mutates output | PASS
studio_boolean_mutation.json | boolean signal control mutates output | PASS
studio_enum_mutation_no_visual_change.json | enum runtime control accepted with explicit no-visual-change diagnostic | PASS
ir_resolved_compat.json | post-effect IR compatibility retained | PASS
TABLE

jq -n \
  --slurpfile wipe "$OUT/native_mask_wipe_timeline.json" \
  --slurpfile sine "$OUT/native_sampler_sinewave_timeline.json" \
  --slurpfile border "$OUT/native_border_sweep_timeline.json" \
  '{
    maskWipeHashes: ($wipe[0].samples | map(.backendHash) | unique),
    samplerSinewaveHashes: ($sine[0].samples | map(.backendHash) | unique),
    borderSweepHashes: ($border[0].samples | map(.backendHash) | unique)
  }' > "$OUT/native_timeline_hashes.json"

jq -n \
  --slurpfile signal "$OUT/studio_signal_mutation.json" \
  --slurpfile color "$OUT/studio_descriptor_color_mutation.json" \
  --slurpfile integer "$OUT/studio_source_integer_mutation.json" \
  --slurpfile boolean "$OUT/studio_boolean_mutation.json" \
  --slurpfile enum "$OUT/studio_enum_mutation_no_visual_change.json" \
  '{signalMutation: $signal[0], descriptorColorMutation: $color[0]}' > "$OUT/studio_control_mutations.json"

cat > "$OUT/user_commands.txt" <<COMMANDS
cd /usr/projects/tui-vfx && ./scripts/k221_source_isolated_native_demo.sh
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-cli -- render-backend-timeline --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json --descriptor-pack descriptors/v3.1/packs/primitive.json --backend compositor --composition-mode native --fail-on-fallback --format ansi --samples 3 --no-clear
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-ui -- --descriptor-pack descriptors/v3.1/packs/primitive.json --recipes-root /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_pill_button_progress_binding.json --backend compositor --composition-mode native --fail-on-fallback --studio --script 'set effect:filter.pillButton:effectNode:activeColor=#ff0000; render; quit' --no-clear
COMMANDS

cat > "$OUT/README.md" <<README
# K2.21 source-isolated native compositor results

This directory proves native compositor mode uses source-only player IR, while irResolved mode remains post-effect IR compatibility.

Key artifacts:
- source_isolation_summary.json
- native_pass_fail_table.txt
- native_timeline_hashes.json
- studio_control_mutations.json
- studio_descriptor_control_ui.txt
- native_border_sweep_timeline.ansi
README

echo "K2.21 source-isolated native demo PASS. Results: $OUT"
# <FILE>scripts/k221_source_isolated_native_demo.sh</FILE> - <DESC>Source-isolated native compositor demo and verification harness</DESC>
# <VERS>END OF VERSION: 0.1.0</VERS>
