volatile int sink;
static const unsigned char map[5] = {240, 1, 241, 241, 241};
int pick(unsigned mode) { if (mode > 4u) return 270; switch (map[mode]) {
  case 240: sink = 0; return (int)(mode * 3u) + 1000;
  case 242: case 244: sink = 1; return (int)(mode * 5u) + 1007;
  case 243: sink = 2; return (int)(mode * 7u) + 1014;
  case 246: sink = 3; return (int)(mode * 9u) + 1021;
  default: sink = 77; return 77; } }
