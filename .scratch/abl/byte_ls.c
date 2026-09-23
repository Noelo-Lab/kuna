// Function: _DT_INIT @ 0x4000
void _DT_INIT(void) // return-dupe
{
  if (!dat_2afc8)
    return;
  (*dat_2afc8)();
}

// Function: sub_4020 @ 0x4020
void sub_4020(void)
{
  (*dat_2ac48)(); // jump-as-call
}

// Function: free @ 0x46c0
void free(void *a0)
{
  (*dat_2af98)(); // jump-as-call
}

// Function: localtime_r @ 0x46d0
tm * localtime_r(void *a0,tm *a1)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_2afb0)(); // jump-as-call
  return v1;
}

// Function: gmtime_r @ 0x46e0
tm * gmtime_r(void *a0,tm *a1)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_2afb8)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x46f0
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_2afc0)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x4700
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2afd0)(); // jump-as-call
  return v1;
}

// Function: __cxa_finalize @ 0x4710
void __cxa_finalize(void)
{
  (*dat_2afe0)(); // jump-as-call
}

// Function: getenv @ 0x4720
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2ac50)(); // jump-as-call
  return v1;
}

// Function: fgetfilecon @ 0x4730
int fgetfilecon(int a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_2ac58)(); // jump-as-call
  return v1;
}

// Function: sigprocmask @ 0x4740
int sigprocmask(int a0,sigset_t *a1,sigset_t *a2)
{
  int v1; // eax
  
  v1 = (*dat_2ac60)(); // jump-as-call
  return v1;
}

// Function: raise @ 0x4750
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_2ac68)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x4760
void abort(void)
{
  (*dat_2ac70)(); // jump-as-call
}

// Function: __errno_location @ 0x4770
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_2ac78)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x4780
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_2ac80)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x4790
void _exit(int a0)
{
  (*dat_2ac88)(); // jump-as-call
}

// Function: strcpy @ 0x47a0
char * strcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2ac90)(); // jump-as-call
  return v1;
}

// Function: __fpending @ 0x47b0
unsigned long __fpending(FILE *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_2ac98)(); // jump-as-call
  return v1;
}

// Function: toupper @ 0x47c0
int toupper(int a0)
{
  int v1; // eax
  
  v1 = (*dat_2aca0)(); // jump-as-call
  return v1;
}

// Function: isatty @ 0x47d0
int isatty(int a0)
{
  int v1; // eax
  
  v1 = (*dat_2aca8)(); // jump-as-call
  return v1;
}

// Function: sigaction @ 0x47e0
int sigaction(int a0,sigaction *a1,sigaction *a2)
{
  int v1; // eax
  
  v1 = (*dat_2acb0)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x47f0
int iswcntrl(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_2acb8)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x4800
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2acc0)(); // jump-as-call
  return v1;
}

// Function: wcswidth @ 0x4810
int wcswidth(wchar_t *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = (*dat_2acc8)(); // jump-as-call
  return v1;
}

// Function: localeconv @ 0x4820
lconv * localeconv(void)
{
  lconv *v1; // rax
  
  v1 = (lconv *)(*dat_2acd0)(); // jump-as-call
  return v1;
}

// Function: faccessat @ 0x4830
int faccessat(int a0,char *a1,int a2,int a3)
{
  int v1; // eax
  
  v1 = (*dat_2acd8)(); // jump-as-call
  return v1;
}

// Function: mbstowcs @ 0x4840
unsigned long mbstowcs(wchar_t *a0,char *a1,unsigned long a2)
{
  unsigned long v1; // rax
  
  v1 = (*dat_2ace0)(); // jump-as-call
  return v1;
}

// Function: readlink @ 0x4850
long readlink(char *a0,char *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_2ace8)(); // jump-as-call
  return v1;
}

// Function: clock_gettime @ 0x4860
int clock_gettime(int a0,timespec *a1)
{
  int v1; // eax
  
  v1 = (*dat_2acf0)(); // jump-as-call
  return v1;
}

// Function: setenv @ 0x4870
int setenv(char *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_2acf8)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x4880
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2ad00)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x4890
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_2ad08)(); // jump-as-call
  return v1;
}

// Function: opendir @ 0x48a0
DIR * opendir(char *a0)
{
  DIR *v1; // rax
  
  v1 = (DIR *)(*dat_2ad10)(); // jump-as-call
  return v1;
}

// Function: getpwuid @ 0x48b0
passwd * getpwuid(unsigned int a0)
{
  passwd *v1; // rax
  
  v1 = (passwd *)(*dat_2ad18)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x48c0
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2ad20)(); // jump-as-call
  return v1;
}

// Function: stpcpy @ 0x48d0
char * stpcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2ad28)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x48e0
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2ad30)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x48f0
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_2ad38)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x4900
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_2ad40)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x4910
void __stack_chk_fail(void)
{
  (*dat_2ad48)(); // jump-as-call
}

// Function: getopt_long @ 0x4920
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_2ad50)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x4930
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_2ad58)(); // jump-as-call
  return v1;
}

// Function: gettext @ 0x4940
char * gettext(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2ad60)(); // jump-as-call
  return v1;
}

// Function: freecon @ 0x4950
void freecon(char *a0)
{
  (*dat_2ad68)(); // jump-as-call
}

// Function: strchr @ 0x4960
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2ad70)(); // jump-as-call
  return v1;
}

// Function: printf @ 0x4970
int printf(char *a0,...)
{
  int v1; // eax
  
  v1 = (*dat_2ad78)(); // jump-as-call
  return v1;
}

// Function: getgrgid @ 0x4980
group * getgrgid(unsigned int a0)
{
  group *v1; // rax
  
  v1 = (group *)(*dat_2ad80)(); // jump-as-call
  return v1;
}

// Function: snprintf @ 0x4990
int snprintf(char *a0,unsigned long a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_2ad88)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x49a0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2ad90)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x49b0
void lseek(void)
{
  (*dat_2ad98)(); // jump-as-call
}

// Function: __assert_fail @ 0x49c0
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_2ada0)(); // jump-as-call
}

// Function: fputs @ 0x49d0
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_2ada8)(); // jump-as-call
  return v1;
}

// Function: fnmatch @ 0x49e0
int fnmatch(char *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_2adb0)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x49f0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2adb8)(); // jump-as-call
  return v1;
}

// Function: ioctl @ 0x4a00
int ioctl(int a0,unsigned long a1,...)
{
  int v1; // eax
  
  v1 = (*dat_2adc0)(); // jump-as-call
  return v1;
}

// Function: getcwd @ 0x4a10
char * getcwd(char *a0,unsigned long a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2adc8)(); // jump-as-call
  return v1;
}

// Function: strspn @ 0x4a20
unsigned long strspn(char *a0,char *a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_2add0)(); // jump-as-call
  return v1;
}

// Function: closedir @ 0x4a30
int closedir(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_2add8)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x4a40
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_2ade0)(); // jump-as-call
  return v1;
}

// Function: _setjmp @ 0x4a50
int _setjmp(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_2ade8)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x4a60
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_2adf0)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x4a70
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2adf8)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x4a80
int ferror_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_2ae00)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x4a90
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2ae08)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x4aa0
void signal(void)
{
  (*dat_2ae10)(); // jump-as-call
}

// Function: dirfd @ 0x4ab0
int dirfd(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_2ae18)(); // jump-as-call
  return v1;
}

// Function: getpwnam @ 0x4ac0
passwd * getpwnam(char *a0)
{
  passwd *v1; // rax
  
  v1 = (passwd *)(*dat_2ae20)(); // jump-as-call
  return v1;
}

// Function: fprintf @ 0x4ad0
int fprintf(FILE *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_2ae28)(); // jump-as-call
  return v1;
}

// Function: sigemptyset @ 0x4ae0
int sigemptyset(sigset_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_2ae30)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x4af0
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_2ae38)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x4b00
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2ae40)(); // jump-as-call
  return v1;
}

// Function: getgrnam @ 0x4b10
group * getgrnam(char *a0)
{
  group *v1; // rax
  
  v1 = (group *)(*dat_2ae48)(); // jump-as-call
  return v1;
}

// Function: putchar_unlocked @ 0x4b20
int putchar_unlocked(int a0)
{
  int v1; // eax
  
  v1 = (*dat_2ae50)(); // jump-as-call
  return v1;
}

// Function: tzset @ 0x4b30
void tzset(void)
{
  (*dat_2ae58)(); // jump-as-call
}

// Function: fileno @ 0x4b40
int fileno(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_2ae60)(); // jump-as-call
  return v1;
}

// Function: tcgetpgrp @ 0x4b50
int tcgetpgrp(int a0)
{
  int v1; // eax
  
  v1 = (*dat_2ae68)(); // jump-as-call
  return v1;
}

// Function: readdir @ 0x4b60
dirent * readdir(DIR *a0)
{
  dirent *v1; // rax
  
  v1 = (dirent *)(*dat_2ae70)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x4b70
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_2ae78)(); // jump-as-call
  return v1;
}

// Function: tolower @ 0x4b80
int tolower(int a0)
{
  int v1; // eax
  
  v1 = (*dat_2ae80)(); // jump-as-call
  return v1;
}

// Function: putc_unlocked @ 0x4b90
int putc_unlocked(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_2ae88)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x4ba0
int fflush(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_2ae90)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x4bb0
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2ae98)(); // jump-as-call
  return v1;
}

// Function: strcoll @ 0x4bc0
int strcoll(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_2aea0)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x4bd0
int __freading(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_2aea8)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x4be0
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_2aeb0)(); // jump-as-call
  return v1;
}

// Function: gnu_dev_major @ 0x4bf0
void gnu_dev_major(void)
{
  (*dat_2aeb8)(); // jump-as-call
}

// Function: realloc @ 0x4c00
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2aec0)(); // jump-as-call
  return v1;
}

// Function: stpncpy @ 0x4c10
char * stpncpy(char *a0,char *a1,unsigned long a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2aec8)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x4c20
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2aed0)(); // jump-as-call
  return v1;
}

// Function: statx @ 0x4c30
int statx(int a0,char *a1,int a2,unsigned int a3,void *a4)
{
  int v1; // eax
  
  v1 = (*dat_2aed8)(); // jump-as-call
  return v1;
}

// Function: longjmp @ 0x4c40
void longjmp(void *a0,int a1)
{
  (*dat_2aee0)(); // jump-as-call
}

// Function: strftime @ 0x4c50
unsigned long strftime(char *a0,unsigned long a1,char *a2,tm *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_2aee8)(); // jump-as-call
  return v1;
}

// Function: mempcpy @ 0x4c60
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2aef0)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x4c70
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2aef8)(); // jump-as-call
  return v1;
}

// Function: error @ 0x4c80
void error(int a0,int a1,char *a2,...)
{
  (*dat_2af00)(); // jump-as-call
}

// Function: fseeko @ 0x4c90
int fseeko(FILE *a0,long a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_2af08)(); // jump-as-call
  return v1;
}

// Function: strtoumax @ 0x4ca0
void strtoumax(void)
{
  (*dat_2af10)(); // jump-as-call
}

// Function: unsetenv @ 0x4cb0
int unsetenv(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_2af18)(); // jump-as-call
  return v1;
}

// Function: gnu_dev_minor @ 0x4cc0
void gnu_dev_minor(void)
{
  (*dat_2af20)(); // jump-as-call
}

// Function: __cxa_atexit @ 0x4cd0
void __cxa_atexit(void)
{
  (*dat_2af28)(); // jump-as-call
}

// Function: wcstombs @ 0x4ce0
unsigned long wcstombs(char *a0,wchar_t *a1,unsigned long a2)
{
  unsigned long v1; // rax
  
  v1 = (*dat_2af30)(); // jump-as-call
  return v1;
}

// Function: gethostname @ 0x4cf0
int gethostname(char *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = (*dat_2af38)(); // jump-as-call
  return v1;
}

// Function: sprintf @ 0x4d00
int sprintf(char *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_2af40)(); // jump-as-call
  return v1;
}

// Function: sigismember @ 0x4d10
int sigismember(void *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_2af48)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x4d20
void exit(int a0)
{
  (*dat_2af50)(); // jump-as-call
}

// Function: getfilecon @ 0x4d30
int getfilecon(char *a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_2af58)(); // jump-as-call
  return v1;
}

// Function: fflush_unlocked @ 0x4d40
int fflush_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_2af60)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x4d50
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_2af68)(); // jump-as-call
  return v1;
}

// Function: lgetfilecon @ 0x4d60
int lgetfilecon(char *a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_2af70)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x4d70
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_2af78)(); // jump-as-call
  return v1;
}

// Function: gnu_dev_makedev @ 0x4d80
void gnu_dev_makedev(void)
{
  (*dat_2af80)(); // jump-as-call
}

// Function: sigaddset @ 0x4d90
int sigaddset(sigset_t *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_2af88)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x4da0
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2af90)(); // jump-as-call
  return v1;
}

// Function: sub_4db0 @ 0x4db0
void sub_4db0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_2afa0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_4de0 @ 0x4de0
void sub_4de0(void)
{
  return;
}

// Function: _FINI_0 @ 0x4e50
void _FINI_0(void)
{
  if (!dat_2b308) {
    if (dat_2afe0)
      __cxa_finalize(dat_2b008);
    sub_4de0();
    dat_2b308 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x4e90
void _INIT_0(void)
{
  return;
}

// Function: sub_4e99 @ 0x4e99
void sub_4e99(int a0) // return-dupe
{
  if (a0 == 1)
    return;
  dat_2b228 = a0;
}

// Function: sub_4eb2 @ 0x4eb2
char sub_4eb2(char a0)
{
  return a0;
}

// Function: sub_4ec1 @ 0x4ec1
unsigned int sub_4ec1(char *a0) // return-dupe, ternary
{
  long v1; // rdx
  
  if (*a0 != '.')
    return 0;
  v1 = (a0[1] != '.') ? 1 : 2; // branch-flip
  if ((a0[v1]) && (a0[v1] != '/'))
    return 0;
  return 1;
}

// Function: sub_4f20 @ 0x4f20
void sub_4f20(void)
{
  FILE *v1;
  
  v1 = stdout;
  fputs_unlocked(gettext("\nMandatory arguments to long options are mandatory for short options too.\n"),v1);
}

// Function: sub_4f51 @ 0x4f51
void sub_4f51(void)
{
  FILE *v1;
  
  v1 = stdout;
  fputs_unlocked(gettext("\nThe SIZE argument is an integer and optional unit (example: 10K is 10*1024).\nUnits are K,M,G,T,P,E,Z,Y (powers of 1024) or KB,MB,... (powers of 1000).\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n"),v1);
}

// Function: sub_4f82 @ 0x4f82
void sub_4f82(char *a0) // ternary
{
  FILE *v1;
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

// Function: sub_5219 @ 0x5219
void sub_5219(long a0,char *a1)
{
  sub_14ef7(a0,a1);
}

// Function: sub_523e @ 0x523e
unsigned long sub_523e(unsigned long a0)
{
  return a0;
}

// Function: sub_524c @ 0x524c
bool sub_524c(int a0)
{
  return a0 == 0x5f;
}

// Function: sub_525c @ 0x525c
unsigned long sub_525c(void)
{
  return 0;
}

// Function: sub_5267 @ 0x5267
undefined16 sub_5267(unsigned long a0,unsigned int a1)
{
  char v1 [16];
  
  v1._8_4_ = a1;
  v1._0_8_ = a0;
  v1._12_4_ = 0;
  return v1._0_16_;
}

// Function: sub_5297 @ 0x5297
void sub_5297(char *a0,struct_4 *a1)
{
  unsigned int v1;
  unsigned int v2;
  char v3 [16];
  
  v1 = *(unsigned int *)&a0[0x8c];
  v2 = *(unsigned int *)&a0[0x88];
  a1->field_0x0 = gnu_dev_makedev(v2,v1);
  a1->field_0x8 = *(unsigned long *)&a0[0x20];
  a1->field_0x18 = (unsigned int)*(unsigned short *)&a0[0x1c];
  a1->field_0x10 = (unsigned long)*(unsigned int *)&a0[0x10];
  a1->field_0x1c = *(unsigned int *)&a0[0x14];
  a1->field_0x20 = *(unsigned int *)&a0[0x18];
  v1 = *(unsigned int *)&a0[0x84];
  v2 = *(unsigned int *)&a0[0x80];
  a1->field_0x28 = gnu_dev_makedev(v2,v1);
  a1->field_0x30 = *(unsigned long *)&a0[0x28];
  a1->field_0x38 = (unsigned long)*(unsigned int *)&a0[4];
  a1->field_0x40 = *(unsigned long *)&a0[0x30];
  v3 = sub_5267(*(unsigned long *)&a0[0x40],*(unsigned long *)&a0[0x48]);
  a1->field_0x48 = SUB168(v3,0);
  a1->field_0x50 = SUB168(v3,8);
  v3 = sub_5267(*(unsigned long *)&a0[0x70],*(unsigned long *)&a0[0x78]);
  a1->field_0x58 = SUB168(v3,0);
  a1->field_0x60 = SUB168(v3,8);
  v3 = sub_5267(*(unsigned long *)&a0[0x60],*(unsigned long *)&a0[0x68]);
  a1->field_0x68 = SUB168(v3,0);
  a1->field_0x70 = SUB168(v3,8);
}

// Function: sub_53e7 @ 0x53e7
unsigned int sub_53e7(char *a0)
{
  unsigned int v1;
  
  if ((dat_2b358) && (a0[0xb9]))
    v1 = *(unsigned int *)&a0[0xac];
  else {
    v1 = *(unsigned int *)&a0[0x30];
  }
  return v1;
}

// Function: sub_5422 @ 0x5422
void sub_5422(char a0)
{
  dat_2b500 += 1;
  putchar_unlocked((int)a0);
}

// Function: sub_5453 @ 0x5453
void sub_5453(char *a0,unsigned long a1)
{
  dat_2b500 = a1 + dat_2b500;
  fwrite_unlocked(a0,1,a1,stdout);
}

// Function: sub_549e @ 0x549e
void sub_549e(char *a0)
{
  sub_5453(a0,strlen(a0));
}

// Function: sub_54cf @ 0x54cf
void sub_54cf(void) // return-dupe
{
  if (!dat_2b3cc)
    return;
  sub_549e("  ");
}

// Function: sub_54f4 @ 0x54f4
void sub_54f4(obstack *a0) // return-dupe
{
  if (!dat_2b3cc)
    return;
  if ((unsigned long)(*(long *)&a0->field_0x20 - *(long *)&a0->field_0x18) < 8)
    _obstack_newchunk(a0,8);
  memcpy(*(void **)&a0->field_0x18,(void *)0x2b500,8);
  *(long *)&a0->field_0x18 = *(long *)&a0->field_0x18 + 8;
}

// Function: sub_558e @ 0x558e
void sub_558e(unsigned long a0,unsigned long a1)
{
  if ((unsigned long)(dat_2b600 - dat_2b5f8) < 0x10)
    _obstack_newchunk((obstack *)0x2b5e0,0x10);
  *(unsigned long *)(dat_2b5f8 + -8) = a0;
  *(unsigned long *)(dat_2b5f8 + -0x10) = a1;
}

// Function: sub_563d @ 0x563d
undefined16 sub_563d(void)
{
  if ((unsigned long)((long)dat_2b5f8 - dat_2b5f0) < 0x10)
    __assert_fail("dev_ino_size <= obstack_object_size (&dev_ino_obstack)","src/ls.c",0x41d,"dev_ino_pop"); // no-return
  dat_2b5f8 = (char (*)[16])((long)dat_2b5f8 + -0x10);
  return *dat_2b5f8;
}

// Function: sub_56dc @ 0x56dc
void sub_56dc(char *a0,long a1,long a2)
{
  stat v1; // stack - 0xa8
  
  if (!a0)
    __assert_fail("name","src/ls.c",0x428,"assert_matching_dev_ino"); // no-return
  if (stat(a0,&v1) <= -1)
    __assert_fail("0 <= stat (name, &sb)","src/ls.c",0x429,"assert_matching_dev_ino"); // no-return
  if (v1._0_8_ != a2)
    __assert_fail("sb.st_dev == di.st_dev","src/ls.c",0x42a,"assert_matching_dev_ino"); // no-return
  if (v1._8_8_ == a1)
    return;
  __assert_fail("sb.st_ino == di.st_ino","src/ls.c",0x42b,"assert_matching_dev_ino"); // no-return
}

// Function: sub_581f @ 0x581f
void sub_581f(char *a0,char *a1) // return-dupe
{
  long v1;
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x40
  
  v2 = (unsigned long)(*(long *)&a1[0x18] - *(long *)&a1[0x10]) >> 3;
  if (!v2)
    return;
  v1 = *(long *)&a1[0x10];
  if (v1 == *(long *)&a1[0x18])
    a1[0x50] = a1[0x50] | 2;
  *(unsigned long *)&a1[0x18] = ~*(unsigned long *)&a1[0x30] & *(long *)&a1[0x18] + *(long *)&a1[0x30];
  if ((unsigned long)(*(long *)&a1[0x20] - *(long *)&a1[8]) < (unsigned long)(*(long *)&a1[0x18] - *(long *)&a1[8]))
    *(unsigned long *)&a1[0x18] = *(unsigned long *)&a1[0x20];
  *(unsigned long *)&a1[0x10] = *(unsigned long *)&a1[0x18];
  fputs_unlocked(a0,stdout);
  for (v3 = 0; v3 < v2; v3 = v3 + 1) {
    printf(" %ld",*(long *)(v1 + v3 * 8));
  }
  putchar_unlocked(10);
}

// Function: sub_5990 @ 0x5990
undefined16 sub_5990(char *a0)
{
  return sub_1b491(a0);
}

// Function: sub_59be @ 0x59be
unsigned long sub_59be(void) // return-dupe x3
{
  switch(dat_2b3b0) {
    case 0:
      return 0x40;
    default:
      abort(); // no-return
    case 1:
      return 0x80;
    case 2:
      return 0x20;
    case 3:
      return 0x800;
    
  }
}

// Function: sub_5a0c @ 0x5a0c
unsigned int sub_5a0c(void)
{
  unsigned int v1; // eax
  unsigned int v2; // stack - 0xc
  
  v2 = 2;
  if (dat_2b3e9)
    v2 = 0x102;
  if (dat_2b3bb)
    v2 |= 0x400;
  if (!dat_2b3ac) {
    v1 = sub_59be() | v2;
    v2 = v1 | 0x204;
    if ((dat_2b022) || (dat_2b3b9))
      v2 = v1 | 0x20c;
    if (dat_2b023)
      v2 |= 0x10;
  }
  switch(dat_2b3b4) {
    case 0:
    case 1:
    case 2:
    case 4:
    case 6:
      break;
    case 3:
      v2 |= 0x200;
      break;
    case 5:
      v2 |= sub_59be();
      break;
    default:
      abort(); // no-return
    
  }
  return v2;
}

// Function: sub_5ad2 @ 0x5ad2
int sub_5ad2(int a0,char *a1,struct_4 *a2,unsigned int a3,unsigned int a4)
{
  int v1; // eax
  unsigned int v2 [20]; // stack - 0x128
  char v3 [16];
  unsigned long v4; // stack - 0xd8
  unsigned long v5; // stack - 0xd0
  
  v1 = statx(a0,a1,a3 | 0x800,a4,v2);
  if ((0 <= v1) && (sub_5297(v2,a2), a4 & 0x800)) {
    if (v2[0] & 0x800) { // branch-flip
      v3 = sub_5267(v4,v5);
      a2->field_0x58 = SUB168(v3,0);
      a2->field_0x60 = SUB168(v3,8);
    }
    else {
      a2->field_0x60 = 0xffffffffffffffff;
      a2->field_0x58 = a2->field_0x60;
    }
  }
  return v1;
}

// Function: sub_5c0e @ 0x5c0e
void sub_5c0e(char *a0,struct_4 *a1)
{
  sub_5ad2(0xffffff9c,a0,a1,0,sub_5a0c());
}

// Function: sub_5c48 @ 0x5c48
void sub_5c48(char *a0,struct_4 *a1)
{
  sub_5ad2(0xffffff9c,a0,a1,0x100,sub_5a0c());
}

// Function: sub_5c82 @ 0x5c82
void sub_5c82(char *a0,struct_4 *a1)
{
  sub_5ad2(0xffffff9c,a0,a1,0,2);
}

// Function: sub_5cb8 @ 0x5cb8
void sub_5cb8(char *a0,struct_4 *a1)
{
  sub_5ad2(0xffffff9c,a0,a1,0,0x100);
}

// Function: sub_5cee @ 0x5cee
void sub_5cee(unsigned int a0,struct_4 *a1)
{
  sub_5ad2(a0,"",a1,0x1000,0x100);
}

// Function: sub_5d23 @ 0x5d23
char * sub_5d23(char *a0)
{
  char *v1; // stack - 0x10
  
  v1 = a0;
  do {
    if (!*v1)
      return NULL;
    if (*v1 == '%') {
      if (v1[1] != '%') { // branch-flip
        if (v1[1] == 'b')
          return v1;
      }
      else {
        v1 = &v1[1];
      }
    }
    v1 = &v1[1];
  } while( true );
}

// Function: sub_5d77 @ 0x5d77
void sub_5d77(void) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_5d77
{
  unsigned char v1; // cl
  int v2; // stack - 0xc
  
  for (v2 = 0; v2 <= 0xff; v2 = v2 + 1) {
    v1 = ((((sub_10016(v2)) || (v2 == 0x7e)) || ((v2 == 0x2d || (v2 == 0x2e)))) || (v2 == 0x5f));
    *(unsigned char *)((long)v2 + 0x2b640) = *(unsigned char *)((long)v2 + 0x2b640) | v1;
  }
}

// Function: sub_5df4 @ 0x5df4
unsigned long sub_5df4(long a0)
{
  char v1;
  long v2;
  char *v3; // rax
  unsigned long v4; // stack - 0x48
  int v5; // stack - 0x50
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  char *v8; // stack - 0x30
  unsigned long v9; // stack - 0x28
  
  v6 = 0xc;
  do {
    v7 = v6;
    v6 = 0;
    v5 = 0;
    while (v5 <= 0xb) {
      v4 = v7;
      v3 = nl_langinfo(v5 + 0x2000e);
      v8 = v3;
      if (strchr(v3,0x25))
        return 0;
      v2 = *(long *)__ctype_b_loc();
      v1 = *v8;
      v9 = sub_15291(v8,(char *)((long)v5 * 0x80 + a0),0x80,&v4,(*(unsigned short *)((unsigned long)(unsigned char)sub_4eb2((int)v1) * 2 + v2) & 0x800) != 0,0);
      if (0x80 <= v9)
        return 0;
      if (v4 <= v6)
        v4 = v6;
      v5 += 1;
      v6 = v4;
    }
    if (v7 <= v6)
      return 1;
  } while( true );
}

// Function: sub_5f43 @ 0x5f43
void sub_5f43(void)
{
  char *v1;
  int v10; // stack - 0x640
  long v2;
  long v3;
  char *v4; // rax
  long v5 [2]; // stack - 0x628
  char v6 [1544];
  int v7; // stack - 0x64c
  int v8; // stack - 0x648
  int v9; // stack - 0x644
  
  for (v7 = 0; v7 <= 1; v7 = v7 + 1) {
    v1 = *(char **)((long)v7 * 8 + 0x2b200);
    v5[v7] = sub_5d23(v1);
  }
  if (((v5[0]) || (v5[1])) && (sub_5df4(v6) == '\x01')) {
    for (v8 = 0; v8 <= 1; v8 = v8 + 1) {
      v1 = *(char **)((long)v8 * 8 + 0x2b200);
      for (v9 = 0; v9 <= 0xb; v9 = v9 + 1) {
        v4 = (char *)(((long)v8 * 0xc + (long)v9) * 0x80 + 0x2b740);
        if (v5[v8]) { // branch-flip
          if (0x80 < v5[v8] - (long)v1)
            return;
          v2 = v5[v8];
          v3 = v5[v8];
          v10 = snprintf(v4,0x80,"%.*s%s%s",(int)v2 - (int)v1,v1,&v6[(long)v9 * 0x80],(char *)(v3 + 2));
        }
        else {
          v10 = snprintf(v4,0x80,"%s",v1);
        }
        if (v10 < 0)
          return;
        if (0x7f < v10)
          return;
      }
    }
    dat_2c340 = 1;
  }
}

// Function: sub_61bd @ 0x61bd
unsigned long sub_61bd(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_61ea @ 0x61ea
unsigned int sub_61ea(struct_0 *a0,struct_0 *a1) // return-dupe
{
  if ((a0->field_0x0 == a1->field_0x0) && (a0->field_0x8 == a1->field_0x8))
    return 1;
  return 0;
}

// Function: sub_6243 @ 0x6243
void sub_6243(void *a0)
{
  free(a0);
}

// Function: sub_6262 @ 0x6262
bool sub_6262(unsigned long a0,unsigned long a1)
{
  struct_8 *v1; // rax
  struct_8 *v2; // rax
  
  v1 = (struct_8 *)sub_1c68b(0x10);
  v1->field_0x0 = a1;
  v1->field_0x8 = a0;
  v2 = (struct_8 *)sub_136d0(dat_2b320,v1);
  if (!v2)
    sub_1ccc3(); // no-return
  if (v2 != v1)
    free(v1);
  return v2 != v1;
}

// Function: sub_62e7 @ 0x62e7
void sub_62e7(struct_1 *a0)
{
  free(a0->field_0x0);
  free(a0->field_0x8);
  free(a0);
}

// Function: sub_6325 @ 0x6325
unsigned int sub_6325(unsigned int a0)
{
  long v1;
  char *v2;
  unsigned int v3; // eax
  
  v1 = *(long *)((unsigned long)a0 * 0x10 + 0x2b080);
  v2 = *(char **)((unsigned long)a0 * 0x10 + 0x2b088);
  if (v1) { // branch-flip
    if (v1 == 1) {
      if (!strncmp(v2,"0",1)) {
        v3 = 0;
        return v3;
      }
    }
    if (v1 == 2) {
      if (!strncmp(v2,"00",2)) {
        v3 = 0;
        return v3;
      }
    }
    v3 = 1;
  }
  else {
    v3 = 0;
  }
  return v3;
}

// Function: sub_63ca @ 0x63ca
void sub_63ca(void)
{
  sub_e4ca((struct_2 *)0x2b080);
  sub_e4ca((struct_2 *)0x2b090);
}

// Function: sub_63f3 @ 0x63f3
void sub_63f3(void)
{
  if (dat_2b3d4) {
    if (sub_6325(4)) {
      sub_e4ca((struct_2 *)0x2b080);
      sub_e4ca((struct_2 *)0x2b0c0);
      sub_e4ca((struct_2 *)0x2b090);
    }
  }
}

// Function: sub_6444 @ 0x6444
void sub_6444(unsigned int a0) // return-dupe
{
  if (dat_2b4e0)
    return;
  dat_2b4e0 = a0;
}

// Function: sub_6465 @ 0x6465
void sub_6465(void) // return-dupe
{
  if (dat_2b4e0)
    return;
  dat_2b4e4 += 1;
}

// Function: sub_648c @ 0x648c
void sub_648c(void)
{
  sigset_t v1;
  int v2; // stack - 0xa0
  
  while ((dat_2b4e0 || (dat_2b4e4))) {
    if (dat_2b3d6)
      sub_63ca();
    fflush_unlocked(stdout);
    sigprocmask(0,(sigset_t *)0x2b460,&v1);
    v2 = dat_2b4e0;
    if (dat_2b4e4) { // branch-flip
      dat_2b4e4 -= 1;
      v2 = 0x13;
    }
    else {
      signal(dat_2b4e0,0);
    }
    raise(v2);
    sigprocmask(2,&v1,NULL);
  }
}

// Function: sub_6593 @ 0x6593
void sub_6593(bool a0) // return-dupe, ternary
{
  int v1;
  void *v2 [17]; // stack - 0xa8
  int v3; // stack - 0xac
  undefined4 v4; // stack - 0x20
  
  if (a0) {
    sigemptyset((sigset_t *)0x2b460);
    for (v3 = 0; v3 <= 0xb; v3 = v3 + 1) {
      sigaction(*(int *)((long)v3 * 4 + 0x23380),NULL,(sigaction *)v2);
      if (v2[0] != (void *)0x1)
        sigaddset((sigset_t *)0x2b460,*(int *)((long)v3 * 4 + 0x23380));
    }
    v2[1] = (void *)dat_2b460;
    v2[2] = (void *)dat_2b468;
    v2[3] = (void *)dat_2b470;
    v2[4] = (void *)dat_2b478;
    v2[5] = (void *)dat_2b480;
    v2[6] = (void *)dat_2b488;
    v2[7] = (void *)dat_2b490;
    v2[8] = (void *)dat_2b498;
    v2[9] = (void *)dat_2b4a0;
    v2[10] = (void *)dat_2b4a8;
    v2[0xb] = (void *)dat_2b4b0;
    v2[0xc] = (void *)dat_2b4b8;
    v2[0xd] = (void *)dat_2b4c0;
    v2[0xe] = (void *)dat_2b4c8;
    v2[0xf] = (void *)dat_2b4d0;
    v2[0x10] = (void *)dat_2b4d8;
    v4 = 0x10000000;
    for (v3 = 0; v3 <= 0xb; v3 = v3 + 1) {
      v1 = *(int *)((long)v3 * 4 + 0x23380);
      if (sigismember((void *)0x2b460,v1)) {
        v2[0] = (*(int *)((long)v3 * 4 + 0x23380) != 0x14) ? sub_6444 : sub_6465; // branch-flip
        sigaction(*(int *)((long)v3 * 4 + 0x23380),(sigaction *)v2,NULL);
      }
    }
    return;
  }
  for (v3 = 0; v3 <= 0xb; v3 = v3 + 1) {
    v1 = *(int *)((long)v3 * 4 + 0x23380);
    if (sigismember((void *)0x2b460,v1))
      signal(*(unsigned int *)((long)v3 * 4 + 0x23380),0);
  }
}

// Function: sub_6858 @ 0x6858
void sub_6858(void)
{
  sub_6593(1);
}

// Function: sub_686d @ 0x686d
void sub_686d(void)
{
  sub_6593(0);
}

// Function: main @ 0x6882
int main(int argc,char **argv,char **envp) // warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in main
{
  int v1;
  char v10 [16];
  int v11; // stack - 0x44
  int v12; // stack - 0x40
  unsigned long v13; // stack - 0x20
  long *v2;
  long *v3;
  char *v4;
  long *v5;
  int v6; // eax
  unsigned int v7; // eax
  void *v8; // rax
  unsigned long v9; // stack - 0x28
  
  sub_18e89(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_4e99(2);
  sub_1ec10(sub_110ae);
  dat_2b4e8 = 0;
  dat_2b428 = 1;
  dat_2b368 = NULL;
  dat_2b370 = 0x8000000000000000;
  dat_2b378 = 0xffffffffffffffff;
  v6 = sub_70af(argc,argv);
  if (dat_2b3d4)
    sub_88ab();
  if (dat_2b3d4)
    dat_2b420 = 0;
  if (dat_2b3f2) // branch-flip
    dat_2b3e8 = 1;
  else if ((dat_2b3d4) && (((sub_6325(0xd) || ((sub_6325(0xe) && (dat_2b358)))) || ((sub_6325(0xc) && (!dat_2b3ac))))))
    dat_2b3e8 = 1;
  if (!dat_2b3ec) {
    if (((dat_2b3f1) || (dat_2b3d0 == 3)) || (!dat_2b3ac))
      dat_2b3ec = 1;
    else {
      dat_2b3ec = 3;
    }
  }
  if (dat_2b3f0) {
    dat_2b320 = sub_1298f(0x1e,NULL,sub_61bd,sub_61ea,sub_6243);
    if (!dat_2b320)
      sub_1ccc3(); // no-return
    _obstack_begin((obstack *)0x2b5e0,0,0,dat_2afd0,dat_2af98);
  }
  dat_2b438 = sub_1b525(getenv("TZ"));
  dat_2b440 = !(((dat_2b3b4 != 5) && (dat_2b3b4 != 3)) && ((dat_2b3ac && ((!dat_2b380 && (!dat_2b3bb))))));
  dat_2b441 = ((dat_2b440 != '\x01') && ((((dat_2b3f0 || (dat_2b3d4)) || (dat_2b3d0)) || (dat_2b3f2))));
  if (dat_2b3cc) {
    _obstack_begin((obstack *)0x2b520,0,0,dat_2afd0,dat_2af98);
    _obstack_begin((obstack *)0x2b580,0,0,dat_2afd0,dat_2af98);
  }
  if (dat_2b3d5) {
    sub_5d77();
    dat_2b360 = sub_1ce96();
    if (!dat_2b360)
      dat_2b360 = 0x1f27b;
  }
  dat_2b330 = 100;
  dat_2b328 = sub_1c804(100,0xd0);
  dat_2b338 = 0;
  sub_96f3();
  v11 = v6; // branch-flip
  if (1 <= argc - v6) {
    do {
      v1 = v11 + 1;
      sub_9a40(argv[v11],0,0,1,"");
      v11 = v1;
    } while (v1 < argc);
  }
  else if (dat_2b3f1) // branch-flip
    sub_9a40(".",3,0,1,"");
  else {
    sub_8e46(".",NULL,1);
  }
  if ((dat_2b338) && (sub_bfc5(), dat_2b3f1 != '\x01'))
    sub_aa65(NULL,1);
  if (dat_2b338) { // branch-flip
    sub_c12d();
    if (dat_2b368)
      sub_5422(10);
  }
  else if (((argc - v6 <= 1) && (dat_2b368)) && (!dat_2b368[3]))
    dat_2b428 = 0;
  while (v5 = dat_2b368, dat_2b368) {
    if ((dat_2b320) && (!*dat_2b368)) {
      dat_2b368 = (long *)dat_2b368[3];
      v10 = sub_563d();
      v13 = SUB168(v10,8);
      v9 = SUB168(v10,0);
      v8 = (void *)sub_13740(dat_2b320,&v9);
      if (!v8)
        __assert_fail("found","src/ls.c",0x70d,"main"); // no-return
      sub_6243(v8);
      sub_62e7(v5);
    }
    else {
      v2 = &dat_2b368[2];
      v3 = &dat_2b368[1];
      v4 = (char *)*dat_2b368;
      dat_2b368 = (long *)dat_2b368[3];
      sub_8ede(v4,(char *)*v3,(unsigned char)*(char *)v2);
      sub_62e7(v5);
      dat_2b428 = 1;
    }
  }
  if ((dat_2b3d4) && (dat_2b3d6)) {
    if ((dat_2b080 != 2) || (((memcmp(dat_2b088,(void *)0x20638,2) || (dat_2b090 != 1)) || (*dat_2b098 != 'm'))))
      sub_63ca();
    fflush_unlocked(stdout);
    sub_686d();
    for (v12 = dat_2b4e4; v12; v12 = v12 + -1) {
      raise(0x13);
    }
    if (dat_2b4e0)
      raise(dat_2b4e0);
  }
  if (dat_2b3cc) {
    sub_581f("//DIRED//",(char *)0x2b520);
    sub_581f("//SUBDIRED//",(char *)0x2b580);
    v7 = sub_18fc3(dat_2b410);
    printf("//DIRED-OPTIONS// --quoting-style=%s\n",*(char **)((unsigned long)v7 * 8 + 0x2a9e0));
  }
  if (dat_2b320) {
    if (sub_12002(dat_2b320))
      __assert_fail("hash_get_n_entries (active_dir_set) == 0","src/ls.c",0x741,"main"); // no-return
    sub_12c0f(dat_2b320);
  }
  return dat_2b4e8;
}

// Function: sub_6fe6 @ 0x6fe6
long sub_6fe6(unsigned char *a0) // early-return x3
{
  int v1; // eax
  long v2; // stack - 0x18
  
  v1 = sub_1d251(a0,NULL,0,&v2,"");
  if (!v1) {
    if (0 <= v2)
      return v2;
    return 0;
  }
  if (v1 != 1)
    return -1;
  return 0;
}

// Function: sub_706a @ 0x706a
bool sub_706a(void)
{
  if (dat_2b031 <= '\xff')
    dat_2b031 = (char)isatty(1);
  return dat_2b031 != '\0';
}

// Function: sub_70af @ 0x70af
unsigned long sub_70af(int a0,char **a1) // ternary x4, warn: iteboolean: re-rolled 4 0/1 select diamond(s) to a boolean assignment in sub_70af
{
  bool v1;
  long v10; // rax
  unsigned short v11; // stack - 0x28
  int v12; // stack - 0xc4
  int v13; // stack - 0xc0
  int v14; // stack - 0xbc
  int v15; // stack - 0xb8
  int v16; // stack - 0xb4
  int v17; // stack - 0xb0
  int v18; // stack - 0xac
  int v19; // stack - 0xa8
  FILE *v2;
  int v20; // stack - 0xa4
  char *v21; // stack - 0x98
  long v22; // stack - 0x90
  unsigned long v23; // stack - 0x88
  unsigned long v24; // stack - 0x80
  char *v25; // stack - 0x78
  char *v26; // stack - 0x70
  char *v27; // stack - 0x68
  long *v28; // stack - 0x60
  unsigned short v29; // stack - 0x26
  int v3; // eax
  unsigned int v30; // stack - 0x24
  unsigned long v4; // rax
  long *v5;
  unsigned char *v6; // rax
  char *v7;
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v21 = NULL;
  v1 = 0;
  v12 = -1;
  v13 = -1;
  v14 = -1;
  v15 = -1;
  v22 = -1;
  v23 = 0xffffffffffffffff;
label_7128:
  v29 = 0xffff;
  v11 = 0xffff;
  v3 = getopt_long(a0,a1,"abcdfghiklmnopqrstuvw:xABCDFGHI:LNQRST:UXZ1",(option *)0x2a0c0,(int *)&v11);
  if (v3 != -1) {
    switch(v3) {
      case 0x31:
        if (v12)
          v12 = 1;
        break;
      case 0x41:
        dat_2b3f4 = 1;
        break;
      case 0x42:
        sub_94e9("*~");
        sub_94e9(".*~");
        break;
      case 0x43:
        v12 = 2;
        break;
      case 0x44:
        dat_2b3cc = 1;
        break;
      case 0x46:
        if (optarg) // branch-flip
          v16 = *(int *)(sub_fe17("--classify",optarg,0x2a720,0x20980,4,dat_2b220,1) * 4 + 0x20980);
        else {
          v16 = 1;
        }
        if (v16 != 1) {
          if (v16 != 2) break;
          if (!sub_706a()) break;
        }
        dat_2b3d0 = 3;
        break;
      case 0x47:
        dat_2b023 = 0;
        break;
      case 0x48:
        dat_2b3ec = 2;
        break;
      case 0x49:
        sub_94e9(optarg);
        break;
      case 0x4c:
        dat_2b3ec = 4;
        break;
      case 0x4e:
        v14 = 0;
        break;
      case 0x51:
        v14 = 5;
        break;
      case 0x52:
        dat_2b3f0 = 1;
        break;
      case 0x53:
        v15 = 3;
        break;
      case 0x54:
        v7 = gettext("invalid tab size");
        v22 = sub_1cd03(optarg,0,0,0x7fffffffffffffff,0x1f27b,v7,2);
        break;
      case 0x55:
        v15 = 6;
        break;
      case 0x58:
        v15 = 1;
        break;
      case 0x5a:
        dat_2b380 = 1;
        break;
      case 0x61:
        dat_2b3f4 = 2;
        break;
      case 0x62:
        v14 = 7;
        break;
      case 99:
        dat_2b3b0 = 1;
        break;
      case 100:
        dat_2b3f1 = 1;
        break;
      case 0x66:
        dat_2b3f4 = 2;
        v15 = 6;
        if (!v12)
          v12 = -1;
        dat_2b3d4 = '\0';
        dat_2b3d5 = 0;
        dat_2b3bb = 0;
        break;
      case 0x67:
        v12 = 0;
        dat_2b022 = 0;
        break;
      case 0x68:
        dat_2b3bc = 0xb0;
        dat_2b3c8 = 0xb0;
        dat_2b3c0 = 1;
        dat_2b028 = 1;
        break;
      case 0x69:
        dat_2b3e9 = 1;
        break;
      case 0x6b:
        v1 = 1;
        break;
      case 0x6c:
        v12 = 0;
        break;
      case 0x6d:
        v12 = 4;
        break;
      case 0x6e:
        dat_2b3ba = 1;
        v12 = 0;
        break;
      case 0x6f:
        v12 = 0;
        dat_2b023 = 0;
        break;
      case 0x70:
        dat_2b3d0 = 1;
        break;
      case 0x71:
        v13 = 1;
        break;
      case 0x72:
        dat_2b3b8 = 1;
        break;
      case 0x73:
        dat_2b3bb = 1;
        break;
      case 0x74:
        v15 = 5;
        break;
      case 0x75:
        dat_2b3b0 = 2;
        break;
      case 0x76:
        v15 = 4;
        break;
      case 0x77:
        v23 = sub_6fe6(optarg);
        if (0 <= (long)v23) break;
        v4 = sub_1aeca(optarg);
        error(2,0,"%s: %s",gettext("invalid line width"),v4);
        goto label_73b3;
      case 0x78:
label_73b3:
        v12 = 3;
        break;
      case 0x80:
        dat_2b3b9 = 1;
        break;
      case 0x81:
        v3 = sub_149fd(optarg,(unsigned int *)0x2b3bc,(long *)0x2b3c0);
        if (v3)
          sub_1d130(v3,CONCAT22(v29,v11),0,0x2a0c0,optarg); // no-return
        dat_2b3c8 = dat_2b3bc;
        dat_2b028 = dat_2b3c0;
        break;
      case 0x82:
        if (optarg) // branch-flip
          v17 = *(int *)(sub_fe17("--color",optarg,0x2a720,0x20980,4,dat_2b220,1) * 4 + 0x20980);
        else {
          v17 = 1;
        }
        dat_2b3d4 = !((v17 != 1) && ((v17 != 2 || (!sub_706a()))));
        break;
      case 0x83:
        dat_2b3ec = 3;
        break;
      case 0x84:
        dat_2b3d0 = 2;
        break;
      case 0x85:
        v12 = *(int *)(sub_fe17("--format",optarg,0x2a660,0x208a0,4,dat_2b220,1) * 4 + 0x208a0);
        break;
      case 0x86:
        v12 = 0;
        v21 = (char *)sub_523e("full-iso");
        break;
      case 0x87:
        dat_2b3f2 = 1;
        break;
      case 0x88:
        v5 = (long *)sub_1c68b(0x10);
        *v5 = (long)optarg;
        v5[1] = (long)dat_2b400;
        dat_2b400 = v5;
        break;
      case 0x89:
        if (optarg) // branch-flip
          v18 = *(int *)(sub_fe17("--hyperlink",optarg,0x2a720,0x20980,4,dat_2b220,1) * 4 + 0x20980);
        else {
          v18 = 1;
        }
        dat_2b3d5 = !((v18 != 1) && ((v18 != 2 || (!sub_706a()))));
        break;
      case 0x8a:
        dat_2b3d0 = *(unsigned int *)(sub_fe17("--indicator-style",optarg,0x29fa0,0x205e0,4,dat_2b220,1) * 4 + 0x205e0);
        break;
      case 0x8b:
        v14 = *(int *)(sub_fe17("--quoting-style",optarg,0x2a9e0,0x238c0,4,dat_2b220,1) * 4 + 0x238c0);
        break;
      case 0x8c:
        v13 = 0;
        break;
      case 0x8d:
        dat_2b3bc = 0x90;
        dat_2b3c8 = 0x90;
        dat_2b3c0 = 1;
        dat_2b028 = 1;
        break;
      case 0x8e:
        v15 = *(int *)(sub_fe17("--sort",optarg,0x2a6a0,0x208d0,4,dat_2b220,1) * 4 + 0x208d0);
        break;
      case 0x8f:
        dat_2b3b0 = *(int *)(sub_fe17("--time",optarg,0x2a6e0,0x20920,4,dat_2b220,1) * 4 + 0x20920);
        break;
      case 0x90:
        v21 = optarg;
        break;
      case 0x91:
        goto label_7a55;
      case 0xffffff7d:
        if (dat_2b210 != 1) { // branch-flip
          v7 = (dat_2b210 != 2) ? "vdir" : "dir"; // branch-flip
        }
        else {
          v7 = "ls";
        }
        sub_1c4ab(stdout,v7,"GNU coreutils",dat_2b218,"Richard M. Stallman","David MacKenzie",0);
        exit(0); // no-return
      case 0xffffff7e:
        sub_f135(0); // no-return
      default:
        sub_f135(2); // no-return
      
    }
    goto label_7128;
  }
  if (!dat_2b3c0) {
    v6 = (unsigned char *)getenv("LS_BLOCK_SIZE");
    sub_149fd(v6,(unsigned int *)0x2b3bc,(long *)0x2b3c0);
    if ((v6) || (getenv("BLOCK_SIZE"))) {
      dat_2b3c8 = dat_2b3bc;
      dat_2b028 = dat_2b3c0;
    }
    if (v1) {
      dat_2b3bc = 0;
      dat_2b3c0 = 0x400;
    }
  }
  if (v12 <= -1) {
    if (dat_2b210 != 1) { // branch-flip
      v12 = (dat_2b210 != 2) ? 0 : 2; // branch-flip
    }
    else {
      v12 = (sub_706a()) ? 2 : 1; // branch-flip
    }
  }
  v24 = v23;
  dat_2b3ac = v12;
  if ((((v12 == 2) || (v12 == 3)) || (v12 == 4)) || (dat_2b3d4)) {
    if ((((long)v23 <= -1) && (sub_706a())) && ((0 <= ioctl(1,0x5413,&v11) && (v29))))
      v24 = (unsigned long)v29;
    if ((((long)v24 <= -1) && (v6 = (unsigned char *)getenv("COLUMNS"), v6)) && (*v6)) {
      v24 = sub_6fe6(v6);
      if ((long)v24 <= -1) {
        v7 = (char *)sub_1aeca(v6);
        error(0,0,gettext("ignoring invalid width in environment variable COLUMNS: %s"),v7);
      }
    }
  }
  if ((long)v24 < 0)
    v24 = 0x50;
  dat_2b4f8 = v24 / 3 + (unsigned long)(v24 % 3 != 0);
  dat_2b430 = v24;
  if (((dat_2b3ac == 2) || (dat_2b3ac == 3)) || (dat_2b3ac == 4)) {
    if (0 <= v22) // branch-flip
      dat_2b420 = v22;
    else {
      dat_2b420 = 8;
      v6 = (unsigned char *)getenv("TABSIZE");
      if (v6) {
        if (sub_1d251(v6,NULL,0,&v11,"")) { // branch-flip
          v7 = (char *)sub_1aeca(v6);
          error(0,0,gettext("ignoring invalid tab size in environment variable TABSIZE: %s"),v7);
        }
        else {
          dat_2b420 = CONCAT44(v30,CONCAT22(v29,v11));
        }
      }
    }
  }
  if (0 <= v13) // branch-flip
    dat_2b408 = v13 != 0;
  else {
    dat_2b408 = !((dat_2b210 != 1) || (!sub_706a()));
  }
  v19 = v14;
  if (v14 <= -1)
    v19 = sub_8cff();
  if (v19 <= -1) {
    if (dat_2b210 != 1) // branch-flip
      v19 = 7;
    else {
      v19 = (sub_706a()) ? 3 : -1; // branch-flip
    }
  }
  if (0 <= v19)
    sub_18fe7(NULL,v19);
  v3 = sub_18fc3(NULL);
  dat_2b341 = !(((dat_2b3ac) && (((dat_2b3ac != 2 && (dat_2b3ac != 3)) || (!dat_2b430)))) || (((v3 != 1 && (v3 != 3)) && (v3 != 6))));
  dat_2b410 = sub_18f74(NULL);
  if (v3 == 7)
    sub_19012(dat_2b410,0x20,1);
  if (2 <= dat_2b3d0) {
    for (v25 = &"*=>@|"[dat_2b3d0 - 2]; *v25; v25 = &v25[1]) {
      sub_19012(dat_2b410,(int)*v25,1);
    }
  }
  dat_2b418 = sub_18f74(NULL);
  sub_19012(dat_2b418,0x3a,1);
  dat_2b3cc = ((dat_2b3d5 ^ 1) & dat_2b3ac == 0 & dat_2b3cc) != 0;
  if ((int)dat_2b030 < (int)(unsigned int)dat_2b3cc) {
    error(2,0,gettext("--dired and --zero are incompatible"));
    return v8;
  }
  if (v15 <= -1) {
    if ((dat_2b3ac) && (((dat_2b3b0 == 1 || (dat_2b3b0 == 2)) || (dat_2b3b0 == 3))))
      v15 = 5;
    else {
      v15 = 0;
    }
  }
  dat_2b3b4 = v15;
  if (!dat_2b3ac) {
    v26 = v21;
    if (!v21) {
      v26 = getenv("TIME_STYLE");
      if (!v26)
        v26 = (char *)sub_523e("locale");
    }
    while (!strncmp(v26,"posix-",6)) {
      if (sub_11f2c(2) != '\x01') {
        return (unsigned long)optind; // return-dupe
      }
      v26 = &v26[6];
    }
    if (*v26 != '+') { // branch-flip
      v10 = sub_fa7e(v26,0x29f60,0x205b0,4);
      if (v10 <= -1) {
        sub_fc27("time style",v26,v10);
        v2 = stderr;
        fputs_unlocked(gettext("Valid arguments are:\n"),v2);
        v28 = (long *)0x29f60;
        while (v2 = stderr, *v28) {
          v5 = &v28[1];
          fprintf(stderr,"  - [posix-]%s\n",(char *)*v28);
          v28 = v5;
        }
        fputs_unlocked(gettext("  - +FORMAT (e.g., +%H:%M) for a \'date\'-style format\n"),v2);
        sub_f135(2); // no-return
      }
      switch(v10) { // branch-flip
        case 0:
          dat_2b208 = "%Y-%m-%d %H:%M:%S.%N %z";
          dat_2b200 = "%Y-%m-%d %H:%M:%S.%N %z";
          break;
        case 1:
          dat_2b208 = "%Y-%m-%d %H:%M";
          dat_2b200 = "%Y-%m-%d %H:%M";
          break;
        case 2:
          dat_2b200 = "%Y-%m-%d ";
          dat_2b208 = "%m-%d %H:%M";
          break;
        case 3:
          if (sub_11f2c(2)) {
            for (v20 = 0; v20 <= 1; v20 = v20 + 1) {
              v7 = *(char **)((long)v20 * 8 + 0x2b200);
              *(char **)((long)v20 * 8 + 0x2b200) = dcgettext(NULL,v7,2);
            }
          }
        
      }
    }
    else {
      v26 = &v26[1];
      v7 = strchr(v26,10);
      if (v7) { // branch-flip
        if (strchr(&v7[1],10)) {
          v7 = (char *)sub_1aeca(v26);
          error(2,0,gettext("invalid time style format %s"),v7);
          return v9;
        }
        v27 = &v7[1];
        *v7 = '\0';
      }
      else {
        v27 = v26;
      }
      dat_2b208 = v27;
      dat_2b200 = v26;
    }
    sub_5f43();
  }
  return (unsigned long)optind;
label_7a55:
  dat_2b030 = '\0';
  v13 = 0;
  if (v12)
    v12 = 1;
  dat_2b3d4 = '\0';
  v14 = 0;
  goto label_7128;
}

// Function: sub_842f @ 0x842f
unsigned long sub_842f(void)
{
  unsigned long v1; // rax
  
  v1 = (*(void *)((long)dat_211d8 + 0x211d8))(); // jump-as-call
  return v1;
}

// Function: sub_87ec @ 0x87ec
unsigned long sub_87ec(void) // return-dupe
{
  char *v1; // rax
  char *v2; // stack - 0x18
  
  v1 = getenv("TERM");
  if ((v1) && (*v1)) {
    v2 = "# Configuration file for dircolors, a utility to help you set the";
    while( true ) {
      if ((char *)0x12c8 <= &v2[-0x1f2c0])
        return 0;
      if ((!strncmp(v2,"TERM ",5)) && (!fnmatch(&v2[5],v1,0))) break;
      v2 = &v2[strlen(v2) + 1];
    }
    return 1;
  }
  return 0;
}

// Function: sub_88ab @ 0x88ab
void sub_88ab(void)
{
  char *v1; // rax
  char v2 [3];
  
  v1 = getenv("LS_COLORS");
  if ((v1) && (*v1)) {
    strcpy(v2,"??");
    dat_2b3e0 = sub_1cc92(v1);
    (*(void *)((long)dat_21390 + 0x2138c))(); // jump-as-call
    return;
  }
  v1 = getenv("COLORTERM");
  if (((!v1) || (!*v1)) && (sub_87ec() != '\x01'))
    dat_2b3d4 = 0;
}

// Function: sub_8cff @ 0x8cff
unsigned int sub_8cff(void) // early-return x2
{
  int v1; // eax
  char *v2; // rax
  
  v2 = getenv("QUOTING_STYLE");
  if (!v2)
    return 0xffffffff;
  v1 = sub_fa7e(v2,0x2a9e0,0x238c0,4);
  if (v1 <= -1) {
    v2 = (char *)sub_1aeca(v2);
    error(0,0,gettext("ignoring invalid value of environment variable QUOTING_STYLE: %s"),v2);
    return 0xffffffff;
  }
  return *(unsigned int *)((long)v1 * 4 + 0x238c0);
}

// Function: sub_8db1 @ 0x8db1
void sub_8db1(bool a0) // return-dupe
{
  if (a0) {
    dat_2b4e8 = 2;
    return;
  }
  if (!dat_2b4e8) {
    dat_2b4e8 = 1;
    return;
  }
}

// Function: sub_8de7 @ 0x8de7
void sub_8de7(char a0,char *a1,char *a2)
{
  unsigned long v1; // rax
  
  v1 = sub_1aa83(4,a2);
  error(0,*__errno_location(),a1,v1);
  sub_8db1((unsigned char)a0);
}

// Function: sub_8e46 @ 0x8e46
void sub_8e46(char *a0,char *a1,char a2)
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  
  v1 = (unsigned long *)sub_1c68b(0x20);
  if (a1) // branch-flip
    v2 = sub_1cc92(a1);
  else {
    v2 = 0;
  }
  v1[1] = v2;
  if (a0) // branch-flip
    v2 = sub_1cc92(a0);
  else {
    v2 = 0;
  }
  *v1 = v2;
  *(char *)&v1[2] = a2;
  v1[3] = dat_2b368;
  dat_2b368 = v1;
}

// Function: sub_8ede @ 0x8ede
void sub_8ede(char *a0,char *a1,char a2) // return-dupe
{
  char v1;
  unsigned long v10; // stack - 0x378
  char *v11; // stack - 0x370
  DIR *v12; // stack - 0x368
  unsigned long v13; // stack - 0x340
  int v2; // eax
  char *v3;
  dirent *v4; // rax
  char *v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // stack - 0x348
  char v8 [663];
  unsigned int v9; // stack - 0x380
  
  v10 = 0;
  *__errno_location() = 0;
  v12 = opendir(a0);
  if (!v12) {
    sub_8de7((unsigned char)a2,gettext("cannot open directory %s"),a0);
    return;
  }
  if (dat_2b320) {
    v2 = dirfd(v12);
    if (0 <= v2) { // branch-flip
      sub_5cee(v2,&v7);
      v1 = dat_3;
    }
    else {
      sub_5cb8(a0,&v7);
      v1 = dat_3;
    }
    if (v1 <= '\xff') {
      sub_8de7((unsigned char)a2,gettext("cannot determine device and inode of %s"),a0);
      closedir(v12);
      return;
    }
    v1 = sub_6262(v7,v13);
    if (v1) {
      v3 = (char *)sub_1ac15(0,3,a0);
      error(0,0,gettext("%s: not listing already-listed directory"),v3);
      closedir(v12);
      sub_8db1(1);
      return;
    }
    sub_558e(v7,v13);
  }
  sub_96f3();
  if ((dat_2b3f0) || (dat_2b428)) {
    if (dat_2b032 != '\x01')
      sub_5422(10);
    dat_2b032 = '\0';
    sub_54cf();
    v11 = NULL;
    if (dat_2b3d5) {
      v11 = (char *)sub_1101b(a0,2);
      if (!v11)
        sub_8de7((unsigned char)a2,gettext("error canonicalizing %s"),a0);
    }
    if (a1) { // branch-flip
    }
    else {
      a1 = a0;
    }
    sub_d87f(a1,dat_2b418,0xffffffff,NULL,1,(obstack *)0x2b580,v11);
    free(v11);
    sub_549e(":\n");
  }
  do {
    *__errno_location() = 0;
    v4 = readdir(v12);
    if (v4) { // branch-flip
      if (sub_958d(&v4->field_0x13) != '\x01') {
        v9 = 0;
        switch(v4->field_0x12) {
          case 1:
            v9 = 1;
            break;
          case 2:
            v9 = 2;
            break;
          case 4:
            v9 = 3;
            break;
          case 6:
            v9 = 4;
            break;
          case 8:
            v9 = 5;
            break;
          case 10:
            v9 = 6;
            break;
          case 0xc:
            v9 = 7;
            break;
          case 0xe:
            v9 = 8;
          
        }
        v10 += sub_9a40(&v4->field_0x13,v9,0,0,a0);
        if ((((dat_2b3ac == 1) && (dat_2b3b4 == 6)) && (dat_2b3bb != '\x01')) && (dat_2b3f0 != '\x01')) {
          sub_bfc5();
          sub_c12d();
          sub_96f3();
        }
      }
    }
    else {
      if (!*__errno_location()) {
label_9382:
        if (closedir(v12))
          sub_8de7((unsigned char)a2,gettext("closing directory %s"),a0);
        sub_bfc5();
        if (dat_2b3f0)
          sub_aa65(a0,0);
        if ((!dat_2b3ac) || (dat_2b3bb)) {
          v5 = (char *)sub_13d91(v10,v8,dat_2b3bc,0x200,dat_2b3c0);
          v6 = strlen(v5);
          v3 = &v5[-1];
          *v3 = ' ';
          v5[v6] = dat_2b030;
          sub_54cf();
          sub_549e(gettext("total"));
          sub_5453(v3,&(&v5[v6])[1 - (long)v3]);
        }
        if (!dat_2b338)
          return;
        sub_c12d();
        return;
      }
      sub_8de7((unsigned char)a2,gettext("reading directory %s"),a0);
      if (*__errno_location() != 0x4b) goto label_9382;
    }
    sub_648c();
  } while( true );
}

// Function: sub_94e9 @ 0x94e9
void sub_94e9(unsigned long a0)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_1c68b(0x10);
  *v1 = a0;
  v1[1] = dat_2b3f8;
  dat_2b3f8 = v1;
}

// Function: sub_952f @ 0x952f
unsigned long sub_952f(unsigned long *a0,char *a1)
{
  char *v1;
  unsigned long *v2; // stack - 0x10
  
  v2 = a0;
  while( true ) {
    if (!v2)
      return 0;
    v1 = (char *)*v2;
    if (!fnmatch(v1,a1,4)) break;
    v2 = (unsigned long *)v2[1];
  }
  return 1;
}

// Function: sub_958d @ 0x958d
unsigned int sub_958d(char *a0) // ternary
{
  char v1; // al
  long v2; // rdx
  
  if ((dat_2b3f4 != 2) && (*a0 == '.')) {
    if (!dat_2b3f4)
      return 1;
    v2 = (a0[1] != '.') ? 1 : 2; // branch-flip
    if (!a0[v2])
      return 1;
  }
  if (((dat_2b3f4) || (v1 = sub_952f(dat_2b400,a0), !v1)) && (v1 = sub_952f(dat_2b3f8,a0), !v1))
    return 0;
  return 1;
}

// Function: sub_9635 @ 0x9635
unsigned long sub_9635(unsigned long a0)
{
  return a0;
}

// Function: sub_9647 @ 0x9647
unsigned long sub_9647(void)
{
  *__errno_location() = 0x5f;
  return 0;
}

// Function: sub_9669 @ 0x9669
void sub_9669(struct_13 *a0) // return-dupe x2
{
  free(a0->field_0x0);
  free(a0->field_0x8);
  free(a0->field_0x10);
  if (a0->field_0xb0 == 0x2b020)
    return;
  if (sub_525c()) {
    free((void *)a0->field_0xb0);
    return;
  }
  freecon((char *)a0->field_0xb0);
}

// Function: sub_96f3 @ 0x96f3
void sub_96f3(void)
{
  unsigned long v1; // stack - 0x18
  
  for (v1 = 0; v1 < dat_2b338; v1 = v1 + 1) {
    sub_9669(*(struct_13 **)(v1 * 8 + dat_2b348));
  }
  dat_2b338 = 0;
  dat_2b340 = 0;
  dat_2b381 = 0;
  dat_2b384 = 0;
  dat_2b388 = 0;
  dat_2b38c = 0;
  dat_2b394 = 0;
  dat_2b398 = 0;
  dat_2b39c = 0;
  dat_2b390 = 0;
  dat_2b3a0 = 0;
  dat_2b3a4 = 0;
  dat_2b3a8 = 0;
}

// Function: sub_97c0 @ 0x97c0
unsigned int sub_97c0(int a0)
{
  if (((a0 != 0x16) && (a0 != 0x26)) && (!sub_524c(a0)))
    return 0;
  return 1;
}

// Function: sub_97fa @ 0x97fa
int sub_97fa(char *a0,struct_11 *a1,bool a2) // early-return
{
  int v1;
  int v2; // eax
  
  if (a1->field_0x18 == dat_2c428) {
    *__errno_location() = 0x5f;
    return -1;
  }
  if (a2) // branch-flip
    v2 = sub_1b22b(a0,&a1[5].field_0x0[0x10]);
  else {
    v2 = sub_1b268(a0,&a1[5].field_0x0[0x10]);
  }
  if (v2 <= -1) {
    v1 = *__errno_location();
    if (sub_97c0(v1))
      dat_2c428 = a1->field_0x18;
  }
  return v2;
}

// Function: sub_98ab @ 0x98ab
int sub_98ab(char *a0,struct_11 *a1) // early-return
{
  int v1;
  int v2; // eax
  
  if (a1->field_0x18 == dat_2c430) {
    *__errno_location() = 0x5f;
    return 0;
  }
  *__errno_location() = 0;
  v2 = sub_112c6(a0,&a1->field_0x18);
  if (v2 <= 0) {
    v1 = *__errno_location();
    if (sub_97c0(v1))
      dat_2c430 = a1->field_0x18;
  }
  return v2;
}

// Function: sub_9936 @ 0x9936
char sub_9936(char *a0,struct_11 *a1) // early-return
{
  int v1;
  char v2; // al
  
  if (a1->field_0x18 == dat_2c438) {
    *__errno_location() = 0x5f;
    return '\0';
  }
  v2 = sub_9647(a0);
  if (v2 != '\x01') {
    v1 = *__errno_location();
    if (sub_97c0(v1))
      dat_2c438 = a1->field_0x18;
  }
  return v2;
}

// Function: sub_99b1 @ 0x99b1
unsigned int sub_99b1(char *a0) // return-dupe
{
  unsigned long v1; // rax
  char v2 [2]; // stack - 0x12
  
  v1 = sub_1a389(v2,2,a0,0xffffffffffffffff,dat_2b410);
  if ((*a0 == v2[0]) && (v1 == strlen(a0)))
    return 0;
  return 1;
}

// Function: sub_9a40 @ 0x9a40
unsigned long sub_9a40(char *a0,int a1,long a2,char a3,char *a4)
{
  long v1;
  char *v10; // stack - 0x3e8
  char v11 [24];
  char v12 [664];
  char v13 [32];
  char **v14; // rsp
  char **v15;
  long v16; // stack - 0x3e0
  char v17; // stack - 0x3d8
  int v18; // stack - 0x3d4
  char *v19; // stack - 0x3d0
  unsigned char v2; // al
  char v20; // stack - 0x3c4
  char v21; // stack - 0x3c3
  char v22; // stack - 0x3c2
  char v23; // stack - 0x3c1
  unsigned int v24; // stack - 0x3c0
  int v25; // stack - 0x3bc
  unsigned int v26; // stack - 0x3b8
  int v27; // stack - 0x3b4
  int v28; // stack - 0x3b0
  int v29; // stack - 0x3ac
  char v3; // al
  int v30; // stack - 0x3a8
  int v31; // stack - 0x3a4
  int v32; // stack - 0x3a0
  int v33; // stack - 0x39c
  int v34; // stack - 0x398
  int v35; // stack - 0x394
  unsigned long v36; // stack - 0x390
  char *v37; // stack - 0x388
  unsigned long *v38; // stack - 0x380
  void *v39; // stack - 0x378
  int v4;
  unsigned long v40; // stack - 0x370
  unsigned int v41; // stack - 0x350
  unsigned int v5; // eax
  unsigned int v6;
  unsigned long v7;
  unsigned long v8;
  char *v9;
  
  v14 = &v10;
  v36 = 0;
  v10 = a4;
  v16 = a2;
  v17 = a3;
  v18 = a1;
  v19 = a0;
  if ((a3 == '\x01') && (a2))
    __assert_fail("! command_line_arg || inode == NOT_AN_INODE_NUMBER","src/ls.c",0xd07,"gobble_file"); // no-return
  if (dat_2b338 == dat_2b330) {
    dat_2b328 = sub_1c638(dat_2b328,dat_2b330,0x1a0);
    dat_2b330 *= 2;
  }
  v38 = (unsigned long *)(dat_2b338 * 0xd0 + dat_2b328);
  memset(v38,0,0xd0);
  v38[4] = v16;
  *(int *)&v38[0x15] = v18;
  *(unsigned int *)((long)v38 + 0xc4) = 0xffffffff;
  if ((dat_2b340 != '\x01') && (dat_2b341)) {
    v2 = sub_99b1(v19);
    *(unsigned int *)((long)v38 + 0xc4) = (unsigned int)v2;
    if (*(int *)((long)v38 + 0xc4))
      dat_2b340 = '\x01';
  }
  if (((((((!v17) && (!dat_2b3d5)) && (!dat_2b440)) && (((v18 != 3 || (!dat_2b3d4)) || ((!sub_6325(0x13) && ((!sub_6325(0x12) && (!sub_6325(0x14))))))))) && (((!dat_2b3e9 && (!dat_2b441)) || (((v18 != 6 && (v18)) || ((dat_2b3ec != 4 && ((!dat_2b358 && (!dat_2b3e8)))))))))) && ((!dat_2b3e9 || (v16)))) && ((v15 = &v10, !dat_2b441 || (((v18 && (!v17)) && ((v15 = &v10, v18 != 5 || ((dat_2b3d0 != 3 && ((v15 = &v10, !dat_2b3d4 || ((((!sub_6325(0xe) && (!sub_6325(0x10))) && (!sub_6325(0x11))) && (v15 = &v10, !sub_6325(0x15))))))))))))))) goto label_a7ca;
  if ((*v19 != '/') && (*v10)) {
    v7 = strlen(v19);
    v7 = ((strlen(v10) + v7 + 0x19) / 0x10) * 0x10;
    for (; v14 != (char **)((long)&v10 - (v7 & 0xfffffffffffff000)); v14 = (char *)((long)v14 + -0x1000)) {
      *(unsigned long *)((long)v14 + -8) = *(unsigned long *)((long)v14 + -8);
    }
    v1 = -(unsigned long)((unsigned int)v7 & 0xfff);
    v15 = (char *)((long)v14 + v1);
    if (v7 & 0xfff)
      *(unsigned long *)((long)v14 + ((unsigned long)((unsigned int)v7 & 0xfff) - 8) + v1) = *(unsigned long *)((long)v14 + ((unsigned long)((unsigned int)v7 & 0xfff) - 8) + v1);
    *(unsigned long *)((long)v14 + v1 + -8) = 0x9e47;
    v37 = (char *)((unsigned long)((long)v14 + v1 + 0xf) & 0xfffffffffffffff0);
    sub_eb94((char *)((unsigned long)((long)v14 + v1 + 0xf) & 0xfffffffffffffff0),v10,v19);
  }
  else {
    v37 = v19;
    v15 = &v10;
  }
  if (dat_2b3d5) {
    *(unsigned long *)((long)v15 + -8) = 0x9e66;
    v8 = sub_1101b(v37,2);
    v38[2] = v8;
    if (!v38[2]) {
      *(unsigned long *)((long)v15 + -8) = 0x9e93;
      v9 = gettext("error canonicalizing %s");
      *(unsigned long *)((long)v15 + -8) = 0x9eae;
      sub_8de7((unsigned char)v17,v9,v37);
    }
  }
  if (4 <= dat_2b3ec) { // branch-flip
    if (dat_2b3ec == 4) {
      *(unsigned long *)((long)v15 + -8) = 0x9ee9;
      v24 = sub_5c0e(v37,(struct_4 *)&v38[3]);
      v20 = 1;
      goto label_9fac;
    }
label_9f7c:
    *(unsigned long *)((long)v15 + -8) = 0x9f99;
    v24 = sub_5c48(v37,(struct_4 *)&v38[3]);
    v20 = 0;
  }
  else {
    if ((dat_2b3ec <= 1) || (!v17)) goto label_9f7c;
    *(unsigned long *)((long)v15 + -8) = 0x9f21;
    v24 = sub_5c0e(v37,(struct_4 *)&v38[3]);
    v20 = 1;
    if (dat_2b3ec != 2) {
      if (0 <= (int)v24) // branch-flip
        v23 = (*(unsigned int *)&v38[6] & 0xf000) != 0x4000;
      else {
        *(unsigned long *)((long)v15 + -8) = 0x9f47;
        v23 = *__errno_location() == 2;
      }
      if ((bool)v23 == 1) goto label_9f7c;
    }
  }
label_9fac:
  if (v24) {
    *(unsigned long *)((long)v15 + -8) = 0x9fc8;
    v9 = gettext("cannot access %s");
    *(unsigned long *)((long)v15 + -8) = 0x9fe3;
    sub_8de7((unsigned char)v17,v9,v37);
    v38[0x16] = 0x2b020;
    if (v17)
      return 0;
    *(unsigned long *)((long)v15 + -8) = 0xa01a;
    *v38 = sub_1cc92(v19);
    dat_2b338 += 1;
    return 0;
  }
  *(char *)&v38[0x17] = 1;
  if (((v18 == 5) || ((*(unsigned int *)&v38[6] & 0xf000) == 0x8000)) && (dat_2b3d4)) {
    *(unsigned long *)((long)v15 + -8) = 0xa085;
    if (sub_6325(0x15)) {
      *(unsigned long *)((long)v15 + -8) = 0xa0a2;
      v3 = sub_9936(v37,v38);
      *(char *)&v38[0x18] = v3;
    }
  }
  if ((!dat_2b3ac) || (dat_2b380)) {
    v21 = '\0';
    v22 = '\0';
    *(unsigned long *)((long)v15 + -8) = 0xa0f6;
    v4 = sub_97fa(v37,v38,(unsigned char)v20);
    v25 = v4;
    v5 = -(v4 >> 0x1f);
    v24 = v5;
    if (v5) { // branch-flip
      v38[0x16] = 0x2b020;
      *(unsigned long *)((long)v15 + -8) = 0xa194;
      v4 = *__errno_location();
      *(unsigned long *)((long)v15 + -8) = 0xa19d;
      if (!sub_524c(v4)) {
        *(unsigned long *)((long)v15 + -8) = 0xa1a6;
        if (*__errno_location() != 0x3d) goto label_a1b7;
      }
      v24 = 0;
    }
    else {
      *(unsigned long *)((long)v15 + -8) = 0xa11c;
      if (sub_525c()) { // branch-flip
        v9 = (char *)v38[0x16];
        *(unsigned long *)((long)v15 + -8) = 0xa140;
        v21 = strcmp("_",v9) != 0;
      }
      else {
        v9 = (char *)v38[0x16];
        *(unsigned long *)((long)v15 + -8) = 0xa16d;
        v21 = strcmp("unlabeled",v9) != 0;
      }
    }
label_a1b7:
    if ((!v24) && (!dat_2b3ac)) {
      *(unsigned long *)((long)v15 + -8) = 0xa1e3;
      v26 = sub_98ab(v37,v38);
      v24 = v26 >> 0x1f;
      v22 = 0 < (int)v26;
    }
    if ((v21) || (v22)) {
      if ((v21) && (v22 != '\x01'))
        v6 = 1;
      else {
        v6 = 2;
      }
    }
    else {
      v6 = 0;
    }
    *(unsigned int *)((long)v38 + 0xbc) = v6;
    dat_2b381 = *(int *)((long)v38 + 0xbc) || dat_2b381;
    if (v24) {
      *(unsigned long *)((long)v15 + -8) = 0xa2a2;
      v9 = (char *)sub_1ac15(0,3,v37);
      *(unsigned long *)((long)v15 + -8) = 0xa2aa;
      v4 = *__errno_location();
      *(unsigned long *)((long)v15 + -8) = 0xa2c7;
      error(0,v4,"%s",v9);
    }
  }
  if (((*(unsigned int *)&v38[6] & 0xf000) == 0xa000) && ((!dat_2b3ac || (dat_2b3e8)))) {
    *(unsigned long *)((long)v15 + -8) = 0xa31a;
    sub_a8f5(v37,v38,(unsigned char)v17);
    v9 = (char *)v38[1];
    *(unsigned long *)((long)v15 + -8) = 0xa337;
    v39 = (void *)sub_a964(v37,v9);
    if ((v39) && (!*(int *)((long)v38 + 0xc4))) {
      v9 = (char *)v38[1];
      *(unsigned long *)((long)v15 + -8) = 0xa36c;
      if (sub_99b1(v9))
        *(unsigned int *)((long)v38 + 0xc4) = 0xffffffff;
    }
    if ((v39) && ((2 <= dat_2b3d0 || (dat_2b3e8)))) {
      *(unsigned long *)((long)v15 + -8) = 0xa3ba;
      v4 = sub_5c82(v39,v11);
      if (!v4) {
        *(char *)((long)v38 + 0xb9) = 1;
        *(unsigned int *)((long)v38 + 0xac) = v41;
      }
    }
    *(unsigned long *)((long)v15 + -8) = 0xa3ee;
    free(v39);
  }
  if ((*(unsigned int *)&v38[6] & 0xf000) != 0xa000) { // branch-flip
    if ((*(unsigned int *)&v38[6] & 0xf000) != 0x4000) // branch-flip
      *(unsigned int *)&v38[0x15] = 5;
    else if ((v17) && (dat_2b3f1 != '\x01'))
      *(unsigned int *)&v38[0x15] = 9;
    else {
      *(unsigned int *)&v38[0x15] = 3;
    }
  }
  else {
    *(unsigned int *)&v38[0x15] = 6;
  }
  v8 = v38[0xb];
  v36 = v8;
  if ((!dat_2b3ac) || (dat_2b3bb)) {
    *(unsigned long *)((long)v15 + -8) = 0xa4cd;
    v9 = (char *)sub_13d91(v8,v12,dat_2b3bc,0x200,dat_2b3c0);
    *(unsigned long *)((long)v15 + -8) = 0xa4da;
    v27 = sub_1574e(v9,0);
    if (dat_2b388 < v27)
      dat_2b388 = v27;
  }
  if (!dat_2b3ac) {
    if (dat_2b022) {
      v6 = *(unsigned int *)((long)v38 + 0x34);
      *(unsigned long *)((long)v15 + -8) = 0xa524;
      v28 = sub_c5c3(v6);
      if (dat_2b394 < v28)
        dat_2b394 = v28;
    }
    if (dat_2b023) {
      v6 = *(unsigned int *)&v38[7];
      *(unsigned long *)((long)v15 + -8) = 0xa560;
      v29 = sub_c603(v6);
      if (dat_2b398 < v29)
        dat_2b398 = v29;
    }
    if (dat_2b3b9) {
      v6 = *(unsigned int *)((long)v38 + 0x34);
      *(unsigned long *)((long)v15 + -8) = 0xa59c;
      v30 = sub_c5c3(v6);
      if (dat_2b39c < v30)
        dat_2b39c = v30;
    }
  }
  if (dat_2b380) {
    v9 = (char *)v38[0x16];
    *(unsigned long *)((long)v15 + -8) = 0xa5dd;
    v31 = (int)strlen(v9);
    if (dat_2b390 < v31)
      dat_2b390 = v31;
  }
  if (!dat_2b3ac) {
    v7 = v38[5];
    *(unsigned long *)((long)v15 + -8) = 0xa628;
    v9 = (char *)sub_15035(v7,v13);
    *(unsigned long *)((long)v15 + -8) = 0xa630;
    v4 = (int)strlen(v9);
    if (dat_2b38c < v4)
      dat_2b38c = v4;
    v32 = v4;
    if (((*(unsigned int *)&v38[6] & 0xf000) != 0x2000) && ((*(unsigned int *)&v38[6] & 0xf000) != 0x6000)) {
      v8 = v38[9];
      *(unsigned long *)((long)v15 + -8) = 0xa768;
      v8 = sub_9635(v8);
      v40 = v8;
      *(unsigned long *)((long)v15 + -8) = 0xa79a;
      v9 = (char *)sub_13d91(v8,v12,dat_2b3c8,1,dat_2b028);
      *(unsigned long *)((long)v15 + -8) = 0xa7a7;
      v33 = sub_1574e(v9,0);
      if (dat_2b3a8 < v33)
        dat_2b3a8 = v33;
    }
    else {
      v8 = v38[8];
      *(unsigned long *)((long)v15 + -8) = 0xa693;
      v6 = gnu_dev_major(v8);
      *(unsigned long *)((long)v15 + -8) = 0xa6a7;
      v9 = (char *)sub_15035(v6,v12);
      *(unsigned long *)((long)v15 + -8) = 0xa6af;
      v34 = (int)strlen(v9);
      if (dat_2b3a0 < v34)
        dat_2b3a0 = v34;
      v8 = v38[8];
      *(unsigned long *)((long)v15 + -8) = 0xa6e2;
      v6 = gnu_dev_minor(v8);
      *(unsigned long *)((long)v15 + -8) = 0xa6f6;
      v9 = (char *)sub_15035(v6,v12);
      *(unsigned long *)((long)v15 + -8) = 0xa6fe;
      v4 = (int)strlen(v9);
      if (dat_2b3a4 < v4)
        dat_2b3a4 = v4;
      v34 = dat_2b3a4 + dat_2b3a0 + 2;
      if (dat_2b3a8 < v34)
        dat_2b3a8 = v34;
    }
  }
label_a7ca:
  if (dat_2b3e9) {
    v7 = v38[4];
    *(unsigned long *)((long)v15 + -8) = 0xa7f2;
    v9 = (char *)sub_15035(v7,v12);
    *(unsigned long *)((long)v15 + -8) = 0xa7fa;
    v35 = (int)strlen(v9);
    if (dat_2b384 < v35)
      dat_2b384 = v35;
  }
  *(unsigned long *)((long)v15 + -8) = 0xa829;
  *v38 = sub_1cc92(v19);
  dat_2b338 += 1;
  return v36;
}

// Function: sub_a869 @ 0xa869
unsigned int sub_a869(struct_17 *a0) // return-dupe
{
  if ((a0->field_0xa8 != 3) && (a0->field_0xa8 != 9))
    return 0;
  return 1;
}

// Function: sub_a8a4 @ 0xa8a4
unsigned int sub_a8a4(char *a0) // return-dupe
{
  if (((*(int *)&a0[0xa8] != 3) && (*(int *)&a0[0xa8] != 9)) && ((*(unsigned int *)&a0[0xac] & 0xf000) != 0x4000))
    return 0;
  return 1;
}

// Function: sub_a8f5 @ 0xa8f5
void sub_a8f5(char *a0,char *a1,char a2) // return-dupe
{
  unsigned long v1;
  
  v1 = *(unsigned long *)&a1[0x48];
  *(unsigned long *)&a1[8] = sub_f7a0(a0,v1);
  if (*(long *)&a1[8])
    return;
  sub_8de7((unsigned char)a2,gettext("cannot read symbolic link %s"),a0);
}

// Function: sub_a964 @ 0xa964
char * sub_a964(char *a0,char *a1) // early-return
{
  char *v1; // rax
  unsigned long v2; // stack - 0x18
  
  if (!a1)
    return NULL;
  if (*a1 != '/') { // branch-flip
    v2 = sub_111a6(a0);
    if (v2) { // branch-flip
      v1 = (char *)sub_1c68b(strlen(a1) + v2 + 2);
      if (a0[v2 - 1] != '/')
        v2 += 1;
      stpcpy(stpncpy(v1,a0,v2),a1);
    }
    else {
      v1 = (char *)sub_1cc92(a1);
    }
  }
  else {
    v1 = (char *)sub_1cc92(a1);
  }
  return v1;
}

// Function: sub_aa37 @ 0xaa37
void sub_aa37(char *a0)
{
  sub_4ec1((char *)sub_ff56(a0));
}

// Function: sub_aa65 @ 0xaa65
void sub_aa65(char *a0,char a1)
{
  unsigned long v1;
  unsigned long *v2;
  long v3;
  char *v4;
  unsigned long v5; // stack - 0x30
  long v6; // stack - 0x28
  
  if ((a0) && (dat_2b320))
    sub_8e46(NULL,a0,0);
  v5 = dat_2b338;
label_aba6:
  do {
    v1 = v5 - 1;
    if (!v5) {
      v6 = 0;
      for (v5 = 0; v5 < dat_2b338; v5 = v5 + 1) {
        v3 = *(long *)(v5 * 8 + dat_2b348);
        *(long *)(dat_2b348 + v6 * 8) = v3;
        v6 += (unsigned long)(*(int *)(v3 + 0xa8) != 9);
      }
      dat_2b338 = v6;
      return;
    }
    v2 = *(unsigned long **)(v1 * 8 + dat_2b348);
    v5 = v1;
  } while (!sub_a869(v2));
  if (a0) goto label_aaf6;
  goto label_ab10;
label_aaf6:
  v4 = (char *)*v2;
  if (sub_aa37(v4) != '\x01') {
label_ab10:
    if ((a0) && (*(char *)*v2 != '/')) {
      v4 = (char *)sub_116a7(a0,(char *)*v2,NULL);
      sub_8e46(v4,(char *)v2[1],(unsigned char)a1);
      free(v4);
    }
    else {
      sub_8e46((char *)*v2,(char *)v2[1],(unsigned char)a1);
    }
    if (*(int *)&v2[0x15] == 9)
      sub_9669(v2);
  }
  goto label_aba6;
}

// Function: sub_ac36 @ 0xac36
int sub_ac36(char *a0,char *a1)
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // rax
  char *v4; // rax
  
  *__errno_location() = 0;
  v1 = strcoll(a0,a1);
  if (*__errno_location()) {
    v2 = sub_1ae9d(1,a1);
    v3 = sub_1ae9d(0,a0);
    v4 = gettext("cannot compare file names %s and %s");
    error(0,*__errno_location(),v4,v3,v2);
    sub_8db1(0);
    longjmp((void *)0x2c360,1); // no-return
  }
  return v1;
}

// Function: sub_ad02 @ 0xad02
unsigned long sub_ad02(char *a0,char *a1,void *a2)
{
  unsigned char v1; // al
  unsigned long v2; // rax
  unsigned int v3; // edx
  unsigned int v4; // ebx
  
  v1 = sub_a8a4(a1);
  v4 = (unsigned int)v1;
  v1 = sub_a8a4(a0);
  v3 = (unsigned int)v1;
  if (v4 - v3) // branch-flip
    v2 = (unsigned long)(v4 - v3);
  else {
    v2 = (*a2)(a0,a1);
  }
  return v2;
}

// Function: sub_ad65 @ 0xad65
unsigned long * sub_ad65(unsigned long *a0,unsigned long *a1,void *a2) // ternary
{
  long v1;
  unsigned int v2; // eax
  unsigned long *v3; // rax
  char v4 [16];
  char v5 [16];
  
  v3 = &a0[3];
  v4 = sub_1b477(v3);
  v3 = SUB168(v4,0);
  v1 = (long)v3;
  v3 = &a1[3];
  v5 = sub_1b477(v3);
  v3 = SUB168(v5,0);
  v2 = sub_1bc1f(v3,SUB168(v5,8),v1,SUB168(v4,8));
  v3 = (v2) ? (unsigned long)(unsigned long)v2 : (unsigned long)(*a2)(*a0,*a1); // branch-flip
  return v3;
}

// Function: sub_adf8 @ 0xadf8
unsigned long * sub_adf8(unsigned long *a0,unsigned long *a1,void *a2) // ternary
{
  long v1;
  unsigned int v2; // eax
  unsigned long *v3; // rax
  char v4 [16];
  char v5 [16];
  
  v3 = &a0[3];
  v4 = sub_1b491(v3);
  v3 = SUB168(v4,0);
  v1 = (long)v3;
  v3 = &a1[3];
  v5 = sub_1b491(v3);
  v3 = SUB168(v5,0);
  v2 = sub_1bc1f(v3,SUB168(v5,8),v1,SUB168(v4,8));
  v3 = (v2) ? (unsigned long)(unsigned long)v2 : (unsigned long)(*a2)(*a0,*a1); // branch-flip
  return v3;
}

// Function: sub_ae8b @ 0xae8b
unsigned long * sub_ae8b(unsigned long *a0,unsigned long *a1,void *a2) // ternary
{
  long v1;
  unsigned int v2; // eax
  unsigned long *v3; // rax
  char v4 [16];
  char v5 [16];
  
  v3 = &a0[3];
  v4 = sub_1b45d(v3);
  v3 = SUB168(v4,0);
  v1 = (long)v3;
  v3 = &a1[3];
  v5 = sub_1b45d(v3);
  v3 = SUB168(v5,0);
  v2 = sub_1bc1f(v3,SUB168(v5,8),v1,SUB168(v4,8));
  v3 = (v2) ? (unsigned long)(unsigned long)v2 : (unsigned long)(*a2)(*a0,*a1); // branch-flip
  return v3;
}

// Function: sub_af1e @ 0xaf1e
unsigned long * sub_af1e(unsigned long *a0,unsigned long *a1,void *a2) // ternary
{
  long v1;
  unsigned int v2; // eax
  unsigned long *v3; // rax
  char v4 [16];
  char v5 [16];
  
  v3 = &a0[3];
  v4 = sub_5990(v3);
  v3 = SUB168(v4,0);
  v1 = (long)v3;
  v3 = &a1[3];
  v5 = sub_5990(v3);
  v3 = SUB168(v5,0);
  v2 = sub_1bc1f(v3,SUB168(v5,8),v1,SUB168(v4,8));
  v3 = (v2) ? (unsigned long)(unsigned long)v2 : (unsigned long)(*a2)(*a0,*a1); // branch-flip
  return v3;
}

// Function: sub_afb1 @ 0xafb1
unsigned long sub_afb1(long a0,long a1) // early-return
{
  if (a1 <= a0)
    return (unsigned long)(a1 < a0);
  return 0xffffffff;
}

// Function: sub_afe2 @ 0xafe2
unsigned long sub_afe2(struct_21 *a0,struct_21 *a1,void *a2)
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  
  v1 = sub_afb1(a1->field_0x48,a0->field_0x48);
  if (v1) // branch-flip
    v2 = (unsigned long)v1;
  else {
    v2 = (*a2)(a0->field_0x0,a1->field_0x0);
  }
  return v2;
}

// Function: sub_b03f @ 0xb03f
void sub_b03f(unsigned long *a0,unsigned long *a1,void *a2)
{
  (*a2)(*a0,*a1);
}

// Function: sub_b073 @ 0xb073
char * sub_b073(unsigned long *a0,unsigned long *a1,void *a2) // ternary
{
  char *v1;
  unsigned int v2; // eax
  char *v3; // rax
  char *v4;
  
  v3 = strrchr((char *)*a0,0x2e);
  v1 = v3;
  v3 = strrchr((char *)*a1,0x2e);
  if (v3) // branch-flip
    v4 = v3;
  else {
    v4 = "";
  }
  if (v1) // branch-flip
    v3 = v1;
  else {
    v3 = "";
  }
  v2 = (*a2)(v3,v4);
  v3 = (v2) ? (unsigned long)(unsigned long)v2 : (unsigned long)(*a2)(*a0,*a1); // branch-flip
  return v3;
}

// Function: sub_b119 @ 0xb119
unsigned long sub_b119(struct_18 *a0)
{
  unsigned long v1; // rax
  
  if (a0->field_0xc8) // branch-flip
    v1 = a0->field_0xc8;
  else {
    v1 = sub_d671(a0->field_0x0,dat_2b410,a0->field_0xc4);
  }
  return v1;
}

// Function: sub_b16b @ 0xb16b
unsigned long sub_b16b(struct_18 *a0,struct_18 *a1,void *a2)
{
  int v1;
  unsigned int v2; // eax
  unsigned long v3; // rax
  
  v2 = sub_b119(a0);
  v1 = v2;
  v2 = sub_b119(a1);
  v2 = v1 - v2;
  if (v2) // branch-flip
    v3 = (unsigned long)v2;
  else {
    v3 = (*a2)(a0->field_0x0,a1->field_0x0);
  }
  return v3;
}

// Function: sub_b1d2 @ 0xb1d2
void sub_b1d2(unsigned long *a0,unsigned long *a1)
{
  sub_ad65(a0,a1,sub_ac36);
}

// Function: sub_b202 @ 0xb202
void sub_b202(unsigned long *a0,unsigned long *a1)
{
  sub_ad65(a0,a1,dat_2afc0);
}

// Function: sub_b232 @ 0xb232
void sub_b232(unsigned long *a0,unsigned long *a1)
{
  sub_ad65(a1,a0,sub_ac36);
}

// Function: sub_b262 @ 0xb262
void sub_b262(unsigned long *a0,unsigned long *a1)
{
  sub_ad65(a1,a0,dat_2afc0);
}

// Function: sub_b292 @ 0xb292
void sub_b292(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b1d2);
}

// Function: sub_b2c2 @ 0xb2c2
void sub_b2c2(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b202);
}

// Function: sub_b2f2 @ 0xb2f2
void sub_b2f2(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b232);
}

// Function: sub_b322 @ 0xb322
void sub_b322(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b262);
}

// Function: sub_b352 @ 0xb352
void sub_b352(unsigned long *a0,unsigned long *a1)
{
  sub_adf8(a0,a1,sub_ac36);
}

// Function: sub_b382 @ 0xb382
void sub_b382(unsigned long *a0,unsigned long *a1)
{
  sub_adf8(a0,a1,dat_2afc0);
}

// Function: sub_b3b2 @ 0xb3b2
void sub_b3b2(unsigned long *a0,unsigned long *a1)
{
  sub_adf8(a1,a0,sub_ac36);
}

// Function: sub_b3e2 @ 0xb3e2
void sub_b3e2(unsigned long *a0,unsigned long *a1)
{
  sub_adf8(a1,a0,dat_2afc0);
}

// Function: sub_b412 @ 0xb412
void sub_b412(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b352);
}

// Function: sub_b442 @ 0xb442
void sub_b442(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b382);
}

// Function: sub_b472 @ 0xb472
void sub_b472(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b3b2);
}

// Function: sub_b4a2 @ 0xb4a2
void sub_b4a2(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b3e2);
}

// Function: sub_b4d2 @ 0xb4d2
void sub_b4d2(unsigned long *a0,unsigned long *a1)
{
  sub_ae8b(a0,a1,sub_ac36);
}

// Function: sub_b502 @ 0xb502
void sub_b502(unsigned long *a0,unsigned long *a1)
{
  sub_ae8b(a0,a1,dat_2afc0);
}

// Function: sub_b532 @ 0xb532
void sub_b532(unsigned long *a0,unsigned long *a1)
{
  sub_ae8b(a1,a0,sub_ac36);
}

// Function: sub_b562 @ 0xb562
void sub_b562(unsigned long *a0,unsigned long *a1)
{
  sub_ae8b(a1,a0,dat_2afc0);
}

// Function: sub_b592 @ 0xb592
void sub_b592(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b4d2);
}

// Function: sub_b5c2 @ 0xb5c2
void sub_b5c2(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b502);
}

// Function: sub_b5f2 @ 0xb5f2
void sub_b5f2(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b532);
}

// Function: sub_b622 @ 0xb622
void sub_b622(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b562);
}

// Function: sub_b652 @ 0xb652
void sub_b652(unsigned long *a0,unsigned long *a1)
{
  sub_af1e(a0,a1,sub_ac36);
}

// Function: sub_b682 @ 0xb682
void sub_b682(unsigned long *a0,unsigned long *a1)
{
  sub_af1e(a0,a1,dat_2afc0);
}

// Function: sub_b6b2 @ 0xb6b2
void sub_b6b2(unsigned long *a0,unsigned long *a1)
{
  sub_af1e(a1,a0,sub_ac36);
}

// Function: sub_b6e2 @ 0xb6e2
void sub_b6e2(unsigned long *a0,unsigned long *a1)
{
  sub_af1e(a1,a0,dat_2afc0);
}

// Function: sub_b712 @ 0xb712
void sub_b712(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b652);
}

// Function: sub_b742 @ 0xb742
void sub_b742(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b682);
}

// Function: sub_b772 @ 0xb772
void sub_b772(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b6b2);
}

// Function: sub_b7a2 @ 0xb7a2
void sub_b7a2(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b6e2);
}

// Function: sub_b7d2 @ 0xb7d2
void sub_b7d2(struct_21 *a0,struct_21 *a1)
{
  sub_afe2(a0,a1,sub_ac36);
}

// Function: sub_b802 @ 0xb802
void sub_b802(struct_21 *a0,struct_21 *a1)
{
  sub_afe2(a0,a1,dat_2afc0);
}

// Function: sub_b832 @ 0xb832
void sub_b832(struct_21 *a0,struct_21 *a1)
{
  sub_afe2(a1,a0,sub_ac36);
}

// Function: sub_b862 @ 0xb862
void sub_b862(struct_21 *a0,struct_21 *a1)
{
  sub_afe2(a1,a0,dat_2afc0);
}

// Function: sub_b892 @ 0xb892
void sub_b892(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b7d2);
}

// Function: sub_b8c2 @ 0xb8c2
void sub_b8c2(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b802);
}

// Function: sub_b8f2 @ 0xb8f2
void sub_b8f2(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b832);
}

// Function: sub_b922 @ 0xb922
void sub_b922(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b862);
}

// Function: sub_b952 @ 0xb952
void sub_b952(unsigned long *a0,unsigned long *a1)
{
  sub_b03f(a0,a1,sub_ac36);
}

// Function: sub_b982 @ 0xb982
void sub_b982(unsigned long *a0,unsigned long *a1)
{
  sub_b03f(a0,a1,dat_2afc0);
}

// Function: sub_b9b2 @ 0xb9b2
void sub_b9b2(unsigned long *a0,unsigned long *a1)
{
  sub_b03f(a1,a0,sub_ac36);
}

// Function: sub_b9e2 @ 0xb9e2
void sub_b9e2(unsigned long *a0,unsigned long *a1)
{
  sub_b03f(a1,a0,dat_2afc0);
}

// Function: sub_ba12 @ 0xba12
void sub_ba12(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b952);
}

// Function: sub_ba42 @ 0xba42
void sub_ba42(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b982);
}

// Function: sub_ba72 @ 0xba72
void sub_ba72(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b9b2);
}

// Function: sub_baa2 @ 0xbaa2
void sub_baa2(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_b9e2);
}

// Function: sub_bad2 @ 0xbad2
void sub_bad2(unsigned long *a0,unsigned long *a1)
{
  sub_b073(a0,a1,sub_ac36);
}

// Function: sub_bb02 @ 0xbb02
void sub_bb02(unsigned long *a0,unsigned long *a1)
{
  sub_b073(a0,a1,dat_2afc0);
}

// Function: sub_bb32 @ 0xbb32
void sub_bb32(unsigned long *a0,unsigned long *a1)
{
  sub_b073(a1,a0,sub_ac36);
}

// Function: sub_bb62 @ 0xbb62
void sub_bb62(unsigned long *a0,unsigned long *a1)
{
  sub_b073(a1,a0,dat_2afc0);
}

// Function: sub_bb92 @ 0xbb92
void sub_bb92(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_bad2);
}

// Function: sub_bbc2 @ 0xbbc2
void sub_bbc2(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_bb02);
}

// Function: sub_bbf2 @ 0xbbf2
void sub_bbf2(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_bb32);
}

// Function: sub_bc22 @ 0xbc22
void sub_bc22(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_bb62);
}

// Function: sub_bc52 @ 0xbc52
void sub_bc52(struct_18 *a0,struct_18 *a1)
{
  sub_b16b(a0,a1,sub_ac36);
}

// Function: sub_bc82 @ 0xbc82
void sub_bc82(struct_18 *a0,struct_18 *a1)
{
  sub_b16b(a0,a1,dat_2afc0);
}

// Function: sub_bcb2 @ 0xbcb2
void sub_bcb2(struct_18 *a0,struct_18 *a1)
{
  sub_b16b(a1,a0,sub_ac36);
}

// Function: sub_bce2 @ 0xbce2
void sub_bce2(struct_18 *a0,struct_18 *a1)
{
  sub_b16b(a1,a0,dat_2afc0);
}

// Function: sub_bd12 @ 0xbd12
void sub_bd12(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_bc52);
}

// Function: sub_bd42 @ 0xbd42
void sub_bd42(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_bc82);
}

// Function: sub_bd72 @ 0xbd72
void sub_bd72(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_bcb2);
}

// Function: sub_bda2 @ 0xbda2
void sub_bda2(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_bce2);
}

// Function: sub_bdd2 @ 0xbdd2
unsigned long sub_bdd2(unsigned long *a0,unsigned long *a1)
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  
  v1 = sub_11c04((char *)*a0,(char *)*a1);
  if (v1) // branch-flip
    v2 = (unsigned long)v1;
  else {
    v1 = strcmp((char *)*a0,(char *)*a1);
  }
  return v2;
}

// Function: sub_be28 @ 0xbe28
void sub_be28(unsigned long *a0,unsigned long *a1)
{
  sub_bdd2(a0,a1);
}

// Function: sub_be51 @ 0xbe51
void sub_be51(unsigned long *a0,unsigned long *a1)
{
  sub_bdd2(a1,a0);
}

// Function: sub_be7a @ 0xbe7a
void sub_be7a(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_be28);
}

// Function: sub_beaa @ 0xbeaa
void sub_beaa(char *a0,char *a1)
{
  sub_ad02(a0,a1,sub_be51);
}

// Function: sub_beda @ 0xbeda
void sub_beda(void)
{
  unsigned long v1; // stack - 0x10
  
  for (v1 = 0; v1 < dat_2b338; v1 = v1 + 1) {
    *(unsigned long *)(v1 * 8 + dat_2b348) = dat_2b328 + v1 * 0xd0;
  }
}

// Function: sub_bf3d @ 0xbf3d
void sub_bf3d(void)
{
  struct_18 *v1;
  unsigned long v2; // stack - 0x18
  
  if ((dat_2b3b4 == 2) || ((dat_2b430 && ((dat_2b3ac == 2 || (dat_2b3ac == 3)))))) {
    for (v2 = 0; v2 < dat_2b338; v2 = v2 + 1) {
      v1 = *(struct_18 **)(v2 * 8 + dat_2b348);
      v1->field_0xc8 = sub_b119(v1);
    }
  }
}

// Function: sub_bfc5 @ 0xbfc5
void sub_bfc5(void) // return-dupe
{
  unsigned int v1;
  unsigned char v2; // stack - 0x9
  
  if (dat_2b350 < (dat_2b338 >> 1) + dat_2b338) {
    free(dat_2b348);
    dat_2b348 = (void *)sub_1c804(dat_2b338,0x18);
    dat_2b350 = dat_2b338 * 3;
  }
  sub_beda();
  sub_bf3d();
  if (dat_2b3b4 == 6)
    return;
  if (_setjmp((void *)0x2c360)) { // branch-flip
    v2 = 1;
    if (dat_2b3b4 == 4)
      __assert_fail("sort_type != sort_version","src/ls.c",0x1008,"sort_files"); // no-return
    sub_beda();
  }
  else {
    v2 = 0;
  }
  if (dat_2b3b4 != 5) // branch-flip
    v1 = 0;
  else {
    v1 = dat_2b3b0;
  }
  sub_15dfc(dat_2b348,dat_2b338,*(void **)((((long)(int)(unsigned int)dat_2b3b8 + ((long)(int)(unsigned int)v2 + (unsigned long)(unsigned int)(v1 + dat_2b3b4) * 2) * 2) * 2 + (long)(int)(unsigned int)dat_2b3f2) * 8 + 0x2a780));
}

// Function: sub_c12d @ 0xc12d
void sub_c12d(void) // return-dupe x7
{
  unsigned long v1; // stack - 0x10
  
  switch(dat_2b3ac) {
    case 0:
      for (v1 = 0; v1 < dat_2b338; v1 = v1 + 1) {
        sub_63f3();
        sub_c6c8(*(unsigned long *)(v1 * 8 + dat_2b348));
        sub_5422((int)dat_2b030);
      }
      return;
    case 1:
      for (v1 = 0; v1 < dat_2b338; v1 = v1 + 1) {
        sub_dd38(*(unsigned long *)(v1 * 8 + dat_2b348),0);
        putchar_unlocked((int)dat_2b030);
      }
      return;
    case 2:
      goto label_c1bf;
    case 3:
      if (!dat_2b430) {
        sub_e9d6(0x20);
        return;
      }
      sub_e86a();
      return;
    case 4:
      sub_e9d6(0x2c);
      return;
    default:
      return;
    
  }
label_c1bf:
  if (!dat_2b430) {
    sub_e9d6(0x20);
    return;
  }
  sub_e708();
}

// Function: sub_c25f @ 0xc25f
void sub_c25f(char *a0,long a1,unsigned char a2,tm *a3,unsigned long a4,unsigned int a5)
{
  char *v1;
  
  if (dat_2c340) // branch-flip
    v1 = (char *)(((long)(int)(unsigned int)a2 * 0xc + (long)*(int *)&a3->field_0x10) * 0x80 + 0x2b740);
  else {
    v1 = *(char **)((long)(int)(unsigned int)a2 * 8 + 0x2b200);
  }
  sub_15f45(a0,a1,v1,a3,a4,a5);
}

// Function: sub_c308 @ 0xc308
int sub_c308(void)
{
  char v1 [64];
  unsigned long v2; // stack - 0x458
  char v3 [1016];
  long v4; // stack - 0x450
  
  if (dat_2b034 <= -1) {
    v2 = 0;
    if ((sub_1b9a5(dat_2b438,&v2,v1)) && (v4 = sub_c25f(v3,0x3e9,0,v1,dat_2b438,0), v4))
      dat_2b034 = sub_15784(v3,v4,0);
    if (dat_2b034 <= -1)
      dat_2b034 = 0;
  }
  return dat_2b034;
}

// Function: sub_c3fc @ 0xc3fc
void sub_c3fc(char *a0,unsigned long a1,unsigned int a2) // return-dupe
{
  int v1;
  bool v2; // zf
  int v3; // stack - 0x10
  
  if (a0) {
    v3 = a2 - sub_1574e(a0,0);
    if (v3 <= -1)
      v3 = 0;
    sub_549e(a0);
    do {
      sub_5422(0x20);
      v1 = v3 + -1;
      v2 = v3 != 0;
      v3 = v1;
    } while (v2);
    return;
  }
  dat_2b500 += printf("%*lu ",(unsigned long)a2,a1);
}

// Function: sub_c49e @ 0xc49e
void sub_c49e(unsigned int a0,unsigned int a1,char a2) // ternary
{
  char *v1; // rax
  
  if (a2 != '\x01') // branch-flip
    v1 = "?";
  else {
    v1 = (dat_2b3ba) ? NULL : (char *)sub_14a53(a0); // branch-flip
  }
  sub_c3fc(v1,a0,a1);
}

// Function: sub_c4fe @ 0xc4fe
void sub_c4fe(unsigned int a0,unsigned int a1,char a2) // ternary
{
  char *v1; // rax
  
  if (a2 != '\x01') // branch-flip
    v1 = "?";
  else {
    v1 = (dat_2b3ba) ? NULL : (char *)sub_14ca5(a0); // branch-flip
  }
  sub_c3fc(v1,a0,a1);
}

// Function: sub_c55e @ 0xc55e
unsigned long sub_c55e(char *a0,unsigned long a1) // early-return
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  bool v3; // sf
  
  if (a0) { // branch-flip
    v1 = sub_1574e(a0,0);
    v3 = (int)v1 <= -1;
    v2 = (unsigned long)v1;
    if (v3) {
      v2 = 0;
      return v2;
    }
  }
  else {
    v1 = snprintf(NULL,0,"%lu",a1);
  }
  return v2;
}

// Function: sub_c5c3 @ 0xc5c3
void sub_c5c3(unsigned int a0) // ternary
{
  char *v1; // rax
  
  v1 = (dat_2b3ba) ? NULL : (char *)sub_14a53(a0); // branch-flip
  sub_c55e(v1,a0);
}

// Function: sub_c603 @ 0xc603
void sub_c603(unsigned int a0) // ternary
{
  char *v1; // rax
  
  v1 = (dat_2b3ba) ? NULL : (char *)sub_14ca5(a0); // branch-flip
  sub_c55e(v1,a0);
}

// Function: sub_c643 @ 0xc643
char * sub_c643(char *a0,unsigned long a1,char *a2) // early-return
{
  unsigned long v1;
  
  if (a1 <= 0x14)
    __assert_fail("INT_BUFSIZE_BOUND (uintmax_t) <= buflen","src/ls.c",0x10be,"format_inode"); // no-return
  if ((a2[0xb8]) && (*(long *)&a2[0x20])) {
    v1 = *(unsigned long *)&a2[0x20];
    return (char *)sub_15035(v1,a0);
  }
  return "?";
}

// Function: sub_c6c8 @ 0xc6c8
void sub_c6c8(char *a0) // return-dupe x4, ternary x2, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_c6c8
{
  unsigned long v1;
  long v10; // rax
  int v11;
  char v12; // stack - 0x1124
  char v13 [9];
  char v14 [3656];
  char v15 [656];
  char v16 [32];
  char v17 [68];
  long v18; // stack - 0x1188
  unsigned long v19; // r9
  unsigned long v2;
  char v20 [16];
  int v21; // stack - 0x11bc
  int v22; // stack - 0x11b8
  long v23; // stack - 0x11b0
  char *v24; // stack - 0x11a8
  char *v25; // stack - 0x11a0
  char *v26; // stack - 0x1198
  unsigned long v27; // stack - 0x1180
  long v28; // stack - 0x1178
  unsigned long v29; // stack - 0x1170
  bool v3;
  char v30; // stack - 0x111a
  char v31; // stack - 0x1119
  unsigned int v4;
  char v5; // al
  int v6; // eax
  int v7;
  char *v8;
  char *v9;
  
  v3 = 1;
  if (a0[0xb8]) // branch-flip
    sub_1167b((struct_25 *)&a0[0x18],&v12);
  else {
    v12 = "?pcdb-lswd"[*(unsigned int *)&a0[0xa8]];
    memset(v13,0x3f,10);
    v31 = 0;
  }
  if (dat_2b381 != '\x01') // branch-flip
    v30 = 0;
  else if (*(int *)&a0[0xbc] != 1) { // branch-flip
    if (*(int *)&a0[0xbc] == 2)
      v30 = 0x2b;
  }
  else {
    v30 = 0x2e;
  }
  switch(dat_2b3b0) { // branch-flip
    case 0:
      v20 = sub_1b491(&a0[0x18]);
      v27 = SUB168(v20,8);
      v18 = SUB168(v20,0);
      break;
    default:
      abort(); // no-return, return-dupe
    case 1:
      v20 = sub_1b477(&a0[0x18]);
      v27 = SUB168(v20,8);
      v18 = SUB168(v20,0);
      break;
    case 2:
      v20 = sub_1b45d(&a0[0x18]);
      v27 = SUB168(v20,8);
      v18 = SUB168(v20,0);
      break;
    case 3:
      v20 = sub_5990(&a0[0x18]);
      v27 = SUB168(v20,8);
      v18 = SUB168(v20,0);
      if ((v18 == -1) && (v27 == 0xffffffffffffffff))
        v3 = 0;
    
  }
  v24 = v14;
  if (dat_2b3e9) {
    v8 = (char *)sub_c643(v15,0x15,a0);
    v24 = &v24[sprintf(v24,"%*s ",dat_2b384,v8)];
  }
  if (dat_2b3bb) {
    v25 = (a0[0xb8] != '\x01') ? "?" : (char *)sub_13d91(*(unsigned long *)&a0[0x58],v15,dat_2b3bc,0x200,dat_2b3c0); // branch-flip
    v21 = dat_2b388;
    v21 -= sub_1574e(v25,0);
    while (v8 = v24, 0 < v21) {
      v8 = &v24[1];
      *v24 = ' ';
      v21 -= 1;
      v24 = v8;
    }
    do {
      v24 = v8;
      v9 = &v25[1];
      v8 = &v24[1];
      *v24 = *v25;
      v25 = v9;
    } while (*v24);
    *v24 = ' ';
    v24 = v8;
  }
  v8 = (a0[0xb8] != '\x01') ? "?" : (char *)sub_15035(*(unsigned long *)&a0[0x28],v15); // branch-flip
  v24 = &v24[sprintf(v24,"%s %*s ",&v12,dat_2b38c,v8)];
  sub_54cf();
  if ((((dat_2b022) || (dat_2b023)) || (dat_2b3b9)) || (dat_2b380)) {
    sub_5453(v14,(long)v24 - (long)v14);
    if (dat_2b022)
      sub_c49e(*(unsigned int *)&a0[0x34],dat_2b394,(unsigned char)a0[0xb8]);
    if (dat_2b023)
      sub_c4fe(*(unsigned int *)&a0[0x38],dat_2b398,(unsigned char)a0[0xb8]);
    if (dat_2b3b9)
      sub_c49e(*(unsigned int *)&a0[0x34],dat_2b39c,(unsigned char)a0[0xb8]);
    if (dat_2b380)
      sub_c3fc(*(char **)&a0[0xb0],0,dat_2b390);
    v24 = v14;
  }
  v4 = dat_2b3c8;
  v1 = dat_2b028;
  if ((a0[0xb8]) && (((*(unsigned int *)&a0[0x30] & 0xf000) == 0x2000 || ((*(unsigned int *)&a0[0x30] & 0xf000) == 0x6000)))) {
    v6 = dat_2b3a8 - (dat_2b3a4 + dat_2b3a0 + 2);
    v1 = *(unsigned long *)&a0[0x40];
    v8 = (char *)sub_15035(gnu_dev_minor(v1),v15);
    v7 = dat_2b3a4;
    v1 = *(unsigned long *)&a0[0x40];
    v9 = (char *)sub_15035(gnu_dev_major(v1),v16);
    v11 = 0;
    if (0 <= v6)
      v11 = v6;
    v11 += dat_2b3a0;
    v24 = &v24[sprintf(v24,"%*s, %*s ",v11,v9,v7,v8)];
  }
  else {
    if (a0[0xb8] != '\x01') // branch-flip
      v26 = "?";
    else {
      v2 = *(unsigned long *)&a0[0x48];
      v26 = (char *)sub_13d91(sub_9635(v2),v15,v4,1,v1);
    }
    v22 = dat_2b3a8;
    v22 -= sub_1574e(v26,0);
    while (v8 = v24, 0 < v22) {
      v8 = &v24[1];
      *v24 = ' ';
      v22 -= 1;
      v24 = v8;
    }
    do {
      v24 = v8;
      v9 = &v26[1];
      v8 = &v24[1];
      *v24 = *v26;
      v26 = v9;
    } while (*v24);
    *v24 = ' ';
    v24 = v8;
  }
  v23 = 0;
  *v24 = '\x01';
  if (((a0[0xb8]) && (v3)) && (sub_1b9a5(dat_2b438,&v18,v17))) {
    v7 = sub_1bc1f(dat_2b370,dat_2b378,v18,v27);
    if (v7 <= -1)
      sub_11ec3((timespec *)0x2b370);
    v28 = dat_2b370 + -0xf0c2ac;
    v29 = dat_2b378;
    v7 = sub_1bc1f(v28,dat_2b378,v18,v27);
    v5 = !((0 <= v7) || (v7 = sub_1bc1f(v18,v27,dat_2b370,dat_2b378), 0 <= v7));
    v19 = v27 & 0xffffffff;
    v23 = sub_c25f(v24,0x3e9,(unsigned char)v5,v17,dat_2b438,v19);
  }
  if ((v23) || (!*v24)) {
    v8 = &v24[v23];
    v24 = &v8[1];
    *v8 = ' ';
  }
  else {
    if ((!a0[0xb8]) || (!v3))
      v8 = "?";
    else {
      v8 = (char *)sub_5219(v18,v15);
    }
    v7 = sub_c308();
    v24 = &v24[sprintf(v24,"%*s ",v7,v8)];
  }
  sub_5453(v14,(long)v24 - (long)v14);
  v10 = sub_db96(a0,0,(obstack *)0x2b520,(long)v24 - (long)v14);
  if (*(int *)&a0[0xa8] != 6) {
    if (!dat_2b3d0)
      return;
    sub_e05c((unsigned char)a0[0xb8],*(unsigned int *)&a0[0x30],*(unsigned int *)&a0[0xa8]);
    return;
  }
  if (!*(long *)&a0[8])
    return;
  sub_549e(" -> ");
  sub_db96(a0,1,NULL,&v24[(v10 - (long)v14) + 4]);
  if (!dat_2b3d0)
    return;
  sub_e05c(1,*(unsigned int *)&a0[0xac],0);
}

// Function: sub_d10c @ 0xd10c
unsigned long sub_d10c(unsigned long *a0,unsigned long a1,char *a2,unsigned int *a3,int a4,unsigned long *a5,char *a6) // warn: iteboolean: re-rolled 3 0/1 select diamond(s) to a boolean assignment in sub_d10c
{
  char *v1;
  unsigned long v10; // rax
  mbstate_t v11; // stack - 0x28
  int v12; // stack - 0x8c
  int v13; // stack - 0x88
  int v14; // stack - 0x84
  char *v15; // stack - 0x80
  unsigned long v16; // stack - 0x78
  unsigned long v17; // stack - 0x70
  char *v18; // stack - 0x68
  char *v19; // stack - 0x60
  char *v2;
  long v20; // stack - 0x58
  char *v21; // stack - 0x50
  char *v22; // stack - 0x48
  char *v23; // stack - 0x40
  char *v24; // stack - 0x38
  char *v25; // stack - 0x30
  char v3;
  long v4;
  char *v5;
  bool v6; // al
  bool v7;
  char v8; // al
  int v9; // eax
  
  v5 = a6;
  v15 = (char *)*a0;
  v17 = 0;
  v13 = sub_18fc3(a3);
  v6 = ((dat_2b408) && (((v13 == 1 || (v13 == 2)) || (!v13))));
  if (a4) { // branch-flip
    v17 = sub_1a389(v15,a1,a2,0xffffffffffffffff,a3);
    if (a1 <= v17) {
      v15 = (char *)sub_1c68b(v17 + 1);
      sub_1a389(v15,v17 + 1,a2,0xffffffffffffffff,a3);
    }
    v7 = ((*a2 != *v15) || (v17 != strlen(a2)));
  }
  else if (v6) {
    v17 = strlen(a2);
    if (a1 <= v17) {
      v10 = v17 + 1;
      v15 = (char *)sub_1c68b(v10);
    }
    memcpy(v15,a2,v17 + 1);
    v7 = 0;
  }
  else {
    v17 = strlen(a2);
    v7 = 0;
    v15 = a2;
  }
  if (v6) {
    if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
      v18 = v15;
      v25 = &v15[v17];
      v19 = v15;
      v16 = 0;
label_d4c6:
      if (v18 < v25) {
        v9 = (int)*v18;
        if (0x7f <= v9) { // branch-flip
label_d3b9:
          v11 = 0;
          do {
            v20 = sub_150be(&v12,v18,(long)v25 - (long)v18,&v11);
            if (v20 == -1) {
              v18 = &v18[1];
              v1 = &v19[1];
              *v19 = '?';
              v16 += 1;
              v19 = v1;
              break;
            }
            if (v20 == -2) {
              v18 = v25;
              v1 = &v19[1];
              *v19 = '?';
              v16 += 1;
              v19 = v1;
              break;
            }
            if (!v20)
              v20 = 1;
            v14 = wcwidth(v12);
            if (0 <= v14) { // branch-flip
              while (v20) {
                v1 = &v18[1];
                v2 = &v19[1];
                *v19 = *v18;
                v20 -= 1;
                v18 = v1;
                v19 = v2;
              }
              v16 += (long)v14;
            }
            else {
              v18 = &v18[v20];
              v1 = &v19[1];
              *v19 = '?';
              v16 += 1;
              v19 = v1;
            }
          } while (!mbsinit(&v11));
        }
        else {
          if (v9 <= 0x60) {
            if (0x60 > v9) {
              if (v9 > 0x40) goto label_d392;
              if (0x24 <= v9) { // branch-flip
                if (0x1b > (unsigned int)(v9 - 0x25U)) goto label_d392;
              }
              else if (v9 > 0x1f) goto label_d392;
            }
            goto label_d3b9;
          }
label_d392:
          v1 = &v18[1];
          v2 = &v19[1];
          *v19 = *v18;
          v16 += 1;
          v18 = v1;
          v19 = v2;
        }
        goto label_d4c6;
      }
      v17 = (long)v19 - (long)v15;
    }
    else {
      v24 = &v15[v17];
      for (v21 = v15; v21 < v24; v21 = &v21[1]) {
        v4 = *(long *)__ctype_b_loc();
        v3 = *v21;
        if (!(*(unsigned short *)((unsigned long)(unsigned char)sub_4eb2((int)v3) * 2 + v4) & 0x4000))
          *v21 = '?';
      }
      v16 = v17;
    }
  }
  else if (a5) {
    if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
      v9 = sub_15784(v15,v17,0);
      v16 = (unsigned long)v9;
    }
    else {
      v23 = &v15[v17];
      v16 = 0;
      for (v22 = v15; v22 < v23; v22 = &v22[1]) {
        v4 = *(long *)__ctype_b_loc();
        v3 = *v22;
        if (*(unsigned short *)((unsigned long)(unsigned char)sub_4eb2((int)v3) * 2 + v4) & 0x4000)
          v16 += 1;
      }
    }
  }
  v8 = !(((!dat_2b341) || (!dat_2b340)) || (v7));
  *v5 = v8;
  if (a5)
    *a5 = v16;
  *a0 = v15;
  return v17;
}

// Function: sub_d671 @ 0xd671
long sub_d671(char *a0,unsigned int *a1,unsigned int a2)
{
  char v1 [8200];
  long v2; // stack - 0x2020
  char *v3; // stack - 0x2028
  unsigned char v4; // stack - 0x2029
  
  v3 = v1;
  sub_d10c(&v3,0x2000,a0,a1,a2,&v2,&v4);
  if ((v3 != v1) && (a0 != v3))
    free(v3);
  return v2 + (unsigned long)v4;
}

// Function: sub_d775 @ 0xd775
char * sub_d775(char *a0,bool a1)
{
  char *v1;
  char *v2;
  char v3;
  unsigned char v4; // al
  char *v5; // rax
  char *v6; // stack - 0x20
  char *v7; // stack - 0x18
  
  v5 = (char *)sub_1c804(3,strlen(a0) + 1);
  v7 = v5;
  v6 = a0;
  while (*v6) {
    if ((a1) && (*v6 == '/')) {
      v1 = &v7[1];
      *v7 = '/';
      v6 = &v6[1];
      v7 = v1;
    }
    else {
      v3 = *v6;
      if (*(char *)((long)(int)(unsigned int)(unsigned char)sub_4eb2((int)v3) + 0x2b640)) { // branch-flip
        v1 = &v6[1];
        v2 = &v7[1];
        *v7 = *v6;
        v6 = v1;
        v7 = v2;
      }
      else {
        v1 = &v6[1];
        v4 = sub_4eb2((int)*v6);
        v7 = &v7[sprintf(v7,"%%%02x",(unsigned int)v4)];
        v6 = v1;
      }
    }
  }
  *v7 = '\0';
  return v5;
}

// Function: sub_d87f @ 0xd87f
long sub_d87f(char *a0,unsigned int *a1,unsigned int a2,struct_2 *a3,bool a4,obstack *a5,char *a6) // ternary
{
  char *v1;
  char *v2; // rax
  char v3 [8200];
  char *v4; // stack - 0x2038
  unsigned char v5; // stack - 0x203a
  unsigned char v6; // stack - 0x2039
  long v7; // stack - 0x2030
  char *v8; // stack - 0x2028
  char *v9; // stack - 0x2020
  
  v1 = a6;
  v4 = v3;
  v7 = sub_d10c(&v4,0x2000,a0,a1,a2,NULL,&v5);
  if ((v5) && (a4))
    sub_5422(0x20);
  if (a3)
    sub_e0a3(a3);
  v6 = 0;
  if (v1) {
    if (((dat_2b341) && (dat_2b340)) && (v5 != 1)) {
      v6 = 1;
      putchar_unlocked((int)*v4);
    }
    v8 = (char *)sub_d775(dat_2b360,0);
    v9 = (char *)sub_d775(v1,1);
    v2 = (*v9 != '/') ? "/" : ""; // branch-flip
    printf("\x1b]8;;file://%s%s%s\a",v8,v2,v9);
    free(v8);
    free(v9);
  }
  if (a5)
    sub_54f4(a5);
  fwrite_unlocked(&v4[v6],1,v7 - (int)((unsigned int)v6 * 2),stdout);
  dat_2b500 = v7 + dat_2b500;
  if (a5)
    sub_54f4(a5);
  if ((v1) && (fputs_unlocked("\x1b]8;;\a",stdout), v6))
    putchar_unlocked((int)v4[v7 + -1]);
  if ((v4 != v3) && (a0 != v4))
    free(v4);
  return v7 + (unsigned long)v5;
}

// Function: sub_db96 @ 0xdb96
long sub_db96(struct_24 *a0,char a1,obstack *a2,unsigned long a3) // ternary x2
{
  bool v1; // al
  char *v2;
  struct_2 *v3; // rax
  long v4; // rax
  unsigned long v5; // rax
  
  v2 = (a1) ? a0->field_0x8 : a0->field_0x0; // branch-flip
  v3 = (dat_2b3d4) ? (struct_2 *)sub_e101(a0,(unsigned char)a1) : NULL; // branch-flip
  if (dat_2b3d4) { // branch-flip
    if (!v3) {
      if (!sub_6325(4)) goto label_dc1d;
    }
    v1 = 1;
  }
  else {
label_dc1d:
    v1 = 0;
  }
  v4 = sub_d87f(v2,dat_2b410,a0->field_0xc4,v3,a1 == '\0',a2,a0->field_0x10);
  sub_648c();
  if (v1) {
    sub_dce3();
    if (dat_2b430) {
      v5 = (v4 + a3) - 1;
      if (a3 / dat_2b430 != v5 / dat_2b430)
        sub_e4ca((struct_2 *)0x2b1f0,dat_2b430,v5 % dat_2b430);
    }
  }
  return v4;
}

// Function: sub_dce3 @ 0xdce3
void sub_dce3(void) // return-dupe
{
  if (dat_2b0a8) {
    sub_e4ca((struct_2 *)0x2b0a0);
    return;
  }
  sub_e4ca((struct_2 *)0x2b080);
  sub_e4ca((struct_2 *)0x2b0b0);
  sub_e4ca((struct_2 *)0x2b090);
}

// Function: sub_dd38 @ 0xdd38
char * sub_dd38(char *a0,unsigned long a1) // ternary
{
  char v1;
  unsigned int v2;
  unsigned int v3;
  char *v4;
  int v5;
  char *v6; // rax
  int v7;
  char v8 [664];
  long v9; // stack - 0x2b0
  
  sub_63f3();
  if (dat_2b3e9) {
    v6 = (char *)sub_c643(v8,0x28c,a0);
    v4 = v6;
    if (dat_2b3ac != 4) // branch-flip
      v5 = dat_2b384;
    else {
      v5 = 0;
    }
    printf("%*s ",v5,v4);
  }
  if (dat_2b3bb) {
    v6 = (a0[0xb8] != '\x01') ? "?" : (char *)sub_13d91(*(unsigned long *)&a0[0x58],v8,dat_2b3bc,0x200,dat_2b3c0); // branch-flip
    if (dat_2b3ac != 4) // branch-flip
      v7 = dat_2b388;
    else {
      v7 = 0;
    }
    printf("%*s ",v7,v6);
  }
  if (dat_2b380) {
    if (dat_2b3ac != 4) // branch-flip
      v5 = dat_2b390;
    else {
      v5 = 0;
    }
    printf("%*s ",v5,*(char **)&a0[0xb0]);
  }
  v6 = (long)sub_db96(a0,0,NULL,a1);
  v9 = (long)v6;
  if (dat_2b3d0) {
    v2 = *(unsigned int *)&a0[0xa8];
    v3 = *(unsigned int *)&a0[0x30];
    v1 = a0[0xb8];
    v6 = (unsigned long)(unsigned long)(unsigned char)sub_e05c((unsigned char)v1,v3,v2);
    v9 += (long)v6;
  }
  v6 = (char *)v9;
  return v6;
}

// Function: sub_df23 @ 0xdf23
char sub_df23(bool a0,unsigned int a1,int a2) // return-dupe x7, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_df23
{
  char v1; // al
  unsigned int v2; // eax
  bool v3;
  
  if (a0) { // branch-flip
    v2 = a1 & 0xf000;
    v3 = v2 == 0x8000;
  }
  else {
    v3 = a2 == 5;
  }
  if (v3) {
    if (((a0) && (dat_2b3d0 == 3)) && (v2 = a1 & 0x49, v2)) {
      v1 = 0x2a;
      return v1;
    }
    v1 = 0;
    return v1;
  }
  if (a0) { // branch-flip
    v2 = a1 & 0xf000;
    v1 = v2 == 0x4000;
  }
  else {
    v1 = !((a2 != 3) && (a2 != 9));
  }
  if ((bool)v1) {
    v1 = 0x2f;
    return v1;
  }
  if (dat_2b3d0 == 1) {
    v1 = 0;
    return v1;
  }
  if (a0) { // branch-flip
    v2 = a1 & 0xf000;
    v3 = v2 == 0xa000;
  }
  else {
    v3 = a2 == 6;
  }
  if (v3) {
    v1 = 0x40;
    return v1;
  }
  if (a0) { // branch-flip
    v2 = a1 & 0xf000;
    v3 = v2 == 0x1000;
  }
  else {
    v3 = a2 == 1;
  }
  if (v3) {
    v1 = 0x7c;
    return v1;
  }
  if (a0) { // branch-flip
    v2 = a1 & 0xf000;
    v3 = v2 == 0xc000;
  }
  else {
    v3 = a2 == 7;
  }
  if (!v3) {
    v1 = 0;
    return v1;
  }
  v1 = 0x3d;
  return v1;
}

// Function: sub_e05c @ 0xe05c
unsigned long sub_e05c(char a0,unsigned int a1,unsigned int a2)
{
  unsigned long v1;
  char v2;
  char v3; // al
  unsigned long v4; // rax
  
  v4 = sub_df23((unsigned char)a0,a1,a2);
  v2 = v3;
  if (v3)
    v4 = sub_5422((int)v3);
  v1 = (unsigned long)v4 >> 8;
  v4 = CONCAT71((undefined7)v1,v2 != '\0');
  return v4;
}

// Function: sub_e0a3 @ 0xe0a3
unsigned long sub_e0a3(struct_2 *a0)
{
  unsigned long v1;
  unsigned long v2; // rax
  
  if (a0) {
    if (sub_6325(4))
      sub_63ca();
    sub_e4ca((struct_2 *)0x2b080);
    sub_e4ca(a0);
    v2 = sub_e4ca((struct_2 *)0x2b090);
  }
  v1 = (unsigned long)v2 >> 8;
  v2 = CONCAT71((undefined7)v1,a0 != NULL);
  return v2;
}

// Function: sub_e101 @ 0xe101
unsigned long * sub_e101(struct_23 *a0,bool a1) // early-return, ternary x3
{
  unsigned long v1;
  char *v2;
  unsigned long v3;
  unsigned long v4; // rax
  unsigned int v5; // stack - 0x34
  unsigned int v6; // stack - 0x30
  unsigned int v7; // stack - 0x2c
  unsigned long *v8; // stack - 0x28
  char *v9; // stack - 0x20
  
  if (a1) { // branch-flip
    v9 = a0->field_0x8;
    v6 = a0->field_0xac;
    v7 = (a0->field_0xb9) ? 0 : 0xffffffff; // branch-flip
  }
  else {
    v9 = a0->field_0x0;
    v6 = sub_53e7(a0);
    v7 = (unsigned int)(unsigned char)a0->field_0xb9;
  }
  if ((v7 != 0xffffffff) || (!sub_6325(0xc))) {
    if (a0->field_0xb8 != '\x01') // branch-flip
      v5 = *(unsigned int *)((unsigned long)a0->field_0xa8 * 4 + 0x2b040);
    else if ((v6 & 0xf000) != 0x8000) { // branch-flip
      if ((v6 & 0xf000) != 0x4000) { // branch-flip
        if ((v6 & 0xf000) != 0xa000) { // branch-flip
          if ((v6 & 0xf000) != 0x1000) { // branch-flip
            if ((v6 & 0xf000) != 0xc000) { // branch-flip
              if ((v6 & 0xf000) != 0x6000) { // branch-flip
                v5 = ((v6 & 0xf000) != 0x2000) ? 0xd : 0xb; // branch-flip
              }
              else {
                v5 = 10;
              }
            }
            else {
              v5 = 9;
            }
          }
          else {
            v5 = 8;
          }
        }
        else {
          v5 = 7;
        }
      }
      else {
        v5 = 6;
        if (((v6 & 0x200) && (v6 & 2)) && (sub_6325(0x14)))
          v5 = 0x14;
        else if ((v6 & 2) && (sub_6325(0x13)))
          v5 = 0x13;
        else if ((v6 & 0x200) && (sub_6325(0x12)))
          v5 = 0x12;
      }
    }
    else {
      v5 = 5;
      if ((v6 & 0x800) && (sub_6325(0x10)))
        v5 = 0x10;
      else if ((v6 & 0x400) && (sub_6325(0x11)))
        v5 = 0x11;
      else if ((sub_6325(0x15)) && (a0->field_0xc0))
        v5 = 0x15;
      else if ((v6 & 0x49) && (sub_6325(0xe)))
        v5 = 0xe;
      else if ((2 <= a0->field_0x28) && (sub_6325(0x16)))
        v5 = 0x16;
    }
  }
  else {
    v5 = 0xc;
  }
  v8 = NULL;
  if (v5 == 5) {
    v4 = strlen(v9);
    v8 = dat_2b3d8;
    while ((v8 && ((v4 < *v8 || (v1 = *v8, v2 = (char *)v8[1], v3 = *v8, sub_102b5(&v9[v4 - v3],v2,v1)))))) {
      v8 = (unsigned long *)v8[4];
    }
  }
  if (((v5 == 7) && (!v7)) && ((dat_2b358 || (sub_6325(0xd)))))
    v5 = 0xd;
  v8 = (v8) ? &v8[2] : (unsigned long *)((unsigned long)v5 * 0x10 + 0x2b080); // branch-flip
  if (!v8[1])
    return NULL;
  return v8;
}

// Function: sub_e4ca @ 0xe4ca
void sub_e4ca(struct_2 *a0)
{
  if (dat_2b3d6 != '\x01') {
    dat_2b3d6 = '\x01';
    if (0 <= tcgetpgrp(1))
      sub_6858();
    sub_dce3();
  }
  fwrite_unlocked(a0->field_0x8,a0->field_0x0,1,stdout);
}

// Function: sub_e530 @ 0xe530
long sub_e530(char *a0) // ternary
{
  char v1;
  unsigned int v2;
  unsigned int v3;
  unsigned long v4;
  char *v5; // rax
  char v6 [664];
  long v7; // stack - 0x2b0
  
  v7 = 0;
  if (dat_2b3e9) {
    if (dat_2b3ac != 4) // branch-flip
      v4 = (unsigned long)dat_2b384;
    else {
      v4 = *(unsigned long *)&a0[0x20];
      v4 = strlen((char *)sub_15035(v4,v6));
    }
    v7 = v4 + 1;
  }
  if (dat_2b3bb) {
    if (dat_2b3ac != 4) // branch-flip
      v4 = (unsigned long)dat_2b388;
    else {
      v5 = (a0[0xb8] != '\x01') ? "?" : (char *)sub_13d91(*(unsigned long *)&a0[0x58],v6,dat_2b3bc,0x200,dat_2b3c0); // branch-flip
      v4 = strlen(v5);
    }
    v7 += v4 + 1;
  }
  if (dat_2b380) {
    if (dat_2b3ac != 4) // branch-flip
      v4 = (unsigned long)dat_2b390;
    else {
      v4 = strlen(*(char **)&a0[0xb0]);
    }
    v7 += v4 + 1;
  }
  v7 += sub_b119(a0);
  if (dat_2b3d0) {
    v2 = *(unsigned int *)&a0[0xa8];
    v3 = *(unsigned int *)&a0[0x30];
    v1 = a0[0xb8];
    v7 += (unsigned long)(sub_df23((unsigned char)v1,v3,v2) != '\0');
  }
  return v7;
}

// Function: sub_e708 @ 0xe708
void sub_e708(void)
{
  long v1;
  unsigned long v10; // stack - 0x40
  char *v2;
  long v3;
  unsigned long v4; // rax
  long v5; // rax
  long v6; // rax
  unsigned long v7; // stack - 0x58
  long v8; // stack - 0x50
  unsigned long v9; // stack - 0x48
  
  v4 = sub_ee73(1);
  v5 = v4 * 0x18 + dat_2b4f0;
  v4 = (unsigned long)(dat_2b338 % v4 != 0) + dat_2b338 / v4;
  for (v7 = 0; v7 < v4; v7 = v7 + 1) {
    v8 = 0;
    v9 = v7;
    v10 = 0;
    while( true ) {
      v2 = *(char **)(v9 * 8 + dat_2b348);
      v6 = sub_e530(v2);
      v1 = v8 + 1;
      v3 = *(long *)(v8 * 8 + *(long *)(v5 + -8));
      sub_dd38(v2,v10);
      v9 += v4;
      if (dat_2b338 <= v9) break;
      sub_eaef(v6 + v10,v10 + v3);
      v10 += v3;
      v8 = v1;
    }
    putchar_unlocked((int)dat_2b030);
  }
}

// Function: sub_e86a @ 0xe86a
void sub_e86a(void)
{
  char *v1;
  unsigned long v2; // rax
  long v3; // rax
  unsigned long v4; // stack - 0x48
  unsigned long v5; // stack - 0x40
  long v6; // stack - 0x38
  long v7; // stack - 0x30
  
  v5 = 0;
  v2 = sub_ee73(0);
  v3 = v2 * 0x18 + -0x18 + dat_2b4f0;
  v1 = (char *)*dat_2b348;
  v6 = sub_e530(v1);
  v7 = **(long **)(v3 + 0x10);
  sub_dd38(v1,0);
  for (v4 = 1; v4 < dat_2b338; v4 = v4 + 1) {
    if (v4 % v2) { // branch-flip
      sub_eaef(v6 + v5,v5 + v7);
      v5 += v7;
    }
    else {
      putchar_unlocked((int)dat_2b030);
      v5 = 0;
    }
    v1 = (char *)dat_2b348[v4];
    sub_dd38(v1,v5);
    v6 = sub_e530(v1);
    v7 = *(long *)((v4 % v2) * 8 + *(long *)(v3 + 0x10));
  }
  putchar_unlocked((int)dat_2b030);
}

// Function: sub_e9d6 @ 0xe9d6
void sub_e9d6(char a0)
{
  char *v1;
  unsigned long v2; // rax
  char v3; // stack - 0x29
  unsigned long v4; // stack - 0x28
  unsigned long v5; // stack - 0x20
  
  v5 = 0;
  for (v4 = 0; v4 < dat_2b338; v4 = v4 + 1) {
    v1 = *(char **)(v4 * 8 + dat_2b348);
    if (dat_2b430) // branch-flip
      v2 = sub_e530(v1);
    else {
      v2 = 0;
    }
    if (v4) {
      if ((dat_2b430) && ((dat_2b430 <= v2 + v5 + 2 || ((unsigned long)(-v2 - 3U) < v5)))) {
        v5 = 0;
        v3 = dat_2b030;
      }
      else {
        v5 += 2;
        v3 = ' ';
      }
      putchar_unlocked((int)a0);
      putchar_unlocked((int)v3);
    }
    sub_dd38(v1,v5);
    v5 += v2;
  }
  putchar_unlocked((int)dat_2b030);
}

// Function: sub_eaef @ 0xeaef
void sub_eaef(unsigned long a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x10
  
  v1 = a0;
  while (v1 < a1) {
    if ((dat_2b420) && ((v1 + 1) / dat_2b420 < a1 / dat_2b420)) {
      putchar_unlocked(9);
      v1 += dat_2b420 - v1 % dat_2b420;
    }
    else {
      putchar_unlocked(0x20);
      v1 += 1;
    }
  }
}

// Function: sub_eb94 @ 0xeb94
void sub_eb94(char *a0,char *a1,char *a2)
{
  char *v1;
  char *v2;
  char *v3; // stack - 0x30
  char *v4; // stack - 0x20
  char *v5; // stack - 0x10
  
  v4 = a0;
  v5 = a1;
  if ((*a1 != '.') || (v3 = a2, v4 = a0, v5 = a1, a1[1])) {
    while (*v5) {
      v1 = &v5[1];
      v2 = &v4[1];
      *v4 = *v5;
      v4 = v2;
      v5 = v1;
    }
    v3 = a2;
    if ((v5 <= a1) || (v3 = a2, v5[-1] == '/')) goto label_ec3b;
    v1 = &v4[1];
    *v4 = '/';
    v3 = a2;
    v4 = v1;
  }
label_ec3b:
  while (*v3) {
    v1 = &v3[1];
    v2 = &v4[1];
    *v4 = *v3;
    v3 = v1;
    v4 = v2;
  }
  *v4 = '\0';
}

// Function: sub_ec50 @ 0xec50
void sub_ec50(unsigned long a0)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // stack - 0x40
  unsigned long v5; // stack - 0x38
  long v6; // stack - 0x30
  unsigned long v7; // stack - 0x28
  
  if (dat_2c440 < a0) {
    if ((dat_2b4f8) && (dat_2b4f8 >> 1 <= a0)) {
      dat_2b4f0 = sub_1c638(dat_2b4f0,dat_2b4f8,0x18);
      v5 = dat_2b4f8;
    }
    else {
      dat_2b4f0 = sub_1c638(dat_2b4f0,a0,0x30);
      v5 = a0 * 2;
    }
    v1 = v5 - dat_2c440;
    v2 = v5 + dat_2c440 + 1;
    v3 = v2 * v1;
    if ((v2 < v5) || (v2 != v3 / v1))
      sub_1ccc3(); // no-return
    v6 = sub_1c804(v3 >> 1,8,v3 % v1);
    for (v4 = dat_2c440; v4 < v5; v4 = v4 + 1) {
      *(long *)(dat_2b4f0 + v4 * 0x18 + 0x10) = v6;
      v6 += (v4 + 1) * 8;
    }
    dat_2c440 = v5;
  }
  for (v4 = 0; v4 < a0; v4 = v4 + 1) {
    *(char *)(v4 * 0x18 + dat_2b4f0) = 1;
    *(unsigned long *)(dat_2b4f0 + v4 * 0x18 + 8) = (v4 + 1) * 3;
    for (v7 = 0; v7 <= v4; v7 = v7 + 1) {
      *(unsigned long *)(v7 * 8 + *(long *)(v4 * 0x18 + dat_2b4f0 + 0x10)) = 3;
    }
  }
}

// Function: sub_ee73 @ 0xee73
unsigned long sub_ee73(bool a0) // ternary
{
  long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  long v4; // rdx
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  
  if ((dat_2b4f8) && (dat_2b4f8 < dat_2b338))
    v6 = dat_2b4f8;
  else {
    v6 = dat_2b338;
  }
  sub_ec50(v6);
  for (v5 = 0; v5 < dat_2b338; v5 = v5 + 1) {
    v1 = sub_e530(*(char **)(v5 * 8 + dat_2b348));
    for (v7 = 0; v7 < v6; v7 = v7 + 1) {
      if (*(char *)(v7 * 0x18 + dat_2b4f0)) {
        if (a0) // branch-flip
          v2 = v5 / ((v7 + dat_2b338) / (v7 + 1));
        else {
          v2 = v5 % (v7 + 1);
        }
        v4 = (v2 != v7) ? 2 : 0; // branch-flip
        v3 = v1 + v4;
        if (*(unsigned long *)(v2 * 8 + *(long *)(v7 * 0x18 + dat_2b4f0 + 0x10)) < v3) {
          *(unsigned long *)(v7 * 0x18 + dat_2b4f0 + 8) = *(long *)(v7 * 0x18 + dat_2b4f0 + 8) + (v3 - *(long *)(v2 * 8 + *(long *)(v7 * 0x18 + dat_2b4f0 + 0x10)));
          *(unsigned long *)(*(long *)(v7 * 0x18 + dat_2b4f0 + 0x10) + v2 * 8) = v3;
          *(bool *)(v7 * 0x18 + dat_2b4f0) = *(unsigned long *)(v7 * 0x18 + dat_2b4f0 + 8) < dat_2b430;
        }
      }
    }
  }
  while ((2 <= v6 && (!*(char *)(v6 * 0x18 + -0x18 + dat_2b4f0)))) {
    v6 -= 1;
  }
  return v6;
}

// Function: sub_f135 @ 0xf135
void sub_f135(int a0) // ternary
{
  FILE *v1;
  char *v2;
  
  v2 = dat_2c478;
  if (a0) // branch-flip
    fprintf(stderr,gettext("Try \'%s --help\' for more information.\n"),v2);
  else {
    printf(gettext("Usage: %s [OPTION]... [FILE]...\n"),v2);
    v1 = stdout;
    fputs_unlocked(gettext("List information about the FILEs (the current directory by default).\nSort entries alphabetically if none of -cftuvSUX nor --sort is specified.\n"),v1);
    sub_4f20();
    v1 = stdout;
    fputs_unlocked(gettext("  -a, --all                  do not ignore entries starting with .\n  -A, --almost-all           do not list implied . and ..\n      --author               with -l, print the author of each file\n  -b, --escape               print C-style escapes for nongraphic characters\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --block-size=SIZE      with -l, scale sizes by SIZE when printing them;\n                             e.g., \'--block-size=M\'; see SIZE format below\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -B, --ignore-backups       do not list implied entries ending with ~\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -c                         with -lt: sort by, and show, ctime (time of last\n                             modification of file status information);\n                             with -l: show ctime and sort by name;\n                             otherwise: sort by ctime, newest first\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -C                         list entries by columns\n      --color[=WHEN]         color the output WHEN; more info below\n  -d, --directory            list directories themselves, not their contents\n  -D, --dired                generate output designed for Emacs\' dired mode\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -f                         list all entries in directory order\n  -F, --classify[=WHEN]      append indicator (one of */=>@|) to entries WHEN\n      --file-type            likewise, except do not append \'*\'\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --format=WORD          across -x, commas -m, horizontal -x, long -l,\n                             single-column -1, verbose -l, vertical -C\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --full-time            like -l --time-style=full-iso\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -g                         like -l, but do not list owner\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --group-directories-first\n                             group directories before files;\n                             can be augmented with a --sort option, but any\n                             use of --sort=none (-U) disables grouping\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -G, --no-group             in a long listing, don\'t print group names\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -h, --human-readable       with -l and -s, print sizes like 1K 234M 2G etc.\n      --si                   likewise, but use powers of 1000 not 1024\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -H, --dereference-command-line\n                             follow symbolic links listed on the command line\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --dereference-command-line-symlink-to-dir\n                             follow each command line symbolic link\n                             that points to a directory\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --hide=PATTERN         do not list implied entries matching shell PATTERN\n                             (overridden by -a or -A)\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --hyperlink[=WHEN]     hyperlink file names WHEN\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --indicator-style=WORD\n                             append indicator with style WORD to entry names:\n                             none (default), slash (-p),\n                             file-type (--file-type), classify (-F)\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -i, --inode                print the index number of each file\n  -I, --ignore=PATTERN       do not list implied entries matching shell PATTERN\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -k, --kibibytes            default to 1024-byte blocks for file system usage;\n                             used only with -s and per directory totals\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -l                         use a long listing format\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -L, --dereference          when showing file information for a symbolic\n                             link, show information for the file the link\n                             references rather than for the link itself\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -m                         fill width with a comma separated list of entries\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -n, --numeric-uid-gid      like -l, but list numeric user and group IDs\n  -N, --literal              print entry names without quoting\n  -o                         like -l, but do not list group information\n  -p, --indicator-style=slash\n                             append / indicator to directories\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -q, --hide-control-chars   print ? instead of nongraphic characters\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --show-control-chars   show nongraphic characters as-is (the default,\n                             unless program is \'ls\' and output is a terminal)\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -Q, --quote-name           enclose entry names in double quotes\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --quoting-style=WORD   use quoting style WORD for entry names:\n                             literal, locale, shell, shell-always,\n                             shell-escape, shell-escape-always, c, escape\n                             (overrides QUOTING_STYLE environment variable)\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -r, --reverse              reverse order while sorting\n  -R, --recursive            list subdirectories recursively\n  -s, --size                 print the allocated size of each file, in blocks\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -S                         sort by file size, largest first\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --sort=WORD            sort by WORD instead of name: none (-U), size (-S),\n                             time (-t), version (-v), extension (-X), width\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --time=WORD            change the default of using modification times;\n                               access time (-u): atime, access, use;\n                               change time (-c): ctime, status;\n                               birth time: birth, creation;\n                             with -l, WORD determines which time to show;\n                             with --sort=time, sort by WORD (newest first)\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --time-style=TIME_STYLE\n                             time/date format with -l; see TIME_STYLE below\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -t                         sort by time, newest first; see --time\n  -T, --tabsize=COLS         assume tab stops at each COLS instead of 8\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -u                         with -lt: sort by, and show, access time;\n                             with -l: show access time and sort by name;\n                             otherwise: sort by access time, newest first\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -U                         do not sort; list entries in directory order\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -v                         natural sort of (version) numbers within text\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -w, --width=COLS           set output width to COLS.  0 means no limit\n  -x                         list entries by lines instead of by columns\n  -X                         sort alphabetically by entry extension\n  -Z, --context              print any security context of each file\n      --zero                 end each output line with NUL, not newline\n  -1                         list one file per line\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --help        display this help and exit\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --version     output version information and exit\n"),v1);
    sub_4f51();
    v1 = stdout;
    fputs_unlocked(gettext("\nThe TIME_STYLE argument can be full-iso, long-iso, iso, locale, or +FORMAT.\nFORMAT is interpreted like in date(1).  If FORMAT is FORMAT1<newline>FORMAT2,\nthen FORMAT1 applies to non-recent files and FORMAT2 to recent files.\nTIME_STYLE prefixed with \'posix-\' takes effect only outside the POSIX locale.\nAlso the TIME_STYLE environment variable sets the default style to use.\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("\nThe WHEN argument defaults to \'always\' and can also be \'auto\' or \'never\'.\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("\nUsing color to distinguish file types is disabled both by default and\nwith --color=never.  With --color=auto, ls emits color codes only when\nstandard output is connected to a terminal.  The LS_COLORS environment\nvariable can change the settings.  Use the dircolors(1) command to set it.\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("\nExit status:\n 0  if OK,\n 1  if minor problems (e.g., cannot access subdirectory),\n 2  if serious trouble (e.g., cannot access command-line argument).\n"),v1);
    if (dat_2b210 != 1) { // branch-flip
      v2 = (dat_2b210 != 2) ? "vdir" : "dir"; // branch-flip
    }
    else {
      v2 = "ls";
    }
    sub_4f82(v2);
  }
  exit(a0); // no-return
}

// Function: sub_f7a0 @ 0xf7a0
char * sub_f7a0(char *a0,unsigned long a1)
{
  unsigned long v1; // rax
  char *v2; // rax
  char v3 [136];
  unsigned long v4; // stack - 0xe0
  char *v5; // stack - 0xd8
  char *v6; // stack - 0xd0
  
  v4 = 0x401;
  if (a1) { // branch-flip
    if (a1 <= 0x400)
      v4 = a1 + 1;
  }
  else {
    v4 = 0x80;
  }
  while( true ) {
    v5 = v3;
    v6 = NULL;
    if ((a1) || (v4 != 0x80)) {
      v6 = malloc(v4);
      v5 = v6;
      if (!v6) {
        *__errno_location() = 0xc;
        return NULL;
      }
    }
    v1 = readlink(a0,v5,v4);
    if ((long)v1 <= -1) break;
    if (v1 < v4) {
      v5[v1] = '\0';
      if (v6) { // branch-flip
        if ((v1 + 1 < v4) && (v2 = realloc(v6,v1 + 1), v2))
          v6 = v2;
      }
      else {
        v6 = malloc(v1 + 1);
        if (v6)
          return memcpy(v6,v5,v1 + 1);
      }
      return v6;
    }
    free(v6);
    if (0x4000000000000000 <= v4) { // branch-flip
      if (0x7ffffffffffffffe < v4) {
        *__errno_location() = 0xc;
        return NULL;
      }
      v4 = 0x7fffffffffffffff;
    }
    else {
      v4 <<= 1;
    }
  }
  free(v6);
  return NULL;
}

// Function: sub_fa69 @ 0xfa69
void sub_fa69(void)
{
  sub_f135(1); // no-return
}

// Function: sub_fa7e @ 0xfa7e
long sub_fa7e(char *a0,long a1,long a2,unsigned long a3) // early-return
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

// Function: sub_fbb1 @ 0xfbb1
long sub_fbb1(char *a0,long a1)
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

// Function: sub_fc27 @ 0xfc27
void sub_fc27(char *a0,char *a1,long a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (a2 != -1) // branch-flip
    v1 = gettext("ambiguous argument %s for %s");
  else {
    v1 = gettext("invalid argument %s for %s");
  }
  v2 = sub_1ae9d(1,a0);
  error(0,0,v1,sub_1a9b5(0,8,a1),v2);
}

// Function: sub_fcc0 @ 0xfcc0
void sub_fcc0(long a0,long a1,unsigned long a2)
{
  char *v1;
  FILE *v2;
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
      if (memcmp(v4,(void *)(a2 * v3 + a1),a2)) goto label_fd41;
      v1 = *(char **)(a0 + v3 * 8);
      fprintf(stderr,", %s",(char *)sub_1aeca(v1));
    }
    else {
label_fd41:
      v1 = *(char **)(a0 + v3 * 8);
      fprintf(stderr,"\n  - %s",(char *)sub_1aeca(v1));
      v4 = (void *)(a1 + a2 * v3);
    }
    v3 += 1;
  } while( true );
}

// Function: sub_fe17 @ 0xfe17
long sub_fe17(char *a0,char *a1,long a2,long a3,unsigned long a4,void *a5,unsigned int a6) // early-return, ternary
{
  long v1; // stack - 0x10
  
  v1 = ((char)a6) ? sub_fa7e(a1,a2,a3,a4) : sub_fbb1(a1,a2); // branch-flip
  if (0 <= v1)
    return v1;
  sub_fc27(a0,a1,v1);
  sub_fcc0(a2,a3,a4);
  (*a5)();
  return -1;
}

// Function: sub_fec6 @ 0xfec6
unsigned long sub_fec6(void *a0,long a1,long a2,unsigned long a3)
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

// Function: sub_ff56 @ 0xff56
char * sub_ff56(char *a0)
{
  bool v1;
  char *v2; // stack - 0x18
  char *v3; // stack - 0x10
  
  v1 = 0;
  for (v2 = a0; *v2 == '/'; v2 = &v2[1]) {
  }
  for (v3 = v2; *v3; v3 = &v3[1]) {
    if (*v3 != '/') { // branch-flip
      if (!v1) goto label_ffad;
      v2 = v3;
      v1 = 0;
    }
    else {
      v1 = 1;
    }
label_ffad:
  }
  return v2;
}

// Function: sub_ffc3 @ 0xffc3
unsigned long sub_ffc3(char *a0)
{
  unsigned long v1; // stack - 0x18
  
  for (v1 = strlen(a0); (2 <= v1 && (a0[v1 - 1] == '/')); v1 = v1 - 1) {
  }
  return v1;
}

// Function: sub_10016 @ 0x10016
unsigned long sub_10016(int a0) // return-dupe
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

// Function: sub_10054 @ 0x10054
unsigned long sub_10054(int a0) // return-dupe
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

// Function: sub_10086 @ 0x10086
bool sub_10086(unsigned int a0)
{
  return a0 < 0x80;
}

// Function: sub_100a7 @ 0x100a7
unsigned int sub_100a7(int a0) // return-dupe
{
  if ((a0 != 0x20) && (a0 != 9))
    return 0;
  return 1;
}

// Function: sub_100cf @ 0x100cf
unsigned long sub_100cf(int a0) // return-dupe
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

// Function: sub_100fc @ 0x100fc
bool sub_100fc(int a0)
{
  return (unsigned int)(a0 - 0x30U) < 10;
}

// Function: sub_10120 @ 0x10120
bool sub_10120(int a0)
{
  return (unsigned int)(a0 - 0x21U) < 0x5e;
}

// Function: sub_10144 @ 0x10144
bool sub_10144(int a0)
{
  return (unsigned int)(a0 - 0x61U) < 0x1a;
}

// Function: sub_10168 @ 0x10168
bool sub_10168(int a0)
{
  return (unsigned int)(a0 - 0x20U) < 0x5f;
}

// Function: sub_1018c @ 0x1018c
unsigned long sub_1018c(int a0) // return-dupe, return-dupe x3
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

// Function: sub_101d6 @ 0x101d6
unsigned long sub_101d6(int a0) // return-dupe
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

// Function: sub_10203 @ 0x10203
bool sub_10203(int a0)
{
  return (unsigned int)(a0 - 0x41U) < 0x1a;
}

// Function: sub_10227 @ 0x10227
unsigned long sub_10227(int a0) // return-dupe
{
  if (((unsigned int)(a0 - 0x30U) <= 0x36) && (0x7e0000007e03ffU >> ((unsigned char)(a0 - 0x30U) & 0x3f) & 1))
    return 1;
  return 0;
}

// Function: sub_1026f @ 0x1026f
int sub_1026f(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_10292 @ 0x10292
int sub_10292(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_102b5 @ 0x102b5
int sub_102b5(char *a0,char *a1,long a2) // early-return
{
  unsigned char v1; // al
  unsigned char v2; // al
  long v3; // stack - 0x40
  
  if ((a0 == a1) || (v3 = a2, !a2))
    return 0;
  do {
    v1 = sub_1026f((unsigned char)*a0);
    v2 = sub_1026f((unsigned char)*a1);
    v3 -= 1;
    if ((!v3) || (!v1)) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while (v1 == v2);
  return (unsigned int)v1 - (unsigned int)v2;
}

// Function: sub_10343 @ 0x10343
void sub_10343(struct_14 *a0)
{
  a0->field_0x0 = (long *)&a0[1];
  a0->field_0x8 = 0x400;
}

// Function: sub_10369 @ 0x10369
void sub_10369(long *a0) // return-dupe
{
  if ((long *)*a0 == &a0[2])
    return;
  free((void *)*a0);
}

// Function: sub_1039b @ 0x1039b
unsigned long sub_1039b(char *a0)
{
  int v1; // eax
  
  v1 = faccessat(-100,a0,0,0x200);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}

// Function: sub_103cd @ 0x103cd
unsigned long sub_103cd(char *a0)
{
  char *v1;
  char *v2;
  char *v3; // stack - 0x10
  
  v1 = a0;
  do {
    v3 = v1;
    v2 = v3;
    if (*v3 != '/')
      return 0;
    do {
      v3 = v2;
      v2 = &v3[1];
    } while (*v2 == '/');
    v1 = &v3[2];
    if (!*v2)
      return 1;
    if (*v2 != '.')
      return 0;
  } while ((*v1) && ((*v1 != '.' || ((v3[3] && (v3[3] != '/'))))));
  return 1;
}

// Function: sub_10462 @ 0x10462
void sub_10462(char *a0,char *a1)
{
  strcpy(a1,"/");
  sub_1039b(a0);
}

// Function: sub_1049a @ 0x1049a
unsigned int sub_1049a(unsigned int a0)
{
  a0 = a0 - 1 & a0;
  return CONCAT31((undefined3)(a0 >> 8),a0 != 0);
}

// Function: sub_104b5 @ 0x104b5
bool sub_104b5(long *a0,char *a1,struct_8 *a2)
{
  struct_9 *v1;
  bool v2; // zf
  
  if (!*a0) {
    *a0 = sub_1298f(7,NULL,sub_13a6f,sub_13abf,sub_13b3b);
    if (!*a0)
      sub_1ccc3(); // no-return
  }
  v1 = (struct_9 *)*a0;
  v2 = sub_1137d(v1,a1,a2) == '\0';
  if (v2)
    sub_112dd((char *)*a0,a1,a2);
  return !v2;
}

// Function: sub_1056e @ 0x1056e
long sub_1056e(char *a0,unsigned int a1,struct_14 *a2) // early-return x4, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_1056e
{
  long *v1;
  void *v10; // stack - 0x838
  char *v11; // stack - 0x428
  stat v12;
  long v13; // stack - 0x988
  bool v14; // zf
  int v15; // stack - 0x990
  long *v16; // stack - 0x980
  char *v17; // stack - 0x978
  char *v18; // stack - 0x970
  long *v19; // stack - 0x968
  int v2;
  long v20; // stack - 0x960
  char *v21; // stack - 0x958
  unsigned long v22; // stack - 0x950
  void *v23; // stack - 0x948
  long v24; // stack - 0x940
  long *v25; // stack - 0x938
  unsigned long v26; // stack - 0x930
  long v27; // stack - 0x928
  unsigned long v28; // stack - 0x920
  long v29; // stack - 0x918
  bool v3;
  long v30; // stack - 0x910
  long v31; // stack - 0x908
  struct_14 *v32; // stack - 0x900
  struct_14 *v33; // stack - 0x8f8
  char **v34; // stack - 0x8f0
  void **v35; // stack - 0x8e8
  struct_14 *v36; // stack - 0x8e0
  unsigned long v37; // stack - 0x8d8
  long v38; // stack - 0x8d0
  unsigned long v39; // stack - 0x830
  bool v4;
  long v40; // stack - 0x420
  char v5; // al
  unsigned char v6; // al
  char *v7;
  char *v8; // rax
  unsigned long v9;
  
  v13 = 0;
  v14 = (a1 & 4) != 0;
  v15 = 0;
  a1 &= 3;
  if (sub_1049a(a1)) {
    *__errno_location() = 0x16;
    return 0;
  }
  if (!a0) {
    *__errno_location() = 0x16;
    return 0;
  }
  if (!*a0) {
    *__errno_location() = 2;
    return 0;
  }
  sub_10343(&v10);
  sub_10343(&v11);
  sub_10343(a2);
  v19 = a2->field_0x0;
  v3 = 0;
  v4 = 1;
  v20 = 0;
  v25 = v19;
  if (*a0 != '/') { // branch-flip
    while (v9 = a2->field_0x8, !getcwd((char *)v19,v9)) {
      v2 = *__errno_location();
      if (v2 == 0xc)
        sub_1ccc3(); // no-return, return-dupe
      if (v2 != 0x22) {
        v16 = v19;
        goto label_10f11;
      }
      v32 = a2;
      if (!sub_1afd3(a2)) {
        sub_1ccc3();
      }
      v19 = a2->field_0x0;
    }
    v16 = rawmemchr(v19,0);
    v20 = 0;
    v17 = a0;
  }
  else {
    v8 = mempcpy(v19,a0,0);
    v16 = (long *)&v8[1];
    *v8 = 0x2f;
    v17 = &a0[v20];
  }
  while (*v17) {
    for (; *v17 == '/'; v17 = &v17[1]) {
    }
    for (v18 = v17; (*v18 && (*v18 != '/')); v18 = &v18[1]) {
    }
    v26 = (long)v18 - (long)v17;
    if (!v26) break;
    if ((v26 != 1) || (*v17 != '.')) {
      if ((v26 != 2) || ((*v17 != '.' || (v17[1] != '.')))) {
        if (*(char *)((long)v16 + -1) != '/') {
          v1 = (long *)((long)v16 + 1);
          *(char *)v16 = '/';
          v16 = v1;
        }
        while ((char *)((long)v19 + (a2->field_0x8 - (long)v16)) < (char *)(v26 + 2)) {
          v30 = (long)v16 - (long)v19;
          v33 = a2;
          if (!sub_1b0a1(a2))
            sub_1ccc3(); // no-return
          v19 = a2->field_0x0;
          v16 = (long *)((long)v19 + v30);
        }
        v16 = mempcpy(v16,v17,v26);
        *(char *)v16 = '\0';
        v22 = 0xffffffffffffffff;
        if (!v14) {
          while( true ) {
            v21 = v11;
            v27 = v40;
            v22 = readlink((char *)v19,v11,v40 - 1);
            if ((long)v22 < v27 + -1) break;
            v34 = &v11;
            v5 = sub_1afd3(v34);
            if (!v5)
              sub_1ccc3(); // no-return
          }
        }
        if (0 <= (long)v22) { // branch-flip
          if (0x14 <= v15) { // branch-flip
            if (*v17) {
              *(char *)((long)v16 - v26) = 0;
              if ((char)*v19) // branch-flip
                v7 = (char *)v19;
              else {
                v7 = ".";
              }
              if (stat(v7,&v12)) goto label_10f11;
              *(char *)((long)v16 - v26) = *v17;
              v5 = sub_104b5(&v13,v17,&v12);
              if (v5) {
                if (a1 == 2) goto label_10ea9;
                *__errno_location() = 0x28;
                goto label_10f11;
              }
            }
          }
          else {
            v15 += 1;
          }
          v21[v22] = '\0';
          v23 = v10;
          if (v3)
            v24 = (long)v18 - (long)v10;
          v28 = strlen(v18);
          if (SCARRY8(v22,v28 + 0x8000000000000000))
            sub_1ccc3(); // no-return
          while (v39 <= v28 + v22) {
            v35 = &v10;
            v5 = sub_1b0a1(v35);
            if (!v5)
              sub_1ccc3(); // no-return
            v23 = v10;
          }
          if (v3)
            v18 = (char *)((long)v23 + v24);
          memmove((void *)(v22 + (long)v23),v18,v28 + 1);
          v18 = memcpy(v23,v21,v22);
          v3 = 1;
          if (*v21 != '/') { // branch-flip
            v1 = v16;
            if ((long *)((long)v19 + v20 + 1) < v16) {
              do {
                v16 = (long *)((long)v1 + -1);
                if (v16 <= v19) break;
                v7 = (char *)((long)v1 + -2);
                v1 = v16;
              } while (*v7 != '/');
            }
          }
          else {
            v29 = 0;
            v8 = mempcpy(v19,v21,0);
            v16 = (long *)&v8[1];
            *v8 = 0x2f;
            v20 = v29;
          }
        }
        else if (a1 != 2) {
          v5 = sub_103cd(v18);
          if (v5) { // branch-flip
            v6 = sub_10462(v19,v16);
            v6 ^= 1;
          }
          else if (v14) {
            v6 = !((*v18) || (v5 = sub_1039b(v19), v5 == '\x01'));
          }
          else {
            v6 = *__errno_location() != 0x16;
          }
          if ((v6) && (((a1 != 1 || (*__errno_location() != 2)) || (v9 = strspn(v18,"/"), v18[v9])))) goto label_10f11;
        }
      }
      else {
        v1 = v16;
        if ((long *)((long)v19 + v20 + 1) < v16) {
          do {
            v16 = (long *)((long)v1 + -1);
            if (v16 <= v19) break;
            v7 = (char *)((long)v1 + -2);
            v1 = v16;
          } while (*v7 != '/');
        }
      }
    }
label_10ea9:
    v17 = v18;
  }
  if (((long *)((long)v19 + v20 + 1) < v16) && (*(char *)((long)v16 + -1) == '/'))
    v16 = (long *)((long)v16 + -1);
  v4 = 0;
label_10f11:
  if (v13)
    sub_12c0f(v13);
  sub_10369(&v10);
  sub_10369(&v11);
  if (!v4) {
    v7 = (char *)((long)v16 + 1);
    *(char *)v16 = '\0';
    v37 = (long)v7 - (long)v19;
    v16 = (long *)v7;
    v36 = a2;
    v38 = sub_1aeed(a2,v37);
    if (v38) // branch-flip
      v31 = v38;
    else {
      v31 = 0;
    }
    if (!v31)
      sub_1ccc3(); // no-return
    return v31;
  }
  sub_10369(a2);
  return 0;
}

// Function: sub_1101b @ 0x1101b
void sub_1101b(char *a0,unsigned int a1)
{
  char v1 [1048];
  
  sub_1056e(a0,a1,v1);
}

// Function: sub_1107a @ 0x1107a
void sub_1107a(unsigned long a0)
{
  dat_2c448 = a0;
}

// Function: sub_11094 @ 0x11094
void sub_11094(char a0)
{
  dat_2c450 = a0;
}

// Function: sub_110ae @ 0x110ae
void sub_110ae(void)
{
  int v1; // eax
  char *v2; // rax
  char *v3; // rax
  
  v1 = sub_1e7ed(stdout);
  if (v1) {
    if (dat_2c450 == '\x01') {
      if (*__errno_location() == 0x20) goto label_1117f;
    }
    v2 = gettext("write error");
    if (dat_2c448) { // branch-flip
      v3 = (char *)sub_1abc4(dat_2c448);
      error(0,*__errno_location(),"%s: %s",v3,v2);
    }
    else {
      error(0,*__errno_location(),"%s",v2);
    }
    _exit(dat_2b228); // no-return
  }
label_1117f:
  v1 = sub_1e7ed(stderr);
  if (v1)
    _exit(dat_2b228); // no-return
}

// Function: sub_111a6 @ 0x111a6
unsigned long sub_111a6(char *a0)
{
  char v1;
  unsigned long v2; // stack - 0x18
  
  v1 = *a0;
  for (v2 = sub_ff56(a0) - (long)a0; ((v1 == '/') < v2 && (a0[v2 - 1] == '/')); v2 = v2 - 1) {
  }
  return v2;
}

// Function: sub_1122b @ 0x1122b
char * sub_1122b(char *a0) // early-return
{
  char *v1; // rax
  unsigned long v2; // stack - 0x18
  
  v2 = sub_111a6(a0);
  v1 = malloc(v2 + (v2 == 0) + 1);
  if (!v1)
    return NULL;
  memcpy(v1,a0,v2);
  if (!v2) {
    v2 = 1;
    *v1 = 0x2e;
  }
  v1[v2] = 0;
  return v1;
}

// Function: sub_112c6 @ 0x112c6
unsigned long sub_112c6(void)
{
  return 0;
}

// Function: sub_112dd @ 0x112dd
void sub_112dd(char *a0,char *a1,struct_8 *a2) // return-dupe x2
{
  unsigned long *v1; // rax
  unsigned long *v2; // rax
  
  if (!a0)
    return;
  v1 = (unsigned long *)sub_1c68b(0x18);
  *v1 = sub_1cc92(a1);
  v1[1] = a2->field_0x8;
  v1[2] = a2->field_0x0;
  v2 = (unsigned long *)sub_136d0(a0,v1);
  if (v2) {
    if (v2 == v1)
      return;
    sub_13b3b(v1);
    return;
  }
  sub_1ccc3(); // no-return
}

// Function: sub_1137d @ 0x1137d
unsigned long sub_1137d(struct_9 *a0,unsigned long a1,struct_8 *a2) // early-return
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
  v1 = sub_122da(a0,&v2);
  return CONCAT71((undefined7)(v1 >> 8),v1 != 0);
}

// Function: sub_11400 @ 0x11400
unsigned long sub_11400(unsigned int a0) // return-dupe x7
{
  if ((a0 & 0xf000) == 0x8000)
    return 0x2d;
  if ((a0 & 0xf000) == 0x4000)
    return 100;
  if ((a0 & 0xf000) == 0x6000)
    return 0x62;
  if ((a0 & 0xf000) == 0x2000)
    return 99;
  if ((a0 & 0xf000) == 0xa000)
    return 0x6c;
  if ((a0 & 0xf000) == 0x1000)
    return 0x70;
  if ((a0 & 0xf000) == 0xc000)
    return 0x73;
  return 0x3f;
}

// Function: sub_114af @ 0x114af
void sub_114af(unsigned int a0,char *a1) // ternary x12
{
  char v1; // al
  
  *a1 = sub_11400(a0);
  v1 = (a0 & 0x100) ? 0x72 : 0x2d; // branch-flip
  a1[1] = v1;
  v1 = (a0 & 0x80) ? 0x77 : 0x2d; // branch-flip
  a1[2] = v1;
  if (a0 & 0x800) { // branch-flip
    v1 = (a0 & 0x40) ? 0x73 : 0x53; // branch-flip
  }
  else {
    v1 = (a0 & 0x40) ? 0x78 : 0x2d; // branch-flip
  }
  a1[3] = v1;
  v1 = (a0 & 0x20) ? 0x72 : 0x2d; // branch-flip
  a1[4] = v1;
  v1 = (a0 & 0x10) ? 0x77 : 0x2d; // branch-flip
  a1[5] = v1;
  if (a0 & 0x400) { // branch-flip
    v1 = (a0 & 8) ? 0x73 : 0x53; // branch-flip
  }
  else {
    v1 = (a0 & 8) ? 0x78 : 0x2d; // branch-flip
  }
  a1[6] = v1;
  v1 = (a0 & 4) ? 0x72 : 0x2d; // branch-flip
  a1[7] = v1;
  v1 = (a0 & 2) ? 0x77 : 0x2d; // branch-flip
  a1[8] = v1;
  if (a0 & 0x200) { // branch-flip
    v1 = (a0 & 1) ? 0x74 : 0x54; // branch-flip
  }
  else {
    v1 = (a0 & 1) ? 0x78 : 0x2d; // branch-flip
  }
  a1[9] = v1;
  a1[10] = 0x20;
  a1[0xb] = 0;
}

// Function: sub_1167b @ 0x1167b
void sub_1167b(struct_25 *a0,char *a1)
{
  sub_114af(a0->field_0x18,a1);
}

// Function: sub_116a7 @ 0x116a7
long sub_116a7(char *a0,char *a1,unsigned long *a2)
{
  long v1; // rax
  
  v1 = sub_116ec(a0,a1,a2);
  if (!v1)
    sub_1ccc3(); // no-return
  return v1;
}

// Function: sub_116ec @ 0x116ec
void * sub_116ec(void *a0,char *a1,unsigned long *a2) // early-return
{
  char *v1; // rax
  long v2; // rax
  unsigned long v3; // rax
  void *v4; // rax
  char v5; // stack - 0x39
  
  v1 = (char *)sub_ff56(a0);
  v2 = sub_ffc3(v1);
  v1 = &v1[v2 - (long)a0];
  v3 = strlen(a1);
  v5 = '\0';
  if (v2) { // branch-flip
    if ((*(char *)((long)a0 + (long)&v1[-1]) != '/') && (*a1 != '/'))
      v5 = '/';
  }
  else if (*a1 == '/')
    v5 = '.';
  v4 = malloc((unsigned long)&v1[v3 + (v5 != '\0') + 1]);
  if (!v4)
    return NULL;
  v1 = mempcpy(v4,a0,(unsigned long)v1);
  *v1 = v5;
  if (a2)
    *a2 = &v1[v5 != '\0'];
  *(char *)mempcpy(&v1[v5 != '\0'],a1,v3) = 0;
  return v4;
}

// Function: sub_1182e @ 0x1182e
unsigned long sub_1182e(char *a0,unsigned long *a1) // ternary
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
    while (((v5 + 1 < v2 && (a0[v5] == '.')) && ((v1 = a0[v5 + 1], sub_10054((int)v1) || (a0[v5 + 1] == '~'))))) {
      for (v5 = v5 + 2; (v5 < v2 && ((v1 = a0[v5], sub_10016((int)v1) || (a0[v5] == '~')))); v5 = v5 + 1) {
      }
    }
    v4 = (0 <= (long)*a1) ? v5 == v2 : a0[v5] == '\0'; // branch-flip
    if (v4) break;
    v5 += 1;
    v3 = v5;
  }
  *a1 = v5;
  return v3;
}

// Function: sub_1194f @ 0x1194f
unsigned int sub_1194f(long a0,long a1,long a2) // early-return x3
{
  unsigned char v1;
  unsigned int v2; // eax
  
  if (a1 == a2)
    return 0xffffffff;
  v1 = *(unsigned char *)(a0 + a1);
  if (sub_100fc(v1))
    return 0;
  if (sub_10054(v1)) // branch-flip
    v2 = (unsigned int)v1;
  else {
    if (v1 == 0x7e)
      return 0xfffffffe;
    v2 = v1 + 0x100;
  }
  return v2;
}

// Function: sub_119cc @ 0x119cc
int sub_119cc(long a0,long a1,long a2,long a3) // early-return x3
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
    while (((v5 < a1 && (v1 = *(char *)(a0 + v5), sub_100fc((int)v1) != '\x01')) || ((v6 < a3 && (v1 = *(char *)(a2 + v6), sub_100fc((int)v1) != '\x01'))))) {
      v2 = sub_1194f(a0,v5,a1);
      v3 = sub_1194f(a2,v6,a3);
      if (v2 != v3)
        return v2 - v3;
      v5 += 1;
      v6 += 1;
    }
    for (; (v5 < a1 && (*(char *)(a0 + v5) == '0')); v5 = v5 + 1) {
    }
    for (; (v6 < a3 && (*(char *)(a2 + v6) == '0')); v6 = v6 + 1) {
    }
    while ((((v5 < a1 && (v6 < a3)) && (v1 = *(char *)(a0 + v5), sub_100fc((int)v1))) && (v1 = *(char *)(a2 + v6), sub_100fc((int)v1)))) {
      if (!v4)
        v4 = (int)*(char *)(a0 + v5) - (int)*(char *)(a2 + v6);
      v5 += 1;
      v6 += 1;
    }
    if ((v5 < a1) && (v1 = *(char *)(a0 + v5), sub_100fc((int)v1))) break;
    if ((v6 < a3) && (v1 = *(char *)(a2 + v6), sub_100fc((int)v1)))
      return -1;
    if (v4)
      return v4;
  }
  return 1;
}

// Function: sub_11c04 @ 0x11c04
void sub_11c04(char *a0,char *a1)
{
  sub_11c38(a0,-1,a1,-1);
}

// Function: sub_11c38 @ 0x11c38
unsigned long sub_11c38(char *a0,long a1,char *a2,long a3) // early-return x5, ternary x4
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
    goto label_11e28;
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
label_11d9a:
    v4 = 0;
  }
  else {
    if (0 <= a1) { // branch-flip
      if (a1 != 2) goto label_11d9a;
    }
    else if (a0[2]) goto label_11d9a;
    v4 = 1;
  }
  if (a2[1] != '.') { // branch-flip
label_11de0:
    v13 = 0;
  }
  else {
    if (0 <= a3) { // branch-flip
      if (a3 != 2) goto label_11de0;
    }
    else if (a2[2]) goto label_11de0;
    v13 = 1;
  }
  if (v4)
    return (unsigned long)-(unsigned int)(v13 ^ 1);
  v10 = 0;
  v11 = 0;
  v12 = 0;
  if (v13)
    return 1;
label_11e28:
  v3 = a3;
  v6 = a2;
  v2 = a1;
  v7 = a0;
  v16 = sub_1182e(a0,&v2);
  v17 = sub_1182e(v6,&v3);
  if ((v16 != v2) || (v17 != v3))
    v14 = '\0';
  else {
    v14 = '\x01';
  }
  v15 = sub_119cc(v7,v16,v6,v17);
  if ((v15) || (v14))
    v1 = (unsigned long)v15;
  else {
    v1 = sub_119cc(v7,v2,v6,v3);
  }
  return v1;
}

// Function: sub_11ec3 @ 0x11ec3
void sub_11ec3(timespec *a0)
{
  clock_gettime(0,a0);
}

// Function: sub_11ee7 @ 0x11ee7
undefined16 sub_11ee7(void)
{
  char v1 [16]; // stack - 0x28
  
  sub_11ec3(v1);
  return v1._0_16_;
}

// Function: sub_11f2c @ 0x11f2c
unsigned int sub_11f2c(unsigned int a0) // return-dupe
{
  char v1 [264];
  
  if (sub_1b3bf(a0,v1,0x101))
    return 0;
  if ((strcmp(v1,"C")) && (strcmp(v1,"POSIX")))
    return 1;
  return 0;
}

// Function: sub_11fd6 @ 0x11fd6
unsigned long sub_11fd6(struct_26 *a0)
{
  return a0->field_0x10;
}

// Function: sub_11fec @ 0x11fec
unsigned long sub_11fec(struct_27 *a0)
{
  return a0->field_0x18;
}

// Function: sub_12002 @ 0x12002
unsigned long sub_12002(struct_19 *a0)
{
  return a0->field_0x20;
}

// Function: sub_12018 @ 0x12018
unsigned long sub_12018(struct_28 *a0)
{
  long *v1; // stack - 0x28
  unsigned long v2; // stack - 0x20
  long *v3; // stack - 0x18
  unsigned long v4; // stack - 0x10
  
  v2 = 0;
  for (v1 = a0->field_0x0; v1 < a0->field_0x8; v1 = &v1[2]) {
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

// Function: sub_1209a @ 0x1209a
unsigned long sub_1209a(struct_28 *a0) // return-dupe
{
  long *v1; // stack - 0x28
  long v2; // stack - 0x20
  long v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = 0;
  for (v1 = a0->field_0x0; v1 < a0->field_0x8; v1 = &v1[2]) {
    if (*v1) {
      v4 = v1;
      v2 += 1;
      do {
        v3 += 1;
        v4 = (long *)v4[1];
      } while (v4);
    }
  }
  if ((v2 == a0->field_0x18) && (v3 == a0->field_0x20))
    return 1;
  return 0;
}

// Function: sub_12138 @ 0x12138
void sub_12138(struct_19 *a0,FILE *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_12002(a0);
  v2 = sub_11fd6(a0);
  v3 = sub_11fec(a0);
  v4 = sub_12018(a0);
  fprintf(a1,"# entries:         %lu\n",v1);
  fprintf(a1,"# buckets:         %lu\n",v2);
  fprintf(a1,"# buckets used:    %lu (%.2f%%)\n",v3,(dat_234f8 * (double)v3) / (double)v2);
  fprintf(a1,"max bucket length: %lu\n",v4);
}

// Function: sub_1227f @ 0x1227f
long sub_1227f(struct_7 *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*a0->field_0x30)(a1,a0->field_0x10);
  if ((unsigned long)a0->field_0x10 <= v1)
    abort(); // no-return
  return v1 * 0x10 + a0->field_0x0;
}

// Function: sub_122da @ 0x122da
long sub_122da(struct_9 *a0,long a1) // return-dupe
{
  void *v1;
  long v2;
  long *v3; // stack - 0x18
  
  v3 = (long *)sub_1227f(a0,a1);
  if (!*v3)
    return 0;
  while( true ) {
    if (!v3)
      return 0;
    if ((a1 == *v3) || (v1 = a0->field_0x38, v2 = *v3, (*v1)(a1,v2))) break;
    v3 = (long *)v3[1];
  }
  return *v3;
}

// Function: sub_12371 @ 0x12371
long sub_12371(struct_28 *a0) // early-return
{
  long *v1; // stack - 0x10
  
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

// Function: sub_123d1 @ 0x123d1
long sub_123d1(char *a0,long a1) // early-return
{
  long *v1; // stack - 0x18
  long *v2; // stack - 0x10
  
  v1 = (long *)sub_1227f(a0,a1);
  v2 = v1;
  while ((a1 != *v2 || (!v2[1]))) {
    v2 = (long *)v2[1];
    if (!v2) {
      do {
        v1 = &v1[2];
        if (*(long **)&a0[8] <= v1)
          return 0;
      } while (!*v1);
      return *v1;
    }
  }
  return *(long *)v2[1];
}

// Function: sub_1246f @ 0x1246f
unsigned long sub_1246f(struct_28 *a0,long a1,unsigned long a2)
{
  unsigned long v1;
  unsigned long v2; // stack - 0x20
  long *v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = a0->field_0x0;
  do {
    if (a0->field_0x8 <= v3)
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

// Function: sub_1250f @ 0x1250f
long sub_1250f(struct_28 *a0,void *a1,unsigned long a2)
{
  long v1;
  long v2; // stack - 0x20
  long *v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = a0->field_0x0;
  do {
    if (a0->field_0x8 <= v3)
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

// Function: sub_125a7 @ 0x125a7
unsigned long sub_125a7(unsigned char *a0,unsigned long a1)
{
  unsigned char *v1; // stack - 0x20
  unsigned long v2; // stack - 0x10
  
  v2 = 0;
  for (v1 = a0; *v1; v1 = &v1[1]) {
    v2 = ((unsigned long)*v1 + v2 * 0x1f) % a1;
  }
  return v2;
}

// Function: sub_12601 @ 0x12601
unsigned long sub_12601(unsigned long a0)
{
  unsigned long v1; // stack - 0x18
  unsigned long v2; // stack - 0x10
  
  v2 = 9;
  for (v1 = 3; (v2 < a0 && (a0 % v1)); v1 = v1 + 2) {
    v2 += (v1 + 1) * 4;
  }
  return CONCAT71((undefined7)(a0 % v1 >> 8),a0 % v1 != 0);
}

// Function: sub_12670 @ 0x12670
unsigned long sub_12670(unsigned long a0)
{
  unsigned long v1; // stack - 0x10
  
  v1 = a0;
  if (a0 <= 9)
    v1 = 10;
  v1 |= 1;
  while( true ) {
    if (v1 == 0xffffffffffffffff)
      return 0xffffffffffffffff;
    if (sub_12601(v1) == '\x01') break;
    v1 += 2;
  }
  return v1;
}

// Function: sub_126bb @ 0x126bb
void sub_126bb(struct_29 *a0)
{
  a0->field_0x0 = 0;
  a0->field_0x4 = dat_23500;
  a0->field_0x8 = dat_23504;
  a0->field_0xc = dat_23508;
  a0->field_0x10 = 0;
}

// Function: sub_12711 @ 0x12711
unsigned long sub_12711(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_1e664(a0,3) % a1;
}

// Function: sub_1274c @ 0x1274c
unsigned long sub_1274c(long a0,long a1)
{
  return CONCAT71((undefined7)((unsigned long)a0 >> 8),a0 == a1);
}

// Function: sub_12769 @ 0x12769
unsigned long sub_12769(struct_5 *a0) // return-dupe x2
{
  float *v1;
  
  v1 = a0->field_0x28;
  if (v1 == (float *)0x23470)
    return 1;
  if ((((dat_2350c < v1[2]) && (v1[2] < dat_23500 - dat_2350c)) && (dat_2350c + dat_23500 < v1[3])) && (((0.0 <= *v1 && (*v1 + dat_2350c < v1[1])) && ((v1[1] <= dat_23500 && (*v1 + dat_2350c < v1[2]))))))
    return 1;
  a0->field_0x28 = (float *)0x23470;
  return 0;
}

// Function: sub_12876 @ 0x12876
unsigned long sub_12876(unsigned long a0,char *a1) // early-return x2
{
  unsigned long v1; // rax
  float v2; // xmm0_da
  unsigned long v3; // stack - 0x20
  
  v3 = a0;
  if (a1[0x10] != '\x01') {
    v2 = (float)a0 / *(float *)&a1[8];
    if (dat_23510 <= v2)
      return 0;
    if (dat_23514 <= v2)
      v3 = (long)(v2 - dat_23514) ^ 0x8000000000000000;
    else {
      v3 = (unsigned long)v2;
    }
  }
  v1 = sub_12670(v3);
  if (!((long)(v1 << 3) <= -1 || v1 >> 0x3d))
    return v1;
  return 0;
}

// Function: sub_1298f @ 0x1298f
long * sub_1298f(unsigned long a0,char *a1,void *a2,void *a3,long a4) // early-return x2
{
  unsigned long v1;
  long *v2; // rax
  void *v3; // stack - 0x38
  void *v4; // stack - 0x30
  char *v5; // stack - 0x28
  
  v4 = a2;
  if (!a2)
    v4 = sub_12711;
  v3 = a3;
  if (!a3)
    v3 = sub_1274c;
  v2 = malloc(0x50);
  if (!v2)
    return NULL;
  v5 = a1;
  if (!a1)
    v5 = (char *)0x23470;
  v2[5] = (long)v5;
  if (sub_12769(v2) == '\x01') {
    v2[2] = sub_12876(a0,v5);
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

// Function: sub_12b04 @ 0x12b04
void sub_12b04(struct_30 *a0)
{
  unsigned long *v1;
  long *v2; // stack - 0x20
  unsigned long *v3; // stack - 0x18
  
  for (v2 = a0->field_0x0; v2 < a0->field_0x8; v2 = &v2[2]) {
    if (*v2) {
      v3 = (unsigned long *)v2[1];
      while (v3) {
        if (a0->field_0x40)
          (*(void *)a0->field_0x40)(*v3);
        *v3 = 0;
        v1 = (unsigned long *)v3[1];
        v3[1] = a0->field_0x48;
        a0->field_0x48 = v3;
        v3 = v1;
      }
      if (a0->field_0x40)
        (*(void *)a0->field_0x40)(*v2);
      *v2 = 0;
      v2[1] = 0;
    }
  }
  a0->field_0x18 = 0;
  a0->field_0x20 = 0;
}

// Function: sub_12c0f @ 0x12c0f
void sub_12c0f(struct_16 *a0)
{
  void *v1;
  long *v2; // stack - 0x20
  long *v3; // stack - 0x18
  
  if ((a0->field_0x40) && (a0->field_0x20)) {
    for (v2 = a0->field_0x0; v2 < a0->field_0x8; v2 = &v2[2]) {
      if (*v2) {
        for (v3 = v2; v3; v3 = (long *)v3[1]) {
          (*(void *)a0->field_0x40)(*v3);
        }
      }
    }
  }
  for (v2 = a0->field_0x0; v2 < a0->field_0x8; v2 = &v2[2]) {
    v3 = (void *)v2[1];
    while (v3) {
      v1 = *(void **)((long)v3 + 8);
      free(v3);
      v3 = v1;
    }
  }
  v3 = a0->field_0x48;
  while (v3) {
    v1 = *(void **)((long)v3 + 8);
    free(v3);
    v3 = v1;
  }
  free(a0->field_0x0);
  free(a0);
}

// Function: sub_12d3e @ 0x12d3e
void * sub_12d3e(struct_6 *a0)
{
  void *v1; // stack - 0x10
  
  if (a0->field_0x48) { // branch-flip
    v1 = (void *)a0->field_0x48;
    a0->field_0x48 = *(unsigned long *)((long)v1 + 8);
  }
  else {
    v1 = malloc(0x10);
  }
  return v1;
}

// Function: sub_12d8d @ 0x12d8d
void sub_12d8d(struct_6 *a0,struct_8 *a1)
{
  a1->field_0x0 = 0;
  a1->field_0x8 = a0->field_0x48;
  a0->field_0x48 = a1;
}

// Function: sub_12dc7 @ 0x12dc7
long sub_12dc7(struct_9 *a0,long a1,unsigned long *a2,bool a3) // early-return
{
  void *v1;
  long v2;
  struct_8 *v3;
  long v4;
  unsigned long v5;
  long *v6; // stack - 0x38
  
  v6 = (long *)sub_1227f(a0,a1);
  *a2 = v6;
  if (!*v6)
    return 0;
  if ((a1 != *v6) && (v1 = a0->field_0x38, v2 = *v6, !(*v1)(a1,v2))) {
    while( true ) {
      if (!v6[1])
        return 0;
      if ((a1 == *(long *)v6[1]) || (v1 = a0->field_0x38, v5 = *(unsigned long *)v6[1], (*v1)(a1,v5))) break;
      v6 = (long *)v6[1];
    }
    v2 = *(long *)v6[1];
    if (a3) {
      v3 = (struct_8 *)v6[1];
      v6[1] = v3->field_0x8;
      sub_12d8d(a0,v3);
    }
    return v2;
  }
  v2 = *v6;
  if (a3) {
    if (v6[1]) { // branch-flip
      v3 = (struct_8 *)v6[1];
      v4 = v3->field_0x8;
      *v6 = v3->field_0x0;
      v6[1] = v4;
      sub_12d8d(a0,v3);
    }
    else {
      *v6 = 0;
    }
  }
  return v2;
}

// Function: sub_12f60 @ 0x12f60
unsigned long sub_12f60(struct_11 *a0,struct_28 *a1,bool a2)
{
  long *v1;
  long v2;
  struct_0 *v3; // rax
  struct_0 *v4; // rax
  long *v5; // stack - 0x38
  long *v6; // stack - 0x30
  
  v5 = a1->field_0x0;
  do {
    if (a1->field_0x8 <= v5)
      return 1;
    if (*v5) {
      v6 = (long *)v5[1];
      while (v6) {
        v2 = *v6;
        v3 = (struct_0 *)sub_1227f(a0,v2);
        v1 = (long *)v6[1];
        if (v3->field_0x0) { // branch-flip
          v6[1] = v3->field_0x8;
          v3->field_0x8 = (long)v6;
        }
        else {
          v3->field_0x0 = v2;
          a0->field_0x18 = a0->field_0x18 + 1;
          sub_12d8d(a0,v6);
        }
        v6 = v1;
      }
      v2 = *v5;
      v5[1] = 0;
      if (!a2) {
        v3 = (struct_0 *)sub_1227f(a0,v2);
        if (v3->field_0x0) { // branch-flip
          v4 = (struct_0 *)sub_12d3e(a0);
          if (!v4)
            return 0;
          v4->field_0x0 = v2;
          v4->field_0x8 = v3->field_0x8;
          v3->field_0x8 = (long)v4;
        }
        else {
          v3->field_0x0 = v2;
          a0->field_0x18 = a0->field_0x18 + 1;
        }
        *v5 = 0;
        a1->field_0x18 = a1->field_0x18 + -1;
      }
    }
    v5 = &v5[2];
  } while( true );
}

// Function: sub_13134 @ 0x13134
unsigned long sub_13134(struct_12 *a0,unsigned long a1) // return-dupe x4
{
  void *v1; // stack - 0x68
  unsigned long v10; // stack - 0x20
  void *v2; // stack - 0x60
  unsigned long v3; // stack - 0x58
  unsigned long v4; // stack - 0x50
  unsigned long v5; // stack - 0x48
  char *v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  unsigned long v8; // stack - 0x30
  unsigned long v9; // stack - 0x28
  
  v3 = sub_12876(a1,a0->field_0x28);
  if (!v3)
    return 0;
  if (v3 == a0->field_0x10)
    return 1;
  v1 = calloc(v3,0x10);
  if (!v1)
    return 0;
  v2 = (void *)((long)v1 + v3 * 0x10);
  v4 = 0;
  v5 = 0;
  v6 = a0->field_0x28;
  v7 = a0->field_0x30;
  v8 = a0->field_0x38;
  v9 = a0->field_0x40;
  v10 = a0->field_0x48;
  if (sub_12f60(&v1,a0,0)) {
    free(a0->field_0x0);
    a0->field_0x0 = v1;
    a0->field_0x8 = v2;
    a0->field_0x10 = v3;
    a0->field_0x18 = v4;
    a0->field_0x48 = v10;
    return 1;
  }
  a0->field_0x48 = v10;
  if ((sub_12f60(a0,&v1,1) == '\x01') && (sub_12f60(a0,&v1,0) == '\x01')) {
    free(v1);
    return 0;
  }
  abort(); // no-return
}

// Function: sub_1335f @ 0x1335f
unsigned long sub_1335f(char *a0,long a1,long *a2) // return-dupe x2, return-dupe, ternary x2
{
  unsigned long v1; // rax
  long *v2; // rax
  long *v3; // stack - 0x30
  float v4; // xmm0_da
  long v5; // stack - 0x28
  long v6; // stack - 0x20
  
  if (!a1)
    abort(); // no-return
  v5 = sub_12dc7(a0,a1,&v3,0);
  if (v5) {
    if (!a2)
      return 0;
    *a2 = v5;
    return 0;
  }
  if (((float)*(unsigned long *)&a0[0x10] * *(float *)(*(long *)&a0[0x28] + 8) < (float)*(unsigned long *)&a0[0x18]) && (sub_12769(a0), (float)*(unsigned long *)&a0[0x10] * *(float *)(*(long *)&a0[0x28] + 8) < (float)*(unsigned long *)&a0[0x18])) {
    v6 = *(long *)&a0[0x28];
    v4 = (*(char *)(v6 + 0x10)) ? (float)*(unsigned long *)&a0[0x10] * *(float *)(v6 + 0xc) : *(float *)(v6 + 8) * (*(float *)(v6 + 0xc) * (float)*(unsigned long *)&a0[0x10]); // branch-flip
    if (dat_23510 <= v4)
      return 0xffffffff;
    v1 = (dat_23514 <= v4) ? (long)(v4 - dat_23514) ^ 0x8000000000000000 : (unsigned long)v4;
    if (sub_13134(a0,v1) != '\x01')
      return 0xffffffff;
    if (sub_12dc7(a0,a1,&v3,0))
      abort(); // no-return
  }
  if (!*v3) {
    *v3 = a1;
    *(long *)&a0[0x20] = *(long *)&a0[0x20] + 1;
    *(long *)&a0[0x18] = *(long *)&a0[0x18] + 1;
    return 1;
  }
  v2 = (long *)sub_12d3e(a0);
  if (v2) {
    *v2 = a1;
    v2[1] = v3[1];
    v3[1] = (long)v2;
    *(long *)&a0[0x20] = *(long *)&a0[0x20] + 1;
    return 1;
  }
  return 0xffffffff;
}

// Function: sub_136d0 @ 0x136d0
unsigned long sub_136d0(char *a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  
  v1 = sub_1335f(a0,a1,&v3);
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

// Function: sub_13740 @ 0x13740
long sub_13740(char *a0,long a1) // early-return, ternary x2
{
  char v1; // al
  long *v2; // stack - 0x40
  float v3; // xmm0_da
  void *v4; // stack - 0x38
  long v5; // stack - 0x30
  long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  void *v8; // stack - 0x18
  
  v5 = sub_12dc7(a0,a1,&v2,1);
  if (!v5)
    return 0;
  *(long *)&a0[0x20] = *(long *)&a0[0x20] + -1;
  if (((!*v2) && (*(long *)&a0[0x18] = *(long *)&a0[0x18] + -1, (float)*(unsigned long *)&a0[0x18] < (float)*(unsigned long *)&a0[0x10] * **(float **)&a0[0x28])) && (sub_12769(a0), (float)*(unsigned long *)&a0[0x18] < (float)*(unsigned long *)&a0[0x10] * **(float **)&a0[0x28])) {
    v6 = *(long *)&a0[0x28];
    if (*(char *)(v6 + 0x10)) { // branch-flip
      v3 = (float)*(unsigned long *)&a0[0x10] * *(float *)(v6 + 4);
      v7 = (dat_23514 <= v3) ? (long)(v3 - dat_23514) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    else {
      v3 = *(float *)(v6 + 8) * (*(float *)(v6 + 4) * (float)*(unsigned long *)&a0[0x10]);
      v7 = (dat_23514 <= v3) ? (long)(v3 - dat_23514) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    v1 = sub_13134(a0,v7);
    if (v1 != '\x01') {
      v4 = *(void **)&a0[0x48];
      while (v4) {
        v8 = *(void **)((long)v4 + 8);
        free(v4);
        v4 = v8;
      }
      *(unsigned long *)&a0[0x48] = 0;
    }
  }
  return v5;
}

// Function: sub_13a46 @ 0x13a46
void sub_13a46(char *a0,long a1)
{
  sub_13740(a0,a1);
}

// Function: sub_13a6f @ 0x13a6f
unsigned long sub_13a6f(struct_15 *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = sub_1ea7a(a0->field_0x0,a1);
  return (a0->field_0x8 ^ v1) % a1;
}

// Function: sub_13abf @ 0x13abf
unsigned int sub_13abf(unsigned long *a0,unsigned long *a1) // return-dupe
{
  char *v1;
  char *v2;
  
  if (((a0[1] == a1[1]) && (a0[2] == a1[2])) && (v1 = (char *)*a1, v2 = (char *)*a0, !strcmp(v2,v1)))
    return 1;
  return 0;
}

// Function: sub_13b3b @ 0x13b3b
void sub_13b3b(unsigned long *a0)
{
  free((void *)*a0);
  free(a0);
}

// Function: sub_13b71 @ 0x13b71
void sub_13b71(void)
{
  return;
}

// Function: sub_13c5a @ 0x13c5a
void * sub_13c5a(void *a0,unsigned long a1,unsigned char *a2,char *a3)
{
  unsigned char v1;
  unsigned long v2; // rax
  char v3 [56];
  unsigned char *v4; // stack - 0x90
  unsigned long v5; // stack - 0x70
  unsigned long v6; // stack - 0x68
  
  v5 = 0xffffffffffffffff;
  v2 = strlen(a3);
  memcpy(v3,a0,a1);
  a0 = (void *)((long)a0 + a1);
  v4 = a2;
  v6 = a1;
  while( true ) {
    v1 = *v4;
    if (v1) {
      if (0x7f <= v1) // branch-flip
        v5 = v6;
      else {
        v5 = (unsigned long)v1;
      }
      v4 = &v4[1];
    }
    if (v6 < v5)
      v5 = v6;
    a0 = (void *)((long)a0 - v5);
    v6 -= v5;
    memcpy(a0,&v3[v6],v5);
    if (!v6) break;
    a0 = (void *)((long)a0 - v2);
    memcpy(a0,a3,v2);
  }
  return a0;
}

// Function: sub_13d91 @ 0x13d91
char * sub_13d91(unsigned long a0,char *a1,unsigned int a2,unsigned long a3,unsigned long a4) // ternary x3, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_13d91
{
  unsigned char *v1;
  char v10;
  long double v11; // st0
  long double v12; // st0
  long double v13; // st0
  unsigned int v14; // stack - 0xe4
  int v15; // stack - 0xe0
  unsigned int v16; // stack - 0xdc
  unsigned long v17; // stack - 0xc0
  char *v18; // stack - 0xb8
  char *v19; // stack - 0xb0
  unsigned long v2;
  char *v20; // stack - 0xa8
  char *v21; // stack - 0xa0
  unsigned long v22; // stack - 0x98
  char *v23; // stack - 0x90
  unsigned long v24; // stack - 0x88
  long v25; // stack - 0x80
  unsigned long v26; // stack - 0x78
  long double v27; // stack - 0x38
  bool v3;
  unsigned int v4; // eax
  unsigned int v5; // eax
  unsigned int v6; // eax
  lconv *v7; // rax
  unsigned long v8;
  char *v9;
  
  v4 = a2 & 3;
  v5 = (a2 & 0x20) ? 0x400 : 1000; // branch-flip
  v15 = -1;
  v21 = ".";
  v22 = 1;
  v23 = "";
  v7 = localeconv();
  v8 = strlen(*(char **)v7);
  if ((v8) && (v8 <= 0x10)) {
    v21 = *(char **)v7;
    v22 = v8;
  }
  v1 = *(unsigned char **)&v7->field_0x10;
  v9 = *(char **)&v7->field_0x8;
  if (strlen(v9) <= 0x10)
    v23 = *(char **)&v7->field_0x8;
  v9 = &a1[0x287];
  if (a4 <= a3) { // branch-flip
    if ((!(a3 % a4)) && (v17 = a0 * (a3 / a4), a0 == v17 / (a3 / a4))) {
      v14 = 0;
      v16 = 0;
label_142e0:
      v18 = v9;
      if ((a2 & 0x10) && (v15 = 0, v5 <= v17)) {
        do {
          v14 += (int)(v17 % (unsigned long)v5) * 10;
          v6 = ((int)v16 >> 1) + (v14 % v5) * 2;
          v17 /= v5;
          v14 /= v5;
          if (v5 <= v6) { // branch-flip
            v16 = (v6 + v16 <= v5) ? 2 : 3; // branch-flip
          }
          else {
            v16 = (unsigned int)(v6 + v16 != 0);
          }
          v15 += 1;
        } while ((v5 <= v17) && (v15 < 8));
        if (v17 <= 9) {
          if (v4 != 1) { // branch-flip
            v3 = !((v4) || (!v16));
          }
          else {
            v3 = 2 < v16 + (v14 & 1);
          }
          if (v3) {
            v14 += 1;
            v16 = 0;
            if (v14 == 10) {
              v17 += 1;
              v14 = 0;
            }
          }
          if ((v17 <= 9) && ((v14 || (!(a2 & 8))))) {
            a1[0x286] = (char)v14 + '0';
            v18 = &(&a1[0x286])[-v22];
            memcpy(v18,v21,v22);
            v16 = 0;
            v14 = 0;
          }
        }
      }
      if (v4 != 1) { // branch-flip
        v3 = !((v4) || ((int)(v16 + v14) <= 0));
      }
      else {
        v3 = 5 < (int)(v14 + (((unsigned int)v17 & 1) + v16 != 0));
      }
      if ((((v3) && (v17 = v17 + 1, a2 & 0x10)) && (v17 == v5)) && (v15 < 8)) {
        v15 += 1;
        if (!(a2 & 8)) {
          v18[-1] = '0';
          v18 = &(&v18[-1])[-v22];
          memcpy(v18,v21,v22);
        }
        v17 = 1;
      }
      v20 = v18;
      do {
        v18 = &v18[-1];
        *v18 = (char)v17 + ((char)(v17 / 10 << 2) + (char)(v17 / 10)) * '\xfe' + '0';
        v17 /= 10;
      } while (v17);
      goto label_1469f;
    }
  }
  else if ((a3) && (!(a4 % a3))) {
    a3 = a4 / a3;
    v8 = (a0 % a3) * 10;
    v2 = (v8 % a3) * 2;
    v17 = a0 / a3;
    v14 = (unsigned int)(v8 / a3);
    if (a3 <= v2) { // branch-flip
      v16 = (v2 <= a3) ? 2 : 3; // branch-flip
    }
    else {
      v16 = (unsigned int)(v2 != 0);
    }
    goto label_142e0;
  }
  v11 = (long double)(long)a4;
  if ((long)a4 <= -1)
    v11 = dat_235b0 + v11;
  v12 = (long double)(long)a0;
  if ((long)a0 <= -1)
    v12 = dat_235b0 + v12;
  v13 = (long double)(long)a3;
  if ((long)a3 <= -1)
    v13 = dat_235b0 + v13;
  if (a2 & 0x10) { // branch-flip
    v27 = (long double)1;
    v15 = 0;
    do {
      v27 = v27 * (long double)v5;
      v15 += 1;
      if ((v13 / v11) * v12 < v27 * (long double)v5) break;
    } while (v15 < 8);
    sub_13b71(v4);
    sprintf(a1,"%.1Lf");
    v24 = strlen(a1);
    v25 = v22 + 1;
    if ((v25 + (unsigned long)((a2 & 0x20) == 0) + 1 < v24) || ((a2 & 8 && (a1[v24 - 1] == '0')))) {
      sub_13b71(v4);
      sprintf(a1,"%.0Lf");
      v24 = strlen(a1);
      v25 = 0;
    }
  }
  else {
    sub_13b71(v4);
    sprintf(a1,"%.0Lf");
    v24 = strlen(a1);
    v25 = 0;
  }
  v18 = &v9[-v24];
  memmove(v18,a1,v24);
  v20 = &v18[v24 - v25];
label_1469f:
  if (a2 & 4)
    v18 = (char *)sub_13c5a(v18,(long)v20 - (long)v18,v1,v23);
  v19 = v9;
  if (a2 & 0x80) {
    if (v15 <= -1) {
      v15 = 0;
      v26 = 1;
      while ((v26 < a4 && (v15 = v15 + 1, v15 != 8))) {
        v26 = v5 * v26;
      }
    }
    if ((a2 & 0x100 || v15) && (a2 & 0x40)) {
      v19 = &a1[0x288];
      *v9 = ' ';
    }
    if (v15) {
      if ((a2 & 0x20) || (v15 != 1))
        v10 = *(char *)((long)v15 + 0x23520);
      else {
        v10 = 'k';
      }
      v9 = &v19[1];
      *v19 = v10;
      v19 = v9;
    }
    if (a2 & 0x100) {
      if ((a2 & 0x20) && (v15)) {
        v9 = &v19[1];
        *v19 = 'i';
        v19 = v9;
      }
      v9 = &v19[1];
      *v19 = 'B';
      v19 = v9;
    }
  }
  *v19 = '\0';
  return v18;
}

// Function: sub_1482b @ 0x1482b
unsigned long sub_1482b(void) // return-dupe
{
  if (getenv("POSIXLY_CORRECT"))
    return 0x200;
  return 0x400;
}

// Function: sub_14855 @ 0x14855
int sub_14855(unsigned char *a0,unsigned long *a1,unsigned int *a2)
{
  unsigned int v1;
  int v2; // eax
  unsigned char *v3; // stack - 0x18
  unsigned char *v4; // stack - 0x30
  unsigned int v5; // stack - 0x24
  
  v5 = 0;
  v4 = a0;
  if (!a0) {
    v4 = (unsigned char *)getenv("BLOCK_SIZE");
    if (!v4) {
      v4 = (unsigned char *)getenv("BLOCKSIZE");
      if (!v4) {
        *a1 = sub_1482b();
        goto label_149d9;
      }
    }
  }
  if (*v4 == 0x27) {
    v5 = 4;
    v4 = &v4[1];
  }
  v2 = sub_fa7e(v4,0x2a9c0,0x23550,4);
  if (0 <= v2) { // branch-flip
    v5 |= *(unsigned int *)((long)v2 * 4 + 0x23550);
    *a1 = 1;
  }
  else {
    v2 = sub_1d251(v4,&v3,0,a1,"eEgGkKmMpPtTyYzZ0");
    if (v2) {
      *a2 = 0;
      return v2;
    }
    while( true ) {
      if (('0' <= (char)*v4) && ((char)*v4 <= '9')) goto label_149d9;
      if (v4 == v3) break;
      v4 = &v4[1];
    }
    v1 = v5 | 0x80;
    if (v3[-1] == 0x42)
      v1 = v5 | 0x180;
    v5 = v1;
    if ((v3[-1] != 0x42) || (v3[-2] == 0x69))
      v5 |= 0x20;
  }
label_149d9:
  *a2 = v5;
  return 0;
}

// Function: sub_149fd @ 0x149fd
unsigned int sub_149fd(unsigned char *a0,unsigned int *a1,long *a2)
{
  unsigned int v1; // stack - 0xc
  
  v1 = sub_14855(a0,a2,a1);
  if (!*a2) {
    *a2 = sub_1482b();
    v1 = 4;
  }
  return v1;
}

// Function: sub_14a53 @ 0x14a53
unsigned int * sub_14a53(unsigned int a0) // early-return
{
  passwd *v1; // rax
  char *v2;
  unsigned int *v3; // stack - 0x28
  unsigned int *v4; // stack - 0x20
  
  v4 = NULL;
  v3 = dat_2c458;
  do {
    if (!v3) {
label_14a9f:
      if (!v4) {
        v1 = getpwuid(a0);
        if (v1) // branch-flip
          v2 = *(char **)v1;
        else {
          v2 = "";
        }
        v4 = (unsigned int *)sub_1c68b(strlen(v2) + 0x18 & 0xfffffffffffffff8);
        *v4 = a0;
        strcpy((char *)&v4[4],v2);
        *(unsigned int **)&v4[2] = dat_2c458;
        dat_2c458 = v4;
      }
      if ((char)v4[4])
        return &v4[4];
      return NULL;
    }
    if (a0 == *v3) {
      v4 = v3;
      goto label_14a9f;
    }
    v3 = *(unsigned int **)&v3[2];
  } while( true );
}

// Function: sub_14b4a @ 0x14b4a
unsigned int * sub_14b4a(char *a0) // early-return
{
  passwd *v1; // rax
  unsigned int *v2; // rax
  unsigned int *v3; // stack - 0x18
  
  v3 = dat_2c458;
  while( true ) {
    if (!v3) {
      v3 = dat_2c460;
      while( true ) {
        if (!v3) {
          v1 = getpwnam(a0);
          v2 = (unsigned int *)sub_1c68b(strlen(a0) + 0x18 & 0xfffffffffffffff8);
          strcpy((char *)&v2[4],a0);
          if (v1) {
            *v2 = *(unsigned int *)&v1->field_0x10;
            *(unsigned int **)&v2[2] = dat_2c458;
            dat_2c458 = v2;
            return v2;
          }
          *(unsigned int **)&v2[2] = dat_2c460;
          dat_2c460 = v2;
          return NULL;
        }
        if ((*(char *)((long)v3 + 0x10) == *a0) && (!strcmp((char *)((long)v3 + 0x10),a0))) break;
        v3 = *(unsigned int **)((long)v3 + 8);
      }
      return NULL;
    }
    if ((*(char *)&v3[4] == *a0) && (!strcmp((char *)&v3[4],a0))) break;
    v3 = *(unsigned int **)&v3[2];
  }
  return v3;
}

// Function: sub_14ca5 @ 0x14ca5
unsigned int * sub_14ca5(unsigned int a0) // early-return
{
  group *v1; // rax
  char *v2;
  unsigned int *v3; // stack - 0x28
  unsigned int *v4; // stack - 0x20
  
  v4 = NULL;
  v3 = dat_2c468;
  do {
    if (!v3) {
label_14cf1:
      if (!v4) {
        v1 = getgrgid(a0);
        if (v1) // branch-flip
          v2 = *(char **)v1;
        else {
          v2 = "";
        }
        v4 = (unsigned int *)sub_1c68b(strlen(v2) + 0x18 & 0xfffffffffffffff8);
        *v4 = a0;
        strcpy((char *)&v4[4],v2);
        *(unsigned int **)&v4[2] = dat_2c468;
        dat_2c468 = v4;
      }
      if ((char)v4[4])
        return &v4[4];
      return NULL;
    }
    if (a0 == *v3) {
      v4 = v3;
      goto label_14cf1;
    }
    v3 = *(unsigned int **)&v3[2];
  } while( true );
}

// Function: sub_14d9c @ 0x14d9c
unsigned int * sub_14d9c(char *a0) // early-return
{
  group *v1; // rax
  unsigned int *v2; // rax
  unsigned int *v3; // stack - 0x18
  
  v3 = dat_2c468;
  while( true ) {
    if (!v3) {
      v3 = dat_2c470;
      while( true ) {
        if (!v3) {
          v1 = getgrnam(a0);
          v2 = (unsigned int *)sub_1c68b(strlen(a0) + 0x18 & 0xfffffffffffffff8);
          strcpy((char *)&v2[4],a0);
          if (v1) {
            *v2 = *(unsigned int *)&v1->field_0x10;
            *(unsigned int **)&v2[2] = dat_2c468;
            dat_2c468 = v2;
            return v2;
          }
          *(unsigned int **)&v2[2] = dat_2c470;
          dat_2c470 = v2;
          return NULL;
        }
        if ((*(char *)((long)v3 + 0x10) == *a0) && (!strcmp((char *)((long)v3 + 0x10),a0))) break;
        v3 = *(unsigned int **)((long)v3 + 8);
      }
      return NULL;
    }
    if ((*(char *)&v3[4] == *a0) && (!strcmp((char *)&v3[4],a0))) break;
    v3 = *(unsigned int **)&v3[2];
  }
  return v3;
}

// Function: sub_14ef7 @ 0x14ef7
char * sub_14ef7(long a0,char *a1)
{
  char *v1;
  long v2; // stack - 0x20
  char *v3; // stack - 0x10
  
  v3 = &a1[0x14];
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
      v3[-1] = 0x30 - ((char)v2 + (char)(v2 / 10) * '\xf6');
      v2 /= 10;
      v1 = &v3[-1];
    } while (v2);
    v3 = &v3[-2];
    *v3 = '-';
  }
  return v3;
}

// Function: sub_15035 @ 0x15035
char * sub_15035(unsigned long a0,char *a1)
{
  unsigned long v1; // stack - 0x20
  char *v2; // stack - 0x10
  
  v2 = &a1[0x14];
  *v2 = '\0';
  v1 = a0;
  do {
    v2 = &v2[-1];
    *v2 = (char)v1 + ((char)(v1 / 10 << 2) + (char)(v1 / 10)) * '\xfe' + '0';
    v1 /= 10;
  } while (v1);
  return v2;
}

// Function: sub_150be @ 0x150be
unsigned long sub_150be(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3) // early-return
{
  long v1;
  unsigned int *v2; // stack - 0x30
  unsigned long v3; // stack - 0x18
  
  v2 = a0;
  if (!a0)
    v2 = &v1;
  v3 = mbrtowc(v2,a1,a2,a3);
  if (((0xfffffffffffffffe <= v3) && (a2)) && (sub_11f2c(0) != '\x01')) {
    *v2 = (unsigned int)(unsigned char)*a1;
    return 1;
  }
  return v3;
}

// Function: sub_15168 @ 0x15168
char sub_15168(unsigned int *a0)
{
  unsigned int v1;
  char v2; // stack - 0x11
  unsigned int *v3; // stack - 0x10
  
  v2 = 0;
  for (v3 = a0; *v3; v3 = &v3[1]) {
    v1 = *v3;
    if (!iswprint(v1)) {
      *v3 = 0xfffd;
      v2 = 1;
    }
  }
  return v2;
}

// Function: sub_151ba @ 0x151ba
long sub_151ba(int *a0,unsigned long a1)
{
  int *v1; // stack - 0x20
  int v2; // stack - 0x14
  long v3; // stack - 0x10
  
  v3 = 0;
  for (v1 = a0; *v1; v1 = &v1[1]) {
    v2 = wcwidth(*v1);
    if (v2 == -1) {
      *v1 = 0xfffd;
      v2 = 1;
    }
    if (a1 < (unsigned long)(v3 + v2)) break;
    v3 += v2;
  }
  *v1 = 0;
  return v3;
}

// Function: sub_15244 @ 0x15244
char * sub_15244(char *a0,char *a1,long a2)
{
  char *v1;
  long v2;
  long v3; // stack - 0x20
  char *v4; // stack - 0x10
  
  v3 = a2;
  v4 = a0;
  while ((v2 = v3 + -1, v3 && (v4 < a1))) {
    v1 = &v4[1];
    *v4 = 0x20;
    v3 = v2;
    v4 = v1;
  }
  *v4 = 0;
  return v4;
}

// Function: sub_15291 @ 0x15291
long sub_15291(char *a0,char *a1,long a2,unsigned long *a3,int a4,unsigned int a5)
{
  unsigned long v1;
  wchar_t *v10; // stack - 0x58
  char *v11; // stack - 0x50
  unsigned long v12; // stack - 0x48
  unsigned long v13; // stack - 0x40
  unsigned long v14; // stack - 0x38
  unsigned long v15; // stack - 0x30
  unsigned long v16; // stack - 0x28
  bool v2;
  unsigned long v3;
  char *v4; // rax
  void *v5; // rax
  bool v6; // stack - 0x72
  long v7; // stack - 0x70
  unsigned long v8; // stack - 0x68
  char *v9; // stack - 0x60
  
  v7 = -1;
  v13 = strlen(a0);
  v9 = NULL;
  v10 = NULL;
  v12 = v13;
  v14 = 0;
  v6 = 0;
  v2 = 0;
  v11 = a0;
  if ((a5 & 2) || (__ctype_get_mb_cur_max() <= 1)) {
label_1540a:
    if ((v2) && ((v6 || (*a3 < v12)))) {
      v8 = v13;
      if (v6)
        v8 = wcstombs(NULL,v10,0);
      v8 += 1;
      v9 = malloc(v8);
      if (!v9) goto label_15474;
      v11 = v9;
      v3 = *a3;
      v12 = sub_151ba(v10,v3);
      v13 = wcstombs(v9,v10,v8);
    }
  }
  else {
    v3 = mbstowcs(NULL,a0,0);
    if (v3 != 0xffffffffffffffff) {
      v1 = v3 + 1;
      v10 = malloc(v1 * 4);
      if (v10) {
        if (mbstowcs(v10,a0,v1)) {
          v10[v3] = L'\0';
          v2 = 1;
          v6 = (bool)sub_15168(v10);
          v12 = (unsigned long)wcswidth(v10,v1);
        }
        goto label_1540a;
      }
    }
label_15474:
    if (!(a5 & 1)) {
      free(v10); // return-dupe
      free(v9);
      return v7;
    }
  }
  if (*a3 < v12) {
    v12 = *a3;
    v13 = v12;
  }
  if (v12 < *a3)
    v14 = *a3 - v12;
  *a3 = v12;
  if (a4) { // branch-flip
    if (a4 != 1) { // branch-flip
      v15 = (unsigned long)((unsigned int)v14 & 1) + (v14 >> 1);
      v16 = v14 >> 1;
    }
    else {
      v15 = v14;
      v16 = 0;
    }
  }
  else {
    v15 = 0;
    v16 = v14;
  }
  if (a5 & 4)
    v15 = 0;
  if (a5 & 8)
    v16 = 0;
  if (a2) {
    v4 = &a1[a2 + -1];
    v5 = (void *)sub_15244(a1,v4,v15);
    v3 = (long)v4 - (long)v5;
    if (v13 < (unsigned long)((long)v4 - (long)v5))
      v3 = v13;
    sub_15244(mempcpy(v5,v11,v3),v4,v16);
  }
  v7 = v13 + v15 + v16;
  free(v10);
  free(v9);
  return v7;
}

// Function: sub_1565c @ 0x1565c
char * sub_1565c(char *a0,unsigned long *a1,unsigned int a2,unsigned int a3)
{
  unsigned long v1;
  char *v2; // rax
  unsigned long v3; // stack - 0x30
  unsigned long v4; // stack - 0x28
  char *v5; // stack - 0x20
  
  v1 = *a1;
  v3 = *a1;
  v5 = NULL;
  v4 = v3;
  while( true ) {
    if (v4 < v3)
      return v5;
    v3 = v4 + 1;
    v2 = realloc(v5,v3);
    if (!v2) break;
    v5 = v2;
    *a1 = v1;
    v4 = sub_15291(a0,v2,v3,a1,a2,a3);
    if (v4 == 0xffffffffffffffff) {
      free(v2);
      return NULL;
    }
  }
  free(v5);
  return NULL;
}

// Function: sub_1574e @ 0x1574e
void sub_1574e(char *a0,unsigned int a1)
{
  sub_15784(a0,strlen(a0),a1);
}

// Function: sub_15784 @ 0x15784
int sub_15784(char *a0,long a1,unsigned int a2)
{
  char *v1;
  char *v10; // stack - 0x20
  char v2;
  int v3; // eax
  mbstate_t v4;
  unsigned int v5; // stack - 0x3c
  int v6; // stack - 0x38
  int v7; // stack - 0x34
  char *v8; // stack - 0x30
  long v9; // stack - 0x28
  
  v10 = &a0[a1];
  v6 = 0;
  v8 = a0;
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
label_1593d:
    if (v8 < v10) {
      v3 = (int)*v8;
      if (0x7f <= v3) { // branch-flip
label_1581c:
        memset(&v4,0,8);
        do {
          v9 = sub_150be(&v5,v8,(long)v10 - (long)v8,&v4);
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
            if (!iswcntrl(v5)) {
              if (v6 == 0x7fffffff)
                return 0x7fffffff;
              v6 += 1;
            }
          }
          v8 = &v8[v9];
        } while (!mbsinit(&v4));
      }
      else {
        if (v3 <= 0x60) {
          if (0x60 > v3) {
            if (v3 > 0x40) goto label_1580e;
            if (0x24 <= v3) { // branch-flip
              if (0x1b > (unsigned int)(v3 - 0x25U)) goto label_1580e;
            }
            else if (v3 > 0x1f) goto label_1580e;
          }
          goto label_1581c;
        }
label_1580e:
        v8 = &v8[1];
        v6 += 1;
      }
      goto label_1593d;
    }
  }
  else {
    while (v8 < v10) {
      v1 = &v8[1];
      v2 = *v8;
      v8 = v1;
      if (*(unsigned short *)((unsigned long)(unsigned char)v2 * 2 + *(long *)__ctype_b_loc()) & 0x4000) { // branch-flip
        if (v6 == 0x7fffffff)
          return 0x7fffffff;
        v6 += 1;
      }
      else {
        if (a2 & 2)
          return -1;
        if (!(*(unsigned short *)((unsigned long)(unsigned char)v2 * 2 + *(long *)__ctype_b_loc()) & 2)) {
          if (v6 == 0x7fffffff)
            return 0x7fffffff;
          v6 += 1;
        }
      }
    }
  }
  return v6;
}

// Function: sub_15a0c @ 0x15a0c
void sub_15a0c(unsigned long *a0,unsigned long a1,unsigned long *a2,void *a3)
{
  unsigned long *v1;
  unsigned long *v2; // stack - 0x60
  unsigned long v3; // stack - 0x48
  unsigned long v4; // stack - 0x40
  unsigned long v5; // stack - 0x38
  unsigned long v6; // stack - 0x30
  unsigned long v7; // stack - 0x28
  
  v4 = a1 >> 1;
  v3 = 0;
  sub_15bb2(&a0[v4],a1 - v4,a2,a3);
  sub_15bb2(a0,v4,a2,a3);
  v6 = *a0;
  v7 = a0[v4];
  v2 = a2;
  v5 = v4;
  while( true ) {
    while( true ) {
      if ((int)(*a3)(v6,v7) <= 0) break;
      v1 = &v2[1];
      *v2 = v7;
      v5 += 1;
      if (v5 == a1) {
        v2 = &v2[1]; // return-dupe
        memcpy(v2,&a0[v3],(v4 - v3) * 8);
        return;
      }
      v7 = a0[v5];
      v2 = v1;
    }
    v1 = &v2[1];
    *v2 = v6;
    v3 += 1;
    if (v3 == v4) break;
    v6 = a0[v3];
    v2 = v1;
  }
  v3 = v5;
  v4 = a1;
  v2 = &v2[1];
  memcpy(v2,&a0[v3],(v4 - v3) * 8);
  return;
}

// Function: sub_15bb2 @ 0x15bb2
void sub_15bb2(unsigned long *a0,unsigned long a1,unsigned long *a2,void *a3) // return-dupe x3
{
  long v1;
  unsigned long v2;
  unsigned long v3;
  unsigned long v4; // rax
  long v5; // stack - 0x60
  unsigned long v6; // stack - 0x58
  unsigned long v7; // stack - 0x50
  unsigned long v8; // stack - 0x48
  unsigned long v9; // stack - 0x40
  
  if (3 <= a1) {
    v4 = a1 >> 1;
    v6 = 0;
    sub_15bb2(&a0[v4],a1 - v4,a2,a3);
    if (2 <= v4) // branch-flip
      sub_15a0c(a0,v4,a2,a3);
    else {
      *a2 = *a0;
    }
    v9 = *a2;
    v8 = a0[v4];
    v5 = 0;
    v7 = v4;
    while( true ) {
      while (0 < (int)(*a3)(v9,v8)) {
        a0[v5] = v8;
        v7 += 1;
        if (v7 == a1) {
          memcpy(&a0[v5 + 1],&a2[v6],(v4 - v6) * 8);
          return;
        }
        v8 = a0[v7];
        v5 += 1;
      }
      v1 = v5 + 1;
      a0[v5] = v9;
      v6 += 1;
      if (v6 == v4) break;
      v9 = a2[v6];
      v5 = v1;
    }
    return;
  }
  if (a1 != 2)
    return;
  v2 = *a0;
  v3 = a0[1];
  if ((int)(*a3)(v2,v3) <= 0)
    return;
  *a0 = v3;
  a0[1] = v2;
}

// Function: sub_15dfc @ 0x15dfc
void sub_15dfc(unsigned long *a0,unsigned long a1,void *a2)
{
  sub_15bb2(a0,a1,&a0[a1],a2);
}

// Function: sub_15e45 @ 0x15e45
long sub_15e45(long a0,long a1,long a2)
{
  long v1;
  unsigned char v2;
  long v3; // stack - 0x20
  
  v3 = a2;
  while (v1 = v3 + -1, v3) {
    v2 = *(unsigned char *)(v1 + a1);
    *(char *)(v1 + a0) = (char)tolower((unsigned int)v2);
    v3 = v1;
  }
  return a0;
}

// Function: sub_15e9d @ 0x15e9d
long sub_15e9d(long a0,long a1,long a2)
{
  long v1;
  unsigned char v2;
  long v3; // stack - 0x20
  
  v3 = a2;
  while (v1 = v3 + -1, v3) {
    v2 = *(unsigned char *)(v1 + a1);
    *(char *)(v1 + a0) = (char)toupper((unsigned int)v2);
    v3 = v1;
  }
  return a0;
}

// Function: sub_15ef5 @ 0x15ef5
int sub_15ef5(int a0,int a1)
{
  return (a0 - ((a0 - a1) + 0x17e) % 7) + 3;
}

// Function: sub_15f45 @ 0x15f45
void sub_15f45(char *a0,long a1,char *a2,tm *a3,unsigned long a4,unsigned int a5)
{
  char v1; // stack - 0x11
  
  v1 = 0;
  sub_15fc8(a0,a1,a2,a3,0,0,0xffffffffffffffff,&v1,a4,a5);
}

// Function: sub_15fc8 @ 0x15fc8
long sub_15fc8(char *a0,long a1,char *a2,tm *a3,char a4,int a5,int a6,unsigned long a7,unsigned long a8,int a9) // early-return x13, ternary x6, warn: iteboolean: re-rolled 3 0/1 select diamond(s) to a boolean assignment in sub_15fc8
{
  int v1;
  unsigned int v10; // eax
  int v11; // eax
  unsigned long v12; // rax
  unsigned long v13;
  unsigned long v14;
  char *v15;
  char *v16; // rax
  char *v17;
  char v18 [3]; // stack - 0x42d
  char v19 [2];
  unsigned int v2;
  char v20 [22];
  char v21 [1010];
  unsigned long v22; // stack - 0x468
  bool v23; // stack - 0x6c4
  bool v24; // stack - 0x6c1
  unsigned int v25; // stack - 0x6bc
  int v26; // stack - 0x6b8
  int v27; // stack - 0x6b0
  unsigned int v28; // stack - 0x6ac
  unsigned int v29; // stack - 0x6a8
  bool v3;
  unsigned int v30; // stack - 0x6a4
  char v31; // stack - 0x6a0
  int v32; // stack - 0x69c
  int v33; // stack - 0x698
  int v34; // stack - 0x694
  int v35; // stack - 0x690
  int v36; // stack - 0x68c
  int v37; // stack - 0x688
  unsigned int v38; // stack - 0x684
  char *v39; // stack - 0x648
  unsigned long v4;
  long v40; // stack - 0x640
  char *v41; // stack - 0x638
  char *v42; // stack - 0x630
  char *v43; // stack - 0x628
  char *v44; // stack - 0x620
  unsigned long v45; // stack - 0x618
  char *v46; // stack - 0x610
  long v47; // stack - 0x608
  unsigned long v48; // stack - 0x460
  unsigned long v49; // stack - 0x458
  unsigned long v5;
  long v50; // stack - 0x450
  unsigned long v51; // stack - 0x448
  unsigned long v52; // stack - 0x440
  unsigned long v53; // stack - 0x438
  char *v6;
  bool v7;
  char v8;
  int v9;
  
  v5 = a8;
  v4 = a7;
  v1 = *__errno_location();
  v25 = *(unsigned int *)&a3->field_0x8;
  v40 = 0;
  v39 = *(char **)&a3->field_0x30;
  if (!v39)
    v39 = "";
  if (0xd <= (int)v25) // branch-flip
    v25 -= 0xc;
  else if (!v25)
    v25 = 0xc;
  v41 = a0;
  v42 = a2;
  do {
    v6 = v42;
    if (!*v42) {
      if ((v41) && (a1))
        *v41 = '\0';
      *__errno_location() = v1;
      return v40;
    }
    v26 = 0;
    v3 = 0;
    v7 = 0;
    v24 = a4;
    if (*v42 != '%') {
      v12 = (0 <= a6) ? (unsigned long)a6 : 0; // branch-flip
      v13 = v12;
      if (v12 <= 1)
        v13 = 1;
      if ((unsigned long)(a1 - v40) <= v13) {
        *__errno_location() = 0x22;
        return 0;
      }
      if (v41) {
        if (1 < v12) {
          memset(v41,0x20,v12 - 1);
          v41 = &v41[v12 - 1];
        }
        *v41 = *v42;
        v41 = &v41[1];
      }
      v40 += v13;
      goto label_187d8;
    }
    while( true ) {
      v42 = &v42[1];
      if (0x3c < (unsigned int)((int)*v42 - 0x23U)) break;
      v12 = 1L << ((unsigned char)((int)*v42 - 0x23U) & 0x3f);
      if (v12 & 0x1000000000002500) // branch-flip
        v26 = (int)*v42;
      else if (v12 & 1) // branch-flip
        v7 = 1;
      else {
        if (!(v12 & 0x800000000000000)) break;
        v24 = '\x01';
      }
    }
    if ((unsigned int)((int)*v42 - 0x30U) <= 9) {
      a6 = 0;
      do {
        v9 = (int)((long)a6 * 10);
        if (((long)v9 != (long)a6 * 10) || (a6 = *v42 + -0x30 + v9, SCARRY4(*v42 + -0x30,v9)))
          a6 = 0x7fffffff;
        v42 = &v42[1];
      } while ((unsigned int)((int)*v42 - 0x30U) <= 9);
    }
    if ((*v42 != 'E') && (*v42 != 'O'))
      v8 = '\0';
    else {
      v15 = &v42[1];
      v8 = *v42;
      v42 = v15;
    }
    v31 = *v42;
    switch(v31) {
      case 0:
label_185d6:
        v42 = &v42[-1];
        break;
      case 0x25:
        if (v6 != &v42[-1]) goto label_185d6;
        if ((v26 != 0x2d) && (0 <= a6))
          v12 = (unsigned long)a6;
        else {
          v12 = 0;
        }
        v13 = v12;
        if (v12 <= 1)
          v13 = 1;
        if ((unsigned long)(a1 - v40) <= v13) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (v41) {
          if (1 < v12) {
            v12 -= 1;
            if ((v26 != 0x30) && (v26 != 0x2b)) {
              memset(v41,0x20,v12);
              v41 = &v41[v12];
            }
            else {
              memset(v41,0x30,v12);
              v41 = &v41[v12];
            }
          }
          *v41 = *v42;
          v41 = &v41[1];
        }
        v40 += v13;
        goto label_187d8;
      case 0x3a:
        for (v45 = 1; v42[v45] == ':'; v45 = v45 + 1) {
        }
        if (v42[v45] == 'z') {
          v42 = &v42[v45];
          goto label_183a7;
        }
        break;
      case 0x41:
        if (!v8) {
          if (v7)
            v24 = 1;
          v3 = 0;
          goto label_16842;
        }
        break;
      case 0x42:
        if (v8 != 'E') {
          if (v7)
            v24 = 1;
          v3 = 0;
          goto label_16842;
        }
        break;
      case 0x43:
        if (v8 == 'E') goto label_16842;
        v23 = *(int *)&a3->field_0x14 < -0x76c;
        v29 = (int)((unsigned int)((*(unsigned int *)&a3->field_0x14 >> 0x1f & (unsigned int)!(bool)v23) != 0) * -99 + *(int *)&a3->field_0x14) / 100 + 0x13;
        v27 = 2;
label_16bf4:
        if (!v26)
          v26 = a5;
        if (v26 != 0x2b) { // branch-flip
label_16c41:
          v7 = 0;
        }
        else {
          v10 = (v27 != 2) ? 9999 : 99; // branch-flip
          if ((v29 <= v10) && (a6 <= v27)) goto label_16c41;
          v7 = 1;
        }
        goto label_16c8d;
      case 0x44:
        if (!v8) {
          v43 = "%m/%d/%y";
          goto label_1662e;
        }
        break;
      case 0x46:
        if (v8) break;
        if ((v26) || (0 <= a6)) {
          v32 = a6 + -6;
          if (v32 <= -1)
            v32 = 0;
        }
        else {
          v26 = 0x2b;
          v32 = 4;
        }
        v43 = "%Y-%m-%d";
        goto label_16638;
      case 0x47:
      case 0x56:
      case 0x67:
        if (v8 != 'E') {
          v9 = (0 <= *(int *)&a3->field_0x14) ? -100 : 300; // branch-flip
          v10 = v9 + *(int *)&a3->field_0x14;
          v36 = 0;
          v37 = sub_15ef5(*(unsigned int *)&a3->field_0x1c,*(unsigned int *)&a3->field_0x18);
          if (0 <= v37) { // branch-flip
            v9 = !((v10 & 3) || ((!((int)v10 % 100) && ((int)v10 % 400))));
            v9 = sub_15ef5(*(int *)&a3->field_0x1c - (v9 + 0x16d),*(unsigned int *)&a3->field_0x18);
            if (0 <= v9) {
              v36 = 1;
              v37 = v9;
            }
          }
          else {
            v36 = -1;
            v2 = *(unsigned int *)&a3->field_0x18;
            v9 = *(int *)&a3->field_0x1c;
            v11 = !((v10 - 1 & 3) || ((!((int)(v10 - 1) % 100) && ((int)(v10 - 1) % 400))));
            v37 = sub_15ef5(v11 + 0x16d + v9,v2);
          }
          if (*v42 != 'G') { // branch-flip
            if (*v42 != 'g') {
              v27 = 2;
              v28 = v37 / 7 + 1;
              goto label_16c6b;
            }
            v29 = (v36 + *(int *)&a3->field_0x14 % 100) % 100;
            v27 = 2;
            v23 = 0;
            if ((int)v29 <= -1) {
              v29 = (0xfffff894U - v36 <= *(int *)&a3->field_0x14) ? v29 + 100 : -v29; // branch-flip
            }
          }
          else {
            v27 = 4;
            v23 = *(int *)&a3->field_0x14 < 0xfffff894U - v36;
            v29 = v36 + *(int *)&a3->field_0x14 + 0x76c;
          }
          goto label_16bf4;
        }
        break;
      case 0x48:
        if (v8 != 'E') {
          v27 = 2;
          v28 = *(unsigned int *)&a3->field_0x8;
          goto label_16c6b;
        }
        break;
      case 0x49:
        if (v8 != 'E') {
          v27 = 2;
          v28 = v25;
          goto label_16c6b;
        }
        break;
      case 0x4d:
        if (v8 != 'E') {
          v27 = 2;
          v28 = *(unsigned int *)&a3->field_0x4;
          goto label_16c6b;
        }
        break;
      case 0x4e:
        if (v8 == 'E') break;
        v33 = a9;
        if (a6 <= 0)
          a6 = 9;
        v34 = 9;
        for (; (a6 < v34 || ((2 <= v34 && (!(v33 % 10))))); v33 = v33 / 10) {
          v34 -= 1;
        }
        for (v35 = v34; 0 < v35; v35 = v35 + -1) {
          v20[v35 + -1] = (char)v33 + (char)(v33 / 10) * '\xf6' + '0';
          v33 /= 10;
        }
        if (!v26)
          v26 = 0x30;
        v12 = (unsigned long)v34;
        if ((unsigned long)(a1 - v40) <= v12) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (v41) {
          if (v24) // branch-flip
            sub_15e9d(v41,v20,v12);
          else {
            memcpy(v41,v20,v12);
          }
          v41 = &v41[v12];
        }
        if ((v26 != 0x2d) && (0 <= a6 - v34))
          v13 = (unsigned long)(a6 - v34);
        else {
          v13 = 0;
        }
        v14 = v13;
        if (!v13)
          v14 = 0;
        if (a1 - (v40 + v12) <= v14) {
          *__errno_location() = 0x22;
          return 0;
        }
        if ((v41) && (v13)) {
          if ((v26 != 0x30) && (v26 != 0x2b)) {
            memset(v41,0x20,v13);
            v41 = &v41[v13];
          }
          else {
            memset(v41,0x30,v13);
            v41 = &v41[v13];
          }
        }
        v40 = v40 + v12 + v14;
        goto label_187d8;
      case 0x50:
        v3 = 1;
        v31 = 'p';
        goto label_17878;
      case 0x52:
        v43 = "%H:%M";
        goto label_1662e;
      case 0x53:
        if (v8 != 'E') {
          v27 = 2;
          v28 = *(unsigned int *)a3;
          goto label_16c6b;
        }
        break;
      case 0x54:
        v43 = "%H:%M:%S";
label_1662e:
        v32 = -1;
label_16638:
        v12 = sub_15fc8(0,0xffffffffffffffff,v43,a3,(unsigned char)v24,v26,v32,v4,v5,a9);
        if ((v26 != 0x2d) && (0 <= a6))
          v13 = (unsigned long)a6;
        else {
          v13 = 0;
        }
        v14 = v13;
        if (v13 <= v12)
          v14 = v12;
        if ((unsigned long)(a1 - v40) <= v14) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (v41) {
          if (v12 < v13) {
            v13 -= v12;
            if ((v26 != 0x30) && (v26 != 0x2b)) {
              memset(v41,0x20,v13);
              v41 = &v41[v13];
            }
            else {
              memset(v41,0x30,v13);
              v41 = &v41[v13];
            }
          }
          sub_15fc8(v41,a1 - v40,v43,a3,(unsigned char)v24,v26,v32,v4,v5,a9);
          v41 = &v41[v12];
        }
        v40 += v14;
        goto label_187d8;
      case 0x55:
        if (v8 != 'E') {
          v27 = 2;
          v28 = ((*(int *)&a3->field_0x1c - *(int *)&a3->field_0x18) + 7) / 7;
          goto label_16c6b;
        }
        break;
      case 0x57:
        if (v8 != 'E') {
          v27 = 2;
          v28 = ((*(int *)&a3->field_0x1c - (*(int *)&a3->field_0x18 + 6) % 7) + 7) / 7;
          goto label_16c6b;
        }
        break;
      case 0x58:
        goto label_16b68;
      case 0x59:
        if (v8 == 'E') goto label_16842;
        if (v8 != 'O') {
          v27 = 4;
          v23 = *(int *)&a3->field_0x14 < -0x76c;
          v29 = *(int *)&a3->field_0x14 + 0x76c;
          goto label_16bf4;
        }
        break;
      case 0x5a:
        if (v7)
          v24 = 0;
        v12 = strlen(v39);
        if ((v26 != 0x2d) && (0 <= a6))
          v13 = (unsigned long)a6;
        else {
          v13 = 0;
        }
        v14 = v13;
        if (v13 <= v12)
          v14 = v12;
        if ((unsigned long)(a1 - v40) <= v14) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (v41) {
          if (v12 < v13) {
            v13 -= v12;
            if ((v26 != 0x30) && (v26 != 0x2b)) {
              memset(v41,0x20,v13);
              v41 = &v41[v13];
            }
            else {
              memset(v41,0x30,v13);
              v41 = &v41[v13];
            }
          }
          if (v7)
            sub_15e45(v41,v39,v12);
          else if (v24) // branch-flip
            sub_15e9d(v41,v39,v12);
          else {
            memcpy(v41,v39,v12);
          }
          v41 = &v41[v12];
        }
        v40 += v14;
        goto label_187d8;
      case 0x61:
        if (!v8) {
          if (v7)
            v24 = 1;
          v3 = 0;
          goto label_16842;
        }
        break;
      case 0x62:
      case 0x68:
        if (v7)
          v24 = 1;
        v3 = 0;
        if (v8 != 'E') goto label_16842;
        break;
      case 99:
        goto label_16b68;
      case 100:
        if (v8 != 'E') {
          v27 = 2;
          v28 = *(unsigned int *)&a3->field_0xc;
          goto label_16c6b;
        }
        break;
      case 0x65:
        if (v8 != 'E') {
          v28 = *(unsigned int *)&a3->field_0xc;
label_16c55:
          v27 = 2;
          if (!v26)
            v26 = 0x5f;
          goto label_16c6b;
        }
        break;
      case 0x6a:
        if (v8 != 'E') {
          v27 = 3;
          v23 = *(int *)&a3->field_0x1c < -1;
          v29 = *(int *)&a3->field_0x1c + 1;
          goto label_16c86;
        }
        break;
      case 0x6b:
        if (v8 != 'E') {
          v28 = *(unsigned int *)&a3->field_0x8;
          goto label_16c55;
        }
        break;
      case 0x6c:
        if (v8 != 'E') {
          v28 = v25;
          goto label_16c55;
        }
        break;
      case 0x6d:
        if (v8 != 'E') {
          v27 = 2;
          v23 = *(int *)&a3->field_0x10 < -1;
          v29 = *(int *)&a3->field_0x10 + 1;
          goto label_16c86;
        }
        break;
      case 0x6e:
        if ((v26 != 0x2d) && (0 <= a6))
          v12 = (unsigned long)a6;
        else {
          v12 = 0;
        }
        v13 = v12;
        if (v12 <= 1)
          v13 = 1;
        if ((unsigned long)(a1 - v40) <= v13) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (v41) {
          if (1 < v12) {
            v12 -= 1;
            if ((v26 != 0x30) && (v26 != 0x2b)) {
              memset(v41,0x20,v12);
              v41 = &v41[v12];
            }
            else {
              memset(v41,0x30,v12);
              v41 = &v41[v12];
            }
          }
          *v41 = '\n';
          v41 = &v41[1];
        }
        v40 += v13;
        goto label_187d8;
      case 0x70:
label_17878:
        if (v7) {
          v24 = 0;
          v3 = 1;
        }
        goto label_16842;
      case 0x71:
        v27 = 1;
        v23 = 0;
        v29 = (*(int *)&a3->field_0x10 * 0xb >> 5) + 1;
        goto label_16c86;
      case 0x72:
        goto label_16842;
      case 0x73:
        v22 = *(unsigned long *)a3;
        v48 = *(unsigned long *)&a3->field_0x8;
        v49 = *(unsigned long *)&a3->field_0x10;
        v51 = *(unsigned long *)&a3->field_0x20;
        v52 = *(unsigned long *)&a3->field_0x28;
        v53 = *(unsigned long *)&a3->field_0x30;
        v50._0_4_ = (unsigned int)*(unsigned long *)&a3->field_0x18;
        v50 = CONCAT44(0xffffffff,(unsigned int)v50);
        v47 = sub_1ba55(v5,&v22);
        if (v50 <= -1) {
          *__errno_location() = 0x4b;
          return 0;
        }
        v44 = &v21[1];
        v23 = -(char)(v47 >> 0x3f);
        do {
          v8 = (char)v47 + (char)(v47 / 10) * '\xf6';
          v47 /= 10;
          if (v23)
            v8 = -v8;
          v44 = &v44[-1];
          *v44 = v8 + '0';
        } while (v47);
        v27 = 1;
        v7 = 0;
        goto label_16d70;
      case 0x74:
        if ((v26 != 0x2d) && (0 <= a6))
          v12 = (unsigned long)a6;
        else {
          v12 = 0;
        }
        v13 = v12;
        if (v12 <= 1)
          v13 = 1;
        if ((unsigned long)(a1 - v40) <= v13) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (v41) {
          if (1 < v12) {
            v12 -= 1;
            if ((v26 != 0x30) && (v26 != 0x2b)) {
              memset(v41,0x20,v12);
              v41 = &v41[v12];
            }
            else {
              memset(v41,0x30,v12);
              v41 = &v41[v12];
            }
          }
          *v41 = '\t';
          v41 = &v41[1];
        }
        v40 += v13;
        goto label_187d8;
      case 0x75:
        v27 = 1;
        v28 = (*(int *)&a3->field_0x18 + 6) % 7 + 1;
label_16c6b:
        v23 = (bool)(v28 >> 0x1f);
        v29 = v28;
label_16c86:
        v7 = 0;
label_16c8d:
        v30 = 0;
label_16c97:
        if ((v8 == 'O') && ((bool)v23 != 1)) goto label_16842;
        v44 = &v21[1];
        if ((bool)v23)
          v29 = -v29;
        do {
          if (v30 & 1) {
            v44 = &v44[-1];
            *v44 = ':';
          }
          v30 = (int)v30 >> 1;
          v44 = &v44[-1];
          *v44 = (char)v29 + (char)(v29 / 10) * '\xf6' + '0';
          v29 /= 10;
        } while ((v29) || (v30));
label_16d70:
        if (!v26)
          v26 = 0x30;
        if (a6 <= -1)
          a6 = v27;
        if (v23) // branch-flip
          v8 = '-';
        else {
          v8 = (v7) ? '+' : '\0';
        }
        v9 = ((int)v20 + 0x17) - (int)v44;
        v11 = (a6 - (unsigned int)(v8 != '\0')) - v9;
        if ((v26 == 0x2d) || (v11 <= 0))
          v11 = 0;
        if (v8) {
          if (v26 == 0x5f) {
            if (v41) {
              memset(v41,0x20,(long)v11);
              v41 = &v41[v11];
            }
            v40 += v11;
            a6 -= v11;
          }
          if ((unsigned long)(a1 - v40) <= 1) {
            *__errno_location() = 0x22;
            return 0;
          }
          if (v41) {
            *v41 = v8;
            v41 = &v41[1];
          }
          v40 += 1;
          a6 -= 1;
        }
        v12 = (unsigned long)v9;
        if ((v26 != 0x2d) && (0 <= a6))
          v13 = (unsigned long)a6;
        else {
          v13 = 0;
        }
        v14 = v13;
        if (v13 <= v12)
          v14 = v12;
        if ((unsigned long)(a1 - v40) <= v14) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (v41) {
          if (v12 < v13) {
            v13 -= v12;
            if ((v26 != 0x30) && (v26 != 0x2b)) {
              memset(v41,0x20,v13);
              v41 = &v41[v13];
            }
            else {
              memset(v41,0x30,v13);
              v41 = &v41[v13];
            }
          }
          if (v24) // branch-flip
            sub_15e9d(v41,v44,v12);
          else {
            memcpy(v41,v44,v12);
          }
          v41 = &v41[v12];
        }
        v40 += v14;
        goto label_187d8;
      case 0x77:
        if (v8 != 'E') {
          v27 = 1;
          v28 = *(unsigned int *)&a3->field_0x18;
          goto label_16c6b;
        }
        break;
      case 0x78:
label_16b68:
        if (v8 != 'O') goto label_16842;
        break;
      case 0x79:
        if (v8 != 'E') {
          v38 = *(int *)&a3->field_0x14 % 100;
          if ((int)v38 <= -1) {
            v38 = (-0x76c <= *(int *)&a3->field_0x14) ? v38 + 100 : -v38; // branch-flip
          }
          v27 = 2;
          v23 = 0;
          v29 = v38;
          goto label_16bf4;
        }
label_16842:
        v18[0] = ' ';
        v46 = &v18[2];
        v18[1] = 0x25;
        if (v8) {
          v46 = v19;
          v18[2] = v8;
        }
        *v46 = v31;
        v46[1] = '\0';
        v12 = strftime(v20,0x400,v18,a3);
        if (v12) {
          v12 -= 1;
          if ((v26 != 0x2d) && (0 <= a6))
            v13 = (unsigned long)a6;
          else {
            v13 = 0;
          }
          v14 = v13;
          if (v13 <= v12)
            v14 = v12;
          if ((unsigned long)(a1 - v40) <= v14) {
            *__errno_location() = 0x22;
            return 0;
          }
          if (v41) {
            if (v12 < v13) {
              v13 -= v12;
              if ((v26 != 0x30) && (v26 != 0x2b)) {
                memset(v41,0x20,v13);
                v41 = &v41[v13];
              }
              else {
                memset(v41,0x30,v13);
                v41 = &v41[v13];
              }
            }
            if (v3)
              sub_15e45(v41,&v20[1],v12);
            else if (v24) // branch-flip
              sub_15e9d(v41,&v20[1],v12);
            else {
              memcpy(v41,&v20[1],v12);
            }
            v41 = &v41[v12];
          }
          v40 += v14;
        }
        goto label_187d8;
      case 0x7a:
        v45 = 0;
label_183a7:
        if (*(int *)&a3->field_0x20 < 0) goto label_187d8;
        v9 = (int)*(unsigned long *)&a3->field_0x28;
        v23 = !((0 <= v9) && ((v9 || (*v39 != '-'))));
        v29 = v9 / 0xe10;
        v11 = (v9 / 0x3c) % 0x3c;
        if (v45 != 3) {
          if (v45 <= 3) {
            if (v45 == 2) goto label_1855c;
            if (v45 <= 2) {
              if (!v45) {
                v27 = 5;
                v30 = 0;
                v29 = v11 + v29 * 100;
                goto label_16be8;
              }
              if (v45 == 1) goto label_1852b;
            }
          }
          break;
        }
        if (v9 % 0x3c) { // branch-flip
label_1855c:
          v27 = 9;
          v30 = 0x14;
          v29 = v9 % 0x3c + v29 * 10000 + v11 * 100;
        }
        else if (v11) { // branch-flip
label_1852b:
          v27 = 6;
          v30 = 4;
          v29 = v11 + v29 * 100;
        }
        else {
          v27 = 3;
          v30 = 0;
        }
label_16be8:
        v7 = 1;
        goto label_16c97;
      
    }
    v15 = &v42[1 - (long)v6];
    if ((v26 != 0x2d) && (0 <= a6))
      v16 = (char *)(long)a6;
    else {
      v16 = NULL;
    }
    v17 = v16;
    if (v16 <= v15)
      v17 = v15;
    if ((char *)(a1 - v40) <= v17) {
      *__errno_location() = 0x22;
      return 0;
    }
    if (v41) {
      if (v15 < v16) {
        v12 = (long)v16 - (long)v15;
        if ((v26 != 0x30) && (v26 != 0x2b)) {
          memset(v41,0x20,v12);
          v41 = &v41[v12];
        }
        else {
          memset(v41,0x30,v12);
          v41 = &v41[v12];
        }
      }
      if (v24) // branch-flip
        sub_15e9d(v41,v6,v15);
      else {
        memcpy(v41,v6,(unsigned long)v15);
      }
      v41 = &v41[(long)v15];
    }
    v40 += (long)v17;
label_187d8:
    a6 = -1;
    v42 = &v42[1];
  } while( true );
}

// Function: sub_18845 @ 0x18845
void sub_18845(char *a0,unsigned long a1) // return-dupe
{
  if (a0[0x50] & 1) {
    (**(void **)&a0[0x38])(*(unsigned long *)&a0[0x48],a1);
    return;
  }
  (**(void **)&a0[0x38])(a1);
}

// Function: sub_18899 @ 0x18899
void sub_18899(char *a0,unsigned long a1) // return-dupe
{
  if (a0[0x50] & 1) {
    (**(void **)&a0[0x40])(*(unsigned long *)&a0[0x48],a1);
    return;
  }
  (**(void **)&a0[0x40])(a1);
}

// Function: sub_188ee @ 0x188ee
unsigned long sub_188ee(struct_20 *a0,long a1,long a2)
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
  a0->field_0x0 = v4;
  a0->field_0x30 = v3 + -1;
  v1 = a0->field_0x0;
  a0->field_0x8 = (long *)sub_18845(a0,v1);
  v2 = a0->field_0x8;
  if (!v2)
    (*obstack_alloc_failed_handler)();
  a0->field_0x10 = -v3 & (long)v2 + v3 + 0xf;
  a0->field_0x18 = a0->field_0x10;
  *v2 = a0->field_0x0 + (long)v2;
  a0->field_0x20 = *v2;
  v2[1] = 0;
  a0->field_0x50 = a0->field_0x50 & 0xfd;
  a0->field_0x50 = a0->field_0x50 & 0xfb;
  return 1;
}

// Function: _obstack_begin @ 0x18a14
int _obstack_begin(obstack *a0,unsigned long a1,unsigned long a2,void *a3,void *a4)
{
  unsigned char v1; // dl
  
  *(void **)&a0->field_0x38 = a3;
  *(void **)&a0->field_0x40 = a4;
  v1 = a0->field_0x50 & 0xfe;
  a0->field_0x50 = v1;
  return sub_188ee(a0,a1,a2);
}

// Function: _obstack_begin_1 @ 0x18a73
int _obstack_begin_1(obstack *a0,unsigned long a1,unsigned long a2,void *a3,void *a4,void *a5)
{
  unsigned char v1; // dl
  
  *(void **)&a0->field_0x38 = a3;
  *(void **)&a0->field_0x40 = a4;
  *(void **)&a0->field_0x48 = a5;
  v1 = a0->field_0x50 | 1;
  a0->field_0x50 = v1;
  return sub_188ee(a0,a1,a2);
}

// Function: _obstack_newchunk @ 0x18ae2
void _obstack_newchunk(obstack *a0,unsigned long a1)
{
  long v1;
  unsigned long v2; // rax
  unsigned long v3; // rax
  void *v4; // rax
  unsigned long *v5; // stack - 0x40
  unsigned long v6; // stack - 0x38
  
  v1 = *(long *)&a0->field_0x8;
  v5 = NULL;
  v2 = *(long *)&a0->field_0x18 - *(long *)&a0->field_0x10;
  a1 += v2;
  v3 = a1 + *(long *)&a0->field_0x30;
  v6 = v3 + (v2 >> 3) + 100;
  if (v6 < v3)
    v6 = v3;
  if (v6 < *(unsigned long *)a0)
    v6 = *(unsigned long *)a0;
  if ((v2 <= a1) && (a1 <= v3))
    v5 = (unsigned long *)sub_18845(a0,v6);
  if (!v5)
    (*obstack_alloc_failed_handler)();
  *(unsigned long **)&a0->field_0x8 = v5;
  v5[1] = v1;
  *(unsigned long *)&a0->field_0x20 = (long)v5 + v6;
  *v5 = *(unsigned long *)&a0->field_0x20;
  v4 = (void *)(~*(unsigned long *)&a0->field_0x30 & (long)v5 + *(long *)&a0->field_0x30 + 0x10);
  memcpy(v4,*(void **)&a0->field_0x10,v2);
  if ((!(a0->field_0x50 & 2)) && (*(unsigned long *)&a0->field_0x10 == (~*(unsigned long *)&a0->field_0x30 & v1 + 0x10 + *(long *)&a0->field_0x30))) {
    v5[1] = *(unsigned long *)(v1 + 8);
    sub_18899(a0,v1);
  }
  *(void **)&a0->field_0x10 = v4;
  *(unsigned long *)&a0->field_0x18 = *(long *)&a0->field_0x10 + v2;
  a0->field_0x50 = a0->field_0x50 & 0xfd;
}

// Function: _obstack_allocated_p @ 0x18cd9
bool _obstack_allocated_p(char *a0,unsigned long *a1)
{
  unsigned long *v1; // stack - 0x18
  
  for (v1 = *(unsigned long **)&a0[8]; (v1 && ((a1 <= v1 || ((unsigned long *)*v1 < a1)))); v1 = (unsigned long *)v1[1]) {
  }
  return v1 != NULL;
}

// Function: _obstack_free @ 0x18d36
void _obstack_free(obstack *a0,void *a1) // return-dupe
{
  unsigned long *v1;
  unsigned long *v2; // stack - 0x18
  
  v2 = *(unsigned long **)&a0->field_0x8;
  while ((v2 && ((a1 <= v2 || ((void *)*v2 < a1))))) {
    v1 = (unsigned long *)v2[1];
    sub_18899(a0,v2);
    a0->field_0x50 = a0->field_0x50 | 2;
    v2 = v1;
  }
  if (v2) {
    *(void **)&a0->field_0x18 = a1;
    *(unsigned long *)&a0->field_0x10 = *(unsigned long *)&a0->field_0x18;
    *(unsigned long *)&a0->field_0x20 = *v2;
    *(unsigned long **)&a0->field_0x8 = v2;
    return;
  }
  if (!a1)
    return;
  abort(); // no-return
}

// Function: _obstack_memory_used @ 0x18dfa
unsigned long _obstack_memory_used(obstack *a0)
{
  long *v1; // stack - 0x18
  unsigned long v2; // stack - 0x10
  
  v2 = 0;
  for (v1 = *(long **)&a0->field_0x8; v1; v1 = (long *)v1[1]) {
    v2 += *v1 - (long)v1;
  }
  return v2;
}

// Function: sub_18e44 @ 0x18e44
void sub_18e44(void)
{
  fprintf(stderr,"%s\n",gettext("memory exhausted"));
  exit(dat_2b228); // no-return
}

// Function: sub_18e89 @ 0x18e89
void sub_18e89(char *a0)
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
  dat_2c478 = v2;
  program_invocation_name = v2;
}

// Function: sub_18f74 @ 0x18f74
unsigned long sub_18f74(void *a0)
{
  int v1;
  unsigned long v2; // rax
  
  v1 = *__errno_location();
  if (!a0)
    a0 = (void *)0x2c480;
  v2 = sub_1cbcb(a0,0x38);
  *__errno_location() = v1;
  return v2;
}

// Function: sub_18fc3 @ 0x18fc3
unsigned int sub_18fc3(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x2c480;
  return *a0;
}

// Function: sub_18fe7 @ 0x18fe7
void sub_18fe7(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x2c480;
  *a0 = a1;
}

// Function: sub_19012 @ 0x19012
unsigned int sub_19012(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int v1; // eax
  unsigned int *v2; // rax
  
  if (!a0)
    a0 = 0x2c480;
  v2 = (unsigned int *)((unsigned long)(a1 >> 5) * 4 + a0 + 8);
  v1 = *v2 >> (a1 & 0x1f) & 1;
  *v2 = *v2 ^ (a2 & 1 ^ v1) << (a1 & 0x1f);
  return v1;
}

// Function: sub_190a2 @ 0x190a2
unsigned int sub_190a2(char *a0,unsigned int a1)
{
  unsigned int v1;
  char *v2; // stack - 0x20
  
  v2 = a0;
  if (!a0)
    v2 = (char *)0x2c480;
  v1 = *(unsigned int *)&v2[4];
  *(unsigned int *)&v2[4] = a1;
  return v1;
}

// Function: sub_190dc @ 0x190dc
void sub_190dc(unsigned int *a0,long a1,long a2)
{
  unsigned int *v1; // stack - 0x10
  
  v1 = a0;
  if (!a0)
    v1 = (unsigned int *)0x2c480;
  *v1 = 10;
  if ((a1) && (a2)) {
    *(long *)&v1[10] = a1;
    *(long *)&v1[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_1913e @ 0x1913e
unsigned long * sub_1913e(unsigned long *a0,unsigned int a1)
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

// Function: sub_19203 @ 0x19203
char * sub_19203(char *a0,int a1) // early-return x6
{
  char *v1; // rax
  char *v2; // rax
  
  v1 = gettext(a0);
  if (v1 != a0)
    return v1;
  v2 = (char *)sub_1ebbf();
  if (!sub_1e773(v2,"UTF-8")) {
    if (*a0 == '`')
      return "‘";
    return "’";
  }
  if (sub_1e773(v2,"GB18030")) {
    if (a1 == 9)
      return "\"";
    return "\'";
  }
  if (*a0 == '`')
    return (char *)0x238fe;
  return (char *)0x23902;
}

// Function: sub_192c8 @ 0x192c8
unsigned long sub_192c8(long a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
{
  char *v1;
  unsigned long v10;
  unsigned long v11;
  mbstate_t v12;
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
  bool v7;
  long v8;
  unsigned long v9; // rax
  
  v8 = a6;
  v18 = a7;
  v17 = a8;
  v25 = 0;
  v26 = 0;
  v27 = NULL;
  v28 = 0;
  v2 = 0;
  v9 = __ctype_get_mb_cur_max();
  v14 = (a5 & 2) != 0;
  v3 = 0;
  v6 = 0;
  v7 = 1;
  v19 = a4;
  v20 = a3;
  v21 = a1;
label_19382:
  switch(v19) {
    case 0:
      v14 = 0;
      break;
    case 1:
label_194d6:
      v14 = 1;
      goto label_194da;
    case 2:
label_194e9:
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
      v2 = 1;
      goto label_194d6;
    case 4:
label_194da:
      if (!v14)
        v2 = 1;
      goto label_194e9;
    case 5:
label_193c4:
      if (!v14) {
        if (v25 < v21)
          *(char *)(v25 + a0) = 0x22;
        v25 += 1;
      }
      v2 = 1;
      v27 = "\"";
      v28 = 1;
      break;
    case 6:
      v19 = 5;
      v14 = 1;
      goto label_193c4;
    case 7:
      v2 = 1;
      v14 = 0;
      break;
    case 8:
    case 9:
    case 10:
      if (v19 != 10) {
        v18 = (char *)sub_19203("`",v19);
        v17 = (char *)sub_19203("\'",v19);
      }
      if (!v14) {
        for (v27 = v18; *v27; v27 = &v27[1]) {
          if (v25 < v21)
            *(char *)(a0 + v25) = *v27;
          v25 += 1;
        }
      }
      v2 = 1;
      v27 = v17;
      v28 = strlen(v17);
      break;
    default:
      abort(); // no-return
    
  }
  v24 = 0;
label_1a188:
  if (v20 != 0xffffffffffffffff) // branch-flip
    v16 = v24 != v20;
  else {
    v16 = a2[v24] != '\0';
  }
  if (!v16) goto label_1a1c0;
  v4 = 0;
  v5 = 0;
  v16 = 0;
  if (((v2) && (v19 != 2)) && (v28)) {
    v11 = v24 + v28;
    if ((v20 != 0xffffffffffffffff) || (v28 <= 1))
      v10 = v20;
    else {
      v20 = strlen(a2);
      v10 = v20;
    }
    if ((v11 <= v10) && (v1 = &a2[v24], !memcmp(v1,v27,v28))) {
      if (v14) goto label_1a304;
      v4 = 1;
    }
  }
  v22 = a2[v24];
  if (v22 == 0x7e) {
label_19aa7:
    if (!v24) {
label_19ab2:
      v16 = 1;
label_19ab6:
      if (v19 != 2) goto label_19fcd;
label_1988f:
      if (v14) goto label_1a304;
    }
    goto label_19fcd;
  }
  if (0x7e < v22) {
label_19b93:
    if (v9 != 1) {
      memset(&v12,0,8);
      v29 = 0;
      v16 = 1;
      if (v20 == 0xffffffffffffffff)
        v20 = strlen(a2);
label_19c0d:
      v31 = sub_150be(&v13,&a2[v24 + v29],v20 - (v24 + v29),&v12);
      if (v31) {
        if (v31 != 0xffffffffffffffff) { // branch-flip
          if (v31 != 0xfffffffffffffffe) goto label_19cbb;
          v16 = 0;
          for (; (v29 + v24 < v20 && (a2[v24 + v29])); v29 = v29 + 1) {
          }
        }
        else {
          v16 = 0;
        }
      }
      goto label_19d6a;
    }
    v29 = 1;
    v16 = (*(unsigned short *)((unsigned long)v22 * 2 + *(long *)__ctype_b_loc()) & 0x4000) != 0;
label_19d6a:
    if ((v29 <= 1) && ((!v2 || (v16)))) goto label_19fcd;
    v32 = v29 + v24;
    while( true ) {
      if ((!v2) || (v16)) {
        if (v4) {
          if (v25 < v21)
            *(char *)(v25 + a0) = 0x5c;
          v25 += 1;
          v4 = 0;
        }
      }
      else {
        if (v14) goto label_1a304;
        v5 = 1;
        if ((v19 == 2) && (!v3)) {
          if (v25 < v21)
            *(char *)(v25 + a0) = 0x27;
          if (v25 + 1 < v21)
            *(char *)(v25 + 1 + a0) = 0x24;
          if (v25 + 2 < v21)
            *(char *)(v25 + 2 + a0) = 0x27;
          v25 += 3;
          v3 = 1;
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
      if ((v3) && (!v5)) {
        if (v25 < v21)
          *(char *)(v25 + a0) = 0x27;
        if (v25 + 1 < v21)
          *(char *)(v25 + 1 + a0) = 0x27;
        v25 += 2;
        v3 = 0;
      }
      if (v25 < v21)
        *(unsigned char *)(a0 + v25) = v22;
      v25 += 1;
      v24 += 1;
      v22 = a2[v24];
    }
    goto label_1a0f3;
  }
  if (v22 == 0x7d) {
label_19a75:
    if (v20 != 0xffffffffffffffff) // branch-flip
      v15 = v20 != 1;
    else {
      v15 = a2[1] != '\0';
    }
    if (v15) goto label_19fcd;
    goto label_19aa7;
  }
  if (0x7d < v22) goto label_19b93;
  if (v22 == 0x7c) goto label_19ab6;
  if (0x7c < v22) goto label_19b93;
  if (v22 == 0x7b) goto label_19a75;
  if (0x7b < v22) goto label_19b93;
  if (0x3f < v22) {
    if ((0x7a < v22) || (v22 <= 0x40)) goto label_19b93;
    v11 = 1L << (v22 + 0xbf & 0x3f);
    if (v11 & 0x3ffffff53ffffff) goto label_19b8a;
    if (v11 & 0xa4000000) goto label_19ab6;
    if (!(v11 & 0x8000000)) goto label_19b93;
    if (v19 == 2) {
      if (!v14) goto label_1a0f3;
      goto label_1a304;
    }
    v23 = v22;
    if (((!v2) || (!v14)) || (!v28)) goto label_19a4b;
    goto label_1a0f3;
  }
  switch(v22) {
    case 0:
      if (!v2) {
        if (!(a5 & 1)) goto label_19fcd;
        goto label_1a183;
      }
      if (!v14) {
        v5 = 1;
        if ((v19 == 2) && (!v3)) {
          if (v25 < v21)
            *(char *)(v25 + a0) = 0x27;
          if (v25 + 1 < v21)
            *(char *)(v25 + 1 + a0) = 0x24;
          if (v25 + 2 < v21)
            *(char *)(v25 + 2 + a0) = 0x27;
          v25 += 3;
          v3 = 1;
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
        goto label_19fcd;
      }
      goto label_1a304;
    default:
      goto label_19b93;
    case 7:
      v23 = 0x61;
      break;
    case 8:
      v23 = 0x62;
      break;
    case 9:
      v23 = 0x74;
      goto label_19a4b;
    case 10:
      v23 = 0x6e;
      goto label_19a4b;
    case 0xb:
      v23 = 0x76;
      break;
    case 0xc:
      v23 = 0x66;
      break;
    case 0xd:
      v23 = 0x72;
label_19a4b:
      if ((v19 == 2) && (v14)) goto label_1a304;
      break;
    case 0x20:
      goto label_19ab2;
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
      goto label_19ab6;
    case 0x23:
      goto label_19aa7;
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
label_19b8a:
      v16 = 1;
      goto label_19fcd;
    case 0x27:
      v6 = 1;
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
          v3 = 0;
          goto label_19fcd;
        }
        goto label_1a304;
      }
      goto label_19fcd;
    case 0x3f:
      if (v19 == 2) goto label_1988f;
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
          goto label_19fcd;
        }
        goto label_1a304;
      }
      goto label_19fcd;
    
  }
  if (v2) {
    v22 = v23;
label_1a038:
    if (v14) goto label_1a304;
    v5 = 1;
    if ((v19 == 2) && (!v3)) {
      if (v25 < v21)
        *(char *)(v25 + a0) = 0x27;
      if (v25 + 1 < v21)
        *(char *)(v25 + 1 + a0) = 0x24;
      if (v25 + 2 < v21)
        *(char *)(v25 + 2 + a0) = 0x27;
      v25 += 3;
      v3 = 1;
    }
    if (v25 < v21)
      *(char *)(v25 + a0) = 0x5c;
    v25 += 1;
  }
  else {
label_19fcd:
    if (((((v2) && (v19 != 2)) || (v14)) && ((v8 && (*(unsigned int *)(v8 + (unsigned long)(v22 >> 5) * 4) >> (v22 & 0x1f) & 1)))) || (v4)) goto label_1a038;
  }
label_1a0f3:
  if ((v3) && (!v5)) {
    if (v25 < v21)
      *(char *)(v25 + a0) = 0x27;
    if (v25 + 1 < v21)
      *(char *)(v25 + 1 + a0) = 0x27;
    v25 += 2;
    v3 = 0;
  }
  if (v25 < v21)
    *(unsigned char *)(a0 + v25) = v22;
  v25 += 1;
  if (!v16)
    v7 = 0;
label_1a183:
  v24 += 1;
  goto label_1a188;
label_19cbb:
  if ((v14) && (v19 == 2)) {
    for (v30 = 1; v30 < v31; v30 = v30 + 1) {
      if (((unsigned int)((int)a2[v24 + v29 + v30] - 0x5bU) <= 0x21) && (0x20000002bU >> ((unsigned char)((int)a2[v24 + v29 + v30] - 0x5bU) & 0x3f) & 1)) goto label_1a304;
    }
  }
  if (!iswprint(v13))
    v16 = 0;
  v29 += v31;
  if (mbsinit(&v12)) goto label_19d6a;
  goto label_19c0d;
label_1a1c0:
  if (((!v25) && (v19 == 2)) && (v14)) {
label_1a304:
    if ((v19 == 2) && (v2))
      v19 = 4;
    return sub_192c8(a0,v21,a2,v20,v19,a5 & 0xfffffffd,0,v18,v17);
  }
  if (((v19 == 2) && (!v14)) && (v6)) {
    if (v7)
      return sub_192c8(a0,v26,a2,v20,5,a5,v8,v18,v17);
    if ((!v21) && (v26)) {
      v21 = v26;
      v25 = 0;
      goto label_19382;
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

// Function: sub_1a389 @ 0x1a389
unsigned long sub_1a389(long a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  unsigned long v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x2c480;
  v1 = *__errno_location();
  v2 = sub_192c8(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *__errno_location() = v1;
  return v2;
}

// Function: sub_1a431 @ 0x1a431
void sub_1a431(char *a0,unsigned long a1,unsigned int *a2)
{
  sub_1a467(a0,a1,NULL,a2);
}

// Function: sub_1a467 @ 0x1a467
long sub_1a467(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  unsigned int v2; // eax
  long v3; // rax
  long v4; // rax
  
  if (!a3)
    a3 = (unsigned int *)0x2c480;
  v1 = *__errno_location();
  v2 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_192c8(0,0,a0,a1,*a3,v2,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = sub_1c6d7(v3 + 1U);
  sub_192c8(v4,v3 + 1U,a0,a1,*a3,v2,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *__errno_location() = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_1a593 @ 0x1a593
void sub_1a593(void) // return-dupe
{
  void *v1;
  int v2; // stack - 0x14
  
  v1 = dat_2b2b0;
  for (v2 = 1; v2 < dat_2b240; v2 = v2 + 1) {
    free(*(void **)((long)v1 + (long)v2 * 0x10 + 8));
  }
  if (*(long *)((long)v1 + 8) != 0x2c4c0) {
    free(*(void **)((long)v1 + 8));
    dat_2b2a0 = 0x100;
    dat_2b2a8 = 0x2c4c0;
  }
  if (v1 == (void *)0x2b2a0) {
    dat_2b240 = 1;
    return;
  }
  free(v1);
  dat_2b240 = 1;
  dat_2b2b0 = (void *)0x2b2a0;
}

// Function: sub_1a652 @ 0x1a652
void * sub_1a652(int a0,char *a1,unsigned long a2,struct_3 *a3)
{
  int v1;
  unsigned int v2; // eax
  unsigned long *v3;
  long v4; // stack - 0x38
  bool v5; // zf
  unsigned long *v6; // stack - 0x30
  void *v7; // stack - 0x28
  unsigned long v8; // stack - 0x20
  unsigned long v9; // stack - 0x18
  
  v1 = *__errno_location();
  v6 = dat_2b2b0;
  if ((0 <= a0) && (a0 <= 0x7ffffffe)) {
    if (dat_2b240 <= a0) {
      v5 = dat_2b2b0 == (unsigned long *)0x2b2a0;
      v4 = (long)dat_2b240;
      if (v5)
        v3 = NULL;
      else {
        v3 = dat_2b2b0;
      }
      dat_2b2b0 = (unsigned long *)sub_1c967(v3,&v4,(long)((a0 - dat_2b240) + 1),0x7fffffff,0x10);
      v6 = dat_2b2b0;
      if (v5) {
        *dat_2b2b0 = dat_2b2a0;
        dat_2b2b0[1] = dat_2b2a8;
      }
      memset(&dat_2b2b0[(long)dat_2b240 * 2],0,(v4 - dat_2b240) * 0x10);
      dat_2b240 = (int)v4;
    }
    v8 = v6[(long)a0 * 2];
    v7 = (void *)v6[(long)a0 * 2 + 1];
    v2 = a3->field_0x4 | 1;
    v9 = sub_192c8(v7,v8,a1,a2,a3->field_0x0,v2,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v8 <= v9) {
      v8 = v9 + 1;
      v6[(long)a0 * 2] = v8;
      if (v7 != (void *)0x2c4c0)
        free(v7);
      v7 = (void *)sub_1c6d7(v8);
      v6[(long)a0 * 2 + 1] = v7;
      sub_192c8(v7,v8,a1,a2,a3->field_0x0,v2,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *__errno_location() = v1;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_1a901 @ 0x1a901
void sub_1a901(unsigned int a0,char *a1)
{
  sub_1a652(a0,a1,0xffffffffffffffff,(struct_3 *)0x2c480);
}

// Function: sub_1a935 @ 0x1a935
void sub_1a935(unsigned int a0,char *a1,unsigned long a2)
{
  sub_1a652(a0,a1,a2,(struct_3 *)0x2c480);
}

// Function: sub_1a967 @ 0x1a967
void sub_1a967(char *a0)
{
  sub_1a901(0,a0);
}

// Function: sub_1a98a @ 0x1a98a
void sub_1a98a(char *a0,unsigned long a1)
{
  sub_1a935(0,a0,a1);
}

// Function: sub_1a9b5 @ 0x1a9b5
void sub_1a9b5(unsigned int a0,unsigned int a1,char *a2)
{
  char v1 [56];
  
  sub_1913e(v1,a1);
  sub_1a652(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_1aa1d @ 0x1aa1d
void sub_1aa1d(unsigned int a0,unsigned int a1,char *a2,unsigned long a3)
{
  char v1 [56];
  
  sub_1913e(v1,a1);
  sub_1a652(a0,a2,a3,v1);
}

// Function: sub_1aa83 @ 0x1aa83
void sub_1aa83(unsigned int a0,char *a1)
{
  sub_1a9b5(0,a0,a1);
}

// Function: sub_1aaab @ 0x1aaab
void sub_1aaab(unsigned int a0,char *a1,unsigned long a2)
{
  sub_1aa1d(0,a0,a1,a2);
}

// Function: sub_1aadb @ 0x1aadb
void sub_1aadb(char *a0,unsigned long a1,char a2)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v1 = dat_2c480;
  v2 = dat_2c488;
  v3 = dat_2c490;
  v4 = dat_2c498;
  v5 = dat_2c4a0;
  v6 = dat_2c4a8;
  v7 = dat_2c4b0;
  sub_19012(&v1,(int)a2,1);
  sub_1a652(0,a0,a1,&v1);
}

// Function: sub_1ab96 @ 0x1ab96
void sub_1ab96(char *a0,char a1)
{
  sub_1aadb(a0,0xffffffffffffffff,(int)a1);
}

// Function: sub_1abc4 @ 0x1abc4
void sub_1abc4(char *a0)
{
  sub_1ab96(a0,0x3a);
}

// Function: sub_1abe7 @ 0x1abe7
void sub_1abe7(char *a0,unsigned long a1)
{
  sub_1aadb(a0,a1,0x3a);
}

// Function: sub_1ac15 @ 0x1ac15
void sub_1ac15(unsigned int a0,unsigned int a1,char *a2)
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
  char *v9; // stack - 0x58
  
  v9 = a2;
  v10 = a1;
  v11 = a0;
  sub_1913e(&v1,a1);
  v2 = v1;
  v12 = v3;
  v13 = v4;
  v14 = v5;
  v15 = v6;
  v16 = v7;
  v17 = v8;
  sub_19012(&v2,0x3a,1);
  sub_1a652(v11,v9,0xffffffffffffffff,&v2);
}

// Function: sub_1acd7 @ 0x1acd7
void sub_1acd7(unsigned int a0,long a1,long a2,char *a3)
{
  sub_1ad11(a0,a1,a2,a3,0xffffffffffffffff);
}

// Function: sub_1ad11 @ 0x1ad11
void sub_1ad11(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v1 = dat_2c480;
  v2 = dat_2c488;
  v3 = dat_2c490;
  v4 = dat_2c498;
  v5 = dat_2c4a0;
  v6 = dat_2c4a8;
  v7 = dat_2c4b0;
  sub_190dc(&v1,a1,a2);
  sub_1a652(a0,a3,a4,&v1);
}

// Function: sub_1adcf @ 0x1adcf
void sub_1adcf(long a0,long a1,char *a2)
{
  sub_1acd7(0,a0,a1,a2);
}

// Function: sub_1ae02 @ 0x1ae02
void sub_1ae02(long a0,long a1,char *a2,unsigned long a3)
{
  sub_1ad11(0,a0,a1,a2,a3);
}

// Function: sub_1ae40 @ 0x1ae40
void sub_1ae40(unsigned int a0,char *a1,unsigned long a2)
{
  sub_1a652(a0,a1,a2,(struct_3 *)0x2b260);
}

// Function: sub_1ae72 @ 0x1ae72
void sub_1ae72(char *a0,unsigned long a1)
{
  sub_1ae40(0,a0,a1);
}

// Function: sub_1ae9d @ 0x1ae9d
void sub_1ae9d(unsigned int a0,char *a1)
{
  sub_1ae40(a0,a1,0xffffffffffffffff);
}

// Function: sub_1aeca @ 0x1aeca
void sub_1aeca(char *a0)
{
  sub_1ae9d(0,a0);
}

// Function: sub_1aeed @ 0x1aeed
struct_14 * sub_1aeed(struct_14 *a0,unsigned long a1) // early-return
{
  void *v1; // rax
  struct_14 *v2;
  struct_14 *v3; // rax
  
  v2 = (struct_14 *)a0->field_0x0;
  if (v2 != &a0[1]) { // branch-flip
    v3 = realloc(v2,a1);
    if (v3)
      v2 = v3;
  }
  else {
    v1 = malloc(a1);
    if (!v1)
      return NULL;
    v2 = memcpy(v1,v2,a1);
  }
  return v2;
}

// Function: sub_1af7b @ 0x1af7b
void sub_1af7b(struct_14 *a0)
{
  a0->field_0x0 = (long *)&a0[1];
  a0->field_0x8 = 0x400;
}

// Function: sub_1afa1 @ 0x1afa1
void sub_1afa1(long *a0) // return-dupe
{
  if ((long *)*a0 == &a0[2])
    return;
  free((void *)*a0);
}

// Function: sub_1afd3 @ 0x1afd3
bool sub_1afd3(struct_14 *a0)
{
  unsigned long v1;
  long *v2; // stack - 0x18
  
  v1 = a0->field_0x8 * 2;
  sub_1afa1(a0);
  if ((unsigned long)a0->field_0x8 <= v1) // branch-flip
    v2 = malloc(v1);
  else {
    *__errno_location() = 0xc;
    v2 = NULL;
  }
  if (v2) {
    a0->field_0x0 = v2;
    a0->field_0x8 = v1;
  }
  else {
    sub_1af7b(a0);
  }
  return v2 != NULL;
}

// Function: sub_1b07b @ 0x1b07b
void sub_1b07b(struct_14 *a0)
{
  a0->field_0x0 = (long *)&a0[1];
  a0->field_0x8 = 0x400;
}

// Function: sub_1b0a1 @ 0x1b0a1
unsigned long sub_1b0a1(struct_14 *a0) // return-dupe x2
{
  long *v1;
  unsigned long v2;
  long *v3; // stack - 0x18
  
  v2 = a0->field_0x8 * 2;
  if ((struct_14 *)a0->field_0x0 != &a0[1]) { // branch-flip
    if ((unsigned long)a0->field_0x8 <= v2) { // branch-flip
      v1 = a0->field_0x0;
      v3 = realloc(v1,v2);
    }
    else {
      *__errno_location() = 0xc;
      v3 = NULL;
    }
    if (!v3) {
      free(a0->field_0x0);
      sub_1b07b(a0);
      return 0;
    }
  }
  else {
    v3 = malloc(v2);
    if (!v3)
      return 0;
    memcpy(v3,&a0[1],a0->field_0x8);
  }
  a0->field_0x0 = v3;
  a0->field_0x8 = v2;
  return 1;
}

// Function: sub_1b1ac @ 0x1b1ac
int sub_1b1ac(int a0,char **a1) // early-return x2
{
  char *v1;
  
  if (!a0) {
    *__errno_location() = 0x5f;
    return -1;
  }
  if ((a0 == 10) && (v1 = *a1, !strcmp(v1,"unlabeled"))) {
    freecon(*a1);
    *a1 = NULL;
    *__errno_location() = 0x3d;
    return -1;
  }
  return a0;
}

// Function: sub_1b22b @ 0x1b22b
void sub_1b22b(char *a0,char **a1)
{
  sub_1b1ac(getfilecon(a0,a1),a1);
}

// Function: sub_1b268 @ 0x1b268
void sub_1b268(char *a0,char **a1)
{
  sub_1b1ac(lgetfilecon(a0,a1),a1);
}

// Function: sub_1b2a5 @ 0x1b2a5
void sub_1b2a5(int a0,char **a1)
{
  sub_1b1ac(fgetfilecon(a0,a1),a1);
}

// Function: sub_1b2df @ 0x1b2df
char * sub_1b2df(int a0)
{
  return setlocale(a0,NULL);
}

// Function: sub_1b307 @ 0x1b307
unsigned long sub_1b307(unsigned int a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_1b2df(a0);
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

// Function: sub_1b3bf @ 0x1b3bf
void sub_1b3bf(unsigned int a0,char *a1,unsigned long a2)
{
  sub_1b307(a0,a1,a2);
}

// Function: sub_1b3ed @ 0x1b3ed
void sub_1b3ed(unsigned int a0)
{
  sub_1b2df(a0);
}

// Function: sub_1b408 @ 0x1b408
unsigned long sub_1b408(char *a0)
{
  return *(unsigned long *)&a0[0x50];
}

// Function: sub_1b41e @ 0x1b41e
unsigned long sub_1b41e(char *a0)
{
  return *(unsigned long *)&a0[0x70];
}

// Function: sub_1b434 @ 0x1b434
unsigned long sub_1b434(char *a0)
{
  return *(unsigned long *)&a0[0x60];
}

// Function: sub_1b44a @ 0x1b44a
unsigned long sub_1b44a(void)
{
  return 0;
}

// Function: sub_1b45d @ 0x1b45d
undefined16 sub_1b45d(char *a0)
{
  return *(char (*)[16])&a0[0x48];
}

// Function: sub_1b477 @ 0x1b477
undefined16 sub_1b477(char *a0)
{
  return *(char (*)[16])&a0[0x68];
}

// Function: sub_1b491 @ 0x1b491
undefined16 sub_1b491(char *a0)
{
  return *(char (*)[16])&a0[0x58];
}

// Function: sub_1b4ab @ 0x1b4ab
undefined16 sub_1b4ab(void)
{
  char v1 [16];
  
  v1._8_8_ = 0xffffffffffffffff;
  v1._0_8_ = 0xffffffffffffffff;
  return v1._0_16_;
}

// Function: sub_1b4d1 @ 0x1b4d1
unsigned int sub_1b4d1(unsigned int a0)
{
  return a0;
}

// Function: sub_1b4e5 @ 0x1b4e5
void sub_1b4e5(void *a0,void *a1,unsigned long a2)
{
  memcpy(a0,a1,a2);
  *(char *)(a2 + (long)a0) = 0;
}

// Function: sub_1b525 @ 0x1b525
unsigned long * sub_1b525(char *a0) // ternary
{
  unsigned long v1; // rax
  unsigned long v2;
  unsigned long *v3; // rax
  
  v1 = (a0) ? strlen(a0) + 1 : 0; // branch-flip
  v2 = v1;
  if (v1 <= 0x75)
    v2 = 0x76;
  v3 = malloc(v2 + 0x11 & 0xfffffffffffffff8);
  if (v3) {
    *v3 = 0;
    *(bool *)&v3[1] = a0 != NULL;
    *(char *)((long)v3 + 9) = 0;
    if (a0)
      sub_1b4e5((void *)((long)v3 + 9),a0,v1);
  }
  return v3;
}

// Function: sub_1b5da @ 0x1b5da
unsigned long sub_1b5da(long *a0,struct_22 *a1) // return-dupe
{
  struct_22 *v1;
  long v2;
  long v3; // rax
  long *v4; // stack - 0x30
  char *v5; // stack - 0x20
  
  v5 = "";
  v1 = (struct_22 *)a1->field_0x30;
  if ((!v1) || ((a1 <= v1 && (v1 < &a1[1]))))
    return 1;
  if (v1->field_0x0[0]) {
    v5 = (char *)((long)a0 + 9);
    v4 = a0;
    while (strcmp(v5,(char *)v1)) {
      if ((!*v5) && ((v5 != (char *)((long)v4 + 9) || (!(char)v4[1])))) {
        v3 = strlen((char *)v1) + 1;
        if ((long)v4 + (0x80U - (long)v5) <= v3) { // branch-flip
          *v4 = sub_1b525(v1);
          v2 = *v4;
          if (!v2)
            return 0;
          *(char *)(v2 + 8) = 0;
          v5 = (char *)(v2 + 9);
        }
        else {
          sub_1b4e5(v5,v1,v3);
        }
        break;
      }
      v5 = &v5[strlen(v5) + 1];
      if ((!*v5) && (*v4)) {
        v4 = (long *)*v4;
        v5 = (char *)((long)v4 + 9);
      }
    }
  }
  a1->field_0x30 = v5;
  return 1;
}

// Function: sub_1b77e @ 0x1b77e
void sub_1b77e(unsigned long *a0) // return-dupe
{
  unsigned long *v1;
  unsigned long *v2; // stack - 0x20
  
  v2 = a0;
  if (a0 == (unsigned long *)0x1)
    return;
  while (v2) {
    v1 = (unsigned long *)*v2;
    free(v2);
    v2 = v1;
  }
}

// Function: sub_1b7c4 @ 0x1b7c4
void sub_1b7c4(void)
{
  getenv("TZ");
}

// Function: sub_1b7dd @ 0x1b7dd
void sub_1b7dd(char *a0) // return-dupe
{
  if (a0) {
    setenv("TZ",a0,1);
    return;
  }
  unsetenv("TZ");
}

// Function: sub_1b822 @ 0x1b822
bool sub_1b822(char *a0) // ternary
{
  char *v1; // rax
  bool v2; // zf
  
  v1 = (a0[8]) ? &a0[9] : NULL; // branch-flip
  v2 = sub_1b7dd(v1) == 0;
  if (v2)
    tzset();
  return v2;
}

// Function: sub_1b86c @ 0x1b86c
unsigned long * sub_1b86c(char *a0) // early-return x2
{
  int v1;
  char *v2; // rax
  unsigned long *v3; // rax
  bool v4;
  
  v2 = (char *)sub_1b7c4();
  if (v2) { // branch-flip
    if ((a0[8]) && (!strcmp(&a0[9],v2)))
      v4 = 1;
    else {
      v4 = 0;
    }
  }
  else {
    v4 = a0[8] == '\0';
  }
  if (v4)
    return (unsigned long *)0x1;
  v3 = (unsigned long *)sub_1b525(v2);
  if ((v3) && (sub_1b822(a0) != '\x01')) {
    v1 = *__errno_location();
    sub_1b77e(v3);
    *__errno_location() = v1;
    return NULL;
  }
  return v3;
}

// Function: sub_1b939 @ 0x1b939
char sub_1b939(unsigned long *a0) // early-return
{
  char v1; // al
  int v2; // stack - 0xc
  
  if (a0 == (unsigned long *)0x1)
    return '\x01';
  v2 = *__errno_location();
  v1 = sub_1b822(a0);
  if (v1 != '\x01')
    v2 = *__errno_location();
  sub_1b77e(a0);
  *__errno_location() = v2;
  return v1;
}

// Function: sub_1b9a5 @ 0x1b9a5
struct_22 * sub_1b9a5(long *a0,void *a1,struct_22 *a2) // early-return, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_1b9a5
{
  bool v1; // al
  unsigned long *v2; // rax
  
  if (!a0)
    return (struct_22 *)gmtime_r(a1,(tm *)a2);
  v2 = (unsigned long *)sub_1b86c(a0);
  if (v2) {
    v1 = ((localtime_r(a1,(tm *)a2)) && (sub_1b5da(a0,a2)));
    if ((sub_1b939(v2)) && (v1))
      return a2;
  }
  return NULL;
}

// Function: sub_1ba55 @ 0x1ba55
unsigned long sub_1ba55(long *a0,unsigned long *a1) // early-return, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_1ba55
{
  bool v1; // al
  unsigned int v10; // stack - 0x40
  int v11; // stack - 0x3c
  unsigned int v12; // stack - 0x38
  unsigned int v13; // stack - 0x34
  unsigned long v14; // stack - 0x30
  unsigned long v15; // stack - 0x28
  unsigned long *v2; // rax
  unsigned long v3; // rax
  unsigned int v4; // stack - 0x58
  unsigned int v5; // stack - 0x54
  unsigned int v6; // stack - 0x50
  unsigned int v7; // stack - 0x4c
  unsigned int v8; // stack - 0x48
  unsigned int v9; // stack - 0x44
  
  if (!a0)
    return sub_1bbbb(a1);
  v2 = (unsigned long *)sub_1b86c(a0);
  if (v2) {
    v4 = *(unsigned int *)a1;
    v5 = *(unsigned int *)((long)a1 + 4);
    v6 = *(unsigned int *)&a1[1];
    v7 = *(unsigned int *)((long)a1 + 0xc);
    v8 = *(unsigned int *)&a1[2];
    v9 = *(unsigned int *)((long)a1 + 0x14);
    v11 = -1;
    v12 = *(unsigned int *)&a1[4];
    v3 = sub_1e57f(&v4);
    v1 = ((0 <= v11) && (sub_1b5da(a0,&v4)));
    if ((sub_1b939(v2)) && (v1)) {
      *a1 = CONCAT44(v5,v4);
      a1[1] = CONCAT44(v7,v6);
      a1[2] = CONCAT44(v9,v8);
      a1[3] = CONCAT44(v11,v10);
      a1[4] = CONCAT44(v13,v12);
      a1[5] = v14;
      a1[6] = v15;
      return v3;
    }
  }
  return 0xffffffffffffffff;
}

// Function: sub_1bbbb @ 0x1bbbb
void sub_1bbbb(struct_31 *a0)
{
  a0->field_0x20 = 0;
  sub_1dd61(a0,dat_2afb8,(long *)0x2c5c0);
}

// Function: sub_1bbf5 @ 0x1bbf5
undefined16 sub_1bbf5(unsigned long a0,unsigned long a1)
{
  char v1 [16];
  
  v1._8_8_ = a1;
  v1._0_8_ = a0;
  return v1._0_16_;
}

// Function: sub_1bc1f @ 0x1bc1f
int sub_1bc1f(long a0,long a1,long a2,long a3)
{
  return ((unsigned int)(a3 < a1) - (unsigned int)(a1 < a3)) + ((unsigned int)(a2 < a0) - (unsigned int)(a0 < a2)) * 2;
}

// Function: sub_1bc95 @ 0x1bc95
int sub_1bc95(long a0,long a1)
{
  return (unsigned int)(a1 && !a0) + (unsigned int)(0 < a0) + (int)(a0 >> 0x3f);
}

// Function: sub_1bce7 @ 0x1bce7
double sub_1bce7(long a0,long a1)
{
  return (double)a1 / dat_23a58 + (double)a0;
}

// Function: sub_1bd36 @ 0x1bd36
void sub_1bd36(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5) // return-dupe x10
{
  unsigned long v1;
  char *v10;
  char *v11;
  char *v12;
  char *v13;
  char *v14;
  char *v15;
  char *v16;
  char *v17;
  char *v18;
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
      v10 = (char *)a4[8];
      v11 = (char *)a4[7];
      v12 = (char *)a4[6];
      v13 = (char *)a4[5];
      v14 = (char *)a4[4];
      v15 = (char *)a4[3];
      v16 = (char *)a4[2];
      v17 = (char *)a4[1];
      v18 = (char *)*a4;
      fprintf(a0,gettext("Written by %s, %s, %s,\n%s, %s, %s, %s,\n%s, %s, and others.\n"),v18,v17,v16,v15,v14,v13,v12,v11,v10);
      return;
    
  }
}

// Function: sub_1c332 @ 0x1c332
void sub_1c332(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4)
{
  long v1; // stack - 0x10
  
  for (v1 = 0; a4[v1]; v1 = v1 + 1) {
  }
  sub_1bd36(a0,a1,a2,a3,a4,v1);
}

// Function: sub_1c3a5 @ 0x1c3a5
void sub_1c3a5(FILE *a0,char *a1,char *a2,char *a3,struct_32 *a4)
{
  long *v1;
  long v2 [11];
  unsigned long v3; // stack - 0x70
  
  for (v3 = 0; v3 <= 9; v3 = v3 + 1) {
    if (0x30 <= a4->field_0x0) { // branch-flip
      v1 = a4->field_0x8;
      a4->field_0x8 = &v1[1];
    }
    else {
      v1 = (long *)((unsigned long)a4->field_0x0 + a4->field_0x10);
      a4->field_0x0 = a4->field_0x0 + 8;
    }
    v2[v3] = *v1;
    if (!v2[v3]) break;
  }
  sub_1bd36(a0,a1,a2,a3,v2,v3);
}

// Function: sub_1c4ab @ 0x1c4ab
void sub_1c4ab(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,FILE *a8,char *a9,char *a10,char *a11,unsigned long a12,unsigned long a13)
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
  sub_1c3a5(a8,a9,a10,a11,&v3);
}

// Function: sub_1c58f @ 0x1c58f
void sub_1c58f(void)
{
  fputs_unlocked("\n",stdout);
  printf(gettext("Report bugs to: %s\n"),"bug-coreutils@gnu.org");
  printf(gettext("%s home page: <%s>\n"),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  printf(gettext("General help using GNU software: <%s>\n"),"https://www.gnu.org/gethelp/");
}

// Function: sub_1c638 @ 0x1c638
void sub_1c638(void *a0,unsigned long a1,unsigned long a2)
{
  sub_1c771(a0,a1,a2);
}

// Function: sub_1c669 @ 0x1c669
long sub_1c669(long a0)
{
  if (!a0)
    sub_1ccc3(); // no-return
  return a0;
}

// Function: sub_1c68b @ 0x1c68b
void sub_1c68b(unsigned long a0)
{
  sub_1c669(malloc(a0));
}

// Function: sub_1c6b1 @ 0x1c6b1
void sub_1c6b1(unsigned long a0)
{
  sub_1c669(sub_1eaf2(a0));
}

// Function: sub_1c6d7 @ 0x1c6d7
void sub_1c6d7(unsigned long a0)
{
  sub_1c68b(a0);
}

// Function: sub_1c6f5 @ 0x1c6f5
void * sub_1c6f5(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = realloc(a0,a1);
  if ((!v1) && ((!a0 || (a1))))
    sub_1ccc3(); // no-return
  return v1;
}

// Function: sub_1c740 @ 0x1c740
void sub_1c740(void *a0,unsigned long a1)
{
  sub_1c669(sub_1eb10(a0,a1));
}

// Function: sub_1c771 @ 0x1c771
void * sub_1c771(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = reallocarray(a0,a1,a2);
  if ((!v1) && ((!a0 || ((a1 && (a2))))))
    sub_1ccc3(); // no-return
  return v1;
}

// Function: sub_1c7cb @ 0x1c7cb
void sub_1c7cb(void *a0,unsigned long a1,unsigned long a2)
{
  sub_1c669(sub_1eb70(a0,a1,a2));
}

// Function: sub_1c804 @ 0x1c804
void sub_1c804(unsigned long a0,unsigned long a1)
{
  sub_1c771(NULL,a0,a1);
}

// Function: sub_1c82f @ 0x1c82f
void sub_1c82f(unsigned long a0,unsigned long a1)
{
  sub_1c7cb(NULL,a0,a1);
}

// Function: sub_1c85a @ 0x1c85a
void sub_1c85a(void *a0,unsigned long *a1)
{
  sub_1c888(a0,a1,1);
}

// Function: sub_1c888 @ 0x1c888
unsigned long sub_1c888(void *a0,unsigned long *a1,unsigned long a2)
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
      sub_1ccc3(); // no-return
  }
  else if (!v6) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v2 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0);
    v6 = v2 + (unsigned long)(v2 == 0);
  }
  v4 = sub_1c771(a0,v6,a2);
  *a1 = v6;
  return v4;
}

// Function: sub_1c967 @ 0x1c967
unsigned long sub_1c967(void *a0,long *a1,long a2,long a3,long a4) // ternary
{
  long v1;
  long v2; // rax
  unsigned long v3; // rax
  long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  
  v1 = *a1;
  v4 = v1 + (v1 >> 1);
  if (SCARRY8(v1,v1 >> 1))
    v4 = 0x7fffffffffffffff;
  if ((0 <= a3) && (a3 < v4))
    v4 = a3;
  v5 = v4 * a4;
  if (SEXT816((long)v5) != SEXT816(v4) * SEXT816(a4)) // branch-flip
    v2 = 0x7fffffffffffffff;
  else {
    v2 = (0x80 <= (long)v5) ? 0 : 0x80; // branch-flip
  }
  if (v2) {
    v4 = v2 / a4;
    v5 = v2 - v2 % a4;
  }
  if (!a0)
    *a1 = 0;
  if ((v4 - v1 < a2) && ((v4 = a2 + v1, SCARRY8(a2,v1) || (((0 <= a3 && (a3 < v4)) || (v5 = v4 * a4, SEXT816((long)v5) != SEXT816(v4) * SEXT816(a4)))))))
    sub_1ccc3(); // no-return
  v3 = sub_1c6f5(a0,v5);
  *a1 = v4;
  return v3;
}

// Function: sub_1cb23 @ 0x1cb23
void sub_1cb23(unsigned long a0)
{
  sub_1cb69(a0,1);
}

// Function: sub_1cb46 @ 0x1cb46
void sub_1cb46(unsigned long a0)
{
  sub_1cb9a(a0,1);
}

// Function: sub_1cb69 @ 0x1cb69
void sub_1cb69(unsigned long a0,unsigned long a1)
{
  sub_1c669(calloc(a0,a1));
}

// Function: sub_1cb9a @ 0x1cb9a
void sub_1cb9a(unsigned long a0,unsigned long a1)
{
  sub_1c669(sub_1eb47(a0,a1));
}

// Function: sub_1cbcb @ 0x1cbcb
void sub_1cbcb(void *a0,unsigned long a1)
{
  memcpy((void *)sub_1c68b(a1),a0,a1);
}

// Function: sub_1cc03 @ 0x1cc03
void sub_1cc03(char *a0,unsigned long a1)
{
  memcpy((void *)sub_1c6b1(a1),a0,a1);
}

// Function: sub_1cc43 @ 0x1cc43
void sub_1cc43(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_1c6b1(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1);
}

// Function: sub_1cc92 @ 0x1cc92
void sub_1cc92(char *a0)
{
  sub_1cbcb(a0,strlen(a0) + 1);
}

// Function: sub_1ccc3 @ 0x1ccc3
void sub_1ccc3(void)
{
  error(dat_2b228,0,"%s",gettext("memory exhausted"));
  abort(); // no-return
}

// Function: sub_1cd03 @ 0x1cd03
unsigned long sub_1cd03(unsigned char *a0,unsigned int a1,unsigned long a2,unsigned long a3,char *a4,char *a5,int a6)
{
  int v1;
  char *v2; // rax
  unsigned long v3; // stack - 0x28
  int v4;
  int v5; // stack - 0x2c
  
  v5 = sub_1d251(a0,NULL,a1,&v3,a4);
  if (v5) { // branch-flip
    if (v5 != 1) { // branch-flip
      if (v5 == 3)
        *__errno_location() = 0;
    }
    else {
      *__errno_location() = 0x4b;
    }
  }
  else if ((v3 < a2) || (a3 < v3)) {
    v5 = 1;
    if (0x40000000 <= v3) // branch-flip
      *__errno_location() = 0x4b;
    else {
      *__errno_location() = 0x22;
    }
  }
  if (v5) {
    v2 = (char *)sub_1aeca(a0);
    if (*__errno_location() != 0x16) // branch-flip
      v4 = *__errno_location();
    else {
      v4 = 0;
    }
    if (a6) // branch-flip
      v1 = a6;
    else {
      v1 = 1;
    }
    error(v1,v4,"%s: %s",a5,v2);
  }
  return v3;
}

// Function: sub_1ce3d @ 0x1ce3d
void sub_1ce3d(unsigned char *a0,unsigned long a1,unsigned long a2,char *a3,char *a4,unsigned int a5)
{
  sub_1cd03(a0,10,a1,a2,a3,a4,a5);
}

// Function: sub_1ce96 @ 0x1ce96
char * sub_1ce96(void)
{
  char v1 [104];
  long v2; // stack - 0xa0
  char *v3; // stack - 0x98
  char *v4; // stack - 0x90
  unsigned long v5; // stack - 0x88
  unsigned long v6; // stack - 0x80
  
  v2 = 100;
  v3 = v1;
  v4 = NULL;
  while( true ) {
    v5 = v2 - 1;
    v3[v5] = '\0';
    *__errno_location() = 0;
    if (!gethostname(v3,v5)) {
      v6 = strlen(v3) + 1;
      if ((long)v6 < (long)v5) {
        if (!v4)
          return (char *)sub_1cc03(v3,v6);
        return v4;
      }
      *__errno_location() = 0;
    }
    free(v4);
    if ((((*__errno_location()) && (*__errno_location() != 0x24)) && (*__errno_location() != 0x16)) && (*__errno_location() != 0xc)) break;
    v4 = (char *)sub_1c967(NULL,&v2,1,-1,1);
    v3 = v4;
  }
  return NULL;
}

// Function: sub_1d012 @ 0x1d012
void sub_1d012(unsigned int a0,int a1,char a2,long a3,unsigned long a4,int a5)
{
  char v1; // stack - 0x12
  char *v2; // stack - 0x30
  char *v3; // stack - 0x28
  char *v4; // stack - 0x20
  char v5; // stack - 0x11
  
  v2 = "--";
  if (a0 != 4) { // branch-flip
    if (5 <= a0)
      abort(); // no-return, return-dupe
    if (a0 != 1) { // branch-flip
      if ((!a0) || (2 <= a0 - 2)) {
        abort();
      }
      v3 = "invalid suffix in %s%s argument \'%s\'";
    }
    else {
      v3 = "%s%s argument \'%s\' too large";
    }
  }
  else {
    v3 = "invalid %s%s argument \'%s\'";
  }
  if (0 <= a1) // branch-flip
    v4 = *(char **)(a3 + (long)a1 * 0x20);
  else {
    v2 = &"--"[-(long)a1];
    v5 = 0;
    v4 = &v1;
    v1 = a2;
  }
  error(a5,0,gettext(v3),v2,v4,a4);
}

// Function: sub_1d130 @ 0x1d130
void sub_1d130(unsigned int a0,unsigned int a1,char a2,long a3,unsigned long a4)
{
  sub_1d012(a0,a1,(int)a2,a3,a4,dat_2b228);
  abort(); // no-return
}

// Function: sub_1d17a @ 0x1d17a
bool sub_1d17a(unsigned long *a0,int a1)
{
  char v1 [16];
  bool v2;
  
  v2 = 0;
  if (((long)a1 <= -1) && (*a0))
    v2 = 1;
  v1 = ZEXT816((unsigned long)(long)a1) * ZEXT816(*a0);
  if (SUB168(v1,8))
    v2 = 1;
  if (!v2)
    *a0 = SUB168(v1,0);
  else {
    *a0 = 0xffffffffffffffff;
  }
  return v2;
}

// Function: sub_1d20c @ 0x1d20c
unsigned int sub_1d20c(unsigned long *a0,unsigned int a1,int a2)
{
  int v1;
  int v2; // stack - 0x28
  unsigned int v3; // stack - 0xc
  
  v3 = 0;
  v2 = a2;
  while (v1 = v2 + -1, v2) {
    v3 |= sub_1d17a(a0,a1);
    v2 = v1;
  }
  return v3;
}

// Function: sub_1d251 @ 0x1d251
unsigned int sub_1d251(unsigned char *a0,long *a1,int a2,unsigned long *a3,char *a4) // early-return x2
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
        v8 = sub_1d17a(&v3,0x400);
        break;
      default:
        *a3 = v3;
        return v5 | 2;
      case 0x45:
        v8 = sub_1d20c(&v3,v6,6);
        break;
      case 0x47:
      case 0x67:
        v8 = sub_1d20c(&v3,v6,3);
        break;
      case 0x4b:
      case 0x6b:
        v8 = sub_1d20c(&v3,v6,1);
        break;
      case 0x4d:
      case 0x6d:
        v8 = sub_1d20c(&v3,v6,2);
        break;
      case 0x50:
        v8 = sub_1d20c(&v3,v6,5);
        break;
      case 0x54:
      case 0x74:
        v8 = sub_1d20c(&v3,v6,4);
        break;
      case 0x59:
        v8 = sub_1d20c(&v3,v6,8);
        break;
      case 0x5a:
        v8 = sub_1d20c(&v3,v6,7);
        break;
      case 0x62:
        v8 = sub_1d17a(&v3,0x200);
        break;
      case 99:
        v8 = 0;
        break;
      case 0x77:
        v8 = sub_1d17a(&v3,2);
      
    }
    v5 |= v8;
    *v10 = *v10 + (long)v7;
    if (*(char *)*v10)
      v5 |= 2;
  }
  *a3 = v3;
  return v5;
}

// Function: sub_1d6af @ 0x1d6af
void sub_1d6af(void)
{
  tzset();
}

// Function: sub_1d6bf @ 0x1d6bf
long sub_1d6bf(long a0,unsigned char a1)
{
  return a0 >> (a1 & 0x3f);
}

// Function: sub_1d736 @ 0x1d736
unsigned int sub_1d736(unsigned long a0) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_1d736
{
  unsigned int v1; // eax
  
  v1 = !((a0 & 3) || ((!((long)a0 % 100) && (((unsigned int)((long)a0 / 100) & 3) != 1))));
  return v1;
}

// Function: sub_1d7df @ 0x1d7df
unsigned int sub_1d7df(int a0,int a1) // return-dupe
{
  if ((((a1 == 0) != (a0 == 0)) && (0 <= a0)) && (0 <= a1))
    return 1;
  return 0;
}

// Function: sub_1d81e @ 0x1d81e
long sub_1d81e(unsigned long a0,long a1,int a2,int a3,int a4,unsigned int a5,int a6,int a7,int a8,int a9)
{
  int v1; // eax
  int v2; // eax
  int v3; // eax
  int v4; // eax
  int v5; // eax
  int v6; // eax
  
  v1 = sub_1d6bf(a0,2);
  v1 = (v1 + sub_1d6bf(0x76c,2)) - (unsigned int)((a0 & 3) == 0);
  v2 = sub_1d6bf((long)(int)a5,2);
  v2 = (v2 + sub_1d6bf(0x76c,2)) - (unsigned int)((a5 & 3) == 0);
  v3 = (v1 - (v1 >> 0x1f)) / 0x19 + (v1 >> 0x1f);
  v4 = (v2 - (v2 >> 0x1f)) / 0x19 + (v2 >> 0x1f);
  v5 = sub_1d6bf((long)v3,2);
  v6 = sub_1d6bf((long)v4,2);
  return (((((((long)((v5 - v6) + ((v1 - v2) - (v3 - v4))) + (((a0 - (long)(int)a5) * 0x16d + a1) - (long)a6)) * 0x18 + (long)a2) - (long)a7) * 0x3c + (long)a3) - (long)a8) * 0x3c + (long)a4) - (long)a9;
}

// Function: sub_1da41 @ 0x1da41
long sub_1da41(long a0,long a1)
{
  long v1; // rax
  
  v1 = sub_1d6bf(a0,1);
  return (unsigned long)(((unsigned int)a0 | (unsigned int)a1) & 1) + v1 + sub_1d6bf(a1,1);
}

// Function: sub_1da93 @ 0x1da93
void sub_1da93(unsigned long a0,long a1,unsigned int a2,unsigned int a3,unsigned int a4,unsigned int *a5)
{
  sub_1d81e(a0,a1,a2,a3,a4,a5[5],a5[7],a5[2],a5[1],*a5);
}

// Function: sub_1db0c @ 0x1db0c
void sub_1db0c(void *a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x18
  
  v1 = a1;
  (*a0)(&v1,a2);
}

// Function: sub_1db65 @ 0x1db65
unsigned long * sub_1db65(void *a0,long *a1,unsigned long *a2) // early-return x3
{
  unsigned long *v1; // rax
  unsigned long v10; // stack - 0x30
  unsigned long v11; // stack - 0x28
  long v2; // rax
  long v3; // stack - 0x80
  long v4; // stack - 0x78
  unsigned long v5; // stack - 0x58
  unsigned long v6; // stack - 0x50
  unsigned long v7; // stack - 0x48
  unsigned long v8; // stack - 0x40
  unsigned long v9; // stack - 0x38
  
  if (-0x8000000000000000 <= *a1) { // branch-flip
    v3 = *a1;
    if (0x7fffffffffffffff < v3)
      v3 = 0x7fffffffffffffff;
  }
  else {
    v3 = -0x8000000000000000;
  }
  v1 = (unsigned long *)sub_1db0c(a0,v3,a2);
  if (v1) { // branch-flip
    *a1 = v3;
    a2 = v1;
  }
  else {
    if (*__errno_location() != 0x4b)
      return NULL;
    v4 = 0;
    v5 = CONCAT44(v5._4_4_,0xffffffff);
    while ((v2 = sub_1da41(v4,v3), v2 != v4 && (v2 != v3))) {
      if (sub_1db0c(a0,v2,a2)) { // branch-flip
        v5 = *a2;
        v6 = a2[1];
        v7 = a2[2];
        v8 = a2[3];
        v9 = a2[4];
        v10 = a2[5];
        v11 = a2[6];
        v4 = v2;
      }
      else {
        if (*__errno_location() != 0x4b)
          return NULL;
        v3 = v2;
      }
    }
    if ((int)v5 <= -1)
      return NULL;
    *a1 = v4;
    *a2 = v5;
    a2[1] = v6;
    a2[2] = v7;
    a2[3] = v8;
    a2[4] = v9;
    a2[5] = v10;
    a2[6] = v11;
  }
  return a2;
}

// Function: sub_1dd61 @ 0x1dd61
long sub_1dd61(int *a0,void *a1,long *a2) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_1dd61
{
  int v1;
  int v10;
  long v11; // rax
  int v12; // edx
  int v13; // stack - 0x98
  long v14; // stack - 0x100
  char v15 [32];
  long v16; // stack - 0xf8
  bool v17;
  int v18; // stack - 0x150
  int v19; // stack - 0x14c
  int v2;
  int v20; // stack - 0x144
  int v21; // stack - 0x140
  long v22; // stack - 0xf0
  long v23; // stack - 0xe8
  long v24; // stack - 0xe0
  unsigned long v25; // stack - 0xd8
  long v26; // stack - 0xd0
  long v27; // stack - 0xc8
  long v28; // stack - 0xc0
  long v29; // stack - 0xb8
  int v3;
  long v30; // stack - 0xb0
  long v31; // stack - 0xa8
  long v32; // stack - 0xa0
  unsigned int v33; // stack - 0x94
  unsigned long v34; // stack - 0x90
  unsigned long v35; // stack - 0x88
  unsigned long v36; // stack - 0x80
  int v37; // stack - 0x78
  unsigned int v38; // stack - 0x74
  unsigned long v39; // stack - 0x70
  int v4;
  unsigned long v40; // stack - 0x68
  unsigned int v41; // stack - 0x38
  int v5;
  int v6;
  int v7;
  unsigned char v8; // al
  char v9; // al
  
  v18 = 6;
  v1 = *a0;
  v10 = a0[1];
  v2 = a0[2];
  v3 = a0[3];
  v4 = a0[4];
  v5 = a0[8];
  v17 = 0;
  v6 = v4 >> 0x1f;
  v12 = v4 + ((v4 / 6 + v6 >> 1) - v6) * -0xc;
  v7 = v12 >> 0x1f;
  v24 = (long)a0[5];
  v25 = v24 + (((v4 / 6 + v6 >> 1) - v6) + v7);
  v8 = sub_1d736(v25);
  v26 = (long)v3;
  v27 = v26 + (int)((unsigned int)*(unsigned short *)(((long)(int)(unsigned int)v8 * 0xd + (long)(v12 + v7 * -0xc)) * 2 + 0x23fe0) - 1);
  v28 = *a2;
  v19 = v1;
  if (v1 <= -1)
    v19 = 0;
  if (0x3c <= v19)
    v19 = 0x3b;
  v3 = (int)v28;
  v29 = sub_1d81e(v25,v27,v2,v10,v19,0x46,0,0,0,-v3);
  v14 = v29;
  v22 = v29;
  v23 = v29;
  while( true ) {
    if (!sub_1db65(a1,&v14,&v13))
      return -1;
    v30 = sub_1da93(v25,v27,v2,v10,v19,&v13);
    if (!v30) break;
    if ((v22 == v14) && (v23 != v14)) {
      if (v37 < 0) goto label_1e3e7;
      if (0 <= v5) { // branch-flip
        if ((v37 != 0) != (v5 != 0)) goto label_1e3e7;
      }
      else if (v17 <= (v37 != 0)) goto label_1e3e7;
    }
    v18 -= 1;
    if (!v18) {
      *__errno_location() = 0x4b;
      return -1;
    }
    v22 = v23;
    v23 = v14;
    v14 = v30 + v14;
    v17 = v37 != 0;
  }
  v9 = sub_1d7df(v5,v37);
  if (v9) {
    v17 = v37 == 0;
    for (v20 = 0x92c70; v20 < 0xda9a994; v20 = v20 + 0x92c70) {
      for (v21 = -1; v21 <= 1; v21 = v21 + 2) {
        v16 = v20 * v21 + v14;
        if (!SCARRY8((long)(v20 * v21),v14)) {
          if (!sub_1db65(a1,&v16,v15))
            return -1;
          v9 = sub_1d7df(v5,v41);
          if (v9 != '\x01') {
            v11 = sub_1da93(v25,v27,v2,v10,v19,v15);
            v31 = v11 + v16;
            if (-0x8000000000000000 <= v31) {
              if (sub_1db0c(a1,v31,&v13)) {
                v14 = v31;
                goto label_1e3e7;
              }
              if (*__errno_location() != 0x4b)
                return -1;
            }
          }
        }
      }
    }
    v14 += (int)(((unsigned int)(v5 == 0) - (unsigned int)v17) * 0xe10);
    if (!sub_1db0c(a1,v14,&v13)) {
      *__errno_location() = 0x4b;
      return -1;
    }
  }
label_1e3e7:
  *a2 = v14 - v29;
  *a2 = *a2 - (long)-v3;
  if (v1 != v13) {
    v10 = !((v19) || (v13 != 0x3c));
    v32 = ((long)v10 - (long)v19) + (long)v1;
    v17 = SCARRY8(v32,v14);
    v14 = v32 + v14;
    if ((v17) || (0x7fffffffffffffff < v14)) {
      *__errno_location() = 0x4b;
      return -1;
    }
    if (!sub_1db0c(a1,v14,&v13))
      return -1;
  }
  *(unsigned long *)a0 = CONCAT44(v33,v13);
  *(unsigned long *)&a0[2] = v34;
  *(unsigned long *)&a0[4] = v35;
  *(unsigned long *)&a0[6] = v36;
  *(unsigned long *)&a0[8] = CONCAT44(v38,v37);
  *(unsigned long *)&a0[10] = v39;
  *(unsigned long *)&a0[0xc] = v40;
  return v14;
}

// Function: sub_1e57f @ 0x1e57f
void sub_1e57f(int *a0)
{
  sub_1d6af();
  sub_1dd61(a0,dat_2afb0,(long *)0x2c5c8);
}

// Function: sub_1e5b3 @ 0x1e5b3
unsigned long sub_1e5b3(unsigned long a0,unsigned char a1)
{
  return a0 << (a1 & 0x3f) | a0 >> 0x40 - (a1 & 0x3f);
}

// Function: sub_1e5d3 @ 0x1e5d3
unsigned long sub_1e5d3(unsigned long a0,unsigned char a1)
{
  return a0 >> (a1 & 0x3f) | a0 << 0x40 - (a1 & 0x3f);
}

// Function: sub_1e5f3 @ 0x1e5f3
unsigned int sub_1e5f3(unsigned int a0,unsigned char a1)
{
  return a0 << (a1 & 0x1f) | a0 >> 0x20 - (a1 & 0x1f);
}

// Function: sub_1e60f @ 0x1e60f
unsigned int sub_1e60f(unsigned int a0,unsigned char a1)
{
  return a0 >> (a1 & 0x1f) | a0 << 0x20 - (a1 & 0x1f);
}

// Function: sub_1e62b @ 0x1e62b
unsigned long sub_1e62b(unsigned long a0,unsigned char a1)
{
  return a0 >> (0x40 - a1 & 0x3f) | a0 << (a1 & 0x3f);
}

// Function: sub_1e664 @ 0x1e664
unsigned long sub_1e664(unsigned long a0,unsigned char a1)
{
  return a0 << (0x40 - a1 & 0x3f) | a0 >> (a1 & 0x3f);
}

// Function: sub_1e69d @ 0x1e69d
unsigned int sub_1e69d(unsigned short a0,unsigned char a1)
{
  return (unsigned int)(a0 >> (0x10 - a1 & 0x1f)) | (unsigned int)a0 << (a1 & 0x1f);
}

// Function: sub_1e6d3 @ 0x1e6d3
unsigned int sub_1e6d3(unsigned short a0,unsigned char a1)
{
  return (unsigned int)a0 << (0x10 - a1 & 0x1f) | (unsigned int)(a0 >> (a1 & 0x1f));
}

// Function: sub_1e709 @ 0x1e709
unsigned int sub_1e709(unsigned char a0,unsigned char a1)
{
  return (unsigned int)(a0 >> (8 - a1 & 0x1f)) | (unsigned int)a0 << (a1 & 0x1f);
}

// Function: sub_1e73e @ 0x1e73e
unsigned int sub_1e73e(unsigned char a0,unsigned char a1)
{
  return (unsigned int)a0 << (8 - a1 & 0x1f) | (unsigned int)(a0 >> (a1 & 0x1f));
}

// Function: sub_1e773 @ 0x1e773
int sub_1e773(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned char v2; // al
  
  if (a0 == a1)
    return 0;
  do {
    v1 = sub_1026f((unsigned char)*a0);
    v2 = sub_1026f((unsigned char)*a1);
    if (!v1) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while (v1 == v2);
  return (unsigned int)v1 - (unsigned int)v2;
}

// Function: sub_1e7ed @ 0x1e7ed
unsigned long sub_1e7ed(FILE *a0)
{
  int v1; // eax
  unsigned long v2; // rax
  bool v3; // zf
  
  v2 = __fpending(a0);
  v1 = ferror_unlocked(a0);
  v3 = sub_1e87c(a0) == 0;
  if ((!v1) && ((v3 || ((!v2 && (*__errno_location() == 9))))))
    return 0;
  if (v3)
    *__errno_location() = 0;
  return 0xffffffff;
}

// Function: sub_1e87c @ 0x1e87c
unsigned long sub_1e87c(FILE *a0)
{
  int v1; // eax
  int v2; // stack - 0x14
  unsigned int v3; // stack - 0x10
  
  v2 = 0;
  if (fileno(a0) <= -1)
    return CONCAT44(dat_4,fclose(a0));
  if (__freading(a0)) {
    v1 = fileno(a0);
    if (lseek(v1,0,1) == -1) goto label_1e90a;
  }
  if (sub_1e96d(a0))
    v2 = *__errno_location();
label_1e90a:
  v3 = fclose(a0);
  if (v2) {
    *__errno_location() = v2;
    v3 = 0xffffffff;
  }
  return (unsigned long)v3;
}

// Function: sub_1e935 @ 0x1e935
void sub_1e935(FILE *a0) // return-dupe
{
  if (!(*(unsigned int *)a0 & 0x100))
    return;
  sub_1e9bc(a0,0,1);
}

// Function: sub_1e96d @ 0x1e96d
void sub_1e96d(FILE *a0)
{
  if ((a0) && (__freading(a0))) {
    sub_1e935(a0);
    fflush(a0);
    return;
  }
  fflush(a0);
}

// Function: sub_1e9bc @ 0x1e9bc
unsigned long sub_1e9bc(FILE *a0,long a1,int a2) // early-return x2
{
  long v1; // rax
  
  if (((*(long *)&a0->field_0x10 == *(long *)&a0->field_0x8) && (*(long *)&a0->field_0x28 == *(long *)&a0->field_0x20)) && (!*(long *)&a0->field_0x48)) {
    v1 = lseek(fileno(a0),a1,a2);
    if (v1 == -1)
      return 0xffffffff;
    *(unsigned int *)a0 = *(unsigned int *)a0 & 0xffffffef;
    *(long *)&a0->field_0x90 = v1;
    return 0;
  }
  return CONCAT44(dat_4,fseeko(a0,a1,a2));
}

// Function: sub_1ea7a @ 0x1ea7a
unsigned long sub_1ea7a(char *a0,unsigned long a1)
{
  char *v1; // stack - 0x18
  unsigned long v2; // stack - 0x10
  
  v2 = 0;
  for (v1 = a0; *v1; v1 = &v1[1]) {
    v2 = (v2 << 9 | v2 >> 0x37) + (long)*v1;
  }
  return v2 % a1;
}

// Function: sub_1ead8 @ 0x1ead8
unsigned long sub_1ead8(void)
{
  *__errno_location() = 0xc;
  return 0;
}

// Function: sub_1eaf2 @ 0x1eaf2
void sub_1eaf2(unsigned long a0)
{
  malloc(a0);
}

// Function: sub_1eb10 @ 0x1eb10
void sub_1eb10(void *a0,unsigned long a1)
{
  realloc(a0,a1 == 0 | a1);
}

// Function: sub_1eb47 @ 0x1eb47
void sub_1eb47(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1);
}

// Function: sub_1eb70 @ 0x1eb70
void sub_1eb70(void *a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x20
  unsigned long v2; // stack - 0x18
  
  if ((!a1) || (v1 = a2, v2 = a1, !a2)) {
    v1 = 1;
    v2 = 1;
  }
  reallocarray(a0,v2,v1);
}

// Function: sub_1ebbf @ 0x1ebbf
char * sub_1ebbf(void)
{
  char *v1; // stack - 0x10
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    v1 = "";
  if (!*v1)
    v1 = "ASCII";
  return v1;
}

// Function: sub_1ec10 @ 0x1ec10
void sub_1ec10(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_2b008); // tail-call
}

// Function: _DT_FINI @ 0x1ec24
void _DT_FINI(void)
{
  return;
}

