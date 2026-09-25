//! Unit tests for the slot table and the run's record. The end-to-end witness
//! -- a comparator declared by the `qsort` call that names it -- is the CLI
//! probe pair over `callbacktype_x86_64`; the stage test
//! `tests/stages/kuna-callbacktype.xml` is the negative control for the
//! one-function path.

use super::*;

fn fact(value: uintb, at: uintb, declared_by: &'static str, sig: Sig) -> Fact {
    Fact { value, declared_by, sig, at }
}

fn ledger(rows: &[((int4, uintb), Vec<Fact>)]) -> Ledger {
    let mut l = Ledger::default();
    for (caller, facts) in rows {
        l.by_caller
            .insert(*caller, CallerFacts { facts: facts.clone(), ..Default::default() });
    }
    l
}

#[test]
fn option_parses_both_values_and_rejects_anything_else() {
    assert_eq!(OptionCallbackType.apply("on").unwrap().0, CallbackTypeMode::On);
    assert_eq!(OptionCallbackType.apply("off").unwrap().0, CallbackTypeMode::Off);
    assert!(OptionCallbackType.apply("types").is_err());
    assert!(CallbackTypeMode::On.is_on());
    assert!(!CallbackTypeMode::Off.is_on());
    assert_eq!(CallbackTypeMode::default(), CallbackTypeMode::Off);
}

/// The table names the parameter the LIBRARY declares as a function pointer,
/// and its own parameter list reaches that far, which is what the prototype
/// model is asked for the argument register.
#[test]
fn every_slot_index_is_inside_the_entry_points_own_parameter_list() {
    for e in SLOTS {
        assert!(!e.slots.is_empty(), "{} declares no slot", e.name);
        for (slot, sig) in e.slots {
            assert!(
                *slot < e.params.len(),
                "{}: slot {slot} is past its {} declared parameters",
                e.name,
                e.params.len()
            );
            assert_eq!(e.params[*slot], Ty::VoidPtr, "{}: slot {slot} is not a pointer", e.name);
            assert!(
                sig.params.iter().all(|t| *t != Ty::Void),
                "{}: a callback parameter cannot be void",
                e.name
            );
        }
    }
}

/// glibc rewrites `atexit(f)` into `__cxa_atexit(f, 0, __dso_handle)` with `f`
/// cast from `void (*)(void)`, so its declared `void (*)(void *)` would give
/// every handler a parameter its source never wrote.
#[test]
fn cxa_atexit_is_not_a_slot() {
    assert!(entry_for("__cxa_atexit").is_none());
    assert!(entry_for("sigaction").is_none());
    assert!(entry_for("qsort").is_some());
    assert!(entry_for("signal").is_some());
}

/// glibc and the BSDs put `qsort_r`'s comparator and its data pointer in
/// opposite argument positions, and the name does not say which library the
/// image links.
#[test]
fn qsort_r_is_not_a_slot() {
    assert!(entry_for("qsort_r").is_none());
}

#[test]
fn a_name_the_table_does_not_carry_is_not_a_slot() {
    assert!(entry_for("memcpy").is_none());
    assert!(entry_for("").is_none());
}

#[test]
fn one_call_site_seen_once_is_one_site() {
    let l = ledger(&[((1, 0x2000), vec![fact(0x3a72, 0x415f, "qsort", CMP2)])]);
    let d = l.decided();
    let row = d.get(&0x3a72).expect("the constant is recorded");
    assert_eq!(row.owners, [0x2000].into_iter().collect());
    assert_eq!(row.args, 1);
    assert_eq!(row.declared_by, "qsort");
    assert!(!row.conflict);
}

/// A redo of the same caller replaces what its first body said, so the site
/// count still matches the image's own cross-reference count.
#[test]
fn a_caller_decompiled_again_does_not_count_its_call_twice() {
    let mut l = ledger(&[((1, 0x2000), vec![fact(0x3a72, 0x415f, "qsort", CMP2)])]);
    l.by_caller.insert(
        (1, 0x2000),
        CallerFacts { facts: vec![fact(0x3a72, 0x415f, "qsort", CMP2)], ..Default::default() },
    );
    assert_eq!(l.decided()[&0x3a72].args, 1);
}

