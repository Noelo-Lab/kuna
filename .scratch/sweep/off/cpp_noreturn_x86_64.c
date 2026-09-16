// Function: _init @ 0x401000
void _init(void) // return-dupe
{
  if (!dat_403ff8)
    return;
  (*dat_403ff8)();
}

// Function: sub_401020 @ 0x401020
void sub_401020(void)
{
  (*dat_404010)(); // jump-as-call
}

// Function: std::terminate @ 0x401070
void std::terminate(void)
{
  (*dat_404018)(); // jump-as-call
}

// Function: __cxa_allocate_exception @ 0x401080
void __cxa_allocate_exception(void)
{
  (*dat_404020)(); // jump-as-call
}

// Function: __stack_chk_fail @ 0x401090
void __stack_chk_fail(void)
{
  (*dat_404028)(); // jump-as-call
}

// Function: __cxa_throw @ 0x4010a0
void __cxa_throw(void)
{
  (*dat_404030)(); // jump-as-call
}

// Function: _start @ 0x4010b0
void _start(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_403ff0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: _dl_relocate_static_pie @ 0x4010e0
void _dl_relocate_static_pie(void)
{
  return;
}

// Function: deregister_tm_clones @ 0x4010f0
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x401120
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x401160
void __do_global_dtors_aux(void)
{
  if (!completed.0) {
    deregister_tm_clones();
    completed.0 = 1;
    return;
  }
}

// Function: frame_dummy @ 0x401190
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: fail @ 0x401196
void fail(void)
{
  std::terminate(); // no-return
}

// Function: main @ 0x4011a3
int main(int a0)
{
  long v1;
  
  app::Guard::throw_it(&v1,a0);
  if (a0 <= 7)
    return a0;
  fail(); // no-return
}

// Function: app::Guard::throw_it @ 0x4011fa
void app::Guard::throw_it(unsigned long a0,int a1)
{
  int *v1; // rax
  
  if (a1) {
    v1 = (int *)__cxa_allocate_exception(4);
    *v1 = a1;
    __cxa_throw(v1,_ZTIi,0); // no-return
  }
}

// Function: _fini @ 0x401238
void _fini(void)
{
  return;
}

