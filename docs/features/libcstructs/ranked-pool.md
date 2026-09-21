| GT struct tag | GT vars in the 444 slices | reachable: in a function that calls a libc slot naming it | of those, a slot the table ALREADY had | NEW | the new slots (slices of 444 carrying the name) |
|---|---:|---:|---:|---:|---|
| `obstack` | 431 | 371 | 0 | **371** | `_obstack_newchunk` 24 (defined), `_obstack_begin` 24 (defined) |
| `timespec` | 47 | 14 | 2 | **12** | ~~`utimensat` 24, `futimens` 18~~ dropped (two-element array slot) |
| `passwd` | 204 | 107 | 98 | **9** | `getpwent` 30 |
| `re_pattern_buffer` | 9 | 9 | 0 | **9** | `re_compile_pattern` 24, `re_compile_fastmap` 3 |
| `lconv` | 8 | 8 | 0 | **8** | `localeconv` 39 |
| `_IO_FILE` | 577 | 431 | 425 | **6** | `fread_unlocked` 24, `feof_unlocked` 22, `__getdelim` 13, `fputc_unlocked` 24 |
| `termios` | 22 | 6 | 0 | **6** | `cfgetispeed` 3, `cfgetospeed` 3, `cfsetispeed` 3, `cfsetospeed` 3 |
| `spwd` | 44 | 6 | 0 | **6** | `getspnam` 30 |
| `timeval` | 6 | 6 | 0 | **6** | ~~`utimes` 3, `futimesat` 3~~ dropped (two-element array slot) |
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

Kept off the pool
-----------------

Every kept row not named in the table above, one line each: what it is there
for, how many of the 444 slices carry it, what it reaches, and what it costs.
"reach" is GT pointer variables of its type in a function that calls it
(any reach listed here is already reached by a shipped slot, which is why the
row is off-pool). "changed" is functions calling it whose text differs in the
444-slice `decompile-all` diff, and `+i/-w` the scored functions calling it
that improved/got worse (co-occurrence, not ablation). Corpus-wide, over all
663 x86-64 binaries, no row splits a frame object or loses a named local.

