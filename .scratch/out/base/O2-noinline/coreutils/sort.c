// Function: _DT_INIT @ 0x3000
void _DT_INIT(void) // return-dupe
{
  if (!dat_1dfb0)
    return;
  (*dat_1dfb0)();
}

// Function: sub_3020 @ 0x3020
void sub_3020(void)
{
  (*dat_1dbe8)(); // jump-as-call
}

// Function: malloc @ 0x37b0
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1dfb8)(); // jump-as-call
  return v1;
}

// Function: free @ 0x37c0
void free(void *a0)
{
  (*dat_1dfc8)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x37d0
void __cxa_finalize(void)
{
  (*dat_1dfd8)(); // jump-as-call
}

// Function: putchar_unlocked @ 0x37e0
int putchar_unlocked(int a0)
{
  int v1; // eax
  
  v1 = (*dat_1dbf0)(); // jump-as-call
  return v1;
}

// Function: euidaccess @ 0x37f0
void euidaccess(void)
{
  (*dat_1dbf8)(); // jump-as-call
}

// Function: pipe2 @ 0x3800
void pipe2(void)
{
  (*dat_1dc00)(); // jump-as-call
}

// Function: fileno @ 0x3810
int fileno(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1dc08)(); // jump-as-call
  return v1;
}

// Function: pthread_join @ 0x3820
void pthread_join(void)
{
  (*dat_1dc10)(); // jump-as-call
}

// Function: strtoumax @ 0x3830
void strtoumax(void)
{
  (*dat_1dc18)(); // jump-as-call
}

// Function: dup2 @ 0x3840
int dup2(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_1dc20)(); // jump-as-call
  return v1;
}

// Function: pthread_cond_destroy @ 0x3850
void pthread_cond_destroy(void)
{
  (*dat_1dc28)(); // jump-as-call
}

// Function: __stpcpy_chk @ 0x3860
char * __stpcpy_chk(char *a0,char *a1,unsigned long a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1dc30)(); // jump-as-call
  return v1;
}

// Function: pause @ 0x3870
void pause(void)
{
  (*dat_1dc38)(); // jump-as-call
}

// Function: strcoll @ 0x3880
int strcoll(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_1dc40)(); // jump-as-call
  return v1;
}

// Function: strtold @ 0x3890
void strtold(void)
{
  (*dat_1dc48)(); // jump-as-call
}

// Function: fputc_unlocked @ 0x38a0
void fputc_unlocked(void)
{
  (*dat_1dc50)(); // jump-as-call
}

// Function: memset @ 0x38b0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1dc58)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x38c0
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1dc60)(); // jump-as-call
  return v1;
}

// Function: dcngettext @ 0x38d0
void dcngettext(void)
{
  (*dat_1dc68)(); // jump-as-call
}

// Function: pthread_sigmask @ 0x38e0
void pthread_sigmask(void)
{
  (*dat_1dc70)(); // jump-as-call
}

// Function: close @ 0x38f0
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_1dc78)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x3900
void abort(void)
{
  (*dat_1dc80)(); // jump-as-call
}

// Function: memchr @ 0x3910
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1dc88)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x3920
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1dc90)(); // jump-as-call
  return v1;
}

// Function: __fprintf_chk @ 0x3930
int __fprintf_chk(void *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_1dc98)(); // jump-as-call
  return v1;
}

// Function: pthread_cond_signal @ 0x3940
void pthread_cond_signal(void)
{
  (*dat_1dca0)(); // jump-as-call
}

// Function: textdomain @ 0x3950
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1dca8)(); // jump-as-call
  return v1;
}

// Function: strxfrm @ 0x3960
void strxfrm(void)
{
  (*dat_1dcb0)(); // jump-as-call
}

// Function: toupper @ 0x3970
int toupper(int a0)
{
  int v1; // eax
  
  v1 = (*dat_1dcb8)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x3980
void exit(int a0)
{
  (*dat_1dcc0)(); // jump-as-call
}

// Function: __assert_fail @ 0x3990
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_1dcc8)(); // jump-as-call
}

// Function: fstat @ 0x39a0
int fstat(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_1dcd0)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x39b0
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_1dcd8)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x39c0
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1dce0)(); // jump-as-call
  return v1;
}

// Function: MD5 @ 0x39d0
void MD5(void)
{
  (*dat_1dce8)(); // jump-as-call
}

// Function: setvbuf @ 0x39e0
int setvbuf(void *a0,char *a1,int a2,unsigned long a3)
{
  int v1; // eax
  
  v1 = (*dat_1dcf0)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x39f0
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1dcf8)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x3a00
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_1dd00)(); // jump-as-call
  return v1;
}

// Function: fopen @ 0x3a10
void * fopen(char *a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1dd08)(); // jump-as-call
  return v1;
}

// Function: execlp @ 0x3a20
int execlp(char *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_1dd10)(); // jump-as-call
  return v1;
}

// Function: unlink @ 0x3a30
int unlink(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_1dd18)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x3a40
void _exit(int a0)
{
  (*dat_1dd20)(); // jump-as-call
}

// Function: sigismember @ 0x3a50
void sigismember(void)
{
  (*dat_1dd28)(); // jump-as-call
}

// Function: feof_unlocked @ 0x3a60
void feof_unlocked(void)
{
  (*dat_1dd30)(); // jump-as-call
}

// Function: __cxa_atexit @ 0x3a70
void __cxa_atexit(void)
{
  (*dat_1dd38)(); // jump-as-call
}

// Function: putc_unlocked @ 0x3a80
int putc_unlocked(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_1dd40)(); // jump-as-call
  return v1;
}

// Function: sysconf @ 0x3a90
long sysconf(int a0)
{
  long v1; // rax
  
  v1 = (*dat_1dd48)(); // jump-as-call
  return v1;
}

// Function: pthread_mutex_init @ 0x3aa0
void pthread_mutex_init(void)
{
  (*dat_1dd50)(); // jump-as-call
}

// Function: clearerr_unlocked @ 0x3ab0
void clearerr_unlocked(void *a0)
{
  (*dat_1dd58)(); // jump-as-call
}

// Function: pthread_create @ 0x3ac0
void pthread_create(void)
{
  (*dat_1dd60)(); // jump-as-call
}

// Function: fread_unlocked @ 0x3ad0
void fread_unlocked(void)
{
  (*dat_1dd68)(); // jump-as-call
}

// Function: strlen @ 0x3ae0
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1dd70)(); // jump-as-call
  return v1;
}

// Function: ferror @ 0x3af0
int ferror(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1dd78)(); // jump-as-call
  return v1;
}

// Function: pthread_cond_init @ 0x3b00
void pthread_cond_init(void)
{
  (*dat_1dd80)(); // jump-as-call
}

// Function: __ctype_get_mb_cur_max @ 0x3b10
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1dd88)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x3b20
int fputs_unlocked(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_1dd90)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x3b30
int __freading(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1dd98)(); // jump-as-call
  return v1;
}

// Function: MD5_Final @ 0x3b40
void MD5_Final(void)
{
  (*dat_1dda0)(); // jump-as-call
}

// Function: __ctype_b_loc @ 0x3b50
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1dda8)(); // jump-as-call
  return v1;
}

// Function: fgetc @ 0x3b60
void fgetc(void)
{
  (*dat_1ddb0)(); // jump-as-call
}

// Function: fdopen @ 0x3b70
void * fdopen(int a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1ddb8)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x3b80
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1ddc0)(); // jump-as-call
  return v1;
}

// Function: getrandom @ 0x3b90
void getrandom(void)
{
  (*dat_1ddc8)(); // jump-as-call
}

// Function: pipe @ 0x3ba0
int pipe(int *a0)
{
  int v1; // eax
  
  v1 = (*dat_1ddd0)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x3bb0
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1ddd8)(); // jump-as-call
  return v1;
}

// Function: __fpending @ 0x3bc0
unsigned long __fpending(void *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1dde0)(); // jump-as-call
  return v1;
}

// Function: sigaction @ 0x3bd0
int sigaction(int a0,void *a1,void *a2)
{
  int v1; // eax
  
  v1 = (*dat_1dde8)(); // jump-as-call
  return v1;
}

// Function: __explicit_bzero_chk @ 0x3be0
void __explicit_bzero_chk(void *a0,unsigned long a1,unsigned long a2)
{
  (*dat_1ddf0)(); // jump-as-call
}

// Function: posix_fadvise @ 0x3bf0
void posix_fadvise(void)
{
  (*dat_1ddf8)(); // jump-as-call
}

// Function: fputs @ 0x3c00
int fputs(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_1de00)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x3c10
void lseek(void)
{
  (*dat_1de08)(); // jump-as-call
}

// Function: strtol @ 0x3c20
long strtol(char *a0,char **a1,int a2)
{
  long v1; // rax
  
  v1 = (*dat_1de10)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x3c30
void iswcntrl(void)
{
  (*dat_1de18)(); // jump-as-call
}

// Function: sched_getaffinity @ 0x3c40
void sched_getaffinity(void)
{
  (*dat_1de20)(); // jump-as-call
}

// Function: mkostemp @ 0x3c50
void mkostemp(void)
{
  (*dat_1de28)(); // jump-as-call
}

// Function: raise @ 0x3c60
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_1de30)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x3c70
void signal(void)
{
  (*dat_1de38)(); // jump-as-call
}

// Function: memmove @ 0x3c80
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1de40)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x3c90
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1de48)(); // jump-as-call
  return v1;
}

// Function: waitpid @ 0x3ca0
int waitpid(int a0,int *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_1de50)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x3cb0
int ferror_unlocked(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1de58)(); // jump-as-call
  return v1;
}

// Function: getopt_long @ 0x3cc0
int getopt_long(int a0,char **a1,char *a2,void *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_1de60)(); // jump-as-call
  return v1;
}

// Function: getenv @ 0x3cd0
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1de68)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x3ce0
int mbsinit(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1de70)(); // jump-as-call
  return v1;
}

// Function: __errno_location @ 0x3cf0
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_1de78)(); // jump-as-call
  return v1;
}

// Function: qsort @ 0x3d00
void qsort(void)
{
  (*dat_1de80)(); // jump-as-call
}

// Function: mkstemp @ 0x3d10
void mkstemp(void)
{
  (*dat_1de88)(); // jump-as-call
}

// Function: dcgettext @ 0x3d20
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1de90)(); // jump-as-call
  return v1;
}

// Function: sigaddset @ 0x3d30
int sigaddset(void *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_1de98)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x3d40
void __stack_chk_fail(void)
{
  (*dat_1dea0)(); // jump-as-call
}

// Function: strcmp @ 0x3d50
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_1dea8)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x3d60
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_1deb0)(); // jump-as-call
  return v1;
}

// Function: MD5_Init @ 0x3d70
void MD5_Init(void)
{
  (*dat_1deb8)(); // jump-as-call
}

// Function: localeconv @ 0x3d80
void localeconv(void)
{
  (*dat_1dec0)(); // jump-as-call
}

// Function: strcpy @ 0x3d90
char * strcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1dec8)(); // jump-as-call
  return v1;
}

// Function: nanosleep @ 0x3da0
void nanosleep(void)
{
  (*dat_1ded0)(); // jump-as-call
}

// Function: pthread_cond_wait @ 0x3db0
void pthread_cond_wait(void)
{
  (*dat_1ded8)(); // jump-as-call
}

// Function: MD5_Update @ 0x3dc0
void MD5_Update(void)
{
  (*dat_1dee0)(); // jump-as-call
}

// Function: memcmp @ 0x3dd0
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_1dee8)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x3de0
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1def0)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x3df0
int fclose(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1def8)(); // jump-as-call
  return v1;
}

// Function: getrlimit @ 0x3e00
void getrlimit(void)
{
  (*dat_1df00)(); // jump-as-call
}

// Function: __sched_cpucount @ 0x3e10
void __sched_cpucount(void)
{
  (*dat_1df08)(); // jump-as-call
}

// Function: fseeko @ 0x3e20
void fseeko(void)
{
  (*dat_1df10)(); // jump-as-call
}

// Function: ftruncate @ 0x3e30
void ftruncate(void)
{
  (*dat_1df18)(); // jump-as-call
}

// Function: fork @ 0x3e40
int fork(void)
{
  int v1; // eax
  
  v1 = (*dat_1df20)(); // jump-as-call
  return v1;
}

// Function: sigemptyset @ 0x3e50
int sigemptyset(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1df28)(); // jump-as-call
  return v1;
}

// Function: pthread_mutex_lock @ 0x3e60
int pthread_mutex_lock(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1df30)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x3e70
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1df38)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x3e80
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_1df40)(); // jump-as-call
  return v1;
}

// Function: error @ 0x3e90
void error(int a0,int a1,char *a2,...)
{
  (*dat_1df48)(); // jump-as-call
}

// Function: pthread_mutex_destroy @ 0x3ea0
void pthread_mutex_destroy(void)
{
  (*dat_1df50)(); // jump-as-call
}

// Function: write @ 0x3eb0
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_1df58)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x3ec0
int stat(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_1df60)(); // jump-as-call
  return v1;
}

// Function: strtoul @ 0x3ed0
unsigned long strtoul(char *a0,char **a1,int a2)
{
  unsigned long v1; // rax
  
  v1 = (*dat_1df68)(); // jump-as-call
  return v1;
}

// Function: sysinfo @ 0x3ee0
void sysinfo(void)
{
  (*dat_1df70)(); // jump-as-call
}

// Function: pthread_mutex_unlock @ 0x3ef0
int pthread_mutex_unlock(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1df78)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x3f00
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_1df80)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x3f10
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_1df88)(); // jump-as-call
  return v1;
}

// Function: fflush_unlocked @ 0x3f20
int fflush_unlocked(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1df90)(); // jump-as-call
  return v1;
}

// Function: open @ 0x3f30
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_1df98)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x3f40
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_1dfa0)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x3f50
int fflush(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_1dfa8)(); // jump-as-call
  return v1;
}

// Function: sub_3f60 @ 0x3f60
undefined16 sub_3f60(void)
{
  char v1 [16];
  unsigned long v2; // rax
  
  *__errno_location() = 0xc;
  v1._8_8_ = 0;
  v1._0_8_ = v2;
  return v1._0_16_ << 0x40;
}

// Function: sub_3f80 @ 0x3f80
unsigned long sub_3f80(int a0,char **a1)
{
  unsigned long *v1;
  long v10;
  long *v11; // rax
  unsigned long v12;
  unsigned long *v13; // rax
  unsigned long *v14;
  unsigned long *v15; // rax
  char *v16; // rax
  unsigned long v17;
  unsigned long v18; // rax
  unsigned long v19; // rax
  bool v2;
  unsigned long v20; // rax
  unsigned long v21; // rax
  unsigned long v22; // rax
  unsigned long v23; // rax
  unsigned long v24; // rax
  unsigned long v25; // rax
  unsigned long v26; // rax
  unsigned long v27; // rax
  unsigned long v28; // rax
  unsigned long v29; // rdx
  bool v3;
  long v30; // rdx
  char *v31;
  unsigned char v32;
  void *v33; // stack - 0xe8
  unsigned long v34 [4]; // stack - 0x258
  int v35; // stack - 0x208
  char v36; // stack - 0x42
  char v37 [80];
  char v38; // r12b
  char v39;
  bool v4;
  int *v40;
  long v41;
  long v42;
  char *v43; // stack - 0x2f8
  char *v44; // stack - 0x2f0
  char *v45; // stack - 0x2e0
  unsigned long v46; // stack - 0x2d8
  int v47; // stack - 0x2d0
  unsigned long v48; // stack - 0x238
  unsigned long v49; // stack - 0x230
  int v5;
  char v50; // stack - 0x228
  char v51; // stack - 0x227
  char v52; // stack - 0x226
  unsigned char v53; // stack - 0x225
  char v54; // stack - 0x224
  char v55; // stack - 0x223
  char v56; // stack - 0x222
  char v57; // stack - 0x221
  char v58; // stack - 0x220
  unsigned int v59; // stack - 0x204
  int v6; // eax
  unsigned long *v60; // stack - 0x200
  unsigned long v61; // stack - 0xe0
  unsigned long v62; // stack - 0xd8
  unsigned long v63; // stack - 0xd0
  unsigned long v64; // stack - 0xc8
  unsigned long v65; // stack - 0xc0
  unsigned long v66; // stack - 0xb8
  unsigned long v67; // stack - 0xb0
  unsigned long v68; // stack - 0xa8
  unsigned long v69; // stack - 0xa0
  char *v7;
  unsigned long v70; // stack - 0x98
  unsigned long v71; // stack - 0x90
  unsigned long v72; // stack - 0x88
  unsigned long v73; // stack - 0x80
  unsigned long v74; // stack - 0x78
  unsigned long v75; // stack - 0x70
  unsigned long v76; // stack - 0x68
  unsigned int v77; // stack - 0x60
  char v78; // stack - 0x41
  long v79; // stack - 0x40
  char *v8; // rax
  unsigned long *v9;
  
  v79 = *(long *)(v42 + 0x28);
  v7 = getenv("POSIXLY_CORRECT");
  v5 = sub_ee70();
  v3 = 0x2b8 < (unsigned int)(v5 - 0x30db0U);
  sub_ef00(*a1);
  v8 = setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_8f60(3);
  dat_1e861 = sub_cfb0();
  dat_1e860 = sub_cfb0(2);
  v9 = (unsigned long *)localeconv();
  dat_1e868 = *(char *)*v9;
  if ((!dat_1e868) || (((char *)*v9)[1]))
    dat_1e868 = '.';
  v39 = *(char *)v9[1];
  dat_1e864 = (int)v39;
  if (v39) { // branch-flip
    if (((char *)v9[1])[1]) {
      dat_1e862 = 1;
      dat_1e864 = 0x80; // crossjump-dupe
    }
  }
  else {
    dat_1e864 = 0x80;
  }
  dat_1e418 = '\0';
  sub_5b80();
  v40 = (int *)0x15324;
  sigemptyset((void *)0x1e380);
  v5 = 0xe;
  while( true ) {
    sigaction(v5,NULL,&v33);
    if (v33 != (void *)0x1)
      sigaddset((void *)0x1e380,v5);
    if (v40 == (int *)0x1534c) break;
    v5 = *v40;
    v40 = &v40[1];
  }
  v77 = 0;
  v33 = sub_5b40;
  v5 = 0xe;
  v61 = dat_1e380;
  v62 = dat_1e388;
  v63 = dat_1e390;
  v64 = dat_1e398;
  v40 = (int *)0x15324;
  v65 = dat_1e3a0;
  v66 = dat_1e3a8;
  v67 = dat_1e3b0;
  v68 = dat_1e3b8;
  v69 = dat_1e3c0;
  v70 = dat_1e3c8;
  v71 = dat_1e3d0;
  v72 = dat_1e3d8;
  v73 = dat_1e3e0;
  v74 = dat_1e3e8;
  v75 = dat_1e3f0;
  v76 = dat_1e3f8;
  while( true ) {
    if (sigismember(0x1e380,v5))
      sigaction(v5,&v33,NULL);
    if (v40 == (int *)0x1534c) break;
    v5 = *v40;
    v40 = &v40[1];
  }
  v41 = 0;
  v5 = 0;
  signal(0x11,0);
  sub_14980(sub_5ad0);
  sub_5cf0(v34,8);
  v34[0] = 0xffffffffffffffff;
  v9 = (unsigned long *)sub_12fd0((long)a0);
  v43 = NULL;
  v47 = 0;
  v44 = NULL;
  v46 = 0;
  v45 = NULL;
  v4 = 0;
label_42ce:
  v35 = -1;
  if (v47 == -1) {
label_42de:
    v10 = (long)optind;
    goto label_42b0;
  }
  if ((!v41) || (!v7)) {
label_4310:
    v40 = &v35;
    v47 = getopt_long(a0,a1,"-bcCdfghik:mMno:rRsS:t:T:uVy:z",(void *)0x1d5a0,v40);
    v31 = optarg;
    if (v47 == -1) goto label_42de;
    if (0x87 < v47) {
      sub_be50(2); // return-dupe, no-return
    }
    if (v47 <= 0) {
      if (v47 == -0x83) {
        sub_12d40(stdout,"sort","GNU coreutils",dat_1e120,"Mike Haertel","Paul Eggert",0,CONCAT44(dat_4,v47));
        exit(0); // no-return
      }
      if (v47 == -0x82)
        sub_be50(0); // no-return
      sub_be50(2);
    }
    switch(v47) {
      default:
        goto label_4db0;
      case 1:
        if (*optarg == '+') {
          if (optind != a0) { // branch-flip
            v2 = 0;
            if (*a1[optind] == '-')
              v2 = (unsigned int)((int)a1[optind][1] - 0x30U) <= 9;
          }
          else {
            v2 = 0;
          }
          v3 = (bool)(v3 | v7 == NULL & v2);
          if (v3) {
            v11 = (long *)sub_5cf0(v37);
            v31 = (char *)sub_5d20(&v31[1],v11,0);
            if (v31) { // branch-flip
              if (*v31 != '.') { // branch-flip
                if ((!*v11) && (!v11[1])) {
label_4bda:
                  *v11 = -1;
                  goto label_4be1;
                }
              }
              else {
                v31 = (char *)sub_5d20(&v31[1],&v11[1],0);
                if ((!*v11) && (!v11[1])) goto label_4bda;
label_4be1:
                if (!v31) goto label_4bea;
              }
              if (!*(char *)sub_57c0(v31,v11,0)) {
                if (v2) {
                  v10 = (long)optind;
                  optind += 1;
                  v31 = a1[v10];
                  v16 = (char *)sub_5d20(&v31[1],&v11[2],"invalid number after \'-\'");
                  if (!v16)
                    __assert_fail("s","src/sort.c",0x1161,"main"); // no-return
                  if (*v16 == '.')
                    v16 = (char *)sub_5d20(&v16[1],&v11[3],"invalid number after \'.\'");
                  if ((!v11[3]) && (v11[2]))
                    v11[2] = v11[2] + -1;
                  if (*(char *)sub_57c0(v16,v11,1))
                    sub_5a10(v31,"stray character in field spec"); // no-return
                }
                *(char *)((long)v11 + 0x39) = 1;
                sub_60c0(v11);
                break;
              }
            }
            else if ((!*v11) && (!v11[1]))
              *v11 = -1;
label_4bea:
            v31 = optarg;
          }
        }
        v9[v41] = v31;
        v41 += 1;
        break;
      case 0x43:
      case 99:
label_4420:
        if (((char)v5) && (v5 != v47))
          sub_5940("cC"); // no-return
        v5 = (int)(char)v47;
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
        goto label_43a5;
      case 0x53:
        sub_89b0(v35,optarg);
        break;
      case 0x54:
        sub_61e0(optarg);
        break;
      case 0x6b:
        v11 = (long *)sub_5cf0(v37);
        v31 = (char *)sub_5d20(optarg,v11,"invalid number at field start");
        v10 = *v11;
        *v11 = v10 + -1;
        if (!v10)
          sub_5a10(optarg,"field number is zero"); // return-dupe, no-return
        if (*v31 == '.') {
          v31 = (char *)sub_5d20(&v31[1],&v11[1],"invalid number after \'.\'");
          v10 = v11[1];
          v11[1] = v10 + -1;
          if (!v10)
            sub_5a10(optarg,"character offset is zero"); // no-return
        }
        if ((!*v11) && (!v11[1]))
          *v11 = -1;
        v31 = (char *)sub_57c0(v31,v11,0);
        if (*v31 != ',') { // branch-flip
          v11[2] = -1;
          v11[3] = 0;
        }
        else {
          v31 = (char *)sub_5d20(&v31[1],&v11[2],"invalid number after \',\'");
          v10 = v11[2];
          v11[2] = v10 + -1;
          if (!v10) {
            sub_5a10(optarg,"field number is zero");
          }
          if (*v31 == '.')
            v31 = (char *)sub_5d20(&v31[1],&v11[3],"invalid number after \'.\'");
          v31 = (char *)sub_57c0(v31,v11,1);
        }
        if (*v31)
          sub_5a10(optarg,"stray character in field spec"); // no-return
        sub_60c0(v11);
        break;
      case 0x6d:
        v4 = 1;
        break;
      case 0x6f:
        if ((v44) && (v6 = strcmp(v44,optarg), v6)) {
          error(2,0,dcgettext(NULL,"multiple output files specified",5));
          return v27;
        }
        v44 = v31;
        break;
      case 0x73:
        dat_1e41a = 1;
        break;
      case 0x74:
        v6 = (int)*optarg;
        if (!*optarg) {
          error(2,0,dcgettext(NULL,"empty tab",5));
          return v24;
        }
        if (optarg[1]) {
          v6 = strcmp(optarg,"\\0");
          if (v6) {
            v17 = sub_11430(v31);
            error(2,0,dcgettext(NULL,"multi-character tab %s",5),v17);
            return v25;
          }
          v6 = 0;
        }
        if ((dat_1e028 != 0x80) && (dat_1e028 != v6)) {
          error(2,0,dcgettext(NULL,"incompatible tabs",5));
          return v26;
        }
        dat_1e028 = v6;
        break;
      case 0x75:
        dat_1e419 = 1;
        break;
      case 0x79:
        if (a1[(long)optind + -1] == optarg) {
          for (; (unsigned int)((int)*v31 - 0x30U) <= 9; v31 = &v31[1]) {
          }
          optind = optind + -1 + (unsigned int)(*v31 == '\0');
        }
        break;
      case 0x7a:
        dat_1e038 = 0;
        break;
      case 0x80:
        v47 = 99;
        if (optarg) {
          v10 = sub_c4b0("--check",optarg,0x1d580,"CCc",1,dat_1e128,1,v29);
          v47 = (int)"CCc"[v10];
        }
        goto label_4420;
      case 0x81:
        if ((dat_1e408) && (v6 = strcmp(dat_1e408,optarg), v6)) {
          error(2,0,dcgettext(NULL,"multiple compress programs specified",5));
          return v23;
        }
        dat_1e408 = v31;
        break;
      case 0x82:
        dat_1e400 = '\x01';
        break;
      case 0x83:
        v43 = optarg;
        break;
      case 0x84:
        sub_8b60(v35,optarg);
        break;
      case 0x85:
        if ((v45) && (v6 = strcmp(v45,optarg), v6)) {
          error(2,0,dcgettext(NULL,"multiple random sources specified",5));
          return v28;
        }
        v45 = v31;
        break;
      case 0x86:
        v10 = sub_c4b0("--sort",optarg,0x1d540,"ghMnRVCCc",1,dat_1e128,1,v40);
        v47 = (int)"ghMnRVCCc"[v10];
label_43a5:
        v36 = (char)v47;
        v78 = 0;
        sub_57c0(&v36,v34,2);
        break;
      case 0x87:
        goto label_481b;
      
    }
    goto label_42ce;
  }
  v10 = (long)optind;
  if ((v3) && (!(char)v5)) {
    if (optind != a0) {
      v31 = a1[optind];
      if (((*v31 == '-') && (v31[1] == 'o')) && ((v31[2] || (optind + 1 != a0)))) goto label_4310;
      goto label_42b0;
    }
    if (!v43) {
      v5 = 0;
      goto label_4466;
    }
label_4d4b:
    v17 = sub_11170(4,*v9);
    error(0,0,dcgettext(NULL,"extra operand %s",5),v17);
    __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"file operands cannot be combined with --files0-from",5));
    sub_be50(2);
  }
label_42b0:
  if ((int)v10 < a0) {
    optind = (int)v10 + 1;
    v9[v41] = a1[v10];
    v41 += 1;
    goto label_42ce;
  }
  if (!v43) goto label_4466;
  if (v41) goto label_4d4b;
  v17 = sub_6010(v43,"r");
  sub_11ee0(&v35);
  if (!sub_11f90(v17,&v35)) {
    v17 = sub_11170(4,v43);
    error(2,0,dcgettext(NULL,"cannot read file names from %s",5),v17);
    return v22;
  }
  sub_6040(v17);
  if (!CONCAT44(v59,v35)) {
    v17 = sub_11170(4,v43);
    error(2,0,dcgettext(NULL,"no input from %s",5),v17);
    return v21;
  }
  free(v9);
  v10 = CONCAT44(v59,v35);
  v9 = v60;
  if (v10) {
    v41 = 0;
    do {
      v7 = (char *)v60[v41];
      if (!strcmp(v7,"-")) {
        v17 = sub_11170(4,v7);
        error(2,0,dcgettext(NULL,"when reading file names from stdin, no file name of %s allowed",5),v17);
        return v18;
      }
      v41 += 1;
      if (!*v7) {
        v17 = sub_11270(0,3,v43);
        error(2,0,dcgettext(NULL,"%s:%lu: invalid zero-length file name",5),v17,v41);
        return v19;
      }
    } while (v10 != v41);
  }
label_4466:
  v39 = (char)v5;
  if (dat_1e410) { // branch-flip
    v32 = 0;
    v10 = dat_1e410;
    do {
      while ((sub_5680(v10) && (!*(char *)(v30 + 0x37)))) {
        *(unsigned long *)(v30 + 0x20) = v48;
        *(unsigned long *)(v30 + 0x28) = v49;
        *(char *)(v30 + 0x30) = v50;
        *(char *)(v30 + 0x31) = v51;
        *(char *)(v30 + 0x36) = v56;
        *(char *)(v30 + 0x32) = v52;
        *(char *)(v30 + 0x34) = v54;
        *(char *)(v30 + 0x35) = v55;
        *(char *)(v30 + 0x37) = v57;
        *(char *)(v30 + 0x38) = v58;
        *(unsigned char *)(v30 + 0x33) = v53;
        v10 = *(long *)(v30 + 0x40);
        v32 |= v53;
        if (!v10) goto label_452f;
      }
      v10 = *(long *)(v30 + 0x40);
      v32 |= *(unsigned char *)(v30 + 0x33);
    } while (v10);
label_452f:
    v38 = 0;
label_4532:
    sub_5970();
    if (dat_1e400) goto label_461e;
  }
  else {
    if (!sub_5680(v34)) {
      v38 = 1;
      sub_60c0(v34);
      v32 = v53;
      goto label_4532;
    }
    sub_5970();
    if (!dat_1e400) {
      dat_1e41b = v57;
      goto label_4560;
    }
    v32 = 0;
    v38 = 0;
label_461e:
    if (v39) {
      s_1e018[0] = v39; // return-dupe
      sub_5940(s_1e018); // no-return
    }
    if (v44) {
      v39 = 'o';
      s_1e018[0] = v39;
      sub_5940(s_1e018);
    }
    if ((!v8) || (!setlocale(3,"")))
      error(0,0,"%s",dcgettext(NULL,"failed to set locale",5));
    if (dat_1e861) { // branch-flip
      v17 = sub_11430(setlocale(3,NULL));
      error(0,0,dcgettext(NULL,"text ordering performed using %s sorting rules",5),v17);
    }
    else {
      error(0,0,"%s",dcgettext(NULL,"text ordering performed using simple byte comparison",5));
    }
    sub_7d30(v34,v38);
  }
  dat_1e41b = v57;
  if (v32)
    sub_6110(v45);
label_4560:
  if (!dat_1e428) {
    v7 = getenv("TMPDIR");
    if (!v7)
      v7 = "/tmp";
    sub_61e0(v7);
  }
  if (!v41) {
    v41 = 1;
    free(v9);
    v9 = (unsigned long *)sub_12eb0(8);
    *v9 = "-";
  }
  if (dat_1e438) {
    v12 = (unsigned long)dat_1e024 * 0x22;
    if ((unsigned long)dat_1e024 * 0x22 < dat_1e438)
      v12 = dat_1e438;
    dat_1e438 = v12;
  }
  if (!v39) {
    sub_6c00(v9,v41);
    sub_6ca0(v44);
    if (v4) {
      v13 = (unsigned long *)sub_13150(v41,0x10);
      v1 = &v9[v41];
      v14 = v13;
      do {
        v17 = *v9;
        v9 = &v9[1];
        v15 = &v14[2];
        *v14 = v17;
        v14 = v15;
      } while (v9 != v1);
      sub_b720(v13,0,v41,v44);
    }
    else {
      if (!v46) {
        v12 = sub_e9d0(2);
        v46 = 8;
        if (v12 <= 8)
          v46 = v12;
      }
      if (0x100000000000000 <= v46)
        v46 = 0xffffffffffffff;
      sub_bac0(v9,v41,v44,v46);
    }
    if ((dat_1e418) && (v5 = sub_c900(stdin), v5 == -1))
      sub_5e10(dcgettext(NULL,"close failed",5),"-"); // no-return
    if (v79 == *(long *)(v42 + 0x28))
      return 0;
    __stack_chk_fail(); // no-return
  }
  if (v41 != 1) {
    v17 = sub_11170(4,v9[1]);
    error(2,0,dcgettext(NULL,"extra operand %s not allowed with -%c",5),v17,(unsigned long)(unsigned int)(int)v39);
    return v20;
  }
  if (v44) {
    dat_1e010 = v39;
    sub_5940(0x1e010); // no-return
  }
  v17 = *v9;
  exit((sub_aac0(v17,(int)v39) ^ 1) & 0xff); // no-return
label_481b:
  v46 = sub_8900(v35,optarg);
  goto label_42ce;
}

// Function: sub_52f0 @ 0x52f0
void sub_52f0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_1dfe0)(sub_3f80,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_5320 @ 0x5320
void sub_5320(void)
{
  return;
}

// Function: _FINI_0 @ 0x5390
void _FINI_0(void)
{
  if (!dat_1e230) {
    if (dat_1dfd8)
      __cxa_finalize(dat_1e008);
    sub_5320();
    dat_1e230 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x53d0
void _INIT_0(void)
{
  return;
}

// Function: sub_53e0 @ 0x53e0
unsigned int sub_53e0(unsigned int a0)
{
  return a0;
}

// Function: sub_53f0 @ 0x53f0
unsigned long sub_53f0(unsigned long a0)
{
  return a0;
}

// Function: sub_5400 @ 0x5400
unsigned long sub_5400(long a0,unsigned long a1)
{
  return (unsigned long)(long)*(int *)(a0 + 8) % a1;
}

// Function: sub_5420 @ 0x5420
unsigned int sub_5420(long a0,long a1)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)(a1 + 8) >> 8),*(int *)(a0 + 8) == *(int *)(a1 + 8));
}

// Function: sub_5430 @ 0x5430
int sub_5430(long *a0)
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
      if (*v1 != dat_1e864)
        v2 = v5;
      v4 = *v2;
      v3 = (int)v4;
      v5 = &v2[1];
    } while ((unsigned int)(v3 - 0x30U) <= 9);
    if (*v1 == dat_1e864) {
      *a0 = (long)&v2[-1];
      return v6;
    }
  }
  if (dat_1e868 != v4) // branch-flip
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

// Function: sub_54f0 @ 0x54f0
int sub_54f0(char *a0) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  int v3; // edx
  unsigned char *v4; // stack - 0x18
  char v5;
  
  v5 = *a0;
  v2 = (unsigned long)(v5 == '-');
  v4 = (unsigned char *)&a0[v2];
  if ((char)sub_5430(&v4) <= '0') {
    v1 = 0;
    return v1;
  }
  v2 = (unsigned long)*v4;
  v1 = (int)*(char *)(v2 + 0x15360);
  v3 = -v1;
  if (v5 == '-')
    v1 = v3;
  return v1;
}

// Function: sub_5570 @ 0x5570
unsigned int sub_5570(char *a0,unsigned long *a1)
{
  char *v1;
  unsigned long v10;
  unsigned long v11; // r8
  char *v12;
  long v13; // r10
  unsigned long v14; // r11
  unsigned char v2; // al
  unsigned char v3; // al
  unsigned int v4; // eax
  unsigned long v5; // rax
  char *v6;
  char v7; // dl
  long v8; // rdx
  unsigned long v9;
  
  while( true ) {
    v4 = sub_53e0((int)*a0);
    v11 = (unsigned long)v4;
    if (!*(char *)(v8 + (unsigned long)(v4 & 0xff))) break;
    a0 = &a0[1];
  }
  v9 = 0xc;
  v10 = 0;
  v13 = 0x1e440;
  do {
    v14 = v10 + v9 >> 1;
    v6 = *(char **)(v14 * 0x10 + 0x1e060);
    if (!*v6) {
label_5630:
      if (a1)
        *a1 = a0;
      return *(unsigned int *)(v14 * 0x10 + 0x1e068);
    }
    v5 = v11 & 0xffffffff;
    v12 = a0;
    while( true ) {
      v2 = sub_53e0((int)*(char *)(v13 + (v5 & 0xff)));
      v3 = sub_53e0((int)v7);
      if (v2 < v3) break;
      if (v3 < v2) {
        v10 = v14 + 1;
        goto label_561a;
      }
      v1 = &v6[1];
      v6 = &v6[1];
      a0 = &a0[1];
      if (!*v1) goto label_5630;
      v5 = sub_53e0((int)*a0);
    }
    v9 = v14;
label_561a:
    a0 = v12;
    if (v9 <= v10)
      return 0;
  } while( true );
}

