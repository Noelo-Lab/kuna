// Function: _DT_INIT @ 0x4000
void _DT_INIT(void) // return-dupe
{
  if (!dat_25fc8)
    return;
  (*dat_25fc8)();
}

// Function: sub_4020 @ 0x4020
void sub_4020(void)
{
  (*dat_25c48)(); // jump-as-call
}

// Function: free @ 0x46c0
void free(void *a0)
{
  (*dat_25f98)(); // jump-as-call
}

// Function: localtime_r @ 0x46d0
void * localtime_r(void *a0,void *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25fb0)(); // jump-as-call
  return v1;
}

// Function: gmtime_r @ 0x46e0
void gmtime_r(void)
{
  (*dat_25fb8)(); // jump-as-call
}

// Function: strcmp @ 0x46f0
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_25fc0)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x4700
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25fd0)(); // jump-as-call
  return v1;
}

// Function: __cxa_finalize @ 0x4710
void __cxa_finalize(void)
{
  (*dat_25fe0)(); // jump-as-call
}

// Function: __ctype_toupper_loc @ 0x4720
void * __ctype_toupper_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25c50)(); // jump-as-call
  return v1;
}

// Function: getenv @ 0x4730
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25c58)(); // jump-as-call
  return v1;
}

// Function: fgetfilecon @ 0x4740
void fgetfilecon(void)
{
  (*dat_25c60)(); // jump-as-call
}

// Function: sigprocmask @ 0x4750
int sigprocmask(int a0,void *a1,void *a2)
{
  int v1; // eax
  
  v1 = (*dat_25c68)(); // jump-as-call
  return v1;
}

// Function: __snprintf_chk @ 0x4760
int __snprintf_chk(char *a0,unsigned long a1,int a2,unsigned long a3,char *a4,...)
{
  int v1; // eax
  
  v1 = (*dat_25c70)(); // jump-as-call
  return v1;
}

// Function: raise @ 0x4770
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_25c78)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x4780
void abort(void)
{
  (*dat_25c80)(); // jump-as-call
}

// Function: __errno_location @ 0x4790
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_25c88)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x47a0
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_25c90)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x47b0
void _exit(int a0)
{
  (*dat_25c98)(); // jump-as-call
}

// Function: strcpy @ 0x47c0
char * strcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25ca0)(); // jump-as-call
  return v1;
}

// Function: __mbstowcs_chk @ 0x47d0
void __mbstowcs_chk(void)
{
  (*dat_25ca8)(); // jump-as-call
}

// Function: __fpending @ 0x47e0
unsigned long __fpending(void *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_25cb0)(); // jump-as-call
  return v1;
}

// Function: isatty @ 0x47f0
int isatty(int a0)
{
  int v1; // eax
  
  v1 = (*dat_25cb8)(); // jump-as-call
  return v1;
}

// Function: sigaction @ 0x4800
int sigaction(int a0,void *a1,void *a2)
{
  int v1; // eax
  
  v1 = (*dat_25cc0)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x4810
void iswcntrl(void)
{
  (*dat_25cc8)(); // jump-as-call
}

// Function: reallocarray @ 0x4820
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25cd0)(); // jump-as-call
  return v1;
}

// Function: wcswidth @ 0x4830
void wcswidth(void)
{
  (*dat_25cd8)(); // jump-as-call
}

// Function: localeconv @ 0x4840
void localeconv(void)
{
  (*dat_25ce0)(); // jump-as-call
}

// Function: faccessat @ 0x4850
void faccessat(void)
{
  (*dat_25ce8)(); // jump-as-call
}

// Function: mbstowcs @ 0x4860
void mbstowcs(void)
{
  (*dat_25cf0)(); // jump-as-call
}

// Function: readlink @ 0x4870
long readlink(char *a0,char *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_25cf8)(); // jump-as-call
  return v1;
}

// Function: clock_gettime @ 0x4880
int clock_gettime(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_25d00)(); // jump-as-call
  return v1;
}

// Function: setenv @ 0x4890
int setenv(char *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_25d08)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x48a0
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25d10)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x48b0
int fclose(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_25d18)(); // jump-as-call
  return v1;
}

// Function: opendir @ 0x48c0
void * opendir(char *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25d20)(); // jump-as-call
  return v1;
}

// Function: getpwuid @ 0x48d0
void * getpwuid(unsigned int a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25d28)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x48e0
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25d30)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x48f0
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25d38)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x4900
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_25d40)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x4910
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_25d48)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x4920
void __stack_chk_fail(void)
{
  (*dat_25d50)(); // jump-as-call
}

// Function: getopt_long @ 0x4930
int getopt_long(int a0,char **a1,char *a2,void *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_25d58)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x4940
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_25d60)(); // jump-as-call
  return v1;
}

// Function: freecon @ 0x4950
void freecon(void)
{
  (*dat_25d68)(); // jump-as-call
}

// Function: strchr @ 0x4960
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25d70)(); // jump-as-call
  return v1;
}

// Function: getgrgid @ 0x4970
void * getgrgid(unsigned int a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25d78)(); // jump-as-call
  return v1;
}

// Function: snprintf @ 0x4980
int snprintf(char *a0,unsigned long a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_25d80)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x4990
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25d88)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x49a0
void lseek(void)
{
  (*dat_25d90)(); // jump-as-call
}

// Function: __assert_fail @ 0x49b0
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_25d98)(); // jump-as-call
}

// Function: fputs @ 0x49c0
int fputs(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_25da0)(); // jump-as-call
  return v1;
}

// Function: fnmatch @ 0x49d0
int fnmatch(char *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_25da8)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x49e0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25db0)(); // jump-as-call
  return v1;
}

// Function: ioctl @ 0x49f0
int ioctl(int a0,unsigned long a1,...)
{
  int v1; // eax
  
  v1 = (*dat_25db8)(); // jump-as-call
  return v1;
}

// Function: getcwd @ 0x4a00
char * getcwd(char *a0,unsigned long a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25dc0)(); // jump-as-call
  return v1;
}

// Function: strspn @ 0x4a10
unsigned long strspn(char *a0,char *a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_25dc8)(); // jump-as-call
  return v1;
}

// Function: closedir @ 0x4a20
int closedir(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_25dd0)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x4a30
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_25dd8)(); // jump-as-call
  return v1;
}

// Function: _setjmp @ 0x4a40
void _setjmp(void)
{
  (*dat_25de0)(); // jump-as-call
}

// Function: fputs_unlocked @ 0x4a50
int fputs_unlocked(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_25de8)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x4a60
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25df0)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x4a70
int ferror_unlocked(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_25df8)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x4a80
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25e00)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x4a90
void signal(void)
{
  (*dat_25e08)(); // jump-as-call
}

// Function: dirfd @ 0x4aa0
int dirfd(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_25e10)(); // jump-as-call
  return v1;
}

// Function: getpwnam @ 0x4ab0
void * getpwnam(char *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25e18)(); // jump-as-call
  return v1;
}

// Function: __memcpy_chk @ 0x4ac0
void * __memcpy_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25e20)(); // jump-as-call
  return v1;
}

// Function: sigemptyset @ 0x4ad0
int sigemptyset(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_25e28)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x4ae0
int stat(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_25e30)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x4af0
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25e38)(); // jump-as-call
  return v1;
}

// Function: getgrnam @ 0x4b00
void * getgrnam(char *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25e40)(); // jump-as-call
  return v1;
}

// Function: putchar_unlocked @ 0x4b10
int putchar_unlocked(int a0)
{
  int v1; // eax
  
  v1 = (*dat_25e48)(); // jump-as-call
  return v1;
}

// Function: tzset @ 0x4b20
void tzset(void)
{
  (*dat_25e50)(); // jump-as-call
}

// Function: fileno @ 0x4b30
int fileno(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_25e58)(); // jump-as-call
  return v1;
}

// Function: tcgetpgrp @ 0x4b40
void tcgetpgrp(void)
{
  (*dat_25e60)(); // jump-as-call
}

// Function: readdir @ 0x4b50
void * readdir(void *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25e68)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x4b60
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_25e70)(); // jump-as-call
  return v1;
}

// Function: putc_unlocked @ 0x4b70
int putc_unlocked(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_25e78)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x4b80
int fflush(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_25e80)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x4b90
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25e88)(); // jump-as-call
  return v1;
}

// Function: strcoll @ 0x4ba0
int strcoll(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_25e90)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x4bb0
int __freading(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_25e98)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x4bc0
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_25ea0)(); // jump-as-call
  return v1;
}

// Function: gnu_dev_major @ 0x4bd0
void gnu_dev_major(void)
{
  (*dat_25ea8)(); // jump-as-call
}

// Function: realloc @ 0x4be0
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25eb0)(); // jump-as-call
  return v1;
}

// Function: stpncpy @ 0x4bf0
char * stpncpy(char *a0,char *a1,unsigned long a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25eb8)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x4c00
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25ec0)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x4c10
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_25ec8)(); // jump-as-call
  return v1;
}

// Function: statx @ 0x4c20
void statx(void)
{
  (*dat_25ed0)(); // jump-as-call
}

// Function: strftime @ 0x4c30
unsigned long strftime(char *a0,unsigned long a1,char *a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_25ed8)(); // jump-as-call
  return v1;
}

// Function: mempcpy @ 0x4c40
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25ee0)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x4c50
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25ee8)(); // jump-as-call
  return v1;
}

// Function: error @ 0x4c60
void error(int a0,int a1,char *a2,...)
{
  (*dat_25ef0)(); // jump-as-call
}

// Function: fseeko @ 0x4c70
void fseeko(void)
{
  (*dat_25ef8)(); // jump-as-call
}

// Function: strtoumax @ 0x4c80
void strtoumax(void)
{
  (*dat_25f00)(); // jump-as-call
}

// Function: unsetenv @ 0x4c90
int unsetenv(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_25f08)(); // jump-as-call
  return v1;
}

// Function: gnu_dev_minor @ 0x4ca0
void gnu_dev_minor(void)
{
  (*dat_25f10)(); // jump-as-call
}

// Function: __cxa_atexit @ 0x4cb0
void __cxa_atexit(void)
{
  (*dat_25f18)(); // jump-as-call
}

// Function: wcstombs @ 0x4cc0
void wcstombs(void)
{
  (*dat_25f20)(); // jump-as-call
}

// Function: gethostname @ 0x4cd0
void gethostname(void)
{
  (*dat_25f28)(); // jump-as-call
}

// Function: sigismember @ 0x4ce0
void sigismember(void)
{
  (*dat_25f30)(); // jump-as-call
}

// Function: exit @ 0x4cf0
void exit(int a0)
{
  (*dat_25f38)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x4d00
int __fprintf_chk(void *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_25f40)(); // jump-as-call
  return v1;
}

// Function: getfilecon @ 0x4d10
void getfilecon(void)
{
  (*dat_25f48)(); // jump-as-call
}

// Function: fflush_unlocked @ 0x4d20
int fflush_unlocked(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_25f50)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x4d30
int mbsinit(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_25f58)(); // jump-as-call
  return v1;
}

// Function: lgetfilecon @ 0x4d40
void lgetfilecon(void)
{
  (*dat_25f60)(); // jump-as-call
}

// Function: iswprint @ 0x4d50
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_25f68)(); // jump-as-call
  return v1;
}

// Function: gnu_dev_makedev @ 0x4d60
void gnu_dev_makedev(void)
{
  (*dat_25f70)(); // jump-as-call
}

// Function: sigaddset @ 0x4d70
int sigaddset(void *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_25f78)(); // jump-as-call
  return v1;
}

// Function: __ctype_tolower_loc @ 0x4d80
void * __ctype_tolower_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25f80)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x4d90
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25f88)(); // jump-as-call
  return v1;
}

// Function: __sprintf_chk @ 0x4da0
int __sprintf_chk(char *a0,int a1,unsigned long a2,char *a3,...)
{
  int v1; // eax
  
  v1 = (*dat_25f90)(); // jump-as-call
  return v1;
}

// Function: sub_4db0 @ 0x4db0
undefined16 sub_4db0(void)
{
  char v1 [16];
  unsigned long v2; // rax
  
  *__errno_location() = 0xc;
  v1._8_8_ = 0;
  v1._0_8_ = v2;
  return v1._0_16_ << 0x40;
}

// Function: sub_4dd0 @ 0x4dd0
unsigned int sub_4dd0(int a0,unsigned long *a1)
{
  unsigned long *v1;
  char v10 [16];
  unsigned long v11; // stack - 0x50
  unsigned long v2;
  unsigned long v3;
  int v4;
  unsigned int v5; // eax
  long v6; // rax
  long v7;
  unsigned long v8; // stack - 0x58
  long *v9;
  
  sub_13390(*a1);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_8dd0();
  sub_18230(sub_df00);
  dat_27210 = 0;
  dat_272b8 = 1;
  dat_27380 = NULL;
  dat_27370 = 0x8000000000000000;
  dat_27378 = 0xffffffffffffffff;
  v4 = sub_bda0(a0,a1);
  v7 = (long)v4;
  if ((dat_27312) && (sub_6af0(), dat_27312)) {
    dat_272c0 = 0;
    if ((((dat_272f4) || (sub_6420(0xd))) || ((sub_6420(0xe) && (dat_27390)))) || ((sub_6420(0xc) && (!dat_27338)))) {
label_51c8:
      dat_272fd = 1;
    }
  }
  else if (dat_272f4) goto label_51c8;
  if (((!dat_272f8) && (dat_272f8 = 1, !dat_272f5)) && (dat_27314 != 3))
    dat_272f8 = (-(unsigned int)(dat_27338 == 0) & 0xfffffffe) + 3;
  if (dat_272f6) {
    dat_273c8 = sub_f380(0x1e,0,sub_56d0,sub_56e0,sub_61a0);
    if (!dat_273c8)
      sub_16da0(); // no-return
    _obstack_begin(0x270e0,0,0,dat_25fd0,dat_25f98);
  }
  dat_272a8 = sub_15d80(getenv("TZ"));
  v3 = dat_25fd0;
  v2 = dat_25f98;
  if (((dat_27330 - 3U & 0xfffffffdU) && (dat_27338)) && ((!dat_27365 && (!dat_2732c)))) {
    dat_272a1 = 0;
    dat_272a0 = 1;
    if (((!dat_272f6) && (!dat_27312)) && (!dat_27314))
      dat_272a0 = dat_272f4;
  }
  else {
    dat_272a1 = 1;
    dat_272a0 = 0;
  }
  dat_272a0 &= 1;
  if (dat_27318) {
    _obstack_begin(0x271a0,0,0,dat_25fd0,dat_25f98);
    _obstack_begin(0x27140,0,0,v3,v2);
  }
  if (dat_27311) {
    sub_5670();
    dat_27388 = sub_16f20();
    if (!dat_27388)
      dat_27388 = 0x1abb9;
  }
  dat_273b8 = 100;
  dat_273c0 = sub_16b00(100,0xd0);
  dat_273b0 = 0;
  sub_6e20();
  if (1 <= a0 - v4) { // branch-flip
    do {
      v1 = &a1[v7];
      v7 += 1;
      sub_a550(*v1,0,1,0x1abb9);
    } while ((int)v7 < a0);
    if (dat_273b0) { // branch-flip
label_52b2:
      sub_89f0();
      if (!dat_272f5)
        sub_a1e0(0,1);
      if (!dat_273b0) goto label_4fe9;
      sub_9f20();
      if (!dat_27380) goto label_50a0;
      sub_7460(10);
    }
    else {
label_4fe9:
      if (a0 - v4 <= 1) goto label_5292;
    }
    while (v9 = dat_27380, dat_27380) {
label_5038:
      while( true ) {
        v7 = dat_273c8;
        dat_27380 = (long *)v9[3];
        if ((dat_273c8) && (!*v9)) break;
        sub_b180(*v9,v9[1],(char)v9[2]);
        sub_61b0(v9);
        dat_272b8 = 1;
        v9 = dat_27380;
        if (!dat_27380) goto label_50a0;
      }
      v10 = sub_6330();
      v11 = SUB168(v10,8);
      v8 = SUB168(v10,0);
      v7 = sub_fa00(v7,&v8);
      if (!v7)
        __assert_fail("found","src/ls.c",0x70d,"main"); // no-return
      sub_61a0(v7);
      sub_61b0(v9);
    }
  }
  else {
    if (dat_272f5) // branch-flip
      sub_a550(".",3,1,0x1abb9);
    else {
      sub_6840(".",0,1);
    }
    if (dat_273b0) goto label_52b2;
label_5292:
    if (dat_27380) {
      v9 = dat_27380;
      if (dat_27380[3]) goto label_5038;
      dat_272b8 = 0;
      goto label_5038;
    }
  }
label_50a0:
  if ((dat_27312) && (dat_27310)) {
    if ((dat_26060 != 2) || (((v4 = memcmp(dat_26068,(void *)0x1add7,2), v4 || (dat_26070 != 1)) || (*dat_26078 != 'm'))))
      sub_7cc0();
    fflush_unlocked(stdout);
    sub_7dd0();
    for (v4 = dat_27214; v4; v4 = v4 + -1) {
      raise(0x13);
    }
    if (dat_27218)
      raise(dat_27218);
  }
  if (dat_27318) {
    sub_8060("//DIRED//",0x271a0);
    sub_8060("//SUBDIRED//",0x27140);
    v5 = sub_151e0(dat_272d0);
    __printf_chk(1,"//DIRED-OPTIONS// --quoting-style=%s\n",*(unsigned long *)((unsigned long)v5 * 8 + 0x259e0));
  }
  v7 = dat_273c8;
  if (dat_273c8) {
    v6 = sub_ef20(dat_273c8);
    if (v6)
      __assert_fail("hash_get_n_entries (active_dir_set) == 0","src/ls.c",0x741,"main"); // no-return
    sub_f540(v7);
  }
  return dat_27210;
}

// Function: sub_5480 @ 0x5480
void sub_5480(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_25fa0)(sub_4dd0,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_54b0 @ 0x54b0
void sub_54b0(void)
{
  return;
}

// Function: _FINI_0 @ 0x5520
void _FINI_0(void)
{
  if (!dat_262c8) {
    if (dat_25fe0)
      __cxa_finalize(dat_26008);
    sub_54b0();
    dat_262c8 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x5560
void _INIT_0(void)
{
  return;
}

// Function: sub_5570 @ 0x5570
unsigned int sub_5570(unsigned int a0)
{
  return a0;
}

// Function: sub_5580 @ 0x5580
bool sub_5580(char *a0)
{
  if (*a0 != '.')
    return 0;
  return !a0[(unsigned long)(a0[1] == '.') + 1] || a0[(unsigned long)(a0[1] == '.') + 1] == '/';
}

// Function: sub_55b0 @ 0x55b0
unsigned long sub_55b0(unsigned long a0)
{
  return a0;
}

// Function: sub_55c0 @ 0x55c0
bool sub_55c0(int a0)
{
  return a0 == 0x5f;
}

// Function: sub_55d0 @ 0x55d0
unsigned long sub_55d0(void)
{
  return 0;
}

// Function: sub_55e0 @ 0x55e0
unsigned int sub_55e0(long a0)
{
  if ((dat_27390) && (*(char *)(a0 + 0xb9)))
    return *(unsigned int *)(a0 + 0xac);
  return *(unsigned int *)(a0 + 0x30);
}

// Function: sub_5610 @ 0x5610
void sub_5610(void)
{
  sub_15cb0(); // tail-call
}

// Function: sub_5620 @ 0x5620
char * sub_5620(char *a0) // return-dupe
{
  char v1;
  char v2;
  
  v2 = *a0;
  if (!v2)
    return NULL;
  do {
    v1 = a0[1];
    if (v2 != '%') { // branch-flip
label_5639:
      v2 = v1;
    }
    else {
      if (v1 != '%') {
        if (v1 == 'b')
          return a0;
        goto label_5639;
      }
      v2 = a0[2];
      a0 = &a0[1];
    }
    a0 = &a0[1];
    if (!v2)
      return NULL;
  } while( true );
}

// Function: sub_5670 @ 0x5670
void sub_5670(void)
{
  int v1; // ebx
  unsigned long v2; // rbx
  bool v3; // al
  
  v2 = 0;
  do {
    v1 = (int)v2;
    v3 = 1;
    if (!sub_d3f0(v2 & 0xffffffff)) {
      if ((2 <= (unsigned int)(v1 - 0x2dU)) && (v1 != 0x7e))
        v3 = v1 == 0x5f;
      else {
        v3 = 1;
      }
    }
    *(unsigned char *)(v2 + 0x26fe0) = *(unsigned char *)(v2 + 0x26fe0) | v3;
    v2 += 1;
  } while (v2 != 0x100);
}

// Function: sub_56d0 @ 0x56d0
unsigned long sub_56d0(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_56e0 @ 0x56e0
unsigned long sub_56e0(long *a0,long *a1)
{
  if (*a0 != *a1)
    return 0;
  return CONCAT71((undefined7)((unsigned long)a1[1] >> 8),a0[1] == a1[1]);
}

// Function: sub_5700 @ 0x5700
void sub_5700(unsigned int a0) // return-dupe
{
  if (dat_27218)
    return;
  dat_27218 = a0;
}

// Function: sub_5720 @ 0x5720
void sub_5720(void) // return-dupe
{
  if (dat_27218)
    return;
  dat_27214 += 1;
}

// Function: sub_5740 @ 0x5740
unsigned int sub_5740(long *a0,long *a1,unsigned int a2,long *a3)
{
  unsigned char v1;
  long v10;
  long v11;
  char *v12;
  bool v13;
  unsigned int v2; // eax
  char *v3;
  char *v4;
  char v5;
  char v6;
  long v7;
  long v8;
  long v9; // rdi
  
  v3 = (char *)*a1;
  v11 = *a0;
  v9 = 1;
  v5 = *v3;
  v10 = 0;
  v8 = v11 + 1;
  v6 = v5 + '\xa4';
  v13 = 0;
  v4 = v3;
  v7 = v8;
  if (v5 == '\\') goto label_57a6;
label_577a:
  if (v13 || SBORROW1(v5,'\\') != v6 < '\0') {
    if (v5 != '=') { // branch-flip
      if ((v5 <= '=') && ((!v5 || (v5 == ':')))) {
        v2 = 1;
label_57de:
        *a0 = v11;
        *a1 = (long)v3;
        *a3 = v10;
        return v2;
      }
    }
    else {
      v2 = a2;
      if ((char)a2) goto label_57de;
    }
  }
  else if (v5 == '^') {
    v1 = v3[1];
    if ((unsigned char)(v1 - 0x40) <= 0x3e) {
      v3 = &v3[2];
      v9 += 1;
      v8 = v7 + 1;
      *(unsigned char *)(v7 + -1) = v1 & 0x1f;
      goto label_5796;
    }
    v3 = &v3[1];
    if (v1 == 0x3f) {
      v9 += 1;
      v8 = v7 + 1;
      *(char *)(v7 + -1) = 0x7f;
      goto label_5796;
    }
    v2 = 0;
    goto label_57de;
  }
  v3 = &v3[1];
  v8 = v7;
label_578b:
  *(char *)(v8 + -1) = v5;
  v9 += 1;
  v8 += 1;
label_5796:
  do {
    v5 = *v3;
    v11 = v8 + -1;
    v10 = v9 + -1;
    v6 = v5 + '\xa4';
    v13 = v6 == '\0';
    v4 = v3;
    v7 = v8;
    if (!v13) goto label_577a;
label_57a6:
    v6 = v4[1];
    v3 = &v4[2];
    if (!v6) {
      v2 = 0;
      goto label_57de;
    }
    v5 = v6 + '\xd0';
    switch(v5) {
      case 0:
      case 1:
      case 2:
      case 3:
      case 4:
      case 5:
      case 6:
      case 7:
        v6 = *v3;
        if (8 <= (unsigned char)(v6 - 0x30U)) goto label_578b;
        do {
          v3 = &v3[1];
          v5 = v6 + '\xd0' + v5 * '\b';
          v6 = *v3;
        } while ((unsigned char)(v6 - 0x30U) <= 7);
        *(char *)(v8 + -1) = v5;
        v9 += 1;
        v8 += 1;
        goto label_5796;
      default:
        goto label_5820;
      case 0xf:
        v6 = '\x7f';
        break;
      case 0x28:
      case 0x48:
        goto label_58b7;
      case 0x2f:
        v6 = ' ';
        break;
      case 0x31:
        v6 = '\a';
        goto label_5820;
      case 0x32:
        v6 = '\b';
        goto label_5820;
      case 0x35:
        v6 = '\x1b';
        break;
      case 0x36:
        v6 = '\f';
        break;
      case 0x3e:
        v6 = '\n';
        break;
      case 0x42:
        v6 = '\r';
        break;
      case 0x44:
        v6 = '\t';
        break;
      case 0x46:
        v6 = '\v';
label_5820:
      
    }
    *(char *)(v8 + -1) = v6;
    v9 += 1;
    v8 += 1;
  } while( true );
label_58b7:
  v6 = v4[2];
  v12 = &v4[3];
  v5 = '\0';
  v3 = &v4[2];
  if ('F' < v6) goto label_5902;
  do {
    if ('A' <= v6) { // branch-flip
      v5 = v6 + '\xc9' + v5 * '\x10';
      v3 = v12;
    }
    else {
      if (10 <= (unsigned char)(v6 - 0x30U)) goto label_578b;
      v5 = v6 + '\xd0' + v5 * '\x10';
      v3 = v12;
    }
    while( true ) {
      v6 = *v3;
      v12 = &v3[1];
      if (v6 <= 'F') break;
label_5902:
      if (6 <= (unsigned char)(v6 + 0x9fU)) goto label_578b;
      v5 = v6 + '\xa9' + v5 * '\x10';
      v3 = v12;
    }
  } while( true );
}

// Function: sub_59b0 @ 0x59b0
void sub_59b0(char a0) // return-dupe
{
  if (a0) {
    dat_27210 = 2;
    return;
  }
  if (dat_27210)
    return;
  dat_27210 = 1;
}

// Function: sub_59e0 @ 0x59e0
unsigned long sub_59e0(unsigned long a0)
{
  return a0;
}

// Function: sub_59f0 @ 0x59f0
unsigned long sub_59f0(int a0)
{
  if (a0 - 0x16U & 0xffffffefU)
    return sub_55c0(); // tail-call
  return 1;
}

// Function: sub_5a10 @ 0x5a10
unsigned long sub_5a10(long a0)
{
  int v1;
  unsigned long v2;
  bool v3; // al
  unsigned int v4; // eax
  unsigned long v5; // rax
  undefined7 v6; // rax
  
  v1 = *(int *)(a0 + 0xa8);
  v2 = v5 >> 8;
  v6 = (undefined7)v2;
  v3 = v1 == 3 || v1 == 9;
  if (v1 != 3 && v1 != 9) {
    v4 = *(unsigned int *)(a0 + 0xac) & 0xf000;
    v5 = (unsigned long)CONCAT31((undefined3)(v4 >> 8),v4 == 0x4000);
  }
  return v5;
}

// Function: sub_5a40 @ 0x5a40
void sub_5a40(unsigned long a0,unsigned long a1,void *a2)
{
  unsigned int v1; // eax
  unsigned int v2; // eax
  
  v1 = sub_5a10(a1);
  v2 = sub_5a10(a0);
  if ((v1 & 0xff) == (v2 & 0xff)) {
    (*a2)(); // jump-as-call
    return;
  }
}

// Function: sub_5a80 @ 0x5a80
void sub_5a80(unsigned long *a0,unsigned long *a1,void *a2)
{
  char v1 [16];
  char v2 [16];
  
  v1 = sub_15ca0(&a0[3]);
  v2 = sub_15ca0(&a1[3]);
  if (!sub_162a0(SUB168(v2,0),SUB168(v2,8),SUB168(v1,0),SUB168(v1,8))) {
    (*a2)(*a0,*a1); // jump-as-call
    return;
  }
}

// Function: sub_5af0 @ 0x5af0
void sub_5af0(unsigned long *a0,unsigned long *a1,void *a2)
{
  char v1 [16];
  char v2 [16];
  
  v1 = sub_15cb0(&a0[3]);
  v2 = sub_15cb0(&a1[3]);
  if (!sub_162a0(SUB168(v2,0),SUB168(v2,8),SUB168(v1,0),SUB168(v1,8))) {
    (*a2)(*a0,*a1); // jump-as-call
    return;
  }
}

// Function: sub_5b60 @ 0x5b60
void sub_5b60(unsigned long *a0,unsigned long *a1,void *a2)
{
  char v1 [16];
  char v2 [16];
  
  v1 = sub_15c90(&a0[3]);
  v2 = sub_15c90(&a1[3]);
  if (!sub_162a0(SUB168(v2,0),SUB168(v2,8),SUB168(v1,0),SUB168(v1,8))) {
    (*a2)(*a0,*a1); // jump-as-call
    return;
  }
}

// Function: sub_5bd0 @ 0x5bd0
void sub_5bd0(unsigned long *a0,unsigned long *a1,void *a2)
{
  char v1 [16];
  char v2 [16];
  
  v1 = sub_5610(&a0[3]);
  v2 = sub_5610(&a1[3]);
  if (!sub_162a0(SUB168(v2,0),SUB168(v2,8),SUB168(v1,0),SUB168(v1,8))) {
    (*a2)(*a0,*a1); // jump-as-call
    return;
  }
}

// Function: sub_5c40 @ 0x5c40
unsigned long sub_5c40(long a0,long a1) // early-return
{
  if (a1 <= a0)
    return (unsigned long)(a1 < a0);
  return 0xffffffff;
}

// Function: sub_5c60 @ 0x5c60
void sub_5c60(unsigned long a0,unsigned long a1)
{
  sub_5a80(a0,a1,sub_72a0); // tail-call
}

// Function: sub_5c70 @ 0x5c70
void sub_5c70(unsigned long a0,unsigned long a1)
{
  sub_5a80(a0,a1,dat_25fc0); // tail-call
}

// Function: sub_5c80 @ 0x5c80
void sub_5c80(unsigned long a0,unsigned long a1)
{
  sub_5a80(a1,a0,sub_72a0); // tail-call
}

// Function: sub_5ca0 @ 0x5ca0
void sub_5ca0(unsigned long a0,unsigned long a1)
{
  sub_5a80(a1,a0,dat_25fc0); // tail-call
}

// Function: sub_5cc0 @ 0x5cc0
void sub_5cc0(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_5c60); // tail-call
}

// Function: sub_5cd0 @ 0x5cd0
void sub_5cd0(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_5c70); // tail-call
}

// Function: sub_5ce0 @ 0x5ce0
void sub_5ce0(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_5c80); // tail-call
}

// Function: sub_5cf0 @ 0x5cf0
void sub_5cf0(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_5ca0); // tail-call
}

// Function: sub_5d00 @ 0x5d00
void sub_5d00(unsigned long a0,unsigned long a1)
{
  sub_5af0(a0,a1,sub_72a0); // tail-call
}

// Function: sub_5d10 @ 0x5d10
void sub_5d10(unsigned long a0,unsigned long a1)
{
  sub_5af0(a0,a1,dat_25fc0); // tail-call
}

// Function: sub_5d20 @ 0x5d20
void sub_5d20(unsigned long a0,unsigned long a1)
{
  sub_5af0(a1,a0,sub_72a0); // tail-call
}

// Function: sub_5d40 @ 0x5d40
void sub_5d40(unsigned long a0,unsigned long a1)
{
  sub_5af0(a1,a0,dat_25fc0); // tail-call
}

// Function: sub_5d60 @ 0x5d60
void sub_5d60(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_5d00); // tail-call
}

// Function: sub_5d70 @ 0x5d70
void sub_5d70(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_5d10); // tail-call
}

// Function: sub_5d80 @ 0x5d80
void sub_5d80(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_5d20); // tail-call
}

// Function: sub_5d90 @ 0x5d90
void sub_5d90(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_5d40); // tail-call
}

// Function: sub_5da0 @ 0x5da0
void sub_5da0(unsigned long a0,unsigned long a1)
{
  sub_5b60(a0,a1,sub_72a0); // tail-call
}

// Function: sub_5db0 @ 0x5db0
void sub_5db0(unsigned long a0,unsigned long a1)
{
  sub_5b60(a0,a1,dat_25fc0); // tail-call
}

// Function: sub_5dc0 @ 0x5dc0
void sub_5dc0(unsigned long a0,unsigned long a1)
{
  sub_5b60(a1,a0,sub_72a0); // tail-call
}

// Function: sub_5de0 @ 0x5de0
void sub_5de0(unsigned long a0,unsigned long a1)
{
  sub_5b60(a1,a0,dat_25fc0); // tail-call
}

// Function: sub_5e00 @ 0x5e00
void sub_5e00(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_5da0); // tail-call
}

// Function: sub_5e10 @ 0x5e10
void sub_5e10(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_5db0); // tail-call
}

// Function: sub_5e20 @ 0x5e20
void sub_5e20(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_5dc0); // tail-call
}

// Function: sub_5e30 @ 0x5e30
void sub_5e30(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_5de0); // tail-call
}

// Function: sub_5e40 @ 0x5e40
void sub_5e40(unsigned long a0,unsigned long a1)
{
  sub_5bd0(a0,a1,sub_72a0); // tail-call
}

// Function: sub_5e50 @ 0x5e50
void sub_5e50(unsigned long a0,unsigned long a1)
{
  sub_5bd0(a0,a1,dat_25fc0); // tail-call
}

// Function: sub_5e60 @ 0x5e60
void sub_5e60(unsigned long a0,unsigned long a1)
{
  sub_5bd0(a1,a0,sub_72a0); // tail-call
}

// Function: sub_5e80 @ 0x5e80
void sub_5e80(unsigned long a0,unsigned long a1)
{
  sub_5bd0(a1,a0,dat_25fc0); // tail-call
}

// Function: sub_5ea0 @ 0x5ea0
void sub_5ea0(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_5e40); // tail-call
}

// Function: sub_5eb0 @ 0x5eb0
void sub_5eb0(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_5e50); // tail-call
}

// Function: sub_5ec0 @ 0x5ec0
void sub_5ec0(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_5e60); // tail-call
}

// Function: sub_5ed0 @ 0x5ed0
void sub_5ed0(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_5e80); // tail-call
}

// Function: sub_5ee0 @ 0x5ee0
void sub_5ee0(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_a1a0); // tail-call
}

// Function: sub_5ef0 @ 0x5ef0
void sub_5ef0(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_a170); // tail-call
}

// Function: sub_5f00 @ 0x5f00
void sub_5f00(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_a150); // tail-call
}

// Function: sub_5f10 @ 0x5f10
void sub_5f10(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_a130); // tail-call
}

// Function: sub_5f20 @ 0x5f20
void sub_5f20(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_a0e0); // tail-call
}

// Function: sub_5f30 @ 0x5f30
void sub_5f30(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_a0c0); // tail-call
}

// Function: sub_5f40 @ 0x5f40
void sub_5f40(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_a0a0); // tail-call
}

// Function: sub_5f50 @ 0x5f50
void sub_5f50(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_a080); // tail-call
}

// Function: sub_5f60 @ 0x5f60
void sub_5f60(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_73a0); // tail-call
}

// Function: sub_5f70 @ 0x5f70
void sub_5f70(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_7370); // tail-call
}

// Function: sub_5f80 @ 0x5f80
void sub_5f80(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_7380); // tail-call
}

// Function: sub_5f90 @ 0x5f90
void sub_5f90(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_7350); // tail-call
}

// Function: sub_5fa0 @ 0x5fa0
void sub_5fa0(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_8970); // tail-call
}

// Function: sub_5fb0 @ 0x5fb0
void sub_5fb0(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_8940); // tail-call
}

// Function: sub_5fc0 @ 0x5fc0
void sub_5fc0(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_8950); // tail-call
}

// Function: sub_5fd0 @ 0x5fd0
void sub_5fd0(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_8920); // tail-call
}

// Function: sub_5fe0 @ 0x5fe0
void sub_5fe0(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_a050); // tail-call
}

// Function: sub_5ff0 @ 0x5ff0
void sub_5ff0(unsigned long a0,unsigned long a1)
{
  sub_5a40(a0,a1,sub_a060); // tail-call
}

// Function: sub_6000 @ 0x6000
void sub_6000(void) // return-dupe
{
  long *v1;
  long v2;
  
  if (!dat_273b0)
    return;
  v1 = dat_273a0;
  v2 = dat_273c0;
  do {
    *v1 = v2;
    v1 = &v1[1];
    v2 += 0xd0;
  } while (v1 != &dat_273a0[dat_273b0]);
}

// Function: sub_6040 @ 0x6040
unsigned char sub_6040(char a0,unsigned int a1,int a2)
{
  unsigned int v1; // ecx
  bool v2;
  
  if (a0) { // branch-flip
    v1 = a1 & 0xf000;
    if (v1 == 0x8000) {
      if (dat_27314 != 3)
        return 0;
      return -((a1 & 0x49) != 0) & 0x2a;
    }
    if (v1 == 0x4000)
      return 0x2f;
    v2 = v1 == 0xa000;
    if (dat_27314 == 1)
      return 0;
  }
  else {
    if (a2 == 5)
      return 0;
    if ((a2 == 3) || (a2 == 9))
      return 0x2f;
    v2 = a2 == 6;
    if (dat_27314 == 1)
      return 0;
  }
  if (v2)
    return 0x40;
  if (a0) { // branch-flip
    if ((a1 & 0xf000) == 0x1000)
      return 0x7c;
    v2 = (a1 & 0xf000) == 0xc000;
  }
  else {
    if (a2 == 1)
      return 0x7c;
    v2 = a2 == 7;
  }
  return -v2 & 0x3d;
}

// Function: sub_6140 @ 0x6140
void sub_6140(char *a0,char *a1,char *a2)
{
  char v1;
  char v2;
  char *v3; // rcx
  char *v4;
  char *v5; // rdi
  
  v2 = *a1;
  v1 = v2;
  if (v2 == '.')
    v1 = a1[1];
  if (!v1) goto label_618a;
  v4 = a1;
  do {
    v5 = a0;
    v3 = v4;
    v4 = &v3[1];
    a0 = &v5[1];
    *v5 = v2;
    v2 = *v4;
  } while (v2);
  if (v4 <= a1) goto label_618a;
  if (*v3 == '/') goto label_618a;
  *a0 = '/';
  a0 = &v5[2];
  v2 = *a2;
  while (v2) {
    *a0 = v2;
    a0 = &a0[1];
    a2 = &a2[1];
label_618a:
    v2 = *a2;
  }
  *a0 = '\0';
}

// Function: sub_61a0 @ 0x61a0
void sub_61a0(void *a0)
{
  free(a0); // tail-call
}

// Function: sub_61b0 @ 0x61b0
void sub_61b0(unsigned long *a0)
{
  free((void *)*a0);
  free((void *)a0[1]);
  free(a0); // tail-call
}

// Function: sub_61d0 @ 0x61d0
void sub_61d0(unsigned long a0) // return-dupe
{
  unsigned long *v1;
  char v2 [16];
  char v3 [16];
  long v4; // rax
  unsigned long *v5;
  long v6;
  unsigned long v7;
  
  if (dat_262e0 < a0) {
    if ((dat_27200) && (dat_27200 >> 1 <= a0)) {
      dat_27208 = sub_16ad0(dat_27208,dat_27200,0x18);
      v7 = dat_27200;
    }
    else {
      dat_27208 = sub_16ad0(dat_27208,a0,0x30);
      v7 = a0 * 2;
    }
    v2._8_8_ = 0;
    v2._0_8_ = v7 - dat_262e0;
    v3._8_8_ = 0;
    v3._0_8_ = dat_262e0 + 1 + v7;
    if ((CARRY8(dat_262e0 + 1,v7)) || (SUB168(v2._0_16_ * v3._0_16_,8)))
      sub_16da0(); // no-return
    v4 = sub_16b00(SUB168(v2._0_16_ * v3._0_16_,0) >> 1,8);
    if (dat_262e0 < v7) {
      v6 = dat_262e0 * 8 + 8;
      do {
        *(long *)(dat_27208 + -8 + v6 * 3) = v4;
        v4 += v6;
        v6 += 8;
      } while (v6 != v7 * 8 + 8);
    }
    dat_262e0 = v7;
  }
  if (!a0)
    return;
  v6 = 0;
  v4 = 3;
  do {
    v5 = *(unsigned long **)(dat_27208 + -8 + v4 * 8);
    *(char *)(dat_27208 + -0x18 + v4 * 8) = 1;
    v6 += 8;
    *(long *)(dat_27208 + -0x10 + v4 * 8) = v4;
    v1 = (unsigned long *)(v6 + (long)v5);
    do {
      *v5 = 3;
      v5 = &v5[1];
    } while (v1 != v5);
    v4 += 3;
  } while (a0 * 3 + 3 != v4);
}

// Function: sub_6330 @ 0x6330
undefined16 sub_6330(void)
{
  long v1;
  
  v1 = dat_270f8;
  if (0x10 <= (unsigned long)(dat_270f8 - dat_270f0)) {
    dat_270f8 -= 0x10;
    return *(char (*)[16])(v1 + -0x10);
  }
  __assert_fail("dev_ino_size <= obstack_object_size (&dev_ino_obstack)","src/ls.c",0x41d,"dev_ino_pop"); // no-return
}

// Function: sub_6380 @ 0x6380
void sub_6380(unsigned long a0)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_169e0(0x10);
  *v1 = a0;
  v1[1] = dat_272e8;
  dat_272e8 = v1;
}

// Function: sub_63b0 @ 0x63b0
long sub_63b0(unsigned long a0) // early-return
{
  int v1; // eax
  long v2; // stack - 0x18
  
  v1 = sub_17190(a0,0,0,&v2,0x1abb9);
  if (v1) // branch-flip
    v2 = -(unsigned long)(v1 != 1);
  else if (v2 <= -1)
    return 0;
  return v2;
}

// Function: sub_6420 @ 0x6420
unsigned long sub_6420(unsigned int a0)
{
  long v1;
  char *v2;
  unsigned long v3;
  
  v3 = 0;
  v1 = *(long *)((unsigned long)a0 * 0x10 + 0x26060);
  if (!v1)
    return 0;
  v2 = *(char **)((unsigned long)a0 * 0x10 + 0x26068);
  if (v1 != 1) {
    v3 = 0;
    if (v1 != 2)
      return 1;
    return CONCAT71((undefined7)((unsigned long)v3 >> 8),strncmp(v2,"00",2) != 0) & 0xffffffff;
  }
  return CONCAT71((undefined7)((unsigned long)v3 >> 8),strncmp(v2,"0",1) != 0) & 0xffffffff;
}

// Function: sub_64b0 @ 0x64b0
unsigned int sub_64b0(void) // early-return x2
{
  int v1; // eax
  char *v2; // rax
  unsigned long v3; // rax
  
  v2 = getenv("QUOTING_STYLE");
  if (!v2)
    return 0xffffffff;
  v1 = sub_cf70(v2,0x259e0,0x1d9c0,4);
  if (0 <= v1)
    return *(unsigned int *)((long)v1 * 4 + 0x1d9c0);
  v3 = sub_158c0(v2);
  error(0,0,dcgettext(NULL,"ignoring invalid value of environment variable QUOTING_STYLE: %s",5),v3);
  return 0xffffffff;
}

// Function: sub_6530 @ 0x6530
void sub_6530(void)
{
  void *v1;
  
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v1); // tail-call
}

// Function: sub_6560 @ 0x6560
void sub_6560(void)
{
  void *v1;
  
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"\nThe SIZE argument is an integer and optional unit (example: 10K is 10*1024).\nUnits are K,M,G,T,P,E,Z,Y (powers of 1024) or KB,MB,... (powers of 1000).\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n",5),v1); // tail-call
}

// Function: sub_6590 @ 0x6590
unsigned int sub_6590(void)
{
  unsigned int v1; // eax
  
  if ('\0' <= (char)dat_26018)
    return dat_26018 & 1;
  v1 = isatty(1);
  dat_26018 = (char)v1;
  return v1 & 1;
}

// Function: sub_65c0 @ 0x65c0
unsigned long sub_65c0(long a0)
{
  char *v1; // rax
  void *v2; // rax
  unsigned long v3;
  unsigned long v4; // stack - 0x48
  long v5;
  unsigned long v6;
  int v7; // r15d
  
  v6 = 0xc;
  do {
    v7 = 0x2000e;
    v3 = 0;
    v5 = a0;
    do {
      v4 = v6;
      v1 = nl_langinfo(v7);
      if (strchr(v1,0x25))
        return 0;
      v2 = __ctype_b_loc();
      sub_5570((int)*v1,v5,v2,&v4);
      if (0x80 <= (unsigned long)sub_10df0(v1))
        return 0;
      if (v3 < v4)
        v3 = v4;
      v7 += 1;
      v5 += 0x80;
    } while (v7 != 0x2001a);
    if (v6 <= v3)
      return 1;
    v6 = v3;
  } while( true );
}

// Function: sub_66c0 @ 0x66c0
void sub_66c0(void) // return-dupe x2
{
  long v1;
  long v2;
  unsigned int v3; // eax
  long v4; // rax
  char *v5;
  char v6 [1536];
  char v7 [8];
  long v8 [2]; // stack - 0x658
  char *v9;
  
  v4 = sub_5620(dat_26040);
  v8[0] = v4;
  v8[1] = sub_5620(dat_26048);
  if (!v4 && !v8[1])
    return;
  if (!sub_65c0(v6))
    return;
  v4 = 0;
  do {
    v1 = *(long *)(v4 * 8 + 0x26040);
    v2 = v8[v4];
    v9 = (char *)(v4 * 0x600 + 0x263e0);
    v5 = v6;
    do {
      if (v2) { // branch-flip
        if (0x80 < v2 - v1)
          return;
        v3 = __snprintf_chk(v9,0x80,1,0xffffffffffffffff,"%.*s%s%s",v2 - v1,v1,v5,v2 + 2);
      }
      else {
        v3 = snprintf(v9,0x80,"%s",v1);
      }
      if (0x7f < v3)
        return;
      v5 = &v5[0x80];
      v9 = &v9[0x80];
    } while (v5 != v7);
    if (v4 == 1) {
      dat_263c8 = 1;
      return;
    }
    v4 = 1;
  } while( true );
}

// Function: sub_6840 @ 0x6840
void sub_6840(long a0,long a1,char a2)
{
  unsigned long *v1; // rax
  unsigned long v2;
  
  v1 = (unsigned long *)sub_169e0(0x20);
  v2 = 0;
  if (a1)
    v2 = sub_16d80(a1);
  v1[1] = v2;
  v2 = 0;
  if (a0)
    v2 = sub_16d80(a0);
  *v1 = v2;
  *(char *)&v1[2] = a2;
  v1[3] = dat_27380;
  dat_27380 = v1;
}

// Function: sub_68b0 @ 0x68b0
unsigned int sub_68b0(void)
{
  if (dat_27334 <= 3)
    return *(unsigned int *)&"@"[(unsigned long)dat_27334 * 4];
  abort(); // no-return
}

// Function: sub_68d0 @ 0x68d0
unsigned int sub_68d0(void)
{
  unsigned int v1; // r12d
  
  v1 = (-(unsigned int)(dat_272fc == '\0') & 0xffffff00) + 0x102;
  if (dat_2732c)
    v1 |= 0x400;
  if (!dat_27338) {
    v1 |= sub_68b0();
    if ((dat_26029) || (dat_2732e))
      v1 |= 0x20c;
    else {
      v1 |= 0x204;
    }
    if (dat_26028)
      v1 |= 0x10;
  }
  switch(dat_27330) {
    case 0:
    case 1:
    case 2:
    case 4:
    case 6:
      break;
    case 3:
      return v1 | 0x200;
    case 5:
      v1 |= sub_68b0();
      break;
    default:
      abort(); // no-return
    
  }
  return v1;
}

// Function: sub_6990 @ 0x6990
unsigned long sub_6990(unsigned long *a0,char *a1)
{
  char *v1;
  
  if (!a0)
    return 0;
  do {
    v1 = (char *)*a0;
    if (!fnmatch(v1,a1,4))
      return 1;
    a0 = (unsigned long *)a0[1];
  } while (a0);
  return 0;
}

// Function: sub_69f0 @ 0x69f0
unsigned long sub_69f0(char *a0)
{
  char v1; // al
  unsigned long v2; // rax
  
  if (dat_272f0 != 2) {
    if (*a0 != '.') { // branch-flip
      if ((!dat_272f0) && (v1 = sub_6990(dat_272e0,a0), v1))
        return 1;
    }
    else if ((!dat_272f0) || (!a0[(unsigned long)(a0[1] == '.') + 1]))
      return 1;
  }
  v2 = sub_6990(dat_272e8,a0); // tail-call
  return v2;
}

// Function: sub_6a50 @ 0x6a50
unsigned long sub_6a50(void)
{
  char *v1; // rax
  char *v2;
  
  v1 = getenv("TERM");
  if ((v1) && (*v1)) {
    v2 = "# Configuration file for dircolors, a utility to help you set the";
    do {
      if ((!strncmp(v2,"TERM ",5)) && (!fnmatch(&v2[5],v1,0)))
        return 1;
      v2 = &v2[strlen(v2) + 1];
    } while (&v2[-0x19780] <= "_r");
  }
  return 0;
}

// Function: sub_6af0 @ 0x6af0
void sub_6af0(void)
{
  char *v1;
  char *v10; // stack - 0x58
  void *v11; // stack - 0x50
  char v12; // stack - 0x41
  char v2;
  void *v3;
  int v4; // eax
  char *v5;
  unsigned long v6; // rax
  void *v7;
  long v8; // rcx
  unsigned short v9; // stack - 0x43
  
  v10 = getenv("LS_COLORS");
  if ((v10) && (*v10)) {
    v9 = 0x3f3f;
    v12 = 0;
    dat_27300 = (void *)sub_16d80(v10);
    v11 = dat_27300;
    do {
      while( true ) {
        while (v2 = *v10, v2 == '*') {
          v5 = &v10[1];
          v7 = (void *)sub_169e0(0x28);
          *(void **)((long)v7 + 0x20) = dat_27308;
          *(void **)((long)v7 + 8) = v11;
          dat_27308 = v7;
          v10 = v5;
          v5 = v10;
          if ((!sub_5740(&v11,&v10,1,v7)) || (v5 = &v10[1], *v10 != '=')) goto label_6c65;
          *(void **)((long)v7 + 0x18) = v11;
          v10 = v5;
          v5 = v10;
          if (!sub_5740(&v11,&v10,0,(long)v7 + 0x10)) goto label_6c65;
        }
        if (v2 != ':') break;
        v10 = &v10[1];
      }
      if (!v2) goto label_6cb8;
      v9 = CONCAT11(v9._1_1_,v2);
      v5 = &v10[1];
      if (!v10[1]) goto label_6c65;
      v9 = CONCAT11(v10[1],v2);
      v1 = &v10[2];
      v10 = &v10[3];
      v5 = v10;
      if (*v1 != '=') goto label_6c65;
      v8 = 0;
      v5 = "lc";
      while (strcmp((char *)&v9,v5)) {
        v8 += 1;
        v5 = *(char **)(v8 * 8 + 0x25860);
        if (!v5) goto label_6c36;
      }
      v8 = (long)(int)v8 * 0x10;
      *(void **)(v8 + 0x26068) = v11;
    } while (sub_5740(&v11,&v10,0,v8 + 0x26060));
label_6c36:
    v6 = sub_158c0(&v9);
    error(0,0,dcgettext(NULL,"unrecognized prefix: %s",5),v6);
    v5 = v10;
label_6c65:
    v10 = v5;
    error(0,0,dcgettext(NULL,"unparsable value for LS_COLORS environment variable",5));
    free(dat_27300);
    v7 = dat_27308;
    while (v7) {
      v3 = *(void **)((long)v7 + 0x20);
      free(v7);
      v7 = v3;
    }
    dat_27312 = 0;
label_6cb8:
    if ((dat_260d0 == 6) && (v4 = strncmp(dat_260d8,"target",6), !v4))
      dat_27390 = 1;
  }
  else {
    v5 = getenv("COLORTERM");
    if (((!v5) || (!*v5)) && (!sub_6a50()))
      dat_27312 = 0;
  }
}

// Function: sub_6dc0 @ 0x6dc0
void sub_6dc0(unsigned long *a0)
{
  void *v1;
  
  free((void *)*a0);
  free((void *)a0[1]);
  free((void *)a0[2]);
  v1 = (void *)a0[0x16];
  if (v1 == (void *)0x2602a)
    return;
  if (sub_55d0()) {
    free(v1); // tail-call
    return;
  }
  freecon(); // tail-call
}

// Function: sub_6e20 @ 0x6e20
void sub_6e20(void)
{
  long v1;
  unsigned long v2; // rbx
  
  if (dat_273b0) {
    v2 = 0;
    do {
      v1 = v2 * 8;
      v2 += 1;
      sub_6dc0(*(unsigned long *)(dat_273a0 + v1));
    } while (v2 < dat_273b0);
  }
  dat_273b0 = 0;
  dat_273a9 = 0;
  dat_27364 = 0;
  dat_27360 = 0;
  dat_2735c = 0;
  dat_27358 = 0;
  dat_27350 = 0;
  dat_2734c = 0;
  dat_27348 = 0;
  dat_27354 = 0;
  dat_27344 = 0;
  dat_27340 = 0;
  dat_2733c = 0;
}

// Function: sub_6ed0 @ 0x6ed0
unsigned long sub_6ed0(long a0,unsigned long a1) // early-return
{
  unsigned int v1; // eax
  
  if (!a0)
    return __snprintf_chk(NULL,0,1,0xffffffffffffffff,"%lu",a1); // tail-call
  v1 = sub_113a0(a0,0);
  if (0 <= (int)v1)
    return (unsigned long)v1;
  return 0;
}

// Function: sub_6f10 @ 0x6f10
char * sub_6f10(unsigned long a0,unsigned long a1,long a2)
{
  long v1;
  
  if (!*(char *)(a2 + 0xb8))
    return "?";
  v1 = *(long *)(a2 + 0x20);
  if (!v1)
    return "?";
  return (char *)sub_10c20(v1,a0); // tail-call
}

// Function: sub_6f50 @ 0x6f50
void sub_6f50(char a0,char *a1,unsigned long a2)
{
  unsigned long v1; // rax
  
  v1 = sub_15600(4,a2);
  error(0,*__errno_location(),a1,v1);
  sub_59b0(a0); // tail-call
}

// Function: sub_6f90 @ 0x6f90
int sub_6f90(unsigned long a0,long a1,char a2)
{
  int v1;
  int v2;
  
  if (*(long *)(a1 + 0x18) != dat_262f0) { // branch-flip
    if (a2) // branch-flip
      v2 = sub_15b10(a0,a1 + 0xb0);
    else {
      v2 = sub_15b30();
    }
    if (v2 <= -1) {
      v1 = *__errno_location();
      if (sub_59f0(v1)) {
        dat_262f0 = *(unsigned long *)(a1 + 0x18);
        return v2;
      }
    }
  }
  else {
    v2 = -1;
    *__errno_location() = 0x5f;
  }
  return v2;
}

// Function: sub_7020 @ 0x7020
unsigned long sub_7020(unsigned long a0,long a1)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  unsigned long v4; // r8
  
  v3 = __errno_location();
  if (*(long *)(a1 + 0x18) == dat_262e8) {
    *v3 = 0x5f;
    return 0;
  }
  *v3 = 0;
  v2 = sub_e060(a0,a1 + 0x18);
  v4 = (unsigned long)v2;
  if ((int)v2 <= 0) {
    v1 = *v3;
    if (sub_59f0(v1)) {
      dat_262e8 = *(unsigned long *)(a1 + 0x18);
      return v4 & 0xffffffff;
    }
  }
  return v4 & 0xffffffff;
}

// Function: sub_70b0 @ 0x70b0
void sub_70b0(unsigned long a0,long a1,char a2)
{
  long v1; // rax
  
  v1 = sub_cdd0(a0,*(unsigned long *)(a1 + 0x48));
  *(long *)(a1 + 8) = v1;
  if (v1)
    return;
  sub_6f50(a2,dcgettext(NULL,"cannot read symbolic link %s",5),a0); // tail-call
}

// Function: sub_7100 @ 0x7100
char * sub_7100(char *a0,char *a1)
{
  unsigned long v1; // rax
  char *v2; // rax
  
  if (a1) { // branch-flip
    if ((*a1 == '/') || (v1 = sub_dfb0(a0), !v1))
      return (char *)sub_16d80(a1); // tail-call
    v2 = (char *)sub_169e0(v1 + 2 + strlen(a1));
    if (a0[v1 - 1] != '/')
      v1 += 1;
    strcpy(stpncpy(v2,a0,v1),a1);
  }
  else {
    v2 = NULL;
  }
  return v2;
}

// Function: sub_71a0 @ 0x71a0
unsigned long sub_71a0(char *a0) // early-return
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char v3 [2]; // stack - 0x22
  
  v1 = sub_152c0(v3,2,a0,0xffffffffffffffff,dat_272d0);
  if (*a0 != v3[0])
    return 1;
  v2 = strlen(a0);
  return CONCAT71((undefined7)(v2 >> 8),v2 != v1);
}

// Function: sub_7220 @ 0x7220
void sub_7220(unsigned long a0)
{
  if (dat_2732d) {
    sub_6ed0(0,a0 & 0xffffffff); // tail-call
    return;
  }
  sub_6ed0(sub_109e0(a0),a0 & 0xffffffff); // tail-call
}

// Function: sub_7260 @ 0x7260
void sub_7260(unsigned long a0)
{
  if (dat_2732d) {
    sub_6ed0(0,a0 & 0xffffffff); // tail-call
    return;
  }
  sub_6ed0(sub_10840(a0),a0 & 0xffffffff); // tail-call
}

// Function: sub_72a0 @ 0x72a0
void sub_72a0(char *a0,char *a1)
{
  *__errno_location() = 0;
  strcoll(a0,a1); // tail-call
}

// Function: sub_72d0 @ 0x72d0
void sub_72d0(unsigned long *a0,unsigned long *a1,void *a2)
{
  char *v1; // rax
  char *v2; // rax
  
  v1 = strrchr((char *)*a0,0x2e);
  v2 = strrchr((char *)*a1,0x2e);
  if (!v2)
    v2 = "";
  if (!v1)
    v1 = "";
  if (!(*a2)(v1,v2)) {
    (*a2)(*a0,*a1); // jump-as-call
    return;
  }
}

// Function: sub_7350 @ 0x7350
void sub_7350(unsigned long a0,unsigned long a1)
{
  sub_72d0(a1,a0,dat_25fc0); // tail-call
}

// Function: sub_7370 @ 0x7370
void sub_7370(unsigned long a0,unsigned long a1)
{
  sub_72d0(a0,a1,dat_25fc0); // tail-call
}

// Function: sub_7380 @ 0x7380
void sub_7380(unsigned long a0,unsigned long a1)
{
  sub_72d0(a1,a0,sub_72a0); // tail-call
}

// Function: sub_73a0 @ 0x73a0
void sub_73a0(unsigned long a0,unsigned long a1)
{
  sub_72d0(a0,a1,sub_72a0); // tail-call
}

// Function: sub_73b0 @ 0x73b0
void sub_73b0(unsigned long a0)
{
  sub_5580(sub_d360(a0)); // tail-call
}

// Function: sub_73d0 @ 0x73d0
unsigned long sub_73d0(unsigned long a0,unsigned long a1)
{
  unsigned long v1; // rax
  unsigned long v2;
  
  if (a1 <= a0)
    return v1;
  do {
    while ((v2 = a0 + 1, !dat_272c0 || (a1 / dat_272c0 <= v2 / dat_272c0))) {
      a0 = v2;
      if (a1 <= v2)
        return CONCAT44(dat_4,putchar_unlocked(0x20));
    }
    putchar_unlocked(9);
    v2 = a0 / dat_272c0;
    a0 = (dat_272c0 + a0) - a0 % dat_272c0;
  } while (a0 < a1);
  return v2;
}

// Function: sub_7460 @ 0x7460
void sub_7460(char a0)
{
  dat_271f8 += 1;
  putchar_unlocked((int)a0); // tail-call
}

// Function: sub_7480 @ 0x7480
unsigned long * sub_7480(unsigned long *a0,char a1) // early-return x2
{
  unsigned long v1;
  unsigned long *v10;
  char v11;
  unsigned long v2;
  unsigned long *v3;
  unsigned int v4;
  long v5; // rax
  unsigned long v6; // rax
  unsigned int v7;
  char *v8;
  unsigned int v9; // esi
  
  if (a1) { // branch-flip
    v8 = (char *)a0[1];
    v4 = *(unsigned int *)((long)a0 + 0xac);
    if (!*(char *)((long)a0 + 0xb9)) {
      v9 = 0xffffffff;
      if (sub_6420(0xc)) {
        v5 = 0xc0;
        goto label_750f;
      }
      goto label_7542;
    }
    v11 = *(char *)&a0[0x17];
    v9 = 0;
    v7 = 0;
    if (v11) goto label_7551;
label_74b9:
    v9 = v7;
    v4 = *(unsigned int *)&"\r"[(unsigned long)*(unsigned int *)&a0[0x15] * 4];
    v6 = (unsigned long)v4;
    v11 = v4 == 7;
    if (v4 == 5) {
label_7660:
      v6 = strlen(v8);
      for (v3 = dat_27308; v3; v3 = (unsigned long *)v3[4]) {
        v1 = *v3;
        if ((v1 <= v6) && (v2 = v3[1], !sub_d5c0(&v8[v6 - v1],v2))) {
          if (!v3[3])
            return NULL;
          return &v3[2];
        }
      }
      v5 = 0x50;
      goto label_750f;
    }
  }
  else {
    v10 = a0;
    v4 = sub_55e0(a0);
    v8 = (char *)*v10;
    v9 = (unsigned int)*(unsigned char *)((long)v10 + 0xb9);
label_7542:
    v11 = *(char *)&a0[0x17];
    v7 = v9;
    if (!v11) goto label_74b9;
label_7551:
    v7 = v4 & 0xf000;
    if (v7 == 0x8000) {
      if ((v4 & 0x800) && (sub_6420(0x10))) {
        v5 = 0x100;
        goto label_750f;
      }
      if ((v4 & 0x400) && (sub_6420(0x11))) {
        v5 = 0x110;
        goto label_750f;
      }
      if ((sub_6420(0x15)) && (*(char *)&a0[0x18])) {
        v5 = 0x150;
        goto label_750f;
      }
      if ((v4 & 0x49) && (sub_6420(0xe))) {
        v5 = 0xe0;
        goto label_750f;
      }
      if ((2 <= (unsigned long)a0[5]) && (sub_6420(0x16))) {
        v5 = 0x160;
        goto label_750f;
      }
      goto label_7660;
    }
    if (v7 == 0x4000) {
      if ((v4 & 0x202) == 0x202) {
        v5 = 0x140;
        if (sub_6420(0x14)) goto label_750f;
      }
      if (v4 & 2) {
        v5 = 0x130;
        if (sub_6420(0x13)) goto label_750f;
      }
      v5 = 0x60;
      if (v4 & 0x200)
        v5 = (-(unsigned long)(sub_6420(0x12) == '\0') & 0xffffffffffffff40) + 0x120;
      goto label_750f;
    }
    if (v7 != 0xa000) {
      v5 = 0x80;
      if ((((v7 != 0x1000) && (v5 = 0x90, v7 != 0xc000)) && (v5 = 0xa0, v7 != 0x6000)) && (v5 = 0xd0, v7 == 0x2000))
        v5 = 0xb0;
      goto label_750f;
    }
    v6 = 7;
  }
  if ((v9) || (!v11))
    v5 = v6 << 4;
  else {
    v5 = 0xd0;
    if (!dat_27390)
      v5 = (-(unsigned long)(sub_6420(0xd) == '\0') & 0xffffffffffffffa0) + 0xd0;
  }
label_750f:
  if (!*(long *)(v5 + 0x26068))
    return NULL;
  return (unsigned long *)(v5 + 0x26060);
}

// Function: sub_77d0 @ 0x77d0
void sub_77d0(void *a0,unsigned long a1)
{
  dat_271f8 += a1;
  fwrite_unlocked(a0,1,a1,stdout); // tail-call
}

// Function: sub_77f0 @ 0x77f0
void sub_77f0(char *a0)
{
  sub_77d0(a0,strlen(a0)); // tail-call
}

// Function: sub_7810 @ 0x7810
void sub_7810(void)
{
  if (!dat_27318)
    return;
  sub_77f0("  "); // tail-call
}

// Function: sub_7830 @ 0x7830
char * sub_7830(char *a0,char a1)
{
  char v1;
  unsigned char v2; // al
  char *v3; // rax
  char v4; // dl
  char *v5;
  char *v6; // r15
  
  v3 = (char *)sub_16b00(3,strlen(a0) + 1);
  v1 = *a0;
  v5 = v3;
  do {
    while( true ) {
      if (!v1) {
        *v5 = '\0';
        return v3;
      }
      a0 = &a0[1];
      if ((v1 != '/') || (!a1)) break;
      *v5 = '/';
label_787f:
      v5 = &v5[1];
      v1 = *a0;
    }
    v2 = sub_5570((int)v1);
    if (*(char *)((unsigned long)v2 + 0x26fe0)) {
      *v5 = v4;
      goto label_787f;
    }
    v6 = &v5[3];
    __sprintf_chk(v5,1,0xffffffffffffffff,"%%%02x",(unsigned long)v2);
    v1 = *a0;
    v5 = v6;
  } while( true );
}

// Function: sub_7900 @ 0x7900
void sub_7900(long a0)
{
  if (!dat_27318)
    return;
  if (8 <= (unsigned long)(*(long *)(a0 + 0x20) - (long)*(unsigned long **)(a0 + 0x18))) {
    **(unsigned long **)(a0 + 0x18) = dat_271f8;
    *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 8;
    return;
  }
  _obstack_newchunk(a0,8);
  **(unsigned long **)(a0 + 0x18) = dat_271f8;
  *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 8;
}

// Function: sub_7970 @ 0x7970
void sub_7970(unsigned long a0,unsigned long a1)
{
  unsigned long *v1;
  
  v1 = dat_270f8;
  if ((unsigned long)(dat_27100 - (long)dat_270f8) <= 0xf) {
    _obstack_newchunk(0x270e0,0x10);
    v1 = dat_270f8;
  }
  dat_270f8 = &v1[2];
  v1[1] = a0;
  *v1 = a1;
}

// Function: sub_79d0 @ 0x79d0
void sub_79d0(char a0) // return-dupe
{
  unsigned int *v1; // rbx
  unsigned long v10; // stack - 0xb8
  unsigned long v11; // stack - 0xb0
  unsigned long v12; // stack - 0xa8
  unsigned long v13; // stack - 0xa0
  unsigned long v14; // stack - 0x98
  unsigned long v15; // stack - 0x90
  unsigned long v16; // stack - 0x88
  unsigned long v17; // stack - 0x80
  unsigned long v18; // stack - 0x78
  unsigned long v19; // stack - 0x70
  int *v2; // rbx
  unsigned long v20; // stack - 0x68
  unsigned long v21; // stack - 0x60
  unsigned long v22; // stack - 0x58
  unsigned int v23; // stack - 0x50
  void *v3; // stack - 0xd8
  unsigned int v4;
  int *v5; // rbp
  int v6;
  unsigned long v7; // stack - 0xd0
  unsigned long v8; // stack - 0xc8
  unsigned long v9; // stack - 0xc0
  
  if (a0) {
    v2 = (int *)0x19644;
    v6 = 0x14;
    v5 = (int *)0x19644;
    sigemptyset((void *)0x27220);
    while( true ) {
      sigaction(v6,NULL,&v3);
      if (v3 != (void *)0x1)
        sigaddset((void *)0x27220,v6);
      if (v5 == (int *)0x19670) break;
      v6 = *v5;
      v5 = &v5[1];
    }
    v23 = 0x10000000;
    v6 = 0x14;
    v7 = dat_27220;
    v8 = dat_27228;
    v9 = dat_27230;
    v10 = dat_27238;
    v11 = dat_27240;
    v12 = dat_27248;
    v13 = dat_27250;
    v14 = dat_27258;
    v15 = dat_27260;
    v16 = dat_27268;
    v17 = dat_27270;
    v18 = dat_27278;
    v19 = dat_27280;
    v20 = dat_27288;
    v21 = dat_27290;
    v22 = dat_27298;
    while( true ) {
      if (sigismember(0x27220,v6)) {
        v3 = sub_5700;
        if (v6 == 0x14)
          v3 = sub_5720;
        sigaction(v6,&v3,NULL);
      }
      if (v2 == (int *)0x19670) break;
      v6 = *v2;
      v2 = &v2[1];
    }
    return;
  }
  v1 = (unsigned int *)0x19644;
  v4 = 0x14;
  while( true ) {
    if (sigismember(0x27220,v4))
      signal(v4,0);
    if (v1 == (unsigned int *)0x19670) break;
    v4 = *v1;
    v1 = &v1[1];
  }
}

// Function: sub_7ba0 @ 0x7ba0
void sub_7ba0(void)
{
  sub_79d0(1); // tail-call
}

// Function: sub_7bb0 @ 0x7bb0
void sub_7bb0(unsigned long *a0)
{
  if (!dat_27310) {
    dat_27310 = '\x01';
    if (0 <= (int)tcgetpgrp(1)) { // branch-flip
      sub_7ba0();
      sub_7c10();
    }
    else {
      sub_7c10();
    }
  }
  fwrite_unlocked((void *)a0[1],*a0,1,stdout); // tail-call
}

// Function: sub_7c10 @ 0x7c10
void sub_7c10(void)
{
  if (dat_26088) {
    sub_7bb0(0x26080); // tail-call
    return;
  }
  sub_7bb0(0x26060);
  sub_7bb0(0x26090);
  sub_7bb0(0x26070); // tail-call
}

// Function: sub_7c60 @ 0x7c60
void sub_7c60(void)
{
  if (!dat_27312)
    return;
  if (!sub_6420(4))
    return;
  sub_7bb0(0x26060);
  sub_7bb0(0x260a0);
  sub_7bb0(0x26070); // tail-call
}

// Function: sub_7cc0 @ 0x7cc0
void sub_7cc0(void)
{
  sub_7bb0(0x26060);
  sub_7bb0(0x26070); // tail-call
}

// Function: sub_7ce0 @ 0x7ce0
void sub_7ce0(void)
{
  char v1 [136];
  int v2;
  
  while ((dat_27218 || (dat_27214))) {
    if (dat_27310)
      sub_7cc0();
    fflush_unlocked(stdout);
    sigprocmask(0,(void *)0x27220,v1);
    v2 = dat_27218;
    if (dat_27214) { // branch-flip
      dat_27214 -= 1;
      v2 = 0x13;
    }
    else {
      signal(dat_27218,0);
    }
    raise(v2);
    sigprocmask(2,v1,NULL);
  }
}

// Function: sub_7dd0 @ 0x7dd0
void sub_7dd0(void)
{
  sub_79d0(0); // tail-call
}

// Function: sub_7de0 @ 0x7de0
void sub_7de0(void)
{
  sub_10b80(); // tail-call
}

// Function: sub_7df0 @ 0x7df0
unsigned long sub_7df0(unsigned long a0,unsigned long a1)
{
  unsigned long *v1; // rax
  unsigned long *v2; // rax
  
  v1 = (unsigned long *)sub_169e0(0x10);
  *v1 = a1;
  v1[1] = a0;
  v2 = (unsigned long *)sub_f9a0(dat_273c8,v1);
  if (v2) {
    if (v1 != v2)
      free(v1);
    return CONCAT71((undefined7)((unsigned long)a1 >> 8),v1 != v2) & 0xffffffff;
  }
  sub_16da0(); // no-return
}

// Function: sub_7e40 @ 0x7e40
void sub_7e40(char *a0)
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
  char *v3;
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
    if (!strcmp(a0,v6)) break;
    v6 = *(char **)((long)v4 + 0x10);
    v4 = (char *)((long)v4 + 0x10);
  } while (v6);
  v6 = *(char **)((long)v4 + 8);
  if (!*(char **)((long)v4 + 8))
    v6 = a0;
  __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  v3 = setlocale(5,NULL);
  if ((v3) && (v2 = strncmp(v3,"en_",3), v1 = stdout, v2))
    fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
  v3 = "test";
  if (strcmp(a0,"["))
    v3 = a0;
  __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v3);
  v3 = "";
  if (v6 == a0)
    v3 = " invocation";
  __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v6,v3); // tail-call
}

// Function: sub_8060 @ 0x8060
void sub_8060(char *a0,long a1)
{
  unsigned long v1;
  unsigned long v2; // rax
  unsigned long v3;
  unsigned long *v4;
  unsigned long *v5;
  
  v5 = *(unsigned long **)(a1 + 0x18);
  v4 = *(unsigned long **)(a1 + 0x10);
  if (8 <= (unsigned long)((long)v5 - (long)v4)) {
    if (v5 == v4)
      *(unsigned char *)(a1 + 0x50) = *(unsigned char *)(a1 + 0x50) | 2;
    v2 = (long)v5 + *(unsigned long *)(a1 + 0x30) & ~*(unsigned long *)(a1 + 0x30);
    v3 = *(unsigned long *)(a1 + 0x20);
    if (v2 - *(long *)(a1 + 8) <= *(unsigned long *)(a1 + 0x20) - *(long *)(a1 + 8))
      v3 = v2;
    v5 = (unsigned long *)(((long)v5 - (long)v4 & 0xfffffffffffffff8U) + (long)v4);
    *(unsigned long *)(a1 + 0x18) = v3;
    *(unsigned long *)(a1 + 0x10) = v3;
    fputs_unlocked(a0,stdout);
    do {
      v1 = *v4;
      v4 = &v4[1];
      __printf_chk(1," %ld",v1);
    } while (v5 != v4);
    putchar_unlocked(10); // tail-call
    return;
  }
}

// Function: sub_8100 @ 0x8100
void sub_8100(long a0,unsigned long a1,int a2)
{
  bool v1; // cf
  
  if (a0) {
    a2 -= sub_113a0(a0,0);
    if (a2 <= -1)
      a2 = 0;
    sub_77f0(a0);
    do {
      sub_7460(0x20);
      v1 = a2 != 0;
      a2 -= 1;
    } while (v1);
    return;
  }
  dat_271f8 += __printf_chk(1,"%*lu ",a2,a1);
}

// Function: sub_8180 @ 0x8180
void sub_8180(unsigned long a0,unsigned int a1,char a2)
{
  char *v1; // rax
  
  v1 = "?";
  if ((a2) && (v1 = NULL, !dat_2732d))
    v1 = (char *)sub_109e0(a0);
  sub_8100(v1,a0 & 0xffffffff,a1); // tail-call
}

// Function: sub_81d0 @ 0x81d0
void sub_81d0(unsigned long a0,unsigned int a1,char a2)
{
  char *v1; // rax
  
  v1 = "?";
  if ((a2) && (v1 = NULL, !dat_2732d))
    v1 = (char *)sub_10840(a0);
  sub_8100(v1,a0 & 0xffffffff,a1); // tail-call
}

// Function: sub_8220 @ 0x8220
unsigned long sub_8220(void)
{
  *__errno_location() = 0x5f;
  return 0;
}

// Function: sub_8240 @ 0x8240
void sub_8240(unsigned long a0,unsigned char a1,long a2,unsigned long a3,unsigned int a4)
{
  if (dat_263c8) {
    sub_13030(a0,0x3e9,((long)*(int *)(a2 + 0x10) + (unsigned long)a1 * 0xc) * 0x80 + 0x263e0,a2,a3,a4); // tail-call
    return;
  }
  sub_13030(a0,0x3e9,*(unsigned long *)((unsigned long)a1 * 8 + 0x26040),a2,a3,a4); // tail-call
}

// Function: sub_82b0 @ 0x82b0
unsigned long sub_82b0(void) // early-return
{
  long v1; // rax
  unsigned long v2; // rax
  char v3 [64];
  unsigned long v4; // stack - 0x460
  char v5 [1016];
  
  if (0 <= (int)dat_26014)
    return (unsigned long)dat_26014;
  v4 = 0;
  v1 = sub_160a0(dat_272a8,&v4,v3);
  if (v1) {
    v1 = sub_8240(v5,0,v3,dat_272a8,0);
    if (v1) {
      v2 = sub_11180(v5,v1,0);
      dat_26014 = (unsigned int)v2;
      goto label_834e;
    }
  }
  v2 = (unsigned long)dat_26014;
label_834e:
  if (0 <= (int)v2)
    return v2;
  dat_26014 = 0;
  return 0;
}

// Function: sub_8390 @ 0x8390
unsigned long sub_8390(unsigned long *a0,char *a1,unsigned long a2,int a3,unsigned long *a4,unsigned char *a5)
{
  char v1;
  char *v10; // rdx
  char *v11;
  long v12;
  unsigned long v13;
  unsigned long v14; // stack - 0x48
  int v15; // stack - 0x4c
  char *v16;
  char *v17;
  unsigned long v18;
  char *v19; // stack - 0x80
  bool v2;
  bool v20; // stack - 0x69
  unsigned char v3;
  int v4; // eax
  unsigned long v5; // rax
  long *v6; // rax
  long v7;
  long v8; // rdx
  char *v9;
  
  v19 = (char *)*a0;
  if ((unsigned int)sub_151e0(a2) <= 2 & dat_272d8) { // branch-flip
    if (a3) { // branch-flip
      v2 = 1;
label_83f2:
      v5 = sub_152c0(v19,0x2000,a1,0xffffffffffffffff,a2);
      if (0x2000 <= v5) {
        v19 = (char *)sub_169e0(v5 + 1);
        sub_152c0(v19,v5 + 1,a1,0xffffffffffffffff,a2);
      }
      v20 = 1;
      if (*a1 == *v19)
        v20 = v5 != strlen(a1);
      v18 = v5;
      if (!v2) goto label_8460;
    }
    else {
      v5 = strlen(a1);
      if (0x2000 <= v5)
        v19 = (char *)sub_169e0(v5 + 1);
      memcpy(v19,a1,v5 + 1);
      v20 = 0;
    }
    v9 = &v19[v5];
    if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
      if (v9 <= v19) { // branch-flip
        v18 = 0;
        v5 = 0;
      }
      else {
        v5 = 0;
        v16 = v19;
        v11 = v19;
        do {
          v1 = *v16;
          if ('`' <= v1) { // branch-flip
            if (0x1e <= (unsigned char)(v1 + 0x9fU)) goto label_8645;
label_8704:
            *v11 = v1;
            v16 = &v16[1];
            v5 += 1;
            v11 = &v11[1];
          }
          else {
            if ('@' < v1) goto label_8704;
            if (v1 <= '#') {
              if ('\x1f' >= v1) goto label_8645;
              goto label_8704;
            }
            if ((unsigned char)(v1 - 0x25U) <= 0x1a) goto label_8704;
label_8645:
            v14 = 0;
            v17 = v11;
            do {
              v7 = sub_10c80(&v15,v16,(long)v9 - (long)v16,&v14);
              v11 = &v17[1];
              if (v7 == -1) {
                v16 = &v16[1];
label_8768:
                v5 += 1;
                *v17 = '?';
                break;
              }
              if (v7 == -2) {
                v16 = v9;
                goto label_8768;
              }
              v12 = 1;
              if (v7)
                v12 = v7;
              v4 = wcwidth(v15);
              if (0 <= v4) { // branch-flip
                v7 = 0;
                do {
                  v17[v7] = v16[v7];
                  v7 += 1;
                } while (v12 != v7);
                v11 = &v17[v12];
                v5 += (long)v4;
              }
              else {
                *v17 = '?';
                v5 += 1;
              }
              v16 = &v16[v12];
              v17 = v11;
            } while (!mbsinit(&v14));
          }
        } while (v16 < v9);
        v18 = (long)v11 - (long)v19;
      }
    }
    else {
      if (v19 < v9) {
        v6 = __ctype_b_loc();
        v11 = v19;
        do {
          v3 = sub_5570((int)*v11);
          if (!(*(unsigned char *)(*v6 + 1 + (unsigned long)v3 * 2) & 0x40))
            *v10 = 0x3f;
          v11 = &v10[1];
        } while (v11 != v9);
      }
      v18 = v5;
    }
    v3 = 0;
    v13 = v5;
    if (dat_273a8) goto label_85a1;
label_85af:
    *a5 = v3;
    if (!a4) goto label_85c6;
  }
  else {
    v2 = 0;
    if (a3) goto label_83f2;
    v18 = strlen(a1);
    v20 = 0;
    v19 = a1;
label_8460:
    if (!a4) {
      if (!dat_273a8) {
        *a5 = 0;
        goto label_85c6;
      }
label_85a1:
      v3 = (v20 ^ 1U) & dat_273a9;
      goto label_85af;
    }
    if (2 <= __ctype_get_mb_cur_max()) // branch-flip
      v13 = (unsigned long)(int)sub_11180(v19,v18,0);
    else if (&v19[v18] <= v19) // branch-flip
      v13 = 0;
    else {
      v13 = 0;
      v7 = *(long *)__ctype_b_loc();
      v9 = v19;
      do {
        v1 = *v9;
        v13 = (v13 + 1) - (unsigned long)((*(unsigned short *)(v7 + (unsigned long)(unsigned char)sub_5570((int)v1) * 2) & 0x4000) == 0);
        v9 = (char *)(v8 + 1);
      } while (&v19[v18] != v9);
    }
    if (dat_273a8) goto label_85a1;
    *a5 = 0;
  }
  *a4 = v13;
label_85c6:
  *a0 = v19;
  return v18;
}

// Function: sub_87f0 @ 0x87f0
long sub_87f0(char *a0,unsigned long a1,unsigned int a2)
{
  char v1 [8200];
  char *v2; // stack - 0x2038
  unsigned char v3; // stack - 0x2039
  long v4; // stack - 0x2030
  
  v2 = v1;
  sub_8390(&v2,a0,a1,a2,&v4,&v3);
  if ((v2 != v1) && (v2 != a0))
    free(v2);
  return (unsigned long)v3 + v4;
}

// Function: sub_8890 @ 0x8890
void sub_8890(unsigned long *a0)
{
  if (a0[0x19])
    return;
  sub_87f0(*a0,dat_272d0,*(unsigned int *)((long)a0 + 0xc4)); // tail-call
}

// Function: sub_88c0 @ 0x88c0
void sub_88c0(unsigned long *a0,unsigned long *a1,void *a2)
{
  int v1; // eax
  
  v1 = sub_8890(a0);
  if (v1 == sub_8890(a1)) {
    (*a2)(*a0,*a1); // jump-as-call
    return;
  }
}

// Function: sub_8920 @ 0x8920
void sub_8920(unsigned long a0,unsigned long a1)
{
  sub_88c0(a1,a0,dat_25fc0); // tail-call
}

// Function: sub_8940 @ 0x8940
void sub_8940(unsigned long a0,unsigned long a1)
{
  sub_88c0(a0,a1,dat_25fc0); // tail-call
}

// Function: sub_8950 @ 0x8950
void sub_8950(unsigned long a0,unsigned long a1)
{
  sub_88c0(a1,a0,sub_72a0); // tail-call
}

// Function: sub_8970 @ 0x8970
void sub_8970(unsigned long a0,unsigned long a1)
{
  sub_88c0(a0,a1,sub_72a0); // tail-call
}

// Function: sub_8980 @ 0x8980
void sub_8980(void)
{
  long v1;
  unsigned long v2; // rbx
  
  if ((dat_27330 != 2) && ((!dat_272b0 || (2 <= (unsigned int)(dat_27338 - 2U)))))
    return;
  v2 = 0;
  if (dat_273b0) {
    do {
      v1 = *(long *)(dat_273a0 + v2 * 8);
      v2 += 1;
      *(unsigned long *)(v1 + 200) = sub_8890(v1);
    } while (v2 < dat_273b0);
  }
}

// Function: sub_89f0 @ 0x89f0
void sub_89f0(void) // return-dupe
{
  int v1; // eax
  int v2;
  unsigned long v3;
  
  v3 = dat_273b0;
  if (dat_27398 < (dat_273b0 >> 1) + dat_273b0) {
    free(dat_273a0);
    dat_273a0 = (void *)sub_16b00(v3,0x18);
    dat_27398 = dat_273b0 * 3;
  }
  sub_6000();
  sub_8980();
  if (dat_27330 == 6)
    return;
  v1 = _setjmp(0x26300);
  v3 = (unsigned long)dat_27330;
  v2 = 0;
  if (v1) {
    if (dat_27330 == 4)
      __assert_fail("sort_type != sort_version","src/ls.c",0x1008,"sort_files"); // no-return
    sub_6000();
    v2 = 1;
  }
  if ((int)v3 == 5)
    v3 = (unsigned long)(dat_27334 + 5);
  sub_11610(dat_273a0,dat_273b0,*(unsigned long *)(((unsigned long)dat_272f4 + ((unsigned long)dat_2732f + ((long)v2 + v3 * 2) * 2) * 2) * 8 + 0x24f60));
}

// Function: sub_8af0 @ 0x8af0
long sub_8af0(long a0)
{
  char v1;
  unsigned int v2;
  unsigned int v3;
  unsigned long v4; // rax
  char *v5; // rax
  long v6;
  char v7 [664];
  
  if (dat_272fc) { // branch-flip
    if (dat_27338 == 4) {
      v5 = (char *)sub_10c20(*(unsigned long *)(a0 + 0x20),v7);
      v6 = strlen(v5) + 1;
      goto label_8bba;
    }
    v6 = (long)dat_27360 + 1;
    if (dat_2732c) {
label_8b41:
      v4 = (unsigned long)dat_2735c;
      goto label_8b4c;
    }
    if (!dat_27365) goto label_8b5c;
label_8bf9:
    v4 = (unsigned long)dat_27354;
  }
  else {
    v6 = 0;
label_8bba:
    if (dat_2732c) {
      if (dat_27338 != 4) goto label_8b41;
      v5 = "?";
      if (*(char *)(a0 + 0xb8))
        v5 = (char *)sub_ffa0(*(unsigned long *)(a0 + 0x58),v7,dat_27328,0x200,dat_27320);
      v4 = strlen(v5);
label_8b4c:
      v6 += v4 + 1;
    }
    if (!dat_27365) goto label_8b5c;
    if (dat_27338 != 4) goto label_8bf9;
    v4 = strlen(*(char **)(a0 + 0xb0));
  }
  v6 += v4 + 1;
label_8b5c:
  v6 += sub_8890(a0);
  if (dat_27314) {
    v2 = *(unsigned int *)(a0 + 0xa8);
    v3 = *(unsigned int *)(a0 + 0x30);
    v1 = *(char *)(a0 + 0xb8);
    v6 = (v6 + 1) - (unsigned long)(sub_6040(v1,v3,v2) == '\0');
  }
  return v6;
}

// Function: sub_8c90 @ 0x8c90
unsigned long sub_8c90(char a0)
{
  unsigned long *v1;
  long v2; // rax
  unsigned long v3;
  unsigned long v4; // rcx
  unsigned long v5;
  unsigned long v6;
  char *v7;
  unsigned long v8;
  
  v6 = dat_27200;
  if (dat_273b0 <= dat_27200)
    v6 = dat_273b0;
  v8 = dat_273b0;
  if (dat_27200)
    v8 = v6;
  v6 = 0;
  sub_61d0(v8);
  if (dat_273b0) {
    do {
      v2 = sub_8af0(*(unsigned long *)(dat_273a0 + v6 * 8));
      if (v8) {
        v3 = 0;
        v7 = dat_27208;
        do {
          v4 = v3 + 1;
          if (*v7) {
            if (a0) // branch-flip
              v5 = v6 / (((dat_273b0 - 1) + v4) / v4);
            else {
              v5 = v6 % v4;
            }
            v1 = (unsigned long *)(*(long *)&v7[0x10] + v5 * 8);
            v3 = v2 + (unsigned long)(v5 != v3) * 2;
            v5 = *v1;
            if (v5 < v3) {
              *(unsigned long *)&v7[8] = *(long *)&v7[8] + (v3 - v5);
              *v1 = v3;
              *v7 = *(unsigned long *)&v7[8] < dat_272b0;
            }
          }
          v7 = &v7[0x18];
          v3 = v4;
        } while (v8 != v4);
      }
      v6 += 1;
    } while (v6 < dat_273b0);
  }
  if (2 <= v8) {
    v7 = &dat_27208[v8 * 0x18 + -0x18];
    v6 = v8;
    do {
      if (*v7)
        return v6;
      v6 -= 1;
      v7 = &v7[-0x18];
      v8 = 1;
    } while (v6 != 1);
  }
  return v8;
}

// Function: sub_8dd0 @ 0x8dd0
void sub_8dd0(void)
{
  dat_261f8 = 2;
}

// Function: sub_8de0 @ 0x8de0
unsigned int sub_8de0(long a0)
{
  int v1;
  unsigned int v2; // eax
  
  if (dat_262f8 != a0) {
    v2 = sub_8220();
    if (!(char)v2) {
      v1 = *__errno_location();
      if (sub_59f0(v1))
        dat_262f8 = a0;
    }
    return v2;
  }
  *__errno_location() = 0x5f;
  return 0;
}

// Function: sub_8e50 @ 0x8e50
void sub_8e50(long a0)
{
  if (a0) {
    if (sub_6420(4))
      sub_7cc0();
    sub_7bb0(0x26060);
    sub_7bb0(a0);
    sub_7bb0(0x26070); // tail-call
    return;
  }
}

// Function: sub_8eb0 @ 0x8eb0
long sub_8eb0(char *a0,unsigned long a1,unsigned int a2,long a3,char a4,long a5,long a6)
{
  long v1;
  char v10;
  char v11;
  long v12; // stack - 0x2078
  unsigned long v2; // rax
  void *v3; // rax
  char *v4; // rax
  char *v5; // rcx
  char v6 [8200];
  char *v7; // stack - 0x2050
  unsigned char v8; // stack - 0x2051
  unsigned long v9;
  
  v1 = a6;
  v7 = v6;
  v2 = sub_8390(&v7,a0,a1,a2,0,&v8);
  if ((v8) && (a4))
    sub_7460(0x20);
  if (a3)
    sub_8e50(a3);
  v11 = dat_273a9;
  v9 = v2;
  if (v1) { // branch-flip
    v10 = '\0';
    if ((dat_273a8) && (v10 = dat_273a9, dat_273a9)) {
      if (v8) { // branch-flip
        v12 = 0;
        v11 = '\0';
      }
      else {
        putchar_unlocked((int)*v7);
        v12 = 1;
        v9 = v2 - 2;
      }
    }
    else {
      v12 = 0;
      v11 = v10;
    }
    v3 = (void *)sub_7830(dat_27388,0);
    v4 = (char *)sub_7830(v1,1);
    v5 = "";
    if (*v4 != '/')
      v5 = "/";
    __printf_chk(1,"\x1b]8;;file://%s%s%s\a",v3,v5);
    free(v3);
    free(v4);
  }
  else {
    v12 = 0;
    v11 = '\0';
  }
  if (a5) { // branch-flip
    sub_7900(a5);
    fwrite_unlocked(&v7[v12],1,v9,stdout);
    dat_271f8 += v2;
    sub_7900(a5);
  }
  else {
    fwrite_unlocked(&v7[v12],1,v9,stdout);
    dat_271f8 += v2;
  }
  if ((v1) && (fputs_unlocked("\x1b]8;;\a",stdout), v11))
    putchar_unlocked((int)v7[v2 - 1]);
  if ((v7 != v6) && (v7 != a0))
    free(v7);
  return v8 + v2;
}

// Function: sub_9160 @ 0x9160
long sub_9160(unsigned long *a0,unsigned char a1,unsigned long a2,unsigned long a3)
{
  unsigned long v1;
  long v2; // rax
  unsigned long v3;
  
  v3 = a0[1];
  if (!a1)
    v3 = *a0;
  if ((dat_27312) && ((v2 = sub_7480(a0,a1), v2 || (sub_6420(4))))) {
    v2 = sub_8eb0(v3,dat_272d0,*(unsigned int *)((long)a0 + 0xc4),v2,a1 ^ 1,a2,a0[2]);
    sub_7ce0();
    sub_7c10();
    if (dat_272b0) {
      v1 = v2 + -1 + a3;
      if (a3 / dat_272b0 != v1 / dat_272b0)
        sub_7bb0(0x261d0,a3 / dat_272b0,v1 % dat_272b0);
    }
  }
  else {
    v2 = sub_8eb0(v3,dat_272d0,*(unsigned int *)((long)a0 + 0xc4),0,a1 ^ 1,a2,a0[2]);
    sub_7ce0();
  }
  return v2;
}

// Function: sub_9280 @ 0x9280
void sub_9280(char a0)
{
  char v1; // al
  
  v1 = sub_6040(a0);
  if (!v1)
    return;
  sub_7460((int)v1); // tail-call
}

// Function: sub_92b0 @ 0x92b0
void sub_92b0(long a0) // return-dupe x4
{
  char v1;
  unsigned long v10; // rdx
  unsigned long v11; // rdx
  unsigned long v12; // rdx
  unsigned long v13; // rdx
  unsigned long v14; // rdx
  char *v15; // rbx
  char *v16;
  char *v17;
  long v18; // rbx
  char v19; // stack - 0x1144
  int v2;
  char v20 [3656];
  char v21 [656];
  char v22 [32];
  char v23 [68];
  unsigned long v24; // stack - 0x1198
  int v25; // ebp
  unsigned long v26;
  unsigned long v27;
  char *v28; // rax
  bool v29;
  unsigned int v3; // eax
  char v30 [16];
  unsigned long v31; // stack - 0x1190
  unsigned long v32; // stack - 0x1143
  unsigned short v33; // stack - 0x113b
  char v34; // stack - 0x1139
  int v4;
  char *v5;
  long v6; // rax
  unsigned long v7;
  unsigned long v8;
  unsigned long v9; // rdx
  
  if (*(char *)(a0 + 0xb8)) // branch-flip
    sub_e2f0(a0 + 0x18,&v19);
  else {
    v34 = 0;
    v33 = 0x3f3f;
    v19 = "?pcdb-lswd"[*(unsigned int *)(a0 + 0xa8)];
    v32 = 0x3f3f3f3f3f3f3f3f;
  }
  if (dat_27364) { // branch-flip
    if (*(int *)(a0 + 0xbc) == 1) {
      v33 = CONCAT11(0x2e,(char)v33);
      goto label_9310;
    }
    if (*(int *)(a0 + 0xbc) != 2) goto label_9310;
    v33 = CONCAT11(0x2b,(char)v33);
    if (dat_27334 == 2) goto label_9600;
label_931f:
    if (3 <= dat_27334) { // branch-flip
      if (dat_27334 != 3)
        abort(); // no-return
      v30 = sub_5610(a0 + 0x18);
      v31 = SUB168(v30,8);
      v24 = SUB168(v30,0);
      v29 = (v24 & v31) != 0xffffffffffffffff;
    }
    else if (dat_27334) { // branch-flip
      v29 = 1;
      v30 = sub_15ca0(a0 + 0x18);
      v31 = SUB168(v30,8);
      v24 = SUB168(v30,0);
    }
    else {
      v29 = 1;
      v30 = sub_15cb0(a0 + 0x18);
      v31 = SUB168(v30,8);
      v24 = SUB168(v30,0);
    }
  }
  else {
    v33 &= 0xff;
label_9310:
    if (dat_27334 != 2) goto label_931f;
label_9600:
    v29 = 1;
    v30 = sub_15c90(a0 + 0x18);
    v31 = SUB168(v30,8);
    v24 = SUB168(v30,0);
  }
  v16 = v20;
  if (dat_272fc) {
    v8 = sub_6f10(v21,0x15,a0);
    v26 = (unsigned long)dat_27360;
    v16 = &v20[__sprintf_chk(v20,1,0xe3b,"%*s ",v26,v8)];
  }
  if (dat_2732c) {
    v28 = "?";
    if (*(char *)(a0 + 0xb8))
      v28 = (char *)sub_ffa0(*(unsigned long *)(a0 + 0x58),v21,dat_27328,0x200,dat_27320);
    v2 = dat_2735c;
    v2 -= sub_113a0(v28,0);
    if (v2 <= 0) goto label_9818;
    v5 = &v16[v2];
    do {
      v15 = &v16[1];
      *v16 = ' ';
      v16 = v15;
      v17 = v28;
    } while (v5 != v15);
    while( true ) {
      v28 = &v17[1];
      v16 = &v5[1];
      v1 = *v17;
      *v5 = v1;
      if (!v1) break;
label_9818:
      v5 = v16;
      v17 = v28;
    }
    *v5 = ' ';
  }
  v28 = "?";
  if (*(char *)(a0 + 0xb8))
    v28 = (char *)sub_10c20(*(unsigned long *)(a0 + 0x28),v21);
  v26 = (unsigned long)dat_27358;
  v16 = &v16[__sprintf_chk(v16,1,0xffffffffffffffff,"%s %*s ",&v19,v26,v28)];
  sub_7810();
  if ((((dat_26029) || (dat_26028)) || (dat_2732e)) || (v8 = v9, dat_27365)) {
    sub_77d0(v20,(long)v16 - (long)v20);
    v8 = v10;
    if (dat_26029) {
      sub_81d0(*(unsigned int *)(a0 + 0x34),dat_27350,*(char *)(a0 + 0xb8));
      v8 = v12;
    }
    if (dat_26028) {
      sub_8180(*(unsigned int *)(a0 + 0x38),dat_2734c,*(char *)(a0 + 0xb8));
      v8 = v13;
    }
    if (dat_2732e) {
      sub_81d0(*(unsigned int *)(a0 + 0x34),dat_27348,*(char *)(a0 + 0xb8));
      v8 = v14;
    }
    v16 = v20;
    if (dat_27365) {
      sub_8100(*(unsigned long *)(a0 + 0xb0),0,dat_27354);
      v8 = v11;
    }
  }
  if (*(char *)(a0 + 0xb8)) { // branch-flip
    if ((*(unsigned int *)(a0 + 0x30) & 0xb000) != 0x2000) {
      v7 = *(unsigned long *)(a0 + 0x48);
      v28 = (char *)sub_ffa0(sub_59e0(v7,v21,v8,1));
      goto label_9493;
    }
    v8 = *(unsigned long *)(a0 + 0x40);
    v25 = dat_2733c - (dat_27344 + 2 + dat_27340);
    v7 = sub_10c20(gnu_dev_minor(v8),v21);
    v2 = dat_27340;
    v8 = *(unsigned long *)(a0 + 0x40);
    v8 = sub_10c20(gnu_dev_major(v8),v22);
    v4 = 0;
    if (0 <= v25)
      v4 = v25;
    v3 = v4 + dat_27344;
    v5 = &v16[__sprintf_chk(v16,1,0xffffffffffffffff,"%*s, %*s ",(unsigned long)v3,v8,v2,v7)];
  }
  else {
    v28 = "?";
label_9493:
    v2 = dat_2733c;
    v2 -= sub_113a0(v28,0);
    if (1 <= v2) { // branch-flip
      v17 = v16;
      do {
        v15 = &v17[1];
        *v17 = ' ';
        v17 = v15;
        v5 = &v16[v2];
      } while (&v16[v2] != v15);
    }
    else {
      v5 = v16;
    }
    do {
      v16 = v5;
      v1 = *v28;
      v28 = &v28[1];
      v5 = &v16[1];
      *v16 = v1;
    } while (v1);
    *v16 = ' ';
  }
  *v5 = '\x01';
  if ((!*(char *)(a0 + 0xb8)) || (!v29)) {
label_94f4:
    v16 = "?";
label_94fb:
    v3 = sub_82b0();
    v5 = &v5[__sprintf_chk(v5,1,0xffffffffffffffff,"%*s ",(unsigned long)v3,v16)];
  }
  else {
    v6 = sub_160a0(dat_272a8,&v24,v23);
    v27 = v31;
    v26 = v24;
    if (v6) { // branch-flip
      v2 = sub_162a0(dat_27370,dat_27378,v24,v31);
      if (v2 < 0) {
        sub_e960(0x27370);
        v26 = v24;
        v27 = v31;
      }
      v2 = sub_162a0(dat_27370 + -0xf0c2ac,dat_27378,v26,v27);
      v3 = 0;
      if (v2 <= -1) {
        v3 = sub_162a0(v24,v31,dat_27370,dat_27378);
        v3 >>= 0x1f;
      }
      v6 = sub_8240(v5,v3,v23,dat_272a8,v31 & 0xffffffff);
      if (!v6) goto label_9a20;
      v5 = &v5[v6];
    }
    else {
label_9a20:
      if (*v5) {
        if (!*(char *)(a0 + 0xb8)) goto label_94f4;
        v16 = (char *)sub_7de0(v24,v21);
        goto label_94fb;
      }
    }
    *v5 = ' ';
    v5 = &v5[1];
  }
  v18 = (long)v5 - (long)v20;
  sub_77d0(v20,v18);
  v6 = sub_9160(a0,0,0x271a0,v18);
  if (*(int *)(a0 + 0xa8) != 6) {
    if (!dat_27314)
      return;
    sub_9280(*(char *)(a0 + 0xb8),*(unsigned int *)(a0 + 0x30));
    return;
  }
  if (!*(long *)(a0 + 8))
    return;
  sub_77f0(" -> ");
  sub_9160(a0,1,0,v18 + 4 + v6);
  if (!dat_27314)
    return;
  sub_9280(1,*(unsigned int *)(a0 + 0xac),0);
}

// Function: sub_9a80 @ 0x9a80
void sub_9a80(long a0,unsigned long a1) // return-dupe
{
  unsigned long v1; // rax
  char *v2;
  unsigned long v3;
  char v4 [664];
  
  sub_7c60();
  if (dat_272fc) {
    v1 = sub_6f10(v4,0x28c,a0);
    v3 = 0;
    if (dat_27338 != 4)
      v3 = (unsigned long)dat_27360;
    __printf_chk(1,"%*s ",v3,v1);
  }
  if (dat_2732c) {
    v2 = "?";
    if (*(char *)(a0 + 0xb8))
      v2 = (char *)sub_ffa0(*(unsigned long *)(a0 + 0x58),v4,dat_27328,0x200,dat_27320);
    v3 = 0;
    if (dat_27338 != 4)
      v3 = (unsigned long)dat_2735c;
    __printf_chk(1,"%*s ",v3,v2);
  }
  if (dat_27365) {
    v3 = 0;
    if (dat_27338 != 4)
      v3 = (unsigned long)dat_27354;
    __printf_chk(1,"%*s ",v3,*(unsigned long *)(a0 + 0xb0));
  }
  sub_9160(a0,0,0,a1);
  if (!dat_27314)
    return;
  sub_9280(*(char *)(a0 + 0xb8),*(unsigned int *)(a0 + 0x30),*(unsigned int *)(a0 + 0xa8));
}

// Function: sub_9c00 @ 0x9c00
void sub_9c00(char a0)
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  unsigned long v4; // rbx
  unsigned long v5;
  int v6; // r15d
  
  if (!dat_273b0) {
    putchar_unlocked((int)dat_26019); // tail-call
    return;
  }
  v4 = 0;
  v5 = 0;
  do {
    v2 = *(unsigned long *)(dat_273a0 + v4 * 8);
    if (dat_272b0) { // branch-flip
      v3 = sub_8af0(v2);
      if (v4) { // branch-flip
        v1 = v3 + v5 + 2;
        if ((!dat_272b0) || ((v1 < dat_272b0 && (v5 <= -v3 - 3)))) {
          v3 = v1;
          goto label_9cb0;
        }
        v6 = (int)dat_26019;
        v5 = 0;
label_9cb6:
        putchar_unlocked((int)a0);
        putchar_unlocked(v6);
      }
      else {
        v3 = v5 + v3;
      }
    }
    else {
      v3 = v5;
      if (v4) {
        v3 = v5 + 2;
label_9cb0:
        v5 += 2;
        v6 = 0x20;
        goto label_9cb6;
      }
    }
    v4 += 1;
    sub_9a80(v2,v5);
    v5 = v3;
    if (dat_273b0 <= v4) {
      putchar_unlocked((int)dat_26019); // tail-call
      return;
    }
  } while( true );
}

// Function: sub_9d30 @ 0x9d30
void sub_9d30(void)
{
  long v1;
  unsigned long v2;
  unsigned long v3; // rax
  long v4; // rax
  unsigned long v5; // rdx
  unsigned long v6; // rbx
  long v7;
  long v8; // rbp
  long v9;
  
  v3 = sub_8c90(0);
  v1 = dat_27208 + -0x18 + v3 * 0x18;
  v2 = *dat_273a0;
  v4 = sub_8af0(v2);
  v7 = **(long **)(v1 + 0x10);
  sub_9a80(v2,0);
  if (2 <= dat_273b0) {
    v6 = 1;
    v9 = 0;
    do {
      v5 = v6 % v3;
      if (v5) { // branch-flip
        v8 = v7 + v9;
        sub_73d0(v9 + v4,v8);
      }
      else {
        v8 = 0;
        putchar_unlocked((int)dat_26019);
      }
      v2 = dat_273a0[v6];
      v6 += 1;
      sub_9a80(v2,v8);
      v4 = sub_8af0(v2);
      v7 = *(long *)(*(long *)(v1 + 0x10) + v5 * 8);
      v9 = v8;
    } while (v6 < dat_273b0);
  }
  putchar_unlocked((int)dat_26019); // tail-call
}

// Function: sub_9e30 @ 0x9e30
void sub_9e30(void) // return-dupe
{
  long v1;
  unsigned long v2;
  unsigned long v3; // rax
  long v4; // rax
  long v5;
  unsigned long v6;
  long v7; // r13
  long v8;
  unsigned long v9; // stack - 0x40
  
  v3 = sub_8c90(1);
  v9 = 0;
  v1 = dat_27208 + v3 * 0x18;
  v3 = (dat_273b0 / v3 + 1) - (unsigned long)(dat_273b0 % v3 == 0);
  if (!v3)
    return;
  do {
    v7 = 0;
    v6 = v9;
    v8 = 0;
    while( true ) {
      v2 = *(unsigned long *)(dat_273a0 + v6 * 8);
      v4 = sub_8af0(v2);
      v5 = *(long *)(*(long *)(v1 + -8) + v7);
      v7 += 8;
      sub_9a80(v2,v8);
      v6 += v3;
      if (dat_273b0 <= v6) break;
      v5 += v8;
      sub_73d0(v8 + v4,v5);
      v8 = v5;
    }
    putchar_unlocked((int)dat_26019);
    v9 += 1;
  } while (v3 != v9);
}

// Function: sub_9f20 @ 0x9f20
void sub_9f20(void) // return-dupe x2
{
  long v1;
  unsigned long v2; // rbx
  
  switch(dat_27338) {
    case 0:
      v2 = 0;
      if (!dat_273b0)
        return;
      do {
        sub_7c60();
        v1 = v2 * 8;
        v2 += 1;
        sub_92b0(*(unsigned long *)(dat_273a0 + v1));
        sub_7460((int)dat_26019);
      } while (v2 < dat_273b0);
      return;
    case 1:
      v2 = 0;
      if (!dat_273b0)
        return;
      do {
        v1 = v2 * 8;
        v2 += 1;
        sub_9a80(*(unsigned long *)(dat_273a0 + v1),0);
        putchar_unlocked((int)dat_26019);
      } while (v2 < dat_273b0);
      return;
    case 2:
      if (dat_272b0) {
        sub_9e30(); // tail-call
        return;
      }
      break;
    case 3:
      if (dat_272b0) {
        sub_9d30(); // tail-call
        return;
      }
      break;
    case 4:
      sub_9c00(0x2c); // tail-call
      return;
    default:
      return;
    
  }
  sub_9c00(0x20); // tail-call
}

// Function: sub_a010 @ 0xa010
void sub_a010(char *a0,char *a1)
{
  if (!sub_e940(a0,a1)) {
    strcmp(a0,a1); // tail-call
    return;
  }
}

// Function: sub_a050 @ 0xa050
void sub_a050(unsigned long *a0,unsigned long *a1)
{
  sub_a010(*a0,*a1); // tail-call
}

// Function: sub_a060 @ 0xa060
void sub_a060(unsigned long *a0,unsigned long *a1)
{
  sub_a010(*a1,*a0); // tail-call
}

// Function: sub_a070 @ 0xa070
void sub_a070(unsigned long a0,unsigned long a1,void *a2)
{
  (*a2)(); // jump-as-call
}

// Function: sub_a080 @ 0xa080
void sub_a080(unsigned long *a0,unsigned long *a1)
{
  sub_a070(*a1,*a0,dat_25fc0); // tail-call
}

// Function: sub_a0a0 @ 0xa0a0
void sub_a0a0(unsigned long *a0,unsigned long *a1)
{
  sub_a070(*a1,*a0,sub_72a0); // tail-call
}

// Function: sub_a0c0 @ 0xa0c0
void sub_a0c0(unsigned long *a0,unsigned long *a1)
{
  sub_a070(*a0,*a1,dat_25fc0); // tail-call
}

// Function: sub_a0e0 @ 0xa0e0
void sub_a0e0(unsigned long *a0,unsigned long *a1)
{
  sub_a070(*a0,*a1,sub_72a0); // tail-call
}

// Function: sub_a100 @ 0xa100
void sub_a100(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,void *a4)
{
  if (!sub_5c40(a3)) {
    (*a4)(a0,a2); // jump-as-call
    return;
  }
}

// Function: sub_a130 @ 0xa130
void sub_a130(unsigned long *a0,unsigned long *a1)
{
  sub_a100(*a1,a1[9],*a0,a0[9],dat_25fc0); // tail-call
}

// Function: sub_a150 @ 0xa150
void sub_a150(unsigned long *a0,unsigned long *a1)
{
  sub_a100(*a1,a1[9],*a0,a0[9],sub_72a0); // tail-call
}

// Function: sub_a170 @ 0xa170
void sub_a170(unsigned long *a0,unsigned long *a1)
{
  sub_a100(*a0,a0[9],*a1,a1[9],dat_25fc0); // tail-call
}

// Function: sub_a1a0 @ 0xa1a0
void sub_a1a0(unsigned long *a0,unsigned long *a1)
{
  sub_a100(*a0,a0[9],*a1,a1[9],sub_72a0); // tail-call
}

// Function: sub_a1d0 @ 0xa1d0
unsigned int sub_a1d0(int a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 3) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == 9);
}

// Function: sub_a1e0 @ 0xa1e0
void sub_a1e0(long a0,char a1) // return-dupe
{
  unsigned int v1;
  int v2;
  unsigned long *v3;
  char *v4;
  void *v5; // rax
  long *v6;
  long v7;
  
  if ((a0) && (dat_273c8))
    sub_6840(0,a0,0);
  v7 = dat_273b0;
  if (!dat_273b0) {
    dat_273b0 = 0;
    return;
  }
  do {
    v7 -= 1;
    v3 = (unsigned long *)dat_273a0[v7];
    v1 = *(unsigned int *)&v3[0x15];
    if (sub_a1d0(v1)) {
      v4 = (char *)*v3;
      if (a0) { // branch-flip
        if (sub_73b0(v4)) goto label_a26c;
        if (*v4 == '/') goto label_a291;
        v5 = (void *)sub_e300(a0,v4,0);
        sub_6840(v5,v3[1],a1);
        free(v5);
        v2 = *(int *)&v3[0x15];
      }
      else {
label_a291:
        sub_6840(v4,v3[1],a1);
        v2 = *(int *)&v3[0x15];
      }
      if (v2 == 9)
        sub_6dc0(v3);
    }
label_a26c:
    if (!v7) {
      if (dat_273b0) {
        v7 = 0;
        v6 = dat_273a0;
        do {
          v2 = *(int *)(*v6 + 0xa8);
          dat_273a0[v7] = *v6;
          v6 = &v6[1];
          v7 += (unsigned long)(v2 != 9);
        } while (&dat_273a0[dat_273b0] != v6);
        dat_273b0 = v7;
        return;
      }
      dat_273b0 = 0;
      return;
    }
  } while( true );
}

// Function: sub_a330 @ 0xa330
undefined16 sub_a330(unsigned long a0,unsigned int a1)
{
  char v1 [16];
  
  v1._8_4_ = a1;
  v1._0_8_ = a0;
  v1._12_4_ = 0;
  return v1._0_16_;
}

// Function: sub_a340 @ 0xa340
void sub_a340(long a0,unsigned long *a1)
{
  unsigned int v1;
  unsigned int v2;
  unsigned long v3;
  unsigned long v4; // rax
  char v5 [16];
  
  v3 = gnu_dev_makedev(*(unsigned int *)(a0 + 0x88),*(unsigned int *)(a0 + 0x8c));
  v1 = *(unsigned int *)(a0 + 0x84);
  v2 = *(unsigned int *)(a0 + 0x80);
  *a1 = v3;
  a1[1] = *(unsigned long *)(a0 + 0x20);
  *(unsigned int *)&a1[3] = (unsigned int)*(unsigned short *)(a0 + 0x1c);
  a1[2] = (unsigned long)*(unsigned int *)(a0 + 0x10);
  *(unsigned int *)((long)a1 + 0x1c) = *(unsigned int *)(a0 + 0x14);
  *(unsigned int *)&a1[4] = *(unsigned int *)(a0 + 0x18);
  v4 = gnu_dev_makedev(v2,v1);
  v1 = *(unsigned int *)(a0 + 0x48);
  v3 = *(unsigned long *)(a0 + 0x40);
  a1[5] = v4;
  a1[6] = *(unsigned long *)(a0 + 0x28);
  a1[7] = (unsigned long)*(unsigned int *)(a0 + 4);
  a1[8] = *(unsigned long *)(a0 + 0x30);
  v5 = sub_a330(v3,v1);
  v1 = *(unsigned int *)(a0 + 0x78);
  v3 = *(unsigned long *)(a0 + 0x70);
  a1[9] = SUB168(v5,0);
  a1[10] = SUB168(v5,8);
  v5 = sub_a330(v3,v1);
  v1 = *(unsigned int *)(a0 + 0x68);
  v3 = *(unsigned long *)(a0 + 0x60);
  a1[0xb] = SUB168(v5,0);
  a1[0xc] = SUB168(v5,8);
  v5 = sub_a330(v3,v1);
  a1[0xd] = SUB168(v5,0);
  a1[0xe] = SUB168(v5,8);
}

// Function: sub_a400 @ 0xa400
int sub_a400(unsigned long a0,unsigned long a1,long a2,unsigned int a3,unsigned int a4)
{
  int v1; // eax
  long v2;
  char v3 [16];
  unsigned char v4; // stack - 0x137
  unsigned long v5; // stack - 0xe8
  unsigned int v6; // stack - 0xe0
  
  v1 = statx(a0,a1,a3 | 0x800,a4,&v2);
  if ((0 <= v1) && (sub_a340(&v2,a2), a4 & 0x800)) {
    if (v4 & 8) { // branch-flip
      v3 = sub_a330(v5,v6);
      *(long *)(a2 + 0x58) = SUB168(v3,0);
      *(long *)(a2 + 0x60) = SUB168(v3,8);
    }
    else {
      *(unsigned long *)(a2 + 0x60) = 0xffffffffffffffff;
      *(unsigned long *)(a2 + 0x58) = 0xffffffffffffffff;
    }
  }
  return v1;
}

// Function: sub_a4c0 @ 0xa4c0
void sub_a4c0(unsigned long a0,unsigned long a1)
{
  sub_a400(0xffffff9c,a0,a1,0,sub_68d0()); // tail-call
}

// Function: sub_a4f0 @ 0xa4f0
void sub_a4f0(unsigned long a0,unsigned long a1)
{
  sub_a400(0xffffff9c,a0,a1,0x100,sub_68d0()); // tail-call
}

// Function: sub_a530 @ 0xa530
void sub_a530(unsigned long a0,unsigned long a1)
{
  sub_a400(0xffffff9c,a0,a1,0,2); // tail-call
}

// Function: sub_a550 @ 0xa550
unsigned long sub_a550(char *a0,unsigned int a1,unsigned char a2,char *a3)
{
  unsigned long *v1;
  unsigned long v10; // rax
  char *v11;
  char *v12;
  void *v13; // rax
  long v14;
  unsigned long v15;
  int *v16; // rax
  unsigned char v17; // al
  unsigned long v18;
  int v19; // edx
  unsigned long *v2;
  int v20; // eax
  char *v21; // stack - 0x398
  char v22 [664];
  char v23 [32];
  char v24 [24];
  char **v25;
  char **v26; // rsp
  char **v27;
  char **v28;
  unsigned long *v29;
  char v3; // al
  char *v30; // stack - 0x390
  unsigned int v31; // stack - 0x370
  char v4;
  char v5;
  int v6;
  unsigned int v7;
  unsigned int v8;
  unsigned long v9;
  
  v28 = &v21;
  v27 = &v21;
  v25 = &v21;
  v26 = &v21;
  v14 = dat_273c0;
  if (dat_273b0 == dat_273b8) {
    v30 = a3;
    v14 = sub_16ad0(dat_273c0,dat_273b0,0x1a0);
    dat_273b8 <<= 1;
    a3 = v30;
    dat_273c0 = v14;
  }
  v1 = (unsigned long *)(v14 + dat_273b0 * 0xd0);
  *v1 = 0;
  v1[0x19] = 0;
  v18 = (unsigned long)(((int)v1 - (int)(unsigned long *)((unsigned long)&v1[1] & 0xfffffffffffffff8)) + 0xd0U >> 3);
  v29 = (unsigned long *)((unsigned long)&v1[1] & 0xfffffffffffffff8);
  while (v18) {
    v2 = &v29[1];
    *v29 = 0;
    v18 -= 1;
    v29 = v2;
  }
  *(unsigned int *)&v1[0x15] = a1;
  *(unsigned int *)((long)v1 + 0xc4) = 0xffffffff;
  if ((!dat_273a9) && (dat_273a8)) {
    v30 = a3;
    v17 = sub_71a0(a0);
    *(unsigned int *)((long)v1 + 0xc4) = (unsigned int)v17;
    a3 = v30;
    if (v17)
      dat_273a9 = '\x01';
  }
  v11 = a0;
  v4 = dat_27311;
  if (a2) { // branch-flip
    v25 = &v21;
    if ((*a0 != '/') && (v25 = &v21, *a3)) {
label_a893:
      v21 = (char *)CONCAT71(v21._1_7_,v4);
      v30 = a3;
      v10 = strlen(a0);
      v18 = strlen(v30);
      v12 = v30;
      v18 = v10 + 0x19 + v18;
      for (; v26 != (char **)((long)&v21 - (v18 & 0xfffffffffffff000)); v26 = (char *)((long)v26 + -0x1000)) {
        *(unsigned long *)((long)v26 + -8) = *(unsigned long *)((long)v26 + -8);
      }
      v18 = (unsigned long)((unsigned int)v18 & 0xff0);
      v14 = -v18;
      v25 = (char *)((long)v26 + v14);
      if (v18)
        *(unsigned long *)((long)v26 + -8) = *(unsigned long *)((long)v26 + -8);
      v30 = (char *)CONCAT71(v30._1_7_,v21._0_1_);
      v11 = (char *)((unsigned long)((long)v26 + v14 + 0xf) & 0xfffffffffffffff0);
      *(unsigned long *)((long)v26 + v14 + -8) = 0xae1f;
      sub_6140(v11,v12,a0);
      v4 = (char)v30;
    }
    v27 = v25;
    v28 = v25;
    if (v4) {
label_acb8:
      *(unsigned long *)((long)v28 + -8) = 0xacc5;
      v14 = sub_de90(v11,2);
      v1[2] = v14;
      v27 = v28;
      if (!v14) {
        *(unsigned long *)((long)v28 + -8) = 0xace5;
        v12 = dcgettext(NULL,"error canonicalizing %s",5,*(char *)((long)v28 + -8));
        *(unsigned long *)((long)v28 + -8) = 0xacf6;
        sub_6f50(a2,v12,v11);
      }
    }
label_a62b:
    v12 = (char *)&v1[3];
    if (4 <= dat_272f8) {
      if (dat_272f8 == 4) goto label_aa99;
label_a647:
      *(unsigned long *)((long)v27 + -8) = 0xa64f;
      v6 = sub_a4f0(v11,v12);
      v18 = 0;
      v25 = v27;
      v12 = v21;
      goto label_aaa8;
    }
    if ((dat_272f8 <= 1) || (!a2)) goto label_a647;
    v30 = (char *)CONCAT44(v30._4_4_,dat_272f8);
    v21 = v12;
    *(unsigned long *)((long)v27 + -8) = 0xac5e;
    v6 = sub_a4c0(v11,v12);
    if ((int)v30 != 2) {
      if (0 <= v6) { // branch-flip
        v12 = v21;
        if ((*(unsigned int *)&v1[6] & 0xf000) == 0x4000) goto label_ac8d;
      }
      else {
        v30 = v21;
        *(unsigned long *)((long)v27 + -8) = 0xb024;
        v5 = *(char *)((long)v27 + -8);
        v12 = v30;
        if (*__errno_location(v5) != 2) goto label_a711;
      }
      goto label_a647;
    }
label_ac8d:
    v18 = (unsigned long)a2;
    if (v6) {
label_a711:
      *(unsigned long *)((long)v27 + -8) = 0xb04b;
      v12 = dcgettext(NULL,"cannot access %s",5,*(char *)((long)v27 + -8));
      *(unsigned long *)((long)v27 + -8) = 0xb05e;
      sub_6f50(1,v12,v11);
      v1[0x16] = 0x2602a;
      return 0;
    }
  }
  else {
    if (dat_27311) {
      if ((*a0 != '/') && (v28 = &v21, *a3)) goto label_a893;
      goto label_acb8;
    }
    if (dat_272a1) {
label_a878:
      v27 = &v21;
      if (*a0 != '/') {
label_a886:
        v27 = &v21;
        if (*a3) goto label_a893;
      }
      goto label_a62b;
    }
    if ((a1 == 3) && (dat_27312)) {
      v30 = (char *)((unsigned long)v30 & 0xffffffffffffff00);
      v21 = a3;
      v3 = sub_6420(0x13);
      v4 = (char)v30;
      a3 = v21;
      if (!v3) {
        v3 = sub_6420(0x12);
        v4 = (char)v30;
        a3 = v21;
        if (!v3) {
          v3 = sub_6420(0x14);
          v4 = (char)v30;
          a3 = v21;
          if (!v3) goto label_a75f;
        }
      }
      goto label_a878;
    }
label_a75f:
    if (!dat_272fc) {
      if (dat_272a0) {
        if (0x41UL >> ((unsigned long)a1 & 0x3f) & 1) goto label_a786;
label_a7b5:
        if (dat_272a0) {
          if (a1) {
            v15 = 0;
            v25 = &v21;
            if (a1 != 5) goto label_a6fe;
            if (dat_27314 != 3) {
              v21 = a3;
              v30 = (char *)CONCAT71(v30._1_7_,v4);
              v25 = &v21;
              if (!dat_27312) goto label_a6fe;
              v3 = sub_6420(0xe);
              v4 = (char)v30;
              a3 = v21;
              if (!v3) {
                v3 = sub_6420(0x10);
                v4 = (char)v30;
                a3 = v21;
                if (!v3) {
                  v3 = sub_6420(0x11);
                  v4 = (char)v30;
                  a3 = v21;
                  if (!v3) {
                    v3 = sub_6420(0x15);
                    v4 = (char)v30;
                    v25 = &v21;
                    a3 = v21;
                    if (!v3) goto label_a6fe;
                  }
                }
              }
            }
          }
          goto label_a878;
        }
      }
      v15 = 0;
      goto label_a6fe;
    }
    if (!(0x41UL >> ((unsigned long)a1 & 0x3f) & 1)) goto label_a878;
label_a786:
    if (dat_272f8 != 4) {
      if (((!dat_27390) && (!dat_272fd)) && (!dat_272fc)) goto label_a7b5;
      goto label_a878;
    }
    if (*a0 != '/') goto label_a886;
label_aa99:
    *(unsigned long *)((long)v27 + -8) = 0xaaa1;
    v6 = sub_a4c0(v11,&v1[3]);
    v18 = 1;
    v25 = v27;
    v12 = v21;
label_aaa8:
    v27 = v25;
    v21 = v12;
    if (v6) {
      *(unsigned long *)((long)v25 + -8) = 0xaac1;
      v12 = dcgettext(NULL,"cannot access %s",5,*(char *)((long)v25 + -8));
      v15 = 0;
      *(unsigned long *)((long)v25 + -8) = 0xaad3;
      sub_6f50(a2,v12,v11);
      v1[0x16] = 0x2602a;
      if (a2)
        return 0;
      goto label_a6fe;
    }
  }
  *(char *)&v1[0x17] = 1;
  if (((a1 == 5) || ((*(unsigned int *)&v1[6] & 0xf000) == 0x8000)) && (dat_27312)) {
    v30 = (char *)CONCAT71(v30._1_7_,(char)v18);
    *(unsigned long *)((long)v27 + -8) = 0xac0d;
    v4 = sub_6420(0x15);
    v18 = (unsigned long)v30 & 0xff;
    if (v4) {
      v15 = v1[3];
      *(unsigned long *)((long)v27 + -8) = 0xac25;
      v5 = sub_8de0(v15);
      v18 = (unsigned long)v30 & 0xff;
      *(char *)&v1[0x18] = v5;
    }
  }
  if ((dat_27338) && (!dat_27365)) {
    v7 = *(unsigned int *)&v1[6] & 0xf000;
    if (v7 != 0xa000) goto label_a6ad;
label_ab70:
    if (dat_272fd) goto label_ab79;
label_abdf:
    v6 = 6;
  }
  else {
    *(unsigned long *)((long)v27 + -8) = 0xaafb;
    if (0 <= (int)sub_6f90(v11,v1,v18)) { // branch-flip
      *(unsigned long *)((long)v27 + -8) = 0xab08;
      v4 = sub_55d0();
      v12 = (char *)v1[0x16];
      if (v4) { // branch-flip
        *(unsigned long *)((long)v27 + -8) = 0xad8c;
        v6 = strcmp("_",v12,*(char *)((long)v27 + -8));
      }
      else {
        *(unsigned long *)((long)v27 + -8) = 0xab23;
        v6 = strcmp("unlabeled",v12,*(char *)((long)v27 + -8));
      }
      v7 = CONCAT31((undefined3)(a1 >> 8),v6 != 0);
      if (dat_27338) { // branch-flip
        if (v6) {
          dat_27364 = 1;
          *(unsigned int *)((long)v1 + 0xbc) = 1;
          v7 = *(unsigned int *)&v1[6] & 0xf000;
          if (v7 == 0xa000) goto label_ab70;
          goto label_a6ad;
        }
label_af1c:
        *(unsigned int *)((long)v1 + 0xbc) = 0;
      }
      else {
label_af93:
        *(unsigned long *)((long)v27 + -8) = 0xaf9e;
        v6 = sub_7020(v11,v1);
        v17 = 0 < v6 | (unsigned char)v7;
        if (v17) { // branch-flip
          v20 = 2 - (v6 <= 0 & v7);
          dat_27364 = v17;
        }
        else {
          v20 = 0;
        }
        *(int *)((long)v1 + 0xbc) = v20;
        if (v6 <= -1) {
          *(unsigned long *)((long)v27 + -8) = 0xafd3;
          v16 = __errno_location(*(char *)((long)v27 + -8));
          goto label_ae9a;
        }
      }
    }
    else {
      v1[0x16] = 0x2602a;
      *(unsigned long *)((long)v27 + -8) = 0xae73;
      v16 = __errno_location(*(char *)((long)v27 + -8));
      v6 = *v16;
      *(unsigned long *)((long)v27 + -8) = 0xae7f;
      v4 = sub_55c0(v6);
      if ((v19 == 0x3d) || (v4)) {
        if (dat_27338) goto label_af1c;
        v7 = 0;
        goto label_af93;
      }
      *(unsigned int *)((long)v1 + 0xbc) = 0;
label_ae9a:
      *(unsigned long *)((long)v27 + -8) = 0xaea9;
      v15 = sub_15700(0,3,v11);
      v6 = *v16;
      *(unsigned long *)((long)v27 + -8) = 0xaebf;
      error(0,v6,"%s",v15);
    }
    v7 = *(unsigned int *)&v1[6] & 0xf000;
    if (v7 == 0xa000) {
      if (dat_27338) goto label_ab70;
label_ab79:
      *(unsigned long *)((long)v27 + -8) = 0xab88;
      sub_70b0(v11,v1,a2);
      v15 = v1[1];
      *(unsigned long *)((long)v27 + -8) = 0xab94;
      v13 = (void *)sub_7100(v11,v15);
      if (v13) {
        if (!*(int *)((long)v1 + 0xc4)) {
          v15 = v1[1];
          *(unsigned long *)((long)v27 + -8) = 0xb109;
          if (sub_71a0(v15))
            *(unsigned int *)((long)v1 + 0xc4) = 0xffffffff;
        }
        if ((2 <= dat_27314) || (dat_272fd)) {
          *(unsigned long *)((long)v27 + -8) = 0xae3f;
          if (!sub_a530(v13,v24)) {
            *(char *)((long)v1 + 0xb9) = 1;
            *(unsigned int *)((long)v1 + 0xac) = v31;
          }
        }
      }
      *(unsigned long *)((long)v27 + -8) = 0xabcc;
      free(v13,*(char *)((long)v27 + -8));
      v7 = *(unsigned int *)&v1[6] & 0xf000;
      if (v7 == 0xa000) goto label_abdf;
    }
label_a6ad:
    v6 = 5;
    if ((v7 == 0x4000) && (v6 = 3, a2))
      v6 = (-(unsigned int)(dat_272f5 == '\0') & 6) + 3;
  }
  v15 = v1[0xb];
  *(int *)&v1[0x15] = v6;
  if ((dat_27338) && (!dat_2732c)) {
label_a6e2:
    if (dat_27365) {
label_a977:
      v11 = (char *)v1[0x16];
      *(unsigned long *)((long)v27 + -8) = 0xa983;
      v5 = *(char *)((long)v27 + -8);
      v6 = (int)strlen(v11,v5);
      if (dat_27354 < v6)
        dat_27354 = v6;
      goto label_a98f;
    }
label_a6ef:
    v6 = dat_2733c;
    v11 = v21;
    v12 = v30;
  }
  else {
    *(unsigned long *)((long)v27 + -8) = 0xa921;
    v9 = sub_ffa0(v15,v22,dat_27328,0x200,dat_27320);
    *(unsigned long *)((long)v27 + -8) = 0xa92b;
    v6 = sub_113a0(v9,0);
    if (dat_2735c < v6)
      dat_2735c = v6;
    if (dat_27338) goto label_a6e2;
    if (dat_26029) {
      v8 = *(unsigned int *)((long)v1 + 0x34);
      *(unsigned long *)((long)v27 + -8) = 0xaf78;
      v6 = sub_7260(v8);
      if (dat_27350 < v6)
        dat_27350 = v6;
    }
    if (dat_26028) {
      v8 = *(unsigned int *)&v1[7];
      *(unsigned long *)((long)v27 + -8) = 0xaf58;
      v6 = sub_7220(v8);
      if (dat_2734c < v6)
        dat_2734c = v6;
    }
    if (dat_2732e) {
      v8 = *(unsigned int *)((long)v1 + 0x34);
      *(unsigned long *)((long)v27 + -8) = 0xaf38;
      v6 = sub_7260(v8);
      if (dat_27348 < v6)
        dat_27348 = v6;
    }
    if (dat_27365) goto label_a977;
label_a98f:
    if (dat_27338) goto label_a6ef;
    v9 = v1[5];
    *(unsigned long *)((long)v27 + -8) = 0xa9ad;
    v11 = (char *)sub_10c20(v9,v23);
    *(unsigned long *)((long)v27 + -8) = 0xa9b5;
    v5 = *(char *)((long)v27 + -8);
    v6 = (int)strlen(v11,v5);
    if (dat_27358 < v6)
      dat_27358 = v6;
    if ((*(unsigned int *)&v1[6] & 0xb000) != 0x2000) { // branch-flip
      v9 = v1[9];
      *(unsigned long *)((long)v27 + -8) = 0xadb5;
      v9 = sub_59e0(v9,v22);
      *(unsigned long *)((long)v27 + -8) = 0xadca;
      v9 = sub_ffa0(v9);
      *(unsigned long *)((long)v27 + -8) = 0xadd4;
      v6 = sub_113a0(v9,0);
      v11 = v21;
      v12 = v30;
      if (v6 <= dat_2733c) goto label_a6ef;
    }
    else {
      v9 = v1[8];
      *(unsigned long *)((long)v27 + -8) = 0xa9e6;
      v8 = gnu_dev_major(v9);
      *(unsigned long *)((long)v27 + -8) = 0xa9f0;
      v11 = (char *)sub_10c20(v8,v22);
      *(unsigned long *)((long)v27 + -8) = 0xa9f8;
      v5 = *(char *)((long)v27 + -8);
      v6 = (int)strlen(v11,v5);
      if (dat_27344 < v6)
        dat_27344 = v6;
      v9 = v1[8];
      *(unsigned long *)((long)v27 + -8) = 0xaa0f;
      v8 = gnu_dev_minor(v9);
      *(unsigned long *)((long)v27 + -8) = 0xaa19;
      v11 = (char *)sub_10c20(v8,v22);
      *(unsigned long *)((long)v27 + -8) = 0xaa21;
      v5 = *(char *)((long)v27 + -8);
      v20 = (int)strlen(v11,v5);
      v6 = dat_27340;
      if (dat_27340 < v20) {
        v6 = v20;
        dat_27340 = v20;
      }
      v6 = v6 + 2 + dat_27344;
      v11 = v21;
      v12 = v30;
      if (v6 <= dat_2733c) goto label_a6ef;
    }
  }
  dat_2733c = v6;
  v25 = v27;
  v21 = v11;
  v30 = v12;
  if (dat_272fc) {
    v9 = v1[4];
    *(unsigned long *)((long)v27 + -8) = 0xaa70;
    v11 = (char *)sub_10c20(v9,v22);
    *(unsigned long *)((long)v27 + -8) = 0xaa78;
    v5 = *(char *)((long)v27 + -8);
    v6 = (int)strlen(v11,v5);
    if (dat_27360 < v6)
      dat_27360 = v6;
  }
label_a6fe:
  *(unsigned long *)((long)v25 + -8) = 0xa706;
  v9 = sub_16d80(a0);
  dat_273b0 += 1;
  *v1 = v9;
  return v15;
}

// Function: sub_b140 @ 0xb140
void sub_b140(unsigned long a0,unsigned long a1)
{
  sub_a400(a0,0x1abb9,a1,0x1000,0x100); // tail-call
}

// Function: sub_b160 @ 0xb160
void sub_b160(unsigned long a0,unsigned long a1)
{
  sub_a400(0xffffff9c,a0,a1,0,0x100); // tail-call
}

// Function: sub_b180 @ 0xb180
void sub_b180(char *a0,char *a1,char a2)
{
  unsigned long v1;
  unsigned long v10; // stack - 0x368
  char v11 [663];
  unsigned int v12;
  long v13; // stack - 0x378
  unsigned long v14; // stack - 0x360
  unsigned char v2; // al
  int v3; // eax
  int *v4; // rax
  void *v5; // rax
  void *v6; // rax
  char *v7; // rax
  unsigned long v8; // rax
  unsigned long v9;
  
  v4 = __errno_location();
  *v4 = 0;
  v5 = opendir(a0);
  if (!v5) {
    sub_6f50(a2,dcgettext(NULL,"cannot open directory %s",5),a0); // tail-call
    return;
  }
  if (dat_273c8) {
    v3 = dirfd(v5);
    if (0 <= v3) { // branch-flip
      v3 = sub_b140(v3);
      v9 = v10;
      v1 = v14;
    }
    else {
      v3 = sub_b160(a0,&v10);
      v9 = v10;
      v1 = v14;
    }
    if (v3 <= -1) {
      sub_6f50(a2,dcgettext(NULL,"cannot determine device and inode of %s",5),a0);
      closedir(v5); // tail-call
      return;
    }
    if (sub_7df0(v9)) {
      v9 = sub_15700(0,3,a0);
      error(0,0,dcgettext(NULL,"%s: not listing already-listed directory",5),v9);
      closedir(v5);
      sub_59b0(1); // tail-call
      return;
    }
    sub_7970(v9,v1);
  }
  sub_6e20();
  if ((dat_272f6) || (dat_272b8)) {
    if (!dat_26010)
      sub_7460(10);
    dat_26010 = '\0';
    v6 = NULL;
    sub_7810();
    if ((dat_27311) && (v6 = (void *)sub_de90(a0,2), !v6))
      sub_6f50(a2,dcgettext(NULL,"error canonicalizing %s",5),a0);
    if (!a1)
      a1 = a0;
    sub_8eb0(a1,dat_272c8,0xffffffff,0,1,0x27140,v6);
    free(v6);
    sub_77f0(":\n");
  }
  v13 = 0;
  do {
    *v4 = 0;
    v6 = readdir(v5);
    if (v6) { // branch-flip
      if (!sub_69f0((long)v6 + 0x13)) {
        v12 = 0;
        v2 = *(char *)((long)v6 + 0x12) - 1;
        if (v2 <= 0xd)
          v12 = *(unsigned int *)((unsigned long)v2 * 4 + 0x195e0);
        v13 += sub_a550((long)v6 + 0x13,v12,0,a0);
        if ((((dat_27338 == 1) && (dat_27330 == 6)) && (!dat_2732c)) && (!dat_272f6)) {
          sub_89f0();
          sub_9f20();
          sub_6e20();
        }
      }
    }
    else {
      if (!*v4) {
label_b357:
        if (closedir(v5))
          sub_6f50(a2,dcgettext(NULL,"closing directory %s",5),a0);
        sub_89f0();
        if (dat_272f6)
          sub_a1e0(a0,0);
        if ((!dat_27338) || (dat_2732c)) {
          v7 = (char *)sub_ffa0(v13,v11,dat_27328,0x200,dat_27320);
          v8 = strlen(v7);
          v7[-1] = ' ';
          v7[v8] = dat_26019;
          sub_7810();
          sub_77f0(dcgettext(NULL,"total",5));
          sub_77d0(&v7[-1],&(&v7[v8])[1 - (long)&v7[-1]]);
        }
        if (dat_273b0) {
          sub_9f20(); // tail-call
          return;
        }
        return;
      }
      sub_6f50(a2,dcgettext(NULL,"reading directory %s",5),a0);
      if (*v4 != 0x4b) goto label_b357;
    }
    sub_7ce0();
  } while( true );
}

// Function: sub_b690 @ 0xb690
void sub_b690(int a0)
{
  void *v1;
  unsigned long v2;
  char *v3; // rdi
  
  v2 = dat_27400;
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n",5),v2);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"List information about the FILEs (the current directory by default).\nSort entries alphabetically if none of -cftuvSUX nor --sort is specified.\n",5),v1);
    sub_6530();
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -a, --all                  do not ignore entries starting with .\n  -A, --almost-all           do not list implied . and ..\n      --author               with -l, print the author of each file\n  -b, --escape               print C-style escapes for nongraphic characters\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --block-size=SIZE      with -l, scale sizes by SIZE when printing them;\n                             e.g., \'--block-size=M\'; see SIZE format below\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -B, --ignore-backups       do not list implied entries ending with ~\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -c                         with -lt: sort by, and show, ctime (time of last\n                             modification of file status information);\n                             with -l: show ctime and sort by name;\n                             otherwise: sort by ctime, newest first\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -C                         list entries by columns\n      --color[=WHEN]         color the output WHEN; more info below\n  -d, --directory            list directories themselves, not their contents\n  -D, --dired                generate output designed for Emacs\' dired mode\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -f                         list all entries in directory order\n  -F, --classify[=WHEN]      append indicator (one of */=>@|) to entries WHEN\n      --file-type            likewise, except do not append \'*\'\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --format=WORD          across -x, commas -m, horizontal -x, long -l,\n                             single-column -1, verbose -l, vertical -C\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --full-time            like -l --time-style=full-iso\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -g                         like -l, but do not list owner\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --group-directories-first\n                             group directories before files;\n                             can be augmented with a --sort option, but any\n                             use of --sort=none (-U) disables grouping\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -G, --no-group             in a long listing, don\'t print group names\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -h, --human-readable       with -l and -s, print sizes like 1K 234M 2G etc.\n      --si                   likewise, but use powers of 1000 not 1024\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -H, --dereference-command-line\n                             follow symbolic links listed on the command line\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --dereference-command-line-symlink-to-dir\n                             follow each command line symbolic link\n                             that points to a directory\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --hide=PATTERN         do not list implied entries matching shell PATTERN\n                             (overridden by -a or -A)\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --hyperlink[=WHEN]     hyperlink file names WHEN\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --indicator-style=WORD\n                             append indicator with style WORD to entry names:\n                             none (default), slash (-p),\n                             file-type (--file-type), classify (-F)\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -i, --inode                print the index number of each file\n  -I, --ignore=PATTERN       do not list implied entries matching shell PATTERN\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -k, --kibibytes            default to 1024-byte blocks for file system usage;\n                             used only with -s and per directory totals\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -l                         use a long listing format\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -L, --dereference          when showing file information for a symbolic\n                             link, show information for the file the link\n                             references rather than for the link itself\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -m                         fill width with a comma separated list of entries\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -n, --numeric-uid-gid      like -l, but list numeric user and group IDs\n  -N, --literal              print entry names without quoting\n  -o                         like -l, but do not list group information\n  -p, --indicator-style=slash\n                             append / indicator to directories\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -q, --hide-control-chars   print ? instead of nongraphic characters\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --show-control-chars   show nongraphic characters as-is (the default,\n                             unless program is \'ls\' and output is a terminal)\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -Q, --quote-name           enclose entry names in double quotes\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --quoting-style=WORD   use quoting style WORD for entry names:\n                             literal, locale, shell, shell-always,\n                             shell-escape, shell-escape-always, c, escape\n                             (overrides QUOTING_STYLE environment variable)\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -r, --reverse              reverse order while sorting\n  -R, --recursive            list subdirectories recursively\n  -s, --size                 print the allocated size of each file, in blocks\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -S                         sort by file size, largest first\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --sort=WORD            sort by WORD instead of name: none (-U), size (-S),\n                             time (-t), version (-v), extension (-X), width\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --time=WORD            change the default of using modification times;\n                               access time (-u): atime, access, use;\n                               change time (-c): ctime, status;\n                               birth time: birth, creation;\n                             with -l, WORD determines which time to show;\n                             with --sort=time, sort by WORD (newest first)\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --time-style=TIME_STYLE\n                             time/date format with -l; see TIME_STYLE below\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -t                         sort by time, newest first; see --time\n  -T, --tabsize=COLS         assume tab stops at each COLS instead of 8\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -u                         with -lt: sort by, and show, access time;\n                             with -l: show access time and sort by name;\n                             otherwise: sort by access time, newest first\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -U                         do not sort; list entries in directory order\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -v                         natural sort of (version) numbers within text\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -w, --width=COLS           set output width to COLS.  0 means no limit\n  -x                         list entries by lines instead of by columns\n  -X                         sort alphabetically by entry extension\n  -Z, --context              print any security context of each file\n      --zero                 end each output line with NUL, not newline\n  -1                         list one file per line\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --help        display this help and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --version     output version information and exit\n",5),v1);
    sub_6560();
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nThe TIME_STYLE argument can be full-iso, long-iso, iso, locale, or +FORMAT.\nFORMAT is interpreted like in date(1).  If FORMAT is FORMAT1<newline>FORMAT2,\nthen FORMAT1 applies to non-recent files and FORMAT2 to recent files.\nTIME_STYLE prefixed with \'posix-\' takes effect only outside the POSIX locale.\nAlso the TIME_STYLE environment variable sets the default style to use.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nThe WHEN argument defaults to \'always\' and can also be \'auto\' or \'never\'.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nUsing color to distinguish file types is disabled both by default and\nwith --color=never.  With --color=auto, ls emits color codes only when\nstandard output is connected to a terminal.  The LS_COLORS environment\nvariable can change the settings.  Use the dircolors(1) command to set it.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nExit status:\n 0  if OK,\n 1  if minor problems (e.g., cannot access subdirectory),\n 2  if serious trouble (e.g., cannot access command-line argument).\n",5),v1);
    v3 = "ls";
    if ((dat_261e0 != 1) && (v3 = "dir", dat_261e0 != 2))
      v3 = "vdir";
    sub_7e40(v3);
  }
  exit(a0); // no-return
}

// Function: sub_bda0 @ 0xbda0
unsigned long sub_bda0(unsigned int a0,char **a1)
{
  char v1;
  unsigned long v10; // rax
  unsigned long v11; // rax
  char *v12;
  long *v13; // rax
  unsigned short v14; // stack - 0x48
  unsigned long v15;
  unsigned int v16; // stack - 0x74
  int v17; // stack - 0x70
  int v18; // stack - 0x6c
  unsigned long v19; // stack - 0x68
  char v2 [16];
  char *v20; // stack - 0x60
  long v21; // stack - 0x58
  int v22; // stack - 0x50
  unsigned short v23; // stack - 0x46
  unsigned int v24; // stack - 0x44
  bool v3;
  void *v4;
  unsigned char v5;
  int v6;
  unsigned int v7;
  long v8; // rax
  unsigned long v9; // rax
  
  v18 = -1;
  v19 = 0xffffffffffffffff;
  v21 = -1;
  v17 = -1;
  v22 = -1;
  v16 = 0xffffffff;
  v3 = 0;
  v20 = NULL;
label_be20:
  v23 = 0xffff;
  v14 = 0xffff;
  v15 = (unsigned long)a0;
  v6 = getopt_long(a0,a1,"abcdfghiklmnopqrstuvw:xABCDFGHI:LNQRST:UXZ1",(void *)0x252c0,(int *)&v14);
  if (v6 != -1) {
    switch(v6 + 0x83) {
      case 0:
        v12 = "ls";
        if ((dat_261e0 != 1) && (v12 = "dir", dat_261e0 != 2))
          v12 = "vdir";
        sub_16870(stdout,v12,"GNU coreutils",dat_261e8,"Richard M. Stallman","David MacKenzie",0,v15);
        exit(0); // no-return
      case 1:
        sub_b690(0); // no-return
      default:
        goto label_cc62;
      case 0xb4:
        v16 = (unsigned int)(v16 != 0);
        break;
      case 0xc4:
        dat_272f0 = 1;
        break;
      case 0xc5:
        sub_6380("*~");
        sub_6380(".*~");
        break;
      case 0xc6:
        v16 = 2;
        break;
      case 199:
        dat_27318 = 1;
        break;
      case 0xc9:
        if (optarg) {
          v8 = sub_d270("--classify",optarg,0x251a0,0x196c0,4,dat_261f0,1);
          v6 = *(int *)(v8 * 4 + 0x196c0);
          if ((v6 != 1) && ((v6 != 2 || (!sub_6590())))) break;
        }
        dat_27314 = 3;
        break;
      case 0xca:
        dat_26028 = 0;
        break;
      case 0xcb:
        dat_272f8 = 2;
        break;
      case 0xcc:
        sub_6380(optarg);
        break;
      case 0xcf:
        dat_272f8 = 4;
        break;
      case 0xd1:
        v17 = 0;
        break;
      case 0xd4:
        v17 = 5;
        break;
      case 0xd5:
        dat_272f6 = 1;
        break;
      case 0xd6:
        v18 = 3;
        break;
      case 0xd7:
        v21 = sub_16de0(optarg,0,0,0x7fffffffffffffff,0x1abb9,dcgettext(NULL,"invalid tab size",5),2);
        break;
      case 0xd8:
        v18 = 6;
        break;
      case 0xdb:
        v18 = 1;
        break;
      case 0xdd:
        dat_27365 = 1;
        break;
      case 0xe4:
        dat_272f0 = 2;
        break;
      case 0xe5:
        v17 = 7;
        break;
      case 0xe6:
        dat_27334 = 1;
        break;
      case 0xe7:
        dat_272f5 = 1;
        break;
      case 0xe9:
        dat_272f0 = 2;
        dat_27312 = 0;
        dat_27311 = 0;
        v7 = 0xffffffff;
        if (v16)
          v7 = v16;
        dat_2732c = 0;
        v18 = 6;
        v16 = v7;
        break;
      case 0xea:
        dat_26029 = 0;
        v16 = 0;
        break;
      case 0xeb:
        dat_27328 = 0xb0;
        dat_2731c = 0xb0;
        dat_27320 = 1;
        dat_26020 = 1;
        break;
      case 0xec:
        dat_272fc = 1;
        break;
      case 0xee:
        v3 = 1;
        break;
      case 0xef:
        v16 = 0;
        break;
      case 0xf0:
        v16 = 4;
        break;
      case 0xf1:
        dat_2732d = 1;
        v16 = 0;
        break;
      case 0xf2:
        dat_26028 = 0;
        v16 = 0;
        break;
      case 0xf3:
        dat_27314 = 1;
        break;
      case 0xf4:
        v22 = 1;
        break;
      case 0xf5:
        dat_2732f = 1;
        break;
      case 0xf6:
        dat_2732c = 1;
        break;
      case 0xf7:
        v18 = 5;
        break;
      case 0xf8:
        dat_27334 = 2;
        break;
      case 0xf9:
label_c180:
        v18 = 4;
        break;
      case 0xfa:
        v19 = sub_63b0(optarg);
        if (0 <= (long)v19) break;
        v10 = sub_158c0(optarg);
        error(2,0,"%s: %s",dcgettext(NULL,"invalid line width",5),v10);
        goto label_c180;
      case 0xfb:
        v16 = 3;
        break;
      case 0x103:
        dat_2732e = 1;
        break;
      case 0x104:
        v6 = sub_10810(optarg,0x27328,0x27320);
        if (v6)
          sub_17110(v6,CONCAT22(v23,v14),0,0x252c0,optarg); // no-return
        dat_2731c = dat_27328;
        dat_26020 = dat_27320;
        break;
      case 0x105:
        if (optarg) { // branch-flip
          v8 = sub_d270("--color",optarg,0x251a0,0x196c0,4,dat_261f0);
          v6 = *(int *)(v8 * 4 + 0x196c0);
          if (v6 == 1) goto label_c0ae;
          v5 = 0;
          if (v6 == 2)
            v5 = sub_6590();
        }
        else {
label_c0ae:
          v5 = 1;
        }
        dat_27312 = v5 & 1;
        break;
      case 0x106:
        dat_272f8 = 3;
        break;
      case 0x107:
        dat_27314 = 2;
        break;
      case 0x108:
        v8 = sub_d270("--format",optarg,0x25280,0x19730,4,dat_261f0);
        v16 = *(unsigned int *)(v8 * 4 + 0x19730);
        break;
      case 0x109:
        v16 = 0;
        v20 = (char *)sub_55b0("full-iso");
        break;
      case 0x10a:
        dat_272f4 = 1;
        break;
      case 0x10b:
        v13 = (long *)sub_169e0(0x10);
        *v13 = (long)optarg;
        v13[1] = (long)dat_272e0;
        dat_272e0 = v13;
        break;
      case 0x10c:
        if (optarg) { // branch-flip
          v8 = sub_d270("--hyperlink",optarg,0x251a0,0x196c0,4,dat_261f0,1);
          v6 = *(int *)(v8 * 4 + 0x196c0);
          if (v6 == 1) goto label_bf7f;
          v5 = 0;
          if (v6 == 2)
            v5 = sub_6590();
        }
        else {
label_bf7f:
          v5 = 1;
        }
        dat_27311 = v5 & 1;
        break;
      case 0x10d:
        v8 = sub_d270("--indicator-style",optarg,0x25940,0x19750,4,dat_261f0,1);
        dat_27314 = *(unsigned int *)(v8 * 4 + 0x19750);
        break;
      case 0x10e:
        v8 = sub_d270("--quoting-style",optarg,0x259e0,0x1d9c0,4,dat_261f0,1);
        v17 = *(int *)(v8 * 4 + 0x1d9c0);
        break;
      case 0x10f:
        goto label_be86;
      case 0x110:
        dat_27328 = 0x90;
        dat_2731c = 0x90;
        dat_27320 = 1;
        dat_26020 = 1;
        break;
      case 0x111:
        v8 = sub_d270("--sort",optarg,0x25240,0x19710,4,dat_261f0,1);
        v18 = *(int *)(v8 * 4 + 0x19710);
        break;
      case 0x112:
        v8 = sub_d270("--time",optarg,0x25200,0x196f0,4,dat_261f0,1);
        dat_27334 = *(int *)(v8 * 4 + 0x196f0);
        break;
      case 0x113:
        goto label_c47c;
      case 0x114:
        dat_26019 = '\0';
        dat_27312 = 0;
        v17 = 0;
        v16 = (unsigned int)(v16 != 0);
label_be86:
        v22 = 0;
      
    }
    goto label_be20;
  }
  if (!dat_27320) {
    v12 = getenv("LS_BLOCK_SIZE");
    sub_10810(v12,0x27328,0x27320);
    if ((v12) || (getenv("BLOCK_SIZE"))) {
      dat_2731c = dat_27328;
      dat_26020 = dat_27320;
    }
    if (v3) {
      dat_27328 = 0;
      dat_27320 = 0x400;
    }
  }
  if (0 <= (int)v16) { // branch-flip
    dat_27338 = v16;
    if (2 < v16 - 2) goto label_c89f;
label_c580:
    if ((v19 == 0xffffffffffffffff) && (((!sub_6590() || (ioctl(1,0x5413,&v14) <= -1)) || (v19 = (unsigned long)v23, !v23)))) {
      v12 = getenv("COLUMNS");
      if ((v12) && (*v12)) {
        v19 = sub_63b0(v12);
        if ((long)v19 <= -1) {
          v10 = sub_158c0(v12);
          error(0,0,dcgettext(NULL,"ignoring invalid width in environment variable COLUMNS: %s",5),v10);
          v19 = 0x50;
        }
      }
      else {
label_c8b8:
        v19 = 0x50;
      }
    }
  }
  else {
    if (dat_261e0 != 1) { // branch-flip
      if (dat_261e0 == 2) {
label_cae0:
        dat_27338 = 2;
        goto label_c580;
      }
      dat_27338 = 0;
    }
    else {
      if (sub_6590()) goto label_cae0;
      dat_27338 = 1;
    }
label_c89f:
    if (dat_27312) goto label_c580;
    if (v19 == 0xffffffffffffffff) goto label_c8b8;
  }
  dat_272b0 = v19;
  v2._8_8_ = 0;
  v2._0_8_ = v19;
  dat_27200 = (unsigned long)(v19 != (SUB168(v2._0_16_ * ZEXT816(0xaaaaaaaaaaaaaaab),8) & 0xfffffffffffffffe) + v19 / 3) + v19 / 3;
  if (dat_27338 - 2 <= 2) {
    if (0 <= v21) { // branch-flip
label_c7a4:
      dat_272c0 = v21;
    }
    else {
      dat_272c0 = 8;
      v12 = getenv("TABSIZE");
      if (v12) {
        if (!sub_17190(v12,0,0,&v14,0x1abb9)) {
          v21 = CONCAT44(v24,CONCAT22(v23,v14));
          goto label_c7a4;
        }
        v10 = sub_158c0(v12);
        error(0,0,dcgettext(NULL,"ignoring invalid tab size in environment variable TABSIZE: %s",5),v10);
      }
    }
  }
  v5 = (unsigned char)v22;
  if ((v22 == -1) && (v5 = 0, dat_261e0 == 1))
    v5 = sub_6590();
  dat_272d8 = v5 & 1;
  if (0 <= v17) { // branch-flip
label_c5fd:
    sub_15200(0,v17); // crossjump-dupe
  }
  else {
    v17 = sub_64b0();
    if ((0 <= v17) || (v17 = 7, dat_261e0 != 1)) goto label_c5fd;
    if (sub_6590()) {
      v17 = 3;
      sub_15200(0,v17);
    }
  }
  v6 = sub_151e0(0);
  if (((dat_27338) && ((2 <= dat_27338 - 2 || (!dat_272b0)))) || ((v6 != 3 && ((v6 != 6 && (v6 != 1)))))) {
    dat_273a8 = 0;
    dat_272d0 = sub_151a0(0);
    if (v6 == 7)
      sub_15220(dat_272d0,0x20,1);
  }
  else {
    dat_273a8 = 1;
    dat_272d0 = sub_151a0(0);
  }
  if (2 <= dat_27314) {
    v12 = &"*=>@|"[dat_27314 - 2];
    v1 = "*=>@|"[dat_27314 - 2];
    while (v1) {
      v12 = &v12[1];
      sub_15220(dat_272d0,(int)v1,1);
      v1 = *v12;
    }
  }
  dat_272c8 = sub_151a0(0);
  sub_15220(dat_272c8,0x3a,1);
  dat_27318 = (dat_27311 ^ 1) & dat_27338 == 0 & dat_27318;
  if ((int)dat_26019 < (int)(unsigned int)dat_27318) {
    error(2,0,dcgettext(NULL,"--dired and --zero are incompatible",5));
    return v9;
  }
  if (0 <= v18) { // branch-flip
label_c700:
    dat_27330 = v18;
    if (dat_27338) {
      return (unsigned long)optind; // return-dupe
    }
  }
  else {
    if (dat_27338) {
      if ((unsigned int)(dat_27334 - 1U) <= 2) {
        dat_27330 = 5;
        return (unsigned long)optind;
      }
      v18 = 0;
      goto label_c700;
    }
    dat_27330 = 0;
  }
  if ((!v20) && (v20 = getenv("TIME_STYLE"), !v20))
    v20 = (char *)sub_55b0("locale");
  while (!strncmp(v20,"posix-",6)) {
    if (!sub_e9c0(2)) {
      return (unsigned long)optind;
    }
    v20 = &v20[6];
  }
  if (*v20 != '+') { // branch-flip
    v13 = (long *)0x25980;
    v8 = sub_cf70(v20,0x25980,0x19760,4);
    if (v8 < 0) {
      sub_d0e0("time style",v20,v8);
      v4 = stderr;
      fputs_unlocked(dcgettext(NULL,"Valid arguments are:\n",5),v4);
      v12 = "full-iso";
      do {
        v13 = &v13[1];
        __fprintf_chk(stderr,1,"  - [posix-]%s\n",v12);
        v4 = stderr;
        v12 = (char *)*v13;
      } while (v12);
      fputs_unlocked(dcgettext(NULL,"  - +FORMAT (e.g., +%H:%M) for a \'date\'-style format\n",5),v4);
label_cc62:
      sub_b690(2); // no-return
    }
    switch(v8) { // branch-flip
      case 0:
        dat_26048 = "%Y-%m-%d %H:%M:%S.%N %z";
        dat_26040 = "%Y-%m-%d %H:%M:%S.%N %z";
        break;
      case 1:
        dat_26048 = "%Y-%m-%d %H:%M";
        dat_26040 = "%Y-%m-%d %H:%M";
        break;
      case 2:
        dat_26040 = "%Y-%m-%d ";
        dat_26048 = "%m-%d %H:%M";
        break;
      case 3:
        if (sub_e9c0(2)) {
          dat_26040 = dcgettext(NULL,dat_26040,2);
          dat_26048 = dcgettext(NULL,dat_26048,2);
        }
      
    }
  }
  else {
    v20 = &v20[1];
    v12 = strchr(v20,10);
    if (v12) { // branch-flip
      if (strchr(&v12[1],10)) {
        v10 = sub_158c0(v20);
        error(2,0,dcgettext(NULL,"invalid time style format %s",5),v10);
        return v11;
      }
      *v12 = '\0';
      dat_26048 = &v12[1];
    }
    else {
      dat_26048 = v20;
    }
    dat_26040 = v20;
  }
  sub_66c0();
  return (unsigned long)optind;
label_c47c:
  v20 = optarg;
  goto label_be20;
}

// Function: sub_cdd0 @ 0xcdd0
char * sub_cdd0(char *a0,unsigned long a1)
{
  unsigned long v1;
  char *v2;
  char *v3;
  unsigned long v4;
  char v5 [136];
  
  v4 = 0x80;
  if ((a1) && (v4 = a1 + 1, 0x401 <= a1))
    v4 = 0x401;
label_ce38:
  if (v4 != 0x80) goto label_ce83;
label_ce41:
  v2 = v5;
  v3 = NULL;
  if (a1) goto label_ce83;
  do {
    v1 = readlink(a0,v2,v4);
    if ((long)v1 < 0) {
      free(v3);
      return NULL;
    }
    if (v1 < v4) {
      v2[v1] = '\0';
      v1 += 1;
      if (!v3) {
        v3 = malloc(v1);
        if (!v3)
          return NULL;
        memcpy(v3,v2,v1);
        return v3;
      }
      if (v4 <= v1)
        return v3;
      v2 = realloc(v3,v1);
      if (!v2)
        return v3;
      return v2;
    }
    free(v3);
    if (0x3fffffffffffffff < v4) {
      if (v4 == 0x7fffffffffffffff) {
label_cecb:
        *__errno_location() = 0xc;
        return NULL;
      }
      v4 = 0x7fffffffffffffff;
      goto label_ce38;
    }
    v4 *= 2;
    if (v4 == 0x80) goto label_ce41;
label_ce83:
    v2 = malloc(v4);
    if (!v2) goto label_cecb;
    v3 = v2;
  } while( true );
}

// Function: sub_cf60 @ 0xcf60
void sub_cf60(void)
{
  sub_b690(1); // tail-call, no-return
}

// Function: sub_cf70 @ 0xcf70
unsigned long sub_cf70(char *a0,long *a1,void *a2,unsigned long a3)
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

// Function: sub_d080 @ 0xd080
long sub_d080(char *a0,long *a1) // return-dupe
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

// Function: sub_d0e0 @ 0xd0e0
void sub_d0e0(unsigned long a0,unsigned long a1,long a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (a2 != -1) // branch-flip
    v1 = dcgettext(NULL,"ambiguous argument %s for %s",5);
  else {
    v1 = dcgettext(NULL,"invalid argument %s for %s",5);
  }
  v2 = sub_158b0(1,a0);
  error(0,0,v1,sub_15530(0,8,a1),v2); // tail-call
}

// Function: sub_d170 @ 0xd170
void sub_d170(long *a0,void *a1,unsigned long a2)
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
      __fprintf_chk(stderr,1,", %s",sub_158c0(v1));
    else {
      __fprintf_chk(stderr,1,"\n  - %s",sub_158c0(v1));
      v4 = a1;
    }
    v3 += 1;
    a1 = (void *)((long)a1 + a2);
    v1 = a0[v3];
  }
  putc_unlocked(10,stderr); // tail-call
}

// Function: sub_d270 @ 0xd270
long sub_d270(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,void *a5,char a6) // early-return
{
  long v1; // rax
  
  if (a6) // branch-flip
    v1 = sub_cf70(a1,a2,a3,a4);
  else {
    v1 = sub_d080(a1,a2);
  }
  if (0 <= v1)
    return v1;
  sub_d0e0(a0,a1,v1);
  sub_d170(a2,a3,a4);
  (*a5)();
  return -1;
}

// Function: sub_d300 @ 0xd300
long sub_d300(void *a0,long *a1,void *a2,unsigned long a3) // return-dupe
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

// Function: sub_d360 @ 0xd360
char * sub_d360(char *a0)
{
  bool v1;
  char *v2;
  char v3;
  
  v3 = *a0;
  while (v3 == '/') {
    v2 = &a0[1];
    a0 = &a0[1];
    v3 = *v2;
  }
  if (v3) {
    v1 = 0;
    v2 = a0;
    do {
      while (v3 != '/') {
        if (v1) {
          v1 = 0;
          a0 = v2;
        }
        v3 = v2[1];
        v2 = &v2[1];
        if (!v3)
          return a0;
      }
      v3 = v2[1];
      v2 = &v2[1];
      v1 = 1;
    } while (v3);
  }
  return a0;
}

// Function: sub_d3c0 @ 0xd3c0
void sub_d3c0(char *a0)
{
  long v1;
  unsigned long v2;
  
  v2 = strlen(a0);
  do {
    if (2 > v2) break;
    v1 = v2 - 1;
    v2 -= 1;
  } while (a0[v1] == '/');
}

// Function: sub_d3f0 @ 0xd3f0
unsigned long sub_d3f0(int a0) // early-return
{
  unsigned long v1; // rax
  
  if (0x5a < a0)
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),(unsigned int)(a0 - 0x61U) <= 0x19);
  if (0x41 <= a0)
    return 1;
  return (unsigned long)((unsigned int)(a0 - 0x30U) <= 9);
}

// Function: sub_d420 @ 0xd420
bool sub_d420(int a0)
{
  if (a0 <= 0x5a)
    return 0x40 < a0;
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_d440 @ 0xd440
bool sub_d440(unsigned int a0)
{
  return a0 <= 0x7f;
}

// Function: sub_d450 @ 0xd450
unsigned int sub_d450(int a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x20) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == 9);
}

// Function: sub_d470 @ 0xd470
unsigned long sub_d470(unsigned int a0)
{
  unsigned long v1; // rax
  
  if ((int)a0 <= 0x1f)
    return (unsigned long)(~a0 >> 0x1f);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x7f);
}

// Function: sub_d490 @ 0xd490
bool sub_d490(int a0)
{
  return (unsigned int)(a0 - 0x30U) <= 9;
}

// Function: sub_d4a0 @ 0xd4a0
bool sub_d4a0(int a0)
{
  return (unsigned int)(a0 - 0x21U) <= 0x5d;
}

// Function: sub_d4b0 @ 0xd4b0
bool sub_d4b0(int a0)
{
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_d4c0 @ 0xd4c0
bool sub_d4c0(int a0)
{
  return (unsigned int)(a0 - 0x20U) <= 0x5e;
}

// Function: sub_d4d0 @ 0xd4d0
unsigned long sub_d4d0(int a0)
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

// Function: sub_d520 @ 0xd520
bool sub_d520(int a0)
{
  if (a0 <= 0xd)
    return 8 < a0;
  return a0 == 0x20;
}

// Function: sub_d540 @ 0xd540
bool sub_d540(int a0)
{
  return (unsigned int)(a0 - 0x41U) <= 0x19;
}

// Function: sub_d550 @ 0xd550
unsigned int sub_d550(int a0)
{
  if ((unsigned int)(a0 - 0x30U) <= 0x36)
    return (unsigned int)(0x7e0000007e03ff >> ((unsigned char)(a0 - 0x30U) & 0x3f)) & 1;
  return 0;
}

// Function: sub_d580 @ 0xd580
int sub_d580(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_d5a0 @ 0xd5a0
int sub_d5a0(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_d5c0 @ 0xd5c0
int sub_d5c0(char *a0,char *a1,long a2)
{
  char *v1;
  unsigned char v2; // al
  unsigned int v3; // eax
  
  if ((a0 == a1) || (!a2))
    return 0;
  v1 = &a0[a2 + -1];
  do {
    v3 = sub_d580(*a0);
    v2 = sub_d580(*a1);
    if ((a0 == v1) || (!(unsigned char)v3)) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while ((unsigned char)v3 == v2);
  return (v3 & 0xff) - (unsigned int)v2;
}

// Function: sub_d640 @ 0xd640
void sub_d640(long *a0)
{
  a0[1] = 0x400;
  *a0 = (long)&a0[2];
}

// Function: sub_d650 @ 0xd650
bool sub_d650(char *a0) // return-dupe
{
  char v1;
  char *v2; // rdi
  
  if (*a0 != '/')
    return 0;
  while( true ) {
    do {
      v2 = a0;
      v1 = v2[1];
      a0 = &v2[1];
    } while (v1 == '/');
    if (!v1)
      return 1;
    if (v1 != '.')
      return 0;
    v1 = v2[2];
    if (!v1) break;
    if (v1 == '.')
      return !v2[3] || v2[3] == '/';
    if (v1 != '/')
      return 0;
    a0 = &v2[2];
  }
  return 1;
}

// Function: sub_d6c0 @ 0xd6c0
unsigned int sub_d6c0(unsigned int a0)
{
  return CONCAT31((undefined3)(a0 - 1 >> 8),(a0 - 1 & a0) != 0);
}

// Function: sub_d6d0 @ 0xd6d0
unsigned int sub_d6d0(long *a0,unsigned long a1,unsigned long a2)
{
  unsigned int v1; // eax
  long v2;
  
  v2 = *a0;
  if (!v2) {
    v2 = sub_f380(7,0,sub_fba0,sub_fbd0,sub_fc10);
    *a0 = v2;
    if (!v2)
      sub_16da0(0); // no-return
  }
  v1 = sub_e100(v2,a1,a2);
  if (!(char)v1)
    sub_e070(*a0,a1,a2);
  return v1;
}

// Function: sub_d760 @ 0xd760
unsigned int sub_d760(unsigned long a0)
{
  int v1; // eax
  
  v1 = faccessat(0xffffff9c,a0,0,0x200);
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}

// Function: sub_d790 @ 0xd790
void sub_d790(unsigned long a0,unsigned short *a1)
{
  *a1 = dat_1d25e;
  sub_d760(); // tail-call
}

// Function: sub_d7a0 @ 0xd7a0
void sub_d7a0(unsigned long *a0)
{
  if ((unsigned long *)*a0 != &a0[2]) {
    free((unsigned long *)*a0); // tail-call
    return;
  }
}

// Function: sub_d7c0 @ 0xd7c0
long sub_d7c0(char *a0,unsigned int a1)
{
  char v1;
  unsigned long *v10; // rdx
  unsigned int v11; // ebx
  char *v12; // stack - 0x868
  char *v13; // stack - 0x458
  char v14 [144];
  long v15; // stack - 0x900
  char *v16;
  char *v17; // r13
  char *v18;
  char *v19;
  int v2;
  bool v20; // cf
  char *v21; // stack - 0x960
  char *v22; // stack - 0x950
  int v23; // stack - 0x92c
  long v24; // stack - 0x920
  unsigned long v25; // stack - 0x860
  long v26; // stack - 0x450
  bool v3;
  char *v4;
  char v5;
  long v6;
  unsigned long v7; // rax
  char *v8;
  unsigned long v9;
  
  v11 = a1 & 3;
  v15 = 0;
  if ((sub_d6c0(v11)) || (!a0)) {
    *__errno_location() = 0x16;
    return 0;
  }
  if (!*a0) {
    *__errno_location() = 2;
    return 0;
  }
  sub_d640(&v12);
  sub_d640(&v13);
  sub_d640(v10);
  v19 = (char *)*v10;
  if (*a0 != '/') { // branch-flip
    while (v9 = v10[1], !getcwd(v19,v9)) {
      v2 = *__errno_location();
      if (v2 == 0xc) {
        sub_16da0(); // no-return, return-dupe
      }
      if (v2 != 0x22) goto label_d886;
      if (!sub_15960(v10)) {
        sub_16da0();
      }
      v19 = (char *)*v10;
    }
    v8 = rawmemchr(v19,0);
    v5 = *a0;
    v21 = v8;
    if (v5) goto label_d934;
    v6 = 0;
  }
  else {
    *v19 = '/';
    v5 = *a0;
    v21 = &v19[1];
    if (!v5) {
      v22 = &v19[2];
      goto label_db4b;
    }
label_d934:
    v23 = 0;
    v3 = 0;
    do {
      v8 = v21;
      if (v5 == '/') {
        do {
          v5 = a0[1];
          a0 = &a0[1];
        } while (v5 == '/');
        if (!v5) break;
      }
      v18 = a0;
      do {
        v17 = v18;
        v1 = v17[1];
        v18 = &v17[1];
        if (!v1) break;
      } while (v1 != '/');
      v9 = (long)v18 - (long)a0;
      if (!v9) break;
      if (v9 != 1) { // branch-flip
        if (((v9 != 2) || (v5 != '.')) || (a0[1] != '.')) {
label_da05:
          if (v21[-1] != '/') {
            *v21 = '/';
            v21 = &v21[1];
          }
          if (&v19[v10[1] - (long)v21] < (char *)(v9 + 2)) {
            do {
              v6 = (long)v21 - (long)v19;
              if (!sub_159e0(v10)) {
                sub_16da0();
              }
              v19 = (char *)*v10;
              v21 = &v19[v6];
            } while ((char *)(v10[1] - v6) < (char *)(v9 + 2));
          }
          v8 = mempcpy(v21,a0,v9);
          *v8 = '\0';
          if (a1 & 4) { // branch-flip
label_da8d:
            if (v11 != 2) {
              if (sub_d650(v18)) { // branch-flip
                v5 = sub_d790(v19,v8);
label_dc16:
                if (v5 != '\x01') {
label_daba:
                  if (((v11 != 1) || (*__errno_location() != 2)) || (v18[strspn(v18,"/")])) {
label_dcf0:
                    if (v15) { // branch-flip
                      sub_f540(v15);
                      sub_d7a0(&v12);
                      sub_d7a0(&v13);
                    }
                    else {
label_d886:
                      sub_d7a0(&v12);
                      sub_d7a0(&v13);
                    }
                    sub_d7a0(v10);
                    return 0;
                  }
                }
              }
              else if (a1 & 4) { // branch-flip
                if (!*v18) {
                  v5 = sub_d760(v19);
                  goto label_dc16;
                }
              }
              else if (*__errno_location() != 0x16) goto label_daba;
            }
          }
          else {
            while( true ) {
              v4 = v13;
              v9 = v26 - 1;
              v7 = readlink(v19,v13,v9);
              if ((long)v9 > (long)v7) break;
              if (!sub_15960(&v13)) {
                sub_16da0();
              }
            }
            if ((long)v7 < 0) goto label_da8d;
            if (v23 <= 0x13) {
              v23 += 1;
label_dd32:
              a0 = v12;
              v4[v7] = '\0';
              v6 = (long)v18 - (long)v12;
              if (!v3)
                v6 = v24;
              v22 = (unsigned long)strlen(v18);
              if (!SCARRY8((long)v22 + 0x8000000000000000,v7)) {
                if (v25 <= v7 + (long)v22) {
                  do {
                    if (!sub_159e0(&v12)) {
                      sub_16da0();
                    }
                  } while (v25 <= v7 + (long)v22);
                  a0 = v12;
                }
                if (v3)
                  v18 = &a0[v6];
                memmove(&a0[v7],v18,(long)v22 + 1);
                memcpy(a0,v4,v7);
                v21 = &v19[1];
                v24 = v6;
                if (*v4 != '/') { // branch-flip
                  v3 = 1;
                  v5 = *a0;
                  v20 = v21 < v8;
                  v21 = v8;
                  if (v20) {
                    do {
                      v21 = &v8[-1];
                      if (v21 == v19) break;
                      v18 = &v8[-2];
                      v8 = v21;
                    } while (*v18 != '/');
                    v3 = 1;
                  }
                }
                else {
                  *v19 = '/';
                  v5 = *a0;
                  v3 = 1;
                }
                goto label_dafa;
              }
              sub_16da0();
            }
            if (!*a0) goto label_dd32;
            v8[(long)a0 - (long)v18] = '\0';
            v16 = ".";
            if (*v19)
              v16 = v19;
            if (stat(v16,v14)) goto label_dcf0;
            v8[(long)a0 - (long)v18] = *a0;
            if (!sub_d6d0(&v15,a0,v14)) goto label_dd32;
            if (v11 != 2) {
              *__errno_location() = 0x28;
              goto label_dcf0;
            }
          }
          v5 = v17[1];
          a0 = v18;
          v21 = v8;
        }
        else {
          a0 = v18;
          v5 = v1;
          if (&v19[1] < v21) {
            v21 = &v21[-1];
            if (v19 < v21) {
              do {
                if (v21[-1] == '/') break;
                v21 = &v21[-1];
              } while (v19 != v21);
              a0 = v18; // crossjump-dupe
              v5 = v1;
            }
          }
        }
      }
      else {
        if (v5 != '.') goto label_da05;
label_d9f8:
        a0 = v18;
        v5 = v1;
      }
label_dafa:
      v8 = v21;
    } while (v5);
    v6 = v15;
  }
  v22 = &v8[1];
  v21 = v8;
  if ((&v19[1] < v8) && (v8[-1] == '/')) {
    v21 = &v8[-1];
    v22 = v8;
  }
  if (v6)
    sub_f540(v6);
label_db4b:
  sub_d7a0(&v12);
  sub_d7a0(&v13);
  *v21 = '\0';
  v6 = sub_158d0(v10,(long)v22 - (long)v19);
  if (v6)
    return v6;
  sub_16da0();
}

// Function: sub_de90 @ 0xde90
void sub_de90(unsigned long a0,unsigned long a1)
{
  char v1 [1048];
  
  sub_d7c0(a0,a1,v1);
}

// Function: sub_dee0 @ 0xdee0
void sub_dee0(unsigned long a0)
{
  dat_273d8 = a0;
}

// Function: sub_def0 @ 0xdef0
void sub_def0(char a0)
{
  dat_273d0 = a0;
}

// Function: sub_df00 @ 0xdf00
void sub_df00(void)
{
  int v1; // eax
  int *v2; // rax
  char *v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_17f60(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!dat_273d0) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (dat_273d8) { // branch-flip
        v4 = sub_156e0(dat_273d8);
        error(0,*v2,"%s: %s",v4,v3);
      }
      else {
        error(0,*v2,"%s",v3);
      }
      _exit(dat_261f8); // return-dupe, no-return
    }
  }
  v1 = sub_17f60(stderr);
  if (!v1)
    return;
  _exit(dat_261f8);
}

// Function: sub_dfb0 @ 0xdfb0
void sub_dfb0(char *a0)
{
  char v1;
  long v2;
  unsigned long v3;
  
  v1 = *a0;
  v3 = sub_d360(a0) - (long)a0;
  do {
    if ((v1 == '/') >= v3) break;
    v2 = v3 - 1;
    v3 -= 1;
  } while (a0[v2] == '/');
}

// Function: sub_dff0 @ 0xdff0
char * sub_dff0(void *a0)
{
  unsigned long v1; // rax
  void *v2; // rax
  char *v3;
  
  v1 = sub_dfb0(a0);
  v2 = malloc(v1 + 1 + (unsigned long)(v1 == 0));
  v3 = NULL;
  if (v2) {
    v3 = memcpy(v2,a0,v1);
    if (!v1) {
      *v3 = 0x2e;
      v1 = 1;
    }
    v3[v1] = 0;
  }
  return v3;
}

// Function: sub_e060 @ 0xe060
unsigned long sub_e060(void)
{
  return 0;
}

// Function: sub_e070 @ 0xe070
void sub_e070(long a0,unsigned long a1,unsigned long *a2)
{
  unsigned long *v1; // rax
  unsigned long *v2; // rax
  
  if (!a0)
    return;
  v1 = (unsigned long *)sub_169e0(0x18);
  *v1 = sub_16d80(a1);
  v1[1] = a2[1];
  v1[2] = *a2;
  v2 = (unsigned long *)sub_f9a0(a0,v1);
  if (v2) {
    if (v1 != v2) {
      sub_fc10(v1); // tail-call
      return;
    }
    return;
  }
  sub_16da0(); // no-return
}

// Function: sub_e100 @ 0xe100
unsigned long sub_e100(long a0,unsigned long a1,unsigned long *a2) // early-return
{
  long v1; // rax
  unsigned long v2; // stack - 0x28
  unsigned long v3; // stack - 0x20
  unsigned long v4; // stack - 0x18
  
  if (!a0)
    return 0;
  v3 = a2[1];
  v4 = *a2;
  v2 = a1;
  v1 = sub_f120(a0,&v2);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 != 0);
}

// Function: sub_e160 @ 0xe160
unsigned long sub_e160(unsigned int a0) // early-return x7
{
  a0 &= 0xf000;
  if (a0 == 0x8000)
    return 0x2d;
  if (a0 == 0x4000)
    return 100;
  if (a0 == 0x6000)
    return 0x62;
  if (a0 == 0x2000)
    return 99;
  if (a0 == 0xa000)
    return 0x6c;
  if (a0 == 0x1000)
    return 0x70;
  if (a0 != 0xc000)
    return 0x3f;
  return 0x73;
}

// Function: sub_e1d0 @ 0xe1d0
void sub_e1d0(unsigned int a0,char *a1) // ternary x2
{
  unsigned char v1;
  char v2; // al
  unsigned long v3; // rcx
  
  v3 = (unsigned long)a0;
  *a1 = sub_e160(a0);
  a1[1] = (-((v3 & 0x100) == 0) & 0xbbU) + 0x72;
  a1[2] = (-((v3 & 0x80) == 0) & 0xb6U) + 0x77;
  v1 = -((v3 & 0x40) == 0);
  v2 = (v3 & 0x800) ? (v1 & 0xe0) + 0x73 : (v1 & 0xb5) + 0x78; // branch-flip
  a1[3] = v2;
  a1[4] = (-((v3 & 0x20) == 0) & 0xbbU) + 0x72;
  a1[5] = (-((v3 & 0x10) == 0) & 0xb6U) + 0x77;
  v1 = -((v3 & 8) == 0);
  v2 = (v3 & 0x400) ? (v1 & 0xe0) + 0x73 : (v1 & 0xb5) + 0x78; // branch-flip
  a1[6] = v2;
  a1[7] = (-((v3 & 4) == 0) & 0xbbU) + 0x72;
  a1[8] = (-((v3 & 2) == 0) & 0xb6U) + 0x77;
  v1 = -((v3 & 1) == 0);
  if (v3 & 0x200) {
    a1[9] = (v1 & 0xe0) + 0x74;
    *(unsigned short *)&a1[10] = 0x20;
    return;
  }
  a1[9] = (v1 & 0xb5) + 0x78;
  *(unsigned short *)&a1[10] = 0x20;
}

// Function: sub_e2f0 @ 0xe2f0
void sub_e2f0(long a0)
{
  sub_e1d0(*(unsigned int *)(a0 + 0x18)); // tail-call
}

// Function: sub_e300 @ 0xe300
void sub_e300(void)
{
  if (sub_e320())
    return;
  sub_16da0(); // no-return
}

// Function: sub_e320 @ 0xe320
void * sub_e320(void *a0,char *a1,unsigned long *a2)
{
  unsigned long v1;
  long v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  void *v5; // rax
  char *v6; // rax
  unsigned long v7; // rbx
  char v8; // stack - 0x41
  
  v2 = sub_d360(a0);
  v3 = sub_d3c0(v2);
  v1 = (v2 - (long)a0) + v3;
  v4 = strlen(a1);
  if (v3) { // branch-flip
    if (*(char *)((long)a0 + (v1 - 1)) != '/') { // branch-flip
      v8 = 0x2f;
      if (*a1 == '/')
        v8 = 0;
      v7 = (unsigned long)(*a1 != '/');
    }
    else {
      v8 = 0;
      v7 = 0;
    }
  }
  else {
    v8 = 0x2e;
    if (*a1 != '/')
      v8 = 0;
    v7 = (unsigned long)(*a1 == '/');
  }
  v5 = malloc(v1 + 1 + v4 + v7);
  if (v5) {
    v6 = mempcpy(v5,a0,v1);
    *v6 = v8;
    if (a2)
      *a2 = &v6[v7];
    *(char *)mempcpy(&v6[v7],a1,v4) = 0;
  }
  return v5;
}

// Function: sub_e420 @ 0xe420
unsigned long sub_e420(long a0,unsigned long *a1)
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
label_e44a:
  do {
    v7 = v3;
    v3 = v5 + 1;
    v6 = v5;
    while ((v5 = v3, v3 = v5, v5 < v2 && (*(char *)(a0 + v6) == '.'))) {
      v1 = *(char *)(a0 + 1 + v6);
      v4 = sub_d420((int)v1);
      if ((v1 != '~') && (!v4)) {
        if (0 <= (long)v2) goto label_e4b5;
        goto label_e44a;
      }
      v5 = v6 + 2;
      v6 = v5;
      v3 = v7;
      if (v2 <= v5) goto label_e44a;
      do {
        v5 = v6;
        v1 = *(char *)(a0 + v5);
        v4 = sub_d3f0((int)v1);
        if ((v1 != '~') && (!v4)) goto label_e44a;
        v6 = v5 + 1;
      } while (v6 < v2);
      v3 = v5 + 2;
    }
    if ((long)v2 <= -1) {
      if (!*(char *)(a0 + v6)) goto label_e4c7;
      goto label_e44a;
    }
label_e4b5:
    if (v2 == v6) {
label_e4c7:
      *a1 = v6;
      return v7;
    }
  } while( true );
}

// Function: sub_e4f0 @ 0xe4f0
unsigned int sub_e4f0(long a0,long a1,long a2) // ternary
{
  unsigned char v1;
  unsigned int v2; // r12d
  
  if (a1 != a2) { // branch-flip
    v1 = *(unsigned char *)(a0 + a1);
    v2 = (unsigned int)v1;
    if (sub_d490(v2))
      return 0;
    if (!sub_d420((unsigned int)v1)) {
      v2 = (v1 != 0x7e) ? v1 + 0x100 : 0xfffffffe; // branch-flip
    }
  }
  else {
    v2 = 0xffffffff;
  }
  return v2;
}

// Function: sub_e560 @ 0xe560
int sub_e560(long a0,long a1,long a2,long a3)
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
label_e580:
  if (v5 < a1) goto label_e59d;
label_e585:
  if (a3 <= v4)
    return 0;
  do {
    if (a1 <= v5) { // branch-flip
label_e5ab:
      if (a3 <= v4) goto label_e610;
      v6 = *(char *)(a2 + v4);
      if (sub_d490((int)v6)) {
        if (a1 <= v5) goto label_e654;
        goto label_e62d;
      }
    }
    else {
label_e59d:
      v6 = *(char *)(a0 + v5);
      if (sub_d490((int)v6)) goto label_e5ab;
    }
    v2 = sub_e4f0(a0,v5,a1);
    v3 = sub_e4f0(a2,v4,a3);
    if (v2 != v3)
      return v2 - v3;
    v5 += 1;
    v4 += 1;
  } while( true );
label_e610:
  if (v5 < a1) goto label_e62d;
  goto label_e585;
label_e62d:
  do {
    if (*(char *)(a0 + v5) != '0') goto label_e638;
    v5 += 1;
  } while (v5 < a1);
  if (a3 <= v4) goto label_e580;
  do {
    v6 = *(char *)(a2 + v4);
label_e654:
    if (v6 != '0') {
      if ((a1 <= v5) || (a3 <= v4)) {
        v7 = 0;
        goto label_e74f;
      }
      v7 = 0;
      goto label_e6ca;
    }
    v4 += 1;
label_e638:
  } while (v4 < a3);
  if (a1 <= v5) goto label_e580;
  v7 = 0;
label_e754:
  v6 = *(char *)(a0 + v5);
  if (sub_d490((int)v6))
    return 1;
label_e6db:
  if ((v4 < a3) && (v6 = *(char *)(a2 + v4), sub_d490((int)v6)))
    return -1;
  if (v7)
    return v7;
  goto label_e580;
  while( true ) {
    v1 = *(char *)(a2 + v4);
    if (!sub_d490((int)v1))
      return 1;
    v2 = (int)v6 - (int)v1;
    if (v7)
      v2 = v7;
    v5 += 1;
    v4 += 1;
    v7 = v2;
    if ((a1 <= v5) || (a3 <= v4)) break;
label_e6ca:
    v6 = *(char *)(a0 + v5);
    if (!sub_d490((int)v6)) goto label_e6db;
  }
label_e74f:
  if (a1 <= v5) goto label_e6db;
  goto label_e754;
}

// Function: sub_e7a0 @ 0xe7a0
unsigned long sub_e7a0(char *a0,long a1,char *a2,long a3) // early-return x4
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
    goto label_e806;
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
label_e8d4:
    if (a2[1] != '.') goto label_e806;
    v2 = 0;
  }
  else {
    if (a1 < 0) {
      if (!a0[2]) goto label_e90b;
      goto label_e8d4;
    }
    if (a1 != 2) goto label_e8d4;
label_e90b:
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
label_e806:
  v6 = a3;
  v7[0] = a1;
  v3 = sub_e420(a0,v7);
  v4 = sub_e420(a2,&v6);
  v1 = v7[0];
  if ((v7[0] != v3) || (v6 != v4)) {
    v5 = sub_e560(a0,v3,a2,v4);
    if ((int)v5)
      return v5;
    v4 = v6;
  }
  return sub_e560(a0,v1,a2,v4);
}

// Function: sub_e940 @ 0xe940
void sub_e940(unsigned long a0,unsigned long a1)
{
  sub_e7a0(a0,0xffffffffffffffff,a1,0xffffffffffffffff); // tail-call
}

// Function: sub_e960 @ 0xe960
void sub_e960(void *a0)
{
  clock_gettime(0,a0); // tail-call
}

// Function: sub_e970 @ 0xe970
undefined16 sub_e970(void)
{
  char v1 [16]; // stack - 0x28
  
  sub_e960(v1);
  return v1._0_16_;
}

// Function: sub_e9c0 @ 0xe9c0
bool sub_e9c0(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_15c30(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_ea50 @ 0xea50
unsigned long sub_ea50(unsigned long a0)
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

// Function: sub_eac0 @ 0xeac0
unsigned long sub_eac0(unsigned long a0)
{
  char v1; // al
  unsigned long v2; // rax
  unsigned long v3;
  
  v2 = 10;
  if (10 <= a0)
    v2 = a0;
  v3 = v2 | 1;
  while ((v3 != 0xffffffffffffffff && (v1 = sub_ea50(v3), !v1))) {
    v3 += 2;
  }
  v2 = v3;
  return v2;
}

// Function: sub_eb00 @ 0xeb00
unsigned long sub_eb00(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_17eb0(a0,3) % a1;
}

// Function: sub_eb20 @ 0xeb20
bool sub_eb20(long a0,long a1)
{
  return a1 == a0;
}

// Function: sub_eb30 @ 0xeb30
unsigned long sub_eb30(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0x1d2f0)
    return 1;
  v1 = v2[2];
  if ((((dat_1d304 < v1) && (v1 < dat_1d308)) && (dat_1d30c < v2[3])) && (dat_1d320 <= *v2)) {
    v3 = *v2 + dat_1d304;
    if (((v3 < v2[1]) && (v2[1] <= dat_1d310)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0x1d2f0;
  return 0;
}

// Function: sub_ebc0 @ 0xebc0
void sub_ebc0(long a0,unsigned long *a1)
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(a0 + 0x48);
  *a1 = 0;
  a1[1] = v1;
  *(unsigned long **)(a0 + 0x48) = a1;
}

// Function: sub_ebe0 @ 0xebe0
long sub_ebe0(long *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*(void *)a0[6])(a1,a0[2]);
  if (v1 < (unsigned long)a0[2])
    return v1 * 0x10 + *a0;
  abort(); // no-return
}

// Function: sub_ec10 @ 0xec10
long sub_ec10(long a0,long a1,unsigned long *a2,char a3) // early-return
{
  void *v1;
  long *v2;
  long v3;
  long *v4;
  long v5;
  
  v4 = (long *)sub_ebe0(a0,a1);
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
      sub_ebc0(a0);
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
      sub_ebc0(a0);
      return v5;
    }
    *v4 = 0;
  }
  return v5;
}

// Function: sub_ed10 @ 0xed10
void sub_ed10(long a0)
{
  if (*(long *)(a0 + 0x48)) {
    *(unsigned long *)(a0 + 0x48) = *(unsigned long *)(*(long *)(a0 + 0x48) + 8);
    return;
  }
  malloc(0x10); // tail-call
}

// Function: sub_ed40 @ 0xed40
unsigned long sub_ed40(long a0,unsigned long *a1,char a2) // return-dupe x2
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
label_ed68:
      v5 = &v5[2];
      if ((long *)a1[1] <= v5)
        return 1;
    }
    v3 = (long *)v5[1];
    if ((long *)v5[1]) {
      do {
        while( true ) {
          v6 = *v3;
          v2 = (long *)sub_ebe0(a0,v6);
          v4 = (long *)v3[1];
          if (*v2) break;
          *v2 = v6;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
          sub_ebc0(a0);
          v3 = v4;
          if (!v4) goto label_edd8;
        }
        v3[1] = v2[1];
        v2[1] = (long)v3;
        v3 = v4;
      } while (v4);
label_edd8:
      v6 = *v5;
    }
    v5[1] = 0;
    if (a2) goto label_ed68;
    v3 = (long *)sub_ebe0(a0,v6);
    if (*v3) { // branch-flip
      v4 = (long *)sub_ed10(a0);
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

// Function: sub_ee70 @ 0xee70
unsigned long sub_ee70(float a0,unsigned long a1,char a2) // early-return, ternary
{
  unsigned long v1; // rax
  
  if (!a2) {
    a0 = (float)a1 / a0;
    if (dat_1d314 <= a0)
      return 0;
    a1 = (a0 < dat_1d318) ? (unsigned long)a0 : (long)(a0 - dat_1d318) ^ 0x8000000000000000;
  }
  v1 = sub_eac0(a1);
  if ((!(v1 >> 0x3c & 1)) && (!(v1 >> 0x3d)))
    return v1;
  return 0;
}

// Function: sub_ef00 @ 0xef00
unsigned long sub_ef00(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}

// Function: sub_ef10 @ 0xef10
unsigned long sub_ef10(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}

// Function: sub_ef20 @ 0xef20
unsigned long sub_ef20(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}

// Function: sub_ef30 @ 0xef30
unsigned long sub_ef30(unsigned long *a0)
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

// Function: sub_ef90 @ 0xef90
bool sub_ef90(unsigned long *a0)
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
label_efb0:
      v2 = &v2[2];
    }
    v1 = v2[1];
    v4 += 1;
    v3 += 1;
    if (!v1) goto label_efb0;
    do {
      v1 = *(long *)(v1 + 8);
      v3 += 1;
    } while (v1);
    v2 = &v2[2];
  } while( true );
}

// Function: sub_f000 @ 0xf000
void sub_f000(unsigned long a0,void *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_ef20(a0);
  v2 = sub_ef00();
  v3 = sub_ef10();
  v4 = sub_ef30();
  __fprintf_chk(a1,1,"# entries:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v2);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v3);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v4); // tail-call
}

// Function: sub_f120 @ 0xf120
long sub_f120(long a0,long a1)
{
  void *v1;
  long *v2;
  long v3;
  
  v2 = (long *)sub_ebe0(a0,a1);
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

// Function: sub_f180 @ 0xf180
long sub_f180(unsigned long *a0)
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

// Function: sub_f1d0 @ 0xf1d0
long sub_f1d0(long a0,long a1)
{
  long v1;
  long *v2; // rax
  long *v3;
  
  v2 = (long *)sub_ebe0(a0,a1);
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

// Function: sub_f240 @ 0xf240
void sub_f240(unsigned long *a0,long a1,unsigned long a2)
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

// Function: sub_f290 @ 0xf290
long sub_f290(unsigned long *a0,void *a1,unsigned long a2)
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

// Function: sub_f310 @ 0xf310
unsigned long sub_f310(unsigned char *a0,unsigned long a1)
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

// Function: sub_f350 @ 0xf350
void sub_f350(unsigned long *a0)
{
  *(char *)&a0[2] = 0;
  *a0 = 0x3f80000000000000;
  a0[1] = 0x3fb4fdf43f4ccccd;
}

// Function: sub_f380 @ 0xf380
unsigned long * sub_f380(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_eb00;
  if (!a3)
    a3 = sub_eb20;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = 0x1d2f0;
  v1[5] = a1;
  if (sub_eb30(v1)) {
    v2 = sub_ee70(*(unsigned int *)(a1 + 8),a0,*(char *)(a1 + 0x10));
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

// Function: sub_f490 @ 0xf490
void sub_f490(unsigned long *a0)
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
        if ((long *)a0[1] <= v5) goto label_f52b;
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
label_f52b:
  a0[3] = 0;
  a0[4] = 0;
  return;
}

// Function: sub_f540 @ 0xf540
void sub_f540(unsigned long *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = (long *)*a0;
  v3 = (long *)a0[1];
  if ((a0[8]) && (a0[4])) {
    if (v3 <= v4) goto label_f5ec;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_f5af;
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
label_f5af:
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
label_f5ec:
  v2 = (void *)a0[9];
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free((void *)*a0);
  free(a0); // tail-call
}

// Function: sub_f630 @ 0xf630
unsigned int sub_f630(unsigned long *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_ee70(*(unsigned int *)(a0[5] + 8),a1,*(char *)(a0[5] + 0x10));
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
  v1 = sub_ed40(&v2,a0,0);
  if (!(char)v1) {
    a0[9] = v11;
    if ((sub_ed40(a0,&v2,1)) && (sub_ed40(a0,&v2,0))) {
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

// Function: sub_f7b0 @ 0xf7b0
unsigned long sub_f7b0(long a0,long a1,long *a2) // ternary
{
  long v1;
  long *v2; // rax
  long *v3; // stack - 0x38
  unsigned long v4; // rsi
  float v5; // xmm4_da
  float v6; // xmm5_da
  
  if (a1) {
    v1 = sub_ec10(a0,a1,&v3,0);
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
      sub_eb30(a0);
      v1 = *(long *)(a0 + 0x28);
      if (v5 * *(float *)(v1 + 8) < v6) {
        v5 = v5 * *(float *)(v1 + 0xc);
        if (!*(char *)(v1 + 0x10))
          v5 = v5 * *(float *)(v1 + 8);
        if (dat_1d314 <= v5)
          return 0xffffffff;
        v4 = (dat_1d318 <= v5) ? (long)(v5 - dat_1d318) ^ 0x8000000000000000 : (unsigned long)v5;
        if (!sub_f630(a0,v4))
          return 0xffffffff;
        if (sub_ec10(a0,a1,&v3,0)) {
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
    v2 = (long *)sub_ed10(a0);
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

// Function: sub_f9a0 @ 0xf9a0
unsigned long sub_f9a0(unsigned long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_f7b0(a0,a1,&v3);
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

// Function: sub_fa00 @ 0xfa00
long sub_fa00(long a0,unsigned long a1) // ternary x2
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
  
  v5 = sub_ec10(a0,a1,&v7,1);
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
      sub_eb30(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_1d318 <= v8) ? (long)(v8 - dat_1d318) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_f630(a0,v6)) {
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

// Function: sub_fb90 @ 0xfb90
void sub_fb90(void)
{
  sub_fa00(); // tail-call
}

// Function: sub_fba0 @ 0xfba0
unsigned long sub_fba0(unsigned long *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = sub_18140(*a0);
  return (v1 ^ a0[1]) % a1;
}

// Function: sub_fbd0 @ 0xfbd0
unsigned long sub_fbd0(unsigned long *a0,unsigned long *a1)
{
  int v1; // eax
  
  if ((a0[1] == a1[1]) && (a0[2] == a1[2])) {
    v1 = strcmp((char *)*a0,(char *)*a1);
    return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
  }
  return 0;
}

// Function: sub_fc10 @ 0xfc10
void sub_fc10(unsigned long *a0)
{
  free((void *)*a0);
  free(a0); // tail-call
}

// Function: sub_fc30 @ 0xfc30
void sub_fc30(void)
{
  return;
}

// Function: sub_fd00 @ 0xfd00
void * sub_fd00(void *a0,unsigned long a1,unsigned char *a2,char *a3)
{
  unsigned char v1;
  unsigned long v2; // rax
  void *v3; // rax
  char v4 [56];
  char *v5;
  unsigned long v6;
  unsigned long v7; // r15
  
  v2 = strlen(a3);
  __memcpy_chk(v4,a0,a1,0x29);
  a0 = (void *)((long)a0 + a1);
  v6 = 0xffffffffffffffff;
  while( true ) {
    v1 = *a2;
    if (v1) { // branch-flip
      if (0x7f <= v1) { // branch-flip
        v7 = 0;
        v5 = v4;
      }
      else {
        v6 = (unsigned long)v1;
        if (a1 < v1)
          v6 = a1;
        v7 = a1 - v6;
        v5 = &v4[v7];
        a1 = v6;
      }
      a2 = &a2[1];
      v6 = a1;
    }
    else {
      if (a1 < v6)
        v6 = a1;
      v7 = a1 - v6;
      v5 = &v4[v7];
    }
    v3 = memcpy((void *)((long)a0 - v6),v5,v6);
    if (!v7) break;
    a0 = memcpy((void *)((long)v3 - v2),a3,v2);
    a1 = v7;
  }
  return v3;
}

// Function: sub_fe00 @ 0xfe00
int sub_fe00(void)
{
  return (-(unsigned int)(getenv("POSIXLY_CORRECT") == NULL) & 0x200) + 0x200;
}

// Function: sub_fe30 @ 0xfe30
unsigned long sub_fe30(char *a0,unsigned long *a1,unsigned int *a2)
{
  char *v1;
  char v2;
  int v3; // eax
  unsigned long v4; // rax
  char *v5; // stack - 0x38
  unsigned int v6; // r13d
  
  if (((a0) || (a0 = getenv("BLOCK_SIZE"), a0)) || (a0 = getenv("BLOCKSIZE"), a0)) {
    v6 = 0;
    if (*a0 == '\'') {
      a0 = &a0[1];
      v6 = 4;
    }
    v3 = sub_cf70(a0,0x259b0,0x1d360,4);
    if (0 <= v3) { // branch-flip
      *a1 = 1;
      v6 |= *(unsigned int *)((long)v3 * 4 + 0x1d360);
    }
    else {
      v4 = sub_17190(a0,&v5,0,a1,"eEgGkKmMpPtTyYzZ0");
      if ((int)v4) {
        *a2 = 0;
        return v4;
      }
      v2 = *a0;
      while (10 <= (unsigned char)(v2 - 0x30U)) {
        if (v5 == a0) {
          if (v5[-1] != 'B') // branch-flip
            v6 |= 0x80;
          else {
            v6 |= 0x180;
            if (v5[-2] != 'i') break;
          }
          v6 |= 0x20;
          break;
        }
        v1 = &a0[1];
        a0 = &a0[1];
        v2 = *v1;
      }
    }
  }
  else {
    v6 = 0;
    *a1 = sub_fe00();
  }
  *a2 = v6;
  return 0;
}

// Function: sub_ffa0 @ 0xffa0
char * sub_ffa0(unsigned long a0,char *a1,unsigned int a2,unsigned long a3,unsigned long a4) // ternary
{
  unsigned int v1;
  char v10; // al
  unsigned int v11; // eax
  unsigned int v12; // eax
  unsigned long *v13; // rax
  unsigned long v14;
  unsigned long v15;
  unsigned long v16;
  unsigned long v17;
  long v18; // rax
  unsigned int v19; // edx
  unsigned long v2;
  unsigned int v20; // ebx
  char *v21;
  int v22; // esi
  unsigned int v23; // eax
  int v24;
  char *v25;
  unsigned long v26;
  char *v27;
  long double v28; // st0
  long double v29;
  bool v3;
  char *v30; // stack - 0x78
  char v4 [16];
  char v5 [16];
  char v6 [16];
  char v7 [16];
  char v8 [16];
  unsigned long v9;
  
  v11 = a2 & 3;
  v13 = (unsigned long *)localeconv();
  v21 = (char *)*v13;
  v20 = (-(unsigned int)((a2 & 0x20) == 0) & 0xffffffe8) + 0x400;
  v14 = strlen(v21);
  if (0x10 <= v14 - 1) {
    v14 = 1;
    v21 = ".";
  }
  v2 = v13[2];
  v27 = (char *)v13[1];
  if (0x11 <= strlen(v27))
    v27 = "";
  v30 = &a1[0x287];
  v19 = (unsigned int)v14; // branch-flip
  v25 = v30;
  if (a4 <= a3) {
    v4._8_8_ = 0;
    v4._0_8_ = a4;
    v5._8_8_ = 0;
    v5._0_8_ = a3;
    if (!(a3 % a4)) {
      v6._8_8_ = 0;
      v6._0_8_ = SUB168(v5._0_16_ / v4._0_16_,0);
      v7._8_8_ = 0;
      v7._0_8_ = a0;
      v8._0_16_ = v6._0_16_ * v7._0_16_;
      v15 = SUB168(v8._0_16_,0);
      if (!SUB168(v8._0_16_,8)) {
        v23 = 0;
        v17 = 0;
        v12 = 0;
        if (a2 & 0x10) goto label_10300;
label_10495:
        v24 = -1;
label_1030f:
        if (v11 != 1) goto label_1031e;
        if (5 < (int)((((unsigned int)v15 & 1) + v23 != 0) + v12)) goto label_1032f;
        goto label_10350;
      }
    }
label_1007a:
    v28 = (long double)(long)a3;
    if ((long)a3 < 0)
      v28 = v28 + (long double)dat_1d314;
    if (0 <= (long)a4) { // branch-flip
      v28 = v28 / (long double)(long)a4;
      if ((long)a0 > -1) goto label_100b7;
label_101a0:
      v28 = ((long double)(long)a0 + (long double)dat_1d314) * v28;
      if (!(a2 & 0x10)) goto label_101e8;
label_100c3:
      v24 = 0;
      v29 = (long double)(int)v20;
      do {
        v24 += 1;
        v29 = v29 * (long double)(int)v20;
        if (v28 < v29) break;
      } while (v24 != 8);
      v22 = 1;
      v21 = "%.1Lf";
      sub_fc30(v11);
      __sprintf_chk(a1,v22,0xffffffffffffffff,v21);
      v16 = strlen(a1);
      if ((v16 <= v14 + 2 + (unsigned long)((a2 & 0x20) == 0)) && ((!(a2 & 8) || (a1[v16 - 1] != '0'))))
        v14 = v16 - (v14 + 1);
      else {
        v21 = "%.0Lf";
        v22 = 1;
        sub_fc30(v11);
        __sprintf_chk(a1,v22,0xffffffffffffffff,v21);
        v16 = strlen(a1);
        v14 = v16;
      }
    }
    else {
      v28 = v28 / ((long double)(long)a4 + (long double)dat_1d314);
      if (0 > (long)a0) goto label_101a0;
label_100b7:
      v28 = (long double)(long)a0 * v28;
      if (a2 & 0x10) goto label_100c3;
label_101e8:
      v24 = 1;
      v21 = "%.0Lf";
      sub_fc30(v11);
      __sprintf_chk(a1,v24,0xffffffffffffffff,v21);
      v16 = strlen(a1);
      v24 = -1;
      v14 = v16;
    }
    v21 = memmove(&v30[-v16],a1,v16);
    v25 = &v21[v14];
  }
  else {
    if ((!a3) || (v16 = a4 / a3, a4 % a3)) goto label_1007a;
    v15 = a0 / v16;
    v17 = (a0 % v16) * 10;
    v9 = v17 / v16;
    v26 = (v17 % v16) * 2;
    v8._8_8_ = v26;
    v8._0_8_ = v15;
    v17 = v9 & 0xffffffff;
    v12 = (unsigned int)v9;
    if (v16 <= v26) { // branch-flip
      v23 = 2;
      if (v16 < v26)
        v23 = 3;
    }
    else {
      v23 = (unsigned int)(v26 != 0);
    }
    if (!(a2 & 0x10)) goto label_10495;
label_10300:
    v12 = (unsigned int)v17;
    v15 = SUB168(v8._0_16_,0);
    v26 = (unsigned long)v20;
    v24 = 0;
    v16 = v15;
    if (v15 < v26) goto label_1030f;
    while( true ) {
      v15 = v16 / v26;
      v12 = (int)v17 + (int)(v16 % v26) * 10;
      v17 = (unsigned long)v12 / (unsigned long)v20;
      v1 = ((int)v23 >> 1) + (v12 % v20) * 2;
      v12 = (unsigned int)v17;
      v23 += v1;
      v23 = (v20 <= v1) ? (v20 < v23) + 2 : (unsigned int)(v23 != 0); // branch-flip
      v24 += 1;
      if (v15 < v26) break;
      v16 = v15;
      if (v24 == 8) goto label_1030f;
    }
    if (10 <= v15) goto label_1030f;
    v16 = v15;
    if (v11 != 1) { // branch-flip
      if ((!(a2 & 3)) && (v23)) goto label_10612;
label_1068a:
      if (!v12) goto label_10693;
      v10 = (char)v17 + '0';
    }
    else {
      if (3 > (v12 & 1) + v23) goto label_1068a;
label_10612:
      v10 = (char)v17 + '1';
      if (v12 == 9) {
        v16 = v15 + 1;
        if (v15 == 9) goto label_1065c;
        v23 = 0;
        v15 = v16;
label_10693:
        if (a2 & 8) goto label_1065e;
        v10 = '0';
        v16 = v15;
      }
    }
    a1[0x286] = v10;
    v17 = v14 & 0xffffffff;
    v25 = &a1[0x286 - v14];
    if (8 <= v19) { // branch-flip
      *(unsigned long *)v25 = *(unsigned long *)v21;
      *(unsigned long *)&v25[(v14 & 0xffffffff) - 8] = *(unsigned long *)&v21[(v14 & 0xffffffff) - 8];
      v18 = (long)v25 - ((unsigned long)&v25[8] & 0xfffffffffffffff8);
      v12 = (int)v18 + v19 & 0xfffffff8;
      if (8 <= v12) {
        v23 = 0;
        do {
          v17 = (unsigned long)v23;
          v23 += 8;
          *(unsigned long *)(((unsigned long)&v25[8] & 0xfffffffffffffff8) + v17) = *(unsigned long *)&v21[v17 - v18];
        } while (v23 < v12);
      }
    }
    else if (v14 & 4) { // branch-flip
      *(unsigned int *)v25 = *(unsigned int *)v21;
      *(unsigned int *)&v25[v17 - 4] = *(unsigned int *)&v21[v17 - 4];
    }
    else if (v19) {
      *v25 = *v21;
      if (v14 & 2)
        *(unsigned short *)&v25[v17 - 2] = *(unsigned short *)&v21[v17 - 2];
    }
label_1065c:
    v23 = 0;
    v15 = v16;
label_1065e:
    v12 = 0;
    if (v11 == 1) goto label_10350;
label_1031e:
    if (a2 & 3) { // branch-flip
label_10350:
      v21 = v25;
      do {
        v21 = &v21[-1];
        *v21 = (char)v15 + (char)(v15 / 10) * '\xf6' + '0';
        v3 = 10 <= v15;
        v15 /= 10;
      } while (v3);
    }
    else {
      if (1 > (int)(v23 + v12)) goto label_10350;
label_1032f:
      v15 += 1;
      if (((!(a2 & 0x10)) || (v20 != v15)) || (v24 == 8)) goto label_10350;
      v24 += 1;
      if (!(a2 & 8)) {
        v25[-1] = '0';
        v25 = &v25[~v14];
        v16 = v14 & 0xffffffff;
        if (8 <= v19) { // branch-flip
          *(unsigned long *)v25 = *(unsigned long *)v21;
          *(unsigned long *)&v25[(v14 & 0xffffffff) - 8] = *(unsigned long *)&v21[(v14 & 0xffffffff) - 8];
          v18 = (long)v25 - ((unsigned long)&v25[8] & 0xfffffffffffffff8);
          v11 = (int)v18 + v19 & 0xfffffff8;
          if (8 <= v11) {
            v19 = 0;
            do {
              v14 = (unsigned long)v19;
              v19 += 8;
              *(unsigned long *)(((unsigned long)&v25[8] & 0xfffffffffffffff8) + v14) = *(unsigned long *)&v21[v14 - v18];
            } while (v19 < v11);
          }
        }
        else if (v14 & 4) { // branch-flip
          *(unsigned int *)v25 = *(unsigned int *)v21;
          *(unsigned int *)&v25[v16 - 4] = *(unsigned int *)&v21[v16 - 4];
        }
        else if ((v19) && (*v25 = *v21, v14 & 2))
          *(unsigned short *)&v25[v16 - 2] = *(unsigned short *)&v21[v16 - 2];
      }
      v25[-1] = '1';
      v21 = &v25[-1];
    }
  }
  if (a2 & 4)
    v21 = (char *)sub_fd00(v21,(long)v25 - (long)v21,v2,v27);
  if (!(a2 & 0x80)) goto label_1027e;
  if (v24 != -1) { // branch-flip
label_1026b:
    if (!(a2 & 0x100) && !v24) goto label_1027e;
    if (a2 & 0x40) {
label_104e1:
      v30 = &a1[0x288];
      a1[0x287] = ' ';
    }
    if (v24) goto label_104ff;
    v27 = v30;
    if (!(a2 & 0x100)) goto label_1027e;
  }
  else {
    if (a4 < 2) {
      v24 = 0;
      goto label_1026b;
    }
    v24 = 1;
    v14 = 1;
    do {
      v14 *= v20;
      if (a4 <= v14) break;
      v24 += 1;
    } while (v24 != 8);
    if (a2 & 0x40) goto label_104e1;
label_104ff:
    v27 = &v30[1];
    if ((a2 & 0x20) || (v24 != 1)) {
      *v30 = *(char *)((long)v24 + 0x1d368);
      if (!(a2 & 0x100)) {
label_105db:
        v30 = v27;
        goto label_1027e;
      }
      if (a2 & 0x20) {
        v30[1] = 'i';
        v27 = &v30[2];
      }
    }
    else {
      *v30 = 'k';
      if (!(a2 & 0x100)) goto label_105db;
    }
  }
  *v27 = 'B';
  v30 = &v27[1];
label_1027e:
  *v30 = '\0';
  return v21;
}

// Function: sub_10810 @ 0x10810
unsigned long sub_10810(unsigned long a0,unsigned long a1,long *a2)
{
  unsigned long v1; // rax
  
  v1 = sub_fe30(a0,a2,a1);
  if (*a2)
    return v1;
  *a2 = sub_fe00();
  return 4;
}

// Function: sub_10840 @ 0x10840
unsigned int * sub_10840(unsigned int a0) // early-return
{
  unsigned long *v1; // rax
  unsigned int *v2;
  char *v3;
  
  for (v2 = dat_273f8; v2; v2 = *(unsigned int **)&v2[2]) {
    if (*v2 == a0) goto label_1086d;
  }
  v3 = "";
  v1 = getpwuid(a0);
  if (v1)
    v3 = (char *)*v1;
  v2 = (unsigned int *)sub_169e0(strlen(v3) + 0x18 & 0xfffffffffffffff8);
  *v2 = a0;
  strcpy((char *)&v2[4],v3);
  *(unsigned int **)&v2[2] = dat_273f8;
  dat_273f8 = v2;
label_1086d:
  if ((char)v2[4])
    return &v2[4];
  return NULL;
}

// Function: sub_108d0 @ 0x108d0
unsigned int * sub_108d0(char *a0)
{
  char v1;
  void *v2; // rax
  unsigned int *v3;
  long v4;
  
  if (dat_273f8) {
    v1 = *a0;
    v3 = dat_273f8;
    do {
      if ((*(char *)&v3[4] == v1) && (!strcmp((char *)&v3[4],a0)))
        return v3;
      v3 = *(unsigned int **)&v3[2];
    } while (v3);
  }
  if (dat_273f0) {
    v1 = *a0;
    v4 = (long)dat_273f0;
    do {
      if ((*(char *)(v4 + 0x10) == v1) && (!strcmp((char *)(v4 + 0x10),a0)))
        return NULL;
      v4 = *(long *)(v4 + 8);
    } while (v4);
  }
  v2 = getpwnam(a0);
  v3 = (unsigned int *)sub_169e0(strlen(a0) + 0x18 & 0xfffffffffffffff8);
  strcpy((char *)&v3[4],a0);
  v4 = (long)dat_273f0;
  if (v2) {
    *v3 = *(unsigned int *)((long)v2 + 0x10);
    *(unsigned int **)&v3[2] = dat_273f8;
    dat_273f8 = v3;
    return v3;
  }
  dat_273f0 = v3;
  *(long *)&v3[2] = v4;
  return NULL;
}

// Function: sub_109e0 @ 0x109e0
unsigned int * sub_109e0(unsigned int a0) // early-return
{
  unsigned long *v1; // rax
  unsigned int *v2;
  char *v3;
  
  for (v2 = dat_273e8; v2; v2 = *(unsigned int **)&v2[2]) {
    if (*v2 == a0) goto label_10a0d;
  }
  v3 = "";
  v1 = getgrgid(a0);
  if (v1)
    v3 = (char *)*v1;
  v2 = (unsigned int *)sub_169e0(strlen(v3) + 0x18 & 0xfffffffffffffff8);
  *v2 = a0;
  strcpy((char *)&v2[4],v3);
  *(unsigned int **)&v2[2] = dat_273e8;
  dat_273e8 = v2;
label_10a0d:
  if ((char)v2[4])
    return &v2[4];
  return NULL;
}

// Function: sub_10a70 @ 0x10a70
unsigned int * sub_10a70(char *a0)
{
  char v1;
  void *v2; // rax
  unsigned int *v3;
  long v4;
  
  if (dat_273e8) {
    v1 = *a0;
    v3 = dat_273e8;
    do {
      if ((*(char *)&v3[4] == v1) && (!strcmp((char *)&v3[4],a0)))
        return v3;
      v3 = *(unsigned int **)&v3[2];
    } while (v3);
  }
  if (dat_273e0) {
    v1 = *a0;
    v4 = (long)dat_273e0;
    do {
      if ((*(char *)(v4 + 0x10) == v1) && (!strcmp((char *)(v4 + 0x10),a0)))
        return NULL;
      v4 = *(long *)(v4 + 8);
    } while (v4);
  }
  v2 = getgrnam(a0);
  v3 = (unsigned int *)sub_169e0(strlen(a0) + 0x18 & 0xfffffffffffffff8);
  strcpy((char *)&v3[4],a0);
  v4 = (long)dat_273e0;
  if (v2) {
    *v3 = *(unsigned int *)((long)v2 + 0x10);
    *(unsigned int **)&v3[2] = dat_273e8;
    dat_273e8 = v3;
    return v3;
  }
  dat_273e0 = v3;
  *(long *)&v3[2] = v4;
  return NULL;
}

// Function: sub_10b80 @ 0x10b80
char * sub_10b80(unsigned long a0,long a1)
{
  unsigned long v1;
  char *v2;
  char *v3; // r8
  
  *(char *)(a1 + 0x14) = 0;
  v2 = (char *)(a1 + 0x14);
  if ((long)a0 <= -1) {
    do {
      v3 = v2;
      v1 = (long)a0 / 10;
      v3[-1] = ((char)v1 * '\n' + '0') - (char)a0;
      a0 = v1;
      v2 = &v3[-1];
    } while (v1);
    v3[-2] = '-';
    return &v3[-2];
  }
  do {
    v2 = &v2[-1];
    v1 = a0 / 10;
    *v2 = (char)a0 + (char)v1 * '\xf6' + '0';
    a0 = v1;
  } while (v1);
  return v2;
}

// Function: sub_10c20 @ 0x10c20
char * sub_10c20(unsigned long a0,long a1)
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

// Function: sub_10c80 @ 0x10c80
unsigned long sub_10c80(unsigned int *a0,unsigned char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,(char *)a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_e9c0(0))) {
    v1 = 1;
    *a0 = (unsigned int)*a1;
  }
  return v1;
}

// Function: sub_10d10 @ 0x10d10
void sub_10d10(char *a0,char *a1,long a2)
{
  char *v1;
  char *v2;
  
  v1 = a0;
  if ((a0 < a1) && (v2 = a0, a2)) {
    do {
      v1 = &v2[1];
      *v2 = 0x20;
      if (!&a0[a2 - (long)v1]) break;
      v2 = v1;
    } while (v1 < a1);
  }
  *v1 = 0;
}

// Function: sub_10d40 @ 0x10d40
unsigned long sub_10d40(unsigned int *a0)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned long v3;
  
  v3 = 0;
  v2 = *a0;
  while (v2) {
    if (!iswprint(v2)) {
      *a0 = 0xfffd;
      v3 = 1;
    }
    v1 = &a0[1];
    a0 = &a0[1];
    v2 = *v1;
  }
  return v3;
}

// Function: sub_10d90 @ 0x10d90
unsigned long sub_10d90(int *a0,unsigned long a1)
{
  int v1;
  unsigned long v2;
  unsigned long v3;
  
  v1 = *a0;
  if (v1) { // branch-flip
    v3 = 0;
    do {
      v1 = wcwidth(v1);
      if (v1 != -1) // branch-flip
        v2 = (long)v1 + v3;
      else {
        *a0 = 0xfffd;
        v2 = v3 + 1;
      }
      if (a1 < v2) break;
      v1 = a0[1];
      a0 = &a0[1];
      v3 = v2;
    } while (v1);
  }
  else {
    v3 = 0;
  }
  *a0 = 0;
  return v3;
}

// Function: sub_10df0 @ 0x10df0
unsigned long sub_10df0(char *a0,long a1,long a2,unsigned long *a3,int a4,unsigned int a5)
{
  char v1; // al
  unsigned long v10;
  char *v11; // stack - 0x60
  unsigned long v2;
  unsigned long v3;
  void *v4; // rax
  long v5;
  void *v6; // rax
  char *v7; // rax
  unsigned long v8;
  unsigned long v9;
  
  v2 = strlen(a0);
  v10 = v2;
  v11 = a0;
  if ((a5 & 2) || (__ctype_get_mb_cur_max() <= 1)) {
label_10e29:
    v6 = NULL;
    v7 = NULL;
label_10e32:
    v3 = *a3;
    if (v10 <= v3) goto label_10ea3;
    v10 = v3;
    v2 = v3;
label_10e40:
    v3 = 0;
    *a3 = v10;
    if (a4) goto label_10e49;
label_10ebe:
    v8 = 0;
    v9 = v3;
  }
  else {
    v5 = mbstowcs(0,a0,0);
    if (v5 == -1) {
      if (!(a5 & 1)) {
        v6 = NULL; // return-dupe
        v7 = NULL;
        v10 = 0xffffffffffffffff;
        free(v6); // return-dupe
        free(v7);
        return v10;
      }
      goto label_10e29;
    }
    v5 += 1;
    v6 = malloc(v5 * 4);
    if (!v6) {
      if (!(a5 & 1)) {
        v6 = NULL;
        v7 = NULL;
        v10 = 0xffffffffffffffff;
        free(v6);
        free(v7);
        return v10;
      }
label_110a6:
      v7 = NULL;
      goto label_10e32;
    }
    if (!__mbstowcs_chk(v6,a0,v5,0x3fffffffffffffff)) goto label_110a6;
    *(unsigned int *)((long)v6 + (v5 * 4 - 4U)) = 0;
    v1 = sub_10d40(v6);
    v10 = (unsigned long)(int)wcswidth(v6,v5);
    if (v1) {
      v9 = wcstombs(0,v6,0);
label_11024:
      v7 = malloc(v9 + 1);
      if (v7) { // branch-flip
        v10 = sub_10d90(v6,*a3);
        v2 = wcstombs(v7,v6,v9 + 1);
        v11 = v7;
      }
      else if (!(a5 & 1)) {
        v10 = 0xffffffffffffffff;
        free(v6);
        free(v7);
        return v10;
      }
      goto label_10e32;
    }
    v3 = *a3;
    v9 = v2;
    if (v3 < v10) goto label_11024;
    v7 = NULL;
label_10ea3:
    if (v3 <= v10) goto label_10e40;
    v3 -= v10;
    *a3 = v10;
    if (!a4) goto label_10ebe;
label_10e49:
    v8 = v3;
    v9 = 0;
    if (a4 != 1) {
      v8 = (unsigned long)((unsigned int)v3 & 1) + (v3 >> 1);
      v9 = v3 >> 1;
    }
  }
  v10 = v8 + v2;
  if (a5 & 4) {
    v8 = 0;
    v10 = v2;
  }
  if (a5 & 8) // branch-flip
    v9 = 0;
  else {
    v10 += v9;
  }
  if (a2) {
    a2 = a1 + -1 + a2;
    v4 = (void *)sub_10d10(a1,a2,v8);
    v3 = a2 - (long)v4;
    if (v2 < (unsigned long)(a2 - (long)v4))
      v3 = v2;
    sub_10d10(mempcpy(v4,v11,v3),a2,v9);
  }
  free(v6);
  free(v7);
  return v10;
}

// Function: sub_110d0 @ 0x110d0
void * sub_110d0(unsigned long a0,unsigned long *a1,unsigned int a2,unsigned int a3)
{
  unsigned long v1;
  unsigned long v2;
  void *v3; // rax
  unsigned long v4;
  void *v5;
  
  v2 = *a1;
  v4 = v2;
  v5 = NULL;
  while( true ) {
    v1 = v4 + 1;
    v3 = realloc(v5,v1);
    if (!v3) {
      free(v5);
      return NULL;
    }
    *a1 = v2;
    v4 = sub_10df0(a0,v3,v1,a1,a2,a3);
    if (v4 == 0xffffffffffffffff) break;
    v5 = v3;
    if (v4 < v1)
      return v3;
  }
  free(v3);
  return NULL;
}

// Function: sub_11180 @ 0x11180
int sub_11180(unsigned char *a0,long a1,unsigned int a2)
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
          if (0x1e <= (unsigned char)(v2 + 0x9f)) goto label_11204;
label_112b3:
          a0 = &a0[1];
          v9 += 1;
        }
        else {
          if ('@' < (char)v2) goto label_112b3;
          if ('#' < (char)v2) {
            if (0x1b <= (unsigned char)(v2 - 0x25)) goto label_11204;
            goto label_112b3;
          }
          if ('\x1f' < (char)v2) goto label_112b3;
label_11204:
          v7 = 0;
          do {
            v5 = sub_10c80(&v8,a0,(long)v1 - (long)a0,&v7);
            if (v5 == -1) {
              if (a2 & 1)
                return -1;
              goto label_112b3;
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
label_11339:
          if (v9 == 0x7fffffff)
            return 0x7fffffff;
          v9 += 1;
        }
        else {
          if (a2 & 2)
            return -1;
          if (!(v3 & 2)) goto label_11339;
        }
      } while (v1 != a0);
    }
  }
  return v9;
}

// Function: sub_113a0 @ 0x113a0
void sub_113a0(char *a0,unsigned int a1)
{
  sub_11180(a0,strlen(a0),a1); // tail-call
}

// Function: sub_113d0 @ 0x113d0
void sub_113d0(unsigned long *a0,unsigned long a1,unsigned long *a2,void *a3)
{
  unsigned long v1; // rbx
  unsigned long v2;
  unsigned long *v3; // rbp
  unsigned long v4;
  unsigned long v5;
  unsigned long v6; // r15
  unsigned long v7; // stack - 0x40
  
  v7 = a1 >> 1;
  sub_114b0(&a0[v7],a1 - v7,a2,a3);
  sub_114b0(a0,v7,a2,a3);
  v4 = *a0;
  v5 = a0[v7];
  v1 = 0;
  v2 = v7;
  while( true ) {
    while( true ) {
      v3 = &a2[1];
      if ((int)(*a3)(v4,v5) <= 0) break;
      *a2 = v5;
      v6 = v2 + 1;
      v2 = v1;
      if (a1 == v6) {
        memcpy(v3,&a0[v2],(v7 - v2) * 8); // return-dupe, tail-call
        return;
      }
      v5 = a0[v6];
      a2 = v3;
      v2 = v6;
    }
    *a2 = v4;
    v1 += 1;
    if (v7 == v1) break;
    v4 = a0[v1];
    a2 = v3;
  }
  v7 = a1;
  memcpy(v3,&a0[v2],(v7 - v2) * 8);
  return;
}

// Function: sub_114b0 @ 0x114b0
void sub_114b0(unsigned long *a0,unsigned long a1,unsigned long *a2,void *a3) // return-dupe x2
{
  unsigned long v1; // rcx
  unsigned long v2; // rbp
  unsigned long v3;
  unsigned long v4;
  unsigned long v5;
  long v6;
  long v7; // r15
  
  if (3 <= a1) {
    v1 = a1 >> 1;
    sub_114b0(&a0[v1],a1 - v1,a2,a3);
    if (a1 != 3) { // branch-flip
      sub_113d0(a0,v1,a2,a3);
      v3 = *a2;
    }
    else {
      v3 = *a0;
      *a2 = v3;
    }
    v4 = a0[v1];
    v2 = 0;
    v5 = v1;
    v6 = 0;
    while( true ) {
      while( true ) {
        v7 = v6 + 1;
        if (1 <= (int)(*a3)(v3,v4)) break;
        a0[v6] = v3;
        v2 += 1;
        if (v1 == v2)
          return;
        v3 = a2[v2];
        v6 = v7;
      }
      a0[v6] = v4;
      v5 += 1;
      if (a1 == v5) break;
      v4 = a0[v5];
      v6 = v7;
    }
    memcpy(&a0[v7],&a2[v2],(v1 - v2) * 8); // tail-call
    return;
  }
  if (a1 != 2)
    return;
  v3 = a0[1];
  v4 = *a0;
  if ((int)(*a3)(v4,v3) <= 0)
    return;
  *a0 = v3;
  a0[1] = v4;
}

// Function: sub_11610 @ 0x11610
void sub_11610(long a0,long a1,unsigned long a2)
{
  sub_114b0(a0,a1,a0 + a1 * 8,a2); // tail-call
}

// Function: sub_11620 @ 0x11620
int sub_11620(int a0,int a1)
{
  int v1; // edx
  
  v1 = (a0 - a1) + 0x17e;
  return (a0 - v1) + 3 + (v1 / 7) * 7;
}

// Function: sub_11660 @ 0x11660
void sub_11660(long a0,long a1,long a2)
{
  long *v1; // rax
  
  if (a2) {
    v1 = __ctype_toupper_loc();
    do {
      a2 -= 1;
      *(char *)(a0 + a2) = (char)*(unsigned int *)(*v1 + (unsigned long)*(unsigned char *)(a1 + a2) * 4);
    } while (a2);
    return;
  }
}

// Function: sub_116b0 @ 0x116b0
void sub_116b0(long a0,long a1,long a2)
{
  long *v1; // rax
  
  if (a2) {
    v1 = __ctype_tolower_loc();
    do {
      a2 -= 1;
      *(char *)(a0 + a2) = (char)*(unsigned int *)(*v1 + (unsigned long)*(unsigned char *)(a1 + a2) * 4);
    } while (a2);
    return;
  }
}

// Function: sub_11700 @ 0x11700
long sub_11700(char *a0,long a1,char *a2,unsigned int *a3,char a4,unsigned int a5,int a6,unsigned long a7,int a8)
{
  int v1;
  char v10;
  bool v11;
  unsigned int v12;
  unsigned int v13; // eax
  unsigned long v14;
  unsigned char *v15;
  long v16;
  int v17;
  char *v18;
  long v19;
  bool v2;
  char v20;
  unsigned int v21;
  char *v22;
  unsigned long v23;
  char *v24;
  unsigned long v25; // stack - 0x450
  unsigned long v26; // stack - 0x448
  char v27 [1010];
  unsigned long v28; // stack - 0x488
  char v29;
  long v3;
  int v30; // ebp
  char *v31;
  char *v32;
  int v33;
  unsigned char v34;
  int v35;
  unsigned int v36;
  unsigned int v37;
  char *v38;
  long v39;
  unsigned char v4;
  int v40;
  unsigned long v41;
  char *v42;
  int *v43; // stack - 0x4e8
  unsigned int v44; // stack - 0x4e0
  unsigned long v45; // stack - 0x4c0
  unsigned long v46; // stack - 0x4b8
  int v47; // stack - 0x4a8
  unsigned long v48; // stack - 0x480
  unsigned long v49; // stack - 0x478
  int *v5;
  long v50; // stack - 0x470
  unsigned long v51; // stack - 0x468
  unsigned long v52; // stack - 0x460
  unsigned long v53; // stack - 0x458
  unsigned int v6;
  unsigned int v7;
  unsigned long v8;
  unsigned long v9;
  
  v9 = a7;
  v41 = (unsigned long)a6;
  v43 = __errno_location();
  v38 = *(char **)&a3[0xc];
  v37 = a3[2];
  v1 = *v43;
  if (!v38)
    v38 = "";
  if (0xd <= (int)v37) // branch-flip
    v44 = v37 - 0xc;
  else {
    v44 = 0xc;
    if (v37)
      v44 = v37;
  }
  v10 = *a2;
  v39 = 0;
  v5 = v43;
  v6 = (unsigned int)v26;
  v7 = v26._4_4_;
  v8 = a7;
  v35 = a6;
  v33 = a8;
label_11798:
  if (!v10) {
    if ((a0) && (a1))
      *a0 = '\0';
    *v5 = v1;
    return v39;
  }
  v24 = a2;
  a6 = v35;
  a7 = v8;
  a8 = v33;
  v26._0_4_ = v6;
  v26._4_4_ = v7;
  if (v10 != '%') {
    v23 = 0;
    if (0 <= (int)v41)
      v23 = v41;
    v14 = 1;
    if (v23)
      v14 = v23;
    if ((unsigned long)(a1 - v39) <= v14) goto label_11820;
    if (a0) {
      v42 = a0;
      if (1 < (int)v41) {
        v42 = &a0[v23 - 1];
        memset(a0,0x20,v23 - 1);
      }
      a0 = &v42[1];
      *v42 = *a2;
    }
    v39 += v14;
    goto label_117e8;
  }
  v12 = 0;
  v37 = 0;
  v10 = a4;
  while( true ) {
    v20 = (char)v12;
    v42 = &v24[1];
    v21 = (unsigned int)*v42;
    v24 = &v24[1];
    v34 = *v42 - 0x23;
    v23 = (unsigned long)v21;
    if (0x3d <= v34) break;
    v14 = 1L << (v34 & 0x3f);
    if (v14 & 0x1000000000002500) // branch-flip
      v37 = v21;
    else if (v34 != 0x3b) { // branch-flip
      v12 = (unsigned int)v14 & 1;
      if (!(v14 & 1)) break;
    }
    else {
      v10 = '\x01';
    }
  }
  v14 = (unsigned long)v21;
  if (v21 - 0x30 <= 9) {
    v41 = 0;
    do {
      v16 = (long)(int)v41 * 10;
      v40 = (int)v16;
      if ((v40 != v16) || (v41 = (unsigned long)(unsigned int)(v40 + *v24 + -0x30), SCARRY4(v40,*v24 + -0x30)))
        v41 = 0x7fffffff;
      v42 = &v24[1];
      v23 = (unsigned long)(unsigned int)(int)*v42;
      v24 = &v24[1];
      v14 = v23;
    } while ((unsigned int)((int)*v42 - 0x30U) <= 9);
  }
  if (((char)v23 != 'E') && ((char)v23 != 'O'))
    v14 = 0;
  else {
    v23 = (unsigned long)(unsigned char)v24[1];
    v24 = &v24[1];
  }
  v34 = (unsigned char)v23;
  v40 = (int)v41;
  v42 = v24;
  v32 = a0;
  if (0x7b <= v34) goto label_11950;
  v17 = (int)v14;
  v12 = v44;
  switch(v23 & 0xff) {
    case 0:
      v42 = &v24[-1];
      break;
    case 0x25:
      v42 = &v24[-1];
      if (v42 == a2) {
        if ((v37 != 0x2d) && (0 <= v40)) {
          v23 = (unsigned long)v40;
          v41 = 1;
          if (v23)
            v41 = v23;
          if ((unsigned long)(a1 - v39) <= v41) goto label_11820;
          if (a0) {
            if (2 <= v40) {
              v23 -= 1;
              v42 = &a0[v23];
              if ((v37 != 0x30) && (v37 != 0x2b)) {
                memset(a0,0x20,v23);
                a0 = v42;
              }
              else {
                memset(a0,0x30,v23);
                a0 = v42;
              }
            }
label_12c97:
            v42 = &a0[1];
            *a0 = *v24;
            a0 = v42;
          }
        }
        else {
          if ((unsigned long)(a1 - v39) <= 1) goto label_11820;
          v41 = 1;
          if (a0) goto label_12c97;
        }
        goto label_11fe7;
      }
      break;
    case 0x3a:
      v18 = &v24[1];
      v41 = 1;
      v20 = v24[1];
      while (v20 == ':') {
        v41 += 1;
        v18 = &v24[v41];
        v20 = v24[v41];
      }
      if (v20 == 'z') goto label_12310;
      break;
    case 0x41:
    case 0x61:
      if (v17) break;
      if (v20)
        v10 = v20;
      v29 = '\0';
      v15 = (unsigned char *)((long)&v25 + 5);
      goto label_11a3e;
    case 0x42:
      if (v17 != 0x45) {
        if (v20)
          v10 = v20;
        v29 = '\0';
        goto label_11a1e;
      }
      break;
    case 0x43:
      if (v17 != 0x45) {
        v21 = a3[5];
        v46._0_1_ = (int)v21 < -0x76c;
        v12 = (int)((-(unsigned int)(v21 + 0x76c < 0x76c) & 0xffffff9d) + v21) / 100 + 0x13;
        v11 = -0x76c <= (int)v21;
        if (!v37)
          v37 = a5;
        if (v37 != 0x2b) { // branch-flip
          v2 = 0;
          v47 = 2;
        }
        else {
          v47 = 2;
          v21 = 99;
label_12ccd:
          if (v12 <= v21) { // branch-flip
label_12c09:
            v2 = v47 < v40;
            v11 = (bool)(v46._0_1_ ^ 1);
            v37 = 0x2b;
          }
          else {
            v2 = 1;
            v37 = 0x2b;
            v11 = (bool)(v46._0_1_ ^ 1);
          }
        }
label_11ca5:
        v21 = 0;
        goto label_11cb0;
      }
      goto label_11cc2;
    case 0x44:
      if (!v17) {
        v17 = -1;
        v42 = "%m/%d/%y";
        goto label_11d8d;
      }
      break;
    case 0x46:
      if (v17) break;
      if ((v37) || (0 <= v40)) {
        v42 = "%Y-%m-%d";
        v17 = 0;
        if (0 <= v40 + -6)
          v17 = v40 + -6;
        goto label_11d8d;
      }
      v41 = sub_11700(0,0xffffffffffffffff,"%Y-%m-%d",a3,v10,0x2b,4,v9,v33,(long)*(int *)((v23 & 0xff) * 4 + 0x1d3c0) + 0x1d3c0);
      v42 = "%Y-%m-%d";
      v23 = 0;
      v37 = 0x2b;
      v45 = v41;
      v17 = 4;
      goto label_11e0b;
    case 0x47:
    case 0x56:
    case 0x67:
      if (v17 != 0x45) {
        v12 = a3[5];
        v13 = (v12 - 100) + ((int)v12 >> 0x1f & 400U);
        v21 = a3[7];
        v36 = v12;
        v35 = sub_11620(v21,a3[6]);
        if (0 <= v35) { // branch-flip
          v33 = 0x16d;
          if ((!(v13 & 3)) && (v33 = 0x16e, (v13 * -0x3d70a3d7 + 0x51eb850 >> 2 | v12 * 0x40000000) <= 0x28f5c28))
            v33 = 0x16e - (unsigned int)((int)v13 % 400 != 0);
          v33 = sub_11620(v21 - v33);
          if (0 <= v33)
            v35 = v33;
          v33 = (v33 >> 0x1f) + 1;
        }
        else {
          v35 = 0x16d;
          v13 -= 1;
          if ((!(v13 & 3)) && (v35 = 0x16e, (v13 * -0x3d70a3d7 + 0x51eb850 >> 2 | v13 * 0x40000000) <= 0x28f5c28))
            v35 = 0x16e - (unsigned int)((int)v13 % 400 != 0);
          v35 = sub_11620(v21 + v35);
          v33 = -1;
        }
        if (v34 == 0x47) {
          v12 = v36 + 0x76c + v33;
          v46._0_1_ = (int)v36 < 0xfffff894U - v33;
          if (v37) { // branch-flip
label_12cb6:
            v47 = 4;
            v21 = 9999;
            if (v37 == 0x2b) goto label_12ccd;
            v2 = 0;
            v11 = (bool)(v46._0_1_ ^ 1);
          }
          else {
            if (a5 == 0x2b) {
label_12ed6:
              v47 = 4;
              v21 = 9999;
              goto label_12ccd;
            }
            v2 = 0;
            v47 = 4;
            v11 = 0xfffff894U - v33 <= (int)v36;
            v37 = a5;
          }
          goto label_11ca5;
        }
        if (v34 == 0x67) {
          v12 = ((int)v36 % 100 + v33) % 100;
          if (0 <= (int)v12) { // branch-flip
            if (!v37)
              v37 = a5;
          }
          else {
            if ((int)v36 < 0xfffff894U - v33) {
              v12 = -v12;
              goto label_11d5a;
            }
            v12 += 100;
            if (!v37) goto label_11d60;
          }
          goto label_12bf6;
        }
        v47 = 2;
        v12 = v35 / 7 + 1;
        goto label_11f12;
      }
      break;
    case 0x48:
      if (v17 != 0x45) {
        v47 = 2;
        v12 = a3[2];
        goto label_11f12;
      }
      break;
    case 0x49:
      if (v17 != 0x45) {
        v47 = 2;
        goto label_11f12;
      }
      break;
    case 0x4d:
      if (v17 != 0x45) {
        v47 = 2;
        v12 = a3[1];
        goto label_11f12;
      }
      break;
    case 0x4e:
      if (v17 != 0x45) {
        v30 = 9;
        v17 = v33;
        if (v40 <= 0)
          v40 = 9;
        do {
          if (v30 <= v40) {
            if (v30 == 1) {
              v23 = 1;
              v41 = 1;
              goto label_125d5;
            }
            if (v17 % 10) goto label_125c7;
          }
          v30 -= 1;
          v17 /= 10;
        } while( true );
      }
      break;
    case 0x50:
      v29 = '\x01';
      goto label_128fc;
    case 0x52:
      v17 = -1;
      v42 = "%H:%M";
      goto label_11d8d;
    case 0x53:
      if (v17 != 0x45) {
        v47 = 2;
        v12 = *a3;
        goto label_11f12;
      }
      break;
    case 0x54:
      v17 = -1;
      v42 = "%H:%M:%S";
label_11d8d:
      v41 = sub_11700(0,0xffffffffffffffff,v42,a3,v10,v37,v17,v9,v33);
      if ((v37 != 0x2d) && (0 <= v40)) {
        v23 = (unsigned long)v40;
        v45 = v23;
        if (v23 <= v41)
          v45 = v41;
      }
      else {
        v45 = v41;
        v23 = 0;
      }
label_11e0b:
      if ((unsigned long)(a1 - v39) <= v45) goto label_11820;
      if (a0) {
        if (v41 < v23) {
          v23 -= v41;
          v32 = &a0[v23];
          if ((v37 != 0x30) && (v37 != 0x2b))
            memset(a0,0x20,v23);
          else {
            memset(a0,0x30,v23);
          }
        }
        a0 = &v32[v41];
        sub_11700(v32,a1 - v39,v42,a3,v10,v37,v17,v9,a8);
      }
      v39 += v45;
      goto label_117e8;
    case 0x55:
      if (v17 != 0x45) {
        v17 = (a3[7] - a3[6]) + 7;
label_12007:
        v47 = 2;
        v12 = v17 / 7;
        goto label_11f12;
      }
      break;
    case 0x57:
      if (v17 != 0x45) {
        v17 = (((int)(a3[6] + 6) / 7) * 7 - (a3[6] + 6)) + 7 + a3[7];
        goto label_12007;
      }
      break;
    case 0x58:
    case 99:
    case 0x78:
      if (v17 != 0x4f) goto label_11a1c;
      break;
    case 0x59:
      if (v17 == 0x45) goto label_11cc2;
      if (v17 == 0x4f) break;
      v46._0_1_ = (int)a3[5] < -0x76c;
      v12 = a3[5] + 0x76c;
      if (v37) goto label_12cb6;
      if (a5 == 0x2b) goto label_12ed6;
      v2 = 0;
      v21 = 0;
      v47 = 4;
      v37 = a5;
      goto label_120d0;
    case 0x5a:
      if (v20)
        v10 = '\0';
      v41 = strlen(v38);
      if ((v37 != 0x2d) && (0 <= v40)) {
        v23 = (unsigned long)v40;
        v46 = v23;
        if (v23 <= v41)
          v46 = v41;
      }
      else {
        v46 = v41;
        v23 = 0;
      }
      if ((unsigned long)(a1 - v39) <= v46) goto label_11820;
      if (a0) {
        v42 = a0;
        if (v41 < v23) {
          v23 -= v41;
          v42 = &a0[v23];
          if ((v37 != 0x30) && (v37 != 0x2b))
            memset(a0,0x20,v23);
          else {
            memset(a0,0x30,v23);
          }
        }
        if (v20) // branch-flip
          sub_116b0(v42);
        else if (v10) // branch-flip
          sub_11660(v42,v38,v41);
        else {
          memcpy(v42,v38,v41);
        }
        a0 = &v42[v41];
      }
      v39 += v46;
      goto label_117e8;
    case 0x62:
    case 0x68:
      if (v20)
        v10 = v20;
      if (v17 != 0x45) goto label_11a1c;
      break;
    case 100:
      if (v17 != 0x45) {
        v47 = 2;
        v12 = a3[3];
        goto label_11f12;
      }
      break;
    case 0x65:
      if (v17 != 0x45) {
        v12 = a3[3];
label_11efe:
        v47 = 2;
        if (!v37)
          v37 = 0x5f;
        goto label_11f12;
      }
      break;
    case 0x6a:
      if (v17 != 0x45) {
        v2 = 0;
        v47 = 3;
        v21 = a3[7];
        v12 = v21 + 1;
        v46._0_1_ = (int)v21 < -1;
        v11 = -1 <= (int)v21;
        v21 = 0;
        goto label_11cb0;
      }
      break;
    case 0x6b:
      if (v17 != 0x45) {
        v12 = a3[2];
        goto label_11efe;
      }
      break;
    case 0x6c:
      if (v17 != 0x45) goto label_11efe;
      break;
    case 0x6d:
      if (v17 != 0x45) {
        v2 = 0;
        v47 = 2;
        v21 = a3[4];
        v12 = v21 + 1;
        v46._0_1_ = (int)v21 < -1;
        v11 = -1 <= (int)v21;
        v21 = 0;
        goto label_11cb0;
      }
      break;
    case 0x6e:
      if ((v37 != 0x2d) && (0 <= v40)) {
        v23 = (unsigned long)v40;
        v41 = 1;
        if (v23)
          v41 = v23;
        if ((unsigned long)(a1 - v39) <= v41) goto label_11820;
        if (a0) {
          if (2 <= v40) {
            v23 -= 1;
            v42 = &a0[v23];
            if ((v37 != 0x30) && (v37 != 0x2b)) {
              memset(a0,0x20,v23);
              a0 = v42;
            }
            else {
              memset(a0,0x30,v23);
              a0 = v42;
            }
          }
label_11fdf:
          *a0 = '\n';
          a0 = &a0[1];
        }
      }
      else {
        if ((unsigned long)(a1 - v39) <= 1) goto label_11820;
        v41 = 1;
        if (a0) goto label_11fdf;
      }
      goto label_11fe7;
    case 0x70:
      v29 = '\0';
label_128fc:
      v34 = 0x70;
      if (v20) {
        v10 = '\0';
        v29 = v20;
      }
label_11a1e:
      if (v17) goto label_11cd1;
      v15 = (unsigned char *)((long)&v25 + 5);
      goto label_11a3e;
    case 0x71:
      if (v17 == 0x4f) goto label_11cc2;
      v2 = 0;
      v21 = 0;
      v47 = 1;
      v46._0_1_ = 0;
      v41 = (unsigned long)(((int)(a3[4] * 0xb) >> 5) + 1);
      goto label_120dc;
    case 0x72:
label_11a1c:
      v29 = '\0';
      goto label_11a1e;
    case 0x73:
      v51 = *(unsigned long *)&a3[8];
      v52 = *(unsigned long *)&a3[10];
      v28 = *(unsigned long *)a3;
      v48 = *(unsigned long *)&a3[2];
      v49 = *(unsigned long *)&a3[4];
      v53 = *(unsigned long *)&a3[0xc];
      v50._0_4_ = (unsigned int)*(unsigned long *)&a3[6];
      v50 = CONCAT44(0xffffffff,(unsigned int)v50);
      v16 = sub_16140(v9,&v28);
      if (v50 <= -1) {
        *v43 = 0x4b;
        return 0;
      }
      v46._0_1_ = SUB81((unsigned long)v16 >> 0x3f,0);
      v42 = &v27[1];
      v19 = v16;
      do {
        v3 = v19 / 10;
        v20 = (char)v19 + (char)v3 * '\xf6';
        if (v16 <= -1)
          v20 = -v20;
        v42 = &v42[-1];
        *v42 = v20 + '0';
        v19 = v3;
      } while (v3);
      v2 = 0;
      v47 = 1;
      goto label_12220;
    case 0x74:
      if ((v37 != 0x2d) && (0 <= v40)) {
        v23 = (unsigned long)v40;
        v41 = 1;
        if (v23)
          v41 = v23;
        if ((unsigned long)(a1 - v39) <= v41) goto label_11820;
        if (a0) {
          if (2 <= v40) {
            v23 -= 1;
            v42 = &a0[v23];
            if ((v37 != 0x30) && (v37 != 0x2b)) {
              memset(a0,0x20,v23);
              a0 = v42;
            }
            else {
              memset(a0,0x30,v23);
              a0 = v42;
            }
          }
label_1289b:
          *a0 = '\t';
          a0 = &a0[1];
        }
      }
      else {
        if ((unsigned long)(a1 - v39) <= 1) goto label_11820;
        v41 = 1;
        if (a0) goto label_1289b;
      }
label_11fe7:
      v39 += v41;
      goto label_117e8;
    case 0x75:
      v47 = 1;
      v12 = (int)(a3[6] + 6) % 7 + 1;
label_11f12:
      v2 = 0;
      v21 = 0;
      v4 = (unsigned char)(v12 >> 0x18);
      v46._0_1_ = (bool)(v4 >> 7);
      v11 = (bool)((unsigned char)~v4 >> 7);
      goto label_11cb0;
    case 0x77:
      if (v17 != 0x45) {
        v47 = 1;
        v12 = a3[6];
        goto label_11f12;
      }
      break;
    case 0x79:
      if (v17 != 0x45) {
        v21 = (int)a3[5] % 100;
        v12 = v21;
        if ((int)v21 <= -1) {
          v12 = v21 + 100;
          if ((int)a3[5] <= -0x76d)
            v12 = -v21;
        }
label_11d5a:
        if (v37) { // branch-flip
label_12bf6:
          if (v37 == 0x2b) {
label_12bfc:
            v46._0_1_ = 0;
            v47 = 2;
            goto label_12c09;
          }
        }
        else {
label_11d60:
          if (a5 == 0x2b) goto label_12bfc;
          v37 = a5;
        }
        v2 = 0;
        v11 = 1;
        v46._0_1_ = 0;
        v47 = 2;
        goto label_11ca5;
      }
label_11cc2:
      v29 = '\0';
label_11cd1:
      v25._5_1_ = (char)v14;
      v15 = (unsigned char *)((long)&v25 + 6);
label_11a3e:
      v25._3_2_ = 0x2520;
      *v15 = v34;
      v15[1] = 0;
      v41 = strftime((char *)&v26,0x400,(char *)((long)&v25 + 3),a3);
      if (v41) {
        v18 = (char *)(v41 - 1);
        if ((v37 != 0x2d) && (0 <= v40)) {
          v42 = (char *)(long)v40;
          v31 = v42;
          if (v42 <= v18)
            v31 = v18;
        }
        else {
          v42 = NULL;
          v31 = v18;
        }
        if ((char *)(a1 - v39) <= v31) goto label_11820;
        if (a0) {
          if (v18 < v42) {
            v41 = (long)v42 - (long)v18;
            v32 = &a0[v41];
            if ((v37 != 0x30) && (v37 != 0x2b))
              memset(a0,0x20,v41);
            else {
              memset(a0,0x30,v41);
            }
          }
          a2 = (char *)((long)&v26 + 1);
          if (!v29) goto label_11b17;
          sub_116b0(v32);
          a0 = &v32[(long)v18]; // crossjump-dupe
        }
        v39 += (long)v31; // crossjump-dupe
      }
      goto label_117e8;
    case 0x7a:
      v41 = 0;
      v18 = v24;
label_12310:
      v24 = v18;
      if ((int)a3[8] < 0) goto label_117e8;
      v46._0_1_ = 1;
      v35 = (int)*(unsigned long *)&a3[10];
      if ((0 <= v35) && (v46._0_1_ = 0, !v35))
        v46._0_1_ = *v38 == '-';
      v12 = v35 / 0xe10;
      v33 = (v35 / 0x3c) % 0x3c;
      if (v41 != 2) { // branch-flip
        if (3 <= v41) { // branch-flip
          v42 = v18;
          if (v41 != 3) break;
          if (v35 % 0x3c) goto label_12de3;
          if (v33) goto label_123ab;
          v2 = 1;
          v47 = 3;
          v11 = (bool)(v46._0_1_ ^ 1);
          v21 = 0;
        }
        else if (v41) { // branch-flip
label_123ab:
          v2 = 1;
          v47 = 6;
          v11 = (bool)(v46._0_1_ ^ 1);
          v12 = v12 * 100 + v33;
          v21 = 4;
        }
        else {
          v2 = 1;
          v47 = 5;
          v11 = (bool)(v46._0_1_ ^ 1);
          v12 = v12 * 100 + v33;
          v21 = 0;
        }
      }
      else {
label_12de3:
        v2 = 1;
        v47 = 9;
        v11 = (bool)(v46._0_1_ ^ 1);
        v21 = 0x14;
        v12 = v12 * 10000 + v33 * 100 + v35 % 0x3c;
      }
label_11cb0:
      if ((int)v14 == 0x4f) {
        if (v11) goto label_11cc2;
      }
label_120d0:
      v41 = (unsigned long)v12;
      if (v46._0_1_)
        v41 = (unsigned long)-v12;
label_120dc:
      v32 = &v27[1];
      while( true ) {
        if (v21 & 1) {
          v32[-1] = ':';
          v32 = &v32[-1];
        }
        v21 = (int)v21 >> 1;
        v42 = &v32[-1];
        v32[-1] = (char)v41 + (char)(v41 / 10) * '\xf6' + '0';
        if (((unsigned int)v41 <= 9) && (!v21)) break;
        v41 /= 10;
        v32 = v42;
      }
label_12220:
      v12 = v37;
      if (!v37)
        v12 = 0x30;
      if (v40 <= -1)
        v40 = v47;
      v35 = ((int)v27 + 1) - (int)v42;
      if (v46._0_1_) { // branch-flip
        v20 = '-';
label_12985:
        v33 = v40 + -1;
        v17 = v33 - v35;
        if ((v17 <= 0) || (v37 == 0x2d))
          v17 = 0;
        if (v12 != 0x5f) { // branch-flip
          if ((unsigned long)(a1 - v39) <= 1) goto label_11820;
          if (a0) goto label_129b7;
label_129be:
          v39 += 1;
          v40 = v33;
          if (v12 != 0x2d) goto label_129cb;
label_129fe:
          v31 = (char *)(long)v35;
          v32 = NULL;
        }
        else {
          v41 = (unsigned long)v17;
          v39 += v41;
          if (a0) {
            memset(a0,0x20,v41);
            a0 = &a0[v41];
            if ((unsigned long)(a1 - v39) <= 1) goto label_11820;
            v33 = (v40 - v17) + -1;
label_129b7:
            *a0 = v20;
            a0 = &a0[1];
            goto label_129be;
          }
          if ((unsigned long)(a1 - v39) <= 1) goto label_11820;
          v39 += 1;
          v40 = (v40 - v17) + -1;
label_129cb:
          v31 = (char *)(long)v35;
          v32 = NULL;
          if (0 <= v40) goto label_12266;
        }
      }
      else {
        if (v2) {
          v20 = '+';
          goto label_12985;
        }
        if (((v35 >= v40) || (v37 == 0x2d)) && (v12 == 0x2d)) goto label_129fe;
label_12266:
        v32 = (char *)(long)v40;
        v31 = (char *)(long)v35;
        if ((char *)(long)v35 <= v32)
          v31 = v32;
      }
      v18 = (char *)(long)v35;
      if ((char *)(a1 - v39) <= v31) goto label_11820;
      if (a0) {
        v22 = a0;
        if (v18 < v32) {
          v41 = (long)v32 - (long)v18;
          v22 = &a0[v41];
          if ((v12 != 0x30) && (v12 != 0x2b))
            memset(a0,0x20,v41);
          else {
            memset(a0,0x30,v41);
          }
        }
        if (v10) // branch-flip
          sub_11660(v22,v42,v18);
        else {
          memcpy(v22,v42,(unsigned long)v18);
        }
        a0 = &v22[(long)v18];
      }
      goto label_119fa;
    
  }
label_11950:
  v18 = &v42[1 - (long)a2];
  if ((0 <= v40) && (v37 != 0x2d)) {
    v22 = (char *)(long)v40;
    v31 = v22;
    if (v22 <= v18)
      v31 = v18;
  }
  else {
    v22 = NULL;
    v31 = v18;
  }
  if ((char *)(a1 - v39) <= v31) goto label_11820;
  v24 = v42;
  if (a0) {
    if (v18 < v22) {
      v41 = (long)v22 - (long)v18;
      v32 = &a0[v41];
      if ((v37 != 0x30) && (v37 != 0x2b))
        memset(a0,0x20,v41);
      else {
        memset(a0,0x30,v41);
      }
    }
label_11b17:
    if (v10) // branch-flip
      sub_11660(v32,a2,v18);
    else {
      memcpy(v32,a2,(unsigned long)v18);
    }
    a0 = &v32[(long)v18];
  }
label_119fa:
  v39 += (long)v31;
  goto label_117e8;
label_125c7:
  v23 = (unsigned long)v30;
  v41 = v23;
  if (v30) { // branch-flip
label_125d5:
    v42 = (char *)((long)&v26 + v41);
    do {
      v42 = &v42[-1];
      *v42 = (char)v17 + (char)(v17 / 10) * '\xf6' + '0';
      v17 /= 10;
    } while (v42 != (char *)((long)&v25 + (v41 - (unsigned int)(v30 - 1)) + 7));
  }
  else {
    v23 = 0;
  }
  if (!v37)
    v37 = 0x30;
  if ((unsigned long)(a1 - v39) > v23) {
    if (a0) {
      if (v10) // branch-flip
        sub_11660(a0,&v26,v23);
      else if (8 <= v23) { // branch-flip
        *(unsigned long *)a0 = CONCAT44(v7,v6);
        *(unsigned long *)&a0[v23 - 8] = *(unsigned long *)((long)&v25 + v23);
        v16 = (long)a0 - ((unsigned long)&a0[8] & 0xfffffffffffffff8);
        v41 = v16 + v23 & 0xfffffffffffffff8;
        if (8 <= v41) {
          v14 = 0;
          do {
            *(unsigned long *)(((unsigned long)&a0[8] & 0xfffffffffffffff8) + v14) = *(unsigned long *)((long)&v26 + (v14 - v16));
            v14 += 8;
          } while (v14 < v41);
        }
      }
      else if (v23 & 4) { // branch-flip
        *(unsigned int *)a0 = v6;
        *(unsigned int *)&a0[v23 - 4] = *(unsigned int *)((long)&v25 + v23 + 4);
      }
      else if (v23) {
        v26._0_1_ = (char)v6;
        *a0 = (char)v26;
        if (v23 & 2)
          *(unsigned short *)&a0[v23 - 2] = *(unsigned short *)((long)&v25 + v23 + 6);
      }
      a0 = &a0[v23];
    }
    v39 += v23;
    if ((v37 != 0x2d) && (0 <= v40 - v30)) {
      v41 = (unsigned long)(v40 - v30);
      if ((unsigned long)(a1 - v39) <= v41) goto label_11820;
      if (a0) { // branch-flip
        if (v41) {
          v42 = &a0[v41];
          v39 += v41;
          if ((v37 != 0x30) && (v37 != 0x2b)) {
            memset(a0,0x20,v41);
            a0 = v42;
          }
          else {
            memset(a0,0x30,v41);
            a0 = v42;
          }
        }
      }
      else {
        v39 += v41;
      }
    }
    else if (a1 == v39) goto label_11820;
label_117e8:
    v10 = v24[1];
    a2 = &v24[1];
    v41 = 0xffffffffffffffff;
    v5 = v43;
    v6 = (unsigned int)v26;
    v7 = v26._4_4_;
    v8 = a7;
    v35 = a6;
    v33 = a8;
    goto label_11798;
  }
label_11820:
  *v43 = 0x22;
  return 0;
}

// Function: sub_13030 @ 0x13030
void sub_13030(void)
{
  sub_11700();
}

// Function: sub_13050 @ 0x13050
void sub_13050(long a0,unsigned long a1)
{
  if (*(unsigned char *)(a0 + 0x50) & 1) {
    (**(void **)(a0 + 0x38))(*(unsigned long *)(a0 + 0x48)); // jump-as-call
    return;
  }
  (**(void **)(a0 + 0x38))(a1); // jump-as-call
}

// Function: sub_13070 @ 0x13070
void sub_13070(long a0,unsigned long a1)
{
  if (*(unsigned char *)(a0 + 0x50) & 1) {
    (**(void **)(a0 + 0x40))(*(unsigned long *)(a0 + 0x48)); // jump-as-call
    return;
  }
  (**(void **)(a0 + 0x40))(a1); // jump-as-call
}

// Function: sub_13090 @ 0x13090
unsigned long sub_13090(long *a0,long a1,long a2)
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
  v1 = (long *)sub_13050(a0);
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

// Function: sub_13120 @ 0x13120
void sub_13120(void)
{
  __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"memory exhausted",5));
  exit(dat_261f8); // no-return
}

// Function: _obstack_begin @ 0x13170
void _obstack_begin(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) & 0xfe;
  *(unsigned long *)(a0 + 0x38) = a3;
  *(unsigned long *)(a0 + 0x40) = a4;
  sub_13090(); // tail-call
}

// Function: _obstack_begin_1 @ 0x13190
void _obstack_begin_1(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5)
{
  *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 1;
  *(unsigned long *)(a0 + 0x38) = a3;
  *(unsigned long *)(a0 + 0x40) = a4;
  *(unsigned long *)(a0 + 0x48) = a5;
  sub_13090(); // tail-call
}

// Function: _obstack_newchunk @ 0x131b0
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
    v4 = (unsigned long *)sub_13050(a0);
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
        sub_13070(a0,v2);
      }
      a0[2] = (unsigned long)v7;
      a0[3] = (long)v7 + v6;
      *(unsigned char *)&a0[10] = (unsigned char)a0[10] & 0xfd;
      return;
    }
  }
  (*obstack_alloc_failed_handler)(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: _obstack_allocated_p @ 0x132b0
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

// Function: _obstack_free @ 0x132f0
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
      sub_13070(a0);
      *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 2;
    } while (v2);
  }
  if (!a1)
    return;
  abort(); // no-return
}

// Function: _obstack_memory_used @ 0x13360
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

// Function: sub_13390 @ 0x13390
void sub_13390(char *a0)
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
    dat_27400 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_13430 @ 0x13430
int * sub_13430(int *a0,int a1)
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

// Function: sub_13480 @ 0x13480
bool sub_13480(long a0,long a1)
{
  return sub_17f00(a0 + 9,a1 + 9) == 0;
}

// Function: sub_134a0 @ 0x134a0
unsigned long sub_134a0(long a0,unsigned long a1,unsigned char a2)
{
  unsigned char v1;
  
  if (sub_d540((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 8) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 8);
  }
  if (v1 == a2) {
    if (a2)
      return sub_13480(a0,a1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_13500 @ 0x13500
unsigned long sub_13500(long a0,unsigned long a1,unsigned char a2,char a3)
{
  unsigned char v1;
  
  if (sub_d540((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 7) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 7);
  }
  if (v1 == a2) {
    if (a2)
      return sub_134a0(a0,a1,(int)a3); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_13580 @ 0x13580
unsigned long sub_13580(long a0,unsigned long a1,unsigned char a2,char a3,char a4)
{
  unsigned char v1;
  
  if (sub_d540((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 6) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 6);
  }
  if (v1 == a2) {
    if (a2)
      return sub_13500(a0,a1,(int)a3,(int)a4); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_135f0 @ 0x135f0
unsigned long sub_135f0(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5)
{
  unsigned char v1;
  
  if (sub_d540((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 5) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 5);
  }
  if (v1 == a2) {
    if (a2)
      return sub_13580(a0,a1,(int)a3,(int)a4,(int)a5); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_13690 @ 0x13690
unsigned long sub_13690(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6)
{
  unsigned int v1;
  unsigned char v2;
  
  v1 = a6;
  if (sub_d540((int)(char)a2)) // branch-flip
    v2 = *(unsigned char *)(a0 + 4) & 0xdf;
  else {
    v2 = *(unsigned char *)(a0 + 4);
  }
  if (v2 == a2) {
    if (a2)
      return sub_135f0(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_13730 @ 0x13730
unsigned long sub_13730(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,int a6,unsigned int a7)
{
  unsigned int v1;
  unsigned int v2;
  unsigned char v3;
  
  v2 = a7;
  v1 = a6;
  if (sub_d540((int)(char)a2)) // branch-flip
    v3 = *(unsigned char *)(a0 + 3) & 0xdf;
  else {
    v3 = *(unsigned char *)(a0 + 3);
  }
  if (v3 == a2) {
    if (a2) {
      a6 = (int)(char)v2;
      return sub_13690(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_137f0 @ 0x137f0
unsigned long sub_137f0(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,unsigned int a8)
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  unsigned char v4;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_d540((int)(char)a2)) // branch-flip
    v4 = *(unsigned char *)(a0 + 2) & 0xdf;
  else {
    v4 = *(unsigned char *)(a0 + 2);
  }
  if (v4 == a2) {
    if (a2) {
      a7 = (int)(char)v3;
      a6 = (int)(char)v2;
      return sub_13730(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_138b0 @ 0x138b0
unsigned long sub_138b0(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,int a8,unsigned int a9)
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
  if (sub_d540((int)(char)a2)) // branch-flip
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
      return sub_137f0(a0,a1,(int)a3,(int)a4,(int)a5,(int)v6); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_13990 @ 0x13990
unsigned long sub_13990(unsigned char *a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6,unsigned int a7,unsigned int a8) // early-return
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_d540((int)(char)a2)) { // branch-flip
    if (a2 != (*a0 & 0xdf))
      return 0;
  }
  else if (a2 != *a0)
    return 0;
  return sub_138b0(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1,(int)(char)v2,(int)(char)v3,0,0);
}

// Function: sub_13a50 @ 0x13a50
char * sub_13a50(char *a0,int a1) // return-dupe, ternary x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = dcgettext(NULL,a0,5);
  if (a0 == v1) {
    v2 = sub_181f0();
    if (sub_13990(v2,"UTF-8",0x55,0x54,0x46,0x2d,0x38,0,0)) { // branch-flip
      v1 = (*v1 != '`') ? "’" : "‘"; // branch-flip
    }
    else {
      if (!sub_13990(v2,"GB18030",0x47,0x42,0x31,0x38,0x30,0x33,0x30)) {
        if (a1 == 9)
          return "\"";
        return "\'";
      }
      v1 = (*v1 != '`') ? (char *)0x1d608 : (char *)0x1d60d; // branch-flip
    }
  }
  return v1;
}

// Function: sub_13b80 @ 0x13b80
long * sub_13b80(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_13bf0:
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
label_14076:
      v2 = 0;
      goto label_14030;
    case 2:
      if (!v17) {
        v2 = 0;
        goto label_14191;
      }
      v24 = 1;
      v2 = 0;
      v12 = 0;
      v30 = 1;
      v31 = "\'";
      break;
    case 3:
      v2 = 1;
label_14030:
      v24 = 1;
      v12 = 0;
      a4 = 2;
      v30 = 1;
      v31 = "\'";
      break;
    case 4:
      if (!v17) {
        v2 = 1;
        goto label_14191;
      }
      goto label_14076;
    case 5:
      if (!v17) goto label_13fd1;
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
        v9 = (char *)sub_13a50("`",a4);
        v36 = (char *)v9;
        v9 = (char *)sub_13a50("\'",a4);
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
label_13cb3:
  do {
label_13cc6:
    v23 = 0;
    a1 = v13;
label_13cd0:
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
          if ((a3 < v23 + v30) || (v8 = memcmp(v11,v31,v30), v8)) goto label_14640;
          if (!v24) {
            v7 = *v11;
            v18 = (unsigned long)v7;
            v21 = v18;
            if ((char)v7 <= '?') {
              switch(v7) {
                case 0:
                  goto label_1423c;
                default:
                  goto label_13f13;
                case 7:
                  goto label_14200;
                case 8:
                  goto label_141e5;
                case 9:
                  goto label_142c3;
                case 10:
                  goto label_141db;
                case 0xb:
                  goto label_142f0;
                case 0xc:
                  goto label_14227;
                case 0xd:
                  goto label_1412d;
                case 0x20:
                  goto label_142fd;
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
                  goto label_13ec6;
                case 0x23:
                  goto label_142a0;
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
                  goto label_13df8;
                case 0x27:
                  goto label_14112;
                case 0x3f:
                  goto label_140d1;
                
              }
            }
            if ('{' > (char)v7) {
              if (v7 == 0x40) goto label_13f13;
              v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
              if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_13df8;
              if (!((unsigned long)v9 & 0xa4000000)) goto label_146e4;
              goto label_14628;
            }
            if (v7 == 0x7d) goto label_147b0;
            if ((char)v7 <= '}') {
              v21 = 0x7b;
              if (v7 == 0x7b) goto label_1460a;
              v21 = 0x7c;
              if (v7 == 0x7c) goto label_14628;
              goto label_13f13;
            }
            if (v7 != 0x7e) goto label_13f13;
label_14764:
            if (!v23) {
              v7 = a4 == 2;
              v21 = 0x7e;
              v26 = v3;
              goto label_13f89;
            }
            v21 = 0x7e;
            goto label_14772;
          }
          goto label_13efc;
        }
label_14640:
        v7 = *v11;
        v18 = (unsigned long)v7;
        v21 = v18;
        if ('?' < (char)v7) {
          if ('{' <= (char)v7) { // branch-flip
            if (v7 != 0x7d) { // branch-flip
              if ('~' <= (char)v7) {
                v22 = 0;
                if (v7 == 0x7e) goto label_14764;
                goto label_13f10;
              }
              v21 = 0x7b;
              v25 = 0;
              if (v7 != 0x7b) {
                v21 = 0x7c;
                v25 = 0;
                v22 = 0;
                if (v7 == 0x7c) goto label_14628;
                goto label_13f13;
              }
            }
            else {
              v21 = 0x7d;
              v25 = 0;
            }
label_1460a:
            if (a3 == 0xffffffffffffffff) goto label_147bf;
label_14618:
            v22 = v25;
            if (a3 == 1) goto label_142a5;
          }
          else {
            if (v7 == 0x40) goto label_13f10;
            v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
            v22 = 0;
            if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_13df8;
            v25 = 0;
            v22 = 0;
            if (!((unsigned long)v9 & 0xa4000000)) {
label_146e4:
              if ((char)v18 != '\\') goto label_13f13;
              if (((bool)(v2 & v24)) && (v30)) goto label_1470a;
              v16 = 0x5c;
              goto label_14132;
            }
          }
label_14628:
          v7 = a4 == 2;
          v26 = v3;
          v25 = 0;
          goto label_13f89;
        }
        switch(v7) {
          case 0:
            goto label_1422e;
          default:
label_13f10:
            v25 = 0;
label_13f13:
            v16 = (unsigned char)v18;
            v22 = v25;
            if (v33 != 1) goto label_14493;
label_13f20:
            v9 = __ctype_b_loc();
            v21 = 1;
            v26 = (*(unsigned char *)(*v9 + 1 + v18 * 2) & 0x40) == 0;
            v27 = (bool)(v26 & v2);
            v26 = !v26;
            v22 = v25;
            goto label_13f6e;
          case 7:
label_14200:
            v16 = 0x61;
            if (!v24) goto label_141f8;
            goto label_1415f;
          case 8:
label_141e5:
            v16 = 0x62;
            goto label_141ea;
          case 9:
            v25 = 0;
label_142c3:
            v21 = 9;
            v16 = 0x74;
            goto label_142d0;
          case 10:
label_141db:
            v16 = 0x6e;
            break;
          case 0xb:
label_142f0:
            v16 = 0x76;
            break;
          case 0xc:
label_14227:
            v16 = 0x66;
label_141ea:
            if (v24) goto label_1415f;
label_141f8:
            v25 = 0;
            goto label_13e7e;
          case 0xd:
label_1412d:
            v16 = 0x72;
            break;
          case 0x20:
            v22 = 0;
label_142fd:
            v21 = 0x20;
            goto label_13df8;
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
            goto label_13ec9;
          case 0x23:
            v21 = 0x23;
            v25 = 0;
            goto label_142a5;
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
label_13df8:
            v7 = 0;
            goto label_13e08;
          case 0x27:
            v22 = 0;
            goto label_14112;
          case 0x3f:
            v22 = 0;
            goto label_140d1;
          
        }
        goto label_14132;
      }
      v16 = *v11;
      v18 = (unsigned long)v16;
      if ('?' < (char)v16) {
        if ('{' <= (char)v16) {
          if (v16 == 0x7d) {
label_147b0:
            v21 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_14618;
label_147bf:
            v22 = v25;
            if (a2[1]) goto label_14628;
            goto label_142a5;
          }
          if ((char)v16 <= '}') {
            v21 = 0x7b;
            if (v16 != 0x7b) {
              if (v16 == 0x7c) goto label_13ec6;
              goto label_13f13;
            }
            goto label_1460a;
          }
          v21 = 0x7e;
          if (v16 == 0x7e) goto label_142a5;
          if (v33 == 1) goto label_13f20;
label_14493:
          v14 = 0;
          if (a3 == 0xffffffffffffffff) {
            v9 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v9;
          }
          v21 = 0;
          do {
            v18 = v23 + v21;
            v9 = (long)sub_10c80(&v15,&a2[v18],a3 - v18,&v14);
            v6 = (long)v9;
            v25 = v26;
            if (!v9) break;
            if (v9 == (long *)0xffffffffffffffff) {
              v25 = 0;
              goto label_149fb;
            }
            if (v9 == (long *)0xfffffffffffffffe) {
              v9 = (long *)v18;
              if (a3 > v18) goto label_14d75;
              goto label_14d7f;
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
                if ((v7 <= 0x21) && (0x20000002bU >> (v18 & 0x3f) & 1)) goto label_145cd;
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
          goto label_149fb;
        }
        if (v16 == 0x40) goto label_13f13;
        v9 = (unsigned long)(1L << (v16 + 0xbf & 0x3f));
        if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_13f80;
        if ((unsigned long)v9 & 0xa4000000) goto label_13ec6;
        if (v16 != 0x5c) goto label_13f13;
        if (a4 == 2) {
          if (!v24) goto label_1470a;
          goto label_13ef3;
        }
        if (!(bool)(v2 & v24 & v30 != 0)) {
          v21 = 0x5c;
          v16 = 0x5c;
          v25 = 0;
          goto label_142d0;
        }
label_1470a:
        v23 += 1;
        v25 = 0;
        v16 = 0x5c;
        v7 = v20;
        goto label_14720;
      }
      switch(v16) {
        case 0:
          if (!v2) {
            v22 = 0;
            v21 = 0;
            if (v34 & 1) goto label_143a2;
            goto label_142db;
          }
label_1422e:
          if (!v24) {
            v22 = 0;
label_1423c:
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
                goto label_13fb3;
              }
label_14b45:
              a0[v13] = 0x5c;
              v12 = v13;
              v20 = (bool)v7;
            }
            else if (v12 < a1) {
              v7 = v20;
              goto label_14b45;
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
label_14b7c:
              v16 = (unsigned char)v21;
              v25 = 0;
              v12 = v13;
              v7 = v26;
              if (!v2 || v27) goto label_13fb3;
              goto label_13e08;
            }
            v16 = 0x30;
            v25 = 0;
            v12 = v13;
            v7 = v26;
            goto label_13fb3;
          }
          if (a4 != 2) goto label_13efc;
          goto label_1416b;
        default:
          goto label_13f13;
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
          goto label_1435f;
        case 10:
          v21 = 10;
          v16 = 0x6e;
          goto label_1435f;
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
label_1435f:
          v7 = a4 == 2 & v24;
          v25 = 0;
          if (!(bool)v7) goto label_142d0;
label_13ee1:
          a4 = 2;
label_13ef3:
          if (!v28) goto label_13efc;
          goto label_1416b;
        case 0x20:
          v18 = 0x20;
          goto label_13ec9;
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
label_13ec6:
          v26 = 0;
          goto label_13ec9;
        case 0x23:
label_142a0:
          v21 = 0x23;
label_142a5:
          v22 = v25;
          if (v23) {
label_14772:
            v7 = a4 == 2;
            v26 = v3;
            v25 = 0;
            goto label_13f89;
          }
          v18 = v21;
label_13ec9:
          v21 = v18;
          v22 = v25;
          v25 = v26;
          if ((a4 == 2) && (v24)) goto label_13ee1;
          goto label_13f87;
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
          goto label_13f80;
        case 0x27:
label_14112:
          v25 = v26;
          if (a4 != 2) {
            v7 = 0;
            v21 = 0x27;
            goto label_13f89;
          }
          if (!v24) {
            if (a1) { // branch-flip
              v13 = 0;
              v18 = a1;
              if (v32) goto label_14b9b;
            }
            else {
label_14b9b:
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
            goto label_13fb3;
          }
          goto label_13ef3;
        case 0x3f:
label_140d1:
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
                  goto label_13f89;
                }
                if (v24) goto label_13efc;
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
                goto label_14b7c;
              }
            }
            v7 = 0;
            v21 = 0x3f;
            v26 = v3;
            v25 = 0;
            goto label_13f89;
          }
          if (v24) goto label_13ef3;
          v7 = 0;
          v16 = 0x3f;
          v25 = 0;
          goto label_13fb3;
        
      }
label_142d0:
      v22 = v25;
      if (!v2) {
label_142db:
        v16 = (unsigned char)v21;
        v25 = 0;
        v7 = 0;
        v26 = v3;
        if (!v24) goto label_13fb0;
        goto label_13e08;
      }
label_14132:
      v25 = 0;
      goto label_14143;
    }
    if (!(bool)(v12 == 0 & v24 & a4 == 2)) {
      v7 = a4 == 2 & (v24 ^ 1U);
      v24 = (bool)(v24 ^ 1U);
      if ((!(bool)v7) || (v24 = (bool)v7, !v3)) {
label_14dff:
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
        if (a1 || !v32) goto label_14dff;
        v23 = v32;
        goto label_148cb;
      }
      v12 = 0;
      a4 = 5;
      v9 = (unsigned long)__ctype_get_mb_cur_max();
      v30 = 1;
      v33 = (unsigned long)v9;
      v31 = "\"";
      if (!(v34 & 2)) goto label_14eaf;
      v3 = 0;
      v20 = 0;
      v32 = 0;
      v2 = v4;
      v24 = v4;
      goto label_13cc6;
    }
label_145cd:
    if (v2) {
label_1416b:
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
label_14191:
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
label_148cb:
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
label_14d75:
    if (!a2[(long)v9]) break;
  }
label_14d7f:
  v25 = 0;
label_149fb:
  v18 = (unsigned long)v16;
  v26 = v25;
  if (2 <= v21) {
label_14a05:
    v19 = 0;
    v21 += v23;
    v13 = v23;
    do {
      v16 = (unsigned char)v18;
      if (v27) {
        v28 = a4 == 2;
        if (v24) goto label_13ef3;
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
        if (v21 <= v23) goto label_13e91;
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
          goto label_14720;
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
label_13f6e:
  if (v27) {
    v25 = 0;
    v27 = v2;
    goto label_14a05;
  }
label_13f80:
  v21 = v18;
  v25 = v26;
label_13f87:
  v7 = a4 == 2;
  v26 = v3;
label_13f89:
  v16 = (unsigned char)v21;
  v27 = (bool)v7 == 0;
  v3 = v26;
  v7 = 0;
  if ((v27 && v2) || (v7 = 0, v24)) {
label_13e08:
    v16 = (unsigned char)v21;
    v12 = v13;
    if (!v29) goto label_13fb3;
    if (!(*(unsigned int *)(v29 + (v21 >> 5) * 4) >> (v16 & 0x1f) & 1)) goto label_13fb3;
  }
  else {
label_13fb0:
    v7 = 0;
    v3 = v26;
label_13fb3:
    if (!v22) {
      v7 ^= 1;
      v23 += 1;
      v7 &= v20;
label_14720:
      if ((bool)v7) {
        if (v12 < a1)
          a0[v12] = 0x27;
        if (v12 + 1 < a1)
          a0[v12 + 1] = 0x27;
        v20 = 0;
        v12 += 2;
      }
      goto label_13e91;
    }
  }
label_14143:
  if (v24) {
    v24 = v2;
label_1415f:
    if ((bool)(v24 & a4 == 2)) goto label_1416b;
label_13efc:
    v34 &= 0xfffffffd;
    v29 = 0;
    goto label_13bf0;
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
label_13e7e:
  if (v12 < a1)
    a0[v12] = 0x5c;
  v12 += 1;
  v23 += 1;
label_13e91:
  if (v12 < a1)
    a0[v12] = v16;
  v12 += 1;
  if (!v25)
    v4 = 0;
  goto label_13cd0;
label_143a2:
  v23 += 1;
  goto label_13cd0;
label_14eaf:
  a1 = v32;
label_13fd1:
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
  goto label_13cb3;
}

// Function: sub_14fb0 @ 0x14fb0
void * sub_14fb0(unsigned int a0,unsigned long a1,unsigned long a2,unsigned int *a3)
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
    v6 = dat_26260;
    if (dat_26258 <= (int)a0) {
      v10 = (long)dat_26258;
      v9 = (long)(int)((a0 - dat_26258) + 1);
      if (dat_26260 != (unsigned long *)0x26270) // branch-flip
        v6 = (unsigned long *)sub_16ba0(dat_26260,&v10,v9,0x7fffffff,0x10);
      else {
        v6 = (unsigned long *)sub_16ba0(0,&v10,v9,0x7fffffff,0x10);
        *v6 = dat_26270;
        v6[1] = dat_26278;
      }
      dat_26260 = v6;
      memset(&v6[(long)dat_26258 * 2],0,(v10 - dat_26258) * 0x10);
      dat_26258 = (int)v10;
    }
    v1 = &v6[(long)(int)a0 * 2];
    v3 = a3[1];
    v5 = *v1;
    v8 = (void *)v1[1];
    v7 = sub_13b80(v8,v5,a1,a2,*a3,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    if (v5 <= v7) {
      v7 += 1;
      *v1 = v7;
      if (v8 != (void *)0x27420)
        free(v8);
      v8 = (void *)sub_16a20(v7);
      v4 = *a3;
      v1[1] = (unsigned long)v8;
      sub_13b80(v8,v7,a1,a2,v4,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    }
    *v11 = v2;
    return v8;
  }
  abort(); // no-return
}

// Function: sub_151a0 @ 0x151a0
void sub_151a0(long a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = 0x27520;
  sub_16ce0(a0,0x38);
  *v2 = v1;
}

// Function: sub_151e0 @ 0x151e0
unsigned int sub_151e0(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x27520;
  return *a0;
}

// Function: sub_15200 @ 0x15200
void sub_15200(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x27520;
  *a0 = a1;
}

// Function: sub_15220 @ 0x15220
unsigned int sub_15220(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0x27520;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_15260 @ 0x15260
unsigned int sub_15260(long a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = 0x27520;
  v1 = *(unsigned int *)(a0 + 4);
  *(unsigned int *)(a0 + 4) = a1;
  return v1;
}

// Function: sub_15280 @ 0x15280
void sub_15280(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0x27520;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_152c0 @ 0x152c0
void sub_152c0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x27520;
  v2 = __errno_location();
  v1 = *v2;
  sub_13b80(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(unsigned long *)&a4[10],*(unsigned long *)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_15340 @ 0x15340
unsigned long sub_15340(unsigned long a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x27520;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_13b80(0,0,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  v4 = sub_16a20(v3 + 1);
  sub_13b80(v4,v3 + 1,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_15430 @ 0x15430
void sub_15430(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_15340(a0,a1,0,a2); // tail-call
}

// Function: sub_15440 @ 0x15440
void sub_15440(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_26260;
  if (2 <= dat_26258) {
    v4 = (unsigned long *)((long)dat_26260 + 0x18);
    v1 = (unsigned long *)((long)dat_26260 + (unsigned long)(unsigned int)(dat_26258 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0x27420) {
    free(*(void **)((long)v3 + 8));
    dat_26278 = 0x27420;
    dat_26270 = 0x100;
  }
  if (v3 == (void *)0x26270) {
    dat_26258 = 1;
    return;
  }
  free(v3);
  dat_26258 = 1;
  dat_26260 = (void *)0x26270;
}

// Function: sub_154e0 @ 0x154e0
void sub_154e0(unsigned long a0,unsigned long a1)
{
  sub_14fb0(a0,a1,0xffffffffffffffff,0x27520); // tail-call
}

// Function: sub_15500 @ 0x15500
void sub_15500(void)
{
  sub_14fb0(); // tail-call
}

// Function: sub_15510 @ 0x15510
void sub_15510(unsigned long a0)
{
  sub_154e0(0,a0); // tail-call
}

// Function: sub_15520 @ 0x15520
void sub_15520(unsigned long a0,unsigned long a1)
{
  sub_15500(0,a0,a1); // tail-call
}

// Function: sub_15530 @ 0x15530
void sub_15530(unsigned int a0,unsigned long a1,unsigned long a2)
{
  char v1 [56];
  
  sub_13430(v1);
  sub_14fb0(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_15590 @ 0x15590
void sub_15590(unsigned int a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  char v1 [56];
  
  sub_13430(v1);
  sub_14fb0(a0,a2,a3,v1);
}

// Function: sub_15600 @ 0x15600
void sub_15600(unsigned int a0,unsigned long a1)
{
  sub_15530(0,a0,a1); // tail-call
}

// Function: sub_15610 @ 0x15610
void sub_15610(unsigned int a0,unsigned long a1,unsigned long a2)
{
  sub_15590(0,a0,a1,a2); // tail-call
}

// Function: sub_15630 @ 0x15630
void sub_15630(unsigned long a0,unsigned long a1,char a2,unsigned long a3)
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
  v1 = dat_27520;
  v3 = dat_27528;
  v4 = dat_27530;
  v5 = dat_27538;
  v8 = dat_27550;
  v6 = dat_27540;
  v7 = dat_27548;
  sub_15220(&v1,(int)a2,1,a3,a0,a1);
  sub_14fb0(0,a0,a1,v2);
}

// Function: sub_156c0 @ 0x156c0
void sub_156c0(unsigned long a0,char a1)
{
  sub_15630(a0,0xffffffffffffffff,(int)a1); // tail-call
}

// Function: sub_156e0 @ 0x156e0
void sub_156e0(unsigned long a0)
{
  sub_156c0(a0,0x3a); // tail-call
}

// Function: sub_156f0 @ 0x156f0
void sub_156f0(unsigned long a0,unsigned long a1)
{
  sub_15630(a0,a1,0x3a); // tail-call
}

// Function: sub_15700 @ 0x15700
void sub_15700(unsigned int a0,unsigned long a1,unsigned long a2)
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
  
  sub_13430(&v1);
  v3 = &v2;
  v2 = v1;
  v10 = v4;
  v15 = v9;
  v11 = v5;
  v12 = v6;
  v13 = v7;
  v14 = v8;
  sub_15220(v3,0x3a,1);
  sub_14fb0(a0,a2,0xffffffffffffffff,v3);
}

// Function: sub_157a0 @ 0x157a0
void sub_157a0(unsigned int a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x68
  unsigned long v2; // stack - 0x60
  unsigned long v3; // stack - 0x58
  unsigned long v4; // stack - 0x50
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  
  v1 = dat_27520;
  v2 = dat_27528;
  v7 = dat_27550;
  v3 = dat_27530;
  v4 = dat_27538;
  v5 = dat_27540;
  v6 = dat_27548;
  sub_15280(&v1);
  sub_14fb0(a0,a3,a4,&v1);
}

// Function: sub_15840 @ 0x15840
void sub_15840(void)
{
  sub_157a0(); // tail-call
}

// Function: sub_15850 @ 0x15850
void sub_15850(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_15840(0,a0,a1,a2); // tail-call
}

// Function: sub_15870 @ 0x15870
void sub_15870(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  sub_157a0(0,a0,a1,a2,a3); // tail-call
}

// Function: sub_15890 @ 0x15890
void sub_15890(void)
{
  sub_14fb0(); // tail-call
}

// Function: sub_158a0 @ 0x158a0
void sub_158a0(unsigned long a0,unsigned long a1)
{
  sub_15890(0,a0,a1); // tail-call
}

// Function: sub_158b0 @ 0x158b0
void sub_158b0(unsigned long a0,unsigned long a1)
{
  sub_15890(a0,a1,0xffffffffffffffff); // tail-call
}

// Function: sub_158c0 @ 0x158c0
void sub_158c0(unsigned long a0)
{
  sub_158b0(0,a0); // tail-call
}

// Function: sub_158d0 @ 0x158d0
unsigned long * sub_158d0(unsigned long *a0,unsigned long a1) // early-return
{
  unsigned long *v1;
  unsigned long *v2;
  void *v3; // rax
  
  v1 = (unsigned long *)*a0;
  if (v1 != &a0[2]) {
    v2 = realloc(v1,a1);
    if (!v2)
      v2 = v1;
    return v2;
  }
  v3 = malloc(a1);
  if (!v3)
    return NULL;
  return memcpy(v3,v1,a1); // tail-call
}

// Function: sub_15930 @ 0x15930
void sub_15930(long *a0)
{
  a0[1] = 0x400;
  *a0 = (long)&a0[2];
}

// Function: sub_15940 @ 0x15940
void sub_15940(unsigned long *a0)
{
  if ((unsigned long *)*a0 != &a0[2]) {
    free((unsigned long *)*a0); // tail-call
    return;
  }
}

// Function: sub_15960 @ 0x15960
unsigned long sub_15960(unsigned long *a0)
{
  unsigned long v1;
  void *v2; // rax
  
  v1 = a0[1] * 2;
  sub_15940(a0);
  if ((unsigned long)a0[1] <= v1) { // branch-flip
    v2 = malloc(v1);
    if (v2) {
      *a0 = v2;
      a0[1] = v1;
      return 1;
    }
  }
  else {
    *__errno_location() = 0xc;
  }
  sub_15930(a0);
  return 0;
}

// Function: sub_159d0 @ 0x159d0
void sub_159d0(long *a0)
{
  a0[1] = 0x400;
  *a0 = (long)&a0[2];
}

// Function: sub_159e0 @ 0x159e0
unsigned long sub_159e0(unsigned long *a0) // return-dupe
{
  unsigned long v1;
  void *v2; // rax
  unsigned long *v3;
  
  v3 = (unsigned long *)*a0;
  v1 = a0[1] * 2;
  if (v3 != &a0[2]) {
    if ((unsigned long)a0[1] <= v1) { // branch-flip
      v2 = realloc(v3,v1);
      if (v2) goto label_15a17;
      v3 = (void *)*a0;
    }
    else {
      *__errno_location() = 0xc;
    }
    free(v3);
    sub_159d0(a0);
    return 0;
  }
  v2 = malloc(v1);
  if (!v2)
    return 0;
  v2 = memcpy(v2,v3,a0[1]);
label_15a17:
  *a0 = v2;
  a0[1] = v1;
  return 1;
}

// Function: sub_15a90 @ 0x15a90
int sub_15a90(int a0,unsigned long *a1)
{
  char *v1;
  
  if (a0) { // branch-flip
    if (a0 == 10) {
      v1 = (char *)*a1;
      if (!strcmp(v1,"unlabeled")) {
        a0 = -1;
        freecon(v1);
        *a1 = 0;
        *__errno_location() = 0x3d;
      }
    }
  }
  else {
    a0 = -1;
    *__errno_location() = 0x5f;
  }
  return a0;
}

// Function: sub_15b10 @ 0x15b10
void sub_15b10(unsigned long a0,unsigned long a1)
{
  sub_15a90(getfilecon(),a1); // tail-call
}

// Function: sub_15b30 @ 0x15b30
void sub_15b30(unsigned long a0,unsigned long a1)
{
  sub_15a90(lgetfilecon(),a1); // tail-call
}

// Function: sub_15b50 @ 0x15b50
void sub_15b50(unsigned long a0,unsigned long a1)
{
  sub_15a90(fgetfilecon(),a1); // tail-call
}

// Function: sub_15b70 @ 0x15b70
void sub_15b70(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_15b80 @ 0x15b80
unsigned long sub_15b80(unsigned long a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_15b70();
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

// Function: sub_15c30 @ 0x15c30
void sub_15c30(void)
{
  sub_15b80(); // tail-call
}

// Function: sub_15c40 @ 0x15c40
void sub_15c40(void)
{
  sub_15b70(); // tail-call
}

// Function: sub_15c50 @ 0x15c50
unsigned long sub_15c50(long a0)
{
  return *(unsigned long *)(a0 + 0x50);
}

// Function: sub_15c60 @ 0x15c60
unsigned long sub_15c60(long a0)
{
  return *(unsigned long *)(a0 + 0x70);
}

// Function: sub_15c70 @ 0x15c70
unsigned long sub_15c70(long a0)
{
  return *(unsigned long *)(a0 + 0x60);
}

// Function: sub_15c80 @ 0x15c80
unsigned long sub_15c80(void)
{
  return 0;
}

// Function: sub_15c90 @ 0x15c90
undefined16 sub_15c90(long a0)
{
  return *(char (*)[16])(a0 + 0x48);
}

// Function: sub_15ca0 @ 0x15ca0
undefined16 sub_15ca0(long a0)
{
  return *(char (*)[16])(a0 + 0x68);
}

// Function: sub_15cb0 @ 0x15cb0
undefined16 sub_15cb0(long a0)
{
  return *(char (*)[16])(a0 + 0x58);
}

// Function: sub_15cc0 @ 0x15cc0
undefined16 sub_15cc0(void)
{
  char v1 [16];
  
  v1._8_8_ = 0xffffffffffffffff;
  v1._0_8_ = 0xffffffffffffffff;
  return v1._0_16_;
}

// Function: sub_15ce0 @ 0x15ce0
unsigned int sub_15ce0(unsigned int a0)
{
  return a0;
}

// Function: sub_15cf0 @ 0x15cf0
void sub_15cf0(void *a0,void *a1,unsigned long a2)
{
  *(char *)((long)memcpy(a0,a1,a2) + a2) = 0;
}

// Function: sub_15d00 @ 0x15d00
void sub_15d00(void)
{
  getenv("TZ"); // tail-call
}

// Function: sub_15d10 @ 0x15d10
void sub_15d10(char *a0)
{
  if (a0) {
    setenv("TZ",a0,1); // tail-call
    return;
  }
  unsetenv("TZ"); // tail-call
}

// Function: sub_15d40 @ 0x15d40
bool sub_15d40(long a0)
{
  long v1;
  bool v2; // zf
  
  v1 = a0 + 9;
  if (!*(char *)(a0 + 8))
    v1 = 0;
  v2 = sub_15d10(v1) == 0;
  if (v2)
    tzset();
  return v2;
}

// Function: sub_15d80 @ 0x15d80
unsigned long * sub_15d80(char *a0)
{
  unsigned long v1;
  unsigned long v2;
  unsigned long *v3;
  unsigned long *v4; // rax
  
  if (a0) { // branch-flip
    v1 = strlen(a0) + 1;
    v2 = 0x76;
    if (0x76 <= v1)
      v2 = v1;
    v3 = malloc(v2 + 0x11 & 0xfffffffffffffff8);
    if (v3) {
      *v3 = 0;
      *(unsigned short *)&v3[1] = 1;
      sub_15cf0((long)v3 + 9,a0,v1);
    }
  }
  else {
    v4 = malloc(0x80);
    v3 = NULL;
    if (v4) {
      *v4 = 0;
      *(unsigned short *)&v4[1] = 0;
      return v4;
    }
  }
  return v3;
}

// Function: sub_15e20 @ 0x15e20
unsigned long sub_15e20(long *a0,char *a1) // return-dupe x2
{
  char *v1;
  long *v2;
  int v3; // eax
  long v4; // rax
  char *v5;
  
  v1 = *(char **)&a1[0x30];
  if (!v1)
    return 1;
  if ((a1 <= v1) && (v1 < &a1[0x38]))
    return 1;
  v5 = (char *)((long)a0 + 9);
  if (!*v1) {
    v5 = "";
label_15eb8:
    *(char **)&a1[0x30] = v5;
    return 1;
  }
  do {
    v3 = strcmp(v5,v1);
    while( true ) {
      if (!v3) goto label_15eb8;
      if ((!*v5) && ((v5 != (char *)((long)a0 + 9) || (!(char)a0[1])))) {
        if ((long)a0 + (0x80U - (long)v5) <= (long)(strlen(v1) + 1)) { // branch-flip
          v4 = sub_15d80(v1);
          *a0 = v4;
          if (!v4)
            return 0;
          *(char *)(v4 + 8) = 0;
          v5 = (char *)(v4 + 9);
        }
        else {
          sub_15cf0(v5,v1);
        }
        goto label_15eb8;
      }
      v5 = &v5[strlen(v5) + 1];
      if ((*v5) || (v2 = (long *)*a0, !v2)) break;
      v5 = (char *)((long)v2 + 9);
      v3 = strcmp(v5,v1);
      a0 = v2;
    }
  } while( true );
}

// Function: sub_15f50 @ 0x15f50
void sub_15f50(long *a0) // return-dupe
{
  long *v1;
  
  if (a0 == (long *)0x1)
    return;
  if (!a0)
    return;
  do {
    v1 = (long *)*a0;
    free(a0);
    a0 = v1;
  } while (v1);
}

// Function: sub_15f90 @ 0x15f90
long sub_15f90(long a0)
{
  int v1;
  char *v2; // rax
  long v3; // rax
  int *v4; // rax
  long v5;
  
  v2 = (char *)sub_15d00();
  if (v2) { // branch-flip
    if (*(char *)(a0 + 8)) {
      if (!strcmp((char *)(a0 + 9),v2))
        return 1;
    }
  }
  else if (!*(char *)(a0 + 8))
    return 1;
  v3 = sub_15d80(v2);
  v5 = v3;
  if (v3) {
    if (!sub_15d40(a0)) {
      v4 = __errno_location();
      v5 = 0;
      v1 = *v4;
      sub_15f50(v3);
      *v4 = v1;
    }
  }
  return v5;
}

// Function: sub_16040 @ 0x16040
unsigned int sub_16040(long a0)
{
  unsigned int v1; // eax
  int *v2; // rax
  int v3;
  
  v1 = 1;
  if (a0 != 1) {
    v2 = __errno_location();
    v3 = *v2;
    v1 = sub_15d40(a0);
    if (!(char)v1)
      v3 = *v2;
    sub_15f50(a0);
    *v2 = v3;
  }
  return v1;
}

// Function: sub_160a0 @ 0x160a0
void * sub_160a0(long a0,void *a1,void *a2) // return-dupe
{
  long v1; // rax
  
  if (!a0)
    return (void *)gmtime_r(a1,a2); // tail-call
  v1 = sub_15f90(a0);
  if (!v1)
    return NULL;
  if ((localtime_r(a1,a2)) && (sub_15e20(a0,a2))) {
    if (!sub_16040(v1))
      return NULL;
    return a2;
  }
  sub_16040(v1);
  return NULL;
}

// Function: sub_16140 @ 0x16140
unsigned long sub_16140(long a0,unsigned long *a1) // return-dupe x2
{
  long v1; // rax
  unsigned long v10; // stack - 0x40
  unsigned long v11; // stack - 0x38
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x68
  unsigned long v4; // stack - 0x60
  unsigned long v5; // stack - 0x58
  unsigned int v6; // stack - 0x50
  int v7; // stack - 0x4c
  unsigned int v8; // stack - 0x48
  unsigned int v9; // stack - 0x44
  
  if (!a0)
    return sub_16270(a1); // tail-call
  v1 = sub_15f90(a0);
  if (!v1)
    return 0xffffffffffffffff;
  v3 = *a1;
  v7 = -1;
  v4 = a1[1];
  v5 = a1[2];
  v8 = *(unsigned int *)&a1[4];
  v2 = sub_17e30(&v3);
  if ((0 <= v7) && (sub_15e20(a0,&v3))) {
    if (!sub_16040(v1))
      return 0xffffffffffffffff;
    *a1 = v3;
    a1[1] = v4;
    a1[6] = v11;
    a1[2] = v5;
    a1[3] = CONCAT44(v7,v6);
    a1[4] = CONCAT44(v9,v8);
    a1[5] = v10;
    return v2;
  }
  sub_16040(v1);
  return 0xffffffffffffffff;
}

// Function: sub_16270 @ 0x16270
void sub_16270(long a0)
{
  *(unsigned int *)(a0 + 0x20) = 0;
  sub_179a0(a0,dat_25fb8,0x27558); // tail-call
}

// Function: sub_16290 @ 0x16290
undefined16 sub_16290(unsigned long a0,unsigned long a1)
{
  char v1 [16];
  
  v1._8_8_ = a1;
  v1._0_8_ = a0;
  return v1._0_16_;
}

// Function: sub_162a0 @ 0x162a0
int sub_162a0(long a0,long a1,long a2,long a3)
{
  return ((unsigned int)(a3 < a1) - (unsigned int)(a1 < a3)) + ((unsigned int)(a2 < a0) - (unsigned int)(a0 < a2)) * 2;
}

// Function: sub_162d0 @ 0x162d0
int sub_162d0(long a0,long a1,unsigned long a2)
{
  return ((unsigned int)(a0 == 0) & (unsigned int)CONCAT71((undefined7)((unsigned long)a2 >> 8),a1 != 0)) + (unsigned int)(0 < a0) + (int)(a0 >> 0x3f);
}

// Function: sub_16300 @ 0x16300
double sub_16300(long a0,long a1)
{
  return (double)a1 / dat_1d9e8 + (double)a0;
}

// Function: sub_16330 @ 0x16330
long sub_16330(void *a0,long a1,unsigned long a2,unsigned long a3,unsigned long *a4,unsigned long a5)
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
label_1645d:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_16496:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0x1dcf0) + 0x1dcf0;
  switch(a5) {
    case 0:
      goto label_16496;
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
      goto label_1645d;
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

// Function: sub_167b0 @ 0x167b0
void sub_167b0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_16330(); // tail-call
}

// Function: sub_167d0 @ 0x167d0
void sub_167d0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
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
  sub_16330(a0,a1,a2,a3);
}

// Function: sub_16870 @ 0x16870
void sub_16870(void)
{
  sub_167d0();
}

// Function: sub_16920 @ 0x16920
void sub_16920(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-coreutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_169d0 @ 0x169d0
long sub_169d0(long a0)
{
  if (a0)
    return a0;
  sub_16da0(); // no-return
}

// Function: sub_169e0 @ 0x169e0
void sub_169e0(unsigned long a0)
{
  sub_169d0(malloc(a0)); // tail-call
}

// Function: sub_16a00 @ 0x16a00
void sub_16a00(void)
{
  sub_169d0(sub_18180()); // tail-call
}

// Function: sub_16a20 @ 0x16a20
void sub_16a20(void)
{
  sub_169e0(); // tail-call
}

// Function: sub_16a30 @ 0x16a30
void sub_16a30(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_16da0(); // no-return
}

// Function: sub_16a70 @ 0x16a70
void sub_16a70(void)
{
  sub_169d0(sub_18190()); // tail-call
}

// Function: sub_16a90 @ 0x16a90
void sub_16a90(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_16da0(); // no-return
}

// Function: sub_16ad0 @ 0x16ad0
void sub_16ad0(void)
{
  sub_16a90(); // tail-call
}

// Function: sub_16ae0 @ 0x16ae0
void sub_16ae0(void)
{
  sub_169d0(sub_181c0()); // tail-call
}

// Function: sub_16b00 @ 0x16b00
void sub_16b00(unsigned long a0,unsigned long a1)
{
  sub_16a90(0,a0,a1); // tail-call
}

// Function: sub_16b10 @ 0x16b10
void sub_16b10(unsigned long a0,unsigned long a1)
{
  sub_16ae0(0,a0,a1); // tail-call
}

// Function: sub_16b20 @ 0x16b20
void sub_16b20(long a0,unsigned long *a1,unsigned long a2)
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
      sub_16da0(); // no-return
  }
  else if (!v3) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v3 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0) + (unsigned long)(0x81 <= a2);
  }
  sub_16a90(a0,v3,a2);
  *a1 = v3;
}

// Function: sub_16b90 @ 0x16b90
void sub_16b90(unsigned long a0,unsigned long a1)
{
  sub_16b20(a0,a1,1); // tail-call
}

// Function: sub_16ba0 @ 0x16ba0
void sub_16ba0(long a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_16c46;
    v3 = 0x80;
  }
  v2 = v3 / a4;
  v3 -= v3 % a4;
label_16c46:
  if (!a0)
    *a1 = 0;
  if ((v2 - v1 < a2) && ((v2 = v1 + a2, SCARRY8(v1,a2) || (((a3 < v2 && (0 <= a3)) || (v3 = v2 * a4, SEXT816(v3) != SEXT816(v2) * SEXT816(a4)))))))
    sub_16da0(a0,v3); // no-return
  sub_16a30(a0,v3);
  *a1 = v2;
}

// Function: sub_16c80 @ 0x16c80
void sub_16c80(unsigned long a0,unsigned long a1)
{
  sub_169d0(calloc(a0,a1)); // tail-call
}

// Function: sub_16ca0 @ 0x16ca0
void sub_16ca0(unsigned long a0)
{
  sub_16c80(a0,1); // tail-call
}

// Function: sub_16cb0 @ 0x16cb0
void sub_16cb0(void)
{
  sub_169d0(sub_181b0()); // tail-call
}

// Function: sub_16cd0 @ 0x16cd0
void sub_16cd0(unsigned long a0)
{
  sub_16cb0(a0,1); // tail-call
}

// Function: sub_16ce0 @ 0x16ce0
void sub_16ce0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_169e0(a1),a0,a1); // tail-call
}

// Function: sub_16d10 @ 0x16d10
void sub_16d10(void *a0,unsigned long a1)
{
  memcpy((void *)sub_16a00(a1),a0,a1); // tail-call
}

// Function: sub_16d40 @ 0x16d40
void sub_16d40(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_16a00(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_16d80 @ 0x16d80
void sub_16d80(char *a0)
{
  sub_16ce0(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_16da0 @ 0x16da0
void sub_16da0(void)
{
  error(dat_261f8,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_16de0 @ 0x16de0
unsigned long sub_16de0(unsigned long a0,unsigned int a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,int a6)
{
  int v1;
  int *v2; // rax
  unsigned long v3; // stack - 0x48
  
  v1 = sub_17190(a0,0,a1,&v3);
  if (v1) { // branch-flip
    v2 = __errno_location();
    if (v1 != 1) {
      if (v1 == 3)
        *v2 = 0;
      goto label_16e88;
    }
  }
  else {
    if ((a2 <= v3) && (v3 <= a3))
      return v3;
    v2 = __errno_location();
    if (v3 <= 0x3fffffff) {
      *v2 = 0x22;
      goto label_16e88;
    }
  }
  *v2 = 0x4b;
label_16e88:
  sub_158c0(a0);
  v1 = *v2;
  if (v1 == 0x16) goto label_16ec0;
  do {
    if (!a6)
      a6 = 1;
    error(a6,v1,"%s: %s",a5);
label_16ec0:
    v1 = 0;
  } while( true );
}

// Function: sub_16ef0 @ 0x16ef0
void sub_16ef0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5)
{
  sub_16de0(a0,10,a1,a2,a3,a4,a5);
}

// Function: sub_16f20 @ 0x16f20
char * sub_16f20(void)
{
  unsigned int *v1; // rax
  char *v2;
  long v3;
  long v4; // rax
  char v5 [104];
  long v6; // stack - 0xa0
  char *v7;
  
  v2 = v5;
  v6 = 100;
  v1 = (unsigned int *)__errno_location();
  v3 = 100;
  v7 = NULL;
  while( true ) {
    v2[v3 + -1] = '\0';
    *v1 = 0;
    if (!gethostname(v2,v3 + -1)) {
      v4 = strlen(v2) + 1;
      if (v4 < v3 + -1) {
        if (v7)
          return v7;
        return (char *)sub_16d10(v2,v4);
      }
      *v1 = 0;
    }
    free(v7);
    if ((0x25 <= *v1) || (0xffffffefffbfeffeU >> ((unsigned long)*v1 & 0x3f) & 1)) break;
    v2 = (char *)sub_16ba0(0,&v6,1,0xffffffffffffffff,1);
    v3 = v6;
    v7 = v2;
  }
  return NULL;
}

// Function: sub_17030 @ 0x17030
void sub_17030(unsigned int a0,int a1,char a2,long a3,unsigned long a4,int a5)
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

// Function: sub_17110 @ 0x17110
void sub_17110(unsigned long a0,unsigned long a1,char a2,unsigned long a3,unsigned long a4)
{
  sub_17030(a0,a1,(int)a2,a3,a4,dat_261f8);
  abort(); // no-return
}

// Function: sub_17130 @ 0x17130
bool sub_17130(unsigned long *a0,int a1)
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

// Function: sub_17160 @ 0x17160
unsigned int sub_17160(unsigned long a0,unsigned long a1,int a2)
{
  unsigned int v1; // r9d
  
  v1 = 0;
  do {
    a2 -= 1;
    v1 |= sub_17130();
  } while (a2);
  return v1;
}

// Function: sub_17190 @ 0x17190
unsigned int sub_17190(unsigned char *a0,unsigned long *a1,unsigned int a2,unsigned long *a3,char *a4)
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
    if (!v5) goto label_17252;
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
label_17259:
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
      v10 |= sub_17130(&v7,0x400);
      break;
    default:
      goto label_17259;
    case 3:
      v10 |= sub_17160(&v7,v9,6);
      break;
    case 5:
    case 0x25:
      v10 |= sub_17160(&v7,v9,3);
      break;
    case 9:
    case 0x29:
      v10 |= sub_17160(&v7,v9,1);
      break;
    case 0xb:
    case 0x2b:
      v10 |= sub_17160(&v7,v9,2);
      break;
    case 0xe:
      v10 |= sub_17160(&v7,v9,5);
      break;
    case 0x12:
    case 0x32:
      v10 |= sub_17160(&v7,v9,4);
      break;
    case 0x17:
      v10 |= sub_17160(&v7,v9,8);
      break;
    case 0x18:
      v10 |= sub_17160(&v7,v9,7);
      break;
    case 0x20:
      v10 |= sub_17130(&v7,0x200);
      break;
    case 0x21:
      break;
    case 0x35:
      v10 |= sub_17130(&v7,2);
    
  }
  *a1 = &v4[v8];
  if (v4[v8])
    v10 |= 2;
label_17252:
  *a3 = v7;
  return v10;
}

// Function: sub_17570 @ 0x17570
long sub_17570(long a0,unsigned char a1)
{
  return a0 >> (a1 & 0x3f);
}

// Function: sub_17580 @ 0x17580
bool sub_17580(unsigned long a0)
{
  bool v1; // r8b
  
  v1 = 0;
  if ((!(a0 & 3)) && (v1 = 1, (a0 * -0x70a3d70a3d70a3d7 + 0x51eb851eb851eb8 >> 2 | a0 * -0x70a3d70a3d70a3d7 << 0x3e) <= 0x28f5c28f5c28f5c))
    return ((unsigned int)((long)a0 / 100) & 3) == 1;
  return v1;
}

// Function: sub_17600 @ 0x17600
unsigned int sub_17600(unsigned int a0,unsigned int a1) // early-return
{
  if ((a0 == 0) == (a1 == 0))
    return 0;
  return ~(a0 | a1) >> 0x1f;
}

// Function: sub_17620 @ 0x17620
long sub_17620(long a0,long a1,unsigned long a2,unsigned long a3,int a4,unsigned int a5,int a6,int a7,int a8,int a9)
{
  unsigned int v1;
  int v2;
  int v3; // eax
  int v4; // edx
  unsigned long v5; // rdx
  long v6; // r8
  long v7; // r11
  long v8; // r12
  int v9; // r15d
  
  v7 = (long)(int)a3;
  v6 = (long)a4;
  v8 = (long)(int)a5;
  sub_17570(a0,2,a2,a3,v6);
  v2 = sub_17570(0x76c);
  a5 &= 3;
  v9 = (v2 + sub_17570(v8)) - (unsigned int)(a5 == 0);
  v1 = (unsigned int)(v5 >> 0x1f) & 1;
  a0 -= v8;
  v2 = v9 >> 0x1f;
  v3 = sub_17570((long)(int)((int)(v1 + (int)v5) / 0x19 - v1));
  v2 = sub_17570((long)((v9 - v2) / 0x19 + v2));
  return (v6 + ((v7 + (((long)(int)a2 + ((long)(v4 + (v3 - v2)) + ((a0 * 0x16d + a1) - (long)a6)) * 0x18) - (long)a7) * 0x3c) - (long)a8) * 0x3c) - (long)a9;
}

// Function: sub_17750 @ 0x17750
long sub_17750(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  long v1; // rax
  long v2; // rdx
  long v3;
  
  v1 = sub_17570(a0,1,a0,a3,a4,a1);
  v3 = v1;
  sub_17570(a1);
  v1 = v2 + v3 + v1;
  return v1;
}

// Function: sub_17780 @ 0x17780
void sub_17780(void)
{
  sub_17620();
}

// Function: sub_177b0 @ 0x177b0
void sub_177b0(void *a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x18
  
  v1 = a1;
  (*a0)(&v1,a2);
}

// Function: sub_17800 @ 0x17800
int * sub_17800(unsigned long a0,long *a1,int *a2)
{
  int *v1; // rax
  int v10; // stack - 0x6c
  int v11; // stack - 0x68
  int v12; // stack - 0x64
  int v13; // stack - 0x60
  int v14; // stack - 0x5c
  unsigned long v15; // stack - 0x58
  int *v16; // stack - 0x50
  long v2; // rax
  long v3;
  long v4;
  int v5;
  int v6; // stack - 0x80
  int v7; // stack - 0x7c
  unsigned long v8; // stack - 0x78
  int v9; // stack - 0x70
  
  v4 = *a1;
  v16 = (int *)sub_177b0(a0,v4,a2);
  if (v16) // branch-flip
    *a1 = v4;
  else {
    v1 = __errno_location();
    v3 = 0;
    v5 = -1;
    if (*v1 == 0x4b) {
      while ((v2 = sub_17750(v3,v4), v3 != v2 && (v4 != v2))) {
        if (sub_177b0(a0,v2,a2)) { // branch-flip
          v7 = a2[1];
          v5 = *a2;
          v10 = a2[2];
          v11 = a2[3];
          v12 = a2[4];
          v9 = a2[5];
          v13 = a2[6];
          v14 = a2[7];
          v6 = a2[8];
          v8 = *(unsigned long *)&a2[10];
          v15 = *(unsigned long *)&a2[0xc];
          v3 = v2;
        }
        else {
          if (*v1 != 0x4b)
            return NULL;
          v4 = v2;
        }
      }
      if (0 <= v5) {
        *a1 = v3;
        *a2 = v5;
        a2[1] = v7;
        a2[2] = v10;
        a2[3] = v11;
        a2[4] = v12;
        a2[5] = v9;
        a2[6] = v13;
        a2[7] = v14;
        a2[8] = v6;
        *(unsigned long *)&a2[10] = v8;
        *(unsigned long *)&a2[0xc] = v15;
        v16 = a2;
      }
    }
  }
  return v16;
}

// Function: sub_17990 @ 0x17990
void sub_17990(void)
{
  tzset(); // tail-call
}

// Function: sub_179a0 @ 0x179a0
long sub_179a0(int *a0,unsigned long a1,long *a2)
{
  long v1;
  long v10; // rax
  long v11;
  long v12;
  int v13;
  int v14; // stack - 0xb8
  long v15; // stack - 0xc8
  char v16 [32];
  long v17; // stack - 0xc0
  long v18; // rbp
  int v19;
  int v2;
  int *v20;
  long *v21;
  unsigned long v22; // r10
  int v23;
  int v24; // r14d
  bool v25;
  long v26; // stack - 0x130
  int v27; // stack - 0x118
  unsigned int v28; // stack - 0xb4
  unsigned long v29; // stack - 0xb0
  int v3;
  unsigned long v30; // stack - 0xa8
  unsigned long v31; // stack - 0xa0
  int v32; // stack - 0x98
  unsigned int v33; // stack - 0x94
  unsigned long v34; // stack - 0x90
  unsigned long v35; // stack - 0x88
  unsigned int v36; // stack - 0x58
  unsigned int v4;
  int v5;
  int v6;
  char v7; // al
  int v8;
  long v9; // rax
  
  v2 = *a0;
  v3 = a0[1];
  v4 = a0[2];
  v22 = (unsigned long)v4;
  v5 = a0[8];
  v19 = a0[4] % 0xc;
  v11 = (long)(a0[4] / 0xc + (v19 >> 0x1f)) + (long)a0[5];
  v20 = a0;
  v21 = a2;
  v8 = v2;
  v1 = (long)(int)((unsigned int)*(unsigned short *)(((long)(int)((v19 >> 0x1f & 0xcU) + v19) + (unsigned long)(unsigned char)sub_17580(v11) * 0xd) * 2 + 0x1dee0) - 1) + (long)v20[3];
  v19 = 0x3b;
  if (v8 <= 0x3b)
    v19 = v8;
  if (v19 <= -1)
    v19 = 0;
  v26._0_4_ = (int)*v21;
  v8 = -(int)v26;
  v9 = sub_17620(v11,v1,v22 & 0xffffffff,v3,v19,0x46,0,0,0,v8);
  v15 = v9;
  v26 = v9;
  v27 = 6;
  v25 = 0;
  v12 = v9;
  while( true ) {
    v18 = v12;
    if (!sub_17800(a1,&v15,&v14))
      return -1;
    v10 = sub_17780(v11,v1,v4,v3,v19,&v14);
    v6 = v32;
    v12 = v15;
    if (!v10) break;
    if ((v18 != v15) && (v26 == v15)) {
      if (v32 < 0) goto label_17b90;
      if (0 <= v5) { // branch-flip
        if ((v5 != 0) != (v32 != 0)) goto label_17b90;
      }
      else if (v25 <= (v32 != 0)) goto label_17b90;
    }
    v27 -= 1;
    if (!v27) goto label_17c90;
    v10 += v15;
    v25 = v32 != 0;
    v26 = v18;
    v15 = v10;
  }
  v7 = sub_17600(v5,v32);
  if (v7) {
    v24 = 0x92c70;
    do {
      v23 = 2;
      v13 = -v24;
      while( true ) {
        v17 = v13 + v12;
        if (!SCARRY8((long)v13,v12)) {
          if (!sub_17800(a1,&v17,v16))
            return -1;
          v7 = sub_17600(v5,v36);
          if (!v7) {
            v10 = sub_17780(v11,v1,v4,v3,v19,v16) + v17;
            if (sub_177b0(a1,v10,&v14)) {
              v12 = v10;
              goto label_17b90;
            }
            if (*__errno_location() != 0x4b)
              return -1;
          }
        }
        v13 += v24 * 2;
        if (v23 == 1) break;
        v23 = 1;
      }
      v24 += 0x92c70;
    } while (v24 != 0xdb04f20);
    v12 += (int)(((unsigned int)(v5 == 0) - (unsigned int)(v6 == 0)) * 0xe10);
    v15 = v12;
    if (!sub_177b0(a1,v12,&v14)) {
label_17c90:
      *__errno_location() = 0x4b;
      return -1;
    }
  }
label_17b90:
  *a2 = v12 - (v8 + v9);
  if (v14 != v2) {
    v11 = (long)v2 + (((unsigned long)(v2 <= 0) & (unsigned long)CONCAT31((undefined3)((unsigned int)v14 >> 8),v14 == 0x3c)) - (long)v19);
    v25 = SCARRY8(v11,v12);
    v12 = v11 + v12;
    v15 = v12;
    if (v25) goto label_17c90;
    if (!sub_177b0(a1,v12,&v14))
      return -1;
  }
  *(unsigned long *)a0 = CONCAT44(v28,v14);
  *(unsigned long *)&a0[2] = v29;
  *(unsigned long *)&a0[0xc] = v35;
  *(unsigned long *)&a0[4] = v30;
  *(unsigned long *)&a0[6] = v31;
  *(unsigned long *)&a0[8] = CONCAT44(v33,v32);
  *(unsigned long *)&a0[10] = v34;
  return v12;
}

// Function: sub_17e30 @ 0x17e30
void sub_17e30(unsigned long a0)
{
  sub_17990();
  sub_179a0(a0,dat_25fb0,0x27560); // tail-call
}

// Function: sub_17e60 @ 0x17e60
unsigned long sub_17e60(unsigned long a0,unsigned char a1)
{
  return a0 << (a1 & 0x3f) | a0 >> 0x40 - (a1 & 0x3f);
}

// Function: sub_17e70 @ 0x17e70
unsigned long sub_17e70(unsigned long a0,unsigned char a1)
{
  return a0 >> (a1 & 0x3f) | a0 << 0x40 - (a1 & 0x3f);
}

// Function: sub_17e80 @ 0x17e80
unsigned int sub_17e80(unsigned int a0,unsigned char a1)
{
  return a0 << (a1 & 0x1f) | a0 >> 0x20 - (a1 & 0x1f);
}

// Function: sub_17e90 @ 0x17e90
unsigned int sub_17e90(unsigned int a0,unsigned char a1)
{
  return a0 >> (a1 & 0x1f) | a0 << 0x20 - (a1 & 0x1f);
}

// Function: sub_17ea0 @ 0x17ea0
void sub_17ea0(void)
{
  sub_17e60(); // tail-call
}

// Function: sub_17eb0 @ 0x17eb0
void sub_17eb0(void)
{
  sub_17e70(); // tail-call
}

// Function: sub_17ec0 @ 0x17ec0
unsigned int sub_17ec0(unsigned int a0,unsigned char a1)
{
  return CONCAT22((short)((unsigned int)a0 >> 0x10),(unsigned short)a0 << (a1 & 0xf) | (unsigned short)a0 >> 0x10 - (a1 & 0xf));
}

// Function: sub_17ed0 @ 0x17ed0
unsigned int sub_17ed0(unsigned int a0,unsigned char a1)
{
  return CONCAT22((short)((unsigned int)a0 >> 0x10),(unsigned short)a0 >> (a1 & 0xf) | (unsigned short)a0 << 0x10 - (a1 & 0xf));
}

// Function: sub_17ee0 @ 0x17ee0
unsigned int sub_17ee0(unsigned int a0,unsigned char a1)
{
  return CONCAT31((undefined3)((unsigned int)a0 >> 8),(unsigned char)a0 << (a1 & 7) | (unsigned char)a0 >> 8 - (a1 & 7));
}

// Function: sub_17ef0 @ 0x17ef0
unsigned int sub_17ef0(unsigned int a0,unsigned char a1)
{
  return CONCAT31((undefined3)((unsigned int)a0 >> 8),(unsigned char)a0 >> (a1 & 7) | (unsigned char)a0 << 8 - (a1 & 7));
}

// Function: sub_17f00 @ 0x17f00
int sub_17f00(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned int v2; // eax
  
  if (a0 == a1)
    return 0;
  do {
    v2 = sub_d580(*a0);
    v1 = sub_d580(*a1);
    if (!(unsigned char)v2) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while ((unsigned char)v2 == v1);
  return (v2 & 0xff) - (unsigned int)v1;
}

// Function: sub_17f60 @ 0x17f60
unsigned long sub_17f60(void *a0) // early-return x2, return-dupe
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // rax
  
  v2 = __fpending(a0);
  v1 = ferror_unlocked(a0);
  v3 = sub_17fd0(a0);
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

// Function: sub_17fd0 @ 0x17fd0
unsigned long sub_17fd0(void *a0) // early-return
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
    if (sub_18080(a0)) {
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

// Function: sub_18060 @ 0x18060
void sub_18060(unsigned int *a0)
{
  if (!(*a0 & 0x100))
    return;
  sub_180c0(a0,0,1); // tail-call
}

// Function: sub_18080 @ 0x18080
void sub_18080(void *a0)
{
  if (a0) {
    if (__freading(a0)) {
      sub_18060(a0);
      fflush(a0); // tail-call
      return;
    }
  }
  fflush(a0); // tail-call
}

// Function: sub_180c0 @ 0x180c0
unsigned long sub_180c0(unsigned int *a0,unsigned long a1,unsigned int a2) // return-dupe
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

// Function: sub_18140 @ 0x18140
unsigned long sub_18140(char *a0,unsigned long a1)
{
  char v1;
  unsigned long v2; // rax
  
  v1 = *a0;
  if (v1) {
    v2 = 0;
    do {
      a0 = &a0[1];
      v2 = (v2 << 9 | v2 >> 0x37) + (long)v1;
      v1 = *a0;
    } while (v1);
    return v2 % a1;
  }
  return 0;
}

// Function: sub_18180 @ 0x18180
void sub_18180(unsigned long a0)
{
  malloc(a0); // tail-call
}

// Function: sub_18190 @ 0x18190
void sub_18190(void *a0,unsigned long a1)
{
  realloc(a0,a1 | a1 == 0); // tail-call
}

// Function: sub_181b0 @ 0x181b0
void sub_181b0(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1); // tail-call
}

// Function: sub_181c0 @ 0x181c0
void sub_181c0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((a1) && (a2)) {
    reallocarray(a0,a1,a2); // tail-call
    return;
  }
  reallocarray(a0,1,1); // tail-call
}

// Function: sub_181f0 @ 0x181f0
char * sub_181f0(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_18230 @ 0x18230
void sub_18230(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_26008); // tail-call
}

// Function: _DT_FINI @ 0x18244
void _DT_FINI(void)
{
  return;
}
