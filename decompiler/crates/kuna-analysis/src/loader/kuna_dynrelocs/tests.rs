//! Synthetic-ELF coverage for [`super::resolve`]: one hand-built image per
//! handled architecture, so the `RELATIVE` / `GLOB_DAT` / `JUMP_SLOT` triple, the
//! defined-vs-undefined symbol split, the `REL` implicit addend and the
//! `PT_GNU_RELRO` intersection are all proven without a checked-in binary.
//!
//! The env GATE itself is NOT tested here: `KUNA_DYNRELOCS` is process-global and
//! cargo runs these in parallel threads, so flipping it mid-suite perturbs every
//! other test in this binary. Its two arms live in their own integration target,
//! `tests/verify_dynrelocs_gate.rs`, exactly as `relocrebase`'s do.

use super::*;

const SHT_PROGBITS: u32 = 1;
const SHT_STRTAB: u32 = 3;
const SHT_RELA: u32 = 4;
const SHT_REL: u32 = 9;
const SHT_DYNSYM: u32 = 11;
const PT_LOAD: u32 = 1;

const TEXT_VMA: u64 = 0x1000;
const GOT_VMA: u64 = 0x3000;
/// 1-based section index of `.text` in the tables below (NULL, dynsym, dynstr,
/// reloc table, .text, .got, .shstrtab).
const TEXT_SHNDX: u16 = 4;

/// One section of a synthetic image.
struct Sec {
    ty: u32,
    flags: u64,
    addr: u64,
    data: Vec<u8>,
    link: u32,
    info: u32,
    entsize: u64,
}

/// Build a minimal but genuinely parseable ELF: one `PT_LOAD` mapping the whole
/// file at vaddr 0 (so vma == file offset), one `PT_GNU_RELRO` over the `.got`,
/// and the `.dynsym`/`.dynstr`/reloc-table/`.text`/`.got`/`.shstrtab` sections
/// `object`'s dynamic-relocation walk needs.
fn build_elf(is64: bool, machine: u16, secs: &[Sec], got_len: u64) -> Vec<u8> {
    let ehsize = if is64 { 64usize } else { 52 };
    let phentsize = if is64 { 56usize } else { 32 };
    let shentsize = if is64 { 64usize } else { 40 };
    let phnum = 2usize;
    let shnum = secs.len() + 1; // + the leading NULL section

    let mut out = vec![0u8; ehsize + phnum * phentsize];
    let mut offsets: Vec<usize> = Vec::new();
    for s in secs {
        while out.len() % 8 != 0 {
            out.push(0);
        }
        if s.addr != 0 {
            // Pad so the payload lands at file offset == vma (identity mapping).
            while (out.len() as u64) < s.addr {
                out.push(0);
            }
            assert_eq!(out.len() as u64, s.addr, "section vma must be reachable by padding");
        }
        offsets.push(out.len());
        out.extend_from_slice(&s.data);
    }
    while out.len() % 8 != 0 {
        out.push(0);
    }
    let shoff = out.len();
    out.extend(std::iter::repeat(0).take(shnum * shentsize));
    let filesz = out.len() as u64;

    fn put(o: &mut [u8], at: usize, v: u64, w: usize) {
        o[at..at + w].copy_from_slice(&v.to_le_bytes()[..w]);
    }

    out[..4].copy_from_slice(b"\x7fELF");
    out[4] = if is64 { 2 } else { 1 };
    out[5] = 1; // little-endian
    out[6] = 1; // EV_CURRENT
    put(&mut out, 16, 3, 2); // ET_DYN
    put(&mut out, 18, machine as u64, 2);
    put(&mut out, 20, 1, 4);
    if is64 {
        put(&mut out, 32, ehsize as u64, 8);
        put(&mut out, 40, shoff as u64, 8);
        put(&mut out, 52, ehsize as u64, 2);
        put(&mut out, 54, phentsize as u64, 2);
        put(&mut out, 56, phnum as u64, 2);
        put(&mut out, 58, shentsize as u64, 2);
        put(&mut out, 60, shnum as u64, 2);
        put(&mut out, 62, (shnum - 1) as u64, 2); // e_shstrndx = last section
    } else {
        put(&mut out, 28, ehsize as u64, 4);
        put(&mut out, 32, shoff as u64, 4);
        put(&mut out, 40, ehsize as u64, 2);
        put(&mut out, 42, phentsize as u64, 2);
        put(&mut out, 44, phnum as u64, 2);
        put(&mut out, 46, shentsize as u64, 2);
        put(&mut out, 48, shnum as u64, 2);
        put(&mut out, 50, (shnum - 1) as u64, 2);
    }

    // PT_LOAD [0, filesz) at vaddr 0, then PT_GNU_RELRO over the .got.
    let ph = ehsize;
    let ph2 = ph + phentsize;
    if is64 {
        put(&mut out, ph, PT_LOAD as u64, 4);
        put(&mut out, ph + 4, 5, 4); // PF_R | PF_X
        put(&mut out, ph + 32, filesz, 8);
        put(&mut out, ph + 40, filesz, 8);
        put(&mut out, ph2, PT_GNU_RELRO as u64, 4);
        put(&mut out, ph2 + 4, 4, 4); // PF_R
        put(&mut out, ph2 + 8, GOT_VMA, 8);
        put(&mut out, ph2 + 16, GOT_VMA, 8);
        put(&mut out, ph2 + 32, got_len, 8);
        put(&mut out, ph2 + 40, got_len, 8);
    } else {
        put(&mut out, ph, PT_LOAD as u64, 4);
        put(&mut out, ph + 16, filesz, 4);
        put(&mut out, ph + 20, filesz, 4);
        put(&mut out, ph + 24, 5, 4);
        put(&mut out, ph2, PT_GNU_RELRO as u64, 4);
        put(&mut out, ph2 + 4, GOT_VMA, 4);
        put(&mut out, ph2 + 8, GOT_VMA, 4);
        put(&mut out, ph2 + 16, got_len, 4);
        put(&mut out, ph2 + 20, got_len, 4);
        put(&mut out, ph2 + 24, 4, 4);
    }

    // Section headers (index 0 is the all-zero NULL entry).
    for (i, s) in secs.iter().enumerate() {
        let sh = shoff + (i + 1) * shentsize;
        if is64 {
            put(&mut out, sh + 4, s.ty as u64, 4);
            put(&mut out, sh + 8, s.flags, 8);
            put(&mut out, sh + 16, s.addr, 8);
            put(&mut out, sh + 24, offsets[i] as u64, 8);
            put(&mut out, sh + 32, s.data.len() as u64, 8);
            put(&mut out, sh + 40, s.link as u64, 4);
            put(&mut out, sh + 44, s.info as u64, 4);
            put(&mut out, sh + 48, 8, 8);
            put(&mut out, sh + 56, s.entsize, 8);
        } else {
            put(&mut out, sh + 4, s.ty as u64, 4);
            put(&mut out, sh + 8, s.flags, 4);
            put(&mut out, sh + 12, s.addr, 4);
            put(&mut out, sh + 16, offsets[i] as u64, 4);
            put(&mut out, sh + 20, s.data.len() as u64, 4);
            put(&mut out, sh + 24, s.link as u64, 4);
            put(&mut out, sh + 28, s.info as u64, 4);
            put(&mut out, sh + 32, 4, 4);
            put(&mut out, sh + 36, s.entsize, 4);
        }
    }
    out
}

