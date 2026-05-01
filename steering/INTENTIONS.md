<!-- <FILE>steering/INTENTIONS.md</FILE> - <DESC>Top-down steering decisions for tui-vfx — the durable framing that outlasts any individual release. Captures engineering discipline, architectural boundaries, naming conventions, and project-level policy. Companion to steering/MARKETING.md: marketing describes what we've built; intentions describe how we decide what to build. Organized in two parts: Part I durable principles, Part II project-specific rules; numbering stable across the split for cross-reference compatibility.</DESC> -->
<!-- <VERS>VERSION: 0.10.0</VERS> -->
<!-- <WCTX>Add pure v3.1 tui-vfx-compost and subagent file-scope discipline as durable project intentions.</WCTX> -->
<!-- <CLOG>0.10.0: MINOR — retarget Intention 45 from the abandoned copied-crate path to active tui-vfx-compost work and flatten the file changelog.</CLOG> -->

# Intentions

This file captures top-down decisions that steer implementation of tui-vfx. It is the durable framing that outlasts any individual release or schema version.

**Top-of-mind intentions:** tui-vfx is grid-first and ecosystem-agnostic (see Intention 1), recipe-authoring truth lives here and downstream consumers wrap rather than reinterpret our semantics (Intention 3), `mixed-signals` is the foundation for all signal primitives and is extended upstream rather than duplicated (Intention 9), recipe-authoring ergonomics are a first-class product goal not polish-to-apply-later (Intention 20), consolidation follows the rule of three (Intention 23), every additive change must earn its place through real value (Intention 24), versioned shader/filter/mask/sampler/style/effect work carries the full pipeline-touch definition of done with exact version labels (Intention 34), onboarding starts from the architecture-first identity rather than an effects-only mental model (Intention 35), we fix root causes rather than leaving landmines — no per-site `#[allow]`, no algorithmic divergence on upstream extractions, no half-finished consolidations (Intention 40), cross-repo audits for large-scale changes scope all four repos: tui-vfx, tui-vfx-recipes, mixed-signals, gt-design (Intention 41), the `ofpf-*` semantic suite is the default interface for any codebase question — read `steering/OFPF-TOOLS.md` for the practical reference (Intention 42), recipe-JSON signal authoring goes through the `VfxRecipeSignalSpec` facade while engine direct-API consumers use `mixed_signals::*` directly — the two surfaces are intentional and meet at `SignalOrFloat`-typed engine fields (Intention 44), and tui-vfx-compost v3.1 work is pure v3.1 end to end with no lowering/adaptation layers and mandatory touched-file/file-tree packet discipline (Intention 45).

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

### Intention shape

Intentions follow the writing-style rules above and one additional content requirement: every intention articulates *why*. Without an articulated why, what you have is not an intention — it's an arbitrary rule wearing the wrong label. The name carries the weight: an intention is by definition a reasoned commitment.

**Required content:**

- **Headline statement** — one-liner, declarative or imperative.
- **Body** — convey the issue, the explanation, and the why. The why is mandatory.
- **(Optional) Project-specific example** — a real incident or pattern that triggered the rule. Examples that name a concrete situation make the rule stick where prose alone won't.
- **(Optional) Bulleted rule list** — for discipline-style intentions where multiple sub-rules apply (see Intentions 23, 24, 25, 34, 40 for the established shape).

**Length is judgment, not a gate.** Use what you need to fully convey the issue, reason, explanation, and example. No more. Some intentions are one paragraph; some need rules and a worked example; both are right when proportional to the topic. The test is whether trimming would lose load-bearing meaning. If trimming wouldn't lose meaning, the prose was padding; if it would, the length is honest.

**Shape is guideline; content is requirement.** Sentence counts, item caps, and structural templates are guidelines that authors apply with judgment. The presence of a *why* — articulating the reason for the rule — is the gate.

Why: rules without context are brittle. They get followed by string-match when they apply unambiguously and ignored or misapplied when they don't. Context-attached rules generalize — the why lets readers (human or AI) extend the rule to novel situations, weigh conflicts between rules, and detect when applying the rule literally would violate its spirit. This is true for human readers and especially load-bearing for AI agents, which can only act on what they understand rather than what they remember. The "why" is what makes a rule transferable, not just memorizable. Length caps as gates would convert judgment into bureaucracy and is exactly the failure mode the project's broader writing style avoids.

## Commit message provenance policy

Do not add co-author credits to commit messages. This includes `Co-authored-by:`
trailers, tool-generated agent credit lines, model/tool signature footers, and any
other commit-message text whose purpose is to credit an AI assistant or secondary
author. Commit messages should record intent, constraints, decisions, rejected
alternatives, and verification evidence — not authorship attribution beyond the
Git author/committer metadata already present.

Why: commit history is a decision log for maintainers. Extra credit trailers add
noise, can violate project attribution policy, and make future archaeology harder
without improving the technical record.

---

## How this file is organized

Intentions are split into two parts. Numbering is stable across the split — existing cross-references resolve to the same content.

- **Part I — Durable principles.** Posture and engineering discipline that would survive a full rewrite of the codebase. These are identity-level commitments: how we approach documentation, consolidation, validation, audits, library design, and architectural strategy. They evolve slowly. Examples: rule-of-three, library-changes-must-earn-their-place, no-landmines, plan-from-codebase.
- **Part II — Project-specific rules.** Operational rules tied to specific tui-vfx surfaces — crate boundaries, naming prefixes, schema versions, debug recipes, the binding system, the `ofpf-*` tooling, V3 vocabulary, ratatui baseline, and so on. These move with the codebase and may be retired when the surface they govern changes. Examples: `Vfx*` prefix policy, ratatui 0.30+ baseline, 3×3 glyph fallback, loopback-required.

**Within each part, numbers are in original order but not sequential.** Part I contains intentions 1, 10, 11, 12, 14, 15, 22, 23, 24, 25, 26, 26A, 28, 29, 33, 40, 43. Part II contains the remaining 26 entries. The split is a navigation aid; the numbers are for cross-reference stability across the corpus of work-packets, design docs, commit messages, and rustdoc that already reference them.

**Note on numbering anomalies.** The pre-split file (v0.6.4) had two intentions numbered 26. The audit pass that produced this file resolved the duplicate by renaming "Consolidation Must Preserve Individual-Item Addressability" to **26A**, mirroring the existing 12/12A pattern. Cross-references to "Intention 26" continue to resolve to "Prefer a Single Source of Truth Over Parallel Seams"; references that cited the second 26 by topic (consolidation, addressability) need a one-character update to 26A.

**When adding a new intention,** decide which part it belongs to:

- If the rule could apply to a different project with similar values, **Part I**.
- If the rule names specific crates, schema fields, version numbers, file paths, or tools, **Part II**.

Borderline cases lean Part II. Numbering continues from the highest existing number in the file, regardless of which part the new intention lands in.

---

# Part I — Durable principles

Posture and engineering discipline that would survive a full rewrite of the codebase. Read this part for orientation: how the project thinks about quality, consolidation, audits, documentation, and library design.

## 1. Grid-First, Ecosystem-Agnostic

tui-vfx's compositor renders to an abstract cell grid; ratatui is *a* consumer, not *the* consumer. This is an intentional architectural commitment, not accidental optionality.

Rules:

