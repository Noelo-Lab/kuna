//! Unit tests for the `foldcallret` option parse.  The order-safety predicate
//! [`call_output_foldable`](super::call_output_foldable) is exercised end-to-end
//! by the stage test `tests/stages/ghangr-call-return-variable-folding-dcde82.xml`.

use super::*;

#[test]
fn option_parse_on_off() {
    let (v, msg) = OptionFoldCallRet.apply("on").unwrap();
    assert!(v);
    assert!(msg.contains("on"));
    let (v, msg) = OptionFoldCallRet.apply("off").unwrap();
    assert!(!v);
    assert!(msg.contains("off"));
}

#[test]
fn option_name_is_foldcallret() {
    assert_eq!(OptionFoldCallRet::NAME, "foldcallret");
}

//===========================================================================
// The storage half of the barrier set (GH-657)
//===========================================================================

mod storage {
    use std::rc::Rc;

    use kuna_base::address::Address;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };
    use kuna_base::types::int4;
    use kuna_num::opcodes::OpCode;

    use crate::context::{ArchContext, OpId, VarnodeId};
    use crate::funcdata::Funcdata;
    use crate::varnode::varnode_flags;

    use super::super::{op_is_barrier, op_is_write_barrier, op_writes_tied_storage};

    fn build_fd() -> Funcdata {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        let glb = Rc::new(ArchContext::new(m));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        Funcdata::new("f", "f", glb, Address::new(ram, 0x1000), 0x1000_0000, 0x40).unwrap()
    }

    fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
        Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
    }

    /// A global read: a persistent, address-tied varnode at `off`.
    fn glob(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
        let addr = Address::new(ram(fd), off);
        let vn = fd.new_varnode(size, &addr, None);
        fd.vbank_mut()
            .get_mut(vn)
            .unwrap()
            .set_flags_pub(varnode_flags::persist | varnode_flags::addrtied);
        vn
    }

    /// `ram[out_off] = <opc> ins` — the shape heritage leaves behind for a write
    /// to a fixed address.
    fn write_op(
        fd: &mut Funcdata,
        opc: OpCode,
        ins: &[VarnodeId],
        out_off: u64,
        out_size: int4,
    ) -> OpId {
        let pc = Address::new(ram(fd), 0x1100);
        let op = fd.new_op(ins.len() as int4, pc);
        fd.op_set_opcode_code(op, opc);
        for (slot, vn) in ins.iter().enumerate() {
            fd.op_set_input(op, *vn, slot as int4).unwrap();
        }
        let addr = Address::new(ram(fd), out_off);
        let out = fd.new_varnode_out(out_size, &addr, op).unwrap();
        fd.vbank_mut()
            .get_mut(out)
            .unwrap()
            .set_flags_pub(varnode_flags::persist | varnode_flags::addrtied);
        op
    }

    /// `k = 42` (GH-657's write): a COPY of a constant into a global.  No STORE
    /// opcode, so only the storage test sees it — on both spans.
    #[test]
    fn a_constant_written_to_a_global_is_a_barrier_on_both_spans() {
        let mut fd = build_fd();
        let c = fd.new_constant(4, 42);
        let op = write_op(&mut fd, OpCode::CPUI_COPY, &[c], 0x4000, 4);
        assert!(op_writes_tied_storage(&fd, op));
        assert!(op_is_barrier(&fd, op));
        assert!(op_is_write_barrier(&fd, op));
    }

    /// The same shape with arithmetic rather than a copy.
    #[test]
    fn arithmetic_into_a_global_is_a_barrier() {
        let mut fd = build_fd();
        let g = glob(&mut fd, 0x4000, 4);
        let c = fd.new_constant(4, 1);
        let op = write_op(&mut fd, OpCode::CPUI_INT_ADD, &[g, c], 0x4000, 4);
        assert!(op_is_barrier(&fd, op));
        assert!(op_is_write_barrier(&fd, op));
    }

    /// `glob = COPY glob`: what `RuleIndirectCollapse` leaves where a call turned
    /// out not to write a global it was given an INDIRECT for.  It stores the
    /// value already at that address, so it is not a write anything can observe
    /// and must not decline a fold — while the raw storage test, which
    /// `foldcallretphi` composes unchanged, still reports the write.
    #[test]
    fn a_copy_of_a_global_into_itself_is_not_a_barrier() {
        let mut fd = build_fd();
        let g = glob(&mut fd, 0x4000, 4);
        let op = write_op(&mut fd, OpCode::CPUI_COPY, &[g], 0x4000, 4);
        assert!(op_writes_tied_storage(&fd, op));
        assert!(!op_is_barrier(&fd, op));
        assert!(!op_is_write_barrier(&fd, op));
    }

    /// One global copied into another is a real write.
    #[test]
    fn a_copy_between_two_globals_is_a_barrier() {
        let mut fd = build_fd();
        let g = glob(&mut fd, 0x4008, 4);
        let op = write_op(&mut fd, OpCode::CPUI_COPY, &[g], 0x4000, 4);
        assert!(op_is_barrier(&fd, op));
        assert!(op_is_write_barrier(&fd, op));
    }

    /// Same address, narrower source: a partial write, not the value already
    /// there.
    #[test]
    fn a_copy_of_a_different_width_at_the_same_address_is_a_barrier() {
        let mut fd = build_fd();
        let g = glob(&mut fd, 0x4000, 1);
        let op = write_op(&mut fd, OpCode::CPUI_COPY, &[g], 0x4000, 4);
        assert!(op_is_barrier(&fd, op));
    }

    /// A volatile location: the access itself is the effect, so the exemption
    /// never applies there.
    #[test]
    fn a_self_copy_of_a_volatile_location_is_a_barrier() {
        let mut fd = build_fd();
        let g = glob(&mut fd, 0x4000, 4);
        fd.vbank_mut().get_mut(g).unwrap().set_flags_pub(varnode_flags::volatil);
        let op = write_op(&mut fd, OpCode::CPUI_COPY, &[g], 0x4000, 4);
        assert!(op_is_barrier(&fd, op));
    }

    /// A register temporary is not storage a callee can name.
    #[test]
    fn a_write_to_an_untied_temporary_is_not_a_barrier() {
        let mut fd = build_fd();
        let c = fd.new_constant(4, 42);
        let pc = Address::new(ram(&fd), 0x1100);
        let op = fd.new_op(1, pc);
        fd.op_set_opcode_code(op, OpCode::CPUI_COPY);
        fd.op_set_input(op, c, 0).unwrap();
        fd.new_unique_out(4, op).unwrap();
        assert!(!op_writes_tied_storage(&fd, op));
        assert!(!op_is_barrier(&fd, op));
        assert!(!op_is_write_barrier(&fd, op));
    }

    /// The two spans differ in exactly one opcode: a `LOAD` holds the call back
    /// up to its single use, and does not past it (module header).  A `STORE`
    /// holds it on both.
    #[test]
    fn load_is_a_barrier_only_up_to_the_use_store_on_both_spans() {
        let mut fd = build_fd();
        let space = fd.new_constant(8, 0);
        let ptr = glob(&mut fd, 0x4000, 8);
        let pc = Address::new(ram(&fd), 0x1100);

        let load = fd.new_op(2, pc.clone());
        fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
        fd.op_set_input(load, space, 0).unwrap();
        fd.op_set_input(load, ptr, 1).unwrap();
        fd.new_unique_out(4, load).unwrap();
        assert!(op_is_barrier(&fd, load));
        assert!(!op_is_write_barrier(&fd, load));

        let space2 = fd.new_constant(8, 0);
        let ptr2 = glob(&mut fd, 0x4008, 8);
        let val = fd.new_constant(4, 7);
        let store = fd.new_op(3, pc);
        fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
        fd.op_set_input(store, space2, 0).unwrap();
        fd.op_set_input(store, ptr2, 1).unwrap();
        fd.op_set_input(store, val, 2).unwrap();
        assert!(op_is_barrier(&fd, store));
        assert!(op_is_write_barrier(&fd, store));
    }
}

