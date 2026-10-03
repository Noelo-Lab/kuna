volatile int sink;
static const unsigned char map[6] = {208, 204, 203, 213, 217, 206};
int pick(unsigned mode) { if (mode > 5u) return 270; switch (map[mode]) {
  case 205: case 211: sink = 0; return (int)(mode * 3u) + 1000;
  case 204: sink = 1; return (int)(mode * 5u) + 1007;
  case 206: sink = 2; return (int)(mode * 7u) + 1014;
  case 203: case 212: case 213: sink = 3; return (int)(mode * 9u) + 1021;
  case 200: case 207: case 208: sink = 4; return (int)(mode * 11u) + 1028;
  default: sink = 77; return 77; } }
