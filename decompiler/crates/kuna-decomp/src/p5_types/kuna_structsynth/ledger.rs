//! (kuna `structsynth`) The program-wide layout ledger: which `struct_N` a
//! freshly measured layout should be, given the ones already minted.
//!
//! # The gap
//!
//! Two functions that read the same object rarely read all of it. `ls`'s two
//! `fileinfo` comparators are the same record twice: `sub_82b0` claims
//! `{0: char *, 0x48: long}` and `sub_a660` claims those two plus
//! `{0xa8: int, 0xac: uint}`. Comparing layouts for equality -- which is all the
//! first version of the pass did -- gives them a name each, and the reader has
//! two names for one type. With the rule below, `fmt`/`ls`/`sort`/`du`/`find`/`tar`
//! at O0 and O2 name 472 records where equality named 516.
//!
//! # The rule
//!
//! A layout `L` is SUBSUMED by a minted structure `S` when every field `L`
//! claims exists in `S` at the same offset, at the same width and with the same
//! type, and `S` is at least as large. `L` then reuses `S`: the larger record is
//! the better answer for both readers, and every offset `L` measured still lands
//! on the member it measured. Agreement is exact -- a field one reader could not
//! type is not a wildcard the other may fill in -- because a wildcard unifies
//! records that merely share a shape.
//!
//! Containment alone is not enough, because it stops being evidence once the
//! container dwarfs what was measured, or once what was measured is too
//! generic to name a record. So a container that says strictly more than the
//! reader measured must also claim at most [`MAX_CLAIM_GROWTH`] times as many
//! fields and be at most [`MAX_SIZE_GROWTH`] times as large, and the reader must
//! claim at least [`MIN_SHARED_CLAIMS`] fields of which one is a typed pointer,
//! or at least [`MIN_UNTYPED_CLAIMS`] without one: answering a reader that
//! measured two words of a `hash_entry` with a 200-byte record declares 174
//! bytes of members nothing measured, and two integer words are how
//! `struct stat`, gnulib's `cycle_check_state` and a list node all begin. The
//! reader is then never declared to have more than twice as many fields as it
//! measured. Two layouts of exactly the same shape always share a name, whatever
//! the bounds say.
//!
//! A pointer is evidence only through its pointee. `code *`, `void *` and
//! `undefined8 *` say nothing about which record holds them, and a layout whose
//! every claim past offset 0 is such a pointer is a table of slots: an
//! interface, an ops vector, a vtable. Distinct tables differ only in how many
//! slots they have, so every table reader is answered by its own shape. Without
//! that, `rsyslogd`'s seven `*_if_s` interface records, 80 to 136 bytes each, all
//! took the 144-byte layout of `statsobj_if_s`.
//!
//! A reader with no typed pointer claim is answered only by a structure of its
//! own size. What it measured says nothing about what follows its last field,
//! and records that begin alike part ways exactly there: every netlink reader in
//! `ip` measures the same `nlmsghdr` words ahead of a payload of its own, and
//! dpkg's 24-byte `pkg_queue` begins the way a 40-byte command record does. A
//! structure of the same size can still fill in the holes of what it measured.
//!
//! When the containment runs the other way -- the new layout strictly contains
//! one already minted -- the minted one cannot be widened, because
//! `TypeFactory::find_add` refuses a second, different definition of a name it
//! already holds and a completed structure is shared by every function that
//! took a pointer to it. So the new layout is minted under a fresh name and the
//! older one is SUPERSEDED: still defined, and handed out again only to a
//! layout no live structure answers for.
//!
//! Supersession is not stored anywhere. It is a property of the minted set --
//! `S` is superseded exactly when some other minted structure strictly subsumes
//! it -- so it is read from the factory on every lookup, for the entries that
//! answer, and cannot drift out of step with the types that are actually
//! defined.
//!
//! # Unclaimed bytes
//!
//! A reader can dereference bytes it claims no field for: the prune drops an
//! unaligned word, a 16-byte copy or a width the header cannot spell, and its
//! address then prints through whatever member holds its first byte. The
//! engine decides whether a load may print after a store by comparing the two
//! addresses, and it takes the same base plus two different constants to be two
//! different objects without asking how wide either access is. A container
//! that puts a member boundary inside such an access can therefore move a load
//! past a store that overwrites part of it. `rd_c8` loads four bytes at 7 and
//! stores one at 8. Its own layout spells the load `*(uint4 *)&a0->field_0x4[3]`,
//! which the comparison cannot tell apart from `a0->field_0x8`, so the load
//! stays first. Its writer's structure has a `char` at 7, and the same load
//! became `*(uint4 *)&a0->field_0x7`, printed after the store. So a structure
//! answers for a reader only if it lays exactly the reader's own members --
//! offsets, widths, types, filler -- over every byte range the reader accessed
//! without claiming. Every unclaimed access is then spelled as it is under the
//! reader's own layout. A reader turned away this way mints its own shape even
//! when a live structure contains it, so that entry can be superseded from the
//! moment it exists.
//!
//! # Order
//!
//! `decompile-all` visits functions in address order, so the ledger's shape
//! depends on that order: the smaller layout at the lower address is minted
//! first and superseded when the larger one arrives, while the reverse order
//! mints only the larger. [`superseded_names`] is what lets a whole-program
//! driver close that gap by re-deciding the functions that named a superseded
//! structure.
//!
//! The bounds are not transitive, so the order can still show through. `fb`
//! claims four fields, `fa` two of them and `fc` eight that include `fb`'s four.
//! In address order `fb` mints, `fa` reuses it and `fc` supersedes it; decided
//! again, `fb` moves to `fc`'s structure, but that structure is out of `fa`'s
//! reach, so `fa` keeps the superseded one. In the reverse order `fa` mints its
//! own shape. Either way the record has two names, and which one `fa` gets
//! depends on the order.

