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

// Function: std::__throw_length_error @ 0x401030
void std::__throw_length_error(void)
{
  (*dat_404018)(); // jump-as-call
}

// Function: std::__throw_out_of_range @ 0x401040
void std::__throw_out_of_range(void)
{
  (*dat_404020)(); // jump-as-call
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

// Function: append_bound @ 0x401140
long append_bound(long a0)
{
  if (0x65 <= a0)
    std::__throw_length_error("basic_string::append"); // no-return
  return a0 * 3;
}

// Function: at_bound @ 0x401170
long at_bound(long a0)
{
  if (a0 <= 0x3f)
    return a0 + 1;
  std::__throw_out_of_range("vector::at"); // no-return
}

// Function: main @ 0x4011a0
unsigned long main(void)
{
  return 0xc;
}

// Function: _fini @ 0x4011a8
void _fini(void)
{
  return;
}

