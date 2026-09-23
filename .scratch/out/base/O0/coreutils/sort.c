// Function: _DT_INIT @ 0x3000
void _DT_INIT(void) // return-dupe
{
  if (!dat_22fc8)
    return;
  (*dat_22fc8)();
}

// Function: sub_3020 @ 0x3020
void sub_3020(void)
{
  (*dat_22c08)(); // jump-as-call
}

// Function: malloc @ 0x37a0
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22fd0)(); // jump-as-call
  return v1;
}

// Function: free @ 0x37b0
void free(void *a0)
{
  (*dat_22fe0)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x37c0
void __cxa_finalize(void)
{
  (*dat_22ff0)(); // jump-as-call
}

// Function: putchar_unlocked @ 0x37d0
int putchar_unlocked(int a0)
{
  int v1; // eax
  
  v1 = (*dat_22c10)(); // jump-as-call
  return v1;
}

// Function: euidaccess @ 0x37e0
void euidaccess(void)
{
  (*dat_22c18)(); // jump-as-call
}

// Function: pipe2 @ 0x37f0
void pipe2(void)
{
  (*dat_22c20)(); // jump-as-call
}

// Function: fileno @ 0x3800
int fileno(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_22c28)(); // jump-as-call
  return v1;
}

// Function: pthread_join @ 0x3810
void pthread_join(void)
{
  (*dat_22c30)(); // jump-as-call
}

// Function: strtoumax @ 0x3820
void strtoumax(void)
{
  (*dat_22c38)(); // jump-as-call
}

// Function: dup2 @ 0x3830
int dup2(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_22c40)(); // jump-as-call
  return v1;
}

// Function: pthread_cond_destroy @ 0x3840
void pthread_cond_destroy(void)
{
  (*dat_22c48)(); // jump-as-call
}

// Function: printf @ 0x3850
int printf(char *a0,...)
{
  int v1; // eax
  
  v1 = (*dat_22c50)(); // jump-as-call
  return v1;
}

// Function: pause @ 0x3860
void pause(void)
{
  (*dat_22c58)(); // jump-as-call
}

// Function: strcoll @ 0x3870
int strcoll(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_22c60)(); // jump-as-call
  return v1;
}

// Function: strtold @ 0x3880
void strtold(void)
{
  (*dat_22c68)(); // jump-as-call
}

// Function: fputc_unlocked @ 0x3890
void fputc_unlocked(void)
{
  (*dat_22c70)(); // jump-as-call
}

// Function: memset @ 0x38a0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22c78)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x38b0
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_22c80)(); // jump-as-call
  return v1;
}

// Function: pthread_sigmask @ 0x38c0
void pthread_sigmask(void)
{
  (*dat_22c88)(); // jump-as-call
}

// Function: close @ 0x38d0
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_22c90)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x38e0
void abort(void)
{
  (*dat_22c98)(); // jump-as-call
}

// Function: memchr @ 0x38f0
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22ca0)(); // jump-as-call
  return v1;
}

// Function: stpcpy @ 0x3900
char * stpcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_22ca8)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x3910
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_22cb0)(); // jump-as-call
  return v1;
}

// Function: pthread_cond_signal @ 0x3920
void pthread_cond_signal(void)
{
  (*dat_22cb8)(); // jump-as-call
}

// Function: textdomain @ 0x3930
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_22cc0)(); // jump-as-call
  return v1;
}

// Function: strxfrm @ 0x3940
void strxfrm(void)
{
  (*dat_22cc8)(); // jump-as-call
}

// Function: toupper @ 0x3950
int toupper(int a0)
{
  int v1; // eax
  
  v1 = (*dat_22cd0)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x3960
void exit(int a0)
{
  (*dat_22cd8)(); // jump-as-call
}

// Function: __assert_fail @ 0x3970
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_22ce0)(); // jump-as-call
}

// Function: fstat @ 0x3980
int fstat(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_22ce8)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x3990
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_22cf0)(); // jump-as-call
  return v1;
}

// Function: MD5 @ 0x39a0
void MD5(void)
{
  (*dat_22cf8)(); // jump-as-call
}

// Function: setvbuf @ 0x39b0
int setvbuf(void *a0,char *a1,int a2,unsigned long a3)
{
  int v1; // eax
  
  v1 = (*dat_22d00)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x39c0
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22d08)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x39d0
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_22d10)(); // jump-as-call
  return v1;
}

// Function: fopen @ 0x39e0
void * fopen(char *a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22d18)(); // jump-as-call
  return v1;
}

// Function: execlp @ 0x39f0
int execlp(char *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_22d20)(); // jump-as-call
  return v1;
}

// Function: gettext @ 0x3a00
char * gettext(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_22d28)(); // jump-as-call
  return v1;
}

// Function: unlink @ 0x3a10
int unlink(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_22d30)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x3a20
void _exit(int a0)
{
  (*dat_22d38)(); // jump-as-call
}

// Function: sigismember @ 0x3a30
void sigismember(void)
{
  (*dat_22d40)(); // jump-as-call
}

// Function: feof_unlocked @ 0x3a40
void feof_unlocked(void)
{
  (*dat_22d48)(); // jump-as-call
}

// Function: __cxa_atexit @ 0x3a50
void __cxa_atexit(void)
{
  (*dat_22d50)(); // jump-as-call
}

// Function: putc_unlocked @ 0x3a60
int putc_unlocked(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_22d58)(); // jump-as-call
  return v1;
}

// Function: sysconf @ 0x3a70
long sysconf(int a0)
{
  long v1; // rax
  
  v1 = (*dat_22d60)(); // jump-as-call
  return v1;
}

// Function: pthread_mutex_init @ 0x3a80
void pthread_mutex_init(void)
{
  (*dat_22d68)(); // jump-as-call
}

// Function: clearerr_unlocked @ 0x3a90
void clearerr_unlocked(void *a0)
{
  (*dat_22d70)(); // jump-as-call
}

// Function: pthread_create @ 0x3aa0
void pthread_create(void)
{
  (*dat_22d78)(); // jump-as-call
}

// Function: fread_unlocked @ 0x3ab0
void fread_unlocked(void)
{
  (*dat_22d80)(); // jump-as-call
}

// Function: strlen @ 0x3ac0
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_22d88)(); // jump-as-call
  return v1;
}

// Function: ferror @ 0x3ad0
int ferror(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_22d90)(); // jump-as-call
  return v1;
}

// Function: pthread_cond_init @ 0x3ae0
void pthread_cond_init(void)
{
  (*dat_22d98)(); // jump-as-call
}

// Function: __ctype_get_mb_cur_max @ 0x3af0
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_22da0)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x3b00
int fputs_unlocked(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_22da8)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x3b10
int __freading(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_22db0)(); // jump-as-call
  return v1;
}

// Function: MD5_Final @ 0x3b20
void MD5_Final(void)
{
  (*dat_22db8)(); // jump-as-call
}

// Function: __ctype_b_loc @ 0x3b30
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22dc0)(); // jump-as-call
  return v1;
}

// Function: fgetc @ 0x3b40
void fgetc(void)
{
  (*dat_22dc8)(); // jump-as-call
}

// Function: fdopen @ 0x3b50
void * fdopen(int a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22dd0)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x3b60
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_22dd8)(); // jump-as-call
  return v1;
}

// Function: getrandom @ 0x3b70
void getrandom(void)
{
  (*dat_22de0)(); // jump-as-call
}

// Function: pipe @ 0x3b80
int pipe(int *a0)
{
  int v1; // eax
  
  v1 = (*dat_22de8)(); // jump-as-call
  return v1;
}

// Function: ngettext @ 0x3b90
void ngettext(void)
{
  (*dat_22df0)(); // jump-as-call
}

// Function: fwrite_unlocked @ 0x3ba0
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_22df8)(); // jump-as-call
  return v1;
}

// Function: __fpending @ 0x3bb0
unsigned long __fpending(void *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_22e00)(); // jump-as-call
  return v1;
}

// Function: sigaction @ 0x3bc0
int sigaction(int a0,void *a1,void *a2)
{
  int v1; // eax
  
  v1 = (*dat_22e08)(); // jump-as-call
  return v1;
}

// Function: posix_fadvise @ 0x3bd0
void posix_fadvise(void)
{
  (*dat_22e10)(); // jump-as-call
}

// Function: fputs @ 0x3be0
int fputs(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_22e18)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x3bf0
void lseek(void)
{
  (*dat_22e20)(); // jump-as-call
}

// Function: strtol @ 0x3c00
long strtol(char *a0,char **a1,int a2)
{
  long v1; // rax
  
  v1 = (*dat_22e28)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x3c10
void iswcntrl(void)
{
  (*dat_22e30)(); // jump-as-call
}

// Function: sched_getaffinity @ 0x3c20
void sched_getaffinity(void)
{
  (*dat_22e38)(); // jump-as-call
}

// Function: mkostemp @ 0x3c30
void mkostemp(void)
{
  (*dat_22e40)(); // jump-as-call
}

// Function: explicit_bzero @ 0x3c40
void explicit_bzero(void *a0,unsigned long a1)
{
  (*dat_22e48)(); // jump-as-call
}

// Function: raise @ 0x3c50
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_22e50)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x3c60
void signal(void)
{
  (*dat_22e58)(); // jump-as-call
}

// Function: memmove @ 0x3c70
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22e60)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x3c80
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_22e68)(); // jump-as-call
  return v1;
}

// Function: waitpid @ 0x3c90
int waitpid(int a0,int *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_22e70)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x3ca0
int ferror_unlocked(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_22e78)(); // jump-as-call
  return v1;
}

// Function: getopt_long @ 0x3cb0
int getopt_long(int a0,char **a1,char *a2,void *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_22e80)(); // jump-as-call
  return v1;
}

// Function: getenv @ 0x3cc0
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_22e88)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x3cd0
int mbsinit(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_22e90)(); // jump-as-call
  return v1;
}

// Function: __errno_location @ 0x3ce0
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_22e98)(); // jump-as-call
  return v1;
}

// Function: qsort @ 0x3cf0
void qsort(void)
{
  (*dat_22ea0)(); // jump-as-call
}

// Function: mkstemp @ 0x3d00
void mkstemp(void)
{
  (*dat_22ea8)(); // jump-as-call
}

// Function: sigaddset @ 0x3d10
int sigaddset(void *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_22eb0)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x3d20
void __stack_chk_fail(void)
{
  (*dat_22eb8)(); // jump-as-call
}

// Function: strcmp @ 0x3d30
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_22ec0)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x3d40
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_22ec8)(); // jump-as-call
  return v1;
}

// Function: MD5_Init @ 0x3d50
void MD5_Init(void)
{
  (*dat_22ed0)(); // jump-as-call
}

// Function: localeconv @ 0x3d60
void localeconv(void)
{
  (*dat_22ed8)(); // jump-as-call
}

// Function: nanosleep @ 0x3d70
void nanosleep(void)
{
  (*dat_22ee0)(); // jump-as-call
}

// Function: pthread_cond_wait @ 0x3d80
void pthread_cond_wait(void)
{
  (*dat_22ee8)(); // jump-as-call
}

// Function: MD5_Update @ 0x3d90
void MD5_Update(void)
{
  (*dat_22ef0)(); // jump-as-call
}

// Function: memcmp @ 0x3da0
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_22ef8)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x3db0
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22f00)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x3dc0
int fclose(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_22f08)(); // jump-as-call
  return v1;
}

// Function: getrlimit @ 0x3dd0
void getrlimit(void)
{
  (*dat_22f10)(); // jump-as-call
}

// Function: __sched_cpucount @ 0x3de0
void __sched_cpucount(void)
{
  (*dat_22f18)(); // jump-as-call
}

// Function: fseeko @ 0x3df0
void fseeko(void)
{
  (*dat_22f20)(); // jump-as-call
}

// Function: ftruncate @ 0x3e00
void ftruncate(void)
{
  (*dat_22f28)(); // jump-as-call
}

// Function: fork @ 0x3e10
int fork(void)
{
  int v1; // eax
  
  v1 = (*dat_22f30)(); // jump-as-call
  return v1;
}

// Function: sigemptyset @ 0x3e20
int sigemptyset(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_22f38)(); // jump-as-call
  return v1;
}

// Function: pthread_mutex_lock @ 0x3e30
int pthread_mutex_lock(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_22f40)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x3e40
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22f48)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x3e50
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_22f50)(); // jump-as-call
  return v1;
}

// Function: error @ 0x3e60
void error(int a0,int a1,char *a2,...)
{
  (*dat_22f58)(); // jump-as-call
}

// Function: fprintf @ 0x3e70
int fprintf(void *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_22f60)(); // jump-as-call
  return v1;
}

// Function: pthread_mutex_destroy @ 0x3e80
void pthread_mutex_destroy(void)
{
  (*dat_22f68)(); // jump-as-call
}

// Function: write @ 0x3e90
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_22f70)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x3ea0
int stat(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_22f78)(); // jump-as-call
  return v1;
}

// Function: strtoul @ 0x3eb0
unsigned long strtoul(char *a0,char **a1,int a2)
{
  unsigned long v1; // rax
  
  v1 = (*dat_22f80)(); // jump-as-call
  return v1;
}

// Function: sysinfo @ 0x3ec0
void sysinfo(void)
{
  (*dat_22f88)(); // jump-as-call
}

// Function: pthread_mutex_unlock @ 0x3ed0
int pthread_mutex_unlock(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_22f90)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x3ee0
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_22f98)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x3ef0
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_22fa0)(); // jump-as-call
  return v1;
}

// Function: fflush_unlocked @ 0x3f00
int fflush_unlocked(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_22fa8)(); // jump-as-call
  return v1;
}

// Function: open @ 0x3f10
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_22fb0)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x3f20
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_22fb8)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x3f30
int fflush(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_22fc0)(); // jump-as-call
  return v1;
}

// Function: sub_3f40 @ 0x3f40
void sub_3f40(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_22ff8)(sub_da8d,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_3f70 @ 0x3f70
void sub_3f70(void)
{
  return;
}

// Function: _FINI_0 @ 0x3fe0
void _FINI_0(void)
{
  if (!dat_23210) {
    if (dat_22ff0)
      __cxa_finalize(dat_23008);
    sub_3f70();
    dat_23210 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x4020
void _INIT_0(void)
{
  return;
}

// Function: sub_4029 @ 0x4029
void sub_4029(int a0) // return-dupe
{
  if (a0 == 1)
    return;
  dat_23118 = a0;
}

// Function: sub_4042 @ 0x4042
char sub_4042(char a0)
{
  return a0;
}

// Function: sub_4051 @ 0x4051
unsigned int sub_4051(unsigned char a0) // return-dupe
{
  if ((!(*(unsigned short *)((unsigned long)a0 * 2 + *(long *)__ctype_b_loc()) & 1)) && (a0 != 10))
    return 0;
  return 1;
}

// Function: sub_4094 @ 0x4094
unsigned long sub_4094(unsigned long a0)
{
  return a0;
}

// Function: sub_40a2 @ 0x40a2
void sub_40a2(void)
{
  void *v1;
  
  v1 = stdout;
  fputs_unlocked(gettext("\nWith no FILE, or when FILE is -, read standard input.\n"),v1);
}

// Function: sub_40d3 @ 0x40d3
void sub_40d3(void)
{
  void *v1;
  
  v1 = stdout;
  fputs_unlocked(gettext("\nMandatory arguments to long options are mandatory for short options too.\n"),v1);
}

// Function: sub_4104 @ 0x4104
void sub_4104(char *a0) // ternary
{
  void *v1;
  char *v10; // stack - 0x78
  char *v11; // stack - 0x70
  char *v12; // stack - 0x68
  char *v13; // stack - 0x60
  char *v14; // stack - 0x58
  char *v15; // stack - 0x50
  char *v16; // stack - 0x48
  char *v17; // stack - 0x40
  unsigned long v18; // stack - 0x38
  unsigned long v19; // stack - 0x30
  int v2; // eax
  char *v3;
  char *v4; // stack - 0x98
  char *v5; // stack - 0xb8
  char **v6; // stack - 0xb0
  char *v7; // stack - 0x90
  char *v8; // stack - 0x88
  char *v9; // stack - 0x80
  
  v4 = "[";
  v7 = "test invocation";
  v8 = "coreutils";
  v9 = "Multi-call invocation";
  v10 = "sha224sum";
  v11 = "sha2 utilities";
  v12 = "sha256sum";
  v13 = "sha2 utilities";
  v14 = "sha384sum";
  v15 = "sha2 utilities";
  v16 = "sha512sum";
  v17 = "sha2 utilities";
  v18 = 0;
  v19 = 0;
  v6 = &v4;
  while ((*v6 && (v3 = *v6, strcmp(a0,v3)))) {
    v6 = &v6[2];
  }
  v5 = a0;
  if (v6[1])
    v5 = v6[1];
  printf(gettext("\n%s online help: <%s>\n"),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  v3 = setlocale(5,NULL);
  if ((v3) && (v2 = strncmp(v3,"en_",3), v1 = stdout, v2))
    fputs_unlocked(gettext("Report any translation bugs to <https://translationproject.org/team/>\n"),v1);
  if (strcmp(a0,"[")) // branch-flip
    v3 = a0;
  else {
    v3 = "test";
  }
  printf(gettext("Full documentation <%s%s>\n"),"https://www.gnu.org/software/coreutils/",v3);
  v3 = (v5 != a0) ? "" : " invocation"; // branch-flip
  printf(gettext("or available locally via: info \'(coreutils) %s%s\'\n"),v5,v3);
}

// Function: sub_439b @ 0x439b
void sub_439b(int a0,char *a1)
{
  char *v1; // rax
  char v2 [12];
  long v3; // fs_offset
  unsigned long v4; // stack - 0x10
  
  v4 = *(unsigned long *)(v3 + 0x28);
  write(2,a1,strlen(a1));
  if (a0) {
    v1 = (char *)sub_126fe(a0,v2);
    write(2,": errno ",8);
    write(2,v1,strlen(v1));
  }
  write(2,"\n",1);
  _exit(2); // no-return
}

// Function: sub_4464 @ 0x4464
void sub_4464(unsigned long a0,char *a1)
{
  unsigned long v1; // rax
  
  if (!a1)
    a1 = gettext("standard output");
  v1 = sub_151eb(0,3,a1);
  error(2,*__errno_location(),"%s: %s",a0,v1); // no-return
}

// Function: sub_44d3 @ 0x44d3
void sub_44d3(int a0)
{
  void *v1;
  unsigned long v2;
  
  v2 = dat_23868;
  if (a0) // branch-flip
    fprintf(stderr,gettext("Try \'%s --help\' for more information.\n"),v2);
  else {
    printf(gettext("Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n"),v2,v2);
    v1 = stdout;
    fputs_unlocked(gettext("Write sorted concatenation of all FILE(s) to standard output.\n"),v1);
    sub_40a2();
    sub_40d3();
    v1 = stdout;
    fputs_unlocked(gettext("Ordering options:\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -b, --ignore-leading-blanks  ignore leading blanks\n  -d, --dictionary-order      consider only blanks and alphanumeric characters\n  -f, --ignore-case           fold lower case to upper case characters\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -g, --general-numeric-sort  compare according to general numerical value\n  -i, --ignore-nonprinting    consider only printable characters\n  -M, --month-sort            compare (unknown) < \'JAN\' < ... < \'DEC\'\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -h, --human-numeric-sort    compare human readable numbers (e.g., 2K 1G)\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -n, --numeric-sort          compare according to string numerical value\n  -R, --random-sort           shuffle, but group identical keys.  See shuf(1)\n      --random-source=FILE    get random bytes from FILE\n  -r, --reverse               reverse the result of comparisons\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --sort=WORD             sort according to WORD:\n                                general-numeric -g, human-numeric -h, month -M,\n                                numeric -n, random -R, version -V\n  -V, --version-sort          natural sort of (version) numbers within text\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("Other options:\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --batch-size=NMERGE   merge at most NMERGE inputs at once;\n                            for more use temp files\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -c, --check, --check=diagnose-first  check for sorted input; do not sort\n  -C, --check=quiet, --check=silent  like -c, but do not report first bad line\n      --compress-program=PROG  compress temporaries with PROG;\n                              decompress them with PROG -d\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --debug               annotate the part of the line used to sort,\n                              and warn about questionable usage to stderr\n      --files0-from=F       read input from the files specified by\n                            NUL-terminated names in file F;\n                            If F is - then read names from standard input\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -k, --key=KEYDEF          sort via a key; KEYDEF gives location and type\n  -m, --merge               merge already sorted files; do not sort\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -o, --output=FILE         write result to FILE instead of standard output\n  -s, --stable              stabilize sort by disabling last-resort comparison\n  -S, --buffer-size=SIZE    use SIZE for main memory buffer\n"),v1);
    printf(gettext("  -t, --field-separator=SEP  use SEP instead of non-blank to blank transition\n  -T, --temporary-directory=DIR  use DIR for temporaries, not $TMPDIR or %s;\n                              multiple options specify multiple directories\n      --parallel=N          change the number of sorts run concurrently to N\n  -u, --unique              with -c, check for strict ordering;\n                              without -c, output only the first of an equal run\n"),"/tmp");
    v1 = stdout;
    fputs_unlocked(gettext("  -z, --zero-terminated     line delimiter is NUL, not newline\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --help        display this help and exit\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --version     output version information and exit\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("\nKEYDEF is F[.C][OPTS][,F[.C][OPTS]] for start and stop position, where F is a\nfield number and C a character position in the field; both are origin 1, and\nthe stop position defaults to the line\'s end.  If neither -t nor -b is in\neffect, characters in a field are counted from the beginning of the preceding\nwhitespace.  OPTS is one or more single-letter ordering options [bdfgiMhnRrV],\nwhich override global ordering options for that key.  If no key is given, use\nthe entire line as the key.  Use --debug to diagnose incorrect key usage.\n\nSIZE may be followed by the following multiplicative suffixes:\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("% 1% of memory, b 1, K 1024 (default), and so on for M, G, T, P, E, Z, Y.\n\n*** WARNING ***\nThe locale specified by the environment affects sort order.\nSet LC_ALL=C to get the traditional sort order that uses\nnative byte values.\n"),v1);
    sub_4104("sort");
  }
  exit(a0); // no-return
}

// Function: sub_47f1 @ 0x47f1
void sub_47f1(long a0)
{
  *(bool *)a0 = pthread_sigmask(0,0x236a0,a0 + 8) == 0;
}

// Function: sub_4833 @ 0x4833
void sub_4833(char *a0) // return-dupe
{
  if (!*a0)
    return;
  pthread_sigmask(2,&a0[8],0);
}

// Function: sub_486b @ 0x486b
unsigned long sub_486b(long a0,unsigned long a1)
{
  return (unsigned long)(long)*(int *)(a0 + 8) % a1;
}

// Function: sub_489a @ 0x489a
unsigned int sub_489a(long a0,long a1)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)(a1 + 8) >> 8),*(int *)(a0 + 8) == *(int *)(a1 + 8));
}

// Function: sub_48cf @ 0x48cf
int sub_48cf(int a0)
{
  char v1; // al
  int v2;
  unsigned long v3; // rax
  char *v4; // rax
  unsigned int v5; // stack - 0x28
  int v6; // stack - 0x24
  
  if (a0) // branch-flip
    v2 = a0;
  else {
    v2 = -1;
  }
  v6 = waitpid(v2,(int *)&v5,(unsigned int)(a0 == 0));
  if (0 <= v6) {
    if ((1 <= v6) && ((1 <= a0 || (v1 = sub_4a90(v6), v1)))) {
      if ((v5 & 0x7f) || ((int)v5 >> 8 & 0xffU)) {
        v3 = sub_15059(4,dat_23690);
        error(2,0,gettext("%s [-d] terminated abnormally"),v3); // no-return
      }
      dat_23730 -= 1;
    }
    return v6;
  }
  v3 = sub_15059(4,dat_23690);
  v4 = gettext("waiting for %s [-d]");
  error(2,*__errno_location(),v4,v3); // no-return
}

// Function: sub_4a08 @ 0x4a08
void sub_4a08(long a0)
{
  long v1; // rax
  
  if (!dat_23728) {
    dat_23728 = sub_1120d(0x2f,0,sub_486b,sub_489a,0);
    if (!dat_23728)
      sub_1865c(); // no-return
  }
  *(char *)(a0 + 0xc) = 1;
  v1 = sub_11f4e(dat_23728,a0);
  if (!v1)
    sub_1865c(); // no-return
}

// Function: sub_4a90 @ 0x4a90
bool sub_4a90(unsigned int a0)
{
  long v1; // rax
  char v2 [8];
  unsigned int v3; // stack - 0x20
  
  v3 = a0;
  v1 = sub_11fbe(dat_23728,v2);
  if (v1)
    *(char *)(v1 + 0xc) = 2;
  return v1 != 0;
}

// Function: sub_4aff @ 0x4aff
void sub_4aff(unsigned int a0) // return-dupe
{
  if (!sub_4a90(a0))
    return;
  sub_48cf(a0);
}

// Function: sub_4b29 @ 0x4b29
void sub_4b29(void)
{
  do {
    if (dat_23730 <= 0)
      return;
  } while (sub_48cf(0));
}

// Function: sub_4b4f @ 0x4b4f
void sub_4b4f(void)
{
  sub_48cf(0xffffffff);
  sub_4b29();
}

// Function: sub_4b69 @ 0x4b69
void sub_4b69(void)
{
  while (0 < dat_23730) {
    sub_48cf(0xffffffff);
  }
}

// Function: sub_4b8b @ 0x4b8b
void sub_4b8b(void)
{
  unsigned long *v1; // stack - 0x10
  
  for (v1 = dat_23720; v1; v1 = (unsigned long *)*v1) {
    unlink((char *)((long)v1 + 0xd));
  }
  dat_23720 = NULL;
}

// Function: sub_4bd4 @ 0x4bd4
void sub_4bd4(void)
{
  char v1 [136];
  
  if (dat_23720) {
    sub_47f1(v1);
    sub_4b8b();
    sub_4833(v1);
  }
  sub_fa34();
}

// Function: sub_4c3d @ 0x4c3d
unsigned long * sub_4c3d(int *a0,char a1)
{
  int v1;
  int v2; // eax
  unsigned long v3; // rax
  void *v4; // rax
  unsigned long v5; // rax
  char *v6;
  char v7 [136];
  unsigned long *v8; // stack - 0xc8
  
  v6 = *(char **)(dat_237a0 * 8 + dat_23668);
  v3 = strlen(v6);
  v8 = (unsigned long *)sub_18024(v3 + 0x20 & 0xfffffffffffffff8);
  v4 = (void *)((long)v8 + 0xd);
  memcpy(v4,v6,v3);
  memcpy((void *)(v3 + (long)v4),"/sortXXXXXX",0xc);
  *v8 = 0;
  dat_237a0 += 1;
  if (dat_237a0 == dat_23670)
    dat_237a0 = 0;
  sub_47f1(v7);
  v2 = sub_1704d(v4,0x80000);
  if (0 <= v2) {
    *dat_23100 = v8;
    dat_23100 = v8;
  }
  v1 = *__errno_location();
  sub_4833(v7);
  *__errno_location() = v1;
  if (v2 <= -1) {
    if ((a1 != '\x01') || (*__errno_location() != 0x18)) {
      v5 = sub_15059(4,v6);
      v6 = gettext("cannot create temporary file in %s");
      error(2,*__errno_location(),v6,v5); // no-return
    }
    free(v8);
    v8 = NULL;
  }
  *a0 = v2;
  return v8;
}

// Function: sub_4e8c @ 0x4e8c
unsigned long sub_4e8c(void) // return-dupe
{
  if (!dat_237a8) {
    if (fstat(1,(void *)0x237c0)) // branch-flip
      dat_237a8 = *__errno_location();
    else {
      dat_237a8 = -1;
    }
  }
  if (dat_237a8 > -1)
    return 0;
  return 0x237c0;
}

// Function: sub_4ee4 @ 0x4ee4
int * sub_4ee4(char *a0,char *a1)
{
  int v1;
  unsigned long v2;
  unsigned int v3; // eax
  int *v4; // rax
  void *v5; // stack - 0x28
  
  if (*a1 != 'r') { // branch-flip
    if (*a1 != 'w')
      __assert_fail("!\"unexpected mode passed to stream_open\"","src/sort.c",0x3d5,"stream_open"); // no-return
    if (a0) {
      v3 = ftruncate(1,0);
      if (v3) {
        v4 = __errno_location();
        v1 = *v4;
        v4 = (long)sub_4e8c();
        if ((!v4) || (v3 = *(unsigned int *)((long)v4 + 0x18) & 0xf000, v3 == 0x8000)) {
          v4 = (unsigned long)sub_151eb(0,3,a0);
          v2 = v4;
          v4 = (char *)gettext("%s: error truncating");
          error(2,v1,(char *)v4,v2); // no-return
        }
      }
    }
    v5 = stdout;
  }
  else {
    v3 = strcmp(a0,"-");
    if (v3) { // branch-flip
      v3 = open(a0,0x80000);
      if (0 <= (int)v3) // branch-flip
        v4 = fdopen(v3,a1);
      else {
        v4 = NULL;
      }
      v5 = v4;
    }
    else {
      dat_23683 = 1;
      v5 = stdin;
    }
    sub_fed8(v5,2);
  }
  v4 = v5;
  return v4;
}

// Function: sub_505e @ 0x505e
long sub_505e(unsigned long a0,unsigned long a1)
{
  long v1; // rax
  
  v1 = sub_4ee4(a0,a1);
  if (!v1)
    sub_4464(gettext("open failed"),a0); // no-return
  return v1;
}

// Function: sub_50b7 @ 0x50b7
void sub_50b7(void *a0,unsigned long a1) // return-dupe x2
{
  int v1; // eax
  
  v1 = fileno(a0);
  if (!v1) {
    clearerr_unlocked(a0);
    return;
  }
  if (v1 != 1) {
    if (!sub_ff17(a0))
      return;
    sub_4464(gettext("close failed"),a1); // no-return
  }
  if (!fflush_unlocked(a0))
    return;
  sub_4464(gettext("fflush failed"),a1); // no-return
}

// Function: sub_515d @ 0x515d
void sub_515d(int a0,int a1) // return-dupe
{
  if (a0 == a1)
    return;
  dup2(a0,a1);
  close(a0);
}

// Function: sub_5193 @ 0x5193
int sub_5193(int *a0,long a1) // early-return
{
  long v1;
  int v2;
  unsigned long v3;
  char v4 [136];
  long v5; // stack - 0xc8
  int v6; // stack - 0xb0
  
  if ((int)sub_1311d(a0,0x80000) <= -1)
    return -1;
  v5 = a1;
  if (dat_23024 + 1U < dat_23730) {
    sub_4b4f();
    v5 = a1;
  }
  while (v1 = v5 + -1, v5) {
    sub_47f1(v4);
    v3 = dat_23720;
    dat_23720 = 0;
    v6 = fork();
    v2 = *__errno_location();
    if (v6)
      dat_23720 = v3;
    sub_4833(v4);
    *__errno_location() = v2;
    if ((0 <= v6) || (*__errno_location() != 0xb)) break;
    sub_18863();
    sub_4b29();
    v5 = v1;
  }
  if (0 <= v6) { // branch-flip
    if (v6) // branch-flip
      dat_23730 += 1;
    else {
      close(0);
      close(1);
    }
  }
  else {
    v2 = *__errno_location();
    close(*a0);
    close(a0[1]);
    *__errno_location() = v2;
  }
  return v6;
}

// Function: sub_5372 @ 0x5372
long sub_5372(long *a0,char a1) // early-return
{
  long v1;
  unsigned int v2; // eax
  void *v3; // rax
  int v4; // stack - 0x34
  int v5; // stack - 0x28
  long v6; // stack - 0x30
  int v7; // stack - 0x24
  
  v6 = sub_4c3d(&v4,a1);
  if (!v6)
    return 0;
  *(char *)(v6 + 0xc) = 0;
  if (dat_23690) {
    v2 = sub_5193(&v5,4);
    *(unsigned int *)(v6 + 8) = v2;
    if (1 <= *(int *)(v6 + 8)) { // branch-flip
      close(v4);
      close(v5);
      v4 = v7;
      sub_4a08(v6);
    }
    else if (!*(int *)(v6 + 8)) {
      close(v7);
      sub_515d(v4,1);
      sub_515d(v5,0);
      execlp(dat_23690,dat_23690,0);
      sub_439b(*__errno_location(),"couldn\'t execute compress program"); // no-return
    }
  }
  v3 = fdopen(v4,"w");
  *a0 = (long)v3;
  if (*a0)
    return v6;
  v1 = v6 + 0xd;
  sub_4464(gettext("couldn\'t create temporary file"),v1); // no-return
}

// Function: sub_54f9 @ 0x54f9
void sub_54f9(unsigned long a0)
{
  sub_5372(a0,0);
}

// Function: sub_551c @ 0x551c
void * sub_551c(long a0) // early-return
{
  int v1;
  int v2; // eax
  unsigned long v3; // rax
  char *v4; // rax
  int v5; // stack - 0x28
  void *v6; // stack - 0x30
  int v7; // stack - 0x24
  
  v6 = NULL;
  if (*(char *)(a0 + 0xc) == '\x01')
    sub_4aff(*(unsigned int *)(a0 + 8));
  v1 = open((char *)(a0 + 0xd),0);
  if (v1 <= -1)
    return NULL;
  v2 = sub_5193(&v5,9);
  if (v2 != -1) { // branch-flip
    if (!v2) {
      close(v5);
      sub_515d(v1,0);
      sub_515d(v7,1);
      execlp(dat_23690,dat_23690,"-d",0);
      sub_439b(*__errno_location(),"couldn\'t execute compress program (with -d)"); // no-return
    }
    *(int *)(a0 + 8) = v2;
    sub_4a08(a0);
    close(v1);
    close(v7);
    v6 = fdopen(v5,"r");
    if (!v6) {
      v1 = *__errno_location();
      close(v5);
      *__errno_location() = v1;
    }
  }
  else {
    if (*__errno_location() != 0x18) {
      v3 = sub_15059(4,dat_23690);
      v4 = gettext("couldn\'t create process for %s -d");
      error(2,*__errno_location(),v4,v3); // no-return
    }
    close(v1);
    *__errno_location() = 0x18;
  }
  return v6;
}

// Function: sub_570f @ 0x570f
void sub_570f(unsigned long a0)
{
  long v1;
  
  if (dat_23670 == dat_23678)
    dat_23668 = sub_18221(dat_23668,0x23678,8);
  v1 = dat_23670;
  dat_23670 += 1;
  *(unsigned long *)(dat_23668 + v1 * 8) = a0;
}

// Function: sub_5782 @ 0x5782
void sub_5782(char *a0)
{
  int v1;
  long *v2;
  long v3;
  int v4; // eax
  unsigned long v5; // rax
  char v6 [136];
  long *v7; // stack - 0xc0
  
  v7 = (long *)0x23720;
  while (v2 = (long *)*v7, a0 != (char *)((long)v2 + 0xd)) {
    v7 = v2;
  }
  if (*(char *)((long)v2 + 0xc) == '\x01')
    sub_4aff((int)v2[1]);
  v3 = *v2;
  sub_47f1(v6);
  v4 = unlink(a0);
  v1 = *__errno_location();
  *v7 = v3;
  sub_4833(v6);
  if (!v4) {
    if (!v3)
      dat_23100 = v7;
    free(v2);
    return;
  }
  v5 = sub_151eb(0,3,a0);
  error(0,v1,gettext("warning: cannot remove: %s"),v5); // no-return
}

// Function: sub_590b @ 0x590b
void sub_590b(unsigned long *a0,unsigned long *a1)
{
  strcmp((char *)*a0,(char *)*a1);
}

// Function: sub_594a @ 0x594a
void sub_594a(void) // return-dupe, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_594a
{
  char v1;
  long v10; // stack - 0x38
  long v2;
  char v3; // al
  int v4; // eax
  char *v5; // rax
  unsigned long v6; // rax
  long v7; // rax
  unsigned long v8; // stack - 0x48
  unsigned long v9; // stack - 0x40
  
  for (v8 = 0; v8 <= 0xff; v8 = v8 + 1) {
    *(char *)(v8 + 0x23260) = sub_4051(v8 & 0xff);
    v4 = (int)v8;
    *(bool *)(v8 + 0x23360) = (*(unsigned short *)((long)v4 * 2 + *(long *)__ctype_b_loc()) & 0x4000) == 0;
    v3 = !((*(unsigned short *)((long)v4 * 2 + *(long *)__ctype_b_loc()) & 8) || (sub_4051(v8 & 0xff) == '\x01'));
    *(char *)(v8 + 0x23460) = v3;
    *(char *)(v8 + 0x23560) = (char)toupper(v4);
  }
  if (!dat_2322a)
    return;
  for (v8 = 0; v8 <= 0xb; v8 = v8 + 1) {
    v5 = nl_langinfo((int)v8 + 0x2000e);
    v6 = strlen(v5);
    v7 = sub_18024(v6 + 1);
    *(long *)(v8 * 0x10 + 0x23040) = v7;
    *(int *)(v8 * 0x10 + 0x23048) = (int)v8 + 1;
    v10 = 0;
    for (v9 = 0; v9 < v6; v9 = v9 + 1) {
      v2 = *(long *)__ctype_b_loc();
      v1 = v5[v9];
      if (!(*(unsigned short *)((unsigned long)(unsigned char)sub_4042((int)v1) * 2 + v2) & 1)) {
        v1 = v5[v9];
        v2 = v10 + 1;
        *(char *)(v7 + v10) = *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v1) + 0x23560);
        v10 = v2;
      }
    }
    *(char *)(v10 + v7) = 0;
  }
  qsort(0x23040,0xc,0x10,sub_590b);
}

// Function: sub_5ba8 @ 0x5ba8
void sub_5ba8(int a0,char a1,unsigned long a2) // ternary
{
  unsigned long v1;
  unsigned int v2; // eax
  unsigned long v3; // rax
  unsigned long v4; // stack - 0x50
  int v5 [10]; // stack - 0x48
  int v6; // stack - 0x58
  
  v6 = sub_18b54(a2,0,10,&v4,0x1b1db);
  v2 = (getrlimit(7,v5)) ? 0x11 : v5[0] - 3; // branch-flip
  if (!v6) {
    dat_23024 = (unsigned int)v4;
    if ((v4 & 0xffffffff) != v4) // branch-flip
      v6 = 1;
    else {
      if (dat_23024 <= 1) {
        v3 = sub_154a0(a2);
        v1 = *(unsigned long *)((long)a0 * 0x20 + 0x22560);
        error(0,0,gettext("invalid --%s argument %s"),v1,v3); // no-return
      }
      if (dat_23024 <= v2)
        return;
      v6 = 1;
    }
  }
  if (v6 != 1)
    sub_18a33(v6,a0,(int)a1,0x22560,a2); // no-return
  v3 = sub_154a0(a2);
  v1 = *(unsigned long *)((long)a0 * 0x20 + 0x22560);
  error(0,0,gettext("--%s argument %s too large"),v1,v3); // no-return
}

// Function: sub_5e0c @ 0x5e0c
void sub_5e0c(unsigned int a0,char a1,unsigned long a2) // return-dupe, ternary
{
  unsigned long v1; // stack - 0x28
  char *v2; // stack - 0x20
  double v3; // xmm0_qa
  int v4; // stack - 0x2c
  double v5; // stack - 0x18
  
  v4 = sub_18b54(a2,&v2,10,&v1,"EgGkKmMPtTYZ");
  if ((!v4) && ((unsigned int)((int)v2[-1] - 0x30U) <= 9)) {
    if (0x40000000000000 <= v1) // branch-flip
      v4 = 1;
    else {
      v1 <<= 10;
    }
  }
  if (((v4 == 2) && ((unsigned int)((int)v2[-1] - 0x30U) <= 9)) && (!v2[1])) {
    if (*v2 != '%') { // branch-flip
      if (*v2 == 'b')
        v4 = 0;
    }
    else {
      sub_12e84();
      v5 = ((double)v1 * v3) / dat_1d140;
      if (dat_1d148 <= v5)
        v4 = 1;
      else {
        v1 = (dat_1d150 <= v5) ? (long)(v5 - dat_1d150) ^ 0x8000000000000000 : (unsigned long)v5;
        v4 = 0;
      }
    }
  }
  if (v4)
    sub_18a33(v4,a0,(int)a1,0x22560,a2); // no-return
  if (v1 < dat_23660)
    return;
  if ((unsigned long)dat_23024 * 0x22 < v1) {
    dat_23660 = v1;
    return;
  }
  dat_23660 = (unsigned long)dat_23024 * 0x22;
}

// Function: sub_6042 @ 0x6042
long sub_6042(unsigned int a0,char a1,unsigned long a2) // early-return
{
  int v1; // eax
  long v2; // stack - 0x18
  
  v1 = sub_18b54(a2,0,10,&v2,0x1b1db);
  if (v1 == 1)
    return -1;
  if (!v1) {
    if (v2)
      return v2;
    error(2,0,gettext("number in parallel must be nonzero")); // no-return
  }
  sub_18a33(v1,a0,(int)a1,0x22560,a2); // no-return
}

// Function: sub_610d @ 0x610d
unsigned long sub_610d(void)
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3 [3]; // stack - 0x28
  double v4; // xmm0_qa
  double v5; // xmm0_qa
  double v6;
  double v7; // xmm0_qa
  unsigned long v8; // stack - 0x48
  
  v8 = 0xffffffffffffffff;
  v1 = getrlimit(2,v3);
  if ((!v1) && (v3[0] != 0xffffffffffffffff))
    v8 = v3[0];
  v1 = getrlimit(9,v3);
  if ((!v1) && (v3[0] < v8))
    v8 = v3[0];
  v2 = v8 >> 1;
  v8 = v2;
  v1 = getrlimit(5,v3);
  if ((!v1) && (v2 = v3[0] >> 4, v2 * 0xf < v8)) {
    v2 = v3[0] >> 4;
    v8 = v2 * 0xf;
  }
  sub_12fab();
  sub_12e84();
  if (v4 <= v5 / dat_1d158)
    v6 = v5 / dat_1d158;
  else {
    v6 = v4;
  }
  if (v5 * dat_1d160 < (double)v8) {
    v7 = dat_1d160 * v5;
    if (dat_1d150 <= v7) {
      v2 = (unsigned long)(v7 - dat_1d150);
      v8 = v2 ^ 0x8000000000000000;
    }
    else {
      v2 = (unsigned long)v7;
      v8 = v2;
    }
  }
  if (v6 < (double)v8) {
    if (dat_1d150 <= v6) {
      v2 = (unsigned long)(v6 - dat_1d150);
      v8 = v2 ^ 0x8000000000000000;
    }
    else {
      v2 = (unsigned long)v6;
      v8 = v2;
    }
  }
  v2 = v8;
  if (v8 <= (unsigned long)dat_23024 * 0x22)
    v2 = (unsigned long)dat_23024 * 0x22;
  return v2;
}

// Function: sub_637c @ 0x637c
long sub_637c(long a0,unsigned long a1,long a2,unsigned long a3,long a4)
{
  char *v1;
  unsigned int v10; // stack - 0xa0
  unsigned long v11; // stack - 0x88
  unsigned long v2;
  int v3; // eax
  unsigned long v4; // rax
  char v5 [24];
  bool v6;
  long v7; // stack - 0xe0
  unsigned long v8; // stack - 0xd8
  unsigned long v9; // stack - 0xd0
  
  v7 = a4 + 2;
  v8 = 0;
  while( true ) {
    if (a3 <= v8)
      return v7;
    if (a1 <= v8) { // branch-flip
      v1 = *(char **)(a2 + v8 * 8);
      if (strcmp(v1,"-")) { // branch-flip
        v1 = *(char **)(a2 + v8 * 8);
        v6 = stat(v1,v5) != 0;
      }
      else {
        v6 = fstat(0,v5) != 0;
      }
    }
    else {
      v3 = fileno(*(void **)(a0 + v8 * 8));
      v6 = fstat(v3,v5) != 0;
    }
    if (v6) {
      v2 = *(unsigned long *)(a2 + v8 * 8);
      sub_4464(gettext("stat failed"),v2); // no-return
    }
    if ((v10 & 0xf000) != 0x8000) { // branch-flip
      if (dat_23660)
        return dat_23660;
      v9 = 0x20000;
    }
    else {
      v9 = v11;
    }
    if ((!dat_23850) && (dat_23850 = dat_23660, !dat_23660))
      dat_23850 = sub_610d();
    v4 = v9 * (a4 + 1U) + 1;
    if ((v4 / (a4 + 1U) != v9) || ((unsigned long)(dat_23850 - v7) <= v4)) break;
    v7 += v4;
    v8 += 1;
  }
  return dat_23850;
}

// Function: sub_6618 @ 0x6618
void sub_6618(long *a0,long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x20
  
  v1 = a2;
  do {
    v1 = (v1 & 0xffffffffffffffe0) + 0x20;
    *a0 = (long)malloc(v1);
    if (*a0) {
      a0[5] = a1;
      a0[3] = v1;
      a0[2] = 0;
      a0[4] = a0[2];
      a0[1] = a0[4];
      *(char *)&a0[6] = 0;
      return;
    }
    v1 >>= 1;
  } while (a1 + 1U < v1);
  sub_1865c(); // no-return
}

// Function: sub_66d0 @ 0x66d0
long sub_66d0(long *a0)
{
  return a0[3] + *a0;
}

// Function: sub_66f4 @ 0x66f4
char * sub_66f4(unsigned long *a0,long *a1)
{
  long v1;
  char v2;
  long v3;
  char *v4;
  char *v5; // stack - 0x28
  long v6; // stack - 0x20
  
  v5 = (char *)*a0;
  v4 = &v5[a0[1] + -1];
  v6 = *a1;
  v3 = a1[1];
  if (dat_23020 != 0x80) { // branch-flip
    while ((v5 < v4 && (v1 = v6 + -1, v6))) {
      for (; (v5 < v4 && (*v5 != dat_23020)); v5 = &v5[1]) {
      }
      v6 = v1;
      if (v5 < v4)
        v5 = &v5[1];
    }
  }
  else {
    while ((v5 < v4 && (v1 = v6 + -1, v6))) {
      while ((v5 < v4 && (v2 = *v5, *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v2) + 0x23260)))) {
        v5 = &v5[1];
      }
      while ((v6 = v1, v5 < v4 && (v2 = *v5, *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v2) + 0x23260) != '\x01'))) {
        v5 = &v5[1];
      }
    }
  }
  if ((char)a1[6]) {
    while ((v5 < v4 && (v2 = *v5, *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v2) + 0x23260)))) {
      v5 = &v5[1];
    }
  }
  if (&v5[v3] < v4)
    v4 = &v5[v3];
  return v4;
}

