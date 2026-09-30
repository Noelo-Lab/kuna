/* (kuna) The static `shared` of elfaliasclash_x86_64.c. */
static __attribute__((noinline)) int shared(int x) { return x - 5; }
int use_b(int y) { return shared(y) * 2; }
