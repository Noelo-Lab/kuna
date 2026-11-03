use crate::common;
use object::write::{Object, Symbol, SymbolSection};
use object::{Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope};
use std::process::Command;

fn get32(b: &[u8], p: usize) -> u32 { u32::from_be_bytes(b[p..p+4].try_into().unwrap()) }
fn get64(b: &[u8], p: usize) -> u64 { u64::from_be_bytes(b[p..p+8].try_into().unwrap()) }
fn put64(b: &mut [u8], p: usize, n: u64) { b[p..p+8].copy_from_slice(&n.to_be_bytes()); }

/// Synthetic linked ELFv1: two descriptors, two TOCs, and a code alias.
fn image(abi: u32) -> Vec<u8> {
    image_with_code(abi, &[0x88,0x62,0,0,0x4e,0x80,0,0x20])
}

fn image_with_code(abi: u32, first: &[u8]) -> Vec<u8> {
    image_with_relocations(abi, first, &[])
}

fn image_with_relocations(abi: u32, first: &[u8], relocations: &[(u64, u32, i64)]) -> Vec<u8> {
    let mut o = Object::new(BinaryFormat::Elf, Architecture::PowerPc64, Endianness::Big);
    let text = o.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    let mut code = first.to_vec();
    code.extend([0x88,0x62,0,0,0x4e,0x80,0,0x20]);
    o.append_section_data(text, &code, 4);
    let opd = o.add_section(Vec::new(), b".opd".to_vec(), SectionKind::Data);
    let mut descriptors = Vec::new();
    for n in [0x1000u64,0x3000,0,0x1000 + first.len() as u64,0x3001,0] { descriptors.extend(n.to_be_bytes()); }
    o.append_section_data(opd, &descriptors, 8);
    let data = o.add_section(Vec::new(), b".rodata".to_vec(), SectionKind::ReadOnlyData);
    o.append_section_data(data, &[7,9], 1);
    for (name, section, value) in [("answer",opd,0),("second",opd,24),(".answer",text,0),("alias",opd,0)] {
        o.add_symbol(Symbol { name:name.as_bytes().to_vec(), value, size:0,
            kind:SymbolKind::Text, scope:SymbolScope::Linkage, weak:false,
            section:SymbolSection::Section(section), flags:SymbolFlags::None });
    }
    if !relocations.is_empty() {
        let strings = o.add_section(Vec::new(), b".dynstr".to_vec(), SectionKind::Metadata);
        o.append_section_data(strings, &[0], 1);
        let symbols = o.add_section(Vec::new(), b".dynsym".to_vec(), SectionKind::Metadata);
        o.append_section_data(symbols, &[0; 24], 8);
        let rela = o.add_section(Vec::new(), b".rela.dyn".to_vec(), SectionKind::Metadata);
        let mut records = Vec::new();
        for &(address, kind, addend) in relocations {
            records.extend(address.to_be_bytes());
            records.extend(u64::from(kind).to_be_bytes());
            records.extend(addend.to_be_bytes());
        }
        o.append_section_data(rela, &records, 8);
    }
    let mut b=o.write().unwrap();
    b[16..18].copy_from_slice(&2u16.to_be_bytes());
    b[48..52].copy_from_slice(&abi.to_be_bytes());
    put64(&mut b,24,0x2000);
    let shoff=get64(&b,40) as usize;
    let shnum=u16::from_be_bytes(b[60..62].try_into().unwrap()) as usize;
    for i in 1..shnum {
        let sh=shoff+i*64;
        if i<=3 {put64(&mut b,sh+16,i as u64*0x1000);}
        if !relocations.is_empty() && matches!(i, 4..=6) {
            let (kind, link, stride) = match i {
                4 => (object::elf::SHT_STRTAB, 0u32, 0),
                5 => (object::elf::SHT_DYNSYM, 4, 24),
                _ => (object::elf::SHT_RELA, 5, 24),
            };
            b[sh+4..sh+8].copy_from_slice(&kind.to_be_bytes());
            b[sh+40..sh+44].copy_from_slice(&link.to_be_bytes());
            put64(&mut b, sh+56, stride);
        }
        if get32(&b,sh+4)==object::elf::SHT_SYMTAB {
            let start=get64(&b,sh+24) as usize;
            let end=start+get64(&b,sh+32) as usize;
            for sym in (start..end).step_by(24) {
                let section=u16::from_be_bytes(b[sym+6..sym+8].try_into().unwrap());
                if matches!(section,1|2) { let v=get64(&b,sym+8)+u64::from(section)*0x1000;put64(&mut b,sym+8,v); }
            }
        }
    }
    let phoff=b.len() as u64;put64(&mut b,32,phoff);
    b[54..56].copy_from_slice(&56u16.to_be_bytes());b[56..58].copy_from_slice(&3u16.to_be_bytes());
    for i in 1..=3 {
        let sh=shoff+i*64;let (offset,size)=(get64(&b,sh+24),get64(&b,sh+32));
        b.extend(1u32.to_be_bytes());b.extend((if i==1 {5u32} else {4u32}).to_be_bytes());
        for n in [offset,i as u64*0x1000,i as u64*0x1000,size,size,1] { b.extend(n.to_be_bytes()); }
    }
    b
}

