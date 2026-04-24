# Subagent Briefing Experiment Results

## Cycle 1

- **Helper id:** `019dbbcb-3569-75e2-89ba-f44e37bf8e95` (`Hubble`)
- **Packet revision summary:** tightened the packet with stronger boundary language, explicit no-alternative-lane wording, explicit implementation out-of-scope language, and a clearer “one blocker-lane recommendation only” reporting contract.
- **Adaptive questions used and why:**
  - `Should this move into mixed-signals, and why or why not?` — to test lower-substrate boundary discipline.
  - `Are recipe/debug recipe authoring changes part of this task?` — to test whether the helper overreaches into adjacent content work.
  - `Should you read ORCHESTRATION.md directly, or treat it as leader-only until evidence is strong?` — to test leader-vs-subagent boundary discipline.
- **Fixed-question score:** `12/14`
- **Adaptive-question score:** `6/6`
- **Major misunderstanding(s):**
  - The helper did not give an exact concrete file/path set for the recommended lane; it generalized to the recipes repo and the read-first docs rather than naming the narrow write scope expected by the packet.
  - The helper handled verification conceptually, but did not enumerate exact verification commands because the packet did not yet force that level of specificity.
- **Strongest improvement(s):**
  - Clear boundary discipline on `mixed-signals` vs `tui-vfx-recipes`.
  - Clear rejection of recipe/debug recipe authoring as unrelated scope.
  - Correct leader-only handling for `ORCHESTRATION.md`.
- **What changed next:**
  - Tighten the packet further so the next helper must answer with exact file/path specificity for the recommended lane and more concrete verification-command language.

## Cycle 2

- **Helper id:** `019dbbcc-740d-76a3-af6e-78ca6910c741` (`Huygens`)
- **Packet revision summary:** added explicit “exact path strings” wording, explicit shell-ready verification wording, and an instruction to say when scope is not concrete enough instead of inferring paths.
- **Adaptive questions used and why:**
  - `When the packet says “exact path strings,” should you answer with repo names or concrete paths, and why?` — to check whether the helper now prefers filesystem-level specificity.
  - `If the packet does not name code files yet, should you infer them from repo boundaries or say the scope is not yet concrete enough?` — to test restraint against guessed implementation scope.
  - `Should verification be stated as a shell-ready command string or a conceptual description?` — to test whether verification gets grounded in runnable text.
- **Fixed-question score:** `13/14`
- **Adaptive-question score:** `6/6`
- **Major misunderstanding(s):**
  - The helper still could not name concrete implementation files because the packet does not yet require a specific candidate lane; it correctly refused to guess.
  - Verification stayed abstract for the same reason.
- **Strongest improvement(s):**
  - Better use of concrete path strings instead of repo names.
  - Better explicit restraint when the docs do not justify a code-file path.
  - Better separation of `tui-vfx-recipes`, `tui-vfx`, and `mixed-signals` ownership.
- **What changed next:**
  - Push the packet toward a concrete candidate lane shape so later helpers can name likely files and verification commands without inventing them.

## Cycle 3

- **Helper id:** `019dbbcd-be54-7eb2-a244-4d7f152ea1a2` (`Ptolemy`)
- **Packet revision summary:** added explicit permission to name the smallest candidate source/test file set when docs support it, plus explicit “do not invent file paths” language.
- **Adaptive questions used and why:**
  - `If the docs support a likely lane, should you name the smallest candidate source/test file set, or wait for more evidence?` — to test whether the helper can now bridge boundary evidence into concrete files.
  - `Should you invent file paths from repo boundaries if the docs do not support them?` — to reinforce non-guessing behavior.
  - `When exact verification commands are not named in the packet, should you state “not yet concrete enough” or guess standard commands?` — to test verification restraint.
- **Fixed-question score:** `13/14`
- **Adaptive-question score:** `6/6`
- **Major misunderstanding(s):**
  - Verification commands were still not concretely named; the helper correctly refused to guess.
