/* Fixture source for `xrefiplt_arm_le32`: a stripped static ARM ELF mapped at
   0x10000 whose one import is a GNU IFUNC (`pick`, resolved by
   `resolve_pick`). A static link puts its stub in `.iplt`, in the same A32
   template as a `.plt` entry (`add ip,pc,#0,12; add ip,ip,#N,20;
   ldr pc,[ip,#M]!`), jumping through its `.got` slot. musl has no IFUNC, so
   the resolver is declared in assembly.

   Built with GCC 14.2 for armhf musl (homebrew `musl-cross` 0.9.11); the
   SHA-256 below is that toolchain's output:

       arm-linux-musleabihf-gcc -march=armv7-a+fp -nostdlib -static \
           -Wl,--build-id=none -s \
           -o xrefiplt_arm_le32 xrefiplt_arm_le32.S

   SHA-256 10d395e9877830a902ad724f910d50b5994f4b20ccb623380f0979bda48f253c. */

    .syntax unified
    .text
    .arm
    .global _start
_start:
    bl      pick
    mov     r7, #1
    svc     #0

impl:
    mov     r0, #7
    bx      lr

resolve_pick:
    adr     r0, impl
    bx      lr

    .global pick
    .type   pick, %gnu_indirect_function
    .set    pick, resolve_pick
