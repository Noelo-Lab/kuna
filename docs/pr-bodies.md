# PR bodies — three short sections, and lead with the repro

The reader does not yet know what is broken, so lead with the repro, not the
mechanism, and keep it easy to read.

**The structure, for a bug fix, in this order:**

1. **The problem — at most two sentences, then a runnable example.** Say what
   is wrong in plain terms, then show it: a command anyone can paste, with its
   actual (wrong) output in a fenced block. Trim the output to the part that
   carries the bug, but do not paraphrase it. The example is the section — the
   prose is only there to say what to look at.
2. **The fix — a few bullets.** What the change does, not a narration of the
   diff. One bullet per idea; mention a design choice only where a reviewer
   would otherwise ask "why not the obvious way?".
3. **The tests — a few lines at most.** Which cases were added and which one
   fails without the fix. Numbers only if they are the evidence (suite
   before/after, "no existing expectation moved"); never a paragraph of them.

**Length is the check.** If the body is longer than roughly a screen, it is
wrong regardless of accuracy — cut, do not reword. Detail that feels too
valuable to drop belongs in `notes/`, which is where it should have been
anyway.

Other rules for bodies:

- **Never open with the internal mechanism.** "`X()` falls through to `Y()`,
  so `Z` lands outside the enum" is section 2 material at best, and usually
  belongs in the code or in `notes/`.
- **The example must be one the reviewer can run**, against the unpatched tree.
  A fixture path they do not have, or a command needing our harness, is not a
  repro — reduce it to `cstool` / `r2 -qc` / `rasm2` on bytes.
- **No LLM register.** Bold-per-clause, "Root cause:", "Note that", em-dashed
  asides stacked three deep, and restating the same fact in two registers all
  read as generated. Write the sentence once, plainly.
- **Never mention our internal process** — milestone codes, review rounds,
  agents, gate runs, how many `/code-review` passes it took. PR bodies are
  public.
- **Fill in the repo's template** if it has one but the template's headings do
  not excuse a long body.
- For a **feature or refactor** rather than a fix, section 1 becomes "what this
  makes possible" plus a before/after of the visible behaviour — same budget,
  same order: the observable thing first, mechanism second.
