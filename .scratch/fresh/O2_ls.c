// Function: _DT_INIT @ 0x4000
void _DT_INIT(void) // return-dupe
{
  if (!dat_24fe0)
    return;
  (*dat_24fe0)();
}

// Function: sub_4020 @ 0x4020
void sub_4020(void)
{
  (*dat_24c88)(); // jump-as-call
}

// Function: free @ 0x4670
void free(void *a0)
{
  (*dat_24fb0)(); // jump-as-call
}

// Function: localtime_r @ 0x4680
tm * localtime_r(void *a0,tm *a1)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_24fc8)(); // jump-as-call
  return v1;
}

// Function: gmtime_r @ 0x4690
tm * gmtime_r(void *a0,tm *a1)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_24fd0)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x46a0
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_24fd8)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x46b0
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24fe8)(); // jump-as-call
  return v1;
}

// Function: __cxa_finalize @ 0x46c0
void __cxa_finalize(void)
{
  (*dat_24ff8)(); // jump-as-call
}

// Function: __ctype_toupper_loc @ 0x46d0
void * __ctype_toupper_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24c90)(); // jump-as-call
  return v1;
}

// Function: getenv @ 0x46e0
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24c98)(); // jump-as-call
  return v1;
}

// Function: fgetfilecon @ 0x46f0
int fgetfilecon(int a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_24ca0)(); // jump-as-call
  return v1;
}

// Function: sigprocmask @ 0x4700
int sigprocmask(int a0,sigset_t *a1,sigset_t *a2)
{
  int v1; // eax
  
  v1 = (*dat_24ca8)(); // jump-as-call
  return v1;
}

// Function: __snprintf_chk @ 0x4710
int __snprintf_chk(char *a0,unsigned long a1,int a2,unsigned long a3,char *a4,...)
{
  int v1; // eax
  
  v1 = (*dat_24cb0)(); // jump-as-call
  return v1;
}

// Function: raise @ 0x4720
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_24cb8)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x4730
void abort(void)
{
  (*dat_24cc0)(); // jump-as-call
}

// Function: __errno_location @ 0x4740
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_24cc8)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x4750
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_24cd0)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x4760
void _exit(int a0)
{
  (*dat_24cd8)(); // jump-as-call
}

// Function: strcpy @ 0x4770
char * strcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24ce0)(); // jump-as-call
  return v1;
}

// Function: __mbstowcs_chk @ 0x4780
unsigned long __mbstowcs_chk(wchar_t *a0,char *a1,unsigned long a2,unsigned long a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_24ce8)(); // jump-as-call
  return v1;
}

// Function: __fpending @ 0x4790
unsigned long __fpending(FILE *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_24cf0)(); // jump-as-call
  return v1;
}

// Function: isatty @ 0x47a0
int isatty(int a0)
{
  int v1; // eax
  
  v1 = (*dat_24cf8)(); // jump-as-call
  return v1;
}

// Function: sigaction @ 0x47b0
int sigaction(int a0,sigaction *a1,sigaction *a2)
{
  int v1; // eax
  
  v1 = (*dat_24d00)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x47c0
int iswcntrl(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_24d08)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x47d0
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24d10)(); // jump-as-call
  return v1;
}

// Function: wcswidth @ 0x47e0
int wcswidth(wchar_t *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = (*dat_24d18)(); // jump-as-call
  return v1;
}

// Function: localeconv @ 0x47f0
lconv * localeconv(void)
{
  lconv *v1; // rax
  
  v1 = (lconv *)(*dat_24d20)(); // jump-as-call
  return v1;
}

// Function: faccessat @ 0x4800
int faccessat(int a0,char *a1,int a2,int a3)
{
  int v1; // eax
  
  v1 = (*dat_24d28)(); // jump-as-call
  return v1;
}

// Function: mbstowcs @ 0x4810
unsigned long mbstowcs(wchar_t *a0,char *a1,unsigned long a2)
{
  unsigned long v1; // rax
  
  v1 = (*dat_24d30)(); // jump-as-call
  return v1;
}

// Function: readlink @ 0x4820
long readlink(char *a0,char *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_24d38)(); // jump-as-call
  return v1;
}

// Function: clock_gettime @ 0x4830
int clock_gettime(int a0,timespec *a1)
{
  int v1; // eax
  
  v1 = (*dat_24d40)(); // jump-as-call
  return v1;
}

// Function: setenv @ 0x4840
int setenv(char *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_24d48)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x4850
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24d50)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x4860
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_24d58)(); // jump-as-call
  return v1;
}

// Function: opendir @ 0x4870
DIR * opendir(char *a0)
{
  DIR *v1; // rax
  
  v1 = (DIR *)(*dat_24d60)(); // jump-as-call
  return v1;
}

// Function: getpwuid @ 0x4880
passwd * getpwuid(unsigned int a0)
{
  passwd *v1; // rax
  
  v1 = (passwd *)(*dat_24d68)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x4890
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24d70)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x48a0
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24d78)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x48b0
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_24d80)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x48c0
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_24d88)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x48d0
void __stack_chk_fail(void)
{
  (*dat_24d90)(); // jump-as-call
}

// Function: getopt_long @ 0x48e0
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_24d98)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x48f0
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_24da0)(); // jump-as-call
  return v1;
}

// Function: freecon @ 0x4900
void freecon(char *a0)
{
  (*dat_24da8)(); // jump-as-call
}

// Function: strchr @ 0x4910
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24db0)(); // jump-as-call
  return v1;
}

// Function: getgrgid @ 0x4920
group * getgrgid(unsigned int a0)
{
  group *v1; // rax
  
  v1 = (group *)(*dat_24db8)(); // jump-as-call
  return v1;
}

// Function: snprintf @ 0x4930
int snprintf(char *a0,unsigned long a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_24dc0)(); // jump-as-call
  return v1;
}

// Function: __overflow @ 0x4940
int __overflow(FILE *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_24dc8)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x4950
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24dd0)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x4960
void lseek(void)
{
  (*dat_24dd8)(); // jump-as-call
}

// Function: __assert_fail @ 0x4970
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_24de0)(); // jump-as-call
}

// Function: fputs @ 0x4980
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_24de8)(); // jump-as-call
  return v1;
}

// Function: fnmatch @ 0x4990
int fnmatch(char *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_24df0)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x49a0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24df8)(); // jump-as-call
  return v1;
}

// Function: ioctl @ 0x49b0
int ioctl(int a0,unsigned long a1,...)
{
  int v1; // eax
  
  v1 = (*dat_24e00)(); // jump-as-call
  return v1;
}

// Function: getcwd @ 0x49c0
char * getcwd(char *a0,unsigned long a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24e08)(); // jump-as-call
  return v1;
}

// Function: strspn @ 0x49d0
unsigned long strspn(char *a0,char *a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_24e10)(); // jump-as-call
  return v1;
}

// Function: closedir @ 0x49e0
int closedir(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_24e18)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x49f0
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_24e20)(); // jump-as-call
  return v1;
}

// Function: _setjmp @ 0x4a00
int _setjmp(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_24e28)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x4a10
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_24e30)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x4a20
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24e38)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x4a30
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24e40)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x4a40
void signal(void)
{
  (*dat_24e48)(); // jump-as-call
}

// Function: dirfd @ 0x4a50
int dirfd(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_24e50)(); // jump-as-call
  return v1;
}

// Function: getpwnam @ 0x4a60
passwd * getpwnam(char *a0)
{
  passwd *v1; // rax
  
  v1 = (passwd *)(*dat_24e58)(); // jump-as-call
  return v1;
}

// Function: __memcpy_chk @ 0x4a70
void * __memcpy_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24e60)(); // jump-as-call
  return v1;
}

// Function: sigemptyset @ 0x4a80
int sigemptyset(sigset_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_24e68)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x4a90
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_24e70)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x4aa0
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24e78)(); // jump-as-call
  return v1;
}

// Function: getgrnam @ 0x4ab0
group * getgrnam(char *a0)
{
  group *v1; // rax
  
  v1 = (group *)(*dat_24e80)(); // jump-as-call
  return v1;
}

// Function: tzset @ 0x4ac0
void tzset(void)
{
  (*dat_24e88)(); // jump-as-call
}

// Function: fileno @ 0x4ad0
int fileno(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_24e90)(); // jump-as-call
  return v1;
}

// Function: tcgetpgrp @ 0x4ae0
int tcgetpgrp(int a0)
{
  int v1; // eax
  
  v1 = (*dat_24e98)(); // jump-as-call
  return v1;
}

// Function: readdir @ 0x4af0
dirent * readdir(DIR *a0)
{
  dirent *v1; // rax
  
  v1 = (dirent *)(*dat_24ea0)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x4b00
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_24ea8)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x4b10
int fflush(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_24eb0)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x4b20
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24eb8)(); // jump-as-call
  return v1;
}

// Function: strcoll @ 0x4b30
int strcoll(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_24ec0)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x4b40
int __freading(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_24ec8)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x4b50
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_24ed0)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x4b60
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24ed8)(); // jump-as-call
  return v1;
}

// Function: stpncpy @ 0x4b70
char * stpncpy(char *a0,char *a1,unsigned long a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24ee0)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x4b80
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_24ee8)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x4b90
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_24ef0)(); // jump-as-call
  return v1;
}

// Function: statx @ 0x4ba0
int statx(int a0,char *a1,int a2,unsigned int a3,void *a4)
{
  int v1; // eax
  
  v1 = (*dat_24ef8)(); // jump-as-call
  return v1;
}

// Function: strftime @ 0x4bb0
unsigned long strftime(char *a0,unsigned long a1,char *a2,tm *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_24f00)(); // jump-as-call
  return v1;
}

// Function: mempcpy @ 0x4bc0
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24f08)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x4bd0
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24f10)(); // jump-as-call
  return v1;
}

// Function: error @ 0x4be0
void error(int a0,int a1,char *a2,...)
{
  (*dat_24f18)(); // jump-as-call
}

// Function: fseeko @ 0x4bf0
int fseeko(FILE *a0,long a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_24f20)(); // jump-as-call
  return v1;
}

// Function: strtoumax @ 0x4c00
void strtoumax(void)
{
  (*dat_24f28)(); // jump-as-call
}

// Function: unsetenv @ 0x4c10
int unsetenv(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_24f30)(); // jump-as-call
  return v1;
}

// Function: __cxa_atexit @ 0x4c20
void __cxa_atexit(void)
{
  (*dat_24f38)(); // jump-as-call
}

// Function: wcstombs @ 0x4c30
unsigned long wcstombs(char *a0,wchar_t *a1,unsigned long a2)
{
  unsigned long v1; // rax
  
  v1 = (*dat_24f40)(); // jump-as-call
  return v1;
}

// Function: gethostname @ 0x4c40
int gethostname(char *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = (*dat_24f48)(); // jump-as-call
  return v1;
}

// Function: sigismember @ 0x4c50
int sigismember(void *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_24f50)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x4c60
void exit(int a0)
{
  (*dat_24f58)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x4c70
int __fprintf_chk(FILE *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_24f60)(); // jump-as-call
  return v1;
}

// Function: getfilecon @ 0x4c80
int getfilecon(char *a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_24f68)(); // jump-as-call
  return v1;
}

// Function: fflush_unlocked @ 0x4c90
int fflush_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_24f70)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x4ca0
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_24f78)(); // jump-as-call
  return v1;
}

// Function: lgetfilecon @ 0x4cb0
int lgetfilecon(char *a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_24f80)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x4cc0
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_24f88)(); // jump-as-call
  return v1;
}

// Function: sigaddset @ 0x4cd0
int sigaddset(sigset_t *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_24f90)(); // jump-as-call
  return v1;
}

// Function: __ctype_tolower_loc @ 0x4ce0
void * __ctype_tolower_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24f98)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x4cf0
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_24fa0)(); // jump-as-call
  return v1;
}

// Function: __sprintf_chk @ 0x4d00
int __sprintf_chk(char *a0,int a1,unsigned long a2,char *a3,...)
{
  int v1; // eax
  
  v1 = (*dat_24fa8)(); // jump-as-call
  return v1;
}

// Function: main @ 0x4d10
int main(int argc,char **argv,char **envp)
{
  char **v1;
  unsigned char v10;
  int v11;
  int v12; // eax
  unsigned int v13;
  int v14; // eax
  int v15; // eax
  unsigned long *v16;
  void *v17;
  unsigned long v18; // rax
  unsigned char *v19; // rax
  char v2;
  void *v20; // stack - 0x58
  unsigned short v21; // stack - 0x43
  char *v22; // stack - 0x60
  char *v23;
  long v24;
  long *v25; // rbp
  char *v26;
  long v27;
  bool v28;
  unsigned int v29; // stack - 0x98
  unsigned long v3;
  int v30; // stack - 0x90
  int v31; // stack - 0x8c
  unsigned long v32; // stack - 0x88
  char *v33; // stack - 0x80
  void *v34; // stack - 0x78
  int v35; // stack - 0x70
  unsigned long v36; // stack - 0x50
  char v37; // stack - 0x41
  long v38; // stack - 0x40
  char *v4;
  void *v5;
  void *v6;
  unsigned short v7;
  char v8;
  FILE *v9;
  
  v38 = *(long *)(v27 + 0x28);
  sub_150f0(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  dat_251f8 = 2;
  sub_19d40(sub_f770);
  dat_262b8 = 1;
  dat_26210 = 0;
  dat_26380 = NULL;
  dat_26370 = 0x8000000000000000;
  dat_26378 = 0xffffffffffffffff;
  v32 = 0xffffffffffffffff;
  v34 = (void *)0xffffffffffffffff;
  v31 = -1;
  v30 = -1;
  v35 = -1;
  v29 = 0xffffffff;
  v28 = 0;
  v33 = NULL;
label_4e20:
  v18 = (unsigned long)(unsigned int)argc;
  v20 = (void *)CONCAT44(v20._4_4_,0xffffffff);
  v11 = getopt_long(argc,argv,"abcdfghiklmnopqrstuvw:xABCDFGHI:LNQRST:UXZ1",(option *)0x24300,(int *)&v20);
  if (v11 != -1) {
    switch(v11) {
      case 0x31:
        v29 = (unsigned int)(v29 != 0);
        break;
      case 0x41:
        dat_262f0 = 1;
        break;
      case 0x42:
        sub_6be0("*~");
        sub_6be0(".*~");
        break;
      case 0x43:
        v29 = 2;
        break;
      case 0x44:
        dat_26318 = 1;
        break;
      case 0x46:
        if (optarg) {
          v11 = *(int *)(sub_ecf0("--classify",optarg,0x241e0,0x1a6e0,4,dat_251f0,1) * 4 + 0x1a6e0);
          if ((v11 != 1) && ((v11 != 2 || (!sub_6d10())))) break;
        }
        dat_26314 = 3;
        break;
      case 0x47:
        dat_25028 = 0;
        break;
      case 0x48:
        dat_262f8 = 2;
        break;
      case 0x49:
        sub_6be0(optarg);
        break;
      case 0x4c:
        dat_262f8 = 4;
        break;
      case 0x4e:
        v30 = 0;
        break;
      case 0x51:
        v30 = 5;
        break;
      case 0x52:
        dat_262f6 = '\x01';
        break;
      case 0x53:
        v31 = 3;
        break;
      case 0x54:
        v23 = dcgettext(NULL,"invalid tab size",5);
        v34 = (void *)sub_18ad0(optarg,0,0,0x7fffffffffffffff,0x1bb19,v23,2);
        break;
      case 0x55:
        v31 = 6;
        break;
      case 0x58:
        v31 = 1;
        break;
      case 0x5a:
        dat_26365 = '\x01';
        break;
      case 0x61:
        dat_262f0 = 2;
        break;
      case 0x62:
        v30 = 7;
        break;
      case 99:
        dat_26334 = 1;
        break;
      case 100:
        dat_262f5 = '\x01';
        break;
      case 0x66:
        dat_262f0 = 2;
        dat_26312 = 0;
        dat_26311 = 0;
        v13 = 0xffffffff;
        if (v29)
          v13 = v29;
        dat_2632c = '\0';
        v31 = 6;
        v29 = v13;
        break;
      case 0x67:
        dat_25029 = 0;
        v29 = 0;
        break;
      case 0x68:
        dat_26328 = 0xb0;
        dat_2631c = 0xb0;
        dat_26320 = 1;
        dat_25020 = 1;
        break;
      case 0x69:
        dat_262fc = 1;
        break;
      case 0x6b:
        v28 = 1;
        break;
      case 0x6c:
        v29 = 0;
        break;
      case 0x6d:
        v29 = 4;
        break;
      case 0x6e:
        dat_2632d = 1;
        v29 = 0;
        break;
      case 0x6f:
        dat_25028 = 0;
        v29 = 0;
        break;
      case 0x70:
        dat_26314 = 1;
        break;
      case 0x71:
        v35 = 1;
        break;
      case 0x72:
        dat_2632f = 1;
        break;
      case 0x73:
        dat_2632c = '\x01';
        break;
      case 0x74:
        v31 = 5;
        break;
      case 0x75:
        dat_26334 = 2;
        break;
      case 0x76:
label_5170:
        v31 = 4;
        break;
      case 0x77:
        v32 = sub_6c10(optarg);
        if (0 <= (long)v32) break;
        argv = (char **)sub_174c0(optarg);
        error(2,0,"%s: %s",dcgettext(NULL,"invalid line width",5),argv);
        goto label_5170;
      case 0x78:
        v29 = 3;
        break;
      case 0x80:
        dat_2632e = 1;
        break;
      case 0x81:
        v11 = sub_12150(optarg,(unsigned int *)0x26328,(unsigned long *)0x26320);
        if (v11)
          sub_18d20(v11,(unsigned long)v20 & 0xffffffff,0,0x24300,optarg); // no-return
        dat_2631c = dat_26328;
        dat_25020 = dat_26320;
        break;
      case 0x82:
        if (optarg) { // branch-flip
          v11 = *(int *)(sub_ecf0("--color",optarg,0x241e0,0x1a6e0,4,dat_251f0) * 4 + 0x1a6e0);
          if (v11 == 1) goto label_50a7;
          v10 = 0;
          if (v11 == 2)
            v10 = sub_6d10();
        }
        else {
label_50a7:
          v10 = 1;
        }
        dat_26312 = v10 & 1;
        break;
      case 0x83:
        dat_262f8 = 3;
        break;
      case 0x84:
        dat_26314 = 2;
        break;
      case 0x85:
        v29 = *(unsigned int *)(sub_ecf0("--format",optarg,0x242c0,0x1a750,4,dat_251f0) * 4 + 0x1a750);
        break;
      case 0x86:
        v29 = 0;
        v33 = "full-iso";
        break;
      case 0x87:
        dat_262f4 = 1;
        break;
      case 0x88:
        v16 = (unsigned long *)sub_18590(0x10);
        *v16 = optarg;
        v16[1] = dat_262e0;
        dat_262e0 = v16;
        break;
      case 0x89:
        if (optarg) { // branch-flip
          v11 = *(int *)(sub_ecf0("--hyperlink",optarg,0x241e0,0x1a6e0,4,dat_251f0,1) * 4 + 0x1a6e0);
          if (v11 == 1) goto label_4f7e;
          v10 = 0;
          if (v11 == 2)
            v10 = sub_6d10();
        }
        else {
label_4f7e:
          v10 = 1;
        }
        dat_26311 = v10 & 1;
        break;
      case 0x8a:
        dat_26314 = *(unsigned int *)(sub_ecf0("--indicator-style",optarg,0x24980,0x1a770,4,dat_251f0,1) * 4 + 0x1a770);
        break;
      case 0x8b:
        v30 = *(int *)(sub_ecf0("--quoting-style",optarg,0x24a20,0x1e9c0,4,dat_251f0,1) * 4 + 0x1e9c0);
        break;
      case 0x8c:
        goto label_4e85;
      case 0x8d:
        dat_26328 = 0x90;
        dat_2631c = 0x90;
        dat_26320 = 1;
        dat_25020 = 1;
        break;
      case 0x8e:
        v31 = *(int *)(sub_ecf0("--sort",optarg,0x24280,0x1a730,4,dat_251f0,1) * 4 + 0x1a730);
        break;
      case 0x8f:
        dat_26334 = *(int *)(sub_ecf0("--time",optarg,0x24240,0x1a710,4,dat_251f0,1) * 4 + 0x1a710);
        break;
      case 0x90:
        goto label_5463;
      case 0x91:
        dat_25019 = '\0';
        dat_26312 = 0;
        v30 = 0;
        v29 = (unsigned int)(v29 != 0);
label_4e85:
        v35 = 0;
        break;
      case 0xffffff7d:
        v23 = "ls";
        if ((dat_251e0 != 1) && (v23 = "dir", dat_251e0 != 2))
          v23 = "vdir";
        sub_183c0(stdout,v23,"GNU coreutils",dat_251e8,"Richard M. Stallman","David MacKenzie",0,v18);
        exit(0); // no-return
      case 0xffffff7e:
        sub_df00(0); // no-return
      default:
        goto label_65b3;
      
    }
    goto label_4e20;
  }
  if (!dat_26320) {
    v19 = (unsigned char *)getenv("LS_BLOCK_SIZE");
    sub_12150(v19,(unsigned int *)0x26328,(unsigned long *)0x26320);
    if ((v19) || (getenv("BLOCK_SIZE"))) {
      dat_2631c = dat_26328;
      dat_25020 = dat_26320;
    }
    if (v28) {
      dat_26328 = 0;
      dat_26320 = 0x400;
    }
  }
  if (0 <= (int)v29) { // branch-flip
    dat_26338 = v29;
    if (2 < v29 - 2) goto label_5af0;
label_556e:
    if (v32 == 0xffffffffffffffff) {
      if ((sub_6d10()) && (0 <= ioctl(1,0x5413,&v20))) {
        v32 = (unsigned long)v20._2_2_;
        if (v20._2_2_) goto label_557a;
      }
      v19 = (unsigned char *)getenv("COLUMNS");
      if ((v19) && (*v19)) {
        v32 = sub_6c10(v19);
        if (0 <= (long)v32) goto label_557a;
        v23 = (char *)sub_174c0(v19);
        error(0,0,dcgettext(NULL,"ignoring invalid width in environment variable COLUMNS: %s",5),v23);
      }
      v32 = 0x50; // crossjump-dupe
    }
  }
  else {
    if (dat_251e0 != 1) { // branch-flip
      if (dat_251e0 == 2) {
label_6386:
        dat_26338 = 2;
        goto label_556e;
      }
      dat_26338 = 0;
    }
    else {
      if (sub_6d10()) goto label_6386;
      dat_26338 = 1;
    }
label_5af0:
    if (dat_26312) goto label_556e;
    if (v32 != 0xffffffffffffffff) goto label_557a;
    v32 = 0x50;
  }
label_557a:
  dat_262b0 = v32;
  dat_26200 = (unsigned long)(v32 != (SUB168(ZEXT816(v32) * ZEXT816(0xaaaaaaaaaaaaaaab),8) & 0xfffffffffffffffe) + v32 / 3) + v32 / 3;
  if (dat_26338 - 2 <= 2) {
    if (0 <= (long)v34) { // branch-flip
label_59f4:
      dat_262c0 = v34;
    }
    else {
      dat_262c0 = (void *)0x8;
      v19 = (unsigned char *)getenv("TABSIZE");
      if (v19) {
        if (!sub_18de0(v19,NULL,0,&v20,"")) {
          v34 = v20;
          goto label_59f4;
        }
        v23 = (char *)sub_174c0(v19);
        error(0,0,dcgettext(NULL,"ignoring invalid tab size in environment variable TABSIZE: %s",5),v23);
      }
    }
  }
  v10 = (unsigned char)v35;
  if ((v35 == -1) && (v10 = 0, dat_251e0 == 1))
    v10 = sub_6d10();
  dat_262d8 = v10 & 1;
  if (0 <= v30) { // branch-flip
label_55eb:
    sub_16950(NULL,v30); // crossjump-dupe
  }
  else {
    v23 = getenv("QUOTING_STYLE");
    if (v23) {
      v11 = sub_e9c0(v23,(long *)0x24a20,(void *)0x1e9c0,4);
      if (0 <= v11) { // branch-flip
        v30 = *(int *)((long)v11 * 4 + 0x1e9c0);
        if (0 <= v30) goto label_55eb;
      }
      else {
        v23 = (char *)sub_174c0(v23);
        error(0,0,dcgettext(NULL,"ignoring invalid value of environment variable QUOTING_STYLE: %s",5),v23);
      }
    }
    v30 = 7;
    if (dat_251e0 != 1) goto label_55eb;
    if (sub_6d10()) {
      v30 = 3;
      sub_16950(NULL,v30);
    }
  }
  v11 = sub_16930(NULL);
  if (((dat_26338) && ((2 <= dat_26338 - 2 || (!dat_262b0)))) || ((v11 != 3 && ((v11 != 6 && (v11 != 1)))))) {
    dat_263a8 = 0;
    dat_262d0 = sub_168f0(NULL);
    if (v11 == 7)
      sub_16970(dat_262d0,0x20,1);
  }
  else {
    dat_263a8 = 1;
    dat_262d0 = sub_168f0(NULL);
  }
  if (2 <= dat_26314) {
    v23 = &"*=>@|"[dat_26314 - 2];
    v2 = "*=>@|"[dat_26314 - 2];
    while (v2) {
      v23 = &v23[1];
      sub_16970(dat_262d0,(int)v2,1);
      v2 = *v23;
    }
  }
  dat_262c8 = sub_168f0(NULL);
  sub_16970(dat_262c8,0x3a,1);
  dat_26318 = (dat_26311 ^ 1) & dat_26338 == 0 & dat_26318;
  if ((int)dat_25019 < (int)(unsigned int)dat_26318) {
    error(2,0,dcgettext(NULL,"--dired and --zero are incompatible",5));
    return v15;
  }
  if (0 <= v31) { // branch-flip
label_56f1:
    dat_26330 = v31;
    if (!dat_26338) {
label_5a12:
      if (!v33) {
        v33 = getenv("TIME_STYLE");
        if (!v33)
          v33 = "locale";
      }
      while (!strncmp(v33,"posix-",6)) {
        if (!sub_10260(2)) goto label_56ff;
        v33 = &v33[6];
      }
      if (*v33 != '+') { // branch-flip
        v25 = (long *)0x249c0;
        v24 = sub_e9c0(v33,(long *)0x249c0,(void *)0x1a780,4);
        if (v24 < 0) {
          sub_eb30("time style",v33,v24);
          v9 = stderr;
          fputs_unlocked(dcgettext(NULL,"Valid arguments are:\n",5),v9);
          v23 = "full-iso";
          do {
            v25 = &v25[1];
            __fprintf_chk(stderr,1,"  - [posix-]%s\n",v23);
            v9 = stderr;
            v23 = (char *)*v25;
          } while (v23);
          fputs_unlocked(dcgettext(NULL,"  - +FORMAT (e.g., +%H:%M) for a \'date\'-style format\n",5),v9);
label_65b3:
          sub_df00(2); // no-return
        }
        switch(v24) { // branch-flip
          case 0:
            dat_25048 = "%Y-%m-%d %H:%M:%S.%N %z";
            dat_25040 = "%Y-%m-%d %H:%M:%S.%N %z";
            break;
          case 1:
            dat_25048 = "%Y-%m-%d %H:%M";
            dat_25040 = "%Y-%m-%d %H:%M";
            break;
          case 2:
            dat_25040 = "%Y-%m-%d ";
            dat_25048 = "%m-%d %H:%M";
            break;
          case 3:
            if (sub_10260(2)) {
              dat_25040 = dcgettext(NULL,dat_25040,2);
              dat_25048 = dcgettext(NULL,dat_25048,2);
            }
          
        }
      }
      else {
        v33 = &v33[1];
        v23 = strchr(v33,10);
        if (v23) { // branch-flip
          if (strchr(&v23[1],10)) {
            v23 = (char *)sub_174c0(v33);
            error(2,0,dcgettext(NULL,"invalid time style format %s",5),v23);
            return v14;
          }
          *v23 = '\0';
          dat_25048 = &v23[1];
        }
        else {
          dat_25048 = v33;
        }
        dat_25040 = v33;
      }
      sub_6d40();
    }
  }
  else {
    if (!dat_26338) {
      dat_26330 = 0;
      goto label_5a12;
    }
    if (3 <= (unsigned int)(dat_26334 - 1U)) {
      v31 = 0;
      goto label_56f1;
    }
    dat_26330 = 5;
  }
label_56ff:
  v11 = optind;
  if (dat_26312) { // branch-flip
    v22 = getenv("LS_COLORS");
    if ((v22) && (*v22)) {
      v21 = 0x3f3f;
      v37 = 0;
      dat_26300 = (void *)sub_18a70(v22);
      v20 = dat_26300;
      do {
        while( true ) {
          while (v2 = *v22, v2 == '*') {
            v23 = &v22[1];
            v17 = (void *)sub_18590(0x28);
            *(void **)((long)v17 + 0x20) = dat_26308;
            *(void **)((long)v17 + 8) = v20;
            dat_26308 = v17;
            if ((!sub_6960(&v20,&v22,1,v17)) || (v23 = &v22[1], *v22 != '=')) goto label_6174;
            *(void **)((long)v17 + 0x18) = v20;
            if (!sub_6960(&v20,&v22,0,(long *)((long)v17 + 0x10))) goto label_6174;
          }
          if (v2 != ':') break;
          v22 = &v22[1]; // branch-flip
        }
        if (!v2) goto label_61c9;
        v21 = CONCAT11(v21._1_1_,v2);
        v23 = &v22[1];
        if (!v22[1]) goto label_6174;
        v21 = CONCAT11(v22[1],v2);
        v23 = &v22[3];
        if (v22[2] != '=') goto label_6174;
        v24 = 0;
        v26 = "lc";
        while (strcmp((char *)&v21,v26)) {
          v24 += 1;
          v26 = *(char **)(v24 * 8 + 0x248a0);
          if (!v26) goto label_6141;
        }
        v24 = (long)(int)v24 * 0x10;
        *(void **)(v24 + 0x25068) = v20;
      } while (sub_6960(&v20,&v22,0,(long *)(v24 + 0x25060)));
label_6141:
      v23 = (char *)sub_174c0(&v21);
      error(0,0,dcgettext(NULL,"unrecognized prefix: %s",5),v23);
label_6174:
      error(0,0,dcgettext(NULL,"unparsable value for LS_COLORS environment variable",5));
      free(dat_26300);
      while (v5) {
        v17 = *(void **)((long)v5 + 0x20);
        free(v5);
      }
      dat_26312 = 0;
label_61c9:
      if ((dat_250d0 == 6) && (!strncmp(dat_250d8,"target",6)))
        dat_26390 = '\x01';
    }
    else {
      v23 = getenv("COLORTERM");
      if ((!v23) || (!*v23)) {
        v23 = getenv("TERM");
        if ((v23) && (*v23)) {
          v26 = "# Configuration file for dircolors, a utility to help you set the";
          do {
            if ((!strncmp(v26,"TERM ",5)) && (!fnmatch(&v26[5],v23,0))) goto label_61d7;
            v26 = &v26[strlen(v26) + 1];
          } while (&v26[-0x1a7a0] <= (char *)0x12c7);
        }
        dat_26312 = 0;
      }
    }
label_61d7:
    if (!dat_26312) goto label_5713;
    dat_262c0 = NULL;
    if ((((dat_262f4) || (sub_6c80(0xd))) || ((sub_6c80(0xe) && (dat_26390)))) || ((sub_6c80(0xc) && (!dat_26338)))) goto label_571c;
  }
  else {
label_5713:
    if (dat_262f4) {
label_571c:
      dat_262fd = 1;
    }
  }
  v24 = (long)v11;
  if (((!dat_262f8) && (dat_262f8 = 1, !dat_262f5)) && (dat_26314 != 3))
    dat_262f8 = (-(unsigned int)(dat_26338 == 0) & 0xfffffffe) + 3;
  if (dat_262f6) {
    dat_263c8 = sub_10c60(0x1e,NULL,sub_6910,sub_6920,sub_6bd0);
    if (!dat_263c8)
      sub_18a90(); // no-return
    _obstack_begin((obstack *)0x260e0,0,0,dat_24fe8,dat_24fb0);
  }
  dat_262a8 = sub_17950(getenv("TZ"));
  v5 = dat_24fe8;
  v17 = dat_24fb0;
  if (((dat_26330 - 3U & 0xfffffffdU) && (dat_26338)) && ((!dat_26365 && (!dat_2632c)))) {
    dat_262a1 = 0;
    dat_262a0 = 1;
    if (((!dat_262f6) && (!dat_26312)) && (!dat_26314)) {
    }
  }
  else {
    dat_262a1 = 1;
    dat_262a0 = 0;
  }
  dat_262a0 &= 1;
  if (dat_26318) {
    _obstack_begin((obstack *)0x261a0,0,0,dat_24fe8,dat_24fb0);
    _obstack_begin((obstack *)0x26140,0,0,v5,v17);
  }
  if (dat_26311) {
    v18 = 0;
    do {
      while (v12 = (int)v18, v18 <= 0x5a) {
        v28 = 1;
        if ((v12 <= 0x40) && (10 <= (unsigned int)(v12 - 0x30U))) goto label_5d58;
label_5d70:
        *(unsigned char *)(v18 + 0x25fe0) = *(unsigned char *)(v18 + 0x25fe0) | v28;
        v18 += 1;
        if (v18 == 0x100) goto label_5db0;
      }
      v28 = 1;
      if (0x1a > (unsigned int)(v12 - 0x61U)) goto label_5d70;
label_5d58:
      if (((unsigned int)(v12 - 0x2dU) > 1) && (v12 != 0x7e)) {
        v28 = v12 == 0x5f;
        goto label_5d70;
      }
      *(unsigned char *)(v18 + 0x25fe0) = *(unsigned char *)(v18 + 0x25fe0) | 1;
      v18 += 1;
    } while (v18 != 0x100);
label_5db0:
    dat_26388 = sub_18c10();
    if (!dat_26388)
      dat_26388 = 0x1bb19;
  }
  dat_263b8 = 100;
  dat_263c0 = sub_186d0(100,0xd0);
  dat_263b0 = 0;
  sub_8730();
  if (1 <= argc - v11) { // branch-flip
    do {
      v1 = &argv[v24];
      v24 += 1;
      sub_96f0(*v1,0,1,"");
    } while ((int)v24 < argc);
    if (!dat_263b0) {
label_587e:
      v16 = dat_26380;
      if (1 < argc - v11) goto label_58c5;
      goto label_5f36;
    }
label_5b72:
    sub_8e30();
    if (!dat_262f5)
      sub_9510(NULL,1);
    if (!dat_263b0) goto label_587e;
    sub_d420();
    if (!dat_26380) goto label_5ba8;
    dat_261f8 += 1;
    v4 = *(char **)&stdout->field_0x28;
    if (*(char **)&stdout->field_0x30 <= v4) {
      __overflow(stdout,10);
      v16 = dat_26380;
      goto label_58c5;
    }
    *(char **)&stdout->field_0x28 = &v4[1];
    *v4 = 10;
    v16 = dat_26380;
  }
  else {
    if (dat_262f5) // branch-flip
      sub_96f0(".",3,1,"");
    else {
      sub_6fb0(".",NULL,1);
    }
    if (dat_263b0) goto label_5b72;
label_5f36:
    if (!dat_26380) goto label_5ba8;
    v16 = dat_26380;
    if (!dat_26380[3])
      dat_262b8 = 0;
  }
  do {
    dat_26380 = (unsigned long *)v16[3];
    if ((dat_263c8) && (!(unsigned char *)*v16)) {
      if ((unsigned long)(dat_260f8 - dat_260f0) <= 0xf)
        __assert_fail("dev_ino_size <= obstack_object_size (&dev_ino_obstack)","src/ls.c",0x41d,"dev_ino_pop"); // no-return
      v24 = dat_260f8 + -0x10;
      v17 = *(void **)(dat_260f8 + -0x10);
      v3 = *(unsigned long *)(dat_260f8 + -8);
      dat_260f8 = v24;
      v17 = (void *)sub_11320(dat_263c8,&v20);
      if (!v17)
        __assert_fail("found","src/ls.c",0x70d,"main"); // no-return
      free(v17);
      free((void *)*v16);
      free((void *)v16[1]);
      free(v16);
    }
    else {
      sub_d820((unsigned char *)*v16,(unsigned char *)v16[1],(unsigned char)*(char *)&v16[2]);
      free((void *)*v16);
      free((void *)v16[1]);
      free(v16);
      dat_262b8 = 1;
    }
    v16 = dat_26380;
label_58c5:
  } while (v16);
label_5ba8:
  if ((dat_26312) && (dat_26310)) {
    if ((dat_25060 != 2) || (((memcmp(dat_25068,(void *)0x1bdf7,2) || (dat_25070 != 1)) || (*dat_25078 != 'm')))) {
      sub_7880((struct_1 *)0x25060);
      sub_7880((struct_1 *)0x25070);
    }
    fflush_unlocked(stdout);
    sub_76b0(0);
    for (; v11; v11 = v11 + -1) {
      raise(0x13);
    }
    if (dat_26218)
      raise(dat_26218);
  }
  if (dat_26318) {
    sub_7450("//DIRED//",(void *)0x261a0);
    sub_7450("//SUBDIRED//",(void *)0x26140);
    v13 = sub_16930(dat_262d0);
    __printf_chk(1,"//DIRED-OPTIONS// --quoting-style=%s\n",*(char **)((unsigned long)v13 * 8 + 0x24a20));
  }
  v24 = dat_263c8;
  if (dat_263c8) {
    if (sub_10790(dat_263c8))
      __assert_fail("hash_get_n_entries (active_dir_set) == 0","src/ls.c",0x741,"main"); // no-return
    sub_10e20(v24);
  }
  if (v38 != *(long *)(v27 + 0x28))
    __stack_chk_fail(); // no-return
  return dat_26210;
label_5463:
  v33 = optarg;
  goto label_4e20;
}

// Function: sub_6820 @ 0x6820
void sub_6820(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_24fb8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_6850 @ 0x6850
void sub_6850(void)
{
  return;
}

// Function: _FINI_0 @ 0x68c0
void _FINI_0(void)
{
  if (!dat_252c8) {
    if (dat_24ff8)
      __cxa_finalize(dat_25008);
    sub_6850();
    dat_252c8 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x6900
void _INIT_0(void)
{
  return;
}

// Function: sub_6910 @ 0x6910
unsigned long sub_6910(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_6920 @ 0x6920
unsigned long sub_6920(struct_0 *a0,struct_0 *a1)
{
  if (a0->field_0x0 != a1->field_0x0)
    return 0;
  return CONCAT71((undefined7)((unsigned long)a1->field_0x8 >> 8),a0->field_0x8 == a1->field_0x8);
}

// Function: sub_6940 @ 0x6940
void sub_6940(unsigned int a0) // return-dupe
{
  if (dat_26218)
    return;
  dat_26218 = a0;
}

// Function: sub_6960 @ 0x6960
unsigned int sub_6960(long *a0,long *a1,unsigned int a2,long *a3)
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
  if (v5 == '\\') goto label_69c6;
label_699a:
  if (v13 || SBORROW1(v5,'\\') != v6 < '\0') {
    if (v5 != '=') { // branch-flip
      if ((v5 <= '=') && ((!v5 || (v5 == ':')))) {
        v2 = 1;
label_69fe:
        *a0 = v11;
        *a1 = (long)v3;
        *a3 = v10;
        return v2;
      }
    }
    else {
      v2 = a2;
      if ((char)a2) goto label_69fe;
    }
  }
  else if (v5 == '^') {
    v1 = v3[1];
    if ((unsigned char)(v1 - 0x40) <= 0x3e) {
      v3 = &v3[2];
      v9 += 1;
      v8 = v7 + 1;
      *(unsigned char *)(v7 + -1) = v1 & 0x1f;
      goto label_69b6;
    }
    v3 = &v3[1];
    if (v1 == 0x3f) {
      v9 += 1;
      v8 = v7 + 1;
      *(char *)(v7 + -1) = 0x7f;
      goto label_69b6;
    }
    v2 = 0;
    goto label_69fe;
  }
  v3 = &v3[1];
  v8 = v7;
label_69ab:
  *(char *)(v8 + -1) = v5;
  v9 += 1;
  v8 += 1;
label_69b6:
  do {
    v5 = *v3;
    v11 = v8 + -1;
    v10 = v9 + -1;
    v6 = v5 + '\xa4';
    v13 = v6 == '\0';
    v4 = v3;
    v7 = v8;
    if (!v13) goto label_699a;
label_69c6:
    v6 = v4[1];
    v3 = &v4[2];
    if (!v6) {
      v2 = 0;
      goto label_69fe;
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
        if (8 <= (unsigned char)(v6 - 0x30U)) goto label_69ab;
        do {
          v3 = &v3[1];
          v5 = v6 + '\xd0' + v5 * '\b';
          v6 = *v3;
        } while ((unsigned char)(v6 - 0x30U) <= 7);
        *(char *)(v8 + -1) = v5;
        v9 += 1;
        v8 += 1;
        goto label_69b6;
      default:
        goto label_6a40;
      case 0xf:
        v6 = '\x7f';
        break;
      case 0x28:
      case 0x48:
        goto label_6ad7;
      case 0x2f:
        v6 = ' ';
        break;
      case 0x31:
        v6 = '\a';
        goto label_6a40;
      case 0x32:
        v6 = '\b';
        goto label_6a40;
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
label_6a40:
      
    }
    *(char *)(v8 + -1) = v6;
    v9 += 1;
    v8 += 1;
  } while( true );
label_6ad7:
  v6 = v4[2];
  v12 = &v4[3];
  v5 = '\0';
  v3 = &v4[2];
  if ('F' < v6) goto label_6b22;
  do {
    if ('A' <= v6) { // branch-flip
      v5 = v6 + '\xc9' + v5 * '\x10';
      v3 = v12;
    }
    else {
      if (10 <= (unsigned char)(v6 - 0x30U)) goto label_69ab;
      v5 = v6 + '\xd0' + v5 * '\x10';
      v3 = v12;
    }
    while( true ) {
      v6 = *v3;
      v12 = &v3[1];
      if (v6 <= 'F') break;
label_6b22:
      if (6 <= (unsigned char)(v6 + 0x9fU)) goto label_69ab;
      v5 = v6 + '\xa9' + v5 * '\x10';
      v3 = v12;
    }
  } while( true );
}

// Function: sub_6bd0 @ 0x6bd0
void sub_6bd0(void *a0)
{
  free(a0); // tail-call
}

// Function: sub_6be0 @ 0x6be0
void sub_6be0(unsigned long a0)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_18590(0x10);
  *v1 = a0;
  v1[1] = dat_262e8;
  dat_262e8 = v1;
}

// Function: sub_6c10 @ 0x6c10
long sub_6c10(unsigned char *a0) // early-return
{
  int v1; // eax
  long v2; // stack - 0x18
  
  v1 = sub_18de0(a0,NULL,0,&v2,"");
  if (v1) // branch-flip
    v2 = -(unsigned long)(v1 != 1);
  else if (v2 <= -1)
    return 0;
  return v2;
}

// Function: sub_6c80 @ 0x6c80
unsigned long sub_6c80(unsigned int a0)
{
  long v1;
  char *v2;
  unsigned long v3;
  
  v3 = 0;
  v1 = *(long *)((unsigned long)a0 * 0x10 + 0x25060);
  if (!v1)
    return 0;
  v2 = *(char **)((unsigned long)a0 * 0x10 + 0x25068);
  if (v1 != 1) {
    v3 = 0;
    if (v1 != 2)
      return 1;
    return CONCAT71((undefined7)((unsigned long)v3 >> 8),strncmp(v2,"00",2) != 0) & 0xffffffff;
  }
  return CONCAT71((undefined7)((unsigned long)v3 >> 8),strncmp(v2,"0",1) != 0) & 0xffffffff;
}

// Function: sub_6d10 @ 0x6d10
unsigned int sub_6d10(void)
{
  unsigned int v1; // eax
  
  if ('\0' <= (char)dat_25018)
    return dat_25018 & 1;
  v1 = isatty(1);
  dat_25018 = (char)v1;
  return v1 & 1;
}

// Function: sub_6d40 @ 0x6d40
void sub_6d40(void) // return-dupe
{
  unsigned short v1;
  char v10 [1536];
  unsigned long v11; // stack - 0x660
  char v12 [8];
  unsigned long v13;
  long v14;
  int v15; // r12d
  unsigned long v16;
  long v17; // stack - 0x678
  char v2;
  unsigned int v3; // eax
  char *v4;
  char *v5;
  long *v6; // rax
  char v7;
  char *v8;
  long v9 [2]; // stack - 0x658
  
  v14 = 0;
  do {
    v4 = *(char **)(v14 + 0x25040);
    v2 = *v4;
    while (v2) {
      v7 = v4[1];
      if (v2 == '%') {
        if (v7 != '%') { // branch-flip
          if (v7 == 'b') goto label_6dae;
        }
        else {
          v7 = v4[2];
          v4 = &v4[1];
        }
      }
      v4 = &v4[1];
      v2 = v7;
    }
    v4 = NULL;
label_6dae:
    *(char **)((long)v9 + v14) = v4;
    v14 += 8;
    if (v14 == 0x10) {
      if ((!v9[0]) && (!v9[1]))
        return;
      v16 = 0xc;
      while( true ) {
        v15 = 0x2000e;
        v13 = 0;
        v4 = v10;
        do {
          v11 = v16;
          v5 = nl_langinfo(v15);
          if (strchr(v5,0x25))
            return;
          v6 = __ctype_b_loc();
          v1 = *(unsigned short *)(*v6 + (unsigned long)(unsigned char)*v5 * 2);
          if (0x80 <= (unsigned long)sub_12810(v5,v4,0x80,&v11,v1 >> 0xb & 1,0))
            return;
          if (v13 < v11)
            v13 = v11;
          v15 += 1;
          v4 = &v4[0x80];
        } while (v4 != v12);
        if (v16 <= v13) break;
        v16 = v13;
      }
      v17 = 0;
      do {
        v5 = *(char **)(v17 * 8 + 0x25040);
        v14 = v9[v17];
        v4 = (char *)(v17 * 0x600 + 0x253e0);
        v8 = v10;
        do {
          if (v14) { // branch-flip
            if (0x80 < v14 - (long)v5)
              return;
            v3 = __snprintf_chk(v4,0x80,1,0xffffffffffffffff,"%.*s%s%s",(int)(v14 - (long)v5),v5,v8,(char *)(v14 + 2));
          }
          else {
            v3 = snprintf(v4,0x80,"%s",v5);
          }
          if (0x80 <= v3)
            return;
          v8 = &v8[0x80];
          v4 = &v4[0x80];
        } while (v8 != v12);
        if (v17 == 1) {
          dat_253c8 = 1;
          return;
        }
        v17 = 1;
      } while( true );
    }
  } while( true );
}

// Function: sub_6fb0 @ 0x6fb0
void sub_6fb0(char *a0,char *a1,char a2)
{
  unsigned long *v1; // rax
  unsigned long v2;
  
  v1 = (unsigned long *)sub_18590(0x20);
  v2 = 0;
  if (a1)
    v2 = sub_18a70(a1);
  v1[1] = v2;
  v2 = 0;
  if (a0)
    v2 = sub_18a70(a0);
  *v1 = v2;
  *(char *)&v1[2] = a2;
  v1[3] = dat_26380;
  dat_26380 = v1;
}

// Function: sub_7020 @ 0x7020
void sub_7020(unsigned long *a0,unsigned long *a1)
{
  strcmp((char *)*a0,(char *)*a1); // tail-call
}

// Function: sub_7030 @ 0x7030
void sub_7030(unsigned long *a0,unsigned long *a1)
{
  strcmp((char *)*a1,(char *)*a0); // tail-call
}

// Function: sub_7050 @ 0x7050
void sub_7050(int a0,char *a1,struct_2 *a2,unsigned int a3,unsigned int a4) // return-dupe x3
{
  long v1;
  unsigned long v10; // stack - 0xe8
  unsigned int v11; // stack - 0xe0
  unsigned long v12; // stack - 0xd8
  unsigned int v13; // stack - 0xd0
  unsigned long v14; // stack - 0xc8
  unsigned int v15; // stack - 0xc0
  unsigned long v16; // stack - 0xb8
  unsigned int v17; // stack - 0xb0
  unsigned int v18; // stack - 0xa8
  unsigned int v19; // stack - 0xa4
  unsigned char v2; // stack - 0x127
  unsigned int v20; // stack - 0xa0
  unsigned int v21; // stack - 0x9c
  unsigned int v3; // stack - 0x124
  unsigned int v4; // stack - 0x118
  unsigned long v5; // stack - 0x114
  unsigned short v6; // stack - 0x10c
  unsigned long v7; // stack - 0x108
  unsigned long v8; // stack - 0x100
  unsigned long v9; // stack - 0xf8
  
  if (statx(a0,a1,a3 | 0x800,a4,&v1) <= -1)
    return;
  a2->field_0x0 = ((unsigned long)v21 & 0xffffff00) << 0xc | ((unsigned long)v20 & 0xfffff000) << 0x20 | (unsigned long)((v20 & 0xfff) << 8) | (unsigned long)(unsigned char)v21;
  a2->field_0x10 = (unsigned long)v4;
  a2->field_0x8 = v7;
  a2->field_0x18 = (unsigned int)v6;
  *(unsigned long *)a2->field_0x1c = v5;
  a2->field_0x28 = ((unsigned long)v19 & 0xffffff00) << 0xc | ((unsigned long)v18 & 0xfffff000) << 0x20 | (unsigned long)((v18 & 0xfff) << 8) | (unsigned long)(unsigned char)v19;
  a2->field_0x38 = (unsigned long)v3;
  a2->field_0x30 = v8;
  a2->field_0x50 = (unsigned long)v11;
  a2->field_0x40 = v9;
  a2->field_0x60 = (unsigned long)v17;
  a2->field_0x48 = v10;
  a2->field_0x70 = (unsigned long)v15;
  a2->field_0x58 = v16;
  a2->field_0x68 = v14;
  if (!(a4 & 0x800))
    return;
  if (v2 & 8) {
    a2->field_0x58 = v12;
    a2->field_0x60 = (unsigned long)v13;
    return;
  }
  a2->field_0x60 = 0xffffffffffffffff;
  a2->field_0x58 = 0xffffffffffffffff;
}

// Function: sub_71f0 @ 0x71f0
void sub_71f0(unsigned long *a0,unsigned long *a1,void *a2)
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

// Function: sub_7270 @ 0x7270
void sub_7270(unsigned long *a0,unsigned long *a1)
{
  sub_71f0(a1,a0,dat_24fd8); // tail-call
}

// Function: sub_7290 @ 0x7290
void sub_7290(unsigned long *a0,unsigned long *a1)
{
  sub_71f0(a0,a1,dat_24fd8); // tail-call
}

// Function: sub_72a0 @ 0x72a0
unsigned char * sub_72a0(unsigned char *a0,bool a1)
{
  unsigned char v1;
  unsigned char *v2; // rax
  unsigned char *v3;
  unsigned char *v4; // rbp
  
  v2 = (unsigned char *)sub_186d0(3,strlen((char *)a0) + 1);
  v1 = *a0;
  v3 = v2;
  do {
    while( true ) {
      if (!v1) {
        *v3 = 0;
        return v2;
      }
      a0 = &a0[1];
      if ((v1 != 0x2f) || (!a1)) break;
      *v3 = 0x2f;
label_72ef:
      v3 = &v3[1];
      v1 = *a0;
    }
    if (*(char *)((unsigned long)v1 + 0x25fe0)) {
      *v3 = v1;
      goto label_72ef;
    }
    v4 = &v3[3];
    __sprintf_chk((char *)v3,1,0xffffffffffffffff,"%%%02x",(unsigned long)v1);
    v1 = *a0;
    v3 = v4;
  } while( true );
}

// Function: sub_7360 @ 0x7360
char * sub_7360(unsigned long a0,unsigned long a1)
{
  unsigned long v1;
  int v2; // eax
  char *v3; // rax
  char *v4;
  char *v5;
  
  if (a1 <= a0)
    return v3;
  do {
    while( true ) {
      v1 = a0 + 1;
      v4 = *(char **)&stdout->field_0x28;
      if ((!dat_262c0) || (a1 / dat_262c0 <= v1 / dat_262c0)) break;
      if (*(char **)&stdout->field_0x30 <= v4) // branch-flip
        __overflow(stdout,9);
      else {
        *(char **)&stdout->field_0x28 = &v4[1];
        *v4 = 9;
      }
      v4 = (char *)(a0 / dat_262c0);
      a0 = (dat_262c0 + a0) - a0 % dat_262c0;
      if (a1 <= a0)
        return v4;
    }
    if (*(char **)&stdout->field_0x30 <= v4) { // branch-flip
      v2 = __overflow(stdout,0x20);
      v5 = (char *)CONCAT44(dat_4,v2);
    }
    else {
      v5 = &v4[1];
      *(char **)&stdout->field_0x28 = v5;
      *v4 = 0x20;
    }
    a0 = v1;
  } while (v1 < a1);
  return v5;
}

// Function: sub_7430 @ 0x7430
void sub_7430(char *a0,unsigned long a1)
{
  dat_261f8 += a1;
  fwrite_unlocked(a0,1,a1,stdout); // tail-call
}

// Function: sub_7450 @ 0x7450
void sub_7450(char *a0,void *a1) // return-dupe
{
  unsigned long v1;
  char *v2;
  unsigned long v3; // rax
  unsigned long v4;
  unsigned long *v5;
  unsigned long *v6;
  
  v6 = *(unsigned long **)((long)a1 + 0x18);
  v5 = *(unsigned long **)((long)a1 + 0x10);
  if ((unsigned long)((long)v6 - (long)v5) <= 7)
    return;
  if (v6 == v5)
    *(unsigned char *)((long)a1 + 0x50) = *(unsigned char *)((long)a1 + 0x50) | 2;
  v3 = (long)v6 + *(unsigned long *)((long)a1 + 0x30) & ~*(unsigned long *)((long)a1 + 0x30);
  v4 = *(unsigned long *)((long)a1 + 0x20);
  if (v3 - *(long *)((long)a1 + 8) <= *(unsigned long *)((long)a1 + 0x20) - *(long *)((long)a1 + 8))
    v4 = v3;
  v6 = (unsigned long *)(((long)v6 - (long)v5 & 0xfffffffffffffff8U) + (long)v5);
  *(unsigned long *)((long)a1 + 0x18) = v4;
  *(unsigned long *)((long)a1 + 0x10) = v4;
  fputs_unlocked(a0,stdout);
  do {
    v1 = *v5;
    v5 = &v5[1];
    __printf_chk(1," %ld",v1);
  } while (v5 != v6);
  v2 = *(char **)&stdout->field_0x28;
  if (v2 < *(char **)&stdout->field_0x30) {
    *(char **)&stdout->field_0x28 = &v2[1];
    *v2 = 10;
    return;
  }
  __overflow(stdout,10); // tail-call
}

// Function: sub_7520 @ 0x7520
void sub_7520(void) // return-dupe
{
  if (dat_26218)
    return;
  dat_26214 += 1;
}

// Function: sub_7540 @ 0x7540
void sub_7540(bool a0,char *a1,char *a2) // return-dupe
{
  unsigned long v1; // rax
  
  v1 = sub_16dd0(4,a2);
  error(0,*__errno_location(),a1,v1);
  if (a0) {
    dat_26210 = 2;
    return;
  }
  if (dat_26210)
    return;
  dat_26210 = 1;
}

// Function: sub_75b0 @ 0x75b0
unsigned char sub_75b0(bool a0,unsigned int a1,int a2)
{
  unsigned int v1; // ecx
  bool v2;
  
  if (a0) { // branch-flip
    v1 = a1 & 0xf000;
    if (v1 == 0x8000) {
      if (dat_26314 != 3)
        return 0;
      return -((a1 & 0x49) != 0) & 0x2a;
    }
    if (v1 == 0x4000)
      return 0x2f;
    v2 = v1 == 0xa000;
    if (dat_26314 == 1)
      return 0;
  }
  else {
    if (a2 == 5)
      return 0;
    if ((a2 == 3) || (a2 == 9))
      return 0x2f;
    v2 = a2 == 6;
    if (dat_26314 == 1)
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

// Function: sub_76b0 @ 0x76b0
void sub_76b0(bool a0) // return-dupe
{
  int *v1; // rbx
  void *v2 [17]; // stack - 0xd8
  int v3;
  int *v4; // rbp
  undefined4 v5; // stack - 0x50
  
  if (a0) {
    v1 = (int *)0x1a664;
    v3 = 0x14;
    v4 = (int *)0x1a664;
    sigemptyset((sigset_t *)0x26220);
    while( true ) {
      sigaction(v3,NULL,(sigaction *)v2);
      if (v2[0] != (void *)0x1)
        sigaddset((sigset_t *)0x26220,v3);
      if (v4 == (int *)0x1a690) break;
      v3 = *v4;
      v4 = &v4[1];
    }
    v5 = 0x10000000;
    v3 = 0x14;
    v2[1] = (void *)dat_26220;
    v2[2] = (void *)dat_26228;
    v2[3] = (void *)dat_26230;
    v2[4] = (void *)dat_26238;
    v2[5] = (void *)dat_26240;
    v2[6] = (void *)dat_26248;
    v2[7] = (void *)dat_26250;
    v2[8] = (void *)dat_26258;
    v2[9] = (void *)dat_26260;
    v2[10] = (void *)dat_26268;
    v2[0xb] = (void *)dat_26270;
    v2[0xc] = (void *)dat_26278;
    v2[0xd] = (void *)dat_26280;
    v2[0xe] = (void *)dat_26288;
    v2[0xf] = (void *)dat_26290;
    v2[0x10] = (void *)dat_26298;
    while( true ) {
      if (sigismember((void *)0x26220,v3)) {
        v2[0] = sub_6940;
        if (v3 == 0x14)
          v2[0] = sub_7520;
        sigaction(v3,(sigaction *)v2,NULL);
      }
      if (v1 == (int *)0x1a690) break;
      v3 = *v1;
      v1 = &v1[1];
    }
    return;
  }
  v1 = (int *)0x1a664;
  v3 = 0x14;
  while( true ) {
    if (sigismember((void *)0x26220,v3))
      signal(v3,0);
    if (v1 == (int *)0x1a690) break;
    v3 = *v1;
    v1 = &v1[1];
  }
}

// Function: sub_7880 @ 0x7880
void sub_7880(struct_1 *a0)
{
  if (!dat_26310) {
    dat_26310 = '\x01';
    if (0 <= tcgetpgrp(1))
      sub_76b0(1);
    if (dat_25088) // branch-flip
      sub_7880(0x25080);
    else {
      sub_7880(0x25060);
      sub_7880(0x25090);
      sub_7880(0x25070);
    }
  }
  fwrite_unlocked(a0->field_0x8,a0->field_0x0,1,stdout); // tail-call
}

// Function: sub_7920 @ 0x7920
unsigned long sub_7920(unsigned long *a0,char *a1,unsigned int *a2,int a3,unsigned long *a4,unsigned char *a5)
{
  char v1;
  long v10;
  unsigned long v11;
  mbstate_t v12; // stack - 0x48
  int v13; // stack - 0x4c
  char *v14;
  char *v15;
  unsigned long v16;
  char *v17; // stack - 0x80
  bool v18; // stack - 0x69
  bool v2;
  unsigned char v3;
  int v4; // eax
  unsigned long v5; // rax
  char *v6;
  long *v7; // rax
  long v8; // rax
  char *v9;
  
  v17 = (char *)*a0;
  if ((unsigned int)sub_16930(a2) <= 2 & dat_262d8) { // branch-flip
    if (a3) { // branch-flip
      v2 = 1;
label_7982:
      v5 = sub_16a10(v17,0x2000,a1,0xffffffffffffffff,a2);
      if (0x2000 <= v5) {
        v17 = (char *)sub_18590(v5 + 1);
        sub_16a10(v17,v5 + 1,a1,0xffffffffffffffff,a2);
      }
      v18 = 1;
      if (*a1 == *v17)
        v18 = v5 != strlen(a1);
      v16 = v5;
      if (!v2) goto label_79f0;
    }
    else {
      v5 = strlen(a1);
      if (0x2000 <= v5)
        v17 = (char *)sub_18590(v5 + 1);
      memcpy(v17,a1,v5 + 1);
      v18 = 0;
    }
    v6 = &v17[v5];
    if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
      if (v6 <= v17) { // branch-flip
        v16 = 0;
        v5 = 0;
      }
      else {
        v5 = 0;
        v14 = v17;
        v9 = v17;
        do {
          v1 = *v14;
          if ('`' <= v1) { // branch-flip
            if (0x1e <= (unsigned char)(v1 + 0x9fU)) goto label_7bc5;
label_7c84:
            *v9 = v1;
            v14 = &v14[1];
            v5 += 1;
            v9 = &v9[1];
          }
          else {
            if ('@' < v1) goto label_7c84;
            if (v1 <= '#') {
              if ('\x1f' >= v1) goto label_7bc5;
              goto label_7c84;
            }
            if ((unsigned char)(v1 - 0x25U) <= 0x1a) goto label_7c84;
label_7bc5:
            v12 = 0;
            v15 = v9;
            do {
              v8 = sub_12780(&v13,v14,(long)v6 - (long)v14,&v12);
              v9 = &v15[1];
              if (v8 == -1) {
                v14 = &v14[1];
label_7ce8:
                v5 += 1;
                *v15 = '?';
                break;
              }
              if (v8 == -2) {
                v14 = v6;
                goto label_7ce8;
              }
              v10 = 1;
              if (v8)
                v10 = v8;
              v4 = wcwidth(v13);
              if (0 <= v4) { // branch-flip
                v8 = 0;
                do {
                  v15[v8] = v14[v8];
                  v8 += 1;
                } while (v10 != v8);
                v9 = &v15[v10];
                v5 += (long)v4;
              }
              else {
                *v15 = '?';
                v5 += 1;
              }
              v14 = &v14[v10];
              v15 = v9;
            } while (!mbsinit(&v12));
          }
        } while (v14 < v6);
        v16 = (long)v9 - (long)v17;
      }
    }
    else {
      if (v17 < v6) {
        v7 = __ctype_b_loc();
        v9 = v17;
        do {
          if (!(*(unsigned char *)(*v7 + 1 + (unsigned long)(unsigned char)*v9 * 2) & 0x40))
            *v9 = '?';
          v9 = &v9[1];
        } while (v9 != v6);
      }
      v16 = v5;
    }
    v3 = 0;
    v11 = v5;
    if (dat_263a8) goto label_7b29;
label_7b37:
    *a5 = v3;
    if (!a4) goto label_7b4e;
  }
  else {
    v2 = 0;
    if (a3) goto label_7982;
    v16 = strlen(a1);
    v18 = 0;
    v17 = a1;
label_79f0:
    if (!a4) {
      if (!dat_263a8) {
        *a5 = 0;
        goto label_7b4e;
      }
label_7b29:
      v3 = (v18 ^ 1U) & dat_263a9;
      goto label_7b37;
    }
    if (2 <= __ctype_get_mb_cur_max()) // branch-flip
      v11 = (unsigned long)(int)sub_12ca0(v17,v16,0);
    else if (&v17[v16] <= v17) // branch-flip
      v11 = 0;
    else {
      v7 = __ctype_b_loc();
      v11 = 0;
      v6 = v17;
      do {
        v11 = (v11 + 1) - (unsigned long)((*(unsigned short *)(*v7 + (unsigned long)(unsigned char)*v6 * 2) & 0x4000) == 0);
        v6 = &v6[1];
      } while (&v17[v16] != v6);
    }
    if (dat_263a8) goto label_7b29;
    *a5 = 0;
  }
  *a4 = v11;
label_7b4e:
  *a0 = v17;
  return v16;
}

// Function: sub_7d70 @ 0x7d70
long sub_7d70(char *a0,unsigned int *a1,unsigned int a2)
{
  char v1 [8200];
  char *v2; // stack - 0x2038
  unsigned char v3; // stack - 0x2039
  long v4; // stack - 0x2030
  
  v2 = v1;
  sub_7920(&v2,a0,a1,a2,&v4,&v3);
  if ((v2 != v1) && (v2 != a0))
    free(v2);
  return (unsigned long)v3 + v4;
}

// Function: sub_7e10 @ 0x7e10
long sub_7e10(struct_16 *a0)
{
  char v1;
  unsigned int v2;
  unsigned int v3;
  unsigned long v4; // rax
  long v5;
  char *v6; // rax
  long v7;
  char v8 [664];
  
  if (dat_262fc) { // branch-flip
    if (dat_26338 == 4) {
      v6 = (char *)sub_12720(a0->field_0x20,v8);
      v7 = strlen(v6) + 1;
      goto label_7ee2;
    }
    v7 = (long)dat_26360 + 1;
    if (dat_2632c) {
label_7e61:
      v4 = (unsigned long)dat_2635c;
      goto label_7e6c;
    }
    if (dat_26365) goto label_7f29;
  }
  else {
    v7 = 0;
label_7ee2:
    if (dat_2632c) {
      if (dat_26338 != 4) goto label_7e61;
      v6 = "?";
      if (a0->field_0xb8)
        v6 = (char *)sub_11550(a0->field_0x58,v8,dat_26328,0x200,dat_26320);
      v4 = strlen(v6);
label_7e6c:
      v7 += v4 + 1;
    }
    if (dat_26365) {
      if (dat_26338 != 4) { // branch-flip
label_7f29:
        v4 = (unsigned long)dat_26354;
      }
      else {
        v4 = strlen(a0->field_0xb0);
      }
      v7 += v4 + 1;
      v5 = a0->field_0xc8;
      goto label_7f41;
    }
  }
  v5 = a0->field_0xc8;
label_7f41:
  if (!v5)
    v5 = sub_7d70(a0->field_0x0,dat_262d0,a0->field_0xc4);
  v7 += v5;
  if (dat_26314) {
    v2 = a0->field_0xa8;
    v3 = a0->field_0x30;
    v1 = a0->field_0xb8;
    v7 = (v7 + 1) - (unsigned long)(sub_75b0((unsigned char)v1,v3,v2) == '\0');
  }
  return v7;
}

// Function: sub_7ff0 @ 0x7ff0
unsigned long sub_7ff0(bool a0)
{
  unsigned long *v1;
  unsigned long v10;
  char *v11;
  unsigned long v12;
  unsigned long v13;
  bool v14; // zf
  unsigned long *v2;
  char v3 [16];
  long v4; // rax
  unsigned long *v5;
  unsigned long *v6; // rax
  unsigned long v7; // rcx
  long v8;
  unsigned long v9;
  
  v10 = dat_263b0;
  v13 = dat_263b0;
  if (dat_26200) { // branch-flip
    if (dat_263b0 <= dat_26200) { // branch-flip
      if (dat_263b0 <= dat_252e0) goto label_827c;
      if (dat_263b0 < dat_26200 >> 1) goto label_8035;
    }
    else {
      v13 = dat_26200;
      if (dat_26200 <= dat_252e0) goto label_80ec;
      if (dat_26200 < dat_26200 >> 1) goto label_8035;
    }
    dat_26208 = (char *)sub_18650(dat_26208,dat_26200,0x18);
    v12 = dat_26200;
label_8054:
    v3 = ZEXT816(v12 - dat_252e0) * ZEXT816(dat_252e0 + 1 + v12);
    if ((CARRY8(dat_252e0 + 1,v12)) || (SUB168(v3,8)))
      sub_18a90(); // no-return
    v4 = sub_186d0(SUB168(v3,0) >> 1,8);
    if (dat_252e0 < v12) {
      v8 = dat_252e0 * 8 + 8;
      do {
        *(long *)&dat_26208[v8 * 3 + -8] = v4;
        v4 += v8;
        v8 += 8;
      } while (v12 * 8 + 8 != v8);
    }
    v10 = dat_263b0;
    dat_252e0 = v12;
    if (!v13) goto label_8288;
label_80ec:
    v4 = 3;
    v12 = 0;
    do {
      v2 = *(unsigned long **)&dat_26208[v4 * 8 + -8];
      dat_26208[v4 * 8 + -0x18] = '\x01';
      *(long *)&dat_26208[v4 * 8 + -0x10] = v4;
      v5 = v2;
      do {
        *v5 = 3;
        v6 = &v5[1];
        v14 = &v2[v12] != v5;
        v5 = v6;
      } while (v14);
      v12 += 1;
      v4 += 3;
    } while (v12 < v13);
    if (!v10) goto label_823d;
  }
  else {
    if (dat_252e0 < dat_263b0) {
label_8035:
      dat_26208 = (char *)sub_18650(dat_26208,v13,0x30);
      v12 = v13 * 2;
      goto label_8054;
    }
label_827c:
    if (dat_263b0) goto label_80ec;
label_8288:
    if (!dat_263b0)
      return v13;
  }
  v10 = 0;
  do {
    v4 = sub_7e10(*(struct_16 **)(dat_263a0 + v10 * 8));
    if (v13) {
      v12 = 0;
      v11 = dat_26208;
      do {
        v7 = v12 + 1;
        if (*v11) {
          if (a0) // branch-flip
            v9 = v10 / (((dat_263b0 - 1) + v7) / v7);
          else {
            v9 = v10 % v7;
          }
          v1 = (unsigned long *)(*(long *)&v11[0x10] + v9 * 8);
          v12 = v4 + (unsigned long)(v9 != v12) * 2;
          v9 = *v1;
          if (v9 < v12) {
            *(unsigned long *)&v11[8] = *(long *)&v11[8] + (v12 - v9);
            *v1 = v12;
            *v11 = *(unsigned long *)&v11[8] < dat_262b0;
          }
        }
        v11 = &v11[0x18];
        v12 = v7;
      } while (v13 != v7);
    }
    v10 += 1;
  } while (v10 < dat_263b0);
label_823d:
  if (2 <= v13) {
    v11 = &dat_26208[v13 * 0x18 + -0x18];
    v10 = v13;
    do {
      if (*v11)
        return v10;
      v10 -= 1;
      v11 = &v11[-0x18];
      v13 = 1;
    } while (v10 != 1);
  }
  return v13;
}

// Function: sub_82b0 @ 0x82b0
int sub_82b0(struct_22 *a0,struct_22 *a1)
{
  char *v1;
  char *v2;
  
  if (a0->field_0x48 < a1->field_0x48)
    return 0xffffffff;
  if (a1->field_0x48 == a0->field_0x48) {
    v1 = a0->field_0x0;
    v2 = a1->field_0x0;
    return strcmp(v2,v1); // tail-call
  }
  return 1;
}

// Function: sub_82f0 @ 0x82f0
void sub_82f0(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  
  v1 = (char *)*a1;
  v2 = (char *)*a0;
  if (!sub_101e0(v1,v2)) {
    strcmp(v1,v2); // tail-call
    return;
  }
}

// Function: sub_8330 @ 0x8330
void sub_8330(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  
  v1 = (char *)*a0;
  v2 = (char *)*a1;
  if (!sub_101e0(v1,v2)) {
    strcmp(v1,v2); // tail-call
    return;
  }
}

// Function: sub_8370 @ 0x8370
int sub_8370(struct_23 *a0,struct_23 *a1)
{
  unsigned int v1;
  char *v2;
  char *v3;
  
  v1 = ((unsigned int)(a1->field_0x88 < a0->field_0x88) - (unsigned int)(a0->field_0x88 < a1->field_0x88)) + ((unsigned int)(a1->field_0x80 < a0->field_0x80) - (unsigned int)(a0->field_0x80 < a1->field_0x80)) * 2;
  if (!v1) {
    v2 = a0->field_0x0;
    v3 = a1->field_0x0;
    return strcmp(v3,v2); // tail-call
  }
  return (unsigned long)v1;
}

// Function: sub_83d0 @ 0x83d0
int sub_83d0(struct_25 *a0,struct_25 *a1)
{
  unsigned int v1;
  char *v2;
  char *v3;
  
  v1 = ((unsigned int)(a1->field_0x78 < a0->field_0x78) - (unsigned int)(a0->field_0x78 < a1->field_0x78)) + ((unsigned int)(a1->field_0x70 < a0->field_0x70) - (unsigned int)(a0->field_0x70 < a1->field_0x70)) * 2;
  if (!v1) {
    v2 = a0->field_0x0;
    v3 = a1->field_0x0;
    return strcmp(v3,v2); // tail-call
  }
  return (unsigned long)v1;
}

// Function: sub_8430 @ 0x8430
int sub_8430(struct_25 *a0,struct_25 *a1)
{
  unsigned int v1;
  char *v2;
  char *v3;
  
  v1 = ((unsigned int)(a1->field_0x78 < a0->field_0x78) - (unsigned int)(a0->field_0x78 < a1->field_0x78)) + ((unsigned int)(a1->field_0x70 < a0->field_0x70) - (unsigned int)(a0->field_0x70 < a1->field_0x70)) * 2;
  if (!v1) {
    v2 = a0->field_0x0;
    v3 = a1->field_0x0;
    return strcmp(v3,v2); // tail-call
  }
  return (unsigned long)v1;
}

// Function: sub_8490 @ 0x8490
int sub_8490(struct_24 *a0,struct_24 *a1)
{
  unsigned int v1;
  char *v2;
  char *v3;
  
  v1 = ((unsigned int)(a1->field_0x68 < a0->field_0x68) - (unsigned int)(a0->field_0x68 < a1->field_0x68)) + ((unsigned int)(a1->field_0x60 < a0->field_0x60) - (unsigned int)(a0->field_0x60 < a1->field_0x60)) * 2;
  if (!v1) {
    v2 = a0->field_0x0;
    v3 = a1->field_0x0;
    return strcmp(v3,v2); // tail-call
  }
  return (unsigned long)v1;
}

// Function: sub_84f0 @ 0x84f0
void sub_84f0(void)
{
  sigset_t v1;
  int v2;
  
  while ((dat_26218 || (dat_26214))) {
    if (dat_26310) {
      sub_7880((struct_1 *)0x25060);
      sub_7880((struct_1 *)0x25070);
    }
    fflush_unlocked(stdout);
    sigprocmask(0,(sigset_t *)0x26220,&v1);
    v2 = dat_26218;
    if (dat_26214) { // branch-flip
      dat_26214 -= 1;
      v2 = 0x13;
    }
    else {
      signal(dat_26218,0);
    }
    raise(v2);
    sigprocmask(2,&v1,NULL);
  }
}

// Function: sub_85f0 @ 0x85f0
int sub_85f0(unsigned long a0) // early-return
{
  unsigned int v1; // eax
  char *v2; // rax
  
  if ((!dat_2632d) && (v2 = (char *)sub_12340(a0), v2)) {
    v1 = sub_12ec0(v2,0);
    if (0 <= (int)v1)
      return (unsigned long)v1;
    return 0;
  }
  return __snprintf_chk(NULL,0,1,0xffffffffffffffff,"%lu",a0 & 0xffffffff); // tail-call
}

// Function: sub_8640 @ 0x8640
unsigned int sub_8640(void)
{
  unsigned int v1; // eax
  unsigned int v2;
  
  v1 = (-(unsigned int)(dat_262fc == '\0') & 0xffffff00) + 0x102;
  if (dat_2632c)
    v1 |= 0x400;
  if (dat_26338) { // branch-flip
    switch(dat_26330) {
      case 0:
      case 1:
      case 2:
      case 4:
      case 6:
label_86f0:
        return v1;
      case 3:
label_86f8:
        return v1 | 0x200;
      case 5:
        if (dat_26334 <= 3) {
          v2 = *(unsigned int *)&"@"[(unsigned long)dat_26334 * 4];
          return v1 | v2; // return-dupe
        }
      
    }
  }
  else if (dat_26334 <= 3) {
    v2 = *(unsigned int *)&"@"[(unsigned long)dat_26334 * 4];
    if ((dat_25029) || (dat_2632e))
      v1 = v1 | v2 | 0x20c;
    else {
      v1 = v1 | v2 | 0x204;
    }
    if (dat_25028)
      v1 |= 0x10;
    switch(dat_26330) {
      case 0:
      case 1:
      case 2:
      case 4:
      case 6:
        goto label_86f0;
      case 3:
        goto label_86f8;
      case 5:
        return v1 | v2;
      
    }
  }
  abort(); // no-return
}

// Function: sub_8730 @ 0x8730
void sub_8730(void)
{
  unsigned long *v1;
  unsigned long v2; // rbp
  
  if (dat_263b0) {
    v2 = 0;
    do {
      v1 = *(unsigned long **)(dat_263a0 + v2 * 8);
      free((void *)*v1);
      free((void *)v1[1]);
      free((void *)v1[2]);
      if ((char *)v1[0x16] != (char *)0x2502a)
        freecon((char *)v1[0x16]);
      v2 += 1;
    } while (v2 < dat_263b0);
  }
  dat_263a9 = 0;
  dat_263b0 = 0;
  dat_26364 = 0;
  dat_26360 = 0;
  dat_2635c = 0;
  dat_26358 = 0;
  dat_26350 = 0;
  dat_2634c = 0;
  dat_26348 = 0;
  dat_26354 = 0;
  dat_26344 = 0;
  dat_26340 = 0;
  dat_2633c = 0;
}

// Function: sub_8820 @ 0x8820
void sub_8820(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  
  v1 = (char *)*a1;
  v2 = (char *)*a0;
  *__errno_location() = 0;
  strcoll(v2,v1); // tail-call
}

// Function: sub_8850 @ 0x8850
void sub_8850(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  
  v1 = (char *)*a0;
  v2 = (char *)*a1;
  *__errno_location() = 0;
  strcoll(v2,v1); // tail-call
}

// Function: sub_8880 @ 0x8880
int sub_8880(struct_22 *a0,struct_22 *a1)
{
  char *v1;
  char *v2;
  
  if (a1->field_0x48 < a0->field_0x48)
    return 0xffffffff;
  if (a0->field_0x48 == a1->field_0x48) {
    v1 = a1->field_0x0;
    v2 = a0->field_0x0;
    return strcmp(v2,v1); // tail-call
  }
  return 1;
}

// Function: sub_88c0 @ 0x88c0
int sub_88c0(struct_22 *a0,struct_22 *a1)
{
  char *v1;
  char *v2;
  
  if (a1->field_0x48 < a0->field_0x48)
    return 0xffffffff;
  if (a0->field_0x48 == a1->field_0x48) {
    v1 = a0->field_0x0;
    v2 = a1->field_0x0;
    *__errno_location() = 0;
    return strcoll(v1,v2); // tail-call
  }
  return 1;
}

// Function: sub_8920 @ 0x8920
int sub_8920(struct_22 *a0,struct_22 *a1)
{
  char *v1;
  char *v2;
  
  if (a0->field_0x48 < a1->field_0x48)
    return 0xffffffff;
  if (a1->field_0x48 == a0->field_0x48) {
    v1 = a1->field_0x0;
    v2 = a0->field_0x0;
    *__errno_location() = 0;
    return strcoll(v1,v2); // tail-call
  }
  return 1;
}

// Function: sub_8980 @ 0x8980
void sub_8980(struct_21 *a0,struct_21 *a1)
{
  long v1;
  long v2;
  
  v2 = a0->field_0xc8;
  if (!v2)
    v2 = sub_7d70(a0->field_0x0,dat_262d0,a0->field_0xc4);
  v1 = a1->field_0xc8;
  if (!v1)
    v1 = sub_7d70(a1->field_0x0,dat_262d0,a1->field_0xc4);
  if ((int)v2 == (int)v1) {
    strcmp(a0->field_0x0,a1->field_0x0); // tail-call
    return;
  }
}

// Function: sub_8a00 @ 0x8a00
void sub_8a00(struct_24 *a0,struct_24 *a1)
{
  char *v1;
  char *v2;
  
  if (!(((unsigned int)(a0->field_0x68 < a1->field_0x68) - (unsigned int)(a1->field_0x68 < a0->field_0x68)) + ((unsigned int)(a0->field_0x60 < a1->field_0x60) - (unsigned int)(a1->field_0x60 < a0->field_0x60)) * 2)) {
    v1 = a0->field_0x0;
    v2 = a1->field_0x0;
    *__errno_location() = 0;
    strcoll(v1,v2); // tail-call
    return;
  }
}

// Function: sub_8a70 @ 0x8a70
void sub_8a70(struct_25 *a0,struct_25 *a1)
{
  char *v1;
  char *v2;
  
  if (!(((unsigned int)(a1->field_0x78 < a0->field_0x78) - (unsigned int)(a0->field_0x78 < a1->field_0x78)) + ((unsigned int)(a1->field_0x70 < a0->field_0x70) - (unsigned int)(a0->field_0x70 < a1->field_0x70)) * 2)) {
    v1 = a1->field_0x0;
    v2 = a0->field_0x0;
    *__errno_location() = 0;
    strcoll(v1,v2); // tail-call
    return;
  }
}

// Function: sub_8ae0 @ 0x8ae0
void sub_8ae0(struct_23 *a0,struct_23 *a1)
{
  char *v1;
  char *v2;
  
  if (!(((unsigned int)(a1->field_0x88 < a0->field_0x88) - (unsigned int)(a0->field_0x88 < a1->field_0x88)) + ((unsigned int)(a1->field_0x80 < a0->field_0x80) - (unsigned int)(a0->field_0x80 < a1->field_0x80)) * 2)) {
    v1 = a1->field_0x0;
    v2 = a0->field_0x0;
    *__errno_location() = 0;
    strcoll(v1,v2); // tail-call
    return;
  }
}

// Function: sub_8b60 @ 0x8b60
unsigned long sub_8b60(void *a0,void *a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)((long)a0 + 0xa8);
  if (((*(int *)((long)a1 + 0xa8) != 3) && (*(int *)((long)a1 + 0xa8) != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_8ae0(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_8ae0(a0,a1);
}

// Function: sub_8bf0 @ 0x8bf0
void sub_8bf0(struct_25 *a0,struct_25 *a1)
{
  char *v1;
  char *v2;
  
  if (!(((unsigned int)(a1->field_0x78 < a0->field_0x78) - (unsigned int)(a0->field_0x78 < a1->field_0x78)) + ((unsigned int)(a1->field_0x70 < a0->field_0x70) - (unsigned int)(a0->field_0x70 < a1->field_0x70)) * 2)) {
    v1 = a1->field_0x0;
    v2 = a0->field_0x0;
    *__errno_location() = 0;
    strcoll(v1,v2); // tail-call
    return;
  }
}

// Function: sub_8c60 @ 0x8c60
void sub_8c60(struct_25 *a0,struct_25 *a1)
{
  char *v1;
  char *v2;
  
  if (!(((unsigned int)(a0->field_0x78 < a1->field_0x78) - (unsigned int)(a1->field_0x78 < a0->field_0x78)) + ((unsigned int)(a0->field_0x70 < a1->field_0x70) - (unsigned int)(a1->field_0x70 < a0->field_0x70)) * 2)) {
    v1 = a0->field_0x0;
    v2 = a1->field_0x0;
    *__errno_location() = 0;
    strcoll(v1,v2); // tail-call
    return;
  }
}

// Function: sub_8cd0 @ 0x8cd0
void sub_8cd0(struct_25 *a0,struct_25 *a1)
{
  char *v1;
  char *v2;
  
  if (!(((unsigned int)(a0->field_0x78 < a1->field_0x78) - (unsigned int)(a1->field_0x78 < a0->field_0x78)) + ((unsigned int)(a0->field_0x70 < a1->field_0x70) - (unsigned int)(a1->field_0x70 < a0->field_0x70)) * 2)) {
    v1 = a0->field_0x0;
    v2 = a1->field_0x0;
    *__errno_location() = 0;
    strcoll(v1,v2); // tail-call
    return;
  }
}

// Function: sub_8d40 @ 0x8d40
void sub_8d40(struct_23 *a0,struct_23 *a1)
{
  char *v1;
  char *v2;
  
  if (!(((unsigned int)(a0->field_0x88 < a1->field_0x88) - (unsigned int)(a1->field_0x88 < a0->field_0x88)) + ((unsigned int)(a0->field_0x80 < a1->field_0x80) - (unsigned int)(a1->field_0x80 < a0->field_0x80)) * 2)) {
    v1 = a0->field_0x0;
    v2 = a1->field_0x0;
    *__errno_location() = 0;
    strcoll(v1,v2); // tail-call
    return;
  }
}

// Function: sub_8dc0 @ 0x8dc0
void sub_8dc0(struct_24 *a0,struct_24 *a1)
{
  char *v1;
  char *v2;
  
  if (!(((unsigned int)(a1->field_0x68 < a0->field_0x68) - (unsigned int)(a0->field_0x68 < a1->field_0x68)) + ((unsigned int)(a1->field_0x60 < a0->field_0x60) - (unsigned int)(a0->field_0x60 < a1->field_0x60)) * 2)) {
    v1 = a1->field_0x0;
    v2 = a0->field_0x0;
    *__errno_location() = 0;
    strcoll(v1,v2); // tail-call
    return;
  }
}

// Function: sub_8e30 @ 0x8e30
void sub_8e30(void) // return-dupe
{
  unsigned long *v1;
  unsigned int v2;
  int v3;
  long *v4;
  long v5;
  unsigned long v6;
  
  v6 = dat_263b0;
  if (dat_26398 < (dat_263b0 >> 1) + dat_263b0) {
    free(dat_263a0);
    dat_263a0 = (long *)sub_186d0(v6,0x18);
    dat_26398 = dat_263b0 * 3;
  }
  if (dat_263b0) { // branch-flip
    v4 = dat_263a0;
    v5 = dat_263c0;
    do {
      *v4 = v5;
      v4 = &v4[1];
      v5 += 0xd0;
    } while (v4 != &dat_263a0[dat_263b0]);
    if (dat_26330 != 2) goto label_8ec2;
label_8fbb:
    v6 = 0;
    do {
      v1 = (unsigned long *)dat_263a0[v6];
      v5 = v1[0x19];
      if (!v5)
        v5 = sub_7d70((char *)*v1,dat_262d0,*(unsigned int *)((long)v1 + 0xc4));
      v6 += 1;
      v1[0x19] = v5;
    } while (v6 < dat_263b0);
    v2 = dat_26330;
  }
  else {
    if (dat_26330 == 2) goto label_8ed9;
label_8ec2:
    v2 = dat_26330;
    if (((dat_262b0) && ((unsigned int)(dat_26338 - 2U) <= 1)) && (dat_263b0)) goto label_8fbb;
  }
  if (v2 == 6)
    return;
label_8ed9:
  if (_setjmp((void *)0x25300)) { // branch-flip
    if (dat_26330 == 4)
      __assert_fail("sort_type != sort_version","src/ls.c",0x1008,"sort_files"); // no-return
    if (dat_263b0) {
      v4 = dat_263a0;
      v5 = dat_263c0;
      do {
        *v4 = v5;
        v4 = &v4[1];
        v5 += 0xd0;
      } while (&dat_263a0[dat_263b0] != v4);
    }
    v3 = 1;
  }
  else {
    v3 = 0;
  }
  v2 = dat_26330;
  if (dat_26330 == 5)
    v2 = dat_26334 + 5;
  sub_131b0(dat_263a0,dat_263b0,*(void **)(((unsigned long)dat_262f4 + ((unsigned long)dat_2632f + ((long)v3 + (unsigned long)v2 * 2) * 2) * 2) * 8 + 0x23fa0));
}

// Function: sub_9050 @ 0x9050
void sub_9050(struct_25 *a0,struct_25 *a1)
{
  if (!(((unsigned int)(a0->field_0x78 < a1->field_0x78) - (unsigned int)(a1->field_0x78 < a0->field_0x78)) + ((unsigned int)(a0->field_0x70 < a1->field_0x70) - (unsigned int)(a1->field_0x70 < a0->field_0x70)) * 2)) {
    strcmp(a0->field_0x0,a1->field_0x0); // tail-call
    return;
  }
}

// Function: sub_90a0 @ 0x90a0
void sub_90a0(struct_24 *a0,struct_24 *a1)
{
  if (!(((unsigned int)(a0->field_0x68 < a1->field_0x68) - (unsigned int)(a1->field_0x68 < a0->field_0x68)) + ((unsigned int)(a0->field_0x60 < a1->field_0x60) - (unsigned int)(a1->field_0x60 < a0->field_0x60)) * 2)) {
    strcmp(a0->field_0x0,a1->field_0x0); // tail-call
    return;
  }
}

// Function: sub_90f0 @ 0x90f0
void sub_90f0(struct_23 *a0,struct_23 *a1)
{
  if (!(((unsigned int)(a0->field_0x88 < a1->field_0x88) - (unsigned int)(a1->field_0x88 < a0->field_0x88)) + ((unsigned int)(a0->field_0x80 < a1->field_0x80) - (unsigned int)(a1->field_0x80 < a0->field_0x80)) * 2)) {
    strcmp(a0->field_0x0,a1->field_0x0); // tail-call
    return;
  }
}

// Function: sub_9150 @ 0x9150
void sub_9150(struct_25 *a0,struct_25 *a1)
{
  if (!(((unsigned int)(a0->field_0x78 < a1->field_0x78) - (unsigned int)(a1->field_0x78 < a0->field_0x78)) + ((unsigned int)(a0->field_0x70 < a1->field_0x70) - (unsigned int)(a1->field_0x70 < a0->field_0x70)) * 2)) {
    strcmp(a0->field_0x0,a1->field_0x0); // tail-call
    return;
  }
}

// Function: sub_91a0 @ 0x91a0
void sub_91a0(struct_21 *a0,struct_21 *a1)
{
  char *v1;
  char *v2;
  int *v3; // rax
  long v4;
  long v5;
  
  v5 = a1->field_0xc8;
  if (!v5)
    v5 = sub_7d70(a1->field_0x0,dat_262d0,a1->field_0xc4);
  v4 = a0->field_0xc8;
  if (!v4)
    v4 = sub_7d70(a0->field_0x0,dat_262d0,a0->field_0xc4);
  if ((int)v5 == (int)v4) {
    v3 = __errno_location();
    v1 = a1->field_0x0;
    v2 = a0->field_0x0;
    *v3 = 0;
    strcoll(v1,v2); // tail-call
    return;
  }
}

// Function: sub_9230 @ 0x9230
void sub_9230(struct_21 *a0,struct_21 *a1)
{
  char *v1;
  char *v2;
  int *v3; // rax
  long v4;
  long v5;
  
  v5 = a0->field_0xc8;
  if (!v5)
    v5 = sub_7d70(a0->field_0x0,dat_262d0,a0->field_0xc4);
  v4 = a1->field_0xc8;
  if (!v4)
    v4 = sub_7d70(a1->field_0x0,dat_262d0,a1->field_0xc4);
  if ((int)v5 == (int)v4) {
    v3 = __errno_location();
    v1 = a0->field_0x0;
    v2 = a1->field_0x0;
    *v3 = 0;
    strcoll(v1,v2); // tail-call
    return;
  }
}

// Function: sub_92c0 @ 0x92c0
void sub_92c0(char *a0,unsigned long a1,int a2)
{
  char *v1;
  bool v2; // cf
  
  if (a0) {
    a2 -= sub_12ec0(a0,0);
    if (a2 <= -1)
      a2 = 0;
    sub_7430(a0,strlen(a0));
    do {
      dat_261f8 += 1;
      v1 = *(char **)&stdout->field_0x28;
      if (*(char **)&stdout->field_0x30 <= v1) // branch-flip
        __overflow(stdout,0x20);
      else {
        *(char **)&stdout->field_0x28 = &v1[1];
        *v1 = 0x20;
      }
      v2 = a2 != 0;
      a2 -= 1;
    } while (v2);
    return;
  }
  dat_261f8 += __printf_chk(1,"%*lu ",a2,a1);
}

// Function: sub_9370 @ 0x9370
void sub_9370(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  char *v3; // rax
  char *v4; // rax
  
  v1 = (char *)*a0;
  v3 = strrchr(v1,0x2e);
  v2 = (char *)*a1;
  v4 = strrchr(v2,0x2e);
  if (!v4)
    v4 = "";
  if (!v3)
    v3 = "";
  *__errno_location() = 0;
  if (!strcoll(v3,v4)) {
    strcoll(v1,v2); // tail-call
    return;
  }
}

// Function: sub_9400 @ 0x9400
void sub_9400(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  char *v3; // rax
  char *v4; // rax
  
  v1 = (char *)*a1;
  v3 = strrchr(v1,0x2e);
  v2 = (char *)*a0;
  v4 = strrchr(v2,0x2e);
  if (!v4)
    v4 = "";
  if (!v3)
    v3 = "";
  *__errno_location() = 0;
  if (!strcoll(v3,v4)) {
    strcoll(v1,v2); // tail-call
    return;
  }
}

// Function: sub_9490 @ 0x9490
void sub_9490(struct_21 *a0,struct_21 *a1)
{
  long v1;
  long v2;
  
  v2 = a1->field_0xc8;
  if (!v2)
    v2 = sub_7d70(a1->field_0x0,dat_262d0,a1->field_0xc4);
  v1 = a0->field_0xc8;
  if (!v1)
    v1 = sub_7d70(a0->field_0x0,dat_262d0,a0->field_0xc4);
  if ((int)v2 == (int)v1) {
    strcmp(a1->field_0x0,a0->field_0x0); // tail-call
    return;
  }
}

// Function: sub_9510 @ 0x9510
void sub_9510(char *a0,char a1) // return-dupe
{
  int v1;
  unsigned long *v2;
  unsigned long v3; // rax
  char *v4; // rax
  char *v5;
  long *v6;
  long v7;
  bool v8; // cf
  
  if ((a0) && (dat_263c8)) {
    v2 = (unsigned long *)sub_18590(0x20);
    v3 = sub_18a70(a0);
    *v2 = 0;
    v2[1] = v3;
    *(char *)&v2[2] = 0;
    v2[3] = dat_26380;
    dat_26380 = v2;
  }
  v7 = dat_263b0 + -1;
  if (!dat_263b0) {
    dat_263b0 = 0;
    return;
  }
  do {
    while( true ) {
      v2 = (unsigned long *)dat_263a0[v7];
      if ((*(int *)&v2[0x15] == 3) || (*(int *)&v2[0x15] == 9)) break;
label_9588:
      v8 = v7 == 0;
      v7 -= 1;
      if (v8) goto label_9640;
    }
    v5 = (char *)*v2;
    if (a0) { // branch-flip
      v4 = (char *)sub_ee20(v5);
      if ((*v4 == '.') && ((!v4[(unsigned long)(v4[1] == '.') + 1] || (v4[(unsigned long)(v4[1] == '.') + 1] == '/')))) goto label_9588;
      if (*v5 == '/') goto label_96a0;
      v5 = (char *)sub_fbb0(a0,v5,NULL);
      sub_6fb0(v5,(char *)v2[1],(unsigned char)a1);
      free(v5);
    }
    else {
label_96a0:
      sub_6fb0(v5,(char *)v2[1],(unsigned char)a1);
    }
    if (*(int *)&v2[0x15] != 9) goto label_9588;
    free((void *)*v2);
    free((void *)v2[1]);
    free((void *)v2[2]);
    if ((char *)v2[0x16] == (char *)0x2502a) goto label_9588;
    freecon((char *)v2[0x16]);
    v8 = v7 != 0;
    v7 -= 1;
  } while (v8);
label_9640:
  if (!dat_263b0) {
    dat_263b0 = 0;
    return;
  }
  v7 = 0;
  v6 = dat_263a0;
  do {
    v1 = *(int *)(*v6 + 0xa8);
    dat_263a0[v7] = *v6;
    v6 = &v6[1];
    v7 += (unsigned long)(v1 != 9);
  } while (&dat_263a0[dat_263b0] != v6);
  dat_263b0 = v7;
}

// Function: sub_96f0 @ 0x96f0
unsigned long sub_96f0(char *a0,unsigned int a1,unsigned int a2,char *a3)
{
  char *v1;
  char *v10;
  unsigned int v11 [2]; // stack - 0x398
  char v12 [664]; // stack - 0x2d8
  unsigned int *v13;
  unsigned int *v14; // rsp
  char v15;
  unsigned long *v16;
  char v17;
  char *v18;
  char v19; // stack - 0x38e
  unsigned long *v2;
  char v20; // stack - 0x38d
  unsigned int v21; // stack - 0x38c
  unsigned long *v3;
  char v4;
  unsigned long v5; // rax
  long v6;
  char *v7;
  char *v8;
  unsigned long v9;
  
  v13 = v11;
  v14 = v11;
  v6 = dat_263c0;
  v21 = a2;
  if (dat_263b0 == dat_263b8) {
    v6 = sub_18650(dat_263c0,dat_263b0,0x1a0);
    dat_263b8 <<= 1;
    dat_263c0 = v6;
  }
  v2 = (unsigned long *)(v6 + dat_263b0 * 0xd0);
  *v2 = 0;
  v2[0x19] = 0;
  v9 = (unsigned long)(((int)v2 - (int)(unsigned long *)((unsigned long)&v2[1] & 0xfffffffffffffff8)) + 0xd0U >> 3);
  v16 = (unsigned long *)((unsigned long)&v2[1] & 0xfffffffffffffff8);
  while (v9) {
    v3 = &v16[1];
    *v16 = 0;
    v9 -= 1;
    v16 = v3;
  }
  *(unsigned int *)&v2[0x15] = a1;
  *(unsigned int *)((long)v2 + 0xc4) = 0xffffffff;
  if ((!dat_263a9) && (dat_263a8)) {
    v9 = sub_16a10(v12,2,a0,0xffffffffffffffff,dat_262d0);
    if ((*a0 != v12[0]) || (v9 != strlen(a0))) {
      *(unsigned int *)((long)v2 + 0xc4) = 1;
      dat_263a9 = '\x01';
    }
    else {
      *(unsigned int *)((long)v2 + 0xc4) = 0;
    }
  }
  if ((char)v21) { // branch-flip
    v13 = v11;
    v7 = a0;
    v15 = dat_26311;
    if (*a0 != '/') {
      v17 = *a3;
      v13 = v11;
      v4 = *a0;
      if (v17) goto label_9a41;
    }
label_97d0:
    a0 = v7;
    if (!v15) goto label_97d9;
  }
  else {
    if (!dat_26311) {
      v15 = dat_26311;
      if (dat_262a1) { // branch-flip
label_9a20:
        v4 = *a0;
        v13 = v11;
        if (v4 == '/') goto label_97d9;
      }
      else {
        if ((a1 == 3) && (dat_26312)) {
          v11[0] &= 0xffffff00;
          v4 = sub_6c80(0x13);
          v15 = (char)v11[0];
          if (!v4) {
            v4 = sub_6c80(0x12);
            v15 = (char)v11[0];
            if (!v4) {
              v4 = sub_6c80(0x14);
              v15 = (char)v11[0];
              if (!v4) goto label_9930;
            }
          }
          goto label_9a20;
        }
label_9930:
        if (!dat_262fc) {
          if (!dat_262a0) goto label_98d7;
          if (0x41UL >> ((unsigned long)a1 & 0x3f) & 1) goto label_9957;
label_9986:
          if (!dat_262a0) {
label_98d7:
            v5 = sub_18a70(a0);
            dat_263b0 += 1;
            *v2 = v5;
            return 0;
          }
          if (a1) {
            if (a1 != 5) goto label_98d7;
            if (dat_26314 != 3) {
              v11[0] = CONCAT31(v11[0]._1_3_,v15);
              if (!dat_26312) goto label_98d7;
              v4 = sub_6c80(0xe);
              v15 = (char)v11[0];
              if (!v4) {
                v4 = sub_6c80(0x10);
                v15 = (char)v11[0];
                if (!v4) {
                  v4 = sub_6c80(0x11);
                  v15 = (char)v11[0];
                  if (!v4) {
                    v4 = sub_6c80(0x15);
                    v15 = (char)v11[0];
                    if (!v4) goto label_98d7;
                  }
                }
              }
            }
          }
          goto label_9a20;
        }
        if (!(0x41UL >> ((unsigned long)a1 & 0x3f) & 1)) goto label_9a20;
label_9957:
        if (dat_262f8 != 4) {
          if (((!dat_26390) && (!dat_262fd)) && (!dat_262fc)) goto label_9986;
          goto label_9a20;
        }
        v4 = *a0;
        if (v4 == '/')
          sub_8640(); // no-return
      }
      v17 = *a3;
      v13 = v11;
      if (!v17) goto label_97d9;
label_9a41:
      v20 = v4;
      v19 = v15;
      v11[0] = CONCAT31(v11[0]._1_3_,v17);
      v9 = strlen(a0);
      v9 = v9 + 0x19 + strlen(a3);
      for (; v14 != (unsigned int *)((long)v11 - (v9 & 0xfffffffffffff000)); v14 = (char *)((long)v14 + -0x1000)) {
        *(unsigned long *)((long)v14 + -8) = *(unsigned long *)((long)v14 + -8);
      }
      v9 = (unsigned long)((unsigned int)v9 & 0xff0);
      v6 = -v9;
      v13 = (char *)((long)v14 + v6);
      if (v9)
        *(unsigned long *)((long)v14 + -8) = *(unsigned long *)((long)v14 + -8);
      v7 = (char *)((unsigned long)((long)v14 + v6 + 0xf) & 0xfffffffffffffff0);
      v15 = v20;
      if (((char)v11[0] != '.') || (v8 = v7, a3[1])) {
        v10 = a3;
        v18 = v7;
        v4 = (char)v11[0];
        v17 = a3[1];
        while( true ) {
          v8 = &v18[1];
          *v18 = v4;
          if (!v17) break;
          v1 = &v10[2];
          v10 = &v10[1];
          v18 = v8;
          v4 = v17;
          v17 = *v1;
        }
        if ((a3 < &v10[1]) && (*v10 != '/')) {
          *v8 = '/';
          v8 = &v18[2];
        }
      }
      while (v15) {
        a0 = &a0[1];
        *v8 = v15;
        v8 = &v8[1];
        v15 = *a0;
      }
      *v8 = '\0';
      v15 = v19;
      goto label_97d0;
    }
    if (*a0 != '/') {
      v17 = *a3;
      v13 = v11;
      v15 = dat_26311;
      v4 = *a0;
      if (v17) goto label_9a41;
    }
  }
  *(unsigned long *)((long)v13 + -8) = 0x9efb;
  v6 = sub_ef30(a0,2);
  v2[2] = v6;
  if (!v6) {
    *(unsigned long *)((long)v13 + -8) = 0x9f1b;
    v7 = dcgettext(NULL,"error canonicalizing %s",5);
    *(unsigned long *)((long)v13 + -8) = 0x9f2f;
    sub_7540((unsigned char)(char)v21,v7,a0);
  }
label_97d9:
  v11[0] = dat_262f8;
  *(unsigned long *)((long)v13 + -8) = 0x97f0; // no-return
  sub_8640();
}

// Function: sub_a660 @ 0xa660
unsigned long sub_a660(struct_22 *a0,struct_22 *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  int v4;
  
  v4 = a0->field_0xa8;
  if (((a1->field_0xa8 != 3) && (a1->field_0xa8 != 9)) && ((a1->field_0xac & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
  }
  else {
    if (v4 == 9 || v4 == 3) goto label_a6e0;
    v4 = 1;
  }
  v3 = v4 - (unsigned int)((a0->field_0xac & 0xf000) == 0x4000);
  if (v3)
    return (unsigned long)v3;
label_a6e0:
  if (a0->field_0x48 < a1->field_0x48)
    return 0xffffffff;
  if (a1->field_0x48 != a0->field_0x48)
    return 1;
  v1 = a0->field_0x0;
  v2 = a1->field_0x0;
  return strcmp(v2,v1); // tail-call
}

// Function: sub_a710 @ 0xa710
unsigned long sub_a710(struct_23 *a0,struct_23 *a1)
{
  char *v1;
  char *v2;
  unsigned int v3;
  int v4;
  
  v4 = a0->field_0xa8;
  if (((a1->field_0xa8 != 3) && (a1->field_0xa8 != 9)) && ((a1->field_0xac & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
label_a771:
    v3 = v4 - (unsigned int)((a0->field_0xac & 0xf000) == 0x4000);
    if (v3) {
      return (unsigned long)v3;
    }
  }
  else if (v4 != 9 && v4 != 3) {
    v4 = 1;
    goto label_a771;
  }
  v3 = ((unsigned int)(a1->field_0x88 < a0->field_0x88) - (unsigned int)(a0->field_0x88 < a1->field_0x88)) + ((unsigned int)(a1->field_0x80 < a0->field_0x80) - (unsigned int)(a0->field_0x80 < a1->field_0x80)) * 2;
  if (!v3) {
    v1 = a0->field_0x0;
    v2 = a1->field_0x0;
    return strcmp(v2,v1); // tail-call
  }
  return (unsigned long)v3;
}

// Function: sub_a7f0 @ 0xa7f0
unsigned long sub_a7f0(struct_24 *a0,struct_24 *a1)
{
  char *v1;
  char *v2;
  unsigned int v3;
  int v4;
  
  v4 = a0->field_0xa8;
  if (((a1->field_0xa8 != 3) && (a1->field_0xa8 != 9)) && ((a1->field_0xac & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
label_a849:
    v3 = v4 - (unsigned int)((a0->field_0xac & 0xf000) == 0x4000);
    if (v3) {
      return (unsigned long)v3;
    }
  }
  else if (v4 != 9 && v4 != 3) {
    v4 = 1;
    goto label_a849;
  }
  v3 = ((unsigned int)(a1->field_0x68 < a0->field_0x68) - (unsigned int)(a0->field_0x68 < a1->field_0x68)) + ((unsigned int)(a1->field_0x60 < a0->field_0x60) - (unsigned int)(a0->field_0x60 < a1->field_0x60)) * 2;
  if (!v3) {
    v1 = a0->field_0x0;
    v2 = a1->field_0x0;
    return strcmp(v2,v1); // tail-call
  }
  return (unsigned long)v3;
}

// Function: sub_a8c0 @ 0xa8c0
unsigned long sub_a8c0(struct_22 *a0,struct_22 *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  int v4;
  
  v4 = a0->field_0xa8;
  if (((a1->field_0xa8 != 3) && (a1->field_0xa8 != 9)) && ((a1->field_0xac & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
  }
  else {
    if (v4 == 9 || v4 == 3) {
      v1 = a1->field_0x0; // return-dupe
      v2 = a0->field_0x0;
      return strcmp(v2,v1); // tail-call
    }
    v4 = 1;
  }
  v3 = v4 - (unsigned int)((a0->field_0xac & 0xf000) == 0x4000);
  if (v3)
    return (unsigned long)v3;
  v1 = a1->field_0x0;
  v2 = a0->field_0x0;
  return strcmp(v2,v1);
}

// Function: sub_a960 @ 0xa960
unsigned long sub_a960(struct_22 *a0,struct_22 *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  int v4;
  
  v4 = a0->field_0xa8;
  if (((a1->field_0xa8 != 3) && (a1->field_0xa8 != 9)) && ((a1->field_0xac & 0xf000) != 0x4000)) {
    if (v4 == 3 || v4 == 9)
      return 0xffffffff;
    v4 = 0;
  }
  else {
    if (v4 == 3 || v4 == 9) goto label_a9e0;
    v4 = 1;
  }
  v3 = v4 - (unsigned int)((a0->field_0xac & 0xf000) == 0x4000);
  if (v3)
    return (unsigned long)v3;
label_a9e0:
  v1 = a0->field_0x0;
  v2 = a1->field_0x0;
  *__errno_location() = 0;
  return strcoll(v2,v1); // tail-call
}

// Function: sub_aa20 @ 0xaa20
unsigned long sub_aa20(struct_22 *a0,struct_22 *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  int v4;
  unsigned long v5; // rax
  
  v4 = a0->field_0xa8;
  if (((a1->field_0xa8 != 3) && (a1->field_0xa8 != 9)) && ((a1->field_0xac & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
  }
  else {
    if (v4 == 9 || v4 == 3) goto label_aaa0;
    v4 = 1;
  }
  v3 = v4 - (unsigned int)((a0->field_0xac & 0xf000) == 0x4000);
  if (v3)
    return (unsigned long)v3;
label_aaa0:
  v1 = a1->field_0x0;
  v2 = a0->field_0x0;
  v5 = sub_101e0(v1,v2);
  if (!(int)v5)
    return strcmp(v1,v2); // tail-call
  return v5;
}

// Function: sub_aaf0 @ 0xaaf0
unsigned long sub_aaf0(struct_22 *a0,struct_22 *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  int v4;
  unsigned long v5; // rax
  
  v4 = a0->field_0xa8;
  if (((a1->field_0xa8 != 3) && (a1->field_0xa8 != 9)) && ((a1->field_0xac & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
  }
  else {
    if (v4 == 9 || v4 == 3) goto label_ab70;
    v4 = 1;
  }
  v3 = v4 - (unsigned int)((a0->field_0xac & 0xf000) == 0x4000);
  if (v3)
    return (unsigned long)v3;
label_ab70:
  v1 = a0->field_0x0;
  v2 = a1->field_0x0;
  v5 = sub_101e0(v1,v2);
  if (!(int)v5)
    return strcmp(v1,v2); // tail-call
  return v5;
}

// Function: sub_abc0 @ 0xabc0
unsigned long sub_abc0(struct_22 *a0,struct_22 *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  int v4;
  
  v4 = a0->field_0xa8;
  if (((a1->field_0xa8 != 3) && (a1->field_0xa8 != 9)) && ((a1->field_0xac & 0xf000) != 0x4000)) {
    if (v4 == 3 || v4 == 9)
      return 0xffffffff;
    v4 = 0;
  }
  else {
    if (v4 == 3 || v4 == 9) goto label_ac40;
    v4 = 1;
  }
  v3 = v4 - (unsigned int)((a0->field_0xac & 0xf000) == 0x4000);
  if (v3)
    return (unsigned long)v3;
label_ac40:
  v1 = a1->field_0x0;
  v2 = a0->field_0x0;
  *__errno_location() = 0;
  return strcoll(v2,v1); // tail-call
}

// Function: sub_ac80 @ 0xac80
unsigned long sub_ac80(void *a0,void *a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)((long)a0 + 0xa8);
  if (((*(int *)((long)a1 + 0xa8) != 3) && (*(int *)((long)a1 + 0xa8) != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_8cd0(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_8cd0(a0,a1);
}

// Function: sub_ad10 @ 0xad10
unsigned long sub_ad10(void *a0,void *a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)((long)a0 + 0xa8);
  if (((*(int *)((long)a1 + 0xa8) != 3) && (*(int *)((long)a1 + 0xa8) != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_8d40(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_8d40(a0,a1);
}

// Function: sub_ada0 @ 0xada0
unsigned long sub_ada0(void *a0,void *a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)((long)a0 + 0xa8);
  if (((*(int *)((long)a1 + 0xa8) != 3) && (*(int *)((long)a1 + 0xa8) != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_9490(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_9490(a0,a1);
}

// Function: sub_ae30 @ 0xae30
unsigned long sub_ae30(struct_25 *a0,struct_25 *a1)
{
  char *v1;
  char *v2;
  unsigned int v3;
  int v4;
  
  v4 = a0->field_0xa8;
  if (((a1->field_0xa8 != 3) && (a1->field_0xa8 != 9)) && ((a1->field_0xac & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
label_ae89:
    v3 = v4 - (unsigned int)((a0->field_0xac & 0xf000) == 0x4000);
    if (v3) {
      return (unsigned long)v3;
    }
  }
  else if (v4 != 9 && v4 != 3) {
    v4 = 1;
    goto label_ae89;
  }
  v3 = ((unsigned int)(a1->field_0x78 < a0->field_0x78) - (unsigned int)(a0->field_0x78 < a1->field_0x78)) + ((unsigned int)(a1->field_0x70 < a0->field_0x70) - (unsigned int)(a0->field_0x70 < a1->field_0x70)) * 2;
  if (!v3) {
    v1 = a0->field_0x0;
    v2 = a1->field_0x0;
    return strcmp(v2,v1); // tail-call
  }
  return (unsigned long)v3;
}

// Function: sub_af00 @ 0xaf00
unsigned long sub_af00(struct_25 *a0,struct_25 *a1)
{
  char *v1;
  char *v2;
  unsigned int v3;
  int v4;
  
  v4 = a0->field_0xa8;
  if (((a1->field_0xa8 != 3) && (a1->field_0xa8 != 9)) && ((a1->field_0xac & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
label_af59:
    v3 = v4 - (unsigned int)((a0->field_0xac & 0xf000) == 0x4000);
    if (v3) {
      return (unsigned long)v3;
    }
  }
  else if (v4 != 9 && v4 != 3) {
    v4 = 1;
    goto label_af59;
  }
  v3 = ((unsigned int)(a0->field_0x78 < a1->field_0x78) - (unsigned int)(a1->field_0x78 < a0->field_0x78)) + ((unsigned int)(a0->field_0x70 < a1->field_0x70) - (unsigned int)(a1->field_0x70 < a0->field_0x70)) * 2;
  if (!v3) {
    v1 = a1->field_0x0;
    v2 = a0->field_0x0;
    return strcmp(v2,v1); // tail-call
  }
  return (unsigned long)v3;
}

// Function: sub_afd0 @ 0xafd0
unsigned long sub_afd0(void *a0,void *a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)((long)a0 + 0xa8);
  if (((*(int *)((long)a1 + 0xa8) != 3) && (*(int *)((long)a1 + 0xa8) != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_8bf0(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_8bf0(a0,a1);
}

// Function: sub_b060 @ 0xb060
unsigned long sub_b060(void *a0,void *a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)((long)a0 + 0xa8);
  if (((*(int *)((long)a1 + 0xa8) != 3) && (*(int *)((long)a1 + 0xa8) != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_9230(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_9230(a0,a1);
}

// Function: sub_b0f0 @ 0xb0f0
unsigned long sub_b0f0(unsigned long *a0,unsigned long *a1)
{
  unsigned int v1; // eax
  int v2;
  unsigned long v3; // rax
  
  v2 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      v3 = sub_71f0(a0,a1,dat_24fd8); // tail-call, return-dupe
      return v3;
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  v3 = sub_71f0(a0,a1,dat_24fd8);
  return v3;
}

// Function: sub_b190 @ 0xb190
unsigned long sub_b190(unsigned long *a0,unsigned long *a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_9400(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_9400(a0,a1);
}

// Function: sub_b220 @ 0xb220
unsigned long sub_b220(struct_22 *a0,struct_22 *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  int v4;
  
  v4 = a0->field_0xa8;
  if (((a1->field_0xa8 != 3) && (a1->field_0xa8 != 9)) && ((a1->field_0xac & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
  }
  else {
    if (v4 == 9 || v4 == 3) goto label_b2a0;
    v4 = 1;
  }
  v3 = v4 - (unsigned int)((a0->field_0xac & 0xf000) == 0x4000);
  if (v3)
    return (unsigned long)v3;
label_b2a0:
  if (a1->field_0x48 < a0->field_0x48)
    return 0xffffffff;
  if (a0->field_0x48 != a1->field_0x48)
    return 1;
  v1 = a1->field_0x0;
  v2 = a0->field_0x0;
  return strcmp(v2,v1); // tail-call
}

// Function: sub_b2d0 @ 0xb2d0
unsigned long sub_b2d0(struct_22 *a0,struct_22 *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  int v4;
  
  v4 = a0->field_0xa8;
  if (((a1->field_0xa8 != 3) && (a1->field_0xa8 != 9)) && ((a1->field_0xac & 0xf000) != 0x4000)) {
    if (v4 == 3 || v4 == 9)
      return 0xffffffff;
    v4 = 0;
  }
  else {
    if (v4 == 3 || v4 == 9) goto label_b350;
    v4 = 1;
  }
  v3 = v4 - (unsigned int)((a0->field_0xac & 0xf000) == 0x4000);
  if (v3)
    return (unsigned long)v3;
label_b350:
  if (a0->field_0x48 < a1->field_0x48)
    return 0xffffffff;
  if (a1->field_0x48 != a0->field_0x48)
    return 1;
  v1 = a1->field_0x0;
  v2 = a0->field_0x0;
  *__errno_location() = 0;
  return strcoll(v1,v2); // tail-call
}

// Function: sub_b3a0 @ 0xb3a0
unsigned long sub_b3a0(void *a0,void *a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)((long)a0 + 0xa8);
  if (((*(int *)((long)a1 + 0xa8) != 3) && (*(int *)((long)a1 + 0xa8) != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_8980(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_8980(a0,a1);
}

// Function: sub_b430 @ 0xb430
unsigned long sub_b430(void *a0,void *a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)((long)a0 + 0xa8);
  if (((*(int *)((long)a1 + 0xa8) != 3) && (*(int *)((long)a1 + 0xa8) != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_91a0(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_91a0(a0,a1);
}

// Function: sub_b4c0 @ 0xb4c0
unsigned long sub_b4c0(unsigned long *a0,unsigned long *a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_9370(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_9370(a0,a1);
}

// Function: sub_b550 @ 0xb550
unsigned long sub_b550(struct_22 *a0,struct_22 *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  int v4;
  
  v4 = a0->field_0xa8;
  if (((a1->field_0xa8 != 3) && (a1->field_0xa8 != 9)) && ((a1->field_0xac & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
  }
  else {
    if (v4 == 9 || v4 == 3) {
      v1 = a0->field_0x0; // return-dupe
      v2 = a1->field_0x0;
      return strcmp(v2,v1); // tail-call
    }
    v4 = 1;
  }
  v3 = v4 - (unsigned int)((a0->field_0xac & 0xf000) == 0x4000);
  if (v3)
    return (unsigned long)v3;
  v1 = a0->field_0x0;
  v2 = a1->field_0x0;
  return strcmp(v2,v1);
}

// Function: sub_b5f0 @ 0xb5f0
unsigned long sub_b5f0(struct_22 *a0,struct_22 *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  int v4;
  
  v4 = a0->field_0xa8;
  if (((a1->field_0xa8 != 3) && (a1->field_0xa8 != 9)) && ((a1->field_0xac & 0xf000) != 0x4000)) {
    if (v4 == 3 || v4 == 9)
      return 0xffffffff;
    v4 = 0;
  }
  else {
    if (v4 == 3 || v4 == 9) goto label_b670;
    v4 = 1;
  }
  v3 = v4 - (unsigned int)((a0->field_0xac & 0xf000) == 0x4000);
  if (v3)
    return (unsigned long)v3;
label_b670:
  if (a1->field_0x48 < a0->field_0x48)
    return 0xffffffff;
  if (a0->field_0x48 != a1->field_0x48)
    return 1;
  v1 = a0->field_0x0;
  v2 = a1->field_0x0;
  *__errno_location() = 0;
  return strcoll(v1,v2); // tail-call
}

// Function: sub_b6c0 @ 0xb6c0
unsigned long sub_b6c0(struct_24 *a0,struct_24 *a1)
{
  char *v1;
  char *v2;
  unsigned int v3;
  int v4;
  
  v4 = a0->field_0xa8;
  if (((a1->field_0xa8 != 3) && (a1->field_0xa8 != 9)) && ((a1->field_0xac & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
label_b719:
    v3 = v4 - (unsigned int)((a0->field_0xac & 0xf000) == 0x4000);
    if (v3) {
      return (unsigned long)v3;
    }
  }
  else if (v4 != 9 && v4 != 3) {
    v4 = 1;
    goto label_b719;
  }
  v3 = ((unsigned int)(a0->field_0x68 < a1->field_0x68) - (unsigned int)(a1->field_0x68 < a0->field_0x68)) + ((unsigned int)(a0->field_0x60 < a1->field_0x60) - (unsigned int)(a1->field_0x60 < a0->field_0x60)) * 2;
  if (!v3) {
    v1 = a1->field_0x0;
    v2 = a0->field_0x0;
    return strcmp(v2,v1); // tail-call
  }
  return (unsigned long)v3;
}

// Function: sub_b790 @ 0xb790
unsigned long sub_b790(void *a0,void *a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)((long)a0 + 0xa8);
  if (((*(int *)((long)a1 + 0xa8) != 3) && (*(int *)((long)a1 + 0xa8) != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_8dc0(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_8dc0(a0,a1);
}

// Function: sub_b820 @ 0xb820
unsigned long sub_b820(unsigned long *a0,unsigned long *a1)
{
  unsigned int v1; // eax
  int v2;
  unsigned long v3; // rax
  
  v2 = *(int *)&a0[0x15];
  if (((*(int *)&a1[0x15] != 3) && (*(int *)&a1[0x15] != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      v3 = sub_71f0(a1,a0,dat_24fd8); // tail-call, return-dupe
      return v3;
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  v3 = sub_71f0(a1,a0,dat_24fd8);
  return v3;
}

// Function: sub_b8c0 @ 0xb8c0
unsigned long sub_b8c0(struct_25 *a0,struct_25 *a1)
{
  char *v1;
  char *v2;
  unsigned int v3;
  int v4;
  
  v4 = a0->field_0xa8;
  if (((a1->field_0xa8 != 3) && (a1->field_0xa8 != 9)) && ((a1->field_0xac & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
label_b919:
    v3 = v4 - (unsigned int)((a0->field_0xac & 0xf000) == 0x4000);
    if (v3) {
      return (unsigned long)v3;
    }
  }
  else if (v4 != 9 && v4 != 3) {
    v4 = 1;
    goto label_b919;
  }
  v3 = ((unsigned int)(a1->field_0x78 < a0->field_0x78) - (unsigned int)(a0->field_0x78 < a1->field_0x78)) + ((unsigned int)(a1->field_0x70 < a0->field_0x70) - (unsigned int)(a0->field_0x70 < a1->field_0x70)) * 2;
  if (!v3) {
    v1 = a0->field_0x0;
    v2 = a1->field_0x0;
    return strcmp(v2,v1); // tail-call
  }
  return (unsigned long)v3;
}

// Function: sub_b990 @ 0xb990
unsigned long sub_b990(struct_25 *a0,struct_25 *a1)
{
  char *v1;
  char *v2;
  unsigned int v3;
  int v4;
  
  v4 = a0->field_0xa8;
  if (((a1->field_0xa8 != 3) && (a1->field_0xa8 != 9)) && ((a1->field_0xac & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
label_b9e9:
    v3 = v4 - (unsigned int)((a0->field_0xac & 0xf000) == 0x4000);
    if (v3) {
      return (unsigned long)v3;
    }
  }
  else if (v4 != 9 && v4 != 3) {
    v4 = 1;
    goto label_b9e9;
  }
  v3 = ((unsigned int)(a0->field_0x78 < a1->field_0x78) - (unsigned int)(a1->field_0x78 < a0->field_0x78)) + ((unsigned int)(a0->field_0x70 < a1->field_0x70) - (unsigned int)(a1->field_0x70 < a0->field_0x70)) * 2;
  if (!v3) {
    v1 = a1->field_0x0;
    v2 = a0->field_0x0;
    return strcmp(v2,v1); // tail-call
  }
  return (unsigned long)v3;
}

// Function: sub_ba60 @ 0xba60
unsigned long sub_ba60(void *a0,void *a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)((long)a0 + 0xa8);
  if (((*(int *)((long)a1 + 0xa8) != 3) && (*(int *)((long)a1 + 0xa8) != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_8a70(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_8a70(a0,a1);
}

// Function: sub_baf0 @ 0xbaf0
unsigned long sub_baf0(void *a0,void *a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)((long)a0 + 0xa8);
  if (((*(int *)((long)a1 + 0xa8) != 3) && (*(int *)((long)a1 + 0xa8) != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_8a00(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_8a00(a0,a1);
}

// Function: sub_bb80 @ 0xbb80
unsigned long sub_bb80(void *a0,void *a1)
{
  unsigned int v1; // eax
  int v2;
  
  v2 = *(int *)((long)a0 + 0xa8);
  if (((*(int *)((long)a1 + 0xa8) != 3) && (*(int *)((long)a1 + 0xa8) != 9)) && ((*(unsigned int *)((long)a1 + 0xac) & 0xf000) != 0x4000)) {
    if (v2 == 9 || v2 == 3)
      return 0xffffffff;
    v2 = 0;
  }
  else {
    if (v2 == 9 || v2 == 3) {
      return sub_8c60(a0,a1); // tail-call, return-dupe
    }
    v2 = 1;
  }
  v1 = v2 - (unsigned int)((*(unsigned int *)((long)a0 + 0xac) & 0xf000) == 0x4000);
  if (v1)
    return (unsigned long)v1;
  return sub_8c60(a0,a1);
}

// Function: sub_bc10 @ 0xbc10
unsigned long sub_bc10(struct_23 *a0,struct_23 *a1)
{
  char *v1;
  char *v2;
  unsigned int v3;
  int v4;
  
  v4 = a0->field_0xa8;
  if (((a1->field_0xa8 != 3) && (a1->field_0xa8 != 9)) && ((a1->field_0xac & 0xf000) != 0x4000)) {
    if (v4 == 9 || v4 == 3)
      return 0xffffffff;
    v4 = 0;
label_bc71:
    v3 = v4 - (unsigned int)((a0->field_0xac & 0xf000) == 0x4000);
    if (v3) {
      return (unsigned long)v3;
    }
  }
  else if (v4 != 9 && v4 != 3) {
    v4 = 1;
    goto label_bc71;
  }
  v3 = ((unsigned int)(a0->field_0x88 < a1->field_0x88) - (unsigned int)(a1->field_0x88 < a0->field_0x88)) + ((unsigned int)(a0->field_0x80 < a1->field_0x80) - (unsigned int)(a1->field_0x80 < a0->field_0x80)) * 2;
  if (!v3) {
    v1 = a1->field_0x0;
    v2 = a0->field_0x0;
    return strcmp(v2,v1); // tail-call
  }
  return (unsigned long)v3;
}

// Function: sub_bcf0 @ 0xbcf0
long sub_bcf0(unsigned char *a0,unsigned int *a1,unsigned int a2,struct_1 *a3,bool a4,obstack *a5,unsigned char *a6)
{
  unsigned char v1;
  unsigned char v10 [8200];
  unsigned char *v11; // stack - 0x2050
  unsigned char v12; // stack - 0x2051
  long v13;
  unsigned long v14;
  char v15;
  char v16;
  unsigned char *v2;
  unsigned char *v3;
  char *v4;
  unsigned long v5; // rax
  char *v6; // rax
  char *v7; // rax
  char *v8; // rcx
  long *v9;
  
  v2 = a6;
  v11 = v10;
  v5 = sub_7920(&v11,a0,a1,a2,NULL,&v12);
  if ((v12) && (a4)) {
    dat_261f8 += 1;
    v4 = *(char **)&stdout->field_0x28;
    if (*(char **)&stdout->field_0x30 <= v4) // branch-flip
      __overflow(stdout,0x20);
    else {
      *(char **)&stdout->field_0x28 = &v4[1];
      *v4 = 0x20;
    }
  }
  if (a3) {
    if (sub_6c80(4)) {
      sub_7880((struct_1 *)0x25060);
      sub_7880((struct_1 *)0x25070);
    }
    sub_7880((struct_1 *)0x25060);
    sub_7880(a3);
    sub_7880((struct_1 *)0x25070);
  }
  v16 = dat_263a9;
  v14 = v5;
  if (v2) { // branch-flip
    v15 = '\0';
    if ((dat_263a8) && (v15 = dat_263a9, dat_263a9)) {
      if (v12) { // branch-flip
        v13 = 0;
        v16 = 0;
      }
      else {
        v1 = *v11;
        v3 = *(unsigned char **)&stdout->field_0x28;
        v14 = v5 - 2;
        if (*(unsigned char **)&stdout->field_0x30 <= v3) { // branch-flip
          __overflow(stdout,(unsigned int)v1);
          v13 = 1;
        }
        else {
          v13 = 1;
          *(unsigned char **)&stdout->field_0x28 = &v3[1];
          *v3 = v1;
        }
      }
    }
    else {
      v13 = 0;
      v16 = v15;
    }
    v6 = (char *)sub_72a0(dat_26388,0);
    v7 = (char *)sub_72a0(v2,1);
    v8 = "";
    if (*v7 != '/')
      v8 = "/";
    __printf_chk(1,"\x1b]8;;file://%s%s%s\a",v6,v8,v7);
    free(v6);
    free(v7);
    v15 = v16;
  }
  else {
    v13 = 0;
    v16 = 0;
    v15 = 0;
  }
  if (a5) { // branch-flip
    if (dat_26318) {
      v9 = *(long **)&a5->field_0x18;
      if ((unsigned long)(*(long *)&a5->field_0x20 - (long)v9) <= 7) {
        _obstack_newchunk(a5,8);
        v9 = *(long **)&a5->field_0x18;
      }
      *v9 = dat_261f8;
      *(long *)&a5->field_0x18 = *(long *)&a5->field_0x18 + 8;
    }
    fwrite_unlocked(&v11[v13],1,v14,stdout);
    dat_261f8 += v5;
    v16 = v15;
    if (dat_26318) {
      v9 = *(long **)&a5->field_0x18;
      if ((unsigned long)(*(long *)&a5->field_0x20 - (long)v9) <= 7) {
        _obstack_newchunk(a5,8);
        v9 = *(long **)&a5->field_0x18;
      }
      *v9 = dat_261f8;
      *(long *)&a5->field_0x18 = *(long *)&a5->field_0x18 + 8;
    }
  }
  else {
    fwrite_unlocked(&v11[v13],1,v14,stdout);
    dat_261f8 += v5;
  }
  if ((v2) && (fputs_unlocked("\x1b]8;;\a",stdout), v16)) {
    v1 = v11[v5 - 1];
    v2 = *(unsigned char **)&stdout->field_0x28;
    if (*(unsigned char **)&stdout->field_0x30 <= v2) // branch-flip
      __overflow(stdout,(unsigned int)v1);
    else {
      *(unsigned char **)&stdout->field_0x28 = &v2[1];
      *v2 = v1;
    }
  }
  if ((v11 != a0) && (v11 != v10))
    free(v11);
  return v12 + v5;
}

// Function: sub_c0e0 @ 0xc0e0
long sub_c0e0(struct_26 *a0,unsigned char a1,obstack *a2,unsigned long a3)
{
  unsigned long v1;
  unsigned char *v10;
  char v11;
  unsigned long v2;
  unsigned long *v3;
  unsigned int v4; // eax
  long v5; // rax
  unsigned long v6;
  struct_1 *v7;
  unsigned int v8; // edx
  unsigned int v9;
  
  if (a1) { // branch-flip
    v10 = a0->field_0x8;
    if (!dat_26312) {
      v5 = sub_bcf0(v10,dat_262d0,a0->field_0xc4,NULL,a1 ^ 1,a2,a0->field_0x10); // return-dupe
      sub_84f0();
      return v5;
    }
    v9 = a0->field_0xac;
    if (!a0->field_0xb9) {
      if (!sub_6c80(0xc)) {
        v4 = 0xffffffff;
        goto label_c1b4;
      }
      v5 = 0xc0;
      goto label_c21e;
    }
    v11 = a0->field_0xb8;
    v4 = 0;
    if (v11) goto label_c303;
label_c1c5:
    v9 = *(unsigned int *)&"\r"[(unsigned long)a0->field_0xa8 * 4];
    v6 = (unsigned long)v9;
    v11 = v9 == 7;
    if (v9 != 5) { // branch-flip
label_c1e6:
      if ((v4) || (!v11))
        v5 = v6 << 4;
      else {
        v5 = 0xd0;
        if (!dat_26390)
          v5 = (-(unsigned long)(sub_6c80(0xd) == '\0') & 0xffffffffffffffa0) + 0xd0;
      }
    }
    else {
label_c3b0:
      v6 = strlen((char *)v10);
      for (v3 = dat_26308; v3; v3 = (unsigned long *)v3[4]) {
        v1 = *v3;
        if ((v1 <= v6) && (v2 = v3[1], !sub_eeb0(&v10[v6 - v1],v2))) {
          v7 = (struct_1 *)&v3[2];
          goto label_c228;
        }
      }
      v5 = 0x50;
    }
  }
  else {
    v10 = a0->field_0x0;
    if (!dat_26312) {
      v5 = sub_bcf0(v10,dat_262d0,a0->field_0xc4,NULL,a1 ^ 1,a2,a0->field_0x10);
      sub_84f0();
      return v5;
    }
    v4 = (unsigned int)(unsigned char)a0->field_0xb9;
    if ((dat_26390) && (a0->field_0xb9))
      v9 = a0->field_0xac;
    else {
      v9 = a0->field_0x30;
    }
label_c1b4:
    v11 = a0->field_0xb8;
    if (!v11) goto label_c1c5;
label_c303:
    v8 = v9 & 0xf000;
    if (v8 == 0x8000) {
      if ((v9 & 0x800) && (sub_6c80(0x10))) {
        v5 = 0x100;
        goto label_c21e;
      }
      if ((v9 & 0x400) && (sub_6c80(0x11))) {
        v5 = 0x110;
        goto label_c21e;
      }
      if ((sub_6c80(0x15)) && (a0->field_0xc0)) {
        v5 = 0x150;
        goto label_c21e;
      }
      if ((v9 & 0x49) && (sub_6c80(0xe))) {
        v5 = 0xe0;
        goto label_c21e;
      }
      if ((2 <= a0->field_0x28) && (sub_6c80(0x16))) {
        v5 = 0x160;
        goto label_c21e;
      }
      goto label_c3b0;
    }
    if (v8 != 0x4000) {
      if (v8 != 0xa000) {
        v5 = 0x80;
        if ((((v8 != 0x1000) && (v5 = 0x90, v8 != 0xc000)) && (v5 = 0xa0, v8 != 0x6000)) && (v5 = 0xb0, v8 != 0x2000))
          v5 = 0xd0;
        goto label_c21e;
      }
      v6 = 7;
      goto label_c1e6;
    }
    if ((v9 & 0x202) == 0x202) {
      v5 = 0x140;
      if (sub_6c80(0x14)) goto label_c21e;
    }
    if (v9 & 2) {
      v5 = 0x130;
      if (sub_6c80(0x13)) goto label_c21e;
    }
    v5 = 0x60;
    if (v9 & 0x200)
      v5 = (-(unsigned long)(sub_6c80(0x12) == '\0') & 0xffffffffffffff40) + 0x120;
  }
label_c21e:
  v7 = (struct_1 *)(v5 + 0x25060);
label_c228:
  if (!v7->field_0x8) {
    if (!sub_6c80(4)) {
      v5 = sub_bcf0(v10,dat_262d0,a0->field_0xc4,NULL,a1 ^ 1,a2,a0->field_0x10);
      sub_84f0();
      return v5;
    }
    v7 = NULL;
  }
  v5 = sub_bcf0(v10,dat_262d0,a0->field_0xc4,v7,a1 ^ 1,a2,a0->field_0x10);
  sub_84f0();
  if (dat_25088) // branch-flip
    sub_7880((struct_1 *)0x25080);
  else {
    sub_7880((struct_1 *)0x25060);
    sub_7880((struct_1 *)0x25090);
    sub_7880((struct_1 *)0x25070);
  }
  if (!dat_262b0)
    return v5;
  v6 = v5 + -1 + a3;
  if (a3 / dat_262b0 == v6 / dat_262b0)
    return v5;
  sub_7880((struct_1 *)0x251d0,a3 / dat_262b0,v6 % dat_262b0);
  return v5;
}

// Function: sub_c590 @ 0xc590
void sub_c590(void *a0,unsigned long a1)
{
  unsigned char *v1;
  unsigned char v2; // al
  char *v3; // rax
  int v4;
  char v5 [664];
  
  if ((dat_26312) && (sub_6c80(4))) {
    sub_7880((struct_1 *)0x25060);
    sub_7880((struct_1 *)0x250a0);
    sub_7880((struct_1 *)0x25070);
  }
  if (dat_262fc) {
    v3 = "?";
    if ((*(char *)((long)a0 + 0xb8)) && (*(unsigned long *)((long)a0 + 0x20)))
      v3 = (char *)sub_12720(*(unsigned long *)((long)a0 + 0x20),v5);
    v4 = 0;
    if (dat_26338 != 4)
      v4 = dat_26360;
    __printf_chk(1,"%*s ",v4,v3);
  }
  if (dat_2632c) {
    v3 = "?";
    if (*(char *)((long)a0 + 0xb8))
      v3 = (char *)sub_11550(*(unsigned long *)((long)a0 + 0x58),v5,dat_26328,0x200,dat_26320);
    v4 = 0;
    if (dat_26338 != 4)
      v4 = dat_2635c;
    __printf_chk(1,"%*s ",v4,v3);
  }
  if (dat_26365) {
    v4 = 0;
    if (dat_26338 != 4)
      v4 = dat_26354;
    __printf_chk(1,"%*s ",v4,*(char **)((long)a0 + 0xb0));
  }
  sub_c0e0(a0,0,NULL,a1);
  if ((dat_26314) && (v2 = sub_75b0((unsigned char)*(char *)((long)a0 + 0xb8),*(unsigned int *)((long)a0 + 0x30),*(unsigned int *)((long)a0 + 0xa8)), v2)) {
    dat_261f8 += 1;
    v1 = *(unsigned char **)&stdout->field_0x28;
    if (*(unsigned char **)&stdout->field_0x30 <= v1) // branch-flip
      __overflow(stdout,(unsigned int)v2);
    else {
      *(unsigned char **)&stdout->field_0x28 = &v1[1];
      *v1 = v2;
    }
  }
}

// Function: sub_c7c0 @ 0xc7c0
void sub_c7c0(char a0)
{
  unsigned long v1;
  struct_16 *v2;
  char *v3;
  unsigned char *v4;
  unsigned long v5;
  unsigned int v6; // ecx
  unsigned char v7;
  unsigned long v8; // rbx
  unsigned long v9;
  
  if (dat_263b0) {
    v8 = 0;
    v9 = 0;
    do {
      v2 = *(struct_16 **)(dat_263a0 + v8 * 8);
      if (dat_262b0) { // branch-flip
        v5 = sub_7e10(v2);
        if (v8) { // branch-flip
          v1 = v5 + v9 + 2;
          if ((!dat_262b0) || ((v1 < dat_262b0 && (v9 <= -v5 - 3)))) {
            v5 = v1;
            goto label_c870;
          }
          v6 = (unsigned int)(char)dat_25019;
          v9 = 0;
          v7 = dat_25019;
label_c87a:
          v3 = *(char **)&stdout->field_0x28;
          if (*(char **)&stdout->field_0x30 <= v3) // branch-flip
            __overflow(stdout,(int)a0);
          else {
            *(char **)&stdout->field_0x28 = &v3[1];
            *v3 = a0;
          }
          v4 = *(unsigned char **)&stdout->field_0x28;
          if (*(unsigned char **)&stdout->field_0x30 <= v4) // branch-flip
            __overflow(stdout,v6 & 0xff);
          else {
            *(unsigned char **)&stdout->field_0x28 = &v4[1];
            *v4 = v7;
          }
        }
        else {
          v5 = v9 + v5;
        }
      }
      else {
        v5 = v9;
        if (v8) {
          v5 = v9 + 2;
label_c870:
          v9 += 2;
          v6 = 0x20;
          v7 = 0x20;
          goto label_c87a;
        }
      }
      v8 += 1;
      sub_c590(v2,v9);
      v9 = v5;
    } while (v8 < dat_263b0);
  }
  v4 = *(unsigned char **)&stdout->field_0x28;
  if (v4 < *(unsigned char **)&stdout->field_0x30) {
    *(unsigned char **)&stdout->field_0x28 = &v4[1];
    *v4 = dat_25019;
    return;
  }
  __overflow(stdout,(unsigned int)dat_25019); // tail-call
}

// Function: sub_c980 @ 0xc980
void sub_c980(void *a0) // return-dupe x5
{
  unsigned int v1;
  char *v10;
  char *v11;
  long v12; // rbx
  char v13; // stack - 0x12a4
  char v14 [3656];
  char v15 [1008];
  char v16 [16];
  unsigned long v17; // stack - 0x1338
  char v18 [32];
  char v19 [16];
  unsigned int v2;
  unsigned long v20; // stack - 0x1340
  int v21; // ebp
  int v22;
  char *v23; // rax
  bool v24;
  unsigned long v25; // stack - 0x1330
  int v26; // stack - 0x1318
  int v27; // stack - 0x12d8
  unsigned long v28; // stack - 0x12a3
  unsigned short v29; // stack - 0x129b
  char *v3;
  char v30; // stack - 0x1299
  char v4;
  int v5;
  unsigned int v6;
  char *v7;
  long v8;
  char *v9; // rbx
  
  if (*(char *)((long)a0 + 0xb8)) // branch-flip
    sub_fba0((struct_28 *)((long)a0 + 0x18),&v13);
  else {
    v30 = 0;
    v29 = 0x3f3f;
    v13 = "?pcdb-lswd"[*(unsigned int *)((long)a0 + 0xa8)];
    v28 = 0x3f3f3f3f3f3f3f3f;
  }
  if (dat_26364) { // branch-flip
    if (*(int *)((long)a0 + 0xbc) == 1) {
      v29 = CONCAT11(0x2e,(char)v29);
      goto label_c9e6;
    }
    if (*(int *)((long)a0 + 0xbc) != 2) goto label_c9e6;
    v29 = CONCAT11(0x2b,(char)v29);
    if (dat_26334 != 2) goto label_c9f5;
label_ccf8:
    v25 = *(unsigned long *)((long)a0 + 0x68);
    v17 = *(unsigned long *)((long)a0 + 0x60);
    v24 = 1;
  }
  else {
    v29 &= 0xff;
label_c9e6:
    if (dat_26334 == 2) goto label_ccf8;
label_c9f5:
    if (3 <= dat_26334) { // branch-flip
      if (dat_26334 != 3)
        abort(); // no-return
      v17 = *(unsigned long *)((long)a0 + 0x70);
      v25 = *(unsigned long *)((long)a0 + 0x78);
      v24 = (v17 & v25) != 0xffffffffffffffff;
    }
    else if (dat_26334) { // branch-flip
      v25 = *(unsigned long *)((long)a0 + 0x88);
      v17 = *(unsigned long *)((long)a0 + 0x80);
      v24 = 1;
    }
    else {
      v25 = *(unsigned long *)((long)a0 + 0x78);
      v17 = *(unsigned long *)((long)a0 + 0x70);
      v24 = 1;
    }
  }
  v10 = v14;
  if (dat_262fc) {
    v10 = "?";
    if ((*(char *)((long)a0 + 0xb8)) && (*(unsigned long *)((long)a0 + 0x20)))
      v10 = (char *)sub_12720(*(unsigned long *)((long)a0 + 0x20),v15);
    v10 = &v14[__sprintf_chk(v14,1,0xe3b,"%*s ",dat_26360,v10)];
  }
  if (dat_2632c) {
    v23 = "?";
    if (*(char *)((long)a0 + 0xb8))
      v23 = (char *)sub_11550(*(unsigned long *)((long)a0 + 0x58),v15,dat_26328,0x200,dat_26320);
    v5 = dat_2635c;
    v5 -= sub_12ec0(v23,0);
    if (v5 <= 0) goto label_cfd0;
    v7 = &v10[v5];
    do {
      v9 = &v10[1];
      *v10 = ' ';
      v10 = v9;
      v11 = v23;
    } while (v9 != v7);
    while( true ) {
      v23 = &v11[1];
      v10 = &v7[1];
      v4 = *v11;
      *v7 = v4;
      if (!v4) break;
label_cfd0:
      v7 = v10;
      v11 = v23;
    }
    *v7 = ' ';
  }
  v23 = "?";
  if (*(char *)((long)a0 + 0xb8))
    v23 = (char *)sub_12720(*(unsigned long *)((long)a0 + 0x28),v15);
  v10 = &v10[__sprintf_chk(v10,1,0xffffffffffffffff,"%s %*s ",&v13,dat_26358,v23)];
  if (dat_26318)
    sub_7430("  ",strlen("  "));
  if ((((dat_25029) || (dat_25028)) || (dat_2632e)) || (dat_26365)) {
    sub_7430(v14,(long)v10 - (long)v14);
    v1 = dat_26350;
    if (dat_25029) {
      v10 = "?";
      v2 = *(unsigned int *)((long)a0 + 0x34);
      if ((*(char *)((long)a0 + 0xb8)) && (v10 = NULL, !dat_2632d))
        v10 = (char *)sub_12340(v2);
      sub_92c0(v10,v2,v1);
    }
    v2 = dat_2634c;
    v1 = dat_26348;
    if (dat_25028) {
      v10 = "?";
      v1 = *(unsigned int *)((long)a0 + 0x38);
      if ((*(char *)((long)a0 + 0xb8)) && (v10 = NULL, !dat_2632d))
        v10 = (char *)sub_124e0(v1);
      sub_92c0(v10,v1,v2);
      v1 = dat_26348;
    }
    dat_26348 = v1;
    if (dat_2632e) {
      v10 = "?";
      v2 = *(unsigned int *)((long)a0 + 0x34);
      if ((*(char *)((long)a0 + 0xb8)) && (v10 = NULL, !dat_2632d))
        v10 = (char *)sub_12340(v2);
      sub_92c0(v10,v2,v1);
    }
    v10 = v14;
    if (dat_26365)
      sub_92c0(*(char **)((long)a0 + 0xb0),0,dat_26354);
  }
  if (*(char *)((long)a0 + 0xb8)) { // branch-flip
    if ((*(unsigned int *)((long)a0 + 0x30) & 0xb000) != 0x2000) {
      v23 = (char *)sub_11550(*(unsigned long *)((long)a0 + 0x48),v15,dat_2631c,1,dat_25020);
      goto label_cb76;
    }
    v21 = dat_2633c - (dat_26344 + 2 + dat_26340);
    v23 = (char *)sub_12720((unsigned int)((*(unsigned long *)((long)a0 + 0x40) >> 0x14) << 8) | (unsigned int)*(unsigned long *)((long)a0 + 0x40) & 0xff,v15);
    v5 = dat_26340;
    v7 = (char *)sub_12720((unsigned int)((unsigned long)*(unsigned long *)((long)a0 + 0x40) >> 0x20) & 0xfffff000 | (unsigned int)((unsigned long)*(unsigned long *)((long)a0 + 0x40) >> 8) & 0xfff,v18);
    v22 = 0;
    if (0 <= v21)
      v22 = v21;
    v22 += dat_26344;
    v7 = &v10[__sprintf_chk(v10,1,0xffffffffffffffff,"%*s, %*s ",v22,v7,v5,v23)];
  }
  else {
    v23 = "?";
label_cb76:
    v5 = dat_2633c;
    v5 -= sub_12ec0(v23,0);
    if (1 <= v5) { // branch-flip
      v11 = v10;
      do {
        v9 = &v11[1];
        *v11 = ' ';
        v11 = v9;
        v7 = &v10[v5];
      } while (v9 != &v10[v5]);
    }
    else {
      v7 = v10;
    }
    do {
      v10 = v7;
      v4 = *v23;
      v23 = &v23[1];
      v7 = &v10[1];
      *v10 = v4;
    } while (v4);
    *v10 = ' ';
  }
  *v7 = '\x01';
  if ((*(char *)((long)a0 + 0xb8)) && (v24)) {
    if (sub_17c60(dat_262a8,&v17,v16)) { // branch-flip
      v24 = (long)v25 < dat_26378;
      v6 = (unsigned int)(dat_26378 < (long)v25);
      v5 = (unsigned int)v24 - (unsigned int)(dat_26378 < (long)v25);
      if ((int)(v5 + ((unsigned int)((long)v17 < dat_26370) - (unsigned int)(dat_26370 < (long)v17)) * 2) < 0) {
        sub_10200((timespec *)0x26370);
        v24 = (long)v25 < dat_26378;
        v6 = (unsigned int)(dat_26378 < (long)v25);
        v5 = v24 - v6;
      }
      v6 = (v6 - v24) + ((unsigned int)(dat_26370 < (long)v17) - (unsigned int)((long)v17 < dat_26370)) * 2 & v5 + ((unsigned int)((long)v17 < dat_26370 + -0xf0c2ac) - (unsigned int)(dat_26370 + -0xf0c2ac < (long)v17)) * 2;
      if (dat_253c8) // branch-flip
        v8 = ((unsigned long)((int)v6 >> 0x1f & 0xc) + (long)v26) * 0x80 + 0x253e0;
      else {
        v8 = *(long *)((unsigned long)(v6 >> 0x1f) * 8 + 0x25040);
      }
      v8 = sub_14d80(v7,0x3e9,v8,v16,dat_262a8,v25);
      if (!v8) goto label_d1c8;
      v7 = &v7[v8];
    }
    else {
label_d1c8:
      if (*v7) {
        if (!*(char *)((long)a0 + 0xb8)) goto label_cbdc;
        v10 = (char *)sub_12680(v17,v18);
        goto label_cbe3;
      }
    }
    *v7 = ' ';
    v7 = &v7[1];
    goto label_cc19;
  }
label_cbdc:
  v10 = "?";
label_cbe3:
  v5 = dat_25014;
  if (dat_25014 < 0) {
    v20 = 0;
    if (sub_17c60(dat_262a8,&v20,v19)) { // branch-flip
      v8 = dat_25040;
      if (dat_253c8)
        v8 = (long)v27 * 0x80 + 0x253e0;
      v8 = sub_14d80(v15,0x3e9,v8,v19,dat_262a8,0);
      if (!v8) goto label_d364;
      v5 = sub_12ca0(v15,v8,0);
      dat_25014 = v5;
    }
    else {
label_d364:
      v5 = dat_25014;
    }
    if (v5 <= -1) {
      dat_25014 = 0;
      v5 = 0;
    }
  }
  v7 = &v7[__sprintf_chk(v7,1,0xffffffffffffffff,"%*s ",v5,v10)];
label_cc19:
  v12 = (long)v7 - (long)v14;
  sub_7430(v14,v12);
  v8 = sub_c0e0(a0,0,(obstack *)0x261a0,v12);
  if (*(int *)((long)a0 + 0xa8) != 6) { // branch-flip
    if (!dat_26314)
      return;
    v6 = sub_75b0((unsigned char)*(char *)((long)a0 + 0xb8),*(unsigned int *)((long)a0 + 0x30),*(int *)((long)a0 + 0xa8));
    v4 = (char)v6;
  }
  else {
    if (!*(long *)((long)a0 + 8))
      return;
    sub_7430(" -> ",strlen(" -> "));
    sub_c0e0(a0,1,NULL,v12 + 4 + v8);
    if (!dat_26314)
      return;
    v6 = sub_75b0(1,*(unsigned int *)((long)a0 + 0xac),0);
    v4 = (char)v6;
  }
  if (!v4)
    return;
  dat_261f8 += 1;
  v3 = *(char **)&stdout->field_0x28;
  if (v3 < *(char **)&stdout->field_0x30) {
    *(char **)&stdout->field_0x28 = &v3[1];
    *v3 = (char)v6;
    return;
  }
  __overflow(stdout,v6 & 0xff);
}

// Function: sub_d420 @ 0xd420
void sub_d420(void) // return-dupe x6
{
  long v1;
  long v10; // rax
  unsigned long v11; // stack - 0x40
  unsigned char *v2;
  unsigned long v3; // rax
  long v4; // rax
  unsigned long v5; // rdx
  unsigned long v6;
  long v7;
  unsigned long v8;
  long v9;
  
  switch(dat_26338) {
    case 0:
      v3 = 0;
      if (!dat_263b0)
        return;
      do {
        if ((dat_26312) && (sub_6c80(4))) {
          sub_7880((struct_1 *)0x25060);
          sub_7880((struct_1 *)0x250a0);
          sub_7880((struct_1 *)0x25070);
        }
        sub_c980(dat_263a0[v3]);
        dat_261f8 += 1;
        v2 = *(unsigned char **)&stdout->field_0x28;
        if (*(unsigned char **)&stdout->field_0x30 <= v2) // branch-flip
          __overflow(stdout,(unsigned int)dat_25019);
        else {
          *(unsigned char **)&stdout->field_0x28 = &v2[1];
          *v2 = dat_25019;
        }
        v3 += 1;
      } while (v3 < dat_263b0);
      return;
    case 1:
      v3 = 0;
      if (!dat_263b0)
        return;
      do {
        sub_c590(dat_263a0[v3],0);
        v2 = *(unsigned char **)&stdout->field_0x28;
        if (*(unsigned char **)&stdout->field_0x30 <= v2) // branch-flip
          __overflow(stdout,(unsigned int)dat_25019);
        else {
          *(unsigned char **)&stdout->field_0x28 = &v2[1];
          *v2 = dat_25019;
        }
        v3 += 1;
      } while (v3 < dat_263b0);
      return;
    case 2:
      if (dat_262b0) {
        v3 = sub_7ff0(1);
        v11 = 0;
        v1 = dat_26208 + v3 * 0x18;
        v3 = (dat_263b0 / v3 + 1) - (unsigned long)(dat_263b0 % v3 == 0);
        if (!v3)
          return;
        do {
          v4 = 0;
          v6 = v11;
          v9 = 0;
          while( true ) {
            v8 = dat_263a0[v6];
            v10 = sub_7e10(v8);
            v7 = *(long *)(*(long *)(v1 + -8) + v4);
            v4 += 8;
            sub_c590(v8,v9);
            v6 += v3;
            if (dat_263b0 <= v6) break;
            v7 += v9;
            sub_7360(v10 + v9,v7);
            v9 = v7;
          }
          v2 = *(unsigned char **)&stdout->field_0x28;
          if (*(unsigned char **)&stdout->field_0x30 <= v2) // branch-flip
            __overflow(stdout,(unsigned int)dat_25019);
          else {
            *(unsigned char **)&stdout->field_0x28 = &v2[1];
            *v2 = dat_25019;
          }
          v11 += 1;
        } while (v3 != v11);
        return;
      }
      break;
    case 3:
      if (dat_262b0) {
        v3 = sub_7ff0(0);
        v1 = dat_26208 + -0x18 + v3 * 0x18;
        v8 = *dat_263a0;
        v4 = sub_7e10(v8);
        v9 = **(long **)(v1 + 0x10);
        sub_c590(v8,0);
        if (2 <= dat_263b0) {
          v6 = 1;
          v7 = 0;
          do {
            v5 = v6 % v3;
            if (v5) { // branch-flip
              v10 = v9 + v7;
              sub_7360(v7 + v4,v10);
            }
            else {
              v2 = *(unsigned char **)&stdout->field_0x28;
              if (*(unsigned char **)&stdout->field_0x30 <= v2) { // branch-flip
                v10 = 0;
                __overflow(stdout,(unsigned int)dat_25019);
              }
              else {
                v10 = 0;
                *(unsigned char **)&stdout->field_0x28 = &v2[1];
                *v2 = dat_25019;
              }
            }
            v8 = dat_263a0[v6];
            v6 += 1;
            sub_c590(v8,v10);
            v4 = sub_7e10(v8);
            v9 = *(long *)(*(long *)(v1 + 0x10) + v5 * 8);
            v7 = v10;
          } while (v6 < dat_263b0);
        }
        v2 = *(unsigned char **)&stdout->field_0x28;
        if (v2 < *(unsigned char **)&stdout->field_0x30) {
          *(unsigned char **)&stdout->field_0x28 = &v2[1];
          *v2 = dat_25019;
          return;
        }
        __overflow(stdout,(unsigned int)dat_25019); // tail-call
        return;
      }
      break;
    case 4:
      v8 = 0x2c;
      sub_c7c0(v8); // return-dupe, tail-call
      return;
    default:
      return;
    
  }
  v8 = 0x20;
  sub_c7c0(v8);
  return;
}

// Function: sub_d820 @ 0xd820
void sub_d820(unsigned char *a0,unsigned char *a1,char a2)
{
  unsigned long *v1;
  struct_9 *v10; // rax
  struct_9 *v11; // rax
  char *v12;
  dirent *v13; // rax
  unsigned char *v14;
  unsigned long v15; // rax
  char *v16; // rax
  unsigned long v17; // stack - 0x368
  char v18 [663];
  unsigned int v19;
  char *v2;
  unsigned long v20; // stack - 0x378
  unsigned long v21; // stack - 0x360
  unsigned long v3;
  unsigned long v4;
  unsigned long *v5;
  unsigned char v6; // al
  int v7; // eax
  int *v8; // rax
  DIR *v9; // rax
  
  v8 = __errno_location();
  *v8 = 0;
  v9 = opendir((char *)a0);
  if (!v9) {
    sub_7540((unsigned char)a2,dcgettext(NULL,"cannot open directory %s",5),a0); // tail-call
    return;
  }
  if (dat_263c8) {
    v7 = dirfd(v9);
    if (0 <= v7) { // branch-flip
      v7 = sub_7050(v7,"",&v17,0x1000,0x100);
      v3 = v17;
      v4 = v21;
    }
    else {
      v7 = sub_7050(0xffffff9c,a0,&v17,0,0x100);
      v3 = v17;
      v4 = v21;
    }
    if (v7 <= -1) {
      sub_7540((unsigned char)a2,dcgettext(NULL,"cannot determine device and inode of %s",5),a0);
      closedir(v9); // tail-call
      return;
    }
    v10 = (struct_9 *)sub_18590(0x10);
    v10->field_0x8 = v3;
    v10->field_0x0 = v4;
    v11 = (struct_9 *)sub_112c0(dat_263c8,v10);
    if (!v11)
      sub_18a90(); // no-return
    if (v10 != v11) {
      free(v10);
      v12 = (char *)sub_17150(0,3,a0);
      error(0,0,dcgettext(NULL,"%s: not listing already-listed directory",5),v12);
      closedir(v9);
      dat_26210 = 2;
      return;
    }
    if ((unsigned long)(dat_26100 - (long)dat_260f8) <= 0xf)
      _obstack_newchunk((obstack *)0x260e0,0x10);
    v1 = &dat_260f8[2];
    dat_260f8[1] = v3;
    *dat_260f8 = v4;
    dat_260f8 = v1;
  }
  sub_8730();
  if (dat_262f6) { // branch-flip
    if (dat_25010) goto label_d9cf;
label_db04:
    dat_261f8 += 1;
    v2 = *(char **)&stdout->field_0x28;
    if (*(char **)&stdout->field_0x30 <= v2) {
      __overflow(stdout,10);
      goto label_d9cf;
    }
    *(char **)&stdout->field_0x28 = &v2[1];
    *v2 = 10;
    if (dat_26318) goto label_db40;
label_d9e3:
    dat_25010 = '\0';
  }
  else {
    if (!dat_262b8) goto label_da47;
    if (!dat_25010) goto label_db04;
label_d9cf:
    if (!dat_26318) goto label_d9e3;
label_db40:
    dat_25010 = '\0';
    sub_7430("  ",strlen("  "));
  }
  v14 = NULL;
  if ((dat_26311) && (v14 = (unsigned char *)sub_ef30(a0,2), !v14))
    sub_7540((unsigned char)a2,dcgettext(NULL,"error canonicalizing %s",5),a0);
  if (!a1)
    a1 = a0;
  sub_bcf0(a1,dat_262c8,0xffffffff,NULL,1,(obstack *)0x26140,v14);
  free(v14);
  sub_7430(":\n",strlen(":\n"));
label_da47:
  v20 = 0;
  do {
    *v8 = 0;
    v13 = readdir(v9);
    if (v13) { // branch-flip
      v2 = &v13->field_0x13;
      v1 = dat_262e8;
      if (dat_262f0 != 2) {
        if (v13->field_0x13 != '.') { // branch-flip
          v5 = dat_262e0;
          if (!dat_262f0) {
            for (; v1 = dat_262e8, v5; v5 = (unsigned long *)v5[1]) {
              v12 = (char *)*v5;
              if (!fnmatch(v12,v2,4)) goto label_dae0;
            }
          }
        }
        else if ((!dat_262f0) || (!(&v13->field_0x14)[v13->field_0x14 == '.'])) goto label_dae0;
      }
      for (; v1; v1 = (unsigned long *)v1[1]) {
        v12 = (char *)*v1;
        if (!fnmatch(v12,v2,4)) goto label_dae0;
      }
      v19 = 0;
      v6 = v13->field_0x12 - 1;
      if (v6 <= 0xd)
        v19 = *(unsigned int *)((unsigned long)v6 * 4 + 0x1a600);
      v20 += sub_96f0(v2,v19,0,a0);
      if ((((dat_26338 == 1) && (dat_26330 == 6)) && (!dat_2632c)) && (!dat_262f6)) {
        sub_8e30();
        sub_d420();
        sub_8730();
      }
    }
    else {
      if (!*v8) {
label_dc53:
        if (closedir(v9))
          sub_7540((unsigned char)a2,dcgettext(NULL,"closing directory %s",5),a0);
        sub_8e30();
        if (dat_262f6)
          sub_9510(a0,0);
        if ((!dat_26338) || (dat_2632c)) {
          v12 = (char *)sub_11550(v20,v18,dat_26328,0x200,dat_26320);
          v15 = strlen(v12);
          v12[-1] = ' ';
          v12[v15] = dat_25019;
          if (dat_26318)
            sub_7430("  ",strlen("  "));
          v16 = dcgettext(NULL,"total",5);
          sub_7430(v16,strlen(v16));
          sub_7430(&v12[-1],&(&v12[v15])[1 - (long)&v12[-1]]);
        }
        if (!dat_263b0)
          return;
        sub_d420(); // tail-call
        return;
      }
      sub_7540((unsigned char)a2,dcgettext(NULL,"reading directory %s",5),a0);
      if (*v8 != 0x4b) goto label_dc53;
    }
label_dae0:
    sub_84f0();
  } while( true );
}

// Function: sub_df00 @ 0xdf00
void sub_df00(int a0)
{
  FILE *v1;
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
  int v2; // eax
  char *v20 [4]; // stack - 0xa8
  char *v3;
  char **v4; // rbx
  char *v5; // stack - 0xb8
  char *v6;
  char *v7;
  char *v8; // r13
  long v9; // fs_offset
  
  v6 = dat_26400;
  v4 = &v5;
  v19 = *(unsigned long *)(v9 + 0x28);
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v6);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n",5),v6);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"List information about the FILEs (the current directory by default).\nSort entries alphabetically if none of -cftuvSUX nor --sort is specified.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v1);
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
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nThe SIZE argument is an integer and optional unit (example: 10K is 10*1024).\nUnits are K,M,G,T,P,E,Z,Y (powers of 1024) or KB,MB,... (powers of 1000).\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nThe TIME_STYLE argument can be full-iso, long-iso, iso, locale, or +FORMAT.\nFORMAT is interpreted like in date(1).  If FORMAT is FORMAT1<newline>FORMAT2,\nthen FORMAT1 applies to non-recent files and FORMAT2 to recent files.\nTIME_STYLE prefixed with \'posix-\' takes effect only outside the POSIX locale.\nAlso the TIME_STYLE environment variable sets the default style to use.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nThe WHEN argument defaults to \'always\' and can also be \'auto\' or \'never\'.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nUsing color to distinguish file types is disabled both by default and\nwith --color=never.  With --color=auto, ls emits color codes only when\nstandard output is connected to a terminal.  The LS_COLORS environment\nvariable can change the settings.  Use the dircolors(1) command to set it.\n",5),v1);
    v1 = stdout;
    v6 = "ls";
    fputs_unlocked(dcgettext(NULL,"\nExit status:\n 0  if OK,\n 1  if minor problems (e.g., cannot access subdirectory),\n 2  if serious trouble (e.g., cannot access command-line argument).\n",5),v1);
    if ((dat_251e0 != 1) && (v6 = "dir", dat_251e0 != 2))
      v6 = "vdir";
    v17 = 0;
    v10 = "test invocation";
    v20[0] = "coreutils";
    v7 = "[";
    v20[1] = "Multi-call invocation";
    v11 = "sha256sum";
    v20[2] = "sha224sum";
    v13 = "sha384sum";
    v5 = "[";
    v20[3] = "sha2 utilities";
    v12 = "sha2 utilities";
    v14 = "sha2 utilities";
    v15 = "sha512sum";
    v16 = "sha2 utilities";
    v18 = 0;
    do {
      if (!strcmp(v6,v7)) break;
      v7 = *(char **)((long)v4 + 0x10);
      v4 = (char *)((long)v4 + 0x10);
    } while (v7);
    v7 = *(char **)((long)v4 + 8);
    if (!*(char **)((long)v4 + 8))
      v7 = v6;
    __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v3 = setlocale(5,NULL);
    if (v3) {
      v2 = strncmp(v3,"en_",3);
      v1 = stdout;
      if (v2)
        fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
    }
    v3 = "test";
    if (strcmp(v6,"["))
      v3 = v6;
    v8 = " invocation";
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v3);
    if (v6 != v7)
      v8 = "";
    __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v7,v8);
  }
  exit(a0); // no-return
}

// Function: sub_e820 @ 0xe820
char * sub_e820(char *a0,unsigned long a1)
{
  unsigned long v1;
  char *v2;
  char *v3;
  unsigned long v4;
  char v5 [136];
  
  v4 = 0x80;
  if ((a1) && (v4 = a1 + 1, 0x401 <= a1))
    v4 = 0x401;
label_e888:
  if (v4 != 0x80) goto label_e8d3;
label_e891:
  v2 = v5;
  v3 = NULL;
  if (a1) goto label_e8d3;
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
label_e91b:
        *__errno_location() = 0xc;
        return NULL;
      }
      v4 = 0x7fffffffffffffff;
      goto label_e888;
    }
    v4 *= 2;
    if (v4 == 0x80) goto label_e891;
label_e8d3:
    v2 = malloc(v4);
    if (!v2) goto label_e91b;
    v3 = v2;
  } while( true );
}

// Function: sub_e9b0 @ 0xe9b0
void sub_e9b0(void)
{
  sub_df00(1); // tail-call, no-return
}

// Function: sub_e9c0 @ 0xe9c0
unsigned long sub_e9c0(char *a0,long *a1,void *a2,unsigned long a3)
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

// Function: sub_ead0 @ 0xead0
long sub_ead0(char *a0,long *a1) // return-dupe
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

// Function: sub_eb30 @ 0xeb30
void sub_eb30(char *a0,char *a1,long a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (a2 != -1) // branch-flip
    v1 = dcgettext(NULL,"ambiguous argument %s for %s",5);
  else {
    v1 = dcgettext(NULL,"invalid argument %s for %s",5);
  }
  v2 = sub_174a0(1,a0);
  error(0,0,v1,sub_16ca0(0,8,a1),v2); // tail-call
}

// Function: sub_ebc0 @ 0xebc0
void sub_ebc0(unsigned long *a0,void *a1,unsigned long a2)
{
  char *v1;
  char *v2;
  FILE *v3;
  long v4;
  void *v5;
  
  v3 = stderr;
  v5 = NULL;
  fputs_unlocked(dcgettext(NULL,"Valid arguments are:",5),v3);
  v4 = 0;
  v1 = (char *)*a0;
  while (v1) {
    if ((v4) && (!memcmp(v5,a1,a2)))
      __fprintf_chk(stderr,1,", %s",(char *)sub_174c0(v1));
    else {
      __fprintf_chk(stderr,1,"\n  - %s",sub_174c0(v1));
      v5 = a1;
    }
    v4 += 1;
    a1 = (void *)((long)a1 + a2);
    v1 = (char *)a0[v4];
  }
  v2 = *(char **)&stderr->field_0x28;
  if (v2 < *(char **)&stderr->field_0x30) {
    *(char **)&stderr->field_0x28 = &v2[1];
    *v2 = 10;
    return;
  }
  __overflow(stderr,10); // tail-call
}

// Function: sub_ecf0 @ 0xecf0
long sub_ecf0(char *a0,char *a1,long *a2,void *a3,unsigned long a4,void *a5,char a6) // early-return
{
  char *v1;
  long v2;
  
  if (a6) { // branch-flip
    v2 = sub_e9c0(a1,a2,a3,a4);
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
  sub_eb30(a0,a1,v2);
  sub_ebc0(a2,a3,a4);
  (*a5)();
  return -1;
}

// Function: sub_edc0 @ 0xedc0
long sub_edc0(void *a0,long *a1,void *a2,unsigned long a3) // return-dupe
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

// Function: sub_ee20 @ 0xee20
char * sub_ee20(char *a0)
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

// Function: sub_ee80 @ 0xee80
void sub_ee80(char *a0)
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

// Function: sub_eeb0 @ 0xeeb0
int sub_eeb0(long a0,long a1,long a2)
{
  unsigned int v1; // eax
  long v2; // rcx
  unsigned char v3;
  unsigned char v4;
  unsigned int v5; // r10d
  
  if ((a0 != a1) && (a2)) {
    v2 = 0;
    do {
      v4 = *(unsigned char *)(a0 + v2);
      v1 = (unsigned int)v4;
      if ((unsigned int)v4 - 0x41 <= 0x19) {
        v1 += 0x20;
        v4 += 0x20;
      }
      v3 = *(unsigned char *)(a1 + v2);
      v5 = (unsigned int)v3;
      if ((unsigned int)v3 - 0x41 <= 0x19) {
        v5 = v3 + 0x20;
        v3 += 0x20;
      }
    } while (((v2 != a2 + -1) && (v1)) && (v2 = v2 + 1, v4 == v3));
    return v1 - v5;
  }
  return 0;
}

// Function: sub_ef30 @ 0xef30
long sub_ef30(char *a0,unsigned int a1) // early-return x2
{
  char v1;
  char v10 [1024];
  char v11; // stack - 0xc68
  char v12 [1032];
  char *v13; // stack - 0xc78
  char v14 [1023];
  char *v15; // stack - 0x458
  stat v16;
  char *v17; // stack - 0x868
  char *v18;
  unsigned long v19;
  int v2;
  char *v20;
  char *v21;
  char *v22;
  unsigned int v23; // r15d
  bool v24;
  char *v25; // stack - 0xd50
  struct_11 *v26; // stack - 0xd40
  int v27; // stack - 0xd24
  long v28; // stack - 0xd18
  unsigned long v29; // stack - 0xc70
  bool v3;
  unsigned long v30; // stack - 0x860
  long v31; // stack - 0x450
  char *v4;
  char v5;
  long v6;
  char *v7;
  unsigned long v8; // rax
  char *v9;
  
  v23 = a1 & 3;
  if ((v23 - 1 & v23) || (!a0)) {
    *__errno_location() = 0x16;
    return 0;
  }
  if (!*a0) {
    *__errno_location() = 2;
    return 0;
  }
  v30 = 0x400;
  v15 = v12;
  v17 = v10;
  v31 = 0x400;
  v29 = 0x400;
  v9 = &v11;
  v19 = 0x400;
  v13 = &v11;
  if (*a0 != '/') { // branch-flip
    while (!getcwd(v9,v19)) {
      v2 = *__errno_location();
      if (v2 == 0xc) {
        sub_18a90(); // no-return, return-dupe
      }
      if (v2 != 0x22) {
        v3 = 1;
        v7 = v9;
        goto label_f02f;
      }
      if (!sub_17540(&v13)) {
        sub_18a90();
      }
      v9 = v13;
      v19 = v29;
    }
    v7 = rawmemchr(v9,0);
    v5 = *a0;
    if (!v5) {
      if ((v7 <= &v9[1]) || (v7[-1] != '/'))
        v3 = 0;
      else {
        v7 = &v7[-1];
        v3 = 0;
      }
      goto label_f02f;
    }
  }
  else {
    v11 = '/';
    v7 = v14;
    v5 = '/';
  }
  v3 = 0;
  v26 = NULL;
  v27 = 0;
  v22 = v7;
label_f1bf:
  do {
    v7 = v22;
    if (v5 != '/') // branch-flip
      v25 = a0;
    else {
      do {
        v5 = a0[1];
        a0 = &a0[1];
      } while (v5 == '/');
      if (!v5) break;
      v25 = a0;
    }
    do {
      v18 = a0;
      v1 = v18[1];
      a0 = &v18[1];
      if (!v1) break;
    } while (v1 != '/');
    v19 = (long)a0 - (long)v25;
    if (!v19) break;
    if (v19 != 1) { // branch-flip
      if (((v5 != '.') || (v19 != 2)) || (v25[1] != '.')) {
label_f0f9:
        if (v22[-1] != '/') {
          *v22 = '/';
          v22 = &v22[1];
        }
        if (&v9[v29 - (long)v22] < (char *)(v19 + 2)) {
          do {
            v6 = (long)v22 - (long)v9;
            if (!sub_175c0(&v13)) {
              sub_18a90();
            }
            v22 = &v13[v6];
            v9 = v13;
          } while ((char *)(v29 - v6) < (char *)(v19 + 2));
        }
        v7 = mempcpy(v22,v25,v19);
        *v7 = '\0';
        v22 = v7;
        if (!(a1 & 4)) {
          while( true ) {
            v4 = v15;
            v19 = v31 - 1;
            v8 = readlink(v9,v15,v19);
            if ((long)v19 > (long)v8) break;
            if (!sub_17540(&v15)) {
              sub_18a90();
            }
          }
          if ((long)v8 < 0) goto label_f312;
          if (v27 <= 0x13) {
            v27 += 1;
label_f587:
            v21 = v17;
            v4[v8] = '\0';
            v6 = (long)a0 - (long)v17;
            if (!v3)
              v6 = v28;
            v25 = (unsigned long)strlen(a0);
            if (!SCARRY8((long)v25 + 0x8000000000000000,v8)) {
              if (v30 <= v8 + (long)v25) {
                do {
                  if (!sub_175c0(&v17)) {
                    sub_18a90();
                  }
                } while (v30 <= v8 + (long)v25);
                v21 = v17;
              }
              if (v3)
                a0 = &v21[v6];
              memmove(&v21[v8],a0,(long)v25 + 1);
              a0 = memcpy(v21,v4,v8);
              v22 = &v9[1];
              v28 = v6;
              if (*v4 != '/') { // branch-flip
                v3 = 1;
                v24 = v22 < v7;
                v22 = v7;
                v5 = *a0;
                if (v24) {
                  do {
                    v22 = &v7[-1];
                    if (v9 == v22) break;
                    v18 = &v7[-2];
                    v7 = v22;
                  } while (*v18 != '/');
                  v3 = 1;
                }
              }
              else {
                *v9 = '/';
                v3 = 1;
                v5 = *a0;
              }
              goto label_f268;
            }
            sub_18a90();
          }
          if (!*v25) goto label_f587;
          v7[(long)v25 - (long)a0] = '\0';
          v20 = ".";
          if (*v9)
            v20 = v9;
          if (!stat(v20,&v16)) {
            v7[(long)v25 - (long)a0] = *v25;
            if (!v26) {
              v26 = (struct_11 *)sub_10c60(7,NULL,sub_114c0,sub_114f0,sub_11530);
              if (!v26) {
                sub_18a90();
              }
            }
            if (!sub_f9c0(v26,v25,&v16)) {
              sub_f930(v26,v25,&v16);
              goto label_f587;
            }
            if (v23 != 2) {
              *__errno_location() = 0x28;
              goto label_f408;
            }
            goto label_f390;
          }
label_f408:
          v3 = 1;
          goto label_f288;
        }
label_f312:
        if (v23 != 2) {
          v5 = *a0;
          if (v5 == '/') {
            v4 = a0;
label_f330:
            do {
              v20 = v4;
              v1 = v20[1];
              v4 = &v20[1];
            } while (v1 == '/');
            if (!v1) {
label_f498:
              *(unsigned short *)v7 = dat_1e27e;
              goto label_f374;
            }
            if (v1 != '.') goto label_f3d0;
            v1 = v20[2];
            if (!v1) goto label_f498;
            if (v1 != '.') {
              if (v1 != '/') goto label_f3d0;
              v4 = &v20[2];
              goto label_f330;
            }
            if ((!v20[3]) || (v20[3] == '/')) goto label_f498;
          }
label_f3d0:
          if (a1 & 4) { // branch-flip
            if (v5) goto label_f1bf;
label_f374:
            if (faccessat(-100,v9,0,0x200)) {
label_f3e2:
              if (((v23 != 1) || (*__errno_location() != 2)) || (a0[strspn(a0,"/")])) goto label_f408;
            }
          }
          else if (*__errno_location() != 0x16) goto label_f3e2;
        }
label_f390:
        v5 = v18[1];
        if (!v5) break;
        goto label_f1bf;
      }
      v5 = v1;
      if (&v9[1] < v22) {
        v22 = &v22[-1];
        if (v9 < v22) {
          do {
            if (v22[-1] == '/') break;
            v22 = &v22[-1];
          } while (v22 != v9);
        }
      }
    }
    else {
      v24 = v5 != '.';
      v5 = v1;
      if (v24) goto label_f0f9;
    }
label_f268:
    v7 = v22;
  } while (v5);
  v3 = 0;
  if (&v9[1] < v7) {
    v7 = &v7[-(unsigned long)(v7[-1] == '/')];
    v3 = 0;
  }
label_f288:
  if (v26)
    sub_10e20(v26);
label_f02f:
  if (v17 != v10)
    free(v17);
  if (v15 != v12)
    free(v15);
  if (v3) {
    if (v13 != &v11) {
      free(v13);
      return 0;
    }
    return 0;
  }
  *v7 = '\0';
  v6 = sub_174e0(&v13,&v7[1 - (long)v9]);
  if (v6)
    return v6;
  sub_18a90();
}

// Function: sub_f750 @ 0xf750
void sub_f750(unsigned long a0)
{
  dat_263d8 = a0;
}

// Function: sub_f760 @ 0xf760
void sub_f760(char a0)
{
  dat_263d0 = a0;
}

// Function: sub_f770 @ 0xf770
void sub_f770(void)
{
  int v1; // eax
  int *v2; // rax
  char *v3; // rax
  char *v4; // rax
  
  v1 = sub_19af0(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!dat_263d0) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (dat_263d8) { // branch-flip
        v4 = (char *)sub_17030(dat_263d8);
        error(0,*v2,"%s: %s",v4,v3);
      }
      else {
        error(0,*v2,"%s",v3);
      }
      _exit(dat_251f8); // return-dupe, no-return
    }
  }
  v1 = sub_19af0(stderr);
  if (!v1)
    return;
  _exit(dat_251f8);
}

// Function: sub_f820 @ 0xf820
void sub_f820(char *a0)
{
  char v1;
  long v2;
  unsigned long v3;
  
  v1 = *a0;
  v3 = sub_ee20(a0) - (long)a0;
  do {
    if ((v1 == '/') >= v3) break;
    v2 = v3 - 1;
    v3 -= 1;
  } while (a0[v2] == '/');
}

// Function: sub_f860 @ 0xf860
char * sub_f860(char *a0)
{
  unsigned long v1;
  char v2;
  void *v3; // rax
  char *v4; // rax
  unsigned long v5;
  
  v2 = *a0;
  v5 = sub_ee20(a0) - (long)a0;
  do {
    if (v5 <= (v2 == '/')) {
      v3 = malloc(v5 + 1 + (v5 ^ 1));
      if (v3) { // branch-flip
        v4 = memcpy(v3,a0,v5);
        if ((v5 ^ 1) & 1) { // branch-flip
          *v4 = 0x2e;
          v5 = 1;
        }
        else {
          v5 = 1;
        }
label_f8ce:
        v4[v5] = 0;
      }
      else {
        v4 = NULL;
      }
      return v4;
    }
    v1 = v5 - 1;
    if (a0[v5 - 1] != '/') {
      v3 = malloc(v5 + 1);
      if (!v3) {
        v4 = NULL;
        return v4;
      }
      v4 = memcpy(v3,a0,v5);
      goto label_f8ce;
    }
    v5 = v1;
  } while( true );
}

// Function: sub_f920 @ 0xf920
unsigned long sub_f920(void)
{
  return 0;
}

// Function: sub_f930 @ 0xf930
void sub_f930(struct_11 *a0,char *a1,struct_9 *a2)
{
  unsigned long *v1; // rax
  unsigned long *v2; // rax
  
  if (!a0)
    return;
  v1 = (unsigned long *)sub_18590(0x18);
  *v1 = sub_18a70(a1);
  v1[1] = a2->field_0x8;
  v1[2] = a2->field_0x0;
  v2 = (unsigned long *)sub_112c0(a0,v1);
  if (v2) {
    if (v1 != v2) {
      sub_11530(v1); // tail-call
      return;
    }
    return;
  }
  sub_18a90(); // no-return
}

// Function: sub_f9c0 @ 0xf9c0
unsigned long sub_f9c0(struct_10 *a0,unsigned long a1,struct_9 *a2) // early-return
{
  unsigned long v1; // rax
  unsigned long v2; // stack - 0x28
  unsigned long v3; // stack - 0x20
  unsigned long v4; // stack - 0x18
  
  if (!a0)
    return 0;
  v3 = a2->field_0x8;
  v4 = a2->field_0x0;
  v2 = a1;
  v1 = sub_109d0(a0,&v2);
  return CONCAT71((undefined7)(v1 >> 8),v1 != 0);
}

// Function: sub_fa20 @ 0xfa20
void sub_fa20(unsigned int a0,struct_27 *a1) // ternary x2
{
  unsigned char v1;
  char v2; // cl
  char v3; // cl
  unsigned int v4; // esi
  
  v2 = 0x2d;
  v4 = a0 & 0xf000;
  if ((((v4 != 0x8000) && (v2 = 100, v4 != 0x4000)) && (v2 = 0x62, v4 != 0x6000)) && (((v2 = 99, v4 != 0x2000 && (v2 = 0x6c, v4 != 0xa000)) && ((v2 = 0x70, v4 != 0x1000 && (v2 = 0x73, v4 != 0xc000))))))
    v2 = 0x3f;
  a1->field_0x0 = v2;
  a1->field_0x1 = (-((a0 & 0x100) == 0) & 0xbbU) + 0x72;
  a1->field_0x2 = (-((a0 & 0x80) == 0) & 0xb6U) + 0x77;
  v1 = -((a0 & 0x40) == 0);
  v3 = (a0 & 0x800) ? (v1 & 0xe0) + 0x73 : (v1 & 0xb5) + 0x78; // branch-flip
  a1->field_0x3 = v3;
  a1->field_0x4 = (-((a0 & 0x20) == 0) & 0xbbU) + 0x72;
  a1->field_0x5 = (-((a0 & 0x10) == 0) & 0xb6U) + 0x77;
  v1 = -((a0 & 8) == 0);
  v3 = (a0 & 0x400) ? (v1 & 0xe0) + 0x73 : (v1 & 0xb5) + 0x78; // branch-flip
  a1->field_0x6 = v3;
  a1->field_0x7 = (-((a0 & 4) == 0) & 0xbbU) + 0x72;
  a1->field_0x8 = (-((a0 & 2) == 0) & 0xb6U) + 0x77;
  if (a0 & 0x200) {
    a1->field_0x9 = (-((a0 & 1) == 0) & 0xe0U) + 0x74;
    a1->field_0xa = 0x20;
    return;
  }
  a1->field_0x9 = (-((a0 & 1) == 0) & 0xb5U) + 0x78;
  a1->field_0xa = 0x20;
}

// Function: sub_fba0 @ 0xfba0
void sub_fba0(struct_28 *a0,struct_27 *a1)
{
  sub_fa20(a0->field_0x18,a1); // tail-call
}

// Function: sub_fbb0 @ 0xfbb0
void sub_fbb0(char *a0,char *a1,unsigned long *a2)
{
  if (sub_fbd0(a0,a1,a2))
    return;
  sub_18a90(); // no-return
}

// Function: sub_fbd0 @ 0xfbd0
void * sub_fbd0(void *a0,char *a1,unsigned long *a2)
{
  char *v1;
  long v2; // rax
  unsigned long v3; // rax
  void *v4; // rax
  char *v5; // rax
  unsigned long v6; // rbx
  char v7; // stack - 0x41
  
  v1 = (char *)sub_ee20(a0);
  v2 = sub_ee80(v1);
  v1 = &v1[v2 - (long)a0];
  v3 = strlen(a1);
  if (v2) { // branch-flip
    if (*(char *)((long)a0 + (long)&v1[-1]) != '/') { // branch-flip
      v7 = 0x2f;
      if (*a1 == '/')
        v7 = 0;
      v6 = (unsigned long)(*a1 != '/');
    }
    else {
      v7 = 0;
      v6 = 0;
    }
  }
  else {
    v7 = 0x2e;
    if (*a1 != '/')
      v7 = 0;
    v6 = (unsigned long)(*a1 == '/');
  }
  v4 = malloc((unsigned long)&v1[v6 + v3 + 1]);
  if (v4) {
    v5 = mempcpy(v4,a0,(unsigned long)v1);
    *v5 = v7;
    if (a2)
      *a2 = &v5[v6];
    *(char *)mempcpy(&v5[v6],a1,v3) = 0;
  }
  return v4;
}

// Function: sub_fcd0 @ 0xfcd0
unsigned long sub_fcd0(long a0,unsigned long *a1)
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
label_fd02:
  v6 = v4;
  v3 = v5;
  v5 = v3 + 1;
  v4 = v5;
  if ((v2 <= v5) || (*(char *)(a0 + v3) != '.')) {
    if (0 <= (long)v2) { // branch-flip
label_fcf0:
      if (v2 == v3) goto label_fd55;
    }
    else if (!*(char *)(a0 + v3)) {
label_fd55:
      *a1 = v3;
      return v6;
    }
    goto label_fd02;
  }
  v1 = *(char *)(a0 + 1 + v3);
  if ('[' <= v1) { // branch-flip
    if ((0x1a <= (unsigned char)(v1 + 0x9fU)) && (v1 != '~')) {
label_fce4:
      if (0 <= (long)v2) goto label_fcf0;
      goto label_fd02;
    }
  }
  else if (v1 <= '@') goto label_fce4;
  for (v5 = v3 + 2; v4 = v6, v5 < v2; v5 = v5 + 1) {
    v1 = *(char *)(a0 + v5);
    if ('[' <= v1) { // branch-flip
      if ((0x1a <= (unsigned char)(v1 + 0x9fU)) && (v1 != '~')) break;
    }
    else if ((v1 <= '@') && (10 <= (unsigned char)(v1 - 0x30U))) break;
  }
  goto label_fd02;
}

// Function: sub_fd80 @ 0xfd80
int sub_fd80(long a0,long a1,long a2,long a3) // ternary
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
label_fd8c:
  if (a1 <= v3) goto label_fef8;
label_fd95:
  v1 = *(unsigned char *)(a0 + v3);
  if ((unsigned int)((int)(char)v1 - 0x30U) <= 9) goto label_fe3c;
  v4 = 0;
  if ((unsigned int)v1 - 0x30 <= 9) goto label_fde0;
label_fdbc:
  v4 = (unsigned int)v1;
  if ('[' <= (char)v1) { // branch-flip
    if ((unsigned char)(v1 + 0x9f) <= 0x19) goto label_fde0;
  }
  else if ('@' < (char)v1) goto label_fde0;
  v4 = (v1 != 0x7e) ? v1 + 0x100 : 0xfffffffe; // branch-flip
label_fde0:
  if (v2 == a3) {
    v11 = 0xffffffff;
    return v4 - v11; // return-dupe
  }
label_fde9:
  v1 = *(unsigned char *)(a2 + v2);
  v9 = (unsigned int)v1;
  v12 = (unsigned int)v1;
  v11 = 0;
  v5 = v4;
  if ((unsigned int)v1 - 0x30 <= 9) goto label_fe22;
label_fdfe:
  v6 = (char)v9;
  v11 = v12; // branch-flip
  v4 = v5;
  if ('[' <= v6) {
    if (0x1a <= (unsigned char)(v6 + 0x9fU)) goto label_fe11;
  }
  else if (v6 <= '@') {
label_fe11:
    if (v6 != '~') // branch-flip
      v11 = v12 + 0x100;
    else {
      v11 = 0xfffffffe;
    }
  }
label_fe22:
  if (v11 == v4) {
    v3 += 1;
    v2 += 1;
label_fe33:
    if (v3 < a1) goto label_fd95;
label_fe3c:
    if (v2 < a3) {
      v9 = (unsigned int)(char)*(unsigned char *)(a2 + v2);
      if (9 < v9 - 0x30) goto label_ff10;
    }
    if (v3 < a1) {
      do {
        if (*(char *)(a0 + v3) != '0') {
          if (a3 <= v2) goto label_ffb0;
          goto label_fe9d;
        }
        v3 += 1;
      } while (a1 != v3);
    }
    if (v2 < a3) {
label_fe9d:
      do {
        if (*(char *)(a2 + v2) != '0') {
          v7 = 0;
          v8 = 0;
          if (a1 <= v3) goto label_1002f;
          goto label_feaf;
        }
        v2 += 1;
      } while (a3 != v2);
label_ffb0:
      if (v3 < a1) {
        v7 = 0;
        goto label_ffbc;
      }
      goto label_fef8;
    }
    goto label_fd8c;
  }
  return v4 - v11;
label_ff10:
  if (v3 != a1) goto label_ff19;
  v12 = (unsigned int)*(unsigned char *)(a2 + v2);
  v5 = 0xffffffff;
  v4 = 0xffffffff;
  if (v12 - 0x30 <= 9) {
    v11 = 0;
    return v4 - v11;
  }
  goto label_fdfe;
label_ff19:
  v1 = *(unsigned char *)(a0 + v3);
  if (9 < (unsigned int)v1 - 0x30) goto label_fdbc;
  v4 = 0;
  goto label_fde9;
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
label_feaf:
    if (a3 <= v2) goto label_1002f;
    if (10 <= (unsigned int)((int)*(char *)(a0 + v3) - 0x30U)) goto label_fecb;
  }
  goto label_fecb;
label_1002f:
  if (v3 < a1) {
label_ffbc:
    if ((unsigned int)((int)*(char *)(a0 + v3) - 0x30U) <= 9)
      return 1;
  }
label_fecb:
  if ((v2 < a3) && ((unsigned int)((int)*(char *)(a2 + v2) - 0x30U) <= 9))
    return -1;
  if (!v7) {
    if (v3 < a1) goto label_fd95;
label_fef8:
    if (v2 < a3) goto label_fe33;
    v7 = 0;
  }
  return v7;
}

// Function: sub_10040 @ 0x10040
unsigned long sub_10040(char *a0,long a1,char *a2,long a3) // early-return x4
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
    goto label_100a4;
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
label_10170:
    if (a2[1] != '.') goto label_100a4;
    v2 = 0;
  }
  else {
    if (a1 < 0) {
      if (!a0[2]) goto label_101ab;
      goto label_10170;
    }
    if (a1 != 2) goto label_10170;
label_101ab:
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
label_100a4:
  v6 = a3;
  v7 = a1;
  v3 = sub_fcd0(a0,&v7);
  v4 = sub_fcd0(a2,&v6);
  v1 = v7;
  if ((v7 != v3) || (v6 != v4)) {
    v5 = sub_fd80(a0,v3,a2,v4);
    if ((int)v5)
      return v5;
    v4 = v6;
  }
  return sub_fd80(a0,v1,a2,v4);
}

// Function: sub_101e0 @ 0x101e0
unsigned long sub_101e0(char *a0,char *a1)
{
  return sub_10040(a0,-1,a1,-1); // tail-call
}

// Function: sub_10200 @ 0x10200
void sub_10200(timespec *a0)
{
  clock_gettime(0,a0); // tail-call
}

// Function: sub_10210 @ 0x10210
timespec sub_10210(void)
{
  timespec v1; // stack - 0x28
  
  clock_gettime(0,&v1);
  return v1;
}

// Function: sub_10260 @ 0x10260
bool sub_10260(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_17800(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_102f0 @ 0x102f0
unsigned long sub_102f0(unsigned long a0,unsigned long a1)
{
  return (a0 >> 3 | a0 << 0x3d) % a1;
}

// Function: sub_10310 @ 0x10310
bool sub_10310(long a0,long a1)
{
  return a1 == a0;
}

// Function: sub_10320 @ 0x10320
unsigned long sub_10320(struct_4 *a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = a0->field_0x28;
  if (v2 == (float *)0x1e310)
    return 1;
  v1 = v2[2];
  if ((((dat_1e324 < v1) && (v1 < dat_1e328)) && (dat_1e32c < v2[3])) && (dat_1e340 <= *v2)) {
    v3 = *v2 + dat_1e324;
    if (((v3 < v2[1]) && (v2[1] <= dat_1e330)) && (v3 < v1))
      return 1;
  }
  a0->field_0x28 = (float *)0x1e310;
  return 0;
}

// Function: sub_103b0 @ 0x103b0
unsigned long sub_103b0(struct_5 *a0,long a1,unsigned long *a2,bool a3) // early-return
{
  long *v1;
  long v2;
  char v3; // al
  unsigned long v4; // rax
  long *v5;
  
  v4 = (*a0->field_0x30)(a1,a0->field_0x10);
  if ((unsigned long)a0->field_0x10 <= v4)
    abort(); // no-return
  v5 = (long *)(v4 * 0x10 + a0->field_0x0);
  *a2 = v5;
  v4 = *v5;
  if (!v4) {
    v4 = 0;
    return v4;
  }
  if (v4 != a1) {
    v3 = (*a0->field_0x38)(a1);
    if (!v3) {
      while( true ) {
        if (!(long *)v5[1]) {
          v4 = 0;
          return v4;
        }
        if ((*(long *)v5[1] == a1) || (v3 = (*a0->field_0x38)(a1), v3)) break;
        v5 = (long *)v5[1];
      }
      v1 = (long *)v5[1];
      v4 = *v1;
      if (!a3)
        return v4;
      v5[1] = v1[1];
      *v1 = 0;
      v1[1] = a0->field_0x48;
      a0->field_0x48 = (long)v1;
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
      v1[1] = a0->field_0x48;
      a0->field_0x48 = (long)v1;
      return v4;
    }
    *v5 = 0;
  }
  return v4;
}

// Function: sub_104c0 @ 0x104c0
unsigned long sub_104c0(float a0,unsigned long a1,bool a2) // ternary
{
  unsigned long v1; // rax
  unsigned long v2; // rcx
  unsigned long v3; // rsi
  long v4; // rdi
  unsigned long v5;
  
  if (!a2) {
    a0 = (float)a1 / a0;
    if (dat_1e334 <= a0) {
      v1 = 0;
      return v1;
    }
    a1 = (a0 < dat_1e338) ? (unsigned long)a0 : (long)(a0 - dat_1e338) ^ 0x8000000000000000;
  }
  v1 = 10;
  if (10 <= a1)
    v1 = a1;
  v5 = v1 | 1;
  do {
    if (v5 == 0xffffffffffffffff) {
      v1 = 0;
      return v1;
    }
    v1 = v5 / 3;
    if (10 <= v5) { // branch-flip
      if (v5 != (SUB168(ZEXT816(v5) * ZEXT816(0xaaaaaaaaaaaaaaab),8) & 0xfffffffffffffffe) + v1) {
        v4 = 0x10;
        v3 = 9;
        v2 = 3;
        do {
          v2 += 2;
          v3 += v4;
          if (v5 <= v3) goto label_10595;
          v4 += 8;
        } while (v5 % v2);
      }
    }
    else {
      v2 = 3;
label_10595:
      if (v5 % v2) {
        v1 = v5 >> 0x3d;
        if (v5 >> 0x3c & 1) {
          v1 = 0;
          return v1;
        }
        if (v1) {
          v1 = 0;
          return v1;
        }
        v1 = v5;
        return v1;
      }
    }
    v5 += 2;
  } while( true );
}

// Function: sub_10600 @ 0x10600
unsigned long sub_10600(struct_6 *a0,struct_31 *a1,bool a2) // return-dupe x2
{
  long v1;
  unsigned long v2; // rax
  long *v3; // rax
  unsigned long v4;
  long *v5;
  long *v6;
  long *v7;
  long v8;
  
  v6 = a1->field_0x0;
  if (a1->field_0x8 <= v6)
    return 1;
label_10636:
  do {
    v8 = *v6;
    if (v8) {
      if ((long *)v6[1]) {
        v4 = a0->field_0x10;
        v7 = (long *)v6[1];
        while( true ) {
          v8 = *v7;
          v2 = (*a0->field_0x30)(v8,v4);
          v4 = a0->field_0x10;
          if (v4 <= v2) {
            abort(); // no-return, return-dupe
          }
          v3 = (long *)(v2 * 0x10 + a0->field_0x0);
          v5 = (long *)v7[1];
          if (*v3) { // branch-flip
            v7[1] = v3[1];
            v3[1] = (long)v7;
          }
          else {
            *v3 = v8;
            v8 = a0->field_0x48;
            a0->field_0x18 = a0->field_0x18 + 1;
            *v7 = 0;
            v7[1] = v8;
            a0->field_0x48 = (long)v7;
          }
          if (!v5) break;
          v7 = v5;
        }
        v8 = *v6;
      }
      v6[1] = 0;
      if (!a2) {
        v4 = (*a0->field_0x30)(v8,a0->field_0x10);
        if ((unsigned long)a0->field_0x10 <= v4)
          abort();
        v7 = (long *)(v4 * 0x10 + a0->field_0x0);
        if (*v7) { // branch-flip
          v5 = (long *)a0->field_0x48;
          if (v5) // branch-flip
            a0->field_0x48 = v5[1];
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
          a0->field_0x18 = a0->field_0x18 + 1;
        }
        *v6 = 0;
        v6 = &v6[2];
        a1->field_0x18 = a1->field_0x18 + -1;
        if (a1->field_0x8 <= v6)
          return 1;
        goto label_10636;
      }
    }
    v6 = &v6[2];
    if (a1->field_0x8 <= v6)
      return 1;
  } while( true );
}

// Function: sub_10770 @ 0x10770
unsigned long sub_10770(void *a0)
{
  return *(unsigned long *)((long)a0 + 0x10);
}

// Function: sub_10780 @ 0x10780
unsigned long sub_10780(void *a0)
{
  return *(unsigned long *)((long)a0 + 0x18);
}

// Function: sub_10790 @ 0x10790
unsigned long sub_10790(struct_14 *a0)
{
  return a0->field_0x20;
}

// Function: sub_107a0 @ 0x107a0
unsigned long sub_107a0(struct_31 *a0)
{
  long v1;
  long *v2;
  unsigned long v3;
  unsigned long v4;
  
  v2 = a0->field_0x0;
  v4 = 0;
  do {
    if (a0->field_0x8 <= v2)
      return v4;
    while (*v2) {
      v3 = 1;
      for (v1 = v2[1]; v1; v1 = *(long *)(v1 + 8)) {
        v3 += 1;
      }
      if (v4 < v3)
        v4 = v3;
      v2 = &v2[2];
      if (a0->field_0x8 <= v2)
        return v4;
    }
    v2 = &v2[2];
  } while( true );
}

// Function: sub_10800 @ 0x10800
bool sub_10800(struct_31 *a0)
{
  long v1;
  long *v2;
  long v3;
  long v4;
  
  v2 = a0->field_0x0;
  v3 = 0;
  v4 = 0;
  do {
    while( true ) {
      if (a0->field_0x8 <= v2) {
        if (a0->field_0x18 != v4)
          return 0;
        return a0->field_0x20 == v3;
      }
      if (*v2) break;
label_10820:
      v2 = &v2[2];
    }
    v1 = v2[1];
    v4 += 1;
    v3 += 1;
    if (!v1) goto label_10820;
    do {
      v1 = *(long *)(v1 + 8);
      v3 += 1;
    } while (v1);
    v2 = &v2[2];
  } while( true );
}

// Function: sub_10870 @ 0x10870
void sub_10870(unsigned long *a0,FILE *a1)
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
      if ((long *)a0[1] <= v4) goto label_108dd;
    }
    v5 = 1;
    for (v3 = v4[1]; v3; v3 = *(long *)(v3 + 8)) {
      v5 += 1;
    }
    if (v6 < v5)
      v6 = v5;
  }
label_108dd:
  __fprintf_chk(a1,1,"# entries:         %lu\n",a0[4]);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v2,((double)v2 * dat_1e340) / (double)v1);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v6); // tail-call
  return;
}

// Function: sub_109d0 @ 0x109d0
long sub_109d0(struct_10 *a0,long a1) // return-dupe
{
  void *v1;
  unsigned long v2; // rax
  long *v3;
  long v4;
  
  v2 = (*a0->field_0x30)(a1,a0->field_0x10);
  if ((unsigned long)a0->field_0x10 <= v2)
    abort(); // no-return
  v3 = (long *)(v2 * 0x10 + a0->field_0x0);
  v4 = *v3;
  if (v4) {
    while( true ) {
      if (v4 == a1)
        return a1;
      v1 = a0->field_0x38;
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

// Function: sub_10a40 @ 0x10a40
long sub_10a40(struct_31 *a0)
{
  long *v1;
  
  if (!a0->field_0x20)
    return 0;
  v1 = a0->field_0x0;
  while( true ) {
    if (a0->field_0x8 <= v1)
      abort(); // no-return
    if (*v1) break;
    v1 = &v1[2];
  }
  return *v1;
}

// Function: sub_10a90 @ 0x10a90
long sub_10a90(struct_32 *a0,long a1)
{
  long v1;
  unsigned long v2; // rax
  long *v3; // rax
  long *v4;
  
  v2 = (*a0->field_0x30)(a1,a0->field_0x10);
  if ((unsigned long)a0->field_0x10 <= v2)
    abort(); // no-return
  v3 = (long *)(v2 * 0x10 + a0->field_0x0);
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
    if (a0->field_0x8 <= v3)
      return 0;
  } while (!*v3);
  return *v3;
}

// Function: sub_10b20 @ 0x10b20
void sub_10b20(struct_31 *a0,long a1,unsigned long a2)
{
  unsigned long v1; // rax
  long *v2;
  long *v3;
  
  v3 = a0->field_0x0;
  v1 = 0;
  if (a0->field_0x8 <= v3)
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
  } while (v3 < a0->field_0x8);
}

// Function: sub_10b70 @ 0x10b70
long sub_10b70(struct_31 *a0,void *a1,unsigned long a2)
{
  long *v1;
  long v2;
  long v3; // r12
  long *v4;
  
  v4 = a0->field_0x0;
  if (a0->field_0x8 <= v4) // branch-flip
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
    } while (v4 < a0->field_0x8);
  }
  return v3;
}

// Function: sub_10bf0 @ 0x10bf0
unsigned long sub_10bf0(unsigned char *a0,unsigned long a1)
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

// Function: sub_10c30 @ 0x10c30
void sub_10c30(struct_33 *a0)
{
  a0->field_0x10 = 0;
  a0->field_0x0 = 0x3f80000000000000;
  a0->field_0x8 = 0x3fb4fdf43f4ccccd;
}

// Function: sub_10c60 @ 0x10c60
unsigned long * sub_10c60(unsigned long a0,void *a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_102f0;
  if (!a3)
    a3 = sub_10310;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = (void *)0x1e310;
  v1[5] = a1;
  if (sub_10320(v1)) {
    v2 = sub_104c0(*(float *)((long)a1 + 8),a0,(unsigned char)*(char *)((long)a1 + 0x10));
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

// Function: sub_10d70 @ 0x10d70
void sub_10d70(struct_34 *a0)
{
  unsigned long *v1;
  unsigned long v2;
  void *v3;
  unsigned long *v4;
  long *v5;
  long *v6; // r12
  
  v5 = a0->field_0x0;
  if (a0->field_0x0 < a0->field_0x8) {
    do {
      while (!*v5) {
        v5 = &v5[2];
        if (a0->field_0x8 <= v5) goto label_10e0b;
      }
      v3 = a0->field_0x40;
      v4 = (unsigned long *)v5[1];
      if ((unsigned long *)v5[1]) {
        while( true ) {
          if (v3) {
            (*v3)(*v4);
            v3 = a0->field_0x40;
          }
          v1 = (unsigned long *)v4[1];
          v2 = a0->field_0x48;
          *v4 = 0;
          v4[1] = v2;
          a0->field_0x48 = v4;
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
    } while (v6 < a0->field_0x8);
  }
label_10e0b:
  a0->field_0x18 = 0;
  a0->field_0x20 = 0;
  return;
}

// Function: sub_10e20 @ 0x10e20
void sub_10e20(struct_11 *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = a0->field_0x0;
  v3 = a0->field_0x8;
  if ((a0->field_0x40) && (a0->field_0x20)) {
    if (v3 <= v4) goto label_10ecc;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_10e8f;
      }
      v3 = v4;
      while( true ) {
        (*(void *)a0->field_0x40)(v5);
        v3 = (long *)v3[1];
        if (!v3) break;
        v5 = *v3;
      }
      v3 = a0->field_0x8;
      v4 = &v4[2];
    } while (v4 < v3);
label_10e8f:
    v4 = a0->field_0x0;
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
    } while (v4 < a0->field_0x8);
  }
label_10ecc:
  v2 = a0->field_0x48;
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free(a0->field_0x0);
  free(a0); // tail-call
}

// Function: sub_10f10 @ 0x10f10
unsigned int sub_10f10(struct_8 *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_104c0(*(float *)(a0->field_0x28 + 8),a1,(unsigned char)*(char *)(a0->field_0x28 + 0x10));
  if (!v4)
    return 0;
  if (a0->field_0x10 == v4)
    return 1;
  v2 = calloc(v4,0x10);
  if (!v2)
    return 0;
  v3 = (void *)((long)v2 + v4 * 0x10);
  v5 = 0;
  v7 = a0->field_0x28;
  v6 = 0;
  v8 = a0->field_0x30;
  v9 = a0->field_0x38;
  v10 = a0->field_0x40;
  v11 = a0->field_0x48;
  v1 = sub_10600(&v2,a0,0);
  if (!(char)v1) {
    a0->field_0x48 = v11;
    if ((sub_10600(a0,&v2,1)) && (sub_10600(a0,&v2,0))) {
      free(v2);
      return v1;
    }
    abort(); // no-return
  }
  free(a0->field_0x0);
  a0->field_0x0 = v2;
  a0->field_0x8 = v3;
  a0->field_0x10 = v4;
  a0->field_0x18 = v5;
  a0->field_0x48 = v11;
  return v1;
}

// Function: sub_11090 @ 0x11090
long * sub_11090(void *a0,long a1,long *a2) // return-dupe, ternary
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
  v2 = (long)sub_103b0(a0,a1,&v3,0);
  if (v2) {
    if (!a2) {
      v2 = NULL;
      return v2;
    }
    *a2 = (long)v2;
    v2 = NULL;
    return v2;
  }
  v4 = *(unsigned long *)((long)a0 + 0x18);
  if (0 <= (long)v4) { // branch-flip
    v6 = (float)(long)v4;
    v2 = *(long **)((long)a0 + 0x10);
    if ((long)v2 > -1) goto label_1111f;
label_111a6:
    v5 = (float)v2;
  }
  else {
    v2 = *(long **)((long)a0 + 0x10);
    v6 = (float)v4;
    if (0 > (long)v2) goto label_111a6;
label_1111f:
    v5 = (float)(long)v2;
  }
  if (*(float *)(*(long *)((long)a0 + 0x28) + 8) * v5 < v6) {
    sub_10320(a0);
    v1 = *(long *)((long)a0 + 0x28);
    if (v5 * *(float *)(v1 + 8) < v6) {
      v5 = v5 * *(float *)(v1 + 0xc);
      if (!*(char *)(v1 + 0x10))
        v5 = v5 * *(float *)(v1 + 8);
      if (dat_1e334 <= v5) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v4 = (dat_1e338 <= v5) ? (long)(v5 - dat_1e338) ^ 0x8000000000000000 : (unsigned long)v5;
      if (!sub_10f10(a0,v4)) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v2 = (long)sub_103b0(a0,a1,&v3,0);
      if (v2)
        abort();
    }
  }
  if (!*v3) {
    *v3 = a1;
    *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + 1;
    *(long *)((long)a0 + 0x18) = *(long *)((long)a0 + 0x18) + 1;
    v2 = (long *)0x1;
    return v2;
  }
  v2 = *(long **)((long)a0 + 0x48);
  if (v2) // branch-flip
    *(long *)((long)a0 + 0x48) = v2[1];
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
  *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + 1;
  v2 = (long *)0x1;
  return v2;
}

// Function: sub_112c0 @ 0x112c0
unsigned long sub_112c0(void *a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_11090(a0,a1,&v3);
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

// Function: sub_11320 @ 0x11320
long sub_11320(void *a0,long a1) // ternary x2
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
  
  v5 = sub_103b0(a0,a1,&v7,1);
  if (!v5)
    return 0;
  *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + -1;
  if (!*v7) {
    v6 = *(long *)((long)a0 + 0x18) - 1;
    *(unsigned long *)((long)a0 + 0x18) = v6;
    v1 = (0 <= (long)v6) ? *(unsigned long *)((long)a0 + 0x10) : *(unsigned long *)((long)a0 + 0x10); // branch-flip
    v9 = (float)v6;
    v8 = (float)v1;
    if (v9 < **(float **)((long)a0 + 0x28) * v8) {
      sub_10320(a0);
      v2 = *(float **)((long)a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_1e338 <= v8) ? (long)(v8 - dat_1e338) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_10f10(a0,v6)) {
          v4 = *(void **)((long)a0 + 0x48);
          while (v4) {
            v3 = *(void **)((long)v4 + 8);
            free(v4);
            v4 = v3;
          }
          *(unsigned long *)((long)a0 + 0x48) = 0;
        }
      }
    }
    return v5;
  }
  return v5;
}

// Function: sub_114b0 @ 0x114b0
long sub_114b0(void *a0)
{
  return sub_11320(a0); // tail-call
}

// Function: sub_114c0 @ 0x114c0
unsigned long sub_114c0(struct_12 *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = sub_19cc0(a0->field_0x0,a1);
  return (v1 ^ a0->field_0x8) % a1;
}

// Function: sub_114f0 @ 0x114f0
unsigned long sub_114f0(unsigned long *a0,unsigned long *a1)
{
  int v1; // eax
  
  if ((a0[1] == a1[1]) && (a0[2] == a1[2])) {
    v1 = strcmp((char *)*a0,(char *)*a1);
    return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
  }
  return 0;
}

// Function: sub_11530 @ 0x11530
void sub_11530(unsigned long *a0)
{
  free((void *)*a0);
  free(a0); // tail-call
}

// Function: sub_11550 @ 0x11550
char * sub_11550(unsigned long a0,char *a1,unsigned int a2,unsigned long a3,unsigned long a4) // ternary
{
  unsigned int v1;
  char v10; // al
  unsigned int v11; // eax
  unsigned int v12; // eax
  lconv *v13; // rax
  unsigned long v14;
  unsigned long v15;
  unsigned long v16;
  unsigned long v17;
  long v18; // rax
  unsigned int v19; // edx
  unsigned char v2;
  unsigned int v20; // eax
  int v21;
  unsigned long v22;
  char v23 [56];
  char *v24;
  unsigned int v25; // eax
  char *v26;
  unsigned char *v27;
  char *v28;
  char *v29;
  bool v3;
  long double v30; // st0
  long double v31;
  char *v32; // stack - 0xc0
  unsigned int v33; // stack - 0xb8
  char v4 [16];
  char v5 [16];
  char v6 [16];
  char v7 [16];
  char v8 [16];
  unsigned long v9;
  
  v19 = a2 & 0x20;
  v33 = a2 & 3;
  v11 = (-(unsigned int)(v19 == 0) & 0xffffffe8) + 0x400;
  v13 = localeconv();
  v29 = *(char **)v13;
  v14 = strlen(v29);
  v27 = *(unsigned char **)&v13->field_0x10;
  v28 = *(char **)&v13->field_0x8;
  if (0x10 <= v14 - 1) {
    v14 = 1;
    v29 = ".";
  }
  if (0x11 <= strlen(v28))
    v28 = "";
  v32 = &a1[0x287];
  v12 = (unsigned int)v14; // branch-flip
  v24 = v32;
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
        v25 = 0;
        v16 = 0;
        v20 = 0;
        if (a2 & 0x10) goto label_117cc;
label_11bdb:
        v21 = -1;
label_117df:
        if (v33 != 1) goto label_117ef;
        if (5 < (int)((((unsigned int)v15 & 1) + v25 != 0) + v20)) goto label_117ff;
        goto label_11818;
      }
    }
label_11634:
    v30 = (long double)(long)a3;
    if ((long)a3 < 0)
      v30 = v30 + (long double)dat_1e334;
    if (0 <= (long)a4) { // branch-flip
      v30 = v30 / (long double)(long)a4;
      if ((long)a0 > -1) goto label_11671;
label_11920:
      v30 = ((long double)(long)a0 + (long double)dat_1e334) * v30;
      if (!(a2 & 0x10)) goto label_11968;
label_1167e:
      v21 = 0;
      v31 = (long double)(int)v11;
      do {
        v21 += 1;
        v31 = v31 * (long double)(int)v11;
        if (v30 < v31) break;
      } while (v21 != 8);
      if (v33 != 1) { // branch-flip
        __sprintf_chk(a1,1,0xffffffffffffffff,"%.1Lf");
        v17 = strlen(a1);
      }
      else {
        __sprintf_chk(a1,1,0xffffffffffffffff,"%.1Lf");
        v17 = strlen(a1);
      }
      if ((v17 <= v14 + 2 + (unsigned long)(v19 == 0)) && ((!(a2 & 8) || (a1[v17 - 1] != '0'))))
        v14 = v17 - (v14 + 1);
      else {
        __sprintf_chk(a1,1,0xffffffffffffffff,"%.0Lf");
        v17 = strlen(a1);
        v14 = v17;
      }
    }
    else {
      v30 = v30 / ((long double)(long)a4 + (long double)dat_1e334);
      if (0 > (long)a0) goto label_11920;
label_11671:
      v30 = (long double)(long)a0 * v30;
      if (a2 & 0x10) goto label_1167e;
label_11968:
      __sprintf_chk(a1,1,0xffffffffffffffff,"%.0Lf");
      v21 = -1;
      v17 = strlen(a1);
      v14 = v17;
    }
    v29 = &v32[-v17];
    memmove(v29,a1,v17);
    v24 = &v29[v14];
  }
  else {
    if ((!a3) || (v17 = a4 / a3, a4 % a3)) goto label_11634;
    v15 = a0 / v17;
    v16 = (a0 % v17) * 10;
    v9 = v16 / v17;
    v22 = (v16 % v17) * 2;
    v8._8_8_ = v22;
    v8._0_8_ = v15;
    v16 = v9 & 0xffffffff;
    v20 = (unsigned int)v9;
    if (v17 <= v22) { // branch-flip
      v25 = 2;
      if (v17 < v22)
        v25 = 3;
    }
    else {
      v25 = (unsigned int)(v22 != 0);
    }
    if (!(a2 & 0x10)) goto label_11bdb;
label_117cc:
    v20 = (unsigned int)v16;
    v15 = SUB168(v8._0_16_,0);
    v22 = (unsigned long)v11;
    v21 = 0;
    v17 = v15;
    if (v15 < v22) goto label_117df;
    while( true ) {
      v15 = v17 / v22;
      v20 = (int)v16 + (int)(v17 % v22) * 10;
      v16 = v20 / v22;
      v1 = ((int)v25 >> 1) + (v20 % v11) * 2;
      v20 = (unsigned int)v16;
      v25 += v1;
      v25 = (v11 <= v1) ? (v11 < v25) + 2 : (unsigned int)(v25 != 0); // branch-flip
      v21 += 1;
      if (v15 < v22) break;
      v17 = v15;
      if (v21 == 8) goto label_117df;
    }
    if (10 <= v15) goto label_117df;
    v17 = v15;
    if (v33 != 1) { // branch-flip
      if ((!v33) && (v25)) goto label_11f67;
label_11fdc:
      if (!v20) goto label_11fe4;
      v10 = (char)v16 + '0';
    }
    else {
      if (3 > (v20 & 1) + v25) goto label_11fdc;
label_11f67:
      if (v20 != 9) // branch-flip
        v10 = (char)v16 + '1';
      else {
        v17 = v15 + 1;
        if (v15 == 9) goto label_11faa;
        v25 = 0;
        v15 = v17;
label_11fe4:
        if (a2 & 8) goto label_11fac;
        v10 = '0';
        v17 = v15;
      }
    }
    a1[0x286] = v10;
    v16 = v14 & 0xffffffff;
    v24 = &a1[0x286 - v14];
    if (8 <= v12) { // branch-flip
      *(unsigned long *)v24 = *(unsigned long *)v29;
      *(unsigned long *)&v24[(v14 & 0xffffffff) - 8] = *(unsigned long *)&v29[(v14 & 0xffffffff) - 8];
      v18 = (long)v24 - ((unsigned long)&v24[8] & 0xfffffffffffffff8);
      v20 = (int)v18 + v12 & 0xfffffff8;
      if (8 <= v20) {
        v25 = 0;
        do {
          v16 = (unsigned long)v25;
          v25 += 8;
          *(unsigned long *)(((unsigned long)&v24[8] & 0xfffffffffffffff8) + v16) = *(unsigned long *)&v29[v16 - v18];
        } while (v25 < v20);
      }
    }
    else if (v14 & 4) { // branch-flip
      *(unsigned int *)v24 = *(unsigned int *)v29;
      *(unsigned int *)&v24[v16 - 4] = *(unsigned int *)&v29[v16 - 4];
    }
    else if (v12) {
      *v24 = *v29;
      if (v14 & 2)
        *(unsigned short *)&v24[v16 - 2] = *(unsigned short *)&v29[v16 - 2];
    }
label_11faa:
    v25 = 0;
    v15 = v17;
label_11fac:
    v20 = 0;
    if (v33 == 1) goto label_11818;
label_117ef:
    if (v33) { // branch-flip
label_11818:
      v29 = v24;
      do {
        v29 = &v29[-1];
        *v29 = (char)v15 + (char)(v15 / 10) * '\xf6' + '0';
        v3 = 10 <= v15;
        v15 /= 10;
      } while (v3);
    }
    else {
      if (1 > (int)(v25 + v20)) goto label_11818;
label_117ff:
      v15 += 1;
      if (((!(a2 & 0x10)) || (v11 != v15)) || (v21 == 8)) goto label_11818;
      v21 += 1;
      if (!(a2 & 8)) {
        v24[-1] = '0';
        v24 = &v24[~v14];
        v17 = v14 & 0xffffffff;
        if (8 <= v12) { // branch-flip
          *(unsigned long *)v24 = *(unsigned long *)v29;
          *(unsigned long *)&v24[(v14 & 0xffffffff) - 8] = *(unsigned long *)&v29[(v14 & 0xffffffff) - 8];
          v18 = (long)v24 - ((unsigned long)&v24[8] & 0xfffffffffffffff8);
          v12 = (int)v18 + v12 & 0xfffffff8;
          if (8 <= v12) {
            v20 = 0;
            do {
              v14 = (unsigned long)v20;
              v20 += 8;
              *(unsigned long *)(((unsigned long)&v24[8] & 0xfffffffffffffff8) + v14) = *(unsigned long *)&v29[v14 - v18];
            } while (v20 < v12);
          }
        }
        else if (v14 & 4) { // branch-flip
          *(unsigned int *)v24 = *(unsigned int *)v29;
          *(unsigned int *)&v24[v17 - 4] = *(unsigned int *)&v29[v17 - 4];
        }
        else if ((v12) && (*v24 = *v29, v14 & 2))
          *(unsigned short *)&v24[v17 - 2] = *(unsigned short *)&v29[v17 - 2];
      }
      v24[-1] = '1';
      v29 = &v24[-1];
    }
  }
  if (a2 & 4) {
    v17 = 0xffffffffffffffff;
    v16 = strlen(v28);
    __memcpy_chk(v23,v29,(long)v24 - (long)v29,0x29);
    v14 = (long)v24 - (long)v29;
    while( true ) {
      v2 = *v27;
      if (v2) { // branch-flip
        if (0x7f <= v2) { // branch-flip
          v22 = 0;
          v26 = v23;
        }
        else {
          v17 = (unsigned long)v2;
          if (v14 < v2)
            v17 = v14;
          v22 = v14 - v17;
          v26 = &v23[v22];
          v14 = v17;
        }
        v27 = &v27[1];
        v17 = v14;
      }
      else {
        if (v14 < v17)
          v17 = v14;
        v22 = v14 - v17;
        v26 = &v23[v22];
      }
      v29 = &v24[-v17];
      memcpy(v29,v26,v17);
      if (!v22) break;
      v24 = &v29[-v16];
      memcpy(v24,v28,v16);
      v14 = v22;
    }
  }
  if (!(a2 & 0x80)) goto label_11a7f;
  if (v21 != -1) { // branch-flip
label_11a6c:
    if (!(a2 & 0x100) && !v21) goto label_11a7f;
    if (a2 & 0x40) {
label_11c29:
      v32 = &a1[0x288];
      a1[0x287] = ' ';
    }
    if (v21) goto label_11c48;
    v28 = v32;
    if (!(a2 & 0x100)) goto label_11a7f;
  }
  else {
    if (a4 < 2) {
      v21 = 0;
      goto label_11a6c;
    }
    v21 = 1;
    v14 = 1;
    do {
      v14 *= v11;
      if (a4 <= v14) break;
      v21 += 1;
    } while (v21 != 8);
    if (a2 & 0x40) goto label_11c29;
label_11c48:
    v28 = &v32[1];
    if ((v19) || (v21 != 1)) {
      *v32 = *(char *)((long)v21 + 0x1e388);
      if (!(a2 & 0x100)) {
label_11f3b:
        v32 = v28;
        goto label_11a7f;
      }
      if (v19) {
        v32[1] = 'i';
        v28 = &v32[2];
      }
    }
    else {
      *v32 = 'k';
      if (!(a2 & 0x100)) goto label_11f3b;
    }
  }
  *v28 = 'B';
  v32 = &v28[1];
label_11a7f:
  *v32 = '\0';
  return v29;
}

// Function: sub_12150 @ 0x12150
unsigned long sub_12150(unsigned char *a0,unsigned int *a1,unsigned long *a2) // early-return x4
{
  unsigned char *v1;
  unsigned char v2;
  int v3; // eax
  unsigned long v4; // rax
  unsigned long v5;
  unsigned char *v6; // stack - 0x38
  unsigned int v7; // r12d
  
  if (((!a0) && (a0 = (unsigned char *)getenv("BLOCK_SIZE"), !a0)) && (a0 = (unsigned char *)getenv("BLOCKSIZE"), !a0)) {
    if (getenv("POSIXLY_CORRECT")) {
      *a2 = 0x200;
      *a1 = 0;
      return 0;
    }
    *a2 = 0x400;
    *a1 = 0;
    return 0;
  }
  v7 = 0;
  if (*a0 == 0x27) {
    a0 = &a0[1];
    v7 = 4;
  }
  v3 = sub_e9c0(a0,(long *)0x249f0,(void *)0x1e380,4);
  if (0 <= v3) {
    *a2 = 1;
    *a1 = v7 | *(unsigned int *)((long)v3 * 4 + 0x1e380);
    return 0;
  }
  v4 = sub_18de0(a0,&v6,0,a2,"eEgGkKmMpPtTyYzZ0");
  if ((int)v4) { // branch-flip
    *a1 = 0;
    v5 = *a2;
  }
  else {
    v2 = *a0;
    while (9 < (unsigned char)(v2 - 0x30)) {
      if (v6 == a0) {
        if (v6[-1] != 0x42) // branch-flip
          v7 |= 0x80;
        else {
          v7 |= 0x180;
          if (v6[-2] != 0x69) break;
        }
        v7 |= 0x20;
        break;
      }
      v1 = &a0[1];
      a0 = &a0[1];
      v2 = *v1;
    }
    v5 = *a2;
    *a1 = v7;
  }
  if (v5)
    return v4;
  *a2 = (unsigned long)(-(unsigned int)(getenv("POSIXLY_CORRECT") == NULL) & 0x200) + 0x200;
  return 4;
}

// Function: sub_12340 @ 0x12340
unsigned int * sub_12340(unsigned int a0) // early-return
{
  passwd *v1; // rax
  unsigned int *v2;
  char *v3;
  
  for (v2 = dat_263f8; v2; v2 = *(unsigned int **)&v2[2]) {
    if (*v2 == a0) goto label_1236d;
  }
  v3 = "";
  v1 = getpwuid(a0);
  if (v1)
    v3 = *(char **)v1;
  v2 = (unsigned int *)sub_18590(strlen(v3) + 0x18 & 0xfffffffffffffff8);
  *v2 = a0;
  strcpy((char *)&v2[4],v3);
  *(unsigned int **)&v2[2] = dat_263f8;
  dat_263f8 = v2;
label_1236d:
  if ((char)v2[4])
    return &v2[4];
  return NULL;
}

// Function: sub_123d0 @ 0x123d0
unsigned int * sub_123d0(char *a0)
{
  char v1;
  passwd *v2; // rax
  unsigned int *v3;
  long v4;
  
  if (dat_263f8) {
    v1 = *a0;
    v3 = dat_263f8;
    do {
      if ((*(char *)&v3[4] == v1) && (!strcmp((char *)&v3[4],a0)))
        return v3;
      v3 = *(unsigned int **)&v3[2];
    } while (v3);
  }
  if (dat_263f0) {
    v1 = *a0;
    v4 = (long)dat_263f0;
    do {
      if ((*(char *)(v4 + 0x10) == v1) && (!strcmp((char *)(v4 + 0x10),a0)))
        return NULL;
      v4 = *(long *)(v4 + 8);
    } while (v4);
  }
  v2 = getpwnam(a0);
  v3 = (unsigned int *)sub_18590(strlen(a0) + 0x18 & 0xfffffffffffffff8);
  strcpy((char *)&v3[4],a0);
  v4 = (long)dat_263f0;
  if (v2) {
    *v3 = *(unsigned int *)&v2->field_0x10;
    *(unsigned int **)&v3[2] = dat_263f8;
    dat_263f8 = v3;
    return v3;
  }
  dat_263f0 = v3;
  *(long *)&v3[2] = v4;
  return NULL;
}

// Function: sub_124e0 @ 0x124e0
unsigned int * sub_124e0(unsigned int a0) // early-return
{
  group *v1; // rax
  unsigned int *v2;
  char *v3;
  
  for (v2 = dat_263e8; v2; v2 = *(unsigned int **)&v2[2]) {
    if (*v2 == a0) goto label_1250d;
  }
  v3 = "";
  v1 = getgrgid(a0);
  if (v1)
    v3 = *(char **)v1;
  v2 = (unsigned int *)sub_18590(strlen(v3) + 0x18 & 0xfffffffffffffff8);
  *v2 = a0;
  strcpy((char *)&v2[4],v3);
  *(unsigned int **)&v2[2] = dat_263e8;
  dat_263e8 = v2;
label_1250d:
  if ((char)v2[4])
    return &v2[4];
  return NULL;
}

// Function: sub_12570 @ 0x12570
unsigned int * sub_12570(char *a0)
{
  char v1;
  group *v2; // rax
  unsigned int *v3;
  long v4;
  
  if (dat_263e8) {
    v1 = *a0;
    v3 = dat_263e8;
    do {
      if ((*(char *)&v3[4] == v1) && (!strcmp((char *)&v3[4],a0)))
        return v3;
      v3 = *(unsigned int **)&v3[2];
    } while (v3);
  }
  if (dat_263e0) {
    v1 = *a0;
    v4 = (long)dat_263e0;
    do {
      if ((*(char *)(v4 + 0x10) == v1) && (!strcmp((char *)(v4 + 0x10),a0)))
        return NULL;
      v4 = *(long *)(v4 + 8);
    } while (v4);
  }
  v2 = getgrnam(a0);
  v3 = (unsigned int *)sub_18590(strlen(a0) + 0x18 & 0xfffffffffffffff8);
  strcpy((char *)&v3[4],a0);
  v4 = (long)dat_263e0;
  if (v2) {
    *v3 = *(unsigned int *)&v2->field_0x10;
    *(unsigned int **)&v3[2] = dat_263e8;
    dat_263e8 = v3;
    return v3;
  }
  dat_263e0 = v3;
  *(long *)&v3[2] = v4;
  return NULL;
}

// Function: sub_12680 @ 0x12680
char * sub_12680(unsigned long a0,void *a1)
{
  unsigned long v1;
  char *v2;
  char *v3; // r8
  
  *(char *)((long)a1 + 0x14) = 0;
  v2 = (char *)((long)a1 + 0x14);
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

// Function: sub_12720 @ 0x12720
char * sub_12720(unsigned long a0,void *a1)
{
  bool v1;
  char *v2;
  
  *(char *)((long)a1 + 0x14) = 0;
  v2 = (char *)((long)a1 + 0x14);
  do {
    v2 = &v2[-1];
    *v2 = (char)a0 + (char)(a0 / 10) * '\xf6' + '0';
    v1 = 10 <= a0;
    a0 /= 10;
  } while (v1);
  return v2;
}

// Function: sub_12780 @ 0x12780
unsigned long sub_12780(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_10260(0))) {
    v1 = 1;
    *a0 = (unsigned int)(unsigned char)*a1;
  }
  return v1;
}

// Function: sub_12810 @ 0x12810
unsigned long sub_12810(char *a0,char *a1,long a2,unsigned long *a3,int a4,unsigned long a5)
{
  wchar_t *v1;
  char *v10;
  char *v11;
  wchar_t *v12;
  wchar_t v13;
  unsigned long v14;
  unsigned long v15; // stack - 0x60
  unsigned long v16; // stack - 0x40
  char *v2;
  bool v3;
  int v4; // eax
  char *v5;
  wchar_t *v6; // rax
  char *v7; // rax
  unsigned long v8;
  unsigned long v9;
  
  v15 = strlen(a0);
  if (a5 & 2) { // branch-flip
label_1284c:
    v6 = NULL;
    v7 = NULL;
    v9 = v15;
label_12855:
    v14 = *a3;
    if (v9 <= v14) goto label_1296b;
    v9 = v14;
    v15 = v14;
label_1286b:
    v14 = 0;
    *a3 = v9;
    if (a4) goto label_1287f;
label_12991:
    v9 = 0;
    v8 = v14;
  }
  else {
    if (__ctype_get_mb_cur_max() < 2) goto label_1284c;
    v14 = mbstowcs(NULL,a0,0);
    if (v14 == 0xffffffffffffffff) {
      if (!(a5 & 1)) {
        v6 = NULL; // return-dupe
        v7 = NULL;
        v14 = 0xffffffffffffffff;
        free(v6); // return-dupe
        free(v7);
        return v14;
      }
      goto label_1284c;
    }
    v9 = v14 + 1;
    v6 = malloc(v9 * 4);
    if (!v6) {
      if (!(a5 & 1)) {
        v6 = NULL;
        v7 = NULL;
        v14 = 0xffffffffffffffff;
        free(v6);
        free(v7);
        return v14;
      }
label_12bc4:
      v7 = NULL;
      v9 = v15;
      goto label_12855;
    }
    if (!__mbstowcs_chk(v6,a0,v9,0x3fffffffffffffff)) goto label_12bc4;
    v6[v14] = L'\0';
    v13 = *v6;
    if (v13) {
      v3 = 0;
      v12 = v6;
      do {
        if (!iswprint(v13)) {
          *v12 = L'�';
          v3 = 1;
        }
        v13 = v12[1];
        v12 = &v12[1];
      } while (v13);
      v9 = (unsigned long)wcswidth(v6,v9);
      if (!v3) goto label_12b9b;
      v16 = wcstombs(NULL,v6,0);
label_12ae4:
      v16 += 1;
      v7 = malloc(v16);
      if (v7) { // branch-flip
        v14 = *a3;
        v13 = *v6;
        v12 = v6;
        v9 = 0;
        while (v13) {
          v4 = wcwidth(v13);
          if (v4 != -1) // branch-flip
            v8 = (long)v4 + v9;
          else {
            *v12 = L'�';
            v8 = v9 + 1;
          }
          if (v14 < v8) break;
          v1 = &v12[1];
          v12 = &v12[1];
          v13 = *v1;
          v9 = v8;
        }
        *v12 = L'\0';
        v15 = wcstombs(v7,v6,v16);
        a0 = v7;
      }
      else {
        v14 = 0xffffffffffffffff;
        if (!(a5 & 1)) {
          free(v6);
          free(v7);
          return v14;
        }
      }
      goto label_12855;
    }
    v9 = (unsigned long)wcswidth(v6,v9);
label_12b9b:
    v14 = *a3;
    v16 = v15;
    if (v14 < v9) goto label_12ae4;
    v7 = NULL;
label_1296b:
    if (v14 <= v9) goto label_1286b;
    v14 -= v9;
    *a3 = v9;
    if (!a4) goto label_12991;
label_1287f:
    v9 = v14;
    v8 = 0;
    if (a4 != 1) {
      v9 = (unsigned long)((unsigned int)v14 & 1) + (v14 >> 1);
      v8 = v14 >> 1;
    }
  }
  v14 = v9 + v15;
  if (a5 & 4) {
    v9 = 0;
    v14 = v15;
  }
  if (a5 & 8) // branch-flip
    v8 = 0;
  else {
    v14 += v8;
  }
  if (a2) {
    v2 = &a1[a2 + -1];
    v10 = a1;
    if ((a1 < v2) && (v5 = a1, v9)) {
      do {
        v10 = &v5[1];
        *v5 = 0x20;
        if (!&a1[v9 - (long)v10]) break;
        v5 = v10;
      } while (v10 < v2);
    }
    v9 = (long)v2 - (long)v10;
    if (v15 < (unsigned long)((long)v2 - (long)v10))
      v9 = v15;
    v5 = mempcpy(v10,a0,v9);
    v10 = v5;
    if ((v5 < v2) && (v11 = v5, v8)) {
      do {
        v10 = &v11[1];
        *v11 = 0x20;
        if (!&v5[v8 - (long)v10]) break;
        v11 = v10;
      } while (v10 < v2);
    }
    *v10 = 0;
  }
  free(v6);
  free(v7);
  return v14;
}

// Function: sub_12bf0 @ 0x12bf0
char * sub_12bf0(char *a0,unsigned long *a1,unsigned int a2,unsigned int a3)
{
  unsigned long v1;
  unsigned long v2;
  char *v3; // rax
  unsigned long v4;
  char *v5;
  
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
    v4 = sub_12810(a0,v3,v1,a1,a2,a3);
    if (v4 == 0xffffffffffffffff) break;
    v5 = v3;
    if (v4 < v1)
      return v3;
  }
  free(v3);
  return NULL;
}

// Function: sub_12ca0 @ 0x12ca0
int sub_12ca0(char *a0,long a1,unsigned int a2)
{
  char *v1;
  int v10; // r12d
  char v2;
  unsigned short v3;
  int v4; // eax
  long v5; // rax
  long *v6; // rax
  long v7;
  mbstate_t v8; // stack - 0x48
  unsigned int v9; // stack - 0x4c
  
  v1 = &a0[a1];
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
    v10 = 0;
    if (a0 < v1) {
      do {
        v2 = *a0;
        if ('`' <= v2) { // branch-flip
          if (0x1e <= (unsigned char)(v2 + 0x9fU)) goto label_12d24;
label_12dd3:
          a0 = &a0[1];
          v10 += 1;
        }
        else {
          if ('@' < v2) goto label_12dd3;
          if ('#' < v2) {
            if (0x1b <= (unsigned char)(v2 - 0x25U)) goto label_12d24;
            goto label_12dd3;
          }
          if ('\x1f' < v2) goto label_12dd3;
label_12d24:
          v8 = 0;
          do {
            v5 = sub_12780(&v9,a0,(long)v1 - (long)a0,&v8);
            if (v5 == -1) {
              if (a2 & 1)
                return -1;
              goto label_12dd3;
            }
            if (v5 == -2) {
              if (a2 & 1)
                return -1;
              v10 += 1;
              a0 = v1;
              break;
            }
            v7 = 1;
            if (v5)
              v7 = v5;
            v4 = wcwidth(v9);
            if (0 <= v4) { // branch-flip
              if (0x7fffffffU - v10 < v4)
                return 0x7fffffff;
              v10 += v4;
            }
            else {
              if (a2 & 2)
                return -1;
              if (!iswcntrl(v9)) {
                if (v10 == 0x7fffffff)
                  return 0x7fffffff;
                v10 += 1;
              }
            }
            a0 = &a0[v7];
          } while (!mbsinit(&v8));
        }
      } while (a0 < v1);
    }
  }
  else {
    v10 = 0;
    if (a0 < v1) {
      v6 = __ctype_b_loc();
      v10 = 0;
      do {
        v2 = *a0;
        a0 = &a0[1];
        v3 = *(unsigned short *)(*v6 + (unsigned long)(unsigned char)v2 * 2);
        if (v3 & 0x4000) { // branch-flip
label_12e59:
          if (v10 == 0x7fffffff)
            return 0x7fffffff;
          v10 += 1;
        }
        else {
          if (a2 & 2)
            return -1;
          if (!(v3 & 2)) goto label_12e59;
        }
      } while (v1 != a0);
    }
  }
  return v10;
}

// Function: sub_12ec0 @ 0x12ec0
void sub_12ec0(char *a0,unsigned int a1)
{
  sub_12ca0(a0,strlen(a0),a1); // tail-call
}

// Function: sub_12ef0 @ 0x12ef0
void sub_12ef0(unsigned long *a0,unsigned long a1,unsigned long *a2,void *a3)
{
  unsigned long *v1;
  long v10;
  unsigned long v11; // stack - 0x68
  unsigned long v12; // stack - 0x48
  int v2; // eax
  unsigned long v3; // rax
  unsigned long v4;
  unsigned long v5;
  unsigned long *v6;
  unsigned long *v7;
  unsigned long v8;
  long v9;
  
  v3 = a1 >> 1;
  v4 = a1 - v3;
  v1 = &a0[v3];
  if (3 <= v4) // branch-flip
    sub_12ef0(v1,v4,a2,a3);
  else if (v4 == 2) {
    v8 = *v1;
    v5 = v1[1];
    if (1 <= (int)(*a3)(v8,v5)) {
      *v1 = v5;
      v1[1] = v8;
    }
  }
  if (a1 == 3) {
    v8 = *a0;
    *a2 = v8;
    goto label_12f4c;
  }
  v12 = a1 >> 2;
  v4 = v3 - v12;
  v6 = &a0[v12];
  if (3 <= v4) // branch-flip
    sub_12ef0(v6,v4,a2,a3);
  else if (v4 == 2) {
    v8 = *v6;
    v5 = v6[1];
    if (1 <= (int)(*a3)(v8,v5)) {
      *v6 = v5;
      v6[1] = v8;
    }
  }
  if (3 <= v12) { // branch-flip
    sub_12ef0(a0,v12,a2,a3);
label_13149:
    v5 = *a0;
  }
  else {
    v8 = *a0;
    v5 = v8;
    if (v12 == 2) {
      v5 = a0[1];
      if ((int)(*a3)(v8,v5) <= 0) goto label_13149;
      a0[1] = v8;
      *a0 = v5;
    }
  }
  v8 = *v6;
  v11 = 0;
  v4 = v12;
  v6 = a2;
  while( true ) {
    v2 = (*a3)(v5,v8);
    v7 = v6;
    while (v6 = &v7[1], 1 <= v2) {
      *v7 = v8;
      v4 += 1;
      if (v3 == v4) goto label_130de;
      v8 = a0[v4];
      v2 = (*a3)(v5,v8);
      v7 = v6;
    }
    v11 += 1;
    *v7 = v5;
    if (v12 == v11) break;
    v5 = a0[v11];
  }
  v11 = v4;
  v12 = v3;
label_130de:
  memcpy(v6,&a0[v11],(v12 - v11) * 8);
  v8 = *a2;
label_12f4c:
  v11 = 0;
  v9 = 0;
  v5 = *v1;
  v4 = v3;
  while( true ) {
    v2 = (*a3)(v8,v5);
    v10 = v9;
    while (v9 = v10 + 1, 1 <= v2) {
      a0[v10] = v5;
      v4 += 1;
      if (a1 == v4) {
        memcpy(&a0[v9],&a2[v11],(v3 - v11) * 8); // tail-call
        return;
      }
      v5 = a0[v4];
      v2 = (*a3)(v8,v5);
      v10 = v9;
    }
    v11 += 1;
    a0[v10] = v8;
    if (v3 == v11) break;
    v8 = a2[v11];
  }
}

// Function: sub_131b0 @ 0x131b0
void sub_131b0(unsigned long *a0,unsigned long a1,void *a2) // return-dupe
{
  unsigned long v1;
  unsigned long v2;
  
  if (3 <= a1) {
    sub_12ef0(a0,a1,&a0[a1],a2); // tail-call
    return;
  }
  if (a1 != 2)
    return;
  v1 = a0[1];
  v2 = *a0;
  if ((int)(*a2)(v2,v1) <= 0)
    return;
  *a0 = v1;
  a0[1] = v2;
}

// Function: sub_13210 @ 0x13210
long sub_13210(char *a0,long a1,char *a2,tm *a3,char a4,unsigned int a5,int a6,long *a7,int a8)
{
  int v1;
  unsigned char *v10;
  long v11;
  long v12;
  char v13;
  unsigned int v14; // edx
  int v15;
  unsigned int v16;
  unsigned long v17;
  unsigned long v18; // rdx
  char *v19;
  int v2;
  unsigned long v20; // stack - 0x450
  unsigned long v21; // stack - 0x448
  unsigned long v22; // stack - 0x488
  char v23 [1010];
  bool v24;
  unsigned char v25; // cl
  int v26;
  char *v27;
  int v28;
  unsigned int v29;
  long v3;
  int v30;
  unsigned int v31;
  char *v32;
  unsigned long v33; // r13
  char *v34;
  int v35;
  unsigned long v36;
  bool v37;
  int *v38; // stack - 0x4e8
  unsigned long v39; // stack - 0x4c0
  unsigned char v4;
  unsigned long v40; // stack - 0x4b8
  int v41; // stack - 0x4a8
  unsigned long v42; // stack - 0x480
  unsigned long v43; // stack - 0x478
  long v44; // stack - 0x470
  unsigned long v45; // stack - 0x468
  unsigned long v46; // stack - 0x460
  unsigned long v47; // stack - 0x458
  long *v5;
  char v6;
  int v7; // eax
  unsigned long v8;
  long *v9; // rax
  
  v5 = a7;
  v36 = (unsigned long)a6;
  v38 = __errno_location();
  v32 = *(char **)&a3->field_0x30;
  v1 = *(unsigned int *)&a3->field_0x8;
  v2 = *v38;
  if (!v32)
    v32 = "";
  if (0xd <= v1) // branch-flip
    v1 -= 0xc;
  else if (!v1)
    v1 = 0xc;
  v6 = *a2;
  v33 = 0;
  if (!v6) {
label_1330b:
    if ((a0) && (a1))
      *a0 = '\0';
    *v38 = v2;
    return v33;
  }
label_132ad:
  v19 = a2;
  if (v6 != '%') {
    v8 = 0;
    if (0 <= (int)v36)
      v8 = v36;
    v17 = 1;
    if (v8)
      v17 = v8;
    if ((unsigned long)(a1 - v33) <= v17) goto label_13330;
    if (a0) {
      v34 = a0;
      if (1 < (int)v36) {
        v34 = &a0[v8 - 1];
        memset(a0,0x20,v8 - 1);
      }
      a0 = &v34[1];
      *v34 = *a2;
    }
    v33 += v17;
    goto label_132f8;
  }
  v31 = 0;
  v16 = 0;
  v6 = a4;
  while( true ) {
    v34 = &v19[1];
    v14 = (unsigned int)*v34;
    v19 = &v19[1];
    v25 = *v34 - 0x23;
    if (0x3d <= v25) break;
    v8 = 1L << (v25 & 0x3f);
    if (v8 & 0x1000000000002500) // branch-flip
      v31 = v14;
    else if (v25 != 0x3b) { // branch-flip
      if (!(v8 & 1)) break;
      v16 = (unsigned int)v8 & 1;
    }
    else {
      v6 = '\x01';
    }
  }
  if (v14 - 0x30 <= 9) {
    v36 = 0;
    do {
      v11 = (long)(int)v36 * 10;
      v35 = (int)v11;
      if ((v35 != v11) || (v36 = (unsigned long)(unsigned int)(v35 + *v19 + -0x30), SCARRY4(v35,*v19 + -0x30)))
        v36 = 0x7fffffff;
      v14 = (unsigned int)v19[1];
      v19 = &v19[1];
    } while (v14 - 0x30 <= 9);
  }
  v8 = (unsigned long)v14;
  if (((char)v14 != 'E') && ((char)v14 != 'O'))
    v14 = 0;
  else {
    v8 = (unsigned long)(unsigned char)v19[1];
    v19 = &v19[1];
  }
  v25 = (unsigned char)v8;
  v35 = (int)v36;
  v34 = v19;
  if (0x7b <= v25) goto label_13450;
  v37 = (bool)v16;
  v16 = v1;
  switch(v8 & 0xff) {
    case 0:
      v34 = &v19[-1];
      break;
    case 0x25:
      v34 = &v19[-1];
      if (v34 == a2) {
        if ((v31 != 0x2d) && (0 <= v35)) {
          v8 = (unsigned long)v35;
          v36 = 1;
          if (v8)
            v36 = v8;
          if ((unsigned long)(a1 - v33) <= v36) goto label_13330;
          if (a0) {
            if (2 <= v35) {
              v8 -= 1;
              v34 = &a0[v8];
              if ((v31 != 0x30) && (v31 != 0x2b)) {
                memset(a0,0x20,v8);
                a0 = v34;
              }
              else {
                memset(a0,0x30,v8);
                a0 = v34;
              }
            }
label_14918:
            v34 = &a0[1];
            *a0 = *v19;
            a0 = v34;
          }
        }
        else {
          if ((unsigned long)(a1 - v33) <= 1) goto label_13330;
          v36 = 1;
          if (a0) goto label_14918;
        }
        goto label_13e38;
      }
      break;
    case 0x3a:
      v27 = &v19[1];
      v36 = 1;
      v13 = v19[1];
      while (v13 == ':') {
        v36 += 1;
        v27 = &v19[v36];
        v13 = v19[v36];
      }
      if (v13 == 'z') goto label_1448c;
      break;
    case 0x41:
    case 0x61:
      if (v14) break;
      if (v37)
        v6 = v37;
      v24 = 0;
      v10 = (unsigned char *)((long)&v20 + 5);
      goto label_135f2;
    case 0x42:
      if (v14 != 0x45) {
        if (v37)
          v6 = v37;
        v24 = 0;
        goto label_135d3;
      }
      break;
    case 0x43:
      if (v14 != 0x45) {
        v28 = *(int *)&a3->field_0x14;
        v40._0_1_ = v28 < -0x76c;
        v16 = (int)((-(unsigned int)(v28 + 0x76cU < 0x76c) & 0xffffff9d) + v28) / 100 + 0x13;
        v24 = -0x76c <= v28;
        if (!v31)
          v31 = a5;
        if (v31 != 0x2b) { // branch-flip
          v37 = 0;
          v41 = 2;
        }
        else {
          v41 = 2;
          v29 = 99;
label_1499f:
          if (v16 <= v29) { // branch-flip
label_1495c:
            v37 = v41 < v35;
            v24 = (bool)(v40._0_1_ ^ 1);
            v31 = 0x2b;
          }
          else {
            v37 = 1;
            v31 = 0x2b;
            v24 = (bool)(v40._0_1_ ^ 1);
          }
        }
label_13893:
        v29 = 0;
        goto label_138a0;
      }
      goto label_138b1;
    case 0x44:
      if (!v14) {
        v28 = -1;
        v34 = "%m/%d/%y";
        goto label_13eef;
      }
      break;
    case 0x46:
      if (v14) break;
      if ((0 <= v35) || (v31)) {
        v34 = "%Y-%m-%d";
        v28 = 0;
        if (0 <= v35 + -6)
          v28 = v35 + -6;
        goto label_13eef;
      }
      v36 = sub_13210(0,0xffffffffffffffff,"%Y-%m-%d",a3,(unsigned char)v6,0x2b,4,v5,a8,(long)*(int *)((v8 & 0xff) * 4 + 0x1e3cc) + 0x1e3cc);
      v34 = "%Y-%m-%d";
      v8 = 0;
      v31 = 0x2b;
      v39 = v36;
      v28 = 4;
      goto label_13f6d;
    case 0x47:
    case 0x56:
    case 0x67:
      if (v14 != 0x45) {
        v30 = *(int *)&a3->field_0x14;
        v26 = *(int *)&a3->field_0x1c;
        v15 = *(int *)&a3->field_0x18;
        v28 = (v26 - v15) + 0x17e;
        v16 = v30 + -100 + (v30 >> 0x1f & 400U);
        v28 = (v26 - v28) + 3 + (v28 / 7) * 7;
        if (0 <= v28) { // branch-flip
          v7 = 0x16d;
          if ((!(v16 & 3)) && (v7 = 0x16e, (v16 * -0x3d70a3d7 + 0x51eb850 >> 2 | v30 * 0x40000000) <= 0x28f5c28))
            v7 = 0x16e - (unsigned int)((int)v16 % 400 != 0);
          v15 = ((v26 - v7) - v15) + 0x17e;
          v26 = ((v26 - v7) - v15) + 3 + (v15 / 7) * 7;
          if (0 <= v26)
            v28 = v26;
          v26 = (v26 >> 0x1f) + 1;
        }
        else {
          v16 -= 1;
          v28 = 0x16d;
          if ((!(v16 & 3)) && (v28 = 0x16e, (v16 * -0x3d70a3d7 + 0x51eb850 >> 2 | v16 * 0x40000000) <= 0x28f5c28))
            v28 = 0x16e - (unsigned int)((int)v16 % 400 != 0);
          v15 = ((v28 + v26) - v15) + 0x17e;
          v28 = ((v28 + v26) - v15) + 3 + (v15 / 7) * 7;
          v26 = -1;
        }
        if (v25 == 0x47) {
          v16 = v30 + 0x76c + v26;
          v40._0_1_ = v30 < 0xfffff894U - v26;
          if (v31) { // branch-flip
label_14988:
            v41 = 4;
            v29 = 9999;
            if (v31 == 0x2b) goto label_1499f;
            v37 = 0;
            v24 = (bool)(v40._0_1_ ^ 1);
          }
          else {
            if (a5 == 0x2b) {
label_14c24:
              v41 = 4;
              v29 = 9999;
              goto label_1499f;
            }
            v37 = 0;
            v41 = 4;
            v24 = 0xfffff894U - v26 <= v30;
            v31 = a5;
          }
          goto label_13893;
        }
        if (v25 == 0x67) {
          v16 = (v30 % 100 + v26) % 100;
          if (0 <= (int)v16) { // branch-flip
            if (!v31)
              v31 = a5;
          }
          else {
            if (v30 < 0xfffff894U - v26) {
              v16 = -v16;
              goto label_143d2;
            }
            v16 += 100;
            if (!v31) goto label_143d8;
          }
          goto label_14949;
        }
        v41 = 2;
        v16 = v28 / 7 + 1;
        goto label_13ae0;
      }
      break;
    case 0x48:
      if (v14 != 0x45) {
        v41 = 2;
        v16 = *(unsigned int *)&a3->field_0x8;
        goto label_13ae0;
      }
      break;
    case 0x49:
      if (v14 != 0x45) {
        v41 = 2;
        goto label_13ae0;
      }
      break;
    case 0x4d:
      if (v14 != 0x45) {
        v41 = 2;
        v16 = *(unsigned int *)&a3->field_0x4;
        goto label_13ae0;
      }
      break;
    case 0x4e:
      if (v14 != 0x45) {
        v30 = 9;
        v28 = a8;
        if (v35 <= 0)
          v35 = 9;
        do {
          if (v30 <= v35) {
            if (v30 == 1) {
              v8 = 1;
              v36 = 1;
              goto label_13985;
            }
            if (v28 % 10) goto label_13977;
          }
          v30 -= 1;
          v28 /= 10;
        } while( true );
      }
      break;
    case 0x50:
      v24 = 1;
      goto label_14563;
    case 0x52:
      v28 = -1;
      v34 = "%H:%M";
      goto label_13eef;
    case 0x53:
      if (v14 != 0x45) {
        v41 = 2;
        v16 = *(unsigned int *)a3;
        goto label_13ae0;
      }
      break;
    case 0x54:
      v28 = -1;
      v34 = "%H:%M:%S";
label_13eef:
      v36 = sub_13210(0,0xffffffffffffffff,v34,a3,(unsigned char)v6,v31,v28,v5,a8);
      if ((v31 != 0x2d) && (0 <= v35)) {
        v8 = (unsigned long)v35;
        v39 = v8;
        if (v8 <= v36)
          v39 = v36;
      }
      else {
        v39 = v36;
        v8 = 0;
      }
label_13f6d:
      if ((unsigned long)(a1 - v33) <= v39) goto label_13330;
      if (a0) {
        v27 = a0;
        if (v36 < v8) {
          v8 -= v36;
          v27 = &a0[v8];
          if ((v31 != 0x30) && (v31 != 0x2b))
            memset(a0,0x20,v8);
          else {
            memset(a0,0x30,v8);
          }
        }
        a0 = &v27[v36];
        sub_13210(v27,a1 - v33,v34,a3,(unsigned char)v6,v31,v28,v5,a8);
      }
      v33 += v39;
      goto label_132f8;
    case 0x55:
      if (v14 != 0x45) {
        v28 = (*(int *)&a3->field_0x1c - *(int *)&a3->field_0x18) + 7;
label_13b93:
        v41 = 2;
        v16 = v28 / 7;
        goto label_13ae0;
      }
      break;
    case 0x57:
      if (v14 != 0x45) {
        v28 = *(int *)&a3->field_0x18 + 6;
        v28 = ((v28 / 7) * 7 - v28) + 7 + *(int *)&a3->field_0x1c;
        goto label_13b93;
      }
      break;
    case 0x58:
    case 99:
    case 0x78:
      if (v14 != 0x4f) goto label_135d1;
      break;
    case 0x59:
      if (v14 == 0x45) goto label_138b1;
      if (v14 == 0x4f) break;
      v40._0_1_ = *(int *)&a3->field_0x14 < -0x76c;
      v16 = *(int *)&a3->field_0x14 + 0x76c;
      if (v31) goto label_14988;
      if (a5 == 0x2b) goto label_14c24;
      v37 = 0;
      v29 = 0;
      v41 = 4;
      v31 = a5;
      goto label_14288;
    case 0x5a:
      if (v37)
        v6 = 0;
      v36 = strlen(v32);
      if ((v31 != 0x2d) && (0 <= v35)) {
        v8 = (unsigned long)v35;
        v40 = v8;
        if (v8 <= v36)
          v40 = v36;
      }
      else {
        v40 = v36;
        v8 = 0;
      }
      if ((unsigned long)(a1 - v33) <= v40) goto label_13330;
      if (a0) {
        v34 = a0;
        if (v36 < v8) {
          v8 -= v36;
          v34 = &a0[v8];
          if ((v31 != 0x30) && (v31 != 0x2b))
            memset(a0,0x20,v8);
          else {
            memset(a0,0x30,v8);
          }
        }
        if (v37) { // branch-flip
          if (v36) {
            v9 = __ctype_tolower_loc();
            v8 = v36;
            do {
              v8 -= 1;
              v34[v8] = (char)*(unsigned int *)(*v9 + (unsigned long)(unsigned char)v32[v8] * 4);
            } while (v8);
          }
        }
        else if (v6) { // branch-flip
          if (v36) {
            v9 = __ctype_toupper_loc();
            v8 = v36;
            do {
              v8 -= 1;
              v34[v8] = (char)*(unsigned int *)(*v9 + (unsigned long)(unsigned char)v32[v8] * 4);
            } while (v8);
          }
        }
        else {
          memcpy(v34,v32,v36);
        }
        a0 = &v34[v36];
      }
      v33 += v40;
      goto label_132f8;
    case 0x62:
    case 0x68:
      if (v37)
        v6 = v37;
      if (v14 != 0x45) goto label_135d1;
      break;
    case 100:
      if (v14 != 0x45) {
        v41 = 2;
        v16 = *(unsigned int *)&a3->field_0xc;
        goto label_13ae0;
      }
      break;
    case 0x65:
      if (v14 != 0x45) {
        v16 = *(unsigned int *)&a3->field_0xc;
label_14148:
        v41 = 2;
        if (!v31)
          v31 = 0x5f;
        goto label_13ae0;
      }
      break;
    case 0x6a:
      if (v14 != 0x45) {
        v37 = 0;
        v41 = 3;
        v28 = *(int *)&a3->field_0x1c;
        v16 = v28 + 1;
        v40._0_1_ = v28 < -1;
        v24 = -1 <= v28;
        v29 = 0;
        goto label_138a0;
      }
      break;
    case 0x6b:
      if (v14 != 0x45) {
        v16 = *(unsigned int *)&a3->field_0x8;
        goto label_14148;
      }
      break;
    case 0x6c:
      if (v14 != 0x45) goto label_14148;
      break;
    case 0x6d:
      if (v14 != 0x45) {
        v37 = 0;
        v41 = 2;
        v28 = *(int *)&a3->field_0x10;
        v16 = v28 + 1;
        v40._0_1_ = v28 < -1;
        v24 = -1 <= v28;
        v29 = 0;
        goto label_138a0;
      }
      break;
    case 0x6e:
      if ((v31 != 0x2d) && (0 <= v35)) {
        v8 = (unsigned long)v35;
        v36 = 1;
        if (v8)
          v36 = v8;
        if ((unsigned long)(a1 - v33) <= v36) goto label_13330;
        if (a0) {
          if (2 <= v35) {
            v8 -= 1;
            v34 = &a0[v8];
            if ((v31 != 0x30) && (v31 != 0x2b)) {
              memset(a0,0x20,v8);
              a0 = v34;
            }
            else {
              memset(a0,0x30,v8);
              a0 = v34;
            }
          }
label_14206:
          *a0 = '\n';
          a0 = &a0[1];
        }
      }
      else {
        if ((unsigned long)(a1 - v33) <= 1) goto label_13330;
        v36 = 1;
        if (a0) goto label_14206;
      }
      goto label_13e38;
    case 0x70:
      v24 = 0;
label_14563:
      v25 = 0x70;
      if (v37) {
        v6 = 0;
        v24 = v37;
      }
label_135d3:
      if (v14) goto label_138c0;
      v10 = (unsigned char *)((long)&v20 + 5);
      goto label_135f2;
    case 0x71:
      if (v14 == 0x4f) goto label_138b1;
      v37 = 0;
      v29 = 0;
      v41 = 1;
      v40._0_1_ = 0;
      v16 = (*(int *)&a3->field_0x10 * 0xb >> 5) + 1;
      goto label_140a0;
    case 0x72:
label_135d1:
      v24 = 0;
      goto label_135d3;
    case 0x73:
      v45 = *(unsigned long *)&a3->field_0x20;
      v46 = *(unsigned long *)&a3->field_0x28;
      v22 = *(unsigned long *)a3;
      v42 = *(unsigned long *)&a3->field_0x8;
      v43 = *(unsigned long *)&a3->field_0x10;
      v47 = *(unsigned long *)&a3->field_0x30;
      v44._0_4_ = (unsigned int)*(unsigned long *)&a3->field_0x18;
      v44 = CONCAT44(0xffffffff,(unsigned int)v44);
      v11 = sub_17d10(v5,&v22);
      if (v44 <= -1) {
        *v38 = 0x4b;
        return 0;
      }
      v40._0_1_ = (bool)((unsigned long)v11 >> 0x3f);
      v34 = &v23[1];
      v12 = v11;
      do {
        v3 = v12 / 10;
        v13 = (char)v12 + (char)v3 * '\xf6';
        if (v11 <= -1)
          v13 = -v13;
        v34 = &v34[-1];
        *v34 = v13 + '0';
        v12 = v3;
      } while (v3);
      v37 = 0;
      v41 = 1;
      goto label_13ca0;
    case 0x74:
      if ((v31 != 0x2d) && (0 <= v35)) {
        v8 = (unsigned long)v35;
        v36 = 1;
        if (v8)
          v36 = v8;
        if ((unsigned long)(a1 - v33) <= v36) goto label_13330;
        if (a0) {
          if (2 <= v35) {
            v8 -= 1;
            v34 = &a0[v8];
            if ((v31 != 0x30) && (v31 != 0x2b)) {
              memset(a0,0x20,v8);
              a0 = v34;
            }
            else {
              memset(a0,0x30,v8);
              a0 = v34;
            }
          }
label_13e30:
          *a0 = '\t';
          a0 = &a0[1];
        }
      }
      else {
        if ((unsigned long)(a1 - v33) <= 1) goto label_13330;
        v36 = 1;
        if (a0) goto label_13e30;
      }
label_13e38:
      v33 += v36;
      goto label_132f8;
    case 0x75:
      v41 = 1;
      v16 = (*(int *)&a3->field_0x18 + 6) % 7 + 1;
label_13ae0:
      v37 = 0;
      v29 = 0;
      v4 = (unsigned char)(v16 >> 0x18);
      v40._0_1_ = (bool)(v4 >> 7);
      v24 = (bool)((unsigned char)~v4 >> 7);
      goto label_138a0;
    case 0x77:
      if (v14 != 0x45) {
        v41 = 1;
        v16 = *(unsigned int *)&a3->field_0x18;
        goto label_13ae0;
      }
      break;
    case 0x79:
      if (v14 != 0x45) {
        v29 = *(int *)&a3->field_0x14 % 100;
        v16 = v29;
        if ((int)v29 <= -1) {
          v16 = v29 + 100;
          if (*(int *)&a3->field_0x14 <= -0x76d)
            v16 = -v29;
        }
label_143d2:
        if (v31) { // branch-flip
label_14949:
          if (v31 == 0x2b) {
label_1494f:
            v40._0_1_ = 0;
            v41 = 2;
            goto label_1495c;
          }
        }
        else {
label_143d8:
          if (a5 == 0x2b) goto label_1494f;
          v31 = a5;
        }
        v37 = 0;
        v24 = 1;
        v40._0_1_ = 0;
        v41 = 2;
        goto label_13893;
      }
label_138b1:
      v24 = 0;
label_138c0:
      v20._5_1_ = (char)v14;
      v10 = (unsigned char *)((long)&v20 + 6);
label_135f2:
      v20._3_2_ = 0x2520;
      *v10 = v25;
      v10[1] = 0;
      v36 = strftime((char *)&v21,0x400,(char *)((long)&v20 + 3),a3);
      if (v36) {
        v8 = v36 - 1;
        if ((v31 != 0x2d) && (0 <= v35)) {
          v18 = (unsigned long)v35;
          v17 = v18;
          if (v18 <= v8)
            v17 = v8;
        }
        else {
          v18 = 0;
          v17 = v8;
        }
        if ((unsigned long)(a1 - v33) <= v17) goto label_13330;
        if (a0) {
          v34 = a0;
          if (v8 < v18) {
            v18 -= v8;
            v34 = &a0[v18];
            if ((v31 != 0x30) && (v31 != 0x2b))
              memset(a0,0x20,v18);
            else {
              memset(a0,0x30,v18);
            }
          }
          if (v24) { // branch-flip
            v11 = v36 - 2;
            if (v8) {
              v9 = __ctype_tolower_loc();
              do {
                v34[v11] = (char)*(unsigned int *)(*v9 + (unsigned long)*(unsigned char *)((long)&v21 + v11 + 1) * 4);
                v37 = v11 != 0;
                v11 -= 1;
              } while (v37);
            }
          }
          else if (v6) { // branch-flip
            v11 = v36 - 2;
            if (v8) {
              v9 = __ctype_toupper_loc();
              do {
                v34[v11] = (char)*(unsigned int *)(*v9 + (unsigned long)*(unsigned char *)((long)&v21 + v11 + 1) * 4);
                v37 = v11 != 0;
                v11 -= 1;
              } while (v37);
            }
          }
          else {
            memcpy(v34,(void *)((long)&v21 + 1),v8);
          }
          a0 = &v34[v8];
        }
label_13707:
        v33 += v17;
      }
      goto label_132f8;
    case 0x7a:
      v36 = 0;
      v27 = v19;
label_1448c:
      v19 = v27;
      if (*(int *)&a3->field_0x20 < 0) goto label_132f8;
      v40._0_1_ = 1;
      v28 = (int)*(unsigned long *)&a3->field_0x28;
      if ((0 <= v28) && (v40._0_1_ = 0, !v28))
        v40._0_1_ = *v32 == '-';
      v16 = v28 / 0xe10;
      v30 = (v28 / 0x3c) % 0x3c;
      if (v36 != 2) { // branch-flip
        if (3 <= v36) { // branch-flip
          v34 = v27;
          if (v36 != 3) break;
          if (v28 % 0x3c) goto label_14b36;
          if (v30) goto label_14532;
          v37 = 1;
          v41 = 3;
          v24 = (bool)(v40._0_1_ ^ 1);
          v29 = 0;
        }
        else if (v36) { // branch-flip
label_14532:
          v37 = 1;
          v41 = 6;
          v24 = (bool)(v40._0_1_ ^ 1);
          v16 = v16 * 100 + v30;
          v29 = 4;
        }
        else {
          v37 = 1;
          v41 = 5;
          v24 = (bool)(v40._0_1_ ^ 1);
          v16 = v16 * 100 + v30;
          v29 = 0;
        }
      }
      else {
label_14b36:
        v37 = 1;
        v41 = 9;
        v24 = (bool)(v40._0_1_ ^ 1);
        v29 = 0x14;
        v16 = v16 * 10000 + v30 * 100 + v28 % 0x3c;
      }
label_138a0:
      if ((v14 == 0x4f) && (v24)) goto label_138b1;
label_14288:
      if (v40._0_1_)
        v16 = -v16;
label_140a0:
      v36 = (unsigned long)v16;
      v27 = &v23[1];
      while( true ) {
        if (v29 & 1) {
          v27[-1] = ':';
          v27 = &v27[-1];
        }
        v29 = (int)v29 >> 1;
        v34 = &v27[-1];
        v27[-1] = (char)v36 + (char)(v36 / 10) * '\xf6' + '0';
        if (((unsigned int)v36 <= 9) && (!v29)) break;
        v36 /= 10;
        v27 = v34;
      }
label_13ca0:
      v16 = v31;
      if (!v31)
        v16 = 0x30;
      if (v35 <= -1)
        v35 = v41;
      v28 = ((int)v23 + 1) - (int)v34;
      if (v40._0_1_) { // branch-flip
        v13 = '-';
label_145e5:
        v26 = v35 + -1;
        v30 = v26 - v28;
        if ((v30 <= 0) || (v31 == 0x2d))
          v30 = 0;
        if (v16 != 0x5f) { // branch-flip
          if ((unsigned long)(a1 - v33) <= 1) goto label_13330;
          if (a0) goto label_14616;
label_1461d:
          v33 += 1;
          v35 = v26;
          if (v16 != 0x2d) goto label_1462a;
label_1465e:
          v17 = (unsigned long)v28;
          v36 = 0;
        }
        else {
          v36 = (unsigned long)v30;
          v33 += v36;
          if (a0) {
            memset(a0,0x20,v36);
            a0 = &a0[v36];
            if ((unsigned long)(a1 - v33) <= 1) goto label_13330;
            v26 = (v35 - v30) + -1;
label_14616:
            *a0 = v13;
            a0 = &a0[1];
            goto label_1461d;
          }
          if ((unsigned long)(a1 - v33) <= 1) goto label_13330;
          v33 += 1;
          v35 = (v35 - v30) + -1;
label_1462a:
          v17 = (unsigned long)v28;
          v36 = 0;
          if (0 <= v35) goto label_13ce9;
        }
      }
      else {
        if (v37) {
          v13 = '+';
          goto label_145e5;
        }
        if (((v28 >= v35) || (v31 == 0x2d)) && (v16 == 0x2d)) goto label_1465e;
label_13ce9:
        v36 = (unsigned long)v35;
        v17 = (long)v28;
        if ((unsigned long)(long)v28 <= v36)
          v17 = v36;
      }
      v8 = (unsigned long)v28;
      if ((unsigned long)(a1 - v33) <= v17) goto label_13330;
      if (a0) {
        v27 = a0;
        if (v8 < v36) {
          v36 -= v8;
          v27 = &a0[v36];
          if ((v16 != 0x30) && (v16 != 0x2b))
            memset(a0,0x20,v36);
          else {
            memset(a0,0x30,v36);
          }
        }
        if (v6) { // branch-flip
          if (v8) {
            v9 = __ctype_toupper_loc();
            v36 = v8;
            do {
              v36 -= 1;
              v27[v36] = (char)*(unsigned int *)(*v9 + (unsigned long)(unsigned char)v34[v36] * 4);
            } while (v36);
          }
        }
        else {
          memcpy(v27,v34,v8);
        }
        a0 = &v27[v8];
      }
      goto label_13707;
    
  }
label_13450:
  v11 = (long)v34 - (long)a2;
  v36 = v11 + 1;
  if ((0 <= v35) && (v31 != 0x2d)) {
    v17 = (unsigned long)v35;
    v8 = v17;
    if (v17 <= v36)
      v8 = v36;
  }
  else {
    v17 = 0;
    v8 = v36;
  }
  if ((unsigned long)(a1 - v33) <= v8) goto label_13330;
  if (a0) {
    v19 = a0;
    if (v36 < v17) {
      v17 -= v36;
      v19 = &a0[v17];
      if ((v31 != 0x30) && (v31 != 0x2b))
        memset(a0,0x20,v17);
      else {
        memset(a0,0x30,v17);
      }
    }
    if (v6) { // branch-flip
      if (v36) {
        v9 = __ctype_toupper_loc();
        do {
          v19[v11] = (char)*(unsigned int *)(*v9 + (unsigned long)(unsigned char)a2[v11] * 4);
          v37 = v11 != 0;
          v11 -= 1;
        } while (v37);
      }
    }
    else {
      memcpy(v19,a2,v36);
    }
    a0 = &v19[v36];
  }
  v33 += v8;
  v19 = v34;
  goto label_132f8;
label_13977:
  v8 = (unsigned long)v30;
  v36 = v8;
  if (v30) { // branch-flip
label_13985:
    v34 = (char *)((long)&v21 + v36);
    do {
      v34 = &v34[-1];
      *v34 = (char)v28 + (char)(v28 / 10) * '\xf6' + '0';
      v28 /= 10;
    } while (v34 != (char *)((long)&v20 + (v36 - (unsigned int)(v30 - 1)) + 7));
  }
  else {
    v8 = 0;
  }
  if (!v31)
    v31 = 0x30;
  if (v8 >= (unsigned long)(a1 - v33)) {
label_13330:
    *v38 = 0x22;
    return 0;
  }
  if (a0) {
    if (v6) { // branch-flip
      if (v8) {
        v9 = __ctype_toupper_loc();
        v36 = v8;
        do {
          v17 = v36 - 1;
          a0[v17] = (char)*(unsigned int *)(*v9 + (unsigned long)*(unsigned char *)((long)&v20 + v36 + 7) * 4);
          v36 = v17;
        } while (v17);
      }
    }
    else if (8 <= v8) { // branch-flip
      *(unsigned long *)a0 = CONCAT44(v21._4_4_,(unsigned int)v21);
      *(unsigned long *)&a0[v8 - 8] = *(unsigned long *)((long)&v20 + v8);
      v11 = (long)a0 - ((unsigned long)&a0[8] & 0xfffffffffffffff8);
      v36 = v11 + v8 & 0xfffffffffffffff8;
      if (8 <= v36) {
        v17 = 0;
        do {
          *(unsigned long *)(((unsigned long)&a0[8] & 0xfffffffffffffff8) + v17) = *(unsigned long *)((long)&v21 + (v17 - v11));
          v17 += 8;
        } while (v17 < v36);
      }
    }
    else if (v8 & 4) { // branch-flip
      *(unsigned int *)a0 = (unsigned int)v21;
      *(unsigned int *)&a0[v8 - 4] = *(unsigned int *)((long)&v20 + v8 + 4);
    }
    else if (v8) {
      *a0 = (char)v21;
      if (v8 & 2)
        *(unsigned short *)&a0[v8 - 2] = *(unsigned short *)((long)&v20 + v8 + 6);
    }
    a0 = &a0[v8];
  }
  v33 += v8;
  if ((v31 != 0x2d) && (0 <= v35 - v30)) {
    v36 = (unsigned long)(v35 - v30);
    if ((unsigned long)(a1 - v33) <= v36) goto label_13330;
    if (a0) { // branch-flip
      if (v36) {
        v34 = &a0[v36];
        v33 += v36;
        if ((v31 != 0x30) && (v31 != 0x2b)) {
          memset(a0,0x20,v36);
          a0 = v34;
        }
        else {
          memset(a0,0x30,v36);
          a0 = v34;
        }
      }
    }
    else {
      v33 += v36;
    }
  }
  else if (a1 == v33) goto label_13330;
label_132f8:
  v6 = v19[1];
  a2 = &v19[1];
  v36 = 0xffffffffffffffff;
  if (!v6) goto label_1330b;
  goto label_132ad;
}

// Function: sub_14d80 @ 0x14d80
long sub_14d80(void)
{
  return sub_13210();
}

// Function: sub_14da0 @ 0x14da0
void sub_14da0(void)
{
  __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"memory exhausted",5));
  exit(dat_251f8); // no-return
}

// Function: sub_14df0 @ 0x14df0
long * sub_14df0(struct_15 *a0,long a1,long a2) // ternary
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
  a0->field_0x30 = v3;
  if (!a1)
    a1 = 0xfe0;
  a0->field_0x0 = a1;
  v1 = (a0->field_0x50 & 1) ? (long *)(*a0->field_0x38)(a0->field_0x48) : (long *)(*a0->field_0x38)(a1); // branch-flip
  a0->field_0x8 = v1;
  if (v1) {
    v2 = (long)v1 + v3 + 0x10 & -a2;
    a0->field_0x10 = v2;
    a0->field_0x18 = v2;
    v3 = a0->field_0x0 + (long)v1;
    *v1 = v3;
    a0->field_0x20 = v3;
    v1[1] = 0;
    v1 = (long *)0x1;
    a0->field_0x50 = a0->field_0x50 & 0xf9;
    return v1;
  }
  (*obstack_alloc_failed_handler)(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: _obstack_begin @ 0x14e90
int _obstack_begin(obstack *a0,unsigned long a1,unsigned long a2,void *a3,void *a4)
{
  a0->field_0x50 = a0->field_0x50 & 0xfe;
  *(void **)&a0->field_0x38 = a3;
  *(void **)&a0->field_0x40 = a4;
  return sub_14df0(a0,a1,a2); // tail-call
}

// Function: _obstack_begin_1 @ 0x14eb0
int _obstack_begin_1(obstack *a0,unsigned long a1,unsigned long a2,void *a3,void *a4,void *a5)
{
  a0->field_0x50 = a0->field_0x50 | 1;
  *(void **)&a0->field_0x38 = a3;
  *(void **)&a0->field_0x40 = a4;
  *(void **)&a0->field_0x48 = a5;
  return sub_14df0(a0,a1,a2); // tail-call
}

// Function: _obstack_newchunk @ 0x14ed0
void _obstack_newchunk(obstack *a0,unsigned long a1) // ternary
{
  unsigned long v1;
  unsigned char v2;
  long v3;
  void *v4;
  long *v5; // rax
  unsigned long v6;
  unsigned long v7; // r13
  void *v8; // r14
  
  v7 = *(long *)&a0->field_0x18 - *(long *)&a0->field_0x10;
  v3 = *(long *)&a0->field_0x8;
  v6 = a1 + v7 + *(long *)&a0->field_0x30;
  v1 = v6 + 100 + (v7 >> 3);
  if (v6 < *(unsigned long *)a0)
    v6 = *(unsigned long *)a0;
  if (v6 <= v1)
    v6 = v1;
  if ((!CARRY8(a1,v7)) && (!CARRY8(a1 + v7,*(unsigned long *)&a0->field_0x30))) {
    v5 = (a0->field_0x50 & 1) ? (long *)(**(void **)&a0->field_0x38)(*(unsigned long *)&a0->field_0x48) : (long *)(**(void **)&a0->field_0x38)(v6); // branch-flip
    if (v5) {
      *(long **)&a0->field_0x8 = v5;
      v4 = *(void **)&a0->field_0x10;
      v5[1] = v3;
      *(unsigned long *)&a0->field_0x20 = (long)v5 + v6;
      *v5 = (long)v5 + v6;
      v8 = (void *)((long)v5 + *(unsigned long *)&a0->field_0x30 + 0x10 & ~*(unsigned long *)&a0->field_0x30);
      memcpy(v8,v4,v7);
      v2 = a0->field_0x50;
      if ((!(v2 & 2)) && (*(unsigned long *)&a0->field_0x10 == (~*(unsigned long *)&a0->field_0x30 & v3 + 0x10 + *(unsigned long *)&a0->field_0x30))) {
        v5[1] = *(long *)(v3 + 8);
        if (v2 & 1) // branch-flip
          (**(void **)&a0->field_0x40)(*(unsigned long *)&a0->field_0x48,v3);
        else {
          (**(void **)&a0->field_0x40)(v3);
        }
      }
      *(void **)&a0->field_0x10 = v8;
      *(unsigned long *)&a0->field_0x18 = (long)v8 + v7;
      a0->field_0x50 = a0->field_0x50 & 0xfd;
      return;
    }
  }
  (*obstack_alloc_failed_handler)(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: _obstack_allocated_p @ 0x15000
unsigned long _obstack_allocated_p(void *a0,unsigned long *a1)
{
  unsigned long *v1;
  
  v1 = *(unsigned long **)((long)a0 + 8);
  if (!v1)
    return 0;
  while ((a1 <= v1 || ((unsigned long *)*v1 < a1))) {
    v1 = (unsigned long *)v1[1];
    if (!v1)
      return 0;
  }
  return 1;
}

// Function: _obstack_free @ 0x15040
void _obstack_free(obstack *a0,void *a1) // return-dupe
{
  void *v1;
  unsigned long *v2;
  unsigned long *v3;
  
  v3 = *(unsigned long **)&a0->field_0x8;
  if (*(unsigned long **)&a0->field_0x8) {
    while( true ) {
      if ((v3 < a1) && (v1 = (void *)*v3, a1 <= v1)) {
        *(void **)&a0->field_0x18 = a1;
        *(void **)&a0->field_0x10 = a1;
        *(void **)&a0->field_0x20 = v1;
        *(unsigned long **)&a0->field_0x8 = v3;
        return;
      }
      v2 = (unsigned long *)v3[1];
      if (a0->field_0x50 & 1) { // branch-flip
        (**(void **)&a0->field_0x40)(*(unsigned long *)&a0->field_0x48);
        a0->field_0x50 = a0->field_0x50 | 2;
      }
      else {
        (**(void **)&a0->field_0x40)(v3);
        a0->field_0x50 = a0->field_0x50 | 2;
      }
      if (!v2) break;
      v3 = v2;
    }
  }
  if (!a1)
    return;
  abort(); // no-return
}

// Function: _obstack_memory_used @ 0x150c0
unsigned long _obstack_memory_used(obstack *a0)
{
  long *v1;
  unsigned long v2;
  
  v2 = 0;
  for (v1 = *(long **)&a0->field_0x8; v1; v1 = (long *)v1[1]) {
    v2 += *v1 - (long)v1;
  }
  return v2;
}

// Function: sub_150f0 @ 0x150f0
void sub_150f0(char *a0)
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
    dat_26400 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_15190 @ 0x15190
char * sub_15190(char *a0,int a1) // return-dupe x3
{
  unsigned char *v1; // rax
  
  v1 = (unsigned char *)sub_19d00();
  if ((*v1 & 0xdf) != 0x55) { // branch-flip
    if (((((*v1 & 0xdf) == 0x47) && ((v1[1] & 0xdf) == 0x42)) && (v1[2] == 0x31)) && (((v1[3] == 0x38 && (v1[4] == 0x30)) && ((v1[5] == 0x33 && ((v1[6] == 0x30 && (!v1[7])))))))) {
      if (*a0 == '`')
        return (char *)0x1e61d;
      return (char *)0x1e618;
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

// Function: sub_15270 @ 0x15270
long * sub_15270(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
{
  unsigned long v1;
  long *v10; // rax
  char v11;
  unsigned char *v12;
  unsigned long v13;
  unsigned long v14;
  mbstate_t v15; // stack - 0x48
  unsigned int v16; // stack - 0x4c
  unsigned char v17;
  unsigned int v18;
  unsigned long v19;
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
  bool v5;
  long v6;
  long v7;
  unsigned char v8;
  unsigned int v9; // eax
  
  v29 = a6;
  v36 = a7;
  v35 = a8;
  v34 = a5;
label_152e0:
  v10 = (unsigned long)__ctype_get_mb_cur_max();
  v33 = (unsigned long)v10;
  v18 = v34 & 2;
  switch(a4) {
    case 0:
      v25 = 0;
      v2 = 0;
      v13 = 0;
      v30 = 0;
      v31 = NULL;
      break;
    case 1:
label_15786:
      v2 = 0;
      goto label_15740;
    case 2:
      if (!v18) {
        v2 = 0;
        goto label_158a1;
      }
      v25 = 1;
      v2 = 0;
      v13 = 0;
      v30 = 1;
      v31 = "\'";
      break;
    case 3:
      v2 = 1;
label_15740:
      v25 = 1;
      v13 = 0;
      a4 = 2;
      v30 = 1;
      v31 = "\'";
      break;
    case 4:
      if (!v18) {
        v2 = 1;
        goto label_158a1;
      }
      goto label_15786;
    case 5:
      if (!v18) goto label_156e1;
      v25 = 1;
      v2 = 1;
      v13 = 0;
      v30 = 1;
      v31 = "\"";
      break;
    case 6:
      v25 = 1;
      v13 = 0;
      v2 = 1;
      v30 = 1;
      a4 = 5;
      v31 = "\"";
      break;
    case 7:
      v25 = 0;
      v2 = 1;
      v13 = 0;
      v30 = 0;
      v31 = NULL;
      break;
    case 8:
    case 9:
    case 10:
      if (a4 != 10) {
        v10 = (char *)dcgettext(NULL,"`",5);
        v36 = (char *)v10;
        if (v10 == (long *)"`") {
          v10 = (char *)sub_15190("`",a4);
          v36 = (char *)v10;
        }
        v10 = (char *)dcgettext(NULL,"\'",5);
        v35 = (char *)v10;
        if (v10 == (long *)0x1ed68) {
          v10 = (char *)sub_15190("\'",a4);
          v35 = (char *)v10;
        }
      }
      v13 = 0;
      if (!v18) {
        v11 = *v36;
        while (v11) {
          if (v13 < a1)
            a0[v13] = v11;
          v13 += 1;
          v11 = v36[v13];
        }
      }
      v25 = v18 != 0;
      v2 = 1;
      v10 = (unsigned long)strlen(v35);
      v31 = v35;
      v30 = (unsigned long)v10;
      break;
    default:
      abort(); // no-return
    
  }
  v4 = 1;
  v20 = 0;
  v32 = 0;
  v14 = a1;
  v3 = 0;
label_153cb:
  do {
label_153de:
    v23 = 0;
    a1 = v14;
label_153e8:
    v14 = v32;
    v26 = a3 != v23;
    if (a3 == 0xffffffffffffffff)
      v26 = a2[v23] != '\0';
    if (v26) {
      v12 = (unsigned char *)&a2[v23];
      v22 = (bool)(a4 != 2 & v2);
      v26 = 1;
      v14 = v13;
      v27 = v2;
      v28 = v2;
      if (v22) {
        v24 = 1;
        if (v30) {
          if ((a3 == 0xffffffffffffffff) && (2 <= v30)) {
            v10 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v10;
          }
          if ((a3 < v23 + v30) || (v9 = memcmp(v12,v31,v30), v9)) goto label_15d50;
          if (!v25) {
            v8 = *v12;
            v19 = (unsigned long)v8;
            v21 = v19;
            if ((char)v8 <= '?') {
              switch(v8) {
                case 0:
                  goto label_1594c;
                default:
                  goto label_15623;
                case 7:
                  goto label_15937;
                case 8:
                  goto label_15906;
                case 9:
                  goto label_159d3;
                case 10:
                  goto label_1592d;
                case 0xb:
                  goto label_15a00;
                case 0xc:
                  goto label_158eb;
                case 0xd:
                  goto label_1583d;
                case 0x20:
                  goto label_15a0d;
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
                  goto label_155d6;
                case 0x23:
                  goto label_159b0;
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
                  goto label_15508;
                case 0x27:
                  goto label_15822;
                case 0x3f:
                  goto label_157e1;
                
              }
            }
            v5 = v22;
            if ((char)v8 <= 'z') {
              if (v8 == 0x40) goto label_15623;
              v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
              if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_15508;
              if (!((unsigned long)v10 & 0xa4000000)) goto label_15df4;
              goto label_15d38;
            }
            if (v8 == 0x7d) goto label_15ec0;
            if ((char)v8 <= '}') {
              v21 = 0x7b;
              if (v8 == 0x7b) goto label_15d1a;
              v21 = 0x7c;
              if (v8 == 0x7c) goto label_15d38;
              goto label_15623;
            }
            if (v8 != 0x7e) goto label_15623;
label_15e74:
            if (!v23) {
              v8 = a4 == 2;
              v21 = 0x7e;
              v26 = v3;
              goto label_15699;
            }
            v21 = 0x7e;
            goto label_15e82;
          }
          goto label_1560c;
        }
label_15d50:
        v8 = *v12;
        v19 = (unsigned long)v8;
        v21 = v19;
        if ('?' < (char)v8) {
          if ('z' < (char)v8) {
            if (v8 != 0x7d) { // branch-flip
              if ('~' <= (char)v8) {
                v22 = 0;
                if (v8 == 0x7e) goto label_15e74;
                goto label_15620;
              }
              v21 = 0x7b;
              v22 = 0;
              if (v8 != 0x7b) {
                v21 = 0x7c;
                v22 = 0;
                v5 = 0;
                if (v8 != 0x7c) goto label_15623;
                goto label_15d38;
              }
            }
            else {
              v21 = 0x7d;
              v22 = 0;
            }
label_15d1a:
            if (a3 == 0xffffffffffffffff) goto label_15ecf;
label_15d28:
            v5 = v22;
            if (a3 == 1) goto label_159b5;
label_15d38:
            v8 = a4 == 2;
            v26 = v3;
            v24 = 0;
            v22 = v5;
            goto label_15699;
          }
          if (v8 == 0x40) goto label_15620;
          v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
          v22 = 0;
          if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_15508;
          v22 = 0;
          v5 = 0;
          if ((unsigned long)v10 & 0xa4000000) goto label_15d38;
label_15df4:
          if ((char)v19 == '\\') {
            if (((bool)(v2 & v25)) && (v30)) goto label_15e1a;
            v17 = 0x5c;
            goto label_15842;
          }
label_15623:
          v17 = (unsigned char)v19;
          if (v33 != 1) goto label_15ba3;
label_15630:
          v10 = __ctype_b_loc();
          v21 = 1;
          v26 = (*(unsigned char *)(*v10 + 1 + v19 * 2) & 0x40) == 0;
          v27 = (bool)(v26 & v2);
          v26 = !v26;
          goto label_1567e;
        }
        switch(v8) {
          case 0:
            goto label_1593e;
          default:
label_15620:
            v22 = 0;
            goto label_15623;
          case 7:
label_15937:
            v17 = 0x61;
            goto label_15917;
          case 8:
label_15906:
            v17 = 0x62;
            goto label_15917;
          case 9:
            v22 = 0;
label_159d3:
            v21 = 9;
            v17 = 0x74;
            goto label_159e0;
          case 10:
label_1592d:
            v17 = 0x6e;
            break;
          case 0xb:
label_15a00:
            v17 = 0x76;
            break;
          case 0xc:
label_158eb:
            v17 = 0x66;
label_15917:
            if (v25) {
              v2 = 1;
              goto label_1586f;
            }
            v24 = 0;
            goto label_1558e;
          case 0xd:
label_1583d:
            v17 = 0x72;
            break;
          case 0x20:
            v22 = 0;
label_15a0d:
            v21 = 0x20;
            goto label_15508;
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
            v22 = 0;
            v26 = 0;
            goto label_155d9;
          case 0x23:
            v21 = 0x23;
            v22 = 0;
            goto label_159b5;
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
label_15508:
            v8 = 0;
            goto label_15518;
          case 0x27:
            v22 = 0;
            goto label_15822;
          case 0x3f:
            v22 = 0;
            goto label_157e1;
          
        }
        goto label_15842;
      }
      v17 = *v12;
      v19 = (unsigned long)v17;
      if ('?' < (char)v17) {
        if ('{' <= (char)v17) {
          if (v17 == 0x7d) {
label_15ec0:
            v21 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_15d28;
label_15ecf:
            v5 = v22;
            if (a2[1]) goto label_15d38;
            goto label_159b5;
          }
          if ((char)v17 <= '}') {
            v21 = 0x7b;
            if (v17 != 0x7b) {
              if (v17 == 0x7c) goto label_155d6;
              goto label_15623;
            }
            goto label_15d1a;
          }
          v21 = 0x7e;
          if (v17 == 0x7e) goto label_159b5;
          if (v33 == 1) goto label_15630;
label_15ba3:
          v15 = 0;
          if (a3 == 0xffffffffffffffff) {
            v10 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v10;
          }
          v21 = 0;
          do {
            v19 = v23 + v21;
            v10 = (long)sub_12780(&v16,&a2[v19],a3 - v19,&v15);
            v7 = (long)v10;
            v24 = v26;
            if (!v10) break;
            if (v10 == (long *)0xffffffffffffffff) {
              v24 = 0;
              goto label_1610b;
            }
            if (v10 == (long *)0xfffffffffffffffe) {
              v10 = (long *)v19;
              if (v19 >= a3) goto label_1648f;
              goto label_16485;
            }
            if (((a4 == 2) && (v25)) && (v10 != (long *)0x1)) {
              v12 = (unsigned char *)&a2[v19 + 1];
              v6 = v19 + (long)v10;
              do {
                v9 = (unsigned int)*v12;
                v9 -= 0x5b;
                v18 = v9;
                v10 = (unsigned long)(unsigned long)v9;
                v19 = (unsigned long)v10;
                v8 = (unsigned char)v18;
                if ((v8 <= 0x21) && (0x20000002bU >> (v19 & 0x3f) & 1)) goto label_15cdd;
                v12 = &v12[1];
              } while ((unsigned char *)&a2[v6] != v12);
            }
            v9 = iswprint(v16);
            if (!v9)
              v26 = 0;
            v21 += v7;
            v9 = mbsinit(&v15);
            v24 = v26;
          } while (!v9);
          v27 = (bool)((v24 ^ 1U) & v2);
          goto label_1610b;
        }
        if (v17 == 0x40) goto label_15623;
        v10 = (unsigned long)(1L << (v17 + 0xbf & 0x3f));
        if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_15690;
        if ((unsigned long)v10 & 0xa4000000) goto label_155d6;
        if (v17 != 0x5c) goto label_15623;
        if (a4 == 2) {
          if (!v25) goto label_15e1a;
          goto label_15603;
        }
        if (!(bool)(v2 & v25 & v30 != 0)) {
          v21 = 0x5c;
          v17 = 0x5c;
          v22 = 0;
          goto label_159e0;
        }
label_15e1a:
        v23 += 1;
        v24 = 0;
        v17 = 0x5c;
        v8 = v20;
        goto label_15e30;
      }
      switch(v17) {
        case 0:
          if (!v2) {
            v22 = 0;
            v21 = 0;
            if (v34 & 1) goto label_15ab2;
            goto label_159eb;
          }
label_1593e:
          if (!v25) {
            v22 = 0;
label_1594c:
            v27 = a4 == 2;
            v8 = v20 ^ 1;
            v8 &= v27;
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
                v24 = 0;
                v17 = 0x30;
                v20 = (bool)v8;
                goto label_156c3;
              }
label_16255:
              a0[v14] = 0x5c;
              v13 = v14;
              v20 = (bool)v8;
            }
            else if (v13 < a1) {
              v8 = v20;
              goto label_16255;
            }
            v14 = v13 + 1;
            if (a4 != 2) {
              v21 = 0x30;
              if ((v23 + 1 < a3) && (v8 = a2[v23 + 1] - 0x30, v8 <= 9)) {
                if (v14 < a1)
                  a0[v14] = 0x30;
                if (v13 + 2 < a1)
                  a0[v13 + 2] = 0x30;
                v14 = v13 + 3;
                v21 = 0x30;
              }
label_1628c:
              v17 = (unsigned char)v21;
              v24 = 0;
              v13 = v14;
              v8 = v26;
              if (!v2 || v27) goto label_156c3;
              goto label_15518;
            }
            v17 = 0x30;
            v24 = 0;
            v13 = v14;
            v8 = v26;
            goto label_156c3;
          }
          if (a4 != 2) goto label_1560c;
          goto label_1587b;
        default:
          goto label_15623;
        case 7:
          v21 = 7;
          v17 = 0x61;
          break;
        case 8:
          v21 = 8;
          v17 = 0x62;
          break;
        case 9:
          v21 = 9;
          v17 = 0x74;
          goto label_15a6f;
        case 10:
          v21 = 10;
          v17 = 0x6e;
          goto label_15a6f;
        case 0xb:
          v21 = 0xb;
          v17 = 0x76;
          break;
        case 0xc:
          v21 = 0xc;
          v17 = 0x66;
          break;
        case 0xd:
          v21 = 0xd;
          v17 = 0x72;
label_15a6f:
          v8 = a4 == 2 & v25;
          v22 = 0;
          if (!(bool)v8) goto label_159e0;
label_155f1:
          a4 = 2;
label_15603:
          if (!v28) goto label_1560c;
          goto label_1587b;
        case 0x20:
          v19 = 0x20;
          goto label_155d9;
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
label_155d6:
          v26 = 0;
label_155d9:
          v21 = v19;
          v24 = v26;
          if ((a4 == 2) && (v25)) goto label_155f1;
          goto label_15697;
        case 0x23:
label_159b0:
          v21 = 0x23;
label_159b5:
          if (!v23) {
            v19 = v21;
            goto label_155d9;
          }
label_15e82:
          v8 = a4 == 2;
          v26 = v3;
          v24 = 0;
          goto label_15699;
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
          goto label_15690;
        case 0x27:
label_15822:
          v24 = v26;
          if (a4 != 2) {
            v8 = 0;
            v21 = 0x27;
            goto label_15699;
          }
          if (!v25) {
            if (a1) { // branch-flip
              v14 = 0;
              v21 = a1;
              if (v32) goto label_162ab;
            }
            else {
label_162ab:
              if (v13 < a1)
                a0[v13] = 0x27;
              if (v13 + 1 < a1)
                a0[v13 + 1] = 0x5c;
              v14 = a1;
              v21 = v32;
              if (v13 + 2 < a1)
                a0[v13 + 2] = 0x27;
            }
            v13 += 3;
            v8 = 0;
            v17 = 0x27;
            a1 = v14;
            v32 = v21;
            v20 = 0;
            v3 = v26;
            goto label_156c3;
          }
          goto label_15603;
        case 0x3f:
label_157e1:
          if (a4 == 2) {
            if (v25) goto label_15603;
            v8 = 0;
            v17 = 0x3f;
            v24 = 0;
            goto label_156c3;
          }
          if ((((a4 == 5) && (v34 & 4)) && (v19 = v23 + 2, v19 < a3)) && (a2[v23 + 1] == '?')) {
            v8 = a2[v19];
            v21 = (unsigned long)v8;
            if (v8 <= 0x3e) {
              v10 = (unsigned long)(0x7000a38200000000 >> (v8 & 0x3f));
              v1 = (unsigned long)v10 & 1;
              v8 = (unsigned char)v10 & 1;
              if (!v1) {
                v21 = 0x3f;
                v26 = v3;
                v24 = 0;
                goto label_15699;
              }
              if (v25) goto label_1560c;
              if (v13 < a1)
                a0[v13] = 0x3f;
              if (v13 + 1 < a1)
                a0[v13 + 1] = 0x22;
              if (v13 + 2 < a1)
                a0[v13 + 2] = 0x22;
              if (v13 + 3 < a1)
                a0[v13 + 3] = 0x3f;
              v14 = v13 + 4;
              v27 = 0;
              v26 = 0;
              v23 = v19;
              goto label_1628c;
            }
          }
          v8 = 0;
          v21 = 0x3f;
          v26 = v3;
          v24 = 0;
          goto label_15699;
        
      }
label_159e0:
      if (!v2) {
label_159eb:
        v17 = (unsigned char)v21;
        v24 = 0;
        v8 = 0;
        v26 = v3;
        if (!v25) goto label_156c0;
        goto label_15518;
      }
label_15842:
      v24 = 0;
      goto label_15853;
    }
    if (!(bool)(v13 == 0 & v25 & a4 == 2)) {
      v8 = a4 == 2 & (v25 ^ 1U);
      v25 = (bool)(v25 ^ 1U);
      if ((!(bool)v8) || (v25 = (bool)v8, !v3)) {
label_1654b:
        v10 = (long *)v13;
        if (((v31) && (v25)) && (v11 = *v31, v11)) {
          do {
            if (v10 < a1)
              a0[(long)v10] = v11;
            v10 = (unsigned long)((long)v10 + 1);
            v11 = v31[(long)v10 - v13];
          } while (v11);
        }
        if (v10 < a1)
          a0[(long)v10] = 0;
        return v10;
      }
      if (!v4) {
        v8 = !a1 && v32;
        v25 = v3;
        if (a1 || !v32) goto label_1654b;
        v23 = v32;
        goto label_15fdb;
      }
      v13 = 0;
      a4 = 5;
      v10 = (unsigned long)__ctype_get_mb_cur_max();
      v30 = 1;
      v33 = (unsigned long)v10;
      v31 = "\"";
      if (!(v34 & 2)) goto label_165fb;
      v3 = 0;
      v20 = 0;
      v32 = 0;
      v2 = v4;
      v25 = v4;
      goto label_153de;
    }
label_15cdd:
    if (v2) {
label_1587b:
      v2 = 1;
      v34 &= 0xfffffffd;
      v10 = (unsigned long)__ctype_get_mb_cur_max();
      v29 = 0;
      v33 = (unsigned long)v10;
    }
    else {
      v2 = 0;
      v34 &= 0xfffffffd;
      v10 = (unsigned long)__ctype_get_mb_cur_max();
      v29 = 0;
      v33 = (unsigned long)v10;
    }
label_158a1:
    v25 = 0;
    a4 = 2;
    v13 = 1;
    v31 = "\'";
    v30 = 1;
    v20 = 0;
    v4 = 1;
    if (a1) { // branch-flip
      v8 = 0;
      v23 = 0;
      v32 = a1;
label_15fdb:
      v13 = 1;
      *a0 = 0x27;
      a4 = 2;
      v25 = 0;
      v30 = 1;
      v31 = "\'";
      v14 = v32;
      v32 = v23;
      v3 = (bool)v8;
    }
    else {
      v32 = 0;
      v14 = a1;
      v3 = 0;
    }
  } while( true );
  while( true ) {
    v21 += 1;
    v10 = (unsigned long)(v23 + v21);
    if (a3 <= v10) break;
label_16485:
    if (!a2[(long)v10]) break;
  }
label_1648f:
  v24 = 0;
label_1610b:
  v19 = (unsigned long)v17;
  v26 = v24;
  if (2 <= v21) {
label_16115:
    v26 = 0;
    v21 += v23;
    v14 = v23;
    do {
      v17 = (unsigned char)v19;
      if (v27) {
        v28 = a4 == 2;
        if (v25) goto label_15603;
        v8 = v28 & (v20 ^ 1U);
        if ((bool)v8) {
          if (v13 < a1)
            a0[v13] = 0x27;
          if (v13 + 1 < a1)
            a0[v13 + 1] = 0x24;
          if (v13 + 2 < a1)
            a0[v13 + 2] = 0x27;
          v13 += 3;
          v20 = 1;
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
        v23 = v14 + 1;
        v13 += 3;
        v17 = (v17 & 7) + 0x30;
        if (v21 <= v23) goto label_155a1;
        v26 = 1;
      }
      else {
        v8 = v26 ^ 1;
        v8 &= v20;
        if (v22) {
          if (v13 < a1)
            a0[v13] = 0x5c;
          v13 += 1;
        }
        v23 = v14 + 1;
        if (v21 <= v23) {
          goto label_15e30;
        }
        if ((bool)v8) {
          if (v13 < a1)
            a0[v13] = 0x27;
          if (v13 + 1 < a1)
            a0[v13 + 1] = 0x27;
          v13 += 2;
          v22 = 0;
          v20 = 0;
        }
        else {
          v22 = 0;
        }
      }
      v14 += 1;
      if (v13 < a1)
        a0[v13] = v17;
      v19 = (unsigned long)(unsigned char)a2[v14];
      v13 += 1;
    } while( true );
  }
label_1567e:
  if (v27) {
    v24 = 0;
    v27 = v2;
    goto label_16115;
  }
label_15690:
  v21 = v19;
  v24 = v26;
label_15697:
  v8 = a4 == 2;
  v26 = v3;
label_15699:
  v17 = (unsigned char)v21;
  v27 = (bool)v8 == 0;
  v3 = v26;
  v8 = 0;
  if ((v27 && v2) || (v8 = 0, v25)) {
label_15518:
    v17 = (unsigned char)v21;
    v13 = v14;
    if (!v29) goto label_156c3;
    if (!(*(unsigned int *)(v29 + (v21 >> 5) * 4) >> (v17 & 0x1f) & 1)) goto label_156c3;
  }
  else {
label_156c0:
    v8 = 0;
    v3 = v26;
label_156c3:
    if (!v22) {
      v8 ^= 1;
      v23 += 1;
      v8 &= v20;
label_15e30:
      if ((bool)v8) {
        if (v13 < a1)
          a0[v13] = 0x27;
        if (v13 + 1 < a1)
          a0[v13 + 1] = 0x27;
        v20 = 0;
        v13 += 2;
      }
      goto label_155a1;
    }
  }
label_15853:
  if (v25) {
label_1586f:
    if ((bool)(v2 & a4 == 2)) goto label_1587b;
label_1560c:
    v34 &= 0xfffffffd;
    v29 = 0;
    goto label_152e0;
  }
  v8 = v20 ^ 1;
  v8 &= a4 == 2;
  if ((bool)v8) {
    if (v13 < a1)
      a0[v13] = 0x27;
    if (v13 + 1 < a1)
      a0[v13 + 1] = 0x24;
    if (v13 + 2 < a1)
      a0[v13 + 2] = 0x27;
    v13 += 3;
    v20 = 1;
  }
label_1558e:
  if (v13 < a1)
    a0[v13] = 0x5c;
  v13 += 1;
  v23 += 1;
label_155a1:
  if (v13 < a1)
    a0[v13] = v17;
  v13 += 1;
  if (!v24)
    v4 = 0;
  goto label_153e8;
label_15ab2:
  v23 += 1;
  goto label_153e8;
label_165fb:
  a1 = v32;
label_156e1:
  v25 = 0;
  a4 = 5;
  v13 = 1;
  v31 = "\"";
  v2 = 1;
  v30 = 1;
  v20 = 0;
  v3 = 0;
  v4 = 1;
  v32 = 0;
  v14 = a1;
  if (a1)
    *a0 = 0x22;
  goto label_153cb;
}

// Function: sub_16700 @ 0x16700
char * sub_16700(unsigned int a0,char *a1,unsigned long a2,struct_3 *a3)
{
  unsigned long *v1;
  long v10; // stack - 0x48
  int *v11; // stack - 0x68
  int v2;
  unsigned int v3;
  unsigned long v4;
  unsigned long *v5;
  unsigned long v6;
  char *v7;
  long v8; // rdx
  unsigned int v9; // ebx
  
  v11 = __errno_location();
  v2 = *v11;
  if (a0 <= 0x7ffffffe) {
    v5 = dat_25260;
    if (dat_25258 <= (int)a0) {
      v10 = (long)dat_25258;
      v8 = (long)(int)((a0 - dat_25258) + 1);
      if (dat_25260 != (unsigned long *)0x25270) // branch-flip
        v5 = (unsigned long *)sub_18870(dat_25260,&v10,v8,0x7fffffff,0x10);
      else {
        v5 = (unsigned long *)sub_18870(NULL,&v10,v8,0x7fffffff,0x10);
        *v5 = dat_25270;
        v5[1] = dat_25278;
      }
      dat_25260 = v5;
      memset(&v5[(long)dat_25258 * 2],0,(v10 - dat_25258) * 0x10);
      dat_25258 = (int)v10;
    }
    v1 = &v5[(long)(int)a0 * 2];
    v4 = *v1;
    v7 = (char *)v1[1];
    v9 = a3->field_0x4 | 1;
    v6 = sub_15270(v7,v4,a1,a2,a3->field_0x0,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v4 <= v6) {
      v6 += 1;
      *v1 = v6;
      if (v7 != (char *)0x26420)
        free(v7);
      v7 = (char *)sub_185d0(v6);
      v3 = a3->field_0x0;
      v1[1] = (unsigned long)v7;
      sub_15270(v7,v6,a1,a2,v3,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *v11 = v2;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_168f0 @ 0x168f0
void sub_168f0(void *a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = (void *)0x26520;
  sub_189d0(a0,0x38);
  *v2 = v1;
}

// Function: sub_16930 @ 0x16930
unsigned int sub_16930(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x26520;
  return *a0;
}

// Function: sub_16950 @ 0x16950
void sub_16950(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x26520;
  *a0 = a1;
}

// Function: sub_16970 @ 0x16970
unsigned int sub_16970(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0x26520;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_169b0 @ 0x169b0
unsigned int sub_169b0(void *a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = (void *)0x26520;
  v1 = *(unsigned int *)((long)a0 + 4);
  *(unsigned int *)((long)a0 + 4) = a1;
  return v1;
}

// Function: sub_169d0 @ 0x169d0
void sub_169d0(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0x26520;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_16a10 @ 0x16a10
void sub_16a10(char *a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x26520;
  v2 = __errno_location();
  v1 = *v2;
  sub_15270(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_16a90 @ 0x16a90
char * sub_16a90(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  char *v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x26520;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_15270(NULL,0,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = (char *)sub_185d0(v3 + 1U);
  sub_15270(v4,v3 + 1U,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_16b80 @ 0x16b80
char * sub_16b80(char *a0,unsigned long a1,unsigned int *a2)
{
  return sub_16a90(a0,a1,NULL,a2); // tail-call
}

// Function: sub_16b90 @ 0x16b90
void sub_16b90(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_25260;
  if (2 <= dat_25258) {
    v4 = (unsigned long *)((long)dat_25260 + 0x18);
    v1 = (unsigned long *)((long)dat_25260 + (unsigned long)(unsigned int)(dat_25258 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0x26420) {
    free(*(void **)((long)v3 + 8));
    dat_25278 = 0x26420;
    dat_25270 = 0x100;
  }
  if (v3 == (void *)0x25270) {
    dat_25258 = 1;
    return;
  }
  free(v3);
  dat_25258 = 1;
  dat_25260 = (void *)0x25270;
}

// Function: sub_16c30 @ 0x16c30
char * sub_16c30(unsigned int a0,char *a1)
{
  return sub_16700(a0,a1,0xffffffffffffffff,(struct_3 *)0x26520); // tail-call
}

// Function: sub_16c50 @ 0x16c50
char * sub_16c50(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_16700(a0,a1,a2); // tail-call
}

// Function: sub_16c60 @ 0x16c60
char * sub_16c60(char *a0)
{
  return sub_16700(0,a0,0xffffffffffffffff,(struct_3 *)0x26520); // tail-call
}

// Function: sub_16c80 @ 0x16c80
char * sub_16c80(char *a0,unsigned long a1)
{
  return sub_16700(0,a0,a1,(struct_3 *)0x26520); // tail-call
}

// Function: sub_16ca0 @ 0x16ca0
void sub_16ca0(unsigned int a0,int a1,char *a2)
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
    sub_16700(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_16d40 @ 0x16d40
void sub_16d40(unsigned int a0,int a1,char *a2,unsigned long a3)
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
    sub_16700(a0,a2,a3,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_16dd0 @ 0x16dd0
void sub_16dd0(int a0,char *a1)
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
    sub_16700(0,a1,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_16e60 @ 0x16e60
void sub_16e60(int a0,char *a1,unsigned long a2)
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
    sub_16700(0,a1,a2,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_16ef0 @ 0x16ef0
void sub_16ef0(char *a0,unsigned long a1,unsigned char a2)
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
  
  v3 = dat_26520;
  v4 = dat_26528;
  v9 = dat_26550;
  v5 = dat_26530;
  v6 = dat_26538;
  v7 = dat_26540;
  v8 = dat_26548;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a2 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a2 & 0x1f)) & 1) << (a2 & 0x1f) ^ v2;
  sub_16700(0,a0,a1,&v3);
}

// Function: sub_16f90 @ 0x16f90
void sub_16f90(char *a0,unsigned char a1)
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
  
  v3 = dat_26520;
  v4 = dat_26528;
  v9 = dat_26550;
  v5 = dat_26530;
  v6 = dat_26538;
  v7 = dat_26540;
  v8 = dat_26548;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a1 & 0x1f)) & 1) << (a1 & 0x1f) ^ v2;
  sub_16700(0,a0,0xffffffffffffffff,&v3);
}

// Function: sub_17030 @ 0x17030
void sub_17030(char *a0)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_26520;
  v3 = (unsigned int)((unsigned long)dat_26528 >> 0x20);
  v8 = dat_26550;
  v4 = dat_26530;
  v5 = dat_26538;
  v6 = dat_26540;
  v7 = dat_26548;
  v2 = (unsigned int)dat_26528;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_16700(0,a0,0xffffffffffffffff,&v1);
}

// Function: sub_170c0 @ 0x170c0
void sub_170c0(char *a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_26520;
  v3 = (unsigned int)((unsigned long)dat_26528 >> 0x20);
  v8 = dat_26550;
  v4 = dat_26530;
  v5 = dat_26538;
  v6 = dat_26540;
  v7 = dat_26548;
  v2 = (unsigned int)dat_26528;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_16700(0,a0,a1,&v1);
}

// Function: sub_17150 @ 0x17150
void sub_17150(unsigned int a0,int a1,char *a2)
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
    sub_16700(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_171f0 @ 0x171f0
void sub_171f0(unsigned int a0,long a1,long a2,char *a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_26528;
  v4 = dat_26530;
  v5 = dat_26538;
  v8 = dat_26550;
  v2 = (unsigned int)((unsigned long)dat_26520 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_26540;
  v7 = dat_26548;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_16700(a0,a3,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_17290 @ 0x17290
void sub_17290(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_26528;
  v4 = dat_26530;
  v5 = dat_26538;
  v8 = dat_26550;
  v2 = (unsigned int)((unsigned long)dat_26520 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_26540;
  v7 = dat_26548;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_16700(a0,a3,a4,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_17330 @ 0x17330
void sub_17330(long a0,long a1,char *a2)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_26528;
  v4 = dat_26530;
  v5 = dat_26538;
  v8 = dat_26550;
  v2 = (unsigned int)((unsigned long)dat_26520 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_26540;
  v7 = dat_26548;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_16700(0,a2,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_173d0 @ 0x173d0
void sub_173d0(long a0,long a1,char *a2,unsigned long a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_26528;
  v4 = dat_26530;
  v5 = dat_26538;
  v8 = dat_26550;
  v2 = (unsigned int)((unsigned long)dat_26520 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_26540;
  v7 = dat_26548;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_16700(0,a2,a3,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_17470 @ 0x17470
char * sub_17470(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_16700(a0,a1,a2); // tail-call
}

// Function: sub_17480 @ 0x17480
char * sub_17480(char *a0,unsigned long a1)
{
  return sub_16700(0,a0,a1,(struct_3 *)0x25220); // tail-call
}

// Function: sub_174a0 @ 0x174a0
char * sub_174a0(unsigned int a0,char *a1)
{
  return sub_16700(a0,a1,0xffffffffffffffff,(struct_3 *)0x25220); // tail-call
}

// Function: sub_174c0 @ 0x174c0
char * sub_174c0(char *a0)
{
  return sub_16700(0,a0,0xffffffffffffffff,(struct_3 *)0x25220); // tail-call
}

// Function: sub_174e0 @ 0x174e0
char ** sub_174e0(char **a0,unsigned long a1) // early-return
{
  char **v1;
  char **v2;
  void *v3; // rax
  
  v1 = (char **)*a0;
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

// Function: sub_17540 @ 0x17540
int * sub_17540(struct_13 *a0)
{
  unsigned long v1;
  int *v2; // rax
  
  v2 = (unsigned long)a0->field_0x8;
  v1 = (long)v2 * 2;
  if (a0->field_0x0 != &a0[1]) {
    free(a0->field_0x0);
    v2 = (unsigned long)a0->field_0x8;
  }
  if (v2 <= v1) { // branch-flip
    v2 = malloc(v1);
    if (v2) {
      a0->field_0x0 = v2;
      a0->field_0x8 = v1;
      v2 = (int *)0x1;
      return v2;
    }
  }
  else {
    v2 = __errno_location();
    *v2 = 0xc;
  }
  a0->field_0x0 = &a0[1];
  v2 = NULL;
  a0->field_0x8 = 0x400;
  return v2;
}

// Function: sub_175c0 @ 0x175c0
unsigned long sub_175c0(struct_13 *a0) // return-dupe
{
  unsigned long v1;
  void *v2; // rax
  struct_13 *v3;
  
  v3 = a0->field_0x0;
  v1 = a0->field_0x8 * 2;
  if (v3 != &a0[1]) {
    if ((unsigned long)a0->field_0x8 <= v1) { // branch-flip
      v2 = realloc(v3,v1);
      if (v2) goto label_175fd;
      v3 = a0->field_0x0;
    }
    else {
      *__errno_location() = 0xc;
    }
    free(v3);
    a0->field_0x0 = &a0[1];
    a0->field_0x8 = 0x400;
    return 0;
  }
  v2 = malloc(v1);
  if (!v2)
    return 0;
  v2 = memcpy(v2,v3,a0->field_0x8);
label_175fd:
  a0->field_0x0 = v2;
  a0->field_0x8 = v1;
  return 1;
}

// Function: sub_17680 @ 0x17680
int sub_17680(char *a0,char **a1)
{
  char *v1;
  int v2; // eax
  
  v2 = getfilecon(a0,a1);
  if (v2) { // branch-flip
    if (v2 == 10) {
      v1 = *a1;
      if (!strcmp(v1,"unlabeled")) {
        v2 = -1;
        freecon(v1);
        *a1 = NULL;
        *__errno_location() = 0x3d;
      }
    }
  }
  else {
    v2 = -1;
    *__errno_location() = 0x5f;
  }
  return v2;
}

// Function: sub_17700 @ 0x17700
int sub_17700(char *a0,char **a1)
{
  char *v1;
  int v2; // eax
  
  v2 = lgetfilecon(a0,a1);
  if (v2) { // branch-flip
    if (v2 == 10) {
      v1 = *a1;
      if (!strcmp(v1,"unlabeled")) {
        v2 = -1;
        freecon(v1);
        *a1 = NULL;
        *__errno_location() = 0x3d;
      }
    }
  }
  else {
    v2 = -1;
    *__errno_location() = 0x5f;
  }
  return v2;
}

// Function: sub_17780 @ 0x17780
int sub_17780(int a0,char **a1)
{
  char *v1;
  int v2; // eax
  
  v2 = fgetfilecon(a0,a1);
  if (v2) { // branch-flip
    if (v2 == 10) {
      v1 = *a1;
      if (!strcmp(v1,"unlabeled")) {
        v2 = -1;
        freecon(v1);
        *a1 = NULL;
        *__errno_location() = 0x3d;
      }
    }
  }
  else {
    v2 = -1;
    *__errno_location() = 0x5f;
  }
  return v2;
}

// Function: sub_17800 @ 0x17800
unsigned long sub_17800(int a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = setlocale(a0,NULL);
  if (!v1) {
    if (!a2)
      return 0x16;
    *a1 = '\0';
    return 0x16;
  }
  v2 = strlen(v1);
  if (a2 <= v2) {
    if (!a2)
      return 0x22;
    memcpy(a1,v1,a2 - 1);
    a1[a2 - 1] = '\0';
    return 0x22;
  }
  memcpy(a1,v1,v2 + 1);
  return 0;
}

// Function: sub_178b0 @ 0x178b0
void sub_178b0(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_178c0 @ 0x178c0
unsigned long sub_178c0(long *a0)
{
  long *v1;
  int v2; // eax
  int *v3; // rax
  int v4;
  unsigned long v5; // r13
  
  v3 = __errno_location();
  v4 = *v3;
  if ((char)a0[1]) // branch-flip
    v2 = setenv("TZ",(char *)((long)a0 + 9),1);
  else {
    v2 = unsetenv("TZ");
  }
  if (v2) { // branch-flip
    v4 = *v3;
    v5 = 0;
  }
  else {
    tzset();
    v5 = 1;
  }
  do {
    v1 = (long *)*a0;
    free(a0);
    a0 = v1;
  } while (v1);
  *v3 = v4;
  return v5;
}

// Function: sub_17950 @ 0x17950
unsigned long * sub_17950(char *a0)
{
  unsigned long v1;
  unsigned long v2; // rax
  unsigned long v3;
  unsigned long *v4;
  unsigned long *v5; // rax
  
  if (a0) { // branch-flip
    v2 = strlen(a0);
    v1 = v2 + 1;
    v3 = 0x76;
    if (0x76 <= v1)
      v3 = v1;
    v4 = malloc(v3 + 0x11 & 0xfffffffffffffff8);
    if (v4) {
      *v4 = 0;
      *(unsigned short *)&v4[1] = 1;
      memcpy((void *)((long)v4 + 9),a0,v1);
      *(char *)((long)v4 + v2 + 10) = 0;
    }
  }
  else {
    v5 = malloc(0x80);
    v4 = NULL;
    if (v5) {
      *v5 = 0;
      *(unsigned short *)&v5[1] = 0;
      return v5;
    }
  }
  return v4;
}

// Function: sub_179f0 @ 0x179f0
unsigned long sub_179f0(long *a0,struct_29 *a1) // return-dupe
{
  struct_29 *v1;
  long *v2;
  int v3; // eax
  unsigned long v4; // rax
  long v5; // rax
  char *v6;
  
  v1 = (struct_29 *)a1->field_0x30;
  if (!v1)
    return 1;
  if ((a1 <= v1) && (v1 < &a1[1]))
    return 1;
  v6 = (char *)((long)a0 + 9);
  if (!v1->field_0x0[0]) {
    v6 = "";
label_17a88:
    a1->field_0x30 = v6;
    return 1;
  }
  do {
    v3 = strcmp(v6,(char *)v1);
    while( true ) {
      if (!v3) goto label_17a88;
      if ((!*v6) && ((v6 != (char *)((long)a0 + 9) || (!(char)a0[1])))) {
        v4 = strlen((char *)v1);
        if ((long)a0 + (0x80U - (long)v6) <= (long)(v4 + 1)) { // branch-flip
          v5 = sub_17950(v1);
          *a0 = v5;
          if (!v5)
            return 0;
          *(char *)(v5 + 8) = 0;
          v6 = (char *)(v5 + 9);
        }
        else {
          memcpy(v6,v1,v4 + 1);
          v6[v4 + 1] = '\0';
        }
        goto label_17a88;
      }
      v6 = &v6[strlen(v6) + 1];
      if ((*v6) || (v2 = (long *)*a0, !v2)) break;
      v6 = (char *)((long)v2 + 9);
      v3 = strcmp(v6,(char *)v1);
      a0 = v2;
    }
  } while( true );
}

// Function: sub_17b20 @ 0x17b20
long * sub_17b20(void *a0)
{
  long *v1;
  int v2;
  char *v3; // rax
  long *v4;
  int *v5; // rax
  
  v3 = getenv("TZ");
  if (v3) { // branch-flip
    if ((*(char *)((long)a0 + 8)) && (!strcmp((char *)((long)a0 + 9),v3)))
      return (long *)0x1;
  }
  else if (!*(char *)((long)a0 + 8))
    return (long *)0x1;
  v4 = (long *)sub_17950(v3);
  if (v4) {
    if (*(char *)((long)a0 + 8)) // branch-flip
      v2 = setenv("TZ",(char *)((long)a0 + 9),1);
    else {
      v2 = unsetenv("TZ");
    }
    if (v2) {
      v5 = __errno_location();
      v2 = *v5;
      if (v4 != (long *)0x1) {
        do {
          v1 = (long *)*v4;
          free(v4);
          v4 = v1;
        } while (v1);
      }
      *v5 = v2;
      return NULL;
    }
    tzset();
  }
  return v4;
}

// Function: sub_17c20 @ 0x17c20
void sub_17c20(long *a0) // return-dupe
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

// Function: sub_17c60 @ 0x17c60
struct_29 * sub_17c60(long *a0,void *a1,struct_29 *a2) // return-dupe x3
{
  long *v1; // rax
  
  if (!a0)
    return (struct_29 *)gmtime_r(a1,(tm *)a2); // tail-call
  v1 = (long *)sub_17b20(a0);
  if (!v1)
    return NULL;
  if ((!localtime_r(a1,(tm *)a2)) || (!sub_179f0(a0,a2))) {
    if (v1 == (long *)0x1)
      return NULL;
    sub_178c0(v1);
    return NULL;
  }
  if ((v1 != (long *)0x1) && (!sub_178c0(v1)))
    return NULL;
  return a2;
}

// Function: sub_17d10 @ 0x17d10
unsigned long sub_17d10(long *a0,unsigned long *a1) // return-dupe x3
{
  long *v1; // rax
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
    return sub_17e60(a1); // tail-call
  v1 = (long *)sub_17b20(a0);
  if (!v1)
    return 0xffffffffffffffff;
  v3 = *a1;
  v7 = -1;
  v4 = a1[1];
  v5 = a1[2];
  v8 = *(unsigned int *)&a1[4];
  v2 = sub_19ac0(&v3);
  if ((v7 <= -1) || (!sub_179f0(a0,&v3))) {
    if (v1 == (long *)0x1)
      return 0xffffffffffffffff;
    sub_178c0(v1);
    return 0xffffffffffffffff;
  }
  if ((v1 != (long *)0x1) && (!sub_178c0(v1)))
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

// Function: sub_17e60 @ 0x17e60
unsigned long sub_17e60(struct_30 *a0)
{
  unsigned long v1; // rax
  
  a0->field_0x20 = 0;
  v1 = sub_19510(a0,dat_24fd0,(long *)0x26558); // tail-call
  return v1;
}

// Function: sub_17e80 @ 0x17e80
long sub_17e80(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5)
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
label_17fad:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_17fe6:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0x1ece8) + 0x1ece8;
  switch(a5) {
    case 0:
      goto label_17fe6;
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
      goto label_17fad;
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

// Function: sub_18300 @ 0x18300
void sub_18300(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_17e80(); // tail-call
}

// Function: sub_18320 @ 0x18320
void sub_18320(FILE *a0,char *a1,char *a2,char *a3,struct_35 *a4)
{
  unsigned int v1;
  long *v2;
  long v3;
  long v4 [11];
  long v5; // r9
  
  v5 = 0;
  do {
    v1 = a4->field_0x0;
    if (0x30 <= v1) { // branch-flip
      v2 = a4->field_0x8;
      a4->field_0x8 = &v2[1];
      v3 = *v2;
      v4[v5] = v3;
    }
    else {
      a4->field_0x0 = v1 + 8;
      v3 = *(long *)((unsigned long)v1 + a4->field_0x10);
      v4[v5] = v3;
    }
  } while ((v3) && (v5 = v5 + 1, v5 != 10));
  sub_17e80(a0,a1,a2,a3);
}

// Function: sub_183c0 @ 0x183c0
void sub_183c0(FILE *a0,char *a1,char *a2,unsigned long a3,unsigned long a4,unsigned long a5)
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
  sub_17e80(a0,a1,a2);
}

// Function: sub_184a0 @ 0x184a0
void sub_184a0(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-coreutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_18550 @ 0x18550
void sub_18550(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_18a90(); // no-return
}

// Function: sub_18590 @ 0x18590
void sub_18590(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_18a90(); // no-return
}

// Function: sub_185b0 @ 0x185b0
void sub_185b0(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_18a90(); // no-return
}

// Function: sub_185d0 @ 0x185d0
void sub_185d0(void)
{
  sub_18590(); // tail-call
}

// Function: sub_185e0 @ 0x185e0
void sub_185e0(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_18a90(); // no-return
}

// Function: sub_18620 @ 0x18620
void sub_18620(void *a0,unsigned long a1)
{
  if (realloc(a0,a1 | a1 == 0))
    return;
  sub_18a90(); // no-return
}

// Function: sub_18650 @ 0x18650
void sub_18650(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_18a90(); // no-return
}

// Function: sub_18690 @ 0x18690
void sub_18690(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!a1) || (!a2)) {
    a1 = 1;
    a2 = 1;
  }
  if (reallocarray(a0,a1,a2))
    return;
  sub_18a90(); // no-return
}

// Function: sub_186d0 @ 0x186d0
void sub_186d0(unsigned long a0,unsigned long a1)
{
  if (reallocarray(NULL,a0,a1))
    return;
  sub_18a90(); // no-return
}

// Function: sub_18700 @ 0x18700
void sub_18700(unsigned long a0,unsigned long a1)
{
  if ((!a0) || (!a1)) {
    a0 = 1;
    a1 = 1;
  }
  if (!reallocarray(NULL,a0,a1))
    sub_18a90(); // no-return
}

// Function: sub_18750 @ 0x18750
void sub_18750(void *a0,unsigned long *a1)
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
  sub_18a90(); // no-return
}

// Function: sub_187e0 @ 0x187e0
void sub_187e0(void *a0,unsigned long *a1,unsigned long a2)
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
  sub_18a90(); // no-return
}

// Function: sub_18870 @ 0x18870
void sub_18870(void *a0,long *a1,long a2,long a3,long a4)
{
  long v1;
  long v2;
  long v3;
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
  if (SEXT816(v3) != SEXT816(v2) * SEXT816(a4)) // branch-flip
    v4 = 0x7fffffffffffffff;
  else {
    if (0x80 <= v3) goto label_1892e;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_1892e:
  if (!a0)
    *a1 = 0;
  if (((a2 <= v2 - v1) || ((v2 = v1 + a2, !SCARRY8(v1,a2) && (((v2 <= a3 || (a3 <= -1)) && (v3 = v2 * a4, SEXT816(v3) == SEXT816(v2) * SEXT816(a4))))))) && ((realloc(a0,v3) || ((a0 && (!v3)))))) {
    *a1 = v2;
    return;
  }
  sub_18a90(); // no-return
}

// Function: sub_18970 @ 0x18970
void sub_18970(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_18a90(); // no-return
}

// Function: sub_18990 @ 0x18990
void sub_18990(unsigned long a0)
{
  sub_18970(a0,1); // tail-call
}

// Function: sub_189a0 @ 0x189a0
void sub_189a0(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_18a90(); // no-return
}

// Function: sub_189c0 @ 0x189c0
void sub_189c0(unsigned long a0)
{
  sub_189a0(a0,1); // tail-call
}

// Function: sub_189d0 @ 0x189d0
void sub_189d0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_18590(a1),a0,a1); // tail-call
}

// Function: sub_18a00 @ 0x18a00
void sub_18a00(char *a0,unsigned long a1)
{
  memcpy((void *)sub_185b0(a1),a0,a1); // tail-call
}

// Function: sub_18a30 @ 0x18a30
void sub_18a30(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_185b0(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_18a70 @ 0x18a70
void sub_18a70(char *a0)
{
  sub_189d0(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_18a90 @ 0x18a90
void sub_18a90(void)
{
  error(dat_251f8,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_18ad0 @ 0x18ad0
unsigned long sub_18ad0(unsigned char *a0,unsigned int a1,unsigned long a2,unsigned long a3,unsigned long a4,char *a5,int a6)
{
  int v1;
  char *v2; // rax
  int *v3; // rax
  unsigned long v4; // stack - 0x48
  
  v1 = sub_18de0(a0,NULL,a1,&v4);
  if (v1) { // branch-flip
    v3 = __errno_location();
    if (v1 != 1) {
      if (v1 == 3)
        *v3 = 0;
      goto label_18b78;
    }
  }
  else {
    if ((a2 <= v4) && (v4 <= a3))
      return v4;
    v3 = __errno_location();
    if (v4 <= 0x3fffffff) {
      *v3 = 0x22;
      goto label_18b78;
    }
  }
  *v3 = 0x4b;
label_18b78:
  v2 = (char *)sub_174c0(a0);
  v1 = *v3;
  if (v1 == 0x16) goto label_18bb0;
  do {
    if (!a6)
      a6 = 1;
    error(a6,v1,"%s: %s",a5,v2);
label_18bb0:
    v1 = 0;
  } while( true );
}

// Function: sub_18be0 @ 0x18be0
void sub_18be0(unsigned char *a0,unsigned long a1,unsigned long a2,unsigned long a3,char *a4,unsigned long a5)
{
  sub_18ad0(a0,10,a1,a2,a3,a4,a5);
}

// Function: sub_18c10 @ 0x18c10
char * sub_18c10(void)
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
    if (!gethostname(v2,v3 - 1U)) {
      v4 = strlen(v2) + 1;
      if (v4 < v3 - 1U) {
        if (v7)
          return v7;
        return (char *)sub_18a00(v2,v4);
      }
      *v1 = 0;
    }
    free(v7);
    if ((0x25 <= *v1) || (0xffffffefffbfeffeU >> ((unsigned long)*v1 & 0x3f) & 1)) break;
    v2 = (char *)sub_18870(NULL,&v6,1,-1,1);
    v3 = v6;
    v7 = v2;
  }
  return NULL;
}

// Function: sub_18d20 @ 0x18d20
void sub_18d20(unsigned int a0,int a1,char a2,long a3,unsigned long a4)
{
  int v1;
  char v2; // stack - 0x32
  char *v3; // r9
  char *v4; // r12
  char *v5;
  long v6; // fs_offset
  char v7; // stack - 0x31
  unsigned long v8; // stack - 0x30
  
  v1 = dat_251f8;
  v8 = *(unsigned long *)(v6 + 0x28);
  if (4 <= a0) { // branch-flip
    if (a0 != 4) {
      abort(); // no-return, return-dupe
    }
    v3 = "invalid %s%s argument \'%s\'";
    if (a1 <= -1) goto label_18db3;
label_18d61:
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
    if (0 <= a1) goto label_18d61;
label_18db3:
    v5 = &v2;
    v7 = 0;
    v4 = &"--"[-(long)a1];
    v2 = a2;
  }
  error(v1,0,dcgettext(NULL,v3,5),v4,v5,a4);
  abort();
}

// Function: sub_18de0 @ 0x18de0
unsigned int sub_18de0(unsigned char *a0,long *a1,unsigned int a2,unsigned long *a3,char *a4)
{
  unsigned char v1;
  unsigned long v10;
  long v11;
  int v12; // esi
  int v13; // r9d
  unsigned int v14;
  char v2 [16];
  unsigned int v3; // eax
  int *v4; // rax
  long *v5; // rax
  unsigned char *v6;
  unsigned long v7;
  unsigned long v8; // rcx
  unsigned char v9;
  
  if (0x25 <= a2)
    __assert_fail("0 <= strtol_base && strtol_base <= 36","lib/xstrtol.c",0x55,"xstrtoumax"); // no-return
  if (!a1)
    a1 = &v11;
  v4 = __errno_location();
  *v4 = 0;
  v9 = *a0;
  v5 = __ctype_b_loc();
  v6 = a0;
  while (*(unsigned char *)(*v5 + 1 + (unsigned long)v9 * 2) & 0x20) {
    v9 = v6[1];
    v6 = &v6[1];
  }
  if (v9 == 0x2d)
    return 4;
  v7 = strtoumax(a0,a1,a2);
  v6 = (unsigned char *)*a1;
  if (v6 != a0) { // branch-flip
    if (*v4) { // branch-flip
      v14 = 1;
      if (*v4 != 0x22)
        return 4;
    }
    else {
      v14 = 0;
    }
    if (!a4) goto label_18e9d;
    v9 = *v6;
    if (!v9) goto label_18e9d;
    v10 = v7;
    if (!strchr(a4,(int)(char)v9)) goto label_18ea5;
  }
  else {
    if ((!a4) || (v9 = *a0, !v9))
      return 4;
    v14 = 0;
    v10 = 1;
    if (!strchr(a4,(int)(char)v9))
      return 4;
  }
  v3 = (unsigned int)v9 - 0x45;
  v13 = 1;
  v8 = 0x400;
  if (((unsigned char)v3 <= 0x2f) && (0x814400308945U >> ((unsigned long)v3 & 0x3f) & 1)) {
    v8 = 0x400;
    v13 = 1;
    if (strchr(a4,0x30)) {
      v1 = v6[1];
      if (v1 != 0x44) { // branch-flip
        if (v1 != 0x69) { // branch-flip
          v13 = (v1 == 0x42) + 1;
          if (v1 == 0x42)
            v8 = 1000;
        }
        else {
          v13 = (v6[2] == 0x42) + 1 + (unsigned int)(v6[2] == 0x42);
        }
      }
      else {
        v13 = 2;
        v8 = 1000;
      }
    }
  }
  v7 = v10;
  switch((unsigned int)v9 - 0x42 & 0xff) {
    case 0:
      v7 = v10 << 10;
      if (v10 >> 0x36) {
        v14 = 1;
        v7 = 0xffffffffffffffff;
      }
      break;
    default:
label_18ea5:
      *a3 = v10;
      return v14 | 2;
    case 3:
      v12 = 6;
      v3 = 0;
      do {
        v2 = ZEXT816(v10);
        v10 = SUB168(v2 * ZEXT816(v8),0);
        if (SUB168(v2 * ZEXT816(v8),8)) {
          v3 = 1;
          v10 = 0xffffffffffffffff;
        }
        v12 -= 1;
      } while (v12);
      goto label_190a4;
    case 5:
    case 0x25:
      if ((SUB168(ZEXT816(v10) * ZEXT816(v8),8)) || (v2 = ZEXT816(SUB168(ZEXT816(v10) * ZEXT816(v8),0)) * ZEXT816(v8), v7 = SUB168(v2,0), SUB168(v2,8))) goto label_19069;
label_19037:
      if (SUB168(ZEXT816(v7) * ZEXT816(v8),8)) { // branch-flip
        v14 = 1;
        v7 = 0xffffffffffffffff;
      }
      else {
        v7 = SUB168(ZEXT816(v7) * ZEXT816(v8),0);
      }
      break;
    case 9:
    case 0x29:
      v7 = SUB168(ZEXT816(v10) * ZEXT816(v8),0);
      if (SUB168(ZEXT816(v10) * ZEXT816(v8),8)) goto label_19069;
      break;
    case 0xb:
    case 0x2b:
      v7 = SUB168(ZEXT816(v10) * ZEXT816(v8),0);
      if (!SUB168(ZEXT816(v10) * ZEXT816(v8),8)) goto label_19037;
label_19069:
      v14 = 1;
      v7 = 0xffffffffffffffff;
      break;
    case 0xe:
      v12 = 5;
      v3 = 0;
      do {
        v2 = ZEXT816(v10);
        v10 = SUB168(v2 * ZEXT816(v8),0);
        if (SUB168(v2 * ZEXT816(v8),8)) {
          v3 = 1;
          v10 = 0xffffffffffffffff;
        }
        v12 -= 1;
      } while (v12);
      goto label_190a4;
    case 0x12:
    case 0x32:
      v12 = 4;
      v3 = 0;
      do {
        v2 = ZEXT816(v10);
        v10 = SUB168(v2 * ZEXT816(v8),0);
        if (SUB168(v2 * ZEXT816(v8),8)) {
          v3 = 1;
          v10 = 0xffffffffffffffff;
        }
        v12 -= 1;
      } while (v12);
      goto label_190a4;
    case 0x17:
      v12 = 8;
      v3 = 0;
      do {
        v2 = ZEXT816(v10);
        v10 = SUB168(v2 * ZEXT816(v8),0);
        if (SUB168(v2 * ZEXT816(v8),8)) {
          v3 = 1;
          v10 = 0xffffffffffffffff;
        }
        v12 -= 1;
      } while (v12);
      goto label_190a4;
    case 0x18:
      v12 = 7;
      v3 = 0;
      do {
        v2 = ZEXT816(v10);
        v10 = SUB168(v2 * ZEXT816(v8),0);
        if (SUB168(v2 * ZEXT816(v8),8)) {
          v3 = 1;
          v10 = 0xffffffffffffffff;
        }
        v12 -= 1;
      } while (v12);
label_190a4:
      v14 |= v3;
      v7 = v10;
      break;
    case 0x20:
      if (v10 >> 0x37) { // branch-flip
        v14 = 1;
        v7 = 0xffffffffffffffff;
      }
      else {
        v7 = v10 << 9;
      }
      break;
    case 0x21:
      break;
    case 0x35:
      if (0 <= (long)v10) // branch-flip
        v7 = v10 * 2;
      else {
        v14 = 1;
        v7 = 0xffffffffffffffff;
      }
    
  }
  *a1 = (long)&v6[v13];
  if (v6[v13])
    v14 |= 2;
label_18e9d:
  *a3 = v7;
  return v14;
}

// Function: sub_19210 @ 0x19210
long sub_19210(unsigned long a0,long a1,int a2,int a3,int a4,unsigned int a5,int a6,int a7,int a8,int a9)
{
  int v1; // eax
  int v2; // ecx
  int v3; // edx
  int v4; // r9d
  
  v1 = ((int)((long)a0 >> 2) + 0x1db) - (unsigned int)((a0 & 3) == 0);
  v4 = ((int)((long)(int)a5 >> 2) + 0x1db) - (unsigned int)((a5 & 3) == 0);
  v3 = (v1 - (v1 >> 0x1f)) / 0x19 + (v1 >> 0x1f);
  v2 = (v4 - (v4 >> 0x1f)) / 0x19 + (v4 >> 0x1f);
  return ((long)a4 + (((long)a3 + (((long)a2 + ((long)(((v1 - v4) - (v3 - v2)) + ((v3 >> 2) - (v2 >> 2))) + (((a0 - (long)(int)a5) * 0x16d + a1) - (long)a6)) * 0x18) - (long)a7) * 0x3c) - (long)a8) * 0x3c) - (long)a9;
}

// Function: sub_19310 @ 0x19310
int * sub_19310(void *a0,long *a1,int *a2)
{
  int *v1; // rax
  int v10; // stack - 0x80
  int v11; // stack - 0x7c
  int v12; // stack - 0x78
  int v13; // stack - 0x74
  int v14; // stack - 0x70
  int v15; // stack - 0x6c
  unsigned long v16; // stack - 0x68
  int *v17; // stack - 0x60
  long v2; // stack - 0x48
  long v3;
  long v4;
  long v5;
  int v6; // stack - 0x94
  int v7; // stack - 0x90
  int v8; // stack - 0x8c
  unsigned long v9; // stack - 0x88
  
  v5 = *a1;
  v2 = v5;
  v17 = (int *)(*a0)(&v2,a2);
  if (v17) // branch-flip
    *a1 = v5;
  else {
    v1 = __errno_location();
    if (((*v1 == 0x4b) && (v3 = (unsigned long)((unsigned int)v5 & 1) + (v5 >> 1), v5 != v3)) && (v3)) {
      v4 = 0;
      v6 = -1;
      do {
        v2 = v3;
        if ((*a0)(&v2,a2)) { // branch-flip
          v6 = *a2;
          v15 = a2[1];
          v14 = a2[2];
          v13 = a2[3];
          v12 = a2[4];
          v8 = a2[5];
          v10 = a2[6];
          v11 = a2[7];
          v7 = a2[8];
          v9 = *(unsigned long *)&a2[10];
          v16 = *(unsigned long *)&a2[0xc];
          v4 = v3;
        }
        else {
          if (*v1 != 0x4b)
            return NULL;
          v5 = v3;
        }
        v3 = (v4 >> 1) + (v5 >> 1) + (unsigned long)(((unsigned int)v4 | (unsigned int)v5) & 1);
      } while ((v3 != v4) && (v3 != v5));
      if (0 <= v6) {
        *a1 = v4;
        *a2 = v6;
        a2[1] = v15;
        a2[2] = v14;
        a2[3] = v13;
        a2[4] = v12;
        a2[5] = v8;
        a2[6] = v10;
        a2[7] = v11;
        a2[8] = v7;
        *(unsigned long *)&a2[10] = v9;
        *(unsigned long *)&a2[0xc] = v16;
        v17 = a2;
      }
    }
  }
  return v17;
}

// Function: sub_19510 @ 0x19510
unsigned long sub_19510(int *a0,void *a1,long *a2)
{
  int v1;
  unsigned long v10;
  int v11;
  int v12; // stack - 0xb8
  unsigned long v13; // stack - 0xd0
  unsigned long v14; // stack - 0x78
  long v15; // stack - 0xc8
  unsigned long v16; // stack - 0xc0
  unsigned long v17; // rbp
  long v18; // rax
  int v19; // eax
  int v2;
  long v20;
  int v21; // r12d
  int v22;
  int v23;
  bool v24;
  bool v25; // zf
  unsigned long v26; // stack - 0x150
  int v27; // stack - 0x130
  unsigned int v28; // stack - 0xb4
  unsigned int v29; // stack - 0xb0
  int v3;
  unsigned int v30; // stack - 0xac
  unsigned int v31; // stack - 0xa8
  unsigned int v32; // stack - 0xa4
  unsigned int v33; // stack - 0xa0
  unsigned int v34; // stack - 0x9c
  unsigned int v35; // stack - 0x98
  unsigned int v36; // stack - 0x94
  unsigned long v37; // stack - 0x90
  unsigned long v38; // stack - 0x88
  unsigned int v39; // stack - 0x70
  int v4;
  unsigned int v40; // stack - 0x64
  unsigned int v41; // stack - 0x5c
  int v42; // stack - 0x58
  int v5;
  unsigned long v6; // rax
  long v7; // rax
  unsigned long v8; // rax
  unsigned long v9;
  
  v1 = *a0;
  v2 = a0[1];
  v3 = a0[2];
  v4 = a0[8];
  v19 = a0[4] % 0xc;
  v5 = v19 >> 0x1f;
  v9 = (long)(int)(a0[4] / 0xc + v5) + (long)a0[5];
  v10 = 0;
  if ((!(v9 & 3)) && (v10 = 1, (v9 * -0x70a3d70a3d70a3d7 + 0x51eb851eb851eb8 >> 2 | v9 * -0x70a3d70a3d70a3d7 << 0x3e) <= 0x28f5c28f5c28f5c))
    v10 = (unsigned long)(((unsigned int)((long)v9 / 100) & 3) == 1);
  v18 = (long)a0[3] + (long)(int)((unsigned int)*(unsigned short *)(((long)(int)((v5 & 0xc) + v19) + v10 * 0xd) * 2 + 0x1eee0) - 1);
  v10 = 0x3b;
  if (v1 <= 0x3b)
    v10 = (long)v1;
  if ((int)v10 <= -1)
    v10 = 0;
  v21 = (int)v10;
  v26._0_4_ = (int)*a2;
  v19 = -(int)v26;
  v6 = sub_19210(v9,v18,v3,v2,v10 & 0xffffffff,0x46,0,0,0,v19);
  v13 = v6;
  v26 = v6;
  v27 = 6;
  v24 = 0;
  v20 = v6;
  while( true ) {
    v17 = v20;
    v7 = sub_19310(a1,&v13,&v12);
    v23 = v12;
    if (!v7)
      return 0xffffffffffffffff;
    v7 = sub_19210(v9,v18,v3,v2,v10 & 0xffffffff,v32,v34,v29,v28,v12);
    v20 = v13;
    if (!v7) break;
    if ((v17 != v13) && (v26 == v13)) {
      if ((int)v35 < 0) goto label_19770;
      if (0 <= v4) { // branch-flip
        if ((v4 != 0) != (v35 != 0)) goto label_19770;
      }
      else if (v24 <= (v35 != 0)) goto label_19770;
    }
    v27 -= 1;
    if (!v27) goto label_19820;
    v8 = v7 + v13;
    v24 = v35 != 0;
    v26 = v17;
    v13 = v8;
  }
  v24 = v4 == 0;
  v25 = v35 == 0;
  if ((v25 != v24) && (0 <= (int)(v35 | v4))) {
    v23 = 0x92c70;
    do {
      v22 = 2;
      v11 = -v23;
      while( true ) {
        v15 = (long)v11 + v20;
        if (!SCARRY8((long)v11,v20)) {
          if (!sub_19310(a1,&v15,&v14))
            return 0xffffffffffffffff;
          if ((v24 == (v42 == 0)) || (v42 <= -1)) {
            v7 = sub_19210(v9,v18,v3,v2,v21,v40,v41,v39,v14 >> 0x20,v14 & 0xffffffff);
            v8 = v7 + v15;
            v16 = v8;
            if ((*a1)(&v16,&v12)) {
              v10 = (unsigned long)v21;
              v13 = v8;
              v23 = v12;
              goto label_19770;
            }
            if (*__errno_location() != 0x4b)
              return 0xffffffffffffffff;
          }
        }
        v11 += v23 * 2;
        if (v22 == 1) break;
        v22 = 1;
      }
      v23 += 0x92c70;
    } while (v23 != 0xdb04f20);
    v10 = (unsigned long)v21;
    v20 += (long)(int)(((unsigned int)v24 - (unsigned int)v25) * 0xe10);
    v13 = v20;
    v14 = v20;
    if (!(*a1)(&v14,&v12)) {
label_19820:
      *__errno_location() = 0x4b;
      return 0xffffffffffffffff;
    }
    v13 = v20;
    v23 = v12;
  }
label_19770:
  *a2 = v13 - ((long)v19 + v6);
  v9 = v13;
  if (v1 != v23) {
    v18 = (long)v1 + ((v1 <= 0 && v23 == 0x3c) - v10);
    v24 = SCARRY8(v18,v13);
    v9 = v18 + v13;
    v13 = v9;
    if (v24) goto label_19820;
    v14 = v9;
    if (!(*a1)(&v14,&v12))
      return 0xffffffffffffffff;
  }
  *(unsigned long *)a0 = CONCAT44(v28,v12);
  *(unsigned long *)&a0[2] = CONCAT44(v30,v29);
  *(unsigned long *)&a0[0xc] = v38;
  *(unsigned long *)&a0[4] = CONCAT44(v32,v31);
  *(unsigned long *)&a0[6] = CONCAT44(v34,v33);
  *(unsigned long *)&a0[8] = CONCAT44(v36,v35);
  *(unsigned long *)&a0[10] = v37;
  return v9;
}

// Function: sub_19ac0 @ 0x19ac0
unsigned long sub_19ac0(int *a0)
{
  unsigned long v1; // rax
  
  tzset();
  v1 = sub_19510(a0,dat_24fc8,(long *)0x26560); // tail-call
  return v1;
}

// Function: sub_19af0 @ 0x19af0
unsigned long sub_19af0(FILE *a0) // early-return x2, return-dupe
{
  unsigned int v1;
  unsigned long v2; // rax
  unsigned long v3; // rax
  
  v2 = __fpending(a0);
  v1 = *(unsigned int *)a0;
  v3 = sub_19b60(a0);
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

// Function: sub_19b60 @ 0x19b60
unsigned long sub_19b60(FILE *a0) // early-return
{
  int v1; // eax
  int v2;
  int *v3; // rax
  
  if (0 <= fileno(a0)) {
    if (__freading(a0)) {
      v2 = fileno(a0);
      if (lseek(v2,0,1) == -1) {
        return fclose(a0); // tail-call, return-dupe
      }
    }
    if (sub_19bf0(a0)) {
      v3 = __errno_location();
      v2 = *v3;
      v1 = fclose(a0);
      if (!v2)
        return CONCAT44(dat_4,v1);
      *v3 = v2;
      return 0xffffffff;
    }
  }
  return fclose(a0);
}

// Function: sub_19bf0 @ 0x19bf0
void sub_19bf0(FILE *a0)
{
  if (a0) {
    if ((__freading(a0)) && (*(unsigned int *)a0 & 0x100)) {
      sub_19c40(a0,0,1);
      fflush(a0); // tail-call
      return;
    }
  }
  fflush(a0); // tail-call
}

// Function: sub_19c40 @ 0x19c40
int sub_19c40(FILE *a0,long a1,int a2) // return-dupe
{
  long v1; // rax
  
  if (((*(long *)&a0->field_0x10 == *(long *)&a0->field_0x8) && (*(long *)&a0->field_0x28 == *(long *)&a0->field_0x20)) && (!*(long *)&a0->field_0x48)) {
    v1 = lseek(fileno(a0),a1,a2);
    if (v1 != -1) {
      *(unsigned int *)a0 = *(unsigned int *)a0 & 0xffffffef;
      *(long *)&a0->field_0x90 = v1;
      return 0;
    }
    return 0xffffffff;
  }
  return fseeko(a0,a1,a2); // tail-call
}

// Function: sub_19cc0 @ 0x19cc0
unsigned long sub_19cc0(char *a0,unsigned long a1)
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

// Function: sub_19d00 @ 0x19d00
char * sub_19d00(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_19d40 @ 0x19d40
void sub_19d40(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_25008); // tail-call
}

// Function: _DT_FINI @ 0x19d54
void _DT_FINI(void)
{
  return;
}

