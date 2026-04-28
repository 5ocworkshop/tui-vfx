<!-- <FILE>docs/new_kernel/PHASE_D2_STATUS.md</FILE> - <DESC>Concise Phase D2 template composition design status</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>New kernel Phase D2 wrap: record final verification and architect approval.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — record final green verification, architect approval, and docs-only deslop pass.
0.1.0: INIT — record Phase D2 design decisions, non-implementation scope, and verification plan.</CLOG> -->

# Phase D2 Status — Template Composition Design

## Status

Phase D2 design is complete, architect-approved, deslop-reviewed, and verified green.

## What changed

Phase D2 adds the design document:

```text
docs/v3.1-template-composition.md
```

The design locks how source authoring documents should compose into one canonical v3.1 recipe before validation, compilation, and runtime.

## Design decisions

Phase D2 locks:

- Template composition is compile-time authoring behavior, not runtime inheritance.
- Runtime receives only canonical expanded strict v3.1 recipes.
- Templates define reusable structure and slots.
- Mixins/traits are additive reusable fragments with deterministic order.
- Presets and profiles are values-only.
- Expansion namespaces ids deterministically.
- Slot contracts define accepted kind, cardinality, namespace, override policy, and diagnostics.
- Conflicts require explicit override syntax or fail.
- Sealed/final fields protect safety and semantic identity.
- Expansion diagnostics report both source-template path and expanded canonical path.
- Canonical v3.1 recipes contain no template refs, inheritance pointers, presets, profiles, mixin references, or legacy aliases.

## What deliberately did not change

Phase D2 does not implement:

- template expansion code;
- recipe schema/compiler;
- runtime inheritance;
- effect descriptor expansion;
- studio manifest;
- runtime bindings;
- phase graph or trigger engine;
- legacy migration;
- real effect ports;
- public Rust contract types.

Because D2 adds no Rust contract types, it does not add schema roots.

## Docs updated

```text
docs/v3.1-template-composition.md
docs/v3.1-architecture-overview.md
docs/v3.1-feature-contract-checklist.md
docs/new_kernel/AGENT_BRIEFING.md
docs/new_kernel/INDEX.md
docs/INDEX.md
```

The architect response is also captured for phase history:

```text
docs/new_kernel/ARCH-RESP-TO-PHASE_D1.md
```

## Verification evidence

Final phase verification passed:

- `git diff --check` on D2 docs — PASS
- no tracked Rust files changed for D2 — PASS
- forbidden dependency grep over `crates/tui-vfx-next` — PASS / no matches
- `cargo test -p tui-vfx-next` — PASS
- `cargo test --workspace` — PASS
- architect verification sidecar — APPROVED
- D2 docs-only deslop pass — PASS; stale pending-verification wording removed

## Open questions for architect follow-up

- Should D3 be contract/engine boundary plus generalized `ScopeSpec`/write model, as recommended in `ARCH-RESP-TO-PHASE_D1.md`?
- Which source authoring schema should be designed first when implementation begins: template, source recipe, or canonical recipe?
- Should provenance reports be mandatory build artifacts or opt-in diagnostics?

<!-- <FILE>docs/new_kernel/PHASE_D2_STATUS.md</FILE> - <DESC>Concise Phase D2 template composition design status</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