1. No ratatui-specific types leak into compositor or recipe vocabulary. A `tui-vfx-types::Cell` that parallels `ratatui::buffer::Cell` exists by design; the cost of the parallel type is paid by the architecture, not reconstructed in consumer code.
2. Adapters translate at L4 (the ratatui-facing boundary), not at L2/L3 (compositor, recipes).
3. Plausible sibling consumers — movie composer, static renderer, wasm embed, SIXEL/SVG exporter, CI visual-regression via grid diffs — are first-class architectural targets, not hypothetical optionality.

Why: grid-first earns its place through validated secondary uses. The cost of the parallel cell type is real; the payback is adjacent uses a single-terminal-library rules out by construction. If we were ratatui-native, we could not ship a movie-player binary without ratatui's widget/event-loop machinery, could not render to wasm without a terminal-emulator shim, could not produce static image exports cleanly. See `MARKETING.md` and the V3 upgrade plan's architectural framing.

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

## 14. Engineering Workflow Is Test-First and Audit-Gated

Development follows TDD: write tests first, observe failure (red), implement minimal code to pass (green), then refactor safely while preserving passing tests. Mocks are used sparingly; prefer real integration paths and realistic scenarios whenever practical, so behavior validates against actual contracts.

At phase end, run formatting and lint gates (`rustfmt`, `clippy` with warnings-as-errors), and fix issues close to source rather than deferring cleanup. Each phase ends with an explicit audit against the phase plan and codebase, with checks for completeness, gaps, regressions, performance risks, security issues, and architecture-boundary violations.

Why: red-green-refactor produces code testable by construction. Audit-gating catches what TDD's local focus misses — cross-module interactions, architectural drift, forgotten edge cases, performance regressions.

## 15. Audit Pass Is a Hard Phase Gate

A phase is not complete when development work is "done"; it is complete only when audit passes. If an audit returns findings, fixes must be applied and then the full audit must be rerun end-to-end. Partial or informal spot-checks are not a substitute. Teams move to the next phase only after an explicit auditor pass on the rerun.

Why: "done" is a dangerous word. A passing audit is an evidence-based claim; "feels done" is wishful thinking that compounds into technical debt across every subsequent phase.

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

## 26A. Consolidation Must Preserve Individual-Item Addressability

When consolidating N independent units (recipe files, primitive definitions, fragment definitions, asset entries) into fewer containers via `template + variants`, `$use` fragment libraries, aggregator manifests, or future bundler primitives, the consolidation must preserve individual-item addressability for debug, preview, reference, and introspection use cases. The file path and unit identity are UX contracts with tooling, not just storage conventions.

Rules:

1. **Identify all consumers of individual-unit addressability before consolidating.** Debug players, file-pickers, documentation, search tooling, validators, probes, traces, external references. If any depend on per-unit addressability, that dependency must be addressed before consolidation lands.
2. **Debug, preview, and reference recipes stay individual by default.** Their one-file-one-preview contract *is* their utility. Consolidation is for production recipe sets where individual items are implementation details.
3. **Consolidation mechanisms that collapse addressable units must ship with tooling updates.** A canonical variant-URI scheme (e.g., `easing_family.json#back_out`) and validator + probe + trace support for that scheme are required. No consolidation lands until tooling supports its expanded form.
4. **Metadata declares consumption mode.** Consolidated files declare intended consumption (`programmatic` / `individual_preview` / `both`). Tooling respects the declaration.
5. **When in doubt, don't consolidate.** Duplication is cheaper than broken addressability. Abstract only when the generalization is proven and the tooling cost has been paid.
6. **Retrospective corrections are valid and encouraged.** If a past consolidation broke addressability, fixing it by re-expanding or extending tooling is the right action — not living with the regression because "it was already shipped."

Why: consolidation for file-count reduction serves programmatic consumers at the expense of human-facing and tool-facing consumers. Both matter. The `recipes/easing/easing_family.json` retrospective (26 individual files collapsed to 1 `template + variants` file, regressing the demo file-picker) is the canonical example of what this intention exists to prevent. This intention is Principle 4 of the V3 upgrade plan; it's codified here as a durable cross-version rule.

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

## 33. Documentation Indexes Are Navigation Contracts

Every `INDEX.md` file is a navigation contract for a busy repository, not a
best-effort table of contents. When docs move, split, merge, promote from plan
to core documentation, or get archived as closed-out design history, update the
nearest relevant index in the same workset.

Rules:

1. If a file path changes, update every index that pointed at the old path before
   committing.
2. If a planning document becomes durable core documentation, add or move its
   link into the core docs index rather than leaving it discoverable only through
   the V3 planning tree.
3. If a document becomes transient/closed-out history, say so in the index or
   move it to an archive section instead of letting stale status masquerade as
   live guidance.
4. Keep index descriptions short and concrete. They should answer "start here
   or not?" without making the reader inspect every file.
5. Treat missing index updates as a documentation bug, especially in versioned recipe/schema work
   where plans, as-built docs, generated docs, and tooling guides all coexist.

Why: large repos punish treasure hunts. Indexes are how humans and AI agents
recover context quickly without grep archaeology. Keeping them current preserves
trust, reduces duplicate work, and prevents old planning notes from pretending
to be today's map. The occasional joke is welcome; a stale map with a funny hat
is still a stale map.

---

## 40. No landmines: fix root causes, no per-site `#[allow]`, no half-finished consolidations

When a strict lint gate (e.g. `cargo clippy --all-targets -- -D warnings`)
flags an issue, the answer is **never** a per-site `#[allow]` suppression.
Find the root cause and fix it, set explicit project-level policy in
`clippy.toml` with a comment explaining the rationale, or restructure the
code so the lint doesn't fire. `#[allow]` is a landmine: the next code
addition past the suppression silently inherits the bypass; the reason for
the suppression decays in code review; and it hides downstream behaviour
changes a future lint update would have caught.

Rules:

1. **No per-site `#[allow]` for clippy gates.** If the lint is firing on
   real code-style debt, refactor the code. If the lint's default doesn't
   fit the crate's nature (e.g. `too-many-arguments-threshold` for a
   math/geometry crate where 8–9 positional `f32` args are idiomatic), set
   the threshold globally in `clippy.toml` with a one-line comment
   explaining why. That is a conscious, contributor-visible policy
   decision; per-site suppressions are not.
2. **`#[expect]` over `#[allow]` when suppression is genuinely required.**
   `#[expect]` fails the build if the warning later goes away, so the
   suppression doesn't outlive its reason. Reach for it only when (a)
   restructuring is genuinely worse than the lint is right, and (b) a
   short comment in source explains the trade-off.
2A. **`-D warnings` is non-negotiable on the audit gate.** Weakening the
    gate (`-W warnings`, dropping `--all-targets`, scoping out examples)
    to "make it pass" is also a landmine. Fix the code or set the policy
    in `clippy.toml`.
3. **Upstream extractions must be byte-equivalent to the canonical
   pre-lift implementation.** When lifting a primitive from one consumer
   into a shared crate (e.g. `mixed-signals`), the upstream MUST reproduce
   the canonical algorithm verbatim — exact magic numbers, exact shift
   counts, exact normalization. Otherwise the migration silently changes
   downstream behaviour (rain drop positions, spark seeds, render output)
   and tests asserting range properties rather than exact values won't
   catch it. Behaviour-preserving means **bit-equivalent**, not
   *plausibly-similar*. Plan documents that say "behaviour-preserving
   migration" mean exactly that.
4. **No half-finished consolidations.** When extracting shared math or
   collapsing duplicated patterns, every caller migrates in the same
   workset that lands the upstream. A half-merged consolidation is a
   landmine: the next contributor sees two implementations and has no way
   to know which is canonical. Either complete the migration or revert
   the extraction.