// Function: sub_6898 @ 0x6898
char * sub_6898(unsigned long *a0,long a1)
{
  long v1;
  long v2;
  char *v3;
  unsigned char v4; // al
  unsigned int v5; // eax
  char *v6; // rax
  char *v7; // stack - 0x28
  long v8; // stack - 0x20
  
  v7 = (char *)*a0;
  v6 = &v7[a0[1] + -1];
  v3 = v6;
  v8 = *(long *)(a1 + 0x10);
  v2 = *(long *)(a1 + 0x18);
  if (!v2)
    v8 += 1;
  if (dat_23020 != 0x80) { // branch-flip
    while ((v7 < v6 && (v1 = v8 + -1, v8))) {
      for (; (v7 < v6 && (*v7 != dat_23020)); v7 = &v7[1]) {
      }
      v8 = v1;
      if ((v7 < v6) && ((v1 || (v2))))
        v7 = &v7[1];
    }
  }
  else {
    while ((v7 < v3 && (v1 = v8 + -1, v8))) {
      while ((v7 < v3 && (v5 = (int)*v7, v4 = sub_4042(v5), v5 = (unsigned int)v4, v6 = (long)(long)(int)v5, *(char *)((long)v6 + 0x23260)))) {
        v7 = &v7[1];
      }
      while ((v8 = v1, v7 < v3 && (v5 = (int)*v7, v4 = sub_4042(v5), v5 = (unsigned int)v4, v6 = (long)(long)(int)v5, *(char *)((long)v6 + 0x23260) != '\x01'))) {
        v7 = &v7[1];
      }
    }
  }
  if (v2) {
    if (*(char *)(a1 + 0x31)) {
      while ((v7 < v3 && (v5 = (int)*v7, v4 = sub_4042(v5), v5 = (unsigned int)v4, v6 = (long)(long)(int)v5, *(char *)((long)v6 + 0x23260)))) {
        v7 = &v7[1];
      }
    }
    v6 = v3;
    if (&v7[v2] < v3)
      v6 = &v7[v2];
    v7 = v6;
  }
  v6 = v7;
  return v6;
}

// Function: sub_6a5e @ 0x6a5e
unsigned long sub_6a5e(long *a0,void *a1,unsigned long a2)
{
  char *v1;
  char *v10; // stack - 0x68
  long *v11; // stack - 0x60
  unsigned long v12; // stack - 0x58
  char *v13; // stack - 0x50
  char *v14; // stack - 0x48
  long *v15; // stack - 0x40
  long v16; // stack - 0x38
  long v17; // stack - 0x30
  unsigned long v18; // stack - 0x28
  unsigned long v19; // stack - 0x20
  long *v2;
  char *v20; // stack - 0x18
  char v3;
  char v4;
  unsigned long v5;
  long v6;
  char *v7; // rax
  unsigned long v8; // stack - 0x78
  unsigned long v9; // stack - 0x70
  
  v4 = dat_23010;
  v15 = dat_23688;
  v16 = a0[5];
  v9 = dat_23018 - 0x22;
  if ((char)a0[6])
    return 0;
  if (a0[1] != a0[4]) {
    memmove((void *)*a0,(void *)((a0[1] - a0[4]) + *a0),a0[4]);
    a0[1] = a0[4];
    a0[2] = 0;
  }
  do {
    v10 = (char *)(a0[1] + *a0);
    v17 = sub_66d0(a0);
    v11 = (long *)(v17 + a0[2] * -0x20);
    v12 = (v17 - a0[2] * v16) - (long)v10;
    if (a0[2]) // branch-flip
      v13 = (char *)(v11[1] + *v11);
    else {
      v13 = (char *)*a0;
    }
    do {
      if (v12 <= v16 + 1U) break;
      v18 = (v12 - 1) / (v16 + 1U);
      v19 = fread_unlocked(v10,1,v18,a1);
      v14 = &v10[v19];
      v12 -= v19;
      if (v19 != v18) {
        if (ferror_unlocked(a1))
          sub_4464(gettext("read failed"),a2); // no-return
        if (feof_unlocked(a1)) {
          *(char *)&a0[6] = 1;
          if (v14 == (char *)*a0)
            return 0;
          if ((v13 != v14) && (v4 != v14[-1])) {
            v1 = &v14[1];
            *v14 = v4;
            v14 = v1;
          }
        }
      }
      while( true ) {
        v7 = memchr(v10,(int)v4,(long)v14 - (long)v10);
        v20 = v7;
        if (!v7) break;
        *v7 = 0;
        v10 = &v7[1];
        v2 = &v11[-4];
        *v2 = (long)v13;
        v11[-3] = (long)v10 - (long)v13;
        v5 = v11[-3];
        if ((unsigned long)v11[-3] <= v9)
          v5 = v9;
        v12 -= v16;
        v9 = v5;
        v11 = v2;
        if (v15) {
          if (v15[2] != -1)
            v7 = (char *)sub_6898(v2,v15);
          v11[3] = (long)v7;
          if (*v15 != -1) { // branch-flip
            v6 = sub_66f4(v11,v15);
            v11[2] = v6;
          }
          else {
            if ((char)v15[6]) {
              while (v3 = *v13, *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v3) + 0x23260)) {
                v13 = &v13[1];
              }
            }
            v11[2] = (long)v13;
          }
        }
        v13 = v10;
      }
      v10 = v14;
    } while (!(char)a0[6]);
    a0[1] = (long)v10 - *a0;
    a0[2] = sub_66d0(a0) - (long)v11 >> 5;
    if (a0[2]) {
      a0[4] = (long)v10 - (long)v13;
      dat_23018 = v9 + 0x22;
      return 1;
    }
    v8 = (unsigned long)a0[3] >> 5;
    v6 = *a0;
    *a0 = sub_18221(v6,&v8,0x20);
    a0[3] = v8 << 5;
  } while( true );
}

// Function: sub_6f64 @ 0x6f64
char sub_6f64(long *a0)
{
  char *v1;
  char v2;
  bool v3;
  char v4; // stack - 0x13
  char *v5; // stack - 0x10
  
  v4 = '\0';
  v3 = 0;
  v1 = (char *)*a0;
  while( true ) {
    v5 = v1;
    v1 = &v5[1];
    v2 = *v5;
    if (10 <= (unsigned int)((int)v2 - 0x30U)) break;
    if (v4 < v2)
      v4 = v2;
    v3 = *v1 == dat_23224;
    if (v3)
      v1 = &v5[2];
  }
  if (v3)
    *a0 = (long)&v5[-1];
  else {
    v5 = v1;
    if (v2 == dat_23220) {
      while( true ) {
        v1 = &v5[1];
        v2 = *v5;
        v5 = v1;
        if (10 <= (unsigned int)((int)v2 - 0x30U)) break;
        if (v4 < v2)
          v4 = v2;
      }
    }
    *a0 = (long)&v5[-1];
  }
  return v4;
}

// Function: sub_7042 @ 0x7042
int sub_7042(char *a0) // early-return
{
  char v1;
  int v2; // eax
  unsigned char *v3; // stack - 0x18
  
  v1 = *a0;
  v3 = (unsigned char *)&a0[v1 == '-'];
  if ('1' > (char)sub_6f64(&v3))
    return 0;
  v2 = (int)*(char *)((long)(int)(unsigned int)*v3 + 0x1c5c0);
  if (v1 == '-')
    v2 = -v2;
  return v2;
}

// Function: sub_70e2 @ 0x70e2
unsigned long sub_70e2(char *a0,char *a1)
{
  int v1;
  unsigned char v2; // al
  unsigned int v3; // eax
  unsigned long v4; // rax
  char *v5; // stack - 0x38
  char *v6; // stack - 0x30
  
  v6 = a0;
  while (v3 = (int)*v6, v2 = sub_4042(v3), v3 = (unsigned int)v2, v4 = (long)(int)v3, v5 = a1, *(char *)(v4 + 0x23260)) {
    v6 = &v6[1];
  }
  while (v3 = (int)*v5, v2 = sub_4042(v3), v3 = (unsigned int)v2, v4 = (long)(int)v3, *(char *)(v4 + 0x23260)) {
    v5 = &v5[1];
  }
  v3 = sub_7042(v6);
  v1 = v3;
  v3 = sub_7042(v5);
  v3 = v1 - v3;
  if (v3) // branch-flip
    v4 = (unsigned long)v3;
  else {
    v4 = sub_1764c(v6,v5,(int)dat_23220,dat_23224);
  }
  return v4;
}

// Function: sub_71a3 @ 0x71a3
void sub_71a3(char *a0,char *a1)
{
  char v1;
  char *v2; // stack - 0x18
  char *v3; // stack - 0x10
  
  v3 = a0;
  while (v1 = *v3, v2 = a1, *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v1) + 0x23260)) {
    v3 = &v3[1];
  }
  while (v1 = *v2, *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v1) + 0x23260)) {
    v2 = &v2[1];
  }
  sub_1764c(v3,v2,(int)dat_23220,dat_23224);
}

// Function: sub_7231 @ 0x7231
void sub_7231(unsigned long a0,unsigned long a1)
{
  undefined10 v1; // stack - 0x38
  undefined10 v2; // stack - 0x28
  undefined10 v3; // st0
  undefined10 v4; // st0
  
  memset(&v1,0,0x10);
  strtold(a0,0);
  v1 = v3;
  memset(&v2,0,0x10);
  strtold(a1,0);
  v2 = v4;
  memcmp(&v1,&v2,0x10);
}

// Function: sub_72d6 @ 0x72d6
unsigned long sub_72d6(long a0,long a1) // early-return x8
{
  long v1; // stack - 0x48
  long v2; // stack - 0x40
  long double v3; // st0
  long double v4; // st0
  long double v5; // stack - 0x38
  
  strtold(a0,&v1);
  v5 = v3;
  strtold(a1,&v2);
  if (a0 == v1) {
    if (a1 == v2)
      return 0;
    return 0xffffffff;
  }
  if (a1 == v2)
    return 1;
  if (v5 < v4)
    return 0xffffffff;
  if (v4 < v5)
    return 1;
  if (v4 == v5)
    return 0;
  if (!NAN(v4) && !NAN(v4))
    return 0xffffffff;
  if (NAN(v5))
    return sub_7231(a0,a1);
  return 1;
}

// Function: sub_73f3 @ 0x73f3
unsigned int sub_73f3(char *a0,unsigned long *a1)
{
  char v1;
  unsigned char v2; // al
  unsigned long v3; // rax
  char *v4; // stack - 0x48
  unsigned long v5; // stack - 0x38
  unsigned long v6; // stack - 0x30
  char *v7; // stack - 0x28
  char *v8; // stack - 0x20
  
  v5 = 0;
  v6 = 0xc;
  v4 = a0;
  while (v1 = *v4, *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v1) + 0x23260)) {
    v4 = &v4[1];
  }
  do {
    v3 = v6 + v5 >> 1;
    v7 = v4;
    v8 = *(char **)(v3 * 0x10 + 0x23040);
    while( true ) {
      if (!*v8) {
        if (a1)
          *a1 = v7;
        return *(unsigned int *)(v3 * 0x10 + 0x23048);
      }
      v1 = *v7;
      v2 = sub_4042((int)*(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v1) + 0x23560));
      v1 = *v8;
      if (v2 < (unsigned char)sub_4042((int)v1)) {
        v6 = v3;
        goto label_755a;
      }
      v1 = *v7;
      v2 = sub_4042((int)*(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v1) + 0x23560));
      v1 = *v8;
      if ((unsigned char)sub_4042((int)v1) < v2) break;
      v7 = &v7[1];
      v8 = &v8[1];
    }
    v5 = v3 + 1;
label_755a:
    if (v6 <= v5)
      return 0;
  } while( true );
}

// Function: sub_7573 @ 0x7573
void sub_7573(char *a0)
{
  long v1; // rax
  char v2 [24];
  
  v1 = sub_15668(a0,0x10);
  if (!v1) {
    if (!a0)
      a0 = "getrandom";
    sub_4464(gettext("open failed"),a0); // no-return
  }
  sub_159b4(v1,v2,0x10);
  if (sub_15a0f(v1))
    sub_4464(gettext("close failed"),a0); // no-return
  sub_fb2c(0x23740);
  sub_fb4b(v2,0x10,0x23740);
}

// Function: sub_7668 @ 0x7668
unsigned long sub_7668(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // rax
  char *v2; // rax
  
  *__errno_location() = 0;
  v1 = strxfrm(a0,a1,a2);
  if (*__errno_location()) {
    v2 = gettext("string transformation failed");
    error(0,*__errno_location(),v2); // no-return
  }
  return v1;
}

// Function: sub_7755 @ 0x7755
int sub_7755(char *a0,unsigned long a1,char *a2,unsigned long a3) // ternary
{
  unsigned long v1;
  unsigned long v10; // stack - 0x1108
  char *v11; // stack - 0x1100
  unsigned long v12; // stack - 0x10f8
  char *v13; // stack - 0x10f0
  int v14; // stack - 0x10e0
  int v15; // stack - 0x10dc
  char *v16; // stack - 0x10d8
  unsigned long v17; // stack - 0x10d0
  char *v18; // stack - 0x10c8
  unsigned long v19; // stack - 0x1070
  unsigned long v2; // rax
  unsigned long v20; // stack - 0x1068
  unsigned long v21; // stack - 0x1060
  unsigned long v22; // stack - 0x1058
  unsigned long v23; // stack - 0x1050
  unsigned long v24; // stack - 0x1048
  unsigned long v25; // stack - 0x1040
  unsigned long v26; // stack - 0x1038
  unsigned long v27; // stack - 0x1030
  unsigned long v28; // stack - 0x1028
  unsigned int v29; // stack - 0x1020
  long v3;
  unsigned long v30; // stack - 0x1014
  unsigned int v31; // stack - 0x100c
  unsigned int v32; // stack - 0x1008
  unsigned int v33; // stack - 0x1004
  unsigned int v34; // stack - 0x1000
  unsigned long v35; // stack - 0xffc
  unsigned long v36; // stack - 0xff4
  unsigned long v37; // stack - 0xfec
  unsigned long v38; // stack - 0xfe4
  unsigned long v39; // stack - 0xfdc
  char *v4; // rcx
  unsigned long v40; // stack - 0xfd4
  unsigned long v41; // stack - 0xfcc
  unsigned int v42; // stack - 0xfc4
  char v5 [4008];
  unsigned long v6; // stack - 0x1078
  unsigned long v7; // stack - 0x101c
  char v8 [16];
  char v9 [16];
  
  v14 = 0;
  v16 = v5;
  v17 = 4000;
  v18 = NULL;
  v7 = dat_23740;
  v30 = dat_23748;
  v32 = (unsigned int)((unsigned long)dat_23750 >> 0x20);
  v31 = (unsigned int)dat_23750;
  v34 = (unsigned int)((unsigned long)dat_23758 >> 0x20);
  v33 = (unsigned int)dat_23758;
  v35 = dat_23760;
  v36 = dat_23768;
  v37 = dat_23770;
  v38 = dat_23778;
  v39 = dat_23780;
  v40 = dat_23788;
  v41 = dat_23790;
  v42 = dat_23798;
  v6 = dat_23740;
  v19 = dat_23748;
  v20 = dat_23750;
  v21 = dat_23758;
  v22 = dat_23760;
  v23 = dat_23768;
  v24 = dat_23770;
  v25 = dat_23778;
  v26 = dat_23780;
  v27 = dat_23788;
  v28 = dat_23790;
  v29 = dat_23798;
  v10 = a3;
  v11 = a2;
  v12 = a1;
  v13 = a0;
  if (dat_23229) {
    a0 = &a0[a1];
    a2 = &a2[a3];
    while( true ) {
      v1 = (a1 + a3) * 3 + 2;
      if (v17 < v1) {
        v2 = v17 * 3 >> 1;
        v17 = v1;
        if (v1 < v2)
          v17 = v2;
        free(v18);
        v18 = malloc(v17);
        v16 = v18;
        if (!v18) {
          v16 = v5;
          v17 = 4000;
        }
      }
      v12 = (a0 <= v13) ? 0 : sub_7668(v16,v13,v17) + 1; // branch-flip
      if (a2 <= v11) // branch-flip
        v10 = 0;
      else {
        if (v17 < v12) {
          v3 = 0;
          v4 = NULL;
        }
        else {
          v3 = v17 - v12;
          v4 = &v16[v12];
        }
        v10 = sub_7668(v4,v11,v3) + 1;
      }
      if ((v17 < v12) || (v17 < v10 + v12)) {
        v17 = v10 + v12;
        if (v17 <= 0x5555555555555554)
          v17 = v17 * 3 >> 1;
        free(v18);
        v16 = (char *)sub_18024(v17);
        v18 = v16;
        if (v13 < a0)
          strxfrm(v16,v13,v12);
        if (v11 < a2)
          strxfrm(&v16[v12],v11,v10);
      }
      if (v13 < a0)
        v13 = &v13[strlen(v13) + 1];
      if (v11 < a2)
        v11 = &v11[strlen(v11) + 1];
      if ((a0 <= v13) && (a2 <= v11)) break;
      sub_fb4b(v16,v12,&v6);
      sub_fb4b(&v16[v12],v10,&v7);
      if (!v14) {
        v1 = v10;
        if (v12 < v10)
          v1 = v12;
        v14 = memcmp(v16,&v16[v12],v1);
        if (!v14)
          v14 = (unsigned int)(v10 < v12) - (unsigned int)(v12 < v10);
      }
    }
    v13 = v16;
    v11 = &v16[v12];
  }
  sub_fb4b(v13,v12,&v6);
  sub_fbaf(&v6,v8);
  sub_fb4b(v11,v10,&v7);
  sub_fbaf(&v7,v9);
  v15 = memcmp(v8,v9,0x10);
  if (!v15) {
    if (!v14) {
      v1 = v10;
      if (v12 < v10)
        v1 = v12;
      v14 = memcmp(v13,v11,v1);
      if (!v14)
        v14 = (unsigned int)(v10 < v12) - (unsigned int)(v12 < v10);
    }
    v15 = v14;
  }
  free(v18);
  return v15;
}

// Function: sub_7ee4 @ 0x7ee4
long sub_7ee4(char *a0,char *a1)
{
  char *v1; // stack - 0x20
  long v2; // stack - 0x10
  
  v2 = (long)(int)sub_1293d(a0,(long)a1 - (long)a0,0);
  for (v1 = a0; v1 < a1; v1 = &v1[1]) {
    v2 += (int)(unsigned int)(*v1 == '\t');
  }
  return v2;
}

// Function: sub_7f4c @ 0x7f4c
void sub_7f4c(long a0,long a1) // return-dupe
{
  long v1;
  long v2; // stack - 0x18
  long v3; // stack - 0x10
  
  v3 = a0;
  while (v1 = v3 + -1, v3) {
    putchar_unlocked(0x20);
    v3 = v1;
  }
  v2 = a1;
  if (!a1) {
    printf(gettext("^ no match for key\n"));
    return;
  }
  do {
    putchar_unlocked(0x5f);
    v2 -= 1;
  } while (v2);
  putchar_unlocked(10);
}

// Function: sub_7fc5 @ 0x7fc5
unsigned int sub_7fc5(long a0) // return-dupe
{
  if (((!*(char *)(a0 + 0x32)) && (!*(char *)(a0 + 0x34))) && (!*(char *)(a0 + 0x35)))
    return 0;
  return 1;
}

// Function: sub_8002 @ 0x8002
void sub_8002(unsigned long *a0,long *a1) // warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_8002
{
  unsigned char v1;
  unsigned long v10; // stack - 0x38
  unsigned long v11; // stack - 0x30
  undefined10 v12; // stack - 0x28
  unsigned char v2;
  int v3; // eax
  unsigned char *v4; // stack - 0x60
  unsigned char *v5; // stack - 0x58
  undefined10 v6; // st0
  unsigned char *v7; // stack - 0x50
  unsigned char *v8; // stack - 0x48
  unsigned char *v9; // stack - 0x40
  
  v7 = (unsigned char *)*a0;
  v8 = &v7[a0[1] + -1];
  v9 = v7;
  if (a1) {
    if (*a1 != -1)
      v7 = (unsigned char *)sub_66f4(a0,a1);
    if (a1[2] != -1)
      v8 = (unsigned char *)sub_6898(a0,a1);
    if ((((char)a1[6]) && (*a1 == -1)) || ((*(char *)((long)a1 + 0x36) || (sub_7fc5(a1))))) {
      v1 = *v8;
      *v8 = 0;
      while (v2 = *v7, *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)(char)v2) + 0x23260)) {
        v7 = &v7[1];
      }
      v4 = v7;
      if (v7 <= v8) { // branch-flip
        if (*(char *)((long)a1 + 0x36)) // branch-flip
          sub_73f3(v7,&v4);
        else if (*(char *)((long)a1 + 0x34)) { // branch-flip
          strtold(v7,&v4);
          v12 = v6;
        }
        else if ((*(char *)((long)a1 + 0x32)) || (*(char *)((long)a1 + 0x35))) {
          v3 = !((v8 <= v7) || (*v7 != 0x2d));
          v5 = &v7[v3];
          if ('0' <= (char)sub_6f64(&v5)) {
            v3 = ((*(char *)((long)a1 + 0x35)) && (*(char *)((long)(int)(unsigned int)*v5 + 0x1c5c0)));
            v4 = &v5[v3];
          }
        }
        else {
          v4 = v8;
        }
      }
      else {
        v4 = v8;
      }
      *v8 = v1;
      v8 = v4;
    }
  }
  v10 = sub_7ee4(v9,v7);
  v11 = sub_7ee4(v7,v8);
  sub_7f4c(v10,v11);
}

// Function: sub_829e @ 0x829e
void sub_829e(unsigned long a0)
{
  long v1; // stack - 0x10
  
  v1 = dat_23688;
  while( true ) {
    do {
      sub_8002(a0,v1);
      if (!v1)
        return;
      v1 = *(long *)(v1 + 0x40);
    } while (v1);
    if (dat_23682 == '\x01') break;
    if (dat_23681 == '\x01')
      return;
  }
}

// Function: sub_8305 @ 0x8305
unsigned int sub_8305(long a0) // return-dupe
{
  if (((((!*(long *)(a0 + 0x20)) && (!*(long *)(a0 + 0x28))) && (*(char *)(a0 + 0x30) != '\x01')) && ((*(char *)(a0 + 0x31) != '\x01' && (sub_7fc5(a0) != '\x01')))) && ((*(char *)(a0 + 0x36) != '\x01' && ((*(char *)(a0 + 0x38) != '\x01' && (*(char *)(a0 + 0x33) != '\x01'))))))
    return 1;
  return 0;
}

// Function: sub_839e @ 0x839e
void sub_839e(long a0,char *a1)
{
  char *v1;
  char *v2; // stack - 0x18
  
  if ((*(char *)(a0 + 0x30)) || (v2 = a1, *(char *)(a0 + 0x31))) {
    v2 = &a1[1];
    *a1 = 0x62;
  }
  if (*(long *)(a0 + 0x20) == 0x23460) {
    v1 = &v2[1];
    *v2 = 100;
    v2 = v1;
  }
  if (*(long *)(a0 + 0x28)) {
    v1 = &v2[1];
    *v2 = 0x66;
    v2 = v1;
  }
  if (*(char *)(a0 + 0x34)) {
    v1 = &v2[1];
    *v2 = 0x67;
    v2 = v1;
  }
  if (*(char *)(a0 + 0x35)) {
    v1 = &v2[1];
    *v2 = 0x68;
    v2 = v1;
  }
  if (*(long *)(a0 + 0x20) == 0x23360) {
    v1 = &v2[1];
    *v2 = 0x69;
    v2 = v1;
  }
  if (*(char *)(a0 + 0x36)) {
    v1 = &v2[1];
    *v2 = 0x4d;
    v2 = v1;
  }
  if (*(char *)(a0 + 0x32)) {
    v1 = &v2[1];
    *v2 = 0x6e;
    v2 = v1;
  }
  if (*(char *)(a0 + 0x33)) {
    v1 = &v2[1];
    *v2 = 0x52;
    v2 = v1;
  }
  if (*(char *)(a0 + 0x37)) {
    v1 = &v2[1];
    *v2 = 0x72;
    v2 = v1;
  }
  if (*(char *)(a0 + 0x38)) {
    v1 = &v2[1];
    *v2 = 0x56;
    v2 = v1;
  }
  *v2 = 0;
}

// Function: sub_84fe @ 0x84fe
void sub_84fe(unsigned long *a0,char a1) // warn: iteboolean: re-rolled 3 0/1 select diamond(s) to a boolean assignment in sub_84fe
{
  unsigned long v1;
  char *v10; // rax
  char v11 [48];
  char v12 [56]; // stack - 0x58
  char v13 [32];
  unsigned long v14; // stack - 0xf8
  unsigned long *v15; // stack - 0x138
  long v16; // stack - 0x130
  unsigned long v17; // stack - 0x128
  unsigned long v18; // stack - 0x120
  unsigned long v19; // stack - 0xf0
  bool v2;
  unsigned long v20; // stack - 0xe8
  unsigned long v21; // stack - 0xe0
  unsigned long v22; // stack - 0xd8
  unsigned long v23; // stack - 0xd0
  unsigned long v24; // stack - 0xc8
  unsigned long v25; // stack - 0xc0
  unsigned long v26; // stack - 0xb8
  bool v3;
  bool v4; // al
  bool v5; // al
  char *v6; // rax
  char *v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v14 = *a0;
  v19 = a0[1];
  v20 = a0[2];
  v21 = a0[3];
  v22 = a0[4];
  v23 = a0[5];
  v24 = a0[6];
  v25 = a0[7];
  v26 = a0[8];
  v16 = 1;
  v2 = 0;
  v3 = 0;
  v15 = dat_23688;
  while( true ) {
    if (!v15) {
      if ((v2) || (v3)) {
        v12[0] = dat_23220;
        v12[1] = 0;
        v9 = sub_154a0(v12);
        if (dat_23220 != dat_23020) // branch-flip
          v10 = gettext("note ");
        else {
          v10 = "";
        }
        error(0,0,gettext("%snumbers use %s as a decimal point in this locale"),v10,v9); // no-return
      }
      if ((sub_8305(&v14) == '\x01') && ((!v24._7_1_ || (((!dat_23681 && (!dat_23682)) || (!dat_23688)))))) {
        if ((((v24._7_1_) && (dat_23681 != '\x01')) && (dat_23682 != '\x01')) && (dat_23688))
          error(0,0,gettext("option \'-r\' only applies to last-resort comparison")); // no-return
        return;
      }
      if ((dat_23681 != '\x01') && (dat_23682 != '\x01'))
        v24 &= 0xffffffffffffff;
      sub_839e(&v14,v12);
      v9 = sub_4094(strlen(v12));
      error(0,0,(char *)ngettext("option \'-%s\' is ignored","options \'-%s\' are ignored",v9),v12); // no-return
    }
    if (sub_7fc5(v15)) {
      if (*(char *)((long)v15 + 0x34)) // branch-flip
        v3 = 1;
      else {
        v2 = 1;
      }
    }
    if (*(char *)((long)v15 + 0x39)) {
      v17 = *v15;
      v1 = v15[2];
      if (v17 == 0xffffffffffffffff)
        v17 = 0;
      v10 = (char *)sub_1287e(v17,v13);
      v10 = stpcpy(stpcpy(v11,"+"),v10);
      v6 = (char *)sub_1287e(v17 + 1,v13);
      v6 = stpcpy(stpcpy(v12,"-k "),v6);
      if (v15[2] != 0xffffffffffffffff) {
        v7 = (char *)sub_1287e(v1 + 1,v13);
        stpcpy(stpcpy(v10," -"),v7);
        v10 = (char *)sub_1287e(v1 + (v15[3] == 0xffffffffffffffff) + 1,v13);
        stpcpy(stpcpy(v6,","),v10);
      }
      v9 = sub_15473(1,v12);
      v8 = sub_15473(0,v11);
      error(0,0,gettext("obsolescent key %s used; consider %s instead"),v8,v9); // no-return
    }
    v4 = ((*v15 != 0xffffffffffffffff) && (v15[2] < *v15));
    if (v4) break;
    v4 = ((sub_7fc5(v15)) || (*(char *)((long)v15 + 0x36)));
    v5 = !((v15[2]) || (!v15[3]));
    if ((((a1 != '\x01') && (dat_23020 == 0x80)) && (!v5)) && (((((char)v15[6] != '\x01' && (!v4)) || (((char)v15[6] != '\x01' && (v15[1])))) || ((*(char *)((long)v15 + 0x31) != '\x01' && (v15[3]))))))
      error(0,0,gettext("leading blanks are significant in key %lu; consider also specifying \'b\'"),v16); // no-return
    if ((a1 != '\x01') && (sub_7fc5(v15))) {
      v18 = *v15 + 1;
      if (!v18)
        v18 = *v15 + 2;
      if ((v15[2] == 0xffffffffffffffff) || (v18 < v15[2] + 1))
        error(0,0,gettext("key %lu is numeric and spans multiple fields"),v16); // no-return
    }
    if ((v22) && (v22 == v15[4]))
      v22 = 0;
    if ((v23) && (v23 == v15[5]))
      v23 = 0;
    v24._0_2_ = CONCAT11(((*(unsigned char *)((long)v15 + 0x31) ^ 1) & v24._1_1_) != 0,(((unsigned char)v15[6] ^ 1) & (unsigned char)v24) != 0);
    v24._0_3_ = CONCAT12(((*(unsigned char *)((long)v15 + 0x32) ^ 1) & v24._2_1_) != 0,(unsigned short)v24);
    v24._0_4_ = CONCAT13(((*(unsigned char *)((long)v15 + 0x33) ^ 1) & v24._3_1_) != 0,(undefined3)v24);
    v24 = CONCAT44(CONCAT22(CONCAT11(v24._7_1_,((*(unsigned char *)((long)v15 + 0x36) ^ 1) & v24._6_1_) != 0),CONCAT11(((*(unsigned char *)((long)v15 + 0x35) ^ 1) & v24._5_1_) != 0,((*(unsigned char *)((long)v15 + 0x34) ^ 1) & v24._4_1_) != 0)),(unsigned int)v24);
    v25 = CONCAT71(v25._1_7_,(((unsigned char)v15[7] ^ 1) & (unsigned char)v25) != 0);
    v24 = CONCAT17(((*(unsigned char *)((long)v15 + 0x37) ^ 1) & v24._7_1_) != 0,(undefined7)v24);
    v15 = (unsigned long *)v15[8];
    v16 += 1;
  }
  error(0,0,gettext("key %lu has zero width and will be ignored"),v16); // no-return
}

