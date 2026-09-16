// Function: sub_5a0 @ 0x5a0
void sub_5a0(void)
{
  long v1; // r2
  
  (**(void **)(v1 + -0x8018))(); // jump-as-call
}

// Function: _init @ 0x5c0
void _init(void)
{
  long v1; // r12
  
  if (*(long *)(v1 + 0x1f948))
    sub_5a0();
  sub_8b8();
  sub_948();
}

// Function: __libc_start_main @ 0x620
void __libc_start_main(void)
{
  long v1; // r2
  
  (**(void **)(v1 + -0x8028))(); // jump-as-call
}

// Function: __cxa_finalize @ 0x640
void __cxa_finalize(void)
{
  long v1; // r2
  
  (**(void **)(v1 + -0x8020))(); // jump-as-call
}

// Function: printf @ 0x660
int printf(char *a0,...)
{
  long v1; // r2
  void *v2; // r12
  
  v2 = *(void **)(v1 + -0x8008);
  return (*v2)((int)a0); // jump-as-call
}

// Function: puts @ 0x680
int puts(char *a0)
{
  long v1; // r2
  void *v2; // r12
  
  v2 = *(void **)(v1 + -0x8010);
  return (*v2)((int)a0); // jump-as-call
}

// Function: _start @ 0x6a0
void _start(void)
{
  __libc_start_main(); // tail-call
}

// Function: deregister_tm_clones @ 0x6f0
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x750
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x7c0
void __do_global_dtors_aux(void)
{
  unsigned long v1; // r9
  long v2; // r12
  unsigned long v3; // r30
  
  if (!*(char *)(v2 + 0x1f850)) {
    if (*(long *)(v2 + 0x1f760))
      __cxa_finalize(*(unsigned long *)(v2 + 0x1f848));
    v1 = *(unsigned long *)(v2 + 0x1f858);
    v3 = ((v2 + 0x1f510) - (v2 + 0x1f508) >> 3) - 1;
    if (v1 < v3) {
      do {
        *(unsigned long *)(v2 + 0x1f858) = v1 + 1;
        (**(void **)(v2 + 0x1f508 + (v1 + 1) * 8))();
        v1 = *(unsigned long *)(v2 + 0x1f858);
      } while (v1 < v3);
    }
    sub_6f8();
    *(char *)(v2 + 0x1f850) = 1;
    return;
  }
}

// Function: frame_dummy @ 0x8b0
void frame_dummy(void)
{
  return;
}

// Function: main @ 0x8bc
unsigned long main(int a0)
{
  long v1; // r12
  long v2; // stack - 0x68
  
  puts((char *)(v1 + 0x17c));
  printf((char *)(v2 + -0x274c0),(long)a0);
  return 0;
}

// Function: __do_global_ctors_aux @ 0x940
void __do_global_ctors_aux(void) // return-dupe
{
  long v1; // r12
  void *v2; // r12
  unsigned long *v3;
  
  v2 = *(void **)(v1 + 0x1f378);
  if (v2 == (void *)0xffffffffffffffff)
    return;
  v3 = (unsigned long *)(v1 + 0x1f378);
  do {
    (*v2)();
    v3 = &v3[-1];
    v2 = (void *)*v3;
  } while (v2 != (void *)0xffffffffffffffff);
}

// Function: sub_9b8 @ 0x9b8
void sub_9b8(void)
{
  (*dat_1fec8)(); // jump-as-call
}

// Function: _fini @ 0xa00
void _fini(void)
{
  sub_7c8();
}

