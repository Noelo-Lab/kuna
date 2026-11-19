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
| `fileset_vcall.macho` | a two-entry MH_FILESET whose kext overrides a kernel vtable slot (below) |

`fileset_vcall.macho` (written by `gen_vcall.py`, plain Python 3) is a two-entry
arm64 MH_FILESET for `arm64evcall`: the kernel's vtable `{Base_m0, Base_m1}`
and the kext's `{Base_m0, Widget_m1}`, every slot an authenticated
`DYLD_CHAINED_PTR_64_KERNEL_CACHE` rebase (IA, address-diversified, 0x1000 for
slot 0 and 0x2001 for slot 1), and two kernel call sites: `kernel_vcall_m0`
(slot 0, pre-index `ldr`) and `kernel_vcall_m1` (slot 1, `mov x17, #8; add
x16, x16, x17`).

`gen_fileset.py` writes `fileset.macho`, `kernelcache.img4` and
`kernelcache.lzss.im4p` (Python 3 plus the reference `lzfse` CLI on `PATH`,
which encodes the IMG4's payload); the arm64 instructions are hand-encoded
there. It does not write the three `*.lzfse` vectors: those come from the
reference encoder run on the matching `.bin` (`lzfse -encode -i text.bin -o
text.bvx2.lzfse`, and likewise for the other two).

`gen_fileset_calls.py` writes `fileset_calls.macho` (Python 3 only):
`python3 gen_fileset_calls.py`.

`iokit_kext.macho` (written by `gen_iokit.py`, plain Python 3) is a stripped arm64
`MH_KEXT_BUNDLE` for the `iokitclasses` pass: one `__mod_init_func` initializer
registers `Base` and `Derived` through an unnamed constructor and stores their
metaclass vtables; each class has a vtable with its `getMetaClass()`; `Derived`
overrides `Base::m2` and adds one method. For `xnutables` it also holds a
`__kalloc_type` view for `site.Derived` (signature `121112`, 0x30 bytes),
`alloc_derived` allocating through `kalloc_type_impl` and storing into the
result, a four-record `IOExternalMethodDispatch` table that `Derived::m3`
addresses, a reader of its third function pointer, a `__kalloc_var` view for
`site.struct Packet` (a 0x10-byte header, signature `12`, then 8-byte elements)
that `alloc_packet` allocates through `kalloc_type_var_impl`, and a `__got` slot
with no indirect-symbol entry holding `Base::m1` that the function at
`0xfffffff007a04260` branches through. Only `meta_of_derived`, `Base::m1`,
`Base::m2`, the two allocators, `alloc_derived` and `alloc_packet` have symbols,
and pointers are plain absolute words.

`dext_osclassinfo.macho` (written by `gen_dext.py`) is a stripped arm64
DriverKit-style executable for the `iokitclasses` class-record reader: one
`__osclassinfo` pointer to an `OSClassLoadInformation` record for `MyDriver`
(superclass `IOService`, 0x40 bytes) with its `OSClassDescription`, metaclass
global and `New` function. The layout follows DriverKit's `OSMetaClass.h` record
declarations.
