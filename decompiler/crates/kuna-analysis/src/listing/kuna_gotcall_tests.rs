use super::*;
use kuna_base::space::spacetype;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::loadimage::LoadImage;
use object::{ObjectSection, ObjectSymbol};

const IMAGE: &[u8] = include_bytes!("../../tests/fixtures/immutable_got_calls_x86_64");

fn symbol(bytes: &[u8], name: &str) -> u64 {
    object::File::parse(bytes)
        .unwrap()
        .symbols()
        .find(|s| s.name().ok() == Some(name))
        .unwrap()
        .address()
}

fn proof(bytes: &[u8]) -> Option<GotCalls> {
    let file = object::File::parse(bytes).unwrap();
    let image =
        crate::loadimage_object::ObjectLoadImage::from_bytes_silent("got-proof", bytes).ok()?;
    let loaded = image.shared_bytes().unwrap();
    let exec = super::super::xrefs::range_union(
        crate::entry::executable_sections(&file)
            .into_iter()
            .map(|(lo, hi, _)| (lo, hi))
            .collect(),
    );
    GotCalls::collect(&file, &exec, |slot| {
        let mut word = [0; 8];
        (loaded.mapped_covers(slot, slot + 8) && loaded.fill_span(&mut word, slot) == 0)
            .then(|| u64::from_le_bytes(word))
    })
}

fn word(bytes: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(bytes[at..at + 8].try_into().unwrap())
}
fn put(bytes: &mut [u8], at: usize, value: u64) {
    bytes[at..at + 8].copy_from_slice(&value.to_le_bytes());
}
fn headers(bytes: &[u8]) -> Vec<usize> {
    let base = word(bytes, 32) as usize;
    let size = u16::from_le_bytes(bytes[54..56].try_into().unwrap()) as usize;
    let count = u16::from_le_bytes(bytes[56..58].try_into().unwrap()) as usize;
    (0..count).map(|i| base + i * size).collect()
}
fn header_kind(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap())
}
fn relocation(bytes: &[u8], slot: u64) -> usize {
    let file = object::File::parse(bytes).unwrap();
    let section = file.section_by_name(".rela.dyn").unwrap();
    let (offset, size) = section.file_range().unwrap();
    (offset as usize..(offset + size) as usize)
        .step_by(24)
        .find(|&at| word(bytes, at) == slot)
        .unwrap()
}

#[test]
fn immutable_relative_pointer_uses_loaded_target_but_mutable_slot_does_not() {
    let slots = proof(IMAGE).unwrap();
    assert_eq!(
        slots.0.get(&symbol(IMAGE, "callback_slot")),
        Some(&symbol(IMAGE, "callback"))
    );
    assert!(!slots.0.contains_key(&symbol(IMAGE, "mutable_slot")));
    let file = object::File::parse(IMAGE).unwrap();
    assert!(GotCalls::collect(&file, &[(0, u64::MAX)], |_| Some(0)).is_none());
}

#[test]
fn unsupported_relocation_spans_and_symbol_bindings_are_never_relative_targets() {
    let slot = symbol(IMAGE, "callback_slot");
    let at = relocation(IMAGE, slot);
    for kind in [
        object::elf::R_X86_64_COPY,
        object::elf::R_X86_64_IRELATIVE,
        object::elf::R_X86_64_TLSDESC,
    ] {
        let mut bytes = IMAGE.to_vec();
        put(&mut bytes, at + 8, u64::from(kind));
        assert!(
            proof(&bytes).is_none(),
            "unknown write span or resolver: {kind}"
        );
    }
    for kind in [
        object::elf::R_X86_64_GLOB_DAT,
        object::elf::R_X86_64_JUMP_SLOT,
    ] {
        let mut bytes = IMAGE.to_vec();
        put(&mut bytes, at + 8, u64::from(kind));
        assert!(
            !proof(&bytes).is_some_and(|slots| slots.0.contains_key(&slot)),
            "bind {kind}"
        );
    }
}

#[test]
fn a_resolved_symbol_bind_is_still_preemptible() {
    let file = object::File::parse(IMAGE).unwrap();
    let slot = symbol(IMAGE, "callback_slot");
    let function = file
        .symbols()
        .find(|sym| sym.name().ok() == Some("callback"))
        .unwrap();
    let import = file
        .dynamic_symbols()
        .find(|sym| sym.is_undefined() && sym.index().0 != 0)
        .unwrap();
    let (table, _) = file
        .section_by_name(".dynsym")
        .unwrap()
        .file_range()
        .unwrap();
    let symbol_row = table as usize + import.index().0 * 24;
    let mut bytes = IMAGE.to_vec();
    bytes[symbol_row + 4] = (object::elf::STB_GLOBAL << 4) | object::elf::STT_FUNC;
    bytes[symbol_row + 5] = object::elf::STV_DEFAULT;
    let section = function.section_index().unwrap().0 as u16;
    bytes[symbol_row + 6..symbol_row + 8].copy_from_slice(&section.to_le_bytes());
    put(&mut bytes, symbol_row + 8, function.address());
    let fixup = relocation(IMAGE, slot);
    put(
        &mut bytes,
        fixup + 8,
        ((import.index().0 as u64) << 32) | u64::from(object::elf::R_X86_64_GLOB_DAT),
    );
    put(&mut bytes, fixup + 16, 0);

    let file = object::File::parse(bytes.as_slice()).unwrap();
    let sites = fixup_sites(&file, &bytes);
    let proven = sites.sites.iter().find(|row| row.vma == slot).unwrap();
    assert_eq!(proven.kind, FixupKind::Bind);
    assert_eq!(proven.target, Some(function.address()));
    assert!(!proof(&bytes).is_some_and(|slots| slots.0.contains_key(&slot)));
}

