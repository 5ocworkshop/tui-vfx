<!-- <FILE>steering/SUBAGENT-GROUNDING.md</FILE> - <DESC>Mandatory grounding pass for subagents before work packets are assigned</DESC> -->
<!-- <VERS>VERSION: 0.1.4</VERS> -->
<!-- <WCTX>Separate subagent onboarding from work-packet execution and prohibit helpers from creating repo/crate copies as part of packet work.</WCTX> -->
<!-- <CLOG>0.1.4: PATCH — retarget grounding examples and copy prohibitions from the abandoned copied-crate path to tui-vfx-compost work.</CLOG> -->

# Subagent Grounding

This document is the mandatory first assignment for a newly launched subagent.
It must be completed before the agent receives a work packet or performs
implementation work.

Grounding is not a ceremonial checklist. The purpose is to make sure the agent
understands the project's headline goals, philosophy, coding rules, repository
boundaries, OFPF workflow, and task-specific vocabulary before any edits are
authorized.

Agents must read every required grounding document completely before work can
begin. Skipping a full read, relying on headings, summaries, excerpts, prior
memory, or "key parts" is a grounding failure. Do not report grounding as
complete until each required document has been read end to end. If a required
document cannot be read completely, stop and report the missing, unreadable, or
incompletely read document as a blocker.


## Version precision rule

Version labels are scope. Do not collapse `v3.1` work into generic `V3` wording.
If a packet says `v3.1`, reports, summaries, docs, and implementation comments
must say `v3.1` or `V3.1` unless they are explicitly referring to a historical
V3 artifact. Treat `V3`, `V3.1`, and V2 as different surfaces with different
authority. A grounding or packet report that describes v3.1 tui-vfx-compost
work as "V3 pipeline" work is imprecise and must be corrected before work
continues.

## Stage 1 — Overall project goals and philosophy

Read first:

1. `/usr/projects/tui-vfx/steering/INTENTIONS.md`

Read additional intentions files only when that repository may be in scope:

- `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
- `/usr/projects/mixed-signals/steering/INTENTIONS.md`
- `/usr/projects/gt-design/steering/INTENTIONS.md`

Minimum requirement: the current repo `INTENTIONS.md` must be read in full, end
to end. It must not be skipped, skimmed, sampled, replaced by a summary, or
reduced to headings plus selected sections.

## Stage 2 — Coding standards

Read the global standards that govern normal code work. Each listed standard is
required and must be read completely before work can begin:

1. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
2. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
3. `/usr/projects/global_prompts/standards/60_file_centric_execution.md`
4. `/usr/projects/global_prompts/standards/65_subagent_orchestration.md`
5. `/usr/projects/global_prompts/standards/90_recycle_bin.md`

## Stage 3 — OFPF tools and practice

Read completely:

1. `/usr/projects/tui-vfx/steering/OFPF-TOOLS.md`

Then run high-value read-only OFPF practice commands in `/usr/projects/tui-vfx`.
Use commands that teach repository shape, symbol lookup, textual lookup, local
context, and impact analysis. Do not spend the practice slot on `ofpf-status`
unless a command fails and daemon health becomes the question.

Recommended practice sequence:

```bash
ofpf-orientation --root /usr/projects/tui-vfx
ofpf-inspect --root /usr/projects/tui-vfx crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs
ofpf-content --root /usr/projects/tui-vfx "tui-vfx-compost" --files-with-matches
ofpf-defs --root /usr/projects/tui-vfx render_pipeline
ofpf-callers --root /usr/projects/tui-vfx render_pipeline
ofpf-blast --root /usr/projects/tui-vfx crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs --why
```

If a command is unavailable in the local OFPF version, run the nearest
equivalent from `OFPF-TOOLS.md` and report the substitution.

## Stage 4 — Vocabulary and public wording

Read when the eventual work may touch public vocabulary, schema wording,
descriptor fields, docs, recipe text, report wording, rustdoc, or user-facing
terms:

1. `/usr/projects/tui-vfx/docs/VOCABULARY.md`

If the future packet clearly cannot touch public vocabulary or wording, say that
you skipped this file and why in the grounding report. If this file is required
for the anticipated scope, it must be read completely; a partial read is a
blocker, not acceptable grounding.

## Workspace and copy prohibition

A leader-provided git worktree is already the isolated checkout. Do not create
a copy of `/usr/projects/tui-vfx`, `tui-vfx-compositor`, or
the abandoned copied-crate path, and do not create nested clones or nested worktrees
inside an assigned worktree. Work packets should give exact write-scope files;
a whole project or crate copy is not an acceptable substitute for scope control.

## Grounding report

After completing grounding, stop. Do not start implementation. Report:

- documents read completely, with any conditional documents skipped and why;
- explicit confirmation that no required document was skimmed, sampled, or
  substituted with headings, summaries, excerpts, or prior memory;
- the project's key goals and constraints most relevant to future work;
- repository ownership boundaries;
- coding/TDD/OFPF rules you must follow;
- exact OFPF practice commands run and what each taught you;
- any missing or unreadable documents;
- biggest compliance risks you need the eventual work packet to constrain.

End the report with:

```text
READY FOR WORK PACKET
```

Only after the leader receives and accepts this report should a work packet be
assigned.

<!-- <FILE>steering/SUBAGENT-GROUNDING.md</FILE> - <DESC>Mandatory grounding pass for subagents before work packets are assigned</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.4</VERS> -->
