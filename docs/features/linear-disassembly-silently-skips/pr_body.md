## The problem

`kuna disassemble` decodes forward from one end, so a branch that jumps over a
junk byte pulls the rest of the listing out of phase — and nothing in the output
says so. On an i386 PE from crackmes.one (`5ab77f6633c5d40ad448cbec`):

```
$ kuna disassemble ./corrupt.exe 0x43d060 --addr --count 70
0x43d08c      eb04                  JMP 0x43d092
0x43d08f      eb01                  JMP 0x43d092
0x43d091      c2aceb                RET 0xebac        <- covers 0x43d092
...
0x43d0ad      e9c0c89504            JMP 0x4d99972     <- outside the image
0x43d0c0      e83492aae2            CALL -0x1d119d07  <- not in the bytes
```

`notes` is empty. Two of the listing's own jumps name `0x43d092` and no row
starts there; the region is really a `LODSB`/`STOSB` decryption loop, and the
call and the out-of-image jump above are bytes the program never executes that
way. The same idiom desynchronizes the listing twice more, at `0x43d0ae` and
`0x43d0c1`.

The idiom on its own, with no binary needed:

```
$ printf '\x33\xc0\xeb\x04\x90\xeb\x01\xc2\xac\xeb\x01\xe9\x90\xfe\xc8' > /tmp/decoy.bin
$ objdump -D -b binary -m i386 --adjust-vma=0x10000 /tmp/decoy.bin
   10002:  eb 04     jmp    0x10008
   10005:  eb 01     jmp    0x10008
   10007:  c2 ac eb  ret    $0xebac      <- 0x10008 is inside it; LODSB is what is there
```

## The fix

- Every code listing now reports the addresses **its own instructions branch or
  call to** that no row of it starts at, on stderr and in the JSON `notes`. The
  evidence was already being collected for the literal-pool fold (`FixedRefs`);
  only targets inside the listed span are reported, because a branch out of the
  range says nothing about the range. Always on — a decode a byte out of phase
  is spelled exactly like one that is not, so the caller cannot infer it.
- New `--follow` decodes from those addresses as well as from the start, to a
  fixpoint over whatever the new rows themselves name. One re-anchor is not
  enough: the instruction at `0x43d092` is another `EB 01`. It fills what flow
  never reaches with the ordinary straight-line walk between the anchored rows,
  clipped so nothing crosses into one — so `--follow` never lists *less* than
  the plain listing, and both cover the same span.
- A run steps forward while the instruction has a fall-through successor
  (`xref_control_flow`'s last-op rule, now exposed as `FixedRefs::falls_through`).
  A `goto` whose destination *is* the fall-through address is not a dead end:
  SLEIGH gives the x86 `E8 00000000` get-PC idiom its own `goto rel32`
  constructor (`ia.sinc:2969`), and reading that as terminal stops the walk at
  the first instruction of most packer stubs.

Driver tier, so no `phases.toml` row and no catalog counts; `docs/history.md`
carries the DIV row.

## The tests

`tests/cli/linear-disassembly-silently-skips.json` is the acceptance probe
re-pointed at a new vendored fixture, `jumpoverdecoy_i386` — the witness's own 60
bytes rebased to `0x10000` — since CI has no dataset. Four `kuna-cli` integration
tests cover the note, the `--follow` recovery, that `--follow` is a no-op on an
ordinary function, and that both modes cover the same bytes; one `kuna-console`
unit test pins the fall-through rule including the `goto`-to-fall-through case.

Sweep of `kuna disassemble` with and without `--follow` over ~800 functions of
every vendored fixture: 2 listings differ, both anti-disassembly cases; the
always-on note fires on 8 of the ~800, every one a real desynchronization.
`make test` 675/675, `make test-stages` PARITY OK, `make test-cli` 114/114,
`make check-spec` green, workspace suite green.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
