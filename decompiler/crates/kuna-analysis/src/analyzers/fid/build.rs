//! The `.fid` generator — kuna's `FidServiceLibraryIngest` analog.
//!
//! Disassembles every named function in a parsed object (a `.o` member or a
//! linked image) through the live SLEIGH decoder, runs the byte-exact PR2
//! [`FidHasher`](crate::fid::hash::FidHasher) over each function's extent, and
//! emits the deduplicated [`FidRecord`](crate::fid::db::FidRecord) rows the
//! [`FidDb`](crate::fid::db::FidDb) serializes.
//!
//! This is the §5.2 generator the `kuna fid build` CLI subcommand drives
//! (PR3). It is **not** wired into the decompile path — it is a build-time tool
//! that produces the fixture `.fid`, never reached from a decompilation.
//!
//! # The function extent
//!
//! The generator's body source is the loader's **rebased** function symbols
//! ([`ObjectLoadImage::func_symbols`]) — the `(load_vma, name)` pairs the engine
//! installs. For an `ET_REL` `.o` these are the *load* VMAs (the loader laid each
//! `SHF_ALLOC` section out above `RELOC_BASE`); the raw `object::File` symbol
//! addresses are section-relative and would miss every instruction.
//!
//! The builder and lookup share the reachable extent in [`crate::fid::extent`],
//! bounded by the next function entry. Jump tables are read from the live,
//! relocated load image, not raw section-relative object bytes.
//!
//! # Faithfulness to Ghidra's `hashAllTheFunctions`
//!
//! Only functions that (a) have a **real** symbol name and (b) hash to a
//! non-`None` quad (≥ 4 code units after skips) are kept. Functions whose body
//! fails to decode are skipped, not fatal. The per-instruction fingerprint
//! assembly (`InsnMask` + `is_call` + the address range →
//! [`InsnFingerprint`](crate::fid::hash::InsnFingerprint)) is the same
//! projection the PR4 matching pass uses, so a function fingerprints identically
//! whether it is *built into* a DB here or *looked up* against one later.

use std::cell::OnceCell;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_decomp::architecture::Architecture;
use kuna_sleigh::sleigh::{OpObject as SleighOpObject, Sleigh};
use kuna_sleigh::translate::Translate;

use crate::loadimage_object::ObjectLoadImage;
use crate::fid::db::{FidRecord, FLAG_HAS_TERMINATOR};
use crate::fid::hash::{
    FidHasher, InsnFingerprint, OpObject, OperandFingerprint, RelocationQuery,
};
use crate::fid::skipper::{InstructionSkipper, X86InstructionSkipper};