5. **Disclosure obligation.** Subagent or solo work that hits an
   out-of-scope clippy/test gate failure must surface it in the report,
   not paper over it with `#[allow]`. Saying "I had to suppress X to keep
   the lint clean" is a flag for the reviewer to evaluate the suppression;
   silently landing the suppression is the failure mode this intention
   exists to prevent.
6. **Pre-commit verification.** Before declaring a workset done, run
   `rg -n '#\[allow|#!\[allow'` over the diff. If new suppressions
   appeared, justify each one in the commit message or remove it.

Why: each landmine is silent at landing time and surfaces unpredictably
later — sometimes weeks later, sometimes at a release boundary. The cost
of fixing root causes once at landing is lower than the cost of a future
contributor (or the same author) re-deriving the suppression's
justification, then either re-suppressing it from inertia or removing it
without understanding what it protected. The user's directive that
prompted this rule was direct: *"stop leaving landmines."* Half-fixes
look productive in the moment and degrade the codebase as durable
infrastructure; this intention is the explicit counter-force.

What this is *not* saying: it is not banning all `#[allow]` everywhere.
Generated code, FFI shims, or genuinely platform-specific paths that
clippy can't reason about may legitimately need one — but those are rare,
documented, and reviewed. The default position when in doubt: don't
suppress; fix.

Companion memory: `feedback_no_landmines.md` in the auto-memory directory
captures the same rule in a more conversational form for in-session
recall.

---

## 43. Plans must orient against the codebase before claiming a contract exists

Implementation plans (anything in `docs/design/`, work-packet documents,
`steering/work-packets/*`, or planning prose in PR descriptions) must run
an `ofpf-defs` / `ofpf-inspect` / `ofpf-content` orientation pass against
every infrastructure contract the plan references. Steering docs describe
a mix of shipped capabilities and aspirational/V3-planned ones; plans
that treat both as equally true bridge the wrong gap and fail on contact
with reality.

Rules:

1. **Verify symbols exist before referencing them.** If a plan says
   "extend `Foo` with `Bar`" or "produces a `HintRef<f32>`," run
   `ofpf-defs Foo` and `ofpf-defs HintRef` first. If results are empty,
   the plan must either build the missing infrastructure as a phase or
   work around its absence — never assume aspirational vocabulary is
   shipped.
2. **Inspect the actual file before describing how to extend it.**
   `ofpf-inspect <path>` returns role/metrics/callers in one call. A
   plan that says "Phase 2 adds a color-space option to `Gradient`"
   should be preceded by inspecting `Gradient` to confirm the option
   isn't already there.
3. **Treat steering docs as 'what we want.' Treat the codebase as
   'what we have.'** When `MARKETING.md` describes a V3 capability,
   verify shipped status before planning against it. The codebase is
   authoritative; steering docs may describe planned-but-unshipped
   contracts (this is normal — steering documents direction).
4. **Document the orientation evidence in the plan.** Include the
   `ofpf-*` queries that confirmed (or refuted) the existence of
   referenced infrastructure, so future readers can re-verify and
   understand which contracts were assumed shipped at plan-write time.
5. **When orientation surfaces existing infrastructure that
   simplifies the plan, say so explicitly.** If Phase N's work was
   already done in prior unrelated work, mark the phase superseded
   with a note about what shipped and where. Don't silently re-do or
   silently skip.

Why: a plan reads as authoritative once it's written; if it claims a
contract exists, downstream readers (humans or agents) take that as
truth and build on it. The TTE port plan's v0.1.0 was assembled by
reasoning over `MARKETING.md` + `INTENTIONS.md` + `pro/main.rs`. It
assumed a `HintRef<f32>` / sampler-emits-hint contract existed because
MARKETING.md describes it as a V3 capability. `ofpf-defs HintRef`
returned 0 hits — planned, not shipped. It also missed that
`ColorSpace` + `blend_colors(..., space)` + `Gradient.space` were
already in place, so what the plan called "Phase 2 — Gradient HSL
color-space option" was already done in prior v2 work. Two phases
collapsed and one needed a redesign on contact with reality. Cost of
the orientation pass at plan-write time: a few minutes. Cost of
discovering the gap at implementation time: a redesign mid-flight.

This is Intention 42 ("default to `ofpf-*` for codebase questions")
applied to the planning lifecycle. Intention 42 says use the tools for
"where is X?" questions; Intention 43 makes it concrete that *plans
themselves* are full of "where is X?" questions, often phrased as
"extend the existing X."

What this is *not* saying: it is not banning aspirational planning. A
plan may legitimately propose new infrastructure that doesn't exist
yet — that's the whole point of planning. The intention is that the
plan be **honest about the gap**: "this requires building HintRef
infrastructure as Phase 0; today the codebase has no equivalent" is a
valid plan shape. "Phase 3 produces a `HintRef<f32>`" without
acknowledging that `HintRef` doesn't exist is the failure mode.

Companion: `feedback_plan_from_codebase.md` in auto-memory captures
the same rule conversationally; `steering/OFPF-TOOLS.md` is the
practical reference for the orientation queries.

---

# Part II — Project-specific rules

Operational rules tied to specific tui-vfx surfaces — crate boundaries, naming prefixes, schema versions, debug recipes, the binding system, the `ofpf-*` tooling, V3 vocabulary, ratatui baseline, and so on. These move with the codebase and may be retired when the surface they govern changes.

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

When downstream adoption discovers a genuinely different geometric model, the
default response is:

- move the generic reusable spatial math into `mixed-signals`
- keep effect semantics in `tui-vfx`
- represent the new geometry as an explicit foundational basis, not as a silent
  semantic mutation of an older leaf

Why: overloading an existing primitive to mean two different geometric things
creates hidden drift. A new explicit basis keeps both the substrate and the
effect code legible.

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

## 27. Byte-Source Loading at All Recipe Boundaries

All recipe loaders, fragment resolvers, substitution APIs, and asset loaders accept byte-source abstractions — they do not assume filesystem access. Concrete implementations may read from the filesystem, embedded resources, zip archives, HTTP responses, or any `impl Read` source.

Why: consumer apps ship in varied environments — embedded resources in binaries for distribution, wasm with no filesystem, mobile with sandboxed storage, CI with only in-memory fixtures. Filesystem-assuming loaders foreclose these cases. This intention preserves the option the V3 upgrade plan's distribution-and-packaging deferred-design section names; it does not require us to solve the packaging design today, only to not preclude it. It's a cheap discipline that costs nothing now and preserves every future option.

## 30. Normalize vocabulary while normalizing math

When a shader, mask, filter, sampler, or related effect file is being touched
for shared-math extraction or geometry normalization, review the file's
terminology in the same pass and normalize it toward the canonical vocabulary.

Rules:

1. If the edit already changes the geometry/math substrate, also check whether
   the file is using drifting words for direction, origin, shape, basis, or
   enter/exit relationship.
2. Prefer updating the file to the canonical vocabulary in the same workset
   rather than leaving the math modernized but the terminology stale.
3. If the file exposes user-facing docs/comments/rustdocs, align them with the
   canonical vocabulary guide while the context is fresh.
4. If the right term is unclear, update the vocabulary guide first or in the
   same tranche, then normalize the file to match it.

Why: math normalization without vocabulary normalization leaves half the drift
in place. The point of the current review lane is not only to share geometry
primitives, but also to keep the language around those primitives coherent.

## 31. Debug recipes are visual reference fixtures, not minimal smoke cases

