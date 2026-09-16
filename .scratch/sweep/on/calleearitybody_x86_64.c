// Function: callee @ 0x401000
unsigned int callee(unsigned int *a0)
{
  return *a0;
}

// Function: caller @ 0x401010
unsigned long caller(long a0)
{
  if (a0)
    return callee(a0);
  return 0;
}

// Function: _start @ 0x401020
void _start(void) // warn: Function flows into unmapped memory
{
  syscall(0x3c,(unsigned long)(unsigned int)caller());
  halt_missing(); // warn: Function flow reaches unmapped memory: r0x0040102f flows to r0x00401031
}

