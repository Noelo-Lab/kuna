mod common;
use object::write::{Object, Relocation, Symbol, SymbolSection};
use object::{Object as _, ObjectSection as _};
use object::{Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope};
use std::process::Command;

fn get32(b: &[u8], p: usize) -> u32 { u32::from_be_bytes(b[p..p+4].try_into().unwrap()) }
fn get64(b: &[u8], p: usize) -> u64 { u64::from_be_bytes(b[p..p+8].try_into().unwrap()) }
fn put64(b: &mut [u8], p: usize, n: u64) { b[p..p+8].copy_from_slice(&n.to_be_bytes()); }

#[derive(Clone, Copy, PartialEq)]
enum AliasToc { Conflicting, Symbolic, LocalDescriptor, HiddenLocalDescriptor }

/// Synthetic linked ELFv1 with a descriptor-call stub and optional TOC aliases.
fn image(abi: u32, import: &str, alias_toc: Option<AliasToc>, import_addend: i64) -> Vec<u8> {
    let conflicting_aliases=alias_toc.is_some();
    let local=matches!(alias_toc,Some(AliasToc::LocalDescriptor|AliasToc::HiddenLocalDescriptor));
    let hidden=alias_toc==Some(AliasToc::HiddenLocalDescriptor);
    let mut o = Object::new(BinaryFormat::Elf, Architecture::PowerPc64, Endianness::Big);
    let text = o.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    let mut code=Vec::new();
    for word in [0x88630000u32,0x2c240000,0x40820008,0x4e800020,0x480000f1,0x38600007,0x4e800020] {code.extend(word.to_be_bytes());}
    code.resize(0x40,0);
    code.extend(0x38600009u32.to_be_bytes());
    code.extend(0x4e800020u32.to_be_bytes());
    if hidden {
        let caller=code[..28].to_vec();
        code.resize(0x60,0);code.extend(caller);
        code[0x70..0x74].copy_from_slice(&0x48000091u32.to_be_bytes());
    }
    code.resize(0x100,0);
    for word in [0xf8410028u32,0x3d620000,0xe98b0000,0x7d8903a6,0xe84b0008,0x28220000,0x4ce20420,0x48000004] {code.extend(word.to_be_bytes());}
    o.append_section_data(text,&code,4);
    let opd = o.add_section(Vec::new(), b".opd".to_vec(), SectionKind::Data);
    let functions = if conflicting_aliases {
        &[("get_byte",0x1000u64,0x3000), ("get_alias",if hidden {0x1060} else {0x1000},if local {0x2030} else {0x3018}), ("helper",if local {0} else {0x1040},0x3000)][..]
    } else { &[("get_byte",0x1000,0x3000)][..] };
    let mut descriptors = Vec::new();
    for (i,&(name,entry,toc)) in functions.iter().enumerate() {
        for n in [entry,toc,0] { descriptors.extend(n.to_be_bytes()); }
        if hidden && i==1 {continue;}
        o.add_symbol(Symbol {name:name.as_bytes().to_vec(),value:i as u64*24,size:24,kind:SymbolKind::Text,
            scope:SymbolScope::Linkage,weak:false,section:SymbolSection::Section(opd),flags:SymbolFlags::None});
    }
    o.append_section_data(opd, &descriptors, 8);
    let plt=o.add_section(Vec::new(),b".plt".to_vec(),SectionKind::Data);
    o.append_section_data(plt,&vec![0;if conflicting_aliases {48} else {24}],8);
    let sym=o.add_symbol(Symbol {name:import.as_bytes().to_vec(),value:0,size:0,kind:SymbolKind::Text,
        scope:SymbolScope::Linkage,weak:false,section:SymbolSection::Undefined,flags:SymbolFlags::None});
    o.add_relocation(plt,Relocation {offset:0,symbol:sym,addend:import_addend,flags:object::RelocationFlags::Elf {r_type:object::elf::R_PPC64_JMP_SLOT}}).unwrap();
    if conflicting_aliases && !local {
        let sym=o.add_symbol(Symbol {name:b"returning_import".to_vec(),value:0,size:0,kind:SymbolKind::Text,
            scope:SymbolScope::Linkage,weak:false,section:SymbolSection::Undefined,flags:SymbolFlags::None});
        o.add_relocation(plt,Relocation {offset:24,symbol:sym,addend:0,flags:object::RelocationFlags::Elf {r_type:object::elf::R_PPC64_JMP_SLOT}}).unwrap();
    }
    if alias_toc==Some(AliasToc::Symbolic) {
        let sym=o.add_symbol(Symbol {name:b"runtime_toc".to_vec(),value:24,size:0,kind:SymbolKind::Data,
            scope:SymbolScope::Linkage,weak:false,section:SymbolSection::Section(plt),flags:SymbolFlags::None});
        o.add_relocation(opd,Relocation {offset:32,symbol:sym,addend:0,flags:object::RelocationFlags::Elf {r_type:object::elf::R_PPC64_ADDR64}}).unwrap();
    }
    if local {
        o.add_relocation(opd,Relocation {offset:48,symbol:sym,addend:0x1040,flags:object::RelocationFlags::Elf {r_type:object::elf::R_PPC64_RELATIVE}}).unwrap();
    }
    let mut b=o.write().unwrap();
    b[16..18].copy_from_slice(&2u16.to_be_bytes());
    b[48..52].copy_from_slice(&abi.to_be_bytes());
    put64(&mut b,24,0x2000);
    let shoff=get64(&b,40) as usize;
    let shnum=u16::from_be_bytes(b[60..62].try_into().unwrap()) as usize;
    let shstr=u16::from_be_bytes(b[62..64].try_into().unwrap()) as usize;
    let names=get64(&b,shoff+shstr*64+24) as usize;
    let section_vmas: Vec<_>=(0..shnum).map(|i| {
        let name=&b[names+get32(&b,shoff+i*64) as usize..];
        let name=&name[..name.iter().position(|&c|c==0).unwrap()];
        match name {b".text"=>0x1000u64,b".opd"=>0x2000,b".plt"=>0x3000,_=>0}
    }).collect();
    for i in 1..shnum {
        let sh=shoff+i*64;
        if section_vmas[i]!=0 {put64(&mut b,sh+16,section_vmas[i]);}
        let kind=get32(&b,sh+4);
        if kind==object::elf::SHT_RELA {
            let offset=get64(&b,sh+24) as usize;
            let end=offset+get64(&b,sh+32) as usize;
            let target=section_vmas[get32(&b,sh+44) as usize];
            for reloc in (offset..end).step_by(24) {
                let address=get64(&b,reloc)+target;put64(&mut b,reloc,address);
                if get64(&b,reloc+8) as u32==object::elf::R_PPC64_RELATIVE {
                    put64(&mut b,reloc+8,u64::from(object::elf::R_PPC64_RELATIVE));
                }
            }
        }
        if kind==object::elf::SHT_SYMTAB {
            b[sh+4..sh+8].copy_from_slice(&object::elf::SHT_DYNSYM.to_be_bytes());
            let start=get64(&b,sh+24) as usize;
            let end=start+get64(&b,sh+32) as usize;
            for sym in (start..end).step_by(24) {
                let section=u16::from_be_bytes(b[sym+6..sym+8].try_into().unwrap());
                if let Some(&vma)=section_vmas.get(section as usize).filter(|&&vma|vma!=0) {
                    let v=get64(&b,sym+8)+vma;put64(&mut b,sym+8,v);
                }
            }
        }
    }
    let phoff=b.len() as u64;put64(&mut b,32,phoff);
    b[54..56].copy_from_slice(&56u16.to_be_bytes());b[56..58].copy_from_slice(&3u16.to_be_bytes());
    for (i,&vma) in section_vmas.iter().enumerate().filter(|&(_,vma)|*vma!=0) {
        let sh=shoff+i*64;let (offset,size)=(get64(&b,sh+24),get64(&b,sh+32));
        b.extend(1u32.to_be_bytes());b.extend((if vma==0x1000 {5u32} else {4u32}).to_be_bytes());
        for n in [offset,vma,vma,size,size,1] { b.extend(n.to_be_bytes()); }
    }
    b
}

