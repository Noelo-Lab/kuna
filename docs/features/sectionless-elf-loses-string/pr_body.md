## The problem

An ELF with no section header table — `sstrip`, a packer, or a CTF author's
`e_shoff = 0` — loses every data cross-reference, so no string has an owning
function and `kuna xrefs --to <string>` answers zero. The same bytes with the
section table intact answer correctly. Two builds of one program, differing only
in whether the section header table is present:

```console
$ printf '#include <stdio.h>\nint main(void){ puts("\\n[+] Correct!"); return 0; }\n' > c.c
$ gcc -O0 -o sec c.c && cp sec nosec
$ python3 -c 'import struct;b=bytearray(open("nosec","rb").read());struct.pack_into("<Q",b,0x28,0);struct.pack_into("<H",b,0x3c,0);struct.pack_into("<H",b,0x3e,0);open("nosec","wb").write(bytes(b))'

$ kuna strings ./sec   --filter Correct --json | grep -E '"xrefs_count"|"name"'
      "xrefs_count": 1,
          "name": "main",

$ kuna strings ./nosec --filter Correct --json | grep -E '"xrefs_count"|"name"'
      "xrefs_count": 0,
```

Control flow survives — `kuna disassemble` prints the `LEA RDI,[<string>]` and
`kuna xrefs --from` returns the function's CALL edges — so the disassembly shows
the reference the index denies.

## The fix

- The reference walk classifies a data operand by asking whether its value lands
  in memory the image maps, and asked that of the section table alone. With no
  section table the answer is "nothing is mapped" and every data operand is
  discarded; control flow never consults it, which is why calls survived.
- `mapped_ranges` now falls back to the `PT_LOAD` segments when there is no
  section table at all — the same substitution `executable_sections` already
  makes for the plausible-code oracle, and the description the loader itself
  works from.
- Only the no-section-table arm falls back. An image that *has* sections which
  are not the runtime layout — a relocatable object, whose section addresses are
  pre-link — is declined a step earlier and is untouched.
- A `PT_LOAD` is coarser than the section list, but the padding and the ELF
  header of a low-based PIE sit below the existing address floor. Measured on the
  control pair above, over the functions both runs discover, the section-less
  image now answers **13 data references, identical to the sectioned image's 13**,
  with none added.

## The tests

`tests/cli/sectionless-elf-loses-string.json` (promoted acceptance probe) plus two
unit tests in `listing/xrefs.rs`, over a new 8 KB vendored fixture
`sectionless_x86_64` (`+ .py` generator) whose one function forms
`LEA RDI,[0x2000]` into a `PF_R` `PT_LOAD`; it reports `functions: []` before the
change and `sub_1000` after. The second unit test is the control: an image with
sections is still classified against them. `make test` 675/675, `make test-stages`
640/640, `make rust-test` green, `make check-spec` OK, `tests/cli` 45/45,
`catalog OK`. Decompiled C is byte-identical before/after on every image swept.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
