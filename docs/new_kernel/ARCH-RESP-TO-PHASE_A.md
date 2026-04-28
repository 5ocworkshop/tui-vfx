<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_A.md</FILE> - <DESC>Architect response to Phase A status memo</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Captured clean-room kernel Phase A architect approval and Phase B recommendation.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add OFPF metadata around captured clean-room kernel planning/status content.</CLOG> -->

I would **accept Phase A** as complete based on the memo and the surface-contract doc.

The important thing is that Phase A proved the right thing: not “we can render cool effects,” but “we can define surface semantics independently of the legacy compositor.” That is the correct foundation.

I would sign it off with a few small doc corrections and one clear recommendation for Phase B.

---

# Phase A architecture sign-off

Status: **approved as a bounded contract spike.**

The clean-room crate did what it was supposed to do:

```text
tui-vfx-types
    ↓
tui-vfx-next
```

It avoided the legacy implementation crates:

```text
no tui-vfx-compositor
no tui-vfx-style
no tui-vfx-content
no tui-vfx-shadow
```

It proved the key semantic rules:

```text
Surface = cells + roles + metadata
copy preserves sampled-source role
visual effects preserve roles
role scope uses sampled-source role space by default
skipped cells preserve destination state
zero-cell scope emits diagnostics
explicit procedural/shadow role write is possible
empty transparent write is distinct from skip
```

That is a successful Phase A.

---

# Small corrections I would make before moving on

## 1. Do not call `Custom(name)` a built-in role

In `v3.1-surface-contract.md`, this section says:

```text
Built-in roles are the roles defined by tui-vfx-types:

- Background
...
- Procedural
- Custom(name)
```

I would revise that to:

```text
First-class built-in roles:

- Background
- Text
- Title
- Caption
- Border
- Image
- Icon
- Indicator
- Highlight
- Shadow
- Decoration
- Procedural

Custom roles:

- Custom(name)
```

`Custom(name)` is part of the role model, but it is not a built-in role. That distinction matters once strict role declaration/validation arrives.

## 2. Clarify that `role` is a surface channel, not a `Cell` field

The doc currently says:

```text
A cell has these channels:
- glyph
- foreground color
- background color
- modifiers
- modifier alpha
- role
```

Given the implementation stores roles separately from `Cell`, I would phrase it as:

```text
A surface cell position has visual cell channels and one semantic role channel.

Visual cell channels:
- glyph
- foreground color
- background color
- modifiers
- modifier alpha

Semantic channel:
- role
```

That avoids future confusion about whether `tui_vfx_types::Cell` itself should grow a role field.

## 3. Clarify visual-only role preservation by operation shape

The doc says:

```text
A visual-only effect uses PreserveDestination.
A copy operation uses CopySampledSource.
```

That is correct, but the nuance should be explicit:

```text
A visual-only effect preserves the role already present on the destination surface.

A source-to-destination copy or transform writes the sampled source role first, then visual-only effects preserve that role.
```

Otherwise someone may interpret “visual-only effects preserve roles” as “they should always copy sampled-source roles,” which is not always true. It depends whether the effect is running in-place over an already materialized destination, or performing source-to-destination transfer.

---

# Recommended Phase B

I would not start recipes, descriptors, studio, or effect porting yet.

The next highest-risk thing is **non-identity sampling**.

Phase A only proved sampled-source role behavior with identity sampling. But the existing legacy compositor has samplers, and sampler + role scope was one of the most important semantic traps.

So I would make Phase B:

```text
Phase B: sampled-source semantics and scoped pipeline proof
```

The goal is to prove that the surface contract still holds when:

```text
destination coordinate != sampled source coordinate
```

This is the next thing to lock before effect descriptors or recipes.

---

# Phase B objective

Phase B should answer:

```text
Can the clean-room kernel preserve correct cell/role/scope/write semantics when source sampling is non-identity?
```

In other words:

```text
local destination coordinate
    → sampler / coordinate mapper
    → sampled source coordinate
    → sampled source cell
    → sampled source role
    → scope evaluation
    → write policy
    → destination cell + destination role
```

The main contract to prove:

```text
Geometry scopes default to destination-local coordinates.
Role scopes default to sampled-source roles.
Copied/transformed cells copy sampled-source roles.
Skipped sampled cells preserve destination state.
Diagnostics reflect actual sampled-source semantics.
```

---

# Phase B definition of done

Phase B is done when `tui-vfx-next` has a minimal non-identity sampling path and tests prove all of these cases:

```text
1. A shifted sampler copies cells from different source coordinates.

2. Copied cells receive the role of the sampled source coordinate, not the destination coordinate.

3. Role scopes match sampled source roles by default.

4. Geometry scopes still match destination-local coordinates by default.

5. Out-of-bounds sampled coordinates skip writes and preserve destination cell + role.

6. Zero-cell scope diagnostics are based on actual sampled-source semantics.

7. Destination-role scopes are available and behave differently from sampled-source role scopes.

8. Empty transparent writes remain distinct from skipped sampled cells.
```

Still not in Phase B:

```text
full recipes
studio manifest
phase engine
trigger engine
legacy migration
real CRT/typewriter/shadow porting
old compositor replacement
```

---

# Copy-paste prompt/spec for Phase B

