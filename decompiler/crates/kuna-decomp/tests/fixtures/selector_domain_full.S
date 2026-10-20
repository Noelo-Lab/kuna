/* SPDX-License-Identifier: Apache-2.0
 * Full 256-value byte selector: every map value has one real target. */
.text
.set origin, dispatch-0x1000
.globl dispatch
.type dispatch,@function
dispatch:
    movl %edi,%eax
    subl $1,%eax
    cmpl $255,%eax
    ja .Ldefault
    leaq origin(%rip),%r8
    cltq
    movzbl .Lmap-origin(%r8,%rax,1),%eax
    movl .Ltable-origin(%r8,%rax,4),%ecx
    addq %r8,%rcx
    jmp *%rcx
.Ldefault:
    movl $-1,%eax
    ret
.p2align 2
.altmacro
.Ltable:
.macro table_entry n
    .long .Lcase\n-origin
.endm
.set i,0
.rept 256
    table_entry %i
    .set i,i+1
.endr
.Lmap:
.set i,0
.rept 256
    .byte i
    .set i,i+1
.endr
.macro emit_case n
.Lcase\n:
    movl $\n,%eax
    ret
.endm
.set i,0
.rept 256
    emit_case %i
    .set i,i+1
.endr
.size dispatch,.-dispatch
.section .note.GNU-stack,"",@progbits
