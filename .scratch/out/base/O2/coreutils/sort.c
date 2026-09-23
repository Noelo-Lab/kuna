// Function: _DT_INIT @ 0x3000
void _DT_INIT(void) // return-dupe
{
  if (!dat_1cfc8)
    return;
  (*dat_1cfc8)();
}

// Function: sub_3020 @ 0x3020
void sub_3020(void)
{
  (*dat_1cc28)(); // jump-as-call
}

// Function: malloc @ 0x3760
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1cfd0)(); // jump-as-call
  return v1;
}

// Function: free @ 0x3770
void free(void *a0)
{
  (*dat_1cfe0)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x3780
void __cxa_finalize(void)
{
  (*dat_1cff0)(); // jump-as-call
}

// Function: euidaccess @ 0x3790
void euidaccess(void)
{
  (*dat_1cc30)(); // jump-as-call
}

// Function: pipe2 @ 0x37a0
void pipe2(void)
{
  (*dat_1cc38)(); // jump-as-call
}

// Function: fileno @ 0x37b0
int fileno(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1cc40)(); // jump-as-call
  return v1;
}

// Function: pthread_join @ 0x37c0
void pthread_join(void)
{
  (*dat_1cc48)(); // jump-as-call
}

// Function: strtoumax @ 0x37d0
void strtoumax(void)
{
  (*dat_1cc50)(); // jump-as-call
}

// Function: dup2 @ 0x37e0
int dup2(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_1cc58)(); // jump-as-call
  return v1;
}

// Function: pthread_cond_destroy @ 0x37f0
void pthread_cond_destroy(void)
{
  (*dat_1cc60)(); // jump-as-call
}

// Function: __stpcpy_chk @ 0x3800
char * __stpcpy_chk(char *a0,char *a1,unsigned long a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1cc68)(); // jump-as-call
  return v1;
}

// Function: pause @ 0x3810
void pause(void)
{
  (*dat_1cc70)(); // jump-as-call
}

// Function: strcoll @ 0x3820
int strcoll(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_1cc78)(); // jump-as-call
  return v1;
}

// Function: strtold @ 0x3830
void strtold(void)
{
  (*dat_1cc80)(); // jump-as-call
}

// Function: memset @ 0x3840
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1cc88)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x3850
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1cc90)(); // jump-as-call
  return v1;
}

// Function: dcngettext @ 0x3860
void dcngettext(void)
{
  (*dat_1cc98)(); // jump-as-call
}

// Function: pthread_sigmask @ 0x3870
void pthread_sigmask(void)
{
  (*dat_1cca0)(); // jump-as-call
}

// Function: close @ 0x3880
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_1cca8)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x3890
void abort(void)
{
  (*dat_1ccb0)(); // jump-as-call
}

// Function: memchr @ 0x38a0
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1ccb8)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x38b0
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1ccc0)(); // jump-as-call
  return v1;
}

// Function: __fprintf_chk @ 0x38c0
int __fprintf_chk(void *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_1ccc8)(); // jump-as-call
  return v1;
}

// Function: pthread_cond_signal @ 0x38d0
void pthread_cond_signal(void)
{
  (*dat_1ccd0)(); // jump-as-call
}

// Function: textdomain @ 0x38e0
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1ccd8)(); // jump-as-call
  return v1;
}

// Function: strxfrm @ 0x38f0
void strxfrm(void)
{
  (*dat_1cce0)(); // jump-as-call
}

// Function: exit @ 0x3900
void exit(int a0)
{
  (*dat_1cce8)(); // jump-as-call
}

// Function: __assert_fail @ 0x3910
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_1ccf0)(); // jump-as-call
}

// Function: fstat @ 0x3920
int fstat(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_1ccf8)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x3930
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_1cd00)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x3940
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1cd08)(); // jump-as-call
  return v1;
}

// Function: setvbuf @ 0x3950
int setvbuf(void *a0,char *a1,int a2,unsigned long a3)
{
  int v1; // eax
  
  v1 = (*dat_1cd10)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x3960
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1cd18)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x3970
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_1cd20)(); // jump-as-call
  return v1;
}

// Function: fopen @ 0x3980
void * fopen(char *a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1cd28)(); // jump-as-call
  return v1;
}

// Function: execlp @ 0x3990
int execlp(char *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_1cd30)(); // jump-as-call
  return v1;
}

// Function: unlink @ 0x39a0
int unlink(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_1cd38)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x39b0
void _exit(int a0)
{
  (*dat_1cd40)(); // jump-as-call
}

// Function: sigismember @ 0x39c0
void sigismember(void)
{
  (*dat_1cd48)(); // jump-as-call
}

// Function: __cxa_atexit @ 0x39d0
void __cxa_atexit(void)
{
  (*dat_1cd50)(); // jump-as-call
}

// Function: sysconf @ 0x39e0
long sysconf(int a0)
{
  long v1; // rax
  
  v1 = (*dat_1cd58)(); // jump-as-call
  return v1;
}

// Function: pthread_mutex_init @ 0x39f0
void pthread_mutex_init(void)
{
  (*dat_1cd60)(); // jump-as-call
}

// Function: clearerr_unlocked @ 0x3a00
void clearerr_unlocked(void *a0)
{
  (*dat_1cd68)(); // jump-as-call
}

// Function: __ctype_toupper_loc @ 0x3a10
void * __ctype_toupper_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1cd70)(); // jump-as-call
  return v1;
}

// Function: pthread_create @ 0x3a20
void pthread_create(void)
{
  (*dat_1cd78)(); // jump-as-call
}

// Function: fread_unlocked @ 0x3a30
void fread_unlocked(void)
{
  (*dat_1cd80)(); // jump-as-call
}

// Function: strlen @ 0x3a40
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1cd88)(); // jump-as-call
  return v1;
}

// Function: ferror @ 0x3a50
int ferror(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1cd90)(); // jump-as-call
  return v1;
}

// Function: pthread_cond_init @ 0x3a60
void pthread_cond_init(void)
{
  (*dat_1cd98)(); // jump-as-call
}

// Function: __ctype_get_mb_cur_max @ 0x3a70
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1cda0)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x3a80
int fputs_unlocked(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_1cda8)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x3a90
int __freading(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1cdb0)(); // jump-as-call
  return v1;
}

// Function: MD5_Final @ 0x3aa0
void MD5_Final(void)
{
  (*dat_1cdb8)(); // jump-as-call
}

// Function: __ctype_b_loc @ 0x3ab0
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1cdc0)(); // jump-as-call
  return v1;
}

// Function: fgetc @ 0x3ac0
void fgetc(void)
{
  (*dat_1cdc8)(); // jump-as-call
}

// Function: fdopen @ 0x3ad0
void * fdopen(int a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1cdd0)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x3ae0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1cdd8)(); // jump-as-call
  return v1;
}

// Function: getrandom @ 0x3af0
void getrandom(void)
{
  (*dat_1cde0)(); // jump-as-call
}

// Function: pipe @ 0x3b00
int pipe(int *a0)
{
  int v1; // eax
  
  v1 = (*dat_1cde8)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x3b10
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1cdf0)(); // jump-as-call
  return v1;
}

// Function: __fpending @ 0x3b20
unsigned long __fpending(void *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1cdf8)(); // jump-as-call
  return v1;
}

// Function: sigaction @ 0x3b30
int sigaction(int a0,void *a1,void *a2)
{
  int v1; // eax
  
  v1 = (*dat_1ce00)(); // jump-as-call
  return v1;
}

// Function: __explicit_bzero_chk @ 0x3b40
void __explicit_bzero_chk(void *a0,unsigned long a1,unsigned long a2)
{
  (*dat_1ce08)(); // jump-as-call
}

// Function: posix_fadvise @ 0x3b50
void posix_fadvise(void)
{
  (*dat_1ce10)(); // jump-as-call
}

// Function: fputs @ 0x3b60
int fputs(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_1ce18)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x3b70
void lseek(void)
{
  (*dat_1ce20)(); // jump-as-call
}

// Function: strtol @ 0x3b80
long strtol(char *a0,char **a1,int a2)
{
  long v1; // rax
  
  v1 = (*dat_1ce28)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x3b90
void iswcntrl(void)
{
  (*dat_1ce30)(); // jump-as-call
}

// Function: sched_getaffinity @ 0x3ba0
void sched_getaffinity(void)
{
  (*dat_1ce38)(); // jump-as-call
}

// Function: mkostemp @ 0x3bb0
void mkostemp(void)
{
  (*dat_1ce40)(); // jump-as-call
}

// Function: raise @ 0x3bc0
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_1ce48)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x3bd0
void signal(void)
{
  (*dat_1ce50)(); // jump-as-call
}

// Function: memmove @ 0x3be0
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1ce58)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x3bf0
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1ce60)(); // jump-as-call
  return v1;
}

// Function: waitpid @ 0x3c00
int waitpid(int a0,int *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_1ce68)(); // jump-as-call
  return v1;
}

// Function: getopt_long @ 0x3c10
int getopt_long(int a0,char **a1,char *a2,void *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_1ce70)(); // jump-as-call
  return v1;
}

// Function: getenv @ 0x3c20
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1ce78)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x3c30
int mbsinit(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1ce80)(); // jump-as-call
  return v1;
}

// Function: __errno_location @ 0x3c40
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_1ce88)(); // jump-as-call
  return v1;
}

// Function: qsort @ 0x3c50
void qsort(void)
{
  (*dat_1ce90)(); // jump-as-call
}

// Function: mkstemp @ 0x3c60
void mkstemp(void)
{
  (*dat_1ce98)(); // jump-as-call
}

// Function: dcgettext @ 0x3c70
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1cea0)(); // jump-as-call
  return v1;
}

// Function: sigaddset @ 0x3c80
int sigaddset(void *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_1cea8)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x3c90
void __stack_chk_fail(void)
{
  (*dat_1ceb0)(); // jump-as-call
}

// Function: strcmp @ 0x3ca0
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_1ceb8)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x3cb0
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_1cec0)(); // jump-as-call
  return v1;
}

// Function: MD5_Init @ 0x3cc0
void MD5_Init(void)
{
  (*dat_1cec8)(); // jump-as-call
}

// Function: localeconv @ 0x3cd0
void localeconv(void)
{
  (*dat_1ced0)(); // jump-as-call
}

// Function: __overflow @ 0x3ce0
int __overflow(void *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_1ced8)(); // jump-as-call
  return v1;
}

// Function: strcpy @ 0x3cf0
char * strcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1cee0)(); // jump-as-call
  return v1;
}

// Function: nanosleep @ 0x3d00
void nanosleep(void)
{
  (*dat_1cee8)(); // jump-as-call
}

// Function: pthread_cond_wait @ 0x3d10
void pthread_cond_wait(void)
{
  (*dat_1cef0)(); // jump-as-call
}

// Function: MD5_Update @ 0x3d20
void MD5_Update(void)
{
  (*dat_1cef8)(); // jump-as-call
}

// Function: memcmp @ 0x3d30
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_1cf00)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x3d40
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1cf08)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x3d50
int fclose(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1cf10)(); // jump-as-call
  return v1;
}

// Function: getrlimit @ 0x3d60
void getrlimit(void)
{
  (*dat_1cf18)(); // jump-as-call
}

// Function: __sched_cpucount @ 0x3d70
void __sched_cpucount(void)
{
  (*dat_1cf20)(); // jump-as-call
}

// Function: fseeko @ 0x3d80
void fseeko(void)
{
  (*dat_1cf28)(); // jump-as-call
}

// Function: ftruncate @ 0x3d90
void ftruncate(void)
{
  (*dat_1cf30)(); // jump-as-call
}

// Function: fork @ 0x3da0
int fork(void)
{
  int v1; // eax
  
  v1 = (*dat_1cf38)(); // jump-as-call
  return v1;
}

// Function: sigemptyset @ 0x3db0
int sigemptyset(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1cf40)(); // jump-as-call
  return v1;
}

// Function: pthread_mutex_lock @ 0x3dc0
int pthread_mutex_lock(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1cf48)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x3dd0
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1cf50)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x3de0
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1cf58)(); // jump-as-call
  return v1;
}

// Function: error @ 0x3df0
void error(int a0,int a1,char *a2,...)
{
  (*dat_1cf60)(); // jump-as-call
}

// Function: pthread_mutex_destroy @ 0x3e00
void pthread_mutex_destroy(void)
{
  (*dat_1cf68)(); // jump-as-call
}

// Function: write @ 0x3e10
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_1cf70)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x3e20
int stat(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_1cf78)(); // jump-as-call
  return v1;
}

// Function: strtoul @ 0x3e30
unsigned long strtoul(char *a0,char **a1,int a2)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1cf80)(); // jump-as-call
  return v1;
}

// Function: sysinfo @ 0x3e40
void sysinfo(void)
{
  (*dat_1cf88)(); // jump-as-call
}

// Function: pthread_mutex_unlock @ 0x3e50
int pthread_mutex_unlock(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1cf90)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x3e60
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1cf98)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x3e70
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_1cfa0)(); // jump-as-call
  return v1;
}

// Function: fflush_unlocked @ 0x3e80
int fflush_unlocked(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1cfa8)(); // jump-as-call
  return v1;
}

// Function: open @ 0x3e90
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_1cfb0)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x3ea0
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_1cfb8)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x3eb0
int fflush(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1cfc0)(); // jump-as-call
  return v1;
}

// Function: sub_3ec0 @ 0x3ec0
unsigned long sub_3ec0(unsigned int a0,char **a1) // ternary x3
{
  unsigned char *v1;
  unsigned char v10;
  unsigned char v100; // stack - 0x3b2
  long v101; // stack - 0x390
  long v102; // stack - 0x388
  long v103; // stack - 0x380
  char v104; // stack - 0x33f
  char v105 [16]; // stack - 0x320
  char v106 [16]; // stack - 0x310
  char v107 [16]; // stack - 0x300
  char v108 [16]; // stack - 0x2f0
  unsigned long v109; // stack - 0x2d0
  unsigned char v11;
  unsigned long v110; // stack - 0x2c8
  long v111; // stack - 0x2c0
  unsigned long v112; // stack - 0x2b8
  unsigned long v113; // stack - 0x2b0
  unsigned char v114; // stack - 0x2a8
  unsigned char v115; // stack - 0x2a7
  unsigned char v116; // stack - 0x2a6
  unsigned char v117; // stack - 0x2a5
  unsigned char v118; // stack - 0x2a4
  unsigned char v119; // stack - 0x2a3
  unsigned char v12;
  unsigned char v120; // stack - 0x2a2
  unsigned char v121; // stack - 0x2a1
  unsigned char v122; // stack - 0x2a0
  unsigned int v123; // stack - 0x284
  unsigned long *v124; // stack - 0x280
  unsigned long v125; // stack - 0x158
  unsigned long v126; // stack - 0x150
  unsigned long v127; // stack - 0x148
  unsigned long v128; // stack - 0x140
  unsigned long v129; // stack - 0x130
  unsigned char v13;
  unsigned long v130; // stack - 0x128
  unsigned long v131; // stack - 0x120
  unsigned long v132; // stack - 0x118
  unsigned long v133; // stack - 0x110
  unsigned long v134; // stack - 0x108
  unsigned long v135; // stack - 0x100
  unsigned long v136; // stack - 0xf8
  unsigned long v137; // stack - 0xf0
  unsigned long v138; // stack - 0xe8
  unsigned int v139; // stack - 0xe0
  char v14;
  long v140; // stack - 0x40
  char v15;
  unsigned char v16;
  int v17;
  int v18; // eax
  unsigned int v19; // eax
  unsigned long *v2;
  unsigned long *v20;
  long *v21;
  long *v22;
  long v23;
  unsigned char *v24;
  unsigned long v25;
  long v26;
  char *v27;
  char *v28;
  unsigned long v29;
  void **v3;
  unsigned long *v30;
  void *v31;
  unsigned long *v32; // rax
  unsigned long *v33;
  unsigned long *v34; // rax
  void *v35;
  void *v36; // rax
  long *v37;
  unsigned long v38; // rax
  unsigned long v39; // rax
  unsigned short v4;
  unsigned long v40; // rax
  unsigned long v41; // rax
  unsigned long v42;
  unsigned long v43; // rax
  unsigned long v44; // rax
  unsigned long v45; // rax
  unsigned long v46; // rax
  unsigned long v47; // rax
  unsigned long *v48;
  unsigned long v49;
  unsigned int v5;
  unsigned char v50;
  unsigned int v51;
  unsigned long *v52;
  void *v53; // stack - 0x168
  void *v54; // stack - 0x398
  unsigned long v55; // stack - 0x328
  unsigned int v56; // stack - 0x78
  void *v57; // stack - 0x2d8
  char v58 [32];
  char v59 [47];
  long v6;
  char v60; // stack - 0xa8
  int v61; // stack - 0x288
  long v62; // stack - 0x378
  unsigned long v63; // stack - 0x138
  unsigned long v64; // stack - 0x160
  long v65; // stack - 0x368
  long v66; // stack - 0x370
  long v67 [4]; // stack - 0x360
  unsigned char v68;
  char v69;
  bool v7;
  unsigned long *v70;
  char v71; // dil
  void **v72;
  unsigned int *v73;
  int *v74; // r12
  char *v75;
  void *v76;
  unsigned char v77;
  int *v78; // r13
  unsigned char v79;
  bool v8;
  unsigned char v80;
  unsigned long v81;
  long v82;
  bool v83;
  bool v84;
  bool v85;
  double v86; // xmm0_qa
  double v87; // xmm0_qa
  double v88; // xmm0_qa
  double v89; // xmm0_qa
  unsigned char v9;
  double v90;
  long v91; // stack - 0x458
  char *v92; // stack - 0x438
  char *v93; // stack - 0x430
  void **v94; // stack - 0x420
  unsigned long v95; // stack - 0x408
  char *v96; // stack - 0x3f8
  long v97; // stack - 0x3e0
  char *v98; // stack - 0x3d8
  unsigned long v99; // stack - 0x3d0
  
  v140 = *(long *)(v82 + 0x28);
  v94 = (char *)getenv("POSIXLY_CORRECT");
  v83 = v94 == NULL;
  v17 = sub_eb70();
  v85 = 0x2b8 < (unsigned int)(v17 - 0x30db0U);
  sub_ec00(*a1);
  v93 = setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  dat_1d130 = 2;
  dat_1d861 = sub_cc40(3);
  v14 = sub_cc40(2);
  dat_1d860 = v14;
  v20 = (unsigned long *)localeconv();
  dat_1d868 = *(unsigned char *)*v20;
  if ((!dat_1d868) || (((unsigned char *)*v20)[1]))
    dat_1d868 = 0x2e;
  v15 = *(char *)v20[1];
  dat_1d864 = (unsigned int)v15;
  if (v15) {
    if (!((char *)v20[1])[1]) goto label_3fc7;
    dat_1d862 = '\x01';
  }
  dat_1d864 = 0x80;
label_3fc7:
  dat_1d418 = '\0';
  v21 = __ctype_b_loc();
  v22 = __ctype_toupper_loc();
  v23 = 0;
  do {
    v4 = *(unsigned short *)(*v21 + v23 * 2);
    if (v4 & 1) { // branch-flip
      v71 = 1;
      v50 = 0;
label_402e:
      *(unsigned char *)(v23 + 0x1d540) = v50;
      v26 = *v22;
      *(char *)(v23 + 0x1d740) = v71;
      v5 = *(unsigned int *)(v26 + v23 * 4);
      *(unsigned char *)(v23 + 0x1d640) = ((unsigned char)(v4 >> 0xe) ^ 1) & 1;
      *(char *)(v23 + 0x1d440) = (char)v5;
      if (v23 == 0xff) goto label_40b8;
    }
    else {
      if (v23 != 10) {
        v71 = 0;
        v50 = ((unsigned char)(v4 >> 3) ^ 1) & 1;
        goto label_402e;
      }
      dat_1d74a = 1;
      dat_1d54a = 0;
      dat_1d64a = ((unsigned char)((unsigned short)*(unsigned short *)(*v21 + 0x14) >> 0xe) ^ 1) & 1;
      dat_1d44a = (char)*(unsigned int *)(*v22 + 0x28);
    }
    v23 += 1;
  } while( true );
label_60e5:
  sub_7790(&v57,v97,v35);
label_5385:
  v94 = &v57;
  v95 -= 1;
  v114 = 0;
  while( true ) {
    v14 = sub_7ef0(v94,v31,v42);
    v50 = v114;
    v26 = v111;
    v81 = v110;
    v76 = v57;
    if (!v14) break;
    if (v114) { // branch-flip
      if (v95) {
        if ((v111 - v109) - v97 * v110 <= v97 + 1U) goto label_5402;
        v112 = v109;
        sub_7660(v31,v42);
        goto label_5d45;
      }
      dat_1d840 = 0;
      if (v91 || v112) goto label_5402;
      sub_7660(v31,v42);
      v35 = (void *)sub_74e0(v96,"w");
      if (!v35) {
        sub_7480(dcgettext(NULL,"open failed",5),v96); // return-dupe, no-return
      }
      v54 = v35;
      v91 = 0;
      v75 = v96;
      v81 = v110;
    }
    else {
label_5402:
      dat_1d840 = 0;
      v91 += 1;
      v50 = 0;
      v75 = (char *)(sub_8c20(&v54,0) + 0xd);
    }
    v26 += (long)v76;
    if (2 <= v81) { // branch-flip
      v53 = (void *)sub_deb0(sub_7c20,v23);
      pthread_mutex_init(&v64,0);
      pthread_cond_init(&v63,0);
      v30 = (unsigned long *)sub_12ab0(v25 << 8);
      v30[3] = 0;
      v30[2] = 0;
      v30[1] = 0;
      *v30 = 0;
      v30[4] = 0;
      v30[6] = v81;
      v30[5] = v81;
      v30[7] = 0;
      *(unsigned int *)&v30[10] = 0;
      *(char *)((long)v30 + 0x54) = 0;
      pthread_mutex_init(&v30[0xb],0);
      sub_7af0(v30,&v30[0x10],v26,v25,v81,0);
      sub_b220(v26,v25,v81,&v30[0x10],&v53,v54);
      if (v23) {
        v33 = &v30[0xb];
        v26 = v23 + -1;
        do {
          v26 -= 1;
          v32 = &v33[0x10];
          pthread_mutex_destroy(v33);
          v33 = v32;
        } while (v26 != -1);
      }
      free(v30);
      sub_df20(v53);
      pthread_cond_destroy(&v63);
      pthread_mutex_destroy(&v64);
    }
    else {
      sub_a3c0(v26 + -0x20,v54,v75);
    }
    sub_7660(v54,v75);
    if (v50) {
      free(v57);
      goto label_5599;
    }
  }
  sub_7660(v31,v42);
  if (!v95) goto label_64ce;
label_5d45:
  v20 = &v20[1];
  goto label_5328;
label_64ce:
  free(v57);
  v20 = dat_1d368;
  v21 = (long *)sub_12bf0(v91,0x10);
  v22 = v21;
  while (v20) {
    v22[1] = (long)v20;
    v23 = (long)v20 + 0xd;
    v20 = (unsigned long *)*v20;
    v37 = &v22[2];
    *v22 = v23;
    v22 = v37;
  }
  sub_ac90(v21,v91,v91,v96);
  free(v21);
label_5599:
  while (1 <= dat_1d35c) {
    sub_78c0(0xffffffff);
  }
  goto label_55b4;
label_40b8:
  if (v14) {
    v23 = 1;
    v22 = (long *)0x1d060;
    do {
      v24 = (unsigned char *)nl_langinfo((int)v23 + 0x2000d);
      v25 = strlen((char *)v24);
      v26 = sub_12ab0(v25 + 1);
      *(int *)&v22[1] = (int)v23;
      *v22 = v26;
      if (v25) {
        v6 = *v21;
        v1 = &v24[v25];
        v25 = 0;
        do {
          if (!(*(unsigned char *)(v6 + (unsigned long)*v24 * 2) & 1)) {
            *(char *)(v26 + v25) = *(char *)((unsigned long)*v24 + 0x1d440);
            v25 += 1;
          }
          v24 = &v24[1];
        } while (v1 != v24);
      }
      v23 += 1;
      *(char *)(v26 + v25) = 0;
      v22 = &v22[2];
    } while (v23 != 0xd);
    qsort(0x1d060,0xc,0x10,sub_7300);
  }
  v78 = (int *)0x15324;
  v74 = (int *)0x15324;
  sigemptyset((void *)0x1d380);
  v17 = 0xe;
  while( true ) {
    sigaction(v17,NULL,&v53);
    if (v53 != (void *)0x1)
      sigaddset((void *)0x1d380,v17);
    if (v74 == (int *)0x1534c) break;
    v17 = *v74;
    v74 = &v74[1];
  }
  v139 = 0;
  v53 = sub_8270;
  v17 = 0xe;
  v64 = dat_1d380;
  v125 = dat_1d388;
  v126 = dat_1d390;
  v127 = dat_1d398;
  v128 = dat_1d3a0;
  v63 = dat_1d3a8;
  v129 = dat_1d3b0;
  v130 = dat_1d3b8;
  v131 = dat_1d3c0;
  v132 = dat_1d3c8;
  v133 = dat_1d3d0;
  v134 = dat_1d3d8;
  v135 = dat_1d3e0;
  v136 = dat_1d3e8;
  v137 = dat_1d3f0;
  v138 = dat_1d3f8;
  while( true ) {
    if (sigismember(0x1d380,v17))
      sigaction(v17,&v53,NULL);
    if (v78 == (int *)0x1534c) break;
    v17 = *v78;
    v78 = &v78[1];
  }
  v81 = 0;
  signal(0x11,0);
  sub_142c0(sub_8ed0);
  v75 = "-bcCdfghik:mMno:rRsS:t:T:uVy:z";
  v105._0_16_ = ZEXT816(0xffffffffffffffff) << 0x40;
  v55 = 0xffffffffffffffff;
  v106._0_16_ = 0;
  v107._0_16_ = 0;
  v108._0_16_ = 0;
  v20 = (unsigned long *)sub_12bf0((long)(int)a0,8);
  v8 = 0;
  v25 = 0;
  v96 = NULL;
  v92 = NULL;
  v99 = 0;
  v98 = NULL;
  v14 = '\0';
  v91._0_4_ = a0;
label_43e0:
  v54 = (void *)CONCAT44(v54._4_4_,0xffffffff);
  if ((int)v25 == -1) {
label_43f1:
    v23 = (long)(int)optind;
    goto label_43c0;
  }
  if ((!v81) || (v83)) {
label_4410:
    v72 = &v54;
    v51 = getopt_long((unsigned int)v91,a1,v75,(void *)0x1c5e0,(int *)v72);
    v31 = v54;
    v27 = optarg;
    v25 = (unsigned long)v51;
    if (v51 == 0xffffffff) goto label_43f1;
    if (0x87 < (int)v51) {
      sub_bab0(2); // return-dupe, no-return
    }
    if ((int)v51 <= 0) {
      if (v51 == 0xffffff7d) {
        sub_128e0(stdout,"sort","GNU coreutils",dat_1d120,"Mike Haertel","Paul Eggert",0,v72);
        exit(0); // no-return
      }
      if (v51 == 0xffffff7e)
        sub_bab0(0); // no-return
      sub_bab0(2);
    }
    if (0x87 < v51) {
      sub_bab0(2);
    }
    v17 = (int)v54;
    switch(v51) {
      default:
        goto label_661d;
      case 1:
        if (*optarg == '+') {
          if (optind != (unsigned int)v91) { // branch-flip
            v84 = 0;
            if (*a1[(int)optind] == '-')
              v84 = (unsigned int)((int)a1[(int)optind][1] - 0x30U) <= 9;
          }
          else {
            v84 = 0;
          }
          v85 = (bool)(v85 | v94 == NULL & v84);
          if (v85) {
            v23 = 0x12;
            v22 = &v62;
            while (v23) {
              v37 = (long *)((long)v22 + 4);
              *(unsigned int *)v22 = 0;
              v23 -= 1;
              v22 = v37;
            }
            v65 = -1;
            v27 = (char *)sub_7390(&optarg[1],&v62,0);
            if (v27) { // branch-flip
              if (*v27 != '.') { // branch-flip
                if (!v66 && !v62) {
label_61b5:
                  v62 = -1;
                  goto label_61c1;
                }
              }
              else {
                v27 = (char *)sub_7390(&v27[1],&v66,0);
                if (!v62 && !v66) goto label_61b5;
label_61c1:
                if (!v27) goto label_67de;
              }
              if (!*(char *)sub_71f0(v27,&v62,0)) {
                if (v84) {
                  v23 = (long)(int)optind;
                  optind += 1;
                  v27 = a1[v23];
                  v28 = (char *)sub_7390(&v27[1],&v65,"invalid number after \'-\'");
                  if (!v28)
                    __assert_fail("s","src/sort.c",0x1161,"main"); // no-return
                  if (*v28 == '.')
                    v28 = (char *)sub_7390(&v28[1],v67,"invalid number after \'.\'");
                  if ((!v67[0]) && (v65))
                    v65 -= 1;
                  if (*(char *)sub_71f0(v28,&v62,1))
                    sub_7340(v27,"stray character in field spec"); // no-return
                }
                v104 = 1;
                sub_76e0(&v62);
                v25 = (unsigned long)v51;
                break;
              }
            }
            else if (!v66 && !v62)
              v62 = -1;
label_67de:
            v27 = optarg;
          }
        }
        v25 = (unsigned long)v51;
        v20[v81] = v27;
        v81 += 1;
        break;
      case 0x43:
      case 99:
label_4527:
        if ((v14) && ((int)v14 != (int)v25))
          sub_7310("cC"); // no-return
        v14 = (char)v25;
        break;
      case 0x4d:
      case 0x52:
      case 0x56:
      case 0x62:
      case 100:
      case 0x66:
      case 0x67:
      case 0x68:
      case 0x69:
      case 0x6e:
      case 0x72:
        goto label_44a1;
      case 0x53:
        v17 = sub_132f0(optarg,&v61,10,&v57,"EgGkKmMPtTYZ");
        if (v17) { // branch-flip
          if (((v17 != 2) || (v28 = (char *)CONCAT44(v123,v61), 10 <= (unsigned int)((int)v28[-1] - 0x30U))) || (v28[1])) {
            sub_13230(v17,(unsigned long)v31 & 0xffffffff,0x53,0x1c5e0,v27); // return-dupe, no-return
          }
          if (*v28 != '%') { // branch-flip
            if (*v28 != 'b') {
              sub_13230(v17,(unsigned long)v31 & 0xffffffff,0x53,0x1c5e0,v27);
            }
          }
          else {
            sub_e760();
            v90 = ((double)v57 * v89) / dat_170f0;
            if (dat_170f8 <= v90) goto label_6691;
            if (v90 < dat_17100) {
              v76 = (void *)(long)v90;
              v57 = (void *)(long)v90;
              goto label_56b9;
            }
            v57 = (void *)((long)(v90 - dat_17100) ^ 0x8000000000000000);
          }
          v76 = v57;
        }
        else {
          v76 = v57;
          if ((unsigned int)((int)*(char *)(CONCAT44(v123,v61) + -1) - 0x30U) <= 9) {
            if ((void *)0x3fffffffffffff < v57) {
label_6691:
              v17 = 1;
              sub_13230(v17,(unsigned long)v31 & 0xffffffff,0x53,0x1c5e0,v27);
            }
            v76 = (void *)((long)v57 << 10);
            v57 = v76;
          }
        }
label_56b9:
        v25 = (unsigned long)v51;
        if ((dat_1d438 <= v76) && (dat_1d438 = (void *)((unsigned long)dat_1d024 * 0x22), (void *)((unsigned long)dat_1d024 * 0x22) < v76))
          dat_1d438 = v76;
        break;
      case 0x54:
        sub_7730(optarg);
        v25 = (unsigned long)v51;
        break;
      case 0x6b:
        v23 = 0x12;
        v22 = &v62;
        while (v23) {
          v37 = (long *)((long)v22 + 4);
          *(unsigned int *)v22 = 0;
          v23 -= 1;
          v22 = v37;
        }
        v65 = -1;
        v27 = (char *)sub_7390(optarg,&v62,"invalid number at field start");
        v23 = v62 + -1;
        v84 = v62 == 0;
        v62 = v23;
        if (v84) {
          sub_7340(optarg,"field number is zero"); // return-dupe, no-return
        }
        if (*v27 == '.') {
          v27 = (char *)sub_7390(&v27[1],&v66,"invalid number after \'.\'");
          v23 = v66 + -1;
          v84 = v66 == 0;
          v66 = v23;
          if (v84)
            sub_7340(optarg,"character offset is zero"); // no-return
        }
        if (!v62 && !v66)
          v62 = -1;
        v27 = (char *)sub_71f0(v27,&v62,0);
        if (*v27 != ',') { // branch-flip
          v65 = -1;
          v67[0] = 0;
        }
        else {
          v27 = (char *)sub_7390(&v27[1],&v65,"invalid number after \',\'");
          v23 = v65 + -1;
          v84 = v65 == 0;
          v65 = v23;
          if (v84) {
            sub_7340(optarg,"field number is zero");
          }
          if (*v27 == '.')
            v27 = (char *)sub_7390(&v27[1],v67,"invalid number after \'.\'");
          v27 = (char *)sub_71f0(v27,&v62,1);
        }
        if (*v27) {
          sub_7340(optarg,"stray character in field spec"); // return-dupe, no-return
        }
        sub_76e0(&v62);
        v25 = (unsigned long)v51;
        break;
      case 0x6d:
        v8 = 1;
        break;
      case 0x6f:
        if ((v96) && (v17 = strcmp(v96,optarg), v17)) goto label_6dbc;
        v25 = (unsigned long)v51;
        v96 = v27;
        break;
      case 0x73:
        dat_1d41a = '\x01';
        break;
      case 0x74:
        v19 = (unsigned int)*optarg;
        if (!*optarg) {
          error(2,0,dcgettext(NULL,"empty tab",5));
          return v39;
        }
        if (optarg[1]) {
          v17 = strcmp(optarg,"\\0");
          if (v17) {
            v42 = sub_10fd0(v27);
            error(2,0,dcgettext(NULL,"multi-character tab %s",5),v42);
            return v40;
          }
          v19 = 0;
        }
        v25 = (unsigned long)v51;
        if ((dat_1d028 != 0x80) && (dat_1d028 != v19)) {
          error(2,0,dcgettext(NULL,"incompatible tabs",5));
label_6dbc:
          error(2,0,dcgettext(NULL,"multiple output files specified",5));
          sub_7340(optarg,"field number is zero");
        }
        dat_1d028 = v19;
        break;
      case 0x75:
        dat_1d419 = 1;
        break;
      case 0x79:
        if (a1[(long)(int)optind + -1] == optarg) {
          for (; (unsigned int)((int)*v27 - 0x30U) <= 9; v27 = &v27[1]) {
          }
          optind = (optind - 1) + (unsigned int)(*v27 == '\0');
        }
        break;
      case 0x7a:
        dat_1d038 = 0;
        break;
      case 0x80:
        v25 = 99;
        if (optarg) {
          v23 = sub_c370("--check",optarg,0x1c5c0,"CCc",1,dat_1d128,1,99);
          v25 = (unsigned long)(unsigned int)(int)"CCc"[v23];
        }
        goto label_4527;
      case 0x81:
        if ((dat_1d408) && (v17 = strcmp(dat_1d408,optarg), v17)) {
label_699a:
          error(2,0,dcgettext(NULL,"multiple compress programs specified",5));
          sub_7340(optarg,"stray character in field spec");
        }
        v25 = (unsigned long)v51;
        dat_1d408 = v27;
        break;
      case 0x82:
        dat_1d400 = '\x01';
        break;
      case 0x83:
        goto label_4ef5;
      case 0x84:
        v18 = sub_132f0(optarg,0,10,&v57,"");
        v25 = (unsigned long)v51;
        v51 = 0x11;
        if (!getrlimit(7,&v61))
          v51 = v61 - 3;
        if (v18) { // branch-flip
          if (v18 != 1)
            sub_13230(v18,(unsigned long)v31 & 0xffffffff,0xffffff84,0x1c5e0,v27); // no-return
        }
        else {
          dat_1d024 = (unsigned int)v57;
          if ((void *)((unsigned long)v57 & 0xffffffff) == v57) {
            if (dat_1d024 < 2) {
              v29 = sub_10fd0(v27);
              v42 = *(unsigned long *)((long)v17 * 0x20 + 0x1c5e0);
              error(0,0,dcgettext(NULL,"invalid --%s argument %s",5),v42,v29);
              v29 = sub_10fd0("2");
              error(2,0,dcgettext(NULL,"minimum --%s argument is %s",5),v42,v29);
              return v41;
            }
            if (dat_1d024 <= v51) break;
          }
        }
        v42 = sub_10fd0(v27);
        v75 = *(char **)((long)v17 * 0x20 + 0x1c5e0);
        error(0,0,dcgettext(NULL,"--%s argument %s too large",5),v75,v42);
        v21 = (long *)sub_e1b0(v51,&v56);
        error(2,0,dcgettext(NULL,"maximum --%s argument with current rlimit is %s",5),v75,v21);
        v91._0_4_ = v51;
label_4ef5:
        v92 = optarg;
        break;
      case 0x85:
label_4d92:
        v27 = optarg;
        if (v98) {
          v17 = strcmp(v98,optarg);
          v25 &= 0xffffffff;
          if (v17) {
            error(2,0,dcgettext(NULL,"multiple random sources specified",5));
            goto label_699a;
          }
        }
        v98 = v27;
        break;
      case 0x86:
        v23 = sub_c370("--sort",optarg,0x1c580,"ghMnRVCCc",1,dat_1d128,1,(long)*(int *)((unsigned long)v51 * 4 + 0x150cc) + 0x150cc);
        v51 = (unsigned int)"ghMnRVCCc"[v23];
label_44a1:
        v56._0_2_ = (unsigned short)(unsigned char)v51;
        sub_71f0(&v56,&v55,2);
        v25 = (unsigned long)v51;
        break;
      case 0x87:
        v17 = sub_132f0(optarg,0,10,&v61,"");
        v25 = (unsigned long)v51;
        if (v17 != 1) { // branch-flip
          if (v17)
            sub_13230(v17,(unsigned long)v31 & 0xffffffff,0xffffff87,0x1c5e0,v27); // no-return
          v99 = CONCAT44(v123,v61);
          if (!v99) {
            error(2,0,dcgettext(NULL,"number in parallel must be nonzero",5));
            goto label_4d92;
          }
        }
        else {
          v99 = 0xffffffffffffffff;
        }
      
    }
    goto label_43e0;
  }
  v23 = (long)(int)optind;
  if ((v85) && (!v14)) {
    if ((unsigned int)v91 != optind) {
      v27 = a1[(int)optind];
      if (((*v27 == '-') && (v27[1] == 'o')) && ((v27[2] || (optind + 1 != (unsigned int)v91)))) goto label_4410;
      goto label_43c0;
    }
    if (!v92) {
      v14 = '\0';
      goto label_4571;
    }
label_65b8:
    v42 = sub_108e0(4,*v20);
    error(0,0,dcgettext(NULL,"extra operand %s",5),v42);
    __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"file operands cannot be combined with --files0-from",5));
    sub_bab0(2);
  }
label_43c0:
  if ((int)v23 < (int)(unsigned int)v91) {
    optind = (int)v23 + 1;
    v20[v81] = a1[v23];
    v81 += 1;
    goto label_43e0;
  }
  if (!v92) goto label_4571;
  if (v81) goto label_65b8;
  v23 = sub_74e0(v92,"r");
  if (!v23)
    sub_7c60(v92); // no-return
  sub_119f0(&v61);
  if (!sub_11aa0(v23,&v61)) {
    v42 = sub_108e0(4,v92);
    error(2,0,dcgettext(NULL,"cannot read file names from %s",5),v42);
    return v46;
  }
  sub_7660(v23,v92);
  if (!CONCAT44(v123,v61)) {
    v42 = sub_108e0(4,v92);
    error(2,0,dcgettext(NULL,"no input from %s",5),v42);
    return v45;
  }
  free(v20);
  v20 = v124;
  if (CONCAT44(v123,v61)) {
    v81 = 0;
    do {
      v75 = (char *)v124[v81];
      if (!strcmp(v75,"-")) {
        v42 = sub_108e0(4,v75);
        error(2,0,dcgettext(NULL,"when reading file names from stdin, no file name of %s allowed",5),v42);
        return v43;
      }
      v81 += 1;
      if (!*v75) {
        v42 = sub_10c60(0,3,v92);
        error(2,0,dcgettext(NULL,"%s:%lu: invalid zero-length file name",5),v42,v81);
        return v44;
      }
    } while (CONCAT44(v123,v61) != v81);
  }
label_4571:
  if (dat_1d410) { // branch-flip
    v50 = 0;
    v48 = dat_1d410;
    v52 = dat_1d410;
    do {
      while ((v15 = sub_70e0(v48), !v15 || (*(char *)((long)v48 + 0x37)))) {
        v24 = (unsigned char *)((long)v48 + 0x33);
        v48 = (unsigned long *)v48[8];
        v50 |= *v24;
        if (!v48) goto label_463f;
      }
      v48[4] = v106._8_8_;
      v48[5] = v107._0_8_;
      *(char *)&v48[6] = v107[8];
      *(char *)((long)v48 + 0x31) = v107[9];
      *(char *)((long)v48 + 0x36) = v107[0xe];
      *(char *)((long)v48 + 0x32) = v107[10];
      *(char *)((long)v48 + 0x34) = v107[0xc];
      *(char *)((long)v48 + 0x35) = v107[0xd];
      *(char *)((long)v48 + 0x37) = v107[0xf];
      *(char *)&v48[7] = v108[0];
      *(unsigned char *)((long)v48 + 0x33) = v107[0xb];
      v48 = (unsigned long *)v48[8];
      v50 |= v107[0xb];
    } while (v48);
label_463f:
    v85 = 0;
    do {
      if (1 < (unsigned int)*(unsigned char *)((long)v52 + 0x32) + (unsigned int)*(unsigned char *)((long)v52 + 0x34) + (unsigned int)*(unsigned char *)((long)v52 + 0x35) + (unsigned int)*(unsigned char *)((long)v52 + 0x36) + (unsigned int)(unsigned char)((unsigned char)v52[7] | *(unsigned char *)((long)v52 + 0x33) | v52[4] != 0)) {
        *(char *)((long)v52 + 0x37) = 0;
        v73 = &v56;
        *(unsigned short *)&v52[6] = 0;
        sub_7140(v52,v73);
        sub_7310(v73); // no-return
      }
      v52 = (unsigned long *)v52[8];
label_58ff:
    } while (v52);
    if (dat_1d400) {
label_4775:
      if (v14 || v96) {
        s_1d018[0] = v14;
        if (!v14)
          s_1d018[0] = 'o';
        sub_7310(s_1d018); // no-return
      }
      if ((!v93) || (!setlocale(3,"")))
        error(0,0,"%s",dcgettext(NULL,"failed to set locale",5));
      if (dat_1d861) { // branch-flip
        v42 = sub_10fd0(setlocale(3,NULL));
        error(0,0,dcgettext(NULL,"text ordering performed using %s sorting rules",5),v42);
      }
      else {
        error(0,0,"%s",dcgettext(NULL,"text ordering performed using simple byte comparison",5));
      }
      v23 = 0x12;
      v70 = &v55;
      v72 = &v57;
      while (v23) {
        v3 = (void **)((long)v72 + 4);
        v2 = (unsigned long *)((long)v70 + 4);
        *(unsigned int *)v72 = *(unsigned int *)v70;
        v23 -= 1;
        v70 = v2;
        v72 = v3;
      }
      if (dat_1d410) {
        v100 = 0;
        v84 = 0;
        v15 = '\0';
        v83 = 0;
        v94 = (void **)0x1;
        v52 = dat_1d410;
        v80 = v115;
        v79 = v119;
        v77 = v121;
        v68 = v122;
        do {
          v13 = v120;
          v12 = v118;
          v11 = v117;
          v10 = v116;
          v9 = v114;
          if (v52[6] & 0xffff00ff0000) {
            v69 = *(char *)((long)v52 + 0x34);
            if (!v69) {
              v83 = 1;
              v69 = v15;
            }
            v15 = v69;
          }
          if (*(char *)((long)v52 + 0x39)) {
            v25 = *v52;
            if (*v52 == 0xffffffffffffffff)
              v25 = 0;
            v49 = v52[2];
            v60 = 0x2b;
            v75 = __stpcpy_chk(v59,(char *)sub_e200(v25,v58),0x2d);
            v56 = 0x206b2d;
            v27 = __stpcpy_chk((char *)((long)&v56 + 3),(char *)sub_e200(v25 + 1,v58),0x2c);
            if (v52[2] != 0xffffffffffffffff) {
              v28 = (char *)sub_e200(v49 + 1,v58);
              v75[0] = ' ';
              v75[1] = '-';
              strcpy(&v75[2],v28);
              v75 = (char *)sub_e200((unsigned long)(v52[3] == 0xffffffffffffffff) + v49 + 1,v58);
              *v27 = ',';
              strcpy(&v27[1],v75);
            }
            v42 = sub_10fb0(1,&v56);
            v29 = sub_10fb0(0,&v60);
            error(0,0,dcgettext(NULL,"obsolescent key %s used; consider %s instead",5),v29,v42);
          }
          v25 = v52[2];
          v7 = v85;
          if ((*v52 != 0xffffffffffffffff) && (v25 < *v52)) {
            error(0,0,dcgettext(NULL,"key %lu has zero width and will be ignored",5),v94);
            v25 = v52[2];
            v7 = 1;
          }
          v69 = '\x01';
          v49 = v52[6] & 0xffff00ff0000;
          if (!v49)
            v69 = *(char *)((long)v52 + 0x36);
          if ((v25) || (!v52[3])) {
            if (!v7) {
              if ((dat_1d028 == 0x80) && (((!(char)v52[6] && ((!v69 || (v52[1])))) || ((!*(char *)((long)v52 + 0x31) && (v52[3])))))) {
                error(0,0,dcgettext(NULL,"leading blanks are significant in key %lu; consider also specifying \'b\'",5),v94);
                v49 = v52[6] & 0xffff00ff0000;
              }
              goto label_48e3;
            }
label_5a20:
            if (!v85) goto label_48e3;
label_4908:
            v16 = *(unsigned char *)((long)v52 + 0x34);
          }
          else {
            if (v7) goto label_5a20;
label_48e3:
            if (!v49) goto label_4908;
            v25 = *v52 + 1;
            if (!v25)
              v25 = 1;
            if (v52[2] < v25) goto label_4908;
            error(0,0,dcgettext(NULL,"key %lu is numeric and spans multiple fields",5),v94);
            v16 = *(unsigned char *)((long)v52 + 0x34);
            if (v16) // branch-flip
              v100 = v16;
            else {
              v84 = 1;
            }
          }
          if ((v112) && (v112 == v52[4]))
            v112 = 0;
          if ((v113) && (v113 == v52[5]))
            v113 = 0;
          v118 = v12 & (v16 ^ 1);
          v114 = v9 & ((unsigned char)v52[6] ^ 1);
          v79 &= *(unsigned char *)((long)v52 + 0x35) ^ 1;
          v80 &= *(unsigned char *)((long)v52 + 0x31) ^ 1;
          v117 = v11 & (*(unsigned char *)((long)v52 + 0x33) ^ 1);
          v120 = v13 & (*(unsigned char *)((long)v52 + 0x36) ^ 1);
          v122 = v68 & ((unsigned char)v52[7] ^ 1);
          v116 = v10 & (*(unsigned char *)((long)v52 + 0x32) ^ 1);
          v77 &= *(unsigned char *)((long)v52 + 0x37) ^ 1;
          v52 = (unsigned long *)v52[8];
          v94 = (long)((long)v94 + 1);
          v115 = v80;
          v119 = v79;
          v121 = v77;
          v68 = v122;
        } while (v52);
        if (v84) {
          if (dat_1d028 != 0x80) {
            if (dat_1d028 == dat_1d864) {
label_6381:
              v56._0_2_ = (unsigned short)(unsigned char)dat_1d864;
              v42 = sub_10fd0(&v56);
              error(0,0,dcgettext(NULL,"field separator %s is treated as a group separator in numbers",5),v42);
              goto label_63c4;
            }
            v84 = 0;
            goto label_5aa7;
          }
          if (dat_1d864 != 0x80) { // branch-flip
            v23 = *v21;
            if (*(unsigned char *)(v23 + ((unsigned long)dat_1d864 & 0xff) * 2) & 1) goto label_6381;
            v84 = 0;
label_640e:
            if (*(unsigned char *)(v23 + (unsigned long)dat_1d868 * 2) & 1) goto label_641f;
          }
          else {
            v84 = 0;
          }
          goto label_5b0c;
        }
        if (v100) { // branch-flip
label_63c4:
          if (dat_1d028 == 0x80) {
            if (dat_1d864 != 0x80) {
              v23 = *v21;
              goto label_640e;
            }
            goto label_5b0c;
          }
label_5aa7:
          if (dat_1d028 != (int)(char)dat_1d868) { // branch-flip
            if (dat_1d028 != 0x2d) { // branch-flip
              if (dat_1d028 == 0x2b) {
                if (v100) {
                  v56 = CONCAT22(v56._2_2_,0x2b);
                  v42 = sub_10fd0(&v56);
                  v75 = "field separator %s is treated as a plus sign in numbers";
                  goto label_5af4;
                }
              }
            }
            else {
              v56 = CONCAT22(v56._2_2_,0x2d);
              v42 = sub_10fd0(&v56);
              v75 = "field separator %s is treated as a minus sign in numbers";
label_5af4:
              error(0,0,dcgettext(NULL,v75,5),v42);
            }
label_5b0c:
            if (!v83) {
              if (!v15) goto label_5b59;
            }
            if (!v84) goto label_5d6d;
          }
          else {
label_641f:
            v56 = CONCAT31((undefined3)(v56 >> 8),dat_1d868) & 0xffff00ff;
            v42 = sub_10fd0(&v56);
            error(0,0,dcgettext(NULL,"field separator %s is treated as a decimal point in numbers",5),v42);
          }
        }
        else {
          if (!v83 && !v15) goto label_5b59;
label_5d6d:
          v75 = "";
          v56 = CONCAT31((undefined3)(v56 >> 8),dat_1d868) & 0xffff00ff;
          v42 = sub_10fd0(&v56);
          if ((int)(char)dat_1d868 != dat_1d028)
            v75 = dcgettext(NULL,"note ",5);
          error(0,0,dcgettext(NULL,"%snumbers use %s as a decimal point in this locale",5),v75,v42);
        }
        if (v83) {
          if (dat_1d862)
            error(0,0,dcgettext(NULL,"the multi-byte number group separator in this locale is not supported",5));
        }
      }
label_5b59:
      v15 = sub_70e0(&v57);
      v80 = v121;
      if (v15) { // branch-flip
        v121 = v80;
        if (v121) {
          if (dat_1d41a) { // branch-flip
            if (dat_1d410) {
              sub_7140(&v57,&v56);
              v25 = strlen((char *)&v56);
              error(0,0,(char *)dcngettext(0,"option \'-%s\' is ignored","options \'-%s\' are ignored",v25,5),&v56);
              v121 = 1;
label_5cd4:
              if (!dat_1d41a) goto label_5ce1;
            }
          }
          else {
            if ((dat_1d419) && (dat_1d410)) {
              sub_7140(&v57,&v56);
              v25 = strlen((char *)&v56);
              error(0,0,(char *)dcngettext(0,"option \'-%s\' is ignored","options \'-%s\' are ignored",v25,5),&v56);
              v121 = v80;
              goto label_5cd4;
            }
label_5ce1:
            if ((!dat_1d419) && (dat_1d410))
              error(0,0,dcgettext(NULL,"option \'-r\' only applies to last-resort comparison",5));
          }
        }
      }
      else {
        if ((!dat_1d41a) && (!dat_1d419))
          v121 = 0;
        sub_7140(&v57,&v56);
        v25 = strlen((char *)&v56);
        error(0,0,(char *)dcngettext(0,"option \'-%s\' is ignored","options \'-%s\' are ignored",v25,5),&v56);
        v121 = v80;
        if (v80) goto label_5cd4;
      }
    }
    dat_1d41b = v107[0xf];
    if (v50) {
      v23 = sub_11060(v98,0x10);
      if (!v23) {
        if (!v98)
          v98 = "getrandom";
        v75 = "open failed";
        sub_7480(dcgettext(NULL,v75,5),v98); // return-dupe, no-return
      }
      sub_11220(v23,&v56,0x10);
      if (sub_113c0(v23)) {
        v75 = "close failed";
        sub_7480(dcgettext(NULL,v75,5),v98);
      }
      MD5_Init(0x1d300);
      MD5_Update(0x1d300,&v56,0x10);
    }
  }
  else {
    if (!sub_70e0(&v55)) {
      sub_76e0();
      v85 = 1;
      v50 = v107[0xb];
      v52 = dat_1d410;
      goto label_58ff;
    }
    if (dat_1d400) {
      v85 = 0;
      v50 = 0;
      goto label_4775;
    }
    dat_1d41b = v107[0xf];
  }
  if (!dat_1d428) {
    v75 = getenv("TMPDIR");
    if (!v75)
      v75 = "/tmp";
    sub_7730(v75);
  }
  if (!v81) {
    v81 = 1;
    free(v20);
    v20 = (unsigned long *)sub_12ab0(8);
    *v20 = "-";
  }
  if (dat_1d438) {
    v31 = (void *)((unsigned long)dat_1d024 * 0x22);
    if ((void *)((unsigned long)dat_1d024 * 0x22) < dat_1d438)
      v31 = dat_1d438;
    dat_1d438 = v31;
  }
  if (v14) {
    if (v81 != 1) {
      v42 = sub_108e0(4,v20[1]);
      error(2,0,dcgettext(NULL,"extra operand %s not allowed with -%c",5),v42,(unsigned long)(unsigned int)(int)v14);
      return v47;
    }
    if (v96) {
      dat_1d010 = v14;
      sub_7310(0x1d010); // no-return
    }
    v42 = *v20;
    v23 = sub_74e0(v42,"r");
    v52 = dat_1d410;
    if (!v23)
      sub_7c60(v42); // no-return
    v51 = (unsigned int)dat_1d419;
    v31 = dat_1d030;
    if (dat_1d030 <= dat_1d438)
      v31 = dat_1d438;
    v25 = 0;
    sub_7790(&v57,0x20,v31);
    v54 = NULL;
    v92 = NULL;
    do {
      v15 = sub_7ef0(&v57,v23,v42);
      v81 = v110;
      if (!v15) {
        v51 = 1;
label_5e04:
        sub_7660(v23,v42);
        free(v57);
        free(v54);
        exit(v51 ^ 1); // no-return
      }
      v22 = (long *)(v111 + (long)v57);
      v26 = v110 * -4;
      if ((v25) && ((int)(v51 ^ 1) <= (int)sub_a0b0(&v54,&v22[-4]))) {
        v21 = v22;
label_5df7:
        v51 = 0;
        if (v14 == 'c') {
          v38 = sub_e200(((long)v22 - (long)&v21[-4] >> 5) + (long)v92,&v56);
          v29 = dat_1d888;
          __fprintf_chk(stderr,1,dcgettext(NULL,"%s: %s:%s: disorder: ",5),v29,v42,v38);
          sub_82d0(&v21[-4],stderr,dcgettext(NULL,"standard error",5));
        }
        goto label_5e04;
      }
      v37 = &v22[-4];
      while (v21 = v37, &v22[v26] < v21) {
        v37 = &v21[-4];
        if ((int)(v51 ^ 1) <= (int)sub_a0b0(v21,&v21[-4])) goto label_5df7;
      }
      v49 = v21[1];
      v92 = (long)((long)v92 + v81);
      if (v25 < v49) {
        do {
          v25 *= 2;
          if (!v25) {
            v25 = v49;
            break;
          }
        } while (v25 < v49);
        free(v54);
        v54 = (void *)sub_12ab0(v25);
        v49 = v21[1];
      }
      v31 = memcpy(v54,(void *)*v21,v49);
      v101 = v21[1];
      if (v52) {
        v102 = (long)v31 + (v21[2] - *v21);
        v103 = (long)v31 + (v21[3] - *v21);
      }
    } while( true );
  }
  v95 = 0;
  v30 = v20;
  do {
    v75 = (char *)*v30;
    if ((strcmp(v75,"-")) && (euidaccess(v75,4))) {
      v42 = *v30;
      v75 = "cannot read";
      sub_7480(dcgettext(NULL,v75,5),v42); // return-dupe, no-return
    }
    v95 += 1;
    v30 = &v30[1];
  } while (v81 != v95);
  if (v96) {
    v17 = open(v96,0x80041,0x1b6);
    if (v17 < 0)
      sub_7480(dcgettext(NULL,"open failed",5),v96);
    if (v17 != 1)
      sub_7c90(v17,1);
  }
  if (!v8) {
    if (!v99) {
      v25 = sub_e570(2);
      v99 = 8;
      if (v25 <= 8)
        v99 = v25;
    }
    v91 = 0;
    v111 = 0;
    v25 = 0xffffffffffffff;
    if (v99 <= 0xffffffffffffff)
      v25 = v99;
    v23 = v25 * 2;
label_5328:
    v42 = *v20;
    v31 = (void *)sub_74e0(v42,"r");
    if (!v31)
      sub_7c60(v42); // no-return
    v97 = 0x30;
    if (2 <= v99) {
      v97 = 1;
      v81 = 1;
      do {
        v81 *= 2;
        v97 += 1;
      } while (v81 < v25);
      v97 *= 0x20;
    }
    if (!v111) {
      v76 = (void *)(v97 + 2);
      v81 = 0;
      do {
        if (v81) { // branch-flip
          v75 = (char *)v20[v81];
          v85 = (strcmp(v75,"-")) ? stat(v75,&v53) != 0 : fstat(0,&v53) != 0; // branch-flip
        }
        else {
          v17 = fileno(v31);
          v85 = fstat(v17,&v53) != 0;
        }
        if (v85) {
          v42 = v20[v81];
          v75 = "stat failed";
          sub_7480(dcgettext(NULL,v75,5),v42);
        }
        if (((unsigned int)v126 & 0xf000) != 0x8000) { // branch-flip
          v35 = dat_1d438;
          if (dat_1d438) goto label_60e5;
          v93 = (char *)0x20000;
          v35 = dat_1d240;
          if (!dat_1d240) {
            v93 = (char *)0x20000;
label_5f41:
            v35 = (void *)0xffffffffffffffff;
            if (!getrlimit(2))
              v35 = v54;
            if ((!getrlimit(9)) && (v54 < v35))
              v35 = v54;
            v35 = (void *)((unsigned long)v35 >> 1);
            if (!getrlimit(5,&v54)) {
              v36 = (void *)(((unsigned long)v54 >> 4) * 0xf);
              if (v36 < v35)
                v35 = v36;
            }
            sub_e850();
            sub_e760();
            v88 = v87 * dat_17110;
            v90 = v86;
            if (v86 <= dat_17108 * v87)
              v90 = dat_17108 * v87;
            if (v88 < (double)(long)v35) {
              v35 = (dat_17100 <= v88) ? (void *)((long)(v88 - dat_17100) ^ 0x8000000000000000) : (void *)(long)v88;
            }
            if (v90 < (double)v35) {
              v35 = (dat_17100 <= v90) ? (void *)((long)(v90 - dat_17100) ^ 0x8000000000000000) : (void *)(long)v90;
            }
            dat_1d240 = (void *)((unsigned long)dat_1d024 * 0x22);
            if ((void *)((unsigned long)dat_1d024 * 0x22) < v35)
              dat_1d240 = v35;
            v35 = dat_1d240;
          }
        }
        else {
          v93 = (char *)v63;
          v35 = dat_1d240;
          if (!dat_1d240) {
            dat_1d240 = dat_1d438;
            v35 = dat_1d438;
            if (!dat_1d438) goto label_5f41;
          }
        }
        v49 = (long)v93 * (v97 + 1U) + 1;
        if ((v93 != (char *)(v49 / (v97 + 1U))) || ((unsigned long)((long)v35 - (long)v76) <= v49)) goto label_60e5;
        v76 = (void *)((long)v76 + v49);
        v81 += 1;
        v35 = v76;
        if (v81 >= v95) goto label_60e5;
      } while( true );
    }
    goto label_5385;
  }
  v32 = (unsigned long *)sub_12e90(v95,0x10);
  v30 = &v20[v95];
  v33 = v32;
  do {
    v42 = *v20;
    v20 = &v20[1];
    v34 = &v33[2];
    *v33 = v42;
    v33 = v34;
  } while (v30 != v20);
  sub_ac90(v32,0,v95,v96);
label_55b4:
  if ((dat_1d418) && (v17 = sub_c5b0(stdin), v17 == -1))
    sub_7480(dcgettext(NULL,"close failed",5),"-"); // no-return
  if (v140 != *(long *)(v82 + 0x28))
    __stack_chk_fail(); // no-return
  return 0;
}

// Function: sub_6e50 @ 0x6e50
void sub_6e50(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_1cff8)(sub_3ec0,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_6e80 @ 0x6e80
void sub_6e80(void)
{
  return;
}

// Function: _FINI_0 @ 0x6ef0
void _FINI_0(void)
{
  if (!dat_1d230) {
    if (dat_1cff0)
      __cxa_finalize(dat_1d008);
    sub_6e80();
    dat_1d230 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x6f30
void _INIT_0(void)
{
  return;
}

// Function: sub_6f40 @ 0x6f40
unsigned long sub_6f40(long a0,unsigned long a1)
{
  return (unsigned long)(long)*(int *)(a0 + 8) % a1;
}

// Function: sub_6f60 @ 0x6f60
unsigned int sub_6f60(long a0,long a1)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)(a1 + 8) >> 8),*(int *)(a0 + 8) == *(int *)(a1 + 8));
}

// Function: sub_6f70 @ 0x6f70
int sub_6f70(long *a0)
{
  char *v1;
  char *v2;
  int v3; // ecx
  char v4;
  char *v5;
  int v6;
  
  v2 = (char *)*a0;
  v4 = *v2;
  v3 = (int)v4;
  v5 = &v2[1];
  if (10 <= (unsigned int)(v3 - 0x30U)) // branch-flip
    v6 = 0;
  else {
    v6 = 0;
    do {
      if ((char)v6 < (char)v3)
        v6 = v3;
      v1 = &v2[1];
      v2 = &v2[2];
      if (*v1 != dat_1d864)
        v2 = v5;
      v4 = *v2;
      v3 = (int)v4;
      v5 = &v2[1];
    } while ((unsigned int)(v3 - 0x30U) <= 9);
    if (*v1 == dat_1d864) {
      *a0 = (long)&v2[-1];
      return v6;
    }
  }
  if (dat_1d868 != v4) // branch-flip
    v5 = &v5[-1];
  else {
    v4 = *v5;
    while ((unsigned int)((int)v4 - 0x30U) <= 9) {
      v5 = &v5[1];
      if ((char)v6 < v4)
        v6 = (int)v4;
      v4 = *v5;
    }
  }
  *a0 = (long)v5;
  return v6;
}

// Function: sub_7030 @ 0x7030
unsigned int sub_7030(unsigned char *a0,unsigned long *a1) // early-return
{
  char v1;
  unsigned char v2;
  unsigned char *v3;
  unsigned char *v4;
  unsigned long v5; // r9
  unsigned long v6;
  unsigned long v7;
  
  v1 = *(char *)((unsigned long)*a0 + 0x1d740);
  while (v1) {
    v4 = &a0[1];
    a0 = &a0[1];
    v1 = *(char *)((unsigned long)*v4 + 0x1d740);
  }
  v6 = 0xc;
  v7 = 0;
  do {
    v5 = v7 + v6 >> 1;
    v4 = *(unsigned char **)(v5 * 0x10 + 0x1d060);
    v2 = *v4;
    v3 = a0;
    while( true ) {
      if (!v2) {
        if (a1)
          *a1 = v3;
        return *(unsigned int *)(v5 * 0x10 + 0x1d068);
      }
      if (*(unsigned char *)((unsigned long)*v3 + 0x1d440) < v2) {
        v6 = v5;
        goto label_70ae;
      }
      if (v2 < *(unsigned char *)((unsigned long)*v3 + 0x1d440)) break;
      v2 = v4[1];
      v4 = &v4[1];
      v3 = &v3[1];
    }
    v7 = v5 + 1;
label_70ae:
    if (v6 <= v7)
      return 0;
  } while( true );
}

// Function: sub_70e0 @ 0x70e0
unsigned char sub_70e0(long a0)
{
  if ((!*(long *)(a0 + 0x20)) && (!*(long *)(a0 + 0x28))) {
    if (*(char *)(a0 + 0x30))
      return 0;
    if ((!*(char *)(a0 + 0x31)) && (!(*(unsigned long *)(a0 + 0x30) & 0xffff00ff0000))) {
      if (*(char *)(a0 + 0x36))
        return 0;
      if (!*(char *)(a0 + 0x38))
        return *(unsigned char *)(a0 + 0x33) ^ 1;
    }
  }
  return 0;
}

// Function: sub_7140 @ 0x7140
void sub_7140(long a0,char *a1)
{
  if (*(short *)(a0 + 0x30)) {
    *a1 = 0x62;
    a1 = &a1[1];
  }
  if (*(long *)(a0 + 0x20) == 0x1d540) {
    *a1 = 100;
    a1 = &a1[1];
  }
  if (*(long *)(a0 + 0x28)) {
    *a1 = 0x66;
    a1 = &a1[1];
  }
  if (*(char *)(a0 + 0x34)) {
    *a1 = 0x67;
    a1 = &a1[1];
  }
  if (*(char *)(a0 + 0x35)) {
    *a1 = 0x68;
    a1 = &a1[1];
  }
  if (*(long *)(a0 + 0x20) == 0x1d640) {
    *a1 = 0x69;
    a1 = &a1[1];
  }
  if (*(char *)(a0 + 0x36)) {
    *a1 = 0x4d;
    a1 = &a1[1];
  }
  if (*(char *)(a0 + 0x32)) {
    *a1 = 0x6e;
    a1 = &a1[1];
  }
  if (*(char *)(a0 + 0x33)) {
    *a1 = 0x52;
    a1 = &a1[1];
  }
  if (*(char *)(a0 + 0x37)) {
    *a1 = 0x72;
    a1 = &a1[1];
  }
  if (*(char *)(a0 + 0x38)) {
    *a1 = 0x56;
    a1 = &a1[1];
  }
  *a1 = 0;
}

// Function: sub_71f0 @ 0x71f0
char * sub_71f0(char *a0,long a1,unsigned int a2)
{
  char v1;
  
  v1 = *a0;
  if (v1) {
    do {
      switch(v1) {
        case 0x4d:
          *(char *)(a1 + 0x36) = 1;
          break;
        default:
          goto label_724d;
        case 0x52:
          *(char *)(a1 + 0x33) = 1;
          break;
        case 0x56:
          *(char *)(a1 + 0x38) = 1;
          break;
        case 0x62:
          if (!(a2 & 0xfffffffd))
            *(char *)(a1 + 0x30) = 1;
          if (a2 - 1 <= 1)
            *(char *)(a1 + 0x31) = 1;
          break;
        case 100:
          *(unsigned long *)(a1 + 0x20) = 0x1d540;
          break;
        case 0x66:
          *(unsigned long *)(a1 + 0x28) = 0x1d440;
          break;
        case 0x67:
          *(char *)(a1 + 0x34) = 1;
          break;
        case 0x68:
          *(char *)(a1 + 0x35) = 1;
          break;
        case 0x69:
          if (!*(long *)(a1 + 0x20))
            *(unsigned long *)(a1 + 0x20) = 0x1d640;
          break;
        case 0x6e:
          *(char *)(a1 + 0x32) = 1;
          break;
        case 0x72:
          *(char *)(a1 + 0x37) = 1;
        
      }
      v1 = a0[1];
      a0 = &a0[1];
    } while (v1);
  }
label_724d:
  return a0;
}

// Function: sub_7300 @ 0x7300
void sub_7300(unsigned long *a0,unsigned long *a1)
{
  strcmp((char *)*a0,(char *)*a1); // tail-call
}

// Function: sub_7310 @ 0x7310
void sub_7310(unsigned long a0)
{
  error(2,0,dcgettext(NULL,"options \'-%s\' are incompatible",5),a0);
}

// Function: sub_7340 @ 0x7340
void sub_7340(unsigned long a0,char *a1)
{
  unsigned long v1; // rax
  char *v2; // rax
  
  v1 = sub_10fd0(a0);
  v2 = dcgettext(NULL,a1,5);
  error(2,0,dcgettext(NULL,"%s: invalid field specification %s",5),v2,v1);
}

// Function: sub_7390 @ 0x7390
char * sub_7390(unsigned long a0,unsigned long *a1,char *a2)
{
  unsigned long v1;
  char *v2;
  char *v3; // rax
  unsigned long v4; // stack - 0x28
  unsigned long v5; // stack - 0x30
  
  switch(sub_132f0(a0,&v5,10,&v4,"")) {
    case 0:
    case 2:
      *a1 = v4;
      break;
    case 1:
    case 3:
      *a1 = 0xffffffffffffffff;
      break;
    case 4:
      if (!a2) {
        v3 = NULL;
        return v3;
      }
      v3 = (unsigned long)sub_10fd0(a0);
      v1 = v3;
      v3 = dcgettext(NULL,a2,5);
      v2 = v3;
      v3 = dcgettext(NULL,"%s: invalid count at start of %s",5);
      error(2,0,v3,v2,v1);
      return v3;
    
  }
  v3 = (char *)v5;
  return v3;
}

// Function: sub_7480 @ 0x7480
void sub_7480(unsigned long a0,char *a1)
{
  unsigned long v1; // rax
  
  if (!a1)
    a1 = dcgettext(NULL,"standard output",5);
  v1 = sub_10c60(0,3,a1);
  error(2,*__errno_location(),"%s: %s",a0,v1);
}

// Function: sub_74e0 @ 0x74e0
int * sub_74e0(char *a0,char *a1)
{
  int v1;
  int *v2;
  unsigned long v3;
  unsigned int v4;
  int *v5; // rax
  void *v6;
  
  if (*a1 == 'r') {
    v4 = strcmp(a0,"-");
    if (v4) { // branch-flip
      v6 = NULL;
      v4 = open(a0,0x80000);
      if (0 <= (int)v4) {
        v5 = fdopen(v4,a1);
        v6 = v5;
      }
    }
    else {
      dat_1d418 = 1;
      v6 = stdin;
    }
    sub_c580(v6,2);
    v5 = v6;
    return v5;
  }
  if (*a1 != 'w')
    __assert_fail("!\"unexpected mode passed to stream_open\"","src/sort.c",0x3d5,"stream_open"); // no-return
  if (!a0) {
    v5 = stdout;
    return v5;
  }
  v4 = ftruncate(1,0);
  if (!v4) {
    v5 = stdout;
    return v5;
  }
  v5 = __errno_location();
  v2 = v5;
  v1 = *v5;
  v4 = dat_1d2f0;
  if (dat_1d2f0) { // branch-flip
label_7579:
    if (0 <= (int)v4) goto label_7593;
  }
  else {
    v4 = fstat(1,(void *)0x1d260);
    if (v4) {
      v4 = *v2;
      dat_1d2f0 = v4;
      goto label_7579;
    }
    dat_1d2f0 = -1;
  }
  v4 = dat_1d278 & 0xf000;
  if (v4 != 0x8000) {
    v5 = stdout;
    return v5;
  }
label_7593:
  v5 = (unsigned long)sub_10c60(0,3,a0);
  v3 = v5;
  v5 = (char *)dcgettext(NULL,"%s: error truncating",5);
  error(2,v1,(char *)v5,v3);
  return v5;
}

// Function: sub_7660 @ 0x7660
void sub_7660(void *a0,unsigned long a1) // return-dupe
{
  int v1; // eax
  char *v2; // rsi
  
  v1 = fileno(a0);
  if (!v1) {
    clearerr_unlocked(a0); // tail-call
    return;
  }
  if (v1 != 1) { // branch-flip
    if (!sub_c5b0(a0))
      return;
    v2 = "close failed";
  }
  else {
    if (!fflush_unlocked(a0))
      return;
    v2 = "fflush failed";
  }
  sub_7480(dcgettext(NULL,v2,5),a1); // no-return
}

// Function: sub_76e0 @ 0x76e0
void sub_76e0(unsigned long a0)
{
  long v1;
  long v2; // rax
  long *v3; // rcx
  long v4; // rdx
  
  v2 = sub_12ef0(a0,0x48);
  v3 = (long *)0x1d410;
  v1 = dat_1d410;
  if (dat_1d410) {
    do {
      v4 = v1;
      v1 = *(long *)(v4 + 0x40);
    } while (v1);
    v3 = (long *)(v4 + 0x40);
  }
  *v3 = v2;
  *(unsigned long *)(v2 + 0x40) = 0;
}

// Function: sub_7730 @ 0x7730
void sub_7730(unsigned long a0)
{
  long v1;
  
  v1 = dat_1d430;
  if (dat_1d428 == dat_1d420) {
    v1 = sub_12d00(dat_1d430,0x1d420,8);
    dat_1d430 = v1;
  }
  *(unsigned long *)(v1 + dat_1d428 * 8) = a0;
  dat_1d428 += 1;
}

// Function: sub_7790 @ 0x7790
void sub_7790(unsigned long *a0,long a1,unsigned long a2)
{
  void *v1; // rax
  
  do {
    a2 = (a2 & 0xffffffffffffffe0) + 0x20;
    v1 = malloc(a2);
    *a0 = v1;
    if (v1) {
      a0[5] = a1;
      a0[3] = a2;
      a0[2] = 0;
      a0[4] = 0;
      a0[1] = 0;
      *(char *)&a0[6] = 0;
      return;
    }
    a2 >>= 1;
  } while (a1 + 1U < a2);
  sub_12fb0(); // no-return
}

// Function: sub_7800 @ 0x7800
void sub_7800(unsigned long a0,unsigned long a1,unsigned long a2)
{
  int *v1; // rax
  char *v2; // rax
  unsigned long v3; // rax
  
  v1 = __errno_location();
  *v1 = 0;
  strxfrm(a0,a1,a2);
  if (!*v1)
    return;
  v2 = dcgettext(NULL,"string transformation failed",5);
  error(0,*v1,v2);
  error(0,0,dcgettext(NULL,"set LC_ALL=\'C\' to work around the problem",5));
  v3 = sub_107b0(0,8,a1);
  error(2,0,dcgettext(NULL,"the untransformed string was %s",5),v3);
}

// Function: sub_78c0 @ 0x78c0
unsigned long sub_78c0(int a0)
{
  unsigned int v1; // eax
  unsigned int v10; // stack - 0x30
  long v2; // rax
  char *v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  unsigned int v7; // stack - 0x3c
  char v8 [8];
  int v9;
  
  v9 = -1;
  if (a0)
    v9 = a0;
  v1 = waitpid(v9,(int *)&v7,(unsigned int)(a0 == 0));
  if ((int)v1 < 0) {
    v5 = sub_108e0(4,dat_1d408);
    v3 = dcgettext(NULL,"waiting for %s [-d]",5);
    error(2,*__errno_location(),v3,v5);
    return v4;
  }
  if (v1) {
    if (a0 <= 0) {
      v10 = v1;
      v2 = sub_dd00(dat_1d360,v8);
      if (!v2) {
        return (unsigned long)v1; // return-dupe
      }
      *(char *)(v2 + 0xc) = 2;
    }
    if (v7 >> 8 & 0xff || v7 & 0x7f) {
      v5 = sub_108e0(4,dat_1d408);
      error(2,0,dcgettext(NULL,"%s [-d] terminated abnormally",5),v5);
      return v6;
    }
    dat_1d35c -= 1;
  }
  return (unsigned long)v1;
}

// Function: sub_79f0 @ 0x79f0
void sub_79f0(int a0,char *a1)
{
  char *v1; // rax
  char v2 [12];
  long v3; // fs_offset
  unsigned long v4; // stack - 0x20
  
  v4 = *(unsigned long *)(v3 + 0x28);
  write(2,a1,strlen(a1));
  if (a0) {
    v1 = (char *)sub_e120(a0,v2);
    write(2,": errno ",8);
    write(2,v1,strlen(v1));
  }
  write(2,"\n",1);
  _exit(2); // no-return
}

// Function: sub_7a90 @ 0x7a90
void sub_7a90(long a0)
{
  long v1;
  
  v1 = dat_1d360;
  if (!dat_1d360) {
    v1 = sub_d640(0x2f,0,sub_6f40,sub_6f60,0);
    dat_1d360 = v1;
    if (!v1) {
      sub_12fb0(); // no-return, return-dupe
    }
  }
  *(char *)(a0 + 0xc) = 1;
  if (sub_dca0(v1,a0))
    return;
  sub_12fb0();
}

// Function: sub_7af0 @ 0x7af0
long * sub_7af0(long *a0,long *a1,long a2,unsigned long a3,long a4,char a5)
{
  long *v1;
  long v2;
  unsigned long v3; // rcx
  long v4; // rdx
  long *v5; // rsi
  long *v6;
  long v7; // r12
  
  while( true ) {
    v5 = a1;
    v7 = a2 + a4 * -0x20;
    if (a5) { // branch-flip
      v3 = (unsigned long)a0[5] >> 1;
      v4 = a0[5] - v3;
      v6 = &a0[2];
    }
    else {
      v3 = (unsigned long)a0[6] >> 1;
      v4 = a0[6] - v3;
      v6 = &a0[3];
    }
    a2 = v7 + v3 * -0x20;
    v5[7] = (long)a0;
    v2 = a0[10];
    v1 = &v5[0x10];
    v5[4] = (long)v6;
    v5[2] = v7;
    *v5 = v7;
    v5[3] = a2;
    v5[1] = a2;
    v5[5] = v3;
    v5[6] = v4;
    *(int *)&v5[10] = (int)v2 + 1;
    *(char *)((long)v5 + 0x54) = 0;
    pthread_mutex_init(&v5[0xb],0);
    if (a3 <= 1) break;
    v5[8] = (long)v1;
    v3 = a3 >> 1;
    a3 -= v3;
    a1 = (long *)sub_7af0(v5,v1,v7,v3,a4,1);
    v5[9] = (long)a1;
    a5 = '\0';
    a0 = v5;
  }
  v5[8] = 0;
  v5[9] = 0;
  return v1;
}

// Function: sub_7c20 @ 0x7c20
bool sub_7c20(long a0,long a1)
{
  if (*(unsigned int *)(a0 + 0x50) != *(unsigned int *)(a1 + 0x50))
    return *(unsigned int *)(a0 + 0x50) < *(unsigned int *)(a1 + 0x50);
  return (unsigned long)(*(long *)(a0 + 0x30) + *(long *)(a0 + 0x28)) < (unsigned long)(*(long *)(a1 + 0x30) + *(long *)(a1 + 0x28));
}

// Function: sub_7c60 @ 0x7c60
void sub_7c60(unsigned long a0)
{
  sub_7480(dcgettext(NULL,"open failed",5),a0); // no-return
}

// Function: sub_7c90 @ 0x7c90
void sub_7c90(int a0,int a1)
{
  dup2(a0,a1);
  close(a0); // tail-call
}

// Function: sub_7ca0 @ 0x7ca0
unsigned char * sub_7ca0(unsigned char *a0,long a1,long a2)
{
  unsigned char *v1;
  long v2;
  unsigned char *v3;
  char v4;
  long v5; // rcx
  long v6;
  bool v7; // cf
  
  v1 = &a0[a1 + -1];
  v2 = *(long *)(a2 + 0x18);
  v5 = *(long *)(a2 + 0x10) + (unsigned long)(v2 == 0);
  if (dat_1d028 != 0x80) { // branch-flip
    if ((a0 < v1) && (v6 = v5 + -1, v5)) {
      do {
        while (dat_1d028 == (char)*a0) {
          if (v1 <= a0) goto label_7cf0;
          if (!v2 && !v6)
            return a0;
          a0 = &a0[1];
          if ((v1 <= a0) || (v7 = v6 == 0, v6 = v6 + -1, v7)) goto label_7cf0;
        }
        a0 = &a0[1];
      } while (a0 < v1);
    }
  }
  else if ((a0 < v1) && (v6 = v5 + -1, v5)) {
    do {
      v4 = *(char *)((unsigned long)*a0 + 0x1d740);
      while (!v4) {
        if (v1 <= a0) goto label_7cf0;
        do {
          a0 = &a0[1];
          if (v1 == a0) goto label_7cf0;
          v4 = *(char *)((unsigned long)*a0 + 0x1d740);
        } while (!v4);
        if ((v1 <= a0) || (v7 = v6 == 0, v6 = v6 + -1, v7)) goto label_7cf0;
      }
      a0 = &a0[1];
    } while (a0 < v1);
  }
label_7cf0:
  v3 = a0;
  if (v2) {
    if ((*(char *)(a2 + 0x31)) && (a0 < v1)) {
      do {
        if (!*(char *)((unsigned long)*a0 + 0x1d740)) break;
        a0 = &a0[1];
      } while (v1 != a0);
    }
    v3 = &a0[v2];
    if (v1 <= &a0[v2])
      v3 = v1;
  }
  return v3;
}

// Function: sub_7dd0 @ 0x7dd0
unsigned char * sub_7dd0(unsigned char *a0,long a1,long *a2)
{
  long v1;
  long v2;
  unsigned char *v3; // rax
  char v4;
  long v5;
  bool v6; // cf
  
  v3 = &a0[a1 + -1];
  v1 = *a2;
  v2 = a2[1];
  if (dat_1d028 != 0x80) { // branch-flip
    if (v3 <= a0) goto label_7e10;
    v5 = v1 + -1;
    if (v1) {
      do {
        while (dat_1d028 == (char)*a0) {
          if ((v3 <= a0) || (a0 = &a0[1], v3 <= a0)) goto label_7e10;
          v6 = v5 == 0;
          v5 -= 1;
          if (v6) goto label_7e34;
        }
        a0 = &a0[1];
      } while (a0 < v3);
      goto label_7e10;
    }
  }
  else {
    if (v3 <= a0) goto label_7e10;
    v5 = v1 + -1;
    if (v1) {
      do {
        v4 = *(char *)((unsigned long)*a0 + 0x1d740);
        while (!v4) {
          if (v3 <= a0) goto label_7e10;
          do {
            a0 = &a0[1];
            if (v3 == a0) goto label_7e10;
            v4 = *(char *)((unsigned long)*a0 + 0x1d740);
          } while (!v4);
          if (v3 <= a0) goto label_7e10;
          v6 = v5 == 0;
          v5 -= 1;
          if (v6) goto label_7e34;
        }
        a0 = &a0[1];
        if (v3 <= a0) {
          if (&a0[v2] < v3)
            v3 = &a0[v2];
          return v3;
        }
      } while( true );
    }
  }
label_7e34:
  if ((char)a2[6]) {
    do {
      if (!*(char *)((unsigned long)*a0 + 0x1d740)) {
        if (&a0[v2] < v3)
          v3 = &a0[v2];
        return v3;
      }
      a0 = &a0[1];
    } while (a0 < v3);
  }
label_7e10:
  if (&a0[v2] < v3)
    v3 = &a0[v2];
  return v3;
}

// Function: sub_7ef0 @ 0x7ef0
unsigned long sub_7ef0(long *a0,unsigned int *a1,unsigned long a2)
{
  long v1;
  unsigned long v10;
  unsigned char *v11;
  unsigned char *v12;
  unsigned char *v13;
  long v14;
  unsigned char *v15; // r15
  unsigned char *v16; // stack - 0x88
  unsigned char v2;
  long *v3;
  unsigned long v4;
  unsigned long v5; // rax
  char *v6; // rax
  long v7;
  unsigned char *v8;
  unsigned long v9; // stack - 0x48
  
  v3 = dat_1d410;
  v2 = dat_1d038;
  v14 = dat_1d030;
  v1 = a0[5];
  if ((char)a0[6])
    return 0;
  v4 = a0[1];
  v10 = a0[4];
  if (v4 != v10) { // branch-flip
    memmove((void *)*a0,(void *)(*a0 + (v4 - v10)),v10);
    v4 = a0[4];
    a0[2] = 0;
    v7 = 0;
    a0[1] = v4;
  }
  else {
    v7 = a0[2];
  }
  v10 = v14 - 0x22;
  v11 = (unsigned char *)*a0;
  v14 = a0[3];
  do {
    v12 = &v11[v4];
    v13 = &v11[v7 * -0x20 + v14];
    v8 = &v11[(v14 - v1 * v7) - (long)v12];
    if (v7)
      v11 = (unsigned char *)(*(long *)&v13[8] + *(long *)v13);
    do {
      if (v8 <= (unsigned char *)(v1 + 1)) {
        v16 = v12;
        break;
      }
      v4 = (unsigned long)&v8[-1] / (unsigned long)(unsigned char *)(v1 + 1);
      v5 = fread_unlocked(v12,1,v4,a1);
      v8 = &v8[-v5];
      v16 = &v12[v5];
      if (v4 != v5) {
        if (*a1 & 0x20)
          sub_7480(dcgettext(NULL,"read failed",5),a2); // no-return
        if (*a1 & 0x10) {
          *(char *)&a0[6] = 1;
          if ((unsigned char *)*a0 == v16)
            return 0;
          if ((v11 != v16) && (v16[-1] != v2)) {
            *v16 = v2;
            v16 = &v16[1];
          }
        }
      }
      while (v6 = memchr(v12,(int)(char)v2,(long)v16 - (long)v12), v6) {
        v12 = &v6[1];
        v15 = &v13[-0x20];
        *v6 = 0;
        *(unsigned char **)v15 = v11;
        v4 = (long)v12 - (long)v11;
        *(unsigned long *)&v13[-0x18] = v4;
        if (v10 < v4)
          v10 = v4;
        v8 = &v8[-v1];
        if (v3) {
          if (v3[2] != -1)
            v6 = (char *)sub_7ca0(v11,v4,v3);
          v14 = *v3;
          *(char **)&v13[-8] = v6;
          if (v14 != -1) // branch-flip
            *(unsigned long *)&v13[-0x10] = sub_7dd0(v11,v4,v3);
          else {
            if ((char)v3[6]) {
              while (*(char *)((unsigned long)*v11 + 0x1d740)) {
                v11 = &v11[1];
              }
            }
            *(unsigned char **)&v13[-0x10] = v11;
          }
        }
        v11 = v12;
        v13 = v15;
      }
      v12 = v16;
    } while (!(char)a0[6]);
    v14 = *a0;
    a0[1] = (long)v16 - v14;
    v7 = (v14 + a0[3]) - (long)v13 >> 5;
    a0[2] = v7;
    if (v7) {
      a0[4] = (long)v16 - (long)v11;
      dat_1d030 = v10 + 0x22;
      return 1;
    }
    v9 = (unsigned long)a0[3] >> 5;
    v11 = (unsigned char *)sub_12d00(v14,&v9,0x20);
    v14 = v9 << 5;
    *a0 = (long)v11;
    v4 = a0[1];
    a0[3] = v14;
    v7 = a0[2];
  } while( true );
}

// Function: sub_8270 @ 0x8270
void sub_8270(int a0)
{
  unsigned long *v1;
  
  for (v1 = dat_1d368; v1; v1 = (unsigned long *)*v1) {
    unlink((char *)((long)v1 + 0xd));
  }
  dat_1d368 = NULL;
  signal(a0,0);
  raise(a0); // tail-call
}

// Function: sub_82d0 @ 0x82d0
void sub_82d0(unsigned long *a0,void *a1,long a2)
{
  char v1;
  long *v10;
  unsigned char *v11;
  long v12; // r14
  unsigned long v13;
  long v14;
  char *v2;
  unsigned char v3;
  unsigned char *v4;
  unsigned char *v5;
  unsigned int v6; // ecx
  unsigned char *v7;
  unsigned char *v8; // stack - 0x48
  unsigned char *v9; // stack - 0x50
  
  v7 = (unsigned char *)*a0;
  v13 = a0[1];
  v5 = &v7[v13];
  if ((a2) || (!dat_1d400)) {
    v5[-1] = dat_1d038;
    if (v13 == fwrite_unlocked(v7,1,v13,a1)) {
      v5[-1] = 0;
      return;
    }
    sub_7480(dcgettext(NULL,"write failed",5),a2); // no-return
  }
  if (v7 < v5) {
    do {
      v3 = *v7;
      v7 = &v7[1];
      if (v3 != 9) { // branch-flip
        v6 = (unsigned int)(char)v3;
        if (v5 == v7) {
          v6 = 10;
          v3 = 10;
        }
        v4 = *(unsigned char **)((long)a1 + 0x28);
        if (*(unsigned char **)((long)a1 + 0x30) <= v4) goto label_83be;
label_838c:
        *(unsigned char **)((long)a1 + 0x28) = &v4[1];
        *v4 = v3;
      }
      else {
        v4 = *(unsigned char **)((long)a1 + 0x28);
        v6 = 0x3e;
        v3 = 0x3e;
        if (v4 < *(unsigned char **)((long)a1 + 0x30)) goto label_838c;
label_83be:
        if (__overflow(a1,v6 & 0xff) == -1)
          sub_7480(dcgettext(NULL,"write failed",5),0); // no-return
      }
    } while (v5 != v7);
    v7 = (unsigned char *)*a0;
    v13 = a0[1];
  }
  v5 = &v7[v13 - 1];
  v10 = dat_1d410;
  v4 = v7;
  if (!dat_1d410) goto label_8642;
  do {
    v14 = *v10;
    if (v14 != -1) { // branch-flip
      v4 = (unsigned char *)sub_7dd0(v7,v13,v10);
      if (v10[2] != -1) {
label_844e:
        v5 = (unsigned char *)sub_7ca0(v7,v13,v10);
        goto label_845f;
      }
label_846b:
      if ((*(char *)((long)v10 + 0x36)) || (v10[6] & 0xffff00ff0000U)) goto label_8485;
    }
    else {
      v4 = v7;
      if (v10[2] != -1) goto label_844e;
label_845f:
      if ((v14 != -1) || (!(char)v10[6])) goto label_846b;
label_8485:
      v3 = *v5;
      *v5 = 0;
      v1 = *(char *)((unsigned long)*v4 + 0x1d740);
      while (v1) {
        v11 = &v4[1];
        v4 = &v4[1];
        v1 = *(char *)((unsigned long)*v11 + 0x1d740);
      }
      if (v4 <= v5) { // branch-flip
        v9 = v4;
        if (*(char *)((long)v10 + 0x36)) // branch-flip
          sub_7030(v4,&v9);
        else if (*(char *)((long)v10 + 0x34)) // branch-flip
          strtold(v4,&v9);
        else {
          if (!(v10[6] & 0xff0000ff0000U)) goto label_8670;
          v8 = v4;
          if (v4 < v5)
            v8 = &v4[*v4 == 0x2d];
          if ('0' <= (char)sub_6f70(&v8)) {
            v9 = v8;
            if (*(char *)((long)v10 + 0x35))
              v9 = &v8[*(char *)((unsigned long)*v8 + 0x15360) != '\0'];
          }
        }
      }
      else {
label_8670:
        v9 = v5;
      }
      *v5 = v3;
      v5 = v9;
    }
    while( true ) {
      while( true ) {
        v14 = (long)(int)sub_e260(v7,(long)v4 - (long)v7,0);
        if (v7 < v4) {
          do {
            v11 = &v7[1];
            v14 += (unsigned long)(*v7 == 9);
            v7 = v11;
          } while (v4 != v11);
        }
        v12 = (long)(int)sub_e260(v4,(long)v5 - (long)v4,0);
        if (v4 < v5) {
          do {
            v7 = &v4[1];
            v12 += (unsigned long)(*v4 == 9);
            v4 = v7;
          } while (v5 != v7);
        }
        while (v14) {
          v14 -= 1;
          v2 = *(char **)((long)stdout + 0x28);
          if (*(char **)((long)stdout + 0x30) <= v2) // branch-flip
            __overflow(stdout,0x20);
          else {
            *(char **)((long)stdout + 0x28) = &v2[1];
            *v2 = 0x20;
          }
        }
        if (v12) { // branch-flip
          do {
            v2 = *(char **)((long)stdout + 0x28);
            if (*(char **)((long)stdout + 0x30) <= v2) // branch-flip
              __overflow(stdout,0x5f);
            else {
              *(char **)((long)stdout + 0x28) = &v2[1];
              *v2 = 0x5f;
            }
            v12 -= 1;
          } while (v12);
          v2 = *(char **)((long)stdout + 0x28);
          if (*(char **)((long)stdout + 0x30) <= v2) // branch-flip
            __overflow(stdout,10);
          else {
            *(char **)((long)stdout + 0x28) = &v2[1];
            *v2 = 10;
          }
        }
        else {
          __printf_chk(1,dcgettext(NULL,"^ no match for key\n",5));
        }
        if (!v10)
          return;
        v10 = (long *)v10[8];
        if (v10) break;
        if (dat_1d419)
          return;
        if (dat_1d41a)
          return;
        v4 = (unsigned char *)*a0;
        v5 = &v4[a0[1] + -1];
        v7 = v4;
      }
      v7 = (unsigned char *)*a0;
      v13 = a0[1];
      v5 = &v7[v13 - 1];
      v4 = v7;
      if (v10) break;
label_8642:
      v7 = v4;
    }
  } while( true );
}

// Function: sub_8770 @ 0x8770
int sub_8770(int *a0,long a1)
{
  unsigned long v1;
  int v2; // eax
  int v3; // eax
  int *v4; // rax
  int v5;
  char v6 [128];
  double v7; // stack - 0xe0
  
  if (0 <= (int)sub_e990(a0,0x80000)) { // branch-flip
    if (dat_1d024 + 1U < dat_1d35c) {
      sub_78c0(0xffffffff);
      do {
        if ((int)dat_1d35c <= 0) break;
      } while (sub_78c0(0));
    }
    v4 = __errno_location();
    v7 = dat_170e8;
    do {
      a1 -= 1;
      v2 = pthread_sigmask(0,0x1d380,v6);
      v1 = dat_1d368;
      dat_1d368 = 0;
      v3 = fork();
      v5 = *v4;
      if (v3) { // branch-flip
        dat_1d368 = v1;
        if (!v2) goto label_8895;
      }
      else {
        if (v2) {
          close(0); // return-dupe
          close(1);
          return 0;
        }
label_8895:
        pthread_sigmask(2,v6,0);
      }
      *v4 = v5;
      if (0 <= v3) {
        if (v3) {
          dat_1d35c += 1;
          return v3;
        }
        close(0);
        close(1);
        return 0;
      }
      if (v5 != 0xb) goto label_8943;
      sub_13180(v7);
      v7 = v7 + v7;
      do {
        if ((int)dat_1d35c <= 0) break;
      } while (sub_78c0(0));
    } while (a1);
    v5 = *v4;
label_8943:
    close(*a0);
    close(a0[1]);
    *v4 = v5;
  }
  else {
    v3 = -1;
  }
  return v3;
}

// Function: sub_8980 @ 0x8980
long * sub_8980(unsigned long *a0,long a1,unsigned long *a2)
{
  unsigned int v1;
  long *v10;
  long v11; // r12
  int v12; // stack - 0x54
  unsigned int v13; // stack - 0x50
  int v2;
  long v3;
  long *v4;
  int *v5;
  unsigned long v6;
  int v7; // eax
  long *v8; // rax
  int v9; // stack - 0x58
  
  v8 = (long *)sub_12bf0(a1,8);
  v4 = v8;
  *a2 = v8;
  if (a1) { // branch-flip
    v11 = 0;
    v10 = v8;
    do {
      v3 = a0[1];
      if ((v3) && (*(char *)(v3 + 0xc))) {
        if (*(char *)(v3 + 0xc) == '\x01') {
          v1 = *(unsigned int *)(v3 + 8);
          v13 = v1;
          v8 = (long)sub_dd00(dat_1d360,&v9);
          if (v8) {
            *(char *)((long)v8 + 0xc) = 2;
            sub_78c0(v1);
          }
        }
        v7 = open((char *)(v3 + 0xd),0);
        v2 = v7;
        if (v7 < 0) {
          v4[v11] = 0;
          v8 = (long *)v11;
          return v8;
        }
        v7 = sub_8770(&v9,9);
        if (v7 == -1) {
          v8 = (int *)__errno_location();
          v5 = (int *)v8;
          if (*(int *)v8 == 0x18) {
            close(v2);
            *(int *)v8 = 0x18;
            v4[v11] = 0;
            v8 = (long *)v11;
            return v8;
          }
          v8 = (unsigned long)sub_108e0(4,dat_1d408);
          v6 = v8;
          v8 = (char *)dcgettext(NULL,"couldn\'t create process for %s -d",5);
          error(2,*v5,(char *)v8,v6);
          return v8;
        }
        if (!v7) {
          close(v9);
          if (v2)
            sub_7c90(v2,0);
          if (v12 != 1)
            sub_7c90(v12,1);
          execlp(dat_1d408,dat_1d408,"-d",0);
          v8 = (int *)__errno_location();
          sub_79f0(*(int *)v8,"couldn\'t execute compress program (with -d)"); // no-return
        }
        *(int *)(v3 + 8) = v7;
        sub_7a90(v3);
        close(v2);
        close(v12);
        v8 = fdopen(v9,"r");
        if (!v8) {
          v8 = (int *)__errno_location();
          v2 = *(int *)v8;
          close(v9);
          *v10 = 0;
          *(int *)v8 = v2;
          v8 = (long *)v11;
          return v8;
        }
        *v10 = (long)v8;
      }
      else {
        v8 = (long)sub_74e0(*a0,"r");
        *v10 = (long)v8;
        if (!v8) {
          v8 = (long *)v11;
          return v8;
        }
      }
      v11 += 1;
      a0 = &a0[2];
      v10 = &v10[1];
    } while (a1 != v11);
  }
  else {
    v11 = 0;
  }
  v8 = (long *)v11;
  return v8;
}

// Function: sub_8c20 @ 0x8c20
int * sub_8c20(unsigned long *a0,char a1)
{
  void *v1;
  char v10 [128];
  int v11; // stack - 0xc8
  int v12;
  unsigned long *v13;
  int v14; // stack - 0xc4
  int v2;
  char *v3;
  unsigned long v4;
  unsigned long *v5;
  int *v6;
  unsigned long v7;
  int v8; // eax
  int *v9; // rax
  
  v3 = *(char **)(dat_1d430 + dat_1d248 * 8);
  v9 = (unsigned long)strlen(v3);
  v4 = (unsigned long)v9;
  v9 = (unsigned long *)sub_12ab0((long)v9 + 0x20U & 0xfffffffffffffff8);
  v5 = (unsigned long *)v9;
  v1 = (void *)((long)v9 + 0xd);
  memcpy(v1,v3,v4);
  *(unsigned long *)((long)v9 + v4 + 0xd) = s_152f0._0_8_;
  *(unsigned int *)((long)v9 + v4 + 0x15) = s_152f0._8_4_;
  *(unsigned long *)v9 = 0;
  v9 = (long)(dat_1d248 + 1);
  dat_1d248 = (long)v9;
  if (v9 == (int *)dat_1d428)
    dat_1d248 = 0;
  v8 = pthread_sigmask(0,0x1d380,v10);
  v11 = CONCAT31(v11._1_3_,v8 == 0);
  v8 = sub_11d70(v1,0x80000);
  v12 = v8;
  v9 = __errno_location();
  v6 = v9;
  if (0 <= v12) { // branch-flip
    *dat_1d040 = v5;
    dat_1d040 = v5;
    if ((char)v11) {
      v2 = *v9;
      pthread_sigmask(2,v10,0);
      *v9 = v2;
    }
    *(char *)((long)v5 + 0xc) = 0;
    if (dat_1d408) {
      v8 = sub_8770(&v11,4);
      *(int *)&v5[1] = v8;
      if (1 <= v8) { // branch-flip
        close(v12);
        close(v11);
        sub_7a90(v5);
        v12 = v14;
      }
      else if (!v8) {
        close(v14);
        if (v12 != 1)
          sub_7c90(v12,1);
        if (v11)
          sub_7c90(v11,0);
        execlp(dat_1d408,dat_1d408,0);
        sub_79f0(*v6,"couldn\'t execute compress program"); // no-return
      }
    }
    v9 = fdopen(v12,"w");
    *a0 = v9;
    v13 = v5;
    if (!v9) {
      v9 = (char *)dcgettext(NULL,"couldn\'t create temporary file",5);
      sub_7480(v9,v1); // no-return
    }
  }
  else {
    v12 = *v9;
    if ((char)v11) {
      pthread_sigmask(2,v10,0);
      *v9 = v12;
    }
    if ((v12 != 0x18) || (a1 != '\x01')) {
      v9 = (unsigned long)sub_108e0(4,v3);
      v7 = v9;
      v9 = (char *)dcgettext(NULL,"cannot create temporary file in %s",5);
      error(2,*v6,(char *)v9,v7);
      return v9;
    }
    v13 = NULL;
    free(v5);
  }
  v9 = (int *)v13;
  return v9;
}

// Function: sub_8ed0 @ 0x8ed0
void sub_8ed0(void)
{
  unsigned long *v1;
  int v2; // eax
  char v3 [128];
  
  if (dat_1d368) {
    v2 = pthread_sigmask(0,0x1d380,v3);
    for (v1 = dat_1d368; v1; v1 = (unsigned long *)*v1) {
      unlink((char *)((long)v1 + 0xd));
    }
    dat_1d368 = NULL;
    if (!v2)
      pthread_sigmask(2,v3,0);
  }
  sub_c4c0(); // tail-call
}

// Function: sub_8f90 @ 0x8f90
void sub_8f90(char *a0)
{
  int v1;
  char v10 [8]; // stack - 0xc8
  long v2;
  long *v3;
  long *v4;
  int v5; // eax
  int *v6; // rax
  unsigned long v7; // rax
  long v8; // rax
  unsigned int v9 [32]; // stack - 0xc0
  
  v3 = (long *)0x1d368;
  for (v4 = dat_1d368; a0 != (char *)((long)v4 + 0xd); v4 = (long *)*v4) {
    v3 = v4;
  }
  if (*(char *)((long)v4 + 0xc) == '\x01') {
    v2 = v4[1];
    v9[0] = (int)v2;
    v8 = sub_dd00(dat_1d360,v10);
    if (v8) {
      *(char *)(v8 + 0xc) = 2;
      sub_78c0((int)v2);
    }
  }
  v2 = *v4;
  v10[0] = pthread_sigmask(0,0x1d380,v9) == 0;
  v5 = unlink(a0);
  v6 = __errno_location();
  v1 = *v6;
  *v3 = v2;
  if (v10[0])
    pthread_sigmask(2,v9,0);
  if (v5) {
    v7 = sub_10c60(0,3,a0);
    error(0,v1,dcgettext(NULL,"warning: cannot remove: %s",5),v7);
  }
  if (!v2)
    dat_1d040 = v3;
  free(v4);
}

// Function: sub_9120 @ 0x9120
int sub_9120(unsigned long *a0,long *a1)
{
  unsigned char *v1;
  unsigned char *v10;
  unsigned char *v11;
  unsigned long v12;
  unsigned long v13;
  unsigned char v14 [4008];
  long double v15; // stack - 0x2078
  long v16;
  unsigned char v17 [3999];
  unsigned char *v18; // stack - 0x2098
  unsigned char *v19; // stack - 0x2090
  char v2;
  unsigned long v20; // stack - 0x2048
  unsigned long v21; // stack - 0x1fec
  char v22 [16];
  char v23 [16];
  long double v24; // stack - 0x2088
  long *v25;
  unsigned char *v26;
  unsigned char *v27;
  unsigned char *v28;
  unsigned long v29; // r9
  unsigned char v3;
  unsigned long v30; // r10
  unsigned long v31;
  unsigned char *v32;
  long double v33;
  long double v34;
  long double v35;
  long double v36;
  long double v37;
  long double v38;
  long double v39;
  long v4;
  long double v40;
  long double v41;
  long double v42;
  long double v43;
  long double v44;
  unsigned char v45; // stack - 0x2132
  unsigned char v46; // stack - 0x2131
  unsigned long v47; // stack - 0x2118
  unsigned char *v48; // stack - 0x2110
  unsigned char *v49; // stack - 0x2100
  long v5;
  unsigned char *v50; // stack - 0x20f8
  unsigned long v51; // stack - 0x20f0
  unsigned long v52; // stack - 0x2040
  unsigned long v53; // stack - 0x2038
  unsigned long v54; // stack - 0x2030
  unsigned long v55; // stack - 0x2028
  unsigned long v56; // stack - 0x2020
  unsigned long v57; // stack - 0x2018
  unsigned long v58; // stack - 0x2010
  unsigned long v59; // stack - 0x2008
  unsigned char v6;
  unsigned long v60; // stack - 0x2000
  unsigned long v61; // stack - 0x1ff8
  unsigned int v62; // stack - 0x1ff0
  unsigned long v63; // stack - 0x1fe4
  unsigned long v64; // stack - 0x1fdc
  unsigned long v65; // stack - 0x1fd4
  unsigned long v66; // stack - 0x1fcc
  unsigned long v67; // stack - 0x1fc4
  unsigned long v68; // stack - 0x1fbc
  unsigned long v69; // stack - 0x1fb4
  int v7;
  unsigned long v70; // stack - 0x1fac
  unsigned long v71; // stack - 0x1fa4
  unsigned long v72; // stack - 0x1f9c
  unsigned int v73; // stack - 0x1f94
  int v8;
  unsigned char *v9;
  
  v26 = (unsigned char *)a0[2];
  v10 = (unsigned char *)a1[2];
  v9 = (unsigned char *)a0[3];
  v27 = (unsigned char *)a1[3];
  v25 = dat_1d410;
label_91a0:
  v4 = v25[5];
  v5 = v25[4];
  if (v9 <= v26)
    v9 = v26;
  if (v27 <= v10)
    v27 = v10;
  v29 = (long)v9 - (long)v26;
  v30 = (long)v27 - (long)v10;
  if (((dat_1d861) || (v25[6] & 0xffffffffff0000U)) || ((char)v25[7])) {
    if (v4 || v5) { // branch-flip
      v12 = v29 + 2 + v30;
      v9 = &v16;
      v48 = NULL;
      if (0xfa1 <= v12) { // branch-flip
        v9 = (unsigned char *)sub_12ab0(v12);
        v48 = v9;
        v27 = &v9[v29 + 1];
        if (!v29) goto label_9887;
label_95c1:
        v32 = &v26[v29];
        v29 = 0;
        do {
          v6 = *v26;
          if ((!v5) || (!*(char *)(v5 + (unsigned long)v6))) {
            if (v4)
              v6 = *(unsigned char *)(v4 + (unsigned long)v6);
            v9[v29] = v6;
            v29 += 1;
          }
          v26 = &v26[1];
        } while (v32 != v26);
        v9[v29] = 0;
        v26 = v27;
        if (v30) goto label_960d;
label_9896:
        v10 = v27;
      }
      else {
        v27 = &v17[v29];
        if (v29) goto label_95c1;
label_9887:
        *v9 = 0;
        v26 = v27;
        if (!v30) goto label_9896;
label_960d:
        v27 = &v10[v30];
        v30 = 0;
        do {
          v6 = *v10;
          if ((!v5) || (!*(char *)(v5 + (unsigned long)v6))) {
            if (v4)
              v6 = *(unsigned char *)(v4 + (unsigned long)v6);
            v26[v30] = v6;
            v30 += 1;
          }
          v10 = &v10[1];
        } while (v27 != v10);
        v27 = &v26[v30];
        v10 = v26;
      }
      *v27 = 0;
    }
    else {
      v45 = *v9;
      *v9 = 0;
      v46 = *v27;
      *v27 = 0;
      v9 = v26;
    }
    if (*(char *)((long)v25 + 0x32)) { // branch-flip
      v2 = *(char *)((unsigned long)*v9 + 0x1d740);
      v27 = v9;
      while (v2) {
        v26 = &v27[1];
        v27 = &v27[1];
        v2 = *(char *)((unsigned long)*v26 + 0x1d740);
      }
      v2 = *(char *)((unsigned long)*v10 + 0x1d740);
      v26 = v10;
      while (v2) {
        v32 = &v26[1];
        v26 = &v26[1];
        v2 = *(char *)((unsigned long)*v32 + 0x1d740);
      }
      v7 = sub_11d90(v27,v26,(int)dat_1d868,dat_1d864);
      v34 = v33;
      v35 = v36;
label_92a0:
      v33 = v34;
      v36 = v35;
      if (!v4 && !v5) {
label_92ac:
        v9[v29] = v45;
        v10[v30] = v46;
        goto label_92c1;
      }
    }
    else {
      if (*(char *)((long)v25 + 0x34)) {
        v34 = v37;
        v35 = v38;
        v37 = v39;
        v38 = v40;
        v39 = v42;
        v40 = v44;
        strtold(v9,&v18);
        strtold(v10,&v19);
        v42 = v40;
        v44 = v40;
        if (v9 != v18) { // branch-flip
          if (v10 != v19) { // branch-flip
            if (v33 < v36) {
label_a045:
              v7 = -1;
            }
            else {
              v7 = 1;
              if ((v36 >= v33) && (v7 = 0, v33 != v36)) {
                if (!NAN(v36)) goto label_a045;
                if (!NAN(v33)) goto label_9748;
                v43 = v40;
                v41 = v40;
                strtold(v9,0);
                v24 = v34;
                v34 = v37;
                v37 = v39;
                v39 = v43;
                v42 = v41;
                strtold(v10,0);
                v15 = v35;
                v44 = v42;
                v7 = memcmp(&v24,&v15,0x10);
                v35 = v38;
                v38 = v40;
                v40 = v41;
              }
            }
          }
          else {
label_9748:
            v7 = 1;
          }
        }
        else {
          v7 = -(unsigned int)(v19 != v10);
        }
        goto label_92a0;
      }
      if (!*(char *)((long)v25 + 0x35)) {
        if (*(char *)((long)v25 + 0x36)) { // branch-flip
          v7 = sub_7030(v9,0);
          v7 -= sub_7030(v10,0);
          v34 = v33;
          v35 = v36;
        }
        else if (*(char *)((long)v25 + 0x33)) { // branch-flip
          v21 = dat_1d300;
          v63 = dat_1d308;
          v64 = dat_1d310;
          v65 = dat_1d318;
          v72 = dat_1d350;
          v73 = dat_1d358;
          v61 = dat_1d350;
          v62 = dat_1d358;
          v66 = dat_1d320;
          v67 = dat_1d328;
          v68 = dat_1d330;
          v69 = dat_1d338;
          v70 = dat_1d340;
          v71 = dat_1d348;
          v20 = dat_1d300;
          v52 = dat_1d308;
          v53 = dat_1d310;
          v54 = dat_1d318;
          v55 = dat_1d320;
          v56 = dat_1d328;
          v57 = dat_1d330;
          v58 = dat_1d338;
          v59 = dat_1d340;
          v60 = dat_1d348;
          if (dat_1d861) {
            v27 = &v9[v29];
            v49 = NULL;
            v1 = &v10[v30];
            v12 = (v29 + v30) * 3 + 2;
            v47 = 4000;
            v7 = 0;
            v26 = v10;
            v11 = v14;
            v32 = v9;
label_9a20:
            v28 = v11;
            if (v47 < v12) goto label_9b04;
label_9a30:
            v11 = v28;
            if (v32 < v27) goto label_9b4f;
label_9a3b:
            v11 = v28;
            if (v26 < v1) {
              v13 = 0;
              v31 = v47;
              goto label_9b7e;
            }
            v51 = 0;
            v13 = 0;
label_9a51:
            v50 = &v11[v13];
            do {
              if (v27 <= v32) { // branch-flip
                if (v1 <= v26) {
label_9c1a:
                  MD5_Update(&v20,v11,v13);
                  MD5_Final(v22,&v20);
                  MD5_Update(&v21,v50,v51);
                  MD5_Final(v23,&v21);
                  v8 = memcmp(v22,v23,0x10);
                  if (!v8) {
                    if (v7) goto label_9cac;
                    goto label_9d87;
                  }
                  v7 = v8;
                  goto label_9cac;
                }
label_9a70:
                v26 = &v26[strlen((char *)v26) + 1];
              }
              else {
                v32 = &v32[strlen((char *)v32) + 1];
                if (v26 < v1) goto label_9a70;
              }
              while( true ) {
                if ((v27 <= v32) && (v1 <= v26)) goto label_9c1a;
                MD5_Update(&v20,v11,v13);
                MD5_Update(&v21,v50,v51);
                if (v7) goto label_9a20;
                v31 = v51;
                if (v13 <= v51)
                  v31 = v13;
                v7 = memcmp(v11,v50,v31);
                if (v7) goto label_9a20;
                v7 = (unsigned int)(v13 >= v51 && v13 != v51) - (unsigned int)(v13 < v51);
                v28 = v11;
                if (v12 <= v47) goto label_9a30;
label_9b04:
                v47 = v47 * 3 >> 1;
                if (v47 <= v12)
                  v47 = v12;
                free(v49);
                v11 = malloc(v47);
                v49 = v11;
                if (!v11) {
                  v47 = 4000;
                  v28 = v14;
                  goto label_9a30;
                }
                v28 = v49;
                if (v27 <= v32) goto label_9a3b;
label_9b4f:
                v13 = sub_7800(v11,v32,v47) + 1;
                if (v1 <= v26) { // branch-flip
                  v51 = 0;
                  v31 = v13;
                }
                else {
                  if (v13 <= v47) { // branch-flip
                    v28 = &v11[v13];
                    v31 = v47 - v13;
                  }
                  else {
                    v28 = NULL;
                    v31 = 0;
                  }
label_9b7e:
                  v51 = sub_7800(v28,v26,v31) + 1;
                  v31 = v51 + v13;
                }
                if ((v13 <= v47) && (v31 <= v47)) goto label_9a51;
                if (v31 <= 0x5555555555555554)
                  v31 = v31 * 3 >> 1;
                free(v49);
                v11 = (unsigned char *)sub_12ab0(v31);
                v47 = v31;
                if (v27 <= v32) break;
                strxfrm(v11,v32,v13);
                if (v26 < v1) goto label_9e3c;
                v50 = &v11[v13];
                v49 = v11;
                v32 = &v32[strlen((char *)v32) + 1];
              }
              if (v1 <= v26) {
                v50 = &v11[v13];
                v49 = v11;
                goto label_9c1a;
              }
label_9e3c:
              v50 = &v11[v13];
              strxfrm(v50,v26,v51);
              v49 = v11;
            } while( true );
          }
          MD5_Update(&v20,v9,v29);
          MD5_Final(v22,&v20);
          MD5_Update(&v21,v10,v30);
          MD5_Final(v23,&v21);
          v7 = memcmp(v22,v23,0x10);
          if (v7) // branch-flip
            v49 = NULL;
          else {
            v49 = NULL;
            v13 = v29;
            v11 = v9;
            v50 = v10;
            v51 = v30;
label_9d87:
            v12 = v51;
            if (v13 <= v51)
              v12 = v13;
            v7 = memcmp(v11,v50,v12);
            if (!v7)
              v7 = (unsigned int)(v51 < v13) - (unsigned int)(v13 < v51);
          }
label_9cac:
          free(v49);
          v34 = v33;
          v35 = v36;
        }
        else if ((char)v25[7]) { // branch-flip
          v7 = sub_ca00(v9,v29,v10,v30);
          v34 = v33;
          v35 = v36;
        }
        else {
          v34 = v33;
          v35 = v36;
          if (v29) { // branch-flip
            v7 = 1;
            if (v30) {
              v7 = sub_13120(v9,v29 + 1,v10,v30 + 1);
              v34 = v33;
              v35 = v36;
            }
          }
          else {
            v7 = -(unsigned int)(v30 != 0);
          }
        }
        goto label_92a0;
      }
      v6 = *v9;
      v2 = *(char *)((unsigned long)v6 + 0x1d740);
      v27 = v9;
      while (v2) {
        v6 = v27[1];
        v27 = &v27[1];
        v2 = *(char *)((unsigned long)v6 + 0x1d740);
      }
      v3 = *v10;
      v2 = *(char *)((unsigned long)v3 + 0x1d740);
      v26 = v10;
      while (v2) {
        v3 = v26[1];
        v26 = &v26[1];
        v2 = *(char *)((unsigned long)v3 + 0x1d740);
      }
      v15._0_8_ = &v27[v6 == 0x2d];
      v8 = 0;
      if ('1' <= (char)sub_6f70(&v15)) {
        v8 = (int)*(char *)((unsigned long)*v15._0_8_ + 0x15360);
        if (v6 == 0x2d)
          v8 = -(int)*(char *)((unsigned long)*v15._0_8_ + 0x15360);
      }
      v15 = (long double)CONCAT28(v15._8_2_,&v26[v3 == 0x2d]);
      v7 = v8;
      if ('1' <= (char)sub_6f70(&v15)) {
        v7 = v8 - *(char *)((unsigned long)*v15._0_8_ + 0x15360);
        if (v3 == 0x2d)
          v7 = *(char *)((unsigned long)*v15._0_8_ + 0x15360) + v8;
      }
      v34 = v33;
      v35 = v36;
      if (v7) goto label_92a0;
      v7 = sub_11d90(v27,v26,(int)dat_1d868,dat_1d864);
      if (!v4 && !v5) goto label_92ac;
    }
    free(v48);
    goto label_92c1;
  }
  if (v5) {
    if (!v4) {
      do {
        if ((v9 <= v26) || (!*(char *)(v5 + (unsigned long)*v26))) {
          if (v27 <= v10) goto label_a018;
          while (*(char *)(v5 + (unsigned long)*v10)) {
            v10 = &v10[1];
            if (v27 == v10) goto label_a018;
          }
          if ((v9 <= v26) || (v27 <= v10)) goto label_a018;
          v7 = (unsigned int)*v26 - (unsigned int)*v10;
          if (v7) goto label_94dd;
          v10 = &v10[1];
        }
        v26 = &v26[1];
      } while( true );
    }
    do {
      if ((v9 <= v26) || (!*(char *)(v5 + (unsigned long)*v26))) {
        if (v27 <= v10) goto label_9f28;
        while (*(char *)(v5 + (unsigned long)*v10)) {
          v10 = &v10[1];
          if (v27 == v10) goto label_9f28;
        }
        if ((v9 <= v26) || (v27 <= v10)) goto label_9f28;
        v7 = (unsigned int)*(unsigned char *)(v4 + (unsigned long)*v26) - (unsigned int)*(unsigned char *)(v4 + (unsigned long)*v10);
        if (v7) goto label_94dd;
        v10 = &v10[1];
      }
      v26 = &v26[1];
    } while( true );
  }
  if (!v29) {
    v7 = -(unsigned int)(v30 != 0);
    goto label_92c1;
  }
  if (!v30) {
label_9f18:
    v7 = 1;
    goto label_94dd;
  }
  if (v4) { // branch-flip
    while ((v26 < v9 && (v10 < v27))) {
      v6 = *v26;
      v32 = &v10[1];
      v26 = &v26[1];
      v7 = (unsigned int)*(unsigned char *)(v4 + (unsigned long)v6) - (unsigned int)*(unsigned char *)(v4 + (unsigned long)*v10);
      v10 = v32;
      if (v7) goto label_94dd;
    }
  }
  else {
    v12 = v30;
    if (v29 <= v30)
      v12 = v29;
    v7 = memcmp(v26,v10,v12);
    if (v7) goto label_94dd;
  }
  if (v29 < v30) {
    v7 = -1;
    goto label_94dd;
  }
  if (v29 != v30) goto label_9f18;
  goto label_92c7;
label_a018:
  v7 = (unsigned int)(v26 < v9) - (unsigned int)(v10 < v27);
  goto label_92c1;
label_9f28:
  v7 = (unsigned int)(v26 < v9) - (unsigned int)(v10 < v27);
label_92c1:
  if (v7) {
label_94dd:
    if (*(char *)((long)v25 + 0x37))
      return -v7;
    return v7;
  }
label_92c7:
  v25 = (long *)v25[8];
  if (!v25)
    return 0;
  v26 = (unsigned char *)*a0;
  v4 = a0[1];
  if (v25[2] != -1) { // branch-flip
    v9 = (unsigned char *)sub_7ca0(v26,v4,v25);
    v27 = (unsigned char *)sub_7ca0(*a1,a1[1],v25);
  }
  else {
    v9 = &v26[v4 + -1];
    v27 = (unsigned char *)(*a1 + -1 + a1[1]);
  }
  if (*v25 != -1) { // branch-flip
    v26 = (unsigned char *)sub_7dd0(v26,v4,v25);
    v10 = (unsigned char *)sub_7dd0(*a1,a1[1],v25);
  }
  else {
    v10 = (unsigned char *)*a1;
    if ((char)v25[6]) {
      if (v26 < v9) {
        do {
          if (!*(char *)((unsigned long)*v26 + 0x1d740)) break;
          v26 = &v26[1];
        } while (v9 != v26);
      }
      if (v10 < v27) {
        do {
          if (!*(char *)((unsigned long)*v10 + 0x1d740)) goto label_91a0;
          v10 = &v10[1];
        } while (v27 != v10);
      }
    }
  }
  goto label_91a0;
}

// Function: sub_a0b0 @ 0xa0b0
unsigned long sub_a0b0(unsigned long *a0,unsigned long *a1)
{
  long v1;
  long v2;
  int v3; // eax
  unsigned long v4;
  unsigned long v5;
  unsigned long v6; // r15
  
  if (dat_1d410) {
    v4 = sub_9120();
    if ((int)v4)
      return v4;
    if (dat_1d419)
      return v4;
    if (dat_1d41a)
      return v4;
  }
  v1 = a0[1];
  v2 = a1[1];
  v4 = v2 - 1;
  v6 = v1 - 1;
  if (v6) { // branch-flip
    v5 = 1;
    if (v4) {
      if (dat_1d861) // branch-flip
        v5 = sub_13120((void *)*a0,v1,(void *)*a1,v2);
      else {
        v5 = v4;
        if (v6 <= v4)
          v5 = v6;
        v3 = memcmp((void *)*a0,(void *)*a1,v5);
        v5 = CONCAT44(dat_4,v3);
        if ((!v3) && (v5 = 0xffffffff, v4 <= v6))
          v5 = (unsigned long)(v1 != v2);
      }
    }
  }
  else {
    v5 = (unsigned long)-(unsigned int)(v4 != 0);
  }
  if (dat_1d41b)
    v5 = (unsigned long)(unsigned int)-(int)v5;
  return v5;
}

// Function: sub_a180 @ 0xa180
void sub_a180(unsigned long *a0,unsigned long a1,unsigned long *a2,char a3) // return-dupe x2
{
  unsigned long v1;
  long v10; // rax
  unsigned long *v11;
  unsigned long *v12; // rbp
  unsigned long *v13;
  unsigned long v14; // r12
  unsigned long *v15;
  unsigned long *v16;
  long v17; // stack - 0x48
  unsigned long v2;
  unsigned long v3;
  unsigned long v4;
  unsigned long v5;
  unsigned long v6;
  unsigned long v7;
  unsigned long v8;
  int v9; // eax
  
  if (a1 != 2) {
    v14 = a1 >> 1;
    v17 = a1 - v14;
    v13 = &a0[v14 * -4];
    if (a3) { // branch-flip
      v16 = &a2[v14 * -4];
      sub_a180(v13,v17,v16,1);
      v11 = a2;
      if (4 <= a1)
        sub_a180(a0,v14,a2,0);
    }
    else {
      sub_a180(v13,v17,a2,0);
      v11 = a0; // branch-flip
      v16 = v13;
      if (4 <= a1) {
        sub_a180(a0,v14,a2,1);
        a0 = a2;
      }
      else {
        v1 = a0[-3];
        v2 = a0[-2];
        v3 = a0[-1];
        a2[-4] = a0[-4];
        a2[-3] = v1;
        a2[-2] = v2;
        a2[-1] = v3;
        a0 = a2;
      }
    }
    v13 = &v16[-4];
    v15 = &a0[-4];
    while( true ) {
      while( true ) {
        v12 = v13;
        v13 = &v11[-4];
        if (1 <= (int)sub_a0b0(v15,v12)) break;
        v1 = a0[-3];
        v2 = a0[-2];
        v3 = a0[-1];
        *v13 = a0[-4];
        v11[-3] = v1;
        v11[-2] = v2;
        v11[-1] = v3;
        v14 -= 1;
        if (!v14)
          return;
        v11 = v13;
        v13 = v12;
        a0 = v15;
        v15 = &v15[-4];
      }
      v1 = v16[-3];
      v17 -= 1;
      *v13 = v16[-4];
      v11[-3] = v1;
      v1 = v16[-1];
      v11[-2] = v16[-2];
      v11[-1] = v1;
      if (!v17) break;
      v11 = v13;
      v13 = &v12[-4];
      v16 = v12;
    }
    v10 = -0x20;
    do {
      v1 = ((unsigned long *)((long)a0 + v10))[1];
      v16 = (unsigned long *)((long)a0 + v10 + 0x10);
      v2 = *v16;
      v3 = v16[1];
      *(unsigned long *)((long)v13 + v10) = *(unsigned long *)((long)a0 + v10);
      ((unsigned long *)((long)v13 + v10))[1] = v1;
      v16 = (unsigned long *)((long)v11 + v10 + -0x10);
      *v16 = v2;
      v16[1] = v3;
      v10 -= 0x20;
      v14 -= 1;
    } while (v14);
    return;
  }
  v9 = sub_a0b0(&a0[-4],&a0[-8]);
  if (a3) {
    v13 = &a0[(long)(int)((unsigned int)(0 < v9) - 2) * 4];
    a0 = &a0[(long)(int)~(unsigned int)(0 < v9) * 4];
    v1 = a0[1];
    v2 = a0[2];
    v3 = a0[3];
    a2[-4] = *a0;
    a2[-3] = v1;
    v1 = *v13;
    v4 = v13[1];
    a2[-2] = v2;
    a2[-1] = v3;
    v2 = v13[2];
    v3 = v13[3];
    a2[-8] = v1;
    a2[-7] = v4;
    a2[-6] = v2;
    a2[-5] = v3;
    return;
  }
  if (v9 <= 0)
    return;
  v1 = a0[-4];
  v2 = a0[-3];
  v3 = a0[-2];
  v4 = a0[-1];
  v5 = a0[-8];
  v6 = a0[-7];
  v7 = a0[-6];
  v8 = a0[-5];
  a2[-4] = v1;
  a2[-3] = v2;
  a2[-2] = v3;
  a2[-1] = v4;
  a0[-4] = v5;
  a0[-3] = v6;
  a0[-2] = v7;
  a0[-1] = v8;
  a0[-8] = v1;
  a0[-7] = v2;
  a0[-6] = v3;
  a0[-5] = v4;
}

// Function: sub_a3c0 @ 0xa3c0
void sub_a3c0(long *a0,unsigned long a1,unsigned long a2)
{
  if (dat_1d419) {
    if (dat_1d840) {
      if (!sub_a0b0(a0,0x1d840))
        return;
    }
    dat_1d840 = *a0;
    dat_1d848 = a0[1];
    dat_1d850 = a0[2];
    dat_1d858 = a0[3];
  }
  sub_82d0(a0,a1,a2); // tail-call
}

// Function: sub_a430 @ 0xa430
void sub_a430(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,void *a5)
{
  unsigned long v1;
  unsigned long v10;
  unsigned long v11;
  unsigned long v12;
  unsigned long v13;
  unsigned long v14;
  long v15;
  int v16; // eax
  void *v17; // rax
  unsigned long *v18; // rax
  void *v19;
  long *v2;
  unsigned long *v20;
  long v21;
  unsigned long *v22;
  unsigned long *v23; // rdx
  unsigned long *v24;
  void *v25; // stack - 0x68
  unsigned long v26;
  unsigned long v27; // r14
  unsigned long v28;
  unsigned long v29;
  unsigned long *v3;
  bool v30; // zf
  void *v31; // stack - 0xd0
  unsigned long v32; // stack - 0xc8
  void *v33; // stack - 0xc0
  void *v34; // stack - 0xb8
  void **v35; // stack - 0xb0
  unsigned long v36; // stack - 0x88
  unsigned long v37; // stack - 0x80
  unsigned long v38; // stack - 0x60
  long v39; // stack - 0x58
  unsigned long v4;
  long v40; // stack - 0x50
  unsigned long v5;
  long v6;
  long v7;
  unsigned long v8;
  unsigned long v9;
  
  v34 = (void *)sub_12bf0(a2,0x38);
  v17 = (void *)sub_12bf0(a2,8);
  v33 = (void *)sub_12bf0(a2,8);
  v18 = (unsigned long *)sub_12bf0(a2,8);
  v15 = dat_1d410;
  v25 = NULL;
  v35 = (void **)v18;
  v31 = v17;
  if (a2) {
    v28 = 0;
    v32 = a2;
    v36 = a1;
    do {
      while( true ) {
        v26 = dat_1d438 / v32;
        if (dat_1d438 / v32 < dat_1d030)
          v26 = dat_1d030;
        v2 = (long *)((long)v34 + v28 * 0x38);
        while( true ) {
          v1 = (v26 & 0xffffffffffffffe0) + 0x20;
          v19 = malloc(v1);
          *v2 = (long)v19;
          if (v19) break;
          v26 = v1 >> 1;
          if (v1 <= 0x42)
            sub_12fb0(); // no-return
        }
        v2[3] = v1;
        v21 = v28 * 8;
        v2[5] = 0x20;
        v24 = (unsigned long *)(v28 * 0x10 + a0);
        v4 = *(unsigned long *)((long)a5 + v21);
        v2[2] = 0;
        v2[4] = 0;
        v5 = *v24;
        v2[1] = 0;
        *(char *)&v2[6] = 0;
        if (sub_7ef0(v2,v4,v5)) break;
        sub_7660(*(unsigned long *)((long)a5 + v21),*v24);
        if (v28 < v36) {
          v36 -= 1;
          sub_8f90(*v24);
        }
        free((void *)*v2);
        v26 = v32 - 1;
        if (v26 <= v28) {
          if (!v26) goto label_aa9e;
          v32 = v26;
          goto label_a5e2;
        }
        do {
          v24 = (unsigned long *)(a0 + 0x10 + v21 * 2);
          v5 = v24[1];
          v4 = *(unsigned long *)((long)a5 + v21 + 8);
          v3 = (unsigned long *)(a0 + v21 * 2);
          *v3 = *v24;
          v3[1] = v5;
          *(unsigned long *)((long)a5 + v21) = v4;
          v21 += 8;
        } while (v32 * 8 + -8 != v21);
        v32 = v26;
      }
      v21 = v2[3];
      v6 = *v2;
      *(long *)((long)v17 + v28 * 8) = v21 + v6 + -0x20;
      *(long *)((long)v33 + v28 * 8) = v21 + v6 + v2[2] * -0x20;
      v28 += 1;
    } while (v28 < v32);
label_a5e2:
    v28 = 0;
    do {
      v18[v28] = v28;
      v28 += 1;
    } while (v28 != v32);
    if (v32 != 1) {
      v28 = 1;
      do {
        while( true ) {
          v20 = &v18[v28];
          v22 = &v18[v28 - 1];
          v4 = *(unsigned long *)((long)v17 + *v20 * 8);
          v5 = *(unsigned long *)((long)v17 + *v22 * 8);
          if ((int)sub_a0b0(v5,v4) <= 0) break;
          v26 = *v22;
          v28 = 1;
          *v22 = *v20;
          *v20 = v26;
        }
        v28 += 1;
      } while (v28 < v32);
    }
    v37 = 0;
    v35 = NULL;
label_a680:
    v2 = *(long **)((long)v17 + *v18 * 8);
    if (dat_1d419) { // branch-flip
      if (v35) {
        if (!sub_a0b0(v35,v2)) goto label_a6b6;
        sub_82d0(&v25,a3,a4);
      }
      v28 = v2[1];
      v19 = v25;
      if (v37 < v28) {
        do {
          v26 = v28;
          if (!v37) goto label_ab1d;
          v37 *= 2;
        } while (v37 < v28);
        v26 = v37;
label_ab1d:
        free(v25);
        v19 = (void *)sub_12ab0(v26);
        v28 = v2[1];
        v25 = v19;
        v37 = v26;
      }
      v38 = v28;
      v19 = memcpy(v19,(void *)*v2,v28);
      v35 = &v25;
      if (v15) {
        v39 = (v2[2] - *v2) + (long)v19;
        v40 = (long)v19 + (v2[3] - *v2);
      }
    }
    else {
      sub_82d0(v2,a3,a4);
    }
label_a6b6:
    v28 = *v18;
    if (*(long **)((long)v33 + v28 * 8) < v2) {
      *(long **)((long)v17 + v28 * 8) = &v2[-4];
      v31 = (void *)v28;
      if (v32 != 1) goto label_a761;
label_a81c:
      *v18 = v28;
      goto label_a680;
    }
    v4 = *(unsigned long *)((long)a5 + v28 * 8);
    v5 = *(unsigned long *)(a0 + v28 * 0x10);
    if (sub_7ef0((void *)((long)v34 + v28 * 0x38),v4,v5)) {
      v28 = *v18;
      v2 = (long *)((long)v34 + v28 * 0x38);
      v21 = v2[3];
      v6 = *v2;
      v7 = v2[2];
      *(long *)((long)v17 + v28 * 8) = v21 + v6 + -0x20;
      *(long *)((long)v33 + v28 * 8) = v21 + v6 + v7 * -0x20;
      v31 = (void *)v28;
      if (v32 == 1) goto label_a81c;
label_a761:
      v26 = 1;
      v1 = 1;
      v29 = v32;
      do {
        while( true ) {
          v27 = v1;
          v16 = sub_a0b0(*(unsigned long *)((long)v17 + v28 * 8),*(unsigned long *)((long)v17 + v18[v27] * 8));
          if ((0 <= v16) && ((v16 || (v18[v27] <= v31)))) break;
          v1 = v26 + v27 >> 1;
          v29 = v27;
          if (v27 <= v26) goto label_a7c9;
        }
        v26 = v27 + 1;
        v1 = v26 + v29 >> 1;
      } while (v26 < v29);
label_a7c9:
      if (v26 - 1) {
        v22 = v18;
        do {
          v20 = &v22[1];
          *v22 = v22[1];
          v22 = v20;
        } while (v20 != &v18[v26 - 1]);
      }
      v18[v26 - 1] = (unsigned long)v31;
      goto label_a680;
    }
    v28 = *v18;
    v22 = &v18[1];
    if (v32 != 1) {
      do {
        if (v28 < *v22) {
          *v22 = *v22 - 1;
          v28 = *v18;
        }
        v22 = &v22[1];
      } while (&v18[v32] != v22);
    }
    v26 = v32 - 1;
    sub_7660(*(unsigned long *)((long)a5 + v28 * 8),*(unsigned long *)(a0 + v28 * 0x10));
    v28 = *v18;
    if (v28 < v36) {
      v36 -= 1;
      sub_8f90(*(unsigned long *)(a0 + v28 * 0x10));
      v28 = *v18;
    }
    free(*(void **)((long)v34 + v28 * 0x38));
    v28 = *v18;
    if (v28 < v26) {
      v21 = v28 * 8;
      v24 = (unsigned long *)((long)v34 + v28 * 0x38);
      do {
        v3 = (unsigned long *)(a0 + 0x10 + v21 * 2);
        v5 = *v3;
        v8 = v3[1];
        v23 = &v24[7];
        v9 = *v23;
        v10 = v24[8];
        v11 = v24[9];
        v12 = v24[10];
        *(unsigned long *)((long)a5 + v21) = *(unsigned long *)((long)a5 + v21 + 8);
        v4 = v24[0xd];
        v13 = v24[0xb];
        v14 = v24[0xc];
        v3 = (unsigned long *)(a0 + v21 * 2);
        *v3 = v5;
        v3[1] = v8;
        v24[6] = v4;
        v4 = *(unsigned long *)((long)v17 + v21 + 8);
        *v24 = v9;
        v24[1] = v10;
        *(unsigned long *)((long)v17 + v21) = v4;
        v4 = *(unsigned long *)((long)v33 + v21 + 8);
        v24[2] = v11;
        v24[3] = v12;
        *(unsigned long *)((long)v33 + v21) = v4;
        v21 += 8;
        v24[4] = v13;
        v24[5] = v14;
        v24 = v23;
      } while (v32 * 8 + -8 != v21);
    }
    if (v26) {
      v22 = v18;
      do {
        v20 = &v22[1];
        *v22 = v22[1];
        v22 = v20;
      } while (&v18[v32 - 1] != v20);
      v32 = v26;
      goto label_a680;
    }
    v30 = v35 != NULL;
    v35 = (void **)v18;
    v31 = v17;
    if (v30) {
      if (dat_1d419) {
        sub_82d0(&v25,a3,a4);
        free(v25);
      }
    }
  }
label_aa9e:
  sub_7660(a3,a4);
  free(a5);
  free(v34);
  free(v35);
  free(v33);
  free(v31); // tail-call
  return;
}

// Function: sub_abe0 @ 0xabe0
unsigned long sub_abe0(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1;
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x48
  
  v2 = sub_8980(a0,a2,&v3);
  if ((v2 < a2) && (v2 < 2)) {
    v1 = *(unsigned long *)(a0 + v2 * 0x10);
    sub_7480(dcgettext(NULL,"open failed",5),v1); // no-return
  }
  sub_a430(a0,a1,v2,a3,a4,v3);
  return v2;
}

// Function: sub_ac90 @ 0xac90
void sub_ac90(long *a0,unsigned long a1,unsigned long a2,char *a3)
{
  char *v1;
  long *v10;
  unsigned long v11; // stack - 0xe0
  long v12; // stack - 0xd8
  long v13; // stack - 0xe8
  unsigned long v14;
  unsigned long v15; // r12
  long *v16;
  unsigned long v17; // stack - 0x110
  unsigned long v18; // stack - 0x108
  long v19; // stack - 0xd0
  int v2; // eax
  int v3;
  long v4;
  unsigned long v5;
  unsigned long v6;
  unsigned long v7; // rdx
  unsigned long v8;
  long v9;
  
  v6 = (unsigned long)dat_1d024;
  v18 = a2;
  if (v6 < a2) {
    do {
      v5 = 0;
      v9 = 0;
      if (v6 <= v18) { // branch-flip
        do {
          v15 = v5;
          v4 = sub_8c20(&v11,0);
          v5 = (unsigned long)dat_1d024;
          v6 = a1;
          if (v5 <= a1)
            v6 = v5;
          v5 = sub_abe0(&a0[v9 * 2],v6,v5,v11,v4 + 0xd);
          v8 = (unsigned long)dat_1d024;
          v6 = a1;
          if (v5 <= a1)
            v6 = v5;
          v9 += v5;
          v5 = v15 + 1;
          a0[v15 * 2] = v4 + 0xd;
          a0[v15 * 2 + 1] = v4;
          a1 -= v6;
        } while (v8 <= v18 - v9);
        v7 = v5 % v8;
        v6 = v8 - v7;
        v14 = v18 - v9;
        v10 = &a0[v15 * 2 + 2];
        v16 = &a0[v9 * 2];
      }
      else {
        v7 = 0;
        v8 = v6;
        v14 = v18;
        v10 = a0;
        v16 = a0;
      }
      if (v6 < v14) {
        v8 = (v14 + 1 + v7) - v8;
        v4 = sub_8c20(&v11,0);
        v6 = a1;
        if (v8 <= a1)
          v6 = v8;
        v8 = sub_abe0(v16,v6,v8,v11,v4 + 0xd);
        v6 = a1;
        if (v8 <= a1)
          v6 = v8;
        v5 += 1;
        v9 += v8;
        *v10 = v4 + 0xd;
        v10[1] = v4;
        a1 -= v6;
        v16 = &a0[v9 * 2];
        v10 = &a0[v5 * 2];
      }
      a1 += v5;
      memmove(v10,v16,(v18 - v9) * 0x10);
      v6 = (unsigned long)dat_1d024;
      v18 += v5 - v9;
    } while (v6 < v18);
  }
  if (a1 < v18) {
    v9 = 0;
    v10 = &a0[a1 * 2];
    v6 = a1;
    do {
      while( true ) {
        v1 = (char *)*v10;
        v2 = strcmp(v1,"-");
        if (((!a3) || (strcmp(a3,v1))) || (!v2)) break;
label_ae9e:
        v4 = v9 + 0xd;
        if (!v9) {
          v9 = sub_8c20(&v11,0);
          v4 = v9 + 0xd;
          sub_abe0(v10,0,1,v11,v4);
        }
        *v10 = v4;
        v6 += 1;
        v16 = &v10[2];
        v10[1] = v9;
        v10 = v16;
        if (v6 == v18) goto label_aecf;
      }
      v3 = dat_1d2f0;
      if (dat_1d2f0) { // branch-flip
label_ae64:
        if (0 <= v3) break;
      }
      else {
        if (fstat(1,(void *)0x1d260)) {
          v3 = *__errno_location();
          dat_1d2f0 = v3;
          goto label_ae64;
        }
        dat_1d2f0 = -1;
      }
      if (v2) { // branch-flip
        v1 = (char *)*v10;
        if ((!stat(v1,&v12)) && (v19 == dat_1d268)) {
label_ae8c:
          if (v12 == dat_1d260) goto label_ae9e;
        }
      }
      else if ((!fstat(0,&v12)) && (v19 == dat_1d268)) goto label_ae8c;
      v6 += 1;
      v10 = &v10[2];
    } while (v6 != v18);
  }
label_aecf:
  v17 = a1;
  do {
    v6 = sub_8980(a0,v18,&v13);
    if (v18 != v6) { // branch-flip
      if (v6 <= 2) {
        v9 = a0[v6 * 2];
        sub_7480(dcgettext(NULL,"open failed",5),v9); // no-return
      }
    }
    else {
      v9 = sub_74e0(a3,"w");
      if (v9) {
        sub_a430(a0,v17,v18,v9,a3,v13);
        return;
      }
      if ((*__errno_location() != 0x18) || (v18 <= 2))
        sub_7480(dcgettext(NULL,"open failed",5),a3); // no-return
    }
    v6 -= 1;
    v10 = &a0[v6 * 2];
    while( true ) {
      sub_7660(*(unsigned long *)(v13 + v6 * 8),*v10);
      v16 = &v10[-2];
      v9 = sub_8c20(&v11,3 <= v6);
      if (v9) break;
      v6 -= 1;
      v10 = v16;
    }
    v5 = v17;
    if (v6 < v17)
      v5 = v6;
    v4 = v18 - v6;
    sub_a430(a0,v5,v6,v11,v9 + 0xd,v13);
    v18 = v4 + 1;
    *a0 = v9 + 0xd;
    a0[1] = v9;
    memmove(&a0[2],v10,v4 * 0x10);
    v17 = (v17 - v5) + 1;
  } while( true );
}

// Function: sub_b1a0 @ 0xb1a0
void sub_b1a0(unsigned long *a0,long *a1)
{
  long v1;
  
  if (*a1 != a1[2]) { // branch-flip
    if (a1[1] != a1[3]) goto label_b1c0;
    v1 = a1[6];
  }
  else {
    if (a1[1] == a1[3])
      return;
    v1 = a1[5];
  }
  if (v1)
    return;
label_b1c0:
  pthread_mutex_lock(&a0[1]);
  sub_df40(*a0,a1);
  *(char *)((long)a1 + 0x54) = 1;
  pthread_cond_signal(&a0[6]);
  pthread_mutex_unlock(&a0[1]); // tail-call
  return;
}

// Function: sub_b220 @ 0xb220
void sub_b220(long a0,unsigned long a1,unsigned long a2,long *a3,unsigned long *a4,unsigned long a5,unsigned long a6)
{
  unsigned long *v1;
  long *v10; // rax
  unsigned long *v11;
  long v12;
  unsigned long *v13;
  long v14;
  unsigned long *v15;
  unsigned long *v16; // rax
  long v17; // stack - 0x78
  unsigned long v18; // stack - 0x80
  unsigned long *v19;
  unsigned long *v2;
  long v20;
  unsigned long *v21;
  char v22;
  long v23;
  long v24;
  unsigned long *v25;
  unsigned long v26;
  unsigned long v27;
  unsigned long v28; // r15
  long v29;
  long *v3;
  unsigned long v30; // stack - 0x70
  unsigned long v31; // stack - 0x68
  long v32; // stack - 0x60
  unsigned long *v33; // stack - 0x58
  unsigned long v34; // stack - 0x50
  unsigned long v35; // stack - 0x48
  int v4;
  unsigned long *v5;
  unsigned long *v6;
  unsigned long v7;
  unsigned long v8;
  unsigned long v9;
  
  v9 = a6;
  v28 = a1 >> 1;
  v27 = a3[5];
  v26 = a3[6];
  v32 = a3[8];
  v35 = a6;
  v17 = a0;
  v30 = v28;
  v31 = a2;
  v33 = a4;
  v34 = a5;
  if ((0x20000 <= v27 + v26) && (2 <= a1)) {
    if (!pthread_create(&v18,0,sub_ba80,&v17)) {
      sub_b220(a0 + a3[5] * -0x20,a1 - v28,a2,a3[9],a4,a5,v9);
      pthread_join(v18,0);
      return;
    }
    v27 = a3[5];
    v26 = a3[6];
  }
  v23 = a0 + a2 * -0x20;
  v29 = a0 + v27 * -0x20;
  if (2 <= v26)
    sub_a180(v29,v26,v23 + (v27 >> 1) * -0x20,0);
  if (2 <= v27)
    sub_a180(a0,v27,v23,0);
  *a3 = a0;
  v1 = &a4[6];
  a3[1] = v29;
  a3[2] = v29;
  a3[3] = a0 + v27 * -0x20 + v26 * -0x20;
  v2 = &a4[1];
  pthread_mutex_lock(v2);
  sub_df40(*a4,a3);
  *(char *)((long)a3 + 0x54) = 1;
  pthread_cond_signal(v1);
  pthread_mutex_unlock(v2);
  do {
    pthread_mutex_lock(v2);
    while (v10 = (long *)sub_e010(*a4), !v10) {
      pthread_cond_wait(v1,v2);
    }
    pthread_mutex_unlock(v2);
    v3 = &v10[0xb];
    pthread_mutex_lock(v3);
    v4 = (int)v10[10];
    *(char *)((long)v10 + 0x54) = 0;
    if (!v4) {
      pthread_mutex_unlock(v3);
      pthread_mutex_lock(v2);
      sub_df40(*a4,v10);
      *(char *)((long)v10 + 0x54) = 1;
      pthread_cond_signal(v1);
      pthread_mutex_unlock(v2);
      return;
    }
    v5 = (unsigned long *)*v10;
    v6 = (unsigned long *)v10[1];
    v19 = (unsigned long *)v10[2];
    v23 = (a2 >> ((char)v4 * '\x02' + 2U & 0x3f)) + 1;
    v21 = v5;
    if (v4 != 1) { // branch-flip
      v13 = (unsigned long *)v10[4];
      v15 = (unsigned long *)*v13;
      v11 = v6;
      if (v5 != v19) { // branch-flip
        v12 = v23;
        v25 = v5;
        while ((unsigned long *)v10[3] != v11) {
          v23 = v12 + -1;
          if (!v12) {
            v21 = (unsigned long *)*v10;
            v22 = *(char *)((long)v10 + 0x54);
            v12 = -1;
            v13 = (unsigned long *)v10[4];
            goto label_b66d;
          }
          v16 = &v15[-4];
          if (1 <= (int)sub_a0b0(&v25[-4],&v11[-4])) { // branch-flip
            v29 = v10[1];
            v25 = (unsigned long *)*v10;
            v19 = (unsigned long *)v10[2];
            v7 = *(unsigned long *)(v29 + -0x20);
            v8 = *(unsigned long *)(v29 + -0x18);
            v11 = (unsigned long *)(v29 + -0x20);
            v10[1] = (long)v11;
            *v16 = v7;
            v15[-3] = v8;
            v7 = *(unsigned long *)(v29 + -8);
            v15[-2] = *(unsigned long *)(v29 + -0x10);
            v15[-1] = v7;
            if (v19 == v25) goto label_b4b8;
          }
          else {
            v29 = *v10;
            v19 = (unsigned long *)v10[2];
            v7 = *(unsigned long *)(v29 + -0x20);
            v8 = *(unsigned long *)(v29 + -0x18);
            v25 = (unsigned long *)(v29 + -0x20);
            *v10 = (long)v25;
            *v16 = v7;
            v15[-3] = v8;
            v7 = *(unsigned long *)(v29 + -8);
            v11 = (unsigned long *)v10[1];
            v15[-2] = *(unsigned long *)(v29 + -0x10);
            v15[-1] = v7;
            if (v19 == v25) {
label_b4b8:
              v22 = *(char *)((long)v10 + 0x54);
              v13 = (unsigned long *)v10[4];
              v24 = (long)v5 - (long)v25 >> 5;
              v29 = (long)v6 - (long)v11 >> 5;
              v21 = v25;
              v15 = v16;
              goto label_b4e8;
            }
          }
          v12 = v23;
          v15 = v16;
        }
        v21 = (unsigned long *)*v10;
        v22 = *(char *)((long)v10 + 0x54);
        v13 = (unsigned long *)v10[4];
label_b66d:
        v29 = (long)v6 - (long)v11 >> 5;
        v24 = (long)v5 - (long)v25 >> 5;
        v14 = v10[6];
        v20 = v10[5];
        if (v14 != v29) { // branch-flip
label_b4f5:
          if (((v24 == v20) && (v19 = (unsigned long *)v10[3], v19 != v11)) && (v23 = v12 + -1, v25 = v15, v12)) {
            do {
              v16 = &v11[-4];
              v7 = *v16;
              v8 = v11[-3];
              v15 = &v25[-4];
              v10[1] = (long)v16;
              *v15 = v7;
              v25[-3] = v8;
              v7 = v11[-1];
              v25[-2] = v11[-2];
              v25[-1] = v7;
              if (v16 == v19) {
                v29 = (long)v6 - (long)v16 >> 5;
                goto label_b4fe;
              }
              v23 -= 1;
              v25 = v15;
              v11 = v16;
            } while (v23 != -1);
            v29 = (long)v6 - (long)v16 >> 5;
          }
        }
        else if ((v19 != v21) && (v23 = v12 + -1, v11 = v15, v6 = v21, v12)) {
          do {
            v21 = &v6[-4];
            v7 = *v21;
            v8 = v6[-3];
            v15 = &v11[-4];
            *v10 = (long)v21;
            *v15 = v7;
            v11[-3] = v8;
            v7 = v6[-1];
            v11[-2] = v6[-2];
            v11[-1] = v7;
            if (v21 == v19) break;
            v23 -= 1;
            v11 = v15;
            v6 = v21;
          } while (v23 != -1);
        }
      }
      else {
        v22 = '\0';
        v29 = 0;
        v24 = 0;
label_b4e8:
        v14 = v10[6];
        v20 = v10[5];
        v12 = v23;
        if (v29 != v14) goto label_b4f5;
      }
label_b4fe:
      *v13 = v15;
    }
    else {
      v11 = v6;
      if (v5 != v19) { // branch-flip
        v24 = v23;
        while ((unsigned long *)v10[3] != v11) {
          v23 = v24 + -1;
          if (!v24) {
            v24 = -1;
            break;
          }
          if (1 <= (int)sub_a0b0(&v21[-4],&v11[-4])) { // branch-flip
            v29 = v10[1];
            v10[1] = v29 + -0x20;
            sub_a3c0(v29 + -0x20,a5,v9);
            v21 = (unsigned long *)*v10;
            v11 = (unsigned long *)v10[1];
            if (v21 == (unsigned long *)v10[2]) goto label_b5f5;
          }
          else {
            v29 = *v10;
            *v10 = v29 + -0x20;
            sub_a3c0(v29 + -0x20,a5,v9);
            v21 = (unsigned long *)*v10;
            v11 = (unsigned long *)v10[1];
            if (v21 == (unsigned long *)v10[2]) {
label_b5f5:
              v14 = (long)v5 - (long)v21 >> 5;
              v29 = (long)v6 - (long)v11 >> 5;
              goto label_b618;
            }
          }
          v24 = v23;
        }
        v29 = v10[6];
        if ((long)v6 - (long)v11 >> 5 != v29) { // branch-flip
          v20 = v10[5];
          v14 = (long)v5 - (long)v21 >> 5;
label_b8a8:
          if (((v20 != v14) || (v11 == (unsigned long *)v10[3])) || (!v24)) {
            v22 = *(char *)((long)v10 + 0x54);
            v29 = (long)v6 - v10[1] >> 5;
            v14 = v10[6];
          }
          else {
            do {
              v24 -= 1;
              v10[1] = (long)&v11[-4];
              sub_a3c0(&v11[-4],a5,v9);
              v11 = (unsigned long *)v10[1];
              if (v11 == (unsigned long *)v10[3]) break;
            } while (v24);
            v22 = *(char *)((long)v10 + 0x54);
            v21 = (unsigned long *)*v10;
            v20 = v10[5];
            v29 = (long)v6 - (long)v11 >> 5;
            v14 = v10[6];
          }
        }
        else if (((unsigned long *)v10[2] != v21) && (v24)) {
          do {
            v24 -= 1;
            *v10 = (long)&v21[-4];
            sub_a3c0(&v21[-4],a5,v9);
            v21 = (unsigned long *)*v10;
            if (v21 == (unsigned long *)v10[2]) break;
          } while (v24);
          v22 = *(char *)((long)v10 + 0x54);
          v29 = (long)v6 - v10[1] >> 5;
          v20 = v10[5];
          v14 = v10[6];
        }
        else {
          v22 = *(char *)((long)v10 + 0x54);
          v20 = v10[5];
          v14 = v29;
        }
      }
      else {
        v29 = 0;
        v14 = 0;
label_b618:
        v20 = v10[5];
        v24 = v23;
        if (v10[6] != v29) goto label_b8a8;
        v22 = *(char *)((long)v10 + 0x54);
        v14 = v29;
      }
    }
    v10[5] = v20 - ((long)v5 - (long)v21 >> 5);
    v10[6] = v14 - v29;
    if (!v22)
      sub_b1a0(a4,v10);
    if (2 <= *(unsigned int *)&v10[10]) { // branch-flip
      pthread_mutex_lock((void *)(v10[7] + 0x58));
      v23 = v10[7];
      if (!*(char *)(v23 + 0x54)) {
        sub_b1a0(a4,v23);
        v23 = v10[7];
      }
      pthread_mutex_unlock((void *)(v23 + 0x58));
    }
    else if (!(v10[6] + v10[5])) {
      v23 = v10[7];
      pthread_mutex_lock(v2);
      sub_df40(*a4,v23);
      *(char *)(v23 + 0x54) = 1;
      pthread_cond_signal(v1);
      pthread_mutex_unlock(v2);
    }
    pthread_mutex_unlock(v3);
  } while( true );
}

// Function: sub_ba80 @ 0xba80
unsigned long sub_ba80(unsigned long *a0)
{
  sub_b220(*a0,a0[1],a0[2],a0[3],a0[4],a0[5],a0[6]);
  return 0;
}

// Function: sub_bab0 @ 0xbab0
void sub_bab0(int a0)
{
  void *v1;
  char *v10; // stack - 0xb0
  char *v11; // stack - 0x88
  char *v12; // stack - 0x80
  char *v13; // stack - 0x78
  char *v14; // stack - 0x70
  char *v15; // stack - 0x68
  char *v16; // stack - 0x60
  unsigned long v17; // stack - 0x58
  unsigned long v18; // stack - 0x50
  unsigned long v19; // stack - 0x40
  unsigned long v2;
  char *v20 [4]; // stack - 0xa8
  int v3; // eax
  char *v4; // rax
  char **v5; // rbx
  char *v6; // stack - 0xb8
  char *v7;
  char *v8; // r13
  long v9; // fs_offset
  
  v2 = dat_1d888;
  v5 = &v6;
  v19 = *(unsigned long *)(v9 + 0x28);
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5),v2,v2);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Write sorted concatenation of all FILE(s) to standard output.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nWith no FILE, or when FILE is -, read standard input.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Ordering options:\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -b, --ignore-leading-blanks  ignore leading blanks\n  -d, --dictionary-order      consider only blanks and alphanumeric characters\n  -f, --ignore-case           fold lower case to upper case characters\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -g, --general-numeric-sort  compare according to general numerical value\n  -i, --ignore-nonprinting    consider only printable characters\n  -M, --month-sort            compare (unknown) < \'JAN\' < ... < \'DEC\'\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -h, --human-numeric-sort    compare human readable numbers (e.g., 2K 1G)\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -n, --numeric-sort          compare according to string numerical value\n  -R, --random-sort           shuffle, but group identical keys.  See shuf(1)\n      --random-source=FILE    get random bytes from FILE\n  -r, --reverse               reverse the result of comparisons\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --sort=WORD             sort according to WORD:\n                                general-numeric -g, human-numeric -h, month -M,\n                                numeric -n, random -R, version -V\n  -V, --version-sort          natural sort of (version) numbers within text\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Other options:\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --batch-size=NMERGE   merge at most NMERGE inputs at once;\n                            for more use temp files\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -c, --check, --check=diagnose-first  check for sorted input; do not sort\n  -C, --check=quiet, --check=silent  like -c, but do not report first bad line\n      --compress-program=PROG  compress temporaries with PROG;\n                              decompress them with PROG -d\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --debug               annotate the part of the line used to sort,\n                              and warn about questionable usage to stderr\n      --files0-from=F       read input from the files specified by\n                            NUL-terminated names in file F;\n                            If F is - then read names from standard input\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -k, --key=KEYDEF          sort via a key; KEYDEF gives location and type\n  -m, --merge               merge already sorted files; do not sort\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -o, --output=FILE         write result to FILE instead of standard output\n  -s, --stable              stabilize sort by disabling last-resort comparison\n  -S, --buffer-size=SIZE    use SIZE for main memory buffer\n",5),v1);
    __printf_chk(1,dcgettext(NULL,"  -t, --field-separator=SEP  use SEP instead of non-blank to blank transition\n  -T, --temporary-directory=DIR  use DIR for temporaries, not $TMPDIR or %s;\n                              multiple options specify multiple directories\n      --parallel=N          change the number of sorts run concurrently to N\n  -u, --unique              with -c, check for strict ordering;\n                              without -c, output only the first of an equal run\n",5),"/tmp");
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -z, --zero-terminated     line delimiter is NUL, not newline\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --help        display this help and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --version     output version information and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nKEYDEF is F[.C][OPTS][,F[.C][OPTS]] for start and stop position, where F is a\nfield number and C a character position in the field; both are origin 1, and\nthe stop position defaults to the line\'s end.  If neither -t nor -b is in\neffect, characters in a field are counted from the beginning of the preceding\nwhitespace.  OPTS is one or more single-letter ordering options [bdfgiMhnRrV],\nwhich override global ordering options for that key.  If no key is given, use\nthe entire line as the key.  Use --debug to diagnose incorrect key usage.\n\nSIZE may be followed by the following multiplicative suffixes:\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"% 1% of memory, b 1, K 1024 (default), and so on for M, G, T, P, E, Z, Y.\n\n*** WARNING ***\nThe locale specified by the environment affects sort order.\nSet LC_ALL=C to get the traditional sort order that uses\nnative byte values.\n",5),v1);
    v6 = "[";
    v7 = "[";
    v10 = "test invocation";
    v20[0] = "coreutils";
    v20[1] = "Multi-call invocation";
    v11 = "sha256sum";
    v20[2] = "sha224sum";
    v13 = "sha384sum";
    v20[3] = "sha2 utilities";
    v12 = "sha2 utilities";
    v14 = "sha2 utilities";
    v15 = "sha512sum";
    v16 = "sha2 utilities";
    v17 = 0;
    v18 = 0;
    do {
      if (!strcmp("sort",v7)) break;
      v7 = *(char **)((long)v5 + 0x10);
      v5 = (char *)((long)v5 + 0x10);
    } while (v7);
    v7 = *(char **)((long)v5 + 8);
    if (!v7)
      v7 = "sort";
    __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v4 = setlocale(5,NULL);
    if (v4) {
      v3 = strncmp(v4,"en_",3);
      v1 = stdout;
      if (v3)
        fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
    }
    v4 = "sort";
    if (!strcmp("sort","["))
      v4 = "test";
    v8 = " invocation";
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v4);
    if (v7 != "sort")
      v8 = "";
    __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v7,v8);
  }
  exit(a0); // no-return
}

// Function: sub_c030 @ 0xc030
void sub_c030(void)
{
  sub_bab0(1); // tail-call, no-return
}

// Function: sub_c040 @ 0xc040
unsigned long sub_c040(char *a0,long *a1,void *a2,unsigned long a3)
{
  bool v1;
  unsigned long v2;
  int v3; // eax
  unsigned long v4; // rax
  long v5; // rbx
  void *v6;
  char *v7;
  long v8; // stack - 0x58
  
  v4 = strlen(a0);
  v2 = v4;
  v7 = (char *)*a1;
  if (v7) { // branch-flip
    v1 = 0;
    v5 = 0;
    v8 = -1;
    v6 = a2;
    do {
      v3 = strncmp(v7,a0,v2);
      if (!v3) {
        v4 = strlen(v7);
        if (v4 == v2) {
          v4 = v5;
          return v4;
        }
        if (v8 != -1) { // branch-flip
          if (a2) { // branch-flip
            v3 = memcmp((void *)(v8 * a3 + (long)a2),v6,a3);
            if (v3)
              v1 = 1;
          }
          else {
            v1 = 1;
          }
        }
        else {
          v8 = v5;
        }
      }
      v5 += 1;
      v6 = (void *)((long)v6 + a3);
      v7 = (char *)a1[v5];
    } while (v7);
    v4 = -2;
    if (!v1)
      v4 = v8;
    v8 = v4;
  }
  else {
    v8 = -1;
  }
  v4 = v8;
  return v4;
}

// Function: sub_c150 @ 0xc150
long sub_c150(char *a0,long *a1) // return-dupe
{
  long v1; // rbx
  char *v2;
  
  v2 = (char *)*a1;
  if (!v2)
    return -1;
  v1 = 0;
  do {
    if (!strcmp(v2,a0))
      return v1;
    v1 += 1;
    v2 = (char *)a1[v1];
  } while (v2);
  return -1;
}

// Function: sub_c1b0 @ 0xc1b0
void sub_c1b0(unsigned long a0,unsigned long a1,long a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (a2 != -1) // branch-flip
    v1 = dcgettext(NULL,"ambiguous argument %s for %s",5);
  else {
    v1 = dcgettext(NULL,"invalid argument %s for %s",5);
  }
  v2 = sub_10fb0(1,a0);
  error(0,0,v1,sub_107b0(0,8,a1),v2); // tail-call
}

// Function: sub_c240 @ 0xc240
void sub_c240(long *a0,void *a1,unsigned long a2)
{
  long v1;
  char *v2;
  void *v3;
  long v4;
  void *v5;
  
  v3 = stderr;
  v5 = NULL;
  fputs_unlocked(dcgettext(NULL,"Valid arguments are:",5),v3);
  v4 = 0;
  v1 = *a0;
  while (v1) {
    if ((v4) && (!memcmp(v5,a1,a2)))
      __fprintf_chk(stderr,1,", %s",sub_10fd0(v1));
    else {
      __fprintf_chk(stderr,1,"\n  - %s",sub_10fd0(v1));
      v5 = a1;
    }
    v4 += 1;
    a1 = (void *)((long)a1 + a2);
    v1 = a0[v4];
  }
  v2 = *(char **)((long)stderr + 0x28);
  if (v2 < *(char **)((long)stderr + 0x30)) {
    *(char **)((long)stderr + 0x28) = &v2[1];
    *v2 = 10;
    return;
  }
  __overflow(stderr,10); // tail-call
}

// Function: sub_c370 @ 0xc370
long sub_c370(unsigned long a0,char *a1,unsigned long *a2,unsigned long a3,unsigned long a4,void *a5,char a6) // early-return
{
  char *v1;
  long v2;
  
  if (a6) { // branch-flip
    v2 = sub_c040(a1,a2,a3,a4);
    if (0 <= v2)
      return v2;
  }
  else {
    v2 = 0;
    v1 = (char *)*a2;
    while (v1) {
      if (!strcmp(v1,a1))
        return v2;
      v2 += 1;
      v1 = (char *)a2[v2];
    }
    v2 = -1;
  }
  sub_c1b0(a0,a1,v2);
  sub_c240(a2,a3,a4);
  (*a5)();
  return -1;
}

// Function: sub_c440 @ 0xc440
long sub_c440(void *a0,long *a1,void *a2,unsigned long a3) // return-dupe
{
  long v1;
  
  v1 = *a1;
  if (!v1)
    return 0;
  do {
    a1 = &a1[1];
    if (!memcmp(a0,a2,a3))
      return v1;
    v1 = *a1;
    a2 = (void *)((long)a2 + a3);
  } while (v1);
  return 0;
}

// Function: sub_c4a0 @ 0xc4a0
void sub_c4a0(unsigned long a0)
{
  dat_1d878 = a0;
}

// Function: sub_c4b0 @ 0xc4b0
void sub_c4b0(char a0)
{
  dat_1d870 = a0;
}

// Function: sub_c4c0 @ 0xc4c0
void sub_c4c0(void)
{
  int v1; // eax
  int *v2; // rax
  char *v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_13720(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!dat_1d870) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (dat_1d878) { // branch-flip
        v4 = sub_10b40(dat_1d878);
        error(0,*v2,"%s: %s",v4,v3);
      }
      else {
        error(0,*v2,"%s",v3);
      }
      _exit(dat_1d130); // return-dupe, no-return
    }
  }
  v1 = sub_13720(stderr);
  if (!v1)
    return;
  _exit(dat_1d130);
}

// Function: sub_c570 @ 0xc570
void sub_c570(void)
{
  posix_fadvise(); // tail-call
}

// Function: sub_c580 @ 0xc580
void sub_c580(void *a0,unsigned int a1)
{
  if (a0) {
    posix_fadvise(fileno(a0),0,0,a1); // tail-call
    return;
  }
}

// Function: sub_c5b0 @ 0xc5b0
unsigned long sub_c5b0(void *a0) // early-return
{
  int v1;
  int *v2; // rax
  
  if (0 <= fileno(a0)) {
    if (__freading(a0)) {
      v1 = fileno(a0);
      if (lseek(v1,0,1) == -1) {
        return fclose(a0); // tail-call, return-dupe
      }
    }
    if (sub_c640(a0)) {
      v2 = __errno_location();
      v1 = *v2;
      if (!v1)
        return CONCAT44(dat_4,fclose(a0));
      *v2 = v1;
      return 0xffffffff;
    }
  }
  return fclose(a0);
}

// Function: sub_c640 @ 0xc640
void sub_c640(unsigned int *a0)
{
  if (a0) {
    if ((__freading(a0)) && (*a0 & 0x100)) {
      sub_cbc0(a0,0,1);
      fflush(a0); // tail-call
      return;
    }
  }
  fflush(a0); // tail-call
}

// Function: sub_c690 @ 0xc690
unsigned long sub_c690(long a0,unsigned long *a1)
{
  char v1;
  unsigned long v2;
  unsigned long v3;
  unsigned long v4;
  unsigned long v5;
  unsigned long v6; // r10
  
  v2 = *a1;
  v5 = 0;
  v4 = 0;
label_c6c2:
  v6 = v4;
  v3 = v5;
  v5 = v3 + 1;
  v4 = v5;
  if ((v2 <= v5) || (*(char *)(a0 + v3) != '.')) {
    if (0 <= (long)v2) { // branch-flip
label_c6b0:
      if (v2 == v3) goto label_c715;
    }
    else if (!*(char *)(a0 + v3)) {
label_c715:
      *a1 = v3;
      return v6;
    }
    goto label_c6c2;
  }
  v1 = *(char *)(a0 + 1 + v3);
  if ('[' <= v1) { // branch-flip
    if ((0x1a <= (unsigned char)(v1 + 0x9fU)) && (v1 != '~')) {
label_c6a4:
      if (0 <= (long)v2) goto label_c6b0;
      goto label_c6c2;
    }
  }
  else if (v1 <= '@') goto label_c6a4;
  for (v5 = v3 + 2; v4 = v6, v5 < v2; v5 = v5 + 1) {
    v1 = *(char *)(a0 + v5);
    if ('[' <= v1) { // branch-flip
      if ((0x1a <= (unsigned char)(v1 + 0x9fU)) && (v1 != '~')) break;
    }
    else if ((v1 <= '@') && (10 <= (unsigned char)(v1 - 0x30U))) break;
  }
  goto label_c6c2;
}

// Function: sub_c740 @ 0xc740
int sub_c740(long a0,long a1,long a2,long a3) // ternary
{
  unsigned char v1;
  int v10; // r10d
  unsigned int v11;
  unsigned int v12; // r11d
  bool v13; // zf
  long v2; // rax
  long v3; // rdx
  unsigned int v4;
  unsigned int v5;
  char v6; // r9b
  int v7;
  int v8;
  unsigned int v9; // r9d
  
  v2 = 0;
  v3 = 0;
label_c74c:
  if (a1 <= v3) goto label_c8b8;
label_c755:
  v1 = *(unsigned char *)(a0 + v3);
  if ((unsigned int)((int)(char)v1 - 0x30U) <= 9) goto label_c7fc;
  v4 = 0;
  if ((unsigned int)v1 - 0x30 <= 9) goto label_c7a0;
label_c77c:
  v4 = (unsigned int)v1;
  if ('[' <= (char)v1) { // branch-flip
    if ((unsigned char)(v1 + 0x9f) <= 0x19) goto label_c7a0;
  }
  else if ('@' < (char)v1) goto label_c7a0;
  v4 = (v1 != 0x7e) ? v1 + 0x100 : 0xfffffffe; // branch-flip
label_c7a0:
  if (v2 == a3) {
    v11 = 0xffffffff;
    return v4 - v11; // return-dupe
  }
label_c7a9:
  v1 = *(unsigned char *)(a2 + v2);
  v9 = (unsigned int)v1;
  v12 = (unsigned int)v1;
  v11 = 0;
  v5 = v4;
  if ((unsigned int)v1 - 0x30 <= 9) goto label_c7e2;
label_c7be:
  v6 = (char)v9;
  v11 = v12; // branch-flip
  v4 = v5;
  if ('[' <= v6) {
    if (0x1a <= (unsigned char)(v6 + 0x9fU)) goto label_c7d1;
  }
  else if (v6 <= '@') {
label_c7d1:
    if (v6 != '~') // branch-flip
      v11 = v12 + 0x100;
    else {
      v11 = 0xfffffffe;
    }
  }
label_c7e2:
  if (v11 == v4) {
    v3 += 1;
    v2 += 1;
label_c7f3:
    if (v3 < a1) goto label_c755;
label_c7fc:
    if (v2 < a3) {
      v9 = (unsigned int)(char)*(unsigned char *)(a2 + v2);
      if (9 < v9 - 0x30) goto label_c8d0;
    }
    if (v3 < a1) {
      do {
        if (*(char *)(a0 + v3) != '0') {
          if (a3 <= v2) goto label_c970;
          goto label_c85d;
        }
        v3 += 1;
      } while (a1 != v3);
    }
    if (v2 < a3) {
label_c85d:
      do {
        if (*(char *)(a2 + v2) != '0') {
          v7 = 0;
          v8 = 0;
          if (a1 <= v3) goto label_c9ef;
          goto label_c86f;
        }
        v2 += 1;
      } while (a3 != v2);
label_c970:
      if (v3 < a1) {
        v7 = 0;
        goto label_c97c;
      }
      goto label_c8b8;
    }
    goto label_c74c;
  }
  return v4 - v11;
label_c8d0:
  if (v3 != a1) goto label_c8d9;
  v12 = (unsigned int)*(unsigned char *)(a2 + v2);
  v5 = 0xffffffff;
  v4 = 0xffffffff;
  if (v12 - 0x30 <= 9) {
    v11 = 0;
    return v4 - v11;
  }
  goto label_c7be;
label_c8d9:
  v1 = *(unsigned char *)(a0 + v3);
  if (9 < (unsigned int)v1 - 0x30) goto label_c77c;
  v4 = 0;
  goto label_c7a9;
  while( true ) {
    if (10 <= (unsigned int)((int)*(char *)(a2 + v2) - 0x30U))
      return 1;
    v10 = (int)*(char *)(a0 + v3) - (int)*(char *)(a2 + v2);
    v13 = v7 == 0;
    v7 = v8;
    if (v13) {
      v7 = v10;
      v8 = v10;
    }
    v3 += 1;
    v2 += 1;
    if (a1 <= v3) break;
label_c86f:
    if (a3 <= v2) goto label_c9ef;
    if (10 <= (unsigned int)((int)*(char *)(a0 + v3) - 0x30U)) goto label_c88b;
  }
  goto label_c88b;
label_c9ef:
  if (v3 < a1) {
label_c97c:
    if ((unsigned int)((int)*(char *)(a0 + v3) - 0x30U) <= 9)
      return 1;
  }
label_c88b:
  if ((v2 < a3) && ((unsigned int)((int)*(char *)(a2 + v2) - 0x30U) <= 9))
    return -1;
  if (!v7) {
    if (v3 < a1) goto label_c755;
label_c8b8:
    if (v2 < a3) goto label_c7f3;
    v7 = 0;
  }
  return v7;
}

// Function: sub_ca00 @ 0xca00
unsigned long sub_ca00(char *a0,long a1,char *a2,long a3) // early-return x4
{
  long v1;
  unsigned int v2; // eax
  long v3; // rax
  long v4;
  unsigned long v5; // rax
  long v6; // stack - 0x28
  long v7; // stack - 0x20
  bool v8;
  bool v9; // zf
  
  v8 = a1 == 0;
  if (a1 < 0)
    v8 = *a0 == '\0';
  v9 = a3 == 0;
  if (a3 <= -1)
    v9 = *a2 == '\0';
  if (v8)
    return (unsigned long)((unsigned int)v9 - 1);
  if (v9)
    return 1;
  if (*a0 != '.') {
    if (*a2 == '.')
      return 1;
    goto label_ca64;
  }
  if (*a2 != '.')
    return 0xffffffff;
  v8 = a1 == 1;
  if (a1 <= -1)
    v8 = a0[1] == '\0';
  v9 = a3 == 1;
  if (a3 <= -1)
    v9 = a2[1] == '\0';
  if (v8) {
    return (unsigned long)((unsigned int)v9 - 1);
  }
  if (v9)
    return 1;
  if (a0[1] != '.') { // branch-flip
label_cb30:
    if (a2[1] != '.') goto label_ca64;
    v2 = 0;
  }
  else {
    if (a1 < 0) {
      if (!a0[2]) goto label_cb6b;
      goto label_cb30;
    }
    if (a1 != 2) goto label_cb30;
label_cb6b:
    if (a2[1] != '.')
      return 0xffffffff;
    v2 = 1;
  }
  if (0 <= a3) { // branch-flip
    if (a3 == 2)
      return (unsigned long)(v2 ^ 1);
  }
  else if (!a2[2]) {
    return (unsigned long)(v2 ^ 1);
  }
  if (v2)
    return 0xffffffff;
label_ca64:
  v6 = a3;
  v7 = a1;
  v3 = sub_c690(a0,&v7);
  v4 = sub_c690(a2,&v6);
  v1 = v7;
  if ((v7 != v3) || (v6 != v4)) {
    v5 = sub_c740(a0,v3,a2,v4);
    if ((int)v5)
      return v5;
    v4 = v6;
  }
  return sub_c740(a0,v1,a2,v4);
}

// Function: sub_cba0 @ 0xcba0
void sub_cba0(unsigned long a0,unsigned long a1)
{
  sub_ca00(a0,0xffffffffffffffff,a1,0xffffffffffffffff); // tail-call
}

// Function: sub_cbc0 @ 0xcbc0
unsigned long sub_cbc0(unsigned int *a0,unsigned long a1,unsigned int a2) // return-dupe
{
  long v1; // rax
  
  if (((*(long *)&a0[4] == *(long *)&a0[2]) && (*(long *)&a0[10] == *(long *)&a0[8])) && (!*(long *)&a0[0x12])) {
    v1 = lseek(fileno(a0),a1,a2);
    if (v1 != -1) {
      *a0 = *a0 & 0xffffffef;
      *(long *)&a0[0x24] = v1;
      return 0;
    }
    return 0xffffffff;
  }
  return fseeko(a0); // tail-call
}

// Function: sub_cc40 @ 0xcc40
bool sub_cc40(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_11c90(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_ccd0 @ 0xccd0
unsigned long sub_ccd0(unsigned long a0,unsigned long a1)
{
  return (a0 >> 3 | a0 << 0x3d) % a1;
}

// Function: sub_ccf0 @ 0xccf0
bool sub_ccf0(long a0,long a1)
{
  return a1 == a0;
}

// Function: sub_cd00 @ 0xcd00
unsigned long sub_cd00(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0x17200)
    return 1;
  v1 = v2[2];
  if ((((dat_17214 < v1) && (v1 < dat_17218)) && (dat_1721c < v2[3])) && (dat_170f0 <= *v2)) {
    v3 = *v2 + dat_17214;
    if (((v3 < v2[1]) && (v2[1] <= dat_17220)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0x17200;
  return 0;
}

// Function: sub_cd90 @ 0xcd90
unsigned long sub_cd90(long *a0,long a1,unsigned long *a2,char a3) // early-return
{
  long *v1;
  long v2;
  char v3; // al
  unsigned long v4; // rax
  long *v5;
  
  v4 = (*(void *)a0[6])(a1,a0[2]);
  if ((unsigned long)a0[2] <= v4)
    abort(); // no-return
  v5 = (long *)(v4 * 0x10 + *a0);
  *a2 = v5;
  v4 = *v5;
  if (!v4) {
    v4 = 0;
    return v4;
  }
  if (v4 != a1) {
    v3 = (*(void *)a0[7])(a1);
    if (!v3) {
      while( true ) {
        if (!(long *)v5[1]) {
          v4 = 0;
          return v4;
        }
        if ((*(long *)v5[1] == a1) || (v3 = (*(void *)a0[7])(a1), v3)) break;
        v5 = (long *)v5[1];
      }
      v1 = (long *)v5[1];
      v4 = *v1;
      if (!a3)
        return v4;
      v5[1] = v1[1];
      *v1 = 0;
      v1[1] = a0[9];
      a0[9] = (long)v1;
      return v4;
    }
    v4 = *v5;
  }
  if (a3) {
    v1 = (long *)v5[1];
    if (v1) {
      v2 = v1[1];
      *v5 = *v1;
      v5[1] = v2;
      *v1 = 0;
      v1[1] = a0[9];
      a0[9] = (long)v1;
      return v4;
    }
    *v5 = 0;
  }
  return v4;
}

// Function: sub_cea0 @ 0xcea0
unsigned long sub_cea0(float a0,unsigned long a1,char a2) // ternary
{
  char v1 [16];
  unsigned long v2; // rax
  unsigned long v3; // rcx
  unsigned long v4; // rsi
  long v5; // rdi
  unsigned long v6;
  
  if (!a2) {
    a0 = (float)a1 / a0;
    if (dat_17224 <= a0) {
      v2 = 0;
      return v2;
    }
    a1 = (a0 < dat_17228) ? (unsigned long)a0 : (long)(a0 - dat_17228) ^ 0x8000000000000000;
  }
  v2 = 10;
  if (10 <= a1)
    v2 = a1;
  v6 = v2 | 1;
  do {
    if (v6 == 0xffffffffffffffff) {
      v2 = 0;
      return v2;
    }
    v1._8_8_ = 0;
    v1._0_8_ = v6;
    v2 = v6 / 3;
    if (10 <= v6) { // branch-flip
      if (v6 != (SUB168(v1._0_16_ * ZEXT816(0xaaaaaaaaaaaaaaab),8) & 0xfffffffffffffffe) + v2) {
        v5 = 0x10;
        v4 = 9;
        v3 = 3;
        do {
          v3 += 2;
          v4 += v5;
          if (v6 <= v4) goto label_cf75;
          v5 += 8;
        } while (v6 % v3);
      }
    }
    else {
      v3 = 3;
label_cf75:
      if (v6 % v3) {
        v2 = v6 >> 0x3d;
        if (v6 >> 0x3c & 1) {
          v2 = 0;
          return v2;
        }
        if (v2) {
          v2 = 0;
          return v2;
        }
        v2 = v6;
        return v2;
      }
    }
    v6 += 2;
  } while( true );
}

// Function: sub_cfe0 @ 0xcfe0
unsigned long sub_cfe0(long *a0,unsigned long *a1,char a2) // return-dupe x2
{
  long v1;
  unsigned long v2; // rax
  long *v3; // rax
  unsigned long v4;
  long *v5;
  long *v6;
  long *v7;
  long v8;
  
  v6 = (long *)*a1;
  if ((long *)a1[1] <= v6)
    return 1;
label_d016:
  do {
    v8 = *v6;
    if (v8) {
      if ((long *)v6[1]) {
        v4 = a0[2];
        v7 = (long *)v6[1];
        while( true ) {
          v8 = *v7;
          v2 = (*(void *)a0[6])(v8,v4);
          v4 = a0[2];
          if (v4 <= v2) {
            abort(); // no-return, return-dupe
          }
          v3 = (long *)(v2 * 0x10 + *a0);
          v5 = (long *)v7[1];
          if (*v3) { // branch-flip
            v7[1] = v3[1];
            v3[1] = (long)v7;
          }
          else {
            *v3 = v8;
            v8 = a0[9];
            a0[3] = a0[3] + 1;
            *v7 = 0;
            v7[1] = v8;
            a0[9] = (long)v7;
          }
          if (!v5) break;
          v7 = v5;
        }
        v8 = *v6;
      }
      v6[1] = 0;
      if (!a2) {
        v4 = (*(void *)a0[6])(v8,a0[2]);
        if ((unsigned long)a0[2] <= v4)
          abort();
        v7 = (long *)(v4 * 0x10 + *a0);
        if (*v7) { // branch-flip
          v5 = (long *)a0[9];
          if (v5) // branch-flip
            a0[9] = v5[1];
          else {
            v5 = malloc(0x10);
            if (!v5)
              return 0;
          }
          v1 = v7[1];
          *v5 = v8;
          v5[1] = v1;
          v7[1] = (long)v5;
        }
        else {
          *v7 = v8;
          a0[3] = a0[3] + 1;
        }
        *v6 = 0;
        v6 = &v6[2];
        a1[3] = a1[3] + -1;
        if ((long *)a1[1] <= v6)
          return 1;
        goto label_d016;
      }
    }
    v6 = &v6[2];
    if ((long *)a1[1] <= v6)
      return 1;
  } while( true );
}

// Function: sub_d150 @ 0xd150
unsigned long sub_d150(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}

// Function: sub_d160 @ 0xd160
unsigned long sub_d160(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}

// Function: sub_d170 @ 0xd170
unsigned long sub_d170(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}

// Function: sub_d180 @ 0xd180
unsigned long sub_d180(unsigned long *a0)
{
  long v1;
  long *v2;
  unsigned long v3;
  unsigned long v4;
  
  v2 = (long *)*a0;
  v4 = 0;
  do {
    if ((long *)a0[1] <= v2)
      return v4;
    while (*v2) {
      v3 = 1;
      for (v1 = v2[1]; v1; v1 = *(long *)(v1 + 8)) {
        v3 += 1;
      }
      if (v4 < v3)
        v4 = v3;
      v2 = &v2[2];
      if ((long *)a0[1] <= v2)
        return v4;
    }
    v2 = &v2[2];
  } while( true );
}

// Function: sub_d1e0 @ 0xd1e0
bool sub_d1e0(unsigned long *a0)
{
  long v1;
  long *v2;
  long v3;
  long v4;
  
  v2 = (long *)*a0;
  v3 = 0;
  v4 = 0;
  do {
    while( true ) {
      if ((long *)a0[1] <= v2) {
        if (a0[3] != v4)
          return 0;
        return a0[4] == v3;
      }
      if (*v2) break;
label_d200:
      v2 = &v2[2];
    }
    v1 = v2[1];
    v4 += 1;
    v3 += 1;
    if (!v1) goto label_d200;
    do {
      v1 = *(long *)(v1 + 8);
      v3 += 1;
    } while (v1);
    v2 = &v2[2];
  } while( true );
}

// Function: sub_d250 @ 0xd250
void sub_d250(unsigned long *a0,void *a1)
{
  unsigned long v1;
  unsigned long v2;
  long v3;
  long *v4;
  unsigned long v5;
  unsigned long v6;
  
  v6 = 0;
  v1 = a0[2];
  v2 = a0[3];
  for (v4 = (long *)*a0; v4 < (long *)a0[1]; v4 = &v4[2]) {
    while (!*v4) {
      v4 = &v4[2];
      if ((long *)a0[1] <= v4) goto label_d2bd;
    }
    v5 = 1;
    for (v3 = v4[1]; v3; v3 = *(long *)(v3 + 8)) {
      v5 += 1;
    }
    if (v6 < v5)
      v6 = v5;
  }
label_d2bd:
  __fprintf_chk(a1,1,"# entries:         %lu\n",a0[4]);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v2);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v6); // tail-call
  return;
}

// Function: sub_d3b0 @ 0xd3b0
long sub_d3b0(long *a0,long a1) // return-dupe
{
  void *v1;
  unsigned long v2; // rax
  long *v3;
  long v4;
  
  v2 = (*(void *)a0[6])(a1,a0[2]);
  if ((unsigned long)a0[2] <= v2)
    abort(); // no-return
  v3 = (long *)(v2 * 0x10 + *a0);
  v4 = *v3;
  if (v4) {
    while( true ) {
      if (v4 == a1)
        return a1;
      v1 = (void *)a0[7];
      if ((*v1)(a1)) break;
      v3 = (long *)v3[1];
      if (!v3)
        return 0;
      v4 = *v3;
    }
    return *v3;
  }
  return 0;
}

// Function: sub_d420 @ 0xd420
long sub_d420(unsigned long *a0)
{
  long *v1;
  
  if (!a0[4])
    return 0;
  v1 = (long *)*a0;
  while( true ) {
    if ((long *)a0[1] <= v1)
      abort(); // no-return
    if (*v1) break;
    v1 = &v1[2];
  }
  return *v1;
}

// Function: sub_d470 @ 0xd470
long sub_d470(long *a0,long a1)
{
  long v1;
  unsigned long v2; // rax
  long *v3; // rax
  long *v4;
  
  v2 = (*(void *)a0[6])(a1,a0[2]);
  if ((unsigned long)a0[2] <= v2)
    abort(); // no-return
  v3 = (long *)(v2 * 0x10 + *a0);
  v4 = v3;
  do {
    v1 = *v4;
    v4 = (long *)v4[1];
    if (v1 == a1) {
      if (v4)
        return *v4;
      break;
    }
  } while (v4);
  do {
    v3 = &v3[2];
    if ((long *)a0[1] <= v3)
      return 0;
  } while (!*v3);
  return *v3;
}

// Function: sub_d500 @ 0xd500
void sub_d500(unsigned long *a0,long a1,unsigned long a2)
{
  unsigned long v1; // rax
  long *v2;
  long *v3;
  
  v3 = (long *)*a0;
  v1 = 0;
  if ((long *)a0[1] <= v3)
    return;
  do {
    if (*v3) {
      v2 = v3;
      do {
        if (a2 <= v1)
          return;
        v1 += 1;
        *(long *)(a1 + -8 + v1 * 8) = *v2;
        v2 = (long *)v2[1];
      } while (v2);
    }
    v3 = &v3[2];
  } while (v3 < (long *)a0[1]);
}

// Function: sub_d550 @ 0xd550
long sub_d550(unsigned long *a0,void *a1,unsigned long a2)
{
  long *v1;
  long v2;
  long v3; // r12
  long *v4;
  
  v4 = (long *)*a0;
  if ((long *)a0[1] <= v4) // branch-flip
    v3 = 0;
  else {
    v3 = 0;
    do {
      v2 = *v4;
      if (v2) {
        v1 = v4;
        while( true ) {
          if (!(*a1)(v2,a2))
            return v3;
          v1 = (long *)v1[1];
          v3 += 1;
          if (!v1) break;
          v2 = *v1;
        }
      }
      v4 = &v4[2];
    } while (v4 < (long *)a0[1]);
  }
  return v3;
}

// Function: sub_d5d0 @ 0xd5d0
unsigned long sub_d5d0(unsigned char *a0,unsigned long a1)
{
  unsigned char v1;
  unsigned long v2;
  
  v2 = 0;
  v1 = *a0;
  while (v1) {
    a0 = &a0[1];
    v2 = (v2 * 0x1f + (unsigned long)v1) % a1;
    v1 = *a0;
  }
  return v2;
}

// Function: sub_d610 @ 0xd610
void sub_d610(unsigned long *a0)
{
  *(char *)&a0[2] = 0;
  *a0 = 0x3f80000000000000;
  a0[1] = 0x3fb4fdf43f4ccccd;
}

// Function: sub_d640 @ 0xd640
unsigned long * sub_d640(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_ccd0;
  if (!a3)
    a3 = sub_ccf0;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = 0x17200;
  v1[5] = a1;
  if (sub_cd00(v1)) {
    v2 = sub_cea0(*(unsigned int *)(a1 + 8),a0,*(char *)(a1 + 0x10));
    v1[2] = v2;
    if (v2) {
      v3 = calloc(v2,0x10);
      *v1 = v3;
      if (v3) {
        v1[6] = a2;
        v1[3] = 0;
        v1[7] = a3;
        v1[8] = a4;
        v1[1] = (void *)((long)v3 + v1[2] * 0x10);
        v1[4] = 0;
        v1[9] = 0;
        return v1;
      }
    }
  }
  free(v1);
  return NULL;
}

// Function: sub_d750 @ 0xd750
void sub_d750(unsigned long *a0)
{
  unsigned long *v1;
  unsigned long v2;
  void *v3;
  unsigned long *v4;
  long *v5;
  long *v6; // r12
  
  v5 = (long *)*a0;
  if ((long *)*a0 < (long *)a0[1]) {
    do {
      while (!*v5) {
        v5 = &v5[2];
        if ((long *)a0[1] <= v5) goto label_d7eb;
      }
      v3 = (void *)a0[8];
      v4 = (unsigned long *)v5[1];
      if ((unsigned long *)v5[1]) {
        while( true ) {
          if (v3) {
            (*v3)(*v4);
            v3 = (void *)a0[8];
          }
          v1 = (unsigned long *)v4[1];
          v2 = a0[9];
          *v4 = 0;
          v4[1] = v2;
          a0[9] = v4;
          if (!v1) break;
          v4 = v1;
        }
      }
      if (v3)
        (*v3)(*v5);
      *v5 = 0;
      v6 = &v5[2];
      v5[1] = 0;
      v5 = v6;
    } while (v6 < (long *)a0[1]);
  }
label_d7eb:
  a0[3] = 0;
  a0[4] = 0;
  return;
}

// Function: sub_d800 @ 0xd800
void sub_d800(unsigned long *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = (long *)*a0;
  v3 = (long *)a0[1];
  if ((a0[8]) && (a0[4])) {
    if (v3 <= v4) goto label_d8ac;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_d86f;
      }
      v3 = v4;
      while( true ) {
        (*(void *)a0[8])(v5);
        v3 = (long *)v3[1];
        if (!v3) break;
        v5 = *v3;
      }
      v3 = (long *)a0[1];
      v4 = &v4[2];
    } while (v4 < v3);
label_d86f:
    v4 = (long *)*a0;
  }
  if (v4 < v3) {
    do {
      v2 = (void *)v4[1];
      while (v2) {
        v1 = *(void **)((long)v2 + 8);
        free(v2);
        v2 = v1;
      }
      v4 = &v4[2];
    } while (v4 < (long *)a0[1]);
  }
label_d8ac:
  v2 = (void *)a0[9];
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free((void *)*a0);
  free(a0); // tail-call
}

// Function: sub_d8f0 @ 0xd8f0
unsigned int sub_d8f0(unsigned long *a0,unsigned long a1) // return-dupe
{
  unsigned int v1; // eax
  unsigned long v10; // stack - 0x48
  unsigned long v11; // stack - 0x40
  void *v2; // stack - 0x88
  void *v3; // stack - 0x80
  unsigned long v4; // stack - 0x78
  unsigned long v5; // stack - 0x70
  unsigned long v6; // stack - 0x68
  unsigned long v7; // stack - 0x60
  unsigned long v8; // stack - 0x58
  unsigned long v9; // stack - 0x50
  
  v4 = sub_cea0(*(unsigned int *)(a0[5] + 8),a1,*(char *)(a0[5] + 0x10));
  if (!v4)
    return 0;
  if (a0[2] == v4)
    return 1;
  v2 = calloc(v4,0x10);
  if (!v2)
    return 0;
  v3 = (void *)((long)v2 + v4 * 0x10);
  v5 = 0;
  v7 = a0[5];
  v6 = 0;
  v8 = a0[6];
  v9 = a0[7];
  v10 = a0[8];
  v11 = a0[9];
  v1 = sub_cfe0(&v2,a0,0);
  if (!(char)v1) {
    a0[9] = v11;
    if ((sub_cfe0(a0,&v2,1)) && (sub_cfe0(a0,&v2,0))) {
      free(v2);
      return v1;
    }
    abort(); // no-return
  }
  free((void *)*a0);
  *a0 = v2;
  a0[1] = v3;
  a0[2] = v4;
  a0[3] = v5;
  a0[9] = v11;
  return v1;
}

// Function: sub_da70 @ 0xda70
long * sub_da70(long a0,long a1,long *a2) // return-dupe, ternary
{
  long v1;
  long *v2; // rax
  long *v3; // stack - 0x38
  unsigned long v4;
  float v5; // xmm4_da
  float v6; // xmm5_da
  
  if (!a1) {
    abort(); // no-return, return-dupe
  }
  v2 = (long)sub_cd90(a0,a1,&v3,0);
  if (v2) {
    if (!a2) {
      v2 = NULL;
      return v2;
    }
    *a2 = (long)v2;
    v2 = NULL;
    return v2;
  }
  v4 = *(unsigned long *)(a0 + 0x18);
  if (0 <= (long)v4) { // branch-flip
    v6 = (float)(long)v4;
    v2 = *(long **)(a0 + 0x10);
    if ((long)v2 > -1) goto label_daff;
label_db86:
    v5 = (float)v2;
  }
  else {
    v2 = *(long **)(a0 + 0x10);
    v6 = (float)v4;
    if (0 > (long)v2) goto label_db86;
label_daff:
    v5 = (float)(long)v2;
  }
  if (*(float *)(*(long *)(a0 + 0x28) + 8) * v5 < v6) {
    sub_cd00(a0);
    v1 = *(long *)(a0 + 0x28);
    if (v5 * *(float *)(v1 + 8) < v6) {
      v5 = v5 * *(float *)(v1 + 0xc);
      if (!*(char *)(v1 + 0x10))
        v5 = v5 * *(float *)(v1 + 8);
      if (dat_17224 <= v5) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v4 = (dat_17228 <= v5) ? (long)(v5 - dat_17228) ^ 0x8000000000000000 : (unsigned long)v5;
      if (!sub_d8f0(a0,v4)) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v2 = (long)sub_cd90(a0,a1,&v3,0);
      if (v2)
        abort();
    }
  }
  if (!*v3) {
    *v3 = a1;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
    v2 = (long *)0x1;
    return v2;
  }
  v2 = *(long **)(a0 + 0x48);
  if (v2) // branch-flip
    *(long *)(a0 + 0x48) = v2[1];
  else {
    v2 = malloc(0x10);
    if (!v2) {
      v2 = (long *)0xffffffff;
      return v2;
    }
  }
  v1 = v3[1];
  *v2 = a1;
  v2[1] = v1;
  v3[1] = (long)v2;
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
  v2 = (long *)0x1;
  return v2;
}

// Function: sub_dca0 @ 0xdca0
unsigned long sub_dca0(unsigned long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_da70(a0,a1,&v3);
  if (v1 != -1) {
    v4 = v1 == 0;
    v2 = a1;
    if (v4)
      v2 = v3;
    return v2;
  }
  v2 = 0;
  return v2;
}

// Function: sub_dd00 @ 0xdd00
long sub_dd00(long a0,unsigned long a1) // ternary x2
{
  unsigned long v1;
  float *v2;
  void *v3;
  void *v4;
  long v5; // rax
  unsigned long v6; // rax
  long *v7; // stack - 0x28
  float v8; // xmm4_da
  float v9;
  
  v5 = sub_cd90(a0,a1,&v7,1);
  if (!v5)
    return 0;
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + -1;
  if (!*v7) {
    v6 = *(long *)(a0 + 0x18) - 1;
    *(unsigned long *)(a0 + 0x18) = v6;
    v1 = (0 <= (long)v6) ? *(unsigned long *)(a0 + 0x10) : *(unsigned long *)(a0 + 0x10); // branch-flip
    v9 = (float)v6;
    v8 = (float)v1;
    if (v9 < **(float **)(a0 + 0x28) * v8) {
      sub_cd00(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_17228 <= v8) ? (long)(v8 - dat_17228) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_d8f0(a0,v6)) {
          v4 = *(void **)(a0 + 0x48);
          while (v4) {
            v3 = *(void **)((long)v4 + 8);
            free(v4);
            v4 = v3;
          }
          *(unsigned long *)(a0 + 0x48) = 0;
        }
      }
    }
    return v5;
  }
  return v5;
}

// Function: sub_de90 @ 0xde90
void sub_de90(void)
{
  sub_dd00(); // tail-call
}

// Function: sub_dea0 @ 0xdea0
unsigned long sub_dea0(void)
{
  return 0;
}

// Function: sub_deb0 @ 0xdeb0
unsigned long * sub_deb0(void *a0,long a1)
{
  unsigned long *v1; // rax
  unsigned long *v2; // rax
  
  v1 = (unsigned long *)sub_12ab0(0x20);
  if (!a1)
    a1 = 1;
  v2 = (unsigned long *)sub_12bf0(a1,8);
  v1[1] = a1;
  *v2 = 0;
  *v1 = v2;
  v1[2] = 0;
  if (!a0)
    a0 = sub_dea0;
  v1[3] = a0;
  return v1;
}

// Function: sub_df20 @ 0xdf20
void sub_df20(unsigned long *a0)
{
  free((void *)*a0);
  free(a0); // tail-call
}

// Function: sub_df40 @ 0xdf40
long sub_df40(long *a0,unsigned long a1)
{
  unsigned long *v1;
  unsigned long v10; // rbx
  unsigned long *v2;
  long v3;
  void *v4;
  unsigned long v5;
  unsigned long v6;
  bool v7;
  long v8; // rax
  unsigned long v9;
  
  v9 = a0[2];
  v8 = *a0;
  if ((unsigned long)(a0[1] - 1U) <= v9) {
    v8 = sub_12d00(v8,&a0[1],8);
    v9 = a0[2];
    *a0 = v8;
  }
  v9 += 1;
  a0[2] = v9;
  *(unsigned long *)(v8 + v9 * 8) = a1;
  v3 = *a0;
  v4 = (void *)a0[3];
  v5 = *(unsigned long *)(v3 + v9 * 8);
  if (v9 != 1) { // branch-flip
    while( true ) {
      v10 = v9 >> 1;
      v1 = (unsigned long *)(v3 + v10 * 8);
      v6 = *v1;
      v8 = v9 * 8;
      v2 = (unsigned long *)(v3 + v8);
      v7 = 1 <= (int)(*v4)(v6,v5);
      if (v7) break;
      *v2 = *v1;
      v9 = v10;
      if (v10 == 1) goto label_dfe0;
    }
  }
  else {
label_dfe0:
    v8 = 8;
  }
  *(unsigned long *)(v3 + v8) = v5;
  v8 = 0;
  return v8;
}

// Function: sub_e010 @ 0xe010
unsigned long * sub_e010(long *a0)
{
  unsigned long v1;
  unsigned long v10;
  unsigned long v11;
  unsigned long *v12;
  unsigned long v13; // stack - 0x40
  unsigned long *v2;
  long v3;
  long v4;
  unsigned long v5;
  void *v6;
  bool v7;
  int v8; // eax
  unsigned long *v9; // rax
  
  v3 = a0[2];
  v13 = 0;
  if (v3) {
    v4 = *a0;
    v13 = *(unsigned long *)(v4 + 8);
    v1 = v3 - 1;
    v5 = *(unsigned long *)(v4 + v3 * 8);
    a0[2] = v1;
    *(unsigned long *)(v4 + 8) = v5;
    v3 = *a0;
    v6 = (void *)a0[3];
    v5 = *(unsigned long *)(v3 + 8);
    if (v1 >> 1) { // branch-flip
      v11 = 1;
      while( true ) {
        v10 = v11 * 2;
        if (v1 <= v10) // branch-flip
          v12 = (unsigned long *)(v3 + v11 * 0x10);
        else {
          v2 = (unsigned long *)(v3 + (v10 + 1) * 8);
          v12 = (unsigned long *)(v3 + v11 * 0x10);
          v8 = (*v6)(*v12,*v2);
          if (v8 <= -1) {
            v10 += 1;
            v12 = v2;
          }
        }
        v8 = (*v6)(*v12,v5);
        v2 = (unsigned long *)(v3 + v11 * 8);
        v7 = v8 <= 0;
        v9 = v2;
        if (v7) goto label_e103;
        *v2 = *v12;
        if (v1 >> 1 < v10) break;
        v11 = v10;
      }
      v9 = v12;
    }
    else {
      v9 = (unsigned long *)(v3 + 8);
    }
label_e103:
    *v9 = v5;
  }
  v9 = (unsigned long *)v13;
  return v9;
}

// Function: sub_e120 @ 0xe120
char * sub_e120(unsigned int a0,long a1)
{
  unsigned int v1;
  char *v2;
  char *v3; // r8
  
  *(char *)(a1 + 0xb) = 0;
  v2 = (char *)(a1 + 0xb);
  if ((int)a0 <= -1) {
    do {
      v3 = v2;
      v1 = (int)a0 / 10;
      v3[-1] = ((char)v1 * '\n' + '0') - (char)a0;
      v2 = &v3[-1];
      a0 = v1;
    } while (v1);
    v3[-2] = '-';
    return &v3[-2];
  }
  do {
    v2 = &v2[-1];
    *v2 = (char)a0 + (char)(a0 / 10) * '\xf6' + '0';
    a0 /= 10;
  } while (a0);
  return v2;
}

// Function: sub_e1b0 @ 0xe1b0
char * sub_e1b0(unsigned long a0,long a1)
{
  unsigned int v1; // edi
  char *v2;
  
  *(char *)(a1 + 10) = 0;
  v2 = (char *)(a1 + 10);
  do {
    v1 = (unsigned int)a0;
    v2 = &v2[-1];
    *v2 = (char)a0 + (char)((a0 & 0xffffffff) / 10) * '\xf6' + '0';
    a0 = (a0 & 0xffffffff) / 10;
  } while (10 <= v1);
  return v2;
}

// Function: sub_e200 @ 0xe200
char * sub_e200(unsigned long a0,long a1)
{
  bool v1;
  char *v2;
  
  *(char *)(a1 + 0x14) = 0;
  v2 = (char *)(a1 + 0x14);
  do {
    v2 = &v2[-1];
    *v2 = (char)a0 + (char)(a0 / 10) * '\xf6' + '0';
    v1 = 10 <= a0;
    a0 /= 10;
  } while (v1);
  return v2;
}

// Function: sub_e260 @ 0xe260
int sub_e260(unsigned char *a0,long a1,unsigned int a2)
{
  unsigned char *v1;
  unsigned char v2;
  unsigned short v3;
  int v4; // eax
  long v5; // rax
  long v6;
  unsigned long v7; // stack - 0x48
  int v8; // stack - 0x4c
  int v9; // r12d
  
  v1 = &a0[a1];
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
    v9 = 0;
    if (a0 < v1) {
      do {
        v2 = *a0;
        if ('`' <= (char)v2) { // branch-flip
          if (0x1e <= (unsigned char)(v2 + 0x9f)) goto label_e2e4;
label_e393:
          a0 = &a0[1];
          v9 += 1;
        }
        else {
          if ('@' < (char)v2) goto label_e393;
          if ('#' < (char)v2) {
            if (0x1b <= (unsigned char)(v2 - 0x25)) goto label_e2e4;
            goto label_e393;
          }
          if ('\x1f' < (char)v2) goto label_e393;
label_e2e4:
          v7 = 0;
          do {
            v5 = sub_13c20(&v8,a0,(long)v1 - (long)a0,&v7);
            if (v5 == -1) {
              if (a2 & 1)
                return -1;
              goto label_e393;
            }
            if (v5 == -2) {
              if (a2 & 1)
                return -1;
              v9 += 1;
              a0 = v1;
              break;
            }
            v6 = 1;
            if (v5)
              v6 = v5;
            v4 = wcwidth(v8);
            if (0 <= v4) { // branch-flip
              if (0x7fffffffU - v9 < v4)
                return 0x7fffffff;
              v9 += v4;
            }
            else {
              if (a2 & 2)
                return -1;
              v4 = iswcntrl(v8);
              if (!v4) {
                if (v9 == 0x7fffffff)
                  return 0x7fffffff;
                v9 += 1;
              }
            }
            a0 = &a0[v6];
          } while (!mbsinit(&v7));
        }
      } while (a0 < v1);
    }
  }
  else {
    v9 = 0;
    if (a0 < v1) {
      v9 = 0;
      do {
        v2 = *a0;
        a0 = &a0[1];
        v3 = *(unsigned short *)(*(long *)__ctype_b_loc() + (unsigned long)v2 * 2);
        if (v3 & 0x4000) { // branch-flip
label_e419:
          if (v9 == 0x7fffffff)
            return 0x7fffffff;
          v9 += 1;
        }
        else {
          if (a2 & 2)
            return -1;
          if (!(v3 & 2)) goto label_e419;
        }
      } while (v1 != a0);
    }
  }
  return v9;
}

// Function: sub_e480 @ 0xe480
void sub_e480(char *a0,unsigned int a1)
{
  sub_e260(a0,strlen(a0),a1); // tail-call
}

// Function: sub_e4b0 @ 0xe4b0
unsigned long sub_e4b0(char *a0) // return-dupe
{
  int v1; // eax
  unsigned long v2; // rax
  char v3;
  char *v4; // stack - 0x18
  
  v3 = *a0;
  v1 = (int)v3;
  if (!v3)
    return 0;
  if ('\r' < v3) goto label_e500;
  do {
    if ((char)v1 <= '\b')
      return 0;
    while( true ) {
      v3 = a0[1];
      v1 = (int)v3;
      a0 = &a0[1];
      if (!v3)
        return 0;
      if (v3 <= '\r') break;
label_e500:
      if ((char)v1 != ' ') {
        if (9 < (unsigned int)(v1 - 0x30U))
          return 0;
        v4 = NULL;
        v2 = strtoul(a0,&v4,10);
        if (v4) {
          v3 = *v4;
          if (!v3)
            return v2;
          v4 = &v4[1];
          if ('\r' < v3) goto label_e553;
          do {
            if (v3 <= '\b')
              return 0;
            while( true ) {
              v3 = *v4;
              v4 = &v4[1];
              if (!v3)
                return v2;
              if (v3 <= '\r') break;
label_e553:
              if (v3 != ' ') {
                if (v3 != ',')
                  return 0;
                return v2;
              }
            }
          } while( true );
        }
        return 0;
      }
    }
  } while( true );
}

// Function: sub_e570 @ 0xe570
unsigned long sub_e570(int a0)
{
  unsigned long v1; // rax
  char *v2; // rax
  unsigned long v3;
  char v4 [136];
  
  if (a0 != 2) { // branch-flip
    if (a0 != 1) {
      v1 = sysconf(0x53);
      v3 = v1;
      if (v1 - 1 < 2) {
        if (sched_getaffinity(0,0x80,v4))
          return v1;
        v3 = (unsigned long)(int)__sched_cpucount(0x80,v4);
        if (!v3)
          return v1;
        if (v3 <= v1)
          return v1;
      }
      if ((long)v3 > 0)
        return v3;
      return 1;
    }
  }
  else {
    v2 = getenv("OMP_NUM_THREADS");
    if (v2) {
      v1 = sub_e4b0(v2);
      v2 = getenv("OMP_THREAD_LIMIT");
      if (v2) { // branch-flip
        v3 = sub_e4b0(v2);
        if (!v3)
          v3 = 0xffffffffffffffff;
      }
      else {
        v3 = 0xffffffffffffffff;
      }
      if (v1) {
        if (v3 > v1)
          return v1;
        return v3;
      }
      goto label_e64f;
    }
    v2 = getenv("OMP_THREAD_LIMIT");
    if ((v2) && (v3 = sub_e4b0(v2), v3)) goto label_e64f;
  }
  v3 = 0xffffffffffffffff;
label_e64f:
  if (!sched_getaffinity(0,0x80,v4)) {
    v1 = (unsigned long)(int)__sched_cpucount(0x80,v4);
    if (v1) {
      if (v3 > v1)
        return v1;
      return v3;
    }
  }
  v1 = sysconf(0x54);
  if (v1 <= v3)
    v3 = v1;
  if ((long)v1 <= 0)
    v3 = 1;
  return v3;
}

// Function: sub_e760 @ 0xe760
double sub_e760(void)
{
  long v1; // rax
  char v2 [32];
  double v3;
  unsigned long v4; // stack - 0x68
  unsigned int v5; // stack - 0x20
  
  v3 = (double)sysconf(0x55);
  v1 = sysconf(0x1e);
  if ((v3 < 0.0) || ((double)v1 < 0.0)) {
    v3 = dat_17250;
    if (!sysinfo(v2))
      v3 = (double)v4 * (double)v5;
  }
  else {
    v3 = v3 * (double)v1;
  }
  return v3;
}

// Function: sub_e850 @ 0xe850
double sub_e850(void)
{
  long v1; // rax
  char v2 [40];
  double v3; // xmm0_qa
  double v4; // xmm0_qa
  unsigned long v5; // stack - 0x60
  unsigned long v6; // stack - 0x50
  unsigned int v7; // stack - 0x20
  
  v3 = (double)sysconf(0x56);
  v1 = sysconf(0x1e);
  if ((0.0 <= v3) && (0.0 <= (double)v1))
    return v3 * (double)v1;
  if (!sysinfo(v2))
    return ((double)v5 + (double)v6) * (double)v7;
  sub_e760();
  return v4 * dat_170e8;
}

// Function: sub_e990 @ 0xe990
unsigned int sub_e990(int *a0,unsigned int a1) // return-dupe x2
{
  int v1;
  unsigned long v2;
  unsigned int v3; // eax
  int *v4; // rax
  
  v2 = *(unsigned long *)a0;
  if (0 <= dat_1d880) {
    v3 = pipe2();
    if ((0 <= (int)v3) || (*__errno_location() != 0x26)) {
      dat_1d880 = 1;
      return v3;
    }
    dat_1d880 = -1;
  }
  if (a1 & 0xfff7f7ff) {
    *__errno_location() = 0x16;
    return 0xffffffff;
  }
  if (0 <= pipe(a0)) {
    if ((!(a1 & 0x800)) || ((((v3 = sub_13850(a0[1],3,0), 0 <= (int)v3 && (v1 = a0[1], sub_13850(v1,4,v3 | 0x800) != -1)) && (v3 = sub_13850(*a0,3,0), 0 <= (int)v3)) && (v1 = *a0, sub_13850(v1,4,v3 | 0x800) != -1)))) {
      if (!(a1 & 0x80000))
        return a1 & 0xfff7f7ff;
      v3 = sub_13850(a0[1],1,0);
      if (((0 <= (int)v3) && (v1 = a0[1], sub_13850(v1,2,v3 | 1) != -1)) && ((v3 = sub_13850(*a0,1,0), 0 <= (int)v3 && (v1 = *a0, sub_13850(v1,2,v3 | 1) != -1))))
        return a1 & 0xfff7f7ff;
    }
    v4 = __errno_location();
    v1 = *v4;
    close(*a0);
    close(a0[1]);
    *(unsigned long *)a0 = v2;
    *v4 = v1;
    return 0xffffffff;
  }
  return 0xffffffff;
}

// Function: sub_eb70 @ 0xeb70
unsigned long sub_eb70(void)
{
  char *v1; // rax
  unsigned long v2; // rax
  char *v3; // stack - 0x18
  unsigned long v4;
  
  v4 = 0x31069;
  v1 = getenv("_POSIX2_VERSION");
  if (((v1) && (*v1)) && (v2 = strtol(v1,&v3,10), !*v3)) {
    if (-0x80000000 <= (long)v2) { // branch-flip
      v4 = 0x7fffffff;
      if ((long)v2 <= 0x7fffffff)
        v4 = v2;
    }
    else {
      v4 = 0x80000000;
    }
  }
  return v4 & 0xffffffff;
}

// Function: sub_ec00 @ 0xec00
void sub_ec00(char *a0)
{
  char *v1;
  char *v2; // rax
  
  if (a0) {
    v2 = strrchr(a0,0x2f);
    if ((v2) && (v1 = &v2[1], 7 <= (long)v1 - (long)a0)) {
      if (!strncmp(&v2[-6],"/.libs/",7)) {
        a0 = v1;
        if (!strncmp(v1,"lt-",3)) {
          a0 = &v2[4];
          __progname = a0;
        }
      }
    }
    dat_1d888 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_eca0 @ 0xeca0
char * sub_eca0(char *a0,int a1) // return-dupe x3
{
  unsigned char *v1; // rax
  
  v1 = (unsigned char *)sub_13be0();
  if ((*v1 & 0xdf) != 0x55) { // branch-flip
    if (((((*v1 & 0xdf) == 0x47) && ((v1[1] & 0xdf) == 0x42)) && (v1[2] == 0x31)) && (((v1[3] == 0x38 && (v1[4] == 0x30)) && ((v1[5] == 0x33 && ((v1[6] == 0x30 && (!v1[7])))))))) {
      if (*a0 == '`')
        return (char *)0x172b5;
      return (char *)0x172b0;
    }
  }
  else if (((((v1[1] & 0xdf) == 0x54) && ((v1[2] & 0xdf) == 0x46)) && (v1[3] == 0x2d)) && ((v1[4] == 0x38 && (!v1[5])))) {
    if (*a0 == '`')
      return "‘";
    return "’";
  }
  if (a1 == 9)
    return "\"";
  return "\'";
}

// Function: sub_ed80 @ 0xed80
long * sub_ed80(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
{
  unsigned long v1;
  long *v10; // rax
  char v11;
  unsigned char *v12;
  unsigned long v13;
  unsigned long v14;
  unsigned long v15; // stack - 0x48
  unsigned int v16; // stack - 0x4c
  unsigned char v17;
  unsigned int v18;
  unsigned long v19;
  bool v2;
  unsigned char v20;
  bool v21;
  unsigned long v22;
  bool v23;
  unsigned long v24;
  bool v25;
  bool v26;
  bool v27;
  bool v28;
  bool v29;
  bool v3;
  long v30; // stack - 0xe0
  unsigned long v31; // stack - 0xd8
  char *v32; // stack - 0xb0
  unsigned long v33; // stack - 0xa8
  unsigned long v34; // stack - 0xa0
  unsigned int v35; // stack - 0x80
  char *v36; // stack - 0x70
  char *v37; // stack - 0x68
  bool v4;
  bool v5;
  long v6;
  long v7;
  unsigned char v8;
  unsigned int v9; // eax
  
  v30 = a6;
  v37 = a7;
  v36 = a8;
  v35 = a5;
label_edf0:
  v10 = (unsigned long)__ctype_get_mb_cur_max();
  v34 = (unsigned long)v10;
  v18 = v35 & 2;
  switch(a4) {
    case 0:
      v26 = 0;
      v2 = 0;
      v13 = 0;
      v31 = 0;
      v32 = NULL;
      break;
    case 1:
label_f296:
      v2 = 0;
      goto label_f250;
    case 2:
      if (!v18) {
        v2 = 0;
        goto label_f3b1;
      }
      v26 = 1;
      v2 = 0;
      v13 = 0;
      v31 = 1;
      v32 = "\'";
      break;
    case 3:
      v2 = 1;
label_f250:
      v26 = 1;
      v13 = 0;
      a4 = 2;
      v31 = 1;
      v32 = "\'";
      break;
    case 4:
      if (!v18) {
        v2 = 1;
        goto label_f3b1;
      }
      goto label_f296;
    case 5:
      if (!v18) goto label_f1f1;
      v26 = 1;
      v2 = 1;
      v13 = 0;
      v31 = 1;
      v32 = "\"";
      break;
    case 6:
      v26 = 1;
      v13 = 0;
      v2 = 1;
      v31 = 1;
      a4 = 5;
      v32 = "\"";
      break;
    case 7:
      v26 = 0;
      v2 = 1;
      v13 = 0;
      v31 = 0;
      v32 = NULL;
      break;
    case 8:
    case 9:
    case 10:
      if (a4 != 10) {
        v10 = (char *)dcgettext(NULL,"`",5);
        v37 = (char *)v10;
        if (v10 == (long *)"`") {
          v10 = (char *)sub_eca0("`",a4);
          v37 = (char *)v10;
        }
        v10 = (char *)dcgettext(NULL,"\'",5);
        v36 = (char *)v10;
        if (v10 == (long *)0x16dcf) {
          v10 = (char *)sub_eca0("\'",a4);
          v36 = (char *)v10;
        }
      }
      v13 = 0;
      if (!v18) {
        v11 = *v37;
        while (v11) {
          if (v13 < a1)
            a0[v13] = v11;
          v13 += 1;
          v11 = v37[v13];
        }
      }
      v26 = v18 != 0;
      v2 = 1;
      v10 = (unsigned long)strlen(v36);
      v32 = v36;
      v31 = (unsigned long)v10;
      break;
    default:
      abort(); // no-return
    
  }
  v4 = 1;
  v21 = 0;
  v33 = 0;
  v14 = a1;
  v3 = 0;
label_eedb:
  do {
label_eeee:
    v24 = 0;
    a1 = v14;
label_eef8:
    v14 = v33;
    v27 = a3 != v24;
    if (a3 == 0xffffffffffffffff)
      v27 = a2[v24] != '\0';
    if (v27) {
      v12 = (unsigned char *)&a2[v24];
      v23 = (bool)(a4 != 2 & v2);
      v27 = 1;
      v14 = v13;
      v28 = v2;
      v29 = v2;
      if (v23) {
        v25 = 1;
        if (v31) {
          if ((a3 == 0xffffffffffffffff) && (2 <= v31)) {
            v10 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v10;
          }
          if ((a3 < v24 + v31) || (v9 = memcmp(v12,v32,v31), v9)) goto label_f860;
          if (!v26) {
            v8 = *v12;
            v19 = (unsigned long)v8;
            v22 = v19;
            if ((char)v8 <= '?') {
              switch(v8) {
                case 0:
                  goto label_f45c;
                default:
                  goto label_f133;
                case 7:
                  goto label_f447;
                case 8:
                  goto label_f416;
                case 9:
                  goto label_f4e3;
                case 10:
                  goto label_f43d;
                case 0xb:
                  goto label_f510;
                case 0xc:
                  goto label_f3fb;
                case 0xd:
                  goto label_f34d;
                case 0x20:
                  goto label_f51d;
                case 0x21:
                case 0x22:
                case 0x24:
                case 0x26:
                case 0x28:
                case 0x29:
                case 0x2a:
                case 0x3b:
                case 0x3c:
                case 0x3d:
                case 0x3e:
                  goto label_f0e6;
                case 0x23:
                  goto label_f4c0;
                case 0x25:
                case 0x2b:
                case 0x2c:
                case 0x2d:
                case 0x2e:
                case 0x2f:
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
                case 0x3a:
                  goto label_f018;
                case 0x27:
                  goto label_f332;
                case 0x3f:
                  goto label_f2f1;
                
              }
            }
            v5 = v23;
            if ((char)v8 <= 'z') {
              if (v8 == 0x40) goto label_f133;
              v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
              if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_f018;
              if (!((unsigned long)v10 & 0xa4000000)) goto label_f904;
              goto label_f848;
            }
            if (v8 == 0x7d) goto label_f9d0;
            if ((char)v8 <= '}') {
              v22 = 0x7b;
              if (v8 == 0x7b) goto label_f82a;
              v22 = 0x7c;
              if (v8 == 0x7c) goto label_f848;
              goto label_f133;
            }
            if (v8 != 0x7e) goto label_f133;
label_f984:
            if (!v24) {
              v8 = a4 == 2;
              v22 = 0x7e;
              v27 = v3;
              goto label_f1a9;
            }
            v22 = 0x7e;
            goto label_f992;
          }
          goto label_f11c;
        }
label_f860:
        v8 = *v12;
        v19 = (unsigned long)v8;
        v22 = v19;
        if ('?' < (char)v8) {
          if ('z' < (char)v8) {
            if (v8 != 0x7d) { // branch-flip
              if ('~' <= (char)v8) {
                v23 = 0;
                if (v8 == 0x7e) goto label_f984;
                goto label_f130;
              }
              v22 = 0x7b;
              v23 = 0;
              if (v8 != 0x7b) {
                v22 = 0x7c;
                v23 = 0;
                v5 = 0;
                if (v8 != 0x7c) goto label_f133;
                goto label_f848;
              }
            }
            else {
              v22 = 0x7d;
              v23 = 0;
            }
label_f82a:
            if (a3 == 0xffffffffffffffff) goto label_f9df;
label_f838:
            v5 = v23;
            if (a3 == 1) goto label_f4c5;
label_f848:
            v8 = a4 == 2;
            v27 = v3;
            v25 = 0;
            v23 = v5;
            goto label_f1a9;
          }
          if (v8 == 0x40) goto label_f130;
          v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
          v23 = 0;
          if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_f018;
          v23 = 0;
          v5 = 0;
          if ((unsigned long)v10 & 0xa4000000) goto label_f848;
label_f904:
          if ((char)v19 == '\\') {
            if (((bool)(v2 & v26)) && (v31)) goto label_f92a;
            v17 = 0x5c;
            goto label_f352;
          }
label_f133:
          v17 = (unsigned char)v19;
          if (v34 != 1) goto label_f6b3;
label_f140:
          v10 = __ctype_b_loc();
          v22 = 1;
          v27 = (*(unsigned char *)(*v10 + 1 + v19 * 2) & 0x40) == 0;
          v28 = (bool)(v27 & v2);
          v27 = !v27;
          goto label_f18e;
        }
        switch(v8) {
          case 0:
            goto label_f44e;
          default:
label_f130:
            v23 = 0;
            goto label_f133;
          case 7:
label_f447:
            v17 = 0x61;
            goto label_f427;
          case 8:
label_f416:
            v17 = 0x62;
            goto label_f427;
          case 9:
            v23 = 0;
label_f4e3:
            v22 = 9;
            v17 = 0x74;
            goto label_f4f0;
          case 10:
label_f43d:
            v17 = 0x6e;
            break;
          case 0xb:
label_f510:
            v17 = 0x76;
            break;
          case 0xc:
label_f3fb:
            v17 = 0x66;
label_f427:
            if (v26) {
              v2 = 1;
              goto label_f37f;
            }
            v25 = 0;
            goto label_f09e;
          case 0xd:
label_f34d:
            v17 = 0x72;
            break;
          case 0x20:
            v23 = 0;
label_f51d:
            v22 = 0x20;
            goto label_f018;
          case 0x21:
          case 0x22:
          case 0x24:
          case 0x26:
          case 0x28:
          case 0x29:
          case 0x2a:
          case 0x3b:
          case 0x3c:
          case 0x3d:
          case 0x3e:
            v23 = 0;
            v27 = 0;
            goto label_f0e9;
          case 0x23:
            v22 = 0x23;
            v23 = 0;
            goto label_f4c5;
          case 0x25:
          case 0x2b:
          case 0x2c:
          case 0x2d:
          case 0x2e:
          case 0x2f:
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
          case 0x3a:
            v23 = 0;
label_f018:
            v8 = 0;
            goto label_f028;
          case 0x27:
            v23 = 0;
            goto label_f332;
          case 0x3f:
            v23 = 0;
            goto label_f2f1;
          
        }
        goto label_f352;
      }
      v17 = *v12;
      v19 = (unsigned long)v17;
      if ('?' < (char)v17) {
        if ('{' <= (char)v17) {
          if (v17 == 0x7d) {
label_f9d0:
            v22 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_f838;
label_f9df:
            v5 = v23;
            if (a2[1]) goto label_f848;
            goto label_f4c5;
          }
          if ((char)v17 <= '}') {
            v22 = 0x7b;
            if (v17 != 0x7b) {
              if (v17 == 0x7c) goto label_f0e6;
              goto label_f133;
            }
            goto label_f82a;
          }
          v22 = 0x7e;
          if (v17 == 0x7e) goto label_f4c5;
          if (v34 == 1) goto label_f140;
label_f6b3:
          v15 = 0;
          if (a3 == 0xffffffffffffffff) {
            v10 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v10;
          }
          v22 = 0;
          do {
            v19 = v24 + v22;
            v10 = (long)sub_13c20(&v16,&a2[v19],a3 - v19,&v15);
            v7 = (long)v10;
            v25 = v27;
            if (!v10) break;
            if (v10 == (long *)0xffffffffffffffff) {
              v25 = 0;
              goto label_fc1b;
            }
            if (v10 == (long *)0xfffffffffffffffe) {
              v10 = (long *)v19;
              if (v19 >= a3) goto label_ff9f;
              goto label_ff95;
            }
            if (((a4 == 2) && (v26)) && (v10 != (long *)0x1)) {
              v12 = (unsigned char *)&a2[v19 + 1];
              v6 = v19 + (long)v10;
              do {
                v9 = (unsigned int)*v12;
                v9 -= 0x5b;
                v18 = v9;
                v10 = (unsigned long)(unsigned long)v9;
                v19 = (unsigned long)v10;
                v8 = (unsigned char)v18;
                if ((v8 <= 0x21) && (0x20000002bU >> (v19 & 0x3f) & 1)) goto label_f7ed;
                v12 = &v12[1];
              } while ((unsigned char *)&a2[v6] != v12);
            }
            v9 = iswprint(v16);
            if (!v9)
              v27 = 0;
            v22 += v7;
            v9 = mbsinit(&v15);
            v25 = v27;
          } while (!v9);
          v28 = (bool)((v25 ^ 1U) & v2);
          goto label_fc1b;
        }
        if (v17 == 0x40) goto label_f133;
        v10 = (unsigned long)(1L << (v17 + 0xbf & 0x3f));
        if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_f1a0;
        if ((unsigned long)v10 & 0xa4000000) goto label_f0e6;
        if (v17 != 0x5c) goto label_f133;
        if (a4 == 2) {
          if (!v26) goto label_f92a;
          goto label_f113;
        }
        if (!(bool)(v2 & v26 & v31 != 0)) {
          v22 = 0x5c;
          v17 = 0x5c;
          v23 = 0;
          goto label_f4f0;
        }
label_f92a:
        v24 += 1;
        v25 = 0;
        v17 = 0x5c;
        v8 = v21;
        goto label_f940;
      }
      switch(v17) {
        case 0:
          if (!v2) {
            v23 = 0;
            v22 = 0;
            if (v35 & 1) goto label_f5c2;
            goto label_f4fb;
          }
label_f44e:
          if (!v26) {
            v23 = 0;
label_f45c:
            v28 = a4 == 2;
            v8 = v21 ^ 1;
            v8 &= v28;
            if ((bool)v8) {
              if (v13 < a1)
                a0[v13] = 0x27;
              if (v13 + 1 < a1)
                a0[v13 + 1] = 0x24;
              if (v13 + 2 < a1)
                a0[v13 + 2] = 0x27;
              v14 = v13 + 3;
              if (a1 <= v13 + 3) {
                v13 += 4;
                v25 = 0;
                v17 = 0x30;
                v21 = (bool)v8;
                goto label_f1d3;
              }
label_fd65:
              a0[v14] = 0x5c;
              v13 = v14;
              v21 = (bool)v8;
            }
            else if (v13 < a1) {
              v8 = v21;
              goto label_fd65;
            }
            v14 = v13 + 1;
            if (a4 != 2) {
              v22 = 0x30;
              if ((v24 + 1 < a3) && (v8 = a2[v24 + 1] - 0x30, v8 <= 9)) {
                if (v14 < a1)
                  a0[v14] = 0x30;
                if (v13 + 2 < a1)
                  a0[v13 + 2] = 0x30;
                v14 = v13 + 3;
                v22 = 0x30;
              }
label_fd9c:
              v17 = (unsigned char)v22;
              v25 = 0;
              v13 = v14;
              v8 = v27;
              if (!v2 || v28) goto label_f1d3;
              goto label_f028;
            }
            v17 = 0x30;
            v25 = 0;
            v13 = v14;
            v8 = v27;
            goto label_f1d3;
          }
          if (a4 != 2) goto label_f11c;
          goto label_f38b;
        default:
          goto label_f133;
        case 7:
          v22 = 7;
          v17 = 0x61;
          break;
        case 8:
          v22 = 8;
          v17 = 0x62;
          break;
        case 9:
          v22 = 9;
          v17 = 0x74;
          goto label_f57f;
        case 10:
          v22 = 10;
          v17 = 0x6e;
          goto label_f57f;
        case 0xb:
          v22 = 0xb;
          v17 = 0x76;
          break;
        case 0xc:
          v22 = 0xc;
          v17 = 0x66;
          break;
        case 0xd:
          v22 = 0xd;
          v17 = 0x72;
label_f57f:
          v8 = a4 == 2 & v26;
          v23 = 0;
          if (!(bool)v8) goto label_f4f0;
label_f101:
          a4 = 2;
label_f113:
          if (!v29) goto label_f11c;
          goto label_f38b;
        case 0x20:
          v19 = 0x20;
          goto label_f0e9;
        case 0x21:
        case 0x22:
        case 0x24:
        case 0x26:
        case 0x28:
        case 0x29:
        case 0x2a:
        case 0x3b:
        case 0x3c:
        case 0x3d:
        case 0x3e:
label_f0e6:
          v27 = 0;
label_f0e9:
          v22 = v19;
          v25 = v27;
          if ((a4 == 2) && (v26)) goto label_f101;
          goto label_f1a7;
        case 0x23:
label_f4c0:
          v22 = 0x23;
label_f4c5:
          if (!v24) {
            v19 = v22;
            goto label_f0e9;
          }
label_f992:
          v8 = a4 == 2;
          v27 = v3;
          v25 = 0;
          goto label_f1a9;
        case 0x25:
        case 0x2b:
        case 0x2c:
        case 0x2d:
        case 0x2e:
        case 0x2f:
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
        case 0x3a:
          goto label_f1a0;
        case 0x27:
label_f332:
          v25 = v27;
          if (a4 != 2) {
            v8 = 0;
            v22 = 0x27;
            goto label_f1a9;
          }
          if (!v26) {
            if (a1) { // branch-flip
              v14 = 0;
              v22 = a1;
              if (v33) goto label_fdbb;
            }
            else {
label_fdbb:
              if (v13 < a1)
                a0[v13] = 0x27;
              if (v13 + 1 < a1)
                a0[v13 + 1] = 0x5c;
              v14 = a1;
              v22 = v33;
              if (v13 + 2 < a1)
                a0[v13 + 2] = 0x27;
            }
            v13 += 3;
            v8 = 0;
            v17 = 0x27;
            a1 = v14;
            v33 = v22;
            v21 = 0;
            v3 = v27;
            goto label_f1d3;
          }
          goto label_f113;
        case 0x3f:
label_f2f1:
          if (a4 == 2) {
            if (v26) goto label_f113;
            v8 = 0;
            v17 = 0x3f;
            v25 = 0;
            goto label_f1d3;
          }
          if ((((a4 == 5) && (v35 & 4)) && (v19 = v24 + 2, v19 < a3)) && (a2[v24 + 1] == '?')) {
            v8 = a2[v19];
            v22 = (unsigned long)v8;
            if (v8 <= 0x3e) {
              v10 = (unsigned long)(0x7000a38200000000 >> (v8 & 0x3f));
              v1 = (unsigned long)v10 & 1;
              v8 = (unsigned char)v10 & 1;
              if (!v1) {
                v22 = 0x3f;
                v27 = v3;
                v25 = 0;
                goto label_f1a9;
              }
              if (v26) goto label_f11c;
              if (v13 < a1)
                a0[v13] = 0x3f;
              if (v13 + 1 < a1)
                a0[v13 + 1] = 0x22;
              if (v13 + 2 < a1)
                a0[v13 + 2] = 0x22;
              if (v13 + 3 < a1)
                a0[v13 + 3] = 0x3f;
              v14 = v13 + 4;
              v28 = 0;
              v27 = 0;
              v24 = v19;
              goto label_fd9c;
            }
          }
          v8 = 0;
          v22 = 0x3f;
          v27 = v3;
          v25 = 0;
          goto label_f1a9;
        
      }
label_f4f0:
      if (!v2) {
label_f4fb:
        v17 = (unsigned char)v22;
        v25 = 0;
        v8 = 0;
        v27 = v3;
        if (!v26) goto label_f1d0;
        goto label_f028;
      }
label_f352:
      v25 = 0;
      goto label_f363;
    }
    if (!(bool)(v13 == 0 & v26 & a4 == 2)) {
      v8 = a4 == 2 & (v26 ^ 1U);
      v26 = (bool)(v26 ^ 1U);
      if ((!(bool)v8) || (v26 = (bool)v8, !v3)) {
label_1005b:
        v10 = (long *)v13;
        if (((v32) && (v26)) && (v11 = *v32, v11)) {
          do {
            if (v10 < a1)
              a0[(long)v10] = v11;
            v10 = (unsigned long)((long)v10 + 1);
            v11 = v32[(long)v10 - v13];
          } while (v11);
        }
        if (v10 < a1)
          a0[(long)v10] = 0;
        return v10;
      }
      if (!v4) {
        v8 = !a1 && v33;
        v26 = v3;
        if (a1 || !v33) goto label_1005b;
        v24 = v33;
        goto label_faeb;
      }
      v13 = 0;
      a4 = 5;
      v10 = (unsigned long)__ctype_get_mb_cur_max();
      v31 = 1;
      v34 = (unsigned long)v10;
      v32 = "\"";
      if (!(v35 & 2)) goto label_1010b;
      v3 = 0;
      v21 = 0;
      v33 = 0;
      v2 = v4;
      v26 = v4;
      goto label_eeee;
    }
label_f7ed:
    if (v2) {
label_f38b:
      v2 = 1;
      v35 &= 0xfffffffd;
      v10 = (unsigned long)__ctype_get_mb_cur_max();
      v30 = 0;
      v34 = (unsigned long)v10;
    }
    else {
      v2 = 0;
      v35 &= 0xfffffffd;
      v10 = (unsigned long)__ctype_get_mb_cur_max();
      v30 = 0;
      v34 = (unsigned long)v10;
    }
label_f3b1:
    v26 = 0;
    a4 = 2;
    v13 = 1;
    v32 = "\'";
    v31 = 1;
    v21 = 0;
    v4 = 1;
    if (a1) { // branch-flip
      v8 = 0;
      v24 = 0;
      v33 = a1;
label_faeb:
      v13 = 1;
      *a0 = 0x27;
      a4 = 2;
      v26 = 0;
      v31 = 1;
      v32 = "\'";
      v14 = v33;
      v33 = v24;
      v3 = (bool)v8;
    }
    else {
      v33 = 0;
      v14 = a1;
      v3 = 0;
    }
  } while( true );
  while( true ) {
    v22 += 1;
    v10 = (unsigned long)(v24 + v22);
    if (a3 <= v10) break;
label_ff95:
    if (!a2[(long)v10]) break;
  }
label_ff9f:
  v25 = 0;
label_fc1b:
  v19 = (unsigned long)v17;
  v27 = v25;
  if (2 <= v22) {
label_fc25:
    v20 = 0;
    v22 += v24;
    v14 = v24;
    do {
      v17 = (unsigned char)v19;
      if (v28) {
        v29 = a4 == 2;
        if (v26) goto label_f113;
        v8 = v29 & (v21 ^ 1U);
        if ((bool)v8) {
          if (v13 < a1)
            a0[v13] = 0x27;
          if (v13 + 1 < a1)
            a0[v13 + 1] = 0x24;
          if (v13 + 2 < a1)
            a0[v13 + 2] = 0x27;
          v13 += 3;
          v21 = 1;
        }
        if (v13 < a1)
          a0[v13] = 0x5c;
        if (v13 + 1 < a1) {
          v8 = (char)(v19 >> 6) + '0';
          a0[v13 + 1] = v8;
        }
        if (v13 + 2 < a1) {
          v8 = ((unsigned char)(v19 >> 3) & 7) + 0x30;
          a0[v13 + 2] = v8;
        }
        v24 = v14 + 1;
        v13 += 3;
        v17 = (v17 & 7) + 0x30;
        if (v22 <= v24) goto label_f0b1;
        v20 = 1;
      }
      else {
        v8 = v20 ^ 1;
        v8 &= v21;
        if (v23) {
          if (v13 < a1)
            a0[v13] = 0x5c;
          v13 += 1;
        }
        v24 = v14 + 1;
        if (v22 <= v24) {
          goto label_f940;
        }
        if ((bool)v8) {
          if (v13 < a1)
            a0[v13] = 0x27;
          if (v13 + 1 < a1)
            a0[v13 + 1] = 0x27;
          v13 += 2;
          v23 = 0;
          v21 = 0;
        }
        else {
          v23 = 0;
        }
      }
      v14 += 1;
      if (v13 < a1)
        a0[v13] = v17;
      v19 = (unsigned long)(unsigned char)a2[v14];
      v13 += 1;
    } while( true );
  }
label_f18e:
  if (v28) {
    v25 = 0;
    v28 = v2;
    goto label_fc25;
  }
label_f1a0:
  v22 = v19;
  v25 = v27;
label_f1a7:
  v8 = a4 == 2;
  v27 = v3;
label_f1a9:
  v17 = (unsigned char)v22;
  v28 = (bool)v8 == 0;
  v3 = v27;
  v8 = 0;
  if ((v28 && v2) || (v8 = 0, v26)) {
label_f028:
    v17 = (unsigned char)v22;
    v13 = v14;
    if (!v30) goto label_f1d3;
    if (!(*(unsigned int *)(v30 + (v22 >> 5) * 4) >> (v17 & 0x1f) & 1)) goto label_f1d3;
  }
  else {
label_f1d0:
    v8 = 0;
    v3 = v27;
label_f1d3:
    if (!v23) {
      v8 ^= 1;
      v24 += 1;
      v8 &= v21;
label_f940:
      if ((bool)v8) {
        if (v13 < a1)
          a0[v13] = 0x27;
        if (v13 + 1 < a1)
          a0[v13 + 1] = 0x27;
        v21 = 0;
        v13 += 2;
      }
      goto label_f0b1;
    }
  }
label_f363:
  if (v26) {
label_f37f:
    if ((bool)(v2 & a4 == 2)) goto label_f38b;
label_f11c:
    v35 &= 0xfffffffd;
    v30 = 0;
    goto label_edf0;
  }
  v8 = v21 ^ 1;
  v8 &= a4 == 2;
  if ((bool)v8) {
    if (v13 < a1)
      a0[v13] = 0x27;
    if (v13 + 1 < a1)
      a0[v13 + 1] = 0x24;
    if (v13 + 2 < a1)
      a0[v13 + 2] = 0x27;
    v13 += 3;
    v21 = 1;
  }
label_f09e:
  if (v13 < a1)
    a0[v13] = 0x5c;
  v13 += 1;
  v24 += 1;
label_f0b1:
  if (v13 < a1)
    a0[v13] = v17;
  v13 += 1;
  if (!v25)
    v4 = 0;
  goto label_eef8;
label_f5c2:
  v24 += 1;
  goto label_eef8;
label_1010b:
  a1 = v33;
label_f1f1:
  v26 = 0;
  a4 = 5;
  v13 = 1;
  v32 = "\"";
  v2 = 1;
  v31 = 1;
  v21 = 0;
  v3 = 0;
  v4 = 1;
  v33 = 0;
  v14 = a1;
  if (a1)
    *a0 = 0x22;
  goto label_eedb;
}

// Function: sub_10210 @ 0x10210
void * sub_10210(unsigned int a0,unsigned long a1,unsigned long a2,unsigned int *a3)
{
  unsigned long *v1;
  long v10; // stack - 0x48
  int *v11; // stack - 0x68
  int v2;
  unsigned int v3;
  unsigned int v4;
  unsigned long v5;
  unsigned long *v6;
  unsigned long v7;
  void *v8;
  long v9; // rdx
  
  v11 = __errno_location();
  v2 = *v11;
  if (a0 <= 0x7ffffffe) {
    v6 = dat_1d180;
    if (dat_1d178 <= (int)a0) {
      v10 = (long)dat_1d178;
      v9 = (long)(int)((a0 - dat_1d178) + 1);
      if (dat_1d180 != (unsigned long *)0x1d190) // branch-flip
        v6 = (unsigned long *)sub_12d90(dat_1d180,&v10,v9,0x7fffffff,0x10);
      else {
        v6 = (unsigned long *)sub_12d90(0,&v10,v9,0x7fffffff,0x10);
        *v6 = dat_1d190;
        v6[1] = dat_1d198;
      }
      dat_1d180 = v6;
      memset(&v6[(long)dat_1d178 * 2],0,(v10 - dat_1d178) * 0x10);
      dat_1d178 = (int)v10;
    }
    v1 = &v6[(long)(int)a0 * 2];
    v3 = a3[1];
    v5 = *v1;
    v8 = (void *)v1[1];
    v7 = sub_ed80(v8,v5,a1,a2,*a3,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    if (v5 <= v7) {
      v7 += 1;
      *v1 = v7;
      if (v8 != (void *)0x1d8a0)
        free(v8);
      v8 = (void *)sub_12af0(v7);
      v4 = *a3;
      v1[1] = (unsigned long)v8;
      sub_ed80(v8,v7,a1,a2,v4,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    }
    *v11 = v2;
    return v8;
  }
  abort(); // no-return
}

// Function: sub_10400 @ 0x10400
void sub_10400(long a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = 0x1d9a0;
  sub_12ef0(a0,0x38);
  *v2 = v1;
}

// Function: sub_10440 @ 0x10440
unsigned int sub_10440(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x1d9a0;
  return *a0;
}

// Function: sub_10460 @ 0x10460
void sub_10460(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x1d9a0;
  *a0 = a1;
}

// Function: sub_10480 @ 0x10480
unsigned int sub_10480(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0x1d9a0;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_104c0 @ 0x104c0
unsigned int sub_104c0(long a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = 0x1d9a0;
  v1 = *(unsigned int *)(a0 + 4);
  *(unsigned int *)(a0 + 4) = a1;
  return v1;
}

// Function: sub_104e0 @ 0x104e0
void sub_104e0(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0x1d9a0;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_10520 @ 0x10520
void sub_10520(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x1d9a0;
  v2 = __errno_location();
  v1 = *v2;
  sub_ed80(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(unsigned long *)&a4[10],*(unsigned long *)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_105a0 @ 0x105a0
unsigned long sub_105a0(unsigned long a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x1d9a0;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_ed80(0,0,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  v4 = sub_12af0(v3 + 1);
  sub_ed80(v4,v3 + 1,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_10690 @ 0x10690
void sub_10690(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_105a0(a0,a1,0,a2); // tail-call
}

// Function: sub_106a0 @ 0x106a0
void sub_106a0(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_1d180;
  if (2 <= dat_1d178) {
    v4 = (unsigned long *)((long)dat_1d180 + 0x18);
    v1 = (unsigned long *)((long)dat_1d180 + (unsigned long)(unsigned int)(dat_1d178 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0x1d8a0) {
    free(*(void **)((long)v3 + 8));
    dat_1d198 = 0x1d8a0;
    dat_1d190 = 0x100;
  }
  if (v3 == (void *)0x1d190) {
    dat_1d178 = 1;
    return;
  }
  free(v3);
  dat_1d178 = 1;
  dat_1d180 = (void *)0x1d190;
}

// Function: sub_10740 @ 0x10740
void sub_10740(unsigned long a0,unsigned long a1)
{
  sub_10210(a0,a1,0xffffffffffffffff,0x1d9a0); // tail-call
}

// Function: sub_10760 @ 0x10760
void sub_10760(void)
{
  sub_10210(); // tail-call
}

// Function: sub_10770 @ 0x10770
void sub_10770(unsigned long a0)
{
  sub_10210(0,a0,0xffffffffffffffff,0x1d9a0); // tail-call
}

// Function: sub_10790 @ 0x10790
void sub_10790(unsigned long a0,unsigned long a1)
{
  sub_10210(0,a0,a1,0x1d9a0); // tail-call
}

// Function: sub_107b0 @ 0x107b0
void sub_107b0(unsigned long a0,int a1,unsigned long a2)
{
  int v1 [14]; // stack - 0x48
  
  if (a1 != 10) {
    v1[1] = 0;
    v1[2] = 0;
    v1[3] = 0;
    v1[4] = 0;
    v1[5] = 0;
    v1[6] = 0;
    v1[7] = 0;
    v1[8] = 0;
    v1[9] = 0;
    v1[10] = 0;
    v1[0xb] = 0;
    v1[0xc] = 0;
    v1[0xd] = 0;
    v1[0] = a1;
    sub_10210(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_10850 @ 0x10850
void sub_10850(unsigned long a0,int a1,unsigned long a2,unsigned long a3)
{
  int v1 [14]; // stack - 0x48
  
  if (a1 != 10) {
    v1[1] = 0;
    v1[2] = 0;
    v1[3] = 0;
    v1[4] = 0;
    v1[5] = 0;
    v1[6] = 0;
    v1[7] = 0;
    v1[8] = 0;
    v1[9] = 0;
    v1[10] = 0;
    v1[0xb] = 0;
    v1[0xc] = 0;
    v1[0xd] = 0;
    v1[0] = a1;
    sub_10210(a0,a2,a3,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_108e0 @ 0x108e0
void sub_108e0(int a0,unsigned long a1)
{
  int v1 [14]; // stack - 0x48
  
  if (a0 != 10) {
    v1[1] = 0;
    v1[2] = 0;
    v1[3] = 0;
    v1[4] = 0;
    v1[5] = 0;
    v1[6] = 0;
    v1[7] = 0;
    v1[8] = 0;
    v1[9] = 0;
    v1[10] = 0;
    v1[0xb] = 0;
    v1[0xc] = 0;
    v1[0xd] = 0;
    v1[0] = a0;
    sub_10210(0,a1,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_10970 @ 0x10970
void sub_10970(int a0,unsigned long a1,unsigned long a2)
{
  int v1 [14]; // stack - 0x48
  
  if (a0 != 10) {
    v1[1] = 0;
    v1[2] = 0;
    v1[3] = 0;
    v1[4] = 0;
    v1[5] = 0;
    v1[6] = 0;
    v1[7] = 0;
    v1[8] = 0;
    v1[9] = 0;
    v1[10] = 0;
    v1[0xb] = 0;
    v1[0xc] = 0;
    v1[0xd] = 0;
    v1[0] = a0;
    sub_10210(0,a1,a2,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_10a00 @ 0x10a00
void sub_10a00(unsigned long a0,unsigned long a1,unsigned char a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned long v3; // stack - 0x48
  unsigned long v4; // stack - 0x40
  unsigned long v5; // stack - 0x38
  unsigned long v6; // stack - 0x30
  unsigned long v7; // stack - 0x28
  unsigned long v8; // stack - 0x20
  unsigned long v9; // stack - 0x18
  
  v3 = dat_1d9a0;
  v4 = dat_1d9a8;
  v9 = dat_1d9d0;
  v5 = dat_1d9b0;
  v6 = dat_1d9b8;
  v7 = dat_1d9c0;
  v8 = dat_1d9c8;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a2 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a2 & 0x1f)) & 1) << (a2 & 0x1f) ^ v2;
  sub_10210(0,a0,a1,&v3);
}

// Function: sub_10aa0 @ 0x10aa0
void sub_10aa0(unsigned long a0,unsigned char a1)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned long v3; // stack - 0x48
  unsigned long v4; // stack - 0x40
  unsigned long v5; // stack - 0x38
  unsigned long v6; // stack - 0x30
  unsigned long v7; // stack - 0x28
  unsigned long v8; // stack - 0x20
  unsigned long v9; // stack - 0x18
  
  v3 = dat_1d9a0;
  v4 = dat_1d9a8;
  v9 = dat_1d9d0;
  v5 = dat_1d9b0;
  v6 = dat_1d9b8;
  v7 = dat_1d9c0;
  v8 = dat_1d9c8;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a1 & 0x1f)) & 1) << (a1 & 0x1f) ^ v2;
  sub_10210(0,a0,0xffffffffffffffff,&v3);
}

// Function: sub_10b40 @ 0x10b40
void sub_10b40(unsigned long a0)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_1d9a0;
  v3 = (unsigned int)((unsigned long)dat_1d9a8 >> 0x20);
  v8 = dat_1d9d0;
  v4 = dat_1d9b0;
  v5 = dat_1d9b8;
  v6 = dat_1d9c0;
  v7 = dat_1d9c8;
  v2 = (unsigned int)dat_1d9a8;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_10210(0,a0,0xffffffffffffffff,&v1);
}

// Function: sub_10bd0 @ 0x10bd0
void sub_10bd0(unsigned long a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_1d9a0;
  v3 = (unsigned int)((unsigned long)dat_1d9a8 >> 0x20);
  v8 = dat_1d9d0;
  v4 = dat_1d9b0;
  v5 = dat_1d9b8;
  v6 = dat_1d9c0;
  v7 = dat_1d9c8;
  v2 = (unsigned int)dat_1d9a8;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_10210(0,a0,a1,&v1);
}

// Function: sub_10c60 @ 0x10c60
void sub_10c60(unsigned long a0,int a1,unsigned long a2)
{
  int v1 [14]; // stack - 0x48
  
  if (a1 != 10) {
    v1[1] = 0;
    v1[2] = 0;
    v1[3] = 0x4000000;
    v1[4] = 0;
    v1[5] = 0;
    v1[6] = 0;
    v1[7] = 0;
    v1[8] = 0;
    v1[9] = 0;
    v1[10] = 0;
    v1[0xb] = 0;
    v1[0xc] = 0;
    v1[0xd] = 0;
    v1[0] = a1;
    sub_10210(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_10d00 @ 0x10d00
void sub_10d00(unsigned long a0,long a1,long a2,unsigned long a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_1d9a8;
  v4 = dat_1d9b0;
  v5 = dat_1d9b8;
  v8 = dat_1d9d0;
  v2 = (unsigned int)((unsigned long)dat_1d9a0 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_1d9c0;
  v7 = dat_1d9c8;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_10210(a0,a3,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_10da0 @ 0x10da0
void sub_10da0(unsigned int a0,long a1,long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_1d9a8;
  v4 = dat_1d9b0;
  v5 = dat_1d9b8;
  v8 = dat_1d9d0;
  v2 = (unsigned int)((unsigned long)dat_1d9a0 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_1d9c0;
  v7 = dat_1d9c8;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_10210(a0,a3,a4,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_10e40 @ 0x10e40
void sub_10e40(long a0,long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_1d9a8;
  v4 = dat_1d9b0;
  v5 = dat_1d9b8;
  v8 = dat_1d9d0;
  v2 = (unsigned int)((unsigned long)dat_1d9a0 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_1d9c0;
  v7 = dat_1d9c8;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_10210(0,a2,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_10ee0 @ 0x10ee0
void sub_10ee0(long a0,long a1,unsigned long a2,unsigned long a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_1d9a8;
  v4 = dat_1d9b0;
  v5 = dat_1d9b8;
  v8 = dat_1d9d0;
  v2 = (unsigned int)((unsigned long)dat_1d9a0 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_1d9c0;
  v7 = dat_1d9c8;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_10210(0,a2,a3,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_10f80 @ 0x10f80
void sub_10f80(void)
{
  sub_10210(); // tail-call
}

// Function: sub_10f90 @ 0x10f90
void sub_10f90(unsigned long a0,unsigned long a1)
{
  sub_10210(0,a0,a1,0x1d140); // tail-call
}

// Function: sub_10fb0 @ 0x10fb0
void sub_10fb0(unsigned long a0,unsigned long a1)
{
  sub_10210(a0,a1,0xffffffffffffffff,0x1d140); // tail-call
}

// Function: sub_10fd0 @ 0x10fd0
void sub_10fd0(unsigned long a0)
{
  sub_10210(0,a0,0xffffffffffffffff,0x1d140); // tail-call
}

// Function: sub_10ff0 @ 0x10ff0
void sub_10ff0(long a0)
{
  unsigned long v1; // rax
  int *v2; // rax
  char *v3; // rax
  
  if (a0) {
    v1 = sub_10fd0(a0);
    v2 = __errno_location();
    if (*v2) // branch-flip
      v3 = dcgettext(NULL,"%s: read error",5);
    else {
      v3 = dcgettext(NULL,"%s: end of file",5);
    }
    error(dat_1d130,*v2,v3,v1);
  }
  abort(); // no-return
}

// Function: sub_11060 @ 0x11060
long * sub_11060(long a0,unsigned long a1)
{
  int v1;
  void *v2; // rax
  long *v3;
  unsigned long v4;
  long *v5; // rax
  long v6;
  int *v7; // rax
  long *v8;
  
  if (a1) { // branch-flip
    if (a0) { // branch-flip
      v2 = (void *)sub_13b40(a0,"rb");
      if (v2) { // branch-flip
        v3 = (long *)sub_12ab0(0x1038);
        *v3 = (long)v2;
        v3[1] = (long)sub_10ff0;
        v3[2] = a0;
        v4 = 0x1000;
        if (a1 <= 0x1000)
          v4 = a1;
        setvbuf(v2,(char *)&v3[3],0,v4);
      }
      else {
        v3 = NULL;
      }
    }
    else {
      v5 = (long *)sub_12ab0(0x1038);
      *v5 = 0;
      v5[1] = (long)sub_10ff0;
      v3 = &v5[4];
      v5[2] = 0;
      v5[3] = 0;
      if (0x801 <= a1)
        a1 = 0x800;
      v8 = v3;
      do {
        while( true ) {
          if ((long *)(a1 + (long)v3) <= v8) {
            sub_116a0(v3);
            return v5;
          }
          v6 = getrandom(v8,(long)(long *)(a1 + (long)v3) - (long)v8,0);
          if (v6 <= -1) break;
          v8 = (long *)((long)v8 + v6);
        }
        v7 = __errno_location();
        v1 = *v7;
      } while (v1 == 4);
      v6 = *v5;
      __explicit_bzero_chk(v5,0x1038,0x1038);
      free(v5);
      if (v6)
        sub_c5b0(v6);
      *v7 = v1;
      v3 = NULL;
    }
  }
  else {
    v3 = (long *)sub_12ab0(0x1038);
    *v3 = 0;
    v3[1] = (long)sub_10ff0;
    v3[2] = 0;
  }
  return v3;
}

// Function: sub_11200 @ 0x11200
void sub_11200(long a0,unsigned long a1)
{
  *(unsigned long *)(a0 + 8) = a1;
}

// Function: sub_11210 @ 0x11210
void sub_11210(long a0,unsigned long a1)
{
  *(unsigned long *)(a0 + 0x10) = a1;
}

// Function: sub_11220 @ 0x11220
void sub_11220(long *a0,void *a1,unsigned long a2)
{
  long *v1;
  int *v2; // rax
  long v3;
  unsigned long v4;
  int v5;
  unsigned long v6; // r13
  void *v7;
  long *v8; // stack - 0x48
  
  v3 = *a0;
  if (v3) {
    v2 = __errno_location();
    while( true ) {
      v3 = fread_unlocked(a1,1,a2,v3);
      v5 = *v2;
      a1 = (void *)((long)a1 + v3);
      a2 -= v3;
      if (!a2) break;
      v3 = a0[2];
      if (!(*(unsigned char *)*a0 & 0x20))
        v5 = 0;
      *v2 = v5;
      (*(void *)a0[1])(v3);
      v3 = *a0;
    }
    return;
  }
  v4 = a0[3];
  v1 = &a0[4];
  v8 = &a0[0x107];
  v7 = a1; // branch-flip
  if (a2 <= v4)
    v8 = (long *)((long)v8 + (0x800 - v4));
  else {
    do {
      memcpy(v7,(void *)((0x800 - v4) + (long)v8),v4);
      v7 = (void *)((long)v7 + v4);
      a2 -= v4;
      if (!((unsigned long)v7 & 7)) {
        v6 = (unsigned long)((unsigned int)a2 & 0x7ff);
        v4 = a2;
        while (a1 = (void *)((long)v7 + (a2 - v4)), v4 != v6) {
          sub_11410(v1,a1);
          v4 -= 0x800;
          if (!v4) {
            a0[3] = 0;
            return;
          }
        }
        sub_11410(v1,v8);
        v4 = 0x800;
        a2 = v6;
        goto label_11296;
      }
      sub_11410(v1,v8);
      v4 = 0x800;
    } while (0x801 <= a2);
    a1 = v7;
  }
label_11296:
  memcpy(a1,v8,a2);
  a0[3] = v4 - a2;
  return;
}

// Function: sub_113c0 @ 0x113c0
unsigned long sub_113c0(long *a0)
{
  long v1;
  
  v1 = *a0;
  __explicit_bzero_chk(a0,0x1038,0xffffffffffffffff);
  free(a0);
  if (v1)
    return sub_c5b0(v1); // tail-call
  return 0;
}

// Function: sub_11410 @ 0x11410
void sub_11410(long *a0,long *a1)
{
  long *v1;
  long *v2;
  unsigned long v3;
  long *v4;
  long *v5; // rdi
  long v6;
  long v7;
  
  v7 = a0[0x102];
  v3 = a0[0x100];
  a0[0x102] = v7 + 1;
  v7 = v7 + 1 + a0[0x101];
  v1 = a0;
  v4 = a1;
  do {
    v6 = *v1;
    v3 = ~(v3 ^ v3 << 0x15) + v1[0x80];
    v7 = *(long *)((long)a0 + (unsigned long)((unsigned int)v6 & 0x7f8)) + v3 + v7;
    *v1 = v7;
    v6 += *(long *)((long)a0 + (unsigned long)((unsigned int)((unsigned long)v7 >> 8) & 0x7f8));
    *v4 = v6;
    v7 = v1[1];
    v3 = (v3 ^ v3 >> 5) + v1[0x81];
    v6 = *(long *)((long)a0 + (unsigned long)((unsigned int)v7 & 0x7f8)) + v3 + v6;
    v1[1] = v6;
    v7 += *(long *)((long)a0 + (unsigned long)((unsigned int)((unsigned long)v6 >> 8) & 0x7f8));
    v4[1] = v7;
    v6 = v1[2];
    v3 = (v3 * 0x1000 ^ v3) + v1[0x82];
    v7 = *(long *)((long)a0 + (unsigned long)((unsigned int)v6 & 0x7f8)) + v3 + v7;
    v1[2] = v7;
    v6 += *(long *)((long)a0 + (unsigned long)((unsigned int)((unsigned long)v7 >> 8) & 0x7f8));
    v4[2] = v6;
    v7 = v1[3];
    v3 = (v3 >> 0x21 ^ v3) + v1[0x83];
    v6 = *(long *)((long)a0 + (unsigned long)((unsigned int)v7 & 0x7f8)) + v3 + v6;
    v2 = &v1[4];
    v5 = &v4[4];
    v1[3] = v6;
    v7 = *(long *)((long)a0 + (unsigned long)((unsigned int)((unsigned long)v6 >> 8) & 0x7f8)) + v7;
    v4[3] = v7;
    v1 = v2;
    v4 = v5;
  } while (v2 != &a0[0x80]);
  v1 = &a1[0x80];
  do {
    v6 = *v2;
    v3 = ~(v3 ^ v3 << 0x15) + v2[-0x80];
    v7 = *(long *)((long)a0 + (unsigned long)((unsigned int)v6 & 0x7f8)) + v3 + v7;
    *v2 = v7;
    v6 += *(long *)((long)a0 + (unsigned long)((unsigned int)((unsigned long)v7 >> 8) & 0x7f8));
    *v1 = v6;
    v7 = v2[1];
    v3 = (v3 ^ v3 >> 5) + v2[-0x7f];
    v6 = *(long *)((long)a0 + (unsigned long)((unsigned int)v7 & 0x7f8)) + v3 + v6;
    v2[1] = v6;
    v7 += *(long *)((long)a0 + (unsigned long)((unsigned int)((unsigned long)v6 >> 8) & 0x7f8));
    v1[1] = v7;
    v6 = v2[2];
    v3 = (v3 * 0x1000 ^ v3) + v2[-0x7e];
    v7 = *(long *)((long)a0 + (unsigned long)((unsigned int)v6 & 0x7f8)) + v3 + v7;
    v2[2] = v7;
    v6 += *(long *)((long)a0 + (unsigned long)((unsigned int)((unsigned long)v7 >> 8) & 0x7f8));
    v1[2] = v6;
    v7 = v2[3];
    v3 = (v3 >> 0x21 ^ v3) + v2[-0x7d];
    v6 = *(long *)((long)a0 + (unsigned long)((unsigned int)v7 & 0x7f8)) + v3 + v6;
    v4 = &v2[4];
    v5 = &v1[4];
    v2[3] = v6;
    v7 += *(long *)((long)a0 + (unsigned long)((unsigned int)((unsigned long)v6 >> 8) & 0x7f8));
    v1[3] = v7;
    v2 = v4;
    v1 = v5;
  } while (v4 != &a0[0x100]);
  a0[0x100] = v3;
  a0[0x101] = v7;
}

// Function: sub_116a0 @ 0x116a0
void sub_116a0(unsigned long *a0)
{
  unsigned long *v1;
  unsigned long v10; // r11
  unsigned long v11; // r11
  unsigned long v12; // r12
  long v13; // rax
  long v14; // r8
  unsigned long *v2; // rax
  unsigned long v3; // rcx
  unsigned long *v4;
  unsigned long v5; // rsi
  unsigned long v6; // rdi
  unsigned long v7; // rdi
  unsigned long v8; // r9
  unsigned long v9; // rax
  
  v3 = 0x98f5704f6c44c0ab;
  v10 = 0x48fe4a0fa5a09315;
  v5 = 0x82f053db8355e0ce;
  v9 = 0xb29b2e824a595524;
  v8 = 0x8c0ea5053d4712a0;
  v7 = 0xb9f8b322c73ac862;
  v12 = 0xae985bf2cbfc89ed;
  v6 = 0x647c4677a2884b7c;
  v1 = a0;
  do {
    v13 = (*v1 - (v5 + v1[4])) + v6;
    v11 = v10 + v1[5] ^ v3 + v1[7] >> 9;
    v10 = v12 + v1[6] ^ v13 * 0x200;
    v7 = (v1[1] - v11) + v7;
    v3 = v7 >> 0x17 ^ v3 + v1[7] + v13;
    v14 = (v1[2] - v10) + v8;
    v9 = (v1[3] - v3) + v9;
    v6 = v14 * 0x8000 ^ v13 + v7;
    v13 = (v5 + v1[4]) - v6;
    *v1 = v6;
    v7 = v9 >> 0xe ^ v7 + v14;
    v11 -= v7;
    v1[1] = v7;
    v8 = v13 * 0x100000 ^ v14 + v9;
    v14 = v10 - v8;
    v1[2] = v8;
    v9 = v11 >> 0x11 ^ v9 + v13;
    v10 = v11 + v14;
    v3 -= v9;
    v1[3] = v9;
    v2 = &v1[8];
    v5 = v14 * 0x4000 ^ v13 + v11;
    v12 = v14 + v3;
    v1[5] = v10;
    v1[4] = v5;
    v1[6] = v12;
    v1[7] = v3;
    v1 = v2;
    v4 = a0;
  } while (&a0[0x100] != v2);
  do {
    v13 = (*v4 - (v5 + v4[4])) + v6;
    v11 = v10 + v4[5] ^ v3 + v4[7] >> 9;
    v10 = v12 + v4[6] ^ v13 * 0x200;
    v7 = (v4[1] - v11) + v7;
    v14 = (v4[2] - v10) + v8;
    v3 = v7 >> 0x17 ^ v3 + v4[7] + v13;
    v9 = (v4[3] - v3) + v9;
    v6 = v14 * 0x8000 ^ v13 + v7;
    v13 = (v5 + v4[4]) - v6;
    *v4 = v6;
    v7 = v9 >> 0xe ^ v7 + v14;
    v11 -= v7;
    v4[1] = v7;
    v8 = v13 * 0x100000 ^ v14 + v9;
    v14 = v10 - v8;
    v4[2] = v8;
    v9 = v11 >> 0x11 ^ v9 + v13;
    v10 = v11 + v14;
    v3 -= v9;
    v4[3] = v9;
    v1 = &v4[8];
    v5 = v14 * 0x4000 ^ v13 + v11;
    v12 = v14 + v3;
    v4[5] = v10;
    v4[4] = v5;
    v4[6] = v12;
    v4[7] = v3;
    v4 = v1;
  } while (&a0[0x100] != v1);
  a0[0x102] = 0;
  a0[0x101] = 0;
  a0[0x100] = 0;
}

// Function: sub_11900 @ 0x11900
void sub_11900(long *a0)
{
  long v1;
  long v2;
  unsigned long v3; // rax
  unsigned long v4;
  long *v5;
  
  v1 = a0[6];
  v2 = a0[5];
  if (v1 == v2)
    *(unsigned char *)&a0[0xd] = *(unsigned char *)&a0[0xd] | 2;
  v3 = v1 + a0[9] & ~a0[9];
  v4 = a0[7];
  if (v3 - a0[4] <= (unsigned long)(a0[7] - a0[4]))
    v4 = v3;
  a0[6] = v4;
  a0[5] = v4;
  v5 = (long *)a0[0x11];
  if ((unsigned long)(a0[0x12] - (long)v5) <= 7) {
    _obstack_newchunk(&a0[0xe],8);
    v5 = (long *)a0[0x11];
  }
  *v5 = v2;
  v5 = (long *)a0[0x1c];
  a0[0x11] = a0[0x11] + 8;
  if ((unsigned long)(a0[0x1d] - (long)v5) <= 7) {
    _obstack_newchunk(&a0[0x19],8);
    v5 = (long *)a0[0x1c];
  }
  *v5 = (v1 - v2) + -1;
  a0[0x1c] = a0[0x1c] + 8;
  *a0 = *a0 + 1;
}

// Function: sub_119f0 @ 0x119f0
void sub_119f0(unsigned long *a0)
{
  unsigned long v1;
  unsigned long v2;
  
  v2 = dat_1cfe0;
  v1 = dat_1cfd0;
  *a0 = 0;
  a0[1] = 0;
  a0[2] = 0;
  _obstack_begin(&a0[3],0,0,dat_1cfd0,dat_1cfe0);
  _obstack_begin(&a0[0xe],0,0,v1,v2);
  _obstack_begin(&a0[0x19],0,0,v1,v2); // tail-call
}

// Function: sub_11a70 @ 0x11a70
void sub_11a70(long a0)
{
  _obstack_free(a0 + 0x18,0);
  _obstack_free(a0 + 0x70,0);
  _obstack_free(a0 + 200,0); // tail-call
}

// Function: sub_11aa0 @ 0x11aa0
unsigned long sub_11aa0(void *a0,long a1)
{
  int v1; // eax
  char *v2;
  long v3; // rax
  unsigned long v4; // rax
  unsigned long *v5;
  unsigned long v6;
  
label_11ace:
  do {
    v1 = fgetc(a0);
    v2 = *(char **)(a1 + 0x30);
    if (v1 == -1) {
      if (v2 != *(char **)(a1 + 0x28)) {
        if (v2 == *(char **)(a1 + 0x38)) {
          _obstack_newchunk(a1 + 0x18,1);
          v2 = *(char **)(a1 + 0x30);
        }
        *(char **)(a1 + 0x30) = &v2[1];
        *v2 = 0;
        sub_11900(a1);
      }
      v5 = *(unsigned long **)(a1 + 0x88);
      if ((unsigned long)(*(long *)(a1 + 0x90) - (long)v5) <= 7) {
        _obstack_newchunk(a1 + 0x70,8);
        v5 = *(unsigned long **)(a1 + 0x88);
      }
      *v5 = 0;
      v3 = *(long *)(a1 + 0x88) + 8;
      if (v3 == *(long *)(a1 + 0x80))
        *(unsigned char *)(a1 + 0xc0) = *(unsigned char *)(a1 + 0xc0) | 2;
      *(long *)(a1 + 8) = *(long *)(a1 + 0x80);
      v4 = v3 + *(unsigned long *)(a1 + 0xa0) & ~*(unsigned long *)(a1 + 0xa0);
      v6 = *(unsigned long *)(a1 + 0x90);
      if (v4 - *(long *)(a1 + 0x78) <= *(unsigned long *)(a1 + 0x90) - *(long *)(a1 + 0x78))
        v6 = v4;
      *(unsigned long *)(a1 + 0x88) = v6;
      *(unsigned long *)(a1 + 0x80) = v6;
      if (*(long *)(a1 + 0xe0) == *(long *)(a1 + 0xd8))
        *(unsigned char *)(a1 + 0x118) = *(unsigned char *)(a1 + 0x118) | 2;
      *(long *)(a1 + 0x10) = *(long *)(a1 + 0xd8);
      v4 = *(long *)(a1 + 0xe0) + *(unsigned long *)(a1 + 0xf8) & ~*(unsigned long *)(a1 + 0xf8);
      v6 = *(unsigned long *)(a1 + 0xe8);
      if (v4 - *(long *)(a1 + 0xd0) <= *(unsigned long *)(a1 + 0xe8) - *(long *)(a1 + 0xd0))
        v6 = v4;
      *(unsigned long *)(a1 + 0xe0) = v6;
      *(unsigned long *)(a1 + 0xd8) = v6;
      v1 = ferror(a0);
      return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
    }
    if (*(char **)(a1 + 0x38) != v2) goto label_11ac0;
    _obstack_newchunk(a1 + 0x18,1);
    v2 = *(char **)(a1 + 0x30);
    *(char **)(a1 + 0x30) = &v2[1];
    *v2 = (char)v1;
  } while (v1);
  goto label_11b10;
label_11ac0:
  *(char **)(a1 + 0x30) = &v2[1];
  *v2 = (char)v1;
  if (!v1) {
label_11b10:
    sub_11900(a1);
  }
  goto label_11ace;
}

// Function: sub_11c90 @ 0x11c90
unsigned long sub_11c90(int a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = setlocale(a0,NULL);
  if (!v1) {
    if (!a2)
      return 0x16;
    *a1 = 0;
    return 0x16;
  }
  v2 = strlen(v1);
  if (a2 <= v2) {
    if (!a2)
      return 0x22;
    memcpy(a1,v1,a2 - 1);
    a1[a2 - 1] = 0;
    return 0x22;
  }
  memcpy(a1,v1,v2 + 1);
  return 0;
}

// Function: sub_11d40 @ 0x11d40
void sub_11d40(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_11d50 @ 0x11d50
void sub_11d50(void)
{
  sub_12340(mkstemp()); // tail-call
}

// Function: sub_11d70 @ 0x11d70
void sub_11d70(unsigned long a0,unsigned int a1)
{
  sub_13ac0(mkostemp(),a1); // tail-call
}

// Function: sub_11d90 @ 0x11d90
unsigned int sub_11d90(unsigned char *a0,unsigned char *a1,unsigned int a2,unsigned int a3)
{
  unsigned char *v1;
  unsigned long v10; // rsi
  unsigned char v11;
  unsigned char v2;
  unsigned char *v3;
  unsigned int v4;
  unsigned char v5;
  unsigned int v6;
  unsigned long v7; // rsi
  unsigned int v8;
  unsigned int v9;
  
  v6 = (unsigned int)*a0;
  v5 = *a1;
  v8 = (unsigned int)v5;
  v11 = (unsigned char)a2;
  if (*a0 != 0x2d) {
    if (v5 != 0x2d) {
      while (((char)v6 == '0' || (a3 == v6))) {
        v3 = &a0[1];
        a0 = &a0[1];
        v6 = (unsigned int)*v3;
      }
      if (v5 != 0x30) goto label_11e1d;
      do {
        do {
          v3 = &a1[1];
          v8 = (unsigned int)*v3;
          a1 = &a1[1];
        } while (*v3 == 0x30);
label_11e1d:
        v4 = v8;
        v9 = v6;
      } while (a3 == v8);
label_11e30:
      do {
        v3 = a1;
        if (((char)v6 != (char)v4) || (10 <= v8 - 0x30)) goto label_12020;
        do {
          v1 = &a0[1];
          v6 = (unsigned int)*v1;
          a0 = &a0[1];
        } while (a3 == *v1);
        do {
          v8 = (unsigned int)v3[1];
          a1 = &v3[1];
          v4 = v8;
          v9 = v6;
          if (a3 != v8) goto label_11e30;
          v8 = (unsigned int)v3[2];
          v3 = &v3[2];
          a1 = v3;
          v4 = v8;
        } while (a3 == v8);
      } while( true );
    }
    do {
      do {
        v5 = a1[1];
        v8 = (unsigned int)v5;
        a1 = &a1[1];
      } while (v5 == 0x30);
    } while (a3 == v5);
    if (a2 == v5) {
      do {
        v8 = (unsigned int)a1[1];
        if (a1[1] != 0x30) break;
        v3 = &a1[2];
        v8 = (unsigned int)*v3;
        a1 = &a1[2];
      } while (*v3 == 0x30);
    }
    if (10 <= v8 - 0x30) {
      while (((char)v6 == '0' || (a3 == v6))) {
        v3 = &a0[1];
        a0 = &a0[1];
        v6 = (unsigned int)*v3;
      }
      if (a2 == v6) {
        do {
          v3 = &a0[1];
          a0 = &a0[1];
        } while (*v3 == 0x30);
        v6 = (unsigned int)*v3;
      }
      return (unsigned int)(v6 - 0x30 <= 9);
    }
    return 1;
  }
  do {
    do {
      v2 = a0[1];
      a0 = &a0[1];
    } while (v2 == 0x30);
    v6 = (unsigned int)v2;
  } while (a3 == v2);
  if (v5 != 0x2d) {
    if (a2 == v2) {
      do {
        v3 = &a0[1];
        a0 = &a0[1];
      } while (*v3 == 0x30);
      v6 = (unsigned int)*v3;
    }
    if (10 <= v6 - 0x30) {
      while (((char)v8 == '0' || (a3 == v8))) {
        v3 = &a1[1];
        a1 = &a1[1];
        v8 = (unsigned int)*v3;
      }
      if (a2 == v8) {
        do {
          v3 = &a1[1];
          a1 = &a1[1];
        } while (*v3 == 0x30);
        v8 = (unsigned int)*v3;
      }
      return -(unsigned int)(v8 - 0x30 < 10);
    }
    return 0xffffffff;
  }
  do {
    do {
      v5 = a1[1];
      a1 = &a1[1];
    } while (v5 == 0x30);
    v9 = (unsigned int)v5;
    v8 = (unsigned int)v5;
    v4 = (unsigned int)v2;
  } while (a3 == v5);
label_11f20:
  do {
    v3 = a1;
    if (((char)v4 != (char)v8) || (9 < v9 - 0x30)) goto label_12122;
    do {
      v1 = &a0[1];
      v6 = (unsigned int)*v1;
      a0 = &a0[1];
    } while (a3 == *v1);
    do {
      v9 = (unsigned int)v3[1];
      a1 = &v3[1];
      v8 = v9;
      v4 = v6;
      if (a3 != v9) goto label_11f20;
      v9 = (unsigned int)v3[2];
      v3 = &v3[2];
      a1 = v3;
      v8 = v9;
    } while (a3 == v9);
  } while( true );
label_12020:
  if ((a2 != v9) || (v8 - 0x30 <= 9)) {
    if (a2 != v8) { // branch-flip
      a2 = v9 - v8;
      if (10 <= v9 - 0x30) {
        v10 = 0;
        if (10 <= v8 - 0x30)
          return 0;
        goto label_1206e;
      }
    }
    else {
      a2 = v9 - a2;
      if (10 <= v9 - 0x30) goto label_120d4;
    }
    v10 = 0;
    do {
      do {
        v1 = &a0[1];
        a0 = &a0[1];
      } while (a3 == *v1);
      v10 += 1;
    } while ((unsigned int)*v1 - 0x30 <= 9);
    if (10 <= v8 - 0x30)
      return (unsigned int)(v10 != 0);
label_1206e:
    v7 = 0;
    do {
      do {
        v1 = &v3[1];
        v3 = &v3[1];
      } while (a3 == *v1);
      v7 += 1;
    } while ((unsigned int)*v1 - 0x30 <= 9);
    if (v7 == v10) {
      if (!v7)
        a2 = 0;
      return a2;
    }
    return -(unsigned int)(v10 < v7) | 1;
  }
label_120d4:
  if (v11 == *a0) {
    if (v11 != *v3) { // branch-flip
      v8 = (unsigned int)a0[1];
      a0 = &a0[1];
    }
    else {
      while( true ) {
        v1 = &a0[1];
        v8 = (unsigned int)(char)*v1;
        v5 = v3[1];
        a0 = &a0[1];
        v3 = &v3[1];
        if (*v1 != v5) break;
        if (10 <= v8 - 0x30)
          return 0;
      }
      v6 = (int)(char)v5 - 0x30;
      if (10 <= v8 - 0x30) {
        if (10 <= v6)
          return 0;
        goto label_120fe;
      }
      if (v6 <= 9)
        return v8 - (int)(char)v5;
    }
    while (v5 = (unsigned char)v8, v5 == 0x30) {
      v3 = &a0[1];
      a0 = &a0[1];
      v8 = (unsigned int)*v3;
    }
    return (unsigned int)((unsigned int)((int)(char)v5 - 0x30U) <= 9); // return-dupe
  }
  if (v11 != *v3)
    return 0;
  v5 = v3[1];
  v3 = &v3[1];
label_120fe:
  while (v5 == 0x30) {
    v1 = &v3[1];
    v3 = &v3[1];
    v5 = *v1;
  }
  return -(unsigned int)((unsigned int)((int)(char)v5 - 0x30U) < 10); // return-dupe
label_12122:
  if ((a2 == v6) && (10 <= v9 - 0x30)) {
label_121f1:
    if (v11 == *v3) {
      if (v11 == *a0) {
        do {
          v5 = v3[1];
          v11 = a0[1];
          v3 = &v3[1];
          a0 = &a0[1];
          v8 = (int)(char)v5 - 0x30;
          if (v5 != v11) {
            v6 = (int)(char)v11 - 0x30;
            if (10 <= v8) {
              if (v6 <= 9) goto label_1220d;
              return 0;
            }
            if (10 <= v6) goto label_122bb;
            return (int)(char)v5 - (int)(char)v11;
          }
        } while (v8 <= 9);
        return 0;
      }
      do {
        v3 = &v3[1];
label_122bb:
        v5 = *v3;
      } while (v5 == 0x30);
      return (unsigned int)((unsigned int)((int)(char)v5 - 0x30U) <= 9);
    }
    if (v11 != *a0)
      return 0;
    do {
      a0 = &a0[1];
label_1220d:
      v5 = *a0;
    } while (v5 == 0x30);
    return -(unsigned int)((unsigned int)((int)(char)v5 - 0x30U) < 10);
  }
  if (a2 != v9) { // branch-flip
    a2 = v9 - v6;
    if (10 <= v6 - 0x30) {
      v10 = 0;
      if (10 <= v9 - 0x30)
        return 0;
      goto label_12172;
    }
  }
  else {
    a2 -= v6;
    if (10 <= v6 - 0x30) goto label_121f1;
  }
  v10 = 0;
  do {
    do {
      v1 = &a0[1];
      a0 = &a0[1];
    } while (a3 == *v1);
    v10 += 1;
  } while ((unsigned int)*v1 - 0x30 <= 9);
  if (9 < v9 - 0x30)
    return -(unsigned int)(v10 != 0);
label_12172:
  v7 = 0;
  do {
    do {
      v1 = &v3[1];
      v3 = &v3[1];
    } while (a3 == *v1);
    v7 += 1;
  } while ((unsigned int)*v1 - 0x30 <= 9);
  if (v7 == v10) {
    if (!v7)
      return 0;
    return a2;
  }
  return (-(unsigned int)(v10 < v7) & 2) - 1;
}

// Function: sub_12340 @ 0x12340
unsigned int sub_12340(unsigned int a0)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  
  if (3 <= a0)
    return a0;
  v2 = sub_142a0();
  v3 = __errno_location();
  v1 = *v3;
  close(a0);
  *v3 = v1;
  return v2;
}

// Function: sub_123a0 @ 0x123a0
long sub_123a0(void *a0,long a1,unsigned long a2,unsigned long a3,unsigned long *a4,unsigned long a5)
{
  long v1;
  unsigned long v10; // stack - 0x48
  char *v2; // rax
  unsigned long v3;
  unsigned long v4;
  unsigned long v5;
  unsigned long v6;
  unsigned long v7; // stack - 0x60
  unsigned long v8; // stack - 0x58
  unsigned long v9; // stack - 0x50
  
  if (a1) // branch-flip
    __fprintf_chk(a0,1,"%s (%s) %s\n",a1,a2,a3);
  else {
    __fprintf_chk(a0,1,"%s %s\n",a2,a3);
  }
  __fprintf_chk(a0,1,"Copyright %s %d Free Software Foundation, Inc.",dcgettext(NULL,"(C)",5),0x7e6);
  fputs_unlocked("\n",a0);
  __fprintf_chk(a0,1,dcgettext(NULL,"License GPLv3+: GNU GPL version 3 or later <%s>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n",5),"https://gnu.org/licenses/gpl.html");
  fputs_unlocked("\n",a0);
  if (10 <= a5) {
    v10 = a4[7];
    v2 = "Written by %s, %s, %s,\n%s, %s, %s, %s,\n%s, %s, and others.\n";
    v9 = a4[6];
    v8 = a4[5];
    v7 = a4[4];
    v6 = a4[3];
    v5 = a4[2];
    v3 = a4[1];
    v4 = *a4;
    v2 = dcgettext(NULL,v2,5); // return-dupe
label_124cd:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_12506:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0x179a8) + 0x179a8;
  switch(a5) {
    case 0:
      goto label_12506;
    case 1:
      v6 = *a4;
      v2 = dcgettext(NULL,"Written by %s.\n",5);
      return __fprintf_chk(a0,1,v2,v6); // tail-call
    case 2:
      v6 = a4[1];
      v5 = *a4;
      v2 = dcgettext(NULL,"Written by %s and %s.\n",5);
      return __fprintf_chk(a0,1,v2,v5,v6); // tail-call
    case 3:
      v6 = a4[2];
      v5 = a4[1];
      v3 = *a4;
      v2 = dcgettext(NULL,"Written by %s, %s, and %s.\n",5);
      return __fprintf_chk(a0,1,v2,v3,v5,v6); // tail-call
    case 4:
      v1 = a4[3];
      v6 = a4[2];
      v5 = a4[1];
      v3 = *a4;
      v2 = dcgettext(NULL,"Written by %s, %s, %s,\nand %s.\n",5);
      break;
    case 5:
      v1 = a4[3];
      v6 = a4[2];
      v5 = a4[1];
      v3 = *a4;
      v2 = dcgettext(NULL,"Written by %s, %s, %s,\n%s, and %s.\n",5);
      break;
    case 6:
      v8 = a4[5];
      v7 = a4[4];
      v6 = a4[3];
      v5 = a4[2];
      v3 = a4[1];
      v4 = *a4;
      v2 = dcgettext(NULL,"Written by %s, %s, %s,\n%s, %s, and %s.\n",5);
      return __fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8); // return-dupe
    case 7:
      v8 = a4[5];
      v7 = a4[4];
      v6 = a4[3];
      v5 = a4[2];
      v3 = a4[1];
      v4 = *a4;
      v2 = dcgettext(NULL,"Written by %s, %s, %s,\n%s, %s, %s, and %s.\n",5);
      return __fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8);
    case 8:
      v10 = a4[7];
      v9 = a4[6];
      v8 = a4[5];
      v7 = a4[4];
      v6 = a4[3];
      v5 = a4[2];
      v3 = a4[1];
      v4 = *a4;
      v2 = dcgettext(NULL,"Written by %s, %s, %s,\n%s, %s, %s, %s,\nand %s.\n",5);
      goto label_124cd;
    case 9:
      v10 = a4[7];
      v2 = "Written by %s, %s, %s,\n%s, %s, %s, %s,\n%s, and %s.\n";
      v9 = a4[6];
      v8 = a4[5];
      v7 = a4[4];
      v6 = a4[3];
      v5 = a4[2];
      v3 = a4[1];
      v4 = *a4;
      v2 = dcgettext(NULL,v2,5);
      v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
      return v1;
    
  }
  __fprintf_chk(a0,1,v2,v3,v5,v6);
  return v1;
}

// Function: sub_12820 @ 0x12820
void sub_12820(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_123a0(); // tail-call
}

// Function: sub_12840 @ 0x12840
void sub_12840(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
{
  unsigned int v1;
  long *v2;
  long v3;
  long v4 [11];
  long v5; // r9
  
  v5 = 0;
  do {
    v1 = *a4;
    if (0x30 <= v1) { // branch-flip
      v2 = *(long **)&a4[2];
      *(long **)&a4[2] = &v2[1];
      v3 = *v2;
      v4[v5] = v3;
    }
    else {
      *a4 = v1 + 8;
      v3 = *(long *)((unsigned long)v1 + *(long *)&a4[4]);
      v4[v5] = v3;
    }
  } while ((v3) && (v5 = v5 + 1, v5 != 10));
  sub_123a0(a0,a1,a2,a3);
}

// Function: sub_128e0 @ 0x128e0
void sub_128e0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5)
{
  long v1;
  unsigned long v2;
  long v3 [4];
  long v4 [11];
  long *v5;
  long *v6; // rsi
  long v7; // r9
  long v8 [2]; // stack - 0x18
  
  v2 = 0x20;
  v8[0] = a4;
  v5 = (long *)&Stack0000000000000008;
  v8[1] = a5;
  v7 = 0;
  do {
    if (0x30 <= (unsigned int)v2) { // branch-flip
      v6 = &v5[1];
      v1 = *v5;
      v4[v7] = v1;
      v5 = v6;
    }
    else {
      v1 = *(long *)((long)v3 + v2);
      v4[v7] = v1;
      v2 = (unsigned long)((unsigned int)v2 + 8);
    }
  } while ((v1) && (v7 = v7 + 1, v7 != 10));
  sub_123a0(a0,a1,a2);
}

// Function: sub_129c0 @ 0x129c0
void sub_129c0(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-coreutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_12a70 @ 0x12a70
void sub_12a70(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_12fb0(); // no-return
}

// Function: sub_12ab0 @ 0x12ab0
void sub_12ab0(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_12fb0(); // no-return
}

// Function: sub_12ad0 @ 0x12ad0
void sub_12ad0(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_12fb0(); // no-return
}

// Function: sub_12af0 @ 0x12af0
void sub_12af0(void)
{
  sub_12ab0(); // tail-call
}

// Function: sub_12b00 @ 0x12b00
void sub_12b00(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_12fb0(); // no-return
}

// Function: sub_12b40 @ 0x12b40
void sub_12b40(void *a0,unsigned long a1)
{
  if (realloc(a0,a1 | a1 == 0))
    return;
  sub_12fb0(); // no-return
}

// Function: sub_12b70 @ 0x12b70
void sub_12b70(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_12fb0(); // no-return
}

// Function: sub_12bb0 @ 0x12bb0
void sub_12bb0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!a1) || (!a2)) {
    a1 = 1;
    a2 = 1;
  }
  if (reallocarray(a0,a1,a2))
    return;
  sub_12fb0(); // no-return
}

// Function: sub_12bf0 @ 0x12bf0
void sub_12bf0(unsigned long a0,unsigned long a1)
{
  if (reallocarray(NULL,a0,a1))
    return;
  sub_12fb0(); // no-return
}

// Function: sub_12c20 @ 0x12c20
void sub_12c20(unsigned long a0,unsigned long a1)
{
  if ((!a0) || (!a1)) {
    a0 = 1;
    a1 = 1;
  }
  if (!reallocarray(NULL,a0,a1))
    sub_12fb0(); // no-return
}

// Function: sub_12c70 @ 0x12c70
void sub_12c70(void *a0,unsigned long *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rbx
  unsigned long v3;
  
  v3 = *a1;
  if (a0) { // branch-flip
    v1 = (v3 >> 1) + 1;
    v2 = v3 + v1;
    if (!CARRY8(v3,v1)) {
      if ((reallocarray(a0,v2,1)) || (!v2)) {
        *a1 = v2;
        return;
      }
    }
  }
  else {
    if (!v3)
      v3 = 0x80;
    if (reallocarray(NULL,v3,1)) {
      *a1 = v3;
      return;
    }
  }
  sub_12fb0(); // no-return
}

// Function: sub_12d00 @ 0x12d00
void sub_12d00(void *a0,unsigned long *a1,unsigned long a2)
{
  char v1 [16];
  unsigned long v2; // rax
  unsigned long v3; // rbx
  unsigned long v4;
  
  v4 = *a1;
  if (a0) { // branch-flip
    v2 = (v4 >> 1) + 1;
    v3 = v4 + v2;
    if (!CARRY8(v4,v2)) {
      if (((reallocarray(a0,v3,a2)) || (!v3)) || (!a2)) {
        *a1 = v3;
        return;
      }
    }
  }
  else {
    if (!v4) {
      v1._8_8_ = 0;
      v1._0_8_ = a2;
      v4 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0) + (unsigned long)(0x81 <= a2);
    }
    if (reallocarray(NULL,v4,a2)) {
      *a1 = v4;
      return;
    }
  }
  sub_12fb0(); // no-return
}

// Function: sub_12d90 @ 0x12d90
void sub_12d90(void *a0,long *a1,long a2,long a3,long a4)
{
  long v1;
  long v2;
  unsigned long v3;
  long v4;
  
  v1 = *a1;
  v2 = (v1 >> 1) + v1;
  if (SCARRY8(v1 >> 1,v1))
    v2 = 0x7fffffffffffffff;
  v4 = a3;
  if (v2 <= a3)
    v4 = v2;
  if (0 <= a3)
    v2 = v4;
  v3 = v2 * a4;
  if (SEXT816((long)v3) != SEXT816(v2) * SEXT816(a4)) // branch-flip
    v4 = 0x7fffffffffffffff;
  else {
    if (0x80 <= (long)v3) goto label_12e4e;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_12e4e:
  if (!a0)
    *a1 = 0;
  if (((a2 <= v2 - v1) || ((v2 = v1 + a2, !SCARRY8(v1,a2) && (((v2 <= a3 || (a3 <= -1)) && (v3 = v2 * a4, SEXT816((long)v3) == SEXT816(v2) * SEXT816(a4))))))) && ((realloc(a0,v3) || ((a0 && (!v3)))))) {
    *a1 = v2;
    return;
  }
  sub_12fb0(); // no-return
}

// Function: sub_12e90 @ 0x12e90
void sub_12e90(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_12fb0(); // no-return
}

// Function: sub_12eb0 @ 0x12eb0
void sub_12eb0(unsigned long a0)
{
  sub_12e90(a0,1); // tail-call
}

// Function: sub_12ec0 @ 0x12ec0
void sub_12ec0(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_12fb0(); // no-return
}

// Function: sub_12ee0 @ 0x12ee0
void sub_12ee0(unsigned long a0)
{
  sub_12ec0(a0,1); // tail-call
}

// Function: sub_12ef0 @ 0x12ef0
void sub_12ef0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_12ab0(a1),a0,a1); // tail-call
}

// Function: sub_12f20 @ 0x12f20
void sub_12f20(void *a0,unsigned long a1)
{
  memcpy((void *)sub_12ad0(a1),a0,a1); // tail-call
}

// Function: sub_12f50 @ 0x12f50
void sub_12f50(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_12ad0(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_12f90 @ 0x12f90
void sub_12f90(char *a0)
{
  sub_12ef0(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_12fb0 @ 0x12fb0
void sub_12fb0(void)
{
  error(dat_1d130,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_12ff0 @ 0x12ff0
void sub_12ff0(int a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  
  error(0,a0,dcgettext(NULL,"string comparison failed",5));
  error(0,0,dcgettext(NULL,"Set LC_ALL=\'C\' to work around the problem.",5));
  v1 = sub_10850(1,8,a3,a4);
  v2 = sub_10850(0,8,a1,a2);
  error(dat_1d130,0,dcgettext(NULL,"The strings compared were %s and %s.",5),v2,v1); // tail-call
}

// Function: sub_130c0 @ 0x130c0
unsigned int sub_130c0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  int v1;
  unsigned int v2; // eax
  
  v2 = sub_13d60(a0,a1,a2,a3);
  v1 = *__errno_location();
  if (!v1)
    return v2;
  sub_12ff0(v1,a0,a1,a2,a3);
  return v2;
}

// Function: sub_13120 @ 0x13120
unsigned int sub_13120(unsigned long a0,long a1,unsigned long a2,long a3)
{
  int v1;
  unsigned int v2; // eax
  
  v2 = sub_13e10(a0,a1,a2,a3);
  v1 = *__errno_location();
  if (!v1)
    return v2;
  sub_12ff0(v1,a0,a1 + -1,a2,a3 + -1);
  return v2;
}

// Function: sub_13180 @ 0x13180
unsigned long sub_13180(double a0)
{
  unsigned int *v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x38
  char v4 [16];
  unsigned long v5; // stack - 0x30
  
  v1 = (unsigned int *)__errno_location();
  if (dat_17100 <= a0) {
    do {
      pause();
      if (*v1 != 4) break;
      pause();
    } while (*v1 == 4);
  }
  v4 = sub_13790();
  v5 = SUB168(v4,8);
  v3 = SUB168(v4,0);
  do {
    *v1 = 0;
    v2 = sub_13e80(&v3,&v3);
    if (!(int)v2)
      return v2;
  } while (!(*v1 & 0xfffffffb));
  return 0xffffffff;
}

// Function: sub_13230 @ 0x13230
void sub_13230(unsigned int a0,int a1,char a2,long a3,unsigned long a4)
{
  int v1;
  char v2; // stack - 0x32
  char *v3; // r9
  char *v4; // r12
  char *v5;
  long v6; // fs_offset
  char v7; // stack - 0x31
  unsigned long v8; // stack - 0x30
  
  v1 = dat_1d130;
  v8 = *(unsigned long *)(v6 + 0x28);
  if (4 <= a0) { // branch-flip
    if (a0 != 4) {
      abort(); // no-return, return-dupe
    }
    v3 = "invalid %s%s argument \'%s\'";
    if (a1 <= -1) goto label_132c3;
label_13271:
    v4 = "--";
    v5 = *(char **)(a3 + (long)a1 * 0x20);
  }
  else {
    if (2 <= a0) // branch-flip
      v3 = "invalid suffix in %s%s argument \'%s\'";
    else {
      if (a0 != 1) {
        abort();
      }
      v3 = "%s%s argument \'%s\' too large";
    }
    if (0 <= a1) goto label_13271;
label_132c3:
    v5 = &v2;
    v7 = 0;
    v4 = &"--"[-(long)a1];
    v2 = a2;
  }
  error(v1,0,dcgettext(NULL,v3,5),v4,v5,a4);
  abort();
}

// Function: sub_132f0 @ 0x132f0
unsigned int sub_132f0(unsigned char *a0,long *a1,unsigned int a2,unsigned long *a3,char *a4)
{
  unsigned char v1;
  char v10 [16];
  char v11 [16];
  char v12 [16];
  char v13 [16];
  char v14 [16];
  char v15 [16];
  char v16 [16];
  char v17 [16];
  char v18 [16];
  char v19 [16];
  char v2 [16];
  char v20 [16];
  char v21 [16];
  unsigned int v22; // eax
  int *v23; // rax
  unsigned char *v24;
  unsigned long v25;
  unsigned long v26; // rcx
  unsigned char v27;
  unsigned long v28;
  long v29;
  char v3 [16];
  int v30; // esi
  int v31; // r9d
  unsigned int v32;
  char v4 [16];
  char v5 [16];
  char v6 [16];
  char v7 [16];
  char v8 [16];
  char v9 [16];
  
  if (0x25 <= a2)
    __assert_fail("0 <= strtol_base && strtol_base <= 36","lib/xstrtol.c",0x55,"xstrtoumax"); // no-return
  if (!a1)
    a1 = &v29;
  v23 = __errno_location();
  *v23 = 0;
  v27 = *a0;
  v24 = a0;
  while (*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)v27 * 2) & 0x20) {
    v27 = v24[1];
    v24 = &v24[1];
  }
  if (v27 == 0x2d)
    return 4;
  v25 = strtoumax(a0,a1,a2);
  v24 = (unsigned char *)*a1;
  if (v24 != a0) { // branch-flip
    if (*v23) { // branch-flip
      v32 = 1;
      if (*v23 != 0x22)
        return 4;
    }
    else {
      v32 = 0;
    }
    if (!a4) goto label_133ad;
    v27 = *v24;
    if (!v27) goto label_133ad;
    v28 = v25;
    if (!strchr(a4,(int)(char)v27)) goto label_133b5;
  }
  else {
    if ((!a4) || (v27 = *a0, !v27))
      return 4;
    v32 = 0;
    v28 = 1;
    if (!strchr(a4,(int)(char)v27))
      return 4;
  }
  v22 = (unsigned int)v27 - 0x45;
  v31 = 1;
  v26 = 0x400;
  if (((unsigned char)v22 <= 0x2f) && (0x814400308945U >> ((unsigned long)v22 & 0x3f) & 1)) {
    v26 = 0x400;
    v31 = 1;
    if (strchr(a4,0x30)) {
      v1 = v24[1];
      if (v1 != 0x44) { // branch-flip
        if (v1 != 0x69) { // branch-flip
          v31 = (v1 == 0x42) + 1;
          if (v1 == 0x42)
            v26 = 1000;
        }
        else {
          v31 = (v24[2] == 0x42) + 1 + (unsigned int)(v24[2] == 0x42);
        }
      }
      else {
        v31 = 2;
        v26 = 1000;
      }
    }
  }
  v25 = v28;
  switch((unsigned int)v27 - 0x42 & 0xff) {
    case 0:
      v25 = v28 << 10;
      if (v28 >> 0x36) {
        v32 = 1;
        v25 = 0xffffffffffffffff;
      }
      break;
    default:
label_133b5:
      *a3 = v28;
      return v32 | 2;
    case 3:
      v30 = 6;
      v22 = 0;
      do {
        v8._8_8_ = 0;
        v8._0_8_ = v28;
        v18._8_8_ = 0;
        v18._0_8_ = v26;
        v28 = SUB168(v8._0_16_ * v18._0_16_,0);
        if (SUB168(v8._0_16_ * v18._0_16_,8)) {
          v22 = 1;
          v28 = 0xffffffffffffffff;
        }
        v30 -= 1;
      } while (v30);
      goto label_135b4;
    case 5:
    case 0x25:
      v5._8_8_ = 0;
      v5._0_8_ = v28;
      v15._8_8_ = 0;
      v15._0_8_ = v26;
      if ((SUB168(v5._0_16_ * v15._0_16_,8)) || (v6._8_8_ = 0, v6._0_8_ = SUB168(v5._0_16_ * v15._0_16_,0), v16._8_8_ = 0, v16._0_8_ = v26, v25 = SUB168(v6._0_16_ * v16._0_16_,0), SUB168(v6._0_16_ * v16._0_16_,8))) goto label_13579;
label_13547:
      v4._8_8_ = 0;
      v4._0_8_ = v25;
      v14._8_8_ = 0;
      v14._0_8_ = v26;
      if (SUB168(v4._0_16_ * v14._0_16_,8)) { // branch-flip
        v32 = 1;
        v25 = 0xffffffffffffffff;
      }
      else {
        v25 = SUB168(v4._0_16_ * v14._0_16_,0);
      }
      break;
    case 9:
    case 0x29:
      v2._8_8_ = 0;
      v2._0_8_ = v28;
      v12._8_8_ = 0;
      v12._0_8_ = v26;
      v25 = SUB168(v2._0_16_ * v12._0_16_,0);
      if (SUB168(v2._0_16_ * v12._0_16_,8)) goto label_13579;
      break;
    case 0xb:
    case 0x2b:
      v3._8_8_ = 0;
      v3._0_8_ = v28;
      v13._8_8_ = 0;
      v13._0_8_ = v26;
      v25 = SUB168(v3._0_16_ * v13._0_16_,0);
      if (!SUB168(v3._0_16_ * v13._0_16_,8)) goto label_13547;
label_13579:
      v32 = 1;
      v25 = 0xffffffffffffffff;
      break;
    case 0xe:
      v30 = 5;
      v22 = 0;
      do {
        v9._8_8_ = 0;
        v9._0_8_ = v28;
        v19._8_8_ = 0;
        v19._0_8_ = v26;
        v28 = SUB168(v9._0_16_ * v19._0_16_,0);
        if (SUB168(v9._0_16_ * v19._0_16_,8)) {
          v22 = 1;
          v28 = 0xffffffffffffffff;
        }
        v30 -= 1;
      } while (v30);
      goto label_135b4;
    case 0x12:
    case 0x32:
      v30 = 4;
      v22 = 0;
      do {
        v7._8_8_ = 0;
        v7._0_8_ = v28;
        v17._8_8_ = 0;
        v17._0_8_ = v26;
        v28 = SUB168(v7._0_16_ * v17._0_16_,0);
        if (SUB168(v7._0_16_ * v17._0_16_,8)) {
          v22 = 1;
          v28 = 0xffffffffffffffff;
        }
        v30 -= 1;
      } while (v30);
      goto label_135b4;
    case 0x17:
      v30 = 8;
      v22 = 0;
      do {
        v11._8_8_ = 0;
        v11._0_8_ = v28;
        v21._8_8_ = 0;
        v21._0_8_ = v26;
        v28 = SUB168(v11._0_16_ * v21._0_16_,0);
        if (SUB168(v11._0_16_ * v21._0_16_,8)) {
          v22 = 1;
          v28 = 0xffffffffffffffff;
        }
        v30 -= 1;
      } while (v30);
      goto label_135b4;
    case 0x18:
      v30 = 7;
      v22 = 0;
      do {
        v10._8_8_ = 0;
        v10._0_8_ = v28;
        v20._8_8_ = 0;
        v20._0_8_ = v26;
        v28 = SUB168(v10._0_16_ * v20._0_16_,0);
        if (SUB168(v10._0_16_ * v20._0_16_,8)) {
          v22 = 1;
          v28 = 0xffffffffffffffff;
        }
        v30 -= 1;
      } while (v30);
label_135b4:
      v32 |= v22;
      v25 = v28;
      break;
    case 0x20:
      if (v28 >> 0x37) { // branch-flip
        v32 = 1;
        v25 = 0xffffffffffffffff;
      }
      else {
        v25 = v28 << 9;
      }
      break;
    case 0x21:
      break;
    case 0x35:
      if (0 <= (long)v28) // branch-flip
        v25 = v28 * 2;
      else {
        v32 = 1;
        v25 = 0xffffffffffffffff;
      }
    
  }
  *a1 = (long)&v24[v31];
  if (v24[v31])
    v32 |= 2;
label_133ad:
  *a3 = v25;
  return v32;
}

// Function: sub_13720 @ 0x13720
unsigned long sub_13720(unsigned int *a0) // early-return x2, return-dupe
{
  unsigned int v1;
  unsigned long v2; // rax
  unsigned long v3; // rax
  
  v2 = __fpending(a0);
  v1 = *a0;
  v3 = sub_c5b0(a0);
  if (v1 & 0x20) {
    if ((int)v3)
      return 0xffffffff;
    *__errno_location() = 0;
    return 0xffffffff;
  }
  if ((int)v3) {
    if (v2)
      return 0xffffffff;
    return (unsigned long)-(unsigned int)(*__errno_location() != 9);
  }
  return v3;
}

// Function: sub_13790 @ 0x13790
undefined16 sub_13790(double a0)
{
  char v1 [16];
  char v2 [16];
  long v3;
  long v4; // r8
  double v5; // xmm0_qa
  
  v4 = -0x8000000000000000;
  v3 = 0;
  if (dat_17c18 < a0) {
    v3 = 999999999;
    v4 = 0x7fffffffffffffff;
    if (a0 < dat_17100) {
      v5 = (a0 - (double)(long)a0) * dat_17c20;
      v3 = (long)v5;
      v3 = (unsigned long)((double)v3 < v5) + v3;
      v4 = (long)a0 + v3 / 1000000000;
      v3 %= 1000000000;
      if (v3 < 0) {
        v2._8_8_ = v3 + 1000000000;
        v2._0_8_ = v4 + -1;
        return v2._0_16_;
      }
    }
  }
  v1._8_8_ = v3;
  v1._0_8_ = v4;
  return v1._0_16_;
}

// Function: sub_13850 @ 0x13850
int sub_13850(int a0,int a1,unsigned long a2)
{
  int v1;
  int v2; // eax
  unsigned int v3; // eax
  unsigned long v4;
  int *v5; // rax
  unsigned long v6; // stack - 0x38
  
  if (!a1)
    return fcntl(a0,0,a2 & 0xffffffff);
  if (a1 == 0x406) {
    if (0 <= dat_1d9d8) { // branch-flip
      v2 = fcntl(a0,0x406,a2 & 0xffffffff);
      if ((0 <= v2) || (*__errno_location() != 0x16)) {
        dat_1d9d8 = 1;
        return v2;
      }
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      dat_1d9d8 = -1;
    }
    else {
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      if (dat_1d9d8 != -1)
        return v2;
    }
    v3 = fcntl(v2,1);
    if ((0 <= (int)v3) && (fcntl(v2,2,(unsigned long)(v3 | 1)) != -1))
      return v2;
    v5 = __errno_location();
    v1 = *v5;
    close(v2);
    *v5 = v1;
    return -1;
  }
  if (0xc <= a1) { // branch-flip
    if (0xb <= (unsigned int)(a1 - 0x400U)) {
      v2 = fcntl(a0,a1,v6); // return-dupe
      return v2;
    }
    v4 = 1L << ((unsigned char)(a1 - 0x400U) & 0x3f);
    if (v4 & 0x2c5) {
      return fcntl(a0,a1,(unsigned long)(unsigned int)v6); // return-dupe
    }
    v4 &= 0x502;
  }
  else {
    if (a1 <= 0) {
      v2 = fcntl(a0,a1,v6);
      return v2;
    }
    v4 = 1L << ((unsigned char)a1 & 0x3f);
    if (v4 & 0x514)
      return fcntl(a0,a1,(unsigned long)(unsigned int)v6);
    v4 &= 0xa0a;
  }
  if (v4)
    return fcntl(a0,a1);
  v2 = fcntl(a0,a1,v6);
  return v2;
}

// Function: sub_13ac0 @ 0x13ac0
unsigned int sub_13ac0(unsigned int a0)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  
  if (3 <= a0)
    return a0;
  v2 = sub_13b20();
  v3 = __errno_location();
  v1 = *v3;
  close(a0);
  *v3 = v1;
  return v2;
}

// Function: sub_13b20 @ 0x13b20
void sub_13b20(unsigned long a0,unsigned int a1)
{
  unsigned int v1;
  
  v1 = 0;
  if (a1 & 0x80000)
    v1 = 0x406;
  sub_13850(a0,v1,3); // tail-call
}

// Function: sub_13b40 @ 0x13b40
void * sub_13b40(char *a0,char *a1)
{
  int v1;
  unsigned int v2; // eax
  int v3;
  void *v4; // rax
  void *v5;
  int *v6; // rax
  
  v4 = fopen(a0,a1);
  v5 = v4;
  if ((v4) && (v2 = fileno(v4), v2 <= 2)) {
    v3 = sub_142a0(v2);
    if (0 <= v3) { // branch-flip
      if ((!sub_c5b0(v4)) && (v5 = fdopen(v3,a1), v5))
        return v5;
      v6 = __errno_location();
      v1 = *v6;
      close(v3);
      *v6 = v1;
      v5 = NULL;
    }
    else {
      v6 = __errno_location();
      v5 = NULL;
      v3 = *v6;
      sub_c5b0(v4);
      *v6 = v3;
    }
  }
  return v5;
}

// Function: sub_13be0 @ 0x13be0
char * sub_13be0(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_13c20 @ 0x13c20
unsigned long sub_13c20(unsigned int *a0,unsigned char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,(char *)a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_cc40(0))) {
    v1 = 1;
    *a0 = (unsigned int)*a1;
  }
  return v1;
}

// Function: sub_13cb0 @ 0x13cb0
int sub_13cb0(char *a0,long a1,char *a2,long a3)
{
  long v1;
  int v2; // eax
  int *v3; // rax
  long v4; // rax
  
  v3 = __errno_location();
  while( true ) {
    *v3 = 0;
    v2 = strcoll(a0,a2);
    if (v2)
      return v2;
    v1 = strlen(a0) + 1;
    a0 = &a0[v1];
    v4 = strlen(a2) + 1;
    a2 = &a2[v4];
    a3 -= v4;
    a1 -= v1;
    if (!a1) break;
    if (!a3)
      return 1;
  }
  return (unsigned long)-(unsigned int)(a3 != 0);
}

// Function: sub_13d60 @ 0x13d60
unsigned int sub_13d60(void *a0,unsigned long a1,void *a2,unsigned long a3)
{
  char *v1;
  char *v2;
  char v3;
  char v4;
  unsigned int v5; // eax
  
  if ((a1 == a3) && (!memcmp(a0,a2,a1))) {
    *__errno_location() = 0;
    return 0;
  }
  v1 = (char *)((long)a0 + a1);
  v2 = (char *)((long)a2 + a3);
  v3 = *v1;
  v4 = *v2;
  *v1 = 0;
  *v2 = 0;
  v5 = sub_13cb0(a0,a1 + 1,a2,a3 + 1);
  *v1 = v3;
  *v2 = v4;
  return v5;
}

// Function: sub_13e10 @ 0x13e10
unsigned long sub_13e10(void *a0,unsigned long a1,void *a2,unsigned long a3)
{
  if (a1 == a3) {
    if (!memcmp(a0,a2,a1)) {
      *__errno_location() = 0;
      return 0;
    }
  }
  return sub_13cb0(a0,a1,a2,a3); // tail-call
}

// Function: sub_13e80 @ 0x13e80
unsigned long sub_13e80(long *a0,long *a1) // early-return
{
  unsigned long v1; // rax
  long v2;
  long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  
  v4 = a0[1];
  if (1000000000 <= v4) {
    *__errno_location() = 0x16;
    return 0xffffffff;
  }
  v2 = *a0;
  while( true ) {
    if (v2 <= 0x1fa400) {
      v3 = v2;
      return nanosleep(&v3,a1);
    }
    v3 = 0x1fa400;
    v2 -= 0x1fa400;
    v1 = nanosleep(&v3,a1);
    if ((int)v1) break;
    v4 = 0;
  }
  if (!a1)
    return v1;
  *a1 = *a1 + v2;
  return v1;
}

// Function: sub_13f50 @ 0x13f50
void sub_13f50(void)
{
  __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"memory exhausted",5));
  exit(dat_1d130); // no-return
}

// Function: sub_13fa0 @ 0x13fa0
long * sub_13fa0(long *a0,long a1,long a2) // ternary
{
  long *v1; // rax
  unsigned long v2; // rdx
  long v3;
  
  if (a2) // branch-flip
    v3 = a2 + -1;
  else {
    v3 = 0xf;
    a2 = 0x10;
  }
  a0[6] = v3;
  if (!a1)
    a1 = 0xfe0;
  *a0 = a1;
  v1 = (*(unsigned char *)&a0[10] & 1) ? (long *)(*(void *)a0[7])(a0[9]) : (long *)(*(void *)a0[7])(a1); // branch-flip
  a0[1] = (long)v1;
  if (v1) {
    v2 = (long)v1 + v3 + 0x10 & -a2;
    a0[2] = v2;
    a0[3] = v2;
    v3 = *a0;
    *v1 = v3 + (long)v1;
    a0[4] = v3 + (long)v1;
    v1[1] = 0;
    v1 = (long *)0x1;
    *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] & 0xf9;
    return v1;
  }
  (*obstack_alloc_failed_handler)(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: _obstack_begin @ 0x14040
void _obstack_begin(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) & 0xfe;
  *(unsigned long *)(a0 + 0x38) = a3;
  *(unsigned long *)(a0 + 0x40) = a4;
  sub_13fa0(); // tail-call
}

// Function: _obstack_begin_1 @ 0x14060
void _obstack_begin_1(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5)
{
  *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 1;
  *(unsigned long *)(a0 + 0x38) = a3;
  *(unsigned long *)(a0 + 0x40) = a4;
  *(unsigned long *)(a0 + 0x48) = a5;
  sub_13fa0(); // tail-call
}

// Function: _obstack_newchunk @ 0x14080
void _obstack_newchunk(unsigned long *a0,unsigned long a1) // ternary
{
  unsigned long v1;
  unsigned long v2;
  void *v3;
  unsigned long *v4; // rax
  unsigned long v5;
  unsigned long v6; // r13
  void *v7; // r14
  
  v6 = a0[3] - a0[2];
  v2 = a0[1];
  v5 = a1 + v6 + a0[6];
  v1 = v5 + 100 + (v6 >> 3);
  if (v5 < *a0)
    v5 = *a0;
  if (v5 <= v1)
    v5 = v1;
  if ((!CARRY8(a1,v6)) && (!CARRY8(a1 + v6,a0[6]))) {
    v4 = (a0[10] & 1) ? (unsigned long *)(*(void *)a0[7])(a0[9]) : (unsigned long *)(*(void *)a0[7])(v5); // branch-flip
    if (v4) {
      a0[1] = (unsigned long)v4;
      v3 = (void *)a0[2];
      v4[1] = v2;
      a0[4] = (long)v4 + v5;
      *v4 = (long)v4 + v5;
      v7 = (void *)((long)v4 + a0[6] + 0x10 & ~a0[6]);
      memcpy(v7,v3,v6);
      v1 = a0[10];
      if ((!(v1 & 2)) && (a0[2] == (~a0[6] & v2 + 0x10 + a0[6]))) {
        v4[1] = *(unsigned long *)(v2 + 8);
        if (v1 & 1) // branch-flip
          (*(void *)a0[8])(a0[9],v2);
        else {
          (*(void *)a0[8])(v2);
        }
      }
      a0[2] = (unsigned long)v7;
      a0[3] = (long)v7 + v6;
      *(unsigned char *)&a0[10] = (unsigned char)a0[10] & 0xfd;
      return;
    }
  }
  (*obstack_alloc_failed_handler)(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: _obstack_allocated_p @ 0x141b0
unsigned long _obstack_allocated_p(long a0,unsigned long *a1)
{
  unsigned long *v1;
  
  v1 = *(unsigned long **)(a0 + 8);
  if (!v1)
    return 0;
  while ((a1 <= v1 || ((unsigned long *)*v1 < a1))) {
    v1 = (unsigned long *)v1[1];
    if (!v1)
      return 0;
  }
  return 1;
}

// Function: _obstack_free @ 0x141f0
void _obstack_free(long a0,unsigned long *a1) // return-dupe
{
  unsigned long *v1;
  unsigned long *v2;
  
  v2 = *(unsigned long **)(a0 + 8);
  if (*(unsigned long **)(a0 + 8)) {
    while( true ) {
      if ((v2 < a1) && (v1 = (unsigned long *)*v2, a1 <= v1)) {
        *(unsigned long **)(a0 + 0x18) = a1;
        *(unsigned long **)(a0 + 0x10) = a1;
        *(unsigned long **)(a0 + 0x20) = v1;
        *(unsigned long **)(a0 + 8) = v2;
        return;
      }
      v1 = (unsigned long *)v2[1];
      if (*(unsigned char *)(a0 + 0x50) & 1) { // branch-flip
        (**(void **)(a0 + 0x40))(*(unsigned long *)(a0 + 0x48));
        *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 2;
      }
      else {
        (**(void **)(a0 + 0x40))(v2);
        *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 2;
      }
      if (!v1) break;
      v2 = v1;
    }
  }
  if (!a1)
    return;
  abort(); // no-return
}

// Function: _obstack_memory_used @ 0x14270
long _obstack_memory_used(long a0)
{
  long *v1;
  long v2;
  
  v2 = 0;
  for (v1 = *(long **)(a0 + 8); v1; v1 = (long *)v1[1]) {
    v2 += *v1 - (long)v1;
  }
  return v2;
}

// Function: sub_142a0 @ 0x142a0
void sub_142a0(unsigned long a0)
{
  sub_13850(a0,0,3); // tail-call
}

// Function: sub_142c0 @ 0x142c0
void sub_142c0(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_1d008); // tail-call
}

// Function: _DT_FINI @ 0x142d4
void _DT_FINI(void)
{
  return;
}
