<!-- <FILE>steering/INTENTIONS.md</FILE> - <DESC>Top-down steering decisions for tui-vfx — the durable framing that outlasts any individual release. Captures engineering discipline, architectural boundaries, naming conventions, and project-level policy. Companion to steering/MARKETING.md: marketing describes what we've built; intentions describe how we decide what to build.</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>Add an explicit SSOT intention for shared infrastructure, especially the recipe loader/semantic seam. Triggered by active V3 migration work where version detection, dispatch, normalization, validation, and compile routing were at risk of being duplicated across tools rather than remaining centralized in tui-vfx-recipes.</WCTX>
<!-- <CLOG>0.3.0: MINOR — add Intention 26 (single source of truth over parallel seams), with the loader architecture called out as the concrete example.
0.2.0: MINOR — add "Writing style" section between the provenance note and the numbered intentions. Covers: no marketing voice, no grandiose framing, no filler, be specific, one idea per sentence. Includes the "why" (developers filter for signal; grandiose framing reads as insecurity; schema regularity applies to prose).</CLOG>
<!-- <CLOG>0.1.0: initial draft. 29 numbered intentions organized into identity / architecture / discipline / philosophy clusters. Top-of-mind intentions called out (1, 3, 9, 20, 23, 24). Cross-references to V3 upgrade plan and MARKETING.md where relevant. Derived from gt-design steering/INTENTIONS.md v0.52.0 with selective adaptation.</CLOG> -->

# Intentions

This file captures top-down decisions that steer implementation of tui-vfx. It is the durable framing that outlasts any individual release or schema version.

**Top-of-mind intentions:** tui-vfx is grid-first and ecosystem-agnostic (see Intention 1), recipe-authoring truth lives here and downstream consumers wrap rather than reinterpret our semantics (Intention 3), `mixed-signals` is the foundation for all signal primitives and is extended upstream rather than duplicated (Intention 9), recipe-authoring ergonomics are a first-class product goal not polish-to-apply-later (Intention 20), consolidation follows the rule of three (Intention 23), and every additive change must earn its place through real value (Intention 24).

**Companion:** `steering/MARKETING.md` answers *how we describe what we've built*; this file answers *how we decide what to build*. The two stay in sync; when they diverge, they must be brought back into agreement.

**Provenance:** this file is distilled from gt-design's `steering/INTENTIONS.md` v0.52.0. Each of its 51 intentions was evaluated against tui-vfx's scope; 29 are carried over here, adapted for tui-vfx's context. Those are hard-earned experiences. Where an intention adapts rather than copies, the adaptation note is inline.

---

## Writing style — applies to all docs

Write like a developer, for developers. No marketing voice.

- **No over-promising.** Don't say "nothing else has this" unless it's true and load-bearing. Claims the reader can disprove in ten minutes cost more trust than they build.
- **No grandiose sentences.** If a sentence could be removed without losing information, remove it. "This is the feature that makes X possible" is almost always cut-able.
- **Be specific.** "Targets a single cell via `Cell(x, y)`" beats "surgical targeting." Name the primitive, show the shape.
- **Skip intensifiers.** "16.7 ms frame budget" not "an extremely tight 16.7 ms frame budget." The number does the work.
- **No filler transitions.** Drop "moreover," "furthermore," "importantly," "it's worth noting." Just the next fact.
- **One idea per sentence.** Long compound sentences hide the point and fatigue the reader.
- **Match the reader.** Internal docs assume readers who know the domain; don't re-explain basics.
- **Be honest about status.** If something is V3-planned, say V3-planned. If we're pre-1.0, say pre-1.0.

Where this applies: `MARKETING.md`, `README.md` files, rustdoc on public items, commit messages, design-doc chapter prose, schema annotations. Short-form replies in code review and issues follow the same rule.

Why: developers filter for signal. Filler makes them work harder to find what matters. Grandiose framing reads as insecurity about the actual capability — the opposite of what we want. Schema regularity applies to prose too; a regular voice is easier for human readers to scan and for AI authors to produce reliably.

What this is *not* saying: don't be dry for its own sake. Clarity and personality coexist. The rule is "cut the filler," not "strip the voice."

---

## 1. Grid-First, Ecosystem-Agnostic

tui-vfx's compositor renders to an abstract cell grid; ratatui is *a* consumer, not *the* consumer. This is an intentional architectural commitment, not accidental optionality.

Rules:

1. No ratatui-specific types leak into compositor or recipe vocabulary. A `tui-vfx-types::Cell` that parallels `ratatui::buffer::Cell` exists by design; the cost of the parallel type is paid by the architecture, not reconstructed in consumer code.
2. Adapters translate at L4 (the ratatui-facing boundary), not at L2/L3 (compositor, recipes).
3. Plausible sibling consumers — movie composer, static renderer, wasm embed, SIXEL/SVG exporter, CI visual-regression via grid diffs — are first-class architectural targets, not hypothetical optionality.

