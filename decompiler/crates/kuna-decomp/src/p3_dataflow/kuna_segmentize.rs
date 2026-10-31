//! (kuna `segmentize`) Segmented addressing: the processor's `<segmentop>`
//! applied to the analysis, as upstream does.
//!
//! A segmented processor's SLEIGH spec computes every effective address through
//! a user-defined p-code op. x86 real mode writes `segment(SS,SP)` for each push
//! and pop, `segment(DS,addr16)` for each data access and `EIP = segment(CS,IP)`
//! before each return; Z80 and HCS12 do the same with their bank registers. The
//! processor spec names that op in a `<segmentop>` element with a p-code
//! snippet for the address it computes (`res = (zext(base) << 4) + zext(inner)`
//! in real mode) and the register that holds the implied segment of a constant
//! near pointer (`DS`).
//!
//! Without this the op stays an opaque CALLOTHER, so every access prints as a
//! call (`*(int *)segment(DS,0x1234)`, `segment(CS,*(unsigned short *)segment(
//! SS,&Stack0000))` for a `RET`), the stack is never recognized through it and
//! no constant pointer is ever resolved.
//!
//! On, upstream's machinery runs:
//! - `ActionSegmentize` rewrites each such CALLOTHER into the internal
//!   `CPUI_SEGMENTOP`, which the stack and type passes see through and which
//!   prints as its near pointer alone.
//! - `RuleSegment` folds a SEGMENTOP whose two inputs are constants through the
//!   snippet, and replaces one whose segment and offset are the two halves of a
//!   single far pointer with that pointer.
//! - A stack reference built against the stack's containing space gets the same
//!   SEGMENTOP (`Funcdata::createStackRef`).
//! - `SegmentedResolver` reads a constant pointer into the space as near (with
//!   the segment register's tracked value) or far (segment in the high bytes),
//!   and the space accepts near-pointer-sized constants as pointers.
//! - A far-pointer segment op sets the data organization's pointer size to the
//!   near size and its alternate pointer size to the far size.
//!
//! Off restores all of it, leaving the CALLOTHER untouched.

use std::cell::RefCell;
use std::rc::Rc;

use kuna_base::address::{calc_mask, Address};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{AddrSpace, AddrSpaceManager};
use kuna_base::types::{int4, uintb};
use kuna_num::opbehavior::OpBehavior;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::emulate::Emulate;
use kuna_sleigh::emulateutil::EmulateSnippet;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::semantics::ConstructTpl;

use crate::inject_sleigh::SleighInjectEngine;
use crate::pcodeinject::{InjectArchitecture, InjectContext, InjectPayload, SnippetLayout};
use crate::userop::SegmentOp;

/// The `segmentize` option value parser.
pub struct OptionSegmentize;

impl OptionSegmentize {
    /// The option name.
    pub const NAME: &'static str = "segmentize";

    /// Parse the `on`/`off` value.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = crate::options::on_or_off(p1)?;
        Ok((val, format!("Segmented address analysis turned {}", if val { "on" } else { "off" })))
    }
}

/// One `<segmentop>` with its compiled snippet, shared into the per-function
/// architecture snapshot (C++ `SegmentOp` with its `ExecutablePcode`).
pub struct SegmentDef {
    /// The CALLOTHER index of the user-op (C++ `useropindex`).
    pub useropindex: int4,
    /// The space a segmented pointer points into (C++ `spc`).
    pub spc: Rc<AddrSpace>,
    /// Size in bytes of the segment value (C++ `baseinsize`).
    pub baseinsize: int4,
    /// Size in bytes of the near pointer (C++ `innerinsize`).
    pub innerinsize: int4,
    /// Whether segment:near acts as a far pointer (C++ `supportsfarpointer`).
    pub supportsfarpointer: bool,
    /// The register holding a constant near pointer's segment (C++ `constresolve`).
    pub constresolve: VarnodeData,
    script: RefCell<ExecutableScript>,
}