/// Two callers are two owners, and the image may show a reference in each.
#[test]
fn two_callers_are_two_owners() {
    let l = ledger(&[
        ((1, 0x2000), vec![fact(0x3a72, 0x415f, "qsort", CMP2)]),
        ((1, 0x2400), vec![fact(0x3a72, 0x4500, "bsearch", CMP2)]),
    ]);
    let row = &l.decided()[&0x3a72];
    assert_eq!(row.owners, [0x2000, 0x2400].into_iter().collect());
    assert_eq!(row.args, 2);
    assert!(!row.conflict, "qsort and bsearch declare the same comparator");
}

/// gcc hoists the `lea` out of two `signal(SIG, handler)` calls in a row, so
/// ONE reference in the image carries two callback arguments. The argument
/// count is the upper bound the driver checks the references against, so the
/// hoisted form passes and a second `lea` for something else does not. What
/// the hoisted form does NOT say is that both uses were callback arguments --
/// that is [`CallerFacts::other_use`]'s question, below.
#[test]
fn one_owner_can_carry_two_arguments() {
    let l = ledger(&[(
        (1, 0x2000),
        vec![fact(0x3b72, 0x21a0, "signal", HANDLER), fact(0x3b72, 0x21c0, "signal", HANDLER)],
    )]);
    let row = &l.decided()[&0x3b72];
    assert_eq!(row.owners.len(), 1);
    assert_eq!(row.args, 2);
}

#[test]
fn two_slots_that_disagree_are_a_conflict() {
    let l = ledger(&[
        ((1, 0x2000), vec![fact(0x3a72, 0x415f, "qsort", CMP2)]),
        ((1, 0x2400), vec![fact(0x3a72, 0x4500, "signal", HANDLER)]),
    ]);
    assert!(l.decided()[&0x3a72].conflict);
}

#[test]
fn a_caller_that_passes_two_different_callbacks_records_both() {
    let l = ledger(&[(
        (1, 0x2000),
        vec![fact(0x3a72, 0x415f, "qsort", CMP2), fact(0x3b72, 0x4180, "signal", HANDLER)],
    )]);
    let d = l.decided();
    assert_eq!(d.len(), 2);
    assert_eq!(d[&0x3a72].sig, CMP2);
    assert_eq!(d[&0x3b72].sig, HANDLER);
}

#[test]
fn an_empty_run_decides_nothing() {
    assert!(Ledger::default().decided().is_empty());
}

#[test]
fn decline_tokens_are_distinct() {
    let all = [
        Decline::Declared,
        Decline::Conflict,
        Decline::Escapes,
        Decline::NotATarget,
        Decline::BodyNotSeen,
        Decline::MoreInputsThanDeclared,
        Decline::FewerInputsThanDeclared,
        Decline::WiderInputThanDeclared,
        Decline::WiderOutputThanDeclared,
        Decline::NarrowerOutputThanDeclared,
        Decline::CallerReadsAWiderReturn,
        Decline::ReadsPastTheList,
        Decline::ReturnsAValue,
        Decline::ReturnsNoValue,
        Decline::StorageUnderivable,
        Decline::Agrees,
    ];
    let mut tokens: Vec<&str> = all.iter().map(|d| d.as_str()).collect();
    tokens.sort_unstable();
    let n = tokens.len();
    tokens.dedup();
    assert_eq!(tokens.len(), n);
}

/// The same hoisted `lea` can feed a `signal` call and a registration that is
/// not a slot at all (glibc compiles `atexit(f)` to `__cxa_atexit`, and a
/// program may call its own registrar). The image shows one reference for both
/// uses, so the reference count cannot see the second one; the body says it
/// used the address somewhere no slot accounts for, and that is an escape.
#[test]
fn a_body_that_uses_the_address_somewhere_else_is_an_escape() {
    let mut l = ledger(&[((1, 0x2000), vec![fact(0x3b72, 0x21a0, "signal", HANDLER)])]);
    assert!(!l.decided()[&0x3b72].other_use);
    l.by_caller.get_mut(&(1, 0x2000)).unwrap().other_use.insert(0x3b72);
    let row = &l.decided()[&0x3b72];
    assert_eq!(row.args, 1, "the registration itself is still one argument");
    assert!(row.other_use, "the second use of the same address is not explained");
}