Why: grid-first earns its place through validated secondary uses. The cost of the parallel cell type is real; the payback is adjacent uses a single-terminal-library rules out by construction. If we were ratatui-native, we could not ship a movie-player binary without ratatui's widget/event-loop machinery, could not render to wasm without a terminal-emulator shim, could not produce static image exports cleanly. See `MARKETING.md` and the V3 upgrade plan's architectural framing.

## 2. Public Crate Surface Is the Only Stable Surface

Consumer-facing examples, starter templates, and documentation must use the intended public integration surface across the `tui-vfx-*` crate family. They must not import internal crate internals or undocumented symbols, and we must not teach consumers to do so.

Rules:

1. If consumers need an item not exposed through the public surface, the response is to expand the public surface with intent. Never teach internal-import workarounds.
2. For recipe playback specifically, examples must use `tui-vfx-recipes`'s canonical builder (V3 Decision 8) rather than reaching through `tui-vfx-compositor`'s internals.
3. Internal refactors are safe by default; they cross no boundary consumers depend on. This property is the whole point of the public-surface contract.

Why: public-surface discipline is the contract with every future consumer. When consumers reach through the surface, a crate-internal refactor breaks downstream; when consumers use the intended surface, internal refactoring stays internal. That difference is what makes the library sustainable to maintain.

## 3. Recipe-Authoring Truth Lives in tui-vfx; Downstream Consumers Own Their Display Truth

`tui-vfx-recipes` owns recipe schema, validator infrastructure, probe/trace tooling, and the canonical recipe → playback-item builder (V3 Decision 8). Downstream consumers — gt-design today, future consumers tomorrow — wrap the canonical output with their own policy. They do not reinterpret upstream semantics.

Why: the V2 era produced a weak seam where gt-design's `item_from_recipe_config()` mirrored upstream's `preview_from_recipe_config()`, drifting silently on fields like `config.shadow`. V3 Decision 8 formalizes the canonical seam so this class of drift becomes a visible deviation rather than an unremarkable default. Our responsibility is to ship and maintain a clean semantic seam; their responsibility is to wrap it with policy (theme resolution, surface identity, render-truth routing). Principle 5 of the V3 upgrade plan applies: meaning lives as low as possible, policy as high as necessary; recipe semantics are meaning, surface identity is policy.

## 4. Engine Internals Live Behind Public Crate Contracts

The compositor, shadow renderer, style pipeline, probe/trace infrastructure, and any other implementation engines live behind their crate's public contract. Consumers depend on contracts (recipe JSON, canonical builder output, trace event schema), not on internal engine types.

Why: if internals become load-bearing for consumers, they're not internals anymore — they're undocumented public surface, and refactoring them cascades breakage. Preserving the distinction keeps the library free to rework compositor internals, swap shadow algorithms, or change trace taxonomies without downstream fallout. The V3 canonical semantic seam is the enforcement mechanism.

## 5. `tui-vfx-recipes` Loader Scope: Parse → Substitute → Resolve → Build

The `tui-vfx-recipes` loader owns JSON parsing, load-time substitution resolution (`Substitutions` per V3 Concern D), schema validation, and canonical playback-item construction. Its contract is a fully-populated, validated playback item that downstream consumers can render without additional parsing or validation logic.

Why: loader/runtime separation keeps the runtime simple. Every validation error that can be caught at load time is caught at load time — strict mode is the default for `Substitutions`; schema shape, scope coherence, binding-contract discovery, and `HintRef<T>` producer resolution are all load-time concerns. Runtime is responsible for rendering a pre-validated item and resolving per-frame `RuntimeBindings`, not for re-validating what the loader already proved.

## 6. Authoring Tools Live in Sibling Crates, Separate from Runtime

Recipe authoring tools, asset generators (rocketsplash and its ecosystem), documentation generators, and any offline tooling live in sibling crates or `xtask`, not in the runtime crates. Runtime crates do not depend on authoring toolchains.

Why: the runtime's dependency closure is a footprint concern for consumers — wasm embeds, static exporters, movie players, CI integrations all care about binary size and compile time. Authoring tools pull in large dependency trees (image codecs, JSON-schema generators, formatting libraries) that don't belong in a consumer's binary. Keeping them separate is how we honor the grid-first-ecosystem-agnostic commitment in practice.

## 7. `tui-vfx-*` Crate Family Naming

