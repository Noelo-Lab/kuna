// pdb_mismatch.c — a DIFFERENT freestanding PE source, built only to produce a
// `.pdb` with a different content-hash GUID than pdb_prog's. Vendored solely as
// `pdb_prog_mismatch.pdb` to drive the s1_pdb FINGERPRINT-GATE negative test
// (a `.pdb` whose guid/age does NOT match pdb_prog.exe's CodeView record must be
// rejected — no rename). The matching EXE is not vendored.
__attribute__((noinline)) int other_function_xyz(int a) { return a + 12345; }
int mainCRTStartup(void) { return other_function_xyz(99); }
