## The problem

“Call-result folding duplicates a random-number call inside the uniqueness loop”
was observed in one RE-friction instance. The binary calls once, but kuna printed
the call in both the loop comparison and the later store.

```console
$ decompiler/target/release/kuna decompile \
    decompiler/crates/kuna-analysis/tests/fixtures/callretmulti_x86_64 \
    sub_140002930
if (v3[v1] == (unsigned short)helper()) goto label_40100d;
v3[v2] = (unsigned short)helper();
```

## The fix

- Keep a derived multi-use value explicit when its printable expression reaches
  a call that `foldcallret` would inline, preserving one evaluation.
- Follow the printer's operand selection and stop at explicit inputs, so the
  check covers same-block and cross-block fan-out without widening unrelated
  expression trees.
- Preserve the existing option and its safe single-use behavior; this corrects
  the documented `foldcallret` contract.

## The tests

The promoted acceptance probe passes, and `kuna-callretmulti.xml` covers the
loop witness plus a same-block two-sink minimizer. The 456-binary sweep found no
wrong-output hunk: 259 changed functions are classified in the feature record,
with every affected callee retained.

The existing `foldcallret` catalog prose and legacy CALLEESCRATCHBODY assertion
names remain byte-for-byte unchanged; the legacy assertions now use stronger
bodies that require one explicit probe result and reuse.

Final rebased gates: 675/675 core assertions, 779/779 stage assertions, 128/128
CLI probes, full Cargo workspace suite, spec check, catalog check, and the 1/1
acceptance probe; mergecheck reports 0 rejects against `origin/main`. Median
option-on timing is 129.19 ms versus 129.43 ms off
(-0.19%).

🤖 Generated with [OpenAI Codex](https://openai.com/codex/).
