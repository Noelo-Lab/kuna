# Footprint of `option indexaliasguard global` (issue #792)

`kuna decompile-all` on stripped decbench binaries, every function compared
after renaming the `vN` variables and the synthesized `struct_N` numbers to a
common numbering.

Against origin/main 72d2e9b6d, on 20 binaries (O2 bash, crazyflie `cf2.elf`,
chibios `ch.elf`, dash, dpkg, e2fsck, find, certtool, grep, gzip, kmod,
libedit, libselinux, nuttx, bootlogd, sort, tar; O0 ls, crontab, grep):

| Comparison | Functions | Changed | `goto` |
|---|---|---|---|
| origin/main 72d2e9b6d to this branch | 14,851 | 789 | 14,126 to 14,135 |
| the branch before values computed ahead of a load were handled, to the branch after it | 14,851 | 259 | 14,124 to 14,135 |
| the branch before a call after the store was handled, to this branch | 14,851 | 35 | 14,135 to 14,135 |

The first sweep, against origin/main 29c832ea9 on 23 binaries, is the one the
classes below come from:

| Corpus | Functions | Changed |
|---|---|---|
| 15 binaries: dash, gzip, bzip2 (O0, O2), sort, ls O0, crond O0, init O0, useradd and diff (O2-noinline), mirai, mydoom, xmlwf, libz, crazyflie `cf2.elf` | 5,620 | 201 |
| 8 binaries: bash (O0, O2), sshd (O0, O2), e2fsck, u-boot, certtool, betaflight | 18,982 | 606, plus 574 betaflight functions whose only change is a synthesized struct's number |

`goto` count in that sweep: 17,668 on main, 17,668 here (+1 in mydoom `sub_403f30`, -1 in bash O2).

## Classes

Every changed function is in one of these classes; the lists below name them.

- **held** -- a read of a global the binary makes before a store through a
  pointer, and uses after it, is printed as its own variable ahead of the
  store, instead of as a read of the global after it. dash `sub_8e80` reads
  `dat_1f024` once into `edi` before its list walk (`8e8b`) and prints
  `v1 = dat_1f024;` there instead of `1 <= dat_1f024` inside the loop; dash
  `sub_e3d0` (`pushstackmark`) reads both globals before storing to `*a0`.
- **operand** -- an operand now reads the global where the binary reads it
  after the store (`gi += 1` after `*p = k` becomes `v1 = gi; *p = k;
  gi = v1 + 1;`, dash `sub_4dc0`'s `subl $1` after the pointer stores keeps
  `dat_1f3e0 -= 1` instead of restoring the value read on entry, and dash
  `sub_105e0` stores the value it re-reads at `10727` into `dat_21a98` instead
  of the one it read at the top), or a pointer load keeps a variable ahead of a
  later store to a global (`v1 = *a0; gi = a1; return v1;`).
- **order** -- the same statements, with a store to a global moved to where
  the binary makes it relative to a pointer store or call (dash `sub_81c0`
  stores `dat_1f428` at `8220`, before `*a0 = ...` at `8236`; gzip `sub_d9d0`
  stores `outcnt` at `dbdc` before the byte store at `dbf4`; mirai
  `sub_4c20` stores the output pointer before the character at `4d60`/`4d67`
  and after it at `4ca8`/`4caa`, and prints both orders as the binary has
  them).
- **shape** -- a statement now sits in a condition's block, so a loop
  condition prints as `while (true) { ...; if (...) break; }` or two
  conditions stop folding into one `||` (mydoom `sub_403f30`, one more `goto`).
- **numbering** -- only a synthesized struct's number moved.

The classes are assigned by comparing the renamed statements (`held`: only
copies differ; `order`: the same statements in another order; `shape`: a
control-flow line differs; `operand`: anything else); the examples named here
were checked against the disassembly. Two kinds of store come back that main
dropped: an increment and
decrement around pointer stores (dash `sub_6bd0`, `dat_1f3e0 += 1` ... `-= 1`,
which main folded away), and a store the global overwrites later
(useradd `sub_10900` stores `dat_266b8` at `10aaf` before its copy loop).

## Changed functions

### held (190)

