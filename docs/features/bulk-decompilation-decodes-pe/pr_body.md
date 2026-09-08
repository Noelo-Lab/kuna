## The problem

A PE may keep its Import Address Table inside its code section, and a packed one
usually does. `kuna decompile-all` then invents a function body for every import
pointer word — 50 of the 56 entries on the reported image, each one four bytes of
pointer read as instructions:

```
$ kuna decompile-all bm3.exe --json --filter '^VirtualAlloc$' --no-vars
  "count": 1,
  "total": 56,
      "name": "VirtualAlloc",
      "address_hex": "0x4010ce",
      "size": 4,
      "code": "void VirtualAlloc(void)\n{\n  char *v1;\n  \n  *v1 = *v1 + (char)v1; // warn: funcboundflow: fall-through reached the next function entry; truncating flow here\n}",
```

`0x4010ce` is the third kernel32 IAT slot. The image has two sections, both
unnamed, and the first (characteristics `0xe0000060` — `CODE|EXECUTE|READ|WRITE`)
holds the whole import directory. The same binary's real function inventory is
six entries. Reproduce on the vendored twin added here:

```
kuna decompile-all decompiler/crates/kuna-analysis/tests/fixtures/pe_iatincode_i386.exe \
     --json --filter '^VirtualAlloc$' --no-vars
```

## The fix

- `function_entries_executable` already meant to keep import slots out of the
  batch set — its only discriminator was the loader `CODE` section flag, which a
  packed PE defeats simply by having one section. It now also excludes the slot
  addresses themselves.
- The loader publishes those addresses rather than the batch list re-deriving
  them: `ObjectFormat::import_slots` (PE's Import Address Table today, an empty
  default everywhere else) → `ObjectLoadImage::import_slot_ranges` →
  `ConsoleProgram::is_import_slot`. This is the address half of the answer
  `resolve_imports` already gives; `peimportcall` has been painting
  `Varnode::externref` over exactly these ranges since it shipped.
- Read from the loader and not from that pass, deliberately: `peimportcall` is a
  decompiler option, and turning it off to compare against upstream should not
  bring 50 phantom bodies back.
- A `--define-function` declaration outranks the new test as it already outranks
  the section-flag one, so an address the import directory claims can still be
  decompiled on request.
- Nothing leaves the inventory. `kuna functions` still lists all 56 entries,
  `--addr`/`--functions VirtualAlloc` still decompiles the slot, and the call
  naming the slot symbol exists for is untouched — the surviving body on the
  reported image still renders `ExitProcess(0)`.

## The tests

Three cargo tests in `verify_pe_imports` (batch set is the entry alone; the
entry's calls still render `VirtualAlloc(`/`GetModuleHandleA(`; a declared entry
survives) against a new 4.6 KB synthesized fixture whose one section is
`0xe0000060`, plus two promoted CLI probes — the acceptance, and its pairing
clause that a fix which merely stopped registering imports would fail. Both fail
on the unpatched tree.

Swept all 158 PE images in the dataset, comparing the executable target list
before and after: 137 unchanged (their IAT is in `.idata`, already excluded), 1
unloadable either way, 20 changed. On 19 of the 20 the number of dropped entries
equals an independent Python parse of the image's own Import Directory exactly;
the 20th drops one more, a discovered `sub_4010ae` sitting unaligned inside the
IAT whose body is the same phantom shape. No image lost a real function.

Gates: `make test` 675/675, `make test-stages` 687/687, `make test-cli` 89/89,
`make check-spec` OK, `kuna catalog --check` OK, `make rust-test` green.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
