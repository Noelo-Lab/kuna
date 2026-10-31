# kuna xrefs fixture: RISC-V absolute (lui) and PC-relative (auipc) address
# pairs, for RV32 and RV64 alike. Linker relaxation is off so every pair stays
# the two instructions written here.
	.option	norelax
	.text
	.p2align 2
	.globl	edge_lui_lw
	.type	edge_lui_lw, %function
edge_lui_lw:
	lui	a1, %hi(edge_word)
	lw	a0, %lo(edge_word)(a1)
	ret

	.globl	edge_lui_addi
	.type	edge_lui_addi, %function
edge_lui_addi:
	lui	a0, %hi(edge_text)
	addi	a0, a0, %lo(edge_text)
	ret

	.globl	edge_lui_sw
	.type	edge_lui_sw, %function
edge_lui_sw:
	lui	a1, %hi(edge_word)
	sw	a0, %lo(edge_word)(a1)
	ret

	.globl	edge_auipc_addi
	.type	edge_auipc_addi, %function
edge_auipc_addi:
1:	auipc	a0, %pcrel_hi(edge_text)
	addi	a0, a0, %pcrel_lo(1b)
	ret

	.globl	edge_auipc_load
	.type	edge_auipc_load, %function
edge_auipc_load:
1:	auipc	a1, %pcrel_hi(edge_ptr)
#if __riscv_xlen == 64
	ld	a0, %pcrel_lo(1b)(a1)
#else
	lw	a0, %pcrel_lo(1b)(a1)
#endif
	ret

	.globl	edge_join
	.type	edge_join, %function
edge_join:                        # two bases meet at 1: — the addi's base is unknown
	lui	a1, %hi(edge_text)
	beqz	a0, 1f
	lui	a1, %hi(edge_other)
1:	addi	a0, a1, 0x10
	ret

	.section .rodata
	.p2align 4
	.globl	edge_pad
	.type	edge_pad, %object
edge_pad:	.asciz	"PAD_SO_EDGE_TEXT_IS_NOT_PAGE_ALIGNED"
	.size	edge_pad, .-edge_pad
	.globl	edge_text
	.type	edge_text, %object
edge_text:	.asciz	"EDGE_TEXT_LABEL"
	.size	edge_text, .-edge_text
	.p2align 12
	.globl	edge_pad2
	.type	edge_pad2, %object
edge_pad2:	.asciz	"PAD_TWO"
	.size	edge_pad2, .-edge_pad2
	.globl	edge_other
	.type	edge_other, %object
edge_other:	.asciz	"EDGE_OTHER_PAGE"
	.size	edge_other, .-edge_other

	.data
	.p2align 3
	.globl	edge_word
	.type	edge_word, %object
edge_word:	.word	42
	.size	edge_word, .-edge_word
	.p2align 3
	.globl	edge_ptr
	.type	edge_ptr, %object
#if __riscv_xlen == 64
edge_ptr:	.dword	edge_text
	.size	edge_ptr, .-edge_ptr
#else
edge_ptr:	.word	edge_text
	.size	edge_ptr, .-edge_ptr
#endif