- `O0_coreutils_ls`: sub_1a652
- `O2-noinline_diffutils_diff`: sub_8e70, sub_c8f0, sub_19a80
- `O2-noinline_shadow_useradd`: sub_10570, sub_10ba0
- `O2_coreutils_sort`: sub_10210
- `O2_crazyflie_cf2.elf`: sub_80069f0, sub_800a0e0, sub_800a8d4, sub_800afcc, sub_800c6e4, sub_800d460, sub_800d55c, sub_800d5b0, sub_801e990, sub_8024100, sub_802590c, sub_8026044, sub_80272fc, sub_8027538, sub_8027ad4, sub_802f6fc, sub_8030b38, sub_80326d0, sub_803aa60, sub_803c648, sub_803c764, sub_803cb58, sub_803ce80, sub_804379c
- `O2_dash_dash`: sub_7dd0, sub_8e80, sub_9a20, sub_b090, sub_b2d0, sub_b7d0, sub_b9b0, sub_d880, sub_d980, sub_10230, sub_11920, sub_11d20, sub_14210, sub_16310
- `O2_gzip_gzip`: sub_6e90, sub_9ea0, sub_add0, sub_be90
- `O2_libexpat_xmlwf`: main
- `O2_mirai_mirai`: sub_44a0, sub_59e0, sub_f5e0, sub_f9d0, sub_10350
- `O2_mydoom_mydoom.exe`: sub_4013e0
- `O0_bash_bash`: sub_4eff3, sub_57f38, bind_function_def, sub_6053c, sub_607f5, run_pending_traps, complete_builtin, compgen_builtin, get_history_event, sub_1238d4, rl_insert_close
- `O2_bash_bash`: main, sub_356f0, sub_413e0, posix_initialize, get_group_list, make_arith_command, make_cond_node, make_bare_simple_command, dispose_word_desc, dispose_words, sub_485e0, sub_49310, shell_execve, sub_4f0c0, sub_50b10, sub_531b0, var_lookup, sub_55580, bind_function, bind_function_def, map_over_funcs, new_var_context, push_scope, assign_in_env, initialize_shell_variables, sub_5ab70, sub_5f130, sub_5f810, procsub_delete, append_process, initialize_job_control, sub_69bd0, ifs_firstchar, sub_70f40, sub_71dc0, clear_fifo_list, copy_fifo_list, close_new_fifos, duplicate_buffered_stream, begin_unwind_frame, add_unwind_protect, unwind_protect_mem, sub_93fd0, sub_a8ef0, sub_baab0, sh_getopt_save_istate, _rl_keyseq_cxt_alloc, sub_d2270, rl_vi_bword, sub_da270, rl_username_completion_function, _rl_scxt_alloc, sub_e2830, _rl_update_final, rl_add_undo, rl_begin_undo_group, rl_end_undo_group, rl_timeout_remaining, sub_f0760, sub_f1320, history_get_history_state, replace_history_entry, clear_history, get_history_event
- `O2_betaflight_betaflight_STM32F405.elf`: sub_800d94e, sub_80115b4, sub_80180f0, sub_80234c8, sub_8028e56, sub_80385bc, sub_803a6c2, sub_803a6ce, sub_803a6d8, sub_803a6e2, sub_803d038, sub_803d9be, sub_8041bb0, sub_80475b0, sub_804804c, sub_8049606, sub_8049a08, sub_8053ae0, sub_80549e0, sub_8058148, sub_805a6bc, sub_805b92e, sub_805b9c0, sub_805bc1c, sub_805bc4a, sub_805bc5e, sub_805bc72, sub_805d44c, sub_805d510, sub_805f860, sub_80603fc, sub_8065a2c, sub_8068d48, sub_8069d9c, sub_806d574, sub_806e254, sub_807220c, sub_8078656
- `O2_e2fsprogs_e2fsck`: sub_3ebe0, sub_46d00, ext2fs_dblist_get_last, ext2fs_zero_blocks2
- `O2_gnutls_certtool`: sub_18680
- `O2_openssh-portable_sshd`: sub_cfd0, sub_2f730, sub_44320, sub_bfe20
- `O2_u-boot_u-boot`: sub_6083a4a4, sub_60840df4, sub_6085649c, sub_608565a0, sub_60856a94, sub_60857cc8, sub_6085ac50, sub_6085b574, sub_60864dfc, sub_60871420, sub_608732f4, sub_60877630

### operand (462)

