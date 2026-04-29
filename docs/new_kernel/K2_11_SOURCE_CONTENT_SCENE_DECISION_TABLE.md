<!-- <FILE>docs/new_kernel/K2_11_SOURCE_CONTENT_SCENE_DECISION_TABLE.md</FILE> - <DESC>K2.11 source content scene decision table</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.11 v3.1 source/content/scene blocker triage.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — classify source, content, scene, asset, and command-capture candidates.</CLOG> -->

# K2.11 Source, Content, and Scene Decision Table

## Scope

This table is restricted to the v3.1 pathway. It does not add runtime command execution and does not treat the legacy recipe runtime as a dependency.

## Source descriptor pilot result

`source.text` is now present in `descriptors/v3.1/packs/primitive.json` with `text`, `width`, and `height` inputs. The contract-native player already has a `source.text` rendering path, and inventory now reports `source.text` as descriptor-covered and `visible` adapter-backed. No v3.1 fixture was added in this repo packet; the canonical recipe repo remains a separate mutation surface.

Inventory evidence:

```text
sourceIds: 2
source.card: descriptorCovered=true, representedByRecipes=true, adapterStatus=visible
source.text: descriptorCovered=true, representedByRecipes=false, adapterStatus=visible
```

## Candidate table

| Candidate | Legacy evidence | Contract support exists? | Descriptor shape clear? | Player adapter exists? | Safe in K2.11? | Remaining blocker / next action |
|---|---|---|---|---|---|---|
| `source.text` | content records that need plain text grids; existing player source rendering path | Yes | Yes: `text`, optional `width`, optional `height` | Yes | Yes, descriptor pilot added | Add canonical v3.1 fixture in the recipe repo and use it to retire plain-text content blockers. |
| `source.ansi` | `scene/ansi_source_chain.json`; terminal content chain examples | Partial source model only | Not yet: ANSI escape handling and style preservation need policy | No explicit adapter evidence | No | Decide descriptor-only evidence versus adapter-backed ANSI parsing. |
| `source.image` | content/image-like and asset-backed examples | Asset references exist, but source semantics are not settled | Not yet: raster loading/resolution is outside current player | No | No | Needs asset resolver seam and non-terminal image policy. |
| `source.procedural.*` | braille flag, spinner, generated content examples | Not as concrete source descriptors | No | No | No | Decide stable procedural identity, seed/determinism, and source-local parameter contract. |
| `source.commandCaptureArtifact` | `fixtures/command_capture_chain.capture.json`; complex command-capture examples | Offline artifact policy only | Candidate shape possible | No runtime adapter by design | No runtime execution | Keep as oracle/offline artifact; never execute commands in the player. |
| `source.card` expansion | existing canonical fixtures use `source.card` | Yes | Existing card shape is clear | Yes | No broad expansion | Avoid overloading debug-card semantics to cover text/ANSI/image/procedural content. |
| `content.typewriter` / `content.marquee` | content source-decision records | Effect-like content transform, not plain source | Not yet | No | No | Needs lifecycle/schedule semantics before descriptor expansion. |
| `content.cellMotion` | content cell-motion records and complex pipeline records | Not settled | Not yet | No | No | Classify as content transform or scene/source-local pipeline before implementing. |
| scene source-local pipeline | scene records and complex chain records | Scene/element model exists | Pipeline semantics not settled | Partial render traversal only | No | K2.12 scene/source-local pipeline packet. |
| scene layer visibility/overflow | scene records | Basic placement exists | Layer semantics not settled | Partial | No | Decide layer visibility, clipping, overflow, and ordering semantics. |

## Family audit

| Family | Record count | Subclass | Representative paths | Required source descriptor | Required content descriptor | Required scene decision | Recommended next packet | Confidence |
|---|---:|---|---|---|---|---|---|---|
| content | 66 | Source/content descriptor blockers | `content/content_marquee.json`; `content/content_cell_motion_slice.json` | `source.text` plus future `source.ansi`/procedural/image decisions | likely typewriter/marquee/cell-motion descriptors | No for plain text; maybe yes for cell-motion pipelines | K2.12 source/content descriptor expansion | Medium |
| content | 45 | Oracle-only deprecated content | `content/_DEPRECATED_content_dissolve.json`; `content/_DEPRECATED_content_glitch_shift.json` | No | No | No | Owner signoff as oracle-only | High |
| fixtures | 1 | Offline command-capture artifact | `fixtures/command_capture_chain.capture.json` | Candidate `source.commandCaptureArtifact` | No | No | Offline artifact policy, no runtime execution | High |
| scene | 12 | Scene/source-local semantics | `scene/ansi_source_chain.json`; `scene/scene_authoring_ladder_toast_basic.json` | `source.ansi`/asset/procedural candidates | Maybe | Yes | K2.12 scene/source-local pipeline packet | Medium |
| scene | 6 | Binding semantics in scene examples | `scene/scene_authoring_ladder_flag_asset_binding.json`; `scene/scene_braille_flag_runtime_wave.json` | Possibly asset/procedural | No | Yes | K2.12 lifecycle/signal/binding/value-source packet | Medium |
| scene | 1 | Lifecycle semantics | `scene/scene_layer_io_filter_shader.json` | No | No | Yes | K2.12 lifecycle/signal/binding/value-source packet | Medium |
| complex | 73 | Mixed source/content/scene pipelines | `complex/command_capture_chain.json`; `complex/complex_cell_motion_shader_pipeline.json` | Maybe | Maybe | Maybe | K2.12 owner-audit normalization before implementation | Low |

## Stop conditions preserved

- Runtime command capture remains prohibited.
- Image/procedural sources remain blocked until asset/runtime policy and deterministic identity are explicit.
- Content transforms that require schedule/lifecycle semantics are not forced into source descriptors.
- Scene-local pipelines remain schema blockers until placement/layer/source-local semantics are approved.

<!-- <FILE>docs/new_kernel/K2_11_SOURCE_CONTENT_SCENE_DECISION_TABLE.md</FILE> - <DESC>K2.11 source content scene decision table</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
