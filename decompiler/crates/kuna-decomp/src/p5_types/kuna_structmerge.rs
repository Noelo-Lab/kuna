//! (kuna `structmerge`) Give a synthesized record the fields a sibling reader of
//! the same record proved.
//!
//! # The gap
//!
//! Two functions that take a pointer to the same object rarely read the same
//! part of it. The layout ledger already shares one `struct_N` between them when
//! one reader's claims are a SUBSET of the other's, but not when they merely
//! overlap. Two readers of a `{char *name; long n; int flags; long tail;}` that
//! each skip one of its members measure `{0: char *, 8: long, 0x18: long}` and
//! `{0: char *, 8: long, 0x10: int}`; neither contains the other, so each gets a
//! name of its own and neither declaration is the record -- one has a hole where
//! `flags` is and the other stops before `tail`.
//!
//! Measured on the eight layout builds (`docs/features/structmerge/recall.md`):
//! 23 of the 77 ground-truth records kuna types at all are given more than one
//! `struct_N`, and merging what the readers `structsynth` already accepts prove
//! about one record is worth 1,451 correct fields against today's 817 -- while
//! the opposite error, one `struct_N` standing for two different records, is 1
//! of 136 today.
//!
//! # The rule
//!
//! When nothing minted answers a freshly measured layout `L`, and some minted
//! structure `S` is COMPATIBLE with it, the union of the two is minted instead
//! of `L` alone. The union strictly contains `S`, so `S` is superseded the
//! moment it exists and the convergence sweep
//! (`converge_synthesized_structs`) decompiles `S`'s readers again and moves
//! them onto the union.
//!
//! Compatible means the two layouts cannot be talking about different records:
//!
//! * every offset both claim carries the same field -- width, type and pointee
//!   spelled exactly as [`ledger::FieldKey`] spells them, a field pointing at
//!   its own record included. "No type at all" is its own answer, never a
//!   wildcard the other side may fill in, exactly as the containment rule has
//!   it;
//! * no claim of one covers bytes a claim of the other covers at a different
//!   offset -- two fields over the same bytes are two different records' fields;
//! * what they agree on is itself evidence: three shared claims, or two of which
//!   one is a pointer with a pointee (its pointee is part of the field's
//!   identity, which is what a pair of integer words at 0 and 8 -- `struct stat`,
//!   a `timespec`, a list node -- has none of). A layout claims offset 0 unless
//!   it is a lone field or a `structheadless` record, and neither lowers the
//!   floor: a lone field has one claim to share, and a headless record agrees
//!   on its first claim the way any other agrees on offset 0;
//! * the union ANSWERS FOR both sides under the containment rule itself
//!   ([`ledger::Layout::answers_for`]): the growth bounds, the table rule, the
//!   alignment-padding rule and the untyped-reader rule all apply to the merged
//!   record against each reader separately. A union that would declare a reader
//!   to hold more than twice the fields it measured, or four times its bytes, or
//!   a member in the alignment padding between two of its own, is refused;
//! * the union lays the reader's own members over every byte the reader
//!   dereferenced without claiming ([`ledger::keeps_unclaimed`]), the spelling
//!   guard a container has to pass anyway.
//!
//! The earlier reader `S` is held to the same standard by the sweep rather than
//! here: it decompiles again, measures its own layout and its own unclaimed
//! bytes, and the ledger lookup applies both rules with the union present. Where
//! the union does not fit it, nothing answers, `S` still exists, and it keeps
//! the structure it had.
//!
//! # Limits
//!
//! Agreement is a shape, not an identity, so the rule fuses records it cannot
//! tell apart. At the floor -- two shared claims, one of them a pointer -- the
//! agreement is `{0: char *, 8: long}`, which is how a large share of C records
//! begin: two functions reading two DIFFERENT records of that shape are merged,
//! and each then declares a field its own object does not have. The witness is
//! the same claim set either way (`tests.rs`
//! `two_records_that_begin_alike_are_fused_at_the_agreement_floor` builds it out
//! of a `Job` and a `Conn`, the one above out of one record), and 19 of the 63
//! merges over sixteen coreutils/grep/diffutils/gzip/findutils builds clear the
//! bar at exactly that floor (38 clear it at three claims, 6 at four).
//! `docs/features/structmerge/default-on-evaluation.md` prices the class: 189 of
//! the 1,105 fields the merge adds over 177 builds are not DWARF fields, pooled
//! precision 0.9330 -> 0.9312. That is why the option ships `off`.
//!
//! The second limit is the sweep. The union supersedes the thinner record it
//! contains, so that record's readers decompile again -- and the containment
//! bounds that keep a reader of two fields off a record of five put the union
//! out of its reach, so a redo can settle on a weaker answer than the one it
//! replaces. On e2fsprogs `e2fsck` -O0 three variables fall from a synthesized
//! record to `void *` and two `char *` parameters become `unsigned long`, while
//! one function gains a five-field record;
//! `docs/features/structmerge/hunks.md` works the class, and
//! `--option protoorder lock`, which turns the sweep off, leaves not one
//! variable on that binary changing type. A union the factory will not complete
//! is not part of it: [`ledger::lookup_or_mint`] falls through to the reader's
//! own claims, exactly as `off` mints them.
//!
//! Only the live ledger merges. A `--jobs N` worker answers through the shard
//! table, whose replay records the layout each request measured, so a mint of
//! something neither side asked for has nothing to replay; a sharded run
//! therefore behaves as `off`, the way it already diverges over `structsynth`.
//! The sweep runs one pass, so a union that supersedes a structure whose readers
//! would then union again converges no further than `structsynth`'s own
//! non-transitivity already does.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::int4;

