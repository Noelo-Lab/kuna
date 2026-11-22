// swift_stdlib_macho.s: callers of the standard library's C entry points
// (stdlib/public/SwiftShims/swift/shims/AssertionReporting.h, FoundationShims.h),
// and of the runtime's swift_unexpectedError and swift_deallocBox.
// Built against the iOS 16.5 SDK with
//   clang -target arm64-apple-ios16.0 -isysroot $SDK -fuse-ld=lld -nostdlib -Wl,-e,_main \
//     -o swift_stdlib_macho swift_stdlib_macho.s \
//     $SDK/usr/lib/swift/libswiftCore.tbd $SDK/usr/lib/libSystem.tbd
	.section __TEXT,__text,regular,pure_instructions
	.globl _report_frame
	.p2align 2
// void report_frame(double *out, double w, double h): reports the unimplemented
// initializer with the two doubles live across the call, then stores w * h.
_report_frame:
	stp	x29, x30, [sp, #-48]!
	stp	x19, x20, [sp, #16]
	stp	d8, d9, [sp, #32]
	add	x29, sp, #0
	mov	x19, x0
	fmov	d8, d0
	fmov	d9, d1
	adrp	x0, l_class@PAGE
	add	x0, x0, l_class@PAGEOFF
	mov	w1, #5
	adrp	x2, l_init@PAGE
	add	x2, x2, l_init@PAGEOFF
	mov	w3, #12
	mov	w4, #0
	bl	__swift_stdlib_reportUnimplementedInitializer
	fmul	d0, d8, d9
	str	d0, [x19]
	ldp	d8, d9, [sp, #32]
	ldp	x19, x20, [sp, #16]
	ldp	x29, x30, [sp], #48
	ret

	.globl _report_rect
	.p2align 2
// void report_rect(double x, double y, double w, double h): reports the
// unimplemented initializer and traps, the rectangle unused.
_report_rect:
	stp	x29, x30, [sp, #-16]!
	add	x29, sp, #0
	adrp	x0, l_class@PAGE
	add	x0, x0, l_class@PAGEOFF
	mov	w1, #5
	adrp	x2, l_init@PAGE
	add	x2, x2, l_init@PAGEOFF
	mov	w3, #12
	mov	w4, #0
	bl	__swift_stdlib_reportUnimplementedInitializer
	brk	#1

	.globl _os_major
	.p2align 2
// long os_major(void): the major version of the operating-system version
// triple the entry point returns through x8.
_os_major:
	stp	x29, x30, [sp, #-48]!
	add	x29, sp, #0
	add	x8, sp, #16
	bl	__swift_stdlib_operatingSystemVersion
	ldr	x0, [sp, #16]
	ldp	x29, x30, [sp], #48
	ret

	.globl _retain_object
	.p2align 2
// void *retain_object(void *obj): the plain entry point, which also marks the
// image as Swift.
_retain_object:
	stp	x29, x30, [sp, #-16]!
	add	x29, sp, #0
	bl	_swift_retain
	ldp	x29, x30, [sp], #16
	ret

	.globl _try_value
	.p2align 2
// long try_value(void *error, long value): `try!` on a call that threw `error`
// traps in swift_unexpectedError; the instructions after the call are never
// reached.
_try_value:
	cbz	x0, Lok
	stp	x29, x30, [sp, #-16]!
	add	x29, sp, #0
	adrp	x1, l_file@PAGE
	add	x1, x1, l_file@PAGEOFF
	mov	w2, #9
	mov	w3, #1
	mov	w4, #42
	bl	_swift_unexpectedError
	mov	x0, #7
	ldp	x29, x30, [sp], #16
	ret
Lok:
	mov	x0, x1
	ret

	.globl _free_box_scaled
	.p2align 2
// double free_box_scaled(void *box, double scale): deallocates the box with
// scale live across the call.
_free_box_scaled:
	stp	x29, x30, [sp, #-32]!
	str	d8, [sp, #16]
	add	x29, sp, #0
	fmov	d8, d0
	bl	_swift_deallocBox
	fmul	d0, d8, d8
	ldr	d8, [sp, #16]
	ldp	x29, x30, [sp], #32
	ret

	.globl _main
	.p2align 2
_main:
	mov	w0, #0
	ret

	.section __TEXT,__cstring,cstring_literals
l_class:
	.asciz	"Frame"
l_init:
	.asciz	"init(frame:)"
l_file:
	.asciz	"Lib.swift"
.subsections_via_symbols
