//! Unit tests for the `structsynth` option surface and the layout rules.

use super::*;

#[test]
fn option_parses_off_and_param_and_rejects_anything_else() {
    assert_eq!(OptionStructSynth.apply("off").unwrap().0, StructSynthMode::Off);
    assert_eq!(OptionStructSynth.apply("param").unwrap().0, StructSynthMode::Param);
    assert!(OptionStructSynth.apply("all").is_err());
    assert!(OptionStructSynth.apply("on").is_err());
}

#[test]
fn off_does_not_fire_and_param_does() {
    assert!(!StructSynthMode::Off.fires());
    assert!(StructSynthMode::Param.fires());
    assert_eq!(StructSynthMode::default(), StructSynthMode::Off);
}

#[test]
fn a_constant_offset_sign_extends_by_its_own_width() {
    assert_eq!(sign_extend(0xff, 1), -1);
    assert_eq!(sign_extend(0xfff8, 2), -8);
    assert_eq!(sign_extend(8, 8), 8);
    assert_eq!(sign_extend(0x7f, 1), 127);
}

/// The widest access at one offset wins: a narrower field would lose the field
/// name entirely at the wide access site, while a wider one only adds a cast.
#[test]
fn the_widest_access_wins_an_offset() {
    let mut e = Evidence::default();
    e.record(8, 4, None);
    e.record(8, 8, None);
    assert_eq!(e.slots[&8].width, 8);
    e.record(8, 1, None);
    assert_eq!(e.slots[&8].width, 8);
}

/// Howard NDSS'11 §4.5: one uniform width over a contiguous run reads as an
/// array, and the less common layout is the one to prefer.
#[test]
fn a_uniform_contiguous_run_reads_as_an_array() {
    let mut fill = Evidence::default();
    for off in [0, 8, 16, 24] {
        fill.record(off, 8, None);
    }
    assert!(is_array_shaped(&fill.slots, 8));

    let mut layout = Evidence::default();
    layout.record(0, 8, None);
    layout.record(8, 4, None);
    layout.record(0xc, 4, None);
    assert!(!is_array_shaped(&layout.slots, 8));

    // A pair of pointer-sized fields is a record far more often than a `long[2]`.
    let mut two_words = Evidence::default();
    two_words.record(0, 8, None);
    two_words.record(8, 8, None);
    assert!(!is_array_shaped(&two_words.slots, 8));

    // `int pipefds[2]` and `arg[0]`/`arg[1]` over a `char *` are arrays.
    let mut two_ints = Evidence::default();
    two_ints.record(0, 4, None);
    two_ints.record(4, 4, None);
    assert!(is_array_shaped(&two_ints.slots, 8));
    let mut two_chars = Evidence::default();
    two_chars.record(0, 1, None);
    two_chars.record(1, 1, None);
    assert!(is_array_shaped(&two_chars.slots, 8));

    // Two sub-pointer offsets with a gap are weak evidence either way.
    let mut gapped = Evidence::default();
    gapped.record(0, 4, None);
    gapped.record(8, 4, None);
    assert!(!is_array_shaped(&gapped.slots, 8));
}

/// A buffer the function does not write every byte of is still a buffer.
/// Demanding exact contiguity made a `char[5]` written at 0, 1, 3 and 4 a
/// four-field structure; three or more offsets on the grid is the array
/// evidence below pointer width, gap or no gap.
#[test]
fn one_untouched_byte_does_not_make_a_buffer_a_record() {
    let mut buf = Evidence::default();
    for off in [0, 1, 3, 4] {
        buf.record(off, 1, None);
    }
    assert!(is_array_shaped(&buf.slots, 8));

    // At pointer width a gap does break the run: a record whose observed
    // members are pointer-sized with an untouched one between them is the
    // common case -- gnulib's `struct hash_table` read at 0, 0x10 and 0x28.
    let mut record = Evidence::default();
    for off in [0, 0x10, 0x28] {
        record.record(off, 8, None);
    }
    assert!(!is_array_shaped(&record.slots, 8));

    // Off the grid entirely is not a run at all.
    let mut off_grid = Evidence::default();
    off_grid.record(0, 4, None);
    off_grid.record(4, 4, None);
    off_grid.record(0xa, 4, None);
    assert!(!is_array_shaped(&off_grid.slots, 8));
}

