# Scene and Source Evidence Report

## Added source evidence

- `source.ansi` renders a bounded ANSI/SGR-stripped text fallback without panics.
- `source.image` renders deterministic missing-asset fallback text.
- `source.procedural` renders a deterministic `dots_spinner` fallback.

## Scene evidence

The added source fixtures exercise scene placement through source-backed elements. Player rendering still preserves element identity in recipe data and z-order sorting in render traversal, but richer layer-local pipelines and visibility predicates remain future player work.

## Boundary

Element/layer identity must remain separate from roles. No new fixture overloads role tags as element identifiers.
