<!-- <FILE>docs/new_kernel/K2_13_SCHEMA_API_DOC_INFRA_REPORT.md</FILE> - <DESC>K2.13 schema, API, and documentation infrastructure report</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.13 schema decision burn-down: record docs/schema/API synchronization for impacted surfaces.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document schema roots, rustdocs, vocabulary, and readiness report updates.</CLOG> -->

# K2.13 Schema, API, and Documentation Infrastructure Report

## Impacted surfaces

Rust contract/API surfaces impacted by K2.13:

- `GradientSpec` / `GradientStop`
- `ValueKind::Gradient`
- `Value::Gradient`
- `ValueSource::SampledField`
- `ScopeSpec` built-ins: `moduloRows`, `moduloColumns`, `nonEmpty`, `outerBand`, `inner`
- `ScopeKind` matching built-ins
- `EffectInputSpec.optional`
- `SourceInputSpec.optional`

Player/report surfaces impacted by K2.13:

- schema-readiness disposition fields
- offender schema-blocking fields
- field-coverage closure for gradient/applyTo/position
- styled-grid scope evaluation

## Schema fixtures

Checked-in schemas under `schemas/v3.1/contract/` were regenerated because the contract DTOs changed. The schema report version was not bumped because v3.1 is pre-release and owner direction explicitly forbids a version bump for additive fields in this lane.

## Documentation updates

This packet updates:

- K2.13 decision reports in `docs/new_kernel/`
- `docs/new_kernel/INDEX.md`
- `docs/VOCABULARY.md`
- `docs/v3.1-feature-contract-checklist.md`

<!-- <FILE>docs/new_kernel/K2_13_SCHEMA_API_DOC_INFRA_REPORT.md</FILE> - <DESC>K2.13 schema, API, and documentation infrastructure report</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
