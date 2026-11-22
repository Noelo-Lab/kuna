/* kuna `thunknames`: MIPS jump thunks whose delay slot decides what they are.
 * `.Lpure` is `j release_box` with a nop in its delay slot, a pure forwarder;
 * `.Ladj` is `j release_box` with `addiu $a0,$a0,16` in its delay slot, a
 * `this` adjustor that is not one. mips_caller calls both. Rebuild: see
 * README.md. */
	.set noreorder
	.text
	.globl release_box
	.type release_box,@function
release_box:
	lw $v0, 0($a0)
	subu $v0, $v0, $a1
	jr $ra
	sw $v0, 0($a0)
	.size release_box, .-release_box
.Lpure:
	j release_box
	nop
.Ladj:
	j release_box
	addiu $a0, $a0, 16
	.globl mips_caller
	.type mips_caller,@function
mips_caller:
	addiu $sp, $sp, -32
	sw $ra, 28($sp)
	sw $s0, 24($sp)
	move $s0, $a0
	jal .Lpure
	li $a1, 1
	move $a0, $s0
	jal .Ladj
	li $a1, 2
	lw $s0, 24($sp)
	lw $ra, 28($sp)
	jr $ra
	addiu $sp, $sp, 32
	.size mips_caller, .-mips_caller