use std::rc::Rc;

use kuna_base::types::int4;

use crate::dtype::{type_metatype, Datatype, TypeFactory, TypeField};

/// The highest `struct_N` the ledger will probe.
const MAX_LEDGER_SLOTS: u32 = 1024;

/// How far a pointer chain is followed when a field type is spelled.
const MAX_POINTEE_DEPTH: u32 = 3;

/// The prefix every synthesized member carries, and the guard that keeps a
/// same-named type from somewhere else (DWARF, a parsed header) out of the
/// ledger.
const FIELD_PREFIX: &str = "field_0x";

/// How many times as many fields a container may claim as the layout it answers.
///
/// The reader measured at least half of what it is then declared to have. This
/// is the bound that does the work: `ls` O0 `sub_afe2` measured two fields of an
/// 80-byte layout and containment alone answered it with a sixteen-field
/// structure only 1.5x the size.
const MAX_CLAIM_GROWTH: usize = 2;

/// How many times its size a container may exceed the layout it answers.
///
/// The backstop for a small measurement inside a large record, where the claim
/// count says nothing: distinct records share their first words, so `ls`'s
/// `hash_entry`, `{void *, hash_entry *}`, is contained by the 200-byte layout
/// of its `stat`, which begins `{dev_t, ino_t}` and claims exactly twice as many
/// fields. Left unbounded that reader is declared to hold 174 bytes of members
/// nothing measured. Four, not two, because two records that really are one --
/// `ls`'s two `fileinfo` comparators, 80 bytes measured against 176 -- are only
/// just over a doubling apart, while the accidental containments measured over
/// the eight-binary census are an order of magnitude out.
const MAX_SIZE_GROWTH: usize = 4;

/// How many fields a layout must claim before a container that says strictly
/// more may answer for it.
///
/// The layout prune turns an access the exported header cannot spell at its own
/// offset into a hole, so a measured layout can reach the ledger with one
/// claimed field or none. That is not evidence of a record: an empty claim list
/// is contained by every structure alive. Such a layout is answered only by one
/// with exactly its shape.
const MIN_SHARED_CLAIMS: usize = 2;

/// How many fields a layout must claim before a container that says strictly
/// more may answer for it, when none of its claims is a pointer.
///
/// Two integer words are what distinct records share. `struct stat` begins
/// `{dev_t, ino_t}`, as do gnulib's `cycle_check_state` and `cp`'s `Src_to_dest`
/// (`{ino_t, dev_t}`); a list node, a `timespec` and a hash entry are two
/// integer words to a stripped reader too. Answering such a layout with the first
/// record that measured more declared `cycle_check`'s `struct stat *` as its
/// `cycle_check_state`. A typed pointer claim is stronger evidence, because its
/// pointee is part of the field's identity: that is what keeps `ls`'s two
/// `fileinfo` comparators, `{0: char *, 0x48: long}` inside four claims, on one
/// name.
const MIN_UNTYPED_CLAIMS: usize = 3;

/// One field a layout CLAIMS: an offset a dereference was actually seen at,
/// with the type the accesses agreed on spelled out.
///
/// Filler members -- the `undefined1[N]` arrays covering the holes -- are not
/// claims and never appear here.
///
/// Two fields are the same field when all three parts match, `undefined<N>`
/// included: two functions agree about a field or they are not looking at the
/// same field. A `char *` against an `unsigned int` at one offset is the obvious
/// conflict, and "no type at all" is its own answer rather than a wildcard the
/// other side may fill in. Letting `undefined8` be absorbed by anything 8 bytes
/// wide was measured on `du` O2 and it unifies records that only look alike:
/// `sub_c090` writes two double bit patterns and a zero byte at `{0, 8, 0x10}`,
/// which a wildcard rule reports as contained by an unrelated record whose
/// offset 0 is a pointer, and the body then prints
/// `field_0x0 = (void *)0x3f80000000000000`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(super) struct FieldKey {
    /// Byte offset inside the structure.
    pub offset: int4,
    /// Width of the access that minted it.
    pub width: int4,
    /// The field type, spelled by [`type_key`].
    pub ty: String,
    /// The field is a pointer whose pointee is a type: `char *`, `long *`,
    /// `struct_N *`.
    pub pointer: bool,
    /// The field is a pointer to code, to `void` or to an undefined type.
    pub opaque: bool,
}

