// Function: _init @ 0x401000
void _init(void) // return-dupe
{
  if (!dat_403ff8)
    return;
  (*dat_403ff8)();
}

// Function: main @ 0x401020
int main(void)
{
  int v1; // eax
  
  v1 = sample(3);
  return latch(1) + v1;
}

// Function: _start @ 0x401040
void _start(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_403ff0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: _dl_relocate_static_pie @ 0x401070
void _dl_relocate_static_pie(void)
{
  return;
}

// Function: deregister_tm_clones @ 0x401080
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x4010b0
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x4010f0
void __do_global_dtors_aux(void)
{
  if (!completed.0) {
    deregister_tm_clones();
    completed.0 = 1;
    return;
  }
}

// Function: frame_dummy @ 0x401120
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: ext @ 0x401130
void ext(void)
{
  return;
}

// Function: sample @ 0x401140
int sample(int a0)
{
  return scale * a0 + bias + dat_50000000 * 2;
}

// Function: latch @ 0x401170
unsigned int latch(unsigned int a0)
{
  dat_50000004 = a0;
  ext();
  return dat_50000004;
}

// Function: _fini @ 0x40118c
void _fini(void)
{
  return;
}

