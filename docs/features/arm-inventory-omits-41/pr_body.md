## The problem

On a stripped, non-PIE ARM32 executable `kuna functions` stops at the C runtime:
everything `main` calls is missing from the inventory, and one entry swallows the
whole program. On the reported image (a 26 KB ARM crackme) it listed 17 functions
and omitted all 41 validators `main` calls, with `_INIT_0` reported as 17736 bytes
— 0x104bc through `main` at 0x14a04.

The repo carries a 1 KB fixture in the same shape; before this change:

```
$ kuna functions decompiler/crates/kuna-analysis/tests/fixtures/armdiscseed_le32 --json
  0x10150  __libc_start_main  12
  0x10160  sub_10160          32
  0x10180  main               28      <- 0x10180..0x1019c, the callee is inside it

$ kuna xrefs decompiler/crates/kuna-analysis/tests/fixtures/armdiscseed_le32 --to 0x10190 --json
  "count": 1, "instruction": "bl 0x10190", "from_function": { "name": "main" }
```

`xrefs` names the callee a function; `functions` does not.

## The fix

- The deferred Listing walk now takes the entries the load-time passes already
  committed as extra roots, handed down from `commit_pending_analysis` instead of
  recomputed. `listing_seeds` rebuilds its roots from the object, which is the
  entry-oracle union — so an entry only a separately gated standalone pass knows
  about was never a root.
- `armlibcmain`'s non-PIE ARM `main` is the case that bites: it is the only
  address that reaches the program, and `_start` hands it to `__libc_start_main`
  through a literal pool word rather than a branch, so a walk rooted at `e_entry`
  alone stops inside crt1. `kuna xrefs` seeds its own descent from the *committed*
  inventory, which is why the two surfaces disagreed.
- Seeds ride the same `funcstart_patterns` gate as the prologue starts, so x86-64
  (where that flag is off) is unchanged by construction, and they are
  exec-filtered like every other seed — the witness's junk-word `bl 0x59684c`
  stays a non-function.

## The tests

`verify_armdiscseed.rs`: the fixture's inventory lists the callee and `main` stops
at it; the same walk run with and without the committed seeds, where only the
seeded one reaches the callee; and a seed outside every executable section that is
not walked. Promoted probe `tests/cli/arm-inventory-omits-41.json`.

Witness: 17 -> 57 functions, all 41 validators, `_INIT_0` 17736 -> 56 bytes, and
`sub_152b4` (a junk word at `0x152b4`, branched over by `b 0x152b8`) dropped.
803 decbench binaries swept before/after: 0 inventories changed. The witness's
41 new functions decompile with 0 errors, and the dropped `sub_152b4` is the
separately reported false entry on the same image.
`make test` 675/675 PARITY OK, `make test-stages` 721/721 PARITY OK,
`make rust-test` green, `make check-spec` green, `catalog OK`.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
