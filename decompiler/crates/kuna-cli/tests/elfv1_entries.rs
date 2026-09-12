mod common;
use object::write::{Object, Symbol, SymbolSection};
use object::{Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope};
use std::process::Command;

fn get32(b: &[u8], p: usize) -> u32 { u32::from_be_bytes(b[p..p+4].try_into().unwrap()) }
fn get64(b: &[u8], p: usize) -> u64 { u64::from_be_bytes(b[p..p+8].try_into().unwrap()) }
fn put64(b: &mut [u8], p: usize, n: u64) { b[p..p+8].copy_from_slice(&n.to_be_bytes()); }

/// Synthetic linked ELFv1: two descriptors, two TOCs, and a code alias.
fn image(abi: u32) -> Vec<u8> {
    let mut o = Object::new(BinaryFormat::Elf, Architecture::PowerPc64, Endianness::Big);
    let text = o.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    o.append_section_data(text, &([0x88,0x62,0,0,0x4e,0x80,0,0x20].repeat(2)), 4);
    let opd = o.add_section(Vec::new(), b".opd".to_vec(), SectionKind::Data);
    let mut descriptors = Vec::new();
    for n in [0x1000u64,0x3000,0,0x1008,0x3001,0] { descriptors.extend(n.to_be_bytes()); }
    o.append_section_data(opd, &descriptors, 8);
    let data = o.add_section(Vec::new(), b".rodata".to_vec(), SectionKind::ReadOnlyData);
    o.append_section_data(data, &[7,9], 1);
    for (name, section, value) in [("answer",opd,0),("second",opd,24),(".answer",text,0),("alias",opd,0)] {
        o.add_symbol(Symbol { name:name.as_bytes().to_vec(), value, size:0,
            kind:SymbolKind::Text, scope:SymbolScope::Linkage, weak:false,
            section:SymbolSection::Section(section), flags:SymbolFlags::None });
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