#[test]
fn a_rebase_needs_an_exact_ordinary_function_symbol() {
    let file = object::File::parse(IMAGE).unwrap();
    let slot = symbol(IMAGE, "callback_slot");
    let function = file
        .symbols()
        .find(|sym| sym.name().ok() == Some("callback"))
        .unwrap();
    let (table, _) = file
        .section_by_name(".symtab")
        .unwrap()
        .file_range()
        .unwrap();
    let info = table as usize + function.index().0 * 24 + 4;
    for kind in [
        object::elf::STT_NOTYPE,
        object::elf::STT_OBJECT,
        object::elf::STT_GNU_IFUNC,
    ] {
        let mut bytes = IMAGE.to_vec();
        bytes[info] = (bytes[info] & 0xf0) | kind;
        assert!(
            !proof(&bytes).is_some_and(|slots| slots.0.contains_key(&slot)),
            "type {kind}"
        );
    }
    let mut wrong_section = IMAGE.to_vec();
    let data_index = file.section_by_name(".data").unwrap().index().0 as u16;
    wrong_section[info + 2..info + 4].copy_from_slice(&data_index.to_le_bytes());
    assert!(!proof(&wrong_section).is_some_and(|slots| slots.0.contains_key(&slot)));

    let mut interior = IMAGE.to_vec();
    put(
        &mut interior,
        relocation(IMAGE, slot) + 16,
        function.address() + 1,
    );
    assert!(!proof(&interior).is_some_and(|slots| slots.0.contains_key(&slot)));
}

#[test]
fn slot_requires_full_protection_in_one_file_backed_mapping() {
    let slot = symbol(IMAGE, "callback_slot");
    let mutable = symbol(IMAGE, "mutable_slot");
    let ph = headers(IMAGE);
    let relro = *ph
        .iter()
        .find(|&&at| header_kind(IMAGE, at) == object::elf::PT_GNU_RELRO)
        .unwrap();
    let writable = *ph
        .iter()
        .find(|&&at| {
            header_kind(IMAGE, at) == object::elf::PT_LOAD
                && word(IMAGE, at + 16) <= slot
                && slot < word(IMAGE, at + 16) + word(IMAGE, at + 40)
        })
        .unwrap();
    let mut no_relro = IMAGE.to_vec();
    no_relro[relro..relro + 4].fill(0);
    assert!(proof(&no_relro).is_none());
    let mut short_relro = IMAGE.to_vec();
    put(
        &mut short_relro,
        relro + 40,
        slot + 7 - word(IMAGE, relro + 16),
    );
    assert!(proof(&short_relro).is_none());
    let mut tail = IMAGE.to_vec();
    put(&mut tail, relro + 40, mutable + 8 - word(IMAGE, relro + 16));
    assert!(
        !proof(&tail).unwrap().0.contains_key(&mutable),
        "trailing partial page is writable"
    );
    let mut readonly = IMAGE.to_vec();
    readonly[writable + 4..writable + 8].copy_from_slice(&object::elf::PF_R.to_le_bytes());
    assert!(!proof(&readonly).unwrap().0.contains_key(&mutable));
    let other = *ph
        .iter()
        .find(|&&at| header_kind(IMAGE, at) == object::elf::PT_NOTE)
        .unwrap();
    let mut overlap = IMAGE.to_vec();
    overlap[other..other + 56].copy_from_slice(&IMAGE[writable..writable + 56]);
    assert!(proof(&overlap).is_none());
    let target = symbol(IMAGE, "callback");
    let code = *ph
        .iter()
        .find(|&&at| {
            header_kind(IMAGE, at) == object::elf::PT_LOAD
                && word(IMAGE, at + 16) <= target
                && target < word(IMAGE, at + 16) + word(IMAGE, at + 40)
        })
        .unwrap();
    let mut overlapping_code = IMAGE.to_vec();
    overlapping_code[other..other + 56].copy_from_slice(&IMAGE[code..code + 56]);
    assert!(proof(&overlapping_code).is_none());
    let mut nonexec = IMAGE.to_vec();
    nonexec[code + 4..code + 8].copy_from_slice(&object::elf::PF_R.to_le_bytes());
    assert!(proof(&nonexec).is_none());
    let mut truncated = IMAGE.to_vec();
    put(
        &mut truncated,
        writable + 32,
        slot - word(IMAGE, writable + 16) + 4,
    );
    assert!(!proof(&truncated).is_some_and(|slots| slots.0.contains_key(&slot)));
}

