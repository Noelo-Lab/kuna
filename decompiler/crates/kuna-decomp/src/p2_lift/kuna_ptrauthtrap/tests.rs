// Tests for `kuna_ptrauthtrap` (included into `mod tests` in
// kuna_ptrauthtrap.rs). The end-to-end cut is pinned by
// `tests/stages/kuna-ptrauthtrap.xml` over the `macho_arm64e_auth` fixture.

use super::*;

#[test]
fn trap_immediates_are_the_four_key_codes() {
    for imm in [0xc470, 0xc471, 0xc472, 0xc473] {
        assert!(is_ptrauth_trap_immediate(imm), "{imm:#x}");
    }
    for imm in [0, 1, 0xf000, 0xc46f, 0xc474, 0x5513] {
        assert!(!is_ptrauth_trap_immediate(imm), "{imm:#x}");
    }
}
