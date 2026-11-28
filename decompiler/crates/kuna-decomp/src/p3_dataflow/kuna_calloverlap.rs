//! P3 partial-range call-overlap guards — the `calloverlap` decision point.
//!
//! # The gap
//!
//! [`Heritage::guard_calls`](crate::p3_dataflow::heritage::Heritage) characterizes
//! every heritaged range against each call site's prototype.  When the range is
//! *bigger* than the ABI storage it contains the characterization is
//! [`Containment::ContainedBy`](crate::fspec::Containment), and upstream Ghidra
//! runs one of two dedicated guards — `Heritage::guardCallOverlappingInput`
//! (`heritage.cc:1210`) on the input side and `Heritage::tryOutputOverlapGuard` /
//! `Heritage::guardOutputOverlap` (`heritage.cc:1249`/`:1293`) on the output side.
//! kuna shipped both as comment-only stubs, so the parameter and return entries
//! inside such a range were invisible to trial recovery.
//!
//! This is routine on x86-64: SLEIGH models `PXOR`, `MOVDQA`, `MOVDQU` and `ORPD`
//! as ONE 128-bit write to the whole `XmmReg`, so `Heritage::refinement`'s
//! `size > 4 && maxw < size` gate never fires and the range stays 16 bytes wide
//! while the SysV parameter/return entry inside it is 8.  The observable symptom is
//! a call rendered with missing arguments and a return value read from a stale
//! pre-call definition (`u(); sink = v1;` where `v1` is never assigned).
//!
//! # The levels
//!
//! The two halves are separable and carry different risk, so the option is a
//! policy with explicit modes rather than an on/off:
//!
//! * [`LEVEL_OFF`] — both branches inert; byte-identical to what kuna shipped
//!   before this option for a single partially covered ABI entry. Complete
//!   multi-slot register ranges are partitioned by `kuna_multiregargs`.
//! * [`LEVEL_AUTO`] (the default) — both guards, but only for a named
//!   eight-byte register's low four-byte general ABI slot on a truncated 32-bit
//!   address target, including e500 GPRs. The output guard also takes the
//!   model's first float return slot at the low end of a wider register
//!   ([`auto_float_output`]).
//! * [`LEVEL_IN`] — the input guard only.  Recovers the dropped argument but
//!   leaves the call's return value unrecovered.
//! * [`LEVEL_FULL`] — both upstream partial-range guards. Complete register partitions
//!   are registered at every level.
//!
//! The guards themselves are upstream `Heritage` methods and live with their
//! siblings in [`crate::p3_dataflow::heritage`]; this module owns the option's
//! parse surface and the level vocabulary the gate reads.

use kuna_base::error::KunaResult;
use kuna_base::types::int4;

/// `option calloverlap off` — both overlap guards stay inert.
pub const LEVEL_OFF: int4 = 0;

/// `option calloverlap auto` (the default) — both guards, but only for a narrow
/// scalar ABI slot inside a wide register on a truncated-address target.
pub const LEVEL_AUTO: int4 = 1;

/// `option calloverlap in` — the input guard (`guardCallOverlappingInput`) for
/// every contained slot.
pub const LEVEL_IN: int4 = 2;

/// `option calloverlap full` — the input guard plus the output guard
/// (`tryOutputOverlapGuard`); complete register partitions are independent.
pub const LEVEL_FULL: int4 = 3;

/// `option calloverlap auto|off|in|full` (C++ has no equivalent — upstream is
/// unconditionally `full`).
pub struct OptionCallOverlap;

impl OptionCallOverlap {
    /// The option name.
    pub const NAME: &'static str = "calloverlap";

    /// Parse `auto`/`off`/`in`/`full` into the guard level + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(int4, String)> {
        match p1 {
            "auto" => Ok((
                LEVEL_AUTO,
                "Partial-range call overlap guards limited to narrow scalar ABI registers"
                    .to_string(),
            )),
            "off" => Ok((
                LEVEL_OFF,
                "Partial-range call overlap guards turned off".to_string(),
            )),
            "in" => Ok((
                LEVEL_IN,
                "Partial-range call overlap guards turned on, input side only \
                 (guardCallOverlappingInput)"
                    .to_string(),
            )),
            // Empty parameter reads as the full guard, matching `options::on_or_off`.
            "full" | "on" | "" => Ok((
                LEVEL_FULL,
                "Partial-range call overlap guards turned on, input + output \
                 (guardCallOverlappingInput + tryOutputOverlapGuard)"
                    .to_string(),
            )),
            _ => Err(kuna_base::error::KunaError::parse(
                "Must specify one of auto, off, in, full",
            )),
        }
    }
}