/// `.dynsym`: index 0 NULL, 1 = defined `func` at [`TEXT_VMA`], 2 = undefined
/// `imp` (the import that must be skipped).
fn dynsym(is64: bool) -> Vec<u8> {
    let mut v = Vec::new();
    if is64 {
        v.extend_from_slice(&[0u8; 24]);
        let mut sym = |name: u32, value: u64, shndx: u16| {
            v.extend_from_slice(&name.to_le_bytes());
            v.push(0x12); // GLOBAL | FUNC
            v.push(0);
            v.extend_from_slice(&shndx.to_le_bytes());
            v.extend_from_slice(&value.to_le_bytes());
            v.extend_from_slice(&0u64.to_le_bytes());
        };
        sym(1, TEXT_VMA, TEXT_SHNDX);
        sym(6, 0, 0);
    } else {
        v.extend_from_slice(&[0u8; 16]);
        let mut sym = |name: u32, value: u32, shndx: u16| {
            v.extend_from_slice(&name.to_le_bytes());
            v.extend_from_slice(&value.to_le_bytes());
            v.extend_from_slice(&0u32.to_le_bytes());
            v.push(0x12);
            v.push(0);
            v.extend_from_slice(&shndx.to_le_bytes());
        };
        sym(1, TEXT_VMA as u32, TEXT_SHNDX);
        sym(6, 0, 0);
    }
    v
}

/// `\0func\0imp\0` — the string table the `.dynsym` names index into.
fn dynstr() -> Vec<u8> {
    b"\0func\0imp\0".to_vec()
}

fn rela64(offset: u64, sym: u32, ty: u32, addend: u64) -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(&offset.to_le_bytes());
    v.extend_from_slice(&(((sym as u64) << 32) | ty as u64).to_le_bytes());
    v.extend_from_slice(&addend.to_le_bytes());
    v
}

fn rel32(offset: u32, sym: u32, ty: u32) -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(&offset.to_le_bytes());
    v.extend_from_slice(&((sym << 8) | ty).to_le_bytes());
    v
}

