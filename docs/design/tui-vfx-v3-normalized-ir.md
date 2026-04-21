<!-- <FILE>docs/design/tui-vfx-v3-normalized-ir.md</FILE> - <DESC>Working design note for the V3 normalized IR. Defines the execution-facing representation the validator, viewer, and runtime should converge on after lowering and canonicalization.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Initial seed after schema hardening, capability cataloging, and lowering-rule definition. This document is deliberately closer to implementation concerns than the author-facing schema draft.</WCTX> -->
<!-- <CLOG>0.1.0: initial normalized-IR note. Establishes the first-pass canonicalization goals and a candidate IR shape.</CLOG> -->

# tui-vfx V3 normalized IR

This document describes the execution-facing representation that should sit between:

- raw authoring schema
- and runtime execution / validation / tooling

It is not the user-facing recipe schema.
It is the canonicalized form that tools and runtime code should target.

---

## 1. Goals

The normalized IR should:

- erase authoring sugar
- make defaults explicit
- make propagation explicit
- make region/placement resolution explicit
- preserve semantic meaning needed for execution
- be stable enough for validator, viewer, and runtime code to share

---

## 2. Candidate top-level shape

```text
NormalizedRecipe
├─ identity
├─ contracts
├─ envelope
│  ├─ layout
│  ├─ lifecycle
│  ├─ border
│  ├─ clock?
│  ├─ theme?
│  └─ shadow?
├─ scene?
│  └─ layers[]
│     ├─ id
│     ├─ role_tag
│     ├─ source
│     ├─ placement
│     ├─ surface
│     └─ pipeline
└─ pipeline
   ├─ timing
   └─ step?
```

Where every `step` is already normalized so that:
- inherited scope/phase are explicit
- region refs are resolved
- wrapper forms are either preserved intentionally or lowered consistently

---

## 3. Candidate normalized Step shape

```text
NormalizedStep
├─ kind
├─ phase
├─ scope
├─ clock?
├─ interaction?
├─ payload
└─ provenance?
```

### Notes

- `phase` is always explicit
- `scope` is always explicit
- `payload` is already in its canonical family-specific form
- `provenance` is optional runtime/tooling metadata, not part of the public authoring schema

---

## 4. First canonicalization passes

### Pass A — envelope defaults

- fill default scope/phase/timing values
- normalize clock defaults
- normalize contract blocks

### Pass B — region resolution

- resolve `region_ref`
- expand `cell_run` / `cell_runs` into canonical internal region form
- validate out-of-bounds or contradictory regions

### Pass C — style normalization

- normalize style patches into one canonical internal representation
- normalize style-native spatial wrappers consistently
- normalize singular/plural historical style forms away

### Pass D — wrapper/hybrid normalization

- lower reusable hybrid templates into ordinary tree structure
- preserve provenance about the source template/wrapper if needed for tooling

### Pass E — scene normalization

- resolve sibling-relative placement
- make per-layer defaults explicit
- validate layer-local source/surface/pipeline coherence

---

## 5. What should remain unresolved until runtime

Normalized IR should **not** prematurely evaluate:

- runtime bindings
- signal graph evaluation
- per-frame hint values
- procedural generator frame output

Those are runtime concerns.

The IR should normalize structure, not execute it.

---

## 6. Immediate next implementation target

If implementation starts tomorrow, the first useful code target would be:

1. parse authoring schema into authoring types
2. canonicalize into normalized IR
3. validate normalized IR
4. expose normalized IR to viewer / validator / runtime

That would let execution code grow under a stable contract.

<!-- <FILE>docs/design/tui-vfx-v3-normalized-ir.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
