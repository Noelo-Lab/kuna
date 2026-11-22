//! Bind a Go stack-input convention when complete linker metadata excludes stack results.

use object::{BinaryFormat, Object, ObjectSection, ObjectSymbol, SectionKind, SymbolKind};

use super::{End, GoFunc, PcHeader, MAGIC_GO_1_18, MAGIC_GO_1_20};
use crate::pass::{AnalysisCtx, LeafStackInputs, ModelFact};

const ARG_INFO: usize = 5;
const MAX_ARGUMENTS: u32 = 10;
const WORD_BYTES: u32 = 8;
const NORMAL_FUNC_ID: u8 = 0;
const ASSEMBLY_FLAG: u8 = 4;

fn u32_at(bytes: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from_le_bytes(
        bytes.get(offset..)?.get(..4)?.try_into().ok()?,
    ))
}

fn symbol_bytes<'a>(file: &object::File<'a>, names: &[&str]) -> Option<(u64, &'a [u8])> {
    let mut symbols = file.symbols().filter(|symbol| {
        symbol.is_definition()
            && symbol.kind() == SymbolKind::Data
            && symbol.name().ok().is_some_and(|name| names.contains(&name))
    });
    let symbol = symbols.next()?;
    if symbols.next().is_some() {
        return None;
    }
    let section = file.section_by_index(symbol.section_index()?).ok()?;
    let offset = usize::try_from(symbol.address().checked_sub(section.address())?).ok()?;
    let bytes = section.data().ok()?.get(offset..)?;
    let bytes = if symbol.size() == 0 {
        bytes
    } else {
        bytes.get(..usize::try_from(symbol.size()).ok()?)?
    };
    Some((symbol.address(), bytes))
}

struct Metadata<'a> {
    functab: &'a [u8],
    functions: usize,
    record_size: usize,
    gofunc: &'a [u8],
}

impl<'a> Metadata<'a> {
    fn load(file: &object::File<'a>, header: &PcHeader, header_vma: u64) -> Option<Self> {
        if !header.ver.is_118plus() || header.end != End::Little || header.ptr_size != 8 {
            return None;
        }
        let section = file
            .section_by_name(".gopclntab")
            .or_else(|| file.section_by_name(".data.rel.ro.gopclntab"))?;
        let data = section.data().ok()?;
        let header_offset = usize::try_from(header_vma.checked_sub(section.address())?).ok()?;
        let (record_size, gofunc_symbols, gofunc_words): (usize, &[&str], &[usize]) =
            match u32_at(data, header_offset)? {
                MAGIC_GO_1_18 => (40, &["go.func.*"], &[38]),
                MAGIC_GO_1_20 => (44, &["go:func.*"], &[38, 41, 43]),
                _ => return None,
            };
        let (gofunc_address, gofunc) = symbol_bytes(file, gofunc_symbols)?;
        let (_, module) = symbol_bytes(file, &["runtime.firstmoduledata"])?;
        let word = |i: usize| {
            Some(u64::from_le_bytes(
                module.get(i * 8..i * 8 + 8)?.try_into().ok()?,
            ))
        };
        if gofunc_address == 0
            || word(0)? != header_vma
            || word(1)? != header.funcname_vma
            || word(13)? != header.funcdata_base
            || word(16)? != header.functab_vma
            || word(17)? != header.nfunc.checked_add(1)?
            || word(22)? != header.text_start
            || gofunc_words
                .iter()
                .filter(|&&i| word(i) == Some(gofunc_address))
                .count()
                != 1
        {
            return None;
        }
        let functions = usize::try_from(header.nfunc).ok()?;
        let table_offset =
            usize::try_from(header.functab_vma.checked_sub(section.address())?).ok()?;
        let table_end = gofunc_address
            .checked_sub(section.address())
            .and_then(|offset| usize::try_from(offset).ok())
            .filter(|&offset| offset > table_offset && offset < data.len())
            .unwrap_or(data.len());
        let functab = data.get(table_offset..table_end)?;
        functab.get(..functions.checked_mul(8)?.checked_add(4)?)?;
        Some(Self {
            functab,
            functions,
            record_size,
            gofunc,
        })
    }

