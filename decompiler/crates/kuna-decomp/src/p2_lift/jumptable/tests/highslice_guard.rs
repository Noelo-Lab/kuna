use super::*;

fn pair(size: i32, output_size: i32, offset: u64, amount: u64) -> (Funcdata, OpId, OpId) {
    pair_with_gate(size, output_size, offset, amount, true)
}

fn pair_with_gate(
    size: i32,
    output_size: i32,
    offset: u64,
    amount: u64,
    enabled: bool,
) -> (Funcdata, OpId, OpId) {
    let mut arch = ArchContext::new(build_manager());
    arch.highslice_guard = enabled;
    let arch = Rc::new(arch);
    let ram = Rc::clone(arch.manage().get_space_by_name("ram").unwrap());
    let mut fd = Funcdata::new(
        "func",
        "func",
        arch,
        Address::new(ram, 0x1000),
        0x10000000,
        0x40,
    )
    .unwrap();
    let input = fd.new_varnode(size, &Address::new(ram_space(&fd), 0x10), None);
    let input = fd
        .vbank_mut()
        .set_input(input, &mut |_, _, _| Ok(()))
        .unwrap();
    let shift = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_RIGHT);
    let slice = mk_op(&mut fd, 2, 0x200, OpCode::CPUI_SUBPIECE);
    let amount = fd.new_constant(4, amount);
    let offset = fd.new_constant(4, offset);
    fd.op_set_input(shift, input, 0).unwrap();
    fd.op_set_input(shift, amount, 1).unwrap();
    fd.op_set_input(slice, input, 0).unwrap();
    fd.op_set_input(slice, offset, 1).unwrap();
    fd.new_unique_out(size, shift).unwrap();
    fd.new_unique_out(output_size, slice).unwrap();
    (fd, shift, slice)
}

fn assert_match(fd: &Funcdata, shift: OpId, slice: OpId, expected: i32) {
    assert_eq!(GuardRecord::one_off_match(fd, shift, slice), expected);
    assert_eq!(GuardRecord::one_off_match(fd, slice, shift), expected);
}

#[test]
fn highslice_guard_matches_all_supported_high_byte_slices() {
    for size in 2..=8 {
        for offset in 1..size {
            let (fd, shift, slice) = pair(size, size - offset, offset as u64, offset as u64 * 8);
            assert_match(&fd, shift, slice, 1);
        }
    }
}

#[test]
fn highslice_guard_rejects_nonidentical_values() {
    for (size, output, offset, amount) in [
        (8, 4, 4, 31),
        (8, 4, 4, 33),
        (8, 2, 4, 32),
        (8, 4, 3, 24),
        (8, 8, 0, 0),
        (8, 1, 8, 64),
        (16, 8, 8, 64),
    ] {
        let (fd, shift, slice) = pair(size, output, offset, amount);
        assert_match(&fd, shift, slice, 0);
    }
}

#[test]
fn highslice_guard_rejects_arithmetic_shift() {
    let (mut fd, shift, slice) = pair(8, 4, 4, 32);
    fd.obank_mut()
        .change_opcode(shift, TypeOp::new(OpCode::CPUI_INT_SRIGHT, 0, "s>>"));
    assert_match(&fd, shift, slice, 0);
}

#[test]
fn highslice_guard_requires_same_ssa_input() {
    let (mut fd, shift, slice) = pair(8, 4, 4, 32);
    let other = fd.new_varnode(8, &Address::new(ram_space(&fd), 0x20), None);
    fd.op_set_input(slice, other, 0).unwrap();
    assert_match(&fd, shift, slice, 0);
}

#[test]
fn highslice_guard_requires_constant_shift_and_offset() {
    for change_shift in [false, true] {
        let (mut fd, shift, slice) = pair(8, 4, 4, 32);
        let other = mk_input(&mut fd, 0x20);
        fd.op_set_input(if change_shift { shift } else { slice }, other, 1)
            .unwrap();
        assert_match(&fd, shift, slice, 0);
    }
}

#[test]
fn highslice_guard_requires_full_width_shift_output() {
    let (mut fd, shift, slice) = pair(8, 4, 4, 32);
    fd.op_unset_output(shift);
    fd.new_unique_out(4, shift).unwrap();
    assert_match(&fd, shift, slice, 0);
}

#[test]
fn highslice_guard_preserves_unsigned_signed_and_wrapping_ranges() {
    for (size, offset) in [(2, 1), (4, 2), (8, 4)] {
        let width = size - offset;
        let mask = (1u64 << (width * 8)) - 1;
        let sign = (mask >> 1) + 1;
        for (start, end) in [(0, 41), (sign - 2, sign + 2), (mask - 2, 2), (mask - 2, 0)] {
            for reverse in [false, true] {
                let (fd, shift, slice) = pair(size, width, offset as u64, offset as u64 * 8);
                let (guard_op, index_op) = if reverse {
                    (shift, slice)
                } else {
                    (slice, shift)
                };
                let guard_vn = fd.obank().get(guard_op).unwrap().get_out().unwrap();
                let index_vn = fd.obank().get(index_op).unwrap().get_out().unwrap();
                let guard_width = fd.vbank().get(guard_vn).unwrap().get_size();
                let end = if reverse && end == 0 { mask + 1 } else { end };
                let expected = CircleRange::new(start, end, guard_width, 1);
                let guards = [GuardRecord::new(
                    &fd,
                    guard_op,
                    guard_op,
                    0,
                    expected.clone(),
                    guard_vn,
                    false,
                )];
                let index_width = fd.vbank().get(index_vn).unwrap().get_size();
                let index_end = if reverse { 0 } else { mask + 1 };
                let mut range = CircleRange::new(0, index_end, index_width, 1);
                crate::kuna_switchguardrange::intersect(&fd, &guards, index_vn, &mut range);
                for value in [
                    0,
                    1,
                    2,
                    40,
                    41,
                    sign - 3,
                    sign - 2,
                    sign,
                    sign + 1,
                    sign + 2,
                    mask - 3,
                    mask - 2,
                    mask,
                ] {
                    assert_eq!(range.contains_val(value), expected.contains_val(value), "size={size} offset={offset} start={start} end={end} reverse={reverse} value={value}");
                }
                let mut wanted = Vec::new();
                let mut value = start;
                loop {
                    wanted.push(value);
                    value = (value + 1) & mask;
                    if value == end & mask {
                        break;
                    }
                    assert!(wanted.len() <= 41);
                }
                assert_eq!(range.get_size(), wanted.len() as u64);
                let mut actual = Vec::new();
                let mut value = range.get_min();
                loop {
                    actual.push(value);
                    if !range.get_next(&mut value) {
                        break;
                    }
                    assert!(actual.len() <= 41);
                }
                assert_eq!(actual, wanted);
            }
        }
    }
}

#[test]
fn highslice_guard_requires_explicit_enablement() {
    let (fd, shift, slice) = pair_with_gate(8, 4, 4, 32, false);
    assert_match(&fd, shift, slice, 0);
    assert!(crate::p2_lift::kuna_highslice_guard::matches(
        &fd, shift, slice
    ));
}
