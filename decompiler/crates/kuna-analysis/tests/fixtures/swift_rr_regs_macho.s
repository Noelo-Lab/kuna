// swift_rr_regs_macho.s: a Swift-style caller of the runtime's register-specific
// retain/release entry points (swift/Runtime/CustomRRABI.h). Built against the
// iOS 16.5 SDK with
//   clang -target arm64-apple-ios16.0 -isysroot $SDK -fuse-ld=lld -nostdlib -Wl,-e,_main \
//     -o swift_rr_regs_macho swift_rr_regs_macho.s \
//     $SDK/usr/lib/swift/libswiftCore.tbd $SDK/usr/lib/libSystem.tbd
	.section __TEXT,__text,regular,pure_instructions
	.globl _keep_first
	.p2align 2
// void *keep_first(void *a, void *b): retains a (plain entry point),
// releases b through x20, returns a.
_keep_first:
	stp	x29, x30, [sp, #-32]!
	stp	x20, x19, [sp, #16]
	add	x29, sp, #0
	mov	x19, x0
	mov	x20, x1
	bl	_swift_retain
	bl	_swift_release_x20
	mov	x0, x19
	ldp	x20, x19, [sp, #16]
	ldp	x29, x30, [sp], #32
	ret

	.globl _retain_second
	.p2align 2
// void retain_second(void **slot, void *b): retains b through x21 and stores
// the result (x0) in *slot.
_retain_second:
	stp	x29, x30, [sp, #-32]!
	stp	x21, x19, [sp, #16]
	add	x29, sp, #0
	mov	x19, x0
	mov	x21, x1
	bl	_swift_retain_x21
	str	x0, [x19]
	ldp	x21, x19, [sp, #16]
	ldp	x29, x30, [sp], #32
	ret

	.globl _retain_first
	.p2align 2
// void *retain_first(void *a): the plain entry point, for comparison.
_retain_first:
	stp	x29, x30, [sp, #-16]!
	add	x29, sp, #0
	bl	_swift_retain
	ldp	x29, x30, [sp], #16
	ret

	.globl _bridge_swap
	.p2align 2
// void *bridge_swap(void *a, void *b): retains bridge object a through x22,
// releases bridge object b through x23, returns the retain's result (x0).
_bridge_swap:
	stp	x29, x30, [sp, #-32]!
	stp	x23, x22, [sp, #16]
	add	x29, sp, #0
	mov	x22, x0
	mov	x23, x1
	bl	_swift_bridgeObjectRetain_x22
	mov	x22, x0
	bl	_swift_bridgeObjectRelease_x23
	mov	x0, x22
	ldp	x23, x22, [sp, #16]
	ldp	x29, x30, [sp], #32
	ret

	.globl _main
	.p2align 2
_main:
	mov	w0, #0
	ret
.subsections_via_symbols
