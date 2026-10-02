volatile int sink;
__attribute__((noinline)) int g(int x) { sink = x; return x * 3; }
static const unsigned int map[5] = {2, 3, 6, 3, 10};
int pick(unsigned mode) {
  if (mode > 4u) return 270;
  switch (map[mode]) {
    case 0: return g(2070) ^ 1;
    case 1: return g(2071) ^ 2;
    case 2: return g(2072) ^ 3;
    case 3: return g(2073) ^ 4;
    case 4: return g(2074) ^ 5;
    default: sink = 7; return 77;
  }
}
