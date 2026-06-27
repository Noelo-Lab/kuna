// pdb_min.c — a minimal freestanding PE built with CodeView debug info so
// lld-link emits an RSDS CodeView record + the .pdb path into the PE debug
// directory (IMAGE_DIRECTORY_ENTRY_DEBUG). Fixture for the PE CodeView
// debug-record extractor (kuna-analysis s1_pdb PR-P0). Freestanding (own entry,
// no CRT) so it links with clang/lld-link on Linux without the MSVC CRT libs.
int add(int a, int b) { return a + b; }
int mainCRTStartup(void) { return add(2, 3); }
