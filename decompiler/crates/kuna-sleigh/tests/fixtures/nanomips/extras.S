# Hand-written cases for instructions a compiler rarely emits. extras(seed)
# folds every result into s1 as h = rotl(h, 5) ^ value; tests/nanomips_core.rs
# carries the same computation.
	.section .table,"a"
	.word	extras
	.text
	.set	noat
	.macro	fold reg
	rotr	$s1,$s1,27
	xor	$s1,$s1,\reg
	.endm

	.type	clobber,@function
clobber:
	save	32,$fp,$ra,$s0-$s1
	li	$fp,1
	li	$s0,2
	li	$s1,3
	restore.jrc	32,$fp,$ra,$s0-$s1

	.globl	extras
	.type	extras,@function
extras:
	save	48,$ra,$s0-$s3,$gp
	move	$s0,$a0
	li	$s1,0
	li	$a1,0x0f0f0f0f
	move	$a2,$a1
	ins	$a2,$s0,5,9
	fold	$a2
	ext	$a2,$s0,7,13
	fold	$a2
	extw	$a2,$s0,$a1,11
	fold	$a2
	bitrevw	$a2,$s0
	fold	$a2
	bitrevh	$a2,$s0
	fold	$a2
	bitrevb	$a2,$s0
	fold	$a2
	byterevw	$a2,$s0
	fold	$a2
	byterevh	$a2,$s0
	fold	$a2
	rotr	$a2,$s0,9
	fold	$a2
	sov	$a2,$s0,$a1
	fold	$a2
	seqi	$a2,$s0,42
	fold	$a2
	clo	$a2,$s0
	fold	$a2
	add	$a2,$s0,$zero
	fold	$a2
	li	$a2,0x89abcdef
	addiu	$a2,$a2,0x76543211
	fold	$a2
	lwpc	$a2,words
	fold	$a2
	swpc	$s0,slot
	lwpc	$a3,slot
	fold	$a3
	lapc	$a3,words
	lw	$a2,4($a3)
	fold	$a2
	lwm	$a4,4($a3),3
	fold	$a4
	fold	$a5
	fold	$a6
	ualwm	$a4,2($a3),2
	fold	$a4
	fold	$a5
	swm	$s0,8($sp),2
	lw	$a2,8($sp)
	fold	$a2
	lw	$a2,12($sp)
	fold	$a2
	swm	$zero,8($sp),2
	lw	$a2,12($sp)
	fold	$a2
	move	$fp,$s0
	li	$gp,77
	balc	clobber
	fold	$fp
	fold	$s0
	movep	$a0,$a1,$s0,$s1
	movep	$s2,$s3,$a0,$a1
	fold	$s2
	fold	$s3
	move	$a0,$s1
	restore.jrc	48,$ra,$s0-$s3,$gp

	.data
	.align	2
words:
	.word	0x11223344, 0x55667788, 0x99aabbcc, 0xddeeff00
slot:
	.word	0
