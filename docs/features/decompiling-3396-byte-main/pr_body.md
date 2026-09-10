## The problem

An x64 PE's `.pdata` says which byte ranges are one function's body, and kuna
starts functions in the middle of them anyway. `nikos_crack_me.exe` declares one
43,152-byte body at `[0x140023350, 0x14002dbe0)`; the gap walk decodes what
recursive descent could not reach and mints three functions inside it, and the
fall-through bound then truncates the real function's flow when it reaches one:

```
$ kuna functions nikos_crack_me.exe | grep -E '14002(3e34|b324|b6a8)'
0x140023e34	sub_140023e34
0x14002b324	sub_14002b324
0x14002b6a8	sub_14002b6a8

$ kuna decompile nikos_crack_me.exe sub_140023350 | grep funcboundflow
        v74 = 0x140024921; // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
        if ((v21 + (v83 - 1) & 1) != (v21 + (v83 - 1) & 1)) { // warn: funcboundflow: ...
```

The image's own exception table contradicts all three: none is a
`BeginAddress`, and every one is strictly inside the record that starts at
`0x140023350`.

## The fix

- New P1 pass `pdatainterior` (`on|off`, default on): report the
  `RUNTIME_FUNCTION` bodies that describe exactly one function, and let the
  commit boundary reject a discovered entry strictly inside one.
- It reports on the same `fde_bodies` channel `fdeinterior` (DIV-61) already
  uses for `.eh_frame`, so the filter that covers the deferred gap walk on ELF
  now covers it on PE without a second filter.
- A range is used only if it holds no other named function start and no other
  record's `BeginAddress`, and does not overlap the range kept before it — the
  `.pdata` reading of the guard that keeps `fdeinterior` off the linker's
  single whole-PLT FDE. An entry *at* a `BeginAddress` is always kept.
- x86/x64 PE only. The 8-byte ARM/ARM64 record carries no `EndAddress` and an
  image with no exception directory vouches for nothing, so the pass abstains
  rather than guess.

## The tests

Seven unit tests over the existing `pe_reloctable_x86_64.exe` fixture: the
interior label is rejected, the `BeginAddress` and the exclusive `EndAddress`
are kept, both eligibility guards reject a range that swallows another
function, and a zeroed exception directory, an ARM64 machine word and a non-PE
each abstain. Swept over 201 x86/x64 PE images: 8 inventories move, 104 starts
dropped, **0 added**. On the witness, 438 → 424 entries and both truncation
warnings are gone; `--option pdatainterior off` restores the previous set
exactly.

Gates: `make test` 675/675 PARITY OK, `make test-stages` 749/749 PARITY OK,
`make rust-test` green, `make check-spec` green, `make test-cli` 119/119,
`kuna catalog --check` OK.

Side effect, measured because this branch is the `decompiling-3396-byte-main`
perf need: removing the false starts takes that witness **10,432 → 9,988 ms
median** (ABBA-balanced, 6 blocks, paired mean −4.16% ± 1.45, 6/6 blocks a win).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
