// Function: m16_square @ 0x400130
int m16_square(int a0)
{
  return a0 * a0 + 3;
}

// Function: m32_caller @ 0x400138
void m32_caller(unsigned int a0,unsigned int a1,unsigned int a2,int a3) // warn: Control flow encountered bad instruction data
{
  if (a3)
    halt_baddata(); // warn: Bad instruction - Truncating control flow here
  halt_baddata(); // warn: Bad instruction - Truncating control flow here
}

// Function: _start @ 0x400158
void _start(unsigned int a0,unsigned int a1,unsigned int a2,int a3) // warn: Control flow encountered bad instruction data
{
  if (a3)
    halt_baddata(); // warn: Bad instruction - Truncating control flow here
  halt_baddata(); // warn: Bad instruction - Truncating control flow here
}

