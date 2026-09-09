## The problem

An import's name sits on two entries — the veneer a direct `call` targets and the
pointer slot that veneer reads — so asking for it by name was refused. Every one
of the nine imports in this Mach-O crackme is that shape.

```
$ kuna decompile ./bin/main strcmp
error: selector "strcmp" is ambiguous; candidates:
  strcmp at synthetic 0x100003ede
  strcmp at synthetic 0x100008030
use a section-qualified selector to choose one candidate
```

Only `--addr` reached the function. The same names fared worse on `disassemble`
and `read`, which do not report the ambiguity — they fall through to the raw
symbol table and answer at the pointer word, so on a MinGW PE
`kuna disassemble ./a.exe memcpy` listed eight bytes of import table as
instructions.

## The fix

- A selector that matches several entries resolves to the one in executable
  memory, when exactly one is. That is the same test whole-binary decompilation
  already uses (`entry_is_executable`: a loader section carrying `CODE`, minus
  the import slots the loader itself filled in) — reusing it is what handles the
  one-section PE whose IAT sits inside `.text`, where the section flag alone
  cannot tell the two apart.
- Both name lookups take it, not just the one that reports ambiguity. Narrowing
  only `resolve_entry` would have left `decompile` and `disassemble` answering
  one name at two different addresses.
- The inventory is not pruned. Dropping non-executable import rows would settle
  the ambiguity too, and would delete the only name a slot with no veneer ever
  carries — `__DATA,__got`, `__DATA,__nl_symbol_ptr`, an IAT entry a
  `call [slot]` reaches directly. Two of the nine such rows on a nearby Mach-O
  are exactly that.
- It requires *exactly* one executable candidate, so two same-named definitions
  in different code sections of a relocatable object still report every
  candidate.

## The tests

`verify_entry_selectors.rs` gains the Mach-O import pair (resolves to the veneer,
both rows survive) — it fails on the unpatched tree — and a case pinning that two
`.text` definitions stay ambiguous. `tests/cli/mach-o-import-data.json` guards the
CLI surface against the vendored `macho_imports`.

Swept across 163 vendored images: of 155 duplicated names, 134 go from the
ambiguity error to a resolved veneer and none go the other way; on `disassemble`
and `read`, 78 names move from the pointer word to its thunk, and 77 of those old
answers were ones kuna itself flagged as "in a non-executable data section".

🤖 Generated with [Claude Code](https://claude.com/claude-code)
