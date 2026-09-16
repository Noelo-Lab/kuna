// Function: _init @ 0x4004e0
void _init(void)
{
  call_gmon_start();
  frame_dummy();
  __do_global_ctors_aux();
}

// Function: sub_400500 @ 0x400500
void sub_400500(void)
{
  (*dat_600ff8)(); // jump-as-call
}

// Function: puts @ 0x400510
int puts(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_601000)(); // jump-as-call
  return v1;
}

// Function: printf @ 0x400520
int printf(char *a0,...)
{
  int v1; // eax
  
  v1 = (*dat_601008)(); // jump-as-call
  return v1;
}

// Function: read @ 0x400530
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_601010)(); // jump-as-call
  return v1;
}

// Function: __libc_start_main @ 0x400540
void __libc_start_main(void)
{
  (*dat_601018)(); // jump-as-call
}

// Function: strcmp @ 0x400550
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_601020)(); // jump-as-call
  return v1;
}

// Function: open @ 0x400560
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_601028)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x400570
void exit(int a0)
{
  (*dat_601030)(); // jump-as-call
}

// Function: _start @ 0x400580
void _start(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  __libc_start_main(main,v2,&Stack0000000000000008,__libc_csu_init,__libc_csu_fini,a2,v1);
  do {
  } while( true );
}

// Function: call_gmon_start @ 0x4005ac
void call_gmon_start(void) // return-dupe
{
  if (!dat_600fe0)
    return;
  (*dat_600fe0)();
}

// Function: __do_global_dtors_aux @ 0x4005d0
void __do_global_dtors_aux(void) // return-dupe
{
  if (completed.6531)
    return;
  completed.6531 = 1;
}

// Function: frame_dummy @ 0x400640
void frame_dummy(void)
{
  return;
}

// Function: authenticate @ 0x400664
unsigned long authenticate(char *a0,char *a1) // return-dupe x2
{
  int v1; // eax
  char v2 [8];
  char v3; // stack - 0x10
  int v4; // stack - 0xc
  
  v3 = 0;
  v1 = strcmp(a1,sneaky);
  if (!v1)
    return 1;
  v4 = open(a0,0);
  read(v4,v2,8);
  if (!strcmp(a1,v2))
    return 1;
  return 0;
}

// Function: accepted @ 0x4006ed
void accepted(void)
{
  puts("Welcome to the admin console, trusted user!");
}

// Function: rejected @ 0x4006fd
void rejected(void)
{
  printf("Go away!");
  exit(1); // no-return
}

// Function: main @ 0x40071d
void main(void)
{
  char v1 [8];
  int v2; // stack - 0x2c
  char v3 [8];
  char v4; // stack - 0x20
  char v5; // stack - 0x10
  
  v5 = 0;
  v4 = 0;
  puts("Username: ");
  read(0,v1,8);
  read(0,&v2,1);
  puts("Password: ");
  read(0,v3,8);
  read(0,&v2,1);
  v2 = authenticate(v1,v3);
  if (v2) {
    accepted();
    return;
  }
  rejected(); // no-return
}

// Function: __libc_csu_init @ 0x4007e0
void __libc_csu_init(void)
{
  _init();
}

// Function: __libc_csu_fini @ 0x400870
void __libc_csu_fini(void)
{
  return;
}

// Function: __do_global_ctors_aux @ 0x400880
void __do_global_ctors_aux(void) // return-dupe
{
  void *v1;
  long *v2; // rbx
  
  if (dat_600e28 == (void *)0xffffffffffffffff)
    return;
  v2 = (long *)0x600e28;
  v1 = dat_600e28;
  do {
    v2 = &v2[-1];
    (*v1)();
    v1 = (void *)*v2;
  } while (v1 != (void *)0xffffffffffffffff);
}

// Function: _fini @ 0x4008b8
void _fini(void)
{
  __do_global_dtors_aux();
}

