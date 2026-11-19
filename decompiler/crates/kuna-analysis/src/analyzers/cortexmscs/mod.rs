//! (kuna `cortexmscs`) The Cortex-M System Control Space as typed volatile
//! globals.
//!
//! Every ARMv7-M and ARMv8-M core maps the same registers at fixed addresses
//! of the Private Peripheral Bus: the System Control Block, NVIC, SysTick,
//! MPU, FPU and debug control at 0xE000E000, and ITM, DWT, FPB and TPI below
//! and above it. On a Cortex-M language (`ARM:*:32:Cortex` or
//! `ARM:*:32:v8-m`), or an ARM ELF whose build attributes state the M
//! profile, each block becomes a structure named as CMSIS-Core names it
//! (`SCB_Type`, `NVIC_Type`, ...) and a volatile global at its address, so a
//! store to 0xE000ED08 reads `SCB.VTOR = ...`.
//!
//! The pass only states which layout applies; the commit builds the types
//! ([`materialize`]), so a run with the option off creates none of them.
//!
//! The layouts are [`tables`], generated from the CMSIS-Core headers by
//! `scripts/gen_cortexmscs.py`. CMSIS structures overlap one another; the
//! generator cuts each at the next block's base and states the registers
//! beyond the cut as globals of their own (`SCB_STIR`, `SCB_ICIALLU`).

mod tables;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase, TypedDataFact};
use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory, TypeField};
use std::rc::Rc;

/// The pass id and option name.
pub const ID: &str = "cortexmscs";

/// One global: a register block, or a single register when `type_name` is
/// empty. A field is `(name, offset, element size, element count)`.
#[derive(Debug)]
pub struct Block {
    pub name: &'static str,
    pub type_name: &'static str,
    pub base: u64,
    pub fields: &'static [(&'static str, u32, u8, u32)],
}

impl Block {
    /// Bytes from the base to the end of the last register, a multiple of 4.
    pub fn extent(&self) -> u64 {
        let end = self.fields.iter().map(|&(_, off, size, count)| u64::from(off) + u64::from(size) * u64::from(count)).max();
        end.unwrap_or(0).next_multiple_of(4)
    }
}

/// The blocks of the language `archid`, when it is a Cortex-M one.
pub fn layout(archid: &str) -> Option<&'static [Block]> {
    let mut parts = archid.split(':');
    if parts.next() != Some("ARM") {
        return None;
    }
    match parts.nth(2) {
        Some("Cortex") => Some(tables::V7M),
        Some("v8-m") => Some(tables::V8M),
        _ => None,
    }
}

/// Whether a structure of either layout is named `name`. A CMSIS-SVD
/// peripheral of the same name needs another one for its own layout.
pub fn names_a_type(name: &str) -> bool {
    !name.is_empty() && [tables::V7M, tables::V8M].iter().any(|blocks| blocks.iter().any(|b| b.type_name == name))
}

fn field_type(types: &dyn TypeFactory, size: u8, count: u32) -> Option<Rc<Datatype>> {
    let base = types.get_base(i32::from(size), type_metatype::TYPE_UINT).ok()?;
    if count == 1 {
        return Some(base);
    }
    types.get_type_array(i32::try_from(count).ok()?, base).ok()
}

/// The type of `block`, or `None` when its structure name is already held by
/// a type of another shape.
fn block_type(types: &dyn TypeFactory, block: &Block) -> Option<Rc<Datatype>> {
    if block.type_name.is_empty() {
        let &(_, _, size, count) = block.fields.first()?;
        return field_type(types, size, count);
    }
    let size = i32::try_from(block.extent()).ok()?;
    if let Some(existing) = types.find_by_name(block.type_name).ok()? {
        let same = existing.get_metatype() == type_metatype::TYPE_STRUCT && !existing.is_incomplete() && existing.get_size() == size;
        return same.then_some(existing);
    }
    let mut fields = Vec::with_capacity(block.fields.len());
    for (i, &(name, off, size, count)) in block.fields.iter().enumerate() {
        fields.push(TypeField::new(i as i32, i32::try_from(off).ok()?, name, field_type(types, size, count)?));
    }
    let shell = types.get_type_struct(block.type_name).ok()?;
    types.set_fields_struct_raw(&shell, fields, Vec::new(), size, 4, 0).ok()
}