`debug_recipes/` exists so humans can clearly see what a primitive or composed
effect is doing while the implementation evolves. Those recipes are not just
"does it render?" smoke cases; they are visual reference fixtures and release
reference baselines.

Rules:

1. Debug recipes should use a layout that makes the intended effect legible.
   If a wide/short panel makes a radial or iris reveal read like a wipe, the
   recipe should use a squarer canvas or add a companion geometry-clarity
   fixture.
2. Recipe size should be large enough for the effect to read. Tiny canvases that
   collapse the effect into noise are not acceptable reference fixtures.
3. Color, contrast, and content should be chosen to reveal the effect clearly.
   Border, foreground, background, and message choices should help the viewer
   understand what is happening, not hide it.
4. When a legacy-parity fixture must keep an awkward presentation for
   comparison, add a companion fixture that shows the effect more clearly rather
   than forcing one recipe to do both jobs badly.
5. If a debug recipe visually aliases another primitive, treat that as a
   fixture-design problem to fix — not automatic evidence that the primitive
   semantics are wrong.
6. There should be at least one **base reference recipe per individual
   primitive/effect** in its representative directory. If a primitive has
   meaningful variations, add one fixture per variation there.
7. Recipes that combine multiple effects belong in a **compositions/combined**
   lane, not in the individual primitive directories. Primitive directories are
   for understanding the primitive itself; combination directories are for
   understanding interaction between primitives.
8. Every debug recipe must have a description that tells the viewer what they
   should expect to see. The description is part of the fixture contract,
   because it appears in demo/explorer UI and gives the human checker an
   explicit statement of intended behavior.
9. Debug-recipe body text should follow a deliberate, standardized pattern.
   The message is not filler; it is part of the fixture presentation. Prefer:
   - first line: the actual human-readable test/effect name (for example
     `Iris Effect`) so the viewer knows what primitive is being discussed
   - second line: a short behavioral cue when the effect benefits from explicit
     viewer guidance
   Avoid unnecessary prefatory words like `Watch` when the intent is already
   obvious from context; they add clutter without adding meaning.
10. Debug-recipe layout must comfortably contain its body text. If the chosen
   label + behavioral cue does not fit cleanly, increase the fixture size or
   retune the content. Do not leave clipped, cramped, or ambiguously wrapped
   text in a reference fixture.
11. Debug recipes are part of the first-quality impression of the library.
   They should be professional, correct, visually clean, free of obvious
   presentation mistakes, useful as references, and instantly understandable in
   their content.

Why: debug recipes are how contributors, reviewers, and downstream integrators
build intuition. They also give maintainers and downstream developers a known
set of working reference recipes to benchmark against during release work and
regression investigation. A fixture that technically exercises the code but
visually conceals the effect fails its job as a reference artifact. Keeping
single-primitive references separate from combinations is how the library
avoids losing diagnostic clarity as the corpus grows. A missing or vague
description removes the human-side acceptance criteria that makes the fixture
useful in the demo UI. Including the test/effect name on-screen also makes it
much easier to match a rendered fixture back to its file during browsing,
comparison, and regression triage. If the text does not fit the surface, the
fixture stops serving as a trustworthy visual reference. These demos are also
the baseline many new users will use to judge the quality, consistency, and
ergonomics of the library itself, so they must feel intentionally professional.


## 32. Schema Field Vocabulary Is the Naming Anchor

When docs, directory names, fixture names, examples, rustdocs, and authoring
guidance refer to a defined versioned schema concept, use the schema field's canonical
vocabulary unless there is a deliberate migration/deprecation reason not to.

Rules:

1. Prefer exact schema terms over near synonyms. If the field is `motion.route`,
   name the reference lane `motion_routes`, not `motion_paths`, unless the doc is
   explicitly discussing a lower-level path primitive.
2. Keep adjacent concepts visibly distinct. `easings/` is for easing functions;
   `motion_routes/` is for route/path-shape fixtures that use `motion.route`.
3. When a rename opportunity appears during versioned schema work, align the human-facing name
   with the canonical field vocabulary in the same slice when the blast radius is
   small and testable.
4. Avoid introducing new aliases in prose, file paths, or recipe IDs unless the
   alias is part of an explicit compatibility story.
5. If the canonical schema term feels wrong, fix the schema vocabulary decision
   or document the exception; do not let local docs drift into a parallel dialect.

Why: consistency builds trust. Authors learn the system faster when the file
tree, examples, docs, rustdocs, validators, and schema fields all use the same
words. Ambiguous or varied language around defined fields makes the API feel
less coherent, increases AI-authoring error rates, and forces humans to ask
whether two different terms imply two different behaviors.


## 34. Pipeline-touch changes carry full family obligations

When a v3.1 shader, filter, mask, sampler, style, content effect, motion route,
shadow, scope, binding, or adjacent pipeline file is touched — or when another
explicitly named versioned schema surface is touched, treat the edit as
family work rather than a one-line local patch. The implementation, vocabulary,
docs, fixtures, and validation evidence must move together.

Rules:

1. **Shared math goes down, effect semantics stay here.** If the touched code
   reveals reusable signal/math substrate that is primitive, renderer-agnostic,
   and useful to three or more real callers, promote that substrate to
   `mixed-signals`. Keep renderer/effect semantics in `tui-vfx` or
   `tui-vfx-recipes`.
2. **Normalize time deliberately.** Keep normalized phase/loop progress
   separate from monotonic elapsed time. Cadence-driven motion uses elapsed
   time; loop progress remains available for phase-based effects. Do not patch
   timing bugs with recipe-period hacks.
3. **Update rustdocs and generated-doc inputs.** Public items, schema-bearing
   types, fields, enum variants, and behavior-critical contracts touched by the
   change need meaningful rustdoc. If the change affects generated schema/API
   docs, update the code-side metadata or curated inputs and run the relevant
   drift/coverage validation before claiming the work is done.
4. **Use canonical vocabulary while the file is open.** Align code comments,
   docs, recipe names, fixture directories, and rustdocs with the applicable
   versioned vocabulary guide. Do not add convenient synonyms for schema terms.
5. **Refresh debug/reference recipes when semantics change.** If visual
   semantics, naming, timing, authoring parameters, or parameter defaults change,
   add or update the simplest primitive-first debug recipe plus any composition
   fixture needed to show the interaction clearly.
6. **Prefer mechanical validation over memory.** When the edit exposes a drift
   class or missing invariant, add or extend the validator/probe/test/gate that
   would catch it next time, unless doing so would be disproportionate to the
   change.
7. **Respect metadata and index hygiene.** Keep `<CLOG>` entries to the latest
   one- or two-line summary, and update relevant `INDEX.md` files when docs or
   documented surfaces move, split, merge, or become canonical.
8. **Tooling artifacts move with the architecture.** A material design change
   to the main code (a new trait family, a new pipeline slot, a renamed
   wire-format type, a new event variant, a migrated schema field, an added
   blend mode, a new canvas-extent shape) propagates *in the same change* to
   every tooling surface that consumes or describes it:
   - tooling crate rustdocs (`tui-vfx-probe`, `tui-vfx-debug`, `tui-vfx-trace`,
     `pipeline-validator`, `recipe-probe`),
   - tooling design docs (`docs/design/pipeline-probe-design.md`,
     `docs/design/tui-vfx-pipeline-observability.md`, plan docs that reference
     the changed surface),
   - authoring guides and the autogen reference (`docs/generated/`,
     `docs/templates/capabilities.toml`, signals reference, schema reference),
   - any work-packet plans for those tools that reference the changed surface
     by name.

   The trigger is "a reader of any of those artifacts would now read something
   materially wrong." The discipline is "land the architecture work and the
   tooling-artifact updates as one change, not as a follow-up." Architectural
   drift across tools is the longest-tail debugging tax we ship: tooling that
   describes a stale model produces investigations that confirm the wrong
   mental model and miss the bug. This rule is the durable counter-force.

