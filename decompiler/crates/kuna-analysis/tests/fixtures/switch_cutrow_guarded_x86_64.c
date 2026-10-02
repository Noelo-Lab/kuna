volatile int sink;
__attribute__((noinline)) int g(int x) { sink = x; return x * 3; }
static const unsigned short map[6] = {15, 17, 9, 30, 18, 19};

int pick(unsigned mode)
{
  if (mode - 16u < 7u) {
    switch (mode - 16u) {
      case 0: shared: return g(1000);
      case 2: return g(501) + 1;
      case 3: return g(502) * 2;
      case 5: return g(503) - 7;
      default: sink = 7; return 77;
    }
  }
  if (mode > 5u)
    return 270;
  switch (map[mode]) {
    case 9: goto shared;
    case 15: return g(1001) + 3;
    case 4: return g(1002) ^ 5;
    case 17: case 13: return g(1003) * 5;
    default: sink = 7; return 77;
  }
}
