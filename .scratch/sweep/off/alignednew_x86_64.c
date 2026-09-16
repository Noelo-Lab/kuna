// Function: callee @ 0x401000
long callee(long a0)
{
  return a0 + 1;
}

// Function: caller @ 0x401010
unsigned long caller(unsigned long a0)
{
  if (a0 < 0x1000) {
    if (a0)
      return callee(a0);
    return 0;
  }
  if (a0 < a0 + 0x27)
    return callee(a0 + 0x27);
  return 0;
}

// Function: _start @ 0x401050
void _start(void) // warn: Function flows into unmapped memory
{
  caller(0x40);
  syscall(0x3c,0);
  halt_missing(); // warn: Function flow reaches unmapped memory: r0x00401061 flows to r0x00401063
}