// Function: sub_9116 @ 0x9116
unsigned int sub_9116(long *a0,long *a1) // ternary x2
{
  char *v1;
  unsigned long v10;
  unsigned long v11; // rax
  unsigned long v12;
  char v13 [4008];
  char v14; // stack - 0x104e
  char v15; // stack - 0x104d
  unsigned int v16; // stack - 0x104c
  long *v17; // stack - 0x1048
  char *v18; // stack - 0x1040
  char *v19; // stack - 0x1038
  char *v2;
  char *v20; // stack - 0x1030
  char *v21; // stack - 0x1028
  char *v22; // stack - 0x1020
  char *v23; // stack - 0x1018
  unsigned long v24; // stack - 0x1010
  unsigned long v25; // stack - 0x1008
  char *v26; // stack - 0x1000
  unsigned long v27; // stack - 0xff8
  long v3;
  long v4;
  char v5;
  unsigned char v6; // al
  int v7; // eax
  char *v8;
  char *v9;
  
  v17 = dat_23688;
  v18 = (char *)a0[2];
  v19 = (char *)a1[2];
  v20 = (char *)a0[3];
  v21 = (char *)a1[3];
  do {
    v3 = v17[5];
    v4 = v17[4];
    v8 = v18;
    if (v18 <= v20)
      v8 = v20;
    v9 = v19;
    if (v19 <= v21)
      v9 = v21;
    v10 = (long)v8 - (long)v18;
    v11 = (long)v9 - (long)v19;
    if ((((dat_23229) || (sub_7fc5(v17))) || (*(char *)((long)v17 + 0x36))) || ((*(char *)((long)v17 + 0x33) || ((char)v17[7])))) {
      if ((v4) || (v3)) {
        v12 = v11 + v10 + 2;
        if (0xfa1 <= v12) { // branch-flip
          v26 = (char *)sub_18024(v12);
          v22 = v26;
        }
        else {
          v22 = v13;
          v26 = NULL;
        }
        v23 = &v22[v10 + 1];
        v24 = 0;
        for (v27 = 0; v27 < v10; v27 = v27 + 1) {
          if (v4) { // branch-flip
            v5 = v18[v27];
            if (*(char *)(v4 + (unsigned long)(unsigned char)sub_4042((int)v5)) != '\x01') goto label_9370;
          }
          else {
label_9370:
            if (v3) { // branch-flip
              v5 = v18[v27];
              v5 = *(char *)(v3 + (unsigned long)(unsigned char)sub_4042((int)v5));
            }
            else {
              v5 = v18[v27];
            }
            v12 = v24 + 1;
            v22[v24] = v5;
            v24 = v12;
          }
        }
        v22[v24] = '\0';
        v25 = 0;
        for (v27 = 0; v27 < v11; v27 = v27 + 1) {
          if (v4) { // branch-flip
            v5 = v19[v27];
            if (*(char *)(v4 + (unsigned long)(unsigned char)sub_4042((int)v5)) != '\x01') goto label_9469;
          }
          else {
label_9469:
            if (v3) { // branch-flip
              v5 = v19[v27];
              v5 = *(char *)(v3 + (unsigned long)(unsigned char)sub_4042((int)v5));
            }
            else {
              v5 = v19[v27];
            }
            v10 = v25 + 1;
            v23[v25] = v5;
            v25 = v10;
          }
        }
        v23[v25] = '\0';
      }
      else {
        v22 = v18;
        v14 = v18[v10];
        v18[v10] = '\0';
        v23 = v19;
        v15 = v19[v11];
        v19[v11] = '\0';
        v24 = v10;
        v25 = v11;
      }
      if (*(char *)((long)v17 + 0x32)) // branch-flip
        v16 = sub_71a3(v22,v23);
      else if (*(char *)((long)v17 + 0x34)) // branch-flip
        v16 = sub_72d6(v22,v23);
      else if (*(char *)((long)v17 + 0x35)) // branch-flip
        v16 = sub_70e2(v22,v23);
      else if (*(char *)((long)v17 + 0x36)) { // branch-flip
        v7 = sub_73f3(v22,0);
        v16 = v7 - sub_73f3(v23,0);
      }
      else if (*(char *)((long)v17 + 0x33)) // branch-flip
        v16 = sub_7755(v22,v24,v23,v25);
      else if ((char)v17[7]) // branch-flip
        v16 = sub_10461(v22,v24,v23,v25);
      else if (v24) { // branch-flip
        v16 = (v25) ? sub_187ec(v22,v24 + 1,v23,v25 + 1) : 1; // branch-flip
      }
      else {
        v16 = -(unsigned int)(v25 != 0);
      }
      if ((v4) || (v3))
        free(v26);
      else {
        v22[v24] = v14;
        v23[v25] = v15;
      }
    }
    else if (v4) { // branch-flip
      if (v3) { // branch-flip
        while( true ) {
          while ((v18 < v8 && (v5 = *v18, *(char *)(v4 + (unsigned long)(unsigned char)sub_4042((int)v5))))) {
            v18 = &v18[1];
          }
          while ((v19 < v9 && (v5 = *v19, *(char *)(v4 + (unsigned long)(unsigned char)sub_4042((int)v5))))) {
            v19 = &v19[1];
          }
          if ((v8 <= v18) || (v9 <= v19)) break;
          v5 = *v18;
          v6 = sub_4042((int)*(char *)(v3 + (unsigned long)(unsigned char)sub_4042((int)v5)));
          v5 = *v19;
          v5 = *(char *)(v3 + (unsigned long)(unsigned char)sub_4042((int)v5));
          v16 = (unsigned int)v6 - (unsigned int)(unsigned char)sub_4042((int)v5);
          if (v16) goto label_9df1;
          v18 = &v18[1];
          v19 = &v19[1];
        }
        v16 = (unsigned int)(v18 < v8) - (unsigned int)(v19 < v9);
      }
      else {
        while( true ) {
          while ((v18 < v8 && (v5 = *v18, *(char *)(v4 + (unsigned long)(unsigned char)sub_4042((int)v5))))) {
            v18 = &v18[1];
          }
          while ((v19 < v9 && (v5 = *v19, *(char *)(v4 + (unsigned long)(unsigned char)sub_4042((int)v5))))) {
            v19 = &v19[1];
          }
          if ((v8 <= v18) || (v9 <= v19)) break;
          v6 = sub_4042((int)*v18);
          v5 = *v19;
          v16 = (unsigned int)v6 - (unsigned int)(unsigned char)sub_4042((int)v5);
          if (v16) goto label_9df1;
          v18 = &v18[1];
          v19 = &v19[1];
        }
        v16 = (unsigned int)(v18 < v8) - (unsigned int)(v19 < v9);
      }
    }
    else if (v10) { // branch-flip
      if (!v11) {
        v16 = 1;
        goto label_9df1;
      }
      if (v3) { // branch-flip
        while ((v18 < v8 && (v19 < v9))) {
          v1 = &v18[1];
          v5 = *v18;
          v6 = sub_4042((int)*(char *)(v3 + (unsigned long)(unsigned char)sub_4042((int)v5)));
          v2 = &v19[1];
          v5 = *v19;
          v5 = *(char *)(v3 + (unsigned long)(unsigned char)sub_4042((int)v5));
          v16 = (unsigned int)v6 - (unsigned int)(unsigned char)sub_4042((int)v5);
          v18 = v1;
          v19 = v2;
          if (v16) goto label_9df1;
        }
      }
      else {
        v12 = v11;
        if (v10 < v11)
          v12 = v10;
        v16 = memcmp(v18,v19,v12);
        if (v16) goto label_9df1;
      }
      v16 = (v11 <= v10) ? (unsigned int)(v10 != v11) : 0xffffffff; // branch-flip
    }
    else {
      v16 = -(unsigned int)(v11 != 0);
    }
    if (v16) {
label_9df1:
      if (!*(char *)((long)v17 + 0x37))
        return v16;
      return -v16;
    }
    v17 = (long *)v17[8];
    if (!v17)
      return 0;
    if (v17[2] != -1) { // branch-flip
      v20 = (char *)sub_6898(a0,v17);
      v21 = (char *)sub_6898(a1,v17);
    }
    else {
      v20 = (char *)(a0[1] + -1 + *a0);
      v21 = (char *)(a1[1] + -1 + *a1);
    }
    if (*v17 != -1) { // branch-flip
      v18 = (char *)sub_66f4(a0,v17);
      v19 = (char *)sub_66f4(a1,v17);
    }
    else {
      v18 = (char *)*a0;
      v19 = (char *)*a1;
      if ((char)v17[6]) {
        while ((v18 < v20 && (v5 = *v18, *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v5) + 0x23260)))) {
          v18 = &v18[1];
        }
        while ((v19 < v21 && (v5 = *v19, *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v5) + 0x23260)))) {
          v19 = &v19[1];
        }
      }
    }
  } while( true );
}

// Function: sub_9e2a @ 0x9e2a
unsigned int sub_9e2a(unsigned long *a0,unsigned long *a1) // ternary
{
  long v1;
  long v2;
  unsigned long v3;
  unsigned long v4;
  unsigned int v5; // eax
  unsigned long v6; // rax
  unsigned long v7; // rax
  unsigned long v8;
  unsigned int v9; // stack - 0x1c
  
  if ((dat_23688) && (((v5 = sub_9116(a0,a1), v5 || (dat_23682)) || (dat_23681))))
    return v5;
  v1 = a0[1];
  v6 = v1 - 1;
  v2 = a1[1];
  v7 = v2 - 1;
  if (v6) { // branch-flip
    if (v7) { // branch-flip
      if (dat_23229) { // branch-flip
        v3 = *a1;
        v4 = *a0;
        v9 = sub_187ec(v4,v1,v3,v2);
      }
      else {
        v8 = v7;
        if (v6 < v7)
          v8 = v6;
        v9 = memcmp((void *)*a0,(void *)*a1,v8);
        if (!v9) {
          v9 = (v7 <= v6) ? (unsigned int)(v6 != v7) : 0xffffffff; // branch-flip
        }
      }
    }
    else {
      v9 = 1;
    }
  }
  else {
    v9 = -(unsigned int)(v7 != 0);
  }
  if (dat_23680)
    v9 = -v9;
  return v9;
}

// Function: sub_9f73 @ 0x9f73
void sub_9f73(unsigned long *a0,void *a1,long a2) // return-dupe
{
  unsigned long v1;
  char *v2; // rax
  char v3; // stack - 0x29
  char *v4; // stack - 0x28
  
  v4 = (char *)*a0;
  v1 = a0[1];
  v2 = &v4[v1];
  if ((!a2) && (dat_23698)) {
    do {
      if (v2 <= v4) {
        sub_829e(a0);
        return;
      }
      v3 = *v4;
      if (v3 != '\t') { // branch-flip
        if (&v4[1] == v2)
          v3 = '\n';
      }
      else {
        v3 = '>';
      }
      v4 = &v4[1];
    } while (fputc_unlocked((int)v3,a1) != -1);
    sub_4464(gettext("write failed"),0); // no-return
  }
  v2[-1] = dat_23010;
  if (v1 == fwrite_unlocked(v4,1,v1,a1)) {
    v2[-1] = '\0';
    return;
  }
  sub_4464(gettext("write failed"),a2); // no-return
}

// Function: sub_a0b1 @ 0xa0b1
char sub_a0b1(unsigned long a0,char a1)
{
  long *v1;
  void *v10; // stack - 0x98
  char v11 [24];
  bool v12; // zf
  char v13; // stack - 0xda
  unsigned long v14; // stack - 0xd8
  long v15; // stack - 0xd0
  long *v16; // stack - 0xc8
  long v17; // stack - 0x90
  long v18; // stack - 0x88
  long v19; // stack - 0x80
  long *v2;
  long v20; // stack - 0x68
  long v3;
  unsigned long v4;
  unsigned long v5; // rax
  long *v6; // rax
  unsigned long v7; // rax
  unsigned long v8;
  void *v9 [2]; // stack - 0x78
  
  v5 = sub_505e(a0,"r");
  v3 = dat_23688;
  v14 = 0;
  v15 = 0;
  v12 = dat_23682 == '\0';
  v13 = 1;
  v8 = dat_23660;
  if (dat_23660 < dat_23018)
    v8 = dat_23018;
  sub_6618(v9,0x20,v8);
  v10 = NULL;
  do {
    if (!sub_6a5e(v9,v5,a0)) {
label_a446:
      sub_50b7(v5,a0);
      free(v9[0]);
      free(v10);
      return v13;
    }
    v16 = (long *)sub_66d0(v9);
    v6 = &v16[v20 * -4];
    if ((v14) && ((int)(unsigned int)v12 <= (int)sub_9e2a(&v10,&v16[-4]))) {
label_a1f4:
      if (a1 == 'c') {
        v7 = sub_1287e(v15 + (sub_66d0(v9) - (long)&v16[-4] >> 5),v11);
        v4 = dat_23868;
        fprintf(stderr,gettext("%s: %s:%s: disorder: "),v4,a0,v7);
        sub_9f73(&v16[-4],stderr,gettext("standard error"));
      }
      v13 = 0;
      goto label_a446;
    }
    while (v2 = &v16[-4], v6 < v2) {
      v1 = &v16[-8];
      v16 = v2;
      if ((int)(unsigned int)v12 <= (int)sub_9e2a(v2,v1)) goto label_a1f4;
    }
    v15 += v20;
    if (v14 < (unsigned long)v16[-3]) {
      do {
        v14 <<= 1;
        if (!v14) {
          v14 = v16[-3];
          break;
        }
      } while (v14 < (unsigned long)v16[-3]);
      free(v10);
      v10 = (void *)sub_18024(v14);
    }
    memcpy(v10,(void *)*v2,v16[-3]);
    v17 = v16[-3];
    if (v3) {
      v18 = (v16[-2] - *v2) + (long)v10;
      v19 = (v16[-1] - *v2) + (long)v10;
    }
  } while( true );
}

// Function: sub_a4a1 @ 0xa4a1
long sub_a4a1(long a0,unsigned long a1,long *a2)
{
  long v1;
  unsigned long v2; // rax
  int v3; // stack - 0x14
  
  *a2 = sub_1819d(a1,8);
  v1 = *a2;
  for (v3 = 0; (unsigned long)(long)v3 < a1; v3 = v3 + 1) {
    if ((*(long *)(a0 + (long)v3 * 0x10 + 8)) && (*(char *)(*(long *)(a0 + (long)v3 * 0x10 + 8) + 0xc)))
      v2 = sub_551c(*(unsigned long *)(a0 + (long)v3 * 0x10 + 8));
    else {
      v2 = sub_4ee4(*(unsigned long *)(a0 + (long)v3 * 0x10),"r");
    }
    *(unsigned long *)(v1 + (long)v3 * 8) = v2;
    if (!*(long *)(v1 + (long)v3 * 8)) break;
  }
  return (long)v3;
}

// Function: sub_a5c0 @ 0xa5c0
void sub_a5c0(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,void *a5)
{
  long *v1;
  long v10; // rax
  unsigned long *v11; // rax
  unsigned long *v12; // rcx
  unsigned long v13;
  void *v14; // stack - 0x48
  unsigned long v15; // stack - 0x100
  unsigned long v16; // stack - 0xf8
  void **v17; // stack - 0xd8
  unsigned long v18; // stack - 0xd0
  unsigned long v19; // stack - 0xc8
  unsigned long v2;
  unsigned long v20; // stack - 0xc0
  unsigned long v21; // stack - 0xb8
  unsigned long v22; // stack - 0xb0
  unsigned long v23; // stack - 0xa8
  unsigned long v24; // stack - 0x40
  long v25; // stack - 0x38
  long v26; // stack - 0x30
  unsigned long v3;
  long v4;
  int v5; // eax
  void *v6; // rax
  void *v7; // rax
  void *v8; // rax
  unsigned long *v9; // rax
  
  v6 = (void *)sub_1819d(a2,0x38);
  v17 = NULL;
  v18 = 0;
  v7 = (void *)sub_1819d(a2,8);
  v8 = (void *)sub_1819d(a2,8);
  v9 = (unsigned long *)sub_1819d(a2,8);
  v4 = dat_23688;
  v14 = NULL;
  v19 = 0;
  v15 = a2;
  v16 = a1;
  while (v19 < v15) {
    v13 = dat_23660 / v15;
    if (dat_23660 / v15 < dat_23018)
      v13 = dat_23018;
    sub_6618((void *)((long)v6 + v19 * 0x38),0x20,v13);
    v2 = *(unsigned long *)(a0 + v19 * 0x10);
    v3 = *(unsigned long *)((long)a5 + v19 * 8);
    if (sub_6a5e((void *)((long)v6 + v19 * 0x38),v3,v2)) { // branch-flip
      v10 = sub_66d0((void *)((long)v6 + v19 * 0x38));
      *(long *)((long)v7 + v19 * 8) = v10 + -0x20;
      *(long *)((long)v8 + v19 * 8) = v10 + *(long *)((long)v6 + v19 * 0x38 + 0x10) * -0x20;
      v19 += 1;
    }
    else {
      sub_50b7(*(unsigned long *)((long)a5 + v19 * 8),*(unsigned long *)(a0 + v19 * 0x10));
      if (v19 < v16) {
        v16 -= 1;
        sub_5782(*(unsigned long *)(a0 + v19 * 0x10));
      }
      free(*(void **)((long)v6 + v19 * 0x38));
      v15 -= 1;
      for (v20 = v19; v20 < v15; v20 = v20 + 1) {
        v11 = (unsigned long *)(a0 + (v20 + 1) * 0x10);
        v12 = (unsigned long *)(v20 * 0x10 + a0);
        v2 = v11[1];
        *v12 = *v11;
        v12[1] = v2;
        *(unsigned long *)((long)a5 + v20 * 8) = *(unsigned long *)((long)a5 + (v20 + 1) * 8);
      }
    }
  }
  for (v19 = 0; v19 < v15; v19 = v19 + 1) {
    v9[v19] = v19;
  }
  for (v19 = 1; v19 < v15; v19 = v19 + 1) {
    v2 = *(unsigned long *)((long)v7 + v9[v19] * 8);
    v3 = *(unsigned long *)((long)v7 + v9[v19 - 1] * 8);
    if (1 <= (int)sub_9e2a(v3,v2)) {
      v13 = v9[v19 - 1];
      v9[v19 - 1] = v9[v19];
      v9[v19] = v13;
      v19 = 0;
    }
  }
label_b27c:
  do {
    if (!v15) {
      if ((dat_23682) && (v17)) {
        sub_9f73(&v14,a3,a4);
        free(v14);
      }
      sub_50b7(a3,a4);
      free(a5);
      free(v6);
      free(v9);
      free(v8);
      free(v7);
      return;
    }
    v1 = *(long **)((long)v7 + *v9 * 8);
    if (dat_23682) { // branch-flip
      if ((v17) && (sub_9e2a(v17,v1))) {
        v17 = NULL;
        sub_9f73(&v14,a3,a4);
      }
      if (!v17) {
        v17 = &v14;
        if (v18 < (unsigned long)v1[1]) {
          do {
            if (!v18) {
              v18 = v1[1];
              break;
            }
            v18 <<= 1;
          } while (v18 < (unsigned long)v1[1]);
          free(v14);
          v14 = (void *)sub_18024(v18);
        }
        v24 = v1[1];
        memcpy(v14,(void *)*v1,v24);
        if (v4) {
          v25 = (v1[2] - *v1) + (long)v14;
          v26 = (v1[3] - *v1) + (long)v14;
        }
      }
    }
    else {
      sub_9f73(v1,a3,a4);
    }
    if (v1 <= *(long **)((long)v8 + *v9 * 8)) { // branch-flip
      v2 = *(unsigned long *)(a0 + *v9 * 0x10);
      v3 = *(unsigned long *)((long)a5 + *v9 * 8);
      v13 = *v9;
      if (!sub_6a5e((void *)((long)v6 + v13 * 0x38),v3,v2)) {
        for (v19 = 1; v19 < v15; v19 = v19 + 1) {
          if (*v9 < v9[v19])
            v9[v19] = v9[v19] - 1;
        }
        v15 -= 1;
        sub_50b7(*(unsigned long *)((long)a5 + *v9 * 8),*(unsigned long *)(a0 + *v9 * 0x10));
        if (*v9 < v16) {
          v16 -= 1;
          sub_5782(*(unsigned long *)(a0 + *v9 * 0x10));
        }
        free(*(void **)((long)v6 + *v9 * 0x38));
        for (v19 = *v9; v19 < v15; v19 = v19 + 1) {
          *(unsigned long *)((long)a5 + v19 * 8) = *(unsigned long *)((long)a5 + (v19 + 1) * 8);
          v11 = (unsigned long *)(a0 + (v19 + 1) * 0x10);
          v12 = (unsigned long *)(v19 * 0x10 + a0);
          v2 = v11[1];
          *v12 = *v11;
          v12[1] = v2;
          v12 = (unsigned long *)((v19 + 1) * 0x38 + (long)v6);
          v11 = (unsigned long *)((long)v6 + v19 * 0x38);
          v2 = v12[1];
          *v11 = *v12;
          v11[1] = v2;
          v2 = v12[3];
          v11[2] = v12[2];
          v11[3] = v2;
          v2 = v12[5];
          v11[4] = v12[4];
          v11[5] = v2;
          v11[6] = v12[6];
          *(unsigned long *)((long)v7 + v19 * 8) = *(unsigned long *)((long)v7 + (v19 + 1) * 8);
          *(unsigned long *)((long)v8 + v19 * 8) = *(unsigned long *)((long)v8 + (v19 + 1) * 8);
        }
        for (v19 = 0; v19 < v15; v19 = v19 + 1) {
          v9[v19] = v9[v19 + 1];
        }
        goto label_b27c;
      }
      v10 = sub_66d0((void *)((long)v6 + *v9 * 0x38));
      *(long *)((long)v7 + *v9 * 8) = v10 + -0x20;
      *(long *)((long)v8 + *v9 * 8) = v10 + *(long *)((long)v6 + *v9 * 0x38 + 0x10) * -0x20;
    }
    else {
      *(long **)((long)v7 + *v9 * 8) = &v1[-4];
    }
    v21 = 1;
    v22 = v15;
    v23 = 1;
    v13 = *v9;
    while (v21 < v22) {
      v5 = sub_9e2a(*(unsigned long *)((long)v7 + v13 * 8),*(unsigned long *)((long)v7 + v9[v23] * 8));
      if ((0 <= v5) && ((v5 || (v9[v23] <= v13))))
        v21 = v23 + 1;
      else {
        v22 = v23;
      }
      v23 = v22 + v21 >> 1;
    }
    for (v20 = 0; v20 < v21 - 1; v20 = v20 + 1) {
      v9[v20] = v9[v20 + 1];
    }
    v9[v21 - 1] = v13;
  } while( true );
}

// Function: sub_b344 @ 0xb344
unsigned long sub_b344(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1;
  unsigned long v2; // stack - 0x30
  unsigned long v3; // stack - 0x28
  
  v3 = sub_a4a1(a0,a2,&v2);
  if ((v3 < a2) && (v3 <= 1)) {
    v1 = *(unsigned long *)(a0 + v3 * 0x10);
    sub_4464(gettext("open failed"),v1); // no-return
  }
  sub_a5c0(a0,a1,v3,a3,a4,v2);
  return v3;
}

// Function: sub_b413 @ 0xb413
void sub_b413(unsigned long *a0,unsigned long a1,unsigned long *a2) // return-dupe
{
  unsigned long *v1;
  unsigned long v2;
  unsigned long *v3; // stack - 0x40
  unsigned long *v4; // stack - 0x30
  unsigned long v5; // stack - 0x20
  long v6; // stack - 0x18
  unsigned long *v7; // stack - 0x10
  
  v5 = a1 >> 1;
  v6 = a1 - v5;
  v3 = a2;
  v4 = a0;
  v7 = &a0[v5 * -4];
  do {
    while ((int)sub_9e2a(&v3[-4],&v7[-4]) <= 0) {
      v1 = &v4[-4];
      v2 = v3[-3];
      *v1 = v3[-4];
      v4[-3] = v2;
      v2 = v3[-1];
      v4[-2] = v3[-2];
      v4[-1] = v2;
      v5 -= 1;
      v3 = &v3[-4];
      v4 = v1;
      if (!v5)
        return;
    }
    v1 = &v4[-4];
    v2 = v7[-3];
    *v1 = v7[-4];
    v4[-3] = v2;
    v2 = v7[-1];
    v4[-2] = v7[-2];
    v4[-1] = v2;
    v6 -= 1;
    v4 = v1;
    v7 = &v7[-4];
  } while (v6);
  do {
    v1 = &v4[-4];
    v2 = v3[-3];
    *v1 = v3[-4];
    v4[-3] = v2;
    v2 = v3[-1];
    v4[-2] = v3[-2];
    v4[-1] = v2;
    v5 -= 1;
    v3 = &v3[-4];
    v4 = v1;
  } while (v5);
}

// Function: sub_b536 @ 0xb536
void sub_b536(long a0,unsigned long a1,long a2,char a3) // return-dupe x2
{
  unsigned long *v1;
  unsigned long v2;
  unsigned int v3; // eax
  unsigned long v4; // rax
  long v5; // rax
  long v6; // stack - 0x38
  long v7; // stack - 0x30
  
  if (a1 != 2) {
    v4 = a1 >> 1;
    if (a3) // branch-flip
      v5 = v4 * -0x20;
    else {
      v5 = 0;
    }
    sub_b536(a0 + v4 * -0x20,a1 - v4,v5 + a2,a3);
    if (2 <= v4) // branch-flip
      sub_b536(a0,v4,a2,a3 == '\0');
    else if (a3 != '\x01') {
      v2 = *(unsigned long *)(a0 + -0x18);
      *(unsigned long *)(a2 + -0x20) = *(unsigned long *)(a0 + -0x20);
      *(unsigned long *)(a2 + -0x18) = v2;
      v2 = *(unsigned long *)(a0 + -8);
      *(unsigned long *)(a2 + -0x10) = *(unsigned long *)(a0 + -0x10);
      *(unsigned long *)(a2 + -8) = v2;
    }
    if (a3) { // branch-flip
      v6 = a2;
      v7 = a0;
    }
    else {
      v6 = a0;
      v7 = a2;
    }
    sub_b413(v6,a1,v7);
    return;
  }
  v3 = (unsigned int)(0 < (int)sub_9e2a(a0 + -0x20,a0 + -0x40));
  if (a3) {
    v1 = (unsigned long *)((long)(int)~v3 * 0x20 + a0);
    v2 = v1[1];
    *(unsigned long *)(a2 + -0x20) = *v1;
    *(unsigned long *)(a2 + -0x18) = v2;
    v2 = v1[3];
    *(unsigned long *)(a2 + -0x10) = v1[2];
    *(unsigned long *)(a2 + -8) = v2;
    v1 = (unsigned long *)((long)(int)(v3 - 2) * 0x20 + a0);
    v2 = v1[1];
    *(unsigned long *)(a2 + -0x40) = *v1;
    *(unsigned long *)(a2 + -0x38) = v2;
    v2 = v1[3];
    *(unsigned long *)(a2 + -0x30) = v1[2];
    *(unsigned long *)(a2 + -0x28) = v2;
    return;
  }
  if (v3) {
    v2 = *(unsigned long *)(a0 + -0x18);
    *(unsigned long *)(a2 + -0x20) = *(unsigned long *)(a0 + -0x20);
    *(unsigned long *)(a2 + -0x18) = v2;
    v2 = *(unsigned long *)(a0 + -8);
    *(unsigned long *)(a2 + -0x10) = *(unsigned long *)(a0 + -0x10);
    *(unsigned long *)(a2 + -8) = v2;
    *(unsigned long *)(a0 + -0x20) = *(unsigned long *)(a0 + -0x40);
    *(unsigned long *)(a0 + -0x18) = *(unsigned long *)(a0 + -0x38);
    *(unsigned long *)(a0 + -0x10) = *(unsigned long *)(a0 + -0x30);
    *(unsigned long *)(a0 + -8) = *(unsigned long *)(a0 + -0x28);
    v2 = *(unsigned long *)(a2 + -0x18);
    *(unsigned long *)(a0 + -0x40) = *(unsigned long *)(a2 + -0x20);
    *(unsigned long *)(a0 + -0x38) = v2;
    v2 = *(unsigned long *)(a2 + -8);
    *(unsigned long *)(a0 + -0x30) = *(unsigned long *)(a2 + -0x10);
    *(unsigned long *)(a0 + -0x28) = v2;
    return;
  }
}

// Function: sub_b7b6 @ 0xb7b6
unsigned long * sub_b7b6(long a0,unsigned long a1,unsigned long a2)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_18024(a0 << 8);
  v1[3] = 0;
  v1[2] = v1[3];
  v1[1] = v1[2];
  *v1 = v1[1];
  v1[4] = 0;
  v1[6] = a1;
  v1[5] = v1[6];
  v1[7] = 0;
  *(unsigned int *)&v1[10] = 0;
  *(char *)((long)v1 + 0x54) = 0;
  pthread_mutex_init(&v1[0xb],0);
  sub_b911(v1,&v1[0x10],a2,a0,a1,0);
  return v1;
}

// Function: sub_b8b3 @ 0xb8b3
void sub_b8b3(long a0,void *a1)
{
  long v1;
  long v2; // stack - 0x18
  void *v3; // stack - 0x10
  
  v2 = a0 * 2;
  v3 = a1;
  while( true ) {
    v1 = v2 + -1;
    if (!v2) break;
    pthread_mutex_destroy((long)v3 + 0x58);
    v3 = (void *)((long)v3 + 0x80);
    v2 = v1;
  }
  free(a1);
}

// Function: sub_b911 @ 0xb911
unsigned long * sub_b911(long a0,unsigned long *a1,long a2,unsigned long a3,long a4,char a5) // ternary x2
{
  unsigned long v1;
  unsigned long v2; // rax
  long v3; // rax
  long v4; // rax
  unsigned long v5; // rax
  unsigned long *v6; // stack - 0x68
  
  v1 = (a5) ? *(unsigned long *)(a0 + 0x28) : *(unsigned long *)(a0 + 0x30); // branch-flip
  v2 = v1 >> 1;
  a2 += a4 * -0x20;
  v3 = a2 + v2 * -0x20;
  v4 = (a5) ? a0 + 0x10 : a0 + 0x18; // branch-flip
  v6 = &a1[0x10];
  a1[2] = a2;
  *a1 = a1[2];
  a1[3] = v3;
  a1[1] = a1[3];
  a1[4] = v4;
  a1[5] = v2;
  a1[6] = v1 - v2;
  a1[7] = a0;
  *(int *)&a1[10] = *(int *)(a0 + 0x50) + 1;
  *(char *)((long)a1 + 0x54) = 0;
  pthread_mutex_init(&a1[0xb],0);
  if (2 <= a3) { // branch-flip
    a1[8] = v6;
    v5 = sub_b911(a1,v6,a2,a3 >> 1,a4,1);
    a1[9] = v5;
    v6 = (unsigned long *)sub_b911(a1,v5,v3,a3 - (a3 >> 1),a4,0);
  }
  else {
    a1[8] = 0;
    a1[9] = 0;
  }
  return v6;
}

// Function: sub_bb0c @ 0xbb0c
bool sub_bb0c(long a0,long a1) // ternary
{
  bool v1; // cf
  
  v1 = (*(int *)(a0 + 0x50) != *(int *)(a1 + 0x50)) ? *(unsigned int *)(a0 + 0x50) < *(unsigned int *)(a1 + 0x50) : (unsigned long)(*(long *)(a0 + 0x28) + *(long *)(a0 + 0x30)) < (unsigned long)(*(long *)(a1 + 0x30) + *(long *)(a1 + 0x28)); // branch-flip
  return v1;
}

// Function: sub_bb87 @ 0xbb87
void sub_bb87(long a0)
{
  pthread_mutex_lock((void *)(a0 + 0x58));
}

// Function: sub_bba6 @ 0xbba6
void sub_bba6(long a0)
{
  pthread_mutex_unlock((void *)(a0 + 0x58));
}

// Function: sub_bbc5 @ 0xbbc5
void sub_bbc5(unsigned long *a0)
{
  sub_12398(*a0);
  pthread_cond_destroy(&a0[6]);
  pthread_mutex_destroy(&a0[1]);
}

// Function: sub_bc07 @ 0xbc07
void sub_bc07(unsigned long *a0,long a1)
{
  *a0 = sub_122ed(sub_bb0c,a1 * 2);
  pthread_mutex_init(&a0[1],0);
  pthread_cond_init(&a0[6],0);
}

// Function: sub_bc6b @ 0xbc6b
void sub_bc6b(unsigned long *a0,long a1)
{
  pthread_mutex_lock(&a0[1]);
  sub_123c6(*a0,a1);
  *(char *)(a1 + 0x54) = 1;
  pthread_cond_signal(&a0[6]);
  pthread_mutex_unlock(&a0[1]);
}

// Function: sub_bcd0 @ 0xbcd0
long sub_bcd0(unsigned long *a0)
{
  long v1; // rax
  
  pthread_mutex_lock(&a0[1]);
  while( true ) {
    v1 = sub_12474(*a0);
    if (v1) break;
    pthread_cond_wait(&a0[6],&a0[1]);
  }
  pthread_mutex_unlock(&a0[1]);
  sub_bb87(v1);
  *(char *)(v1 + 0x54) = 0;
  return v1;
}

// Function: sub_bd51 @ 0xbd51
void sub_bd51(long *a0,unsigned long a1,unsigned long a2)
{
  if (dat_23682) {
    if ((dat_23240) && (!sub_9e2a(a0,0x23240)))
      return;
    dat_23240 = *a0;
    dat_23248 = a0[1];
    dat_23250 = a0[2];
    dat_23258 = a0[3];
  }
  sub_9f73(a0,a1,a2);
}

