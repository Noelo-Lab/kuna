	.file	1 "switch_nop_case_mipsel.c"
	.section .mdebug.abi32
	.previous
	.nan	legacy
	.module	fp=xx
	.module	nooddspreg
	.text
	.align	2
	.globl	g
	.set	nomips16
	.set	nomicromips
	.ent	g
	.type	g, @function
g:
	.frame	$sp,0,$31		# vars= 0, regs= 0/0, args= 0, gp= 0
	.mask	0x00000000,0
	.fmask	0x00000000,0
	.set	noreorder
	.set	nomacro
	lui	$2,%hi(sink)
	sw	$4,%lo(sink)($2)
	sll	$2,$4,1
	jr	$31
	addu	$2,$2,$4

	.set	macro
	.set	reorder
	.end	g
	.size	g, .-g
	.align	2
	.globl	pick
	.set	nomips16
	.set	nomicromips
	.ent	pick
	.type	pick, @function
pick:
	.frame	$sp,24,$31		# vars= 0, regs= 1/0, args= 16, gp= 0
	.mask	0x80000000,-4
	.fmask	0x00000000,0
	.set	noreorder
	.set	nomacro
	sltu	$2,$4,7
	beq	$2,$0,$L10
	nop

	lui	$2,%hi(map)
	addiu	$2,$2,%lo(map)
	addu	$4,$4,$2
	lbu	$2,0($4)
	sltu	$2,$2,9
	beq	$2,$0,$L12
	lui	$2,%hi(sink)

	addiu	$sp,$sp,-24
	lbu	$2,0($4)
	sll	$3,$2,2
	lui	$2,%hi($L6)
	addiu	$2,$2,%lo($L6)
	addu	$2,$2,$3
	lw	$2,0($2)
	jr	$2
	sw	$31,20($sp)

	.rdata
	.align	2
	.align	2
$L6:
	.word	$L9
	.word	$L7
	.word	$L4
	.word	$L4
	.word	$L8
	.word	$L4
	.word	$L4
	.word	$L7
	.word	$L5
	.text
$L9:
	jal	g
	li	$4,2070			# 0x816

	xori	$2,$2,0x1
$L2:
	lw	$31,20($sp)
	jr	$31
	addiu	$sp,$sp,24

$L8:
	jal	g
	li	$4,2074			# 0x81a

	b	$L2
	xori	$2,$2,0x5

$L5:
	jal	g
	li	$4,2078			# 0x81e

	b	$L2
	xori	$2,$2,0x9

$L7:
	nop
	jal	g
	li	$4,3007			# 0xbbf

	b	$L2
	addiu	$2,$2,11

$L4:
	lui	$2,%hi(sink)
	li	$3,7			# 0x7
	sw	$3,%lo(sink)($2)
	b	$L2
	li	$2,77			# 0x4d

$L10:
	jr	$31
	li	$2,270			# 0x10e

$L12:
	li	$3,7			# 0x7
	sw	$3,%lo(sink)($2)
	jr	$31
	li	$2,77			# 0x4d

	.set	macro
	.set	reorder
	.end	pick
	.size	pick, .-pick
	.rdata
	.align	2
	.type	map, @object
	.size	map, 7
map:
	.ascii	"\007\000\005\023\012\001\000"
	.globl	sink
	.section	.bss,"aw",@nobits
	.align	2
	.type	sink, @object
	.size	sink, 4
sink:
	.space	4
	.ident	"GCC: (Ubuntu 10.3.0-1ubuntu1) 10.3.0"
	.section	.note.GNU-stack,"",@progbits
