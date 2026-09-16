//! ELFv1 descriptor PLT stubs, correlated with JMP_SLOT relocations and TOCs.
use object::{Object, ObjectSection, ObjectSymbol, RelocationFlags, RelocationTarget, SectionFlags};
use std::collections::{BTreeSet, HashMap};
use super::{elf_plt::PltSym, elfv1::Descriptors};

pub(super) fn resolve(file: &object::File<'_>) -> Vec<PltSym> {
    let descriptors=Descriptors::read_for_imports(file);
    if descriptors.0.is_empty() {return Vec::new();}
    let Some(tocs): Option<BTreeSet<_>>=descriptors.0.iter().map(|d|d.toc).collect() else {return Vec::new();};
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
    if slots.is_empty() {return Vec::new();}
    let lazy=LazyResolver::read(file);
    let mut out=Vec::new();
    for section in file.sections() {
        if !matches!(section.flags(),SectionFlags::Elf {sh_flags} if sh_flags & 6 == 6) {continue;}
        let Ok(bytes)=section.data() else {continue;};
        for off in (0..bytes.len()).step_by(4) {
            let Some(first)=bytes.get(off..off+4) else {break;};
            let first=first.try_into().unwrap();
            let first=if file.is_little_endian() {u32::from_le_bytes(first)} else {u32::from_be_bytes(first)};
            if first!=0xf8410028 {continue;}
            let words=words(file,&bytes[off..bytes.len().min(off+36)]);
            let Some((delta,branch))=stub_displacement(&words) else {continue;};
            let addr=section.address().wrapping_add(off as u64);
            if let Some(branch)=branch {
                let Some(slot)=lazy.as_ref().and_then(|r|r.slot(file,addr.wrapping_add(branch as u64))) else {continue;};
                if !tocs.iter().all(|toc|toc.wrapping_add(delta as u64)==slot) {continue;}
            }
            if let Some(name)=unique_import(delta,&tocs,&slots) {out.push(PltSym {addr,name});}
        }
    }
    out
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
fn stub_displacement(w: &[u32]) -> Option<(i64,Option<i64>)> {
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
    if ld(*w.get(i)?,2,base)!=Some(low+8) {return None;}i+=1;
    if !early_ctr {
        if *w.get(i)?!=mtctr(target) {return None;}i+=1;
    }
    if base!=2 && ld(*w.get(i)?,11,base)==Some(low+16) {
        i+=1;
        return (*w.get(i)?==0x4e800420).then_some((high+low,None));
    }
    if *w.get(i)?!=0x28220000 {return None;}i+=1;
    if *w.get(i)? & !0x00600000 != 0x4c820420 {return None;}i+=1;
    Some((high+low,Some(i as i64*4+branch_displacement(*w.get(i)?)?)))
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
}
