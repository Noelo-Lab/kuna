// Function: sink @ 0x401000
void sink(void)
{
  return;
}

// Function: cal1 @ 0x401001
void cal1(void)
{
  sink();
}

// Function: cal2 @ 0x40100a
void cal2(void)
{
  sink();
}

// Function: cal3 @ 0x401013
void cal3(void)
{
  sink();
}

// Function: cal4 @ 0x40101c
void cal4(void)
{
  sink();
}

// Function: scratch_wide @ 0x401025
int scratch_wide(unsigned long *a0)
{
  unsigned long v1;
  
  v1 = *a0;
  return cal1(a0) + (int)a0 + (int)v1;
}

// Function: scratch_byte @ 0x401038
int scratch_byte(unsigned char *a0)
{
  a0[4] = a0[4] ^ *a0;
  return cal2(a0) + (int)a0;
}

// Function: boundary_live @ 0x40104c
int boundary_live(unsigned long *a0)
{
  unsigned long v1;
  
  v1 = *a0;
  return cal3(a0,v1) + (int)a0;
}

// Function: boundary_const @ 0x40105c
int boundary_const(int a0)
{
  return cal4() + a0 + 2;
}

