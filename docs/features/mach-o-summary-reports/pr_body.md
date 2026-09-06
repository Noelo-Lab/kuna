## The problem

`kuna functions --summary` reports the image entry point, and on any modern
Mach-O it reported a file offset where every other format reports a virtual
address. The offset matches no function, so the name degrades to a hex string
and the reachability count collapses to `null` — the two fields the summary
exists to answer.

```
$ kuna functions decompiler/crates/kuna-analysis/tests/fixtures/macho_stripped_main --summary --json
  "summary": {
    "entry": {
      "name": "0x5b0",
      "address": 1456,
      "address_hex": "0x5b0"
    },
    "reachable_from_entry": null,
```

`main` is at `0x1000005b0`: `LC_MAIN.entryoff` is `0x5b0` and `__TEXT.vmaddr` is
`0x100000000`. The inventory in the same document already has it right — the
`largest` list names `main` at `0x1000005b0`.

## The fix

- `object`'s `File::entry()` returns the raw header field, which is already a VMA
  for ELF `e_entry`, PE `AddressOfEntryPoint` and Mach-O `LC_UNIXTHREAD` (the
  saved thread state's PC), but is a `__TEXT`-relative file offset for `LC_MAIN`.
  New `kuna_analysis::analyzers::entry::image_entry_vma` states that once.
- It rebases **only** `LC_MAIN`, through the existing `macho_main_entry_vma`,
  which answers for nothing else. Adding `__TEXT.vmaddr` by format instead would
  double-count an `LC_UNIXTHREAD` entry, which already carries it.
- Three surfaces read the raw field and now read the helper: `functions
  --summary`'s `entry`/`reachable_from_entry`, `decompile-graph`'s
  `isEntryPoint` (which flagged no row at all on an `LC_MAIN` image), and the
  `decompile-project` README's entry row.
- A `0` entry is still reported as absent, not as `0x0` — a relocatable declares
  no entry and `0` is a real address there.

After:

```
    "entry": { "name": "main", "address": 4294968752, "address_hex": "0x1000005b0" },
    "reachable_from_entry": 2,
```

## The tests

Three unit tests on `image_entry_vma` (Mach-O rebase, ELF/PE/ARM-Thumb
pass-through, `.o` → `None`) and one end-to-end test per consumer; each of the
three end-to-end tests fails on the unpatched tree. `tests/cli/mach-o-summary-reports.json`
promotes the acceptance, re-pointed onto the in-repo `macho_stripped_main`
fixture because CI has no dataset.

`make test` 675/675 PARITY OK · `make test-stages` 635/635 PARITY OK ·
`make check-spec` OK · `kuna catalog --check` OK · `make test-cli` 42/42.
No existing expectation moved.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
