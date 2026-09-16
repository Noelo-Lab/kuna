// Function: sub_10000 @ 0x10000
unsigned int sub_10000(char *a0) // warn: Function flows into unmapped memory
{
  if (!a0)
    return 7;
  *a0 = *a0 + (char)a0;
  halt_missing(); // warn: Function flow reaches unmapped memory: the instruction at r0x00010011 runs past the mapped bytes
}

