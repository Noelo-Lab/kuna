// Function: retseven @ 0x401000
unsigned long retseven(void)
{
  return 7;
}

// Function: gap1 @ 0x401006
void gap1(void) // warn: Control flow encountered bad instruction data
{
  retseven();
  halt_baddata(); // warn: Bad instruction - Truncating control flow here
}

// Function: gap2 @ 0x40100d
void gap2(void) // warn: Control flow encountered bad instruction data
{
  retseven();
  halt_baddata(); // warn: Bad instruction - Truncating control flow here
}

// Function: gap3 @ 0x401014
void gap3(void) // warn: Control flow encountered bad instruction data
{
  retseven();
  halt_baddata(); // warn: Bad instruction - Truncating control flow here
}

// Function: usesret @ 0x40101b
int usesret(void)
{
  return retseven() + 3;
}

// Function: _start @ 0x40102a
unsigned long _start(void)
{
  gap1();
  gap2();
  gap3();
  return syscall(0x3c,(unsigned long)(unsigned int)usesret());
}