#[test]
fn descriptor_relocations_override_payloads_and_uncertain_tocs_are_not_seeded() {
    use kuna_analysis::loader::elfv1::Descriptors;
    use object::{Object as _, ObjectSection as _};
    let code = [0x88,0x62,0,0,0x4e,0x80,0,0x20];
    let relative = object::elf::R_PPC64_RELATIVE;
    let relocs = [(0x2000, relative, 0x1000), (0x2008, relative, 0x3000)];
    for (entry_payload, toc_payload) in [(0x1008, 0x3001), (0x1000, 0)] {
        let mut bytes = image_with_relocations(1, &code, &relocs);
        bytes[16..18].copy_from_slice(&3u16.to_be_bytes());
        let file = object::File::parse(&*bytes).unwrap();
        assert_eq!(file.dynamic_relocations().unwrap().count(), 2);
        let offset = file.section_by_name(".opd").unwrap().file_range().unwrap().0 as usize;
        put64(&mut bytes, offset, entry_payload);
        put64(&mut bytes, offset+8, toc_payload);
        let loader = kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes_silent("synthetic", &bytes).unwrap();
        assert_eq!(loader.image_entry(), Some(0x1000));
        assert_eq!(loader.elfv1_descriptors().entry_tocs().get(&0x1000), Some(&0x3000));
        let mut shifted = loader.elfv1_descriptors().clone();
        shifted.adjust_vma(0x10000);
        assert_eq!(shifted.code_address(0x12000), 0x11000);
        assert_eq!(shifted.entry_tocs().get(&0x11000), Some(&0x13000));
        let path = common::scratch_file("elfv1-rela-descriptor", "elf");
        std::fs::write(&path, bytes).unwrap();
        let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args(["decompile", path.to_str().unwrap(), "answer", "--json", "--option", "readonly", "on"])
            .output().unwrap();
        let text = String::from_utf8_lossy(&out.stdout);
        assert!(out.status.success(), "{text}\n{}", String::from_utf8_lossy(&out.stderr));
        assert!(text.contains("\"address_hex\": \"0x1000\"") && text.contains("return 7;"), "{text}");
        std::fs::remove_file(path).unwrap();
    }
    for unknown_alias in [1, 0] {
        let relocs = [(0x2000, relative, 0x1000), (0x2018, relative, 0x1000),
            (0x2008 + unknown_alias * 24, object::elf::R_PPC64_ADDR64, 0x3000 + unknown_alias as i64)];
        let mut bytes = image_with_relocations(1, &code, &relocs);
        bytes[16..18].copy_from_slice(&3u16.to_be_bytes());
        let file = object::File::parse(&*bytes).unwrap();
        let descriptors = Descriptors::read(&file);
        let path = common::scratch_file("elfv1-uncertain-alias", "elf");
        std::fs::write(&path, bytes).unwrap();
        let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args(["decompile", path.to_str().unwrap(), "0x1000", "--addr", "--json", "--option", "readonly", "on"])
            .output().unwrap();
        let text = String::from_utf8_lossy(&out.stdout);
        assert!(out.status.success(), "{text}\n{}", String::from_utf8_lossy(&out.stderr));
        assert!(descriptors.entry_tocs().is_empty(), "{text}");
        assert_eq!(descriptors.code_address(0x2000), 0x1000);
        assert_eq!(descriptors.code_address(0x2018), 0x1000);
        assert_eq!(descriptors.0[unknown_alias as usize].toc, None);
        let mut shifted = descriptors.clone();
        shifted.adjust_vma(0x10000);
        assert!(shifted.entry_tocs().is_empty());
        assert_eq!(shifted.code_address(0x12018), 0x11000);
        assert_eq!(shifted.0[unknown_alias as usize].toc, None);
        assert!(text.contains("// r2") && text.contains("return *"), "{text}");
        std::fs::remove_file(path).unwrap();
    }
    for (bad_relocs, entry) in [
        (vec![(0x2000, object::elf::R_PPC64_ADDR64, 0x1000)], 0x2000),
        (vec![(0x2008, object::elf::R_PPC64_ADDR64, 0x3000)], 0x1000),
        (vec![(0x2004, relative, 0x1000)], 0x2000),
        (vec![(0x2000, relative, 0x1000), (0x2000, relative, 0x1008)], 0x2000),
    ] {
        let bytes = image_with_relocations(1, &code, &bad_relocs);
        let file = object::File::parse(&*bytes).unwrap();
        let descriptors = Descriptors::read(&file);
        assert_eq!(descriptors.code_address(0x2000), entry);
        assert!(!descriptors.entry_tocs().contains_key(&0x1000));
    }
}