- `O0_coreutils_ls`: sub_558e
- `O2-noinline_diffutils_diff`: main, sub_7460, sub_9da0
- `O2-noinline_shadow_useradd`: sub_6d20, sub_6d90, sub_6e30, sub_114f0
- `O2_bzip2_bzip2`: sub_ca80
- `O2_coreutils_sort`: sub_7730, sub_8c20, sub_119f0
- `O2_crazyflie_cf2.elf`: sub_80069e0, sub_8008c0c, sub_800aba4, sub_800ac68, sub_800b4c8, sub_800c3c0, sub_800c938, sub_800cd80, sub_8011176, sub_8016660, sub_8016688, sub_80166c0, sub_8016778, sub_801699c, sub_8016cc4, sub_8017520, sub_8017918, sub_8017a98, sub_8018090, sub_8018d24, sub_8019238, sub_801bb4c, sub_801dca0, sub_80225dc, sub_80227f2, sub_8023acc, sub_8023e00, sub_8023ffc, sub_8024160, sub_80249d8, sub_8024c98, sub_8025628, sub_8025798, sub_8026238, sub_802aafc, sub_802ad04, sub_802bb18, sub_802cca0, sub_802d72c, sub_80310d4, sub_8031268, sub_803228c, sub_8033f14, sub_80359b0, sub_80360e4, sub_803625c, sub_8038d88, sub_803ace0, sub_803c974, sub_803cb40, sub_803d7a4, sub_803fbc0, sub_8040024, sub_8040158, sub_80404b0, sub_8040c2c, sub_80423ec, sub_8042acc, sub_8042e48, sub_8043804, sub_80440cc, sub_8044bec, sub_8044d68, sub_80450b4, sub_8045358, sub_8045420, sub_80454b0, sub_8045658, sub_804595c, sub_8045a10
- `O2_dash_dash`: sub_4dc0, sub_5cc0, sub_6bd0, sub_9480, sub_9890, sub_aa60, sub_b4c0, sub_b730, sub_b8a0, sub_bec0, sub_d5f0, sub_dc20, sub_e3d0, sub_fb20, sub_12010, sub_145c0, sub_16c40
- `O2_gzip_gzip`: sub_43b0, sub_44d0, sub_45c0, sub_4bc0, sub_5750, sub_a090, sub_aa20, sub_b160, sub_b920, sub_c590, sub_cb20, sub_cc20, sub_d9d0
- `O2_mirai_mirai`: sub_2640, main, sub_4bc0, sub_10480
- `O2_mydoom_mydoom.exe`: sub_408aa8
- `O0_bash_bash`: yyparse, push_stream, sub_39ce2, sub_3e673, tilde_initialize, get_group_array, dispose_word, dispose_word_desc, dispose_words, add_or_supercede_exported_var, sub_611fc, push_dollar_vars, sub_65798, evalexp, sub_67931, initialize_job_control, ungetc_with_restart, sub_9725b, sub_98470, sub_98637, sub_9896a, sub_acd16, sub_b192a, user_command_matches, shift_args, sub_debca, zreadc, zreadcintr, zreadn, mailstat, _rl_dispatch_subseq, sub_f8ca8, _rl_end_executing_keyseq, sub_fb9de, sub_ff79e, sub_10847e, sub_10bf4d, sub_10d8d7, sub_10e203, sub_10e3c7, rl_redisplay, _rl_erase_at_end_of_line, rl_do_undo, rl_call_last_kbd_macro, rl_insert_text, _rl_insert_char, previous_history, next_history
- `O0_openssh-portable_sshd`: main, sub_51eb2
- `O2_bash_bash`: sub_35a10, sub_35b20, push_stream, pop_stream, shell_ungets, gather_here_documents, sub_37bb0, save_parser_state, sub_3a4e0, yyparse, save_input_line_state, save_posix_options, get_group_array, sub_43380, indirection_level_string, sub_466f0, sub_47650, sub_4f720, sv_histchars, make_local_variable, push_dollar_vars, sub_5a800, sub_5a990, sub_5daa0, sub_5ddc0, evalexp, sub_5f4e0, sub_5f6a0, sub_5f9f0, kill_pid, make_child, wait_for, nohup_all_jobs, sub_66070, sub_69d50, string_list_dollar_at, reap_procsubs, setifs, sub_817b0, reset_mail_files, sub_84610, sub_85290, discard_unwind_frame, alias_expand, sub_93df0, sub_94360, sub_95900, sub_96770, bashline_reset, set_locale_var, user_command_matches, sub_9d570, sub_9dd60, sub_9f820, sub_9f8b0, sub_9f940, sub_9f9d0, shift_args, popd_builtin, pushd_builtin, set_shellopts, set_var_attribute, sub_b8540, set_bashopts, sub_bab30, printf_builtin, internal_getopt, zreadc, zreadcintr, zreadn, _rl_dispatch_subseq, rl_initialize, _rl_vi_domove_motion_cleanup, rl_vi_delete_to, rl_vi_change_to, rl_vi_yank_to, rl_vi_char_search, rl_vi_replace, rl_add_funmap_entry, sub_d73b0, rl_generic_bind, rl_bind_key, sub_e2ac0, sub_e2c10, sub_e30d0, rl_on_new_line, rl_on_new_line_with_prompt, rl_forced_update_display, rl_reset_line_state, _rl_erase_at_end_of_line, sub_e4ca0, sub_eb860, rl_modifying, rl_call_last_kbd_macro, _rl_insert_typein, rl_stuff_char, sub_f0bd0, sub_f2c50, _rl_revert_previous_lines, add_history, _hs_replace_history_data, remove_history_range, stifle_history, history_expand, read_history_range
- `O2_betaflight_betaflight_STM32F405.elf`: sub_800d694, sub_800ea84, sub_80118a0, sub_8012850, sub_8013968, sub_8013f10, sub_8013f38, sub_8013fe0, sub_8014088, sub_801a5ec, sub_801d75e, sub_8022acc, sub_8023214, sub_802335c, sub_8024198, sub_8024230, sub_8024588, sub_8024de0, sub_8025854, sub_8028164, sub_8028742, sub_8028c66, sub_8029aea, sub_802bb28, sub_802bca0, sub_802bdc0, sub_802c542, sub_802cf68, sub_802d00c, sub_802d098, sub_802ddd0, sub_802e8de, sub_802eb1a, sub_8030b40, sub_8031580, sub_8031b40, sub_803492c, sub_80349c6, sub_8037714, sub_8038384, sub_803a304, sub_803aa60, sub_803ac66, sub_803cc90, sub_803dfa6, sub_803ed28, sub_803f270, sub_8040828, sub_8040ab8, sub_8040cbc, sub_80428c0, sub_8045152, sub_80454de, sub_8047cde, sub_804950a, sub_8049592, sub_8049bec, sub_804ee88, sub_804ef84, sub_804ef96, sub_804efac, sub_804f010, sub_804f04e, sub_8050810, sub_8050996, sub_8050abc, sub_8054858, sub_8055bf0, sub_8056b66, sub_805785c, sub_8057978, sub_8059404, sub_805b048, sub_805bc96, sub_805d9b4, sub_805db54, sub_805e6a0, sub_805e890, sub_805e916, sub_805ef90, sub_805f000, sub_805f33e, sub_8064240, sub_8064b94, sub_8064dec, sub_8068c78, sub_8069ae4, sub_8069cf2, sub_806a210, sub_806a27c, sub_806a8ec, sub_806bd04, sub_806bec8, sub_806bfaa, sub_806d134, sub_806df30, sub_806e3ec, sub_806eb38, sub_806fc08, sub_8070054, sub_80700a4, sub_807153c, sub_807203e, sub_8072314, sub_807238a, sub_80725a0, sub_8072736, sub_8072a5e, sub_8072e72, sub_807354e, sub_8073636, sub_80738e2, sub_8073b10, sub_80740b8, sub_80743e6, sub_807520a, sub_8076710, sub_80788c6, sub_8078b72, sub_8078f46, sub_80793ec, sub_8079712, sub_80798ae, sub_8079d7a, sub_807a012, sub_807a1f0, sub_807a392, sub_807a53e, sub_807a88a, sub_807ab12, sub_807abfe, sub_807acba, sub_807ae04, sub_807b0e2, sub_807b306, sub_8080fda
- `O2_e2fsprogs_e2fsck`: e2fsck_pass1, quota_file_create, ext2fs_tdb_open_ex, sub_6f170, add_error_table
- `O2_gnutls_certtool`: sub_12e60, sub_1ce20, sub_1f010
- `O2_openssh-portable_sshd`: sub_12340, sub_229b0, sub_2ba50, sub_2fcc0, sub_38a10, sub_398e0, sub_3a9f0, sub_b9660, sub_b99a0, sub_bd0d0, sub_be650
- `O2_u-boot_u-boot`: sub_60800518, sub_60800be0, sub_60813080, sub_6081bf14, sub_60820140, sub_6082185c, sub_60821964, sub_6082b5bc, sub_60831604, sub_6083e6f8, sub_60845a90, sub_60846cd0, sub_60854f60, sub_60855390, sub_60855870, sub_608559a4, sub_60855e80, sub_60855f54, sub_60855f6c, sub_6085672c, sub_60856cec, sub_6085773c, sub_60859328, sub_60859fe8, sub_6085fdbc, sub_6086139c, sub_60863450, sub_6086477c, sub_608647f4, sub_6086e010, sub_608747e8, sub_60874c38, sub_60874ebc, sub_60876cfc, sub_608770e8

