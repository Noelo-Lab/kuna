## The problem

`kuna functions` loses `main` on a non-PIE x86-64 executable, and with it
everything `main` calls. `_start` names `main` explicitly, but the entry oracle
only reads the position-independent encoding of that hand-over.

The repro is in the tree — `funcstart_patterns_x86_64` is a stripped, non-PIE
fixture whose `main` sits at `0x401020`:

```
$ kuna functions decompiler/crates/kuna-analysis/tests/fixtures/funcstart_patterns_x86_64 --json \
    | jq -r '.count, (.functions[] | .address_hex)'
8
0x401000
0x401040      <- _start
0x401070
0x401080
0x4010f0
0x401120
0x401130
0x401178
```

`0x401020` is not there, and `_start` is pointing straight at it:

```
$ objdump -d --start-address=0x401058 --stop-address=0x401065 \
    decompiler/crates/kuna-analysis/tests/fixtures/funcstart_patterns_x86_64
  401058: 48 c7 c7 20 10 40 00   mov    $0x401020,%rdi
  40105f: ff 15 8b 2f 00 00      call   *0x2f8b(%rip)     # __libc_start_main
```

On the reported binary (a stripped crackme with no `.eh_frame`) the same miss
cost seven functions: the inventory reported 34, `main` at `0x11a9b` was absent,
`xrefs --to 0x11266` answered `count: 0` for a function `main` calls directly,
and the entry before `main` ran on through its bytes with a size of 2230.

## The fix

- Oracle 4 matched on `48 8d 3d` — `lea rdi,[rip+disp32]` — alone. A non-PIE
  `_start` hands `main` over as a bare address immediate instead. It is the same
  idiom, the same register and the same following call; only the link model
  differs, so the three immediate encodings (`mov edi,imm32`, `mov rdi,imm32`,
  `movabs rdi,imm64`) are now read as well.
- The immediate scan runs only where the `lea` scan finds nothing, so a PIE image
  decodes byte-identically to before. Of 1,919 x86-64 ELF objects swept, 1,788
  carry the `lea` and are untouched by construction.
- A bare immediate is much weaker evidence than a `lea` displacement, so a
  candidate is emitted only if a `call` (`e8` or `ff /2`) begins within 16 bytes,
  the immediate lands in an executable section, and it is the only immediate in
  the window that satisfies both. An ambiguous decode returns nothing rather than
  a guess — the rule the ARM GOT-offset decode already applies.

Seeding `main` also corrects the bodies that had run through it: on the witness
`sub_112e9` goes 2230 → 1855 bytes and `_INIT_0` 716 → 6 (`0x10af0` really is a
6-byte `endbr64; jmp` thunk). No entry is lost anywhere in either sweep, and every
newly-recovered start was checked against its disassembly.

## The tests

`tests/cli/elf-discovery-omits-explicit.json` is the acceptance probe on the
vendored fixture above; it fails without this change. Three cargo tests in
`kuna-analysis` cover the fixture's `main`, the three guards (no following call,
two competing immediates, an immediate outside every executable section), and the
`ff /2` vs `ff /4` ModRM read.

Sweeps: 119 of the 131 fallback-reachable system binaries were runnable on both
builds and two changed, both gaining a real `main` (`/usr/bin/busybox` 2116 → 2164);
across the 43 ELF binaries in the RE corpus, two changed, both gaining a real
`main`. `make test` 675/675, `make test-stages` 725/725, `make rust-test`,
`make check-spec`, `make test-cli` 109/109 and `kuna catalog --check` all green.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
