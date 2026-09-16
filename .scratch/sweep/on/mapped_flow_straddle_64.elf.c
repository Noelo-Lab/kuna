// Function: sub_10000 @ 0x10000
unsigned long sub_10000(void) // warn: Function flows into unmapped memory
{
  char *v1; // rax
  
  if (!(int)v1)
    return 7;
  *v1 = *v1 + (char)v1;
  halt_missing(); // warn: Function flow reaches unmapped memory: the instruction at r0x00010011 runs past the mapped bytes
}