/// Another function's unexplained use of a DIFFERENT address says nothing
/// about this one.
#[test]
fn another_addresss_other_use_does_not_touch_this_row() {
    let mut l = ledger(&[
        ((1, 0x2000), vec![fact(0x3b72, 0x21a0, "signal", HANDLER)]),
        ((1, 0x2400), vec![fact(0x3a72, 0x4500, "qsort", CMP2)]),
    ]);
    l.by_caller.get_mut(&(1, 0x2400)).unwrap().other_use.insert(0x3a72);
    assert!(!l.decided()[&0x3b72].other_use);
    assert!(l.decided()[&0x3a72].other_use);
}

/// An op that merely carries the address along is not a use of it: the `PTRSUB`
/// a global address is built as, the copies and casts between it and the call,
/// the phi that merges two paths, the `INDIRECT` a call leaves behind.
#[test]
fn the_ops_that_only_carry_an_address_are_not_uses() {
    for code in [
        OpCode::CPUI_COPY,
        OpCode::CPUI_CAST,
        OpCode::CPUI_INT_ZEXT,
        OpCode::CPUI_PTRSUB,
        OpCode::CPUI_MULTIEQUAL,
        OpCode::CPUI_INDIRECT,
    ] {
        assert!(carries_a_value(code), "{code:?} carries the value");
    }
    for code in [
        OpCode::CPUI_CALL,
        OpCode::CPUI_CALLIND,
        OpCode::CPUI_STORE,
        OpCode::CPUI_INT_EQUAL,
        OpCode::CPUI_RETURN,
    ] {
        assert!(!carries_a_value(code), "{code:?} consumes it");
    }
}

/// What a body's own recovery found: its inputs, the output it computes (if
/// any) and, when it computes none, what the call it ends in leaves.
fn own(inputs: &[Range], output: Option<Range>, call_return: Option<Range>) -> OwnFacts {
    OwnFacts {
        returns_a_value: output.is_some(),
        inputs: inputs.to_vec(),
        output,
        output_set: output,
        call_return,
        noreturn_calls: Vec::new(),
        printed: None,
    }
}

const EAX: Range = (1, 0, 4);
const RAX: Range = (1, 0, 8);
const ESI: Range = (1, 0x30, 4);
const RSI: Range = (1, 0x30, 8);
const EDI: Range = (1, 0x38, 4);
const RDI: Range = (1, 0x38, 8);
const RDX: Range = (1, 0x10, 8);

/// Where the x86-64 convention puts each tested signature's parameters.
const CMP2_IN: &[Range] = &[RDI, RSI];
const HANDLER_IN: &[Range] = &[EDI];
const THREAD_IN: &[Range] = &[RDI];
const GLOBERR_IN: &[Range] = &[RDI, ESI];

/// The recovered input count is what a forwarding body is held to. A
/// three-argument function cast into `qsort`'s slot passes its third register
/// on without reading it, so the entry-fact walk states nothing; the count its
/// own decompile recovered does.
#[test]
fn a_recovered_list_longer_than_the_declaration_is_refused() {
    assert_eq!(
        body_contradicts(&own(&[RDI, RSI, RDX], Some(EAX), None), &CMP2, false, CMP2_IN, Some(EAX)),
        Some(Decline::MoreInputsThanDeclared)
    );
    assert_eq!(
        body_contradicts(&own(&[RDI, RSI], Some(EAX), None), &CMP2, true, CMP2_IN, Some(EAX)),
        None
    );
}

/// A declaration longer than the body materializes an argument at every direct
/// call site, out of whatever the register last held: `signal(SIGALRM, (void
/// (*)(int))cleanup); ... cleanup();` on a `void cleanup(void)`, or a
/// one-parameter scorer cast into `qsort`'s two-parameter slot and called
/// directly too. With no direct call site the parameter is only unread, and
/// the park stands.
#[test]
fn a_recovered_list_shorter_than_the_declaration_is_refused_only_with_a_direct_caller() {
    assert_eq!(
        body_contradicts(&own(&[], None, None), &HANDLER, true, HANDLER_IN, None),
        Some(Decline::FewerInputsThanDeclared)
    );
    assert_eq!(
        body_contradicts(&own(&[RDI], Some(EAX), None), &CMP2, true, CMP2_IN, Some(EAX)),
        Some(Decline::FewerInputsThanDeclared)
    );
    assert_eq!(
        body_contradicts(&own(&[], None, None), &HANDLER, false, HANDLER_IN, None),
        None,
        "a handler that never reads its argument and is only ever called by the slot"
    );
}

