<!-- <FILE>docs/design/tui-vfx-2026-04-26-handoff-outstanding.md</FILE> - <DESC>Handoff capture of outstanding work from the 2026-04-26 Phase F session — single source so the next session can pick items off one at a time</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>End of 2026-04-26 session. Phase F (Slice 6.6 §F.1–F.8) shipped clean. Adjacent work surfaced during cross-repo audit + design discussion is captured here so a fresh session can address each item independently.</WCTX> -->
<!-- <CLOG>1.0.0: initial handoff — focused_row_btop rendering bug (P1, gt-design), two design proposals awaiting decision, three Model B follow-on moves queued, sweep-doc remaining tier captured.</CLOG> -->

# Handoff — outstanding items as of 2026-04-26

> **Scope.** This document captures everything that was *surfaced but not closed* during the 2026-04-26 Phase F session. Each item is independent — pick them off in any order. Use this as the queue for the next session(s).
>
> **What shipped this session (no action needed):** Slice 6.6 Phase F.0–F.8 (TransformContext + VfxCellContext bundle across Filter/Mask/Sampler/StyleShader). All five sibling-trait migrations + cleanup landed at commits `51f5204`, `c883683`, `b628abd`, `5535b0e`, `7821815`, `5249550` on tui-vfx. See `docs/design/tui-vfx-buy-once-architecture-sweep.md` §Top 1 for the closing record. Builds clean across all three repos; tui-vfx and tui-vfx-recipes are 100% green.

## Index

| # | Item | Repo | Severity | Estimated effort |
|---|---|---|---|---|
| 1 | `focused_row_btop` recipe doesn't visually distinguish selected row | gt-design | **P1 — real product bug** | half-day to a day |
| 2 | Decide effect-composition model (named stages vs free-form graph) | tui-vfx | architectural decision | one decision meeting + small doc bump |
| 3 | Decide signal-facade placement + scope (`tui_vfx_recipes::signals`) | tui-vfx-recipes | architectural decision | one decision meeting + Slice α/β packets |
| 4 | Three Model B follow-on moves (composite-effect templates, filter-discard bit, resolved-coord fields) | tui-vfx | future Slice work | one Slice each |
| 5 | Buy-once sweep findings 1.2.A / 1.2.B / 1.3.A still queued | tui-vfx | future Slice work | per-finding |

---

## 1. `focused_row_btop` recipe — selected row not visually distinguished (P1)

**Reproducer:**
```bash
cd /usr/projects/gt-design && cargo test -p gtd-ratatui --test test_content_shell \
  test_content_shell_pipeline_override_without_animation_hints_uses_dwelling_effects \
  test_content_shell_stateful_pipeline_override_without_animation_hints_uses_dwelling_effects
```

Both fail with the same assertion:

```
assertion `left != right` failed: selected row should differ from the previous row when the focused-row recipe is active
  left: Rgb(245, 248, 252)
 right: Rgb(245, 248, 252)
```

at `crates/gtd-ratatui/tests/test_content_shell.rs:336` (the `assert_selected_row_differs_from_neighbors` helper). Row 3 (the selected row) renders the same fg as rows 2 and 4.

**The recipe under test:**
- `themes/eichler/recipes/focused_row_btop.json` — the override key the test sets is `pipeline_override_key("focused_row_btop")` (lines 182 and 211 of `test_content_shell.rs`).
- Runtime params: `selected_row = 3_u16` (via `focused_row_runtime_params()` at `test_content_shell.rs:310`).

**Verified pre-existing.** The test was checked against pre-Phase-F tui-vfx (`git checkout 6554947` for tui-vfx, run the gt-design test) — fails identically. Phase F is not the cause.

**Prior context — DF-010.** `df-tickets/DF-010-focused-row-integrated-render-no-op.json` describes this exact pathology and claims resolution. The ticket's fix had two parts:
1. Stop `gtd-ratatui` from synthesizing an entering animation snapshot from `pipeline_override_key` alone (which made `focused_row_btop` resolve with zero shader layers).
2. Realign the vendored `focused_row_btop` recipe with the canonical btop foreground-brightness reference, preserving `selected_row_binding` for live viewport-local state.

