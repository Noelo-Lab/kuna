# Kernelcache fixtures

Inputs for the `kernelcache` option (`loader/kuna_kernelcache`), and for
`tests/stages/kuna-kernelcache.xml`.

| File | What |
|---|---|
| `text.bin`, `small.bin`, `random.bin` | plain inputs |
| `text.bvx2.lzfse`, `small.bvxn.lzfse`, `random.raw.lzfse` | the same inputs encoded by the reference `lzfse -encode` (github.com/lzfse/lzfse); the encoder picks an FSE (`bvx2`), LZVN (`bvxn`) or stored (`bvx-`) block by size and entropy |
| `fileset.macho` | a two-entry arm64 MH_FILESET: `com.apple.kernel` (`kernel_add`, `kernel_dispatch`, `kernel_table`) and `com.example.driver.Widget` (`widget_start` calls `kernel_add` with a direct `bl`, `widget_via_stub` through `__stubs`/`__got`, `widget_name` returns a `__const` pointer). One `DYLD_CHAINED_PTR_64_KERNEL_CACHE` table rebases three slots, one of them authenticated. A `__PRELINK_INFO` segment (no section header, so `kernelcache off` still maps the top-level segments) holding a plist with only `_PrelinkKCID` marks it a boot collection (cache level 0); its comment line is padding a test overwrites with link keys |
| `kernelcache.img4` | IMG4 { IM4P `krnl` { LZFSE(`fileset.macho`) }, IM4M stub } |
| `kernelcache.lzss.im4p` | a bare IM4P whose payload is a `complzss` container of `fileset.macho` |

All of the fileset files are written by `gen_fileset.py` (Python 3 plus the
reference `lzfse` CLI on `PATH`); the arm64 instructions are hand-encoded there.