// Function: sub_5660 @ 0x5660
unsigned long sub_5660(long a0)
{
  unsigned long v1; // rax
  
  v1 = *(unsigned long *)(a0 + 0x30) & 0xffff00ff0000;
  return CONCAT71((undefined7)(v1 >> 8),v1 != 0);
}

// Function: sub_5680 @ 0x5680
unsigned long sub_5680(long a0) // early-return x3
{
  unsigned long v1; // rax
  
  if (*(long *)(a0 + 0x20))
    return 0;
  if (*(long *)(a0 + 0x28))
    return 0;
  if (*(char *)(a0 + 0x30))
    return 0;
  if (*(char *)(a0 + 0x31))
    return 0;
  v1 = sub_5660();
  if ((char)v1)
    return 0;
  if ((!*(char *)(a0 + 0x36)) && (!*(char *)(a0 + 0x38)))
    return (unsigned long)(*(unsigned char *)(a0 + 0x33) ^ 1);
  return v1;
}

// Function: sub_56d0 @ 0x56d0
void sub_56d0(long a0,char *a1)
{
  if (*(short *)(a0 + 0x30)) {
    *a1 = 0x62;
    a1 = &a1[1];
  }
  if (*(long *)(a0 + 0x20) == 0x1e540) {
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
  if (*(long *)(a0 + 0x20) == 0x1e640) {
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

// Function: sub_5780 @ 0x5780
bool sub_5780(long a0,long a1)
{
  if (*(unsigned int *)(a0 + 0x50) != *(unsigned int *)(a1 + 0x50))
    return *(unsigned int *)(a0 + 0x50) < *(unsigned int *)(a1 + 0x50);
  return (unsigned long)(*(long *)(a0 + 0x30) + *(long *)(a0 + 0x28)) < (unsigned long)(*(long *)(a1 + 0x30) + *(long *)(a1 + 0x28));
}

// Function: sub_57c0 @ 0x57c0
char * sub_57c0(char *a0,long a1,unsigned int a2)
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
          goto label_581d;
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
          *(unsigned long *)(a1 + 0x20) = 0x1e540;
          break;
        case 0x66:
          *(unsigned long *)(a1 + 0x28) = 0x1e440;
          break;
        case 0x67:
          *(char *)(a1 + 0x34) = 1;
          break;
        case 0x68:
          *(char *)(a1 + 0x35) = 1;
          break;
        case 0x69:
          if (!*(long *)(a1 + 0x20))
            *(unsigned long *)(a1 + 0x20) = 0x1e640;
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
label_581d:
  return a0;
}

// Function: sub_58d0 @ 0x58d0
void sub_58d0(void)
{
  void *v1;
  
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"\nWith no FILE, or when FILE is -, read standard input.\n",5),v1); // tail-call
}

// Function: sub_5900 @ 0x5900
void sub_5900(void)
{
  void *v1;
  
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v1); // tail-call
}

// Function: sub_5930 @ 0x5930
void sub_5930(unsigned long *a0,unsigned long *a1)
{
  strcmp((char *)*a0,(char *)*a1); // tail-call
}

// Function: sub_5940 @ 0x5940
void sub_5940(unsigned long a0)
{
  error(2,0,dcgettext(NULL,"options \'-%s\' are incompatible",5),a0);
}

// Function: sub_5970 @ 0x5970
void sub_5970(void)
{
  long v1;
  char v2 [40];
  char *v3; // rsp
  long v4; // fs_offset
  unsigned long v5; // stack - 0x10
  
  v3 = v2;
  v5 = *(unsigned long *)(v4 + 0x28);
  v1 = dat_1e410;
  while( true ) {
    if (!v1)
      return;
    if (1 < (unsigned int)*(unsigned char *)(v1 + 0x32) + (unsigned int)*(unsigned char *)(v1 + 0x34) + (unsigned int)*(unsigned char *)(v1 + 0x35) + (unsigned int)*(unsigned char *)(v1 + 0x36) + (unsigned int)(unsigned char)(*(unsigned char *)(v1 + 0x38) | *(unsigned char *)(v1 + 0x33) | *(long *)(v1 + 0x20) != 0)) break;
    v1 = *(long *)(v1 + 0x40);
  }
  *(char *)(v1 + 0x37) = 0;
  *(unsigned short *)(v1 + 0x30) = 0;
  sub_56d0(v1,v2);
  sub_5940(v3); // no-return
}

// Function: sub_5a10 @ 0x5a10
void sub_5a10(unsigned long a0,char *a1)
{
  unsigned long v1; // rax
  char *v2; // rax
  
  v1 = sub_11430(a0);
  v2 = dcgettext(NULL,a1,5);
  error(2,0,dcgettext(NULL,"%s: invalid field specification %s",5),v2,v1);
}

// Function: sub_5a60 @ 0x5a60
void sub_5a60(long a0)
{
  *(bool *)a0 = pthread_sigmask(0,0x1e380,a0 + 8) == 0;
}

// Function: sub_5a80 @ 0x5a80
void sub_5a80(char *a0)
{
  if (!*a0)
    return;
  pthread_sigmask(2,&a0[8],0); // tail-call
}

// Function: sub_5aa0 @ 0x5aa0
void sub_5aa0(void)
{
  unsigned long *v1;
  
  for (v1 = dat_1e368; v1; v1 = (unsigned long *)*v1) {
    unlink((char *)((long)v1 + 0xd));
  }
  dat_1e368 = NULL;
}

// Function: sub_5ad0 @ 0x5ad0
void sub_5ad0(void)
{
  char v1 [136];
  
  if (dat_1e368) {
    sub_5a60(v1);
    sub_5aa0();
    sub_5a80(v1);
  }
  sub_c5c0(); // tail-call
}

// Function: sub_5b40 @ 0x5b40
void sub_5b40(int a0)
{
  sub_5aa0();
  signal(a0,0);
  raise(a0); // tail-call
}

// Function: sub_5b60 @ 0x5b60
unsigned short sub_5b60(unsigned char a0)
{
  return *(unsigned short *)(*(long *)__ctype_b_loc() + (unsigned long)a0 * 2) & 1 | (unsigned short)(a0 == 10);
}

// Function: sub_5b80 @ 0x5b80
unsigned long sub_5b80(void)
{
  char v1;
  char *v10;
  unsigned long v11; // rdi
  long v12;
  long v13; // r13
  long *v14; // r14
  unsigned long v15; // stack - 0x38
  unsigned short v2;
  unsigned char v3; // al
  long *v4; // rax
  char *v5; // rax
  long v6; // rax
  unsigned long v7; // rax
  long v8; // rdx
  unsigned long v9; // rax
  
  v9 = 0;
  v4 = __ctype_b_loc();
  do {
    *(char *)(v9 + 0x1e740) = sub_5b60(v9 & 0xffffffff);
    v2 = *(unsigned short *)(*v4 + v9 * 2);
    *(unsigned char *)(v9 + 0x1e640) = ((unsigned char)(v2 >> 0xe) ^ 1) & 1;
    v3 = 0;
    if (!(v2 & 8))
      v3 = sub_5b60(v9 & 0xffffffff) ^ 1;
    *(unsigned char *)(v9 + 0x1e540) = v3;
    *(char *)(v9 + 0x1e440) = (char)toupper((int)v9);
    v9 += 1;
  } while (v9 != 0x100);
  if (dat_1e860) {
    v14 = (long *)0x1e060;
    v13 = 1;
    do {
      v5 = nl_langinfo((int)v13 + 0x2000d);
      v9 = strlen(v5);
      v6 = sub_12eb0(v9 + 1);
      *(int *)&v14[1] = (int)v13;
      *v14 = v6;
      if (v9) {
        v12 = *v4;
        v10 = &v5[v9];
        v9 = 0;
        do {
          v1 = *v5;
          v11 = (unsigned long)(unsigned char)sub_53e0((int)v1);
          if (!(*(unsigned char *)(v12 + v11 * 2) & 1)) {
            *(char *)(v6 + v9) = *(char *)(v11 + 0x1e440);
            v9 += 1;
          }
          v5 = (char *)(v8 + 1);
        } while (v10 != v5);
      }
      v13 += 1;
      *(char *)(v6 + v9) = 0;
      v14 = &v14[2];
    } while (v13 != 0xd);
    v7 = qsort(0x1e060,0xc,0x10,sub_5930,v15); // tail-call
    return v7;
  }
  return v15;
}

// Function: sub_5cf0 @ 0x5cf0
char (* sub_5cf0(char (*a0)[16]))[16]
{
  *(unsigned long *)a0[4] = 0;
  a0[1] = 0;
  *a0 = 0;
  *(unsigned long *)a0[1] = 0xffffffffffffffff;
  a0[2] = 0;
  a0[3] = 0;
  return a0;
}

// Function: sub_5d20 @ 0x5d20
char * sub_5d20(unsigned long a0,unsigned long *a1,char *a2)
{
  unsigned long v1;
  char *v2;
  char *v3; // rax
  unsigned long v4; // stack - 0x28
  unsigned long v5; // stack - 0x30
  
  switch(sub_13650(a0,&v5,10,&v4,"")) {
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
      v3 = (unsigned long)sub_11430(a0);
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

// Function: sub_5e10 @ 0x5e10
void sub_5e10(unsigned long a0,char *a1)
{
  unsigned long v1; // rax
  
  if (!a1)
    a1 = dcgettext(NULL,"standard output",5);
  v1 = sub_11270(0,3,a1);
  error(2,*__errno_location(),"%s: %s",a0,v1);
}

// Function: sub_5e70 @ 0x5e70
unsigned long sub_5e70(void) // return-dupe x2, return-dupe
{
  if (dat_1e2f0) {
    if (dat_1e2f0 <= -1)
      return 0x1e260;
    return 0;
  }
  if (!fstat(1,(void *)0x1e260)) {
    dat_1e2f0 = 0xffffffff;
    return 0x1e260;
  }
  dat_1e2f0 = *__errno_location();
  if (dat_1e2f0 <= -1)
    return 0x1e260;
  return 0;
}

// Function: sub_5ee0 @ 0x5ee0
int * sub_5ee0(char *a0,char *a1)
{
  int v1;
  unsigned long v2;
  unsigned int v3; // eax
  int *v4; // rax
  void *v5;
  
  if (*a1 == 'r') {
    v3 = strcmp(a0,"-");
    if (v3) { // branch-flip
      v5 = NULL;
      v3 = open(a0,0x80000);
      if (0 <= (int)v3) {
        v4 = fdopen(v3,a1);
        v5 = v4;
      }
    }
    else {
      dat_1e418 = 1;
      v5 = stdin;
    }
    sub_c8d0(v5,2);
    v4 = v5;
    return v4;
  }
  if (*a1 == 'w') {
    if (a0) {
      v3 = ftruncate(1,0);
      if (v3) {
        v4 = __errno_location();
        v1 = *v4;
        v4 = (long)sub_5e70();
        if ((!v4) || (v3 = *(unsigned int *)((long)v4 + 0x18) & 0xf000, v3 == 0x8000)) {
          v4 = (unsigned long)sub_11270(0,3,a0);
          v2 = v4;
          v4 = (char *)dcgettext(NULL,"%s: error truncating",5);
          error(2,v1,(char *)v4,v2);
          return v4;
        }
      }
    }
    v4 = stdout;
    return v4;
  }
  __assert_fail("!\"unexpected mode passed to stream_open\"","src/sort.c",0x3d5,"stream_open"); // no-return
}

// Function: sub_6010 @ 0x6010
void sub_6010(unsigned long a0,unsigned long a1)
{
  if (sub_5ee0(a0,a1))
    return;
  sub_5e10(dcgettext(NULL,"open failed",5),a0); // no-return
}

// Function: sub_6040 @ 0x6040
void sub_6040(void *a0,unsigned long a1) // return-dupe
{
  int v1; // eax
  char *v2; // rsi
  
  v1 = fileno(a0);
  if (!v1) {
    clearerr_unlocked(a0); // tail-call
    return;
  }
  if (v1 != 1) { // branch-flip
    if (!sub_c900(a0))
      return;
    v2 = "close failed";
  }
  else {
    if (!fflush_unlocked(a0))
      return;
    v2 = "fflush failed";
  }
  sub_5e10(dcgettext(NULL,v2,5),a1); // no-return
}

// Function: sub_60c0 @ 0x60c0
void sub_60c0(unsigned long a0)
{
  long v1;
  long v2; // rax
  long *v3; // rcx
  long v4; // rdx
  
  v2 = sub_131b0(a0,0x48);
  v3 = (long *)0x1e410;
  v1 = dat_1e410;
  if (dat_1e410) {
    do {
      v4 = v1;
      v1 = *(long *)(v4 + 0x40);
    } while (v1);
    v3 = (long *)(v4 + 0x40);
  }
  *v3 = v2;
  *(unsigned long *)(v2 + 0x40) = 0;
}

// Function: sub_6110 @ 0x6110
void sub_6110(char *a0)
{
  long v1; // rax
  char v2 [24];
  char *v3; // rsi
  
  v1 = sub_11710(a0,0x10);
  if (v1) { // branch-flip
    sub_11830(v1,v2,0x10);
    if (!sub_11850(v1)) {
      sub_c680(0x1e300);
      sub_c690(v2,0x10,0x1e300);
      return;
    }
    v3 = "close failed";
  }
  else {
    if (!a0)
      a0 = "getrandom";
    v3 = "open failed";
  }
  sub_5e10(dcgettext(NULL,v3,5),a0); // no-return
}

// Function: sub_61e0 @ 0x61e0
void sub_61e0(unsigned long a0)
{
  long v1;
  
  v1 = dat_1e430;
  if (dat_1e428 == dat_1e420) {
    v1 = sub_12ff0(dat_1e430,0x1e420,8);
    dat_1e430 = v1;
  }
  *(unsigned long *)(v1 + dat_1e428 * 8) = a0;
  dat_1e428 += 1;
}

// Function: sub_6240 @ 0x6240
void sub_6240(unsigned long *a0,long a1,unsigned long a2)
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
  sub_13270(); // no-return
}

// Function: sub_62b0 @ 0x62b0
long sub_62b0(char *a0,char *a1)
{
  long v1; // rax
  char *v2; // rbx
  
  v1 = (long)(int)sub_e580(a0,(long)a1 - (long)a0,0);
  if (a0 < a1) {
    do {
      v2 = &a0[1];
      v1 += (unsigned long)(*a0 == '\t');
      a0 = v2;
    } while (a1 != v2);
  }
  return v1;
}

// Function: sub_62f0 @ 0x62f0
void sub_62f0(long a0,long a1)
{
  for (; a0; a0 = a0 + -1) {
    putchar_unlocked(0x20);
  }
  if (a1) {
    do {
      putchar_unlocked(0x5f);
      a1 -= 1;
    } while (a1);
    putchar_unlocked(10); // tail-call
    return;
  }
  __printf_chk(1,dcgettext(NULL,"^ no match for key\n",5)); // tail-call
}

// Function: sub_6370 @ 0x6370
void sub_6370(unsigned long a0,unsigned long a1)
{
  undefined10 v1; // stack - 0x38
  undefined10 v2; // stack - 0x28
  undefined10 v3; // st0
  undefined10 v4; // st0
  
  strtold(a0,0);
  v1 = v3;
  strtold(a1,0);
  v2 = v4;
  memcmp(&v1,&v2,0x10);
}

// Function: sub_63d0 @ 0x63d0
unsigned long sub_63d0(long a0,long a1) // return-dupe
{
  long v1; // stack - 0x30
  long v2; // stack - 0x28
  long double v3; // st0
  long double v4; // st0
  
  strtold(a0,&v1);
  strtold(a1,&v2);
  if (v1 == a0)
    return (unsigned long)-(unsigned int)(v2 != a1);
  if (v2 == a1)
    return 1;
  if (v3 < v4)
    return 0xffffffff;
  if (v3 <= v4) {
    if (v3 == v4)
      return 0;
    if (!NAN(v4))
      return 0xffffffff;
    if (!NAN(v3))
      return 1;
    return sub_6370(a0,a1);
  }
  return 1;
}

// Function: sub_64d0 @ 0x64d0
void sub_64d0(char *a0,char *a1)
{
  char v1;
  long v2; // rdx
  long v3; // rdx
  
  while (v1 = *a0, *(char *)(v2 + (unsigned long)(unsigned char)sub_53e0((int)v1))) {
    a0 = &a0[1];
  }
  while (v1 = *a1, *(char *)(v3 + (unsigned long)(unsigned char)sub_53e0((int)v1))) {
    a1 = &a1[1];
  }
  sub_12790(a0); // tail-call
}

// Function: sub_6530 @ 0x6530
void sub_6530(char *a0,char *a1)
{
  char v1;
  int v2; // eax
  long v3; // rdx
  long v4; // rdx
  
  while (v1 = *a0, *(char *)(v3 + (unsigned long)(unsigned char)sub_53e0((int)v1))) {
    a0 = &a0[1];
  }
  while (v1 = *a1, *(char *)(v4 + (unsigned long)(unsigned char)sub_53e0((int)v1))) {
    a1 = &a1[1];
  }
  v2 = sub_54f0(a0);
  if (v2 != sub_54f0(a1))
    return;
  sub_12790(a0,a1,(int)dat_1e868,dat_1e864); // tail-call
}

// Function: sub_65c0 @ 0x65c0
void sub_65c0(unsigned long a0,unsigned long a1,unsigned long a2)
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
  v3 = sub_110a0(0,8,a1);
  error(2,0,dcgettext(NULL,"the untransformed string was %s",5),v3);
}

// Function: sub_6680 @ 0x6680
int sub_6680(char *a0,unsigned long a1,char *a2,unsigned long a3)
{
  char *v1;
  char v10 [16];
  char v11 [16];
  char *v12;
  unsigned long v13;
  char *v14;
  unsigned long v15;
  char *v16; // stack - 0x1118
  char *v17; // stack - 0x1108
  unsigned long v18; // stack - 0x10f8
  int v19; // stack - 0x10ec
  char *v2;
  unsigned long v20; // stack - 0x10a0
  unsigned long v21; // stack - 0x1098
  unsigned long v22; // stack - 0x1090
  unsigned long v23; // stack - 0x1088
  unsigned long v24; // stack - 0x1080
  unsigned long v25; // stack - 0x1078
  unsigned long v26; // stack - 0x1070
  unsigned long v27; // stack - 0x1068
  unsigned long v28; // stack - 0x1060
  unsigned long v29; // stack - 0x1058
  int v3; // eax
  unsigned int v30; // stack - 0x1050
  unsigned long v31; // stack - 0x1044
  unsigned long v32; // stack - 0x103c
  unsigned int v33; // stack - 0x1034
  unsigned int v34; // stack - 0x1030
  unsigned int v35; // stack - 0x102c
  unsigned int v36; // stack - 0x1028
  unsigned long v37; // stack - 0x1024
  unsigned long v38; // stack - 0x101c
  unsigned long v39; // stack - 0x1014
  char *v4;
  unsigned long v40; // stack - 0x100c
  unsigned long v41; // stack - 0x1004
  unsigned long v42; // stack - 0xffc
  unsigned int v43; // stack - 0xff4
  unsigned long v5;
  unsigned long v6;
  char v7 [4008];
  unsigned long v8; // stack - 0x10a8
  unsigned long v9; // stack - 0x104c
  
  v43 = dat_1e358;
  v42 = dat_1e350;
  v29 = dat_1e350;
  v30 = dat_1e358;
  v9 = dat_1e300;
  v31 = dat_1e308;
  v32 = dat_1e310;
  v34 = (unsigned int)((unsigned long)dat_1e318 >> 0x20);
  v33 = (unsigned int)dat_1e318;
  v36 = (unsigned int)((unsigned long)dat_1e320 >> 0x20);
  v35 = (unsigned int)dat_1e320;
  v37 = dat_1e328;
  v38 = dat_1e330;
  v39 = dat_1e338;
  v40 = dat_1e340;
  v41 = dat_1e348;
  v8 = dat_1e300;
  v20 = dat_1e308;
  v21 = dat_1e310;
  v22 = dat_1e318;
  v23 = dat_1e320;
  v24 = dat_1e328;
  v25 = dat_1e330;
  v26 = dat_1e338;
  v27 = dat_1e340;
  v28 = dat_1e348;
  if (dat_1e861) { // branch-flip
    v1 = &a0[a1];
    v14 = NULL;
    v13 = 4000;
    v2 = &a2[a3];
    v6 = (a1 + a3) * 3 + 2;
    v19 = 0;
    v16 = v7;
label_67c8:
    if (v13 < v6) goto label_6899;
label_67d3:
    v5 = v13;
    v4 = v14;
    if (a0 < v1) goto label_68d7;
label_67de:
    v14 = v4;
    if (a2 < v2) {
      a1 = 0;
      v12 = v16;
      v5 = v13;
      goto label_6902;
    }
    v18 = 0;
    a1 = 0;
    v17 = v16;
label_67fd:
    if (a2 < v2) {
label_6808:
      do {
        v4 = (unsigned long)strlen(a2);
        a2 = &a2[(long)v4 + 1];
        do {
          while( true ) {
            if ((v1 <= a0) && (v2 <= a2)) goto label_6bee;
            sub_c690(v16,a1,&v8);
            sub_c690(v17,v18,&v9);
            if (v19) goto label_67c8;
            v5 = v18;
            if (a1 <= v18)
              v5 = a1;
            v3 = memcmp(v16,v17,v5);
            v19 = v3;
            if (v3) goto label_67c8;
            v3 = (unsigned int)(a1 >= v18 && a1 != v18) - (unsigned int)(a1 < v18);
            v19 = v3;
            if (v6 <= v13) goto label_67d3;
label_6899:
            v13 = v13 * 3 >> 1;
            if (v13 < v6)
              v13 = v6;
            free(v14);
            v4 = malloc(v13);
            v14 = v4;
            if (!v4) {
              v13 = 4000;
              v16 = v7;
              goto label_67d3;
            }
            v16 = v4;
            v5 = v13;
            if (v1 <= a0) goto label_67de;
label_68d7:
            v4 = (long)sub_65c0(v16,a0,v5);
            a1 = (long)v4 + 1;
            if (v2 > a2) break;
            v18 = 0;
            v13 = v5;
            v15 = a1;
            if (v5 >= a1) goto label_6b78;
label_6920:
            v13 = v15;
            if (v15 <= 0x5555555555555554)
              v13 = v15 * 3 >> 1;
            free(v14);
            v4 = (char *)sub_12eb0(v13);
            v14 = v4;
            if (v1 <= a0) { // branch-flip
              v17 = &v4[a1];
              if (a2 < v2) {
                strxfrm(v17,a2,v18);
                v16 = v4;
                goto label_6808;
              }
            }
            else {
              strxfrm(v4,a0,a1);
              v17 = &v4[a1];
              if (a2 < v2) {
                strxfrm(v17,a2,v18);
                v4 = (unsigned long)strlen(a0);
                v16 = v14;
                a0 = &a0[(long)v4 + 1];
                goto label_6808;
              }
            }
            v17 = &v4[a1];
            if (v1 <= a0) {
              a0 = v4;
              goto label_697c;
            }
            v4 = (unsigned long)strlen(a0);
            v16 = v14;
            a0 = &a0[(long)v4 + 1];
          }
          if (a1 <= v5) { // branch-flip
            v13 = v5 - a1;
            v12 = &v16[a1];
          }
          else {
            v13 = 0;
            v12 = NULL;
          }
label_6902:
          v4 = (long)sub_65c0(v12,a2,v13);
          v4 = (unsigned long)((long)v4 + 1);
          v13 = v5;
          v15 = (long)v4 + a1;
          v18 = (unsigned long)v4;
          if (a1 > v5) goto label_6920;
label_6b78:
          if (v13 < v15) goto label_6920;
          v4 = &v16[a1];
          v17 = v4;
          if (v1 <= a0) goto label_67fd;
          v4 = (unsigned long)strlen(a0);
          a0 = &a0[(long)v4 + 1];
        } while (v2 <= a2);
      } while( true );
    }
label_6bee:
    a0 = v16;
label_697c:
    sub_c690(a0,a1,&v8);
    sub_c6c0(&v8,v10);
    sub_c690(v17,v18,&v9);
    sub_c6c0(&v9,v11);
    v3 = memcmp(v10,v11,0x10);
    if (!v3) {
      if (v19) {
        free(v14); // return-dupe
        v3 = v19;
        return v3;
      }
      goto label_6b27;
    }
  }
  else {
    sub_c690(a0,a1,&v8);
    sub_c6c0(&v8,v10);
    sub_c690(a2,a3,&v9);
    sub_c6c0(&v9,v11);
    v3 = memcmp(v10,v11,0x10);
    v14 = NULL;
    v19 = v3;
    v18 = a3;
    v17 = a2;
    if (v3) {
      free(v14);
      v3 = v19;
      return v3;
    }
label_6b27:
    v6 = v18;
    if (a1 <= v18)
      v6 = a1;
    v3 = memcmp(a0,v17,v6);
    v19 = v3;
    if (v3) {
      free(v14);
      v3 = v19;
      return v3;
    }
    v3 = (unsigned int)(v18 < a1) - (unsigned int)(a1 < v18);
  }
  v19 = v3;
  free(v14);
  v3 = v19;
  return v3;
}

// Function: sub_6c00 @ 0x6c00
void sub_6c00(unsigned long *a0,long a1)
{
  char *v1;
  unsigned long v2;
  long v3; // rbp
  
  if (!a1)
    return;
  v3 = 0;
  do {
    v1 = (char *)*a0;
    if (strcmp(v1,"-")) {
      if (euidaccess(v1,4)) {
        v2 = *a0;
        sub_5e10(dcgettext(NULL,"cannot read",5),v2); // no-return
      }
    }
    v3 += 1;
    a0 = &a0[1];
  } while (a1 != v3);
}

// Function: sub_6c80 @ 0x6c80
void sub_6c80(int a0,int a1)
{
  if (a0 == a1)
    return;
  dup2(a0,a1);
  close(a0); // tail-call
}

// Function: sub_6ca0 @ 0x6ca0
void sub_6ca0(char *a0)
{
  int v1; // eax
  
  if (!a0)
    return;
  v1 = open(a0,0x80041,0x1b6);
  if (0 <= v1) {
    sub_6c80(v1,1); // tail-call
    return;
  }
  sub_5e10(dcgettext(NULL,"open failed",5),a0); // no-return
}

// Function: sub_6cf0 @ 0x6cf0
bool sub_6cf0(unsigned int a0)
{
  long v1; // rax
  char v2 [8];
  unsigned int v3; // stack - 0x20
  
  v3 = a0;
  v1 = sub_dff0(dat_1e360,v2);
  if (v1)
    *(char *)(v1 + 0xc) = 2;
  return v1 != 0;
}

// Function: sub_6d50 @ 0x6d50
unsigned long sub_6d50(int a0)
{
  unsigned int v1; // eax
  char *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // rax
  unsigned int v6; // stack - 0x24
  int v7;
  
  v7 = -1;
  if (a0)
    v7 = a0;
  v1 = waitpid(v7,(int *)&v6,(unsigned int)(a0 == 0));
  if (0 <= (int)v1) {
    if ((v1) && ((1 <= a0 || (sub_6cf0(v1))))) {
      if (v6 >> 8 & 0xff || v6 & 0x7f) {
        v4 = sub_11170(4,dat_1e408);
        error(2,0,dcgettext(NULL,"%s [-d] terminated abnormally",5),v4);
        return v5;
      }
      dat_1e35c -= 1;
    }
    return (unsigned long)v1;
  }
  v4 = sub_11170(4,dat_1e408);
  v2 = dcgettext(NULL,"waiting for %s [-d]",5);
  error(2,*__errno_location(),v2,v4);
  return v3;
}

// Function: sub_6e60 @ 0x6e60
void sub_6e60(unsigned long a0)
{
  if (!sub_6cf0(a0))
    return;
  sub_6d50(a0 & 0xffffffff); // tail-call
}

// Function: sub_6e80 @ 0x6e80
void sub_6e80(char *a0)
{
  int v1;
  long v2;
  long *v3;
  long *v4;
  int v5; // eax
  unsigned long v6; // rax
  char v7 [136];
  
  v3 = (long *)0x1e368;
  for (v4 = dat_1e368; a0 != (char *)((long)v4 + 0xd); v4 = (long *)*v4) {
    v3 = v4;
  }
  if (*(char *)((long)v4 + 0xc) == '\x01')
    sub_6e60((int)v4[1]);
  v2 = *v4;
  sub_5a60(v7);
  v5 = unlink(a0);
  v1 = *__errno_location();
  *v3 = v2;
  sub_5a80(v7);
  if (v5) {
    v6 = sub_11270(0,3,a0);
    error(0,v1,dcgettext(NULL,"warning: cannot remove: %s",5),v6);
  }
  if (!v2)
    dat_1e040 = v3;
  free(v4);
}

// Function: sub_6fb0 @ 0x6fb0
void sub_6fb0(void) // return-dupe
{
  if (dat_1e35c <= 0)
    return;
  do {
    if (!sub_6d50(0))
      return;
  } while (0 < dat_1e35c);
}

// Function: sub_6ff0 @ 0x6ff0
void sub_6ff0(void)
{
  sub_6d50(0xffffffff);
  sub_6fb0(); // tail-call
}

// Function: sub_7010 @ 0x7010
void sub_7010(void)
{
  if (dat_1e35c <= 0)
    return;
  do {
    sub_6d50(0xffffffff);
  } while (0 < dat_1e35c);
}

// Function: sub_7040 @ 0x7040
int sub_7040(int *a0,long a1)
{
  unsigned long v1;
  int v2; // eax
  int *v3; // rax
  char v4 [136];
  int v5;
  double v6; // stack - 0xd8
  
  if (0 <= (int)sub_ec90(a0,0x80000)) { // branch-flip
    if (dat_1e024 + 1U < dat_1e35c)
      sub_6ff0();
    v3 = __errno_location();
    v6 = dat_170f0;
    do {
      a1 -= 1;
      sub_5a60(v4);
      v1 = dat_1e368;
      dat_1e368 = 0;
      v2 = fork();
      v5 = *v3;
      if (!v2) {
        sub_5a80(v4);
        *v3 = v5;
        close(0);
        close(1);
        return 0;
      }
      dat_1e368 = v1;
      sub_5a80(v4);
      *v3 = v5;
      if (0 <= v2) {
        dat_1e35c += 1;
        return v2;
      }
      if (v5 != 0xb) goto label_717d;
      sub_13440(v6);
      v6 = v6 + v6;
      sub_6fb0();
    } while (a1);
    v5 = *v3;
label_717d:
    close(*a0);
    close(a0[1]);
    *v3 = v5;
  }
  else {
    v2 = -1;
  }
  return v2;
}

// Function: sub_71d0 @ 0x71d0
void sub_71d0(int a0,char *a1)
{
  char *v1; // rax
  char v2 [12];
  long v3; // fs_offset
  unsigned long v4; // stack - 0x20
  
  v4 = *(unsigned long *)(v3 + 0x28);
  write(2,a1,strlen(a1));
  if (a0) {
    v1 = (char *)sub_e440(a0,v2);
    write(2,": errno ",8);
    write(2,v1,strlen(v1));
  }
  write(2,"\n",1);
  _exit(2); // no-return
}

// Function: sub_7270 @ 0x7270
void sub_7270(long a0)
{
  long v1;
  
  v1 = dat_1e360;
  if (!dat_1e360) {
    v1 = sub_d970(0x2f,0,sub_5400,sub_5420,0);
    dat_1e360 = v1;
    if (!v1) {
      sub_13270(); // no-return, return-dupe
    }
  }
  *(char *)(a0 + 0xc) = 1;
  if (sub_df90(v1,a0))
    return;
  sub_13270();
}

// Function: sub_72d0 @ 0x72d0
int * sub_72d0(long a0)
{
  int v1;
  int *v2;
  unsigned long v3;
  int v4; // eax
  int *v5; // rax
  int v6; // stack - 0x28
  void *v7;
  int v8; // stack - 0x24
  
  if (*(char *)(a0 + 0xc) == '\x01')
    sub_6e60(*(unsigned int *)(a0 + 8));
  v4 = open((char *)(a0 + 0xd),0);
  v1 = v4;
  if (0 <= v4) { // branch-flip
    v4 = sub_7040(&v6,9);
    if (v4 != -1) { // branch-flip
      if (!v4) {
        close(v6);
        sub_6c80(v1,0);
        sub_6c80(v8,1);
        execlp(dat_1e408,dat_1e408,"-d",0);
        v5 = __errno_location();
        sub_71d0(*v5,"couldn\'t execute compress program (with -d)"); // no-return
      }
      *(int *)(a0 + 8) = v4;
      sub_7270(a0);
      close(v1);
      close(v8);
      v5 = fdopen(v6,"r");
      v7 = v5;
      if (!v5) {
        v5 = __errno_location();
        v1 = *v5;
        close(v6);
        *v5 = v1;
      }
    }
    else {
      v5 = __errno_location();
      v2 = v5;
      if (*v5 != 0x18) {
        v5 = (unsigned long)sub_11170(4,dat_1e408);
        v3 = v5;
        v5 = (char *)dcgettext(NULL,"couldn\'t create process for %s -d",5);
        error(2,*v2,(char *)v5,v3);
        return v5;
      }
      v7 = NULL;
      close(v1);
      *v5 = 0x18;
    }
  }
  else {
    v7 = NULL;
  }
  v5 = v7;
  return v5;
}

// Function: sub_7470 @ 0x7470
long sub_7470(unsigned long *a0,long a1,long *a2)
{
  long v1;
  long v2;
  long v3; // rax
  long v4; // r13
  
  v3 = sub_12fd0(a1,8);
  v2 = v3;
  *a2 = v3;
  if (a1) { // branch-flip
    v4 = 0;
    do {
      v1 = a0[1];
      if ((v1) && (*(char *)(v1 + 0xc))) {
        v3 = sub_72d0(v1);
        *(long *)(v2 + v4 * 8) = v3;
      }
      else {
        v3 = sub_5ee0(*a0,"r");
        *(long *)(v2 + v4 * 8) = v3;
      }
      if (!v3) {
        v3 = v4;
        return v3;
      }
      v4 += 1;
      a0 = &a0[2];
    } while (a1 != v4);
  }
  else {
    v4 = 0;
  }
  v3 = v4;
  return v3;
}

// Function: sub_7510 @ 0x7510
int * sub_7510(int *a0,char a1)
{
  void *v1;
  char v10 [136];
  unsigned long *v11;
  int v2;
  char *v3;
  int v4;
  unsigned long v5;
  unsigned long *v6;
  int *v7;
  unsigned long v8;
  int *v9; // rax
  
  v3 = *(char **)(dat_1e430 + dat_1e248 * 8);
  v9 = (unsigned long)strlen(v3);
  v5 = (unsigned long)v9;
  v9 = (unsigned long *)sub_12eb0((long)v9 + 0x20U & 0xfffffffffffffff8);
  v6 = (unsigned long *)v9;
  v1 = (void *)((long)v9 + 0xd);
  memcpy(v1,v3,v5);
  *(unsigned long *)((long)v9 + v5 + 0xd) = s_152f0._0_8_;
  *(unsigned int *)((long)v9 + v5 + 0x15) = s_152f0._8_4_;
  *(unsigned long *)v9 = 0;
  v9 = (long)(dat_1e248 + 1);
  dat_1e248 = (long)v9;
  if (v9 == (int *)dat_1e428)
    dat_1e248 = 0;
  sub_5a60(v10);
  v4 = sub_12280(v1,0x80000);
  v9 = __errno_location();
  v7 = v9;
  if (0 <= v4) { // branch-flip
    v2 = *v9;
    *dat_1e040 = v6;
    dat_1e040 = v6;
    sub_5a80(v10);
    *v9 = v2;
    v11 = v6;
  }
  else {
    v2 = *v9;
    sub_5a80(v10);
    *v9 = v2;
    if ((a1 != '\x01') || (v2 != 0x18)) {
      v9 = (unsigned long)sub_11170(4,v3);
      v8 = v9;
      v9 = (char *)dcgettext(NULL,"cannot create temporary file in %s",5);
      error(2,*v7,(char *)v9,v8);
      return v9;
    }
    v11 = NULL;
    free(v6);
  }
  *a0 = v4;
  v9 = (int *)v11;
  return v9;
}