### order (70)

- `O2-noinline_shadow_useradd`: sub_c4d0
- `O2_coreutils_sort`: sub_7ef0
- `O2_crazyflie_cf2.elf`: sub_80262f0
- `O2_dash_dash`: sub_81c0, sub_cde0
- `O2_gzip_gzip`: sub_4a60
- `O2_mydoom_mydoom.exe`: sub_409180
- `O0_bash_bash`: restore_token_state, clear_shell_input_line, sub_3a256, sub_3dc8d, restore_parser_state, execute_command_internal, sub_90633, sub_9605a, sub_bce34, parse_string, sub_f8fff, sub_109b6d, sub_109c9f, sub_109dd1, _rl_isearch_dispatch, rl_expand_prompt, rl_on_new_line
- `O2_bash_bash`: restore_token_state, clear_shell_input_line, sub_38ba0, pop_dollar_vars, save_pipeline, sub_82310, trap_handler, sub_945e0, sub_a2170, parse_string, sub_bddc0, mailstat, rl_insert_close
- `O2_betaflight_betaflight_STM32F405.elf`: sub_800c9dc, sub_80144c8, sub_8014f60, sub_8014f76, sub_8014fe6, sub_8030588, sub_8030a5a, sub_8032638, sub_803f1a4, sub_8040ed4, sub_8043ae4, sub_804ba6c, sub_8069f54, sub_8069f92, sub_806a5f2, sub_806b934, sub_8072706, sub_807511e, sub_80786ec, sub_8079150, sub_807a49c, sub_807fc96
- `O2_openssh-portable_sshd`: sub_2bbd0, sub_3a440, sub_b9c10
- `O2_u-boot_u-boot`: sub_60857530, sub_60857ec8, sub_608591dc, sub_60859814, sub_6085ae3c, sub_608606a0, sub_60863ebc, sub_60864c44

