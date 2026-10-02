volatile int sink;
static const unsigned short map[6] = {13, 15, 9, 4, 13, 17};

int pick(unsigned mode)
{
  if (mode > 5u)
    return 270;
  switch (map[mode]) {
    case 9: sink = 0; return 1000;
    case 15: sink = 1; return 1001;
    case 4: sink = 2; return 1002;
    case 17: case 13: sink = 3; return 1003;
    default: sink = 7; return 77;
  }
}
