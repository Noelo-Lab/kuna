## The problem

The faithful PE's `sub_1400012f0` has three constant-result exits that share an
MSVC `/GS` cookie check. Even with that checker asserted as locked `void`, kuna
returned an unassigned EAX local instead of the machine's zero and one values.

## The fix

- Propagate pending backedges through nested cookie phis so the exact `/GS`
  recognizer can reach the real scramble seed.
- Keep seedless SCCs, unknown entries, conflicting offsets, writes, explicit
  effects, and incomplete generic callee proofs conservative.
- Reuse exact-site locked-void ABI preservation and retain every checker call.
- Render only the exact adjacent literal/checker/return triplet as a literal
  return when the checker line's markup identifies that exact P7-marked call
  occurrence and the whole line is one standalone call statement. Move the
  literal assignment origin onto the synthesized return's provenance.

## The tests

The original 16 KB witness is promoted byte-for-byte with its exact SHA-256.
The default probe requires both literal exits and the visible checker; the
`calleeretpreserves off` control restores the undefined-EAX symptom. Unit tests
cover the nested-phi positive and seedless, unknown-entry, conflicting-offset,
different-local, non-literal, intervening-statement, trailing-text, and
same-name-but-unmarked-call negatives, plus resolved return provenance.

Final rebased gates: 675/675 core assertions, 813/813 stage assertions, 153/153
CLI probes, the full Cargo workspace suite and doctests, lenient and strict spec
checks, catalog check, and the 1/1 acceptance probe. The canonical median-of-five
timing is 536.39 ms on versus 556.65 ms off (-3.64%) on the faithful function.

🤖 Generated with [OpenAI Codex](https://openai.com/codex/).
