volatile int sink;
static const unsigned short map[7] = {2, 3, 6, 11, 3, 14, 10};

int pick(unsigned mode)
{
  if (mode > 6u)
    return 270;
  switch (map[mode]) {
    case 2: case 8: sink = 0; return (int)(mode * 3u) + 1000;
    case 4: case 10: sink = 1; return (int)(mode * 5u) + 1007;
    case 6: sink = 2; return (int)(mode * 7u) + 1014;
    case 3: sink = 3; return (int)(mode * 9u) + 1021;
    case 0: sink = 4; return (int)(mode * 11u) + 1028;
    default: sink = 77; return 77;
  }
}