The workspace umbrella is `tui-vfx`. Published subcrates use the `tui-vfx-*` prefix for consistent family identity: `tui-vfx-recipes`, `tui-vfx-compositor`, `tui-vfx-shadow`, `tui-vfx-style`, `tui-vfx-types`, `tui-vfx-trace`, `tui-vfx-probe`, `tui-vfx-content`, `tui-vfx-geometry`, `tui-vfx-core`, `tui-vfx-core-macros`, `tui-vfx-debug`.

Why: the family name is both discoverability (`rg tui-vfx` surfaces the ecosystem) and positioning (consumers recognize one library rather than a scattered set of packages). It's also how the dependency graph legibly declares "I'm part of the tui-vfx family" to downstream.

## 8. `Vfx*` Prefix on Wire-Format Types: Three-Test Criterion

A public type in any `tui-vfx-*` crate carries the `Vfx` prefix if and only if it passes at least one of these tests:

- **Wire-format data** — the type flows as data across crate boundaries inside recipe JSON, the playback item, or the render-pipeline contract
- **Errors returned from public APIs** — by Rust convention (mirroring `std::io::Error`, `serde_json::Error`)
- **Contract-producing traits** — traits consumers implement to produce wire-format data

Everything else stays unprefixed. Internal engine types, helpers, builders, and widget-facing APIs don't carry the prefix because their import path already disambiguates them.

Why: `rg Vfx` becomes the grep-anchor for the wire-format surface — the set of lines that matter during migration, debugging, or API review. The prefix is a feature, not noise; stripping it from the wire format loses a load-bearing disambiguation property with no gain. V3 Decision 4 renames the legacy `Ra*` prefix to `Vfx*` in one bundled event; this intention is the durable rule that prevents future prefix drift.

## 9. `mixed-signals` Is the Foundation for Signal Primitives

`mixed-signals` owns the signal-primitive domain — sines, triangles, keyframes, ADSR envelopes, damped springs, spatial noise, and any composition of the above. When tui-vfx needs a capability missing from mixed-signals, the correct response is to extend mixed-signals upstream, not to build a parallel signal surface inside tui-vfx.

Why: signal-primitive duplication is the failure mode. V3 consumes signals; V3 does not invent signals. The flag-animation PRD's `SpatialSignalSpec` is the canonical example — upstream extension (Path B: `Signal2d` trait in mixed-signals) is primary; local implementation (Path A: `SpatialSignalSpec` in tui-vfx-compositor) is fallback only if upstream velocity genuinely blocks V3 delivery. The same rule applies for any future signal capability tui-vfx needs.

Additional boundary rule:

- if a new capability is fundamentally **signal/math substrate**
- and it is applicable to **three or more use cases**
- and it is not inherently tied to tui-vfx rendering semantics

then it should be added to `mixed-signals`, not re-rolled locally in tui-vfx.

Examples that likely belong upstream:

- spatial coordinate leaves
- centered/radial/angle field primitives
- other reusable authored signal-graph math

Examples that should stay in tui-vfx:

- shaders
- masks
- filters
- samplers
- terminal/glyph rendering behaviors

Why: this is the practical rule that keeps crate boundaries clean. `mixed-signals`
is the shared primitive generator; `tui-vfx` is the rendering/effect system that
consumes those primitives.

## 10. Clean-Sheet Naming and Ergonomics Reset at Version Boundaries

Major-version boundaries are deliberate moments to clean up naming that evolved under rapid scope changes. When legacy names conflict with clarity or ergonomics, we prefer the clearer clean-sheet name and provide migration notes rather than preserving confusing historical naming by inertia.

