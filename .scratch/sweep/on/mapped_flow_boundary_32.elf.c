// Function: sub_10000 @ 0x10000
unsigned int sub_10000(int a0) // warn: Function flows into unmapped memory
{
  if (!a0)
    return 7;
  halt_missing(); // warn: Function flow reaches unmapped memory: r0x0001000a flows to r0x0001000f
}

