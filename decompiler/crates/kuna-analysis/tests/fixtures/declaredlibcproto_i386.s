# (kuna `declaredlibcproto`) Naming a callee must also give it a prototype.
#
# The reduction of the witness (a stripped, statically linked i386 crackme whose
# anti-debug state machine calls glibc's `ptrace`, dataset challenge
# 5ab77f5c33c5d40ad448c615).  `wrapper` is a VARARGS wrapper in the shape glibc's
# own `ptrace` has -- `va_start` takes the address of the first argument and every
# later read goes through that pointer -- so no amount of body analysis can say
# what arrives.  The image is stripped, so the only evidence the callee is `ptrace`
# is the operator's `--define-function 0x8049027=ptrace`, and until that
# declaration answered out of the built-in signature tables it bought the call
# sites a spelling and nothing else.
#
#   as --32 -o declaredlibcproto_i386.o declaredlibcproto_i386.s
#   ld -m elf_i386 -o declaredlibcproto_i386 declaredlibcproto_i386.o
#   strip declaredlibcproto_i386

    .text
    .globl _start

_start:
    xor  %ebx, %ebx
    xor  %esi, %esi
1:
    push $slot
    push $0
    push %ebx
    push $2
    call wrapper
    add  $16, %esp
    add  %eax, %esi
    inc  %ebx
    cmp  $3, %ebx
    jbe  1b
    mov  %esi, %ebx
    mov  $1, %eax
    int  $0x80

wrapper:
    push %ebp
    mov  %esp, %ebp
    lea  8(%ebp), %eax
    mov  %eax, ap
    mov  ap, %edx
    mov  (%edx), %eax
    pop  %ebp
    ret

    .data
ap:
    .long 0
slot:
    .long 0