// Function: sub_76b0 @ 0x76b0
long sub_76b0(unsigned long *a0,char a1)
{
  int v1; // eax
  long v2; // rax
  void *v3; // rax
  int v4; // stack - 0x2c
  int v5; // stack - 0x28
  int v6; // stack - 0x24
  
  v2 = sub_7510(&v4,a1);
  if (v2) {
    *(char *)(v2 + 0xc) = 0;
    if (dat_1e408) {
      v1 = sub_7040(&v5,4);
      *(int *)(v2 + 8) = v1;
      if (1 <= v1) { // branch-flip
        close(v4);
        close(v5);
        v4 = v6;
        sub_7270(v2);
      }
      else if (!v1) {
        close(v6);
        sub_6c80(v4,1);
        sub_6c80(v5,0);
        execlp(dat_1e408,dat_1e408,0);
        sub_71d0(*__errno_location(),"couldn\'t execute compress program"); // no-return
      }
    }
    v3 = fdopen(v4,"w");
    *a0 = v3;
    if (!v3)
      sub_5e10(dcgettext(NULL,"couldn\'t create temporary file",5),v2 + 0xd); // no-return
  }
  return v2;
}

// Function: sub_77e0 @ 0x77e0
void sub_77e0(unsigned long a0)
{
  sub_76b0(a0,0); // tail-call
}

// Function: sub_77f0 @ 0x77f0
unsigned long sub_77f0(void) // ternary x2
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3 [3]; // stack - 0x38
  double v4; // xmm0_qa
  double v5; // xmm0_qa
  double v6; // xmm0_qa
  double v7;
  
  v2 = 0xffffffffffffffff;
  if (!getrlimit(2,v3))
    v2 = v3[0];
  if ((!getrlimit(9,v3)) && (v3[0] < v2))
    v2 = v3[0];
  v2 >>= 1;
  if ((!getrlimit(5,v3)) && (v1 = (v3[0] & 0xfffffffffffffff0) - (v3[0] >> 4), v1 < v2))
    v2 = v1;
  sub_eb50();
  sub_ea60();
  v6 = v5 * dat_17100;
  v7 = v4;
  if (v4 <= dat_170f8 * v5)
    v7 = dat_170f8 * v5;
  if (v6 < (double)(long)v2) {
    v2 = (v6 < dat_17108) ? (unsigned long)v6 : (long)(v6 - dat_17108) ^ 0x8000000000000000;
  }
  if (v7 < (double)v2) {
    v2 = (v7 < dat_17108) ? (unsigned long)v7 : (long)(v7 - dat_17108) ^ 0x8000000000000000;
  }
  v1 = (unsigned long)dat_1e024 * 0x22;
  if ((unsigned long)dat_1e024 * 0x22 < v2)
    v1 = v2;
  return v1;
}

// Function: sub_7970 @ 0x7970
long * sub_7970(long *a0,long *a1,long a2,unsigned long a3,long a4,char a5)
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
    a1 = (long *)sub_7970(v5,v1,v7,v3,a4,1);
    v5[9] = (long)a1;
    a5 = '\0';
    a0 = v5;
  }
  v5[8] = 0;
  v5[9] = 0;
  return v1;
}

// Function: sub_7aa0 @ 0x7aa0
unsigned long * sub_7aa0(long a0,unsigned long a1,unsigned long a2)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_12eb0(a0 << 8);
  v1[6] = a1;
  v1[5] = a1;
  v1[3] = 0;
  v1[2] = 0;
  v1[1] = 0;
  *v1 = 0;
  v1[4] = 0;
  v1[7] = 0;
  *(unsigned int *)&v1[10] = 0;
  *(char *)((long)v1 + 0x54) = 0;
  pthread_mutex_init(&v1[0xb],0);
  sub_7970(v1,&v1[0x10],a2,a0,a1,0);
  return v1;
}

// Function: sub_7b40 @ 0x7b40
void sub_7b40(unsigned long *a0,long a1)
{
  *a0 = sub_e300(sub_5780,a1 * 2);
  pthread_mutex_init(&a0[1],0);
  pthread_cond_init(&a0[6],0); // tail-call
}

// Function: sub_7b70 @ 0x7b70
void sub_7b70(long a0)
{
  pthread_mutex_lock((void *)(a0 + 0x58)); // tail-call
}

// Function: sub_7b80 @ 0x7b80
void sub_7b80(unsigned long *a0,long a1)
{
  pthread_mutex_lock(&a0[1]);
  sub_e390(*a0,a1);
  *(char *)(a1 + 0x54) = 1;
  pthread_cond_signal(&a0[6]);
  pthread_mutex_unlock(&a0[1]); // tail-call
}

// Function: sub_7bc0 @ 0x7bc0
void sub_7bc0(unsigned long a0,long *a1) // return-dupe
{
  long v1;
  
  if (*(char *)((long)a1 + 0x54))
    return;
  if (*a1 != a1[2]) { // branch-flip
    if (a1[1] != a1[3]) {
      sub_7b80(); // tail-call, return-dupe
      return;
    }
    v1 = a1[6];
  }
  else {
    if (a1[1] == a1[3])
      return;
    v1 = a1[5];
  }
  if (v1)
    return;
  sub_7b80();
  return;
}

// Function: sub_7c00 @ 0x7c00
void sub_7c00(long a0)
{
  pthread_mutex_unlock((void *)(a0 + 0x58)); // tail-call
}

// Function: sub_7c10 @ 0x7c10
void sub_7c10(unsigned long a0,long a1)
{
  if (2 <= *(unsigned int *)(a1 + 0x50)) {
    sub_7b70(*(unsigned long *)(a1 + 0x38));
    sub_7bc0(a0,*(unsigned long *)(a1 + 0x38));
    sub_7c00(*(unsigned long *)(a1 + 0x38)); // tail-call
    return;
  }
  if (*(long *)(a1 + 0x30) + *(long *)(a1 + 0x28))
    return;
  sub_7b80(a0,*(unsigned long *)(a1 + 0x38)); // tail-call
}

// Function: sub_7c70 @ 0x7c70
long sub_7c70(unsigned long *a0)
{
  unsigned long *v1;
  long v2; // rax
  
  v1 = &a0[1];
  pthread_mutex_lock(v1);
  while( true ) {
    v2 = sub_e400(*a0);
    if (v2) break;
    pthread_cond_wait(&a0[6],v1);
  }
  pthread_mutex_unlock(v1);
  sub_7b70(v2);
  *(char *)(v2 + 0x54) = 0;
  return v2;
}

// Function: sub_7cd0 @ 0x7cd0
void sub_7cd0(unsigned long a0,void *a1)
{
  long v1;
  long v2; // rbx
  
  if (a0 & 0x7fffffffffffffff) {
    v1 = (long)a1 + 0x58;
    do {
      v2 = v1 + 0x80;
      pthread_mutex_destroy(v1);
      v1 = v2;
    } while (v2 != a0 * 0x100 + (long)a1 + 0x58);
  }
  free(a1); // tail-call
}

// Function: sub_7d10 @ 0x7d10
void sub_7d10(unsigned long *a0)
{
  sub_e370(*a0);
  pthread_cond_destroy(&a0[6]);
  pthread_mutex_destroy(&a0[1]); // tail-call
}

// Function: sub_7d30 @ 0x7d30
void sub_7d30(unsigned long *a0,char a1)
{
  unsigned char v1;
  char *v10; // rax
  char *v11; // rax
  unsigned long v12; // rax
  unsigned long v13; // rax
  unsigned long v14;
  char v15; // dl
  long *v16; // rdx
  long *v17; // rdx
  long v18;
  char v19 [32];
  unsigned long v2;
  char v20 [47];
  unsigned int v21; // stack - 0x78
  char v22; // stack - 0xa8
  unsigned long v23; // stack - 0x118
  unsigned char v24;
  unsigned long *v25;
  unsigned char v26;
  unsigned char v27; // stack - 0x14a
  unsigned char v28; // stack - 0x149
  long v29; // stack - 0x148
  unsigned short v3;
  unsigned char v30; // stack - 0x120
  unsigned char v31; // stack - 0x11f
  unsigned char v32; // stack - 0x11e
  char v33; // stack - 0x11c
  char v34; // stack - 0x11b
  char v35; // stack - 0x11a
  unsigned char v36; // stack - 0x119
  unsigned long v37; // stack - 0x110
  unsigned long v38; // stack - 0x108
  unsigned long v39; // stack - 0x100
  undefined3 v4;
  unsigned long v40; // stack - 0xf8
  unsigned long v41; // stack - 0xf0
  long v42; // stack - 0xe8
  unsigned long v43; // stack - 0xe0
  unsigned long v44; // stack - 0xd8
  char v5;
  char v6;
  unsigned char v7;
  unsigned char v8;
  char *v9; // rax
  
  v23 = *a0;
  v37 = a0[1];
  v38 = a0[2];
  v39 = a0[3];
  v40 = a0[4];
  v41 = a0[5];
  v14 = a0[6];
  v43 = a0[7];
  v44 = a0[8];
  v42 = v14;
  if (dat_1e410) {
    v42._1_1_ = (unsigned char)(v14 >> 8);
    v36 = 0;
    v35 = '\0';
    v42._0_1_ = (unsigned char)v14;
    v27 = v42._1_1_;
    v42._6_1_ = (unsigned char)(v14 >> 0x30);
    v42._4_1_ = (unsigned char)(v14 >> 0x20);
    v34 = '\0';
    v28 = v42._6_1_;
    v42._2_1_ = (unsigned char)(v14 >> 0x10);
    v33 = '\0';
    v42._3_1_ = (unsigned char)(v14 >> 0x18);
    v30 = v42._2_1_;
    v42._5_1_ = (unsigned char)(v14 >> 0x28);
    v29 = 1;
    v31 = v42._5_1_;
    v42._7_1_ = (unsigned char)(v14 >> 0x38);
    v32 = v42._7_1_;
    v25 = dat_1e410;
    v8 = (unsigned char)v42;
    v7 = (unsigned char)v43;
    v26 = v42._3_1_;
    v24 = v42._4_1_;
    do {
      v42 = v14;
      v5 = sub_5660(v25);
      if (v5) {
        v6 = *(char *)((long)v25 + 0x34);
        if (!v6) {
          v6 = v34;
          v33 = v5;
        }
        v34 = v6;
      }
      if (*(char *)((long)v25 + 0x39)) {
        v14 = *v25;
        v2 = v25[2];
        v18 = v14 + 1;
        if (v14 == 0xffffffffffffffff) {
          v18 = 1;
          v14 = 0;
        }
        v22 = 0x2b;
        v9 = __stpcpy_chk(v20,(char *)sub_e520(v14,v19),0x2d);
        v21 = 0x206b2d;
        v10 = __stpcpy_chk((char *)((long)&v21 + 3),(char *)sub_e520(v18,v19),0x2c);
        if (v25[2] != 0xffffffffffffffff) {
          v11 = (char *)sub_e520(v2 + 1,v19);
          v9[0] = ' ';
          v9[1] = '-';
          strcpy(&v9[2],v11);
          v9 = (char *)sub_e520((unsigned long)(v25[3] == 0xffffffffffffffff) + v2 + 1,v19);
          *v10 = ',';
          strcpy(&v10[1],v9);
        }
        v12 = sub_11420(1,&v21);
        v13 = sub_11420(0,&v22);
        error(0,0,dcgettext(NULL,"obsolescent key %s used; consider %s instead",5),v13,v12);
      }
      if ((*v25 != 0xffffffffffffffff) && (v25[2] < *v25))
        error(0,0,dcgettext(NULL,"key %lu has zero width and will be ignored",5),v29);
      v6 = sub_5660(v25);
      v5 = '\x01';
      if (!v6)
        v5 = *(char *)((long)v25 + 0x36);
      if ((v25[2]) || (!v25[3])) {
        if (v15) { // branch-flip
label_7e17:
          if (a1) goto label_7e22;
        }
        else if ((dat_1e028 == 0x80) && (((!(char)v25[6] && ((!v5 || (v25[1])))) || ((!*(char *)((long)v25 + 0x31) && (v25[3])))))) {
          error(0,0,dcgettext(NULL,"leading blanks are significant in key %lu; consider also specifying \'b\'",5),v29);
          v6 = sub_5660(v25);
        }
        if (!v6) goto label_7e22;
        v14 = *v25 + 1;
        if (!v14)
          v14 = 1;
        if (v25[2] < v14) goto label_7e22;
        error(0,0,dcgettext(NULL,"key %lu is numeric and spans multiple fields",5),v29);
        v1 = *(unsigned char *)((long)v25 + 0x34);
        if (v1) {
          v36 = v1;
          v6 = v35;
        }
        v35 = v6;
      }
      else {
        if (v15) goto label_7e17;
label_7e22:
        v1 = *(unsigned char *)((long)v25 + 0x34);
      }
      if ((v40) && (v40 == v25[4]))
        v40 = 0;
      if ((v41) && (v41 == v25[5]))
        v41 = 0;
      v24 &= v1 ^ 1;
      v42 = (unsigned long)v24 << 0x20;
      v8 &= (unsigned char)v25[6] ^ 1;
      v27 &= *(unsigned char *)((long)v25 + 0x31) ^ 1;
      v28 &= *(unsigned char *)((long)v25 + 0x36) ^ 1;
      v42._0_2_ = CONCAT11(v27,v8);
      v3 = (unsigned short)v42;
      v30 &= *(unsigned char *)((long)v25 + 0x32) ^ 1;
      v42._0_7_ = CONCAT16(v28,(undefined6)v42);
      v31 &= *(unsigned char *)((long)v25 + 0x35) ^ 1;
      v42._0_3_ = CONCAT12(v30,v3);
      v4 = (undefined3)v42;
      v26 &= *(unsigned char *)((long)v25 + 0x33) ^ 1;
      v42._0_5_ = (undefined5)CONCAT43((int)((undefined7)(undefined7)v42 >> 0x18),(undefined3)v42);
      v42._0_6_ = CONCAT15(v31,(undefined5)v42);
      v42._0_4_ = CONCAT13(v26,v4);
      v7 &= (unsigned char)v25[7] ^ 1;
      v43 = CONCAT71(v43._1_7_,v7);
      v32 &= *(unsigned char *)((long)v25 + 0x37) ^ 1;
      v42 = CONCAT17(v32,CONCAT34((undefined3)(CONCAT16(v28,(undefined6)v42) >> 0x20),(unsigned int)v42));
      v25 = (unsigned long *)v25[8];
      v29 += 1;
      v14 = v42;
    } while (v25);
    if (v35) { // branch-flip
      v8 = (unsigned char)dat_1e864;
      if (dat_1e028 != 0x80) { // branch-flip
        if (dat_1e028 == dat_1e864) {
label_8667:
          v21._0_2_ = (unsigned short)v8;
          v12 = sub_11430(&v21);
          error(0,0,dcgettext(NULL,"field separator %s is treated as a group separator in numbers",5),v12);
          goto label_8580;
        }
        v35 = '\0';
label_843e:
        if (dat_1e868 != dat_1e028) {
          if (dat_1e028 != 0x2d) { // branch-flip
            if ((dat_1e028 != 0x2b) || (!v36)) goto label_8466;
            v21 = CONCAT22(v21._2_2_,0x2b);
            v12 = sub_11430(&v21);
            v9 = "field separator %s is treated as a plus sign in numbers";
          }
          else {
            v21 = CONCAT22(v21._2_2_,0x2d);
            v12 = sub_11430(&v21);
            v9 = "field separator %s is treated as a minus sign in numbers";
          }
          error(0,0,dcgettext(NULL,v9,5),v12);
label_8466:
          if ((!v33) && (!v34)) goto label_8298;
          if (!v35) goto label_8483;
          goto label_8261;
        }
      }
      else {
        if (dat_1e864 == 0x80) {
          v35 = '\0';
          goto label_86b4;
        }
        __ctype_b_loc();
        v7 = sub_53e0((int)(char)v8);
        if (*(unsigned char *)(*v16 + (unsigned long)v7 * 2) & 1) goto label_8667;
        v35 = '\0';
label_81f3:
        __ctype_b_loc();
        v8 = sub_53e0((int)dat_1e868);
        if (!(*(unsigned char *)(*v17 + (unsigned long)v8 * 2) & 1)) goto label_8466;
      }
      v21 = CONCAT31((undefined3)(v21 >> 8),dat_1e868) & 0xffff00ff;
      v12 = sub_11430(&v21);
      error(0,0,dcgettext(NULL,"field separator %s is treated as a decimal point in numbers",5),v12);
    }
    else {
      if (v36) {
label_8580:
        if (dat_1e028 == 0x80) {
label_86b4:
          if (dat_1e864 != 0x80) goto label_81f3;
          goto label_8466;
        }
        goto label_843e;
      }
      if ((!v33) && (!v34)) goto label_8298;
label_8483:
      v9 = "";
      v21 = CONCAT31((undefined3)(v21 >> 8),dat_1e868) & 0xffff00ff;
      v12 = sub_11430(&v21);
      if (dat_1e868 != dat_1e028)
        v9 = dcgettext(NULL,"note ",5);
      error(0,0,dcgettext(NULL,"%snumbers use %s as a decimal point in this locale",5),v9,v12);
    }
label_8261:
    if ((v33) && (dat_1e862))
      error(0,0,dcgettext(NULL,"the multi-byte number group separator in this locale is not supported",5));
  }
label_8298:
  v6 = sub_5680(&v23);
  v5 = v42._7_1_;
  if (v6) { // branch-flip
    if (!v42._7_1_)
      return;
    if (dat_1e41a) { // branch-flip
      if (!dat_1e410)
        return;
      sub_56d0();
      sub_53f0(strlen((char *)&v21),"option \'-%s\' is ignored","options \'-%s\' are ignored");
      error(0,0,(char *)dcngettext(0),&v21);
      v42 = CONCAT17(1,(undefined7)v42);
    }
    else {
      if ((!dat_1e419) || (!dat_1e410)) goto label_83b5;
      sub_56d0();
      sub_53f0(strlen((char *)&v21),"option \'-%s\' is ignored","options \'-%s\' are ignored");
      error(0,0,(char *)dcngettext(0),&v21);
      v42 = CONCAT17(v5,(undefined7)v42);
    }
  }
  else {
    if ((!dat_1e41a) && (!dat_1e419))
      v42 &= 0xffffffffffffff;
    sub_56d0();
    sub_53f0(strlen((char *)&v21),"option \'-%s\' is ignored","options \'-%s\' are ignored");
    error(0,0,(char *)dcngettext(0),&v21);
    v42 = CONCAT17(v5,(undefined7)v42);
    if (!v5)
      return;
  }
  if (dat_1e41a)
    return;
label_83b5:
  if ((!dat_1e419) && (dat_1e410)) {
    error(0,0,dcgettext(NULL,"option \'-r\' only applies to last-resort comparison",5)); // tail-call
    return;
  }
}

// Function: sub_8700 @ 0x8700
long sub_8700(unsigned long *a0,unsigned long *a1,long a2,long a3)
{
  char *v1;
  unsigned long *v10; // stack - 0xf0
  unsigned int v11; // stack - 0xc0
  unsigned long v12; // stack - 0xa8
  unsigned long v2;
  int v3;
  unsigned long v4; // rcx
  long v5; // rbx
  char v6 [24];
  long v7;
  unsigned long v8;
  long v9;
  
  v7 = a3 + 2;
  if (a2) {
    v5 = 0;
    v10 = a1;
    do {
      if (v5) { // branch-flip
        v1 = (char *)*v10;
        if (strcmp(v1,"-")) // branch-flip
          v3 = stat(v1,v6);
        else {
          v3 = fstat(0,v6);
        }
        if (v3) {
          v2 = *v10; // return-dupe
          sub_5e10(dcgettext(NULL,"stat failed",5),v2); // no-return
        }
      }
      else {
        v3 = fileno((void *)*a0);
        if (fstat(v3,v6)) {
          v10 = a1;
          v2 = *v10;
          sub_5e10(dcgettext(NULL,"stat failed",5),v2);
        }
      }
      v9 = dat_1e240;
      if ((v11 & 0xf000) != 0x8000) { // branch-flip
        if (dat_1e438)
          return dat_1e438;
        v8 = 0x20000;
label_8852:
        if (!v9) {
          dat_1e240 = sub_77f0();
          v9 = dat_1e240;
        }
      }
      else {
        v8 = v12;
        if (!dat_1e240) {
          dat_1e240 = dat_1e438;
          v9 = dat_1e438;
          goto label_8852;
        }
      }
      v4 = (a3 + 1U) * v8 + 1;
      if (v4 / (a3 + 1U) != v8)
        return v9;
      if ((unsigned long)(v9 - v7) <= v4)
        return v9;
      v7 += v4;
      v5 += 1;
      v10 = &v10[1];
    } while (v5 != a2);
  }
  return v7;
}

// Function: sub_8900 @ 0x8900
char * sub_8900(unsigned int a0,unsigned long a1) // early-return
{
  int v1; // eax
  char *v2; // rax
  long v3; // stack - 0x28
  
  v1 = sub_13650(a1,0,10,&v3,"");
  if (v1 == 1) {
    v2 = (char *)0xffffffffffffffff;
    return v2;
  }
  if (!v1) {
    if (v3) {
      v2 = (char *)v3;
      return v2;
    }
    v2 = dcgettext(NULL,"number in parallel must be nonzero",5);
    error(2,0,v2);
    return v2;
  }
  sub_135d0(v1,a0,0xffffff87,0x1d5a0,a1); // no-return
}

// Function: sub_89b0 @ 0x89b0
void sub_89b0(unsigned int a0,unsigned long a1) // return-dupe, ternary
{
  int v1; // eax
  unsigned long v2; // stack - 0x30
  char *v3; // stack - 0x28
  double v4; // xmm0_qa
  double v5; // xmm0_qa
  
  v1 = sub_13650(a1,&v3,10,&v2,"EgGkKmMPtTYZ");
  if (v1) { // branch-flip
    if (((v1 != 2) || (9 < (unsigned int)((int)v3[-1] - 0x30U))) || (v3[1])) {
      sub_135d0(v1,a0,0x53,0x1d5a0,a1); // return-dupe, no-return
    }
    if (*v3 != '%') {
      if (*v3 == 'b') goto label_8a14;
      sub_135d0(v1,a0,0x53,0x1d5a0,a1);
    }
    sub_ea60();
    v5 = ((double)v2 * v4) / dat_17110;
    if (v5 < dat_17118) {
      v2 = (dat_17108 <= v5) ? (long)(v5 - dat_17108) ^ 0x8000000000000000 : (unsigned long)v5;
      goto label_8a14;
    }
  }
  else {
    if (9 < (unsigned int)((int)v3[-1] - 0x30U)) {
label_8a14:
      if (v2 < dat_1e438)
        return;
      if (v2 > (unsigned long)dat_1e024 * 0x22) {
        dat_1e438 = v2;
        return;
      }
      dat_1e438 = (unsigned long)dat_1e024 * 0x22;
      return;
    }
    if (!(v2 >> 0x36)) {
      v2 <<= 10;
      goto label_8a14;
    }
  }
  v1 = 1;
  sub_135d0(v1,a0,0x53,0x1d5a0,a1);
}

// Function: sub_8b60 @ 0x8b60
void sub_8b60(int a0,unsigned long a1)
{
  unsigned long v1;
  int v2; // eax
  unsigned long v3; // rax
  unsigned long v4; // stack - 0x60
  int v5 [7]; // stack - 0x58
  char v6 [11];
  unsigned int v7;
  
  v7 = 0x11;
  v2 = sub_13650(a1,0,10,&v4,"");
  if (!getrlimit(7,v5))
    v7 = v5[0] - 3;
  if (v2) { // branch-flip
    if (v2 != 1)
      sub_135d0(v2,a0,0xffffff84,0x1d5a0,a1); // no-return
  }
  else {
    dat_1e024 = (unsigned int)v4;
    if (v4 == (v4 & 0xffffffff)) {
      if (dat_1e024 < 2) {
        v3 = sub_11430(a1);
        v1 = *(unsigned long *)((long)a0 * 0x20 + 0x1d5a0);
        error(0,0,dcgettext(NULL,"invalid --%s argument %s",5),v1,v3);
        v3 = sub_11430("2");
        error(2,0,dcgettext(NULL,"minimum --%s argument is %s",5),v1,v3);
        return;
      }
      if (dat_1e024 <= v7)
        return;
    }
  }
  v3 = sub_11430(a1);
  v1 = *(unsigned long *)((long)a0 * 0x20 + 0x1d5a0);
  error(0,0,dcgettext(NULL,"--%s argument %s too large",5),v1,v3);
  v3 = sub_e4d0(v7,v6);
  error(2,0,dcgettext(NULL,"maximum --%s argument with current rlimit is %s",5),v1,v3);
}

// Function: sub_8d30 @ 0x8d30
void sub_8d30(void)
{
  void *v1;
  char *v10; // stack - 0x78
  char *v11; // stack - 0x70
  char *v12; // stack - 0x68
  char *v13; // stack - 0x60
  unsigned long v14; // stack - 0x58
  unsigned long v15; // stack - 0x50
  char *v16 [4]; // stack - 0xa8
  int v2; // eax
  char *v3; // rax
  char **v4; // rbx
  char *v5; // stack - 0xb8
  char *v6;
  char *v7; // stack - 0xb0
  char *v8; // stack - 0x88
  char *v9; // stack - 0x80
  
  v6 = "[";
  v4 = &v5;
  v5 = "[";
  v7 = "test invocation";
  v16[0] = "coreutils";
  v16[1] = "Multi-call invocation";
  v16[2] = "sha224sum";
  v8 = "sha256sum";
  v16[3] = "sha2 utilities";
  v9 = "sha2 utilities";
  v10 = "sha384sum";
  v11 = "sha2 utilities";
  v12 = "sha512sum";
  v13 = "sha2 utilities";
  v14 = 0;
  v15 = 0;
  do {
    if (!strcmp("sort",v6)) break;
    v6 = *(char **)((long)v4 + 0x10);
    v4 = (char *)((long)v4 + 0x10);
  } while (v6);
  v6 = *(char **)((long)v4 + 8);
  if (!v6)
    v6 = "sort";
  __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  v3 = setlocale(5,NULL);
  if ((v3) && (v2 = strncmp(v3,"en_",3), v1 = stdout, v2))
    fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
  v3 = "sort";
  if (!strcmp("sort","["))
    v3 = "test";
  __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v3);
  v3 = "";
  if (v6 == "sort")
    v3 = " invocation";
  __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v6,v3); // tail-call
}

// Function: sub_8f60 @ 0x8f60
void sub_8f60(void)
{
  dat_1e130 = 2;
}

// Function: sub_8f70 @ 0x8f70
char * sub_8f70(char *a0,long a1,long a2)
{
  char v1;
  bool v10; // cf
  long v2; // rax
  char *v3; // rdx
  char *v4; // rdx
  char *v5; // rdx
  char *v6;
  char *v7; // rdx
  long v8;
  long v9;
  
  v6 = &a0[a1 + -1];
  v8 = *(long *)(a2 + 0x18);
  v2 = *(long *)(a2 + 0x10) + (unsigned long)(v8 == 0);
  if (dat_1e028 != 0x80) { // branch-flip
    if ((a0 < v6) && (v9 = v2 + -1, v2)) {
      do {
        while (dat_1e028 == *a0) {
          if (v6 <= a0) goto label_8fc1;
          if (!v8 && !v9)
            return a0;
          a0 = &a0[1];
          if ((v6 <= a0) || (v10 = v9 == 0, v9 = v9 + -1, v10)) goto label_8fc1;
        }
        a0 = &a0[1];
      } while (a0 < v6);
    }
  }
  else if ((a0 < v6) && (v9 = v2 + -1, v2)) {
    v1 = *a0;
    v2 = 0x1e740;
    v1 = *(char *)(v2 + (unsigned long)(unsigned char)sub_53e0((int)v1));
    v6 = v4;
    while( true ) {
      while (v1) {
        a0 = &a0[1];
        if (v6 <= a0) goto label_8fc1;
        v1 = *a0;
        v1 = *(char *)(v2 + (unsigned long)(unsigned char)sub_53e0((int)v1));
        v6 = v5;
      }
      if (v6 <= a0) break;
      do {
        a0 = &a0[1];
        if (v6 == a0) goto label_8fc1;
        v1 = *a0;
        v1 = *(char *)(v2 + (unsigned long)(unsigned char)sub_53e0((int)v1));
        v6 = v7;
      } while (!v1);
      if ((v7 <= a0) || (v10 = v9 == 0, v9 = v9 + -1, v10)) break;
    }
  }
label_8fc1:
  if (v8) {
    if ((*(char *)(a2 + 0x31)) && (v2 = 0x1e740, a0 < v6)) {
      do {
        v1 = *a0;
        v6 = v3;
        if (!*(char *)(v2 + (unsigned long)(unsigned char)sub_53e0((int)v1))) break;
        a0 = &a0[1];
      } while (v3 != a0);
    }
    a0 = &a0[v8];
    if (v6 <= a0)
      a0 = v6;
  }
  return a0;
}

// Function: sub_90d0 @ 0x90d0
char * sub_90d0(char *a0,long a1,long *a2)
{
  unsigned char v1;
  long v10;
  bool v11; // cf
  char *v2; // rax
  char *v3; // rdx
  char *v4; // rdx
  char *v5; // rdx
  char *v6;
  char *v7; // rdx
  long v8;
  long v9;
  
  v6 = &a0[a1 + -1];
  v8 = *a2;
  v10 = a2[1];
  if (dat_1e028 != 0x80) { // branch-flip
    if (v6 <= a0) goto label_9118;
    v9 = v8 + -1;
    if (v8) {
      do {
        while (dat_1e028 == *a0) {
          if ((v6 <= a0) || (a0 = &a0[1], v6 <= a0)) goto label_9118;
          v11 = v9 == 0;
          v9 -= 1;
          if (v11) goto label_9144;
        }
        a0 = &a0[1];
      } while (a0 < v6);
      goto label_9118;
    }
  }
  else {
    if (v6 <= a0) goto label_9118;
    v9 = v8 + -1;
    if (v8) {
      v8 = 0x1e740;
      v1 = sub_53e0((int)*a0);
      v2 = (unsigned long)(unsigned long)v1;
      v1 = *(char *)(v8 + (long)v2);
      v6 = v4;
      do {
        while (v1) {
          a0 = &a0[1];
          if (v6 <= a0) {
            v2 = &a0[v10];
            if (v6 <= &a0[v10])
              v2 = v6;
            return v2;
          }
          v1 = sub_53e0((int)*a0);
          v2 = (unsigned long)(unsigned long)v1;
          v1 = *(char *)(v8 + (long)v2);
          v6 = v5;
        }
        if (v6 <= a0) goto label_9118;
        do {
          a0 = &a0[1];
          if (v6 == a0) goto label_9118;
          v1 = sub_53e0((int)*a0);
          v2 = (unsigned long)(unsigned long)v1;
          v1 = *(char *)(v8 + (long)v2);
          v6 = v7;
        } while (!v1);
        if (v7 <= a0) goto label_9118;
        v11 = v9 != 0;
        v9 -= 1;
      } while (v11);
    }
  }
label_9144:
  v8 = 0x1e740;
  if ((char)a2[6]) {
    do {
      v1 = sub_53e0((int)*a0);
      v2 = (unsigned long)(unsigned long)v1;
      if (!*(char *)(v8 + (long)v2)) {
        v2 = &a0[v10];
        if (v3 <= v2)
          v2 = v3;
        return v2;
      }
      a0 = &a0[1];
      v6 = v3;
    } while (a0 < v3);
  }
label_9118:
  v2 = &a0[v10];
  if (v6 <= &a0[v10])
    v2 = v6;
  return v2;
}

// Function: sub_9230 @ 0x9230
void sub_9230(unsigned char *a0,long a1,long *a2)
{
  unsigned char v1;
  unsigned char *v10; // stack - 0x40
  unsigned char v2;
  long v3;
  char v4; // al
  unsigned char *v5;
  unsigned char *v6;
  unsigned long v7; // rax
  long v8; // rdx
  unsigned char *v9; // stack - 0x38
  
  v6 = &a0[a1 + -1];
  v5 = a0;
  if (!a2) goto label_9376;
  v3 = *a2;
  if (v3 != -1) { // branch-flip
    v5 = (unsigned char *)sub_90d0(a0,a1,a2);
    if (a2[2] != -1) {
label_9282:
      v6 = (unsigned char *)sub_8f70(a0,a1,a2);
      goto label_9293;
    }
label_92a0:
    if ((!*(char *)((long)a2 + 0x36)) && (v4 = sub_5660(a2), !v4)) goto label_9376;
  }
  else {
    if (a2[2] != -1) goto label_9282;
label_9293:
    if ((!(char)a2[6]) || (v3 != -1)) goto label_92a0;
  }
  v1 = *v6;
  *v6 = 0;
  while (v2 = *v5, *(char *)(v8 + (unsigned long)(unsigned char)sub_53e0((int)(char)v2))) {
    v5 = &v5[1];
  }
  if (v5 <= v6) { // branch-flip
    v10 = v5;
    if (*(char *)((long)a2 + 0x36)) // branch-flip
      sub_5570(v5,&v10);
    else if (*(char *)((long)a2 + 0x34)) // branch-flip
      strtold(v5,&v10);
    else {
      if (!(a2[6] & 0xff0000ff0000U)) goto label_9368;
      v9 = v5;
      if (v5 < v6)
        v9 = &v5[*v5 == 0x2d];
      if ('0' <= (char)sub_5430(&v9)) {
        v10 = v9;
        if (*(char *)((long)a2 + 0x35))
          v10 = &v9[*(char *)((unsigned long)*v9 + 0x15360) != '\0'];
      }
    }
  }
  else {
label_9368:
    v10 = v6;
  }
  *v6 = v1;
  v6 = v10;
label_9376:
  v7 = sub_62b0(a0,v5);
  sub_62f0(v7,sub_62b0(v5,v6)); // tail-call
  return;
}

// Function: sub_9400 @ 0x9400
unsigned long sub_9400(unsigned long *a0)
{
  unsigned long v1; // rax
  long v2;
  unsigned long v3; // stack - 0x18
  
  v2 = dat_1e410;
  do {
    sub_9230(*a0,a0[1],v2);
    if (!v2)
      return v3;
    v2 = *(long *)(v2 + 0x40);
  } while (v2);
  if ((!dat_1e419) && (!dat_1e41a)) {
    v1 = sub_9230(*a0,a0[1],0); // tail-call
    return v1;
  }
  return v3;
}

// Function: sub_9460 @ 0x9460
void sub_9460(unsigned long *a0,void *a1,long a2)
{
  char *v1;
  char v2;
  unsigned long v3;
  char *v4;
  int v5;
  
  v4 = (char *)*a0;
  v3 = a0[1];
  v1 = &v4[v3];
  if ((!a2) && (dat_1e400)) {
    do {
      if (v1 <= v4) {
        sub_9400(a0); // tail-call
        return;
      }
      v2 = *v4;
      v4 = &v4[1];
      v5 = 0x3e;
      if ((v2 != '\t') && (v5 = 10, v1 != v4))
        v5 = (int)v2;
    } while (fputc_unlocked(v5,a1) != -1);
    sub_5e10(dcgettext(NULL,"write failed",5),0); // no-return
  }
  v1[-1] = dat_1e038;
  if (fwrite_unlocked(v4,1,v3,a1) != v3)
    sub_5e10(dcgettext(NULL,"write failed",5),a2); // no-return
  v1[-1] = '\0';
}

