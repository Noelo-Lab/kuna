# Kernelcache fixtures

Inputs for the `kernelcache` option (`loader/kuna_kernelcache`), and for
`tests/stages/kuna-kernelcache.xml`.

| File | What |
|---|---|
| `text.bin`, `small.bin`, `random.bin` | plain inputs |
| `text.bvx2.lzfse`, `small.bvxn.lzfse`, `random.raw.lzfse` | the same inputs encoded by the reference `lzfse -encode` (github.com/lzfse/lzfse); the encoder picks an FSE (`bvx2`), LZVN (`bvxn`) or stored (`bvx-`) block by size and entropy |
| `fileset.macho` | a two-entry arm64 MH_FILESET: `com.apple.kernel` (`kernel_add`, `kernel_dispatch`, `kernel_table`) and `com.example.driver.Widget` (`widget_start` calls `kernel_add` with a direct `bl`, `widget_via_stub` through `__stubs`/`__got`, `widget_name` returns a `__const` pointer). One `DYLD_CHAINED_PTR_64_KERNEL_CACHE` table rebases three slots, one of them authenticated. A `__PRELINK_INFO` segment (no section header, so `kernelcache off` still maps the top-level segments) holding a plist with only `_PrelinkKCID` marks it a boot collection (cache level 0); its comment line is padding a test overwrites with link keys |
| `fileset_calls.macho` | a two-entry arm64 MH_FILESET at arm64e-kernelcache addresses (base `0xfffffe0007004000`, no fixup table): `com.apple.kernel` (`kernel_main` calls `kernel_add`) and `com.example.driver.PMGR` (`pmgr_set_perf_state` calls `pmgr_wait_pending` from four `bl` sites and `kernel_add` from one; `pmgr_idle_enter` calls `pmgr_wait_pending` with a backward `bl`). Each entry has one `__TEXT,__cstring` string. For `kuna-cli/tests/xrefs_fileset_entry.rs` |
| `kernelcache.img4` | IMG4 { IM4P `krnl` { LZFSE(`fileset.macho`) }, IM4M stub } |
| `kernelcache.lzss.im4p` | a bare IM4P whose payload is a `complzss` container of `fileset.macho` |

`gen_fileset.py` writes `fileset.macho`, `kernelcache.img4` and
`kernelcache.lzss.im4p` (Python 3 plus the reference `lzfse` CLI on `PATH`,
which encodes the IMG4's payload); the arm64 instructions are hand-encoded
there. It does not write the three `*.lzfse` vectors: those come from the
reference encoder run on the matching `.bin` (`lzfse -encode -i text.bin -o
text.bvx2.lzfse`, and likewise for the other two).

`gen_fileset_calls.py` writes `fileset_calls.macho` (Python 3 only):
`python3 gen_fileset_calls.py`.
