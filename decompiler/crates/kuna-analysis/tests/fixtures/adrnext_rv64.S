# GH #1360 on RV64: `la a0, 2f` (auipc + addi) ends right before 2f, so the
# addi forms the address of the next instruction. The links are controls:
# `call`, `jalr ra`, `jal t0` (lifted as a link-register write and a jump) and
# `jal` to the next instruction.
	.option norvc
	.text
	.globl	auipc_self
	.type	auipc_self,@function
auipc_self:
	auipc	a0, 0
	ret
	.globl	la_next
	.type	la_next,@function
la_next:
1:	auipc	a0, %pcrel_hi(2f)
	addi	a0, a0, %pcrel_lo(1b)
2:	ret
	.globl	calls
	.type	calls,@function
calls:
	addi	sp, sp, -16
	sd	ra, 8(sp)
	call	auipc_self
	lla	t1, la_next
	jalr	ra, 0(t1)
	jal	t0, millicode
	jal	1f
1:	ld	ra, 8(sp)
	addi	sp, sp, 16
	ret
	.globl	millicode
	.type	millicode,@function
millicode:
	jr	t0
	.globl	main
	.type	main,@function
main:
	j	calls