Why: pipeline capability is central to versioned recipe/compositor work, but version labels are scope. V3 and v3.1 are different surfaces; do not describe v3.1 work as generic V3 pipeline work. A shader or mask can be
made to compile locally while still leaving stale timing language, undocumented
schema fields, missing fixtures, duplicated math, or untested authoring
behavior behind it. Those debts are hardest to detect after context has cooled.
Pipeline-touch work is therefore complete only when the family is coherent
across implementation, generated documentation, hand documentation, recipes,
validation evidence, *and the tooling artifacts that describe the surface to
the next investigator*.


## 35. Lead onboarding with the architectural identity

When introducing tui-vfx to a human or AI author, start with the architecture:
tui-vfx is a high-performance Rust terminal scene renderer, VFX compositor, and
recipe-driven animation runtime for grid-based UIs. Do not lead with
"effects library" as the primary frame unless the task is specifically about
the low-level post-processing surface.

Rules:

1. The first sentence in prompts, READMEs, marketing notes, and agent briefings
   should establish the scene-renderer / VFX-compositor / recipe-runtime mental
   model before listing individual effects.
2. Treat flagship recipes such as Madeira as primary usage examples of the scene
   renderer, not as advanced optional curiosities.
3. When asking an AI author to inspect or use tui-vfx, direct it to ask the
   architecture questions first: where is the scene model, how are layers
   composed, how does the animation clock work, how do recipes drive rendering,
   and where do pipeline stages consume and produce data?
4. Keep "effects" language accurate but subordinate. The post-processing effect
   layer exists, but it is one surface inside the broader V3 renderer/runtime.
5. When comparing tui-vfx to adjacent effects libraries, do not create a false
   layer split where they "do effects" and tui-vfx "does scenes." tui-vfx is
   feature-competitive in the effects/compositor lane and also owns the broader
   scene-renderer and recipe-runtime architecture.

Why: framing determines search strategy. An "effects library" prompt sends an
author looking for `apply_effect(grid, params)` APIs and encourages confirmation
bias when those APIs are found. The architecture-first prompt sends the author
to the scene graph, composition model, clock, recipe runtime, and flagship
recipes first. That is the map needed to use the active versioned recipe/runtime surface seriously.


## 36. The 3x3 line glyph table is the default and fallback font

The canonical heavy-weight Line 3x3 glyph table (a fixed-cell ASCII/box-drawing
font covering space, digits, A–Z, common punctuation, currency, and a small
operator set) is tui-vfx's default font and the runtime fallback when a recipe-
declared font cannot be located. The table lives at
`tui-vfx-content/src/fonts/col_line_3x3_heavy_glyphs.rs` and is mirrored
byte-for-byte from gt-design's `gtd-components` so the family agrees on which
strokes draw which glyph.

Rules:

1. **Default when none declared.** Recipes that exercise typography (Odometer
   3x3 digit drums, font-rendered marquees, Solari-style multi-cell flap
   stacks) without an explicit font selection render through the Line 3x3
   table.
2. **Fallback when declared font is missing.** When a recipe names a font that
   the loader cannot resolve (filesystem miss, wrong path, asset not embedded),
   the runtime emits one warning to the trace surface and renders through the
   Line 3x3 table instead of failing the recipe outright.
3. **Strict mode is a validator concern.** The recipe validator (in
   `tui-vfx-recipes`) MAY reject missing-font recipes at load time when run in
   strict mode. The runtime fallback is for resilience; strictness lives one
   level up.
4. **Cell shape is fixed.** Line 3x3 is exactly 3 cells wide and 3 cells tall.
   Recipes that author against a different cell shape (e.g., 20-row braille
   faces) and lose their declared font will visibly degrade — a tile spec of
   `tile_width: 5, tile_height: 8` will render Line 3x3 glyphs padded into
   the larger tile rather than scaled. This is deliberate: silent rescaling
   would hide the asset miss.
5. **One canonical home.** The glyph table lives once in
   `tui-vfx-content/src/fonts/`. Other crates (recipes, debug, probe, examples)
   import from there rather than duplicating the table. Consolidating later
   into a sibling `tui-vfx-fonts` crate is a possible future refactor; doing
   it now is premature (only one consumer).
6. **Authoring helper exposed publicly.** A small public helper that maps
   `(char, weight) → [&str; 3]` lives alongside the table so AI authors and
   recipe-tooling code can synthesize multi-line glyph strings programmatically
   without reaching into internals.

Why: a recipe library at scale (500+ recipes, AI-authored extensions) must
survive font drift. Treating the canonical Line 3x3 face as the default removes
a class of "blank screen on missing asset" recipe failures, normalizes the
authoring experience for typography effects, and keeps tui-vfx renderable
without external font assets at all. Per Principle 5 (meaning lives low,
policy lives high): the canonical glyph table is meaning; "should we reject
the recipe instead of falling back" is policy and lives at the validator
boundary.


## 37. Loopback is required: every binding declaration is preview-playable

Every entry in a recipe's `requires_bindings` block (and, by parity, any
future bindable-asset declarations under `requires_assets`) must yield an
**effective loopback** value. Authors supply it explicitly via
`loopback: <value-or-signal>`, or the legacy `default: N` form lifts to a
static loopback automatically. The strict-contracts validator rejects any
declaration where `effective_loopback()` returns `None`. "Production-only
bindings" — declarations meant to be wired only by a live host with no
preview behavior — are not a valid category.

Rules:

1. **Hard rule.** Every `requires_bindings` entry must yield an effective
   loopback. Recipes that try to declare a binding without one fail
   `--rules --strict-contracts`. The same rule extends to bindable
   `requires_assets` entries when the loopback layer reaches them.
2. **Best practice for fields where loopback is technically optional.**
   Even where the schema does not enforce loopback (legacy fields, future
   forward-compat shapes, or non-binding `requires_assets` slots that
   carry a `canonical_path` instead), populate a loopback or canonical
   default anyway. A populated value keeps the player rendering
   well-formed when it's driven by loopback params, and it prevents
   surprise empty-state cells from leaking into preview tiles.
3. **Synthetic loopbacks for "production-only" intent.** When a binding's
   real value only makes sense in production (live user count, runtime
   metric, host-supplied scrubber), the recipe still declares a synthetic
   loopback that makes preview meaningful — a static literal, a slow
   `ramp` signal, or a sentinel like `default_font` once that forward-
   compat shape lands. Never advise omitting loopback to "indicate
   production-only intent." Document that intent in the entry's
   `description`; supply a synthetic loopback for the player.
4. **Forward-compat shapes still satisfy the gate.** A future
   `loopback: { "player_default": "default_font" }` form delegates
   fallback resolution to the player's authority; it still produces an
   effective loopback at frame time, so it satisfies strict-contracts
   without requiring authors to hardcode asset names in recipes.

Why: a recipe that requires host wiring to render is a recipe that can't
be reviewed in the browser, can't be exercised by `--probe`, can't be
demoed, and rots unobserved between authoring and the production host
coming online. Making preview-playable a per-recipe contract — enforced
at the validator boundary — eliminates that whole class of "looks fine,
fails in prod" failures. The user's framing was direct: *"I want loopback
to be required so recipes are always functional and portable before the
application that drives them is available."* That portability is a
project-level commitment, not a per-recipe choice.

