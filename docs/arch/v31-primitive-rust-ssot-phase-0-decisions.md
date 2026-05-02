<!-- <FILE>docs/arch/v31-primitive-rust-ssot-phase-0-decisions.md</FILE> - <DESC>Phase 0 substrate decision record for Rust-owned v3.1 primitive catalog execution</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Close the hard Phase 0 gates before any primitive runtime ports begin: cell access enforcement, binding scheduling, scalar/wrapper homes, runtime trait split, and legacy dependency policy.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record source-verified Phase 0 decisions and the exact substrate slice implemented in tui-vfx-compost.</CLOG> -->

# Phase 0 Decision Record: V3.1 Primitive Rust SSOT

## Status

Accepted for the first execution slice of `docs/arch/v31-primitive-rust-ssot-implementation-plan.md`.

This record closes the five Phase 0 gates needed before primitive runtime ports start. It is intentionally scoped to substrate shape and dependency policy; it does **not** declare any primitive runtime semantics complete.

## Source evidence read before decision

- `crates/tui-vfx-contract/src/cls_effect_domain.rs` defines the current effect domains: `contentGenerator`, `contentTransform`, `cellShader`, `frameFilter`, `coordinateSampler`, `mask`, `shadow`, `postProcess`, `diagnosticTooling`.
- `crates/tui-vfx-contract/src/cls_effect_descriptor.rs` owns descriptor DTO fields and validation for `cellAccess`, `scopeSupport`, `writeSupport`, `inputs`, and `outputs`.
- `crates/tui-vfx-contract/src/cls_value_source.rs` owns the `ValueSource` vocabulary used by runtime scheduling decisions.
- `crates/tui-vfx-contract/src/cls_runtime_mutability.rs` owns the only accepted mutability vocabulary: `compileTime`, `phaseStart`, `resetOnly`, `runtime`.
- `crates/tui-vfx-compost/src/render/cls_sample_context.rs` and `src/runtime/cls_runtime_context.rs` already separate sampled time/progress from resolved runtime values.
- Current legacy-adjacent compost imports remain in existing style/shadow seams (`tui-vfx-style`, `tui-vfx-shadow`); no new primitive substrate imports them.

## Decisions

### 1. `CellView<'_, P>` enforcement strategy

**Decision:** start with runtime debug assertions in `CellView<'_, P>`.

`CellView` now wraps a `tui_vfx_types::Cell` and reads `P::descriptor().cell_access` from the Rust-owned primitive descriptor. Read/write methods call `debug_assert!` when a primitive touches a channel not declared in the descriptor.

**Why not type-state now?** Compile-time channel enforcement is still desirable, but it would force heavier type plumbing before the first three domain runtimes prove what they need. Debug assertions keep the trait signatures readable and testable while preserving a migration path to type-state later.

**Guardrail:** targeted tests must cover declared writes and undeclared writes. The Phase 0 substrate test currently proves an undeclared foreground write panics in debug builds.

### 2. Bindable resolution and scheduling policy

**Decision:** `Literal<T>` vs `Bindable<T>` is descriptor metadata only.

- `Literal<T>` maps to descriptor `bindable: false`.
- `Bindable<T>` maps to descriptor `bindable: true`.
- Neither wrapper decides when a runtime samples or caches the value.

Scheduling is driven by `ValueSource` kind plus `RuntimeMutability`:

| Input source/mutability evidence | Runtime cadence rule |
| --- | --- |
| literal/default value with `compileTime` | Resolve at descriptor/instance construction; cache for the primitive instance. |
| parameter/default value with `phaseStart` | Resolve once at phase entry; cache until phase boundary. |
| `resetOnly` | Resolve when the primitive instance resets. |
| signal / graph value / signal expression with `runtime` | Resolve at the runtime point required by that source; do not infer cadence from `Bindable<T>`. |
| sampled-field value source | Resolve per cell because the source depends on spatial coordinates. |
| phase progress / clock value source | Resolve from the current `SampleContext`; cadence follows render sampling, not descriptor bindability. |

### 3. V3.1 scalar and wrapper type home

**Decision:** keep the type split aligned with existing crate responsibilities.

