use kuna_num::opcodes::{get_booleanflip, OpCode};

#[test]
fn undefined_complements_preserve_both_reorder_states() {
    for opcode in [
        OpCode::CPUI_COPY,
        OpCode::CPUI_INT_ADD,
        OpCode::CPUI_FLOAT_ADD,
        OpCode::CPUI_FLOAT_LESS,
        OpCode::CPUI_FLOAT_LESSEQUAL,
        OpCode::CPUI_MAX,
    ] {
        for initial in [false, true] {
            let mut reorder = initial;
            assert_eq!(get_booleanflip(opcode, &mut reorder), OpCode::CPUI_MAX);
            assert_eq!(reorder, initial, "{opcode:?}");
        }
    }
}