use crate::dtype::{type_metatype, Datatype, TypeFactory, TypeField};
use crate::p5_types::kuna_structsynth::ledger::{self, Layout, Member};

/// How many claims two layouts must share before the union is evidence, when one
/// of the shared claims is a pointer with a pointee.
const MIN_SHARED_CLAIMS: usize = 2;

/// How many claims two layouts must share when none of them is such a pointer.
const MIN_SHARED_UNTYPED: usize = 3;

/// (kuna) Whether a synthesized record takes in the claims of a sibling reader.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StructMergeMode {
    /// Upstream: a record carries only what one reader's claims contain.
    #[default]
    Off,
    /// A record carries the claims of every compatible reader of the same record.
    Siblings,
}

impl StructMergeMode {
    /// Does the rule act at all?
    pub fn fires(self) -> bool {
        self != StructMergeMode::Off
    }
}

/// (kuna) Parse `option structmerge off|siblings`; the caller writes the live field.
pub struct OptionStructMerge;

impl OptionStructMerge {
    /// The option name.
    pub const NAME: &'static str = "structmerge";

    /// Parse + validate the value.
    pub fn apply(&self, p1: &str) -> KunaResult<(StructMergeMode, String)> {
        let mode = match p1 {
            "off" => StructMergeMode::Off,
            "siblings" => StructMergeMode::Siblings,
            other => {
                return Err(KunaError::parse(format!(
                    "Unknown structmerge value: {other} (expected off|siblings)"
                )))
            }
        };
        Ok((mode, format!("Sibling layout merge set to {p1}")))
    }
}

/// The claimed members of a minted structure by offset, its self-pointer offsets
/// and its size. Filler is array-typed and is not a claim.
fn claims_of(ct: &Datatype) -> Option<(BTreeMap<int4, Rc<Datatype>>, Vec<int4>, int4)> {
    let mut claims = BTreeMap::new();
    let mut selfs = Vec::new();
    for i in 0..ct.num_depend() {
        let f = ct.get_field(i)?;
        if f.field_type.get_metatype() == type_metatype::TYPE_ARRAY {
            continue;
        }
        if f.field_type.get_ptr_to().is_some_and(|p| {
            p.get_metatype() == type_metatype::TYPE_STRUCT && p.get_name() == ct.get_name()
        }) {
            selfs.push(f.offset);
        }
        claims.insert(f.offset, Rc::clone(&f.field_type));
    }
    Some((claims, selfs, ct.get_size()))
}

