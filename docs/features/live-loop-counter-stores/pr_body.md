## The problem

A `for` loop initialises and tests one variable but assigns its increment to
another, so the induction variable is never updated and the loop as written
cannot terminate. Five loops in one function come out this way:

```
$ kuna decompile ObfuscationFiesta.exe sub_140002530
      for (v55 = 0; v55 <= 4; v7 = v55 + 1) {
        v46 = (v46 ^ v55 * 0x17) + 0x11;
      }
```

The counter is a frame slot that is really written. The store is right there in
the disassembly — paste the bytes into any x86-64 disassembler:

```
$ cstool -a 0x1400035f4 x64 eb108b842484000000ffc08984248400000083bc2484000000057d19
 0x1400035f4  jmp  0x140003606
 0x1400035f6  mov  eax, dword ptr [rsp + 0x84]
 0x1400035fd  inc  eax
 0x1400035ff  mov  dword ptr [rsp + 0x84], eax   <- the store the output loses
 0x140003606  cmp  dword ptr [rsp + 0x84], 5
 0x14000360e  jge  0x140003629
```

One slot is loaded, incremented, stored back and re-read by the test. kuna
renders the load side on the slot (`v55 <= 4`) and the store side on the
register (`v7 = ...`), and `v7` collects the increments of four other counters
and three call returns besides.

## The fix

- After heritage the write-back is an address-tied `COPY` whose only reader is
  the loop header's `MULTIEQUAL` — and that `MULTIEQUAL`'s output is the same
  frame slot. `RulePropagateCopy` rewrites the marker to read the register, the
  `COPY` loses its last descendant and dies as dead code. `option
  loopcounterstore` (new, default-on, DIV-146) refuses that one propagation.
- The shape is the self-updating tied phi: same address *and* size on both
  sides, the stored value computed from the phi's own output, and the register
  clause that separates `mov eax,[m]` / `inc eax` / `mov [m],eax` from
  `add dword [m],1`. Without the register clause the brake also keeps the
  lifter temporary of an `add [mem],1` alive, which displaces the iterator
  statement and costs the `for` form — the `forloop_loaditer` datatest catches
  exactly that.
- The `COPY`-chain walk is not cosmetic: the rule pool decides for itself
  whether the counter's load side or its write-back folds first, so a predicate
  that only accepted a direct reference to the phi output would fire or not
  depending on that order.
- Where `Merge` would have succeeded anyway the kept `COPY` becomes the
  statement's root op, and `emitInplaceOp` matched only a bare binary op — 85
  statements corpus-wide fell back from `v += 1` to `v = v + 1`. The in-place
  render now looks through a `COPY` of an *implied* two-input value, so those
  are byte-identical again.
- `tiedstorekeep` (DIV-105) does not reach this: its predicate wants the stored
  value to come from a call, and a counter bump is an `INT_ADD`. Flipping it
  either way leaves these loops unchanged.

## The tests

`make test` PARITY OK 675/675, `make test-stages` PARITY OK 714/714,
`make rust-test` green, `make check-spec` green in strict mode, `kuna catalog
--check` OK. `loopcounterstore_defaults_on_and_toggles` pins the option
default and its ArchSeam copy. Corpus sweep over 134 binaries / 54,471
functions in both arms: the only differences are the five repaired loops plus
nine `bool // cf` flag temporaries that stop being emitted. No stages testcase
and no promoted CLI probe — the misprint needs a `Merge` decline that no
synthetic fixture reproduces; five were built and measured, and DIV-105 records
the same negative for the same function.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
