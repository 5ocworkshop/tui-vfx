#!/usr/bin/env bash
# <FILE>scripts/k222_native_coverage_audit.sh</FILE> - <DESC>Audit source-isolated native compositor coverage across v3.1 debug recipes</DESC>
# <VERS>VERSION: 0.1.0</VERS>
# <WCTX>K2.22 native coverage: classify every non-deprecated v3.1 debug recipe by native compositor fallback and unsupported effects.</WCTX>
# <CLOG>0.1.0: INIT — render every non-deprecated debug recipe in auto mode and summarize native pass/fallback blockers.</CLOG>
set -euo pipefail

ROOT="${ROOT:-/usr/projects/tui-vfx}"
RECIPES_ROOT="${RECIPES_ROOT:-/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes}"
RESULTS_ROOT="${RESULTS_ROOT:-/tmp/k222-native-coverage-results}"
DESCRIPTOR_PACK="${DESCRIPTOR_PACK:-$ROOT/descriptors/v3.1/packs/primitive.json}"

rm -rf "$RESULTS_ROOT"
mkdir -p "$RESULTS_ROOT/rendered"

find "$RECIPES_ROOT" -type f -name '*.json' ! -iname '*DEPRECATED*' | sort > "$RESULTS_ROOT/recipe_paths.txt"

while IFS= read -r recipe_path; do
  relative_path="${recipe_path#$RECIPES_ROOT/}"
  artifact_name="${relative_path//\//__}"
  artifact_name="${artifact_name%.json}.json"
  output_path="$RESULTS_ROOT/rendered/$artifact_name"
  error_path="$RESULTS_ROOT/rendered/${artifact_name%.json}.stderr"
  if cargo run -q -p tui-vfx-player-cli -- render-backend \
    --recipe "$recipe_path" \
    --descriptor-pack "$DESCRIPTOR_PACK" \
    --backend compositor \
    --composition-mode auto \
    --format json > "$output_path" 2> "$error_path"; then
    rm -f "$error_path"
  else
    printf '{"recipePath":%s,"hardError":true,"stderr":%s}\n' \
      "$(python3 -c 'import json,sys; print(json.dumps(sys.argv[1]))' "$recipe_path")" \
      "$(python3 -c 'import json,sys,pathlib; print(json.dumps(pathlib.Path(sys.argv[1]).read_text()))' "$error_path")" \
      > "$output_path"
  fi
done < "$RESULTS_ROOT/recipe_paths.txt"

python3 - "$RESULTS_ROOT" "$RECIPES_ROOT" <<'PY'
import json, pathlib, sys, collections
results_root=pathlib.Path(sys.argv[1])
recipes_root=pathlib.Path(sys.argv[2])
records=[]
unsupported=collections.Counter()
lowered=collections.Counter()
hard_errors=[]
fallbacks=[]
passes=[]
for output in sorted((results_root/'rendered').glob('*.json')):
    data=json.loads(output.read_text())
    recipe_path=pathlib.Path(data.get('recipePath',''))
    rel=str(recipe_path.relative_to(recipes_root)) if recipe_path and recipe_path.is_absolute() and recipes_root in recipe_path.parents else data.get('recipePath', output.name)
    if data.get('hardError'):
        hard_errors.append(rel)
        records.append({'recipe':rel,'result':'hardError','unsupportedEffects':[],'loweredEffects':[],'sourceRenderMode':None,'nativeSourceIsolated':False})
        continue
    unlowered=data.get('unloweredEffectIds') or []
    lowered_ids=data.get('loweredEffectIds') or []
    for effect in unlowered: unsupported[effect]+=1
    for effect in lowered_ids: lowered[effect]+=1
    fallback=bool(data.get('fallbackUsed'))
    result='fallback' if fallback else 'nativePass'
    if fallback: fallbacks.append(rel)
    else: passes.append(rel)
    records.append({
        'recipe': rel,
        'result': result,
        'fallbackUsed': fallback,
        'compositionMode': data.get('compositionMode'),
        'sourceRenderMode': data.get('sourceRenderMode'),
        'nativeSourceIsolated': data.get('nativeSourceIsolated'),
        'loweredEffects': lowered_ids,
        'unsupportedEffects': unlowered,
        'diagnosticCodes': [d.get('code') for d in data.get('diagnostics',[])],
    })
summary={
    'schemaVersion':'v3.1.player.nativeCoverageAudit.1',
    'recipeCount': len(records),
    'nativePassCount': len(passes),
    'fallbackCount': len(fallbacks),
    'hardErrorCount': len(hard_errors),
    'unsupportedEffectCounts': [{'effect':k,'count':v} for k,v in sorted(unsupported.items(), key=lambda kv:(-kv[1],kv[0]))],
    'loweredEffectCounts': [{'effect':k,'count':v} for k,v in sorted(lowered.items(), key=lambda kv:(-kv[1],kv[0]))],
    'nativePasses': passes,
    'fallbacks': fallbacks,
    'hardErrors': hard_errors,
}
(results_root/'native_coverage_summary.json').write_text(json.dumps(summary, indent=2)+'\n')
(results_root/'native_coverage_records.json').write_text(json.dumps(records, indent=2)+'\n')
with (results_root/'native_coverage_table.txt').open('w') as f:
    f.write('recipe | result | loweredEffects | unsupportedEffects | sourceRenderMode\n')
    for record in records:
        f.write(f"{record['recipe']} | {record['result']} | {','.join(record.get('loweredEffects') or []) or '-'} | {','.join(record.get('unsupportedEffects') or []) or '-'} | {record.get('sourceRenderMode') or '-'}\n")
print(f"K2.22 native coverage audit PASS. Results: {results_root}")
print(f"recipes={len(records)} nativePasses={len(passes)} fallbacks={len(fallbacks)} hardErrors={len(hard_errors)}")
if unsupported:
    print('topUnsupported=' + ', '.join(f'{k}:{v}' for k,v in sorted(unsupported.items(), key=lambda kv:(-kv[1],kv[0]))[:8]))
PY

# <FILE>scripts/k222_native_coverage_audit.sh</FILE> - <DESC>Audit source-isolated native compositor coverage across v3.1 debug recipes</DESC>
# <VERS>END OF VERSION: 0.1.0</VERS>