/// A pointer routine cast into `signal`'s `void (*)(int)` reads all eight bytes
/// of the register the slot declares a four-byte `int` in. Refused whether or
/// not anything calls it directly: the body alone would rebuild its pointer
/// from half a register. An input inside the declared storage -- the `int`
/// itself, or a narrower read of a declared pointer -- stands.
#[test]
fn an_input_wider_than_the_declared_storage_is_refused() {
    for direct in [false, true] {
        assert_eq!(
            body_contradicts(&own(&[RDI], None, None), &HANDLER, direct, HANDLER_IN, None),
            Some(Decline::WiderInputThanDeclared)
        );
    }
    assert_eq!(body_contradicts(&own(&[EDI], None, None), &HANDLER, true, HANDLER_IN, None), None);
    assert_eq!(
        body_contradicts(&own(&[EDI, RSI], Some(EAX), None), &CMP2, true, CMP2_IN, Some(EAX)),
        None
    );
    assert_eq!(
        body_contradicts(&own(&[RSI, RDI], Some(EAX), None), &CMP2, false, CMP2_IN, Some(EAX)),
        Some(Decline::WiderInputThanDeclared),
        "each input is held to the storage of ITS position"
    );
    assert_eq!(
        body_contradicts(&own(&[RDI, RSI], Some(EAX), None), &GLOBERR, false, GLOBERR_IN, Some(EAX)),
        Some(Decline::WiderInputThanDeclared),
        "the second input of an errfunc is an `int`"
    );
}

/// A `void` slot on a body that hands back a value it computed.
#[test]
fn a_computed_value_in_a_void_slot_is_refused() {
    assert_eq!(
        body_contradicts(&own(&[EDI], Some(EAX), None), &HANDLER, false, HANDLER_IN, None),
        Some(Decline::ReturnsAValue)
    );
}

/// A value-returning slot on a body that computes no value is refused. A
/// `void` start routine ending in a tail-called `puts` leaves four of the eight
/// bytes a `void *` return reads, and a body with no tail call leaves nothing.
#[test]
fn a_value_slot_on_a_body_that_computes_no_value_is_refused() {
    assert_eq!(
        body_contradicts(&own(&[RDI], None, Some(EAX)), &THREAD, false, THREAD_IN, Some(RAX)),
        Some(Decline::ReturnsNoValue)
    );
    assert_eq!(
        body_contradicts(&own(&[RDI, ESI], None, None), &GLOBERR, false, GLOBERR_IN, Some(EAX)),
        Some(Decline::ReturnsNoValue)
    );
    assert_eq!(
        body_contradicts(&own(&[RDI, RSI], None, Some(EAX)), &CMP2, false, CMP2_IN, None),
        Some(Decline::ReturnsNoValue),
        "no declared storage to compare against states nothing"
    );
}

/// A comparator ending `return strcmp(a, b);` recovers `void` on its own, but
/// the callee leaves its declared `int` in the register and nothing after the
/// call writes it: the `int` slot is computed. A computed value narrower than
/// the declared return sits inside it.
#[test]
fn a_call_that_leaves_exactly_the_declared_value_computes_it() {
    assert_eq!(
        body_contradicts(&own(&[RDI, RSI], None, Some(EAX)), &CMP2, false, CMP2_IN, Some(EAX)),
        None
    );
    assert_eq!(
        body_contradicts(&own(&[RDI], None, Some(RAX)), &THREAD, false, THREAD_IN, Some(RAX)),
        None
    );
    assert_eq!(
        body_contradicts(&own(&[RDI], Some(EAX), None), &THREAD, false, THREAD_IN, Some(RAX)),
        None
    );
}

/// A `long` comparator cast into `qsort`'s `int` slot computes all eight bytes
/// of the return register; declaring `int` would subtract in the low half and
/// hand a direct caller reading the whole register an invented high half. The
/// same holds when the value is what a call left: a call declared `long` hands
/// back more than the slot returns.
#[test]
fn a_value_wider_than_the_declared_return_is_refused() {
    for direct in [false, true] {
        assert_eq!(
            body_contradicts(&own(&[RDI, RSI], Some(RAX), None), &CMP2, direct, CMP2_IN, Some(EAX)),
            Some(Decline::WiderOutputThanDeclared)
        );
    }
    assert_eq!(
        body_contradicts(&own(&[RDI, RSI], None, Some(RAX)), &CMP2, false, CMP2_IN, Some(EAX)),
        Some(Decline::WiderOutputThanDeclared)
    );
    assert_eq!(
        body_contradicts(&own(&[RDI, RSI], Some(EAX), None), &CMP2, false, CMP2_IN, None),
        Some(Decline::ReturnsNoValue),
        "no declared storage to hold the value in states nothing"
    );
}

