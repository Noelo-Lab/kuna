// Function: classify @ 0x8049000
unsigned int classify(int a0) // early-return
{
  unsigned int v1; // eax
  
  if (a0) // branch-flip
    v1 = helper(2,1);
  else {
    v1 = 0x1234;
  }
  if (!v1)
    return 0;
  return v1 | 7;
}

// Function: helper @ 0x8049027
int helper(int a0)
{
  return a0 + 0x5a5a5a;
}

// Function: _start @ 0x8049031
void _start(void) // warn: Function flows into unmapped memory
{
  classify(0x35);
  (*(void *)swi(0x80))();
  halt_missing(); // warn: Function flow reaches unmapped memory: r0x08049042 flows to r0x08049044
}

