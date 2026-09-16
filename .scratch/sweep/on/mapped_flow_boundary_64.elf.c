// Function: sub_10000 @ 0x10000
unsigned long sub_10000(void) // warn: Function flows into unmapped memory
{
  int v1; // eax
  
  if (!v1)
    return 7;
  halt_missing(); // warn: Function flow reaches unmapped memory: r0x0001000a flows to r0x0001000f
}

