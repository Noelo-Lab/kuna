// Function: __libc_start_main @ 0x5d0
void __libc_start_main(void)
{
  (*(void *)(dat_2018 & 0xfffffffffffffffe))(); // jump-as-call
}

// Function: puts @ 0x5e0
int puts(char *a0)
{
  int v1; // a0
  
  v1 = (*(void *)(dat_2020 & 0xfffffffffffffffe))((int)a0); // jump-as-call
  return v1;
}

// Function: printf @ 0x5f0
int printf(char *a0,...)
{
  int v1; // a0
  
  v1 = (*(void *)(dat_2028 & 0xfffffffffffffffe))((int)a0); // jump-as-call
  return v1;
}

// Function: _start @ 0x600
void _start(unsigned long a0)
{
  unsigned long v1; // stack + 0x0
  
  sub_622();
  __libc_start_main(dat_2040,v1,&Stack0000000000000008,0,0,a0,&Stack0000000000000000);
  ebreak(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: sub_622 @ 0x622
void sub_622(void)
{
  gp = 0x2800;
}

// Function: deregister_tm_clones @ 0x62e
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x650
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x67c
void __do_global_dtors_aux(void) // return-dupe
{
  if (completed.0)
    return;
  if (dat_2048)
    (*dat_2048)(dat_2000);
  deregister_tm_clones();
  completed.0 = 1;
}

// Function: frame_dummy @ 0x6b6
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: main @ 0x6b8
unsigned long main(int a0,unsigned long a1)
{
  puts("hello");
  printf("%d\n",(long)a0,a1);
  return 0;
}

