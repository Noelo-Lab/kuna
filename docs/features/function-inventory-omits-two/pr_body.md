## The problem

`kuna functions` misses any PE function that is only ever reached through a
stored pointer — a VM handler table, a vtable slot, a registered callback — and
then folds it into the function ahead of it. On this x64 MSVC crackme the
inventory lists 228 functions, and four of the 30 entries in the handler table at
`0x1400093e0` are not among them:

```
$ kuna functions vm.exe --json | grep -c address_hex
228
$ kuna functions vm.exe --json | grep 0x140003c60
$ kuna decompile vm.exe 0x140003c60 --addr
void sub_140003c60(long long a0)
{
  *(char *)(a0 + 0x48) = 0;
}
$ kuna functions vm.exe --json | grep -A5 '"0x140003c50"'
      "address_hex": "0x140003c50",
      "size": 32                      <- 0x140003c50..0x140003c70: two functions
```

The bytes decompile fine when you name the address by hand; nothing in the
inventory tells you the address is there. The four handler-table entries the
image itself holds pointers to are `0x140003c20`, `0x140003f40`, `0x140003c60`
and `0x1400041b0`.

## The fix

- A new PE entry oracle reads the base-relocation directory. It lists the image
  words holding an absolute address — the loader has to fix them up — so a
  relocated word whose value lands in an executable section is a stored code
  address, not a byte pattern that resembles one. `DIR64` on a PE32+,
  `HIGHLOW` on a PE32, each at its own pointer width.
- `.pdata` separates a start from a label. A candidate strictly inside a
  `RUNTIME_FUNCTION`'s `[BeginAddress, EndAddress)` is a non-PIC jump-table
  destination and is dropped; a `BeginAddress` is kept.
- A candidate must also lie in the part of its own section that `.pdata` vouches
  for. Without that, a packed image sinks the oracle: `jormungandr.exe` merges
  its payload into `.text`, so 666 relocated pointers into a UTF-16 locale table
  read as code pointers — all of them below the first `RUNTIME_FUNCTION`.
- That guard is the precondition too, so an image with no readable `[begin, end)`
  table — a PE32 with no exception directory, or an ARM64 PE whose 8-byte records
  carry no `EndAddress` — abstains rather than seed unvetted addresses.

No option: a relocation is the image's own statement that the word is an address,
so this corrects wrong output rather than trading one plausible reading for
another.

## The tests

`pe_reloctable_x86_64.exe` (hand-assembled, 3584 bytes) carries one address-taken
handler and the three things the oracle must decline — an interior label, a data
pointer, and an executable address past what `.pdata` vouches for. Four unit
tests; without the fix the first fails and the handler is swallowed by the
function before it (2 functions, one of 80 bytes, instead of 3).

Witness: 228 → 232 functions, `sub_140003c50` 32 → 16 bytes, nothing dropped.
Swept over 150 dataset PE images before/after: 38 inventories move, 747 starts
added and 31 removed — each removal a start the new seed
supersedes, e.g. `0x140028d1c` in `CrackVM-V2.exe`, four bytes into the
`MOV RAX,[RSP+0x28]` that opens the function its vtable slot names.

Gates: `make test` 675/675 PARITY OK · `make test-stages` 725/725 PARITY OK ·
`make rust-test` green (382 test binaries, 0 failed) · `make check-spec` OK · `make test-cli` 109/109 ·
`kuna catalog --check` OK.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