/// Build the FID records for every named, hashable function in `bytes` (a parsed
/// object image).
///
/// The arguments mirror [`crate::passes::run_listing_consumers`]: the raw image
/// `bytes`, the opened `image` loader (the rebased funcsym source), the resolved
/// `arch`, and the engine `translate` (the live SLEIGH decoder, reached as
/// `arch.translate()`). The caller (the CLI) bootstraps these via
/// `kuna_console::bootstrap_from_object`.
///
/// Returns the records in function (address) order. Identical `(full, specific,
/// name)` rows are deduplicated. Never panics: a function whose body fails to
/// decode is skipped, not fatal.
pub fn build_records(
    bytes: &[u8],
    image: &ObjectLoadImage,
    arch: &Architecture,
    translate: &Sleigh,
) -> Vec<FidRecord> {
    let Ok(file) = object::File::parse(bytes) else {
        return Vec::new();
    };

    // The code space the per-instruction Addresses are built in (the space the
    // engine decodes through).
    let Some(code_space) = arch.manage().get_default_code_space() else {
        return Vec::new();
    };
    let code_space = Rc::clone(code_space);

    // The loader's REBASED function symbols `(load_vma, name)`, sorted by address
    // so each function's exclusive upper bound is the next entry. This is the
    // load-bearing seed/name source (see the module doc): for an `ET_REL` `.o`
    // the raw `object::File` symbol address is section-relative.
    let mut funcs = image.func_symbols();
    let arm32 = arch.archid.starts_with("ARM:") && arch.archid.contains(":32:");
    funcs.sort_by_key(|(a, _)| if arm32 { (*a & !1, a & 1 == 0) } else { (*a, false) });
    funcs.dedup_by_key(|(a, _)| if arm32 { *a & !1 } else { *a });

    // The relocation oracle for the 4-way scalar rule: a relocation overlapping
    // an operand's value bytes suppresses that scalar to the `0xfeeddead`
    // placeholder (so a relocatable reference does not leak into the hash).
    let relocs = ObjectRelocations::from_object(&file, image);

    // The per-architecture skipper set (x86 ⇒ the NOP/alignment skipper; non-x86
    // ⇒ empty, never skips).
    let is_x86 = arch.archid.starts_with("x86:");

    let dbg = std::env::var("KUNA_FID_DEBUG").is_ok();
    if dbg {
        eprintln!("[fid] funcs={} archid={} is_x86={is_x86}", funcs.len(), arch.archid);
    }

    let mut records: Vec<FidRecord> = Vec::new();
    let mut seen: std::collections::HashSet<(u64, u64, String)> = std::collections::HashSet::new();
    let switches = OnceCell::new();

    for (idx, (entry, name)) in funcs.iter().enumerate() {
        if name.is_empty() {
            continue;
        }
        // The exclusive upper bound is the next function entry — NOT the symbol
        // size. A library object carries sizes and a stripped image does not, so
        // honoring the size made the two sides hash different instruction sets
        // (the object's body stopped before its trailing padding, the stripped
        // image's ran into it) and a database built from the object stopped
        // matching its own linked program. Bounding by the next entry gives both
        // sides the same extent.
        let address = if arm32 { *entry & !1 } else { *entry };
        let next = funcs.get(idx + 1).map(|(a, _)| if arm32 { *a & !1 } else { *a });
        let Some(_scope) = translate.context_scope() else { continue };
        if arm32 && entry & 1 != 0 {
            let addr = Address::new(Rc::clone(&code_space), address);
            if arch.with_context_db_mut(|db| db.set_variable(b"TMode", &addr, 1)).is_err() {
                continue;
            }
        }
        let extent = match crate::fid::extent::decode_extent(
            arch, translate, address, next, None, &code_space, &switches,
        ) {
            Ok(extent) => extent,
            Err(reason) => {
                if dbg {
                    eprintln!("[fid] {name} @ {entry:#x}: {reason}, skip");
                }
                continue;
            }
        };
        if extent.len() < crate::fid::hash::SHORT_HASH_CODE_UNIT_LENGTH as usize {
            if dbg {
                eprintln!("[fid] {name} @ {entry:#x}: extent too short ({}), skip", extent.len());
            }
            continue;
        }

        // Assemble the per-instruction fingerprints via the SLEIGH mask. A decode
        // failure on any instruction drops the whole function (a partially-masked
        // body would mis-hash).
        let mut fps: Vec<InsnFingerprint> = Vec::with_capacity(extent.len());
        let mut decode_ok = true;
        for ins in &extent {
            let addr = Address::new(Rc::clone(&code_space), ins.addr);
            match translate.instruction_mask(&addr) {
                Ok(mask) => fps.push(fingerprint_from_mask(ins.addr, ins.len, ins.is_call, mask)),
                Err(e) => {
                    if dbg {
                        eprintln!("[fid]   instruction_mask @ {:#x} err: {e}", ins.addr);
                    }
                    decode_ok = false;
                    break;
                }
            }
        }
        if !decode_ok {
            if dbg {
                eprintln!("[fid] {name} @ {entry:#x}: instruction_mask failed, skip");
            }
            continue;
        }

        let skippers: Vec<Box<dyn InstructionSkipper>> = if is_x86 {
            vec![Box::new(X86InstructionSkipper)]
        } else {
            vec![]
        };
        let mut hasher = FidHasher::new(skippers);
        let Some(quad) = hasher.hash(&fps, &relocs) else {
            if dbg {
                eprintln!("[fid] {name} @ {entry:#x}: hash None (< 4 usable), skip");
            }
            continue;
        };

        // Dedup identical (full, specific, name) rows.
        let key = (quad.full_hash, quad.specific_hash, name.clone());
        if !seen.insert(key) {
            continue;
        }

        // The terminator flag is informational for the PR5 seeker; set it when the
        // body's last code unit is a return/terminal flow.
        let flags = match extent.last() {
            Some(last) if last.is_terminal => FLAG_HAS_TERMINATOR,
            _ => 0,
        };

        if dbg {
            eprintln!(
                "[fid] {name} @ {entry:#x}: full={:#018x} spec={:#018x} cus={} addl={}",
                quad.full_hash, quad.specific_hash, quad.code_unit_size, quad.specific_addl_size
            );
        }

        records.push(FidRecord {
            full_hash: quad.full_hash,
            specific_hash: quad.specific_hash,
            code_unit_size: quad.code_unit_size,
            specific_addl: quad.specific_addl_size,
            flags,
            name: name.as_str().into(),
        });
    }

    records
}

