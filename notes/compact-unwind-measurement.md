# Mach-O `__unwind_info` as a function-start source

`unwindstarts` (default off) reads the second-level pages of
`__TEXT,__unwind_info` and adds each recorded function start to discovery. This
note records the measurement behind that default.

## Images that keep `LC_FUNCTION_STARTS`: nothing to gain

On every image that still carries `LC_FUNCTION_STARTS`, no `__unwind_info` start
is missing from it. The unwind set is a strict subset: the linker writes one
entry for a run of adjacent functions with the same compact encoding, so many
entries span several functions. That makes unwind-derived bounds coarser than
the function-starts table, never finer.

| Binary | Arch | `LC_FUNCTION_STARTS` | `__unwind_info` starts | Only in unwind | Unwind entries spanning >1 function |
|---|---|---:|---:|---:|---:|
| Toyota | arm64 | 296,775 | 185,936 | 0 | 44,624 |
| Chase | arm64 | 708,530 | 460,828 | 0 | 101,087 |
| Costco | arm64 | 181,805 | 117,613 | 0 | 26,564 |
| NavyFederal | arm64 | 80,543 | 50,728 | 0 | 11,838 |
| ios_kcm.arm64.macho | arm64 | 5,318 | 3,192 | 0 | 835 |
| backboardd | arm64e | 2,999 | 2,016 | 0 | 441 |
| trustd | arm64e | 1,123 | 808 | 0 | 164 |
| debugserver | arm64e | 907 | 636 | 0 | 155 |
| forkfix.dylib | arm64e | 58 | 30 | 0 | 9 |
| libCoreKE.dylib | arm64e | 407 | 288 | 0 | 71 |
| libRPAC.dylib | arm64e | 100 | 60 | 0 | 15 |
| idownloadd | arm64 | 432 | 254 | 0 | 55 |
| MachOMerger | arm64 | 961 | 583 | 0 | 159 |
| jbctl | arm64e | 17 | 10 | 0 | 3 |
| libjailbreak.dylib | arm64e | 341 | 60 | 0 | 22 |
| launchdhook.dylib | arm64e | 103 | 77 | 0 | 17 |
| opainject | arm64e | 69 | 51 | 0 | 11 |
| systemhook.dylib | arm64e | 116 | 73 | 0 | 24 |
| AppleAccount | arm64e | 3,576 | 1,812 | 0 | 521 |
| AppleIDAuthSupport | arm64e | 129 | 78 | 0 | 21 |
| ApplePushService | arm64e | 756 | 417 | 0 | 116 |
| AuthKit | arm64e | 4,397 | 2,017 | 0 | 495 |
| IDS | arm64e | 5,340 | 3,316 | 0 | 637 |
| IDSFoundation | arm64e | 7,396 | 3,459 | 0 | 893 |
| IMCore | arm64e | 7,070 | 4,234 | 0 | 873 |
| Security | arm64e | 6,080 | 4,322 | 0 | 861 |
| MobileSMS | arm64e | 545 | 394 | 0 | 79 |
| apsd | arm64e | 3,395 | 1,966 | 0 | 508 |
| identityservicesd | arm64e | 13,851 | 8,648 | 0 | 1,804 |
| imagent | arm64e | 1,247 | 997 | 0 | 105 |

SpringBoard, lsd and boomerang (16 KB extracts with 1–2 starts each) also show 0.
On Toyota, `kuna functions` finds 314,038 functions with the option off and the
same 314,038 with it on.

The compact encodings add nothing for kuna either. Toyota has 123,580
frame-based, 43,202 frameless (43,162 with stack size 0), 18,865 DWARF-mode and
290 empty encodings, and none with an LSDA bit set. S5/S7 recovers the frame
from the code, exception tables come from `__eh_frame`, and the encoding has no
noreturn bit.

## Images without `LC_FUNCTION_STARTS`: the Instagram case

The decrypted 249 MB arm64 Instagram.app has no `LC_FUNCTION_STARTS`. The default
inventory has 59,222 entries, but only 23 of them are in `__text`: 36,177 are
`__got` slots and 23,019 are `__stubs`. arm64 has no default prologue sweep, so
`LC_MAIN`, the exports and the 40,842-entry symbol table are the only seeds.
`__unwind_info` records 204,800 starts in `__text`; sampled ones begin with real
prologues (`stp x29, x30`, `sub sp, sp`).

| Instagram | Functions | Unwind starts found | Time | RSS |
|---|---:|---:|---:|---:|
| `unwindstarts` off | 59,222 | 1 of 204,800 | 5.66 s | 1.41 GB |
| `unwindstarts` on | 264,021 | 204,800 of 204,800 | 6.99 s | 1.61 GB |

The option stays out of the `aggressive` preset: it is inert where
`LC_FUNCTION_STARTS` exists, and the stripped apps it helps are above the 500 KiB
size at which `auto` selects `aggressive`.
