// Function: _init @ 0x5b8
void _init(void)
{
  call_weak_fn();
}

// Function: __libc_start_main @ 0x5f0
void __libc_start_main(void)
{
  (*dat_10fa8)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x600
void __cxa_finalize(void)
{
  (*dat_10fb0)(); // jump-as-call
}

// Function: __gmon_start__ @ 0x610
void __gmon_start__(void)
{
  (*dat_10fb8)(); // jump-as-call
}

// Function: abort @ 0x620
void abort(void)
{
  (*dat_10fc0)(); // jump-as-call
}

// Function: printf @ 0x630
int printf(char *a0,...)
{
  int v1; // w0
  
  v1 = (*dat_10fc8)((int)a0); // jump-as-call
  return v1;
}

// Function: _start @ 0x640
void _start(unsigned long a0)
{
  unsigned long v1; // stack + 0x0
  
  __libc_start_main(0x754,v1,&Stack0000000000000008,0,0,a0);
  abort(); // no-return
}

// Function: call_weak_fn @ 0x674
void call_weak_fn(void)
{
  if (dat_10fe8) {
    __gmon_start__(); // tail-call
    return;
  }
}

// Function: deregister_tm_clones @ 0x690
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x6c0
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x700
void __do_global_dtors_aux(void) // return-dupe
{
  if (completed.0)
    return;
  if (dat_10fe0)
    __cxa_finalize(dat_11008);
  deregister_tm_clones();
  completed.0 = 1;
}

// Function: frame_dummy @ 0x750
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: main @ 0x754
unsigned long main(unsigned int a0,unsigned long *a1)
{
  printf("%d %s\n",(unsigned long)a0,*a1);
  return 0;
}

// Function: _fini @ 0x78c
void _fini(void)
{
  return;
}

