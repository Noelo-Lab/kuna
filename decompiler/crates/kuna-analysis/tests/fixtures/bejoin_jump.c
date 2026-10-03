/* gcc copies `len` into $3 before the switch for the case that passes it on,
 * and the jump table, whose entries the object leaves to relocations, is not
 * recovered: the default path returns "???" in $2 with that copy beside it.
 * zb returns its second argument zero-extended: the same two registers. */
typedef unsigned long long u64;
extern const char *g(int, const void *, char *, int);
const char *pick(int af, int len, const void *addr, char *buf, int buflen)
{
	switch (af) {
	case 0: return g(1, addr, buf, buflen);
	case 2: return g(2, addr, buf, buflen);
	case 3: return g(3, addr, buf, len);
	case 7: return g(7, addr, buf, buflen);
	case 10: return g(10, addr, buf, buflen);
	case 28: return g(28, addr, buf, buflen);
	default: return "???";
	}
}
u64 zb(unsigned a, unsigned b) { return b; }
