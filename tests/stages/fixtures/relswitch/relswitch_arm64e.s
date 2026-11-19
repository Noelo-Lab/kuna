// relswitch_arm64e: a Swift-style enum dispatch through an arm64e hardened
// jump table (clang -faarch64-jump-table-hardening): the index is clamped by
// cmp/csel, not guarded, and the table sits in __text, listed in
// LC_FUNCTION_STARTS, as Apple's linker lays it out.
//
// Build: clang -target arm64e-apple-ios16.0 -c relswitch_arm64e.s
//        ld64.lld -arch arm64e -dylib -platform_version ios 16.0 16.0
//                 -undefined dynamic_lookup; llvm-strip -x
// _dispatch is at 0x4000, the table at 0x4060, its offsets relative to 0x4020.
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_dispatch
	.p2align	2
_dispatch:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	lsr	x16, x0, #62
	cmp	x16, #3
	csel	x16, x16, xzr, ls
	adrp	x17, _dispatch_jt@PAGE
	add	x17, x17, _dispatch_jt@PAGEOFF
	ldrsw	x16, [x17, x16, lsl #2]
Lbase:
	adr	x17, Lbase
	add	x16, x17, x16
	br	x16
Lc0:
	mov	w0, #11
	bl	_sink
	b	Lout
Lc1:
	add	w0, w1, #5
	bl	_sink
	b	Lout
Lc2:
	lsl	w0, w1, #3
	bl	_sink
	b	Lout
Lc3:
	mov	w0, #99
	bl	_g
Lout:
	ldp	x29, x30, [sp], #16
	ret
	.p2align	2
	.data_region jt32
_dispatch_jt:
	.long	Lc0-Lbase
	.long	Lc1-Lbase
	.long	Lc2-Lbase
	.long	Lc3-Lbase
	.end_data_region
	.globl	_after
	.p2align	2
_after:
	add	w0, w0, #1
	b	_g
.subsections_via_symbols
