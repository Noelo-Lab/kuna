//! Identify the Go scheduler entry whose caller can resume on AMD64 ELF.

use object::{Architecture, BinaryFormat, Object};

use crate::listing::Listing;
use crate::pass::AnalysisCtx;

use super::{exec_end, recover_go_table};

pub(crate) fn resuming_entry(ctx: &AnalysisCtx<'_>, listing: &Listing) -> Option<u64> {
    if !ctx.arch.source_lang_detected.is_go()
        || ctx.file.format() != BinaryFormat::Elf
        || ctx.file.architecture() != Architecture::X86_64
    {
        return None;
    }
    let (functions, _) = recover_go_table(ctx.file);
    let mut entries = functions.iter().filter(|function| function.name == "runtime.mcall");
    let entry = entries.next()?.addr;
    if entries.next().is_some()
        || functions
            .iter()
            .filter(|function| function.addr == entry)
            .count()
            != 1
    {
        return None;
    }
    exec_end(listing.exec_ranges(), entry)?;
    Some(entry)
}
