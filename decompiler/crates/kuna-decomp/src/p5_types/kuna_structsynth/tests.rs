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

/// A field commits to a signedness only when every access of its width does.
/// `find`'s `consider_visiting` reads `fts_info` (offset 0x68) into signed
/// comparisons and, with `movzwl`, into a call whose extension the call
/// absorbed; a `short` field would make that argument sign-extend.
#[test]
fn accesses_that_disagree_on_sign_leave_the_field_undefined() {
    use crate::dtype::TypeFactoryImpl;
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    let short = f.get_base(2, type_metatype::TYPE_INT).unwrap();
    let ushort = f.get_base(2, type_metatype::TYPE_UINT).unwrap();
    let undef2 = f.get_base(2, type_metatype::TYPE_UNKNOWN).unwrap();
    let int4 = f.get_base(4, type_metatype::TYPE_INT).unwrap();
    let ptr = f.get_type_pointer(8, f.get_base(1, type_metatype::TYPE_INT).unwrap(), 1).unwrap();
    let void_ptr = f.get_type_pointer(8, f.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap(), 1).unwrap();

    for (first, second) in [(&short, &undef2), (&undef2, &short), (&short, &ushort)] {
        let mut e = Evidence::default();
        e.record(0x68, 2, Some(Rc::clone(first)));
        e.record(0x68, 2, Some(Rc::clone(second)));
        e.record(0x68, 2, Some(Rc::clone(&short)));
        assert!(e.slots[&0x68].committed().is_none(), "a contested sign stays contested");
    }

    let mut agree = Evidence::default();
    agree.record(8, 4, Some(Rc::clone(&int4)));
    agree.record(8, 4, Some(Rc::clone(&int4)));
    assert!(agree.slots[&8].committed().is_some(), "agreeing accesses keep their type");

    let mut pointers = Evidence::default();
    pointers.record(0, 8, Some(Rc::clone(&ptr)));
    pointers.record(0, 8, Some(Rc::clone(&void_ptr)));
    assert!(pointers.slots[&0].committed().is_some(), "pointer spellings are not sign evidence");

    let mut widened = Evidence::default();
    widened.record(0x68, 2, Some(Rc::clone(&short)));
    widened.record(0x68, 2, Some(Rc::clone(&undef2)));
    widened.record(0x68, 4, Some(Rc::clone(&int4)));
    assert!(widened.slots[&0x68].committed().is_some(), "a wider access starts a new contest");
}

/// A float and a non-float of one width are a union member read two ways, so the
/// field is raw bytes: a `long` or `undefined8` field read as `double` prints a
/// value conversion of bits the binary reinterprets, and a byte array makes the
/// read cast the address instead.
#[test]
fn a_float_and_an_integer_of_one_width_make_the_field_raw_bytes() {
    use crate::dtype::TypeFactoryImpl;
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    let double = f.get_base(8, type_metatype::TYPE_FLOAT).unwrap();
    let long = f.get_base(8, type_metatype::TYPE_INT).unwrap();
    let ulong = f.get_base(8, type_metatype::TYPE_UINT).unwrap();
    let undef8 = f.get_base(8, type_metatype::TYPE_UNKNOWN).unwrap();
    let int4 = f.get_base(4, type_metatype::TYPE_INT).unwrap();
    let ptr = f.get_type_pointer(8, f.get_base(1, type_metatype::TYPE_INT).unwrap(), 1).unwrap();

    for other in [&long, &undef8, &ptr] {
        for (first, second) in [(&double, other), (other, &double)] {
            let mut e = Evidence::default();
            e.record(0, 4, Some(Rc::clone(&int4)));
            e.record(8, 8, Some(Rc::clone(first)));
            e.record(8, 8, Some(Rc::clone(second)));
            let slot = &e.slots[&8];
            assert!(slot.reinterpreted() && slot.committed().is_none(), "{} beside {}", first.get_name(), second.get_name());
            let (plan, size) = layout_plan(&e.slots).unwrap();
            assert_eq!(size, 0x10);
            assert!(plan.contains(&(8, 8, true)), "the union member is filler: {plan:?}");
        }
    }

    // A sign contest that already cleared the type does not hide a later float.
    let mut late = Evidence::default();
    late.record(8, 8, Some(Rc::clone(&long)));
    late.record(8, 8, Some(Rc::clone(&ulong)));
    late.record(8, 8, Some(Rc::clone(&double)));
    assert!(late.slots[&8].reinterpreted());

    let mut floats = Evidence::default();
    floats.record(8, 8, Some(Rc::clone(&double)));
    floats.record(8, 8, Some(Rc::clone(&double)));
    floats.record(8, 4, Some(Rc::clone(&int4)));
    assert!(!floats.slots[&8].reinterpreted(), "a narrower access is not this field's evidence");
    assert!(floats.slots[&8].committed().is_some(), "agreeing float accesses keep their type");
}