| row | why | slices | reach | changed | +i/-w | also costs / buys |
|---|---|---:|---:|---:|---:|---|
| `rewinddir` | DIR surface | 15 | 0 | 0 | +0/-0 | off-pool. |
| `fgetgrent` | account-db family | 21 | 0 | 42 | +0/-0 | off-pool. |
| `fgetpwent` | account-db family | 21 | 0 | 63 | +0/-0 | off-pool. |
| `fgetspent` | account-db family | 21 | 0 | 21 | +0/-0 | off-pool. |
| `getgrgid_r` | account-db family | 27 | 0 | 27 | +0/-0 | off-pool. |
| `getgrnam_r` | account-db family | 27 | 0 | 27 | +0/-0 | off-pool. |
| `getpwnam_r` | account-db family | 30 | 0 | 30 | +0/-0 | off-pool. |
| `getpwuid_r` | account-db family | 27 | 0 | 27 | +0/-0 | off-pool. |
| `getspent` | account-db family | 0 | 0 | 0 | +0/-0 | off-pool. |
| `getspnam_r` | account-db family | 3 | 0 | 3 | +0/-0 | off-pool. |
| `putgrent` | account-db family | 42 | 0 | 42 | +0/-0 | off-pool. |
| `putpwent` | account-db family | 48 | 0 | 48 | +0/-0 | off-pool. |
| `putspent` | account-db family | 42 | 0 | 42 | +0/-0 | off-pool. |
| `sgetspent` | account-db family | 42 | 0 | 42 | +0/-0 | off-pool. |
| `wcrtomb` | mbstate_t (by-value pool) | 6 | 0 | 12 | +3/-0 | off-pool. buys 3 GT `__mbstate_t` by-value vars |
| `_obstack_begin_1` | obstack entry point | 24 def | 0 | 0 | +0/-0 | off-pool. |
| `_obstack_free` | obstack entry point | 24 def | 0 | 34 | +0/-0 | off-pool. |
| `_obstack_memory_used` | obstack entry point | 24 def | 0 | 0 | +0/-0 | off-pool. |
| `pthread_mutex_destroy` | pthread_mutex_t surface | 3 | 0 | 5 | +0/-0 | off-pool. |
| `pthread_mutex_init` | pthread_mutex_t surface | 3 | 0 | 8 | +0/-0 | off-pool. 2 calls gain the alloca trailing argument (libselinux) |
| `re_match` | regex family | 12 | 0 | 14 | +3/-0 | off-pool. 2 callers print `dat_4`; buys 5 args at 2 one-argument calls |
| `re_search` | regex family | 18 | 0 | 21 | +3/-0 | off-pool. |
| `regcomp` | regex family | 9 | 0 | 9 | +0/-0 | off-pool. |
| `regerror` | regex family | 0 | 0 | 0 | +0/-0 | off-pool. |
| `regexec` | regex family | 9 | 0 | 9 | +0/-0 | off-pool. |
| `regfree` | regex family | 12 | 0 | 15 | +3/-0 | off-pool. |
| `fstatfs` | statfs (by-value pool) | 45 | 0 | 45 | +3/-0 | off-pool. buys whole `statfs` locals (5 GT by-value vars) |
| `statfs` | statfs (by-value pool) | 15 | 0 | 15 | +2/-0 | off-pool. buys whole `statfs` locals (with `fstatfs`); the `statfs` name collision, above |
| `fgets_unlocked` | stdio surface | 3 | 3 | 3 | +0/-0 | off-pool. |
| `fseeko` | stdio surface | 306 | 11 | 315 | +0/-0 | off-pool. 98 `-O0` wrappers print `dat_4`; buys 3 args at 196 one-argument calls |
| `ftello` | stdio surface | 9 | 3 | 3 | +1/-0 | off-pool. |
| `setbuf` | stdio surface | 3 | 0 | 3 | +0/-0 | off-pool. |
| `vfprintf` | stdio surface | 12 | 3 | 17 | +0/-0 | off-pool. |
| `tcgetattr` | termios family | 15 | 0 | 9 | +0/-0 | off-pool. |
| `asctime` | tm/timespec surface | 0 | 0 | 0 | +0/-0 | off-pool. |
| `clock_getres` | tm/timespec surface | 3 | 0 | 3 | +0/-0 | off-pool. |
| `clock_settime` | tm/timespec surface | 3 | 0 | 3 | +0/-0 | off-pool. |
| `gmtime` | tm/timespec surface | 15 | 6 | 24 | +0/-0 | off-pool. |
| `gmtime_r` | tm/timespec surface | 36 | 0 | 47 | +0/-0 | off-pool. |
| `mktime` | tm/timespec surface | 18 | 0 | 18 | +0/-0 | off-pool. |
| `nanosleep` | tm/timespec surface | 9 | 0 | 9 | +0/-0 | off-pool. 21 wrappers print `dat_4`; `sleep`'s `timespec` stops splitting |
| `timegm` | tm/timespec surface | 9 | 0 | 9 | +0/-0 | off-pool. |
| `getutid` | utmp family | 0 | 0 | 0 | +0/-0 | off-pool. |
| `getutline` | utmp family | 0 | 0 | 0 | +0/-0 | off-pool. |
| `pututline` | utmp family | 3 | 0 | 3 | +0/-0 | off-pool. |
| `getutxent` | utmpx family | 12 | 0 | 12 | +0/-0 | off-pool. |
| `getutxid` | utmpx family | 0 | 0 | 0 | +0/-0 | off-pool. |
| `getutxline` | utmpx family | 0 | 0 | 0 | +0/-0 | off-pool. |
| `pututxline` | utmpx family | 0 | 0 | 0 | +0/-0 | off-pool. |
| `LIBC_IMPORTED_OBSTACK` (5) | the glibc import channel of the obstack entry points | 0 (15 `dpkg` slices outside the 444) | 0 | 0 | +0/-0 | off-pool. Spells glibc's `int` size slots; moves 7 lines of `-O2` dpkg-query. |

Dropped after round 4 (not in the table): `utimensat`, `futimens`, `utimes`,
`futimesat` (the slot is a two-element array, and naming one element shrinks the
caller's frame object), and `sigfillset`, `sigsuspend`, `pthread_sigmask`,
`sigdelset`, `sigismember`, `sigwait` (each splits a `struct sigaction` local
handed `&sa.sa_mask`). See `analysis.md`.

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
