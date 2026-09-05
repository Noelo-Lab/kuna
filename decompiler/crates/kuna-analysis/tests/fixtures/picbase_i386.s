# (kuna) Fixture for `option picbase` -- the i386 PIC base register folded into
# the cross-reference index (kuna-analysis/src/listing/kuna_picbase.rs).
#
# Reduced from crackmes.one 5ab77f6333c5d40ad448ca52 (pancrackme v1.0), where
# `kuna strings --json --filter Password` found the prompt at 0x8049127 and
# reported xrefs_count 0 with an empty functions list, as did all 23 of the
# image's other strings: the address is formed at run time out of the GOT
# pointer, so the literal occurs nowhere in the file.
#
# Four functions, one per case the pass has to get right:
#
#   _start    establishes the base with the inline `call <next>; pop ebx; add`
#             idiom and forms `prompt` from it; its epilogue `pop ebx` restore
#             is what closes the live window.
#   inherits  never writes ebx at all, so it uses the base its caller left --
#             the shape that matters most, because the filing crackme's
#             function inventory splits its prompt routine at the int3 traps
#             and the `lea` lands in a different entry from the idiom.
#   clobbers  writes ebx for its own purposes. Its `lea secret@GOTOFF(%ebx)`
#             would resolve to `secret` if the base were assumed here, so this
#             is the negative case: `secret` must stay referenced by nothing.
#   thunked   takes the base from the out-of-line `__x86.get_pc_thunk.bx`
#             helper GCC emits instead of the inline idiom.
#
# `unused` is referenced by nothing and must stay that way.
#
# Built (see the `.py`-less convention of the hand-written fixtures here):
#   as --32 -o picbase_i386.o picbase_i386.s
#   ld -m elf_i386 -o picbase_i386 picbase_i386.o && rm picbase_i386.o
# Freestanding: never run, only decoded.

	.text
	.globl	_start
	.type	_start, @function
_start:
	pushl	%ebp
	movl	%esp, %ebp
	pushl	%ebx
	call	.L_pc
.L_pc:
	popl	%ebx
	addl	$_GLOBAL_OFFSET_TABLE_+[.-.L_pc], %ebx
	leal	prompt@GOTOFF(%ebx), %eax
	call	inherits
	call	clobbers
	call	thunked
	popl	%ebx
	popl	%ebp
	ret
	.size	_start, .-_start

	.globl	inherits
	.type	inherits, @function
inherits:
	leal	banner@GOTOFF(%ebx), %eax
	movl	counter@GOTOFF(%ebx), %edx
	addl	$1, %edx
	movl	%edx, counter@GOTOFF(%ebx)
	ret
	.size	inherits, .-inherits

	.globl	clobbers
	.type	clobbers, @function
clobbers:
	pushl	%ebx
	movl	$0x11111111, %ebx
	leal	secret@GOTOFF(%ebx), %eax
	popl	%ebx
	ret
	.size	clobbers, .-clobbers

	.globl	thunked
	.type	thunked, @function
thunked:
	pushl	%ebx
	call	__x86.get_pc_thunk.bx
	addl	$_GLOBAL_OFFSET_TABLE_, %ebx
	leal	trailer@GOTOFF(%ebx), %eax
	popl	%ebx
	ret
	.size	thunked, .-thunked

	.globl	__x86.get_pc_thunk.bx
	.type	__x86.get_pc_thunk.bx, @function
__x86.get_pc_thunk.bx:
	movl	(%esp), %ebx
	ret
	.size	__x86.get_pc_thunk.bx, .-__x86.get_pc_thunk.bx

	.section	.rodata
prompt:
	.string	"Password: "
banner:
	.string	"kuna picbase fixture"
secret:
	.string	"picbase must not claim this"
trailer:
	.string	"thunked reference"
unused:
	.string	"never referenced anywhere"

	.data
	.align	4
counter:
	.long	0
