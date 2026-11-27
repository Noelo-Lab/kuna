//! (kuna) Follow an AArch64 jump table in the Listing walk, so the case bodies
//! of a switch belong to the function that dispatches to them.
//!
//! clang and gcc lower an AArch64 switch to a compact table of byte, halfword
//! or word offsets in `__const`/`.rodata`, scaled to instructions and added to
//! an `adr` base right after the range check:
//!
//! ```text
//! cmp    w10, #0x46
//! b.hi   default
//! adrp   x11, table@PAGE
//! add    x11, x11, table@PAGEOFF
//! adr    x12, cases
//! ldrh   w13, [x11, x10, lsl #1]
//! add    x12, x12, x13, lsl #2
//! br     x12
//! ```
//!
//! The walk gives a computed jump no successor, so every case body stays
//! undefined, and the aggressive instruction finder, whose fingerprints an
//! AArch64 image's case bodies share, cuts the dispatcher into functions: a
//! 4.5 KB re2c scanner in cmark-gfm became 104 bytes and five `sub_`s.
//!
//! [`A64Tables::targets`] reads the dispatch the way the xrefs walk does, with
//! [`super::kuna_switchtable::register_targets`] over the instructions the
//! walk has decoded ahead of it, bounded by the range check or, for a table
//! with none (Swift's enum dispatch), by the next address the function's code
//! forms ([`super::kuna_switchextent`]). The function's extent for that bound
//! runs between the seeds around the dispatch.

use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_num::opcodes::OpCode;
use kuna_sleigh::translate::Translate;

use super::kuna_poolref::PoolImage;
use super::kuna_switchextent::FunctionTables;
use super::kuna_switchtable::{preserved_registers, register_targets, Reach, TableRead, DEFAULT_MAX_ENTRIES};
use super::xrefs::{decode, in_range, FullCapture};

/// Whether AArch64 tables are read in `file`: a linked AArch64 image.
pub(crate) fn applies(file: &object::File<'_>) -> bool {
    use object::Object;
    file.architecture() == object::Architecture::Aarch64 && file.kind() != object::ObjectKind::Relocatable
}

/// The read-only image an AArch64 table is read from, and the seeds that
/// bound a function's extent.
pub(crate) struct A64Tables<'a> {
    pool: &'a PoolImage<'a>,
    data_space: Option<Rc<AddrSpace>>,
    preserved: Vec<(u64, u64)>,
    max_entries: usize,
    entries: Vec<u64>,
    tables: FunctionTables<'a>,
    extent: bool,
}

impl<'a> A64Tables<'a> {
    /// The reader over the walk's read-only image `pool`, for an image where
    /// [`applies`] holds; `seeds` are the function entries the walk starts from.
    pub(crate) fn new(arch: &Architecture, pool: &'a PoolImage<'a>, seeds: &[u64]) -> Self {
        let max_entries = match arch.max_jumptable_size as usize {
            0 => DEFAULT_MAX_ENTRIES,
            n => n,
        };
        let mut entries = seeds.to_vec();
        entries.sort_unstable();
        entries.dedup();
        A64Tables {
            pool,
            data_space: arch.manage().get_default_data_space().cloned(),
            preserved: preserved_registers(arch),
            max_entries,
            entries,
            tables: FunctionTables::new(pool),
            extent: arch.switch_extent,
        }
    }

    /// The case bodies of the computed jump at `from`, and the case count its
    /// range check or its extent states; nothing for a jump that is not a
    /// table dispatch or whose table could not be bounded. Every aligned word
    /// of the executable ranges counts as decoded: AArch64 instructions are
    /// four bytes, so the code above a dispatch reads the same whether the
    /// walk reached it yet or not, and a range check in a block the walk has
    /// not visited is still found.
    pub(super) fn targets(
        &self,
        translate: &dyn Translate,
        code_space: &Rc<AddrSpace>,
        exec: &[(u64, u64)],
        from: u64,
    ) -> TableRead {
        let decoded = |at: u64| at % 4 == 0 && in_range(exec, at);
        let mut cap = FullCapture::default();
        if decode(translate, from, code_space, &mut cap).is_none() {
            return TableRead::default();
        }
        let Some(branch) = cap
            .ops()
            .iter()
            .find(|op| op.opcode == OpCode::CPUI_BRANCHIND)
            .and_then(|op| op.ins.first().cloned())
        else {
            return TableRead::default();
        };
        let next_table = |from: u64, table: u64| {
            let at = self.entries.partition_point(|&e| e <= from);
            let lo = *self.entries.get(at.checked_sub(1)?)?;
            let hi = self.entries.get(at).copied();
            self.tables.next_after(translate, code_space, self.data_space.as_ref(), exec, lo, hi, table)
        };
        let reach = Reach {
            max_entries: self.max_entries,
            preserved: &self.preserved,
            settled: false,
            named: None,
            next_table: self.extent.then_some(&next_table),
            entered: None,
        };
        let read = register_targets(
            translate, code_space, self.data_space.as_ref(), &decoded, from, &branch, self.pool, exec, &reach,
        );
        if read.unbounded {
            return TableRead::default();
        }
        read
    }
}
