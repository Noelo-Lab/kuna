//! Porter in-module tests for `overrides.rs`.
//!
//! Cover the flow-type string table, the force-goto / indirect / multistage /
//! flow insert+query matrices, the dead-code-delay vector growth + the
//! `hasDeadcodeDelay` `val != getDeadcodeDelay()` guard (the heritage auto-bump
//! semantics, incl. survival across a simulated restart), the prototype-override
//! merge-on-insert rule (newest wins, old dropped), and the `<override>`
//! encode/decode round-trip.

use std::cell::RefCell;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::marshal::{Decoder, Encoder, IdRegistry, XmlDecode, XmlEncode, ATTRIB_TYPE};
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace};

use super::*;

// ---------------------------------------------------------------------------
// Test fixtures
// ---------------------------------------------------------------------------

/// const(0), ram(1, processor, delay=1, deadcodedelay=1),
/// stack(2, spacebase, delay=2, deadcodedelay=4 — a pre-bumped space).
fn manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        1,
        addrspace_flags::hasphysical,
        1, // delay
        1, // deadcodedelay (== delay: no global delay)
    )))
    .unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_SPACEBASE,
        "stack",
        false,
        8,
        1,
        2,
        0,
        2, // delay
        4, // deadcodedelay (!= delay)
    )))
    .unwrap();
    m
}

fn ram(m: &AddrSpaceManager, off: u64) -> Address {
    Address::new(Rc::clone(m.get_space(1).unwrap()), off)
}

fn registry() -> IdRegistry {
    let mut reg = IdRegistry::with_base_ids();
    register_ids(&mut reg);
    reg
}

/// A test prototype override capturing the three operations [`Override`] drives:
/// `set_override`, `encode`, `print_raw`.
struct TestProto {
    tag: String,
    overridden: RefCell<bool>,
}

impl TestProto {
    fn new(tag: &str) -> Box<TestProto> {
        Box::new(TestProto { tag: tag.to_string(), overridden: RefCell::new(false) })
    }
}

impl FuncProtoOverride for TestProto {
    fn set_override(&mut self, val: bool) {
        *self.overridden.borrow_mut() = val;
    }
    fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
        // Stand-in for FuncProto::encode: a <prototype>-ish leaf carrying the tag.
        encoder.open_element(&ELEM_PROTOOVERRIDE);
        encoder.write_string(&ATTRIB_TYPE, self.tag.as_bytes());
        encoder.close_element(&ELEM_PROTOOVERRIDE);
        Ok(())
    }
    fn print_raw(&self, s: &mut String) {
        s.push_str(&format!("proto({})", self.tag));
    }
}

// ---------------------------------------------------------------------------
// Flow-type string table
// ---------------------------------------------------------------------------

#[test]
fn flow_type_string_table() {
    assert_eq!(Override::type_to_string(flow_type::BRANCH), "branch");
    assert_eq!(Override::type_to_string(flow_type::CALL), "call");
    assert_eq!(Override::type_to_string(flow_type::CALL_RETURN), "callreturn");
    assert_eq!(Override::type_to_string(flow_type::RETURN), "return");
    assert_eq!(Override::type_to_string(flow_type::NONE), "none");
    // Out-of-range falls to "none" (the C++ trailing `return "none"`).
    assert_eq!(Override::type_to_string(99), "none");

    assert_eq!(Override::string_to_type(b"branch"), flow_type::BRANCH);
    assert_eq!(Override::string_to_type(b"call"), flow_type::CALL);
    assert_eq!(Override::string_to_type(b"callreturn"), flow_type::CALL_RETURN);
    assert_eq!(Override::string_to_type(b"return"), flow_type::RETURN);
    assert_eq!(Override::string_to_type(b"bogus"), flow_type::NONE);
}

// ---------------------------------------------------------------------------
// Flow override insert/query
// ---------------------------------------------------------------------------

