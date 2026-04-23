# Packet 11 — motion-disabled demo UX

## Objective
Finish or polish the demo support that disables motion paths while keeping effects centered and clearly labeled as motion-disabled.

## Why this matters
Users need to separate motion from effect semantics during evaluation. The demo should make that easy and obvious.

## Mode
BLOCKER_MODE

## Success condition
- motion-disabled mode is visible and understandable
- the on-screen state clearly signals motion-disabled mode
- behavior is verified through the demo/example surface

## In scope
- `/usr/projects/tui-vfx-recipes/examples/` demo surfaces
- related state/UI handling for the toggle

## Out of scope
- unrelated recipe/validator work
- broad demo redesign

## UI expectation
- motion-disabled state should be unmistakable
- user should be able to assess effect behavior without motion-path interference
- highlight should visually draw attention without being confusing

## Verification required
- `cargo check -p tui-vfx-recipes --example <relevant-example>`
- any narrow example/demo tests if present
- exact proof of the visible state behavior

## Reporting format
Report:
- exact files changed
- exact toggle behavior
- exact verification commands
- any remaining UX caveat

## Task reminder
Your task is still: polish the motion-disabled demo behavior, not branch into unrelated demo cleanup.
