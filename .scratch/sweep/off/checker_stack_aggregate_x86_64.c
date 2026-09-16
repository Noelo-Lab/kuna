// Function: sink @ 0x401000
void sink(unsigned long *p)
{
  return;
}

// Function: stack_aggregate @ 0x401013
unsigned int stack_aggregate(unsigned int a,unsigned int b)
{
  unsigned int v12; // stack - 0x10
  
  v12._0_4_ = a;
  v12._4_4_ = b;
  sink(&v12);
  return (unsigned int)v12;
}

// Function: _start @ 0x40104f
void _start(void)
{
  stack_aggregate(1,2);
}