### shape (85)

- `O2-noinline_diffutils_diff`: sub_c270
- `O2-noinline_shadow_useradd`: sub_10900
- `O2_coreutils_sort`: main
- `O2_crazyflie_cf2.elf`: sub_800efe0, sub_800f14c, sub_80164f4
- `O2_dash_dash`: sub_6e90, sub_e1c0, sub_105e0, sub_14510
- `O2_mirai_mirai`: sub_30c0, sub_4c20, sub_5a70, sub_6e90, sub_85c0, sub_90b0, sub_c680, sub_d3b0, sub_e7e0, sub_105a0
- `O2_mydoom_mydoom.exe`: sub_403f30
- `O0_bash_bash`: pop_var_context, pop_scope, getc_with_restart, sub_b806c, ulimit_builtin, sub_defec, sub_df034, sub_107737, _rl_prev_macro_key, sub_118c66, rl_stuff_char
- `O0_openssh-portable_sshd`: sub_dd51
- `O2_bash_bash`: decode_prompt_string, read_secondary_line, xparse_dolparen, execute_command_internal, find_function, find_function_def, sub_56e80, sub_573a0, add_or_supercede_exported_var, pop_scope, sub_60550, sub_628b0, stop_pipeline, sub_6f3d0, string_list_dollar_star, run_pending_traps, getc_with_restart, sub_88450, sub_93bf0, sub_97720, read_builtin, ulimit_builtin, complete_builtin, compgen_builtin, sub_d9550, sub_df000, sub_e1660, _rl_isearch_cleanup, rl_redisplay, rl_read_key, rl_insert_text, _rl_parse_colors, sub_fb130
- `O2_betaflight_betaflight_STM32F405.elf`: sub_80155c8, sub_801c9c8, sub_802d7a2, sub_80329a4, sub_805b65c
- `O2_e2fsprogs_e2fsck`: main, quota_file_open, sub_6fcb0
- `O2_openssh-portable_sshd`: main
- `O2_u-boot_u-boot`: sub_6081c204, sub_60829384, sub_60855c50, sub_60859c04, sub_6085b110, sub_6085b470, sub_6085b47c, sub_6085b488, sub_60864abc, sub_60865008

### numbering (574)

- `O2_betaflight_betaflight_STM32F405.elf`: 574 functions

## Loads across a store on a branch or in a loop

`kuna_loadorder` first counted only an op whose own output is the global's
varnode, so a store on a branch or in a loop, which reaches the global
through a `MULTIEQUAL`, did not keep a load explicit (`x = *p; if (c) gi = b;
return x + c;` printed `if (a2) gi = a1; return *a0 + a2;`). It now counts an
op whose output's HighVariable holds the global, and a load whose value is
live across a store to another global does not join a global's HighVariable.
Against the branch before that change, 58 of 9,628 functions change on 16
binaries (O2 chibios, sort, crazyflie `cf2.elf`, dash, dpkg, find, grep, gzip,
kmod, libedit, libselinux, nuttx, bootlogd, tar; O0 crontab, grep). In every
one a load through a pointer is printed as its own variable ahead of a store to
a global it used to print after; 57 keep the same stores to globals, and
crazyflie `sub_802cca0` loses four early writes: the binary loads `[r4, #76]` at
`802ccfe`, stores the constant at `802cd0a` and the product at `802cd16`, which
printed as `dat_1000d228 = a0[0x13]; ... dat_1000d1fc = 0x3f800000;
dat_1000d228 = dat_1000d228 * v12;` and now prints `v6 = a0[0x13]; ...
dat_1000d1fc = 0x3f800000; dat_1000d228 = v6 * v12;`.

