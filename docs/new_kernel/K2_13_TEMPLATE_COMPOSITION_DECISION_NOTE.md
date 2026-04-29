<!-- <FILE>docs/new_kernel/K2_13_TEMPLATE_COMPOSITION_DECISION_NOTE.md</FILE> - <DESC>K2.13 template composition decision note</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.13 schema decision burn-down: record compile-time template boundary.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — reaffirm template support as mandatory compiler-layer work outside runtime/player.</CLOG> -->

# K2.13 Template Composition Decision Note

## Decision

Template support is mandatory, but it remains a compile-time layer above runtime/player execution.

The model is:

```text
template + slots + overrides + mixins
  -> expanded canonical v3.1 recipe
  -> strict validation
  -> runtime/player evidence
```

Runtime/player must never see unresolved template inheritance.

Canonical v3.1 runtime recipes contain:

```text
no extends
no unresolved inherited fields
no template refs required for execution
```

## Disposition

Template support is not a schema-readiness blocker for the current debug-recipes declaration once the compiler-layer boundary is explicit. Future template work must compile into the accepted v3.1 recipe, source, graph, lifecycle, scene, value-source, and descriptor contracts.

<!-- <FILE>docs/new_kernel/K2_13_TEMPLATE_COMPOSITION_DECISION_NOTE.md</FILE> - <DESC>K2.13 template composition decision note</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
