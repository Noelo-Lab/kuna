# ELF test fixtures

Small, real, dynamically-linked ELF binaries used by the loader gates in
`loadimage_object.rs`'s test module (PLT/GOT import-name resolution — see
`src/elf_plt.rs`) and the console e2e gate
(`kuna-console/tests/verify_w11_elf_plt_names.rs`).

The XML datatest corpus cannot exercise these: it embeds raw bytechunks with
explicit `<symbol>` definitions and never constructs an `ObjectLoadImage`, so the
ELF loader (and thus PLT resolution) is off that path. These fixtures drive the
real ELF parser.

| File | What | Exercises |
|---|---|---|
| `fauxware` | classic non-PIE x86-64, not stripped (the angr `fauxware` sample) | `.plt` classic stubs (`FF 25` rip-rel), `.symtab` defined functions |
| `cet_pie_x86_64` | PIE x86-64 with CET (`.plt.sec`) | `endbr64; FF 25` CET stubs, naming at the `.plt.sec` call target |
| `stripped_dynamic_x86_64` | PIE x86-64, `.symtab` stripped (only `.dynsym`) | PLT resolution with no `.symtab` (dynsym/rela.plt only) |

Provenance: copied verbatim from `bs-artifacts/binaries/` (`fauxware`,
`debug_symbol`, `debug_symbol_mod_stripped` respectively). They are checked in
(each well under 32 KB) so the gates are hermetic and reproducible.
