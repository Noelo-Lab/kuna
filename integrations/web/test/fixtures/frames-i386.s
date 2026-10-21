# An i386 PIC prologue (assembled, no libc): `pic` saves two registers, calls
# the get-PC thunk, and only then reserves its locals.
#
# Rebuild:  gcc -m32 -c -o frames-i386.o frames-i386.s
	.intel_syntax noprefix
	.text

	.globl pic
	.type pic, @function
pic:
	push esi
	push ebx
	call get_pc_thunk_bx
	add ebx, 2
	sub esp, 0x3c
	lea eax, [esp + 0x10]
	push eax
	call fill
	add esp, 4
	mov eax, dword ptr [esp + 0x14]
	add esp, 0x3c
	pop ebx
	pop esi
	ret
	.size pic, .-pic

	.globl get_pc_thunk_bx
	.type get_pc_thunk_bx, @function
get_pc_thunk_bx:
	mov ebx, dword ptr [esp]
	ret
	.size get_pc_thunk_bx, .-get_pc_thunk_bx

	.globl fill
	.type fill, @function
fill:
	mov eax, dword ptr [esp + 4]
	mov dword ptr [eax], 0
	ret
	.size fill, .-fill