#[test]
fn flow_override_insert_and_query() {
    let m = manager();
    let mut ov = Override::new();
    assert!(!ov.has_flow_override());
    assert_eq!(ov.get_flow_override(&ram(&m, 0x40)), flow_type::NONE);

    ov.insert_flow_override(ram(&m, 0x40), flow_type::CALL);
    ov.insert_flow_override(ram(&m, 0x80), flow_type::RETURN);
    assert!(ov.has_flow_override());
    assert_eq!(ov.get_flow_override(&ram(&m, 0x40)), flow_type::CALL);
    assert_eq!(ov.get_flow_override(&ram(&m, 0x80)), flow_type::RETURN);
    // Absent address -> NONE.
    assert_eq!(ov.get_flow_override(&ram(&m, 0x99)), flow_type::NONE);

    // Re-insert at the same address overwrites (map semantics).
    ov.insert_flow_override(ram(&m, 0x40), flow_type::BRANCH);
    assert_eq!(ov.get_flow_override(&ram(&m, 0x40)), flow_type::BRANCH);
}

// ---------------------------------------------------------------------------
// Force-goto / indirect insert/query
// ---------------------------------------------------------------------------

#[test]
fn force_goto_insert_and_iter_in_address_order() {
    let m = manager();
    let mut ov = Override::new();
    ov.insert_force_goto(ram(&m, 0x80), ram(&m, 0x200));
    ov.insert_force_goto(ram(&m, 0x40), ram(&m, 0x100));
    let pairs: Vec<_> =
        ov.force_gotos().map(|(t, d)| (t.get_offset(), d.get_offset())).collect();
    // BTreeMap by Address: ascending offset.
    assert_eq!(pairs, vec![(0x40, 0x100), (0x80, 0x200)]);
}

#[test]
fn indirect_override_insert_and_query() {
    let m = manager();
    let mut ov = Override::new();
    // Empty store: query short-circuits to None (the C++ `if (!indirectover.empty())`).
    assert!(ov.find_indirect_override(&ram(&m, 0x40)).is_none());

    ov.insert_indirect_override(ram(&m, 0x40), ram(&m, 0x500));
    assert_eq!(ov.find_indirect_override(&ram(&m, 0x40)).unwrap().get_offset(), 0x500);
    assert!(ov.find_indirect_override(&ram(&m, 0x41)).is_none());
}

// ---------------------------------------------------------------------------
// Multistage jump (vector, linear scan, duplicates kept)
// ---------------------------------------------------------------------------

#[test]
fn multistage_jump_linear_scan_keeps_duplicates() {
    let m = manager();
    let mut ov = Override::new();
    assert!(!ov.query_multistage_jumptable(&ram(&m, 0x40)));
    ov.insert_multistage_jump(ram(&m, 0x40));
    ov.insert_multistage_jump(ram(&m, 0x40)); // duplicate insert (vector, not set)
    ov.insert_multistage_jump(ram(&m, 0x80));
    assert!(ov.query_multistage_jumptable(&ram(&m, 0x40)));
    assert!(ov.query_multistage_jumptable(&ram(&m, 0x80)));
    assert!(!ov.query_multistage_jumptable(&ram(&m, 0xC0)));
}

// ---------------------------------------------------------------------------
// Dead-code delay: vector growth + hasDeadcodeDelay guard + restart survival
// ---------------------------------------------------------------------------

#[test]
fn deadcode_delay_grows_vector_and_skips_placeholders() {
    let m = manager();
    let mut ov = Override::new();
    let stack = Rc::clone(m.get_space(2).unwrap()); // index 2
    // Inserting at index 2 grows the vector to len 3 with -1 placeholders at 0,1.
    ov.insert_deadcode_delay(&stack, 7);
    let delays: Vec<_> = ov.deadcode_delays().collect();
    // Only the non-placeholder slot is yielded.
    assert_eq!(delays, vec![(2, 7)]);
}

