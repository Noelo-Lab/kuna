# GH #1360's MIPS64 controls: n64's compiler spec names no return-address
# register, so only the delay slot says `bltzal $zero, 1f` (SLEIGH's `nal`) is
# a link; `bal 1f` past its delay slot and `jal`/`jalr` are links as on MIPS32.
# None of them references the return address it leaves in ra.
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
	daddiu	$sp, $sp, -16
	sd	$ra, 8($sp)
	jal	bal_next
	nop
	lui	$t9, %hi(nal_next)
	daddiu	$t9, $t9, %lo(nal_next)
	jalr	$t9
	nop
	ld	$ra, 8($sp)
	jr	$ra
	daddiu	$sp, $sp, 16
	.end	main