What this is *not* saying: it is not requiring loopback to be a literal
of the production value. The point is that *something visually meaningful*
plays in preview, not that the preview matches production. A binding that
selects a live user count can have `loopback: 42` (or a `ramp` signal) —
the preview shows recognizable activity, the production host injects the
real number, and both render correctly. Authors should reach for the
loopback shape that best communicates the binding's role; "useless
default" is rarely the right answer when a one-line synthetic gives the
recipe a clean preview.

Companions:

- `docs/design/completed/tui-vfx-binding-loopback.md` — design proposal, the WHY.
- `docs/design/completed/tui-vfx-binding-loopback-implementation-plan.md` — phased
  HOW (L1 engine fallback, L2 `requires_bindings` typing + strict-
  contracts gate, L3 visibility badge, L4 strictness modes, L5 probe +
  browser integration).


## 38. Bindable surface gets an explicit marker (transitional)

Recipes and recipe-fixture files that consume a bindable-typed field
at any depth in their config — `font: BindableString` on
`MechanicalContentSource::Preset`, `BindableU16` row coordinates on
`StyleRegion::RowRange` / `Modulo` / `Cell`, `BindableValue::Binding`
shader parameters, future `BindableString` asset references — carry
two explicit markers so they stay distinguishable in the noise of a
growing recipe library:

1. **Filename suffix `_bindable`.** A recipe that exercises a
   bindable field is named `<family>_<base>_bindable.json`. The
   `_bindable` suffix preserves existing family-grouping (recipes
   alphabetize next to their non-bindable siblings) while making the
   bindable-vs-not distinction visible in any directory listing.
2. **Metadata tag `bindable`.** The recipe's `metadata.aesthetic_tags`
   array includes the literal string `"bindable"`. Programmatic
   filters (the recipe browser, AI-author corpus selection, validator
   discovery) read the tag rather than parsing filenames.

Both markers apply when the recipe carries the bindable shape in its
config — including the literal-form authoring path (e.g.
`font: "line-3x3"` parses to `BindableString::Literal`). The marker
is about the field surface the recipe occupies, not about whether
the recipe currently exercises the binding payload.

Source files: existing OFPF naming already carries "bindable" in the
filename for the type definition (`cls_bindable_string.rs`,
`cls_bindable_u16.rs`). No additional convention is needed for
implementation files; this Intention targets recipes/fixtures.

Why: at debug_recipes-library scale (50+ recipes today, 500+ at
target), recipes exercising the binding surface need to be visible
at a glance for binding-layer testing, migration audits, AI-author
training, and recipe-browser filtering. Without an explicit marker,
bindable recipes get lost in the noise.

**Transitional framing.** This convention exists because the binding-
loopback design is in flight: the strict-contracts validator has not
yet typed the `requires_bindings` / `requires_assets` declaration
shape, and that typing is the binding-loopback plan's L2 work. Once
the validator can derive the same view from a recipe's declared
binding contract — every recipe with a non-empty `requires_bindings`
or `requires_assets` block IS bindable by definition — this marker
may be retired or refined. Until that view exists, the explicit
suffix + tag is the navigation aid.

Mechanical detail when retiring (forward note, not work today):

- The validator's `bindings_summary()` query (sibling's L5) lists
  declared bindings + their loopback presence per recipe.
- A discovery query like `where bindings_summary().is_some()` over
  the recipe corpus yields the same set the marker identifies today.
- At that point, dropping the marker is a mechanical rename + tag-
  removal sweep across the marked corpus; the convention's
  retirement is itself a single coordinated session.

What this is *not* saying: it is not requiring recipes to declare
bindings they don't use. Recipes that don't consume any bindable
field stay un-marked. The marker is a positive signal that the
recipe touches the bindable surface, not a negative requirement
that every recipe declare anything.

Companion docs:

- `docs/design/completed/tui-vfx-binding-loopback.md` — the broader design
  shape this convention bridges to.
- `docs/design/completed/tui-vfx-mechanical-circular-content-cycles-plan.md` —
  Phase 6 / Phase 7 sub-plans that produced the first bindable
  recipes; concrete examples of the marker in use.


## 39. Engine surfaces are recipes, not parallel renderers

When the engine itself needs to draw something visible — a debug
badge, a status indicator, a probe overlay, a watermark, a
diagnostic toast — the answer is **author it as a V3 recipe in the
standard JSON format on disk**, not a hardcoded cell-painter or
parallel rendering primitive. The recipe is `include_str!`-inlined
into the binary so it ships with the engine, but the artifact lives
on disk in `recipes/internal/` (or equivalent), is editable by hand,
and goes through the same compile + render path every other recipe
uses.

Rules:

1. **Default to recipes.** If the visible thing can be expressed
   with the V3 vocabulary (text + layout + base_style + pipeline
   + motion), it MUST be a recipe. Inventing a parallel "overlay"
   primitive that paints cells directly is a code smell that
   indicates the vocabulary needs to grow, not that the engine
   needs an escape hatch.
2. **Inline at compile time.** Engine-required recipes
   (`recipes/internal/*.json`) are bundled into the binary via
   `include_str!()` so they're always available, but the source of
   truth stays on disk. This means recipe-authors can iterate on the
   appearance without touching engine code, and the recipe browser
   shows them alongside every other recipe.
3. **Cache the compile, not the JSON.** The first call parses +
   compiles the inlined JSON; the compiled plan goes in a
   `OnceLock`. Per-frame cost is the render call, not re-parsing.
4. **Vocabulary gaps are not exemptions.** If the V3 vocabulary
   genuinely cannot express the surface (e.g. a 1-row alpha-faded
   status badge with no border at width=4), the right move is to
   extend the vocabulary so the recipe path works — and to do that
   work as part of whatever feature surfaced the gap, not as a
   side-quest that delays the feature. Falling back to a hardcoded
   cell-painter "just for this one" never stops at one.

Why: the engine has exactly one rendering path. Two paths mean two
sets of bugs, two sets of tests, two sets of styling vocabularies,
two places authors have to look to understand how anything renders.
Worse: parallel renderers calcify the choices their initial author
made (color, glyph, position, fade behaviour), so iterating on the
look later means editing engine code instead of editing a JSON file.
The recipe path was built precisely to let the visible behaviour
evolve independently of the engine; bypassing it for engine-internal
surfaces forfeits that flexibility for no upside.

The canonical example: the loopback visibility badge (Phase L3).
The first attempt at L3 hardcoded a 4-cell `[LB]` glyph string,
two `Color { r, g, b, a }` constants, and a per-cell paint loop in
a new `tui-vfx-compositor/src/overlays/` module. The user's
correction landed instantly: *"Don't re-invent or hard code things
we can do with recipes and maintain flexibility."* The badge is now
a tiny V3 recipe (`recipes/internal/loopback_badge.json` and a
Nerd Font sibling), inlined via `include_str!`, compiled into a
small grid via the same render path everything else uses, and
blitted into the top-right of the host scene. Every styling
decision — orange shade, fade approach, glyph choice, padding —
lives in JSON the recipe-author can edit without recompiling the
engine.

The flexibility upside is concrete and load-bearing: because the
badge is a recipe and the host's invocation is "render this recipe
when loopback fires," **switching from a top-right badge to a
center-screen toast notification, or to an ambient banner, or to a
brief fly-in animation, is a recipe-file edit + a one-line change
in the host's call site.** The user's framing: *"if it is a recipe
we can toggle it to a notification later if we want by updating
the recipe and changing how we call it. So... flexible."* The
recipe path exists precisely to keep these visual decisions
editable; engine-internal surfaces forfeit that flexibility the
moment they're hardcoded.