/// Assemble a 64-bit RELA image for `machine` with the three-relocation set:
/// RELATIVE (addend), GLOB_DAT on the defined symbol, JUMP_SLOT on the import.
fn image64(machine: u16, r_rel: u32, r_glob: u32, r_jmp: u32) -> Vec<u8> {
    let mut relocs = rela64(GOT_VMA, 0, r_rel, TEXT_VMA);
    relocs.extend(rela64(GOT_VMA + 8, 1, r_glob, 0));
    relocs.extend(rela64(GOT_VMA + 16, 2, r_jmp, 0));
    let secs = vec![
        Sec { ty: SHT_DYNSYM, flags: 2, addr: 0, data: dynsym(true), link: 2, info: 1, entsize: 24 },
        Sec { ty: SHT_STRTAB, flags: 2, addr: 0, data: dynstr(), link: 0, info: 0, entsize: 0 },
        Sec { ty: SHT_RELA, flags: 2, addr: 0, data: relocs, link: 1, info: 0, entsize: 24 },
        Sec { ty: SHT_PROGBITS, flags: 6, addr: TEXT_VMA, data: vec![0x90; 16], link: 0, info: 0, entsize: 0 },
        Sec { ty: SHT_PROGBITS, flags: 3, addr: GOT_VMA, data: vec![0u8; 24], link: 0, info: 0, entsize: 0 },
        Sec { ty: SHT_STRTAB, flags: 0, addr: 0, data: b"\0".to_vec(), link: 0, info: 0, entsize: 0 },
    ];
    build_elf(true, machine, &secs, 24)
}

fn run(bytes: &[u8]) -> DynRelocs {
    let file = object::File::parse(bytes).expect("synthetic ELF must parse");
    resolve(&file, bytes)
}

/// x86-64: `RELATIVE` takes the addend, `GLOB_DAT` on a defined symbol takes the
/// symbol address, `JUMP_SLOT` on an *undefined* import is skipped, and both
/// written slots land in the RELRO const set.
#[test]
fn x86_64_relative_and_globdat_apply_import_is_skipped() {
    let bytes = image64(62, 8, 6, 7);
    let r = run(&bytes);
    assert_eq!(
        r.writes,
        vec![
            DynRelocWrite { vma: GOT_VMA, value: TEXT_VMA, width: 8 },
            DynRelocWrite { vma: GOT_VMA + 8, value: TEXT_VMA, width: 8 },
        ],
        "the undefined JUMP_SLOT import must not be written"
    );
    assert_eq!(r.const_ranges, vec![(GOT_VMA, GOT_VMA + 7), (GOT_VMA + 8, GOT_VMA + 15)]);
}

/// AArch64 uses a different `r_type` numbering for the same triple.
#[test]
fn aarch64_uses_its_own_reloc_numbering() {
    let bytes = image64(183, 1027, 1025, 1026);
    let r = run(&bytes);
    assert_eq!(
        r.writes,
        vec![
            DynRelocWrite { vma: GOT_VMA, value: TEXT_VMA, width: 8 },
            DynRelocWrite { vma: GOT_VMA + 8, value: TEXT_VMA, width: 8 },
        ]
    );
}

/// i386: a `REL` table carries no addend, so `RELATIVE` must read it back out of
/// the slot being patched, and every slot is 4 bytes wide.
#[test]
fn i386_rel_table_reads_the_implicit_addend_from_the_slot() {
    let mut relocs = rel32(GOT_VMA as u32, 0, 8); // R_386_RELATIVE
    relocs.extend(rel32(GOT_VMA as u32 + 4, 1, 6)); // R_386_GLOB_DAT (defined)
    relocs.extend(rel32(GOT_VMA as u32 + 8, 2, 7)); // R_386_JMP_SLOT (import)
    let mut got = vec![0u8; 12];
    got[..4].copy_from_slice(&(TEXT_VMA as u32).to_le_bytes()); // the implicit addend
    let secs = vec![
        Sec { ty: SHT_DYNSYM, flags: 2, addr: 0, data: dynsym(false), link: 2, info: 1, entsize: 16 },
        Sec { ty: SHT_STRTAB, flags: 2, addr: 0, data: dynstr(), link: 0, info: 0, entsize: 0 },
        Sec { ty: SHT_REL, flags: 2, addr: 0, data: relocs, link: 1, info: 0, entsize: 8 },
        Sec { ty: SHT_PROGBITS, flags: 6, addr: TEXT_VMA, data: vec![0x90; 16], link: 0, info: 0, entsize: 0 },
        Sec { ty: SHT_PROGBITS, flags: 3, addr: GOT_VMA, data: got, link: 0, info: 0, entsize: 0 },
        Sec { ty: SHT_STRTAB, flags: 0, addr: 0, data: b"\0".to_vec(), link: 0, info: 0, entsize: 0 },
    ];
    let bytes = build_elf(false, 3, &secs, 12);
    let r = run(&bytes);
    assert_eq!(
        r.writes,
        vec![
            DynRelocWrite { vma: GOT_VMA, value: TEXT_VMA, width: 4 },
            DynRelocWrite { vma: GOT_VMA + 4, value: TEXT_VMA, width: 4 },
        ]
    );
    assert_eq!(r.const_ranges, vec![(GOT_VMA, GOT_VMA + 3), (GOT_VMA + 4, GOT_VMA + 7)]);
}

/// An architecture kuna has no triple for produces nothing rather than guessing.
#[test]
fn unhandled_machine_is_inert() {
    // EM_PPC64 (21) — parsed fine, but no RELATIVE/GLOB_DAT/JUMP_SLOT triple.
    let bytes = image64(21, 8, 6, 7);
    assert!(run(&bytes).writes.is_empty());
}
