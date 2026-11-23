@ Callers of memset_s and memcpy_s, as GCC 15 -O2 -fPIC compiles them for
@ 32-bit ARM in Thumb-2 (arm-linux-gnueabihf). Both callees are undefined imports.
@
@   void *clear_obj(void)        { void *p = alloc(64); if (p) memset_s(p, 64, 0, 64); return p; }
@   void *dup_obj(const void *s) { void *p = alloc(32); if (p) memcpy_s(p, 32, s, 32); return p; }
@
@ Each call loads the shared size into r3 and copies it into r1.

 .syntax unified
 .arch armv7-a
 .thumb
 .text

 .globl clear_obj
 .type clear_obj,%function
 .thumb_func
clear_obj:
 push {r4, lr}
 movs r0, #64
 bl alloc
 mov r4, r0
 cbz r0, 1f
 movs r3, #64
 movs r2, #0
 mov r1, r3
 bl memset_s
1:
 mov r0, r4
 pop {r4, pc}
 .size clear_obj, .-clear_obj

 .globl dup_obj
 .type dup_obj,%function
 .thumb_func
dup_obj:
 push {r3, r4, r5, lr}
 mov r5, r0
 movs r0, #32
 bl alloc
 mov r4, r0
 cbz r0, 1f
 movs r3, #32
 mov r2, r5
 mov r1, r3
 bl memcpy_s
1:
 mov r0, r4
 pop {r3, r4, r5, pc}
 .size dup_obj, .-dup_obj
