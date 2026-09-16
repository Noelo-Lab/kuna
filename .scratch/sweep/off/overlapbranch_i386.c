// Function: sub_804881c @ 0x804881c
int sub_804881c(int a0)
{
  if ((char)a0 == '\xff')
    halt_baddata(); // warn: overlapbranch: this instruction overlaps the branch target at 0x08048825; truncating the fall-through here
  return a0 + 7 + *(int *)(a0 + 7) + -0x26722300;
}

