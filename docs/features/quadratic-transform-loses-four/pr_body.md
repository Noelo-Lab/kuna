## The problem

A stack buffer the binary fills with four constants is declared, XORed through a
pointer loop, and never assigned. Build the 252-byte MSVC function this comes from
and ask for it:

```
$ as --32 -o /tmp/f.o decompiler/crates/kuna-analysis/tests/fixtures/indexaliasguard_i386.s
$ ld -m elf_i386 -T decompiler/crates/kuna-analysis/tests/fixtures/indexaliasguard_i386.ld -o /tmp/f /tmp/f.o
$ kuna decompile /tmp/f 0x401330 --addr
unsigned int transform(int a0,int a1,int a2,int a3)
{
  unsigned int v10 [4];
  ...
          do {
            v6 = &v5[1];
            v3 = *v8;
            v8 = &v8[1];
            *v5 = *v5 ^ -(unsigned int)((v2 & v1) != 0) & v3;
```

`v10` is never written. The binary writes it four times, right at the top:

```
$ kuna disassemble /tmp/f --addr 0x401330 --count 8
0x401330      83ec20        SUB ESP,0x20
0x401333      a1bc604000    MOV EAX,[0x4060bc]
0x401338      8b0dc0604000  MOV ECX,dword ptr [0x4060c0]
0x40133e      8b15c4604000  MOV EDX,dword ptr [0x4060c4]
0x401344      89442410      MOV dword ptr [ESP + 0x10],EAX
0x401348      a1c8604000    MOV EAX,[0x4060c8]
0x40134d      894c2414      MOV dword ptr [ESP + 0x14],ECX
0x401355      8944241c      MOV dword ptr [ESP + 0x1c],EAX
```

Every *read* of those slots goes through `LEA EAX,[ESP+0x20]` at 0x401399, so
nothing in the SSA reads them directly and dead-code elimination takes the four
stores. The analyst reads the buffer as starting from garbage.

Filed as RE-friction need `quadratic-transform-loses-four` (blocker, 1 instance,
sddecoder `decoder.exe`).

## The fix

- New option `indexaliasguard off|load|full`, default `load`, restoring the last
  arm of `Heritage::guard` (heritage.cc:1194) that kuna shipped behind a
  hard-coded `highPtrPossible == false`.
- `load` runs `Heritage::guardLoads` (heritage.cc:1570): an `addrforce` `COPY`
  read of the range in front of every indexed-stack LOAD whose guard window
  covers it, so the slot has a reader where the pointer is dereferenced. The
  COPY is artificial and `handleNewLoadCopies` — already ported, and consuming
  exactly the list nothing had been filling — propagates it away again, keeping
  only the `addrforce` marks it earned.
- `full` also runs `Heritage::guardStores` (heritage.cc:1538), which is what
  upstream always does. It is not the default: an A/B `decompile-all` over 60
  decbench O2 binaries (12,118 functions) puts `full` at 232 changed functions,
  +1,166 lines and +14 gotos, against `load`'s 56 changed functions, **-61 lines
  and no goto change** — and `full` recovers nothing `load` does not.
- `highPtrPossible` is upstream's inline minus the cspec `<nohighptr>` ranges,
  which only the PIC families set and kuna does not read.

The need's filed hypothesis (store liveness) and the round-8 refutation (P6
`RangeHint` joining) are both overturned; the shadow scalars the refutation found
inside the array's extent have no readers *because* the defining stores were
already gone, and go away with them.

## The tests

`tests/stages/ghdec-indexaliasguard.xml` decompiles the witness twice: `off`
pins the bug (array declared, XORed, never assigned) and `load` pins all four
recovered stores; the read-side assertions are `min=2 max=2`, so the reads
provably do not move. The acceptance probe is promoted to
`tests/cli/quadratic-transform-loses-four.json` against a vendored 4,656-byte
ELF fixture at the original VMAs.

Over the 56 changed functions no real call or statement is lost — the only calls
that disappear are `PTRSUB` pseudo-ops. `dexter.dll` gains its 10-entry lookup
table `m[0..9] = {0,2,4,6,8,1,3,5,7,9}`, `mydoom.exe` gains two base64-alphabet
`builtin_strncpy` initialisers, and coreutils `ginstall`/`mv` at 0x8720 stop
redeclaring one name eight times with eight different types.

Gates: `make test` PARITY OK 675/675, `make test-stages` PARITY OK 721/721,
`make rust-test` green, `make check-spec` green (strict too), `make test-cli`
105/105, `kuna catalog --check` OK. Speed: +2.24% on the witness function
(median of 7, 5% budget) and +0.28% on a whole-binary `decompile-all` of
coreutils `ginstall` (14.46 s -> 14.50 s, median of 3).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
