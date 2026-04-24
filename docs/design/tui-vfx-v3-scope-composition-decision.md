<!-- <FILE>docs/design/tui-vfx-v3-scope-composition-decision.md</FILE> - <DESC>Accepted V3 decisions for scope inheritance modes and composition combine defaults.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Record the project-owner-approved resolution for scope inheritance and composition defaults: intersect by default, replace as explicit escape hatch, union deferred, and normalized IR makes combine semantics explicit.</WCTX> -->
<!-- <CLOG>0.1.0: initial accepted scope/composition decision with configurable scope inheritance, deferred union, and explicit normalized combine/merge semantics.</CLOG> -->

# V3 scope and composition decision

This document records accepted V3 decisions for scope inheritance and composition
combine defaults.

## 1. Scope inheritance

### Decision

Scope inheritance defaults to narrowing by intersection.

Containers may set:

```json
"scope_mode": "intersect"
```

or:

```json
"scope_mode": "replace"
```

`intersect` is the default. `replace` is the explicit escape hatch. `union` is
deferred until real recipes prove it is needed.

Accepted wording:

> Scope inheritance defaults to intersection. Containers may set `scope_mode:
> "intersect"` or `scope_mode: "replace"`. `replace` allows a child scope to
> override inherited scope explicitly. `union` is deferred until real recipes
> prove it is needed. Normalized IR must show both authored scope and effective
> scope.

### Semantics

At the root:

```text
effective_scope = all
```

For a normal inherited child under `intersect`:

```text
effective_scope = inherited_scope ∩ child_scope
```

If the child omits `scope`, it inherits the parent effective scope.

For `replace`:

```text
effective_scope = child_scope
```

If a `replace` child omits `scope`, validation should reject it because the
replacement has no explicit target.

If an `intersect` result is statically empty, validation should reject it as an
unreachable step. Runtime-bound scopes may be reported as review-needed when
emptiness cannot be known statically.

### Example: intersection

```json
{
  "kind": "sequence",
  "scope": { "kind": "role", "value": "text" },
  "children": [
    {
      "kind": "shader",
      "scope": { "kind": "rect", "rect": [0, 0, 20, 4] },
      "payload": { "type": "diffusion" }
    }
  ]
}
```

The shader applies to text-role cells inside `rect [0, 0, 20, 4]`.

### Example: replace

```json
{
  "kind": "sequence",
  "scope": { "kind": "role", "value": "text" },
  "children": [
    {
      "kind": "shader",
      "scope_mode": "replace",
      "scope": { "kind": "role", "value": "border" },
      "payload": { "type": "border_sweep" }
    }
  ]
}
```

The shader applies to border-role cells, not text-role cells.

## 2. Composition combine defaults

### Decision

Authoring uses per-kind defaults. Normalized IR makes effective combine and merge
semantics explicit.

Accepted wording:

> Authoring uses per-kind defaults. `sequence` is ordered feed-forward.
> `parallel` is snapshot-isolated with post-join merge. Normalized IR must make
> effective combine/merge semantics explicit. Overlapping parallel outputs keep
> authored-order conflict resolution unless a fixture proves a specific
> optimization-safe class.

### Defaults

| Container / family | Default |
|---|---|
| `sequence` | Run children in authored order; later steps see earlier outputs. |
| `parallel` | All children read the same pre-parallel snapshot; outputs join after the block. |
| filters in sequence | Chain in order. |
| masks in sequence | Intersect / narrow visibility by default. |
| shaders in sequence | Apply in order, channel-aware. |
| samplers in sequence | Compose coordinate transforms in order. |
| parallel non-overlap | Merge safely. |
| parallel overlap | Authored order wins at join; serial-required unless proven safe. |

### Tooling requirements

Validator and canonicalization tooling should:

- apply default combine/merge rules during normalization
- emit explicit effective semantics in normalized IR
- classify unsafe parallel overlaps as serial-required
- preserve authored-order conflict resolution for overlapping parallel outputs
- require fixture-backed proof before any optimization changes observable merge
  behavior

## Plan impact

This resolves the active decision shape for:

- Open Q #4 — composition combine semantics
- Chapter 90 scope composition precedence
- scheduler/batching optimization preconditions
- normalized IR canonicalization requirements

Implementation remains in validator/canonicalization, scheduler classification,
and docs/schema-generation lanes.

<!-- <FILE>docs/design/tui-vfx-v3-scope-composition-decision.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
