<!-- <FILE>docs/new_kernel/PROCESS_OVERVIEW_DIAGRAM.md</FILE> - <DESC>Clean-room kernel process overview diagram</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Roadmap diagram for progressive clean-room contract locks.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add OFPF metadata around captured clean-room kernel planning/status content.</CLOG> -->

Absolutely. I would walk us through this as a sequence of **contract locks**, not feature locks. The point is to avoid porting real effects until the semantics underneath them are stable.

```text
+==================================================================================================+
|                                TUI-VFX v3.1 CONTRACT ROADMAP                                      |
|                          Clean-room kernel first, legacy engine as oracle                         |
+==================================================================================================+

  LEGEND
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [DONE]       already completed / accepted
      [ACTIVE]     current implementation phase
      [LOCK]       decision becomes architectural contract
      [SPIKE]      exploratory, allowed to change
      [PORT]       old effects begin moving into the new model
      [GATE]       must pass before moving forward


+==================================================================================================+
| PHASE A — SEMANTIC SURFACE CONTRACT                                                              |
| Status: DONE                                                                                     |
+==================================================================================================+

        +--------------------------+
        |  tui-vfx-next             |
        |  clean-room crate         |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        |  Surface                  |
        |  cells + roles + metadata |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        |  identity sampling        |
        |  role-preserving writes   |
        |  zero-cell diagnostics    |
        +--------------------------+

  LOCKED IN
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [LOCK] Semantic surface is the canonical v3.1 render primitive.
      [LOCK] A surface position has visual cell channels plus one semantic role.
      [LOCK] Roles are not part of Cell itself; they are a parallel semantic channel.
      [LOCK] Visual-only effects preserve roles.
      [LOCK] Copy operations copy sampled-source roles.
      [LOCK] Skipped cells preserve destination cell and destination role.
      [LOCK] Empty transparent write is not the same as skip.
      [LOCK] Zero-cell scope emits structured diagnostics.
      [LOCK] v3.1 clean-room kernel does not depend on legacy compositor/style/content/shadow crates.

  NOT LOCKED YET
  ────────────────────────────────────────────────────────────────────────────────────────────────

      Non-identity sampling.
      Layer stack semantics.
      Full effect descriptors.
      Recipe schema.
      Runtime phase/trigger engine.
      Effect porting pattern.


+==================================================================================================+
| PHASE B — SAMPLED-SOURCE SEMANTICS                                                               |
| Status: ACTIVE / IN PROGRESS                                                                     |
+==================================================================================================+

        +--------------------------+
        | destination coordinate    |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        | sampler / coordinate map  |
        | identity, shift, later FX |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        | sampled source coordinate |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        | sampled source cell       |
        | sampled source role       |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        | scope evaluation          |
        | write policy              |
        | diagnostics               |
        +--------------------------+

  LOCKS WE WANT FROM PHASE B
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [LOCK] Role scopes default to sampled-source role space.
      [LOCK] Geometry scopes default to destination-local coordinate space.
      [LOCK] Non-identity sampling copies the sampled source cell and sampled source role.
      [LOCK] Out-of-bounds samples skip writes and preserve destination state.
      [LOCK] Scope diagnostics use the same semantics as actual writes.
      [LOCK] Destination-role scope is explicit, not accidental.
      [LOCK] Sampler semantics are proven before descriptors or recipes are designed.

  GATE
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [GATE] Tests prove shifted sampling, sampled-role copying, destination-local geometry,
             out-of-bounds skip, zero-cell diagnostics, and destination-role-space selection.


+==================================================================================================+
| PHASE C — CONTRACT / ENGINE BOUNDARY                                                             |
| Status: NEXT AFTER PHASE B                                                                       |
+==================================================================================================+

        +----------------------------+
        | tui-vfx-next incubator      |
        +-------------+--------------+
                      |
                      v
        +----------------------------+       +----------------------------+
        | tui-vfx-contract            | ----> | tui-vfx-engine              |
        | pure model types            |       | clean runtime kernel        |
        +----------------------------+       +----------------------------+

  DECISION POINT
  ────────────────────────────────────────────────────────────────────────────────────────────────

      Either keep `tui-vfx-next` as an incubator a little longer,
      or split it into:

          tui-vfx-contract
              pure schema/model/contract types

          tui-vfx-engine
              clean-room execution kernel

  LOCKS WE WANT FROM PHASE C
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [LOCK] Crate dependency direction.
      [LOCK] Which types are pure contract types.
      [LOCK] Which types are execution/runtime types.
      [LOCK] Old implementation crates do not own v3.1 public vocabulary.
      [LOCK] The clean contract crate remains independent of legacy compositor/style/content/shadow.

  GATE
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [GATE] Contract types can compile and test without old implementation crates.
      [GATE] Engine depends on contract, not the other way around.


+==================================================================================================+
| PHASE D — GENERAL SCOPE + WRITE MODEL                                                            |
| Status: AFTER CONTRACT BOUNDARY                                                                  |
+==================================================================================================+

        +--------------------------+
        | ScopeSpec                 |
        | all / role / rect / rows  |
        | cols / cell / modulo      |
        | later: and / or / not     |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        | ScopeEvalInput            |
        | dest coord                |
        | sampled coord             |
        | sampled role              |
        | destination role          |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        | SurfaceWrite              |
        | cell write policy         |
        | role write policy         |
        +--------------------------+

  LOCKS WE WANT FROM PHASE D
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [LOCK] Final initial ScopeSpec vocabulary.
      [LOCK] Coordinate-space vocabulary.
      [LOCK] Role-space vocabulary.
      [LOCK] Cell write policies.
      [LOCK] Role write policies.
      [LOCK] Empty / transparent / skipped / unfilled semantics.
      [LOCK] Zero-cell scope behavior.
      [LOCK] Scope behavior is shared across domains, not shader-only.

  KEY DECISION
  ────────────────────────────────────────────────────────────────────────────────────────────────

      StyleRegion remains legacy/executable adapter for now.
      ScopeSpec becomes v3.1 contract vocabulary.

  GATE
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [GATE] Same ScopeSpec can drive toy shader/filter/copy/procedural writes.
      [GATE] Diagnostics remain structured and deterministic.


+==================================================================================================+
| PHASE E — EFFECT DESCRIPTOR MODEL                                                                |
| Status: AFTER SCOPE / WRITE MODEL                                                                |
+==================================================================================================+

        +----------------------------+
        | EffectDescriptor            |
        +-------------+--------------+
                      |
      +---------------+---------------+----------------+
      |                               |                |
      v                               v                v
+-------------+              +----------------+   +----------------+
| inputs      |              | surface access |   | lifecycle      |
| types       |              | reads/writes   |   | completion     |
| ranges      |              | role policy    |   | mutability     |
| defaults    |              | scope support  |   | events         |
+-------------+              +----------------+   +----------------+

  LOCKS WE WANT FROM PHASE E
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [LOCK] Effect descriptor shape.
      [LOCK] Effect domain vocabulary:
             contentGenerator, contentTransform, cellShader, frameFilter,
             coordinateSampler, mask, shadow, postProcess.
      [LOCK] Input type/value vocabulary.
      [LOCK] Runtime mutability vocabulary.
      [LOCK] Completion semantics vocabulary.
      [LOCK] Cell channel read/write declaration.
      [LOCK] Scope support declaration.
      [LOCK] Descriptor is source of truth for effect capability.

  INITIAL EFFECTS ONLY
  ────────────────────────────────────────────────────────────────────────────────────────────────

      Tiny descriptors for:
          dim/tint style-only effect
          explicit role writer
          identity/shift sampler
          maybe one trivial mask

      No real CRT/typewriter/matrix/shadow port yet.

  GATE
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [GATE] Engine can validate that a node’s requested scope/write policy is supported
             by the effect descriptor.


+==================================================================================================+
| PHASE F — VALUE + PARAMETER CONTRACT                                                             |
| Status: AFTER EFFECT DESCRIPTORS                                                                 |
+==================================================================================================+

        +--------------------------+
        | Value                    |
        | bool / int / number      |
        | text / color / duration  |
        | enum / rect / curve      |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        | ValueSource               |
        | literal                   |
        | parameter                 |
        | signal                    |
        | map/select                |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        | ParameterSpec             |
        | SignalSpec                |
        | BindingSpec               |
        +--------------------------+

  LOCKS WE WANT FROM PHASE F
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [LOCK] Closed v3.1 value type vocabulary.
      [LOCK] ValueSource AST.
      [LOCK] Parameter default vs preset override vs runtime override.
      [LOCK] Signal default/fallback behavior.
      [LOCK] Missing binding behavior.
      [LOCK] Clamping vs rejection policy.
      [LOCK] Smoothing/interpolation ownership.

  GATE
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [GATE] Toy effect inputs can be driven by literals, parameters, and signals.
      [GATE] Invalid value types produce structured diagnostics.


+==================================================================================================+
| PHASE G — NODE GRAPH + MINI PIPELINE                                                             |
| Status: BEFORE FULL RECIPES                                                                      |
+==================================================================================================+

        +--------------------------+
        | Node                     |
        | effect id                |
        | inputs                   |
        | scope                    |
        | write policy             |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        | RuntimeGraph              |
        | ordered nodes             |
        | surfaces                  |
        | diagnostics               |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        | execute frame             |
        | deterministic output      |
        +--------------------------+

  LOCKS WE WANT FROM PHASE G
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [LOCK] What a v3.1 node is.
      [LOCK] Node ordering semantics.
      [LOCK] Whether nodes write in-place, source-to-destination, or layer-to-layer.
      [LOCK] Surface read/write rules for node execution.
      [LOCK] Node-level diagnostics.
      [LOCK] Deterministic frame execution contract.

  STILL NOT FULL RECIPES
  ────────────────────────────────────────────────────────────────────────────────────────────────

      This phase may use Rust-built graphs or small JSON fixtures,
      but it does not need the full public recipe schema yet.

  GATE
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [GATE] A tiny graph with sampler + scope + visual effect + procedural write
             executes predictably and preserves roles.


+==================================================================================================+
| PHASE H — STRICT RECIPE v3.1 SCHEMA + COMPILER                                                   |
| Status: AFTER NODE GRAPH PROOF                                                                   |
+==================================================================================================+

        +--------------------------+
        | recipe.v3.1.json          |
        | strict names              |
        | no aliases                |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        | raw deserialize           |
        | schema validation         |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        | semantic validation       |
        | effect registry check     |
        | type/range check          |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        | compiled runtime graph    |
        +--------------------------+

  LOCKS WE WANT FROM PHASE H
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [LOCK] v3.1 recipe document shape.
      [LOCK] No legacy aliases in v3.1.
      [LOCK] Parameter/node/signal/scope schema.
      [LOCK] Effect requirement/version semantics.
      [LOCK] Unknown fields rejected.
      [LOCK] Compile-time diagnostics structure.
      [LOCK] Schema generated from Rust-owned types.

  GATE
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [GATE] Minimal recipe loads, validates, compiles, and executes.
      [GATE] Bad field/effect/input/type/scope produces structured diagnostics.


+==================================================================================================+
| PHASE I — PHASE ENGINE + TRIGGER CONTRACT                                                        |
| Status: AFTER RECIPE COMPILER                                                                    |
+==================================================================================================+

        +--------------------------+
        | enter                    |
        +------------+-------------+
                     |
              completeWhen
                     |
                     v
        +--------------------------+
        | dwell                    |
        +------------+-------------+
                     |
              completeWhen
                     |
                     v
        +--------------------------+
        | exit                     |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        | done                     |
        +--------------------------+

  LOCKS WE WANT FROM PHASE I
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [LOCK] Required lifecycle phases.
      [LOCK] Additional phase rules.
      [LOCK] Trigger AST.
      [LOCK] Event vs signal semantics.
      [LOCK] Event latching/windowing rules.
      [LOCK] Node completion semantics.
      [LOCK] Manual events.
      [LOCK] Time scopes: recipe / phase / node.

  GATE
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [GATE] enter → dwell → exit works.
      [GATE] node completion can trigger transition.
      [GATE] signal predicate can trigger transition.
      [GATE] ambiguous event AND without latch/window is rejected.


+==================================================================================================+
| PHASE J — FIRST REAL PORTS                                                                       |
| Status: ONLY AFTER CONTRACTS ARE STABLE                                                          |
+==================================================================================================+

        +--------------------------+
        | choose one tiny family    |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        | port as descriptor+node   |
        | not legacy schema clone   |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        | compare legacy behavior   |
        | accept intentional breaks |
        +--------------------------+

  RECOMMENDED PORT ORDER
  ────────────────────────────────────────────────────────────────────────────────────────────────

      1. Dim / Tint / Greyscale-style visual filters
      2. Wipe-style mask
      3. Simple sampler
      4. Simple shader
      5. Simple content generator
      6. Shadow
      7. Typewriter
      8. More complex procedural effects

  LOCKS WE WANT FROM PHASE J
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [LOCK] Effect porting pattern.
      [LOCK] How old executable specs are wrapped or replaced.
      [LOCK] What is intentionally not preserved from legacy behavior.
      [LOCK] Golden test strategy.
      [LOCK] Performance baseline strategy.

  GATE
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [GATE] One real effect family runs through v3.1 descriptor → recipe/compiler → engine.


+==================================================================================================+
| PHASE K — STUDIO MANIFEST + TOOLING CONTRACT                                                     |
| Status: AFTER RECIPE + DESCRIPTOR MODEL                                                          |
+==================================================================================================+

        +--------------------------+
        | compiled recipe           |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        | studio manifest           |
        | controls                  |
        | signals                   |
        | phases                    |
        | diagnostics               |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        | studio / CLI / demos      |
        | consume manifest          |
        | not raw recipe internals  |
        +--------------------------+

  LOCKS WE WANT FROM PHASE K
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [LOCK] Studio manifest schema.
      [LOCK] Control generation rules.
      [LOCK] Used-by links from controls to nodes/inputs.
      [LOCK] Diagnostics presentation contract.
      [LOCK] Preset save semantics.
      [LOCK] Studio does not infer effect behavior from raw recipe internals.

  GATE
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [GATE] Minimal studio/CLI manifest can list controls and update parameters.


+==================================================================================================+
| PHASE L — LEGACY BRIDGE + MIGRATION                                                              |
| Status: AFTER STRICT v3.1 WORKS                                                                  |
+==================================================================================================+

        +--------------------------+
        | legacy recipe / spec      |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        | quarantine loader         |
        | alias handling            |
        | migration diagnostics     |
        +------------+-------------+
                     |
                     v
        +--------------------------+
        | canonical v3.1 output     |
        | strict validation         |
        +--------------------------+

  LOCKS WE WANT FROM PHASE L
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [LOCK] Aliases exist only in migration/legacy bridge.
      [LOCK] Runtime loads strict v3.1 by default.
      [LOCK] Migration report format.
      [LOCK] Manual rewrite vs automated migration policy.
      [LOCK] Legacy compatibility boundary.

  GATE
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [GATE] A legacy fixture migrates to canonical v3.1 and validates strictly.


+==================================================================================================+
| PHASE M — CI / RELEASE GATES                                                                     |
| Status: FINAL HARDENING                                                                          |
+==================================================================================================+

        +--------------------------+
        | schema generation         |
        | descriptor export         |
        | recipe validation         |
        | demo rendering            |
        | golden snapshots          |
        +--------------------------+

  LOCKS WE WANT FROM PHASE M
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [LOCK] CI gates.
      [LOCK] Schema check command.
      [LOCK] Descriptor check command.
      [LOCK] Recipe validation command.
      [LOCK] Demo snapshot policy.
      [LOCK] Performance and allocation budgets.

  GATE
  ────────────────────────────────────────────────────────────────────────────────────────────────

      [GATE] v3.1 artifacts cannot drift silently.
      [GATE] Recipes, descriptors, manifests, and demos validate in CI.
```

