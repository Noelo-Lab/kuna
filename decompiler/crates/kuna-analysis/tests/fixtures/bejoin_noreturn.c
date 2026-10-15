/* int functions with a path that ends in a call that does not return.  gcc
 * -Os computes chk's exit argument from b + 1, made in the delay slot of the
 * branch to the RETURN (`beqz a0; addiu v1,a1,1`), so `return 0` finds b + 1
 * in $3 beside the zero in $2, the registers a zero-extended long long comes
 * back in; only the exit path reads it.  zk returns a long long product made
 * in the same kind of delay slot beside an abort path.  The stubs keep the
 * calls out of line in a linked image (an object file leaves exit unresolved). */
typedef unsigned long long u64;
extern void exit(int) __attribute__((noreturn));
extern void abort(void) __attribute__((noreturn));
int chk(int a, int b, int c) { if (a) exit(((c - b) ^ (a * (b + 1))) - c * (b + 1)); return 0; }
int chk2(int a, int b, int c) { if (a == c) exit((b + 1) * c); return 0; }
u64 zk(int a, unsigned b, unsigned c) { if (a) abort(); return (u64)b * c; }
#ifdef LINK
int gi;
__attribute__((noipa)) void exit(int v) { for (;;) gi = v; }
__attribute__((noipa)) void abort(void) { for (;;) gi = 1; }
void __start(void) {}
#endif