// Function: sub_bde5 @ 0xbde5
void sub_bde5(long *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  long v1;
  long v10; // stack - 0x38
  unsigned long *v11; // stack - 0x30
  unsigned long *v2;
  long v3;
  long v4;
  long v5;
  unsigned long *v6;
  unsigned long v7;
  long v8;
  bool v9; // zf
  
  v3 = *a0;
  v4 = a0[1];
  v10 = (a1 >> (((char)(int)a0[10] + '\x01') * '\x02' & 0x3fU)) + 1;
  if (2 <= *(unsigned int *)&a0[10]) { // branch-flip
    v11 = *(unsigned long **)a0[4];
    while (((*a0 != a0[2] && (a0[1] != a0[3])) && (v1 = v10 + -1, v9 = v10 != 0, v10 = v1, v9))) {
      v1 = a0[1];
      v5 = *a0;
      if (1 <= (int)sub_9e2a(v5 + -0x20,v1 + -0x20)) { // branch-flip
        a0[1] = a0[1] + -0x20;
        v6 = (unsigned long *)a0[1];
        v2 = &v11[-4];
        v7 = v6[1];
        *v2 = *v6;
        v11[-3] = v7;
        v7 = v6[3];
        v11[-2] = v6[2];
        v11[-1] = v7;
        v11 = v2;
      }
      else {
        *a0 = *a0 + -0x20;
        v6 = (unsigned long *)*a0;
        v2 = &v11[-4];
        v7 = v6[1];
        *v2 = *v6;
        v11[-3] = v7;
        v7 = v6[3];
        v11[-2] = v6[2];
        v11[-1] = v7;
        v11 = v2;
      }
    }
    if (v4 - a0[1] >> 5 != a0[6]) { // branch-flip
      if (v3 - *a0 >> 5 == a0[5]) {
        while ((a0[1] != a0[3] && (v1 = v10 + -1, v10))) {
          a0[1] = a0[1] + -0x20;
          v6 = (unsigned long *)a0[1];
          v2 = &v11[-4];
          v7 = v6[1];
          *v2 = *v6;
          v11[-3] = v7;
          v7 = v6[3];
          v11[-2] = v6[2];
          v11[-1] = v7;
          v10 = v1;
          v11 = v2;
        }
      }
    }
    else {
      while ((*a0 != a0[2] && (v1 = v10 + -1, v10))) {
        *a0 = *a0 + -0x20;
        v6 = (unsigned long *)*a0;
        v2 = &v11[-4];
        v7 = v6[1];
        *v2 = *v6;
        v11[-3] = v7;
        v7 = v6[3];
        v11[-2] = v6[2];
        v11[-1] = v7;
        v10 = v1;
        v11 = v2;
      }
    }
    *(unsigned long **)a0[4] = v11;
  }
  else {
    while (((*a0 != a0[2] && (a0[1] != a0[3])) && (v1 = v10 + -1, v9 = v10 != 0, v10 = v1, v9))) {
      v5 = a0[1];
      v8 = *a0;
      if (1 <= (int)sub_9e2a(v8 + -0x20,v5 + -0x20)) { // branch-flip
        a0[1] = a0[1] + -0x20;
        sub_bd51(a0[1],a2,a3);
      }
      else {
        *a0 = *a0 + -0x20;
        sub_bd51(*a0,a2,a3);
      }
    }
    if (v4 - a0[1] >> 5 != a0[6]) { // branch-flip
      if (v3 - *a0 >> 5 == a0[5]) {
        while ((a0[1] != a0[3] && (v1 = v10 + -1, v10))) {
          a0[1] = a0[1] + -0x20;
          sub_bd51(a0[1],a2,a3);
          v10 = v1;
        }
      }
    }
    else {
      while ((*a0 != a0[2] && (v1 = v10 + -1, v10))) {
        *a0 = *a0 + -0x20;
        sub_bd51(*a0,a2,a3);
        v10 = v1;
      }
    }
  }
  a0[5] = a0[5] - (v3 - *a0 >> 5);
  a0[6] = a0[6] - (v4 - a0[1] >> 5);
}

// Function: sub_c28e @ 0xc28e
void sub_c28e(unsigned long a0,long *a1) // return-dupe x2
{
  bool v1;
  
  if (*(char *)((long)a1 + 0x54) == '\x01')
    return;
  if (*a1 != a1[2]) { // branch-flip
    if ((a1[1] != a1[3]) || (!a1[6]))
      v1 = 1;
    else {
      v1 = 0;
    }
  }
  else if ((a1[1] == a1[3]) || (a1[5]))
    v1 = 0;
  else {
    v1 = 1;
  }
  if (v1) {
    sub_bc6b(a0,a1);
    return;
  }
}

// Function: sub_c34c @ 0xc34c
void sub_c34c(unsigned long a0,long a1) // return-dupe
{
  if (2 <= *(unsigned int *)(a1 + 0x50)) {
    sub_bb87(*(unsigned long *)(a1 + 0x38));
    sub_c28e(a0,*(unsigned long *)(a1 + 0x38));
    sub_bba6(*(unsigned long *)(a1 + 0x38));
    return;
  }
  if (!(*(long *)(a1 + 0x30) + *(long *)(a1 + 0x28))) {
    sub_bc6b(a0,*(unsigned long *)(a1 + 0x38));
    return;
  }
}

// Function: sub_c3d7 @ 0xc3d7
void sub_c3d7(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  long v1; // rax
  
  while( true ) {
    v1 = sub_bcd0(a0);
    if (!*(int *)(v1 + 0x50)) break;
    sub_bde5(v1,a1,a2,a3);
    sub_c28e(a0,v1);
    sub_c34c(a0,v1);
    sub_bba6(v1);
  }
  sub_bba6(v1);
  sub_bc6b(a0,v1);
}

// Function: sub_c481 @ 0xc481
unsigned long sub_c481(unsigned long *a0)
{
  sub_c4e8(*a0,a0[1],a0[2],a0[3],a0[4],a0[5],a0[6]);
  return 0;
}

// Function: sub_c4e8 @ 0xc4e8
void sub_c4e8(long a0,unsigned long a1,long a2,long *a3,unsigned long a4,unsigned long a5,unsigned long a6) // return-dupe
{
  unsigned long v1;
  unsigned long v10; // stack - 0x40
  long v11; // stack - 0x38
  long v12; // stack - 0x30
  unsigned long v13; // stack - 0x28
  unsigned long v14; // stack - 0x20
  unsigned long v15; // stack - 0x18
  long v2; // stack - 0x48
  unsigned long v3; // stack - 0x80
  unsigned long v4; // stack - 0x78
  unsigned long v5; // stack - 0x70
  long v6; // stack - 0x68
  unsigned long v7; // stack - 0x60
  unsigned long v8; // stack - 0x58
  long v9; // stack - 0x50
  
  v1 = a6;
  v4 = a3[6] + a3[5];
  v5 = a1 >> 1;
  v6 = a1 - v5;
  v12 = a3[8];
  v15 = a6;
  v2 = a0;
  v10 = v5;
  v11 = a2;
  v13 = a4;
  v14 = a5;
  if (((2 <= a1) && (0x20000 <= v4)) && (!pthread_create(&v3,0,sub_c481,&v2))) {
    sub_c4e8(a3[5] * -0x20 + a0,v6,a2,a3[9],a4,a5,v1);
    pthread_join(v3,0);
    return;
  }
  v7 = a3[5];
  v8 = a3[6];
  v9 = a0 + a2 * -0x20;
  if (2 <= v8)
    sub_b536(v7 * -0x20 + a0,v8,(v7 >> 1) * -0x20 + v9,0);
  if (2 <= v7)
    sub_b536(a0,v7,v9,0);
  *a3 = a0;
  a3[1] = v7 * -0x20 + a0;
  a3[2] = v7 * -0x20 + a0;
  a3[3] = v7 * -0x20 + v8 * -0x20 + a0;
  sub_bc6b(a4,a3);
  sub_c3d7(a4,a2,a5,v1);
}

// Function: sub_c7fa @ 0xc7fa
void sub_c7fa(long a0,unsigned long a1,unsigned long a2,char *a3) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_c7fa
{
  char *v1;
  int v2; // eax
  long v3; // stack - 0xa8
  unsigned long v4; // stack - 0xc8
  bool v5;
  long v6; // stack - 0xc0
  unsigned long v7; // stack - 0xb8
  long *v8; // stack - 0xb0
  long v9; // stack - 0xa0
  
  v6 = 0;
  v7 = a1;
  do {
    if (a2 <= v7)
      return;
    v1 = *(char **)(a0 + v7 * 0x10);
    v5 = strcmp(v1,"-") == 0;
    if (((!a3) || (v1 = *(char **)(a0 + v7 * 0x10), strcmp(a3,v1))) || (v5)) {
      v8 = (long *)sub_4e8c();
      if (!v8)
        return;
      if (v5)
        v2 = fstat(0,&v3);
      else {
        v2 = stat(*(char **)(a0 + v7 * 0x10),&v3);
      }
      v5 = !(((v2) || (v9 != v8[1])) || (v3 != *v8));
    }
    else {
      v5 = 1;
    }
    if (v5) {
      if (!v6) {
        v6 = sub_54f9(&v4);
        sub_b344(v7 * 0x10 + a0,0,1,v4,v6 + 0xd);
      }
      *(long *)(a0 + v7 * 0x10) = v6 + 0xd;
      *(long *)(v7 * 0x10 + a0 + 8) = v6;
    }
    v7 += 1;
  } while( true );
}

// Function: sub_ca81 @ 0xca81
void sub_ca81(long a0,unsigned long a1)
{
  char *v1;
  unsigned long v2;
  unsigned long v3; // stack - 0x20
  
  v3 = 0;
  do {
    if (a1 <= v3)
      return;
    v1 = *(char **)(a0 + v3 * 8);
    if (strcmp(v1,"-")) {
      v2 = *(unsigned long *)(a0 + v3 * 8);
      if (euidaccess(v2,4)) {
        v2 = *(unsigned long *)(a0 + v3 * 8);
        sub_4464(gettext("cannot read"),v2); // no-return
      }
    }
    v3 += 1;
  } while( true );
}

// Function: sub_cb44 @ 0xcb44
void sub_cb44(char *a0) // return-dupe
{
  int v1; // eax
  
  if (!a0)
    return;
  v1 = open(a0,0x80041,0x1b6);
  if (0 <= v1) {
    sub_515d(v1,1);
    return;
  }
  sub_4464(gettext("open failed"),a0); // no-return
}

// Function: sub_cbb9 @ 0xcbb9
void sub_cbb9(long *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  long v1;
  long v10; // stack - 0x68
  long v11; // stack - 0x60
  unsigned long v12; // stack - 0x58
  unsigned long v13; // stack - 0x50
  unsigned long v14; // stack - 0x48
  long v15; // stack - 0x40
  unsigned long v16; // stack - 0x38
  long v17; // stack - 0x30
  unsigned long v18; // stack - 0x28
  unsigned long v2;
  unsigned long v3; // stack - 0x88
  long v4; // stack - 0x90
  unsigned long v5; // stack - 0xb0
  unsigned long v6; // stack - 0xa8
  long v7; // stack - 0x80
  unsigned long v8; // stack - 0x78
  unsigned long v9; // stack - 0x70
  
  v6 = a1;
  for (v5 = a2; dat_23024 < v5; v5 = v5 + (v8 - v7)) {
    v8 = 0;
    for (v7 = 0; (unsigned long)dat_23024 <= v5 - v7; v7 = v7 + v18) {
      v17 = sub_54f9(&v3);
      v2 = v6;
      if (dat_23024 <= v6)
        v2 = (unsigned long)dat_23024;
      v18 = sub_b344(&a0[v7 * 2],v2,dat_23024,v3,v17 + 0xd);
      v2 = v6;
      if (v18 <= v6)
        v2 = v18;
      v6 -= v2;
      a0[v8 * 2] = v17 + 0xd;
      a0[v8 * 2 + 1] = v17;
      v8 += 1;
    }
    v12 = v5 - v7;
    v13 = (unsigned long)dat_23024 - v8 % (unsigned long)dat_23024;
    if (v13 < v12) {
      v14 = (v12 - v13) + 1;
      v15 = sub_54f9(&v3);
      v2 = v6;
      if (v14 <= v6)
        v2 = v14;
      v16 = sub_b344(&a0[v7 * 2],v2,v14,v3,v15 + 0xd);
      v2 = v6;
      if (v16 <= v6)
        v2 = v16;
      v6 -= v2;
      a0[v8 * 2] = v15 + 0xd;
      v2 = v8 + 1;
      a0[v8 * 2 + 1] = v15;
      v7 += v16;
      v8 = v2;
    }
    memmove(&a0[v8 * 2],&a0[v7 * 2],(v5 - v7) * 0x10);
    v6 += v8;
  }
  sub_c7fa(a0,v6,v5,a3);
  do {
    v9 = sub_a4a1(a0,v5,&v4);
    if (v9 != v5) { // branch-flip
      if (v9 <= 2) {
        v1 = a0[v9 * 2];
        sub_4464(gettext("open failed"),v1); // no-return
      }
    }
    else {
      v10 = sub_4ee4(a3,"w");
      if (v10) {
        sub_a5c0(a0,v6,v5,v10,a3,v4);
        return;
      }
      if ((*__errno_location() != 0x18) || (v9 <= 2))
        sub_4464(gettext("open failed"),a3); // no-return
    }
    do {
      v9 -= 1;
      sub_50b7(*(unsigned long *)(v9 * 8 + v4),a0[v9 * 2]);
      v11 = sub_5372(&v3,3 <= v9);
    } while (!v11);
    v2 = v6;
    if (v9 <= v6)
      v2 = v9;
    sub_a5c0(a0,v2,v9,v3,v11 + 0xd,v4);
    v2 = v6;
    if (v9 <= v6)
      v2 = v9;
    *a0 = v11 + 0xd;
    a0[1] = v11;
    memmove(&a0[2],&a0[v9 * 2],(v5 - v9) * 0x10);
    v6 = (v6 - v2) + 1;
    v5 = (v5 - v9) + 1;
  } while( true );
}

// Function: sub_d0f9 @ 0xd0f9
void sub_d0f9(unsigned long *a0,long a1,long a2,unsigned long a3)
{
  bool v1;
  long v10; // stack - 0x110
  long v11; // stack - 0x108
  long v12; // stack - 0x100
  unsigned long v13; // stack - 0xf8
  long v14; // stack - 0xf0
  unsigned long *v15; // stack - 0xe8
  long v16; // stack - 0xe0
  unsigned long v17; // stack - 0xd8
  long v18; // stack - 0xd0
  long v19; // stack - 0xc8
  char v2; // al
  void *v20; // stack - 0xc0
  long v21; // stack - 0xb0
  unsigned long v22; // stack - 0xa8
  long v23; // stack - 0xa0
  long v24; // stack - 0x98
  char v25; // stack - 0x88
  unsigned long v3; // rax
  unsigned long v4; // stack - 0x120
  void *v5; // stack - 0xb8
  unsigned long v6; // stack - 0x118
  char v7 [104];
  long v8; // stack - 0x138
  unsigned long *v9; // stack - 0x130
  
  v10 = 0;
  v1 = 0;
  v23 = 0;
  v8 = a1;
  v9 = a0;
  do {
    if (!v8) {
label_d4c6:
      free(v5);
      if (!v1) {
        v15 = dat_23720;
        v20 = (void *)sub_1819d(v10,0x10);
        v16 = 0;
        for (; v15; v15 = (unsigned long *)*v15) {
          *(long *)((long)v20 + v16 * 0x10) = (long)v15 + 0xd;
          *(unsigned long **)((long)v20 + v16 * 0x10 + 8) = v15;
          v16 += 1;
        }
        sub_cbb9(v20,v10,v10,a2);
        free(v20);
      }
      sub_4b69();
      return;
    }
    v17 = *v9;
    v4 = sub_505e(v17,"r");
    if (2 <= a3) { // branch-flip
      v14 = 1;
      for (v13 = 1; v13 < a3; v13 = v13 << 1) {
        v14 += 1;
      }
      v12 = v14 << 5;
    }
    else {
      v12 = 0x30;
    }
    if (!v23) {
      v3 = sub_637c(&v4,1,v9,v8,v12);
      sub_6618(&v5,v12,v3);
    }
    v25 = '\0';
    v9 = &v9[1];
    v8 -= 1;
    while (v2 = sub_6a5e(&v5,v4,v17), v2) {
      if (((v25) && (v8)) && (v12 + 1U < (v23 - v21) - v22 * v12)) {
        v24 = v21;
        break;
      }
      dat_23240 = 0;
      v18 = sub_66d0(&v5);
      if (((v25) && (!v8)) && ((!v10 && (!v24)))) {
        sub_50b7(v4,v17);
        v6 = sub_505e(a2,"w");
        v1 = 1;
        v11 = a2;
      }
      else {
        v10 += 1;
        v11 = sub_54f9(&v6) + 0xd;
      }
      if (2 <= v22) { // branch-flip
        sub_bc07(v7,a3);
        v19 = sub_b7b6(a3,v22,v18);
        sub_c4e8(v18,a3,v22,v19 + 0x80,v7,v6,v11);
        sub_b8b3(a3,v19);
        sub_bbc5(v7);
      }
      else {
        sub_bd51(v18 + -0x20,v6,v11);
      }
      sub_50b7(v6,v11);
      if (v1) goto label_d4c6;
    }
    sub_50b7(v4,v17);
  } while( true );
}

// Function: sub_d5d8 @ 0xd5d8
void sub_d5d8(unsigned long a0)
{
  long v1; // rax
  long *v2; // stack - 0x18
  
  v1 = sub_18564(a0,0x48);
  for (v2 = (long *)0x23688; *v2; v2 = (long *)(*v2 + 0x40)) {
  }
  *v2 = v1;
  *(unsigned long *)(v1 + 0x40) = 0;
}

// Function: sub_d640 @ 0xd640
void sub_d640(unsigned long a0,char *a1)
{
  unsigned long v1; // rax
  char *v2; // rax
  
  v1 = sub_154a0(a0);
  v2 = gettext(a1);
  error(2,0,gettext("%s: invalid field specification %s"),v2,v1); // no-return
}

// Function: sub_d6a1 @ 0xd6a1
void sub_d6a1(unsigned long a0)
{
  error(2,0,gettext("options \'-%s\' are incompatible"),a0); // no-return
}

// Function: sub_d6de @ 0xd6de
void sub_d6de(void)
{
  char v1 [40];
  long v2; // fs_offset
  long v3; // stack - 0x40
  unsigned long v4; // stack - 0x10
  
  v4 = *(unsigned long *)(v2 + 0x28);
  v3 = dat_23688;
  while( true ) {
    if (!v3)
      return;
    if (2 <= (unsigned int)(unsigned char)(*(long *)(v3 + 0x20) != 0 | *(unsigned char *)(v3 + 0x33) | *(unsigned char *)(v3 + 0x38)) + (unsigned int)*(unsigned char *)(v3 + 0x32) + (unsigned int)*(unsigned char *)(v3 + 0x34) + (unsigned int)*(unsigned char *)(v3 + 0x35) + (unsigned int)*(unsigned char *)(v3 + 0x36)) break;
    v3 = *(long *)(v3 + 0x40);
  }
  *(char *)(v3 + 0x37) = 0;
  *(char *)(v3 + 0x31) = *(char *)(v3 + 0x37);
  *(char *)(v3 + 0x30) = *(char *)(v3 + 0x31);
  sub_839e(v3,v1);
  sub_d6a1(v1); // no-return
}

// Function: sub_d7de @ 0xd7de
unsigned long sub_d7de(unsigned long a0,long *a1,char *a2)
{
  unsigned long v1; // rax
  char *v2; // rax
  long v3; // stack - 0x28
  unsigned long v4; // stack - 0x30
  
  switch(sub_18b54(a0,&v4,10,&v3,0x1b1db)) {
    case 0:
    case 2:
      *a1 = v3;
      if (*a1 == v3)
        return v4;
label_d872:
      *a1 = -1;
      return v4;
    case 1:
    case 3:
      goto label_d872;
    case 4:
      if (a2) {
        v1 = sub_154a0(a0);
        v2 = gettext(a2);
        error(2,0,gettext("%s: invalid count at start of %s"),v2,v1); // no-return
      }
      return 0;
    default:
      return v4;
    
  }
}

// Function: sub_d8f9 @ 0xd8f9
void sub_d8f9(int a0)
{
  sub_4b8b();
  signal(a0,0);
  raise(a0);
}

// Function: sub_d929 @ 0xd929
char * sub_d929(char *a0,long a1,int a2)
{
  char *v1; // stack - 0x10
  
  for (v1 = a0; *v1; v1 = &v1[1]) {
    switch(*v1) {
      case 0x4d:
        *(char *)(a1 + 0x36) = 1;
        break;
      default:
        return v1;
      case 0x52:
        *(char *)(a1 + 0x33) = 1;
        break;
      case 0x56:
        *(char *)(a1 + 0x38) = 1;
        break;
      case 0x62:
        if ((!a2) || (a2 == 2))
          *(char *)(a1 + 0x30) = 1;
        if ((a2 == 1) || (a2 == 2))
          *(char *)(a1 + 0x31) = 1;
        break;
      case 100:
        *(unsigned long *)(a1 + 0x20) = 0x23460;
        break;
      case 0x66:
        *(unsigned long *)(a1 + 0x28) = 0x23560;
        break;
      case 0x67:
        *(char *)(a1 + 0x34) = 1;
        break;
      case 0x68:
        *(char *)(a1 + 0x35) = 1;
        break;
      case 0x69:
        if (!*(long *)(a1 + 0x20))
          *(unsigned long *)(a1 + 0x20) = 0x23360;
        break;
      case 0x6e:
        *(char *)(a1 + 0x32) = 1;
        break;
      case 0x72:
        *(char *)(a1 + 0x37) = 1;
      
    }
  }
  return v1;
}

// Function: sub_da55 @ 0xda55
void * sub_da55(void *a0)
{
  memset(a0,0,0x48);
  *(unsigned long *)((long)a0 + 0x10) = 0xffffffffffffffff;
  return a0;
}

// Function: sub_da8d @ 0xda8d
unsigned long sub_da8d(int a0,char **a1) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_da8d
{
  unsigned int v1;
  long *v10; // rax
  unsigned long v11;
  long v12;
  unsigned long v13;
  void *v14; // stack - 0xc8
  unsigned long v15 [4]; // stack - 0x238
  int v16; // stack - 0x1e8
  char v17 [80];
  char v18; // stack - 0x22
  long v19; // fs_offset
  char *v2;
  bool v20; // zf
  bool v21; // zf
  bool v22;
  char v23; // stack - 0x338
  char v24; // stack - 0x333
  int v25; // stack - 0x330
  long *v26; // stack - 0x328
  char *v27; // stack - 0x320
  char *v28; // stack - 0x318
  unsigned long v29; // stack - 0x310
  bool v3;
  unsigned long v30; // stack - 0x308
  unsigned long *v31; // stack - 0x300
  char *v32; // stack - 0x2f8
  char *v33; // stack - 0x2f0
  unsigned long v34; // stack - 0x2e8
  char *v35; // stack - 0x2e0
  unsigned long v36; // stack - 0x2d8
  unsigned long v37; // stack - 0x2d0
  unsigned long v38; // stack - 0x218
  unsigned long v39; // stack - 0x210
  bool v4;
  char v40; // stack - 0x208
  char v41; // stack - 0x207
  char v42; // stack - 0x206
  char v43; // stack - 0x205
  char v44; // stack - 0x204
  char v45; // stack - 0x203
  char v46; // stack - 0x202
  char v47; // stack - 0x201
  char v48; // stack - 0x200
  unsigned int v49; // stack - 0x1e4
  bool v5;
  unsigned long *v50; // stack - 0x1e0
  unsigned long v51; // stack - 0xc0
  unsigned long v52; // stack - 0xb8
  unsigned long v53; // stack - 0xb0
  unsigned long v54; // stack - 0xa8
  unsigned long v55; // stack - 0xa0
  unsigned long v56; // stack - 0x98
  unsigned long v57; // stack - 0x90
  unsigned long v58; // stack - 0x88
  unsigned long v59; // stack - 0x80
  char v6; // al
  unsigned long v60; // stack - 0x78
  unsigned long v61; // stack - 0x70
  unsigned long v62; // stack - 0x68
  unsigned long v63; // stack - 0x60
  unsigned long v64; // stack - 0x58
  unsigned long v65; // stack - 0x50
  unsigned long v66; // stack - 0x48
  unsigned int v67; // stack - 0x40
  char v68; // stack - 0x21
  long v69; // stack - 0x20
  bool v7; // al
  int v8;
  char *v9; // rax
  
  v69 = *(long *)(v19 + 0x28);
  v25 = 0;
  v23 = '\0';
  v5 = 0;
  v28 = NULL;
  v22 = 0;
  v29 = 0;
  v30 = 0;
  v20 = getenv("POSIXLY_CORRECT") != NULL;
  v8 = sub_133ab();
  if ((0x30db0 <= v8) && (v8 <= 0x31068))
    v4 = 0;
  else {
    v4 = 1;
  }
  v32 = NULL;
  v33 = NULL;
  sub_1345f(*a1);
  v9 = setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_4029(2);
  dat_23229 = sub_107aa(3);
  dat_2322a = sub_107aa(2);
  v10 = (long *)localeconv();
  dat_23220 = *(char *)*v10;
  if ((!dat_23220) || (*(char *)(*v10 + 1)))
    dat_23220 = '.';
  dat_23224 = (int)*(char *)v10[1];
  if ((dat_23224) && (*(char *)(v10[1] + 1)))
    dat_23228 = 1;
  if ((!dat_23224) || (*(char *)(v10[1] + 1)))
    dat_23224 = 0x80;
  dat_23683 = '\0';
  sub_594a();
  sigemptyset((void *)0x236a0);
  for (v34 = 0; v34 <= 10; v34 = v34 + 1) {
    sigaction(*(int *)(v34 * 4 + 0x1d100),NULL,&v14);
    if (v14 != (void *)0x1)
      sigaddset((void *)0x236a0,*(int *)(v34 * 4 + 0x1d100));
  }
  v14 = sub_d8f9;
  v51 = dat_236a0;
  v52 = dat_236a8;
  v53 = dat_236b0;
  v54 = dat_236b8;
  v55 = dat_236c0;
  v56 = dat_236c8;
  v57 = dat_236d0;
  v58 = dat_236d8;
  v59 = dat_236e0;
  v60 = dat_236e8;
  v61 = dat_236f0;
  v62 = dat_236f8;
  v63 = dat_23700;
  v64 = dat_23708;
  v65 = dat_23710;
  v66 = dat_23718;
  v67 = 0;
  for (v34 = 0; v34 <= 10; v34 = v34 + 1) {
    v1 = *(unsigned int *)(v34 * 4 + 0x1d100);
    if (sigismember(0x236a0,v1))
      sigaction(*(int *)(v34 * 4 + 0x1d100),&v14,NULL);
  }
  signal(0x11,0);
  sub_1a960(sub_4bd4);
  sub_da55(v15);
  v15[0] = 0xffffffffffffffff;
  v31 = (unsigned long *)sub_1819d((long)a0,8);
label_dee8:
  do {
    v16 = -1;
    if ((v25 != -1) && (((!v20 || (!v30)) || (((v4 && ((((!v23 && (a0 != optind)) && (*a1[optind] == '-')) && (a1[optind][1] == 'o')))) && ((a1[optind][2] || (a0 != optind + 1)))))))) {
      v25 = getopt_long(a0,a1,"-bcCdfghik:mMno:rRsS:t:T:uVy:z",(void *)0x22560,&v16);
      if (v25 != -1) {
        if (0x88 > v25) {
          if (1 <= v25) { // branch-flip
            v6 = (char)v25;
            switch(v25) {
              case 1:
                v26 = NULL;
                if (*optarg == '+') {
                  v7 = (((a0 != optind) && (*a1[optind] == '-')) && ((unsigned int)((int)a1[optind][1] - 0x30U) <= 9));
                  if ((!v7) || (v20))
                    v3 = 0;
                  else {
                    v3 = 1;
                  }
                  v21 = v3 || v4;
                  v3 = v3 || v4;
                  v4 = v21;
                  if (v3) {
                    v26 = (long *)sub_da55(v17);
                    v27 = (char *)sub_d7de(&optarg[1],v26,0);
                    if ((v27) && (*v27 == '.'))
                      v27 = (char *)sub_d7de(&v27[1],&v26[1],0);
                    if ((!*v26) && (!v26[1]))
                      *v26 = -1;
                    if ((v27) && (!*(char *)sub_d929(v27,v26,0))) {
                      if (v7) {
                        v8 = optind + 1;
                        v2 = a1[optind];
                        optind = v8;
                        v27 = (char *)sub_d7de(&v2[1],&v26[2],"invalid number after \'-\'");
                        if (!v27)
                          __assert_fail("s","src/sort.c",0x1161,"main"); // no-return
                        if (*v27 == '.')
                          v27 = (char *)sub_d7de(&v27[1],&v26[3],"invalid number after \'.\'");
                        if ((!v26[3]) && (v26[2]))
                          v26[2] = v26[2] + -1;
                        if (*(char *)sub_d929(v27,v26,1))
                          sub_d640(v2,"stray character in field spec"); // no-return
                      }
                      *(char *)((long)v26 + 0x39) = 1;
                      sub_d5d8(v26);
                    }
                    else {
                      v26 = NULL;
                    }
                  }
                }
                if (!v26) {
                  v13 = v30 + 1;
                  v31[v30] = optarg;
                  v30 = v13;
                }
                goto label_dee8;
              case 0x43:
              case 99:
label_e561:
                if ((v23) && (v25 != v23))
                  sub_d6a1("cC"); // no-return
                v23 = (char)v25;
                goto label_dee8;
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
label_e4c9:
                v18 = (char)v25;
                v68 = 0;
                sub_d929(&v18,v15,2);
                goto label_dee8;
              case 0x53:
                sub_5e0c(v16,(int)v6,optarg);
                goto label_dee8;
              case 0x54:
                sub_570f(optarg);
                goto label_dee8;
              case 0x6b:
                v10 = (long *)sub_da55(v17);
                v27 = (char *)sub_d7de(optarg,v10,"invalid number at field start");
                v12 = *v10;
                *v10 = v12 + -1;
                if (!v12)
                  sub_d640(optarg,"field number is zero"); // no-return
                if (*v27 == '.') {
                  v27 = (char *)sub_d7de(&v27[1],&v10[1],"invalid number after \'.\'");
                  v12 = v10[1];
                  v10[1] = v12 + -1;
                  if (!v12)
                    sub_d640(optarg,"character offset is zero"); // no-return
                }
                if ((!*v10) && (!v10[1]))
                  *v10 = -1;
                v27 = (char *)sub_d929(v27,v10,0);
                if (*v27 != ',') { // branch-flip
                  v10[2] = -1;
                  v10[3] = 0;
                }
                else {
                  v27 = (char *)sub_d7de(&v27[1],&v10[2],"invalid number after \',\'");
                  v12 = v10[2];
                  v10[2] = v12 + -1;
                  if (!v12)
                    sub_d640(optarg,"field number is zero"); // no-return
                  if (*v27 == '.')
                    v27 = (char *)sub_d7de(&v27[1],&v10[3],"invalid number after \'.\'");
                  v27 = (char *)sub_d929(v27,v10,1);
                }
                if (*v27)
                  sub_d640(optarg,"stray character in field spec"); // no-return
                sub_d5d8(v10);
                goto label_dee8;
              case 0x6d:
                v5 = 1;
                goto label_dee8;
              case 0x6f:
                if ((v33) && (v8 = strcmp(v33,optarg), v8))
                  error(2,0,gettext("multiple output files specified")); // no-return
                v33 = optarg;
                goto label_dee8;
              case 0x73:
                dat_23681 = 1;
                goto label_dee8;
              case 0x74:
                v24 = *optarg;
                if (!v24)
                  error(2,0,gettext("empty tab")); // no-return
                if (optarg[1]) {
                  v8 = strcmp(optarg,"\\0");
                  if (v8) {
                    v11 = sub_154a0(optarg);
                    error(2,0,gettext("multi-character tab %s"),v11); // no-return
                  }
                  v24 = '\0';
                }
                if ((dat_23020 != 0x80) && (v24 != dat_23020))
                  error(2,0,gettext("incompatible tabs")); // no-return
                dat_23020 = (int)v24;
                goto label_dee8;
              case 0x75:
                dat_23682 = 1;
                goto label_dee8;
              case 0x79:
                if (a1[(long)optind + -1] == optarg) {
                  for (v35 = optarg; (unsigned int)((int)*v35 - 0x30U) <= 9; v35 = &v35[1]) {
                  }
                  optind -= (unsigned int)(*v35 != '\0');
                }
                goto label_dee8;
              case 0x7a:
                dat_23010 = 0;
                goto label_dee8;
              case 0x80:
                if (optarg) { // branch-flip
                  v12 = sub_f8c1("--check",optarg,0x22940,"CCcgeneral-numeric",1,dat_23110,1);
                  v25 = (int)"CCcgeneral-numeric"[v12];
                }
                else {
                  v25 = 99;
                }
                goto label_e561;
              case 0x81:
                if ((dat_23690) && (v8 = strcmp(dat_23690,optarg), v8))
                  error(2,0,gettext("multiple compress programs specified")); // no-return
                dat_23690 = optarg;
                goto label_dee8;
              case 0x82:
                dat_23698 = '\x01';
                goto label_dee8;
              case 0x83:
                v32 = optarg;
                goto label_dee8;
              case 0x84:
                sub_5ba8(v16,(int)v6,optarg);
                goto label_dee8;
              case 0x85:
                if ((v28) && (v8 = strcmp(v28,optarg), v8))
                  error(2,0,gettext("multiple random sources specified")); // no-return
                v28 = optarg;
                goto label_dee8;
              case 0x86:
                v12 = sub_f8c1("--sort",optarg,0x22960,"ghMnRVwaiting for %s [-d]",1,dat_23110,1);
                v25 = (int)"ghMnRVwaiting for %s [-d]"[v12];
                goto label_e4c9;
              case 0x87:
                v29 = sub_6042(v16,(int)v6,optarg);
                goto label_dee8;
              
            }
          }
          else {
            if (v25 == -0x83) {
              sub_17e44(stdout,"sort","GNU coreutils",dat_23108,"Mike Haertel","Paul Eggert",0);
              exit(0); // no-return
            }
            if (v25 == -0x82)
              sub_44d3(0); // no-return
          }
        }
        sub_44d3(2); // no-return
      }
    }
    if (a0 <= optind) {
      if (v32) {
        if (v30) {
          v11 = sub_15059(4,*v31);
          error(0,0,gettext("extra operand %s"),v11); // no-return
        }
        v11 = sub_505e(v32,"r");
        sub_1676c(&v16);
        if (sub_16b8b(v11,&v16) != '\x01') {
          v11 = sub_15059(4,v32);
          error(2,0,gettext("cannot read file names from %s"),v11); // no-return
        }
        sub_50b7(v11,v32);
        if (!CONCAT44(v49,v16)) {
          v11 = sub_15059(4,v32);
          error(2,0,gettext("no input from %s"),v11); // no-return
        }
        free(v31);
        v31 = v50;
        v30 = CONCAT44(v49,v16);
        for (v36 = 0; v36 < v30; v36 = v36 + 1) {
          v2 = (char *)v50[v36];
          if (!strcmp(v2,"-")) {
            v11 = sub_15059(4,v50[v36]);
            error(2,0,gettext("when reading file names from stdin, no file name of %s allowed"),v11); // no-return
          }
          if (!*(char *)v50[v36]) {
            v11 = sub_151eb(0,3,v32);
            error(2,0,gettext("%s:%lu: invalid zero-length file name"),v11,v36 + 1); // no-return
          }
        }
      }
      for (v26 = (long *)dat_23688; v26; v26 = *(long **)((long)v26 + 0x40)) {
        if ((sub_8305(v26)) && (*(char *)((long)v26 + 0x37) != '\x01')) {
          *(unsigned long *)((long)v26 + 0x20) = v38;
          *(unsigned long *)((long)v26 + 0x28) = v39;
          *(char *)((long)v26 + 0x30) = v40;
          *(char *)((long)v26 + 0x31) = v41;
          *(char *)((long)v26 + 0x36) = v46;
          *(char *)((long)v26 + 0x32) = v42;
          *(char *)((long)v26 + 0x34) = v44;
          *(char *)((long)v26 + 0x35) = v45;
          *(char *)((long)v26 + 0x38) = v48;
          *(char *)((long)v26 + 0x33) = v43;
          *(char *)((long)v26 + 0x37) = v47;
        }
        v22 = *(char *)((long)v26 + 0x33) || v22;
      }
      if ((!dat_23688) && (sub_8305(v15) != '\x01')) {
        sub_d5d8(v15);
        v22 = v43 || v22;
      }
      sub_d6de();
      if (!dat_23698) {
        dat_23680 = v47;
        if (v22)
          sub_7573(v28);
        if (!dat_23670) {
          v9 = getenv("TMPDIR");
          if (!v9)
            v9 = "/tmp";
          sub_570f(v9);
        }
        if (!v30) {
          v30 = 1;
          free(v31);
          v31 = (unsigned long *)sub_18024(8);
          *v31 = "-";
        }
        if (dat_23660) {
          if (dat_23660 <= (unsigned long)dat_23024 * 0x22)
            dat_23660 = (unsigned long)dat_23024 * 0x22;
        }
        if (!v23) {
          sub_ca81(v31,v30);
          sub_cb44(v33);
          if (v5) {
            v12 = sub_18502(v30,0x10);
            for (v37 = 0; v37 < v30; v37 = v37 + 1) {
              *(unsigned long *)(v12 + v37 * 0x10) = v31[v37];
            }
            sub_cbb9(v12,0,v30,v33);
          }
          else {
            if ((!v29) && (v29 = sub_12de4(2), 9 <= v29))
              v29 = 8;
            v13 = 0xffffffffffffff;
            if (v29 <= 0xffffffffffffff)
              v13 = v29;
            sub_d0f9(v31,v30,v33,v13);
          }
          if ((dat_23683) && (v8 = sub_ff17(stdin), v8 == -1))
            sub_4464(gettext("close failed"),"-"); // no-return
          if (v69 == *(long *)(v19 + 0x28))
            return 0;
          __stack_chk_fail(); // no-return
        }
        if (2 <= v30) {
          v11 = sub_15059(4,v31[1]);
          error(2,0,gettext("extra operand %s not allowed with -%c"),v11,(unsigned long)(unsigned int)(int)v23); // no-return
        }
        if (!v33) {
          v11 = *v31;
          exit((sub_a0b1(v11,(int)v23) ^ 1) & 0xff); // no-return
        }
        dat_23032 = v23;
        sub_d6a1(0x23032); // no-return
      }
      if ((v23) || (v33)) {
        if (!v23)
          v23 = 'o';
        s_23028[0] = v23;
        sub_d6a1(s_23028); // no-return
      }
      v22 = 0;
      if (v9)
        v22 = setlocale(3,"") != NULL;
      if (v22) {
        if (!dat_23229)
          error(0,0,"%s",gettext("text ordering performed using simple byte comparison")); // no-return
        v11 = sub_154a0(setlocale(3,NULL));
        error(0,0,gettext("text ordering performed using %s sorting rules"),v11); // no-return
      }
      error(0,0,"%s",gettext("failed to set locale")); // no-return
    }
    v8 = optind + 1;
    v13 = v30 + 1;
    v31[v30] = a1[optind];
    optind = v8;
    v30 = v13;
  } while( true );
}