- **Strongest improvement(s):**
  - The helper produced a concrete smallest candidate source/test file set from the docs.
  - It kept the blocker lane in `tui-vfx-recipes`.
  - It preserved boundary discipline against invented paths and commands.
- **What changed next:**
  - Nudge the packet to make verification-command expectation stronger so the next helper has a better chance of naming runnable checks.

## Cycle 4

- **Helper id:** `019dbbd0-24b2-7212-a858-244282de2b6e` (`Hilbert`)
- **Packet revision summary:** added explicit guidance to prefer documented validator/test commands and to avoid invented verification alternatives.
- **Adaptive questions used and why:**
  - `If the docs support candidate tests, should you give the smallest runnable command from repo root or remain conceptual?` — to test whether the helper can now produce runnable verification text.
  - `Should you omit runnable verification text if the candidate test set is supported?` — to test whether command text is treated as mandatory.
  - `If the exact command is not in the docs, should you say “not yet concrete enough” or infer the smallest test command family?` — to test guessing restraint on command families.
- **Fixed-question score:** `13/14`
- **Adaptive-question score:** `6/6`
- **Major misunderstanding(s):**
  - The helper still treated verification as partly conceptual because the packet did not yet force a command family.
- **Strongest improvement(s):**
  - It preserved a concrete path set from the docs.
  - It stayed disciplined about refusing to invent command text.
- **What changed next:**
  - Add a stronger prompt to prefer documented validator/test commands, then re-test whether the helper can name a real runnable sequence.

## Cycle 5

- **Helper id:** `019dbbd1-5abf-7733-916f-98e0bf6363eb` (`Mendel`)
- **Packet revision summary:** added explicit wording to prefer the repo’s documented validator/test commands over invented alternatives.
- **Adaptive questions used and why:**
  - `From the repo docs, what exact shell command would you run first to validate the candidate lane?` — to force a concrete runnable check.
  - `Is cargo test -p tui-vfx-recipes or a pipeline-validator --rules --stages ... command the more faithful first check, and why?` — to test validator-first versus generic test-first discipline.
  - `If both apply, what minimal sequence would you give?` — to test whether the helper can order validator and regression checks cleanly.
- **Fixed-question score:** `14/14`
- **Adaptive-question score:** `6/6`
- **Major misunderstanding(s):**
  - None material; the helper converged on a concrete lane and a runnable validation sequence.
- **Strongest improvement(s):**
  - It named a smallest candidate source/test file set.
  - It named a runnable validator command from repo root.
  - It correctly preferred `pipeline-validator --rules --stages ...` over generic `cargo test` as the first check.
- **What changed next:**
  - Freeze the packet’s concrete-lane shape and use the remaining cycles to check stability against broadening back into engine/code edits or repo-wide work.

## Cycle 6

- **Helper id:** `019dbbd3-403f-77b2-a3e5-173e2bcc9f50` (`Russell`)
- **Packet revision summary:** added an explicit `src/` and `tools/` out-of-scope ban unless docs justify crossing out of recipe JSON validation.
- **Adaptive questions used and why:**
  - `Are src/ or tools/ code changes in scope?` — to test whether the helper keeps implementation out of the lane.
  - `Should cargo test --workspace be the primary gate or a secondary backstop?` — to verify validator-first order.
  - `Is pipeline-validator --rules --stages ... still the first gate?` — to check sequencing stability.
- **Fixed-question score:** `14/14`
- **Adaptive-question score:** `6/6`
- **Major misunderstanding(s):**
  - None material; the helper stayed on the filter JSON lane and held the validator-first order.
- **Strongest improvement(s):**
  - Stronger directory and implementation boundary discipline.
  - Stable regression-anchor and validator order.
- **What changed next:**
  - Tighten the packet to keep later helpers pinned to the filter directory only, then test that boundary.

## Cycle 7

