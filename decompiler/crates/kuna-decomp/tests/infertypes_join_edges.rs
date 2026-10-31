//! Type propagation skips a join's other input slots once it enters the join at
//! one input. That is exact only while `MULTIEQUAL` and `INDIRECT` refuse every
//! input-to-input edge and still carry an input to their output.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use kuna_decomp::context::{ArchContext, OpId, TypeOp, VarnodeId};
use kuna_decomp::coreaction_infertypes::propagate_type_pub;
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;

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
    Funcdata::new("func", "func", glb, Address::new(ram, 0x1000), 0x10000000, 0x40).unwrap()
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

/// A `opc` op reading `n` register inputs into a register output.
fn mk_join(fd: &mut Funcdata, opc: OpCode, n: int4) -> (OpId, Vec<VarnodeId>, VarnodeId) {
    let r = ram(fd);
    let op = fd.new_op(n, Address::new(Rc::clone(&r), 0x100));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    let unk = Rc::new(Datatype::new(4, type_metatype::TYPE_UNKNOWN));
    let ins: Vec<VarnodeId> = (0..n)
        .map(|i| {
            let vn = fd.vbank_mut().create(4, Address::new(Rc::clone(&r), 0x10 * (i as u64 + 1)), Rc::clone(&unk));
            let vn = fd.vbank_mut().set_input(vn, &mut |_: &mut kuna_decomp::varnode::VarnodeBank, _, _| Ok(())).unwrap();
            fd.op_set_input(op, vn, i).unwrap();
            vn
        })
        .collect();
    let out = fd.new_varnode(4, &Address::new(r, 0x400), Some(unk));
    fd.op_set_output(op, out).unwrap();
    (op, ins, out)
}

#[test]
fn joins_carry_an_input_only_to_their_output() {
    let types = [
        Rc::new(Datatype::new(4, type_metatype::TYPE_INT)),
        Rc::new(Datatype::new(4, type_metatype::TYPE_UINT)),
        Rc::new(Datatype::new(4, type_metatype::TYPE_FLOAT)),
    ];
    for (opc, n) in [(OpCode::CPUI_MULTIEQUAL, 5), (OpCode::CPUI_INDIRECT, 2)] {
        let mut fd = build_fd();
        let (op, ins, out) = mk_join(&mut fd, opc, n);
        for ty in &types {
            for i in 0..n {
                for j in 0..n {
                    if i != j {
                        let got = propagate_type_pub(&mut fd, Rc::clone(ty), op, ins[i as usize], ins[j as usize], i, j);
                        assert!(got.is_none(), "{opc:?} carried input {i} to input {j}");
                    }
                }
            }
            let to_out = propagate_type_pub(&mut fd, Rc::clone(ty), op, ins[0], out, 0, -1);
            assert!(to_out.is_some(), "{opc:?} no longer carries an input to its output");
        }
    }
}
