/* Fixture for the uncomputed-return-value repair -- the NEGATIVE control.
 *
 * A 16-byte struct is returned in the RAX:RDX register pair under x86-64 SysV,
 * and that pair is REAL: `make` computes both halves. The repair that strips a
 * bogus register pair must not touch it.
 *
 * `passthru` is the second control: it returns a struct it received, so both
 * halves reach the RETURN through loads/copies rather than arithmetic -- the
 * shape closest to the leftover it must be distinguished from.
 *
 * Built WITHOUT -g on purpose: a DWARF prototype would lock the return type and
 * the recovery path this exercises would never run.
 *
 * Build:
 *   gcc -O2 -fno-stack-protector -fcf-protection=none \
 *       -o structreturn_x86_64 structreturn_x86_64.c
 */

#include <stdio.h>

struct pair {
	long a;
	long b;
};

__attribute__((noinline)) struct pair make(long x, long y)
{
	struct pair p;
	p.a = x * 3;
	p.b = y + 7;
	return p;
}

__attribute__((noinline)) struct pair passthru(struct pair *p)
{
	return *p;
}

int main(int argc, char **argv)
{
	struct pair p = make(argc, (long)argv[0][0]);
	struct pair q = passthru(&p);
	printf("%ld %ld\n", q.a, q.b);
	return 0;
}
