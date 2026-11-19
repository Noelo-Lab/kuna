//! (kuna `iokitclasses`) DriverKit class records.
//!
//! A DriverKit extension lists every class it defines in `__osclassinfo`: an
//! array of pointers to `OSClassLoadInformation` records (0xa0 bytes:
//! `description`, `metaPointer`, `version`, `instanceSize`, eight reserved
//! words, `New`, eight more), each pointing at an `OSClassDescription` (0x138
//! bytes and the variable tables behind it: `descriptionSize`, `name[96]`,
//! `superName[96]`, then size/offset pairs of the method-option, queue-name and
//! method-name tables). The names are exact, so no code needs reading.

use object::read::{Object, ObjectSection};

use crate::loader::format::macho::resolve_chained_fixups;
use crate::objc::sections::MachoImage;
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, DataObjectFact, FidMatch, Phase, SymFact, SymKind};

const LOAD_INFO_SIZE: u32 = 0xa0;
const DESCRIPTION_SIZE: u32 = 0x138;
const NAME_LEN: u64 = 96;
const NEW_OFFSET: u64 = 0x58;

#[derive(Default)]
pub struct OsClassInfoPass;

/// One `__osclassinfo` class.
#[derive(Debug, PartialEq, Eq)]
pub struct DextClass {
    pub name: String,
    pub super_name: String,
    pub load_info: u64,
    pub description: u64,
    pub description_size: u32,
    pub meta_pointer: u64,
    pub instance_size: u32,
    pub new: u64,
}

impl AnalysisPass for OsClassInfoPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "osclassinfo"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        if ctx.file.format() != object::BinaryFormat::MachO || ctx.file.section_by_name("__osclassinfo").is_none() {
            return out;
        }
        let img = MachoImage::with_fixups(ctx.file, resolve_chained_fixups(ctx.file, ctx.bytes));
        for c in classes(ctx.file, &img) {
            emit(&c, &mut out);
        }
        out
    }
}

fn identifier(img: &MachoImage, va: u64) -> Option<String> {
    let raw = img.read(va, NAME_LEN as usize)?;
    let s = std::str::from_utf8(&raw[..raw.iter().position(|&b| b == 0)?]).ok()?;
    let ok = s.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    ok.then(|| s.to_string())
}

/// Every valid record the `__osclassinfo` sections point at.
pub fn classes(file: &object::File, img: &MachoImage) -> Vec<DextClass> {
    let mut out = Vec::new();
    for sec in file.sections().filter(|s| s.name() == Ok("__osclassinfo")) {
        for k in 0..sec.size() / 8 {
            let Some(info) = img.read_ptr(sec.address() + 8 * k).filter(|&p| p != 0) else { continue };
            if let Some(c) = class(img, info) {
                out.push(c);
            }
        }
    }
    out
}

fn class(img: &MachoImage, info: u64) -> Option<DextClass> {
    img.read(info, LOAD_INFO_SIZE as usize)?;
    let description = img.read_ptr(info)?;
    let size = img.read_u32(description)?;
    if size < DESCRIPTION_SIZE || img.read(description, size as usize).is_none() {
        return None;
    }
    Some(DextClass {
        name: identifier(img, description + 4)?,
        super_name: identifier(img, description + 4 + NAME_LEN).unwrap_or_default(),
        load_info: info,
        description,
        description_size: size,
        meta_pointer: img.read_ptr(info + 8)?,
        instance_size: img.read_u32(info + 0x14)?,
        new: img.read_ptr(info + NEW_OFFSET)?,
    })
}

fn emit(c: &DextClass, out: &mut AnalysisOutput) {
    let data = [
        (c.load_info, format!("{}_Class", c.name), LOAD_INFO_SIZE),
        (c.description, format!("{}_ClassDescription", c.name), c.description_size),
        (c.meta_pointer, format!("g{}MetaClass", c.name), 8),
    ];
    for (addr, name, size) in data {
        if addr != 0 {
            out.data_objects.push(DataObjectFact { addr, name, size });
        }
    }
    if c.new != 0 {
        let name = format!("{}_New", c.name);
        out.entries.push(c.new);
        out.symbols.push(SymFact { addr: c.new, name: name.clone(), kind: SymKind::Function });
        out.fid_names.push(FidMatch { addr: c.new, name, by_signature: false });
    }
}