/// A structural spelling of a type, deep enough that two different pointers are
/// two different fields: `PTR:8:>TYPE_INT:4:int4` is not `PTR:8:>TYPE_UINT:1:uint1`.
///
/// A pointer type has no name of its own, so comparing names alone makes every
/// `T **` the same field; the chain is followed instead, to the depth
/// `structsynth` can produce.
fn type_key(ct: &Datatype) -> String {
    let mut key = format!("{:?}:{}:{}", ct.get_metatype(), ct.get_size(), ct.get_name());
    let mut next = ct.get_ptr_to();
    for _ in 0..MAX_POINTEE_DEPTH {
        let Some(p) = next else { break };
        key.push_str(&format!(">{:?}:{}:{}", p.get_metatype(), p.get_size(), p.get_name()));
        next = p.get_ptr_to();
    }
    key
}

/// A synthesized structure reduced to what the dedup decision needs: its size
/// and its claimed fields, in offset order.
#[derive(Clone, Debug)]
pub(super) struct Layout {
    /// The structure's size in bytes, holes and tail rounding included.
    pub size: int4,
    /// Claimed fields, ascending by offset.
    pub fields: Vec<FieldKey>,
}

impl Layout {
    /// Does this layout contain every claim `other` makes, field for field?
    pub(super) fn subsumes(&self, other: &Layout) -> bool {
        if self.size < other.size {
            return false;
        }
        other.fields.iter().all(|want| self.fields.contains(want))
    }

    /// Is every claim past offset 0 an opaque pointer -- a table of slots?
    fn is_table(&self) -> bool {
        let mut slots = self.fields.iter().filter(|f| f.offset != 0).peekable();
        slots.peek().is_some() && slots.all(|f| f.opaque)
    }

    /// Is this layout the answer a reader who measured `other` should be given?
    ///
    /// A layout answers for itself and for anything with exactly its shape
    /// whatever the bounds say -- two readers that measured the same thing are
    /// looking at the same record, and refusing them one name would mint a
    /// second `struct_N` for every function that measured it. Saying strictly
    /// more than the reader measured is where the bounds apply, a table is never
    /// answered by more than its own shape, and a reader with no typed pointer
    /// is never answered by a larger structure.
    pub(super) fn answers_for(&self, other: &Layout) -> bool {
        if !self.subsumes(other) {
            return false;
        }
        if self.size == other.size && self.fields.len() == other.fields.len() {
            return true;
        }
        let claims = other.fields.len();
        let typed = other.fields.iter().any(|f| f.pointer);
        let evidence = !other.is_table()
            && (claims >= MIN_UNTYPED_CLAIMS || (claims >= MIN_SHARED_CLAIMS && typed))
            && (typed || self.size == other.size);
        evidence
            && self.size as usize <= (other.size as usize).saturating_mul(MAX_SIZE_GROWTH)
            && self.fields.len() <= claims.saturating_mul(MAX_CLAIM_GROWTH)
    }

    /// Does this layout answer for `other` while `other` does not answer back?
    fn strictly_subsumes(&self, other: &Layout) -> bool {
        self.answers_for(other) && !other.answers_for(self)
    }
}

/// Does this pointer chain end in code, `void` or an undefined type?
fn opaque_pointee(ct: &Datatype) -> bool {
    let mut next = ct.get_ptr_to();
    let mut last = None;
    for _ in 0..MAX_POINTEE_DEPTH {
        let Some(p) = next else { break };
        next = p.get_ptr_to();
        last = Some(p);
    }
    last.is_some_and(|p| {
        matches!(
            p.get_metatype(),
            type_metatype::TYPE_CODE | type_metatype::TYPE_VOID | type_metatype::TYPE_UNKNOWN
        )
    })
}

/// The key of one field of a completed structure, or `None` for a filler member.
fn key_of(f: &TypeField) -> Option<FieldKey> {
    if f.field_type.get_metatype() == type_metatype::TYPE_ARRAY {
        return None;
    }
    let is_ptr = f.field_type.get_ptr_to().is_some();
    let opaque = is_ptr && opaque_pointee(&f.field_type);
    Some(FieldKey {
        offset: f.offset,
        width: f.field_type.get_size(),
        ty: type_key(&f.field_type),
        pointer: is_ptr && !opaque,
        opaque,
    })
}

/// Read a completed structure's layout, or `None` when it is not one this pass
/// could have minted.
///
/// Every member of a synthesized structure is named for its own offset, so a
/// member named anything else says the name `struct_N` is held by a type that
/// came from somewhere else -- a DWARF record, a parsed header -- and such a
/// type is never handed out as the answer to an access pattern.
pub(super) fn layout_of(ct: &Datatype) -> Option<Layout> {
    if ct.get_metatype() != type_metatype::TYPE_STRUCT {
        return None;
    }
    let mut fields = Vec::new();
    for i in 0..ct.num_depend() {
        let f = ct.get_field(i)?;
        if !f.name.starts_with(FIELD_PREFIX) {
            return None;
        }
        if let Some(k) = key_of(f) {
            fields.push(k);
        }
    }
    Some(Layout { size: ct.get_size(), fields })
}

