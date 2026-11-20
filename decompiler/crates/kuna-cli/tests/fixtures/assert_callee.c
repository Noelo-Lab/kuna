// Fixture for the `callee` assertion: an indirect tail jump, an indirect call
// and a direct call.
// clang -O2 -fPIC -fno-inline -fcf-protection=full -c assert_callee.c -o assert_callee.o
// ld.lld -shared -o assert_callee_x86_64.elf assert_callee.o
// (clang and LLD 22.1.8; version 21 gives the same bytes outside .comment)
struct ops { int (*fn)(void *ctx, int a); };
struct ops *table[4];
int handle_open(void *ctx, int a) { return *(int *)ctx + a; }
int dispatch(void *ctx, int i, int a) { return table[i & 3]->fn(ctx, a); }
int twice(void *ctx, int i, int a) { return table[i & 3]->fn(ctx, a) + handle_open(ctx, 1); }
