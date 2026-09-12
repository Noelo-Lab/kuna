# function_boundary_return_x86_64: a discovered one-instruction RET which is
# both separately callable and the shared epilogue of the function before it.
#
# `shared_wrapper` reaches `callable_ret` only by fall-through from `.Lepilogue`.
# Since `callable_ret` is a real STT_FUNC entry, funcboundflow used to truncate
# before its RET and lose both return paths.  The function immediately after the
# RET returns 33, making any overrun visible.
#
# `ordinary_first` is the adversarial control: it falls directly into a distinct
# function whose entry is an ordinary MOV.  Default funcboundflow must still cut
# there; admitting every foreign entry would change its result from 11 to 22.
# The two `before_*_branch` pairs pin the other tempting over-broad exception:
# direct and computed branches terminate an instruction but can lead into more
# code, so neither is safe to share.
#
# Built with:
#   as -o function_boundary_return_x86_64.o function_boundary_return_x86_64.s
#   ld -o function_boundary_return_x86_64 function_boundary_return_x86_64.o

        .text
        .globl shared_wrapper
        .type shared_wrapper, @function
shared_wrapper:
        mov     $7, %eax
        test    %edi, %edi
        je      .Lepilogue
        mov     $-1, %eax
.Lepilogue:
        nop
        .size shared_wrapper, .-shared_wrapper

        .globl callable_ret
        .type callable_ret, @function
callable_ret:
        ret
        .size callable_ret, .-callable_ret

        .globl after_ret
        .type after_ret, @function
after_ret:
        mov     $33, %eax
        ret
        .size after_ret, .-after_ret

        .globl ordinary_first
        .type ordinary_first, @function
ordinary_first:
        mov     $11, %eax
        .size ordinary_first, .-ordinary_first

        .globl ordinary_next
        .type ordinary_next, @function
ordinary_next:
        mov     $22, %eax
        ret
        .size ordinary_next, .-ordinary_next

        .globl before_direct_branch
        .type before_direct_branch, @function
before_direct_branch:
        mov     $55, %eax
        .size before_direct_branch, .-before_direct_branch

        .globl direct_branch_entry
        .type direct_branch_entry, @function
direct_branch_entry:
        jmp     after_ret
        .size direct_branch_entry, .-direct_branch_entry

        .globl before_indirect_branch
        .type before_indirect_branch, @function
before_indirect_branch:
        mov     $66, %eax
        .size before_indirect_branch, .-before_indirect_branch

        .globl indirect_branch_entry
        .type indirect_branch_entry, @function
indirect_branch_entry:
        jmp     *%rax
        .size indirect_branch_entry, .-indirect_branch_entry

        .globl ret_caller
        .type ret_caller, @function
ret_caller:
        call    callable_ret
        mov     $44, %eax
        ret
        .size ret_caller, .-ret_caller

        .globl _start
        .type _start, @function
_start:
        xor     %edi, %edi
        call    shared_wrapper
        call    ordinary_first
        call    before_direct_branch
        call    before_indirect_branch
        call    ret_caller
        mov     %eax, %edi
        mov     $60, %eax
        syscall
        .size _start, .-_start
