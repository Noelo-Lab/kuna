// Function: _DT_INIT @ 0x38c
void _DT_INIT(void)
{
  sub_410();
}

// Function: __libc_start_main @ 0x3ac
void __libc_start_main(void)
{
  (*dat_10fdc)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x3b8
void __cxa_finalize(void)
{
  (*dat_10fe0)(); // jump-as-call
}

// Function: __gmon_start__ @ 0x3c4
void __gmon_start__(void)
{
  (*dat_10fe4)(); // jump-as-call
}

// Function: abort @ 0x3d0
void abort(void)
{
  (*dat_10fe8)(); // jump-as-call
}

// Function: sub_3dc @ 0x3dc
void sub_3dc(void) // return-dupe
{
  bool v1; // zr
  
  if (!v1) { // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
  }
  sub_3c520();
}

// Function: sub_3e0 @ 0x3e0
void sub_3e0(unsigned int a0,unsigned int *a1) // warn: Control flow encountered bad instruction data, return-dupe
{
  char *v1; // r10
  unsigned int v2; // r11
  bool v3; // ng
  bool v4; // zr
  char v5; // ov
  unsigned int v6; // cr0
  unsigned int v7; // cr15
  
  if (v4)
    coprocessor_function(0,0,2,v7,v6,v7);
  if (v3)
    *v1 = (char)v2;
  if (v3 == (bool)v5)
    halt_baddata(); // warn: Bad instruction - Truncating control flow here
  *a1 = v2;
  halt_baddata();
}

// Function: sub_410 @ 0x410
void sub_410(void)
{
  if (!dat_10ff4)
    return;
  __gmon_start__(); // tail-call
}

// Function: _FINI_0 @ 0x494
void _FINI_0(unsigned int a0,unsigned int a1) // warn: Control flow encountered bad instruction data, return-dupe
{
  int v1; // r11
  char v2; // ng
  char v3; // ov
  unsigned long long v4;
  
  if ((bool)v2) {
    v4 = sub_2ad8bc();
    a1 = (unsigned int)((unsigned long long)v4 >> 0x20);
    a0 = (unsigned int)v4;
  }
  if ((bool)v2)
    v1 -= 0xa0a;
  if (!(bool)v3)
    a1 = *(unsigned int *)(v1 + -0x18);
  if (!(bool)v2)
    halt_baddata(); // warn: Bad instruction - Truncating control flow here
  sub_26ea54(a0,a1);
  halt_baddata();
}

// Function: _INIT_0 @ 0x4d4
void _INIT_0(void) // return-dupe
{
  char v1; // ng
  char v2; // ov
  
  if (v1 == v2) { // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
  }
  software_interrupt(0xe7c4);
}

// Function: sub_4d8 @ 0x4d8
unsigned int sub_4d8(unsigned int a0)
{
  return a0;
}

// Function: _DT_FINI @ 0x4f0  (error: Unable to load 512 bytes at r0x000006e0)