#[test]
fn descriptor_import_names_and_noreturn_require_unambiguous_targets() {
    for (import,alias_toc,addend) in [
        ("__stack_chk_fail",None,0),("returning_import",None,0),
        ("__stack_chk_fail",Some(AliasToc::Conflicting),0),
        ("__stack_chk_fail",Some(AliasToc::Symbolic),0),
        ("__stack_chk_fail",Some(AliasToc::LocalDescriptor),0),
        ("__stack_chk_fail",Some(AliasToc::HiddenLocalDescriptor),0),
        ("__stack_chk_fail",None,24),("__stack_chk_fail",None,-24),
    ] {
        let conflicts=alias_toc.is_some();
        let named=!conflicts && addend==0;
        let bytes=image(1,import,alias_toc,addend);
        let file=object::File::parse(&*bytes).unwrap();
        assert!(file.dynamic_relocations().unwrap().any(|(slot,r)|
            slot==0x3000 && r.addend()==addend && !r.has_implicit_addend()
            && matches!(r.flags(),object::RelocationFlags::Elf {r_type} if r_type==object::elf::R_PPC64_JMP_SLOT)));
        if alias_toc==Some(AliasToc::Symbolic) {
            let file=object::File::parse(&*bytes).unwrap();
            let descriptors=kuna_analysis::loader::elfv1::Descriptors::read(&file);
            assert!(descriptors.0.iter().any(|d|d.address==0x2018 && d.toc.is_none()));
            assert!(file.dynamic_relocations().unwrap().any(|(slot,r)|
                slot==0x3018 && matches!(r.flags(),object::RelocationFlags::Elf {r_type} if r_type==object::elf::R_PPC64_JMP_SLOT)));
        }
        if matches!(alias_toc,Some(AliasToc::LocalDescriptor|AliasToc::HiddenLocalDescriptor)) {
            let file=object::File::parse(&*bytes).unwrap();
            let descriptors=kuna_analysis::loader::elfv1::Descriptors::read(&file);
            if alias_toc==Some(AliasToc::HiddenLocalDescriptor) {
                assert!(descriptors.0.iter().all(|d|d.address!=0x2018));
                let opd=file.section_by_name(".opd").unwrap().data().unwrap();
                assert_eq!((get64(opd,24),get64(opd,32)),(0x1060,0x2030));
            } else {assert!(descriptors.0.iter().any(|d|d.address==0x2018 && d.toc==Some(0x2030)));}
            assert!(descriptors.0.iter().any(|d|d.address==0x2030 && d.entry==0x1040));
            assert!(file.dynamic_relocations().unwrap().any(|(slot,r)|
                slot==0x2030 && matches!(r.flags(),object::RelocationFlags::Elf {r_type} if r_type==object::elf::R_PPC64_RELATIVE)));
        }
        let path=common::scratch_file("elfv1-import","elf");std::fs::write(&path,&bytes).unwrap();
        let hidden=alias_toc==Some(AliasToc::HiddenLocalDescriptor);
        let selector=if hidden {"0x1060"} else if conflicts {"get_alias"} else {"get_byte"};
        let mut command=Command::new(env!("CARGO_BIN_EXE_kuna"));
        command.args(["decompile",path.to_str().unwrap(),selector,"--json"]);
        if hidden {command.arg("--addr");}
        let out=command.output().unwrap();
        let text=String::from_utf8_lossy(&out.stdout);
        assert!(out.status.success(),"{text}\n{}",String::from_utf8_lossy(&out.stderr));
        if !named {
            assert!(!text.contains(&format!("{import}(")),"{text}");
            let file=object::File::parse(&*bytes).unwrap();
            assert!(kuna_analysis::loader::format::resolve_imports(&file,&bytes).is_empty());
        } else {assert!(text.contains(&format!("{import}(")),"{text}");}
        if import=="__stack_chk_fail" && named {
            assert!(text.contains("return "),"{text}");
            assert!(text.contains("char") && text.contains("no-return"),"{text}");
        } else {
            assert!(!text.contains("no-return"),"{text}");
            assert!(text.contains("return 7;"),"{text}");
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn other_abis_and_non_jump_relocations_do_not_name_descriptor_stubs() {
    for abi in [2,3] {
        let bytes=image(abi,"__stack_chk_fail",None,0);let file=object::File::parse(&*bytes).unwrap();
        assert!(kuna_analysis::loader::format::resolve_imports(&file,&bytes).is_empty());
    }
    let mut bytes=image(1,"__stack_chk_fail",None,0);
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
