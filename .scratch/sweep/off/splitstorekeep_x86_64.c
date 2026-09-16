// Function: copy31 @ 0x100000
void copy31(void)
{
  unsigned long v1; // stack - 0x58
  unsigned long v10; // stack - 0x21
  unsigned long v2; // stack - 0x38
  undefined7 v3; // stack - 0x50
  char v4; // stack - 0x49
  undefined7 v5; // stack - 0x48
  unsigned long v6; // stack - 0x41
  undefined7 v7; // stack - 0x30
  char v8; // stack - 0x29
  undefined7 v9; // stack - 0x28
  
  sink(&v1);
  v2 = v1;
  v7 = v3;
  v9 = v5;
  v8 = v4;
  v10 = v6;
  sink(&v2);
}

// Function: copy32 @ 0x100042
void copy32(void)
{
  unsigned long v1; // stack - 0x58
  unsigned long v2; // stack - 0x38
  unsigned long v3; // stack - 0x50
  unsigned long v4; // stack - 0x48
  unsigned long v5; // stack - 0x40
  unsigned long v6; // stack - 0x30
  unsigned long v7; // stack - 0x28
  unsigned long v8; // stack - 0x20
  
  sink(&v1);
  v2 = v1;
  v6 = v3;
  v7 = v4;
  v8 = v5;
  sink(&v2);
}

// Function: sink @ 0x100084
void sink(void)
{
  return;
}

