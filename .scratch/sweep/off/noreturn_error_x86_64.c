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

// Function: __errno_location @ 0x401050
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_404018)(); // jump-as-call
  return v1;
}

// Function: error @ 0x401060
void error(int a0,int a1,char *a2,...)
{
  (*dat_404020)(); // jump-as-call
}

// Function: main @ 0x401070
int main(int a0,int a1)
{
  int v1; // eax
  
  v1 = compute((long)a0);
  return compute_warn((long)a0) + v1 + a1;
}

// Function: _start @ 0x4010a0
void _start(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_403ff0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: _dl_relocate_static_pie @ 0x4010d0
void _dl_relocate_static_pie(void)
{
  return;
}

// Function: deregister_tm_clones @ 0x4010e0
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x401110
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x401150
void __do_global_dtors_aux(void)
{
  if (!completed.0) {
    deregister_tm_clones();
    completed.0 = 1;
    return;
  }
}

// Function: frame_dummy @ 0x401180
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: err_warn.constprop.0 @ 0x401190
unsigned long err_warn.constprop.0(void)
{
  error(0,*__errno_location(),"%s","neg");
  return 1;
}

// Function: err_fatal.constprop.0 @ 0x4011c0
void err_fatal.constprop.0(void)
{
  error(2,*__errno_location(),"%s","negative");
}

// Function: compute @ 0x4011f0
long compute(long a0)
{
  if (0 <= a0)
    return a0 * 2 + 7;
  err_fatal.constprop.0(); // no-return
}

// Function: compute_warn @ 0x401210
long compute_warn(long a0)
{
  if (0 <= a0)
    return a0 + 1;
  return (long)(int)err_warn.constprop.0();
}

// Function: _fini @ 0x401230
void _fini(void)
{
  return;
}