#[test]
fn overlapping_fixups_nonaligned_slots_and_noncode_targets_decline() {
    let slot = symbol(IMAGE, "callback_slot");
    let at = relocation(IMAGE, slot);
    let mut overlap = IMAGE.to_vec();
    let other = relocation(IMAGE, symbol(IMAGE, "mutable_slot"));
    put(&mut overlap, other, slot + 4);
    assert!(!proof(&overlap).is_some_and(|slots| slots.0.contains_key(&slot)));
    let mut unaligned = IMAGE.to_vec();
    put(&mut unaligned, at, slot + 1);
    assert!(!proof(&unaligned).is_some_and(|slots| slots.0.contains_key(&(slot + 1))));
    let mut data = IMAGE.to_vec();
    put(&mut data, at + 16, symbol(IMAGE, "mutable_slot"));
    assert!(!proof(&data).is_some_and(|slots| slots.0.contains_key(&slot)));
    let file = object::File::parse(IMAGE).unwrap();
    let section = file.section_by_name(".rela.dyn").unwrap();
    let shoff = word(IMAGE, 40) as usize + section.index().0 * 64;
    let mut damaged = IMAGE.to_vec();
    put(&mut damaged, shoff + 24, IMAGE.len() as u64 + 4096);
    assert!(proof(&damaged).is_none());
}

#[test]
fn call_target_requires_latest_whole_copy_in_the_same_instruction() {
    let ram = Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_PROCESSOR));
    let tmp = Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_INTERNAL));
    let vn = |space: &Rc<AddrSpace>, offset, size| VarnodeData {
        space: Some(Rc::clone(space)),
        offset,
        size,
    };
    let slot = vn(&ram, 0x3000, 8);
    let value = vn(&tmp, 0, 8);
    let copy = FullOp {
        opcode: OpCode::CPUI_COPY,
        out: Some(value.clone()),
        ins: vec![slot.clone()],
    };
    let call = FullOp {
        opcode: OpCode::CPUI_CALLIND,
        out: None,
        ins: vec![value.clone()],
    };
    let proof = GotCalls(BTreeMap::from([(0x3000, 0x1000)]));
    assert_eq!(
        proof.target(&[copy.clone(), call.clone()], &ram),
        Some((0x3000, 0x1000))
    );
    let earlier = FullOp {
        ins: vec![vn(&ram, 0x4000, 8)],
        ..copy.clone()
    };
    assert_eq!(
        proof.target(&[earlier, copy.clone(), call.clone()], &ram),
        Some((0x3000, 0x1000))
    );
    let unknown = FullOp {
        ins: vec![vn(&ram, 0x4000, 8)],
        ..copy.clone()
    };
    assert_eq!(proof.target(&[unknown, call.clone()], &ram), None);
    assert_eq!(
        proof.target(
            &[FullOp {
                ins: vec![slot],
                ..call.clone()
            }],
            &ram
        ),
        Some((0x3000, 0x1000))
    );
    assert_eq!(
        proof.target(&[call.clone()], &ram),
        None,
        "register carried from another instruction"
    );
    let narrow = FullOp {
        ins: vec![vn(&ram, 0x3000, 4)],
        ..copy.clone()
    };
    assert_eq!(proof.target(&[narrow, call.clone()], &ram), None);
    for offset in [0, 4] {
        let partial = FullOp {
            opcode: OpCode::CPUI_COPY,
            out: Some(vn(&tmp, offset, 4)),
            ins: vec![vn(&ram, 0x4000, 4)],
        };
        assert_eq!(
            proof.target(&[copy.clone(), partial, call.clone()], &ram),
            None
        );
    }
    let changed = FullOp {
        opcode: OpCode::CPUI_INT_ADD,
        ..copy.clone()
    };
    assert_eq!(
        proof.target(&[copy.clone(), changed, call.clone()], &ram),
        None
    );
    let branch = FullOp {
        opcode: OpCode::CPUI_CBRANCH,
        out: None,
        ins: vec![],
    };
    assert_eq!(
        proof.target(&[copy.clone(), branch.clone(), call.clone()], &ram),
        None
    );
    assert_eq!(
        proof.target(&[copy.clone(), call.clone(), branch], &ram),
        None
    );
    for opcode in [
        OpCode::CPUI_CALL,
        OpCode::CPUI_CALLOTHER,
        OpCode::CPUI_RETURN,
    ] {
        let other_flow = FullOp {
            opcode,
            out: None,
            ins: vec![],
        };
        assert_eq!(
            proof.target(&[copy.clone(), call.clone(), other_flow], &ram),
            None
        );
    }
    assert_eq!(proof.target(&[copy, call.clone(), call], &ram), None);
}
