# TDD Pairing Workflow — Copilot CLI + Claude

A reference for practicing Kent Beck–style TDD while pairing with AI tools.
Keep this open next to the repo during a session.

## Core loop

Red → Green → Refactor. One test at a time. Smallest possible step.
The loop itself doesn't change with AI in the room — the discipline is in
resisting the model's default toward bulk output.

## 1. Plan before writing any test

Have a short planning exchange (with Claude) to produce an ordered list of
small behaviors for the feature — not code, just the list. This is the
shared roadmap so scope doesn't drift mid-session.

Before touching a test file, say explicitly: **"We're only working on item
N right now."** This single habit prevents most bulk-test drift.

## 2. Division of labor

- **Claude** — thinking partner. Use for the test list, writing _one_ test
  at a time, discussing the minimal implementation, and proposing refactors
  once green.
- **Copilot CLI** — the hands. Runs the test suite, handles git operations,
  scaffolding, diffs. Executes what you and Claude just agreed on.

Only one tool proposes code per step. Don't let both design in the same
step — you'll get conflicting implementations.

## 3. Prompts to reuse each cycle

**Red:**

> Write exactly one failing test for [behavior]. Do not write the
> implementation. Do not write additional tests.

Run it. Confirm it fails for the _right_ reason — read the actual output,
don't take the model's word for it.

**Green:**

> Now write the minimal code to make only this test pass. Don't handle
> cases we haven't tested yet.

Run it. Confirm it passes — again, check the real terminal output.

**Refactor:**

> Propose a refactor if one is needed. If nothing needs cleaning up, say
> so — don't refactor for its own sake.

If the AI hands you multiple tests or jumps ahead to implementation, stop
and flag it like a code review comment — don't accept it just because it
happened to work.

## 4. Atomic commits, one per loop stage

| Stage    | Commit message example         | When                                          |
| -------- | ------------------------------ | --------------------------------------------- |
| Red      | `test: add failing test for X` | test written, confirmed failing               |
| Green    | `feat: make X pass`            | minimal implementation, confirmed passing     |
| Refactor | `refactor: simplify Y`         | only if a refactor happened, only after green |

This keeps behavioral changes (red/green) separate from structural changes
(refactor) in the git history — Beck's "tidy first" principle — and makes
the log itself a readable TDD trail, useful for bisecting later.

## 5. Guardrails

- Always see the actual failing output before implementing.
- Always see the actual passing output before committing.
- Never trust a claimed test result — verify in the terminal yourself.
- If scope creeps (extra tests, extra handling, unrequested refactors),
  stop the cycle and re-anchor on the single next item from the plan.
