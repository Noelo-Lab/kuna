// Function: guarded_status @ 0x400000
int guarded_status(int a0)
{
  if (dat_400050 == dat_400054)
    return -(unsigned int)(a0 != 0);
  __stack_chk_fail(-(unsigned int)(a0 != 0)); // no-return
}

// Function: status_caller @ 0x400034
bool status_caller(unsigned long a0)
{
  return guarded_status(a0) != 0;
}

