//! (kuna `fid`) The prototype of a `.fid` record, at build and at match.
//!
//! At build, [`describe`] gives each record the prototype the object's own
//! committed analysis parked on its function (its DWARF signature), as the C
//! text a record stores ([`kuna_decomp::kuna_fidproto`]). At match, [`pick`]
//! chooses the prototype of a bucket whose records all carry one name: the
//! record the specific hash and code-unit count single out, else the one
//! prototype every describing record agrees on.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_decomp::architecture::Architecture;

use super::db::FidRecord;
use crate::loadimage_object::ObjectLoadImage;
use super::hash::FidHashQuad;

/// Give each of `records` the prototype parked on the function symbol of its
/// name; a name several of `image`'s functions share takes none. Returns how
/// many records were described.
pub fn describe(arch: &Architecture, image: &ObjectLoadImage, records: &mut [FidRecord]) -> usize {
    let Some(space) = arch.manage().get_default_code_space() else { return 0 };
    let arm32 = arch.archid.starts_with("ARM:") && arch.archid.contains(":32:");
    let mut entries: BTreeMap<String, Option<u64>> = BTreeMap::new();
    for (addr, name) in image.func_symbols() {
        let addr = if arm32 { addr & !1 } else { addr };
        entries
            .entry(name)
            .and_modify(|seen| *seen = seen.filter(|a| *a == addr))
            .or_insert(Some(addr));
    }
    let mut described = 0;
    for record in records {
        let Some(Some(entry)) = entries.get(&record.name) else { continue };
        let addr = Address::new(Rc::clone(space), *entry);
        let declaration = arch
            .symboltab
            .function_proto_pieces_across_scopes(&addr)
            .and_then(kuna_decomp::kuna_fidproto::declaration);
        if let Some(declaration) = declaration {
            record.prototype = Some(declaration.prototype);
            record.typedefs = declaration.typedefs;
            described += 1;
        }
    }
    described
}

/// The record whose prototype a match on `records` applies.
pub(super) fn pick<'a>(records: &'a [FidRecord], query: &FidHashQuad) -> Option<&'a FidRecord> {
    let mut described = records.iter().filter(|r| r.prototype.is_some());
    let first = described.next()?;
    let exact = |r: &&FidRecord| {
        r.specific_hash == query.specific_hash && r.code_unit_size == query.code_unit_size
    };
    if let Some(record) = std::iter::once(first).chain(described.clone()).find(exact) {
        return Some(record);
    }
    described.all(|r| r.prototype == first.prototype && r.typedefs == first.typedefs).then_some(first)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(specific: u64, prototype: Option<&str>) -> FidRecord {
        FidRecord { full_hash: 42, specific_hash: specific, code_unit_size: 5, specific_addl: 0,
            flags: 0, name: "f".into(), prototype: prototype.map(str::to_owned), typedefs: Vec::new() }
    }

    #[test]
    fn the_exact_record_wins_and_a_disagreement_applies_nothing() {
        let query = FidHashQuad { full_hash: 42, specific_hash: 9, code_unit_size: 5, specific_addl_size: 0 };
        assert!(pick(&[record(9, None)], &query).is_none());
        let agree = [record(1, Some("int f(void)")), record(2, None), record(3, Some("int f(void)"))];
        assert_eq!(pick(&agree, &query).unwrap().specific_hash, 1);
        let differ = [record(1, Some("int f(void)")), record(3, Some("int f(int a)"))];
        assert!(pick(&differ, &query).is_none());
        let exact = [record(1, Some("int f(void)")), record(9, Some("int f(int a)"))];
        assert_eq!(pick(&exact, &query).unwrap().specific_hash, 9);
    }
}
