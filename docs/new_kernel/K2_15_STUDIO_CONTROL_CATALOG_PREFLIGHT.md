# K2.15 studio control/catalog preflight

K2.15 adds more player evidence but does not promote studio-control derivation to a release gate.

## New control evidence available

- Graph topology kind: sequence, parallel, node.
- Merge policy: child-order last-writer and error-on-conflict paths are represented in the player executor.
- Graph value sources and input re-emission outputs now have player evidence.
- Source controls for ANSI text, image asset id, procedural generator, width, height, and seed remain descriptor-pack-backed.

## Not ready for catalog promotion

- Backend/compositor lowering is not yet a stable player backend adapter.
- Scene visibility controls are not a complete runtime model.
- ANSI/image/procedural sources are intentionally bounded evidence, not visual parity.

Studio control derivation should wait until the player IR/backend seam and scene/layer fidelity blockers are resolved.