// Function: sub_f513 @ 0xf513
void sub_f513(void)
{
  sub_44d3(1); // no-return
}

// Function: sub_f528 @ 0xf528
long sub_f528(char *a0,long a1,long a2,unsigned long a3) // early-return
{
  char *v1;
  bool v2;
  unsigned long v3; // rax
  long v4; // stack - 0x20
  long v5; // stack - 0x18
  
  v5 = -1;
  v2 = 0;
  v3 = strlen(a0);
  v4 = 0;
  do {
    if (!*(long *)(a1 + v4 * 8)) {
      if (v2)
        return -2;
      return v5;
    }
    v1 = *(char **)(a1 + v4 * 8);
    if (!strncmp(v1,a0,v3)) {
      v1 = *(char **)(a1 + v4 * 8);
      if (v3 == strlen(v1))
        return v4;
      if (v5 != -1) { // branch-flip
        if ((!a2) || (memcmp((void *)(v5 * a3 + a2),(void *)(a3 * v4 + a2),a3)))
          v2 = 1;
      }
      else {
        v5 = v4;
      }
    }
    v4 += 1;
  } while( true );
}

// Function: sub_f65b @ 0xf65b
long sub_f65b(char *a0,long a1)
{
  char *v1;
  long v2; // stack - 0x10
  
  v2 = 0;
  while( true ) {
    if (!*(long *)(a1 + v2 * 8))
      return -1;
    v1 = *(char **)(a1 + v2 * 8);
    if (!strcmp(v1,a0)) break;
    v2 += 1;
  }
  return v2;
}

// Function: sub_f6d1 @ 0xf6d1
void sub_f6d1(unsigned long a0,unsigned long a1,long a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (a2 != -1) // branch-flip
    v1 = gettext("ambiguous argument %s for %s");
  else {
    v1 = gettext("invalid argument %s for %s");
  }
  v2 = sub_15473(1,a0);
  error(0,0,v1,sub_14f8b(0,8,a1),v2); // no-return
}

// Function: sub_f76a @ 0xf76a
void sub_f76a(long a0,long a1,unsigned long a2)
{
  unsigned long v1;
  void *v2;
  long v3; // stack - 0x28
  void *v4; // stack - 0x20
  
  v2 = stderr;
  v4 = NULL;
  fputs_unlocked(gettext("Valid arguments are:"),v2);
  v3 = 0;
  do {
    if (!*(long *)(a0 + v3 * 8)) {
      putc_unlocked(10,stderr);
      return;
    }
    if (v3) { // branch-flip
      if (memcmp(v4,(void *)(a2 * v3 + a1),a2)) goto label_f7eb;
      v1 = *(unsigned long *)(a0 + v3 * 8);
      fprintf(stderr,", %s",sub_154a0(v1));
    }
    else {
label_f7eb:
      v1 = *(unsigned long *)(a0 + v3 * 8);
      fprintf(stderr,"\n  - %s",sub_154a0(v1));
      v4 = (void *)(a1 + a2 * v3);
    }
    v3 += 1;
  } while( true );
}

// Function: sub_f8c1 @ 0xf8c1
long sub_f8c1(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,void *a5,unsigned int a6) // early-return, ternary
{
  long v1; // stack - 0x10
  
  v1 = ((char)a6) ? sub_f528(a1,a2,a3,a4) : sub_f65b(a1,a2); // branch-flip
  if (0 <= v1)
    return v1;
  sub_f6d1(a0,a1,v1);
  sub_f76a(a2,a3,a4);
  (*a5)();
  return -1;
}

// Function: sub_f970 @ 0xf970
unsigned long sub_f970(void *a0,long a1,long a2,unsigned long a3)
{
  long v1; // stack - 0x10
  
  v1 = 0;
  while( true ) {
    if (!*(long *)(a1 + v1 * 8))
      return 0;
    if (!memcmp(a0,(void *)(a3 * v1 + a2),a3)) break;
    v1 += 1;
  }
  return *(unsigned long *)(a1 + v1 * 8);
}

// Function: sub_fa00 @ 0xfa00
void sub_fa00(unsigned long a0)
{
  dat_23858 = a0;
}

// Function: sub_fa1a @ 0xfa1a
void sub_fa1a(char a0)
{
  dat_23860 = a0;
}

// Function: sub_fa34 @ 0xfa34
void sub_fa34(void)
{
  int v1; // eax
  char *v2; // rax
  unsigned long v3; // rax
  
  v1 = sub_1948b(stdout);
  if (v1) {
    if (dat_23860 == '\x01') {
      if (*__errno_location() == 0x20) goto label_fb05;
    }
    v2 = gettext("write error");
    if (dat_23858) {
      v3 = sub_1519a(dat_23858);
      error(0,*__errno_location(),"%s: %s",v3,v2); // no-return
    }
    error(0,*__errno_location(),"%s",v2); // no-return
  }
label_fb05:
  v1 = sub_1948b(stderr);
  if (v1)
    _exit(dat_23118); // no-return
}

// Function: sub_fb2c @ 0xfb2c
void sub_fb2c(unsigned long a0)
{
  MD5_Init(a0);
}

// Function: sub_fb4b @ 0xfb4b
void sub_fb4b(unsigned long a0,unsigned long a1,unsigned long a2)
{
  MD5_Update(a2,a0,a1);
}

// Function: sub_fb7d @ 0xfb7d
void sub_fb7d(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_fb4b(a0,a1,a2);
}

// Function: sub_fbaf @ 0xfbaf
unsigned long sub_fbaf(unsigned long a0,unsigned long a1)
{
  MD5_Final(a1,a0);
  return a1;
}

// Function: sub_fbdc @ 0xfbdc
void sub_fbdc(unsigned long a0,unsigned long a1,unsigned long a2)
{
  MD5(a0,a1,a2);
}

// Function: sub_fc0d @ 0xfc0d
unsigned long sub_fc0d(unsigned long *a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x88
  unsigned long v10; // stack - 0x40
  unsigned long v11; // stack - 0x38
  unsigned int v12; // stack - 0x30
  unsigned long v2; // stack - 0x80
  unsigned long v3; // stack - 0x78
  unsigned long v4; // stack - 0x70
  unsigned long v5; // stack - 0x68
  unsigned long v6; // stack - 0x60
  unsigned long v7; // stack - 0x58
  unsigned long v8; // stack - 0x50
  unsigned long v9; // stack - 0x48
  
  v1 = *a0;
  v2 = a0[1];
  v3 = a0[2];
  v4 = a0[3];
  v5 = a0[4];
  v6 = a0[5];
  v7 = a0[6];
  v8 = a0[7];
  v9 = a0[8];
  v10 = a0[9];
  v11 = a0[10];
  v12 = *(unsigned int *)&a0[0xb];
  MD5_Final(a1,&v1);
  return a1;
}

// Function: sub_fcd5 @ 0xfcd5
unsigned long sub_fcd5(void)
{
  return 0xffffff9f;
}

// Function: sub_fcf0 @ 0xfcf0
unsigned long sub_fcf0(void *a0,unsigned long a1) // return-dupe x2
{
  int v1; // eax
  void *v2; // rax
  long v3; // rax
  char v4 [104];
  long v5; // stack - 0x90
  
  v1 = sub_fcd5(a0,"md5",a1,0x10);
  if (v1 == -5)
    return 1;
  if (!v1)
    return 0;
  v2 = malloc(0x8048);
  if (!v2)
    return 1;
  sub_fb2c(v4);
  do {
    v5 = 0;
    while( true ) {
      if (feof_unlocked(a0)) goto label_fe42;
      v3 = fread_unlocked((long)v2 + v5,1,0x8000 - v5,a0);
      v5 += v3;
      if (v5 == 0x8000) break;
      if (!v3) {
        if (ferror_unlocked(a0)) {
          free(v2);
          return 1;
        }
label_fe42:
        if (v5)
          sub_fb4b(v2,v5,v4);
        sub_fbaf(v4,a1);
        free(v2);
        return 0;
      }
    }
    sub_fb7d(v2,0x8000,v4);
  } while( true );
}

// Function: sub_fea3 @ 0xfea3
void sub_fea3(unsigned int a0,unsigned long a1,unsigned long a2,unsigned int a3)
{
  posix_fadvise(a0,a1,a2,a3);
}

// Function: sub_fed8 @ 0xfed8
void sub_fed8(void *a0,unsigned int a1) // return-dupe
{
  if (!a0)
    return;
  sub_fea3(fileno(a0),0,0,a1);
}

// Function: sub_ff17 @ 0xff17
unsigned long sub_ff17(void *a0)
{
  int v1; // eax
  int v2; // stack - 0x14
  unsigned int v3; // stack - 0x10
  
  v2 = 0;
  if (fileno(a0) <= -1)
    return CONCAT44(dat_4,fclose(a0));
  if (__freading(a0)) {
    v1 = fileno(a0);
    if (lseek(v1,0,1) == -1) goto label_ffa5;
  }
  if (sub_10008(a0))
    v2 = *__errno_location();
label_ffa5:
  v3 = fclose(a0);
  if (v2) {
    *__errno_location() = v2;
    v3 = 0xffffffff;
  }
  return (unsigned long)v3;
}

// Function: sub_ffd0 @ 0xffd0
void sub_ffd0(unsigned int *a0) // return-dupe
{
  if (!(*a0 & 0x100))
    return;
  sub_106ec(a0,0,1);
}

// Function: sub_10008 @ 0x10008
void sub_10008(void *a0)
{
  if ((a0) && (__freading(a0))) {
    sub_ffd0(a0);
    fflush(a0);
    return;
  }
  fflush(a0);
}

// Function: sub_10057 @ 0x10057
unsigned long sub_10057(long a0,unsigned long *a1) // ternary
{
  char v1;
  unsigned long v2;
  unsigned long v3;
  bool v4;
  unsigned long v5; // stack - 0x20
  
  v2 = *a1;
  v5 = 0;
  v3 = v5;
  while( true ) {
    while (((v5 + 1 < v2 && (*(char *)(a0 + v5) == '.')) && ((v1 = *(char *)(a0 + v5 + 1), sub_191b0((int)v1) || (*(char *)(a0 + v5 + 1) == '~'))))) {
      for (v5 = v5 + 2; (v5 < v2 && ((v1 = *(char *)(a0 + v5), sub_19172((int)v1) || (*(char *)(a0 + v5) == '~')))); v5 = v5 + 1) {
      }
    }
    v4 = (0 <= (long)*a1) ? v5 == v2 : *(char *)(a0 + v5) == '\0'; // branch-flip
    if (v4) break;
    v5 += 1;
    v3 = v5;
  }
  *a1 = v5;
  return v3;
}

// Function: sub_10178 @ 0x10178
unsigned int sub_10178(long a0,long a1,long a2) // early-return x3
{
  unsigned char v1;
  unsigned int v2; // eax
  
  if (a1 == a2)
    return 0xffffffff;
  v1 = *(unsigned char *)(a0 + a1);
  if (sub_19258(v1))
    return 0;
  if (sub_191b0(v1)) // branch-flip
    v2 = (unsigned int)v1;
  else {
    if (v1 == 0x7e)
      return 0xfffffffe;
    v2 = v1 + 0x100;
  }
  return v2;
}

// Function: sub_101f5 @ 0x101f5
int sub_101f5(long a0,long a1,long a2,long a3) // early-return x3
{
  char v1;
  int v2; // eax
  int v3; // eax
  int v4; // stack - 0x24
  long v5; // stack - 0x18
  long v6; // stack - 0x10
  
  v5 = 0;
  v6 = 0;
  while( true ) {
    if ((a1 <= v5) && (a3 <= v6))
      return 0;
    v4 = 0;
    while (((v5 < a1 && (v1 = *(char *)(a0 + v5), sub_19258((int)v1) != '\x01')) || ((v6 < a3 && (v1 = *(char *)(a2 + v6), sub_19258((int)v1) != '\x01'))))) {
      v2 = sub_10178(a0,v5,a1);
      v3 = sub_10178(a2,v6,a3);
      if (v2 != v3)
        return v2 - v3;
      v5 += 1;
      v6 += 1;
    }
    for (; (v5 < a1 && (*(char *)(a0 + v5) == '0')); v5 = v5 + 1) {
    }
    for (; (v6 < a3 && (*(char *)(a2 + v6) == '0')); v6 = v6 + 1) {
    }
    while ((((v5 < a1 && (v6 < a3)) && (v1 = *(char *)(a0 + v5), sub_19258((int)v1))) && (v1 = *(char *)(a2 + v6), sub_19258((int)v1)))) {
      if (!v4)
        v4 = (int)*(char *)(a0 + v5) - (int)*(char *)(a2 + v6);
      v5 += 1;
      v6 += 1;
    }
    if ((v5 < a1) && (v1 = *(char *)(a0 + v5), sub_19258((int)v1))) break;
    if ((v6 < a3) && (v1 = *(char *)(a2 + v6), sub_19258((int)v1)))
      return -1;
    if (v4)
      return v4;
  }
  return 1;
}

// Function: sub_1042d @ 0x1042d
void sub_1042d(unsigned long a0,unsigned long a1)
{
  sub_10461(a0,0xffffffffffffffff,a1,0xffffffffffffffff);
}

// Function: sub_10461 @ 0x10461
unsigned long sub_10461(char *a0,long a1,char *a2,long a3) // early-return x5, ternary x4
{
  unsigned long v1; // rax
  char v10; // stack - 0x21
  char v11; // stack - 0x20
  char v12; // stack - 0x1f
  unsigned char v13; // stack - 0x1e
  char v14; // stack - 0x1d
  unsigned int v15; // stack - 0x1c
  long v16; // stack - 0x18
  long v17; // stack - 0x10
  long v2; // stack - 0x38
  long v3; // stack - 0x48
  bool v4; // al
  bool v5; // al
  char *v6; // stack - 0x40
  char *v7; // stack - 0x30
  char v8; // stack - 0x23
  unsigned char v9; // stack - 0x22
  
  v8 = (0 <= a1) ? a1 == 0 : *a0 == '\0'; // branch-flip
  v9 = (0 <= a3) ? a3 == 0 : *a2 == '\0'; // branch-flip
  if ((bool)v8)
    return (unsigned long)-(unsigned int)(v9 ^ 1);
  if ((bool)v9)
    return 1;
  if (*a0 != '.') {
    if (*a2 == '.')
      return 1;
    goto label_10651;
  }
  if (*a2 != '.')
    return 0xffffffff;
  v4 = (0 <= a1) ? a1 == 1 : a0[1] == '\0'; // branch-flip
  v5 = (0 <= a3) ? a3 == 1 : a2[1] == '\0'; // branch-flip
  if (v4)
    return (unsigned long)-(unsigned int)(v5 ^ 1);
  if (v5)
    return 1;
  if (a0[1] != '.') { // branch-flip
label_105c3:
    v4 = 0;
  }
  else {
    if (0 <= a1) { // branch-flip
      if (a1 != 2) goto label_105c3;
    }
    else if (a0[2]) goto label_105c3;
    v4 = 1;
  }
  if (a2[1] != '.') { // branch-flip
label_10609:
    v13 = 0;
  }
  else {
    if (0 <= a3) { // branch-flip
      if (a3 != 2) goto label_10609;
    }
    else if (a2[2]) goto label_10609;
    v13 = 1;
  }
  if (v4)
    return (unsigned long)-(unsigned int)(v13 ^ 1);
  v10 = 0;
  v11 = 0;
  v12 = 0;
  if (v13)
    return 1;
label_10651:
  v3 = a3;
  v6 = a2;
  v2 = a1;
  v7 = a0;
  v16 = sub_10057(a0,&v2);
  v17 = sub_10057(v6,&v3);
  if ((v16 != v2) || (v17 != v3))
    v14 = '\0';
  else {
    v14 = '\x01';
  }
  v15 = sub_101f5(v7,v16,v6,v17);
  if ((v15) || (v14))
    v1 = (unsigned long)v15;
  else {
    v1 = sub_101f5(v7,v2,v6,v3);
  }
  return v1;
}

// Function: sub_106ec @ 0x106ec
unsigned long sub_106ec(unsigned int *a0,unsigned long a1,unsigned int a2) // early-return x2
{
  long v1; // rax
  
  if (((*(long *)&a0[4] == *(long *)&a0[2]) && (*(long *)&a0[10] == *(long *)&a0[8])) && (!*(long *)&a0[0x12])) {
    v1 = lseek(fileno(a0),a1,a2);
    if (v1 == -1)
      return 0xffffffff;
    *a0 = *a0 & 0xffffffef;
    *(long *)&a0[0x24] = v1;
    return 0;
  }
  return fseeko(a0,a1,a2);
}

// Function: sub_107aa @ 0x107aa
unsigned int sub_107aa(unsigned int a0) // return-dupe
{
  char v1 [264];
  
  if (sub_16fdf(a0,v1,0x101))
    return 0;
  if ((strcmp(v1,"C")) && (strcmp(v1,"POSIX")))
    return 1;
  return 0;
}

// Function: sub_10854 @ 0x10854
unsigned long sub_10854(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}

// Function: sub_1086a @ 0x1086a
unsigned long sub_1086a(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}

// Function: sub_10880 @ 0x10880
unsigned long sub_10880(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}

// Function: sub_10896 @ 0x10896
unsigned long sub_10896(unsigned long *a0)
{
  long *v1; // stack - 0x28
  unsigned long v2; // stack - 0x20
  long *v3; // stack - 0x18
  unsigned long v4; // stack - 0x10
  
  v2 = 0;
  for (v1 = (long *)*a0; v1 < (long *)a0[1]; v1 = &v1[2]) {
    if (*v1) {
      v3 = v1;
      v4 = 1;
      while (v3 = (long *)v3[1], v3) {
        v4 += 1;
      }
      if (v2 < v4)
        v2 = v4;
    }
  }
  return v2;
}

// Function: sub_10918 @ 0x10918
unsigned long sub_10918(unsigned long *a0) // return-dupe
{
  long *v1; // stack - 0x28
  long v2; // stack - 0x20
  long v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = 0;
  for (v1 = (long *)*a0; v1 < (long *)a0[1]; v1 = &v1[2]) {
    if (*v1) {
      v4 = v1;
      v2 += 1;
      do {
        v3 += 1;
        v4 = (long *)v4[1];
      } while (v4);
    }
  }
  if ((v2 == a0[3]) && (v3 == a0[4]))
    return 1;
  return 0;
}

// Function: sub_109b6 @ 0x109b6
void sub_109b6(unsigned long a0,void *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_10880(a0);
  v2 = sub_10854(a0);
  v3 = sub_1086a(a0);
  v4 = sub_10896(a0);
  fprintf(a1,"# entries:         %lu\n",v1);
  fprintf(a1,"# buckets:         %lu\n",v2);
  fprintf(a1,"# buckets used:    %lu (%.2f%%)\n",(double)v2,(dat_1d278 * (double)v3) / (double)v2,v3);
  fprintf(a1,"max bucket length: %lu\n",v4);
}

// Function: sub_10afd @ 0x10afd
long sub_10afd(long *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*(void *)a0[6])(a1,a0[2]);
  if ((unsigned long)a0[2] <= v1)
    abort(); // no-return
  return v1 * 0x10 + *a0;
}

// Function: sub_10b58 @ 0x10b58
long sub_10b58(long a0,long a1) // return-dupe
{
  void *v1;
  long v2;
  long *v3; // stack - 0x18
  
  v3 = (long *)sub_10afd(a0,a1);
  if (!*v3)
    return 0;
  while( true ) {
    if (!v3)
      return 0;
    if ((a1 == *v3) || (v1 = *(void **)(a0 + 0x38), v2 = *v3, (*v1)(a1,v2))) break;
    v3 = (long *)v3[1];
  }
  return *v3;
}

