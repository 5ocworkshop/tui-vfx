<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/58_normalized_ir_and_authoring_types.md</FILE> - <DESC>Chapter 58 — normalized IR and authoring-type implementation phase. Defines why authoring types and normalized IR must be implemented together, what the normalized IR should erase/resolve, and what the minimum first implementation slice should cover.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Follows the capability catalog and lowering-rules phases. This chapter turns the normalized-IR recommendation into an implementation planning surface with concrete responsibilities and minimum guarantees.</WCTX> -->
<!-- <CLOG>1.0.0: initial chapter. Establishes normalized IR as the execution-facing representation, defines authoring-type vs normalized-IR responsibilities, and lists the first required canonicalization passes.</CLOG> -->

# 58 — Normalized IR and Authoring Types

V3 now has three distinct layers that should not be collapsed into one implementation type:

1. **Authoring schema**
2. **Lowering / canonicalization rules**
3. **Execution-facing normalized IR**

This chapter defines the third layer and its relationship to the first two.

---

## 10 — Why normalized IR is required

The authoring schema is intentionally ergonomic.
That means it allows forms that are good for authors, but not ideal as a runtime or tooling contract.

Examples already present in V3:

- region refs vs inline scopes
- `cell_runs` compression vs explicit `cells`
- style-native spatial wrappers vs sibling shader steps
- hybrid templates lowered into ordinary tree structure
- scene-layer conveniences that should normalize before execution

A runtime that executes the raw authoring schema directly will accumulate special cases quickly.
A validator that validates only the raw authoring schema will have to re-implement canonicalization logic in many places.

So the rule is:

> The runtime, validator, viewer, and migration-equivalence tooling should target normalized IR, not raw authoring syntax.

---

## 20 — Authoring types vs normalized IR

### Authoring types

These preserve ergonomic structure:
- `config.regions`
- `scope.kind = region_ref`
- wrapper/router payloads
- explicit hybrid templates lowered from V2
- user-facing field names and convenience forms

### Normalized IR

This should erase or resolve:
- region refs
- compressed selectors (`cell_run`, `cell_runs`) into canonical region representations
- default propagation
- style normalization differences
- wrapper lowering where the execution engine does not need wrapper structure
- explicit phase/scope inheritance after propagation is resolved

The normalized IR is not a second public schema.
It is the execution-facing internal contract.

---

## 30 — Minimum first normalized-IR pass set

The first implementation slice should at minimum canonicalize:

1. **Defaults**
   - explicit default scope
   - explicit default phase
   - explicit timing defaults

2. **Region resolution**
   - `region_ref` → concrete scope
   - `cell_run` / `cell_runs` → canonical concrete region form

3. **Style normalization**
   - singular/plural style history disappears
   - `base_style_override` normal form becomes one canonical representation
   - style-native spatial wrappers normalized consistently

4. **Wrapper/hybrid lowering**
   - where runtime does not need the wrapper form, lower it
   - where wrapper identity matters for tooling/debugging, preserve it in metadata

5. **Scene-layer normalization**
   - source / placement / surface / pipeline defaults made explicit
   - sibling-relative placement validated and canonicalized

6. **Contract discovery normalization**
   - required bindings / tokens / assets / primitives resolved into a canonical contract block for validator/tooling use

---

## 40 — What normalized IR should preserve

Even after canonicalization, normalized IR should still preserve:

- lane identity (`mask` / `sampler` / `filter` / `shader` / `style_effect`)
- explicit scene-layer boundaries
- hint producer/consumer structure
- payload family identity where execution semantics differ
- enough provenance/debug metadata to explain where a node came from when needed

The point is not to flatten away meaning.
The point is to remove authoring sugar and ambiguity.

---

## 50 — First implementation target

The first implementation target should produce:

1. authoring-layer parse types
2. canonicalization passes
3. normalized IR type definitions
4. validation on both authoring schema and normalized IR

Only after that should large-scale family execution work begin.

---

## 60 — Execution companion

The live execution companion for this phase is:

- `docs/design/tui-vfx-v3-normalized-ir.md`

That document is where the concrete normalized-IR shape and first passes should be refined as the work moves from planning into code.

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/58_normalized_ir_and_authoring_types.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
