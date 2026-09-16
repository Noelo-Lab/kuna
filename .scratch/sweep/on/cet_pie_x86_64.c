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

// Function: _start @ 0x1160
void _start(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_3fd8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: deregister_tm_clones @ 0x1190
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x11c0
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x1200
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

// Function: frame_dummy @ 0x1240
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: get_elf_header @ 0x1249
Elf64_Ehdr * get_elf_header(_IO_FILE *fp) // early-return x3
{
  Elf64_Ehdr *v1; // rax
  
  v1 = malloc(0x40);
  if (!v1) {
    perror("malloc");
    fclose(fp);
    return NULL;
  }
  if (fread(v1,0x40,1,fp) != 1) {
    perror("fread");
    free(v1);
    fclose(fp);
    return NULL;
  }
  if (memcmp(v1,&ELF_MAGIC,4)) {
    fwrite("Not a valid ELF file\n",1,0x15,stderr);
    free(v1);
    fclose(fp);
    return NULL;
  }
  fclose(fp);
  return v1;
}

// Function: elaborate_debug_symbol @ 0x1357
int elaborate_debug_symbol(char *binary) // return-dupe x3
{
  _IO_FILE *fp; // rax
  Elf64_Ehdr *v1; // rax
  
  fp = fopen(binary,"rb");
  if (!fp) {
    perror("fopen");
    return 1;
  }
  v1 = get_elf_header(fp);
  if (!v1)
    return 1;
  if (v1->e_ident[4] != '\x02') {
    fwrite("Not a 64-bit ELF file\n",1,0x16,stderr);
    free(v1);
    return 1;
  }
  return 0;
}

// Function: main @ 0x1405
int main(int argc,char **argv)
{
  int v1; // eax
  
  if (2 <= argc) // branch-flip
    v1 = elaborate_debug_symbol(argv[1]);
  else {
    fprintf(stderr,"Usage: %s <binary>\n",*argv);
    v1 = 1;
  }
  return v1;
}

// Function: _fini @ 0x1464
void _fini(void)
{
  return;
}