/// The blocks of an ARM ELF whose build attributes state the M profile:
/// the ARMv8-M ones when `Tag_CPU_arch` is an ARMv8-M value, else ARMv7-M
/// (which ARMv6-M is a subset of).
pub fn layout_of_object(file: &object::File<'_>) -> Option<&'static [Block]> {
    let arch = crate::loader::kuna_armfloatabi::m_profile_arch(file)?;
    Some(if matches!(arch, 16 | 17 | 21) { tables::V8M } else { tables::V7M })
}

/// The typed globals of `blocks`, built in `types`, and the `(address, size)`
/// range of each, to be marked volatile before the global is mapped.
pub fn materialize(types: &dyn TypeFactory, blocks: &[Block]) -> (Vec<TypedDataFact>, Vec<(u64, u64)>) {
    let mut globals = Vec::with_capacity(blocks.len());
    let mut ranges = Vec::with_capacity(blocks.len());
    for block in blocks {
        let Some(type_) = block_type(types, block) else { continue };
        ranges.push((block.base, block.extent()));
        globals.push(TypedDataFact { addr: block.base, name: block.name.to_string(), type_ });
    }
    (globals, ranges)
}

/// The System Control Space pass of an object-file image.
pub struct CortexMScsPass;

impl AnalysisPass for CortexMScsPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        ID
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        AnalysisOutput { cortexm_scs: layout(&ctx.arch.archid).or_else(|| layout_of_object(ctx.file)), ..AnalysisOutput::default() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find<'a>(blocks: &'a [Block], name: &str) -> &'a Block {
        blocks.iter().find(|b| b.name == name).unwrap_or_else(|| panic!("no block {name}"))
    }

    fn offset(block: &Block, field: &str) -> u32 {
        block.fields.iter().find(|f| f.0 == field).unwrap_or_else(|| panic!("no field {field}")).1
    }

    #[test]
    fn only_cortex_m_languages_have_a_layout() {
        assert!(layout("ARM:LE:32:Cortex:default").is_some());
        assert!(layout("ARM:BE:32:v8-m:default").is_some());
        assert!(layout("ARM:LE:32:v8:default").is_none());
        assert!(layout("AARCH64:LE:64:v8A:default").is_none());
        assert!(layout("").is_none());
    }

    #[test]
    fn registers_sit_at_their_architectural_addresses() {
        for blocks in [tables::V7M, tables::V8M] {
            let scb = find(blocks, "SCB");
            assert_eq!(scb.base + u64::from(offset(scb, "VTOR")), 0xe000_ed08);
            assert_eq!(scb.base + u64::from(offset(scb, "CPACR")), 0xe000_ed88);
            assert_eq!(find(blocks, "NVIC").base, 0xe000_e100);
            assert_eq!(find(blocks, "SysTick").base, 0xe000_e010);
            assert_eq!(find(blocks, "SCB_STIR").base, 0xe000_ef00);
            let fpu = find(blocks, "FPU");
            assert_eq!(fpu.base + u64::from(offset(fpu, "FPCCR")), 0xe000_ef34);
        }
        let dcb = find(tables::V8M, "DCB");
        assert_eq!(dcb.base + u64::from(offset(dcb, "DEMCR")), 0xe000_edfc);
    }

    #[test]
    fn structure_names_are_recognized() {
        assert!(names_a_type("SCB_Type") && names_a_type("SAU_Type"));
        assert!(!names_a_type("") && !names_a_type("USART1_Type"));
    }

    #[test]
    fn globals_do_not_overlap() {
        for blocks in [tables::V7M, tables::V8M] {
            for pair in blocks.windows(2) {
                assert!(pair[0].base + pair[0].extent() <= pair[1].base, "{} overlaps {}", pair[0].name, pair[1].name);
            }
        }
    }
}
