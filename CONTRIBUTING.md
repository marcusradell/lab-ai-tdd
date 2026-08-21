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

## 2. One behavior per test

A test asserts **one** behavior. If a test has several assertions covering
different observable properties, it's several tests wearing a trenchcoat —
split it and drive them from separate red/green cycles.

Concretely: `single element sums to itself` and `single element's min is
itself` are two items on the test list, two red steps, two commits. Don't
bundle them just because one implementation happens to satisfy both.

The same rule applies to data structures: don't add fields to a struct
because you know you'll need them later. Each field appears because a
failing test demanded it.

## 3. Division of labor

- **Claude** — thinking partner. Use for the test list, writing _one_ test
  at a time, discussing the minimal implementation, and proposing refactors
  once green.
- **Copilot CLI** — the hands. Runs the test suite, handles git operations,
  scaffolding, diffs. Executes what you and Claude just agreed on.

Only one tool proposes code per step. Don't let both design in the same
step — you'll get conflicting implementations.

### Alternating cycles

Cycles alternate between the human and the AI. The AI does one full
red/green/refactor cycle, then **the human does the next one** — writing the
test and the implementation themselves.

On the human's cycles the AI switches to reviewer:

- read the test that was just written and say whether it's really one
  behavior, and whether it fails for the right reason;
- read the implementation and say whether it's the minimal step or whether
  it jumped ahead;
- propose a refactor if the code asks for one — and say plainly when it
  doesn't.

The AI does not write code during the human's cycle. It reviews: reads the
test, reads the implementation, and proposes a refactor when the code asks
for one. This keeps the human's hands in the loop and stops the session from
turning into supervised code generation.

The human runs the suite on their own cycles. The AI does not re-run
`cargo test` to confirm red or green for a human step, and does not ask for
confirmation of output the human is already watching — it waits until the
human hands over the code for review.

## 4. Prompts to reuse each cycle

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

## 5. Atomic commits, never on red

**Never commit a failing test.** Every commit on the branch must have a
green suite — the red state lives in your working tree, not in history. A
committed red test breaks bisect, breaks CI, and breaks anyone who checks
out that commit.

So the test and the minimal implementation that makes it pass land in the
_same_ commit, once you've seen it go red and then green:

| Stage    | Commit?                | Message example        | When                                          |
| -------- | ---------------------- | ---------------------- | --------------------------------------------- |
| Red      | no — working tree only | —                      | test written, confirmed failing for the right reason |
| Green    | yes                    | `feat: sum a single element` | test + minimal implementation, confirmed passing |
| Refactor | yes, separately        | `refactor: simplify Y` | only if a refactor happened, only after green  |

Refactors stay in their own commit so structural changes are separate from
behavioral ones — Beck's "tidy first" principle — and the log stays a
readable TDD trail, useful for bisecting later.

## 6. Guardrails

- Always see the actual failing output before implementing.
- Always see the actual passing output before committing.
- Never commit while the suite is red — red is a working-tree state.
- Never trust a claimed test result — verify in the terminal yourself.
- If scope creeps (extra tests, extra handling, unrequested refactors),
  stop the cycle and re-anchor on the single next item from the plan.
- No production code without a failing test that demands it — including
  scaffolding, module skeletons, and struct fields.
- One behavior per test; multiple assertions on different properties is a
  smell, not a shortcut.
- Alternate cycles between human and AI; on the human's cycle the AI reviews
  and suggests refactors instead of writing code.
- On the human's cycle the human runs the suite; the AI doesn't re-run it to
  confirm red or green.

## 7. Session memory lives here

Any workflow preference the AI stores in its own memory must also be written
into this file. Memory is invisible to everyone else on the repo and to
future tools; this document is the single readable source of truth for how
the session runs. If a convention is worth remembering, it's worth a line
here.
