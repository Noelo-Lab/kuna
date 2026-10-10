//! Unit tests for the `calleedeadarg` callee entry-liveness probe.

use super::*;
use crate::p0_knowledge::options::KUNA_OPTION_NAMES;

#[test]
fn option_parses_on_and_off() {
    assert!(OptionCalleeDeadArg.apply("on").unwrap().0);
    assert!(!OptionCalleeDeadArg.apply("off").unwrap().0);
    assert!(OptionCalleeDeadArg.apply("maybe").is_err());
}

#[test]
fn option_is_registered() {
    assert!(KUNA_OPTION_NAMES.contains(&OptionCalleeDeadArg::NAME));
}

#[test]
fn incomplete_summary_proves_nothing() {
    let d = CalleeEntryDead::default();
    assert!(!d.is_complete());
}

#[test]
fn summary_with_no_terminator_proves_nothing() {
    // A walk whose every path closes back onto an already-visited address ends
    // complete, with no reads and no cuts.  The cut test is a conjunction, so
    // over an empty list it holds for every register at once.
    let reg = Rc::new(kuna_base::space::AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        8,
        1,
        3,
        kuna_base::space::addrspace_flags::hasphysical,
        1,
        1,
    ));
    let complete_no_cuts =
        CalleeEntryDead::from_parts(3, Vec::new(), Vec::new(), true);
    assert!(complete_no_cuts.is_complete());
    assert!(!complete_no_cuts.proves_dead(&Address::new(Rc::clone(&reg), 8), 8));

    // The same summary with one terminator that wrote RCX still proves it dead.
    let mut cut = ByteSet::new();
    for b in 8u64..16 {
        cut.insert((3, b));
    }
    let one_cut =
        CalleeEntryDead::from_parts(3, Vec::new(), vec![cut.into_iter().collect()], true);
    assert!(one_cut.proves_dead(&Address::new(reg, 8), 8));
}

/// A callee that leaves for a target nothing accounts for can still be carrying
/// the caller's value out with it, unless it wrote the register first.
/// `argclobber` reads exactly this before deleting a trailing argument, and the
/// direct-call arm is what the reviewer's two-frame counterexamples land on: a
/// NAMED target is not an accounted-for one.
#[test]
fn an_unaccounted_transfer_keeps_the_register_unproven() {
    let reg = Rc::new(kuna_base::space::AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        8,
        1,
        3,
        kuna_base::space::addrspace_flags::hasphysical,
        1,
        1,
    ));
    let rdx = Address::new(Rc::clone(&reg), 8);
    let target = Address::new(Rc::clone(&reg), 0x1000);
    let rdx_bytes: Vec<(int4, u64)> = (8u64..16).map(|b| (3, b)).collect();
    let cut: Vec<(int4, u64)> = rdx_bytes.clone();
    let base = || CalleeEntryDead::from_parts(3, Vec::new(), vec![cut.clone()], true);
    let free = |d: &CalleeEntryDead, a: TargetAnswer| {
        let mut once = Some(a);
        fold_node(d, 3, move |_| once.take().unwrap_or(TargetAnswer::Unaccounted))
            .transfer_free(&rdx, 8)
    };

    // No transfer at all: nothing can leave, so the register is free.
    assert!(free(&base(), TargetAnswer::Unaccounted));

    // A tail call through a pointer, reached without writing rdx.
    assert!(!free(&base().with_opaque_cut(Vec::new()), TargetAnswer::Unaccounted));

    // The same transfer, with rdx written on the way to it.
    assert!(free(&base().with_opaque_cut(rdx_bytes.clone()), TargetAnswer::Unaccounted));

    // A DIRECT call whose target nothing accounts for -- an import with no
    // signature -- is the same hole, and the one the recovered prototype cannot
    // see: the callee renders with two parameters either way.
    let forwards = base().with_named_cut(target.clone(), Vec::new());
    assert!(!free(&forwards, TargetAnswer::Unaccounted));

    // Declared, so the recovery was typed against it.
    assert!(free(&forwards, TargetAnswer::Accounted));

    // Recovered, so the question is asked of the target in turn -- and its own
    // answer carries: a target that forwards to a hole is one too.
    let denied = fold_node(
        &CalleeEntryDead::from_parts(3, Vec::new(), vec![cut.clone()], true)
            .with_opaque_cut(Vec::new()),
        3,
        |_| TargetAnswer::Unaccounted,
    );
    assert!(!free(&forwards, TargetAnswer::Through(denied)));
    let clean = fold_node(&base(), 3, |_| TargetAnswer::Unaccounted);
    assert!(free(&forwards, TargetAnswer::Through(clean)));

    // ... unless the register was written before the forwarding call.
    let written_first = base().with_named_cut(target, rdx_bytes);
    let denied2 = fold_node(
        &CalleeEntryDead::from_parts(3, Vec::new(), vec![cut.clone()], true)
            .with_opaque_cut(Vec::new()),
        3,
        |_| TargetAnswer::Unaccounted,
    );
    assert!(free(&written_first, TargetAnswer::Through(denied2)));

    // An incomplete walk says nothing, in this direction too.
    let incomplete = CalleeEntryDead::from_parts(3, Vec::new(), Vec::new(), false);
    assert!(!free(&incomplete, TargetAnswer::Unaccounted));
}