- `O2_chibios_ch.elf`: sub_8000dc8, sub_800397c
- `O2_coreutils_sort`: main
- `O2_crazyflie_cf2.elf`: sub_800bfb8, sub_800cd80, sub_801336c, sub_801df88, sub_8023398, sub_80237b4, sub_8023820, sub_8023e00, sub_80262f0, sub_8029494, sub_802961c, sub_802cca0, sub_802e620, sub_802f6fc, sub_8030b38, sub_8031268, sub_803228c, sub_80326d0, sub_80335b8, sub_80340bc, sub_803c228, sub_803c398, sub_803c514, sub_803ca18, sub_803caa4, sub_803d168, sub_803d290, sub_803d3e4, sub_80423ec, sub_80440cc, sub_8044d68
- `O2_dash_dash`: sub_6620, sub_68c0, sub_6fe0, sub_16ab0, sub_16c40
- `O2_dpkg_dpkg`: main, sub_192b0, sub_23270, sub_2f5b0
- `O2_findutils_find`: sub_c9f0, sub_d820
- `O2_gzip_gzip`: sub_4bc0
- `O2_libedit_libedit.so.0.0.70`: sub_15380, sub_16c30, sub_16e90, sub_18410, fn_filename_completion_function
- `O2_libselinux_libselinux.so.1`: sub_129a0
- `O2_tar_tar`: main, sub_e570, sub_10cf0, sub_27eb0, sub_27f40, argp_parse

## Values computed before a pointer load

A value merged into a global's variable printed as a write of the global where
the value is computed, so a pointer load between that point and the binary's
store read the new value (`t = gi * 2 + b; x = *p; gi = t;` printed
`gi = a1 + gi * 2; v1 = *a0;`). Merge now refuses that join, `mergeOp` copies
such a value out of a `MULTIEQUAL`, and `RulePropagateCopy` keeps a store's
`COPY` in a marker when a pointer load follows the store. Against the branch
before that change, 259 of 14,851 functions change on the 20 binaries above
(`goto` 14,124 to 14,135). The changes seen, each checked against the disassembly
in the functions named:

- **store after the load** -- a store to a global that the binary makes after
  a pointer load no longer prints ahead of it. gzip `sub_5690` loads the byte
  at `56d1` and stores `inptr` (`dat_1a008`) at `56d6`; it printed
  `V0 = dat_1a008; dat_1a008 += 1; V1 = *(V0 + 0x9c000);` and now prints
  `V0 = *(dat_1a008 + 0x9c000); dat_1a008 += 1;` (also `sub_8460`, `sub_92b0`,
  `sub_9a50`, `sub_b320`). crazyflie `sub_800d5b0` stores `dat_2000281c` at
  `800d5c0` before `dat_20002026`; it printed the second store first.
- **register-carried global** -- a global the binary keeps in a register
  around a loop and stores after it prints that way, instead of as a write of
  the global in every iteration. gzip `sub_d0d0` (`updcrc`) keeps the CRC in
  `rdx` and stores it once at `d121`; it printed
  `dat_183f0 = dat_183f0 >> 8 ^ ...` in the loop and now prints
  `V0 = dat_183f0; do { V0 = V0 >> 8 ^ ...; } while (...); dat_183f0 = V0;`
  (also `sub_d640`, `sub_4960`).
- **store kept in place** -- the store's `COPY` stays where the binary makes
  it, which also drops write-backs of a value the global already held. tar
  `sub_f710` stores `dat_81f38` at `f7c2` before `dat_82160` at `f7cd`; bash
  `find_global_variable_last_nameref` printed `global_variables = v4;` on six
  exits that the binary does not store, and `sub_4ef30` printed
  `breaking = 0;` and `breaking = v3;` where the binary only decrements
  `breaking` (`4f073`).
- **shape** -- the same statements under different control flow. bash
  `sub_49eb0`'s list walk prints its two exits in the other order; both forms
  follow the jumps at `49f1a`/`49f23`. The `goto` count moves in 12 functions
  (bash 8, find 1, tar 3).
- **store sunk to the exits** -- dash `sub_9890` keeps `dat_1f3e0` in `eax`
  around its free loop and stores it before and after `free` (`98d3`,
  `98f2`); the decrement's store now prints at the loop's exits and in the
  next iteration's increment instead of in the loop body. Nothing reads the
  global in between: the loads there are of `dat_1f570` at a constant
  address and of `dat_1f3dc`.

The changed functions:

