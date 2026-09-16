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

// Function: strlen @ 0x401060
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_404018)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x401070
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_404020)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x401080
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_404028)(); // jump-as-call
  return v1;
}

// Function: main @ 0x401090
bool main(int a0,unsigned long *a1)
{
  unsigned long v1;
  
  v1 = *a1;
  return make_dir_clone(v1,(long)a0) != 0;
}

// Function: _start @ 0x4010c0
void _start(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_403ff0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: _dl_relocate_static_pie @ 0x4010f0
void _dl_relocate_static_pie(void)
{
  return;
}

// Function: deregister_tm_clones @ 0x401100
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x401130
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x401170
void __do_global_dtors_aux(void)
{
  if (!completed.0) {
    deregister_tm_clones();
    completed.0 = 1;
    return;
  }
}

// Function: frame_dummy @ 0x4011a0
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: make_dir_clone @ 0x4011b0
unsigned long * make_dir_clone(char *a0,unsigned long a1)
{
  unsigned long v1;
  unsigned long *v2; // rax
  void *v3; // rax
  unsigned long v4;
  
  v1 = strlen(a0);
  v2 = malloc(0x60);
  *v2 = 0;
  v2[6] = 0;
  v2[5] = 0;
  v2[8] = 0;
  if ((2 <= v1) && (v4 = v1 - 1, a0[v1 - 1] == '/')) {
  }
  else {
    v4 = v1;
    v1 += 1;
  }
  v3 = malloc(v1);
  v2[0xb] = v3;
  v3 = memcpy(v3,a0,v4);
  v2[10] = a1;
  *(char *)((long)v3 + v4) = 0;
  v2[9] = 0;
  return v2;
}

// Function: _fini @ 0x401250
void _fini(void)
{
  return;
}

