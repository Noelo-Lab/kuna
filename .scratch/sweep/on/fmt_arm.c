// Function: _init @ 0x3ac
void _init(void)
{
  call_weak_fn();
}

// Function: __libc_start_main @ 0x3cc
void __libc_start_main(void)
{
  (*dat_10fd8)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x3d8
void __cxa_finalize(void)
{
  (*dat_10fdc)(); // jump-as-call
}

// Function: printf @ 0x3e4
int printf(char *a0,...)
{
  int v1; // r0
  
  v1 = (*dat_10fe0)(a0); // jump-as-call
  return v1;
}

// Function: __gmon_start__ @ 0x3f0
void __gmon_start__(void)
{
  (*dat_10fe4)(); // jump-as-call
}

// Function: abort @ 0x3fc
void abort(void)
{
  (*dat_10fe8)(); // jump-as-call
}

// Function: _start @ 0x408
void _start(unsigned int a0)
{
  unsigned int v1;
  char *v2; // stack + 0x0
  
  v1 = v2;
  v2 = &Stack00000004;
  __libc_start_main(0x505,v1,v2,0,0,a0);
  abort(); // no-return
}

// Function: call_weak_fn @ 0x43c
void call_weak_fn(void)
{
  if (!dat_10ff4)
    return;
  __gmon_start__(); // tail-call
}

// Function: deregister_tm_clones @ 0x460
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x48c
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x4c0
void __do_global_dtors_aux(void) // return-dupe
{
  if (completed.0)
    return;
  if (dat_10fec)
    __cxa_finalize(dat_11004);
  deregister_tm_clones();
  completed.0 = 1;
}

// Function: frame_dummy @ 0x500
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: main @ 0x504
unsigned int main(unsigned int a0,unsigned int *a1)
{
  printf("%d %s\n",a0,*a1);
  return 0;
}

// Function: _fini @ 0x530
void _fini(void)
{
  return;
}

