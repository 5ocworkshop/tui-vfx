# Studio Control Derivation Report

This packet does not build the studio UI. It identifies the stable derivation inputs for a future control catalog.

Controls can be derived from:

- `graph.parameters` and `graph.signals`
- source descriptor inputs
- effect descriptor inputs
- `ValueSpec.kind`
- `ValueSpec.range`
- `allowedValues`
- `unit`
- `semantic`
- `runtimeMutability`
- `bindable`
- `optional`

Suggested control mapping:

| Input metadata | Control |
| --- | --- |
| `kind=number` or `integer` with range | slider/spinbox |
| `kind=enum` with `allowedValues` | select/radio |
| `kind=boolean` | toggle |
| `kind=color` | color picker or token selector |
| `kind=gradient` | gradient editor |
| `bindable=true` | binding/source picker affordance |
| `runtimeMutability=compileTime` | disabled at runtime, editable in authoring only |

Future command shape can be a JSON control-catalog report over descriptor packs and recipe graph declarations. Full dynamic UI should wait until graph/value/source controls are stable.
