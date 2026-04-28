<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_B.md</FILE> - <DESC>Architect response to Phase B status memo</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Captured clean-room kernel Phase B architect approval and Phase C recommendation.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add OFPF metadata around captured clean-room kernel planning/status content.</CLOG> -->

I would **approve Phase B** as complete.

Based on the memo, Phase B locked the key ambiguity we needed resolved before building descriptors or recipes:

```text
Geometry scope:
    destination-local by default

Role scope:
    sampled-source by default

Sample out of bounds:
    skip and preserve destination cell + role

Transparent empty sampled cell:
    still a write unless policy says to skip it

Zero-cell diagnostics:
    based on actual sampled-source semantics, not a separate preflight model
```

That is a major architectural win.

# What is now locked

```text
+====================================================================================+
|                                PHASE B LOCKS                                        |
+====================================================================================+

  SURFACE
      [LOCK] Surface remains cell grid + semantic role channel.
      [LOCK] Role remains a surface-position semantic channel, not a Cell field.

  SAMPLING
      [LOCK] Destination coordinate may map to a different sampled source coordinate.
      [LOCK] ShiftSampler semantics are explicit:
                 destination (x, y) -> source (x + dx, y + dy)
      [LOCK] Out-of-bounds sample returns None and skips the write.

  ROLE SEMANTICS
      [LOCK] Copied cells receive the sampled source role by default.
      [LOCK] Role scopes use RoleSpace::SampledSource by default.
      [LOCK] RoleSpace::Destination is explicit and behaves differently.

  GEOMETRY SEMANTICS
      [LOCK] Geometry scopes use CoordinateSpace::DestinationLocal by default.
      [LOCK] Geometry matching does not silently switch to sampled-source coordinates.

  WRITE / SKIP
      [LOCK] Skipped samples preserve destination cell and destination role.
      [LOCK] Empty transparent sampled writes are not skipped unless the write policy says so.

  DIAGNOSTICS
      [LOCK] Zero-cell diagnostics are sampler-aware.
      [LOCK] Diagnostics match actual write semantics.

  BOUNDARY
      [LOCK] tui-vfx-next remains independent of compositor/style/content/shadow crates.
```

# My recommendation for the next phase

Do **not** start full effect descriptors or recipe schema yet.

The next semantic risk is **multi-stage pipeline behavior**.

Phase A proved a surface.
Phase B proved non-identity sampling.
Phase C should prove what happens when multiple operations run in sequence.

In other words, we need to answer:

```text
When stage 1 writes cells and roles,
what does stage 2 read?

Does stage 2 scope against:
    the original source surface?
    the current pipeline surface?
    the destination before the pipeline?
    some explicit named surface?

When a stage skips a cell,
does it preserve the previous stage output or the pre-pipeline destination?
```

Do that before descriptors, because effect descriptors need to describe the execution model they participate in.

So I would define:

```text
Phase C — Linear Pipeline / Pass Semantics
```

not descriptor/schema work yet.

---

# Phase C goal

```text
Can the clean-room kernel execute multiple ordered stages while preserving the
same surface, sampling, scope, write, skip, and diagnostic semantics proven in
Phases A and B?
```

A minimal Phase C pipeline should look like this:

```text
+====================================================================================+
|                              PHASE C — LINEAR PIPELINE                              |
+====================================================================================+

        +-------------------------+
        | Initial Source Surface  |
        | cells + roles           |
        +------------+------------+
                     |
                     v
        +-------------------------+
        | Stage 1                 |
        | copy / sample / write   |
        | writes cells + roles    |
        +------------+------------+
                     |
                     v
        +-------------------------+
        | Current Surface         |
        | result of Stage 1       |
        +------------+------------+
                     |
                     v
        +-------------------------+
        | Stage 2                 |
        | visual-only dim/tint    |
        | role-scoped             |
        | preserves roles         |
        +------------+------------+
                     |
                     v
        +-------------------------+
        | Current Surface         |
        | result of Stage 2       |
        +------------+------------+
                     |
                     v
        +-------------------------+
        | Stage 3                 |
        | explicit role writer    |
        | procedural/shadow-like  |
        +------------+------------+
                     |
                     v
        +-------------------------+
        | Final Surface           |
        | cells + roles + diags   |
        +-------------------------+
```

The important contract is:

```text
Each stage reads from a well-defined read surface and writes to a well-defined
write surface. Stage order is semantic.
```

---

# What Phase C should lock

