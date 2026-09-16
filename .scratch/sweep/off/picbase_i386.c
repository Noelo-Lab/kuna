// Function: _start @ 0x8049000
void _start(void)
{
  inherits();
  clobbers();
  thunked();
}

// Function: inherits @ 0x8049028
int inherits(void)
{
  int v1; // ebx
  
  *(int *)(v1 + 0xc) = *(int *)(v1 + 0xc) + 1;
  return v1 + -0x105d;
}

// Function: clobbers @ 0x804903e
unsigned int clobbers(void)
{
  return 0x111100c9;
}

// Function: thunked @ 0x804904c
char * thunked(void) // injected
{
  return "thunked reference";
}

// Function: __x86.get_pc_thunk.bx @ 0x8049060
void __x86.get_pc_thunk.bx(void)
{
  return;
}

