/* Fixture for DWARF enum recovery (matching IDA Pro / Ghidra).
 *
 * The decompiler has always been able to render an enum-typed constant by member
 * name; what was missing was the TYPE. The DWARF pass flattened every
 * `DW_TAG_enumeration_type` to its underlying integer, so an enum argument
 * printed as a bare number.
 *
 * `mode` is the ordinary case: an unsigned, int-width enum whose member is passed
 * as a constant. `level` covers the signed case with a NEGATIVE member, which
 * exercises the width-masking of the value map (`-1` in a 4-byte enum is keyed
 * `0xffffffff`, the value the constant Varnode actually carries).
 *
 * `apply` is noinline + non-static so the calls survive -O2 with their constant
 * arguments intact, and so the enum appears both as a callee parameter type and
 * at a call site.
 *
 * Build (DWARF required -> -g):
 *   gcc -O2 -g -fno-stack-protector -fcf-protection=none \
 *       -o dwarf_enums_x86_64 dwarf_enums_x86_64.c
 */

#include <stdio.h>

enum mode {
	mode_read = 0,
	mode_write = 1,
	mode_append = 2,
	mode_truncate = 3,
};

enum level {
	level_error = -1,
	level_quiet = 0,
	level_loud = 1,
};

__attribute__((noinline)) int apply(enum mode m, enum level l, int n)
{
	if (m == mode_append)
		return n + 1;
	if (l == level_error)
		return -n;
	return n;
}

int main(void)
{
	printf("%d\n", apply(mode_truncate, level_error, 7));
	return 0;
}
