ENTRY(_start)
PHDRS { code PT_LOAD FLAGS(5); }
SECTIONS {
    . = 0x401000;
    .text : { *(.text) } :code
    . = 0x402006;
    .padding : { *(.padding) } :code
    . = 0x403006;
    .short_slot : { *(.short_slot) } :code
    .short_tail : { *(.short_tail) } :code
    . = 0x404006;
    .full_slot : { *(.full_slot) } :code
    .full_tail : { *(.full_tail) } :code
    . = 0x405006;
    .shift_short : { *(.shift_short) } :code
    .shift_tail : { *(.shift_tail) } :code
    . = 0x406006;
    .overlap : { *(.overlap) } :code
    . = 0x407006;
    .abutting : { *(.abutting) } :code
    . = 0x408006;
    .long_padding : { *(.long_padding) } :code
    . = 0x418006;
    .long_insn : { *(.long_insn) } :code
    .long_insn_tail : { *(.long_insn_tail) } :code
    . = 0x419006;
    .trapped : { *(.trapped) } :code
    . = 0x41a006;
    .padded_wide : { *(.padded_wide) } :code
    . = 0x41b006;
    .weak_prefix : { *(.weak_prefix) } :code
    . = 0x41c006;
    .claimed_imm : { *(.claimed_imm) } :code
    . = 0x41d006;
    .claimed_end : { *(.claimed_end) } :code
    . = 0x41e006;
    .mixed_padding : { *(.mixed_padding) } :code
    . = 0x425000;
    .outer_insn : { *(.outer_insn) } :code
    .nested_bytes : { *(.nested_bytes) } :code
    . = 0x426006;
    .zero_after_trap : { *(.zero_after_trap) } :code
}