What this is *not* saying: it is not requiring every engine pixel
to be a recipe. The compositing primitives (cell blending, shadow
gradients, mask compose) stay in code — they're the *machinery* the
recipe path uses, not surfaces themselves. The line is: anything a
user would describe as "a visible thing" (a badge, a label, a
toast, a watermark) is a recipe. Anything that's "how the engine
puts pixels on the surface" is code.

Companion docs:

- `docs/design/completed/tui-vfx-binding-loopback.md` — the design that
  surfaced this principle when L3 needed a visibility badge.
- `docs/design/completed/tui-vfx-binding-loopback-implementation-plan.md` —
  the L3 phase notes that record the recipe-based badge
  architecture.


---

## 41. Cross-repo audits scope all four repos: tui-vfx, tui-vfx-recipes, mixed-signals, gt-design

When a change touches a public surface that downstream consumers might reach
— a struct field's visibility, a public type's shape, a free function's
signature, an exported constant — the audit-time `rg` / `ofpf-search` /
`ofpf-content` query MUST cover every repo where that surface could be
consumed:

1. `/usr/projects/tui-vfx`
2. `/usr/projects/tui-vfx-recipes`
3. `/usr/projects/mixed-signals`
4. `/usr/projects/gt-design` (the first production consumer; in scope when
   the surface is anything gt-design imports or constructs)

Two-repo audits that stop at "tui-vfx + mixed-signals" are not enough.
The four-repo scope is the default; narrowing requires a positive reason.

Rules:

1. **Audit before the change lands, not after.** The plan or packet that
   introduces the change must list the audit query and the four-repo scope
   explicitly. "I'll grep callers as I write the code" is not a plan; it's
   how landmines slip through.
2. **Use literal-pattern queries that match the actual construction
   syntax.** `rg "SignalContext\s*\{"` finds struct-literal sites;
   `rg "SignalContext::"` finds method/constructor calls;
   `ofpf-content "<symbol>"` covers non-indexed scripts and docs. Pick the
   query that matches the surface being changed and run it across all four
   repos.
3. **Subagent packets dispatching audit work must name the four-repo
   scope explicitly.** Don't assume the agent will infer it; the packet
   says "rg / ofpf-search the literal pattern across `/usr/projects/tui-vfx`,
   `/usr/projects/tui-vfx-recipes`, `/usr/projects/mixed-signals`, and
   `/usr/projects/gt-design`."
4. **Field-visibility changes are a "large-scale" change.** Promoting a
   field from `pub` to `pub(crate)` (or vice versa) breaks struct-literal
   construction at every external call site. The audit catches them
   before they break a downstream build.
5. **Subagent reports include the audit output.** The packet asks the
   agent to record the rg counts per repo in its final report so the
   leader can verify the audit happened.