/// Every 32-bit write on x86-64 zero-extends into the whole register, so an
/// `int` comparator ending `movzbl %al,%eax; cmovl %edx,%eax` recovers an
/// 8-byte `rax` output whose value only ever sets the low four bytes. That
/// value fits the `int` slot, unless something calls the function directly:
/// a direct caller reading the whole register is what a `long` holding a
/// zero-extended value looks like.
#[test]
fn a_zero_extended_value_in_a_wider_register_fits_without_a_direct_caller() {
    let zext = OwnFacts { output_set: Some(EAX), ..own(&[RDI, RSI], Some(RAX), None) };
    assert_eq!(body_contradicts(&zext, &CMP2, false, CMP2_IN, Some(EAX)), None);
    assert_eq!(
        body_contradicts(&zext, &CMP2, true, CMP2_IN, Some(EAX)),
        Some(Decline::WiderOutputThanDeclared)
    );
    let wide = own(&[RDI, RSI], Some(RAX), None);
    assert_eq!(
        body_contradicts(&wide, &CMP2, false, CMP2_IN, Some(EAX)),
        Some(Decline::WiderOutputThanDeclared),
        "a 64-bit difference sets the upper half"
    );
}

/// The set part is the low bytes that hold every possibly non-zero bit, on
/// whichever end of the storage the space keeps its low bytes.
#[test]
fn the_set_part_is_the_low_bytes_that_hold_every_set_bit() {
    assert_eq!(set_part(RAX, 0xffff_ffff, false), Some(EAX));
    assert_eq!(set_part(RAX, 1, false), Some((1, 0, 1)));
    assert_eq!(set_part(RAX, 0, false), Some((1, 0, 1)));
    assert_eq!(set_part(RAX, 0x1_0000_0000, false), Some((1, 0, 5)));
    assert_eq!(set_part(RAX, uintb::MAX, false), Some(RAX));
    assert_eq!(set_part(RAX, 0xffff_ffff, true), Some((1, 4, 4)));
    assert_eq!(set_part(EAX, uintb::MAX, false), Some(EAX));
}

/// Every recorded direct call reads some part of its callee's return, and a
/// caller decompiled again replaces what its first body read. A `long`
/// comparator that returns a zero-extended value computes it in `eax`, so only
/// the caller that passes the whole `rax` on to `printf("%ld")` says the
/// function is wider than the slot's `int`.
#[test]
fn the_reads_of_a_callee_are_every_recorded_callers() {
    let mut l = Ledger::default();
    let reads = |r: Vec<(uintb, Range)>| CallerFacts { reads: r, ..Default::default() };
    l.by_caller.insert((1, 0x1000), reads(vec![(0x3a72, EAX), (0x4000, RAX)]));
    l.by_caller.insert((1, 0x2000), reads(vec![(0x3a72, RAX)]));
    let mut got: Vec<Range> = l.reads_of(0x3a72).collect();
    got.sort_unstable();
    assert_eq!(got, vec![EAX, RAX]);
    l.by_caller.insert((1, 0x2000), reads(vec![(0x3a72, EAX)]));
    assert!(l.reads_of(0x3a72).all(|r| contains(EAX, r)));
    assert_eq!(l.reads_of(0x5000).count(), 0);
}

/// Containment is by bytes in one space, so a big-endian `int` in the low half
/// of a 64-bit register sits inside the register and a different space never
/// does.
#[test]
fn containment_is_by_bytes_in_one_space() {
    assert!(contains(RAX, EAX));
    assert!(contains(RAX, (1, 4, 4)));
    assert!(!contains(EAX, RAX));
    assert!(!contains(RAX, (2, 0, 4)));
    assert!(!contains(EAX, (1, 2, 4)));
}

const AL: Range = (1, 0, 1);

fn reg(offset: uintb, size: int4) -> Raw {
    Raw { kind: RawKind::Space, space: 1, offset, size }
}

fn konst(value: uintb, size: int4) -> Raw {
    Raw { kind: RawKind::Const, space: 0, offset: value, size }
}

