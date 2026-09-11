# (kuna) `RDTSC` x86-64 zero-extension fixture -- RE-friction round 11,
# need `rdtsc-timestamp-incorporates-stale`.
#
# Both 64-bit destination registers start with non-zero upper halves.  RDTSC
# replaces EDX:EAX and clears RDX[63:32] and RAX[63:32]; recombining the two
# 32-bit timestamp halves must therefore return exactly rdtsc(), independent of
# the seeded values.
#
# Built with:  as -o rdtsc_zero_extend_x86_64.o rdtsc_zero_extend_x86_64.s \
#           && ld -o rdtsc_zero_extend_x86_64 rdtsc_zero_extend_x86_64.o
        .text
        .globl timestamp
        .type timestamp, @function
timestamp:
        .byte 0x48,0xb8,0x88,0x77,0x66,0x55,0x44,0x33,0x22,0x11
        .byte 0x48,0xba,0x00,0xff,0xee,0xdd,0xcc,0xbb,0xaa,0x99
        .byte 0x0f,0x31
        .byte 0x48,0xc1,0xe2,0x20
        .byte 0x48,0x09,0xd0
        .byte 0xc3
        .size timestamp, .-timestamp