/// A forwarder that calls its target reads what the target takes, for the
/// bytes it has not written by the call; `passthrough` reads this, and only
/// `proves_input` answers differently for it.
#[test]
fn a_call_forwards_what_its_target_takes() {
    let reg = Rc::new(kuna_base::space::AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        8,
        1,
        3,
        kuna_base::space::addrspace_flags::hasphysical,
        1,
        1,
    ));
    let (rdi, rsi) = (Address::new(Rc::clone(&reg), 0x38), Address::new(Rc::clone(&reg), 0x30));
    let target = Address::new(Rc::clone(&reg), 0x1000);
    let narrow = Rc::new(vec![(3, 0x38, 8), (3, 0x30, 8)]);
    let rsi_bytes: Vec<(int4, u64)> = (0x30u64..0x38).map(|b| (3, b)).collect();
    let forwarder = |written: Vec<(int4, u64)>| {
        CalleeEntryDead::from_parts(3, Vec::new(), vec![written.clone()], true).with_named_cut(target.clone(), written)
    };
    let through = |d: &CalleeEntryDead, t: Option<Rc<Vec<RegRead>>>| match add_reads_through(d, |_| t.clone()) {
        Some(reads_live) => CalleeEntryDead { reads_live, ..d.clone() },
        None => d.clone(),
    };

    let fwd = forwarder(Vec::new());
    assert!(!fwd.proves_input(&rdi, 8));
    let t = through(&fwd, Some(Rc::clone(&narrow)));
    assert!(t.proves_input(&rdi, 8) && t.proves_input(&rsi, 8));
    assert_eq!(t.live_input_width(&rdi, 8), Some(8));
    assert!(!t.proves_read(&rdi, 8));

    // A byte written before the call is the forwarder's own value.
    let t = through(&forwarder(rsi_bytes), Some(Rc::clone(&narrow)));
    assert!(t.proves_input(&rdi, 8) && !t.proves_input(&rsi, 8));

    // A register the forwarder reads itself keeps the width it reads it at: the
    // target's wider read of it adds nothing, while `rdi` still comes through.
    let reads_esi = CalleeEntryDead::from_parts(3, vec![(3, 0x30, 4)], vec![Vec::new()], true)
        .with_named_cut(target.clone(), Vec::new());
    assert_eq!(reads_esi.live_input_width(&rsi, 8), Some(4));
    let t = through(&reads_esi, Some(Rc::clone(&narrow)));
    assert!(t.proves_input(&rdi, 8));
    assert_eq!(t.live_input_width(&rsi, 8), Some(4));

    // A target that takes nothing, or an indirect call, adds nothing.
    assert!(add_reads_through(&fwd, |_| Some(Rc::new(Vec::new()))).is_none());
    let opaque = CalleeEntryDead::from_parts(3, Vec::new(), vec![Vec::new()], true).with_opaque_cut(Vec::new());
    assert!(add_reads_through(&opaque, |_| Some(Rc::clone(&narrow))).is_none());

    // An incomplete forwarder proves nothing whatever its targets take.
    let mut broken = forwarder(Vec::new());
    broken.complete = false;
    assert!(add_reads_through(&broken, |_| Some(Rc::clone(&narrow))).is_none());
}

/// A target's read counts only for a parameter its own statement takes: a
/// variadic's register-save prologue reads every argument register, and the
/// ones past its named parameters are nothing a forwarder hands it.
#[test]
fn a_target_takes_only_what_it_stated() {
    let reg = Rc::new(kuna_base::space::AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        8,
        1,
        3,
        kuna_base::space::addrspace_flags::hasphysical,
        1,
        1,
    ));
    let long = Rc::new(crate::dtype::Datatype::new(8, crate::dtype::type_metatype::TYPE_INT));
    let (rdi, rsi) = (Address::new(Rc::clone(&reg), 0x38), Address::new(Rc::clone(&reg), 0x30));
    let stated = crate::kuna_protoorder::RecoveredTypes {
        inputs: vec![(rdi.clone(), 8, Rc::clone(&long)), (rsi.clone(), 8, long)],
        arity_sound: true,
        output: None,
        result: None,
        vararg_tail: vec![rsi],
    };
    // `edi` is a read of the stated `rdi`.
    assert!(takes(&stated, &(3, 0x38, 4)));
    // `rsi` is stated only as a variadic tail, `rcx` not at all.
    assert!(!takes(&stated, &(3, 0x30, 8)));
    assert!(!takes(&stated, &(3, 0x08, 8)));
    // Another space's bytes are not the register.
    assert!(!takes(&stated, &(4, 0x38, 8)));
}

