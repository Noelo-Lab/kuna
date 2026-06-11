//! VERIFIER adversarial tests for item `w1-base-space-address`
//! (space.cc/.hh + address.cc/.hh).  Written by the independent verifier,
//! NOT the porter.  Expected values are pinned against the C++ semantics:
//! either the vendored sources re-derived by hand, or (where noted) a
//! standalone C++ micro-oracle transcribing the exact upstream expression
//! (g++ -std=c++11, x86-64).

use std::rc::Rc;

use kuna_base::address::{Address, RangeList};
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager};

fn proc_space(name: &str, index: i32, addr_size: u32, wordsize: u32) -> AddrSpace {
    AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        name,
        false,
        addr_size,
        wordsize,
        index,
        addrspace_flags::hasphysical,
        0,
        0,
    )
}

/// calcScaleMask (space.cc:34-44) for nontrivial wordsize, pinned against a
/// C++ micro-oracle of the exact upstream expression
/// `highest = calc_mask(addressSize); highest = highest*wordsize + (wordsize-1);`
/// run on x86-64:
///   as=2 ws=2      -> highest 0x1ffff,              plb 0x100,  pub 0x1feff
///   as=8 ws=2      -> highest 0xffffffffffffffff (64-bit wrap), pub MAX-0x1000
///   as=7 ws=0x80   -> highest 0x7fffffffffffffff,   plb 0x1000, pub 0x7fffffffffffefff
#[test]
fn verify_calc_scale_mask_wordsize_scaling_cpp_oracle() {
    let s = proc_space("w2", 1, 2, 2);
    assert_eq!(s.get_highest(), 0x1ffff);
    assert_eq!(s.get_pointer_lower_bound(), 0x100);
    assert_eq!(s.get_pointer_upper_bound(), 0x1feff);

    // 64-bit wrap of highest: calc_mask(8)*2 + 1 == u64::MAX (mod 2^64)
    let s = proc_space("w8", 2, 8, 2);
    assert_eq!(s.get_highest(), u64::MAX);
    assert_eq!(s.get_pointer_upper_bound(), u64::MAX - 0x1000);

    // highest in the signed-negative-modulus regime for wrapOffset
    let s = proc_space("w7", 3, 7, 0x80);
    assert_eq!(s.get_highest(), 0x7fffffffffffffff);
    assert_eq!(s.get_pointer_lower_bound(), 0x1000);
    assert_eq!(s.get_pointer_upper_bound(), 0x7fffffffffffefff);

    // wrapOffset (space.hh:383-391) with mod = (intb)(highest+1) =
    // i64::MIN: C++ micro-oracle gives
    //   wrapOffset(0xffffffffffffffff) == 0x7fffffffffffffff
    // (res = -1 % MIN = -1; res += MIN wraps to i64::MAX on x86).
    assert_eq!(s.wrap_offset(0xffffffffffffffff), 0x7fffffffffffffff);
    assert_eq!(s.wrap_offset(0x8000000000000000), 0);
    assert_eq!(s.wrap_offset(0x7fffffffffffffff), 0x7fffffffffffffff);
}

/// FINDING F1 (pinned, #[ignore]d pending repair): with wordsize == 0 the
/// C++ computes `(wordsize-1)` in **uint4** arithmetic, so
/// `highest = calc_mask(as)*0 + 0xFFFFFFFF = 0x00000000FFFFFFFF`
/// (C++ micro-oracle, any addressSize).  The Rust
/// `(self.wordsize as u64).wsub(1)` wraps at 64 bits and yields u64::MAX.
/// Run with `cargo test -- --ignored` to observe the divergence.
#[test]
#[ignore = "F1 w1-base-space-address: ws==0 wraps at u64 width, C++ wraps at uint4 (space.cc:38)"]
fn verify_calc_scale_mask_wordsize_zero_uint4_wrap_f1() {
    let s = proc_space("wz", 4, 4, 0);
    // C++ oracle: highest == 0x00000000FFFFFFFF for wordsize == 0
    assert_eq!(s.get_highest(), 0x00000000ffffffff);
}

