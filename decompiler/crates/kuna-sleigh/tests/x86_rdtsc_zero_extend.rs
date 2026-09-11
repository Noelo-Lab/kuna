//! Direct x86-64 lift regression for `RDTSC` (`0f 31`).
//!
//! The instruction writes both 32-bit halves of the timestamp and, in 64-bit
//! mode, clears the upper halves of their parent registers.  Pin the raw p-code
//! so stale values in RAX/RDX cannot survive the lift.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::sleigh::Sleigh;
use kuna_sleigh::translate::{PcodeEmit, Translate};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

struct MemImg {
    base: u64,
    bytes: Vec<u8>,
}

impl LoadImage for MemImg {
    fn get_file_name(&self) -> &str {
        "rdtsc"
    }

    fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
        let start = addr.get_offset();
        for (i, byte) in ptr.iter_mut().enumerate() {
            *byte = start
                .wrapping_add(i as u64)
                .checked_sub(self.base)
                .and_then(|offset| self.bytes.get(offset as usize).copied())
                .unwrap_or(0);
        }
        Ok(())
    }

    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }

    fn adjust_vma(&mut self, _adjust: i64) {}
}

struct DummyImg;

impl LoadImage for DummyImg {
    fn get_file_name(&self) -> &str {
        "dummy"
    }

    fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
        Err(KunaError::data_unavail("dummy"))
    }

    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }

    fn adjust_vma(&mut self, _adjust: i64) {}
}

#[derive(Debug)]
struct RawOp {
    opcode: OpCode,
    output: Option<VarnodeData>,
    inputs: Vec<VarnodeData>,
}

#[derive(Default)]
struct RawEmit {
    ops: Vec<RawOp>,
}

impl PcodeEmit for RawEmit {
    fn dump(
        &mut self,
        _addr: &Address,
        opcode: OpCode,
        output: Option<&VarnodeData>,
        inputs: &[VarnodeData],
    ) {
        self.ops.push(RawOp {
            opcode,
            output: output.cloned(),
            inputs: inputs.to_vec(),
        });
    }
}

fn zext_index(ops: &[RawOp], output: &VarnodeData, input: &VarnodeData) -> Option<usize> {
    ops.iter().position(|op| {
        op.opcode == OpCode::CPUI_INT_ZEXT
            && op.output.as_ref() == Some(output)
            && op.inputs.len() == 1
            && &op.inputs[0] == input
    })
}

fn subpiece_write_index(ops: &[RawOp], output: &VarnodeData) -> Option<usize> {
    ops.iter()
        .position(|op| op.opcode == OpCode::CPUI_SUBPIECE && op.output.as_ref() == Some(output))
}

#[test]
fn rdtsc_zero_extends_eax_and_edx_into_rax_and_rdx() {
    let sla_path = repo_root().join("specs/Ghidra/Processors/x86/data/languages/x86-64.sla");
    let Ok(sla) = std::fs::read(&sla_path) else {
        eprintln!(
            "x86_rdtsc_zero_extend: skipping (no `{}`; compile x86-64.slaspec)",
            sla_path.display()
        );
        return;
    };

    let base = 0x1000;
    let ctx = Box::new(ContextInternal::new());
    let mut sleigh = Sleigh::new(Box::new(DummyImg), ctx);
    sleigh
        .initialize_from_sla(&sla)
        .expect("initialize x86-64.sla");
    sleigh.set_context_default("addrsize", 2);
    sleigh.set_context_default("opsize", 1);
    sleigh.set_context_default("rexprefix", 0);
    sleigh.set_context_default("longMode", 1);
    sleigh.set_loader(Box::new(MemImg {
        base,
        bytes: vec![0x0f, 0x31],
    }));

    let ram = Rc::clone(
        sleigh
            .base()
            .manager()
            .get_space_by_name("ram")
            .expect("ram space"),
    );
    let address = Address::new(ram, base);
    let eax = sleigh.get_register_varnode(b"EAX").expect("EAX");
    let edx = sleigh.get_register_varnode(b"EDX").expect("EDX");
    let rax = sleigh.get_register_varnode(b"RAX").expect("RAX");
    let rdx = sleigh.get_register_varnode(b"RDX").expect("RDX");

    let mut emit = RawEmit::default();
    assert_eq!(sleigh.one_instruction(&mut emit, &address).unwrap(), 2);
    let edx_write = subpiece_write_index(&emit.ops, &edx)
        .unwrap_or_else(|| panic!("RDTSC must write EDX; raw ops: {:?}", emit.ops));
    let rdx_zext = zext_index(&emit.ops, &rdx, &edx).unwrap_or_else(|| {
        panic!(
            "RDTSC must clear RDX[63:32] after writing EDX; raw ops: {:?}",
            emit.ops
        )
    });
    let eax_write = subpiece_write_index(&emit.ops, &eax)
        .unwrap_or_else(|| panic!("RDTSC must write EAX; raw ops: {:?}", emit.ops));
    let rax_zext = zext_index(&emit.ops, &rax, &eax).unwrap_or_else(|| {
        panic!(
            "RDTSC must clear RAX[63:32] after writing EAX; raw ops: {:?}",
            emit.ops
        )
    });

    assert_eq!(
        rdx_zext,
        edx_write + 1,
        "EDX write must be immediately followed by RDX zero-extension; raw ops: {:?}",
        emit.ops
    );
    assert_eq!(
        rax_zext,
        eax_write + 1,
        "EAX write must be immediately followed by RAX zero-extension; raw ops: {:?}",
        emit.ops
    );
}
