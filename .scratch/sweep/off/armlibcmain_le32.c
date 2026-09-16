// Function: __libc_start_main @ 0x1035c
void __libc_start_main(void)
{
  (*dat_1200c)(); // jump-as-call
}

// Function: fgets @ 0x10368
char * fgets(char *a0,int a1,void *a2)
{
  char *v1; // r0
  
  v1 = (char *)(*dat_12010)(a0); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x10374
void __stack_chk_fail(void)
{
  (*dat_12014)(); // jump-as-call
}

// Function: __printf_chk @ 0x10380
int __printf_chk(int a0,char *a1,...)
{
  int v1; // r0
  
  v1 = (*dat_12018)(a0); // jump-as-call
  return v1;
}

// Function: abort @ 0x1038c
void abort(void)
{
  (*dat_1201c)(); // jump-as-call
}

// Function: sub_10398 @ 0x10398
void sub_10398(unsigned int a0)
{
  unsigned int v1;
  char *v2; // stack + 0x0
  
  v1 = v2;
  v2 = &Stack00000004;
  __libc_start_main(main,v1,&Stack00000004,0x103d4,0x103d8,a0);
  abort(); // no-return
}

// Function: main @ 0x103dc
void main(unsigned int a0,unsigned int a1,unsigned int a2)
{
  char *v1; // r0
  unsigned int v2;
  char v3 [32]; // stack - 0x2c
  void *v4; // lr
  int v5; // stack - 0xc
  
  v5 = __stack_chk_guard;
  __printf_chk(2,"kuna armlibcmain prompt",a2,0);
  v1 = fgets(v3,0x20,stdin);
  if (v1) { // branch-flip
    v2 = 0;
    if (v3[0] != 'k')
      v2 = 1;
  }
  else {
    v2 = 1;
  }
  if (v5 != __stack_chk_guard)
    __stack_chk_fail(); // no-return
  (*v4)(v2); // jump-as-call
}

