// Function: _DT_INIT @ 0x4000
void _DT_INIT(void) // return-dupe
{
  if (!dat_25fd0)
    return;
  (*dat_25fd0)();
}

// Function: sub_4020 @ 0x4020
void sub_4020(void)
{
  (*dat_25be0)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x47d0
void __cxa_finalize(void)
{
  (*dat_25fe0)(); // jump-as-call
}

// Function: __ctype_toupper_loc @ 0x47e0
void * __ctype_toupper_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25be8)(); // jump-as-call
  return v1;
}

// Function: getenv @ 0x47f0
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25bf0)(); // jump-as-call
  return v1;
}

// Function: sigprocmask @ 0x4800
int sigprocmask(int a0,sigset_t *a1,sigset_t *a2)
{
  int v1; // eax
  
  v1 = (*dat_25bf8)(); // jump-as-call
  return v1;
}

// Function: raise @ 0x4810
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_25c00)(); // jump-as-call
  return v1;
}

// Function: free @ 0x4820
void free(void *a0)
{
  (*dat_25c08)(); // jump-as-call
}

// Function: strtoimax @ 0x4830
void strtoimax(void)
{
  (*dat_25c10)(); // jump-as-call
}

// Function: strcasecmp @ 0x4840
int strcasecmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_25c18)(); // jump-as-call
  return v1;
}

// Function: localtime @ 0x4850
tm * localtime(void *a0)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_25c20)(); // jump-as-call
  return v1;
}

// Function: __mempcpy_chk @ 0x4860
void * __mempcpy_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25c28)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x4870
void abort(void)
{
  (*dat_25c30)(); // jump-as-call
}

// Function: __errno_location @ 0x4880
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_25c38)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x4890
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_25c40)(); // jump-as-call
  return v1;
}

// Function: localtime_r @ 0x48a0
tm * localtime_r(void *a0,tm *a1)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_25c48)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x48b0
void _exit(int a0)
{
  (*dat_25c50)(); // jump-as-call
}

// Function: strcpy @ 0x48c0
char * strcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25c58)(); // jump-as-call
  return v1;
}

// Function: iconv @ 0x48d0
void iconv(void)
{
  (*dat_25c60)(); // jump-as-call
}

// Function: ferror @ 0x48e0
int ferror(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_25c68)(); // jump-as-call
  return v1;
}

// Function: qsort @ 0x48f0
void qsort(void)
{
  (*dat_25c70)(); // jump-as-call
}

// Function: isatty @ 0x4900
int isatty(int a0)
{
  int v1; // eax
  
  v1 = (*dat_25c78)(); // jump-as-call
  return v1;
}

// Function: sigaction @ 0x4910
int sigaction(int a0,sigaction *a1,sigaction *a2)
{
  int v1; // eax
  
  v1 = (*dat_25c80)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x4920
int iswcntrl(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_25c88)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x4930
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25c90)(); // jump-as-call
  return v1;
}

// Function: readlink @ 0x4940
long readlink(char *a0,char *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_25c98)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x4950
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_25ca0)(); // jump-as-call
  return v1;
}

// Function: clock_gettime @ 0x4960
int clock_gettime(int a0,timespec *a1)
{
  int v1; // eax
  
  v1 = (*dat_25ca8)(); // jump-as-call
  return v1;
}

// Function: setenv @ 0x4970
int setenv(char *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_25cb0)(); // jump-as-call
  return v1;
}

// Function: write @ 0x4980
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_25cb8)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x4990
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25cc0)(); // jump-as-call
  return v1;
}

// Function: __open_2 @ 0x49a0
int __open_2(char *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_25cc8)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x49b0
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_25cd0)(); // jump-as-call
  return v1;
}

// Function: opendir @ 0x49c0
DIR * opendir(char *a0)
{
  DIR *v1; // rax
  
  v1 = (DIR *)(*dat_25cd8)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x49d0
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25ce0)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x49e0
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25ce8)(); // jump-as-call
  return v1;
}

// Function: regfree @ 0x49f0
void regfree(re_pattern_buffer *a0)
{
  (*dat_25cf0)(); // jump-as-call
}

// Function: __ctype_get_mb_cur_max @ 0x4a00
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_25cf8)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x4a10
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_25d00)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x4a20
void __stack_chk_fail(void)
{
  (*dat_25d08)(); // jump-as-call
}

// Function: mmap @ 0x4a30
void mmap(void)
{
  (*dat_25d10)(); // jump-as-call
}

// Function: getopt_long @ 0x4a40
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_25d18)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x4a50
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_25d20)(); // jump-as-call
  return v1;
}

// Function: dup2 @ 0x4a60
int dup2(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_25d28)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x4a70
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25d30)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x4a80
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25d38)(); // jump-as-call
  return v1;
}

// Function: gmtime_r @ 0x4a90
tm * gmtime_r(void *a0,tm *a1)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_25d40)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x4aa0
void lseek(void)
{
  (*dat_25d48)(); // jump-as-call
}

// Function: __assert_fail @ 0x4ab0
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_25d50)(); // jump-as-call
}

// Function: fputs @ 0x4ac0
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_25d58)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x4ad0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25d60)(); // jump-as-call
  return v1;
}

// Function: strnlen @ 0x4ae0
unsigned long strnlen(char *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_25d68)(); // jump-as-call
  return v1;
}

// Function: close @ 0x4af0
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_25d70)(); // jump-as-call
  return v1;
}

// Function: pipe @ 0x4b00
int pipe(int *a0)
{
  int v1; // eax
  
  v1 = (*dat_25d78)(); // jump-as-call
  return v1;
}

// Function: closedir @ 0x4b10
int closedir(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_25d80)(); // jump-as-call
  return v1;
}

// Function: regcomp @ 0x4b20
int regcomp(re_pattern_buffer *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_25d88)(); // jump-as-call
  return v1;
}

// Function: memchr @ 0x4b30
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25d90)(); // jump-as-call
  return v1;
}

// Function: read @ 0x4b40
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_25d98)(); // jump-as-call
  return v1;
}

// Function: lstat @ 0x4b50
int lstat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_25da0)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x4b60
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_25da8)(); // jump-as-call
  return v1;
}

// Function: _setjmp @ 0x4b70
int _setjmp(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_25db0)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x4b80
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_25db8)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x4b90
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25dc0)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x4ba0
int ferror_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_25dc8)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x4bb0
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25dd0)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x4bc0
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_25dd8)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x4bd0
void signal(void)
{
  (*dat_25de0)(); // jump-as-call
}

// Function: fputc_unlocked @ 0x4be0
int fputc_unlocked(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_25de8)(); // jump-as-call
  return v1;
}

// Function: sigemptyset @ 0x4bf0
int sigemptyset(sigset_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_25df0)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x4c00
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_25df8)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x4c10
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25e00)(); // jump-as-call
  return v1;
}

// Function: putchar_unlocked @ 0x4c20
int putchar_unlocked(int a0)
{
  int v1; // eax
  
  v1 = (*dat_25e08)(); // jump-as-call
  return v1;
}

// Function: tzset @ 0x4c30
void tzset(void)
{
  (*dat_25e10)(); // jump-as-call
}

// Function: fileno @ 0x4c40
int fileno(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_25e18)(); // jump-as-call
  return v1;
}

// Function: readdir @ 0x4c50
dirent * readdir(DIR *a0)
{
  dirent *v1; // rax
  
  v1 = (dirent *)(*dat_25e20)(); // jump-as-call
  return v1;
}

// Function: mincore @ 0x4c60
int mincore(void *a0,unsigned long a1,char *a2)
{
  int v1; // eax
  
  v1 = (*dat_25e28)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x4c70
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_25e30)(); // jump-as-call
  return v1;
}

// Function: putc_unlocked @ 0x4c80
int putc_unlocked(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_25e38)(); // jump-as-call
  return v1;
}

// Function: re_set_syntax @ 0x4c90
void re_set_syntax(void)
{
  (*dat_25e40)(); // jump-as-call
}

// Function: iswalnum @ 0x4ca0
int iswalnum(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_25e48)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x4cb0
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25e50)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x4cc0
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25e58)(); // jump-as-call
  return v1;
}

// Function: __vasprintf_chk @ 0x4cd0
int __vasprintf_chk(char **a0,int a1,char *a2,void *a3)
{
  int v1; // eax
  
  v1 = (*dat_25e60)(); // jump-as-call
  return v1;
}

// Function: regexec @ 0x4ce0
int regexec(re_pattern_buffer *a0,char *a1,unsigned long a2,void *a3,int a4)
{
  int v1; // eax
  
  v1 = (*dat_25e68)(); // jump-as-call
  return v1;
}

// Function: strcoll @ 0x4cf0
int strcoll(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_25e70)(); // jump-as-call
  return v1;
}

// Function: mktime @ 0x4d00
long mktime(tm *a0)
{
  long v1; // rax
  
  v1 = (*dat_25e78)(); // jump-as-call
  return v1;
}

// Function: re_search @ 0x4d10
int re_search(re_pattern_buffer *a0,char *a1,int a2,int a3,int a4,void *a5)
{
  int v1; // eax
  
  v1 = (*dat_25e80)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x4d20
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_25e88)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x4d30
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25e90)(); // jump-as-call
  return v1;
}

// Function: fdopen @ 0x4d40
FILE * fdopen(int a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_25e98)(); // jump-as-call
  return v1;
}

// Function: munmap @ 0x4d50
int munmap(void *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = (*dat_25ea0)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x4d60
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25ea8)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x4d70
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_25eb0)(); // jump-as-call
  return v1;
}

// Function: iconv_close @ 0x4d80
void iconv_close(void)
{
  (*dat_25eb8)(); // jump-as-call
}

// Function: timegm @ 0x4d90
long timegm(tm *a0)
{
  long v1; // rax
  
  v1 = (*dat_25ec0)(); // jump-as-call
  return v1;
}

// Function: iswspace @ 0x4da0
int iswspace(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_25ec8)(); // jump-as-call
  return v1;
}

// Function: strftime @ 0x4db0
unsigned long strftime(char *a0,unsigned long a1,char *a2,tm *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_25ed0)(); // jump-as-call
  return v1;
}

// Function: mempcpy @ 0x4dc0
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25ed8)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x4dd0
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25ee0)(); // jump-as-call
  return v1;
}

// Function: error @ 0x4de0
void error(int a0,int a1,char *a2,...)
{
  (*dat_25ee8)(); // jump-as-call
}

// Function: waitpid @ 0x4df0
int waitpid(int a0,int *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_25ef0)(); // jump-as-call
  return v1;
}

// Function: open @ 0x4e00
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_25ef8)(); // jump-as-call
  return v1;
}

// Function: fopen @ 0x4e10
FILE * fopen(char *a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_25f00)(); // jump-as-call
  return v1;
}

// Function: perror @ 0x4e20
void perror(char *a0)
{
  (*dat_25f08)(); // jump-as-call
}

// Function: towlower @ 0x4e30
unsigned int towlower(unsigned int a0)
{
  unsigned int v1; // eax
  
  v1 = (*dat_25f10)(); // jump-as-call
  return v1;
}

// Function: unsetenv @ 0x4e40
int unsetenv(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_25f18)(); // jump-as-call
  return v1;
}

// Function: getpagesize @ 0x4e50
int getpagesize(void)
{
  int v1; // eax
  
  v1 = (*dat_25f20)(); // jump-as-call
  return v1;
}

// Function: sigismember @ 0x4e60
int sigismember(void *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_25f28)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x4e70
void exit(int a0)
{
  (*dat_25f30)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x4e80
int __fprintf_chk(FILE *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_25f38)(); // jump-as-call
  return v1;
}

// Function: re_compile_pattern @ 0x4e90
char * re_compile_pattern(char *a0,unsigned long a1,re_pattern_buffer *a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25f40)(); // jump-as-call
  return v1;
}

// Function: fflush_unlocked @ 0x4ea0
int fflush_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_25f48)(); // jump-as-call
  return v1;
}

// Function: strdup @ 0x4eb0
char * strdup(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25f50)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x4ec0
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_25f58)(); // jump-as-call
  return v1;
}

// Function: sigaltstack @ 0x4ed0
int sigaltstack(void *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_25f60)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x4ee0
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_25f68)(); // jump-as-call
  return v1;
}

// Function: fstat @ 0x4ef0
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_25f70)(); // jump-as-call
  return v1;
}

// Function: getc @ 0x4f00
int getc(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_25f78)(); // jump-as-call
  return v1;
}

// Function: sigaddset @ 0x4f10
int sigaddset(sigset_t *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_25f80)(); // jump-as-call
  return v1;
}

// Function: execv @ 0x4f20
int execv(char *a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_25f88)(); // jump-as-call
  return v1;
}

// Function: fork @ 0x4f30
int fork(void)
{
  int v1; // eax
  
  v1 = (*dat_25f90)(); // jump-as-call
  return v1;
}

// Function: __ctype_tolower_loc @ 0x4f40
void * __ctype_tolower_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25f98)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x4f50
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25fa0)(); // jump-as-call
  return v1;
}

// Function: iconv_open @ 0x4f60
void iconv_open(void)
{
  (*dat_25fa8)(); // jump-as-call
}

// Function: __sprintf_chk @ 0x4f70
int __sprintf_chk(char *a0,int a1,unsigned long a2,char *a3,...)
{
  int v1; // eax
  
  v1 = (*dat_25fb0)(); // jump-as-call
  return v1;
}

// Function: sub_4f80 @ 0x4f80
undefined16 sub_4f80(void)
{
  char v1 [16];
  unsigned long v2; // rax
  
  *__errno_location() = 0xc;
  v1._8_8_ = 0;
  v1._0_8_ = v2;
  return v1._0_16_ << 0x40;
}

// Function: main @ 0x4fa0
int main(int argc,char **argv,char **envp)
{
  bool v1;
  long v10;
  unsigned long v11; // rcx
  long v12;
  char *v13; // stack - 0x48
  long v14; // stack - 0x58
  long v15; // stack - 0x50
  int v16; // ebp
  long v17;
  char *v18;
  char v19; // dil
  bool v2;
  unsigned long v20;
  unsigned long v21; // stack - 0x88
  long v22; // stack - 0x80
  int v3; // eax
  int v4;
  unsigned long v5;
  unsigned long v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v17 = (long)argc;
  v14 = 0;
  v15 = 0;
  dat_260b0 = 2;
  sub_13d30(*argv);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_fa70(NULL);
  dat_26600 = 0x26500;
  dat_265c0 = 0x264c0;
  re_set_syntax(0x50a46);
  dat_26410 = sub_10050();
  dat_26180 = 0;
  sub_16390();
  v2 = 0;
  v21 = 0;
  v1 = 0;
  v22 = -1;
  v4 = -1;
label_50a0:
  v16 = (int)v17;
  v3 = getopt_long(v16,argv,"0123456789abBcC:dD:eEfF:hHiI:lL:nNpPqrsS:tTuU:vwW:x:X:yZ",(option *)0x25160,NULL);
  if (v3 != -1) {
    switch(v3) {
      case 0:
      case 0x68:
      case 0x80:
      case 0x85:
        break;
      default:
        sub_8c00(0,0); // no-return
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
        if (10 <= (unsigned int)(v4 - 0x30U)) { // branch-flip
          v22 = (long)(v3 + -0x30);
          v4 = v3;
        }
        else if (0x666666666666666 <= (long)(v22 - (unsigned long)(v3 <= 0x33))) { // branch-flip
          v22 = 0x3fffffffffffffff;
          v4 = v3;
        }
        else {
          v22 = (long)(v3 + -0x30) + v22 * 10;
          v4 = v3;
        }
        goto label_50a0;
      case 0x42:
        dat_26554 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x43:
      case 0x55:
        if (optarg) { // branch-flip
          v12 = strtoimax(optarg,&v13,10);
          if ((*v13) || (v12 < 0))
            sub_8c00("invalid context length \'%s\'",optarg); // no-return
          v10 = 0x3fffffffffffffff;
          if (v12 <= 0x3fffffffffffffff)
            v10 = v12;
        }
        else {
          v10 = 3;
        }
        v19 = (v3 == 0x55) + '\x02';
        sub_8d50(v19);
        if (v10 <= dat_26570) { // branch-flip
          v1 = 1;
          v4 = v3;
        }
        else {
          v1 = 1;
          dat_26570 = v10;
          v4 = v3;
        }
        goto label_50a0;
      case 0x44:
        sub_8d50(7);
        v18 = (char *)sub_15f50(strlen(optarg) * 7 + 0x61);
        __sprintf_chk(v18,1,0xffffffffffffffff,"%%=%c#ifndef %s\n%%<#endif /* ! %s */\n%c#ifdef %s\n%%>#endif /* %s */\n%c#ifndef %s\n%%<#else /* %s */\n%%>#endif /* %s */\n",0,optarg,optarg,0,optarg,optarg,0,optarg,optarg,optarg);
        v17 = 0x26460;
        do {
          v10 = v17 + 8;
          sub_8c60(v17,v18,"-D");
          v18 = &v18[strlen(v18) + 1];
          v17 = v10;
        } while (v10 != 0x26480);
        v17 = (long)v16;
        v4 = v3;
        goto label_50a0;
      case 0x45:
        v4 = v3;
        if (dat_26558 <= 3)
          dat_26558 |= 1;
        goto label_50a0;
      case 0x46:
        sub_8930(0x265e0,optarg);
        v4 = v3;
        goto label_50a0;
      case 0x48:
        dat_26418 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x49:
        sub_8930(0x265a0,optarg);
        v4 = v3;
        goto label_50a0;
      case 0x4c:
        if (dat_26540) { // branch-flip
          if (dat_26548)
            sub_e230("too many file label options"); // no-return
          dat_26548 = optarg;
          v4 = v3;
        }
        else {
          dat_26540 = optarg;
          v4 = v3;
        }
        goto label_50a0;
      case 0x4e:
        dat_26586 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x50:
        dat_26585 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x53:
        sub_8c60(0x26488,optarg,"-S");
        v4 = v3;
        goto label_50a0;
      case 0x54:
        dat_26492 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x57:
        v5 = strtoimax(optarg,&v13,10);
        if (((long)v5 <= 0) || (*v13))
          sub_8c00("invalid width \'%s\'",optarg); // no-return
        if (v5 != v21) {
          if (v21)
            sub_e230("conflicting width options"); // no-return
          v21 = v5;
          v4 = v3;
          goto label_50a0;
        }
        break;
      case 0x58:
        v4 = sub_10770(sub_102e0,dat_26410,optarg,sub_8910());
        if (v4)
          sub_e1f0(optarg); // no-return
        break;
      case 0x5a:
        v4 = v3;
        if (dat_26558 <= 3)
          dat_26558 |= 2;
        goto label_50a0;
      case 0x61:
        dat_26568 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x62:
        v4 = v3;
        if (dat_26558 <= 3)
          dat_26558 = 4;
        goto label_50a0;
      case 99:
        v6 = 2;
        goto label_5820;
      case 100:
        dat_26408 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x65:
        sub_8d50(4);
        v4 = v3;
        goto label_50a0;
      case 0x66:
        sub_8d50(5);
        v4 = v3;
        goto label_50a0;
      case 0x69:
        dat_26552 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x6c:
        if (!s_1d558[0])
          sub_8c00("pagination not supported on this host",0); // no-return
        dat_26480 = 1;
        signal(0x11,0);
        v4 = v3;
        goto label_50a0;
      case 0x6e:
        sub_8d50(6);
        v4 = v3;
        goto label_50a0;
      case 0x70:
        sub_8930(0x265e0,"^[[:alpha:]$_]");
        v2 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x71:
        dat_264a1 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x72:
        dat_26608 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x73:
        dat_26584 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x74:
        dat_264a0 = '\x01';
        v4 = v3;
        goto label_50a0;
      case 0x75:
        v6 = 3;
label_5820:
        sub_8d50(v6);
        v4 = v3;
        if (dat_26570 <= 2)
          dat_26570 = 3;
        goto label_50a0;
      case 0x76:
        v6 = sub_14040("Len Tower");
        v7 = sub_14040("Richard Stallman");
        v8 = sub_14040("David Hayes");
        v9 = sub_14040("Mike Haertel");
        sub_15de0(stdout,"diff","GNU diffutils",dat_260a8,sub_14040("Paul Eggert"),v9,v8,v7,v6,0);
        sub_8e20();
        return 0;
      case 0x77:
        dat_26558 = 5;
        v4 = v3;
        goto label_50a0;
      case 0x78:
        sub_102e0(dat_26410,optarg,sub_8910());
        v4 = v3;
        goto label_50a0;
      case 0x79:
        sub_8d50(8);
        v4 = v3;
        goto label_50a0;
      case 0x81:
        sub_8c60(&v14,optarg,"--from-file");
        v4 = v3;
        goto label_50a0;
      case 0x82:
        sub_8a10();
        sub_8e20();
        return 0;
      case 0x83:
        v10 = strtoimax(optarg,&v13,10);
        if ((*v13) || (v10 < 0))
          sub_8c00("invalid horizon length \'%s\'",optarg); // no-return
        if (dat_26560 < v10)
          dat_26560 = v10;
        v4 = v3;
        goto label_50a0;
      case 0x84:
        dat_26551 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x86:
        dat_26439 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x87:
        sub_8d50(7);
        v10 = 0x26440;
        do {
          v12 = v10 + 8;
          sub_8c60(v10,optarg,"--line-format");
          v10 = v12;
        } while (v12 != 0x26458);
        v4 = v3;
        goto label_50a0;
      case 0x88:
        dat_26550 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x89:
        dat_26551 = 0;
        v4 = v3;
        goto label_50a0;
      case 0x8a:
        sub_8d50(1);
        v4 = v3;
        goto label_50a0;
      case 0x8b:
        sub_8d50(8);
        dat_2643a = 1;
        v4 = v3;
        goto label_50a0;
      case 0x8c:
        dat_26490 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x8d:
        dat_26491 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x8e:
        dat_26438 = 1;
        v4 = v3;
        goto label_50a0;
      case 0x8f:
        v5 = strtoimax(optarg,&v13,10);
        if ((((long)v5 <= 0) || (0xfffffffffffffffd <= v5)) || (*v13))
          sub_8c00("invalid tabsize \'%s\'",optarg); // no-return
        if (v5 != dat_26498) {
          if (dat_26498)
            sub_e230("conflicting tabsize options"); // no-return
          dat_26498 = v5;
          v4 = v3;
          goto label_50a0;
        }
        break;
      case 0x90:
        sub_8c60(&v15,optarg,"--to-file");
        v4 = v3;
        goto label_50a0;
      case 0x91:
      case 0x92:
      case 0x93:
        sub_8d50(7);
        v4 = v3 + -0x91;
        sub_8c60((long)v4 * 8 + 0x26440,optarg,&"--unchanged-line-format"[(long)v4 * 0x18]);
        goto label_50a0;
      case 0x94:
      case 0x95:
      case 0x96:
      case 0x97:
        sub_8d50(7);
        v4 = v3 + -0x94;
        sub_8c60((long)v4 * 8 + 0x26460,optarg,&"--unchanged-group-format"[(long)v4 * 0x19]);
        goto label_50a0;
      case 0x98:
        sub_8cc0(optarg);
        v4 = v3;
        goto label_50a0;
      case 0x99:
        sub_e270(optarg);
        v4 = v3;
        goto label_50a0;
      case 0x9a:
        goto label_564a;
      
    }
    v4 = v3;
    goto label_50a0;
  }
  if (((dat_2657c == 1) && (v18 = getenv("TERM"), v18)) && (!strcmp(v18,"dumb")))
    dat_2657c = 0;
  if (dat_26580) { // branch-flip
label_5b6d:
    if ((dat_26580 != 2) || (sub_10ab0(2)))
      dat_26400 = "%Y-%m-%d %H:%M:%S.%N %z";
    else {
      dat_26400 = "%a %b %e %T %Y";
    }
    if (0 <= v22) { // branch-flip
label_59d6:
      v10 = dat_26570;
      if (((unsigned int)(dat_26580 - 2U) < 2) && ((dat_26570 < v22 || ((v22 != dat_26570 && (!v1)))))) {
        dat_26570 = v22;
        v10 = v22;
      }
    }
    else {
label_5b94:
      v10 = dat_26570;
    }
  }
  else {
    if (!v2) {
      sub_8d50(1);
      goto label_5b6d;
    }
    sub_8d50(2);
    if (0 <= v22) {
      if ((dat_26580 != 2) || (sub_10ab0(2)))
        dat_26400 = "%Y-%m-%d %H:%M:%S.%N %z";
      else {
        dat_26400 = "%a %b %e %T %Y";
      }
      goto label_59d6;
    }
    dat_26570 = 3;
    if (dat_26580 == 2) {
      if (sub_10ab0(2)) {
        dat_26400 = "%Y-%m-%d %H:%M:%S.%N %z";
        v10 = dat_26570;
        goto label_59ef;
      }
      dat_26400 = "%a %b %e %T %Y";
      goto label_5b94;
    }
    dat_26400 = "%Y-%m-%d %H:%M:%S.%N %z";
    v10 = 3;
  }
label_59ef:
  if (!dat_26498)
    dat_26498 = 8;
  if (v21) { // branch-flip
    v5 = 1;
    if (!dat_264a0) {
label_5a1f:
      v5 = dat_26498;
    }
    v11 = (v21 >> 1) + (v5 + 3 >> 1) + (unsigned long)((unsigned int)(v5 + 3) & (unsigned int)v21 & 1);
    v20 = v11 - v11 % v5;
    if ((4 <= v20) && (v20 < v21)) {
      dat_26430 = (v11 % v5 - v11) + v21;
      if (v20 - 3 < dat_26430)
        dat_26430 = v20 - 3;
      if (!dat_26430)
        v20 = v21;
      v21 = v20;
    }
    else {
      dat_26430 = 0;
    }
  }
  else {
    if (!dat_264a0) {
      v21 = 0x82;
      goto label_5a1f;
    }
    v21 = 0x43;
    dat_26430 = 0x3f;
  }
  dat_26428 = v21;
  if (dat_26560 < v10)
    dat_26560 = v10;
  sub_8ba0(0x265e0);
  sub_8ba0(0x265a0);
  if (dat_26580 != 7) {
label_5acd:
    dat_26578 = dat_26580 != 8 | dat_26438;
    goto label_5ad9;
  }
  if (!dat_26440)
    dat_26440 = "%l\n";
  if (!dat_26448)
    dat_26448 = "%l\n";
  if (!dat_26450)
    dat_26450 = "%l\n";
  if ((!dat_26468) && (dat_26468 = "%<", dat_26478))
    dat_26468 = dat_26478;
  if ((!dat_26470) && (dat_26470 = "%>", dat_26478))
    dat_26470 = dat_26478;
  if (dat_26460) { // branch-flip
    if (!dat_26478) {
label_5e60:
      dat_26478 = (char *)sub_f560(dat_26468,dat_26470,"");
      if (dat_26580 != 7) goto label_5acd;
    }
    v18 = dat_26460;
    v2 = 1;
    if (*dat_26460) goto label_5dae;
  }
  else {
    v18 = "%=";
    dat_26460 = "%=";
    if (!dat_26478) goto label_5e60;
label_5dae:
    v2 = 0;
    if (!strcmp(v18,"%="))
      v2 = *dat_26440 == '\0';
  }
  dat_26578 = v2;
label_5ad9:
  dat_26553 = ~(dat_26554 | dat_26552 | dat_26490 | (dat_265a0 || dat_26558)) & dat_264a1 & 1;
  dat_26420 = sub_8d80(&argv[1],optind + -1);
  v12 = v15;
  v10 = v14;
  if (v14) { // branch-flip
    if (v15)
      sub_e230("--from-file and --to-file both specified"); // no-return
    v4 = 0;
    for (; optind < v16; optind = optind + 1) {
      v3 = sub_8e70(0,v10,argv[optind]);
      if (v4 < v3)
        v4 = v3;
    }
  }
  else {
    v10 = (long)optind;
    if (v15) { // branch-flip
      v4 = 0;
      while ((int)v10 < v16) {
        v3 = sub_8e70(0,argv[v10],v12);
        if (v4 < v3)
          v4 = v3;
        optind += 1;
        v10 = (long)optind;
      }
    }
    else {
      if (v16 - optind != 2) {
        if (v16 - optind > 1)
          sub_8c00("extra operand \'%s\'",argv[v10 + 2]); // no-return
        sub_8c00("missing operand after \'%s\'",argv[v17 + -1]); // no-return
      }
      v4 = sub_8e70(0,argv[v10],argv[v10 + 1]);
    }
  }
  sub_e0f0();
  sub_8e20();
  exit(v4); // no-return
label_564a:
  dat_26180 = 1;
  v4 = v3;
  goto label_50a0;
}

// Function: sub_6040 @ 0x6040
void sub_6040(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_25fb8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_6070 @ 0x6070
void sub_6070(void)
{
  return;
}

// Function: _FINI_0 @ 0x60e0
void _FINI_0(void)
{
  if (!dat_26168) {
    if (dat_25fe0)
      __cxa_finalize(dat_26008);
    sub_6070();
    dat_26168 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x6120
void _INIT_0(void)
{
  return;
}

// Function: sub_6130 @ 0x6130
void sub_6130(long a0,long a1,long a2,long a3,bool a4,struct_4 *a5,long *a6) // return-dupe x4
{
  long *v1;
  long v10;
  long v11;
  long v12;
  long v13;
  long v14;
  long v15; // r8
  long v16;
  long v17; // r10
  long v18;
  long v19;
  long *v2;
  long v20; // r11
  long v21; // r12
  long v22; // r13
  long v23;
  long v24; // stack - 0xd8
  long v25; // stack - 0xc0
  long v26; // stack - 0x50
  long v27; // stack - 0x48
  long v3;
  bool v4;
  unsigned char v5;
  bool v6;
  long v7;
  long v8;
  long v9; // rcx
  
  v22 = a1 - a3;
  v20 = a0 - a3;
  v8 = a1 - a2;
  v11 = a6[2];
  v9 = a0 - a2;
  v5 = (char)v9 - (char)v22;
  v3 = a6[3];
  v16 = *a6;
  v7 = a6[1];
  *(long *)(v11 + v9 * 8) = a0;
  v17 = 2 - v22;
  *(long *)(v3 + v22 * 8) = a1;
  v25 = 1;
  v10 = v9;
  v23 = v22;
  v24 = v9;
  if (v9 <= v20) goto label_6485;
label_6216:
  v10 -= 1;
  *(unsigned long *)(v11 + -8 + v10 * 8) = 0xffffffffffffffff;
  if (v8 <= v24) goto label_649c;
label_6236:
  *(unsigned long *)(v11 + 0x10 + v24 * 8) = 0xffffffffffffffff;
  v24 += 1;
  do {
    v6 = 0;
    for (v14 = v24; v10 <= v14; v14 = v14 + -2) {
      v18 = *(long *)(v11 + -8 + v14 * 8);
      v12 = *(long *)(v11 + 8 + v14 * 8);
      if (v12 <= v18)
        v12 = v18 + 1;
      v13 = v12 - v14;
      v4 = v13 < a3;
      v18 = v12;
      if (v12 < a1 && v4) {
        while (*(long *)(v16 + v18 * 8) == *(long *)(v7 + v13 * 8)) {
          v18 += 1;
          v13 += 1;
          if ((a1 <= v18) || (a3 <= v13)) break;
        }
        if (0x15 <= v18 - v12)
          v6 = v12 < a1 && v4;
      }
      *(long *)(v11 + v14 * 8) = v18;
      if (((v22 <= v14 && v14 <= v23) && (v5 & 1)) && (*(long *)(v3 + v14 * 8) <= v18)) goto label_62e6;
    }
    if (v22 <= v20) // branch-flip
      v22 += 1;
    else {
      v22 -= 1;
      *(unsigned long *)(v3 + -8 + v22 * 8) = 0x7fffffffffffffff;
    }
    if (v8 <= v23) // branch-flip
      v23 -= 1;
    else {
      *(unsigned long *)(v3 + 0x10 + v23 * 8) = 0x7fffffffffffffff;
      v23 += 1;
    }
    if (v22 <= v23) {
      v14 = v23;
label_63a5:
      do {
        v18 = *(long *)(v3 + 8 + v14 * 8);
        v12 = *(long *)(v3 + -8 + v14 * 8);
        if (v18 <= v12)
          v12 = v18 + -1;
        v13 = v12 - v14;
        v4 = a2 < v13;
        v18 = v12;
        if (v4 && a0 < v12) {
          while (*(long *)(v16 + -8 + v18 * 8) == *(long *)(v7 + -8 + v13 * 8)) {
            v18 -= 1;
            v13 -= 1;
            if ((v18 <= a0) || (v13 <= a2)) break;
          }
          if (0x15 <= v12 - v18)
            v6 = v4 && a0 < v12;
        }
        *(long *)(v3 + v14 * 8) = v18;
        if ((!(v10 <= v14 && v14 <= v24)) || (!(~v5 & 1))) {
          v14 -= 2;
          if (v22 > v14) break;
          goto label_63a5;
        }
        if (v18 <= *(long *)(v11 + v14 * 8)) {
label_62e6:
          a5->field_0x0 = v18;
          a5->field_0x8 = v13;
          a5->field_0x10 = 0x101;
          return;
        }
        v14 -= 2;
      } while (v14 >= v22);
    }
    if (!a4) {
      if ((200 < v25 & *(unsigned char *)&a6[4]) && (v6)) {
        if (v10 <= v24) {
          v18 = 0;
          v14 = v24;
          do {
            while( true ) {
              v21 = v14 + -2;
              v13 = *(long *)(v11 + 0x10 + v21 * 8);
              v12 = (2U - v9) + v21;
              v15 = (v13 - a0) * 2 - v12;
              v19 = -v12;
              if (-v12 <= -1)
                v19 = v12;
              v12 = (v19 + v25) * 0xc;
              if (v12 < v18)
                v12 = v18;
              if ((((v15 <= v12) || (v13 <= a0 + 0x13)) || (a1 <= v13)) || (((v14 = v13 - v14, v14 <= a2 + 0x13 || (a3 <= v14)) || (*(long *)(v7 + -8 + v14 * 8) != *(long *)(v16 + (v13 + -1) * 8))))) break;
              v12 = 0;
              do {
                if (*(long *)(v16 + (v13 + -1) * 8 + -8 + v12) != *(long *)(v7 + v14 * 8 + -0x10 + v12)) goto label_6538;
                v12 -= 8;
              } while (v12 != -0x98);
              a5->field_0x0 = v13;
              a5->field_0x8 = v14;
              if (v21 < v10) goto label_62fd;
              v18 = v15;
              v14 = v21;
            }
label_6538:
            v14 = v21;
          } while (v10 <= v21);
          if (v18) {
label_62fd:
            a5->field_0x10 = 1;
            return;
          }
        }
        if (v22 <= v23) {
          v18 = 0;
          v14 = v23;
label_665c:
          do {
            v15 = v14 + -2;
            v12 = v17 + v15;
            v13 = *(long *)(v3 + 0x10 + v15 * 8);
            v19 = -v12;
            if (-v12 <= -1)
              v19 = v12;
            v12 += (a1 - v13) * 2;
            if ((((v19 + v25) * 0xc < v12) && (v18 < v12)) && ((a0 < v13 && (((v14 = v13 - v14, v13 < a1 + -0x13 && (a2 < v14)) && (v14 < a3 + -0x13)))))) {
              v1 = (long *)(v16 + v13 * 8);
              v2 = (long *)(v7 + v14 * 8);
              if (*v2 == *v1) {
                v19 = 1;
                do {
                  if (v1[v19] != v2[v19]) goto label_66b0;
                  v19 += 1;
                } while (v19 != 0x14);
                a5->field_0x0 = v13;
                a5->field_0x8 = v14;
                if (v15 < v22) goto label_62fd;
                v18 = v12;
                v14 = v15;
                goto label_665c;
              }
            }
label_66b0:
            v14 = v15;
          } while (v22 <= v15);
          if (v18) {
label_62fd:
            a5->field_0x10 = 0x100;
            return;
          }
        }
      }
      if (a6[5] <= v25) {
        v16 = -1;
        if (v10 <= v24) {
          do {
            v7 = *(long *)(v11 + v24 * 8);
            if (a1 <= v7)
              v7 = a1;
            v8 = v7 - v24;
            if (a3 < v7 - v24) {
              v7 = a3 + v24;
              v8 = a3;
            }
            if (v16 < v8 + v7) {
              v16 = v8 + v7;
              v26 = v7;
            }
            v24 -= 2;
          } while (v10 <= v24);
        }
        v11 = 0x7fffffffffffffff;
        if (v22 <= v23) {
          do {
            v7 = *(long *)(v3 + v23 * 8);
            if (v7 <= a0)
              v7 = a0;
            v10 = v7 - v23;
            if (v7 - v23 < a2) {
              v7 = a2 + v23;
              v10 = a2;
            }
            if (v10 + v7 < v11) {
              v11 = v10 + v7;
              v27 = v7;
            }
            v23 -= 2;
          } while (v22 <= v23);
        }
        if (v16 - (a0 + a2) > (a3 + a1) - v11) {
          a5->field_0x10 = 1;
          a5->field_0x0 = v26;
          a5->field_0x8 = v16 - v26;
          return;
        }
        a5->field_0x10 = 0x100;
        a5->field_0x0 = v27;
        a5->field_0x8 = v11 - v27;
        return;
      }
    }
    v25 += 1;
    if (v20 < v10) goto label_6216;
label_6485:
    v10 += 1;
    if (v24 < v8) goto label_6236;
label_649c:
    v24 -= 1;
  } while( true );
}

// Function: sub_6890 @ 0x6890
unsigned long sub_6890(long a0,long a1,long a2,long a3,char a4,struct_5 *a5) // return-dupe
{
  char v1;
  long v10;
  long v11; // stack - 0x70
  long v12; // stack - 0x50
  char v13; // stack - 0x48
  char v14; // stack - 0x47
  long v2;
  long v3;
  unsigned long v4; // rax
  long v5;
  long v6;
  long v7; // stack - 0x58
  long v8;
  long v9;
  
  v2 = a5->field_0x0;
  v3 = a5->field_0x8;
  v11 = a1;
  do {
    if (v11 <= a0) goto label_69d0;
    while( true ) {
      if (a3 <= a2) goto label_69d0;
      if (*(long *)(v2 + a0 * 8) == *(long *)(v3 + a2 * 8)) break;
      while (v8 = a0, v9 = v11, *(long *)(v2 + -8 + v11 * 8) == *(long *)(v3 + -8 + a3 * 8)) {
        v11 -= 1;
        a3 -= 1;
        if (v11 <= a0) goto label_69d0;
        if (a3 <= a2) goto label_69d0;
      }
      while( true ) {
        if (a2 == a3) {
          if (v9 <= v8)
            return 0;
          do {
            v2 = v8 * 8;
            v8 += 1;
            *(char *)(dat_262b8 + *(long *)(dat_262a8 + v2)) = 1;
          } while (v8 != v9);
          return 0;
        }
        sub_6130(v8,v9,a2,a3,(unsigned char)a4,&v7,a5);
        if ((v7 + v12) - (v8 + a2) <= (v9 + a3) - (v7 + v12)) { // branch-flip
          v5 = v12;
          v6 = a2;
          a2 = v12;
          a0 = v7;
          v10 = v7;
          a4 = v14;
          v1 = v13;
          v11 = v9;
        }
        else {
          v11 = v7;
          v5 = a3;
          v6 = v12;
          a0 = v8;
          v10 = v9;
          v8 = v7;
          a3 = v12;
          a4 = v13;
          v1 = v14;
        }
        v4 = sub_6890(v8,v10,v6,v5,(unsigned char)v1,a5);
        if ((char)v4)
          return v4;
        if (a0 < v11) break;
label_69d0:
        v8 = a0;
        v9 = v11;
        if (a0 == v11) {
          if (a3 <= a2)
            return 0;
          do {
            v2 = a2 * 8;
            a2 += 1;
            *(char *)(dat_263e8 + *(long *)(dat_263d8 + v2)) = 1;
          } while (a2 != a3);
          return 0;
        }
      }
    }
    a0 += 1;
    a2 += 1;
  } while( true );
}

// Function: sub_6a70 @ 0x6a70
void sub_6a70(long a0)
{
  char v1;
  long v10; // rbx
  long v11; // rbp
  long v12;
  char *v13;
  long v2;
  long v3;
  long v4;
  long v5;
  long v6;
  long v7;
  long v8;
  long v9;
  
  v11 = 0;
label_6a7d:
  v2 = *(long *)(a0 + 0x118 + v11);
  v8 = 0;
  v6 = 0;
  v3 = *(long *)(a0 + 0xf8 + v11);
  v4 = *(long *)(a0 + 200 + v11);
  v5 = *(long *)((a0 + 0x248) - v11);
label_6aac:
  do {
    v12 = v6;
    if (v4 <= v6) goto label_6bea;
    while (v9 = v12, *(char *)(v2 + v12)) {
      while( true ) {
        v6 = v9;
        do {
          v6 += 1;
        } while (*(char *)(v2 + v6));
        for (; *(char *)(v5 + v8); v8 = v8 + 1) {
        }
        do {
          v10 = v6 - v9;
          while ((v9 && (*(long *)(v3 + -8 + v9 * 8) == *(long *)(v3 + -8 + v6 * 8)))) {
            v6 -= 1;
            *(char *)(v2 + v9 + -1) = 1;
            *(char *)(v2 + v6) = 0;
            v1 = *(char *)(v2 + -2 + v9);
            v9 -= 1;
            while (v1) {
              v9 -= 1;
              v1 = *(char *)(v2 + -1 + v9);
            }
            do {
              v8 -= 1;
            } while (*(char *)(v5 + v8));
          }
          v12 = v6;
          if (!*(char *)(v5 + -1 + v8))
            v12 = v4;
          while ((v4 != v6 && (*(long *)(v3 + v9 * 8) == *(long *)(v3 + v6 * 8)))) {
            v9 += 1;
            v7 = v6 + 1;
            *(char *)(v2 + -1 + v9) = 0;
            *(char *)(v2 + v6) = 1;
            if (!*(char *)(v2 + v7)) goto label_6b93;
            do {
              v7 += 1;
            } while (*(char *)(v2 + v7));
            v8 += 1;
            v1 = *(char *)(v5 + v8);
            while (v6 = v7, v1) {
              v12 = v7;
label_6b93:
              v8 += 1;
              v1 = *(char *)(v5 + v8);
            }
          }
        } while (v6 - v9 != v10);
        if (v6 <= v12) goto label_6aac;
        v13 = (char *)(v2 + -1 + v6);
        do {
          v13[v9 - v6] = 1;
          *v13 = 0;
          do {
            v8 -= 1;
          } while (*(char *)(v5 + v8));
          v13 = &v13[-1];
        } while ((char *)(v2 + -1 + v12) != v13);
        if (v12 < v4) break;
label_6bea:
        v9 = v12;
        if (v12 == v4) {
          v11 += 0x130;
          if (v11 == 0x260)
            return;
          goto label_6a7d;
        }
      }
    }
    do {
      v8 += 1;
    } while (*(char *)(v5 + -1 + v8));
    v6 = v12 + 1;
  } while( true );
}

// Function: sub_6c30 @ 0x6c30
void sub_6c30(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_15f50(0x30);
  v1[3] = a0;
  v1[4] = a1;
  v1[1] = a3;
  v1[2] = a2;
  *v1 = a4;
}

// Function: sub_6c70 @ 0x6c70
unsigned long sub_6c70(void *a0)
{
  char v1;
  long v10;
  long v11; // rsi
  long v12; // rdi
  long v13; // r9
  char v2;
  long v3;
  long v4;
  long v5;
  long v6;
  unsigned long v7; // rax
  long v8; // rcx
  long v9;
  
  v3 = *(long *)((long)a0 + 200);
  v4 = *(long *)((long)a0 + 0x118);
  v5 = *(long *)((long)a0 + 0x248);
  v6 = *(long *)((long)a0 + 0x1f8);
  if ((1 <= v3) || (1 <= v6)) {
    v11 = 0;
    v12 = 0;
    v7 = 0;
    do {
      v1 = *(char *)(v4 + v12);
      v2 = *(char *)(v5 + v11);
      v10 = v11;
      v9 = v12;
      if (v1 || v2) {
        if (v1) { // branch-flip
          do {
            v9 += 1;
          } while (*(char *)(v4 + v9));
          v13 = v9 - v12;
        }
        else {
          v13 = 0;
        }
        if (v2) { // branch-flip
          do {
            v10 += 1;
          } while (*(char *)(v5 + v10));
          v8 = v10 - v11;
        }
        else {
          v8 = 0;
        }
        v7 = sub_6c30(v12,v11,v13,v8);
      }
      v12 = v9 + 1;
      v11 = v10 + 1;
    } while ((v12 < v3) || (v11 < v6));
  }
  else {
    v7 = 0;
  }
  return v7;
}

// Function: sub_6d50 @ 0x6d50
unsigned long sub_6d50(void *a0)
{
  char v1;
  unsigned long v10;
  char v2;
  long v3;
  long v4;
  unsigned long v5;
  long v6; // rcx
  unsigned long v7;
  unsigned long v8;
  long v9; // rbx
  
  v5 = 0;
  v8 = *(unsigned long *)((long)a0 + 200);
  v10 = *(unsigned long *)((long)a0 + 0x1f8);
  v3 = *(long *)((long)a0 + 0x118);
  v4 = *(long *)((long)a0 + 0x248);
  v7 = v8 & v10;
  do {
    if ((long)v7 <= -1)
      return v5;
    do {
      v1 = *(char *)(v3 + -1 + v8);
      v2 = *(char *)(v4 + -1 + v10);
      if (v1 || v2) {
        if (v1) { // branch-flip
          v7 = v8;
          do {
            v7 -= 1;
          } while (*(char *)(v3 + -1 + v7));
          v9 = v8 - v7;
          v8 = v7;
          if (v2) goto label_6dd9;
label_6e20:
          v6 = 0;
        }
        else {
          v9 = 0;
          if (!v2) goto label_6e20;
label_6dd9:
          v7 = v10;
          do {
            v7 -= 1;
          } while (*(char *)(v4 + -1 + v7));
          v6 = v10 - v7;
          v10 = v7;
        }
        v5 = sub_6c30(v8,v10,v9,v6);
      }
      v10 -= 1;
      v8 -= 1;
      v7 = v10;
    } while (0 <= (long)v8);
  } while( true );
}

// Function: sub_6e30 @ 0x6e30
void sub_6e30(void *a0)
{
  char v1;
  long *v10;
  long v11;
  long v12; // rcx
  unsigned long v13;
  void *v14;
  long v15;
  void *v16 [3]; // stack - 0x58
  unsigned long *v17;
  void *v18;
  long v19; // rdi
  unsigned long v2;
  long *v20;
  void **v21;
  char *v22;
  long v23;
  long v24;
  long v25;
  long v26;
  unsigned long *v27;
  long v28; // stack - 0x60
  long v3;
  void *v4; // rax
  long v5;
  void *v6; // rax
  unsigned long v7; // rax
  char *v8;
  unsigned long v9;
  
  v3 = sub_15f50((*(long *)((long)a0 + 0x1f8) + *(long *)((long)a0 + 200)) * 0x10);
  *(long *)((long)a0 + 0x100) = v3;
  v26 = *(long *)((long)a0 + 200) * 8;
  v3 += v26;
  *(long *)((long)a0 + 0x108) = v3;
  v3 += v26;
  *(long *)((long)a0 + 0x230) = v3;
  *(long *)((long)a0 + 0x238) = v3 + *(long *)((long)a0 + 0x1f8) * 8;
  v4 = (void *)sub_f5e0(*(long *)((long)a0 + 0x128) << 4);
  v16[0] = v4;
  v14 = (void *)((long)v4 + *(long *)((long)a0 + 0x128) * 8);
  if (1 <= *(long *)((long)a0 + 200)) {
    v26 = *(long *)((long)a0 + 0xf8);
    v3 = 0;
    do {
      v5 = v3 * 8;
      v3 += 1;
      v10 = (long *)((long)v4 + *(long *)(v26 + v5) * 8);
      *v10 = *v10 + 1;
    } while (v3 < *(long *)((long)a0 + 200));
  }
  v26 = *(long *)((long)a0 + 0x1f8);
  if (1 <= v26) {
    v3 = *(long *)((long)a0 + 0x228);
    v5 = 0;
    do {
      v26 = v5 * 8;
      v5 += 1;
      v10 = (long *)((long)v14 + *(long *)(v3 + v26) * 8);
      *v10 = *v10 + 1;
      v26 = *(long *)((long)a0 + 0x1f8);
    } while (v5 < v26);
  }
  v27 = &v15;
  v16[1] = v14;
  v6 = (void *)sub_f5e0(v26 + *(long *)((long)a0 + 200));
  v9 = *(unsigned long *)((long)a0 + 200);
  v10 = (long *)((long)a0 + 0xf8);
  v21 = v16;
  v26 = *v10;
  v28 = (long)v6 + v9;
  v7 = v9 >> 8;
  v18 = v6;
  v17 = v27;
  if (!v7) goto label_737e;
label_6f76:
  v13 = 5;
  do {
    v13 *= 2;
    v7 >>= 2;
    v20 = v10;
  } while (v7);
  do {
    v7 = 0;
    do {
      while (v3 = *(long *)(v26 + v7 * 8), !v3) {
label_6f94:
        v7 += 1;
        if (v7 == v9) goto label_6fc8;
      }
      v2 = *(unsigned long *)((long)v14 + v3 * 8);
      if (!v2) {
        *(char *)((long)v18 + v7) = 1;
        goto label_6f94;
      }
      if (v2 <= v13) goto label_6f94;
      *(char *)((long)v18 + v7) = 2;
      v7 += 1;
    } while (v7 != v9);
label_6fc8:
    do {
      v10 = &v20[0x26];
      v17 = &v17[1];
      if ((long *)((long)a0 + 0x358) == v10) {
        v26 = 0;
        v14 = v6;
        v17 = v27;
        goto label_6ff4;
      }
      v9 = v20[0x20];
      v14 = *v21;
      v21 = &v21[-1];
      v18 = (void *)*v17;
      v26 = *v10;
      v7 = v9 >> 8;
      if (v7) goto label_6f76;
label_737e:
      v20 = v10;
    } while (!v9);
    v13 = 5;
  } while( true );
label_6ff4:
  v3 = *(long *)((long)a0 + v26 + 200);
  v5 = 0;
  if (0 < v3) {
    do {
      while( true ) {
        v8 = (char *)((long)v14 + v5);
        if (*v8 != '\x02') break;
        v5 += 1;
        *v8 = '\0';
        if (v3 <= v5) goto label_703b;
      }
      if (*v8) {
        v23 = 0;
        v11 = v5;
        while ((v11 = v11 + 1, v11 < v3 && (*(char *)((long)v14 + v11)))) {
          v23 += (unsigned long)(*(char *)((long)v14 + v11) == '\x02');
        }
        if (v11 <= v5) { // branch-flip
          v19 = v11 - v5;
          if (v23 * 4 <= v19) {
label_71cb:
            v11 = v19 >> 4;
            if (v11 <= 0) goto label_719b;
            v23 = 1;
            do {
              v23 *= 2;
              v11 >>= 2;
            } while (v11);
            v11 = v19 + -1 + v5;
label_71ee:
            v12 = 0;
            v25 = 0;
            do {
              while (v22 = (char *)(v12 + v5 + (long)v14), *v22 != '\x02') {
                v24 = 0;
label_71fb:
                v12 += 1;
                v25 = v24;
                if (v19 <= v12) goto label_7238;
              }
              v24 = v25 + 1;
              if (v25 == v23) {
                v12 -= v24;
                goto label_71fb;
              }
              if (v25 <= v23) goto label_71fb;
              v12 += 1;
              *v22 = '\0';
              v25 = v24;
            } while (v12 < v19);
label_7238:
            v5 = 0;
            v22 = v8;
            do {
              while( true ) {
                v1 = *v22;
                if ((8 <= (long)v22 - (long)v8) && (v1 == '\x01')) goto label_7290;
                if (v1 != '\x02') break;
                *v22 = '\0';
                v22 = &v22[1];
                v5 = 0;
                if (&v8[v19] == v22) goto label_7290;
              }
              if (v1) { // branch-flip
                v5 += 1;
                if (v5 == 3) break;
              }
              else {
                v5 = 0;
              }
              v22 = &v22[1];
            } while (&v8[v19] != v22);
label_7290:
            v5 = 0;
            v8 = (char *)((long)v14 + v11);
            do {
              if ((8 <= ((long)v14 + v11) - (long)v8) && (*v8 == '\x01')) break;
              if (*v8 != '\x02') { // branch-flip
                if (*v8) { // branch-flip
                  v5 += 1;
                  if (v5 == 3) break;
                }
                else {
                  v5 = 0;
                }
              }
              else {
                *v8 = '\0';
                v5 = 0;
              }
              v8 = &v8[-1];
            } while ((char *)((long)v14 + (v11 - v19)) != v8);
label_71b0:
            v5 = v11; // crossjump-dupe
          }
        }
        else {
          do {
            if (*(char *)((long)v14 + v11 + -1) != '\x02') {
              v19 = v11 - v5;
              if (v23 * 4 <= v19) goto label_71cb;
              if (v5 < v11) {
                v8 = (char *)((long)v14 + v11 + -1);
                do {
                  if (*v8 == '\x02')
                    *v8 = '\0';
                  v8 = &v8[-1];
                } while ((char *)((long)v14 + v5 + -1) != v8);
              }
              goto label_7018;
            }
            v11 -= 1;
            v23 -= 1;
            *(char *)((long)v14 + v11) = 0;
          } while (v11 != v5);
          v19 = 0;
          if (v23 <= 0) {
label_719b:
            v11 = v19 + -1 + v5;
            if (0 < v19) {
              v23 = 1;
              goto label_71ee;
            }
            v5 = v11;
          }
        }
      }
label_7018:
      v5 += 1;
    } while (v5 < v3);
  }
label_703b:
  v26 += 0x130;
  v17 = &v17[1];
  if (v26 == 0x260) {
    v10 = (long *)((long)a0 + 200);
    v14 = v6;
    do {
      v26 = *v10;
      v3 = 0;
      if (1 <= v26) {
        v5 = 0;
        do {
          while ((!dat_26408 && (*(char *)((long)v14 + v5)))) {
            *(char *)(v10[10] + v5) = 1;
            v5 += 1;
            if (v26 == v5) goto label_70d6;
          }
          v11 = v3 * 8;
          *(unsigned long *)(v10[7] + v3 * 8) = *(unsigned long *)(v10[6] + v5 * 8);
          v3 += 1;
          *(long *)(v10[8] + v11) = v5;
          v5 += 1;
        } while (v26 != v5);
      }
label_70d6:
      v10[9] = v3;
      v10 = &v10[0x26];
      v27 = &v27[1];
      if ((long *)((long)a0 + 0x328) == v10) {
        free(v6);
        free(v4); // tail-call
        return;
      }
      v14 = (void *)*v27;
    } while( true );
  }
  v14 = (void *)*v17;
  goto label_6ff4;
}

// Function: sub_73b0 @ 0x73b0
void sub_73b0(int a0,void *a1)
{
  long v1;
  long v2;
  char *v3; // rax
  
  if (!a0)
    return;
  v2 = dat_26548;
  if (!dat_26548)
    v2 = *(long *)((long)a1 + 0x138);
  v1 = dat_26540;
  if (!dat_26540)
    v1 = *(long *)((long)a1 + 8);
  if (dat_264a1) // branch-flip
    v3 = dcgettext(NULL,"Files %s and %s differ\n",5);
  else {
    v3 = dcgettext(NULL,"Binary files %s and %s differ\n",5);
  }
  sub_e0e0(v3,v1,v2); // tail-call
}

// Function: sub_7460 @ 0x7460
char sub_7460(int *a0) // ternary
{
  int *v1;
  unsigned long *v10;
  long *v11; // rax
  char *v12; // rax
  long v13;
  char *v14;
  long v15; // rbx
  unsigned long v16; // stack - 0x78
  char v17 [8];
  char v18 [8];
  char v19 [8];
  unsigned long v2;
  char v20 [8];
  unsigned long *v21;
  int *v22;
  void *v23;
  long v24;
  char v25; // r12b
  void *v26; // stack - 0xa0
  unsigned long v27; // stack - 0x70
  long v28; // stack - 0x68
  long v29; // stack - 0x60
  void *v3;
  char v30; // stack - 0x58
  long v31; // stack - 0x50
  unsigned long *v4;
  char v5; // al
  int v6; // eax
  unsigned long v7;
  unsigned long v8; // rax
  long v9; // rax
  
  v5 = sub_c8f0(a0,(unsigned char)dat_26553);
  if (v5) {
    if ((((1 <= *(long *)&a0[0x10] && *(long *)&a0[0x10] != *(long *)&a0[0x5c]) && (1 <= *(long *)&a0[0x5c])) && ((*a0 <= -1 || ((a0[10] & 0xf000U) == 0x8000)))) && ((a0[0x4c] <= -1 || ((a0[0x56] & 0xf000U) == 0x8000)))) {
label_75bb:
      v25 = 1;
    }
    else {
      if (*a0 != a0[0x4c]) {
        v7 = *(unsigned long *)&a0[0x5e];
        v2 = *(unsigned long *)&a0[0x12];
        v7 = sub_165d0(8,sub_165d0(v2,v7,0x7ffffffffffffffe),0x7ffffffffffffffe);
        v8 = sub_15fa0(*(void **)&a0[0x28],v7);
        v23 = *(void **)&a0[0x74];
        *(unsigned long *)&a0[0x28] = v8;
        *(unsigned long *)&a0[0x74] = sub_15fa0(v23,v7);
        while( true ) {
          if (0 <= *a0)
            sub_bf50(a0,v7 - *(long *)&a0[0x2c]);
          if (0 <= a0[0x4c])
            sub_bf50((struct_12 *)&a0[0x4c],v7 - *(long *)&a0[0x78]);
          v2 = *(unsigned long *)&a0[0x2c];
          if ((v2 != *(unsigned long *)&a0[0x78]) || (v23 = *(void **)&a0[0x74], v3 = *(void **)&a0[0x28], memcmp(v3,v23,v2))) goto label_75bb;
          if (v2 != v7) break;
          a0[0x78] = 0;
          a0[0x79] = 0;
          a0[0x2c] = 0;
          a0[0x2d] = 0;
        }
      }
      v25 = 0;
    }
    sub_73b0((unsigned char)v25,a0);
    goto label_75d6;
  }
  v26 = (void *)sub_f5e0(*(long *)&a0[0x7e] + *(long *)&a0[0x32] + 4);
  *(long *)&a0[0x46] = (long)v26 + 1;
  *(long *)&a0[0x92] = (long)v26 + *(long *)&a0[0x32] + 3;
  sub_6e30(a0);
  v16 = *(unsigned long *)&a0[0x40];
  v15 = *(long *)&a0[0x90] + *(long *)&a0[0x44] + 3;
  v27 = *(unsigned long *)&a0[0x8c];
  v9 = sub_15f50(v15 * 0x10);
  v24 = *(long *)&a0[0x90];
  v13 = v24 * 8 + 8;
  v28 = v9 + v13;
  v29 = v9 + v13 + v15 * 8;
  v30 = dat_26418;
  if (v15) { // branch-flip
    v31 = 1;
    do {
      v31 *= 2;
      v15 >>= 2;
    } while (v15);
    if (v31 <= 0xfff)
      v31 = 0x1000;
  }
  else {
    v31 = 0x1000;
  }
  v13 = 0x26;
  v22 = a0;
  v10 = (unsigned long *)0x261a0;
  while (v13) {
    v21 = &v10[1];
    v1 = &v22[2];
    *v10 = *(unsigned long *)v22;
    v13 -= 1;
    v22 = v1;
    v10 = v21;
  }
  v13 = 0x26;
  v22 = &a0[0x4c];
  while (v13) {
    v21 = &v10[1];
    v1 = &v22[2];
    *v10 = *(unsigned long *)v22;
    v13 -= 1;
    v22 = v1;
    v10 = v21;
  }
  sub_6890(0,*(long *)&a0[0x44],0,v24,(unsigned char)dat_26408,&v16);
  free((void *)(*(long *)&a0[0x90] * -8 + -8 + v28));
  sub_6a70(a0);
  v10 = (dat_26580 != 4) ? (unsigned long *)sub_6d50(a0) : (unsigned long *)sub_6c70(a0); // branch-flip
  if ((dat_26554) || (dat_264e0)) {
    if (v10) {
      v21 = v10;
      do {
        v11 = (long *)sub_eaf0(v21);
        v4 = (unsigned long *)*v11;
        *v11 = 0;
        v6 = sub_f1a0(v21,v19,v17,v20,v18);
        *v11 = (long)v4;
        if (!v4) break;
        v21 = v4;
      } while (!v6);
      v25 = v6 != 0;
      goto label_780b;
    }
    if (!dat_264a1) goto label_78dc;
    v25 = 0;
label_79cb:
    sub_73b0((unsigned char)v25,a0);
  }
  else {
    v25 = v10 != NULL;
label_780b:
    if (dat_264a1) goto label_79cb;
    if ((bool)v25) { // branch-flip
label_7821:
      v13 = dat_26548;
      if (!dat_26548)
        v13 = *(long *)&a0[0x4e];
      v24 = dat_26540;
      if (!dat_26540)
        v24 = *(long *)&a0[2];
      sub_e280(v24,v13,*(long *)&a0[0x98] != 0);
      switch(dat_26580) {
        default:
          abort(); // no-return
        case 1:
          sub_ccf0(v10);
          break;
        case 2:
          sub_8870(v10,0);
          break;
        case 3:
          sub_8870(v10,1);
          break;
        case 4:
          sub_aa10(v10);
          break;
        case 5:
          sub_aa30(v10);
          break;
        case 6:
          sub_aa50(v10);
          break;
        case 7:
          sub_b430(v10);
          break;
        case 8:
          sub_d5b0(v10);
        
      }
      sub_e550();
    }
    else {
label_78dc:
      v25 = 0;
      if (!dat_26578) goto label_7821;
    }
  }
  free(*(void **)&a0[0x40]);
  free(v26);
  free(*(void **)&a0[0x3e]);
  free((void *)(*(long *)&a0[0x2e] + *(long *)&a0[0x30] * 8));
  free(*(void **)&a0[0x8a]);
  free((void *)(*(long *)&a0[0x7a] + *(long *)&a0[0x7c] * 8));
  while (v10) {
    v21 = (unsigned long *)*v10;
    free(v10);
    v10 = v21;
  }
  if ((unsigned int)(dat_26580 - 4U) <= 1) {
    v11 = (long *)0x26540;
    v22 = &a0[0x48];
    do {
      if ((char)*v22) {
        v12 = dcgettext(NULL,"No newline at end of file",5);
        v14 = (char *)*v11;
        if (!v14)
          v14 = *(char **)&v22[-0x46];
        error(0,0,"%s: %s\n",v14,v12);
        v25 = 2;
      }
      v11 = &v11[1];
      v22 = &v22[0x4c];
    } while (v11 != (long *)0x26550);
  }
label_75d6:
  v23 = *(void **)&a0[0x74];
  if (*(void **)&a0[0x28] != v23) {
    free(*(void **)&a0[0x28]);
    v23 = *(void **)&a0[0x74];
  }
  free(v23);
  return v25;
}

// Function: sub_7ab0 @ 0x7ab0
void sub_7ab0(char *a0,struct_9 *a1,char *a2,char *a3)
{
  tm *v1; // rax
  unsigned long v2; // rax
  char v3 [56];
  
  sub_eb70(0);
  if (a3) // branch-flip
    __fprintf_chk(dat_26188,1,"%s %s",a0,a3);
  else {
    v1 = localtime(&a1->field_0x68);
    v2 = sub_154d0((struct_8 *)&a1->field_0x0[0x10]);
    if ((!v1) || (!sub_13d10(v3,0x2b,dat_26400,v1,0,v2 & 0xffffffff)))
      __sprintf_chk(v3,1,0x2b,"%ld.%.9d",a1->field_0x68,(int)v2);
    __fprintf_chk(dat_26188,1,"%s %s\t%s",a0,a2,v3);
  }
  sub_eb70(3);
  putc_unlocked(10,dat_26188); // tail-call
}

// Function: sub_7be0 @ 0x7be0
void sub_7be0(unsigned long *a0) // return-dupe
{
  unsigned long *v1;
  int v2; // eax
  char v3 [8];
  char v4 [8];
  char v5 [8];
  char v6 [8];
  
  if (!a0)
    return;
  do {
    v1 = (unsigned long *)*a0;
    *a0 = 0;
    v2 = sub_f1a0(a0,v6,v5,v4,v3);
    *a0 = v1;
    *(bool *)&a0[5] = v2 == 0;
    a0 = v1;
  } while (v1);
}

// Function: sub_7c80 @ 0x7c80
char * sub_7c80(long a0,long a1)
{
  long v1;
  unsigned long v2; // r8
  char *v3;
  
  v1 = dat_26178;
  dat_26178 = a1;
  do {
    a1 -= 1;
    if (a1 < v1) {
      v3 = NULL;
      if (dat_26170 != 0x7fffffffffffffff)
        v3 = *(char **)(a0 + dat_26170 * 8);
      return v3;
    }
    v3 = *(char **)(a0 + a1 * 8);
    v2 = ~(unsigned long)v3 + *(long *)(a0 + 8 + a1 * 8);
    if (0x80000000 <= v2)
      v2 = 0x7fffffff;
  } while (re_search((re_pattern_buffer *)0x26500,v3,(int)v2,0,(int)v2,NULL) <= -1);
  dat_26170 = a1;
  return v3;
}

// Function: sub_7d30 @ 0x7d30
void sub_7d30(FILE *a0,char *a1)
{
  char v1;
  char v2; // al
  int v3;
  char *v4;
  char *v5;
  int v6; // r14d
  
  v6 = 0;
  putc_unlocked(0x20,a0);
  v4 = a1;
  while( true ) {
    v1 = *v4;
    v5 = &v4[1];
    v2 = sub_f850((unsigned char)v1);
    if ((v1 == '\n') || (!v2)) break;
    v6 += 1;
    v4 = v5;
  }
  v5 = &a1[(long)v6 + 1];
  v3 = v6;
  while ((v1 != '\n' && (v3 = v3 + 1, v3 <= v6 + 0x27))) {
    v1 = *v5;
    v5 = &v5[1];
  }
  if (v6 < v3) {
    a1 = &a1[(long)v3 + -1];
    do {
      v1 = *a1;
      if (!sub_f850((unsigned char)v1)) break;
      v3 -= 1;
      a1 = &a1[-1];
    } while (v6 != v3);
  }
  fwrite_unlocked(v4,1,(long)(v3 - v6),a0); // tail-call
}

// Function: sub_7e00 @ 0x7e00
void sub_7e00(void) // return-dupe
{
  long v1; // stack - 0x20
  long v2; // stack - 0x18
  
  sub_f0c0();
  if (v2 <= v1) {
    __fprintf_chk(dat_26188,1,"%ld",v2);
    return;
  }
  __fprintf_chk(dat_26188,1,"%ld,%ld",v1,v2);
}

// Function: sub_7e90 @ 0x7e90
void sub_7e90(unsigned long *a0)
{
  bool v1;
  unsigned long *v10;
  unsigned int v2; // eax
  char *v3;
  long v4;
  long v5; // stack - 0x50
  long v6; // stack - 0x58
  long v7; // stack - 0x60
  long v8; // stack - 0x48
  FILE *v9;
  
  v2 = sub_f1a0(a0,&v7,&v6,&v5,&v8);
  if (v2) {
    v4 = -dat_26288;
    v7 -= dat_26570;
    if (v7 < v4)
      v7 = v4;
    v5 -= dat_26570;
    if (v5 < v4)
      v5 = v4;
    v4 = dat_26570 + v6;
    v1 = v6 < dat_26270 - dat_26570;
    v6 = dat_26270 + -1;
    if (v1)
      v6 = v4;
    v4 = dat_26570 + v8;
    v1 = v8 < dat_263a0 - dat_26570;
    v8 = dat_263a0 + -1;
    if (v1)
      v8 = v4;
    if (dat_26520) { // branch-flip
      v3 = (char *)sub_7c80(dat_26258);
      sub_e2b0();
      v9 = dat_26188;
      fputs_unlocked("***************",dat_26188);
      if (v3)
        sub_7d30(v9,v3);
    }
    else {
      sub_e2b0();
      v9 = dat_26188;
      fputs_unlocked("***************",dat_26188);
    }
    putc_unlocked(10,v9);
    sub_eb70(4);
    fputs_unlocked("*** ",v9);
    sub_7e00(0x261a0,v7,v6);
    fputs_unlocked(" ****",v9);
    sub_eb70(3);
    putc_unlocked(10,v9);
    if ((v2 & 1) && (v7 <= v6)) {
      v10 = a0;
      v4 = v7;
      do {
        sub_eb70(2);
        for (; v10; v10 = (unsigned long *)*v10) {
          if (v4 < (long)(v10[2] + v10[3])) {
            if ((long)v10[3] <= v4) {
              v3 = "-";
              if (1 <= (long)v10[1])
                v3 = "!";
              goto label_8084;
            }
            break;
          }
        }
        v3 = " ";
label_8084:
        sub_ef20(v3,(struct_11 *)(dat_26258 + v4 * 8),1);
        sub_eb70(3);
        if (*(char *)(*(long *)(dat_26258 + 8 + v4 * 8) + -1) == '\n')
          putc_unlocked(10,v9);
        v4 += 1;
      } while (v4 <= v6);
    }
    sub_eb70(4);
    fputs_unlocked("--- ",v9);
    sub_7e00(0x262d0,v5,v8);
    fputs_unlocked(" ----",v9);
    sub_eb70(3);
    putc_unlocked(10,v9);
    if ((v2 & 2) && (v5 <= v8)) {
      v4 = v5;
      do {
        sub_eb70(1);
        for (; a0; a0 = (unsigned long *)*a0) {
          if (v4 < (long)(a0[1] + a0[4])) {
            if ((long)a0[4] <= v4) {
              v3 = "+";
              if (1 <= (long)a0[2])
                v3 = "!";
              goto label_819d;
            }
            break;
          }
        }
        v3 = " ";
label_819d:
        sub_ef20(v3,(struct_11 *)(dat_26388 + v4 * 8),1);
        sub_eb70(3);
        if (*(char *)(*(long *)(dat_26388 + 8 + v4 * 8) + -1) == '\n')
          putc_unlocked(10,v9);
        v4 += 1;
      } while (v4 <= v8);
    }
  }
}

// Function: sub_8280 @ 0x8280
void sub_8280(void) // return-dupe
{
  char *v1; // rdx
  long v2; // stack - 0x20
  long v3; // stack - 0x18
  
  sub_f0c0();
  if (v3 <= v2) {
    v1 = "%ld,0";
    if (v2 <= v3)
      v1 = "%ld";
    __fprintf_chk(dat_26188,1,v1);
    return;
  }
  __fprintf_chk(dat_26188,1,"%ld,%ld",v2,(v3 - v2) + 1);
}

// Function: sub_8320 @ 0x8320
void sub_8320(unsigned long *a0) // return-dupe
{
  long v1;
  long v10; // stack - 0x48
  long v11;
  struct_11 *v12;
  long v13;
  long v14; // stack - 0x78
  bool v2;
  long v3;
  char *v4; // rax
  long v5;
  FILE *v6;
  long v7; // stack - 0x50
  long v8; // stack - 0x58
  long v9; // stack - 0x60
  
  if (!sub_f1a0(a0,&v9,&v8,&v7,&v10))
    return;
  v9 -= dat_26570;
  v5 = -dat_26288;
  if (v9 < v5)
    v9 = v5;
  v7 -= dat_26570;
  if (v7 < v5)
    v7 = v5;
  v5 = dat_26570 + v8;
  v2 = v8 < dat_26270 - dat_26570;
  v8 = dat_26270 + -1;
  if (v2)
    v8 = v5;
  v5 = dat_26570 + v10;
  v2 = v10 < dat_263a0 - dat_26570;
  v10 = dat_263a0 + -1;
  if (v2)
    v10 = v5;
  if (dat_26520) { // branch-flip
    v4 = (char *)sub_7c80(dat_26258);
    sub_e2b0();
    v6 = dat_26188;
    sub_eb70(4);
    fputs_unlocked("@@ -",v6);
    sub_8280(0x261a0,v9,v8);
    fputs_unlocked(" +",v6);
    sub_8280(0x262d0,v7,v10);
    fputs_unlocked(" @@",v6);
    sub_eb70(3);
    if (v4)
      sub_7d30(v6,v4);
  }
  else {
    sub_e2b0(dat_263a0 - dat_26570,v9);
    v6 = dat_26188;
    sub_eb70(4);
    fputs_unlocked("@@ -",v6);
    sub_8280(0x261a0,v9,v8);
    fputs_unlocked(" +",v6);
    sub_8280(0x262d0,v7,v10);
    fputs_unlocked(" @@",v6);
    sub_eb70(3);
  }
  putc_unlocked(10,v6);
  v14 = v7;
  v5 = v9;
  do {
    if ((v8 < v5) && (v10 < v14))
      return;
    while ((!a0 || (v5 < (long)a0[3]))) {
      v1 = v5 + 1;
      v12 = (struct_11 *)(dat_26258 + v5 * 8);
      if ((!dat_26491) || (*v12->field_0x0 != 10))
        putc_unlocked((-(unsigned int)(dat_26492 == '\0') & 0x17) + 9,v6);
      sub_f0a0(NULL,v12);
      v14 += 1;
      v5 = v1;
      if ((v8 < v1) && (v10 < v14))
        return;
    }
    v1 = a0[2];
    if (v1) {
      v13 = v5 * 8;
      v11 = v1;
      do {
        while( true ) {
          v3 = dat_26258;
          sub_eb70(2);
          putc_unlocked(0x2d,v6);
          v12 = (struct_11 *)(v3 + v13);
          if ((dat_26492) && ((!dat_26491 || (*v12->field_0x0 != 10))))
            putc_unlocked(9,v6);
          sub_ef20(NULL,v12,1);
          sub_eb70(3);
          if (v12->field_0x8[-1] != 10) break;
          v13 += 8;
          putc_unlocked(10,v6);
          v11 -= 1;
          if (!v11) goto label_8686;
        }
        v13 += 8;
        v11 -= 1;
      } while (v11);
label_8686:
      v5 += v1;
    }
    v1 = a0[1];
    if (v1) {
      v13 = v14 * 8;
      v11 = v1;
      do {
        while( true ) {
          v3 = dat_26388;
          sub_eb70(1);
          putc_unlocked(0x2b,v6);
          v12 = (struct_11 *)(v3 + v13);
          if ((dat_26492) && ((!dat_26491 || (*v12->field_0x0 != 10))))
            putc_unlocked(9,v6);
          sub_ef20(NULL,v12,1);
          sub_eb70(3);
          if (v12->field_0x8[-1] != 10) break;
          v13 += 8;
          putc_unlocked(10,v6);
          v11 -= 1;
          if (!v11) goto label_874e;
        }
        v13 += 8;
        v11 -= 1;
      } while (v11);
label_874e:
      v14 += v1;
    }
    a0 = (unsigned long *)*a0;
  } while( true );
}

// Function: sub_8770 @ 0x8770
long * sub_8770(long *a0)
{
  long *v1;
  long v2; // rax
  long v3;
  long v4;
  long v5;
  
  v3 = a0[4];
  v4 = a0[3];
  while( true ) {
    v1 = (long *)*a0;
    if (!v1)
      return a0;
    v5 = dat_26570 * 2 + 1;
    if (*(char *)&v1[5])
      v5 = dat_26570;
    v2 = v3 + a0[1];
    v4 = v1[3] - (v4 + a0[2]);
    v3 = v1[4];
    if (v4 != v3 - v2) break;
    if (v5 <= v4)
      return a0;
    v4 = v1[3];
    a0 = v1;
  }
  abort(); // no-return
}

// Function: sub_87e0 @ 0x87e0
void sub_87e0(long a0,struct_10 *a1,bool a2)
{
  if (a2) {
    sub_7ab0("---",a0,a1->field_0x0,dat_26540);
    sub_7ab0("+++",(struct_9 *)(a0 + 0x130),a1->field_0x8,dat_26548); // tail-call
    return;
  }
  sub_7ab0("***",a0,a1->field_0x0,dat_26540);
  sub_7ab0("---",(struct_9 *)(a0 + 0x130),a1->field_0x8,dat_26548); // tail-call
}

// Function: sub_8870 @ 0x8870
void sub_8870(long *a0,bool a1)
{
  unsigned long *v1;
  long *v2;
  void *v3; // rdx
  
  if ((dat_26554) || (v2 = a0, dat_264e0))
    sub_7be0(a0);
  else {
    while (v2) {
      *(char *)&v2[5] = 0;
      v1 = (unsigned long *)*v2;
      if (!v1) break;
      *(char *)&v1[5] = 0;
      v2 = (long *)*v1;
    }
  }
  v3 = sub_8320;
  dat_26178 = -dat_26288;
  dat_26170 = 0x7fffffffffffffff;
  if (!a1)
    v3 = sub_7e90;
  sub_eb10(a0,sub_8770,v3); // tail-call
}

// Function: sub_8910 @ 0x8910
int sub_8910(void)
{
  return (-(unsigned int)(dat_26551 == '\0') & 0xfffffff0) + 0x10000010;
}

// Function: sub_8930 @ 0x8930
void sub_8930(struct_17 *a0,char *a1)
{
  long v1;
  void *v2;
  unsigned long v3; // rax
  char *v4; // rax
  void *v5;
  unsigned long v6; // rdx
  unsigned long v7;
  long v8;
  
  v3 = strlen(a1);
  v4 = re_compile_pattern(a1,v3,a0->field_0x20);
  if (v4) {
    error(2,0,"%s: %s",a1,v4); // tail-call
    return;
  }
  v2 = a0->field_0x0;
  v8 = a0->field_0x8;
  v7 = a0->field_0x10;
  a0->field_0x18 = v2 != NULL;
  v6 = (unsigned long)((unsigned int)(v2 != NULL) * 2) + v8 + v3;
  a0->field_0x8 = v6;
  v5 = v2;
  if (v7 <= v6) {
    if (!v7)
      v7 = 1;
    do {
      v7 *= 2;
    } while (v7 <= v6);
    a0->field_0x10 = v7;
    v5 = (void *)sub_15fa0(v2);
    a0->field_0x0 = v5;
  }
  if (v2) {
    v1 = v8 + 1;
    *(char *)((long)v5 + v8) = 0x5c;
    v8 += 2;
    *(char *)((long)v5 + v1) = 0x7c;
  }
  memcpy((void *)((long)v5 + v8),a1,v3 + 1); // tail-call
}

// Function: sub_8a10 @ 0x8a10
void sub_8a10(void)
{
  FILE *v1;
  char v2;
  unsigned long v3; // rax
  char *v4; // rax
  unsigned long v5; // rdx
  char *v6;
  long v7; // r12
  
  v6 = dat_36860;
  v7 = 0x24f20;
  __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... FILES\n",5),v6);
  __printf_chk(1,"%s\n\n",dcgettext(NULL,"Compare FILES line by line.",5));
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"Mandatory arguments to long options are mandatory for short options too.\n",5),v1);
  v2 = ' ';
  v6 = "    --normal                  output a normal diff (the default)";
  do {
    if (v2) { // branch-flip
      v6 = dcgettext(NULL,v6,5);
      while (v4 = strchr(v6,10), v4) {
        v3 = (long)&v4[1] - (long)v6;
        if (0xfff < (int)v3)
          __assert_fail("msglen < 4096","diff.c",0x3f8,"usage"); // no-return
        __printf_chk(1,"  %.*s",v3 & 0xffffffff,v6);
        v6 = &v4[1];
      }
      __printf_chk(1,&"  %s\n"[(int)(((unsigned int)(*v6 != '-') & (unsigned int)CONCAT71((undefined7)((unsigned long)v5 >> 8),*v6 != ' ')) * 2)],v6);
      v6 = *(char **)(v7 + 8);
    }
    else {
      putchar_unlocked(10);
      v6 = *(char **)(v7 + 8);
    }
    if (!v6) {
      sub_15e90(); // tail-call
      return;
    }
    v7 += 8;
    v2 = *v6;
  } while( true );
}

// Function: sub_8ba0 @ 0x8ba0
char * sub_8ba0(struct_18 *a0,unsigned long a1,unsigned long a2)
{
  char v1;
  long v2;
  char *v3; // rax
  
  if (!a0->field_0x0)
    return v3;
  v2 = a0->field_0x20;
  v3 = (unsigned long)sub_15f50(0x100);
  v1 = a0->field_0x18;
  *(char **)(v2 + 0x20) = v3;
  if (v1) {
    v3 = re_compile_pattern((char *)a0->field_0x0,a0->field_0x8,(re_pattern_buffer *)a0->field_0x20);
    if (v3) {
      error(2,0,"%s: %s",(char *)a0->field_0x0,v3);
      return v3;
    }
  }
  v3 = (char *)a2;
  return v3;
}

// Function: sub_8c00 @ 0x8c00
void sub_8c00(char *a0,unsigned long a1)
{
  char *v1;
  
  if (a0)
    error(0,0,dcgettext(NULL,a0,5),a1);
  v1 = dat_36860;
  error(2,0,dcgettext(NULL,"Try \'%s --help\' for more information.",5),v1);
}

// Function: sub_8c60 @ 0x8c60
void sub_8c60(unsigned long *a0,char *a1,char *a2)
{
  char *v1;
  
  v1 = (char *)*a0;
  if (v1) {
    if (strcmp(v1,a1)) {
      error(0,0,dcgettext(NULL,"conflicting %s option value \'%s\'",5),a2,a1);
      sub_8c00(NULL,0); // no-return
    }
  }
  *a0 = a1;
}

// Function: sub_8cc0 @ 0x8cc0
void sub_8cc0(char *a0)
{
  if (!a0) {
    dat_2657c = 1;
    return;
  }
  if (!strcmp(a0,"auto")) {
    dat_2657c = 1;
    return;
  }
  if (!strcmp(a0,"always")) {
    dat_2657c = 2;
    return;
  }
  if (!strcmp(a0,"never")) {
    dat_2657c = 0;
    return;
  }
  sub_8c00("invalid color \'%s\'",a0); // no-return
}

// Function: sub_8d50 @ 0x8d50
void sub_8d50(int a0) // return-dupe
{
  if (dat_26580 == a0)
    return;
  if (!dat_26580) {
    dat_26580 = a0;
    return;
  }
  sub_8c00("conflicting output style options",0); // no-return
}

// Function: sub_8d80 @ 0x8d80
char * sub_8d80(unsigned long *a0,int a1)
{
  unsigned long *v1;
  char *v2;
  char *v3;
  char *v4;
  unsigned long *v5;
  unsigned long v6;
  
  if (1 <= a1) { // branch-flip
    v6 = 1;
    v1 = &a0[(unsigned long)(unsigned int)(a1 - 1) + 1];
    v5 = a0;
    do {
      v2 = (char *)*v5;
      v5 = &v5[1];
      v6 = v6 + 1 + sub_144e0(v2);
    } while (v5 != v1);
    v3 = (char *)sub_15f50(v6);
    v4 = v3;
    do {
      *v4 = 0x20;
      v2 = (char *)*a0;
      a0 = &a0[1];
      v4 = (char *)sub_14530(&v4[1],v2);
    } while (a0 != v1);
  }
  else {
    v4 = (char *)sub_15f50(1);
    v3 = v4;
  }
  *v4 = 0;
  return v3;
}

// Function: sub_8e10 @ 0x8e10
void sub_8e10(long a0)
{
  sub_10a50((timespec *)(a0 + 0x58)); // tail-call
}

// Function: sub_8e20 @ 0x8e20
void sub_8e20(void)
{
  if (ferror_unlocked(stdout))
    sub_e230("write failed"); // no-return
  if (!fclose(stdout))
    return;
  sub_e1f0(dcgettext(NULL,"standard output",5)); // no-return
}

// Function: sub_8e70 @ 0x8e70
unsigned int sub_8e70(void *a0,char *a1,char *a2)
{
  stat *v1;
  char *v10;
  int v11 [2]; // stack - 0x2a8
  stat v12 [2]; // stack - 0x298
  long v13 [3]; // stack - 0x168
  int *v14;
  char *v15;
  char *v16;
  unsigned long *v17;
  long *v18;
  unsigned int v19; // r9d
  unsigned long *v2;
  char *v20;
  int v21; // ebx
  bool v22;
  bool v23;
  char *v24; // stack - 0x2c8
  char *v25; // stack - 0x2a0
  int v26; // stack - 0x178
  char *v27; // stack - 0x170
  unsigned long v28; // stack - 0x150
  unsigned long v29; // stack - 0x148
  long *v3;
  long v30; // stack - 0x140
  long v31; // stack - 0x138
  undefined8 v32; // stack - 0x130
  undefined8 v33; // stack - 0x128
  undefined8 v34; // stack - 0x120
  undefined8 v35; // stack - 0x118
  long v36; // stack - 0x110
  undefined8 v37; // stack - 0x108
  long v38; // stack - 0x100
  undefined8 v39; // stack - 0xf8
  long v4;
  undefined8 v40; // stack - 0xf0
  undefined8 v41; // stack - 0xe8
  undefined8 v42; // stack - 0xe0
  void *v43; // stack - 0x48
  unsigned char v5; // al
  int v6; // eax
  int *v7;
  long v8;
  int v9;
  
  if ((!a1) || (!a2)) {
    if ((dat_26585 == '\x01') && (a2)) {
      v43 = a0;
      v8 = 0x4c;
      v7 = v11;
      while (v8) {
        v14 = &v7[2];
        v7[0] = 0;
        v7[1] = 0;
        v8 -= 1;
        v7 = v14;
      }
      if (a1) { // branch-flip
        v11[0] = -2;
        v26 = 0xfffffffe;
      }
      else {
        v11[0] = -1;
label_9578:
        v11[0] = -1;
        v26 = -2;
        if (!a2) goto label_9560;
      }
      v26 = -2;
      a1 = a2;
      goto label_8f50;
    }
    if (!dat_26586) {
      v20 = a1;
      if (!a1)
        v20 = a2;
      v7 = (unsigned long)(unsigned long)(a1 == NULL);
      v7 = (long)((long)v7 * 0x130);
      sub_e0e0("Only in %s: %s\n",*(unsigned long *)((long)a0 + (long)v7 + 8),v20);
      v6 = 1;
      return v6;
    }
    v43 = a0;
    v8 = 0x4c;
    v7 = v11;
    while (v8) {
      v14 = &v7[2];
      v7[0] = 0;
      v7[1] = 0;
      v8 -= 1;
      v7 = v14;
    }
    if (!a1) {
      v11[0] = -1;
      if (a2) goto label_9578;
      v26 = -1;
label_9560:
      v11[0] = -1;
      a1 = NULL;
      goto label_8f50;
    }
    v11[0] = -2;
    if (!a2) {
      v26 = -1;
      a2 = a1;
      goto label_8f50;
    }
  }
  else {
    v43 = a0;
    v8 = 0x4c;
    v7 = v11;
    while (v8) {
      v14 = &v7[2];
      v7[0] = 0;
      v7[1] = 0;
      v8 -= 1;
      v7 = v14;
    }
    v11[0] = -2;
  }
  v11[0] = -2;
  v26 = -2;
label_8f50:
  v43 = a0;
  if (a0) { // branch-flip
    v7 = (char *)sub_10920(*(void **)((long)a0 + 8),a1,NULL);
    v20 = (char *)v7;
    v25 = (char *)v7;
    v7 = (char *)sub_10920(*(void **)((long)a0 + 0x138),a2,NULL);
    v24 = (char *)v7;
  }
  else {
    v25 = a1;
    v20 = NULL;
    v24 = NULL;
    v7 = (int *)a2;
  }
  v27 = (char *)v7;
  v22 = 0;
  v17 = (unsigned long *)v12;
  do {
    if (*(int *)&v17[-2] != -1) {
      if ((!v22) || (v6 = strcmp(v27,v25), v6)) {
        v10 = (char *)v17[-1];
        v6 = strcmp(v10,"-");
        if (v6) { // branch-flip
          if (dat_26550) // branch-flip
            v6 = lstat(v10,(stat *)v17);
          else {
            v6 = stat(v10,(stat *)v17);
          }
          if (v6) {
label_926f:
            v7 = __errno_location();
            v6 = 0xfffffffd - *v7;
            *(unsigned int *)&v17[-2] = v6;
          }
        }
        else {
          *(unsigned int *)&v17[-2] = 0;
          isatty(0);
          v6 = fstat(0,(stat *)v17);
          if (v6) goto label_926f;
          v6 = *(unsigned int *)&v17[3] & 0xf000;
          if (v6 == 0x8000) {
            v7 = (long)lseek(0,0,1);
            if (0 <= (long)v7) { // branch-flip
              v17[6] = v17[6] - (long)v7;
              v7 = (int *)v17[6];
              if ((long)v17[6] <= -1)
                v7 = (int *)0;
              v17[6] = v7;
            }
            else {
              v7 = __errno_location();
              v6 = 0xfffffffd - *v7;
              *(unsigned int *)&v17[-2] = v6;
            }
          }
          sub_8e10(v17);
        }
      }
      else {
        v28 = CONCAT44(v12[0]._28_4_,v12[0]._24_4_);
        v29 = ZEXT48(v12[0]._32_4_);
        v13[0] = v12[0]._0_8_;
        v13[1] = v12[0]._8_8_;
        v26 = v11[0];
        v13[2] = v12[0]._16_8_;
        v30 = v12[0]._40_8_;
        v31 = v12[0]._48_8_;
        v32 = v12[0]._56_8_;
        v33 = v12[0]._64_8_;
        v34 = v12[0]._72_8_;
        v35 = v12[0]._80_8_;
        v36 = v12[0]._88_8_;
        v37 = v12[0]._96_8_;
        v38 = v12[0]._104_8_;
        v39 = v12[0]._112_8_;
        v40 = v12[0]._120_8_;
        v41 = v12[0]._128_8_;
        v42 = v12[0]._136_8_;
      }
    }
    v17 = &v17[0x26];
    if (v22) goto label_906c;
    v22 = 1;
  } while( true );
label_906c:
  v21 = 0;
  v7 = v11;
  v14 = v11;
label_9084:
  v21 += 1;
  if (!dat_26586) {
    if (v21 != 1) goto label_909b;
    if (dat_26585) goto label_91c0;
    goto label_920a;
  }
label_91c0:
  v9 = *v7;
  if (v9 != -2) { // branch-flip
    if (((v9 == -5 || v9 == -0xc) && (!a0)) && (!(v14[0x4c] + 2U & 0xfffffffd))) {
label_91fa:
      *v7 = -1;
      if (v21 != 2) goto label_920a;
label_90a5:
      if (v11[0] != -1) {
        v9 = 0xfffffffd - v11[0];
        if (v26 != -1) { // branch-flip
          if (v9 < 0) goto label_92b4;
        }
        else {
          v8 = 0x12;
          v18 = v13;
          while (v8) {
            v3 = &v18[1];
            *v18 = 0;
            v8 -= 1;
            v18 = v3;
          }
          v28 = CONCAT44(v28._4_4_,v12[0]._24_4_);
          if (v9 <= -1) goto label_92c2;
        }
        v7 = __errno_location();
        *v7 = v9;
        sub_df30(v25);
        v9 = 0xfffffffd - v26;
        if (0 <= v9) {
label_90fd:
          *v7 = v9;
          sub_df30(v27);
        }
        goto label_910c;
      }
      v8 = 0x12;
      v17 = (unsigned long *)v12;
      while (v8) {
        v2 = &v17[1];
        *v17 = 0;
        v8 -= 1;
        v17 = v2;
      }
      v12[0]._24_4_ = (unsigned int)v28;
      if (v26 != -1) { // branch-flip
label_92b4:
        v6 = 0xfffffffd - v26;
        v9 = v6;
        if (0 <= v6) {
          v7 = __errno_location();
          goto label_90fd;
        }
      }
      else {
        v8 = 0x12;
        v18 = v13;
        while (v8) {
          v3 = &v18[1];
          *v18 = 0;
          v8 -= 1;
          v18 = v3;
        }
      }
label_92c2:
      v9 = v11[0];
      if (!a0) {
        v6 = v12[0]._24_4_ & 0xf000;
        v22 = v6 == 0x4000;
        if (v22 != (((unsigned int)v28 & 0xf000) == 0x4000)) {
          v7 = (unsigned long)(unsigned long)v22;
          v4 = (unsigned long)v7;
          v6 = (unsigned int)!v22;
          v8 = (long)v6;
          v7 = (long)(v4 * 0x130);
          v10 = *(char **)((long)v12 + (long)v7 + -8);
          v7 = (char *)sub_f670(v10);
          v20 = (char *)v7;
          v7 = (long)(v8 * 0x130);
          v8 = (long)v7;
          v7 = (char *)sub_a4f0(*(unsigned long *)((long)v12 + (long)v7 + -8),v20);
          v20 = (char *)v7;
          *(int **)((long)v12 + v8 + -8) = v7;
          v6 = strcmp(v10,"-");
          if (!v6)
            sub_e230("cannot compare \'-\' to a directory"); // no-return
          v1 = (stat *)((long)v12 + v8);
          if (dat_26550) // branch-flip
            v6 = lstat(v20,v1);
          else {
            v6 = stat(v20,v1);
          }
          v5 = v6 != 0;
          v9 = v11[0];
          if ((bool)v5) {
            sub_df30(v20);
            goto label_910c;
          }
        }
      }
      if (v9 != -1) { // branch-flip
        if (v26 != -1) { // branch-flip
          if ((v12[0]._8_8_ == v13[1]) && (v12[0]._0_8_ == v13[0])) {
label_972b:
            if (v12[0]._24_4_ == (unsigned int)v28) {
              if (((((v12[0]._16_8_ != v13[2]) || (v12[0]._28_4_ != v28._4_4_)) || (v12[0]._32_4_ != (unsigned int)v29)) || ((v12[0]._48_8_ != v31 || (v12[0]._88_8_ != v36)))) || (v12[0]._104_8_ != v38)) {
                v22 = 0;
                v6 = (unsigned int)v28 & 0xf000;
              }
              else {
                if (dat_26578) goto label_9610;
                v22 = 1;
                v6 = (unsigned int)v28 & 0xf000;
              }
              goto label_95aa;
            }
            goto label_95a0;
          }
          v6 = v12[0]._24_4_ & 0xf000;
          v19 = (unsigned int)v28 & 0xf000;
          if (v6 != 0x6000) { // branch-flip
            v22 = 0;
            if (v6 != 0x2000) goto label_95aa;
            if (v19 == 0x2000) goto label_97ed;
          }
          else if (v19 == 0x6000) {
label_97ed:
            if (v30 == v12[0]._40_8_) goto label_972b;
          }
          if ((v6 == 0x4000) || (v22 = 0, v19 == 0x4000)) {
label_9820:
            if (v26 == -1) goto label_9829;
label_9780:
            v21 = 1;
            v7 = (char *)sub_10830(v13);
            v10 = (char *)v7;
            v16 = dat_26548;
            if (!dat_26548)
              v16 = v27;
            v7 = (unsigned long)sub_10830(v12);
            v15 = dat_26540;
            if (!dat_26540)
              v15 = v25;
            sub_df60("File %s is a %s while file %s is a %s\n",v15,v7,v16,v10);
            goto label_910c;
          }
label_9437:
          v19 = (unsigned int)v28 & 0xf000;
          if ((a0) && (((v6 & 0xffffdfff) != 0x8000 || (((unsigned int)v28 & 0xd000) != 0x8000)))) {
            if ((v9 == -1) || (v26 == -1)) goto label_9473;
            goto label_9780;
          }
          if ((v6 == 0xa000) || (v19 == 0xa000)) {
            if (!dat_26550)
              __assert_fail("no_dereference_symlinks","diff.c",0x549,"compare_files"); // no-return
            if ((v6 != 0xa000) || (v19 != 0xa000)) goto label_9780;
            v7 = (char *)sub_16360(v25);
            v10 = (char *)v7;
            if (v7) { // branch-flip
              v7 = (char *)sub_16360(v27);
              v16 = (char *)v7;
              v9 = 1;
              if (v7) {
                v6 = strcmp(v10,(char *)v7);
                if (!v6) {
                  free(v10);
                  free(v16);
                  goto label_9610;
                }
                v21 = 1;
                sub_e0e0("Symbolic links %s and %s differ\n",v25,v27);
                free(v10);
                free(v16);
                goto label_910c;
              }
            }
            else {
              v9 = 0;
            }
            v7 = (long)(long)v9;
            v7 = (long)((long)v7 * 0x130);
            sub_df30(*(unsigned long *)((long)v12 + (long)v7 + -8));
            free(v10);
            free(NULL);
            goto label_910c;
          }
          v5 = v6 == 0x8000 & dat_26553;
          if ((((v5) && (v19 == 0x8000)) && (v12[0]._48_8_ != v31 && 1 <= v12[0]._48_8_)) && (1 <= v31)) {
            v10 = dat_26548;
            if (!dat_26548)
              v10 = v27;
            v16 = dat_26540;
            if (!dat_26540)
              v16 = v25;
            v21 = 1;
            sub_e0e0("Files %s and %s differ\n",v16,v10);
            goto label_910c;
          }
          if (v9 == -2) {
            v6 = open(v25,0,0);
            v11[0] = v6;
            if (0 <= v6) goto label_98e7;
            sub_df30(v25);
            if (v26 == -2) {
              v23 = 1;
              goto label_98f7;
            }
label_9912:
            if (0 <= v11[0]) {
              v9 = 2;
              goto label_9a55;
            }
            if (v26 < 0) goto label_910c;
label_992d:
            v6 = close(v26);
            if (v6) {
label_993a:
              sub_df30(v27);
            }
            goto label_910c;
          }
label_98e7:
          v23 = 0;
          if (v26 == -2) {
label_98f7:
            if (v22)
              v26 = v11[0];
            else {
              v6 = open(v27,0,0);
              v26 = v6;
              if (v6 <= -1) {
                sub_df30(v27);
                goto label_9912;
              }
            }
            if (v23) goto label_9912;
          }
          v6 = sub_7460(v11);
          v9 = v6;
          if (0 <= v11[0]) {
label_9a55:
            v6 = close(v11[0]);
            if (v6) {
              sub_df30(v25);
              if ((v26 <= -1) || (v26 == v11[0])) goto label_910c;
              goto label_992d;
            }
          }
          v6 = v9;
          if ((0 <= v26) && (v26 != v11[0])) {
            v6 = close(v26);
            v6 = v9;
            if (v6) goto label_993a;
          }
label_9608:
          v21 = v6;
          if (!v21) goto label_9610;
        }
        else {
label_95a0:
          v22 = 0;
          v6 = v12[0]._24_4_ & 0xf000;
label_95aa:
          v23 = ((unsigned int)v28 & 0xf000) != 0x4000;
          if (v6 == 0x4000) {
            if (v23) goto label_9811;
            if (dat_26580 == 7)
              sub_e230("-D option not supported with directories"); // no-return
            if ((a0) && (!dat_26608)) {
              sub_e0e0("Common subdirectories: %s and %s\n",v25,v27);
              goto label_9610;
            }
label_95f7:
            v6 = sub_a0c0(v11,sub_8e70);
            goto label_9608;
          }
          if (v23) goto label_9437;
label_9811:
          if (v9 != -1) goto label_9820;
label_9829:
          if ((dat_26608) && ((dat_26586 || ((dat_26585 && (v9 == -1)))))) goto label_95f7;
          if (!a0)
            __assert_fail("parent","diff.c",0x52d,"compare_files"); // no-return
label_9473:
          v21 = 1;
          v7 = (unsigned long)(unsigned long)(v9 == -1);
          v7 = (long)((long)v7 * 0x130);
          sub_e0e0("Only in %s: %s\n",*(unsigned long *)((long)a0 + (long)v7 + 8),a1);
        }
label_910c:
        v6 = fflush_unlocked(stdout);
        if (v6) {
          v7 = (char *)dcgettext(NULL,"standard output",5);
          sub_e1f0(v7); // no-return
        }
      }
      else {
        if (v26 != -1) {
          v22 = 0;
          v6 = v12[0]._24_4_ & 0xf000;
          goto label_95aa;
        }
label_9610:
        v21 = 0;
        if ((dat_26584) && (v6 = v12[0]._24_4_ & 0xf000, v6 != 0x4000)) {
          v10 = dat_26548;
          if (!dat_26548)
            v10 = v27;
          v16 = dat_26540;
          if (!dat_26540)
            v16 = v25;
          sub_e0e0("Files %s and %s are identical\n",v16,v10);
        }
      }
      free(v20);
      free(v24);
      v6 = v21;
      return v6;
    }
  }
  else if ((((v7[10] & 0xf000U) == 0x8000) && (!(v7[10] & 0x1ffU))) && (!*(long *)&v7[0x10])) goto label_91fa;
label_909b:
  if (v21 == 2) goto label_90a5;
label_920a:
  v7 = &v7[0x4c];
  v14 = &v14[-0x4c];
  goto label_9084;
}

// Function: sub_9cd0 @ 0x9cd0
unsigned long sub_9cd0(void *a0,int a1) // return-dupe x2
{
  long v1;
  long v2; // rcx
  unsigned int v3; // edx
  
  v1 = *(long *)((long)a0 + 0x260);
  if (!v1)
    return 0;
  v2 = (long)a1 * 0x130;
  do {
    if ((*(long *)(v1 + v2 + 0x18) == *(long *)((long)a0 + v2 + 0x18)) && (*(long *)(v1 + v2 + 0x10) == *(long *)((long)a0 + v2 + 0x10)))
      return 1;
    v3 = *(unsigned int *)(v1 + 0x28 + v2) & 0xf000;
    if (v3 != 0x6000) { // branch-flip
      if ((v3 != 0x2000) || ((*(unsigned int *)((long)a0 + v2 + 0x28) & 0xf000) != 0x2000)) goto label_9d1a;
      if (*(long *)(v1 + 0x38 + v2) == *(long *)((long)a0 + v2 + 0x38))
        return 1;
label_9d5c:
      v1 = *(long *)(v1 + 0x260);
    }
    else {
      if ((*(unsigned int *)((long)a0 + v2 + 0x28) & 0xf000) == 0x6000) {
        if (*(long *)(v1 + 0x38 + v2) == *(long *)((long)a0 + v2 + 0x38))
          return 1;
        goto label_9d5c;
      }
label_9d1a:
      v1 = *(long *)(v1 + 0x260);
    }
    if (!v1)
      return 0;
  } while( true );
}

// Function: sub_9da0 @ 0x9da0
void sub_9da0(char *a0,char *a1)
{
  int *v1; // rax
  
  v1 = __errno_location();
  *v1 = 0;
  if (dat_26551) {
    strcasecmp(a0,a1); // tail-call
    return;
  }
  strcoll(a0,a1); // tail-call
}

// Function: sub_9df0 @ 0x9df0
void sub_9df0(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  
  v1 = (char *)*a0;
  v2 = (char *)*a1;
  if (dat_266e8) {
    if (sub_9da0(v1,v2))
      return;
  }
  strcmp(v1,v2); // tail-call
}

// Function: sub_9e40 @ 0x9e40
void sub_9e40(char *a0,char *a1)
{
  if (!dat_266e8) {
    strcmp(a0,a1); // tail-call
    return;
  }
  if (!sub_9da0(a0,a1)) {
    if (!dat_26551) {
      strcmp(a0,a1); // tail-call
      return;
    }
  }
}

// Function: sub_9e90 @ 0x9e90
unsigned long sub_9e90(struct_23 *a0,unsigned long *a1) // return-dupe x2
{
  char *v1;
  unsigned long v10; // rbx
  unsigned long *v11; // rbp
  unsigned long v12;
  unsigned long v13; // stack - 0x48
  unsigned long v14; // stack - 0x40
  int v2;
  char v3; // al
  DIR *v4; // rax
  char *v5;
  int *v6; // rax
  dirent *v7; // rax
  unsigned long v8; // rax
  unsigned long *v9; // rax
  
  v2 = a0->field_0x0;
  a1[1] = 0;
  a1[2] = 0;
  if (v2 != -1) { // branch-flip
    v4 = opendir(a0->field_0x8);
    if (!v4)
      return 0;
    v12 = 0x200;
    v5 = (char *)sub_15f50(0x200);
    a1[2] = (unsigned long)v5;
    v6 = __errno_location();
    v13 = 0;
    v14 = 0;
label_9f10:
    *v6 = 0;
    v7 = readdir(v4);
    if (v7) {
      v1 = &v7->field_0x13;
      v8 = strlen(v1);
      if (v7->field_0x13 == '.') goto label_9f48;
      goto label_9f58;
    }
    v2 = *v6;
    if (v2) {
      closedir(v4);
      *v6 = v2;
      return 0;
    }
    if (closedir(v4))
      return 0;
    if (0xffffffffffffffd < v14)
      sub_16300(); // no-return, return-dupe
    v9 = (unsigned long *)sub_15f50(v14 * 8 + 8);
    *a1 = v14;
    a1[1] = (unsigned long)v9;
    if (v14) {
      v11 = &v9[v14];
      do {
        *v9 = v5;
        v9 = &v9[1];
        v5 = &v5[strlen(v5) + 1];
      } while (v9 != v11);
    }
  }
  else {
    v9 = (unsigned long *)sub_15f50(8);
    *a1 = 0;
    a1[1] = (unsigned long)v9;
  }
  *v9 = 0;
  return 1;
label_9f48:
  if ((v7->field_0x14) && ((v7->field_0x14 != '.' || (v7->field_0x15)))) {
label_9f58:
    v3 = sub_10220(dat_26410,v1);
    if (!v3) {
      v10 = v13 + v8 + 1;
      while (v12 < v10) {
        if (0x3ffffffffffffffe < v12) {
          sub_16300();
        }
        v12 *= 2;
        v5 = (char *)sub_15fa0(v5,v12);
        a1[2] = (unsigned long)v5;
      }
      memcpy(&v5[v13],v1,v8 + 1);
      v14 += 1;
      v13 = v10;
    }
  }
  goto label_9f10;
}

// Function: sub_a0c0 @ 0xa0c0
int sub_a0c0(int *a0,void *a1) // early-return
{
  long *v1;
  int v10; // stack - 0x8c
  void *v11; // stack - 0x68
  long *v12; // stack - 0x58
  void *v13; // stack - 0x50
  char *v2;
  int v3;
  long v4;
  long *v5 [4]; // stack - 0x88
  unsigned long v6; // stack - 0x60
  char *v7;
  long v8;
  long *v9;
  
  v3 = *a0;
  v10 = 0;
  if (((v3 == -1) || (sub_9cd0(a0,0))) && ((a0[0x4c] == -1 || (sub_9cd0(a0,1))))) {
    v7 = *(char **)&a0[(unsigned long)(v3 == -1) * 0x4c + 2];
    error(0,0,dcgettext(NULL,"%s: recursive directory loop",5),v7);
    return 2;
  }
  if (!sub_9e90(a0,&v5[2])) {
    sub_df30(*(char **)&a0[2]);
    v10 = 2;
  }
  if (sub_9e90((struct_23 *)&a0[0x4c],&v6)) { // branch-flip
    if (!v10) {
      dat_266e8 = 1;
      v5[0] = v5[3];
      v5[1] = v12;
      if (_setjmp((void *)0x26620))
        dat_266e8 = 0;
      qsort(v5[0],v5[2],8,sub_9df0);
      qsort(v5[1],v6,8,sub_9df0);
      v8 = dat_26488;
      if ((dat_26488) && (!*(long *)&a0[0x98])) {
        while ((*v5[0] && (v7 = (char *)*v5[0], (int)sub_9e40(v7,v8) <= -1))) {
          v5[0] = &v5[0][1];
        }
        while ((*v5[1] && (v7 = (char *)*v5[1], (int)sub_9e40(v7,v8) <= -1))) {
          v5[1] = &v5[1][1];
        }
      }
      while ((*v5[0] || (*v5[1]))) {
        if (*v5[0]) { // branch-flip
          if (*v5[1]) { // branch-flip
            v3 = sub_9e40((char *)*v5[0],(char *)*v5[1]);
            if (v3) {
              if (0 <= v3) {
                v1 = &v5[1][1];
                v4 = *v5[1];
                v8 = 0;
                v5[1] = v1;
                goto label_a2f5;
              }
              goto label_a33b;
            }
            if ((dat_26551) && (v3 = strcmp((char *)*v5[0],(char *)*v5[1]), v3)) {
              v1 = v5[(v3 >> 0x1f) + 1];
              v7 = (char *)v1[1];
              v2 = (char *)*v5[(unsigned int)-(v3 >> 0x1f)];
              v9 = &v1[1];
              if (v7) {
                while (!sub_9e40(v7,v2)) {
                  v3 = strcmp(v7,v2);
                  if (0 <= v3) {
                    if (!v3) {
                      memmove(&v1[1],v1,(long)v9 - (long)v1);
                      *v1 = (long)v2;
                    }
                    break;
                  }
                  v7 = (char *)v9[1];
                  v9 = &v9[1];
                  if (!v7) break;
                }
              }
            }
            v1 = &v5[1][1];
            v4 = *v5[1];
            v5[1] = v1;
          }
          else {
label_a33b:
            v4 = 0;
          }
          v1 = &v5[0][1];
          v8 = *v5[0];
          v5[0] = v1;
        }
        else {
          v1 = &v5[1][1];
          v4 = *v5[1];
          v8 = 0;
          v5[1] = v1;
        }
label_a2f5:
        v3 = (*a1)(a0,v8,v4);
        if (v10 < v3)
          v10 = v3;
      }
    }
  }
  else {
    sub_df30(*(char **)&a0[0x4e]);
    v10 = 2;
  }
  free(v5[3]);
  free(v11);
  free(v12);
  free(v13);
  return v10;
}

// Function: sub_a4f0 @ 0xa4f0
unsigned long sub_a4f0(void *a0,char *a1)
{
  unsigned long *v1;
  void *v10; // stack - 0x168
  void *v11; // stack - 0x150
  char *v2;
  char *v3;
  unsigned long v4; // rax
  char v5 [8];
  unsigned int v6 [2]; // stack - 0x158
  unsigned long *v7;
  char *v8; // stack - 0x190
  unsigned long *v9; // stack - 0x170
  
  v9 = NULL;
  v10 = NULL;
  v8 = a1;
  if (dat_26551) {
    v6[0] = 0;
    v11 = a0;
    v8 = a1;
    if (sub_9e90(v6,v5)) {
      dat_266e8 = 1;
      if (_setjmp((void *)0x26620)) // branch-flip
        v8 = a1;
      else {
        v2 = (char *)*v9;
        v3 = a1;
        v7 = v9;
        while (v8 = v3, v2) {
          if (!sub_9e40(v2,a1)) {
            v8 = v2;
            if (!strcmp(v2,a1)) break;
            if (a1 != v3)
              v8 = v3;
          }
          v1 = &v7[1];
          v7 = &v7[1];
          v2 = (char *)*v1;
          v3 = v8;
        }
      }
    }
  }
  v4 = sub_10920(a0,v8,NULL);
  free(v9);
  free(v10);
  return v4;
}

// Function: sub_a650 @ 0xa650
void sub_a650(long *a0) // return-dupe x2
{
  char *v1;
  unsigned int v2; // eax
  long v3; // rbx
  long v4; // stack - 0x40
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x50
  long v7; // stack - 0x38
  
  v2 = sub_f1a0(a0,&v6,&v5,&v4,&v7);
  if (!v2)
    return;
  sub_e2b0();
  sub_f0f0(0x2c,(struct_6 *)0x261a0,v6,v5);
  fputc_unlocked((int)*(char *)((unsigned long)v2 + 0x1d550),dat_26188);
  fputc_unlocked(10,dat_26188);
  if (v2 == 1)
    return;
  if (v4 <= v7) {
    do {
      while( true ) {
        v3 = v4 + 1;
        v1 = *(char **)(dat_26388 + v4 * 8);
        v4 = v3;
        if ((*v1 != '.') || (v1[1] != '\n')) break;
        fputs_unlocked("..\n.\ns/.//\n",dat_26188);
        if (v7 < v3)
          return;
        fputs_unlocked("a\n",dat_26188);
      }
      sub_f0a0("");
    } while (v3 <= v7);
  }
  fputs_unlocked(".\n",dat_26188);
}

// Function: sub_a7a0 @ 0xa7a0
void sub_a7a0(long *a0) // return-dupe x2
{
  unsigned int v1; // eax
  long v2; // rbx
  long v3; // stack - 0x30
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x40
  long v6; // stack - 0x28
  
  v1 = sub_f1a0(a0,&v5,&v4,&v3,&v6);
  if (!v1)
    return;
  sub_e2b0();
  fputc_unlocked((int)*(char *)((unsigned long)v1 + 0x1d550),dat_26188);
  sub_f0f0(0x20,(struct_6 *)0x261a0,v5,v4);
  fputc_unlocked(10,dat_26188);
  if (v1 == 1)
    return;
  if (v3 <= v6) {
    do {
      v2 = v3 + 1;
      sub_f0a0("",(struct_11 *)(dat_26388 + v3 * 8));
      v3 = v2;
    } while (v2 <= v6);
  }
  fputs_unlocked(".\n",dat_26188);
}

// Function: sub_a8a0 @ 0xa8a0
void sub_a8a0(long *a0) // return-dupe x3
{
  unsigned int v1; // eax
  long v10; // stack - 0x28
  long v2;
  long v3; // stack - 0x50
  unsigned long v4; // stack - 0x58
  unsigned long v5; // stack - 0x60
  long v6; // stack - 0x48
  long v7; // stack - 0x40
  long v8; // stack - 0x38
  long v9; // stack - 0x30
  
  v1 = sub_f1a0(a0,&v5,&v4,&v3,&v6);
  if (!v1)
    return;
  sub_e2b0();
  sub_f0c0((struct_6 *)0x261a0,v5,v4,&v7,&v8);
  if (v1 & 1) {
    v2 = 1;
    if (v7 <= v8)
      v2 = (v8 - v7) + 1;
    __fprintf_chk(dat_26188,1,"d%ld %ld\n",v7,v2);
  }
  if (!(v1 & 2))
    return;
  sub_f0c0((struct_6 *)0x262d0,v3,v6,&v9,&v10);
  v2 = 1;
  if (v9 <= v10)
    v2 = (v10 - v9) + 1;
  __fprintf_chk(dat_26188,1,"a%ld %ld\n",v8,v2);
  if (v6 < v3)
    return;
  do {
    v2 = v3 + 1;
    sub_f0a0("",(struct_11 *)(dat_26388 + v3 * 8));
    v3 = v2;
  } while (v2 <= v6);
}

// Function: sub_aa10 @ 0xaa10
void sub_aa10(long a0)
{
  sub_eb10(a0,sub_eb00,sub_a650); // tail-call
}

// Function: sub_aa30 @ 0xaa30
void sub_aa30(long a0)
{
  sub_eb10(a0,sub_eaf0,sub_a7a0); // tail-call
}

// Function: sub_aa50 @ 0xaa50
void sub_aa50(long a0)
{
  sub_eb10(a0,sub_eaf0,sub_a8a0); // tail-call
}

// Function: sub_aa70 @ 0xaa70
char * sub_aa70(char *a0,char *a1) // return-dupe x3
{
  char v1;
  char v2;
  int v3;
  char *v4;
  char *v5; // r8
  
  v1 = *a0;
  if (v1 == '\'')
    return NULL;
  if (v1 != '\\') { // branch-flip
    if (!v1)
      return NULL;
    v4 = NULL;
    if (a0[1] == '\'') {
      *a1 = v1;
      v4 = &a0[2];
    }
  }
  else {
    v3 = 0;
    v2 = '\0';
    v1 = a0[1];
    v4 = &a0[2];
    while (v1 != '\'') {
      if (8 <= (unsigned int)((int)v1 - 0x30U))
        return NULL;
      v5 = &v4[1];
      v3 = ((int)v1 - 0x30U) + v3 * 8;
      v2 = (char)v3;
      v1 = *v4;
      v4 = v5;
    }
    if ((char *)0x3 <= &v4[0xfffffffffffffffd - (long)a0])
      return NULL;
    *a1 = v2;
  }
  return v4;
}

// Function: sub_aaf0 @ 0xaaf0
long sub_aaf0(unsigned long *a0,char a1) // early-return
{
  long v1;
  struct_6 *v2;
  
  switch(a1) {
    case 0x45:
      a0 = &a0[3];
      break;
    case 0x46:
      a0 = &a0[3];
label_ab64:
      v1 = a0[1];
      v2 = (struct_6 *)*a0;
      return sub_f0b0(v2,v1); // tail-call
    default:
      switch(a1) {
        case 0x65:
          break;
        case 0x66:
          goto label_ab64;
        default:
          return -1;
        case 0x6c:
          goto label_ab7c;
        case 0x6d:
          goto label_ab9c;
        case 0x6e:
          goto label_abb4;
        
      }
    case 0x4c:
      a0 = &a0[3];
label_ab7c:
      v1 = a0[2];
      v2 = (struct_6 *)*a0;
      return sub_f0b0(v2,v1) + -1;
    case 0x4d:
      a0 = &a0[3];
label_ab9c:
      v1 = a0[2];
      v2 = (struct_6 *)*a0;
      return sub_f0b0(v2,v1); // tail-call
    case 0x4e:
      a0 = &a0[3];
label_abb4:
      return a0[2] - a0[1];
    
  }
  v1 = a0[1];
  v2 = (struct_6 *)*a0;
  return sub_f0b0(v2,v1) + -1;
}

// Function: sub_abe0 @ 0xabe0
unsigned char * sub_abe0(FILE *a0,void *a1,struct_6 *a2,long a3,unsigned long *a4)
{
  char *v1;
  unsigned int v10; // ebx
  char v11 [8];
  char v12; // stack - 0x41
  char *v13; // rsp
  char *v14; // rsp
  char *v15; // rsp
  char *v16; // rsp
  char *v17; // rsp
  char *v18; // rsp
  char *v19; // rsp
  unsigned char v2;
  char *v20; // rsp
  unsigned long v21; // rdi
  char *v22; // rax
  unsigned char *v23; // stack - 0x60
  char *v24; // stack - 0x58
  unsigned char *v25; // stack - 0x50
  long v3;
  char *v4;
  char *v5;
  unsigned char v6; // al
  unsigned char *v7;
  unsigned int v8; // edx
  unsigned int v9;
  
  v13 = v11;
  v14 = v11;
  v15 = v11;
  v20 = v11;
  v7 = (unsigned char *)((long)a1 + 1);
  do {
    v2 = *v7;
    v10 = (unsigned int)v2;
    v9 = (unsigned int)v2;
    v7 = &v7[1];
    if (0x31 <= v2) {
      if ((unsigned int)((int)(char)v2 - 0x30U) <= 9) goto label_aca0;
      goto label_ac40;
    }
  } while (!(0xfffedf7fffffffffU >> ((unsigned long)v2 & 0x3f) & 1));
  v8 = (unsigned int)(char)v2;
  while (v8 - 0x30 <= 9) {
label_aca0:
    v2 = *v7;
    v7 = &v7[1];
    v9 = (unsigned int)(char)v2;
  }
  if (v2 == 0x2e) {
    do {
      v9 = (unsigned int)(char)*v7;
      v7 = &v7[1];
    } while (v9 - 0x30 <= 9);
  }
label_ac40:
  v2 = *v7;
  v7 = &v7[1];
  v6 = (unsigned char)(v9 - 0x58);
  if (v6 <= 0x20) {
    if (0x100801001U >> ((unsigned long)(v9 - 0x58) & 0x3f) & 1) { // branch-flip
      if (a2) { // branch-flip
        if (v2 == 0x6e) {
          v24 = (long)sub_f0b0(a2,a3);
label_ad1a:
          if (!a0)
            return v7;
          v23 = &v7[0xfffffffffffffffe - (long)a1];
          v25 = &v7[0xffffffffffffffff - (long)a1];
          if (0xfa1 <= &v7[1 - (long)a1]) // branch-flip
            v22 = (char *)sub_16340();
          else {
            while (v16 != &v11[-((unsigned long)&v7[0x37 - (long)a1] & 0xfffffffffffff000)]) {
              v17 = &v4[-0x1000];
              v18 = &v4[-0x1000];
              *(unsigned long *)&v4[-8] = *(unsigned long *)&v4[-8];
              v19 = &v4[-0x1000];
            }
            v21 = (unsigned long)((unsigned int)&v7[0x37 - (long)a1] & 0xff0);
            v3 = -v21;
            v20 = &v5[v3];
            if (v21)
              *(unsigned long *)&v5[-8] = *(unsigned long *)&v5[-8];
            v22 = (char *)(((unsigned long)&v5[v3 + 0xf] & 0xfffffffffffffff0) + 0x1f & 0xffffffffffffffe0);
          }
          v1 = &v22[(long)v25];
          *(unsigned long *)&v20[-8] = 0xadb3;
          memcpy(v22,a1,(unsigned long)v23);
          v22[(long)v25] = 'l';
          *v24 = (char)v9;
          v24[1] = '\0';
          *(unsigned long *)&v20[-8] = 0xadde;
          __fprintf_chk(a0,1,v22,v23);
          *(unsigned long *)&v20[-8] = 0xade6;
          sub_11ce0(v22);
          return v7;
        }
      }
      else {
        v24 = (long)sub_aaf0(a4,(int)(char)v2);
        if (0 <= (long)v24) goto label_ad1a;
      }
    }
    else if ((v6 == 0xb) && (v2 == 0x27)) {
      v7 = (unsigned char *)sub_aa70(v7,&v12);
      if (!v7)
        return NULL;
      if (!a0)
        return v7;
      putc_unlocked((int)v12,a0);
      return v7;
    }
  }
  return NULL;
}

// Function: sub_ae40 @ 0xae40
void sub_ae40(FILE *a0,char *a1,long *a2) // return-dupe x3
{
  char *v1;
  unsigned char *v10;
  int v11; // edi
  void *v12;
  char *v13;
  char v2;
  long v3;
  long v4;
  long v5;
  long v6;
  char *v7;
  unsigned long v8; // rax
  long v9;
  
  v3 = *a2;
  v9 = a2[1];
  v4 = *(long *)(v3 + 0xb8);
  v5 = a2[2];
  if (!a0)
    return;
  if ((!dat_264a0) && (*a1 == '%')) {
    if (a1[1] != 'l') { // branch-flip
      if ((a1[1] == 'L') && (!a1[2])) {
        v12 = *(void **)(v4 + v9 * 8);
        v8 = *(long *)(v4 + v5 * 8) - (long)v12;
        fwrite_unlocked(v12,1,v8,a0); // return-dupe, tail-call
        return;
      }
    }
    else if ((a1[2] == '\n') && (!a1[3])) {
      if (v5 <= v9)
        return;
      v12 = *(void **)(v4 + v9 * 8);
      v3 = *(long *)(v4 + v5 * 8);
      v8 = (v3 + (unsigned long)(*(char *)(v3 + -1) != '\n')) - (long)v12;
      fwrite_unlocked(v12,1,v8,a0);
      return;
    }
  }
  if (v5 <= v9)
    return;
  do {
    v2 = *a1;
    v7 = a1;
    while( true ) {
      v13 = &v7[1];
      if (!v2) break;
      while (v11 = (int)v2, v2 == '%') {
        v2 = v7[1];
        v1 = &v7[2];
        if (v2 != 'L') { // branch-flip
          if (v2 != 'l') {
            if (v2 != '%') { // branch-flip
              v7 = (char *)sub_abe0(a0,v7,v3,v9,NULL);
              if (v7) goto label_aecb;
              v11 = 0x25;
            }
            else {
              v11 = 0x25;
              v13 = v1;
            }
            break;
          }
          v6 = *(long *)(v4 + 8 + v9 * 8);
          v10 = (unsigned char *)(v6 - (unsigned long)(*(char *)(v6 + -1) == '\n'));
        }
        else {
          v10 = *(unsigned char **)(v4 + 8 + v9 * 8);
        }
        v13 = &v7[3];
        sub_ed50(*(unsigned char **)(v4 + v9 * 8),v10,NULL,0);
        v2 = *v1;
        v7 = v1;
        if (!v2) goto label_af6b;
      }
      putc_unlocked(v11,a0);
      v7 = v13;
label_aecb:
      v2 = *v7;
    }
label_af6b:
    v9 += 1;
    if (v5 == v9)
      return;
  } while( true );
}

// Function: sub_b040 @ 0xb040
void sub_b040(FILE *a0,char *a1,char a2,unsigned long *a3)
{
  FILE *v1;
  char v10;
  char *v11; // stack - 0x60
  long v12 [3]; // stack - 0x58
  unsigned long v13;
  FILE *v14;
  char *v2;
  int *v3; // rax
  char *v4;
  char *v5;
  long v6; // rax
  long v7; // rcx
  unsigned long *v8;
  char v9;
  
  v9 = *a1;
  if ((v9 == a2) || (!v9))
    return;
  do {
    v5 = &a1[1];
    v10 = v9;
    if (v9 != '%') {
label_b08e:
      v2 = v5;
      if (a0)
        putc_unlocked((int)v10,a0);
      v9 = *v2;
      goto label_b0a5;
    }
    v10 = a1[1];
    v2 = &a1[2];
    v8 = a3;
    switch(v10) {
      case 0x25:
        v5 = v2;
        goto label_b08e;
      default:
        v2 = (char *)sub_abe0(a0,a1,NULL,0,a3);
        v10 = v9;
        if (!v2) goto label_b08e;
        v9 = *v2;
        goto label_b0a5;
      case 0x28:
        v7 = 0;
        while( true ) {
          if (10 <= (unsigned int)((int)*v2 - 0x30U)) { // branch-flip
            v6 = sub_aaf0(a3);
            v12[v7] = v6;
            v10 = v9;
            if (v6 < 0) goto label_b08e;
            v4 = &v2[1];
          }
          else {
            v3 = __errno_location();
            *v3 = 0;
            v12[v7] = strtoimax(v2,&v11,10);
            v10 = v9;
            if (*v3) goto label_b08e;
            v4 = v11;
          }
          v2 = &v4[1];
          v10 = v9;
          if (*v4 != "=?"[v7]) goto label_b08e;
          if (v7 == 1) break;
          v7 = 1;
        }
        if (v12[0] != v12[1]) { // branch-flip
          v14 = a0;
          v1 = NULL;
        }
        else {
          v14 = NULL;
          v1 = a0;
        }
        v5 = (char *)sub_b040(v1,v2,0x3a,a3);
        if (!*v5)
          return;
        v2 = (char *)sub_b040(v14,&v5[1],0x29,a3);
        if (!*v2)
          return;
        v9 = v2[1];
        v2 = &v2[1];
        goto label_b0a5;
      case 0x3c:
        v13 = dat_26448;
        break;
      case 0x3d:
        v13 = dat_26440;
        break;
      case 0x3e:
        v8 = &a3[3];
        v13 = dat_26450;
      
    }
    sub_ae40(a0,v13,v8);
    v9 = a1[2];
label_b0a5:
    if (!v9)
      return;
    a1 = v2;
    if (a2 == v9)
      return;
  } while( true );
}

// Function: sub_b2e0 @ 0xb2e0
void sub_b2e0(char *a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  
  v1 = 0x261a0;
  v4 = 0x262d0;
  v2 = a1;
  v3 = a2;
  v5 = a3;
  v6 = a4;
  sub_b040(dat_26188,a0,0,&v1);
}

// Function: sub_b350 @ 0xb350
void sub_b350(long *a0) // return-dupe
{
  unsigned int v1; // eax
  long v2; // stack - 0x20
  long v3; // stack - 0x28
  long v4; // stack - 0x30
  long v5; // stack - 0x18
  
  v1 = sub_f1a0(a0,&v4,&v3,&v2,&v5);
  if (!v1)
    return;
  sub_e2b0();
  if ((dat_266f8 < v4) || (dat_266f0 < v2))
    sub_b2e0(dat_26460,dat_266f8,v4,dat_266f0,v2);
  dat_266f8 = v3 + 1;
  dat_266f0 = v5 + 1;
  sub_b2e0(*(char **)((unsigned long)v1 * 8 + 0x26460),v4,dat_266f8,v2,dat_266f0);
}

// Function: sub_b430 @ 0xb430
void sub_b430(long a0)
{
  dat_266f0 = -dat_26288;
  dat_266f8 = dat_266f0;
  sub_eb10(a0,sub_eaf0,sub_b350);
  if ((dat_26270 <= dat_266f8) && (dat_263a0 <= dat_266f0))
    return;
  sub_e2b0();
  sub_b2e0(dat_26460,dat_266f8,dat_26270,dat_266f0,dat_263a0); // tail-call
}

// Function: sub_b4c0 @ 0xb4c0
long sub_b4c0(long a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // rax
  
  v1 = 0x20;
  if (10 <= a0)
    v1 = a1 / (unsigned long)(a0 - 1U);
  a2 /= v1;
  if (!a2)
    a2 = 1;
  if (0x787878787878783 <= a2)
    a2 = 0x787878787878782;
  return a2 + 5;
}

// Function: sub_b510 @ 0xb510
void sub_b510(void *a0) // return-dupe
{
  char *v1;
  char v2;
  unsigned long *v3;
  char *v4; // rax
  char *v5;
  char *v6;
  long v7;
  unsigned long *v8;
  
  v3 = *(unsigned long **)((long)a0 + 0xa0);
  if (!v3)
    return;
  v7 = *(long *)((long)a0 + 0xb0);
  if (dat_26490) {
    v1 = (char *)((long)v3 + v7);
    *v1 = '\r';
    v4 = rawmemchr(v3,0xd);
    if (v1 != v4) { // branch-flip
      v5 = v4;
      do {
        while ((v2 = *v4, v2 == '\r' && (v4[1] == '\n'))) {
          v6 = &v5[1];
          v4 = &v4[2];
          *v5 = '\n';
          v5 = v6;
          if (v1 == v4) goto label_b5a0;
        }
        v4 = &v4[1];
        v6 = &v5[1];
        *v5 = v2;
        v5 = v6;
      } while (v1 != v4);
    }
    else {
      v6 = v1;
    }
label_b5a0:
    v7 -= (long)v1 - (long)v6;
  }
  v8 = v3;
  if ((v7) && (v8 = (unsigned long *)((long)v3 + v7), *(char *)((long)v3 + v7 + -1) != '\n')) {
    *(char *)v8 = 10;
    v7 += 1;
    *(char *)((long)a0 + 0x120) = 1;
    v8 = (unsigned long *)((long)v3 + v7);
  }
  *v8 = 0;
  *(long *)((long)a0 + 0xb0) = v7;
}

// Function: sub_b600 @ 0xb600
void sub_b600(void *a0)
{
  unsigned char v1;
  unsigned char *v10;
  unsigned long v11; // rcx
  unsigned long v12; // rsi
  unsigned long v13;
  unsigned long v14; // rax
  unsigned long *v15;
  unsigned long v16; // r13
  long v17;
  unsigned char *v18;
  unsigned char *v19;
  unsigned char v2;
  unsigned char *v20;
  bool v21; // cf
  bool v22; // zf
  unsigned long v23; // stack - 0xc0
  unsigned long *v24; // stack - 0xb8
  unsigned char *v25; // stack - 0xb0
  unsigned long v26; // stack - 0xa8
  unsigned long v27; // stack - 0xa0
  unsigned long v28; // stack - 0x90
  long v29; // stack - 0x88
  long v3;
  void *v30; // stack - 0x80
  unsigned long v31; // stack - 0x78
  char v4;
  unsigned int v5;
  unsigned char *v6; // rax
  long v7;
  long *v8;
  unsigned int v9; // edx
  
  v29 = *(long *)((long)a0 + 0xb8);
  v3 = *(long *)((long)a0 + 0xc0);
  v20 = *(unsigned char **)((long)a0 + 0xe0);
  v31 = *(unsigned long *)((long)a0 + 0xd8);
  v30 = (void *)sub_15f50(v31 * 8);
  v5 = dat_26558;
  v4 = dat_26552;
  v19 = *(unsigned char **)((long)a0 + 0xf0);
  v26 = dat_26708;
  v28 = dat_26700;
  v6 = (unsigned char *)(*(long *)((long)a0 + 0xb0) + *(long *)((long)a0 + 0xa0));
  v22 = dat_26558 == 0;
  v7 = dat_26710;
  if (v20 < v19) {
    v23 = 0;
    v9 = dat_26558 & 1;
    v10 = v20;
    do {
      v13 = dat_26498;
      v1 = *v10;
      v20 = &v10[1];
      v14 = (unsigned long)v1;
      if (v5 != 4) { // branch-flip
        if (5 <= v5) { // branch-flip
          if (v5 != 5) { // branch-flip
label_bbe8:
            if (v4) { // branch-flip
              if (v1 == 10) goto label_bd57;
              v8 = __ctype_tolower_loc();
              v12 = 0;
              v18 = v20;
              do {
                v20 = &v18[1];
                v17 = v14 * 4;
                v1 = *v18;
                v14 = (unsigned long)v1;
                v12 = (v12 << 7 | v12 >> 0x39) + (long)*(int *)(*v8 + v17);
                v18 = v20;
              } while (v1 != 10);
              v25 = &v20[0xffffffffffffffff - (long)v10];
            }
            else {
              v12 = 0;
              v18 = v20;
              if (v1 != 10) { // branch-flip
                do {
                  v20 = &v18[1];
                  v12 = (v12 << 7 | v12 >> 0x39) + v14;
                  v1 = *v18;
                  v14 = (unsigned long)v1;
                  v18 = v20;
                } while (v1 != 10);
                v25 = &v20[0xffffffffffffffff - (long)v10];
              }
              else {
                v25 = NULL;
              }
            }
          }
          else {
            if (v1 == 10) goto label_bd57;
            v12 = 0;
            v17 = *(long *)__ctype_b_loc();
            do {
              if (!(*(unsigned char *)(v17 + 1 + v14 * 2) & 0x20)) {
                if (v4)
                  v14 = (unsigned long)*(int *)(*(long *)__ctype_tolower_loc() + v14 * 4);
                v12 = (v12 << 7 | v12 >> 0x39) + v14;
              }
              v1 = *v20;
              v14 = (unsigned long)v1;
              v20 = &v20[1];
            } while (v1 != 10);
            v25 = &v20[0xffffffffffffffff - (long)v10];
          }
        }
        else {
          if (v5 - 1 > 2) goto label_bbe8;
          if (v1 == 10) goto label_bd57;
          v14 = 0;
          v12 = 0;
          v18 = v20;
          do {
            v16 = (unsigned long)v1;
            if (v5 & 2) {
              v17 = *(long *)__ctype_b_loc();
              if (*(unsigned char *)(v17 + 1 + v16 * 2) & 0x20) {
                v20 = v18;
                do {
                  v2 = *v20;
                  v20 = &v20[1];
                  if (v2 == 10) {
                    v25 = &v20[0xffffffffffffffff - (long)v10];
                    goto label_b820;
                  }
                } while (*(unsigned char *)(v17 + 1 + (unsigned long)v2 * 2) & 0x20);
              }
            }
            if (v9) { // branch-flip
              if (v1 != 9) { // branch-flip
                if (v1 != 0xd) { // branch-flip
                  if (v1 == 8) {
                    v16 = 8;
                    v14 = (v14 - 1) + (unsigned long)(v14 == 0);
                    goto label_ba60;
                  }
                  v14 += 1;
                  v11 = 1;
                }
                else {
                  v16 = 0xd;
                  v11 = 1;
                  v14 = 0;
                }
              }
              else {
                v16 = 0x20;
                v11 = v13 - v14 % v13;
                v21 = CARRY8(v14,v11);
                v14 += v11;
                if (v21)
                  v14 = 0;
              }
            }
            else {
label_ba60:
              v11 = 1;
            }
            if (v4)
              v16 = (unsigned long)*(unsigned char *)(*(long *)__ctype_tolower_loc() + v16 * 4);
            do {
              v12 = (v12 << 7 | v12 >> 0x39) + v16;
              v11 -= 1;
            } while (v11);
            v1 = *v18;
            v20 = &v18[1];
            v18 = v20;
          } while (v1 != 10);
          v25 = &v20[0xffffffffffffffff - (long)v10];
        }
      }
      else {
        if (v1 != 10) {
          v12 = 0;
          v17 = *(long *)__ctype_b_loc();
          do {
            v18 = v20;
            if (*(unsigned char *)(v17 + 1 + v14 * 2) & 0x20) {
              v1 = *v20;
              while( true ) {
                v20 = &v20[1];
                if (v1 == 10) goto label_bcb3;
                v14 = (unsigned long)v1;
                if (!(*(unsigned char *)(v17 + 1 + v14 * 2) & 0x20)) break;
                v1 = *v20;
              }
              v12 = (v12 << 7 | v12 >> 0x39) + 0x20;
              v18 = v20;
            }
            if (v4)
              v14 = (unsigned long)*(int *)(*(long *)__ctype_tolower_loc() + v14 * 4);
            v20 = &v18[1];
            v12 = (v12 << 7 | v12 >> 0x39) + v14;
            v14 = (unsigned long)*v18;
          } while (*v18 != 10);
label_bcb3:
          v25 = &v20[0xffffffffffffffff - (long)v10];
          goto label_b820;
        }
label_bd57:
        v25 = NULL;
        v12 = 0;
      }
label_b820:
      v15 = (unsigned long *)(dat_26720 + (v12 % dat_26718) * 8);
      v24 = v15;
      if ((((v20 == v6) && (*(char *)((long)a0 + 0x120))) && (2 <= (unsigned int)(dat_26580 - 4U))) && (v24 = (unsigned long *)(dat_26720 + -8), 2 <= v5))
        v24 = v15;
      v27 = *v24;
      if (v27) {
        v13 = v27;
label_b885:
        v15 = (unsigned long *)(v13 * 0x20 + v7);
        if (v15[1] == v12) {
          v18 = (unsigned char *)v15[2];
          if ((unsigned char *)v15[3] == v25) {
            if (!memcmp(v18,v10,(unsigned long)v25)) goto label_b8cb;
            if (!(v22 && !v4)) goto label_b8aa;
            v13 = *v15;
            if (!v13) goto label_b953;
            goto label_b885;
          }
          if (v5) {
label_b8aa:
            if (sub_e690(v18,v10)) goto label_b878;
            goto label_b8cb;
          }
        }
label_b878:
        v13 = *v15;
        if (!v13) goto label_b953;
        goto label_b885;
      }
label_b953:
      if (v28 == v26) {
        if (0x1fffffffffffffe < v28) {
          sub_16300(); // no-return, return-dupe
        }
        v13 = v28 << 1;
        v7 = sub_15fa0(v7,v28 << 6);
        v27 = *v24;
        v28 = v13;
      }
      v15 = (unsigned long *)(v26 * 0x20 + v7);
      *v15 = v27;
      v15[1] = v12;
      v15[2] = (unsigned long)v10;
      v15[3] = (unsigned long)v25;
      *v24 = v26;
      v13 = v26;
      v26 += 1;
label_b8cb:
      if (v23 == v31) {
        if (((0x2aaaaaaaaaaaaaa9 < (long)v23) || (v31 = v23 * 2 - v3, 0xffffffffffffffe < v31)) || (0xffffffffffffffe < v23 - v3)) {
          sub_16300();
        }
        v30 = (void *)sub_15fa0(v30,v31 * 8);
        v29 = sub_15fa0((void *)(v29 + v3 * 8),(v31 - v3) * 8) + v3 * -8;
      }
      *(unsigned char **)(v29 + v23 * 8) = v10;
      *(unsigned long *)((long)v30 + v23 * 8) = v13;
      v23 += 1;
      if (v19 <= v20) goto label_bd68;
      v10 = v20;
    } while( true );
  }
  v23 = 0;
label_bd6b:
  v17 = 0;
  *(unsigned long *)((long)a0 + 200) = v23;
  do {
    if (v31 != v23) { // branch-flip
      v8 = (long *)(v29 + v23 * 8);
      *v8 = (long)v20;
    }
    else {
      if (((0x2aaaaaaaaaaaaaa9 < (long)v31) || (v13 = v31 * 2 - v3, 0xffffffffffffffe < v13)) || (0xffffffffffffffe < v31 - v3))
        sub_16300();
      v29 = sub_15fa0((void *)(v29 + v3 * 8),(v13 - v3) * 8) + v3 * -8;
      v8 = (long *)(v29 + v23 * 8);
      *v8 = (long)v20;
      v31 = v13;
    }
    if (v20 == v6) {
      if ((*(char *)((long)a0 + 0x120)) && (2 <= (unsigned int)(dat_26580 - 4U)))
        *v8 = (long)&v20[-1];
label_bea8:
      dat_26710 = v7;
      *(long *)((long)a0 + 0xb8) = v29;
      *(unsigned long *)((long)a0 + 0xd0) = v23;
      *(unsigned long *)((long)a0 + 0xd8) = v31;
      *(void **)((long)a0 + 0xf8) = v30;
      dat_26700 = v28;
      dat_26708 = v26;
      return;
    }
    if ((dat_26570 <= v17) && (dat_26578)) goto label_bea8;
    v23 += 1;
    v19 = v20;
    do {
      v20 = &v19[1];
      v1 = *v19;
      v19 = v20;
    } while (v1 != 10);
    v17 += 1;
  } while( true );
label_bd68:
  goto label_bd6b;
}

// Function: sub_bf50 @ 0xbf50
void sub_bf50(struct_12 *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  if (!a1)
    return;
  if (a0->field_0x121)
    return;
  v1 = sub_16520(a0->field_0x0,(void *)(a0->field_0xb0 + a0->field_0xa0),a1);
  if (v1 != 0xffffffffffffffff) {
    a0->field_0xb0 = a0->field_0xb0 + v1;
    a0->field_0x121 = v1 < a1;
    return;
  }
  sub_e1f0(a0->field_0x8); // no-return
}

// Function: sub_bfd0 @ 0xbfd0
unsigned long sub_bfd0(struct_13 *a0,bool a1)
{
  int v1;
  unsigned long v2;
  void *v3; // rax
  
  if (0 <= a0->field_0x0) { // branch-flip
    v2 = sub_165d0(8,a0->field_0x48,0x7fffffffffffffef);
    a0->field_0xa8 = v2;
    a0->field_0xa0 = (void *)sub_15f50(v2);
    if (!a1) {
      v1 = sub_f710(a0->field_0x0,0);
      sub_bf50(a0,a0->field_0xa8);
      v2 = a0->field_0xb0;
      if (v1) {
        v1 = a0->field_0x0;
        if ((long)lseek(v1,-v2,1) < 0)
          sub_e1f0(a0->field_0x8); // no-return
        a0->field_0xb0 = 0;
        a0->field_0x121 = 0;
      }
      v3 = memchr(a0->field_0xa0,0,v2);
      return CONCAT71((undefined7)((unsigned long)v3 >> 8),v3 != NULL);
    }
  }
  else {
    a0->field_0xa8 = 8;
    a0->field_0xa0 = (void *)sub_15f50(8);
  }
  a0->field_0xb0 = 0;
  a0->field_0x121 = 0;
  return 0;
}

// Function: sub_c0d0 @ 0xc0d0
void sub_c0d0(struct_14 *a0) // return-dupe x2
{
  void *v1;
  unsigned long v2;
  unsigned long v3;
  
  if (a0->field_0x0 < 0)
    return;
  if ((a0->field_0x28 & 0xf000) != 0x8000) // branch-flip
    v2 = a0->field_0xb0;
  else {
    v3 = a0->field_0x40;
    v2 = (v3 & 0xfffffffffffffff8) + 0x10;
    if ((v2 < v3) || (0x7ffffffffffffffe < v2))
      sub_16300(); // no-return, return-dupe
    if (v2 <= (unsigned long)a0->field_0xa8) // branch-flip
      v2 = a0->field_0xb0;
    else {
      a0->field_0xa8 = v2;
      v1 = a0->field_0xa0;
      a0->field_0xa0 = (void *)sub_15fa0(v1);
      v2 = a0->field_0xb0;
    }
    if (v2 <= v3) {
      sub_bf50(a0,(v3 - v2) + 1);
      v2 = a0->field_0xb0;
      if (v2 <= v3)
        return;
    }
  }
  sub_bf50(a0,a0->field_0xa8 - v2);
  v2 = a0->field_0xb0;
  if (!v2)
    return;
  v3 = a0->field_0xa8;
  if (v2 == v3) {
    do {
      if (0x3ffffffffffffff7 < v3) {
        sub_16300();
      }
      a0->field_0xa8 = v3 * 2;
      v1 = (void *)sub_15fa0(a0->field_0xa0);
      a0->field_0xa0 = v1;
      sub_bf50(a0,a0->field_0xa8 - a0->field_0xb0);
      v2 = a0->field_0xb0;
      v3 = a0->field_0xa8;
    } while (v2 == v3);
  }
  v1 = a0->field_0xa0;
  a0->field_0xa8 = v2 + 0x10 & 0xfffffffffffffff8;
  a0->field_0xa0 = (void *)sub_15fa0(v1);
}

// Function: sub_c270 @ 0xc270
void sub_c270(struct_15 *a0)
{
  unsigned char *v1;
  unsigned long v10; // rax
  long v11; // rcx
  long v12;
  unsigned int v13;
  unsigned char *v14;
  unsigned char *v15;
  unsigned long v16;
  unsigned long v17;
  unsigned char *v18;
  long v19;
  unsigned char *v2;
  unsigned char *v20;
  bool v21; // zf
  unsigned char *v22; // stack - 0x70
  unsigned char *v23; // stack - 0x50
  unsigned char *v24; // stack - 0x48
  long v25; // stack - 0x40
  unsigned char v3;
  unsigned char *v4;
  unsigned char *v5;
  unsigned long v6;
  void *v7; // rax
  long v8;
  unsigned long v9;
  
  sub_c0d0(a0);
  sub_b510(a0);
  if (a0->field_0x0 != a0->field_0x130) { // branch-flip
    sub_c0d0(&a0->field_0x130);
    sub_b510(&a0->field_0x130);
    v22 = a0->field_0x1d0;
    v20 = a0->field_0xa0;
    v9 = a0->field_0xb0;
    v17 = a0->field_0x1e0;
    if (v20 == v22) goto label_c2da;
    v18 = &v20[v9];
    v23 = &v22[v17];
    if (v17 <= v9) // branch-flip
      *v23 = ~v20[v17];
    else {
      *v18 = ~v22[v9];
    }
    v5 = v20;
    v14 = v22;
    if (*(long *)v20 == *(long *)v22) {
      do {
        v5 = &v5[8];
        v14 = &v14[8];
      } while (*(long *)v5 == *(long *)v14);
    }
    if (*v14 == *v5) {
      do {
        v5 = &v5[1];
        v14 = &v14[1];
      } while (*v5 == *v14);
    }
    v13 = dat_26580 - 4;
    v8 = dat_26560;
    if ((2 <= v13) && (&v20[v9 - (unsigned char)a0->field_0x120] < v5 != &v22[v17 - (unsigned char)a0->field_0x250] < v14)) {
      v5 = &v5[-1];
      v14 = &v14[-1];
    }
  }
  else {
    v17 = a0->field_0xb0;
    v20 = a0->field_0xa0;
    a0->field_0x1d8 = a0->field_0xa8;
    a0->field_0x1d0 = v20;
    a0->field_0x1e0 = v17;
    a0->field_0x250 = a0->field_0x120;
    v9 = v17;
label_c2da:
    v5 = &v20[v17];
    v18 = &v20[v9];
    v13 = dat_26580 - 4;
    v14 = v5;
    v8 = dat_26560;
    v23 = v5;
    v22 = v20;
  }
  while (v6 = dat_26570, v20 != v5) {
    while (v5[-1] != 10) {
      v5 = &v5[-1];
      v14 = &v14[-1];
      if (v20 == v5) goto label_c33c;
    }
    v12 = v8 + -1;
    if (!v8) break;
    v14 = &v14[-1];
    v5 = &v5[-1];
    v8 = v12;
  }
label_c33c:
  a0->field_0xe0 = v5;
  a0->field_0x210 = v14;
  if (2 <= v13) { // branch-flip
    v24 = v23;
    v14 = v18;
    if (a0->field_0x120 == a0->field_0x250) goto label_c480;
  }
  else {
label_c480:
    v15 = v18;
    v24 = v23;
    v14 = &v5[v9 - v17];
    if (v9 < v17)
      v14 = v5;
    while (v4 = v14, v15 != v14) {
      v1 = &v15[-1];
      v2 = &v24[-1];
      v4 = v15;
      if (v15[-1] != v24[-1]) break;
      v15 = v1;
      v24 = v2;
    }
    if ((v20 != v15) && (v15[-1] != 10))
      v8 = dat_26560 + 1;
    else {
      v8 = dat_26560;
      if (v22 != v24)
        v8 = dat_26560 + (unsigned long)(v24[-1] != 10);
    }
    v12 = v8 + -1;
    v14 = v15;
    if ((v8) && (v15 != v18)) {
      do {
        do {
          v14 = &v15[1];
          v3 = *v15;
          v15 = v14;
        } while (v3 != 10);
        v21 = v12 != 0;
        v12 -= 1;
      } while (v14 != v18 && v21);
    }
    v24 = &v24[(long)v14 - (long)v4];
  }
  a0->field_0xf0 = v14;
  a0->field_0x220 = v24;
  if ((((dat_26578) && (!dat_26520)) && ((long)dat_26570 <= 0x1ffffffffffffffe)) && (dat_26570 < v9)) {
    v8 = sub_b4c0(0,0,(long)v14 - (long)v5);
    v17 = 0;
    v9 = sub_b4c0(0,0,(long)v18 - (long)v14);
    v25 = 1;
    if (0 < (long)v6) {
      do {
        v25 *= 2;
      } while (v25 <= (long)v6);
      v17 = v25 - 1;
    }
    if ((long)v9 < (long)v6)
      v6 = v9;
    v6 = v8 + v25 + v6;
  }
  else {
    v17 = 0xffffffffffffffff;
    v6 = sub_b4c0(0,0,v9);
    v25 = 0;
  }
  v7 = (void *)sub_15f50(v6 * 8);
  v18 = a0->field_0xe0;
  v5 = v20;
  if ((dat_26578) && (v18 == v14)) {
    v9 = 0;
    v11 = 0;
    v14 = v24;
    if (a0->field_0x210 != v24) {
      v9 = 0;
      if (v20 != v18) goto label_c46f;
label_c3fb:
      v11 = 0;
      v9 = 0;
      v14 = a0->field_0x210;
    }
  }
  else {
    v9 = 0;
    if (v20 == v18) goto label_c3fb;
label_c46f:
    do {
      v11 = v9 + 1;
      v19 = v6;
      if ((v9 & v17) == v6) {
        if (0x7fffffffffffffe < (long)v6) {
          sub_16300(); // no-return, return-dupe
        }
        v19 = v6 * 2;
        v7 = (void *)sub_15fa0(v7,v6 << 4);
      }
      *(unsigned char **)((long)v7 + (v9 & v17) * 8) = v5;
      do {
        v14 = &v5[1];
        v3 = *v5;
        v5 = v14;
      } while (v3 != 10);
      v9 = v11;
      v6 = v19;
    } while (v14 != v18);
    v9 = (long)v14 - (long)v20;
    v14 = a0->field_0x210;
  }
  v19 = dat_26570;
  v16 = dat_26570;
  if (v11 <= (long)dat_26570)
    v16 = v11;
  if (!v25)
    v16 = v11;
  v8 = sub_b4c0(v11,v9,(long)v24 - (long)v14);
  v9 = sub_b4c0(v11,v9,(long)v23 - (long)v24);
  if (v19 < (long)v9)
    v9 = v19;
  v9 = v16 + v8 + v9;
  if (((long)v9 < (long)v16) || (0xffffffffffffffe < v9))
    sub_16300();
  v8 = sub_15f50(v9 * 8);
  if (v11 != v16) { // branch-flip
    if ((long)v16 <= 0) goto label_c7a2;
    v10 = v11 - dat_26570;
    v19 = v16 + v10;
    v12 = v10 * -8;
    do {
      *(unsigned long *)(v8 + v12 + v10 * 8) = *(unsigned long *)((long)v7 + (v17 & v10) * 8);
      v10 += 1;
    } while (v19 != v10);
    v17 = 0;
    do {
      *(unsigned long *)((long)v7 + v17 * 8) = *(unsigned long *)(v8 + v17 * 8);
      v17 += 1;
    } while (v16 != v17);
  }
  else if (v11 <= 0) goto label_c7a2;
  v12 = 0;
  do {
    *(unsigned char **)(v8 + v12 * 8) = &v22[*(long *)((long)v7 + v12 * 8) - (long)v20];
    v12 += 1;
  } while (v12 < (long)v16);
label_c7a2:
  a0->field_0x218 = v11;
  a0->field_0x208 = v9 - v16;
  a0->field_0x1e8 = v16 * 8 + v8;
  a0->field_0xb8 = (void *)((long)v7 + v16 * 8);
  a0->field_0x1f0 = -v16;
  a0->field_0xc0 = -v16;
  a0->field_0xd8 = v6 - v16;
  a0->field_0xe8 = v11;
  return;
}

// Function: sub_c8f0 @ 0xc8f0
unsigned int sub_c8f0(struct_16 *a0,unsigned int a1)
{
  int v1; // ecx
  unsigned long v2;
  char v3; // r12b
  unsigned int v4;
  unsigned int v5; // r13d
  
  v5 = (unsigned int)dat_26568;
  v4 = a1 | sub_bfd0(a0,(v5 | a1) & 0xff);
  if (a0->field_0x0 != a0->field_0x130) { // branch-flip
    v4 |= sub_bfd0(&a0->field_0x130,(v5 | a1 | v4) & 0xff);
    v3 = (char)v4;
  }
  else {
    a0->field_0x1d0 = a0->field_0xa0;
    a0->field_0x1d8 = a0->field_0xa8;
    a0->field_0x1e0 = a0->field_0xb0;
    v3 = (char)v4;
  }
  if (v3)
    return v4;
  sub_c270(a0);
  dat_26700 = a0->field_0x208 + a0->field_0xd8 + 1;
  if (0x3ffffffffffffff <= dat_26700)
    sub_16300(); // no-return, return-dupe
  dat_26710 = (void *)sub_15f50(dat_26700 * 0x20);
  dat_26708 = 1;
  if (0x201 <= (unsigned long)((long)dat_26700 / 3)) { // branch-flip
    v1 = 9;
    do {
      v1 += 1;
      dat_26718 = 1L << ((unsigned char)v1 & 0x3f);
    } while (dat_26718 < (unsigned long)((long)dat_26700 / 3));
    dat_26718 -= *(unsigned char *)((long)v1 + 0x1d1a0);
    if (0xffffffffffffffe < dat_26718) {
      sub_16300();
    }
    v2 = dat_26718 * 8 + 8;
  }
  else {
    dat_26718 = 0x1fd;
    v2 = 0xff0;
  }
  dat_26720 = sub_f5e0(v2) + 8;
  sub_b600(a0);
  sub_b600(&a0->field_0x130);
  a0->field_0x258 = dat_26708;
  a0->field_0x128 = dat_26708;
  free(dat_26710);
  free((void *)(dat_26720 + -8));
  return v4;
}

// Function: sub_cac0 @ 0xcac0
void sub_cac0(long *a0) // return-dupe x2
{
  unsigned int v1; // eax
  long v2;
  long v3; // stack - 0x40
  long v4; // stack - 0x48
  long v5; // stack - 0x50
  long v6; // stack - 0x38
  long v7;
  struct_11 *v8; // rsi
  
  v1 = sub_f1a0(a0,&v5,&v4,&v3,&v6);
  if (!v1)
    return;
  sub_e2b0();
  sub_eb70(4);
  sub_f0f0(0x2c,(struct_6 *)0x261a0,v5,v4);
  fputc_unlocked((int)*(char *)((unsigned long)v1 + 0x1d550),dat_26188);
  sub_f0f0(0x2c,(struct_6 *)0x262d0,v3,v6);
  sub_eb70(3);
  fputc_unlocked(10,dat_26188);
  if ((v1 & 1) && (v5 <= v4)) {
    v2 = v5 * 8;
    v7 = v5;
    do {
      while( true ) {
        sub_eb70(2);
        v8 = (struct_11 *)(dat_26258 + v2);
        v2 += 8;
        sub_ef20("<",v8,1);
        sub_eb70(3);
        if (*(char *)(*(long *)(dat_26258 + v2) + -1) != '\n') break;
        v7 += 1;
        putc_unlocked(10,dat_26188);
        if (v4 < v7) goto label_cc30;
      }
      v7 += 1;
    } while (v7 <= v4);
  }
label_cc30:
  if (v1 != 3) { // branch-flip
    if (!(v1 & 2))
      return;
  }
  else {
    fputs_unlocked("---\n",dat_26188);
  }
  if (v6 < v3)
    return;
  v2 = v3 * 8;
  v7 = v3;
  do {
    sub_eb70(1);
    v8 = (struct_11 *)(dat_26388 + v2);
    v2 += 8;
    sub_ef20(">",v8,1);
    sub_eb70(3);
    if (*(char *)(*(long *)(dat_26388 + v2) + -1) == '\n')
      putc_unlocked(10,dat_26188);
    v7 += 1;
  } while (v7 <= v6);
}

// Function: sub_ccf0 @ 0xccf0
void sub_ccf0(long a0)
{
  sub_eb10(a0,sub_eaf0,sub_cac0); // tail-call
}

// Function: sub_cd10 @ 0xcd10
unsigned long sub_cd10(unsigned long a0,unsigned long a1)
{
  unsigned long v1;
  FILE *v2;
  unsigned long v3;
  unsigned long v4;
  
  v3 = dat_26498;
  v2 = dat_26188;
  if (!dat_264a0) {
    v4 = (dat_26498 + a0) - a0 % dat_26498;
    while (v1 = v4, v1 <= a1) {
      putc_unlocked(9,v2);
      v4 = v1 + v3;
      a0 = v1;
    }
  }
  if (a0 < a1) {
    do {
      a0 += 1;
      putc_unlocked(0x20,v2);
    } while (a1 != a0);
  }
  return a1;
}

// Function: sub_cda0 @ 0xcda0
unsigned long sub_cda0(struct_10 *a0,unsigned long a1,unsigned long a2)
{
  char v1;
  unsigned long v10;
  int v11; // eax
  unsigned long v12;
  char *v2;
  FILE *v3;
  unsigned long v4;
  long v5; // rcx
  char *v6;
  char *v7;
  unsigned long v8; // stack - 0x48
  int v9; // stack - 0x4c
  
  v3 = dat_26188;
  v12 = 0;
  v2 = a0->field_0x8;
  v8 = 0;
  if (a0->field_0x0 < v2) {
    v6 = a0->field_0x0;
    v10 = 0;
    do {
      while( true ) {
        v1 = *v6;
        v11 = (int)v1;
        v7 = &v6[1];
        if ('_' < v1) break;
        if ('\b' <= v1) { // branch-flip
          switch(v11 - 8U & 0xff) {
            case 0:
              if (v10) {
                v10 -= 1;
                if (v10 < a2) {
                  if (v12 <= v10) { // branch-flip
                    if (v12 < v10) {
                      do {
                        v12 += 1;
                        putc_unlocked(0x20,v3);
                      } while (v10 != v12);
                    }
                  }
                  else {
                    putc_unlocked(8,v3);
                    v12 = v10;
                  }
                }
              }
              break;
            case 1:
              v5 = dat_26498 - v10 % dat_26498;
              if (v12 == v10) {
                v4 = v5 + v12;
                if (dat_264a0) { // branch-flip
                  if (a2 <= v4)
                    v4 = a2;
                  if (v12 < v4) {
                    do {
                      v12 += 1;
                      putc_unlocked(0x20,v3);
                    } while (v12 != v4);
                  }
                }
                else if (v4 < a2) {
                  putc_unlocked(9,v3);
                  v12 = v4;
                }
              }
              v10 += v5;
              break;
            case 2:
              return v12;
            case 3:
            case 4:
              goto label_ceb3;
            case 5:
              v10 = 0;
              putc_unlocked(0xd,v3);
              sub_cd10(0,a1);
              v12 = 0;
              break;
            default:
              goto label_ce80;
            case 0x18:
            case 0x19:
            case 0x1a:
            case 0x1b:
            case 0x1d:
            case 0x1e:
            case 0x1f:
            case 0x20:
            case 0x21:
            case 0x22:
            case 0x23:
            case 0x24:
            case 0x25:
            case 0x26:
            case 0x27:
            case 0x28:
            case 0x29:
            case 0x2a:
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
            case 0x39:
            case 0x3a:
            case 0x3b:
            case 0x3c:
            case 0x3d:
            case 0x3e:
            case 0x3f:
            case 0x40:
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
              goto label_ce2f;
            
          }
        }
        else {
label_ce80:
          v4 = sub_16640(&v9,v6,(long)v2 - (long)v6,&v8);
          if (0xfffffffffffffffd <= v4 - 1) { // branch-flip
label_ceb3:
            if (v10 < a2)
              putc_unlocked(v11,v3);
          }
          else {
            v11 = wcwidth(v9);
            if (1 <= v11)
              v10 = (long)v11 + v10;
            if (v10 <= a2) {
              fwrite_unlocked(v6,1,v4,stdout);
              v12 = v10;
            }
            v7 = &v6[v4];
          }
        }
label_cec0:
        v6 = v7;
        if (v2 <= v7)
          return v12;
      }
      if (0x1e <= (unsigned char)(v1 + 0x9fU)) goto label_ce80;
label_ce2f:
      v4 = v10 + 1;
      if (v10 < a2) {
        putc_unlocked(v11,v3);
        v10 = v4;
        v12 = v4;
        goto label_cec0;
      }
      v6 = v7;
      v10 = v4;
    } while (v7 < v2);
  }
  return v12;
}

// Function: sub_d0a0 @ 0xd0a0
void sub_d0a0(struct_10 *a0,char a1,struct_10 *a2)
{
  bool v1;
  bool v2;
  FILE *v3;
  unsigned long v4;
  unsigned long v5;
  bool v6;
  unsigned long v7;
  int v8; // edi
  bool v9;
  
  v5 = dat_26430;
  v4 = dat_26428;
  v3 = dat_26188;
  if (a1 != '<') { // branch-flip
    if (a1 == '>') {
      sub_eb70(1);
      goto label_d202;
    }
    v1 = 0;
    if (a0) goto label_d0ec;
    v9 = 0;
    v7 = 0;
    v6 = 0;
    v2 = v1;
label_d10b:
    v1 = v6;
    if (a1 == ' ') goto label_d111;
    v7 = sub_cd10(v7,(unsigned long)(v5 + -1 + v4) >> 1) + 1;
    v1 = v2;
    if (a1 != '|') goto label_d224;
    v8 = 0x7c;
    if ((a2->field_0x8[-1] == '\n') != v9)
      v8 = (-(unsigned int)(v9 == 0) & 0x2d) + 0x2f;
  }
  else {
    sub_eb70(2);
label_d202:
    v1 = 1;
    if (a0) {
label_d0ec:
      v9 = a0->field_0x8[-1] == '\n';
      v7 = sub_cda0(a0,0,v5);
      v6 = v1;
      v2 = v1;
      goto label_d10b;
    }
    v9 = 0;
    v7 = sub_cd10(0,(unsigned long)(v5 + -1 + v4) >> 1) + 1;
label_d224:
    v8 = (int)a1;
  }
  putc_unlocked(v8,v3);
label_d111:
  if ((a2) && (v9 = (bool)(v9 | a2->field_0x8[-1] == '\n'), *a2->field_0x0 != '\n'))
    sub_cda0(a2,sub_cd10(v7,v4),v5);
  if (v9)
    putc_unlocked(10,v3);
  if (v1) {
    sub_eb70(3); // tail-call
    return;
  }
}

// Function: sub_d260 @ 0xd260
void sub_d260(long a0,long a1) // return-dupe x3
{
  long v1;
  long v2;
  long v3;
  bool v4; // zf
  bool v5; // zf
  
  v3 = dat_26730;
  v2 = dat_26728;
  if (dat_26438) {
    dat_26730 = a0;
    dat_26728 = a1;
    return;
  }
  v4 = dat_26730 != a0;
  v5 = dat_26728 != a1;
  if ((!v4) && (!v5)) {
    dat_26728 = a1;
    dat_26730 = a0;
    return;
  }
  if (dat_2643a)
    __fprintf_chk(dat_26188,1,"i%ld,%ld\n",a0 - dat_26730,a1 - dat_26728);
  if (dat_26439) goto label_d37b;
  if ((v4) && (v5)) {
    while( true ) {
      v2 += 1;
      v3 += 1;
      sub_d0a0((struct_10 *)(dat_26258 + -8 + v3 * 8),0x20,(struct_10 *)(dat_26388 + -8 + v2 * 8));
      if (a0 == v3) break;
      if (a1 == v2) goto label_d37b;
    }
  }
  if (a1 == v2) goto label_d37b;
  do {
    v1 = v2 * 8;
    v2 += 1;
    sub_d0a0(NULL,0x29,(struct_10 *)(dat_26388 + v1));
  } while (a1 != v2);
  if (a0 == v3) {
    dat_26728 = a1;
    dat_26730 = a0;
    return;
  }
  do {
    v2 = v3 * 8;
    v3 += 1;
    sub_d0a0((struct_10 *)(dat_26258 + v2),0x28,NULL);
label_d37b:
  } while (a0 != v3);
  dat_26728 = a1;
  dat_26730 = a0;
}

// Function: sub_d3c0 @ 0xd3c0
void sub_d3c0(long *a0) // return-dupe x2, return-dupe
{
  unsigned int v1;
  long v2;
  long v3;
  long v4;
  long v5; // stack - 0x30
  long v6; // stack - 0x38
  long v7; // stack - 0x40
  long v8; // stack - 0x28
  long v9;
  
  v1 = sub_f1a0(a0,&v7,&v6,&v5,&v8);
  if (!v1)
    return;
  sub_d260(v7,v5);
  if (dat_2643a)
    __fprintf_chk(dat_26188,1,"c%ld,%ld\n",(v6 - v7) + 1,(v8 - v5) + 1);
  if (v1 == 3) {
    v3 = v8;
    v2 = v6;
    v4 = v7;
    v9 = v5;
    if (v7 <= v6) {
      do {
        v3 = v8;
        v2 = v6;
        v7 = v4;
        v5 = v9;
        if (v9 > v8) goto label_d52f;
        v3 = v9 * 8;
        v9 += 1;
        v2 = v4 * 8;
        v4 += 1;
        sub_d0a0((struct_10 *)(dat_26258 + v2),0x7c,(struct_10 *)(dat_26388 + v3));
      } while (v6 >= v4);
      v3 = v8;
      v2 = v6;
      v7 = v4;
      v5 = v9;
    }
label_d52f:
    v1 = (unsigned int)(v7 <= v2) + (unsigned int)(v5 <= v3) * 2;
    dat_26728 = v5;
    dat_26730 = v7;
  }
  if (v1 & 2) {
    v3 = v5;
    v2 = v5;
    if (v5 <= v8) {
      do {
        v3 = v2 + 1;
        sub_d0a0(NULL,0x3e,(struct_10 *)(dat_26388 + v2 * 8));
        v2 = v3;
      } while (v3 <= v8);
    }
    dat_26728 = v3;
  }
  if (!(v1 & 1))
    return;
  v3 = v7;
  if (v6 >= v7) {
    do {
      v2 = v3 * 8;
      v3 += 1;
      sub_d0a0((struct_10 *)(dat_26258 + v2),0x3c,NULL);
    } while (v3 <= v6);
    dat_26730 = v3;
    return;
  }
  dat_26730 = v7;
}

// Function: sub_d5b0 @ 0xd5b0
void sub_d5b0(long a0)
{
  sub_e2b0();
  dat_26728 = -dat_26288;
  dat_26730 = dat_26728;
  sub_eb10(a0,sub_eaf0,sub_d3c0);
  sub_d260(dat_26270,dat_263a0); // tail-call
}

// Function: sub_d600 @ 0xd600
void sub_d600(unsigned int a0) // return-dupe
{
  if (dat_26774)
    return;
  dat_26774 = a0;
}

// Function: sub_d620 @ 0xd620
void sub_d620(void) // return-dupe
{
  if (dat_26774)
    return;
  dat_26770 += 1;
}

// Function: sub_d640 @ 0xd640
unsigned int sub_d640(long *a0,long *a1,unsigned int a2,long *a3)
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
  if (v5 == '\\') goto label_d6a6;
label_d67a:
  if (v13 || SBORROW1(v5,'\\') != v6 < '\0') {
    if (v5 != '=') { // branch-flip
      if ((v5 <= '=') && ((!v5 || (v5 == ':')))) {
        v2 = 1;
label_d6de:
        *a0 = v11;
        *a1 = (long)v3;
        *a3 = v10;
        return v2;
      }
    }
    else {
      v2 = a2;
      if ((char)a2) goto label_d6de;
    }
  }
  else if (v5 == '^') {
    v1 = v3[1];
    if ((unsigned char)(v1 - 0x40) <= 0x3e) {
      v3 = &v3[2];
      v9 += 1;
      v8 = v7 + 1;
      *(unsigned char *)(v7 + -1) = v1 & 0x1f;
      goto label_d696;
    }
    v3 = &v3[1];
    if (v1 == 0x3f) {
      v9 += 1;
      v8 = v7 + 1;
      *(char *)(v7 + -1) = 0x7f;
      goto label_d696;
    }
    v2 = 0;
    goto label_d6de;
  }
  v3 = &v3[1];
  v8 = v7;
label_d68b:
  *(char *)(v8 + -1) = v5;
  v9 += 1;
  v8 += 1;
label_d696:
  do {
    v5 = *v3;
    v11 = v8 + -1;
    v10 = v9 + -1;
    v6 = v5 + '\xa4';
    v13 = v6 == '\0';
    v4 = v3;
    v7 = v8;
    if (!v13) goto label_d67a;
label_d6a6:
    v6 = v4[1];
    v3 = &v4[2];
    if (!v6) {
      v2 = 0;
      goto label_d6de;
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
        if (8 <= (unsigned char)(v6 - 0x30U)) goto label_d68b;
        do {
          v3 = &v3[1];
          v5 = v6 + '\xd0' + v5 * '\b';
          v6 = *v3;
        } while ((unsigned char)(v6 - 0x30U) <= 7);
        *(char *)(v8 + -1) = v5;
        v9 += 1;
        v8 += 1;
        goto label_d696;
      default:
        goto label_d720;
      case 0xf:
        v6 = '\x7f';
        break;
      case 0x28:
      case 0x48:
        goto label_d7b7;
      case 0x2f:
        v6 = ' ';
        break;
      case 0x31:
        v6 = '\a';
        goto label_d720;
      case 0x32:
        v6 = '\b';
        goto label_d720;
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
label_d720:
      
    }
    *(char *)(v8 + -1) = v6;
    v9 += 1;
    v8 += 1;
  } while( true );
label_d7b7:
  v6 = v4[2];
  v12 = &v4[3];
  v5 = '\0';
  v3 = &v4[2];
  if ('F' < v6) goto label_d802;
  do {
    if ('A' <= v6) { // branch-flip
      v5 = v6 + '\xc9' + v5 * '\x10';
      v3 = v12;
    }
    else {
      if (10 <= (unsigned char)(v6 - 0x30U)) goto label_d68b;
      v5 = v6 + '\xd0' + v5 * '\x10';
      v3 = v12;
    }
    while( true ) {
      v6 = *v3;
      v12 = &v3[1];
      if (v6 <= 'F') break;
label_d802:
      if (6 <= (unsigned char)(v6 + 0x9fU)) goto label_d68b;
      v5 = v6 + '\xa9' + v5 * '\x10';
      v3 = v12;
    }
  } while( true );
}

// Function: sub_d8b0 @ 0xd8b0
int sub_d8b0(int a0)
{
  char v1; // dil
  
  v1 = (char)a0;
  if ('#' <= v1) { // branch-flip
    if (v1 == '\\')
      return a0;
  }
  else if ('\a' <= v1) {
    switch(a0 - 7U & 0xff) {
      case 0:
        a0 = 0x61;
        break;
      case 1:
        a0 = 0x62;
        break;
      case 2:
        return 0x74;
      case 3:
        return 0x6e;
      case 4:
        return 0x76;
      case 5:
        return 0x66;
      case 6:
        return 0x72;
      default:
        goto label_d8f0;
      case 0x1b:
        break;
      
    }
    return a0;
  }
label_d8f0:
  return CONCAT31((undefined3)((unsigned int)a0 >> 8),v1 <= '\x1f');
}

// Function: sub_d970 @ 0xd970
unsigned char * sub_d970(unsigned char *a0)
{
  long v1;
  int v10; // edi
  char v11; // r9b
  unsigned long v12; // r9
  char v2; // al
  unsigned char v3;
  unsigned char *v4;
  unsigned char v5; // cl
  long v6;
  unsigned long v7; // rcx
  unsigned char *v8;
  unsigned char *v9; // rsi
  
  v10 = (int)(char)*a0;
  if (*a0) {
    v12 = 0;
    v6 = 0;
    v8 = a0;
    do {
      while ((char)v10 != ' ') {
        v2 = sub_d8b0(v10);
        v11 = (char)v12;
        if (v2) {
          v1 = v6 + 3;
          v6 += 1;
          if (v2 == '\x01')
            v6 = v1;
        }
        v4 = &v8[1];
        v10 = (int)(char)*v4;
        v8 = &v8[1];
        if (!*v4) goto label_d9cd;
      }
      v4 = &v8[1];
      v10 = (int)(char)*v4;
      v8 = &v8[1];
      v12 = 1;
      v11 = 1;
    } while (*v4);
label_d9cd:
    if ((v6) || (v11)) {
      v4 = (unsigned char *)sub_15f50(&v8[(v6 + 3) - (long)a0]);
      *v4 = 0x22;
      v3 = *a0;
      v8 = &v4[1];
      while (v3) {
        v7 = (unsigned long)v3;
        v3 = sub_d8b0((int)(char)v3);
        v5 = (unsigned char)v7;
        if (v3) { // branch-flip
          if (v3 != 1) { // branch-flip
            *v8 = 0x5c;
            v9 = &v8[2];
            v8[1] = v3;
          }
          else {
            *v8 = 0x5c;
            v9 = &v8[4];
            v8[1] = ((unsigned char)((v7 & 0xffffffff) >> 6) & 3) + 0x30;
            v8[3] = (v5 & 7) + 0x30;
            v8[2] = ((char)v5 >> 3 & 7U) + 0x30;
          }
        }
        else {
          *v8 = v5;
          v9 = &v8[1];
        }
        v8 = &a0[1];
        a0 = &a0[1];
        v3 = *v8;
        v8 = v9;
      }
      v8[0] = 0x22;
      v8[1] = 0;
      return v4;
    }
  }
  return a0;
}

// Function: sub_da80 @ 0xda80
void sub_da80(void)
{
  char v1;
  void *v2;
  void *v3;
  long v4; // rcx
  unsigned short v5; // stack - 0x43
  char *v6; // stack - 0x58
  void *v7; // stack - 0x50
  char *v8;
  char v9; // stack - 0x41
  
  v8 = dat_26748;
  v6 = dat_26748;
  if ((!dat_26748) || (!*dat_26748))
    return;
  v9 = 0;
  v5 = 0x3f3f;
  v2 = (void *)sub_162e0(dat_26748);
  v7 = v2;
label_db0d:
  do {
    v1 = *v8;
    if (v1 != '*') { // branch-flip
      if (v1 == ':') {
        v8 = &v8[1];
        v6 = v8;
        goto label_db0d;
      }
      if (!v1)
        return;
      v5 = CONCAT11(v5._1_1_,v1);
      v6 = &v8[1];
      if (!v8[1]) goto label_dbda;
      v5 = CONCAT11(v8[1],v1);
      v6 = &v8[3];
      if (v8[2] != '=') goto label_dbda;
      v4 = 0;
      v8 = "lc";
      while (strcmp((char *)&v5,v8)) {
        v4 += 1;
        v8 = *(char **)(v4 * 8 + 0x25900);
        if (!v8) goto label_dbb6;
      }
      v4 = (long)(int)v4 * 0x10;
      *(void **)(v4 + 0x26028) = v7;
      if (!sub_d640(&v7,&v6,0,(long *)(v4 + 0x26020))) {
label_dbb6:
        error(0,0,dcgettext(NULL,"unrecognized prefix: %s",5),(char *)&v5);
label_dbda:
        error(0,0,dcgettext(NULL,"unparsable value for --palette",5));
        free(v2);
        v2 = dat_26750;
        while (v2) {
          v3 = *(void **)((long)v2 + 0x20);
          free(v2);
          v2 = v3;
        }
        dat_26758 = 0;
        return;
      }
    }
    else {
      v3 = (void *)sub_15f50(0x28);
      *(void **)((long)v3 + 0x20) = dat_26750;
      *(void **)((long)v3 + 8) = v7;
      dat_26750 = v3;
      v6 = &v8[1];
      if ((!sub_d640(&v7,&v6,1,v3)) || (v8 = &v6[1], v1 = *v6, v6 = v8, v1 != '=')) goto label_dbda;
      *(void **)((long)v3 + 0x18) = v7;
      if (!sub_d640(&v7,&v6,0,(long *)((long)v3 + 0x10))) goto label_dbda;
    }
    v8 = v6;
  } while( true );
}

// Function: sub_dcf0 @ 0xdcf0
void sub_dcf0(void)
{
  int *v1; // rbx
  void *v2 [17]; // stack - 0xd8
  int *v3; // rbp
  int v4;
  undefined4 v5; // stack - 0x50
  
  v4 = 0x14;
  v1 = (int *)0x1d524;
  v3 = (int *)0x1d524;
  sigemptyset((sigset_t *)0x26780);
  while( true ) {
    sigaction(v4,NULL,(sigaction *)v2);
    if (v2[0] != (void *)0x1)
      sigaddset((sigset_t *)0x26780,v4);
    if (v3 == (int *)0x1d550) break;
    v4 = *v3;
    v3 = &v3[1];
  }
  v5 = 0x10000000;
  v4 = 0x14;
  v2[1] = (void *)dat_26780;
  v2[2] = (void *)dat_26788;
  v2[3] = (void *)dat_26790;
  v2[4] = (void *)dat_26798;
  v2[5] = (void *)dat_267a0;
  v2[6] = (void *)dat_267a8;
  v2[7] = (void *)dat_267b0;
  v2[8] = (void *)dat_267b8;
  v2[9] = (void *)dat_267c0;
  v2[10] = (void *)dat_267c8;
  v2[0xb] = (void *)dat_267d0;
  v2[0xc] = (void *)dat_267d8;
  v2[0xd] = (void *)dat_267e0;
  v2[0xe] = (void *)dat_267e8;
  v2[0xf] = (void *)dat_267f0;
  v2[0x10] = (void *)dat_267f8;
  while( true ) {
    if (sigismember((void *)0x26780,v4)) {
      v2[0] = sub_d600;
      if (v4 == 0x14)
        v2[0] = sub_d620;
      sigaction(v4,(sigaction *)v2,NULL);
    }
    if (v1 == (int *)0x1d550) break;
    v4 = *v1;
    v1 = &v1[1];
  }
}

// Function: sub_de60 @ 0xde60
void sub_de60(bool a0)
{
  int v1; // eax
  bool v2;
  
  if ((!dat_26188) || (!dat_2657c))
    return;
  if (dat_26180) // branch-flip
    v2 = 1;
  else {
    if (a0) {
      if (dat_2657c != 2) {
        dat_26758 = 0;
        return;
      }
      dat_26758 = 1;
      sub_da80(); // tail-call
      return;
    }
    v1 = fileno(dat_26188);
    v2 = isatty(v1) != 0;
  }
  if ((dat_2657c == 2) || ((dat_2657c == 1 && (v2)))) {
    dat_26758 = 1;
    sub_da80();
  }
  else {
    dat_26758 = 0;
  }
  if (!v2)
    return;
  sub_dcf0(); // tail-call
}

// Function: sub_df10 @ 0xdf10
void sub_df10(unsigned long a0,void *a1)
{
  fwrite_unlocked(a1,a0,1,dat_26188); // tail-call
}

// Function: sub_df30 @ 0xdf30
void sub_df30(char *a0)
{
  error(0,*__errno_location(),"%s",a0); // tail-call
}

// Function: sub_df60 @ 0xdf60
void sub_df60(char *a0,long a1,unsigned long a2,char *a3,char *a4)
{
  unsigned long *v1;
  unsigned long v10;
  char *v11; // stack - 0x70
  char *v12; // stack - 0x68
  long v2; // rax
  unsigned long *v3; // rax
  void *v4; // rax
  unsigned long *v5;
  long v6;
  unsigned long v7 [5];
  long v8 [2]; // stack - 0x80
  char *v9;
  
  if (dat_26480) {
    v10 = 8;
    if (!a3)
      a3 = "";
    if (!a4)
      a4 = "";
    v6 = 0;
    v9 = a0;
    v8[0] = a1;
    v8[1] = a2;
    v11 = a3;
    v12 = a4;
    while( true ) {
      v2 = strlen(v9) + 1;
      *(long *)((long)v7 + v6) = v2;
      v10 += v2;
      if (v6 + 8 == 0x28) break;
      v9 = *(char **)((long)v8 + v6);
      v6 += 8;
    }
    v3 = (unsigned long *)sub_15f50(v10);
    v5 = &v3[1];
    v6 = 0;
    while( true ) {
      v10 = *(unsigned long *)((long)v7 + v6);
      v4 = memcpy(v5,a0,v10);
      v1 = dat_260a0;
      v5 = (unsigned long *)((long)v4 + v10);
      if (v6 + 8 == 0x28) break;
      a0 = *(char **)((long)v8 + v6);
      v6 += 8;
    }
    dat_260a0 = v3;
    *v1 = v3;
    *v3 = 0;
    return;
  }
  if (dat_2643a)
    putchar_unlocked(0x20);
  __printf_chk(1,dcgettext(NULL,a0,5),a1,a2,a3,a4); // tail-call
}

// Function: sub_e0e0 @ 0xe0e0
void sub_e0e0(char *a0)
{
  sub_df60(a0); // tail-call
}

// Function: sub_e0f0 @ 0xe0f0
void sub_e0f0(void) // return-dupe
{
  long *v1;
  long *v10;
  long *v11;
  unsigned long v12; // stack - 0x60
  unsigned long v13; // stack - 0x58
  unsigned long v14; // stack - 0x50
  unsigned long v2;
  unsigned long v3;
  unsigned long v4;
  long v5;
  unsigned long v6; // rax
  long *v7;
  long v8;
  long v9; // stack - 0x48
  
  if (!dat_26800)
    return;
  v10 = dat_26800;
  do {
    v1 = (long *)*v10;
    v7 = &v10[1];
    v11 = &v8;
    do {
      v11 = &v11[1];
      v6 = strlen((char *)v7);
      v5 = v9;
      v4 = v14;
      v3 = v13;
      v2 = v12;
      v7 = (long *)((long)v7 + v6 + 1);
      *v11 = (long)v7;
    } while (&v9 != v11);
    __printf_chk(1,dcgettext(NULL,(char *)&v10[1],5),v2,v3,v4,v5);
    free(v10);
    v10 = v1;
  } while (v1);
}

// Function: sub_e1f0 @ 0xe1f0
void sub_e1f0(char *a0)
{
  int v1;
  
  v1 = *__errno_location();
  sub_e0f0();
  error(2,v1,"%s",a0);
}

// Function: sub_e230 @ 0xe230
void sub_e230(char *a0)
{
  sub_e0f0();
  error(2,0,"%s",dcgettext(NULL,a0,5));
}

// Function: sub_e270 @ 0xe270
void sub_e270(unsigned long a0)
{
  dat_26748 = a0;
}

// Function: sub_e280 @ 0xe280
void sub_e280(unsigned long a0,unsigned long a1,char a2)
{
  dat_26768 = a0;
  dat_26760 = a1;
  dat_26759 = a2;
  dat_26188 = 0;
}

// Function: sub_e2b0 @ 0xe2b0
void sub_e2b0(void) // return-dupe x2
{
  char *v1; // rax
  void *v2; // stack - 0x58
  int v3; // stack - 0x60
  char *v4; // stack - 0x48
  int v5; // stack - 0x5c
  void *v6; // stack - 0x50
  char *v7; // stack - 0x40
  char *v8; // stack - 0x38
  unsigned long v9; // stack - 0x30
  
  if (dat_26188)
    return;
  v2 = (void *)sub_d970(dat_26768);
  v6 = (void *)sub_d970(dat_26760);
  v1 = (char *)sub_16460("diff%s %s %s",dat_26420,v2,v6);
  if (dat_26480) { // branch-flip
    if (fflush_unlocked(stdout))
      sub_e1f0(dcgettext(NULL,"write failed",5)); // no-return
    v8 = v1;
    v4 = "/usr/bin/pr";
    v7 = "-h";
    v9 = 0;
    if (pipe(&v3))
      sub_e1f0("pipe"); // no-return
    dat_26740 = fork();
    if (dat_26740 < 0)
      sub_e1f0("fork"); // no-return
    if (!dat_26740) {
      close(v5);
      if (v3) {
        if (dup2(v3,0) < 0)
          sub_e1f0("dup2"); // no-return
        close(v3);
      }
      execv("/usr/bin/pr",&v4);
      _exit((*__errno_location() == 2) + 0x7e); // no-return
    }
    close(v3);
    dat_26188 = fdopen(v5,"w");
    if (!dat_26188)
      sub_e1f0("fdopen"); // no-return
    sub_de60(1);
  }
  else {
    dat_26188 = stdout;
    sub_de60(0);
    if (dat_26759)
      __printf_chk(1,"%s\n",v1);
  }
  free(v1);
  if (dat_26580 != 2) { // branch-flip
    if (dat_26580 == 3)
      sub_87e0(0x261a0,&v2,1);
  }
  else {
    sub_87e0(0x261a0,&v2,0);
  }
  if (v2 != dat_26768)
    free(v2);
  if (v6 == dat_26760)
    return;
  free(v6);
}

// Function: sub_e550 @ 0xe550
void sub_e550(void)
{
  unsigned char v1; // ah
  unsigned int v2;
  unsigned int v3; // stack - 0x14
  char *v4; // rsi
  
  if ((!dat_26188) || (dat_26188 == stdout)) {
    dat_26188 = NULL;
    return;
  }
  if (ferror_unlocked(dat_26188))
    sub_e230("write failed"); // no-return
  if (!fclose(dat_26188)) {
    if (waitpid(dat_26740,(int *)&v3,0) < 0)
      sub_e1f0("waitpid"); // no-return
    if (v3 & 0x7f) { // branch-flip
      v4 = "subsidiary program \'%s\' failed";
      v2 = 0x7fffffff;
    }
    else {
      v1 = (unsigned char)(v3 >> 8);
      if (!v1) {
        dat_26188 = NULL;
        return;
      }
      v4 = "subsidiary program \'%s\' could not be invoked";
      v2 = (unsigned int)v1;
      if ((v1 != 0x7e) && (v4 = "subsidiary program \'%s\' failed (exit status %d)", v2 = (unsigned int)v1, v1 == 0x7f))
        v4 = "subsidiary program \'%s\' not found";
    }
    error(2,0,dcgettext(NULL,v4,5),"/usr/bin/pr",(unsigned long)v2);
    return;
  }
  sub_e1f0(dcgettext(NULL,"write failed",5)); // no-return
}

// Function: sub_e690 @ 0xe690
unsigned long sub_e690(unsigned char *a0,unsigned char *a1)
{
  unsigned char *v1;
  unsigned char v10;
  char v11; // bl
  unsigned long v12;
  unsigned char *v13;
  unsigned char *v14;
  unsigned long v15;
  unsigned char *v16;
  unsigned short v17; // r11w
  unsigned char *v18;
  unsigned long v19; // r8
  unsigned char v2;
  unsigned long v3;
  char v4;
  unsigned int v5;
  unsigned int v6; // eax
  long v7;
  unsigned char v8;
  bool v9;
  
  v5 = dat_26558;
  v4 = dat_26552;
  v3 = dat_26498;
  v19 = 0;
  v6 = dat_26558 - 2;
  v16 = a1;
  v18 = a0;
label_e6e0:
  do {
    v8 = *v18;
    v10 = *v16;
    v12 = (unsigned long)v10;
    v14 = &v18[1];
    v13 = &v16[1];
    if (v8 == v10) goto label_e7ca;
    if (v5 != 4) {
      if (5 <= v5) { // branch-flip
        if (v5 == 5) {
          v7 = *(long *)__ctype_b_loc();
          while ((*(unsigned char *)(v7 + 1 + (unsigned long)v8 * 2) & 0x20 && (v8 != 10))) {
            v16 = &v14[1];
            v8 = *v14;
            v14 = v16;
          }
          v10 = *(unsigned char *)(v7 + 1 + (unsigned long)v10 * 2);
          while ((v10 & 0x20 && ((char)v12 != '\n'))) {
            v12 = (unsigned long)*v13;
            v13 = &v13[1];
            v10 = *(unsigned char *)(v7 + 1 + v12 * 2);
          }
        }
      }
      else if (v5 != 1) { // branch-flip
        if (v6 <= 1) {
          v7 = *(long *)__ctype_b_loc();
          if ((*(unsigned char *)(v7 + 1 + (unsigned long)v8 * 2) & 0x20) && (*(unsigned char *)(v7 + 1 + (unsigned long)v10 * 2) & 0x20)) {
            if (v8 != 10) {
              v2 = v18[1];
              v18 = v14;
              while (v2 != 10) {
                if (!(*(unsigned char *)(v7 + 1 + (unsigned long)v2 * 2) & 0x20)) goto label_e798;
                v1 = &v18[1];
                v18 = &v18[1];
                v2 = *v1;
              }
            }
            if ((v10 == 10) || (v10 = v16[1], v10 == 10))
              return 0;
            v16 = v13;
            while (*(unsigned char *)(v7 + 1 + (unsigned long)v10 * 2) & 0x20) {
              v10 = v16[1];
              v16 = &v16[1];
              if (v10 == 10)
                return 0;
            }
          }
          else if (v5 != 2) goto label_e880;
        }
      }
      else {
label_e880:
        if ((v8 == 0x20) && (v10 == 9)) {
label_e8a1:
          v15 = v19;
          do {
            if (v8 != 0x20) { // branch-flip
              if (v8 != 9) goto label_e8fc;
              v15 = (v15 + v3) - v15 % v3;
            }
            else {
              v15 += 1;
            }
            v8 = *v14;
            v14 = &v14[1];
          } while( true );
        }
        if ((v8 == 9) && (v10 == 0x20)) {
          v8 = 9;
          goto label_e8a1;
        }
      }
      goto label_e798;
    }
    v7 = *(long *)__ctype_b_loc();
    v15 = (unsigned long)v8;
    v17 = *(unsigned short *)(v7 + (unsigned long)v10 * 2) & 0x2000;
    if (*(unsigned char *)(v7 + 1 + v15 * 2) & 0x20) { // branch-flip
      while ((char)v15 != '\n') {
        v15 = (unsigned long)*v14;
        v16 = &v14[1];
        if (!(*(unsigned char *)(v7 + 1 + v15 * 2) & 0x20)) {
          v15 = 0x20;
          break;
        }
        v14 = v16;
      }
      v8 = (unsigned char)v15;
      if (v17) goto label_ea1f;
label_e9f5:
      if ((unsigned char)v12 == v8) goto label_e798;
    }
    else if (v17) {
label_ea1f:
      while (v8 = (unsigned char)v15, (char)v12 != '\n') {
        v12 = (unsigned long)*v13;
        v16 = &v13[1];
        if (!(*(unsigned char *)(v7 + 1 + v12 * 2) & 0x20)) {
          v12 = 0x20;
          goto label_e9f5;
        }
        v13 = v16;
      }
      goto label_e798;
    }
    v11 = (char)v12;
    v9 = v11 == ' ' && v8 != 10;
    if (v11 != ' ' || v8 == 10) {
      v9 = v11 != '\n';
      break;
    }
    if (v14 <= &a0[1]) {
      v12 = 0x20;
      break;
    }
    v12 = 0x20;
    v16 = v13;
    v18 = &v14[-1];
  } while (*(unsigned char *)(v7 + 1 + (unsigned long)v14[-2] * 2) & 0x20);
  if ((v8 == 0x20) && (v9)) {
    if (v13 <= &a1[1]) // branch-flip
      v8 = 0x20;
    else {
      v16 = &v13[-1];
      v18 = v14;
      if (*(unsigned char *)(v7 + 1 + (unsigned long)v13[-2] * 2) & 0x20) goto label_e6e0;
    }
  }
label_e798:
  if (v4) {
    v7 = *(long *)__ctype_tolower_loc();
    v8 = *(unsigned char *)(v7 + (unsigned long)v8 * 4);
    v12 = (unsigned long)*(unsigned char *)(v7 + v12 * 4);
  }
  v10 = (unsigned char)v12;
  if (v8 != v10)
    return 1;
label_e7ca:
  if (v10 == 10)
    return 0;
  v7 = 1;
  if (v10 == 9)
    v7 = v3 - v19 % v3;
  v19 += v7;
  v16 = v13;
  v18 = v14;
  goto label_e6e0;
label_e8fc:
  if ((char)v12 != ' ') { // branch-flip
    if ((char)v12 != '\t') goto label_ea30;
    v19 = (v3 + v19) - v19 % v3;
  }
  else {
    v19 += 1;
  }
  v12 = (unsigned long)*v13;
  v13 = &v13[1];
  goto label_e8fc;
label_ea30:
  if (v15 != v19)
    return 1;
  goto label_e798;
}

// Function: sub_eaf0 @ 0xeaf0
unsigned long sub_eaf0(unsigned long a0)
{
  return a0;
}

// Function: sub_eb00 @ 0xeb00
unsigned long sub_eb00(unsigned long a0)
{
  return sub_eaf0(a0); // tail-call
}

// Function: sub_eb10 @ 0xeb10
void sub_eb10(long a0,void *a1,void *a2)
{
  long v1;
  long *v2; // rax
  
  if (a0) {
    do {
      v2 = (long *)(*a1)(a0);
      v1 = *v2;
      *v2 = 0;
      (*a2)(a0);
      *v2 = v1;
      a0 = v1;
    } while (v1);
    return;
  }
}

// Function: sub_eb70 @ 0xeb70
void sub_eb70(int a0)
{
  if (a0 != 3)
    sub_ec70();
  if ((dat_26758) && (dat_26010 != a0)) {
    sub_df10(dat_26020,dat_26028);
    switch(a0) {
      case 0:
        sub_df10(dat_26060,dat_26068);
        break;
      case 1:
        sub_df10(dat_26070,dat_26078);
        break;
      case 2:
        sub_df10(dat_26080,dat_26088);
        break;
      case 3:
        sub_df10(dat_26050,dat_26058);
        break;
      case 4:
        sub_df10(dat_26090,dat_26098);
        break;
      default:
        abort(); // no-return
      
    }
    sub_df10(dat_26030,dat_26038);
    dat_26010 = a0;
    return;
  }
}

// Function: sub_ec70 @ 0xec70
void sub_ec70(void)
{
  sigset_t v1;
  int v2;
  
  while ((dat_26774 || (dat_26770))) {
    sub_eb70(3);
    fflush_unlocked(stdout);
    sigprocmask(0,(sigset_t *)0x26780,&v1);
    v2 = dat_26774;
    if (dat_26770) { // branch-flip
      dat_26770 -= 1;
      v2 = 0x13;
    }
    else {
      signal(dat_26774,0);
    }
    raise(v2);
    sigprocmask(2,&v1,NULL);
  }
}

// Function: sub_ed50 @ 0xed50
void sub_ed50(unsigned char *a0,unsigned char *a1,char *a2,unsigned long a3) // return-dupe x2
{
  unsigned char v1;
  FILE *v2;
  unsigned long v3; // rax
  long v4; // rcx
  unsigned long v5;
  long v6; // r14
  unsigned long v7;
  
  v5 = dat_26498;
  v2 = dat_26188;
  if (!dat_264a0) {
    v5 = (long)a1 - (long)a0;
    dat_264a0 = 0;
    while( true ) {
      if (!v5)
        return;
      v7 = 0x400;
      if (v5 <= 0x400)
        v7 = v5;
      v3 = fwrite_unlocked(a0,1,v7,dat_26188);
      if (v3 < v7) break;
      v5 -= v3;
      a0 = &a0[v3];
      sub_ec70();
    }
    return;
  }
  v7 = 0;
  v6 = 1;
  if (a1 <= a0)
    return;
  do {
    v1 = *a0;
    a0 = &a0[1];
    if (v1 != 9) { // branch-flip
      if (v1 != 0xd) {
        if (v1 != 8) { // branch-flip
          v7 += *(unsigned short *)(*(long *)__ctype_b_loc() + (unsigned long)v1 * 2) >> 0xe & 1;
          putc_unlocked((unsigned int)v1,v2);
        }
        else {
          if (!v7) goto label_ef10;
          v7 -= 1;
          putc_unlocked(8,v2);
        }
        goto label_ee4f;
      }
      putc_unlocked(0xd,v2);
      if ((!a2) || (a1 <= a0)) {
label_ef10:
        v7 = 0;
        goto label_ee4f;
      }
      v7 = 0;
      if (*a0 != 10)
        __fprintf_chk(v2,1,a2,a3);
    }
    else {
      v4 = v5 - v7 % v5;
      v7 += v4;
      do {
        putc_unlocked(0x20,v2);
        v4 -= 1;
      } while (v4);
label_ee4f:
      if (a1 == a0)
        return;
    }
    v6 += 1;
    if (v6 == 0x400) {
      sub_ec70();
      v6 = 0;
    }
  } while( true );
}

// Function: sub_ef20 @ 0xef20
void sub_ef20(char *a0,struct_11 *a1,bool a2)
{
  char v1;
  unsigned char *v2;
  unsigned char *v3;
  FILE *v4;
  char *v5;
  char *v6;
  char *v7; // r13
  
  v4 = dat_26188;
  v2 = a1->field_0x0;
  v3 = a1->field_0x8;
  if (a0) { // branch-flip
    v7 = NULL;
    if (*a0) {
      v7 = "%s\t";
      if (!dat_26492)
        v7 = "%s ";
      v5 = a0;
      v6 = v7;
      if ((dat_26491) && (*v2 == 10)) {
        v6 = "%s";
        v5 = &a0[*a0 == ' '];
      }
      __fprintf_chk(dat_26188,1,v6,v5);
    }
    if (a2) { // branch-flip
      sub_ed50(v2,&v3[-(unsigned long)(v3[-1] == 10)],v7,a0);
      v1 = *a0;
    }
    else {
      sub_ed50(v2,v3,v7,a0);
      v1 = *a0;
    }
    if (!v1)
      return;
  }
  else if (a2) // branch-flip
    sub_ed50(v2,&v3[-(unsigned long)(v3[-1] == 10)],NULL,0);
  else {
    sub_ed50(v2,v3,NULL,0);
  }
  if (v3[-1] == 10)
    return;
  sub_eb70(3);
  __fprintf_chk(v4,1,"\n\\ %s\n",dcgettext(NULL,"No newline at end of file",5)); // tail-call
}

// Function: sub_f0a0 @ 0xf0a0
void sub_f0a0(char *a0,struct_11 *a1)
{
  sub_ef20(a0,a1,0); // tail-call
}

// Function: sub_f0b0 @ 0xf0b0
long sub_f0b0(struct_6 *a0,long a1)
{
  return a1 + a0->field_0xe8 + 1;
}

// Function: sub_f0c0 @ 0xf0c0
void sub_f0c0(struct_6 *a0,long a1,unsigned long a2,long *a3,long *a4)
{
  long v1; // rax
  long v2; // rdx
  
  sub_f0b0(a0,a1 + -1);
  *a3 = v1 + 1;
  *a4 = sub_f0b0(a0,v2 + 1) + -1;
}

// Function: sub_f0f0 @ 0xf0f0
void sub_f0f0(char a0,struct_6 *a1,long a2,unsigned long a3)
{
  long v1; // stack - 0x18
  long v2; // stack - 0x20
  
  sub_f0c0(a1,a2,a3,&v2,&v1);
  if (v2 < v1) {
    __fprintf_chk(dat_26188,1,"%ld%c%ld",v2,a0,v1); // tail-call
    return;
  }
  __fprintf_chk(dat_26188,1,"%ld",v1); // tail-call
}

// Function: sub_f1a0 @ 0xf1a0
unsigned char sub_f1a0(long *a0,long *a1,long *a2,long *a3,long *a4)
{
  long v1;
  long *v10; // rax
  unsigned char *v11;
  unsigned char v12;
  long v13;
  long v14;
  long v15; // r8
  long v16;
  unsigned char v17; // stack - 0x69
  int v18; // stack - 0x64
  long v19; // stack - 0x60
  long v2;
  long v20; // stack - 0x58
  unsigned char v3;
  unsigned char *v4;
  bool v5;
  bool v6;
  long v7;
  long v8;
  unsigned int v9; // eax
  
  v8 = dat_26388;
  v7 = dat_26258;
  v9 = (unsigned int)dat_26554;
  if (dat_26554) { // branch-flip
    if (2 <= dat_26558) { // branch-flip
      v18 = 0;
      v6 = 1;
      v5 = 3 < dat_26558;
    }
    else {
      v18 = 0;
      v6 = 0;
      v5 = 0;
    }
  }
  else {
    v18 = -1;
    v5 = 0;
    v9 = (unsigned int)(dat_264e0 != 0);
    v6 = 0;
  }
  v20 = 0;
  v19 = 0;
  *a1 = a0[3];
  *a3 = a0[4];
  do {
    v13 = a0[3];
    v16 = a0[4];
    v1 = a0[2] + -1 + v13;
    v20 += a0[1];
    v19 += a0[2];
    v2 = v16 + -1 + a0[1];
    v12 = v13 <= v1 & (unsigned char)v9;
    if (v12) { // branch-flip
      do {
        v16 = *(long *)(v7 + 8 + v13 * 8);
        v4 = *(unsigned char **)(v7 + v13 * 8);
        v16 = v16 + -1 + (unsigned long)(*(char *)(v16 + -1) != '\n');
        v15 = v16 - (long)v4;
        v14 = v15;
        if ((v6) && (v3 = *v4, v3 != 10)) {
          v10 = __ctype_b_loc();
          v11 = v4;
          do {
            if (!(*(unsigned char *)(*v10 + 1 + (unsigned long)v3 * 2) & 0x20)) {
              v14 = v16 - (long)v11;
              if (!v5)
                v14 = v15;
              goto label_f2e2;
            }
            v3 = v11[1];
            v11 = &v11[1];
          } while (v3 != 10);
          v14 = v16 - (long)v11;
        }
label_f2e2:
        if (v18 != v14) { // branch-flip
          if (!dat_264e0) {
            v9 = 0;
            goto label_f2fb;
          }
          v9 = (unsigned int)~re_search((re_pattern_buffer *)0x264c0,(char *)v4,(int)v15,0,(int)v15,NULL) >> 0x1f;
        }
        else {
          v9 = (unsigned int)v12;
        }
        v13 += 1;
      } while ((v13 <= v1) && ((unsigned char)v9));
      v16 = a0[4];
      v17 = v16 <= v2 & (unsigned char)v9;
    }
    else {
      v17 = v16 <= v2 & (unsigned char)v9;
    }
    if (v17) {
      do {
        v13 = *(long *)(v8 + 8 + v16 * 8);
        v4 = *(unsigned char **)(v8 + v16 * 8);
        v13 = v13 + -1 + (unsigned long)(*(char *)(v13 + -1) != '\n');
        v15 = v13 - (long)v4;
        v14 = v15;
        if ((v6) && (v12 = *v4, v12 != 10)) {
          v10 = __ctype_b_loc();
          v11 = v4;
          do {
            if (!(*(unsigned char *)(*v10 + 1 + (unsigned long)v12 * 2) & 0x20)) {
              v14 = v13 - (long)v11;
              if (!v5)
                v14 = v15;
              goto label_f465;
            }
            v12 = v11[1];
            v11 = &v11[1];
          } while (v12 != 10);
          v14 = v13 - (long)v11;
        }
label_f465:
        if (v18 != v14) { // branch-flip
          if (!dat_264e0) {
            v9 = 0;
            break;
          }
          v9 = (unsigned int)~re_search((re_pattern_buffer *)0x264c0,(char *)v4,(int)v15,0,(int)v15,NULL) >> 0x1f;
        }
        else {
          v9 = (unsigned int)v17;
        }
        v16 += 1;
        if ((v16 > v2) || (!(char)v9)) break;
      } while( true );
    }
label_f2fb:
    a0 = (long *)*a0;
    if (!a0) {
      v12 = 0;
      *a2 = v1;
      *a4 = v2;
      if (!(char)v9)
        v12 = (v20 != 0) * '\x02' | v19 != 0;
      return v12;
    }
  } while( true );
}

// Function: sub_f560 @ 0xf560
undefined16 sub_f560(char *a0,char *a1,char *a2)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char v3 [16];
  
  v1 = strlen(a0);
  v2 = strlen(a1);
  v3._0_8_ = (char *)sub_15f50(v1 + v2 + 1 + strlen(a2));
  __sprintf_chk(v3._0_8_,1,0xffffffffffffffff,"%s%s%s",a0,a1,a2);
  v3._8_8_ = 0xf59a;
  return v3._0_16_;
}

// Function: sub_f5e0 @ 0xf5e0
void sub_f5e0(unsigned long a0)
{
  memset((void *)sub_15f50(),0,a0);
}

// Function: sub_f600 @ 0xf600
void sub_f600(unsigned long *a0)
{
  fflush_unlocked(stdout);
  for (; a0; a0 = (unsigned long *)*a0) {
    __fprintf_chk(stderr,1,"%3ld %3ld delete %ld insert %ld\n",a0[3],a0[4],a0[2],a0[1]);
  }
  fflush_unlocked(stderr); // tail-call
}

// Function: sub_f670 @ 0xf670
char * sub_f670(char *a0)
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

// Function: sub_f6d0 @ 0xf6d0
void sub_f6d0(char *a0)
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

// Function: sub_f700 @ 0xf700
unsigned long sub_f700(void)
{
  return 0;
}

// Function: sub_f710 @ 0xf710
void sub_f710(void)
{
  sub_f700(); // tail-call
}

// Function: sub_f720 @ 0xf720
unsigned long sub_f720(int a0) // early-return
{
  unsigned long v1; // rax
  
  if (0x5a < a0)
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),(unsigned int)(a0 - 0x61U) <= 0x19);
  if (0x41 <= a0)
    return 1;
  return (unsigned long)((unsigned int)(a0 - 0x30U) <= 9);
}

// Function: sub_f750 @ 0xf750
bool sub_f750(int a0)
{
  if (a0 <= 0x5a)
    return 0x40 < a0;
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_f770 @ 0xf770
bool sub_f770(unsigned int a0)
{
  return a0 <= 0x7f;
}

// Function: sub_f780 @ 0xf780
unsigned int sub_f780(int a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x20) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == 9);
}

// Function: sub_f7a0 @ 0xf7a0
unsigned long sub_f7a0(unsigned int a0)
{
  unsigned long v1; // rax
  
  if ((int)a0 <= 0x1f)
    return (unsigned long)(~a0 >> 0x1f);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x7f);
}

// Function: sub_f7c0 @ 0xf7c0
bool sub_f7c0(int a0)
{
  return (unsigned int)(a0 - 0x30U) <= 9;
}

// Function: sub_f7d0 @ 0xf7d0
bool sub_f7d0(int a0)
{
  return (unsigned int)(a0 - 0x21U) <= 0x5d;
}

// Function: sub_f7e0 @ 0xf7e0
bool sub_f7e0(int a0)
{
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_f7f0 @ 0xf7f0
bool sub_f7f0(int a0)
{
  return (unsigned int)(a0 - 0x20U) <= 0x5e;
}

// Function: sub_f800 @ 0xf800
unsigned long sub_f800(int a0)
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

// Function: sub_f850 @ 0xf850
bool sub_f850(int a0)
{
  if (a0 <= 0xd)
    return 8 < a0;
  return a0 == 0x20;
}

// Function: sub_f870 @ 0xf870
bool sub_f870(int a0)
{
  return (unsigned int)(a0 - 0x41U) <= 0x19;
}

// Function: sub_f880 @ 0xf880
unsigned int sub_f880(int a0)
{
  if ((unsigned int)(a0 - 0x30U) <= 0x36)
    return (unsigned int)(0x7e0000007e03ff >> ((unsigned char)(a0 - 0x30U) & 0x3f)) & 1;
  return 0;
}

// Function: sub_f8b0 @ 0xf8b0
int sub_f8b0(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_f8d0 @ 0xf8d0
int sub_f8d0(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_f8f0 @ 0xf8f0
void sub_f8f0(void)
{
  return;
}

// Function: sub_f900 @ 0xf900
void sub_f900(int a0)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  char *v4; // rax
  char v5 [4104]; // stack - 0x1038
  char *v6;
  long v7; // fs_offset
  unsigned long v8; // stack - 0x30
  
  v8 = *(unsigned long *)(v7 + 0x28);
  (*dat_26840)();
  if (a0) // branch-flip
    v6 = dat_26838;
  else {
    v6 = dat_26830;
  }
  v1 = strlen(dat_26828);
  v2 = strlen(v6);
  if (0x1000 <= v1 + v2) { // branch-flip
    write(2,dat_26828,v1);
    write(2,": re",2);
    write(2,v6,v2);
    v5[0] = 10;
    v4 = (char *)0x1;
  }
  else {
    v3 = __mempcpy_chk(v5,dat_26828,v1,0x1002);
    *(unsigned short *)v3 = s_1d6f2._0_2_;
    v4 = mempcpy(&v3[2],v6,v2);
    *v4 = 10;
    v4 = &v4[1 - (long)v5];
  }
  write(2,v5,(unsigned long)v4);
  if (!a0)
    _exit(dat_260b0); // no-return
  raise(a0);
  abort(); // no-return
}

// Function: sub_fa20 @ 0xfa20
unsigned long sub_fa20(unsigned long a0,int a1)
{
  if (!a1)
    return 0;
  sub_f900(0xb); // no-return
}

// Function: sub_fa40 @ 0xfa40
void sub_fa40(unsigned long a0)
{
  if ((int)a0)
    a0 = (unsigned long)(-(unsigned int)(dat_26820 == 0) & 0xb);
  sub_f900(a0); // no-return
}

// Function: sub_fa70 @ 0xfa70
unsigned long sub_fa70(void *a0) // return-dupe
{
  if (!a0)
    a0 = sub_f8f0;
  dat_26840 = a0;
  dat_26838 = dcgettext(NULL,"program error",5);
  dat_26830 = dcgettext(NULL,"stack overflow",5);
  dat_26828 = sub_10a40();
  if (!sub_14a50(sub_fa40,0x26860,0x10000)) {
    dat_26820 = sub_149d0(sub_fa20);
    return 0;
  }
  *__errno_location() = 0x5f;
  return 0xffffffff;
}

// Function: sub_fb20 @ 0xfb20
void sub_fb20(char *a0)
{
  char v1;
  char *v2;
  char *v3;
  
  v2 = a0;
  do {
    while (v1 = *a0, v1 == '\\') {
      v3 = &a0[1];
      v1 = '\\';
      if (*v3) {
        a0 = &a0[1];
        v1 = *v3;
      }
      v3 = &v2[1];
      *v2 = v1;
      a0 = &a0[1];
      v2 = v3;
    }
    v3 = &v2[1];
    a0 = &a0[1];
    *v2 = v1;
    v2 = v3;
  } while (v1);
}

// Function: sub_fb60 @ 0xfb60
void sub_fb60(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long *a3)
{
  (*(void *)*a3)(); // jump-as-call
}

// Function: sub_fb70 @ 0xfb70
void sub_fb70(void *a0)
{
  free(a0); // tail-call
}

// Function: sub_fb80 @ 0xfb80
void sub_fb80(void *a0)
{
  unsigned int *v1;
  long v2; // rbx
  void *v3;
  
  if (!*(int *)((long)a0 + 8)) {
    sub_11630(*(unsigned long *)((long)a0 + 0x10));
    free(a0); // tail-call
    return;
  }
  if (*(int *)((long)a0 + 8) == 1) {
    v3 = *(void **)((long)a0 + 0x10);
    if (1 <= *(long *)((long)a0 + 0x20)) {
      v2 = 0;
      do {
        while (v1 = (unsigned int *)((long)v3 + v2 * 0x48), *v1 & 0x8000000) {
          v2 += 1;
          regfree((re_pattern_buffer *)&v1[2]);
          v3 = *(void **)((long)a0 + 0x10);
          if (*(long *)((long)a0 + 0x20) <= v2) {
            free(v3); // return-dupe
            free(a0); // tail-call
            return;
          }
        }
        v2 += 1;
      } while (v2 < *(long *)((long)a0 + 0x20));
    }
    free(v3);
  }
  free(a0);
  return;
}

// Function: sub_fc10 @ 0xfc10
unsigned int sub_fc10(unsigned char *a0,unsigned char *a1)
{
  unsigned int v1; // eax
  
  v1 = sub_11d10(a0,a1);
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}

// Function: sub_fc30 @ 0xfc30
unsigned int sub_fc30(char *a0,char *a1)
{
  unsigned int v1; // eax
  
  v1 = strcmp(a0,a1);
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}

// Function: sub_fc50 @ 0xfc50
unsigned long sub_fc50(unsigned char *a0,unsigned char *a1,unsigned int a2) // early-return
{
  int v1; // eax
  unsigned int v2; // eax
  unsigned long v3; // rax
  unsigned long v4; // rax
  unsigned char *v5; // rax
  char *v6; // rax
  unsigned char *v7;
  
  if (!(a2 & 8)) {
    if (!(a2 & 0x10))
      return strcmp((char *)a0,(char *)a1); // tail-call
    return sub_11d10(a0,a1); // tail-call
  }
  if (!(a2 & 0x10)) {
    v3 = strlen((char *)a0);
    v1 = strncmp((char *)a0,(char *)a1,v3);
    v4 = CONCAT44(dat_4,v1);
    if (!v1) {
      if ((int)(char)a1[v3] == 0x2f)
        return 0;
      v4 = (unsigned long)(unsigned int)(int)(char)a1[v3];
    }
    return v4;
  }
  v5 = (unsigned char *)sub_162e0(a1);
  v7 = v5;
  while (v6 = strchr((char *)v7,0x2f), v6) {
    *v6 = '\0';
    v2 = sub_11d10(a0,v5);
    if ((int)v2 <= 0) {
      free(v5); // return-dupe
      return (unsigned long)v2;
    }
    *v6 = '/';
    v7 = (unsigned char *)&v6[1];
  }
  v2 = sub_11d10(a0,v5);
  free(v5);
  return (unsigned long)v2;
}

// Function: sub_fd40 @ 0xfd40
void sub_fd40(unsigned long *a0,int a1,unsigned int a2)
{
  unsigned long v1;
  struct_26 *v2; // rax
  void *v3; // rcx
  void *v4; // rdx
  bool v5; // zf
  
  v2 = (struct_26 *)sub_16200(0x28);
  v2->field_0x8 = a1;
  v2->field_0xc = a2;
  if (!a1) {
    v5 = (a2 & 0x10) == 0;
    v3 = sub_fc10;
    if (v5)
      v3 = sub_fc30;
    v4 = sub_fdd0;
    if (v5)
      v4 = sub_fdc0;
    v2->field_0x10 = sub_11470(0,NULL,v4,v3,sub_fb70);
  }
  v1 = *a0;
  *a0 = v2;
  v2->field_0x0 = v1;
}

// Function: sub_fdc0 @ 0xfdc0
unsigned long sub_fdc0(unsigned char *a0,unsigned long a1)
{
  return sub_11400(a0,a1); // tail-call
}

// Function: sub_fdd0 @ 0xfdd0
unsigned long sub_fdd0(char *a0,unsigned long a1)
{
  int v1; // eax
  char v2 [4]; // stack - 0x68
  unsigned long v3;
  unsigned long v4; // stack - 0x64
  char v5; // stack - 0x5c
  char *v6; // stack - 0x58
  long v7; // stack - 0x50
  char v8; // stack - 0x48
  unsigned int v9; // stack - 0x44
  
  v3 = 0;
  v2[0] = 0;
  v4 = 0;
  v6 = a0;
  while( true ) {
    v5 = 0;
    sub_11f60(v2);
    if ((v8) && (!v9)) break;
    if (v8) // branch-flip
      v1 = towlower(v9);
    else {
      v1 = (unsigned int)*v6;
    }
    v6 = &v6[v7];
    v3 = (long)v1 + v3 * 0x1f;
  }
  return v3 % a1;
}

// Function: sub_fea0 @ 0xfea0
char * sub_fea0(unsigned int a0,struct_20 *a1,char *a2,char *a3)
{
  char *v1; // rax
  
  while( true ) {
    strcpy(a3,a2);
    v1 = (long)sub_11210(a1,a3);
    while( true ) {
      if (v1) {
        v1 = (char *)0x1;
        return v1;
      }
      if ((!(a0 & 8)) || (v1 = strrchr(a3,0x2f), !v1)) break;
      *v1 = '\0';
      v1 = (long)sub_11210(a1,a3);
    }
    if ((a0 & 0x40000000) || (v1 = strchr(a2,0x2f), !v1)) break;
    a2 = &v1[1];
  }
  v1 = NULL;
  return v1;
}

// Function: sub_ff50 @ 0xff50
void sub_ff50(struct_28 *a0,unsigned long a1)
{
  unsigned long v1;
  struct_27 *v2; // rax
  
  v2 = (struct_27 *)sub_15f50(0x10);
  v1 = a0->field_0x8;
  v2->field_0x8 = a1;
  v2->field_0x0 = v1;
  a0->field_0x8 = v2;
}

// Function: sub_ff80 @ 0xff80
unsigned long sub_ff80(char *a0,unsigned int a1) // return-dupe x2
{
  char v1;
  unsigned long v2; // rax
  char *v3;
  
  do {
    v1 = *a0;
    v3 = &a0[1];
    if ('A' <= v1) { // branch-flip
      if (v1 != '\\') { // branch-flip
        if (']' <= v1) { // branch-flip
          if (!(v1 + 0x85U & 0xfd)) goto label_10010;
        }
        else if (v1 == '[')
          return 1;
      }
      else if (((!(a1 & 0x8000000)) && (!(a1 & 2))) && (a0[1]))
        v3 = &a0[2];
    }
    else if ('!' <= v1) { // branch-flip
      v2 = 1L << (v1 - 0x21U & 0x3fU);
      if (v2 & 0x80000401) { // branch-flip
        if ((a1 & 0x20) && (a0[1] == '('))
          return 1;
      }
      else if (v2 & 0x2180) { // branch-flip
label_10010:
        if (a1 & 0x8000000)
          return 1;
      }
      else if (v2 & 0x40000200)
        return 1;
    }
    else if (!v1)
      return 0;
    a0 = v3;
  } while( true );
}

// Function: sub_10050 @ 0x10050
void sub_10050(void)
{
  sub_16200(0x10); // tail-call
}

// Function: sub_10060 @ 0x10060
void sub_10060(struct_29 *a0)
{
  unsigned long *v1;
  unsigned long *v2;
  
  v2 = a0->field_0x0;
  while (v2) {
    v1 = (unsigned long *)*v2;
    sub_fb80(v2);
    v2 = v1;
  }
  v2 = a0->field_0x8;
  while (v2) {
    v1 = (unsigned long *)*v2;
    free((void *)v2[1]);
    free(v2);
    v2 = v1;
  }
  free(a0); // tail-call
}

// Function: sub_100c0 @ 0x100c0
unsigned long sub_100c0(unsigned long a0,char *a1,unsigned int a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // rax
  char v2;
  char v3;
  char *v4;
  unsigned long v5; // r8
  char *v6;
  void *v7;
  
  v7 = sub_fc50;
  if (a2 & 0x10000000)
    v7 = dat_25fc8;
  v1 = (*v7)(a0,a1,a2);
  v5 = CONCAT71((undefined7)((unsigned long)a4 >> 8),(int)v1 == 0);
  if ((!(a2 & 0x40000000)) && (v2 = *a1, v2)) {
    do {
      if (!(int)v1)
        return 1;
      while( true ) {
        v4 = &a1[1];
        v3 = *v4;
        v6 = a1;
        if (v2 == '/') break;
        a1 = v4;
        v2 = v3;
        if (!v3) {
          v5 = 0;
          return v5 & 0xffffffff; // return-dupe
        }
      }
      while (a1 = v4, v3 == '/') {
        v3 = a1[1];
        v4 = &a1[1];
        v6 = a1;
      }
      v1 = (*v7)(a0,a1,a2);
      v2 = v6[1];
    } while (v2);
    v5 = CONCAT71((undefined7)(v5 >> 8),(int)v1 == 0);
  }
  return v5 & 0xffffffff;
}

// Function: sub_10180 @ 0x10180
unsigned int sub_10180(struct_22 *a0,char *a1)
{
  unsigned long v1;
  unsigned int v2; // eax
  
  if (!(a0->field_0x0 & 0x8000000)) {
    v1 = a0->field_0x8;
    return sub_100c0(v1,a1); // tail-call
  }
  v2 = regexec((re_pattern_buffer *)&a0->field_0x8,a1,0,NULL,0);
  return CONCAT31((undefined3)(v2 >> 8),v2 == 0);
}

// Function: sub_101c0 @ 0x101c0
unsigned long sub_101c0(long a0,long a1,char *a2)
{
  unsigned long v1; // rax
  long v2; // r13
  
  if (a1 <= 0)
    return 0;
  v2 = 0;
  do {
    v1 = sub_10180(a0,a2);
    if ((char)v1)
      return v1;
    v2 += 1;
    a0 += 0x48;
  } while (v2 != a1);
  return 0;
}

// Function: sub_10220 @ 0x10220
unsigned int sub_10220(long *a0,char *a1)
{
  unsigned int v1;
  struct_20 *v2;
  long v3;
  long v4;
  char *v5; // rax
  long *v6;
  unsigned int v7; // r12d
  
  if (!(long *)*a0)
    return 0;
  v5 = NULL;
  v6 = (long *)*a0;
  do {
    if (!(int)v6[1]) {
      if (!v5)
        v5 = (char *)sub_15f50(strlen(a1) + 1);
      v2 = (struct_20 *)v6[2];
      v1 = *(unsigned int *)((long)v6 + 0xc);
      if (!sub_fea0(v1,v2,a1,v5)) goto label_1025b;
      v7 = 0; // return-dupe
      free(v5); // return-dupe
      return (*(unsigned int *)((long)v6 + 0xc) >> 0x1d ^ 1) & 1 ^ v7;
    }
    v3 = v6[4];
    v4 = v6[2];
    if (sub_101c0(v4,v3,a1)) {
      v7 = 0;
      free(v5);
      return (*(unsigned int *)((long)v6 + 0xc) >> 0x1d ^ 1) & 1 ^ v7;
    }
label_1025b:
    if (!(long *)*v6) {
      v7 = 1;
      free(v5);
      return (*(unsigned int *)((long)v6 + 0xc) >> 0x1d ^ 1) & 1 ^ v7;
    }
    v6 = (long *)*v6;
  } while( true );
}

// Function: sub_102e0 @ 0x102e0
void sub_102e0(long *a0,char *a1,unsigned int a2) // return-dupe x2
{
  unsigned int *v1;
  int v2; // eax
  unsigned long v3; // rax
  char *v4; // rax
  void *v5;
  unsigned long v6;
  long v7;
  unsigned int v8; // r14d
  long v9;
  
  v7 = *a0;
  if ((!(a2 & 0x18000000)) || (!sub_ff80(a1,a2))) {
    if (((!v7) || (*(int *)(v7 + 8))) || ((*(unsigned int *)(v7 + 0xc) ^ a2) & 0x60000018)) {
      sub_fd40(a0,0,a2);
      v7 = *a0;
    }
    v4 = (char *)sub_162e0(a1);
    if ((a2 & 0x10000002) == 0x10000000)
      sub_fb20(v4);
    v5 = *(void **)(v7 + 0x10);
    if (v4 == (char *)sub_11a90(v5,v4))
      return;
    free(v4); // tail-call
    return;
  }
  if ((!v7) || ((*(int *)(v7 + 8) != 1 || ((*(unsigned int *)(v7 + 0xc) ^ a2) & 0x20000000)))) {
    sub_fd40(a0,1,a2);
    v7 = *a0;
  }
  v9 = *(long *)(v7 + 0x20);
  v5 = *(void **)(v7 + 0x10);
  if (v9 == *(long *)(v7 + 0x18)) {
    v5 = (void *)sub_16100(v5,(long *)(v7 + 0x18),1,-1,0x48);
    v9 = *(long *)(v7 + 0x20);
    *(void **)(v7 + 0x10) = v5;
  }
  *(long *)(v7 + 0x20) = v9 + 1;
  v1 = (unsigned int *)((long)v5 + v9 * 0x48);
  *v1 = a2;
  if (!(a2 & 0x8000000)) {
    if (a2 & 0x4000000) {
      a1 = (char *)sub_162e0(a1);
      sub_ff50(a0,a1);
    }
    *(char **)&v1[2] = a1;
    return;
  }
  v8 = (int)a2 >> 3 & 2U | 9;
  if (a2 & 8) { // branch-flip
    v3 = strlen(a1);
    if (0 < (long)v3) {
      do {
        v6 = v3;
        if (a1[v3 - 1] != '/') goto label_103c1;
        v3 -= 1;
      } while (v3);
      goto label_10421;
    }
    v6 = v3;
    if (!v3) goto label_10421;
label_103c1:
    v4 = (char *)sub_15f80(v6 + 7);
    memcpy(v4,a1,v3);
    builtin_strncpy(&v4[v3],"(/.*)?",7);
    v2 = regcomp((re_pattern_buffer *)&v1[2],v4,v8);
    free(v4);
  }
  else {
    v2 = regcomp((re_pattern_buffer *)&v1[2],a1,v8);
  }
  if (!v2)
    return;
  v9 = *(long *)(v7 + 0x20) + -1;
label_10421:
  *(long *)(v7 + 0x20) = v9;
  return;
}

// Function: sub_10580 @ 0x10580
int sub_10580(void *a0,struct_28 *a1,FILE *a2,unsigned int a3,unsigned char a4,unsigned long a5)
{
  int v1; // eax
  int *v10; // stack - 0x58
  int v11; // stack - 0x50
  void *v2; // rax
  unsigned char *v3;
  long v4; // stack - 0x48
  unsigned char *v5; // rbp
  unsigned char *v6; // r14
  long v7;
  unsigned char *v8;
  int v9; // stack - 0x5c
  
  v7 = 0;
  v2 = NULL;
  v4 = 0;
  while( true ) {
    v1 = getc(a2);
    if (v1 == -1) break;
    if (v4 == v7)
      v2 = (void *)sub_16100(v2,&v4,1,-1,1);
    *(char *)((long)v2 + v7) = (char)v1;
    v7 += 1;
  }
  v9 = ferror(a2);
  v10 = __errno_location();
  v11 = 0;
  if (v9) {
    v9 = *v10;
    v11 = -(unsigned int)(v9 != 0);
  }
  v3 = (unsigned char *)sub_15fe0(v2,v7 + 1);
  v3[v7] = a4;
  if (v7) { // branch-flip
    v5 = &v3[(unsigned long)(v3[v7 + -1] != a4) + v7];
    sub_ff50(a1,v3);
    if (v3 < v5) {
      v8 = v3;
      do {
        v6 = &v8[1];
        if (*v8 == a4) {
          v7 = *(long *)__ctype_b_loc();
          if (*(unsigned short *)(v7 + (unsigned long)a4 * 2) & 0x2000) { // branch-flip
            for (; v3 != v8; v8 = &v8[-1]) {
              if (!(*(unsigned char *)(v7 + 1 + (unsigned long)v8[-1] * 2) & 0x20)) goto label_1071d;
            }
          }
          else {
label_1071d:
            *v8 = 0;
            (*a0)(a1,v3,a3,a5);
          }
          v3 = v6;
        }
        v8 = v6;
      } while (v5 != v6);
    }
  }
  else {
    sub_ff50(a1,v3);
  }
  *v10 = v9;
  return v11;
}

// Function: sub_10770 @ 0x10770
unsigned int sub_10770(unsigned long a0,struct_28 *a1,char *a2,unsigned long a3,char a4) // return-dupe
{
  unsigned int v1; // eax
  FILE *v2; // rax
  unsigned long v3; // stack - 0x20
  
  v3 = a0;
  if ((*a2 == '-') && (!a2[1])) {
    v1 = sub_10580(sub_fb60,a1,stdin,a3,(int)a4,&v3);
    return v1;
  }
  v2 = fopen(a2,"re");
  if (!v2)
    return 0xffffffff;
  v1 = sub_10580(sub_fb60,a1,v2,a3 & 0xffffffff,(int)a4,&v3);
  if (fclose(v2))
    return 0xffffffff;
  return v1;
}

// Function: sub_10830 @ 0x10830
void sub_10830(void *a0)
{
  unsigned int v1; // eax
  
  v1 = *(unsigned int *)((long)a0 + 0x18) & 0xf000;
  if (v1 == 0x8000) {
    if (!*(long *)((long)a0 + 0x30)) {
      dcgettext(NULL,"regular empty file",5); // tail-call
      return;
    }
    dcgettext(NULL,"regular file",5); // tail-call
    return;
  }
  if (v1 == 0x4000) {
    dcgettext(NULL,"directory",5); // tail-call
    return;
  }
  if (v1 == 0xa000) {
    dcgettext(NULL,"symbolic link",5); // tail-call
    return;
  }
  if (v1 != 0x6000) {
    if (v1 == 0x2000) {
      dcgettext(NULL,"character special file",5); // tail-call
      return;
    }
    if (v1 != 0x1000) {
      if (v1 != 0xc000) {
        dcgettext(NULL,"weird file",5); // tail-call
        return;
      }
      dcgettext(NULL,"socket",5); // tail-call
      return;
    }
    dcgettext(NULL,"fifo",5); // tail-call
    return;
  }
  dcgettext(NULL,"block special file",5); // tail-call
}

// Function: sub_10920 @ 0x10920
void sub_10920(void *a0,char *a1,unsigned long *a2)
{
  if (sub_10940(a0,a1,a2))
    return;
  sub_16300(); // no-return
}

// Function: sub_10940 @ 0x10940
void * sub_10940(void *a0,char *a1,unsigned long *a2)
{
  char *v1;
  long v2; // rax
  unsigned long v3; // rax
  void *v4; // rax
  char *v5; // rax
  unsigned long v6; // rbx
  char v7; // stack - 0x41
  
  v1 = (char *)sub_f670(a0);
  v2 = sub_f6d0(v1);
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

// Function: sub_10a40 @ 0x10a40
unsigned long sub_10a40(void)
{
  return __progname;
}

// Function: sub_10a50 @ 0x10a50
void sub_10a50(timespec *a0)
{
  clock_gettime(0,a0); // tail-call
}

// Function: sub_10a60 @ 0x10a60
undefined16 sub_10a60(void)
{
  char v1 [16]; // stack - 0x28
  
  sub_10a50(v1);
  return v1._0_16_;
}

// Function: sub_10ab0 @ 0x10ab0
bool sub_10ab0(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_14490(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_10b40 @ 0x10b40
unsigned long sub_10b40(unsigned long a0)
{
  unsigned long v1; // rax
  unsigned long v2;
  unsigned long v3; // rdx
  unsigned long v4; // rsi
  long v5; // r8
  
  v1 = SUB168(ZEXT816(a0) * ZEXT816(0xaaaaaaaaaaaaaaab),0);
  v3 = a0 % 3;
  if ((10 <= a0) && (v3)) {
    v5 = 0x10;
    v4 = 9;
    v2 = 3;
    do {
      v2 += 2;
      v4 += v5;
      v1 = a0 / v2;
      v3 = a0 % v2;
      if (a0 <= v4) break;
      v5 += 8;
    } while (v3);
  }
  v2 = v1 >> 8;
  v1 = CONCAT71((undefined7)v2,v3 != 0);
  return v1;
}

// Function: sub_10bb0 @ 0x10bb0
unsigned long sub_10bb0(unsigned long a0)
{
  char v1; // al
  unsigned long v2; // rax
  unsigned long v3;
  
  v2 = 10;
  if (10 <= a0)
    v2 = a0;
  v3 = v2 | 1;
  while ((v3 != 0xffffffffffffffff && (v1 = sub_10b40(v3), !v1))) {
    v3 += 2;
  }
  v2 = v3;
  return v2;
}

// Function: sub_10bf0 @ 0x10bf0
unsigned long sub_10bf0(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_16ce0(a0,3) % a1;
}

// Function: sub_10c10 @ 0x10c10
bool sub_10c10(long a0,long a1)
{
  return a1 == a0;
}

// Function: sub_10c20 @ 0x10c20
unsigned long sub_10c20(struct_25 *a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = a0->field_0x28;
  if (v2 == (float *)0x1d7f0)
    return 1;
  v1 = v2[2];
  if ((((dat_1d804 < v1) && (v1 < dat_1d808)) && (dat_1d80c < v2[3])) && (dat_1d820 <= *v2)) {
    v3 = *v2 + dat_1d804;
    if (((v3 < v2[1]) && (v2[1] <= dat_1d810)) && (v3 < v1))
      return 1;
  }
  a0->field_0x28 = (float *)0x1d7f0;
  return 0;
}

// Function: sub_10cb0 @ 0x10cb0
void sub_10cb0(struct_30 *a0,struct_27 *a1)
{
  unsigned long v1;
  
  v1 = a0->field_0x48;
  a1->field_0x0 = 0;
  a1->field_0x8 = v1;
  a0->field_0x48 = a1;
}

// Function: sub_10cd0 @ 0x10cd0
long sub_10cd0(struct_19 *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*a0->field_0x30)(a1,a0->field_0x10);
  if (v1 < (unsigned long)a0->field_0x10)
    return v1 * 0x10 + a0->field_0x0;
  abort(); // no-return
}

// Function: sub_10d00 @ 0x10d00
long sub_10d00(struct_20 *a0,long a1,unsigned long *a2,bool a3) // early-return
{
  void *v1;
  long *v2;
  long v3;
  long *v4;
  long v5;
  
  v4 = (long *)sub_10cd0(a0,a1);
  *a2 = v4;
  v5 = *v4;
  if (!v5)
    return 0;
  if (v5 != a1) {
    v1 = a0->field_0x38;
    if (!(*v1)(a1)) {
      while( true ) {
        if (!(long *)v4[1])
          return 0;
        if ((*(long *)v4[1] == a1) || (v1 = a0->field_0x38, (*v1)(a1))) break;
        v4 = (long *)v4[1];
      }
      v5 = *(long *)v4[1];
      if (!a3)
        return v5;
      v4[1] = ((long *)v4[1])[1];
      sub_10cb0(a0);
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
      sub_10cb0(a0);
      return v5;
    }
    *v4 = 0;
  }
  return v5;
}

// Function: sub_10e00 @ 0x10e00
void sub_10e00(struct_30 *a0)
{
  if (a0->field_0x48) {
    a0->field_0x48 = *(unsigned long *)(a0->field_0x48 + 8);
    return;
  }
  malloc(0x10); // tail-call
}

// Function: sub_10e30 @ 0x10e30
unsigned long sub_10e30(struct_32 *a0,struct_37 *a1,bool a2) // return-dupe x2
{
  long *v1;
  long v2;
  struct_5 *v3; // rax
  struct_5 *v4; // rax
  long *v5;
  long *v6;
  long v7;
  
  v5 = a1->field_0x0;
  if (a1->field_0x8 <= v5)
    return 1;
  do {
    while (v7 = *v5, !v7) {
label_10e58:
      v5 = &v5[2];
      if (a1->field_0x8 <= v5)
        return 1;
    }
    v6 = (long *)v5[1];
    if ((long *)v5[1]) {
      do {
        while( true ) {
          v7 = *v6;
          v3 = (struct_5 *)sub_10cd0(a0,v7);
          v1 = (long *)v6[1];
          if (v3->field_0x0) break;
          v3->field_0x0 = v7;
          a0->field_0x18 = a0->field_0x18 + 1;
          sub_10cb0(a0);
          v6 = v1;
          if (!v1) goto label_10ec8;
        }
        v6[1] = v3->field_0x8;
        v3->field_0x8 = (long)v6;
        v6 = v1;
      } while (v1);
label_10ec8:
      v7 = *v5;
    }
    v5[1] = 0;
    if (a2) goto label_10e58;
    v3 = (struct_5 *)sub_10cd0(a0,v7);
    if (v3->field_0x0) { // branch-flip
      v4 = (struct_5 *)sub_10e00(a0);
      if (!v4)
        return 0;
      v2 = v3->field_0x8;
      v4->field_0x0 = v7;
      v4->field_0x8 = v2;
      v3->field_0x8 = (long)v4;
    }
    else {
      v3->field_0x0 = v7;
      a0->field_0x18 = a0->field_0x18 + 1;
    }
    *v5 = 0;
    v5 = &v5[2];
    a1->field_0x18 = a1->field_0x18 + -1;
    if (a1->field_0x8 <= v5)
      return 1;
  } while( true );
}

// Function: sub_10f60 @ 0x10f60
unsigned long sub_10f60(float a0,unsigned long a1,bool a2) // early-return, ternary
{
  unsigned long v1; // rax
  
  if (!a2) {
    a0 = (float)a1 / a0;
    if (dat_1d814 <= a0)
      return 0;
    a1 = (a0 < dat_1d818) ? (unsigned long)a0 : (long)(a0 - dat_1d818) ^ 0x8000000000000000;
  }
  v1 = sub_10bb0(a1);
  if ((!(v1 >> 0x3c & 1)) && (!(v1 >> 0x3d)))
    return v1;
  return 0;
}

// Function: sub_10ff0 @ 0x10ff0
unsigned long sub_10ff0(struct_34 *a0)
{
  return a0->field_0x10;
}

// Function: sub_11000 @ 0x11000
unsigned long sub_11000(struct_35 *a0)
{
  return a0->field_0x18;
}

// Function: sub_11010 @ 0x11010
unsigned long sub_11010(struct_36 *a0)
{
  return a0->field_0x20;
}

// Function: sub_11020 @ 0x11020
unsigned long sub_11020(struct_37 *a0)
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

// Function: sub_11080 @ 0x11080
bool sub_11080(struct_37 *a0)
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
label_110a0:
      v2 = &v2[2];
    }
    v1 = v2[1];
    v4 += 1;
    v3 += 1;
    if (!v1) goto label_110a0;
    do {
      v1 = *(long *)(v1 + 8);
      v3 += 1;
    } while (v1);
    v2 = &v2[2];
  } while( true );
}

// Function: sub_110f0 @ 0x110f0
void sub_110f0(struct_36 *a0,FILE *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_11010(a0);
  v2 = sub_10ff0();
  v3 = sub_11000();
  v4 = sub_11020();
  __fprintf_chk(a1,1,"# entries:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v2);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v3,((double)v3 * dat_1d820) / (double)v2);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v4); // tail-call
}

// Function: sub_11210 @ 0x11210
char * sub_11210(struct_20 *a0,char *a1)
{
  void *v1;
  long *v2;
  char *v3;
  
  v2 = (long *)sub_10cd0(a0,a1);
  v3 = (char *)*v2;
  if (!v3)
    return NULL;
  while( true ) {
    if (v3 == a1)
      return a1;
    v1 = a0->field_0x38;
    if ((*v1)(a1)) break;
    v2 = (long *)v2[1];
    if (!v2)
      return NULL;
    v3 = (char *)*v2;
  }
  return (char *)*v2;
}

// Function: sub_11270 @ 0x11270
long sub_11270(struct_37 *a0)
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

// Function: sub_112c0 @ 0x112c0
long sub_112c0(void *a0,long a1)
{
  long v1;
  long *v2; // rax
  long *v3;
  
  v2 = (long *)sub_10cd0(a0,a1);
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
    if (*(long **)((long)a0 + 8) <= v2)
      return 0;
  } while (!*v2);
  return *v2;
}

// Function: sub_11330 @ 0x11330
void sub_11330(struct_37 *a0,long a1,unsigned long a2)
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

// Function: sub_11380 @ 0x11380
long sub_11380(struct_37 *a0,void *a1,unsigned long a2)
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

// Function: sub_11400 @ 0x11400
unsigned long sub_11400(unsigned char *a0,unsigned long a1)
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

// Function: sub_11440 @ 0x11440
void sub_11440(struct_38 *a0)
{
  a0->field_0x10 = 0;
  a0->field_0x0 = 0x3f80000000000000;
  a0->field_0x8 = 0x3fb4fdf43f4ccccd;
}

// Function: sub_11470 @ 0x11470
unsigned long * sub_11470(unsigned long a0,void *a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_10bf0;
  if (!a3)
    a3 = sub_10c10;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = (void *)0x1d7f0;
  v1[5] = a1;
  if (sub_10c20(v1)) {
    v2 = sub_10f60(*(float *)((long)a1 + 8),a0,(unsigned char)*(char *)((long)a1 + 0x10));
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

// Function: sub_11580 @ 0x11580
void sub_11580(struct_39 *a0)
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
        if (a0->field_0x8 <= v5) goto label_1161b;
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
label_1161b:
  a0->field_0x18 = 0;
  a0->field_0x20 = 0;
  return;
}

// Function: sub_11630 @ 0x11630
void sub_11630(struct_24 *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = a0->field_0x0;
  v3 = a0->field_0x8;
  if ((a0->field_0x40) && (a0->field_0x20)) {
    if (v3 <= v4) goto label_116dc;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_1169f;
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
label_1169f:
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
label_116dc:
  v2 = a0->field_0x48;
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free(a0->field_0x0);
  free(a0); // tail-call
}

// Function: sub_11720 @ 0x11720
unsigned int sub_11720(struct_33 *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_10f60(*(float *)(a0->field_0x28 + 8),a1,(unsigned char)*(char *)(a0->field_0x28 + 0x10));
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
  v1 = sub_10e30(&v2,a0,0);
  if (!(char)v1) {
    a0->field_0x48 = v11;
    if ((sub_10e30(a0,&v2,1)) && (sub_10e30(a0,&v2,0))) {
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

// Function: sub_118a0 @ 0x118a0
unsigned long sub_118a0(void *a0,long a1,long *a2) // ternary
{
  long v1;
  struct_5 *v2; // rax
  long *v3; // stack - 0x38
  unsigned long v4; // rsi
  float v5; // xmm4_da
  float v6; // xmm5_da
  
  if (a1) {
    v1 = sub_10d00(a0,a1,&v3,0);
    if (v1) {
      if (a2) {
        *a2 = v1;
        return 0;
      }
      return 0;
    }
    v6 = (float)*(unsigned long *)((long)a0 + 0x18);
    v5 = (float)*(unsigned long *)((long)a0 + 0x10);
    if (*(float *)(*(long *)((long)a0 + 0x28) + 8) * v5 < v6) {
      sub_10c20(a0);
      v1 = *(long *)((long)a0 + 0x28);
      if (v5 * *(float *)(v1 + 8) < v6) {
        v5 = v5 * *(float *)(v1 + 0xc);
        if (!*(char *)(v1 + 0x10))
          v5 = v5 * *(float *)(v1 + 8);
        if (dat_1d814 <= v5)
          return 0xffffffff;
        v4 = (dat_1d818 <= v5) ? (long)(v5 - dat_1d818) ^ 0x8000000000000000 : (unsigned long)v5;
        if (!sub_11720(a0,v4))
          return 0xffffffff;
        if (sub_10d00(a0,a1,&v3,0)) {
          abort(); // no-return, return-dupe
        }
      }
    }
    if (!*v3) {
      *v3 = a1;
      *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + 1;
      *(long *)((long)a0 + 0x18) = *(long *)((long)a0 + 0x18) + 1;
      return 1;
    }
    v2 = (struct_5 *)sub_10e00(a0);
    if (!v2)
      return 0xffffffff;
    v2->field_0x0 = a1;
    v2->field_0x8 = v3[1];
    v3[1] = (long)v2;
    *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + 1;
    return 1;
  }
  abort();
}

// Function: sub_11a90 @ 0x11a90
unsigned long sub_11a90(void *a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_118a0(a0,a1,&v3);
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

// Function: sub_11af0 @ 0x11af0
long sub_11af0(void *a0,long a1) // ternary x2
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
  
  v5 = sub_10d00(a0,a1,&v7,1);
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
      sub_10c20(a0);
      v2 = *(float **)((long)a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_1d818 <= v8) ? (long)(v8 - dat_1d818) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_11720(a0,v6)) {
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

// Function: sub_11c80 @ 0x11c80
long sub_11c80(void *a0)
{
  return sub_11af0(a0); // tail-call
}

// Function: sub_11c90 @ 0x11c90
long sub_11c90(unsigned long a0)
{
  long v1;
  void *v2; // rax
  unsigned long v3; // rax
  
  v3 = a0 + 0x20;
  if ((0 <= (long)v3) && (a0 <= v3)) {
    v2 = malloc(v3);
    if (v2) {
      v3 = (long)v2 + 0x10U & 0xffffffffffffffe0;
      v1 = v3 + 0x10;
      *(char *)(v3 + 0xf) = (char)v1 - (char)v2;
      return v1;
    }
    return 0;
  }
  return 0;
}

// Function: sub_11ce0 @ 0x11ce0
void sub_11ce0(void *a0)
{
  if ((unsigned long)a0 & 0xf)
    abort(); // no-return
  if (!((unsigned long)a0 & 0x10))
    return;
  free((void *)((long)a0 - (unsigned long)*(unsigned char *)((long)a0 + -1))); // tail-call
}

// Function: sub_11d10 @ 0x11d10
unsigned long sub_11d10(unsigned char *a0,unsigned char *a1) // early-return x3
{
  unsigned char v1;
  unsigned char *v10; // stack - 0xa8
  unsigned long v11; // stack - 0xa0
  char v12; // stack - 0x98
  unsigned int v13; // stack - 0x94
  unsigned long v14; // stack - 0x74
  char v15; // stack - 0x6c
  unsigned char *v16; // stack - 0x68
  unsigned long v17; // stack - 0x60
  char v18; // stack - 0x58
  unsigned int v19; // stack - 0x54
  long v2;
  unsigned int v3;
  int v4; // eax
  unsigned char v5;
  char v6 [4]; // stack - 0xb8
  char v7 [4]; // stack - 0x78
  unsigned long v8; // stack - 0xb4
  char v9; // stack - 0xac
  
  if (a1 == a0)
    return 0;
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
    v6[0] = 0;
    v8 = 0;
    v7[0] = 0;
    v14 = 0;
    v10 = a0;
    v16 = a1;
    while( true ) {
      v9 = 0;
      v15 = 0;
      sub_11f60(v6);
      if ((v12) && (!v13)) break;
      sub_11f60(v7);
      if (v18) { // branch-flip
        if (!v19) break;
        if (!v12)
          return 1;
        v3 = towlower(v13);
        v3 -= towlower(v19);
        if (v3)
          return (unsigned long)v3;
      }
      else {
        if (v12)
          return 0xffffffff;
        if (v11 != v17) {
          if (v11 < v17) {
            if (memcmp(v10,v16,v11) <= 0)
              return 0xffffffff;
            return 1;
          }
          if (0 <= memcmp(v10,v16,v17))
            return 1;
          return 0xffffffff;
        }
        v4 = memcmp(v10,v16,v11);
        if (v4)
          return CONCAT44(dat_4,v4);
      }
      v10 = &v10[v11];
      v16 = &v16[v17];
    }
    sub_11f60(v6);
    if ((!v12) || (v13))
      return 1;
    sub_11f60(v7);
    if ((!v18) || (v3 = 0, v19))
      return 0xffffffff;
  }
  else {
    v2 = *(long *)__ctype_b_loc();
    do {
      v5 = *a0;
      if (*(unsigned char *)(v2 + 1 + (unsigned long)v5 * 2) & 1)
        v5 = *(unsigned char *)(*(long *)__ctype_tolower_loc() + (unsigned long)v5 * 4);
      v1 = *a1;
      if (*(unsigned char *)(v2 + 1 + (unsigned long)v1 * 2) & 1)
        v1 = *(unsigned char *)(*(long *)__ctype_tolower_loc() + (unsigned long)v1 * 4);
      if (!v5)
        return (unsigned long)-(unsigned int)v1;
      a0 = &a0[1];
      a1 = &a1[1];
    } while (v5 == v1);
    v3 = (unsigned int)v5 - (unsigned int)v1;
  }
  return (unsigned long)v3;
}

// Function: sub_11f60 @ 0x11f60
void sub_11f60(struct_21 *a0)
{
  char v1;
  char *v2;
  unsigned long v3; // rax
  long v4; // rax
  
  if (a0->field_0xc)
    return;
  if (!a0->field_0x0) {
    v2 = a0->field_0x10;
    v1 = *v2;
    if (sub_170c0((int)v1)) {
      a0->field_0x18 = 1;
      v1 = *v2;
      a0->field_0x20 = '\x01';
      a0->field_0x24 = (int)v1;
      goto label_11fa2;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x1[3]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    a0->field_0x0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v2 = a0->field_0x10;
  v4 = sub_16640(&a0->field_0x24,v2,sub_15550(v2,v3),(mbstate_t *)&a0->field_0x1[3]);
  a0->field_0x18 = v4;
  if (v4 == -1) {
    a0->field_0x18 = 1;
    a0->field_0x20 = '\0';
    a0->field_0xc = '\x01';
    return;
  }
  if (v4 != -2) { // branch-flip
    if (!v4) {
      a0->field_0x18 = 1;
      if (*a0->field_0x10)
        __assert_fail("*iter->cur.ptr == \'\\0\'","mbuiter.h",0xab,"mbuiter_multi_next"); // no-return
      if (a0->field_0x24)
        __assert_fail("iter->cur.wc == 0","mbuiter.h",0xac,"mbuiter_multi_next"); // no-return
    }
    a0->field_0x20 = '\x01';
    if (mbsinit((mbstate_t *)&a0->field_0x1[3]))
      a0->field_0x0 = '\0';
  }
  else {
    v3 = strlen(a0->field_0x10);
    a0->field_0x20 = '\0';
    a0->field_0x18 = v3;
  }
label_11fa2:
  a0->field_0xc = '\x01';
  return;
}

// Function: sub_120e0 @ 0x120e0
void sub_120e0(void *a0,long a1)
{
  *(long *)((long)a0 + 0x10) = *(long *)((long)a0 + 0x10) + a1;
}

// Function: sub_120f0 @ 0x120f0
void sub_120f0(struct_42 *a0,struct_42 *a1)
{
  char v1;
  
  v1 = a1->field_0x0;
  a0->field_0x0 = v1;
  if (v1) // branch-flip
    *(unsigned long *)&a0->field_0x1[3] = *(unsigned long *)&a1->field_0x1[3];
  else {
    *(unsigned long *)&a0->field_0x1[3] = 0;
  }
  a0->field_0xc = a1->field_0xc;
  sub_17060((struct_40 *)&a0[1].field_0x1[2],(struct_41 *)&a1[1].field_0x1[2]); // tail-call
}

// Function: sub_12130 @ 0x12130
int sub_12130(int a0,int a1)
{
  int v1; // edx
  
  v1 = (a0 - a1) + 0x17e;
  return (a0 - v1) + 3 + (v1 / 7) * 7;
}

// Function: sub_12170 @ 0x12170
void sub_12170(char *a0,long a1,long a2)
{
  long *v1; // rax
  
  if (a2) {
    v1 = __ctype_toupper_loc();
    do {
      a2 -= 1;
      a0[a2] = (char)*(unsigned int *)(*v1 + (unsigned long)*(unsigned char *)(a1 + a2) * 4);
    } while (a2);
    return;
  }
}

// Function: sub_121c0 @ 0x121c0
void sub_121c0(char *a0,long a1,long a2)
{
  long *v1; // rax
  
  if (a2) {
    v1 = __ctype_tolower_loc();
    do {
      a2 -= 1;
      a0[a2] = (char)*(unsigned int *)(*v1 + (unsigned long)*(unsigned char *)(a1 + a2) * 4);
    } while (a2);
    return;
  }
}

// Function: sub_12210 @ 0x12210
long sub_12210(char *a0,long a1,char *a2,tm *a3,char a4,unsigned int a5,int a6,long *a7,int a8) // ternary
{
  void *v1;
  long v10;
  unsigned char v11;
  int v12;
  long v13;
  char v14;
  unsigned int v15;
  int v16; // edx
  int v17;
  unsigned long v18;
  unsigned long v19;
  char v2;
  bool v20;
  unsigned long v21; // stack - 0x450
  unsigned long v22; // stack - 0x448
  char v23 [1010];
  unsigned long v24; // stack - 0x488
  unsigned int v25;
  char *v26;
  int v27;
  unsigned int v28;
  unsigned long v29; // r13
  int v3;
  int v30;
  unsigned long v31;
  char *v32;
  bool v33;
  int *v34; // stack - 0x4e8
  unsigned long v35; // stack - 0x4b8
  unsigned long v36; // stack - 0x4a8
  unsigned long v37; // stack - 0x480
  unsigned long v38; // stack - 0x478
  long v39; // stack - 0x470
  int v4;
  unsigned long v40; // stack - 0x468
  unsigned long v41; // stack - 0x460
  unsigned long v42; // stack - 0x458
  long v5;
  long *v6;
  char v7;
  bool v8;
  char *v9;
  
  v6 = a7;
  v31 = (unsigned long)a6;
  v34 = __errno_location();
  v3 = *(unsigned int *)&a3->field_0x8;
  v4 = *v34;
  v9 = "";
  if (*(char **)&a3->field_0x30)
    v9 = *(char **)&a3->field_0x30;
  if (0xd <= v3) // branch-flip
    v3 -= 0xc;
  else if (!v3)
    v3 = 0xc;
  v7 = *a2;
  v29 = 0;
  if (!v7) {
label_1230c:
    if ((a0) && (a1))
      *a0 = '\0';
    *v34 = v4;
    return v29;
  }
label_122ae:
  if (v7 != '%') {
    v19 = 0;
    if (0 <= (int)v31)
      v19 = v31;
    v18 = 1;
    if (v19)
      v18 = v19;
    if ((unsigned long)(a1 - v29) <= v18) goto label_12338;
    if (a0) {
      v32 = a0;
      if (1 < (int)v31) {
        v32 = &a0[v19 - 1];
        memset(a0,0x20,v19 - 1);
      }
      a0 = &v32[1];
      *v32 = *a2;
    }
    v29 += v18;
    goto label_122f9;
  }
  v15 = 0;
  v28 = 0;
  v7 = a4;
  while( true ) {
    v33 = (bool)v15;
    v32 = &a2[1];
    v25 = (unsigned int)*v32;
    a2 = &a2[1];
    v11 = *v32 - 0x23;
    v19 = (unsigned long)v25;
    if (0x3d <= v11) break;
    v18 = 1L << (v11 & 0x3f);
    if (v18 & 0x1000000000002500) // branch-flip
      v28 = v25;
    else if (v11 != 0x3b) { // branch-flip
      v15 = (unsigned int)v18 & 1;
      if (!(v18 & 1)) break;
    }
    else {
      v7 = '\x01';
    }
  }
  v18 = (unsigned long)v25;
  if (v25 - 0x30 <= 9) {
    v31 = 0;
    do {
      v10 = (long)(int)v31 * 10;
      v30 = (int)v10;
      if ((v30 != v10) || (v31 = (unsigned long)(unsigned int)(v30 + *a2 + -0x30), SCARRY4(v30,*a2 + -0x30)))
        v31 = 0x7fffffff;
      v32 = &a2[1];
      v19 = (unsigned long)(unsigned int)(int)*v32;
      a2 = &a2[1];
      v18 = v19;
    } while ((unsigned int)((int)*v32 - 0x30U) <= 9);
  }
  v14 = (char)v19;
  v30 = (int)v31;
  v15 = v3;
  if ((v14 == 'E') || (v14 == 'O')) {
    v14 = a2[1];
    a2 = &a2[1];
    v27 = (int)v18;
    switch(v14) {
      case 0:
        goto label_127f6;
      default:
label_12480:
        goto label_12483;
      case 0x25:
        if (!v27) goto label_1347f;
        v31 = 1;
        v32 = a2;
        goto label_124ac;
      case 0x3a:
        goto label_1287b;
      case 0x41:
        if (!v27) goto label_12846;
        break;
      case 0x42:
        if (v27 != 0x45) goto label_12829;
        break;
      case 0x43:
        if (v27 != 0x45) goto label_12af0;
        goto label_126ba;
      case 0x44:
        if (!v27) goto label_12ad8;
        break;
      case 0x46:
        if (!v27) goto label_1298d;
        break;
      case 0x47:
      case 0x56:
      case 0x67:
        if (v27 != 0x45) goto label_12593;
        goto label_12483;
      case 0x48:
        if (v27 != 0x45) goto label_13449;
        break;
      case 0x49:
        if (v27 != 0x45) goto label_1342c;
        break;
      case 0x4d:
        if (v27 != 0x45) goto label_12ce4;
        break;
      case 0x4e:
        if (v27 != 0x45) goto label_12b67;
        break;
      case 0x50:
        goto label_1346a;
      case 0x52:
        goto label_13414;
      case 0x53:
        if (v27 != 0x45) goto label_133f7;
        break;
      case 0x54:
        goto label_133df;
      case 0x55:
        if (v27 != 0x45) goto label_133a3;
        break;
      case 0x57:
        if (v27 != 0x45) goto label_13339;
        break;
      case 0x58:
        if (v27 != 0x4f) goto label_13326;
        break;
      case 0x59:
        if (v27 == 0x45) goto label_126ba;
        if (v27 != 0x4f) goto label_12d21;
        break;
      case 0x5a:
        goto label_13256;
      case 0x61:
        if (!v27) goto label_12962;
        break;
      case 0x62:
      case 0x68:
        goto label_12462;
      case 99:
        if (v27 != 0x4f) goto label_12819;
        break;
      case 100:
        if (v27 != 0x45) goto label_13238;
        break;
      case 0x65:
        if (v27 != 0x45) goto label_13222;
        break;
      case 0x6a:
        if (v27 != 0x45) goto label_131ed;
        break;
      case 0x6b:
        if (v27 != 0x45) goto label_131d7;
        break;
      case 0x6c:
        if (v27 != 0x45) goto label_131ae;
        break;
      case 0x6d:
        if (v27 != 0x45) goto label_13179;
        break;
      case 0x6e:
        goto label_130f7;
      case 0x70:
        goto label_130ce;
      case 0x71:
        v14 = 'q';
        if (v27 != 0x4f) goto label_1309d;
        goto label_126ba;
      case 0x72:
        goto label_127cc;
      case 0x73:
        goto label_12dd9;
      case 0x74:
        goto label_1301f;
      case 0x75:
        goto label_12fd6;
      case 0x77:
        if (v27 != 0x45) goto label_12fb5;
        break;
      case 0x78:
        if (v27 != 0x4f) goto label_12fa2;
        break;
      case 0x79:
        if (v27 != 0x45) goto label_12f1c;
        goto label_126ba;
      case 0x7a:
        goto label_12983;
      
    }
label_12490:
    v32 = a2;
    do {
      v32 = &v32[-1];
    } while (*v32 != '%');
    v31 = (unsigned long)(((int)a2 + 1) - (int)v32);
label_124ac:
    if ((v30 < 0) || (v28 == 0x2d)) goto label_136d8;
    v18 = (unsigned long)v30;
    v19 = v18;
    if (v18 <= v31)
      v19 = v31;
    goto label_124d0;
  }
  switch(v19 & 0xff) {
    case 0:
label_127f6:
      v14 = a2[-1];
      a2 = &a2[-1];
label_127fe:
      if (v14 != '%') goto label_12490;
      v31 = 1;
      v32 = a2;
      goto label_124ac;
    default:
      goto label_12483;
    case 0x25:
label_1347f:
      v7 = '%';
      if ((v28 != 0x2d) && (0 <= v30)) {
        v19 = (unsigned long)v30;
        v31 = 1;
        if (v19)
          v31 = v19;
        if ((unsigned long)(a1 - v29) <= v31) goto label_12338;
        if (a0) {
          if (2 <= v30) {
            v19 -= 1;
            v32 = &a0[v19];
            if ((v28 != 0x30) && (v28 != 0x2b)) {
              memset(a0,0x20,v19);
              v7 = *a2;
              a0 = v32;
            }
            else {
              memset(a0,0x30,v19);
              v7 = *a2;
              a0 = v32;
            }
          }
label_134ea:
          *a0 = v7;
          a0 = &a0[1];
        }
      }
      else {
        if ((unsigned long)(a1 - v29) <= 1) goto label_12338;
        v31 = 1;
        if (a0) goto label_134ea;
      }
      goto label_13092;
    case 0x3a:
      v18 = 0;
label_1287b:
      v32 = &a2[1];
      v31 = 1;
      v2 = a2[1];
      while (v2 == ':') {
        v31 += 1;
        v32 = &a2[v31];
        v2 = a2[v31];
      }
      v14 = ':';
      if (v2 == 'z') goto label_1289e;
      goto label_12483;
    case 0x41:
label_12846:
      v14 = 'A';
      if (v33)
        v7 = v33;
      goto label_1285c;
    case 0x42:
      v18 = 0;
label_12829:
      v14 = 'B';
      if (v33)
        v7 = v33;
      v20 = 0;
      goto label_127ce;
    case 0x43:
      v18 = 0;
label_12af0:
      v27 = *(int *)&a3->field_0x14;
      v33 = v27 < -0x76c;
      v15 = (int)((-(unsigned int)(v27 + 0x76cU < 0x76c) & 0xffffff9d) + v27) / 100 + 0x13;
      v8 = -0x76c <= v27;
      if (!v28)
        v28 = a5;
      if (v28 == 0x2b) {
        v35._0_4_ = 2;
        v14 = 'C';
        v25 = 99;
        goto label_1395e;
      }
      v20 = 0;
      v14 = 'C';
      v35._0_4_ = 2;
      goto label_126a0;
    case 0x44:
label_12ad8:
      v27 = -1;
      v32 = "%m/%d/%y";
      goto label_129b0;
    case 0x46:
label_1298d:
      if ((v28) || (0 <= v30)) {
        v27 = v30 + -6;
        v32 = "%Y-%m-%d";
        if (v27 <= -1)
          v27 = 0;
        goto label_129b0;
      }
      v27 = 4;
      v31 = sub_12210(0,0xffffffffffffffff,"%Y-%m-%d",a3,(unsigned char)v7,0x2b,4,v6,a8);
      v32 = "%Y-%m-%d";
      v19 = 0;
      v28 = 0x2b;
      v36 = v31;
      goto label_12a2b;
    case 0x47:
    case 0x56:
    case 0x67:
      v18 = 0;
label_12593:
      v27 = *(int *)&a3->field_0x14;
      v15 = v27 + -100 + (v27 >> 0x1f & 400U);
      v12 = *(int *)&a3->field_0x1c;
      v17 = sub_12130(v12,*(unsigned int *)&a3->field_0x18);
      if (0 <= v17) { // branch-flip
        v16 = 0x16d;
        if (!(v15 & 3)) {
          v16 = 0x16e;
          v15 = v15 * -0x3d70a3d7 + 0x51eb850;
          if ((v15 >> 2 | v27 * 0x40000000) <= 0x28f5c28)
            v16 = ((v15 >> 4 | (v27 + -100) * -0x70000000) < 0xa3d70b) + 0x16d;
        }
        v12 = sub_12130(v12 - v16);
        if (0 <= v12)
          v17 = v12;
        v12 = (v12 >> 0x1f) + 1;
      }
      else {
        v15 -= 1;
        v17 = 0x16d;
        if ((!(v15 & 3)) && (v17 = 0x16e, (v15 * -0x3d70a3d7 + 0x51eb850 >> 2 | v15 * 0x40000000) <= 0x28f5c28))
          v17 = 0x16e - (unsigned int)((int)v15 % 400 != 0);
        v17 = sub_12130(v12 + v17);
        v12 = -1;
      }
      if (v14 != 'G') {
        if (v14 == 'g') {
          v15 = (v27 % 100 + v12) % 100;
          if ((int)v15 < 0) {
            v15 = (0xfffff894U - v12 <= v27) ? v15 + 100 : -v15; // branch-flip
          }
          v2 = v14;
          if (v28) goto label_12f70;
          v28 = a5;
          if (a5 != 0x2b) goto label_1268b;
          goto label_12f76;
        }
        v35._0_4_ = 2;
        v15 = v17 / 7 + 1;
        goto label_12d00;
      }
      v15 = v27 + 0x76c + v12;
      v33 = v27 < 0xfffff894U - v12;
      if (v28) goto label_13947;
      if (a5 == 0x2b) {
        v35._0_4_ = 4;
        v25 = 9999;
        goto label_1395e;
      }
      v20 = 0;
      v35._0_4_ = 4;
      v8 = 0xfffff894U - v12 <= v27;
      v28 = a5;
      goto label_126a0;
    case 0x48:
      v18 = 0;
label_13449:
      v35._0_4_ = 2;
      v14 = 'H';
      v15 = *(unsigned int *)&a3->field_0x8;
      break;
    case 0x49:
      v18 = 0;
label_1342c:
      v35._0_4_ = 2;
      v14 = 'I';
      break;
    case 0x4d:
      v18 = 0;
label_12ce4:
      v35._0_4_ = 2;
      v14 = 'M';
      v15 = *(unsigned int *)&a3->field_0x4;
      break;
    case 0x4e:
label_12b67:
      v12 = 9;
      v27 = a8;
      if (v30 <= 0)
        v30 = 9;
      do {
        if (v12 <= v30) {
          if (v12 == 1) {
            v31 = 1;
            v19 = 1;
            goto label_12bdd;
          }
          if (v27 % 10) goto label_12bcf;
        }
        v12 -= 1;
        v27 /= 10;
      } while( true );
    case 0x50:
      v18 = 0;
label_1346a:
      v20 = 1;
      goto label_130d0;
    case 0x52:
label_13414:
      v27 = -1;
      v32 = "%H:%M";
      goto label_129b0;
    case 0x53:
      v18 = 0;
label_133f7:
      v35._0_4_ = 2;
      v14 = 'S';
      v15 = *(unsigned int *)a3;
      break;
    case 0x54:
label_133df:
      v27 = -1;
      v32 = "%H:%M:%S";
label_129b0:
      v31 = sub_12210(0,0xffffffffffffffff,v32,a3,(unsigned char)v7,v28,v27,v6,a8);
      if ((v28 != 0x2d) && (0 <= v30)) {
        v19 = (unsigned long)v30;
        v36 = v19;
        if (v19 <= v31)
          v36 = v31;
      }
      else {
        v36 = v31;
        v19 = 0;
      }
label_12a2b:
      if ((unsigned long)(a1 - v29) <= v36) goto label_12338;
      if (a0) {
        v26 = a0;
        if (v31 < v19) {
          v19 -= v31;
          v26 = &a0[v19];
          if ((v28 != 0x30) && (v28 != 0x2b))
            memset(a0,0x20,v19);
          else {
            memset(a0,0x30,v19);
          }
        }
        a0 = &v26[v31];
        sub_12210(v26,a1 - v29,v32,a3,(unsigned char)v7,v28,v27,v6,a8);
      }
      v29 += v36;
      goto label_122f9;
    case 0x55:
      v18 = 0;
label_133a3:
      v35._0_4_ = 2;
      v14 = 'U';
      v15 = ((*(int *)&a3->field_0x1c - *(int *)&a3->field_0x18) + 7) / 7;
      break;
    case 0x57:
      v18 = 0;
label_13339:
      v35._0_4_ = 2;
      v14 = 'W';
      v27 = *(int *)&a3->field_0x18 + 6;
      v15 = (((v27 / 7) * 7 - v27) + 7 + *(int *)&a3->field_0x1c) / 7;
      break;
    case 0x58:
      v18 = 0;
label_13326:
      v14 = 'X';
      v20 = 0;
      goto label_127ce;
    case 0x59:
      v18 = 0;
label_12d21:
      v14 = 'Y';
      v33 = *(int *)&a3->field_0x14 < -0x76c;
      v15 = *(int *)&a3->field_0x14 + 0x76c;
      if (v28) {
label_13947:
        v35._0_4_ = 4;
        v25 = 9999;
        if (v28 != 0x2b) {
          v20 = 0;
          v8 = (bool)(v33 ^ 1);
          goto label_126a0;
        }
label_1395e:
        if (v15 <= v25) goto label_12f81;
        v20 = 1;
        v28 = 0x2b;
        v8 = (bool)(v33 ^ 1);
        goto label_126a0;
      }
      if (a5 == 0x2b) {
        v35._0_4_ = 4;
        v14 = 'Y';
        v25 = 9999;
        goto label_1395e;
      }
      v20 = 0;
      v25 = 0;
      v35._0_4_ = 4;
      v28 = a5;
      goto label_12d70;
    case 0x5a:
label_13256:
      if (v33)
        v7 = 0;
      v31 = strlen(v9);
      if ((v28 != 0x2d) && (0 <= v30)) {
        v19 = (unsigned long)v30;
        v35 = v19;
        if (v19 <= v31)
          v35 = v31;
      }
      else {
        v35 = v31;
        v19 = 0;
      }
      if ((unsigned long)(a1 - v29) <= v35) goto label_12338;
      if (a0) {
        v32 = a0;
        if (v31 < v19) {
          v19 -= v31;
          v32 = &a0[v19];
          if ((v28 != 0x30) && (v28 != 0x2b))
            memset(a0,0x20,v19);
          else {
            memset(a0,0x30,v19);
          }
        }
        if (v33) // branch-flip
          sub_121c0(v32,v9,v31);
        else if (v7) // branch-flip
          sub_12170(v32,v9,v31);
        else {
          memcpy(v32,v9,v31);
        }
        a0 = &v32[v31];
      }
      v29 += v35;
      goto label_122f9;
    case 0x61:
label_12962:
      v14 = 'a';
      if (v33)
        v7 = v33;
label_1285c:
      v20 = 0;
      v32 = (char *)((long)&v21 + 5);
      goto label_126d9;
    case 0x62:
    case 0x68:
      v18 = 0;
label_12462:
      if (v33)
        v7 = v33;
      if ((int)v18 == 0x45) goto label_12480;
      goto label_127cc;
    case 99:
      v18 = 0;
label_12819:
      v14 = 'c';
      v20 = 0;
      goto label_127ce;
    case 100:
      v18 = 0;
label_13238:
      v35._0_4_ = 2;
      v14 = 'd';
      v15 = *(unsigned int *)&a3->field_0xc;
      break;
    case 0x65:
      v18 = 0;
label_13222:
      v14 = 'e';
      v15 = *(unsigned int *)&a3->field_0xc;
      goto label_131b8;
    case 0x6a:
      v18 = 0;
label_131ed:
      v20 = 0;
      v14 = 'j';
      v35._0_4_ = 3;
      v27 = *(int *)&a3->field_0x1c;
      v15 = v27 + 1;
      v33 = v27 < -1;
      v8 = -1 <= v27;
      v25 = 0;
      goto label_126a8;
    case 0x6b:
      v18 = 0;
label_131d7:
      v14 = 'k';
      v15 = *(unsigned int *)&a3->field_0x8;
      goto label_131b8;
    case 0x6c:
      v18 = 0;
label_131ae:
      v14 = 'l';
label_131b8:
      v35._0_4_ = 2;
      if (!v28)
        v28 = 0x5f;
      break;
    case 0x6d:
      v18 = 0;
label_13179:
      v20 = 0;
      v14 = 'm';
      v35._0_4_ = 2;
      v27 = *(int *)&a3->field_0x10;
      v15 = v27 + 1;
      v33 = v27 < -1;
      v8 = -1 <= v27;
      v25 = 0;
      goto label_126a8;
    case 0x6e:
label_130f7:
      if ((v28 != 0x2d) && (0 <= v30)) {
        v19 = (unsigned long)v30;
        v31 = 1;
        if (v19)
          v31 = v19;
        if ((unsigned long)(a1 - v29) <= v31) goto label_12338;
        if (a0) {
          if (2 <= v30) {
            v19 -= 1;
            v32 = &a0[v19];
            if ((v28 != 0x30) && (v28 != 0x2b)) {
              memset(a0,0x20,v19);
              a0 = v32;
            }
            else {
              memset(a0,0x30,v19);
              a0 = v32;
            }
          }
label_13166:
          *a0 = '\n';
          a0 = &a0[1];
        }
      }
      else {
        if ((unsigned long)(a1 - v29) <= 1) goto label_12338;
        v31 = 1;
        if (a0) goto label_13166;
      }
      goto label_13092;
    case 0x70:
      v18 = 0;
label_130ce:
      v20 = 0;
label_130d0:
      v14 = 'p';
      if (v33) {
        v7 = 0;
        v20 = v33;
      }
      goto label_127ce;
    case 0x71:
label_1309d:
      v20 = 0;
      v33 = 0;
      v25 = 0;
      v35._0_4_ = 1;
      v31 = (unsigned long)((*(int *)&a3->field_0x10 * 0xb >> 5) + 1);
      goto label_12d7a;
    case 0x72:
      v18 = 0;
label_127cc:
      v20 = 0;
      goto label_127ce;
    case 0x73:
label_12dd9:
      v40 = *(unsigned long *)&a3->field_0x20;
      v41 = *(unsigned long *)&a3->field_0x28;
      v24 = *(unsigned long *)a3;
      v37 = *(unsigned long *)&a3->field_0x8;
      v38 = *(unsigned long *)&a3->field_0x10;
      v42 = *(unsigned long *)&a3->field_0x30;
      v39._0_4_ = (unsigned int)*(unsigned long *)&a3->field_0x18;
      v39 = CONCAT44(0xffffffff,(unsigned int)v39);
      v10 = sub_16b20(v6,&v24);
      if (v39 <= -1) {
        *v34 = 0x4b;
        return 0;
      }
      v33 = (bool)((unsigned long)v10 >> 0x3f);
      v32 = &v23[1];
      v13 = v10;
      do {
        v5 = v13 / 10;
        v14 = (char)v13 + (char)v5 * '\xf6';
        if (v10 <= -1)
          v14 = -v14;
        v32 = &v32[-1];
        *v32 = v14 + '0';
        v13 = v5;
      } while (v5);
      v20 = 0;
      v35._0_4_ = 1;
      v15 = v28;
      goto label_12ec0;
    case 0x74:
label_1301f:
      if ((v28 != 0x2d) && (0 <= v30)) {
        v19 = (unsigned long)v30;
        v31 = 1;
        if (v19)
          v31 = v19;
        if ((unsigned long)(a1 - v29) <= v31) goto label_12338;
        if (a0) {
          if (2 <= v30) {
            v19 -= 1;
            v32 = &a0[v19];
            if ((v28 != 0x30) && (v28 != 0x2b)) {
              memset(a0,0x20,v19);
              a0 = v32;
            }
            else {
              memset(a0,0x30,v19);
              a0 = v32;
            }
          }
label_1308a:
          *a0 = '\t';
          a0 = &a0[1];
        }
      }
      else {
        if ((unsigned long)(a1 - v29) <= 1) goto label_12338;
        v31 = 1;
        if (a0) goto label_1308a;
      }
label_13092:
      v29 += v31;
      goto label_122f9;
    case 0x75:
      v18 = 0;
label_12fd6:
      v35._0_4_ = 1;
      v14 = 'u';
      v15 = (*(int *)&a3->field_0x18 + 6) % 7 + 1;
      break;
    case 0x77:
      v18 = 0;
label_12fb5:
      v35._0_4_ = 1;
      v14 = 'w';
      v15 = *(unsigned int *)&a3->field_0x18;
      break;
    case 0x78:
      v18 = 0;
label_12fa2:
      v14 = 'x';
      v20 = 0;
label_127ce:
      if ((int)v18) goto label_126c9;
      v32 = (char *)((long)&v21 + 5);
      goto label_126d9;
    case 0x79:
      v18 = 0;
label_12f1c:
      v25 = *(int *)&a3->field_0x14 % 100;
      v15 = v25;
      if ((int)v25 <= -1) {
        v15 = v25 + 100;
        if (*(int *)&a3->field_0x14 <= -0x76d)
          v15 = -v25;
      }
      v14 = 'y';
      v2 = v14;
      if (!v28) {
        v14 = 'y';
        v28 = a5;
        v2 = 'y';
      }
label_12f70:
      if (v28 != 0x2b) { // branch-flip
label_1268b:
        v20 = 0;
        v8 = 1;
        v33 = 0;
        v35._0_4_ = 2;
      }
      else {
label_12f76:
        v14 = v2;
        v35._0_4_ = 2;
        v33 = 0;
label_12f81:
        v20 = (int)v35 < v30;
        v8 = (bool)(v33 ^ 1);
        v28 = 0x2b;
      }
label_126a0:
      v25 = 0;
      goto label_126a8;
    case 0x7a:
      v18 = 0;
label_12983:
      v31 = 0;
      v32 = a2;
label_1289e:
      a2 = v32;
      if (*(int *)&a3->field_0x20 < 0) goto label_122f9;
      v33 = 1;
      v27 = (int)*(unsigned long *)&a3->field_0x28;
      if ((0 <= v27) && (v33 = 0, !v27))
        v33 = *v9 == '-';
      v15 = v27 / 0xe10;
      v12 = (v27 / 0x3c) % 0x3c;
      if (v31 != 2) { // branch-flip
        if (3 <= v31) { // branch-flip
          if (v31 != 3) {
            v14 = *v32;
            goto label_127fe;
          }
          if (v27 % 0x3c) goto label_13ab8;
          if (v12) goto label_1293d;
          v20 = 1;
          v35._0_4_ = 3;
          v8 = (bool)(v33 ^ 1);
          v25 = 0;
        }
        else if (v31) { // branch-flip
label_1293d:
          v20 = 1;
          v35._0_4_ = 6;
          v8 = (bool)(v33 ^ 1);
          v15 = v15 * 100 + v12;
          v25 = 4;
        }
        else {
          v20 = 1;
          v35._0_4_ = 5;
          v8 = (bool)(v33 ^ 1);
          v15 = v15 * 100 + v12;
          v25 = 0;
        }
      }
      else {
label_13ab8:
        v20 = 1;
        v35._0_4_ = 9;
        v25 = 0x14;
        v15 = v27 % 0x3c + v15 * 10000 + v12 * 100;
        v8 = (bool)(v33 ^ 1);
      }
      goto label_126a8;
    
  }
label_12d00:
  v20 = 0;
  v25 = 0;
  v11 = (unsigned char)(v15 >> 0x18);
  v33 = (bool)(v11 >> 7);
  v8 = (bool)((unsigned char)~v11 >> 7);
label_126a8:
  if (((int)v18 != 0x4f) || (!v8)) {
label_12d70:
    v31 = (unsigned long)v15;
    if (v33)
      v31 = (unsigned long)-v15;
label_12d7a:
    v26 = &v23[1];
    while( true ) {
      if (v25 & 1) {
        v26[-1] = ':';
        v26 = &v26[-1];
      }
      v25 = (int)v25 >> 1;
      v32 = &v26[-1];
      v26[-1] = (char)v31 + (char)(v31 / 10) * '\xf6' + '0';
      if (((unsigned int)v31 <= 9) && (v15 = v28, !v25)) break;
      v31 /= 10;
      v26 = v32;
    }
label_12ec0:
    v28 = v15;
    if (!v15)
      v28 = 0x30;
    if (v30 <= -1)
      v30 = (int)v35;
    v27 = ((int)v23 + 1) - (int)v32;
    if (v33) { // branch-flip
      v14 = '-';
label_1370d:
      v12 = v30 + -1;
      v17 = v12 - v27;
      if ((v17 <= 0) || (v15 == 0x2d))
        v17 = 0;
      if (v28 != 0x5f) { // branch-flip
        if ((unsigned long)(a1 - v29) <= 1) goto label_12338;
        if (a0) goto label_1373f;
label_13746:
        v29 += 1;
        v31 = (unsigned long)v27;
        v30 = v12;
        if (v28 != 0x2d) goto label_13753;
label_136d8:
        v18 = 0;
        v19 = v31;
      }
      else {
        v31 = (unsigned long)v17;
        v29 += v31;
        if (a0) {
          memset(a0,0x20,v31);
          a0 = &a0[v31];
          if ((unsigned long)(a1 - v29) <= 1) goto label_12338;
          v12 = (v30 - v17) + -1;
label_1373f:
          *a0 = v14;
          a0 = &a0[1];
          goto label_13746;
        }
        if ((unsigned long)(a1 - v29) <= 1) goto label_12338;
        v29 += 1;
        v30 = (v30 - v17) + -1;
label_13753:
        v31 = (unsigned long)v27;
        v18 = 0;
        v19 = v31;
        if (0 <= v30) goto label_12f04;
      }
    }
    else {
      if (v20) {
        v14 = '+';
        goto label_1370d;
      }
      v31 = (unsigned long)v27;
      if (((v27 >= v30) || (v15 == 0x2d)) && (v28 == 0x2d)) goto label_136d8;
label_12f04:
      v31 = (unsigned long)v27;
      v18 = (unsigned long)v30;
      v19 = v31;
      if (v31 <= v18)
        v19 = v18;
    }
label_124d0:
    if ((unsigned long)(a1 - v29) <= v19) goto label_12338;
    if (a0) {
      v26 = a0;
      if (v31 < v18) {
        v18 -= v31;
        v26 = &a0[v18];
        if ((v28 != 0x30) && (v28 != 0x2b))
          memset(a0,0x20,v18);
        else {
          memset(a0,0x30,v18);
        }
      }
      if (v7) // branch-flip
        sub_12170(v26,v32,v31);
      else {
        memcpy(v26,v32,v31);
      }
      a0 = &v26[v31];
    }
    v29 += v19; // crossjump-dupe
  }
  else {
label_126ba:
    v20 = 0;
label_126c9:
    v21._5_1_ = (char)v18;
    v32 = (char *)((long)&v21 + 6);
label_126d9:
    v21._3_2_ = 0x2520;
    *v32 = v14;
    v32[1] = '\0';
    v31 = strftime((char *)&v22,0x400,(char *)((long)&v21 + 3),a3);
    if (v31) {
      v31 -= 1;
      if ((v28 != 0x2d) && (0 <= v30)) {
        v18 = (unsigned long)v30;
        v19 = v18;
        if (v18 <= v31)
          v19 = v31;
      }
      else {
        v18 = 0;
        v19 = v31;
      }
      if ((unsigned long)(a1 - v29) <= v19) goto label_12338;
      if (a0) {
        v32 = a0;
        if (v31 < v18) {
          v18 -= v31;
          v32 = &a0[v18];
          if ((v28 != 0x30) && (v28 != 0x2b))
            memset(a0,0x20,v18);
          else {
            memset(a0,0x30,v18);
          }
        }
        v1 = (void *)((long)&v22 + 1);
        if (v20) // branch-flip
          sub_121c0(v32,v1,v31);
        else if (v7) // branch-flip
          sub_12170(v32,v1,v31);
        else {
          memcpy(v32,v1,v31);
        }
        a0 = &v32[v31];
      }
      v29 += v19;
    }
  }
  goto label_122f9;
label_12bcf:
  v19 = (unsigned long)v12;
  v31 = v19;
  if (v12) { // branch-flip
label_12bdd:
    v32 = (char *)((long)&v22 + v19);
    do {
      v32 = &v32[-1];
      *v32 = (char)v27 + (char)(v27 / 10) * '\xf6' + '0';
      v27 /= 10;
    } while ((char *)((long)&v21 + (v19 - (unsigned int)(v12 - 1)) + 7) != v32);
  }
  else {
    v31 = 0;
  }
  if (!v28)
    v28 = 0x30;
  if (v31 >= (unsigned long)(a1 - v29)) {
label_12338:
    *v34 = 0x22;
    return 0;
  }
  if (a0) {
    if (v7) // branch-flip
      sub_12170(a0,&v22,v31);
    else if (8 <= v31) { // branch-flip
      *(unsigned long *)a0 = CONCAT44(v22._4_4_,(unsigned int)v22);
      *(unsigned long *)&a0[v31 - 8] = *(unsigned long *)((long)&v21 + v31);
      v10 = (long)a0 - ((unsigned long)&a0[8] & 0xfffffffffffffff8);
      v19 = v10 + v31 & 0xfffffffffffffff8;
      if (8 <= v19) {
        v18 = 0;
        do {
          *(unsigned long *)(((unsigned long)&a0[8] & 0xfffffffffffffff8) + v18) = *(unsigned long *)((long)&v22 + (v18 - v10));
          v18 += 8;
        } while (v18 < v19);
      }
    }
    else if (v31 & 4) { // branch-flip
      *(unsigned int *)a0 = (unsigned int)v22;
      *(unsigned int *)&a0[v31 - 4] = *(unsigned int *)((long)&v21 + v31 + 4);
    }
    else if (v31) {
      *a0 = (char)v22;
      if (v31 & 2)
        *(unsigned short *)&a0[v31 - 2] = *(unsigned short *)((long)&v21 + v31 + 6);
    }
    a0 = &a0[v31];
  }
  v29 += v31;
  if ((v28 != 0x2d) && (0 <= v30 - v12)) {
    v31 = (unsigned long)(v30 - v12);
    if (v31 >= (unsigned long)(a1 - v29)) goto label_12338;
    if (a0) { // branch-flip
      if (v31) {
        v32 = &a0[v31];
        v29 += v31;
        if ((v28 != 0x30) && (v28 != 0x2b)) {
          memset(a0,0x20,v31);
          a0 = v32;
        }
        else {
          memset(a0,0x30,v31);
          a0 = v32;
        }
      }
    }
    else {
      v29 += v31;
    }
  }
  else if (a1 == v29) goto label_12338;
label_122f9:
  v7 = a2[1];
  a2 = &a2[1];
  v31 = 0xffffffffffffffff;
  if (!v7) goto label_1230c;
  goto label_122ae;
label_12483:
  goto label_12490;
}

// Function: sub_13d10 @ 0x13d10
long sub_13d10(void)
{
  return sub_12210();
}

// Function: sub_13d30 @ 0x13d30
void sub_13d30(char *a0)
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
    dat_36860 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_13dd0 @ 0x13dd0
unsigned long sub_13dd0(char *a0,char *a1)
{
  char v1;
  char v10; // stack - 0xac
  char *v11; // stack - 0xa8
  long v12; // stack - 0xa0
  char v13; // stack - 0x98
  unsigned int v14; // stack - 0x94
  unsigned long v15; // stack - 0x74
  char v16; // stack - 0x6c
  char *v17; // stack - 0x68
  long v18; // stack - 0x60
  char v19; // stack - 0x58
  unsigned char v2;
  int v20; // stack - 0x54
  char *v3; // rax
  long *v4; // rax
  char *v5; // rax
  char v6 [4]; // stack - 0xb8
  char v7 [4]; // stack - 0x78
  unsigned char *v8;
  unsigned long v9; // stack - 0xb4
  
  v3 = (char *)sub_15590(a1,2);
  v1 = *a0;
  while ((v1 && (v5 = (char *)sub_17860(a0,v3), v5))) {
    if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
      v6[0] = 0;
      v9 = 0;
      v11 = a0;
      if (v5 <= a0) { // branch-flip
label_13ef9:
        v8 = (unsigned char *)0x1;
      }
      else {
        do {
          v10 = 0;
          sub_11f60(v6);
          if ((v13) && (!v14)) {
            abort(); // no-return, return-dupe
          }
          v11 = &v11[v12];
          v10 = 0;
        } while (v11 < v5);
        if (!v13) goto label_13ef9;
        v8 = (unsigned char *)CONCAT71((undefined7)((unsigned long)v8 >> 8),iswalnum(v14) == 0);
      }
      v11 = v5;
      v6[0] = 0;
      v9 = 0;
      v17 = v3;
      v7[0] = 0;
      v15 = 0;
      while( true ) {
        v10 = 0;
        v16 = 0;
        sub_11f60(v7);
        if ((v19) && (!v20)) break;
        sub_11f60(v6);
        if ((v13) && (!v14))
          abort();
        v11 = &v11[v12];
        v17 = &v17[v18];
      }
      sub_11f60(v6);
      if ((((v13) && (!v14)) || ((!v13 || (!iswalnum(v14))))) && ((char)v8)) {
        free(v3); // return-dupe
        return (unsigned long)v8 & 0xffffffff;
      }
      v11 = v5;
      v6[0] = 0;
      v9 = 0;
      v10 = 0;
      sub_11f60(v6);
      if ((v13) && (!v14)) break;
      a0 = &v5[v12];
      v1 = *a0;
    }
    else {
      v8 = (unsigned char *)&v5[strlen(v3)];
      if (((v5 <= a0) || (v4 = __ctype_b_loc(), !(*(unsigned char *)(*v4 + (unsigned long)(unsigned char)v5[-1] * 2) & 8))) && ((v2 = *v8, !v2 || (!(*(unsigned char *)(*(long *)__ctype_b_loc() + (unsigned long)v2 * 2) & 8))))) {
        v8 = (unsigned char *)0x1;
        free(v3);
        return (unsigned long)v8 & 0xffffffff;
      }
      if (!*v5) break;
      v1 = v5[1];
      a0 = &v5[1];
    }
  }
  v8 = NULL;
  free(v3);
  return (unsigned long)v8 & 0xffffffff;
}

// Function: sub_14040 @ 0x14040
char * sub_14040(char *a0)
{
  char *v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = dcgettext(NULL,a0,5);
  if (a0 != v1) {
    if (!sub_13dd0(v1,a0)) {
      v2 = strlen(v1);
      v3 = (char *)sub_15f50(v2 + 4 + strlen(a0));
      __sprintf_chk(v3,1,0xffffffffffffffff,"%s (%s)",v1,a0);
      return v3;
    }
  }
  return v1;
}

// Function: sub_140d0 @ 0x140d0
char * sub_140d0(char *a0,char *a1)
{
  char v1; // al
  char *v10;
  char *v11; // stack - 0x48
  int v2; // eax
  char *v3; // rax
  char *v4;
  unsigned long v5; // rax
  char *v6;
  char *v7; // rax
  char *v8; // rax
  unsigned long *v9; // rbx
  
  v3 = dcgettext(NULL,a0,5);
  v4 = (char *)sub_16ff0();
  v6 = v3;
  if (sub_16d30(v4,"UTF-8")) { // branch-flip
    v7 = (char *)sub_16430(a1,"UTF-8",v4);
    v5 = strlen(v4);
    v8 = memcpy((void *)sub_15f50(v5 + 0xb),v4,v5);
    v9 = (unsigned long *)&v8[v5];
    *v9 = 0x4c534e4152542f2f;
    *(unsigned short *)&v9[1] = 0x5449;
    *(char *)((long)v9 + 10) = 0;
    a1 = (char *)sub_16430(a1,"UTF-8",v8);
    free(v8);
    v11 = v7;
    if (a1) { // branch-flip
      if (strchr(a1,0x3f)) {
        free(a1);
        goto label_142d3;
      }
      v4 = a1;
      if (!v7) {
        v10 = a1;
        goto label_1413a;
      }
      if (!strcmp(v3,a0)) {
        if (v7 == a1)
          return v7;
        free(a1);
        return v7;
      }
    }
    else {
label_142d3:
      if (!v7) {
        v11 = NULL;
        a1 = NULL;
        v10 = a0;
        v4 = NULL;
        goto label_1413a;
      }
      if (!strcmp(v3,a0))
        return v7;
      a1 = NULL;
      v4 = NULL;
    }
    if (!sub_13dd0(v3,a0)) {
      a0 = v7;
label_1415f:
      if (((!v11) || (!sub_13dd0(v3,v11))) && ((!a1 || (!sub_13dd0(v3,a1))))) goto label_141dc;
      if (!v7) goto label_1418d;
    }
    free(v7);
  }
  else {
    v2 = strcmp(v3,a0);
    if (a1) {
      v10 = a1;
      v4 = NULL;
      v11 = a1;
label_1413a:
      if (!strcmp(v3,a0))
        return v10;
      v1 = sub_13dd0(v3,a0);
      v7 = NULL;
      a0 = v10;
      if (v1) goto label_1418d;
      goto label_1415f;
    }
    if (!v2)
      return a0;
    if (sub_13dd0(v3,a0))
      return v3;
    v4 = NULL;
    v7 = NULL;
label_141dc:
    v5 = strlen(v3);
    v6 = (char *)sub_15f50(v5 + 4 + strlen(a0));
    __sprintf_chk(v6,1,0xffffffffffffffff,"%s (%s)",v3,a0);
    if (v7)
      free(v7);
  }
label_1418d:
  if (v4)
    free(v4);
  return v6;
}

// Function: sub_143d0 @ 0x143d0
void sub_143d0(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_143e0 @ 0x143e0
unsigned long sub_143e0(unsigned long a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_143d0();
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

// Function: sub_14490 @ 0x14490
unsigned long sub_14490(void)
{
  return sub_143e0(); // tail-call
}

// Function: sub_144a0 @ 0x144a0
void sub_144a0(void)
{
  sub_143d0(); // tail-call
}

// Function: sub_144b0 @ 0x144b0
void sub_144b0(void)
{
  dat_36868 = sub_19c70(NULL);
  sub_19cd0(dat_36868,1); // tail-call
}

// Function: sub_144e0 @ 0x144e0
void sub_144e0(char *a0)
{
  long v1;
  
  if (!dat_36868)
    sub_144b0();
  v1 = dat_36868;
  sub_19d90(NULL,0,a0,strlen(a0),v1); // tail-call
}

// Function: sub_14530 @ 0x14530
char * sub_14530(char *a0,char *a1)
{
  long v1;
  unsigned long v2; // rax
  
  if (!dat_36868)
    sub_144b0();
  v1 = dat_36868;
  v2 = strlen(a1);
  return &a0[sub_19d90(a0,0xffffffffffffffff,a1,v2,v1)];
}

// Function: sub_14590 @ 0x14590
void sub_14590(char *a0)
{
  long v1;
  
  if (!dat_36868)
    sub_144b0();
  v1 = dat_36868;
  sub_19f00(a0,strlen(a0),v1); // tail-call
}

// Function: sub_145e0 @ 0x145e0
char * sub_145e0(long *a0)
{
  long v1; // rax
  char *v2; // rax
  char *v3;
  long *v4;
  char *v5;
  unsigned long v6;
  
  v5 = (char *)*a0;
  if (v5) {
    v6 = 0;
    v4 = a0;
    do {
      v1 = sub_144e0(v5);
      v5 = (char *)v4[1];
      v4 = &v4[1];
      v6 = v6 + 1 + v1;
    } while (v5);
    v2 = (char *)sub_15f50(v6);
    v3 = v2;
    while( true ) {
      v5 = (char *)*a0;
      a0 = &a0[1];
      v3 = (char *)sub_14530(v3,v5);
      if (!*a0) break;
      *v3 = 0x20;
      v3 = &v3[1];
    }
    *v3 = 0;
    return v2;
  }
  return (char *)sub_162e0(""); // tail-call
}

// Function: sub_14670 @ 0x14670
void sub_14670(unsigned long a0) // return-dupe
{
  char v1 [8];
  long v2; // stack - 0x30
  
  if ((int)sub_15410(a0,v1) <= -1)
    return;
  dat_36890 = v2 + -1;
}

// Function: sub_146c0 @ 0x146c0
void sub_146c0(unsigned long a0,void *a1,void *a2)
{
  unsigned long v1;
  unsigned long v2;
  long v3;
  int v4;
  int v5; // eax
  int *v6; // rax
  unsigned long v7; // stack - 0x68
  long v8; // stack - 0x60
  void *v9; // stack - 0x58
  
  v1 = *(unsigned long *)((long)a1 + 0x10);
  if (dat_36870) { // branch-flip
    v4 = (*dat_36870)(v1,0);
    if (v4)
      return;
    if (dat_36888) goto label_1470e;
  }
  else {
    if (!dat_36888) {
      signal(0xb,0); // return-dupe, tail-call
      return;
    }
label_1470e:
    v3 = dat_36890;
    v2 = *(unsigned long *)((long)a2 + 0xa0);
    if (dat_36890) {
      v6 = __errno_location();
      v4 = *v6;
      v5 = sub_15410(v3,&v7);
      *v6 = v4;
      if (0 <= v5) {
        if (v7 <= v1) { // branch-flip
          if (v1 <= (unsigned long)(v8 - 1U)) goto label_147f5;
        }
        else if ((*v9)(v1,&v7)) {
label_147f5:
          (*dat_36888)(dat_36880 <= v2 && v2 <= dat_36880 + dat_36878,a2);
        }
      }
    }
  }
  if ((dat_36870) && (v4 = (*dat_36870)(v1,1), v4))
    return;
  signal(0xb,0);
  return;
}

// Function: sub_14850 @ 0x14850
void sub_14850(void)
{
  void *v1; // stack - 0xa8
  sigset_t v2;
  int v3; // stack - 0x20
  
  v1 = sub_146c0;
  sigemptyset(&v2);
  sigaddset(&v2,1);
  sigaddset(&v2,2);
  sigaddset(&v2,3);
  sigaddset(&v2,0xd);
  sigaddset(&v2,0xe);
  sigaddset(&v2,0xf);
  sigaddset(&v2,10);
  sigaddset(&v2,0xc);
  sigaddset(&v2,0x11);
  sigaddset(&v2,0x11);
  sigaddset(&v2,0x17);
  sigaddset(&v2,0x1d);
  sigaddset(&v2,0x1d);
  sigaddset(&v2,0x18);
  sigaddset(&v2,0x19);
  sigaddset(&v2,0x1a);
  sigaddset(&v2,0x1b);
  sigaddset(&v2,0x1e);
  sigaddset(&v2,0x1c);
  v3 = (-(unsigned int)(dat_36888 == 0) & 0xf8000000) + 0x8000004;
  sigaction(0xb,(sigaction *)&v1,NULL);
}

// Function: sub_149d0 @ 0x149d0
unsigned long sub_149d0(unsigned long a0)
{
  dat_36870 = a0;
  sub_14850();
  return 0;
}

// Function: sub_149f0 @ 0x149f0
void sub_149f0(void)
{
  dat_36870 = 0;
  if (dat_36888)
    return;
  signal(0xb,0); // tail-call
}

// Function: sub_14a20 @ 0x14a20
unsigned long sub_14a20(void *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  (*a0)(a1,a2,a3);
  return 1;
}

// Function: sub_14a50 @ 0x14a50
unsigned long sub_14a50(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  
  if ((!dat_36890) && (sub_14670(&v1), !dat_36890))
    return 0xffffffff;
  v2 = 0;
  dat_36878 = a2;
  dat_36880 = a1;
  dat_36888 = a0;
  v1 = a1;
  v3 = a2;
  if (sigaltstack(&v1,NULL) <= -1)
    return 0xffffffff;
  sub_14850();
  return 0;
}

// Function: sub_14b00 @ 0x14b00
void sub_14b00(void) // return-dupe
{
  char v1 [8];
  unsigned int v2; // stack - 0x20
  
  dat_36888 = 0;
  if (dat_36870) // branch-flip
    sub_14850();
  else {
    signal(0xb,0);
  }
  v2 = 2;
  if (0 <= sigaltstack(v1,NULL))
    return;
  perror("gnulib sigsegv (stackoverflow_deinstall_handler)");
}

// Function: sub_14b90 @ 0x14b90
bool sub_14b90(long a0,struct_0 *a1)
{
  return (unsigned long)(a1->field_0x0 - a0) <= (unsigned long)(a1->field_0x0 - a1->field_0x18) >> 1;
}

// Function: sub_14bb0 @ 0x14bb0
unsigned long sub_14bb0(struct_1 *a0)
{
  if (a0->field_0x0 != a0->field_0x8)
    return (unsigned long)*(unsigned char *)(a0->field_0x18 + a0->field_0x0);
  a0->field_0x10 = 1;
  return 0xffffffff;
}

// Function: sub_14bd0 @ 0x14bd0
void sub_14bd0(struct_1 *a0) // return-dupe
{
  int v1; // eax
  
  v1 = sub_14bb0(a0);
  if (v1 <= -1)
    return;
  a0->field_0x0 = a0->field_0x0 + 1;
}

// Function: sub_14be0 @ 0x14be0
unsigned long sub_14be0(unsigned long a0,long *a1)
{
  int v1; // eax
  long v2; // rcx
  int v3; // esi
  
  v2 = 0;
  v3 = 0;
  do {
    v1 = sub_14bb0();
    if (10 <= (unsigned int)(v1 - 0x30U)) { // branch-flip
      if (6 <= (unsigned int)(v1 - 0x41U)) { // branch-flip
        if (6 <= (unsigned int)(v1 - 0x61U)) {
          if (v3) {
            *a1 = v2;
            return 0;
          }
          return 0xffffffff;
        }
        v2 = v2 * 0x10 + (long)(v1 + -0x57);
      }
      else {
        v2 = v2 * 0x10 + (long)(v1 + -0x37);
      }
    }
    else {
      v2 = v2 * 0x10 + (long)(v1 - 0x30U);
    }
    sub_14bd0();
    v3 += 1;
  } while( true );
}

// Function: sub_14c50 @ 0x14c50
unsigned long sub_14c50(struct_2 *a0,unsigned long a1,unsigned long a2)
{
  unsigned long *v1;
  unsigned long v2;
  
  if ((a1 <= a0->field_0x0) && (a0->field_0x0 <= a2 - 1)) {
    v1 = a0->field_0x8;
    v1[1] = a2;
    v2 = a0->field_0x10;
    *v1 = a1;
    v1[3] = v2;
    a0->field_0x18 = 0;
    return 1;
  }
  a0->field_0x10 = a2;
  return 0;
}

// Function: sub_14c90 @ 0x14c90
void sub_14c90(void)
{
  dat_36898 = (long)getpagesize();
}

// Function: sub_14cb0 @ 0x14cb0
void sub_14cb0(void *a0)
{
  if (*(void **)((long)a0 + 0x20)) {
    munmap(*(void **)((long)a0 + 0x20),*(unsigned long *)((long)a0 + 0x28)); // tail-call
    return;
  }
}

// Function: sub_14cd0 @ 0x14cd0
unsigned int sub_14cd0(void *a0)
{
  unsigned int v1; // eax
  long v2;
  
  v1 = mincore(a0,dat_36898,&v2);
  return ~v1 >> 0x1f;
}

// Function: sub_14d20 @ 0x14d20
unsigned long sub_14d20(unsigned long a0,unsigned long a1) // return-dupe
{
  unsigned long v1;
  long v2; // rax
  unsigned long v3; // rax
  unsigned long v4;
  void *v5;
  unsigned long v6; // r14
  unsigned long v7;
  
  v1 = dat_36898;
  v2 = (a0 / dat_36898) * dat_36898;
  v3 = ((a1 / dat_36898 + 1) * dat_36898 - v2) / dat_36898;
  if (v3 <= 1)
    return 1;
  v4 = 1;
  do {
    v4 *= 2;
  } while (v4 < v3);
  do {
    v6 = v4 >> 1;
    v5 = (void *)(v2 + v1 * v6);
    if (v6 < v3) {
      v7 = v6;
      do {
        if (sub_14cd0(v5))
          return 0;
        v7 += v4 & 0xfffffffffffffffe;
        v5 = (void *)((long)v5 + v1 * v6 * 2);
      } while (v7 < v3);
    }
    v4 = v6;
  } while (2 <= v6);
  return 1;
}

// Function: sub_14e10 @ 0x14e10
unsigned long sub_14e10(unsigned long a0,long *a1)
{
  long v1;
  unsigned long v2; // rdi
  
  v1 = *a1;
  v2 = a0 * 2 - v1;
  if (a0 < v2)
    return 0;
  return sub_14d20(v2,v1 - 1); // tail-call
}

// Function: sub_14e40 @ 0x14e40
void * sub_14e40(void *a0)
{
  unsigned long v1;
  unsigned long v2;
  char v3 [1032];
  unsigned long v4; // rsi
  unsigned long v5; // rsi
  void *v6; // r14
  
  v2 = 0x400;
  while( true ) {
    v1 = dat_36898;
    if (!a0)
      return NULL;
    if ((unsigned long)a0 / dat_36898 < v2)
      v2 = (unsigned long)a0 / dat_36898;
    v4 = dat_36898 * v2;
    v6 = (void *)((long)a0 - v4);
    if (mincore(v6,v4,v3) < 0) break;
    a0 = v6;
  }
  do {
    if (v2 == 1)
      return a0;
    while( true ) {
      v4 = v2 + 1 >> 1;
      v5 = v1 * v4;
      v6 = (void *)((long)a0 - v5);
      if (mincore(v6,v5,v3) < 0) break;
      v2 >>= 1;
      a0 = v6;
      if (v2 == 1)
        return v6;
    }
    v2 = v4;
  } while( true );
}

// Function: sub_14f20 @ 0x14f20
void * sub_14f20(long a0) // return-dupe
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  char v4 [1032];
  void *v5; // rdi
  
  v1 = dat_36898;
  v5 = (void *)(a0 + dat_36898);
  if (!v5)
    return NULL;
  v2 = 0x400;
  while( true ) {
    if ((unsigned long)-(long)v5 / v1 < v2)
      v2 = (unsigned long)-(long)v5 / v1;
    if (mincore(v5,v1 * v2,v4) <= -1) break;
    v5 = (void *)((long)v5 + v1 * v2);
    if (!v5)
      return NULL;
  }
  do {
    v3 = v2;
    if (v2 == 1)
      return v5;
    while( true ) {
      v2 = v3 + 1 >> 1;
      if (mincore(v5,v1 * v2,v4) < 0) break;
      v3 >>= 1;
      v5 = (void *)((long)v5 + v1 * v2);
      if (v3 == 1)
        return v5;
    }
  } while( true );
}

// Function: sub_15020 @ 0x15020
unsigned long sub_15020(unsigned long a0,unsigned long *a1)
{
  long v1; // rbp
  
  if (!dat_36898)
    sub_14c90();
  v1 = (a0 / dat_36898) * dat_36898;
  *a1 = sub_14e40(v1);
  a1[1] = sub_14f20(v1);
  a1[2] = sub_14e10;
  return 0;
}

// Function: sub_15090 @ 0x15090
unsigned long sub_15090(struct_3 *a0) // return-dupe x2
{
  int v1; // eax
  int v2; // eax
  void *v3;
  unsigned long v4; // rax
  long v5;
  unsigned long v6; // r12
  unsigned long v7;
  
  v1 = open("/proc/self/maps",0);
  if (v1 <= -1)
    return 0xffffffff;
  a0->field_0x0 = 0;
  v7 = 1;
  a0->field_0x10 = 0;
  v6 = 0;
  a0->field_0x18 = &a0[1];
  a0->field_0x20 = NULL;
  a0->field_0x30 = 0;
  a0->field_0x38 = 0;
  do {
    if (0x104a <= v7) { // branch-flip
      v3 = a0->field_0x18;
      v2 = (int)read(v1,v3,v7);
      if (0 <= v2) {
        if (!v2) goto label_1525c;
        if (v7 < (unsigned long)(long)(v2 + 0x1049)) goto label_1512f;
        v5 = (long)v2;
        while( true ) {
          a0->field_0x8 = v5;
          v4 = read(v1,(void *)(v5 + (long)a0->field_0x18),v7 - v5);
          v2 = (int)v4;
          if (v2 <= -1) break;
          if (v7 - a0->field_0x8 < (unsigned long)(long)(v2 + 0x1049)) goto label_1512f;
          if (!v2) {
            close(v1);
            return v4 & 0xffffffff;
          }
          v5 = (long)v2 + a0->field_0x8;
        }
      }
      if (*__errno_location() != 4) {
label_1525c:
        close(v1);
        v3 = a0->field_0x20;
        goto label_1526a;
      }
    }
    else {
label_1512f:
      if (v6) { // branch-flip
        v7 *= 2;
        if (!v7) goto label_1525c;
        if (a0->field_0x20)
          munmap(a0->field_0x20,a0->field_0x28);
      }
      else {
        v6 = (unsigned long)getpagesize();
        for (v7 = v6; v7 <= 0x1049; v7 = v7 * 2) {
        }
      }
      v3 = (void *)mmap(0,v7,3,0x22,0xffffffff,0);
      a0->field_0x20 = v3;
      if (v3 == (void *)0xffffffffffffffff) {
        close(v1);
        return 0xffffffff;
      }
      a0->field_0x28 = v7;
      a0->field_0x30 = v3;
      a0->field_0x38 = v7 + (long)v3;
      a0->field_0x18 = v3;
    }
    if ((long)lseek(v1,0,0) <= -1) {
      close(v1);
      v1 = open("/proc/self/maps",0);
      if (v1 <= -1) {
        v3 = a0->field_0x20;
label_1526a:
        if (!v3)
          return 0xffffffff;
        munmap(v3,a0->field_0x28);
        return 0xffffffff;
      }
    }
  } while( true );
}

// Function: sub_152d0 @ 0x152d0
unsigned long sub_152d0(struct_2 *a0) // return-dupe
{
  unsigned long v1;
  unsigned long v10; // stack - 0x48
  unsigned long v11; // stack - 0x40
  int v2; // eax
  unsigned long v3; // stack - 0x88
  char v4 [48];
  long v5; // stack - 0x80
  long v6;
  unsigned long v7;
  unsigned long *v8;
  long *v9;
  
  v8 = &v3;
  if ((int)sub_15090(v4) <= -1)
    return 0xffffffff;
  v9 = &v5;
  v7 = v10;
label_15320:
  do {
    v2 = sub_14be0(v4,v8);
    if (((v2 < 0) || (sub_14bd0() != 0x2d)) || (v2 = sub_14be0(v9), v2 < 0)) {
      sub_14cb0(v4); // return-dupe
      return 0;
    }
    do {
      v2 = sub_14bd0(v4);
      if (v2 == -1) break;
    } while (v2 != 10);
    if (v3 <= v7) { // branch-flip
      v1 = v5 - 1;
      if (v1 < v11 - 1) goto label_153c0;
      v6 = v5;
      if ((v3 < v7) && (v2 = sub_14c50(a0,v3,v7), v2)) {
        sub_14cb0(v4);
        return 0;
      }
      if (v1 <= v11 - 1) goto label_15320;
      v2 = sub_14c50(a0,v11,v6);
    }
    else {
label_153c0:
      v2 = sub_14c50(a0,v3,v5);
    }
    if (v2) {
      sub_14cb0(v4);
      return 0;
    }
  } while( true );
}

// Function: sub_15400 @ 0x15400
unsigned long sub_15400(struct_2 *a0)
{
  return sub_152d0(a0); // tail-call
}

// Function: sub_15410 @ 0x15410
unsigned long sub_15410(unsigned long a0,unsigned long *a1)
{
  unsigned long v1; // stack - 0x48
  unsigned long *v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  int v4; // stack - 0x30
  
  v3 = 0;
  v4 = -1;
  v1 = a0;
  v2 = a1;
  sub_15400(&v1);
  if (!v4) {
    a1[2] = sub_14b90;
    return 0;
  }
  return sub_15020(a0,a1); // tail-call
}

// Function: sub_154b0 @ 0x154b0
unsigned long sub_154b0(void *a0)
{
  return *(unsigned long *)((long)a0 + 0x50);
}

// Function: sub_154c0 @ 0x154c0
unsigned long sub_154c0(void *a0)
{
  return *(unsigned long *)((long)a0 + 0x70);
}

// Function: sub_154d0 @ 0x154d0
unsigned long sub_154d0(struct_8 *a0)
{
  return a0->field_0x60;
}

// Function: sub_154e0 @ 0x154e0
unsigned long sub_154e0(void)
{
  return 0;
}

// Function: sub_154f0 @ 0x154f0
undefined16 sub_154f0(void *a0)
{
  return *(char (*)[16])((long)a0 + 0x48);
}

// Function: sub_15500 @ 0x15500
undefined16 sub_15500(void *a0)
{
  return *(char (*)[16])((long)a0 + 0x68);
}

// Function: sub_15510 @ 0x15510
undefined16 sub_15510(void *a0)
{
  return *(char (*)[16])((long)a0 + 0x58);
}

// Function: sub_15520 @ 0x15520
undefined16 sub_15520(void)
{
  char v1 [16];
  
  v1._8_8_ = 0xffffffffffffffff;
  v1._0_8_ = 0xffffffffffffffff;
  return v1._0_16_;
}

// Function: sub_15540 @ 0x15540
unsigned int sub_15540(unsigned int a0)
{
  return a0;
}

// Function: sub_15550 @ 0x15550
void * sub_15550(void *a0,unsigned long a1)
{
  void *v1; // rax
  unsigned long v2; // rdx
  bool v3; // zf
  
  v1 = memchr(a0,0,a1);
  v2 = (long)v1 + (1U - (long)a0);
  v3 = v1 != NULL;
  v1 = (void *)a1;
  if (v3)
    v1 = (void *)v2;
  return v1;
}

// Function: sub_15590 @ 0x15590
unsigned char * sub_15590(char *a0,int a1)
{
  unsigned char v1;
  char v10; // stack - 0x70
  unsigned long v11; // stack - 0x6c
  char v12; // stack - 0x64
  unsigned char *v13; // stack - 0x60
  long v14; // stack - 0x58
  char v15; // stack - 0x50
  unsigned int v16; // stack - 0x4c
  long v2;
  unsigned char *v3; // rax
  unsigned long v4; // rax
  long *v5; // rax
  unsigned char *v6;
  unsigned char *v7; // stack - 0x78
  int v8; // ebp
  unsigned char *v9;
  
  v3 = (unsigned char *)strdup(a0);
  if (!v3)
    sub_16300(); // no-return
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
    if (a1) {
      v13 = v3;
      v4 = strlen((char *)v3);
      v10 = 0;
      v7 = &v3[v4];
      v12 = 0;
      v11 = 0;
      v6 = v3;
      if (v3 < v7) {
        do {
          v12 = 0;
          sub_170e0(&v7);
          if ((!v15) || (!iswspace(v16))) {
            v6 = v13;
            break;
          }
          v13 = &v13[v14];
          v12 = 0;
          v6 = v13;
        } while (v13 < v7);
      }
      memmove(v3,v6,strlen((char *)v6) + 1);
      if (a1 == 1)
        return v3;
    }
    v13 = v3;
    v8 = 0;
    v4 = strlen((char *)v3);
    v10 = 0;
    v7 = &v3[v4];
    v11 = 0;
    if (v3 < v7) {
      do {
        v12 = 0;
        sub_170e0(&v7);
        if (v8) { // branch-flip
          if (v8 != 1) { // branch-flip
            if ((v8 != 2) || (!v15)) goto label_1561b;
            if (!iswspace(v16))
              v8 = 1;
          }
          else if ((((v15) && (iswspace(v16))) && (v15)) && (iswspace(v16))) {
            v8 = 2;
            v9 = v13;
          }
          else {
label_1575f:
          }
        }
        else {
          if (v15) {
            if (iswspace(v16)) goto label_1575f;
            if (v15) {
              v8 = 1;
              iswspace(v16);
              goto label_15625;
            }
          }
label_1561b:
          v8 = 1;
        }
label_15625:
        v13 = &v13[v14];
      } while (v13 < v7);
      if (v8 == 2)
        *v9 = 0;
    }
  }
  else {
    if (a1) {
      v1 = *v3;
      v6 = v3;
      if (v1) {
        v5 = __ctype_b_loc();
        do {
          if (!(*(unsigned char *)(*v5 + 1 + (unsigned long)v1 * 2) & 0x20)) break;
          v1 = v6[1];
          v6 = &v6[1];
        } while (v1);
      }
      memmove(v3,v6,strlen((char *)v6) + 1);
      if (a1 == 1)
        return v3;
    }
    v6 = &v3[strlen((char *)v3) - 1];
    if (v3 <= v6) {
      v2 = *(long *)__ctype_b_loc();
      do {
        if (!(*(unsigned char *)(v2 + 1 + (unsigned long)*v6 * 2) & 0x20))
          return v3;
        *v6 = 0;
        v6 = &v6[-1];
      } while (v3 <= v6);
    }
  }
  return v3;
}

// Function: sub_158a0 @ 0x158a0
long sub_158a0(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5)
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
  __fprintf_chk(a0,1,"Copyright %s %d Free Software Foundation, Inc.",dcgettext(NULL,"(C)",5),0x7e5);
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
label_159cd:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_15a06:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0x1e050) + 0x1e050;
  switch(a5) {
    case 0:
      goto label_15a06;
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
      goto label_159cd;
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

// Function: sub_15d20 @ 0x15d20
void sub_15d20(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_158a0(); // tail-call
}

// Function: sub_15d40 @ 0x15d40
void sub_15d40(FILE *a0,char *a1,char *a2,char *a3,struct_44 *a4)
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
  sub_158a0(a0,a1,a2,a3);
}

// Function: sub_15de0 @ 0x15de0
void sub_15de0(void)
{
  sub_15d40();
}

// Function: sub_15e90 @ 0x15e90
void sub_15e90(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-diffutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU diffutils","https://www.gnu.org/software/diffutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_15f40 @ 0x15f40
long sub_15f40(long a0)
{
  if (a0)
    return a0;
  sub_16300(); // no-return
}

// Function: sub_15f50 @ 0x15f50
void sub_15f50(unsigned long a0)
{
  sub_15f40(malloc(a0)); // tail-call
}

// Function: sub_15f70 @ 0x15f70
void sub_15f70(void)
{
  sub_15f50(); // tail-call
}

// Function: sub_15f80 @ 0x15f80
void sub_15f80(void)
{
  sub_15f40(sub_16f80()); // tail-call
}

// Function: sub_15fa0 @ 0x15fa0
void sub_15fa0(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_16300(); // no-return
}

// Function: sub_15fe0 @ 0x15fe0
void sub_15fe0(void)
{
  sub_15f40(sub_16f90()); // tail-call
}

// Function: sub_16000 @ 0x16000
void sub_16000(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_16300(); // no-return
}

// Function: sub_16040 @ 0x16040
void sub_16040(unsigned long a0,unsigned long a1)
{
  sub_16000(NULL,a0,a1); // tail-call
}

// Function: sub_16050 @ 0x16050
void sub_16050(void *a0,unsigned long a1,unsigned long a2)
{
  sub_16000(a0,a1,a2); // tail-call
}

// Function: sub_16060 @ 0x16060
void sub_16060(void)
{
  sub_15f40(sub_16fc0()); // tail-call
}

// Function: sub_16080 @ 0x16080
void sub_16080(void *a0,unsigned long *a1,unsigned long a2)
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
      sub_16300(); // no-return
  }
  else if (!v3) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v3 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0) + (unsigned long)(0x81 <= a2);
  }
  sub_16000(a0,v3,a2);
  *a1 = v3;
}

// Function: sub_160f0 @ 0x160f0
void sub_160f0(void *a0,unsigned long *a1)
{
  sub_16080(a0,a1,1); // tail-call
}

// Function: sub_16100 @ 0x16100
void sub_16100(void *a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_161a6;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_161a6:
  if (!a0)
    *a1 = 0;
  if ((v2 - v1 < a2) && ((v2 = v1 + a2, SCARRY8(v1,a2) || (((a3 < v2 && (0 <= a3)) || (v3 = v2 * a4, SEXT816(v3) != SEXT816(v2) * SEXT816(a4)))))))
    sub_16300(a0,v3); // no-return
  sub_15fa0(a0,v3);
  *a1 = v2;
}

// Function: sub_161e0 @ 0x161e0
void sub_161e0(unsigned long a0,unsigned long a1)
{
  sub_15f40(calloc(a0,a1)); // tail-call
}

// Function: sub_16200 @ 0x16200
void sub_16200(unsigned long a0)
{
  sub_161e0(a0,1); // tail-call
}

// Function: sub_16210 @ 0x16210
void sub_16210(void)
{
  sub_15f40(sub_16fb0()); // tail-call
}

// Function: sub_16230 @ 0x16230
void sub_16230(unsigned long a0)
{
  sub_16210(a0,1); // tail-call
}

// Function: sub_16240 @ 0x16240
void sub_16240(void *a0,unsigned long a1)
{
  memcpy((void *)sub_15f50(a1),a0,a1); // tail-call
}

// Function: sub_16270 @ 0x16270
void sub_16270(void *a0,unsigned long a1)
{
  memcpy((void *)sub_15f80(a1),a0,a1); // tail-call
}

// Function: sub_162a0 @ 0x162a0
void sub_162a0(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_15f80(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_162e0 @ 0x162e0
void sub_162e0(char *a0)
{
  sub_16240(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_16300 @ 0x16300
void sub_16300(void)
{
  error(dat_260b0,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_16340 @ 0x16340
void sub_16340(unsigned long a0)
{
  if (sub_11c90(a0))
    return;
  sub_16300(); // no-return
}

// Function: sub_16360 @ 0x16360
long sub_16360(char *a0)
{
  long v1; // rax
  
  v1 = sub_16c70(a0);
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_16300(); // no-return
  }
  return v1;
}

// Function: sub_16390 @ 0x16390
void sub_16390(void)
{
  int v1; // eax
  
  v1 = sub_1a3a0();
  if (!v1)
    return;
  error(dat_260b0,v1,dcgettext(NULL,"standard file descriptors",5)); // tail-call
}

// Function: sub_163d0 @ 0x163d0
int sub_163d0(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4)
{
  int v1; // eax
  
  v1 = sub_1a450(a0,a1,a2,a3,a4);
  if (v1 < 0) {
    if (*__errno_location() == 0xc)
      sub_16300(); // no-return
  }
  return v1;
}

// Function: sub_16400 @ 0x16400
long sub_16400(void)
{
  long v1; // rax
  
  v1 = sub_1a6d0();
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_16300(); // no-return
  }
  return v1;
}

// Function: sub_16430 @ 0x16430
long sub_16430(char *a0,char *a1,char *a2)
{
  long v1; // rax
  
  v1 = sub_1a920(a0,a1,a2);
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_16300(); // no-return
  }
  return v1;
}

// Function: sub_16460 @ 0x16460
void sub_16460(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,char *a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
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
  unsigned int v2; // stack - 0xd8
  char v3 [8];
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
  v5 = &Stack0000000000000008;
  v2 = 8;
  v6 = v3;
  v4 = 0x30;
  v7 = a9;
  v8 = a10;
  v9 = a11;
  v10 = a12;
  v11 = a13;
  sub_1ab40(a8,&v2);
}

// Function: sub_16520 @ 0x16520
long sub_16520(int a0,void *a1,long a2)
{
  void *v1;
  unsigned long v2;
  long v3; // rax
  unsigned long v4; // r14
  void *v5;
  
  v4 = 0x7fffffffffffffff;
  v1 = (void *)((long)a1 + a2);
  v5 = a1;
  do {
    while( true ) {
      v2 = (long)v1 - (long)v5;
      if (v4 < (unsigned long)((long)v1 - (long)v5))
        v2 = v4;
      v3 = read(a0,v5,v2);
      if (0 < v3) break;
      if (!v3) {
        return (long)v5 - (long)a1;
      }
      if ((*__errno_location() != 0x16) || (v2 < 0x80000000))
        return -1;
      v4 = 0x7fffffff;
      if (v1 <= v5) {
        return (long)v5 - (long)a1;
      }
    }
    v5 = (void *)((long)v5 + v3);
  } while (v5 < v1);
  return (long)v5 - (long)a1;
}

// Function: sub_165d0 @ 0x165d0
unsigned long sub_165d0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  char v2 [16];
  unsigned long v3;
  unsigned long v4; // rdx
  unsigned long v5;
  
  if (a0) {
    if (a1) {
      v3 = a0;
      v5 = a1;
      while (v4 = v3 % v5, v4) {
        v3 = v5;
        v5 = v4;
      }
      v1._8_8_ = 0;
      v1._0_8_ = v5;
      v2._8_8_ = 0;
      v2._0_8_ = a0;
      v1._0_16_ = ZEXT816(a1) * ZEXT816(SUB168(v2._0_16_ / v1._0_16_,0));
      v3 = SUB168(v1._0_16_,0);
      if ((v3 <= a2) && (!SUB168(v1._0_16_,8)))
        a0 = v3;
    }
    return a0;
  }
  v3 = 0x2000;
  if (a1)
    v3 = a1;
  return v3;
}

// Function: sub_16640 @ 0x16640
unsigned long sub_16640(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_10ab0(0))) {
    v1 = 1;
    *a0 = (unsigned int)(unsigned char)*a1;
  }
  return v1;
}

// Function: sub_166d0 @ 0x166d0
void sub_166d0(void *a0,void *a1,unsigned long a2)
{
  *(char *)((long)memcpy(a0,a1,a2) + a2) = 0;
}

// Function: sub_166e0 @ 0x166e0
void sub_166e0(void)
{
  getenv("TZ"); // tail-call
}

// Function: sub_166f0 @ 0x166f0
void sub_166f0(char *a0)
{
  if (a0) {
    setenv("TZ",a0,1); // tail-call
    return;
  }
  unsetenv("TZ"); // tail-call
}

// Function: sub_16720 @ 0x16720
bool sub_16720(void *a0)
{
  char *v1;
  bool v2; // zf
  
  v1 = (char *)((long)a0 + 9);
  if (!*(char *)((long)a0 + 8))
    v1 = NULL;
  v2 = sub_166f0(v1) == 0;
  if (v2)
    tzset();
  return v2;
}

// Function: sub_16760 @ 0x16760
unsigned long * sub_16760(char *a0)
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
      sub_166d0((void *)((long)v3 + 9),a0,v1);
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

// Function: sub_16800 @ 0x16800
unsigned long sub_16800(long *a0,struct_7 *a1) // return-dupe x2
{
  struct_7 *v1;
  long *v2;
  int v3; // eax
  long v4; // rax
  char *v5;
  
  v1 = (struct_7 *)a1->field_0x30;
  if (!v1)
    return 1;
  if ((a1 <= v1) && (v1 < &a1[1]))
    return 1;
  v5 = (char *)((long)a0 + 9);
  if (!v1->field_0x0[0]) {
    v5 = "";
label_16898:
    a1->field_0x30 = v5;
    return 1;
  }
  do {
    v3 = strcmp(v5,(char *)v1);
    while( true ) {
      if (!v3) goto label_16898;
      if ((!*v5) && ((v5 != (char *)((long)a0 + 9) || (!(char)a0[1])))) {
        if ((long)a0 + (0x80U - (long)v5) <= (long)(strlen((char *)v1) + 1)) { // branch-flip
          v4 = sub_16760(v1);
          *a0 = v4;
          if (!v4)
            return 0;
          *(char *)(v4 + 8) = 0;
          v5 = (char *)(v4 + 9);
        }
        else {
          sub_166d0(v5,v1);
        }
        goto label_16898;
      }
      v5 = &v5[strlen(v5) + 1];
      if ((*v5) || (v2 = (long *)*a0, !v2)) break;
      v5 = (char *)((long)v2 + 9);
      v3 = strcmp(v5,(char *)v1);
      a0 = v2;
    }
  } while( true );
}

// Function: sub_16930 @ 0x16930
void sub_16930(long *a0) // return-dupe
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

// Function: sub_16970 @ 0x16970
long * sub_16970(void *a0)
{
  int v1;
  char *v2; // rax
  long *v3; // rax
  int *v4; // rax
  long *v5;
  
  v2 = (char *)sub_166e0();
  if (v2) { // branch-flip
    if (*(char *)((long)a0 + 8)) {
      if (!strcmp((char *)((long)a0 + 9),v2))
        return (long *)0x1;
    }
  }
  else if (!*(char *)((long)a0 + 8))
    return (long *)0x1;
  v3 = (long *)sub_16760(v2);
  v5 = v3;
  if (v3) {
    if (!sub_16720(a0)) {
      v4 = __errno_location();
      v5 = NULL;
      v1 = *v4;
      sub_16930(v3);
      *v4 = v1;
    }
  }
  return v5;
}

// Function: sub_16a20 @ 0x16a20
unsigned int sub_16a20(long *a0)
{
  unsigned int v1; // eax
  int *v2; // rax
  int v3;
  
  v1 = 1;
  if (a0 != (long *)0x1) {
    v2 = __errno_location();
    v3 = *v2;
    v1 = sub_16720(a0);
    if (!(char)v1)
      v3 = *v2;
    sub_16930(a0);
    *v2 = v3;
  }
  return v1;
}

// Function: sub_16a80 @ 0x16a80
struct_7 * sub_16a80(long *a0,void *a1,struct_7 *a2) // return-dupe
{
  long *v1; // rax
  
  if (!a0)
    return (struct_7 *)gmtime_r(a1,(tm *)a2); // tail-call
  v1 = (long *)sub_16970(a0);
  if (!v1)
    return NULL;
  if ((localtime_r(a1,(tm *)a2)) && (sub_16800(a0,a2))) {
    if (!sub_16a20(v1))
      return NULL;
    return a2;
  }
  sub_16a20(v1);
  return NULL;
}

// Function: sub_16b20 @ 0x16b20
long sub_16b20(long *a0,tm *a1) // return-dupe x2
{
  long *v1; // rax
  long v2; // rax
  tm v3; // stack - 0x68
  
  if (!a0)
    return timegm(a1); // tail-call
  v1 = (long *)sub_16970(a0);
  if (!v1)
    return -1;
  v3._0_8_ = *(unsigned long *)a1;
  v3._28_4_ = -1;
  v3._8_8_ = *(unsigned long *)&a1->field_0x8;
  v3._16_8_ = *(unsigned long *)&a1->field_0x10;
  v3._32_4_ = *(unsigned int *)&a1->field_0x20;
  v2 = mktime(&v3);
  if ((0 <= (int)v3._28_4_) && (sub_16800(a0,&v3))) {
    if (!sub_16a20(v1))
      return -1;
    *(unsigned long *)a1 = v3._0_8_;
    *(unsigned long *)&a1->field_0x8 = v3._8_8_;
    *(unsigned long *)&a1->field_0x30 = v3._48_8_;
    *(unsigned long *)&a1->field_0x10 = v3._16_8_;
    *(unsigned long *)&a1->field_0x18 = CONCAT44(v3._28_4_,v3._24_4_);
    *(unsigned long *)&a1->field_0x20 = CONCAT44(v3._36_4_,v3._32_4_);
    *(unsigned long *)&a1->field_0x28 = v3._40_8_;
    return v2;
  }
  sub_16a20(v1);
  return -1;
}

// Function: sub_16c50 @ 0x16c50
void sub_16c50(int a0,char *a1,char *a2,unsigned long a3)
{
  if (a0 == -100) {
    readlink(a1,a2,a3); // tail-call
    return;
  }
  abort(); // no-return
}

// Function: sub_16c70 @ 0x16c70
void sub_16c70(char *a0)
{
  sub_16f30(0xffffff9c,a0,0,0,0,sub_16c50); // tail-call
}

// Function: sub_16c90 @ 0x16c90
unsigned long sub_16c90(unsigned long a0,unsigned char a1)
{
  return a0 << (a1 & 0x3f) | a0 >> 0x40 - (a1 & 0x3f);
}

// Function: sub_16ca0 @ 0x16ca0
unsigned long sub_16ca0(unsigned long a0,unsigned char a1)
{
  return a0 >> (a1 & 0x3f) | a0 << 0x40 - (a1 & 0x3f);
}

// Function: sub_16cb0 @ 0x16cb0
unsigned int sub_16cb0(unsigned int a0,unsigned char a1)
{
  return a0 << (a1 & 0x1f) | a0 >> 0x20 - (a1 & 0x1f);
}

// Function: sub_16cc0 @ 0x16cc0
unsigned int sub_16cc0(unsigned int a0,unsigned char a1)
{
  return a0 >> (a1 & 0x1f) | a0 << 0x20 - (a1 & 0x1f);
}

// Function: sub_16cd0 @ 0x16cd0
unsigned long sub_16cd0(unsigned long a0,unsigned char a1)
{
  return sub_16c90(a0,a1); // tail-call
}

// Function: sub_16ce0 @ 0x16ce0
unsigned long sub_16ce0(unsigned long a0,unsigned char a1)
{
  return sub_16ca0(a0,a1); // tail-call
}

// Function: sub_16cf0 @ 0x16cf0
unsigned int sub_16cf0(unsigned int a0,unsigned char a1)
{
  return CONCAT22((short)((unsigned int)a0 >> 0x10),(unsigned short)a0 << (a1 & 0xf) | (unsigned short)a0 >> 0x10 - (a1 & 0xf));
}

// Function: sub_16d00 @ 0x16d00
unsigned int sub_16d00(unsigned int a0,unsigned char a1)
{
  return CONCAT22((short)((unsigned int)a0 >> 0x10),(unsigned short)a0 >> (a1 & 0xf) | (unsigned short)a0 << 0x10 - (a1 & 0xf));
}

// Function: sub_16d10 @ 0x16d10
unsigned int sub_16d10(unsigned int a0,unsigned char a1)
{
  return CONCAT31((undefined3)((unsigned int)a0 >> 8),(unsigned char)a0 << (a1 & 7) | (unsigned char)a0 >> 8 - (a1 & 7));
}

// Function: sub_16d20 @ 0x16d20
unsigned int sub_16d20(unsigned int a0,unsigned char a1)
{
  return CONCAT31((undefined3)((unsigned int)a0 >> 8),(unsigned char)a0 >> (a1 & 7) | (unsigned char)a0 << 8 - (a1 & 7));
}

// Function: sub_16d30 @ 0x16d30
int sub_16d30(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned int v2; // eax
  
  if (a0 == a1)
    return 0;
  do {
    v2 = sub_f8b0((unsigned char)*a0);
    v1 = sub_f8b0((unsigned char)*a1);
    if (!(unsigned char)v2) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while ((unsigned char)v2 == v1);
  return (v2 & 0xff) - (unsigned int)v1;
}

// Function: sub_16d90 @ 0x16d90
void * sub_16d90(unsigned int a0,unsigned long a1,void *a2,unsigned long a3,unsigned long *a4,void *a5,void *a6)
{
  int v1;
  long v2; // rax
  void *v3;
  int *v4; // rax
  void *v5;
  long v6;
  long v7;
  
  v5 = a6;
  if (!a4)
    a4 = (unsigned long *)0x259c0;
  if (a2) { // branch-flip
    v7 = 0x7fffffffffffffff;
    if (a3 <= 0x7fffffffffffffff)
      v7 = a3;
  }
  else {
    v7 = 0x400;
    a2 = a6;
  }
  v3 = a2;
  do {
    v2 = (*a5)(a0,a1,v3,v7);
    if (v2 < 0) {
      if (a2 == v3)
        return NULL;
      v4 = __errno_location();
      v1 = *v4;
      (*(void *)a4[2])(v3);
      *v4 = v1;
      return NULL;
    }
    if (v2 < v7) {
      *(char *)((long)v3 + v2) = 0;
      v6 = v2 + 1;
      if (v5 != v3) {
        if (v7 <= v6)
          return v3;
        if (a2 == v3)
          return v3;
        if (!(void *)a4[1])
          return v3;
        v5 = (void *)(*(void *)a4[1])(v3,v6);
        if (v5)
          return v5;
        return v3;
      }
      v3 = (void *)(*(void *)*a4)(v6);
      if (v3)
        return memcpy(v3,v5,v6); // tail-call
      break;
    }
    if (a2 != v3)
      (*(void *)a4[2])(v3);
    if (0x3ffffffffffffffe < v7) {
      *__errno_location() = 0x24;
      return NULL;
    }
    v6 = v7 * 2 + 1;
    v3 = (void *)(*(void *)*a4)(v6);
    v7 = v6;
  } while (v3);
  if ((void *)a4[3])
    (*(void *)a4[3])(v6);
  *__errno_location() = 0xc;
  return NULL;
}

// Function: sub_16f30 @ 0x16f30
void sub_16f30(void)
{
  sub_16d90();
}

// Function: sub_16f80 @ 0x16f80
void sub_16f80(unsigned long a0)
{
  malloc(a0); // tail-call
}

// Function: sub_16f90 @ 0x16f90
void sub_16f90(void *a0,unsigned long a1)
{
  realloc(a0,a1 | a1 == 0); // tail-call
}

// Function: sub_16fb0 @ 0x16fb0
void sub_16fb0(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1); // tail-call
}

// Function: sub_16fc0 @ 0x16fc0
void sub_16fc0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((a1) && (a2)) {
    reallocarray(a0,a1,a2); // tail-call
    return;
  }
  reallocarray(a0,1,1); // tail-call
}

// Function: sub_16ff0 @ 0x16ff0
char * sub_16ff0(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_17030 @ 0x17030
int sub_17030(unsigned int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (0 <= v1)
    return v1;
  return (unsigned long)(iswcntrl(a0) == 0);
}

// Function: sub_17060 @ 0x17060
void sub_17060(struct_40 *a0,struct_41 *a1) // return-dupe
{
  char v1;
  long v2;
  struct_41 *v3;
  
  v3 = (struct_41 *)a1->field_0x0;
  if (v3 == &a1[1])
    v3 = memcpy(&a0[1],&a1[1],a1->field_0x8);
  v2 = a1->field_0x8;
  a0->field_0x0 = v3;
  a0->field_0x8 = v2;
  v1 = a1->field_0x10;
  a0->field_0x10 = v1;
  if (!v1)
    return;
  a0->field_0x14 = a1->field_0x14;
}

// Function: sub_170c0 @ 0x170c0
unsigned int sub_170c0(unsigned char a0)
{
  return *(unsigned int *)((unsigned long)(a0 >> 5) * 4 + 0x1e100) >> (a0 & 0x1f) & 1;
}

// Function: sub_170e0 @ 0x170e0
void sub_170e0(struct_43 *a0)
{
  char v1;
  char *v2;
  long v3; // rax
  
  if (a0->field_0x14)
    return;
  v2 = a0->field_0x18;
  if (!a0->field_0x8) {
    v1 = *v2;
    if (sub_170c0((int)v1)) {
      a0->field_0x20 = 1;
      v1 = *v2;
      a0->field_0x28 = 1;
      a0->field_0x2c = (int)v1;
      goto label_17123;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x9[3]))
      __assert_fail("mbsinit (&iter->state)","mbiter.h",0x87,"mbiter_multi_next"); // no-return
    a0->field_0x8 = 1;
  }
  v3 = sub_16640(&a0->field_0x2c,v2,a0->field_0x0 - (long)v2,(mbstate_t *)&a0->field_0x9[3]);
  a0->field_0x20 = v3;
  if (v3 == -1) {
    a0->field_0x20 = 1;
    a0->field_0x28 = 0;
    a0->field_0x14 = 1;
    return;
  }
  if (v3 != -2) { // branch-flip
    if (!v3) {
      a0->field_0x20 = 1;
      if (*a0->field_0x18)
        __assert_fail("*iter->cur.ptr == \'\\0\'","mbiter.h",0xa2,"mbiter_multi_next"); // no-return
      if (a0->field_0x2c)
        __assert_fail("iter->cur.wc == 0","mbiter.h",0xa3,"mbiter_multi_next"); // no-return
    }
    a0->field_0x28 = 1;
    if (mbsinit((mbstate_t *)&a0->field_0x9[3]))
      a0->field_0x8 = 0;
  }
  else {
    a0->field_0x28 = 0;
    a0->field_0x20 = a0->field_0x0 - (long)a0->field_0x18;
  }
label_17123:
  a0->field_0x14 = 1;
  return;
}

// Function: sub_17250 @ 0x17250
void sub_17250(struct_0 *a0,long a1)
{
  a0->field_0x18 = a0->field_0x18 + a1;
  a0->field_0x0 = a0->field_0x0 + a1;
}

// Function: sub_17260 @ 0x17260
void sub_17260(struct_45 *a0,struct_45 *a1)
{
  char v1;
  
  a0->field_0x0 = a1->field_0x0;
  v1 = a1->field_0x8;
  a0->field_0x8 = v1;
  if (v1) // branch-flip
    *(unsigned long *)&a0->field_0x9[3] = *(unsigned long *)&a1->field_0x9[3];
  else {
    *(unsigned long *)&a0->field_0x9[3] = 0;
  }
  a0->field_0x14 = a1->field_0x14;
  sub_17060((struct_40 *)&a0[1],(struct_41 *)&a1[1]); // tail-call
}

// Function: sub_172b0 @ 0x172b0
unsigned long sub_172b0(char *a0,long a1,unsigned long a2,unsigned long *a3)
{
  unsigned long v1; // rax
  char *v10;
  char v2;
  long v3;
  char v4 [8];
  char *v5;
  char *v6; // rsp
  char *v7; // rsp
  char *v8;
  unsigned long v9;
  
  v5 = v4;
  v7 = v4;
  v8 = v4;
  v9 = a2 * 8;
  if ((0 <= (long)v9) && (!(a2 >> 0x3d))) {
    if (0xfa1 <= v9) // branch-flip
      v9 = sub_11c90(v9);
    else {
      v8 = v4;
      while (v7 != &v4[-(v9 + 0x36 & 0xfffffffffffff000)]) {
        v5 = &v8[-0x1000];
        v6 = &v8[-0x1000];
        *(unsigned long *)&v8[-8] = *(unsigned long *)&v8[-8];
        v7 = &v8[-0x1000];
        v8 = v6;
      }
      v9 = (unsigned long)((unsigned int)(v9 + 0x36) & 0xff0);
      v3 = -v9;
      v8 = &v5[v3];
      if (v9)
        *(unsigned long *)&v5[-8] = *(unsigned long *)&v5[-8];
      v9 = ((unsigned long)&v5[v3 + 0xf] & 0xfffffffffffffff0) + 0x1f & 0xffffffffffffffe0;
    }
    if (v9) {
      *(unsigned long *)(v9 + 8) = 1;
      if (3 <= a2) {
        v3 = 0;
        v1 = 2;
        do {
          v2 = *(char *)(a1 + v3);
          while (*(char *)(a1 + -1 + v1) != v2) {
            if (!v3) {
              *(unsigned long *)(v9 + v1 * 8) = v1;
              v3 = 0;
              goto label_173a8;
            }
            v3 -= *(long *)(v9 + v3 * 8);
            v2 = *(char *)(a1 + v3);
          }
          v3 += 1;
          *(unsigned long *)(v9 + v1 * 8) = v1 - v3;
label_173a8:
          v1 += 1;
        } while (a2 != v1);
      }
      *a3 = 0;
      v2 = *a0;
      if (v2) {
        v1 = 0;
        v10 = a0;
        do {
          if (*(char *)(a1 + v1) != v2) { // branch-flip
            if (v1) { // branch-flip
              v10 = &v10[*(long *)(v9 + v1 * 8)];
              v1 -= *(long *)(v9 + v1 * 8);
            }
            else {
              v10 = &v10[1];
              a0 = &a0[1];
            }
          }
          else {
            v1 += 1;
            a0 = &a0[1];
            if (a2 == v1) {
              *a3 = v10;
              break;
            }
          }
          v2 = *a0;
        } while (v2);
      }
      *(unsigned long *)&v8[-8] = 0x173fe;
      sub_11ce0(v9);
      return 1;
    }
  }
  return 0;
}

// Function: sub_17450 @ 0x17450
char sub_17450(void *a0,void *a1,long *a2)
{
  char v1;
  char v10 [4]; // stack - 0x88
  char v11 [4]; // stack - 0xc8
  void **v12;
  char *v13; // rsp
  void **v14; // rsp
  void **v15;
  unsigned long v16;
  long v17;
  void *v18;
  unsigned long *v19;
  void *v2;
  long *v20; // stack - 0xf0
  char v21; // stack - 0xe1
  void *v22; // stack - 0xe0
  void *v23; // stack - 0xd8
  char *v24; // stack - 0xd0
  unsigned long v25; // stack - 0xc4
  char v26; // stack - 0xbc
  void *v27; // stack - 0xb8
  long v28; // stack - 0xb0
  char v29; // stack - 0xa8
  int v3; // eax
  int v30; // stack - 0xa4
  unsigned long v31; // stack - 0x84
  char v32; // stack - 0x7c
  unsigned long v33; // stack - 0x70
  char v34; // stack - 0x68
  int v35; // stack - 0x64
  unsigned long *v4; // rax
  unsigned long *v5;
  void *v6;
  unsigned long *v7; // rbx
  void *v8; // stack - 0xf8
  void *v9; // stack - 0x78
  
  v14 = &v8;
  v12 = &v8;
  v15 = &v8;
  v8 = a0;
  v20 = a2;
  v22 = (void *)sub_1ae40(a1);
  if ((0 <= SUB168(ZEXT816(0x38) * ZEXT816(v22),0)) && (!SUB168(ZEXT816(0x38) * ZEXT816(v22),8))) {
    v16 = (long)v22 * 0x38;
    if (0xfa1 <= v16) { // branch-flip
      v19 = (unsigned long *)sub_11c90(v16);
      v2 = v22;
    }
    else {
      v15 = &v8;
      while (v14 != (void **)((long)&v8 - (v16 + 0x36 & 0xfffffffffffff000))) {
        v12 = (char *)((long)v15 + -0x1000);
        v13 = (char *)((long)v15 + -0x1000);
        *(unsigned long *)((long)v15 + -8) = *(unsigned long *)((long)v15 + -8);
        v14 = (char *)((long)v15 + -0x1000);
        v15 = (void **)v13;
      }
      v16 = (unsigned long)((unsigned int)(v16 + 0x36) & 0xff0);
      v17 = -v16;
      v15 = (char *)((long)v12 + v17);
      if (v16)
        *(unsigned long *)((long)v12 + -8) = *(unsigned long *)((long)v12 + -8);
      v19 = (unsigned long *)(((unsigned long)((long)v12 + v17 + 0xf) & 0xfffffffffffffff0) + 0x1f & 0xffffffffffffffe0);
      v2 = v22;
    }
    if (v19) {
      v10[0] = 0;
      v24 = v10;
      v31 = 0;
      v5 = v19;
      v22 = v2;
      v9 = a1;
      while( true ) {
        v32 = 0;
        *(unsigned long *)((long)v15 + -8) = 0x175cf;
        sub_11f60(v24);
        v21 = v34;
        if ((v34) && (!v35)) break;
        v7 = &v5[6];
        *(unsigned long *)((long)v15 + -8) = 0x175b7;
        sub_17060(v5,&v9);
        v9 = (void *)((long)v9 + v33);
        v5 = v7;
      }
      v17 = 0;
      v19[(long)v2 * 6 + 1] = 1;
      v23 = (void *)0x2;
      v5 = v19;
      if ((void *)0x3 <= v22) {
label_17610:
        v7 = &v5[6];
        v1 = *(char *)&v5[8];
        do {
          v4 = &v19[v17 * 6];
          if ((v1) && (*(char *)&v4[2])) {
            if (*(int *)((long)v5 + 0x44) == *(int *)((long)v4 + 0x14)) {
label_17665:
              v17 += 1;
              v19[(long)((long)v2 * 6 + (long)v23)] = (long)v23 - v17;
              goto label_1767a;
            }
          }
          else {
            v16 = v5[7];
            if (v16 == v4[1]) {
              v18 = (void *)*v4;
              v6 = (void *)*v7;
              *(unsigned long *)((long)v15 + -8) = 0x17661;
              if (!memcmp(v6,v18,v16)) goto label_17665;
            }
          }
          if (!v17) goto label_17840;
          v17 -= v19[(long)v2 * 6 + v17];
        } while( true );
      }
label_1769a:
      v10[0] = 0;
      v18 = NULL;
      v11[0] = 0;
      *v20 = 0;
      v25 = 0;
      v27 = v8;
      v26 = 0;
      v9 = v8;
      v31 = 0;
      v32 = 0;
      do {
        while( true ) {
          *(unsigned long *)((long)v15 + -8) = 0x176fc;
          sub_11f60(v24);
          v16 = v33;
          if (!v34) break;
          if (!v35) goto label_17537;
          v5 = &v19[(long)v18 * 6];
          if (!*(char *)&v5[2]) goto label_1776b;
          if (*(int *)((long)v5 + 0x14) == v35) {
            v6 = v9;
            goto label_17727;
          }
label_177eb:
          if (v18) { // branch-flip
            v17 = v19[(long)((long)v2 * 6 + (long)v18)];
            v18 = (void *)((long)v18 - v17);
            for (; v17; v17 = v17 + -1) {
              *(unsigned long *)((long)v15 + -8) = 0x17827;
              sub_11f60(v11);
              if ((v29) && (!v30)) goto label_1783a;
              v26 = 0;
              v27 = (void *)((long)v27 + v28);
            }
          }
          else {
            *(unsigned long *)((long)v15 + -8) = 0x17782;
            sub_11f60(v11);
            if ((v29) && (!v30)) {
label_1783a:
              *(unsigned long *)((long)v15 + -8) = 0x1783f; // no-return
              abort();
            }
            v26 = 0;
            v27 = (void *)((long)v27 + v28);
            v32 = 0;
            v9 = (void *)((long)v9 + v33);
          }
        }
        v5 = &v19[(long)v18 * 6];
label_1776b:
        if (v5[1] != v33) goto label_177eb;
        v6 = (void *)*v5;
        v23 = v9;
        *(unsigned long *)((long)v15 + -8) = 0x177d9;
        v3 = memcmp(v6,v9,v33);
        v6 = v23;
        if (v3) goto label_177eb;
label_17727:
        v9 = (void *)((long)v6 + v16);
        v32 = 0;
        v18 = (void *)((long)v18 + 1);
      } while (v22 != v18);
      *v20 = (long)v27;
label_17537:
      *(unsigned long *)((long)v15 + -8) = 0x17758;
      sub_11ce0(v19);
      return v21;
    }
  }
  return '\0';
label_17840:
  v19[(long)((long)v2 * 6 + (long)v23)] = v23;
label_1767a:
  v23 = (void *)((long)v23 + 1);
  v5 = v7;
  if (v22 == v23) goto label_1769a;
  goto label_17610;
}

// Function: sub_17860 @ 0x17860
char * sub_17860(char *a0,char *a1) // early-return, return-dupe x4
{
  char *v1;
  char *v10; // stack - 0x190
  char v11 [4]; // stack - 0x188
  char v12 [4]; // stack - 0x88
  unsigned long v13; // stack - 0xc8
  unsigned int v14; // stack - 0x108
  char v15 [4]; // stack - 0x148
  long v16;
  unsigned long v17;
  unsigned long v18; // stack - 0x1d8
  unsigned char v19; // stack - 0x1c9
  char v2;
  unsigned long v20; // stack - 0x1a8
  unsigned long v21; // stack - 0x184
  char v22; // stack - 0x17c
  char *v23; // stack - 0x178
  unsigned long v24; // stack - 0x170
  char v25; // stack - 0x168
  int v26; // stack - 0x164
  unsigned long v27; // stack - 0x144
  char v28; // stack - 0x13c
  char *v29; // stack - 0x138
  char v3;
  long v30; // stack - 0x130
  unsigned char v31; // stack - 0x128
  int v32; // stack - 0x124
  unsigned int v33; // stack - 0x104
  unsigned int v34; // stack - 0x100
  unsigned int v35; // stack - 0xfc
  char *v36; // stack - 0xf8
  unsigned long v37; // stack - 0xf0
  unsigned int v38; // stack - 0xe8
  int v39; // stack - 0xe4
  unsigned char v4; // al
  unsigned long v40; // stack - 0xe0
  unsigned long v41; // stack - 0xd8
  unsigned long v42; // stack - 0xd0
  unsigned long v43; // stack - 0xc0
  undefined3 v44; // stack - 0xbb
  char *v45; // stack - 0xb8
  unsigned long v46; // stack - 0xb0
  unsigned long v47; // stack - 0xa8
  unsigned long v48; // stack - 0xa0
  unsigned long v49; // stack - 0x98
  unsigned int v5; // eax
  unsigned long v50; // stack - 0x90
  unsigned long v51; // stack - 0x84
  char v52; // stack - 0x7c
  char *v53; // stack - 0x78
  unsigned long v54; // stack - 0x70
  char v55; // stack - 0x68
  int v56; // stack - 0x64
  char *v6;
  unsigned long v7;
  unsigned long v8;
  unsigned long v9; // rdx
  
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
    v11[0] = 0;
    v21 = 0;
    v22 = 0;
    v23 = a1;
    sub_11f60(v11);
    if ((!v25) || (v26)) {
      v19 = 1;
      v15[0] = 0;
      v27 = 0;
      v28 = 0;
      v14 &= 0xffffff00;
      v34 = 0;
      v33 = 0;
      v18 = 0;
      v20 = 0;
      v8 = 0;
      v29 = a1;
      v36 = a0;
      do {
        v35 &= 0xffffff00;
        sub_11f60(&v14);
        if ((char)v38) { // branch-flip
          if (!v39)
            return NULL;
          v4 = 10 <= v18 & v19;
          if (v4) {
            if (v18 * 5 <= v8) { // branch-flip
label_17de0:
              v16 = v8 - v20;
              while ((v16 && ((sub_11f60(v15), !v31 || (v32))))) {
                v28 = 0;
                v29 = &v29[v30];
                v16 -= 1;
              }
              sub_11f60(v15);
              v19 = v31;
              if (v31) { // branch-flip
                if (!v32) {
                  v19 = sub_17450(a0,a1,&v10);
                  if (v19)
                    return v10;
                }
              }
              else {
                v19 = 1;
              }
              v20 = v8;
              if (!(char)v38) goto label_17bb6;
            }
            else {
              v19 = v4;
            }
          }
          v9 = v8 + 1;
          if (!v25) goto label_17bb6;
          if (v39 == v26) goto label_17bf6;
label_17b48:
          v6 = v36;
          v17 = v37;
        }
        else {
          v4 = 10 <= v18 & v19;
          if (v4) { // branch-flip
            if (v18 * 5 <= v8) goto label_17de0;
            v19 = v4;
          }
          else {
          }
label_17bb6:
          v17 = v37;
          v6 = v36;
          v9 = v8 + 1;
          if (v37 == v24) {
            if (!memcmp(v36,v23,v37)) {
label_17bf6:
              v9 = v8 + 1;
              v13 = CONCAT44(v33,v14);
              v12[0] = 0;
              v47 = CONCAT44(v39,v38);
              v52 = 0;
              v46 = v37;
              v45 = &v36[v37];
              v44 = (undefined3)(v35 >> 8);
              v43 = (undefined5)v34;
              v51 = 0;
              v48 = v40;
              v49 = v41;
              v50 = v42;
              v53 = a1;
              sub_11f60(v12);
              if ((v55) && (!v56))
                abort(); // no-return
              v53 = &v53[v54];
              do {
                v9 += 1;
                v52 = 0;
                sub_11f60(v12);
                if ((v55) && (!v56))
                  return v36;
                sub_11f60(&v13);
                v17 = v54;
                v1 = v53;
                v8 = v46;
                v6 = v45;
                if ((char)v47) { // branch-flip
                  if (!v47._4_4_)
                    return NULL;
                  if (!v55) goto label_17d63;
                  if (v47._4_4_ != v56) goto label_17b48;
                }
                else {
label_17d63:
                  if ((v46 != v54) || (memcmp(v45,v53,v46))) goto label_17b48;
                }
                v45 = &v6[v8];
                v43 = (undefined5)(unsigned int)v43;
                v53 = &v1[v17];
              } while( true );
            }
          }
        }
        v18 += 1;
        v36 = &v6[v17];
        v8 = v9;
      } while( true );
    }
  }
  else {
    v2 = *a1;
    if (v2) {
      v3 = *a0;
      if (!v3)
        return NULL;
      v17 = 0;
      v9 = 0;
      v8 = 0;
      v5 = 1;
      v6 = a1;
      do {
        v17 += 1;
        v7 = v9 + 1;
        v1 = &a0[1];
        if (v2 == v3) {
          v3 = a1[1];
          if (!v3)
            return a0;
          while( true ) {
            v16 = v7 - v9;
            if (!a0[v16])
              return NULL;
            v7 += 1;
            if (a0[v16] != v3) break;
            v3 = a1[v7 - v9];
            if (!v3)
              return a0;
          }
        }
        v9 = v7;
        if (!*v1)
          return NULL;
        v4 = 10 <= v17 & (unsigned char)v5;
        if (v4) {
          if (v17 * 5 <= v9) { // branch-flip
            if (v6) {
              v6 = &v6[strnlen(v6,v9 - v8)];
              v8 = v9;
              if (*v6) goto label_1798d;
            }
            v5 = sub_172b0(v1,a1,strlen(a1),&v10);
            if ((char)v5)
              return v10;
            v6 = NULL;
          }
          else {
label_1798d:
            v5 = (unsigned int)v4;
          }
        }
        v3 = *v1;
        a0 = v1;
      } while( true );
    }
  }
  return a0;
}

// Function: sub_17f00 @ 0x17f00
struct_46 * sub_17f00(struct_46 *a0,int a1)
{
  if (a1 != 10) {
    a0->field_0x0 = a1;
    a0->field_0x4 = 0;
    a0->field_0x8 = 0;
    a0->field_0x10 = 0;
    a0->field_0x18 = 0;
    a0->field_0x20 = 0;
    a0->field_0x28 = 0;
    a0->field_0x30 = 0;
    return a0;
  }
  abort(); // no-return
}

// Function: sub_17f50 @ 0x17f50
bool sub_17f50(long a0,long a1)
{
  return sub_16d30((char *)(a0 + 9),(char *)(a1 + 9)) == 0;
}

// Function: sub_17f70 @ 0x17f70
unsigned long sub_17f70(long a0,long a1,unsigned char a2)
{
  unsigned char v1;
  
  if (sub_f870((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 8) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 8);
  }
  if (v1 == a2) {
    if (a2)
      return sub_17f50(a0,a1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_17fd0 @ 0x17fd0
unsigned long sub_17fd0(long a0,long a1,unsigned char a2,char a3)
{
  unsigned char v1;
  
  if (sub_f870((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 7) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 7);
  }
  if (v1 == a2) {
    if (a2)
      return sub_17f70(a0,a1,(int)a3); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_18050 @ 0x18050
unsigned long sub_18050(long a0,long a1,unsigned char a2,char a3,char a4)
{
  unsigned char v1;
  
  if (sub_f870((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 6) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 6);
  }
  if (v1 == a2) {
    if (a2)
      return sub_17fd0(a0,a1,(int)a3,(int)a4); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_180c0 @ 0x180c0
unsigned long sub_180c0(long a0,long a1,unsigned char a2,char a3,char a4,char a5)
{
  unsigned char v1;
  
  if (sub_f870((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 5) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 5);
  }
  if (v1 == a2) {
    if (a2)
      return sub_18050(a0,a1,(int)a3,(int)a4,(int)a5); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_18160 @ 0x18160
unsigned long sub_18160(long a0,long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6)
{
  unsigned int v1;
  unsigned char v2;
  
  v1 = a6;
  if (sub_f870((int)(char)a2)) // branch-flip
    v2 = *(unsigned char *)(a0 + 4) & 0xdf;
  else {
    v2 = *(unsigned char *)(a0 + 4);
  }
  if (v2 == a2) {
    if (a2)
      return sub_180c0(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_18200 @ 0x18200
unsigned long sub_18200(long a0,long a1,unsigned char a2,char a3,char a4,char a5,int a6,unsigned int a7)
{
  unsigned int v1;
  unsigned int v2;
  unsigned char v3;
  
  v2 = a7;
  v1 = a6;
  if (sub_f870((int)(char)a2)) // branch-flip
    v3 = *(unsigned char *)(a0 + 3) & 0xdf;
  else {
    v3 = *(unsigned char *)(a0 + 3);
  }
  if (v3 == a2) {
    if (a2) {
      a6 = (int)(char)v2;
      return sub_18160(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_182c0 @ 0x182c0
unsigned long sub_182c0(long a0,long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,unsigned int a8)
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  unsigned char v4;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_f870((int)(char)a2)) // branch-flip
    v4 = *(unsigned char *)(a0 + 2) & 0xdf;
  else {
    v4 = *(unsigned char *)(a0 + 2);
  }
  if (v4 == a2) {
    if (a2) {
      a7 = (int)(char)v3;
      a6 = (int)(char)v2;
      return sub_18200(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_18380 @ 0x18380
unsigned long sub_18380(long a0,long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,int a8,unsigned int a9)
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
  if (sub_f870((int)(char)a2)) // branch-flip
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
      return sub_182c0(a0,a1,(int)a3,(int)a4,(int)a5,(int)v6); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_18460 @ 0x18460
unsigned long sub_18460(unsigned char *a0,long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6,unsigned int a7,unsigned int a8) // early-return
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_f870((int)(char)a2)) { // branch-flip
    if (a2 != (*a0 & 0xdf))
      return 0;
  }
  else if (a2 != *a0)
    return 0;
  return sub_18380(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1,(int)(char)v2,(int)(char)v3,0,0);
}

// Function: sub_18520 @ 0x18520
char * sub_18520(char *a0,int a1) // return-dupe, ternary x2
{
  char *v1; // rax
  unsigned char *v2; // rax
  
  v1 = dcgettext(NULL,a0,5);
  if (a0 == v1) {
    v2 = (unsigned char *)sub_16ff0();
    if (sub_18460(v2,"UTF-8",0x55,0x54,0x46,0x2d,0x38,0,0)) { // branch-flip
      v1 = (*v1 != '`') ? "’" : "‘"; // branch-flip
    }
    else {
      if (!sub_18460(v2,"GB18030",0x47,0x42,0x31,0x38,0x30,0x33,0x30)) {
        if (a1 == 9)
          return "\"";
        return "\'";
      }
      v1 = (*v1 != '`') ? (char *)0x1e146 : (char *)0x1e14b; // branch-flip
    }
  }
  return v1;
}

// Function: sub_18650 @ 0x18650
long * sub_18650(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
{
  unsigned long v1;
  char v10;
  unsigned char *v11;
  unsigned long v12;
  unsigned long v13;
  mbstate_t v14; // stack - 0x48
  unsigned int v15; // stack - 0x4c
  unsigned char v16;
  unsigned int v17;
  unsigned long v18;
  bool v19;
  bool v2;
  unsigned long v20;
  bool v21;
  unsigned long v22;
  bool v23;
  bool v24;
  bool v25;
  bool v26;
  bool v27;
  long v28; // stack - 0xe0
  unsigned long v29; // stack - 0xd8
  bool v3;
  char *v30; // stack - 0xb0
  unsigned long v31; // stack - 0xa8
  unsigned long v32; // stack - 0xa0
  unsigned int v33; // stack - 0x80
  char *v34; // stack - 0x70
  char *v35; // stack - 0x68
  bool v4;
  long v5;
  long v6;
  unsigned char v7;
  unsigned int v8; // eax
  long *v9; // rax
  
  v28 = a6;
  v35 = a7;
  v34 = a8;
  v33 = a5;
label_186c0:
  v9 = (unsigned long)__ctype_get_mb_cur_max();
  v32 = (unsigned long)v9;
  v17 = v33 & 2;
  switch(a4) {
    case 0:
      v23 = 0;
      v2 = 0;
      v12 = 0;
      v29 = 0;
      v30 = NULL;
      break;
    case 1:
label_18b46:
      v2 = 0;
      goto label_18b00;
    case 2:
      if (!v17) {
        v2 = 0;
        goto label_18c61;
      }
      v23 = 1;
      v2 = 0;
      v12 = 0;
      v29 = 1;
      v30 = "\'";
      break;
    case 3:
      v2 = 1;
label_18b00:
      v23 = 1;
      v12 = 0;
      a4 = 2;
      v29 = 1;
      v30 = "\'";
      break;
    case 4:
      if (!v17) {
        v2 = 1;
        goto label_18c61;
      }
      goto label_18b46;
    case 5:
      if (!v17) goto label_18aa1;
      v23 = 1;
      v2 = 1;
      v12 = 0;
      v29 = 1;
      v30 = "\"";
      break;
    case 6:
      v23 = 1;
      v12 = 0;
      v2 = 1;
      v29 = 1;
      a4 = 5;
      v30 = "\"";
      break;
    case 7:
      v23 = 0;
      v2 = 1;
      v12 = 0;
      v29 = 0;
      v30 = NULL;
      break;
    case 8:
    case 9:
    case 10:
      if (a4 != 10) {
        v9 = (char *)sub_18520("`",a4);
        v35 = (char *)v9;
        v9 = (char *)sub_18520("\'",a4);
        v34 = (char *)v9;
      }
      v12 = 0;
      if (!v17) {
        v10 = *v35;
        while (v10) {
          if (v12 < a1)
            a0[v12] = v10;
          v12 += 1;
          v10 = v35[v12];
        }
      }
      v23 = v17 != 0;
      v2 = 1;
      v9 = (unsigned long)strlen(v34);
      v30 = v34;
      v29 = (unsigned long)v9;
      break;
    default:
      abort(); // no-return
    
  }
  v4 = 1;
  v19 = 0;
  v31 = 0;
  v13 = a1;
  v3 = 0;
label_18783:
  do {
label_18796:
    v22 = 0;
    a1 = v13;
label_187a0:
    v13 = v31;
    v24 = a3 != v22;
    if (a3 == 0xffffffffffffffff)
      v24 = a2[v22] != '\0';
    if (v24) {
      v11 = (unsigned char *)&a2[v22];
      v24 = (bool)(a4 != 2 & v2);
      v25 = 1;
      v13 = v12;
      v26 = v2;
      v27 = v2;
      v21 = v24;
      if (v24) {
        if (v29) {
          if ((a3 == 0xffffffffffffffff) && (2 <= v29)) {
            v9 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v9;
          }
          if ((a3 < v22 + v29) || (v8 = memcmp(v11,v30,v29), v8)) goto label_19110;
          if (!v23) {
            v7 = *v11;
            v18 = (unsigned long)v7;
            v20 = v18;
            if ((char)v7 <= '?') {
              switch(v7) {
                case 0:
                  goto label_18d0c;
                default:
                  goto label_189e3;
                case 7:
                  goto label_18cd0;
                case 8:
                  goto label_18cb5;
                case 9:
                  goto label_18d93;
                case 10:
                  goto label_18cab;
                case 0xb:
                  goto label_18dc0;
                case 0xc:
                  goto label_18cf7;
                case 0xd:
                  goto label_18bfd;
                case 0x20:
                  goto label_18dcd;
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
                  goto label_18996;
                case 0x23:
                  goto label_18d70;
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
                  goto label_188c8;
                case 0x27:
                  goto label_18be2;
                case 0x3f:
                  goto label_18ba1;
                
              }
            }
            if ('{' > (char)v7) {
              if (v7 == 0x40) goto label_189e3;
              v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
              if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_188c8;
              if (!((unsigned long)v9 & 0xa4000000)) goto label_191b4;
              goto label_190f8;
            }
            if (v7 == 0x7d) goto label_19280;
            if ((char)v7 <= '}') {
              v20 = 0x7b;
              if (v7 == 0x7b) goto label_190da;
              v20 = 0x7c;
              if (v7 == 0x7c) goto label_190f8;
              goto label_189e3;
            }
            if (v7 != 0x7e) goto label_189e3;
label_19234:
            if (!v22) {
              v7 = a4 == 2;
              v20 = 0x7e;
              v25 = v3;
              goto label_18a59;
            }
            v20 = 0x7e;
            goto label_19242;
          }
          goto label_189cc;
        }
label_19110:
        v7 = *v11;
        v18 = (unsigned long)v7;
        v20 = v18;
        if ('?' < (char)v7) {
          if ('{' <= (char)v7) { // branch-flip
            if (v7 != 0x7d) { // branch-flip
              if ('~' <= (char)v7) {
                v21 = 0;
                if (v7 == 0x7e) goto label_19234;
                goto label_189e0;
              }
              v20 = 0x7b;
              v24 = 0;
              if (v7 != 0x7b) {
                v20 = 0x7c;
                v24 = 0;
                v21 = 0;
                if (v7 == 0x7c) goto label_190f8;
                goto label_189e3;
              }
            }
            else {
              v20 = 0x7d;
              v24 = 0;
            }
label_190da:
            if (a3 == 0xffffffffffffffff) goto label_1928f;
label_190e8:
            v21 = v24;
            if (a3 == 1) goto label_18d75;
          }
          else {
            if (v7 == 0x40) goto label_189e0;
            v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
            v21 = 0;
            if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_188c8;
            v24 = 0;
            v21 = 0;
            if (!((unsigned long)v9 & 0xa4000000)) {
label_191b4:
              if ((char)v18 != '\\') goto label_189e3;
              if (((bool)(v2 & v23)) && (v29)) goto label_191da;
              v16 = 0x5c;
              goto label_18c02;
            }
          }
label_190f8:
          v7 = a4 == 2;
          v25 = v3;
          v24 = 0;
          goto label_18a59;
        }
        switch(v7) {
          case 0:
            goto label_18cfe;
          default:
label_189e0:
            v24 = 0;
label_189e3:
            v16 = (unsigned char)v18;
            v21 = v24;
            if (v32 != 1) goto label_18f63;
label_189f0:
            v9 = __ctype_b_loc();
            v20 = 1;
            v25 = (*(unsigned char *)(*v9 + 1 + v18 * 2) & 0x40) == 0;
            v26 = (bool)(v25 & v2);
            v25 = !v25;
            v21 = v24;
            goto label_18a3e;
          case 7:
label_18cd0:
            v16 = 0x61;
            if (!v23) goto label_18cc8;
            goto label_18c2f;
          case 8:
label_18cb5:
            v16 = 0x62;
            goto label_18cba;
          case 9:
            v24 = 0;
label_18d93:
            v20 = 9;
            v16 = 0x74;
            goto label_18da0;
          case 10:
label_18cab:
            v16 = 0x6e;
            break;
          case 0xb:
label_18dc0:
            v16 = 0x76;
            break;
          case 0xc:
label_18cf7:
            v16 = 0x66;
label_18cba:
            if (v23) goto label_18c2f;
label_18cc8:
            v24 = 0;
            goto label_1894e;
          case 0xd:
label_18bfd:
            v16 = 0x72;
            break;
          case 0x20:
            v21 = 0;
label_18dcd:
            v20 = 0x20;
            goto label_188c8;
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
            v24 = 0;
            v25 = 0;
            goto label_18999;
          case 0x23:
            v20 = 0x23;
            v24 = 0;
            goto label_18d75;
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
            v21 = 0;
label_188c8:
            v7 = 0;
            goto label_188d8;
          case 0x27:
            v21 = 0;
            goto label_18be2;
          case 0x3f:
            v21 = 0;
            goto label_18ba1;
          
        }
        goto label_18c02;
      }
      v16 = *v11;
      v18 = (unsigned long)v16;
      if ('?' < (char)v16) {
        if ('{' <= (char)v16) {
          if (v16 == 0x7d) {
label_19280:
            v20 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_190e8;
label_1928f:
            v21 = v24;
            if (a2[1]) goto label_190f8;
            goto label_18d75;
          }
          if ((char)v16 <= '}') {
            v20 = 0x7b;
            if (v16 != 0x7b) {
              if (v16 == 0x7c) goto label_18996;
              goto label_189e3;
            }
            goto label_190da;
          }
          v20 = 0x7e;
          if (v16 == 0x7e) goto label_18d75;
          if (v32 == 1) goto label_189f0;
label_18f63:
          v14 = 0;
          if (a3 == 0xffffffffffffffff) {
            v9 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v9;
          }
          v20 = 0;
          do {
            v18 = v22 + v20;
            v9 = (long)sub_16640(&v15,&a2[v18],a3 - v18,&v14);
            v6 = (long)v9;
            v24 = v25;
            if (!v9) break;
            if (v9 == (long *)0xffffffffffffffff) {
              v24 = 0;
              goto label_194cb;
            }
            if (v9 == (long *)0xfffffffffffffffe) {
              v9 = (long *)v18;
              if (a3 > v18) goto label_19845;
              goto label_1984f;
            }
            if (((a4 == 2) && (v23)) && (v9 != (long *)0x1)) {
              v11 = (unsigned char *)&a2[v18 + 1];
              v5 = v18 + (long)v9;
              do {
                v8 = (unsigned int)*v11;
                v8 -= 0x5b;
                v17 = v8;
                v9 = (unsigned long)(unsigned long)v8;
                v18 = (unsigned long)v9;
                v7 = (unsigned char)v17;
                if ((v7 <= 0x21) && (0x20000002bU >> (v18 & 0x3f) & 1)) goto label_1909d;
                v11 = &v11[1];
              } while ((unsigned char *)&a2[v5] != v11);
            }
            v8 = iswprint(v15);
            if (!v8)
              v25 = 0;
            v20 += v6;
            v8 = mbsinit(&v14);
            v24 = v25;
          } while (!v8);
          v26 = (bool)((v24 ^ 1U) & v2);
          goto label_194cb;
        }
        if (v16 == 0x40) goto label_189e3;
        v9 = (unsigned long)(1L << (v16 + 0xbf & 0x3f));
        if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_18a50;
        if ((unsigned long)v9 & 0xa4000000) goto label_18996;
        if (v16 != 0x5c) goto label_189e3;
        if (a4 == 2) {
          if (!v23) goto label_191da;
          goto label_189c3;
        }
        if (!(bool)(v2 & v23 & v29 != 0)) {
          v20 = 0x5c;
          v16 = 0x5c;
          v24 = 0;
          goto label_18da0;
        }
label_191da:
        v22 += 1;
        v24 = 0;
        v16 = 0x5c;
        v7 = v19;
        goto label_191f0;
      }
      switch(v16) {
        case 0:
          if (!v2) {
            v21 = 0;
            v20 = 0;
            if (v33 & 1) goto label_18e72;
            goto label_18dab;
          }
label_18cfe:
          if (!v23) {
            v21 = 0;
label_18d0c:
            v26 = a4 == 2;
            v7 = v19 ^ 1;
            v7 &= v26;
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
                v24 = 0;
                v16 = 0x30;
                v19 = (bool)v7;
                goto label_18a83;
              }
label_19615:
              a0[v13] = 0x5c;
              v12 = v13;
              v19 = (bool)v7;
            }
            else if (v12 < a1) {
              v7 = v19;
              goto label_19615;
            }
            v13 = v12 + 1;
            if (a4 != 2) {
              v20 = 0x30;
              if ((v22 + 1 < a3) && (v7 = a2[v22 + 1] - 0x30, v7 <= 9)) {
                if (v13 < a1)
                  a0[v13] = 0x30;
                if (v12 + 2 < a1)
                  a0[v12 + 2] = 0x30;
                v13 = v12 + 3;
                v20 = 0x30;
              }
label_1964c:
              v16 = (unsigned char)v20;
              v24 = 0;
              v12 = v13;
              v7 = v25;
              if (!v2 || v26) goto label_18a83;
              goto label_188d8;
            }
            v16 = 0x30;
            v24 = 0;
            v12 = v13;
            v7 = v25;
            goto label_18a83;
          }
          if (a4 != 2) goto label_189cc;
          goto label_18c3b;
        default:
          goto label_189e3;
        case 7:
          v20 = 7;
          v16 = 0x61;
          break;
        case 8:
          v20 = 8;
          v16 = 0x62;
          break;
        case 9:
          v20 = 9;
          v16 = 0x74;
          goto label_18e2f;
        case 10:
          v20 = 10;
          v16 = 0x6e;
          goto label_18e2f;
        case 0xb:
          v20 = 0xb;
          v16 = 0x76;
          break;
        case 0xc:
          v20 = 0xc;
          v16 = 0x66;
          break;
        case 0xd:
          v20 = 0xd;
          v16 = 0x72;
label_18e2f:
          v7 = a4 == 2 & v23;
          v24 = 0;
          if (!(bool)v7) goto label_18da0;
label_189b1:
          a4 = 2;
label_189c3:
          if (!v27) goto label_189cc;
          goto label_18c3b;
        case 0x20:
          v18 = 0x20;
          goto label_18999;
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
label_18996:
          v25 = 0;
          goto label_18999;
        case 0x23:
label_18d70:
          v20 = 0x23;
label_18d75:
          v21 = v24;
          if (v22) {
label_19242:
            v7 = a4 == 2;
            v25 = v3;
            v24 = 0;
            goto label_18a59;
          }
          v18 = v20;
label_18999:
          v20 = v18;
          v21 = v24;
          v24 = v25;
          if ((a4 == 2) && (v23)) goto label_189b1;
          goto label_18a57;
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
          goto label_18a50;
        case 0x27:
label_18be2:
          v24 = v25;
          if (a4 != 2) {
            v7 = 0;
            v20 = 0x27;
            goto label_18a59;
          }
          if (!v23) {
            if (a1) { // branch-flip
              v13 = 0;
              v18 = a1;
              if (v31) goto label_1966b;
            }
            else {
label_1966b:
              if (v12 < a1)
                a0[v12] = 0x27;
              if (v12 + 1 < a1)
                a0[v12 + 1] = 0x5c;
              v13 = a1;
              v18 = v31;
              if (v12 + 2 < a1)
                a0[v12 + 2] = 0x27;
            }
            v12 += 3;
            v7 = 0;
            v16 = 0x27;
            a1 = v13;
            v31 = v18;
            v19 = 0;
            v3 = v25;
            goto label_18a83;
          }
          goto label_189c3;
        case 0x3f:
label_18ba1:
          if (a4 != 2) {
            if ((((a4 == 5) && (v33 & 4)) && (v18 = v22 + 2, v18 < a3)) && (a2[v22 + 1] == '?')) {
              v7 = a2[v18];
              v20 = (unsigned long)v7;
              if (v7 <= 0x3e) {
                v9 = (unsigned long)(0x7000a38200000000 >> (v7 & 0x3f));
                v1 = (unsigned long)v9 & 1;
                v7 = (unsigned char)v9 & 1;
                if (!v1) {
                  v20 = 0x3f;
                  v25 = v3;
                  v24 = 0;
                  goto label_18a59;
                }
                if (v23) goto label_189cc;
                if (v12 < a1)
                  a0[v12] = 0x3f;
                if (v12 + 1 < a1)
                  a0[v12 + 1] = 0x22;
                if (v12 + 2 < a1)
                  a0[v12 + 2] = 0x22;
                if (v12 + 3 < a1)
                  a0[v12 + 3] = 0x3f;
                v13 = v12 + 4;
                v26 = 0;
                v25 = 0;
                v22 = v18;
                goto label_1964c;
              }
            }
            v7 = 0;
            v20 = 0x3f;
            v25 = v3;
            v24 = 0;
            goto label_18a59;
          }
          if (v23) goto label_189c3;
          v7 = 0;
          v16 = 0x3f;
          v24 = 0;
          goto label_18a83;
        
      }
label_18da0:
      v21 = v24;
      if (!v2) {
label_18dab:
        v16 = (unsigned char)v20;
        v24 = 0;
        v7 = 0;
        v25 = v3;
        if (!v23) goto label_18a80;
        goto label_188d8;
      }
label_18c02:
      v24 = 0;
      goto label_18c13;
    }
    if (!(bool)(v12 == 0 & v23 & a4 == 2)) {
      v7 = a4 == 2 & (v23 ^ 1U);
      v23 = (bool)(v23 ^ 1U);
      if ((!(bool)v7) || (v23 = (bool)v7, !v3)) {
label_198cf:
        v9 = (long *)v12;
        if (((v30) && (v23)) && (v10 = *v30, v10)) {
          do {
            if (v9 < a1)
              a0[(long)v9] = v10;
            v9 = (unsigned long)((long)v9 + 1);
            v10 = v30[(long)v9 - v12];
          } while (v10);
        }
        if (v9 < a1)
          a0[(long)v9] = 0;
        return v9;
      }
      if (!v4) {
        v7 = !a1 && v31;
        v23 = v3;
        if (a1 || !v31) goto label_198cf;
        v22 = v31;
        goto label_1939b;
      }
      v12 = 0;
      a4 = 5;
      v9 = (unsigned long)__ctype_get_mb_cur_max();
      v29 = 1;
      v32 = (unsigned long)v9;
      v30 = "\"";
      if (!(v33 & 2)) goto label_1997f;
      v3 = 0;
      v19 = 0;
      v31 = 0;
      v2 = v4;
      v23 = v4;
      goto label_18796;
    }
label_1909d:
    if (v2) {
label_18c3b:
      v2 = 1;
      v33 &= 0xfffffffd;
      v9 = (unsigned long)__ctype_get_mb_cur_max();
      v28 = 0;
      v32 = (unsigned long)v9;
    }
    else {
      v2 = 0;
      v33 &= 0xfffffffd;
      v9 = (unsigned long)__ctype_get_mb_cur_max();
      v28 = 0;
      v32 = (unsigned long)v9;
    }
label_18c61:
    v23 = 0;
    a4 = 2;
    v12 = 1;
    v30 = "\'";
    v29 = 1;
    v19 = 0;
    v4 = 1;
    if (a1) { // branch-flip
      v7 = 0;
      v22 = 0;
      v31 = a1;
label_1939b:
      v12 = 1;
      *a0 = 0x27;
      a4 = 2;
      v23 = 0;
      v29 = 1;
      v30 = "\'";
      v13 = v31;
      v31 = v22;
      v3 = (bool)v7;
    }
    else {
      v31 = 0;
      v13 = a1;
      v3 = 0;
    }
  } while( true );
  while( true ) {
    v20 += 1;
    v9 = (unsigned long)(v22 + v20);
    if (a3 <= v9) break;
label_19845:
    if (!a2[(long)v9]) break;
  }
label_1984f:
  v24 = 0;
label_194cb:
  v18 = (unsigned long)v16;
  v25 = v24;
  if (2 <= v20) {
label_194d5:
    v25 = 0;
    v20 += v22;
    v13 = v22;
    do {
      v16 = (unsigned char)v18;
      if (v26) {
        v27 = a4 == 2;
        if (v23) goto label_189c3;
        v7 = v27 & (v19 ^ 1U);
        if ((bool)v7) {
          if (v12 < a1)
            a0[v12] = 0x27;
          if (v12 + 1 < a1)
            a0[v12 + 1] = 0x24;
          if (v12 + 2 < a1)
            a0[v12 + 2] = 0x27;
          v12 += 3;
          v19 = 1;
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
        v22 = v13 + 1;
        v12 += 3;
        v16 = (v16 & 7) + 0x30;
        if (v20 <= v22) goto label_18961;
        v25 = 1;
      }
      else {
        v7 = v25 ^ 1;
        v7 &= v19;
        if (v21) {
          if (v12 < a1)
            a0[v12] = 0x5c;
          v12 += 1;
        }
        v22 = v13 + 1;
        if (v20 <= v22) {
          goto label_191f0;
        }
        if ((bool)v7) {
          if (v12 < a1)
            a0[v12] = 0x27;
          if (v12 + 1 < a1)
            a0[v12 + 1] = 0x27;
          v12 += 2;
          v21 = 0;
          v19 = 0;
        }
        else {
          v21 = 0;
        }
      }
      v13 += 1;
      if (v12 < a1)
        a0[v12] = v16;
      v18 = (unsigned long)(unsigned char)a2[v13];
      v12 += 1;
    } while( true );
  }
label_18a3e:
  if (v26) {
    v24 = 0;
    v26 = v2;
    goto label_194d5;
  }
label_18a50:
  v20 = v18;
  v24 = v25;
label_18a57:
  v7 = a4 == 2;
  v25 = v3;
label_18a59:
  v16 = (unsigned char)v20;
  v26 = (bool)v7 == 0;
  v3 = v25;
  v7 = 0;
  if ((v26 && v2) || (v7 = 0, v23)) {
label_188d8:
    v16 = (unsigned char)v20;
    v12 = v13;
    if (!v28) goto label_18a83;
    if (!(*(unsigned int *)(v28 + (v20 >> 5) * 4) >> (v16 & 0x1f) & 1)) goto label_18a83;
  }
  else {
label_18a80:
    v7 = 0;
    v3 = v25;
label_18a83:
    if (!v21) {
      v7 ^= 1;
      v22 += 1;
      v7 &= v19;
label_191f0:
      if ((bool)v7) {
        if (v12 < a1)
          a0[v12] = 0x27;
        if (v12 + 1 < a1)
          a0[v12 + 1] = 0x27;
        v19 = 0;
        v12 += 2;
      }
      goto label_18961;
    }
  }
label_18c13:
  if (v23) {
    v23 = v2;
label_18c2f:
    if ((bool)(v23 & a4 == 2)) goto label_18c3b;
label_189cc:
    v33 &= 0xfffffffd;
    v28 = 0;
    goto label_186c0;
  }
  v7 = v19 ^ 1;
  v7 &= a4 == 2;
  if ((bool)v7) {
    if (v12 < a1)
      a0[v12] = 0x27;
    if (v12 + 1 < a1)
      a0[v12 + 1] = 0x24;
    if (v12 + 2 < a1)
      a0[v12 + 2] = 0x27;
    v12 += 3;
    v19 = 1;
  }
label_1894e:
  if (v12 < a1)
    a0[v12] = 0x5c;
  v12 += 1;
  v22 += 1;
label_18961:
  if (v12 < a1)
    a0[v12] = v16;
  v12 += 1;
  if (!v24)
    v4 = 0;
  goto label_187a0;
label_18e72:
  v22 += 1;
  goto label_187a0;
label_1997f:
  a1 = v31;
label_18aa1:
  v23 = 0;
  a4 = 5;
  v12 = 1;
  v30 = "\"";
  v2 = 1;
  v29 = 1;
  v19 = 0;
  v3 = 0;
  v4 = 1;
  v31 = 0;
  v13 = a1;
  if (a1)
    *a0 = 0x22;
  goto label_18783;
}

// Function: sub_19a80 @ 0x19a80
char * sub_19a80(unsigned int a0,char *a1,unsigned long a2,struct_47 *a3)
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
    v5 = dat_26100;
    if (dat_260f8 <= (int)a0) {
      v10 = (long)dat_260f8;
      v8 = (long)(int)((a0 - dat_260f8) + 1);
      if (dat_26100 != (unsigned long *)0x26110) // branch-flip
        v5 = (unsigned long *)sub_16100(dat_26100,&v10,v8,0x7fffffff,0x10);
      else {
        v5 = (unsigned long *)sub_16100(NULL,&v10,v8,0x7fffffff,0x10);
        *v5 = dat_26110;
        v5[1] = dat_26118;
      }
      dat_26100 = v5;
      memset(&v5[(long)dat_260f8 * 2],0,(v10 - dat_260f8) * 0x10);
      dat_260f8 = (int)v10;
    }
    v1 = &v5[(long)(int)a0 * 2];
    v4 = *v1;
    v7 = (char *)v1[1];
    v9 = a3->field_0x4 | 1;
    v6 = sub_18650(v7,v4,a1,a2,a3->field_0x0,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v4 <= v6) {
      v6 += 1;
      *v1 = v6;
      if (v7 != (char *)0x368a0)
        free(v7);
      v7 = (char *)sub_15f70(v6);
      v3 = a3->field_0x0;
      v1[1] = (unsigned long)v7;
      sub_18650(v7,v6,a1,a2,v3,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *v11 = v2;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_19c70 @ 0x19c70
void sub_19c70(void *a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = (void *)0x369a0;
  sub_16240(a0,0x38);
  *v2 = v1;
}

// Function: sub_19cb0 @ 0x19cb0
unsigned int sub_19cb0(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x369a0;
  return *a0;
}

// Function: sub_19cd0 @ 0x19cd0
void sub_19cd0(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x369a0;
  *a0 = a1;
}

// Function: sub_19cf0 @ 0x19cf0
unsigned int sub_19cf0(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0x369a0;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_19d30 @ 0x19d30
unsigned int sub_19d30(void *a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = (void *)0x369a0;
  v1 = *(unsigned int *)((long)a0 + 4);
  *(unsigned int *)((long)a0 + 4) = a1;
  return v1;
}

// Function: sub_19d50 @ 0x19d50
void sub_19d50(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0x369a0;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_19d90 @ 0x19d90
void sub_19d90(char *a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x369a0;
  v2 = __errno_location();
  v1 = *v2;
  sub_18650(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_19e10 @ 0x19e10
char * sub_19e10(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  char *v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x369a0;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_18650(NULL,0,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = (char *)sub_15f70(v3 + 1U);
  sub_18650(v4,v3 + 1U,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_19f00 @ 0x19f00
char * sub_19f00(char *a0,unsigned long a1,unsigned int *a2)
{
  return sub_19e10(a0,a1,NULL,a2); // tail-call
}

// Function: sub_19f10 @ 0x19f10
void sub_19f10(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_26100;
  if (2 <= dat_260f8) {
    v4 = (unsigned long *)((long)dat_26100 + 0x18);
    v1 = (unsigned long *)((long)dat_26100 + (unsigned long)(unsigned int)(dat_260f8 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0x368a0) {
    free(*(void **)((long)v3 + 8));
    dat_26118 = 0x368a0;
    dat_26110 = 0x100;
  }
  if (v3 == (void *)0x26110) {
    dat_260f8 = 1;
    return;
  }
  free(v3);
  dat_260f8 = 1;
  dat_26100 = (void *)0x26110;
}

// Function: sub_19fb0 @ 0x19fb0
char * sub_19fb0(unsigned int a0,char *a1)
{
  return sub_19a80(a0,a1,0xffffffffffffffff,(struct_47 *)0x369a0); // tail-call
}

// Function: sub_19fd0 @ 0x19fd0
char * sub_19fd0(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_19a80(a0,a1,a2); // tail-call
}

// Function: sub_19fe0 @ 0x19fe0
char * sub_19fe0(char *a0)
{
  return sub_19fb0(0,a0); // tail-call
}

// Function: sub_19ff0 @ 0x19ff0
char * sub_19ff0(char *a0,unsigned long a1)
{
  return sub_19fd0(0,a0,a1); // tail-call
}

// Function: sub_1a000 @ 0x1a000
void sub_1a000(unsigned int a0,unsigned long a1,char *a2)
{
  char v1 [56];
  
  sub_17f00(v1);
  sub_19a80(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_1a060 @ 0x1a060
void sub_1a060(unsigned int a0,unsigned long a1,char *a2,unsigned long a3)
{
  char v1 [56];
  
  sub_17f00(v1);
  sub_19a80(a0,a2,a3,v1);
}

// Function: sub_1a0d0 @ 0x1a0d0
void sub_1a0d0(unsigned int a0,char *a1)
{
  sub_1a000(0,a0,a1); // tail-call
}

// Function: sub_1a0e0 @ 0x1a0e0
void sub_1a0e0(unsigned int a0,char *a1,unsigned long a2)
{
  sub_1a060(0,a0,a1,a2); // tail-call
}

// Function: sub_1a100 @ 0x1a100
void sub_1a100(char *a0,unsigned long a1,char a2,unsigned long a3)
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
  v1 = dat_369a0;
  v3 = dat_369a8;
  v4 = dat_369b0;
  v5 = dat_369b8;
  v8 = dat_369d0;
  v6 = dat_369c0;
  v7 = dat_369c8;
  sub_19cf0(&v1,(int)a2,1,a3,a0,a1);
  sub_19a80(0,a0,a1,v2);
}

// Function: sub_1a190 @ 0x1a190
void sub_1a190(char *a0,char a1)
{
  sub_1a100(a0,0xffffffffffffffff,(int)a1); // tail-call
}

// Function: sub_1a1b0 @ 0x1a1b0
void sub_1a1b0(char *a0)
{
  sub_1a190(a0,0x3a); // tail-call
}

// Function: sub_1a1c0 @ 0x1a1c0
void sub_1a1c0(char *a0,unsigned long a1)
{
  sub_1a100(a0,a1,0x3a); // tail-call
}

// Function: sub_1a1d0 @ 0x1a1d0
void sub_1a1d0(unsigned int a0,unsigned long a1,char *a2)
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
  
  sub_17f00(&v1);
  v3 = &v2;
  v2 = v1;
  v10 = v4;
  v15 = v9;
  v11 = v5;
  v12 = v6;
  v13 = v7;
  v14 = v8;
  sub_19cf0(v3,0x3a,1);
  sub_19a80(a0,a2,0xffffffffffffffff,v3);
}

// Function: sub_1a270 @ 0x1a270
void sub_1a270(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x68
  unsigned long v2; // stack - 0x60
  unsigned long v3; // stack - 0x58
  unsigned long v4; // stack - 0x50
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  
  v1 = dat_369a0;
  v2 = dat_369a8;
  v7 = dat_369d0;
  v3 = dat_369b0;
  v4 = dat_369b8;
  v5 = dat_369c0;
  v6 = dat_369c8;
  sub_19d50(&v1,a1,a2);
  sub_19a80(a0,a3,a4,&v1);
}

// Function: sub_1a310 @ 0x1a310
void sub_1a310(unsigned int a0)
{
  sub_1a270(a0); // tail-call
}

// Function: sub_1a320 @ 0x1a320
void sub_1a320(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_1a310(0,a0,a1,a2); // tail-call
}

// Function: sub_1a340 @ 0x1a340
void sub_1a340(long a0,long a1,char *a2,unsigned long a3)
{
  sub_1a270(0,a0,a1,a2,a3); // tail-call
}

// Function: sub_1a360 @ 0x1a360
char * sub_1a360(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_19a80(a0,a1,a2); // tail-call
}

// Function: sub_1a370 @ 0x1a370
char * sub_1a370(char *a0,unsigned long a1)
{
  return sub_1a360(0,a0,a1); // tail-call
}

// Function: sub_1a380 @ 0x1a380
char * sub_1a380(unsigned int a0,char *a1)
{
  return sub_1a360(a0,a1,0xffffffffffffffff); // tail-call
}

// Function: sub_1a390 @ 0x1a390
char * sub_1a390(char *a0)
{
  return sub_1a380(0,a0); // tail-call
}

// Function: sub_1a3a0 @ 0x1a3a0
int sub_1a3a0(void) // early-return x2
{
  int v1; // eax
  int v2; // ebx
  int v3; // ebp
  
  v3 = 1;
  v2 = 0;
  do {
    v1 = sub_1acf0(v2,1);
    if (0 <= v1) { // branch-flip
label_1a3d3:
      if (v3 == 3) {
        v1 = 0;
        return v1;
      }
    }
    else {
      if ((v2) || (v1 = __open_2("/dev/full",1), v1 < 0)) {
        v1 = __open_2("/dev/null",(unsigned int)(v2 == 0));
        if (v1 < 0) {
          v1 = *__errno_location();
          return v1;
        }
        if (3 <= v1) {
          close(v1); // return-dupe
          v1 = 0;
          return v1;
        }
        goto label_1a3d3;
      }
      if (3 <= v1) {
        close(v1);
        v1 = 0;
        return v1;
      }
    }
    v2 += 1;
    v3 += 1;
  } while( true );
}

// Function: sub_1a450 @ 0x1a450
unsigned long sub_1a450(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4) // return-dupe
{
  int v1;
  char *v2; // rax
  char *v3;
  char *v4; // stack - 0x1050
  char *v5; // stack - 0x1058
  char v6 [4104];
  unsigned long v7; // stack - 0x1068
  long v8; // stack - 0x1060
  
  v2 = NULL;
  iconv(a2,0,0,0,0);
  v7 = a0;
  v8 = a1;
  if (a1) {
    do {
      v4 = (char *)0x1000;
      v5 = v6;
      if (iconv(a2,&v7,&v8,&v5,&v4) == -1) {
        v1 = *__errno_location();
        if (v1 != 7) {
          if (v1 != 0x16)
            return 0xffffffff;
          break;
        }
      }
      v2 = &v5[(long)v2 - (long)v6];
    } while (v8);
  }
  v4 = (char *)0x1000;
  v5 = v6;
  if (iconv(a2,0,0,&v5,&v4) == -1)
    return 0xffffffff;
  v2 = &v5[(long)v2 - (long)v6];
  if (!v2) {
    *a4 = 0;
    return 0;
  }
  v3 = (char *)*a3;
  if (((!v3) || ((char *)*a4 < v2)) && (v3 = malloc((unsigned long)v2), !v3)) {
    *__errno_location() = 0xc;
    return 0xffffffff;
  }
  iconv(a2,0,0,0,0);
  v7 = a0;
  v8 = a1;
  v5 = v3;
  v4 = v2;
  do {
    if (!a1) {
label_1a5f2:
      if (iconv(a2,0,0,&v5,&v4) != -1) {
        if (v4)
          abort(); // no-return
        *a3 = (long)v3;
        *a4 = (unsigned long)v2;
        return 0;
      }
label_1a69e:
      if ((char *)*a3 != v3) {
        free(v3);
        return 0xffffffff;
      }
      return 0xffffffff;
    }
    if (iconv(a2,&v7,&v8,&v5,&v4) == -1) {
      if (*__errno_location() != 0x16) goto label_1a69e;
      goto label_1a5f2;
    }
    a1 = v8;
  } while( true );
}

// Function: sub_1a6d0 @ 0x1a6d0
char * sub_1a6d0(char *a0,unsigned long a1) // return-dupe
{
  unsigned long v1;
  unsigned long v10;
  char *v11;
  long v12; // rdx
  char *v2;
  char *v3; // rax
  int *v4; // rax
  unsigned long v5;
  unsigned long v6; // stack - 0x48
  char *v7; // stack - 0x50
  unsigned long v8; // stack - 0x58
  char *v9; // stack - 0x60
  
  v9 = a0;
  v8 = strlen(a0);
  v10 = v8;
  if (v8 <= 0xfffffff)
    v10 = v8 << 4;
  v5 = v10 + 1;
  v2 = malloc(v5);
  if (!v2) {
    *__errno_location() = 0xc;
    return NULL;
  }
  iconv(a1,0,0,0,0,v10,v10);
  v11 = v2;
  while (v7 = v2, v6 = v10, iconv(a1,&v9,&v8,&v7,&v6) == -1) {
    v4 = __errno_location();
    if (*v4 == 0x16) goto label_1a86d;
    if (*v4 != 7) {
      free(v11); // return-dupe
      return NULL;
    }
    v1 = v5 * 2;
    v12 = (long)v7 - (long)v11;
    if ((v1 <= v5) || (v3 = realloc(v11,v1), !v3)) {
      *v4 = 0xc;
      free(v11);
      return NULL;
    }
    v2 = &v3[v12];
    v10 = (v1 - 1) - v12;
    v5 = v1;
    v11 = v3;
  }
label_1a86d:
  do {
    if (iconv(a1,0,0,&v7,&v6) != -1) {
      v2 = &v7[1];
      *v7 = 0;
      v10 = (long)v2 - (long)v11;
      if (v5 <= v10)
        return v11;
      v7 = v2;
      v2 = realloc(v11,v10);
      if (!v2)
        return v11;
      return v2;
    }
    v4 = __errno_location();
    if (*v4 != 7) {
      free(v11);
      return NULL;
    }
    v10 = v5 * 2;
    v12 = (long)v7 - (long)v11;
    if ((v10 <= v5) || (v2 = realloc(v11,v10), !v2)) {
      *v4 = 0xc;
      free(v11);
      return NULL;
    }
    v7 = &v2[v12];
    v6 = (v10 - 1) - v12;
    v5 = v10;
    v11 = v2;
  } while( true );
}

// Function: sub_1a920 @ 0x1a920
char * sub_1a920(char *a0,char *a1,char *a2)
{
  int v1;
  char *v2; // rax
  long v3; // rax
  int *v4; // rax
  
  if ((!*a0) || (!sub_16d30(a1,a2))) {
    v2 = strdup(a0);
    if (!v2)
      *__errno_location() = 0xc;
    return v2;
  }
  v3 = iconv_open(a2,a1);
  if (v3 == -1)
    return NULL;
  v2 = (char *)sub_1a6d0(a0,v3);
  if (v2) {
    if (0 <= (int)iconv_close(v3))
      return v2;
    free(v2);
    return NULL;
  }
  v4 = __errno_location();
  v1 = *v4;
  iconv_close(v3);
  *v4 = v1;
  return NULL;
}

// Function: sub_1a9e0 @ 0x1a9e0
char * sub_1a9e0(long a0,unsigned long *a1)
{
  unsigned long *v1;
  char *v10;
  unsigned long v11; // stack - 0x58
  unsigned long *v12; // stack - 0x50
  unsigned int v2;
  char *v3;
  unsigned long v4; // rax
  unsigned long v5; // rax
  char *v6;
  unsigned long v7;
  unsigned long *v8;
  long v9;
  
  v11 = *a1;
  v12 = (unsigned long *)a1[1];
  v7 = a1[2];
  if (a0) { // branch-flip
    v4 = 0;
    v9 = a0;
    do {
      v5 = v11 & 0xffffffff;
      if (0x30 <= (unsigned int)v11) { // branch-flip
        v1 = &v12[1];
        v8 = v12;
        v12 = v1;
      }
      else {
        v11 = (unsigned long)((unsigned int)v11 + 8);
        v8 = (unsigned long *)(v5 + v7);
      }
      v3 = (char *)*v8;
      v4 = sub_1af00(v4,strlen(v3));
      v9 -= 1;
    } while (v9);
    if (0x80000000 <= v4) {
      *__errno_location() = 0x4b;
      return NULL;
    }
    v6 = (char *)sub_15f50(v4 + 1);
    v10 = v6;
    do {
      v2 = (unsigned int)*a1;
      if (0x30 <= v2) { // branch-flip
        v8 = (unsigned long *)a1[1];
        a1[1] = (unsigned long)&v8[1];
      }
      else {
        v8 = (unsigned long *)((unsigned long)v2 + a1[2]);
        *(unsigned int *)a1 = v2 + 8;
      }
      v3 = (char *)*v8;
      v7 = strlen(v3);
      memcpy(v10,v3,v7);
      v10 = &v10[v7];
      a0 -= 1;
    } while (a0);
  }
  else {
    v10 = (char *)sub_15f50(1);
    v6 = v10;
  }
  *v10 = 0;
  return v6;
}

// Function: sub_1ab40 @ 0x1ab40
unsigned long sub_1ab40(char *a0,unsigned long *a1) // early-return
{
  char v1;
  char *v2; // stack - 0x18
  long v3;
  
  v1 = *a0;
  v3 = 0;
  while( true ) {
    if (!v1)
      return sub_1a9e0(v3,a1); // tail-call
    if ((v1 != '%') || (a0[v3 * 2 + 1] != 's')) break;
    v3 += 1;
    v1 = a0[v3 * 2];
  }
  if (0 <= __vasprintf_chk(&v2,1,a0,a1))
    return v2;
  if (*__errno_location() != 0xc)
    return 0;
  sub_16300(); // no-return
}

// Function: sub_1ac00 @ 0x1ac00
void sub_1ac00(int a0,unsigned long a1)
{
  fcntl(a0,0,a1 & 0xffffffff); // tail-call
}

// Function: sub_1ac10 @ 0x1ac10
int sub_1ac10(unsigned long a0,unsigned long a1)
{
  int v1;
  int v2; // eax
  unsigned int v3; // eax
  int *v4; // rax
  
  if (0 <= dat_369d8) { // branch-flip
    v2 = fcntl((int)a0,0x406,a1 & 0xffffffff);
    if ((0 <= v2) || (*__errno_location() != 0x16)) {
      dat_369d8 = 1;
      return v2;
    }
    v2 = sub_1ac00(a0 & 0xffffffff,a1 & 0xffffffff);
    if (v2 < 0)
      return v2;
    dat_369d8 = -1;
  }
  else {
    v2 = sub_1ac00(a0,a1);
    if (v2 < 0)
      return v2;
    if (dat_369d8 != -1)
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

// Function: sub_1acf0 @ 0x1acf0
void sub_1acf0(int a0,int a1,unsigned int a2)
{
  unsigned long v1;
  unsigned long v2; // stack - 0x28
  
  v2 = CONCAT44(v2._4_4_,a2);
  if (!a1) {
    sub_1ac00(a0,a2);
    return;
  }
  if (a1 == 0x406) {
    sub_1ac10(a0,a2);
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

// Function: sub_1ae40 @ 0x1ae40
unsigned long sub_1ae40(char *a0)
{
  char v1 [4]; // stack - 0x68
  unsigned long v2; // r12
  unsigned long v3; // stack - 0x64
  char v4; // stack - 0x5c
  char *v5; // stack - 0x58
  long v6; // stack - 0x50
  char v7; // stack - 0x48
  int v8; // stack - 0x44
  
  if (__ctype_get_mb_cur_max() <= 1)
    return strlen(a0); // tail-call
  v2 = 0;
  v1[0] = 0;
  v3 = 0;
  v5 = a0;
  while( true ) {
    v4 = 0;
    sub_11f60(v1);
    if ((v7) && (!v8)) break;
    v5 = &v5[v6];
    v2 += 1;
  }
  return v2;
}

// Function: sub_1af00 @ 0x1af00
long sub_1af00(unsigned long a0,unsigned long a1) // early-return
{
  if (CARRY8(a0,a1))
    return -1;
  return a0 + a1;
}

// Function: sub_1af20 @ 0x1af20
void sub_1af20(unsigned long a0,unsigned long a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rdx
  
  sub_1af00(a0,a1);
  sub_1af00(v1,v2); // tail-call
}

// Function: sub_1af40 @ 0x1af40
void sub_1af40(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  unsigned long v1; // rax
  unsigned long v2; // rdx
  
  sub_1af00(a0,a1);
  sub_1af00(sub_1af00(v1,v2),a3); // tail-call
}

// Function: sub_1af60 @ 0x1af60
unsigned long sub_1af60(unsigned long a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = a1;
  if (a1 <= a0)
    v1 = a0;
  return v1;
}

// Function: _DT_FINI @ 0x1af70
void _DT_FINI(void)
{
  return;
}
