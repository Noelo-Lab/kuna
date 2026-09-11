## The problem

`sub_140001740` in `phantomgate.exe` writes both halves of `v12` and repeatedly
uses its address, but kuna emitted no declaration for `v12`. The model already
carried the local, its type, stack offset, and use sites; only P9 dropped the
declaration.

## The fix

- Require a claimed whole scalar storage sibling to be non-constant.
- Let partial storage highs reach the existing symbol-keyed declaration
  collapse when the only full-width sibling is an address reference.
- Preserve actual whole-storage suppression and its duplicate-declaration
  protection.

This is a strict correction to the existing declaration predicate and adds no
option.

## The tests

The helper-level unit test pins the discriminator directly. A raw-byte stage
test covers both partial writes and the address expression, while a deterministic
DWARF ELF promotes the original anchored acceptance regex into the CLI corpus.

An exact-build differential covered all 274 XML files and every 1,100
decompilable function in the witness binary. Only the minimizer and two witness
functions changed, and every hunk adds one declaration without moving or
changing a body statement.

Final gates: 675/675 core assertions, 783/783 stage assertions, 130/130 CLI
probes, the full Cargo workspace suite, spec/catalog/counter checks, and the 1/1
acceptance probe. Mergecheck reported zero rejects.

Timing is not claimed because concurrent host workloads made wall-clock samples
non-diagnostic; the fixed path adds one boolean check to an existing predicate
and does not change its complexity.

🤖 Generated with [OpenAI Codex](https://openai.com/codex/).
