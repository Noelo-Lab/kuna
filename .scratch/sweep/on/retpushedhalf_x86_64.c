// Function: xordec @ 0x401000
unsigned long xordec(long a0,int a1,long a2,int a3)
{
  long v1;
  undefined8 v2; // rax
  unsigned long v3; // rcx
  
  v1 = (long)a1 + 1;
  for (v3 = 0; (int)v3 < a1; v3 = v3 + 1) {
    *(unsigned char *)(v1 + v3) = *(unsigned char *)(a2 + (int)((long)((unsigned long)(unsigned int)((int)v3 >> 0x1f) << 0x20 | v3 & 0xffffffff) % (long)a3)) ^ *(unsigned char *)(a0 + v3);
  }
  *(char *)(v1 + a1) = 0;
  v2 = v1;
  return v2;
}

// Function: regmove @ 0x401050
undefined16 regmove(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  char v1 [16];
  
  v1._8_8_ = a4;
  v1._0_8_ = a0 + 1;
  return v1._0_16_;
}

// Function: _start @ 0x401060
void _start(void)
{
  xordec(0,0,0,0,0);
  regmove();
}