```text
+====================================================================================+
|                                PHASE C LOCK TARGETS                                 |
+====================================================================================+

  PIPELINE SURFACE FLOW
      [LOCK] What surface a stage reads.
      [LOCK] What surface a stage writes.
      [LOCK] Whether stages are in-place or use read/write buffers.
      [LOCK] Whether later stages see earlier stage role writes.

  STAGE ORDER
      [LOCK] Stage order is semantic and deterministic.
      [LOCK] Reordering stages may change output.

  CURRENT SURFACE
      [LOCK] Define "current surface" explicitly.
      [LOCK] RoleSpace::SampledSource means sampled role from the stage's read surface.

  SKIP BEHAVIOR ACROSS STAGES
      [LOCK] A skipped cell preserves the current destination for that stage.
      [LOCK] Skipping in Stage 2 does not rewind to the initial source.

  DIAGNOSTICS
      [LOCK] Diagnostics include stage identity.
      [LOCK] Zero-cell scope is evaluated per stage.
      [LOCK] Diagnostics are ordered deterministically.

  MINIMAL PIPELINE API
      [LOCK] A tiny stage/pipeline abstraction exists.
      [LOCK] It is still not a full descriptor registry.
```

The key wording I would introduce is:

```text
sampled source role = role from the sampled coordinate of the stage's read surface
```

That avoids future confusion when the “source” is no longer the original input but the current pipeline surface.

---

# Should we split `tui-vfx-next` now?

I would **not physically split crates yet** unless the agent says it is becoming painful.

Instead, Phase C should enforce an internal boundary:

```text
crates/tui-vfx-next/src/contract/
    surface
    scope
    write
    diagnostic
    sampler traits
    tiny effect descriptors if needed

crates/tui-vfx-next/src/engine/
    pipeline execution
    stage runner
    test helpers
```

or keep the existing files but document the boundary clearly.

A physical split into:

```text
tui-vfx-contract
tui-vfx-engine
```

becomes more valuable after Phase C proves the pipeline model. Splitting too early can create churn while the model is still moving.

So my Phase C recommendation is:

```text
Logical boundary now.
Physical crate split after Phase C or D.
```

---

# Phase C definition of done

Phase C is done when the clean-room kernel has a tiny ordered pipeline and tests prove:

```text
1. Later stages read earlier stage output.

2. Later stages see earlier stage role writes.

3. Stage order affects output and is deterministic.

4. A visual-only stage preserves roles written by a prior copy/procedural stage.

5. A role-scoped stage matches roles on its stage read surface.

6. A skipped cell in Stage 2 preserves the Stage 1 output, not the original source.

7. Zero-cell diagnostics include stage identity.

8. Diagnostics are deterministic and ordered.

9. The clean-room crate still does not depend on legacy compositor/style/content/shadow crates.

10. No real effects, recipes, studio, runtime graph, trigger engine, or legacy migration are introduced.
```

---

# Copy-paste Phase C prompt for the agent

