// Function: sub_401000 @ 0x401000
unsigned long sub_401000(int a0) // warn: Function flows into unmapped memory
{
  if (a0)
    return 0x539;
  halt_missing(); // warn: Function flow reaches unmapped memory: r0x0040100c flows to r0x00000000
}

