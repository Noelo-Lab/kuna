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

// Function: __printf_chk @ 0x401060
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_404018)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x401070
void exit(int a0)
{
  (*dat_404020)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x401080
int __fprintf_chk(void *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_404028)(); // jump-as-call
  return v1;
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

// Function: die @ 0x401176
void die(unsigned long a0)
{
  __fprintf_chk(stderr,1,"fatal: %s\n",a0);
  exit(1); // no-return
}

// Function: compute_a @ 0x4011a4
int compute_a(int a0)
{
  if (0 <= a0)
    return a0 * 2;
  die("a: negative"); // no-return
}

// Function: compute_b @ 0x4011be
undefined16 compute_b(int a0)
{
  char v1 [16];
  
  if (a0) {
    v1._0_8_ = 100 / (long)a0 & 0xffffffff;
    v1._8_8_ = 100 % (long)a0 & 0xffffffff;
    return v1._0_16_;
  }
  die("b: zero"); // no-return
}

// Function: compute_c @ 0x4011dd
undefined16 compute_c(int a0,int a1)
{
  char v1 [16];
  
  if (a1) {
    v1._0_8_ = (long)a0 / (long)a1 & 0xffffffff;
    v1._8_8_ = (long)a0 % (long)a1 & 0xffffffff;
    return v1._0_16_;
  }
  die("c: divide by zero"); // no-return
}

// Function: compute_d @ 0x4011f9
int compute_d(char *a0)
{
  if (a0)
    return (int)*a0;
  die("d: null pointer"); // no-return
}

// Function: main @ 0x401214
unsigned long main(unsigned long a0,unsigned long *a1)
{
  unsigned long v1;
  unsigned int v2; // eax
  unsigned int v3; // eax
  unsigned int v4; // eax
  
  v2 = compute_a(a0);
  v3 = compute_b((int)a0 + 1);
  v4 = compute_c(v2,v3);
  v1 = *a1;
  __printf_chk(1,"%d %d %d %d\n",(unsigned long)v2,(unsigned long)v3,(unsigned long)v4,(unsigned long)(unsigned int)compute_d(v1));
  return 0;
}

// Function: _fini @ 0x40127c
void _fini(void)
{
  return;
}