/// The emulator a snippet runs in (C++ `ExecutablePcode` after `build`).
struct ExecutableScript {
    emulator: EmulateSnippet,
    input_list: Vec<uintb>,
    output_list: Vec<uintb>,
}

/// A load image with no bytes: a legal snippet only reads temporaries.
struct NoImage;

impl LoadImage for NoImage {
    fn get_file_name(&self) -> &str {
        "snippet"
    }
    fn load_fill(&mut self, _ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
        Err(KunaError::data_unavail(format!(
            "Snippet emulation cannot read memory at {:#x}",
            addr.get_offset()
        )))
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

impl SegmentDef {
    /// Build the snippet emulator for `seg` (C++ `ExecutablePcode::build`).
    pub fn build(
        seg: &SegmentOp,
        useropindex: int4,
        payload: &dyn InjectPayload,
        tpl: &ConstructTpl,
        arch: &dyn InjectArchitecture,
        manager: Rc<AddrSpaceManager>,
        behaviors: &[Option<Rc<dyn OpBehavior>>],
    ) -> KunaResult<SegmentDef> {
        let spc = seg
            .get_space()
            .cloned()
            .ok_or_else(|| KunaError::lowlevel("<segmentop> expecting space attribute"))?;
        let const_space = manager
            .get_constant_space()
            .cloned()
            .ok_or_else(|| KunaError::lowlevel("No constant space"))?;
        let code_space = arch.get_default_code_space();
        let mut context = InjectContext::default();
        context.baseaddr = Some(Address::new(Rc::clone(&code_space), 0x1000));
        context.nextaddr = context.baseaddr.clone();
        let layout = SnippetLayout::build(payload.core(), &mut context, arch);
        let loader: Rc<RefCell<dyn LoadImage>> = Rc::new(RefCell::new(NoImage));
        let mut emulator = EmulateSnippet::new(loader, manager);
        {
            let mut emitter = emulator.build_emitter(behaviors, layout.uniq_reserve);
            let engine = SleighInjectEngine::new(const_space, arch.get_unique_space(), code_space);
            engine.emit_payload(payload, tpl, &mut context, &mut emitter)?;
        }
        if !emulator.check_for_legal_code() {
            return Err(KunaError::lowlevel("Illegal p-code in executable snippet"));
        }
        Ok(SegmentDef {
            useropindex,
            spc,
            baseinsize: seg.get_base_size(),
            innerinsize: seg.get_inner_size(),
            supportsfarpointer: seg.has_far_pointer_support(),
            constresolve: seg.get_resolve().clone(),
            script: RefCell::new(ExecutableScript {
                emulator,
                input_list: layout.input_list,
                output_list: layout.output_list,
            }),
        })
    }

    /// Get the number of input Varnodes expected (C++ `getNumVariableTerms`).
    pub fn get_num_variable_terms(&self) -> int4 {
        if self.baseinsize != 0 {
            2
        } else {
            1
        }
    }

    /// Whether a constant near pointer can be resolved, i.e. the spec names its
    /// segment register (the C++ `Architecture::getSegmentOp(spc)` filter).
    pub fn has_resolve(&self) -> bool {
        self.constresolve.space.is_some()
    }

    /// Compute the address for constant inputs (C++ `SegmentOp::execute` via
    /// `ExecutablePcode::evaluate`).
    pub fn execute(&self, input: &[uintb]) -> KunaResult<uintb> {
        let mut script = self.script.borrow_mut();
        let script = &mut *script;
        script.emulator.reset_memory();
        if input.len() != script.input_list.len() {
            return Err(KunaError::lowlevel("Wrong number of input parameters to executable snippet"));
        }
        let Some(&out) = script.output_list.first() else {
            return Err(KunaError::lowlevel("No registered outputs to executable snippet"));
        };
        for (slot, val) in script.input_list.iter().zip(input) {
            script.emulator.set_varnode_value(*slot, *val);
        }
        while !script.emulator.get_halt() {
            script.emulator.execute_current_op()?;
        }
        Ok(script.emulator.get_temp_value(out))
    }

    /// Resolve a constant pointer into [`spc`](Self::spc) (C++
    /// `SegmentedResolver::resolve`). A constant no wider than the near pointer
    /// takes its segment from the tracked value of the resolve register at
    /// `point`; anything wider, or `sz == -1`, is a far pointer with the segment
    /// in its high bytes. Returns an invalid address when neither applies.
    pub fn resolve(
        &self,
        val: uintb,
        sz: int4,
        point: &Address,
        full_encoding: &mut uintb,
        tracked: impl FnOnce(&VarnodeData, &Address) -> uintb,
    ) -> Address {
        let innersz = self.innerinsize;
        let (base, near) = if sz >= 0 && sz <= innersz {
            if !self.has_resolve() {
                return Address::new_invalid();
            }
            let base = tracked(&self.constresolve, point);
            *full_encoding = (base.wrapping_shl(8 * innersz as u32)).wrapping_add(val & calc_mask(innersz));
            (base, val)
        } else {
            *full_encoding = val;
            let base = val.wrapping_shr(8 * innersz as u32) & calc_mask(self.baseinsize);
            (base, val & calc_mask(innersz))
        };
        match self.execute(&[base, near]) {
            Ok(res) => Address::new(
                Rc::clone(&self.spc),
                AddrSpace::address_to_byte(res, self.spc.get_word_size()),
            ),
            Err(_) => Address::new_invalid(),
        }
    }
}

/// Does a jump table of segmented code target bytes the load image lacks?  A
/// `jmp [cs:bx+table]` in a code segment the spec's 64 KiB `currentCS` guess gets
/// wrong reads arbitrary words; such a table is rejected whole (the jump stays a
/// jump-as-call), since truncating would keep the rows before the first miss.
/// Only a jump whose target is worked out through a SEGMENTOP is judged: a Z80
/// spec declares a segment op its unsegmented build never emits, and a `jp (hl)`
/// table there with a row in RAM or the BIOS is followed as upstream does.
pub fn table_leaves_image(
    fd: &crate::funcdata::Funcdata,
    indop: crate::context::OpId,
    addresstable: &[Address],
) -> bool {
    let arch = fd.get_arch();
    if arch.num_segment_ops() == 0 || !target_through_segment(fd, indop) {
        return false;
    }
    let Some(loader) = arch.loader.as_ref() else { return false };
    let Ok(mut loader) = loader.try_borrow_mut() else { return false };
    let mut probe = [0u8; 1];
    addresstable.iter().any(|addr| loader.load_fill(&mut probe, addr).is_err())
}

/// Is the indirect branch `indop`'s target, or the table read that feeds it,
/// computed by a SEGMENTOP?  A bounded walk up the target's definitions.
fn target_through_segment(fd: &crate::funcdata::Funcdata, indop: crate::context::OpId) -> bool {
    use kuna_num::opcodes::OpCode;
    let mut work: Vec<crate::context::VarnodeId> = fd.obank().get(indop).and_then(|o| o.get_in(0)).into_iter().collect();
    let mut budget = 64u32;
    while let Some(vn) = work.pop() {
        if budget == 0 {
            return false;
        }
        budget -= 1;
        let Some(op) = fd.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| fd.obank().get(d)) else { continue };
        if op.code() == OpCode::CPUI_SEGMENTOP {
            return true;
        }
        work.extend((0..op.num_input()).filter_map(|i| op.get_in(i)));
    }
    false
}

/// The values the option changes outside the per-function snapshot, kept so
/// `off` can put them back.
#[derive(Debug, Clone)]
pub struct SegmentSaved {
    /// Data organization pointer size before the far-pointer adjustment.
    pub size_of_pointer: int4,
    /// Alternate (far) pointer size before the adjustment.
    pub size_of_alt_pointer: int4,
    /// Each marked space with its prior near-pointer flag and minimum pointer size.
    pub spaces: Vec<(Rc<AddrSpace>, bool, i32)>,
}
