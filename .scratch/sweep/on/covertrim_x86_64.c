// Function: gfinv @ 0x401000
unsigned long gfinv(unsigned char a0)
{
  unsigned char v1;
  unsigned long v2; // rax
  unsigned char v3;
  unsigned char v4; // bl
  unsigned char v5;
  unsigned char v6;
  unsigned char v7;
  bool v8; // sf
  
  if (!a0)
    return v2 & 0xffffffffffffff00;
  v5 = 1;
  v4 = 0xfe;
  do {
    if (v4 & 1) {
      v6 = a0;
      v7 = v5;
      v5 = 0;
      while (v6) {
        v3 = v7 ^ v5;
        if (!(v6 & 1))
          v3 = v5;
        v5 = v7 * '\x02';
        v8 = '\0' <= (char)v7;
        v7 = v5 ^ 0x2d;
        if (v8)
          v7 = v5;
        v6 >>= 1;
        v5 = v3;
      }
    }
    v6 = 0;
    v7 = a0;
    v3 = a0;
    while (a0 = v6, v7) {
      v6 = v3 ^ a0;
      if (!(v7 & 1))
        v6 = a0;
      v1 = v3 * '\x02';
      v8 = '\0' <= (char)v3;
      v3 = v1 ^ 0x2d;
      if (v8)
        v3 = v1;
      v7 >>= 1;
    }
    v4 >>= 1;
  } while (v4);
  return (unsigned long)v5;
}

// Function: _start @ 0x4010d1
void _start(void) // warn: Function flows into unmapped memory
{
  syscall(0x3c,(unsigned long)(unsigned char)gfinv(3));
  halt_missing(); // warn: Function flow reaches unmapped memory: r0x004010e3 flows to r0x004010e5
}

