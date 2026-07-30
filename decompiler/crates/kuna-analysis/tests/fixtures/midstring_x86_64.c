/* Fixture for mid-string literal recovery (matching IDA Pro / Ghidra).
 *
 * A compiler that merges string constants (-fmerge-constants, on at -O2) stores
 * a literal that is a SUFFIX of another literal only once, and points at the
 * interior of the longer one. `"coreutils"` below is the tail of
 * `"GNU coreutils"`, so the second `sink` call gets `&banner[4]`, not its own
 * `.rodata` object.
 *
 * Before the fix, kuna typed a constant pointer only when it hit the START of a
 * detected string; an interior pointer fell back to the bare integer, so a real
 * coreutils binary rendered `bindtextdomain(0x68d8, ...)` where IDA shows
 * `bindtextdomain("coreutils", ...)`.
 *
 * `numbers` is the NEGATIVE control: a read-only NON-character array. A constant
 * pointing into it must keep rendering as a number -- the guard is that the
 * covering symbol's element type is character-printable, and an `int` array's is
 * not.
 *
 * `sink`/`consume` are noinline + non-static so the calls survive -O2 with their
 * constant arguments intact.
 *
 * Build (no DWARF, so this exercises the *type* path, not the prototype path):
 *   gcc -O2 -fno-stack-protector -fcf-protection=none \
 *       -o midstring_x86_64 midstring_x86_64.c
 */

#include <stdio.h>

const char banner[] = "GNU coreutils";
const int numbers[8] = { 11, 22, 33, 44, 55, 66, 77, 88 };

__attribute__((noinline)) void sink(const char *s)
{
	puts(s);
}

__attribute__((noinline)) void consume(const int *p)
{
	printf("%d\n", *p);
}

int main(void)
{
	sink("GNU coreutils"); /* start of the merged literal */
	sink("coreutils");     /* INTERIOR: &banner[4] */
	consume(&numbers[3]);  /* interior of a NON-char array: stays numeric */
	return 0;
}
