/* Source for arm_thumb_switch_le32 -- the ARM/Thumb jump-table + callother
 * fixture (decbench NOVEL case "setISAMode leaks into emitted C").
 *
 * Built in the dev container (`docs/devcontainer.md`) with:
 *   arm-linux-gnueabihf-gcc -mthumb -march=armv7-a -Os -static -nostdlib \
 *       -e _start arm_thumb_switch_le32.c -o arm_thumb_switch_le32
 *
 * `dispatch` compiles to a Thumb-2 `tbb [pc,rN]` jump table whose case blocks
 * each contain a `bl`.  Both instructions lower through the SLEIGH
 * `SetThumbMode` -> `setISAMode` user-op, which `ARM.cspec`'s
 * `<callotherfixup targetop="setISAMode">` declares to be a NOP, so no
 * `setISAMode` may survive into the emitted C.
 */
__attribute__((noinline)) int f0(int x) { return x + 3; }
__attribute__((noinline)) int f1(int x) { return x * 5; }
__attribute__((noinline)) int f2(int x) { return x - 7; }
__attribute__((noinline)) int f3(int x) { return x ^ 9; }

int dispatch(int sel, int x) {
    switch (sel) {
    case 0: return f0(x);
    case 1: return f1(x);
    case 2: return f2(x);
    case 3: return f3(x);
    case 4: return f0(x) + f1(x);
    case 5: return f2(x) + f3(x);
    case 6: return f0(x) - f2(x);
    case 7: return f1(x) - f3(x);
    default: return 0;
    }
}

void _start(void) {
    volatile int r = dispatch(2, 11);
    (void)r;
    for (;;) {
    }
}