/// The same reading for a field list that has not been completed yet.
pub(super) fn layout_of_fields(fields: &[TypeField], size: int4) -> Layout {
    Layout { size, fields: fields.iter().filter_map(key_of).collect() }
}

/// One member of a structure as a body sees it: where it starts, how many bytes
/// it covers, and the field it claims (`None` for filler).
#[derive(Clone, PartialEq, Eq, Debug)]
pub(super) struct Member {
    /// Byte offset inside the structure.
    pub offset: int4,
    /// Bytes covered.
    pub size: int4,
    /// The claimed field, or `None` for an `undefined1[N]` filler.
    pub claim: Option<FieldKey>,
}

impl Member {
    fn of(f: &TypeField) -> Member {
        Member { offset: f.offset, size: f.field_type.get_size(), claim: key_of(f) }
    }
}

/// Every member of a completed structure, filler included.
fn members_of(ct: &Datatype) -> Vec<Member> {
    (0..ct.num_depend()).filter_map(|i| ct.get_field(i)).map(Member::of).collect()
}

/// Does `held` lay exactly the members `own` does over every byte range in
/// `unclaimed`? See "Unclaimed bytes" in the module documentation.
pub(super) fn keeps_unclaimed(held: &[Member], own: &[Member], unclaimed: &[(int4, int4)]) -> bool {
    let over = |members: &[Member], lo: int4, hi: int4| -> Vec<Member> {
        members
            .iter()
            .filter(|m| m.offset < hi && m.offset.saturating_add(m.size) > lo)
            .cloned()
            .collect()
    };
    unclaimed.iter().all(|&(off, width)| {
        let hi = off.saturating_add(width);
        over(held, off, hi) == over(own, off, hi)
    })
}

/// One minted entry.
struct Entry {
    /// The completed type.
    ct: Rc<Datatype>,
    /// Its claimed layout.
    layout: Layout,
}

/// Every `struct_N` the factory currently holds whose size lies in `sizes`,
/// plus the first free name.
///
/// The scan stops at the first unused slot, which is where a mint lands, so the
/// walk is as long as the ledger and no longer. `Err` from the factory ends it
/// with no mint name: a probe that cannot answer is not evidence that a name is
/// free. A structure outside `sizes` is skipped before its layout is read.
fn entries(
    types: &dyn TypeFactory,
    sizes: std::ops::RangeInclusive<i64>,
) -> (Vec<Entry>, Option<String>) {
    let mut out: Vec<Entry> = Vec::new();
    let mut free = None;
    for n in 0..MAX_LEDGER_SLOTS {
        let name = format!("struct_{n}");
        match types.find_by_name(&name) {
            Ok(None) => {
                free = Some(name);
                break;
            }
            Ok(Some(t)) => {
                if !sizes.contains(&i64::from(t.get_size())) {
                    continue;
                }
                if let Some(layout) = layout_of(&t) {
                    out.push(Entry { ct: t, layout });
                }
            }
            Err(_) => break,
        }
    }
    (out, free)
}

/// Does some other entry say strictly more than entry `i`?
fn is_superseded(layouts: &[&Layout], i: usize) -> bool {
    layouts.iter().enumerate().any(|(j, l)| j != i && l.strictly_subsumes(layouts[i]))
}

/// The entry a new layout should reuse: the SMALLEST live one that answers for
/// it, or failing that the smallest that answers at all.
///
/// Smallest, not largest, because every extra field a reused structure carries
/// is a claim this function's evidence does not support. Supersession is read
/// only for the entries that answer, so a lookup costs one pass over the ledger
/// plus one per candidate.
///
/// The growth bounds cost the relation its transitivity -- the structure that
/// superseded the smallest match can be out of this layout's reach -- so a
/// superseded entry still answers when nothing live does. Minting there instead
/// would give a name that entry supersedes the moment it exists. A new name is
/// therefore minted only when nothing held answers at all. `fits` is the
/// per-reader veto of [`keeps_unclaimed`].
fn best_of(layouts: &[&Layout], want: &Layout, fits: impl Fn(usize) -> bool) -> Option<usize> {
    let mut answering: Vec<usize> =
        (0..layouts.len()).filter(|&i| layouts[i].answers_for(want) && fits(i)).collect();
    answering.sort_by_key(|&i| (layouts[i].fields.len(), layouts[i].size));
    answering
        .iter()
        .copied()
        .find(|&i| !is_superseded(layouts, i))
        .or_else(|| answering.first().copied())
}

/// The sizes a lookup for a layout of `size` bytes has to read.
///
/// An entry that answers is at least `size` and at most [`MAX_SIZE_GROWTH`]
/// times it, and an entry that supersedes one of those is at most
/// [`MAX_SIZE_GROWTH`] times larger again. Nothing outside the window can change
/// the answer.
fn lookup_window(size: int4) -> std::ops::RangeInclusive<i64> {
    let lo = i64::from(size);
    let growth = MAX_SIZE_GROWTH as i64;
    lo..=lo.saturating_mul(growth).saturating_mul(growth)
}

