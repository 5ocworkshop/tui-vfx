<!-- <FILE>docs/new_kernel/K2_12_DESCRIPTOR_EXPANSION_QUEUE.md</FILE> - <DESC>K2.12 descriptor expansion queue</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.12 schema lock: sort descriptor and field blockers into actionable expansion queues.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document descriptor expansion candidates, field decisions, and holdbacks.</CLOG> -->

# K2.12 Descriptor Expansion Queue

## Queue summary

K2.12 treats descriptor expansion as a burn-down queue, not as a schema-version bump. The schema remains pre-release v3.1; additive descriptor/report fields do not change `v3.1.player.schemaReadiness.1` during this phase.

| Queue | Records | Disposition | Notes |
|---|---:|---|---|
| Low-friction now | 13 | Candidate descriptor/adapter expansion | Safe only when existing player semantics already match or failure remains explicit. |
| Needs adapter decision | 30 | Defer | Requires player adapter behavior before fixture migration. |
| Needs field semantics | 51 | Defer | Authored fields need exact descriptor ownership and adapter consumption. |
| Needs descriptor decision | 32 | Defer | New descriptors require owner naming and contract semantics. |
| Hold back problematic | 25 | Hold | GUI, backend, oracle, or ambiguous-intent records. |

## Recommended low-friction candidates

These candidates are suitable for a follow-up descriptor-expansion tranche because they are named primitives rather than broad schema aliases:

- `shader.revealWipe`
- `shader.barberPole`
- `shader.pulseWave`
- `shader.orbit`
- `style.colorShift`

Style-effect candidates should only proceed after the style scope vocabulary decision, so they are not bundled into the K2.12 low-friction source fixture.

## Field-coverage closure

| Field blocker | Affected records | K2.12 classification | Decision |
|---|---:|---|---|
| `gradient` | 3 | Low-friction normalization candidate | Map to existing `startColor`, `endColor`, and `colorSpace` only if the legacy gradient object contains exactly that meaning. |
| `applyTo` | 1 direct shader blocker plus many composition examples | Adapter/descriptor decision | Candidate foreground/background/both target for `shader.linearGradient`; do not copy filter semantics blindly. |
| `position` | 1 | Hold behind value-source/binding semantics | Do not mark handled until position is either a descriptor field or a binding/value-source contract. |

## Guardrail

A descriptor queue item is not a license to add permissive fields. Each accepted field needs:

1. canonical v3.1 name,
2. value type and unit,
3. runtime mutability,
4. player adapter behavior or explicit unsupported-adapter diagnostics,
5. fixture or report evidence.

<!-- <FILE>docs/new_kernel/K2_12_DESCRIPTOR_EXPANSION_QUEUE.md</FILE> - <DESC>K2.12 descriptor expansion queue</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
