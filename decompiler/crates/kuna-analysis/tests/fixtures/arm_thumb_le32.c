/* Source for arm_thumb_le32.o — the ARM/Thumb mapping-symbol fixture.
 *
 * Built (off any ARM linker; this host has none) with:
 *   clang --target=arm-linux-gnueabihf -mthumb -nostdlib -c \
 *       arm_thumb_le32.c -o arm_thumb_le32.o
 *
 * Emits a bare ET_REL object whose `.symtab` carries both
 * ArmSymbolAnalyzer/ARM_ElfExtension mechanisms:
 *   - the `$t.0` Thumb mapping symbol at .text+0x0
 *   - STT_FUNC symbols with LSB-set st_value (`thumb_add`@0x1, `_start`@0x15),
 *     the odd-address Thumb convention.
 *
 * The two `__attribute__((target("thumb")))` functions force Thumb codegen so
 * the assembler lays the `$t` mapping symbol. Object only — a LINKED ARM exe
 * (ET_EXEC/ET_DYN with PT_LOAD) for the decode e2e is a documented follow-up
 * (no ARM linker on the build host: no lld, and gold/mold are x86-only here).
 */
__attribute__((target("thumb"))) int thumb_add(int a, int b) {
    return a + b * 3;
}
__attribute__((target("thumb"))) void _start(void) {
    volatile int r = thumb_add(7, 11);
    (void)r;
    for (;;) {
    }
}
