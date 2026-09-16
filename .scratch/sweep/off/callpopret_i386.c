// Function: sub_8049000 @ 0x8049000
void sub_8049000(void)
{
  dllname();
  do {
  } while( true );
}

// Function: dllname @ 0x8049007
char * dllname(void)
{
  return "kernel32.dll"; // warn: callpopret: 0x0804905e discards the return address -- following this call as a branch
}

// Function: getptr @ 0x804905e
int getptr(void)
{
  int v1; // stack + 0x0
  
  return v1 + 1;
}

