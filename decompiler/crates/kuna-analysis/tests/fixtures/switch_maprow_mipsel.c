volatile int sink;
static const unsigned short map[5] = {12, 1, 13, 14, 12};

int pick(unsigned mode)
{
  if (mode > 4u)
    return 270;
  switch (map[mode]) {
    case 14: sink = 0; return (int)(mode * 3u) + 1000;
    case 8: case 13: sink = 1; return (int)(mode * 5u) + 1007;
    case 12: sink = 2; return (int)(mode * 7u) + 1014;
    case 1: sink = 3; return (int)(mode * 9u) + 1021;
    default: sink = 77; return 77;
  }
}
