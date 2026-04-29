<!-- <FILE>docs/new_kernel/K2_12_SOURCE_CONTENT_DECISION_TABLE.md</FILE> - <DESC>K2.12 source and content decision table</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.12 schema lock: classify source/content blockers and retire the low-friction source.text fixture gap.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document source/content dispositions for the v3.1 schema-lock burn-down.</CLOG> -->

# K2.12 Source and Content Decision Table

## Decision summary

K2.12 keeps the v3.1 pathway focused on contract-native source descriptors. It adds one low-risk fixture-backed path and explicitly holds back the remaining source/content families instead of overloading `source.card` or pretending legacy content transforms are plain text.

| Source/content lane | Records | K2.12 disposition | Schema-readiness effect | Next action |
|---|---:|---|---|---|
| `source.text` plain text | Pilot fixture added | Accepted as low-friction canonical fixture | No longer a fixture gap for plain text source smoke coverage | Keep descriptor fields `text`, `width`, and `height`; do not expand into richer content transforms. |
| Typewriter content | 18 | Hold back as content descriptor work | Blocks until content transform descriptor policy is accepted or explicitly deferred | Design `content.typewriter`/typed content descriptors; do not map to `source.text` directly. |
| Split-flap content | 20 | Hold back as content descriptor work | Blocks until descriptor and adapter semantics exist | Decide whether split-flap is a source descriptor, effect descriptor, or content pipeline primitive. |
| Odometer content | 10 | Hold back as content descriptor work | Blocks until descriptor and adapter semantics exist | Decide numeric/text source ownership and formatting lifecycle. |
| Cell-motion content | 3 | Hold back behind source/local pipeline and motion semantics | Blocks until source-local pipeline and motion value semantics exist | Defer until runtime dynamism and scene-local pipeline decisions are settled. |
| Single-purpose text transforms | 12 | Descriptor expansion candidate | Blocks visible migration, not a core schema rename | Queue as content descriptor expansion once source identity policy is stable. |
| Marquee text | 1 | Source/content descriptor decision | Blocks until owner chooses source descriptor versus content effect | Decide `source.marqueeText` versus content transform descriptor. |
| Glyph/procedural text | 2 | Hold back behind procedural source policy | Blocks until procedural/source generation semantics exist | Defer to procedural source packet; do not add command or filesystem execution to player. |
| Command-capture source | 1 | Oracle-only/offline artifact | Does not block schema once explicitly signed off | Keep runtime playback command-free and filesystem-free. |

## Implemented low-friction tranche

Added canonical external-recipe fixture:

```text
../tui-vfx-recipes/recipes/v3.1/debug_recipes/sources/source_text_basic.json
```

The fixture embeds the `source.text` descriptor and uses literal `text`, `width`, and `height` inputs. The player already renders `source.text`, so the only code change needed in this repo was marking those three inputs as handled for primitive field coverage.

Verification evidence:

```text
validate-recipe source_text_basic.json: valid=1 invalid=0
render-recipe source_text_basic.json: rows=["SOURCE.TEXT         ", "Plain text fixture  "] errors=[] warnings=[]
fixture-qc source_text_basic.json: overallStatus=pass, errors=0, gaps=0
```

## Guardrails

- `source.card` remains card-shaped; it is not a compatibility bucket for legacy content transforms.
- `source.text` remains plain text; typewriter, split-flap, odometer, marquee, and procedural text need their own descriptor decisions.
- Command-capture records are oracle evidence only. The v3.1 player must not spawn commands or read arbitrary files during recipe playback.

<!-- <FILE>docs/new_kernel/K2_12_SOURCE_CONTENT_DECISION_TABLE.md</FILE> - <DESC>K2.12 source and content decision table</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
