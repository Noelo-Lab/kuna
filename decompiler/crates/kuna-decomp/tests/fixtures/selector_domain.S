/* SPDX-License-Identifier: Apache-2.0
 * Independently authored compact-selector dispatch. No external binary input.
 */
#ifndef INPUT_COUNT
#define INPUT_COUNT 256
#endif
#ifndef MAP_WIDTH
#define MAP_WIDTH 1
#endif
#ifndef GAP_BYTES
#define GAP_BYTES 0
#endif
#ifndef TRAILING_BYTES
#define TRAILING_BYTES 1024
#endif

.text
.set origin, dispatch-0x1000
.globl dispatch
.type dispatch,@function
dispatch:
#ifdef WINDOWS_ABI
    movl %ecx,%eax
#else
    movl %edi,%eax
#endif
    subl $1,%eax
    cmpl $(INPUT_COUNT-1),%eax
    ja .Ldefault
    leaq origin(%rip),%r8
    cltq
#if MAP_WIDTH == 1
    movzbl .Lmap-origin(%r8,%rax,1),%eax
#elif MAP_WIDTH == 2
    movzwl .Lmap-origin(%r8,%rax,2),%eax
#else
    movl .Lmap-origin(%r8,%rax,4),%eax
#endif
    movl .Ltable-origin(%r8,%rax,4),%ecx
    addq %r8,%rcx
    jmp *%rcx
.Lcase0:
    movl $11,%eax
    ret
.Lcase1:
    movl $22,%eax
    ret
.Lcase2:
    movl $33,%eax
    ret
.Ldefault:
    movl $-1,%eax
    ret
.size dispatch,.-dispatch
.p2align 2
.Ltable:
    .long .Lcase0-origin, .Lcase1-origin, .Lcase2-origin
    .fill GAP_BYTES,1,0xcc
.Lmap:
#if MAP_WIDTH == 1
    .byte 0,0,0,0,1,0,0,0,2
#elif MAP_WIDTH == 2
    .short 0,0,0,0,1,0,0,0,2
#else
    .long 0,0,0,0,1,0,0,0,2
#endif
    .fill (INPUT_COUNT-9),MAP_WIDTH,0
    /* Unrelated readable bytes expose excess table reads instead of an EOF. */
    .fill TRAILING_BYTES,1,0
.section .note.GNU-stack,"",@progbits
