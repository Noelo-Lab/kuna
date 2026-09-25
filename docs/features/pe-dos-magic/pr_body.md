A PE whose DOS `e_magic` is not `MZ` is rejected outright, even when `e_lfanew` still points at an intact `PE\0\0` header. Overwrite the first byte of any in-repo PE and every command exits 1:

```
$ python3 -c "import sys;b=bytearray(open(sys.argv[1],'rb').read());b[0]=0x15;open('bad.exe','wb').write(b)" \
    decompiler/crates/kuna-analysis/tests/fixtures/calleeprotostack_pe_i386.exe
$ kuna functions bad.exe --summary --json
error: could not build an architecture for bad.exe: File: bad.exe : not in recognized object file format: Unknown file magic; for a headerless image use --raw-image ...
$ echo $?
1
```

With this change the same command loads the image and says what it repaired:

```
[kuna] PE DOS header e_magic is 0x5a15, not MZ (0x5a4d); loaded via e_lfanew 0x40 -> PE signature (repaired in memory; the file is unchanged)
{
  "binary": "bad.exe",
  "count": 4,
  "total": 4,
  "error": null,
  "warnings": [
    "PE DOS header e_magic is 0x5a15, not MZ (0x5a4d); loaded via e_lfanew 0x40 -> PE signature (repaired in memory; the file is unchanged)"
  ],
  ...
```

## Fix

- New `loader/pe_dosmagic.rs`: when `object` recognizes no format and `e_lfanew` points at `PE\0\0`, `MZ` is written over `e_magic` in the loaded copy. The file on disk is never touched; anything that already parses is passed through byte for byte.
- Applied at the same point as the existing ELF section-table and `NumberOfRvaAndSizes` repairs (before the latter, which reads the headers behind `e_lfanew`), in both the engine bootstrap and `read_image`, so `strings`, `xrefs`, `disassemble` and `decompile-all` see the same view. The `load file` format sniff behind `kuna decompile` admits the same images.
- The loader's header repairs are kept on the loaded program and published as a `warnings` array in `functions --summary --json` (and `warning\t...` lines in its text form); a well-formed image has `"warnings": []`.

## Tests

- `pe_dosmagic` unit tests: healthy image untouched, trashed magic repaired and parses, no signature / wild `e_lfanew` / other formats left alone.
- `triage_cli`: the new 1 KB fixture `pe_dosmagic_i386.exe` joins the repaired-header summary test, which now also checks `warnings` and stderr for all three repairs; a healthy image reports `"warnings": []`.
- `tests/cli/pe-dos-magic.json`: exit 1 with `Unknown file magic` before, exit 0 with the entry and the warning after.
