# (kuna) Option `funcchunks`. Two functions share one error tail that sits
# after a third function: `f` and `g` both jump to `shared`, which lies past
# `h`. The [entry, next) clip gives `shared` to `h`; the chunk model gives it
# to `f` and `g` as a shared tail and keeps `h` to its own two instructions.
# Build: as funcchunks_x86_64.s -o t.o && ld -o funcchunks_x86_64 t.o
    .text
    .globl _start
    .type _start, @function
_start:
    xor  %edi,%edi
    call f
    call g
    call h
    mov  $60,%eax
    syscall
    hlt

    .type f, @function
f:
    test %rdi,%rdi
    je   shared
    mov  $1,%eax
    ret

    .type g, @function
g:
    cmp  $2,%rdi
    jne  shared
    mov  $2,%eax
    ret

    .type h, @function
h:
    mov  $3,%eax
    ret

shared:
    mov  $-1,%eax
    ret
