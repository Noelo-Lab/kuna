// Function: callee @ 0x401000
int callee(int a0,int a1,int a2,int a3)
{
  return a0 + a1 + a2 + a3;
}

// Function: callee2 @ 0x401010
int callee2(int a0,int a1,int a2,int a3,int a4)
{
  return a0 + a1 + a2 + a3 + a4;
}

// Function: caller @ 0x401020
void caller(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // ecx
  
  v1 = (int)a2 + 0x8000;
  if (v1 <= 0xffff) {
    callee(a0,4,a1,v1);
    return;
  }
  callee(a0,4,a1,(int)((unsigned long)a2 >> 0x10) + 0x8000);
}

// Function: caller2 @ 0x401060
void caller2(unsigned long a0,unsigned long a1,unsigned long a2)
{
  if ((int)a2 + 0x8000U <= 0xffff) {
    callee2(a0,4,a1);
    return;
  }
  callee2(a0,4,a1,(int)((unsigned long)a2 >> 0x10) + 0x8000);
}

// Function: _start @ 0x4010a0
void _start(void)
{
  caller(0,0,0);
  caller2();
}

