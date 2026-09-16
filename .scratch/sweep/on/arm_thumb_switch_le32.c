// Function: f0 @ 0x100b8
int f0(int a0)
{
  return a0 + 3;
}

// Function: f1 @ 0x100bc
int f1(int a0)
{
  return a0 * 5;
}

// Function: f2 @ 0x100c2
int f2(int a0)
{
  return a0 + -7;
}

// Function: f3 @ 0x100c6
unsigned int f3(unsigned int a0)
{
  return a0 ^ 9;
}

// Function: dispatch @ 0x100cc
int dispatch(unsigned int a0,unsigned int a1,unsigned int a2,unsigned int a3)
{
  int v1;
  int v10; // r3
  int v11; // r3
  unsigned long long v12;
  int v2;
  unsigned int v3; // r1
  unsigned int v4; // r2
  unsigned int v5; // r2
  unsigned int v6; // r2
  unsigned int v7; // r2
  int v8; // r3
  int v9; // r3
  
  switch(a0) {
    case 0:
      return f0(a1,a1,a2,a3); // tail-call
    case 1:
      return f1(a1,a1,a2,a3); // tail-call
    case 2:
      return f2(a1,a1,a2,a3); // tail-call
    case 3:
      return f3(a1,a1,a2,a3); // tail-call
    case 4:
      v12 = f0(a1);
      v3 = (unsigned int)((unsigned long long)v12 >> 0x20);
      v2 = f1(v3,v3,v4,(int)v12);
      v1 = v8;
      v2 += v1; // return-dupe
      break;
    case 5:
      v12 = f2(a1);
      v3 = (unsigned int)((unsigned long long)v12 >> 0x20);
      v2 = f3(v3,v3,v5,(int)v12);
      v1 = v9;
      v2 += v1;
      return v2;
    case 6:
      v12 = f0(a1);
      v3 = (unsigned int)((unsigned long long)v12 >> 0x20);
      v1 = f2(v3,v3,v6,(int)v12);
      v2 = v10;
      v2 -= v1; // return-dupe
      break;
    case 7:
      v12 = f1(a1);
      v3 = (unsigned int)((unsigned long long)v12 >> 0x20);
      v1 = f3(v3,v3,v7,(int)v12);
      v2 = v11;
      v2 -= v1;
      return v2;
    default:
      v2 = 0;
    
  }
  return v2;
}

// Function: _start @ 0x1014e
void _start(void)
{
  f2(0xb);
  do {
  } while( true );
}