/// RangeList insert/remove boundary semantics (address.cc:383-449):
/// overlap-by-exactly-one-byte merges, pure adjacency does NOT, equal-key
/// (same `first`) inserts merge through the upper_bound/step-back walk, and
/// removal at the u64::MAX top edge must not wrap an (last+1) split back in.
#[test]
fn verify_rangelist_boundary_merge_and_top_edge() {
    let ram = Rc::new(proc_space("ram", 3, 8, 1));

    // overlap by exactly one byte (first == predecessor.last) merges
    let mut list = RangeList::new();
    list.insert_range(Rc::clone(&ram), 0x100, 0x1ff);
    list.insert_range(Rc::clone(&ram), 0x1ff, 0x2ff);
    assert_eq!(list.num_ranges(), 1);
    let r = list.get_first_range().unwrap();
    assert_eq!((r.get_first(), r.get_last()), (0x100, 0x2ff));

    // pure adjacency (first == predecessor.last + 1) does NOT merge
    let mut list = RangeList::new();
    list.insert_range(Rc::clone(&ram), 0x100, 0x1ff);
    list.insert_range(Rc::clone(&ram), 0x200, 0x2ff);
    assert_eq!(list.num_ranges(), 2);

    // equal-key insert (same spc,first as an existing element) merges via
    // the predecessor step-back; bounds take the max
    let mut list = RangeList::new();
    list.insert_range(Rc::clone(&ram), 5, 10);
    list.insert_range(Rc::clone(&ram), 5, 7);
    assert_eq!(list.num_ranges(), 1);
    let r = list.get_first_range().unwrap();
    assert_eq!((r.get_first(), r.get_last()), (5, 10));

    // interior removal splits, exactly like C++ (a < first / b > last arms)
    list.remove_range(Rc::clone(&ram), 6, 7);
    assert_eq!(list.num_ranges(), 2);
    let mut s = String::new();
    list.print_bounds(&mut s);
    assert_eq!(s, "ram: 5-5\nram: 8-a\n"); // C++ printBounds is hex (address.cc:287)

    // top-edge removal: range ending at u64::MAX, removing through MAX must
    // not insert a wrapped (last+1, b) piece (C++ guard `b > last` is false)
    let mut list = RangeList::new();
    list.insert_range(Rc::clone(&ram), 0xffffffffffffff00, u64::MAX);
    list.remove_range(Rc::clone(&ram), 0xffffffffffffff80, u64::MAX);
    assert_eq!(list.num_ranges(), 1);
    let r = list.get_first_range().unwrap();
    assert_eq!((r.get_first(), r.get_last()), (0xffffffffffffff00, 0xffffffffffffff7f));

    // longest_fit chaining into a range ending at u64::MAX exercises the
    // C++ wrapping `(last + 1 - offset)` (last+1 == 0): the wrap yields the
    // correct 0x80 size, as upstream relies on
    let mut list = RangeList::new();
    list.insert_range(Rc::clone(&ram), 0xffffffffffffff00, u64::MAX);
    let addr = Address::new(Rc::clone(&ram), 0xffffffffffffff80);
    assert_eq!(list.longest_fit(&addr, 0x10000), 0x80);

    // in_range with size 0: offset+size-1 wraps to offset-1 (C++ unsigned
    // wrap); a range [0,10] therefore does NOT contain (offset 0, size 0)
    let mut list = RangeList::new();
    list.insert_range(Rc::clone(&ram), 0, 10);
    let zero = Address::new(Rc::clone(&ram), 0);
    assert!(!list.in_range(&zero, 0));
    assert!(list.in_range(&zero, 1));
}

/// assignShortcut collision walk (translate.cc:505-561): 26 distinct
/// 'a'-leading spaces fill 'a'..'z'; the 27th exhausts the walk
/// (collisionCount > 26), gets shortcut 'z' WITHOUT remapping
/// shortcut2space — lookup of 'z' still returns the 26th space.  Also the
/// signed-char map key: a name whose first byte is >= 0x80 keys the map at
/// its sign-extended int4 value and must round-trip through
/// get_space_by_shortcut.
#[test]
fn verify_shortcut_collision_walk_and_signed_char_key() {
    let mut manager = AddrSpaceManager::new();
    for i in 0..27 {
        manager
            .insert_space(Rc::new(proc_space(&format!("a{i:02}"), i, 4, 1)))
            .unwrap();
    }
    // first 26 spaces walk 'a'..'z'
    for i in 0..26u8 {
        assert_eq!(
            manager.get_space(i as i32).unwrap().get_shortcut(),
            (b'a' + i) as char,
            "space {i}"
        );
    }
    // 27th: collision budget exhausted -> shortcut 'z', map unchanged
    assert_eq!(manager.get_space(26).unwrap().get_shortcut(), 'z');
    assert!(Rc::ptr_eq(
        manager.get_space_by_shortcut(b'z').unwrap(),
        manager.get_space(25).unwrap()
    ));
    assert!(!Rc::ptr_eq(
        manager.get_space_by_shortcut(b'z').unwrap(),
        manager.get_space(26).unwrap()
    ));

    // uppercase first letter is lowercased before the walk
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(proc_space("RAM", 0, 8, 1))).unwrap();
    assert_eq!(manager.get_space(0).unwrap().get_shortcut(), 'r');
    // "register" is special-cased to '%'
    manager.insert_space(Rc::new(proc_space("register", 1, 4, 1))).unwrap();
    assert_eq!(manager.get_space(1).unwrap().get_shortcut(), '%');

    // signed-char shortcut key: "épsilon" starts with byte 0xC3; the C++
    // map key is (int4)(char)0xC3 == -61, and the byte must look the space
    // back up
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(proc_space("épsilon", 0, 4, 1))).unwrap();
    let spc = Rc::clone(manager.get_space(0).unwrap());
    assert_eq!(spc.get_shortcut() as u32, 0xC3);
    assert!(Rc::ptr_eq(manager.get_space_by_shortcut(0xC3).unwrap(), &spc));
}