/// Whether a call has one low-word scalar ABI slot inside this physical register.
pub(crate) fn auto_register(
    manager: &kuna_base::space::AddrSpaceManager,
    proto: &crate::fspec::FuncProto,
    addr: &kuna_base::address::Address,
    size: int4,
    output: bool,
) -> bool {
    use crate::dtype::type_class;
    use kuna_num::pcoderaw::VarnodeData;

    if size != 8
        || !manager
            .get_default_data_space()
            .is_some_and(|s| s.is_truncated() && s.get_addr_size() == 4)
    {
        return false;
    }
    let Some(space) = addr.get_space() else {
        return false;
    };
    if !manager
        .get_space_by_name("register")
        .is_some_and(|s| s.get_index() == space.get_index())
    {
        return false;
    }
    let Some(lookup) = manager.register_lookup() else {
        return false;
    };
    if lookup
        .get_exact_register_name(space, addr.get_offset(), size)
        .is_empty()
    {
        return false;
    }
    let mut slot = VarnodeData::default();
    let found = if output {
        proto.get_biggest_contained_output(addr, size, &mut slot)
    } else {
        proto.get_biggest_contained_input_param(addr, size, &mut slot)
    };
    if !found
        || slot.size != 4
        || addr.justified_contain(size, &slot.get_addr(), 4, false) != 0
        || lookup
            .get_exact_register_name(space, slot.offset, 4)
            .is_empty()
    {
        return false;
    }
    let list = if output {
        proto.model().output_list()
    } else {
        proto.model().input_opt()
    };
    let Some(list) = list else { return false };
    let mut contained = list
        .get_entry()
        .iter()
        .filter(|e| e.contained_by(addr, size))
        .peekable();
    contained.peek().is_some()
        && contained.all(|e| {
            e.get_type() == type_class::TYPECLASS_GENERAL
                && e.is_exclusion()
                && e.get_join_record().is_none()
                && e.get_base() == slot.offset
                && e.get_size() == 4
        })
}

/// Whether the output guard may give the call `fc`, whose output is not
/// locked, the model's first float return slot as the only output slot inside
/// this wider register range, at its low end: x86-64 `XMM0_Qa` inside an `xmm0`
/// the caller moves whole with `movapd` after the call, or inside the `zmm0`
/// every VEX instruction writes. A callee the run states or recovered a value
/// for takes it as an unprototyped one does, except a `floatbits` helper whose
/// float is only the bits of its float-register input: its readers keep those
/// bits as integers, and a read the guard connected would withdraw the float
/// the helper's own body proves (`voidret`).
pub(crate) fn auto_float_output(
    fd: &crate::funcdata::Funcdata,
    fc: &crate::fspec::FuncCallSpecs,
    addr: &kuna_base::address::Address,
    size: int4,
) -> bool {
    if float_output_slot(fc.proto(), addr, size).is_none() {
        return false;
    }
    let entry = fc.get_entry_address();
    entry
        .get_space()
        .is_none_or(|space| !fd.kuna_callee_float_bits((space.get_index(), entry.get_offset())))
}

