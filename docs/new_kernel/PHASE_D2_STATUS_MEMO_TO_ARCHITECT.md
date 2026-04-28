<!-- <FILE>docs/new_kernel/PHASE_D2_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase D2 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>New kernel Phase D2 wrap: record final green verification and architect approval.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — record final green verification and architect approval.
0.1.0: INIT — add Phase D2 architect memo in the established status-memo style.</CLOG> -->

# Phase D2 Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-28
Repo: `/usr/projects/tui-vfx`
Phase: D2 — Template Composition Design

## Executive summary

Phase D2 has reached the design point recommended in `ARCH-RESP-TO-PHASE_D1.md`:

```text
How do templates, mixins/traits, presets, profiles, and recipes compose into
one strict canonical v3.1 recipe before validation, compilation, and runtime?
```

Current answer: **template composition belongs entirely above runtime, as deterministic compile-time expansion into a strict canonical v3.1 recipe.**

The new design document, `docs/v3.1-template-composition.md`, defines the vocabulary and rules needed before recipe/compiler implementation begins. It distinguishes templates, mixins/traits, presets, profiles, source recipes, canonical recipes, and expansion reports. It locks the runtime boundary: runtime consumes canonical recipes only and never sees template inheritance or composition metadata.

Phase D2 is intentionally docs-only. It adds no Rust contract types, no schema roots, no template expander, and no recipe compiler.

## Current design state

New document:

```text
docs/v3.1-template-composition.md
```

Updated supporting docs:

```text
docs/v3.1-architecture-overview.md
docs/v3.1-feature-contract-checklist.md
docs/new_kernel/AGENT_BRIEFING.md
docs/new_kernel/INDEX.md
docs/INDEX.md
```

New status artifacts:

```text
docs/new_kernel/PHASE_D2_STATUS.md
docs/new_kernel/PHASE_D2_STATUS_MEMO_TO_ARCHITECT.md
```

Captured architect response:

```text
docs/new_kernel/ARCH-RESP-TO-PHASE_D1.md
```

## Goal-by-goal status against the D2 recommendation

| D2 goal / constraint | Current status |
|---|---|
| Define template vs mixin/trait vs preset vs profile vs recipe | **Done.** The design gives each concept an explicit role. |
| Lock compile-time composition rather than runtime inheritance | **Done.** Runtime receives canonical recipes only. |
| Define expansion pipeline | **Done.** Source recipe + inputs → expand structure → apply values → validate composition → canonical recipe. |
| Define merge rules | **Done.** Maps merge by key; arrays require explicit operations; scalars need explicit replacement paths. |
| Define override rules | **Done.** Accidental collisions are errors; explicit override records are required. |
| Define sealed/final fields | **Done.** Sealed fields protect safety and semantic identity. |
| Define id namespacing | **Done.** Template-local ids are scoped to template instances; collisions after namespacing are errors. |
| Define slot filling | **Done.** Slot contracts need kind, cardinality, namespace, override policy, and diagnostics. |
| Define conflict diagnostics | **Done.** The design lists future structured diagnostic categories and required diagnostic fields. |
| Define provenance model | **Done.** Canonical recipe and expansion report are separate; diagnostics can report source and expanded paths. |
| Define canonical expanded recipe output | **Done.** The design gives an illustrative concrete shape and locks the no-template-refs rule. |
| Avoid implementation | **Respected.** No recipe compiler, template expander, descriptors, runtime, or Rust DTOs were added. |

## Key decisions

### Runtime does not see templates

The most important D2 boundary is:

```text
source authoring inputs
    -> deterministic expansion
    -> canonical v3.1 recipe
    -> validation/compiler/runtime
```

Runtime should never branch on template ancestry. If runtime behavior changes based on whether a scene came from a template, the design has failed.

### Presets and profiles are values-only

D2 separates values-only reuse from structure reuse:

```text
Preset: design flavor values.
Profile: environment/product-mode values.
Template: structure and slots.
Mixin/trait: additive reusable structure fragment.
```

A preset or profile that adds elements, nodes, scopes, phases, or slots is invalid.

### Conflicts fail unless explicit

D2 rejects silent inheritance semantics. Maps merge by key, but duplicate ownership requires an explicit override. Arrays do not append implicitly. Removal is deferred until a later phase designs an explicit operation.

### Provenance is tooling output, not runtime input

D2 separates:

```text
Canonical recipe: strict compiler/runtime input.
Expansion report: diagnostics/tooling provenance artifact.
```

This keeps the runtime clean while preserving enough source-path and expanded-path data for editor/compiler diagnostics.

## What deliberately was not added

Phase D2 does not add:

```text
template expansion implementation
recipe schema/compiler
runtime inheritance
effect descriptor expansion
studio manifest
runtime bindings
phase graph
trigger engine
legacy migration
real effect ports
public Rust contract types
schema roots
```

## Verification evidence

Final D2 wrap verification passed:

```text
git diff --check on D2 docs                 PASS
tracked Rust changes for D2                 none
forbidden dependency grep over tui-vfx-next PASS / no matches
cargo test -p tui-vfx-next                  PASS
cargo test --workspace                      PASS
architect verification                      APPROVED
docs-only deslop pass                       PASS
post-deslop regression verification         PASS
```

## Open questions for next assignment

1. Should D3 be the contract/engine boundary plus generalized `ScopeSpec`/write model phase recommended in `ARCH-RESP-TO-PHASE_D1.md`?
2. Should the next implementation-facing design start with source authoring schemas or canonical recipe schemas?
3. Which sealed fields should be default in the first implementation?
4. Should expansion reports be mandatory build artifacts or optional diagnostics?

## Bottom line

Phase D2 gives reusable recipe authoring a clean contract boundary: templates, mixins, presets, and profiles are source-authoring inputs; canonical v3.1 recipes are the compiler/runtime input. That lets later descriptor and recipe phases add reuse without reintroducing hidden runtime inheritance or legacy alias behavior.

Recommended next architect assignment: confirm **Phase D3 — Contract/engine boundary + generalized ScopeSpec/write model** or redirect to the first schema-focused recipe/template implementation slice.

<!-- <FILE>docs/new_kernel/PHASE_D2_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase D2 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
