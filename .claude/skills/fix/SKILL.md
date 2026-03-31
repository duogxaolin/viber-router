---
name: fix
description: Investigate and fix a bug. Explore root cause, assess scope, then implement with optional spec creation.
---

You are investigating and fixing a bug. This command helps you trace the root cause, assess the fix scope, and decide on the best implementation path.

BEFORE PROCEEDING: You MUST use the Skill tool to invoke "osf-skill-explore-mode". This loads the shared explore mode behavior (stance, verification, workflow, subagent protocols, OpenSpec awareness, guardrails) that this command depends on. Do not proceed without loading it first.

---

## What You Might Do

**Investigate a problem (bug, unexpected behavior, "something's wrong")**
- Trace, don't theorize — read actual code, follow execution flow step by step
- Form hypotheses then verify — "I think the issue is X" → read the code → confirm or reject
- Find root cause, not symptoms — when you find where it breaks, ask "why does it break here?" and keep digging
- 5 Whys — each answer becomes the next question until you hit the real cause
- Don't stop at the first plausible explanation — verify it in code before presenting it

**Explore the problem space**
- Feynman Echo — restate the bug in the simplest possible language, then ask user to confirm or correct
- Ask clarifying questions that emerge from what they said
- Challenge assumptions
- Reframe the problem

**Investigate the codebase**
- Map existing architecture relevant to the bug
- Find integration points
- Identify patterns already in use
- Surface hidden complexity

**Compare fix options**
- Brainstorm multiple fix approaches
- Build comparison tables
- Sketch tradeoffs
- Recommend a path (if asked)

**Visualize**
```
┌─────────────────────────────────────────┐
│     Use ASCII diagrams liberally        │
├─────────────────────────────────────────┤
│   Causal chains, state machines,        │
│   data flows, dependency graphs,        │
│   before/after comparisons              │
└─────────────────────────────────────────┘
```

**Research external knowledge**
- When discussion involves technology choices, best practices, or security concerns → delegate to osf-researcher

**Look up API documentation**
- When discussion needs precise API usage → delegate to osf-researcher for web research

**Surface risks and unknowns**
- Identify what could go wrong with the fix
- Find gaps in understanding
- Suggest spikes or investigations

---

## Stress-test Questions

Resolve these before ending discovery. Self-answer by exploring the codebase. Only surface items to the user that are genuinely ambiguous or require a personal/team style choice:

1. Regression risks:
   "Could this fix break anything else:
    A. No — isolated change
    B. Maybe — need to check related code
    C. ★ Likely — need comprehensive testing
    D. Khác/Other: ___"

2. Edge cases:
   "For [input/data], edge cases to handle:
    A. Empty/null — show empty state
    B. Too long — truncate at N chars
    C. Special characters — sanitize
    D. ★ All of the above
    E. Khác/Other: ___"

3. Test strategy:
   "Test level needed:
    A. Unit tests for the fix
    B. Unit + integration tests
    C. ★ Unit + integration + regression tests
    D. Khác/Other: ___"

4. Architecture decisions:
   "Error handling strategy for this fix:
    A. Throw exceptions, catch at boundary
    B. Result/Either pattern (no exceptions)
    C. Error codes + error handler
    D. ★ Follow existing project pattern: [detected pattern]
    E. Khác/Other: ___"

---

## Zero-Fog Checklist (additions)

- [ ] Root cause is identified and verified in code (not just a symptom)
- [ ] Fix approach is specific enough for a verifier to objectively check
- [ ] All edge cases are explicitly named (not "handle edge cases" — which ones?)
- [ ] Error paths are defined for every operation that can fail
- [ ] Regression risks identified and mitigation strategy defined
- [ ] Test strategy decided (unit? integration? regression? which functions need edge case tests?)

---

## Extra Subagents

| Subagent | When to Use |
|----------|-------------|
| osf-uiux-designer | Fix involves UI changes |

The following is the user's request: