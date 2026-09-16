// Function: __libc_start_main @ 0x540
void __libc_start_main(void)
{
  (*(void *)(dat_2018 & 0xfffffffffffffffe))(); // jump-as-call
}

// Function: sub_550 @ 0x550
void sub_550(unsigned long a0)
{
  unsigned long v1; // stack + 0x0
  
  sub_572();
  __libc_start_main(dat_2030,v1,&Stack0000000000000008,0,0,a0,&Stack0000000000000000);
  ebreak(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: sub_572 @ 0x572
void sub_572(void)
{
  gp = 0x2800;
}

// Function: sub_57e @ 0x57e
void sub_57e(void)
{
  return;
}

// Function: sub_5cc @ 0x5cc
void sub_5cc(void) // return-dupe
{
  if (dat_2048)
    return;
  if (dat_2038)
    (*dat_2038)(dat_2000);
  sub_57e();
  dat_2048 = 1;
}

// Function: main @ 0x608
int main(int argc,char **argv,char **envp)
{
  return argc;
}

