volatile int sink;
__attribute__((noinline)) int g(int x) { sink = x; return x * 3; }
static const unsigned char map[7] = {7, 0, 5, 19, 10, 1, 0};
int pick(unsigned mode) {
  if (mode > 6u) return 270;
  switch (map[mode]) {
    case 0: return g(2070) ^ 1;
    case 4: return g(2074) ^ 5;
    case 8: return g(2078) ^ 9;
    case 7:
    case 1:
      return g(3007) + 11;
    default: sink = 7; return 77;
  }
}
