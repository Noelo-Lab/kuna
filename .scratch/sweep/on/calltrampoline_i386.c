// Function: sub_8049000 @ 0x8049000
void sub_8049000(void)
{
  protected(7);
  do {
  } while( true );
}

// Function: protected @ 0x804900b
int protected(int a0)
{
  return a0 + 0x6e; // warn: calltrampoline: 0x0804901c discards the return address -- following this call as a branch
}

// Function: tramp @ 0x804901c
int tramp(int a0)
{
  return a0 + 0x6e;
}

