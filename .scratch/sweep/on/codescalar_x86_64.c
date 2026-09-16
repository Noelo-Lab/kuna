// Function: sink @ 0x100000
void sink(void)
{
  return;
}

// Function: codebyte @ 0x100001
char codebyte(void *a0,int a1)
{
  char v1; // al
  
  v1 = '\0';
  if (a1) {
    v1 = (*a0)();
    *a0 = v1;
  }
  return v1 + '$';
}

// Function: databyte @ 0x100016
char databyte(char *a0,int a1)
{
  char v1; // al
  
  v1 = '\0';
  if (a1) {
    v1 = sink();
    *a0 = v1;
  }
  return v1 + '$';
}