// Function: sub_9550 @ 0x9550
unsigned long sub_9550(unsigned long *a0,long *a1)
{
  long v1;
  char v10; // dl
  char v11;
  char v12; // dl
  int v13; // edx
  unsigned int v14; // edx
  long v15; // rdx
  long v16;
  char v17 [4007];
  long *v18;
  long v19;
  unsigned char v2; // al
  long v20;
  unsigned long v21; // r12
  unsigned long v22; // r13
  char *v23;
  bool v24; // cf
  char v25; // stack - 0x1002
  char v26; // stack - 0x1001
  char *v27; // stack - 0xff8
  unsigned int v3; // eax
  int v4; // eax
  char *v5;
  char *v6;
  char *v7;
  char *v8;
  unsigned long v9;
  
  v7 = (char *)a0[2];
  v8 = (char *)a1[2];
  v5 = (char *)a0[3];
  v6 = (char *)a1[3];
  v18 = dat_1e410;
label_95a0:
  v19 = v18[5];
  v1 = v18[4];
  if (v5 <= v7)
    v5 = v7;
  if (v6 <= v8)
    v6 = v8;
  v22 = (long)v5 - (long)v7;
  v21 = (long)v6 - (long)v8;
  if ((((dat_1e861) || (sub_5660(v18))) || (v18[6] & 0xff0000ff000000U)) || ((char)v18[7])) {
    if (v19 || v1) { // branch-flip
      v27 = NULL;
      if (0xfa1 <= v22 + 2 + v21) { // branch-flip
        v6 = (char *)sub_12eb0();
        v27 = v6;
        v5 = &v6[v22 + 1];
        if (!v22) goto label_9abb;
label_9986:
        v23 = &v7[v22];
        v22 = 0;
        v20 = v19;
        do {
          v11 = *v7;
          if (v1) { // branch-flip
            if (!*(char *)(v1 + (unsigned long)(unsigned char)sub_53e0((int)v11))) {
              v11 = v10;
              if (v20) goto label_99c8;
              goto label_99ab;
            }
          }
          else {
label_99c8:
            v11 = *(char *)(v20 + (unsigned long)(unsigned char)sub_53e0());
label_99ab:
            v6[v22] = v11;
            v22 += 1;
          }
          v7 = &v7[1];
        } while (v23 != v7);
        v6[v22] = '\0';
        v7 = v5;
        if (v21) goto label_99f0;
label_9ad0:
        v8 = v5;
      }
      else {
        v5 = &v17[v22];
        v6 = &v16;
        if (v22) goto label_9986;
label_9abb:
        *v6 = '\0';
        v7 = v5;
        if (!v21) goto label_9ad0;
label_99f0:
        v5 = &v8[v21];
        v21 = 0;
        v20 = v19;
        do {
          v11 = *v8;
          if (v1) { // branch-flip
            if (!*(char *)(v1 + (unsigned long)(unsigned char)sub_53e0((int)v11))) {
              v11 = v12;
              if (v20) goto label_9a30;
              goto label_9a13;
            }
          }
          else {
label_9a30:
            v11 = *(char *)(v20 + (unsigned long)(unsigned char)sub_53e0());
label_9a13:
            v7[v21] = v11;
            v21 += 1;
          }
          v8 = &v8[1];
        } while (v5 != v8);
        v5 = &v7[v21];
        v8 = v7;
      }
      *v5 = '\0';
    }
    else {
      v25 = *v5;
      *v5 = '\0';
      v26 = *v6;
      *v6 = '\0';
      v6 = v7;
    }
    if (*(char *)((long)v18 + 0x32)) { // branch-flip
      v9 = sub_64d0(v6,v8);
label_977e:
      if (!v19 && !v1) {
label_978a:
        v6[v22] = v25;
        v8[v21] = v26;
        goto label_979c;
      }
    }
    else {
      if (*(char *)((long)v18 + 0x34)) {
        v9 = sub_63d0(v6,v8);
        goto label_977e;
      }
      if (!*(char *)((long)v18 + 0x35)) {
        if (*(char *)((long)v18 + 0x36)) { // branch-flip
          v4 = sub_5570(v6,0);
          v9 = (unsigned long)(unsigned int)(v4 - sub_5570(v8,0));
        }
        else if (*(char *)((long)v18 + 0x33)) // branch-flip
          v9 = sub_6680(v6,v22,v8,v21);
        else if ((char)v18[7]) // branch-flip
          v9 = sub_cd70(v6,v22,v8,v21);
        else if (v22) { // branch-flip
          v9 = 1;
          if (v21)
            v9 = sub_133e0(v6,v22 + 1,v8,v21 + 1);
        }
        else {
          v9 = (unsigned long)-(unsigned int)(v21 != 0);
        }
        goto label_977e;
      }
      v9 = sub_6530(v6,v8);
      if (!v19 && !v1) goto label_978a;
    }
    v4 = (int)v9;
    free(v27);
    v9 &= 0xffffffff;
    goto label_9874;
  }
  if (v1) {
    if (!v19) {
      do {
        if ((v5 <= v7) || (v11 = *v7, !*(char *)(v1 + (unsigned long)(unsigned char)sub_53e0((int)v11)))) {
          if (v6 <= v8) goto label_9c50;
          while (v11 = *v8, *(char *)(v1 + (sub_53e0((int)v11) & 0xff))) {
            v8 = &v8[1];
            if (v6 == v8) goto label_9c50;
          }
          if ((v5 <= v7) || (v6 <= v8)) goto label_9c50;
          v11 = *v7;
          v3 = (unsigned int)(unsigned char)sub_53e0((int)v11) - v13;
          v9 = (unsigned long)v3;
          if (v3) goto label_9880;
          v8 = &v8[1];
        }
        v7 = &v7[1];
      } while( true );
    }
    do {
      if ((v5 <= v7) || (v11 = *v7, !*(char *)(v1 + (unsigned long)(unsigned char)sub_53e0((int)v11)))) {
        if (v6 <= v8) goto label_9bc0;
        while (v11 = *v8, *(char *)(v1 + (unsigned long)(unsigned char)sub_53e0((int)v11))) {
          v8 = &v8[1];
          if (v6 == v8) goto label_9bc0;
        }
        if ((v5 <= v7) || (v6 <= v8)) goto label_9bc0;
        v11 = *v7;
        v3 = sub_53e0((int)*(char *)(v19 + (unsigned long)(unsigned char)sub_53e0((int)v11)));
        v2 = sub_53e0((int)*(char *)(v19 + v15));
        v3 = (v3 & 0xff) - (unsigned int)v2;
        if (v3) {
          v11 = *(char *)((long)v18 + 0x37);
          v9 = (unsigned long)v3;
          goto label_9be0;
        }
        v8 = &v8[1];
      }
      v7 = &v7[1];
    } while( true );
  }
  if (!v22) {
    v9 = (unsigned long)-(unsigned int)(v21 != 0);
    goto label_979c;
  }
  if (!v21) {
label_9baf:
    v9 = 1;
    goto label_9880;
  }
  if (v19) { // branch-flip
    while ((v7 < v5 && (v8 < v6))) {
      v11 = *v7;
      v23 = &v8[1];
      v7 = &v7[1];
      sub_53e0((int)*(char *)(v19 + (unsigned long)(unsigned char)sub_53e0((int)v11)));
      v11 = *v8;
      v3 = sub_53e0((int)*(char *)(v19 + (unsigned long)(unsigned char)sub_53e0((int)v11)));
      v3 = (v14 & 0xff) - (v3 & 0xff);
      v9 = (unsigned long)v3;
      v8 = v23;
      if (v3) goto label_9880;
    }
  }
  else {
    v9 = v21;
    if (v22 <= v21)
      v9 = v22;
    v4 = memcmp(v7,v8,v9);
    v9 = CONCAT44(dat_4,v4);
    if (v4) goto label_9880;
  }
  if (v22 < v21) {
    v9 = 0xffffffff;
    goto label_9880;
  }
  if (v22 != v21) goto label_9baf;
  goto label_97a4;
label_9c50:
  v24 = v7 < v5;
  goto label_9bc3;
label_9bc0:
  v24 = v7 < v5;
label_9bc3:
  v9 = (unsigned long)((unsigned int)v24 - (unsigned int)(v8 < v6));
label_979c:
  v4 = (int)v9;
label_9874:
  if (v4) {
label_9880:
    v11 = *(char *)((long)v18 + 0x37);
label_9be0:
    if (v11)
      return (unsigned long)(unsigned int)-(int)v9;
    return v9;
  }
label_97a4:
  v18 = (long *)v18[8];
  if (!v18)
    return 0;
  v7 = (char *)*a0;
  v19 = a0[1];
  if (v18[2] != -1) { // branch-flip
    v5 = (char *)sub_8f70(v7,v19,v18);
    v6 = (char *)sub_8f70(*a1,a1[1],v18);
  }
  else {
    v5 = &v7[v19 + -1];
    v6 = (char *)(*a1 + -1 + a1[1]);
  }
  if (*v18 != -1) { // branch-flip
    v7 = (char *)sub_90d0(v7,v19,v18);
    v8 = (char *)sub_90d0(*a1,a1[1],v18);
  }
  else {
    v8 = (char *)*a1;
    if ((char)v18[6]) {
      if (v7 < v5) {
        do {
          v11 = *v7;
          v19 = 0x1e740;
          if (!*(char *)(v19 + (unsigned long)(unsigned char)sub_53e0((int)v11))) break;
          v7 = &v7[1];
        } while (v5 != v7);
      }
      if (v8 < v6) {
        do {
          v11 = *v8;
          v19 = 0x1e740;
          if (!*(char *)(v19 + (unsigned long)(unsigned char)sub_53e0((int)v11))) goto label_95a0;
          v8 = &v8[1];
        } while (v6 != v8);
      }
    }
  }
  goto label_95a0;
}

// Function: sub_9c70 @ 0x9c70
unsigned long sub_9c70(unsigned long *a0,unsigned long *a1)
{
  long v1;
  long v2;
  int v3; // eax
  unsigned long v4;
  unsigned long v5;
  unsigned long v6; // r15
  
  if (dat_1e410) {
    v4 = sub_9550();
    if ((int)v4)
      return v4;
    if (dat_1e419)
      return v4;
    if (dat_1e41a)
      return v4;
  }
  v1 = a0[1];
  v2 = a1[1];
  v4 = v2 - 1;
  v6 = v1 - 1;
  if (v6) { // branch-flip
    v5 = 1;
    if (v4) {
      if (dat_1e861) // branch-flip
        v5 = sub_133e0((void *)*a0,v1,(void *)*a1,v2);
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
  if (dat_1e41b)
    v5 = (unsigned long)(unsigned int)-(int)v5;
  return v5;
}

// Function: sub_9d40 @ 0x9d40
void sub_9d40(unsigned long *a0,unsigned long a1,long a2)
{
  unsigned long v1;
  long v10; // stack - 0x40
  unsigned long v2;
  unsigned long v3;
  long v4;
  unsigned long *v5;
  unsigned long *v6; // rbp
  unsigned long v7; // r12
  long v8; // r13
  unsigned long *v9;
  
  v7 = a1 >> 1;
  v10 = a1 - v7;
  v5 = &(&a0[v7 * -4])[-4];
  v4 = a2 + -0x20;
  v9 = &a0[v7 * -4];
  while( true ) {
    while( true ) {
      v8 = v4;
      v6 = v5;
      v5 = &a0[-4];
      if ((int)sub_9c70(v8,v6) <= 0) break;
      v1 = v9[-3];
      v10 -= 1;
      *v5 = v9[-4];
      a0[-3] = v1;
      v1 = v9[-1];
      a0[-2] = v9[-2];
      a0[-1] = v1;
      if (!v10) {
        v4 = -0x20;
        do {
          v1 = ((unsigned long *)(a2 + v4))[1];
          v9 = (unsigned long *)(a2 + 0x10 + v4);
          v2 = *v9;
          v3 = v9[1];
          *(unsigned long *)((long)v5 + v4) = *(unsigned long *)(a2 + v4);
          ((unsigned long *)((long)v5 + v4))[1] = v1;
          v9 = (unsigned long *)((long)a0 + v4 + -0x10);
          *v9 = v2;
          v9[1] = v3;
          v4 -= 0x20;
          v7 -= 1;
        } while (v7);
        return;
      }
      a0 = v5;
      v5 = &v6[-4];
      v4 = v8;
      v9 = v6;
    }
    v1 = *(unsigned long *)(a2 + -0x18);
    v2 = *(unsigned long *)(a2 + -0x10);
    v3 = *(unsigned long *)(a2 + -8);
    *v5 = *(unsigned long *)(a2 + -0x20);
    a0[-3] = v1;
    a0[-2] = v2;
    a0[-1] = v3;
    v7 -= 1;
    if (!v7) break;
    a0 = v5;
    v5 = v6;
    v4 = v8 + -0x20;
    a2 = v8;
  }
}

// Function: sub_9e20 @ 0x9e20
void sub_9e20(long a0,unsigned long a1,long a2,char a3) // return-dupe
{
  unsigned long v1;
  unsigned long *v10; // rax
  unsigned long *v11; // rdx
  long v12;
  unsigned long v13; // r14
  unsigned long v2;
  unsigned long v3;
  unsigned long v4;
  unsigned long v5;
  unsigned long v6;
  unsigned long v7;
  unsigned long v8;
  int v9; // eax
  
  if (a1 != 2) {
    v13 = a1 >> 1;
    v12 = a0 + v13 * -0x20;
    if (a3) { // branch-flip
      sub_9e20(v12,a1 - v13,v13 * -0x20 + a2,1);
      v12 = a0;
      if (4 <= a1)
        sub_9e20(a0,v13,a2,0);
    }
    else {
      sub_9e20(v12,a1 - v13,a2,0);
      v12 = a2; // branch-flip
      if (4 <= a1) {
        sub_9e20(a0,v13,a2,1);
        a2 = a0;
      }
      else {
        v1 = *(unsigned long *)(a0 + -0x18);
        v2 = *(unsigned long *)(a0 + -0x10);
        v3 = *(unsigned long *)(a0 + -8);
        *(unsigned long *)(a2 + -0x20) = *(unsigned long *)(a0 + -0x20);
        *(unsigned long *)(a2 + -0x18) = v1;
        *(unsigned long *)(a2 + -0x10) = v2;
        *(unsigned long *)(a2 + -8) = v3;
        a2 = a0;
      }
    }
    sub_9d40(a2,a1,v12); // tail-call
    return;
  }
  v9 = sub_9c70(a0 + -0x20,a0 + -0x40);
  if (a3) {
    v10 = (unsigned long *)((long)(int)((unsigned int)(0 < v9) - 2) * 0x20 + a0);
    v11 = (unsigned long *)((long)(int)~(unsigned int)(0 < v9) * 0x20 + a0);
    v1 = v10[1];
    v2 = v10[2];
    v3 = v10[3];
    v4 = *v11;
    v5 = v11[1];
    v6 = v11[2];
    v7 = v11[3];
    *(unsigned long *)(a2 + -0x40) = *v10;
    *(unsigned long *)(a2 + -0x38) = v1;
    *(unsigned long *)(a2 + -0x20) = v4;
    *(unsigned long *)(a2 + -0x18) = v5;
    *(unsigned long *)(a2 + -0x10) = v6;
    *(unsigned long *)(a2 + -8) = v7;
    *(unsigned long *)(a2 + -0x30) = v2;
    *(unsigned long *)(a2 + -0x28) = v3;
    return;
  }
  if (v9 <= 0)
    return;
  v1 = *(unsigned long *)(a0 + -0x20);
  v2 = *(unsigned long *)(a0 + -0x18);
  v3 = *(unsigned long *)(a0 + -0x10);
  v4 = *(unsigned long *)(a0 + -8);
  v5 = *(unsigned long *)(a0 + -0x40);
  v6 = *(unsigned long *)(a0 + -0x38);
  v7 = *(unsigned long *)(a0 + -0x30);
  v8 = *(unsigned long *)(a0 + -0x28);
  *(unsigned long *)(a2 + -0x20) = v1;
  *(unsigned long *)(a2 + -0x18) = v2;
  *(unsigned long *)(a2 + -0x10) = v3;
  *(unsigned long *)(a2 + -8) = v4;
  *(unsigned long *)(a0 + -0x20) = v5;
  *(unsigned long *)(a0 + -0x18) = v6;
  *(unsigned long *)(a0 + -0x10) = v7;
  *(unsigned long *)(a0 + -8) = v8;
  *(unsigned long *)(a0 + -0x40) = v1;
  *(unsigned long *)(a0 + -0x38) = v2;
  *(unsigned long *)(a0 + -0x30) = v3;
  *(unsigned long *)(a0 + -0x28) = v4;
}

// Function: sub_9fb0 @ 0x9fb0
void sub_9fb0(long *a0,unsigned long a1,unsigned long a2)
{
  if (dat_1e419) {
    if (dat_1e840) {
      if (!sub_9c70(a0,0x1e840))
        return;
    }
    dat_1e840 = *a0;
    dat_1e848 = a0[1];
    dat_1e850 = a0[2];
    dat_1e858 = a0[3];
  }
  sub_9460(a0,a1,a2); // tail-call
}

// Function: sub_a020 @ 0xa020
void sub_a020(long *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  unsigned long *v1;
  long v10;
  long v11;
  long v12;
  long v13; // r8
  unsigned long *v14;
  long v15;
  unsigned long *v16; // rdx
  unsigned long *v17;
  unsigned long *v2;
  unsigned long v3;
  unsigned long v4;
  unsigned long *v5;
  unsigned long *v6;
  unsigned long *v7;
  long v8;
  unsigned long *v9;
  
  v1 = (unsigned long *)*a0;
  v2 = (unsigned long *)a0[1];
  v7 = (unsigned long *)a0[2];
  v10 = (a1 >> ((char)*(unsigned int *)&a0[10] * '\x02' + 2U & 0x3f)) + 1;
  v6 = v1;
  if (*(unsigned int *)&a0[10] < 2) {
    v12 = 0;
    v11 = 0;
    v9 = v2;
    v15 = v10;
    if (v1 != v7) { // branch-flip
      while ((unsigned long *)a0[3] != v9) {
        v10 = v15 + -1;
        if (!v15) {
          v15 = -1;
          break;
        }
        if (1 <= (int)sub_9c70(&v6[-4],&v9[-4])) { // branch-flip
          v12 = a0[1];
          a0[1] = v12 + -0x20;
          sub_9fb0(v12 + -0x20,a2,a3);
          v6 = (unsigned long *)*a0;
          v9 = (unsigned long *)a0[1];
          if (v6 == (unsigned long *)a0[2]) goto label_a1d9;
        }
        else {
          v12 = *a0;
          *a0 = v12 + -0x20;
          sub_9fb0(v12 + -0x20,a2,a3);
          v6 = (unsigned long *)*a0;
          v9 = (unsigned long *)a0[1];
          if (v6 == (unsigned long *)a0[2]) {
label_a1d9:
            v11 = (long)v1 - (long)v6 >> 5;
            v12 = (long)v2 - (long)v9 >> 5;
            goto label_a1f2;
          }
        }
        v15 = v10;
      }
      v12 = (long)v2 - (long)v9 >> 5;
      if (a0[6] == v12) {
        if (((unsigned long *)a0[2] != v6) && (v15)) {
          do {
            v15 -= 1;
            *a0 = (long)&v6[-4];
            sub_9fb0(&v6[-4],a2,a3);
            v6 = (unsigned long *)*a0;
            if (v6 == (unsigned long *)a0[2]) break;
          } while (v15);
          v8 = a0[5];
          v12 = a0[6];
          v15 = (long)v2 - a0[1] >> 5;
        }
        else {
          v8 = a0[5];
          v15 = v12;
        }
        goto label_a13a;
      }
      v8 = a0[5];
      v11 = (long)v1 - (long)v6 >> 5;
    }
    else {
label_a1f2:
      v8 = a0[5];
      v15 = v10;
      if (a0[6] == v12) {
        v15 = v12;
        goto label_a13a;
      }
    }
    if (((v8 != v11) || (v9 == (unsigned long *)a0[3])) || (!v15)) {
      v12 = a0[6];
      v15 = (long)v2 - a0[1] >> 5;
    }
    else {
      do {
        v15 -= 1;
        a0[1] = (long)&v9[-4];
        sub_9fb0(&v9[-4],a2,a3);
        v9 = (unsigned long *)a0[1];
        if (v9 == (unsigned long *)a0[3]) break;
      } while (v15);
      v6 = (unsigned long *)*a0;
      v8 = a0[5];
      v12 = a0[6];
      v15 = (long)v2 - (long)v9 >> 5;
    }
    goto label_a13a;
  }
  v14 = (unsigned long *)a0[4];
  v15 = 0;
  v13 = 0;
  v17 = (unsigned long *)*v14;
  v5 = v1;
  v9 = v2;
  v11 = v10;
  if (v1 != v7) { // branch-flip
    while ((unsigned long *)a0[3] != v9) {
      v10 = v11 + -1;
      if (!v11) {
        v14 = (unsigned long *)a0[4];
        v11 = -1;
        v6 = (unsigned long *)*a0;
        goto label_a2dd;
      }
      v16 = &v17[-4];
      if (1 <= (int)sub_9c70(&v5[-4],&v9[-4])) { // branch-flip
        v12 = a0[1];
        v7 = (unsigned long *)a0[2];
        v3 = *(unsigned long *)(v12 + -0x20);
        v4 = *(unsigned long *)(v12 + -0x18);
        v9 = (unsigned long *)(v12 + -0x20);
        a0[1] = (long)v9;
        *v16 = v3;
        v17[-3] = v4;
        v3 = *(unsigned long *)(v12 + -8);
        v5 = (unsigned long *)*a0;
        v17[-2] = *(unsigned long *)(v12 + -0x10);
        v17[-1] = v3;
        if (v7 == v5) goto label_a104;
      }
      else {
        v12 = *a0;
        v7 = (unsigned long *)a0[2];
        v3 = *(unsigned long *)(v12 + -0x20);
        v4 = *(unsigned long *)(v12 + -0x18);
        v5 = (unsigned long *)(v12 + -0x20);
        *a0 = (long)v5;
        *v16 = v3;
        v17[-3] = v4;
        v3 = *(unsigned long *)(v12 + -8);
        v9 = (unsigned long *)a0[1];
        v17[-2] = *(unsigned long *)(v12 + -0x10);
        v17[-1] = v3;
        if (v7 == v5) {
label_a104:
          v14 = (unsigned long *)a0[4];
          v13 = (long)v1 - (long)v5 >> 5;
          v15 = (long)v2 - (long)v9 >> 5;
          v6 = v5;
          v17 = v16;
          goto label_a121;
        }
      }
      v11 = v10;
      v17 = v16;
    }
    v14 = (unsigned long *)a0[4];
    v6 = (unsigned long *)*a0;
label_a2dd:
    v15 = (long)v2 - (long)v9 >> 5;
    v13 = (long)v1 - (long)v5 >> 5;
    v12 = a0[6];
    v8 = a0[5];
    if (v12 != v15) goto label_a12e;
    if ((v7 != v6) && (v10 = v11 + -1, v9 = v17, v2 = v6, v11)) {
      do {
        v6 = &v2[-4];
        v3 = *v6;
        v4 = v2[-3];
        v17 = &v9[-4];
        *a0 = (long)v6;
        *v17 = v3;
        v9[-3] = v4;
        v3 = v2[-1];
        v9[-2] = v2[-2];
        v9[-1] = v3;
        if (v6 == v7) goto label_a137;
        v10 -= 1;
        v9 = v17;
        v2 = v6;
      } while (v10 != -1);
    }
  }
  else {
label_a121:
    v12 = a0[6];
    v8 = a0[5];
    v11 = v10;
    if (v15 != v12) {
label_a12e:
      if (((v13 == v8) && (v7 = (unsigned long *)a0[3], v7 != v9)) && (v10 = v11 + -1, v5 = v17, v11)) {
        do {
          v16 = &v9[-4];
          v3 = *v16;
          v4 = v9[-3];
          v17 = &v5[-4];
          a0[1] = (long)v16;
          *v17 = v3;
          v5[-3] = v4;
          v3 = v9[-1];
          v5[-2] = v9[-2];
          v5[-1] = v3;
          if (v16 == v7) {
            v15 = (long)v2 - (long)v16 >> 5;
            goto label_a137;
          }
          v10 -= 1;
          v5 = v17;
          v9 = v16;
        } while (v10 != -1);
        v15 = (long)v2 - (long)v16 >> 5;
      }
    }
  }
label_a137:
  *v14 = v17;
label_a13a:
  a0[6] = v12 - v15;
  a0[5] = v8 - ((long)v1 - (long)v6 >> 5);
  return;
}

// Function: sub_a460 @ 0xa460
void sub_a460(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  long v1; // rax
  
  while( true ) {
    v1 = sub_7c70(a0);
    if (!*(int *)(v1 + 0x50)) break;
    sub_a020(v1,a1,a2,a3);
    sub_7bc0(a0,v1);
    sub_7c10(a0,v1);
    sub_7c00(v1);
  }
  sub_7c00(v1);
  sub_7b80(a0,v1); // tail-call
}

// Function: sub_a4e0 @ 0xa4e0
void sub_a4e0(long a0,unsigned long a1,long a2,long *a3,unsigned long a4,unsigned long a5,unsigned long a6)
{
  unsigned long v1;
  long v10; // stack - 0x68
  long v11; // stack - 0x60
  unsigned long v12; // stack - 0x58
  unsigned long v13; // stack - 0x50
  unsigned long v14; // stack - 0x48
  long v2; // stack - 0x78
  unsigned long v3; // stack - 0x80
  unsigned long v4;
  long v5; // r10
  unsigned long v6; // r11
  long v7; // r11
  unsigned long v8;
  unsigned long v9; // stack - 0x70
  
  v1 = a6;
  v6 = a1 >> 1;
  v4 = a3[5];
  v8 = a3[6];
  v11 = a3[8];
  v14 = a6;
  v2 = a0;
  v9 = v6;
  v10 = a2;
  v12 = a4;
  v13 = a5;
  if ((0x20000 <= v4 + v8) && (2 <= a1)) {
    if (!pthread_create(&v3,0,sub_a710,&v2)) {
      sub_a4e0(a0 + a3[5] * -0x20,a1 - v6,a2,a3[9],a4,a5,v1);
      pthread_join(v3,0);
      return;
    }
    v4 = a3[5];
    v8 = a3[6];
  }
  v7 = a0 + a2 * -0x20;
  v5 = a0 + v4 * -0x20;
  if (2 <= v8)
    sub_9e20(v5,v8,v7 + (v4 >> 1) * -0x20,0);
  if (2 <= v4)
    sub_9e20(a0,v4,v7,0);
  *a3 = a0;
  a3[1] = v5;
  a3[2] = v5;
  a3[3] = a0 + v4 * -0x20 + v8 * -0x20;
  sub_7b80(a4,a3);
  sub_a460(a4,a2,a5,v1);
}

// Function: sub_a710 @ 0xa710
unsigned long sub_a710(unsigned long *a0)
{
  sub_a4e0(*a0,a0[1],a0[2],a0[3],a0[4],a0[5],a0[6]);
  return 0;
}

// Function: sub_a740 @ 0xa740
long sub_a740(long a0,long a1)
{
  return a0 + a1;
}

// Function: sub_a750 @ 0xa750
unsigned long sub_a750(long *a0,void *a1,unsigned long a2) // return-dupe
{
  char v1;
  unsigned long v10;
  long v11;
  long *v12;
  long v13; // rdx
  long v14;
  unsigned long v15; // rdx
  long *v16;
  char *v17;
  char *v18;
  unsigned long v19;
  long v2;
  char *v20; // stack - 0x88
  char v3;
  long *v4;
  long v5; // rax
  unsigned long v6;
  unsigned long v7; // rax
  char *v8; // rax
  long v9; // rax
  
  v4 = dat_1e410;
  v3 = dat_1e038;
  v11 = dat_1e030;
  v2 = a0[5];
  if ((char)a0[6])
    return 0;
  v6 = a0[1];
  v10 = a0[4];
  if (v6 != v10) { // branch-flip
    memmove((void *)*a0,(void *)(*a0 + (v6 - v10)),v10);
    v6 = a0[4];
    a0[2] = 0;
    v14 = 0;
    a0[1] = v6;
  }
  else {
    v14 = a0[2];
  }
  v19 = v11 - 0x22;
  v17 = (char *)*a0;
  v10 = a0[3];
  do {
    v18 = &v17[v6];
    sub_a740(v17,v10,v14);
    v16 = (long *)(v5 + v13 * -0x20);
    v10 = (v5 - v2 * v13) - (long)v18;
    if (v13)
      v17 = (char *)(v16[1] + *v16);
    do {
      if (v10 <= v2 + 1U) {
        v20 = v18;
        break;
      }
      v6 = (v10 - 1) / (v2 + 1U);
      v7 = fread_unlocked(v18,1,v6,a1);
      v10 -= v7;
      v20 = &v18[v7];
      if (v6 != v7) {
        if (ferror_unlocked(a1))
          sub_5e10(dcgettext(NULL,"read failed",5),a2); // no-return
        if (feof_unlocked(a1)) {
          *(char *)&a0[6] = 1;
          if ((char *)*a0 == v20)
            return 0;
          if ((v17 != v20) && (v20[-1] != v3)) {
            *v20 = v3;
            v20 = &v20[1];
          }
        }
      }
      while (v8 = memchr(v18,(int)v3,(long)v20 - (long)v18), v8) {
        v18 = &v8[1];
        v12 = &v16[-4];
        *v8 = 0;
        *v12 = (long)v17;
        v6 = (long)v18 - (long)v17;
        v16[-3] = v6;
        if (v19 < v6)
          v19 = v6;
        v10 -= v2;
        if (v4) {
          if (v4[2] != -1)
            v8 = (char *)sub_8f70(v17,v6,v4);
          v11 = *v4;
          v16[-1] = (long)v8;
          if (v11 != -1) // branch-flip
            v16[-2] = sub_90d0(v17,v6,v4);
          else {
            if ((char)v4[6]) {
              while( true ) {
                v1 = *v17;
                v11 = 0x1e740;
                if (!*(char *)(v11 + (unsigned long)(unsigned char)sub_53e0((int)v1))) break;
                v17 = &v17[1];
              }
            }
            v16[-2] = (long)v17;
          }
        }
        v17 = v18;
        v16 = v12;
      }
      v18 = v20;
    } while (!(char)a0[6]);
    a0[1] = (long)v20 - *a0;
    v12 = a0;
    sub_a740(*a0,a0[3]);
    v11 = v9 - (long)v16 >> 5;
    v12[2] = v11;
    if (v11) {
      v12[4] = (long)v20 - (long)v17;
      dat_1e030 = v19 + 0x22;
      return 1;
    }
    v17 = (char *)sub_12ff0();
    v10 = v15 & 0xffffffffffffffe0;
    v6 = a0[1];
    v14 = a0[2];
    *a0 = (long)v17;
    a0[3] = v10;
  } while( true );
}

// Function: sub_aac0 @ 0xaac0
unsigned long sub_aac0(unsigned long a0,char a1)
{
  long v1;
  long *v10; // rdx
  unsigned long v11;
  unsigned int v12; // ebx
  void *v13 [2]; // stack - 0x98
  void *v14; // stack - 0xb8
  char v15 [24];
  unsigned long v16;
  unsigned long v17; // r12
  long *v18; // stack - 0x100
  long v19; // stack - 0xd8
  unsigned long v2;
  long v20; // stack - 0xb0
  long v21; // stack - 0xa8
  long v22; // stack - 0xa0
  long v23; // stack - 0x88
  unsigned long v24; // stack - 0x80
  long v3;
  long *v4;
  char v5; // al
  unsigned long v6; // rax
  long *v7; // rax
  void *v8;
  unsigned long v9; // rax
  
  v6 = sub_6010(a0,"r");
  v1 = dat_1e410;
  v12 = (unsigned int)dat_1e419;
  v11 = dat_1e438;
  if (dat_1e438 < dat_1e030)
    v11 = dat_1e030;
  v16 = 0;
  sub_6240(v13,0x20,v11);
  v14 = NULL;
  v19 = 0;
  do {
    v5 = sub_a750(v13,v6,a0);
    v8 = v13[0];
    if (!v5) {
      v17 = 1;
label_acaa:
      sub_6040(v6,a0);
      free(v8);
      free(v14);
      return v17;
    }
    v7 = (long *)sub_a740(v13[0],v24);
    v3 = v23;
    v18 = v7;
    v10 = &v7[v23 * -4];
    if ((v16) && ((int)(v12 ^ 1) <= (int)sub_9c70(&v14,&v7[-4]))) {
label_aca0:
      v17 = 0;
      if (a1 == 'c') {
        v9 = sub_e520(((long)v18 - (long)&v7[-4] >> 5) + v19,v15);
        v2 = dat_1e888;
        __fprintf_chk(stderr,1,dcgettext(NULL,"%s: %s:%s: disorder: ",5),v2,a0,v9);
        sub_9460(&v7[-4],stderr,dcgettext(NULL,"standard error",5));
      }
      goto label_acaa;
    }
    v4 = &v18[-4];
    while (v7 = v4, v10 < v7) {
      v4 = &v7[-4];
      if ((int)(v12 ^ 1) <= (int)sub_9c70(v7,&v7[-4])) goto label_aca0;
    }
    v11 = v7[1];
    v19 += v3;
    if (v16 < v11) {
      do {
        v16 *= 2;
        if (!v16) {
          v16 = v11;
          break;
        }
      } while (v16 < v11);
      free(v14);
      v14 = (void *)sub_12eb0(v16);
      v11 = v7[1];
    }
    v8 = memcpy(v14,(void *)*v7,v11);
    v20 = v7[1];
    if (v1) {
      v21 = (long)v8 + (v7[2] - *v7);
      v22 = (long)v8 + (v7[3] - *v7);
    }
  } while( true );
}

// Function: sub_ad90 @ 0xad90
void sub_ad90(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,void *a5)
{
  unsigned long v1;
  unsigned long v10;
  long v11;
  unsigned long v12;
  int v13; // eax
  void *v14; // rax
  unsigned long *v15; // rax
  long v16;
  long v17; // rax
  unsigned long *v18;
  void *v19;
  unsigned long v2;
  unsigned long *v20;
  long v21; // rdx
  unsigned long *v22;
  unsigned long *v23; // rdx
  unsigned long v24;
  void *v25; // stack - 0x68
  unsigned long *v26;
  unsigned long v27;
  unsigned long v28; // r14
  unsigned long v29;
  long *v3;
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
  unsigned long v6;
  unsigned long v7;
  unsigned long v8;
  unsigned long v9;
  
  v34 = (void *)sub_12fd0(a2,0x38);
  v14 = (void *)sub_12fd0(a2,8);
  v33 = (void *)sub_12fd0(a2,8);
  v15 = (unsigned long *)sub_12fd0(a2,8);
  v11 = dat_1e410;
  v25 = NULL;
  v35 = (void **)v15;
  v31 = v14;
  if (a2) {
    v24 = 0;
    v32 = a2;
    v36 = a1;
    do {
      while( true ) {
        v16 = v24 * 8;
        v22 = (unsigned long *)((long)v34 + v24 * 0x38);
        v27 = dat_1e030;
        if (dat_1e030 <= dat_1e438 / v32)
          v27 = dat_1e438 / v32;
        v26 = (unsigned long *)(v24 * 0x10 + a0);
        sub_6240(v22,0x20,v27);
        v1 = *v26;
        v2 = *(unsigned long *)((long)a5 + v16);
        if (sub_a750(v22,v2,v1)) break;
        sub_6040(*(unsigned long *)((long)a5 + v16),*v26);
        if (v24 < v36) {
          v36 -= 1;
          sub_6e80(*v26);
        }
        free((void *)*v22);
        v27 = v32 - 1;
        if (v27 <= v24) {
          if (!v27) goto label_b3cd;
          v32 = v27;
          goto label_af06;
        }
        do {
          v26 = (unsigned long *)(a0 + 0x10 + v16 * 2);
          v2 = v26[1];
          v1 = *(unsigned long *)((long)a5 + v16 + 8);
          v22 = (unsigned long *)(a0 + v16 * 2);
          *v22 = *v26;
          v22[1] = v2;
          *(unsigned long *)((long)a5 + v16) = v1;
          v16 += 8;
        } while (v32 * 8 + -8 != v16);
        v32 = v27;
      }
      v16 = sub_a740(*v22,v22[3]);
      *(long *)((long)v14 + v24 * 8) = v16 + -0x20;
      *(long *)((long)v33 + v24 * 8) = v16 + v22[2] * -0x20;
      v24 += 1;
    } while (v24 < v32);
label_af06:
    v24 = 0;
    do {
      v15[v24] = v24;
      v24 += 1;
    } while (v24 != v32);
    if (v32 != 1) {
      v24 = 1;
      do {
        while( true ) {
          v18 = &v15[v24];
          v20 = &v15[v24 - 1];
          v1 = *(unsigned long *)((long)v14 + *v18 * 8);
          v2 = *(unsigned long *)((long)v14 + *v20 * 8);
          if ((int)sub_9c70(v2,v1) <= 0) break;
          v27 = *v20;
          v24 = 1;
          *v20 = *v18;
          *v18 = v27;
        }
        v24 += 1;
      } while (v24 < v32);
    }
    v37 = 0;
    v35 = NULL;
label_afa0:
    v3 = *(long **)((long)v14 + *v15 * 8);
    if (dat_1e419) { // branch-flip
      if (v35) {
        if (!sub_9c70(v35,v3)) goto label_afd6;
        sub_9460(&v25,a3,a4);
      }
      v24 = v3[1];
      v19 = v25;
      if (v37 < v24) {
        do {
          v27 = v24;
          if (!v37) goto label_b451;
          v37 *= 2;
        } while (v37 < v24);
        v27 = v37;
label_b451:
        free(v25);
        v19 = (void *)sub_12eb0(v27);
        v24 = v3[1];
        v25 = v19;
        v37 = v27;
      }
      v38 = v24;
      v19 = memcpy(v19,(void *)*v3,v24);
      v35 = &v25;
      if (v11) {
        v39 = (v3[2] - *v3) + (long)v19;
        v40 = (long)v19 + (v3[3] - *v3);
      }
    }
    else {
      sub_9460(v3,a3,a4);
    }
label_afd6:
    v24 = *v15;
    if (*(long **)((long)v33 + v24 * 8) < v3) {
      *(long **)((long)v14 + v24 * 8) = &v3[-4];
      v31 = (void *)v24;
      if (v32 != 1) goto label_b086;
label_b14c:
      *v15 = v24;
      goto label_afa0;
    }
    v1 = *(unsigned long *)((long)a5 + v24 * 8);
    v2 = *(unsigned long *)(a0 + v24 * 0x10);
    if (sub_a750((void *)((long)v34 + v24 * 0x38),v1,v2)) {
      v24 = *v15;
      v22 = (unsigned long *)((long)v34 + v24 * 0x38);
      sub_a740(*v22,v22[3]);
      v16 = *(long *)(v21 + 0x10);
      *(long *)((long)v14 + v24 * 8) = v17 + -0x20;
      *(long *)((long)v33 + v24 * 8) = v17 + v16 * -0x20;
      v31 = (void *)v24;
      if (v32 == 1) goto label_b14c;
label_b086:
      v27 = 1;
      v12 = 1;
      v29 = v32;
      do {
        while( true ) {
          v28 = v12;
          v13 = sub_9c70(*(unsigned long *)((long)v14 + v24 * 8),*(unsigned long *)((long)v14 + v15[v28] * 8));
          if ((0 <= v13) && ((v13 || (v15[v28] <= v31)))) break;
          v12 = v27 + v28 >> 1;
          v29 = v28;
          if (v28 <= v27) goto label_b0f1;
        }
        v27 = v28 + 1;
        v12 = v27 + v29 >> 1;
      } while (v27 < v29);
label_b0f1:
      if (v27 - 1) {
        v20 = v15;
        do {
          v18 = &v20[1];
          *v20 = v20[1];
          v20 = v18;
        } while (v18 != &v15[v27 - 1]);
      }
      v15[v27 - 1] = (unsigned long)v31;
      goto label_afa0;
    }
    v24 = *v15;
    v20 = &v15[1];
    if (v32 != 1) {
      do {
        if (v24 < *v20) {
          *v20 = *v20 - 1;
          v24 = *v15;
        }
        v20 = &v20[1];
      } while (&v15[v32] != v20);
    }
    v27 = v32 - 1;
    sub_6040(*(unsigned long *)((long)a5 + v24 * 8),*(unsigned long *)(a0 + v24 * 0x10));
    v24 = *v15;
    if (v24 < v36) {
      v36 -= 1;
      sub_6e80(*(unsigned long *)(a0 + v24 * 0x10));
      v24 = *v15;
    }
    free(*(void **)((long)v34 + v24 * 0x38));
    v24 = *v15;
    if (v24 < v27) {
      v16 = v24 * 8;
      v22 = (unsigned long *)((long)v34 + v24 * 0x38);
      do {
        v26 = (unsigned long *)(a0 + 0x10 + v16 * 2);
        v2 = *v26;
        v4 = v26[1];
        v23 = &v22[7];
        v5 = *v23;
        v6 = v22[8];
        v7 = v22[9];
        v8 = v22[10];
        *(unsigned long *)((long)a5 + v16) = *(unsigned long *)((long)a5 + v16 + 8);
        v1 = v22[0xd];
        v9 = v22[0xb];
        v10 = v22[0xc];
        v26 = (unsigned long *)(a0 + v16 * 2);
        *v26 = v2;
        v26[1] = v4;
        v22[6] = v1;
        v1 = *(unsigned long *)((long)v14 + v16 + 8);
        *v22 = v5;
        v22[1] = v6;
        *(unsigned long *)((long)v14 + v16) = v1;
        v1 = *(unsigned long *)((long)v33 + v16 + 8);
        v22[2] = v7;
        v22[3] = v8;
        *(unsigned long *)((long)v33 + v16) = v1;
        v16 += 8;
        v22[4] = v9;
        v22[5] = v10;
        v22 = v23;
      } while (v32 * 8 + -8 != v16);
    }
    if (v27) {
      v20 = v15;
      do {
        v18 = &v20[1];
        *v20 = v20[1];
        v20 = v18;
      } while (&v15[v32 - 1] != v18);
      v32 = v27;
      goto label_afa0;
    }
    v30 = v35 != NULL;
    v35 = (void **)v15;
    v31 = v14;
    if (v30) {
      if (dat_1e419) {
        sub_9460(&v25,a3,a4);
        free(v25);
      }
    }
  }
label_b3cd:
  sub_6040(a3,a4);
  free(a5);
  free(v34);
  free(v35);
  free(v33);
  free(v31); // tail-call
  return;
}

// Function: sub_b500 @ 0xb500
unsigned long sub_b500(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1;
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x48
  
  v2 = sub_7470(a0,a2,&v3);
  if ((v2 < a2) && (v2 < 2)) {
    v1 = *(unsigned long *)(a0 + v2 * 0x10);
    sub_5e10(dcgettext(NULL,"open failed",5),v1); // no-return
  }
  sub_ad90(a0,a1,v2,a3,a4,v3);
  return v2;
}

// Function: sub_b5b0 @ 0xb5b0
void sub_b5b0(long a0,unsigned long a1,unsigned long a2,char *a3) // return-dupe
{
  long *v1;
  long v10; // stack - 0xd0
  char *v2;
  int v3; // eax
  long *v4; // rax
  long v5; // rax
  long *v6;
  long v7; // stack - 0xd8
  unsigned long v8; // stack - 0xe0
  long v9;
  
  if (a2 <= a1)
    return;
  v5 = 0;
  v1 = (long *)(a0 + a2 * 0x10);
  v6 = (long *)(a0 + a1 * 0x10);
  do {
    while( true ) {
      v2 = (char *)*v6;
      v3 = strcmp(v2,"-");
      if ((a3) && (!strcmp(a3,v2))) break;
      v4 = (long *)sub_5e70();
      if (!v4)
        return;
      if (!v3) goto label_b629;
      v2 = (char *)*v6;
      if ((!stat(v2,&v7)) && (v10 == v4[1])) goto label_b6a3;
label_b648:
      v6 = &v6[2];
      if (v6 == v1)
        return;
    }
    if (!v3) {
      v4 = (long *)sub_5e70();
      if (!v4)
        return;
label_b629:
      if ((!fstat(0,&v7)) && (v10 == v4[1])) {
label_b6a3:
        if (v7 == *v4) goto label_b6b0;
      }
      goto label_b648;
    }
label_b6b0:
    v9 = v5 + 0xd;
    if (!v5) {
      v5 = sub_77e0(&v8);
      v9 = v5 + 0xd;
      sub_b500(v6,0,1,v8,v9);
    }
    *v6 = v9;
    v4 = &v6[2];
    v6[1] = v5;
    v6 = v4;
    if (v4 == v1)
      return;
  } while( true );
}

// Function: sub_b720 @ 0xb720
void sub_b720(long *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  long v1; // rax
  unsigned long v10;
  unsigned long v11; // r13
  long *v12;
  unsigned long v13; // stack - 0x80
  unsigned long v14; // stack - 0x78
  unsigned long v2;
  unsigned long v3;
  unsigned long v4; // rdx
  unsigned long v5;
  long v6;
  unsigned long v7; // stack - 0x48
  long v8; // stack - 0x50
  long *v9;
  
  v3 = (unsigned long)dat_1e024;
  v14 = a2;
  if (v3 < a2) {
    do {
      v2 = 0;
      v6 = 0;
      if (v3 <= v14) { // branch-flip
        do {
          v11 = v2;
          v1 = sub_77e0(&v7);
          v2 = (unsigned long)dat_1e024;
          v3 = a1;
          if (v2 <= a1)
            v3 = v2;
          v2 = sub_b500(&a0[v6 * 2],v3,v2,v7);
          v5 = (unsigned long)dat_1e024;
          v3 = a1;
          if (v2 <= a1)
            v3 = v2;
          v6 += v2;
          v2 = v11 + 1;
          a0[v11 * 2] = v1 + 0xd;
          a0[v11 * 2 + 1] = v1;
          a1 -= v3;
        } while (v5 <= v14 - v6);
        v4 = v2 % v5;
        v3 = v5 - v4;
        v10 = v14 - v6;
        v9 = &a0[v11 * 2 + 2];
        v12 = &a0[v6 * 2];
      }
      else {
        v4 = 0;
        v5 = v3;
        v10 = v14;
        v9 = a0;
        v12 = a0;
      }
      if (v3 < v10) {
        v5 = (v10 + 1 + v4) - v5;
        v1 = sub_77e0(&v7);
        v3 = a1;
        if (v5 <= a1)
          v3 = v5;
        v5 = sub_b500(v12,v3,v5,v7);
        v3 = a1;
        if (v5 <= a1)
          v3 = v5;
        v2 += 1;
        v6 += v5;
        *v9 = v1 + 0xd;
        v9[1] = v1;
        a1 -= v3;
        v12 = &a0[v6 * 2];
        v9 = &a0[v2 * 2];
      }
      a1 += v2;
      memmove(v9,v12,(v14 - v6) * 0x10);
      v3 = (unsigned long)dat_1e024;
      v14 += v2 - v6;
    } while (v3 < v14);
  }
  sub_b5b0(a0,a1,v14,a3);
  v13 = a1;
  do {
    v3 = sub_7470(a0,v14,&v8);
    if (v14 != v3) { // branch-flip
      if (v3 <= 2) {
        v6 = a0[v3 * 2];
        sub_5e10(dcgettext(NULL,"open failed",5),v6); // no-return
      }
    }
    else {
      v6 = sub_5ee0(a3,"w");
      if (v6) {
        sub_ad90(a0,v13,v14,v6,a3,v8);
        return;
      }
      if ((*__errno_location() != 0x18) || (v14 <= 2))
        sub_5e10(dcgettext(NULL,"open failed",5),a3); // no-return
    }
    v3 -= 1;
    v9 = &a0[v3 * 2];
    while( true ) {
      sub_6040(*(unsigned long *)(v8 + v3 * 8),*v9);
      v12 = &v9[-2];
      v6 = sub_76b0(&v7,3 <= v3);
      if (v6) break;
      v3 -= 1;
      v9 = v12;
    }
    v2 = v13;
    if (v3 < v13)
      v2 = v3;
    v1 = v14 - v3;
    sub_ad90(a0,v2,v3,v7,v6 + 0xd,v8);
    v14 = v1 + 1;
    *a0 = v6 + 0xd;
    a0[1] = v6;
    memmove(&a0[2],v9,v1 * 0x10);
    v13 = (v13 - v2) + 1;
  } while( true );
}

// Function: sub_bac0 @ 0xbac0
void sub_bac0(unsigned long *a0,long a1,long a2,unsigned long a3)
{
  unsigned long v1;
  unsigned long v10; // rdx
  long *v11; // rdx
  char v12;
  void *v13; // stack - 0xe8
  char v14 [104];
  unsigned long v15; // stack - 0xf0
  unsigned long v16; // stack - 0xf8
  unsigned long v17;
  long v18;
  long v19; // stack - 0x138
  unsigned long *v2;
  long v20; // stack - 0x118
  long v21; // stack - 0x110
  unsigned long *v22; // stack - 0x108
  long v23; // stack - 0xe0
  unsigned long v24; // stack - 0xd8
  long v25; // stack - 0xd0
  long v26; // stack - 0xc8
  char v27; // stack - 0xb8
  long *v3;
  char v4; // al
  long v5; // rax
  long v6;
  unsigned long v7;
  long *v8; // rax
  unsigned long v9;
  
  v25 = 0;
  v20 = a1;
  if (a1) {
    v19 = 0;
    v6 = 0;
    v22 = a0;
    do {
      v1 = *v22;
      v16 = sub_6010(v1,"r");
      v21 = 0x30;
      v9 = v16;
      if (2 <= a3) {
        v21 = 1;
        v7 = 1;
        do {
          v7 *= 2;
          v21 += 1;
        } while (v7 < a3);
        v21 *= 0x20;
        v9 = v16;
      }
      v17 = v16;
      if (!v6) {
        sub_6240(&v13,v21,sub_8700(&v16,v22,v20,v21));
        v17 = v16;
        v9 = v16;
      }
      v16 = v9;
      v20 -= 1;
      v27 = '\0';
      while (v4 = sub_a750(&v13,v17,v1), v12 = v27, v7 = v24, v4) {
        if (v27) { // branch-flip
          if (v20) {
            if ((v25 - v23) - v21 * v24 <= v21 + 1U) goto label_bc43;
            v26 = v23;
            sub_6040(v16,v1);
            goto label_bd03;
          }
          dat_1e840 = 0;
          v6 = sub_a740(v13);
          if (v19 || v26) goto label_bc5b;
          sub_6040(v16,v1);
          v19 = 0;
          v15 = sub_6010(a2,"w");
          v18 = a2;
        }
        else {
label_bc43:
          dat_1e840 = 0;
          v6 = sub_a740(v13);
label_bc5b:
          v19 += 1;
          v12 = '\0';
          v18 = sub_77e0(&v15) + 0xd;
        }
        if (2 <= v7) { // branch-flip
          sub_7b40(v14,a3);
          v5 = sub_7aa0(a3,v7,v6);
          sub_a4e0(v6,a3,v7,v5 + 0x80,v14,v15,v18);
          sub_7cd0(a3,v5);
          sub_7d10(v14);
          v9 = 0xbb91;
        }
        else {
          sub_9fb0(v6 + -0x20,v15,v18);
          v9 = v10;
        }
        sub_6040(v15,v18,v9);
        if (v12) {
          free(v13);
          sub_7010(); // return-dupe, tail-call
          return;
        }
        v17 = v16;
      }
      sub_6040(v16,v1);
      if (!v20) goto label_bdb4;
label_bd03:
      v22 = &v22[1];
      v6 = v25;
    } while( true );
  }
label_bdbd:
  free(v13);
  v2 = dat_1e368;
  v8 = (long *)sub_12fd0(v20,0x10);
  v3 = v8;
  while (v2) {
    v3[1] = (long)v2;
    v6 = (long)v2 + 0xd;
    v2 = (unsigned long *)*v2;
    v11 = &v3[2];
    *v3 = v6;
    v3 = v11;
  }
  sub_b720(v8,v20,v20,a2);
  free(v8);
  sub_7010();
  return;
label_bdb4:
  v20 = v19;
  goto label_bdbd;
}

// Function: sub_be50 @ 0xbe50
void sub_be50(int a0)
{
  void *v1;
  unsigned long v2;
  
  v2 = dat_1e888;
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5),v2,v2);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"Write sorted concatenation of all FILE(s) to standard output.\n",5),v1);
    sub_58d0();
    sub_5900();
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
    sub_8d30();
  }
  exit(a0); // no-return
}

