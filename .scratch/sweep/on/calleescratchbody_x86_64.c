// Function: f @ 0x401000
unsigned long f(unsigned long a0)
{
  chkstk();
  return a0;
}

// Function: chkstk @ 0x401020
void chkstk(void) // return-dupe
{
  char *v1; // rax
  char *v2; // r10
  char *v3;
  long v4; // gs_offset
  
  v2 = &(&Stack0000000000000008)[-(long)v1];
  if (&Stack0000000000000008 < v1)
    v2 = NULL;
  v3 = *(char **)(v4 + 0x10);
  if (v3 <= v2)
    return;
  do {
    v3 = &v3[-0x1000];
    *v3 = 0;
  } while ((char *)((unsigned long)v2 & 0xfffffffffffff000) != v3);
}