Why: incremental renames accumulate costs — each one is a breaking change, each one pays the migration tax for just its own rename. Bundled renames at version boundaries pay the breaking-change cost once and let the whole naming surface move forward together. V3 is this moment for tui-vfx — the `Ra*` → `Vfx*` prefix rename (Decision 4), the "preview" seam naming (Open Q #19), the vocabulary refresh (Open Q #15) all ride in one cutover.

## 11. Hybrid Documentation: Generated Facts + Curated Guidance

API and schema docs follow a hybrid model. Technical facts (public items, types, fields, serde shape, rustdoc) are extracted from Rust source. Human-curated editorial content (examples, context, policy, migration guides) is provided in dedicated documentation config files. Generated output files are treated as build artifacts, not hand-edited.

Every change to public API or schema must include corresponding documentation inputs: rustdoc updates on affected public items, curated editorial updates where context changes, and generator validation passing for drift and coverage.

Why: extraction catches drift mechanically — if a field changes shape, the generator notices. Curation carries judgment that extraction cannot — why a field exists, when to use it, what failure modes to watch for. Both layers are required; neither alone is sufficient.

## 12. Documentation Is a First-Class Automated Engineering Contract

Rustdoc coverage on public surfaces is mandatory, not optional. Public types, functions, traits, fields, enums/variants, and behavior-critical contracts must have meaningful rustdoc entries explaining intent, constraints, and usage. Generated API and schema docs are release-gating artifacts; drift and coverage checks run in CI and through `just` workflows. Code changes that affect behavior or contracts are incomplete unless accompanying rustdoc and documentation-generation inputs are updated and validation passes.

Why: undocumented public surface is a perpetual tax on every new consumer who has to guess at intent, constraints, and failure modes. The generator plus drift-check infrastructure pays once; it catches undocumented additions on every subsequent build at zero marginal cost. This is the canonical example of Intention 25 applied to documentation.

## 12A. V3 Schema-Bearing Rust Types Must Meet V2-Grade Generation Standards

For work in both `tui-vfx` and `tui-vfx-recipes`, V3 schema-bearing Rust types must be maintained to the same standard V2 established for schema/doc generation.

This is a hard requirement.

Rules:

1. Any Rust type that defines or materially shapes the V3 authoring/runtime schema surface must carry enough code-side metadata to participate directly in:
   - schema generation
   - API doc generation
   - capability/reference doc generation
   - validation/drift tooling
2. The default expectation is:
   - meaningful rustdoc on public schema-bearing items
   - `ConfigSchema` derivation, or an explicit schema implementation when derive is insufficient
   - serde shape correctness
   - field-level metadata where the generators and validators depend on it
3. Do not leave V3 schema-bearing types as thin undocumented DTOs unless the DTO is intentionally private and not part of the schema-defining surface.
4. Reuse and adapt the existing generation/validation tools in `tui-vfx` and `tui-vfx-recipes` wherever possible rather than inventing parallel checks. If the current tools don't cover a new V3 type cleanly, extend the toolchain so the code remains the source of truth.

Why: the project already auto-generates substantial documentation from code, not just raw schema output. If V3 schema-bearing types fall below the V2 standard, generated artifacts drift, validation weakens, and the code stops being the reliable source of truth across both repositories.

## 13. `justfile` / `xtask` Is the Workflow Entry Point

Project workflow tasks — build, test, check, doc generation, validation, gates — are launched through `just` recipes or `xtask` subcommands as the central command surface for local development and CI. When a recurring task appears, we add a named recipe rather than relying on ad-hoc shell knowledge. Documentation generation and drift detection must be runnable via named `just`/`xtask` targets.

Why: workflow discoverability is contributor experience. `just --list` or `cargo xtask --help` surfaces everything the project can do in one place. Shell-knowledge fragmentation produces "how does this project work?" confusion that silences new contributors.

## 14. Engineering Workflow Is Test-First and Audit-Gated

Development follows TDD: write tests first, observe failure (red), implement minimal code to pass (green), then refactor safely while preserving passing tests. Mocks are used sparingly; prefer real integration paths and realistic scenarios whenever practical, so behavior validates against actual contracts.

At phase end, run formatting and lint gates (`rustfmt`, `clippy` with warnings-as-errors), and fix issues close to source rather than deferring cleanup. Each phase ends with an explicit audit against the phase plan and codebase, with checks for completeness, gaps, regressions, performance risks, security issues, and architecture-boundary violations.

Why: red-green-refactor produces code testable by construction. Audit-gating catches what TDD's local focus misses — cross-module interactions, architectural drift, forgotten edge cases, performance regressions.

## 15. Audit Pass Is a Hard Phase Gate

A phase is not complete when development work is "done"; it is complete only when audit passes. If an audit returns findings, fixes must be applied and then the full audit must be rerun end-to-end. Partial or informal spot-checks are not a substitute. Teams move to the next phase only after an explicit auditor pass on the rerun.

Why: "done" is a dangerous word. A passing audit is an evidence-based claim; "feels done" is wishful thinking that compounds into technical debt across every subsequent phase.

## 16. Official Validation Scope Is Linux-Only

Required CI, test, and quality gates target Linux as the official release-blocking validation platform. Non-Linux platforms may be community-validated and best-effort; they are not release-blocking validation targets until explicitly promoted to first-class scope. This scopes what the maintainer tests, not what consumers can run.

Why: validation scope is a maintainer-capacity concern. Linux is the realistic scope for this team; expanding validation without expanding the team produces false-confidence gates that miss failures on unvalidated platforms. Consumers are welcome to test their own platforms; they should not expect the maintainer to block releases on platforms the maintainer doesn't validate.

## 17. Toolchain Policy Is Forward-Only

The project targets modern Rust on purpose. MSRV is anchored to the latest stable toolchain baseline selected for active development and advances intentionally rather than being held back for legacy environments. The workspace uses the latest stable edition (current: 2024), unless an explicit steering decision updates that baseline.

Why: legacy-toolchain support is a tax paid by every contributor every day, for a capability most consumers don't need. Forward-only policy lets us use modern language features (let-else, async closures, `matches!` macros, etc.) that produce clearer code.

## 18. Ratatui Baseline Is `0.30.0+`

tui-vfx's ratatui adapter layer — the L4 boundary where the grid renders into ratatui's buffer — aligns with Ratatui `0.30.0+` APIs. Compatibility work prioritizes current ratatui idioms and ergonomics over preserving older adapter patterns unless an explicit steering decision requires otherwise.

Why: tui-vfx is grid-first, but the ratatui adapter is where real consumers meet the library today. Tracking current ratatui avoids adapter-layer debt and keeps the integration story simple for consumers upgrading ratatui. See Intention 1 for the architectural framing — ratatui is *a* consumer; that consumer stays current.

## 19. No Hardcoded Theme Values in Canonical Examples

Canonical examples and reference code must derive themeable values — colors, motion timing, depth, anchor offsets — from resolved theme data via the consumer's design system (e.g., gt-design's `GtdResolvedDesign`), not from hardcoded terminal colors or magic numbers. Examples that bypass theme resolution teach patterns that don't survive theme switching.

