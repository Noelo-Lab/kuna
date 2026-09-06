## The problem

`--assert 'param <func>::<i> <storage> <decl>'` documents a `<func>::` qualifier
that names the function the directive describes. The qualifier was dropped, so
the directive was applied to whichever function was being decompiled — the
caller — and the function it named was left alone, with no rejection.

```console
$ kuna decompile ./a.out authenticate --assert 'param open::0 RDI char *pathname'
unsigned long authenticate(char *pathname,char *a1)
...
  v4 = open(a0,0);
```

`authenticate`'s own first parameter has been renamed and retyped to `open`'s
declared one; the call to `open` is untouched. Exit code 0, nothing on stderr.
(`a.out` here is `decompiler/crates/kuna-analysis/tests/fixtures/fauxware`.)

## The fix

- `param` and `return` qualified with another function now state **that
  function's prototype**, which is the thing a caller needs about a callee.
  The parked `PrototypePieces` is the only channel to a caller, and it carries
  types only, so declared storage rides alongside in a new `input_storage`
  (the input-side twin of the `output_storage` `map return` already parks) and
  `FuncProto::set_pieces` re-applies it after the model assignment. That is
  what makes a non-default convention statable: `param f::0 ECX …` renders the
  ECX argument, not the first stack slot.
- The console gains the same qualifier on `map param` / `map return`, so a
  hand-driven `decomp_dbg` session and the generated script say the same thing.
  Neither operand can hold a `::` of its own (a decimal index, a machine
  address), so the spelling is unambiguous.
- `comment`, `flow`, `name` and `type` describe the inside of one function body
  and have no cross-function reading. Qualified with a function the run did not
  decompile they are now **rejected**, with the reason on stderr and a non-zero
  exit under `--assert-strict`, instead of landing on the selected function.
- Unqualified directives, and a directive qualified with the selected function,
  lower to exactly the console lines they did before.

```console
$ kuna decompile ./a.out authenticate --assert 'param open::0 RDI char *pathname'
unsigned long authenticate(char *a0,char *a1)
...
  v4 = open(a0);
```

## The tests

`tests/cli/qualified-parameter-assertions-modify.json` is the case above; both
its expectations fail before the fix (the caller's signature carries `pathname`,
and the call is still `open(a0,0)`). `verify_assertplane` gains an end-to-end
pair that decompiles the caller twice with the same slot and type but `%RDI` and
then `%RSI`, so a lowering that dropped the storage could not tell the two runs
apart. `assertdecl` pins the lowering and the rejection for all six qualifiable
directives.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
