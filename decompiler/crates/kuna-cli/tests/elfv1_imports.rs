mod common;
use object::write::{Object, Relocation, Symbol, SymbolSection};
use object::{Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope};
use std::process::Command;

fn get32(b: &[u8], p: usize) -> u32 { u32::from_be_bytes(b[p..p+4].try_into().unwrap()) }
fn get64(b: &[u8], p: usize) -> u64 { u64::from_be_bytes(b[p..p+8].try_into().unwrap()) }
fn put64(b: &mut [u8], p: usize, n: u64) { b[p..p+8].copy_from_slice(&n.to_be_bytes()); }

/// Synthetic linked ELFv1: two descriptors, two TOCs, and a code alias.
fn image(abi: u32, import: &str) -> Vec<u8> {
    let mut o = Object::new(BinaryFormat::Elf, Architecture::PowerPc64, Endianness::Big);
    let text = o.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    let mut code=Vec::new();
    for word in [0x88630000u32,0x2c240000,0x40820008,0x4e800020,0x480000f1,0x4e800020] {code.extend(word.to_be_bytes());}
    code.resize(0x100,0);
    for word in [0xf8410028u32,0x3d620000,0xe98b0000,0x7d8903a6,0xe84b0008,0x28220000,0x4ce20420,0x48000004] {code.extend(word.to_be_bytes());}
    o.append_section_data(text,&code,4);
    let opd = o.add_section(Vec::new(), b".opd".to_vec(), SectionKind::Data);
    let mut descriptors = Vec::new();
    for n in [0x1000u64,0x3000,0] { descriptors.extend(n.to_be_bytes()); }
    o.append_section_data(opd, &descriptors, 8);
    let plt=o.add_section(Vec::new(),b".plt".to_vec(),SectionKind::Data);
    o.append_section_data(plt,&[0;24],8);
    o.add_symbol(Symbol {name:b"get_byte".to_vec(),value:0,size:24,kind:SymbolKind::Text,
        scope:SymbolScope::Linkage,weak:false,section:SymbolSection::Section(opd),flags:SymbolFlags::None});
    let sym=o.add_symbol(Symbol {name:import.as_bytes().to_vec(),value:0,size:0,kind:SymbolKind::Text,
        scope:SymbolScope::Linkage,weak:false,section:SymbolSection::Undefined,flags:SymbolFlags::None});
    o.add_relocation(plt,Relocation {offset:0,symbol:sym,addend:0,flags:object::RelocationFlags::Elf {r_type:object::elf::R_PPC64_JMP_SLOT}}).unwrap();
    let mut b=o.write().unwrap();
    b[16..18].copy_from_slice(&2u16.to_be_bytes());
    b[48..52].copy_from_slice(&abi.to_be_bytes());
    put64(&mut b,24,0x2000);
    let shoff=get64(&b,40) as usize;
    let shnum=u16::from_be_bytes(b[60..62].try_into().unwrap()) as usize;
    for i in 1..shnum {
        let sh=shoff+i*64;
        if i<=3 {put64(&mut b,sh+16,i as u64*0x1000);}
        let kind=get32(&b,sh+4);
        if kind==object::elf::SHT_RELA {
            let offset=get64(&b,sh+24) as usize;
            put64(&mut b,offset,0x3000);
        }
        if kind==object::elf::SHT_SYMTAB {
            b[sh+4..sh+8].copy_from_slice(&object::elf::SHT_DYNSYM.to_be_bytes());
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
fn descriptor_stubs_receive_import_names_and_known_noreturn_facts() {
    for import in ["__stack_chk_fail","returning_import"] {
        let path=common::scratch_file("elfv1-import","elf");std::fs::write(&path,image(1,import)).unwrap();
        let out=Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args(["decompile",path.to_str().unwrap(),"get_byte","--json"]).output().unwrap();
        let text=String::from_utf8_lossy(&out.stdout);
        assert!(out.status.success(),"{text}\n{}",String::from_utf8_lossy(&out.stderr));
        assert!(text.contains(&format!("{import}(")),"{text}");
        if import=="__stack_chk_fail" {
            assert!(text.contains("return "),"{text}");
            assert!(text.contains("char") && text.contains("no-return"),"{text}");
        } else {assert!(!text.contains("no-return"),"{text}");}
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn other_abis_and_non_jump_relocations_do_not_name_descriptor_stubs() {
    for abi in [2,3] {
        let bytes=image(abi,"__stack_chk_fail");let file=object::File::parse(&*bytes).unwrap();
        assert!(kuna_analysis::loader::format::resolve_imports(&file,&bytes).is_empty());
    }
    let mut bytes=image(1,"__stack_chk_fail");
    let shoff=get64(&bytes,40) as usize;
    let shnum=u16::from_be_bytes(bytes[60..62].try_into().unwrap()) as usize;
    for i in 0..shnum {
        let sh=shoff+i*64;
        if get32(&bytes,sh+4)==object::elf::SHT_RELA {
            let offset=get64(&bytes,sh+24) as usize;
            let info=get64(&bytes,offset+8);
            put64(&mut bytes,offset+8,(info & !0xffffffff)|u64::from(object::elf::R_PPC64_ADDR64));
        }
    }
    let file=object::File::parse(&*bytes).unwrap();
    assert!(kuna_analysis::loader::format::resolve_imports(&file,&bytes).is_empty());
}