/// An access inside a wider access's byte range cannot become its own field:
/// the C compiler would place it after the wider one and move every later
/// field. It is pruned to a hole and renders as the raw cast form instead.
#[test]
fn an_access_inside_a_wider_one_is_pruned_to_a_hole() {
    // `unsigned long lo = *(unsigned long *)p; int hi = *(int *)(p + 4);
    //  *(unsigned long *)(p + 8) = lo + hi;`
    let mut e = Evidence::default();
    e.record(0, 8, None);
    e.record(4, 4, None);
    e.record(8, 8, None);
    e.prune();
    assert_eq!(e.slots.keys().copied().collect::<Vec<_>>(), vec![0, 8]);
    // Two surviving 8-byte fields, no filler, and the structure ends at 16.
    assert_eq!(layout_plan(&e.slots).unwrap(), (vec![(0, 8, false), (8, 8, false)], 16));
}

/// A field whose offset its own alignment does not divide is padded forward by
/// the compiler, so it is pruned as well.
#[test]
fn a_misaligned_offset_is_not_a_field() {
    let mut e = Evidence::default();
    e.record(0, 4, None);
    e.record(0x18, 4, None);
    e.record(0x1c, 8, None);
    e.record(0x28, 8, None);
    e.prune();
    assert_eq!(e.slots.keys().copied().collect::<Vec<_>>(), vec![0, 0x18, 0x28]);
    // The pruned 0x1c access is inside the 0x4..0x18 filler, and the structure
    // ends at the last surviving field rather than at the pruned access.
    assert_eq!(
        layout_plan(&e.slots).unwrap(),
        (vec![(0, 4, false), (4, 0x14, true), (0x18, 4, false), (0x1c, 0xc, true), (0x28, 8, false)], 0x30)
    );
}

/// A 3-byte access cannot be a field: the exported header spells the
/// decompiler's 3-byte `undefined3` as a 4-byte `unsigned int`, which would move
/// every field after it.  Restricting the widths this way is also what makes the
/// surviving layout provably overlap-free -- two naturally aligned power-of-two
/// ranges either nest or are disjoint.
#[test]
fn an_access_the_header_cannot_spell_at_its_own_width_is_not_a_field() {
    let mut e = Evidence::default();
    e.record(0, 8, None);
    e.record(8, 3, None);
    e.record(0x10, 8, None);
    e.prune();
    assert_eq!(e.slots.keys().copied().collect::<Vec<_>>(), vec![0, 0x10]);
    // The 3-byte access is inside an 8-byte filler, so offset 8 still names a
    // declared member.
    assert_eq!(
        layout_plan(&e.slots).unwrap(),
        (vec![(0, 8, false), (8, 8, true), (0x10, 8, false)], 0x18)
    );
}

/// The misaligned half of an 8-byte pair overlapping an 8-byte field is pruned
/// by the alignment rule, leaving one field.
#[test]
fn a_misaligned_wide_access_over_a_field_leaves_one_field() {
    let mut e = Evidence::default();
    e.record(0, 8, None);
    e.record(4, 8, None);
    e.prune();
    assert_eq!(e.slots.keys().copied().collect::<Vec<_>>(), vec![0]);
}

/// The optimized `int fd[2]`: one 8-byte save/restore of the pair plus the two
/// 4-byte element accesses. The widths differ, so the uniform-run rule says
/// nothing -- the layout rule is what declines it, by leaving one field.
#[test]
fn a_saved_int_pair_collapses_to_one_field() {
    // sort O2 `rpl_pipe2`: `pipe(fd); close(fd[0]); close(fd[1]);` with the
    // pair saved and restored as one 8-byte word.
    let mut e = Evidence::default();
    e.record(0, 8, None);
    e.record(0, 4, None);
    e.record(4, 4, None);
    e.prune();
    assert_eq!(e.slots.keys().copied().collect::<Vec<_>>(), vec![0]);
    // One field is not a layout, so `accepts` declines on `slots.len() < 2`.
    assert!(e.slots.len() < 2);
}

/// A well-formed layout survives pruning untouched.
#[test]
fn a_disjoint_aligned_layout_is_left_alone() {
    let mut e = Evidence::default();
    e.record(0, 8, None);
    e.record(8, 4, None);
    e.record(0xc, 4, None);
    e.record(0x10, 8, None);
    e.prune();
    assert_eq!(e.slots.keys().copied().collect::<Vec<_>>(), vec![0, 8, 0xc, 0x10]);
}

