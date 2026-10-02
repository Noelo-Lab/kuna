volatile int sink;
__attribute__((noinline)) int g(int x) { sink = x; return x * 3; }
static const signed char map[7] = {12, 10, 13, 10, 12, 21, 15};
int pick(unsigned mode) {
  if (mode > 6u) return 270;
  switch (map[mode]) {
    case 9: return g(2070) ^ 1;
    case 10: return g(2071) ^ 2;
    case 13: return g(2074) ^ 5;
    case 14: return g(2077) ^ 7;
    case 17: return g(2078) ^ 9;
    default: sink = 7; return 77;
  }
}