/// The model's first float return slot of `proto`, when the output is not
/// locked and that slot is the only output slot inside `[addr, addr+size)`, at
/// its low end, narrower than the range.
fn float_output_slot(
    proto: &crate::fspec::FuncProto,
    addr: &kuna_base::address::Address,
    size: int4,
) -> Option<(kuna_base::address::Address, int4)> {
    use crate::dtype::type_class;

    if proto.is_output_locked() || !proto.has_model() {
        return None;
    }
    let entries = proto.model().output_list()?.get_entry();
    let mut contained = entries.iter().filter(|e| e.contained_by(addr, size));
    let (Some(slot), None) = (contained.next(), contained.next()) else {
        return None;
    };
    let first_float = entries.iter().find(|e| e.get_type() == type_class::TYPECLASS_FLOAT)?;
    let at = kuna_base::address::Address::new(std::rc::Rc::clone(slot.get_space()), slot.get_base());
    (std::ptr::eq(first_float, slot)
        && slot.get_size() < size
        && slot.get_join_record().is_none()
        && addr.justified_contain(size, &at, slot.get_size(), false) == 0)
        .then_some((at, slot.get_size()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dtype::{type_class, type_metatype, Datatype};
    use crate::fspec::{FuncProto, ParamEntry, ProtoModel};
    use kuna_base::address::Address;
    use kuna_base::space::{
        spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, RegisterLookup, VarnodeStorage,
    };
    use std::rc::Rc;

    struct Registers;
    impl RegisterLookup for Registers {
        fn get_register(&self, _: &str) -> KunaResult<VarnodeStorage> {
            Err(kuna_base::error::KunaError::lowlevel("name lookup unused"))
        }
        fn get_register_name(&self, space: &Rc<AddrSpace>, off: u64, size: i32) -> String {
            self.get_exact_register_name(space, off, size)
        }
        fn get_exact_register_name(&self, _: &Rc<AddrSpace>, off: u64, size: i32) -> String {
            if (off == 0x20 && size == 8) || ([0x20, 0x24].contains(&off) && size == 4) {
                "register".to_string()
            } else {
                String::new()
            }
        }
    }

    fn sample(
        big: bool,
        truncated: bool,
        slots: &[(u64, type_class)],
    ) -> (AddrSpaceManager, FuncProto, Address) {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        let ram = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            big,
            8,
            1,
            1,
            0,
            0,
            0,
        ));
        if truncated {
            ram.truncate_space(4);
        }
        manager.insert_space(ram).unwrap();
        manager.set_default_code_space(1).unwrap();
        manager.set_default_data_space(1).unwrap();
        let register = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "register",
            big,
            4,
            1,
            2,
            0,
            0,
            0,
        ));
        manager.insert_space(register.clone()).unwrap();
        manager.set_register_lookup(Rc::new(Registers));
        let mut model = ProtoModel::new(&manager);
        model.build_param_list("standard").unwrap();
        for (group, &(offset, class)) in slots.iter().enumerate() {
            let entry = ParamEntry::seed(
                group as int4,
                class,
                register.clone(),
                offset,
                4,
                1,
                0,
                0,
                true,
                false,
                model.input().get_entry(),
                &manager,
            )
            .unwrap();
            model.input_mut().push_entry(entry);
        }
        model.input_mut().finish_decode();
        let mut proto = FuncProto::new();
        proto.set_internal(
            Rc::new(model),
            Rc::new(Datatype::new(0, type_metatype::TYPE_VOID)),
        );
        (manager, proto, Address::new(register, 0x20))
    }

    #[test]
    fn auto_excludes_high_words_float_slots_and_multiple_slots() {
        use type_class::{TYPECLASS_FLOAT as FLOAT, TYPECLASS_GENERAL as GENERAL};
        for big in [false, true] {
            let low = if big { 0x24 } else { 0x20 };
            let high = if big { 0x20 } else { 0x24 };
            for (slots, expected) in [
                (vec![(low, GENERAL)], true),
                (vec![(high, GENERAL)], false),
                (vec![(low, FLOAT)], false),
                (vec![(low, GENERAL), (high, GENERAL)], false),
            ] {
                let (manager, proto, addr) = sample(big, true, &slots);
                assert_eq!(auto_register(&manager, &proto, &addr, 8, false), expected);
                assert!(!auto_register(&manager, &proto, &addr, 16, false));
                assert!(!auto_register(&manager, &proto, &(&addr + 8), 8, false));
                let memory = Address::new(manager.get_default_data_space().unwrap().clone(), 0x20);
                assert!(!auto_register(&manager, &proto, &memory, 8, false));
                let (manager, proto, addr) = sample(big, false, &slots);
                assert!(!auto_register(&manager, &proto, &addr, 8, false));
            }
        }
    }

    #[test]
    fn float_output_slot_is_only_the_first_float_return_slot_at_the_low_end() {
        use type_class::{TYPECLASS_FLOAT as FLOAT, TYPECLASS_GENERAL as GENERAL};
        let (manager, _, _) = sample(false, false, &[]);
        let register = manager.get_space_by_name("register").unwrap().clone();
        let mut model = ProtoModel::new(&manager);
        model.build_param_list("standard").unwrap();
        for (group, &(offset, class)) in [(0x1200u64, FLOAT), (0x1220, FLOAT), (0x0, GENERAL)].iter().enumerate() {
            let entry = ParamEntry::seed(
                group as int4,
                class,
                register.clone(),
                offset,
                8,
                1,
                0,
                0,
                true,
                false,
                model.output().get_entry(),
                &manager,
            )
            .unwrap();
            model.output_mut().push_entry(entry);
        }
        model.output_mut().finish_decode();
        let mut proto = FuncProto::new();
        proto.set_internal(
            Rc::new(model),
            Rc::new(Datatype::new(0, type_metatype::TYPE_VOID)),
        );
        let at = |off: u64| Address::new(register.clone(), off);
        assert_eq!(float_output_slot(&proto, &at(0x1200), 16), Some((at(0x1200), 8)));
        for (off, size) in [(0x1200, 8), (0x1220, 16), (0x11f8, 16), (0x1200, 64), (0x0, 16)] {
            assert_eq!(float_output_slot(&proto, &at(off), size), None);
        }
    }

    #[test]
    fn levels_are_ordered_so_the_gate_can_use_a_threshold() {
        // Both guards read `level >= LEVEL_IN` / `level >= LEVEL_FULL`, and
        // `auto` enables a subset of `in`, so the vocabulary must stay monotone.
        assert!(LEVEL_OFF < LEVEL_AUTO);
        assert!(LEVEL_AUTO < LEVEL_IN);
        assert!(LEVEL_IN < LEVEL_FULL);
    }

    #[test]
    fn apply_parses_the_four_modes() {
        assert_eq!(OptionCallOverlap.apply("auto").expect("auto").0, LEVEL_AUTO);
        assert_eq!(OptionCallOverlap.apply("off").expect("off").0, LEVEL_OFF);
        assert_eq!(OptionCallOverlap.apply("in").expect("in").0, LEVEL_IN);
        assert_eq!(OptionCallOverlap.apply("full").expect("full").0, LEVEL_FULL);
        // `on` and the empty parameter are the full guard.
        assert_eq!(OptionCallOverlap.apply("on").expect("on").0, LEVEL_FULL);
        assert_eq!(OptionCallOverlap.apply("").expect("empty").0, LEVEL_FULL);
        assert!(OptionCallOverlap.apply("wide").is_err());
    }
}
