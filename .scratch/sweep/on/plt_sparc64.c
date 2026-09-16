// Function: _init @ 0x1004a0
unsigned long long _init(unsigned long long a0) // injected
{
  if ((int)dat_202008)
    __gmon_start__();
  frame_dummy();
  __do_global_ctors_aux();
  return a0;
}

// Function: sub_1004b0 @ 0x1004b0
void sub_1004b0(void)
{
  long long v1; // l7
  
  if ((int)*(unsigned long long *)(v1 + 8))
    __gmon_start__();
  frame_dummy();
  __do_global_ctors_aux();
}

// Function: sub_1004d0 @ 0x1004d0
void sub_1004d0(void)
{
  frame_dummy();
  __do_global_ctors_aux();
}

// Function: sub_1004d8 @ 0x1004d8
void sub_1004d8(void)
{
  __do_global_ctors_aux();
}

// Function: sub_1004e0 @ 0x1004e0
void sub_1004e0(void)
{
  return;
}

// Function: _start @ 0x100500
void _start(void) // injected
{
  unsigned long long v1; // stack + 0x87f
  
  __libc_start_main(0x200858,v1,&Stack0000000000000887,0,0);
  IllegalInstructionTrap(0);
}

// Function: sub_10050c @ 0x10050c
void sub_10050c(void)
{
  long long v1; // l7
  unsigned long long v2; // stack + 0x87f
  
  __libc_start_main(v1 + -0x1018b0,v2,&Stack0000000000000887,0,0);
  IllegalInstructionTrap(0);
}

// Function: sub_10053c @ 0x10053c
void sub_10053c(void)
{
  IllegalInstructionTrap(0);
}

// Function: __sparc_get_pc_thunk.l7 @ 0x100540
void __sparc_get_pc_thunk.l7(void)
{
  return;
}

// Function: deregister_tm_clones @ 0x100560
unsigned long long deregister_tm_clones(unsigned long long a0)
{
  return a0;
}

// Function: register_tm_clones @ 0x1005cc
unsigned long long register_tm_clones(unsigned long long a0)
{
  return a0;
}

// Function: __do_global_dtors_aux @ 0x100648
unsigned long long __do_global_dtors_aux(unsigned long long a0)
{
  if (!completed.1) {
    deregister_tm_clones();
    completed.1 = '\x01';
  }
  return a0;
}

// Function: sub_100714 @ 0x100714
void sub_100714(void)
{
  long long v1; // i2
  long long v2; // i3
  
  *(char *)((v2 << 0x20) + v1 + 0x210) = 1;
}

// Function: call___do_global_dtors_aux @ 0x10072c
unsigned long long call___do_global_dtors_aux(unsigned long long a0)
{
  return a0;
}

// Function: frame_dummy @ 0x100738
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: call_frame_dummy @ 0x100744
unsigned long long call_frame_dummy(unsigned long long a0)
{
  return a0;
}

// Function: main @ 0x100750
unsigned long long main(int a0)
{
  puts("hello");
  printf("%d\n",(long long)a0);
  return 0;
}

// Function: sub_100770 @ 0x100770
unsigned long long sub_100770(void)
{
  long long v1; // fp
  
  printf("%d\n",(long long)*(int *)(v1 + 0x87f));
  return 0;
}

// Function: sub_10078c @ 0x10078c
unsigned long long sub_10078c(void)
{
  return 0;
}

// Function: __do_global_ctors_aux @ 0x1007a0
unsigned long long __do_global_ctors_aux(unsigned long long a0)
{
  void *v1;
  unsigned long long *v2; // i5
  
  if (dat_201e10 != (void *)0xffffffffffffffff) {
    v2 = (unsigned long long *)0x201e10;
    v1 = dat_201e10;
    do {
      v2 = &v2[-1];
      (*v1)();
      v1 = (void *)*v2;
    } while (v1 != (void *)0xffffffffffffffff);
  }
  return a0;
}

// Function: call___do_global_ctors_aux @ 0x1007ec
unsigned long long call___do_global_ctors_aux(unsigned long long a0)
{
  return a0;
}

// Function: _fini @ 0x1007f8
unsigned long long _fini(unsigned long long a0)
{
  __do_global_dtors_aux();
  return a0;
}

// Function: sub_100804 @ 0x100804
void sub_100804(void)
{
  return;
}

// Function: __libc_start_main @ 0x202180
void __libc_start_main(void)
{
  do {
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
    IllegalInstructionTrap(0);
  } while( true );
}

// Function: printf @ 0x2021a0
int printf(char *a0,...)
{
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: puts @ 0x2021c0
int puts(char *a0)
{
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: __gmon_start__ @ 0x2021e0
void __gmon_start__(void)
{
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0);
  IllegalInstructionTrap(0); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

