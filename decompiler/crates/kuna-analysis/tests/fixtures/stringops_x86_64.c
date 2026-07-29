/* Fixture for the direction-flag guarantee (matching IDA Pro).
 *
 * gcc inlines a short fixed-length `strcmp` as `repe cmpsb`, and every x86 string
 * instruction steps its pointers by `1 - 2*DF` -- forward when the direction flag
 * is clear, backward when it is set. SLEIGH lowers that faithfully, so before the
 * fix the decompiled loop carried the flag as a live variable and scaled every
 * step by it:
 *
 *     uint1 v8;                          // df
 *     v8 = 0;
 *     v5 = &v6[(uint8)v8 * -2 + 1];      // instead of &v6[1]
 *
 * The flag IS pinned to 0 -- the processor spec tracks it at function entry --
 * but the gcc compiler spec omits DF from the registers a call leaves alone, so
 * SSA construction plants `DF = INDIRECT(DF, <call>)` at every call and the
 * constant never reaches the stride. `sink` is called before the comparison
 * precisely to place such a call in the way.
 *
 * `pick` returns 1 for the exact two-byte string "-" and 0 otherwise, which is
 * the shape gcc turns into `repe cmpsb` at -O2 (the same idiom coreutils `fmt`
 * uses to detect the stdin argument).
 *
 * Built WITHOUT -g: this exercises the prototype model, not DWARF.
 *
 * Build:
 *   gcc -O2 -fno-stack-protector -fcf-protection=none \
 *       -o stringops_x86_64 stringops_x86_64.c
 */

#include <stdio.h>
#include <string.h>

__attribute__((noinline)) void sink(const char *s)
{
	puts(s);
}

__attribute__((noinline)) int pick(const char *name)
{
	sink(name);
	return strcmp(name, "-") == 0;
}

int main(int argc, char **argv)
{
	printf("%d\n", pick(argc > 1 ? argv[1] : "x"));
	return 0;
}
