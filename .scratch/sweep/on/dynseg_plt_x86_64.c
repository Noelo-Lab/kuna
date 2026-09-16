// Function: puts @ 0x240
int puts(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_318)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x250
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_320)(); // jump-as-call
  return v1;
}

// Function: sub_260 @ 0x260
unsigned long sub_260(void)
{
  char v1 [16];
  
  memcpy(v1,(void *)0x238,3);
  puts(v1);
  return 0;
}

