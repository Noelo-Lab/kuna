# Prologue shapes the frame walk has to read (assembled, no libc):
# `shrink_wrapped` tests its argument before it saves a register and reserves
# its locals; `probed` reserves a large frame a page at a time through a probe
# loop, as -fstack-clash-protection does; `interleaved` puts a CMOV (a branch to
# the next instruction in SLEIGH) between its pushes; `tail_first` leaves
# through a conditional tail call before it sets its frame up; `rotated` enters
# its probe loop at the loop's test.
#
# Rebuild:  gcc -nostdlib -static -o frames-asm.elf frames-asm.s
	.intel_syntax noprefix
	.text

	.globl shrink_wrapped
	.type shrink_wrapped, @function
shrink_wrapped:
	test rdi, rdi
	je 1f
	push rbx
	sub rsp, 0x20
	mov rbx, rdi
	mov rdi, rsp
	call fill
	mov rax, qword ptr [rsp + 0x8]
	add rax, rbx
	add rsp, 0x20
	pop rbx
	ret
1:	xor eax, eax
	ret
	.size shrink_wrapped, .-shrink_wrapped

	.globl probed
	.type probed, @function
probed:
	lea r11, [rsp - 0x3000]
2:	sub rsp, 0x1000
	or qword ptr [rsp], 0
	cmp rsp, r11
	jne 2b
	sub rsp, 0x20
	mov rdi, rsp
	call fill
	mov eax, dword ptr [rsp + 0x10]
	add rsp, 0x3020
	ret
	.size probed, .-probed

	.globl interleaved
	.type interleaved, @function
interleaved:
	push rbx
	cmp edi, esi
	cmovc edi, esi
	push rbp
	sub rsp, 0x10
	mov rbx, rdi
	mov rdi, rsp
	call fill
	mov eax, dword ptr [rsp + 0x8]
	add eax, ebx
	add rsp, 0x10
	pop rbp
	pop rbx
	ret
	.size interleaved, .-interleaved

	.globl tail_first
	.type tail_first, @function
tail_first:
	test rdi, rdi
	jne fill
	push rbx
	sub rsp, 0x20
	mov rdi, rsp
	call fill
	mov eax, dword ptr [rsp + 0x8]
	add rsp, 0x20
	pop rbx
	ret
	.size tail_first, .-tail_first

	.globl rotated
	.type rotated, @function
rotated:
	lea r11, [rsp - 0x2000]
	jmp 2f
1:	sub rsp, 0x1000
	or qword ptr [rsp], 0
2:	cmp rsp, r11
	jne 1b
	push rbx
	sub rsp, 0x20
	mov rdi, rsp
	call fill
	add rsp, 0x2028
	pop rbx
	ret
	.size rotated, .-rotated

	.globl fill
	.type fill, @function
fill:
	mov qword ptr [rdi], 0
	mov qword ptr [rdi + 0x8], 0
	ret
	.size fill, .-fill

	.globl _start
	.type _start, @function
_start:
	xor edi, edi
	call shrink_wrapped
	call probed
	call interleaved
	call tail_first
	call rotated
	mov eax, 60
	xor edi, edi
	syscall
	.size _start, .-_start
