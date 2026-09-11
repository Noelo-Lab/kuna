## The problem

“CALL/POP pointer helper decodes embedded DLL names as instructions” appeared in 1 round-12 instance. The helper should return its inline `kernel32.dll` pointer, but the fall-through lifts the string bytes as port I/O and arbitrary stores.

```sh
decompiler/target/release/kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/callpopret_i386 dllname --option callpopret off
```

```c
getptr();
*v8 = in(v7);
v8[1] = in(v7);
v8[2] = in(v7);
```

## The fix

- Add the default-on P2 `callpopret` decision, which proves the callee discards the pushed return address and returns through the word above it.
- Rewrite a proven site from CALL to BRANCH so existing flow and dataflow recover `return "kernel32.dll";`.
- Share the bounded target decode with `calltrampoline`, avoiding any increase in pinned Ghidra `getPcode` traffic.
- Make the merge guard compare complete `phases.toml` table rows, so a distinct option is not mistaken for a duplicated row.

## The tests

The promoted acceptance probe passes and requires the returned string, not only removal of `in(...)`. Focused coverage is 17 raw-pcode cases plus the two-pass stage fixture; the corpus sweep changed only the witness (1/403 comparable images, 0 wrong changes), and timing was +4.97% within the 5% budget.

`make test` 675/675; `make test-stages` 776/776; `make test-cli` 127/127; workspace, spec, catalog, and acceptance gates green.

🤖 Generated with [OpenAI Codex](https://openai.com/codex/).
