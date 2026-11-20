# Stripped PE32 i386 fixture for the incremental-link table rule of thunkentry
# (tests/stages/kuna-iltentry.xml).
#
# An MSVC /INCREMENTAL image routes every call and vtable slot through a table of
# `jmp rel32` stubs. Here the entry calls only the first stub; the other fifteen
# bodies are reachable through the table alone.
#
# Build:
#   clang --target=i686-pc-windows-msvc -c iltentry_pe_i386.s -o iltentry.obj
#   lld-link /nologo /entry:start /subsystem:console /nodefaultlib /base:0x400000 \
#     /fixed /safeseh:no /opt:noref /Brepro /out:iltentry_pe_i386.exe iltentry.obj

	.intel_syntax noprefix
	.text
	.globl	_start
_start:
	call	stub0
	ret

	.p2align 4, 0xcc
	.irp k,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15
stub\k:
	.byte	0xe9
	.long	body\k - . - 4
	.endr

	.irp k,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15
	.p2align 4, 0xcc
body\k:
	push	ebp
	.byte	0x8b, 0xec
	mov	eax, \k * 3 + 1
	imul	eax, dword ptr [ebp + 8]
	pop	ebp
	ret	4
	.endr
