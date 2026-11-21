//! ELFv1 descriptor PLT stubs, correlated with JMP_SLOT relocations and TOCs.
use object::{Object, ObjectSection, ObjectSymbol, RelocationFlags, RelocationTarget, SectionFlags};
use std::collections::{BTreeSet, HashMap};
use super::{elf_plt::PltSym, elfv1::Descriptors};

pub(super) fn resolve(file: &object::File<'_>) -> Vec<PltSym> {
    let mut out = Vec::new();
    visit(file, |addr, name, _, _| out.push(PltSym { addr, name, slot: None }));
    out
}

pub(super) fn visit(file: &object::File<'_>, mut found: impl FnMut(u64, Vec<u8>, Option<u64>, usize)) {
    let descriptors=Descriptors::read_for_imports(file);
    if descriptors.0.is_empty() {return;}
    let Some(tocs): Option<BTreeSet<_>>=descriptors.0.iter().map(|d|d.toc).collect() else {return;};
    let names: HashMap<_,_>=file.dynamic_symbols().filter_map(|s|
        s.name_bytes().ok().filter(|n|!n.is_empty()).map(|n|(s.index().0,super::elf_plt::strip_version(n)))
    ).collect();
    let mut slots=HashMap::new();
    if let Some(relocations)=file.dynamic_relocations() {
        for (slot,reloc) in relocations {
            if !matches!(reloc.flags(),RelocationFlags::Elf {r_type} if r_type==object::elf::R_PPC64_JMP_SLOT) {continue;}
            if reloc.has_implicit_addend() || reloc.addend()!=0 {continue;}
            let RelocationTarget::Symbol(index)=reloc.target() else {continue;};
            if let Some(name)=names.get(&index.0) {slots.insert(slot,name.clone());}
        }
    }
    if slots.is_empty() {return;}
    let lazy=LazyResolver::read(file);
    for section in file.sections() {
        if !matches!(section.flags(),SectionFlags::Elf {sh_flags} if sh_flags & 6 == 6) {continue;}
        let Ok(bytes)=section.data() else {continue;};
        for off in (0..bytes.len()).step_by(4) {
            let Some(first)=bytes.get(off..off+4) else {break;};
            let first=first.try_into().unwrap();
            let first=if file.is_little_endian() {u32::from_le_bytes(first)} else {u32::from_be_bytes(first)};
            if first!=0xf8410028 {continue;}
            let words=words(file,&bytes[off..bytes.len().min(off+36)]);
            let Some((delta,branch,len))=stub_shape(&words) else {continue;};
            let addr=section.address().wrapping_add(off as u64);
            if let Some(branch)=branch {
                let Some(slot)=lazy.as_ref().and_then(|r|r.slot(file,addr.wrapping_add(branch as u64))) else {continue;};
                if !tocs.iter().all(|toc|toc.wrapping_add(delta as u64)==slot) {continue;}
            }
            if let Some(name) = unique_import(delta, &tocs, &slots) {
                let slot = tocs.first().filter(|_| tocs.len() == 1).map(|toc| toc.wrapping_add(delta as u64));
                found(addr, name, slot, len);
            }
        }
    }
}

fn unique_import(delta: i64, tocs: &BTreeSet<u64>, slots: &HashMap<u64,Vec<u8>>) -> Option<Vec<u8>> {
    let matches: BTreeSet<_>=tocs.iter().map(|toc|slots.get(&toc.wrapping_add(delta as u64))).collect::<Option<_>>()?;
    (matches.len()==1).then(||(*matches.first().unwrap()).clone())
}

fn ld(word: u32, rt: u32, ra: u32) -> Option<i64> {
    (word & 0xffff0003 == (0xe8000000 | (rt<<21) | (ra<<16)))
        .then_some((word as u16 as i16) as i64)
}
fn mtctr(reg: u32) -> u32 { 0x7c0903a6 | (reg<<21) }