- `O0_coreutils_ls`: main
- `O0_cronie_crontab`: sub_ab10
- `O0_grep_grep`: main
- `O2_bash_bash`: main, sub_33a30, sub_33ce0, sub_35a10, sub_35b20, restore_parser_state, sub_38ba0, sub_38f20, sub_3a4e0, parse_string_to_word_list, sub_3dc90, yyparse, bash_tilde_expand, sub_485e0, sub_48d40, sub_49eb0, execute_command_internal, sub_4ef30, sub_4fa90, sub_51b80, sub_52630, sub_52ec0, sub_532f0, find_global_variable_last_nameref, find_global_variable_noref, find_global_variable, find_shell_variable, make_local_variable, sub_55580, bind_variable, kill_all_local_variables, reset_local_contexts, pop_dollar_vars, assign_in_env, unbind_variable, unbind_nameref, unbind_variable_noref, unbind_global_variable, unbind_global_variable_noref, initialize_shell_variables, sub_5c650, sub_5daa0, sub_5f350, sub_60900, restore_pipeline, sub_628b0, wait_for, wait_for_background_pids, stop_pipeline, sub_67d70, sub_69bd0, sub_69d50, skip_to_histexp, reap_procsubs, sub_79210, sub_7b800, expand_subscript_string, sub_7dfc0, sub_82310, run_pending_traps, restore_traps, getc_with_restart, sub_85290, sub_88080, sub_88450, test_command, unbind_array_element, brace_expand, sub_93bf0, sub_93df0, sub_95460, sub_96150, sub_96770, sub_97720, command_word_completion_function, sub_99c40, set_lang, set_locale_var, gen_compspec_completions, builtin_builtin, fc_builtin, help_builtin, jobs_builtin, popd_builtin, pushd_builtin, read_builtin, set_or_show_attributes, printf_builtin, sh_getopt, internal_getopt, compgen_builtin, readline_internal_char, rl_restore_state, rl_vi_unix_word_rubout, sub_d24e0, rl_vi_fWord, rl_vi_eWord, rl_vi_eword, _rl_vi_domove_motion_cleanup, rl_prep_terminal, _rl_find_completion_word, rl_old_menu_complete, rl_menu_complete, sub_e1660, _rl_isearch_cleanup, rl_on_new_line_with_prompt, sub_e4ca0, rl_redisplay, rl_tilde_expand, rl_unix_filename_rubout, _rl_pop_executing_macro, _rl_rubout_char, rl_delete_horizontal_space, rl_maybe_unsave_line, rl_replace_from_history, history_truncate_file, _rl_print_prefix_color, sub_fb130
- `O2_chibios_ch.elf`: sub_8001188, sub_8004b74
- `O2_coreutils_sort`: main, sub_74e0, sub_a3c0
- `O2_crazyflie_cf2.elf`: sub_800aba4, sub_800afcc, sub_800d3fc, sub_800d460, sub_800d55c, sub_800d5b0, sub_800dee8, sub_8010c70, sub_8011744, sub_80171c4, sub_801777c, sub_8018090, sub_8018f68, sub_8023398, sub_80237b4, sub_8023820, sub_8023e00, sub_8023ffc, sub_8024278, sub_80273a8, sub_80288ba, sub_8029494, sub_802961c, sub_802ae70, sub_802cca0, sub_8030b38, sub_8031268, sub_803228c, sub_80326d0, sub_80335b8, sub_8033f14, sub_80340bc, sub_8035854, sub_8036aa8, sub_8039238, sub_803bc64, sub_803c398, sub_803fbc0, sub_80404b0, sub_8040c2c, sub_80423ec, sub_8042acc, sub_8042e48, sub_8043804, sub_80440cc, sub_8044d68, sub_80450b4, sub_8045658, sub_8045a10
- `O2_dash_dash`: sub_5140, sub_5520, sub_5cc0, sub_69b0, sub_6bd0, sub_6e90, sub_7210, sub_7400, sub_9890, sub_9f60, sub_bb50, sub_d0f0, sub_d5f0, sub_f8c0, sub_fcb0, sub_105e0, sub_12f80, sub_14510, sub_145c0, sub_16570, sub_16c40
- `O2_dpkg_dpkg`: main, sub_c6a0, sub_118e0, sub_13ed0, sub_148f0, sub_192b0, sub_1ec20, sub_20000, sub_23270, sub_25190, sub_2a610
- `O2_e2fsprogs_e2fsck`: main, ext2fs_group_desc, ext2fs_dblist_sort
- `O2_findutils_find`: sub_7670, sub_c110, sub_1bb50
- `O2_gnutls_certtool`: sub_2ce80
- `O2_grep_grep`: main, sub_9130
- `O2_gzip_gzip`: sub_4960, sub_4a60, sub_4bc0, sub_5690, sub_6e90, sub_8460, sub_92b0, sub_9a50, sub_aa20, sub_b320, sub_be90, sub_c590, sub_d0d0, sub_d640
- `O2_libedit_libedit.so.0.0.70`: sub_15380, fn_filename_completion_function
- `O2_libselinux_libselinux.so.1`: sub_98b0, avc_destroy, sub_129a0, sub_1dc30, selinux_status_updated
- `O2_nuttx_nuttx`: sub_8001da4, sub_800233c, sub_800c62c, sub_80108a8
- `O2_tar_tar`: main, sub_e570, sub_ec80, sub_f710, sub_10cf0, sub_149e0, sub_1b9b0, sub_203b0, sub_222e0, sub_270f0, sub_27340, sub_28c00, sub_308a0, sub_31150, sub_31920, argp_parse, sub_44440, sub_5aa30

