// Source of chkstkdarwin_macho_arm64 (recipe in chkstkdarwin_macho_arm64.provenance).
// Apple clang calls ___chkstk_darwin, with the byte count in x9, before it
// allocates a frame larger than a page or a variable-length array. The probe
// preserves the argument registers, and the code after it reads them.

// A 20,000-byte frame: k stays in w0 across the probe.
__attribute__((noinline)) int big_frame(int k) {
    volatile char big[20000];
    big[k] = (char)k;
    return big[k / 2] + 3;
}

// A variable-length array: i stays in w0 and n in x1 across the probe.
__attribute__((noinline)) int vla_last(int i, long n) {
    volatile char buf[n];
    buf[i] = 7;
    return buf[n - 1];
}

// The same probe called directly, through a __stubs entry.
__asm__(".globl _probe_stub\n"
        ".p2align 2\n"
        "_probe_stub:\n"
        "    stp x29, x30, [sp, #-16]!\n"
        "    mov x29, sp\n"
        "    mov x9, #0x2000\n"
        "    bl ___chkstk_darwin\n"
        "    sub sp, sp, #0x2000\n"
        "    add x0, x0, x1\n"
        "    add sp, sp, #0x2000\n"
        "    ldp x29, x30, [sp], #16\n"
        "    ret\n");
long probe_stub(long a, long b);

int main(int argc, char **argv) {
    (void)argv;
    return big_frame(argc) + vla_last(argc, argc * 9000L) + (int)probe_stub(argc, 2);
}
