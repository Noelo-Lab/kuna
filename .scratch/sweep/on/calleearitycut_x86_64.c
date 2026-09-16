// Function: sink @ 0x401000
void sink(void)
{
  return;
}

// Function: callee @ 0x401001
void callee(void)
{
  sink();
}

// Function: callee2 @ 0x401010
void callee2(void)
{
  sink();
}

// Function: caller @ 0x401025
int caller(unsigned long a0,long a1,long a2)
{
  if (a2 != a1)
    return callee(a0,a1,a2,1) + (int)a0;
  return 0;
}

// Function: caller2 @ 0x401040
int caller2(int a0,long a1,long a2)
{
  if (a2 != a1)
    return callee2() + a0;
  return 0;
}

// Function: caller3 @ 0x40105b
int caller3(int a0,long a1,long a2)
{
  if (a2 != a1)
    return callee() + a0;
  return 0;
}