/// Project a decoded [`InsnMask`](kuna_sleigh::sleigh::InsnMask) + the flow/address
/// info into the hasher's [`InsnFingerprint`]. The field mapping is 1:1 (the
/// hasher's `OpObject` mirrors the SLEIGH one); the only translation is the enum
/// re-tag.
///
/// `pub(crate)` so the FID matching pass ([`crate::fid::FidPass`]) assembles a
/// looked-up function's fingerprints with the **exact same** projection the
/// generator uses — a function fingerprints identically whether it is built into a
/// DB here or matched against one later (the load-bearing build/lookup symmetry).
pub(crate) fn fingerprint_from_mask(
    addr: u64,
    len: u32,
    is_call: bool,
    mask: kuna_sleigh::sleigh::InsnMask,
) -> InsnFingerprint {
    let operands = mask
        .operands
        .into_iter()
        .map(|op| OperandFingerprint {
            value_mask: op.value_mask,
            objects: op.objects.into_iter().map(map_op_object).collect(),
        })
        .collect();
    InsnFingerprint {
        min_addr: addr,
        // getMaxAddress = min + len - 1 (the inclusive end).
        max_addr: addr.wrapping_add(u64::from(len)).wrapping_sub(1),
        bytes: mask.bytes,
        fixed_mask: mask.fixed_mask,
        is_call,
        operands,
    }
}

/// Re-tag the SLEIGH op-object to the hasher's mirror enum (identical shape).
fn map_op_object(o: SleighOpObject) -> OpObject {
    match o {
        SleighOpObject::Scalar { signed, whole_operand, is_address } => {
            OpObject::Scalar { signed, whole_operand, is_address }
        }
        SleighOpObject::Register { offset } => OpObject::Register { offset },
        SleighOpObject::Address => OpObject::Address,
    }
}

/// Sorted load-VMA ranges overlapped by relocations in the loaded object.
struct ObjectRelocations {
    /// `(start_vma, end_vma_inclusive)` per relocation, sorted by `start_vma`.
    ranges: Vec<(u64, u64)>,
}

impl ObjectRelocations {
    fn from_object(file: &object::File, image: &ObjectLoadImage) -> Self {
        use object::read::{Object, ObjectSection};
        use object::SectionIndex;

        let mut ranges = Vec::new();
        for section in image.reloc_sections() {
            let Ok(sec) = file.section_by_index(SectionIndex(section.index)) else { continue };
            for (offset, reloc) in sec.relocations() {
                let nbytes = (u64::from(reloc.size()) / 8).max(1);
                let Some(start) = section.vma.checked_add(offset) else { continue };
                let Some(end) = start.checked_add(nbytes - 1) else { continue };
                ranges.push((start, end));
            }
        }
        ranges.sort_unstable_by_key(|r| r.0);
        Self { ranges }
    }
}

impl RelocationQuery for ObjectRelocations {
    fn has_relocation_in(&self, min_addr: u64, max_addr: u64) -> bool {
        // Any stored range overlapping the inclusive query range [min, max].
        self.ranges.iter().any(|&(s, e)| s <= max_addr && min_addr <= e)
    }
}

