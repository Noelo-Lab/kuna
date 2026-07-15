//! Regression gate for the `ParserContext::expandState` walker-cursor rebase
//! (kuna port bug in `sleigh.rs::allocate_operand`).
//!
//! Decoding the ARM Thumb-2 instruction `vldmia r0, {d16-d31}`
//! (bytes `d0 ec 20 0b`, little-endian) resolves an operand tree deep enough to
//! exhaust the `INITIAL_STATE_NUM` (64) node pool, forcing
//! `ParserContext::expandState`.  In upstream C++ (`context.cc`/`context.hh`)
//! `expandState` front-inserts nodes (`state.insert(state.begin(), amount, ...)`)
//! and the walker's `point` is a raw pointer, so the reshuffle never invalidates
//! it.  kuna models `point` as a `Vec` index; the front-insertion shifts every
//! stored index by `amount`, and the *live* walker cursor must be rebased too.
//! Before the fix the stale cursor corrupted the walk and drove `depth` to `-1`,
//! panicking in `ParserWalker::getOperand` (`breadcrumb[depth as usize]`, index
//! `usize::MAX`).  Observed in the wild on stripped ARM Cortex-M firmware
//! (libopencm3 `usart_irq_console.elf` @ 0x8001d1c): the whole-binary load
//! panicked, producing zero output.
//!
//! This test lifts exactly that instruction against the built `ARM8_le.sla`
//! (`ARM:LE:32:v8`) in Thumb mode and asserts it decodes cleanly (no panic,
//! 4-byte instruction).  It is skipped when the `.sla` is absent (`make specs`).

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
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// In-memory image: the instruction bytes at `base`, zero-filled elsewhere so
/// the decoder's read-ahead never errors.
struct MemImg {
    base: u64,
    bytes: Vec<u8>,
}

impl LoadImage for MemImg {
    fn get_file_name(&self) -> &str {
        "mem"
    }
    fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
        let start = addr.get_offset();
        for (i, b) in ptr.iter_mut().enumerate() {
            let a = start.wrapping_add(i as u64);
            *b = a
                .checked_sub(self.base)
                .and_then(|off| self.bytes.get(off as usize).copied())
                .unwrap_or(0);
        }
        Ok(())
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

/// A p-code sink that only needs to prove the lift ran (counts emitted ops).
struct CountEmit {
    ops: usize,
}
impl PcodeEmit for CountEmit {
    fn dump(
        &mut self,
        _addr: &Address,
        _opc: OpCode,
        _outvar: Option<&VarnodeData>,
        _vars: &[VarnodeData],
    ) {
        self.ops += 1;
    }
}

/// A stub loader used only until the real image is installed.
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

#[test]
fn arm_thumb_vldmia_upper_neon_regs_does_not_panic() {
    let sla_path = repo_root()
        .join("specs/Ghidra/Processors/ARM/data/languages/ARM8_le.sla");
    let Ok(sla) = std::fs::read(&sla_path) else {
        eprintln!(
            "arm_neon_vldm_regression: skipping (no `{}`; run `make specs`)",
            sla_path.display()
        );
        return;
    };

    let base: u64 = 0x1000;
    // `vldmia r0, {d16-d31}` (d0 ec 20 0b) then `bx lr` / nop padding, matching
    // the shape seen on the firmware; the trailing bytes cover any read-ahead.
    let bytes = vec![0xd0, 0xec, 0x20, 0x0b, 0x70, 0x47, 0x00, 0xbf, 0, 0, 0, 0, 0, 0, 0, 0];

    let ctx = Box::new(ContextInternal::new());
    let mut sleigh = Sleigh::new(Box::new(DummyImg), ctx);
    sleigh.initialize_from_sla(&sla).expect("initialize ARM8_le.sla");
    // Thumb decode mode (ARM.sinc: `TMode = (0,0)`, 1 => Thumb).
    sleigh.set_context_default("TMode", 1);

    sleigh.set_loader(Box::new(MemImg { base, bytes }));

    let ram = Rc::clone(
        sleigh.base().manager().get_space_by_name("ram").expect("ram space"),
    );
    let addr = Address::new(ram, base);

    let mut emit = CountEmit { ops: 0 };
    // Before the fix this panicked in ParserWalker::getOperand (depth == -1)
    // while resolving the operand tree after expandState front-inserted nodes.
    let len = sleigh
        .one_instruction(&mut emit, &addr)
        .expect("vldmia {d16-d31} must lift without error");
    assert_eq!(len, 4, "Thumb-2 vldmia is a 4-byte instruction (got len {len})");
    assert!(emit.ops > 0, "the lift must emit at least one p-code op");
}
