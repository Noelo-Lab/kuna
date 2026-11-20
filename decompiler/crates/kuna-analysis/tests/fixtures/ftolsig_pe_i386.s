# Stripped PE32 i386 fixture for kuna_msvcftolsig (tests/stages/kuna-msvcftolsig.xml).
#
# `_f` is ftolsig_pe_i386.c compiled by Visual C++ 2005 SP1 (`cl /O2 /c`); it calls
# `__ftol2_sse`. `_g` calls `__ftol`. Both helpers below are the instruction streams
# of the VS2005 libcmt `ftol2.obj` and `ftol.obj`, reassembled byte for byte.
# `_h` calls `_ftol_unknown`, the same contract in bytes no signature knows.
#
# Build:
#   clang --target=i686-pc-windows-msvc -c ftolsig_pe_i386.s -o h.obj
#   lld-link /nologo /entry:start /subsystem:console /nodefaultlib /base:0x400000 \
#     /fixed /safeseh:no /out:ftolsig_pe_i386.exe h.obj ftolsig_pe_i386_f.obj

	.intel_syntax noprefix
	.text
	.globl	_start
_start:
	mov	eax, offset outbuf
	push	eax
	push	0x3f800000
	mov	eax, offset qv
	push	eax
	mov	eax, offset pv
	push	eax
	call	_f
	add	esp, 16
	mov	eax, offset pv
	push	eax
	call	_g
	call	_h
	add	esp, 4
	ret

_g:
	mov	eax, dword ptr [esp + 4]
	fld	dword ptr [eax]
	fmul	dword ptr [eax + 4]
	call	_ftol_stripped
	ret

_h:
	mov	eax, dword ptr [esp + 4]
	fld	dword ptr [eax + 8]
	fadd	dword ptr [eax + 12]
	call	_ftol_unknown
	ret

	.p2align 4, 0xcc
_ftol_unknown:
	sub	esp, 8
	fistp	qword ptr [esp]
	pop	eax
	pop	edx
	ret

	.p2align 4, 0xcc
	.globl	__ftol2_sse
__ftol2_sse:
	cmp	dword ptr [sse2_available], 0
	je	ftol2_x87
ftol2_pentium4:
	push	ebp
	.byte	0x8b, 0xec
	sub	esp, 8
	and	esp, -8
	fstp	qword ptr [esp]
	cvttsd2si eax, qword ptr [esp]
	leave
	ret
ftol2_sse_excpt:
	cmp	dword ptr [sse2_available], 0
	je	ftol2_x87
	sub	esp, 4
	fnstcw	word ptr [esp]
	pop	eax
	and	ax, 0x7f
	cmp	ax, 0x7f
	je	ftol2_pentium4
ftol2_x87:
	push	ebp
	.byte	0x8b, 0xec
	sub	esp, 0x20
	and	esp, -16
	fld	st(0)
	fst	dword ptr [esp + 0x18]
	fistp	qword ptr [esp + 0x10]
	fild	qword ptr [esp + 0x10]
	mov	edx, dword ptr [esp + 0x18]
	mov	eax, dword ptr [esp + 0x10]
	test	eax, eax
	je	ftol2_zero
ftol2_nonint:
	fsubp	st(1), st
	test	edx, edx
	jns	ftol2_pos
	fstp	dword ptr [esp]
	mov	ecx, dword ptr [esp]
	xor	ecx, 0x80000000
	add	ecx, 0x7fffffff
	adc	eax, 0
	mov	edx, dword ptr [esp + 0x14]
	adc	edx, 0
	jmp	ftol2_exit
ftol2_pos:
	fstp	dword ptr [esp]
	mov	ecx, dword ptr [esp]
	add	ecx, 0x7fffffff
	sbb	eax, 0
	mov	edx, dword ptr [esp + 0x14]
	sbb	edx, 0
	jmp	ftol2_exit
ftol2_zero:
	mov	edx, dword ptr [esp + 0x14]
	test	edx, 0x7fffffff
	jne	ftol2_nonint
	fstp	dword ptr [esp + 0x18]
	fstp	dword ptr [esp + 0x18]
ftol2_exit:
	leave
	ret

	.p2align 4, 0xcc
_ftol_stripped:
	push	ebp
	.byte	0x8b, 0xec
	add	esp, -12
	wait
	fnstcw	word ptr [ebp - 2]
	wait
	mov	ax, word ptr [ebp - 2]
	or	ah, 0x0c
	mov	word ptr [ebp - 4], ax
	fldcw	word ptr [ebp - 4]
	fistp	qword ptr [ebp - 12]
	fldcw	word ptr [ebp - 2]
	mov	eax, dword ptr [ebp - 12]
	mov	edx, dword ptr [ebp - 8]
	leave
	ret

	.data
sse2_available:	.long	1
pv:	.long	0x3f800000, 0x40000000, 0x40400000, 0x40800000, 0x40a00000, 0x40c00000
qv:	.long	0x3f800000, 0x40000000
outbuf:	.long	0, 0
	.globl	__fltused
__fltused:	.long	0x9875