Why: canonical examples are teaching artifacts. An example that hardcodes `Color::Rgb(64, 128, 255)` teaches future authors to hardcode; an example that pulls from theme-resolved data teaches the intended integration pattern. Since recipes can target cells by `Scope::ThemeRole(...)` (V3 Decision 1), the integration path is already first-class — examples should exercise it, not route around it.

## 20. Recipe-Authoring Ergonomics Are a First-Class Product Goal

Recipe authoring is the primary composition pathway tui-vfx is designed for (see `MARKETING.md`). Authoring ergonomics — schema regularity, bounded vocabularies, explicit contracts, inline-readable tree shape, validator feedback quality, contract-discovery APIs, authoring guides — are first-class framework concerns, not polish to apply later.

Rules:

1. **Humans and AI are co-equal primary authors.** Recipe schema and tooling must be easy for humans to read, edit, and review *and* easy for AI to generate correctly at scale. The two audiences aren't in tension when the schema is well-designed — clarity for one typically serves clarity for the other. When a design choice benefits one and costs the other, that's a signal to revisit rather than accept.
2. Recipe schema decisions are evaluated against authoring ergonomics explicitly, not implicitly. "Is this easy to author?" is a shipping criterion for both audiences, not a stretch goal.
3. When recurring friction appears in authoring workflows — human or AI — the default response is to improve the shared authoring surface (schema, validator, briefing infrastructure, SKILLS.md, recipe library), not document the friction as a known issue.
4. Schema regularity, proximity-weighted shape design, bounded scope vocabularies, and explicit contracts are deliberate choices that serve both audiences — comprehensibility and learnability for humans, reliable generation for AI. They are not AI-specific accommodations, and they are not human-specific either.

Why: authoring-ergonomics debt compounds in every recipe written. At library scale (500+ recipes plus third-party extensions), schema that's hard to author correctly produces an unboundedly large correctness problem regardless of who's authoring. The two audiences share more concerns than they differ on — bounded vocabularies are easier to learn *and* easier to generate; explicit contracts are easier to reason about *and* easier to validate; inline-readable tree shape is easier to review *and* easier to produce. When those design choices are honored, a well-designed schema is a well-designed schema for everyone. V3's redesign exists specifically to reduce this debt; this intention makes the reduction a durable concern rather than a one-time push.

## 21. No Hardcoded Effect Parameters in Consumer Code

Every effect parameter that a recipe can express — color, intensity, duration, timing, motion path, blend mode, signal graph — must come from the recipe JSON (via `Substitutions` at load or `RuntimeBindings` per frame), not from hardcoded values in consumer code. If a consumer needs an effect parameter that can't be expressed in the current recipe schema, that's a recipe-schema gap to close, not a license to bypass recipes.

Why: hardcoded parameters in consumer code are the exact failure mode recipes are designed to prevent. An effect baked into Rust code at the consumer is not themeable, not substitutable, not runtime-bindable, not previewable, not audit-visible, not AI-authorable. The value of recipes is that they make all of these things possible; bypassing recipes throws the value away and simultaneously reintroduces the weak-seam failure class Intention 3 exists to prevent.

## 22. Batteries-Included Behavior Must Be Resolved in the Library Path

If tui-vfx presents a behavior as part of its product contract — canonical playback, probe fingerprinting, trace observability, signal resolution, shadow composition, binding-contract discovery — that behavior must be owned by library crates on the public path, not reconstructed in each application or example.