/// The claimed members of a field list the pass has just built.
fn claims_of_fields(fields: &[TypeField]) -> BTreeMap<int4, Rc<Datatype>> {
    fields
        .iter()
        .filter(|f| f.field_type.get_metatype() != type_metatype::TYPE_ARRAY)
        .map(|f| (f.offset, Rc::clone(&f.field_type)))
        .collect()
}

/// How many claims two layouts share, or `None` when one contradicts the other:
/// a different field at the same offset, or two fields over the same bytes.
fn shared_claims(a: &Layout, b: &Layout) -> Option<usize> {
    let mut shared = 0usize;
    for ka in a.fields.iter() {
        for kb in b.fields.iter() {
            if ka.offset == kb.offset {
                if ka != kb {
                    return None;
                }
                shared += 1;
            } else if ka.offset < kb.offset.saturating_add(kb.width)
                && kb.offset < ka.offset.saturating_add(ka.width)
            {
                return None;
            }
        }
    }
    Some(shared)
}

/// Is the agreement between two layouts evidence that they read one record?
fn agreement_is_evidence(a: &Layout, b: &Layout, shared: usize) -> bool {
    let typed = a.fields.iter().any(|ka| {
        ka.pointer && b.fields.iter().any(|kb| kb.offset == ka.offset && kb == ka)
    });
    shared >= MIN_SHARED_UNTYPED || (shared >= MIN_SHARED_CLAIMS && typed)
}

/// The member list of a record holding exactly `claims`, with every gap and the
/// tail covered by `undefined1` filler and the size rounded the way a C compiler
/// rounds it -- the same layout [`crate::kuna_structsynth`] builds for one
/// reader, over two readers' claims.
fn plan(
    types: &dyn TypeFactory,
    claims: &BTreeMap<int4, Rc<Datatype>>,
    min_size: int4,
) -> Option<(Vec<TypeField>, int4)> {
    let byte = types.get_base(1, type_metatype::TYPE_UNKNOWN).ok()?;
    let mut out: Vec<(int4, int4, Option<Rc<Datatype>>)> = Vec::new();
    let mut next: int4 = 0;
    let mut align: int4 = 1;
    for (off, ct) in claims.iter() {
        let off = *off;
        if off < next {
            return None;
        }
        if off > next {
            out.push((next, off - next, None));
        }
        let width = ct.get_size();
        out.push((off, width, Some(Rc::clone(ct))));
        align = align.max(width);
        next = off.checked_add(width)?;
    }
    let mut size = next.max(min_size);
    size = size.checked_add(align - 1)? / align * align;
    if size > next {
        out.push((next, size - next, None));
    }
    let mut fields = Vec::with_capacity(out.len());
    for (i, (off, width, ct)) in out.into_iter().enumerate() {
        let ct = match ct {
            Some(t) => t,
            None => types.get_type_array(width, Rc::clone(&byte)).ok()?,
        };
        fields.push(TypeField::new(i as int4, off, format!("field_0x{off:x}"), ct));
    }
    Some((fields, size))
}

