## The problem

A merged local was declared at a pointer type nothing in its own body was
checked against, so a one-byte comparison read as an eight-byte dereference.
`sub_1400542a1` of the round-12 witness (x86-64 PE `crackme.obf.exe`) merges one
RAX across an obfuscated dispatch loop:

```
$ kuna decompile crackme.obf.exe 0x1400542a1 --option tailcallframe off
  unsigned long long *v9; // rax
  ...
        v9 = (char *)sub_1400778c4(&v14[0x58],6);
        v8 = (uint4)*v9;
        v7 = v8;
```

The instruction is one byte:

```
$ rasm2 -a x86 -b 64 -D 0fbe0083f852
0x00000000   3   0fbe00  movsx eax, byte [rax]
0x00000003   3   83f852  cmp eax, 0x52
```

Read against that declaration, `*v9` loads eight bytes -- seven past the object,
with the sign extension dropped. Nine sites in this one function; the need
records one instance.

The IR is correct and is not touched here. The SSA load is one byte, the compare
downstream is `SBORROW4`, and the cast machinery left `*v9` uncast precisely
because it saw a one-byte pointee. Only the declaration disagreed.

## The fix

- Upstream declares a local at `sym->getType()` (`PrintC::emitVarDecl`,
  printc.cc:1719), which is the merged HighVariable's type -- the member
  `HighVariable::getTypeRepresentative` picks as most specialized, and the same
  type `ActionSetCasts` reads through `getHighTypeReadFacing`. Declaration and
  body agree by construction. kuna's printer walks HighVariables rather than
  Symbols and took the type off its *declaration* representative (first
  address-tied member, else member zero) -- a different Varnode.
- New option `declhightype` (default on, DIV-160) declares the type
  representative's type instead.
- Bounded to a high the Symbol table does not describe. kuna does not always
  type-lock the storage a Symbol claims, so on a mapped local the most
  specialized member can be a transient the Symbol never described: an ungated
  first cut re-declared a DWARF `acl_entry_obj_tag *` as the `obj_prefix` struct
  one member was cast to. The declaration representative still supplies the
  storage comment, the array adornment and the composite mapped-Symbol override.

## The tests

Seven unit tests on the representative ordering (lock beats specialization,
pointer beats same-width integer, ties keep the earlier member). The acceptance
probe is promoted as `tests/cli/byte-comparisons-incorrectly-dereference.json`
against the vendored `covercopy_x86_64`, whose `lookup_service` has the same
shape (`unsigned long *v2` assigned an uncast `char *`); CI has no dataset, so
the witness image stays in the need's Reproduction. `make test` 675/675 and
`make test-stages` 763/763 both PARITY OK, `make test-cli` 124/124,
`make rust-test` green (6,458 tests), `make check-spec` OK.

The same option also repairs a second, independently reported function -- a
different binary, challenge and tester (`graphy`, `sub_1005350`), where a VM
temporary was declared `unsigned long` and then dereferenced. That defect is
present with `--option declhightype off` and gone at the default, so one
decision point covers both reports.

Corpus sweep, `decompile-all` off vs default over 73 decbench binaries (22,848
functions): under the ungated rule 517 functions changed and **every changed
line was a declaration** -- not one statement or cast moved anywhere. With the
mapped-Symbol guard, 277 of 22,595 change: 220 declaration lines gain or lose a
level of indirection, 78 change pointee type, 84 are scalar spelling. dpkg
`f_archives` declared `archivedetails **v7` and then wrote `v7->next`, which
only type-checks at `archivedetails *`.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