// Function: sub_c1a0 @ 0xc1a0
void sub_c1a0(void)
{
  sub_be50(1); // tail-call, no-return
}

// Function: sub_c1b0 @ 0xc1b0
unsigned long sub_c1b0(char *a0,long *a1,void *a2,unsigned long a3)
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

// Function: sub_c2c0 @ 0xc2c0
long sub_c2c0(char *a0,long *a1) // return-dupe
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

// Function: sub_c320 @ 0xc320
void sub_c320(unsigned long a0,unsigned long a1,long a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (a2 != -1) // branch-flip
    v1 = dcgettext(NULL,"ambiguous argument %s for %s",5);
  else {
    v1 = dcgettext(NULL,"invalid argument %s for %s",5);
  }
  v2 = sub_11420(1,a0);
  error(0,0,v1,sub_110a0(0,8,a1),v2); // tail-call
}

// Function: sub_c3b0 @ 0xc3b0
void sub_c3b0(long *a0,void *a1,unsigned long a2)
{
  long v1;
  void *v2;
  long v3;
  void *v4;
  
  v2 = stderr;
  v4 = NULL;
  fputs_unlocked(dcgettext(NULL,"Valid arguments are:",5),v2);
  v3 = 0;
  v1 = *a0;
  while (v1) {
    if ((v3) && (!memcmp(v4,a1,a2)))
      __fprintf_chk(stderr,1,", %s",sub_11430(v1));
    else {
      __fprintf_chk(stderr,1,"\n  - %s",sub_11430(v1));
      v4 = a1;
    }
    v3 += 1;
    a1 = (void *)((long)a1 + a2);
    v1 = a0[v3];
  }
  putc_unlocked(10,stderr); // tail-call
}

// Function: sub_c4b0 @ 0xc4b0
long sub_c4b0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,void *a5,char a6) // early-return
{
  long v1; // rax
  
  if (a6) // branch-flip
    v1 = sub_c1b0(a1,a2,a3,a4);
  else {
    v1 = sub_c2c0(a1,a2);
  }
  if (0 <= v1)
    return v1;
  sub_c320(a0,a1,v1);
  sub_c3b0(a2,a3,a4);
  (*a5)();
  return -1;
}

// Function: sub_c540 @ 0xc540
long sub_c540(void *a0,long *a1,void *a2,unsigned long a3) // return-dupe
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

// Function: sub_c5a0 @ 0xc5a0
void sub_c5a0(unsigned long a0)
{
  dat_1e878 = a0;
}

// Function: sub_c5b0 @ 0xc5b0
void sub_c5b0(char a0)
{
  dat_1e870 = a0;
}

// Function: sub_c5c0 @ 0xc5c0
void sub_c5c0(void)
{
  int v1; // eax
  int *v2; // rax
  char *v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_13d00(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!dat_1e870) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (dat_1e878) { // branch-flip
        v4 = sub_11250(dat_1e878);
        error(0,*v2,"%s: %s",v4,v3);
      }
      else {
        error(0,*v2,"%s",v3);
      }
      _exit(dat_1e130); // return-dupe, no-return
    }
  }
  v1 = sub_13d00(stderr);
  if (!v1)
    return;
  _exit(dat_1e130);
}

// Function: sub_c670 @ 0xc670
unsigned long sub_c670(void)
{
  return 0xffffff9f;
}

// Function: sub_c680 @ 0xc680
void sub_c680(void)
{
  MD5_Init(); // tail-call
}

// Function: sub_c690 @ 0xc690
void sub_c690(unsigned long a0,unsigned long a1,unsigned long a2)
{
  MD5_Update(a2,a0,a1); // tail-call
}

// Function: sub_c6b0 @ 0xc6b0
void sub_c6b0(void)
{
  sub_c690(); // tail-call
}

// Function: sub_c6c0 @ 0xc6c0
unsigned long sub_c6c0(unsigned long a0,unsigned long a1)
{
  MD5_Final(a1,a0);
  return a1;
}

// Function: sub_c6e0 @ 0xc6e0
void sub_c6e0(void)
{
  MD5(); // tail-call
}

// Function: sub_c6f0 @ 0xc6f0
unsigned long sub_c6f0(unsigned long *a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x78
  unsigned long v10; // stack - 0x30
  unsigned long v11; // stack - 0x28
  unsigned int v12; // stack - 0x20
  unsigned long v2; // stack - 0x70
  unsigned long v3; // stack - 0x68
  unsigned long v4; // stack - 0x60
  unsigned long v5; // stack - 0x58
  unsigned long v6; // stack - 0x50
  unsigned long v7; // stack - 0x48
  unsigned long v8; // stack - 0x40
  unsigned long v9; // stack - 0x38
  
  v1 = *a0;
  v2 = a0[1];
  v3 = a0[2];
  v4 = a0[3];
  v11 = a0[10];
  v5 = a0[4];
  v6 = a0[5];
  v7 = a0[6];
  v8 = a0[7];
  v9 = a0[8];
  v10 = a0[9];
  v12 = *(unsigned int *)&a0[0xb];
  MD5_Final(a1,&v1);
  return a1;
}

// Function: sub_c780 @ 0xc780
unsigned long sub_c780(void *a0,unsigned long a1) // return-dupe
{
  unsigned long v1; // rax
  void *v2; // rax
  long v3; // rax
  char v4 [104];
  long v5; // rbp
  
  v1 = sub_c670();
  if ((int)v1 == -5)
    return 1;
  if (!(int)v1)
    return v1;
  v2 = malloc(0x8048);
  if (!v2)
    return 1;
  sub_c680(v4);
  do {
    v5 = 0;
    while( true ) {
      if (feof_unlocked(a0)) goto label_c825;
      v3 = fread_unlocked((long)v2 + v5,1,0x8000 - v5,a0);
      v5 += v3;
      if (v5 == 0x8000) break;
      if (!v3) {
        if (ferror_unlocked(a0)) {
          free(v2);
          return 1;
        }
label_c825:
        if (v5)
          sub_c690(v2,v5,v4);
        sub_c6c0(v4,a1);
        free(v2);
        return 0;
      }
    }
    sub_c6b0(v2,0x8000,v4);
  } while( true );
}

// Function: sub_c8c0 @ 0xc8c0
void sub_c8c0(void)
{
  posix_fadvise(); // tail-call
}

// Function: sub_c8d0 @ 0xc8d0
void sub_c8d0(void *a0,unsigned int a1)
{
  if (a0) {
    sub_c8c0(fileno(a0),0,0,a1); // tail-call
    return;
  }
}

// Function: sub_c900 @ 0xc900
unsigned long sub_c900(void *a0) // early-return
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
    if (sub_c9b0(a0)) {
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

// Function: sub_c990 @ 0xc990
void sub_c990(unsigned int *a0)
{
  if (!(*a0 & 0x100))
    return;
  sub_cf30(a0,0,1); // tail-call
}

// Function: sub_c9b0 @ 0xc9b0
void sub_c9b0(void *a0)
{
  if (a0) {
    if (__freading(a0)) {
      sub_c990(a0);
      fflush(a0); // tail-call
      return;
    }
  }
  fflush(a0); // tail-call
}

// Function: sub_c9f0 @ 0xc9f0
unsigned long sub_c9f0(long a0,unsigned long *a1)
{
  char v1;
  unsigned long v2;
  unsigned long v3;
  char v4; // al
  unsigned long v5;
  unsigned long v6;
  unsigned long v7; // r13
  
  v5 = 0;
  v2 = *a1;
  v3 = v5;
label_ca1a:
  do {
    v7 = v3;
    v3 = v5 + 1;
    v6 = v5;
    while ((v5 = v3, v3 = v5, v5 < v2 && (*(char *)(a0 + v6) == '.'))) {
      v1 = *(char *)(a0 + 1 + v6);
      v4 = sub_13b00((int)v1);
      if ((v1 != '~') && (!v4)) {
        if (0 <= (long)v2) goto label_ca85;
        goto label_ca1a;
      }
      v5 = v6 + 2;
      v6 = v5;
      v3 = v7;
      if (v2 <= v5) goto label_ca1a;
      do {
        v5 = v6;
        v1 = *(char *)(a0 + v5);
        v4 = sub_13ad0((int)v1);
        if ((v1 != '~') && (!v4)) goto label_ca1a;
        v6 = v5 + 1;
      } while (v6 < v2);
      v3 = v5 + 2;
    }
    if ((long)v2 <= -1) {
      if (!*(char *)(a0 + v6)) goto label_ca97;
      goto label_ca1a;
    }
label_ca85:
    if (v2 == v6) {
label_ca97:
      *a1 = v6;
      return v7;
    }
  } while( true );
}

// Function: sub_cac0 @ 0xcac0
unsigned int sub_cac0(long a0,long a1,long a2) // ternary
{
  unsigned char v1;
  unsigned int v2; // r12d
  
  if (a1 != a2) { // branch-flip
    v1 = *(unsigned char *)(a0 + a1);
    v2 = (unsigned int)v1;
    if (sub_13b70(v2))
      return 0;
    if (!sub_13b00((unsigned int)v1)) {
      v2 = (v1 != 0x7e) ? v1 + 0x100 : 0xfffffffe; // branch-flip
    }
  }
  else {
    v2 = 0xffffffff;
  }
  return v2;
}

// Function: sub_cb30 @ 0xcb30
int sub_cb30(long a0,long a1,long a2,long a3)
{
  char v1;
  int v2;
  int v3; // eax
  long v4;
  long v5;
  char v6;
  int v7; // stack - 0x40
  
  v5 = 0;
  v4 = 0;
label_cb50:
  if (v5 < a1) goto label_cb6d;
label_cb55:
  if (a3 <= v4)
    return 0;
  do {
    if (a1 <= v5) { // branch-flip
label_cb7b:
      if (a3 <= v4) goto label_cbe0;
      v6 = *(char *)(a2 + v4);
      if (sub_13b70((int)v6)) {
        if (a1 <= v5) goto label_cc24;
        goto label_cbfd;
      }
    }
    else {
label_cb6d:
      v6 = *(char *)(a0 + v5);
      if (sub_13b70((int)v6)) goto label_cb7b;
    }
    v2 = sub_cac0(a0,v5,a1);
    v3 = sub_cac0(a2,v4,a3);
    if (v2 != v3)
      return v2 - v3;
    v5 += 1;
    v4 += 1;
  } while( true );
label_cbe0:
  if (v5 < a1) goto label_cbfd;
  goto label_cb55;
label_cbfd:
  do {
    if (*(char *)(a0 + v5) != '0') goto label_cc08;
    v5 += 1;
  } while (v5 < a1);
  if (a3 <= v4) goto label_cb50;
  do {
    v6 = *(char *)(a2 + v4);
label_cc24:
    if (v6 != '0') {
      if ((a1 <= v5) || (a3 <= v4)) {
        v7 = 0;
        goto label_cd1f;
      }
      v7 = 0;
      goto label_cc9a;
    }
    v4 += 1;
label_cc08:
  } while (v4 < a3);
  if (a1 <= v5) goto label_cb50;
  v7 = 0;
label_cd24:
  v6 = *(char *)(a0 + v5);
  if (sub_13b70((int)v6))
    return 1;
label_ccab:
  if ((v4 < a3) && (v6 = *(char *)(a2 + v4), sub_13b70((int)v6)))
    return -1;
  if (v7)
    return v7;
  goto label_cb50;
  while( true ) {
    v1 = *(char *)(a2 + v4);
    if (!sub_13b70((int)v1))
      return 1;
    v2 = (int)v6 - (int)v1;
    if (v7)
      v2 = v7;
    v5 += 1;
    v4 += 1;
    v7 = v2;
    if ((a1 <= v5) || (a3 <= v4)) break;
label_cc9a:
    v6 = *(char *)(a0 + v5);
    if (!sub_13b70((int)v6)) goto label_ccab;
  }
label_cd1f:
  if (a1 <= v5) goto label_ccab;
  goto label_cd24;
}

// Function: sub_cd70 @ 0xcd70
unsigned long sub_cd70(char *a0,long a1,char *a2,long a3) // early-return x4
{
  long v1;
  unsigned int v2; // eax
  long v3; // rax
  long v4;
  unsigned long v5; // rax
  long v6; // stack - 0x38
  long v7 [2]; // stack - 0x30
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
    goto label_cdd6;
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
label_cea4:
    if (a2[1] != '.') goto label_cdd6;
    v2 = 0;
  }
  else {
    if (a1 < 0) {
      if (!a0[2]) goto label_cedb;
      goto label_cea4;
    }
    if (a1 != 2) goto label_cea4;
label_cedb:
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
label_cdd6:
  v6 = a3;
  v7[0] = a1;
  v3 = sub_c9f0(a0,v7);
  v4 = sub_c9f0(a2,&v6);
  v1 = v7[0];
  if ((v7[0] != v3) || (v6 != v4)) {
    v5 = sub_cb30(a0,v3,a2,v4);
    if ((int)v5)
      return v5;
    v4 = v6;
  }
  return sub_cb30(a0,v1,a2,v4);
}

// Function: sub_cf10 @ 0xcf10
void sub_cf10(unsigned long a0,unsigned long a1)
{
  sub_cd70(a0,0xffffffffffffffff,a1,0xffffffffffffffff); // tail-call
}

// Function: sub_cf30 @ 0xcf30
unsigned long sub_cf30(unsigned int *a0,unsigned long a1,unsigned int a2) // return-dupe
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

// Function: sub_cfb0 @ 0xcfb0
bool sub_cfb0(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_12240(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_d040 @ 0xd040
unsigned long sub_d040(unsigned long a0)
{
  char v1 [16];
  unsigned long v2; // rax
  unsigned long v3;
  unsigned long v4; // rdx
  unsigned long v5; // rsi
  long v6; // r8
  
  v1._8_8_ = 0;
  v1._0_8_ = a0;
  v2 = SUB168(v1._0_16_ * ZEXT816(0xaaaaaaaaaaaaaaab),0);
  v4 = a0 % 3;
  if ((10 <= a0) && (v4)) {
    v6 = 0x10;
    v5 = 9;
    v3 = 3;
    do {
      v3 += 2;
      v5 += v6;
      v2 = a0 / v3;
      v4 = a0 % v3;
      if (a0 <= v5) break;
      v6 += 8;
    } while (v4);
  }
  v3 = v2 >> 8;
  v2 = CONCAT71((undefined7)v3,v4 != 0);
  return v2;
}

// Function: sub_d0b0 @ 0xd0b0
unsigned long sub_d0b0(unsigned long a0)
{
  char v1; // al
  unsigned long v2; // rax
  unsigned long v3;
  
  v2 = 10;
  if (10 <= a0)
    v2 = a0;
  v3 = v2 | 1;
  while ((v3 != 0xffffffffffffffff && (v1 = sub_d040(v3), !v1))) {
    v3 += 2;
  }
  v2 = v3;
  return v2;
}

// Function: sub_d0f0 @ 0xd0f0
unsigned long sub_d0f0(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_13a80(a0,3) % a1;
}

// Function: sub_d110 @ 0xd110
bool sub_d110(long a0,long a1)
{
  return a1 == a0;
}

// Function: sub_d120 @ 0xd120
unsigned long sub_d120(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0x17200)
    return 1;
  v1 = v2[2];
  if ((((dat_17214 < v1) && (v1 < dat_17218)) && (dat_1721c < v2[3])) && (dat_17110 <= *v2)) {
    v3 = *v2 + dat_17214;
    if (((v3 < v2[1]) && (v2[1] <= dat_17220)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0x17200;
  return 0;
}

// Function: sub_d1b0 @ 0xd1b0
void sub_d1b0(long a0,unsigned long *a1)
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(a0 + 0x48);
  *a1 = 0;
  a1[1] = v1;
  *(unsigned long **)(a0 + 0x48) = a1;
}

// Function: sub_d1d0 @ 0xd1d0
long sub_d1d0(long *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*(void *)a0[6])(a1,a0[2]);
  if (v1 < (unsigned long)a0[2])
    return v1 * 0x10 + *a0;
  abort(); // no-return
}

// Function: sub_d200 @ 0xd200
long sub_d200(long a0,long a1,unsigned long *a2,char a3) // early-return
{
  void *v1;
  long *v2;
  long v3;
  long *v4;
  long v5;
  
  v4 = (long *)sub_d1d0(a0,a1);
  *a2 = v4;
  v5 = *v4;
  if (!v5)
    return 0;
  if (v5 != a1) {
    v1 = *(void **)(a0 + 0x38);
    if (!(*v1)(a1)) {
      while( true ) {
        if (!(long *)v4[1])
          return 0;
        if ((*(long *)v4[1] == a1) || (v1 = *(void **)(a0 + 0x38), (*v1)(a1))) break;
        v4 = (long *)v4[1];
      }
      v5 = *(long *)v4[1];
      if (!a3)
        return v5;
      v4[1] = ((long *)v4[1])[1];
      sub_d1b0(a0);
      return v5;
    }
    v5 = *v4;
  }
  if (a3) {
    v2 = (long *)v4[1];
    if (v2) {
      v3 = v2[1];
      *v4 = *v2;
      v4[1] = v3;
      sub_d1b0(a0);
      return v5;
    }
    *v4 = 0;
  }
  return v5;
}

// Function: sub_d300 @ 0xd300
void sub_d300(long a0)
{
  if (*(long *)(a0 + 0x48)) {
    *(unsigned long *)(a0 + 0x48) = *(unsigned long *)(*(long *)(a0 + 0x48) + 8);
    return;
  }
  malloc(0x10); // tail-call
}

// Function: sub_d330 @ 0xd330
unsigned long sub_d330(long a0,unsigned long *a1,char a2) // return-dupe x2
{
  long v1;
  long *v2; // rax
  long *v3;
  long *v4;
  long *v5;
  long v6;
  
  v5 = (long *)*a1;
  if ((long *)a1[1] <= v5)
    return 1;
  do {
    while (v6 = *v5, !v6) {
label_d358:
      v5 = &v5[2];
      if ((long *)a1[1] <= v5)
        return 1;
    }
    v3 = (long *)v5[1];
    if ((long *)v5[1]) {
      do {
        while( true ) {
          v6 = *v3;
          v2 = (long *)sub_d1d0(a0,v6);
          v4 = (long *)v3[1];
          if (*v2) break;
          *v2 = v6;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
          sub_d1b0(a0);
          v3 = v4;
          if (!v4) goto label_d3c8;
        }
        v3[1] = v2[1];
        v2[1] = (long)v3;
        v3 = v4;
      } while (v4);
label_d3c8:
      v6 = *v5;
    }
    v5[1] = 0;
    if (a2) goto label_d358;
    v3 = (long *)sub_d1d0(a0,v6);
    if (*v3) { // branch-flip
      v4 = (long *)sub_d300(a0);
      if (!v4)
        return 0;
      v1 = v3[1];
      *v4 = v6;
      v4[1] = v1;
      v3[1] = (long)v4;
    }
    else {
      *v3 = v6;
      *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
    }
    *v5 = 0;
    v5 = &v5[2];
    a1[3] = a1[3] + -1;
    if ((long *)a1[1] <= v5)
      return 1;
  } while( true );
}

// Function: sub_d460 @ 0xd460
unsigned long sub_d460(float a0,unsigned long a1,char a2) // early-return, ternary
{
  unsigned long v1; // rax
  
  if (!a2) {
    a0 = (float)a1 / a0;
    if (dat_17224 <= a0)
      return 0;
    a1 = (a0 < dat_17228) ? (unsigned long)a0 : (long)(a0 - dat_17228) ^ 0x8000000000000000;
  }
  v1 = sub_d0b0(a1);
  if ((!(v1 >> 0x3c & 1)) && (!(v1 >> 0x3d)))
    return v1;
  return 0;
}

// Function: sub_d4f0 @ 0xd4f0
unsigned long sub_d4f0(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}

// Function: sub_d500 @ 0xd500
unsigned long sub_d500(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}

// Function: sub_d510 @ 0xd510
unsigned long sub_d510(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}

// Function: sub_d520 @ 0xd520
unsigned long sub_d520(unsigned long *a0)
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

// Function: sub_d580 @ 0xd580
bool sub_d580(unsigned long *a0)
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
label_d5a0:
      v2 = &v2[2];
    }
    v1 = v2[1];
    v4 += 1;
    v3 += 1;
    if (!v1) goto label_d5a0;
    do {
      v1 = *(long *)(v1 + 8);
      v3 += 1;
    } while (v1);
    v2 = &v2[2];
  } while( true );
}

