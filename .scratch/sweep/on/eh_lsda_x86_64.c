// Function: _DT_INIT @ 0x401000
void _DT_INIT(void) // return-dupe
{
  if (!dat_403ff0)
    return;
  (*dat_403ff0)();
}

// Function: sub_401020 @ 0x401020
void sub_401020(void)
{
  (*dat_404010)(); // jump-as-call
}

// Function: __printf_chk @ 0x4010d0
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_404018)(); // jump-as-call
  return v1;
}

// Function: std::runtime_error::runtime_error @ 0x4010e0
void std::runtime_error::runtime_error(void)
{
  (*dat_404020)(); // jump-as-call
}

// Function: __cxa_begin_catch @ 0x4010f0
void __cxa_begin_catch(void)
{
  (*dat_404028)(); // jump-as-call
}

// Function: __cxa_allocate_exception @ 0x401100
void __cxa_allocate_exception(void)
{
  (*dat_404030)(); // jump-as-call
}

// Function: __cxa_free_exception @ 0x401110
void __cxa_free_exception(void)
{
  (*dat_404038)(); // jump-as-call
}

// Function: std::runtime_error::~runtime_error @ 0x401120
void std::runtime_error::~runtime_error(runtime_error *this)
{
  (*dat_404040)(); // jump-as-call
}

// Function: __cxa_end_catch @ 0x401130
void __cxa_end_catch(void)
{
  (*dat_404048)(); // jump-as-call
}

// Function: __gxx_personality_v0 @ 0x401140
void __gxx_personality_v0(void)
{
  (*dat_404050)(); // jump-as-call
}

// Function: __cxa_throw @ 0x401150
void __cxa_throw(void)
{
  (*dat_404058)(); // jump-as-call
}

// Function: _Unwind_Resume @ 0x401160
void _Unwind_Resume(void)
{
  (*dat_404060)(); // jump-as-call
}

// Function: sub_401170 @ 0x401170
void sub_401170(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_403fe8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_4011a0 @ 0x4011a0
void sub_4011a0(void)
{
  return;
}

// Function: sub_4011b0 @ 0x4011b0
void sub_4011b0(void)
{
  return;
}

// Function: _FINI_0 @ 0x401220
void _FINI_0(void)
{
  if (!dat_404078) {
    sub_4011b0();
    dat_404078 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x401250
void _INIT_0(void)
{
  return;
}

// Function: sub_401256 @ 0x401256
int sub_401256(int a0)
{
  unsigned long v1; // rax
  unsigned int *v2; // rax
  
  if (a0 < 0) {
    v1 = __cxa_allocate_exception(0x10);
    std::runtime_error::runtime_error(v1,"neg");
    __cxa_throw(v1,_ZTISt13runtime_error,~runtime_error); // no-return
  }
  if (a0)
    return a0 * 3 + 7;
  v2 = (unsigned int *)__cxa_allocate_exception(4);
  *v2 = 0x2a;
  __cxa_throw(v2,_ZTIi,0); // no-return
}

// Function: sub_4012d6 @ 0x4012d6
void sub_4012d6(void)
{
  sub_401256();
}

// Function: main @ 0x40137a
int main(int argc,char **argv,char **envp)
{
  int v1; // ebx
  unsigned int v2; // ebp
  
  if (1 <= argc) { // branch-flip
    v1 = -1;
    v2 = 0;
    do {
      v2 += sub_4012d6(v1);
      v1 += 1;
    } while (v1 != argc + -1);
  }
  else {
    v2 = 0;
  }
  __printf_chk(1,"%d\n",(unsigned long)v2);
  return v2;
}

// Function: _DT_FINI @ 0x4013cc
void _DT_FINI(void)
{
  return;
}

