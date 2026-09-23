# calleevote — implementation plan

## 1. The option

`calleevote off|types|fields`, default `fields`, P4 `prototype-source`
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_calleevote.rs`). It needs the
callee-first whole-binary pass `protoorder` drives, so it lives beside it and is
inert wherever that pass is (single-function `kuna decompile`, narrowed runs,
`--jobs`, raw images, `--option protoorder off`).

## 2. The pieces

1. **Recording** (`record`). While the driver's `Ledger::recording` is set, each
   successful decompile files the function's own recovered parameters and, per
   direct CALL, the callee entry, the call address and each argument's
   `(storage, HighVariable type)`. Hooked in `kuna-console` `decompile_pulled`
   next to `protoorder`'s park.
2. **Knowing every caller** (`CallGraph::direct_call_sites`,
   `open_function_entries`, `kuna-cli` `decompile_all.rs`). Direct calls and
   tail jumps from other functions; `None` when the address is a data reference,
   a pointer-width word of any section the image loads (`SHF_ALLOC` on ELF), a
   Mach-O chained-fixup target, a dynamic relocation, an exported symbol or the
   entry; every function is open off x86-64 (an address built from two
   instructions is invisible to the one-instruction walk), in a relocatable
   object, or in an image with no section holding an entry.
3. **Deciding** (`decide_ledger`, pure, unit-tested). A parameter typed `void *`,
   `undefined1 *`, a pointer-width integer or a one-field record takes the type
   every recorded call passes, when the recorded calls equal the graph's list,
   the storage matches, and the type is the same committed pointer everywhere.
4. **Voting** (`input_vote`, from `buildLocaltypes` ahead of `ptrfromuse`). Only
   over an uncommitted fold, refused by `protoorder`'s own family refusals
   (`kuna_protoorder::input_refuses`, a new `Reading::Input`), and a `char *`
   refused over a constant stored through the value wider than a byte.
5. **Rounds** (`callee_vote_rounds`). Redo the newly decided functions in plan
   order, re-record, decide again, up to 3 rounds. A redo that fails keeps the
   first body; one that moves arity keeps the first body and forgets the
   function's records.
6. **`fields`** (`kuna_structsynth.rs` `is_lone_field`, `wants_settle_pass`,
   `points_at_lone_record`). A closed function's pointer parameter read at one
   non-zero constant offset, 4+ bytes wide, and not stepped through a loop from
   there (`address_walks`), is a one-field record; such a
   function gets one extra propagation pass so the synthesizer can run; the
   record gives way to one every caller passes.

## 3. Tests

* Unit: `kuna_calleevote/tests.rs` (12: parse, committed/uncommitted, the
  decision's agreement, storage, graph and recursion rules),
  `kuna_structsynth/tests.rs` `a_lone_field_is_one_wide_access_away_from_the_base`,
  `kuna-cli` `decompile_all.rs` `calleevote_stored_tests` (4: only an aligned
  word equal to an entry stores it, a relocatable object is open, every MIPS
  function is open, a table in a `.text` loaded at address 0 is read);
  `kuna_calleevote/tests.rs` `a_forgotten_function_keeps_no_statement`,
  `a_first_statement_that_repeats_the_callees_record_is_not_made`,
  `a_callers_record_replaces_the_callees_lone_record`,
  `a_statement_naming_a_superseded_record_is_forgotten`.
* CLI, two-pass over the whole binary path, fixture
  `decompiler/crates/kuna-analysis/tests/fixtures/calleevote_x86_64` (+ `.c`):
  `tests/cli/calleevote-off-leaves-the-callee-untyped.json`,
  `calleevote-types-takes-what-every-caller-passes.json`,
  `calleevote-fields-reads-a-closed-getters-lone-field.json` (`mark` pins the
  refused `char *` over a four-byte store), and
  `calleevote-leaves-a-relocatable-objects-callbacks-open.json` over
  `calleevote_ops_x86_64.o` (+ `.c`): callbacks in a const table filled by
  relocations keep `void *`; `calleevote-leaves-a-mips-callback-open.json` over
  `calleevote_callback_mipsel` (a callback whose address `lui`+`addiu` builds,
  also called directly, keeps `void *`); and
  `calleevote-reads-a-table-in-code-at-address-zero.json` over
  `calleevote_zero_x86_64` (a const ops table in a `.text` loaded at 0).
* Stage: `tests/stages/kuna-calleevote.xml`, the negative control (one function
  from bytes, no caller set: both passes print the raw offset), following
  `kuna-argclobber.xml`'s precedent for a whole-binary option.

## 4. Default

`fields`, per the flip procedure; the measurements are in `record.json`.
