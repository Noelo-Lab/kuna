// Function: _init @ 0x400468
void _init(void)
{
  call_weak_fn();
}

// Function: __libc_start_main @ 0x4004a0
void __libc_start_main(void)
{
  (*dat_411000)(); // jump-as-call
}

// Function: __gmon_start__ @ 0x4004b0
void __gmon_start__(void)
{
  (*dat_411008)(); // jump-as-call
}

// Function: abort @ 0x4004c0
void abort(void)
{
  (*dat_411010)(); // jump-as-call
}

// Function: puts @ 0x4004d0
int puts(char *a0)
{
  int v1; // w0
  
  v1 = (*dat_411018)((int)a0); // jump-as-call
  return v1;
}

// Function: printf @ 0x4004e0
int printf(char *a0,...)
{
  int v1; // w0
  
  v1 = (*dat_411020)((int)a0); // jump-as-call
  return v1;
}

// Function: _start @ 0x400500
void _start(unsigned long a0)
{
  unsigned long v1; // stack + 0x0
  
  __libc_start_main(0x400534,v1,&Stack0000000000000008,0,0,a0);
  abort(); // no-return
}

// Function: _dl_relocate_static_pie @ 0x400540
void _dl_relocate_static_pie(void)
{
  return;
}

// Function: call_weak_fn @ 0x400544
void call_weak_fn(void)
{
  if (dat_410fe0) {
    __gmon_start__(); // tail-call
    return;
  }
}

// Function: deregister_tm_clones @ 0x400560
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x400590
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x4005d0
void __do_global_dtors_aux(void) // return-dupe
{
  if (completed.0)
    return;
  deregister_tm_clones();
  completed.0 = 1;
}

// Function: frame_dummy @ 0x400600
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: main @ 0x400604
unsigned long main(unsigned int a0)
{
  puts("hello");
  printf("%d\n",(unsigned long)a0);
  return 0;
}

// Function: _fini @ 0x40063c
void _fini(void)
{
  return;
}

