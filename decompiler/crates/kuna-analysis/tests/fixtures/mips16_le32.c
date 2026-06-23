/* MIPS16 ISA_MODE fixture (freestanding, -nostdlib).

   `m16_square` is compiled as MIPS16 (the `mips16` function attribute), so the
   linker records its STT_FUNC symbol at `entry|1` (the LSB-set odd-address
   convention, exactly like ARM Thumb).  Everything else stays MIPS32
   (`nomips16`).  The kuna `mips_isa` pass paints ISA_MODE=1 at the (even) entry
   so the body decodes as MIPS16 instead of being misdecoded as MIPS32.

   Freestanding (no libc): the container has the MIPS *runtime* libc but no
   `libc6-dev` (no crt1.o / headers), so the fixture supplies its own `_start`
   and never calls into libc.  We only need a decodable MIPS16 function body, so
   no runtime is required.

   Built with mips-linux-gnu-gcc (big-endian; the `_le32` name follows the
   sibling mips_gp fixture's convention — endianness is recorded in the ELF
   header and read by the loader). */

/* A genuinely MIPS16-encoded leaf: arithmetic the decompiler must recover.  The
   `mips16` attribute forces MIPS16 ISA for this function, so its FUNC symbol is
   recorded at entry|1. */
int __attribute__((mips16, noinline)) m16_square(int n) {
    return n * n + 3;
}

/* MIPS32 caller, kept out of MIPS16 so the file mixes the two ISAs. */
int __attribute__((nomips16, noinline)) m32_caller(int n) {
    return m16_square(n) + 1;
}

/* Freestanding entry: no libc.  Calls the MIPS16 leaf so it is reachable and
   not garbage-collected. */
void __attribute__((nomips16)) _start(void) {
    volatile int r = m32_caller(7);
    (void)r;
    for (;;) {
    }
}