    fn record(&self, pc: u32) -> Option<(&'a [u8], u32)> {
        let (mut lo, mut hi) = (0, self.functions);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if u32_at(self.functab, mid * 8)? < pc {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        if lo >= self.functions || u32_at(self.functab, lo * 8)? != pc {
            return None;
        }
        // The next record or the end of the table bounds this record's arrays.
        let start = u32_at(self.functab, lo * 8 + 4)? as usize;
        let end = if lo + 1 == self.functions {
            self.functab.len()
        } else {
            u32_at(self.functab, lo * 8 + 12)? as usize
        };
        if start < self.functions * 8 + 4 || start % 4 != 0 || end % 4 != 0 {
            return None;
        }
        let record = self.functab.get(start..end)?;
        let next_pc = u32_at(self.functab, (lo + 1) * 8)?;
        (u32_at(record, 0)? == pc && record.len() >= self.record_size && next_pc > pc)
            .then_some((record, next_pc))
    }

    fn stack_input(&self, pc: u32) -> Option<(u32, u32)> {
        let (record, next_pc) = self.record(pc)?;
        if record[self.record_size - 4] != NORMAL_FUNC_ID
            || record[self.record_size - 3] & !ASSEMBLY_FLAG != 0
        {
            return None;
        }
        let args = u32_at(record, 8)?;
        let npcdata = u32_at(record, 28)? as usize;
        let nfuncdata = record[self.record_size - 1] as usize;
        let offset = arg_info_offset(self.record_size, npcdata, nfuncdata, record)?;
        let bytes = void_input_bytes(self.gofunc.get(offset..)?, args)?;
        Some((bytes, next_pc))
    }
}

fn arg_info_offset(
    record_size: usize,
    npcdata: usize,
    nfuncdata: usize,
    record: &[u8],
) -> Option<usize> {
    let data = record_size.checked_add(npcdata.checked_mul(4)?)?;
    if nfuncdata <= ARG_INFO || data.checked_add(nfuncdata.checked_mul(4)?)? > record.len() {
        return None;
    }
    let offset = u32_at(record, data + ARG_INFO * 4)?;
    (offset != u32::MAX).then_some(offset as usize)
}

fn void_input_bytes(info: &[u8], args: u32) -> Option<u32> {
    if args == 0 || args > MAX_ARGUMENTS * WORD_BYTES || args % WORD_BYTES != 0 {
        return None;
    }
    // ArgInfo describes inputs only. Reject padding, aggregate and truncation markers
    // so exact coverage of _func.args leaves no space for any nonzero stack result.
    let slots = (args / WORD_BYTES) as usize;
    let (pairs, rest) = info.split_at_checked(slots * 2)?;
    if rest.first() != Some(&0xff) {
        return None;
    }
    pairs
        .chunks_exact(2)
        .enumerate()
        .all(|(i, pair)| pair == [(i as u8) * WORD_BYTES as u8, WORD_BYTES as u8])
        .then_some(args)
}

fn text_body<'a>(
    file: &object::File<'a>,
    symbol: &impl ObjectSymbol<'a>,
    limit: u64,
) -> Option<&'a [u8]> {
    let address = symbol.address();
    let size = symbol.size();
    if size == 0 || address.checked_add(size)? > limit {
        return None;
    }
    let section = file.section_by_index(symbol.section_index()?).ok()?;
    if section.kind() != SectionKind::Text {
        return None;
    }
    let offset = usize::try_from(address.checked_sub(section.address())?).ok()?;
    let length = usize::try_from(size).ok()?;
    section
        .data()
        .ok()?
        .get(offset..offset.checked_add(length)?)
}

fn collect(
    file: &object::File<'_>,
    funcs: &[GoFunc],
    header: &PcHeader,
    header_vma: u64,
) -> Vec<ModelFact> {
    let Some(metadata) = Metadata::load(file, header, header_vma) else {
        return Vec::new();
    };
    let mut facts = Vec::new();
    for symbol in file.symbols() {
        if !symbol.is_definition() || symbol.kind() != SymbolKind::Text {
            continue;
        }
        let Some(base) = symbol
            .name()
            .ok()
            .and_then(|name| name.strip_suffix(".abi0"))
        else {
            continue;
        };
        if base.contains(['[', ']']) {
            continue;
        }
        let addr = symbol.address();
        let index = funcs.partition_point(|func| func.addr < addr);
        if funcs
            .get(index)
            .is_none_or(|func| func.addr != addr || func.name != base)
            || funcs.get(index + 1).is_some_and(|next| next.addr == addr)
        {
            continue;
        }
        let Some(pc) = addr
            .checked_sub(header.text_start)
            .and_then(|pc| u32::try_from(pc).ok())
        else {
            continue;
        };
        let Some((bytes, next_pc)) = metadata.stack_input(pc) else {
            continue;
        };
        let Some(limit) = header.text_start.checked_add(u64::from(next_pc)) else {
            continue;
        };
        let Some(body) = text_body(file, &symbol, limit) else {
            continue;
        };
        let Some(inputs) = LeafStackInputs::new(bytes, body) else {
            continue;
        };
        facts.push(ModelFact {
            func_addr: addr,
            model: "abi0",
            maybe_self: false,
            leaf_stack_inputs: Some(inputs),
        });
    }
    facts.sort_unstable_by_key(|fact| fact.func_addr);
    facts.dedup_by_key(|fact| fact.func_addr);
    facts
}