/// The plateau flag `structsynth` gates on has to latch on a pass that changed
/// nothing even when it is the FIRST pass: a lattice that never needed to move
/// is still a decided one, and gating on "at least one pass changed something"
/// would decline such a function forever.
#[test]
fn the_plateau_latches_when_the_first_pass_changes_nothing() {
    use kuna_base::address::Address;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };
    use std::rc::Rc;

    use crate::action::ActionContext;
    use crate::context::ArchContext;
    use crate::coreaction_render::ActionInferTypes;

    let mut manage = AddrSpaceManager::new();
    manage.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manage.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    manage
        .insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
    let glb = Rc::new(ArchContext::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let entry = Address::new(ram, 0x1000);
    let mut fd = Funcdata::new("settled", "settled", glb, entry, 0x1000_0000, 0x40).unwrap();
    fd.set_type_recovery(true);
    assert!(fd.start_type_recovery());
    assert!(!fd.kuna_infertypes_settled());

    // A function with nothing to propagate: the first pass changes nothing.
    let mut infer = ActionInferTypes::boxed("typerecovery");
    let mut ctx = ActionContext::new();
    infer.apply(&mut fd, &mut ctx);
    assert!(fd.kuna_infertypes_settled());
}

/// The structure ends at its last surviving field, not at the furthest byte any
/// access touched.  One misaligned far access used to carry the size with it:
/// `*(int *)(p + 0x10001)` alongside two 8-byte fields declared a 64 KB type for
/// a 16-byte object.
#[test]
fn a_pruned_far_access_does_not_carry_the_size() {
    let mut e = Evidence::default();
    e.record(0, 8, None);
    e.record(8, 8, None);
    e.record(0x10001, 4, None);
    e.prune();
    assert_eq!(e.slots.keys().copied().collect::<Vec<_>>(), vec![0, 8]);
    let (plan, size) = layout_plan(&e.slots).unwrap();
    assert_eq!(size, 16);
    assert!(plan.iter().all(|(_, _, filled)| !filled), "no filler in a dense layout");
}

/// Every byte of the structure belongs to a member, so every offset the printer
/// can name is one the exported header declares.  `printc` invents the name
/// `field_0x<hex>` for an offset inside the structure that no field covers
/// (`printc.cc:1015-1033`), while the header would render that gap as
/// `undefined1 _pad<hex>[N]`.
#[test]
fn a_hole_becomes_a_named_member_at_its_own_offset() {
    let mut e = Evidence::default();
    e.record(0, 4, None);
    e.record(4, 4, None);
    e.record(0x28, 8, None);
    e.record(0x30, 8, None);
    e.prune();
    let (plan, size) = layout_plan(&e.slots).unwrap();
    assert_eq!(size, 0x38);
    // The 8..0x28 gap -- which `&p->field_0x8` names in the body -- is a member.
    assert!(plan.contains(&(8, 0x20, true)), "the hole at 8 is a member: {plan:?}");
    // No byte is left over.
    let mut next = 0;
    for (off, width, _) in &plan {
        assert_eq!(*off, next, "layout is dense: {plan:?}");
        next = off + width;
    }
    assert_eq!(next, size);
}


/// The structure's size is a multiple of its own alignment, as a C compiler
/// would make it.  `chibios`'s `chPipeReset` indexes its parameter, and a
/// structure the decompiler sizes 0x15 while the compiler sizes 0x18 puts
/// `a0[1]` at two different addresses.
#[test]
fn the_size_is_rounded_up_to_the_widest_field() {
    let mut e = Evidence::default();
    e.record(0, 4, None);
    e.record(8, 4, None);
    e.record(0x14, 1, None);
    e.prune();
    let (plan, size) = layout_plan(&e.slots).unwrap();
    assert_eq!(size, 0x18);
    assert_eq!(plan.last(), Some(&(0x15, 3, true)), "the tail is filler too: {plan:?}");
}

fn scalar(size: int4, mt: type_metatype) -> Option<Rc<Datatype>> {
    Some(Rc::new(Datatype::new(size, mt)))
}

fn run(accesses: &[(intb, int4, Option<Rc<Datatype>>)]) -> Evidence {
    let mut e = Evidence::default();
    for (off, width, ct) in accesses {
        e.record(*off, *width, ct.clone());
    }
    e.prune();
    e
}