Rules:

1. When examples or apps need ad-hoc compensating logic to reach canonical tui-vfx behavior, that's evidence of a library contract gap and must be treated as a defect to fix in the library.
2. Canonical examples, starter templates, and demos must exercise the intended batteries-included path. They may demonstrate explicit overrides where a real app deliberately wants them, but they must not be the place where baseline tui-vfx behavior is assembled by hand.
3. Public APIs preserve escape hatches for intentional overrides, but the default path must already deliver the behavior the product claims to support.

Why: batteries-included is a commitment that outlives any individual example. Shipping compensating logic in examples teaches contributors that the library doesn't deliver on its own promise; it erodes trust and guarantees duplicate workarounds in every consumer. The correct response to a batteries-included gap is to fix the library, not to paper over it at the call site.

## 23. Top-Down Consolidation Discipline (Rule of Three)

tui-vfx has a growing surface area — named shader factories, Pattern variants, hint-namespace kinds, fragment definitions, scope variants, and more. Feature-by-feature organic growth produces quiet fragmentation. The consolidation discipline below is the active counter-force.

Rules:

1. **The rule of three.** When three or more units (shader factories, Pattern variants, hint kinds, fragment definitions) implement behavior that is conceptually the same, extract a shared primitive into the appropriate crate before the next unit adds a fourth parallel implementation.
2. **Top-down periodic review.** At major milestones, survey the surface top-down — walk the full catalog, group by category, look for behavioral overlap. Do not wait for pain to become acute; the longer fragmentation sits, the more expensive consolidation becomes.
3. **Shared vocabulary over convenient nicknames.** When a new type is needed, first ask "does this match an existing primitive?" Canonical types are the first choice; per-case aliases are acceptable only when they compose over a shared primitive.
4. **Additive migration, never breaking churn.** Consolidation adds shared primitives; it does not delete or rename existing types in the same change. Per-case types that duplicate a shared primitive get marked "internal implementation detail" and are gradually retargeted over releases.
5. **Abstractions earn their place.** Only abstract when the generalization is genuine — three cases is the threshold, not two. Forced or speculative abstraction is worse than honest duplication.
6. **Document the consolidation rationale.** Every shared primitive extraction includes a rustdoc comment explaining *why* it exists and *which* cases motivated the extraction. Future contributors need to see the pattern the primitive captures, or they will re-fragment it.