## A call after the store

A value computed before a pointer load and stored to a global before a call
reached the global through the call's `INDIRECT`, which `Merge::mergeIndirect`
merged with the global directly, so the store still printed ahead of the load
(`t = gi + b; x = *p; gi = t; if (c) touch();` printed
`gi = a1 + gi; v1 = *a0;`). `mergeIndirect` now snips that `INDIRECT` instead,
and the store prints just ahead of the call, when `RulePropagateCopy` gave the
`INDIRECT` the stored value from a `COPY` that no load follows. Against the
branch before that change, 35 of 14,851 functions change on the 20 binaries above and the `goto`
count stays at 14,135. The changes seen, each checked against the disassembly in
the functions named:

- **store after the load** -- the store prints after the loads the binary
  makes before it. dash `sub_68c0` loads `0x8(%rdi)` at `68eb` and stores
  `dat_1f3d8` at `68f4`; it printed `dat_1f3d8 = a0[1];` ahead of that load
  (also `sub_6620`, `sub_6fe0`, `sub_b090`, `sub_b2d0`, `sub_e420`). bash
  `yyparse` loads `-0x20(%r8)` and `(%r8)` before storing `dat_143b48` and
  `dat_143b40` (`3ec74`, `3ec7b`); it printed `dat_143b48 = V[-4];` ahead of
  the second load. The same holds in bash `sub_33a30` (`33b73` before
  `33b7b`), `pop_stream` (`35f1d` before `35f2b`), `printf_builtin` (`bbabe`
  before `bbac1`), `readline_internal_char` (`d0fcf` before `d0fd3`),
  `rl_filename_completion_function` (`d8b6d` before `d8b71`) and
  `history_truncate_file` (`f8308` before `f830a`), sort `main` (`620b`
  before `6210`), tar `sub_19da0` (`19ebf` before `19ec6`), dpkg `sub_c4c0`
  (`c4d4` before `c4d8`) and chibios `sub_8004b74`, where each call's result
  is stored after the next call's argument is loaded (`8004bd4`, `8004bd8`).
- **stored once** -- a value set on several branches is stored once after
  them, as the binary does. bash `sub_97720` keeps 0, 1 or 2 in `ebx` and
  stores `dat_14b7f0` once at `97779`; it printed a store on each branch,
  the first ahead of the load of `a0[1]` (also `sub_37bb0`'s
  `history_quoting_state`). crazyflie `sub_8036aa8` sums a message's bytes in
  `r3` and stores the sum once after the loop (`8036b18`); it printed the
  store in every iteration.
- **shape** -- two loads print in the other order with no store between them
  (dash `sub_16570`); crazyflie `sub_8042e48` is data decoded as code.

Two guards keep the snip to that case. A `COPY` of the global itself is not
kept apart: an earlier trim can leave one as the `INDIRECT`'s input, and kept
apart it printed a save of sixteen globals after a call and their restore
before the next (bash `sub_38f20`). And an `INDIRECT` input that reached the
call another way says nothing about where the binary stored it: crazyflie
`sub_80335b8` stores `dat_2000c528` at `8033604` and loads `a1[0xf]` at
`803361e`, then stores again on a branch; the `INDIRECT` at the call reads the
register that carries both values, and snipping it printed one store after
both loads. That function prints as before.

The changed functions:

- `O2_bash_bash`: sub_33a30, pop_stream, sub_37bb0, yyparse, sub_97720, printf_builtin, readline_internal_char, rl_filename_completion_function, _rl_init_terminal_io, history_truncate_file
- `O2_chibios_ch.elf`: sub_8004b74, sub_8005118, sub_8007064
- `O2_coreutils_sort`: main
- `O2_crazyflie_cf2.elf`: sub_800c938, sub_800d460, sub_800d55c, sub_800d5b0, sub_80237b4, sub_8023820, sub_80340bc, sub_8036aa8, sub_8042e48
- `O2_dash_dash`: sub_6620, sub_68c0, sub_6fe0, sub_b090, sub_b2d0, sub_e420, sub_16570
- `O2_dpkg_dpkg`: sub_c4c0, sub_c560, sub_20440
- `O2_tar_tar`: sub_19da0, sub_27fd0