// Function: sub_d5f0 @ 0xd5f0
void sub_d5f0(unsigned long a0,void *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_d510(a0);
  v2 = sub_d4f0();
  v3 = sub_d500();
  v4 = sub_d520();
  __fprintf_chk(a1,1,"# entries:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v2);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v3);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v4); // tail-call
}

// Function: sub_d710 @ 0xd710
long sub_d710(long a0,long a1)
{
  void *v1;
  long *v2;
  long v3;
  
  v2 = (long *)sub_d1d0(a0,a1);
  v3 = *v2;
  if (!v3)
    return 0;
  while( true ) {
    if (v3 == a1)
      return a1;
    v1 = *(void **)(a0 + 0x38);
    if ((*v1)(a1)) break;
    v2 = (long *)v2[1];
    if (!v2)
      return 0;
    v3 = *v2;
  }
  return *v2;
}

// Function: sub_d770 @ 0xd770
long sub_d770(unsigned long *a0)
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

// Function: sub_d7c0 @ 0xd7c0
long sub_d7c0(long a0,long a1)
{
  long v1;
  long *v2; // rax
  long *v3;
  
  v2 = (long *)sub_d1d0(a0,a1);
  v3 = v2;
  do {
    v1 = *v3;
    v3 = (long *)v3[1];
    if (v1 == a1) {
      if (v3)
        return *v3;
      break;
    }
  } while (v3);
  do {
    v2 = &v2[2];
    if (*(long **)(a0 + 8) <= v2)
      return 0;
  } while (!*v2);
  return *v2;
}

// Function: sub_d830 @ 0xd830
void sub_d830(unsigned long *a0,long a1,unsigned long a2)
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

// Function: sub_d880 @ 0xd880
long sub_d880(unsigned long *a0,void *a1,unsigned long a2)
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

// Function: sub_d900 @ 0xd900
unsigned long sub_d900(unsigned char *a0,unsigned long a1)
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

// Function: sub_d940 @ 0xd940
void sub_d940(unsigned long *a0)
{
  *(char *)&a0[2] = 0;
  *a0 = 0x3f80000000000000;
  a0[1] = 0x3fb4fdf43f4ccccd;
}

// Function: sub_d970 @ 0xd970
unsigned long * sub_d970(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_d0f0;
  if (!a3)
    a3 = sub_d110;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = 0x17200;
  v1[5] = a1;
  if (sub_d120(v1)) {
    v2 = sub_d460(*(unsigned int *)(a1 + 8),a0,*(char *)(a1 + 0x10));
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

// Function: sub_da80 @ 0xda80
void sub_da80(unsigned long *a0)
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
        if ((long *)a0[1] <= v5) goto label_db1b;
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
label_db1b:
  a0[3] = 0;
  a0[4] = 0;
  return;
}

// Function: sub_db30 @ 0xdb30
void sub_db30(unsigned long *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = (long *)*a0;
  v3 = (long *)a0[1];
  if ((a0[8]) && (a0[4])) {
    if (v3 <= v4) goto label_dbdc;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_db9f;
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
label_db9f:
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
label_dbdc:
  v2 = (void *)a0[9];
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free((void *)*a0);
  free(a0); // tail-call
}

// Function: sub_dc20 @ 0xdc20
unsigned int sub_dc20(unsigned long *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_d460(*(unsigned int *)(a0[5] + 8),a1,*(char *)(a0[5] + 0x10));
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
  v1 = sub_d330(&v2,a0,0);
  if (!(char)v1) {
    a0[9] = v11;
    if ((sub_d330(a0,&v2,1)) && (sub_d330(a0,&v2,0))) {
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

// Function: sub_dda0 @ 0xdda0
unsigned long sub_dda0(long a0,long a1,long *a2) // ternary
{
  long v1;
  long *v2; // rax
  long *v3; // stack - 0x38
  unsigned long v4; // rsi
  float v5; // xmm4_da
  float v6; // xmm5_da
  
  if (a1) {
    v1 = sub_d200(a0,a1,&v3,0);
    if (v1) {
      if (a2) {
        *a2 = v1;
        return 0;
      }
      return 0;
    }
    v6 = (float)*(unsigned long *)(a0 + 0x18);
    v5 = (float)*(unsigned long *)(a0 + 0x10);
    if (*(float *)(*(long *)(a0 + 0x28) + 8) * v5 < v6) {
      sub_d120(a0);
      v1 = *(long *)(a0 + 0x28);
      if (v5 * *(float *)(v1 + 8) < v6) {
        v5 = v5 * *(float *)(v1 + 0xc);
        if (!*(char *)(v1 + 0x10))
          v5 = v5 * *(float *)(v1 + 8);
        if (dat_17224 <= v5)
          return 0xffffffff;
        v4 = (dat_17228 <= v5) ? (long)(v5 - dat_17228) ^ 0x8000000000000000 : (unsigned long)v5;
        if (!sub_dc20(a0,v4))
          return 0xffffffff;
        if (sub_d200(a0,a1,&v3,0)) {
          abort(); // no-return, return-dupe
        }
      }
    }
    if (!*v3) {
      *v3 = a1;
      *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
      *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
      return 1;
    }
    v2 = (long *)sub_d300(a0);
    if (!v2)
      return 0xffffffff;
    *v2 = a1;
    v2[1] = v3[1];
    v3[1] = (long)v2;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    return 1;
  }
  abort();
}

// Function: sub_df90 @ 0xdf90
unsigned long sub_df90(unsigned long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_dda0(a0,a1,&v3);
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

// Function: sub_dff0 @ 0xdff0
long sub_dff0(long a0,unsigned long a1) // ternary x2
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
  
  v5 = sub_d200(a0,a1,&v7,1);
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
      sub_d120(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_17228 <= v8) ? (long)(v8 - dat_17228) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_dc20(a0,v6)) {
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

// Function: sub_e180 @ 0xe180
void sub_e180(void)
{
  sub_dff0(); // tail-call
}

// Function: sub_e190 @ 0xe190
unsigned long sub_e190(void)
{
  return 0;
}

// Function: sub_e1a0 @ 0xe1a0
void sub_e1a0(long a0,unsigned long a1,void *a2)
{
  unsigned long *v1;
  unsigned long v2;
  unsigned long v3;
  long v4;
  unsigned long v5; // rbx
  
  v2 = *(unsigned long *)(a0 + a1 * 8);
  if (a1 != 1) { // branch-flip
    while( true ) {
      v5 = a1 >> 1;
      v1 = (unsigned long *)(a0 + v5 * 8);
      v3 = *v1;
      v4 = a1 * 8;
      if (1 <= (int)(*a2)(v3,v2)) break;
      *(unsigned long *)(a0 + v4) = *v1;
      a1 = v5;
      if (v5 == 1) goto label_e210;
    }
  }
  else {
label_e210:
    v4 = 8;
  }
  *(unsigned long *)(a0 + v4) = v2;
}

// Function: sub_e220 @ 0xe220
unsigned long * sub_e220(long a0,unsigned long a1,void *a2)
{
  unsigned long *v1;
  unsigned long v2;
  bool v3;
  int v4; // eax
  unsigned long *v5; // rax
  unsigned long *v6;
  unsigned long v7;
  unsigned long v8;
  
  v2 = *(unsigned long *)(a0 + 8);
  if (a1 >> 1) { // branch-flip
    v8 = 1;
    do {
      v7 = v8 * 2;
      if (a1 <= v7) // branch-flip
        v6 = (unsigned long *)(v8 * 0x10 + a0);
      else {
        v1 = (unsigned long *)(a0 + (v7 + 1) * 8);
        v6 = (unsigned long *)(v8 * 0x10 + a0);
        v4 = (*a2)(*v6,*v1);
        if (v4 <= -1) {
          v6 = v1;
          v7 += 1;
        }
      }
      v4 = (*a2)(*v6,v2);
      v1 = (unsigned long *)(a0 + v8 * 8);
      v3 = v4 <= 0;
      v5 = v1;
      if (v3) goto label_e2d6;
      *v1 = *v6;
      v8 = v7;
    } while (a1 >> 1 >= v7);
    v5 = v6;
  }
  else {
    v5 = (unsigned long *)(a0 + 8);
    v8 = 1;
  }
label_e2d6:
  *v5 = v2;
  v5 = (unsigned long *)v8;
  return v5;
}

// Function: sub_e300 @ 0xe300
unsigned long * sub_e300(void *a0,long a1)
{
  unsigned long *v1; // rax
  unsigned long *v2; // rax
  
  v1 = (unsigned long *)sub_12eb0(0x20);
  if (!a1)
    a1 = 1;
  v2 = (unsigned long *)sub_12fd0(a1,8);
  v1[1] = a1;
  *v2 = 0;
  *v1 = v2;
  v1[2] = 0;
  if (!a0)
    a0 = sub_e190;
  v1[3] = a0;
  return v1;
}

// Function: sub_e370 @ 0xe370
void sub_e370(unsigned long *a0)
{
  free((void *)*a0);
  free(a0); // tail-call
}

// Function: sub_e390 @ 0xe390
unsigned long sub_e390(long *a0,unsigned long a1)
{
  long v1;
  long v2; // rsi
  unsigned long v3;
  
  v3 = a0[2];
  v1 = *a0;
  if ((unsigned long)(a0[1] - 1U) <= v3) {
    v1 = sub_12ff0(v1,&a0[1],8);
    v3 = a0[2];
    *a0 = v1;
  }
  v2 = v3 + 1;
  a0[2] = v2;
  *(unsigned long *)(v1 + v2 * 8) = a1;
  sub_e1a0(*a0,v2,a0[3]);
  return 0;
}

// Function: sub_e400 @ 0xe400
unsigned long sub_e400(long *a0)
{
  long v1;
  long v2;
  unsigned long v3;
  unsigned long v4;
  
  v1 = a0[2];
  v4 = 0;
  if (v1) {
    v2 = *a0;
    v3 = *(unsigned long *)(v2 + v1 * 8);
    v4 = *(unsigned long *)(v2 + 8);
    a0[2] = v1 + -1;
    *(unsigned long *)(v2 + 8) = v3;
    sub_e220(*a0,v1 + -1,a0[3]);
  }
  return v4;
}

// Function: sub_e440 @ 0xe440
char * sub_e440(unsigned int a0,long a1)
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

// Function: sub_e4d0 @ 0xe4d0
char * sub_e4d0(unsigned long a0,long a1)
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

// Function: sub_e520 @ 0xe520
char * sub_e520(unsigned long a0,long a1)
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

// Function: sub_e580 @ 0xe580
int sub_e580(unsigned char *a0,long a1,unsigned int a2)
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
          if (0x1e <= (unsigned char)(v2 + 0x9f)) goto label_e604;
label_e6b3:
          a0 = &a0[1];
          v9 += 1;
        }
        else {
          if ('@' < (char)v2) goto label_e6b3;
          if ('#' < (char)v2) {
            if (0x1b <= (unsigned char)(v2 - 0x25)) goto label_e604;
            goto label_e6b3;
          }
          if ('\x1f' < (char)v2) goto label_e6b3;
label_e604:
          v7 = 0;
          do {
            v5 = sub_14250(&v8,a0,(long)v1 - (long)a0,&v7);
            if (v5 == -1) {
              if (a2 & 1)
                return -1;
              goto label_e6b3;
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
label_e739:
          if (v9 == 0x7fffffff)
            return 0x7fffffff;
          v9 += 1;
        }
        else {
          if (a2 & 2)
            return -1;
          if (!(v3 & 2)) goto label_e739;
        }
      } while (v1 != a0);
    }
  }
  return v9;
}

// Function: sub_e7a0 @ 0xe7a0
void sub_e7a0(char *a0,unsigned int a1)
{
  sub_e580(a0,strlen(a0),a1); // tail-call
}

// Function: sub_e7d0 @ 0xe7d0
unsigned long sub_e7d0(char *a0) // return-dupe x2
{
  char v1;
  unsigned long v2; // rax
  char *v3; // stack - 0x28
  char *v4; // rbp
  
  if (!a0)
    return 0;
  v1 = *a0;
  do {
    if (!v1) {
      v1 = sub_13b70(0);
label_e82a:
      if (!v1)
        return 0;
      v3 = NULL;
      v2 = strtoul(a0,&v3,10);
      if (!v3)
        return 0;
      v1 = *v3;
      while( true ) {
        if (!v1)
          return v2;
        v4 = &v3[1];
        if (!sub_13c00((int)v1)) break;
        v1 = *v4;
        v3 = v4;
      }
      if (v1 != ',')
        return 0;
      return v2;
    }
    if (!sub_13c00((int)v1)) {
      v1 = sub_13b70((int)v1);
      goto label_e82a;
    }
    v1 = a0[1];
    a0 = &a0[1];
  } while( true );
}

// Function: sub_e8d0 @ 0xe8d0
long sub_e8d0(void) // return-dupe
{
  long v1; // rax
  char v2 [136];
  
  if (sched_getaffinity(0,0x80,v2))
    return 0;
  v1 = (long)(int)__sched_cpucount(0x80,v2);
  if (!v1)
    return 0;
  return v1;
}

// Function: sub_e940 @ 0xe940
unsigned long sub_e940(int a0) // return-dupe
{
  unsigned long v1; // rax
  unsigned long v2;
  
  if (a0 != 1) {
    v1 = sysconf(0x53);
    v2 = v1;
    if ((v1 - 1 < 2) && (v2 = sub_e8d0(), v2 <= v1))
      return v1;
    if ((long)v2 > 0)
      return v2;
    return 1;
  }
  v2 = sub_e8d0();
  if (v2)
    return v2;
  v2 = sysconf(0x54);
  if ((long)v2 > 0)
    return v2;
  return 1;
}

// Function: sub_e9d0 @ 0xe9d0
unsigned long sub_e9d0(unsigned long a0)
{
  unsigned long v1;
  unsigned long v2; // rax
  
  v2 = 0xffffffffffffffff;
  if ((int)a0 == 2) {
    v1 = sub_e7d0(getenv("OMP_NUM_THREADS"));
    v2 = sub_e7d0(getenv("OMP_THREAD_LIMIT"));
    a0 = 1;
    if (!v2)
      v2 = 0xffffffffffffffff;
    if (v1) {
      if (v2 <= v1)
        v1 = v2;
      return v1;
    }
  }
  v1 = sub_e940(a0);
  if (v2 <= v1)
    v1 = v2;
  return v1;
}

// Function: sub_ea60 @ 0xea60
double sub_ea60(void)
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

// Function: sub_eb50 @ 0xeb50
double sub_eb50(void)
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
  sub_ea60();
  return v4 * dat_170f0;
}

// Function: sub_ec90 @ 0xec90
unsigned int sub_ec90(int *a0,unsigned int a1) // return-dupe x2
{
  int v1;
  unsigned long v2;
  unsigned int v3; // eax
  int *v4; // rax
  
  v2 = *(unsigned long *)a0;
  if (0 <= dat_1e880) {
    v3 = pipe2();
    if ((0 <= (int)v3) || (*__errno_location() != 0x26)) {
      dat_1e880 = 1;
      return v3;
    }
    dat_1e880 = -1;
  }
  if (a1 & 0xfff7f7ff) {
    *__errno_location() = 0x16;
    return 0xffffffff;
  }
  if (0 <= pipe(a0)) {
    if ((!(a1 & 0x800)) || ((((v3 = sub_13f30(a0[1],3,0), 0 <= (int)v3 && (v1 = a0[1], sub_13f30(v1,4,v3 | 0x800) != -1)) && (v3 = sub_13f30(*a0,3,0), 0 <= (int)v3)) && (v1 = *a0, sub_13f30(v1,4,v3 | 0x800) != -1)))) {
      if (!(a1 & 0x80000))
        return a1 & 0xfff7f7ff;
      v3 = sub_13f30(a0[1],1,0);
      if (((0 <= (int)v3) && (v1 = a0[1], sub_13f30(v1,2,v3 | 1) != -1)) && ((v3 = sub_13f30(*a0,1,0), 0 <= (int)v3 && (v1 = *a0, sub_13f30(v1,2,v3 | 1) != -1))))
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

// Function: sub_ee70 @ 0xee70
unsigned long sub_ee70(void)
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

// Function: sub_ef00 @ 0xef00
void sub_ef00(char *a0)
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
    dat_1e888 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_efa0 @ 0xefa0
int * sub_efa0(int *a0,int a1)
{
  if (a1 != 10) {
    *a0 = a1;
    a0[1] = 0;
    a0[2] = 0;
    a0[3] = 0;
    a0[4] = 0;
    a0[5] = 0;
    a0[6] = 0;
    a0[7] = 0;
    a0[8] = 0;
    a0[9] = 0;
    a0[10] = 0;
    a0[0xb] = 0;
    a0[0xc] = 0;
    a0[0xd] = 0;
    return a0;
  }
  abort(); // no-return
}

// Function: sub_eff0 @ 0xeff0
bool sub_eff0(long a0,long a1)
{
  return sub_13ca0(a0 + 9,a1 + 9) == 0;
}

// Function: sub_f010 @ 0xf010
unsigned long sub_f010(long a0,unsigned long a1,unsigned char a2)
{
  unsigned char v1;
  
  if (sub_13c20((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 8) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 8);
  }
  if (v1 == a2) {
    if (a2)
      return sub_eff0(a0,a1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_f070 @ 0xf070
unsigned long sub_f070(long a0,unsigned long a1,unsigned char a2,char a3)
{
  unsigned char v1;
  
  if (sub_13c20((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 7) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 7);
  }
  if (v1 == a2) {
    if (a2)
      return sub_f010(a0,a1,(int)a3); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_f0f0 @ 0xf0f0
unsigned long sub_f0f0(long a0,unsigned long a1,unsigned char a2,char a3,char a4)
{
  unsigned char v1;
  
  if (sub_13c20((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 6) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 6);
  }
  if (v1 == a2) {
    if (a2)
      return sub_f070(a0,a1,(int)a3,(int)a4); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_f160 @ 0xf160
unsigned long sub_f160(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5)
{
  unsigned char v1;
  
  if (sub_13c20((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 5) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 5);
  }
  if (v1 == a2) {
    if (a2)
      return sub_f0f0(a0,a1,(int)a3,(int)a4,(int)a5); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_f200 @ 0xf200
unsigned long sub_f200(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6)
{
  unsigned int v1;
  unsigned char v2;
  
  v1 = a6;
  if (sub_13c20((int)(char)a2)) // branch-flip
    v2 = *(unsigned char *)(a0 + 4) & 0xdf;
  else {
    v2 = *(unsigned char *)(a0 + 4);
  }
  if (v2 == a2) {
    if (a2)
      return sub_f160(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_f2a0 @ 0xf2a0
unsigned long sub_f2a0(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,int a6,unsigned int a7)
{
  unsigned int v1;
  unsigned int v2;
  unsigned char v3;
  
  v2 = a7;
  v1 = a6;
  if (sub_13c20((int)(char)a2)) // branch-flip
    v3 = *(unsigned char *)(a0 + 3) & 0xdf;
  else {
    v3 = *(unsigned char *)(a0 + 3);
  }
  if (v3 == a2) {
    if (a2) {
      a6 = (int)(char)v2;
      return sub_f200(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_f360 @ 0xf360
unsigned long sub_f360(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,unsigned int a8)
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  unsigned char v4;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_13c20((int)(char)a2)) // branch-flip
    v4 = *(unsigned char *)(a0 + 2) & 0xdf;
  else {
    v4 = *(unsigned char *)(a0 + 2);
  }
  if (v4 == a2) {
    if (a2) {
      a7 = (int)(char)v3;
      a6 = (int)(char)v2;
      return sub_f2a0(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_f420 @ 0xf420
unsigned long sub_f420(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,int a8,unsigned int a9)
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  unsigned int v4;
  unsigned char v5;
  char v6; // stack - 0x3c
  
  v4 = a9;
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_13c20((int)(char)a2)) // branch-flip
    v5 = *(unsigned char *)(a0 + 1) & 0xdf;
  else {
    v5 = *(unsigned char *)(a0 + 1);
  }
  if (v5 == a2) {
    if (a2) {
      a8 = (int)(char)v4;
      a7 = (int)(char)v3;
      a6 = (int)(char)v2;
      v6 = (char)v1;
      return sub_f360(a0,a1,(int)a3,(int)a4,(int)a5,(int)v6); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_f500 @ 0xf500
unsigned long sub_f500(unsigned char *a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6,unsigned int a7,unsigned int a8) // early-return
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_13c20((int)(char)a2)) { // branch-flip
    if (a2 != (*a0 & 0xdf))
      return 0;
  }
  else if (a2 != *a0)
    return 0;
  return sub_f420(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1,(int)(char)v2,(int)(char)v3,0,0);
}

// Function: sub_f5c0 @ 0xf5c0
char * sub_f5c0(char *a0,int a1) // return-dupe, ternary x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = dcgettext(NULL,a0,5);
  if (a0 == v1) {
    v2 = sub_14210();
    if (sub_f500(v2,"UTF-8",0x55,0x54,0x46,0x2d,0x38,0,0)) { // branch-flip
      v1 = (*v1 != '`') ? "’" : "‘"; // branch-flip
    }
    else {
      if (!sub_f500(v2,"GB18030",0x47,0x42,0x31,0x38,0x30,0x33,0x30)) {
        if (a1 == 9)
          return "\"";
        return "\'";
      }
      v1 = (*v1 != '`') ? (char *)0x172b0 : (char *)0x172b5; // branch-flip
    }
  }
  return v1;
}

// Function: sub_f6f0 @ 0xf6f0
long * sub_f6f0(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
{
  unsigned long v1;
  char v10;
  unsigned char *v11;
  unsigned long v12;
  unsigned long v13;
  unsigned long v14; // stack - 0x48
  unsigned int v15; // stack - 0x4c
  unsigned char v16;
  unsigned int v17;
  unsigned long v18;
  unsigned char v19;
  bool v2;
  bool v20;
  unsigned long v21;
  bool v22;
  unsigned long v23;
  bool v24;
  bool v25;
  bool v26;
  bool v27;
  bool v28;
  long v29; // stack - 0xe0
  bool v3;
  unsigned long v30; // stack - 0xd8
  char *v31; // stack - 0xb0
  unsigned long v32; // stack - 0xa8
  unsigned long v33; // stack - 0xa0
  unsigned int v34; // stack - 0x80
  char *v35; // stack - 0x70
  char *v36; // stack - 0x68
  bool v4;
  long v5;
  long v6;
  unsigned char v7;
  unsigned int v8; // eax
  long *v9; // rax
  
  v29 = a6;
  v36 = a7;
  v35 = a8;
  v34 = a5;
label_f760:
  v9 = (unsigned long)__ctype_get_mb_cur_max();
  v33 = (unsigned long)v9;
  v17 = v34 & 2;
  switch(a4) {
    case 0:
      v24 = 0;
      v2 = 0;
      v12 = 0;
      v30 = 0;
      v31 = NULL;
      break;
    case 1:
label_fbe6:
      v2 = 0;
      goto label_fba0;
    case 2:
      if (!v17) {
        v2 = 0;
        goto label_fd01;
      }
      v24 = 1;
      v2 = 0;
      v12 = 0;
      v30 = 1;
      v31 = "\'";
      break;
    case 3:
      v2 = 1;
label_fba0:
      v24 = 1;
      v12 = 0;
      a4 = 2;
      v30 = 1;
      v31 = "\'";
      break;
    case 4:
      if (!v17) {
        v2 = 1;
        goto label_fd01;
      }
      goto label_fbe6;
    case 5:
      if (!v17) goto label_fb41;
      v24 = 1;
      v2 = 1;
      v12 = 0;
      v30 = 1;
      v31 = "\"";
      break;
    case 6:
      v24 = 1;
      v12 = 0;
      v2 = 1;
      v30 = 1;
      a4 = 5;
      v31 = "\"";
      break;
    case 7:
      v24 = 0;
      v2 = 1;
      v12 = 0;
      v30 = 0;
      v31 = NULL;
      break;
    case 8:
    case 9:
    case 10:
      if (a4 != 10) {
        v9 = (char *)sub_f5c0("`",a4);
        v36 = (char *)v9;
        v9 = (char *)sub_f5c0("\'",a4);
        v35 = (char *)v9;
      }
      v12 = 0;
      if (!v17) {
        v10 = *v36;
        while (v10) {
          if (v12 < a1)
            a0[v12] = v10;
          v12 += 1;
          v10 = v36[v12];
        }
      }
      v24 = v17 != 0;
      v2 = 1;
      v9 = (unsigned long)strlen(v35);
      v31 = v35;
      v30 = (unsigned long)v9;
      break;
    default:
      abort(); // no-return
    
  }
  v4 = 1;
  v20 = 0;
  v32 = 0;
  v13 = a1;
  v3 = 0;
label_f823:
  do {
label_f836:
    v23 = 0;
    a1 = v13;
label_f840:
    v13 = v32;
    v25 = a3 != v23;
    if (a3 == 0xffffffffffffffff)
      v25 = a2[v23] != '\0';
    if (v25) {
      v11 = (unsigned char *)&a2[v23];
      v25 = (bool)(a4 != 2 & v2);
      v26 = 1;
      v13 = v12;
      v27 = v2;
      v28 = v2;
      v22 = v25;
      if (v25) {
        if (v30) {
          if ((a3 == 0xffffffffffffffff) && (2 <= v30)) {
            v9 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v9;
          }
          if ((a3 < v23 + v30) || (v8 = memcmp(v11,v31,v30), v8)) goto label_101b0;
          if (!v24) {
            v7 = *v11;
            v18 = (unsigned long)v7;
            v21 = v18;
            if ((char)v7 <= '?') {
              switch(v7) {
                case 0:
                  goto label_fdac;
                default:
                  goto label_fa83;
                case 7:
                  goto label_fd70;
                case 8:
                  goto label_fd55;
                case 9:
                  goto label_fe33;
                case 10:
                  goto label_fd4b;
                case 0xb:
                  goto label_fe60;
                case 0xc:
                  goto label_fd97;
                case 0xd:
                  goto label_fc9d;
                case 0x20:
                  goto label_fe6d;
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
                  goto label_fa36;
                case 0x23:
                  goto label_fe10;
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
                  goto label_f968;
                case 0x27:
                  goto label_fc82;
                case 0x3f:
                  goto label_fc41;
                
              }
            }
            if ('{' > (char)v7) {
              if (v7 == 0x40) goto label_fa83;
              v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
              if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_f968;
              if (!((unsigned long)v9 & 0xa4000000)) goto label_10254;
              goto label_10198;
            }
            if (v7 == 0x7d) goto label_10320;
            if ((char)v7 <= '}') {
              v21 = 0x7b;
              if (v7 == 0x7b) goto label_1017a;
              v21 = 0x7c;
              if (v7 == 0x7c) goto label_10198;
              goto label_fa83;
            }
            if (v7 != 0x7e) goto label_fa83;
label_102d4:
            if (!v23) {
              v7 = a4 == 2;
              v21 = 0x7e;
              v26 = v3;
              goto label_faf9;
            }
            v21 = 0x7e;
            goto label_102e2;
          }
          goto label_fa6c;
        }
label_101b0:
        v7 = *v11;
        v18 = (unsigned long)v7;
        v21 = v18;
        if ('?' < (char)v7) {
          if ('{' <= (char)v7) { // branch-flip
            if (v7 != 0x7d) { // branch-flip
              if ('~' <= (char)v7) {
                v22 = 0;
                if (v7 == 0x7e) goto label_102d4;
                goto label_fa80;
              }
              v21 = 0x7b;
              v25 = 0;
              if (v7 != 0x7b) {
                v21 = 0x7c;
                v25 = 0;
                v22 = 0;
                if (v7 == 0x7c) goto label_10198;
                goto label_fa83;
              }
            }
            else {
              v21 = 0x7d;
              v25 = 0;
            }
label_1017a:
            if (a3 == 0xffffffffffffffff) goto label_1032f;
label_10188:
            v22 = v25;
            if (a3 == 1) goto label_fe15;
          }
          else {
            if (v7 == 0x40) goto label_fa80;
            v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
            v22 = 0;
            if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_f968;
            v25 = 0;
            v22 = 0;
            if (!((unsigned long)v9 & 0xa4000000)) {
label_10254:
              if ((char)v18 != '\\') goto label_fa83;
              if (((bool)(v2 & v24)) && (v30)) goto label_1027a;
              v16 = 0x5c;
              goto label_fca2;
            }
          }
label_10198:
          v7 = a4 == 2;
          v26 = v3;
          v25 = 0;
          goto label_faf9;
        }
        switch(v7) {
          case 0:
            goto label_fd9e;
          default:
label_fa80:
            v25 = 0;
label_fa83:
            v16 = (unsigned char)v18;
            v22 = v25;
            if (v33 != 1) goto label_10003;
label_fa90:
            v9 = __ctype_b_loc();
            v21 = 1;
            v26 = (*(unsigned char *)(*v9 + 1 + v18 * 2) & 0x40) == 0;
            v27 = (bool)(v26 & v2);
            v26 = !v26;
            v22 = v25;
            goto label_fade;
          case 7:
label_fd70:
            v16 = 0x61;
            if (!v24) goto label_fd68;
            goto label_fccf;
          case 8:
label_fd55:
            v16 = 0x62;
            goto label_fd5a;
          case 9:
            v25 = 0;
label_fe33:
            v21 = 9;
            v16 = 0x74;
            goto label_fe40;
          case 10:
label_fd4b:
            v16 = 0x6e;
            break;
          case 0xb:
label_fe60:
            v16 = 0x76;
            break;
          case 0xc:
label_fd97:
            v16 = 0x66;
label_fd5a:
            if (v24) goto label_fccf;
label_fd68:
            v25 = 0;
            goto label_f9ee;
          case 0xd:
label_fc9d:
            v16 = 0x72;
            break;
          case 0x20:
            v22 = 0;
label_fe6d:
            v21 = 0x20;
            goto label_f968;
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
            v25 = 0;
            v26 = 0;
            goto label_fa39;
          case 0x23:
            v21 = 0x23;
            v25 = 0;
            goto label_fe15;
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
            v22 = 0;
label_f968:
            v7 = 0;
            goto label_f978;
          case 0x27:
            v22 = 0;
            goto label_fc82;
          case 0x3f:
            v22 = 0;
            goto label_fc41;
          
        }
        goto label_fca2;
      }
      v16 = *v11;
      v18 = (unsigned long)v16;
      if ('?' < (char)v16) {
        if ('{' <= (char)v16) {
          if (v16 == 0x7d) {
label_10320:
            v21 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_10188;
label_1032f:
            v22 = v25;
            if (a2[1]) goto label_10198;
            goto label_fe15;
          }
          if ((char)v16 <= '}') {
            v21 = 0x7b;
            if (v16 != 0x7b) {
              if (v16 == 0x7c) goto label_fa36;
              goto label_fa83;
            }
            goto label_1017a;
          }
          v21 = 0x7e;
          if (v16 == 0x7e) goto label_fe15;
          if (v33 == 1) goto label_fa90;
label_10003:
          v14 = 0;
          if (a3 == 0xffffffffffffffff) {
            v9 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v9;
          }
          v21 = 0;
          do {
            v18 = v23 + v21;
            v9 = (long)sub_14250(&v15,&a2[v18],a3 - v18,&v14);
            v6 = (long)v9;
            v25 = v26;
            if (!v9) break;
            if (v9 == (long *)0xffffffffffffffff) {
              v25 = 0;
              goto label_1056b;
            }
            if (v9 == (long *)0xfffffffffffffffe) {
              v9 = (long *)v18;
              if (a3 > v18) goto label_108e5;
              goto label_108ef;
            }
            if (((a4 == 2) && (v24)) && (v9 != (long *)0x1)) {
              v11 = (unsigned char *)&a2[v18 + 1];
              v5 = v18 + (long)v9;
              do {
                v8 = (unsigned int)*v11;
                v8 -= 0x5b;
                v17 = v8;
                v9 = (unsigned long)(unsigned long)v8;
                v18 = (unsigned long)v9;
                v7 = (unsigned char)v17;
                if ((v7 <= 0x21) && (0x20000002bU >> (v18 & 0x3f) & 1)) goto label_1013d;
                v11 = &v11[1];
              } while ((unsigned char *)&a2[v5] != v11);
            }
            v8 = iswprint(v15);
            if (!v8)
              v26 = 0;
            v21 += v6;
            v8 = mbsinit(&v14);
            v25 = v26;
          } while (!v8);
          v27 = (bool)((v25 ^ 1U) & v2);
          goto label_1056b;
        }
        if (v16 == 0x40) goto label_fa83;
        v9 = (unsigned long)(1L << (v16 + 0xbf & 0x3f));
        if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_faf0;
        if ((unsigned long)v9 & 0xa4000000) goto label_fa36;
        if (v16 != 0x5c) goto label_fa83;
        if (a4 == 2) {
          if (!v24) goto label_1027a;
          goto label_fa63;
        }
        if (!(bool)(v2 & v24 & v30 != 0)) {
          v21 = 0x5c;
          v16 = 0x5c;
          v25 = 0;
          goto label_fe40;
        }
label_1027a:
        v23 += 1;
        v25 = 0;
        v16 = 0x5c;
        v7 = v20;
        goto label_10290;
      }
      switch(v16) {
        case 0:
          if (!v2) {
            v22 = 0;
            v21 = 0;
            if (v34 & 1) goto label_ff12;
            goto label_fe4b;
          }
label_fd9e:
          if (!v24) {
            v22 = 0;
label_fdac:
            v27 = a4 == 2;
            v7 = v20 ^ 1;
            v7 &= v27;
            if ((bool)v7) {
              if (v12 < a1)
                a0[v12] = 0x27;
              if (v12 + 1 < a1)
                a0[v12 + 1] = 0x24;
              if (v12 + 2 < a1)
                a0[v12 + 2] = 0x27;
              v13 = v12 + 3;
              if (a1 <= v12 + 3) {
                v12 += 4;
                v25 = 0;
                v16 = 0x30;
                v20 = (bool)v7;
                goto label_fb23;
              }
label_106b5:
              a0[v13] = 0x5c;
              v12 = v13;
              v20 = (bool)v7;
            }
            else if (v12 < a1) {
              v7 = v20;
              goto label_106b5;
            }
            v13 = v12 + 1;
            if (a4 != 2) {
              v21 = 0x30;
              if ((v23 + 1 < a3) && (v7 = a2[v23 + 1] - 0x30, v7 <= 9)) {
                if (v13 < a1)
                  a0[v13] = 0x30;
                if (v12 + 2 < a1)
                  a0[v12 + 2] = 0x30;
                v13 = v12 + 3;
                v21 = 0x30;
              }
label_106ec:
              v16 = (unsigned char)v21;
              v25 = 0;
              v12 = v13;
              v7 = v26;
              if (!v2 || v27) goto label_fb23;
              goto label_f978;
            }
            v16 = 0x30;
            v25 = 0;
            v12 = v13;
            v7 = v26;
            goto label_fb23;
          }
          if (a4 != 2) goto label_fa6c;
          goto label_fcdb;
        default:
          goto label_fa83;
        case 7:
          v21 = 7;
          v16 = 0x61;
          break;
        case 8:
          v21 = 8;
          v16 = 0x62;
          break;
        case 9:
          v21 = 9;
          v16 = 0x74;
          goto label_fecf;
        case 10:
          v21 = 10;
          v16 = 0x6e;
          goto label_fecf;
        case 0xb:
          v21 = 0xb;
          v16 = 0x76;
          break;
        case 0xc:
          v21 = 0xc;
          v16 = 0x66;
          break;
        case 0xd:
          v21 = 0xd;
          v16 = 0x72;
label_fecf:
          v7 = a4 == 2 & v24;
          v25 = 0;
          if (!(bool)v7) goto label_fe40;
label_fa51:
          a4 = 2;
label_fa63:
          if (!v28) goto label_fa6c;
          goto label_fcdb;
        case 0x20:
          v18 = 0x20;
          goto label_fa39;
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
label_fa36:
          v26 = 0;
          goto label_fa39;
        case 0x23:
label_fe10:
          v21 = 0x23;
label_fe15:
          v22 = v25;
          if (v23) {
label_102e2:
            v7 = a4 == 2;
            v26 = v3;
            v25 = 0;
            goto label_faf9;
          }
          v18 = v21;
label_fa39:
          v21 = v18;
          v22 = v25;
          v25 = v26;
          if ((a4 == 2) && (v24)) goto label_fa51;
          goto label_faf7;
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
          goto label_faf0;
        case 0x27:
label_fc82:
          v25 = v26;
          if (a4 != 2) {
            v7 = 0;
            v21 = 0x27;
            goto label_faf9;
          }
          if (!v24) {
            if (a1) { // branch-flip
              v13 = 0;
              v18 = a1;
              if (v32) goto label_1070b;
            }
            else {
label_1070b:
              if (v12 < a1)
                a0[v12] = 0x27;
              if (v12 + 1 < a1)
                a0[v12 + 1] = 0x5c;
              v13 = a1;
              v18 = v32;
              if (v12 + 2 < a1)
                a0[v12 + 2] = 0x27;
            }
            v12 += 3;
            v7 = 0;
            v16 = 0x27;
            a1 = v13;
            v32 = v18;
            v20 = 0;
            v3 = v26;
            goto label_fb23;
          }
          goto label_fa63;
        case 0x3f:
label_fc41:
          if (a4 != 2) {
            if ((((a4 == 5) && (v34 & 4)) && (v18 = v23 + 2, v18 < a3)) && (a2[v23 + 1] == '?')) {
              v7 = a2[v18];
              v21 = (unsigned long)v7;
              if (v7 <= 0x3e) {
                v9 = (unsigned long)(0x7000a38200000000 >> (v7 & 0x3f));
                v1 = (unsigned long)v9 & 1;
                v7 = (unsigned char)v9 & 1;
                if (!v1) {
                  v21 = 0x3f;
                  v26 = v3;
                  v25 = 0;
                  goto label_faf9;
                }
                if (v24) goto label_fa6c;
                if (v12 < a1)
                  a0[v12] = 0x3f;
                if (v12 + 1 < a1)
                  a0[v12 + 1] = 0x22;
                if (v12 + 2 < a1)
                  a0[v12 + 2] = 0x22;
                if (v12 + 3 < a1)
                  a0[v12 + 3] = 0x3f;
                v13 = v12 + 4;
                v27 = 0;
                v26 = 0;
                v23 = v18;
                goto label_106ec;
              }
            }
            v7 = 0;
            v21 = 0x3f;
            v26 = v3;
            v25 = 0;
            goto label_faf9;
          }
          if (v24) goto label_fa63;
          v7 = 0;
          v16 = 0x3f;
          v25 = 0;
          goto label_fb23;
        
      }
label_fe40:
      v22 = v25;
      if (!v2) {
label_fe4b:
        v16 = (unsigned char)v21;
        v25 = 0;
        v7 = 0;
        v26 = v3;
        if (!v24) goto label_fb20;
        goto label_f978;
      }
label_fca2:
      v25 = 0;
      goto label_fcb3;
    }
    if (!(bool)(v12 == 0 & v24 & a4 == 2)) {
      v7 = a4 == 2 & (v24 ^ 1U);
      v24 = (bool)(v24 ^ 1U);
      if ((!(bool)v7) || (v24 = (bool)v7, !v3)) {
label_1096f:
        v9 = (long *)v12;
        if (((v31) && (v24)) && (v10 = *v31, v10)) {
          do {
            if (v9 < a1)
              a0[(long)v9] = v10;
            v9 = (unsigned long)((long)v9 + 1);
            v10 = v31[(long)v9 - v12];
          } while (v10);
        }
        if (v9 < a1)
          a0[(long)v9] = 0;
        return v9;
      }
      if (!v4) {
        v7 = !a1 && v32;
        v24 = v3;
        if (a1 || !v32) goto label_1096f;
        v23 = v32;
        goto label_1043b;
      }
      v12 = 0;
      a4 = 5;
      v9 = (unsigned long)__ctype_get_mb_cur_max();
      v30 = 1;
      v33 = (unsigned long)v9;
      v31 = "\"";
      if (!(v34 & 2)) goto label_10a1f;
      v3 = 0;
      v20 = 0;
      v32 = 0;
      v2 = v4;
      v24 = v4;
      goto label_f836;
    }
label_1013d:
    if (v2) {
label_fcdb:
      v2 = 1;
      v34 &= 0xfffffffd;
      v9 = (unsigned long)__ctype_get_mb_cur_max();
      v29 = 0;
      v33 = (unsigned long)v9;
    }
    else {
      v2 = 0;
      v34 &= 0xfffffffd;
      v9 = (unsigned long)__ctype_get_mb_cur_max();
      v29 = 0;
      v33 = (unsigned long)v9;
    }
label_fd01:
    v24 = 0;
    a4 = 2;
    v12 = 1;
    v31 = "\'";
    v30 = 1;
    v20 = 0;
    v4 = 1;
    if (a1) { // branch-flip
      v7 = 0;
      v23 = 0;
      v32 = a1;
label_1043b:
      v12 = 1;
      *a0 = 0x27;
      a4 = 2;
      v24 = 0;
      v30 = 1;
      v31 = "\'";
      v13 = v32;
      v32 = v23;
      v3 = (bool)v7;
    }
    else {
      v32 = 0;
      v13 = a1;
      v3 = 0;
    }
  } while( true );
  while( true ) {
    v21 += 1;
    v9 = (unsigned long)(v23 + v21);
    if (a3 <= v9) break;
label_108e5:
    if (!a2[(long)v9]) break;
  }
label_108ef:
  v25 = 0;
label_1056b:
  v18 = (unsigned long)v16;
  v26 = v25;
  if (2 <= v21) {
label_10575:
    v19 = 0;
    v21 += v23;
    v13 = v23;
    do {
      v16 = (unsigned char)v18;
      if (v27) {
        v28 = a4 == 2;
        if (v24) goto label_fa63;
        v7 = v28 & (v20 ^ 1U);
        if ((bool)v7) {
          if (v12 < a1)
            a0[v12] = 0x27;
          if (v12 + 1 < a1)
            a0[v12 + 1] = 0x24;
          if (v12 + 2 < a1)
            a0[v12 + 2] = 0x27;
          v12 += 3;
          v20 = 1;
        }
        if (v12 < a1)
          a0[v12] = 0x5c;
        if (v12 + 1 < a1) {
          v7 = (char)(v18 >> 6) + '0';
          a0[v12 + 1] = v7;
        }
        if (v12 + 2 < a1) {
          v7 = ((unsigned char)(v18 >> 3) & 7) + 0x30;
          a0[v12 + 2] = v7;
        }
        v23 = v13 + 1;
        v12 += 3;
        v16 = (v16 & 7) + 0x30;
        if (v21 <= v23) goto label_fa01;
        v19 = 1;
      }
      else {
        v7 = v19 ^ 1;
        v7 &= v20;
        if (v22) {
          if (v12 < a1)
            a0[v12] = 0x5c;
          v12 += 1;
        }
        v23 = v13 + 1;
        if (v21 <= v23) {
          goto label_10290;
        }
        if ((bool)v7) {
          if (v12 < a1)
            a0[v12] = 0x27;
          if (v12 + 1 < a1)
            a0[v12 + 1] = 0x27;
          v12 += 2;
          v22 = 0;
          v20 = 0;
        }
        else {
          v22 = 0;
        }
      }
      v13 += 1;
      if (v12 < a1)
        a0[v12] = v16;
      v18 = (unsigned long)(unsigned char)a2[v13];
      v12 += 1;
    } while( true );
  }
label_fade:
  if (v27) {
    v25 = 0;
    v27 = v2;
    goto label_10575;
  }
label_faf0:
  v21 = v18;
  v25 = v26;
label_faf7:
  v7 = a4 == 2;
  v26 = v3;
label_faf9:
  v16 = (unsigned char)v21;
  v27 = (bool)v7 == 0;
  v3 = v26;
  v7 = 0;
  if ((v27 && v2) || (v7 = 0, v24)) {
label_f978:
    v16 = (unsigned char)v21;
    v12 = v13;
    if (!v29) goto label_fb23;
    if (!(*(unsigned int *)(v29 + (v21 >> 5) * 4) >> (v16 & 0x1f) & 1)) goto label_fb23;
  }
  else {
label_fb20:
    v7 = 0;
    v3 = v26;
label_fb23:
    if (!v22) {
      v7 ^= 1;
      v23 += 1;
      v7 &= v20;
label_10290:
      if ((bool)v7) {
        if (v12 < a1)
          a0[v12] = 0x27;
        if (v12 + 1 < a1)
          a0[v12 + 1] = 0x27;
        v20 = 0;
        v12 += 2;
      }
      goto label_fa01;
    }
  }
label_fcb3:
  if (v24) {
    v24 = v2;
label_fccf:
    if ((bool)(v24 & a4 == 2)) goto label_fcdb;
label_fa6c:
    v34 &= 0xfffffffd;
    v29 = 0;
    goto label_f760;
  }
  v7 = v20 ^ 1;
  v7 &= a4 == 2;
  if ((bool)v7) {
    if (v12 < a1)
      a0[v12] = 0x27;
    if (v12 + 1 < a1)
      a0[v12 + 1] = 0x24;
    if (v12 + 2 < a1)
      a0[v12 + 2] = 0x27;
    v12 += 3;
    v20 = 1;
  }
label_f9ee:
  if (v12 < a1)
    a0[v12] = 0x5c;
  v12 += 1;
  v23 += 1;
label_fa01:
  if (v12 < a1)
    a0[v12] = v16;
  v12 += 1;
  if (!v25)
    v4 = 0;
  goto label_f840;
label_ff12:
  v23 += 1;
  goto label_f840;
label_10a1f:
  a1 = v32;
label_fb41:
  v24 = 0;
  a4 = 5;
  v12 = 1;
  v31 = "\"";
  v2 = 1;
  v30 = 1;
  v20 = 0;
  v3 = 0;
  v4 = 1;
  v32 = 0;
  v13 = a1;
  if (a1)
    *a0 = 0x22;
  goto label_f823;
}

// Function: sub_10b20 @ 0x10b20
void * sub_10b20(unsigned int a0,unsigned long a1,unsigned long a2,unsigned int *a3)
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
    v6 = dat_1e180;
    if (dat_1e178 <= (int)a0) {
      v10 = (long)dat_1e178;
      v9 = (long)(int)((a0 - dat_1e178) + 1);
      if (dat_1e180 != (unsigned long *)0x1e190) // branch-flip
        v6 = (unsigned long *)sub_13070(dat_1e180,&v10,v9,0x7fffffff,0x10);
      else {
        v6 = (unsigned long *)sub_13070(0,&v10,v9,0x7fffffff,0x10);
        *v6 = dat_1e190;
        v6[1] = dat_1e198;
      }
      dat_1e180 = v6;
      memset(&v6[(long)dat_1e178 * 2],0,(v10 - dat_1e178) * 0x10);
      dat_1e178 = (int)v10;
    }
    v1 = &v6[(long)(int)a0 * 2];
    v3 = a3[1];
    v5 = *v1;
    v8 = (void *)v1[1];
    v7 = sub_f6f0(v8,v5,a1,a2,*a3,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    if (v5 <= v7) {
      v7 += 1;
      *v1 = v7;
      if (v8 != (void *)0x1e8a0)
        free(v8);
      v8 = (void *)sub_12ef0(v7);
      v4 = *a3;
      v1[1] = (unsigned long)v8;
      sub_f6f0(v8,v7,a1,a2,v4,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    }
    *v11 = v2;
    return v8;
  }
  abort(); // no-return
}

// Function: sub_10d10 @ 0x10d10
void sub_10d10(long a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = 0x1e9a0;
  sub_131b0(a0,0x38);
  *v2 = v1;
}

// Function: sub_10d50 @ 0x10d50
unsigned int sub_10d50(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x1e9a0;
  return *a0;
}

// Function: sub_10d70 @ 0x10d70
void sub_10d70(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x1e9a0;
  *a0 = a1;
}

// Function: sub_10d90 @ 0x10d90
unsigned int sub_10d90(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0x1e9a0;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_10dd0 @ 0x10dd0
unsigned int sub_10dd0(long a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = 0x1e9a0;
  v1 = *(unsigned int *)(a0 + 4);
  *(unsigned int *)(a0 + 4) = a1;
  return v1;
}

// Function: sub_10df0 @ 0x10df0
void sub_10df0(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0x1e9a0;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_10e30 @ 0x10e30
void sub_10e30(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x1e9a0;
  v2 = __errno_location();
  v1 = *v2;
  sub_f6f0(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(unsigned long *)&a4[10],*(unsigned long *)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_10eb0 @ 0x10eb0
unsigned long sub_10eb0(unsigned long a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x1e9a0;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_f6f0(0,0,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  v4 = sub_12ef0(v3 + 1);
  sub_f6f0(v4,v3 + 1,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_10fa0 @ 0x10fa0
void sub_10fa0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_10eb0(a0,a1,0,a2); // tail-call
}

// Function: sub_10fb0 @ 0x10fb0
void sub_10fb0(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_1e180;
  if (2 <= dat_1e178) {
    v4 = (unsigned long *)((long)dat_1e180 + 0x18);
    v1 = (unsigned long *)((long)dat_1e180 + (unsigned long)(unsigned int)(dat_1e178 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0x1e8a0) {
    free(*(void **)((long)v3 + 8));
    dat_1e198 = 0x1e8a0;
    dat_1e190 = 0x100;
  }
  if (v3 == (void *)0x1e190) {
    dat_1e178 = 1;
    return;
  }
  free(v3);
  dat_1e178 = 1;
  dat_1e180 = (void *)0x1e190;
}

// Function: sub_11050 @ 0x11050
void sub_11050(unsigned long a0,unsigned long a1)
{
  sub_10b20(a0,a1,0xffffffffffffffff,0x1e9a0); // tail-call
}

// Function: sub_11070 @ 0x11070
void sub_11070(void)
{
  sub_10b20(); // tail-call
}

// Function: sub_11080 @ 0x11080
void sub_11080(unsigned long a0)
{
  sub_11050(0,a0); // tail-call
}

// Function: sub_11090 @ 0x11090
void sub_11090(unsigned long a0,unsigned long a1)
{
  sub_11070(0,a0,a1); // tail-call
}

// Function: sub_110a0 @ 0x110a0
void sub_110a0(unsigned int a0,unsigned long a1,unsigned long a2)
{
  char v1 [56];
  
  sub_efa0(v1);
  sub_10b20(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_11100 @ 0x11100
void sub_11100(unsigned int a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  char v1 [56];
  
  sub_efa0(v1);
  sub_10b20(a0,a2,a3,v1);
}

// Function: sub_11170 @ 0x11170
void sub_11170(unsigned int a0,unsigned long a1)
{
  sub_110a0(0,a0,a1); // tail-call
}

// Function: sub_11180 @ 0x11180
void sub_11180(unsigned int a0,unsigned long a1,unsigned long a2)
{
  sub_11100(0,a0,a1,a2); // tail-call
}

// Function: sub_111a0 @ 0x111a0
void sub_111a0(unsigned long a0,unsigned long a1,char a2,unsigned long a3)
{
  unsigned long v1; // stack - 0x48
  unsigned long *v2; // rsp
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v2 = &v1;
  v1 = dat_1e9a0;
  v3 = dat_1e9a8;
  v4 = dat_1e9b0;
  v5 = dat_1e9b8;
  v8 = dat_1e9d0;
  v6 = dat_1e9c0;
  v7 = dat_1e9c8;
  sub_10d90(&v1,(int)a2,1,a3,a0,a1);
  sub_10b20(0,a0,a1,v2);
}

// Function: sub_11230 @ 0x11230
void sub_11230(unsigned long a0,char a1)
{
  sub_111a0(a0,0xffffffffffffffff,(int)a1); // tail-call
}

// Function: sub_11250 @ 0x11250
void sub_11250(unsigned long a0)
{
  sub_11230(a0,0x3a); // tail-call
}

// Function: sub_11260 @ 0x11260
void sub_11260(unsigned long a0,unsigned long a1)
{
  sub_111a0(a0,a1,0x3a); // tail-call
}

// Function: sub_11270 @ 0x11270
void sub_11270(unsigned int a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x98
  unsigned long v10; // stack - 0x50
  unsigned long v11; // stack - 0x48
  unsigned long v12; // stack - 0x40
  unsigned long v13; // stack - 0x38
  unsigned long v14; // stack - 0x30
  unsigned long v15; // stack - 0x28
  unsigned long v2; // stack - 0x58
  unsigned long *v3;
  unsigned long v4; // stack - 0x90
  unsigned long v5; // stack - 0x88
  unsigned long v6; // stack - 0x80
  unsigned long v7; // stack - 0x78
  unsigned long v8; // stack - 0x70
  unsigned long v9; // stack - 0x68
  
  sub_efa0(&v1);
  v3 = &v2;
  v2 = v1;
  v10 = v4;
  v15 = v9;
  v11 = v5;
  v12 = v6;
  v13 = v7;
  v14 = v8;
  sub_10d90(v3,0x3a,1);
  sub_10b20(a0,a2,0xffffffffffffffff,v3);
}

// Function: sub_11310 @ 0x11310
void sub_11310(unsigned int a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x68
  unsigned long v2; // stack - 0x60
  unsigned long v3; // stack - 0x58
  unsigned long v4; // stack - 0x50
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  
  v1 = dat_1e9a0;
  v2 = dat_1e9a8;
  v7 = dat_1e9d0;
  v3 = dat_1e9b0;
  v4 = dat_1e9b8;
  v5 = dat_1e9c0;
  v6 = dat_1e9c8;
  sub_10df0(&v1);
  sub_10b20(a0,a3,a4,&v1);
}

// Function: sub_113b0 @ 0x113b0
void sub_113b0(void)
{
  sub_11310(); // tail-call
}

// Function: sub_113c0 @ 0x113c0
void sub_113c0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_113b0(0,a0,a1,a2); // tail-call
}

// Function: sub_113e0 @ 0x113e0
void sub_113e0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  sub_11310(0,a0,a1,a2,a3); // tail-call
}

// Function: sub_11400 @ 0x11400
void sub_11400(void)
{
  sub_10b20(); // tail-call
}

// Function: sub_11410 @ 0x11410
void sub_11410(unsigned long a0,unsigned long a1)
{
  sub_11400(0,a0,a1); // tail-call
}

// Function: sub_11420 @ 0x11420
void sub_11420(unsigned long a0,unsigned long a1)
{
  sub_11400(a0,a1,0xffffffffffffffff); // tail-call
}

// Function: sub_11430 @ 0x11430
void sub_11430(unsigned long a0)
{
  sub_11420(0,a0); // tail-call
}

// Function: sub_11440 @ 0x11440
void sub_11440(unsigned long a0,unsigned long a1)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_12eb0(0x1038);
  *v1 = a0;
  v1[1] = sub_11470;
  v1[2] = a1;
}

// Function: sub_11470 @ 0x11470
void sub_11470(long a0)
{
  unsigned long v1; // rax
  int *v2; // rax
  char *v3; // rax
  
  if (a0) {
    v1 = sub_11430(a0);
    v2 = __errno_location();
    if (*v2) // branch-flip
      v3 = dcgettext(NULL,"%s: read error",5);
    else {
      v3 = dcgettext(NULL,"%s: end of file",5);
    }
    error(dat_1e130,*v2,v3,v1);
  }
  abort(); // no-return
}

// Function: sub_114e0 @ 0x114e0
unsigned long sub_114e0(unsigned long a0,long a1) // return-dupe
{
  unsigned long v1;
  long v2; // rax
  
  v1 = a0 + a1;
  if (v1 <= a0)
    return 1;
  do {
    v2 = getrandom(a0,v1 - a0,0);
    if (0 <= v2) // branch-flip
      a0 += v2;
    else if (*__errno_location() != 4)
      return 0;
  } while (a0 < v1);
  return 1;
}

// Function: sub_11540 @ 0x11540
unsigned long sub_11540(long *a0)
{
  long v1;
  
  v1 = *a0;
  __explicit_bzero_chk(a0,0x1038,0xffffffffffffffff);
  free(a0);
  if (v1)
    return sub_c900(v1); // tail-call
  return 0;
}

// Function: sub_11590 @ 0x11590
void sub_11590(unsigned long *a0,long a1,long a2)
{
  unsigned long v1;
  int v2; // eax
  int *v3; // rax
  long v4; // rax
  int v5;
  
  v3 = __errno_location();
  while( true ) {
    v4 = fread_unlocked(a1,1,a2,*a0);
    v5 = *v3;
    a1 += v4;
    a2 -= v4;
    if (!a2) break;
    v2 = ferror_unlocked((void *)*a0);
    v1 = a0[2];
    if (!v2)
      v5 = 0;
    *v3 = v5;
    (*(void *)a0[1])(v1);
  }
}

// Function: sub_11610 @ 0x11610
void sub_11610(unsigned long *a0,void *a1,unsigned long a2)
{
  unsigned long *v1;
  unsigned long v2;
  unsigned long v3;
  void *v4; // r13
  unsigned long *v5;
  unsigned long v6;
  
  v5 = &a0[0x104];
  v1 = &a0[1];
  v6 = *a0;
  if (a2 <= v6) // branch-flip
    v5 = (unsigned long *)((long)v5 + (0x800 - v6));
  else {
    do {
      v4 = (void *)((long)a1 + v6);
      a2 -= v6;
      memcpy(a1,(void *)((0x800 - v6) + (long)v5),v6);
      if (!((unsigned long)v4 & 7)) {
        v3 = (unsigned long)((unsigned int)a2 & 0x7ff);
        v2 = a2;
        goto label_116b4;
      }
      v6 = 0x800;
      sub_11880(v1,v5);
      a1 = v4;
    } while (0x801 <= a2);
    v3 = a2;
label_116ca:
    v6 = 0x800;
    a2 = v3;
  }
  memcpy(a1,v5,a2);
  v6 -= a2;
  goto label_116e4;
  while( true ) {
    sub_11880(v1,a1);
    v2 -= 0x800;
    v6 = 0;
    if (!v2) break;
label_116b4:
    a1 = (void *)((long)v4 + (a2 - v2));
    if (v3 == v2) {
      sub_11880(v1,v5);
      goto label_116ca;
    }
  }
label_116e4:
  *a0 = v6;
  return;
}

// Function: sub_11710 @ 0x11710
long sub_11710(long a0,unsigned long a1)
{
  int v1;
  void *v2; // rax
  long v3; // rax
  unsigned long v4;
  long v5; // rax
  int *v6; // rax
  
  if (!a1)
    return sub_11440(0,0); // tail-call
  if (a0) { // branch-flip
    v2 = (void *)sub_14100(a0,"rb");
    if (v2) { // branch-flip
      v3 = sub_11440(v2,a0);
      v4 = 0x1000;
      if (a1 <= 0x1000)
        v4 = a1;
      setvbuf(v2,(char *)(v3 + 0x18),0,v4);
    }
    else {
      v3 = 0;
    }
  }
  else {
    v5 = sub_11440(0,0);
    *(unsigned long *)(v5 + 0x18) = 0;
    v4 = 0x800;
    if (a1 <= 0x800)
      v4 = a1;
    if (sub_114e0(v5 + 0x20,v4)) {
      sub_11b50(v5 + 0x20);
      return v5;
    }
    v6 = __errno_location();
    v3 = 0;
    v1 = *v6;
    sub_11540(v5);
    *v6 = v1;
  }
  return v3;
}

// Function: sub_11810 @ 0x11810
void sub_11810(long a0,unsigned long a1)
{
  *(unsigned long *)(a0 + 8) = a1;
}

// Function: sub_11820 @ 0x11820
void sub_11820(long a0,unsigned long a1)
{
  *(unsigned long *)(a0 + 0x10) = a1;
}

// Function: sub_11830 @ 0x11830
void sub_11830(long *a0)
{
  if (*a0) {
    sub_11590(); // tail-call
    return;
  }
  sub_11610(&a0[3]); // tail-call
}

// Function: sub_11850 @ 0x11850
void sub_11850(void)
{
  sub_11540(); // tail-call
}

// Function: sub_11860 @ 0x11860
unsigned long sub_11860(unsigned long a0)
{
  return a0;
}

// Function: sub_11870 @ 0x11870
unsigned long sub_11870(long a0,unsigned int a1)
{
  return *(unsigned long *)(a0 + (unsigned long)(a1 & 0x7f8));
}

// Function: sub_11880 @ 0x11880
void sub_11880(long *a0,long *a1)
{
  long v1; // rax
  long v10; // rax
  unsigned long v11; // rax
  long v12; // rax
  long v13; // rax
  long v14; // rax
  unsigned long v15; // rax
  long v16; // rax
  long v17; // rax
  unsigned long v18; // rcx
  long *v19; // rdx
  long v2;
  long v20; // rdx
  long v21; // rdx
  long v22; // rdx
  long v23; // rdx
  long v24; // rdx
  long v25; // rdx
  long *v26; // rdx
  long *v27;
  long *v28; // rdx
  long v29; // rdx
  unsigned long v3; // rax
  long v30; // rdx
  long v31; // rdx
  long v32; // rdx
  long v33; // rdx
  long v34; // rdx
  long *v35; // rdx
  unsigned long v36;
  long v37;
  long *v38;
  long *v39;
  long v4; // rax
  long *v40;
  long v5; // rax
  long v6; // rax
  unsigned long v7; // rax
  long v8; // rax
  long v9; // rax
  
  v39 = &a0[0x80];
  v37 = a0[0x102];
  v36 = a0[0x100];
  a0[0x102] = v37 + 1;
  v37 = v37 + 1 + a0[0x101];
  v38 = a0;
  v40 = a1;
  do {
    v2 = *a0;
    v36 = ~(v36 << 0x15 ^ v36) + a0[0x80];
    sub_11870(v38,v2);
    *v19 = v1 + v36 + v37;
    v2 = sub_11860(sub_11870() + v2);
    *a1 = v2;
    sub_11860(v36);
    v37 = *(long *)(v20 + 8);
    v18 = (v3 >> 5 ^ v36) + *(long *)(v20 + 0x408);
    sub_11870(v38,v37);
    *(unsigned long *)(v21 + 8) = v4 + v2 + v18;
    v36 = v18;
    sub_11860(sub_11870() + v37);
    a1[1] = v5;
    v37 = *(long *)(v22 + 0x10);
    v36 = (v18 << 0xc ^ v36) + *(long *)(v22 + 0x410);
    sub_11870(v38,v37);
    *(unsigned long *)(v23 + 0x10) = v6 + v5 + v36;
    v2 = sub_11860(sub_11870() + v37);
    a1[2] = v2;
    sub_11860(v36);
    v37 = *(long *)(v24 + 0x18);
    v36 = (v7 >> 0x21 ^ v36) + *(long *)(v24 + 0x418);
    sub_11870(v38,v37);
    *(unsigned long *)(v25 + 0x18) = v8 + v2 + v36;
    v2 = sub_11870();
    a1 = &a1[4];
    sub_11860(v2 + v37);
    a1[-1] = v9;
    a0 = v26;
    v37 = v9;
  } while (v26 != v39);
  v40 = &v40[0x80];
  v39 = &v38[0x100];
  v27 = v26;
  do {
    v2 = *v27;
    v36 = ~(v36 << 0x15 ^ v36) + v27[-0x80];
    sub_11870(v38,v2);
    *v28 = v10 + v36 + v37;
    v2 = sub_11860(sub_11870() + v2);
    *v40 = v2;
    sub_11860(v36);
    v37 = *(long *)(v29 + 8);
    v18 = (v11 >> 5 ^ v36) + *(long *)(v29 + -0x3f8);
    sub_11870(v38,v37);
    *(unsigned long *)(v30 + 8) = v12 + v2 + v18;
    v36 = v18;
    sub_11860(sub_11870() + v37);
    v40[1] = v13;
    v37 = *(long *)(v31 + 0x10);
    v36 = (v36 << 0xc ^ v18) + *(long *)(v31 + -0x3f0);
    sub_11870(v38,v37);
    *(unsigned long *)(v32 + 0x10) = v14 + v13 + v36;
    v2 = sub_11860(sub_11870() + v37);
    v40[2] = v2;
    sub_11860(v36);
    v37 = *(long *)(v33 + 0x18);
    v36 = (v15 >> 0x21 ^ v36) + *(long *)(v33 + -1000);
    sub_11870(v38,v37);
    *(unsigned long *)(v34 + 0x18) = v16 + v2 + v36;
    v2 = sub_11870();
    v40 = &v40[4];
    sub_11860(v2 + v37);
    v40[-1] = v17;
    v27 = v35;
    v37 = v17;
  } while (v35 != v39);
  v38[0x100] = v36;
  v38[0x101] = v17;
}

// Function: sub_11b50 @ 0x11b50
void sub_11b50(unsigned long *a0)
{
  unsigned long v1;
  unsigned long *v10; // rcx
  long v11; // rdx
  long v12; // rdx
  long v13; // rdx
  long v14; // rdx
  long v15; // rdx
  unsigned long *v16;
  unsigned long v17;
  unsigned long v18;
  unsigned long v19; // rax
  unsigned long v2;
  unsigned long v20; // r10
  unsigned long v21; // r11
  unsigned long v22;
  unsigned long v23;
  long v24; // rdi
  long v25; // r12
  long v26; // r15
  long v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long *v9;
  
  v2 = 0x98f5704f6c44c0ab;
  v19 = 0xb29b2e824a595524;
  v23 = 0xae985bf2cbfc89ed;
  v21 = 0x8c0ea5053d4712a0;
  v18 = 0xb9f8b322c73ac862;
  v20 = 0x647c4677a2884b7c;
  v17 = 0x48fe4a0fa5a09315;
  v9 = a0;
  v16 = a0;
  do {
    v22 = v9[5];
    v3 = v2 + v9[7];
    v2 = *v9;
    sub_11860(v3);
    v24 = (v2 - v11) + v20;
    v2 = v3 + v24;
    v22 = v4 >> 9 ^ v17 + v22;
    v17 = v24 * 0x200 ^ v23 + v9[6];
    v3 = (v9[1] - v22) + v18;
    v18 = sub_11860(v3);
    v25 = (v9[2] - v17) + v21;
    v2 = v18 >> 0x17 ^ v2;
    v18 = v3 + v25;
    v20 = v25 * 0x8000 ^ v24 + v3;
    v3 = (v9[3] - v2) + v19;
    sub_11860(v3);
    v23 = v25 + v3;
    v18 = v5 >> 0xe ^ v18;
    v24 = v22 - v18;
    v21 = v12 << 0x14;
    sub_11860(v24);
    v21 ^= v23;
    *v9 = v20;
    v25 = v17 - v21;
    v9[1] = v18;
    v9[2] = v21;
    v19 = v6 >> 0x11 ^ v3 + v12;
    v17 = v24 + v25;
    v10 = &v9[8];
    v2 -= v19;
    v9[3] = v19;
    v23 = v25 + v2;
    v9[4] = v25 * 0x4000 ^ v13 + v24;
    v9[5] = v17;
    v9[6] = v23;
    v9[7] = v2;
    v9 = v10;
  } while (&a0[0x100] != v10);
  do {
    v22 = v16[5];
    v3 = v2 + v16[7];
    v2 = *v16;
    sub_11860(v3);
    v1 = v16[2];
    v24 = (v2 - v14) + v20;
    v2 = v3 + v24;
    v17 = v7 >> 9 ^ v17 + v22;
    v22 = v23 + v16[6] ^ v24 * 0x200;
    v3 = (v16[1] - v17) + v18;
    v23 = sub_11860(v3);
    v25 = (v1 - v22) + v21;
    v18 = v3 + v25;
    v2 = v23 >> 0x17 ^ v2;
    v26 = (v16[3] - v2) + v19;
    v20 = v25 * 0x8000 ^ v24 + v3;
    sub_11860(v26);
    v18 = v8 >> 0xe ^ v18;
    v3 = v17 - v18;
    v21 = v15 << 0x14 ^ v25 + v26;
    v23 = v15 + v3;
    v24 = v22 - v21;
    v19 = sub_11860(v3);
    v17 = v3 + v24;
    *v16 = v20;
    v16[1] = v18;
    v9 = &v16[8];
    v16[2] = v21;
    v19 = v19 >> 0x11 ^ v26 + v15;
    v16[4] = v24 * 0x4000 ^ v23;
    v2 -= v19;
    v16[3] = v19;
    v23 = v24 + v2;
    v16[5] = v17;
    v16[6] = v23;
    v16[7] = v2;
    v16 = v9;
  } while (&a0[0x100] != v9);
  a0[0x102] = 0;
  a0[0x101] = 0;
  a0[0x100] = 0;
}

// Function: sub_11df0 @ 0x11df0
void sub_11df0(long *a0)
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

// Function: sub_11ee0 @ 0x11ee0
void sub_11ee0(unsigned long *a0)
{
  unsigned long v1;
  unsigned long v2;
  
  v2 = dat_1dfc8;
  v1 = dat_1dfb8;
  *a0 = 0;
  a0[1] = 0;
  a0[2] = 0;
  _obstack_begin(&a0[3],0,0,dat_1dfb8,dat_1dfc8);
  _obstack_begin(&a0[0xe],0,0,v1,v2);
  _obstack_begin(&a0[0x19],0,0,v1,v2); // tail-call
}

// Function: sub_11f60 @ 0x11f60
void sub_11f60(long a0)
{
  _obstack_free(a0 + 0x18,0);
  _obstack_free(a0 + 0x70,0);
  _obstack_free(a0 + 200,0); // tail-call
}

// Function: sub_11f90 @ 0x11f90
unsigned long sub_11f90(void *a0,long a1)
{
  int v1; // eax
  char *v2;
  long v3; // rax
  unsigned long v4; // rax
  unsigned long *v5;
  unsigned long v6;
  
label_11fbe:
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
        sub_11df0(a1);
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
    if (*(char **)(a1 + 0x38) != v2) goto label_11fb0;
    _obstack_newchunk(a1 + 0x18,1);
    v2 = *(char **)(a1 + 0x30);
    *(char **)(a1 + 0x30) = &v2[1];
    *v2 = (char)v1;
  } while (v1);
  goto label_12000;
label_11fb0:
  *(char **)(a1 + 0x30) = &v2[1];
  *v2 = (char)v1;
  if (!v1) {
label_12000:
    sub_11df0(a1);
  }
  goto label_11fbe;
}

// Function: sub_12180 @ 0x12180
void sub_12180(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_12190 @ 0x12190
unsigned long sub_12190(unsigned long a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_12180();
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

// Function: sub_12240 @ 0x12240
void sub_12240(void)
{
  sub_12190(); // tail-call
}

// Function: sub_12250 @ 0x12250
void sub_12250(void)
{
  sub_12180(); // tail-call
}

// Function: sub_12260 @ 0x12260
void sub_12260(void)
{
  sub_127a0(mkstemp()); // tail-call
}

// Function: sub_12280 @ 0x12280
void sub_12280(unsigned long a0,unsigned int a1)
{
  sub_14080(mkostemp(),a1); // tail-call
}

// Function: sub_122a0 @ 0x122a0
unsigned int sub_122a0(char *a0,char *a1,char a2)
{
  unsigned int v1;
  char *v2;
  char v3;
  int v4; // ecx
  int v5; // eax
  
  if (*a0 != a2) {
    if (a2 != *a1)
      return 0;
    v4 = (int)a1[1];
    a1 = &a1[1];
label_12338:
    while ((char)v4 == '0') {
      v2 = &a1[1];
      a1 = &a1[1];
      v4 = (int)*v2;
    }
    return -(unsigned int)((unsigned int)(v4 - 0x30U) < 10);
  }
  if (a2 != *a1) { // branch-flip
    v5 = (int)a0[1];
    a0 = &a0[1];
  }
  else {
    while( true ) {
      v2 = &a0[1];
      v5 = (int)*v2;
      v3 = a1[1];
      v4 = (int)v3;
      a0 = &a0[1];
      a1 = &a1[1];
      if (*v2 != v3) break;
      if (10 <= (unsigned int)(v5 - 0x30U))
        return 0;
    }
    v1 = (int)v3 - 0x30;
    if (10 <= (unsigned int)(v5 - 0x30U)) {
      if (10 <= v1)
        return 0;
      goto label_12338;
    }
    if (v1 <= 9)
      return v5 - v3;
  }
  while ((char)v5 == '0') {
    v2 = &a0[1];
    a0 = &a0[1];
    v5 = (int)*v2;
  }
  return (unsigned int)((unsigned int)(v5 - 0x30U) <= 9);
}

// Function: sub_12370 @ 0x12370
unsigned long sub_12370(unsigned char *a0,unsigned char *a1,unsigned int a2,unsigned int a3)
{
  unsigned char *v1;
  unsigned int v10;
  unsigned char v2;
  unsigned char v3;
  unsigned long v4; // rcx
  unsigned int v5;
  unsigned int v6;
  unsigned long v7; // rsi
  unsigned char *v8;
  unsigned int v9;
  
  v9 = (unsigned int)*a0;
  v2 = *a1;
  v5 = (unsigned int)v2;
  if (*a0 != 0x2d) {
    if (v2 != 0x2d) {
      while (((char)v9 == '0' || (v9 == a3))) {
        v8 = &a0[1];
        a0 = &a0[1];
        v9 = (unsigned int)*v8;
      }
      if (v2 != 0x30) goto label_123f6;
      do {
        do {
          v8 = &a1[1];
          v5 = (unsigned int)*v8;
          a1 = &a1[1];
        } while (*v8 == 0x30);
label_123f6:
        v6 = v5;
        v10 = v9;
      } while (v5 == a3);
label_12400:
      do {
        v8 = a1;
        if (((char)v9 != (char)v6) || (10 <= v5 - 0x30)) goto label_125c0;
        do {
          v1 = &a0[1];
          v9 = (unsigned int)*v1;
          a0 = &a0[1];
        } while (*v1 == a3);
        do {
          v5 = (unsigned int)v8[1];
          a1 = &v8[1];
          v6 = v5;
          v10 = v9;
          if (v5 != a3) goto label_12400;
          v5 = (unsigned int)v8[2];
          v8 = &v8[2];
          a1 = v8;
          v6 = v5;
        } while (v5 == a3);
      } while( true );
    }
    do {
      do {
        v2 = a1[1];
        v5 = (unsigned int)v2;
        a1 = &a1[1];
      } while (v2 == 0x30);
    } while (v2 == a3);
    if (v2 == a2) {
      do {
        v5 = (unsigned int)a1[1];
        if (a1[1] != 0x30) break;
        v8 = &a1[2];
        v5 = (unsigned int)*v8;
        a1 = &a1[2];
      } while (*v8 == 0x30);
    }
    if (10 <= v5 - 0x30) {
      while (((char)v9 == '0' || (v9 == a3))) {
        v8 = &a0[1];
        a0 = &a0[1];
        v9 = (unsigned int)*v8;
      }
      if (v9 == a2) {
        do {
          v8 = &a0[1];
          a0 = &a0[1];
        } while (*v8 == 0x30);
        v9 = (unsigned int)*v8;
      }
      return (unsigned long)(v9 - 0x30 <= 9);
    }
    return 1;
  }
  do {
    do {
      v3 = a0[1];
      a0 = &a0[1];
    } while (v3 == 0x30);
    v6 = (unsigned int)v3;
  } while (v3 == a3);
  if (v2 != 0x2d) {
    if (v3 == a2) {
      do {
        v3 = a0[1];
        a0 = &a0[1];
      } while (v3 == 0x30);
    }
    if (10 <= (unsigned int)v3 - 0x30) {
      while (((char)v5 == '0' || (v5 == a3))) {
        v8 = &a1[1];
        a1 = &a1[1];
        v5 = (unsigned int)*v8;
      }
      if (v5 == a2) {
        do {
          v8 = &a1[1];
          a1 = &a1[1];
        } while (*v8 == 0x30);
        v5 = (unsigned int)*v8;
      }
      return (unsigned long)-(unsigned int)(v5 - 0x30 < 10);
    }
    return 0xffffffff;
  }
  do {
    do {
      v2 = a1[1];
      a1 = &a1[1];
    } while (v2 == 0x30);
    v10 = (unsigned int)v2;
    v9 = (unsigned int)v2;
    v5 = (unsigned int)v3;
  } while (v2 == a3);
label_124d0:
  do {
    v8 = a1;
    if (((char)v5 != (char)v9) || (9 < v10 - 0x30)) goto label_1263d;
    do {
      v1 = &a0[1];
      v6 = (unsigned int)*v1;
      a0 = &a0[1];
    } while (*v1 == a3);
    do {
      v10 = (unsigned int)v8[1];
      a1 = &v8[1];
      v9 = v10;
      v5 = v6;
      if (v10 != a3) goto label_124d0;
      v10 = (unsigned int)v8[2];
      v8 = &v8[2];
      a1 = v8;
      v9 = v10;
    } while (v10 == a3);
  } while( true );
label_125c0:
  if ((a2 == v10) && (10 <= v5 - 0x30))
    return sub_122a0(a0,v8,(int)(char)a2); // return-dupe, tail-call
  v9 = v10 - v5;
  if (v5 != a2) { // branch-flip
    if (v10 - 0x30 <= 9) goto label_125e2;
    v4 = 0;
    if (10 <= v5 - 0x30)
      return 0;
  }
  else {
    if (10 <= v10 - 0x30) {
      return sub_122a0(a0,v8,(int)(char)a2);
    }
label_125e2:
    v4 = 0;
    do {
      do {
        v1 = &a0[1];
        a0 = &a0[1];
      } while (*v1 == a3);
      v4 += 1;
    } while ((unsigned int)*v1 - 0x30 <= 9);
    if (9 < v5 - 0x30)
      return (unsigned long)(v4 != 0);
  }
  v7 = 0;
  do {
    do {
      v1 = &v8[1];
      v8 = &v8[1];
    } while (*v1 == a3);
    v7 += 1;
  } while ((unsigned int)*v1 - 0x30 <= 9);
  if (v4 != v7)
    return (unsigned long)(-(unsigned int)(v4 < v7) | 1);
  goto label_126ae;
label_1263d:
  if ((a2 == v6) && (10 <= v10 - 0x30))
    return sub_122a0(v8,a0,(int)(char)a2); // return-dupe, tail-call
  v9 = v10 - v6;
  if (v10 != a2) { // branch-flip
    if (v6 - 0x30 <= 9) goto label_12661;
    v4 = 0;
    if (10 <= v10 - 0x30)
      return 0;
  }
  else {
    if (10 <= v6 - 0x30) {
      return sub_122a0(v8,a0,(int)(char)a2);
    }
label_12661:
    v4 = 0;
    do {
      do {
        v1 = &a0[1];
        a0 = &a0[1];
      } while (*v1 == a3);
      v4 += 1;
    } while ((unsigned int)*v1 - 0x30 <= 9);
    if (9 < v10 - 0x30)
      return (unsigned long)-(unsigned int)(v4 != 0);
  }
  v7 = 0;
  do {
    do {
      v1 = &v8[1];
      v8 = &v8[1];
    } while (*v1 == a3);
    v7 += 1;
  } while ((unsigned int)*v1 - 0x30 <= 9);
  if (v4 != v7)
    return (unsigned long)((-(unsigned int)(v4 < v7) & 2) - 1);
label_126ae:
  v7 = (unsigned long)v9;
  if (!v4)
    v7 = 0;
  return v7;
}

// Function: sub_12790 @ 0x12790
void sub_12790(void)
{
  sub_12370(); // tail-call
}

// Function: sub_127a0 @ 0x127a0
unsigned int sub_127a0(unsigned int a0)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  
  if (3 <= a0)
    return a0;
  v2 = sub_14960();
  v3 = __errno_location();
  v1 = *v3;
  close(a0);
  *v3 = v1;
  return v2;
}

// Function: sub_12800 @ 0x12800
long sub_12800(void *a0,long a1,unsigned long a2,unsigned long a3,unsigned long *a4,unsigned long a5)
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
label_1292d:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_12966:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0x179a8) + 0x179a8;
  switch(a5) {
    case 0:
      goto label_12966;
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
      goto label_1292d;
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

// Function: sub_12c80 @ 0x12c80
void sub_12c80(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_12800(); // tail-call
}

// Function: sub_12ca0 @ 0x12ca0
void sub_12ca0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
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
  sub_12800(a0,a1,a2,a3);
}

// Function: sub_12d40 @ 0x12d40
void sub_12d40(void)
{
  sub_12ca0();
}

// Function: sub_12df0 @ 0x12df0
void sub_12df0(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-coreutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_12ea0 @ 0x12ea0
long sub_12ea0(long a0)
{
  if (a0)
    return a0;
  sub_13270(); // no-return
}

// Function: sub_12eb0 @ 0x12eb0
void sub_12eb0(unsigned long a0)
{
  sub_12ea0(malloc(a0)); // tail-call
}

// Function: sub_12ed0 @ 0x12ed0
void sub_12ed0(void)
{
  sub_12ea0(sub_141a0()); // tail-call
}

// Function: sub_12ef0 @ 0x12ef0
void sub_12ef0(void)
{
  sub_12eb0(); // tail-call
}

// Function: sub_12f00 @ 0x12f00
void sub_12f00(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_13270(); // no-return
}

// Function: sub_12f40 @ 0x12f40
void sub_12f40(void)
{
  sub_12ea0(sub_141b0()); // tail-call
}

// Function: sub_12f60 @ 0x12f60
void sub_12f60(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_13270(); // no-return
}

// Function: sub_12fa0 @ 0x12fa0
void sub_12fa0(void)
{
  sub_12f60(); // tail-call
}

// Function: sub_12fb0 @ 0x12fb0
void sub_12fb0(void)
{
  sub_12ea0(sub_141e0()); // tail-call
}

// Function: sub_12fd0 @ 0x12fd0
void sub_12fd0(unsigned long a0,unsigned long a1)
{
  sub_12f60(0,a0,a1); // tail-call
}

// Function: sub_12fe0 @ 0x12fe0
void sub_12fe0(unsigned long a0,unsigned long a1)
{
  sub_12fb0(0,a0,a1); // tail-call
}

// Function: sub_12ff0 @ 0x12ff0
void sub_12ff0(long a0,unsigned long *a1,unsigned long a2)
{
  char v1 [16];
  unsigned long v2; // rax
  unsigned long v3;
  bool v4; // cf
  
  v3 = *a1;
  if (a0) { // branch-flip
    v2 = (v3 >> 1) + 1;
    v4 = CARRY8(v3,v2);
    v3 += v2;
    if (v4)
      sub_13270(); // no-return
  }
  else if (!v3) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v3 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0) + (unsigned long)(0x81 <= a2);
  }
  sub_12f60(a0,v3,a2);
  *a1 = v3;
}

// Function: sub_13060 @ 0x13060
void sub_13060(unsigned long a0,unsigned long a1)
{
  sub_12ff0(a0,a1,1); // tail-call
}

// Function: sub_13070 @ 0x13070
void sub_13070(long a0,long *a1,long a2,long a3,long a4)
{
  long v1;
  long v2;
  long v3;
  
  v1 = *a1;
  v2 = (v1 >> 1) + v1;
  if (SCARRY8(v1 >> 1,v1))
    v2 = 0x7fffffffffffffff;
  v3 = a3;
  if (v2 <= a3)
    v3 = v2;
  if (0 <= a3)
    v2 = v3;
  v3 = v2 * a4;
  if (SEXT816(v3) != SEXT816(v2) * SEXT816(a4)) // branch-flip
    v3 = 0x7fffffffffffffff;
  else {
    if (0x80 <= v3) goto label_13116;
    v3 = 0x80;
  }
  v2 = v3 / a4;
  v3 -= v3 % a4;
label_13116:
  if (!a0)
    *a1 = 0;
  if ((v2 - v1 < a2) && ((v2 = v1 + a2, SCARRY8(v1,a2) || (((a3 < v2 && (0 <= a3)) || (v3 = v2 * a4, SEXT816(v3) != SEXT816(v2) * SEXT816(a4)))))))
    sub_13270(a0,v3); // no-return
  sub_12f00(a0,v3);
  *a1 = v2;
}

// Function: sub_13150 @ 0x13150
void sub_13150(unsigned long a0,unsigned long a1)
{
  sub_12ea0(calloc(a0,a1)); // tail-call
}

// Function: sub_13170 @ 0x13170
void sub_13170(unsigned long a0)
{
  sub_13150(a0,1); // tail-call
}

// Function: sub_13180 @ 0x13180
void sub_13180(void)
{
  sub_12ea0(sub_141d0()); // tail-call
}

// Function: sub_131a0 @ 0x131a0
void sub_131a0(unsigned long a0)
{
  sub_13180(a0,1); // tail-call
}

// Function: sub_131b0 @ 0x131b0
void sub_131b0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_12eb0(a1),a0,a1); // tail-call
}

// Function: sub_131e0 @ 0x131e0
void sub_131e0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_12ed0(a1),a0,a1); // tail-call
}

// Function: sub_13210 @ 0x13210
void sub_13210(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_12ed0(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_13250 @ 0x13250
void sub_13250(char *a0)
{
  sub_131b0(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_13270 @ 0x13270
void sub_13270(void)
{
  error(dat_1e130,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_132b0 @ 0x132b0
void sub_132b0(int a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  
  error(0,a0,dcgettext(NULL,"string comparison failed",5));
  error(0,0,dcgettext(NULL,"Set LC_ALL=\'C\' to work around the problem.",5));
  v1 = sub_11100(1,8,a3,a4);
  v2 = sub_11100(0,8,a1,a2);
  error(dat_1e130,0,dcgettext(NULL,"The strings compared were %s and %s.",5),v2,v1); // tail-call
}

// Function: sub_13380 @ 0x13380
unsigned int sub_13380(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  int v1;
  unsigned int v2; // eax
  
  v2 = sub_14390(a0,a1,a2,a3);
  v1 = *__errno_location();
  if (!v1)
    return v2;
  sub_132b0(v1,a0,a1,a2,a3);
  return v2;
}

// Function: sub_133e0 @ 0x133e0
unsigned int sub_133e0(unsigned long a0,long a1,unsigned long a2,long a3)
{
  int v1;
  unsigned int v2; // eax
  
  v2 = sub_14440(a0,a1,a2,a3);
  v1 = *__errno_location();
  if (!v1)
    return v2;
  sub_132b0(v1,a0,a1 + -1,a2,a3 + -1);
  return v2;
}

// Function: sub_13440 @ 0x13440
unsigned long sub_13440(double a0)
{
  unsigned int *v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x38
  char v4 [16];
  unsigned long v5; // stack - 0x30
  
  v1 = (unsigned int *)__errno_location();
  if (dat_17108 <= a0) {
    do {
      pause();
      if (*v1 != 4) break;
      pause();
    } while (*v1 == 4);
  }
  v4 = sub_13d70();
  v5 = SUB168(v4,8);
  v3 = SUB168(v4,0);
  do {
    *v1 = 0;
    v2 = sub_144b0(&v3,&v3);
    if (!(int)v2)
      return v2;
  } while (!(*v1 & 0xfffffffb));
  return 0xffffffff;
}

// Function: sub_134f0 @ 0x134f0
void sub_134f0(unsigned int a0,int a1,char a2,long a3,unsigned long a4,int a5)
{
  char v1; // stack - 0x32
  char *v2;
  char *v3; // r12
  char *v4;
  char v5; // stack - 0x31
  
  if (4 <= a0) { // branch-flip
    if (a0 != 4)
      abort(); // no-return, return-dupe
    v2 = "invalid %s%s argument \'%s\'";
  }
  else if (2 <= a0) // branch-flip
    v2 = "invalid suffix in %s%s argument \'%s\'";
  else {
    if (a0 != 1) {
      abort();
    }
    v2 = "%s%s argument \'%s\' too large";
  }
  if (0 <= a1) { // branch-flip
    v3 = "--";
    v4 = *(char **)(a3 + (long)a1 * 0x20);
  }
  else {
    v4 = &v1;
    v5 = 0;
    v3 = &"--"[-(long)a1];
    v1 = a2;
  }
  error(a5,0,dcgettext(NULL,v2,5),v3,v4,a4);
}

// Function: sub_135d0 @ 0x135d0
void sub_135d0(unsigned long a0,unsigned long a1,char a2,unsigned long a3,unsigned long a4)
{
  sub_134f0(a0,a1,(int)a2,a3,a4,dat_1e130);
  abort(); // no-return
}

// Function: sub_135f0 @ 0x135f0
bool sub_135f0(unsigned long *a0,int a1)
{
  char v1 [16];
  char v2 [16];
  unsigned long v3; // rax
  bool v4; // cf
  
  v1._8_8_ = 0;
  v1._0_8_ = (long)a1;
  v2._8_8_ = 0;
  v2._0_8_ = *a0;
  v3 = SUB168(v1._0_16_ * v2._0_16_,0);
  v4 = SUB168(v1._0_16_ * v2._0_16_,8) != 0;
  if (v4)
    v3 = 0xffffffffffffffff;
  *a0 = v3;
  return v4;
}

// Function: sub_13620 @ 0x13620
unsigned int sub_13620(unsigned long a0,unsigned long a1,int a2)
{
  unsigned int v1; // r9d
  
  v1 = 0;
  do {
    a2 -= 1;
    v1 |= sub_135f0();
  } while (a2);
  return v1;
}

// Function: sub_13650 @ 0x13650
unsigned int sub_13650(unsigned char *a0,unsigned long *a1,unsigned int a2,unsigned long *a3,char *a4)
{
  unsigned char v1;
  unsigned int v10;
  unsigned int v2; // eax
  int *v3; // rax
  unsigned char *v4;
  unsigned char v5;
  long v6;
  unsigned long v7; // stack - 0x48
  int v8; // ecx
  unsigned long v9; // r8
  
  if (0x25 <= a2)
    __assert_fail("0 <= strtol_base && strtol_base <= 36","lib/xstrtol.c",0x55,"xstrtoumax"); // no-return
  if (!a1)
    a1 = &v6;
  v3 = __errno_location();
  *v3 = 0;
  v5 = *a0;
  v4 = a0;
  while (*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)v5 * 2) & 0x20) {
    v5 = v4[1];
    v4 = &v4[1];
  }
  if (v5 == 0x2d)
    return 4;
  v7 = strtoumax(a0,a1,a2);
  v4 = (unsigned char *)*a1;
  if (v4 != a0) { // branch-flip
    if (*v3) { // branch-flip
      v10 = 1;
      if (*v3 != 0x22)
        return 4;
    }
    else {
      v10 = 0;
    }
    if (!a4) {
      *a3 = v7;
      return v10;
    }
    v5 = *v4;
    if (!v5) goto label_13712;
    v8 = (int)(char)v5;
  }
  else {
    if ((!a4) || (v5 = *a0, !v5))
      return 4;
    v8 = (int)(char)v5;
    if (!strchr(a4,v8))
      return 4;
    v7 = 1;
    v10 = 0;
  }
  if (!strchr(a4,v8)) {
label_13719:
    *a3 = v7;
    return v10 | 2;
  }
  v2 = (unsigned int)v5 - 0x45;
  v8 = 1;
  v9 = 0x400;
  if (((unsigned char)v2 <= 0x2f) && (0x814400308945U >> ((unsigned long)v2 & 0x3f) & 1)) {
    v9 = 0x400;
    v8 = 1;
    if (strchr(a4,0x30)) {
      v1 = v4[1];
      if (v1 != 0x44) { // branch-flip
        if (v1 != 0x69) { // branch-flip
          v8 = (v1 == 0x42) + 1;
          v9 = 0x400;
          if (v1 == 0x42)
            v9 = 1000;
        }
        else {
          v8 = (v4[2] == 0x42) + 1 + (unsigned int)(v4[2] == 0x42);
        }
      }
      else {
        v8 = 2;
        v9 = 1000;
      }
    }
  }
  switch((unsigned int)v5 - 0x42 & 0xff) {
    case 0:
      v10 |= sub_135f0(&v7,0x400);
      break;
    default:
      goto label_13719;
    case 3:
      v10 |= sub_13620(&v7,v9,6);
      break;
    case 5:
    case 0x25:
      v10 |= sub_13620(&v7,v9,3);
      break;
    case 9:
    case 0x29:
      v10 |= sub_13620(&v7,v9,1);
      break;
    case 0xb:
    case 0x2b:
      v10 |= sub_13620(&v7,v9,2);
      break;
    case 0xe:
      v10 |= sub_13620(&v7,v9,5);
      break;
    case 0x12:
    case 0x32:
      v10 |= sub_13620(&v7,v9,4);
      break;
    case 0x17:
      v10 |= sub_13620(&v7,v9,8);
      break;
    case 0x18:
      v10 |= sub_13620(&v7,v9,7);
      break;
    case 0x20:
      v10 |= sub_135f0(&v7,0x200);
      break;
    case 0x21:
      break;
    case 0x35:
      v10 |= sub_135f0(&v7,2);
    
  }
  *a1 = &v4[v8];
  if (v4[v8])
    v10 |= 2;
label_13712:
  *a3 = v7;
  return v10;
}

// Function: sub_13a30 @ 0x13a30
unsigned long sub_13a30(unsigned long a0,unsigned char a1)
{
  return a0 << (a1 & 0x3f) | a0 >> 0x40 - (a1 & 0x3f);
}

// Function: sub_13a40 @ 0x13a40
unsigned long sub_13a40(unsigned long a0,unsigned char a1)
{
  return a0 >> (a1 & 0x3f) | a0 << 0x40 - (a1 & 0x3f);
}

// Function: sub_13a50 @ 0x13a50
unsigned int sub_13a50(unsigned int a0,unsigned char a1)
{
  return a0 << (a1 & 0x1f) | a0 >> 0x20 - (a1 & 0x1f);
}

// Function: sub_13a60 @ 0x13a60
unsigned int sub_13a60(unsigned int a0,unsigned char a1)
{
  return a0 >> (a1 & 0x1f) | a0 << 0x20 - (a1 & 0x1f);
}

// Function: sub_13a70 @ 0x13a70
void sub_13a70(void)
{
  sub_13a30(); // tail-call
}

// Function: sub_13a80 @ 0x13a80
void sub_13a80(void)
{
  sub_13a40(); // tail-call
}

// Function: sub_13a90 @ 0x13a90
unsigned int sub_13a90(unsigned int a0,unsigned char a1)
{
  return CONCAT22((short)((unsigned int)a0 >> 0x10),(unsigned short)a0 << (a1 & 0xf) | (unsigned short)a0 >> 0x10 - (a1 & 0xf));
}

// Function: sub_13aa0 @ 0x13aa0
unsigned int sub_13aa0(unsigned int a0,unsigned char a1)
{
  return CONCAT22((short)((unsigned int)a0 >> 0x10),(unsigned short)a0 >> (a1 & 0xf) | (unsigned short)a0 << 0x10 - (a1 & 0xf));
}

// Function: sub_13ab0 @ 0x13ab0
unsigned int sub_13ab0(unsigned int a0,unsigned char a1)
{
  return CONCAT31((undefined3)((unsigned int)a0 >> 8),(unsigned char)a0 << (a1 & 7) | (unsigned char)a0 >> 8 - (a1 & 7));
}

// Function: sub_13ac0 @ 0x13ac0
unsigned int sub_13ac0(unsigned int a0,unsigned char a1)
{
  return CONCAT31((undefined3)((unsigned int)a0 >> 8),(unsigned char)a0 >> (a1 & 7) | (unsigned char)a0 << 8 - (a1 & 7));
}

// Function: sub_13ad0 @ 0x13ad0
unsigned long sub_13ad0(int a0) // early-return
{
  unsigned long v1; // rax
  
  if (0x5a < a0)
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),(unsigned int)(a0 - 0x61U) <= 0x19);
  if (0x41 <= a0)
    return 1;
  return (unsigned long)((unsigned int)(a0 - 0x30U) <= 9);
}

// Function: sub_13b00 @ 0x13b00
bool sub_13b00(int a0)
{
  if (a0 <= 0x5a)
    return 0x40 < a0;
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_13b20 @ 0x13b20
bool sub_13b20(unsigned int a0)
{
  return a0 <= 0x7f;
}

// Function: sub_13b30 @ 0x13b30
unsigned int sub_13b30(int a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x20) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == 9);
}

// Function: sub_13b50 @ 0x13b50
unsigned long sub_13b50(unsigned int a0)
{
  unsigned long v1; // rax
  
  if ((int)a0 <= 0x1f)
    return (unsigned long)(~a0 >> 0x1f);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x7f);
}

// Function: sub_13b70 @ 0x13b70
bool sub_13b70(int a0)
{
  return (unsigned int)(a0 - 0x30U) <= 9;
}

// Function: sub_13b80 @ 0x13b80
bool sub_13b80(int a0)
{
  return (unsigned int)(a0 - 0x21U) <= 0x5d;
}

// Function: sub_13b90 @ 0x13b90
bool sub_13b90(int a0)
{
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_13ba0 @ 0x13ba0
bool sub_13ba0(int a0)
{
  return (unsigned int)(a0 - 0x20U) <= 0x5e;
}

// Function: sub_13bb0 @ 0x13bb0
unsigned long sub_13bb0(int a0)
{
  unsigned long v1; // rax
  undefined7 v2; // rax
  
  v2 = (undefined7)((unsigned long)v1 >> 8);
  if (0x60 < a0)
    return CONCAT71(v2,(unsigned int)(a0 - 0x7bU) <= 3);
  if (a0 <= 0x5a) {
    if (a0 <= 0x2f)
      return CONCAT71(v2,0x20 < a0);
    return CONCAT71(v2,(unsigned int)(a0 - 0x3aU) <= 6);
  }
  return 1;
}

// Function: sub_13c00 @ 0x13c00
bool sub_13c00(int a0)
{
  if (a0 <= 0xd)
    return 8 < a0;
  return a0 == 0x20;
}

// Function: sub_13c20 @ 0x13c20
bool sub_13c20(int a0)
{
  return (unsigned int)(a0 - 0x41U) <= 0x19;
}

// Function: sub_13c30 @ 0x13c30
unsigned int sub_13c30(int a0)
{
  if ((unsigned int)(a0 - 0x30U) <= 0x36)
    return (unsigned int)(0x7e0000007e03ff >> ((unsigned char)(a0 - 0x30U) & 0x3f)) & 1;
  return 0;
}

// Function: sub_13c60 @ 0x13c60
int sub_13c60(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_13c80 @ 0x13c80
int sub_13c80(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_13ca0 @ 0x13ca0
int sub_13ca0(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned int v2; // eax
  
  if (a0 == a1)
    return 0;
  do {
    v2 = sub_13c60(*a0);
    v1 = sub_13c60(*a1);
    if (!(unsigned char)v2) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while ((unsigned char)v2 == v1);
  return (v2 & 0xff) - (unsigned int)v1;
}

// Function: sub_13d00 @ 0x13d00
unsigned long sub_13d00(void *a0) // early-return x2, return-dupe
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // rax
  
  v2 = __fpending(a0);
  v1 = ferror_unlocked(a0);
  v3 = sub_c900(a0);
  if (v1) {
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

// Function: sub_13d70 @ 0x13d70
void sub_13d70(double a0)
{
  long v1; // rax
  long v2; // rdi
  double v3; // xmm0_qa
  
  if (a0 <= dat_17c18) {
    sub_148c0(0x8000000000000000,0); // tail-call
    return;
  }
  if (dat_17108 <= a0) {
    sub_148c0(0x7fffffffffffffff,999999999); // tail-call
    return;
  }
  v3 = (a0 - (double)(long)a0) * dat_17c20;
  v1 = (long)v3;
  v1 = (unsigned long)((double)v1 < v3) + v1;
  v2 = (long)a0 + v1 / 1000000000;
  v1 %= 1000000000;
  if (0 <= v1) {
    sub_148c0(v2); // tail-call
    return;
  }
  sub_148c0(v2 + -1,v1 + 1000000000); // tail-call
}

// Function: sub_13e40 @ 0x13e40
void sub_13e40(int a0,unsigned long a1)
{
  fcntl(a0,0,a1 & 0xffffffff); // tail-call
}

// Function: sub_13e50 @ 0x13e50
int sub_13e50(unsigned long a0,unsigned long a1)
{
  int v1;
  int v2; // eax
  unsigned int v3; // eax
  int *v4; // rax
  
  if (0 <= dat_1e9d8) { // branch-flip
    v2 = fcntl((int)a0,0x406,a1 & 0xffffffff);
    if ((0 <= v2) || (*__errno_location() != 0x16)) {
      dat_1e9d8 = 1;
      return v2;
    }
    v2 = sub_13e40(a0 & 0xffffffff,a1 & 0xffffffff);
    if (v2 < 0)
      return v2;
    dat_1e9d8 = -1;
  }
  else {
    v2 = sub_13e40(a0,a1);
    if (v2 < 0)
      return v2;
    if (dat_1e9d8 != -1)
      return v2;
  }
  v3 = fcntl(v2,1);
  if ((0 <= (int)v3) && (fcntl(v2,2,(unsigned long)(v3 | 1)) != -1))
    return v2;
  v4 = __errno_location();
  v1 = *v4;
  close(v2);
  *v4 = v1;
  return -1;
}

// Function: sub_13f30 @ 0x13f30
void sub_13f30(int a0,int a1,unsigned int a2)
{
  unsigned long v1;
  unsigned long v2; // stack - 0x28
  
  v2 = CONCAT44(v2._4_4_,a2);
  if (!a1) {
    sub_13e40(a0,a2);
    return;
  }
  if (a1 == 0x406) {
    sub_13e50(a0,a2);
    return;
  }
  if (0xc <= a1) { // branch-flip
    if (0xb <= (unsigned int)(a1 - 0x400U)) {
      fcntl(a0,a1,v2); // return-dupe
      return;
    }
    v1 = 1L << ((unsigned char)(a1 - 0x400U) & 0x3f);
    if (v1 & 0x2c5) {
      fcntl(a0,a1,(unsigned long)a2); // return-dupe, return-dupe
      return;
    }
    v1 &= 0x502;
  }
  else {
    if (a1 <= 0) {
      fcntl(a0,a1,v2);
      return;
    }
    v1 = 1L << ((unsigned char)a1 & 0x3f);
    if (v1 & 0x514) {
      fcntl(a0,a1,(unsigned long)a2);
      return;
    }
    v1 &= 0xa0a;
  }
  if (v1) {
    fcntl(a0,a1);
    return;
  }
  fcntl(a0,a1,v2);
  return;
}

// Function: sub_14080 @ 0x14080
unsigned int sub_14080(unsigned int a0)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  
  if (3 <= a0)
    return a0;
  v2 = sub_140e0();
  v3 = __errno_location();
  v1 = *v3;
  close(a0);
  *v3 = v1;
  return v2;
}

// Function: sub_140e0 @ 0x140e0
void sub_140e0(unsigned long a0,unsigned int a1)
{
  unsigned int v1;
  
  v1 = 0;
  if (a1 & 0x80000)
    v1 = 0x406;
  sub_13f30(a0,v1,3); // tail-call
}

// Function: sub_14100 @ 0x14100
void * sub_14100(char *a0,char *a1)
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
    v3 = sub_14960(v2);
    if (0 <= v3) { // branch-flip
      if ((!sub_c900(v4)) && (v5 = fdopen(v3,a1), v5))
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
      sub_c900(v4);
      *v6 = v3;
    }
  }
  return v5;
}

// Function: sub_141a0 @ 0x141a0
void sub_141a0(unsigned long a0)
{
  malloc(a0); // tail-call
}

// Function: sub_141b0 @ 0x141b0
void sub_141b0(void *a0,unsigned long a1)
{
  realloc(a0,a1 | a1 == 0); // tail-call
}

// Function: sub_141d0 @ 0x141d0
void sub_141d0(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1); // tail-call
}

// Function: sub_141e0 @ 0x141e0
void sub_141e0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((a1) && (a2)) {
    reallocarray(a0,a1,a2); // tail-call
    return;
  }
  reallocarray(a0,1,1); // tail-call
}

// Function: sub_14210 @ 0x14210
char * sub_14210(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_14250 @ 0x14250
unsigned long sub_14250(unsigned int *a0,unsigned char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,(char *)a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_cfb0(0))) {
    v1 = 1;
    *a0 = (unsigned int)*a1;
  }
  return v1;
}

// Function: sub_142e0 @ 0x142e0
int sub_142e0(char *a0,long a1,char *a2,long a3)
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

// Function: sub_14390 @ 0x14390
unsigned int sub_14390(void *a0,unsigned long a1,void *a2,unsigned long a3)
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
  v5 = sub_142e0(a0,a1 + 1,a2,a3 + 1);
  *v1 = v3;
  *v2 = v4;
  return v5;
}

// Function: sub_14440 @ 0x14440
unsigned long sub_14440(void *a0,unsigned long a1,void *a2,unsigned long a3)
{
  if (a1 == a3) {
    if (!memcmp(a0,a2,a1)) {
      *__errno_location() = 0;
      return 0;
    }
  }
  return sub_142e0(a0,a1,a2,a3); // tail-call
}

// Function: sub_144b0 @ 0x144b0
unsigned long sub_144b0(long *a0,long *a1) // early-return
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

// Function: sub_14580 @ 0x14580
void sub_14580(long a0,unsigned long a1)
{
  if (*(unsigned char *)(a0 + 0x50) & 1) {
    (**(void **)(a0 + 0x38))(*(unsigned long *)(a0 + 0x48)); // jump-as-call
    return;
  }
  (**(void **)(a0 + 0x38))(a1); // jump-as-call
}

// Function: sub_145a0 @ 0x145a0
void sub_145a0(long a0,unsigned long a1)
{
  if (*(unsigned char *)(a0 + 0x50) & 1) {
    (**(void **)(a0 + 0x40))(*(unsigned long *)(a0 + 0x48)); // jump-as-call
    return;
  }
  (**(void **)(a0 + 0x40))(a1); // jump-as-call
}

// Function: sub_145c0 @ 0x145c0
unsigned long sub_145c0(long *a0,long a1,long a2)
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
  v1 = (long *)sub_14580(a0);
  a0[1] = (long)v1;
  if (v1) {
    v2 = (long)v1 + v3 + 0x10 & -a2;
    a0[2] = v2;
    a0[3] = v2;
    v3 = *a0;
    *v1 = v3 + (long)v1;
    a0[4] = v3 + (long)v1;
    v1[1] = 0;
    *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] & 0xf9;
    return 1;
  }
  (*obstack_alloc_failed_handler)(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: sub_14650 @ 0x14650
void sub_14650(void)
{
  __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"memory exhausted",5));
  exit(dat_1e130); // no-return
}

// Function: _obstack_begin @ 0x146a0
void _obstack_begin(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) & 0xfe;
  *(unsigned long *)(a0 + 0x38) = a3;
  *(unsigned long *)(a0 + 0x40) = a4;
  sub_145c0(); // tail-call
}

// Function: _obstack_begin_1 @ 0x146c0
void _obstack_begin_1(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5)
{
  *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 1;
  *(unsigned long *)(a0 + 0x38) = a3;
  *(unsigned long *)(a0 + 0x40) = a4;
  *(unsigned long *)(a0 + 0x48) = a5;
  sub_145c0(); // tail-call
}

// Function: _obstack_newchunk @ 0x146e0
void _obstack_newchunk(unsigned long *a0,unsigned long a1)
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
    v4 = (unsigned long *)sub_14580(a0);
    if (v4) {
      a0[1] = (unsigned long)v4;
      v3 = (void *)a0[2];
      v4[1] = v2;
      a0[4] = (long)v4 + v5;
      *v4 = (long)v4 + v5;
      v7 = (void *)((long)v4 + a0[6] + 0x10 & ~a0[6]);
      memcpy(v7,v3,v6);
      if ((!(a0[10] & 2)) && (a0[2] == (~a0[6] & v2 + 0x10 + a0[6]))) {
        v4[1] = *(unsigned long *)(v2 + 8);
        sub_145a0(a0,v2);
      }
      a0[2] = (unsigned long)v7;
      a0[3] = (long)v7 + v6;
      *(unsigned char *)&a0[10] = (unsigned char)a0[10] & 0xfd;
      return;
    }
  }
  (*obstack_alloc_failed_handler)(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: _obstack_allocated_p @ 0x147e0
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

// Function: _obstack_free @ 0x14820
void _obstack_free(long a0,unsigned long *a1)
{
  unsigned long *v1;
  unsigned long *v2;
  
  v2 = *(unsigned long **)(a0 + 8);
  if (v2) {
    do {
      if ((v2 < a1) && (v1 = (unsigned long *)*v2, a1 <= v1)) {
        *(unsigned long **)(a0 + 0x18) = a1;
        *(unsigned long **)(a0 + 0x10) = a1;
        *(unsigned long **)(a0 + 0x20) = v1;
        *(unsigned long **)(a0 + 8) = v2;
        return;
      }
      v2 = (unsigned long *)v2[1];
      sub_145a0(a0);
      *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 2;
    } while (v2);
  }
  if (!a1)
    return;
  abort(); // no-return
}

// Function: _obstack_memory_used @ 0x14890
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

// Function: sub_148c0 @ 0x148c0
undefined16 sub_148c0(unsigned long a0,unsigned long a1)
{
  char v1 [16];
  
  v1._8_8_ = a1;
  v1._0_8_ = a0;
  return v1._0_16_;
}

// Function: sub_148d0 @ 0x148d0
int sub_148d0(long a0,long a1,long a2,long a3)
{
  return ((unsigned int)(a3 < a1) - (unsigned int)(a1 < a3)) + ((unsigned int)(a2 < a0) - (unsigned int)(a0 < a2)) * 2;
}

// Function: sub_14900 @ 0x14900
int sub_14900(long a0,long a1,unsigned long a2)
{
  return ((unsigned int)(a0 == 0) & (unsigned int)CONCAT71((undefined7)((unsigned long)a2 >> 8),a1 != 0)) + (unsigned int)(0 < a0) + (int)(a0 >> 0x3f);
}

// Function: sub_14930 @ 0x14930
double sub_14930(long a0,long a1)
{
  return (double)a1 / dat_17c20 + (double)a0;
}

// Function: sub_14960 @ 0x14960
void sub_14960(unsigned long a0)
{
  sub_13f30(a0,0,3); // tail-call
}

// Function: sub_14980 @ 0x14980
void sub_14980(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_1e008); // tail-call
}

// Function: _DT_FINI @ 0x14994
void _DT_FINI(void)
{
  return;
}