Why: organic feature growth produces quiet fragmentation that compounds into cognitive load on consumers and maintenance cost on the team. A healthy tui-vfx is one where new shader variants, Pattern variants, or fragment definitions feel natural to compose because the primitives they build from already exist. The V3 Pattern-as-separable-axis model (Decision 2) is this intention applied to shader composition; the V3 `$use` fragment library (Open Q #17) is the planned application to recipe composition.

Related: Intention 24 establishes when *not* to abstract — one case is fine without a new primitive. Both rules must pass for any additive change.

## 24. Library Changes Must Earn Their Place Through Real Value

Every additive change — new helper, new macro, new type, new method, new module — must earn its place by passing a deliberate step-back review before it lands. tui-vfx is easy to add to and hard to subtract from once shipped. The cost of a helper that does not pull its weight is not zero: it becomes public surface area every future contributor must read, document, and preserve.

Rules:

1. **Real value, not abstract principle.** A change must solve a problem visible in current code at current scale. "This would be useful if we had ten of these" is not a reason to add when we have three. The threshold is a concrete pain point a reader can point at in the repository today.
2. **Move toward ecosystem norms.** When the idiomatic ratatui, Rust, or serde answer already covers the use case, use it. A macro when a direct method call is clearer is a step backward. Novel solutions need to clear a higher bar than idiomatic ones.
3. **Reduce lines and reduce complexity at the call site.** Measure before/after at the canonical call site. If "after" has more lines, more tokens, or more nesting than "before" for the common case, the change has not earned its place yet.
4. **Improve readability, measured at the reader side.** The test is not "is it more concise for the writer" but "is it easier for a cold reader to understand what is happening." Macros, trait objects, and generic bounds move complexity from writer to reader; they must produce a concrete readability win large enough to justify the shift.
5. **The step-back test is mandatory.** After designing a helper, before committing: stop and read the before/after side by side. Ask "would I rather maintain the after for three years, or the before?" If the answer is "the before," the helper has not earned its place.
6. **Watch for the rationalization chain.** When the argument for a new helper requires more than two sentences, that length is a signal. Good helpers justify themselves in one sentence. When the justification keeps growing — "this *also* does X", "this *also* prevents Y" — the helper is accumulating rationalizations rather than earning them.
7. **When a change fails the filter, back out cleanly and record the lesson.** Reverts are not failures; unexamined accretion is the failure. A clean revert with a commit message explaining what filter the change did not pass is strictly better than a helper that lingers because reverting "felt like backtracking."
8. **This filter does not apply to real features.** Net-new features that solve real product problems (new shader primitives, new scope variants, validator capabilities) pass a different gate — MARKETING alignment, scoped correctly, tested. This filter specifically targets the seductive "let me add a little helper" category where cost is invisible until shipped.

Why: the failure mode is a chain of plausible-sounding rationalizations ("this batches the call", "this reduces duplication", "this is more discoverable") that individually seem reasonable but together add cognitive load without reducing call-site work. The step-back test catches the failure mode before shipping.

Related: Intention 23 establishes when to *extract* shared primitives; Intention 24 establishes when *not* to. Both rules must pass for any additive change.

## 25. Actively Hunt for Infrastructure Wins That Automate Validation

Throughout every phase of work, actively look for opportunities to turn a manual audit or one-time correction into automated validation infrastructure. When an audit finds a drift class that required human investigation to detect, ask immediately: "could a mechanical check have caught this at introduction?" If yes, build the check in the same session when scope allows.

Rules:

1. **Look for infrastructure wins during every substantive change.** After completing a manual correction, pause and ask what mechanical check would have caught it. Building the check now while context is fresh is high-ROI; waiting loses the insight.
2. **Prefer mechanization of drift classes already seen.** Building a check that catches a drift you just manually fixed always pays off — by construction, the drift has already happened. Generic "just in case" infrastructure is speculative and frequently expires before paying off.
3. **Step back top-down before picking the integration point.** Ask what layer the check should live at — shell script, Rust unit test, integration test, build script, generator tool, CI workflow, `just` recipe. Pick what serves the whole project, not just the file you're editing. Ad-hoc integration fragments the gate system.
4. **Consider positive and negative effects before adding.** A check that catches one drift class cheaply but breaks three other workflows is net negative. Describe scope, cost, and failure mode in one paragraph before writing code; if the paragraph runs long, the check is probably wrong or wrongly placed.
5. **Prefer the smallest intervention that delivers full coverage.** A 15-line bash assertion comparing two counts is often strictly better than a 200-line Rust tool — easier to read, debug, modify, and approve. Reach for heavier tooling only when the simpler assertion genuinely cannot express the invariant.
6. **Integrate with existing gates rather than inventing new ones.** New entry points fragment the mental model of "how do I run the checks" and multiply places where drift can hide.
7. **Document the resolution playbook inside the check.** When a drift check fails, the contributor hit by it should be able to read the error message or surrounding comment and know exactly what to do. A check that fails without explaining how to fix is a trap.

Why: validation infrastructure is the single highest-leverage category of work in any long-lived project. Every check gets reused at zero marginal cost on every subsequent build. Manual audits cost time every time; mechanical checks cost time once. This intention is the explicit counter-force against "defer automation to later" — later never arrives.

Related: Intention 24 gates *ergonomic helpers* that do not earn their place. Intention 25 actively pursues *validation infrastructure* that prevents classes of failure. Both rules must pass for any additive change; the overlap is zero because they target disjoint categories.

## 26. Prefer a Single Source of Truth Over Parallel Seams

When multiple consumers need the same semantic operation, the default answer is a single upstream implementation, not multiple parallel ones.

Rules:

1. **Shared semantics live once.** Version detection, loader dispatch, normalization, validation, compile-to-execution-plan, and similar semantic seams should have one canonical upstream implementation. Tools and downstream consumers call that seam; they do not rebuild it locally.
2. **Do not create parallel loaders.** If two binaries need to load recipes, the answer is not "write another loader for the second binary." The answer is to improve the canonical loader surface until both binaries can use it.
3. **Abstract at the seam, not at the call sites.** Repeated branching in tools ("if V2 do X, if V3 do Y") is a signal that the upstream seam is incomplete. Fix the seam rather than proliferating policy in consumers.
4. **SSOT beats convenience duplication.** A local shortcut that duplicates semantic logic is only cheaper today. It becomes long-term drift, conflicting fixes, and silent behavior mismatches.
5. **Preserve escape hatches without forking meaning.** Specialized consumers may still have policy-specific wrappers, but those wrappers compose over the canonical seam rather than replacing it.

Why: duplicated semantic seams are how drift becomes normal. The V2→V3 migration work made this concrete: schema-version detection, version-aware dispatch, load/normalize/validate/compile routing, and canonical playback semantics all want to spread into tools unless they are explicitly centralized. SSOT is the durable counter-force. The loader architecture is the clearest example — there should be one canonical loader family in `tui-vfx-recipes`, not multiple parallel loaders across binaries.

## 26. Consolidation Must Preserve Individual-Item Addressability

When consolidating N independent units (recipe files, primitive definitions, fragment definitions, asset entries) into fewer containers via `template + variants`, `$use` fragment libraries, aggregator manifests, or future bundler primitives, the consolidation must preserve individual-item addressability for debug, preview, reference, and introspection use cases. The file path and unit identity are UX contracts with tooling, not just storage conventions.

Rules:

1. **Identify all consumers of individual-unit addressability before consolidating.** Debug players, file-pickers, documentation, search tooling, validators, probes, traces, external references. If any depend on per-unit addressability, that dependency must be addressed before consolidation lands.
2. **Debug, preview, and reference recipes stay individual by default.** Their one-file-one-preview contract *is* their utility. Consolidation is for production recipe sets where individual items are implementation details.
3. **Consolidation mechanisms that collapse addressable units must ship with tooling updates.** A canonical variant-URI scheme (e.g., `easing_family.json#back_out`) and validator + probe + trace support for that scheme are required. No consolidation lands until tooling supports its expanded form.
4. **Metadata declares consumption mode.** Consolidated files declare intended consumption (`programmatic` / `individual_preview` / `both`). Tooling respects the declaration.
5. **When in doubt, don't consolidate.** Duplication is cheaper than broken addressability. Abstract only when the generalization is proven and the tooling cost has been paid.
6. **Retrospective corrections are valid and encouraged.** If a past consolidation broke addressability, fixing it by re-expanding or extending tooling is the right action — not living with the regression because "it was already shipped."

Why: consolidation for file-count reduction serves programmatic consumers at the expense of human-facing and tool-facing consumers. Both matter. The `recipes/easing/easing_family.json` retrospective (26 individual files collapsed to 1 `template + variants` file, regressing the demo file-picker) is the canonical example of what this intention exists to prevent. This intention is Principle 4 of the V3 upgrade plan; it's codified here as a durable cross-version rule.

## 27. Byte-Source Loading at All Recipe Boundaries

All recipe loaders, fragment resolvers, substitution APIs, and asset loaders accept byte-source abstractions — they do not assume filesystem access. Concrete implementations may read from the filesystem, embedded resources, zip archives, HTTP responses, or any `impl Read` source.

Why: consumer apps ship in varied environments — embedded resources in binaries for distribution, wasm with no filesystem, mobile with sandboxed storage, CI with only in-memory fixtures. Filesystem-assuming loaders foreclose these cases. This intention preserves the option the V3 upgrade plan's distribution-and-packaging deferred-design section names; it does not require us to solve the packaging design today, only to not preclude it. It's a cheap discipline that costs nothing now and preserves every future option.

## 28. Validator-Friendly Type Patterns at Boundaries

Public types that cross crate boundaries — wire-format types, recipe schema types, playback items — use serde patterns that support trial deserialization: `#[serde(deny_unknown_fields)]` on tagged unions and strictly-typed structs; explicit discriminators; documented shape per variant. When a downstream consumer (gt-design's SSOT, or any future consumer) needs to validate our types at their own load time without duplicating our type definitions, our types must be pattern-compatible with that use.

Why: strict-typed validation at the boundary is how consumers protect themselves from upstream drift. If our types are loose, their validation can't be tight. The pattern is low-cost on our side (serde attributes) and high-value for consumers (load-time validation instead of render-time surprises). gt-design's Intention 38 formalizes this from the consumer side — trial-deserialize our types against their typed definitions at SSOT load time. Our obligation is to make that possible by never shipping types that can't be cleanly trial-deserialized.

## 29. The MARKETING / INTENTIONS Split

`steering/MARKETING.md` answers *how we describe what we've built* — positioning, feature hierarchy, differentiators, talk-track. `steering/INTENTIONS.md` (this file) answers *how we decide what to build* — engineering discipline, architectural boundaries, naming conventions, project policy. The two documents are peers and stay in sync.

Rules:

1. When a new strategic decision shifts positioning (new primary audience, new headline feature, architecture pivot), update MARKETING.md's 90-second description first, then re-derive the 60s and 30s, then refresh callouts.
2. When a new steering decision changes engineering discipline or architectural boundary, add or update an intention here; then check whether MARKETING.md's positioning needs a matching update.
3. When the two documents diverge on fact or framing, neither is authoritative over the other — both must be brought back into agreement, with the code state as the tiebreaker.

Why: a project that describes itself inconsistently loses the trust of both contributors and consumers. Keeping MARKETING and INTENTIONS as peers — one outward-facing, one inward-facing — acknowledges that both audiences matter and prevents either document from silently drifting.

---

<!-- <FILE>steering/INTENTIONS.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
