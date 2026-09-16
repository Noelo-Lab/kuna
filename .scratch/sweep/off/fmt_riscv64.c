// Function: __libc_start_main @ 0x590
void __libc_start_main(void)
{
  (*(void *)(dat_2018 & 0xfffffffffffffffe))(); // jump-as-call
}

// Function: printf @ 0x5a0
int printf(char *a0,...)
{
  int v1; // a0
  
  v1 = (*(void *)(dat_2020 & 0xfffffffffffffffe))((int)a0); // jump-as-call
  return v1;
}

// Function: _start @ 0x5b0
void _start(unsigned long a0)
{
  unsigned long v1; // stack + 0x0
  
  sub_5d2();
  __libc_start_main(dat_2038,v1,&Stack0000000000000008,0,0,a0,&Stack0000000000000000);
  ebreak(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: sub_5d2 @ 0x5d2
void sub_5d2(void)
{
  gp = 0x2800;
}

// Function: deregister_tm_clones @ 0x5de
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x600
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x62c
void __do_global_dtors_aux(void) // return-dupe
{
  if (completed.0)
    return;
  if (dat_2040)
    (*dat_2040)(dat_2000);
  deregister_tm_clones();
  completed.0 = 1;
}

// Function: frame_dummy @ 0x666
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: main @ 0x668
unsigned long main(int a0,unsigned long *a1)
{
  printf("%d %s\n",(long)a0,*a1);
  return 0;
}

