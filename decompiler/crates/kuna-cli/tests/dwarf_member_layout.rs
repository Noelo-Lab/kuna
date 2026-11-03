//! Compiler-built DWARF layouts: DWARF 2 member and bitfield offsets, big-endian
//! bitfields, and C++ static data members, which are not part of the layout.
use crate::common;

fn decompile(fixture: &str, function: &str) -> String {
    let (out, err, rc) = common::run_kuna(&["decompile", &common::fixture(fixture), function]);
    assert_eq!(rc, 0, "{fixture} {function}: {err}\n{out}");
    out
}

#[test]
fn dwarf2_plus_uconst_offsets_place_members_and_bitfields() {
    let code = decompile("dwarf2_members_x86_64.o", "second");
    assert!(code.contains("return p->b;"), "{code}");
    let code = decompile("dwarf2_members_x86_64.o", "high_bits");
    assert!(code.contains("return f->hi;"), "{code}");
}

#[test]
fn static_data_members_take_no_place_in_the_layout() {
    let code = decompile("dwarf_static_member_x86_64.o", "first_of");
    assert!(code.contains("return c->first;") && !code.contains("instances"), "{code}");
}

#[test]
fn big_endian_bitfields_read_their_own_bits() {
    let fixtures = [("dwarf_bitfields_ppc_dwarf4.o", true), ("dwarf_bitfields_mips_dwarf5.o", false)];
    for (fixture, wide) in fixtures {
        let (code, err, rc) = common::run_kuna(&["decompile-all", &common::fixture(fixture)]);
        assert_eq!(rc, 0, "{fixture}: {err}\n{code}");
        assert!(!code.contains("error"), "{fixture}: {code}");
        let lines = ["return p->lo;", "return p->hi;", "return p->b;", "return p->count;", "p->b = v;"];
        for line in lines {
            assert!(code.contains(line), "{fixture} lacks `{line}`:\n{code}");
        }
        assert_eq!(code.contains("return p->wide;"), wide, "{fixture}:\n{code}");
    }
}
