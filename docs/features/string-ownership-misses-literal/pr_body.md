## The problem

`kuna strings` and `kuna xrefs` lose every reference a switch case body forms: the
reference walk's successors are an instruction's static branch targets plus
fall-through, and a computed jump has neither, so the walk stops dead at the
dispatch and the case bodies are never decoded. The fixture added here is the
reduction — a `dispatch` whose four cases each push a literal — and on the
unpatched tree four of its five literals belong to nobody:

```console
$ kuna strings decompiler/crates/kuna-analysis/tests/fixtures/switchtable_i386 \
      --filter 'switch case alpha reached' --json
      "address_hex": "0x101010",
      "text": "switch case alpha reached",
      "section": ".rodata",
      "xrefs_count": 0,
      "functions": []
```

Its `beta`/`gamma`/`delta` siblings answer the same. The fifth literal, in the
default arm, reports `xrefs_count: 1` and `dispatch` — it is reached by the `JA`
and not by the table, which is the whole difference. The reported instance is an MSVC window procedure
(crackmes.one/60be2ad433c5d410b8842c95) dispatching `JMP dword ptr [EAX*0x4 +
0x4017c4]` at `0x40143b`: `xrefs --from` its entry filed 61 references that stop
there and resume at `0x401732`, leaving 758 bytes of the function dark and
`"Product Already Registered"` with `xrefs_count: 0`.

## The fix

- `listing/kuna_switchtable.rs` reads the table a computed jump indexes and
  queues its entries as walk successors.
- The base is the address the dispatch *materializes* — the `Data` reference the
  constant scan already files — rather than a pattern match on `JMP dword ptr
  [reg*n + imm]`. A base held in a data-space varnode is deliberately not a
  candidate: that is an import veneer's slot (`jmp qword ptr [__imp_X]`, an ELF
  PLT entry), which the scan files as a `Read`, so a veneer is never read as a
  one-entry table of whatever its unrelocated slot holds.
- Entries are read through the same read-only image dereference `poolref` uses,
  and admitted only while pointer-aligned and inside the **same executable
  section** as the dispatch. The first word that is not both ends the table,
  which is what stops the scan at the `cc cc cc cc` padding after the last case.
  A run under two entries is not a table; the ceiling is 1024.
- The entries become successors of the **dispatching function**, not function
  entries of their own, so an agent asking who prints a message gets the handler
  and not an address inside it.

## The tests

`verify_switchtable.rs` runs the above over two fixtures — `.long` and `.quad`
strides, `PUSH imm32` and RIP-relative `LEA` — and pins the stop rule: the
dispatch jumps to exactly the four table entries and nothing past them. Six unit
tests cover the admission rules directly, and
`tests/cli/string-ownership-misses-literal.json` is the promoted probe.

Swept 76 crackme and decbench images before/after: 3 changed, 20 references
gained, **zero lost**. On `Cube.exe` three strings also moved off an address
inside `inflate`'s own extent onto `inflate` itself, which the walk could not
reach past the `state->mode` switch. `kuna strings` on a 7 MB PE is unchanged
(22.9 s → 23.1 s, median of 3).

`make test` 675/675 PARITY OK · `make test-stages` PARITY OK · `make rust-test`
green · `make check-spec` OK · `make test-cli` 53/53 · `kuna catalog --check` OK.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
