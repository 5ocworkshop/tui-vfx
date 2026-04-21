<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/30_why_now.md</FILE> - <DESC>Chapter 30 — Why now: the clean-break framing that legitimizes breaking changes across the V3 scope, the feedback cross-reference mapping external weak-seams concerns to specific V3 coverage, and the concrete migration drivers that surfaced during recent work.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Extracted from the monolithic plan (v0.16.0) "Feedback cross-reference" and "Why now" sections. Combined here because both address the question "why is V3 the right shape for right now?" — the feedback cross-ref shows what V3 is responding to; the drivers list shows why the response must happen now rather than incrementally.</WCTX> -->
<!-- <CLOG>1.0.0: initial extraction from the monolith. Added an ANSI timeline diagram showing the clean-break vs V4-back-compat discipline.</CLOG> -->

# 30 — Why now

## 10 — Clean-break framing

**V3 is a clean break. Backwards compatibility is not a constraint.**

`tui-vfx-recipes` and the `tui-vfx` family are published to crates.io but have not been promoted and have not been discovered — gt-design is effectively the only consumer. V3 is pre-audience work. That changes the shape of the right decisions:

- **No compatibility shim window is required.** V2 recipes do not need to keep loading under V3. The corpus of shipped recipes is internal; it migrates in one pass and V3 is the new floor.
- **Breaking changes are acceptable everywhere they improve the design.** "Since we're breaking things anyway" is a valid argument for bundling related changes (Ra→Vfx rename, "preview" naming, vocabulary refresh, schema tree restructure). "We'd have to maintain compatibility" is NOT a valid argument against doing the right thing.
- **Named-factory preservation for backwards compat has no weight.** If a named shader earns its place via encoded design judgment, keep it. If it doesn't, drop it. The "existing recipes use this name" consideration carries no weight because there are no external recipes to worry about.
- **Deprecation warnings and dual-path loaders are overhead we don't need to pay.** We can do the rename/rewrite once, update the corpus once, and ship V3 as the new baseline.

This framing explicitly replaces any prior reasoning in this plan that hedged toward preservation. If a decision in this document appears to favor compatibility over the right design, flag it and re-evaluate. V3 is the moment to do it right.

**When V3 ships, if and when external adoption grows, future versioning will need real compatibility discipline.** V4 → V3 migration, if V3 gains actual consumers, gets the care V2 → V3 does not need to carry. This context is time-scoped to V3; it is not a license for future generations to break things freely.

```
   V2 ─────▶ V3 (clean break, pre-audience)
              │
              │   if external adoption grows:
              │
              ▼
              V3 ─────▶ V4  (full compatibility discipline — shim window,
                             deprecation warnings, dual-path loaders,
                             validator-enforced migration paths)

   The clean-break license is scoped to this version bump.
   Future bumps assume real consumers and earn their care.
```

## 20 — Migration drivers

The migration drivers are not speculative — each one surfaced concretely during recent work:

- **Schema accretion is starting to bite.** The ember-felt debugging session exposed that `spatial_shader` is a deprecated-but-still-loaded field that silently compiles to `dwell_effect` only. Authoring AI (and me specifically) tripped on it. SKILLS.md-based mitigation would require documenting every asymmetry in perpetuity.
- **Comparative data supports the uniform pattern.** The `tachyonfx` architecture review showed that a unified scope primitive (`CellFilter`) with algebraic composition, paired with explicit composition containers (`sequence` / `parallel`) and method-chain propagation, is a proven shape at ~30K downloads of production use. We're not pioneering — we're adopting a validated pattern.
- **AI-assisted authoring is the primary composition pathway.** At the scale we plan to ship (large reference library + SKILLS.md + capability matrices + community extensions), AI reasoning reliability matters, and schema shape directly affects it through attention proximity, shape regularity, context-budget consumption, and error-recovery cost. Every mechanism pushes toward tree.
- **Ambient-halo and ember-felt both want this substrate.** Both explorations in `docs/internal/specs/relative-light-architecture.md` would compose more naturally against a tree schema with a unified scope primitive and Pattern-as-axis than against the current flat schema. Landing the schema first unblocks both.
- **Migration cost only grows.** Every recipe we ship in the current schema increases the V3 migration surface area. Deciding now and staging the migration is cheaper than deciding in six months with 3× the corpus.

## 30 — Feedback cross-reference — mapping external concerns to plan coverage

A developer working daily with gt-design produced a weak-seams review (at `feedback/2026-04-21-gtd-tui-vfx-weak-seams-feedback.md`) surfacing seven concrete seams where the current system can still drift. This plan addresses each of them either directly or by naming the explicit work item:

| Weak seam | V3 coverage |
|---|---|
| 1. Duplicated semantic conversion in `gtd-ratatui/src/recipes/{item,planner,player}.rs` — the `config.shadow` miss pattern | **Decision 8** (canonical upstream semantic seam) names this as structural V3 work |
| 2. `RecipeSceneCanvas` identity too generic — overloaded between substrate and family-specific surface | **Open Q #20** (surface identity vs neutral substrate) |
| 3. Labs honest but not fully faithful — mix product demo + teaching + debug roles | Principle 4 (authoring-affordance preservation) provides the governance lens; primarily a gt-design governance issue, not V3-structural |
| 4. Policy distributed across layers — shadow/elevation outcomes are products of many policies | Principle 5 (meaning-low-policy-high) is the organizing principle; Decisions 1, 5, 7 consolidate V3-side pieces |
| 5. Tests encode implementation detail — identity assumptions, trace names, route labels | Open Q #9 (validator redesign) — contract-based testing as a V3 principle for new validator infrastructure |
| 6. Naming doesn't match abstraction — "preview" naming for canonical engine | **Open Q #19** (Preview naming); Decision 4 (Ra→Vfx rename) addresses one instance |
| 7. Intake layer complex — raw/resolved/template-backed/runtime-param-injected pathways | Open Q #14 (tokenization), Open Q #17 (fragment composition), Decision 6 (ParamValue unification) consolidate pieces |
| 8. Key seam conventional rather than structural | **Decision 8** formalizes the upstream canonical-builder seam |

The developer's proposed principle — *"Meaning should live as low as possible. Policy should live as high as necessary."* — is elevated to **Principle 5** in `10_philosophy.md` because it generalizes beyond the seams it was surfaced to address.

## 40 — Summary of the case

```
 ┌──────────────────────────────────────────────────────────────┐
 │  Why V3 now, not later                                       │
 │                                                              │
 │  1. Pre-audience — no shim cost                              │
 │  2. Schema accretion is causing concrete authoring errors    │
 │  3. Tachyonfx validates the target shape at 30K downloads    │
 │  4. AI-authoring is the primary pathway; tree shape helps    │
 │  5. In-flight explorations (ambient-halo, ember-felt) want   │
 │     the V3 substrate to compose cleanly                      │
 │  6. Migration cost compounds with every recipe shipped       │
 │                                                              │
 │  → V3 is the last window where the clean-break               │
 │    license is zero-cost.                                     │
 └──────────────────────────────────────────────────────────────┘
```

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/30_why_now.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