fn temp(offset: uintb, size: int4) -> Raw {
    Raw { kind: RawKind::Unique, space: 2, offset, size }
}

/// Run raw operations over a window, as one straight line.
fn run(window: Range, ops: &[(OpCode, Raw, Vec<Raw>)], conditional: bool) -> u32 {
    let mut st = ZeroState::default();
    for (code, out, ins) in ops {
        write_zero(*code, out, ins, &mut st, window, false, conditional);
    }
    st.window
}

/// `movzbl (%rdi),%eax` is `EAX = zext(tmp:1); RAX = zext(EAX)`, and a later
/// `sub (%rsi),%al` touches only the low byte: the rest of `eax` stays zero.
#[test]
fn a_zero_extension_then_a_byte_write_leaves_the_upper_bytes_zero() {
    let ops = [
        (OpCode::CPUI_INT_ZEXT, reg(0, 4), vec![temp(0x100, 1)]),
        (OpCode::CPUI_INT_ZEXT, reg(0, 8), vec![reg(0, 4)]),
        (OpCode::CPUI_INT_SUB, reg(0, 1), vec![reg(0, 1), temp(0x200, 1)]),
    ];
    assert_eq!(run(EAX, &ops, false), 0b1110);
    assert_eq!(run(RAX, &ops, false), 0b1111_1110);
}

/// `setg %al` and `mov (%rdi),%al` write one byte and leave the rest of the
/// register as the caller left it.
#[test]
fn a_byte_write_alone_proves_nothing_above_it() {
    let ops = [(OpCode::CPUI_COPY, reg(0, 1), vec![reg(0x207, 1)])];
    assert_eq!(run(EAX, &ops, false), 0);
    let ops = [(OpCode::CPUI_LOAD, reg(0, 1), vec![konst(0, 8), reg(0x38, 8)])];
    assert_eq!(run(EAX, &ops, false), 0);
}

/// `xor %eax,%eax` clears the register; a constant clears the bytes it holds
/// as zero; an `and` with a zero byte clears that byte.
#[test]
fn a_self_xor_a_constant_and_a_mask_are_zero_where_they_say() {
    let xor = [(OpCode::CPUI_INT_XOR, reg(0, 4), vec![reg(0, 4), reg(0, 4)])];
    assert_eq!(run(EAX, &xor, false), 0b1111);
    let one = [(OpCode::CPUI_COPY, reg(0, 4), vec![konst(1, 4)])];
    assert_eq!(run(EAX, &one, false), 0b1110);
    let and = [(OpCode::CPUI_INT_AND, reg(0, 4), vec![reg(0x38, 4), konst(0xff, 4)])];
    assert_eq!(run(EAX, &and, false), 0b1110);
    let two = [(OpCode::CPUI_INT_XOR, reg(0, 4), vec![reg(0, 4), reg(0x38, 4)])];
    assert_eq!(run(EAX, &two, false), 0);
}

/// A sign extension is zero above only when the value's top byte is.
#[test]
fn a_sign_extension_is_zero_above_only_under_a_zero_top_byte() {
    let ops = [(OpCode::CPUI_INT_SEXT, reg(0, 4), vec![temp(0x100, 1)])];
    assert_eq!(run(EAX, &ops, false), 0);
    let ops = [
        (OpCode::CPUI_COPY, temp(0x100, 2), vec![konst(0x7f, 2)]),
        (OpCode::CPUI_INT_SEXT, reg(0, 4), vec![temp(0x100, 2)]),
    ];
    assert_eq!(run(EAX, &ops, false), 0b1110);
}

/// A write that may not happen (an instruction with a p-code relative
/// branch) zeroes a byte only if it was already zero.
#[test]
fn a_conditional_write_zeroes_only_what_was_zero() {
    let ops = [(OpCode::CPUI_COPY, reg(0, 4), vec![konst(0, 4)])];
    assert_eq!(run(EAX, &ops, true), 0);
    let ops = [
        (OpCode::CPUI_COPY, reg(0, 4), vec![konst(0, 4)]),
        (OpCode::CPUI_COPY, reg(0, 1), vec![reg(0x207, 1)]),
    ];
    assert_eq!(run(EAX, &ops, true), 0);
}

