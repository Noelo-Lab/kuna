// pdb_prog.c — a freestanding PE built with CodeView debug info so lld-link emits
// a matching `pdb_prog.pdb` alongside the EXE (plus the RSDS CodeView record in
// the PE debug directory). Fixture for the PDB-consuming pass (kuna-analysis
// s1_pdb PR-P1: stripped FUN_* -> real names via the matching `.pdb`).
//
// The EXE's own symbol table is NOT a source of these names — kuna's loader does
// not read the COFF symbol table for function naming, so without the `.pdb` the
// uniquely-named function below is a stripped `FUN_<addr>`. Only the PDB symbol
// stream (S_PUB32/S_GPROC32) recovers the real name.
//
// Built freestanding (own entry, no CRT) so it links with clang/lld-link on Linux
// without the MSVC CRT libs (the same recipe as pdb_min). `pdb_demo_compute` is a
// distinctively-named function whose recovered name proves the rename end to end.

// A distinctively-named, non-inlined function: its name is what the PDB recovers.
__attribute__((noinline)) int pdb_demo_compute(int a, int b) {
    return a * 3 + b * 7;
}

// The freestanding entry (so no CRT/`main` is needed). lld-link records this as
// the entry; the PDB names it `mainCRTStartup`.
int mainCRTStartup(void) {
    return pdb_demo_compute(2, 3);
}
