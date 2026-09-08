## The problem

`kuna decompile-all` reports zero functions on a PE that declares its entry
point inside its own header page — the `SizeOfHeaders` bytes Windows maps before
the first section, where a packer is free to lay its stub. Naming the same
address decompiles it in full.

```console
$ F=decompiler/crates/kuna-analysis/tests/fixtures/pe_headerentry_i386.exe
$ kuna decompile-all $F --json
error: no functions discovered in .../pe_headerentry_i386.exe
{
  "count": 0,
  "error": null,
  "functions": [],
  "assertions": []
}
$ kuna decompile $F 0x400154 --addr
int sub_400154(int a0)
{
  return a0 + 7;
}
```

The reported image (crackmes.one `5ab77f6333c5d40ad448ca40`, MEW-packed, two
nameless sections both `0xc00000e0` with `MEM_EXECUTE` clear, entry `0x154`) has
no executable section for a candidate to land in *and* its entry is in no
section, so its whole inventory was the two Import Address Table pointer slots.
Once those were correctly withheld from the batch set, `decompile-all` answered
`count: 0`.

## The fix

- The header page is published `CODE | READONLY` rather than `DATA | READONLY`
  when `AddressOfEntryPoint` points into it, and that one address is exempt from
  discovery's executable-section filter. Nothing is guessed: the image names the
  address the OS jumps to, and no section flag ever spoke for bytes that are in
  no section.
- `AddressOfEntryPoint` `0` is the "no entry point" encoding, which `object`
  reports as `ImageBase` — inside the page — so it is rejected rather than
  making a function of the `MZ` signature.
- Only the entry is exempt, never a `.pdata`, TLS or export candidate. An entry
  inside a section the image flags *non*-executable is deliberately still not
  exempt: there the flag is a statement about those bytes, and kuna answers it by
  naming the cause and the `--define-function` that overrides it.

## The tests

`pe_headercode_i386.exe` (1,024 bytes, built by the sibling `.py`) reproduces the
reported layout: unpatched it answers `count: 0, functions: []`, after it emits
`GetProcAddress(LoadLibraryA(0x402080),0x402052)` at `0x400154` with the two IAT
slots still absent. Promoted verbatim as
`tests/cli/whole-binary-decompilation-treats.json`; seven unit tests cover the
entry-RVA-0 and entry-past-the-page edges and pin the non-exempt data-section
entry. Sweeping 250 dataset PE images and 22 in-repo ones before/after, one
changed — the fixture built for the trait.

One existing expectation moves with the rule: `pe_headerentry_i386.exe` declares
its entry at RVA `0x154`, so its header page is now `CODE | READONLY`.
`the_header_page_never_overlaps_a_section` still pins `DATA | READONLY` for an
ordinary PE, which is every image a compiler emits.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
