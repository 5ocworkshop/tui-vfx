<!-- <FILE>docs/new_kernel/K2_13_STUDIO_CONTROL_SURFACE_PREFLIGHT.md</FILE> - <DESC>K2.13 studio control surface preflight</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.13 schema decision burn-down: record studio control derivation preflight.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document studio control derivation from descriptors and typed specs.</CLOG> -->

# K2.13 Studio Control Surface Preflight

## Decision

The studio UI should derive controls from schema/descriptor data rather than inventing a separate manifest.

Primary derivation inputs:

- `graph.parameters`
- `graph.signals`
- source descriptors
- effect descriptors
- `ValueSpec`
- `range`
- `allowedValues`
- `unit`
- `semantic`
- `runtimeMutability`
- `bindable`
- `optional`

## K2.13 additions relevant to studio

- `ValueKind::Gradient` and `GradientSpec` imply gradient-stop editor controls.
- `ValueSource::SampledField` implies spatial-field pickers plus numeric mapping controls.
- `EffectInputSpec.optional` / `SourceInputSpec.optional` distinguish omitted canonical fields from missing required authoring fields.
- `dispositionCounts` and `remainingOwnerDecisionCount` allow a schema-readiness control panel to show declaration status without parsing prose.

## Holdback

A complete studio control generator is not implemented in this packet. The accepted boundary is a future docs/tooling lane that consumes descriptor/schema metadata after canonical recipe compilation.

<!-- <FILE>docs/new_kernel/K2_13_STUDIO_CONTROL_SURFACE_PREFLIGHT.md</FILE> - <DESC>K2.13 studio control surface preflight</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
