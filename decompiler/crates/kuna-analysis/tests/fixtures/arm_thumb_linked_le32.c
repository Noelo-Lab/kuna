/* Source for arm_thumb_linked_le32 — the LINKED ARM/Thumb decode e2e fixture.
 *
 * This is the linked-executable counterpart to the bare `arm_thumb_le32.o`
 * (ET_REL) symbol-scan fixture: a real ET_EXEC/ET_DYN with a PT_LOAD segment so
 * kuna's ObjectLoadImage (which reads only segments, never sections) can load it.
 * It proves the analysis-tier `arm_markers` TMode paint + Thumb-FUNC re-home
 * actually drive a correct Thumb decode through the full pipeline.
 *
 * Built in the kuna-dev container (arm-linux-gnueabihf-gcc 11.4.0) with:
 *   arm-linux-gnueabihf-gcc -mthumb -static -nostdlib -e _start \
 *       arm_thumb_linked_le32.c -o arm_thumb_linked_le32
 *
 * `-mthumb` forces Thumb codegen so the assembler lays the `$t` mapping symbol
 * and the linker records the STT_FUNC symbols at `entry|1` (the Thumb
 * odd-address convention). `compute` is deliberately non-trivial (x*3 + 7) so a
 * correct Thumb decode is visibly distinct from an ARM-mode misdecode (which
 * would yield garbage at the same byte stream).
 */
int compute(int x) {
    return x * 3 + 7;
}

void _start(void) {
    volatile int r = compute(5);
    (void)r;
    for (;;) {
    }
}
