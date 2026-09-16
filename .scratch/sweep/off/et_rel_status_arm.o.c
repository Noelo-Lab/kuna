// Function: guarded_status @ 0x400000
unsigned int guarded_status(int a0) // ternary
{
  unsigned int v1; // r0
  
  v1 = (a0) ? 0xffffffff : 0; // branch-flip
  if (dat_40004c != dat_400050)
    __stack_chk_fail(v1); // no-return
  return v1;
}

// Function: status_caller @ 0x400038
bool status_caller(unsigned int a0)
{
  return guarded_status(a0) != 0;
}

