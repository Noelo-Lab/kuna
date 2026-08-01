/*
 * covercopy_x86_64 — regression for the two P6 cover-extension miscompilations.
 *
 *     gcc -O0 -no-pie -fno-pic -fno-stack-protector covercopy_x86_64.c -o covercopy_x86_64
 *
 * Both functions here used to emit C that computes a DIFFERENT VALUE than the
 * binary; both root causes are a Cover that the port built smaller than upstream
 * Ghidra builds it, so a legality test saw no conflict where one exists.
 *
 * 1. `lookup_service` (Merge::checkCopyPair, merge.cc:1121).  At -O0 every
 *    `return name;` compiles to `mov -N(%rbp),%rax ; jmp .Lepi`, so several COPYs
 *    of the same parameter feed one epilogue MULTIEQUAL and land in one
 *    HighVariable.  `markRedundantCopies` asked `checkCopyPair` whether the later
 *    COPY was redundant; the port built the dominance range from the dominant
 *    COPY's def point ALONE, omitting `range.addRefPoint(subOp, subOp->getIn(0))`,
 *    so the intervening `v = lookup(...)` write was never seen inside the range and
 *    the restore was marked non-printing.  The emitted C then returned the NULL
 *    from the failed lookup instead of the parameter.
 *
 * 2. `two_selects` (Merge::markImplied, merge.cc:1595-1605 + Varnode::setFlags,
 *    varnode.cc:371-380).  Two `cond ? t : 0` phis whose reads are both inlined
 *    into one call expression printed after both writes.  Marking a Varnode implied
 *    must dirty its operands' Covers, because `Cover::rebuild` walks forward through
 *    implied consumers — that is what stretches the first phi's Cover to the point
 *    where the expression is actually printed.  Without the dirtying (and without
 *    forwarding the Varnode `coverdirty` bit to the owning HighVariable) the two
 *    phis looked cover-disjoint, got speculatively merged into one variable, and the
 *    emitted C subtracted the second select's value twice.
 */
#include <stdlib.h>
#include <string.h>

struct svc { char *name; };

__attribute__((noinline)) struct svc *lookup(int port)
{
	static struct svc s = { (char *)"http" };
	return port == 80 ? &s : (struct svc *)0;
}

__attribute__((noinline)) int is_digit_c(int c)
{
	return c >= '0' && c <= '9';
}

/* Three `return name;` guards share one epilogue; the middle path clobbers the
 * return register with lookup()'s result and must reload `name` afterwards. */
__attribute__((noinline)) char *lookup_service(char *name, int *warned)
{
	int port;
	struct svc *sr;

	if (!is_digit_c(name[0]))
		return name;

	port = atoi(name);
	if (port == 0)
		return name;

	sr = lookup(port);
	if (sr == (struct svc *)0) {
		*warned = *warned + 1;
		return name;
	}
	return sr->name;
}

extern long g_a, g_b, g_c, g_d, g_step;
extern char g_flag1, g_flag2;
long g_a, g_b, g_c, g_d, g_step;
char g_flag1, g_flag2;

__attribute__((noinline)) void emit(long n);
__attribute__((noinline)) void emit(long n) { g_d = n; }

/* Two independent selects, both read inside one call argument. */
__attribute__((noinline)) void two_selects(void)
{
	emit(g_a - g_b - (g_flag1 ? g_step : 0) - g_c - (g_flag2 ? g_step : 0));
}

int main(void)
{
	int w = 0;
	two_selects();
	return lookup_service((char *)"99999", &w)[0];
}