/// Only a computed value narrower than a declared return is asked about,
/// and it stands when every byte above it is zero at every RETURN.
#[test]
fn a_narrower_computed_return_needs_zero_above_it() {
    let byte = own(CMP2_IN, Some(AL), None);
    assert_eq!(narrower_output(&byte, &CMP2, Some(EAX)), Some((EAX, AL)));
    assert_eq!(narrower_output(&byte, &THREAD, Some(RAX)), Some((RAX, AL)));
    assert_eq!(narrower_output(&own(CMP2_IN, Some(EAX), None), &CMP2, Some(EAX)), None);
    assert_eq!(narrower_output(&own(CMP2_IN, None, Some(EAX)), &CMP2, Some(EAX)), None);
    assert_eq!(narrower_output(&byte, &HANDLER, None), None);
    assert!(rest_is_zero(EAX, AL, Some(0b1110)));
    assert!(rest_is_zero(RAX, EAX, Some(0b1111_0000)));
    assert!(!rest_is_zero(EAX, AL, Some(0b0110)));
    assert!(!rest_is_zero(RAX, AL, Some(0b1110)));
    assert!(!rest_is_zero(EAX, AL, None));
}

fn base(size: int4, meta: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, meta))
}

fn declared(intypes: &[Rc<Datatype>], outtype: Option<Rc<Datatype>>) -> PrototypePieces {
    PrototypePieces {
        name: "cb".to_string(),
        outtype,
        intypes: intypes.to_vec(),
        innames: vec![String::new(); intypes.len()],
        first_var_arg_slot: -1,
        output_storage: None,
        input_storage: Vec::new(),
    }
}

/// A declaration that says exactly what the body already printed -- a
/// `pthread_once` routine recovered `void f(void)` -- is not parked: it would
/// change nothing but the cost of decompiling the function and its callers
/// again. Any difference in a type, the count, the return or a `...` parks.
#[test]
fn a_declaration_the_body_already_printed_is_not_parked() {
    let int = base(4, type_metatype::TYPE_INT);
    let long = base(8, type_metatype::TYPE_INT);
    let printed = |inputs: Vec<Rc<Datatype>>, output: Option<Rc<Datatype>>, dotdotdot: bool| {
        PrintedSig { inputs, output, dotdotdot }
    };
    assert!(agrees(Some(&printed(vec![], None, false)), &declared(&[], None)));
    let handler = printed(vec![Rc::clone(&int)], None, false);
    assert!(agrees(Some(&handler), &declared(&[Rc::clone(&int)], None)));
    assert!(!agrees(Some(&handler), &declared(&[Rc::clone(&long)], None)), "a type differs");
    assert!(!agrees(Some(&handler), &declared(&[], None)), "the count differs");
    assert!(!agrees(Some(&handler), &declared(&[Rc::clone(&int)], Some(Rc::clone(&int)))));
    let open = printed(vec![Rc::clone(&int)], None, true);
    assert!(!agrees(Some(&open), &declared(&[Rc::clone(&int)], None)), "a `...` differs");
    assert!(!agrees(None, &declared(&[], None)), "an untyped signature is not known to agree");
}

/// A function that calls a parked callback directly is decompiled again only
/// where the declaration changes that call: it passes another number of
/// arguments, or it consumes a returned value the declaration types
/// differently from what the callback printed. A caller the run never
/// recorded is decompiled again.
#[test]
fn a_caller_is_decompiled_again_only_where_the_declaration_changes_its_call() {
    let mut l = Ledger::default();
    let calls = |c: Vec<(uintb, usize, bool)>| CallerFacts { calls: c, ..Default::default() };
    l.by_caller.insert((1, 0x1000), calls(vec![(0x3a72, 1, false), (0x4000, 2, true)]));
    l.by_caller.insert((1, 0x2000), calls(vec![(0x3a72, 1, true)]));
    l.by_caller.insert((1, 0x3000), calls(vec![(0x3a72, 0, false)]));
    for same_return in [false, true] {
        assert!(!l.caller_changes(0x1000, 0x3a72, 1, same_return), "same count, result ignored");
        assert!(l.caller_changes(0x1000, 0x3a72, 2, same_return), "the declaration passes one more");
        assert!(l.caller_changes(0x3000, 0x3a72, 1, same_return), "the call passed fewer");
        assert!(!l.caller_changes(0x1000, 0x5000, 1, same_return), "no call to it in the body");
        assert!(l.caller_changes(0x9000, 0x3a72, 1, same_return), "never recorded");
    }
    assert!(l.caller_changes(0x2000, 0x3a72, 1, false), "the result it consumes is typed anew");
    assert!(!l.caller_changes(0x2000, 0x3a72, 1, true), "it consumes the same return as before");
}

