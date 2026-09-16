// Function: _DT_INIT @ 0x103b8
void _DT_INIT(void)
{
  sub_10464();
}

// Function: __libc_start_main @ 0x103d8
void __libc_start_main(void)
{
  (*dat_1200c)(); // jump-as-call
}

// Function: fgets @ 0x103e4
char * fgets(char *a0,int a1,void *a2)
{
  char *v1; // r0
  
  v1 = (char *)(*dat_12010)(a0); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x103f0
void __stack_chk_fail(void)
{
  (*dat_12014)(); // jump-as-call
}

// Function: __gmon_start__ @ 0x103fc
void __gmon_start__(void)
{
  (*dat_12018)(); // jump-as-call
}

// Function: __printf_chk @ 0x10408
int __printf_chk(int a0,char *a1,...)
{
  int v1; // r0
  
  v1 = (*dat_1201c)(a0); // jump-as-call
  return v1;
}

// Function: abort @ 0x10414
void abort(void)
{
  (*dat_12020)(); // jump-as-call
}

// Function: sub_10420 @ 0x10420
void sub_10420(unsigned int a0)
{
  unsigned int v1;
  char *v2; // stack + 0x0
  
  v1 = v2;
  v2 = &Stack00000004;
  __libc_start_main(dat_12028,v1,&Stack00000004,0,0,a0);
  abort(); // no-return
}

// Function: sub_10464 @ 0x10464
void sub_10464(void)
{
  if (!dat_12024)
    return;
  __gmon_start__(); // tail-call
}

// Function: sub_10488 @ 0x10488
void sub_10488(void)
{
  return;
}

// Function: sub_104b4 @ 0x104b4
void sub_104b4(void)
{
  return;
}

// Function: _FINI_0 @ 0x104ec
void _FINI_0(void)
{
  if (dat_1203c)
    return;
  sub_10488();
  dat_1203c = 1;
}

// Function: _INIT_0 @ 0x10514
void _INIT_0(void)
{
  sub_104b4(); // tail-call
}

// Function: main @ 0x10518
int main(int argc,char **argv,char **envp)
{
  char *v1; // r0
  unsigned int v2;
  int v3; // r0
  char v4 [32]; // stack - 0x2c
  void *v5; // lr
  int v6; // stack - 0xc
  
  v6 = __stack_chk_guard;
  __printf_chk(2,"kuna armlibcmain got prompt",envp,0);
  v1 = fgets(v4,0x20,stdin);
  if (v1) { // branch-flip
    v2 = 0;
    if (v4[0] != 'k')
      v2 = 1;
  }
  else {
    v2 = 1;
  }
  if (v6 != __stack_chk_guard)
    __stack_chk_fail(); // no-return
  v3 = (*v5)(v2); // jump-as-call
  return v3;
}

// Function: _DT_FINI @ 0x10598
void _DT_FINI(void)
{
  return;
}