The important sequencing rule is this:

```text
+====================================================================================+
|                                ORDERING PRINCIPLE                                   |
+====================================================================================+
|                                                                                    |
|   Surface semantics                                                                |
|        ↓                                                                           |
|   Sampling semantics                                                               |
|        ↓                                                                           |
|   Scope/write semantics                                                            |
|        ↓                                                                           |
|   Effect descriptors                                                               |
|        ↓                                                                           |
|   Node graph                                                                       |
|        ↓                                                                           |
|   Recipe schema                                                                    |
|        ↓                                                                           |
|   Phase/trigger runtime                                                            |
|        ↓                                                                           |
|   Real effect ports                                                                |
|        ↓                                                                           |
|   Studio/migration/CI                                                              |
|                                                                                    |
|   Do not port complex effects before the surface/sampling/scope/write model locks. |
|                                                                                    |
+====================================================================================+
```

And the “locks” stack looks like this:

```text
+==================================================================================================+
|                                       LOCK STACK                                                  |
+==================================================================================================+

  A. SURFACE LOCK
     └─ What is a surface?
     └─ What is a cell?
     └─ What is a role?
     └─ What does skip mean?
     └─ What does empty transparent write mean?

  B. SAMPLING LOCK
     └─ What is sampled source?
     └─ Which coordinate space does geometry use?
     └─ Which role space does role scope use?
     └─ What happens out of bounds?

  C. BOUNDARY LOCK
     └─ Which crate owns contract types?
     └─ Which crate owns execution?
     └─ Which crates must remain legacy implementation details?

  D. SCOPE / WRITE LOCK
     └─ What scopes exist?
     └─ What write policies exist?
     └─ What role policies exist?
     └─ What diagnostics fire?

  E. DESCRIPTOR LOCK
     └─ What can an effect declare?
     └─ What inputs exist?
     └─ What channels can it read/write?
     └─ What lifecycle/events/completion semantics exist?

  F. VALUE / PARAMETER LOCK
     └─ What is a runtime value?
     └─ What is a parameter?
     └─ What is a signal?
     └─ What is a binding?
     └─ What is the resolution order?

  G. NODE GRAPH LOCK
     └─ What is a node?
     └─ What is execution order?
     └─ What surface does each node read/write?

  H. RECIPE LOCK
     └─ What is strict recipe v3.1?
     └─ What fields are allowed?
     └─ What aliases are forbidden?
     └─ What does compilation guarantee?

  I. PHASE / TRIGGER LOCK
     └─ What is enter/dwell/exit?
     └─ What is completion?
     └─ What are events vs signals?
     └─ What does latching/windowing mean?

  J. PORTING LOCK
     └─ How do real legacy effects become v3.1 effects?
     └─ What behavior is preserved?
     └─ What behavior is intentionally broken?

  K. TOOLING LOCK
     └─ What does studio consume?
     └─ What does CLI validate?
     └─ What does CI enforce?
```

My current recommendation is:

```text
Do Phase B next.
Then Phase C.
Then Phase D.
Only then start Phase E descriptors.
```

That means the next three architectural questions we should answer after Phase B are:

```text
1. Does tui-vfx-next stay as incubator, or split into tui-vfx-contract + tui-vfx-engine?
2. Does ScopeSpec become the canonical generalized successor to StyleRegion?
3. What exact write-policy and role-propagation rules must every effect descriptor declare?
```

<!-- <FILE>docs/new_kernel/PROCESS_OVERVIEW_DIAGRAM.md</FILE> - <DESC>Clean-room kernel process overview diagram</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
