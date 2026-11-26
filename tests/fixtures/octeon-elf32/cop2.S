/* A genuine COP2 load with the encoding Octeon reuses for bbit0. */
	.set noreorder
	.text
	.globl	load
	.type	load, @function
load:
	lwc2	$15, -6($3)
	jr	$31
	nop
	.size	load, .-load
