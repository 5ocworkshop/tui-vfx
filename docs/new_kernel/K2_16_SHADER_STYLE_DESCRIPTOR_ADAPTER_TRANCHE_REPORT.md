# K2.16 shader/style descriptor/adapter tranche report

## Added canonical shader/style fixtures

- `shaders/primitives/shader_linear_gradient_background_channel.json`
- `shaders/primitives/shader_linear_gradient_diagonal.json`
- `shaders/primitives/shader_reveal_wipe.json`
- `shaders/primitives/shader_reveal_wipe_right_to_left.json`
- `styles/style_modulo_columns_period.json`

## Evidence covered

The tranche covers foreground/background style channels, a multi-stop gradient input, a direction enum, a numeric angle/intensity input, and an accepted built-in scope (`moduloColumns`).

## Holdbacks

Shader/style backlog remains large for effects that require backend/subcell/light propagation semantics. No compositor parity claim is made.

## Naming note

The legacy `shader_reveal_wipe_corner_in_bottom_right.json` record maps to `shaders/primitives/shader_reveal_wipe_right_to_left.json` because the current v3.1 descriptor exposes the authored behavior as `direction: rightToLeft`; no bottom-right corner direction exists in the accepted descriptor vocabulary.