/// AddrSpace::read / get_offset_size (space.cc:224-298) strtoul-base-0 and
/// suffix quirks, with the register branch dead (no Translate):
/// octal detection, hex size/offset suffixes, wordsize scaling, the bare
/// "0x" endptr quirk, and the leading-'+' string that strtoul consumes
/// whole (so the suffix parser never runs).
#[test]
fn verify_addrspace_read_strtoul_quirks() {
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(proc_space("ram", 3, 8, 1))).unwrap();
    manager.set_default_code_space(3).unwrap();
    let ram = Rc::clone(manager.get_space(3).unwrap());
    let mut size = 0i32;

    // strtoul base-0: leading 0 selects octal
    assert_eq!(ram.read("010", &mut size, &manager).unwrap(), 8);
    assert_eq!(size, 8); // natural size of the 8-byte default space

    // hex size suffix via get_offset_size (":0x4")
    assert_eq!(ram.read("0x10:0x4", &mut size, &manager).unwrap(), 0x10);
    assert_eq!(size, 4);

    // size suffix plus hex offset adjust (":2+0x10")
    assert_eq!(ram.read("0x100:2+0x10", &mut size, &manager).unwrap(), 0x110);
    assert_eq!(size, 2);

    // bare "0x": strtoul consumes only the "0" (endptr at 'x'), so the
    // string is not fully consumed; no ':'/'+' suffix exists -> offset 0
    size = -99;
    assert_eq!(ram.read("0x", &mut size, &manager).unwrap(), 0);
    assert_eq!(size, 8);

    // leading '+': strtoul accepts the sign and consumes the whole string,
    // taking the early return; the '+' suffix parser never runs
    assert_eq!(ram.read("+4", &mut size, &manager).unwrap(), 4);
    assert_eq!(size, 8);

    // wordsize scaling: offsets are addressToByte-scaled before suffix
    // adjustment (suffix adds bytes, not words)
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(proc_space("wram", 0, 4, 2))).unwrap();
    manager.set_default_code_space(0).unwrap();
    let wram = Rc::clone(manager.get_space(0).unwrap());
    assert_eq!(wram.read("0x10", &mut size, &manager).unwrap(), 0x20);
    assert_eq!(size, 4);
    assert_eq!(wram.read("0x10+1", &mut size, &manager).unwrap(), 0x21);
}

/// Address::overlap (address.cc:153-165) mixed-signedness quirks: the
/// `dist >= size` comparison sign-extends int4 `size` to uintb, so size==-1
/// compares against u64::MAX and the overlap "succeeds"; size==0 never
/// does; skip is sign-extended into the unsigned offset arithmetic; and
/// space identity is pointer identity, not value equality.
#[test]
fn verify_address_overlap_signedness_and_pointer_identity() {
    let ram = Rc::new(proc_space("ram", 3, 8, 1));
    let a = Address::new(Rc::clone(&ram), 0x100);

    // size = -1 sign-extends to u64::MAX: dist 0 < MAX -> returns 0
    assert_eq!(a.overlap(0, &a, -1), 0);
    // size = 0: dist 0 >= 0 -> -1
    assert_eq!(a.overlap(0, &a, 0), -1);

    // negative skip sign-extends through the wrapping offset arithmetic
    let hi = Address::new(Rc::clone(&ram), 0x110);
    assert_eq!(hi.overlap(-8, &a, 16), 8);
    // skip pushing below the range start wraps enormous -> -1
    assert_eq!(a.overlap(-1, &a, 16), -1);

    // a value-identical but distinct AddrSpace allocation is a different
    // space: C++ compares raw pointers
    let ram2 = Rc::new(proc_space("ram", 3, 8, 1));
    let b = Address::new(Rc::clone(&ram2), 0x100);
    assert_eq!(a.overlap(0, &b, 0x10), -1);
    assert!(a != b);
}