/// Find or mint the program-wide `struct_N` for this layout.
///
/// `unclaimed` is the `(offset, width)` of every access the reader made without
/// claiming a field; a structure answers only if it lays the reader's own
/// members over those bytes ([`keeps_unclaimed`]).
///
/// `find_add` rejects a second, different definition of a held name with a hard
/// `Err`, so the mint declines rather than propagating and the caller keeps the
/// parameter untyped.
pub(super) fn lookup_or_mint(
    types: &dyn TypeFactory,
    fields: Vec<TypeField>,
    size: int4,
    unclaimed: &[(int4, int4)],
) -> Option<Rc<Datatype>> {
    let want = layout_of_fields(&fields, size);
    let (held, free) = entries(types, lookup_window(size));
    let layouts: Vec<&Layout> = held.iter().map(|e| &e.layout).collect();
    let own: Vec<Member> = fields.iter().map(Member::of).collect();
    let fits = |i: usize| unclaimed.is_empty() || keeps_unclaimed(&members_of(&held[i].ct), &own, unclaimed);
    if let Some(i) = best_of(&layouts, &want, fits) {
        return Some(Rc::clone(&held[i].ct));
    }
    // Nothing held answers for this layout. Minting it supersedes every entry
    // it strictly contains; that is derived from the minted set on the next
    // lookup, so there is nothing to record here.
    let name = free?;
    let shell = types.get_type_struct(&name).ok()?;
    types.set_fields_struct_raw(&shell, fields, Vec::new(), size, 1, 0).ok()
}

