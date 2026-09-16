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

// Function: atoi @ 0x401040
int atoi(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_404018)(); // jump-as-call
  return v1;
}

// Function: _start @ 0x401050
void _start(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_403ff0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: _dl_relocate_static_pie @ 0x401080
void _dl_relocate_static_pie(void)
{
  return;
}

// Function: deregister_tm_clones @ 0x401090
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x4010c0
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x401100
void __do_global_dtors_aux(void)
{
  if (!completed.0) {
    deregister_tm_clones();
    completed.0 = 1;
    return;
  }
}

// Function: frame_dummy @ 0x401130
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: lookup @ 0x401136
unsigned long * lookup(int a0) // return-dupe
{
  if (a0 == 0x50)
    return &s.0;
  return NULL;
}

// Function: is_digit_c @ 0x401155
unsigned long is_digit_c(int a0) // return-dupe
{
  if ((0x30 <= a0) && (a0 <= 0x39))
    return 1;
  return 0;
}

// Function: lookup_service @ 0x40117a
char * lookup_service(char *a0,int *a1)
{
  int v1; // eax
  char *v2; // rax
  
  v1 = (int)*a0;
  v1 = is_digit_c(v1);
  v2 = a0;
  if ((v1) && (v1 = atoi(a0), v1)) {
    v2 = (unsigned long *)lookup(v1);
    if (v2) // branch-flip
      v2 = *(char **)v2;
    else {
      *a1 = *a1 + 1;
      v2 = a0;
    }
  }
  return v2;
}

// Function: emit @ 0x4011f7
void emit(unsigned long a0)
{
  g_d = a0;
}

// Function: two_selects @ 0x401211
void two_selects(void)
{
  long v1;
  long v2;
  
  if (g_flag1) // branch-flip
    v1 = g_step;
  else {
    v1 = 0;
  }
  if (g_flag2) // branch-flip
    v2 = g_step;
  else {
    v2 = 0;
  }
  emit((((g_a - g_b) - v1) - g_c) - v2);
}

// Function: main @ 0x401283
int main(void)
{
  unsigned int v1; // stack - 0xc
  
  v1 = 0;
  two_selects();
  return (int)*(char *)lookup_service("99999",&v1);
}

// Function: _fini @ 0x4012b4
void _fini(void)
{
  return;
}

