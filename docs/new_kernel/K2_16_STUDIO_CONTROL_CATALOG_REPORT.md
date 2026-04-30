# K2.16 studio control catalog preflight report

## Status

K2.16 did not add a `control-catalog` CLI or final catalog spec. The render IR work creates a better downstream target for studio/backend work, and descriptor inputs already carry the core data a future catalog needs.

## Catalog source data

A future catalog can derive controls from:

- `graph.parameters`
- `graph.signals`
- source descriptor inputs
- effect descriptor inputs
- `ValueKind`
- `ValueSpec.range`
- `allowedValues`
- `unit`
- `semantic`
- `runtimeMutability`
- `bindable`
- optional/default fields

## Control mapping

```text
number/integer + range -> slider + numeric input
number/integer no range -> numeric input
boolean -> toggle
enum + allowedValues -> select
color -> color picker/token selector
gradient -> gradient editor placeholder
text/string -> text input
binding-capable input -> binding picker affordance
compile-time-only input -> disabled at runtime / authoring only
optional input -> enable/disable checkbox
sampledField ValueSource -> spatial-field picker placeholder
```

## Template boundary

Templates remain mandatory compile-time inputs. Runtime/player sees expanded canonical v3.1 recipes with no unresolved extends, mixins, or slots.
