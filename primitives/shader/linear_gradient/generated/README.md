# Generated Primitive Workbench artifacts

This directory contains descriptor-derived scaffold artifacts for `shader.linearGradient`:

- `linear_gradient_input_manifest.json` lists descriptor inputs in source order.
- `linear_gradient_inputs.rs` is a checked-in Rust input-shape skeleton, not yet compiled into runtime code.
- `linear_gradient_accessors.rs` reserves descriptor-derived accessor names for the future runtime adapter.
- `linear_gradient_control_catalog.json` records descriptor-derived studio control metadata for this primitive.
- `linear_gradient_validation_manifest.json` lists the fixture and regression commands that guard the slice.

Runtime behavior remains hand-owned under `runtime/` and still uses copied compositor parity until the next vertical step wires generated accessors into compositor-next.
## Provenance

These are bootstrap descriptor-derived scaffold artifacts produced during the compositor-next Ralph slice from `descriptor.v31.json`. They are intentionally checked in as reviewable reference artifacts until the Primitive Workbench CLI owns deterministic regeneration. Do not place hand-owned visual behavior in this directory.
