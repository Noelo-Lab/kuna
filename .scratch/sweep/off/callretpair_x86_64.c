// Function: read_value @ 0x401000
undefined16 read_value(unsigned long a0,unsigned long *a1,unsigned long a2,int a3)
{
  unsigned long v1;
  char v2 [16];
  unsigned long v3;
  
  v1 = *a1;
  v3 = a1[1];
  if (!a3) {
    v3 = 6;
    v1 = 0;
  }
  v2._8_8_ = v3;
  v2._0_8_ = v1;
  return v2._0_16_;
}

// Function: read_tag @ 0x401020
long read_tag(unsigned long a0,unsigned long a1)
{
  char v1 [16];
  
  v1 = read_value(a0,a1,0x10,1);
  if (SUB164(v1,8) != 6)
    return *SUB168(v1,0) + SUB168(v1,8);
  return -1;
}

// Function: no_payload @ 0x401050
long no_payload(long *a0)
{
  return *a0 + 1;
}

// Function: read_tag_clobber @ 0x401060
long * read_tag_clobber(unsigned long a0,unsigned long a1)
{
  long *v1; // rax
  long v2; // rdx
  
  no_payload(a0,a1,7);
  if ((int)v2 != 6) {
    v1 = (long)(*v1 + v2);
    return v1;
  }
  v1 = (long *)0xffffffffffffffff;
  return v1;
}

// Function: _start @ 0x401084
void _start(void) // warn: Function flows into unmapped memory
{
  read_tag_clobber(read_tag(0,0x402000));
  syscall(0x3c);
  halt_missing(); // warn: Function flow reaches unmapped memory: r0x0040109f flows to r0x004010a1
}