//===========================================================================
// The short-circuit operand (GH-684)
//===========================================================================

mod shortcircuit {
    use std::rc::Rc;

    use kuna_base::address::Address;
    use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, IopSpace, UniqueSpace};
    use kuna_num::opcodes::OpCode;

    use crate::context::{ArchContext, OpId, VarnodeId};
    use crate::funcdata::Funcdata;

    use super::super::chain_reaches_short_circuit_rhs;

    fn build_fd() -> Funcdata {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(IopSpace::new(3))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        let glb = Rc::new(ArchContext::new(m));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        Funcdata::new("f", "f", glb, Address::new(ram, 0x1000), 0x1000_0000, 0x40).unwrap()
    }

    fn op(fd: &mut Funcdata, opc: OpCode, ins: &[VarnodeId], out_size: i32) -> (OpId, VarnodeId) {
        let pc = Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap()), 0x1100);
        let o = fd.new_op(ins.len() as i32, pc);
        fd.op_set_opcode_code(o, opc);
        for (slot, vn) in ins.iter().enumerate() {
            fd.op_set_input(o, *vn, slot as i32).unwrap();
        }
        let out = fd.new_unique_out(out_size, o).unwrap();
        (o, out)
    }

    /// `r = tick(); t = (r == 0); c = other && t` (or `||`): the call reaches
    /// input 1 through the comparison, which C skips when `other` decides.
    #[test]
    fn a_call_reaching_input_1_declines_and_input_0_does_not() {
        for bool_op in [OpCode::CPUI_BOOL_AND, OpCode::CPUI_BOOL_OR] {
            for rhs in [true, false] {
                let mut fd = build_fd();
                let target = fd.new_constant(8, 0x2000);
                let (call, r) = op(&mut fd, OpCode::CPUI_CALL, &[target], 4);
                let zero = fd.new_constant(4, 0);
                let (eq, t) = op(&mut fd, OpCode::CPUI_INT_EQUAL, &[r, zero], 1);
                let other = fd.new_constant(1, 1);
                let ins = if rhs { [other, t] } else { [t, other] };
                let (bop, _) = op(&mut fd, bool_op, &ins, 1);
                assert_eq!(chain_reaches_short_circuit_rhs(&fd, call, &[eq, bop]), rhs, "{bool_op:?} rhs={rhs}");
            }
        }
    }

    /// The left-hand operand of an `&&` that is itself the right-hand operand of
    /// an `||` is still skipped when the `||`'s left side decides.
    #[test]
    fn a_left_operand_nested_on_the_right_declines() {
        let mut fd = build_fd();
        let target = fd.new_constant(8, 0x2000);
        let (call, r) = op(&mut fd, OpCode::CPUI_CALL, &[target], 1);
        let a = fd.new_constant(1, 1);
        let (and, t) = op(&mut fd, OpCode::CPUI_BOOL_AND, &[r, a], 1);
        let b = fd.new_constant(1, 0);
        let (or, _) = op(&mut fd, OpCode::CPUI_BOOL_OR, &[b, t], 1);
        assert!(!chain_reaches_short_circuit_rhs(&fd, call, &[and]));
        assert!(chain_reaches_short_circuit_rhs(&fd, call, &[and, or]));
    }

    #[test]
    fn a_separate_snapshot_of_this_calls_effect_keeps_the_call_before_the_branch() {
        for (own_effect, derived, classification, same_high) in [false, true].into_iter().flat_map(|own| {
            [OpCode::CPUI_COPY, OpCode::CPUI_CAST, OpCode::CPUI_INT_ADD].into_iter().flat_map(move |code| {
                [0, 1, 2].into_iter().flat_map(move |classification| {
                    [false, true].map(|same_high| (own, code, classification, same_high))
                })
            })
        }) {
            let mut fd = build_fd();
            let root = fd.bblocks_root_pub();
            let block = fd.bblocks_mut().new_block_basic(root);
            let target = fd.new_constant(8, 0x2000);
            let (other, _) = op(&mut fd, OpCode::CPUI_CALL, &[target], 4);
            fd.op_insert_end(other, block);
            let (call, result) = op(&mut fd, OpCode::CPUI_CALL, &[target], 4);
            let old = fd.new_constant(8, 0);
            let effect = fd.new_varnode_iop(if own_effect { call } else { other });
            let (indirect, updated) = op(&mut fd, OpCode::CPUI_INDIRECT, &[old, effect], 8);
            fd.op_insert_end(indirect, block);
            fd.op_insert_end(call, block);
            let zero = fd.new_constant(4, 0);
            let (compare, condition) = op(&mut fd, OpCode::CPUI_INT_SLESSEQUAL, &[result, zero], 1);
            fd.vbank_mut().get_mut(condition).unwrap().set_implied();
            fd.op_insert_end(compare, block);
            let source = if derived == OpCode::CPUI_COPY {
                updated
            } else {
                let one = fd.new_constant(8, 1);
                let inputs = if derived == OpCode::CPUI_CAST { vec![updated] } else { vec![updated, one] };
                let (expression, source) = op(&mut fd, derived, &inputs, 8);
                fd.vbank_mut().get_mut(source).unwrap().set_implied();
                fd.op_insert_end(expression, block);
                source
            };
            let (copy, snapshot) = op(&mut fd, OpCode::CPUI_COPY, &[source], 8);
            if same_high {
                fd.set_high_level();
                let high = fd.vbank().get(source).unwrap().get_high().unwrap();
                fd.vbank_mut().get_mut(snapshot).unwrap().set_high(high, 0);
            }
            if classification == 1 {
                fd.vbank_mut().get_mut(snapshot).unwrap().set_explicit();
            } else if classification == 2 {
                fd.vbank_mut().get_mut(snapshot).unwrap().set_implied();
            }
            fd.op_insert_end(copy, block);
            let branch = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(branch, OpCode::CPUI_CBRANCH);
            fd.op_set_all_input(branch, &[target, condition]).unwrap();
            fd.op_insert_end(branch, block);

            assert!(super::super::call_output_foldable(&fd, result));
            assert_eq!(super::super::fold_print_point_is_order_safe(&fd, result), !(own_effect && classification != 2 && !same_high), "own={own_effect} derived={derived:?} classification={classification} same_high={same_high}");
        }
    }

    #[test]
    fn an_unclassified_input_expression_cannot_hide_a_call_effect_read() {
        for derived in [OpCode::CPUI_CAST, OpCode::CPUI_INT_ADD] {
            for explicit_input in [false, true] {
                let mut fd = build_fd();
                let target = fd.new_constant(8, 0x2000);
                let (call, _) = op(&mut fd, OpCode::CPUI_CALL, &[target], 4);
                let old = fd.new_constant(8, 0);
                let effect = fd.new_varnode_iop(call);
                let (_, updated) = op(&mut fd, OpCode::CPUI_INDIRECT, &[old, effect], 8);
                let one = fd.new_constant(8, 1);
                let inputs = if derived == OpCode::CPUI_CAST { vec![updated] } else { vec![updated, one] };
                let (_, expression) = op(&mut fd, derived, &inputs, 8);
                if explicit_input {
                    fd.vbank_mut().get_mut(expression).unwrap().set_explicit();
                }
                let (copy, snapshot) = op(&mut fd, OpCode::CPUI_COPY, &[expression], 8);
                fd.vbank_mut().get_mut(snapshot).unwrap().set_explicit();
                assert_eq!(super::super::statement_reads_indirect_output_of(&fd, copy, call), !explicit_input, "derived={derived:?} explicit_input={explicit_input}");
            }
        }
    }
}
