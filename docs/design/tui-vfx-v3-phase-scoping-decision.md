<!-- <FILE>docs/design/tui-vfx-v3-phase-scoping-decision.md</FILE> - <DESC>Accepted V3 phase-scoping decision for step-level phase, container propagation, and normalized PhaseSet behavior.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Record the project-owner-approved resolution for Chapter 80 phase scoping: steps and containers can both declare phase membership, effective phase is inherited by intersection, and normalized IR makes it explicit.</WCTX> -->
<!-- <CLOG>0.1.0: initial accepted phase-scoping decision with examples and implementation rules.</CLOG> -->

# V3 phase-scoping decision

This document records the accepted V3 phase-scoping rule.

## Decision

V3 supports phase membership at both levels:

1. every step may carry `phase`
2. containers may carry `phase`
3. child step phase intersects with inherited container phase
4. if no phase is specified, default to `all`
5. canonical normalized IR makes the effective `PhaseSet` explicit

Accepted wording:

> V3 supports step-level `phase` plus container phase propagation. Effective
> phase is inherited by intersection and normalized to explicit `PhaseSet` in
> canonical IR. Default phase is `all`.

## Why this shape

This keeps simple recipes compact while preserving readable grouping for larger
recipes.

A single step can declare its own phase directly:

```json
{
  "kind": "filter",
  "phase": "enter",
  "payload": { "type": "fade" }
}
```

A container can group multiple steps under shared phase membership:

```json
{
  "kind": "sequence",
  "phase": ["enter", "dwell"],
  "children": [
    { "kind": "shader", "payload": { "type": "glow" } },
    {
      "kind": "style_effect",
      "phase": "dwell",
      "payload": { "type": "pulse" }
    }
  ]
}
```

Effective phase behavior in that example:

- `glow` applies during `enter + dwell` because it inherits the container phase
- `pulse` applies only during `dwell` because its child phase intersects the
  inherited `enter + dwell` container phase

## PhaseSet shape

Authoring may use either a single phase string or an array of phase strings:

```json
"phase": "dwell"
```

```json
"phase": ["enter", "dwell"]
```

Normalized IR should canonicalize both forms into an explicit `PhaseSet`.

Recommended canonical phase vocabulary:

- `enter`
- `dwell`
- `exit`
- `all` as authoring shorthand for `{ enter, dwell, exit }`

## Inheritance rule

Let:

- `P_parent` be the effective phase inherited from the nearest ancestor
- `P_child` be the phase declared on the current node, or `all` if omitted

Then:

```text
P_effective = P_parent ∩ P_child
```

At the root, `P_parent = all`.

If the intersection is empty, validation should reject the step or container as
unreachable rather than silently dropping it.

## Tooling requirements

Validator and canonicalization tooling should:

- accept single-string and array authoring forms
- reject unknown phase names
- reject empty phase arrays
- reject empty effective phase intersections
- emit normalized IR with explicit effective `PhaseSet`
- make inherited-vs-authored phase visible in debug/canonical dumps where useful

## Plan impact

This resolves the active shape decision for:

- Open Q #3 — phase scoping as per-step field vs container
- Open Q #13 — partial phase spans / `PhaseSet` granularity
- Chapter 90 scope/phase grouping concerns where phase propagation affects tree
  readability

Implementation remains in the validator/canonicalization and schema-doc lanes.

<!-- <FILE>docs/design/tui-vfx-v3-phase-scoping-decision.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
