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

// Function: abort @ 0x401050
void abort(void)
{
  (*dat_404018)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x401060
int __fprintf_chk(void *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_404020)(); // jump-as-call
  return v1;
}

// Function: main @ 0x401070
void main(int a0)
{
  compute((long)a0 + -2);
}

// Function: _start @ 0x401090
void _start(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_403ff0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: _dl_relocate_static_pie @ 0x4010c0
void _dl_relocate_static_pie(void)
{
  return;
}

// Function: deregister_tm_clones @ 0x4010d0
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x401100
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x401140
void __do_global_dtors_aux(void)
{
  if (!completed.0) {
    deregister_tm_clones();
    completed.0 = 1;
    return;
  }
}

// Function: frame_dummy @ 0x401170
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: my_die.constprop.0 @ 0x401180
void my_die.constprop.0(void)
{
  __fprintf_chk(stderr,1,"fatal: %s\n","negative");
  abort(); // no-return
}

// Function: compute @ 0x4011b0
long compute(long a0)
{
  if (0 <= a0)
    return a0 * 2 + 7;
  my_die.constprop.0(); // no-return
}

// Function: _fini @ 0x4011c8
void _fini(void)
{
  return;
}