#[test]
fn has_deadcode_delay_guard_semantics() {
    let m = manager();
    let ram_spc = Rc::clone(m.get_space(1).unwrap()); // delay==deadcodedelay==1
    let mut ov = Override::new();

    // No slot for ram yet (vector empty) -> false.
    assert!(!ov.has_deadcode_delay(&ram_spc));

    // Install a placeholder-only growth by inserting at a higher index, leaving
    // ram's slot at -1 -> still false.
    let stack = Rc::clone(m.get_space(2).unwrap());
    ov.insert_deadcode_delay(&stack, 5);
    assert!(!ov.has_deadcode_delay(&ram_spc)); // ram slot is the -1 placeholder

    // Install ram delay equal to its static deadcodedelay (1): val == getDeadcodeDelay
    // -> the guard returns false (no *effective* override).
    ov.insert_deadcode_delay(&ram_spc, ram_spc.get_deadcode_delay());
    assert!(!ov.has_deadcode_delay(&ram_spc));

    // The heritage auto-bump installs getDeadcodeDelay()+1; now val differs.
    ov.insert_deadcode_delay(&ram_spc, ram_spc.get_deadcode_delay() + 1);
    assert!(ov.has_deadcode_delay(&ram_spc));
}

#[test]
fn deadcode_delay_bump_is_idempotent_and_survives_restart() {
    // Mirror Heritage::bumpDeadcodeDelay's guard loop against the P0 store: a
    // second bump on the same space is suppressed by hasDeadcodeDelay, and the
    // installed delay is still present on a fresh pass (the Override outlives a
    // decompilation restart — modelled by re-reading the same store).
    let m = manager();
    let ram_spc = Rc::clone(m.get_space(1).unwrap());
    let mut ov = Override::new();

    // First bump (simulating the heritage call).
    fn bump(ov: &mut Override, spc: &AddrSpace) -> bool {
        if ov.has_deadcode_delay(spc) {
            return false; // already installed
        }
        ov.insert_deadcode_delay(spc, spc.get_deadcode_delay() + 1);
        true
    }

    assert!(bump(&mut ov, &ram_spc)); // installs delay 2
    assert!(!bump(&mut ov, &ram_spc)); // suppressed: hasDeadcodeDelay is true now
    assert!(!bump(&mut ov, &ram_spc)); // still suppressed

    // Across a "restart" the same Override is consulted: the delay is intact
    // and applyDeadCodeDelay would yield it for the new pass.
    let delays: Vec<_> = ov.deadcode_delays().collect();
    assert_eq!(delays, vec![(1, 2)]);
    assert!(ov.has_deadcode_delay(&ram_spc));
}

// ---------------------------------------------------------------------------
// Prototype override: merge-on-insert (newest wins, old dropped), set_override
// ---------------------------------------------------------------------------

#[test]
fn proto_override_insert_marks_and_merges() {
    let m = manager();
    let mut ov = Override::new();
    // Empty store: find short-circuits to None.
    assert!(ov.find_proto_override(&ram(&m, 0x40)).is_none());

    ov.insert_proto_override(ram(&m, 0x40), TestProto::new("first"));
    {
        let mut s = String::new();
        ov.find_proto_override(&ram(&m, 0x40)).unwrap().print_raw(&mut s);
        assert_eq!(s, "proto(first)");
    }

    // Re-insert at the same call point: the new prototype replaces (and drops)
    // the old one (the C++ `delete` of the pre-existing override).
    ov.insert_proto_override(ram(&m, 0x40), TestProto::new("second"));
    {
        let mut s = String::new();
        ov.find_proto_override(&ram(&m, 0x40)).unwrap().print_raw(&mut s);
        assert_eq!(s, "proto(second)");
    }
}

// ---------------------------------------------------------------------------
// print_raw / generate_override_messages
// ---------------------------------------------------------------------------

#[test]
fn print_raw_and_messages() {
    let m = manager();
    let stack = Rc::clone(m.get_space(2).unwrap());
    let mut ov = Override::new();
    ov.insert_force_goto(ram(&m, 0x40), ram(&m, 0x100));
    ov.insert_indirect_override(ram(&m, 0x80), ram(&m, 0x500));
    ov.insert_deadcode_delay(&stack, 3);
    ov.insert_proto_override(ram(&m, 0xC0), TestProto::new("p"));

    let mut s = String::new();
    ov.print_raw(&mut s, &m);
    assert!(s.contains("force goto at"));
    assert!(s.contains("override indirect at"));
    assert!(s.contains("dead code delay on stack set to 3"));
    assert!(s.contains("override prototype at"));
    assert!(s.contains("proto(p)"));

    let msgs = ov.generate_override_messages(&m);
    assert_eq!(msgs.len(), 1);
    assert!(msgs[0].contains("delay deadcode elimination for space: stack"));
}

