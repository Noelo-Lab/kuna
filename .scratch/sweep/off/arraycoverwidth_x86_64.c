// Function: vm @ 0x100000
char vm(void)
{
  char v1 [16];
  char v2 [16]; // stack - 0x38
  undefined16 v3 [2]; // stack - 0x28
  
  v2._0_16_ = 0;
  v3[0]._0_16_ = 0;
  sink(v2);
  sink(v3);
  v1 = v2._0_16_;
  v2._0_16_ = v3[0]._0_16_;
  v3[0]._0_16_ = v1;
  sink(v2);
  return v2[3];
}

// Function: sink @ 0x100049
void sink(void)
{
  return;
}

