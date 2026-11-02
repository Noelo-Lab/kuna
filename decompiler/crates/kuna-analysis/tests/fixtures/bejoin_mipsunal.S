# option bejoin callers: an unaligned load into $3 after a call reads
# nothing of $3 (GH-904). add_hi returns the high word of a 64-bit sum and
# leaves the carry in $3; c_unal adds an unaligned word it loads into $3 to
# add_hi's result; c_low stores add_hi's $2 only.
	.set noreorder
	.text
	.globl add_hi
	.type add_hi, @function
add_hi:
	addu	$3, $5, $7
	sltu	$3, $3, $5
	addu	$2, $4, $6
	jr	$31
	addu	$2, $3, $2
	.size add_hi, .-add_hi

	.globl c_unal
	.type c_unal, @function
c_unal:
	addiu	$29, $29, -24
	sw	$31, 20($29)
	sw	$16, 16($29)
	jal	add_hi
	lw	$16, 40($29)
	lwl	$3, 1($16)
	lwr	$3, 4($16)
	addu	$2, $3, $2
	lw	$16, 16($29)
	lw	$31, 20($29)
	jr	$31
	addiu	$29, $29, 24
	.size c_unal, .-c_unal

	.globl c_low
	.type c_low, @function
c_low:
	addiu	$29, $29, -24
	sw	$31, 20($29)
	jal	add_hi
	nop
	lui	$4, %hi(sink32)
	sw	$2, %lo(sink32)($4)
	lw	$31, 20($29)
	jr	$31
	addiu	$29, $29, 24
	.size c_low, .-c_low

	.data
	.globl sink32
sink32:	.word 0
