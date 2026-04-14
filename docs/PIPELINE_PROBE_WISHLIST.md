<!-- <FILE>docs/PIPELINE_PROBE_WISHLIST.md</FILE> - <DESC>Prioritized wishlist for the AI-native probe and validator toolchain</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Tonight wrap-up wishlist for finishing the dream recipe debugger</WCTX> -->
<!-- <CLOG>NEW: Capture the remaining high-value debug-tool ideas, their priority, and the diminishing-returns assessment so future sessions can continue from a canonical roadmap instead of recreating the same judgment calls</CLOG> -->

# Pipeline Probe Wishlist

This is the prioritized wishlist for making `pipeline-probe`, `recipe-probe`, and `pipeline-validator --probe` the best possible recipe-authoring and debugging workstation for AI and humans.

## Already strong enough for serious debugging

The stack already supports:
- structured frame / timeline / diff output
- per-cell unified traces
- focused-cell root-cause inspection
- runtime binding visibility
- diagnostics
- stage / effect / lifecycle analysis
- motion span analysis
- SQL querying for the major debug surfaces

That means the remaining work is no longer about basic visibility. It is mostly about speed, ergonomics, and better automatic triage.

## Priority 1 — still clearly worth doing

### 1. Motion quality heuristics
Add generalized timeline heuristics for:
- snapback detection
- asymmetry / dead-zone detection
- cadence / offscreen-phase checks
- direction reversals that violate the intended motion contract

Why it matters:
- motion bugs are still among the hardest failures to detect quickly
- the timeline + trace + SQL substrate is already strong enough to support these checks

### 2. Text quality classification
Extend text diagnostics beyond presence and similarity into explicit categories:
- leading-glyph loss
- trailing truncation
- internal corruption
- severity buckets for readability degradation

Why it matters:
- these failures are common while authoring expressive recipes
- the existing normalized-message and LCS groundwork already exists

### 3. Audit synthesis / triage summary
Add a higher-level summary surface that answers:
- what is most likely wrong first?
- which stage/effect is the best next suspect?
- which next SQL query / focused-cell probe should be run?

Why it matters:
- the primitives are now powerful, but this would make the system much faster to drive under pressure

## Priority 2 — useful, but starting to enter diminishing returns

### 4. Non-compositor instance identity for all recipe-side elements
Compositor stages now have stable ordinals (`Dim#1`, `KittScanner#1`, etc.).
Continue pushing the same rigor into any remaining recipe-side paths that can still collapse multiple same-name elements.

### 5. More motion-shape semantics
Possible future checks:
- circular sweep verification
- spiral/orbit coverage characteristics
- path-following drift vs authored path

### 6. Visual scoring beyond diagnostics
Potential long-term additions:
- confidence/severity scoring for aesthetics
- ranking “visually valid but compositionally weak” outcomes

## Diminishing-returns assessment

The largest structural visibility gaps are already closed.

From this point on, new work is mostly about:
- better heuristics
- faster workflows
- clearer summaries

That work is still valuable, but the returns are no longer as dramatic as the earlier probe/trace/SQL/root-cause milestones.

## Recommended next-session order

1. Motion quality heuristics
2. Text quality classification
3. Audit synthesis / triage summary
4. Any remaining instance-identity cleanup
5. Nice-to-have aesthetic scoring only if the earlier items are done

<!-- <FILE>docs/PIPELINE_PROBE_WISHLIST.md</FILE> - <DESC>Prioritized wishlist for the AI-native probe and validator toolchain</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
