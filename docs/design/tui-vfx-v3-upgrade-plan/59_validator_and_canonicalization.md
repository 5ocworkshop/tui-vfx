<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/59_validator_and_canonicalization.md</FILE> - <DESC>Chapter 59 — validator and canonicalization phase. Defines the execution contract for schema validation, normalized-IR validation, lowering invariants, and migration-equivalence checks before large runtime family work begins.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Follows the normalized-IR phase. This chapter answers what the validator/canonicalizer must prove before the runtime grows broad family support.</WCTX> -->
<!-- <CLOG>1.0.0: initial chapter. Establishes validator scope across authoring schema, lowering, normalized IR, and migration-equivalence; identifies first canonical checks and output artifacts.</CLOG> -->

# 59 — Validator and Canonicalization Phase

Once the authoring schema, capability catalog, lowering rules, and normalized IR shape exist, the next mandatory layer is validation and canonicalization.

This chapter exists to make one thing explicit:

> Runtime execution should not be the first place that schema mistakes are discovered.

---

## 10 — What this phase is responsible for

The validator/canonicalizer should validate **three levels**:

1. **Authoring schema validity**
2. **Lowering validity**
3. **Normalized IR validity**

And it should produce one stable canonical output:

- **normalized IR** suitable for tooling and runtime consumption

---

## 20 — Validation layers

### 20.1 Authoring-schema validation

This checks:
- required fields present
- field types valid
- enum/tag values valid
- references syntactically valid
- contracts declared consistently (`requires_*`)

### 20.2 Lowering validation

This checks:
- V2→V3 structural lifts succeeded
- normalization rules were applied consistently
- ambiguous cases were not silently guessed when human-review rules should have triggered
- wrapper / hybrid lowering produced valid ordinary tree structure

### 20.3 Normalized-IR validation

This checks:
- no unresolved refs remain
- scope/phase defaults are explicit
- placement is canonicalized
- hint producer/consumer relationships are valid
- no contradictory tree structures survive

---

## 30 — First required canonical checks

At minimum, the validator/canonicalizer must eventually cover:

1. **Scope coherence**
   - valid region refs
   - valid resolved cell/row/column ranges
   - no impossible combinations

2. **Phase coherence**
   - legal phase values
   - valid propagation / override behavior
   - no empty or contradictory structural nodes

3. **Hint coherence**
   - every `HintRef` resolves to exactly one visible producer in the intended scope
   - duplicate visible producers are rejected unless explicitly supported later

4. **Scene coherence**
   - sibling-relative placement resolves correctly
   - per-layer fields normalize cleanly
   - layer-local pipelines are structurally sound

5. **Style normalization coherence**
   - no lingering dual normal forms for style patches
   - wrapper forms and sibling shader forms canonicalize predictably

6. **Contract coherence**
   - `requires_tokens`
   - `requires_bindings`
   - `requires_assets`
   - `requires_primitives`
   are internally consistent with actual recipe usage

---

## 40 — Migration-equivalence checks

The validator/canonicalizer also has a migration role.

It should support proving that:

- important V2 recipes and their V3 counterparts lower to equivalent normalized intent
- critical fixtures still produce expected behavior after migration
- family-level migration did not silently drift in semantics

This does **not** mean every recipe needs pixel-perfect equivalence.
But it does mean the migration infrastructure needs a place to express and test equivalence where it matters.

---

## 50 — Canonical outputs of this phase

The phase should eventually produce or standardize:

- authoring validation report
- lowering validation report
- normalized IR artifact / dump
- migration-equivalence report
- human-review-needed report for unresolved lowering cases

This is the layer that makes the ecosystem auditable.

---

## 60 — What should still be deferred to runtime

Validator/canonicalization should **not** attempt to execute:

- runtime bindings
- signal graph evaluation
- procedural generator frame output
- per-frame hint values
- renderer-specific frame production

Its job is to validate and normalize structure, not to simulate time.

---

## 70 — Immediate execution companion

The live execution companion for this phase is:

- `docs/design/tui-vfx-v3-validator-canonicalization-checklist.md`

That file should hold the concrete checklist and implementation checklist as the phase moves from planning to code.

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/59_validator_and_canonicalization.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
