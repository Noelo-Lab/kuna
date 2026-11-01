# GH #1360's controls on MIPS32: `bal 1f` to the instruction after its delay
# slot lifts as an ra write and a jump, `bltzal $zero, 1f` (SLEIGH's `nal`)
# as an ra write with no branch at all, `jal`/`jalr` as calls. None of them
# references the return address it leaves in ra.
	.set	noreorder
	.text
	.globl	bal_next
	.type	bal_next,@function
	.ent	bal_next
bal_next:
	move	$t0, $ra
	bal	1f
	nop
1:	move	$v0, $ra
	jr	$t0
	move	$ra, $t0
	.end	bal_next
	.globl	nal_next
	.type	nal_next,@function
	.ent	nal_next
nal_next:
	move	$t0, $ra
	bltzal	$zero, 1f
	nop
1:	move	$v0, $ra
	jr	$t0
	nop
	.end	nal_next
	.globl	main
	.type	main,@function
	.ent	main
main:
	addiu	$sp, $sp, -16
	sw	$ra, 12($sp)
	jal	bal_next
	nop
	lui	$t9, %hi(nal_next)
	addiu	$t9, $t9, %lo(nal_next)
	jalr	$t9
	nop
	lw	$ra, 12($sp)
	jr	$ra
	addiu	$sp, $sp, 16
	.end	main
