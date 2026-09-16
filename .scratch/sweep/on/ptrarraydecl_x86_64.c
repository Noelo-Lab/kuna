// Function: _init @ 0x1000
void _init(void) // return-dupe
{
  if (!dat_3fe8)
    return;
  (*dat_3fe8)();
}

// Function: sub_1020 @ 0x1020
void sub_1020(void)
{
  (*dat_3fd0)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x1030
void __cxa_finalize(void)
{
  (*dat_3ff8)(); // jump-as-call
}

// Function: main @ 0x1040
int main(int argc,char **argv)
{
  int v1; // eax
  
  v1 = use_rows(argc);
  return use_names(argc) + v1;
}

// Function: _start @ 0x1060
void _start(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_3fd8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: deregister_tm_clones @ 0x1090
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x10c0
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x1100
void __do_global_dtors_aux(void)
{
  if (!completed.0) {
    if (dat_3ff8)
      __cxa_finalize(dat_4008);
    deregister_tm_clones();
    completed.0 = 1;
    return;
  }
}

// Function: frame_dummy @ 0x1140
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: get_row @ 0x1150
char (* get_row(int i))[16]
{
  return (char (*)[16])&(&rows)[(unsigned long)(i & 3) * 4];
}

// Function: use_rows @ 0x1170
int use_rows(int i)
{
  char (*pair [2])[16]; // stack - 0x18
  char (*v1)[16]; // rax
  unsigned int v2;
  unsigned long v3; // rcx
  
  v2 = i;
  v1 = get_row(i);
  v3 = (unsigned long)(v2 & 1);
  pair[0] = v1;
  pair[1] = get_row(i + 1);
  last_row = pair[v3];
  return (int)(*pair[1])[2] + (int)(*v1)[3];
}

// Function: get_names @ 0x11b0
char *(* get_names(void))[3]
{
  return (char *(*)[3])names;
}

// Function: use_names @ 0x11c0
int use_names(int i)
{
  char *first [2]; // stack - 0x18
  unsigned int v1;
  
  v1 = i;
  last_names = get_names();
  first[0] = (*last_names)[0];
  first[1] = (*last_names)[(int)(v1 + (((int)v1 / 3 + ((int)v1 >> 0x1f)) - (i >> 0x1f)) * -3)];
  return (int)*first[v1 & 1];
}

// Function: _fini @ 0x1210
void _fini(void)
{
  return;
}

