// Function: _DT_INIT @ 0x1000
void _DT_INIT(void) // return-dupe
{
  if (!dat_3fe8)
    return;
  (*dat_3fe8)();
}

// Function: sub_1020 @ 0x1020
void sub_1020(void)
{
  (*dat_3f88)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x10c0
void __cxa_finalize(void)
{
  (*dat_3ff8)(); // jump-as-call
}

// Function: free @ 0x10d0
void free(void *a0)
{
  (*dat_3f90)(); // jump-as-call
}

// Function: fread @ 0x10e0
unsigned long fread(void *a0,unsigned long a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_3f98)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x10f0
int fclose(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_3fa0)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x1100
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_3fa8)(); // jump-as-call
  return v1;
}

// Function: fprintf @ 0x1110
int fprintf(void *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_3fb0)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x1120
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3fb8)(); // jump-as-call
  return v1;
}

// Function: fopen @ 0x1130
void * fopen(char *a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_3fc0)(); // jump-as-call
  return v1;
}

// Function: perror @ 0x1140
void perror(char *a0)
{
  (*dat_3fc8)(); // jump-as-call
}

// Function: fwrite @ 0x1150
unsigned long fwrite(void *a0,unsigned long a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_3fd0)(); // jump-as-call
  return v1;
}

// Function: sub_1160 @ 0x1160
void sub_1160(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_3fd8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_1190 @ 0x1190
void sub_1190(void)
{
  return;
}

// Function: _FINI_0 @ 0x1200
void _FINI_0(void)
{
  if (!dat_4028) {
    if (dat_3ff8)
      __cxa_finalize(dat_4008);
    sub_1190();
    dat_4028 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x1240
void _INIT_0(void)
{
  return;
}

// Function: sub_1249 @ 0x1249
void * sub_1249(void *a0) // early-return x3
{
  void *v1; // rax
  
  v1 = malloc(0x40);
  if (!v1) {
    perror("malloc failed");
    fclose(a0);
    return NULL;
  }
  if (fread(v1,0x40,1,a0) != 1) {
    perror("fread failed");
    free(v1);
    fclose(a0);
    return NULL;
  }
  if (memcmp(v1,(void *)0x4010,4)) {
    fwrite("This is not a valid ELF file\n",1,0x1d,stderr);
    free(v1);
    fclose(a0);
    return NULL;
  }
  fclose(a0);
  return v1;
}

// Function: sub_1357 @ 0x1357
unsigned long sub_1357(char *a0) // return-dupe x3
{
  void *v1; // rax
  
  v1 = fopen(a0,"rb");
  if (!v1) {
    perror("fopen failed");
    return 1;
  }
  v1 = (void *)sub_1249(v1);
  if (!v1)
    return 1;
  if (*(char *)((long)v1 + 4) != '\x02') {
    fwrite("This is not a 64-bit ELF file\n",1,0x1e,stderr);
    free(v1);
    return 1;
  }
  return 0;
}

// Function: main @ 0x1405
int main(int argc,char **argv,char **envp)
{
  int v1; // eax
  
  if (2 <= argc) // branch-flip
    v1 = sub_1357(argv[1]);
  else {
    fprintf(stderr,"Usage: %s <binary>\n",*argv);
    v1 = 1;
  }
  return v1;
}

// Function: _DT_FINI @ 0x1464
void _DT_FINI(void)
{
  return;
}

