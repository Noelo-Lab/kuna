// Function: _DT_INIT @ 0x4000
void _DT_INIT(void) // return-dupe
{
  if (!dat_23fd0)
    return;
  (*dat_23fd0)();
}

// Function: sub_4020 @ 0x4020
void sub_4020(void)
{
  (*dat_23d28)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x4550
void __cxa_finalize(void)
{
  (*dat_23fe0)(); // jump-as-call
}

// Function: sigprocmask @ 0x4560
int sigprocmask(int a0,void *a1,void *a2)
{
  int v1; // eax
  
  v1 = (*dat_23d30)(); // jump-as-call
  return v1;
}

// Function: raise @ 0x4570
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_23d38)(); // jump-as-call
  return v1;
}

// Function: free @ 0x4580
void free(void *a0)
{
  (*dat_23d40)(); // jump-as-call
}

// Function: strtoimax @ 0x4590
void strtoimax(void)
{
  (*dat_23d48)(); // jump-as-call
}

// Function: strcasecmp @ 0x45a0
int strcasecmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_23d50)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x45b0
void abort(void)
{
  (*dat_23d58)(); // jump-as-call
}

// Function: __errno_location @ 0x45c0
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_23d60)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x45d0
void _exit(int a0)
{
  (*dat_23d68)(); // jump-as-call
}

// Function: strcpy @ 0x45e0
char * strcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_23d70)(); // jump-as-call
  return v1;
}

// Function: qsort @ 0x45f0
void qsort(void)
{
  (*dat_23d78)(); // jump-as-call
}

// Function: isatty @ 0x4600
int isatty(int a0)
{
  int v1; // eax
  
  v1 = (*dat_23d80)(); // jump-as-call
  return v1;
}

// Function: sigaction @ 0x4610
int sigaction(int a0,void *a1,void *a2)
{
  int v1; // eax
  
  v1 = (*dat_23d88)(); // jump-as-call
  return v1;
}

// Function: strtod @ 0x4620
void strtod(void)
{
  (*dat_23d90)(); // jump-as-call
}

// Function: vsnprintf @ 0x4630
int vsnprintf(char *a0,unsigned long a1,char *a2,void *a3)
{
  int v1; // eax
  
  v1 = (*dat_23d98)(); // jump-as-call
  return v1;
}

// Function: strchrnul @ 0x4640
void strchrnul(void)
{
  (*dat_23da0)(); // jump-as-call
}

// Function: faccessat @ 0x4650
void faccessat(void)
{
  (*dat_23da8)(); // jump-as-call
}

// Function: fcntl @ 0x4660
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_23db0)(); // jump-as-call
  return v1;
}

// Function: write @ 0x4670
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_23db8)(); // jump-as-call
  return v1;
}

// Function: getpid @ 0x4680
int getpid(void)
{
  int v1; // eax
  
  v1 = (*dat_23dc0)(); // jump-as-call
  return v1;
}

// Function: lstat64 @ 0x4690
void lstat64(void)
{
  (*dat_23dc8)(); // jump-as-call
}

// Function: wait3 @ 0x46a0
void wait3(void)
{
  (*dat_23dd0)(); // jump-as-call
}

// Function: opendir @ 0x46b0
void * opendir(char *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_23dd8)(); // jump-as-call
  return v1;
}

// Function: stpcpy @ 0x46c0
char * stpcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_23de0)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x46d0
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_23de8)(); // jump-as-call
  return v1;
}

// Function: chdir @ 0x46e0
int chdir(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_23df0)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x46f0
void __stack_chk_fail(void)
{
  (*dat_23df8)(); // jump-as-call
}

// Function: dup2 @ 0x4700
int dup2(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_23e00)(); // jump-as-call
  return v1;
}

// Function: stat64 @ 0x4710
void stat64(void)
{
  (*dat_23e08)(); // jump-as-call
}

// Function: strchr @ 0x4720
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_23e10)(); // jump-as-call
  return v1;
}

// Function: fnmatch @ 0x4730
int fnmatch(char *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_23e18)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x4740
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_23e20)(); // jump-as-call
  return v1;
}

// Function: geteuid @ 0x4750
unsigned int geteuid(void)
{
  unsigned int v1; // eax
  
  v1 = (*dat_23e28)(); // jump-as-call
  return v1;
}

// Function: getcwd @ 0x4760
char * getcwd(char *a0,unsigned long a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_23e30)(); // jump-as-call
  return v1;
}

// Function: close @ 0x4770
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_23e38)(); // jump-as-call
  return v1;
}

// Function: pipe @ 0x4780
int pipe(int *a0)
{
  int v1; // eax
  
  v1 = (*dat_23e40)(); // jump-as-call
  return v1;
}

// Function: strspn @ 0x4790
unsigned long strspn(char *a0,char *a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_23e48)(); // jump-as-call
  return v1;
}

// Function: closedir @ 0x47a0
int closedir(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_23e50)(); // jump-as-call
  return v1;
}

// Function: strcspn @ 0x47b0
unsigned long strcspn(char *a0,char *a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_23e58)(); // jump-as-call
  return v1;
}

// Function: read @ 0x47c0
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_23e60)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x47d0
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_23e68)(); // jump-as-call
  return v1;
}

// Function: _setjmp @ 0x47e0
void _setjmp(void)
{
  (*dat_23e70)(); // jump-as-call
}

// Function: execve @ 0x47f0
int execve(char *a0,char **a1,char **a2)
{
  int v1; // eax
  
  v1 = (*dat_23e78)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x4800
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_23e80)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x4810
void signal(void)
{
  (*dat_23e88)(); // jump-as-call
}

// Function: getpwnam @ 0x4820
void * getpwnam(char *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_23e90)(); // jump-as-call
  return v1;
}

// Function: umask @ 0x4830
unsigned int umask(unsigned int a0)
{
  unsigned int v1; // eax
  
  v1 = (*dat_23e98)(); // jump-as-call
  return v1;
}

// Function: sigfillset @ 0x4840
void sigfillset(void)
{
  (*dat_23ea0)(); // jump-as-call
}

// Function: memcpy @ 0x4850
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_23ea8)(); // jump-as-call
  return v1;
}

// Function: kill @ 0x4860
int kill(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_23eb0)(); // jump-as-call
  return v1;
}

// Function: tcgetpgrp @ 0x4870
void tcgetpgrp(void)
{
  (*dat_23eb8)(); // jump-as-call
}

// Function: malloc @ 0x4880
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_23ec0)(); // jump-as-call
  return v1;
}

// Function: killpg @ 0x4890
void killpg(void)
{
  (*dat_23ec8)(); // jump-as-call
}

// Function: getegid @ 0x48a0
void getegid(void)
{
  (*dat_23ed0)(); // jump-as-call
}

// Function: strpbrk @ 0x48b0
char * strpbrk(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_23ed8)(); // jump-as-call
  return v1;
}

// Function: sigsuspend @ 0x48c0
void sigsuspend(void)
{
  (*dat_23ee0)(); // jump-as-call
}

// Function: setrlimit @ 0x48d0
void setrlimit(void)
{
  (*dat_23ee8)(); // jump-as-call
}

// Function: strsignal @ 0x48e0
void strsignal(void)
{
  (*dat_23ef0)(); // jump-as-call
}

// Function: realloc @ 0x48f0
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_23ef8)(); // jump-as-call
  return v1;
}

// Function: stpncpy @ 0x4900
char * stpncpy(char *a0,char *a1,unsigned long a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_23f00)(); // jump-as-call
  return v1;
}

// Function: sigsetmask @ 0x4910
void sigsetmask(void)
{
  (*dat_23f08)(); // jump-as-call
}

// Function: longjmp @ 0x4920
void longjmp(void)
{
  (*dat_23f10)(); // jump-as-call
}

// Function: open64 @ 0x4930
void open64(void)
{
  (*dat_23f18)(); // jump-as-call
}

// Function: mempcpy @ 0x4940
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_23f20)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x4950
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_23f28)(); // jump-as-call
  return v1;
}

// Function: readdir64 @ 0x4960
void readdir64(void)
{
  (*dat_23f30)(); // jump-as-call
}

// Function: getpgrp @ 0x4970
void getpgrp(void)
{
  (*dat_23f38)(); // jump-as-call
}

// Function: tcsetpgrp @ 0x4980
void tcsetpgrp(void)
{
  (*dat_23f40)(); // jump-as-call
}

// Function: times @ 0x4990
void times(void)
{
  (*dat_23f48)(); // jump-as-call
}

// Function: strtok @ 0x49a0
char * strtok(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_23f50)(); // jump-as-call
  return v1;
}

// Function: sysconf @ 0x49b0
long sysconf(int a0)
{
  long v1; // rax
  
  v1 = (*dat_23f58)(); // jump-as-call
  return v1;
}

// Function: strtoumax @ 0x49c0
void strtoumax(void)
{
  (*dat_23f60)(); // jump-as-call
}

// Function: vfork @ 0x49d0
void vfork(void)
{
  (*dat_23f68)(); // jump-as-call
}

// Function: atoi @ 0x49e0
int atoi(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_23f70)(); // jump-as-call
  return v1;
}

// Function: setpgid @ 0x49f0
void setpgid(void)
{
  (*dat_23f78)(); // jump-as-call
}

// Function: getppid @ 0x4a00
void getppid(void)
{
  (*dat_23f80)(); // jump-as-call
}

// Function: fstat64 @ 0x4a10
void fstat64(void)
{
  (*dat_23f88)(); // jump-as-call
}

// Function: getrlimit @ 0x4a20
void getrlimit(void)
{
  (*dat_23f90)(); // jump-as-call
}

// Function: bsearch @ 0x4a30
void bsearch(void)
{
  (*dat_23f98)(); // jump-as-call
}

// Function: strdup @ 0x4a40
char * strdup(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_23fa0)(); // jump-as-call
  return v1;
}

// Function: strerror @ 0x4a50
char * strerror(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_23fa8)(); // jump-as-call
  return v1;
}

// Function: fork @ 0x4a60
int fork(void)
{
  int v1; // eax
  
  v1 = (*dat_23fb0)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x4a70
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_23fb8)(); // jump-as-call
  return v1;
}

// Function: sub_4a80 @ 0x4a80
void sub_4a80(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_23fc0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_4ab0 @ 0x4ab0
void sub_4ab0(void)
{
  return;
}

// Function: _FINI_0 @ 0x4b20
void _FINI_0(void)
{
  if (!dat_24268) {
    if (dat_23fe0)
      __cxa_finalize(dat_24008);
    sub_4ab0();
    dat_24268 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x4b60
void _INIT_0(void)
{
  return;
}

// Function: sub_4b69 @ 0x4b69
void sub_4b69(unsigned long a0,unsigned long a1)
{
  long v1;
  long *v2; // rax
  unsigned long *v3; // rax
  
  v2 = (long *)sub_50b9(a0);
  v1 = *v2;
  dat_243e4 += 1;
  if (v1) { // branch-flip
    if (!(*(unsigned int *)(v1 + 0x18) & 1))
      free(*(void **)(v1 + 0x10));
    *(unsigned long *)(v1 + 0x10) = sub_103e1(a1);
    *(unsigned int *)(v1 + 0x18) = *(unsigned int *)(v1 + 0x18) & 0xfffffffd;
  }
  else {
    v3 = (unsigned long *)sub_10354(0x20);
    v3[1] = sub_103e1(a0);
    v3[2] = sub_103e1(a1);
    *(unsigned int *)&v3[3] = 0;
    *v3 = 0;
    *v2 = (long)v3;
  }
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
}

// Function: sub_4c77 @ 0x4c77
unsigned long sub_4c77(unsigned long a0) // return-dupe
{
  long v1;
  long *v2; // rax
  
  v2 = (long *)sub_50b9(a0);
  if (!*v2)
    return 1;
  dat_243e4 += 1;
  v1 = *v2;
  *v2 = sub_5000(v1);
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
  return 0;
}

// Function: sub_4cfe @ 0x4cfe
void sub_4cfe(void)
{
  unsigned long v1;
  int v2; // stack - 0x1c
  unsigned long *v3; // stack - 0x18
  unsigned long *v4; // stack - 0x10
  
  dat_243e4 += 1;
  for (v2 = 0; v2 <= 0x26; v2 = v2 + 1) {
    v4 = (unsigned long *)((long)v2 * 8 + 0x24280);
    v3 = (unsigned long *)*v4;
    while (v3) {
      v1 = *v4;
      *v4 = sub_5000(v1);
      if (v3 == (unsigned long *)*v4)
        v4 = v3;
      v3 = (unsigned long *)*v4;
    }
  }
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
}

// Function: sub_4dbc @ 0x4dbc
long sub_4dbc(unsigned long a0,int a1) // early-return
{
  long v1;
  
  v1 = *(long *)sub_50b9(a0);
  if (((a1) && (v1)) && (*(unsigned int *)(v1 + 0x18) & 1))
    return 0;
  return v1;
}

// Function: sub_4e0a @ 0x4e0a
unsigned int sub_4e0a(int a0,long *a1) // early-return
{
  long v1;
  long v2;
  char *v3; // rax
  long *v4; // stack - 0x38
  unsigned int v5; // stack - 0x28
  int v6; // stack - 0x24
  unsigned long *v7; // stack - 0x20
  
  v5 = 0;
  v4 = a1;
  if (a0 == 1) {
    for (v6 = 0; v6 <= 0x26; v6 = v6 + 1) {
      for (v7 = *(unsigned long **)((long)v6 * 8 + 0x24280); v7; v7 = (unsigned long *)*v7) {
        sub_5074(v7);
      }
    }
    return 0;
  }
  while( true ) {
    v4 = &v4[1];
    v2 = *v4;
    if (!v2) break;
    v3 = strchr((char *)(v2 + 1),0x3d);
    if (v3) { // branch-flip
      *v3 = '\0';
      sub_4b69(v2,&v3[1]);
    }
    else {
      v1 = *(long *)sub_50b9(v2);
      if (v1) // branch-flip
        sub_5074(v1);
      else {
        sub_1714e(dat_240d0,"%s: %s not found\n","alias",v2);
        v5 = 1;
      }
    }
  }
  return v5;
}

// Function: sub_4f43 @ 0x4f43
unsigned int sub_4f43(void)
{
  long v1;
  int v2; // eax
  unsigned int v3; // stack - 0xc
  
  do {
    v2 = sub_12769("a");
    if (!v2) {
      v3 = 0;
      for (; *dat_26a18; dat_26a18 = &dat_26a18[1]) {
        v1 = *dat_26a18;
        if (sub_4c77(v1)) {
          sub_1714e(dat_240d0,"%s: %s not found\n","unalias",*dat_26a18);
          v3 = 1;
        }
      }
      return v3;
    }
  } while (v2 != 0x61);
  sub_4cfe();
  return 0;
}

// Function: sub_5000 @ 0x5000
unsigned long * sub_5000(unsigned long *a0)
{
  unsigned long *v1;
  unsigned int v2; // eax
  unsigned long *v3; // rax
  
  v2 = *(unsigned int *)&a0[3] & 1;
  if (v2) { // branch-flip
    v2 = *(unsigned int *)&a0[3] | 2;
    *(unsigned int *)&a0[3] = v2;
    v3 = a0;
  }
  else {
    v1 = (unsigned long *)*a0;
    free((void *)a0[1]);
    free((void *)a0[2]);
    free(a0);
    v3 = v1;
  }
  return v3;
}

// Function: sub_5074 @ 0x5074
void sub_5074(long a0)
{
  unsigned long v1; // rax
  
  v1 = sub_117ce(*(unsigned long *)(a0 + 0x10));
  sub_17224("%s=%s\n",*(unsigned long *)(a0 + 8),v1);
}

// Function: sub_50b9 @ 0x50b9
long * sub_50b9(unsigned char *a0)
{
  char *v1;
  unsigned int v2; // stack - 0x20
  unsigned int v3; // stack - 0x1c
  long *v4; // stack - 0x18
  unsigned char *v5; // stack - 0x10
  
  v3 = (unsigned int)*a0;
  v2 = v3 << 4;
  v5 = a0;
  while (v3) {
    v2 += v3;
    v5 = &v5[1];
    v3 = (unsigned int)*v5;
  }
  v4 = (long *)((unsigned long)(v2 % 0x27) * 8 + 0x24280);
  while ((*v4 && (v1 = *(char **)(*v4 + 8), strcmp((char *)a0,v1)))) {
    v4 = (long *)*v4;
  }
  return v4;
}

// Function: sub_5186 @ 0x5186
void sub_5186(unsigned long a0)
{
  sub_6621("arithmetic expression: %s: \"%s\"",a0,dat_243c8); // no-return
}

// Function: sub_51b8 @ 0x51b8
int sub_51b8(int a0)
{
  return (int)*(char *)((long)(a0 + -8) + 0x1c030);
}

// Function: sub_51d7 @ 0x51d7
bool sub_51d7(unsigned int a0,unsigned int a1)
{
  int v1; // eax
  
  v1 = sub_51b8(a0);
  return v1 < (int)sub_51b8(a1);
}

// Function: sub_520a @ 0x520a
unsigned long sub_520a(int a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // rax
  
  switch(a0) {
    case 8:
      v1 = (unsigned long)((long)a1 <= (long)a2);
      break;
    case 9:
      v1 = (unsigned long)((long)a2 <= (long)a1);
      break;
    case 10:
      v1 = (unsigned long)((long)a1 < (long)a2);
      break;
    case 0xb:
      v1 = (unsigned long)((long)a2 < (long)a1);
      break;
    case 0xc:
      v1 = (unsigned long)(a1 == a2);
      break;
    default:
      if (!a2)
        sub_5186("division by zero"); // no-return
      if (a0 != 0xd) // branch-flip
        v1 = (long)a1 / (long)a2;
      else {
        v1 = (long)a1 % (long)a2;
      }
      break;
    case 0xe:
      v1 = a1 & a2;
      break;
    case 0xf:
      v1 = a1 << ((unsigned char)a2 & 0x3f);
      break;
    case 0x10:
      v1 = (long)a1 >> ((unsigned char)a2 & 0x3f);
      break;
    case 0x11:
      v1 = a1 * a2;
      break;
    case 0x12:
      v1 = a2 + a1;
      break;
    case 0x13:
      v1 = a1 | a2;
      break;
    case 0x14:
      v1 = a1 - a2;
      break;
    case 0x15:
      v1 = a1 ^ a2;
      break;
    case 0x17:
      v1 = (unsigned long)(a1 != a2);
    
  }
  return v1;
}

// Function: sub_535a @ 0x535a
unsigned long sub_535a(unsigned int a0,unsigned long *a1,unsigned int a2,int a3)
{
  unsigned int v1; // eax
  unsigned long v2;
  unsigned int v3; // stack - 0x20
  unsigned int v4; // stack - 0x1c
  
  v3 = a2;
  v4 = a0;
  do {
    switch(v4) {
      case 5:
        dat_243d0 = v3;
        return *a1;
      case 6:
        dat_243d0 = v3;
        if (a3)
          return *a1;
        v2 = *a1;
        return sub_19de7(v2);
      case 7:
        *a1 = dat_243c0;
        v1 = sub_59f1();
        return (unsigned long)(sub_535a(v3,a1,v1,a3) == 0);
      default:
        sub_5186("expecting primary"); // no-return
      case 0x12:
        v4 = v3;
        *a1 = dat_243c0;
        v3 = sub_59f1();
        break;
      case 0x14:
        *a1 = dat_243c0;
        v1 = sub_59f1();
        return -sub_535a(v3,a1,v1,a3);
      case 0x22:
        v2 = sub_589d(v3,a3);
        if (dat_243d0 == 0x23) {
          dat_243d0 = sub_59f1();
          return v2;
        }
        sub_5186("expecting \')\'"); // no-return
      case 0x24:
        *a1 = dat_243c0;
        v1 = sub_59f1();
        return ~sub_535a(v3,a1,v1,a3);
      
    }
  } while( true );
}

// Function: sub_54ef @ 0x54ef
unsigned long sub_54ef(unsigned long a0,int a1,int a2,int a3)
{
  int v1;
  unsigned int v2; // eax
  int v3; // eax
  unsigned long v4; // stack - 0x20
  int v5; // stack - 0x34
  unsigned long v6; // stack - 0x30
  int v7; // stack - 0x28
  unsigned long v8; // stack - 0x18
  
  v5 = a1;
  v6 = a0;
  while( true ) {
    v2 = sub_59f1();
    v4 = dat_243c0;
    v8 = sub_535a(v2,&v4,sub_59f1(),a3);
    v1 = dat_243d0;
    v7 = dat_243d0;
    if (((8 <= dat_243d0) && (dat_243d0 <= 0x17)) && (v3 = sub_51d7(dat_243d0,v5), v3)) {
      v2 = sub_51b8(v5);
      v8 = sub_54ef(v8,v1,v2,a3);
      v7 = dat_243d0;
    }
    if (a3) // branch-flip
      v6 = v8;
    else {
      v6 = sub_520a(v5,v6,v8);
    }
    if (((v7 <= 7) || (0x17 < v7)) || (a2 <= (int)sub_51b8(v7))) break;
    v5 = v7;
  }
  return v6;
}

// Function: sub_5608 @ 0x5608
unsigned long sub_5608(unsigned int a0,unsigned long a1,unsigned int a2,unsigned int a3)
{
  unsigned long v1; // rax
  
  v1 = sub_535a(a0,a1,a2,a3);
  if ((dat_243d0 > 7) && (0x18 > dat_243d0))
    v1 = sub_54ef(v1,dat_243d0,8,a3);
  return v1;
}

// Function: sub_566f @ 0x566f
long sub_566f(unsigned int a0,unsigned long *a1,unsigned int a2,unsigned int a3) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_566f
{
  unsigned int v1;
  unsigned int v2;
  long v3;
  unsigned int v4; // eax
  long v5; // rax
  
  v5 = sub_5608(a0,a1,a2,a3);
  v3 = v5;
  if (dat_243d0 == 3) {
    v4 = sub_59f1();
    v1 = v4;
    *a1 = dat_243c0;
    v4 = (unsigned int)(v3 == 0);
    v4 |= a3;
    v2 = v4;
    v4 = sub_59f1();
    v5 = sub_566f(v1,a1,v4,v2);
    v4 = ((v3) && (v5));
    v5 = (long)(int)v4;
  }
  return v5;
}

// Function: sub_5719 @ 0x5719
long sub_5719(unsigned int a0,unsigned long *a1,unsigned int a2,unsigned int a3) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_5719
{
  unsigned int v1;
  unsigned int v2;
  long v3;
  unsigned int v4; // eax
  long v5; // rax
  
  v5 = sub_566f(a0,a1,a2,a3);
  v3 = v5;
  if (dat_243d0 == 2) {
    v4 = sub_59f1();
    v1 = v4;
    *a1 = dat_243c0;
    v4 = (unsigned int)(v3 != 0);
    v4 |= a3;
    v2 = v4;
    v4 = sub_59f1();
    v5 = sub_5719(v1,a1,v4,v2);
    v4 = ((v3) || (v5));
    v5 = (long)(int)v4;
  }
  return v5;
}

// Function: sub_57c3 @ 0x57c3
long sub_57c3(unsigned int a0,unsigned long *a1,unsigned int a2,unsigned int a3)
{
  unsigned int v1;
  unsigned int v2;
  long v3;
  long v4;
  unsigned int v5; // eax
  long v6; // rax
  
  v6 = sub_5719(a0,a1,a2,a3);
  v3 = v6;
  if (dat_243d0 != 0x25) { // branch-flip
  }
  else {
    v5 = (unsigned int)(v6 == 0);
    v5 |= a3;
    v1 = v5;
    v5 = sub_59f1();
    v6 = sub_589d(v5,v1);
    v4 = v6;
    if (dat_243d0 != 0x26)
      sub_5186("expecting \':\'"); // no-return
    v5 = sub_59f1();
    v2 = v5;
    *a1 = dat_243c0;
    v5 = (unsigned int)(v3 != 0);
    v5 |= a3;
    v1 = v5;
    v5 = sub_59f1();
    v6 = sub_57c3(v2,a1,v5,v1);
    if (v3)
      v6 = v4;
  }
  return v6;
}

// Function: sub_589d @ 0x589d
unsigned long sub_589d(int a0,int a1)
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x20
  unsigned long v4; // stack - 0x18
  
  v3 = dat_243c0;
  v1 = sub_59f1();
  if (a0 != 6) // branch-flip
    v2 = sub_57c3(a0,&v3,v1,a1);
  else if ((v1 != 1) && ((v1 <= 0x17 || (0x22 <= v1))))
    v2 = sub_57c3(6,&v3,v1,a1);
  else {
    v2 = sub_589d(sub_59f1(),a1);
    if (!a1) {
      v4 = v2;
      if (v1 != 1) {
        v2 = sub_19de7(v3);
        v2 = sub_520a(v1 + -0xb,v2,v4);
      }
      v2 = sub_199bd(v3,v2,0);
    }
  }
  return v2;
}

// Function: sub_5994 @ 0x5994
unsigned long sub_5994(unsigned long a0)
{
  unsigned long v1; // rax
  
  dat_243b8 = a0;
  dat_243c8 = a0;
  v1 = sub_589d(sub_59f1(),0);
  if (dat_243d0)
    sub_5186("expecting EOF"); // no-return
  return v1;
}

// Function: sub_59f1 @ 0x59f1
int sub_59f1(void) // early-return x2
{
  bool v1;
  unsigned char *v2;
  char *v3; // rax
  int v4; // stack - 0x1c
  unsigned char *v5; // stack - 0x18
  
  v5 = dat_243b8;
label_5a08:
  v2 = v5;
  v4 = (int)(char)*v5;
  switch(v4) {
    case 0:
      dat_243b8 = v5;
      return v4;
    default:
      return 4;
    case 9:
    case 10:
    case 0x20:
      goto label_5a3f;
    case 0x21:
      v5 = &v5[1];
      if (*v5 != 0x3d) {
        dat_243b8 = v5;
        return v4 + -0x1a;
      }
      v4 -= 10;
      dat_243b8 = &v5[1]; // return-dupe
      return v4;
    case 0x25:
      v4 -= 0x18;
      break;
    case 0x26:
      v5 = &v5[1];
      if (*v5 == 0x26) {
        v4 -= 0x23;
        dat_243b8 = &v5[1];
        return v4;
      }
      v4 -= 0x18;
      goto label_5b21;
    case 0x28:
      v4 -= 6;
      dat_243b8 = &v5[1];
      return v4;
    case 0x29:
      v4 -= 6;
      dat_243b8 = &v5[1];
      return v4;
    case 0x2a:
      v4 -= 0x19;
      break;
    case 0x2b:
      v4 -= 0x19;
      break;
    case 0x2d:
      v4 -= 0x19;
      break;
    case 0x2f:
      v4 -= 0x19;
      break;
    case 0x30:
    case 0x31:
    case 0x32:
    case 0x33:
    case 0x34:
    case 0x35:
    case 0x36:
    case 0x37:
    case 0x38:
    case 0x39:
      dat_243c0 = (unsigned long)strtoimax(v5,0x243b8,0);
      return 5;
    case 0x3a:
      v4 -= 0x14;
      dat_243b8 = &v5[1];
      return v4;
    case 0x3c:
      v5 = &v5[1];
      if (*v5 != 0x3c) {
        if (*v5 != 0x3d) {
          dat_243b8 = v5;
          return v4 + -0x32;
        }
        v4 -= 0x34;
        dat_243b8 = &v5[1];
        return v4;
      }
      v4 -= 0x2d;
      break;
    case 0x3d:
      v4 -= 0x3c;
      break;
    case 0x3e:
      v5 = &v5[1];
      if (*v5 == 0x3d) {
        v4 -= 0x35;
        dat_243b8 = &v5[1];
        return v4;
      }
      if (*v5 != 0x3e) {
        dat_243b8 = v5;
        return v4 + -0x33;
      }
      v4 -= 0x2e;
      break;
    case 0x3f:
      v4 -= 0x1a;
      dat_243b8 = &v5[1];
      return v4;
    case 0x41:
    case 0x42:
    case 0x43:
    case 0x44:
    case 0x45:
    case 0x46:
    case 0x47:
    case 0x48:
    case 0x49:
    case 0x4a:
    case 0x4b:
    case 0x4c:
    case 0x4d:
    case 0x4e:
    case 0x4f:
    case 0x50:
    case 0x51:
    case 0x52:
    case 0x53:
    case 0x54:
    case 0x55:
    case 0x56:
    case 0x57:
    case 0x58:
    case 0x59:
    case 0x5a:
    case 0x5f:
    case 0x61:
    case 0x62:
    case 99:
    case 100:
    case 0x65:
    case 0x66:
    case 0x67:
    case 0x68:
    case 0x69:
    case 0x6a:
    case 0x6b:
    case 0x6c:
    case 0x6d:
    case 0x6e:
    case 0x6f:
    case 0x70:
    case 0x71:
    case 0x72:
    case 0x73:
    case 0x74:
    case 0x75:
    case 0x76:
    case 0x77:
    case 0x78:
    case 0x79:
    case 0x7a:
      do {
        v5 = &v5[1];
        if ((*v5 != 0x5f) && (!(*(unsigned short *)((unsigned long)*v5 * 2 + *(long *)__ctype_b_loc()) & 8)))
          v1 = 0;
        else {
          v1 = 1;
        }
      } while (v1);
      dat_243c0 = (void *)sub_10422(&v5[1 - (long)v2]);
      v3 = mempcpy(dat_243c0,v2,(long)v5 - (long)v2);
      *v3 = 0;
      dat_243b8 = v5;
      return 6;
    case 0x5e:
      v4 -= 0x49;
      break;
    case 0x7c:
      v5 = &v5[1];
      if (*v5 == 0x7c) {
        v4 -= 0x7a;
        dat_243b8 = &v5[1];
        return v4;
      }
      v4 -= 0x69;
      goto label_5b21;
    case 0x7e:
      v4 -= 0x5a;
      dat_243b8 = &v5[1];
      return v4;
    
  }
  v5 = &v5[1];
label_5b21:
  if (*v5 != 0x3d) {
    dat_243b8 = v5;
    return v4;
  }
  v4 += 0xb;
  dat_243b8 = &v5[1];
  return v4;
label_5a3f:
  v5 = &v5[1];
  goto label_5a08;
}

// Function: sub_5c70 @ 0x5c70
void sub_5c70(unsigned long a0)
{
  sub_19d32(a0);
}

// Function: sub_5c8a @ 0x5c8a
unsigned int sub_5c8a(void)
{
  int v1; // eax
  unsigned int v2; // stack - 0x14
  int v3; // stack - 0x10
  
  v2 = 0;
  v3 = 0x4c;
  while( true ) {
    v1 = sub_12769("LP");
    if (!v1) break;
    if (v1 != v3) {
      v2 ^= 1;
      v3 = v1;
    }
  }
  return v2;
}

// Function: sub_5cd5 @ 0x5cd5
unsigned long sub_5cd5(void) // return-dupe
{
  char v1;
  unsigned int v10; // stack - 0x90
  int v2; // eax
  char *v3; // rax
  char v4 [24];
  char *v5; // stack - 0xc0
  char v6; // stack - 0xc9
  unsigned int v7; // stack - 0xc8
  char *v8; // stack - 0xb8
  char *v9; // stack - 0xb0
  
  v7 = sub_5c8a();
  v8 = (char *)*dat_26a18;
  if (v8) { // branch-flip
    if ((*v8 == '-') && (!v8[1])) {
      v8 = (char *)sub_5c70("OLDPWD");
      v7 |= 2;
    }
  }
  else {
    v8 = (char *)sub_5c70("HOME");
  }
  if (!v8)
    v8 = (char *)0x269e0;
  if (*v8 != '/') { // branch-flip
    if (*v8 == '.') {
      v6 = v8[1];
label_5dc7:
      switch(v6) {
        case 0:
        case 0x2f:
          goto label_5f13;
        default:
          break;
        case 0x2e:
          goto label_5de9;
        
      }
    }
label_5e08:
    if (!*v8)
      v8 = ".";
    v5 = (char *)sub_5c70("CDPATH");
    do {
      v9 = v5;
      v2 = sub_8b10(&v5,v8,0);
      if (v2 <= -1) goto label_5f13;
      v1 = *v9;
      v3 = (char *)sub_10422((long)v2);
      v9 = v3;
    } while (((int)stat64(v3,v4) < 0) || ((v10 & 0xf000) != 0x4000));
    if ((v1) && (v1 != ':'))
      v7 |= 2;
  }
  else {
label_5f13:
    v9 = v8;
  }
  v2 = sub_5f88(v9,v7);
  if (v2)
    sub_6621("can\'t cd to %s",v8); // no-return
  if (!(v7 & 2))
    return 0;
  sub_17224("%s\n",dat_24010);
  return 0;
label_5de9:
  v6 = v8[2];
  if (v6 == '.') goto label_5e08;
  goto label_5dc7;
}

// Function: sub_5f88 @ 0x5f88
int sub_5f88(char *a0,unsigned int a1)
{
  int v1; // eax
  char *v2; // stack - 0x20
  char *v3; // stack - 0x10
  
  v3 = NULL;
  dat_243e4 += 1;
  v2 = a0;
  if (!(a1 & 1)) {
    v3 = (char *)sub_6036(a0);
    if (v3)
      v2 = v3;
  }
  v1 = chdir(v2);
  if (!v1) {
    sub_6325(v3,1);
    sub_95dd();
  }
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
  return v1;
}

// Function: sub_6036 @ 0x6036
long sub_6036(char *a0) // early-return
{
  char *v1;
  char *v2;
  char *v3; // rax
  char *v4; // rax
  char *v5; // stack - 0x28
  char *v6; // stack - 0x20
  char *v7; // stack - 0x18
  char *v8; // stack - 0x10
  
  v2 = (char *)sub_11925(a0);
  v7 = v2;
  v5 = (char *)dat_24048;
  if (*a0 != '/') {
    if (dat_24010 == 0x269e0)
      return 0;
    v5 = (long)sub_1099d(dat_24010,dat_24048);
  }
  v3 = (char *)sub_1090b(strlen(a0) + 2,v5);
  v8 = (char *)(dat_24048 + 1);
  if (*a0 != '/') { // branch-flip
    v5 = v3;
    if (v3[-1] != '/') {
      v5 = &v3[1];
      *v3 = '/';
    }
    if ((v8 < v5) && (*v8 == '/'))
      v8 = (char *)(dat_24048 + 2);
  }
  else {
    v1 = &v3[1];
    *v3 = '/';
    v7 = &v2[1];
    v5 = v1;
    if ((a0[1] == '/') && (a0[2] != '/')) {
      v5 = &v3[2];
      *v1 = '/';
      v7 = &v2[2];
      v8 = (char *)(dat_24048 + 2);
    }
  }
  v6 = strtok(v7,"/");
  do {
    if (!v6) {
      if (v8 < v5)
        v5 = &v5[-1];
      *v5 = '\0';
      return dat_24048;
    }
    if (*v6 != '.') { // branch-flip
label_61e5:
      v4 = (char *)sub_1099d(v6,v5);
      *v4 = 0x2f;
      v5 = &v4[1];
    }
    else if ((v6[1] != '.') || (v6[2])) {
      if (v6[1]) goto label_61e5;
    }
    else {
      do {
        if (v5 <= v8) break;
        v2 = &v5[-1];
        v3 = &v5[-2];
        v5 = v2;
      } while (*v3 != '/');
    }
    v6 = strtok(NULL,"/");
  } while( true );
}

// Function: sub_6250 @ 0x6250
char * sub_6250(void) // early-return
{
  int v1;
  char *v2; // rax
  
  v2 = getcwd(NULL,0);
  if (v2)
    return v2;
  v1 = *dat_266e8;
  sub_679b("getcwd() failed: %s",strerror(v1));
  return (char *)0x269e0;
}

// Function: sub_62a8 @ 0x62a8
unsigned long sub_62a8(void)
{
  long v1;
  long v2; // stack - 0x10
  
  v1 = dat_24010;
  v2 = dat_24010;
  if (sub_5c8a()) {
    if (dat_24018 == 0x269e0)
      sub_6325(v1,0);
    v2 = dat_24018;
  }
  sub_17224("%s\n",v2);
  return 0;
}

// Function: sub_6325 @ 0x6325
void sub_6325(void *a0,int a1)
{
  void *v1;
  void *v2; // stack - 0x20
  
  v1 = dat_24010;
  v2 = dat_24010;
  if (a1)
    sub_1985b("OLDPWD",dat_24010,1);
  dat_243e4 += 1;
  if (dat_24018 != (void *)0x269e0) {
    if (v1 != dat_24018)
      free(dat_24018);
    dat_24018 = (void *)0x269e0;
  }
  if ((v1 != a0) && (a0))
    v2 = (void *)sub_103e1(a0);
  else {
    dat_24018 = (void *)sub_6250();
    if (!a0)
      v2 = dat_24018;
  }
  if ((v1 != v2) && (v1 != (void *)0x269e0))
    free(v1);
  dat_24010 = v2;
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
  sub_1985b("PWD",v2,1);
}

// Function: sub_6475 @ 0x6475
void sub_6475(void)
{
  sigsetmask(0);
}

// Function: sub_6486 @ 0x6486
void sub_6486(unsigned int a0)
{
  if (dat_2664c)
    _exit(dat_24400); // no-return
  dat_243e4 += 1;
  dat_243e0 = a0;
  longjmp(dat_243d8,1); // no-return
}

// Function: sub_64d8 @ 0x64d8
void sub_64d8(void)
{
  dat_243e8 = 0;
  sub_6475();
  if ((dat_266e4) || (!dat_26a43)) {
    signal(2,0);
    raise(2);
  }
  dat_24400 = 0x82;
  sub_6486(0); // no-return
}

// Function: sub_6531 @ 0x6531
void sub_6531(unsigned long a0,unsigned long a1) // ternary
{
  unsigned long v1;
  char *v2;
  char *v3; // stack - 0x20
  
  v1 = dat_240d0;
  if (dat_269f0) // branch-flip
    v2 = dat_269f0;
  else {
    v2 = "sh";
  }
  v3 = (dat_243f8) ? "%s: %d: %s: " : "%s: %d: "; // branch-flip
  sub_1714e(dat_240d0,v3,v2,dat_243ec,dat_243f8);
  sub_175c9(v1,a0,a1);
  sub_1705d(10,v1);
}

// Function: sub_65e8 @ 0x65e8
void sub_65e8(unsigned int a0,unsigned long a1,unsigned long a2)
{
  sub_6531(a1,a2);
  sub_170b1();
  sub_6486(a0); // no-return
}

// Function: sub_6621 @ 0x6621
void sub_6621(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,unsigned long a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
{
  char v1; // al
  unsigned long v10; // stack - 0xa8
  unsigned long v11; // stack - 0xa0
  unsigned long v12; // stack - 0x98
  unsigned long v13; // stack - 0x90
  unsigned long v14; // stack - 0x88
  unsigned long v15; // stack - 0x78
  unsigned long v16; // stack - 0x68
  unsigned long v17; // stack - 0x58
  unsigned long v18; // stack - 0x48
  unsigned long v19; // stack - 0x38
  char v2 [8];
  unsigned long v20; // stack - 0x28
  unsigned long v21; // stack - 0x18
  unsigned int v3; // stack - 0xd8
  long v4; // fs_offset
  unsigned int v5; // stack - 0xd4
  char *v6; // stack - 0xd0
  char *v7; // stack - 0xc8
  unsigned long v8; // stack - 0xc0
  unsigned long v9; // stack - 0xb0
  
  if (v1) {
    v14 = a0;
    v15 = a1;
    v16 = a2;
    v17 = a3;
    v18 = a4;
    v19 = a5;
    v20 = a6;
    v21 = a7;
  }
  v8 = *(unsigned long *)(v4 + 0x28);
  dat_24400 = 2;
  v3 = 8;
  v5 = 0x30;
  v6 = &Stack0000000000000008;
  v7 = v2;
  v9 = a9;
  v10 = a10;
  v11 = a11;
  v12 = a12;
  v13 = a13;
  sub_65e8(1,a8,&v3); // no-return
}

// Function: sub_66e2 @ 0x66e2
void sub_66e2(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,unsigned int a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
{
  char v1; // al
  unsigned long v10; // stack - 0xa0
  unsigned long v11; // stack - 0x98
  unsigned long v12; // stack - 0x90
  unsigned long v13; // stack - 0x88
  unsigned long v14; // stack - 0x78
  unsigned long v15; // stack - 0x68
  unsigned long v16; // stack - 0x58
  unsigned long v17; // stack - 0x48
  unsigned long v18; // stack - 0x38
  unsigned long v19; // stack - 0x28
  char v2 [16];
  unsigned long v20; // stack - 0x18
  unsigned int v3; // stack - 0xd8
  long v4; // fs_offset
  unsigned int v5; // stack - 0xd4
  char *v6; // stack - 0xd0
  char *v7; // stack - 0xc8
  unsigned long v8; // stack - 0xc0
  unsigned long v9; // stack - 0xa8
  
  if (v1) {
    v13 = a0;
    v14 = a1;
    v15 = a2;
    v16 = a3;
    v17 = a4;
    v18 = a5;
    v19 = a6;
    v20 = a7;
  }
  v8 = *(unsigned long *)(v4 + 0x28);
  v3 = 0x10;
  v5 = 0x30;
  v6 = &Stack0000000000000008;
  v7 = v2;
  v9 = a10;
  v10 = a11;
  v11 = a12;
  v12 = a13;
  sub_65e8(a8,a9,&v3); // no-return
}

// Function: sub_679b @ 0x679b
void sub_679b(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,unsigned long a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
{
  char v1; // al
  unsigned long v10; // stack - 0x98
  unsigned long v11; // stack - 0x90
  unsigned long v12; // stack - 0x88
  unsigned long v13; // stack - 0x78
  unsigned long v14; // stack - 0x68
  unsigned long v15; // stack - 0x58
  unsigned long v16; // stack - 0x48
  unsigned long v17; // stack - 0x38
  unsigned long v18; // stack - 0x28
  unsigned long v19; // stack - 0x18
  char v2 [8];
  unsigned int v3; // stack - 0xd8
  unsigned int v4; // stack - 0xd4
  char *v5; // stack - 0xd0
  char *v6; // stack - 0xc8
  unsigned long v7; // stack - 0xb0
  unsigned long v8; // stack - 0xa8
  unsigned long v9; // stack - 0xa0
  
  if (v1) {
    v12 = a0;
    v13 = a1;
    v14 = a2;
    v15 = a3;
    v16 = a4;
    v17 = a5;
    v18 = a6;
    v19 = a7;
  }
  v3 = 8;
  v4 = 0x30;
  v5 = &Stack0000000000000008;
  v6 = v2;
  v7 = a9;
  v8 = a10;
  v9 = a11;
  v10 = a12;
  v11 = a13;
  sub_6531(a8,&v3);
}

// Function: sub_686a @ 0x686a
char * sub_686a(int a0,unsigned int a1) // early-return x3
{
  if ((a0 != 2) && (a0 != 0x14))
    return strerror(a0);
  if (a1 & 1)
    return "No such file";
  if (a1 & 2)
    return "Directory nonexistent";
  return "not found";
}

// Function: sub_68c3 @ 0x68c3
bool sub_68c3(unsigned long a0)
{
  return *(char *)sub_155e7(a0) == '\0';
}

// Function: sub_68e8 @ 0x68e8
unsigned long sub_68e8(void) // return-dupe
{
  if ((dat_26a6c) && (!dat_26a68))
    return 1;
  return 0;
}

// Function: sub_690e @ 0x690e
void sub_690e(void)
{
  dat_24060 = dat_24070;
  dat_24084 = 0;
}

// Function: sub_692d @ 0x692d
unsigned int sub_692d(void)
{
  return dat_26cc8;
}

// Function: sub_6939 @ 0x6939
char * sub_6939(char a0,char *a1)
{
  char *v1; // stack - 0x18
  
  v1 = a1;
  if (a1 == dat_24050)
    v1 = (char *)sub_108a8();
  *v1 = a0;
  return &v1[1];
}

// Function: sub_6975 @ 0x6975
unsigned long sub_6975(int a0,long a1,unsigned int a2) // early-return
{
  long *v1;
  unsigned long v2; // rax
  long v3; // rax
  long v4; // stack - 0x20
  long v5; // stack - 0x18
  long *v6; // stack - 0x10
  
  if (2 <= a0) {
    v4 = *(long *)(a1 + 8);
    if (3 <= a0) {
      v5 = dat_24048;
      v6 = (long *)(a1 + 0x10);
      while( true ) {
        v2 = sub_1099d(v4,v5);
        v1 = &v6[1];
        v4 = *v6;
        if (!v4) break;
        v5 = sub_6939(0x20,v2);
        v6 = v1;
      }
      v3 = sub_6939(0,v2) - dat_24048;
      v4 = sub_10422(v3);
    }
    v2 = sub_6a59(v4,a2 & 2);
    return v2;
  }
  return 0;
}

// Function: sub_6a59 @ 0x6a59
unsigned int sub_6a59(unsigned long a0,unsigned int a1)
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  long v3; // rax
  char v4 [24];
  unsigned int v5; // stack - 0x38
  
  v2 = sub_11925(a0);
  sub_d25c(v2);
  sub_105e6(v4);
  v5 = 0;
  while (v3 = sub_129c3(0), v3 != 0x26a6c) {
    v1 = sub_6b48(v3,~(unsigned int)(sub_68e8() == 0) & a1);
    if (v3)
      v5 = v1;
    if (dat_243f0) break;
    sub_10641(v4);
  }
  sub_10641(v4);
  sub_d363();
  sub_1055e(v2);
  return v5;
}

// Function: sub_6b48 @ 0x6b48
unsigned int sub_6b48(int *a0,unsigned int a1)
{
  int v1;
  unsigned long v2;
  char v3 [24];
  int *v4; // stack - 0x50
  unsigned int v5; // stack - 0x3c
  unsigned int v6; // stack - 0x38
  void *v7; // stack - 0x30
  
  v5 = 0;
  v6 = 0;
  sub_105e6(v3);
  if ((dat_26a45) || (!a0)) goto label_6e08;
  sub_16b38();
  v4 = a0;
  switch(*a0) {
    case 0:
      v7 = sub_79d9;
      goto label_6cb1;
    case 1:
      v7 = sub_74ac;
      goto label_6cb1;
    case 2:
      dat_243ec = a0[1];
      dat_26e08 = dat_243ec;
      if (dat_24410)
        dat_26e08 = dat_243ec - (dat_24410 + -1);
      sub_73ba(*(unsigned long *)&a0[4]);
      sub_1646d(*(unsigned long *)&a0[4]);
      v6 = sub_1633b(*(unsigned long *)&a0[4],1);
      if (!v6)
        v6 = sub_6b48(*(unsigned long *)&a0[2],a1 & 2);
      if (*(long *)&a0[4])
        sub_16171(0);
      goto label_6df3;
    case 3:
    case 4:
      v7 = sub_7241;
label_6cb1:
      v5 = ~a1 & 2;
      break;
    case 5:
    case 6:
    case 7:
      v1 = *a0;
      v6 = sub_6b48(*(unsigned long *)&a0[2],(a1 | ((unsigned int)(v1 - 5U) >> 1) - 1) & 2);
      if ((v1 - 5U == (unsigned int)(v6 == 0)) || (dat_243f0)) goto label_6e08;
      v4 = *(int **)&a0[4];
      goto label_6d68;
    case 8:
      v6 = sub_6b48(*(unsigned long *)&a0[2],2);
      if (dat_243f0) goto label_6e08;
      if (v6) { // branch-flip
        if (!*(long *)&a0[6]) {
          v6 = 0;
          goto label_6df3;
        }
        v4 = *(int **)&a0[6];
      }
      else {
        v4 = *(int **)&a0[4];
      }
label_6d68:
      v7 = sub_6b48;
      break;
    case 9:
    case 10:
      v7 = sub_6ef1;
      break;
    case 0xb:
      v7 = sub_6fb2;
      break;
    case 0xc:
      v7 = sub_70ff;
      break;
    default:
      v2 = *(unsigned long *)&a0[2];
      v6 = (unsigned int)(sub_6b48(v2,2) == 0);
      goto label_6df3;
    case 0xe:
      sub_9a28(a0);
      goto label_6df3;
    
  }
  v6 = (*v7)(v4,a1);
label_6df3:
  dat_24400 = v6;
label_6e08:
  sub_16b38();
  if ((((!dat_26a40) || (!v5)) || (!v6)) && (!(a1 & 1))) {
    sub_10641(v3);
    return dat_24400;
  }
  sub_6486(3); // no-return
}

// Function: sub_6e63 @ 0x6e63
void sub_6e63(unsigned long a0,unsigned int a1)
{
  sub_6b48(a0,a1);
  abort(); // no-return
}

// Function: sub_6e8c @ 0x6e8c
int sub_6e8c(void)
{
  int v1; // stack - 0xc
  
  v1 = dat_243f0;
  if (((dat_243f0) && (0 <= dat_243f0)) && ((unsigned int)(dat_243f0 - 1U) <= 1)) {
    dat_2440c -= 1;
    if (1 <= dat_2440c) // branch-flip
      v1 = 1;
    else {
      dat_243f0 = 0;
    }
  }
  return v1;
}

// Function: sub_6ef1 @ 0x6ef1
unsigned int sub_6ef1(int *a0,unsigned int a1)
{
  unsigned long v1;
  unsigned int v2; // stack - 0x14
  unsigned int v3; // stack - 0x10
  unsigned int v4; // stack - 0xc
  
  dat_243f4 += 1;
  v3 = 0;
  do {
    v4 = sub_6b48(*(unsigned long *)&a0[2],2);
    v2 = sub_6e8c();
    if (v2 == 4)
      v3 = v4;
    if (!v2) {
      if (*a0 != 9)
        v4 = (unsigned int)(v4 == 0);
      if (v4) break;
      v1 = *(unsigned long *)&a0[4];
      v3 = sub_6b48(v1,a1 & 2);
      v2 = sub_6e8c();
    }
  } while (!(v2 & 0xfffffffd));
  dat_243f4 -= 1;
  return v3;
}

// Function: sub_6fb2 @ 0x6fb2
unsigned int sub_6fb2(long a0,unsigned int a1)
{
  unsigned long *v1; // stack - 0x28
  unsigned int v2; // stack - 0x3c
  long v3; // stack - 0x38
  unsigned long *v4; // stack - 0x30
  unsigned long *v5; // stack - 0x20
  
  dat_243ec = *(int *)(a0 + 4);
  dat_26e08 = dat_243ec;
  if (dat_24410)
    dat_26e08 = dat_243ec - (dat_24410 + -1);
  v5 = &v1;
  for (v3 = *(long *)(a0 + 8); v3; v3 = *(long *)(v3 + 8)) {
    sub_a021(v3,&v1,3);
  }
  *v5 = 0;
  v2 = 0;
  dat_243f4 += 1;
  for (v4 = v1; v4; v4 = (unsigned long *)*v4) {
    sub_1985b(*(unsigned long *)(a0 + 0x18),v4[1],0);
    v2 = sub_6b48(*(unsigned long *)(a0 + 0x10),a1 & 2);
    if (sub_6e8c() & 0xfffffffd) break;
  }
  dat_243f4 -= 1;
  return v2;
}

// Function: sub_70ff @ 0x70ff
unsigned int sub_70ff(long a0,unsigned int a1)
{
  unsigned long v1;
  long v2; // stack - 0x28
  long v3; // stack - 0x38
  long v4; // stack - 0x30
  long *v5; // stack - 0x20
  
  dat_243ec = *(int *)(a0 + 4);
  dat_26e08 = dat_243ec;
  if (dat_24410)
    dat_26e08 = dat_243ec - (dat_24410 + -1);
  v5 = &v2;
  sub_a021(*(unsigned long *)(a0 + 8),&v2,2);
  v3 = *(long *)(a0 + 0x10);
  do {
    if ((!v3) || (dat_243f0))
      return 0;
    for (v4 = *(long *)(v3 + 0x10); v4; v4 = *(long *)(v4 + 8)) {
      v1 = *(unsigned long *)(v2 + 8);
      if (sub_c70c(v4,v1)) {
        if (dat_243f0)
          return 0;
        if (!*(long *)(v3 + 0x18))
          return 0;
        v1 = *(unsigned long *)(v3 + 0x18);
        return sub_6b48(v1,a1);
      }
    }
    v3 = *(long *)(v3 + 8);
  } while( true );
}

// Function: sub_7241 @ 0x7241
unsigned int sub_7241(int *a0,unsigned int a1)
{
  unsigned long v1; // rax
  bool v2; // zf
  unsigned int v3; // stack - 0x24
  unsigned int v4; // stack - 0x18
  
  v2 = *a0 == 3;
  dat_243ec = a0[1];
  dat_26e08 = dat_243ec;
  if (dat_24410)
    dat_26e08 = dat_243ec - (dat_24410 + -1);
  sub_73ba(*(unsigned long *)&a0[4]);
  dat_243e4 += 1;
  if ((!v2) && (a1 & 1)) {
    if (!sub_692d()) {
      sub_1aa88();
      v3 = a1;
      goto label_731f;
    }
  }
  v1 = sub_e61f(a0,1);
  if (sub_eb5f(v1,a0,v2)) {
    v4 = 0;
    if (!v2)
      v4 = sub_ec74(v1);
    dat_243e4 -= 1;
    if ((!dat_243e4) && (dat_243e8))
      sub_64d8(); // no-return
    return v4;
  }
  v3 = a1 | 1;
  if (v2)
    v3 &= 0xfffffffd;
label_731f:
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
  sub_15ac5(*(unsigned long *)&a0[4],0);
  sub_6e63(*(unsigned long *)&a0[2],v3); // no-return
}

// Function: sub_73ba @ 0x73ba
void sub_73ba(int *a0)
{
  int v1;
  long v2; // stack - 0x28
  int *v3; // stack - 0x30
  long *v4; // stack - 0x20
  
  for (v3 = a0; v3; v3 = *(int **)&v3[2]) {
    v4 = &v2;
    v1 = *v3;
    if (0x15 <= v1) { // branch-flip
      if ((2 <= (unsigned int)(v1 - 0x15U)) || (!*(long *)&v3[6])) goto label_747e;
      sub_a021(*(unsigned long *)&v3[6],&v2,10);
      sub_13809(v3,*(unsigned long *)(v2 + 8),1);
    }
    else if (0x10 <= v1) {
      sub_a021(*(unsigned long *)&v3[6],&v2,10);
      *(unsigned long *)&v3[8] = *(unsigned long *)(v2 + 8);
    }
label_747e:
  }
}

// Function: sub_74ac @ 0x74ac
unsigned int sub_74ac(long a0,unsigned int a1)
{
  unsigned int v1;
  long v2;
  unsigned long v3; // rax
  int v4; // stack - 0x18
  int v5; // stack - 0x34
  int v6; // stack - 0x30
  unsigned int v7; // stack - 0x2c
  long *v8; // stack - 0x28
  int v9; // stack - 0x14
  
  v7 = 0;
  v5 = 0;
  for (v8 = *(long **)(a0 + 8); v8; v8 = (unsigned long *)*v8) {
    v5 += 1;
  }
  dat_243e4 += 1;
  v3 = sub_e61f(a0,v5);
  v6 = -1;
  v8 = *(long **)(a0 + 8);
  while( true ) {
    if (!v8) {
      if (!*(int *)(a0 + 4))
        v7 = sub_ec74(v3);
      dat_243e4 -= 1;
      if ((!dat_243e4) && (dat_243e8))
        sub_64d8(); // no-return
      return v7;
    }
    sub_856c(v8[1]);
    v9 = -1;
    if ((*v8) && (pipe(&v4) <= -1)) {
      close(v6);
      sub_6621("Pipe call failed"); // no-return
    }
    v1 = *(unsigned int *)(a0 + 4);
    v2 = v8[1];
    if (!sub_eb5f(v3,v2,v1)) break;
    if (0 <= v6)
      close(v6);
    v6 = v4;
    close(v9);
    v8 = (long *)*v8;
  }
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
  if (0 <= v9)
    close(v4);
  if (1 <= v6) {
    dup2(v6,0);
    close(v6);
  }
  if (2 <= v9) {
    dup2(v9,1);
    close(v9);
  }
  sub_6e63(v8[1],a1 | 1); // no-return
}

// Function: sub_76d6 @ 0x76d6
void sub_76d6(long a0,int *a1) // return-dupe
{
  unsigned long v1; // rax
  int v2; // stack - 0x18
  int v3; // stack - 0x14
  
  *a1 = -1;
  a1[2] = 0;
  a1[3] = 0;
  a1[4] = 0;
  a1[6] = 0;
  a1[7] = 0;
  if (!a0)
    return;
  if (pipe(&v2) <= -1)
    sub_6621("Pipe call failed"); // no-return
  v1 = sub_e61f(a0,1);
  if (sub_eb5f(v1,a0,2)) {
    close(v3);
    *a1 = v2;
    *(unsigned long *)&a1[6] = v1;
    return;
  }
  dat_243e4 = 0;
  if (dat_243e8)
    sub_64d8(); // no-return
  close(v2);
  if (v3 != 1) {
    dup2(v3,1);
    close(v3);
  }
  sub_baa8();
  sub_6e63(a0,1); // no-return
}

// Function: sub_7819 @ 0x7819
long sub_7819(long a0,long *a1)
{
  long *v1;
  long v2;
  
  v1 = *(long **)(a0 + 8);
  do {
    v2 = *a1;
    if (!v2) break;
    sub_a021(v2,a0,3);
    *a1 = *(long *)(v2 + 8);
  } while (!*v1);
  return *v1;
}

// Function: sub_788c @ 0x788c
unsigned long sub_788c(long *a0,unsigned long a1,unsigned long *a2) // return-dupe x3
{
  char *v1;
  char v2; // stack - 0x19
  long *v3; // stack - 0x18
  char *v4; // stack - 0x10
  
  v3 = (long *)*a0;
  while( true ) {
    if (*v3) // branch-flip
      v3 = (long *)*v3;
    else {
      v3 = (long *)sub_7819(a0,a1);
    }
    if (!v3)
      return 0;
    v1 = (char *)v3[1];
    if (*v1 != '-') break;
    v4 = &v1[2];
    v2 = v1[1];
    if (!v2) break;
    if ((v2 == '-') && (!*v4)) {
      if ((!*v3) && (!sub_7819(a0,a1)))
        return 0;
      v3 = (long *)*v3;
      break;
    }
    do {
      if (v2 != 'p')
        return 0;
      *a2 = "/usr/sbin:/usr/bin:/sbin:/bin";
      v1 = &v4[1];
      v2 = *v4;
      v4 = v1;
    } while (v2);
  }
  *a0 = (long)v3;
  return 4;
}

// Function: sub_79d9 @ 0x79d9
int sub_79d9(long a0,unsigned int a1)
{
  unsigned long *v1;
  long v10; // stack - 0xa8
  bool v11;
  int v12; // stack - 0xd4
  unsigned int v13; // stack - 0xd0
  unsigned int v14; // stack - 0xcc
  int v15; // stack - 0xc4
  unsigned int v16; // stack - 0xc0
  unsigned int v17; // stack - 0xbc
  unsigned int v18; // stack - 0xb8
  unsigned long *v19; // stack - 0xa0
  unsigned int v2; // eax
  unsigned long v20; // stack - 0x98
  long v21; // stack - 0x90
  unsigned long *v22; // stack - 0x88
  unsigned long v23; // stack - 0x80
  long v24; // stack - 0x78
  unsigned long v25; // stack - 0x70
  unsigned long *v26; // stack - 0x68
  unsigned long v27; // stack - 0x60
  unsigned long v28; // stack - 0x58
  long *v29; // stack - 0x50
  int v3; // eax
  unsigned long *v30; // stack - 0x40
  long *v31; // stack - 0x30
  unsigned long v32; // stack - 0x20
  long v4; // rax
  unsigned long v5;
  long v6; // stack - 0x38
  unsigned long *v7; // stack - 0x48
  long v8; // stack - 0xb0
  int v9 [2]; // stack - 0x28
  
  dat_243ec = *(int *)(a0 + 4);
  dat_26e08 = dat_243ec;
  if (dat_24410)
    dat_26e08 = dat_243ec - (dat_24410 + -1);
  v23 = dat_24028;
  dat_24404 = 0;
  v9[0] = 2;
  v32 = 0x23750;
  v31 = &v6;
  v30 = &v7;
  v14 = 0;
  v11 = 0;
  v13 = 0xffffffff;
  v16 = 0;
  v17 = 0;
  v10 = 0;
  v12 = 0;
  v8 = *(long *)(a0 + 0x10);
  v24 = sub_7819(&v7,&v8);
  if (v24) {
    v18 = 0;
    do {
      sub_8f3b(v7[1],v9,v14 | 0x10,dat_24190 + 5);
      v17 += 1;
      if (v9[0] != 2) break;
      v18 = *(unsigned int *)(v32 + 0x10) & 4;
      if ((int)v13 < 0) {
        v13 = *(unsigned int *)(v32 + 0x10) & 1;
        v17 = v13 ^ 1;
      }
      v11 = v32 == 0x238a0;
      if (v32 != 0x23840) break;
      v14 = sub_788c(&v7,&v8,&v10);
    } while (v14);
    for (; v8; v8 = *(long *)(v8 + 8)) {
      if ((v18) && (v5 = *(unsigned long *)(v8 + 0x10), sub_1295d(v5)))
        v5 = 4;
      else {
        v5 = 3;
      }
      sub_a021(v8,&v7,v5);
    }
    for (v19 = v7; v19; v19 = (unsigned long *)*v19) {
      v12 += 1;
    }
    if ((v11) && (2 <= v12))
      v16 = 1;
  }
  v25 = sub_1a539(v17);
  v26 = (unsigned long *)(sub_10422((long)(v12 + 2) << 3) + 8);
  v19 = v7;
  v22 = v26;
  while (v19) {
    v1 = &v22[1];
    *v22 = v19[1];
    v19 = (unsigned long *)*v19;
    v22 = v1;
  }
  *v22 = 0;
  v21 = 0;
  if (((dat_26a43) && (!dat_24410)) && (1 <= v12))
    v21 = v22[-1];
  dat_26dc0 = 2;
  sub_73ba(*(unsigned long *)(a0 + 0x18));
  v5 = *(unsigned long *)(a0 + 0x18);
  v27 = sub_1646d(v5);
  v15 = sub_1633b(*(unsigned long *)(a0 + 0x18),3);
  if (v15) { // branch-flip
label_7dd2:
    dat_24400 = v15;
    if (1 <= (int)v13)
      sub_6486(1); // no-return
  }
  else {
    for (v8 = *(long *)(a0 + 8); v8; v8 = *(long *)(v8 + 8)) {
      v29 = v31;
      sub_a021(v8,&v6,4);
      if (v17) // branch-flip
        sub_1a1e6(*(unsigned long *)(*v29 + 8),1);
      else {
        sub_19b05(*(unsigned long *)(*v29 + 8),v16);
      }
    }
    if ((dat_26a47) && (!dat_24408)) {
      v28 = 0x26da0;
      dat_24408 = 1;
      v4 = dat_241f0 + 4;
      sub_1701f(sub_15816(v4),v28);
      dat_24408 = 0;
      v2 = sub_8796(v28,v6,0);
      sub_8796(v28,v24,v2);
      sub_1705d(10,v28);
    }
    if ((v9[0] != 2) || (!(*(unsigned int *)(v32 + 0x10) & 2))) {
      if (!v10)
        v10 = dat_24190 + 5;
      sub_8f3b(*v26,v9,v14 | 1,v10);
    }
    v20 = 0;
    switch(v9[0]) { // branch-flip
      default:
        if ((a1 & 1) && (!sub_692d()))
          sub_8884(v26,v10,v32 & 0xffffffff); // no-return
        dat_243e4 += 1;
        v20 = sub_ebb7(a0,v26,v10,v32 & 0xffffffff);
        break;
      case 1:
        v3 = sub_831b(v32,v12,v26,a1);
        if (v3)
          longjmp(dat_243d8,1); // return-dupe, no-return
        break;
      case 2:
        v3 = sub_8179(v32,v12,v26,a1);
        if ((v3) && ((dat_243e0 != 1 || (1 <= (int)v13)))) {
          longjmp(dat_243d8,1);
        }
        break;
      case -1:
        v15 = 0x7f;
        goto label_7dd2;
      
    }
    v15 = sub_ec74(v20);
    dat_243e4 = 0;
    if (dat_243e8)
      sub_64d8(); // no-return
  }
  if (*(long *)(a0 + 0x18))
    sub_16171(v11);
  sub_16440(v27);
  sub_d44e(v23);
  sub_1a5c7(v25);
  if (v21)
    sub_1985b("_",v21,0);
  return v15;
}

// Function: sub_8179 @ 0x8179
int sub_8179(long a0,unsigned int a1,unsigned long *a2,unsigned int a3)
{
  void *v1;
  char *v2;
  unsigned long v3;
  int v4; // eax
  char v5 [200];
  unsigned int v6; // stack - 0xf0
  
  v3 = dat_243f8;
  v2 = dat_243d8;
  v4 = _setjmp(v5);
  if (!v4) {
    dat_243d8 = v5;
    dat_243f8 = *a2;
    dat_26a18 = &a2[1];
    dat_26a28 = 0;
    if (a0 != 0x23888) { // branch-flip
      v1 = *(void **)(a0 + 8);
      v6 = (*v1)(a1,a2);
    }
    else {
      v6 = sub_6975(a1,a2,a3);
    }
    sub_170b1();
    if (*(int *)(dat_240c8 + 0x24))
      sub_679b("%s: I/O error",dat_243f8);
    dat_24400 = v6 | *(unsigned int *)(dat_240c8 + 0x24);
  }
  sub_690e();
  dat_243f8 = v3;
  dat_243d8 = v2;
  return v4;
}

// Function: sub_831b @ 0x831b
int sub_831b(int *a0,int a1,long a2,unsigned int a3)
{
  char *v1;
  unsigned int v2;
  unsigned int v3;
  unsigned long v4;
  long v5;
  unsigned long v6;
  int v7; // eax
  char v8 [200];
  
  v6 = dat_26a10;
  v5 = dat_26a08;
  v4 = dat_26a00;
  v3 = dat_24410;
  v2 = dat_243f4;
  v1 = dat_243d8;
  v7 = _setjmp(v8);
  if (!v7) {
    dat_243d8 = v8;
    dat_26a00 = (undefined5)dat_26a00;
    *a0 = *a0 + 1;
    dat_24410 = a0[3];
    dat_243f4 = 0;
    if ((!dat_243e4) && (dat_243e8))
      sub_64d8(); // no-return
    dat_26a00 = CONCAT44(dat_26a04,a1 + -1);
    dat_26a08 = a2 + 8;
    dat_26a10 = 0xffffffff00000001;
    sub_6b48(*(unsigned long *)&a0[6],a3 & 2);
  }
  dat_243e4 += 1;
  dat_243f4 = v2;
  dat_24410 = v3;
  sub_1b4c3(a0);
  sub_120da(0x26a00);
  dat_26a00 = v4;
  dat_26a08 = v5;
  dat_26a10 = v6;
  dat_243d8 = v1;
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
  dat_243f0 &= 0xfffffff3;
  return v7;
}

// Function: sub_856c @ 0x856c
void sub_856c(int *a0)
{
  unsigned long v1;
  char v2 [24];
  
  if (((!*a0) && (*(long *)&a0[4])) && (v1 = *(unsigned long *)(*(long *)&a0[4] + 0x10), sub_68c3(v1)))
    sub_8f3b(*(unsigned long *)(*(long *)&a0[4] + 0x10),v2,0,dat_24190 + 5);
}

// Function: sub_8600 @ 0x8600
unsigned int sub_8600(void)
{
  return dat_24404;
}

// Function: sub_8617 @ 0x8617
unsigned long sub_8617(int a0,unsigned long *a1) // return-dupe
{
  int v1; // stack - 0xc
  
  if (2 <= a0) // branch-flip
    v1 = sub_1174e(a1[1]);
  else {
    v1 = 1;
  }
  if (v1 <= 0)
    sub_11621(a1[1]); // no-return
  if (dat_243f4 < v1)
    v1 = dat_243f4;
  if (1 <= v1) {
    if (*(char *)*a1 == 'c') {
      dat_243f0 = 2;
      dat_2440c = v1;
      return 0;
    }
    dat_243f0 = 1;
    dat_2440c = v1;
    return 0;
  }
  return 0;
}

// Function: sub_86b0 @ 0x86b0
unsigned int sub_86b0(unsigned long a0,long a1)
{
  unsigned long v1;
  unsigned int v2; // stack - 0x10
  unsigned int v3; // stack - 0xc
  
  if (*(long *)(a1 + 8)) { // branch-flip
    v2 = 4;
    v1 = *(unsigned long *)(a1 + 8);
    v3 = sub_1174e(v1);
  }
  else {
    v2 = 8;
    v3 = dat_24400;
  }
  dat_243f0 = v2;
  return v3;
}

// Function: sub_8710 @ 0x8710
unsigned long sub_8710(void)
{
  return 1;
}

// Function: sub_8726 @ 0x8726
unsigned long sub_8726(void)
{
  return 0;
}

// Function: sub_873c @ 0x873c
unsigned long sub_873c(int a0,long a1)
{
  if (2 <= a0) {
    dat_26a43 = 0;
    dat_26a44 = 0;
    sub_11be5();
    sub_8884(a1 + 8,dat_24190 + 5,0); // no-return
  }
  return 0;
}

// Function: sub_8796 @ 0x8796
unsigned int sub_8796(unsigned long a0,unsigned long *a1,unsigned int a2)
{
  int v1; // eax
  unsigned int v2; // stack - 0x2c
  unsigned long *v3; // stack - 0x28
  
  v2 = a2;
  for (v3 = a1; v3; v3 = (unsigned long *)*v3) {
    v1 = 1 - v2;
    v2 |= 1;
    sub_1714e(a0,&" %s"[v1],v3[1]);
  }
  return v2;
}

// Function: sub_8803 @ 0x8803
void sub_8803(unsigned long a0,unsigned long a1)
{
  sub_8b10(a0,a1,1);
}

// Function: sub_882d @ 0x882d
void sub_882d(unsigned int a0,long *a1) // return-dupe
{
  if (*a1 == a1[1]) {
    sub_1705d(a0,a1);
    return;
  }
  *(char *)*a1 = (char)a0;
  *a1 = *a1 + 1;
}

// Function: sub_8884 @ 0x8884
void sub_8884(unsigned long *a0,unsigned long a1,int a2)
{
  char *v1;
  unsigned long v2;
  unsigned long v3; // stack - 0x38
  int v4; // stack - 0x3c
  unsigned long *v5; // stack - 0x30
  unsigned int v6; // stack - 0x20
  unsigned int v7; // stack - 0x1c
  unsigned long v8; // stack - 0x18
  unsigned long v9; // stack - 0x10
  
  v3 = a1;
  v5 = a0;
  v8 = sub_19e1e(1,0x20,0);
  v1 = (char *)*v5;
  if (strchr(v1,0x2f)) { // branch-flip
    sub_89f8(*v5,v5,v8);
    v6 = *dat_266e8;
  }
  else {
    v6 = 2;
    v4 = a2;
    while( true ) {
      v2 = *v5;
      if ((int)sub_8803(&v3,v2) <= -1) break;
      v9 = dat_24048;
      v4 -= 1;
      if ((v4 <= -1) && (!dat_24420)) {
        sub_89f8(dat_24048,v5,v8);
        if ((*dat_266e8 != 2) && (*dat_266e8 != 0x14))
          v6 = *dat_266e8;
      }
    }
  }
  if ((0x29 <= v6) || (!(0x11000100004U >> ((unsigned char)v6 & 0x3f) & 1)))
    v7 = 0x7e;
  else {
    v7 = 0x7f;
  }
  dat_24400 = v7;
  v2 = sub_686a(v6,4);
  sub_66e2(3,"%s: %s",*v5,v2); // no-return
}

// Function: sub_89f8 @ 0x89f8
void sub_89f8(char *a0,char **a1,char **a2)
{
  char **v1;
  char **v2; // stack - 0x28
  char *v3; // stack - 0x20
  
  v2 = a1;
  v3 = a0;
  while( true ) {
    execve(v3,v2,a2);
    if ((v3 == "/bin/sh") || (*dat_266e8 != 8)) break;
    v1 = &v2[-1];
    *v2 = v3;
    v3 = "/bin/sh";
    *v1 = "/bin/sh";
    v2 = v1;
  }
}

// Function: sub_8a75 @ 0x8a75
char * sub_8a75(char *a0,char *a1,int a2)
{
  char *v1; // rax
  char *v2; // stack - 0x10
  
  if (a2) { // branch-flip
    if (a2 != 1) { // branch-flip
      v1 = (unsigned long)strcspn(a0,a1);
      v2 = &a0[(long)v1];
    }
    else {
      v1 = (char *)sub_115d5(a0,"builtin");
      if (!v1)
        v1 = (char *)sub_115d5(a0,"func");
      v2 = v1;
    }
  }
  else {
    v2 = NULL;
  }
  if ((v2) && (*v2 == '%'))
    v2 = &v2[1];
  v1 = v2;
  return v1;
}

// Function: sub_8b10 @ 0x8b10
long sub_8b10(long *a0,char *a1,unsigned int a2) // early-return
{
  char *v1; // rax
  long v2; // rax
  char *v3; // rax
  char *v4; // stack - 0x48
  char *v5; // stack - 0x40
  char *v6; // stack - 0x38
  char *v7; // stack - 0x30
  char *v8; // stack - 0x28
  unsigned long v9; // stack - 0x20
  
  v4 = "%:";
  if (!*a0)
    return 0xffffffff;
  v5 = NULL;
  v8 = (char *)*a0;
  if ((*v8 == '%') && (v1 = (char *)sub_8a75(&v8[1],"%:",a2), v1)) {
    v5 = &v8[1];
    v8 = v1;
    v4 = ":";
  }
  v9 = strcspn(v8,v4);
  v6 = &v8[v9];
  if (*v6 == '%') {
    v2 = strchrnul(v6,0x3a) - (long)v6;
    if (sub_8a75(&v6[1],v4,a2)) // branch-flip
      v5 = &v6[1];
    else {
      v9 += v2;
    }
    v6 = &v6[v2];
  }
  dat_24420 = v5;
  if (*v6 != ':') // branch-flip
    v6 = NULL;
  else {
    v6 = &v6[1];
  }
  *a0 = (long)v6;
  v2 = strlen(a1) + v9 + 2;
  v7 = (char *)sub_108d9(v2);
  if (v9) {
    v3 = mempcpy(v7,v8,v9);
    v7 = &v3[1];
    *v3 = 0x2f;
  }
  strcpy(v7,a1);
  return v2;
}

// Function: sub_8cdb @ 0x8cdb
unsigned int sub_8cdb(void) // early-return x2
{
  long v1;
  long v2; // rax
  int v3 [6]; // stack - 0x28
  unsigned int v4; // stack - 0x44
  unsigned long *v5; // stack - 0x40
  unsigned long *v6; // stack - 0x38
  
  if (sub_12769("r")) {
    sub_96f7();
    return 0;
  }
  if (!*dat_26a18) {
    for (v5 = (unsigned long *)0x24440; v5 < (unsigned long *)0x24538; v5 = &v5[1]) {
      for (v6 = (unsigned long *)*v5; v6; v6 = (unsigned long *)*v6) {
        if (!*(short *)&v6[2])
          sub_8e73(v6);
      }
    }
    return 0;
  }
  v4 = 0;
  for (; v1 = *dat_26a18, v1; dat_26a18 = &dat_26a18[1]) {
    v2 = sub_97d7(v1,0);
    if ((v2) && ((!*(short *)(v2 + 0x10) || (((*(short *)(v2 + 0x10) == 2 && (!(*(unsigned int *)(*(long *)(v2 + 8) + 0x10) & 2))) && (1 <= dat_24024))))))
      sub_9927();
    sub_8f3b(v1,v3,1,dat_24190 + 5);
    if (v3[0] == -1)
      v4 = 1;
  }
  return v4;
}

// Function: sub_8e73 @ 0x8e73
void sub_8e73(long a0) // ternary
{
  char *v1; // rax
  long v2; // stack - 0x20
  int v3; // stack - 0x24
  unsigned long v4; // stack - 0x18
  
  v3 = *(int *)(a0 + 8);
  v2 = dat_24190 + 5;
  do {
    sub_8803(&v2,a0 + 0x13);
    v3 -= 1;
  } while (0 <= v3);
  v4 = dat_24048;
  sub_1701f(dat_24048,dat_240c8);
  v1 = (*(char *)(a0 + 0x12)) ? "*" : (char *)0x269e0; // branch-flip
  sub_17224("%s\n",v1);
}

// Function: sub_8f3b @ 0x8f3b
void sub_8f3b(char *a0,int *a1,unsigned int a2,long a3) // ternary
{
  int v1; // eax
  int v10; // stack - 0xd8
  unsigned int v11; // stack - 0xd4
  unsigned int v12; // stack - 0xd0
  int v13; // stack - 0xcc
  long v14; // stack - 0xc8
  long v15; // stack - 0xc0
  char *v16; // stack - 0xb8
  char *v17; // stack - 0xb0
  unsigned int v18; // stack - 0x90
  unsigned long v2; // rax
  char v3 [24];
  long v4; // stack - 0x108
  unsigned int v5; // stack - 0xfc
  int *v6; // stack - 0xf8
  char *v7; // stack - 0xf0
  int v8; // stack - 0xe0
  int v9; // stack - 0xdc
  
  v4 = a3;
  v5 = a2;
  v6 = a1;
  v7 = a0;
  if (strchr(a0,0x2f)) {
    v6[2] = -1;
    if ((v5 & 2) && ((int)stat64(v7,v3) <= -1)) {
      *v6 = -1;
      return;
    }
    *v6 = 0;
    return;
  }
  v11 = (unsigned int)(dat_24190 + 5 == v4);
  if (!v11)
    v5 |= 8;
  v14 = sub_97d7(v7,0);
  if (v14) {
    if (*(short *)(v14 + 0x10) != 1) { // branch-flip
      if (*(short *)(v14 + 0x10) != 2) // branch-flip
        v12 = 0x18;
      else {
        v12 = (*(unsigned int *)(*(long *)(v14 + 8) + 0x10) & 2) ? 0 : 0x10; // branch-flip
      }
    }
    else {
      v12 = 4;
    }
    if (v5 & v12) { // branch-flip
      if (v5 & v12 & 0x10) goto label_9493;
      v11 = 0;
      v14 = 0;
    }
    else if (!*(char *)(v14 + 0x12)) goto label_954e;
  }
  v15 = sub_959c(v7);
  if ((!v15) || (1 <= dat_24024 && (!(*(unsigned int *)(v15 + 0x10) & 2) && !(v5 & 8)))) {
    if (v5 & 0x10) {
label_9493:
      *v6 = -1;
      return;
    }
    v9 = -1;
    if ((v14) && (*(char *)(v14 + 0x12))) {
      if (*(short *)(v14 + 0x10) != 2) // branch-flip
        v9 = *(int *)(v14 + 8);
      else {
        v9 = dat_24024;
      }
    }
    v10 = 2;
    v8 = -1;
label_9406:
    do {
      while( true ) {
        v13 = sub_8803(&v4,v7);
        if (v13 <= -1) {
          if ((v14) && (v11))
            sub_9927();
          if (v5 & 1) {
            v2 = sub_686a(v10,4);
            sub_679b("%s: %s",v7,v2);
          }
          goto label_9493;
        }
        v16 = dat_24420;
        v17 = dat_24048;
        v8 += 1;
        if (dat_24420) break;
label_921e:
        if ((*dat_24048 != '/') || (v9 < v8)) {
          v1 = stat64(dat_24048,v3);
          if (0 <= v1) { // branch-flip
            v10 = 0xd;
            if ((v18 & 0xf000) == 0x8000) {
              if (v16) { // branch-flip
                sub_10422((long)v13);
                sub_10109(v17);
                v14 = sub_97d7(v7,0);
                if ((!v14) || (*(short *)(v14 + 0x10) != 1))
                  sub_6621("%s not defined in %s",v7,v17); // no-return
                sub_1055e(v17);
              }
              else {
                if (!v11) {
                  *v6 = 0;
                  v6[2] = v8;
                  return;
                }
                dat_243e4 += 1;
                v14 = sub_97d7(v7,1);
                *(unsigned short *)(v14 + 0x10) = 0;
                *(int *)(v14 + 8) = v8;
                dat_243e4 -= 1;
                if ((!dat_243e4) && (dat_243e8))
                  sub_64d8(); // no-return
              }
              goto label_954e;
            }
          }
          else if ((*dat_266e8 != 2) && (*dat_266e8 != 0x14))
            v10 = *dat_266e8;
        }
        else if (v9 <= v8) goto label_954e;
      }
      if (*dat_24420 != 'b') {
        if (!(v5 & 4)) goto label_921e;
        goto label_9406;
      }
    } while (!v15);
  }
  if (!v11) {
    *v6 = 2;
    *(long *)&v6[2] = v15;
    return;
  }
  dat_243e4 += 1;
  v14 = sub_97d7(v7,1);
  *(unsigned short *)(v14 + 0x10) = 2;
  *(long *)(v14 + 8) = v15;
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
label_954e:
  *(char *)(v14 + 0x12) = 0;
  *v6 = (int)*(short *)(v14 + 0x10);
  *(unsigned long *)&v6[2] = *(unsigned long *)(v14 + 8);
  return;
}

// Function: sub_959c @ 0x959c
unsigned long sub_959c(unsigned long a0)
{
  unsigned long v1 [3]; // stack - 0x20
  
  v1[0] = a0;
  return bsearch(v1,0x23780,0x27,0x18,sub_1196d);
}

// Function: sub_95dd @ 0x95dd
void sub_95dd(void)
{
  unsigned long *v1; // stack - 0x18
  unsigned long *v2; // stack - 0x10
  
  for (v1 = (unsigned long *)0x24440; v1 < (unsigned long *)0x24538; v1 = &v1[1]) {
    for (v2 = (unsigned long *)*v1; v2; v2 = (unsigned long *)*v2) {
      if ((!*(short *)&v2[2]) || (((*(short *)&v2[2] == 2 && (!(*(unsigned int *)(v2[1] + 0x10) & 2))) && (1 <= dat_24024))))
        *(char *)((long)v2 + 0x12) = 1;
    }
  }
}

// Function: sub_9666 @ 0x9666
void sub_9666(char *a0)
{
  int v1; // stack - 0x18
  int v2; // stack - 0x14
  char *v3; // stack - 0x10
  
  v1 = 0;
  v2 = -1;
  v3 = a0;
  do {
    if (*v3 == '%') {
      if (sub_115d5(&v3[1],"builtin")) {
        v2 = v1;
        dat_24024 = v2; // return-dupe
        sub_96f7();
        return;
      }
    }
    v3 = strchr(v3,0x3a);
    if (!v3) {
      dat_24024 = v2;
      sub_96f7();
      return;
    }
    v1 += 1;
    v3 = &v3[1];
  } while( true );
}

// Function: sub_96f7 @ 0x96f7
void sub_96f7(void)
{
  unsigned long *v1;
  unsigned long *v2; // stack - 0x20
  unsigned long *v3; // stack - 0x18
  
  dat_243e4 += 1;
  for (v2 = (unsigned long *)0x24440; v2 < (unsigned long *)0x24538; v2 = &v2[1]) {
    v3 = v2;
    while (v1 = (unsigned long *)*v3, v1) {
      if ((*(short *)&v1[2]) && (((*(short *)&v1[2] != 2 || (*(unsigned int *)(v1[1] + 0x10) & 2)) || (dat_24024 <= 0))))
        v3 = v1;
      else {
        *v3 = *v1;
        free(v1);
      }
    }
  }
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
}

// Function: sub_97d7 @ 0x97d7
unsigned long * sub_97d7(unsigned char *a0,int a1)
{
  unsigned long v1; // rax
  unsigned int v2; // stack - 0x24
  unsigned char *v3; // stack - 0x20
  unsigned long *v4; // stack - 0x18
  unsigned long *v5; // stack - 0x10
  
  v2 = (unsigned int)*a0 << 4;
  for (v3 = a0; *v3; v3 = &v3[1]) {
    v2 += *v3;
  }
  v5 = (unsigned long *)((unsigned long)((v2 & 0x7fff) % 0x1f) * 8 + 0x24440);
  v4 = (unsigned long *)*v5;
  while ((v4 && (strcmp((char *)((long)v4 + 0x13),(char *)a0)))) {
    v5 = v4;
    v4 = (unsigned long *)*v4;
  }
  if ((a1) && (!v4)) {
    v1 = strlen((char *)a0);
    *v5 = sub_10354(v1 + 0x18);
    v4 = (unsigned long *)*v5;
    *v4 = 0;
    *(unsigned short *)&v4[2] = 0xffff;
    strcpy((char *)((long)v4 + 0x13),(char *)a0);
  }
  dat_24428 = v5;
  return v4;
}

// Function: sub_9927 @ 0x9927
void sub_9927(void)
{
  unsigned long *v1;
  
  dat_243e4 += 1;
  v1 = (unsigned long *)*dat_24428;
  *dat_24428 = *v1;
  if (*(short *)&v1[2] == 1)
    sub_1b4c3(v1[1]);
  free(v1);
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
}

// Function: sub_99b6 @ 0x99b6
void sub_99b6(unsigned long a0,unsigned int *a1)
{
  long v1; // rax
  
  v1 = sub_97d7(a0,1);
  if (*(short *)(v1 + 0x10) == 1)
    sub_1b4c3(*(unsigned long *)(v1 + 8));
  *(short *)(v1 + 0x10) = (short)*a1;
  *(unsigned long *)(v1 + 8) = *(unsigned long *)&a1[2];
  *(char *)(v1 + 0x12) = 0;
}

// Function: sub_9a28 @ 0x9a28
void sub_9a28(long a0)
{
  unsigned int v1 [2]; // stack - 0x28
  unsigned long v2; // stack - 0x20
  
  dat_243e4 += 1;
  v1[0] = 1;
  v2 = sub_1abd6(a0);
  sub_99b6(*(unsigned long *)(a0 + 8),v1);
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
}

// Function: sub_9ac3 @ 0x9ac3
void sub_9ac3(unsigned long a0)
{
  long v1; // rax
  
  v1 = sub_97d7(a0,0);
  if ((v1) && (*(short *)(v1 + 0x10) == 1))
    sub_9927();
}

// Function: sub_9b05 @ 0x9b05
unsigned int sub_9b05(int a0,long a1)
{
  unsigned int v1; // eax
  int v2; // stack - 0x10
  unsigned int v3; // stack - 0xc
  
  v3 = 0;
  for (v2 = 1; v2 < a0; v2 = v2 + 1) {
    v1 = sub_9b6c(dat_240c8,*(unsigned long *)(a1 + (long)v2 * 8),0,1);
    v3 |= v1;
  }
  return v3;
}

// Function: sub_9b6c @ 0x9b6c
char * sub_9b6c(unsigned long a0,char *a1,long a2,int a3) // return-dupe, ternary x2
{
  unsigned int v1; // eax
  long v10; // stack - 0x30
  long v11; // stack - 0x20
  char *v2; // rax
  int v3 [2]; // stack - 0x28
  long v4; // stack - 0x60
  char *v5; // stack - 0x58
  unsigned long v6; // stack - 0x50
  int v7; // stack - 0x44
  long v8; // stack - 0x40
  char *v9; // stack - 0x38
  
  v4 = a2;
  v5 = a1;
  v6 = a0;
  if (a3)
    sub_1701f(a1,a0);
  v2 = (long)sub_15a3c(v5);
  if (v2) { // branch-flip
    if (a3) // branch-flip
      v2 = " is a shell keyword";
    else {
      v2 = v5;
    }
    sub_1701f(v2,v6);
  }
  else {
    v2 = (long)sub_4dbc(v5,0);
    v10 = (long)v2;
    if (v2) { // branch-flip
      if (!a3) {
        sub_1701f("alias ",v6);
        sub_5074(v10);
        v2 = NULL;
        return v2;
      }
      sub_1714e(v6," is an alias for %s",*(unsigned long *)((long)v2 + 0x10));
    }
    else {
      if (v4) // branch-flip
        v8 = 0;
      else {
        v2 = (long)(dat_24190 + 5);
        v4 = (long)v2;
        v2 = (long)sub_97d7(v5,0);
        v8 = (long)v2;
      }
      if (v8) { // branch-flip
        v1 = (int)*(short *)(v8 + 0x10);
        v11 = *(long *)(v8 + 8);
        v3[0] = v1;
      }
      else {
        sub_8f3b(v5,v3,2,v4);
      }
      if (v3[0] != 2) { // branch-flip
        if (2 < v3[0]) {
label_9e25:
          if (!a3) {
            v2 = (char *)0x7f;
            return v2;
          }
          sub_1701f(": not found\n",v6);
          v2 = (char *)0x7f;
          return v2;
        }
        if (v3[0]) { // branch-flip
          if (v3[0] != 1) goto label_9e25;
          if (a3) // branch-flip
            sub_1701f(" is a shell function",v6);
          else {
            sub_1701f(v5,v6);
          }
        }
        else {
          v7 = (int)v11;
          if ((int)v11 != -1) { // branch-flip
            do {
              sub_8803(&v4,v5);
              v7 -= 1;
            } while (0 <= v7);
            v9 = dat_24048;
          }
          else {
            v9 = v5;
          }
          if (a3) { // branch-flip
            v2 = (v8) ? " a tracked alias for" : (char *)0x269e0; // branch-flip
            sub_1714e(v6," is%s %s",v2,v9);
          }
          else {
            sub_1701f(v9,v6);
          }
        }
      }
      else if (a3) { // branch-flip
        v1 = *(unsigned int *)(v11 + 0x10) & 1;
        v2 = (v1) ? "special " : (char *)0x269e0; // branch-flip
        sub_1714e(v6," is a %sshell builtin",v2);
      }
      else {
        sub_1701f(v5,v6);
      }
    }
  }
  sub_882d(10,v6);
  v2 = NULL;
  return v2;
}

// Function: sub_9e74 @ 0x9e74
unsigned long sub_9e74(void) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned int v3; // stack - 0x20
  char *v4; // stack - 0x18
  
  v3 = 0;
  v4 = NULL;
  while (v1 = sub_12769("pvV"), v1) {
    if (v1 != 0x56) { // branch-flip
      if (v1 != 0x76) // branch-flip
        v4 = "/usr/sbin:/usr/bin:/sbin:/bin";
      else {
        v3 |= 1;
      }
    }
    else {
      v3 |= 2;
    }
  }
  if ((v3) && (*dat_26a18)) {
    v2 = sub_9b6c(dat_240c8,*dat_26a18,v4,v3 - 1);
    return v2;
  }
  return 0;
}

// Function: sub_9f16 @ 0x9f16
int sub_9f16(int a0)
{
  return (int)(dat_1c618 + (double)(a0 * 8 + -1) * dat_1c610);
}

// Function: sub_9f4c @ 0x9f4c
char * sub_9f4c(char a0,char *a1)
{
  char *v1; // stack - 0x18
  
  v1 = a1;
  if (a1 == dat_24050)
    v1 = (char *)sub_108a8();
  *v1 = a0;
  return &v1[1];
}

// Function: sub_9f88 @ 0x9f88
void sub_9f88(unsigned long a0,unsigned int a1)
{
  sub_c50c(a0,a1 | 2);
}

// Function: sub_9fae @ 0x9fae
long sub_9fae(char *a0,char *a1)
{
  char *v1; // stack - 0x28
  long v2; // stack - 0x10
  
  v2 = 0;
  v1 = a1;
  while ((a0 < v1 && (v1 = &v1[-1], *v1 == '\x81'))) {
    v2 += 1;
  }
  return v2;
}

// Function: sub_9fed @ 0x9fed
unsigned long sub_9fed(char *a0) // early-return
{
  void *v1; // rax
  
  v1 = getpwnam(a0);
  if (v1)
    return *(unsigned long *)((long)v1 + 0x20);
  return 0;
}

// Function: sub_a021 @ 0xa021
void sub_a021(long a0,long a1,unsigned int a2)
{
  unsigned long v1; // rax
  unsigned long *v2; // rax
  
  dat_24548 = *(unsigned long *)(a0 + 0x18);
  dat_24540 = dat_24048;
  sub_a16b(*(unsigned long *)(a0 + 0x10),a2);
  if (a1) {
    v1 = sub_10422(dat_24540 - dat_24048);
    dat_24578 = (unsigned long *)0x24570;
    if (a2 & 1) { // branch-flip
      sub_b7cc(v1,0xffffffff,0x24570);
      *dat_24578 = 0;
      dat_24578 = (unsigned long *)0x24570;
      sub_bb3d(dat_24570);
    }
    else {
      v2 = (unsigned long *)sub_10422(0x10);
      v2[1] = v1;
      *dat_24578 = v2;
      dat_24578 = v2;
    }
    *dat_24578 = 0;
    if (dat_24570) {
      **(long **)(a1 + 8) = dat_24570;
      *(unsigned long **)(a1 + 8) = dat_24578;
    }
  }
  sub_baa8();
}

// Function: sub_a16b @ 0xa16b
char * sub_a16b(char *a0,unsigned int a1) // ternary
{
  unsigned int v1; // eax
  long v10; // stack - 0x18
  int v2; // eax
  long v3; // rax
  unsigned int v4; // stack - 0x44
  char *v5; // stack - 0x40
  unsigned int v6; // stack - 0x38
  int v7; // stack - 0x34
  unsigned int v8; // stack - 0x30
  char *v9; // stack - 0x20
  
  v3 = (a1 & 4) ? 0 : 2; // branch-flip
  v9 = (char *)((unsigned long)((a1 & 0x40) != 0) + 0x1c600 + v3);
  v6 = 0;
  v4 = a1;
  v5 = a0;
  if (!(a1 & 2)) goto label_a20b;
  v4 = a1 & 0xfffffffd;
label_a1e8:
  if (*v5 == '~')
    v5 = (char *)sub_a4e3(v5,v4);
label_a20b:
  v10 = 0;
  v7 = (int)dat_24540 - (int)dat_24048;
label_a21f:
  do {
    while( true ) {
      v10 += strcspn(&v5[v10],v9);
      v8 = 0;
      v1 = (unsigned int)v5[v10];
      if (((!(v1 & 0x80)) || (v1 == 0xffffff87)) || (v1 == 0xffffff83)) {
        v10 += 1;
        v8 = (unsigned int)((v1 - 1 & 0x80) != 0);
      }
      if ((v10) && (!(v4 & 0x400))) {
        v3 = sub_1094d(v5,v10,dat_24540);
        *(unsigned char *)(v3 + -1) = *(unsigned char *)(v3 + -1) & (char)v8 - 1U;
        if (v4 & 0x80) // branch-flip
          dat_24540 = -(long)(int)v8;
        else {
          dat_24540 = 0;
        }
        dat_24540 = v3 + dat_24540;
        v2 = ((int)v3 - (int)dat_24048) - v8;
        if ((((a1 & 0x180) == 0x80) && (!v6)) && (v7 < v2))
          sub_b70b(v7,v2,0);
        v7 = v2;
      }
      v5 = &v5[v10 + 1];
      v10 = 0;
      if (v8)
        return &v5[-1];
      if (v1 != 0x3d) break;
      v4 |= 0x40;
      v9 = &v9[1];
label_a3d7:
      v5 = &v5[-1];
      if (*v5 == '~') goto label_a1e8;
    }
  } while ((int)v1 > 0x3d);
  if (-0x78 < (int)v1) {
    if (v1 == 0x3a) goto label_a3d7;
    goto label_a21f;
  }
  if (-0x7f > (int)v1) goto label_a21f;
  switch(v1) {
    case 0xffffff81:
      v7 += 1;
      v10 = 1;
      break;
    case 0xffffff82:
      goto label_a46c;
    default:
      goto label_a21f;
    case 0xffffff84:
      sub_a8f0(*(unsigned long *)(dat_24548 + 8),v4 | v6);
      goto label_a20b;
    case 0xffffff86:
      v5 = (char *)sub_a79a(v5,v4 | v6);
      goto label_a20b;
    case 0xffffff88:
      if ((!v6) && (!memcmp(v5,(void *)0x1cc07,5))) {
        v5 = (char *)(sub_af66(&v5[1],v4 | 0x100) + 1);
        goto label_a20b;
      }
      v6 ^= 0x100;
    
  }
  if (v4 & 0x11) {
    v5 = &v5[-1];
    v10 += 1;
    v7 += 1;
  }
  goto label_a21f;
label_a46c:
  v5 = (char *)sub_af66(v5,v4 | v6);
  goto label_a20b;
}

// Function: sub_a4e3 @ 0xa4e3
char * sub_a4e3(char *a0,unsigned int a1) // ternary
{
  char v1;
  long v2; // stack - 0x20
  char *v3; // stack - 0x18
  
  v3 = a0;
  do {
    do {
      do {
        do {
          while( true ) {
            v3 = &v3[1];
            v1 = *v3;
            if (!v1) goto label_a572;
            if (v1 != ':') break;
            if (a1 & 4) goto label_a572;
          }
        } while (';' <= v1);
        if (v1 == '/') goto label_a572;
      } while ('/' < v1);
      if (v1 == '\x88')
        return a0;
    } while ('\x88' < v1);
    if (v1 == '\x81')
      return a0;
  } while (v1 != '\x83');
label_a572:
  if (!(a1 & 0x400)) {
    *v3 = '\0';
    v2 = (a0[1]) ? sub_9fed(&a0[1]) : sub_19d32("HOME"); // branch-flip
    *v3 = v1;
    if (!v2)
      return a0;
    sub_b34b(v2,a1 | 0x100);
  }
  return v3;
}

// Function: sub_a5ec @ 0xa5ec
void sub_a5ec(int a0) // return-dupe x4
{
  unsigned long *v1;
  long v2;
  
  if (!dat_24568)
    return;
  if (a0 < dat_2455c) {
    do {
      if (!dat_24550) {
        if (a0 < dat_24558) {
          dat_24568 = NULL;
          return;
        }
        dat_2455c = a0;
        dat_24568 = (long *)0x24550;
        return;
      }
      dat_243e4 += 1;
      v1 = (unsigned long *)*dat_24550;
      free(dat_24550);
      dat_243e4 -= 1;
      dat_24550 = v1;
    } while ((dat_243e4) || (!dat_243e8));
    sub_64d8(); // no-return
  }
  for (dat_24568 = (long *)0x24550; (*dat_24568 && (*(int *)(*dat_24568 + 8) < a0)); dat_24568 = (long *)*dat_24568) {
  }
  do {
    if (!*dat_24568) {
      if (*(int *)((long)dat_24568 + 0xc) <= a0)
        return;
      *(int *)((long)dat_24568 + 0xc) = a0;
      return;
    }
    dat_243e4 += 1;
    v2 = *(long *)*dat_24568;
    free((void *)*dat_24568);
    *dat_24568 = v2;
    dat_243e4 -= 1;
  } while ((dat_243e4) || (!dat_243e8));
  sub_64d8(); // no-return
}

// Function: sub_a79a @ 0xa79a
unsigned long sub_a79a(unsigned long a0,unsigned int a1)
{
  long v1;
  int v2;
  int v3; // eax
  int v4;
  unsigned long v5; // rax
  unsigned long v6; // rax
  char v7 [24];
  
  v3 = (int)dat_24540 - (int)dat_24048;
  v5 = sub_a16b(a0,a1 & 0x400);
  if (!(a1 & 0x400)) {
    v2 = (int)dat_24048;
    v4 = (int)dat_24540;
    v1 = dat_24048 + v3;
    dat_24540 = v1;
    sub_a5ec(v3);
    if (a1 & 0x11)
      sub_c50c(v1,0);
    sub_10597(v7,(long)(v4 - v2));
    v6 = sub_5994(v1);
    sub_10641(v7);
    v4 = sub_c7b0(v6,a1);
    if (!(a1 & 0x100))
      sub_b70b(v3,v3 + v4,0);
  }
  return v5;
}

// Function: sub_a8f0 @ 0xa8f0
void sub_a8f0(unsigned long a0,unsigned int a1)
{
  int v1; // eax
  int v10; // stack - 0xa8
  unsigned long v11; // stack - 0xa0
  long v2; // rax
  char v3 [32];
  int v4 [2]; // stack - 0xb8
  char v5 [136];
  int v6; // stack - 0xf0
  char *v7; // stack - 0xe8
  unsigned long v8; // stack - 0xe0
  char *v9; // stack - 0xb0
  
  if (!(a1 & 0x400)) {
    dat_243e4 += 1;
    v1 = (int)dat_24540 - (int)dat_24048;
    sub_10597(v3,(long)v1);
    sub_76d6(a0,v4);
    sub_10641(v3);
    v7 = v9;
    v6 = v10;
    if (!v10) goto label_a9de;
    while( true ) {
      sub_b226(v7,(long)v6,a1);
label_a9de:
      if (v4[0] < 0) break;
      do {
        v2 = read(v4[0],v5,0x80);
        v6 = (int)v2;
        if (0 <= v6) break;
      } while (*dat_266e8 == 4);
      if (v6 <= 0) break;
      v7 = v5;
    }
    if (v9)
      free(v9);
    if (0 <= v4[0]) {
      close(v4[0]);
      dat_24404 = sub_ec74(v11);
    }
    dat_243e4 -= 1;
    if ((!dat_243e4) && (dat_243e8))
      sub_64d8(); // no-return
    for (v8 = dat_24540; ((unsigned long)(v1 + dat_24048) < v8 && (*(char *)(v8 - 1) == '\n')); v8 = v8 - 1) {
    }
    dat_24540 = v8;
    if (!(a1 & 0x100))
      sub_b70b(v1,(int)v8 - (int)dat_24048,0);
  }
  dat_24548 = (unsigned long *)*dat_24548;
}

// Function: sub_ab64 @ 0xab64
char * sub_ab64(char *a0,char *a1,unsigned long a2,char *a3,int a4,int a5)
{
  char v1;
  char *v2; // stack - 0x20
  char *v3; // stack - 0x18
  char *v4; // stack - 0x10
  
  v2 = a0;
  v3 = a1;
  do {
    v4 = v3;
    v1 = *v3;
    if (a5) {
      *v3 = '\0';
      v4 = a1;
    }
    *v3 = v1;
    if (!fnmatch(a3,v4,0))
      return v2;
    if ((a4) && (*v2 == '\x81'))
      v2 = &v2[1];
    v2 = &v2[1];
    v3 = &v3[1];
  } while (v1);
  return NULL;
}

// Function: sub_ac29 @ 0xac29
char * sub_ac29(char *a0,char *a1,char *a2,char *a3,int a4,int a5)
{
  char *v1;
  char v2;
  unsigned int v3; // stack - 0x28
  char *v4; // stack - 0x20
  char *v5; // stack - 0x18
  char *v6; // stack - 0x10
  
  v3 = 0;
  v4 = &a3[-1];
  v5 = a2;
  while( true ) {
    if (v4 < a0)
      return NULL;
    v2 = *v5;
    v6 = v5;
    if (a5) {
      *v5 = '\0';
      v6 = a1;
    }
    *v5 = v2;
    if (!fnmatch(a3,v6,0)) break;
    v1 = &v4[-1];
    if (a4) {
      v3 -= 1;
      if ((int)v3 <= -1)
        v3 = sub_9fae(a0,v1);
      if (v3 & 1) {
        v3 -= 1;
        v1 = &v4[-2];
      }
    }
    v4 = v1;
    v5 = &v5[-1];
  }
  return v4;
}

// Function: sub_ad25 @ 0xad25
unsigned long sub_ad25(unsigned long a0,long a1,int a2,int a3,unsigned int a4,unsigned int a5) // ternary x2
{
  unsigned int v1; // eax
  unsigned int v2; // eax
  unsigned long v3; // rax
  long v4; // rax
  void *v5; // rax
  char *v6; // stack - 0x40
  char *v7; // stack - 0x38
  char *v8; // stack - 0x30
  char *v9; // stack - 0x28
  
  v1 = a4 & 0xf;
  v2 = (a1) ? 0 : 0x10; // branch-flip
  v3 = sub_a16b(a0,v2 | a5 & 0x400 | 2);
  if (!(a5 & 0x400)) {
    v6 = (char *)(a3 + dat_24048);
    if (v1 == 4)
      sub_c8f5(a0,a1,v6,a4); // no-return
    if (v1 != 5) { // branch-flip
      v9 = (char *)(a2 + dat_24048);
      v8 = v6;
      if (a5 & 0x11) {
        v8 = (char *)sub_c50c(v6,9);
        if (v8 != v6) {
          v9 = dat_24540;
          v6 = (char *)(a3 + dat_24048);
        }
      }
      v4 = a2 + dat_24048;
      sub_9f88(v4,0);
      v2 = (int)(v1 - 6) >> 1;
      v5 = (v2 != (v1 - 6 & 1)) ? sub_ab64 : sub_ac29; // branch-flip
      v7 = (char *)(*v5)(v6,v8,&v9[-1],v4,a5 & 0x11,v2);
      if (v7) { // branch-flip
        if (v2) {
          memmove(v6,v7,v4 - (long)v7);
          v7 = &v6[(v4 - (long)v7) + -1];
        }
        *v7 = 0;
      }
      else {
        v7 = (char *)(v4 + -1);
      }
    }
    else {
      sub_1985b(a1,v6,0);
      v7 = v6;
    }
    dat_24540 = v7;
    sub_a5ec(a3);
  }
  return v3;
}

// Function: sub_af66 @ 0xaf66
char * sub_af66(char *a0,unsigned int a1) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_af66
{
  char *v1;
  unsigned int v10; // stack - 0x24
  long v11; // stack - 0x18
  int v2;
  int v3;
  int v4; // eax
  unsigned int v5; // stack - 0x44
  char *v6; // stack - 0x40
  unsigned int v7; // stack - 0x30
  unsigned int v8; // stack - 0x2c
  unsigned int v9; // stack - 0x28
  
  v1 = &a0[1];
  v8 = (unsigned int)*a0;
  v7 = v8 & 0xf;
  v10 = a1 & 0x100;
  v4 = (int)dat_24540 - (int)dat_24048;
  v6 = &strchr(v1,0x3d)[1];
  while( true ) {
    v11 = sub_b38a(v1,v8,a1,v10);
    if (v8 & 0x10)
      v11 -= 1;
    v9 = (unsigned int)(v11 >> 0x35) & 0x400;
    v5 = a1;
    if (5 < v7) break;
    if (v7 <= 3) {
      if (v7 != 3) { // branch-flip
        if ((4 <= v7) || ((v7 && (v7 != 2)))) goto label_b0cb;
      }
      else {
        v9 ^= 0x400;
      }
      v6 = (char *)sub_a16b(v6,a1 | 0x82 | v9 ^ 0x400);
      goto label_b1b2;
    }
    v6 = (char *)sub_ad25(v6,v1,0,v4,v8,a1 & 0xffffffee | v9 ^ 0x400);
    if ((~v9 | a1) & 0x400) goto label_b1b2;
    v8 &= 0xffffffef;
    v7 = 1;
  }
label_b0cb:
  if ((~a1 & v9) && (dat_26a4e))
    sub_c8f5(v6,v1,0,0); // no-return
  if (v7 == 10) {
    v6 = &v6[1];
    if (a1 & 0x400)
      return v6;
    if (v11 <= -1)
      v11 = 0;
    sub_c7b0(v11,a1);
    goto label_b1c8;
  }
  if (v7 != 1) {
    v5 = a1 | v9;
    if (!(v5 & 0x400))
      dat_24540 = sub_9f4c(0,dat_24540);
    v3 = (int)dat_24048;
    v2 = (int)dat_24540;
    v6 = (char *)sub_ad25(v6,0,v2 - v3,v4,v8,v5);
  }
label_b1b2:
  if (!((v5 | v9) & 0x400)) {
label_b1c8:
    if (v10) {
      v10 = !((*v1 != '@') || (!dat_26a00));
      if (!v10)
        return v6;
    }
    sub_b70b(v4,(int)dat_24540 - (int)dat_24048,v10);
  }
  return v6;
}

// Function: sub_b226 @ 0xb226
long sub_b226(char *a0,long a1,unsigned int a2) // early-return, ternary
{
  char *v1;
  char *v2;
  char v3;
  int v4; // eax
  long v5; // rax
  char *v6; // rax
  long v7; // stack - 0x38
  char *v8; // stack - 0x30
  char *v9; // stack - 0x20
  
  v5 = (a2 & 0x100) ? 0x1dbe1 : 0x1dac1; // branch-flip
  if (!a1)
    return 0;
  v6 = (char *)sub_1090b(a1 * 2,dat_24540);
  v9 = v6;
  v7 = a1;
  v8 = a0;
  do {
    v1 = &v8[1];
    v3 = *v8;
    v4 = (int)v3;
    if (v4) { // branch-flip
      if ((a2 & 0x11) && ((*(char *)(v5 + v4) == '\f' || ((a2 & 0x100 && (*(char *)(v5 + v4) == '\x02')))))) {
        v2 = &v9[1];
        *v9 = '\x81';
        v9 = v2;
      }
label_b312:
      v2 = &v9[1];
      *v9 = v3;
      v9 = v2;
    }
    else if (a2 & 0x200) goto label_b312;
    v7 -= 1;
    v8 = v1;
    if (!v7) {
      dat_24540 = v9;
      return (long)v9 - (long)v6;
    }
  } while( true );
}

// Function: sub_b34b @ 0xb34b
unsigned long sub_b34b(char *a0,unsigned int a1)
{
  unsigned long v1; // rax
  
  v1 = strlen(a0);
  sub_b226(a0,v1,a1);
  return v1;
}

// Function: sub_b38a @ 0xb38a
long sub_b38a(char *a0,unsigned int a1,unsigned int a2,unsigned int a3) // early-return x5, ternary x2, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_b38a
{
  char *v1;
  unsigned int v10; // stack - 0x34
  unsigned int v11; // stack - 0x30
  unsigned int v12; // stack - 0x2c
  char *v13; // stack - 0x28
  long *v14; // stack - 0x20
  long v15; // stack - 0x18
  long *v2;
  unsigned int v3;
  unsigned int v4; // eax
  long v5; // rax
  char v6; // stack - 0x3e
  unsigned char v7; // stack - 0x3d
  int v8; // stack - 0x3c
  int v9; // stack - 0x38
  
  v11 = a1 & 0xf;
  v12 = !((v11 != 3) && (v11 != 10));
  v12 = a2 & 0x400 | v12;
  v15 = 0;
  if (!v11) {
    if (!v12)
      sub_6621("Bad substitution"); // no-return
    return -1;
  }
  v3 = (v12) ? 0xffffffee : 0xffffffff; // branch-flip
  v3 = (a2 | 0x200) & v3;
  v10 = (v3 & 1) << 8;
  switch(*a0) {
    case 0x21:
      v8 = dat_26648;
      if (!dat_26648)
        return -1;
      break;
    default:
      v13 = (char *)sub_19d32(a0);
      goto label_b6ad;
    case 0x23:
      v8 = dat_26a00;
      break;
    case 0x24:
      v8 = dat_266e0;
      break;
    case 0x2a:
label_b562:
      if ((~v10 | a3) & 0x100) // branch-flip
        v7 = 0xff;
      else {
        v7 = 0;
      }
      v4 = (dat_24128 & 0x20) ? 0x20 : (unsigned int)(*(unsigned char *)(dat_24130 + 4) & v7); // branch-flip
      v10 = v10 & ~a3 | v4;
      goto label_b5b8;
    case 0x2d:
      v13 = (char *)sub_1090b(0x11,dat_24540);
      for (v9 = 0x10; 0 <= v9; v9 = v9 + -1) {
        if ((*(char *)((long)v9 + 0x26a40)) && ("efIimnsxvVECabu"[v9])) {
          v1 = &v13[1];
          *v13 = "efIimnsxvVECabu"[v9];
          v15 += 1;
          v13 = v1;
        }
      }
      dat_24540 = v13;
      goto label_b6d3;
    case 0x30:
    case 0x31:
    case 0x32:
    case 0x33:
    case 0x34:
    case 0x35:
    case 0x36:
    case 0x37:
    case 0x38:
    case 0x39:
      v8 = atoi(a0);
      if ((v8 < 0) || (dat_26a00 < v8))
        return -1;
      if (v8) // branch-flip
        v13 = (char *)dat_26a08[(long)v8 + -1];
      else {
        v13 = (char *)dat_269f0;
      }
label_b6ad:
      if (!v13)
        return -1;
      v15 = sub_b34b(v13,v3);
      goto label_b6d3;
    case 0x3f:
      v8 = dat_24400;
      break;
    case 0x40:
      if ((!a3) || (!(v3 & 1))) goto label_b562;
label_b5b8:
      v6 = (char)v10;
      v14 = dat_26a08;
      if (!dat_26a08)
        return -1;
      while( true ) {
        v2 = &v14[1];
        v13 = (long)*v14;
        if (!v13) break;
        v14 = v2;
        v5 = sub_b34b(v13,v3);
        v15 = v5 + v15;
        if ((*v14) && (v10)) {
          v15 += 1;
          sub_b226(&v6,1,v3);
        }
      }
      goto label_b6d3;
    
  }
  v5 = (long)v8;
  v15 = sub_c7b0(v5,v3);
label_b6d3:
  if (v12)
    dat_24540 = &dat_24540[-v15];
  return v15;
}

// Function: sub_b70b @ 0xb70b
void sub_b70b(unsigned int a0,unsigned int a1,unsigned int a2)
{
  unsigned long *v1; // stack - 0x10
  
  if (dat_24568) { // branch-flip
    dat_243e4 += 1;
    v1 = (unsigned long *)sub_10354(0x18);
    *v1 = 0;
    *dat_24568 = v1;
    dat_243e4 -= 1;
    if ((!dat_243e4) && (dat_243e8))
      sub_64d8(); // no-return
  }
  else {
    v1 = (unsigned long *)0x24550;
  }
  dat_24568 = v1;
  *(unsigned int *)&v1[1] = a0;
  *(unsigned int *)((long)v1 + 0xc) = a1;
  *(unsigned int *)&v1[2] = a2;
}

// Function: sub_b7cc @ 0xb7cc
void sub_b7cc(char *a0,int a1,long a2) // return-dupe, ternary
{
  int v1;
  int v10; // stack - 0x74
  int v11; // stack - 0x54
  int v12; // stack - 0x50
  long *v13; // stack - 0x48
  char *v14; // stack - 0x40
  char *v15; // stack - 0x38
  char *v16; // stack - 0x30
  char *v17; // stack - 0x28
  bool v2;
  char *v3;
  char *v4;
  char *v5; // rax
  char *v6;
  long v7; // rax
  bool v8;
  bool v9;
  
  v17 = NULL;
  v14 = a0;
  if (dat_24568) {
    v11 = 0;
    v5 = (dat_24128 & 0x20) ? (char *)0x240e4 : (char *)(dat_24130 + 4); // branch-flip
    v13 = (long *)0x24550;
    v10 = a1;
    do {
      v15 = &a0[(int)v13[1]];
      v1 = (int)v13[2];
      if (v1) // branch-flip
        v6 = (char *)0x269e0;
      else {
        v6 = v5;
      }
      v2 = 0;
label_ba1b:
      v3 = v15;
      if (v15 < &a0[*(int *)((long)v13 + 0xc)]) {
        v16 = v15;
        v12 = (int)*v15;
        v4 = &v15[1];
        if (v12 == -0x7f) {
          v12 = (int)v15[1];
          v4 = &v15[2];
        }
        v15 = v4;
        v8 = strchr(v6,v12) != NULL;
        v9 = 0;
        if (v8)
          v9 = strchr((char *)0x240e4,v12) != NULL;
        if (v10) {
          if (v2) {
            if (v8)
              v16 = v15;
            v14 = v16;
            if (v9) goto label_ba1b;
            v8 = 0;
          }
          if (v8) {
            if ((!v11) && (!v1))
              v2 = v9;
            if ((v16 == v14) && (v2)) {
              v14 = v15;
              v2 = 0;
            }
            else if ((1 <= v10) && (v10 = v10 + -1, !v10))
              v17 = v16;
            else {
              *v16 = '\0';
              v7 = sub_10422(0x10);
              *(char **)(v7 + 8) = v14;
              **(long **)(a2 + 8) = v7;
              *(long *)(a2 + 8) = v7;
              v14 = v15;
            }
          }
          else {
            v2 = 0;
          }
          goto label_ba1b;
        }
        if (v9) {
          if (!v17)
            v17 = v3;
        }
        else {
          if ((!v8) || (!v2))
            v17 = NULL;
          v2 = 0;
        }
        goto label_ba1b;
      }
      v13 = (long *)*v13;
      v11 = v1;
    } while (v13);
    if (v1) goto label_ba6e;
  }
  if (v17)
    *v17 = '\0';
  if (!*v14)
    return;
label_ba6e:
  v7 = sub_10422(0x10);
  *(char **)(v7 + 8) = v14;
  **(long **)(a2 + 8) = v7;
  *(long *)(a2 + 8) = v7;
  return;
}

// Function: sub_baa8 @ 0xbaa8
void sub_baa8(void)
{
  long *v1;
  long *v2; // stack - 0x18
  
  v2 = dat_24550;
  if (dat_24550) {
    dat_243e4 += 1;
    do {
      v1 = (long *)*v2;
      free(v2);
      v2 = v1;
    } while (v1);
    dat_24550 = NULL;
    dat_243e4 -= 1;
    if ((!dat_243e4) && (dat_243e8))
      sub_64d8(); // no-return
  }
  dat_24568 = 0;
}

// Function: sub_bb3d @ 0xbb3d
void sub_bb3d(long *a0)
{
  long *v1;
  char *v2; // rax
  unsigned long v3; // rax
  long *v4; // stack - 0x30
  long *v5; // stack - 0x20
  
  v4 = a0;
  do {
    if (!v4)
      return;
    if ((dat_26a41) || (v2 = strpbrk((char *)v4[1],"*?["), v1 = dat_24578, !v2)) {
label_bc65:
      *dat_24578 = (long)v4;
      sub_c50c(v4[1],0);
      dat_24578 = v4;
    }
    else {
      dat_243e4 += 1;
      v2 = (char *)sub_9f88(v4[1],0x11);
      v3 = strlen(v2);
      dat_24588 = (int)v3 + 0x1000;
      dat_24580 = (void *)sub_10354(dat_24588);
      sub_bcff(v2,v3 & 0xffffffff,0);
      free(dat_24580);
      if (v2 != (char *)v4[1])
        free(v2);
      dat_243e4 -= 1;
      if ((!dat_243e4) && (dat_243e8))
        sub_64d8(); // no-return
      if (v1 == dat_24578) goto label_bc65;
      *dat_24578 = 0;
      v5 = (long *)sub_c358(*v1);
      *v1 = (long)v5;
      for (; *v5; v5 = (long *)*v5) {
      }
      dat_24578 = v5;
    }
    v4 = (long *)*v4;
  } while( true );
}

// Function: sub_bcff @ 0xbcff
void sub_bcff(char *a0,int a1,unsigned int a2) // return-dupe x5, ternary
{
  char *v1;
  char v10 [152];
  bool v11;
  int v12; // stack - 0xf4
  char *v13; // stack - 0xe8
  char *v14; // stack - 0xe0
  char *v15; // stack - 0xd8
  char *v16; // stack - 0xd0
  char *v17; // stack - 0xc8
  char *v18; // stack - 0xc0
  void *v19; // stack - 0xb8
  unsigned int v2;
  bool v3;
  int v4; // eax
  unsigned int v5; // eax
  int v6; // eax
  char *v7;
  long v8; // rax
  char v9;
  
  v13 = &dat_24580[a2];
  v11 = 0;
  v16 = a0;
  for (v14 = a0; v17 = v14, v12 = 0, *v14; v14 = &v14[(long)v12 + 1]) {
    if ((*v14 != '*') && (*v14 != '?')) {
      if (*v14 != '[') { // branch-flip
        if ((*v14 == '\\') && (v14[1]))
          v12 = 1;
        if (v14[v12] == '/') {
          if (v11) break;
          v16 = &v14[(long)v12 + 1];
        }
      }
      else {
        v18 = &v14[1];
        if (*v18 == '!')
          v18 = &v14[2];
        do {
          if (*v18 == '\\')
            v18 = &v18[1];
          if ((*v18 == '/') || (!*v18)) goto label_be9a;
          v18 = &v18[1];
        } while (*v18 != ']');
        v11 = 1;
      }
    }
    else {
      v11 = 1;
    }
label_be9a:
  }
  if (v11) {
    if (a0 < v16) {
      v14 = a0;
      do {
        if ((*v14 == '\\') && (v14[1]))
          v14 = &v14[1];
        v1 = &v14[1];
        v7 = &v13[1];
        *v13 = *v14;
        v13 = v7;
        v14 = v1;
      } while (v1 < v16);
    }
    *v13 = '\0';
    v15 = dat_24580;
    v5 = (int)v13 - (int)dat_24580;
    if (!v5)
      v15 = ".";
    v19 = opendir(v15);
    if (!v19)
      return;
    v11 = *v17 != '\0';
    if (v11) {
      *v17 = '\0';
      v17 = &v17[(long)v12 + 1];
    }
    a1 -= (int)v17 - (int)a0;
    v3 = 0;
    v14 = v16;
    if (*v16 == '\\')
      v14 = &v16[1];
    if (*v14 == '.')
      v3 = 1;
    while ((!dat_243e8 && (v8 = readdir64(v19), v8))) {
      if (((*(char *)(v8 + 0x13) != '.') || (v3)) && (!fnmatch(v16,(char *)(v8 + 0x13),0))) {
        if (v11) {
          v7 = stpcpy(v13,(char *)(v8 + 0x13));
          *v7 = '/';
          v4 = ((int)v7 - (int)dat_24580) + 1;
          v6 = a1 + v4;
          if (dat_24588 < v6 + 0xffU) {
            v2 = v6 + 0x10ff;
            dat_24580 = (char *)sub_10395(dat_24580,v2);
            dat_24588 = v2;
          }
          sub_bcff(v17,a1,v4);
          v13 = &dat_24580[v5];
        }
        else {
          strcpy(v13,(char *)(v8 + 0x13));
          sub_c30a(dat_24580);
        }
      }
    }
    closedir(v19);
    if (!v11)
      return;
    v9 = (v12) ? '\\' : '/'; // branch-flip
    v17[(long)-v12 + -1] = v9;
    return;
  }
  if (!a2)
    return;
  v14 = a0;
  do {
    if ((*v14 == '\\') && (v14[1]))
      v14 = &v14[1];
    v7 = &v13[1];
    *v13 = *v14;
    v1 = &v14[1];
    v9 = *v14;
    v13 = v7;
    v14 = v1;
  } while (v9);
  v4 = lstat64(dat_24580,v10);
  if (v4 <= -1)
    return;
  sub_c30a(dat_24580);
}

// Function: sub_c30a @ 0xc30a
void sub_c30a(unsigned long a0)
{
  long v1; // rax
  
  v1 = sub_10422(0x10);
  *(unsigned long *)(v1 + 8) = sub_11925(a0);
  *dat_24578 = v1;
  dat_24578 = (long *)v1;
}

// Function: sub_c358 @ 0xc358
void sub_c358(unsigned long *a0)
{
  int v1; // stack - 0x14
  unsigned long *v2; // stack - 0x10
  
  v1 = 0;
  for (v2 = a0; v2; v2 = (unsigned long *)*v2) {
    v1 += 1;
  }
  sub_c3a2(a0,v1);
}

// Function: sub_c3a2 @ 0xc3a2
long * sub_c3a2(unsigned long *a0,int a1)
{
  long *v1;
  long *v10; // stack - 0x10
  long *v2;
  int v3; // eax
  long *v4; // rax
  unsigned long *v5; // stack - 0x30
  int v6; // stack - 0x28
  int v7; // stack - 0x24
  long *v8; // stack - 0x20
  long *v9; // stack - 0x18
  
  v9 = NULL;
  if (2 <= a1) { // branch-flip
    v3 = a1 >> 1;
    v6 = v3;
    v8 = a0;
    while (v6 = v6 + -1, 0 <= v6) {
      v9 = v8;
      v8 = (unsigned long *)*v8;
    }
    *v9 = 0;
    v5 = a0;
    v7 = v3;
    v4 = (long *)sub_c3a2(a0,v3);
    v9 = v4;
    v3 = a1 - v7;
    v4 = (long *)sub_c3a2(v8,v3);
    v8 = v4;
    v10 = (long *)&v5;
    v1 = v8;
    v2 = v9;
    do {
      while( true ) {
        v9 = v2;
        v8 = v1;
        v3 = strcmp((char *)v8[1],(char *)v9[1]);
        if (0 <= v3) break;
        *v10 = (long)v8;
        v10 = v8;
        v1 = (long *)*v8;
        v2 = v9;
        if (!(long *)*v8) {
          *v8 = (long)v9;
          v4 = v5; // return-dupe
          return v4;
        }
      }
      *v10 = (long)v9;
      v10 = v9;
      v1 = v8;
      v2 = (long *)*v9;
    } while ((long *)*v9);
    *v9 = (long)v8;
    v4 = v5;
  }
  else {
    v4 = a0;
  }
  return v4;
}

// Function: sub_c4ca @ 0xc4ca
bool sub_c4ca(unsigned long a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)sub_9f88(a0,0);
  return fnmatch(v1,a1,0) == 0;
}

// Function: sub_c50c @ 0xc50c
char * sub_c50c(char *a0,unsigned int a1) // ternary
{
  char *v1;
  char *v10; // stack - 0x28
  char *v11; // stack - 0x20
  char *v2;
  int v3;
  unsigned int v4; // eax
  unsigned long v5; // rax
  long v6; // rax
  char *v7; // stack - 0x50
  unsigned int v8; // stack - 0x3c
  char *v9; // stack - 0x30
  
  v9 = strpbrk(a0,(char *)0x1cc0d);
  if (v9) {
    v10 = v9;
    v11 = a0;
    if (a1 & 1) {
      v5 = (long)v9 - (long)a0;
      v6 = strlen(v9) + v5 + 1;
      if (a1 & 8) { // branch-flip
        v3 = (int)dat_24048;
        v11 = (char *)sub_1090b(v6,dat_24540);
        v7 = (char *)(((int)a0 - v3) + dat_24048);
        v9 = &v7[v5];
      }
      else {
        v7 = a0;
        v11 = (a1 & 0x10) ? (char *)sub_10354(v6) : (char *)sub_10422(v6); // branch-flip
      }
      v10 = v11;
      if (v5)
        v10 = mempcpy(v11,v7,v5);
    }
    v4 = a1 & 2;
    v8 = v4;
    while (*v9) {
      if (*v9 != '\x88') { // branch-flip
        if (*v9 != '\\') { // branch-flip
          if (*v9 == '\x81') {
            v9 = &v9[1];
            if (v8) {
              v1 = &v10[1];
              *v10 = '\\';
              v10 = v1;
            }
          }
          v8 = v4;
        }
        else {
          v8 = 0;
        }
        v1 = &v9[1];
        v2 = &v10[1];
        *v10 = *v9;
        v9 = v1;
        v10 = v2;
      }
      else {
        v9 = &v9[1];
        v8 = v4;
      }
    }
    *v10 = '\0';
    if (a1 & 8)
      dat_24540 = &v10[1];
    a0 = v11;
  }
  return a0;
}

// Function: sub_c70c @ 0xc70c
unsigned int sub_c70c(long a0,unsigned long a1)
{
  unsigned int v1; // eax
  char v2 [24];
  
  sub_105e6(v2);
  dat_24548 = *(unsigned long *)(a0 + 0x18);
  dat_24540 = dat_24048;
  sub_a16b(*(unsigned long *)(a0 + 0x10),0x12);
  sub_baa8();
  v1 = sub_c4ca(dat_24048,a1);
  sub_10641(v2);
  return v1;
}

// Function: sub_c7b0 @ 0xc7b0
void sub_c7b0(unsigned long a0,unsigned int a1)
{
  long v1;
  char *v10; // stack - 0x48
  int v2; // eax
  unsigned long v3; // rax
  char v4 [4];
  char *v5; // rsp
  unsigned int v6; // stack - 0x64
  unsigned long v7; // stack - 0x60
  int v8; // stack - 0x54
  long v9; // stack - 0x50
  
  v6 = a1;
  v7 = a0;
  v2 = sub_9f16(8);
  v8 = v2;
  v9 = (long)v2 + -1;
  v3 = (((long)v2 + 0xfU) / 0x10) * 0x10;
  for (v5 = v4; v5 != &v4[-(v3 & 0xfffffffffffff000)]; v5 = &v5[-0x1000]) {
    *(unsigned long *)&v5[-8] = *(unsigned long *)&v5[-8];
  }
  v1 = -(unsigned long)((unsigned int)v3 & 0xfff);
  if (v3 & 0xfff)
    *(unsigned long *)&v5[((unsigned long)((unsigned int)v3 & 0xfff) - 8) + v1] = *(unsigned long *)&v5[((unsigned long)((unsigned int)v3 & 0xfff) - 8) + v1];
  v10 = &v5[v1];
  *(unsigned long *)&v5[v1 + -8] = 0xc8b4;
  v2 = sub_172fa((long)v5 + v1,(long)v2,"%ld",v7);
  v8 = v2;
  *(unsigned long *)&v5[v1 + -8] = 0xc8cf;
  sub_b226(v10,(long)v2,v6);
}

// Function: sub_c8f5 @ 0xc8f5
void sub_c8f5(char *a0,long a1,char *a2,unsigned int a3)
{
  char *v1; // stack - 0x18
  char *v2; // stack - 0x10
  
  v2 = (char *)0x269e0;
  v1 = "parameter not set";
  if (a2) {
    if (*a0 != '\x83') // branch-flip
      v1 = a2;
    else if (a3 & 0x10)
      v2 = " or null";
  }
  sub_6621("%.*s: %s%s",&a0[0xffffffffffffffff - a1],a1,v1,v2); // no-return
}

// Function: sub_c98c @ 0xc98c
void sub_c98c(void *a0)
{
  void *v1;
  void *v2; // stack - 0x20
  
  dat_243e4 += 1;
  v2 = a0;
  do {
    if (*(long *)((long)v2 + 0x18)) {
      *(unsigned int *)(*(long *)((long)v2 + 0x18) + 0x18) = *(unsigned int *)(*(long *)((long)v2 + 0x18) + 0x18) & 0xfffffffe;
      if (*(unsigned int *)(*(long *)((long)v2 + 0x18) + 0x18) & 2)
        sub_4c77(*(unsigned long *)(*(long *)((long)v2 + 0x18) + 8));
    }
    v1 = *(void **)((long)v2 + 0x28);
    if (v2 != (void *)(dat_24028 + 0x30))
      free(v2);
    v2 = v1;
  } while (v1);
  *(unsigned long *)(dat_24028 + 0x70) = 0;
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
}

// Function: sub_ca6d @ 0xca6d
int sub_ca6d(void)
{
  char *v1;
  int v2; // stack - 0xc
  
  if (*(int *)(dat_24028 + 0x80)) { // branch-flip
    *(int *)(dat_24028 + 0x80) = *(int *)(dat_24028 + 0x80) + -1;
    v2 = *(int *)(dat_24028 + 8 + ((long)*(int *)(dat_24028 + 0x80) + 0x1c) * 4);
  }
  else {
    *(int *)(dat_24028 + 0x10) = *(int *)(dat_24028 + 0x10) + -1;
    if (0 <= *(int *)(dat_24028 + 0x10)) { // branch-flip
      v1 = *(char **)(dat_24028 + 0x18);
      *(char **)(dat_24028 + 0x18) = &v1[1];
      v2 = (int)*v1;
    }
    else {
      v2 = sub_cc4f();
    }
    *(unsigned int *)(dat_24028 + 0x7c) = *(unsigned int *)(dat_24028 + 0x78);
    *(int *)(dat_24028 + 0x78) = v2;
  }
  return v2;
}

// Function: sub_cb1c @ 0xcb1c
void sub_cb1c(void)
{
  if (*(long *)(dat_24028 + 0x70))
    sub_c98c(*(long *)(dat_24028 + 0x70));
  sub_ca6d();
}

// Function: sub_cb5a @ 0xcb5a
int sub_cb5a(void)
{
  void *v1;
  int v2;
  unsigned int v3; // eax
  
  v1 = *(void **)(dat_24028 + 0x20);
  *(void **)(dat_24028 + 0x18) = v1;
  while( true ) {
    do {
      v2 = *(int *)(dat_24028 + 0xc);
      v2 = (int)read(v2,v1,0x2000);
      if (0 <= v2)
        return v2;
    } while (*dat_266e8 == 4);
    if (*(int *)(dat_24028 + 0xc))
      return v2;
    if (*dat_266e8 != 0xb)
      return v2;
    v3 = fcntl(0,3,0);
    if ((int)v3 < 0) break;
    if (!(v3 & 0x800))
      return v2;
    if (fcntl(0,4,(unsigned long)(v3 & 0xfffff7ff)) < 0)
      return v2;
    sub_1701f("sh: turning off NDELAY mode\n",dat_240d0);
  }
  return v2;
}

// Function: sub_cc4f @ 0xcc4f
unsigned long sub_cc4f(void) // early-return x2
{
  char v1;
  char *v2;
  unsigned long v3; // rax
  int v4; // stack - 0x18
  char *v5; // stack - 0x10
  
  if (*(long *)(dat_24028 + 0x28)) { // branch-flip
    sub_cfeb();
    v3 = sub_ca6d();
  }
  else {
    if ((*(int *)(dat_24028 + 0x10) == -99) || (!*(long *)(dat_24028 + 0x20))) {
      v3 = 0xffffff7f;
      return v3;
    }
    sub_170b1();
    v4 = *(int *)(dat_24028 + 0x14);
    if (0 < (int)v4) goto label_cd0a;
    do {
      v4 = sub_cb5a();
      if ((int)v4 <= 0) {
        *(unsigned int *)(dat_24028 + 0x10) = 0xffffff9d;
        *(unsigned int *)(dat_24028 + 0x14) = *(unsigned int *)(dat_24028 + 0x10);
        v3 = 0xffffff7f;
        return v3;
      }
label_cd0a:
      v5 = *(char **)(dat_24028 + 0x18);
      do {
        v4 -= 1;
        v1 = *v5;
        if (v1) { // branch-flip
          v5 = &v5[1];
          if (v1 == '\n') {
            *(int *)(dat_24028 + 0x10) = ((int)v5 - (int)*(unsigned long *)(dat_24028 + 0x18)) + -1;
            goto label_cdb4;
          }
        }
        else {
          memmove(v5,&v5[1],(long)(int)v4);
        }
      } while (0 < (int)v4);
      *(int *)(dat_24028 + 0x10) = ((int)v5 - (int)*(unsigned long *)(dat_24028 + 0x18)) + -1;
    } while (*(int *)(dat_24028 + 0x10) <= -1);
label_cdb4:
    *(unsigned int *)(dat_24028 + 0x14) = v4;
    v1 = *v5;
    *v5 = '\0';
    if (dat_26a48)
      sub_1701f(*(unsigned long *)(dat_24028 + 0x18),dat_240d0);
    *v5 = v1;
    v2 = *(char **)(dat_24028 + 0x18);
    *(char **)(dat_24028 + 0x18) = &v2[1];
    v4 = (unsigned int)*v2;
    v3 = (unsigned long)v4;
  }
  return v3;
}

// Function: sub_ce1f @ 0xce1f
void sub_ce1f(void)
{
  *(int *)(dat_24028 + 0x80) = *(int *)(dat_24028 + 0x80) + 1;
}

// Function: sub_ce40 @ 0xce40
void sub_ce40(char *a0,long a1)
{
  unsigned long v1; // rax
  unsigned long *v2; // stack - 0x18
  
  v1 = strlen(a0);
  dat_243e4 += 1;
  if (*(long *)(dat_24028 + 0x70) || *(long *)(dat_24028 + 0x28)) { // branch-flip
    v2 = (unsigned long *)sub_10354(0x40);
    *v2 = *(unsigned long *)(dat_24028 + 0x28);
    *(unsigned long **)(dat_24028 + 0x28) = v2;
  }
  else {
    *(long *)(dat_24028 + 0x28) = dat_24028 + 0x30;
    v2 = *(unsigned long **)(dat_24028 + 0x28);
  }
  v2[1] = *(unsigned long *)(dat_24028 + 0x18);
  *(unsigned int *)&v2[2] = *(unsigned int *)(dat_24028 + 0x10);
  *(unsigned int *)&v2[7] = *(unsigned int *)(dat_24028 + 0x80);
  v2[5] = *(unsigned long *)(dat_24028 + 0x70);
  memcpy(&v2[6],(void *)(dat_24028 + 0x78),8);
  v2[3] = a1;
  if (a1) {
    *(unsigned int *)(a1 + 0x18) = *(unsigned int *)(a1 + 0x18) | 1;
    v2[4] = a0;
  }
  *(char **)(dat_24028 + 0x18) = a0;
  *(int *)(dat_24028 + 0x10) = (int)v1;
  *(unsigned int *)(dat_24028 + 0x80) = 0;
  *(unsigned long *)(dat_24028 + 0x70) = 0;
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
}

// Function: sub_cfeb @ 0xcfeb
void sub_cfeb(void)
{
  unsigned long *v1;
  
  v1 = *(unsigned long **)(dat_24028 + 0x28);
  dat_243e4 += 1;
  if (v1[3]) {
    if ((*(char *)(*(long *)(dat_24028 + 0x18) + -1) == ' ') || (*(char *)(*(long *)(dat_24028 + 0x18) + -1) == '\t'))
      dat_26a78 |= 1;
    if (v1[4] != *(long *)(v1[3] + 0x10))
      free((void *)v1[4]);
  }
  *(unsigned long *)(dat_24028 + 0x18) = v1[1];
  *(unsigned int *)(dat_24028 + 0x10) = *(unsigned int *)&v1[2];
  *(unsigned int *)(dat_24028 + 0x80) = *(unsigned int *)&v1[7];
  memcpy((void *)(dat_24028 + 0x78),&v1[6],8);
  *(unsigned long *)(dat_24028 + 0x28) = *v1;
  *(unsigned long **)(dat_24028 + 0x70) = v1;
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
}

// Function: sub_d12d @ 0xd12d
int sub_d12d(unsigned long a0,unsigned int a1)
{
  int v1; // stack - 0xc
  
  dat_243e4 += 1;
  v1 = sub_15cc1(a0,0,a1 & 2);
  if (0 <= v1) {
    if (v1 <= 9)
      v1 = sub_1629a(v1,v1);
    sub_d1cd(v1,a1 & 1);
  }
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
  return v1;
}

// Function: sub_d1cd @ 0xd1cd
void sub_d1cd(unsigned int a0,int a1)
{
  long v1;
  
  if (a1) {
    sub_d2f0();
    *(unsigned long *)(dat_24028 + 0x20) = 0;
  }
  v1 = dat_24028;
  *(unsigned int *)(dat_24028 + 0xc) = a0;
  if (!*(long *)(dat_24028 + 0x20))
    *(unsigned long *)(v1 + 0x20) = sub_10354(0x2001);
  *(unsigned int *)(dat_24028 + 0x10) = 0;
  *(unsigned int *)(dat_24028 + 0x14) = *(unsigned int *)(dat_24028 + 0x10);
  *(unsigned int *)(dat_24028 + 8) = 1;
}

// Function: sub_d25c @ 0xd25c
void sub_d25c(char *a0)
{
  unsigned long v1; // rax
  
  dat_243e4 += 1;
  sub_d2f0();
  *(char **)(dat_24028 + 0x18) = a0;
  v1 = strlen(a0);
  *(int *)(dat_24028 + 0x10) = (int)v1;
  *(unsigned long *)(dat_24028 + 0x20) = 0;
  *(unsigned int *)(dat_24028 + 8) = 1;
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
}

// Function: sub_d2f0 @ 0xd2f0
void sub_d2f0(void)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_10354(0x88);
  *v1 = dat_24028;
  *(unsigned int *)((long)v1 + 0xc) = 0xffffffff;
  v1[5] = 0;
  v1[0xe] = 0;
  v1[6] = 0;
  *(unsigned int *)&v1[0x10] = 0;
  dat_24028 = v1;
}

// Function: sub_d363 @ 0xd363
void sub_d363(void)
{
  unsigned long *v1;
  
  v1 = dat_24028;
  dat_243e4 += 1;
  if (0 <= *(int *)((long)dat_24028 + 0xc))
    close(*(int *)((long)dat_24028 + 0xc));
  if (v1[4])
    free((void *)v1[4]);
  if (dat_24028[0xe])
    sub_c98c(dat_24028[0xe]);
  while (v1[5]) {
    sub_cfeb();
    sub_c98c(dat_24028[0xe]);
  }
  dat_24028 = (unsigned long)*v1;
  free(v1);
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
}

// Function: sub_d44e @ 0xd44e
void sub_d44e(long a0)
{
  while (a0 != dat_24028) {
    sub_d363();
  }
}

// Function: sub_d476 @ 0xd476
void sub_d476(void)
{
  sub_d44e(0x245a0);
}

// Function: sub_d490 @ 0xd490
void sub_d490(void)
{
  sigsetmask(0);
}

// Function: sub_d4a1 @ 0xd4a1
void sub_d4a1(long a0,int a1)
{
  long v1;
  long *v2; // stack - 0x20
  
  v2 = (long *)0x26668;
  while (v1 = *v2, v1 != a0) {
    v2 = (long *)(v1 + 0x20);
  }
  *v2 = *(long *)(v1 + 0x20);
  v2 = (long *)0x26668;
  if (a1) {
    if (a1 != 1)
      return;
    while ((v1 = *v2, v1 && (*(char *)(v1 + 0x1e) == '\x01'))) {
      v2 = (long *)(v1 + 0x20);
    }
  }
  *(long *)(a0 + 0x20) = *v2;
  *v2 = a0;
}

// Function: sub_d556 @ 0xd556
void sub_d556(int a0)
{
  unsigned int v1;
  int v2;
  int v3; // stack - 0x1c
  int v4; // stack - 0x14
  
  v2 = dat_26664;
  if ((a0 != dat_26650) && (!dat_266e4)) {
    v3 = a0;
    if (a0) { // branch-flip
      v2 = sub_15cc1("/dev/tty",2,1);
      v4 = v2;
      if (0 <= v2) { // branch-flip
label_d5d2:
        v4 = sub_1629a(v4,v2);
        while (v2 = tcgetpgrp(v4), 0 <= v2) {
          if (v2 == getpgrp()) {
            dat_26664 = v2;
            sub_1677f(0x14);
            sub_1677f(0x16);
            sub_1677f(0x15);
            v1 = dat_266e0;
            setpgid(0,dat_266e0);
            sub_f9ff(v4,v1);
            dat_24030 = v4; // return-dupe
            dat_26650 = v3;
            return;
          }
          killpg(0,0x15);
        }
      }
      else {
        v4 = v2 + 3;
        do {
          if (isatty(v4)) goto label_d5d2;
          v4 -= 1;
        } while (0 <= v4);
      }
      sub_679b("can\'t access tty; job control turned off");
      v3 = 0;
      dat_26a44 = 0;
    }
    else {
      v4 = dat_24030;
      sub_f9ff(dat_24030,dat_26664);
      setpgid(0,v2);
      sub_1677f(0x14);
      sub_1677f(0x16);
      sub_1677f(0x15);
    }
    close(v4);
    v4 = -1;
    dat_24030 = v4;
    dat_26650 = v3;
  }
  return;
}

// Function: sub_d703 @ 0xd703
unsigned int sub_d703(int a0,long a1) // early-return x2, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_d703
{
  long v1;
  unsigned long v2;
  int v3;
  long *v4; // stack - 0x48
  int v5; // stack - 0x2c
  int v6; // stack - 0x28
  int v7; // stack - 0x24
  int v8; // stack - 0x20
  
  v5 = -1;
  v6 = 0;
  if (1 < a0) {
    v4 = (long *)(a1 + 8);
    if (*(char *)*v4 == '-') {
      v5 = sub_16db1(*v4 + 1,1);
      if (0 <= v5) // branch-flip
        v4 = (long *)(a1 + 0x10);
      else {
        while (v3 = sub_12769("ls:"), v3) {
          if (v3 != 0x73) // branch-flip
            v6 = 1;
          else {
            v5 = sub_16db1(dat_26a20,1);
            if (v5 <= -1)
              sub_6621("invalid signal number or name: %s",dat_26a20); // no-return
          }
        }
        v4 = dat_26a18;
      }
    }
    v2 = dat_240c8;
    if ((!v6) && (v5 <= -1))
      v5 = 0xf;
    v3 = !((0 <= v5) && (*v4));
    if (v3 == v6) {
      if (!v6) {
        v7 = 0;
        do {
          if (*(char *)*v4 != '%') { // branch-flip
            if (*(char *)*v4 != '-') // branch-flip
              v8 = sub_1174e(*v4);
            else {
              v1 = *v4;
              v8 = -sub_1174e(v1 + 1);
            }
          }
          else {
            v1 = *v4;
            v8 = -**(int **)(sub_e413(v1,0) + 0x10);
          }
          if (kill(v8,v5)) {
            v3 = *dat_266e8;
            sub_679b("%s\n",strerror(v3));
            v7 = 1;
          }
          v4 = &v4[1];
        } while (*v4);
        return v7;
      }
      if (*v4) {
        v5 = sub_1174e(*v4);
        if (0x81 <= v5)
          v5 -= 0x80;
        if ((1 <= v5) && (v5 <= 0x40)) {
          sub_1714e(v2,"%s\n",*(unsigned long *)((long)v5 * 8 + 0x23540));
          return 0;
        }
        sub_6621("invalid signal number or exit status: %s",*v4); // no-return
      }
      sub_1701f("0\n",dat_240c8);
      for (v7 = 1; v7 <= 0x40; v7 = v7 + 1) {
        sub_1714e(v2,"%s\n",*(unsigned long *)((long)v7 * 8 + 0x23540));
      }
      return 0;
    }
  }
  sub_6621("Usage: kill [-s sigspec | -signum | -sigspec] [pid | job]... or\nkill -l [exitstatus]"); // no-return
}

// Function: sub_da08 @ 0xda08
int sub_da08(long a0)
{
  return (int)(a0 - dat_26658 >> 3) * -0x33333333 + 1;
}

// Function: sub_da3c @ 0xda3c
unsigned int sub_da3c(unsigned long a0,unsigned long *a1)
{
  char v1;
  unsigned long v2;
  unsigned int v3; // eax
  long v4; // rax
  long *v5; // stack - 0x38
  
  v1 = *(char *)*a1;
  sub_12769(0x269e0);
  v2 = dat_240c8;
  v5 = dat_26a18;
  do {
    v4 = sub_e413(*v5,1);
    if (v1 != 'f') {
      sub_d4a1(v4,1);
      sub_1714e(v2,"[%d] ",sub_da08(v4));
    }
    sub_1701f(*(unsigned long *)(*(long *)(v4 + 0x10) + 8),v2);
    sub_f970(v4,v2);
    v3 = sub_db49(v4,v1 != 'f');
  } while ((*v5) && (v5 = &v5[1], *v5));
  return v3;
}

// Function: sub_db49 @ 0xdb49
unsigned int sub_db49(long a0,int a1)
{
  unsigned int v1;
  unsigned int v2; // stack - 0x1c
  long v3; // stack - 0x10
  
  dat_243e4 += 1;
  if (*(char *)(a0 + 0x1e) != '\x02') {
    *(char *)(a0 + 0x1e) = 0;
    v1 = **(unsigned int **)(a0 + 0x10);
    if (!a1)
      sub_f9ff(dat_24030,v1);
    killpg(v1,0x12);
    v3 = *(long *)(a0 + 0x10);
    v2 = (unsigned int)*(unsigned short *)(a0 + 0x1c);
    do {
      if ((*(unsigned int *)(v3 + 4) & 0xff) == 0x7f)
        *(unsigned int *)(v3 + 4) = 0xffffffff;
      v3 += 0x10;
      v2 -= 1;
    } while (v2);
  }
  if (a1) // branch-flip
    v1 = 0;
  else {
    v1 = sub_ec74(a0);
  }
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
  return v1;
}

// Function: sub_dc46 @ 0xdc46
long sub_dc46(char *a0,unsigned int a1,int a2)
{
  unsigned int v1; // eax
  unsigned int v2; // stack - 0x14
  char *v3; // stack - 0x10
  
  v1 = (int)a1 >> 8 & 0xff;
  v3 = a0;
  if (a1 & 0x7f) { // branch-flip
    v2 = (int)a1 >> 8 & 0xff;
    if ((a1 & 0xff) != 0x7f)
      v2 = a1 & 0x7f;
    if ((!a2) || (((v2 != 2 && (v2 != 0xd)) && ((a1 & 0xff) != 0x7f)))) {
      v3 = stpncpy(a0,(char *)strsignal(v2),0x20);
      if (a1 & 0x80)
        v3 = stpcpy(v3," (core dumped)");
    }
  }
  else if (!a2) {
    if (v1) // branch-flip
      v3 = &a0[(int)sub_172fa(a0,0x10,"Done(%d)",v1)];
    else {
      v3 = stpcpy(a0,"Done");
    }
  }
  return (long)v3 - (long)a0;
}

// Function: sub_dd7b @ 0xdd7b
void sub_dd7b(unsigned long a0,long a1,unsigned int a2) // return-dupe x2
{
  unsigned int v1;
  unsigned int *v2; // rax
  int v3;
  char v4 [88];
  int v5; // stack - 0x84
  unsigned int v6; // stack - 0x80
  int v7; // stack - 0x7c
  unsigned int *v8; // stack - 0x78
  
  v8 = *(unsigned int **)(a1 + 0x10);
  if (a2 & 1) {
    sub_1714e(a0,"%d\n",*v8);
    return;
  }
  v7 = sub_172fa(v4,0x10,"[%d]   ",sub_da08(a1));
  v5 = v7;
  if (a1 != dat_26668) { // branch-flip
    if ((dat_26668) && (a1 == *(long *)(dat_26668 + 0x20)))
      v4[v7 + -2] = '-';
  }
  else {
    v4[v7 + -2] = '+';
  }
  if (a2 & 4) {
    v1 = *v8;
    v5 = v7 + sub_172fa(&v4[v7],0x10,"%d ",v1);
  }
  v2 = &v8[(unsigned long)*(unsigned short *)(a1 + 0x1c) * 4];
  if (*(char *)(a1 + 0x1e)) { // branch-flip
    v6 = v2[-3];
    if (*(char *)(a1 + 0x1e) == '\x01')
      v6 = *(unsigned int *)(a1 + 0x18);
    v5 += sub_dc46(&v4[v5],v6,0);
  }
  else {
    strcpy(&v4[v5],"Running");
    v5 += (int)strlen("Running");
  }
  while( true ) {
    v3 = 0;
    if (0 <= 0x21U - v5)
      v3 = 0x21U - v5;
    sub_1714e(a0,"%s%*c%s",v4,v3,0x20,*(unsigned long *)&v8[2]);
    if (!(a2 & 4)) break;
    v8 = &v8[4];
    if (v8 == v2) {
      sub_1705d(10,a0);
label_e03b:
      *(unsigned char *)(a1 + 0x1f) = *(unsigned char *)(a1 + 0x1f) & 0xef;
      if (*(char *)(a1 + 0x1e) != '\x02')
        return;
      sub_e1b5(a1);
      return;
    }
    v1 = *v8;
    v5 = sub_172fa(v4,0x30," |\n%*c%d ",v7,0x20,v1) + -3;
  }
  sub_f970(a1,a0);
  goto label_e03b;
}

// Function: sub_e080 @ 0xe080
unsigned long sub_e080(void) // return-dupe, ternary
{
  long v1;
  unsigned long v2;
  int v3; // eax
  long *v4; // stack - 0x28
  unsigned int v5; // stack - 0x18
  
  v5 = 0;
  while (v3 = sub_12769("lp"), v2 = dat_240c8, v3) {
    v5 = (v3 != 0x6c) ? 1 : 4; // branch-flip
  }
  v4 = dat_26a18;
  if (*dat_26a18) {
    do {
      v1 = *v4;
      sub_dd7b(v2,sub_e413(v1,0),v5);
      v4 = &v4[1];
    } while (*v4);
    return 0;
  }
  sub_e140(dat_240c8,v5);
  return 0;
}

// Function: sub_e140 @ 0xe140
void sub_e140(unsigned long a0,unsigned int a1)
{
  long v1; // stack - 0x10
  
  sub_ef38(0,0);
  for (v1 = dat_26668; v1; v1 = *(long *)(v1 + 0x20)) {
    if ((!(a1 & 8)) || (*(unsigned char *)(v1 + 0x1f) & 0x10))
      sub_dd7b(a0,v1,a1);
  }
}

// Function: sub_e1b5 @ 0xe1b5
void sub_e1b5(long a0)
{
  unsigned int v1; // stack - 0x14
  long v2; // stack - 0x10
  
  dat_243e4 += 1;
  v1 = (unsigned int)*(unsigned short *)(a0 + 0x1c);
  v2 = *(long *)(a0 + 0x10);
  while (v1 = v1 - 1, 0 <= (int)v1) {
    if (*(long *)(v2 + 8) != 0x269e0)
      free(*(void **)(v2 + 8));
    v2 += 0x10;
  }
  if (*(long *)(a0 + 0x10) != a0)
    free(*(void **)(a0 + 0x10));
  *(unsigned char *)(a0 + 0x1f) = *(unsigned char *)(a0 + 0x1f) & 0xf7;
  sub_d4a1(a0,2);
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
}

// Function: sub_e28e @ 0xe28e
int sub_e28e(void)
{
  long v1;
  int v2; // eax
  long *v3; // stack - 0x38
  int v4; // stack - 0x20
  long v5; // stack - 0x18
  long v6; // stack - 0x10
  
  sub_12769(0x269e0);
  v3 = dat_26a18;
  if (*dat_26a18) { // branch-flip
    v4 = 0x7f;
    do {
      if (*(char *)*v3 != '%') { // branch-flip
        v2 = sub_1174e(*v3);
        for (v5 = dat_26668; v5; v5 = *(long *)(v5 + 0x20)) {
          if (v2 == *(int *)((unsigned long)*(unsigned short *)(v5 + 0x1c) * 0x10 + -0x10 + *(long *)(v5 + 0x10))) goto label_e3b0;
        }
      }
      else {
        v1 = *v3;
        v5 = sub_e413(v1,0);
label_e3b0:
        if (!sub_ef38(2,v5)) {
          v4 = dat_26d20 + 0x80; // return-dupe
          return v4;
        }
        *(unsigned char *)(v5 + 0x1f) = *(unsigned char *)(v5 + 0x1f) | 4;
        v4 = sub_fa62(v5);
      }
      v3 = &v3[1];
    } while (*v3);
  }
  else {
    do {
      v6 = dat_26668;
      while( true ) {
        if (!v6)
          return 0;
        if (!*(char *)(v6 + 0x1e)) break;
        *(unsigned char *)(v6 + 0x1f) = *(unsigned char *)(v6 + 0x1f) | 4;
        v6 = *(long *)(v6 + 0x20);
      }
    } while (sub_ef38(4,0));
    v4 = dat_26d20 + 0x80;
  }
  return v4;
}

// Function: sub_e413 @ 0xe413
long sub_e413(char *a0,int a1)
{
  char v1;
  unsigned long v2;
  unsigned int v3; // eax
  long v4; // stack - 0x30
  long v5; // stack - 0x28
  char *v6; // stack - 0x20
  char *v7; // stack - 0x18
  void *v8; // stack - 0x10
  
  v6 = "No such job: %s";
  v4 = dat_26668;
  if (a0) { // branch-flip
    if (*a0 != '%') {
      sub_6621(v6,a0); // return-dupe, no-return
    }
    v7 = &a0[1];
    v1 = *v7;
    if (!v1) goto label_e496;
    if (!a0[2]) {
      if ((v1 == '+') || (v1 == '%')) goto label_e496;
      if (v1 != '-') goto label_e4d7;
      if (dat_26668)
        v4 = *(long *)(dat_26668 + 0x20);
      v6 = "No previous job";
      goto label_e4c7;
    }
label_e4d7:
    if (((sub_11792(v7)) && (v3 = atoi(v7), v3)) && (v3 <= dat_26660)) {
      v4 = (unsigned long)v3 * 0x28 + -0x28 + dat_26658;
      if (!(*(unsigned char *)(v4 + 0x1f) & 8)) {
        sub_6621(v6,a0);
      }
    }
    else {
      v8 = sub_115d5;
      if (*v7 == '?') {
        v8 = dat_23fe8;
        v7 = &a0[2];
      }
      v5 = 0;
      for (; v4; v4 = *(long *)(v4 + 0x20)) {
        v2 = *(unsigned long *)(*(long *)(v4 + 0x10) + 8);
        if ((*v8)(v2,v7)) {
          if (v5) {
            sub_6621(v6,a0);
          }
          v5 = v4;
          v6 = "%s: ambiguous";
        }
      }
      if (!v5) {
        sub_6621(v6,a0);
      }
      v4 = v5;
    }
  }
  else {
label_e496:
    v6 = "No current job";
label_e4c7:
    if (!v4) {
      sub_6621(v6,a0);
    }
  }
  v6 = "job %s not created under job control";
  if ((!a1) || (*(unsigned char *)(v4 + 0x1f) & 2))
    return v4;
  sub_6621(v6,a0);
}

// Function: sub_e61f @ 0xe61f
void * sub_e61f(unsigned long a0,int a1)
{
  int v1; // stack - 0x14
  void *v2; // stack - 0x10
  
  v1 = dat_26660;
  v2 = dat_26658;
  do {
    v1 -= 1;
    if (v1 <= -1) {
      v2 = (void *)sub_e730();
label_e6a9:
      memset(v2,0,0x28);
      if (dat_26650)
        *(unsigned char *)((long)v2 + 0x1f) = *(unsigned char *)((long)v2 + 0x1f) | 2;
      *(void **)((long)v2 + 0x20) = dat_26668;
      dat_26668 = v2;
      *(unsigned char *)((long)v2 + 0x1f) = *(unsigned char *)((long)v2 + 0x1f) | 8;
      *(void **)((long)v2 + 0x10) = v2;
      if (2 <= a1)
        *(unsigned long *)((long)v2 + 0x10) = sub_10354((long)a1 << 4);
      return v2;
    }
    if (!(*(unsigned char *)((long)v2 + 0x1f) & 8)) goto label_e6a9;
    if (((*(char *)((long)v2 + 0x1e) == '\x02') && (*(unsigned char *)((long)v2 + 0x1f) & 4)) && (!dat_26650)) {
      sub_e1b5(v2);
      goto label_e6a9;
    }
    v2 = (void *)((long)v2 + 0x28);
  } while( true );
}

// Function: sub_e730 @ 0xe730
unsigned long sub_e730(void)
{
  unsigned long v1;
  long v2;
  long v3; // rax
  long v4; // rax
  unsigned long v5; // stack - 0x30
  long v6; // stack - 0x28
  
  v2 = dat_26658;
  v3 = (unsigned long)dat_26660 * 0x28;
  dat_26658 = sub_10395(dat_26658,v3 + 0xa0);
  v4 = dat_26658 - v2;
  if (v4) {
    v5 = v2 + v3;
    v6 = v3;
    while (v6) {
      v6 -= 0x28;
      v5 -= 0x28;
      if (*(long *)(v6 + dat_26658 + 0x10) == v5)
        *(long *)(v6 + dat_26658 + 0x10) = v4 + *(long *)(v6 + dat_26658 + 0x10);
      if (*(long *)(v6 + dat_26658 + 0x20))
        *(long *)(v6 + dat_26658 + 0x20) = v4 + *(long *)(v6 + dat_26658 + 0x20);
    }
    if (dat_26668)
      dat_26668 = v4 + dat_26668;
  }
  dat_26660 += 4;
  v1 = dat_26658 + v3;
  v5 = v1 + 0x78;
  do {
    *(unsigned char *)(v5 + 0x1f) = *(unsigned char *)(v5 + 0x1f) & 0xf7;
    v5 -= 0x28;
  } while (v1 <= v5);
  return v1;
}

// Function: sub_e8b1 @ 0xe8b1
void sub_e8b1(long a0,unsigned long a1,int a2) // return-dupe
{
  int v1;
  int v2;
  long v3; // stack - 0x20
  int v4; // stack - 0x14
  
  v2 = dat_266e4;
  v1 = dat_2664c;
  if (!dat_2664c) {
    dat_266e4 += 1;
    sub_1aa88();
    dat_26650 = 0;
  }
  if (((a2 != 2) && (*(unsigned char *)(a0 + 0x1f) & 2)) && (!v2)) {
    if (*(short *)(a0 + 0x1c)) // branch-flip
      v4 = **(int **)(a0 + 0x10);
    else {
      v4 = getpid();
    }
    setpgid(0,v4);
    if (!a2)
      sub_f9ff(dat_24030,v4);
    sub_1677f(0x14);
    sub_1677f(0x16);
  }
  else if (a2 == 1) {
    sub_16a48(2);
    sub_16a48(3);
    if (!*(short *)(a0 + 0x1c)) {
      close(0);
      sub_15cc1("/dev/null",0,0);
    }
  }
  if ((!v2) && (dat_26a43)) {
    sub_1677f(2);
    sub_1677f(3);
    sub_1677f(0xf);
  }
  if (v1)
    return;
  for (v3 = dat_26668; v3; v3 = *(long *)(v3 + 0x20)) {
    sub_e1b5(v3);
  }
}

// Function: sub_ea2a @ 0xea2a
void sub_ea2a(long a0,long a1,int a2,int a3)
{
  unsigned short v1;
  int *v2; // rax
  int v3; // stack - 0x14
  
  if (a3 <= -1) {
    if (a0)
      sub_e1b5(a0);
    sub_6621("Cannot fork"); // no-return
  }
  if (a0) {
    if ((a2 != 2) && (*(unsigned char *)(a0 + 0x1f) & 2)) {
      if (*(short *)(a0 + 0x1c)) // branch-flip
        v3 = **(int **)(a0 + 0x10);
      else {
        v3 = a3;
      }
      setpgid(a3,v3);
    }
    if (a2 == 1) {
      dat_26648 = a3;
      sub_d4a1(a0,1);
    }
    if (a0) {
      v1 = *(unsigned short *)(a0 + 0x1c);
      *(unsigned short *)(a0 + 0x1c) = v1 + 1;
      v2 = (int *)((unsigned long)v1 * 0x10 + *(long *)(a0 + 0x10));
      *v2 = a3;
      v2[1] = -1;
      v2[2] = 0x269e0;
      v2[3] = 0;
      if ((dat_26650) && (a1))
        *(unsigned long *)&v2[2] = sub_f164(a1);
    }
  }
}

// Function: sub_eb5f @ 0xeb5f
int sub_eb5f(unsigned long a0,unsigned long a1,unsigned int a2)
{
  int v1; // eax
  
  v1 = fork();
  if (v1) // branch-flip
    sub_ea2a(a0,a1,a2,v1);
  else {
    sub_e8b1(a0,a1,a2);
  }
  return v1;
}

// Function: sub_ebb7 @ 0xebb7
unsigned long sub_ebb7(unsigned long a0,unsigned long a1,unsigned long a2,unsigned int a3)
{
  int v1; // eax
  unsigned long v2; // rax
  
  v2 = sub_e61f(a0,1);
  sub_16e27(0);
  dat_2664c += 1;
  v1 = vfork();
  if (!v1) {
    sub_e8b1(v2,a0,0);
    sub_d490();
    sub_8884(a1,a2,a3); // no-return
  }
  dat_2664c = 0;
  sub_d490();
  sub_ea2a(v2,a0,0,v1);
  return v2;
}

// Function: sub_ec74 @ 0xec74
unsigned int sub_ec74(long a0)
{
  unsigned int v1;
  
  sub_ef38(a0 != 0,a0);
  if (a0) { // branch-flip
    v1 = sub_fa62(a0);
    if ((*(unsigned char *)(a0 + 0x1f) & 2) && (sub_f9ff(dat_24030,dat_266e0), *(unsigned char *)(a0 + 0x1f) & 1))
      raise(2);
    if (*(char *)(a0 + 0x1e) == '\x02')
      sub_e1b5(a0);
  }
  else {
    v1 = dat_24400;
  }
  return v1;
}

// Function: sub_ed15 @ 0xed15
int sub_ed15(unsigned int a0,long a1)
{
  int v1; // stack - 0x78
  char v2 [56];
  unsigned int v3; // stack - 0x74
  int v4; // stack - 0x70
  int v5; // stack - 0x6c
  long v6; // stack - 0x68
  long v7; // stack - 0x60
  int *v8; // stack - 0x58
  int *v9; // stack - 0x50
  
  v7 = 0;
  dat_243e4 += 1;
  v4 = sub_efdb(a0,&v1);
  if (1 <= v4) {
    for (v6 = dat_26668; v6; v6 = *(long *)(v6 + 0x20)) {
      if (*(char *)(v6 + 0x1e) != '\x02') {
        v3 = 2;
        v9 = (int *)((unsigned long)*(unsigned short *)(v6 + 0x1c) * 0x10 + *(long *)(v6 + 0x10));
        v8 = *(int **)(v6 + 0x10);
        do {
          if (v4 == *v8) {
            v8[1] = v1;
            v7 = v6;
          }
          if (v8[1] == -1)
            v3 = 0;
          if ((v3) && ((v8[1] & 0xffU) == 0x7f)) {
            *(int *)(v6 + 0x18) = v8[1];
            v3 = 1;
          }
          v8 = &v8[4];
        } while (v8 < v9);
        if (v7) {
          if (((v3) && (*(unsigned char *)(v7 + 0x1f) = *(unsigned char *)(v7 + 0x1f) | 0x10, v3 != *(unsigned char *)(v7 + 0x1e))) && (*(char *)(v7 + 0x1e) = (char)v3, v3 == 1))
            sub_d4a1(v7,0);
          break;
        }
      }
    }
  }
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
  if (((v7) && (v7 == a1)) && (v5 = sub_dc46(v2,v1,1), v5)) {
    v2[v5] = 10;
    v2[v5 + 1] = 0;
    sub_1701f(v2,dat_240d0);
  }
  return v4;
}

// Function: sub_ef38 @ 0xef38
unsigned char sub_ef38(unsigned int a0,long a1) // early-return
{
  unsigned char v1;
  int v2; // eax
  unsigned int v3; // stack - 0x1c
  
  v3 = a0;
  if ((a1) && (*(char *)(a1 + 0x1e)))
    v3 = 0;
  if ((!v3) && (!dat_26d24))
    return 1;
  v1 = 1;
  do {
    v2 = sub_ed15(v3,a1);
    v1 &= v2 != 0;
    v3 &= 0xfffffffb;
    if ((!v2) || ((a1 && (*(char *)(a1 + 0x1e)))))
      v3 = 0;
  } while (0 <= v2);
  return v1;
}

// Function: sub_efdb @ 0xefdb
int sub_efdb(int a0,unsigned long a1)
{
  int v1; // eax
  char v2 [136];
  unsigned char v3;
  
  v3 = a0 != 1;
  if (dat_26650)
    v3 |= 2;
  do {
    dat_26d24 = 0;
    do {
      v1 = wait3(a1,v3,0);
      if (0 <= v1) break;
    } while (*dat_266e8 == 4);
    if (v1)
      return v1;
    if (-(unsigned int)(a0 == 0))
      return -(unsigned int)(a0 == 0);
    sub_16e27(v2);
    while ((!dat_26d24 && (!dat_26d20))) {
      sigsuspend(v2);
    }
    sub_d490();
    if (!dat_26d24)
      return 0;
  } while( true );
}

// Function: sub_f0f7 @ 0xf0f7
unsigned int sub_f0f7(void)
{
  unsigned int v1; // stack - 0x14
  
  v1 = 0;
  if (((!dat_26654) && (dat_26668)) && (*(char *)(dat_26668 + 0x1e) == '\x01')) {
    sub_1701f("You have stopped jobs.\n",dat_240d0);
    dat_26654 = 2;
    v1 = 1;
  }
  return v1;
}

// Function: sub_f164 @ 0xf164
void sub_f164(unsigned long a0)
{
  dat_26670 = dat_24048;
  sub_f1a7(a0);
  sub_103e1(dat_24048);
}

// Function: sub_f1a7 @ 0xf1a7
void sub_f1a7(int *a0) // return-dupe
{
  char v1 [2]; // stack - 0x12
  unsigned long v2; // stack - 0x40
  long v3; // stack - 0x30
  long *v4; // stack - 0x28
  char *v5; // stack - 0x20
  
  if (!a0)
    return;
  switch(*a0) {
    case 0:
      sub_f631(*(unsigned long *)&a0[4],1);
      sub_f631(*(unsigned long *)&a0[6],0);
      return;
    default:
      v4 = *(long **)&a0[2];
      while( true ) {
        sub_f1a7(v4[1]);
        v4 = (long *)*v4;
        if (!v4) break;
        sub_f6a0(" | ");
      }
      return;
    case 2:
    case 3:
      v2 = *(unsigned long *)&a0[2];
      sub_f1a7(v2); // return-dupe
      return;
    case 4:
      sub_f6a0("(");
      v2 = *(unsigned long *)&a0[2];
      v5 = ")";
      sub_f1a7(v2); // return-dupe
      sub_f6a0(v5); // return-dupe
      return;
    case 5:
      v5 = " && ";
      goto label_f269;
    case 6:
      v5 = " || ";
      goto label_f269;
    case 7:
      v5 = "; ";
label_f269:
      sub_f1a7(*(unsigned long *)&a0[2]);
      sub_f6a0(v5);
      v2 = *(unsigned long *)&a0[4];
      sub_f1a7(v2);
      return;
    case 8:
      sub_f6a0("if ");
      sub_f1a7(*(unsigned long *)&a0[2]);
      sub_f6a0("; then ");
      if (*(long *)&a0[6]) { // branch-flip
        sub_f1a7(*(unsigned long *)&a0[4]);
        sub_f6a0("; else ");
        v2 = *(unsigned long *)&a0[6];
      }
      else {
        v2 = *(unsigned long *)&a0[4];
      }
      v5 = "; fi";
      sub_f1a7(v2);
      sub_f6a0(v5);
      return;
    case 9:
      v5 = "while ";
      goto label_f391;
    case 10:
      v5 = "until ";
label_f391:
      sub_f6a0(v5);
      sub_f1a7(*(unsigned long *)&a0[2]);
      v2 = *(unsigned long *)&a0[4];
label_f3c4:
      v5 = "; done";
      sub_f6a0("; do ");
      sub_f1a7(v2);
      sub_f6a0(v5);
      return;
    case 0xb:
      sub_f6a0("for ");
      sub_f6a0(*(unsigned long *)&a0[6]);
      sub_f6a0(" in ");
      sub_f631(*(unsigned long *)&a0[2],1);
      v2 = *(unsigned long *)&a0[4];
      goto label_f3c4;
    case 0xc:
      sub_f6a0("case ");
      sub_f6a0(*(unsigned long *)(*(long *)&a0[2] + 0x10));
      sub_f6a0(" in ");
      for (v3 = *(long *)&a0[4]; v3; v3 = *(long *)(v3 + 8)) {
        sub_f1a7(*(unsigned long *)(v3 + 0x10));
        sub_f6a0(") ");
        sub_f1a7(*(unsigned long *)(v3 + 0x18));
        sub_f6a0(";; ");
      }
      v5 = "esac";
      sub_f6a0(v5);
      return;
    case 0xe:
      sub_f6a0(*(unsigned long *)&a0[2]);
      v5 = "() { ... }";
      sub_f6a0(v5);
      return;
    case 0xf:
      v5 = *(char **)&a0[4];
      sub_f6a0(v5);
      return;
    case 0x10:
      v5 = ">";
      break;
    case 0x11:
      v5 = ">|";
      break;
    case 0x12:
      v5 = "<";
      break;
    case 0x13:
      v5 = "<>";
      break;
    case 0x14:
      v5 = ">>";
      break;
    case 0x15:
      v5 = ">&";
      break;
    case 0x16:
      v5 = "<&";
      break;
    case 0x17:
    case 0x18:
      v5 = "<<...";
      sub_f6a0(v5);
      return;
    case 0x19:
      sub_f6a0("!");
      v2 = *(unsigned long *)&a0[2];
      sub_f1a7(v2);
      return;
    
  }
  v1[0] = (char)a0[4] + '0';
  v1[1] = 0;
  sub_f6a0(v1);
  sub_f6a0(v5);
  if ((*a0 != 0x15) && (*a0 != 0x16)) {
    v2 = *(unsigned long *)&a0[6];
    sub_f1a7(v2);
    return;
  }
  v1[0] = (char)a0[5] + '0';
  v5 = v1;
  sub_f6a0(v5);
  return;
}

// Function: sub_f631 @ 0xf631
void sub_f631(long a0,int a1)
{
  long v1; // stack - 0x10
  
  for (v1 = a0; v1; v1 = *(long *)(v1 + 8)) {
    if (!a1)
      sub_f6a0(" ");
    sub_f1a7(v1);
    if ((a1) && (*(long *)(v1 + 8)))
      sub_f6a0(" ");
  }
}

// Function: sub_f6a0 @ 0xf6a0
void sub_f6a0(char *a0) // ternary
{
  char *v1;
  char *v10; // stack - 0x40
  char *v11; // stack - 0x38
  char *v12; // stack - 0x30
  char *v2;
  unsigned long v3;
  int v4; // eax
  unsigned long v5; // rax
  char v6 [2]; // stack - 0x22
  char v7; // stack - 0x49
  unsigned int v8; // stack - 0x48
  unsigned int v9; // stack - 0x44
  
  v3 = dat_26670;
  v6[0] = ' ';
  v6[1] = '\0';
  v8 = 0;
  v9 = 0;
  v5 = strlen(a0);
  v12 = (char *)sub_1090b((v5 + 1) * 8,v3);
  v10 = a0;
label_f90e:
  v2 = &v10[1];
  v7 = *v10;
  if (!v7) {
    if (v9 & 1) {
      v2 = &v12[1];
      *v12 = '\"';
      v12 = v2;
    }
    *v12 = '\0';
    dat_26670 = v12;
    return;
  }
  v11 = NULL;
  v4 = (int)v7;
  if (-0x78 < v4) {
    if ((0x5c < v4) || (v4 <= 0x21)) goto label_f8c0;
    v5 = 1L << (v7 - 0x22U & 0x3fU);
    if (v5 & 0x400000000000025) {
      v11 = v6;
      v7 = '\\';
      v10 = v2;
      goto label_f8c3;
    }
    if (!(v5 & 0x8000000)) goto label_f8c0;
    v10 = v2;
    if (!v8) goto label_f8c3;
    if ((v8 & 0xf) != 1)
      v9 <<= 1;
    v11 = (char *)((long)(int)(v8 & 0xf) * 4 + 0x1c980);
    if (v8 & 0x10) {
      v7 = ':';
      goto label_f8c3;
    }
    goto label_f8d8;
  }
  if (v4 <= -0x80) {
label_f8c0:
    v10 = v2;
    goto label_f8c3;
  }
  switch(v4) {
    case 0xffffff81:
      v10 = &v10[2];
      v7 = *v2;
      goto label_f8c3;
    case 0xffffff82:
      v10 = &v10[2];
      v8 = (unsigned int)*v2;
      v11 = ((v8 & 0xf) != 10) ? "${" : "${#"; // branch-flip
      goto label_f8f6;
    case 0xffffff83:
      v11 = &"\"}"[(v9 & 1) == 0];
      v9 = (int)v9 >> 1;
      v8 = 0;
      v10 = v2;
      break;
    case 0xffffff84:
      v11 = "$(...)";
      v10 = v2;
      break;
    default:
      goto label_f8c0;
    case 0xffffff86:
      v11 = "$((";
      v10 = v2;
      break;
    case 0xffffff87:
      v11 = "))";
      v10 = v2;
      break;
    case 0xffffff88:
      v9 ^= 1;
      v7 = '\"';
      v10 = v2;
label_f8c3:
      v2 = &v12[1];
      *v12 = v7;
      v12 = v2;
label_f8d8:
      if (!v11) goto label_f90e;
    
  }
label_f8f6:
  while( true ) {
    v1 = &v11[1];
    if (!*v11) break;
    v2 = &v12[1];
    *v12 = *v11;
    v11 = v1;
    v12 = v2;
  }
  goto label_f90e;
}

// Function: sub_f970 @ 0xf970
void sub_f970(long a0,unsigned long a1)
{
  unsigned short v1;
  long v2;
  unsigned long v3;
  unsigned long v4; // stack - 0x18
  
  v2 = *(long *)(a0 + 0x10);
  v1 = *(unsigned short *)(a0 + 0x1c);
  v3 = *(unsigned long *)(a0 + 0x10);
  while (v4 = v3 + 0x10, v4 < (unsigned long)v1 * 0x10 + v2) {
    sub_1714e(a1," | %s",*(unsigned long *)(v3 + 0x18));
    v3 = v4;
  }
  sub_1705d(10,a1);
  sub_170b1();
}

// Function: sub_f9ff @ 0xf9ff
void sub_f9ff(unsigned int a0,unsigned int a1)
{
  int v1;
  
  sub_16e27(0);
  v1 = tcsetpgrp(a0,a1);
  sub_d490();
  if (v1) {
    v1 = *dat_266e8;
    sub_6621("Cannot set tty process group (%s)",strerror(v1)); // no-return
  }
}

// Function: sub_fa62 @ 0xfa62
unsigned int sub_fa62(long a0)
{
  unsigned int v1;
  unsigned int v2; // stack - 0x10
  
  v1 = *(unsigned int *)((unsigned long)*(unsigned short *)(a0 + 0x1c) * 0x10 + *(long *)(a0 + 0x10) + -0xc);
  v2 = (int)v1 >> 8 & 0xff;
  if (v1 & 0x7f) {
    v2 = (int)v1 >> 8 & 0xff;
    if (((v1 & 0xff) != 0x7f) && (v2 = v1 & 0x7f, v2 == 2))
      *(unsigned char *)(a0 + 0x1f) = *(unsigned char *)(a0 + 0x1f) | 1;
    v2 += 0x80;
  }
  return v2;
}

// Function: sub_fae9 @ 0xfae9
void sub_fae9(void) // ternary
{
  int v1; // eax
  char *v2;
  char v3 [32];
  long v4; // stack - 0xe8
  char v5 [88];
  char *v6; // stack - 0xe0
  long *v7; // stack - 0xd8
  char *v8; // stack - 0xd0
  long v9; // stack - 0x50
  
  sub_105e6(v3);
  v4 = (dat_24168 & 0x20) ? dat_24150 + 5 : dat_24170 + 9; // branch-flip
  v7 = (long *)0x26680;
  while ((v7 < (long *)0x266d0 && (sub_8b10(&v4,0x269e0,2)))) {
    v8 = dat_24048;
    if (*dat_24048) {
      for (v6 = dat_24048; *v6; v6 = &v6[1]) {
      }
      v6[-1] = '\0';
      v1 = stat64(dat_24048,v5);
      if (0 <= v1) { // branch-flip
        if ((!dat_266d0) && (v9 != *v7)) {
          if (dat_24420) // branch-flip
            v2 = dat_24420;
          else {
            v2 = "you have mail";
          }
          sub_1714e(0x240a0,"%s\n",v2);
        }
        *v7 = v9;
      }
      else {
        *v7 = 0;
      }
    }
    v7 = &v7[1];
  }
  dat_266d0 = 0;
  sub_10641(v3);
}

// Function: sub_fccd @ 0xfccd
void sub_fccd(void)
{
  dat_266d0 += 1;
}

// Function: sub_fceb @ 0xfceb
void sub_fceb(unsigned long a0,unsigned long a1)
{
  sub_8b10(a0,a1,1);
}

// Function: main @ 0xfd15
int main(int argc,char **argv,char **envp)
{
  char *v1; // rax
  char v2 [24];
  long v3; // fs_offset
  unsigned long v4; // stack - 0x10
  
  v4 = *(unsigned long *)(v3 + 0x28);
  dat_266e8 = __errno_location();
  if (!_setjmp(0x26700)) {
    dat_243d8 = 0x26700;
    dat_266e0 = getpid();
    sub_1a81e();
    sub_105e6(v2);
    if (sub_119d6(argc,argv)) {
      sub_100c0("/etc/profile");
      sub_100c0("$HOME/.profile");
    }
    if (((dat_26a43) && (v1 = (char *)sub_19d32("ENV"), v1)) && (*v1))
      sub_100c0(v1);
    sub_10641(v2);
    if (dat_26a30)
      sub_6a59(dat_26a30,dat_26a46 == '\0');
    if ((dat_26a46) || (!dat_26a30))
      sub_ff38(1);
    sub_16c88(); // no-return
  }
  sub_1aa1a();
  sub_16c88(); // no-return
}

// Function: sub_ff38 @ 0xff38
unsigned int sub_ff38(int a0)
{
  unsigned int v1; // eax
  long v2; // rax
  char v3 [24];
  unsigned int v4; // stack - 0x44
  unsigned int v5; // stack - 0x40
  int v6; // stack - 0x3c
  
  v5 = 0;
  v6 = 0;
  do {
    sub_105e6(v3);
    if (dat_26650)
      sub_e140(dat_240d0,8);
    v4 = 0;
    if ((dat_26a43) && (a0)) {
      v4 = 1;
      sub_fae9();
    }
    v2 = sub_129c3(v4);
    if (v2 != 0x26a6c) { // branch-flip
      dat_26654 = (unsigned int)(dat_26654 == 2);
      v6 = 0;
      v1 = sub_6b48(v2,0);
      if (v2)
        v5 = v1;
    }
    else {
      if (!a0)
        return v5;
      if (0x31 < v6)
        return v5;
      if (!sub_f0f7()) {
        if (!dat_26a42) {
          if (!dat_26a43)
            return v5;
          sub_1705d(10,dat_240d0);
          return v5;
        }
        sub_1701f("\nUse \"exit\" to leave shell.\n",dat_240d0);
      }
      v6 += 1;
    }
    sub_10641(v3);
    if (dat_243f0) {
      dat_243f0 &= 0xfffffff3;
      return v5;
    }
  } while( true );
}

// Function: sub_100c0 @ 0x100c0
void sub_100c0(unsigned long a0) // return-dupe
{
  unsigned long v1; // rax
  
  v1 = sub_15816(a0);
  if ((int)sub_d12d(v1,3) <= -1)
    return;
  sub_ff38(0);
  sub_d363();
}

// Function: sub_10109 @ 0x10109
void sub_10109(unsigned long a0)
{
  sub_d12d(a0,1);
  sub_ff38(0);
  sub_d363();
}

// Function: sub_1013c @ 0x1013c
char * sub_1013c(char *a0)
{
  int v1; // eax
  int v2; // eax
  char v3 [24];
  long v4; // stack - 0xb8
  unsigned long v5; // stack - 0xb0
  unsigned int v6; // stack - 0x90
  
  v4 = dat_24190 + 5;
  if (!strchr(a0,0x2f)) {
    do {
      v2 = sub_fceb(&v4,a0);
      if (v2 <= -1)
        sub_6621("%s: not found",a0); // no-return
      v5 = dat_24048;
    } while ((((dat_24420) && (*dat_24420 != 'f')) || (v1 = stat64(dat_24048,v3), v1)) || ((v6 & 0xf000) != 0x8000));
    a0 = (char *)sub_10422((long)v2);
  }
  return a0;
}

// Function: sub_10265 @ 0x10265
unsigned int sub_10265(void)
{
  unsigned long v1; // rax
  unsigned int v2; // stack - 0x14
  
  v2 = 0;
  sub_12769(0x269e0);
  if (*dat_26a18) {
    v1 = sub_1013c(*dat_26a18);
    sub_d12d(v1,1);
    dat_243f8 = v1;
    v2 = sub_ff38(0);
    sub_d363();
  }
  return v2;
}

// Function: sub_102eb @ 0x102eb
unsigned long sub_102eb(int a0,long a1)
{
  if (sub_f0f7())
    return 0;
  if (2 <= a0)
    dat_24020 = sub_1174e(*(unsigned long *)(a1 + 8));
  sub_6486(4); // no-return
}

// Function: sub_10339 @ 0x10339
void sub_10339(unsigned long a0)
{
  sub_10422(a0);
}

// Function: sub_10354 @ 0x10354
void * sub_10354(unsigned long a0)
{
  void *v1; // rax
  
  v1 = malloc(a0);
  if (!v1)
    sub_6621("Out of space"); // no-return
  return v1;
}

// Function: sub_10395 @ 0x10395
void * sub_10395(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = realloc(a0,a1);
  if (!v1)
    sub_6621("Out of space"); // no-return
  return v1;
}

// Function: sub_103e1 @ 0x103e1
char * sub_103e1(char *a0)
{
  char *v1; // rax
  
  v1 = strdup(a0);
  if (!v1)
    sub_6621("Out of space"); // no-return
  return v1;
}

// Function: sub_10422 @ 0x10422
unsigned long * sub_10422(long a0)
{
  unsigned long v1; // rax
  unsigned long *v2;
  unsigned long v3; // stack - 0x30
  
  v1 = a0 + 7U & 0xfffffffffffffff8;
  if (dat_24038 < v1) {
    v3 = v1;
    if (v1 <= 0x1f7)
      v3 = 0x1f8;
    if (v3 + 8 < v3)
      sub_6621("Out of space"); // no-return
    dat_243e4 += 1;
    v2 = (unsigned long *)sub_10354(v3 + 8);
    *v2 = dat_24040;
    dat_24048 = &v2[1];
    dat_24038 = v3;
    dat_24050 = v3 + (long)dat_24048;
    dat_243e4 -= 1;
    dat_24040 = v2;
    if ((!dat_243e4) && (dat_243e8))
      sub_64d8(); // no-return
  }
  v2 = dat_24048;
  dat_24048 = (unsigned long *)(v1 + (long)dat_24048);
  dat_24038 -= v1;
  return v2;
}

// Function: sub_1055e @ 0x1055e
void sub_1055e(long a0)
{
  dat_24038 += dat_24048 - a0;
  dat_24048 = a0;
}

// Function: sub_10597 @ 0x10597
void sub_10597(unsigned long *a0,unsigned long a1)
{
  *a0 = dat_24040;
  a0[1] = dat_24048;
  a0[2] = dat_24038;
  sub_10339(a1);
}

// Function: sub_105e6 @ 0x105e6
void sub_105e6(unsigned long a0) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_105e6
{
  unsigned int v1; // eax
  
  v1 = !((dat_24040 + 8 != dat_24048) || (dat_24040 == 0x267e0));
  sub_10597(a0,v1);
}

// Function: sub_10641 @ 0x10641
void sub_10641(unsigned long *a0)
{
  unsigned long *v1;
  
  dat_243e4 += 1;
  while (v1 = dat_24040, (unsigned long *)*a0 != dat_24040) {
    dat_24040 = (unsigned long *)*dat_24040;
    free(v1);
  }
  dat_24048 = a0[1];
  dat_24038 = a0[2];
  dat_24050 = a0[2] + a0[1];
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
}

// Function: sub_106fd @ 0x106fd
void sub_106fd(unsigned long a0)
{
  unsigned long v1;
  unsigned long *v2;
  int v3; // eax
  unsigned long v4; // rax
  unsigned long v5; // stack - 0x38
  
  v2 = dat_24048;
  v5 = dat_24038 * 2;
  if (v5 < dat_24038)
    sub_6621("Out of space"); // no-return
  v4 = (a0 | 0x80) + 7 & 0xfffffffffffffff8;
  if (v5 < v4)
    v5 += v4;
  if ((&dat_24040[1] != dat_24048) || (dat_24040 == (unsigned long *)0x267e0)) {
    v3 = (int)dat_24038;
    dat_24048 = memcpy((void *)sub_10422(v5),v2,(long)v3);
    dat_24038 = v5 + dat_24038;
  }
  else {
    dat_243e4 += 1;
    v1 = *dat_24040;
    dat_24040 = (unsigned long *)sub_10395(dat_24040,v5 + 8);
    *dat_24040 = v1;
    dat_24048 = &dat_24040[1];
    dat_24038 = v5;
    dat_24050 = (long)dat_24040 + v5 + 8;
    dat_243e4 -= 1;
    if ((!dat_243e4) && (dat_243e8))
      sub_64d8(); // no-return
  }
}

// Function: sub_108a8 @ 0x108a8
long sub_108a8(void)
{
  long v1;
  
  v1 = dat_24038;
  sub_106fd(0);
  return v1 + dat_24048;
}

// Function: sub_108d9 @ 0x108d9
unsigned long sub_108d9(unsigned long a0)
{
  if (dat_24038 < a0)
    sub_106fd(a0);
  return dat_24048;
}

// Function: sub_1090b @ 0x1090b
long sub_1090b(long a0,long a1)
{
  a1 -= dat_24048;
  return sub_108d9(a0 + a1) + a1;
}

// Function: sub_1094d @ 0x1094d
void * sub_1094d(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)sub_1090b(a1,a2);
  return mempcpy(v1,a0,a1);
}

// Function: sub_1099d @ 0x1099d
void sub_1099d(char *a0,unsigned long a1)
{
  sub_1094d(a0,strlen(a0),a1);
}

// Function: sub_109d5 @ 0x109d5
void sub_109d5(long a0,unsigned int a1,long *a2)
{
  unsigned long *v1; // stack - 0x28
  long *v2; // stack - 0x50
  unsigned long *v3; // stack - 0x30
  unsigned long *v4; // stack - 0x20
  
  a0 -= dat_24048;
  v4 = &v1;
  sub_b7cc(sub_10422(a0),a1,&v1);
  *v4 = 0;
  sub_baa8();
  v3 = v1;
  v2 = a2;
  do {
    if (!v3) {
      do {
        sub_1985b(*v2,0x269e0,0);
        v2 = &v2[1];
      } while (*v2);
      return;
    }
    sub_c50c(v3[1],0);
    sub_1985b(*v2,v3[1],0);
    v3 = (unsigned long *)*v3;
    v2 = &v2[1];
  } while (*v2);
}

// Function: sub_10aec @ 0x10aec
unsigned int sub_10aec(int a0,long a1)
{
  char *v1;
  long v10; // stack - 0x40
  char *v11; // stack - 0x38
  long *v12; // stack - 0x30
  char *v13; // stack - 0x28
  unsigned long v14; // stack - 0x20
  unsigned long v15; // stack - 0x18
  int v2; // eax
  long v3; // rax
  char v4; // stack - 0x55
  int v5; // stack - 0x54
  int v6; // stack - 0x50
  unsigned int v7; // stack - 0x4c
  int v8; // stack - 0x48
  int v9; // stack - 0x44
  
  v5 = 0;
  v10 = 0;
  while( true ) {
    v8 = sub_12769("p:r");
    if (!v8) break;
    if (v8 != 0x70) // branch-flip
      v5 = 1;
    else {
      v10 = dat_26a20;
    }
  }
  if ((v10) && (isatty(0)))
    sub_1701f(v10,dat_240d0);
  v12 = dat_26a18;
  if (!*dat_26a18)
    sub_6621("arg count"); // no-return
  v7 = 0;
  v11 = dat_24048;
  do {
    v9 = (int)v11 - (int)dat_24048;
    v6 = v9 + -1;
label_10cf9:
    do {
      do {
        v3 = read(0,&v4,1);
        if (!v3) {
label_10be1:
          v7 = 1;
label_10d1e:
          sub_b70b(v9,(int)v11 - (int)dat_24048,0);
          if (v11 != dat_24050) // branch-flip
            *v11 = '\0';
          else {
            v11 = (char *)sub_108a8();
            *v11 = '\0';
          }
          sub_109d5(&v11[1],a0 - (int)((long)v12 - a1 >> 3),v12);
          return v7;
        }
        if (v3 != 1) {
          if ((*dat_266e8 != 4) || (dat_26d20)) goto label_10be1;
          goto label_10cf9;
        }
      } while (!v4);
      if (v9 <= v6) { // branch-flip
        if (v4 == '\n') break;
      }
      else {
        if ((!v5) && (v4 == '\\')) {
          v6 = (int)v11 - (int)dat_24048;
          goto label_10cf9;
        }
        if (v4 == '\n') goto label_10d1e;
      }
      v13 = v11;
      v14 = 2;
      v15 = (long)dat_24050 - (long)v11;
      if (v15 < 2)
        v11 = (char *)sub_1090b(2,v11);
      v2 = (int)v4;
      if (strchr((char *)0x1cc0d,v2)) {
        v1 = &v11[1];
        *v11 = '\x81';
        v11 = v1;
      }
      v1 = &v11[1];
      *v11 = v4;
      v11 = v1;
    } while (v6 < v9);
    sub_b70b(v9,v6,0);
  } while( true );
}

// Function: sub_10dad @ 0x10dad
unsigned long sub_10dad(void) // return-dupe x2
{
  char *v1;
  int v10; // stack - 0x44
  unsigned int v11; // stack - 0x40
  unsigned int v12; // stack - 0x3c
  unsigned int v13; // stack - 0x38
  unsigned char *v14; // stack - 0x30
  unsigned char *v2;
  unsigned char v3;
  unsigned char v4;
  bool v5;
  unsigned int v6;
  unsigned int v7; // eax
  char v8 [2];
  int v9; // stack - 0x4c
  
  v5 = 0;
  while (sub_12769("S")) {
    v5 = 1;
  }
  dat_243e4 += 1;
  v6 = umask(0);
  umask(v6);
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
  v14 = (unsigned char *)*dat_26a18;
  if (!v14) {
    if (v5) {
      v9 = 0;
      v14 = (unsigned char *)v8;
      while (v9 <= 2) {
        v1 = (char *)&v14[1];
        *v14 = "ugo"[v9];
        v14 = &v14[2];
        *v1 = '=';
        for (v10 = 0; v10 <= 2; v10 = v10 + 1) {
          if ((int)~v6 >> (((char)v9 * '\xfd' - (char)v10) + 8U & 0x1f) & 1U) {
            v1 = (char *)&v14[1];
            *v14 = "rwx"[v10];
            v14 = (unsigned char *)v1;
          }
        }
        v1 = (char *)&v14[1];
        *v14 = ',';
        v9 += 1;
        v14 = (unsigned char *)v1;
      }
      v14[-1] = '\0';
      sub_17224("%s\n",v8);
      return 0;
    }
    sub_17224("%.4o\n",v6);
    return 0;
  }
  if (*(unsigned short *)((unsigned long)*v14 * 2 + *(long *)__ctype_b_loc()) & 0x800) { // branch-flip
    v11 = 0;
    do {
      if (('7' < (char)*v14) || ((char)*v14 <= '/'))
        sub_6621("Illegal number: %s",*dat_26a18); // no-return
      v11 = (char)*v14 + -0x30 + v11 * 8;
      v14 = &v14[1];
    } while (*v14);
  }
  else {
    v6 = ~v6;
    v12 = 0;
    v11 = v6;
    do {
      while( true ) {
        if (!*v14) goto label_11217;
        while ((*v14 && (v3 = *v14, strchr("augo",(int)(char)v3)))) {
          v2 = &v14[1];
          v3 = *v14;
          v14 = v2;
          if (v3 != 0x75) { // branch-flip
            if ((char)v3 <= 'u') {
              if (v3 != 0x6f) { // branch-flip
                if ((char)v3 <= 'o') {
                  if (v3 != 0x61) { // branch-flip
                    if (v3 == 0x67)
                      v12 |= 8;
                  }
                  else {
                    v12 = 0x49;
                  }
                }
              }
              else {
                v12 |= 1;
              }
            }
          }
          else {
            v12 |= 0x40;
          }
        }
        if (!v12)
          v12 = 0x49;
        v3 = *v14;
        if (!strchr("=+-",(int)(char)v3)) goto label_11217;
        v14 = &v14[1];
        v13 = 0;
        while ((*v14 && (v4 = *v14, strchr("rwxugoXs",(int)(char)v4)))) {
          v2 = &v14[1];
          v4 = *v14;
          v14 = v2;
          switch(v4) {
            case 0x58:
              if (v6 & 0x49)
                v13 |= 1;
              break;
            case 0x67:
              v13 |= (int)v6 >> 3;
              break;
            case 0x6f:
              v13 |= v6;
              break;
            case 0x72:
              v13 |= 4;
              break;
            case 0x75:
              v13 |= (int)v6 >> 6;
              break;
            case 0x77:
              v13 |= 2;
              break;
            case 0x78:
              v13 |= 1;
            
          }
        }
        v7 = v12 * (v13 & 7);
        if (v3 != 0x3d) { // branch-flip
          if ((char)v3 <= '=') {
            if (v3 != 0x2b) { // branch-flip
              if (v3 == 0x2d)
                v11 &= ~v7;
            }
            else {
              v11 |= v7;
            }
          }
        }
        else {
          v11 = ~(v12 * 7) & v11 | v7;
        }
        if (*v14 != 0x2c) break;
        v12 = 0;
        v14 = &v14[1];
      }
      v3 = *v14;
    } while (strchr("=+-",(int)(char)v3));
label_11217:
    if (*v14)
      sub_6621("Illegal mode: %s",*dat_26a18); // no-return
    v11 = ~v11;
  }
  umask(v11);
  return 0;
}

// Function: sub_1126b @ 0x1126b
void sub_1126b(unsigned int a0,unsigned long *a1,long a2) // return-dupe
{
  unsigned long v1; // stack - 0x10
  
  v1 = a1[1];
  if (a0 & 1)
    v1 = *a1;
  if (v1 == 0xffffffffffffffff) {
    sub_17224("unlimited\n");
    return;
  }
  sub_17224("%ld\n",v1 / (unsigned long)(long)*(int *)(a2 + 0xc),v1 % (unsigned long)(long)*(int *)(a2 + 0xc));
}

// Function: sub_112f8 @ 0x112f8
unsigned long sub_112f8(void) // return-dupe x2
{
  char *v1;
  long *v10; // stack - 0x38
  char *v11; // stack - 0x30
  long v12; // stack - 0x20
  unsigned int v2;
  long v3;
  bool v4;
  int v5;
  long v6; // stack - 0x28
  unsigned int v7; // stack - 0x58
  int v8; // stack - 0x50
  long v9; // stack - 0x40
  
  v9 = 0;
  v7 = 3;
  v4 = 0;
  v8 = 0x66;
  while (v5 = sub_12769("HSatfdscmlpnvw"), v5) {
    switch(v5) {
      default:
        v8 = v5;
        break;
      case 0x48:
        v7 = 2;
        break;
      case 0x53:
        v7 = 1;
        break;
      case 0x61:
        v4 = 1;
      
    }
  }
  for (v10 = (long *)0x22f60; v8 != *(char *)((long)v10 + 0x10); v10 = (long)((long)v10 + 0x18)) {
  }
  v3 = *dat_26a18;
  if (v3) {
    v11 = (char *)*dat_26a18;
    if ((v4) || (dat_26a18[1]))
      sub_6621("too many arguments"); // no-return
    if (strcmp(v11,"unlimited")) { // branch-flip
      v9 = 0;
      while( true ) {
        v1 = &v11[1];
        v5 = (int)*v11;
        if ((v5 <= 0x2f) || (0x3a <= v5)) break;
        v9 = (long)(v5 + -0x30) + v9 * 10;
        v11 = v1;
      }
      if (v5)
        sub_6621("bad number"); // no-return
      v9 = *(int *)((long)v10 + 0xc) * v9;
    }
    else {
      v9 = -1;
    }
  }
  if (v4) {
    for (v10 = (long *)0x22f60; *v10; v10 = &v10[3]) {
      getrlimit((int)v10[1],&v6);
      sub_17224("%-20s ",*v10);
      sub_1126b(v7,&v6,v10);
    }
    return 0;
  }
  getrlimit(*(unsigned int *)((long)v10 + 8),&v6);
  if (v3) {
    if (v7 & 2)
      v12 = v9;
    if (v7 & 1)
      v6 = v9;
    v2 = *(unsigned int *)((long)v10 + 8);
    if (0 <= (int)setrlimit(v2,&v6))
      return 0;
    v5 = *dat_266e8;
    sub_6621("error setting limit (%s)",strerror(v5)); // no-return
  }
  sub_1126b(v7,&v6,v10);
  return 0;
}

// Function: sub_115d5 @ 0x115d5
char * sub_115d5(char *a0,char *a1)
{
  char *v1;
  char *v2;
  char v3;
  char v4;
  char *v5; // stack - 0x18
  char *v6; // stack - 0x10
  
  v5 = a1;
  v6 = a0;
  do {
    if (!*v5)
      return v6;
    v1 = &v5[1];
    v3 = *v5;
    v2 = &v6[1];
    v4 = *v6;
    v5 = v1;
    v6 = v2;
  } while (v3 == v4);
  return NULL;
}

// Function: sub_11621 @ 0x11621
void sub_11621(unsigned long a0)
{
  sub_6621("Illegal number: %s",a0); // no-return
}

// Function: sub_1164c @ 0x1164c
unsigned long sub_1164c(unsigned char *a0,int a1)
{
  unsigned char *v1; // stack - 0x20
  unsigned long v2; // stack - 0x18
  
  *__errno_location() = 0;
  v2 = strtoimax(a0,&v1,a1);
  if (*__errno_location() == 0x22)
    sub_11621(a0); // no-return
  if ((a0 == v1) && (a1))
    sub_11621(a0); // no-return
  while (*(unsigned short *)((unsigned long)*v1 * 2 + *(long *)__ctype_b_loc()) & 0x2000) {
    v1 = &v1[1];
  }
  if (!*v1)
    return v2;
  sub_11621(a0); // no-return
}

// Function: sub_1172b @ 0x1172b
void sub_1172b(unsigned long a0)
{
  sub_1164c(a0,10);
}

// Function: sub_1174e @ 0x1174e
long sub_1174e(unsigned long a0)
{
  long v1; // rax
  
  v1 = sub_1172b(a0);
  if ((0 <= v1) && (v1 < 0x80000000))
    return v1;
  sub_11621(a0); // no-return
}

// Function: sub_11792 @ 0x11792
unsigned long sub_11792(char *a0)
{
  char *v1; // stack - 0x10
  
  v1 = a0;
  do {
    if (10 <= (unsigned int)((int)*v1 - 0x30U))
      return 0;
    v1 = &v1[1];
  } while (*v1);
  return 1;
}

// Function: sub_117ce @ 0x117ce
char * sub_117ce(char *a0)
{
  unsigned long v1; // rax
  char *v2; // rax
  char *v3; // stack - 0x30
  char *v4; // stack - 0x20
  
  v4 = dat_24048;
  v3 = a0;
  do {
    v1 = strchrnul(v3,0x27) - (long)v3;
    v2 = (char *)sub_1090b(v1 + 3,v4);
    *v2 = 0x27;
    v2 = mempcpy(&v2[1],v3,v1);
    v4 = &v2[1];
    *v2 = 0x27;
    v3 = &v3[v1];
    v1 = strspn(v3,"\'");
    if (!v1) break;
    v2 = (char *)sub_1090b(v1 + 3,v4);
    *v2 = 0x22;
    v2 = mempcpy(&v2[1],v3,v1);
    v4 = &v2[1];
    *v2 = 0x22;
    v3 = &v3[v1];
  } while (*v3);
  *v4 = 0;
  return dat_24048;
}

// Function: sub_11925 @ 0x11925
void sub_11925(char *a0)
{
  unsigned long v1; // rax
  
  v1 = strlen(a0) + 1;
  memcpy((void *)sub_10422(v1),a0,v1);
}

// Function: sub_1196d @ 0x1196d
void sub_1196d(unsigned long *a0,unsigned long *a1)
{
  strcmp((char *)*a0,(char *)*a1);
}

// Function: sub_1199c @ 0x1199c
void sub_1199c(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x10
  
  v1 = a0;
  bsearch(&v1,a1,a2,8,sub_1196d);
}

// Function: sub_119d6 @ 0x119d6
unsigned int sub_119d6(int a0,long *a1) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_119d6
{
  long *v1;
  unsigned int v2;
  long v3;
  unsigned int v4; // eax
  long v5; // rax
  int v6; // stack - 0x20
  long *v7; // stack - 0x18
  
  v4 = ((*a1) && (*(char *)*a1 == '-'));
  v2 = v4;
  dat_269f0 = *a1;
  v7 = a1;
  if (1 <= a0)
    v7 = &a1[1];
  for (v6 = 0; v6 <= 0x10; v6 = v6 + 1) {
    v5 = (long)v6;
    *(char *)(v5 + 0x26a40) = 2;
  }
  dat_26a18 = v7;
  v4 = sub_11c12(1);
  v3 = dat_26a30;
  v1 = dat_26a18;
  v2 |= v4;
  v7 = dat_26a18;
  if (!*dat_26a18) {
    if (dat_26a30)
      sub_6621("-c requires an argument"); // no-return
    dat_26a46 = '\x01';
  }
  if ((((dat_26a43 == '\x02') && (dat_26a46 == '\x01')) && (v4 = isatty(0), v4)) && (v4 = isatty(1), v4))
    dat_26a43 = '\x01';
  if (dat_26a44 == '\x02')
    dat_26a44 = dat_26a43;
  for (v6 = 0; v6 <= 0x10; v6 = v6 + 1) {
    v5 = (long)v6;
    if (*(char *)(v5 + 0x26a40) == '\x02') {
      v5 = (long)v6;
      *(char *)(v5 + 0x26a40) = 0;
    }
  }
  if (v3) { // branch-flip
    v7 = &v1[1];
    dat_26a30 = *v1;
    if (!*v7) goto label_11b9a;
  }
  else {
    if (dat_26a46) goto label_11b9a;
    sub_d12d(*v1,0);
  }
  v1 = &v7[1];
  dat_269f0 = *v7;
  v7 = v1;
label_11b9a:
  dat_26a08 = v7;
  dat_26a10 = 1;
  dat_26a14 = 0xffffffff;
  while (*v7) {
    v4 = dat_26a00 + 1;
    v7 = &v7[1];
    dat_26a00 = v4;
  }
  sub_11be5();
  v4 = v2;
  return v4;
}

// Function: sub_11be5 @ 0x11be5
void sub_11be5(void)
{
  sub_16c3e((int)dat_26a43);
  sub_d556((int)dat_26a44);
}

// Function: sub_11c12 @ 0x11c12
unsigned int sub_11c12(int a0)
{
  char *v1;
  int v2; // eax
  long *v3; // rax
  unsigned int v4; // stack - 0x1c
  unsigned int v5; // stack - 0x18
  char *v6; // stack - 0x10
  
  v5 = 0;
  if (a0)
    dat_26a30 = NULL;
  do {
    v1 = (char *)*dat_26a18;
    if (!v1)
      return v5;
    v3 = &dat_26a18[1];
    v6 = &v1[1];
    if (*v1 != '-') { // branch-flip
      if (*v1 != '+')
        return v5;
      v4 = 0;
      dat_26a18 = v3;
    }
    else {
      v4 = 1;
      if ((!*v6) || ((dat_26a18 = v3, *v6 == '-' && (!v1[2])))) {
        if (a0) {
          dat_26a18 = v3;
          return v5;
        }
        if (!*v6) {
          dat_26a18 = v3;
          dat_26a47 = 0;
          dat_26a48 = 0;
          return v5;
        }
        if (*v3) {
          dat_26a18 = v3;
          return v5;
        }
        dat_26a18 = v3;
        sub_11ffc(v3);
        return v5;
      }
    }
    while( true ) {
      v1 = &v6[1];
      v2 = (int)*v6;
      if (!v2) break;
      v6 = v1;
      if ((v2 != 99) || (!a0)) {
        if ((v2 != 0x6c) || (!a0)) {
          if (v2 != 0x6f) // branch-flip
            sub_11f6b(v2,v4);
          else {
            sub_11de2(*dat_26a18,v4);
            if (*dat_26a18)
              dat_26a18 = &dat_26a18[1];
          }
        }
        else {
          v5 = 1;
        }
      }
      else {
        dat_26a30 = v1;
      }
    }
  } while( true );
}

// Function: sub_11de2 @ 0x11de2
void sub_11de2(char *a0,int a1) // return-dupe x2, ternary x2
{
  char *v1;
  int v2; // stack - 0xc
  
  if (a0) {
    v2 = 0;
    while( true ) {
      if (0x11 <= v2)
        sub_6621("Illegal option -o %s",a0); // no-return
      v1 = *(char **)((long)v2 * 8 + 0x230a0);
      if (!strcmp(a0,v1)) break;
      v2 += 1;
    }
    *(char *)((long)v2 + 0x26a40) = (char)a1;
    return;
  }
  if (a1) {
    sub_1701f("Current option settings\n",dat_240c8);
    for (v2 = 0; v2 <= 0x10; v2 = v2 + 1) {
      v1 = (*(char *)((long)v2 + 0x26a40)) ? "on" : "off"; // branch-flip
      sub_17224("%-16s%s\n",*(unsigned long *)((long)v2 * 8 + 0x230a0),v1);
    }
    return;
  }
  for (v2 = 0; v2 <= 0x10; v2 = v2 + 1) {
    v1 = (*(char *)((long)v2 + 0x26a40)) ? "-o" : "+o"; // branch-flip
    sub_17224("set %s %s\n",v1,*(unsigned long *)((long)v2 * 8 + 0x230a0));
  }
}

// Function: sub_11f6b @ 0x11f6b
void sub_11f6b(int a0,int a1) // return-dupe x3
{
  int v1; // stack - 0xc
  
  v1 = 0;
  while( true ) {
    if (0x11 <= v1)
      sub_6621("Illegal option -%c",a0); // no-return
    if (a0 == "efIimnsxvVECabu"[v1]) break;
    v1 += 1;
  }
  *(char *)((long)v1 + 0x26a40) = (char)a1;
  if (!a1)
    return;
  if (a0 == 0x56) {
    dat_26a4a = 0;
    return;
  }
  if (a0 != 0x45)
    return;
  dat_26a49 = 0;
}

// Function: sub_11ffc @ 0x11ffc
void sub_11ffc(long *a0)
{
  long *v1;
  unsigned long *v2;
  long v3;
  unsigned long *v4; // rax
  long *v5; // stack - 0x40
  int v6; // stack - 0x2c
  unsigned long *v7; // stack - 0x28
  
  for (v6 = 0; a0[v6]; v6 = v6 + 1) {
  }
  v4 = (unsigned long *)sub_10354((long)(v6 + 1) << 3);
  v7 = v4;
  v5 = a0;
  while (*v5) {
    v1 = &v5[1];
    v3 = *v5;
    v2 = &v7[1];
    *v7 = sub_103e1(v3);
    v5 = v1;
    v7 = v2;
  }
  *v7 = 0;
  sub_120da(0x26a00);
  dat_26a04 = 1;
  dat_26a00 = v6;
  dat_26a08 = v4;
  dat_26a10 = 1;
  dat_26a14 = 0xffffffff;
}

// Function: sub_120da @ 0x120da
void sub_120da(long a0) // return-dupe
{
  long *v1; // stack - 0x10
  
  if (!*(char *)(a0 + 4))
    return;
  for (v1 = *(long **)(a0 + 8); *v1; v1 = &v1[1]) {
    free((void *)*v1);
  }
  free(*(void **)(a0 + 8));
}

// Function: sub_12137 @ 0x12137
unsigned long sub_12137(int a0,long a1)
{
  long *v1;
  long *v2;
  unsigned long v3;
  long v4;
  int v5; // stack - 0x1c
  long *v6; // stack - 0x18
  long *v7; // stack - 0x10
  
  v5 = 1;
  if (2 <= a0) {
    v3 = *(unsigned long *)(a1 + 8);
    v5 = sub_1174e(v3);
  }
  if (dat_26a00 < v5)
    sub_6621("can\'t shift that many"); // no-return
  dat_243e4 += 1;
  dat_26a00 -= v5;
  v6 = dat_26a08;
  while (v5 = v5 + -1, 0 <= v5) {
    if (dat_26a04)
      free((void *)*v6);
    v6 = &v6[1];
  }
  v7 = dat_26a08;
  do {
    v1 = &v6[1];
    v2 = &v7[1];
    *v7 = *v6;
    v4 = *v7;
    v6 = v1;
    v7 = v2;
  } while (v4);
  dat_26a10 = 1;
  dat_26a14 = 0xffffffff;
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
  return 0;
}

// Function: sub_12255 @ 0x12255
unsigned long sub_12255(int a0) // early-return
{
  unsigned long v1; // rax
  
  if (a0 == 1) {
    v1 = sub_19f13(0x269e0,0,0x20);
    return v1;
  }
  dat_243e4 += 1;
  sub_11c12(0);
  sub_11be5();
  if (*dat_26a18)
    sub_11ffc(dat_26a18);
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
  return 0;
}

// Function: sub_122f4 @ 0x122f4
void sub_122f4(unsigned long a0) // return-dupe
{
  dat_26a10 = sub_1174e(a0);
  if (dat_26a10) {
    dat_26a14 = 0xffffffff;
    return;
  }
  dat_26a10 = 1;
  dat_26a14 = 0xffffffff;
}

// Function: sub_1232c @ 0x1232c
void sub_1232c(int a0,long a1)
{
  long v1; // stack - 0x10
  
  if (a0 <= 2)
    sub_6621("Usage: getopts optstring var [arg]"); // no-return
  if (a0 != 3) { // branch-flip
    v1 = a1 + 0x18;
    if ((unsigned int)(a0 - 2U) < dat_26a10) {
      dat_26a10 = 1;
      dat_26a14 = 0xffffffff;
    }
  }
  else {
    v1 = dat_26a08;
    if (dat_26a00 + 1U < dat_26a10) {
      dat_26a10 = 1;
      dat_26a14 = 0xffffffff;
    }
  }
  sub_123ee(*(unsigned long *)(a1 + 8),*(unsigned long *)(a1 + 0x10),v1);
}

// Function: sub_123ee @ 0x123ee
unsigned int sub_123ee(char *a0,unsigned long a1,long a2) // return-dupe
{
  char *v1;
  unsigned long *v10; // stack - 0x20
  int v2;
  char *v3;
  int v4;
  char v5 [2]; // stack - 0x12
  char v6; // stack - 0x3d
  unsigned int v7; // stack - 0x3c
  char *v8; // stack - 0x30
  char *v9; // stack - 0x28
  
  v2 = dat_26a14;
  v4 = dat_26a10;
  v6 = '?';
  v7 = 0;
  dat_26a10 = 0xffffffff;
  v10 = (unsigned long *)(a2 + (long)v4 * 8 + -8);
  if (((2 <= v4) && (0 <= dat_26a14)) && (v1 = (char *)v10[-1], (unsigned long)(long)v2 <= strlen(v1)))
    v8 = (char *)((long)v2 + v10[-1]);
  else {
    v8 = NULL;
  }
  if (((v8) && (*v8)) || (((v1 = (char *)*v10, v1 && ((*v1 == '-' && (v8 = &v1[1], *v8)))) && ((v10 = &v10[1], *v8 != '-' || (v1[2])))))) {
    v1 = &v8[1];
    v6 = *v8;
    v3 = a0;
    while (v9 = v3, v8 = v1, v6 != *v9) {
      if (!*v9) {
        if (*a0 != ':') { // branch-flip
          sub_1714e(0x240a0,"Illegal option -%c\n",(int)v6);
          sub_1a67a("OPTARG");
        }
        else {
          v5[1] = 0;
          v5[0] = v6;
          sub_1985b("OPTARG",v5,0);
        }
        v6 = '?';
        goto label_126d0;
      }
      v3 = &v9[1];
      if (v9[1] == ':')
        v3 = &v9[2];
    }
    if (v9[1] != ':') // branch-flip
      sub_1985b("OPTARG",0x269e0,0);
    else if ((*v1) || (v8 = (char *)*v10, v8)) {
      if (v8 == (char *)*v10)
        v10 = &v10[1];
      sub_1985b("OPTARG",v8,0);
      v8 = NULL;
    }
    else {
      v8 = NULL;
      if (*a0 != ':') { // branch-flip
        sub_1714e(0x240a0,"No arg for -%c option\n",(int)v6);
        sub_1a67a("OPTARG");
        v6 = '?';
      }
      else {
        v5[1] = 0;
        v5[0] = v6;
        sub_1985b("OPTARG",v5,0);
        v6 = ':';
      }
    }
  }
  else {
    v8 = NULL;
    v7 = 1;
  }
label_126d0:
  v4 = (int)((long)v10 - a2 >> 3) + 1;
  sub_199bd("OPTIND",(long)v4,0x40);
  v5[0] = v6;
  v5[1] = 0;
  sub_1985b(a1,v5,0);
  if (v8) {
    dat_26a14 = (int)v8 - (int)v10[-1];
    dat_26a10 = v4;
    return v7;
  }
  dat_26a10 = v4;
  dat_26a14 = 0xffffffff;
  return v7;
}

// Function: sub_12769 @ 0x12769
int sub_12769(char *a0)
{
  unsigned long *v1;
  char v2;
  char *v3;
  char *v4;
  char *v5; // stack - 0x18
  char *v6; // stack - 0x10
  
  v5 = dat_26a28;
  if ((!dat_26a28) || (!*dat_26a28)) {
    v3 = (char *)*dat_26a18;
    if ((!v3) || ((*v3 != '-' || (v5 = &v3[1], !*v5))))
      return 0;
    dat_26a18 = &dat_26a18[1];
    if ((*v5 == '-') && (!v3[2]))
      return 0;
  }
  v3 = &v5[1];
  v2 = *v5;
  v4 = a0;
  while (v6 = v4, v2 != *v6) {
    if (!*v6)
      sub_6621("Illegal option -%c",(int)v2); // no-return
    v4 = &v6[1];
    if (v6[1] == ':')
      v4 = &v6[2];
  }
  v5 = v3;
  if (v6[1] == ':') {
    if (!*v3) {
      v1 = &dat_26a18[1];
      v5 = (char *)*dat_26a18;
      dat_26a18 = v1;
      if (!v5)
        sub_6621("No arg for -%c option",(int)v2); // no-return
    }
    dat_26a20 = v5;
    v5 = NULL;
  }
  dat_26a28 = v5;
  return (int)v2;
}

// Function: sub_128e1 @ 0x128e1
bool sub_128e1(unsigned long a0)
{
  return *(char *)sub_155e7(a0) == '\0';
}

// Function: sub_12906 @ 0x12906
void sub_12906(unsigned long a0)
{
  sub_10422(a0);
}

// Function: sub_12921 @ 0x12921
char * sub_12921(char a0,char *a1)
{
  char *v1; // stack - 0x18
  
  v1 = a1;
  if (a1 == dat_24050)
    v1 = (char *)sub_108a8();
  *v1 = a0;
  return &v1[1];
}

// Function: sub_1295d @ 0x1295d
bool sub_1295d(char *a0) // early-return
{
  char *v1; // rax
  
  v1 = (char *)sub_155e7(a0);
  if (a0 == v1)
    return 0;
  return *v1 == '=';
}

// Function: sub_1299f @ 0x1299f
unsigned long sub_1299f(long a0) // return-dupe
{
  if ((a0) && (a0 != 1))
    return 1;
  return 0;
}

// Function: sub_129c3 @ 0x129c3
void sub_129c3(int a0)
{
  dat_26a6c = 0;
  dat_26a78 = 0;
  dat_26a58 = 0;
  dat_26a60 = a0;
  if (a0)
    sub_1577e(a0);
  dat_26a64 = 0;
  sub_12a27(1);
}

// Function: sub_12a27 @ 0x12a27
int * sub_12a27(unsigned int a0) // ternary
{
  unsigned int v1; // eax
  int v2; // eax
  int *v3; // rax
  unsigned int v4; // stack - 0x2c
  int *v5; // stack - 0x20
  int *v6; // stack - 0x18
  
  v1 = (a0 & 1) ? 0 : 4; // branch-flip
  v5 = NULL;
  v4 = a0;
  while( true ) {
    dat_26a78 = v1 | 3;
    v2 = sub_13b02();
    if (!v2) break;
    if (v2 == 1) {
      sub_139df();
      return v5;
    }
    dat_26a6c += 1;
    if ((v4 == 2) && (*(char *)((long)v2 + 0x1cde0)))
      return v5;
    v4 |= 2;
    v6 = (int *)sub_12c15();
    v2 = sub_13b02();
    if (v2 == 3) {
      if (*v6 != 1) { // branch-flip
        if (*v6 != 2) {
          v3 = (int *)sub_10422(0x18);
          *(int **)&v3[2] = v6;
          v3[4] = 0;
          v3[5] = 0;
          v6 = v3;
        }
        *v6 = 3;
      }
      else {
        v6[1] = 1;
      }
    }
    if (v5) { // branch-flip
      v3 = (int *)sub_10422(0x18);
      *v3 = 7;
      *(int **)&v3[2] = v5;
      *(int **)&v3[4] = v6;
      v5 = v3;
    }
    else {
      v5 = v6;
    }
    if (4 <= v2) goto label_12be8;
    if (v2 <= 1) {
      if (!v2) {
        sub_139df(); // return-dupe
        dat_26a68 = 0;
        dat_26a6c += 1;
        return v5;
      }
      if (v2 != 1) {
label_12be8:
        if (v1) {
          dat_26a6c += 1;
          return v5;
        }
        sub_15680(0xffffffff); // no-return
      }
      dat_26a6c += 1;
    }
  }
  if ((!v5) && (!v1))
    v5 = (int *)0x26a6c;
  sub_139df();
  dat_26a68 = 0;
  dat_26a6c += 1;
  return v5;
}

// Function: sub_12c15 @ 0x12c15
unsigned int * sub_12c15(void)
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned int *v3; // rax
  unsigned int v4; // stack - 0x24
  unsigned int *v5; // stack - 0x20
  
  v5 = (unsigned int *)sub_12cb6();
  do {
    v1 = sub_13b02();
    if (v1 != 4) { // branch-flip
      if (v1 != 5) {
        dat_26a6c += 1;
        return v5;
      }
      v4 = 6;
    }
    else {
      v4 = 5;
    }
    dat_26a78 = 7;
    v2 = sub_12cb6();
    v3 = (unsigned int *)sub_10422(0x18);
    *v3 = v4;
    *(unsigned int **)&v3[2] = v5;
    *(unsigned long *)&v3[4] = v2;
    v5 = v3;
  } while( true );
}

// Function: sub_12cb6 @ 0x12cb6
unsigned long * sub_12cb6(void)
{
  unsigned int *v1;
  unsigned long *v2;
  unsigned long *v3;
  int v4; // eax
  unsigned long *v5; // rax
  bool v6; // zf
  unsigned int *v7; // stack - 0x30
  unsigned long *v8; // stack - 0x28
  
  v4 = sub_13b02();
  v6 = v4 != 0xd;
  if (v6) {
    v4 = dat_26a6c + 1;
    dat_26a6c = v4;
  }
  else {
    dat_26a78 = 3;
  }
  v5 = (unsigned int *)sub_12df5();
  v1 = (unsigned int *)v5;
  v7 = (unsigned int *)v5;
  v4 = sub_13b02();
  if (v4 == 6) {
    v5 = (unsigned int *)sub_10422(0x10);
    v7 = (unsigned int *)v5;
    *(unsigned int *)v5 = 1;
    *(unsigned int *)((long)v5 + 4) = 0;
    v5 = (unsigned long *)sub_10422(0x10);
    v8 = v5;
    *(unsigned long **)&v7[2] = v5;
    v5[1] = v1;
    do {
      v3 = v8;
      v5 = (unsigned long *)sub_10422(0x10);
      v2 = v5;
      v8 = v5;
      dat_26a78 = 7;
      v5 = (unsigned long)sub_12df5();
      v2[1] = v5;
      *v3 = v2;
      v4 = sub_13b02();
    } while (v4 == 6);
    *v2 = 0;
  }
  v4 = dat_26a6c + 1;
  dat_26a6c = v4;
  if (v6) {
  }
  else {
    v5 = (unsigned int *)sub_10422(0x10);
    *(unsigned int *)v5 = 0x19;
    *(unsigned int **)((long)v5 + 8) = v7;
    v7 = (unsigned int *)v5;
  }
  v5 = (unsigned long *)v7;
  return v5;
}

// Function: sub_12df5 @ 0x12df5
int * sub_12df5(void) // ternary
{
  int v1;
  unsigned int **v10; // stack - 0x38
  int *v11; // stack - 0x30
  long *v12; // stack - 0x28
  long *v13; // stack - 0x20
  unsigned int *v14; // stack - 0x18
  int v2; // eax
  unsigned long v3; // rax
  int *v4; // rax
  long v5; // stack - 0x50
  unsigned int *v6; // stack - 0x58
  int v7; // stack - 0x64
  int *v8; // stack - 0x48
  int *v9; // stack - 0x40
  
  v5 = 0;
  v13 = &v5;
  v1 = *(int *)(dat_24028 + 8);
  switch(sub_13b02()) {
    case 7:
      v8 = (int *)sub_10422(0x18);
      *v8 = 4;
      v8[1] = v1;
      v3 = sub_12a27(0);
      *(unsigned long *)&v8[2] = v3;
      v8[4] = 0;
      v8[5] = 0;
      v7 = 8;
      break;
    default:
      sub_15680(0xffffffff); // no-return
    case 0xb:
    case 0xc:
      dat_26a6c += 1;
      return (int *)sub_134e1();
    case 0xe:
      v8 = (int *)sub_10422(0x18);
      *v8 = 0xc;
      v8[1] = v1;
      if (sub_13b02() != 0xc)
        sub_15680(0xc); // no-return
      v9 = (int *)sub_10422(0x20);
      *(int **)&v8[2] = v9;
      *v9 = 0xf;
      *(unsigned long *)&v9[4] = dat_26a70;
      *(unsigned long *)&v9[6] = dat_26a80;
      v9[2] = 0;
      v9[3] = 0;
      dat_26a78 = 7;
      if (sub_13b02() != 0x17)
        sub_15680(0x17); // no-return
      v11 = &v8[4];
      do {
        dat_26a78 = 6;
        v7 = sub_13b02();
        do {
          if (v7 == 0x13) {
            v11[0] = 0;
            v11[1] = 0;
            goto label_13412;
          }
          if (dat_26a68 == 7)
            sub_13b02();
          v14 = (unsigned int *)sub_10422(0x20);
          *(unsigned int **)v11 = v14;
          *v14 = 0xd;
          v10 = (unsigned int **)&v14[4];
          while( true ) {
            v6 = (unsigned int *)sub_10422(0x20);
            *v10 = v6;
            *v6 = 0xf;
            *(unsigned long *)&v6[4] = dat_26a70;
            *(unsigned long *)&v6[6] = dat_26a80;
            if (sub_13b02() != 6) break;
            v10 = (unsigned int **)&v6[2];
            sub_13b02();
          }
          *(unsigned long *)&v6[2] = 0;
          if (dat_26a68 != 8)
            sub_15680(8); // no-return
          v3 = sub_12a27(2);
          *(unsigned long *)&v14[6] = v3;
          v11 = &v14[2];
          dat_26a78 = 6;
          v7 = sub_13b02();
        } while (v7 == 0x13);
        if (v7 != 9)
          sub_15680(9); // no-return
      } while( true );
    case 0x15:
      if (((sub_13b02() != 0xc) || (dat_26a98)) || (v2 = sub_128e1(dat_26a70), !v2))
        sub_15743("Bad for loop variable"); // no-return
      v8 = (int *)sub_10422(0x20);
      *v8 = 0xb;
      v8[1] = v1;
      *(unsigned long *)&v8[6] = dat_26a70;
      dat_26a78 = 7;
      if (sub_13b02() != 0x17) { // branch-flip
        v9 = (int *)sub_10422(0x20);
        *v9 = 0xf;
        v9[4] = 0x1cc06;
        v9[5] = 0;
        v9[6] = 0;
        v9[7] = 0;
        v9[2] = 0;
        v9[3] = 0;
        *(int **)&v8[2] = v9;
        if (dat_26a68 != 2)
          dat_26a6c += 1;
      }
      else {
        v10 = &v6;
        while (sub_13b02() == 0xc) {
          v4 = (int *)sub_10422(0x20);
          v9 = v4;
          *v4 = 0xf;
          *(unsigned long *)&v4[4] = dat_26a70;
          *(unsigned long *)&v4[6] = dat_26a80;
          *v10 = v4;
          v10 = (unsigned int **)&v4[2];
        }
        *v10 = NULL;
        *(unsigned int **)&v8[2] = v6;
        if ((dat_26a68 != 1) && (dat_26a68 != 2))
          sub_15680(0xffffffff); // no-return
      }
      dat_26a78 = 7;
      if (sub_13b02() != 0xf)
        sub_15680(0xf); // no-return
      v3 = sub_12a27(0);
      *(unsigned long *)&v8[4] = v3;
      v7 = 0x10;
      break;
    case 0x16:
      v8 = (int *)sub_10422(0x20);
      *v8 = 8;
      v3 = sub_12a27(0);
      *(unsigned long *)&v8[2] = v3;
      if (sub_13b02() != 0x18)
        sub_15680(0x18); // no-return
      v3 = sub_12a27(0);
      *(unsigned long *)&v8[4] = v3;
      v9 = v8;
      while (sub_13b02() == 0x11) {
        v3 = sub_10422(0x20);
        *(unsigned long *)&v9[6] = v3;
        v9 = *(int **)&v9[6];
        *v9 = 8;
        v3 = sub_12a27(0);
        *(unsigned long *)&v9[2] = v3;
        if (sub_13b02() != 0x18)
          sub_15680(0x18); // no-return
        v3 = sub_12a27(0);
        *(unsigned long *)&v9[4] = v3;
      }
      if (dat_26a68 != 0x12) { // branch-flip
        v9[6] = 0;
        v9[7] = 0;
        dat_26a6c += 1;
      }
      else {
        v3 = sub_12a27(0);
        *(unsigned long *)&v9[6] = v3;
      }
      v7 = 0x14;
      break;
    case 0x19:
    case 0x1a:
      v8 = (int *)sub_10422(0x18);
      v2 = (dat_26a68 != 0x1a) ? 10 : 9; // branch-flip
      *v8 = v2;
      v3 = sub_12a27(0);
      *(unsigned long *)&v8[2] = v3;
      if (sub_13b02() != 0xf)
        sub_15680(0xf); // no-return
      v3 = sub_12a27(0);
      *(unsigned long *)&v8[4] = v3;
      v7 = 0x10;
      break;
    case 0x1b:
      v8 = (int *)sub_12a27(0);
      v7 = 0x1c;
    
  }
  if (v7 != sub_13b02())
    sub_15680(v7); // no-return
label_13412:
  dat_26a78 = 3;
  v12 = v13;
  while (sub_13b02() == 0xb) {
    v9 = dat_26a88;
    *v12 = (long)dat_26a88;
    v12 = (long *)&dat_26a88[2];
    sub_138b6();
  }
  dat_26a6c += 1;
  *v12 = 0;
  if (v5) {
    if (*v8 != 4) {
      v4 = (int *)sub_10422(0x18);
      *v4 = 2;
      v4[1] = v1;
      *(int **)&v4[2] = v8;
      v8 = v4;
    }
    *(long *)&v8[4] = v5;
  }
  return v8;
}

// Function: sub_134e1 @ 0x134e1
unsigned int * sub_134e1(void)
{
  unsigned int v1;
  unsigned int *v10; // stack - 0x38
  long *v11; // stack - 0x30
  long *v12; // stack - 0x28
  unsigned long v13; // stack - 0x20
  long v14; // stack - 0x18
  int v2; // eax
  unsigned long v3; // rax
  unsigned int *v4; // rax
  long v5; // stack - 0x58
  long v6; // stack - 0x50
  long v7; // stack - 0x48
  int v8; // stack - 0x60
  long *v9; // stack - 0x40
  
  v10 = NULL;
  v5 = 0;
  v9 = &v5;
  v6 = 0;
  v11 = &v6;
  v7 = 0;
  v12 = &v7;
  v8 = 1;
  v1 = *(unsigned int *)(dat_24028 + 8);
  while( true ) {
    while( true ) {
      dat_26a78 = v8;
      v2 = sub_13b02();
      if (v2 != 0xc) break;
      v10 = (unsigned int *)sub_10422(0x20);
      *v10 = 0xf;
      *(unsigned long *)&v10[4] = dat_26a70;
      *(unsigned long *)&v10[6] = dat_26a80;
      if ((v8) && (v2 = sub_1295d(dat_26a70), v2)) {
        *v11 = (long)v10;
        v11 = (long *)&v10[2];
      }
      else {
        *v9 = (long)v10;
        v9 = (long *)&v10[2];
        v8 = 0;
      }
    }
    if (0xc < v2) goto label_1371d;
    if (v2 == 7) break;
    if (v2 != 0xb) goto label_1371d;
    v10 = dat_26a88;
    *v12 = (long)dat_26a88;
    v12 = (long *)&dat_26a88[2];
    sub_138b6();
  }
  if ((((v5) && (v9 == (long *)(v5 + 8))) && (!v6)) && (!v7)) {
    if (sub_13b02() != 8)
      sub_15680(8); // no-return
    v13 = *(unsigned long *)&v10[4];
    v2 = sub_128e1(v13);
    if (v2) {
      v14 = sub_959c(v13);
      if ((!v14) || (!(*(unsigned int *)(v14 + 0x10) & 1))) {
        *v10 = 0xe;
        dat_26a78 = 7;
        *(unsigned long *)&v10[2] = *(unsigned long *)&v10[4];
        v10[1] = *(unsigned int *)(dat_24028 + 8);
        v3 = sub_12df5();
        *(unsigned long *)&v10[4] = v3;
        return v10;
      }
    }
    sub_15743("Bad function name"); // no-return
  }
label_1371d:
  dat_26a6c += 1;
  *v9 = 0;
  *v11 = 0;
  *v12 = 0;
  v4 = (unsigned int *)sub_10422(0x20);
  *v4 = 0;
  v4[1] = v1;
  *(long *)&v4[4] = v5;
  *(long *)&v4[2] = v6;
  *(long *)&v4[6] = v7;
  return v4;
}

// Function: sub_137b5 @ 0x137b5
unsigned int * sub_137b5(void)
{
  unsigned int *v1; // rax
  
  v1 = (unsigned int *)sub_10422(0x20);
  *v1 = 0xf;
  *(unsigned long *)&v1[2] = 0;
  *(unsigned long *)&v1[4] = dat_26a70;
  *(unsigned long *)&v1[6] = dat_26a80;
  return v1;
}

// Function: sub_13809 @ 0x13809
void sub_13809(long a0,char *a1,int a2) // return-dupe x2
{
  if (!a2)
    *(unsigned long *)(a0 + 0x18) = 0;
  if (((unsigned int)((int)*a1 - 0x30U) <= 9) && (!a1[1])) {
    *(int *)(a0 + 0x14) = *a1 + -0x30;
    return;
  }
  if ((*a1 == '-') && (!a1[1])) {
    *(unsigned int *)(a0 + 0x14) = 0xffffffff;
    return;
  }
  if (!a2) {
    *(unsigned long *)(a0 + 0x18) = sub_137b5();
    return;
  }
  sub_15743("Bad fd number"); // no-return
}

// Function: sub_138b6 @ 0x138b6
void sub_138b6(void) // return-dupe x3
{
  int *v1;
  unsigned long *v2;
  int v3; // eax
  long *v4; // stack - 0x20
  
  v1 = dat_26a88;
  if (*dat_26a88 == 0x17)
    dat_26a78 = 8;
  v3 = sub_13b02();
  v2 = dat_26a90;
  if (v3 != 0xc)
    sub_15680(0xffffffff); // no-return
  if (*v1 != 0x17) {
    if ((*v1 != 0x15) && (*v1 != 0x16)) {
      *(unsigned long *)&v1[6] = sub_137b5();
      return;
    }
    sub_13809(v1,dat_26a70,0);
    return;
  }
  if (!dat_26a98)
    *v1 = 0x18;
  sub_c50c(dat_26a70,0);
  v2[2] = dat_26a70;
  *v2 = 0;
  if (!dat_26a58) {
    dat_26a58 = v2;
    return;
  }
  for (v4 = dat_26a58; *v4; v4 = (long *)*v4) {
  }
  *v4 = (long)v2;
}

// Function: sub_139df @ 0x139df
void sub_139df(void)
{
  unsigned int v1;
  unsigned long v2;
  unsigned int *v3; // rax
  unsigned long *v4; // stack - 0x38
  
  v4 = dat_26a58;
  dat_26a58 = NULL;
  for (; v4; v4 = (unsigned long *)*v4) {
    if (dat_26a64)
      sub_1577e(2);
    if (*(int *)v4[1] != 0x17) { // branch-flip
      v1 = *(unsigned int *)&v4[3];
      v2 = v4[2];
      sub_13f69(sub_13e7b(),0x1dbe1,v2,v1);
    }
    else {
      v1 = *(unsigned int *)&v4[3];
      v2 = v4[2];
      sub_13f69(sub_cb1c(),0x1dd01,v2,v1);
    }
    v3 = (unsigned int *)sub_10422(0x20);
    *v3 = 0xf;
    *(unsigned long *)&v3[2] = 0;
    *(unsigned long *)&v3[4] = dat_26a70;
    *(unsigned long *)&v3[6] = dat_26a80;
    *(unsigned int **)(v4[1] + 0x18) = v3;
  }
}

// Function: sub_13b02 @ 0x13b02
int sub_13b02(void)
{
  long v1; // rax
  int v2; // stack - 0x20
  unsigned int v3; // stack - 0x1c
  
  v3 = dat_26a78;
  while( true ) {
    v2 = sub_13c71();
    if (v3 & 4) {
      while (v2 == 1) {
        sub_139df();
        dat_26a78 = 0;
        v2 = sub_13c71();
      }
    }
    v3 |= dat_26a78;
    dat_26a78 = 0;
    if (v2 != 0xc) {
      dat_26a78 = 0;
      return v2;
    }
    if (dat_26a98) {
      dat_26a78 = 0;
      return 0xc;
    }
    if ((v3 & 2) && (v1 = sub_15a3c(dat_26a70), v1)) break;
    if (!(v3 & 1))
      return 0xc;
    v1 = sub_4dbc(dat_26a70,1);
    if (!v1)
      return 0xc;
    if (**(char **)(v1 + 0x10))
      sub_ce40(*(unsigned long *)(v1 + 0x10),v1);
  }
  dat_26a68 = (int)(v1 + -0x23240 >> 3) + 0xd;
  return dat_26a68;
}

// Function: sub_13c1b @ 0x13c1b
void sub_13c1b(void) // return-dupe
{
  *(int *)(dat_24028 + 8) = *(int *)(dat_24028 + 8) + 1;
  if (!dat_26a60)
    return;
  sub_1577e(2);
}

// Function: sub_13c4a @ 0x13c4a
void sub_13c4a(void)
{
  *(int *)(dat_24028 + 8) = *(int *)(dat_24028 + 8) + 1;
  dat_26a64 = dat_26a60;
}

// Function: sub_13c71 @ 0x13c71
unsigned long sub_13c71(void) // early-return x10
{
  int v1; // eax
  unsigned long v2; // rax
  
  if (dat_26a6c) { // branch-flip
    dat_26a6c = 0;
    v2 = (unsigned long)dat_26a68;
  }
  else {
    if (dat_26a64)
      sub_1577e(2);
    while( true ) {
      v1 = sub_13e7b();
      if (v1 == 0x7c) {
        v1 = sub_13e7b();
        if (v1 != 0x7c) {
          sub_ce1f();
          v2 = 6;
          dat_26a68 = 6;
          return v2;
        }
        v2 = 5;
        dat_26a68 = 5;
        return v2;
      }
      if (0x7c < v1) break;
      if (v1 == -0x81) {
        v2 = 0;
        dat_26a68 = 0;
        return v2;
      }
      if (((v1 < -0x81) || (0x3b < v1)) || (v1 < 9)) break;
      switch(v1) {
        case 10:
          sub_13c4a();
          v2 = 1;
          dat_26a68 = 1;
          return v2;
        default:
          goto label_13e5e;
        case 0x23:
          do {
            v1 = sub_cb1c();
            if (v1 == 10) break;
          } while (v1 != -0x81);
          sub_ce1f();
          goto label_13e58;
        case 9:
        case 0x20:
          break;
        case 0x26:
          v1 = sub_13e7b();
          if (v1 != 0x26) {
            sub_ce1f();
            v2 = 3;
            dat_26a68 = 3;
            return v2;
          }
          v2 = 4;
          dat_26a68 = 4;
          return v2;
        case 0x28:
          v2 = 7;
          dat_26a68 = 7;
          return v2;
        case 0x29:
          v2 = 8;
          dat_26a68 = 8;
          return v2;
        case 0x3b:
          v1 = sub_13e7b();
          if (v1 != 0x3b) {
            sub_ce1f();
            v2 = 2;
            dat_26a68 = 2;
            return v2;
          }
          v2 = 9;
          dat_26a68 = 9;
          return v2;
        
      }
    }
label_13e5e:
    v2 = sub_13f69(v1,0x1dac1,0,0);
  }
  return v2;
}

// Function: sub_13e7b @ 0x13e7b
int sub_13e7b(void)
{
  int v1; // eax
  
  while( true ) {
    v1 = sub_cb1c();
    if (v1 != 0x5c)
      return v1;
    if (sub_cb1c() != 10) break;
    sub_13c1b();
  }
  sub_ce1f();
  return 0x5c;
}

// Function: sub_13eb2 @ 0x13eb2
void sub_13eb2(long *a0) // return-dupe
{
  if (*a0 == 0x1dd01) {
    sub_cb1c();
    return;
  }
  sub_13e7b();
}

// Function: sub_13ee3 @ 0x13ee3
void sub_13ee3(long *a0,unsigned long *a1,unsigned long a2)
{
  memset(a1,0,0x30);
  *a1 = a2;
  a1[2] = *a0;
  *(unsigned long **)(*a0 + 8) = a1;
  *a0 = (long)a1;
}

// Function: sub_13f48 @ 0x13f48
void sub_13f48(long *a0)
{
  *a0 = *(long *)(*a0 + 0x10);
}

// Function: sub_13f69 @ 0x13f69
unsigned int sub_13f69(unsigned int a0,long a1,char *a2,int a3) // return-dupe, ternary
{
  char v1;
  long v10 [7]; // stack - 0x48
  long *v11; // stack - 0xe0
  long v12; // stack - 0xe8
  char *v13;
  char *v14; // rsp
  char *v15; // rsp
  char *v16; // rsp
  char *v17; // stack - 0x130
  long v18; // stack - 0x128
  int v19; // stack - 0x120
  long v2;
  unsigned int v20; // stack - 0x11c
  char v21; // stack - 0x115
  unsigned int v22; // stack - 0x114
  int v23; // stack - 0x110
  int v24; // stack - 0x10c
  unsigned int v25; // stack - 0x108
  unsigned int v26; // stack - 0x104
  unsigned int v27; // stack - 0x100
  int v28; // stack - 0xfc
  int v29; // stack - 0xf8
  int v3;
  int v30; // stack - 0xf4
  int v31; // stack - 0xf0
  unsigned int v32; // stack - 0xec
  char *v33; // stack - 0xd8
  char *v34; // stack - 0xd0
  unsigned int *v35; // stack - 0xc8
  long v36; // stack - 0xc0
  long *v37; // stack - 0xb8
  void *v38; // stack - 0xb0
  char *v39; // stack - 0xa8
  char *v4;
  void *v40; // stack - 0xa0
  char *v41; // stack - 0x98
  unsigned long v42; // stack - 0x90
  unsigned long v43; // stack - 0x88
  char *v44; // stack - 0x80
  unsigned long v45; // stack - 0x78
  long v46; // stack - 0x70
  unsigned long v47; // stack - 0x68
  unsigned long v48; // stack - 0x60
  unsigned long v49; // stack - 0x58
  unsigned long v5;
  char *v50; // stack - 0x50
  unsigned long v6; // rax
  unsigned long v7; // rax
  char v8; // dl
  char v9 [8];
  
  v16 = v9;
  v20 = a0;
  v18 = a1;
  v17 = a2;
  v19 = a3;
  v22 = a0;
  v10[1] = 0;
  v10[2] = 0;
  v10[3] = 0;
  v10[4] = 0;
  v10[5] = 0;
  v10[0] = a1;
  v11 = v10;
  v23 = 0;
  v12 = 0;
  v33 = dat_24048;
label_14035:
  *(unsigned long *)&v16[-8] = 0x14044;
  v3 = sub_1299f(v17);
  if (v3) {
    if (v19) {
      while (v22 == 9) {
        *(unsigned long *)&v16[-8] = 0x14722;
        v22 = sub_cb1c();
      }
    }
    v29 = (int)v33 - (int)dat_24048;
    v34 = v17;
    while( true ) {
      *(unsigned long *)&v16[-8] = 0x14797;
      v33 = (char *)sub_12921(v22,v33);
      if (!*v34) break;
      if (v22 != (int)*v34) goto label_147d9;
      *(unsigned long *)&v16[-8] = 0x14772;
      v22 = sub_cb1c();
      v34 = &v34[1];
    }
    if ((v22 != 10) && (v22 != 0xffffff7f)) {
label_147d9:
      v4 = &dat_24048[(long)v29 + 1];
      v34 = v4;
      v30 = (int)v33 - (int)v4;
      if (v30) {
        v3 = v30 + ((int)v22 >> 0x1f);
        v30 = v3;
        v22 = (int)v4[-1];
        if (v3) {
          v5 = (((long)(v3 + 1) + 0x17U) / 0x10) * 0x10;
          for (v13 = v16; v13 != &v16[-(v5 & 0xfffffffffffff000)]; v13 = &v13[-0x1000]) {
            *(unsigned long *)&v13[-8] = *(unsigned long *)&v13[-8];
          }
          v2 = -(unsigned long)((unsigned int)v5 & 0xfff);
          v16 = &v13[v2];
          if (v5 & 0xfff)
            *(unsigned long *)&v13[((unsigned long)((unsigned int)v5 & 0xfff) - 8) + v2] = *(unsigned long *)&v13[((unsigned long)((unsigned int)v5 & 0xfff) - 8) + v2];
          v40 = (void *)((unsigned long)&v13[v2 + 0xf] & 0xfffffffffffffff0);
          *(unsigned long *)&v13[v2 + -8] = 0x14902;
          v1 = v13[v2 + -8];
          *(char *)mempcpy((void *)((unsigned long)&v13[v2 + 0xf] & 0xfffffffffffffff0),v4,(long)v3,v1) = 0;
          *(unsigned long *)&v13[v2 + -8] = 0x14919;
          sub_ce40(v40,0);
        }
      }
    }
    else {
      v22 = 0xffffff7f;
      *(unsigned long *)&v16[-8] = 0x147d0;
      sub_13c4a();
    }
    v33 = &dat_24048[v29];
  }
label_1404d:
  v41 = v33;
  v42 = 4;
  v43 = dat_24050 - (long)v33;
  if ((unsigned long)(dat_24050 - (long)v33) < 4) {
    *(unsigned long *)&v16[-8] = 0x1409e;
    v33 = (char *)sub_1090b(4,v33);
  }
  v4 = v33;
  v8 = (char)v22;
  switch(*(char *)((long)(int)v22 + *v11)) {
    case 0:
      v33 = &v33[1];
      *v4 = v8;
      goto label_145a3;
    case 1:
      goto label_140ec;
    case 2:
      *(unsigned long *)&v16[-8] = 0x141cf;
      v22 = sub_cb1c();
      if (v22 != 0xffffff7f) { // branch-flip
        if (((((((int)v11[4]) && (v22 != 0x5c)) && (v22 != 0x60)) && (v22 != 0x24)) && ((v22 != 0x22 || ((v17 && (!*(int *)((long)v11 + 0x24))))))) && ((v22 != 0x7d || (!*(int *)((long)v11 + 0x24))))) {
          v4 = &v33[1];
          *v33 = '\x81';
          v33 = &v33[2];
          *v4 = '\\';
        }
        v4 = &v33[1];
        *v33 = '\x81';
        v33 = &v33[2];
        *v4 = (char)v22;
        v23 += 1;
      }
      else {
        v4 = &v33[1];
        *v33 = '\x81';
        v33 = &v33[2];
        *v4 = '\\';
        *(unsigned long *)&v16[-8] = 0x14210;
        sub_ce1f();
      }
      break;
    case 3:
      *v11 = 0x1dd01;
      goto label_142f2;
    case 4:
      *v11 = 0x1dbe1;
      *(unsigned int *)&v11[4] = 1;
label_14339:
      if (*(int *)((long)v11 + 0x24))
        *(unsigned int *)&v11[3] = *(unsigned int *)&v11[3] ^ 1;
label_142f2:
      if (!v17) {
        v33 = &v33[1];
        *v4 = '\x88';
      }
      break;
    case 5:
      if ((!v17) || (*(int *)((long)v11 + 0x24))) {
        if (!*(int *)((long)v11 + 0x2c)) {
          *v11 = 0x1dac1;
          *(unsigned int *)&v11[4] = 0;
        }
        v23 += 1;
        if (v22 == 0x22) goto label_14339;
        goto label_142f2;
      }
      v33 = &v33[1];
      *v4 = v8;
      break;
    case 6:
      if (dat_26a78 & 8) { // branch-flip
        v33 = &v33[1];
        *v4 = '`';
      }
      else {
        v24 = 1;
label_15134:
        v38 = NULL;
        v5 = (long)v33 - (long)dat_24048;
        v45 = v5;
        if (v5) {
          v6 = ((v5 + 0x17) / 0x10) * 0x10;
          for (v13 = v16; v13 != &v16[-(v6 & 0xfffffffffffff000)]; v13 = &v13[-0x1000]) {
            *(unsigned long *)&v13[-8] = *(unsigned long *)&v13[-8];
          }
          v2 = -(unsigned long)((unsigned int)v6 & 0xfff);
          v16 = &v13[v2];
          if (v6 & 0xfff)
            *(unsigned long *)&v13[((unsigned long)((unsigned int)v6 & 0xfff) - 8) + v2] = *(unsigned long *)&v13[((unsigned long)((unsigned int)v6 & 0xfff) - 8) + v2];
          v38 = (void *)((unsigned long)&v13[v2 + 0xf] & 0xfffffffffffffff0);
          *(unsigned long *)&v13[v2 + -8] = 0x15208;
          memcpy((void *)((unsigned long)&v13[v2 + 0xf] & 0xfffffffffffffff0),dat_24048,v5,v13[v2 + -8]);
        }
        if (v24) {
          v39 = dat_24048;
          while( true ) {
            if (dat_26a64) {
              *(unsigned long *)&v16[-8] = 0x15237;
              sub_1577e(2);
            }
            *(unsigned long *)&v16[-8] = 0x1523c;
            v28 = sub_13e7b();
            if (v28 == 0x60) break;
            if (v28 <= 0x60) {
              if (v28 != 0x5c) { // branch-flip
                if (v28 <= 0x5c) {
                  if (v28 == -0x81) {
                    *(unsigned long *)&v16[-8] = 0x152f2; // no-return
                    sub_15743("EOF in backquote substitution");
                  }
                  if (v28 == 10) {
                    *(unsigned long *)&v16[-8] = 0x152f7;
                    sub_13c4a();
                  }
                }
              }
              else {
                *(unsigned long *)&v16[-8] = 0x1528e;
                v28 = sub_cb1c();
                if (((v28 != 0x5c) && (v28 != 0x60)) && ((v28 != 0x24 && ((!(int)v11[4] || (v28 != 0x22)))))) {
                  *(unsigned long *)&v16[-8] = 0x152da;
                  v39 = (char *)sub_12921(0x5c,v39);
                }
              }
            }
            *(unsigned long *)&v16[-8] = 0x15314;
            v39 = (char *)sub_12921(v28,v39);
          }
          *(unsigned long *)&v16[-8] = 0x15335;
          v39 = (long)sub_12921(0,v39);
          v46 = (long)v39 - (long)dat_24048;
          if ((long)v39 - (long)dat_24048) {
            *(unsigned long *)&v16[-8] = 0x15371;
            v7 = sub_10422((long)v39 - (long)dat_24048);
            v47 = v7;
            *(unsigned long *)&v16[-8] = 0x15381;
            sub_d25c(v7);
          }
        }
        for (v37 = &v12; *v37; v37 = (long *)*v37) {
        }
        *(unsigned long *)&v16[-8] = 0x153bb;
        *v37 = sub_10422(0x10);
        *(unsigned long *)*v37 = 0;
        v48 = dat_26a58;
        dat_26a58 = 0;
        if (v24) {
          v27 = dat_26a60;
          dat_26a60 = 0;
        }
        *(unsigned long *)&v16[-8] = 0x15415;
        v49 = sub_12a27(2);
        if (v24) // branch-flip
          dat_26a60 = v27;
        else {
          *(unsigned long *)&v16[-8] = 0x15435;
          if (sub_13b02() != 8) {
            *(unsigned long *)&v16[-8] = 0x15444; // no-return
            sub_15680(8);
          }
          *(unsigned long *)&v16[-8] = 0x15453;
          sub_d25c(0x269e0);
        }
        *(unsigned long *)&v16[-8] = 0x15458;
        sub_139df();
        dat_26a58 = v48;
        *(unsigned long *)(*v37 + 8) = v49;
        *(unsigned long *)&v16[-8] = 0x1547a;
        sub_d363();
        if (v24)
          dat_26a6c = 0;
        *(unsigned long *)&v16[-8] = 0x1549d;
        v33 = (char *)sub_108d9(v45 + 1);
        if (v38) {
          *(unsigned long *)&v16[-8] = 0x154cb;
          memcpy(v33,v38,v45,v16[-8]);
          v33 = &v33[v45];
        }
        v4 = v33;
        v33 = &v33[1];
        *v4 = 0x84;
      }
      break;
    case 7:
      *(unsigned long *)&v16[-8] = 0x14b34;
      v22 = sub_13e7b();
      if (dat_26a78 & 8) {
label_14bb0:
        v4 = v33;
        v33 = &v33[1];
        *v4 = '$';
        *(unsigned long *)&v16[-8] = 0x14bca;
        sub_ce1f();
        goto label_145a3;
      }
      if (((v22 != 0x28) && (v22 != 0x7b)) && (v22 != 0x5f)) {
        *(unsigned long *)&v16[-8] = 0x14b67;
        v1 = v16[-8];
        if ((!(*(unsigned short *)((unsigned long)(v22 & 0xff) * 2 + *(long *)__ctype_b_loc(v1)) & 0x400)) && (!(*(unsigned char *)((long)(char)v22 + 0x1df41) & 0x11))) goto label_14bb0;
      }
      if (v22 == 0x28) {
        *(unsigned long *)&v16[-8] = 0x14bdd;
        if (sub_13e7b() != 0x28) {
          *(unsigned long *)&v16[-8] = 0x14beb;
          sub_ce1f();
          v24 = 0;
          goto label_15134;
        }
        v5 = v11[1];
        v13 = v16;
        if (!v5) {
          while (v13 != v16) {
            *(unsigned long *)&v13[-8] = *(unsigned long *)&v13[-8];
            v15 = &v13[-0x1000];
            v13 = v15;
          }
          v16 = &v13[-0x40];
          *(unsigned long *)&v13[-8] = *(unsigned long *)&v13[-8];
          v5 = (unsigned long)&v13[-0x31] & 0xfffffffffffffff0;
        }
        *(unsigned long *)&v16[-8] = 0x155a8;
        sub_13ee3(&v11,v5,0x1de21);
        v4 = v33;
        *(unsigned int *)&v11[4] = 1;
        v33 = &v33[1];
        *v4 = '\x86';
        goto label_145a3;
      }
      v36 = *v11;
      *v33 = '\x82';
      v31 = ((int)v33 + 1) - (int)dat_24048;
      v33 = &v33[2];
      v25 = 1;
      if (v22 == 0x7b) {
        *(unsigned long *)&v16[-8] = 0x14c66;
        v22 = sub_13e7b();
        v25 = 0;
      }
      do {
        do {
          if (v22 == 0x5f) goto label_14ca8;
          *(unsigned long *)&v16[-8] = 0x14c87;
          v1 = v16[-8];
          if (*(unsigned short *)((unsigned long)(v22 & 0xff) * 2 + *(long *)__ctype_b_loc(v1)) & 0x400) goto label_14ca8;
          if (v22 - 0x30 <= 9) goto label_14d11;
          if (v22 == 0x7d) goto label_14f96;
          v26 = v22;
          *(unsigned long *)&v16[-8] = 0x14d81;
          v22 = sub_13e7b();
          if ((v25) || (v26 != 0x23)) goto label_14e2e;
          v25 = 10;
        } while (v22 == 0x5f);
        *(unsigned long *)&v16[-8] = 0x14dbd;
        v1 = v16[-8];
      } while (*(unsigned short *)((long)(int)v22 * 2 + *(long *)__ctype_b_loc(v1)) & 8);
      v26 = v22;
      *(unsigned long *)&v16[-8] = 0x14df1;
      v22 = sub_13e7b();
      if ((v26 == 0x7d) || (v22 != 0x7d)) {
        *(unsigned long *)&v16[-8] = 0x14e0e;
        sub_ce1f();
        v25 = 0;
        v22 = v26;
        v26 = 0x23;
      }
label_14e2e:
      v4 = v33;
      if (*(unsigned char *)((long)(char)v26 + 0x1df41) & 0x11) {
        v33 = &v33[1];
        *v4 = (char)v26;
        goto label_14e8c;
      }
      if (v25 == 10)
        v25 = 0;
      goto label_14f96;
    case 8:
      if (((int)v11[3]) || (*(int *)((long)v11 + 0x24) <= 0)) {
        v33 = &v33[1];
        *v4 = v8;
      }
      else {
        *(int *)((long)v11 + 0x24) = *(int *)((long)v11 + 0x24) + -1;
        if ((*(int *)((long)v11 + 0x24)) || (!*(int *)((long)v11 + 0x1c))) {
          if (1 <= *(int *)((long)v11 + 0x2c))
            *(int *)((long)v11 + 0x2c) = *(int *)((long)v11 + 0x2c) + -1;
        }
        else {
          *(unsigned long *)&v16[-8] = 0x14432;
          sub_13f48(&v11);
        }
        v4 = v33;
        v33 = &v33[1];
        *v4 = '\x83';
      }
      break;
    case 9:
      *(int *)&v11[5] = (int)v11[5] + 1;
      v33 = &v33[1];
      *v4 = v8;
      break;
    case 10:
      if (1 <= (int)v11[5]) { // branch-flip
        v33 = &v33[1];
        *v4 = v8;
        *(int *)&v11[5] = (int)v11[5] + -1;
      }
      else {
        *(unsigned long *)&v16[-8] = 0x144fc;
        v3 = sub_13e7b();
        v4 = v33;
        if (v3 != 0x29) { // branch-flip
          *(unsigned long *)&v16[-8] = 0x1452c;
          sub_ce1f();
          v4 = v33;
          v33 = &v33[1];
          *v4 = ')';
        }
        else {
          v33 = &v33[1];
          *v4 = '\x87';
          *(unsigned long *)&v16[-8] = 0x14525;
          sub_13f48(&v11);
        }
      }
      break;
    case 0xb:
      goto label_145c4;
    case 0xc:
      if (*(int *)((long)v11 + 0x24) || (!v17 || (int)v11[4])) {
        v4 = &v33[1];
        *v33 = '\x81';
        v33 = v4;
      }
      v4 = v33;
      v33 = &v33[1];
      *v4 = v8;
      break;
    default:
      if (!*(int *)((long)v11 + 0x24)) goto label_145c4;
      v33 = &v33[1];
      *v4 = v8;
    
  }
  goto label_145a3;
label_14ca8:
  do {
    do {
      *(unsigned long *)&v16[-8] = 0x14cbf;
      v33 = (char *)sub_12921(v22,v33);
      *(unsigned long *)&v16[-8] = 0x14ccb;
      v22 = sub_13e7b();
    } while (v22 == 0x5f);
    *(unsigned long *)&v16[-8] = 0x14cdf;
    v1 = v16[-8];
  } while (*(unsigned short *)((unsigned long)(v22 & 0xff) * 2 + *(long *)__ctype_b_loc(v1)) & 8);
  goto label_14e8c;
  while (v22 - 0x30 <= 9) {
label_14d11:
    *(unsigned long *)&v16[-8] = 0x14d28;
    v33 = (char *)sub_12921(v22,v33);
    *(unsigned long *)&v16[-8] = 0x14d34;
    v22 = sub_13e7b();
    if ((1 <= (int)v25) && ((int)v25 <= 9)) break;
  }
label_14e8c:
  if (v25) { // branch-flip
    if ((v25 == 10) && (v22 != 0x7d))
      v25 = 0;
label_14f96:
    *(unsigned long *)&v16[-8] = 0x14f9b;
    sub_ce1f();
  }
  else {
    v32 = v22;
    if (v22 != 0x3a) { // branch-flip
      if (((int)v22 <= 0x3a) && ((v22 == 0x23 || (v22 == 0x25)))) {
        v25 = (v22 != 0x23) ? 6 : 8; // branch-flip
        *(unsigned long *)&v16[-8] = 0x14f3f;
        v22 = sub_13e7b();
        if (v22 != v32) { // branch-flip
          *(unsigned long *)&v16[-8] = 0x14f61;
          sub_ce1f();
        }
        else {
          v25 += 1;
        }
        v36 = 0x1dac1;
        goto label_14f9e;
      }
    }
    else {
      v25 = 0x10;
      *(unsigned long *)&v16[-8] = 0x14eda;
      v22 = sub_13e7b();
    }
    *(unsigned long *)&v16[-8] = 0x14ef7;
    v44 = strchr("}-+?=",v22,v16[-8]);
    if (v44)
      v25 |= (int)v44 - 0x1d103U;
  }
label_14f9e:
  if (v36 == 0x1de21)
    v36 = 0x1dbe1;
  if (((v36 != *v11) || ((int)v11[3])) && (v25 != 1)) {
    v5 = v11[1];
    v13 = v16;
    if (!v5) {
      while (v13 != v16) {
        *(unsigned long *)&v13[-8] = *(unsigned long *)&v13[-8];
        v14 = &v13[-0x1000];
        v13 = v14;
      }
      v16 = &v13[-0x40];
      *(unsigned long *)&v13[-8] = *(unsigned long *)&v13[-8];
      v5 = (unsigned long)&v13[-0x31] & 0xfffffffffffffff0;
    }
    *(unsigned long *)&v16[-8] = 0x15095;
    sub_13ee3(&v11,v5,v36);
    *(int *)((long)v11 + 0x1c) = *(int *)((long)v11 + 0x1c) + 1;
    *(unsigned int *)&v11[4] = (unsigned int)(v36 != 0x1dac1);
  }
  dat_24048[v31] = (char)v25;
  if ((v25 != 1) && (*(int *)((long)v11 + 0x24) = *(int *)((long)v11 + 0x24) + 1, (int)v11[4]))
    *(int *)((long)v11 + 0x2c) = *(int *)((long)v11 + 0x2c) + 1;
  *(unsigned long *)&v16[-8] = 0x15128;
  v33 = (char *)sub_12921(0x3d,v33);
label_145a3:
  *(unsigned long *)&v16[-8] = 0x145b2;
  v22 = sub_13eb2(v11);
  goto label_1404d;
label_140ec:
  if ((*v11 == 0x1dac1) && (!*(int *)((long)v11 + 0x24))) goto label_145c4;
  v33 = &v33[1];
  *v4 = v8;
  *(unsigned long *)&v16[-8] = 0x14133;
  sub_13c1b();
  *(unsigned long *)&v16[-8] = 0x14142;
  v22 = sub_13eb2(v11);
  goto label_14035;
label_145c4:
  if (*v11 == 0x1de21) {
    *(unsigned long *)&v16[-8] = 0x145e9; // no-return
    sub_15743("Missing \'))\'");
  }
  if ((*v11 != 0x1dac1) && (!v17)) {
    *(unsigned long *)&v16[-8] = 0x14618; // no-return
    sub_15743("Unterminated quoted string");
  }
  if (*(int *)((long)v11 + 0x24)) {
    *(unsigned long *)&v16[-8] = 0x14635; // no-return
    sub_15743("Missing \'}\'");
  }
  *v33 = '\0';
  v50 = &v33[1 - (long)dat_24048];
  v33 = dat_24048;
  if (!v17) {
    if ((((v22 == 0x3e) || (v22 == 0x3c)) && (!v23)) && ((v50 <= (char *)0x2 && ((!*dat_24048 || ((unsigned int)((int)*dat_24048 - 0x30U) <= 9)))))) {
      v21 = *dat_24048;
      *(unsigned long *)&v16[-8] = 0x14952;
      v35 = (unsigned int *)sub_10422(0x28);
      if (v22 != 0x3e) { // branch-flip
        v35[4] = 0;
        *(unsigned long *)&v16[-8] = 0x149fa;
        v22 = sub_13e7b();
        if (v22 != 0x3e) { // branch-flip
          if ((int)v22 <= 0x3e) {
            if (v22 == 0x26) {
              *v35 = 0x16;
              goto label_14aed;
            }
            if (v22 == 0x3c) {
              *(unsigned long *)&v16[-8] = 0x14a3e;
              v35 = (unsigned int *)sub_10422(0x20);
              v35[4] = 0;
              *v35 = 0x17;
              *(unsigned long *)&v16[-8] = 0x14a6a;
              dat_26a90 = sub_10422(0x20);
              *(unsigned int **)(dat_26a90 + 8) = v35;
              *(unsigned long *)&v16[-8] = 0x14a88;
              v22 = sub_13e7b();
              if (v22 != 0x2d) { // branch-flip
                *(unsigned int *)(dat_26a90 + 0x18) = 0;
                *(unsigned long *)&v16[-8] = 0x14aba;
                sub_ce1f();
              }
              else {
                *(unsigned int *)(dat_26a90 + 0x18) = 1;
              }
              goto label_14aed;
            }
          }
          *v35 = 0x12;
          *(unsigned long *)&v16[-8] = 0x14aec;
          sub_ce1f();
        }
        else {
          *v35 = 0x13;
        }
      }
      else {
        v35[4] = 1;
        *(unsigned long *)&v16[-8] = 0x14979;
        v22 = sub_13e7b();
        if (v22 != 0x3e) { // branch-flip
          if (v22 != 0x7c) { // branch-flip
            if (v22 != 0x26) { // branch-flip
              *v35 = 0x10;
              *(unsigned long *)&v16[-8] = 0x149e2;
              sub_ce1f();
            }
            else {
              *v35 = 0x15;
            }
          }
          else {
            *v35 = 0x11;
          }
        }
        else {
          *v35 = 0x14;
        }
      }
label_14aed:
      if (!v21) {
        dat_26a68 = 0xb;
        dat_26a88 = v35;
        return 0xb;
      }
      v35[4] = v21 + -0x30;
      dat_26a68 = 0xb;
      dat_26a88 = v35;
      return 0xb;
    }
    *(unsigned long *)&v16[-8] = 0x146c9;
    sub_ce1f();
  }
  dat_26a98 = v23;
  dat_26a80 = v12;
  *(unsigned long *)&v16[-8] = 0x146ef;
  sub_12906(v50);
  dat_26a70 = v33;
  dat_26a68 = 0xc;
  return 0xc;
}

// Function: sub_155e7 @ 0x155e7
unsigned short * sub_155e7(unsigned char *a0)
{
  long v1;
  unsigned short *v2; // rax
  unsigned char *v3; // stack - 0x10
  
  v3 = a0;
  if ((*a0 != 0x5f) && (v2 = __ctype_b_loc(), v1 = *(long *)v2, v2 = (unsigned long)(unsigned long)*a0, v2 = (unsigned short *)((long)v2 * 2 + v1), !(*v2 & 0x400)))
    v2 = (unsigned short *)a0;
  else {
    do {
      v3 = &v3[1];
      if (!*v3) break;
    } while ((*v3 == 0x5f) || (v2 = __ctype_b_loc(), v1 = *(long *)v2, v2 = (unsigned long)(unsigned long)*v3, v2 = (unsigned short *)((long)v2 * 2 + v1), *v2 & 8));
    v2 = (unsigned short *)v3;
  }
  return v2;
}

// Function: sub_15680 @ 0x15680
void sub_15680(int a0)
{
  char v1 [72];
  long v2; // fs_offset
  unsigned long v3; // stack - 0x10
  
  v3 = *(unsigned long *)(v2 + 0x28);
  if (0 <= a0) // branch-flip
    sub_172fa(v1,0x40,"%s unexpected (expecting %s)",*(unsigned long *)((long)dat_26a68 * 8 + 0x23140),*(unsigned long *)((long)a0 * 8 + 0x23140));
  else {
    sub_172fa(v1,0x40,"%s unexpected",*(unsigned long *)((long)dat_26a68 * 8 + 0x23140));
  }
  sub_15743(v1); // tail-call, no-return
}

// Function: sub_15743 @ 0x15743
void sub_15743(unsigned long a0)
{
  dat_243ec = *(unsigned int *)(dat_24028 + 8);
  sub_6621("Syntax error: %s",a0); // no-return
}

// Function: sub_1577e @ 0x1577e
void sub_1577e(unsigned int a0)
{
  unsigned long v1;
  char v2 [24];
  
  dat_26a64 = 0;
  dat_26644 = a0;
  sub_10597(v2,dat_24038);
  v1 = dat_240d0;
  sub_1701f(sub_159dc(0),v1);
  sub_10641(v2);
}

// Function: sub_15816 @ 0x15816
unsigned long sub_15816(unsigned long a0)
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  unsigned int v4;
  char v5 [200];
  unsigned int v6 [2]; // stack - 0x118
  unsigned long v7; // stack - 0x110
  unsigned long v8; // stack - 0x108
  unsigned long v9; // stack - 0x100
  
  v1 = dat_24028;
  sub_d25c(a0);
  v4 = dat_26a60;
  v3 = dat_26a58;
  v2 = dat_243d8;
  dat_26a58 = 0;
  dat_26a60 = 0;
  if (_setjmp(v5)) { // branch-flip
    if (dat_243e0 != 1) {
      dat_243d8 = (char *)v2;
      longjmp(v2,1); // no-return
    }
  }
  else {
    dat_243d8 = v5;
    sub_13f69(sub_13e7b(),0x1dbe1,1,0);
    v6[0] = 0xf;
    v7 = 0;
    v8 = dat_26a70;
    v9 = dat_26a80;
    sub_a021(v6,0,0x100);
    a0 = dat_24048;
  }
  dat_243d8 = (char *)v2;
  dat_26a60 = v4;
  sub_d44e(v1);
  dat_26a58 = v3;
  return a0;
}

// Function: sub_159dc @ 0x159dc
unsigned long sub_159dc(void) // early-return
{
  unsigned long v1; // rax
  long v2; // stack - 0x10
  
  v2 = dat_241b0;
  if ((dat_26644 != 1) && (v2 = dat_241d0, dat_26644 != 2))
    return 0x269e0;
  v2 += 4;
  v1 = sub_15816(v2);
  return v1;
}

// Function: sub_15a3c @ 0x15a3c
void sub_15a3c(unsigned long a0)
{
  sub_1199c(a0,0x23240,0x10);
}

// Function: sub_15a69 @ 0x15a69
unsigned int sub_15a69(unsigned char a0,int a1) // ternary
{
  unsigned int v1;
  unsigned int v2; // edx
  
  v1 = dat_26aa8;
  v2 = 1 << (a0 & 0x1f);
  dat_26aa8 = (0 <= a1) ? dat_26aa8 & ~v2 : dat_26aa8 | v2; // branch-flip
  return v1 & v2;
}

// Function: sub_15ac5 @ 0x15ac5
void sub_15ac5(long a0,unsigned int a1)
{
  int v1; // eax
  int v2; // eax
  int *v3; // rax
  int v4; // stack - 0x30
  int v5; // stack - 0x2c
  long v6; // stack - 0x20
  long v7; // stack - 0x18
  
  if (a0) {
    v7 = 0;
    dat_243e4 += 1;
    if (a1 & 1)
      v7 = dat_26aa0;
    v6 = a0;
    do {
      v1 = sub_15d3e(v6);
      if (-1 <= v1) {
        v5 = *(int *)(v6 + 0x10);
        if (v7) {
          v3 = (int *)(v7 + (long)v5 * 4 + 8);
          v4 = *v3;
          v2 = sub_15a69(v5,v1);
          if (((v4 == -2) && (v4 = -1, v5 != v1)) && (!v2)) {
            v4 = sub_1629a(v5,v5);
            v5 = -1;
          }
          *v3 = v4;
        }
        if (v5 != v1)
          sub_15f3d(v6,v1);
      }
      v6 = *(long *)(v6 + 8);
    } while (v6);
    dat_243e4 -= 1;
    if ((!dat_243e4) && (dat_243e8))
      sub_64d8(); // no-return
    if ((a1 & 3) && (0 <= *(int *)(v7 + 0x10)))
      dat_26dc0 = *(unsigned int *)(v7 + 0x10);
  }
}

// Function: sub_15c4c @ 0x15c4c
void sub_15c4c(unsigned long a0,unsigned int a1,unsigned int a2)
{
  unsigned int v1; // stack - 0x14
  char *v2; // stack - 0x10
  
  v2 = "open";
  v1 = 1;
  if (a1 & 0x40) {
    v2 = "create";
    v1 = 2;
  }
  sub_6621("cannot %s %s: %s",v2,a0,sub_686a(a2,v1)); // no-return
}

// Function: sub_15cc1 @ 0x15cc1
int sub_15cc1(unsigned long a0,unsigned int a1,int a2)
{
  int v1; // eax
  
  do {
    v1 = open64(a0,a1,0x1b6);
    if ((0 <= v1) || (*dat_266e8 != 4)) break;
  } while (!dat_26d20);
  if ((!a2) && (v1 < 0))
    sub_15c4c(a0,a1,*dat_266e8); // no-return
  return v1;
}

// Function: sub_15d3e @ 0x15d3e
int sub_15d3e(unsigned int *a0)
{
  unsigned long v1;
  int v2; // eax
  char v3 [24];
  unsigned int v4; // stack - 0xb8
  unsigned int v5; // stack - 0x90
  
  switch(*a0) {
    case 0x10:
      if (dat_26a4b) {
        v1 = *(unsigned long *)&a0[8];
        if (0 <= (int)stat64(v1,v3)) {
          if ((v5 & 0xf000) != 0x8000) {
            v2 = sub_15cc1(v1,1,0);
            if (fstat64(v2,v3))
              return v2;
            if ((v5 & 0xf000) != 0x8000)
              return v2;
            close(v2);
          }
          sub_15c4c(v1,0x40,0x11); // no-return
        }
        v4 = 0xc1;
        break;
      }
      v4 = 0x241; // return-dupe
      v2 = sub_15cc1(*(unsigned long *)&a0[8],v4,0);
      return v2;
    case 0x11:
      v4 = 0x241;
      break;
    case 0x12:
      v4 = 0;
      break;
    case 0x13:
      v4 = 0x42;
      break;
    case 0x14:
      v4 = 0x441;
      break;
    case 0x15:
    case 0x16:
      if (a0[5] != a0[4])
        return a0[5];
      return -2;
    default:
      return sub_16002(a0);
    
  }
  v2 = sub_15cc1(*(unsigned long *)&a0[8],v4,0);
  return v2;
}

// Function: sub_15f3d @ 0x15f3d
void sub_15f3d(int *a0,int a1) // return-dupe
{
  int v1; // eax
  int v2; // stack - 0x24
  int v3; // stack - 0x10
  
  v2 = a0[4];
  v3 = 0;
  if ((*a0 != 0x15) && (*a0 != 0x16)) {
    v1 = dup2(a1,v2);
    v2 = a1;
    if (v1 <= -1)
      v3 = *dat_266e8;
  }
  else if (0 <= a1) {
    if (0 <= dup2(a1,v2))
      return;
    v3 = *dat_266e8;
    v2 = a1;
    sub_6621("%d: %s",v2,strerror(v3)); // return-dupe, no-return
  }
  close(v2);
  if (0 <= v3)
    return;
  sub_6621("%d: %s",v2,strerror(v3));
}

// Function: sub_16002 @ 0x16002
int sub_16002(int *a0)
{
  unsigned long v1; // rax
  int v2; // stack - 0x18
  char *v3; // stack - 0x28
  int v4; // stack - 0x14
  
  if (pipe(&v2) <= -1)
    sub_6621("Pipe call failed"); // no-return
  v3 = *(char **)(*(long *)&a0[6] + 0x10);
  if (*a0 == 0x18) {
    sub_a021(*(unsigned long *)&a0[6],0,0x100);
    v3 = dat_24048;
  }
  v1 = strlen(v3);
  if (0x1001 <= v1) { // branch-flip
    if (!sub_eb5f(0,0,2)) {
      close(v2);
      signal(2,1);
      signal(3,1);
      signal(1,1);
      signal(0x14,1);
      signal(0xd,0);
      sub_176a1(v4,v3,v1);
      _exit(0); // no-return
    }
  }
  else {
    sub_176a1(v4,v3,v1);
  }
  close(v4);
  return v2;
}

// Function: sub_16171 @ 0x16171
void sub_16171(int a0)
{
  unsigned long *v1;
  int v2; // eax
  int v3; // stack - 0x18
  
  v1 = dat_26aa0;
  dat_243e4 += 1;
  for (v3 = 0; v3 <= 9; v3 = v3 + 1) {
    if (*(int *)((long)v1 + (long)v3 * 4 + 8) != -2) {
      if (a0) // branch-flip
        v2 = 1;
      else {
        v2 = sub_15a69(v3,*(unsigned int *)((long)v1 + (long)v3 * 4 + 8));
      }
      if (*(int *)((long)v1 + (long)v3 * 4 + 8) != -1) { // branch-flip
        if (!a0)
          dup2(*(int *)((long)v1 + (long)v3 * 4 + 8),v3);
        close(*(int *)((long)v1 + (long)v3 * 4 + 8));
      }
      else if (!v2)
        close(v3);
    }
  }
  dat_26aa0 = (unsigned long)*v1;
  free(v1);
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
}

// Function: sub_1629a @ 0x1629a
int sub_1629a(int a0,int a1)
{
  int v1; // eax
  int v2;
  
  v1 = fcntl(a0,0,10);
  if (0 <= v1) // branch-flip
    v2 = 0;
  else {
    v2 = *dat_266e8;
  }
  if (v2 != 9) {
    close(a1);
    if (v2)
      sub_6621("%d: %s",a0,strerror(v2)); // no-return
    fcntl(v1,2,1);
  }
  return v1;
}

// Function: sub_1633b @ 0x1633b
int sub_1633b(unsigned long a0,unsigned int a1)
{
  int v1;
  unsigned long v2;
  int v3;
  char v4 [200];
  
  v3 = dat_243e4;
  v2 = dat_243d8;
  v1 = _setjmp(v4) * 2;
  if (!v1) {
    dat_243d8 = v4;
    sub_15ac5(a0,a1);
  }
  dat_243d8 = (char *)v2;
  if ((v1) && (dat_243e0 != 1))
    longjmp(v2,1); // no-return
  dat_243e4 = v3;
  if ((!v3) && (dat_243e8))
    sub_64d8(); // no-return
  return v1;
}

// Function: sub_16440 @ 0x16440
void sub_16440(long a0)
{
  while (a0 != dat_26aa0) {
    sub_16171(0);
  }
}

// Function: sub_1646d @ 0x1646d
unsigned long * sub_1646d(long a0)
{
  unsigned long *v1;
  int v2; // stack - 0x1c
  
  v1 = dat_26aa0;
  if (a0) {
    dat_26aa0 = (unsigned long *)sub_10354(0x30);
    *dat_26aa0 = v1;
    for (v2 = 0; v2 <= 9; v2 = v2 + 1) {
      *(unsigned int *)((long)dat_26aa0 + (long)v2 * 4 + 8) = 0xfffffffe;
    }
  }
  return v1;
}

// Function: sub_164e1 @ 0x164e1
unsigned long sub_164e1(void)
{
  unsigned long v1;
  unsigned long v2;
  long v3;
  long *v4;
  int v5; // eax
  int v6; // stack - 0x2c
  char *v7; // stack - 0x28
  long *v8; // stack - 0x20
  
  sub_12769(0x269e0);
  v4 = dat_26a18;
  v8 = dat_26a18;
  if (!*dat_26a18) {
    for (v6 = 0; v6 <= 0x40; v6 = v6 + 1) {
      if (*(long *)((long)v6 * 8 + 0x26ac0)) {
        v1 = *(unsigned long *)((long)v6 * 8 + 0x23540);
        v2 = *(unsigned long *)((long)v6 * 8 + 0x26ac0);
        sub_17224("trap -- %s %s\n",sub_117ce(v2),v1);
      }
    }
    return 0;
  }
  if ((dat_26a18[1]) && (v3 = *dat_26a18, (int)sub_16d69(v3) <= -1)) {
    v8 = &v4[1];
    v7 = (char *)*v4;
  }
  else {
    v7 = NULL;
  }
  while( true ) {
    if (!*v8)
      return 0;
    v5 = sub_16db1(*v8,0);
    if (v5 <= -1) break;
    dat_243e4 += 1;
    if (v7) {
      if ((*v7 != '-') || (v7[1])) {
        if (*v7)
          dat_26cc8 += 1;
        v7 = (char *)sub_103e1(v7);
      }
      else {
        v7 = NULL;
      }
    }
    if (*(long *)((long)v5 * 8 + 0x26ac0)) {
      if (**(char **)((long)v5 * 8 + 0x26ac0))
        dat_26cc8 -= 1;
      free(*(void **)((long)v5 * 8 + 0x26ac0));
    }
    *(char **)((long)v5 * 8 + 0x26ac0) = v7;
    if (v5)
      sub_1677f(v5);
    dat_243e4 -= 1;
    if ((!dat_243e4) && (dat_243e8))
      sub_64d8(); // no-return
    v8 = &v8[1];
  }
  sub_1714e(dat_240d0,"trap: %s: bad trap\n",*v8);
  return 1;
}

// Function: sub_1677f @ 0x1677f
void sub_1677f(int a0) // ternary
{
  char v1;
  int v2;
  char *v3;
  void *v4; // stack - 0xa8
  char v5 [128];
  char v6; // stack - 0xb9
  unsigned int v7; // stack - 0x20
  
  v2 = dat_2664c;
  v3 = *(char **)((long)a0 * 8 + 0x26ac0);
  if (v3) { // branch-flip
    if (*v3) // branch-flip
      v1 = '\x02';
    else {
      v1 = '\x03';
    }
  }
  else {
    v1 = '\x01';
  }
  if (((!dat_266e4) && (v1 == '\x01')) && (!dat_2664c)) {
    switch(a0) {
      case 2:
        if (((dat_26a43) || (dat_26a30)) || (!dat_26a46))
          v1 = '\x02';
        break;
      case 3:
      case 0xf:
        if (dat_26a43)
          v1 = '\x03';
        break;
      case 0x14:
      case 0x16:
        if (dat_26a44)
          v1 = '\x03';
      
    }
  }
  if (a0 == 0x11)
    v1 = '\x02';
  v3 = (char *)((long)(a0 + -1) + 0x26ce0);
  v6 = *v3;
  if (!v6) {
    if (sigaction(a0,NULL,&v4) == -1)
      return;
    if (v4 != (void *)0x1) // branch-flip
      v6 = '\x05';
    else if ((dat_26a44) && (((a0 == 0x14 || (a0 == 0x15)) || (a0 == 0x16))))
      v6 = '\x03';
    else {
      v6 = '\x04';
    }
  }
  if ((v6 != '\x04') && (v1 != v6)) {
    if (v1 != '\x02') { // branch-flip
      v4 = (v1 != '\x03') ? NULL : (void *)0x1; // branch-flip
    }
    else {
      v4 = sub_16ab4;
    }
    if (!v2)
      *v3 = v1;
    v7 = 0;
    sigfillset(v5);
    sigaction(a0,&v4,NULL);
  }
}

// Function: sub_16a48 @ 0x16a48
void sub_16a48(int a0) // return-dupe
{
  if ((*(char *)((long)(a0 + -1) + 0x26ce0) != '\x03') && (*(char *)((long)(a0 + -1) + 0x26ce0) != '\x04'))
    signal(a0,1);
  if (dat_2664c)
    return;
  *(char *)((long)(a0 + -1) + 0x26ce0) = 4;
}

// Function: sub_16ab4 @ 0x16ab4
void sub_16ab4(int a0)
{
  if (((!dat_2664c) && (((a0 != 0x11 || (dat_26d24 = 1, dat_26b48)) && (*(char *)((long)(a0 + -1) + 0x26d40) = 1, dat_26d20 = a0, a0 == 2)))) && (!dat_26ad0)) {
    if (!dat_243e4)
      sub_64d8(); // no-return
    dat_243e8 = 1;
  }
}

// Function: sub_16b38 @ 0x16b38
void sub_16b38(void) // return-dupe x2
{
  long v1;
  int v2;
  int v3; // stack - 0x24
  int v4; // stack - 0x20
  char *v5; // stack - 0x18
  
  v2 = dat_24020;
  if (!dat_26d20)
    return;
  v4 = dat_24020;
  if (dat_24020 < 0) {
    v4 = dat_24400;
    dat_24020 = dat_24400;
  }
  dat_26d20 = 0;
  v3 = 0;
  v5 = (char *)0x26d40;
  do {
    if (0x40 <= v3) {
      dat_24020 = v2;
      return;
    }
    if (*v5) {
      if (dat_243f0) {
        dat_24020 = v2;
        dat_26d20 = v3 + 1;
        return;
      }
      *v5 = '\0';
      v1 = *(long *)((long)(v3 + 1) * 8 + 0x26ac0);
      if ((v1) && (sub_6a59(v1,0), dat_243f0 != 4))
        dat_24400 = v4;
    }
    v3 += 1;
    v5 = &v5[1];
  } while( true );
}

// Function: sub_16c3e @ 0x16c3e
void sub_16c3e(int a0) // return-dupe
{
  if (a0 + 1 == dat_26d80)
    return;
  dat_26d80 = a0 + 1;
  sub_1677f(2);
  sub_1677f(3);
  sub_1677f(0xf);
}

// Function: sub_16c88 @ 0x16c88
void sub_16c88(void)
{
  long v1;
  int v2; // eax
  char v3 [200];
  long v4; // fs_offset
  unsigned long v5; // stack - 0x10
  
  v5 = *(unsigned long *)(v4 + 0x28);
  dat_24020 = dat_24400;
  v2 = _setjmp(v3);
  v1 = dat_26ac0;
  if ((!v2) && (dat_243d8 = v3, dat_26ac0)) {
    dat_26ac0 = 0;
    dat_243f0 = 0;
    sub_6a59(v1,0);
    dat_243f0 = 8;
  }
  sub_1aa1a();
  if (!_setjmp(v3))
    sub_d556(0);
  sub_170b1();
  _exit(dat_24400); // no-return
}

// Function: sub_16d69 @ 0x16d69
int sub_16d69(char *a0)
{
  int v1; // stack - 0xc
  
  v1 = -1;
  if (sub_11792(a0)) {
    v1 = atoi(a0);
    if (0x41 <= v1)
      v1 = -1;
  }
  return v1;
}

// Function: sub_16db1 @ 0x16db1
int sub_16db1(char *a0,int a1) // early-return
{
  char *v1;
  int v2; // eax
  int v3; // stack - 0xc
  
  v2 = sub_16d69(a0);
  if (0 <= v2)
    return v2;
  v3 = a1;
  while( true ) {
    if (0x41 <= v3)
      return -1;
    v1 = *(char **)((long)v3 * 8 + 0x23540);
    if (!strcasecmp(a0,v1)) break;
    v3 += 1;
  }
  return v3;
}

// Function: sub_16e27 @ 0x16e27
void sub_16e27(void *a0)
{
  char v1 [136];
  
  sigfillset(v1);
  sigprocmask(2,v1,a0);
}

// Function: sub_16e8d @ 0x16e8d
void sub_16e8d(void *a0,unsigned long a1,long *a2) // return-dupe x2
{
  void *v1;
  long v2;
  long v3;
  
  if ((unsigned long)(a2[1] - *a2) < a1) {
    v2 = a2[3];
    if (v2) {
      if (a2[2]) // branch-flip
        sub_170cb(a2);
      else {
        dat_243e4 += 1;
        v3 = a2[2];
        a2[2] = sub_10395(v3,v2);
        a2[3] = v2;
        a2[1] = a2[2] + v2;
        *a2 = a2[2];
        dat_243e4 -= 1;
        if ((!dat_243e4) && (dat_243e8))
          sub_64d8(); // no-return
      }
    }
    if ((unsigned long)(a2[1] - *a2) <= a1) {
      v2 = a2[4];
      if (!sub_176a1((int)v2,a0,a1))
        return;
      *(unsigned int *)((long)a2 + 0x24) = *(unsigned int *)((long)a2 + 0x24) | 1;
      return;
    }
  }
  v1 = (void *)*a2;
  *a2 = (long)mempcpy(v1,a0,a1);
}

// Function: sub_1701f @ 0x1701f
void sub_1701f(char *a0,unsigned long a1)
{
  sub_16e8d(a0,strlen(a0),a1);
}

// Function: sub_1705d @ 0x1705d
void sub_1705d(char a0,unsigned long a1)
{
  char v1; // stack - 0x11
  
  v1 = a0;
  sub_16e8d(&v1,1,a1);
}

// Function: sub_170b1 @ 0x170b1
void sub_170b1(void)
{
  sub_170cb(0x24060);
}

// Function: sub_170cb @ 0x170cb
void sub_170cb(long *a0)
{
  long v1;
  long v2;
  long v3;
  long v4;
  
  v1 = *a0;
  v2 = a0[2];
  if ((v1 - v2) && (0 <= (int)a0[4])) {
    *a0 = a0[2];
    v3 = a0[2];
    v4 = a0[4];
    if (sub_176a1((int)v4,v3,v1 - v2))
      *(unsigned int *)((long)a0 + 0x24) = *(unsigned int *)((long)a0 + 0x24) | 1;
  }
}

// Function: sub_1714e @ 0x1714e
void sub_1714e(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,unsigned long a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
{
  char v1; // al
  unsigned long v10; // stack - 0x90
  unsigned long v11; // stack - 0x88
  unsigned long v12; // stack - 0x78
  unsigned long v13; // stack - 0x68
  unsigned long v14; // stack - 0x58
  unsigned long v15; // stack - 0x48
  unsigned long v16; // stack - 0x38
  unsigned long v17; // stack - 0x28
  unsigned long v18; // stack - 0x18
  char v2 [16];
  unsigned int v3; // stack - 0xd8
  unsigned int v4; // stack - 0xd4
  char *v5; // stack - 0xd0
  char *v6; // stack - 0xc8
  unsigned long v7; // stack - 0xa8
  unsigned long v8; // stack - 0xa0
  unsigned long v9; // stack - 0x98
  
  if (v1) {
    v11 = a0;
    v12 = a1;
    v13 = a2;
    v14 = a3;
    v15 = a4;
    v16 = a5;
    v17 = a6;
    v18 = a7;
  }
  v3 = 0x10;
  v4 = 0x30;
  v5 = &Stack0000000000000008;
  v6 = v2;
  v7 = a10;
  v8 = a11;
  v9 = a12;
  v10 = a13;
  sub_175c9(a8,a9,&v3);
}

// Function: sub_17224 @ 0x17224
void sub_17224(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,unsigned long a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
{
  char v1; // al
  unsigned long v10; // stack - 0x98
  unsigned long v11; // stack - 0x90
  unsigned long v12; // stack - 0x88
  unsigned long v13; // stack - 0x78
  unsigned long v14; // stack - 0x68
  unsigned long v15; // stack - 0x58
  unsigned long v16; // stack - 0x48
  unsigned long v17; // stack - 0x38
  unsigned long v18; // stack - 0x28
  unsigned long v19; // stack - 0x18
  char v2 [8];
  unsigned int v3; // stack - 0xd8
  unsigned int v4; // stack - 0xd4
  char *v5; // stack - 0xd0
  char *v6; // stack - 0xc8
  unsigned long v7; // stack - 0xb0
  unsigned long v8; // stack - 0xa8
  unsigned long v9; // stack - 0xa0
  
  if (v1) {
    v12 = a0;
    v13 = a1;
    v14 = a2;
    v15 = a3;
    v16 = a4;
    v17 = a5;
    v18 = a6;
    v19 = a7;
  }
  v3 = 8;
  v4 = 0x30;
  v5 = &Stack0000000000000008;
  v6 = v2;
  v7 = a9;
  v8 = a10;
  v9 = a11;
  v10 = a12;
  v11 = a13;
  sub_175c9(dat_240c8,a8,&v3);
}

// Function: sub_172fa @ 0x172fa
unsigned long sub_172fa(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,unsigned long a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
{
  unsigned int v1;
  unsigned long v10; // stack - 0xa0
  unsigned long v11; // stack - 0x98
  unsigned long v12; // stack - 0x90
  unsigned long v13; // stack - 0x88
  unsigned long v14; // stack - 0x78
  unsigned long v15; // stack - 0x68
  unsigned long v16; // stack - 0x58
  unsigned long v17; // stack - 0x48
  unsigned long v18; // stack - 0x38
  unsigned long v19; // stack - 0x28
  char v2; // al
  unsigned long v20; // stack - 0x18
  unsigned int v3; // eax
  unsigned long v4; // rax
  char v5 [24];
  unsigned int v6; // stack - 0xd8
  unsigned int v7; // stack - 0xd4
  char *v8; // stack - 0xd0
  char *v9; // stack - 0xc8
  
  if (v2) {
    v13 = a0;
    v14 = a1;
    v15 = a2;
    v16 = a3;
    v17 = a4;
    v18 = a5;
    v19 = a6;
    v20 = a7;
  }
  v6 = 0x18;
  v7 = 0x30;
  v8 = &Stack0000000000000008;
  v9 = v5;
  v10 = a11;
  v11 = a12;
  v12 = a13;
  v3 = sub_1773b(a8,a9,a10,&v6);
  v1 = v3;
  v4 = (unsigned long)v3;
  if ((int)a9 <= (int)v1)
    v4 = a9 & 0xffffffff;
  return v4;
}

// Function: sub_173ed @ 0x173ed
int sub_173ed(unsigned long *a0,unsigned long a1,unsigned long a2,unsigned long *a3)
{
  int v1;
  unsigned long v2;
  unsigned long v3; // rax
  unsigned long v4; // stack - 0x28
  unsigned long v5; // stack - 0x20
  unsigned long v6; // stack - 0x18
  
  v4 = *a3;
  v5 = a3[1];
  v6 = a3[2];
  v1 = sub_1773b(*a0,a1,a2,&v4);
  if (0 > v1)
    sub_6621("xvsnprintf failed"); // no-return
  if ((unsigned long)(long)v1 >= a1) {
    v2 = dat_24038;
    if (dat_24038 <= (unsigned long)(long)v1)
      v2 = (long)v1;
    v3 = sub_10422(v2 + 1);
    *a0 = v3;
    v1 = sub_1773b(v3,(long)(v1 + 1),a2,a3);
  }
  return v1;
}

// Function: sub_174e6 @ 0x174e6
unsigned int sub_174e6(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,unsigned long a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
{
  char v1; // al
  unsigned long v10; // stack - 0x90
  unsigned long v11; // stack - 0x88
  unsigned long v12; // stack - 0x78
  unsigned long v13; // stack - 0x68
  unsigned long v14; // stack - 0x58
  unsigned long v15; // stack - 0x48
  unsigned long v16; // stack - 0x38
  unsigned long v17; // stack - 0x28
  unsigned long v18; // stack - 0x18
  char v2 [16];
  unsigned int v3; // stack - 0xd8
  unsigned int v4; // stack - 0xd4
  char *v5; // stack - 0xd0
  char *v6; // stack - 0xc8
  unsigned long v7; // stack - 0xa8
  unsigned long v8; // stack - 0xa0
  unsigned long v9; // stack - 0x98
  
  if (v1) {
    v11 = a0;
    v12 = a1;
    v13 = a2;
    v14 = a3;
    v15 = a4;
    v16 = a5;
    v17 = a6;
    v18 = a7;
  }
  v3 = 0x10;
  v4 = 0x30;
  v5 = &Stack0000000000000008;
  v6 = v2;
  v7 = a10;
  v8 = a11;
  v9 = a12;
  v10 = a13;
  return sub_173ed(a8,0,a9,&v3);
}

// Function: sub_175c9 @ 0x175c9
void sub_175c9(long *a0,unsigned long a1,unsigned long a2)
{
  int v1; // eax
  int v2; // eax
  char v3 [24];
  long v4; // stack - 0x30
  
  sub_105e6(v3);
  v4 = *a0;
  v1 = (int)a0[1] - (int)*a0;
  v2 = sub_173ed(&v4,(long)v1,a1,a2);
  if (v1 <= v2) // branch-flip
    sub_16e8d(v4,(long)v2,a0);
  else {
    *a0 = *a0 + (long)v2;
  }
  sub_10641(v3);
}

// Function: sub_176a1 @ 0x176a1
unsigned long sub_176a1(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  unsigned long v2; // stack - 0x40
  void *v3; // stack - 0x20
  unsigned long v4; // stack - 0x18
  
  v2 = a2;
  v3 = a1;
  do {
    if (!v2)
      return 0;
    v4 = v2;
    if ((long)v2 <= -1)
      v4 = 0x7fffffffffffffff;
    do {
      v1 = write(a0,v3,v4);
      if (0 <= v1) break;
    } while (*dat_266e8 == 4);
    if (v1 <= -1)
      return 0xffffffff;
    v3 = (void *)((long)v3 + v1);
    v2 -= v1;
  } while( true );
}

// Function: sub_1773b @ 0x1773b
int sub_1773b(char *a0,unsigned long a1,char *a2,void *a3)
{
  int v1; // eax
  
  dat_243e4 += 1;
  v1 = vsnprintf(a0,a1,a2,a3);
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
  return v1;
}

// Function: sub_177ae @ 0x177ae
char * sub_177ae(char a0,char *a1)
{
  char *v1; // stack - 0x18
  
  v1 = a1;
  if (a1 == dat_24050)
    v1 = (char *)sub_108a8();
  *v1 = a0;
  return &v1[1];
}

// Function: sub_177ea @ 0x177ea
void sub_177ea(unsigned int a0,long *a1) // return-dupe
{
  if (*a1 == a1[1]) {
    sub_1705d(a0,a1);
    return;
  }
  *(char *)*a1 = (char)a0;
  *a1 = *a1 + 1;
}

// Function: sub_17841 @ 0x17841
int sub_17841(long a0,unsigned int *a1,unsigned int *a2,unsigned long a3) // ternary
{
  int v1; // eax
  int v2; // eax
  unsigned long v3; // rax
  unsigned char v4; // cl
  char v5 [24];
  void *v6; // stack - 0x30
  void *v7; // stack - 0x38
  int v8; // stack - 0x48
  int v9; // stack - 0x44
  
  sub_105e6(v5);
  v1 = sub_17ff3(a3,&v6);
  v7 = dat_24048;
  v2 = (int)v6 - (int)dat_24048;
  v4 = (*(char *)(a0 + 1) != 's' || v1) ? 0 : 0xff; // branch-flip
  *(unsigned char *)((long)v6 + -1) = *(unsigned char *)(a0 + 2) & v4;
  v8 = v2 + -1 + (unsigned int)(*(char *)((long)v6 + -1) != '\0');
  if (*(char *)(a0 + 1) != 's') {
    v7 = (void *)sub_1090b((long)v2,v6);
    memset(v7,0x58,(long)v8);
    *(char *)((long)v8 + (long)v7) = 0;
    v6 = dat_24048;
    if (a1 != a2) { // branch-flip
      if ((long)a1 - (long)a2 != 4) // branch-flip
        v9 = sub_174e6(&v7,a0,*a2,a2[1],v7);
      else {
        v9 = sub_174e6(&v7,a0,*a2,v7);
      }
    }
    else {
      v9 = sub_174e6(&v7,a0,v7);
    }
    v8 = v9;
    v2 = strchrnul(v7,0x58);
    v2 -= (int)v7;
    v3 = strspn((char *)((long)v2 + (long)v7),"X");
    memcpy((void *)((long)v2 + (long)v7),v6,v3);
  }
  sub_16e8d(v7,(long)v8,dat_240c8);
  sub_10641(v5);
  return v1;
}

// Function: sub_17a87 @ 0x17a87
unsigned int sub_17a87(void)
{
  unsigned int *v1;
  unsigned long v10; // xmm0_qa
  int v11; // stack - 0x60
  unsigned int v12; // stack - 0x5c
  char *v13; // stack - 0x58
  unsigned int *v14; // stack - 0x50
  char *v15; // stack - 0x48
  char *v16; // stack - 0x40
  unsigned long v17; // stack - 0x38
  unsigned long v18; // stack - 0x30
  unsigned long v19; // stack - 0x28
  char *v2;
  unsigned long v20; // stack - 0x20
  unsigned int v21; // stack - 0x14
  long *v3;
  char v4;
  unsigned int v5; // eax
  int v6; // eax
  unsigned long v7; // rax
  int v8; // stack - 0x64
  unsigned int v9; // stack - 0x18
  
  dat_26dc8 = 0;
  sub_12769(0x269e0);
  v15 = (char *)*dat_26a18;
  if (!v15)
    sub_6621("usage: printf format [arg ...]"); // no-return
  v3 = &dat_26a18[1];
  dat_26dd0 = v3;
  do {
    v13 = v15;
    while( true ) {
      v2 = &v13[1];
      v11 = (int)*v13;
      if (!v11) break;
      if (v11 != 0x5c) { // branch-flip
        if (v11 != 0x25) goto label_17b4c;
        if (*v2 == '%') {
          v2 = &v13[2];
          goto label_17b4c;
        }
        v16 = v13;
        v14 = &v9;
        v13 = v2;
        v13 = &v13[strspn(v2,"#-+ 0")];
        if (*v13 != '*') { // branch-flip
          v7 = strspn(v13,"*0123456789");
          v13 = &v13[v7];
        }
        else {
          v13 = &v13[1];
          v5 = sub_18331(1);
          v1 = &v14[1];
          *v14 = v5;
          v14 = v1;
        }
        if (*v13 == '.') {
          v2 = &v13[1];
          if (*v2 != '*') { // branch-flip
            v13 = v2;
            v13 = &v13[strspn(v2,"*0123456789")];
          }
          else {
            v13 = &v13[2];
            v5 = sub_18331(1);
            v1 = &v14[1];
            *v14 = v5;
            v14 = v1;
          }
        }
        v11 = (int)*v13;
        if (!v11)
          sub_6621("missing format character"); // no-return
        v4 = v13[1];
        v13[1] = '\0';
        switch(v11) {
          case 0x41:
          case 0x45:
          case 0x46:
          case 0x47:
          case 0x61:
          case 0x65:
          case 0x66:
          case 0x67:
            sub_18424();
            v20 = v10;
            if (v14 != &v9) { // branch-flip
              if ((long)v14 - (long)&v9 != 4) // branch-flip
                sub_17224(v16,v9,v21);
              else {
                sub_17224(v16,v9,v9,v10);
              }
            }
            else {
              sub_17224(v16);
            }
            break;
          default:
            sub_6621("%s: invalid directive",v16); // no-return
          case 0x58:
          case 0x6f:
          case 0x75:
          case 0x78:
            v19 = sub_18331(0);
            v16 = (char *)sub_1820a(v16,v13);
            if (v14 != &v9) { // branch-flip
              if ((long)v14 - (long)&v9 != 4) // branch-flip
                sub_17224(v16,v9,v21,v19);
              else {
                sub_17224(v16,v9,v19);
              }
            }
            else {
              sub_17224(v16,v19);
            }
            break;
          case 0x62:
            *v13 = 's';
            v6 = sub_17841(v16,v14,&v9,sub_182f0());
            if (v6)
              return dat_26dc8;
            *v13 = 'b';
            break;
          case 99:
            v12 = sub_182af();
            if (v14 != &v9) { // branch-flip
              if ((long)v14 - (long)&v9 != 4) // branch-flip
                sub_17224(v16,v9,v21,v12);
              else {
                sub_17224(v16,v9,v12);
              }
            }
            else {
              sub_17224(v16,v12);
            }
            break;
          case 100:
          case 0x69:
            v18 = sub_18331(1);
            v16 = (char *)sub_1820a(v16,v13);
            if (v14 != &v9) { // branch-flip
              if ((long)v14 - (long)&v9 != 4) // branch-flip
                sub_17224(v16,v9,v21,v18);
              else {
                sub_17224(v16,v9,v18);
              }
            }
            else {
              sub_17224(v16,v18);
            }
            break;
          case 0x73:
            v17 = sub_182f0();
            if (v14 != &v9) { // branch-flip
              if ((long)v14 - (long)&v9 != 4) // branch-flip
                sub_17224(v16,v9,v21,v17);
              else {
                sub_17224(v16,v9,v17);
              }
            }
            else {
              sub_17224(v16,v17);
            }
          
        }
        v13 = &v13[1];
        *v13 = v4;
      }
      else {
        v13 = v2;
        v13 = (char *)sub_180f3(v2,&v8);
        v11 = v8;
        v2 = v13;
label_17b4c:
        v13 = v2;
        sub_177ea(v11,dat_240c8);
      }
    }
    if ((v3 == dat_26dd0) || (!*dat_26dd0))
      return dat_26dc8;
  } while( true );
}

// Function: sub_17ff3 @ 0x17ff3
int sub_17ff3(char *a0,unsigned long *a1)
{
  char *v1;
  int v2; // stack - 0x20
  char *v3; // stack - 0x30
  int v4; // stack - 0x1c
  unsigned long v5; // stack - 0x18
  
  v5 = dat_24048;
  v3 = a0;
  do {
    v4 = (int)*v3;
    v1 = &v3[1];
    v2 = v4;
    if (v4 == 0x5c) {
      v1 = &v3[2];
      v2 = (int)v3[1];
      if (v2 != 99) { // branch-flip
        if (((v2 == 0x30) && ('0' <= *v1)) && (*v1 <= '7'))
          v1 = &v3[3];
        v3 = v1;
        v3 = (char *)sub_180f3(&v3[-1],&v2);
        v1 = v3;
      }
      else {
        v4 = 0x100;
        v2 = 0x100;
      }
    }
    v3 = v1;
    v5 = sub_177ae(v2,v5);
  } while ((char)v4);
  *a1 = v5;
  return v4;
}

// Function: sub_180f3 @ 0x180f3
char * sub_180f3(char *a0,int *a1)
{
  char *v1;
  char *v2; // stack - 0x20
  int v3; // stack - 0x10
  int v4; // stack - 0xc
  
  switch(*a0) {
    case 0x5c:
      v3 = 0x5c;
      break;
    default:
      if (('0' <= *a0) && (*a0 <= '7')) {
        v4 = 3;
        v3 = 0;
        v2 = a0;
        do {
          v1 = &v2[1];
          v3 = v3 * 8 + *v2 + -0x30;
          v2 = v1;
          if ((*v1 <= '/') || ('7' < *v1)) break;
          v4 -= 1;
        } while (v4);
      }
      else {
        v3 = 0x5c;
        v2 = a0;
      }
      goto label_181fb;
    case 0x61:
      v3 = 7;
      break;
    case 0x62:
      v3 = 8;
      break;
    case 0x66:
      v3 = 0xc;
      break;
    case 0x6e:
      v3 = 10;
      break;
    case 0x72:
      v3 = 0xd;
      break;
    case 0x74:
      v3 = 9;
      break;
    case 0x76:
      v3 = 0xb;
    
  }
  v2 = &a0[1];
label_181fb:
  *a1 = v3;
  return v2;
}

// Function: sub_1820a @ 0x1820a
void * sub_1820a(void *a0,char *a1)
{
  unsigned long v1; // rax
  void *v2; // rax
  
  v1 = (long)a1 - (long)a0;
  v2 = (void *)sub_1090b(v1 + 3,dat_24048);
  memcpy(v2,a0,v1);
  memcpy((void *)((long)v2 + v1),"ld",3);
  *(char *)(v1 + 1 + (long)v2) = *a1;
  return v2;
}

// Function: sub_182af @ 0x182af
int sub_182af(void)
{
  long *v1;
  int v2; // stack - 0xc
  
  v2 = 0;
  if (*dat_26dd0) {
    v1 = &dat_26dd0[1];
    v2 = (int)*(char *)*dat_26dd0;
    dat_26dd0 = v1;
  }
  return v2;
}

// Function: sub_182f0 @ 0x182f0
long sub_182f0(void)
{
  long *v1;
  long v2; // stack - 0x10
  
  v2 = 0x269e0;
  if (*dat_26dd0) {
    v1 = &dat_26dd0[1];
    v2 = *dat_26dd0;
    dat_26dd0 = v1;
  }
  return v2;
}

// Function: sub_18331 @ 0x18331
unsigned char * sub_18331(int a0) // ternary
{
  unsigned char *v1; // rax
  unsigned long v2; // stack - 0x28
  unsigned long v3; // stack - 0x20
  char *v4; // stack - 0x18
  
  v3 = 0;
  v4 = (char *)*dat_26dd0;
  if (v4) {
    v1 = (unsigned long *)&dat_26dd0[1];
    dat_26dd0 = (unsigned long *)v1;
    v1 = (unsigned char *)&v4[1];
    v1 = (unsigned long)(unsigned long)*v1;
    v3 = (unsigned long)v1;
    if ((*v4 != '\"') && (*v4 != '\'')) {
      v1 = (int *)__errno_location();
      *(int *)v1 = 0;
      v1 = (a0) ? (unsigned long)strtoimax(v4,&v2,0) : (unsigned long)strtoumax(v4,&v2,0); // branch-flip
      v3 = (unsigned long)v1;
      sub_184f9(v4,v2);
    }
  }
  v1 = (unsigned char *)v3;
  return v1;
}

// Function: sub_18424 @ 0x18424
double sub_18424(void)
{
  unsigned long v1; // stack - 0x28
  double v2; // xmm0_qa
  char *v3; // stack - 0x20
  double v4; // stack - 0x18
  
  v3 = (char *)*dat_26dd0;
  if (v3) { // branch-flip
    dat_26dd0 = &dat_26dd0[1];
    if ((*v3 != '\"') && (*v3 != '\'')) {
      *__errno_location() = 0;
      strtod(v3,&v1);
      v4 = v2;
      sub_184f9(v3,v1);
    }
    else {
      v4 = (double)(unsigned char)v3[1];
    }
  }
  else {
    v4 = 0.0;
  }
  return v4;
}

// Function: sub_184f9 @ 0x184f9
void sub_184f9(char *a0,char *a1) // return-dupe
{
  if (*a1) {
    if (a1 == a0) {
      sub_679b("%s: expected numeric value",a0);
      dat_26dc8 = 1;
      return;
    }
    sub_679b("%s: not completely converted",a0);
    dat_26dc8 = 1;
    return;
  }
  if (*__errno_location() != 0x22)
    return;
  sub_679b("%s: %s",a0,strerror(0x22));
  dat_26dc8 = 1;
}

// Function: sub_185a7 @ 0x185a7
long sub_185a7(unsigned long a0,long a1)
{
  int v1; // eax
  long v2; // rax
  long *v3; // stack - 0x38
  char *v4; // stack - 0x20
  char *v5; // stack - 0x18
  
  v4 = "%s\n";
  v3 = (long *)(a1 + 8);
  if (*v3) {
    v1 = strcmp((char *)*v3,"-n");
    if (!v1) {
      v3 = (long *)(a1 + 0x10);
      v4 = "%s";
    }
  }
  do {
    v5 = "%s ";
    v2 = *v3;
    if ((!v2) || (v3 = &v3[1], !*v3))
      v5 = v4;
    if (!v2)
      v2 = 0x269e0;
    v1 = sub_17841(v5,0,0,v2);
  } while ((!v1) && (*v3));
  v2 = 0;
  return v2;
}

// Function: sub_1867b @ 0x1867b
unsigned long sub_1867b(void)
{
  return 0;
}

// Function: sub_18686 @ 0x18686
void sub_18686(unsigned long a0)
{
  sub_1172b(a0);
}

// Function: sub_186a0 @ 0x186a0
long * sub_186a0(char *a0)
{
  char *v1;
  long *v2; // stack - 0x10
  
  v2 = (long *)0x232c0;
  while( true ) {
    if (!*v2)
      return NULL;
    v1 = (char *)*v2;
    if (!strcmp(a0,v1)) break;
    v2 = &v2[2];
  }
  return v2;
}

// Function: sub_186f5 @ 0x186f5
unsigned int sub_186f5(int a0,long *a1)
{
  int v1;
  char *v2;
  long *v3;
  long v4; // rax
  long *v5; // stack - 0x28
  int v6; // stack - 0x1c
  unsigned int v7; // stack - 0x18
  unsigned int v8; // stack - 0x14
  
  v8 = 1;
  v6 = a0;
  if (*(char *)*a1 == '[') {
    v6 = a0 + -1;
    if (*(char *)a1[v6] != ']')
      sub_6621("missing ]"); // no-return
    a1[v6] = 0;
  }
  dat_26de0 = 0;
  v5 = a1;
  do {
    v3 = &v5[1];
    v1 = v6 + -1;
    if (v1 <= 0)
      return v8;
    if (v1 != 3) {
      if (v1 == 4) goto label_187d9;
label_18876:
      v5 = v3;
      v7 = sub_19167(v5);
label_18885:
      dat_26dd8 = v5;
      if ((*dat_26dd8) && (dat_26dd8[1]))
        sub_188e2(*dat_26dd8,"unexpected operator"); // no-return
      return v8 ^ sub_18942(v7);
    }
    v4 = sub_186a0(v5[2]);
    if ((v4) && (*(short *)(v4 + 10) == 1)) {
      v7 = 0x27;
      v5 = v3;
      goto label_18885;
    }
label_187d9:
    v2 = (char *)*v3;
    if ((!strcmp(v2,"(")) && (v2 = (char *)v3[(long)v1 + -1], !strcmp(v2,")"))) {
      v3[v6 + -2] = 0;
      v3 = &v5[2];
      goto label_18876;
    }
    v2 = (char *)*v3;
    if (strcmp(v2,"!")) goto label_18876;
    v8 = 0;
    v5 = v3;
    v6 = v1;
  } while( true );
}

// Function: sub_188e2 @ 0x188e2
void sub_188e2(char *a0,unsigned long a1)
{
  if ((a0) && (*a0))
    sub_6621("%s: %s",a0,a1); // no-return
  sub_6621("%s",a1); // no-return
}

// Function: sub_18942 @ 0x18942
unsigned int sub_18942(unsigned int a0)
{
  long v1; // rax
  unsigned int v2; // stack - 0x1c
  unsigned int v3; // stack - 0xc
  
  v3 = 0;
  v2 = a0;
  while( true ) {
    v3 |= sub_189ad(v2);
    v1 = dat_26dd8 + 8;
    if (sub_19167(v1) != 0x24) break;
    dat_26dd8 += 0x10;
    v2 = sub_19167(dat_26dd8);
  }
  return v3;
}

// Function: sub_189ad @ 0x189ad
unsigned int sub_189ad(unsigned int a0)
{
  long v1; // rax
  unsigned int v2; // stack - 0x1c
  unsigned int v3; // stack - 0xc
  
  v3 = 1;
  v2 = a0;
  while( true ) {
    if (!sub_18a20(v2))
      v3 = 0;
    v1 = dat_26dd8 + 8;
    if (sub_19167(v1) != 0x23) break;
    dat_26dd8 += 0x10;
    v2 = sub_19167(dat_26dd8);
  }
  return v3;
}

// Function: sub_18a20 @ 0x18a20
unsigned long sub_18a20(int a0)
{
  int v1;
  int v2; // eax
  unsigned long v3; // rax
  
  if (a0 != 0x22) // branch-flip
    v3 = sub_18a83(a0);
  else {
    v3 = dat_26dd8 + 8;
    v2 = sub_19167(v3);
    v1 = v2;
    if (v2) {
      v3 = dat_26dd8 + 8;
      dat_26dd8 = v3;
    }
    v2 = sub_18a20(v1);
    v3 = (unsigned long)(v2 == 0);
  }
  return v3;
}

// Function: sub_18a83 @ 0x18a83
long * sub_18a83(int a0) // early-return x2
{
  bool v1;
  unsigned int v2;
  unsigned int v3; // eax
  long *v4; // rax
  
  if (!a0) {
    v4 = NULL;
    return v4;
  }
  if (a0 != 0x25) { // branch-flip
    if ((dat_26de0) && (!*(short *)((long)dat_26de0 + 10))) {
      v4 = &dat_26dd8[1];
      dat_26dd8 = v4;
      if (!*v4)
        sub_188e2(*dat_26de0,"argument expected"); // no-return
      switch(a0) {
        default:
          v4 = (unsigned long)sub_18f56(*v4,a0);
          break;
        case 1:
          v4 = (unsigned long)sub_1950f(*v4,4);
          break;
        case 2:
          v4 = (unsigned long)sub_1950f(*v4,2);
          break;
        case 3:
          v4 = (unsigned long)sub_1950f(*v4,1);
          break;
        case 0xd:
          v3 = sub_18686(*v4);
          v3 = isatty(v3);
          break;
        case 0x16:
          v4 = (unsigned long)strlen((char *)*v4);
          v4 = (unsigned long)(unsigned long)(v4 == NULL);
          break;
        case 0x17:
          v4 = (unsigned long)strlen((char *)*v4);
          v4 = (unsigned long)(unsigned long)(v4 != NULL);
        
      }
    }
    else {
      v4 = &dat_26dd8[1];
      sub_19167(v4);
      if ((dat_26de0) && (*(short *)((long)dat_26de0 + 10) == 1))
        v1 = 1;
      else {
        v1 = 0;
      }
      if (v1)
        v4 = (unsigned long)sub_18cea();
      else {
        v4 = (unsigned long)strlen((char *)*dat_26dd8);
        v4 = (unsigned long)(unsigned long)(v4 != NULL);
      }
    }
  }
  else {
    v4 = &dat_26dd8[1];
    dat_26dd8 = v4;
    v3 = sub_19167(v4);
    if (v3 == 0x26) {
      v4 = NULL;
      return v4;
    }
    v3 = sub_18942(v3);
    v2 = v3;
    v4 = &dat_26dd8[1];
    dat_26dd8 = v4;
    v3 = sub_19167(v4);
    if (v3 != 0x26)
      sub_188e2(0,"closing paren expected"); // no-return
    v4 = (unsigned long)(unsigned long)v2;
  }
  return v4;
}

// Function: sub_18cea @ 0x18cea
unsigned long * sub_18cea(void)
{
  char *v1;
  char *v2;
  long v3;
  unsigned int v4; // eax
  unsigned long *v5; // rax
  
  v1 = (char *)*dat_26dd8;
  v5 = &dat_26dd8[1];
  dat_26dd8 = v5;
  sub_19167(v5);
  v5 = &dat_26dd8[1];
  v2 = (char *)*v5;
  dat_26dd8 = v5;
  if (v2) {
    switch(*(unsigned short *)&dat_26de0[1]) {
      case 0x11:
        v5 = (unsigned long)sub_19289(v1,v2);
        break;
      case 0x12:
        v5 = (unsigned long)sub_19341(v1,v2);
        break;
      case 0x13:
        v5 = (unsigned long)sub_193f9(v1,v2);
        break;
      default:
        v4 = strcmp(v1,v2);
        v5 = (unsigned long)(unsigned long)(v4 == 0);
        break;
      case 0x19:
        v4 = strcmp(v1,v2);
        v5 = (unsigned long)(unsigned long)(v4 != 0);
        break;
      case 0x1a:
        v4 = strcmp(v1,v2);
        v4 >>= 0x1f;
        v5 = (unsigned long)(unsigned long)v4;
        break;
      case 0x1b:
        v4 = strcmp(v1,v2);
        v5 = (unsigned long)(unsigned long)(0 < (int)v4);
        break;
      case 0x1c:
        v5 = (long)sub_18686(v1);
        v3 = (long)v5;
        v5 = (long)sub_18686(v2);
        v5 = (unsigned long)(unsigned long)((unsigned long *)v3 == v5);
        break;
      case 0x1d:
        v5 = (long)sub_18686(v1);
        v3 = (long)v5;
        v5 = (long)sub_18686(v2);
        v5 = (unsigned long)(unsigned long)((unsigned long *)v3 != v5);
        break;
      case 0x1e:
        v5 = (long)sub_18686(v1);
        v3 = (long)v5;
        v5 = (long)sub_18686(v2);
        v5 = (unsigned long)(unsigned long)((long)v5 <= v3);
        break;
      case 0x1f:
        v5 = (long)sub_18686(v1);
        v3 = (long)v5;
        v5 = (long)sub_18686(v2);
        v5 = (unsigned long)(unsigned long)((long)v5 < v3);
        break;
      case 0x20:
        v5 = (long)sub_18686(v1);
        v3 = (long)v5;
        v5 = (long)sub_18686(v2);
        v5 = (unsigned long)(unsigned long)(v3 <= (long)v5);
        break;
      case 0x21:
        v5 = (long)sub_18686(v1);
        v3 = (long)v5;
        v5 = (long)sub_18686(v2);
        v5 = (unsigned long)(unsigned long)(v3 < (long)v5);
      
    }
    return v5;
  }
  sub_188e2(*dat_26de0,"argument expected"); // no-return
}

// Function: sub_18f56 @ 0x18f56
bool sub_18f56(unsigned long a0,int a1) // early-return x3
{
  bool v1; // al
  unsigned int v2; // eax
  char v3 [24];
  unsigned int v4; // stack - 0xa0
  unsigned int v5; // stack - 0x9c
  int v6; // stack - 0x98
  long v7; // stack - 0x88
  
  if (a1 != 0xb) // branch-flip
    v2 = stat64(a0,v3);
  else {
    v2 = lstat64(a0,v3);
  }
  v1 = v2 == 0;
  if (v1) {
    switch(a1) {
      case 4:
        v1 = 1;
        return v1;
      case 5:
        v2 = v4 & 0xf000;
        v1 = v2 == 0x8000;
        break;
      case 6:
        v2 = v4 & 0xf000;
        v1 = v2 == 0x4000;
        break;
      case 7:
        v2 = v4 & 0xf000;
        v1 = v2 == 0x2000;
        break;
      case 8:
        v2 = v4 & 0xf000;
        v1 = v2 == 0x6000;
        break;
      case 9:
        v2 = v4 & 0xf000;
        v1 = v2 == 0x1000;
        break;
      case 10:
        v2 = v4 & 0xf000;
        v1 = v2 == 0xc000;
        break;
      case 0xb:
        v2 = v4 & 0xf000;
        v1 = v2 == 0xa000;
        break;
      case 0xc:
        v1 = v7 != 0;
        break;
      default:
        v1 = 1;
        return v1;
      case 0xe:
        v2 = v4 & 0x800;
        v1 = v2 != 0;
        break;
      case 0xf:
        v2 = v4 & 0x400;
        v1 = v2 != 0;
        break;
      case 0x10:
        v2 = v4 & 0x200;
        v1 = v2 != 0;
        break;
      case 0x14:
        v2 = geteuid();
        v1 = v5 == v2;
        break;
      case 0x15:
        v2 = getegid();
        v1 = v6 == v2;
      
    }
    return v1;
  }
  v1 = 0;
  return v1;
}

// Function: sub_19167 @ 0x19167
int sub_19167(long *a0)
{
  long v1; // rax
  
  if (!*a0) {
    dat_26de0 = 0;
    return 0;
  }
  v1 = sub_186a0(*a0);
  if (((v1) && ((*(short *)(v1 + 10) || (!sub_19215(a0))))) && ((*(short *)(v1 + 8) != 0x25 || (a0[1])))) {
    dat_26de0 = v1;
    return (int)*(short *)(v1 + 8);
  }
  dat_26de0 = 0;
  return 0x27;
}

// Function: sub_19215 @ 0x19215
unsigned long sub_19215(long a0) // return-dupe x3
{
  long v1; // rax
  
  if (!*(long *)(a0 + 8))
    return 1;
  if (!*(long *)(a0 + 0x10))
    return 0;
  v1 = sub_186a0(*(long *)(a0 + 8));
  if ((v1) && (*(short *)(v1 + 10) == 1))
    return 1;
  return 0;
}

// Function: sub_19289 @ 0x19289
unsigned long sub_19289(unsigned long a0,unsigned long a1) // early-return
{
  char v1 [88];
  char v2 [88];
  long v3; // stack - 0xe0
  long v4; // stack - 0xd8
  long v5; // stack - 0x50
  long v6; // stack - 0x48
  
  if (((!stat64(a0,v1)) && (!stat64(a1,v2))) && ((v5 < v3 || ((v3 == v5 && (v6 < v4))))))
    return 1;
  return 0;
}

// Function: sub_19341 @ 0x19341
unsigned long sub_19341(unsigned long a0,unsigned long a1) // early-return
{
  char v1 [88];
  char v2 [88];
  long v3; // stack - 0xe0
  long v4; // stack - 0xd8
  long v5; // stack - 0x50
  long v6; // stack - 0x48
  
  if (((!stat64(a0,v1)) && (!stat64(a1,v2))) && ((v3 < v5 || ((v3 == v5 && (v4 < v6))))))
    return 1;
  return 0;
}

// Function: sub_193f9 @ 0x193f9
unsigned long sub_193f9(unsigned long a0,unsigned long a1) // return-dupe
{
  long v1; // stack - 0x138
  long v2; // stack - 0xa8
  long v3; // stack - 0x130
  long v4; // stack - 0xa0
  
  if ((((!stat64(a0,&v1)) && (!stat64(a1,&v2))) && (v1 == v2)) && (v3 == v4))
    return 1;
  return 0;
}

// Function: sub_194a7 @ 0x194a7
unsigned int sub_194a7(unsigned long a0) // early-return
{
  char v1 [24];
  unsigned int v2; // stack - 0x90
  
  if (stat64(a0,v1))
    return 0;
  return v2 & 0x49;
}

// Function: sub_1950f @ 0x1950f
bool sub_1950f(unsigned long a0,int a1)
{
  if ((((sub_1867b()) && (a1 == 1)) && (!geteuid())) && (!sub_194a7(a0)))
    return 0;
  return faccessat(0xffffff9c,a0,a1,0x200) == 0;
}

// Function: sub_19574 @ 0x19574
unsigned long sub_19574(unsigned long a0,unsigned long a1)
{
  int v1; // eax
  int v2; // eax
  long v3; // rax
  long v4; // stack - 0x38
  long v5; // stack - 0x30
  long v6; // stack - 0x28
  long v7; // stack - 0x20
  
  v3 = sysconf(2);
  times(&v4);
  v1 = (int)(((double)v6 / (double)v3) / dat_1d5d0);
  v2 = (int)(((double)v7 / (double)v3) / dat_1d5d0);
  sub_17224(a0,a1,(double)v6 / (double)v3 - (double)v1 * dat_1d5d0,(double)v7 / (double)v3 - (double)v2 * dat_1d5d0,"%dm%fs %dm%fs\n%dm%fs %dm%fs\n",(int)(((double)v4 / (double)v3) / dat_1d5d0),(int)(((double)v5 / (double)v3) / dat_1d5d0),v1,v2);
  return 0;
}

// Function: sub_19779 @ 0x19779
int sub_19779(int a0)
{
  return (int)(dat_1d6b0 + (double)(a0 * 8 + -1) * dat_1d6a8);
}

// Function: sub_197af @ 0x197af
bool sub_197af(unsigned long a0,unsigned long a1)
{
  return sub_1a729(a0,a1) == 0;
}

// Function: sub_197dc @ 0x197dc
void sub_197dc(void) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long *v2; // stack - 0x20
  
  v2 = (unsigned long *)0x24120;
  do {
    v1 = (unsigned long *)sub_1a6a3(v2[2]);
    *v2 = *v1;
    *v1 = (unsigned long)v2;
    v2 = &v2[4];
  } while (v2 < s_24240);
  if (geteuid())
    return;
  dat_241b0 = "PS1=# ";
}

// Function: sub_1985b @ 0x1985b
unsigned long sub_1985b(void *a0,char *a1,unsigned int a2)
{
  long v1; // rax
  long v2; // rax
  unsigned long v3; // rax
  void *v4; // rax
  unsigned long v5; // rax
  unsigned int v6; // stack - 0x4c
  char *v7; // stack - 0x38
  unsigned long v8; // stack - 0x30
  
  v1 = sub_155e7(a0);
  v2 = strchrnul(v1,0x3d);
  v3 = v2 - (long)a0;
  if ((!v3) || (v2 != v1))
    sub_6621("%.*s: bad variable name",v3,a0); // no-return
  v8 = 0;
  if (a1) { // branch-flip
    v8 = strlen(a1);
    v6 = a2;
  }
  else {
    v6 = a2 | 0x20;
  }
  dat_243e4 += 1;
  v4 = (void *)sub_10354(v8 + v3 + 2);
  v7 = mempcpy(v4,a0,v3);
  if (a1) {
    *v7 = 0x3d;
    v7 = mempcpy(&v7[1],a1,v8);
  }
  *v7 = 0;
  v5 = sub_19b05(v4,v6 | 0x100);
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
  return v5;
}

// Function: sub_199bd @ 0x199bd
unsigned long sub_199bd(unsigned long a0,unsigned long a1,unsigned int a2)
{
  long v1;
  long v10; // stack - 0x50
  char *v11; // stack - 0x48
  int v2; // eax
  unsigned long v3; // rax
  char v4 [12];
  char *v5; // rsp
  unsigned int v6; // stack - 0x6c
  unsigned long v7; // stack - 0x68
  unsigned long v8; // stack - 0x60
  int v9; // stack - 0x54
  
  v6 = a2;
  v7 = a1;
  v8 = a0;
  v2 = sub_19779(8);
  v9 = v2;
  v10 = (long)v2 + -1;
  v3 = (((long)v2 + 0xfU) / 0x10) * 0x10;
  for (v5 = v4; v5 != &v4[-(v3 & 0xfffffffffffff000)]; v5 = &v5[-0x1000]) {
    *(unsigned long *)&v5[-8] = *(unsigned long *)&v5[-8];
  }
  v1 = -(unsigned long)((unsigned int)v3 & 0xfff);
  if (v3 & 0xfff)
    *(unsigned long *)&v5[((unsigned long)((unsigned int)v3 & 0xfff) - 8) + v1] = *(unsigned long *)&v5[((unsigned long)((unsigned int)v3 & 0xfff) - 8) + v1];
  v11 = &v5[v1];
  *(unsigned long *)&v5[v1 + -8] = 0x19ac5;
  sub_172fa((long)v5 + v1,(long)v2,"%ld",v7);
  *(unsigned long *)&v5[v1 + -8] = 0x19adb;
  sub_1985b(v8,v11,v6);
  return v7;
}

// Function: sub_19b05 @ 0x19b05
unsigned long * sub_19b05(void *a0,unsigned int a1)
{
  long v1;
  void *v2;
  unsigned long v3; // rax
  unsigned long *v4; // rax
  unsigned int v5; // stack - 0x44
  void *v6; // stack - 0x40
  unsigned long *v7; // stack - 0x30
  
  v3 = sub_1a6a3(a0);
  v5 = a1 | -(int)dat_26a4c & 1U;
  v4 = (unsigned long *)sub_1a7ca(v3,a0);
  v7 = (unsigned long *)*v4;
  if (v7) { // branch-flip
    if (*(unsigned int *)&v7[1] & 2) {
      if (v5 & 0x100)
        free(a0);
      v1 = v7[2];
      sub_6621("%.*s: is read only",strchrnul(v1,0x3d) - v1,v1); // no-return
    }
    if (v5 & 0x80)
      return v7;
    if ((v7[3]) && (!(v5 & 0x40))) {
      v2 = (void *)v7[3];
      (*v2)(strchrnul(a0,0x3d) + 1);
    }
    if (!(*(unsigned int *)&v7[1] & 0x18))
      free((void *)v7[2]);
    if ((*(unsigned int *)&v7[1] & 4 | v5 & 0x27) != 0x20) {
      v5 |= *(unsigned int *)&v7[1] & 0xfffffec7;
label_19ced:
      v6 = a0;
      if (!(v5 & 0x118))
        v6 = (void *)sub_103e1(a0);
      v7[2] = v6;
      *(unsigned int *)&v7[1] = v5;
      return v7;
    }
    *v4 = *v7;
    free(v7);
  }
  else {
    if (v5 & 0x80)
      return NULL;
    if ((v5 & 0x27) != 0x20) {
      v7 = (unsigned long *)sub_10354(0x20);
      *v7 = *v4;
      v7[3] = 0;
      *v4 = v7;
      goto label_19ced;
    }
  }
  if ((v5 & 0x118) == 0x100)
    free(a0);
  return v7;
}

// Function: sub_19d32 @ 0x19d32
long sub_19d32(unsigned long a0) // early-return
{
  long v1;
  unsigned long v2;
  
  v2 = sub_1a6a3(a0);
  v1 = *(long *)sub_1a7ca(v2,a0);
  if ((v1) && (!(*(unsigned int *)(v1 + 8) & 0x20))) {
    if ((v1 == 0x24220) && (dat_24230 == s_24100))
      sub_172fa((char *)0x24107,0xc,"%d",dat_26e08);
    v2 = *(unsigned long *)(v1 + 0x10);
    return strchrnul(v2,0x3d) + 1;
  }
  return 0;
}

// Function: sub_19de7 @ 0x19de7
void sub_19de7(unsigned long a0)
{
  long v1; // rax
  
  v1 = sub_19d32(a0);
  if (!v1)
    v1 = 0x269e0;
  sub_1164c(v1,0);
}

// Function: sub_19e1e @ 0x19e1e
void sub_19e1e(unsigned int a0,unsigned int a1,unsigned long *a2)
{
  unsigned long *v1;
  unsigned long *v2; // stack - 0x20
  unsigned long *v3; // stack - 0x18
  unsigned long *v4; // stack - 0x10
  
  v4 = dat_24048;
  v2 = (unsigned long *)0x26e20;
  do {
    for (v3 = (unsigned long *)*v2; v3; v3 = (unsigned long *)*v3) {
      if (a0 == (*(unsigned int *)&v3[1] & (a0 | a1))) {
        if (v4 == dat_24050)
          v4 = (unsigned long *)sub_108a8();
        v1 = &v4[1];
        *v4 = v3[2];
        v4 = v1;
      }
    }
    v2 = &v2[1];
  } while (v2 < (unsigned long *)0x26f58);
  if (v4 == dat_24050)
    v4 = (unsigned long *)sub_108a8();
  if (a2)
    *a2 = v4;
  *v4 = 0;
  sub_10422((long)v4 + (8U - (long)dat_24048));
}

// Function: sub_19f13 @ 0x19f13
unsigned long sub_19f13(char *a0,unsigned int a1,unsigned int a2)
{
  unsigned long *v1; // stack - 0x38
  unsigned long *v2; // stack - 0x30
  char *v3; // stack - 0x28
  unsigned long v4; // stack - 0x20
  char *v5; // stack - 0x18
  
  v2 = (unsigned long *)sub_19e1e(a1,a2,&v1);
  qsort(v2,(long)v1 - (long)v2 >> 3,8,sub_1a79b);
  if (*a0) // branch-flip
    v5 = " ";
  else {
    v5 = a0;
  }
  for (; v2 < v1; v2 = &v2[1]) {
    v3 = (char *)strchrnul(*v2,0x3d);
    v4 = 0x269e0;
    if (*v3) {
      v3 = &v3[1];
      v4 = sub_117ce(v3);
    }
    sub_17224("%s%s%.*s%s\n",a0,v5,(int)v3 - (int)*v2,*v2,v4);
  }
  return 0;
}

// Function: sub_1a04d @ 0x1a04d
unsigned long sub_1a04d(unsigned long a0,unsigned long *a1) // return-dupe, ternary
{
  long v1;
  unsigned int v2; // eax
  unsigned long v3; // rax
  char *v4; // stack - 0x28
  char *v5; // stack - 0x20
  long *v6; // stack - 0x18
  
  v2 = (*(char *)*a1 != 'r') ? 1 : 2; // branch-flip
  if (sub_12769("p") != 0x70) {
    v6 = dat_26a18;
    v4 = (char *)*dat_26a18;
    if (v4) {
      do {
        v5 = strchr(v4,0x3d);
        if (v5) { // branch-flip
          v5 = &v5[1];
label_1a121:
          sub_1985b(v4,v5,v2);
        }
        else {
          v3 = sub_1a6a3(v4);
          v1 = *(long *)sub_1a7ca(v3,v4);
          if (!v1) goto label_1a121;
          *(unsigned int *)(v1 + 8) = *(unsigned int *)(v1 + 8) | v2;
        }
        v6 = &v6[1];
        v4 = (char *)*v6;
        if (!v4)
          return 0;
      } while( true );
    }
  }
  sub_19f13(*a1,v2,0);
  return 0;
}

// Function: sub_1a174 @ 0x1a174
unsigned long sub_1a174(void)
{
  long *v1;
  long *v2; // stack - 0x28
  
  if (!dat_26e00)
    sub_6621("not in a function"); // no-return
  v2 = dat_26a18;
  while( true ) {
    v1 = &v2[1];
    if (!*v2) break;
    sub_1a1e6(*v2,0);
    v2 = v1;
  }
  return 0;
}

// Function: sub_1a1e6 @ 0x1a1e6
void sub_1a1e6(char *a0,unsigned int a1) // ternary
{
  unsigned long *v1; // rax
  void *v2; // rax
  unsigned long v3; // rax
  char *v4; // rax
  long v5; // stack - 0x30
  
  dat_243e4 += 1;
  v1 = (unsigned long *)sub_10354(0x20);
  if ((*a0 != '-') || (a0[1])) {
    v3 = sub_1a6a3(a0);
    v5 = *(long *)sub_1a7ca(v3,a0);
    v4 = strchr(a0,0x3d);
    if (v5) { // branch-flip
      v1[3] = *(unsigned long *)(v5 + 0x10);
      *(unsigned int *)&v1[2] = *(unsigned int *)(v5 + 8);
      *(unsigned int *)(v5 + 8) = *(unsigned int *)(v5 + 8) | 0xc;
      if (v4)
        sub_19b05(a0,a1);
    }
    else {
      v5 = (v4) ? sub_19b05(a0,a1 | 4) : sub_1985b(a0,0,a1 | 4); // branch-flip
      *(unsigned int *)&v1[2] = 0x20;
    }
  }
  else {
    v2 = (void *)sub_10354(0x11);
    v1[3] = memcpy(v2,(void *)0x26a40,0x11);
    v5 = 0;
  }
  v1[1] = v5;
  *v1 = *(unsigned long *)(dat_26e00 + 8);
  *(unsigned long **)(dat_26e00 + 8) = v1;
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
}

// Function: sub_1a3a2 @ 0x1a3a2
void sub_1a3a2(void)
{
  unsigned long v1;
  unsigned long *v2;
  long v3;
  void *v4;
  unsigned long *v5; // stack - 0x38
  
  v2 = dat_26e00;
  dat_243e4 += 1;
  v1 = *dat_26e00;
  v5 = (unsigned long *)dat_26e00[1];
  dat_26e00 = (unsigned long *)v1;
  free(v2);
  while (v5) {
    v2 = (unsigned long *)*v5;
    v3 = v5[1];
    if (v3) { // branch-flip
      if (*(int *)&v5[2] != 0x20) { // branch-flip
        if (*(long *)(v3 + 0x18)) {
          v4 = *(void **)(v3 + 0x18);
          v1 = v5[3];
          (*v4)(strchrnul(v1,0x3d) + 1);
        }
        if (!(*(unsigned int *)(v3 + 8) & 0x18))
          free(*(void **)(v3 + 0x10));
        *(unsigned int *)(v3 + 8) = *(unsigned int *)&v5[2];
        *(unsigned long *)(v3 + 0x10) = v5[3];
      }
      else {
        *(unsigned int *)(v3 + 8) = *(unsigned int *)(v3 + 8) & 0xfffffff9;
        sub_1a67a(*(unsigned long *)(v3 + 0x10));
      }
    }
    else {
      memcpy((void *)0x26a40,(void *)v5[3],0x11);
      free((void *)v5[3]);
      sub_11be5();
    }
    free(v5);
    v5 = v2;
  }
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
}

// Function: sub_1a539 @ 0x1a539
unsigned long * sub_1a539(int a0)
{
  unsigned long *v1;
  
  v1 = dat_26e00;
  if (a0) {
    dat_243e4 += 1;
    dat_26e00 = (unsigned long *)sub_10354(0x10);
    dat_26e00[1] = 0;
    *dat_26e00 = v1;
    dat_243e4 -= 1;
    if ((!dat_243e4) && (dat_243e8))
      sub_64d8(); // no-return
  }
  return v1;
}

// Function: sub_1a5c7 @ 0x1a5c7
void sub_1a5c7(long a0)
{
  while (a0 != dat_26e00) {
    sub_1a3a2();
  }
}

// Function: sub_1a5ef @ 0x1a5ef
unsigned long sub_1a5ef(void)
{
  int v1; // eax
  int v2; // stack - 0x18
  long *v3; // stack - 0x10
  
  v2 = 0;
  while (v1 = sub_12769("vf"), v1) {
    v2 = v1;
  }
  for (v3 = dat_26a18; *v3; v3 = &v3[1]) {
    if (v2 != 0x66) // branch-flip
      sub_1a67a(*v3);
    else {
      sub_9ac3(*v3);
    }
  }
  return 0;
}

// Function: sub_1a67a @ 0x1a67a
void sub_1a67a(unsigned long a0)
{
  sub_1985b(a0,0,0);
}

// Function: sub_1a6a3 @ 0x1a6a3
int sub_1a6a3(unsigned char *a0)
{
  unsigned char *v1; // stack - 0x20
  unsigned int v2; // stack - 0xc
  
  v2 = (unsigned int)*a0 << 4;
  for (v1 = a0; (*v1 && (*v1 != 0x3d)); v1 = &v1[1]) {
    v2 += *v1;
  }
  return (v2 % 0x27) * 8 + 0x26e20;
}

// Function: sub_1a729 @ 0x1a729
int sub_1a729(char *a0,char *a1)
{
  char *v1; // stack - 0x28
  char *v2; // stack - 0x20
  int v3; // stack - 0x10
  int v4; // stack - 0xc
  
  v1 = a1;
  v2 = a0;
  while( true ) {
    v3 = (int)*v2;
    v4 = (int)*v1;
    if (v3 != v4) break;
    if ((!v3) || (v3 == 0x3d)) {
      return v3 - v4;
    }
    v2 = &v2[1];
    v1 = &v1[1];
  }
  if (v3 == 0x3d)
    v3 = 0;
  if (v4 == 0x3d)
    v4 = 0;
  return v3 - v4;
}

// Function: sub_1a79b @ 0x1a79b
void sub_1a79b(unsigned long *a0,unsigned long *a1)
{
  sub_1a729(*a0,*a1);
}

// Function: sub_1a7ca @ 0x1a7ca
long * sub_1a7ca(long *a0,unsigned long a1)
{
  unsigned long v1;
  long *v2; // stack - 0x10
  
  v2 = a0;
  while ((*v2 && (v1 = *(unsigned long *)(*v2 + 0x10), !sub_197af(v1,a1)))) {
    v2 = (long *)*v2;
  }
  return v2;
}

// Function: sub_1a81e @ 0x1a81e
void sub_1a81e(void)
{
  char *v1; // rax
  long v2; // stack - 0x138
  long v3; // stack - 0xa8
  long *v4; // stack - 0x148
  char *v5; // stack - 0x140
  long v6; // stack - 0x130
  long v7; // stack - 0xa0
  
  dat_245c0 = 0x24640;
  dat_245b8 = 0x24640;
  dat_245a8 = 1;
  dat_26cf0 = 1;
  sub_1677f(0x11);
  sub_197dc();
  for (v4 = _environ; *v4; v4 = &v4[1]) {
    v1 = (char *)sub_155e7(*v4);
    if ((v1 != (char *)*v4) && (*v1 == '='))
      sub_19b05(*v4,9);
  }
  sub_19b05(s_240e0,8);
  sub_19b05(s_240e8,8);
  sub_172fa((char *)0x24245,0x1b,"%ld",(long)(int)getppid());
  sub_19b05(s_24240,8);
  v5 = (char *)sub_19d32("PWD");
  if ((v5) && ((((*v5 != '/' || (stat64(v5,&v2))) || (stat64(".",&v3))) || ((v2 != v3 || (v6 != v7))))))
    v5 = NULL;
  sub_6325(v5,0);
}

// Function: sub_1aa1a @ 0x1aa1a
void sub_1aa1a(void)
{
  if (0 <= dat_24020) {
    if ((dat_243e0 == 4) || (dat_243f0 == 8))
      dat_24400 = dat_24020;
    dat_24020 = -1;
  }
  dat_243f0 = 0;
  dat_243f4 = 0;
  dat_24408 = 0;
  sub_baa8();
  sub_16440(0);
}

// Function: sub_1aa88 @ 0x1aa88
void sub_1aa88(void)
{
  long *v1; // stack - 0x10
  
  sub_d476();
  if (1 <= *(int *)(dat_24028 + 0xc)) {
    close(*(int *)(dat_24028 + 0xc));
    *(unsigned int *)(dat_24028 + 0xc) = 0;
  }
  dat_243d8 = 0x26700;
  dat_26aa0 = 0;
  dat_243e4 += 1;
  for (v1 = (long *)0x26ac0; v1 < (long *)0x26cc8; v1 = &v1[1]) {
    if ((*v1) && (*(char *)*v1)) {
      free((void *)*v1);
      *v1 = 0;
      if (v1 != (long *)0x26ac0)
        sub_1677f((long)&v1[-0x4d58] >> 3 & 0xffffffff);
    }
  }
  dat_26cc8 = 0;
  dat_243e4 -= 1;
  if ((!dat_243e4) && (dat_243e8))
    sub_64d8(); // no-return
}

// Function: sub_1ab9c @ 0x1ab9c
void sub_1ab9c(void)
{
  dat_245b0 = 0;
  dat_245b4 = 0;
  dat_24620 = 0;
  sub_d476();
  sub_1a5c7(0);
}

// Function: sub_1abd6 @ 0x1abd6
unsigned int * sub_1abd6(unsigned long a0)
{
  long v1; // rax
  unsigned int *v2; // rax
  
  dat_26f58 = 8;
  dat_26f5c = 0;
  sub_1ac6b(a0);
  v1 = (long)dat_26f58;
  v2 = (unsigned int *)sub_10354(v1 + dat_26f5c);
  dat_26f60 = &v2[2];
  dat_26f68 = v1 + (long)v2;
  sub_1af8a(a0);
  *v2 = 0;
  return v2;
}

// Function: sub_1ac6b @ 0x1ac6b
void sub_1ac6b(int *a0) // return-dupe x15
{
  char *v1;
  
  if (!a0)
    return;
  dat_26f58 += *(short *)&" "[(long)*a0 * 2];
  switch(*a0) {
    case 0:
      sub_1ac6b(*(unsigned long *)&a0[6]);
      sub_1ac6b(*(unsigned long *)&a0[4]);
      sub_1ac6b(*(unsigned long *)&a0[2]);
      return;
    case 1:
      sub_1af43(*(unsigned long *)&a0[2]);
      return;
    case 2:
    case 3:
    case 4:
      sub_1ac6b(*(unsigned long *)&a0[4]);
      sub_1ac6b(*(unsigned long *)&a0[2]);
      return;
    case 5:
    case 6:
    case 7:
    case 9:
    case 10:
      sub_1ac6b(*(unsigned long *)&a0[4]);
      sub_1ac6b(*(unsigned long *)&a0[2]);
      return;
    case 8:
      sub_1ac6b(*(unsigned long *)&a0[6]);
      sub_1ac6b(*(unsigned long *)&a0[4]);
      sub_1ac6b(*(unsigned long *)&a0[2]);
      return;
    case 0xb:
      v1 = *(char **)&a0[6];
      dat_26f5c = dat_26f5c + (int)strlen(v1) + 1;
      sub_1ac6b(*(unsigned long *)&a0[4]);
      sub_1ac6b(*(unsigned long *)&a0[2]);
      return;
    case 0xc:
      sub_1ac6b(*(unsigned long *)&a0[4]);
      sub_1ac6b(*(unsigned long *)&a0[2]);
      return;
    case 0xd:
      sub_1ac6b(*(unsigned long *)&a0[6]);
      sub_1ac6b(*(unsigned long *)&a0[4]);
      sub_1ac6b(*(unsigned long *)&a0[2]);
      return;
    case 0xe:
      sub_1ac6b(*(unsigned long *)&a0[4]);
      v1 = *(char **)&a0[2];
      dat_26f5c = dat_26f5c + (int)strlen(v1) + 1;
      return;
    case 0xf:
      sub_1af43(*(unsigned long *)&a0[6]);
      v1 = *(char **)&a0[4];
      dat_26f5c = dat_26f5c + (int)strlen(v1) + 1;
      sub_1ac6b(*(unsigned long *)&a0[2]);
      return;
    case 0x10:
    case 0x11:
    case 0x12:
    case 0x13:
    case 0x14:
      sub_1ac6b(*(unsigned long *)&a0[6]);
      sub_1ac6b(*(unsigned long *)&a0[2]);
      return;
    case 0x15:
    case 0x16:
      sub_1ac6b(*(unsigned long *)&a0[6]);
      sub_1ac6b(*(unsigned long *)&a0[2]);
      return;
    case 0x17:
    case 0x18:
      sub_1ac6b(*(unsigned long *)&a0[6]);
      sub_1ac6b(*(unsigned long *)&a0[2]);
      return;
    case 0x19:
      sub_1ac6b(*(unsigned long *)&a0[2]);
      return;
    default:
      return;
    
  }
}

// Function: sub_1af43 @ 0x1af43
void sub_1af43(unsigned long *a0)
{
  unsigned long *v1; // stack - 0x10
  
  for (v1 = a0; v1; v1 = (unsigned long *)*v1) {
    dat_26f58 += 0x10;
    sub_1ac6b(v1[1]);
  }
}

// Function: sub_1af8a @ 0x1af8a
int * sub_1af8a(int *a0) // early-return
{
  unsigned long v1;
  int *v2;
  
  v2 = dat_26f60;
  if (!a0)
    return NULL;
  dat_26f60 = (long)((long)*(short *)&" "[(long)*a0 * 2] + (long)dat_26f60);
  switch(*a0) {
    case 0:
      v1 = *(unsigned long *)&a0[6];
      *(unsigned long *)&v2[6] = sub_1af8a(v1);
      v1 = *(unsigned long *)&a0[4];
      *(unsigned long *)&v2[4] = sub_1af8a(v1);
      v1 = *(unsigned long *)&a0[2];
      *(unsigned long *)&v2[2] = sub_1af8a(v1);
      v2[1] = a0[1];
      break;
    case 1:
      v1 = *(unsigned long *)&a0[2];
      *(unsigned long *)&v2[2] = sub_1b3d6(v1);
      v2[1] = a0[1];
      break;
    case 2:
    case 3:
    case 4:
      v1 = *(unsigned long *)&a0[4];
      *(unsigned long *)&v2[4] = sub_1af8a(v1);
      v1 = *(unsigned long *)&a0[2];
      *(unsigned long *)&v2[2] = sub_1af8a(v1);
      v2[1] = a0[1];
      break;
    case 5:
    case 6:
    case 7:
    case 9:
    case 10:
      v1 = *(unsigned long *)&a0[4];
      *(unsigned long *)&v2[4] = sub_1af8a(v1);
      v1 = *(unsigned long *)&a0[2];
      *(unsigned long *)&v2[2] = sub_1af8a(v1);
      break;
    case 8:
      v1 = *(unsigned long *)&a0[6];
      *(unsigned long *)&v2[6] = sub_1af8a(v1);
      v1 = *(unsigned long *)&a0[4];
      *(unsigned long *)&v2[4] = sub_1af8a(v1);
      v1 = *(unsigned long *)&a0[2];
      *(unsigned long *)&v2[2] = sub_1af8a(v1);
      break;
    case 0xb:
      v1 = *(unsigned long *)&a0[6];
      *(unsigned long *)&v2[6] = sub_1b481(v1);
      v1 = *(unsigned long *)&a0[4];
      *(unsigned long *)&v2[4] = sub_1af8a(v1);
      v1 = *(unsigned long *)&a0[2];
      *(unsigned long *)&v2[2] = sub_1af8a(v1);
      v2[1] = a0[1];
      break;
    case 0xc:
      v1 = *(unsigned long *)&a0[4];
      *(unsigned long *)&v2[4] = sub_1af8a(v1);
      v1 = *(unsigned long *)&a0[2];
      *(unsigned long *)&v2[2] = sub_1af8a(v1);
      v2[1] = a0[1];
      break;
    case 0xd:
      v1 = *(unsigned long *)&a0[6];
      *(unsigned long *)&v2[6] = sub_1af8a(v1);
      v1 = *(unsigned long *)&a0[4];
      *(unsigned long *)&v2[4] = sub_1af8a(v1);
      v1 = *(unsigned long *)&a0[2];
      *(unsigned long *)&v2[2] = sub_1af8a(v1);
      break;
    case 0xe:
      v1 = *(unsigned long *)&a0[4];
      *(unsigned long *)&v2[4] = sub_1af8a(v1);
      v1 = *(unsigned long *)&a0[2];
      *(unsigned long *)&v2[2] = sub_1b481(v1);
      v2[1] = a0[1];
      break;
    case 0xf:
      v1 = *(unsigned long *)&a0[6];
      *(unsigned long *)&v2[6] = sub_1b3d6(v1);
      v1 = *(unsigned long *)&a0[4];
      *(unsigned long *)&v2[4] = sub_1b481(v1);
      v1 = *(unsigned long *)&a0[2];
      *(unsigned long *)&v2[2] = sub_1af8a(v1);
      break;
    case 0x10:
    case 0x11:
    case 0x12:
    case 0x13:
    case 0x14:
      v1 = *(unsigned long *)&a0[6];
      *(unsigned long *)&v2[6] = sub_1af8a(v1);
      v2[4] = a0[4];
      v1 = *(unsigned long *)&a0[2];
      *(unsigned long *)&v2[2] = sub_1af8a(v1);
      break;
    case 0x15:
    case 0x16:
      v1 = *(unsigned long *)&a0[6];
      *(unsigned long *)&v2[6] = sub_1af8a(v1);
      v2[5] = a0[5];
      v2[4] = a0[4];
      v1 = *(unsigned long *)&a0[2];
      *(unsigned long *)&v2[2] = sub_1af8a(v1);
      break;
    case 0x17:
    case 0x18:
      v1 = *(unsigned long *)&a0[6];
      *(unsigned long *)&v2[6] = sub_1af8a(v1);
      v2[4] = a0[4];
      v1 = *(unsigned long *)&a0[2];
      *(unsigned long *)&v2[2] = sub_1af8a(v1);
      break;
    case 0x19:
      v1 = *(unsigned long *)&a0[2];
      *(unsigned long *)&v2[2] = sub_1af8a(v1);
    
  }
  *v2 = *a0;
  return v2;
}

// Function: sub_1b3d6 @ 0x1b3d6
long sub_1b3d6(unsigned long *a0)
{
  unsigned long v1;
  long v2;
  long v3; // stack - 0x30
  unsigned long *v4; // stack - 0x40
  long *v5; // stack - 0x28
  
  v5 = &v3;
  for (v4 = a0; v4; v4 = (unsigned long *)*v4) {
    *v5 = dat_26f60;
    dat_26f60 += 0x10;
    v1 = v4[1];
    v2 = *v5;
    *(unsigned long *)(v2 + 8) = sub_1af8a(v1);
    v5 = (long *)*v5;
  }
  *v5 = 0;
  return v3;
}

// Function: sub_1b481 @ 0x1b481
char * sub_1b481(char *a0)
{
  char *v1;
  
  v1 = dat_26f68;
  dat_26f68 = &stpcpy(dat_26f68,a0)[1];
  return v1;
}

// Function: sub_1b4c3 @ 0x1b4c3
void sub_1b4c3(int *a0)
{
  if ((a0) && (*a0 = *a0 + -1, *a0 <= -1))
    free(a0);
}

// Function: _DT_FINI @ 0x1b504
void _DT_FINI(void)
{
  return;
}