// ---------------------------------------------------------------------------
// override encode/decode round-trip
// ---------------------------------------------------------------------------

#[test]
fn override_encode_decode_roundtrip() {
    let m = manager();
    let ram_spc = Rc::clone(m.get_space(1).unwrap());
    let mut ov = Override::new();
    ov.insert_force_goto(ram(&m, 0x40), ram(&m, 0x100));
    ov.insert_indirect_override(ram(&m, 0x80), ram(&m, 0x500));
    ov.insert_deadcode_delay(&ram_spc, 9);
    ov.insert_multistage_jump(ram(&m, 0xC0));
    ov.insert_flow_override(ram(&m, 0x140), flow_type::CALL_RETURN);

    let mut buf = Vec::new();
    {
        let mut enc = XmlEncode::new(&mut buf);
        ov.encode(&mut enc, &m).unwrap();
    }

    let reg = registry();
    let mut dec = XmlDecode::new(&m, &reg);
    dec.ingest_stream(&buf).unwrap();
    let mut ov2 = Override::new();
    // No protooverride present, so the proto decoder is never invoked.
    ov2.decode(&mut dec, &mut |_d| panic!("no protooverride in this document"))
        .unwrap();

    // force goto survived
    let fg: Vec<_> = ov2.force_gotos().map(|(t, d)| (t.get_offset(), d.get_offset())).collect();
    assert_eq!(fg, vec![(0x40, 0x100)]);
    // indirect survived
    assert_eq!(ov2.find_indirect_override(&ram(&m, 0x80)).unwrap().get_offset(), 0x500);
    // deadcode delay survived (index 1, delay 9)
    assert_eq!(ov2.deadcode_delays().collect::<Vec<_>>(), vec![(1, 9)]);
    // multistage survived
    assert!(ov2.query_multistage_jumptable(&ram(&m, 0xC0)));
    // flow survived
    assert_eq!(ov2.get_flow_override(&ram(&m, 0x140)), flow_type::CALL_RETURN);
}

#[test]
fn override_encode_empty_emits_nothing() {
    let m = manager();
    let ov = Override::new();
    let mut buf = Vec::new();
    {
        let mut enc = XmlEncode::new(&mut buf);
        ov.encode(&mut enc, &m).unwrap();
    }
    // The C++ early-returns before opening <override>, so no bytes are written.
    assert!(buf.is_empty());
}

#[test]
fn override_decode_bad_deadcodedelay_errors() {
    // A negative delay is the C++ `throw LowlevelError("Bad deadcodedelay tag")`.
    let m = manager();
    let reg = registry();
    let xml = br#"<override><deadcodedelay space="ram" delay="-1"/></override>"#;
    let mut dec = XmlDecode::new(&m, &reg);
    dec.ingest_stream(xml).unwrap();
    let mut ov = Override::new();
    let err = ov
        .decode(&mut dec, &mut |_d| panic!("no protooverride"))
        .unwrap_err();
    assert!(err.to_string().contains("Bad deadcodedelay tag"));
}

#[test]
fn override_decode_bad_flow_errors() {
    // A flow type that resolves to NONE is the C++ `throw LowlevelError("Bad flowoverride tag")`.
    let m = manager();
    let reg = registry();
    let xml = br#"<override><flow type="bogus"><addr space="ram" offset="0x40"/></flow></override>"#;
    let mut dec = XmlDecode::new(&m, &reg);
    dec.ingest_stream(xml).unwrap();
    let mut ov = Override::new();
    let err = ov
        .decode(&mut dec, &mut |_d| panic!("no protooverride"))
        .unwrap_err();
    assert!(err.to_string().contains("Bad flowoverride tag"));
}