/// The union of a freshly measured layout with the minted structure it shares a
/// record with, or `None` when no held structure qualifies.
///
/// `candidates` are the minted structures in the size window a merge could
/// reach, in the order the ledger probes them. The partner is the one sharing
/// the most claims, then the smallest, then the first minted -- a total order,
/// so the answer does not depend on how the factory happens to store them.
pub(crate) fn merge(
    types: &dyn TypeFactory,
    candidates: &[(Rc<Datatype>, Layout)],
    want: &Layout,
    fields: &[TypeField],
    size: int4,
    selfs: &[int4],
    unclaimed: &[(int4, int4)],
) -> Option<(Vec<TypeField>, int4, Vec<int4>)> {
    let own: Vec<Member> = fields.iter().map(|f| Member::of_with(f, selfs)).collect();
    let mine = claims_of_fields(fields);
    let mut best: Option<((usize, std::cmp::Reverse<(int4, usize)>), Vec<TypeField>, int4, Vec<int4>)> =
        None;
    for (rank, (ct, layout)) in candidates.iter().enumerate() {
        let Some(shared) = shared_claims(layout, want) else {
            trace(ct, want, 0, "conflict");
            continue;
        };
        if !agreement_is_evidence(layout, want, shared) {
            trace(ct, want, shared, "agreement-too-weak");
            continue;
        }
        let Some((theirs, their_selfs, their_size)) = claims_of(ct) else { continue };
        let mut claims = mine.clone();
        for (off, ty) in theirs {
            claims.entry(off).or_insert(ty);
        }
        let mut merged_selfs = their_selfs;
        merged_selfs.extend(selfs.iter().copied());
        merged_selfs.sort_unstable();
        merged_selfs.dedup();
        let Some((mfields, msize)) = plan(types, &claims, size.max(their_size)) else {
            trace(ct, want, shared, "overlapping-claims");
            continue;
        };
        let merged = ledger::layout_of_fields(&mfields, msize, &merged_selfs);
        if !merged.answers_for(want) {
            trace(ct, want, shared, "union-too-far-for-the-reader");
            continue;
        }
        if !merged.answers_for(layout) {
            trace(ct, want, shared, "union-too-far-for-the-held");
            continue;
        }
        let members: Vec<Member> =
            mfields.iter().map(|f| Member::of_with(f, &merged_selfs)).collect();
        if !unclaimed.is_empty() && !ledger::keeps_unclaimed(&members, &own, unclaimed) {
            trace(ct, want, shared, "unclaimed-bytes");
            continue;
        }
        trace(ct, want, shared, "merged");
        let key = (shared, std::cmp::Reverse((msize, rank)));
        if best.as_ref().is_none_or(|(k, ..)| key > *k) {
            best = Some((key, mfields, msize, merged_selfs));
        }
    }
    best.map(|(_, f, s, sel)| (f, s, sel))
}

/// The union was accepted and the factory still refused to complete it, so the
/// reader falls back to a record of its own claims. One `structmerge:` line on
/// stderr under `KUNA_STRUCTMERGE_TRACE=1`.
pub(crate) fn trace_mint_failed(name: &str, want: &Layout) {
    if std::env::var_os("KUNA_STRUCTMERGE_TRACE").is_none() {
        return;
    }
    eprintln!(
        "structmerge: the union for a layout of {} claims and {} bytes would not mint as {name} \
         -- falling back to the reader's own claims",
        want.fields.len(),
        want.size
    );
}

/// One `structmerge:` line per candidate on stderr under
/// `KUNA_STRUCTMERGE_TRACE=1`: which held structure was considered for a
/// freshly measured layout, how many claims the two agree on, and what came of
/// it. Silent, and free, when the variable is not set.
fn trace(ct: &Datatype, want: &Layout, shared: usize, verdict: &str) {
    if std::env::var_os("KUNA_STRUCTMERGE_TRACE").is_none() {
        return;
    }
    eprintln!(
        "structmerge: {} ({} claims, {} bytes) vs a layout of {} claims and {} bytes: \
         {shared} shared -- {verdict}",
        ct.get_name(),
        ledger::layout_of(ct).map(|l| l.fields.len()).unwrap_or(0),
        ct.get_size(),
        want.fields.len(),
        want.size
    );
}

#[cfg(test)]
mod tests;