- **Helper id:** `019dbbd4-cfd6-7462-af05-2b4ae573c6f0` (`Descartes`)
- **Packet revision summary:** added an explicit ban on `src/`/`tools/` changes and clarified the filter directory as the only recipe write scope.
- **Adaptive questions used and why:**
  - `Are src/ or tools/ code changes in scope?` — to ensure code expansion stays blocked.
  - `Should cargo test --workspace be the primary gate or a secondary backstop?` — to keep validator-first sequencing stable.
  - `Is pipeline-validator --rules --stages ... still the first gate?` — to confirm the same gate holds.
- **Fixed-question score:** `14/14`
- **Adaptive-question score:** `6/6`
- **Major misunderstanding(s):**
  - None material; the helper remained consistent.
- **Strongest improvement(s):**
  - Maintained narrow scope and validator-first ordering.
  - Explicitly rejected `src/` and `tools/` work.
- **What changed next:**
  - Add the regression test as a visible anchor and see whether helpers keep the stable working set pinned to filter JSONs plus the coverage test.

## Cycle 8

- **Helper id:** `019dbbd6-36ce-79f2-b553-356af14f17df` (`Dewey`)
- **Packet revision summary:** added `tests/test_filter_recipe_coverage.rs` as the current regression anchor in the packet hypothesis.
- **Adaptive questions used and why:**
  - `Is tests/test_filter_recipe_coverage.rs the regression anchor?` — to check whether the helper can name the guardrail precisely.
  - `Should anything outside recipes/debug_recipes/filters/*.json be touched?` — to test whether scope remains confined to the filter corpus.
  - `Should cargo test --workspace stay secondary after validator?` — to confirm ordering stability.
- **Fixed-question score:** `14/14`
- **Adaptive-question score:** `6/6`
- **Major misunderstanding(s):**
  - None material; the helper kept the regression anchor and directory boundary stable.
- **Strongest improvement(s):**
  - Stable identification of the coverage test as the guardrail.
  - Stable validator-first sequencing.
- **What changed next:**
  - Tighten the packet one more time to keep later helpers from widening beyond the filter directory.

## Cycle 9

- **Helper id:** `019dbbd7-889c-7671-be50-52873d633ce7` (`Lagrange`)
- **Packet revision summary:** added an explicit exclusion for recipe directories outside `recipes/debug_recipes/filters/`.
- **Adaptive questions used and why:**
  - `Are any recipe directories outside recipes/debug_recipes/filters/ in scope?` — to test directory-boundary discipline.
  - `Is the lane still just filter JSONs?` — to confirm that the helper keeps the lane concrete.
  - `Should the coverage anchor stay tests/test_filter_recipe_coverage.rs?` — to verify the guardrail remains stable.
- **Fixed-question score:** `14/14`
- **Adaptive-question score:** `6/6`
- **Major misunderstanding(s):**
  - None material; the helper stayed within the filter recipe lane.
- **Strongest improvement(s):**
  - No expansion to other recipe directories.
  - Stable regression anchor and validator gate.
- **What changed next:**
  - Final cycle checks whether the helper can hold the minimum stable working set: filter JSONs plus the coverage test.

## Cycle 10

- **Helper id:** `019dbbd8-a0f3-7862-b81d-f8ef2d110c49` (`Pasteur`)
- **Packet revision summary:** added a “minimum stable working set” cue: filter JSONs plus the coverage test.
- **Adaptive questions used and why:**
  - `Is the minimum stable working set the filter JSONs plus the coverage test?` — to test whether the helper retains the narrow working set.
  - `Should anything outside that stable working set be touched?` — to ensure no widening sneaks in at the end.
  - `Should validator-first sequencing still hold?` — to verify the final order stays stable.
- **Fixed-question score:** `14/14`
- **Adaptive-question score:** `6/6`
- **Major misunderstanding(s):**
  - None material; the helper kept the minimum working set intact.
- **Strongest improvement(s):**
  - Strong, stable boundary recall.
  - Strong validator-first ordering.
  - Clear articulation of the coverage test as the regression anchor.
- **What changed next:**
  - None; the packet converged on a stable, concrete lane.
