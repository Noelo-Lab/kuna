use super::*;

fn fixture(name: &str) -> Vec<u8> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name);
    std::fs::read(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn exec_ranges(file: &object::File) -> Vec<(u64, u64)> {
    file.sections()
        .filter(|s| s.kind() == SectionKind::Text)
        .map(|s| (s.address(), s.address() + s.size()))
        .collect()
}

#[test]
fn source_paths_are_told_from_format_strings() {
    assert!(looks_like_source_path("drivers/virt/irqbypass.c"));
    assert!(looks_like_source_path("kwarn_bugsite.c"));
    assert!(!looks_like_source_path(""));
    assert!(!looks_like_source_path("[cond]"));
}

/// The 7.x x86-64 module layout: `bug_addr`, `format`, `file`, `line`, `flags`
/// in 16 bytes, recovered from `.rela__bug_table` on the rebased view.
#[test]
fn x86_64_module_table_names_a_warn_and_a_bug() {
    let bytes = fixture("kernel_bugtable_x86_64.ko");
    let raw = object::File::parse(&*bytes).expect("parse");
    let view = crate::loader::kuna_relocrebase::rebased_view(&raw, &bytes).expect("a module is laid out");
    let file = object::File::parse(&*view.bytes).expect("rebased view parses");
    let sites = scan(&file, exec_ranges(&file));
    let text = file.section_by_name(".text").expect(".text").address();
    assert_eq!(sites.len(), 2);
    assert_eq!(sites[0].addr, text + 0x35);
    assert!(sites[0].warning);
    assert_eq!(sites[0].line, Some(9));
    assert_eq!(sites[1].addr, text + 0x4a);
    assert!(!sites[1].warning);
    assert_eq!(sites[1].line, Some(11));
    let image = Image::new(&file, exec_ranges(&file));
    for s in &sites {
        assert_eq!(image.string_at(s.file.expect("verbose table")).as_deref(), Some("kwarn_bugsite.c"));
    }
}


/// A minimal ELF64 x86-64 executable: the named `PROGBITS` sections at their
/// addresses, no segments. Enough for the `object` crate to answer section
/// queries, which is all the relocation-free probe reads.
fn elf64(sections: &[(&str, u64, u64, &[u8])]) -> Vec<u8> {
    let mut shstr = vec![0u8];
    let mut names = Vec::new();
    for (name, _, _, _) in sections {
        names.push(shstr.len() as u32);
        shstr.extend_from_slice(name.as_bytes());
        shstr.push(0);
    }
    let shstrtab_name = shstr.len() as u32;
    shstr.extend_from_slice(b".shstrtab\0");
    let mut body: Vec<u8> = Vec::new();
    let mut offsets = Vec::new();
    for (_, _, _, data) in sections {
        offsets.push(64 + body.len() as u64);
        body.extend_from_slice(data);
    }
    let shstr_off = 64 + body.len() as u64;
    body.extend_from_slice(&shstr);
    while (64 + body.len()) % 8 != 0 {
        body.push(0);
    }
    let shoff = 64 + body.len() as u64;
    let shnum = sections.len() as u16 + 2;
    let mut out = vec![0x7f, b'E', b'L', b'F', 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    out.extend_from_slice(&2u16.to_le_bytes());
    out.extend_from_slice(&62u16.to_le_bytes());
    out.extend_from_slice(&1u32.to_le_bytes());
    out.extend_from_slice(&0u64.to_le_bytes());
    out.extend_from_slice(&0u64.to_le_bytes());
    out.extend_from_slice(&shoff.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&64u16.to_le_bytes());
    out.extend_from_slice(&56u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&64u16.to_le_bytes());
    out.extend_from_slice(&shnum.to_le_bytes());
    out.extend_from_slice(&(shnum - 1).to_le_bytes());
    out.extend_from_slice(&body);
    let shdr = |out: &mut Vec<u8>, name: u32, flags: u64, addr: u64, off: u64, size: u64| {
        out.extend_from_slice(&name.to_le_bytes());
        out.extend_from_slice(&1u32.to_le_bytes());
        out.extend_from_slice(&flags.to_le_bytes());
        out.extend_from_slice(&addr.to_le_bytes());
        out.extend_from_slice(&off.to_le_bytes());
        out.extend_from_slice(&size.to_le_bytes());
        out.extend_from_slice(&[0u8; 8]);
        out.extend_from_slice(&1u64.to_le_bytes());
        out.extend_from_slice(&0u64.to_le_bytes());
    };
    out.extend_from_slice(&[0u8; 64]);
    for (i, (_, addr, flags, data)) in sections.iter().enumerate() {
        shdr(&mut out, names[i], *flags, *addr, offsets[i], data.len() as u64);
    }
    shdr(&mut out, shstrtab_name, 0, 0, shstr_off, shstr.len() as u64);
    out
}

const SHF_ALLOC_EXEC: u64 = 0x2 | 0x4;
const SHF_ALLOC: u64 = 0x2;
const SHF_ALLOC_WRITE: u64 = 0x2 | 0x1;

fn rel32(from: u64, to: u64) -> [u8; 4] {
    ((to as i64 - from as i64) as i32).to_le_bytes()
}

/// A linked image with the verbose 12-byte layout (`bug_addr`, `file`, `line`,
/// `flags`): the probe settles on 12 and reads both entries.
#[test]
fn linked_image_probe_recovers_the_verbose_layout() {
    let text = [0x90, 0x0f, 0x0b, 0x90, 0x90, 0x0f, 0x0b, 0x90];
    let rodata = b"lib/a.c\0";
    let (text_vma, rodata_vma, table_vma) = (0x1000u64, 0x2000u64, 0x3000u64);
    let mut table = Vec::new();
    for (i, (trap, line, flags)) in [(1u64, 42u16, 0x901u16), (5, 77, 0)].iter().enumerate() {
        let e = table_vma + 12 * i as u64;
        table.extend_from_slice(&rel32(e, text_vma + trap));
        table.extend_from_slice(&rel32(e + 4, rodata_vma));
        table.extend_from_slice(&line.to_le_bytes());
        table.extend_from_slice(&flags.to_le_bytes());
    }
    let bytes = elf64(&[
        (".text", text_vma, SHF_ALLOC_EXEC, &text),
        (".rodata", rodata_vma, SHF_ALLOC, rodata),
        (BUG_TABLE_SECTION, table_vma, SHF_ALLOC_WRITE, &table),
    ]);
    let file = object::File::parse(&*bytes).expect("parse");
    let sites = scan(&file, exec_ranges(&file));
    assert_eq!(
        sites,
        vec![
            KernelBugSite { addr: 0x1001, warning: true, file: Some(0x2000), line: Some(42) },
            KernelBugSite { addr: 0x1005, warning: false, file: Some(0x2000), line: Some(77) },
        ]
    );
}

/// Before Linux 5.19 a self-relative `file` counts from the entry's start: read
/// the 5.19 way it lands four bytes into the path, so the table is read the old
/// way and `file` names the path's first byte.
#[test]
fn pre_5_19_file_counts_from_the_entry() {
    let text = [0x90, 0x0f, 0x0b, 0x90, 0x90, 0x0f, 0x0b, 0x90];
    let rodata = b"x\0include/linux/mm.h\0fs/nfs/write.c\0";
    let (text_vma, rodata_vma, table_vma) = (0x1000u64, 0x2000u64, 0x3000u64);
    let mut table = Vec::new();
    for (i, (trap, path, line, flags)) in [(1u64, 2u64, 607u16, 0x903u16), (5, 21, 77, 0)].iter().enumerate() {
        let e = table_vma + 12 * i as u64;
        table.extend_from_slice(&rel32(e, text_vma + trap));
        table.extend_from_slice(&rel32(e, rodata_vma + path));
        table.extend_from_slice(&line.to_le_bytes());
        table.extend_from_slice(&flags.to_le_bytes());
    }
    let bytes = elf64(&[
        (".text", text_vma, SHF_ALLOC_EXEC, &text),
        (".rodata", rodata_vma, SHF_ALLOC, rodata),
        (BUG_TABLE_SECTION, table_vma, SHF_ALLOC_WRITE, &table),
    ]);
    let file = object::File::parse(&*bytes).expect("parse");
    let sites = scan(&file, exec_ranges(&file));
    assert_eq!(
        sites,
        vec![
            KernelBugSite { addr: 0x1001, warning: true, file: Some(0x2002), line: Some(607) },
            KernelBugSite { addr: 0x1005, warning: false, file: Some(0x2015), line: Some(77) },
        ]
    );
}

/// A linked image keeps its entries in link order, and the first string of a
/// merged string blob can follow binary data with no NUL before it. Neither
/// reading puts the first 64 entries' `file` on a string start here, so the
/// vote samples the whole table: the later entries settle it.
#[test]
fn pre_5_19_vote_spans_the_table() {
    let text = [0x0f, 0x0b].repeat(128);
    let rodata = b"\x07\x01include/linux/a.h\0lib/b.c\0";
    let (text_vma, rodata_vma, table_vma) = (0x1000u64, 0x2000u64, 0x3000u64);
    let path = |i: u64| rodata_vma + if i < 64 { 2 } else { 20 };
    let mut table = Vec::new();
    for i in 0..128u64 {
        let e = table_vma + 12 * i;
        table.extend_from_slice(&rel32(e, text_vma + 2 * i));
        table.extend_from_slice(&rel32(e, path(i)));
        table.extend_from_slice(&(i as u16 + 1).to_le_bytes());
        table.extend_from_slice(&1u16.to_le_bytes());
    }
    let bytes = elf64(&[
        (".text", text_vma, SHF_ALLOC_EXEC, &text),
        (".rodata", rodata_vma, SHF_ALLOC, rodata),
        (BUG_TABLE_SECTION, table_vma, SHF_ALLOC_WRITE, &table),
    ]);
    let file = object::File::parse(&*bytes).expect("parse");
    let sites = scan(&file, exec_ranges(&file));
    assert_eq!(sites.len(), 128);
    for (i, s) in sites.iter().enumerate() {
        assert_eq!(s.file, Some(path(i as u64)), "entry {i}");
    }
}

/// Without `CONFIG_DEBUG_BUGVERBOSE` an entry is `bug_addr` and `flags` in 8
/// bytes: no file, no line.
#[test]
fn linked_image_probe_recovers_the_bare_layout() {
    let text = [0x0f, 0x0b, 0x90, 0x90, 0x0f, 0x0b];
    let (text_vma, table_vma) = (0x1000u64, 0x3000u64);
    let mut table = Vec::new();
    for (i, (trap, flags)) in [(0u64, 1u16), (4, 0)].iter().enumerate() {
        let e = table_vma + 8 * i as u64;
        table.extend_from_slice(&rel32(e, text_vma + trap));
        table.extend_from_slice(&flags.to_le_bytes());
        table.extend_from_slice(&[0, 0]);
    }
    let bytes = elf64(&[(".text", text_vma, SHF_ALLOC_EXEC, &text), (BUG_TABLE_SECTION, table_vma, SHF_ALLOC_WRITE, &table)]);
    let file = object::File::parse(&*bytes).expect("parse");
    let sites = scan(&file, exec_ranges(&file));
    assert_eq!(
        sites,
        vec![
            KernelBugSite { addr: 0x1000, warning: true, file: None, line: None },
            KernelBugSite { addr: 0x1004, warning: false, file: None, line: None },
        ]
    );
}

/// A 12-byte entry whose second word is no string pointer leaves room the
/// kernel's layouts do not have, so `flags` cannot be placed: nothing is read
/// rather than a displacement's low half standing in for the flags.
#[test]
fn an_entry_with_unexplained_room_is_declined() {
    let text = [0x90, 0x0f, 0x0b, 0x90, 0x90, 0x0f, 0x0b, 0x90];
    let (text_vma, table_vma) = (0x1000u64, 0x3000u64);
    let mut table = Vec::new();
    for (i, trap) in [1u64, 5].iter().enumerate() {
        let e = table_vma + 12 * i as u64;
        table.extend_from_slice(&rel32(e, text_vma + trap));
        table.extend_from_slice(&rel32(e + 4, text_vma));
        table.extend_from_slice(&[0x01, 0x09, 0x01, 0x09]);
    }
    let bytes = elf64(&[(".text", text_vma, SHF_ALLOC_EXEC, &text), (BUG_TABLE_SECTION, table_vma, SHF_ALLOC_WRITE, &table)]);
    let file = object::File::parse(&*bytes).expect("parse");
    assert!(scan(&file, exec_ranges(&file)).is_empty());
}

/// A table whose entries point nowhere executable under any size yields nothing.
#[test]
fn a_table_no_size_explains_is_declined() {
    let text = [0x0f, 0x0b];
    let table = [0xff; 24];
    let bytes = elf64(&[(".text", 0x1000, SHF_ALLOC_EXEC, &text), (BUG_TABLE_SECTION, 0x3000, SHF_ALLOC_WRITE, &table)]);
    let file = object::File::parse(&*bytes).expect("parse");
    assert!(scan(&file, exec_ranges(&file)).is_empty());
}
