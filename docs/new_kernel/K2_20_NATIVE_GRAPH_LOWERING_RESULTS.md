<!-- <FILE>docs/new_kernel/K2_20_NATIVE_GRAPH_LOWERING_RESULTS.md</FILE> - <DESC>K2.20 graph-to-CompositionSpec lowering results</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Native compositor lowering: summarize request boundary and lowering registry behavior.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document native graph lowering request/output shape.</CLOG> -->

# K2.20 native graph lowering results

## Backend request boundary

The player-owned request boundary is now represented by:

- `PlayerRenderCompositionMode`
- `PlayerRenderBackendOptions`
- `PlayerRenderBackendRequest`

The UI and CLI pass recipe, descriptor catalog, sample request, IR, and backend options to backend adapters without constructing compositor DTOs.

## Native lowering behavior

The compositor backend supports explicit modes:

- `irResolved`: K2.19-compatible player IR path with `playerIrAlreadyResolved`.
- `native`: recipe graph/effect nodes lower into native `CompositionSpec` content, with no fallback.
- `auto`: native where supported and explicit IR fallback when unsupported.

Successful native diagnostics include `nativeCompositionSpecApplied`. Unsupported nodes report `unsupportedNativeEffect`; auto fallback reports `requiresIrFallback`.

## CompositionSpec summary fields

Every compositor backend JSON now includes top-level evidence and mirrored `backendMetadata` fields for:

- `compositionMode`
- `fallbackUsed`
- `nativeLoweringAttempted`
- `nativeLoweringSucceeded`
- `compositionSpecNonEmpty`
- `loweredNodeCount`
- `unloweredNodeCount`
- `loweredEffectIds`
- `unloweredEffectIds`
- `compositionSpecSummary`

<!-- <FILE>docs/new_kernel/K2_20_NATIVE_GRAPH_LOWERING_RESULTS.md</FILE> - <DESC>K2.20 graph-to-CompositionSpec lowering results</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
