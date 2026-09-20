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

Named in libc headers but UNREACHABLE (no slot in the 444 slices names them):

- `hash_entry` 2222 GT vars
- `hash_table` 1639 GT vars
- `predicate` 453 GT vars
- `<anon>` 372 GT vars
- `tar_stat_info` 369 GT vars
- `hash_tuning` 352 GT vars
- `parser_table` 281 GT vars
- `fileinfo` 258 GT vars
- `cp_options` 167 GT vars
- `line` 160 GT vars
- `xheader` 120 GT vars
- `name` 119 GT vars
- `change` 117 GT vars
- `_ftsent` 100 GT vars
- `directory` 88 GT vars
- `valinfo` 77 GT vars
- `tar_sparse_file` 74 GT vars
- `Src_to_dest` 69 GT vars
- `selabel_handle` 65 GT vars
- `COLUMN` 64 GT vars
- `dev_ino` 62 GT vars
- `file_data` 56 GT vars
- `mount_entry` 55 GT vars
- `Spec_list` 53 GT vars
- `error_context` 51 GT vars
- `File_spec` 48 GT vars
- `item` 47 GT vars
- `tempnode` 45 GT vars
- `keyfield` 44 GT vars
- `kwset` 44 GT vars
- `merge_node` 42 GT vars
- `utmpx` 41 GT vars
- `linebuffer` 40 GT vars
- `tree` 39 GT vars
- `option_locus` 39 GT vars
- `bin_str` 38 GT vars
- `diff3_block` 38 GT vars
- `name_elt` 38 GT vars
- `Word` 34 GT vars
- `delayed_set_stat` 32 GT vars
- `deferred_unlink` 32 GT vars
- `dir_list` 30 GT vars
- `color_ext_type` 30 GT vars
- `diff_block` 30 GT vars
- `ignore_pattern` 27 GT vars
- `huft` 27 GT vars
- `trie` 27 GT vars
- `ct_data` 26 GT vars
- `quoting_options` 24 GT vars
- `rm_options` 24 GT vars
- `List_element` 24 GT vars
- `exec_val` 24 GT vars
- `bufmap` 24 GT vars
- `buffer_record` 23 GT vars
- `devlist` 23 GT vars
- `line_filter` 23 GT vars
- `Chown_option` 22 GT vars
- `format_val` 22 GT vars
- `pending` 21 GT vars
- `delayed_link` 21 GT vars
- `namebuf` 21 GT vars
- `transform` 21 GT vars