/// The synthesized structures a later, larger layout has taken over.
///
/// A whole-program driver reads this after its batch: a function that named one
/// of these was decided before the structure that contains it existed, and
/// deciding it again answers with the survivor.
pub fn superseded_names(types: &dyn TypeFactory) -> Vec<String> {
    let (held, _) = entries(types, i64::MIN..=i64::MAX);
    let layouts: Vec<&Layout> = held.iter().map(|e| &e.layout).collect();
    (0..held.len())
        .filter(|&i| is_superseded(&layouts, i))
        .map(|i| held[i].ct.get_name().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A claimed scalar field.
    fn f(offset: int4, width: int4, meta: type_metatype) -> FieldKey {
        FieldKey { offset, width, ty: format!("{meta:?}:{width}:"), pointer: false, opaque: false }
    }

    /// A claimed pointer field.
    fn p(offset: int4, pointee: &str) -> FieldKey {
        FieldKey {
            offset,
            width: 8,
            ty: format!("TYPE_PTR:8:>TYPE_INT:1:{pointee}"),
            pointer: true,
            opaque: false,
        }
    }

    /// A claimed `code *` field.
    fn code(offset: int4) -> FieldKey {
        FieldKey {
            offset,
            width: 8,
            ty: "TYPE_PTR:8:>TYPE_CODE:1:code".into(),
            pointer: false,
            opaque: true,
        }
    }

    fn layout(size: int4, fields: Vec<FieldKey>) -> Layout {
        Layout { size, fields }
    }

    /// No per-reader veto.
    fn any(_: usize) -> bool {
        true
    }

    fn claim(offset: int4, width: int4, meta: type_metatype) -> Member {
        Member { offset, size: width, claim: Some(f(offset, width, meta)) }
    }

    fn filler(offset: int4, size: int4) -> Member {
        Member { offset, size, claim: None }
    }

    /// cx6 `rd_c8` loads four bytes at 7 and stores a byte at 8; its writer
    /// `full` stores a byte at each of 7..0xa. Answered by `full`'s structure,
    /// the load is spelled through `field_0x7` and prints after the store.
    #[test]
    fn a_container_may_not_split_bytes_the_reader_accessed_without_claiming() {
        let u = type_metatype::TYPE_UINT;
        let own = vec![
            claim(0, 4, u),
            filler(4, 4),
            claim(8, 1, type_metatype::TYPE_INT),
            filler(9, 0xf),
            claim(0x18, 8, u),
            claim(0x20, 8, u),
            claim(0x28, 8, u),
        ];
        let full = vec![
            claim(0, 4, u),
            filler(4, 3),
            claim(7, 1, type_metatype::TYPE_INT),
            claim(8, 1, type_metatype::TYPE_INT),
            claim(9, 1, type_metatype::TYPE_INT),
            claim(0xa, 1, type_metatype::TYPE_INT),
            filler(0xb, 0xd),
            claim(0x18, 8, u),
            claim(0x20, 8, u),
            claim(0x28, 8, u),
        ];
        let load = [(7, 4)];
        assert!(!keeps_unclaimed(&full, &own, &load));
        assert!(keeps_unclaimed(&own, &own, &load));
        // Nothing unclaimed: containment alone decides.
        assert!(keeps_unclaimed(&full, &own, &[]));
        // A member that ends where the access starts moves a filler boundary
        // onto it, which spells the load through a member of its own.
        let mut edge = own.clone();
        edge.splice(1..2, [filler(4, 2), claim(6, 1, type_metatype::TYPE_INT), filler(7, 1)]);
        assert!(!keeps_unclaimed(&edge, &own, &load));
        // Members away from the unclaimed bytes do not matter, but past the
        // reader's own end every member the container has is new.
        let mut longer = own.clone();
        longer.push(claim(0x30, 8, u));
        assert!(keeps_unclaimed(&longer, &own, &load));
        let past = [(0x30, 0x10)];
        assert!(!keeps_unclaimed(&longer, &own, &past));
        assert!(keeps_unclaimed(&own, &own, &past));
    }

    #[test]
    fn a_vetoed_container_leaves_the_reader_to_mint() {
        let want = layout(16, vec![p(0, "uint1"), f(8, 4, type_metatype::TYPE_INT)]);
        let wide = layout(
            24,
            vec![p(0, "uint1"), f(8, 4, type_metatype::TYPE_INT), f(0xc, 4, type_metatype::TYPE_INT)],
        );
        let own = want.clone();
        assert_eq!(best_of(&[&wide], &want, any), Some(0));
        assert_eq!(best_of(&[&wide], &want, |_| false), None);
        // The reader's own shape is then minted beside the container, which
        // supersedes it; a superseded entry still answers when nothing else fits.
        let held = [&wide, &own];
        assert!(is_superseded(&held, 1));
        assert_eq!(best_of(&held, &want, |i| i == 1), Some(1));
    }

    #[test]
    fn a_layout_contained_at_the_same_offsets_is_subsumed() {
        let put_word = layout(16, vec![p(0, "uint1"), f(8, 4, type_metatype::TYPE_INT)]);
        let get_line = layout(
            24,
            vec![
                p(0, "uint1"),
                f(8, 4, type_metatype::TYPE_INT),
                f(0xc, 4, type_metatype::TYPE_INT),
                f(0x10, 1, type_metatype::TYPE_UINT),
            ],
        );
        assert!(get_line.answers_for(&put_word));
        assert!(!put_word.answers_for(&get_line));
        assert!(get_line.strictly_subsumes(&put_word));
    }

    #[test]
    fn a_smaller_structure_never_subsumes_a_larger_one() {
        let small = layout(8, vec![f(0, 8, type_metatype::TYPE_INT)]);
        let big = layout(16, vec![f(0, 8, type_metatype::TYPE_INT)]);
        assert!(big.subsumes(&small));
        assert!(!small.subsumes(&big));
    }

    #[test]
    fn two_types_at_one_offset_are_a_conflict_not_a_refinement() {
        let a = layout(16, vec![p(0, "char"), f(8, 4, type_metatype::TYPE_INT)]);
        let b = layout(16, vec![f(0, 8, type_metatype::TYPE_UINT), f(8, 4, type_metatype::TYPE_INT)]);
        assert!(!a.subsumes(&b));
        assert!(!b.subsumes(&a));
    }

    #[test]
    fn an_untyped_field_is_its_own_answer_not_a_wildcard() {
        let known = layout(8, vec![p(0, "char")]);
        let unknown = layout(8, vec![f(0, 8, type_metatype::TYPE_UNKNOWN)]);
        assert!(!known.subsumes(&unknown));
        assert!(!unknown.subsumes(&known));
        // Two readers that both failed to type the same 8 bytes still agree.
        assert!(unknown.subsumes(&unknown.clone()));
    }

    #[test]
    fn a_field_of_a_different_width_at_one_offset_is_not_the_same_field() {
        let wide = layout(8, vec![f(0, 8, type_metatype::TYPE_INT)]);
        let narrow = layout(8, vec![f(0, 4, type_metatype::TYPE_INT)]);
        assert!(!wide.subsumes(&narrow));
        assert!(!narrow.subsumes(&wide));
    }

    #[test]
    fn equal_layouts_subsume_each_other_and_neither_is_superseded() {
        let a = layout(16, vec![p(0, "uint1"), f(8, 4, type_metatype::TYPE_INT)]);
        let b = a.clone();
        assert!(a.subsumes(&b) && b.subsumes(&a));
        assert!(!a.strictly_subsumes(&b));
    }

    #[test]
    fn the_smallest_live_container_is_the_one_reused() {
        let want = layout(16, vec![p(0, "uint1"), f(8, 4, type_metatype::TYPE_INT)]);
        let wide = layout(
            32,
            vec![
                p(0, "uint1"),
                f(8, 4, type_metatype::TYPE_INT),
                f(0xc, 4, type_metatype::TYPE_INT),
                f(0x10, 1, type_metatype::TYPE_UINT),
            ],
        );
        // Disagrees with `wide` at 0xc, so neither supersedes the other.
        let tight = layout(
            16,
            vec![p(0, "uint1"), f(8, 4, type_metatype::TYPE_INT), f(0xc, 4, type_metatype::TYPE_UINT)],
        );
        assert!(!wide.subsumes(&tight));
        assert_eq!(best_of(&[&wide, &tight], &want, any), Some(1));
    }

    #[test]
    fn a_superseded_entry_is_never_handed_out_again() {
        let want = layout(16, vec![p(0, "uint1"), f(8, 4, type_metatype::TYPE_INT)]);
        let gone = want.clone();
        let survivor = layout(
            24,
            vec![p(0, "uint1"), f(8, 4, type_metatype::TYPE_INT), f(0xc, 4, type_metatype::TYPE_INT)],
        );
        let held = [&gone, &survivor];
        assert!(is_superseded(&held, 0));
        assert!(!is_superseded(&held, 1));
        assert_eq!(best_of(&held, &want, any), Some(1));
    }

    #[test]
    fn a_container_that_dwarfs_the_measurement_does_not_answer_for_it() {
        // `ls` O0: two words of a `hash_entry` against a 200-byte record that
        // happens to start with two words of the same width.
        let measured = layout(16, vec![f(0, 8, type_metatype::TYPE_UINT), f(8, 8, type_metatype::TYPE_UINT)]);
        let big = layout(
            200,
            vec![
                f(0, 8, type_metatype::TYPE_UINT),
                f(8, 8, type_metatype::TYPE_UINT),
                f(0x10, 8, type_metatype::TYPE_UINT),
                f(0xc4, 4, type_metatype::TYPE_UINT),
            ],
        );
        assert!(big.subsumes(&measured));
        assert!(!big.answers_for(&measured));
        assert_eq!(best_of(&[&big], &measured, any), None);
        // `ls` O0 `sub_afe2`: only 1.5x the size, but eight times the claims.
        let two = layout(80, vec![f(0, 8, type_metatype::TYPE_UINT), f(8, 8, type_metatype::TYPE_UINT)]);
        let sixteen = layout(
            120,
            (0..16).map(|i| f(i * 8, 8, type_metatype::TYPE_UINT)).collect(),
        );
        assert!(sixteen.subsumes(&two));
        assert!(!sixteen.answers_for(&two));
        // A layout the prune left with one claim is answered only by its shape.
        let lone = layout(32, vec![f(0, 8, type_metatype::TYPE_UINT)]);
        let pair = layout(40, vec![f(0, 8, type_metatype::TYPE_UINT), f(0x20, 8, type_metatype::TYPE_UINT)]);
        assert!(!pair.answers_for(&lone));
        assert!(lone.answers_for(&layout(32, vec![f(0, 8, type_metatype::TYPE_UINT)])));
        // The witness pair is well inside both bounds.
        let put_word = layout(16, vec![p(0, "uint1"), f(8, 4, type_metatype::TYPE_INT)]);
        let get_line = layout(
            24,
            vec![
                p(0, "uint1"),
                f(8, 4, type_metatype::TYPE_INT),
                f(0xc, 4, type_metatype::TYPE_INT),
                f(0x10, 1, type_metatype::TYPE_UINT),
            ],
        );
        assert!(get_line.answers_for(&put_word));
    }

    /// The bound costs subsumption its transitivity, so a layout whose only
    /// container has been superseded is still answered by that container.
    /// Minting instead gives a name the container supersedes the moment it
    /// exists: `fb` claims four fields, `fa` two of them, `fc` eight that include
    /// `fb`'s four. In address order `fb` mints, `fa` reuses it, `fc` supersedes
    /// it, and when the convergence sweep decides `fa` again the survivor is out
    /// of its reach.
    #[test]
    fn a_superseded_container_answers_when_nothing_live_reaches_this_layout() {
        let wide = |n: int4, size: int4| {
            let mut fields = vec![p(0, "char")];
            fields.extend((1..n).map(|i| f(i * 8, 8, type_metatype::TYPE_UINT)));
            layout(size, fields)
        };
        let fa = wide(2, 16);
        let fb = wide(4, 32);
        let fc = wide(8, 64);
        assert!(fb.answers_for(&fa));
        assert!(fc.strictly_subsumes(&fb));
        assert!(!fc.answers_for(&fa));
        let held = [&fb, &fc];
        assert!(is_superseded(&held, 0));
        assert_eq!(best_of(&held, &fa, any), Some(0));
        // An entry holding exactly the layout is the smallest answer of all.
        let held = [&fb, &fa, &fc];
        assert_eq!(best_of(&held, &fa, any), Some(1));
    }

    /// `du`/`cp` `cycle_check(struct cycle_check_state *, const struct stat *)`:
    /// the `stat` reader measures `{dev, ino}`, which is also how the state
    /// record begins, and the two must not share a name.
    #[test]
    fn two_integer_words_are_answered_only_by_their_own_shape() {
        let u8_ = |o: int4| f(o, 8, type_metatype::TYPE_UINT);
        let stat_reader = layout(16, vec![u8_(0), u8_(8)]);
        let state = layout(32, vec![u8_(0), u8_(8), u8_(0x10), f(0x18, 4, type_metatype::TYPE_INT)]);
        assert!(state.subsumes(&stat_reader));
        assert!(!state.answers_for(&stat_reader));
        assert_eq!(best_of(&[&state], &stat_reader, any), None);
        assert!(stat_reader.answers_for(&stat_reader.clone()));
        // A typed pointer among the two is evidence enough.
        let typed = layout(16, vec![p(0, "char"), u8_(8)]);
        let typed_state = layout(32, vec![p(0, "char"), u8_(8), u8_(0x10), u8_(0x18)]);
        assert!(typed_state.answers_for(&typed));
    }

    /// dpkg `pkg_queue_destroy` measures `{0: ulong, 8: ulong, 0x10: uint}` of a
    /// 24-byte `pkg_queue`, which is also how a 40-byte command record begins.
    /// Without a typed pointer, a reader is answered only by a structure of its
    /// own size, which may fill in its holes but never extends it.
    #[test]
    fn integer_claims_are_never_answered_by_a_larger_structure() {
        let u8_ = |o: int4| f(o, 8, type_metatype::TYPE_UINT);
        let queue = layout(24, vec![u8_(0), u8_(8), f(0x10, 4, type_metatype::TYPE_UINT)]);
        let command = layout(
            40,
            vec![u8_(0), u8_(8), f(0x10, 4, type_metatype::TYPE_UINT), u8_(0x18), u8_(0x20)],
        );
        assert!(command.subsumes(&queue));
        assert!(!command.answers_for(&queue));
        assert_eq!(best_of(&[&command], &queue, any), None);
        // The same extent with its hole filled in still answers.
        let holey = layout(32, vec![u8_(0), u8_(8), u8_(0x18)]);
        let filled = layout(32, vec![u8_(0), u8_(8), u8_(0x10), u8_(0x18)]);
        assert!(filled.answers_for(&holey));
        assert!(filled.strictly_subsumes(&holey));
        // A typed pointer lifts the restriction.
        let typed = layout(24, vec![p(0, "char"), u8_(8), u8_(0x10)]);
        let wider = layout(40, vec![p(0, "char"), u8_(8), u8_(0x10), u8_(0x18)]);
        assert!(wider.answers_for(&typed));
    }

    /// Reading only the structures inside [`lookup_window`] gives the same
    /// answer as reading all of them.
    #[test]
    fn the_size_window_never_changes_the_answer() {
        let chain = |n: int4, size: int4| {
            let mut fields = vec![p(0, "char")];
            fields.extend((1..n).map(|i| f(i * 8, 8, type_metatype::TYPE_UINT)));
            layout(size, fields)
        };
        let held = [
            chain(1, 8),
            chain(2, 16),
            chain(4, 32),
            chain(8, 64),
            chain(16, 256),
            chain(40, 400),
        ];
        for want in [chain(2, 16), chain(3, 24), chain(4, 32), chain(8, 64)] {
            let all: Vec<&Layout> = held.iter().collect();
            let window = lookup_window(want.size);
            let kept: Vec<usize> =
                (0..held.len()).filter(|&i| window.contains(&i64::from(held[i].size))).collect();
            let some: Vec<&Layout> = kept.iter().map(|&i| &held[i]).collect();
            assert_eq!(best_of(&all, &want, any), best_of(&some, &want, any).map(|k| kept[k]));
        }
    }

    /// `rsyslogd` O2 `strgenQueryInterface` fills `{int ifVersion; code *...}`
    /// up to 0x48 and `statsobjQueryInterface` a table of the same shape up to
    /// 0x88. They are different interface records, and each gets its own name.
    #[test]
    fn a_table_of_code_pointers_is_answered_only_by_its_own_shape() {
        let table = |slots: int4| {
            let mut fields = vec![f(0, 4, type_metatype::TYPE_INT)];
            fields.extend((1..=slots).map(|i| code(i * 8)));
            layout(8 * (slots + 1), fields)
        };
        let strgen = table(9);
        let statsobj = table(17);
        assert!(strgen.is_table() && statsobj.is_table());
        assert!(statsobj.subsumes(&strgen));
        assert!(!statsobj.answers_for(&strgen));
        assert_eq!(best_of(&[&statsobj], &strgen, any), None);
        assert!(!is_superseded(&[&strgen, &statsobj], 0));
        assert!(strgen.answers_for(&table(9)));
        // A table without a header at offset 0 is still a table.
        let vtable = layout(24, vec![code(0), code(8), code(0x10)]);
        let wider = layout(32, vec![code(0), code(8), code(0x10), code(0x18)]);
        assert!(!wider.answers_for(&vtable));
        // One typed claim past offset 0 makes it a record again.
        let record = layout(24, vec![f(0, 4, type_metatype::TYPE_INT), code(8), p(0x10, "char")]);
        let bigger = layout(
            32,
            vec![f(0, 4, type_metatype::TYPE_INT), code(8), p(0x10, "char"), code(0x18)],
        );
        assert!(!record.is_table());
        assert!(bigger.answers_for(&record));
        // An opaque pointer is not the pointer that makes two claims evidence.
        let two = layout(16, vec![code(0), f(8, 8, type_metatype::TYPE_UINT)]);
        let three = layout(
            24,
            vec![code(0), f(8, 8, type_metatype::TYPE_UINT), f(0x10, 8, type_metatype::TYPE_UINT)],
        );
        assert!(!two.is_table());
        assert!(three.subsumes(&two));
        assert!(!three.answers_for(&two));
    }

    #[test]
    fn nothing_contains_a_conflicting_layout_so_it_mints() {
        let want = layout(16, vec![p(0, "char"), f(8, 4, type_metatype::TYPE_INT)]);
        let held = layout(16, vec![p(0, "uint1"), f(8, 4, type_metatype::TYPE_INT)]);
        assert_eq!(best_of(&[&held], &want, any), None);
    }
}
