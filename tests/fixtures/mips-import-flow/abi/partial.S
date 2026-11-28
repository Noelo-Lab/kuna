/* A real PIC call and its GNU call-site DIE, followed by an incomplete
   declaration of the same import. The ELF linker retains R_MIPS_JALR. */
.abicalls
.set noreorder
.text
.globl proof_incomplete
.type proof_incomplete,@function
proof_incomplete:
.cpload $25
addiu $sp,$sp,-24
sw $31,20($sp)
lw $25,%call16(partial)($28)
.reloc .Lcall,R_MIPS_JALR,partial
.Lcall:
jalr $25
nop
.Lreturn:
lw $31,20($sp)
jr $31
addiu $sp,$sp,24
.Lend_code:
.size proof_incomplete,.-proof_incomplete

.section .debug_abbrev,"",@progbits
.Labbrev:
.byte 1, 0x11, 1                 /* compilation unit with children */
.byte 0x13, 5, 0, 0              /* DW_AT_language, DW_FORM_data2 */
.byte 2, 0x2e, 1                 /* concrete subprogram */
.byte 0x11, 1, 0x12, 6, 0, 0     /* low_pc, high_pc as size */
.byte 3
.uleb128 0x4109                  /* GNU_call_site */
.byte 0, 0x31, 0x13, 0x11, 1, 0, 0 /* abstract_origin ref4, low_pc addr */
.byte 4, 0x2e, 1                 /* declaration with children */
.byte 0x6e, 8                    /* linkage_name, string */
.byte 0x3f, 0x19, 0x3c, 0x19     /* external and declaration flags */
.byte 0x27, 0x19, 0, 0           /* prototyped */
.byte 5, 5, 0, 0x49, 0x13, 0, 0  /* formal parameter, type ref4 */
.byte 6, 0x24, 0                 /* signed int base type */
.byte 3, 8, 0x0b, 0x0b, 0x3e, 0x0b, 0, 0
.byte 7, 0x2e, 0, 0x3c, 0x19, 0, 0 /* nested subprogram declaration */
.byte 0

.section .debug_info,"",@progbits
.Lunit:
.4byte .Lend-.Lversion
.Lversion:
.2byte 4
.4byte .Labbrev
.byte 4, 1
.2byte 0x0c                      /* C99 */
.Lint:
.byte 6
.asciz "int"
.byte 4, 5
.byte 2
.4byte proof_incomplete
.4byte .Lend_code-proof_incomplete
.byte 7                          /* resume the enclosing range after this DIE */
.byte 3
.4byte .Ldecl-.Lunit
.4byte .Lreturn
.byte 0
.Ldecl:
.byte 4
.asciz "partial"
.byte 5
.4byte .Lint-.Lunit
.byte 5
.4byte .Lint-.Lunit
.Lend:                           /* deliberately omit both child-list terminators */
.section .note.GNU-stack,"",@progbits
