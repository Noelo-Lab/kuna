// A library that defines its own memset_s and strcpy_s, each with a parameter
// list of its own, exports them, and calls them through their PLT stubs, as
// clang 21 -O2 -fPIC -fno-optimize-sibling-calls compiles it for AArch64:
//
//   void memset_s(void *d, int c) { ((char *)d)[0] = c; sink(d); }
//   int strcpy_s(char *d, const char *s) { int i = 0; while ((d[i] = s[i])) i++; return i; }
//   void *clear_obj(void) { void *p = alloc(64); if (p) memset_s(p, 64); return p; }
//   int copy_name(char *d, const char *s) { return strcpy_s(d, s) + 1; }

 .text

 .globl memset_s
 .p2align 2
 .type memset_s,@function
memset_s:
 stp x29, x30, [sp, #-16]!
 mov x29, sp
 strb w1, [x0]
 bl sink
 ldp x29, x30, [sp], #16
 ret
 .size memset_s, .-memset_s

 .globl strcpy_s
 .p2align 2
 .type strcpy_s,@function
strcpy_s:
 mov x8, x0
 mov w0, #-1
1:
 ldrb w9, [x1], #1
 add w0, w0, #1
 strb w9, [x8], #1
 cbnz w9, 1b
 ret
 .size strcpy_s, .-strcpy_s

 .globl clear_obj
 .p2align 2
 .type clear_obj,@function
clear_obj:
 stp x29, x30, [sp, #-32]!
 mov w0, #64
 str x19, [sp, #16]
 mov x29, sp
 bl alloc
 cbz x0, 1f
 mov w1, #64
 mov x19, x0
 bl memset_s
 mov x0, x19
1:
 ldr x19, [sp, #16]
 ldp x29, x30, [sp], #32
 ret
 .size clear_obj, .-clear_obj

 .globl copy_name
 .p2align 2
 .type copy_name,@function
copy_name:
 stp x29, x30, [sp, #-16]!
 mov x29, sp
 bl strcpy_s
 add w0, w0, #1
 ldp x29, x30, [sp], #16
 ret
 .size copy_name, .-copy_name

 .section .note.GNU-stack,"",@progbits
