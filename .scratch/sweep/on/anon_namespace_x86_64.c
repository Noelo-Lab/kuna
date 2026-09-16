// Function: _init @ 0x1000
void _init(void) // return-dupe
{
  if (!dat_3fe8)
    return;
  (*dat_3fe8)();
}

// Function: sub_1020 @ 0x1020
void sub_1020(void)
{
  (*dat_3fc8)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x1040
void __cxa_finalize(void)
{
  (*dat_3ff8)(); // jump-as-call
}

// Function: __stack_chk_fail @ 0x1050
void __stack_chk_fail(void)
{
  (*dat_3fd0)(); // jump-as-call
}

// Function: _start @ 0x1060
void _start(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_3fd8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: deregister_tm_clones @ 0x1090
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x10c0
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x1100
void __do_global_dtors_aux(void)
{
  if (!completed.0) {
    if (dat_3ff8)
      __cxa_finalize(dat_4008);
    deregister_tm_clones();
    completed.0 = 1;
    return;
  }
}

// Function: frame_dummy @ 0x1140
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: outer::anonymous_namespace::nested_helper @ 0x114a
int outer::anonymous_namespace::nested_helper(int a0)
{
  return a0 * 3;
}

// Function: outer::anonymous_namespace::Widget::emit @ 0x114e
void outer::anonymous_namespace::Widget::emit(unsigned long a0,unsigned int a1)
{
  outer::anonymous_namespace::nested_helper(a1);
}

// Function: anonymous_namespace::top_helper @ 0x1156
int anonymous_namespace::top_helper(int a0)
{
  return a0 + 1;
}

// Function: outer::entry @ 0x115a
void outer::entry(unsigned int a0)
{
  long v1;
  
  outer::anonymous_namespace::Widget::emit(&v1,a0);
}

// Function: main @ 0x1198
int main(void)
{
  int v1; // eax
  
  v1 = outer::entry(2);
  return anonymous_namespace::top_helper(1) + v1;
}

// Function: _fini @ 0x11b8
void _fini(void)
{
  return;
}