```text
You are working in the tui-vfx Rust workspace.

Phases A and B created and extended the clean-room crate:

    crates/tui-vfx-next

Phase A proved the semantic surface contract with identity sampling.
Phase B proved sampled-source semantics with non-identity sampling.

Your task is Phase C: prove ordered multi-stage pipeline semantics.

Goal:
Extend `tui-vfx-next` with a minimal ordered pipeline/pass model that can execute multiple tiny stages while preserving the surface, sampling, scope, write, skip, and diagnostic semantics proven in Phases A and B.

Primary question:
Can the clean-room kernel define exactly what each stage reads and writes, and can later stages observe earlier stage cell and role writes?

Hard constraints:
- Do not replace or refactor the legacy compositor.
- Do not port real effects such as CRT, typewriter, matrix rain, or shadow.
- Do not add recipe compiler, studio manifest, phase graph, trigger engine, or runtime binding system.
- Do not add legacy aliases.
- Do not depend on `tui-vfx-compositor`, `tui-vfx-style`, `tui-vfx-content`, or `tui-vfx-shadow`.
- Keep the phase small and test-focused.
- Treat `DRAFT_CONTRACTS.md` as directional context only; do not implement the whole descriptor/schema system yet.
- Use the name V3.1 / v3.1 consistently. Do not call this v2.

Allowed dependencies:
- `tui-vfx-types`
- `tui-vfx-geometry` only if needed

Implementation requirements:

1. Add a minimal ordered pipeline abstraction.

Suggested shape, but adapt if the existing Phase A/B code suggests a better fit:

    pub struct SurfacePipeline { ... }

    pub trait SurfaceStage {
        fn name(&self) -> &str;
        fn apply(
            &self,
            read: &Surface,
            write: &mut Surface,
            diagnostics: &mut Vec<SurfaceDiagnostic>,
        );
    }

or an equivalent enum-based model.

2. Define the stage read/write rule clearly.

Recommended Phase C rule:

    Each stage reads from the current surface and writes into the next surface.
    After the stage completes, next becomes current.
    Therefore later stages see earlier stage cell and role writes.

If you choose a different rule, document it clearly and update tests accordingly.

3. Clarify `sampled source role` wording.

In a multi-stage pipeline, `sampled source role` means:

    the role at the sampled coordinate of the stage's read surface

not necessarily the original input surface.

4. Keep the stages tiny.

Use existing toy semantics only:
- copy/sample stage
- dim/tint visual-only stage
- explicit role writer stage
- optionally a scoped no-op/write stage for diagnostics

Do not port real compositor effects.

5. Update docs:

    docs/v3.1-surface-contract.md

Add a Phase C section describing:
- current surface
- stage read surface
- stage write surface
- stage order
- stage skip behavior
- stage diagnostics
- sampled-source role meaning in a multi-stage pipeline

Required tests:

1. `pipeline_later_stage_reads_earlier_stage_cells`
   - Stage 1 writes or copies a distinctive glyph.
   - Stage 2 reads/applies to that glyph.
   - Final output proves Stage 2 saw Stage 1 output.

2. `pipeline_later_stage_reads_earlier_stage_roles`
   - Stage 1 writes role `Text` or `Shadow`.
   - Stage 2 uses a role scope matching that role.
   - Final output proves Stage 2 matched the role written by Stage 1.

3. `pipeline_stage_order_is_semantic`
   - Run the same two stages in opposite order.
   - Prove final output differs.

4. `visual_stage_preserves_prior_stage_roles`
   - Stage 1 copies or writes roles.
   - Stage 2 dim/tint changes visual channels only.
   - Final roles remain from Stage 1.

5. `stage_skip_preserves_current_surface`
   - Stage 1 writes a cell and role.
   - Stage 2 skips that coordinate.
   - Final output preserves Stage 1 cell and role.

6. `stage_zero_cell_scope_diagnostic_names_stage`
   - A scoped stage matches zero cells.
   - Diagnostic includes stable zero-cell code and stage identity/name/path.

7. `pipeline_diagnostics_are_deterministic`
   - Multiple stages emit diagnostics.
   - Diagnostic order is stable and follows stage order.

8. `pipeline_keeps_phase_b_sampled_role_semantics`
   - Use non-identity sampling inside a pipeline stage.
   - Prove sampled-source roles still come from the stage read surface.

Suggested command checks:

    cargo fmt --package tui-vfx-next -- --check
    cargo clippy -p tui-vfx-next --all-targets -- -D warnings
    cargo test -p tui-vfx-next
    cargo test --workspace
    cargo tree -p tui-vfx-next
    grep -R -nE 'tui_vfx_(compositor|style|content|shadow)|tui-vfx-(compositor|style|content|shadow)' crates/tui-vfx-next

Deliverables:
- Code changes in `tui-vfx-next`
- Updated `docs/v3.1-surface-contract.md`
- Tests proving Phase C semantics
- Short status memo summarizing:
  - what was added
  - what was deliberately not added
  - verification commands/results
  - open questions

Definition of done:
Phase C is done when an ordered multi-stage clean-room pipeline proves that later stages see earlier stage cell and role writes, stage order is deterministic and semantic, skipped cells preserve the current stage destination, diagnostics are stage-aware and deterministic, and the crate remains independent of legacy compositor/style/content/shadow crates.
```

---

# Why not descriptors/schema next?

Effect descriptors describe what effects can do:

```text
reads/writes cell channels
supports scopes
preserves/writes roles
uses sampling
emits events
has lifecycle/completion behavior
```

But before descriptors can be right, the engine must know what a stage means:

```text
What surface does it read?
What surface does it write?
Can later stages see its role writes?
What does skip preserve?
What does sampled source mean after Stage 1?
```

Phase C answers those questions.

After Phase C, descriptors become much easier and less speculative.

---

# Updated roadmap position

```text
+====================================================================================+
| CURRENT ROADMAP POSITION                                                            |
+====================================================================================+

  [DONE]   Phase A — semantic surface contract
  [DONE]   Phase B — sampled-source semantics
  [NEXT]   Phase C — ordered multi-stage pipeline semantics
  [LATER]  Phase D — contract/engine split and generalized ScopeSpec/write model
  [LATER]  Phase E — effect descriptors
  [LATER]  Phase F — value/parameter/source model
  [LATER]  Phase G — node graph
  [LATER]  Phase H — strict recipe v3.1 schema/compiler
  [LATER]  Phase I — phase/trigger engine
  [LATER]  Phase J — first real effect ports
```

The new immediate rule is:

```text
Do not describe real effects until we know what a stage is.
```

Phase C should lock that.

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_B.md</FILE> - <DESC>Architect response to Phase B status memo</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
