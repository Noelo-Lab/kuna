## The problem

A stack-to-stack copy of a buffer whose size is not a multiple of eight loses
the bytes in its middle. gcc copies 31 bytes with four 8-byte moves at offsets
0, 8, 15 and 23 — so the middle two overlap on byte 15 — and kuna emits the
first and the last and nothing between them.

```
$ kuna decompile 0xJam3z-Medium sub_15dc
    v3 = sub_1348(v6,0x1f,v4);
    v7[0] = v6[0];          # bytes 0..7
    v10 = v9;               # bytes 23..30
    sub_1348(v7,0x1f,v4);   # 8..22 are never written
```

The disassembly writes every one of the 31 bytes:

```
$ objdump -d --start-address=0x17da --stop-address=0x1812 0xJam3z-Medium
  17da: mov -0x1d0(%rbp),%rax     17e8: mov %rax,-0x1b0(%rbp)   # [0,8)
  17e1: mov -0x1c8(%rbp),%rdx     17ef: mov %rdx,-0x1a8(%rbp)   # [8,16)
  17f6: mov -0x1c1(%rbp),%rax     1804: mov %rax,-0x1a1(%rbp)   # [15,23)
  17fd: mov -0x1b9(%rbp),%rdx     180b: mov %rdx,-0x199(%rbp)   # [23,31)
```

An analyst reads the gap as an uninitialised tail, which is the opposite of what
the program does.

## The fix

- `RuleStoreVarnode` marks a frame store's `COPY` output `stack_store`, and that
  mark is what keeps the store printed: `ActionDirectWrite` calls a `COPY` into
  the frame a *direct write* only when its output `isStackStore()`, and
  `ActionDeadCode` drops the `addrforce` of anything that is not one.
- `Heritage::refineWrite` rebuilds a refined store's storage out of `SUBPIECE`
  cells that `splitByRefinement` creates with plain `newVarnode`, so the cells
  carry no mark and every one of them is swept. Only overlapping accesses are
  refined, which is why exactly the two middle stores vanish.
- New option `splitstorekeep` (default on) carries the mark onto the cells. It
  claims nothing new — a piece of a store is a store — and is inert on any
  function whose stack accesses agree on their boundaries.
- Fixed at `refineWrite` rather than by widening `ActionDirectWrite`'s `COPY`
  arm, which would also mark the ordinary stack copies that arm deliberately
  excludes.

## The tests

`tests/stages/kuna-splitstorekeep.xml` is two-pass over a new vendored
reduction (`splitstorekeep_x86_64`): `copy31` is five assignments at the default
and one with `splitstorekeep off` (the filed bug), and the sibling `copy32` — the
same shape at 32 bytes, where nothing overlaps — is identical in both passes.
`tests/cli/31-byte-buffer-copy.json` is the promoted acceptance probe.

`make test` PARITY OK 675/675 (no re-pin), `make test-stages` PARITY OK 746/746
(740 → 746, none moved), `make test-cli` 119/119, `make check-spec` green,
`kuna catalog --check` OK, workspace suite green. `decompile-all` in both arms
over 55 binaries: 0 differ. Speed +0.21% on the witness function (median of 5).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