thread_local! {
    static RAM: Rc<kuna_base::space::AddrSpace> = Rc::new(kuna_base::space::AddrSpace::new(
        kuna_base::space::spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        1,
        0,
        0,
        0,
    ));
}

fn ram(off: uintb) -> Address {
    RAM.with(|sp| Address::new(Rc::clone(sp), off))
}

/// The return a consuming caller reads is typed anew only when the declared
/// return is not the one the callback's first decompile printed. A callback
/// whose printed signature has an untyped part is not known to agree.
#[test]
fn the_return_is_the_printed_one_only_when_the_types_agree() {
    let int = base(4, type_metatype::TYPE_INT);
    let long = base(8, type_metatype::TYPE_INT);
    let entry = ram(0x3a72);
    let key = key_of(&entry).unwrap();
    let mut l = Ledger::default();
    assert!(!l.printed_the_return(&entry, &declared(&[], Some(Rc::clone(&int)))), "never decompiled");
    let printed = |output: Option<Rc<Datatype>>| OwnFacts {
        printed: Some(PrintedSig { inputs: vec![], output, dotdotdot: false }),
        ..Default::default()
    };
    l.own.insert(key, printed(Some(Rc::clone(&int))));
    assert!(l.printed_the_return(&entry, &declared(&[Rc::clone(&long)], Some(Rc::clone(&int)))));
    assert!(!l.printed_the_return(&entry, &declared(&[], Some(Rc::clone(&long)))), "another width");
    assert!(!l.printed_the_return(&entry, &declared(&[], None)), "the declaration returns nothing");
    l.own.insert(key, printed(None));
    assert!(l.printed_the_return(&entry, &declared(&[], None)));
    l.own.insert(key, OwnFacts::default());
    assert!(!l.printed_the_return(&entry, &declared(&[], None)), "an untyped signature");
}

fn ptr_to(to: Rc<Datatype>, size: int4) -> Rc<Datatype> {
    let mut p = Datatype::new(size, type_metatype::TYPE_PTR);
    p.submeta = crate::dtype::sub_metatype::SUB_PTR;
    p.kind = crate::dtype::DatatypeKind::Pointer { ptrto: to, spaceid: None, truncate: None, wordsize: 1 };
    Rc::new(p)
}

/// A caller keeps what the callback's body read through a parameter the
/// declaration calls `void *`: a pointer to anything but `void`, of the same
/// size, an unknown pointee included. Nothing narrows a declared `char *` or
/// `int`, and a `void *` vote says no more than the declaration.
#[test]
fn only_a_pointer_to_something_narrows_a_declared_void_pointer() {
    let void_ptr = ptr_to(base(0, type_metatype::TYPE_VOID), 8);
    let record = {
        let mut t = Datatype::new(0x10, type_metatype::TYPE_STRUCT);
        t.name = "struct_0".to_string();
        Rc::new(t)
    };
    let record_ptr = ptr_to(Rc::clone(&record), 8);
    assert!(narrows_a_void_pointer(&void_ptr, &record_ptr));
    assert!(narrows_a_void_pointer(&void_ptr, &ptr_to(base(8, type_metatype::TYPE_UINT), 8)));
    assert!(!narrows_a_void_pointer(&void_ptr, &void_ptr), "void * says nothing more");
    assert!(narrows_a_void_pointer(&void_ptr, &ptr_to(base(8, type_metatype::TYPE_UNKNOWN), 8)));
    assert!(narrows_a_void_pointer(&void_ptr, &ptr_to(base(1, type_metatype::TYPE_UNKNOWN), 8)));
    assert!(!narrows_a_void_pointer(&void_ptr, &base(8, type_metatype::TYPE_INT)), "not a pointer");
    assert!(!narrows_a_void_pointer(&void_ptr, &ptr_to(Rc::clone(&record), 4)), "another size");
    let char_ptr = ptr_to(base(1, type_metatype::TYPE_INT), 8);
    assert!(!narrows_a_void_pointer(&char_ptr, &record_ptr), "the declaration is not void *");
    assert!(!narrows_a_void_pointer(&base(4, type_metatype::TYPE_INT), &record_ptr));
}