| Concern | Home |
| --- | --- |
| Stable v3.1 DTOs, descriptor schemas, `Value`, `ValueSpec`, `ValueSource`, `RuntimeMutability` | `tui-vfx-contract` |
| Low-level terminal/runtime foundation types such as `Cell`, `Color`, `Modifiers`, `Style`, `SemanticScene`, roles, and geometry | `tui-vfx-types` |
| Runtime-only primitive declaration wrappers, `CellView`, domain runtime traits, registry construction, and codegen registry assembly | `tui-vfx-compost` |

This avoids coupling `tui-vfx-contract` to compost runtime traits while preserving contract DTOs as the contract-schema SSOT.

### 4. Domain-specific runtime split

**Decision:** adopt descriptor trait plus domain runtime traits, not a universal `EffectRuntime`.

| Contract domain | Phase 0 runtime surface |
| --- | --- |
| `cellShader` | `CellShaderRuntime` |
| `frameFilter` | `FrameFilterRuntime` |
| `coordinateSampler` | `CoordinateSamplerRuntime` |
| `mask` | `MaskRuntime` |
| `contentTransform` | `ContentTransformRuntime` |
| source descriptors | `SourcePrimitive` + `SourceRuntime` |
| `contentGenerator`, `shadow`, `postProcess`, `diagnosticTooling` | descriptor-only until their runtime substrates are explicitly designed |

The registry now stores one descriptor view plus separate runtime-id tables for each implemented runtime domain. This gives codegen a complete descriptor view and gives runtime dispatch a clear domain map without pretending all primitives are filter-shaped.

### 5. Legacy dependency policy

**Decision:** enforce the three-tier policy from the accepted plan.

| Dependency | Policy for new primitive/source ports | Notes |
| --- | --- | --- |
| `tui-vfx-compositor` | Permanently forbidden | Legacy reference only; semantic evidence must be copied through clean v3.1 code, not imports. |
| `tui-vfx-style` | Forbidden for new primitive/source ports; grandfathered for existing compost seam | Existing `shader.linearGradient` support still imports style code and must be retired deliberately. |
| `tui-vfx-content` | Forbidden for new primitive/source ports | No new compost primitive substrate may import it. |
| `tui-vfx-shadow` | Forbidden for new primitive/source ports; grandfathered for existing compost seam | Existing shadow render helpers still import shadow code and must be retired deliberately. |
| `tui-vfx-contract` | Allowed | Owns v3.1 descriptor/value DTOs. |
| `tui-vfx-types` | Allowed | Owns low-level terminal and semantic-surface types. |

**Cleanup ticket:** `V31-PRIM-CLEANUP-001` — retire current grandfathered `tui-vfx-style` and `tui-vfx-shadow` compost imports by moving needed clean math/render helpers into v3.1-owned modules or shared non-legacy foundation crates.

## Implemented substrate slice

The first execution slice adds `crates/tui-vfx-compost/src/primitive/` with:

- `EffectPrimitive`, `PrimitiveInputs`, `PrimitiveOutputs`, `NoInputs`, and `NoOutputs` descriptor traits/markers;
- `SourcePrimitive` and `SourceRuntime` for source descriptors;
- `CellShaderRuntime`, `FrameFilterRuntime`, `MaskRuntime`, `CoordinateSamplerRuntime`, and `ContentTransformRuntime`;
- `CellView<'_, P>` with debug assertion channel enforcement;
- `EffectRuntimeContext`, `EffectRuntimeError`, `MaskVisibility`, `CoordinateSample`, and `SourceSurface`;
- `EffectRegistry` with descriptor maps, domain runtime-id tables, domain mismatch checks, duplicate-id checks, and `DescriptorPack` export;
- a scoped no-legacy-import test for the new primitive substrate directory.

## Phase 0 exit criteria

- The five hard gates have explicit decisions: **complete**.
- Trait names map unambiguously to `EffectDomain` and `SourceDescriptor`: **complete**.
- A primitive author can tell which trait to implement for any current descriptor id: **complete for implemented domains; descriptor-only for deferred domains is explicit**.
- Primitive runtime ports may now start, but the first three ports must still prove the runtime semantics and may require additive trait refinements before Phase 2 derive work.

<!-- <FILE>docs/arch/v31-primitive-rust-ssot-phase-0-decisions.md</FILE> - <DESC>Phase 0 substrate decision record for Rust-owned v3.1 primitive catalog execution</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
