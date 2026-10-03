# bejoin: plan (as built)

Behind `option bejoin` (default on; off is byte-identical to main). It changes
only two-register returns and call outputs whose output rule joins the first
register high (every big-endian target, and AVR's gcc spec through
`reversesignif`); every other target is byte-identical. The rule is a prior:
an `int` holding the high word of a 64-bit temporary, its stale low half left
in the second register, reads as the `long long` with the same registers
(`analysis.md`, "Why it is an option").

1. `ParamActive::join_pair_order` (`p4_calls/fspec.rs`) names which of the first
   two used trials holds the low half, from the flag the matched output rule
   sets. `ParamListStandard::holds_high_first` / `FuncProto::output_holds_high_first`
   read the same order back from where a joined value's halves sit.
2. `kuna_bejoin::join_order`, called by `ActionReturnRecovery` once the output
   map is derived, classifies the second register at every live RETURN
   (`classify`: `Wide`, `Returned`, `Zero`, `Nothing`, `Entry`, `Scratch`,
   `Compared` -- a computed 0/1 in the first register, or a sum or difference
   in the low word that no carry ties a non-literal first register to
   (`computed_flag`, `nonzero_bits`, `sum_or_difference`, `carry_free`), or
   one whose tested carry sits beside a first register made of flags and
   literals (`carries_of`, `tests_carry`, `of_flags`); `Truncated` -- an
   `int` reloaded from a stack temporary low word first
   (`loads_low_word_first`); `Quotient` -- the low half of a product whose
   high half the first register shifts down, a division by a constant
   (`divided_down`); `ShiftedOut` -- a right shift whose dropped bits reach
   the first register, a 64-bit temporary held the reverse way round
   (`shifts_into_first`, following stack reloads through `stack_stores`);
   `Reworked` -- an `int` worked out from the high half of a 64-bit
   temporary: with the two registers paired through moves and same-block
   choices (`reworked`), a low-word sum or difference whose carry or borrow
   reaches the first register other than as that sum's own high word, each
   flag entering with its sign (`carried_into`, `carry_signs`, `value_key`,
   `Terms::walk`), or, with no carry between them, a first register that is a
   literal operation on a value the low word never reads (`works_over`,
   `worked_operand`, `sources`, `reads`); `ProductHigh` -- the high half of a
   product beside a first register that reads its low half
   (`product_high_half`, `reads_low_half`);
   `ends_of` walks back with the low word's byte offset, `only_returned` walks
   forward, `halves_of_one_value`, `moves_register_window`, `never_reached`) and
   returns the ABI's order only when the option is on and some RETURN is `Wide`
   or `Returned` and none is `Entry`, `Scratch`, `Compared`, `Truncated`,
   `Quotient`, `ShiftedOut`, `Reworked` or `ProductHigh`, and a `Returned` one counts as a veto when the
   code behind an indirect jump flow could not follow, or an operation given
   no register inputs (an inline system call), can read it (`hidden_readers`,
   `reaches_jump`: an input, a literal or a value made in a block that
   dominates the reader);
   otherwise
   `(0, 1)`, main's order.
   `build_return_output` builds `PIECE(hi, lo)` and its join address from that
   order.
3. `Funcdata::kuna_pairs_first_low` records a function whose pair kept the old
   join; `build_output_from_trials` then builds the call-output pairs in that
   function first register low too, and `ParamActive::join_pair_order` order
   elsewhere (`kuna_rustabi::build_call_output_pair` takes the order).
4. `kuna_returnuncomputed::strip_uncomputed_return_piece` skips a pair joined
   first register high (`kuna_bejoin::first_register_holds_high`); otherwise it
   is main's code. `kuna_rustabi::holds_scalar_pair` maps the PIECE's halves back
   to register order before asking for the Rust tag.
5. `kuna_rustabi::pair_join_address` keeps a join record when the contiguous
   pair's parent register is global storage (AVR `R25R24`), with the option on.
6. `option bejoin` (`phases.toml`, `Architecture::be_join` copied to
   `ArchContext::be_join`, `kuna_bejoin::live`) gates steps 2-5: off, every
   pair and call pair joins first register low and nothing else moves.

Tests: `kuna_bejoin/tests.rs` (the classification on hand-built IR);
`kuna-cli/tests/decompile_all_cli.rs` compiled round trips for wide values
(`bejoin_*.o`, `bejoin_carry_*`, `bejoin_avr.bin`, `bejoin_window64_*`) and for
functions that return one register and must keep it (`bejoin_zero_mips32_O0.o`,
`bejoin_window_sparc32_*`, `bejoin_narrow_sparc32_*`, the ARM big-endian
comparisons in `bejoin_cmp_arm32_be_O{0,2}.o`, the range checks in
`bejoin_trunc_*` and `bejoin_code_*`, the divisions by constants in
`bejoin_div_arm32_be_*`, the reverse-pair shifts in `bejoin_shr_arm32_be_*`,
the unrecovered switch in `bejoin_jump_mips32_O2.o`, the reworked
temporaries and products in `bejoin_hiop_*`), and the option's trade-off read
both ways (`bejoin_tradeoff_ppc32_O2.o`, `bejoin_tradeoff_mips32_O0.o`); stages
`kuna-bejoin{,-sparc,-sparc64,-mips,-avr}.xml`, `kuna-bejoin.xml` with an off
and an on pass.
