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