// Function: sub_10bef @ 0x10bef
long sub_10bef(unsigned long *a0) // early-return
{
  long *v1; // stack - 0x10
  
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

// Function: sub_10c4f @ 0x10c4f
long sub_10c4f(long a0,long a1) // early-return
{
  long *v1; // stack - 0x18
  long *v2; // stack - 0x10
  
  v1 = (long *)sub_10afd(a0,a1);
  v2 = v1;
  while ((a1 != *v2 || (!v2[1]))) {
    v2 = (long *)v2[1];
    if (!v2) {
      do {
        v1 = &v1[2];
        if (*(long **)(a0 + 8) <= v1)
          return 0;
      } while (!*v1);
      return *v1;
    }
  }
  return *(long *)v2[1];
}

// Function: sub_10ced @ 0x10ced
unsigned long sub_10ced(unsigned long *a0,long a1,unsigned long a2)
{
  unsigned long v1;
  unsigned long v2; // stack - 0x20
  long *v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = (long *)*a0;
  do {
    if ((long *)a0[1] <= v3)
      return v2;
    if (*v3) {
      v4 = v3;
      while (v4) {
        if (a2 <= v2)
          return v2;
        v1 = v2 + 1;
        *(long *)(v2 * 8 + a1) = *v4;
        v4 = (long *)v4[1];
        v2 = v1;
      }
    }
    v3 = &v3[2];
  } while( true );
}

// Function: sub_10d8d @ 0x10d8d
long sub_10d8d(unsigned long *a0,void *a1,unsigned long a2)
{
  long v1;
  long v2; // stack - 0x20
  long *v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = (long *)*a0;
  do {
    if ((long *)a0[1] <= v3)
      return v2;
    if (*v3) {
      for (v4 = v3; v4; v4 = (long *)v4[1]) {
        v1 = *v4;
        if ((*a1)(v1,a2) != '\x01')
          return v2;
        v2 += 1;
      }
    }
    v3 = &v3[2];
  } while( true );
}

// Function: sub_10e25 @ 0x10e25
unsigned long sub_10e25(unsigned char *a0,unsigned long a1)
{
  unsigned char *v1; // stack - 0x20
  unsigned long v2; // stack - 0x10
  
  v2 = 0;
  for (v1 = a0; *v1; v1 = &v1[1]) {
    v2 = ((unsigned long)*v1 + v2 * 0x1f) % a1;
  }
  return v2;
}

// Function: sub_10e7f @ 0x10e7f
unsigned long sub_10e7f(unsigned long a0)
{
  unsigned long v1; // stack - 0x18
  unsigned long v2; // stack - 0x10
  
  v2 = 9;
  for (v1 = 3; (v2 < a0 && (a0 % v1)); v1 = v1 + 2) {
    v2 += (v1 + 1) * 4;
  }
  return CONCAT71((undefined7)(a0 % v1 >> 8),a0 % v1 != 0);
}

// Function: sub_10eee @ 0x10eee
unsigned long sub_10eee(unsigned long a0)
{
  unsigned long v1; // stack - 0x10
  
  v1 = a0;
  if (a0 <= 9)
    v1 = 10;
  v1 |= 1;
  while( true ) {
    if (v1 == 0xffffffffffffffff)
      return 0xffffffffffffffff;
    if (sub_10e7f(v1) == '\x01') break;
    v1 += 2;
  }
  return v1;
}

// Function: sub_10f39 @ 0x10f39
void sub_10f39(unsigned int *a0)
{
  *a0 = 0;
  a0[1] = dat_1d280;
  a0[2] = dat_1d284;
  a0[3] = dat_1d288;
  *(char *)&a0[4] = 0;
}

// Function: sub_10f8f @ 0x10f8f
unsigned long sub_10f8f(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_19063(a0,3) % a1;
}

// Function: sub_10fca @ 0x10fca
unsigned long sub_10fca(long a0,long a1)
{
  return CONCAT71((undefined7)((unsigned long)a0 >> 8),a0 == a1);
}

// Function: sub_10fe7 @ 0x10fe7
unsigned long sub_10fe7(long a0) // return-dupe x2
{
  float *v1;
  
  v1 = *(float **)(a0 + 0x28);
  if (v1 == (float *)0x1d1f0)
    return 1;
  if ((((dat_1d28c < v1[2]) && (v1[2] < dat_1d280 - dat_1d28c)) && (dat_1d28c + dat_1d280 < v1[3])) && (((0.0 <= *v1 && (*v1 + dat_1d28c < v1[1])) && ((v1[1] <= dat_1d280 && (*v1 + dat_1d28c < v1[2]))))))
    return 1;
  *(unsigned long *)(a0 + 0x28) = 0x1d1f0;
  return 0;
}

// Function: sub_110f4 @ 0x110f4
unsigned long sub_110f4(unsigned long a0,long a1) // early-return x2
{
  unsigned long v1; // rax
  float v2; // xmm0_da
  unsigned long v3; // stack - 0x20
  
  v3 = a0;
  if (*(char *)(a1 + 0x10) != '\x01') {
    v2 = (float)a0 / *(float *)(a1 + 8);
    if (dat_1d290 <= v2)
      return 0;
    if (dat_1d294 <= v2)
      v3 = (long)(v2 - dat_1d294) ^ 0x8000000000000000;
    else {
      v3 = (unsigned long)v2;
    }
  }
  v1 = sub_10eee(v3);
  if (!((long)(v1 << 3) <= -1 || v1 >> 0x3d))
    return v1;
  return 0;
}

// Function: sub_1120d @ 0x1120d
long * sub_1120d(unsigned long a0,long a1,void *a2,void *a3,long a4) // early-return x2
{
  unsigned long v1;
  long *v2; // rax
  void *v3; // stack - 0x38
  void *v4; // stack - 0x30
  long v5; // stack - 0x28
  
  v4 = a2;
  if (!a2)
    v4 = sub_10f8f;
  v3 = a3;
  if (!a3)
    v3 = sub_10fca;
  v2 = malloc(0x50);
  if (!v2)
    return NULL;
  v5 = a1;
  if (!a1)
    v5 = 0x1d1f0;
  v2[5] = v5;
  if (sub_10fe7(v2) == '\x01') {
    v2[2] = sub_110f4(a0,v5);
    if (v2[2]) {
      v1 = v2[2];
      *v2 = (long)calloc(v1,0x10);
      if (*v2) {
        v2[1] = *v2 + v2[2] * 0x10;
        v2[3] = 0;
        v2[4] = 0;
        v2[6] = (long)v4;
        v2[7] = (long)v3;
        v2[8] = a4;
        v2[9] = 0;
        return v2;
      }
    }
  }
  free(v2);
  return NULL;
}

// Function: sub_11382 @ 0x11382
void sub_11382(unsigned long *a0)
{
  unsigned long *v1;
  long *v2; // stack - 0x20
  unsigned long *v3; // stack - 0x18
  
  for (v2 = (long *)*a0; v2 < (long *)a0[1]; v2 = &v2[2]) {
    if (*v2) {
      v3 = (unsigned long *)v2[1];
      while (v3) {
        if (a0[8])
          (*(void *)a0[8])(*v3);
        *v3 = 0;
        v1 = (unsigned long *)v3[1];
        v3[1] = a0[9];
        a0[9] = v3;
        v3 = v1;
      }
      if (a0[8])
        (*(void *)a0[8])(*v2);
      *v2 = 0;
      v2[1] = 0;
    }
  }
  a0[3] = 0;
  a0[4] = 0;
}

// Function: sub_1148d @ 0x1148d
void sub_1148d(unsigned long *a0)
{
  void *v1;
  long *v2; // stack - 0x20
  long *v3; // stack - 0x18
  
  if ((a0[8]) && (a0[4])) {
    for (v2 = (long *)*a0; v2 < (long *)a0[1]; v2 = &v2[2]) {
      if (*v2) {
        for (v3 = v2; v3; v3 = (long *)v3[1]) {
          (*(void *)a0[8])(*v3);
        }
      }
    }
  }
  for (v2 = (unsigned long)*a0; v2 < a0[1]; v2 = (unsigned long)((long)v2 + 0x10)) {
    v3 = *(void **)((long)v2 + 8);
    while (v3) {
      v1 = *(void **)((long)v3 + 8);
      free(v3);
      v3 = v1;
    }
  }
  v3 = (void *)a0[9];
  while (v3) {
    v1 = *(void **)((long)v3 + 8);
    free(v3);
    v3 = v1;
  }
  free((void *)*a0);
  free(a0);
}

// Function: sub_115bc @ 0x115bc
void * sub_115bc(long a0)
{
  void *v1; // stack - 0x10
  
  if (*(long *)(a0 + 0x48)) { // branch-flip
    v1 = *(void **)(a0 + 0x48);
    *(unsigned long *)(a0 + 0x48) = *(unsigned long *)((long)v1 + 8);
  }
  else {
    v1 = malloc(0x10);
  }
  return v1;
}

// Function: sub_1160b @ 0x1160b
void sub_1160b(long a0,unsigned long *a1)
{
  *a1 = 0;
  a1[1] = *(unsigned long *)(a0 + 0x48);
  *(unsigned long **)(a0 + 0x48) = a1;
}

// Function: sub_11645 @ 0x11645
long sub_11645(long a0,long a1,unsigned long *a2,char a3) // early-return
{
  void *v1;
  long v2;
  long *v3;
  long v4;
  unsigned long v5;
  long *v6; // stack - 0x38
  
  v6 = (long *)sub_10afd(a0,a1);
  *a2 = v6;
  if (!*v6)
    return 0;
  if ((a1 != *v6) && (v1 = *(void **)(a0 + 0x38), v2 = *v6, !(*v1)(a1,v2))) {
    while( true ) {
      if (!v6[1])
        return 0;
      if ((a1 == *(long *)v6[1]) || (v1 = *(void **)(a0 + 0x38), v5 = *(unsigned long *)v6[1], (*v1)(a1,v5))) break;
      v6 = (long *)v6[1];
    }
    v2 = *(long *)v6[1];
    if (a3) {
      v4 = v6[1];
      v6[1] = *(long *)(v4 + 8);
      sub_1160b(a0,v4);
    }
    return v2;
  }
  v2 = *v6;
  if (a3) {
    if (v6[1]) { // branch-flip
      v3 = (long *)v6[1];
      v4 = v3[1];
      *v6 = *v3;
      v6[1] = v4;
      sub_1160b(a0,v3);
    }
    else {
      *v6 = 0;
    }
  }
  return v2;
}

// Function: sub_117de @ 0x117de
unsigned long sub_117de(long a0,unsigned long *a1,char a2)
{
  long v1;
  long *v2;
  long *v3; // rax
  long *v4; // stack - 0x38
  long *v5; // stack - 0x30
  
  v4 = (long *)*a1;
  do {
    if ((long *)a1[1] <= v4)
      return 1;
    if (*v4) {
      v5 = (long *)v4[1];
      while (v5) {
        v1 = *v5;
        v3 = (long *)sub_10afd(a0,v1);
        v2 = (long *)v5[1];
        if (*v3) { // branch-flip
          v5[1] = v3[1];
          v3[1] = (long)v5;
        }
        else {
          *v3 = v1;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
          sub_1160b(a0,v5);
        }
        v5 = v2;
      }
      v1 = *v4;
      v4[1] = 0;
      if (!a2) {
        v2 = (long *)sub_10afd(a0,v1);
        if (*v2) { // branch-flip
          v3 = (long *)sub_115bc(a0);
          if (!v3)
            return 0;
          *v3 = v1;
          v3[1] = v2[1];
          v2[1] = (long)v3;
        }
        else {
          *v2 = v1;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
        }
        *v4 = 0;
        a1[3] = a1[3] + -1;
      }
    }
    v4 = &v4[2];
  } while( true );
}

// Function: sub_119b2 @ 0x119b2
unsigned long sub_119b2(unsigned long *a0,unsigned long a1) // return-dupe x4
{
  void *v1; // stack - 0x68
  unsigned long v10; // stack - 0x20
  void *v2; // stack - 0x60
  unsigned long v3; // stack - 0x58
  unsigned long v4; // stack - 0x50
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  unsigned long v8; // stack - 0x30
  unsigned long v9; // stack - 0x28
  
  v3 = sub_110f4(a1,a0[5]);
  if (!v3)
    return 0;
  if (v3 == a0[2])
    return 1;
  v1 = calloc(v3,0x10);
  if (!v1)
    return 0;
  v2 = (void *)((long)v1 + v3 * 0x10);
  v4 = 0;
  v5 = 0;
  v6 = a0[5];
  v7 = a0[6];
  v8 = a0[7];
  v9 = a0[8];
  v10 = a0[9];
  if (sub_117de(&v1,a0,0)) {
    free((void *)*a0);
    *a0 = v1;
    a0[1] = v2;
    a0[2] = v3;
    a0[3] = v4;
    a0[9] = v10;
    return 1;
  }
  a0[9] = v10;
  if ((sub_117de(a0,&v1,1) == '\x01') && (sub_117de(a0,&v1,0) == '\x01')) {
    free(v1);
    return 0;
  }
  abort(); // no-return
}

// Function: sub_11bdd @ 0x11bdd
unsigned long sub_11bdd(long a0,long a1,long *a2) // return-dupe x2, return-dupe, ternary x2
{
  unsigned long v1; // rax
  long *v2; // rax
  long *v3; // stack - 0x30
  float v4; // xmm0_da
  long v5; // stack - 0x28
  long v6; // stack - 0x20
  
  if (!a1)
    abort(); // no-return
  v5 = sub_11645(a0,a1,&v3,0);
  if (v5) {
    if (!a2)
      return 0;
    *a2 = v5;
    return 0;
  }
  if (((float)*(unsigned long *)(a0 + 0x10) * *(float *)(*(long *)(a0 + 0x28) + 8) < (float)*(unsigned long *)(a0 + 0x18)) && (sub_10fe7(a0), (float)*(unsigned long *)(a0 + 0x10) * *(float *)(*(long *)(a0 + 0x28) + 8) < (float)*(unsigned long *)(a0 + 0x18))) {
    v6 = *(long *)(a0 + 0x28);
    v4 = (*(char *)(v6 + 0x10)) ? (float)*(unsigned long *)(a0 + 0x10) * *(float *)(v6 + 0xc) : *(float *)(v6 + 8) * (*(float *)(v6 + 0xc) * (float)*(unsigned long *)(a0 + 0x10)); // branch-flip
    if (dat_1d290 <= v4)
      return 0xffffffff;
    v1 = (dat_1d294 <= v4) ? (long)(v4 - dat_1d294) ^ 0x8000000000000000 : (unsigned long)v4;
    if (sub_119b2(a0,v1) != '\x01')
      return 0xffffffff;
    if (sub_11645(a0,a1,&v3,0))
      abort(); // no-return
  }
  if (!*v3) {
    *v3 = a1;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
    return 1;
  }
  v2 = (long *)sub_115bc(a0);
  if (v2) {
    *v2 = a1;
    v2[1] = v3[1];
    v3[1] = (long)v2;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    return 1;
  }
  return 0xffffffff;
}

// Function: sub_11f4e @ 0x11f4e
unsigned long sub_11f4e(unsigned long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  
  v1 = sub_11bdd(a0,a1,&v3);
  if (v1 != -1) {
    if (v1) // branch-flip
      v2 = a1;
    else {
      v2 = v3;
    }
    return v2;
  }
  v2 = 0;
  return v2;
}

// Function: sub_11fbe @ 0x11fbe
long sub_11fbe(long a0,unsigned long a1) // early-return, ternary x2
{
  char v1; // al
  long *v2; // stack - 0x40
  float v3; // xmm0_da
  void *v4; // stack - 0x38
  long v5; // stack - 0x30
  long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  void *v8; // stack - 0x18
  
  v5 = sub_11645(a0,a1,&v2,1);
  if (!v5)
    return 0;
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + -1;
  if (((!*v2) && (*(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + -1, (float)*(unsigned long *)(a0 + 0x18) < (float)*(unsigned long *)(a0 + 0x10) * **(float **)(a0 + 0x28))) && (sub_10fe7(a0), (float)*(unsigned long *)(a0 + 0x18) < (float)*(unsigned long *)(a0 + 0x10) * **(float **)(a0 + 0x28))) {
    v6 = *(long *)(a0 + 0x28);
    if (*(char *)(v6 + 0x10)) { // branch-flip
      v3 = (float)*(unsigned long *)(a0 + 0x10) * *(float *)(v6 + 4);
      v7 = (dat_1d294 <= v3) ? (long)(v3 - dat_1d294) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    else {
      v3 = *(float *)(v6 + 8) * (*(float *)(v6 + 4) * (float)*(unsigned long *)(a0 + 0x10));
      v7 = (dat_1d294 <= v3) ? (long)(v3 - dat_1d294) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    v1 = sub_119b2(a0,v7);
    if (v1 != '\x01') {
      v4 = *(void **)(a0 + 0x48);
      while (v4) {
        v8 = *(void **)((long)v4 + 8);
        free(v4);
        v4 = v8;
      }
      *(unsigned long *)(a0 + 0x48) = 0;
    }
  }
  return v5;
}

// Function: sub_122c4 @ 0x122c4
void sub_122c4(unsigned long a0,unsigned long a1)
{
  sub_11fbe(a0,a1);
}

// Function: sub_122ed @ 0x122ed
unsigned long * sub_122ed(void *a0,long a1)
{
  unsigned long *v1; // rax
  long v2; // stack - 0x28
  
  v1 = (unsigned long *)sub_18024(0x20);
  v2 = a1;
  if (!a1)
    v2 = 1;
  *v1 = sub_1819d(v2,8);
  *(unsigned long *)*v1 = 0;
  v1[1] = v2;
  v1[2] = 0;
  if (!a0)
    a0 = sub_12381;
  v1[3] = a0;
  return v1;
}

// Function: sub_12381 @ 0x12381
unsigned long sub_12381(void)
{
  return 0;
}

// Function: sub_12398 @ 0x12398
void sub_12398(unsigned long *a0)
{
  free((void *)*a0);
  free(a0);
}

// Function: sub_123c6 @ 0x123c6
unsigned long sub_123c6(long *a0,unsigned long a1)
{
  long v1;
  
  if ((unsigned long)(a0[1] - 1U) <= (unsigned long)a0[2]) {
    v1 = *a0;
    *a0 = sub_18221(v1,&a0[1],8);
  }
  a0[2] = a0[2] + 1;
  *(unsigned long *)(*a0 + a0[2] * 8) = a1;
  sub_12637(*a0,a0[2],a0[3]);
  return 0;
}

// Function: sub_12474 @ 0x12474
unsigned long sub_12474(long *a0) // early-return
{
  unsigned long v1;
  long v2;
  
  if (!a0[2])
    return 0;
  v1 = *(unsigned long *)(*a0 + 8);
  v2 = a0[2];
  a0[2] = v2 + -1;
  *(unsigned long *)(*a0 + 8) = *(unsigned long *)(*a0 + v2 * 8);
  sub_12508(*a0,a0[2],1,a0[3]);
  return v1;
}

// Function: sub_12508 @ 0x12508
unsigned long sub_12508(long a0,unsigned long a1,unsigned long a2,void *a3)
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  unsigned long v4; // stack - 0x20
  unsigned long v5; // stack - 0x18
  
  v1 = *(unsigned long *)(a0 + a2 * 8);
  v4 = a2;
  while (v4 <= a1 >> 1) {
    v5 = v4 * 2;
    if ((v5 < a1) && (v2 = *(unsigned long *)(a0 + (v5 + 1) * 8), v3 = *(unsigned long *)(a0 + v4 * 0x10), (int)(*a3)(v3,v2) <= -1))
      v5 += 1;
    v2 = *(unsigned long *)(a0 + v5 * 8);
    if ((int)(*a3)(v2,v1) <= 0) break;
    *(unsigned long *)(a0 + v4 * 8) = *(unsigned long *)(a0 + v5 * 8);
    v4 = v5;
  }
  *(unsigned long *)(v4 * 8 + a0) = v1;
  return v4;
}

// Function: sub_12637 @ 0x12637
void sub_12637(long a0,unsigned long a1,void *a2)
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3; // stack - 0x18
  
  v1 = *(unsigned long *)(a0 + a1 * 8);
  for (v3 = a1; v3 != 1; v3 = v3 >> 1) {
    v2 = *(unsigned long *)(a0 + (v3 >> 1) * 8);
    if (1 <= (int)(*a2)(v2,v1)) break;
    *(unsigned long *)(a0 + v3 * 8) = *(unsigned long *)(a0 + (v3 >> 1) * 8);
  }
  *(unsigned long *)(v3 * 8 + a0) = v1;
}

// Function: sub_126fe @ 0x126fe
char * sub_126fe(int a0,long a1)
{
  char *v1;
  int v2; // stack - 0x1c
  char *v3; // stack - 0x10
  
  v3 = (char *)(a1 + 0xb);
  *v3 = '\0';
  v2 = a0;
  v1 = v3;
  if (0 <= a0) { // branch-flip
    do {
      v3 = &v3[-1];
      *v3 = (char)v2 + (char)(v2 / 10) * '\xf6' + '0';
      v2 /= 10;
    } while (v2);
  }
  else {
    do {
      v3 = v1;
      v3[-1] = ((char)(v2 / 10) * '\n' - (char)v2) + '0';
      v2 /= 10;
      v1 = &v3[-1];
    } while (v2);
    v3 = &v3[-2];
    *v3 = '-';
  }
  return v3;
}

// Function: sub_12802 @ 0x12802
char * sub_12802(unsigned int a0,long a1)
{
  unsigned int v1; // stack - 0x1c
  char *v2; // stack - 0x10
  
  v2 = (char *)(a1 + 10);
  *v2 = '\0';
  v1 = a0;
  do {
    v2 = &v2[-1];
    *v2 = (char)v1 + (char)(v1 / 10) * '\xf6' + '0';
    v1 /= 10;
  } while (v1);
  return v2;
}

// Function: sub_1287e @ 0x1287e
char * sub_1287e(unsigned long a0,long a1)
{
  unsigned long v1; // stack - 0x20
  char *v2; // stack - 0x10
  
  v2 = (char *)(a1 + 0x14);
  *v2 = '\0';
  v1 = a0;
  do {
    v2 = &v2[-1];
    *v2 = (char)v1 + ((char)(v1 / 10 << 2) + (char)(v1 / 10)) * '\xfe' + '0';
    v1 /= 10;
  } while (v1);
  return v2;
}

// Function: sub_12907 @ 0x12907
void sub_12907(char *a0,unsigned int a1)
{
  sub_1293d(a0,strlen(a0),a1);
}

// Function: sub_1293d @ 0x1293d
int sub_1293d(unsigned char *a0,long a1,unsigned int a2)
{
  unsigned char *v1;
  unsigned char *v10; // stack - 0x20
  unsigned char v2;
  int v3; // eax
  char v4 [8];
  int v5; // stack - 0x3c
  int v6; // stack - 0x38
  int v7; // stack - 0x34
  unsigned char *v8; // stack - 0x30
  long v9; // stack - 0x28
  
  v10 = &a0[a1];
  v6 = 0;
  v8 = a0;
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
label_12af6:
    if (v8 < v10) {
      v3 = (int)(char)*v8;
      if (0x7f <= v3) { // branch-flip
label_129d5:
        memset(v4,0,8);
        do {
          v9 = sub_19dd4(&v5,v8,(long)v10 - (long)v8,v4);
          if (v9 == -1) {
            if (a2 & 1)
              return -1;
            v8 = &v8[1];
            v6 += 1;
            break;
          }
          if (v9 == -2) {
            if (a2 & 1)
              return -1;
            v8 = v10;
            v6 += 1;
            break;
          }
          if (!v9)
            v9 = 1;
          v7 = wcwidth(v5);
          if (0 <= v7) { // branch-flip
            if (0x7fffffffU - v6 < v7)
              return 0x7fffffff;
            v6 += v7;
          }
          else {
            if (a2 & 2)
              return -1;
            v3 = iswcntrl(v5);
            if (!v3) {
              if (v6 == 0x7fffffff)
                return 0x7fffffff;
              v6 += 1;
            }
          }
          v8 = &v8[v9];
        } while (!mbsinit(v4));
      }
      else {
        if (v3 <= 0x60) {
          if (0x60 > v3) {
            if (v3 > 0x40) goto label_129c7;
            if (0x24 <= v3) { // branch-flip
              if (0x1b > (unsigned int)(v3 - 0x25U)) goto label_129c7;
            }
            else if (v3 > 0x1f) goto label_129c7;
          }
          goto label_129d5;
        }
label_129c7:
        v8 = &v8[1];
        v6 += 1;
      }
      goto label_12af6;
    }
  }
  else {
    while (v8 < v10) {
      v1 = &v8[1];
      v2 = *v8;
      v8 = v1;
      if (*(unsigned short *)((unsigned long)v2 * 2 + *(long *)__ctype_b_loc()) & 0x4000) { // branch-flip
        if (v6 == 0x7fffffff)
          return 0x7fffffff;
        v6 += 1;
      }
      else {
        if (a2 & 2)
          return -1;
        if (!(*(unsigned short *)((unsigned long)v2 * 2 + *(long *)__ctype_b_loc()) & 2)) {
          if (v6 == 0x7fffffff)
            return 0x7fffffff;
          v6 += 1;
        }
      }
    }
  }
  return v6;
}

// Function: sub_12bc5 @ 0x12bc5
long sub_12bc5(void) // return-dupe
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

// Function: sub_12c4b @ 0x12c4b
unsigned long sub_12c4b(int a0) // return-dupe
{
  unsigned long v1; // rax
  unsigned long v2; // stack - 0x28
  
  if (a0 != 1) {
    v2 = sysconf(0x53);
    if (((v2 == 1) || (v2 == 2)) && (v1 = sub_12bc5(), v2 < v1))
      v2 = v1;
    if ((long)v2 <= 0)
      return 1;
    return v2;
  }
  v1 = sub_12bc5();
  if (v1)
    return v1;
  v1 = sysconf(0x54);
  if ((long)v1 <= 0)
    return 1;
  return v1;
}

// Function: sub_12cdc @ 0x12cdc
unsigned long sub_12cdc(char *a0)
{
  char v1;
  char *v2; // stack - 0x28
  char *v3; // stack - 0x30
  unsigned long v4; // stack - 0x20
  unsigned long v5; // stack - 0x18
  
  v4 = 0;
  v3 = a0;
  if (!a0)
    return 0;
  while ((*v3 && (v1 = *v3, sub_19332((int)v1)))) {
    v3 = &v3[1];
  }
  v1 = *v3;
  if (sub_19258((int)v1)) {
    v2 = NULL;
    v5 = strtoul(v3,&v2,10);
    if (v2) {
      while ((*v2 && (v1 = *v2, sub_19332((int)v1)))) {
        v2 = &v2[1];
      }
      if (!*v2)
        return v5;
      if (*v2 == ',')
        return v5;
    }
  }
  return v4;
}

// Function: sub_12de4 @ 0x12de4
unsigned long sub_12de4(int a0)
{
  unsigned long v1;
  int v2; // stack - 0x2c
  unsigned long v3; // stack - 0x20
  
  v3 = 0xffffffffffffffff;
  v2 = a0;
  if (a0 == 2) {
    v1 = sub_12cdc(getenv("OMP_NUM_THREADS"));
    v3 = sub_12cdc(getenv("OMP_THREAD_LIMIT"));
    if (!v3)
      v3 = 0xffffffffffffffff;
    if (v1) {
      if (v1 < v3)
        return v1;
      return v3;
    }
    v2 = 1;
  }
  v1 = sub_12c4b(v2);
  if (v3 <= v1)
    v1 = v3;
  return v1;
}

// Function: sub_12e84 @ 0x12e84
double sub_12e84(void)
{
  char v1 [32];
  double v2;
  double v3; // xmm0_qa
  unsigned long v4; // stack - 0x68
  unsigned int v5; // stack - 0x20
  
  v2 = (double)sysconf(0x55);
  v3 = (double)sysconf(0x1e);
  if ((v2 < 0.0) || (v3 < 0.0)) {
    if (sysinfo(v1)) // branch-flip
      v2 = dat_1d2c0;
    else {
      v2 = (double)v5 * (double)v4;
    }
  }
  else {
    v2 = v2 * v3;
  }
  return v2;
}

// Function: sub_12fab @ 0x12fab
double sub_12fab(void)
{
  char v1 [40];
  double v2; // xmm0_qa
  double v3; // xmm0_qa
  double v4; // xmm0_qa
  unsigned long v5; // stack - 0x60
  unsigned long v6; // stack - 0x50
  unsigned int v7; // stack - 0x20
  
  v2 = (double)sysconf(0x56);
  v3 = (double)sysconf(0x1e);
  if ((v2 < 0.0) || (v3 < 0.0)) {
    if (sysinfo(v1)) { // branch-flip
      sub_12e84();
      v2 = v4 / dat_1d2c8;
    }
    else {
      v2 = (double)v7 * ((double)v5 + (double)v6);
    }
  }
  else {
    v2 = v2 * v3;
  }
  return v2;
}

// Function: sub_1311d @ 0x1311d
int sub_1311d(int *a0,unsigned int a1) // early-return x3
{
  int v1;
  int v2;
  int v3;
  unsigned int v4; // eax
  
  v1 = *a0;
  v2 = a0[1];
  if (0 <= dat_23864) {
    v3 = pipe2(a0,a1);
    if ((0 <= v3) || (*__errno_location() != 0x26)) {
      dat_23864 = 1;
      return v3;
    }
    dat_23864 = -1;
  }
  if (a1 & 0xfff7f7ff) {
    *__errno_location() = 0x16;
    return -1;
  }
  if (0 <= pipe(a0)) {
    if (((!(a1 & 0x800)) || ((((v4 = sub_1967a(a0[1],3,0), 0 <= (int)v4 && (v3 = a0[1], sub_1967a(v3,4,v4 | 0x800) != -1)) && (v4 = sub_1967a(*a0,3,0), 0 <= (int)v4)) && (v3 = *a0, sub_1967a(v3,4,v4 | 0x800) != -1)))) && ((!(a1 & 0x80000) || (((v4 = sub_1967a(a0[1],1,0), 0 <= (int)v4 && (v3 = a0[1], sub_1967a(v3,2,v4 | 1) != -1)) && ((v4 = sub_1967a(*a0,1,0), 0 <= (int)v4 && (v3 = *a0, sub_1967a(v3,2,v4 | 1) != -1))))))))
      return 0;
    v3 = *__errno_location();
    close(*a0);
    close(a0[1]);
    *a0 = v1;
    a0[1] = v2;
    *__errno_location() = v3;
    return -1;
  }
  return -1;
}

// Function: sub_133ab @ 0x133ab
long sub_133ab(void) // early-return x2
{
  long v1; // rax
  char *v2; // stack - 0x30
  long v3; // stack - 0x28
  char *v4; // stack - 0x20
  
  v3 = 0x31069;
  v4 = getenv("_POSIX2_VERSION");
  if (((v4) && (*v4)) && (v1 = strtol(v4,&v2,10), !*v2))
    v3 = v1;
  if (-0x80000000 <= v3) {
    if (v3 <= 0x7fffffff)
      return v3;
    return 0x7fffffff;
  }
  return 0x80000000;
}

// Function: sub_1345f @ 0x1345f
void sub_1345f(char *a0)
{
  char *v1;
  char *v2; // stack - 0x20
  
  if (!a0) {
    fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
    abort(); // no-return
  }
  v1 = strrchr(a0,0x2f);
  if (v1) // branch-flip
    v1 = &v1[1];
  else {
    v1 = a0;
  }
  v2 = a0;
  if (7 <= (long)v1 - (long)a0) {
    v2 = a0;
    if (!strncmp(&v1[-7],"/.libs/",7)) {
      v2 = v1;
      if (!strncmp(v1,"lt-",3)) {
        __progname = &v1[3];
        v2 = __progname;
      }
    }
  }
  dat_23868 = v2;
  program_invocation_name = v2;
}

// Function: sub_1354a @ 0x1354a
unsigned long sub_1354a(long a0)
{
  int v1;
  unsigned long v2; // rax
  
  v1 = *__errno_location();
  if (!a0)
    a0 = 0x23880;
  v2 = sub_18564(a0,0x38);
  *__errno_location() = v1;
  return v2;
}

// Function: sub_13599 @ 0x13599
unsigned int sub_13599(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x23880;
  return *a0;
}

// Function: sub_135bd @ 0x135bd
void sub_135bd(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x23880;
  *a0 = a1;
}

// Function: sub_135e8 @ 0x135e8
unsigned int sub_135e8(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int v1; // eax
  unsigned int *v2; // rax
  
  if (!a0)
    a0 = 0x23880;
  v2 = (unsigned int *)((unsigned long)(a1 >> 5) * 4 + a0 + 8);
  v1 = *v2 >> (a1 & 0x1f) & 1;
  *v2 = *v2 ^ (a2 & 1 ^ v1) << (a1 & 0x1f);
  return v1;
}

// Function: sub_13678 @ 0x13678
unsigned int sub_13678(long a0,unsigned int a1)
{
  unsigned int v1;
  long v2; // stack - 0x20
  
  v2 = a0;
  if (!a0)
    v2 = 0x23880;
  v1 = *(unsigned int *)(v2 + 4);
  *(unsigned int *)(v2 + 4) = a1;
  return v1;
}

// Function: sub_136b2 @ 0x136b2
void sub_136b2(unsigned int *a0,long a1,long a2)
{
  unsigned int *v1; // stack - 0x10
  
  v1 = a0;
  if (!a0)
    v1 = (unsigned int *)0x23880;
  *v1 = 10;
  if ((a1) && (a2)) {
    *(long *)&v1[10] = a1;
    *(long *)&v1[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_13714 @ 0x13714
unsigned long * sub_13714(unsigned long *a0,unsigned int a1)
{
  unsigned long v1; // stack - 0x58
  
  if (a1 != 10) {
    v1 = (unsigned long)a1;
    *a0 = v1;
    a0[1] = 0;
    a0[2] = 0;
    a0[3] = 0;
    a0[4] = 0;
    a0[5] = 0;
    a0[6] = 0;
    return a0;
  }
  abort(); // no-return
}

// Function: sub_137d9 @ 0x137d9
char * sub_137d9(char *a0,int a1) // early-return x6
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = gettext(a0);
  if (v1 != a0)
    return v1;
  v2 = sub_19d8c();
  if (!sub_19411(v2,"UTF-8")) {
    if (*a0 == '`')
      return "‘";
    return "’";
  }
  if (sub_19411(v2,"GB18030")) {
    if (a1 == 9)
      return "\"";
    return "\'";
  }
  if (*a0 == '`')
    return (char *)0x1d3de;
  return (char *)0x1d3e2;
}

// Function: sub_1389e @ 0x1389e
unsigned long sub_1389e(long a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
{
  bool v1;
  unsigned long v10;
  unsigned long v11;
  char v12 [8];
  unsigned int v13; // stack - 0x74
  bool v14;
  bool v15;
  bool v16;
  char *v17; // stack - 0xc8
  char *v18; // stack - 0xc0
  int v19; // stack - 0xac
  bool v2;
  unsigned long v20; // stack - 0xa8
  unsigned long v21; // stack - 0x98
  unsigned char v22; // stack - 0x7b
  unsigned char v23; // stack - 0x7a
  unsigned long v24; // stack - 0x70
  unsigned long v25; // stack - 0x68
  unsigned long v26; // stack - 0x60
  char *v27; // stack - 0x58
  unsigned long v28; // stack - 0x50
  unsigned long v29; // stack - 0x48
  bool v3;
  unsigned long v30; // stack - 0x40
  unsigned long v31; // stack - 0x38
  unsigned long v32; // stack - 0x30
  bool v4;
  bool v5;
  bool v6;
  long v7;
  int v8; // eax
  unsigned long v9; // rax
  
  v7 = a6;
  v18 = a7;
  v17 = a8;
  v25 = 0;
  v26 = 0;
  v27 = NULL;
  v28 = 0;
  v1 = 0;
  v9 = __ctype_get_mb_cur_max();
  v14 = (a5 & 2) != 0;
  v2 = 0;
  v5 = 0;
  v6 = 1;
  v19 = a4;
  v20 = a3;
  v21 = a1;
label_13958:
  switch(v19) {
    case 0:
      v14 = 0;
      break;
    case 1:
label_13aac:
      v14 = 1;
      goto label_13ab0;
    case 2:
label_13abf:
      v19 = 2;
      if (!v14) {
        if (v25 < v21)
          *(char *)(v25 + a0) = 0x27;
        v25 += 1;
      }
      v27 = "\'";
      v28 = 1;
      break;
    case 3:
      v1 = 1;
      goto label_13aac;
    case 4:
label_13ab0:
      if (!v14)
        v1 = 1;
      goto label_13abf;
    case 5:
label_1399a:
      if (!v14) {
        if (v25 < v21)
          *(char *)(v25 + a0) = 0x22;
        v25 += 1;
      }
      v1 = 1;
      v27 = "\"";
      v28 = 1;
      break;
    case 6:
      v19 = 5;
      v14 = 1;
      goto label_1399a;
    case 7:
      v1 = 1;
      v14 = 0;
      break;
    case 8:
    case 9:
    case 10:
      if (v19 != 10) {
        v18 = (char *)sub_137d9("`",v19);
        v17 = (char *)sub_137d9("\'",v19);
      }
      if (!v14) {
        for (v27 = v18; *v27; v27 = &v27[1]) {
          if (v25 < v21)
            *(char *)(a0 + v25) = *v27;
          v25 += 1;
        }
      }
      v1 = 1;
      v27 = v17;
      v28 = strlen(v17);
      break;
    default:
      abort(); // no-return
    
  }
  v24 = 0;
label_1475e:
  if (v20 != 0xffffffffffffffff) // branch-flip
    v16 = v24 != v20;
  else {
    v16 = a2[v24] != '\0';
  }
  if (!v16) goto label_14796;
  v3 = 0;
  v4 = 0;
  v16 = 0;
  if (((v1) && (v19 != 2)) && (v28)) {
    v11 = v24 + v28;
    if ((v20 != 0xffffffffffffffff) || (v28 <= 1))
      v10 = v20;
    else {
      v20 = strlen(a2);
      v10 = v20;
    }
    if ((v11 <= v10) && (v8 = memcmp(&a2[v24],v27,v28), !v8)) {
      if (v14) goto label_148da;
      v3 = 1;
    }
  }
  v22 = a2[v24];
  if (v22 == 0x7e) {
label_1407d:
    if (!v24) {
label_14088:
      v16 = 1;
label_1408c:
      if (v19 != 2) goto label_145a3;
label_13e65:
      if (v14) goto label_148da;
    }
    goto label_145a3;
  }
  if (0x7e < v22) {
label_14169:
    if (v9 != 1) {
      memset(v12,0,8);
      v29 = 0;
      v16 = 1;
      if (v20 == 0xffffffffffffffff)
        v20 = strlen(a2);
label_141e3:
      v31 = sub_19dd4(&v13,&a2[v24 + v29],v20 - (v24 + v29),v12);
      if (v31) {
        if (v31 != 0xffffffffffffffff) { // branch-flip
          if (v31 != 0xfffffffffffffffe) goto label_14291;
          v16 = 0;
          for (; (v29 + v24 < v20 && (a2[v24 + v29])); v29 = v29 + 1) {
          }
        }
        else {
          v16 = 0;
        }
      }
      goto label_14340;
    }
    v29 = 1;
    v16 = (*(unsigned short *)((unsigned long)v22 * 2 + *(long *)__ctype_b_loc()) & 0x4000) != 0;
label_14340:
    if ((v29 <= 1) && ((!v1 || (v16)))) goto label_145a3;
    v32 = v29 + v24;
    while( true ) {
      if ((!v1) || (v16)) {
        if (v3) {
          if (v25 < v21)
            *(char *)(v25 + a0) = 0x5c;
          v25 += 1;
          v3 = 0;
        }
      }
      else {
        if (v14) goto label_148da;
        v4 = 1;
        if ((v19 == 2) && (!v2)) {
          if (v25 < v21)
            *(char *)(v25 + a0) = 0x27;
          if (v25 + 1 < v21)
            *(char *)(v25 + 1 + a0) = 0x24;
          if (v25 + 2 < v21)
            *(char *)(v25 + 2 + a0) = 0x27;
          v25 += 3;
          v2 = 1;
        }
        if (v25 < v21)
          *(char *)(v25 + a0) = 0x5c;
        if (v25 + 1 < v21)
          *(unsigned char *)(v25 + 1 + a0) = (v22 >> 6) + 0x30;
        if (v25 + 2 < v21)
          *(unsigned char *)(v25 + 2 + a0) = (v22 >> 3 & 7) + 0x30;
        v25 += 3;
        v22 = (v22 & 7) + 0x30;
      }
      if (v32 <= v24 + 1) break;
      if ((v2) && (!v4)) {
        if (v25 < v21)
          *(char *)(v25 + a0) = 0x27;
        if (v25 + 1 < v21)
          *(char *)(v25 + 1 + a0) = 0x27;
        v25 += 2;
        v2 = 0;
      }
      if (v25 < v21)
        *(unsigned char *)(a0 + v25) = v22;
      v25 += 1;
      v24 += 1;
      v22 = a2[v24];
    }
    goto label_146c9;
  }
  if (v22 == 0x7d) {
label_1404b:
    if (v20 != 0xffffffffffffffff) // branch-flip
      v15 = v20 != 1;
    else {
      v15 = a2[1] != '\0';
    }
    if (v15) goto label_145a3;
    goto label_1407d;
  }
  if (0x7d < v22) goto label_14169;
  if (v22 == 0x7c) goto label_1408c;
  if (0x7c < v22) goto label_14169;
  if (v22 == 0x7b) goto label_1404b;
  if (0x7b < v22) goto label_14169;
  if (0x3f < v22) {
    if ((0x7a < v22) || (v22 <= 0x40)) goto label_14169;
    v11 = 1L << (v22 + 0xbf & 0x3f);
    if (v11 & 0x3ffffff53ffffff) goto label_14160;
    if (v11 & 0xa4000000) goto label_1408c;
    if (!(v11 & 0x8000000)) goto label_14169;
    if (v19 == 2) {
      if (!v14) goto label_146c9;
      goto label_148da;
    }
    v23 = v22;
    if (((!v1) || (!v14)) || (!v28)) goto label_14021;
    goto label_146c9;
  }
  switch(v22) {
    case 0:
      if (!v1) {
        if (!(a5 & 1)) goto label_145a3;
        goto label_14759;
      }
      if (!v14) {
        v4 = 1;
        if ((v19 == 2) && (!v2)) {
          if (v25 < v21)
            *(char *)(v25 + a0) = 0x27;
          if (v25 + 1 < v21)
            *(char *)(v25 + 1 + a0) = 0x24;
          if (v25 + 2 < v21)
            *(char *)(v25 + 2 + a0) = 0x27;
          v25 += 3;
          v2 = 1;
        }
        if (v25 < v21)
          *(char *)(v25 + a0) = 0x5c;
        v11 = v25 + 1;
        if ((((v19 != 2) && (v24 + 1 < v20)) && ('0' <= a2[v24 + 1])) && (a2[v24 + 1] <= '9')) {
          if (v11 < v21)
            *(char *)(v11 + a0) = 0x30;
          if (v25 + 2 < v21)
            *(char *)(v25 + 2 + a0) = 0x30;
          v11 = v25 + 3;
        }
        v25 = v11;
        v22 = 0x30;
        goto label_145a3;
      }
      goto label_148da;
    default:
      goto label_14169;
    case 7:
      v23 = 0x61;
      break;
    case 8:
      v23 = 0x62;
      break;
    case 9:
      v23 = 0x74;
      goto label_14021;
    case 10:
      v23 = 0x6e;
      goto label_14021;
    case 0xb:
      v23 = 0x76;
      break;
    case 0xc:
      v23 = 0x66;
      break;
    case 0xd:
      v23 = 0x72;
label_14021:
      if ((v19 == 2) && (v14)) goto label_148da;
      break;
    case 0x20:
      goto label_14088;
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
      goto label_1408c;
    case 0x23:
      goto label_1407d;
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
label_14160:
      v16 = 1;
      goto label_145a3;
    case 0x27:
      v5 = 1;
      v16 = 1;
      if (v19 == 2) {
        if (!v14) {
          if ((v21) && (!v26)) {
            v26 = v21;
            v21 = 0;
          }
          if (v25 < v21)
            *(char *)(v25 + a0) = 0x27;
          if (v25 + 1 < v21)
            *(char *)(v25 + 1 + a0) = 0x5c;
          if (v25 + 2 < v21)
            *(char *)(v25 + 2 + a0) = 0x27;
          v25 += 3;
          v2 = 0;
          goto label_145a3;
        }
        goto label_148da;
      }
      goto label_145a3;
    case 0x3f:
      if (v19 == 2) goto label_13e65;
      if ((((v19 == 5) && (a5 & 4)) && ((v24 + 2 < v20 && ((a2[v24 + 1] == '?' && ((unsigned char)a2[v24 + 2] <= 0x3e)))))) && (0x7000a38200000000U >> (a2[v24 + 2] & 0x3fU) & 1)) {
        if (!v14) {
          v22 = a2[v24 + 2];
          v24 += 2;
          if (v25 < v21)
            *(char *)(v25 + a0) = 0x3f;
          if (v25 + 1 < v21)
            *(char *)(v25 + 1 + a0) = 0x22;
          if (v25 + 2 < v21)
            *(char *)(v25 + 2 + a0) = 0x22;
          if (v25 + 3 < v21)
            *(char *)(v25 + 3 + a0) = 0x3f;
          v25 += 4;
          goto label_145a3;
        }
        goto label_148da;
      }
      goto label_145a3;
    
  }
  if (v1) {
    v22 = v23;
label_1460e:
    if (v14) goto label_148da;
    v4 = 1;
    if ((v19 == 2) && (!v2)) {
      if (v25 < v21)
        *(char *)(v25 + a0) = 0x27;
      if (v25 + 1 < v21)
        *(char *)(v25 + 1 + a0) = 0x24;
      if (v25 + 2 < v21)
        *(char *)(v25 + 2 + a0) = 0x27;
      v25 += 3;
      v2 = 1;
    }
    if (v25 < v21)
      *(char *)(v25 + a0) = 0x5c;
    v25 += 1;
  }
  else {
label_145a3:
    if (((((v1) && (v19 != 2)) || (v14)) && ((v7 && (*(unsigned int *)(v7 + (unsigned long)(v22 >> 5) * 4) >> (v22 & 0x1f) & 1)))) || (v3)) goto label_1460e;
  }
label_146c9:
  if ((v2) && (!v4)) {
    if (v25 < v21)
      *(char *)(v25 + a0) = 0x27;
    if (v25 + 1 < v21)
      *(char *)(v25 + 1 + a0) = 0x27;
    v25 += 2;
    v2 = 0;
  }
  if (v25 < v21)
    *(unsigned char *)(a0 + v25) = v22;
  v25 += 1;
  if (!v16)
    v6 = 0;
label_14759:
  v24 += 1;
  goto label_1475e;
label_14291:
  if ((v14) && (v19 == 2)) {
    for (v30 = 1; v30 < v31; v30 = v30 + 1) {
      if (((unsigned int)((int)a2[v24 + v29 + v30] - 0x5bU) <= 0x21) && (0x20000002bU >> ((unsigned char)((int)a2[v24 + v29 + v30] - 0x5bU) & 0x3f) & 1)) goto label_148da;
    }
  }
  v8 = iswprint(v13);
  if (!v8)
    v16 = 0;
  v29 += v31;
  if (mbsinit(v12)) goto label_14340;
  goto label_141e3;
label_14796:
  if (((!v25) && (v19 == 2)) && (v14)) {
label_148da:
    if ((v19 == 2) && (v1))
      v19 = 4;
    return sub_1389e(a0,v21,a2,v20,v19,a5 & 0xfffffffd,0,v18,v17);
  }
  if (((v19 == 2) && (!v14)) && (v5)) {
    if (v6)
      return sub_1389e(a0,v26,a2,v20,5,a5,v7,v18,v17);
    if ((!v21) && (v26)) {
      v21 = v26;
      v25 = 0;
      goto label_13958;
    }
  }
  if ((v27) && (!v14)) {
    for (; *v27; v27 = &v27[1]) {
      if (v25 < v21)
        *(char *)(a0 + v25) = *v27;
      v25 += 1;
    }
  }
  if (v25 < v21)
    *(char *)(v25 + a0) = 0;
  return v25;
}

// Function: sub_1495f @ 0x1495f
unsigned long sub_1495f(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  unsigned long v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x23880;
  v1 = *__errno_location();
  v2 = sub_1389e(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(unsigned long *)&a4[10],*(unsigned long *)&a4[0xc]);
  *__errno_location() = v1;
  return v2;
}

// Function: sub_14a07 @ 0x14a07
void sub_14a07(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_14a3d(a0,a1,0,a2);
}

// Function: sub_14a3d @ 0x14a3d
unsigned long sub_14a3d(unsigned long a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  unsigned int v2; // eax
  long v3; // rax
  unsigned long v4; // rax
  
  if (!a3)
    a3 = (unsigned int *)0x23880;
  v1 = *__errno_location();
  v2 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_1389e(0,0,a0,a1,*a3,v2,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  v4 = sub_18070(v3 + 1);
  sub_1389e(v4,v3 + 1,a0,a1,*a3,v2,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  *__errno_location() = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_14b69 @ 0x14b69
void sub_14b69(void) // return-dupe
{
  void *v1;
  int v2; // stack - 0x14
  
  v1 = dat_23190;
  for (v2 = 1; v2 < dat_23120; v2 = v2 + 1) {
    free(*(void **)((long)v1 + (long)v2 * 0x10 + 8));
  }
  if (*(long *)((long)v1 + 8) != 0x238c0) {
    free(*(void **)((long)v1 + 8));
    dat_23180 = 0x100;
    dat_23188 = 0x238c0;
  }
  if (v1 == (void *)0x23180) {
    dat_23120 = 1;
    return;
  }
  free(v1);
  dat_23120 = 1;
  dat_23190 = (void *)0x23180;
}

// Function: sub_14c28 @ 0x14c28
void * sub_14c28(int a0,unsigned long a1,unsigned long a2,unsigned int *a3)
{
  int v1;
  unsigned int v2;
  unsigned long *v3;
  long v4; // stack - 0x38
  bool v5; // zf
  unsigned long *v6; // stack - 0x30
  void *v7; // stack - 0x28
  unsigned long v8; // stack - 0x20
  unsigned long v9; // stack - 0x18
  
  v1 = *__errno_location();
  v6 = dat_23190;
  if ((0 <= a0) && (a0 <= 0x7ffffffe)) {
    if (dat_23120 <= a0) {
      v5 = dat_23190 == (unsigned long *)0x23180;
      v4 = (long)dat_23120;
      if (v5)
        v3 = NULL;
      else {
        v3 = dat_23190;
      }
      dat_23190 = (unsigned long *)sub_18300(v3,&v4,(long)((a0 - dat_23120) + 1),0x7fffffff,0x10);
      v6 = dat_23190;
      if (v5) {
        *dat_23190 = dat_23180;
        dat_23190[1] = dat_23188;
      }
      memset(&dat_23190[(long)dat_23120 * 2],0,(v4 - dat_23120) * 0x10);
      dat_23120 = (int)v4;
    }
    v8 = v6[(long)a0 * 2];
    v7 = (void *)v6[(long)a0 * 2 + 1];
    v2 = a3[1];
    v9 = sub_1389e(v7,v8,a1,a2,*a3,v2 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    if (v8 <= v9) {
      v8 = v9 + 1;
      v6[(long)a0 * 2] = v8;
      if (v7 != (void *)0x238c0)
        free(v7);
      v7 = (void *)sub_18070(v8);
      v6[(long)a0 * 2 + 1] = v7;
      sub_1389e(v7,v8,a1,a2,*a3,v2 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    }
    *__errno_location() = v1;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_14ed7 @ 0x14ed7
void sub_14ed7(unsigned int a0,unsigned long a1)
{
  sub_14c28(a0,a1,0xffffffffffffffff,0x23880);
}

// Function: sub_14f0b @ 0x14f0b
void sub_14f0b(unsigned int a0,unsigned long a1,unsigned long a2)
{
  sub_14c28(a0,a1,a2,0x23880);
}

// Function: sub_14f3d @ 0x14f3d
void sub_14f3d(unsigned long a0)
{
  sub_14ed7(0,a0);
}

// Function: sub_14f60 @ 0x14f60
void sub_14f60(unsigned long a0,unsigned long a1)
{
  sub_14f0b(0,a0,a1);
}

// Function: sub_14f8b @ 0x14f8b
void sub_14f8b(unsigned int a0,unsigned int a1,unsigned long a2)
{
  char v1 [56];
  
  sub_13714(v1,a1);
  sub_14c28(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_14ff3 @ 0x14ff3
void sub_14ff3(unsigned int a0,unsigned int a1,unsigned long a2,unsigned long a3)
{
  char v1 [56];
  
  sub_13714(v1,a1);
  sub_14c28(a0,a2,a3,v1);
}

// Function: sub_15059 @ 0x15059
void sub_15059(unsigned int a0,unsigned long a1)
{
  sub_14f8b(0,a0,a1);
}

// Function: sub_15081 @ 0x15081
void sub_15081(unsigned int a0,unsigned long a1,unsigned long a2)
{
  sub_14ff3(0,a0,a1,a2);
}

// Function: sub_150b1 @ 0x150b1
void sub_150b1(unsigned long a0,unsigned long a1,char a2)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v1 = dat_23880;
  v2 = dat_23888;
  v3 = dat_23890;
  v4 = dat_23898;
  v5 = dat_238a0;
  v6 = dat_238a8;
  v7 = dat_238b0;
  sub_135e8(&v1,(int)a2,1);
  sub_14c28(0,a0,a1,&v1);
}

// Function: sub_1516c @ 0x1516c
void sub_1516c(unsigned long a0,char a1)
{
  sub_150b1(a0,0xffffffffffffffff,(int)a1);
}

// Function: sub_1519a @ 0x1519a
void sub_1519a(unsigned long a0)
{
  sub_1516c(a0,0x3a);
}

// Function: sub_151bd @ 0x151bd
void sub_151bd(unsigned long a0,unsigned long a1)
{
  sub_150b1(a0,a1,0x3a);
}

// Function: sub_151eb @ 0x151eb
void sub_151eb(unsigned int a0,unsigned int a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x98
  unsigned int v10; // stack - 0x50
  unsigned int v11; // stack - 0x4c
  unsigned long v12; // stack - 0x40
  unsigned long v13; // stack - 0x38
  unsigned long v14; // stack - 0x30
  unsigned long v15; // stack - 0x28
  unsigned long v16; // stack - 0x20
  unsigned long v17; // stack - 0x18
  unsigned long v2; // stack - 0x48
  unsigned long v3; // stack - 0x90
  unsigned long v4; // stack - 0x88
  unsigned long v5; // stack - 0x80
  unsigned long v6; // stack - 0x78
  unsigned long v7; // stack - 0x70
  unsigned long v8; // stack - 0x68
  unsigned long v9; // stack - 0x58
  
  v9 = a2;
  v10 = a1;
  v11 = a0;
  sub_13714(&v1,a1);
  v2 = v1;
  v12 = v3;
  v13 = v4;
  v14 = v5;
  v15 = v6;
  v16 = v7;
  v17 = v8;
  sub_135e8(&v2,0x3a,1);
  sub_14c28(v11,v9,0xffffffffffffffff,&v2);
}

// Function: sub_152ad @ 0x152ad
void sub_152ad(unsigned int a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  sub_152e7(a0,a1,a2,a3,0xffffffffffffffff);
}

// Function: sub_152e7 @ 0x152e7
void sub_152e7(unsigned int a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v1 = dat_23880;
  v2 = dat_23888;
  v3 = dat_23890;
  v4 = dat_23898;
  v5 = dat_238a0;
  v6 = dat_238a8;
  v7 = dat_238b0;
  sub_136b2(&v1,a1,a2);
  sub_14c28(a0,a3,a4,&v1);
}

// Function: sub_153a5 @ 0x153a5
void sub_153a5(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_152ad(0,a0,a1,a2);
}

// Function: sub_153d8 @ 0x153d8
void sub_153d8(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  sub_152e7(0,a0,a1,a2,a3);
}

// Function: sub_15416 @ 0x15416
void sub_15416(unsigned int a0,unsigned long a1,unsigned long a2)
{
  sub_14c28(a0,a1,a2,0x23140);
}

// Function: sub_15448 @ 0x15448
void sub_15448(unsigned long a0,unsigned long a1)
{
  sub_15416(0,a0,a1);
}

// Function: sub_15473 @ 0x15473
void sub_15473(unsigned int a0,unsigned long a1)
{
  sub_15416(a0,a1,0xffffffffffffffff);
}

// Function: sub_154a0 @ 0x154a0
void sub_154a0(unsigned long a0)
{
  sub_15473(0,a0);
}

// Function: sub_154c3 @ 0x154c3
void sub_154c3(long a0)
{
  unsigned long v1; // rax
  char *v2; // rax
  
  if (a0) {
    v1 = sub_154a0(a0);
    if (*__errno_location()) // branch-flip
      v2 = gettext("%s: read error");
    else {
      v2 = gettext("%s: end of file");
    }
    error(dat_23118,*__errno_location(),v2,v1); // no-return
  }
  abort(); // no-return
}

// Function: sub_15541 @ 0x15541
unsigned long * sub_15541(unsigned long a0,unsigned long a1)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_18024(0x1038);
  *v1 = a0;
  v1[1] = sub_154c3;
  v1[2] = a1;
  return v1;
}

// Function: sub_1558f @ 0x1558f
unsigned long sub_1558f(unsigned long a0,long a1)
{
  long v1; // rax
  unsigned long v2; // stack - 0x20
  
  v2 = a0;
  do {
    while( true ) {
      if (a1 + a0 <= v2)
        return 1;
      v1 = getrandom(v2,(a1 + a0) - v2,0);
      if (v1 < 0) break;
      v2 += v1;
    }
  } while (*__errno_location() == 4);
  return 0;
}

// Function: sub_15614 @ 0x15614
unsigned long sub_15614(long *a0) // early-return
{
  long v1;
  
  v1 = *a0;
  explicit_bzero(a0,0x1038);
  free(a0);
  if (v1)
    return sub_ff17(v1);
  return 0;
}

// Function: sub_15668 @ 0x15668
long sub_15668(long a0,unsigned long a1)
{
  int v1;
  unsigned long v2;
  void *v3; // stack - 0x18
  long v4; // stack - 0x10
  
  if (a1) { // branch-flip
    v3 = NULL;
    if (a0) {
      v3 = (void *)sub_19bbb(a0,"rb");
      if (!v3)
        return 0;
    }
    v4 = sub_15541(v3,a0);
    if (v3) { // branch-flip
      v2 = 0x1000;
      if (a1 <= 0x1000)
        v2 = a1;
      setvbuf(v3,(char *)(v4 + 0x18),0,v2);
    }
    else {
      *(unsigned long *)(v4 + 0x18) = 0;
      if (0x801 <= a1)
        a1 = 0x800;
      if (sub_1558f(v4 + 0x20,a1) != '\x01') {
        v1 = *__errno_location();
        sub_15614(v4);
        *__errno_location() = v1;
        return 0;
      }
      sub_161e0(v4 + 0x20);
    }
  }
  else {
    v4 = sub_15541(0,0);
  }
  return v4;
}

// Function: sub_15790 @ 0x15790
void sub_15790(long a0,unsigned long a1)
{
  *(unsigned long *)(a0 + 8) = a1;
}

// Function: sub_157af @ 0x157af
void sub_157af(long a0,unsigned long a1)
{
  *(unsigned long *)(a0 + 0x10) = a1;
}

// Function: sub_157ce @ 0x157ce
void sub_157ce(unsigned long *a0,long a1,long a2)
{
  void *v1;
  long v2; // rax
  int v3;
  long v4; // stack - 0x40
  long v5; // stack - 0x38
  
  v4 = a2;
  v5 = a1;
  while( true ) {
    v2 = fread_unlocked(v5,1,v4,*a0);
    v3 = *__errno_location();
    v5 += v2;
    v4 -= v2;
    if (!v4) break;
    v1 = (void *)*a0;
    if (!ferror_unlocked(v1))
      v3 = 0;
    *__errno_location() = v3;
    (*(void *)a0[1])(a0[2]);
  }
}

// Function: sub_1586b @ 0x1586b
void sub_1586b(unsigned long *a0,void *a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x40
  void *v2; // stack - 0x38
  unsigned long v3; // stack - 0x20
  void *v4; // stack - 0x18
  
  v3 = *a0;
  v1 = a2;
  v2 = a1;
  do {
    if (v1 <= v3) {
      memcpy(v2,(void *)((long)a0 + (0x1020 - v3)),v1);
      *a0 = v3 - v1;
      return;
    }
    memcpy(v2,(void *)((long)a0 + (0x1020 - v3)),v3);
    v2 = (void *)(v3 + (long)v2);
    v1 -= v3;
    if (!((unsigned long)v2 & 7)) {
      v4 = v2;
      while (0x800 <= v1) {
        sub_15a8b(&a0[1],v4);
        v4 = (void *)((long)v4 + 0x800);
        v1 -= 0x800;
        if (!v1) {
          *a0 = 0;
          return;
        }
      }
      v2 = v4;
    }
    sub_15a8b(&a0[1],&a0[0x104]);
    v3 = 0x800;
  } while( true );
}

// Function: sub_159b4 @ 0x159b4
void sub_159b4(long *a0,unsigned long a1,unsigned long a2) // return-dupe
{
  if (*a0) {
    sub_157ce(a0,a1,a2);
    return;
  }
  sub_1586b(&a0[3],a1,a2);
}

// Function: sub_15a0f @ 0x15a0f
void sub_15a0f(unsigned long a0)
{
  sub_15614(a0);
}

// Function: sub_15a2d @ 0x15a2d
unsigned long sub_15a2d(unsigned long a0)
{
  return a0;
}

// Function: sub_15a47 @ 0x15a47
unsigned long sub_15a47(long a0,unsigned int a1)
{
  return *(unsigned long *)(a0 + (unsigned long)(a1 & 0x7f8));
}

// Function: sub_15a8b @ 0x15a8b
void sub_15a8b(unsigned long *a0,long *a1)
{
  unsigned long v1;
  unsigned long v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // stack - 0xa8
  unsigned long v6; // stack - 0xa0
  unsigned long *v7; // stack - 0x98
  long *v8; // stack - 0x90
  
  v5 = a0[0x100];
  a0[0x102] = a0[0x102] + 1;
  v6 = a0[0x102] + a0[0x101];
  v7 = a0;
  v8 = a1;
  do {
    v2 = (v7[0x80] - (v5 << 0x15 ^ v5)) - 1;
    v1 = *v7;
    v6 += v2 + sub_15a47(a0,v1);
    *v7 = v6;
    v3 = sub_15a2d(sub_15a47(a0,v6 >> 8) + v1);
    *v8 = v3;
    v2 = v7[0x81] + ((unsigned long)sub_15a2d(v2) >> 5 ^ v2);
    v1 = v7[1];
    v4 = v3 + v2 + sub_15a47(a0,v1);
    v7[1] = v4;
    v3 = sub_15a2d(sub_15a47(a0,v4 >> 8) + v1);
    v8[1] = v3;
    v2 = v7[0x82] + (v2 * 0x1000 ^ v2);
    v1 = v7[2];
    v4 = v3 + v2 + sub_15a47(a0,v1);
    v7[2] = v4;
    v3 = sub_15a2d(sub_15a47(a0,v4 >> 8) + v1);
    v8[2] = v3;
    v5 = v7[0x83] + ((unsigned long)sub_15a2d(v2) >> 0x21 ^ v2);
    v1 = v7[3];
    v2 = v3 + v5 + sub_15a47(a0,v1);
    v7[3] = v2;
    v6 = sub_15a2d(sub_15a47(a0,v2 >> 8) + v1);
    v8[3] = v6;
    v8 = &v8[4];
    v7 = &v7[4];
  } while (v7 < &a0[0x80]);
  do {
    v2 = (v7[-0x80] - (v5 << 0x15 ^ v5)) - 1;
    v1 = *v7;
    v6 += v2 + sub_15a47(a0,v1);
    *v7 = v6;
    v3 = sub_15a2d(sub_15a47(a0,v6 >> 8) + v1);
    *v8 = v3;
    v2 = v7[-0x7f] + ((unsigned long)sub_15a2d(v2) >> 5 ^ v2);
    v1 = v7[1];
    v4 = v3 + v2 + sub_15a47(a0,v1);
    v7[1] = v4;
    v3 = sub_15a2d(sub_15a47(a0,v4 >> 8) + v1);
    v8[1] = v3;
    v2 = v7[-0x7e] + (v2 * 0x1000 ^ v2);
    v1 = v7[2];
    v4 = v3 + v2 + sub_15a47(a0,v1);
    v7[2] = v4;
    v3 = sub_15a2d(sub_15a47(a0,v4 >> 8) + v1);
    v8[2] = v3;
    v5 = v7[-0x7d] + ((unsigned long)sub_15a2d(v2) >> 0x21 ^ v2);
    v1 = v7[3];
    v2 = v3 + v5 + sub_15a47(a0,v1);
    v7[3] = v2;
    v6 = sub_15a2d(sub_15a47(a0,v2 >> 8) + v1);
    v8[3] = v6;
    v8 = &v8[4];
    v7 = &v7[4];
  } while (v7 < &a0[0x100]);
  a0[0x100] = v5;
  a0[0x101] = v6;
}

// Function: sub_161e0 @ 0x161e0
void sub_161e0(long a0)
{
  long v1;
  long v10;
  int v11; // stack - 0x50
  int v12; // stack - 0x4c
  unsigned long v13; // stack - 0x48
  unsigned long v14; // stack - 0x40
  unsigned long v15; // stack - 0x38
  unsigned long v16; // stack - 0x30
  unsigned long v17; // stack - 0x28
  long v18; // stack - 0x20
  long v19; // stack - 0x18
  long v2;
  long v20; // stack - 0x10
  unsigned long v3;
  unsigned long v4;
  unsigned long v5;
  long v6;
  long v7;
  long v8;
  long v9;
  
  v13 = 0x647c4677a2884b7c;
  v14 = 0xb9f8b322c73ac862;
  v15 = 0x8c0ea5053d4712a0;
  v16 = 0xb29b2e824a595524;
  v17 = 0x82f053db8355e0ce;
  v18 = 0x48fe4a0fa5a09315;
  v19 = -0x5167a40d34037613;
  v20 = -0x670a8fb093bb3f55;
  for (v11 = 0; v11 <= 0xff; v11 = v11 + 8) {
    v6 = *(long *)(a0 + (long)(v11 + 1) * 8);
    v7 = *(long *)(a0 + (long)(v11 + 2) * 8);
    v8 = *(long *)(a0 + (long)(v11 + 3) * 8);
    v1 = v17 + *(long *)(a0 + (long)(v11 + 4) * 8);
    v9 = *(long *)(a0 + (long)(v11 + 5) * 8);
    v10 = *(long *)(a0 + (long)(v11 + 6) * 8);
    v20 += *(long *)(a0 + (long)(v11 + 7) * 8);
    v2 = (v13 + *(long *)(a0 + (long)v11 * 8)) - v1;
    v3 = v18 + v9 ^ (unsigned long)sub_15a2d(v20) >> 9;
    v6 = (v14 + v6) - v3;
    v4 = v19 + v10 ^ v2 * 0x200;
    v7 = (v15 + v7) - v4;
    v5 = v20 + v2 ^ (unsigned long)sub_15a2d(v6) >> 0x17;
    v8 = (v16 + v8) - v5;
    v13 = v2 + v6 ^ v7 * 0x8000;
    v1 -= v13;
    v14 = v6 + v7 ^ (unsigned long)sub_15a2d(v8) >> 0xe;
    v2 = v3 - v14;
    v15 = v7 + v8 ^ v1 * 0x100000;
    v19 = v4 - v15;
    v16 = v8 + v1 ^ (unsigned long)sub_15a2d(v2) >> 0x11;
    v18 = v2 + v19;
    v20 = v5 - v16;
    v17 = v1 + v2 ^ v19 * 0x4000;
    v19 += v20;
    *(unsigned long *)(a0 + (long)v11 * 8) = v13;
    *(unsigned long *)(a0 + (long)(v11 + 1) * 8) = v14;
    *(unsigned long *)(a0 + (long)(v11 + 2) * 8) = v15;
    *(unsigned long *)(a0 + (long)(v11 + 3) * 8) = v16;
    *(unsigned long *)(a0 + (long)(v11 + 4) * 8) = v17;
    *(long *)(a0 + (long)(v11 + 5) * 8) = v18;
    *(long *)(a0 + (long)(v11 + 6) * 8) = v19;
    *(long *)(a0 + (long)(v11 + 7) * 8) = v20;
  }
  for (v12 = 0; v12 <= 0xff; v12 = v12 + 8) {
    v6 = *(long *)(a0 + (long)(v12 + 1) * 8);
    v7 = *(long *)(a0 + (long)(v12 + 2) * 8);
    v8 = *(long *)(a0 + (long)(v12 + 3) * 8);
    v1 = v17 + *(long *)(a0 + (long)(v12 + 4) * 8);
    v9 = *(long *)(a0 + (long)(v12 + 5) * 8);
    v10 = *(long *)(a0 + (long)(v12 + 6) * 8);
    v20 += *(long *)(a0 + (long)(v12 + 7) * 8);
    v2 = (v13 + *(long *)(a0 + (long)v12 * 8)) - v1;
    v3 = v18 + v9 ^ (unsigned long)sub_15a2d(v20) >> 9;
    v6 = (v14 + v6) - v3;
    v4 = v19 + v10 ^ v2 * 0x200;
    v7 = (v15 + v7) - v4;
    v5 = v20 + v2 ^ (unsigned long)sub_15a2d(v6) >> 0x17;
    v8 = (v16 + v8) - v5;
    v13 = v2 + v6 ^ v7 * 0x8000;
    v1 -= v13;
    v14 = v6 + v7 ^ (unsigned long)sub_15a2d(v8) >> 0xe;
    v2 = v3 - v14;
    v15 = v7 + v8 ^ v1 * 0x100000;
    v19 = v4 - v15;
    v16 = v8 + v1 ^ (unsigned long)sub_15a2d(v2) >> 0x11;
    v18 = v2 + v19;
    v20 = v5 - v16;
    v17 = v1 + v2 ^ v19 * 0x4000;
    v19 += v20;
    *(unsigned long *)(a0 + (long)v12 * 8) = v13;
    *(unsigned long *)(a0 + (long)(v12 + 1) * 8) = v14;
    *(unsigned long *)(a0 + (long)(v12 + 2) * 8) = v15;
    *(unsigned long *)(a0 + (long)(v12 + 3) * 8) = v16;
    *(unsigned long *)(a0 + (long)(v12 + 4) * 8) = v17;
    *(long *)(a0 + (long)(v12 + 5) * 8) = v18;
    *(long *)(a0 + (long)(v12 + 6) * 8) = v19;
    *(long *)(a0 + (long)(v12 + 7) * 8) = v20;
  }
  *(unsigned long *)(a0 + 0x810) = 0;
  *(unsigned long *)(a0 + 0x808) = *(unsigned long *)(a0 + 0x810);
  *(unsigned long *)(a0 + 0x800) = *(unsigned long *)(a0 + 0x808);
}

// Function: sub_1676c @ 0x1676c
void sub_1676c(unsigned long *a0)
{
  *a0 = 0;
  a0[1] = 0;
  a0[2] = 0;
  _obstack_begin(&a0[3],0,0,dat_22fd0,dat_22fe0);
  _obstack_begin(&a0[0xe],0,0,dat_22fd0,dat_22fe0);
  _obstack_begin(&a0[0x19],0,0,dat_22fd0,dat_22fe0);
}

// Function: sub_1682e @ 0x1682e
void sub_1682e(long a0)
{
  _obstack_free(a0 + 0x18,0);
  _obstack_free(a0 + 0x70,0);
  _obstack_free(a0 + 200,0);
}

// Function: sub_16967 @ 0x16967
void sub_16967(long *a0)
{
  long v1; // stack - 0x70
  long *v10; // stack - 0x28
  unsigned long v11; // stack - 0x20
  long *v12; // stack - 0x18
  long *v2; // stack - 0x68
  long *v3; // stack - 0x60
  long v4; // stack - 0x58
  long v5; // stack - 0x50
  long *v6; // stack - 0x48
  long *v7; // stack - 0x40
  long *v8; // stack - 0x38
  long *v9; // stack - 0x30
  
  v2 = &a0[3];
  v1 = (a0[6] - a0[5]) + -1;
  v3 = &a0[3];
  v4 = a0[5];
  if (v4 == a0[6])
    *(unsigned char *)&a0[0xd] = *(unsigned char *)&a0[0xd] | 2;
  a0[6] = ~a0[9] & a0[6] + a0[9];
  if ((unsigned long)(a0[7] - a0[4]) < (unsigned long)(a0[6] - a0[4]))
    a0[6] = a0[7];
  a0[5] = a0[6];
  v6 = &a0[0xe];
  v5 = v4;
  v7 = v6;
  if ((unsigned long)(a0[0x12] - a0[0x11]) <= 7)
    _obstack_newchunk(v6,8);
  v8 = v6;
  v9 = (long *)v6[3];
  *v9 = v5;
  v6[3] = v6[3] + 8;
  v10 = &a0[0x19];
  v11 = 8;
  v12 = v10;
  if ((unsigned long)(a0[0x1d] - a0[0x1c]) < 8)
    _obstack_newchunk(v10,8);
  memcpy((void *)v10[3],&v1,v11);
  v10[3] = v10[3] + v11;
  *a0 = *a0 + 1;
}

// Function: sub_16b8b @ 0x16b8b
unsigned long sub_16b8b(void *a0,long a1)
{
  char *v1;
  long v2;
  int v3; // eax
  
  while( true ) {
    v3 = fgetc(a0);
    if (v3 == -1) break;
    if (*(long *)(a1 + 0x38) == *(long *)(a1 + 0x30))
      _obstack_newchunk(a1 + 0x18,1);
    v1 = *(char **)(a1 + 0x30);
    *(char **)(a1 + 0x30) = &v1[1];
    *v1 = (char)v3;
    if (!v3)
      sub_16967(a1);
  }
  if (*(long *)(a1 + 0x30) != *(long *)(a1 + 0x28)) {
    if (*(long *)(a1 + 0x38) == *(long *)(a1 + 0x30))
      _obstack_newchunk(a1 + 0x18,1);
    v1 = *(char **)(a1 + 0x30);
    *(char **)(a1 + 0x30) = &v1[1];
    *v1 = 0;
    sub_16967(a1);
  }
  if ((unsigned long)(*(long *)(a1 + 0x90) - *(long *)(a1 + 0x88)) <= 7)
    _obstack_newchunk(a1 + 0x70,8);
  **(unsigned long **)(a1 + 0x88) = 0;
  *(long *)(a1 + 0x88) = *(long *)(a1 + 0x88) + 8;
  v2 = *(long *)(a1 + 0x80);
  if (v2 == *(long *)(a1 + 0x88))
    *(unsigned char *)(a1 + 0xc0) = *(unsigned char *)(a1 + 0xc0) | 2;
  *(unsigned long *)(a1 + 0x88) = ~*(unsigned long *)(a1 + 0xa0) & *(long *)(a1 + 0x88) + *(long *)(a1 + 0xa0);
  if ((unsigned long)(*(long *)(a1 + 0x90) - *(long *)(a1 + 0x78)) < (unsigned long)(*(long *)(a1 + 0x88) - *(long *)(a1 + 0x78)))
    *(unsigned long *)(a1 + 0x88) = *(unsigned long *)(a1 + 0x90);
  *(unsigned long *)(a1 + 0x80) = *(unsigned long *)(a1 + 0x88);
  *(long *)(a1 + 8) = v2;
  v2 = *(long *)(a1 + 0xd8);
  if (v2 == *(long *)(a1 + 0xe0))
    *(unsigned char *)(a1 + 0x118) = *(unsigned char *)(a1 + 0x118) | 2;
  *(unsigned long *)(a1 + 0xe0) = ~*(unsigned long *)(a1 + 0xf8) & *(long *)(a1 + 0xe0) + *(long *)(a1 + 0xf8);
  if ((unsigned long)(*(long *)(a1 + 0xe8) - *(long *)(a1 + 0xd0)) < (unsigned long)(*(long *)(a1 + 0xe0) - *(long *)(a1 + 0xd0)))
    *(unsigned long *)(a1 + 0xe0) = *(unsigned long *)(a1 + 0xe8);
  *(unsigned long *)(a1 + 0xd8) = *(unsigned long *)(a1 + 0xe0);
  *(long *)(a1 + 0x10) = v2;
  v3 = ferror(a0);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v3) >> 8),v3 == 0);
}

// Function: sub_16eff @ 0x16eff
char * sub_16eff(int a0)
{
  return setlocale(a0,NULL);
}

// Function: sub_16f27 @ 0x16f27
unsigned long sub_16f27(unsigned int a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_16eff(a0);
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

// Function: sub_16fdf @ 0x16fdf
void sub_16fdf(unsigned int a0,unsigned long a1,unsigned long a2)
{
  sub_16f27(a0,a1,a2);
}

// Function: sub_1700d @ 0x1700d
void sub_1700d(unsigned int a0)
{
  sub_16eff(a0);
}

// Function: sub_17028 @ 0x17028
void sub_17028(unsigned long a0)
{
  sub_1767e(mkstemp(a0));
}

// Function: sub_1704d @ 0x1704d
void sub_1704d(unsigned long a0,unsigned int a1)
{
  sub_19b20(mkostemp(a0,a1),a1);
}

// Function: sub_17081 @ 0x17081
unsigned int sub_17081(char *a0,char *a1,char a2) // early-return x3
{
  char *v1; // stack - 0x18
  char *v2; // stack - 0x10
  
  if ((a2 != *a0) || (v1 = a1, v2 = a0, a2 != *a1)) {
    v2 = &a0[1];
    if (a2 == *a0) goto label_17179;
    v1 = &a1[1];
    if (a2 != *a1)
      return 0;
  }
  else {
    while( true ) {
      v2 = &v2[1];
      v1 = &v1[1];
      if (*v2 != *v1) break;
      if (10 <= (unsigned int)((int)*v2 - 0x30U))
        return 0;
    }
    if (((unsigned int)((int)*v2 - 0x30U) <= 9) && ((unsigned int)((int)*v1 - 0x30U) <= 9))
      return (int)*v2 - (int)*v1;
    if ((unsigned int)((int)*v2 - 0x30U) <= 9) {
label_17179:
      for (; *v2 == '0'; v2 = &v2[1]) {
      }
      return (unsigned int)((unsigned int)((int)*v2 - 0x30U) <= 9);
    }
    if (10 <= (unsigned int)((int)*v1 - 0x30U))
      return 0;
  }
  for (; *v1 == '0'; v1 = &v1[1]) {
  }
  return -(unsigned int)((unsigned int)((int)*v1 - 0x30U) <= 9);
}

// Function: sub_171e6 @ 0x171e6
unsigned long sub_171e6(unsigned char *a0,unsigned char *a1,unsigned int a2,unsigned int a3) // early-return x8
{
  char v1;
  unsigned long v10; // stack - 0x10
  unsigned int v2;
  unsigned int v3; // eax
  unsigned long v4; // rax
  unsigned char *v5; // stack - 0x38
  unsigned char *v6; // stack - 0x30
  unsigned char v7; // stack - 0x1e
  unsigned char v8; // stack - 0x1d
  unsigned long v9; // stack - 0x18
  
  v7 = *a0;
  v8 = *a1;
  v1 = (char)a2;
  v6 = a0;
  v5 = a1;
  if (v7 != 0x2d) { // branch-flip
    if (v8 != 0x2d) { // branch-flip
      while ((v7 == 0x30 || (v3 = (unsigned int)v7, a3 == v3))) {
        v6 = &v6[1];
        v7 = *v6;
      }
      while ((v8 == 0x30 || (v3 = (unsigned int)v8, a3 == v3))) {
        v5 = &v5[1];
        v8 = *v5;
      }
      while ((v7 == v8 && (v3 = (unsigned int)v7, v3 = v3 - 0x30, v3 <= 9))) {
        do {
          v6 = &v6[1];
          v7 = *v6;
          v3 = (unsigned int)v7;
        } while (a3 == v3);
        do {
          v5 = &v5[1];
          v8 = *v5;
          v3 = (unsigned int)v8;
        } while (a3 == v3);
      }
      v3 = (unsigned int)v7;
      if (((a2 != v3) || (v3 = (unsigned int)v8, v3 = v3 - 0x30, v3 <= 9)) && ((v3 = (unsigned int)v8, a2 != v3 || (v3 = (unsigned int)v7, v3 = v3 - 0x30, v3 <= 9)))) {
        v3 = (unsigned int)v7;
        v3 -= v8;
        v2 = v3;
        v9 = 0;
        while (v3 = (unsigned int)v7, v3 = v3 - 0x30, v3 <= 9) {
          do {
            v6 = &v6[1];
            v7 = *v6;
            v3 = (unsigned int)v7;
          } while (a3 == v3);
          v9 += 1;
        }
        v10 = 0;
        while (v3 = (unsigned int)v8, v3 = v3 - 0x30, v3 <= 9) {
          do {
            v5 = &v5[1];
            v8 = *v5;
            v3 = (unsigned int)v8;
          } while (a3 == v3);
          v10 += 1;
        }
        if (v9 != v10) {
          if (v9 < v10) {
            v4 = 0xffffffff;
            return v4;
          }
          v4 = 1;
          return v4;
        }
        if (!v9) {
          v4 = 0;
          return v4;
        }
        v4 = (unsigned long)v2;
      }
      else {
        v4 = sub_17081(v6,v5,(int)v1);
      }
    }
    else {
      do {
        do {
          v5 = &v5[1];
          v8 = *v5;
        } while (v8 == 0x30);
        v3 = (unsigned int)v8;
      } while (a3 == v3);
      v3 = (unsigned int)v8;
      if (a2 == v3) {
        do {
          v5 = &v5[1];
          v8 = *v5;
        } while (v8 == 0x30);
      }
      v3 = (unsigned int)v8;
      v3 -= 0x30;
      if (v3 <= 9) {
        v4 = 1;
        return v4;
      }
      while ((v7 == 0x30 || (v3 = (unsigned int)v7, a3 == v3))) {
        v6 = &v6[1];
        v7 = *v6;
      }
      v3 = (unsigned int)v7;
      if (a2 == v3) {
        do {
          v6 = &v6[1];
          v7 = *v6;
        } while (v7 == 0x30);
      }
      v3 = (unsigned int)v7;
      v3 -= 0x30;
      v4 = (unsigned long)(v3 <= 9);
    }
  }
  else {
    do {
      do {
        v6 = &v6[1];
        v7 = *v6;
      } while (v7 == 0x30);
      v3 = (unsigned int)v7;
    } while (a3 == v3);
    if (v8 != 0x2d) { // branch-flip
      v3 = (unsigned int)v7;
      if (a2 == v3) {
        do {
          v6 = &v6[1];
          v7 = *v6;
        } while (v7 == 0x30);
      }
      v3 = (unsigned int)v7;
      v3 -= 0x30;
      if (v3 <= 9) {
        v4 = 0xffffffff;
        return v4;
      }
      while ((v8 == 0x30 || (v3 = (unsigned int)v8, a3 == v3))) {
        v5 = &v5[1];
        v8 = *v5;
      }
      v3 = (unsigned int)v8;
      if (a2 == v3) {
        do {
          v5 = &v5[1];
          v8 = *v5;
        } while (v8 == 0x30);
      }
      v3 = (unsigned int)v8;
      v3 -= 0x30;
      v3 = (unsigned int)(v3 <= 9);
      v3 = -v3;
      v4 = (unsigned long)v3;
    }
    else {
      do {
        do {
          v5 = &v5[1];
          v8 = *v5;
        } while (v8 == 0x30);
        v3 = (unsigned int)v8;
      } while (a3 == v3);
      while ((v7 == v8 && (v3 = (unsigned int)v7, v3 = v3 - 0x30, v3 <= 9))) {
        do {
          v6 = &v6[1];
          v7 = *v6;
          v3 = (unsigned int)v7;
        } while (a3 == v3);
        do {
          v5 = &v5[1];
          v8 = *v5;
          v3 = (unsigned int)v8;
        } while (a3 == v3);
      }
      v3 = (unsigned int)v7;
      if (((a2 != v3) || (v3 = (unsigned int)v8, v3 = v3 - 0x30, v3 <= 9)) && ((v3 = (unsigned int)v8, a2 != v3 || (v3 = (unsigned int)v7, v3 = v3 - 0x30, v3 <= 9)))) {
        v3 = (unsigned int)v8;
        v3 -= v7;
        v2 = v3;
        v9 = 0;
        while (v3 = (unsigned int)v7, v3 = v3 - 0x30, v3 <= 9) {
          do {
            v6 = &v6[1];
            v7 = *v6;
            v3 = (unsigned int)v7;
          } while (a3 == v3);
          v9 += 1;
        }
        v10 = 0;
        while (v3 = (unsigned int)v8, v3 = v3 - 0x30, v3 <= 9) {
          do {
            v5 = &v5[1];
            v8 = *v5;
            v3 = (unsigned int)v8;
          } while (a3 == v3);
          v10 += 1;
        }
        if (v9 != v10) {
          if (v9 < v10) {
            v4 = 1;
            return v4;
          }
          v4 = 0xffffffff;
          return v4;
        }
        if (!v9) {
          v4 = 0;
          return v4;
        }
        v4 = (unsigned long)v2;
      }
      else {
        v4 = sub_17081(v5,v6,(int)v1);
      }
    }
  }
  return v4;
}

// Function: sub_1764c @ 0x1764c
void sub_1764c(unsigned long a0,unsigned long a1,unsigned int a2,unsigned int a3)
{
  sub_171e6(a0,a1,a2,a3);
}

// Function: sub_1767e @ 0x1767e
int sub_1767e(int a0)
{
  int v1;
  int v2; // stack - 0x1c
  
  v2 = a0;
  if ((0 <= a0) && (a0 <= 2)) {
    v2 = sub_1a92b(a0);
    v1 = *__errno_location();
    close(a0);
    *__errno_location() = v1;
  }
  return v2;
}

// Function: sub_176cf @ 0x176cf
void sub_176cf(void *a0,long a1,unsigned long a2,unsigned long a3,unsigned long *a4,unsigned long a5) // return-dupe x10
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  unsigned long v4;
  unsigned long v5;
  unsigned long v6;
  unsigned long v7;
  unsigned long v8;
  unsigned long v9;
  
  if (a1) // branch-flip
    fprintf(a0,"%s (%s) %s\n",a1,a2,a3);
  else {
    fprintf(a0,"%s %s\n",a2,a3);
  }
  fprintf(a0,"Copyright %s %d Free Software Foundation, Inc.",gettext("(C)"),0x7e6);
  fputs_unlocked("\n",a0);
  fprintf(a0,gettext("License GPLv3+: GNU GPL version 3 or later <%s>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n"),"https://gnu.org/licenses/gpl.html");
  fputs_unlocked("\n",a0);
  switch(a5) {
    case 0:
      return;
    case 1:
      v1 = *a4;
      fprintf(a0,gettext("Written by %s.\n"),v1);
      return;
    case 2:
      v1 = a4[1];
      v2 = *a4;
      fprintf(a0,gettext("Written by %s and %s.\n"),v2,v1);
      return;
    case 3:
      v1 = a4[2];
      v2 = a4[1];
      v3 = *a4;
      fprintf(a0,gettext("Written by %s, %s, and %s.\n"),v3,v2,v1);
      return;
    case 4:
      v1 = a4[3];
      v2 = a4[2];
      v3 = a4[1];
      v4 = *a4;
      fprintf(a0,gettext("Written by %s, %s, %s,\nand %s.\n"),v4,v3,v2,v1);
      return;
    case 5:
      v1 = a4[4];
      v2 = a4[3];
      v3 = a4[2];
      v4 = a4[1];
      v5 = *a4;
      fprintf(a0,gettext("Written by %s, %s, %s,\n%s, and %s.\n"),v5,v4,v3,v2,v1);
      return;
    case 6:
      v1 = a4[5];
      v2 = a4[4];
      v3 = a4[3];
      v4 = a4[2];
      v5 = a4[1];
      v6 = *a4;
      fprintf(a0,gettext("Written by %s, %s, %s,\n%s, %s, and %s.\n"),v6,v5,v4,v3,v2,v1);
      return;
    case 7:
      v1 = a4[6];
      v2 = a4[5];
      v3 = a4[4];
      v4 = a4[3];
      v5 = a4[2];
      v6 = a4[1];
      v7 = *a4;
      fprintf(a0,gettext("Written by %s, %s, %s,\n%s, %s, %s, and %s.\n"),v7,v6,v5,v4,v3,v2,v1);
      return;
    case 8:
      v1 = a4[7];
      v2 = a4[6];
      v3 = a4[5];
      v4 = a4[4];
      v5 = a4[3];
      v6 = a4[2];
      v7 = a4[1];
      v8 = *a4;
      fprintf(a0,gettext("Written by %s, %s, %s,\n%s, %s, %s, %s,\nand %s.\n"),v8,v7,v6,v5,v4,v3,v2,v1);
      return;
    case 9:
      v1 = a4[8];
      v2 = a4[7];
      v3 = a4[6];
      v4 = a4[5];
      v5 = a4[4];
      v6 = a4[3];
      v7 = a4[2];
      v8 = a4[1];
      v9 = *a4;
      fprintf(a0,gettext("Written by %s, %s, %s,\n%s, %s, %s, %s,\n%s, and %s.\n"),v9,v8,v7,v6,v5,v4,v3,v2,v1);
      return;
    default:
      v1 = a4[8];
      v2 = a4[7];
      v3 = a4[6];
      v4 = a4[5];
      v5 = a4[4];
      v6 = a4[3];
      v7 = a4[2];
      v8 = a4[1];
      v9 = *a4;
      fprintf(a0,gettext("Written by %s, %s, %s,\n%s, %s, %s, %s,\n%s, %s, and others.\n"),v9,v8,v7,v6,v5,v4,v3,v2,v1);
      return;
    
  }
}

// Function: sub_17ccb @ 0x17ccb
void sub_17ccb(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long a4)
{
  long v1; // stack - 0x10
  
  for (v1 = 0; *(long *)(a4 + v1 * 8); v1 = v1 + 1) {
  }
  sub_176cf(a0,a1,a2,a3,a4,v1);
}

// Function: sub_17d3e @ 0x17d3e
void sub_17d3e(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
{
  long *v1;
  long v2 [11];
  unsigned long v3; // stack - 0x70
  
  for (v3 = 0; v3 <= 9; v3 = v3 + 1) {
    if (0x30 <= *a4) { // branch-flip
      v1 = *(long **)&a4[2];
      *(long **)&a4[2] = &v1[1];
    }
    else {
      v1 = (long *)((unsigned long)*a4 + *(long *)&a4[4]);
      *a4 = *a4 + 8;
    }
    v2[v3] = *v1;
    if (!v2[v3]) break;
  }
  sub_176cf(a0,a1,a2,a3,v2,v3);
}

// Function: sub_17e44 @ 0x17e44
void sub_17e44(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,unsigned long a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
{
  char v1; // al
  unsigned long v10; // stack - 0x78
  unsigned long v11; // stack - 0x68
  unsigned long v12; // stack - 0x58
  unsigned long v13; // stack - 0x48
  unsigned long v14; // stack - 0x38
  unsigned long v15; // stack - 0x28
  unsigned long v16; // stack - 0x18
  char v2 [32];
  unsigned int v3; // stack - 0xd8
  unsigned int v4; // stack - 0xd4
  char *v5; // stack - 0xd0
  char *v6; // stack - 0xc8
  unsigned long v7; // stack - 0x98
  unsigned long v8; // stack - 0x90
  unsigned long v9; // stack - 0x88
  
  if (v1) {
    v9 = a0;
    v10 = a1;
    v11 = a2;
    v12 = a3;
    v13 = a4;
    v14 = a5;
    v15 = a6;
    v16 = a7;
  }
  v3 = 0x20;
  v4 = 0x30;
  v5 = &Stack0000000000000008;
  v6 = v2;
  v7 = a12;
  v8 = a13;
  sub_17d3e(a8,a9,a10,a11,&v3);
}

// Function: sub_17f28 @ 0x17f28
void sub_17f28(void)
{
  fputs_unlocked("\n",stdout);
  printf(gettext("Report bugs to: %s\n"),"bug-coreutils@gnu.org");
  printf(gettext("%s home page: <%s>\n"),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  printf(gettext("General help using GNU software: <%s>\n"),"https://www.gnu.org/gethelp/");
}

// Function: sub_17fd1 @ 0x17fd1
void sub_17fd1(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_1810a(a0,a1,a2);
}

// Function: sub_18002 @ 0x18002
long sub_18002(long a0)
{
  if (!a0)
    sub_1865c(); // no-return
  return a0;
}

// Function: sub_18024 @ 0x18024
void sub_18024(unsigned long a0)
{
  sub_18002(malloc(a0));
}

// Function: sub_1804a @ 0x1804a
void sub_1804a(unsigned long a0)
{
  sub_18002(sub_19cbf(a0));
}

// Function: sub_18070 @ 0x18070
void sub_18070(unsigned long a0)
{
  sub_18024(a0);
}

// Function: sub_1808e @ 0x1808e
void * sub_1808e(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = realloc(a0,a1);
  if ((!v1) && ((!a0 || (a1))))
    sub_1865c(); // no-return
  return v1;
}

// Function: sub_180d9 @ 0x180d9
void sub_180d9(unsigned long a0,unsigned long a1)
{
  sub_18002(sub_19cdd(a0,a1));
}

// Function: sub_1810a @ 0x1810a
void * sub_1810a(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = reallocarray(a0,a1,a2);
  if ((!v1) && ((!a0 || ((a1 && (a2))))))
    sub_1865c(); // no-return
  return v1;
}

// Function: sub_18164 @ 0x18164
void sub_18164(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_18002(sub_19d3d(a0,a1,a2));
}

// Function: sub_1819d @ 0x1819d
void sub_1819d(unsigned long a0,unsigned long a1)
{
  sub_1810a(0,a0,a1);
}

// Function: sub_181c8 @ 0x181c8
void sub_181c8(unsigned long a0,unsigned long a1)
{
  sub_18164(0,a0,a1);
}

// Function: sub_181f3 @ 0x181f3
void sub_181f3(unsigned long a0,unsigned long a1)
{
  sub_18221(a0,a1,1);
}

// Function: sub_18221 @ 0x18221
unsigned long sub_18221(long a0,unsigned long *a1,unsigned long a2)
{
  char v1 [16];
  long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  bool v5; // cf
  unsigned long v6; // stack - 0x18
  
  v6 = *a1;
  if (a0) { // branch-flip
    v3 = (v6 >> 1) + 1;
    v5 = CARRY8(v3,v6);
    v6 = v3 + v6;
    if (v5)
      sub_1865c(); // no-return
  }
  else if (!v6) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v2 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0);
    v6 = v2 + (unsigned long)(v2 == 0);
  }
  v4 = sub_1810a(a0,v6,a2);
  *a1 = v6;
  return v4;
}

// Function: sub_18300 @ 0x18300
unsigned long sub_18300(long a0,long *a1,long a2,long a3,long a4) // ternary
{
  long v1;
  long v2; // rax
  unsigned long v3; // rax
  long v4; // stack - 0x30
  long v5; // stack - 0x28
  
  v1 = *a1;
  v4 = v1 + (v1 >> 1);
  if (SCARRY8(v1,v1 >> 1))
    v4 = 0x7fffffffffffffff;
  if ((0 <= a3) && (a3 < v4))
    v4 = a3;
  v5 = v4 * a4;
  if (SEXT816(v5) != SEXT816(v4) * SEXT816(a4)) // branch-flip
    v2 = 0x7fffffffffffffff;
  else {
    v2 = (0x80 <= v5) ? 0 : 0x80; // branch-flip
  }
  if (v2) {
    v4 = v2 / a4;
    v5 = v2 - v2 % a4;
  }
  if (!a0)
    *a1 = 0;
  if ((v4 - v1 < a2) && ((v4 = a2 + v1, SCARRY8(a2,v1) || (((0 <= a3 && (a3 < v4)) || (v5 = v4 * a4, SEXT816(v5) != SEXT816(v4) * SEXT816(a4)))))))
    sub_1865c(); // no-return
  v3 = sub_1808e(a0,v5);
  *a1 = v4;
  return v3;
}

// Function: sub_184bc @ 0x184bc
void sub_184bc(unsigned long a0)
{
  sub_18502(a0,1);
}

// Function: sub_184df @ 0x184df
void sub_184df(unsigned long a0)
{
  sub_18533(a0,1);
}

// Function: sub_18502 @ 0x18502
void sub_18502(unsigned long a0,unsigned long a1)
{
  sub_18002(calloc(a0,a1));
}

// Function: sub_18533 @ 0x18533
void sub_18533(unsigned long a0,unsigned long a1)
{
  sub_18002(sub_19d14(a0,a1));
}

// Function: sub_18564 @ 0x18564
void sub_18564(void *a0,unsigned long a1)
{
  memcpy((void *)sub_18024(a1),a0,a1);
}

// Function: sub_1859c @ 0x1859c
void sub_1859c(void *a0,unsigned long a1)
{
  memcpy((void *)sub_1804a(a1),a0,a1);
}

// Function: sub_185dc @ 0x185dc
void sub_185dc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_1804a(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1);
}

// Function: sub_1862b @ 0x1862b
void sub_1862b(char *a0)
{
  sub_18564(a0,strlen(a0) + 1);
}

// Function: sub_1865c @ 0x1865c
void sub_1865c(void)
{
  error(dat_23118,0,"%s",gettext("memory exhausted")); // no-return
}

// Function: sub_1869c @ 0x1869c
void sub_1869c(int a0)
{
  error(0,a0,gettext("string comparison failed")); // no-return
}

// Function: sub_18783 @ 0x18783
unsigned int sub_18783(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  int v1;
  unsigned int v2; // eax
  
  v2 = sub_19f53(a0,a1,a2,a3);
  v1 = *__errno_location();
  if (v1)
    sub_1869c(v1,a0,a1,a2,a3);
  return v2;
}

// Function: sub_187ec @ 0x187ec
unsigned int sub_187ec(unsigned long a0,long a1,unsigned long a2,long a3)
{
  int v1;
  unsigned int v2; // eax
  
  v2 = sub_1a033(a0,a1,a2,a3);
  v1 = *__errno_location();
  if (v1)
    sub_1869c(v1,a0,a1 + -1,a2,a3 + -1);
  return v2;
}

// Function: sub_18863 @ 0x18863
unsigned long sub_18863(double a0)
{
  unsigned long v1; // stack - 0x28
  char v2 [16];
  unsigned long v3; // stack - 0x20
  
  if (dat_1d980 <= a0) {
    do {
      pause();
    } while (*__errno_location() == 4);
  }
  v2 = sub_1951a();
  v3 = SUB168(v2,8);
  v1 = SUB168(v2,0);
  do {
    *__errno_location() = 0;
    if (!sub_1a0a0(&v1,&v1))
      return 0;
  } while ((*__errno_location() == 4) || (!*__errno_location()));
  return 0xffffffff;
}

// Function: sub_18915 @ 0x18915
void sub_18915(unsigned int a0,int a1,char a2,long a3,unsigned long a4,int a5)
{
  char v1; // stack - 0x12
  long v2; // fs_offset
  char *v3; // stack - 0x30
  char *v4; // stack - 0x28
  char *v5; // stack - 0x20
  char v6; // stack - 0x11
  unsigned long v7; // stack - 0x10
  
  v7 = *(unsigned long *)(v2 + 0x28);
  v3 = "--";
  if (a0 != 4) { // branch-flip
    if (5 <= a0)
      abort(); // no-return, return-dupe
    if (a0 != 1) { // branch-flip
      if ((!a0) || (2 <= a0 - 2)) {
        abort();
      }
      v4 = "invalid suffix in %s%s argument \'%s\'";
    }
    else {
      v4 = "%s%s argument \'%s\' too large";
    }
  }
  else {
    v4 = "invalid %s%s argument \'%s\'";
  }
  if (0 <= a1) // branch-flip
    v5 = *(char **)(a3 + (long)a1 * 0x20);
  else {
    v3 = &"--"[-(long)a1];
    v6 = 0;
    v5 = &v1;
    v1 = a2;
  }
  error(a5,0,gettext(v4),v3,v5,a4); // no-return
}

// Function: sub_18a33 @ 0x18a33
void sub_18a33(unsigned int a0,unsigned int a1,char a2,unsigned long a3,unsigned long a4)
{
  sub_18915(a0,a1,(int)a2,a3,a4,dat_23118);
  abort(); // no-return
}

// Function: sub_18a7d @ 0x18a7d
bool sub_18a7d(unsigned long *a0,int a1)
{
  char v1 [16];
  char v2 [16];
  bool v3;
  
  v3 = 0;
  if (((long)a1 <= -1) && (*a0))
    v3 = 1;
  v1._8_8_ = 0;
  v1._0_8_ = (long)a1;
  v2._8_8_ = 0;
  v2._0_8_ = *a0;
  if (SUB168(v1._0_16_ * v2._0_16_,8))
    v3 = 1;
  if (!v3)
    *a0 = SUB168(v1._0_16_ * v2._0_16_,0);
  else {
    *a0 = 0xffffffffffffffff;
  }
  return v3;
}

// Function: sub_18b0f @ 0x18b0f
unsigned int sub_18b0f(unsigned long a0,unsigned int a1,int a2)
{
  int v1;
  int v2; // stack - 0x28
  unsigned int v3; // stack - 0xc
  
  v3 = 0;
  v2 = a2;
  while (v1 = v2 + -1, v2) {
    v3 |= sub_18a7d(a0,a1);
    v2 = v1;
  }
  return v3;
}

// Function: sub_18b54 @ 0x18b54
unsigned int sub_18b54(unsigned char *a0,long *a1,int a2,unsigned long *a3,char *a4) // early-return x2
{
  char v1;
  long *v10; // stack - 0x18
  long v2;
  unsigned long v3; // stack - 0x28
  unsigned char v4; // stack - 0x41
  unsigned int v5; // stack - 0x40
  unsigned int v6; // stack - 0x3c
  int v7; // stack - 0x38
  unsigned int v8; // stack - 0x34
  unsigned char *v9; // stack - 0x20
  
  v5 = 0;
  if ((a2 < 0) || (0x25 <= a2))
    __assert_fail("0 <= strtol_base && strtol_base <= 36","lib/xstrtol.c",0x55,"xstrtoumax"); // no-return
  if (!a1)
    a1 = &v2;
  v10 = a1;
  *__errno_location() = 0;
  v4 = *a0;
  v9 = a0;
  while (*(unsigned short *)((unsigned long)v4 * 2 + *(long *)__ctype_b_loc()) & 0x2000) {
    v9 = &v9[1];
    v4 = *v9;
  }
  if (v4 == 0x2d)
    return 4;
  v3 = strtoumax(a0,v10,a2);
  if (a0 != (unsigned char *)*v10) { // branch-flip
    if (*__errno_location()) {
      if (*__errno_location() != 0x22)
        return 4;
      v5 = 1;
    }
  }
  else {
    if (((!a4) || (!*(char *)*v10)) || (v1 = *(char *)*v10, !strchr(a4,(int)v1)))
      return 4;
    v3 = 1;
  }
  if (!a4) {
    *a3 = v3;
    return v5;
  }
  if (*(char *)*v10) {
    v6 = 0x400;
    v7 = 1;
    v1 = *(char *)*v10;
    if (!strchr(a4,(int)v1)) {
      *a3 = v3;
      return v5 | 2;
    }
    if ((((unsigned int)((int)*(char *)*v10 - 0x45U) <= 0x2f) && (0x814400308945U >> ((unsigned char)((int)*(char *)*v10 - 0x45U) & 0x3f) & 1)) && (strchr(a4,0x30))) {
      v1 = *(char *)(*v10 + 1);
      if (v1 != 'i') { // branch-flip
        if ((v1 <= 'i') && ((v1 == 'B' || (v1 == 'D')))) {
          v6 = 1000;
          v7 = 2;
        }
      }
      else if (*(char *)(*v10 + 2) == 'B')
        v7 = 3;
    }
    switch(*(char *)*v10) {
      case 0x42:
        v8 = sub_18a7d(&v3,0x400);
        break;
      default:
        *a3 = v3;
        return v5 | 2;
      case 0x45:
        v8 = sub_18b0f(&v3,v6,6);
        break;
      case 0x47:
      case 0x67:
        v8 = sub_18b0f(&v3,v6,3);
        break;
      case 0x4b:
      case 0x6b:
        v8 = sub_18b0f(&v3,v6,1);
        break;
      case 0x4d:
      case 0x6d:
        v8 = sub_18b0f(&v3,v6,2);
        break;
      case 0x50:
        v8 = sub_18b0f(&v3,v6,5);
        break;
      case 0x54:
      case 0x74:
        v8 = sub_18b0f(&v3,v6,4);
        break;
      case 0x59:
        v8 = sub_18b0f(&v3,v6,8);
        break;
      case 0x5a:
        v8 = sub_18b0f(&v3,v6,7);
        break;
      case 0x62:
        v8 = sub_18a7d(&v3,0x200);
        break;
      case 99:
        v8 = 0;
        break;
      case 0x77:
        v8 = sub_18a7d(&v3,2);
      
    }
    v5 |= v8;
    *v10 = *v10 + (long)v7;
    if (*(char *)*v10)
      v5 |= 2;
  }
  *a3 = v3;
  return v5;
}

// Function: sub_18fb2 @ 0x18fb2
unsigned long sub_18fb2(unsigned long a0,unsigned char a1)
{
  return a0 << (a1 & 0x3f) | a0 >> 0x40 - (a1 & 0x3f);
}

// Function: sub_18fd2 @ 0x18fd2
unsigned long sub_18fd2(unsigned long a0,unsigned char a1)
{
  return a0 >> (a1 & 0x3f) | a0 << 0x40 - (a1 & 0x3f);
}

// Function: sub_18ff2 @ 0x18ff2
unsigned int sub_18ff2(unsigned int a0,unsigned char a1)
{
  return a0 << (a1 & 0x1f) | a0 >> 0x20 - (a1 & 0x1f);
}

// Function: sub_1900e @ 0x1900e
unsigned int sub_1900e(unsigned int a0,unsigned char a1)
{
  return a0 >> (a1 & 0x1f) | a0 << 0x20 - (a1 & 0x1f);
}

// Function: sub_1902a @ 0x1902a
unsigned long sub_1902a(unsigned long a0,unsigned char a1)
{
  return a0 >> (0x40 - a1 & 0x3f) | a0 << (a1 & 0x3f);
}

// Function: sub_19063 @ 0x19063
unsigned long sub_19063(unsigned long a0,unsigned char a1)
{
  return a0 << (0x40 - a1 & 0x3f) | a0 >> (a1 & 0x3f);
}

// Function: sub_1909c @ 0x1909c
unsigned int sub_1909c(unsigned short a0,unsigned char a1)
{
  return (unsigned int)(a0 >> (0x10 - a1 & 0x1f)) | (unsigned int)a0 << (a1 & 0x1f);
}

// Function: sub_190d2 @ 0x190d2
unsigned int sub_190d2(unsigned short a0,unsigned char a1)
{
  return (unsigned int)a0 << (0x10 - a1 & 0x1f) | (unsigned int)(a0 >> (a1 & 0x1f));
}

// Function: sub_19108 @ 0x19108
unsigned int sub_19108(unsigned char a0,unsigned char a1)
{
  return (unsigned int)(a0 >> (8 - a1 & 0x1f)) | (unsigned int)a0 << (a1 & 0x1f);
}

// Function: sub_1913d @ 0x1913d
unsigned int sub_1913d(unsigned char a0,unsigned char a1)
{
  return (unsigned int)a0 << (8 - a1 & 0x1f) | (unsigned int)(a0 >> (a1 & 0x1f));
}

// Function: sub_19172 @ 0x19172
unsigned long sub_19172(int a0) // return-dupe
{
  if (0x7a < a0)
    return 0;
  if (0x61 <= a0)
    return 1;
  if (0x39 < a0) {
    if (0x1a <= (unsigned int)(a0 - 0x41U))
      return 0;
    return 1;
  }
  if (a0 <= 0x2f)
    return 0;
  return 1;
}

// Function: sub_191b0 @ 0x191b0
unsigned long sub_191b0(int a0) // return-dupe
{
  if (0x5a < a0) {
    if (0x1a <= (unsigned int)(a0 - 0x61U))
      return 0;
    return 1;
  }
  if (a0 <= 0x40)
    return 0;
  return 1;
}

// Function: sub_191e2 @ 0x191e2
bool sub_191e2(unsigned int a0)
{
  return a0 < 0x80;
}

// Function: sub_19203 @ 0x19203
unsigned int sub_19203(int a0) // return-dupe
{
  if ((a0 != 0x20) && (a0 != 9))
    return 0;
  return 1;
}

// Function: sub_1922b @ 0x1922b
unsigned long sub_1922b(int a0) // return-dupe
{
  if (0x1f < a0) {
    if (a0 != 0x7f)
      return 0;
    return 1;
  }
  if (a0 <= -1)
    return 0;
  return 1;
}

// Function: sub_19258 @ 0x19258
bool sub_19258(int a0)
{
  return (unsigned int)(a0 - 0x30U) < 10;
}

// Function: sub_1927c @ 0x1927c
bool sub_1927c(int a0)
{
  return (unsigned int)(a0 - 0x21U) < 0x5e;
}

// Function: sub_192a0 @ 0x192a0
bool sub_192a0(int a0)
{
  return (unsigned int)(a0 - 0x61U) < 0x1a;
}

// Function: sub_192c4 @ 0x192c4
bool sub_192c4(int a0)
{
  return (unsigned int)(a0 - 0x20U) < 0x5f;
}

// Function: sub_192e8 @ 0x192e8
unsigned long sub_192e8(int a0) // return-dupe, return-dupe x3
{
  if (0x7f <= a0)
    return 0;
  if (0x7b <= a0)
    return 1;
  if (0x61 <= a0)
    return 0;
  if (a0 <= 0x5a) {
    if (0x2f < a0) {
      if (7 <= (unsigned int)(a0 - 0x3aU))
        return 0;
      return 1;
    }
    if (a0 <= 0x20)
      return 0;
    return 1;
  }
  return 1;
}

// Function: sub_19332 @ 0x19332
unsigned long sub_19332(int a0) // return-dupe
{
  if (0xd < a0) {
    if (a0 != 0x20)
      return 0;
    return 1;
  }
  if (a0 <= 8)
    return 0;
  return 1;
}

// Function: sub_1935f @ 0x1935f
bool sub_1935f(int a0)
{
  return (unsigned int)(a0 - 0x41U) < 0x1a;
}

// Function: sub_19383 @ 0x19383
unsigned long sub_19383(int a0) // return-dupe
{
  if (((unsigned int)(a0 - 0x30U) <= 0x36) && (0x7e0000007e03ffU >> ((unsigned char)(a0 - 0x30U) & 0x3f) & 1))
    return 1;
  return 0;
}

// Function: sub_193cb @ 0x193cb
int sub_193cb(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_193ee @ 0x193ee
int sub_193ee(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_19411 @ 0x19411
int sub_19411(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned char v2; // al
  
  if (a0 == a1)
    return 0;
  do {
    v1 = sub_193cb(*a0);
    v2 = sub_193cb(*a1);
    if (!v1) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while (v1 == v2);
  return (unsigned int)v1 - (unsigned int)v2;
}

// Function: sub_1948b @ 0x1948b
unsigned long sub_1948b(void *a0)
{
  int v1; // eax
  unsigned long v2; // rax
  bool v3; // zf
  
  v2 = __fpending(a0);
  v1 = ferror_unlocked(a0);
  v3 = sub_ff17(a0) == 0;
  if ((!v1) && ((v3 || ((!v2 && (*__errno_location() == 9))))))
    return 0;
  if (v3)
    *__errno_location() = 0;
  return 0xffffffff;
}

// Function: sub_1951a @ 0x1951a
void sub_1951a(double a0) // return-dupe x2
{
  double v1; // xmm0_qa
  long v2; // stack - 0x20
  long v3; // stack - 0x18
  
  if (a0 <= dat_1db10) {
    sub_1a7ea(0x8000000000000000,0);
    return;
  }
  if (dat_1db18 <= a0) {
    sub_1a7ea(0x7fffffffffffffff,999999999);
    return;
  }
  v1 = dat_1db20 * (a0 - (double)(long)a0);
  v3 = (long)v1;
  v3 += (unsigned long)((double)v3 < v1);
  v2 = (long)a0 + v3 / 1000000000;
  v3 %= 1000000000;
  if (v3 <= -1) {
    v2 -= 1;
    v3 += 1000000000;
  }
  sub_1a7ea(v2,v3);
}

// Function: sub_1967a @ 0x1967a
int sub_1967a(int a0,int a1)
{
  unsigned long v1;
  unsigned long v2; // stack - 0xa8
  
  if (!a1)
    return sub_199ec(a0,(unsigned int)v2);
  if (a1 == 0x406)
    return sub_19a1d(a0,(unsigned int)v2);
  if (0xc <= a1) { // branch-flip
    if ((0x40b <= a1) || (a1 <= 0x3ff)) {
      return fcntl(a0,a1,v2); // return-dupe
    }
    v1 = 1L << ((unsigned char)a1 & 0x3f);
    if (v1 & 0x2c5) {
      return fcntl(a0,a1,v2 & 0xffffffff); // return-dupe
    }
    v1 &= 0x502;
  }
  else {
    if (a1 <= -1) {
      return fcntl(a0,a1,v2);
    }
    v1 = 1L << ((unsigned char)a1 & 0x3f);
    if (v1 & 0x515)
      return fcntl(a0,a1,v2 & 0xffffffff);
    v1 &= 0xa0a;
  }
  if (v1)
    return fcntl(a0,a1);
  return fcntl(a0,a1,v2);
}

// Function: sub_199ec @ 0x199ec
int sub_199ec(int a0,unsigned int a1)
{
  return fcntl(a0,0,(unsigned long)a1);
}

// Function: sub_19a1d @ 0x19a1d
int sub_19a1d(int a0,unsigned int a1)
{
  int v1;
  unsigned int v2; // eax
  int v3; // stack - 0x14
  
  if (0 <= dat_239c0) { // branch-flip
    v3 = fcntl(a0,0x406,(unsigned long)a1);
    if ((0 <= v3) || (*__errno_location() != 0x16))
      dat_239c0 = 1;
    else {
      v3 = sub_199ec(a0,a1);
      if (0 <= v3)
        dat_239c0 = -1;
    }
  }
  else {
    v3 = sub_199ec(a0,a1);
  }
  if ((0 <= v3) && (dat_239c0 == -1)) {
    v2 = fcntl(v3,1);
    if ((0 <= (int)v2) && (fcntl(v3,2,(unsigned long)(v2 | 1)) != -1))
      return v3;
    v1 = *__errno_location();
    close(v3);
    *__errno_location() = v1;
    v3 = -1;
  }
  return v3;
}

// Function: sub_19b20 @ 0x19b20
int sub_19b20(int a0,unsigned int a1)
{
  int v1;
  int v2; // stack - 0x1c
  
  v2 = a0;
  if ((0 <= a0) && (a0 <= 2)) {
    v2 = sub_19b79(a0,a1);
    v1 = *__errno_location();
    close(a0);
    *__errno_location() = v1;
  }
  return v2;
}

// Function: sub_19b79 @ 0x19b79
void sub_19b79(unsigned int a0,unsigned int a1) // ternary
{
  unsigned long v1; // rcx
  
  v1 = (a1 & 0x80000) ? 0x406 : 0; // branch-flip
  sub_1967a(a0,v1,3);
}

// Function: sub_19bbb @ 0x19bbb
void * sub_19bbb(char *a0,char *a1)
{
  int v1;
  int v2;
  void *v3; // rax
  void *v4; // stack - 0x10
  
  v4 = fopen(a0,a1);
  if (((!v4) || (v2 = fileno(v4), v2 <= -1)) || (3 <= v2))
    return v4;
  v2 = sub_1a92b(v2);
  if (v2 <= -1) {
    v2 = *__errno_location();
    sub_ff17(v4);
    *__errno_location() = v2;
    return NULL;
  }
  if ((!sub_ff17(v4)) && (v3 = fdopen(v2,a1), v3))
    return v3;
  v1 = *__errno_location();
  close(v2);
  *__errno_location() = v1;
  return NULL;
}

// Function: sub_19ca5 @ 0x19ca5
unsigned long sub_19ca5(void)
{
  *__errno_location() = 0xc;
  return 0;
}

// Function: sub_19cbf @ 0x19cbf
void sub_19cbf(unsigned long a0)
{
  malloc(a0);
}

// Function: sub_19cdd @ 0x19cdd
void sub_19cdd(void *a0,unsigned long a1)
{
  realloc(a0,a1 == 0 | a1);
}

// Function: sub_19d14 @ 0x19d14
void sub_19d14(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1);
}

// Function: sub_19d3d @ 0x19d3d
void sub_19d3d(void *a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x20
  unsigned long v2; // stack - 0x18
  
  if ((!a1) || (v1 = a2, v2 = a1, !a2)) {
    v1 = 1;
    v2 = 1;
  }
  reallocarray(a0,v2,v1);
}

// Function: sub_19d8c @ 0x19d8c
char * sub_19d8c(void)
{
  char *v1; // stack - 0x10
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    v1 = "";
  if (!*v1)
    v1 = "ASCII";
  return v1;
}

// Function: sub_19dd4 @ 0x19dd4
unsigned long sub_19dd4(unsigned int *a0,unsigned char *a1,unsigned long a2,void *a3) // early-return
{
  long v1;
  unsigned int *v2; // stack - 0x30
  unsigned long v3; // stack - 0x18
  
  v2 = a0;
  if (!a0)
    v2 = &v1;
  v3 = mbrtowc(v2,(char *)a1,a2,a3);
  if (((0xfffffffffffffffe <= v3) && (a2)) && (sub_107aa(0) != '\x01')) {
    *v2 = (unsigned int)*a1;
    return 1;
  }
  return v3;
}

// Function: sub_19e7e @ 0x19e7e
int sub_19e7e(char *a0,long a1,char *a2,long a3)
{
  bool v1;
  int v2; // eax
  long v3; // rax
  long v4; // rax
  long v5; // stack - 0x48
  char *v6; // stack - 0x40
  long v7; // stack - 0x38
  char *v8; // stack - 0x30
  
  v5 = a3;
  v6 = a2;
  v7 = a1;
  v8 = a0;
  do {
    *__errno_location() = 0;
    v2 = strcoll(v8,v6);
    if ((v2) || (*__errno_location()))
      v1 = 1;
    else {
      v1 = 0;
    }
    if (v1)
      return v2;
    v3 = strlen(v8) + 1;
    v4 = strlen(v6) + 1;
    v8 = &v8[v3];
    v6 = &v6[v4];
    v7 -= v3;
    v5 -= v4;
    if (!v7)
      return -(unsigned int)(v5 != 0);
  } while (v5);
  return 1;
}

// Function: sub_19f53 @ 0x19f53
unsigned int sub_19f53(void *a0,unsigned long a1,void *a2,unsigned long a3)
{
  char v1;
  char v2;
  unsigned int v3; // stack - 0xc
  
  if (a1 != a3) { // branch-flip
label_19fab:
    v1 = *(char *)(a1 + (long)a0);
    v2 = *(char *)(a3 + (long)a2);
    *(char *)(a1 + (long)a0) = 0;
    *(char *)(a3 + (long)a2) = 0;
    v3 = sub_19e7e(a0,a1 + 1,a2,a3 + 1);
    *(char *)((long)a0 + a1) = v1;
    *(char *)((long)a2 + a3) = v2;
  }
  else {
    if (memcmp(a0,a2,a1)) goto label_19fab;
    *__errno_location() = 0;
    v3 = 0;
  }
  return v3;
}

// Function: sub_1a033 @ 0x1a033
unsigned long sub_1a033(void *a0,unsigned long a1,void *a2,unsigned long a3) // early-return
{
  if ((a1 == a3) && (!memcmp(a0,a2,a1))) {
    *__errno_location() = 0;
    return 0;
  }
  return sub_19e7e(a0,a1,a2,a3);
}

// Function: sub_1a0a0 @ 0x1a0a0
unsigned long sub_1a0a0(long *a0,long *a1) // early-return
{
  unsigned int v1; // eax
  long v2; // stack - 0x28
  long v3; // stack - 0x38
  long v4; // stack - 0x20
  
  if ((0 <= a0[1]) && (a0[1] <= 999999999)) {
    v3 = *a0;
    v4 = a0[1];
    while( true ) {
      if (v3 <= 0x1fa400) {
        v2 = v3;
        return nanosleep(&v2,a1);
      }
      v2 = 0x1fa400;
      v1 = nanosleep(&v2,a1);
      v3 -= 0x1fa400;
      if (v1) break;
      v4 = 0;
    }
    if (a1)
      *a1 = *a1 + v3;
    return (unsigned long)v1;
  }
  *__errno_location() = 0x16;
  return 0xffffffff;
}

// Function: sub_1a1a6 @ 0x1a1a6
void sub_1a1a6(long a0,unsigned long a1) // return-dupe
{
  if (*(unsigned char *)(a0 + 0x50) & 1) {
    (**(void **)(a0 + 0x38))(*(unsigned long *)(a0 + 0x48),a1);
    return;
  }
  (**(void **)(a0 + 0x38))(a1);
}

// Function: sub_1a1fa @ 0x1a1fa
void sub_1a1fa(long a0,unsigned long a1) // return-dupe
{
  if (*(unsigned char *)(a0 + 0x50) & 1) {
    (**(void **)(a0 + 0x40))(*(unsigned long *)(a0 + 0x48),a1);
    return;
  }
  (**(void **)(a0 + 0x40))(a1);
}

// Function: sub_1a24f @ 0x1a24f
unsigned long sub_1a24f(long *a0,long a1,long a2)
{
  long v1;
  long *v2;
  long v3; // stack - 0x30
  long v4; // stack - 0x28
  
  v3 = a2;
  if (!a2)
    v3 = 0x10;
  v4 = a1;
  if (!a1)
    v4 = 0xfe0;
  *a0 = v4;
  a0[6] = v3 + -1;
  v1 = *a0;
  a0[1] = sub_1a1a6(a0,v1);
  v2 = (long *)a0[1];
  if (!v2)
    (*obstack_alloc_failed_handler)();
  a0[2] = -v3 & (long)v2 + v3 + 0xf;
  a0[3] = a0[2];
  *v2 = *a0 + (long)v2;
  a0[4] = *v2;
  v2[1] = 0;
  *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] & 0xfd;
  *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] & 0xfb;
  return 1;
}

// Function: _obstack_begin @ 0x1a375
void _obstack_begin(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  *(unsigned long *)(a0 + 0x38) = a3;
  *(unsigned long *)(a0 + 0x40) = a4;
  *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) & 0xfe;
  sub_1a24f(a0,a1,a2);
}

// Function: _obstack_begin_1 @ 0x1a3d4
void _obstack_begin_1(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5)
{
  *(unsigned long *)(a0 + 0x38) = a3;
  *(unsigned long *)(a0 + 0x40) = a4;
  *(unsigned long *)(a0 + 0x48) = a5;
  *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 1;
  sub_1a24f(a0,a1,a2);
}

// Function: _obstack_newchunk @ 0x1a443
void _obstack_newchunk(unsigned long *a0,long a1)
{
  unsigned long v1;
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  void *v5; // rax
  unsigned long *v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  
  v1 = a0[1];
  v6 = NULL;
  v2 = a0[3] - a0[2];
  v3 = a1 + v2;
  v4 = v3 + a0[6];
  v7 = v4 + (v2 >> 3) + 100;
  if (v7 < v4)
    v7 = v4;
  if (v7 < *a0)
    v7 = *a0;
  if ((v2 <= v3) && (v3 <= v4))
    v6 = (unsigned long *)sub_1a1a6(a0,v7);
  if (!v6)
    (*obstack_alloc_failed_handler)();
  a0[1] = (unsigned long)v6;
  v6[1] = v1;
  a0[4] = (long)v6 + v7;
  *v6 = a0[4];
  v5 = (void *)(~a0[6] & (long)v6 + a0[6] + 0x10);
  memcpy(v5,(void *)a0[2],v2);
  if ((!(a0[10] & 2)) && (a0[2] == (~a0[6] & v1 + 0x10 + a0[6]))) {
    v6[1] = *(unsigned long *)(v1 + 8);
    sub_1a1fa(a0,v1);
  }
  a0[2] = (unsigned long)v5;
  a0[3] = a0[2] + v2;
  *(unsigned char *)&a0[10] = (unsigned char)a0[10] & 0xfd;
}

// Function: _obstack_allocated_p @ 0x1a63a
bool _obstack_allocated_p(long a0,unsigned long *a1)
{
  unsigned long *v1; // stack - 0x18
  
  for (v1 = *(unsigned long **)(a0 + 8); (v1 && ((a1 <= v1 || ((unsigned long *)*v1 < a1)))); v1 = (unsigned long *)v1[1]) {
  }
  return v1 != NULL;
}

// Function: _obstack_free @ 0x1a697
void _obstack_free(long a0,unsigned long *a1) // return-dupe
{
  unsigned long *v1;
  unsigned long *v2; // stack - 0x18
  
  v2 = *(unsigned long **)(a0 + 8);
  while ((v2 && ((a1 <= v2 || ((unsigned long *)*v2 < a1))))) {
    v1 = (unsigned long *)v2[1];
    sub_1a1fa(a0,v2);
    *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 2;
    v2 = v1;
  }
  if (v2) {
    *(unsigned long **)(a0 + 0x18) = a1;
    *(unsigned long *)(a0 + 0x10) = *(unsigned long *)(a0 + 0x18);
    *(unsigned long *)(a0 + 0x20) = *v2;
    *(unsigned long **)(a0 + 8) = v2;
    return;
  }
  if (!a1)
    return;
  abort(); // no-return
}

// Function: _obstack_memory_used @ 0x1a75b
long _obstack_memory_used(long a0)
{
  long *v1; // stack - 0x18
  long v2; // stack - 0x10
  
  v2 = 0;
  for (v1 = *(long **)(a0 + 8); v1; v1 = (long *)v1[1]) {
    v2 += *v1 - (long)v1;
  }
  return v2;
}

// Function: sub_1a7a5 @ 0x1a7a5
void sub_1a7a5(void)
{
  fprintf(stderr,"%s\n",gettext("memory exhausted"));
  exit(dat_23118); // no-return
}

// Function: sub_1a7ea @ 0x1a7ea
undefined16 sub_1a7ea(unsigned long a0,unsigned long a1)
{
  char v1 [16];
  
  v1._8_8_ = a1;
  v1._0_8_ = a0;
  return v1._0_16_;
}

// Function: sub_1a814 @ 0x1a814
int sub_1a814(long a0,long a1,long a2,long a3)
{
  return ((unsigned int)(a3 < a1) - (unsigned int)(a1 < a3)) + ((unsigned int)(a2 < a0) - (unsigned int)(a0 < a2)) * 2;
}

// Function: sub_1a88a @ 0x1a88a
int sub_1a88a(long a0,long a1)
{
  return (unsigned int)(a1 && !a0) + (unsigned int)(0 < a0) + (int)(a0 >> 0x3f);
}

// Function: sub_1a8dc @ 0x1a8dc
double sub_1a8dc(long a0,long a1)
{
  return (double)a1 / dat_1db48 + (double)a0;
}

// Function: sub_1a92b @ 0x1a92b
void sub_1a92b(unsigned int a0)
{
  sub_1967a(a0,0,3);
}

// Function: sub_1a960 @ 0x1a960
void sub_1a960(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_23008); // tail-call
}

// Function: _DT_FINI @ 0x1a974
void _DT_FINI(void)
{
  return;
}
