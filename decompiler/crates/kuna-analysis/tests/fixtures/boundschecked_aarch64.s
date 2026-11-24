// Callers of the C11 Annex K bounds-checked functions, as GCC 15 -O2 -fPIC
// compiles them for AArch64. Every callee is an undefined import.
//
//   void *clear_obj(void)            { void *p = alloc(64); if (p) memset_s(p, 64, 0, 64); return p; }
//   void *dup_obj(const void *s)     { void *p = alloc(32); if (p) memcpy_s(p, 32, s, 32); return p; }
//   void *shift_obj(const void *s)   { void *p = alloc(32); if (p) memmove_s(p, 32, s, 32); return p; }
//   char *dup_name(const char *s)    { char *p = alloc(16); if (p) strncpy_s(p, 16, s, 16); return p; }
//   char *add_name(char *p, const char *s) { if (p) strncat_s(p, 16, s, 16); return p; }
//   char *join(char *p, const char *a, const char *b)
//                                    { if (strcpy_s(p, 16, a) == 0) strcat_s(p, 16, b); return p; }
//   char *format_pair(char *p, int a, int b) { sprintf_s(p, 32, "%d-%d", a, b); return p; }
//
// The four-argument calls load the shared size into x3 and copy it into x1.

 .text

 .globl clear_obj
 .type clear_obj,%function
clear_obj:
 stp x29, x30, [sp, #-32]!
 mov x0, #64
 mov x29, sp
 bl alloc
 mov x4, x0
 cbz x0, 1f
 mov x3, #64
 mov w2, #0
 mov x1, x3
 str x0, [sp, #24]
 bl memset_s
 ldr x4, [sp, #24]
1:
 mov x0, x4
 ldp x29, x30, [sp], #32
 ret
 .size clear_obj, .-clear_obj

 .globl dup_obj
 .type dup_obj,%function
dup_obj:
 stp x29, x30, [sp, #-48]!
 mov x29, sp
 str x19, [sp, #16]
 mov x19, x0
 mov x0, #32
 bl alloc
 mov x4, x0
 cbz x0, 1f
 mov x3, #32
 mov x2, x19
 mov x1, x3
 str x0, [sp, #40]
 bl memcpy_s
 ldr x4, [sp, #40]
1:
 mov x0, x4
 ldr x19, [sp, #16]
 ldp x29, x30, [sp], #48
 ret
 .size dup_obj, .-dup_obj

 .globl shift_obj
 .type shift_obj,%function
shift_obj:
 stp x29, x30, [sp, #-48]!
 mov x29, sp
 str x19, [sp, #16]
 mov x19, x0
 mov x0, #32
 bl alloc
 mov x4, x0
 cbz x0, 1f
 mov x3, #32
 mov x2, x19
 mov x1, x3
 str x0, [sp, #40]
 bl memmove_s
 ldr x4, [sp, #40]
1:
 mov x0, x4
 ldr x19, [sp, #16]
 ldp x29, x30, [sp], #48
 ret
 .size shift_obj, .-shift_obj

 .globl dup_name
 .type dup_name,%function
dup_name:
 stp x29, x30, [sp, #-48]!
 mov x29, sp
 str x19, [sp, #16]
 mov x19, x0
 mov x0, #16
 bl alloc
 mov x4, x0
 cbz x0, 1f
 mov x3, #16
 mov x2, x19
 mov x1, x3
 str x0, [sp, #40]
 bl strncpy_s
 ldr x4, [sp, #40]
1:
 mov x0, x4
 ldr x19, [sp, #16]
 ldp x29, x30, [sp], #48
 ret
 .size dup_name, .-dup_name

 .globl add_name
 .type add_name,%function
add_name:
 cbz x0, 1f
 stp x29, x30, [sp, #-32]!
 mov x2, x1
 mov x3, #16
 mov x29, sp
 mov x1, x3
 str x0, [sp, #24]
 bl strncat_s
 ldr x4, [sp, #24]
 ldp x29, x30, [sp], #32
 mov x0, x4
 ret
1:
 ret
 .size add_name, .-add_name

 .globl join
 .type join,%function
join:
 stp x29, x30, [sp, #-32]!
 mov x29, sp
 stp x19, x20, [sp, #16]
 mov x20, x2
 mov x19, x0
 mov x2, x1
 mov x1, #16
 bl strcpy_s
 cbz w0, 1f
 mov x0, x19
 ldp x19, x20, [sp, #16]
 ldp x29, x30, [sp], #32
 ret
1:
 mov x2, x20
 mov x0, x19
 mov x1, #16
 bl strcat_s
 mov x0, x19
 ldp x19, x20, [sp, #16]
 ldp x29, x30, [sp], #32
 ret
 .size join, .-join

 .globl format_pair
 .type format_pair,%function
format_pair:
 stp x29, x30, [sp, #-32]!
 mov w4, w2
 mov w3, w1
 mov x29, sp
 mov x1, #32
 adrp x2, .Lpair
 add x2, x2, :lo12:.Lpair
 str x19, [sp, #16]
 mov x19, x0
 bl sprintf_s
 mov x0, x19
 ldr x19, [sp, #16]
 ldp x29, x30, [sp], #32
 ret
 .size format_pair, .-format_pair

 .section .rodata.str1.8,"aMS",@progbits,1
.Lpair:
 .string "%d-%d"