/// Return the TOC displacement and optional lazy-branch offset from the stub start.
/// The environment load is optional; from r2 itself it must precede the TOC load.
#[cfg(test)]
fn stub_displacement(w: &[u32]) -> Option<(i64, Option<i64>)> {
    stub_shape(w).map(|(delta, branch, _)| (delta, branch))
}

fn stub_shape(w: &[u32]) -> Option<(i64,Option<i64>,usize)> {
    if w.first()!=Some(&0xf8410028) {return None;}
    let mut i=1;
    let mut base=2;
    let mut high=0;
    let first=*w.get(i)?;
    if first & 0xfc1f0000 == 0x3c020000 {
        base=(first>>21)&31;
        if !matches!(base,11|12) {return None;}
        high=(first as u16 as i16 as i64)<<16;
        i+=1;
    }
    let target=(*w.get(i)?>>21)&31;
    if !matches!(target,11|12) || target==base {return None;}
    let low=ld(*w.get(i)?,target,base)?;i+=1;
    let early_ctr=*w.get(i)?==mtctr(target);
    if early_ctr {i+=1;}
    if early_ctr && base==2 && ld(*w.get(i)?,11,2)==Some(low+16) {i+=1;}
    if ld(*w.get(i)?,2,base)!=Some(low+8) {return None;}i+=1;
    if !early_ctr {
        if *w.get(i)?!=mtctr(target) {return None;}i+=1;
    }
    if base!=2 && ld(*w.get(i)?,11,base)==Some(low+16) {i+=1;}
    if *w.get(i)?==0x4e800420 {return Some((high+low,None,(i+1)*4));}
    if *w.get(i)?!=0x28220000 {return None;}i+=1;
    if *w.get(i)? & !0x00600000 != 0x4c820420 {return None;}i+=1;
    Some((high+low,Some(i as i64*4+branch_displacement(*w.get(i)?)?),(i+1)*4))
}

fn branch_displacement(word: u32) -> Option<i64> {
    (word & 0xfc000003 == 0x48000000).then_some(((word << 6) as i32 >> 6) as i64)
}

fn read64(file: &object::File<'_>, bytes: &[u8]) -> u64 {
    let word=bytes[..8].try_into().unwrap();
    if file.is_little_endian() {u64::from_le_bytes(word)} else {u64::from_be_bytes(word)}
}

fn bytes_at<'a>(file: &object::File<'a>, addr: u64, size: usize, executable: bool) -> Option<&'a [u8]> {
    for section in file.sections() {
        let flags=if executable {6} else {2};
        if !matches!(section.flags(),SectionFlags::Elf {sh_flags} if sh_flags & flags == flags) {continue;}
        let Some(off)=addr.checked_sub(section.address()).and_then(|n|usize::try_from(n).ok()) else {continue;};
        if let Some(bytes)=section.data().ok().and_then(|b|b.get(off..off.checked_add(size)?)) {return Some(bytes);}
    }
    None
}

fn words(file: &object::File<'_>, bytes: &[u8]) -> Vec<u32> {
    bytes.chunks_exact(4).map(|b| {
        let word=b.try_into().unwrap();
        if file.is_little_endian() {u32::from_le_bytes(word)} else {u32::from_be_bytes(word)}
    }).collect()
}