The ticket says the resolution landed. The integration test failing today says either (a) the fix regressed, (b) the fix landed for `gitui` but not for the eichler-themed test path, or (c) the test reproduces a different code path that's still broken.

**Suggested first probes (no edits):**

```bash
# 1. Confirm the recipe is loadable and what shaders it declares
cd /usr/projects/gt-design && cat themes/eichler/recipes/focused_row_btop.json | head -80

# 2. Run pipeline-validator probe on the recipe — does the focused-row shader fire?
cd /usr/projects/tui-vfx-recipes && cargo run -q -p pipeline-validator -- \
  --probe --probe-causation --probe-widget-cell 4,3 --phase dwelling --sample-t 1.0 \
  -- /usr/projects/gt-design/themes/eichler/recipes/focused_row_btop.json

# 3. Use recipe-probe to compare expected vs actual cell color at the selected row
cd /usr/projects/tui-vfx-recipes && cargo run -q -p recipe-probe -- \
  /usr/projects/gt-design/themes/eichler/recipes/focused_row_btop.json \
  --phase dwelling --sample-t 1.0 --with-causation --widget-cell 4,3

# 4. Compare to the canonical btop reference if it still exists
ls /usr/projects/tui-vfx-recipes/recipes/btop_focused_row_demo.json 2>&1
# (DF-010 cited this path as the canonical btop foreground-brightness reference)
```

**Likely root-cause hypotheses, in order:**
1. The vendored `focused_row_btop` regressed back to the background-bar variant DF-010 fixed (look at recent commits touching the recipe).
2. The `selected_row` runtime param is reaching the recipe but not reaching the shader (the binding plumbing broke in some subset of the pipeline override path).
3. The shader is firing but writing to background rather than foreground (the test reads `cell_fg` at `test_content_shell.rs:347`).
4. The `entering`-animation-snapshot fix from DF-010 regressed and the recipe is again resolving with zero shader layers in the test's specific code path.

**Bonus — likely the same bug in the demo viewer.** User flagged 2026-04-26 that the focused-row recipe doesn't render properly in the demo viewer either. Same recipe, same code path; fixing the test will likely fix the demo. Worth checking the demo viewer at `examples/binding_lab/` (which has a comment at `cls_lab_state.rs:63` referencing the `focused_row_btop` shader's HLL coordinate read).

---

## 2. Decide effect-composition model (named stages vs free-form graph)

**Doc:** `docs/design/tui-vfx-effect-composition-model.md` v0.1.0 — companion proposal written this session.

**Recommendation in the doc:** Stay with Model B (named stages: sample → mask → shade → filter, with within-stage layered chaining). Reject Model A (free-form graph) for tui-vfx's lifetime — the per-cell traversal tax doesn't pay for itself on a 16ms terminal budget at ~10K cells/frame. Three surgical follow-on moves cover the real boundary-crossing cases without building a graph engine.

**What's needed:** explicit decision (acceptance, modification, or rejection) so item 4 below knows what to schedule. If accepted, the V3 schema redesign should lock Model B's vocabulary explicitly per §7 of the doc.

---

## 3. Decide signal-facade placement + scope (`tui_vfx_recipes::signals`)

**Doc:** `docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md` v0.2.0 — companion proposal written this session.

**Recommendation in the doc:**
- **Placement:** Option A — in-crate module at `tui_vfx_recipes::signals::*` (no new crate; cannot be published independently). If the module ever outgrows reasonable size, promote to a sub-crate `tui-vfx-recipes-signals` (Option B) — mechanical conversion.
- **Scope:** narrow (recipe-deserialization only). Production code keeps importing `mixed_signals::*` directly; the facade does not intercept production paths.
- **Headline maintenance lever (user articulation):** locally-named/scoped interface point to drive recipe inputs; future swaps, plug-ins, exposure-limiting, and rename/remap stay in one place.