/// `xor ecx,ecx`, `and edx,0` and `or rdx,-1` all write a CONSTANT into the
/// register they read, and their p-code reads it.  The read is formal — the
/// result does not depend on it — so it is kept out of the
/// [`CalleeEntryDead::proves_input`] set, while [`CalleeEntryDead::proves_read`]
/// still counts it: dropping a read can only make `proves_dead` answer `true`
/// more often, and that direction deletes arguments.
#[test]
fn a_constant_writing_idiom_is_not_a_live_read() {
    let reg = Rc::new(kuna_base::space::AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        8,
        1,
        3,
        kuna_base::space::addrspace_flags::hasphysical,
        1,
        1,
    ));
    let vd = |off: u64, sz: u32| VarnodeData {
        space: Some(Rc::clone(&reg)),
        offset: off,
        size: sz,
    };
    let cspace = Rc::new(kuna_base::space::AddrSpace::new(
        spacetype::IPTR_CONSTANT,
        "const",
        false,
        8,
        1,
        0,
        0,
        0,
        0,
    ));
    let selfop = |opc: OpCode| RawOp {
        opc,
        out: Some(vd(8, 4)),
        ins: vec![vd(8, 4), vd(8, 4)],
    };
    assert!(is_value_erasing(&selfop(OpCode::CPUI_INT_XOR)));
    assert!(is_value_erasing(&selfop(OpCode::CPUI_INT_SUB)));
    // Two different registers is an ordinary read of both.
    assert!(!is_value_erasing(&RawOp {
        opc: OpCode::CPUI_INT_XOR,
        out: Some(vd(8, 4)),
        ins: vec![vd(8, 4), vd(0x10, 4)],
    }));
    // `x & x` and `x | x` return x, so the self form of those is a real read.
    assert!(!is_value_erasing(&selfop(OpCode::CPUI_INT_AND)));
    assert!(!is_value_erasing(&selfop(OpCode::CPUI_INT_OR)));

    let konst = |k: u64, sz: u32| VarnodeData {
        space: Some(Rc::clone(&cspace)),
        offset: k,
        size: sz,
    };
    let with = |opc: OpCode, k: VarnodeData| RawOp {
        opc,
        out: Some(vd(0x10, 8)),
        ins: vec![vd(0x10, 8), k],
    };
    // `and rdx,0` is zero and `or rdx,-1` is all ones, whatever rdx held.
    assert!(is_value_erasing(&with(OpCode::CPUI_INT_AND, konst(0, 8))));
    assert!(is_value_erasing(&with(OpCode::CPUI_INT_OR, konst(u64::MAX, 8))));
    // ...but `and rdx,0xff` and `or rdx,1` both keep bits of rdx.
    assert!(!is_value_erasing(&with(OpCode::CPUI_INT_AND, konst(0xff, 8))));
    assert!(!is_value_erasing(&with(OpCode::CPUI_INT_OR, konst(1, 8))));
    // The all-ones mask is per operand width: 0xffffffff fills a 4-byte
    // constant and leaves the top half of an 8-byte one alone.
    assert!(is_value_erasing(&RawOp {
        opc: OpCode::CPUI_INT_OR,
        out: Some(vd(0x10, 4)),
        ins: vec![vd(0x10, 4), konst(0xffff_ffff, 4)],
    }));
    assert!(!is_value_erasing(&with(OpCode::CPUI_INT_OR, konst(0xffff_ffff, 8))));
}

/// [`CalleeEntryDead::low_read_width`] measures a read from the register's
/// least significant end: `mov %eax,%esi` reads four bytes of `rax`, `test
/// %al,%al` one, and a read of `ah` alone does not read the low byte at all.
/// On a big-endian register file the low end is the last byte.
#[test]
fn a_read_is_measured_from_the_low_end() {
    let space = |big: bool| {
        Rc::new(kuna_base::space::AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "register",
            big,
            8,
            1,
            3,
            kuna_base::space::addrspace_flags::hasphysical,
            1,
            1,
        ))
    };
    let summary = |off: u64, sz: int4| CalleeEntryDead {
        reg_idx: 3,
        reads_live: vec![(3, off, sz)],
        live_read_bytes: (off..off + sz as u64).map(|b| (3, b)).collect(),
        complete: true,
        ..CalleeEntryDead::default()
    };
    let le = Address::new(space(false), 0);
    assert_eq!(summary(0, 4).low_read_width(&le, 8), Some(4));
    assert_eq!(summary(0, 1).low_read_width(&le, 8), Some(1));
    assert_eq!(summary(0, 8).low_read_width(&le, 8), Some(8));
    assert_eq!(summary(1, 1).low_read_width(&le, 8), None);
    let be = Address::new(space(true), 0x18);
    assert_eq!(summary(0x1c, 4).low_read_width(&be, 8), Some(4));
    assert_eq!(summary(0x18, 4).low_read_width(&be, 8), None);
}