```text
You are working in the tui-vfx Rust workspace.

Phase A created a clean-room crate:

    crates/tui-vfx-next

and proved the minimal v3.1 semantic surface contract with identity sampling.

Your task is Phase B: prove sampled-source semantics with non-identity sampling.

Goal:
Extend the clean-room `tui-vfx-next` kernel so it can apply a minimal coordinate mapper/sampler where destination coordinates may sample from different source coordinates. This is not a visual effect port. It is a semantic contract proof.

Primary question:
Can the v3.1 surface model preserve correct cell, role, scope, skip, and diagnostic behavior when:

    destination coordinate != sampled source coordinate

Hard constraints:
- Do not replace or refactor the legacy compositor.
- Do not port real effects such as CRT, typewriter, matrix rain, or shadow.
- Do not add recipe compiler, studio manifest, phase graph, trigger engine, or runtime binding system.
- Do not add legacy aliases.
- Do not depend on `tui-vfx-compositor`, `tui-vfx-style`, `tui-vfx-content`, or `tui-vfx-shadow`.
- Keep the phase test-focused and small.

Allowed dependencies:
- `tui-vfx-types`
- `tui-vfx-geometry` if needed

Implementation requirements:

1. Add a minimal sampling abstraction.

Suggested shape:

    pub trait CoordinateSampler {
        fn sample(&self, dest_x: u16, dest_y: u16, width: u16, height: u16)
            -> Option<(u16, u16)>;
    }

or equivalent.

2. Implement at least two samplers:

    IdentitySampler
    ShiftSampler { dx: i16, dy: i16 }

The shift sampler should allow sampled source coordinates to differ from destination coordinates.

3. Preserve the Phase A defaults:

    geometry scopes:
        CoordinateSpace::DestinationLocal

    role scopes:
        RoleSpace::SampledSource

4. Ensure copy/write behavior uses the sampled source role when `RoleWritePolicy::CopySampledSource` is selected.

5. Ensure out-of-bounds sampled coordinates skip the write and preserve destination cell and role.

6. Ensure zero-cell scope diagnostics reflect the same semantics used by actual writes. Do not pre-tally scopes in a way that disagrees with the sampled-source write path.

7. Update docs:

    docs/v3.1-surface-contract.md

Add a Phase B section or amend the coordinate-space / role-space sections to describe non-identity sampling.

Required tests:

1. `shift_sampler_copies_sampled_source_cell`
   - Destination position samples a different source position.
   - The copied glyph proves non-identity sampling occurred.

2. `shift_sampler_copies_sampled_source_role`
   - Destination position starts with one role.
   - Sampled source position has another role.
   - After copy, destination role equals sampled source role.

3. `role_scope_uses_sampled_source_role_with_shift`
   - Use a role scope such as `Role(Text)`.
   - Arrange destination coordinate role and sampled source role to differ.
   - Prove matching follows sampled source role by default.

4. `geometry_scope_uses_destination_local_with_shift`
   - Use a rect/row/column scope.
   - Prove geometry matching is based on destination coordinate, not sampled source coordinate.

5. `out_of_bounds_sample_preserves_destination`
   - Shift sampler returns out-of-bounds for at least one destination cell.
   - That cell’s destination cell and destination role remain unchanged.

6. `zero_cell_scope_with_sampler_emits_diagnostic`
   - Use a scope that matches no sampled-source roles.
   - Verify structured `ZeroCellScope` diagnostic and no mutation.

7. `destination_role_space_can_be_selected`
   - Use `RoleSpace::Destination`.
   - Prove it behaves differently from `RoleSpace::SampledSource`.

8. `transparent_empty_sample_write_is_not_skip`
   - Preserve the Phase A distinction between writing an empty transparent cell and skipping due to out-of-bounds/scope.

Suggested command checks:

    cargo fmt --package tui-vfx-next -- --check
    cargo clippy -p tui-vfx-next --all-targets -- -D warnings
    cargo test -p tui-vfx-next
    cargo tree -p tui-vfx-next

Deliverables:
- Code changes in `tui-vfx-next`
- Updated `docs/v3.1-surface-contract.md`
- Tests proving the semantics above
- Short status memo summarizing decisions and open questions

Definition of done:
Phase B is done when non-identity sampling is proven without depending on the legacy compositor, and every role/scope/write/skip diagnostic test passes.
```

---

# Phase B success criteria

A reviewer should be able to answer these questions after Phase B:

```text
When a sampler shifts source coordinates, which role is copied?
When a role scope is evaluated, does it inspect destination role or sampled-source role?
When a geometry scope is evaluated, does it inspect destination or source coordinates?
What happens when sampling goes out of bounds?
Can zero-cell diagnostics disagree with actual writes?
Can destination-role scope be selected intentionally?
Is empty transparent write still distinct from skip?
```

The desired answers are:

```text
Copied cells use sampled-source role.
Role scopes default to sampled-source role.
Geometry scopes default to destination-local coordinates.
Out-of-bounds sampling skips and preserves destination.
Diagnostics match actual sampled behavior.
Destination-role scope is explicit, not accidental.
Empty transparent write remains a write unless skipped by policy.
```

---

# What I would not do next

I would not yet do:

```text
tui-vfx-contract / tui-vfx-engine split
full effect descriptors
recipe compiler
studio manifest
legacy bridge
filter/mask/sampler ports
StyleRegion migration
old compositor refactor
```

Those are tempting, but Phase B should kill the biggest semantic ambiguity first: sampled-source behavior.

---

# After Phase B

Once non-identity sampling is proven, I would choose between two paths:

```text
Phase C1:
    split tui-vfx-next into tui-vfx-contract + tui-vfx-engine

Phase C2:
    add minimal effect descriptors and a tiny registry
```

I would probably do the split first if `tui-vfx-next` starts accumulating both data contracts and execution code. Until then, keeping it as an incubator is fine.

---

# Architect note

Phase A’s clean-room direction was the right move. The next danger is accidentally designing descriptors or recipes before sampled-source semantics are fully nailed down.

So the next bounded milestone should be:

```text
Make sampling real.
Prove roles still work.
Then build descriptors.
```

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_A.md</FILE> - <DESC>Architect response to Phase A status memo</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
