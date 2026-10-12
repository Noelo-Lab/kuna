//! Unit tests for the `mixedtailret` option surface and the storage overlap the
//! walk asks of every write. The end-to-end behaviour is
//! `decompiler/crates/kuna-cli/tests/mixed_tail_returns.rs`.

use super::*;

use kuna_base::space::{spacetype, AddrSpace};
use std::rc::Rc;

fn space(name: &str, index: int4) -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, name, false, 8, 1, index, 0, 0, 0))
}

#[test]
fn option_parses_on_and_off_and_rejects_anything_else() {
    assert!(OptionMixedTailRet.apply("on").unwrap().0);
    assert!(!OptionMixedTailRet.apply("off").unwrap().0);
    assert!(OptionMixedTailRet.apply("mixed").is_err());
    assert_eq!(OptionMixedTailRet::NAME, "mixedtailret");
}

#[test]
fn storage_overlap_is_by_byte_within_one_space() {
    let reg = space("register", 3);
    let ram = space("ram", 1);
    let at = |sp: &Rc<AddrSpace>, off: u64| Address::new(Rc::clone(sp), off);
    assert!(overlaps(&at(&reg, 0), 8, &at(&reg, 0), 4));
    assert!(overlaps(&at(&reg, 1), 1, &at(&reg, 0), 4));
    assert!(!overlaps(&at(&reg, 4), 4, &at(&reg, 0), 4));
    assert!(!overlaps(&at(&reg, 0x10), 8, &at(&reg, 0), 8));
    assert!(!overlaps(&at(&ram, 0), 8, &at(&reg, 0), 8));
}
