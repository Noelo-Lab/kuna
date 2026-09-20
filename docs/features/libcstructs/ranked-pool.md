| GT struct tag | GT vars in the 444 slices | reachable: in a function that calls a libc slot naming it | of those, a slot the table ALREADY had | NEW | the new slots (slices of 444 carrying the name) |
|---|---:|---:|---:|---:|---|
| `obstack` | 431 | 371 | 0 | **371** | `_obstack_newchunk` 24 (defined), `_obstack_begin` 24 (defined) |
| `timespec` | 47 | 14 | 2 | **12** | `utimensat` 24, `futimens` 18 |
| `passwd` | 204 | 107 | 98 | **9** | `getpwent` 30 |
| `re_pattern_buffer` | 9 | 9 | 0 | **9** | `re_compile_pattern` 24, `re_compile_fastmap` 3 |
| `lconv` | 8 | 8 | 0 | **8** | `localeconv` 39 |
| `_IO_FILE` | 577 | 431 | 425 | **6** | `fread_unlocked` 24, `feof_unlocked` 22, `__getdelim` 13, `fputc_unlocked` 24 |
| `termios` | 22 | 6 | 0 | **6** | `cfgetispeed` 3, `cfgetospeed` 3, `cfsetispeed` 3, `cfsetospeed` 3 |
| `spwd` | 44 | 6 | 0 | **6** | `getspnam` 30 |
| `timeval` | 6 | 6 | 0 | **6** | `utimes` 3, `futimesat` 3 |
| `sgrp` | 61 | 3 | 0 | **3** | `getsgnam` 51 (defined) |
| `utmp` | 12 | 3 | 0 | **3** | `getutent` 6 |
| `argp_state` | 6 | 3 | 0 | **3** | `argp_error` 3 (defined) |
| `group` | 161 | 63 | 61 | **2** | `getgrent` 30 |

No new slot exists (the table already reaches everything reachable):

| GT struct tag | GT vars | reachable | covered |
|---|---:|---:|---:|
| `stat` | 469 | 99 | 99 |
| `tm` | 42 | 39 | 39 |
| `__dirstream` | 34 | 34 | 34 |
| `dirent` | 20 | 18 | 18 |

Named in libc headers, but NOT reachable by a direct call in the 444 slices
---------------------------------------------------------------------------

(`obstack` is in the table above only because this round adds the DEFINED-name
channel; by the imports-only rule it belongs here, with 0 reachable.)

| tag | GT vars | why |
|---|---:|---|
| `_ftsent` | 100 | gnulib's fts is linked in the same way, but `fts_open` is an ordinary name a program may define itself, so the reserved-namespace argument does not cover it |
| `sgrp` | 61 | shadow ships its own gshadow; `getsgnam` is likewise ordinary |
| `utmpx` | 41 | `getutxent` IS imported in 12 slices, but every variable of this type sits in a gnulib wrapper's caller rather than in the calling function |
| `argp_state` | 6 | gnulib's argp, same as fts |
| `statfs` | 4 | `fstatfs`/`statfs` imported in 45/15 slices; same one-hop gap |

Everything else in the pool is a PROGRAM-defined name a stripped binary does not
carry at all -- `hash_entry` 2,222, `hash_table` 1,639, `predicate` 453,
`tar_stat_info` 369, `hash_tuning` 352, `parser_table` 281, `fileinfo` 258, and
189 more. No declaration anywhere names them, so no table can.
