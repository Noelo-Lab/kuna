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

// Function: __stack_chk_fail @ 0x401040
void __stack_chk_fail(void)
{
  (*dat_404018)(); // jump-as-call
}

// Function: _start @ 0x401050
void _start(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_403ff0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: _dl_relocate_static_pie @ 0x401080
void _dl_relocate_static_pie(void)
{
  return;
}

// Function: deregister_tm_clones @ 0x401090
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x4010c0
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x401100
void __do_global_dtors_aux(void)
{
  if (!completed.0) {
    deregister_tm_clones();
    completed.0 = 1;
    return;
  }
}

// Function: frame_dummy @ 0x401130
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: foo::Bar::baz @ 0x401136
int foo::Bar::baz(unsigned long a0,int a1)
{
  return a1 + 1;
}

// Function: main @ 0x401154
unsigned long main(void)
{
  long v1;
  
  foo::Bar::baz(&v1,0x2a);
  return 0;
}

// Function: _fini @ 0x40119c
void _fini(void)
{
  return;
}

