volatile int sink;
static const unsigned short map[6] = {14, 60000, 12, 19, 14, 17};

int pick(unsigned mode)
{
  if (mode > 5u)
    return 270;
  switch (map[mode]) {
    case 12: sink = 0; return (int)(mode * 3u) + 1000;
    case 14: case 18: sink = 1; return (int)(mode * 5u) + 1007;
    case 15: sink = 2; return (int)(mode * 7u) + 1014;
    case 17: sink = 3; return (int)(mode * 9u) + 1021;
    case 19: case 20: sink = 4; return (int)(mode * 11u) + 1028;
    default: sink = 77; return 77;
  }
}