pub(super) fn model_facts(
    ctx: &AnalysisCtx<'_>,
    funcs: &[GoFunc],
    header: &PcHeader,
    header_vma: u64,
) -> Vec<ModelFact> {
    if funcs.is_empty()
        || ctx.file.format() != BinaryFormat::Elf
        || ctx.arch.archid != "x86:LE:64:default:golang"
        || !ctx.arch.has_model("abi0")
        || !kuna_decomp::kuna_goabi::goabi_env_enabled()
    {
        return Vec::new();
    }
    collect(ctx.file, funcs, header, header_vma)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pclntab::{decode_functions, recover_go_header, ImageBytes};

    #[test]
    fn complete_words_fill_the_entire_input_and_output_area() {
        let words = [0, 8, 8, 8, 0xff];
        assert_eq!(void_input_bytes(&words, 16), Some(16));
        assert_eq!(void_input_bytes(&words, 24), None);
        assert_eq!(void_input_bytes(&words, 8), None);
        assert_eq!(void_input_bytes(&words, 0x8000_0000), None);
        assert_eq!(void_input_bytes(&[0xff], 0), None);
        for info in [
            &[0, 4, 8, 8, 0xff][..],
            &[0, 8, 16, 8, 0xff],
            &[0, 8, 0, 8, 0xff],
            &[0, 8, 8, 8],
            &[0, 8, 0xfc, 8, 0xff],
            &[0xfe, 0, 8, 8, 8, 0xfd, 0xff],
        ] {
            assert_eq!(void_input_bytes(info, 16), None, "{info:x?}");
        }
    }

    #[test]
    fn arginfo_must_fit_within_the_same_function_record() {
        let mut record = vec![0; 44 + 4 * (2 + 6)];
        let offset = record.len() - 4;
        record[offset..].copy_from_slice(&0x1122u32.to_le_bytes());
        assert_eq!(arg_info_offset(44, 2, 6, &record), Some(0x1122));
        assert_eq!(arg_info_offset(44, 2, 6, &record[..record.len() - 1]), None);
        assert_eq!(arg_info_offset(44, 2, 5, &record), None);
        assert_eq!(arg_info_offset(44, 2, 7, &record), None);
        assert_eq!(arg_info_offset(44, usize::MAX, 6, &record), None);
        record[offset..].copy_from_slice(&u32::MAX.to_le_bytes());
        assert_eq!(arg_info_offset(44, 2, 6, &record), None);
    }

    #[test]
    fn function_record_must_match_the_table_entry() {
        let mut table = vec![0; 100];
        for (offset, word) in [
            (0, 0x10u32),
            (4, 20),
            (8, 0x20),
            (12, 60),
            (16, 0x30),
            (20, 0x10),
            (60, 0x20),
        ] {
            table[offset..offset + 4].copy_from_slice(&word.to_le_bytes());
        }
        let metadata = Metadata {
            functab: &table,
            functions: 2,
            record_size: 40,
            gofunc: &[],
        };
        let (first, next_pc) = metadata.record(0x10).unwrap();
        assert_eq!((first.len(), next_pc), (40, 0x20));
        let (last, end_pc) = metadata.record(0x20).unwrap();
        assert_eq!((last.len(), end_pc), (40, 0x30));
        table[20] = 0x11;
        let metadata = Metadata {
            functab: &table,
            functions: 2,
            record_size: 40,
            gofunc: &[],
        };
        assert!(metadata.record(0x10).is_none());
    }

    #[test]
    fn linker_metadata_accepts_the_authored_assembly_and_refuses_runtime_overclaims() {
        let old = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../tests/fixtures/go-abi0-model/linux-amd64"
        ));
        let new = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../tests/fixtures/go-pair-results/go1.27.3-pair"
        ));
        for (data, argcatalog, authored) in
            [(old.as_slice(), 40, true), (new.as_slice(), 44, false)]
        {
            let file = object::File::parse(data).unwrap();
            let img = ImageBytes::new(&file);
            let (header, header_vma) = recover_go_header(&file, &img).unwrap();
            let mut funcs = decode_functions(&img, &header);
            funcs.sort_unstable_by_key(|f| f.addr);
            let metadata = Metadata::load(&file, &header, header_vma).unwrap();
            assert_eq!(metadata.record_size, argcatalog);
            let facts = collect(&file, &funcs, &header, header_vma);
            let addr = |name: &str| {
                file.symbols()
                    .find(|s| s.name().ok() == Some(name))
                    .unwrap()
                    .address()
            };
            if authored {
                assert!(facts.iter().any(|f| f.func_addr == addr("main.store.abi0")
                    && f.leaf_stack_inputs
                        .as_ref()
                        .is_some_and(|inputs| inputs.byte_limit == 16)));
                assert!(!facts.iter().any(|f| f.func_addr == addr("main.abi0")));
                assert!(!facts
                    .iter()
                    .any(|f| f.func_addr == addr("runtime.sigtrampgo.abi0")));
            }
            for name in [
                "runtime.write.abi0",
                "runtime.sigtramp.abi0",
                "runtime.gogo.abi0",
            ] {
                assert!(!facts.iter().any(|f| f.func_addr == addr(name)), "{name}");
            }
            if !authored {
                assert!(
                    facts
                        .iter()
                        .any(|f| f.func_addr == addr("runtime.exitThread.abi0")),
                    "exitThread must be refused by the decoded body, independently of the metadata"
                );
            }
        }
    }
}