Why: the SignalContext lift to `pub(crate) subcell_offset` shipped in
glyph framework Phase 1 with a two-repo audit ("`rg "SignalContext\s*\{"`
in `tui-vfx/crates` and `mixed-signals/src`"). That audit was correct
syntax but wrong scope — it missed tui-vfx-recipes and gt-design, where
~12 production call sites used struct-literal construction. The recipes
build broke; the production consumers became blocked work; the field had
to be promoted back to `pub` in a follow-up commit. Total: one extra
commit, one cross-repo distraction, an Intention 40 violation surfaced,
and the original "extensibility via `pub(crate)`" benefit forfeited.
Cost of the four-repo audit at landing time: thirty seconds.

What this is *not* saying: it is not requiring four-repo audits for every
internal change. Refactoring a private function inside one crate stays
local. The scope is set by the surface being changed: if it's
public/exported/visible to consumers, the audit covers all four. If it's
purely internal, the local repo is enough.

Companion: `feedback_no_landmines.md` in auto-memory captures the
underlying "no landmines" rule; this intention is the operational
counter-force that prevents the audit-scope failure mode specifically.

---

## 42. Learn the `ofpf-*` tooling and use it by default

The `ofpf-*` semantic suite (a thin alias layer over `librarian-cli`,
backed by a long-running multi-tenant daemon) is the canonical interface
for codebase questions in this repository. Use it before reaching for
`find`, `grep`, `cat`, or whole-file reads. When you are not yet fluent
in the tool surface, **read `steering/OFPF-TOOLS.md`** — it is the
practical reference for which tool answers which question, output-handling
patterns, multi-repo workflow, response-guard semantics, and the
non-obvious flags that bite first-time users.

Rules:

1. **Default to `ofpf-*` for any "where is X?" / "who calls Y?" / "what
   does this file do?" / "what breaks if I change Z?" question.** Raw
   `find` / `grep` / whole-file reads are fallbacks for cases the
   `ofpf-*` suite genuinely cannot reach (small ad-hoc shell tasks,
   non-text files, pre-load probing).
2. **Read `steering/OFPF-TOOLS.md` once per session if you have not
   internalized the tool surface.** It is a fast skim and pays back the
   first time it saves a wrong-tool detour. The CLAUDE.md orientation
   step links to it transitively through this intention.
3. **Use `librarian-cli --help-json` and `librarian-cli meta` as the
   canonical reference when in doubt.** `--help-json` returns the full
   command schema (subcommands, args, aliases, exit-code semantics,
   response-guard behavior, JSON-mode protocol). `meta` decodes the
   abbreviated keys (`co`/`in`/`out`/`f`/`p`/`n`/`l`/`k`) used in
   compact responses. Both beat guessing.
4. **Capture new pitfalls and patterns in `OFPF-TOOLS.md`.** When you
   discover a non-obvious flag, an empty-result interpretation, or a
   tool combination that is the right answer to a recurring question,
   add it to the reference so the next session benefits. This is the
   Intention 25 ("hunt for infrastructure wins") application to tooling
   knowledge.
5. **Treat `ofpf-status` failure as a precondition violation.** If the
   daemon is unhealthy, surface it before answering repo questions
   instead of silently falling back to slower or less accurate
   alternatives.

Why: the `ofpf-*` suite is daemon-backed, multi-repo (up to ten loaded
in parallel via `--root`), and returns structured JSON that composes
cleanly with downstream tooling. It is materially faster than shelling
out for the same query, and the response guard protects context from
runaway result sets. Treating it as the default — and keeping a living
reference for it — keeps every contributor (human or AI) on the path of
least context burn. The reference exists because the tool surface is
larger than any one session needs to memorize, and because the standards
docs in `~/.claude/rules/` are necessarily generic; `OFPF-TOOLS.md` is
the project-local distillation.

What this is *not* saying: it is not banning shell utilities. `git`,
`cargo`, `just`, `rg` directly, and small scripts remain the right
answer for execution, build, and ad-hoc tasks. The intention targets
**codebase interrogation** specifically — symbol lookup, dependency
reasoning, blast radius, file neighborhood — where the `ofpf-*` suite is
purpose-built and the alternatives leave performance and clarity on the
table.

Companion: `steering/OFPF-TOOLS.md` (project-local reference);
`~/.claude/rules/ofpf.md` and `~/.claude/CLAUDE.md` (global standards
that introduce the suite); `librarian-cli --help-json` (canonical
upstream schema).

---

## 44. Recipe-JSON signal authoring goes through `VfxRecipeSignalSpec`; engine direct-API consumers use `mixed_signals::*` directly

The two surfaces are intentional and meet at `SignalOrFloat`-typed engine
fields. The facade at `tui_vfx_recipes::signals::VfxRecipeSignalSpec` is a
thin newtype around `mixed_signals::SignalSpec` with a custom `Deserialize`
that gates the recipe-author catalog at the JSON boundary. Direct-API
consumers — those constructing `FilterSpec`, `MaskSpec`, `SamplerSpec` in
Rust — depend on `tui-vfx` and `mixed_signals` directly. The facade does
not exist for them.

Rules:

1. **Recipe deserialization seams use the facade.** Every JSON-deserialized
   signal expression in `tui-vfx-recipes` routes through
   `VfxRecipeSignalSpec`. The newtype's `Deserialize` rejects any `"type"`
   discriminant not in `vfx_recipe_signal_catalog()`; the engine substrate
   `mixed_signals::SignalSpec` is produced one layer below.
2. **Engine field types stay engine-native.** `FilterSpec.factor:
   SignalOrFloat`, `VfxBindableValue::Signal(SignalOrFloat)` (which is
   `VfxBindable<f32, SignalOrFloat>::Signal(_)`), and similar engine field
   types remain. The facade lives one layer above the engine; lowering is a
   one-liner because the newtype wraps the substrate directly.
3. **One wire format.** `VfxRecipeSignalSpec` and
   `mixed_signals::SignalSpec` share the JSON shape by construction.
   Recipes that worked through `mixed_signals::SignalSpec` continue to work
   through the facade. The catalog is the only thing that diverges — and
   only as a *subset* of `SignalSpec` discriminants, never a parallel
   encoding.
4. **Adding a recipe-author variant is a deliberate decision.** New
   `mixed_signals` primitives do not auto-expose. Adding a variant requires
   three things: (a) the variant exists in `mixed_signals::SignalSpec` with
   stable wire format, (b) a `VfxRecipeSignalMeta` entry in
   `vfx_recipe_signal_catalog`, (c) a round-trip serde test in
   `test_signals.rs`. No wrapper struct, no dispatch arm — the newtype's
   delegation to `SignalSpec::build` handles construction.
5. **Strict-contracts validator enforces curation.**
   `validate_normalized_recipe_strict_contracts` walks the recipe's raw
   `Value` payloads (filter / shader / sampler payloads stay as
   `serde_json::Value`, not typed) and rejects any `{"signal": {"type":
   "<x>"}}` whose `<x>` is not in `vfx_recipe_signal_catalog()`. This
   catches authorings that bypass the typed `Deserialize` boundary.
   Drift-prevention per Intention 25.
6. **Direct-API examples use `mixed_signals::*` directly.** Examples in
   `tui-vfx/examples/` (e.g. `direct_api_signal_strength.rs`) show signal
   usage through `SignalOrFloat::Signal(SignalSpec::...)`, not through the
   facade.
7. **`Binding(String)` is orthogonal.** Host-supplied runtime values flow
   through `RuntimeBindings`. Both authoring paths can use them; this rule
   does not constrain bindings.

Why: a single recipe-author entrypoint lets us swap, plug-in,
exposure-limit, rename, or remap signals without touching recipes or
examples; lets us attach authoring metadata once (the catalog drives
`docs/generated/RECIPE_SIGNALS_REFERENCE.md` via the doc generator,
packet 01); and lets the validator enforce the curation invariant. The
two-surface design is intentional because the audiences (Rust developers
writing direct-API code; recipe authors writing JSON) have different
ergonomic needs (type safety vs themability / AI-authoring / hot-reload /
probe visibility).

What this is *not* saying: it does not say the facade introduces a parallel
type system or a rename layer over `mixed_signals`. The facade is a *thin*
newtype gate — wire format is identical to the substrate — not a
re-encoding. Phase γ explored a parallel-enum + per-variant wrapper-struct
design and Phase 2 (packet 65) redesigned away from it because the
parallel encoding leaked wire-format-parity gaps into the recipe corpus
(per-field `#[serde(default)]` annotations live on `SignalSpec`'s enum arms
not the underlying structs; `SignalSpec::Keyframes::keyframes: Vec<(f32,
f32)>` and `mixed_signals::generators::Keyframes::keyframes: Vec<Keyframe
{ time, value }>` are not wire-format equivalent). Future contributors:
do not reintroduce wrappers without weighing the parity-debt cost.

Companion: `tui-vfx/docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md`
§9 (the architectural plan and its as-built record);
`tui-vfx-recipes/src/signals/mod.rs` (the facade module rustdoc);
`steering/work-packets/completed/64-recipe-signal-facade-completion-phase1.md`,
`completed/65-recipe-signal-facade-consolidation-phase2.md`,
`completed/66-engine-vs-recipe-player-delineation-phase3.md` (the three
packets that shipped this surface).

---

## 45. tui-vfx-compost v3.1 work is pure v3.1 end to end; packet structure and touched-file lists are leader-reviewed gates

The active v3.1 compositor target is `tui-vfx-compost`. Historical copied-crate
work is reference/recovery material only, not a reusable subagent instruction.
Future compost work migrates proven behavior in place, in the existing assigned
workspace or leader-provided worktree, by lightly updating implementation
boundaries to consume canonical v3.1 schema fields directly.

Rules:

1. **Pure v3.1 end to end.** The accepted path is canonical v3.1 recipe/load
   structures plus explicit sample context into tui-vfx-compost runtime
   execution. Do not adapt v3.1 back into `CompositionSpec`,
   `ShaderLayerSpec`, `SpatialShaderType`, legacy-shaped field names, bridge
   DTOs, shim DTOs, or transition-seam lowerers. Adding such a layer is a
   failure condition; halt and correct scope.
2. **No per-agent crate copies.** Historical copied-crate work is not the active
   target and must not appear as a narrow subagent task. A subagent assigned to an existing worktree already
   has its isolated checkout; it must not create nested clones, nested
   worktrees, project copies, or crate copies.
3. **Packets include expected tree shape.** Every implementation/refactor work
   packet names the intended file tree and approximate file-name breakdown before
   work starts. It marks expected edits, expected new files, generated files, and
   should-not-touch files. If the work needs a different structure, the agent
   reports that deviation before broad edits.
4. **Touched-file list is reviewed every time.** Every subagent report lists all
   created or edited files and marks each as `edited-existing`, `new-authored`,
   `copied`, `moved`, or `generated`. The leader must review that list before
   acceptance/integration. Unexpected files, broad edits, copied source,
   generated artifacts, or unexplained origins are blockers until corrected.
5. **OFPF structure is part of correctness.** The migration is also the chance to
   break large legacy files into professional OFPF-compliant modules. File layout
   is not cleanup after the fact; it is part of the packet's definition of done.

Why: tui-vfx-compost exists to remove transition complexity, not to institutionalize
another translation layer. If work packets omit tree shape, origin reporting, or
leader review of touched files, capable helpers will fill gaps with local
structure choices and duplicate code. The result is exactly the failure this
intention prevents: technically compiling work that increases legacy surface area
and creates cleanup debt before the primitive migration is even finished.

Companion: `steering/ORCHESTRATION.md`;
`steering/work-packets/COMMON_EXECUTION_RULES.md`;
`docs/arch/tui-vfx-compost-agent-workflow-handoff.md`;
`docs/arch/tui-vfx-compost-vertical-implementation-plan.md`;
`docs/arch/v31-schema-boundary-north-star.md`.

---

<!-- <FILE>steering/INTENTIONS.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.10.0</VERS> -->
