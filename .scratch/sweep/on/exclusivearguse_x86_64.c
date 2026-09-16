// Function: append_slow @ 0x401000
void append_slow(unsigned long *a0,unsigned long a1,char *a2)
{
  a0[1] = a1;
  *(char *)*a0 = *a2;
}

// Function: append @ 0x401020
void append(long a0,unsigned char a1) // return-dupe
{
  char *v1;
  
  v1 = *(char **)(a0 + 8);
  if (v1 != *(char **)(a0 + 0x10)) {
    *v1 = *(char *)((unsigned long)a1 + 0x402000);
    *(long *)(a0 + 8) = *(long *)(a0 + 8) + 1;
    return;
  }
  append_slow(a0,v1,(char *)((unsigned long)a1 + 0x402000));
}

// Function: append_loop @ 0x401050
void append_loop(long a0,char *a1)
{
  int *v1;
  
  do {
    if (*(char **)(a0 + 8) != *(char **)(a0 + 0x10)) { // branch-flip
      **(char **)(a0 + 8) = *a1;
      *(long *)(a0 + 8) = *(long *)(a0 + 8) + 1;
    }
    else {
      append_slow(a0);
    }
    v1 = (int *)(a0 + 0x18);
    *v1 = *v1 + -1;
  } while (*v1);
}

