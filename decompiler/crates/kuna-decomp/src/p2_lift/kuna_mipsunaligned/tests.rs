use super::*;

fn i(op: u32, rs: u32, rt: u32, imm: i32) -> u32 {
    (op << 26) | (rs << 21) | (rt << 16) | (imm as u32 & 0xffff)
}

fn lwl(rt: u32, off: i32, base: u32) -> u32 {
    i(0x22, base, rt, off)
}

fn lwr(rt: u32, off: i32, base: u32) -> u32 {
    i(0x26, base, rt, off)
}

fn move_(rd: u32, rs: u32) -> u32 {
    (rs << 21) | (rd << 11) | 0x21
}

fn pairs(words: &[u32], big: bool) -> Vec<u64> {
    pairs_in(&[(0x1000, words.to_vec())], big, false, false, &[], &|_| false)
}

#[test]
fn big_endian_pair_has_its_left_half_first() {
    assert_eq!(pairs(&[lwl(3, 0, 2), lwr(3, 3, 2)], true), vec![0x1000, 0x1004]);
    assert_eq!(pairs(&[lwl(3, 3, 2), lwr(3, 0, 2)], true), Vec::<u64>::new());
}

#[test]
fn little_endian_pair_has_its_right_half_at_the_lower_address() {
    assert_eq!(pairs(&[lwl(3, 3, 2), lwr(3, 0, 2)], false), vec![0x1000, 0x1004]);
    assert_eq!(pairs(&[lwl(3, 0, 2), lwr(3, 3, 2)], false), Vec::<u64>::new());
}

#[test]
fn halves_must_share_both_registers() {
    assert!(pairs(&[lwl(3, 0, 2), lwr(4, 3, 2)], true).is_empty());
    assert!(pairs(&[lwl(3, 0, 2), lwr(3, 3, 5)], true).is_empty());
    assert!(pairs(&[lwl(2, 0, 2), lwr(2, 3, 2)], true).is_empty());
}

#[test]
fn an_unrelated_instruction_between_keeps_the_pair() {
    let lw_ra = i(0x23, 29, 31, 36);
    assert_eq!(pairs(&[lwl(3, 1, 17), lw_ra, lwr(3, 4, 17)], true), vec![0x1000, 0x1008]);
}

#[test]
fn a_write_of_the_base_or_a_use_of_the_data_register_between_breaks_it() {
    assert!(pairs(&[lwl(3, 0, 2), i(0x09, 2, 2, 4), lwr(3, 3, 2)], true).is_empty());
    assert!(pairs(&[lwl(3, 0, 2), move_(4, 3), lwr(3, 3, 2)], true).is_empty());
    let swl = i(0x2a, 2, 3, 0);
    let swr = i(0x2e, 2, 3, 3);
    assert_eq!(pairs(&[swl, move_(4, 3), swr], true), vec![0x1000, 0x1008]);
    assert!(pairs(&[swl, move_(3, 4), swr], true).is_empty());
}

#[test]
fn the_partner_may_sit_in_a_delay_slot_but_not_a_likely_one() {
    let bne = i(0x05, 30, 16, -20);
    let swl = i(0x2a, 17, 2, 0);
    let swr = i(0x2e, 17, 2, 3);
    assert_eq!(pairs(&[swl, bne, swr], true), vec![0x1000, 0x1008]);
    let bnel = i(0x15, 30, 16, -20);
    assert!(pairs(&[swl, bnel, swr], true).is_empty());
    assert!(pairs(&[swl, bne, 0, swr], true).is_empty());
    let beq = i(0x04, 4, 0, 4);
    assert!(pairs(&[beq, lwl(3, 0, 2), lwr(3, 3, 2)], true).is_empty());
}

#[test]
fn a_branch_into_the_pair_breaks_it() {
    let into = i(0x04, 4, 0, -3);
    assert!(pairs(&[lwl(3, 0, 2), lwr(3, 3, 2), 0, into, 0], true).is_empty());
    let onto_first = i(0x04, 4, 0, -4);
    assert_eq!(pairs(&[lwl(3, 0, 2), lwr(3, 3, 2), 0, onto_first, 0], true), vec![0x1000, 0x1004]);
}

#[test]
fn interleaved_pairs_both_pair() {
    let words = [lwl(2, 0, 4), lwl(3, 4, 4), lwr(2, 3, 4), lwr(3, 7, 4)];
    assert_eq!(pairs(&words, true), vec![0x1000, 0x1004, 0x1008, 0x100c]);
}

#[test]
fn a_left_half_before_a_lone_half_is_dropped() {
    let words = [lwl(3, 0, 2), lwr(5, 3, 6), lwr(3, 3, 2)];
    assert!(pairs(&words, true).is_empty());
}

#[test]
fn doubleword_halves_pair_only_on_a_64_bit_target() {
    let ldl = i(0x1a, 2, 3, 0);
    let ldr = i(0x1b, 2, 3, 7);
    let runs = [(0x1000, vec![ldl, ldr])];
    assert_eq!(pairs_in(&runs, true, true, false, &[], &|_| false), vec![0x1000, 0x1004]);
    assert!(pairs_in(&runs, true, false, false, &[], &|_| false).is_empty());
}

#[test]
fn another_instruction_set_is_left_alone() {
    let runs = [(0x1000, vec![lwl(3, 0, 2), lwr(3, 3, 2)])];
    assert!(pairs_in(&runs, true, false, false, &[], &|_| true).is_empty());
}

#[test]
fn a_function_starting_inside_the_pair_breaks_it() {
    let runs = [(0x1000, vec![lwl(3, 0, 2), lwr(3, 3, 2), 0])];
    assert!(pairs_in(&runs, true, false, false, &[0x1004], &|_| false).is_empty());
    assert_eq!(pairs_in(&runs, true, false, false, &[0x1000], &|_| false), vec![0x1000, 0x1004]);
}

#[test]
fn an_octeon_bit_branch_is_a_branch_on_octeon_alone() {
    let bbit0 = i(0x32, 5, 4, 2);
    let skips = [(0x1000, vec![lwl(3, 0, 2), bbit0, 0, lwr(3, 3, 2), 0])];
    assert!(pairs_in(&skips, true, true, true, &[], &|_| false).is_empty());
    assert_eq!(pairs_in(&skips, true, true, false, &[], &|_| false), vec![0x1000, 0x100c]);
    let lands = [(0x1000, vec![bbit0, 0, lwl(3, 0, 2), lwr(3, 3, 2), 0])];
    assert!(pairs_in(&lands, true, true, true, &[], &|_| false).is_empty());
}