#[test]
fn entry_tocs_preserve_live_register_defaults_and_user_overrides() {
    use kuna_console::engine::bootstrap_from_file;
    use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
    use kuna_console::ifaceterm::ConsoleCommands;
    let specs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    let path = common::scratch_file("elfv1-live-tracking", "elf");
    // lbz r4,0(r2); add r3,r3,r4; blr.
    std::fs::write(&path, image_with_code(1, &[0x88,0x82,0,0,0x7c,0x63,0x22,0x14,0x4e,0x80,0,0x20])).unwrap();
    for (tracking, value) in [
        (vec!["set track r3 5"], "0xc"),
        (vec!["set track r3 5", "set track r2 0x3001"], "0xe"),
        (vec!["set track r3 5 0x1000 0x1001"], "0xc"),
        (vec!["set track r3 5", "set track r2 0x3001 0x1000 0x1001"], "0xe"),
    ] {
        let prog = bootstrap_from_file(path.to_str().unwrap(), "", &[specs.to_str().unwrap().into()]).unwrap();
        let mut commands: Vec<String> = tracking.into_iter().map(str::to_string).collect();
        commands.extend(["option readonly on", "load function answer", "decompile", "print C"].map(str::to_string));
        let count = commands.len();
        let mut status = ConsoleCommands::into_status(commands);
        register_decomp_commands(&mut status);
        status.get_data_mut(DECOMPILE_MODULE).unwrap().as_any_mut()
            .downcast_mut::<IfaceDecompData>().unwrap().conf = Some(prog);
        for _ in 0..count { execute(&mut status); }
        assert!(status.optr.contains(&format!("return {value};")), "{}", status.optr);
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn descriptors_select_code_and_seed_each_functions_toc() {
    for abi in [0,1] {
        let bytes=image(abi);
        let loader=kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes_silent("synthetic",&bytes).unwrap();
        assert_eq!(loader.image_entry(),Some(0x1000));
        assert!(loader.func_symbols().iter().all(|(address,_)| matches!(address,0x1000|0x1008)));
        assert_eq!(loader.func_symbols().len(),4);
        let path=common::scratch_file("elfv1-entries","elf");std::fs::write(&path,bytes).unwrap();
        for (selector,address,value) in [("answer","0x1000",7),("alias","0x1000",7),(".answer","0x1000",7),("second","0x1008",9),("0x1008","0x1008",9)] {
            let out=Command::new(env!("CARGO_BIN_EXE_kuna"))
                .args(["decompile",path.to_str().unwrap(),selector,"--json","--option","readonly","on"]).output().unwrap();
            let text=String::from_utf8_lossy(&out.stdout);
            assert!(out.status.success(),"{text}\n{}",String::from_utf8_lossy(&out.stderr));
            assert!(text.contains(&format!("\"address_hex\": \"{address}\"")),"{text}");
            assert!(text.contains(&format!("return {value};")),"{text}");
        }
        let out=Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args(["decompile",path.to_str().unwrap(),"0x2000","--addr","--json"]).output().unwrap();
        assert!(String::from_utf8_lossy(&out.stdout).contains("\"address_hex\": \"0x2000\""));
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn invalid_descriptors_and_other_abis_keep_literal_addresses() {
    use kuna_analysis::loader::elfv1::Descriptors;
    use object::{Object as _, ObjectSection as _};
    for abi in [2,3] {
        let bytes=image(abi);let file=object::File::parse(&*bytes).unwrap();
        assert!(Descriptors::read(&file).0.is_empty());
    }
    for entry in [0x1001,0x3000,0x100f,0x4000,u64::MAX] {
        let mut bytes=image(1);
        let file=object::File::parse(&*bytes).unwrap();
        let offset=file.section_by_name(".opd").unwrap().file_range().unwrap().0 as usize;
        put64(&mut bytes,offset,entry);
        let file=object::File::parse(&*bytes).unwrap();
        assert_eq!(Descriptors::read(&file).code_address(0x2000),0x2000);
    }
    let mut bytes=image(1);bytes[16..18].copy_from_slice(&1u16.to_be_bytes());
    let file=object::File::parse(&*bytes).unwrap();
    assert!(Descriptors::read(&file).0.is_empty());
    let mut bytes=image(1);let shoff=get64(&bytes,40) as usize;
    put64(&mut bytes,shoff+2*64+32,16);
    let file=object::File::parse(&*bytes).unwrap();
    assert!(Descriptors::read(&file).0.is_empty());
}

#[test]
fn entry_reports_use_normalized_code_and_reachability() {
    for abi in [0, 1] {
        let path = common::scratch_file("elfv1-entry-summary", "elf");
        // bl 0x1008; blr. The entry reaches the second function.
        std::fs::write(&path, image_with_code(abi, &[0x48,0,0,9,0x4e,0x80,0,0x20])).unwrap();
        let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args(["functions", path.to_str().unwrap(), "--summary", "--json"])
            .output().unwrap();
        let text = String::from_utf8_lossy(&out.stdout);
        assert!(out.status.success(), "{text}\n{}", String::from_utf8_lossy(&out.stderr));
        let entry = text.split("\"entry\": {").nth(1).unwrap().split('}').next().unwrap();
        assert!(entry.contains("\"address_hex\": \"0x1000\""), "{text}");
        assert!(["answer", "alias", ".answer"].iter()
            .any(|name| entry.contains(&format!("\"name\": \"{name}\""))), "{text}");
        assert!(text.contains("\"reachable_from_entry\": 2"), "{text}");
        let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args(["decompile-graph", path.to_str().unwrap()])
            .output().unwrap();
        let text = String::from_utf8_lossy(&out.stdout);
        assert!(out.status.success(), "{text}\n{}", String::from_utf8_lossy(&out.stderr));
        let flagged: Vec<&str> = text.split("\n    {").skip(1)
            .map(|row| row.split("\n    }").next().unwrap())
            .filter(|row| row.contains("\"isEntryPoint\": true"))
            .collect();
        assert_eq!(flagged.len(), 1, "{text}");
        assert!(flagged[0].contains("\"address\": 4096,"), "{text}");
        std::fs::remove_file(path).unwrap();
    }
}

/// A call through a TOC-switching stub, the caller's `ld r2,40(r1)` reload, and a
/// four-entry relative jump table addressed `addis/addi` from the reloaded `r2`.
fn toc_reload_switch() -> Vec<u8> {
    let words: [u32; 36] = [
        0x7c0802a6, 0xf8010010, 0xf821ff81, 0x38610070, // mflr; std r0; stdu; addi r3,r1,112
        0x48000075, 0xe8410028, 0x81210070, 0x28090003, // bl stub; ld r2,40(r1); lwz r9,112(r1); cmplwi r9,3
        0x41810050, 0x3d420000, 0x79291764, 0x394ae040, // bgt default; addis r10,r2,0; sldi r9,r9,2; addi r10,r10,-0x1fc0
        0x7d2a4aaa, 0x7d295214, 0x7d2903a6, 0x4e800420, // lwax; add; mtctr; bctr
        0x10, 0x18, 0x20, 0x28,                         // table at 0x1040
        0x3860000b, 0x48000020, 0x38600016, 0x48000018, // case 0: li r3,11; case 1: li r3,22
        0x38600021, 0x48000010, 0x3860002c, 0x48000008, // case 2: li r3,33; case 3: li r3,44
        0x3860ffff, 0x38210080, 0xe8010010, 0x7c0803a6, // default: li r3,-1; epilogue
        0x4e800020, 0xf8410028, 0x38420001, 0x48000004, // blr; stub: std r2,40(r1); addi r2,r2,1; b second
    ];
    words.iter().flat_map(|w| w.to_be_bytes()).collect()
}

#[test]
fn elfv1_toc_reload_after_a_call_keeps_the_jump_table_base() {
    let decompile = |path: &std::path::Path, option: Option<&str>| {
        let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
        command.env("KUNA_SPECS", common::repo_root().join("specs"))
            .args(["decompile", path.to_str().unwrap(), ".answer"]);
        if let Some(value) = option { command.args(["--option", "callmechanisms", value]); }
        let out = command.output().unwrap();
        let text = String::from_utf8(out.stdout).unwrap();
        assert!(out.status.success(), "{text}\n{}", String::from_utf8_lossy(&out.stderr));
        text
    };
    let recovered = |text: &str| text.contains("switch(") && !text.contains("jump-as-call")
        && ["case 0:", "case 1:", "case 2:", "case 3:"].iter().all(|label| text.contains(label))
        && ["0xb;", "0x16;", "0x21;", "0x2c;"].iter().all(|value| text.contains(&format!("return {value}")));
    for abi in [0, 1, 2] {
        let path = common::scratch_file("elfv1-toc-reload", "elf");
        std::fs::write(&path, image_with_code(abi, &toc_reload_switch())).unwrap();
        let default = decompile(&path, None);
        assert!(abi == 2 || recovered(&default), "abi {abi}\n{default}");
        let [auto, on, off] = [Some("auto"), Some("on"), Some("off")].map(|o| decompile(&path, o));
        std::fs::remove_file(&path).unwrap();
        assert!(off.contains("jump-as-call") && !off.contains("switch("), "abi {abi}\n{off}");
        assert_eq!(default, auto, "abi {abi}");
        assert_eq!(default, if abi == 2 { off } else { on }, "abi {abi}");
    }
}

/// `int valid(int); void both(int,int), one(int);`
/// `void route(int a, int b) { if (valid(a) && valid(b)) both(a, b); else one(a); }`
/// at `-O2 -fPIC`. With same-module callees `route` (0x1040) never reloads its TOC.
fn short_circuit_local_calls() -> Vec<u8> {
    let words: [u32; 48] = [
        0x2c030004, 0x38600001, 0x7c60181e, 0x4e800020, 0x00000000, 0x00000000,
        0x00000000, 0x60000000, 0x4e800020, 0x00000000, 0x00000000, 0x00000000,
        0x4e800020, 0x00000000, 0x00000000, 0x00000000, 0x7c0802a6, 0xfbc1fff0,
        0xfbe1fff8, 0x7c7e1b78, 0xf8010010, 0x7c9f2378, 0xf821ff81, 0x4bffffa5,
        0x2c030000, 0x41c20014, 0x7fe3fb78, 0x4bffff95, 0x2c030000, 0x40c2002c,
        0x38210080, 0x7fc3f378, 0xe8010010, 0xebc1fff0, 0xebe1fff8, 0x7c0803a6,
        0x4bffffa0, 0x60000000, 0x60000000, 0x60000000, 0x38210080, 0x7fe4fb78,
        0xe8010010, 0x7fc3f378, 0xebe1fff8, 0xebc1fff0, 0x7c0803a6, 0x4bffff64,
    ];
    words.iter().flat_map(|w| w.to_be_bytes()).collect()
}

/// The same `route` with external callees: three PLT call stubs, then `route` (0x1060),
/// which reloads its TOC with `ld r2,40(r1)` after every call but never reads it.
fn short_circuit_plt_calls() -> Vec<u8> {
    let words: [u32; 62] = [
        0xf8410028, 0xe9828148, 0x7d8903a6, 0xe8428150, 0x28220000, 0x4ce20420,
        0x4800011c, 0x00000000, 0xf8410028, 0xe9828130, 0x7d8903a6, 0xe8428138,
        0x28220000, 0x4ce20420, 0x480000f4, 0x00000000, 0xf8410028, 0xe9828118,
        0x7d8903a6, 0xe8428120, 0x28220000, 0x4ce20420, 0x480000cc, 0x00000000,
        0x7c0802a6, 0xf8010010, 0xf821ff91, 0xf88100a8, 0xf86100a0, 0x4bffffcd,
        0xe8410028, 0x2c030000, 0x41c20018, 0xe86100a8, 0x4bffffb9, 0xe8410028,
        0x2c030000, 0x40c2002c, 0xe86100a0, 0x4bffff85, 0xe8410028, 0x38210070,
        0xe8010010, 0x7c0803a6, 0x4e800020, 0x60000000, 0x60000000, 0x60000000,
        0xe88100a8, 0xe86100a0, 0x4bffff39, 0xe8410028, 0x38210070, 0xe8010010,
        0x7c0803a6, 0x4e800020, 0x00000000, 0x00000001, 0x80000000, 0x60000000,
        0x00000000, 0x0001fc00,
    ];
    words.iter().flat_map(|w| w.to_be_bytes()).collect()
}

#[test]
fn elfv1_calls_keep_their_short_circuit_condition() {
    for (code, address) in [(short_circuit_local_calls(), "0x1040"), (short_circuit_plt_calls(), "0x1060")] {
        let path = common::scratch_file("elfv1-short-circuit", "elf");
        std::fs::write(&path, image_with_code(1, &code)).unwrap();
        let [default, on, off] = [None, Some("on"), Some("off")].map(|option| {
            let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
            command.env("KUNA_SPECS", common::repo_root().join("specs"))
                .args(["decompile", path.to_str().unwrap(), address, "--addr"]);
            if let Some(value) = option { command.args(["--option", "callmechanisms", value]); }
            let out = command.output().unwrap();
            let text = String::from_utf8(out.stdout).unwrap();
            assert!(out.status.success(), "{text}\n{}", String::from_utf8_lossy(&out.stderr));
            text
        });
        std::fs::remove_file(&path).unwrap();
        assert!(off.contains(") && ("), "{address}\n{off}");
        assert_eq!(default, off, "{address}");
        assert!(!on.contains(") && ("), "a store after each call splits the condition at {address}\n{on}");
    }
}