**Phasing (from §8.6 of the doc):**
- α: autogen `SIGNALS_REFERENCE.md` from SignalSpec rustdoc + `signals.toml` overlay (smallest; doc-only).
- β: curated "Core 12" cheatsheet (one section of α's autogen output).
- γ: build the `signals` module + collapse the parallel physics channel.
- δ: symmetric `BindableF32` / `BindableColor` family with signal-form variants pointing at `VfxRecipeSignalSpec`.

**What's needed:** explicit decision (yes/no on facade; yes/no on each phase). If accepted, α + β can land as one packet.

---

## 4. Three Model B follow-on moves (gated on item 2 acceptance)

From `docs/design/tui-vfx-effect-composition-model.md` §5. All three address real cases where Model B is awkward today; none require a graph engine.

| Move | Cost | Earns its place via |
|---|---|---|
| **Composite-effect templates** in the recipe schema (V3): `"type": "ripple"` declares a name + parameter schema; recipe-load expands it into the constituent layers (sampler + mask + shader). Like SCSS mixins. | medium; one expansion pass at recipe-load | covers the boundary-crossing ergonomics ("ripple is one effect, not three") without runtime cost |
| **Filter-discard bit** on `VfxCellContext` (or a return-type change on `Filter::apply`) | small | covers "filter wants to also gate visibility" — closes a real gap |
| **Resolved-coord fields** on `VfxCellContext` (`resolved_x`, `resolved_y`) populated after sampler stage | small; one field bump on the type Phase F just introduced | covers "mask wants to react to post-sampler coords" — cheap given the bundle pattern |

Each is a separate Slice. Not blocked by anything except the item 2 decision.

---

## 5. Buy-once sweep findings still queued

From `docs/design/tui-vfx-buy-once-architecture-sweep.md` v1.3.0 §Recommendation Summary table (excluding finished 1.1.A which Phase F closed):

| Finding | Risk | Recommendation |
|---|---|---|
| **1.2.A** Bindable<T> generalization | L | Next slice |
| **1.2.B** Pool<T> generalization | S | **Do now** |
| **1.3.A** VfxImageSource.image_name → BindableString | S | **Do now** |
| 1.1.B Bindable*::evaluate signature unification | M | Wait for third trigger |
| (others — see sweep doc tier list) | various | various |

**Note:** 1.2.A (Bindable<T> generalization) is closely related to item 3 (signal facade) — the symmetric Bindable family in proposal phase δ depends on 1.2.A's shape. If item 3 is accepted, schedule 1.2.A immediately before phase δ.

---

## What's known-good after this session (for orientation)

- **tui-vfx:** master at `ac2289a` ("Resolve build warnings — gate test-only helpers with #[cfg(test)]"). Build clean; zero warnings; 93 test suites OK; all doctests pass.
- **tui-vfx-recipes:** master at `92d57f4` ("Tolerate loopback_fire warnings in Madeira probe tests"). Build clean; 76 test suites OK; zero failures.
- **gt-design:** master at `384e211` ("Update command palette contract test for documented overshoot behavior"). Build clean (incl. all examples); 129 test suites OK; only 2 failures — both item 1 above (same root cause).
- **mixed-signals:** unchanged this session.

## Process notes for the next session

- The `feedback_clean_build_no_warnings.md` rule landed in memory this session: every code-touching agent prompt now includes a "warmup with ofpf-*" block AND a "build must be warning-free at commit time" verification block. Both are now standing project rules.
- The OFPF tooling sweep against gt-design / tui-vfx-recipes during F.6 confirmed neither sibling repo has any Filter/Mask/Sampler impls — the trait surface is fully self-contained inside tui-vfx-compositor. Cross-repo migrations for these three traits are no longer a concern.
- All design proposals written this session live under `docs/design/`. None of them have implementation packets yet — the next session decides which (if any) advance.

<!-- <FILE>docs/design/tui-vfx-2026-04-26-handoff-outstanding.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
