---
name: osf
description: Launch any kit skill by name. Usage: /osf [skill] [args]
---

Available skills: feat, fix, chore, refactor, perf, docs, test, ci, docker, git, autopilot, research, browser, explain, analyze, apply, archive, proposal, verify, uiux-design, setup

Supporting subagents (used internally by skills):
- osf-analyze — Structural codebase analysis (dependencies, blast radius, call chains) via GitNexus + codebase-retrieval
- osf-apply — Implement tasks from spec or conversation plan
- osf-archive — Archive completed change to openspec/changes/archive/
- osf-proposal — Create spec (proposal, design, tasks) for implementation
- osf-researcher — Web research (technical docs, best practices, comparisons, security advisories)
- osf-uiux-designer — UI/UX design analysis and reports
- osf-verify — Verify implementation matches spec

Use the Skill tool to invoke "$0".

If the user provided additional arguments beyond the skill name, include them as context for the invoked skill.

ARGUMENTS: $ARGUMENTS