/// Validate the GNU ELFv1 glink code and its dynamic-linker descriptor before
/// accepting a lazy branch. Unknown resolver layouts carry no import facts.
struct LazyResolver<'a> { start: u64, relocations: &'a [u8] }
impl<'a> LazyResolver<'a> {
    fn read(file: &object::File<'a>) -> Option<Self> {
        use object::elf::{DT_JMPREL,DT_PLTGOT,DT_PLTREL,DT_PLTRELSZ,DT_PPC64_GLINK,DT_RELA};
        let mut tags=HashMap::new();
        for entry in file.section_by_name(".dynamic")?.data().ok()?.chunks_exact(16) {
            let tag=read64(file,entry);
            if tag==0 {break;}
            if [DT_JMPREL,DT_PLTGOT,DT_PLTREL,DT_PLTRELSZ,DT_PPC64_GLINK].iter().any(|&t|t as u64==tag)
                && tags.insert(tag,read64(file,&entry[8..])).is_some() {return None;}
        }
        let get=|tag: i64|tags.get(&(tag as u64)).copied();
        if get(DT_PLTREL)?!=DT_RELA as u64 {return None;}
        let start=get(DT_PPC64_GLINK)?.checked_sub(12)?;
        let code=words(file,bytes_at(file,start,44,true)?);
        if code!=[0x7d8802a6,0x429f0005,0x7d6802a6,0xe84bfff0,0x7d8803a6,0x7d625a14,
            0xe98b0000,0xe84b0008,0x7d8903a6,0xe96b0010,0x4e800420] {return None;}
        let offset=read64(file,bytes_at(file,start.checked_sub(8)?,8,true)?);
        if start.wrapping_add(8).wrapping_add(offset)!=get(DT_PLTGOT)? {return None;}
        let size=usize::try_from(get(DT_PLTRELSZ)?).ok()?;
        if size%24!=0 {return None;}
        let relocations=bytes_at(file,get(DT_JMPREL)?,size,false)?;
        Some(Self {start,relocations})
    }
    fn slot(&self, file: &object::File<'_>, entry: u64) -> Option<u64> {
        let offset=entry.checked_sub(self.start.checked_add(44)?)?;
        if offset%8!=0 || offset/8>=0x8000 {return None;}
        let index=offset/8;
        let code=words(file,bytes_at(file,entry,8,true)?);
        if code[0]!=(0x38000000|index as u32)
            || entry.wrapping_add(4).wrapping_add(branch_displacement(code[1])? as u64)!=self.start {return None;}
        let off=index as usize*24;
        let reloc=self.relocations.get(off..off+24)?;
        if read64(file,&reloc[8..]) as u32!=object::elf::R_PPC64_JMP_SLOT || read64(file,&reloc[16..])!=0 {return None;}
        Some(read64(file,reloc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const LAZY: [u32;8]=[0xf8410028,0x3d620000,0xe98b0000,0x7d8903a6,0xe84b0008,0x28220000,0x4ca20420,0x48000004];
    #[test]
    fn conflicting_toc_resolutions_are_declined() {
        let tocs=BTreeSet::from([0x3000,0x4000]);
        let mut slots=HashMap::from([(0x3020,b"exit".to_vec()),(0x4020,b"ordinary".to_vec())]);
        assert_eq!(unique_import(0x20,&tocs,&slots),None);
        slots.insert(0x4020,b"exit".to_vec());
        assert_eq!(unique_import(0x20,&tocs,&slots),Some(b"exit".to_vec()));
        slots.remove(&0x4020);
        assert_eq!(unique_import(0x20,&tocs,&slots),None);
        assert_eq!(unique_import(0x28,&tocs,&slots),None);
    }
    #[test]
    fn lazy_descriptor_stubs_validate_the_full_transfer() {
        assert_eq!(stub_displacement(&LAZY),Some((0,Some(32))));
        for hints in 0..4 {
            let mut predicted=LAZY;predicted[6]=0x4c820420|(hints<<21);
            assert_eq!(stub_displacement(&predicted),Some((0,Some(32))));
        }
        let mut negative=LAZY;negative[1]|=0xffff;negative[2]|=0x8000;negative[4]|=0x8000;
        assert_eq!(stub_displacement(&negative),Some((-0x18000,Some(32))));
        let mut backwards=LAZY;backwards[7]=0x4bffffe0;
        assert_eq!(stub_displacement(&backwards),Some((0,Some(-4))));
        for i in 0..LAZY.len() {assert_eq!(stub_displacement(&LAZY[..i]),None);}
        for (i,value) in [(0,0xf8410018),(3,0x7d6903a6),(4,0xe84b0010),(5,0x2c220000),(6,0x4c820421),(7,0x48000005)] {
            let mut bad=LAZY;bad[i]=value;assert_eq!(stub_displacement(&bad),None,"word {i}");
        }
    }
    #[test]
    fn full_environment_and_small_toc_forms_are_recognized() {
        let full=[0xf8410028,0x3d820000,0xe96c0040,0xe84c0048,0x7d6903a6,0xe96c0050,0x4e800420];
        assert_eq!(stub_displacement(&full),Some((0x40,None)));
        let small=[0xf8410028,0xe9820040,0x7d8903a6,0xe8420048,0x28220000,0x4ca20420,0x48000004];
        assert_eq!(stub_displacement(&small),Some((0x40,Some(28))));
    }
    #[test]
    fn small_toc_environment_form_loads_the_environment_before_the_toc() {
        const R11: [u32;6]=[0xf8410028,0xe9628118,0x7d6903a6,0xe9628128,0xe8428120,0x4e800420];
        const R12: [u32;6]=[0xf8410028,0xe9828118,0x7d8903a6,0xe9628128,0xe8428120,0x4e800420];
        for stub in [R11,R12] {
            assert_eq!(stub_displacement(&stub),Some((-32488,None)));
            let mut lazy=stub[..5].to_vec();lazy.extend([0x28220000,0x4ce20420,0x480000b8]);
            assert_eq!(stub_displacement(&lazy),Some((-32488,Some(28+0xb8))));
            for i in 0..lazy.len() {assert_eq!(stub_displacement(&lazy[..i]),None,"{i} words");}
            for (i,value,why) in [
                (0,0xf8410018,"TOC save slot"),(2,stub[2]^0x00200000,"CTR source"),
                (3,0xe9628120,"environment at +8"),(3,0xe9628130,"environment at +24"),
                (3,0xe9828128,"environment in r12"),(3,0xe96b8128,"environment from r11"),
                (3,0xe9628129,"environment load with update"),
                (4,0xe8428128,"TOC at +16"),(4,0xe8428118,"TOC at +0"),(4,0xe84b8120,"TOC from r11"),
                (4,0xe9628120,"TOC in r11"),(5,0x4e800020,"return"),(5,0x4e800421,"linking branch"),
            ] {
                let mut bad=stub;bad[i]=value;assert_eq!(stub_displacement(&bad),None,"{why}");
            }
            let swapped=[stub[0],stub[1],stub[2],stub[4],stub[3],stub[5]];
            assert_eq!(stub_displacement(&swapped),None,"environment read through the callee TOC");
            let late_ctr=[stub[0],stub[1],stub[3],stub[2],stub[4],stub[5]];
            assert_eq!(stub_displacement(&late_ctr),None,"environment load before the CTR move");
            let last_ctr=[stub[0],stub[1],stub[3],stub[4],stub[2],stub[5]];
            assert_eq!(stub_displacement(&last_ctr),None,"CTR move after both loads");
        }
        let full_base=[0xf8410028,0x3d620000,0xe98b0040,0x7d8903a6,0xe96b0050,0xe84b0048,0x4e800420];
        assert_eq!(stub_displacement(&full_base),None,"environment overwrites the high-adjusted base");
        let mixed_base=[0xf8410028,0x3d820001,0xe96c0040,0x7d6903a6,0xe9620050,0xe84c0048,0x4e800420];
        assert_eq!(stub_displacement(&mixed_base),None,"environment from r2 beside a high-adjusted base");
    }
    #[test]
    fn either_ending_follows_every_base_and_environment_choice() {
        const TAIL: [u32;3]=[0x28220000,0x4ce20420,0x480000b8];
        let near=[0xf8410028u32,0xe9828118,0x7d8903a6,0xe8428120];
        let near_env=[0xf8410028,0xe9828118,0x7d8903a6,0xe9628128,0xe8428120];
        let far=[0xf8410028,0x3d620001,0xe98b8118,0x7d8903a6,0xe84b8120];
        let far_env=[0xf8410028,0x3d620001,0xe98b8118,0x7d8903a6,0xe84b8120,0xe96b8128];
        let far_r12=[0xf8410028,0x3d820001,0xe96c8118,0x7d6903a6,0xe84c8120];
        let late_ctr=[0xf8410028,0xe9628118,0xe8428120,0x7d6903a6];
        for (body,toc,delta) in [(&near[..],3,-32488),(&near_env[..],4,-32488),(&far[..],4,33048),
            (&far_env[..],4,33048),(&far_r12[..],4,33048),(&late_ctr[..],2,-32488)] {
            let direct=[body,&[0x4e800420][..]].concat();
            let lazy=[body,&TAIL[..]].concat();
            assert_eq!(stub_displacement(&direct),Some((delta,None)),"{body:x?}");
            assert_eq!(stub_displacement(&lazy),Some((delta,Some(body.len() as i64*4+8+0xb8))),"{body:x?}");
            for stub in [&direct,&lazy] {
                for i in 0..stub.len() {assert_eq!(stub_displacement(&stub[..i]),None,"{i} of {stub:x?}");}
                for (i,value,why) in [
                    (0,0xf8410018,"TOC save slot"),(toc,0x60000000,"no TOC load"),
                    (toc,stub[toc]+8,"TOC at +16"),(toc,stub[toc]^0x00010000,"TOC from another base"),
                    (body.len(),0x4e800020,"return"),(body.len(),0x4e800421,"linking branch"),
                    (body.len(),0x4c820420,"conditional branch without its compare"),
                    (body.len(),0x2c220000,"signed compare"),
                ] {
                    let mut bad=stub.to_vec();bad[i]=value;assert_eq!(stub_displacement(&bad),None,"{why} in {stub:x?}");
                }
            }
            for (i,value,why) in [(1,0x4ce20421,"linking conditional branch"),(2,0x480000b9,"linking resolver branch"),
                (2,0x480000ba,"absolute resolver branch")] {
                let mut bad=lazy.clone();bad[body.len()+i]=value;assert_eq!(stub_displacement(&bad),None,"{why} in {lazy:x?}");
            }
        }
        for (value,why) in [(0xe96b8130,"environment at +24"),(0xe96b8120,"environment at +8"),
            (0xe98b8128,"environment in r12"),(0xe9628128,"environment through the new TOC"),(0xe96c8128,"environment from r12")] {
            for ending in [&[0x4e800420][..],&TAIL[..]] {
                let mut bad=far_env.to_vec();bad[5]=value;bad.extend(ending);
                assert_eq!(stub_displacement(&bad),None,"{why}");
            }
        }
        let twice=[&far_env[..],&[0xe96b8128,0x4e800420][..]].concat();
        assert_eq!(stub_displacement(&twice),None,"second environment load");
        let same_register=[0xf8410028,0x3d820001,0xe98c8118,0x7d8903a6,0xe84c8120,0x4e800420];
        assert_eq!(stub_displacement(&same_register),None,"entry loaded over its own base");
        let other_base=[0xf8410028,0x3d220001,0xe9898118,0x7d8903a6,0xe8498120,0x4e800420];
        assert_eq!(stub_displacement(&other_base),None,"base outside r11 and r12");
        let late_wrong=[0xf8410028,0xe9628118,0xe8428120,0x7d8903a6,0x4e800420];
        assert_eq!(stub_displacement(&late_wrong),None,"late CTR move from another register");
        for (entry,ctr,why) in [(0xe8628118,0x7c6903a6,"entry through r3"),(0xe8028118,0x7c0903a6,"entry through r0")] {
            for ending in [&[0x4e800420][..],&TAIL[..]] {
                let stub=[&[0xf8410028,entry,ctr,0xe8428120][..],ending].concat();
                assert_eq!(stub_displacement(&stub),None,"{why}");
            }
        }
        for (stub,why) in [
            (&[0xf8410028u32,0x3d620002,0xe98b7ff8,0x396b7ff8,0x7d8903a6,0xe84b0008,0xe96b0010,0x4e800420][..],"addi-adjusted base"),
            (&[0xf8410028,0xe9828118,0x7d8903a6,0x7d8b6278,0x7c425a14,0xe8428120,0x4e800420][..],"xor/add dependency"),
        ] {assert_eq!(stub_displacement(stub),None,"{why}");}
    }
}