/// `factor`'s `powm2`: two limbs of a `uintmax_t *` read at 0 and 8 over a base
/// the lattice already typed `unsigned long *`. The structure would be the same
/// bytes under a record name, so the element pointer is kept.
#[test]
fn a_same_typed_integer_pair_over_its_element_pointer_is_an_element_run() {
    use type_metatype::*;
    let uint8 = Datatype::new(8, TYPE_UINT);
    let pair = run(&[(0, 8, scalar(8, TYPE_UINT)), (8, 8, scalar(8, TYPE_UINT))]);
    assert!(is_element_run(&pair, &uint8));

    // A value whose type was never decided is `undefined8` on both sides, and a
    // signed pointee spells the same element.
    let unknown = run(&[(0, 8, None), (8, 8, None)]);
    assert!(is_element_run(&unknown, &uint8));
    assert!(is_element_run(&unknown, &Datatype::new(8, TYPE_INT)));
    assert!(is_element_run(&unknown, &Datatype::new(8, TYPE_UNKNOWN)));

    // The uniform-run rule already takes three or more; this one agrees.
    let three = run(&[(0, 8, None), (8, 8, None), (0x10, 8, None)]);
    assert!(is_element_run(&three, &uint8));
}

/// Any heterogeneous evidence keeps the structure.
#[test]
fn heterogeneous_evidence_is_not_an_element_run() {
    use type_metatype::*;
    let uint8 = Datatype::new(8, TYPE_UINT);

    // A pair of pointers: every such layout in the census is a record
    // (`hash_table`'s `bucket`/`bucket_limit`).
    let pointers = run(&[(0, 8, scalar(8, TYPE_PTR)), (8, 8, scalar(8, TYPE_PTR))]);
    assert!(!is_element_run(&pointers, &uint8));

    // One field an integer, the other a pointer.
    let mixed = run(&[(0, 8, scalar(8, TYPE_UINT)), (8, 8, scalar(8, TYPE_PTR))]);
    assert!(!is_element_run(&mixed, &uint8));

    // Fields that disagree in signedness: `timespec`, `stat`'s leading pair.
    let signs = run(&[(0, 8, scalar(8, TYPE_INT)), (8, 8, scalar(8, TYPE_UINT))]);
    assert!(!is_element_run(&signs, &uint8));

    // A float pair is left to the other rules.
    let floats = run(&[(0, 8, scalar(8, TYPE_FLOAT)), (8, 8, scalar(8, TYPE_FLOAT))]);
    assert!(!is_element_run(&floats, &Datatype::new(8, TYPE_FLOAT)));
}

/// The element pointer is kept only where the lattice already gave one.
#[test]
fn a_base_that_does_not_point_at_the_element_keeps_the_structure() {
    use type_metatype::*;
    let pair = run(&[(0, 8, None), (8, 8, None)]);
    // `char *` read as two words: the structure is what says "8 bytes".
    assert!(!is_element_run(&pair, &Datatype::new(1, TYPE_INT)));
    assert!(!is_element_run(&pair, &Datatype::new(8, TYPE_PTR)));
    assert!(!is_element_run(&pair, &Datatype::new(8, TYPE_FLOAT)));
}

/// The run is judged on the raw accesses, not the pruned layout: an access the
/// prune drops is still evidence that the object is not one element wide.
#[test]
fn a_pruned_or_off_grid_access_breaks_the_run() {
    use type_metatype::*;
    let uint8 = Datatype::new(8, TYPE_UINT);

    // `structsynth-overlap-layout.xml`: a 4-byte read inside the offset-0 word.
    let overlap = run(&[(0, 8, None), (4, 4, None), (8, 8, None)]);
    assert_eq!(overlap.slots.keys().copied().collect::<Vec<_>>(), vec![0, 8]);
    assert!(!is_element_run(&overlap, &uint8));

    // A narrower read at an element's own offset.
    let narrow = run(&[(0, 8, None), (0, 4, None), (8, 8, None)]);
    assert!(!is_element_run(&narrow, &uint8));

    // A misaligned word the prune drops.
    let misaligned = run(&[(0, 8, None), (8, 8, None), (0xc, 8, None)]);
    assert!(!is_element_run(&misaligned, &uint8));

    // A hole, and a run that does not start at 0.
    let hole = run(&[(0, 8, None), (0x10, 8, None)]);
    assert!(!is_element_run(&hole, &uint8));
    let late = run(&[(8, 8, None), (0x10, 8, None)]);
    assert!(!is_element_run(&late, &uint8));

    // One access is not a run.
    let one = run(&[(0, 8, None)]);
    assert!(!is_element_run(&one, &uint8));
}
