# shader.linearGradient — compositor-next slice foundation

This is the first existing shader primitive tree for the compositor-next vertical workflow.

Selection note: `shader.highlighter` remains a desired early primitive, but the Phase 3.5 descriptor audit found owner-decision blockers for `textContrast` and `rowMask` descriptor kinds. `shader.linearGradient` is safer for proving the co-located primitive workflow without hardening known descriptor mismatches.

Current status:

- Descriptor copied from `descriptors/v3.1/packs/primitive.json`.
- Field coverage manifest exists.
- Runtime behavior still uses copied compositor parity path.
- No existing `v3.1/debug_recipes` were mutated.


Generated scaffold status:

- Bootstrap descriptor-derived artifacts are checked in under `generated/`.
- They are review/reference scaffold files, not yet compiled runtime code.
- Future Primitive Workbench tooling must prove deterministic regeneration before this pattern is scaled broadly.
