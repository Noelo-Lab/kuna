// Function: reader @ 0x401000
int reader(int a0,int a1)
{
  return a0 + a1;
}

// Function: watchdog @ 0x401010
undefined16 watchdog(long a0,int a1,unsigned long a2,unsigned long a3)
{
  long v1; // rax
  long *v2;
  char v3 [16];
  long v4; // rdx
  
  v1 = now(0);
  v4 = 0;
  do {
    if (a1 <= (int)v4) {
      v2 = NULL;
      break;
    }
    v2 = *(long **)(a0 + v4 * 8);
    v4 += 1;
  } while (v1 - *v2 <= 2);
  v3._8_8_ = a3;
  v3._0_8_ = v2;
  return v3._0_16_;
}

// Function: now @ 0x401050
unsigned long now(void)
{
  return 0xc9;
}

// Function: feed @ 0x401060
int feed(int a0)
{
  int v1; // r12d
  
  v1 = 0;
  do {
    if (a0 <= v1)
      return v1;
    reader(0,0);
    v1 += 1;
  } while (!watchdog(0x402000,2));
  return v1;
}

// Function: _start @ 0x4010a0
void _start(void) // warn: Function flows into unmapped memory
{
  syscall(0x3c,(unsigned long)(unsigned int)feed(3));
  halt_missing(); // warn: Function flow reaches unmapped memory: r0x004010b1 flows to r0x004010b3
}

