// Function: _DT_INIT @ 0x4000
void _DT_INIT(void) // return-dupe
{
  if (!dat_2cfe8)
    return;
  (*dat_2cfe8)();
}

// Function: sub_4020 @ 0x4020
void sub_4020(void)
{
  (*dat_2cc00)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x47c0
void __cxa_finalize(void)
{
  (*dat_2cff8)(); // jump-as-call
}

// Function: getenv @ 0x47d0
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2cc08)(); // jump-as-call
  return v1;
}

// Function: sigprocmask @ 0x47e0
int sigprocmask(int a0,sigset_t *a1,sigset_t *a2)
{
  int v1; // eax
  
  v1 = (*dat_2cc10)(); // jump-as-call
  return v1;
}

// Function: raise @ 0x47f0
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_2cc18)(); // jump-as-call
  return v1;
}

// Function: free @ 0x4800
void free(void *a0)
{
  (*dat_2cc20)(); // jump-as-call
}

// Function: strtoimax @ 0x4810
void strtoimax(void)
{
  (*dat_2cc28)(); // jump-as-call
}

// Function: strcasecmp @ 0x4820
int strcasecmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_2cc30)(); // jump-as-call
  return v1;
}

// Function: localtime @ 0x4830
tm * localtime(void *a0)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_2cc38)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x4840
void abort(void)
{
  (*dat_2cc40)(); // jump-as-call
}

// Function: __errno_location @ 0x4850
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_2cc48)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x4860
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_2cc50)(); // jump-as-call
  return v1;
}

// Function: localtime_r @ 0x4870
tm * localtime_r(void *a0,tm *a1)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_2cc58)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x4880
void _exit(int a0)
{
  (*dat_2cc60)(); // jump-as-call
}

// Function: strcpy @ 0x4890
char * strcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2cc68)(); // jump-as-call
  return v1;
}

// Function: toupper @ 0x48a0
int toupper(int a0)
{
  int v1; // eax
  
  v1 = (*dat_2cc70)(); // jump-as-call
  return v1;
}

// Function: iconv @ 0x48b0
void iconv(void)
{
  (*dat_2cc78)(); // jump-as-call
}

// Function: ferror @ 0x48c0
int ferror(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_2cc80)(); // jump-as-call
  return v1;
}

// Function: qsort @ 0x48d0
void qsort(void)
{
  (*dat_2cc88)(); // jump-as-call
}

// Function: isatty @ 0x48e0
int isatty(int a0)
{
  int v1; // eax
  
  v1 = (*dat_2cc90)(); // jump-as-call
  return v1;
}

// Function: sigaction @ 0x48f0
int sigaction(int a0,sigaction *a1,sigaction *a2)
{
  int v1; // eax
  
  v1 = (*dat_2cc98)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x4900
int iswcntrl(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_2cca0)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x4910
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2cca8)(); // jump-as-call
  return v1;
}

// Function: readlink @ 0x4920
long readlink(char *a0,char *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_2ccb0)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x4930
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_2ccb8)(); // jump-as-call
  return v1;
}

// Function: clock_gettime @ 0x4940
int clock_gettime(int a0,timespec *a1)
{
  int v1; // eax
  
  v1 = (*dat_2ccc0)(); // jump-as-call
  return v1;
}

// Function: setenv @ 0x4950
int setenv(char *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_2ccc8)(); // jump-as-call
  return v1;
}

// Function: write @ 0x4960
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_2ccd0)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x4970
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2ccd8)(); // jump-as-call
  return v1;
}

// Function: vasprintf @ 0x4980
int vasprintf(char **a0,char *a1,void *a2)
{
  int v1; // eax
  
  v1 = (*dat_2cce0)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x4990
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_2cce8)(); // jump-as-call
  return v1;
}

// Function: opendir @ 0x49a0
DIR * opendir(char *a0)
{
  DIR *v1; // rax
  
  v1 = (DIR *)(*dat_2ccf0)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x49b0
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2ccf8)(); // jump-as-call
  return v1;
}

// Function: regfree @ 0x49c0
void regfree(re_pattern_buffer *a0)
{
  (*dat_2cd00)(); // jump-as-call
}

// Function: __ctype_get_mb_cur_max @ 0x49d0
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_2cd08)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x49e0
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_2cd10)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x49f0
void __stack_chk_fail(void)
{
  (*dat_2cd18)(); // jump-as-call
}

// Function: mmap @ 0x4a00
void mmap(void)
{
  (*dat_2cd20)(); // jump-as-call
}

// Function: getopt_long @ 0x4a10
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_2cd28)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x4a20
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_2cd30)(); // jump-as-call
  return v1;
}

// Function: gettext @ 0x4a30
char * gettext(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2cd38)(); // jump-as-call
  return v1;
}

// Function: dup2 @ 0x4a40
int dup2(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_2cd40)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x4a50
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2cd48)(); // jump-as-call
  return v1;
}

// Function: printf @ 0x4a60
int printf(char *a0,...)
{
  int v1; // eax
  
  v1 = (*dat_2cd50)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x4a70
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2cd58)(); // jump-as-call
  return v1;
}

// Function: gmtime_r @ 0x4a80
tm * gmtime_r(void *a0,tm *a1)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_2cd60)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x4a90
void lseek(void)
{
  (*dat_2cd68)(); // jump-as-call
}

// Function: __assert_fail @ 0x4aa0
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_2cd70)(); // jump-as-call
}

// Function: fputs @ 0x4ab0
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_2cd78)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x4ac0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2cd80)(); // jump-as-call
  return v1;
}

// Function: strnlen @ 0x4ad0
unsigned long strnlen(char *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_2cd88)(); // jump-as-call
  return v1;
}

// Function: close @ 0x4ae0
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_2cd90)(); // jump-as-call
  return v1;
}

// Function: pipe @ 0x4af0
int pipe(int *a0)
{
  int v1; // eax
  
  v1 = (*dat_2cd98)(); // jump-as-call
  return v1;
}

// Function: closedir @ 0x4b00
int closedir(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_2cda0)(); // jump-as-call
  return v1;
}

// Function: regcomp @ 0x4b10
int regcomp(re_pattern_buffer *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_2cda8)(); // jump-as-call
  return v1;
}

// Function: memchr @ 0x4b20
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2cdb0)(); // jump-as-call
  return v1;
}

// Function: read @ 0x4b30
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_2cdb8)(); // jump-as-call
  return v1;
}

// Function: lstat @ 0x4b40
int lstat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_2cdc0)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x4b50
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_2cdc8)(); // jump-as-call
  return v1;
}

// Function: _setjmp @ 0x4b60
int _setjmp(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_2cdd0)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x4b70
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_2cdd8)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x4b80
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2cde0)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x4b90
int ferror_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_2cde8)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x4ba0
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2cdf0)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x4bb0
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_2cdf8)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x4bc0
void signal(void)
{
  (*dat_2ce00)(); // jump-as-call
}

// Function: fputc_unlocked @ 0x4bd0
int fputc_unlocked(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_2ce08)(); // jump-as-call
  return v1;
}

// Function: fprintf @ 0x4be0
int fprintf(FILE *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_2ce10)(); // jump-as-call
  return v1;
}

// Function: sigemptyset @ 0x4bf0
int sigemptyset(sigset_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_2ce18)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x4c00
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_2ce20)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x4c10
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2ce28)(); // jump-as-call
  return v1;
}

// Function: putchar_unlocked @ 0x4c20
int putchar_unlocked(int a0)
{
  int v1; // eax
  
  v1 = (*dat_2ce30)(); // jump-as-call
  return v1;
}

// Function: tzset @ 0x4c30
void tzset(void)
{
  (*dat_2ce38)(); // jump-as-call
}

// Function: fileno @ 0x4c40
int fileno(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_2ce40)(); // jump-as-call
  return v1;
}

// Function: readdir @ 0x4c50
dirent * readdir(DIR *a0)
{
  dirent *v1; // rax
  
  v1 = (dirent *)(*dat_2ce48)(); // jump-as-call
  return v1;
}

// Function: mincore @ 0x4c60
int mincore(void *a0,unsigned long a1,char *a2)
{
  int v1; // eax
  
  v1 = (*dat_2ce50)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x4c70
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_2ce58)(); // jump-as-call
  return v1;
}

// Function: tolower @ 0x4c80
int tolower(int a0)
{
  int v1; // eax
  
  v1 = (*dat_2ce60)(); // jump-as-call
  return v1;
}

// Function: putc_unlocked @ 0x4c90
int putc_unlocked(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_2ce68)(); // jump-as-call
  return v1;
}

// Function: re_set_syntax @ 0x4ca0
void re_set_syntax(void)
{
  (*dat_2ce70)(); // jump-as-call
}

// Function: iswalnum @ 0x4cb0
int iswalnum(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_2ce78)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x4cc0
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2ce80)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x4cd0
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2ce88)(); // jump-as-call
  return v1;
}

// Function: regexec @ 0x4ce0
int regexec(re_pattern_buffer *a0,char *a1,unsigned long a2,void *a3,int a4)
{
  int v1; // eax
  
  v1 = (*dat_2ce90)(); // jump-as-call
  return v1;
}

// Function: strcoll @ 0x4cf0
int strcoll(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_2ce98)(); // jump-as-call
  return v1;
}

// Function: mktime @ 0x4d00
long mktime(tm *a0)
{
  long v1; // rax
  
  v1 = (*dat_2cea0)(); // jump-as-call
  return v1;
}

// Function: re_search @ 0x4d10
int re_search(re_pattern_buffer *a0,char *a1,int a2,int a3,int a4,void *a5)
{
  int v1; // eax
  
  v1 = (*dat_2cea8)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x4d20
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_2ceb0)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x4d30
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2ceb8)(); // jump-as-call
  return v1;
}

// Function: fdopen @ 0x4d40
FILE * fdopen(int a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_2cec0)(); // jump-as-call
  return v1;
}

// Function: munmap @ 0x4d50
int munmap(void *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = (*dat_2cec8)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x4d60
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2ced0)(); // jump-as-call
  return v1;
}

// Function: iconv_close @ 0x4d70
void iconv_close(void)
{
  (*dat_2ced8)(); // jump-as-call
}

// Function: timegm @ 0x4d80
long timegm(tm *a0)
{
  long v1; // rax
  
  v1 = (*dat_2cee0)(); // jump-as-call
  return v1;
}

// Function: iswspace @ 0x4d90
int iswspace(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_2cee8)(); // jump-as-call
  return v1;
}

// Function: longjmp @ 0x4da0
void longjmp(void *a0,int a1)
{
  (*dat_2cef0)(); // jump-as-call
}

// Function: strftime @ 0x4db0
unsigned long strftime(char *a0,unsigned long a1,char *a2,tm *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_2cef8)(); // jump-as-call
  return v1;
}

// Function: mempcpy @ 0x4dc0
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2cf00)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x4dd0
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2cf08)(); // jump-as-call
  return v1;
}

// Function: error @ 0x4de0
void error(int a0,int a1,char *a2,...)
{
  (*dat_2cf10)(); // jump-as-call
}

// Function: waitpid @ 0x4df0
int waitpid(int a0,int *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_2cf18)(); // jump-as-call
  return v1;
}

// Function: open @ 0x4e00
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_2cf20)(); // jump-as-call
  return v1;
}

// Function: fopen @ 0x4e10
FILE * fopen(char *a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_2cf28)(); // jump-as-call
  return v1;
}

// Function: perror @ 0x4e20
void perror(char *a0)
{
  (*dat_2cf30)(); // jump-as-call
}

// Function: towlower @ 0x4e30
unsigned int towlower(unsigned int a0)
{
  unsigned int v1; // eax
  
  v1 = (*dat_2cf38)(); // jump-as-call
  return v1;
}

// Function: unsetenv @ 0x4e40
int unsetenv(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_2cf40)(); // jump-as-call
  return v1;
}

// Function: getpagesize @ 0x4e50
int getpagesize(void)
{
  int v1; // eax
  
  v1 = (*dat_2cf48)(); // jump-as-call
  return v1;
}

// Function: sprintf @ 0x4e60
int sprintf(char *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_2cf50)(); // jump-as-call
  return v1;
}

// Function: sigismember @ 0x4e70
int sigismember(void *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_2cf58)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x4e80
void exit(int a0)
{
  (*dat_2cf60)(); // jump-as-call
}

// Function: re_compile_pattern @ 0x4e90
char * re_compile_pattern(char *a0,unsigned long a1,re_pattern_buffer *a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2cf68)(); // jump-as-call
  return v1;
}

// Function: fflush_unlocked @ 0x4ea0
int fflush_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_2cf70)(); // jump-as-call
  return v1;
}

// Function: strdup @ 0x4eb0
char * strdup(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2cf78)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x4ec0
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_2cf80)(); // jump-as-call
  return v1;
}

// Function: sigaltstack @ 0x4ed0
int sigaltstack(void *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_2cf88)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x4ee0
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_2cf90)(); // jump-as-call
  return v1;
}

// Function: fstat @ 0x4ef0
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_2cf98)(); // jump-as-call
  return v1;
}

// Function: getc @ 0x4f00
int getc(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_2cfa0)(); // jump-as-call
  return v1;
}

// Function: sigaddset @ 0x4f10
int sigaddset(sigset_t *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_2cfa8)(); // jump-as-call
  return v1;
}

// Function: execv @ 0x4f20
int execv(char *a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_2cfb0)(); // jump-as-call
  return v1;
}

// Function: fork @ 0x4f30
int fork(void)
{
  int v1; // eax
  
  v1 = (*dat_2cfb8)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x4f40
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2cfc0)(); // jump-as-call
  return v1;
}

// Function: iconv_open @ 0x4f50
void iconv_open(void)
{
  (*dat_2cfc8)(); // jump-as-call
}

// Function: sub_4f60 @ 0x4f60
void sub_4f60(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_2cfd0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_4f90 @ 0x4f90
void sub_4f90(void)
{
  return;
}

// Function: _FINI_0 @ 0x5000
void _FINI_0(void)
{
  if (!dat_2d1a8) {
    if (dat_2cff8)
      __cxa_finalize(dat_2d008);
    sub_4f90();
    dat_2d1a8 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x5040
void _INIT_0(void)
{
  return;
}

// Function: sub_5049 @ 0x5049
void sub_5049(long a0,long a1,long a2,long a3,bool a4,struct_1 *a5,long *a6) // return-dupe
{
  long v1;
  long v10; // rax
  long v11; // rax
  long v12; // rax
  bool v13; // zf
  int v14; // stack - 0x160
  int v15; // stack - 0x15c
  long v16; // stack - 0x158
  long v17; // stack - 0x150
  long v18; // stack - 0x148
  long v19; // stack - 0x140
  long v2;
  long v20; // stack - 0x138
  long v21; // stack - 0x130
  long v22; // stack - 0x128
  long v23; // stack - 0x120
  long v24; // stack - 0x118
  long v25; // stack - 0x110
  long v26; // stack - 0x108
  long v27; // stack - 0x100
  long v28; // stack - 0xf8
  long v29; // stack - 0xf0
  long v3;
  long v30; // stack - 0xe8
  long v31; // stack - 0xe0
  long v32; // stack - 0xd8
  long v33; // stack - 0xd0
  long v34; // stack - 0xc8
  long v35; // stack - 0xc0
  long v4;
  bool v5;
  long v6; // rax
  long v7; // rax
  long v8;
  long v9;
  
  v1 = a6[2];
  v2 = a6[3];
  v3 = *a6;
  v4 = a6[1];
  v6 = a0 - a2;
  v7 = a1 - a3;
  v13 = ((int)v6 - (int)v7 & 1U) != 0;
  *(long *)(v6 * 8 + v1) = a0;
  *(long *)(v7 * 8 + v2) = a1;
  v20 = 1;
  v16 = v6;
  v17 = v6;
  v18 = v7;
  v19 = v7;
  do {
    v5 = 0;
    if (v16 <= a0 - a3) // branch-flip
      v16 += 1;
    else {
      v16 -= 1;
      *(unsigned long *)(v1 + v16 * 8 + -8) = 0xffffffffffffffff;
    }
    if (a1 - a2 <= v17) // branch-flip
      v17 -= 1;
    else {
      v9 = v17 + 1;
      *(unsigned long *)(v1 + (v17 + 2) * 8) = 0xffffffffffffffff;
      v17 = v9;
    }
    for (v21 = v17; v16 <= v21; v21 = v21 + -2) {
      v9 = *(long *)(v1 + v21 * 8 + -8);
      v8 = *(long *)(v1 + (v21 + 1) * 8);
      if (v9 >= v8)
        v8 = v9 + 1;
      v23 = v8 - v21;
      for (v22 = v8; ((v22 < a1 && (v23 < a3)) && (*(long *)(v3 + v22 * 8) == *(long *)(v4 + v23 * 8))); v22 = v22 + 1) {
        v23 += 1;
      }
      if (0x15 <= v22 - v8)
        v5 = 1;
      *(long *)(v21 * 8 + v1) = v22;
      if (((v13) && (v18 <= v21)) && ((v21 <= v19 && (*(long *)(v2 + v21 * 8) <= v22)))) {
        a5->field_0x0 = v22;
        a5->field_0x8 = v23;
        a5->field_0x11 = 1;
        a5->field_0x10 = a5->field_0x11;
        return;
      }
    }
    if (v18 <= a0 - a3) // branch-flip
      v18 += 1;
    else {
      v18 -= 1;
      *(unsigned long *)(v2 + v18 * 8 + -8) = 0x7fffffffffffffff;
    }
    if (a1 - a2 <= v19) // branch-flip
      v19 -= 1;
    else {
      v9 = v19 + 1;
      *(unsigned long *)(v2 + (v19 + 2) * 8) = 0x7fffffffffffffff;
      v19 = v9;
    }
    for (v21 = v19; v18 <= v21; v21 = v21 + -2) {
      v9 = *(long *)(v2 + v21 * 8 + -8);
      v8 = *(long *)(v2 + (v21 + 1) * 8);
      if (v9 >= v8)
        v9 = v8 + -1;
      v25 = v9 - v21;
      v24 = v9;
      while (((a0 < v24 && (a2 < v25)) && (*(long *)(v3 + v24 * 8 + -8) == *(long *)(v4 + v25 * 8 + -8)))) {
        v24 -= 1;
        v25 -= 1;
      }
      if (0x15 <= v9 - v24)
        v5 = 1;
      *(long *)(v21 * 8 + v2) = v24;
      if ((((!v13) && (v16 <= v21)) && (v21 <= v17)) && (v24 <= *(long *)(v1 + v21 * 8))) {
        a5->field_0x0 = v24;
        a5->field_0x8 = v25;
        a5->field_0x11 = 1;
        a5->field_0x10 = a5->field_0x11;
        return;
      }
    }
    if (!a4) {
      if (((0xc9 <= v20) && (v5)) && ((char)a6[4])) {
        v26 = 0;
        for (v21 = v17; v16 <= v21; v21 = v21 + -2) {
          v10 = v21 - v6;
          v9 = *(long *)(v1 + v21 * 8);
          v11 = v9 - v21;
          v12 = (v9 - a0) * 2 - v10;
          v8 = -v10;
          if (-v10 <= -1)
            v8 = v10;
          if ((((v8 + v20) * 0xc < v12) && (v26 < v12)) && ((a0 + 0x13 < v9 && (((v9 < a1 && (a2 + 0x13 < v11)) && (v11 < a3)))))) {
            v14 = 1;
            while (*(long *)(v3 + (v9 - v14) * 8) == *(long *)(v4 + (v11 - v14) * 8)) {
              if (v14 == 0x14) {
                a5->field_0x0 = v9;
                a5->field_0x8 = v11;
                v26 = v12;
                break;
              }
              v14 += 1;
            }
          }
        }
        if (1 <= v26) {
          a5->field_0x10 = 1;
          a5->field_0x11 = 0;
          return;
        }
        v27 = 0;
        for (v21 = v19; v18 <= v21; v21 = v21 + -2) {
          v10 = v21 - v7;
          v9 = *(long *)(v2 + v21 * 8);
          v11 = v9 - v21;
          v12 = v10 + (a1 - v9) * 2;
          v8 = -v10;
          if (-v10 <= -1)
            v8 = v10;
          if ((((v8 + v20) * 0xc < v12) && (v27 < v12)) && (((a0 < v9 && ((v9 < a1 + -0x13 && (a2 < v11)))) && (v11 < a3 + -0x13)))) {
            v15 = 0;
            while (*(long *)(v3 + (v9 + v15) * 8) == *(long *)(v4 + (v11 + v15) * 8)) {
              if (v15 == 0x13) {
                a5->field_0x0 = v9;
                a5->field_0x8 = v11;
                v27 = v12;
                break;
              }
              v15 += 1;
            }
          }
        }
        if (1 <= v27) {
          a5->field_0x10 = 0;
          a5->field_0x11 = 1;
          return;
        }
      }
      if (a6[5] <= v20) {
        v28 = -1;
        for (v21 = v17; v16 <= v21; v21 = v21 + -2) {
          v32 = *(long *)(v1 + v21 * 8);
          if (a1 <= v32)
            v32 = a1;
          v33 = v32 - v21;
          if (a3 < v33) {
            v32 = v21 + a3;
            v33 = a3;
          }
          if (v28 < v33 + v32) {
            v28 = v33 + v32;
            v29 = v32;
          }
        }
        v30 = 0x7fffffffffffffff;
        for (v21 = v19; v18 <= v21; v21 = v21 + -2) {
          v34 = *(long *)(v2 + v21 * 8);
          if (v34 <= a0)
            v34 = a0;
          v35 = v34 - v21;
          if (v35 < a2) {
            v34 = v21 + a2;
            v35 = a2;
          }
          if (v35 + v34 < v30) {
            v30 = v35 + v34;
            v31 = v34;
          }
        }
        if ((a3 + a1) - v30 >= v28 - (a0 + a2)) {
          a5->field_0x0 = v31;
          a5->field_0x8 = v30 - v31;
          a5->field_0x10 = 0;
          a5->field_0x11 = 1;
          return;
        }
        a5->field_0x0 = v29;
        a5->field_0x8 = v28 - v29;
        a5->field_0x10 = 1;
        a5->field_0x11 = 0;
        return;
      }
    }
    v20 += 1;
  } while( true );
}

// Function: sub_5d78 @ 0x5d78
char sub_5d78(long a0,long a1,long a2,long a3,char a4,struct_2 *a5)
{
  long v1;
  char v10; // stack - 0x7b
  char v11; // stack - 0x7a
  long v12; // stack - 0x78
  long v13; // stack - 0x70
  long v14; // stack - 0x68
  long v15; // stack - 0x60
  long v16; // stack - 0x58
  long v17; // stack - 0x50
  long v18; // stack - 0x48
  long v19; // stack - 0x40
  long v2;
  long v20; // stack - 0x20
  char v21; // stack - 0x18
  char v22; // stack - 0x17
  char v3; // al
  long v4; // stack - 0x28
  char v5; // stack - 0xac
  long v6; // stack - 0xa8
  long v7; // stack - 0xa0
  long v8; // stack - 0x98
  long v9; // stack - 0x90
  
  v1 = a5->field_0x0;
  v2 = a5->field_0x8;
  v7 = a2;
  v9 = a0;
  v8 = a1;
  v6 = a3;
  v5 = a4;
  while( true ) {
    for (; ((v9 < v8 && (v7 < v6)) && (*(long *)(v1 + v9 * 8) == *(long *)(v2 + v7 * 8))); v9 = v9 + 1) {
      v7 += 1;
    }
    while (((v9 < v8 && (v7 < v6)) && (*(long *)(v1 + v8 * 8 + -8) == *(long *)(v2 + v6 * 8 + -8)))) {
      v8 -= 1;
      v6 -= 1;
    }
    if (v9 == v8) {
      for (; v7 < v6; v7 = v7 + 1) {
        *(char *)(*(long *)(v7 * 8 + dat_2d598) + dat_2d5a8) = 1;
      }
      return '\0';
    }
    if (v7 == v6) {
      for (; v9 < v8; v9 = v9 + 1) {
        *(char *)(*(long *)(v9 * 8 + dat_2d468) + dat_2d478) = 1;
      }
      return '\0';
    }
    sub_5049(v9,v8,v7,v6,(unsigned char)v5,&v4,a5);
    if ((v20 + v4) - (v7 + v9) <= (v6 + v8) - (v4 + v20)) { // branch-flip
      v12 = v9;
      v13 = v4;
      v14 = v7;
      v15 = v20;
      v10 = v21;
      v16 = v4;
      v17 = v8;
      v18 = v20;
      v19 = v6;
      v11 = v22;
    }
    else {
      v12 = v4;
      v13 = v8;
      v14 = v20;
      v15 = v6;
      v10 = v22;
      v16 = v9;
      v17 = v4;
      v18 = v7;
      v19 = v20;
      v11 = v21;
    }
    v3 = sub_5d78(v12,v13,v14,v15,(unsigned char)v10,a5);
    if (v3) break;
    v9 = v16;
    v8 = v17;
    v7 = v18;
    v6 = v19;
    v5 = v11;
  }
  return v3;
}

// Function: sub_6122 @ 0x6122
void sub_6122(void *a0) // ternary x2
{
  long v1;
  void *v10 [3]; // stack - 0x48
  long v11; // r12
  int v12; // stack - 0xe4
  unsigned long v13; // stack - 0xe0
  long v14; // stack - 0xd8
  unsigned long v15; // stack - 0xd0
  unsigned long v16; // stack - 0xc8
  long v17; // stack - 0xc0
  long v18; // stack - 0xb8
  long v19; // stack - 0xb0
  void *v2;
  long v20; // stack - 0xa8
  long v21; // stack - 0x50
  unsigned long v3;
  void *v4;
  long *v5; // rax
  long v6;
  unsigned long v7;
  long v8; // rbx
  void *v9; // stack - 0x58
  
  v1 = *(long *)((long)a0 + 200);
  v6 = *(long *)((long)a0 + 0x1f8);
  v14 = sub_1c945((v6 + v1) * 0x10);
  for (v12 = 0; v12 <= 1; v12 = v12 + 1) {
    *(long *)((long)a0 + (long)v12 * 0x130 + 0x100) = v14;
    v14 += *(long *)((long)a0 + (long)v12 * 0x130 + 200) * 8;
    *(long *)((long)a0 + (long)v12 * 0x130 + 0x108) = v14;
    v14 += *(long *)((long)a0 + (long)v12 * 0x130 + 200) * 8;
  }
  v10[0] = (void *)sub_13030(*(long *)((long)a0 + 0x128) << 4);
  v10[1] = (void *)((long)v10[0] + *(long *)((long)a0 + 0x128) * 8);
  for (v13 = 0; (long)v13 < *(long *)((long)a0 + 200); v13 = v13 + 1) {
    v5 = (long *)(*(long *)(v13 * 8 + *(long *)((long)a0 + 0xf8)) * 8 + (long)v10[0]);
    *v5 = *v5 + 1;
  }
  for (v13 = 0; (long)v13 < *(long *)((long)a0 + 0x1f8); v13 = v13 + 1) {
    v5 = (long *)(*(long *)(v13 * 8 + *(long *)((long)a0 + 0x228)) * 8 + (long)v10[1]);
    *v5 = *v5 + 1;
  }
  v9 = (void *)sub_13030(*(long *)((long)a0 + 0x1f8) + *(long *)((long)a0 + 200));
  v21 = *(long *)((long)a0 + 200) + (long)v9;
  for (v12 = 0; v12 <= 1; v12 = v12 + 1) {
    v7 = *(unsigned long *)((long)a0 + (long)v12 * 0x130 + 200);
    v4 = (&v9)[v12];
    v2 = v10[1 - v12];
    v1 = *(long *)((long)a0 + (long)v12 * 0x130 + 0xf8);
    v15 = 5;
    v16 = v7 >> 6;
    while (v16 = v16 >> 2, v16) {
      v15 <<= 1;
    }
    for (v13 = 0; v13 < v7; v13 = v13 + 1) {
      if (*(long *)(v1 + v13 * 8)) {
        v3 = *(unsigned long *)((long)v2 + *(long *)(v1 + v13 * 8) * 8);
        if (v3) { // branch-flip
          if (v15 < v3)
            *(char *)((long)v4 + v13) = 2;
        }
        else {
          *(char *)((long)v4 + v13) = 1;
        }
      }
    }
  }
  for (v12 = 0; v12 <= 1; v12 = v12 + 1) {
    v1 = *(long *)((long)a0 + (long)v12 * 0x130 + 200);
    v4 = (&v9)[v12];
    for (v13 = 0; (long)v13 < v1; v13 = v13 + 1) {
      if (*(char *)(v13 + (long)v4) != '\x02') { // branch-flip
        if (*(char *)(v13 + (long)v4)) {
          v17 = 0;
          for (v7 = v13; ((long)v7 < v1 && (*(char *)(v7 + (long)v4))); v7 = v7 + 1) {
            if (*(char *)(v7 + (long)v4) == '\x02')
              v17 += 1;
          }
          while (((long)v13 < (long)v7 && (*(char *)(v7 + -1 + (long)v4) == '\x02'))) {
            v7 -= 1;
            *(char *)(v7 + (long)v4) = 0;
            v17 -= 1;
          }
          v6 = v7 - v13;
          if (v17 * 4 <= v6) { // branch-flip
            v18 = 1;
            v19 = v6 >> 2;
            while (v19 = v19 >> 2, 0 < v19) {
              v18 <<= 1;
            }
            v11 = 0;
            for (v8 = 0; v8 < v6; v8 = v8 + 1) {
              if (*(char *)(v13 + v8 + (long)v4) != '\x02') // branch-flip
                v11 = 0;
              else {
                v11 += 1;
                if (v11 != v18 + 1) { // branch-flip
                  if (v18 + 1 < v11)
                    *(char *)(v13 + v8 + (long)v4) = 0;
                }
                else {
                  v8 -= v11;
                }
              }
            }
            v11 = 0;
            for (v8 = 0; (v8 < v6 && ((v8 <= 7 || (*(char *)(v13 + v8 + (long)v4) != '\x01')))); v8 = v8 + 1) {
              if (*(char *)(v13 + v8 + (long)v4) != '\x02') { // branch-flip
                v11 = (*(char *)(v13 + v8 + (long)v4)) ? v11 + 1 : 0; // branch-flip
              }
              else {
                v11 = 0;
                *(char *)(v13 + v8 + (long)v4) = 0;
              }
              if (v11 == 3) break;
            }
            v13 += v6 + -1;
            v11 = 0;
            for (v8 = 0; (v8 < v6 && ((v8 <= 7 || (*(char *)((v13 - v8) + (long)v4) != '\x01')))); v8 = v8 + 1) {
              if (*(char *)((v13 - v8) + (long)v4) != '\x02') { // branch-flip
                v11 = (*(char *)((v13 - v8) + (long)v4)) ? v11 + 1 : 0; // branch-flip
              }
              else {
                v11 = 0;
                *(char *)((v13 - v8) + (long)v4) = 0;
              }
              if (v11 == 3) break;
            }
          }
          else {
            while ((long)v13 < (long)v7) {
              v7 -= 1;
              if (*(char *)(v7 + (long)v4) == '\x02')
                *(char *)(v7 + (long)v4) = 0;
            }
          }
        }
      }
      else {
        *(char *)(v13 + (long)v4) = 0;
      }
    }
  }
  for (v12 = 0; v12 <= 1; v12 = v12 + 1) {
    v4 = (&v9)[v12];
    v1 = *(long *)((long)a0 + (long)v12 * 0x130 + 200);
    v20 = 0;
    for (v13 = 0; (long)v13 < v1; v13 = v13 + 1) {
      if ((dat_2d348) || (!*(char *)((long)v4 + v13))) {
        *(unsigned long *)(*(long *)((long)a0 + (long)v12 * 0x130 + 0x100) + v20 * 8) = *(unsigned long *)(*(long *)((long)a0 + (long)v12 * 0x130 + 0xf8) + v13 * 8);
        v6 = v20 + 1;
        *(unsigned long *)(*(long *)((long)a0 + (long)v12 * 0x130 + 0x108) + v20 * 8) = v13;
        v20 = v6;
      }
      else {
        *(char *)(v13 + *(long *)((long)a0 + (long)v12 * 0x130 + 0x118)) = 1;
      }
    }
    *(long *)((long)a0 + (long)v12 * 0x130 + 0x110) = v20;
  }
  free(v9);
  free(v10[0]);
}

// Function: sub_6b31 @ 0x6b31
void sub_6b31(long a0)
{
  long v1;
  long v10; // stack - 0x48
  long v11; // stack - 0x40
  long v12; // stack - 0x38
  long v2;
  long v3;
  long v4;
  long v5;
  char *v6; // rax
  long v7;
  int v8; // stack - 0x54
  long v9; // stack - 0x50
  
  for (v8 = 0; v8 <= 1; v8 = v8 + 1) {
    v2 = *(long *)(a0 + (long)v8 * 0x130 + 0x118);
    v3 = *(long *)(a0 + (long)(1 - v8) * 0x130 + 0x118);
    v4 = *(long *)(a0 + (long)v8 * 0x130 + 0xf8);
    v9 = 0;
    v10 = 0;
    v5 = *(long *)(a0 + (long)v8 * 0x130 + 200);
    while( true ) {
      for (; (v9 < v5 && (!*(char *)(v2 + v9))); v9 = v9 + 1) {
        do {
          v7 = v10 + 1;
          v6 = (char *)(v3 + v10);
          v10 = v7;
        } while (*v6);
      }
      if (v9 == v5) break;
      v11 = v9;
      do {
        v9 += 1;
      } while (*(char *)(v2 + v9));
      for (; *(char *)(v3 + v10); v10 = v10 + 1) {
      }
      do {
        v7 = v9 - v11;
        while ((v11 && (*(long *)(v4 + v11 * 8 + -8) == *(long *)(v4 + v9 * 8 + -8)))) {
          v11 -= 1;
          *(char *)(v2 + v11) = 1;
          v9 -= 1;
          *(char *)(v2 + v9) = 0;
          for (; *(char *)(v2 + v11 + -1); v11 = v11 + -1) {
          }
          do {
            v10 -= 1;
          } while (*(char *)(v3 + v10));
        }
        if (*(char *)(v3 + v10 + -1)) // branch-flip
          v12 = v9;
        else {
          v12 = v5;
        }
        while ((v9 != v5 && (*(long *)(v4 + v11 * 8) == *(long *)(v4 + v9 * 8)))) {
          v1 = v11 + 1;
          *(char *)(v2 + v11) = 0;
          *(char *)(v2 + v9) = 1;
          do {
            v9 += 1;
          } while (*(char *)(v2 + v9));
          while (v10 = v10 + 1, v11 = v1, *(char *)(v3 + v10)) {
            v12 = v9;
          }
        }
      } while (v7 != v9 - v11);
      while (v12 < v9) {
        v11 -= 1;
        *(char *)(v2 + v11) = 1;
        v9 -= 1;
        *(char *)(v2 + v9) = 0;
        do {
          v10 -= 1;
        } while (*(char *)(v3 + v10));
      }
    }
  }
}

// Function: sub_6e92 @ 0x6e92
unsigned long * sub_6e92(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_1c945(0x30);
  v1[3] = a0;
  v1[4] = a1;
  v1[1] = a3;
  v1[2] = a2;
  *v1 = a4;
  return v1;
}

// Function: sub_6f01 @ 0x6f01
unsigned long sub_6f01(void *a0)
{
  long v1;
  long v2;
  long v3;
  long v4;
  long v5;
  long v6;
  unsigned long v7; // stack - 0x50
  long v8; // stack - 0x48
  long v9; // stack - 0x40
  
  v7 = 0;
  v1 = *(long *)((long)a0 + 0x118);
  v2 = *(long *)((long)a0 + 0x248);
  v3 = *(long *)((long)a0 + 200);
  v4 = *(long *)((long)a0 + 0x1f8);
  v8 = 0;
  for (v9 = 0; (v6 = v9, v5 = v8, v8 < v3 || (v9 < v4)); v9 = v9 + 1) {
    if (*(char *)(v2 + v9) || *(char *)(v1 + v8)) {
      for (; *(char *)(v1 + v8); v8 = v8 + 1) {
      }
      for (; *(char *)(v2 + v9); v9 = v9 + 1) {
      }
      v7 = sub_6e92(v5,v6,v8 - v5,v9 - v6,v7);
    }
    v8 += 1;
  }
  return v7;
}

// Function: sub_7037 @ 0x7037
unsigned long sub_7037(void *a0)
{
  long v1;
  long v2;
  long v3;
  long v4;
  unsigned long v5; // stack - 0x40
  long v6; // stack - 0x38
  long v7; // stack - 0x30
  
  v5 = 0;
  v1 = *(long *)((long)a0 + 0x118);
  v2 = *(long *)((long)a0 + 0x248);
  v6 = *(long *)((long)a0 + 200);
  for (v7 = *(long *)((long)a0 + 0x1f8); (v4 = v7, v3 = v6, 0 <= v6 || (0 <= v7)); v7 = v7 + -1) {
    if (*(char *)(v2 + v7 + -1) || *(char *)(v1 + v6 + -1)) {
      for (; *(char *)(v1 + v6 + -1); v6 = v6 + -1) {
      }
      for (; *(char *)(v2 + v7 + -1); v7 = v7 + -1) {
      }
      v5 = sub_6e92(v6,v7,v3 - v6,v4 - v7,v5);
    }
    v6 -= 1;
  }
  return v5;
}

// Function: sub_7167 @ 0x7167
void sub_7167(int a0,void *a1) // return-dupe
{
  char *v1; // rax
  long v2;
  long v3;
  
  if (!a0)
    return;
  if (dat_2d208) // branch-flip
    v3 = dat_2d208;
  else {
    v3 = *(long *)((long)a1 + 0x138);
  }
  if (dat_2d200) // branch-flip
    v2 = dat_2d200;
  else {
    v2 = *(long *)((long)a1 + 8);
  }
  if (dat_2d2a0) // branch-flip
    v1 = gettext("Files %s and %s differ\n");
  else {
    v1 = gettext("Binary files %s and %s differ\n");
  }
  sub_109a8(v1,v2,v3);
}

// Function: sub_7206 @ 0x7206
bool sub_7206(int *a0) // ternary x2
{
  int *v1;
  char *v10; // rax
  char *v11;
  unsigned long v12; // stack - 0x48
  char v13 [8];
  char v14 [8];
  char v15 [8];
  char v16 [8];
  int *v17;
  unsigned long *v18;
  bool v19;
  unsigned long *v2;
  int v20; // stack - 0xd0
  long *v21; // stack - 0xa8
  long *v22; // stack - 0xa0
  long v23; // stack - 0x98
  long v24; // stack - 0x90
  long *v25; // stack - 0x88
  unsigned long v26; // stack - 0x80
  void *v27; // stack - 0x78
  long *v28; // stack - 0x70
  unsigned long *v29; // stack - 0x68
  void *v3;
  long *v30; // stack - 0x60
  unsigned long v31; // stack - 0x58
  unsigned long v32; // stack - 0x50
  unsigned long v33; // stack - 0x40
  long v34; // stack - 0x38
  long v35; // stack - 0x30
  char v36; // stack - 0x28
  long v37; // stack - 0x20
  void *v4;
  char v5; // al
  int v6; // eax
  unsigned long v7;
  long v8;
  long v9;
  
  v5 = sub_fa13(a0,(unsigned char)dat_2d1ed);
  if (v5) { // branch-flip
    if (((((*(long *)&a0[0x10] != *(long *)&a0[0x5c]) && (1 <= *(long *)&a0[0x10])) && (1 <= *(long *)&a0[0x5c])) && ((*a0 <= -1 || ((a0[10] & 0xf000U) == 0x8000)))) && ((a0[0x4c] <= -1 || ((a0[0x56] & 0xf000U) == 0x8000))))
      v19 = 1;
    else if (*a0 != a0[0x4c]) { // branch-flip
      v31 = 0x7ffffffffffffffe;
      v7 = sub_1d2a2(*(unsigned long *)&a0[0x12],*(unsigned long *)&a0[0x5e],0x7ffffffffffffffe);
      v32 = sub_1d2a2(8,v7,v31);
      for (v20 = 0; v20 <= 1; v20 = v20 + 1) {
        v3 = *(void **)&a0[(long)v20 * 0x4c + 0x28];
        *(unsigned long *)&a0[(long)v20 * 0x4c + 0x28] = sub_1c991(v3,v32);
      }
      while( true ) {
        for (v20 = 0; v20 <= 1; v20 = v20 + 1) {
          if (0 <= a0[(long)v20 * 0x4c])
            sub_de51((struct_3 *)&a0[(long)v20 * 0x4c],v32 - *(long *)&a0[(long)v20 * 0x4c + 0x2c]);
        }
        if ((*(long *)&a0[0x2c] != *(long *)&a0[0x78]) || (v7 = *(unsigned long *)&a0[0x2c], v3 = *(void **)&a0[0x74], v4 = *(void **)&a0[0x28], memcmp(v4,v3,v7))) {
          v19 = 1;
          goto label_7572;
        }
        if (v32 != *(unsigned long *)&a0[0x2c]) break;
        a0[0x78] = 0;
        a0[0x79] = 0;
        *(unsigned long *)&a0[0x2c] = *(unsigned long *)&a0[0x78];
      }
      v19 = 0;
    }
    else {
      v19 = 0;
    }
label_7572:
    sub_7167(v19,a0);
  }
  else {
    v26 = *(long *)&a0[0x7e] + *(long *)&a0[0x32] + 4;
    v27 = (void *)sub_13030(v26);
    *(long *)&a0[0x46] = (long)v27 + 1;
    *(long *)&a0[0x92] = *(long *)&a0[0x32] + 3 + (long)v27;
    sub_6122(a0);
    v12 = *(unsigned long *)&a0[0x40];
    v33 = *(unsigned long *)&a0[0x8c];
    v23 = *(long *)&a0[0x90] + *(long *)&a0[0x44] + 3;
    v8 = sub_1c945(v23 * 0x10);
    v34 = (*(long *)&a0[0x90] + 1) * 8 + v8;
    v35 = (*(long *)&a0[0x90] + 1) * 8 + v23 * 8 + v8;
    v36 = dat_2d338;
    v24 = 1;
    for (; v23; v23 = v23 >> 2) {
      v24 <<= 1;
    }
    v37 = v24;
    if (v24 <= 0xfff)
      v37 = 0x1000;
    v8 = 0x26;
    v17 = a0;
    v18 = (unsigned long *)0x2d360;
    while (v8) {
      v2 = &v18[1];
      v1 = &v17[2];
      *v18 = *(unsigned long *)v17;
      v8 -= 1;
      v17 = v1;
      v18 = v2;
    }
    v8 = 0x26;
    v17 = &a0[0x4c];
    v18 = (unsigned long *)0x2d490;
    while (v8) {
      v2 = &v18[1];
      v1 = &v17[2];
      *v18 = *(unsigned long *)v17;
      v8 -= 1;
      v17 = v1;
      v18 = v2;
    }
    sub_5d78(0,*(long *)&a0[0x44],0,*(long *)&a0[0x90],(unsigned char)dat_2d348,&v12);
    free((void *)(*(long *)&a0[0x90] * -8 + -8 + v34));
    sub_6b31(a0);
    v22 = (dat_2d1c0 != 4) ? (long *)sub_7037(a0) : (long *)sub_6f01(a0); // branch-flip
    if ((dat_2d1ec) || (dat_2d280)) {
      v25 = v22;
      v19 = 0;
      while ((v25 && (!v19))) {
        v28 = v25;
        v29 = (unsigned long *)sub_12437(v25);
        v25 = (long *)*v29;
        *v29 = 0;
        v6 = sub_12af4(v28,v16,v15,v14,v13);
        if (v6)
          v19 = 1;
        *v29 = v25;
      }
    }
    else {
      v19 = v22 != NULL;
    }
    if (dat_2d2a0) // branch-flip
      sub_7167(v19,a0);
    else if ((v19) || (dat_2d1c8 != '\x01')) {
      if (dat_2d208) // branch-flip
        v8 = dat_2d208;
      else {
        v8 = *(long *)&a0[0x4e];
      }
      if (dat_2d200) // branch-flip
        v9 = dat_2d200;
      else {
        v9 = *(long *)&a0[2];
      }
      sub_118ac(v9,v8,*(long *)&a0[0x98] != 0);
      switch(dat_2d1c0) {
        default:
          abort(); // no-return
        case 1:
          sub_fcfd(v22);
          break;
        case 2:
          sub_7f2c(v22,0);
          break;
        case 3:
          sub_7f2c(v22,1);
          break;
        case 4:
          sub_cb85(v22);
          break;
        case 5:
          sub_cd6a(v22);
          break;
        case 6:
          sub_ceba(v22);
          break;
        case 7:
          sub_d076(v22);
          break;
        case 8:
          sub_ff70(v22);
        
      }
      sub_11e1b();
    }
    free(*(void **)&a0[0x40]);
    free(v27);
    for (v20 = 0; v20 <= 1; v20 = v20 + 1) {
      free(*(void **)&a0[(long)v20 * 0x4c + 0x3e]);
      free((void *)(*(long *)&a0[(long)v20 * 0x4c + 0x30] * 8 + *(long *)&a0[(long)v20 * 0x4c + 0x2e]));
    }
    v21 = v22;
    while (v21) {
      v30 = (long *)*v21;
      free(v21);
      v21 = v30;
    }
    if ((dat_2d1c0 == 4) || (dat_2d1c0 == 5)) {
      for (v20 = 0; v20 <= 1; v20 = v20 + 1) {
        if ((char)a0[(long)v20 * 0x4c + 0x48]) {
          v10 = gettext("No newline at end of file");
          v11 = (*(long *)((long)v20 * 8 + 0x2d200)) ? *(char **)((long)v20 * 8 + 0x2d200) : *(char **)&a0[(long)v20 * 0x4c + 2]; // branch-flip
          error(0,0,"%s: %s\n",v11,v10); // no-return
        }
      }
    }
  }
  if (*(long *)&a0[0x28] != *(long *)&a0[0x74])
    free(*(void **)&a0[0x28]);
  free(*(void **)&a0[0x74]);
  return v19;
}

// Function: sub_7d13 @ 0x7d13
void sub_7d13(char *a0,struct_18 *a1,char *a2,char *a3)
{
  int v1; // eax
  tm *v2; // rax
  char v3 [56];
  
  sub_128c2(0);
  if (a3) // branch-flip
    fprintf(dat_2d5c0,"%s %s",a0,a3);
  else {
    v2 = localtime(&a1->field_0x68);
    v1 = sub_1bb4f((struct_17 *)&a1->field_0x0[0x10]);
    if ((!v2) || (!sub_1710b(v3,0x2b,dat_2d350,v2,0,v1)))
      sprintf(v3,"%ld.%.9d",a1->field_0x68,v1);
    fprintf(dat_2d5c0,"%s %s\t%s",a0,a2,v3);
  }
  sub_128c2(3);
  putc_unlocked(10,dat_2d5c0);
}

// Function: sub_7e5c @ 0x7e5c
void sub_7e5c(long a0,struct_19 *a1,bool a2) // return-dupe
{
  if (a2) {
    sub_7d13("---",a0,a1->field_0x0,dat_2d200);
    sub_7d13("+++",(struct_18 *)(a0 + 0x130),a1->field_0x8,dat_2d208);
    return;
  }
  sub_7d13("***",a0,a1->field_0x0,dat_2d200);
  sub_7d13("---",(struct_18 *)(a0 + 0x130),a1->field_0x8,dat_2d208);
}

// Function: sub_7f2c @ 0x7f2c
void sub_7f2c(unsigned long *a0,bool a1) // return-dupe
{
  unsigned long *v1; // stack - 0x10
  
  if ((dat_2d1ec) || (dat_2d280))
    sub_8c9d(a0);
  else {
    for (v1 = a0; v1; v1 = (unsigned long *)*v1) {
      *(char *)&v1[5] = 0;
    }
  }
  dat_2d1b0 = -dat_2d448;
  dat_2d1b8 = 0x7fffffffffffffff;
  if (a1) {
    sub_1245b(a0,sub_8bbf,sub_8710);
    return;
  }
  sub_1245b(a0,sub_8bbf,sub_818f);
}

// Function: sub_7ff1 @ 0x7ff1
void sub_7ff1(struct_20 *a0,long a1,long a2) // return-dupe
{
  long v1; // stack - 0x18
  long v2; // stack - 0x20
  
  sub_129d0(a0,a1,a2,&v2,&v1);
  if (v1 <= v2) {
    fprintf(dat_2d5c0,"%ld",v1);
    return;
  }
  fprintf(dat_2d5c0,"%ld,%ld",v2,v1);
}

// Function: sub_80a2 @ 0x80a2
void sub_80a2(FILE *a0,long a1)
{
  char v1;
  int v2; // stack - 0x10
  int v3; // stack - 0xc
  
  putc_unlocked(0x20,a0);
  for (v2 = 0; (v1 = *(char *)(a1 + v2), sub_133c9((unsigned char)v1) && (*(char *)(a1 + v2) != '\n')); v2 = v2 + 1) {
  }
  for (v3 = v2; (v3 <= v2 + 0x27 && (*(char *)(a1 + v3) != '\n')); v3 = v3 + 1) {
  }
  while ((v2 < v3 && (v1 = *(char *)(a1 + (long)v3 + -1), sub_133c9((unsigned char)v1)))) {
    v3 -= 1;
  }
  fwrite_unlocked((void *)(v2 + a1),1,(long)(v3 - v2),a0);
}

// Function: sub_818f @ 0x818f
void sub_818f(unsigned long *a0) // return-dupe x2, ternary x4
{
  unsigned int v1; // eax
  unsigned long *v10; // stack - 0x28
  unsigned long *v11; // stack - 0x20
  FILE *v12; // stack - 0x18
  long v2; // rax
  long v3; // stack - 0x48
  long v4; // stack - 0x50
  long v5; // stack - 0x58
  long v6; // stack - 0x60
  long v7; // stack - 0x40
  char *v8; // stack - 0x38
  long v9; // stack - 0x30
  
  v1 = sub_12af4(a0,&v6,&v5,&v4,&v3);
  if (!v1)
    return;
  v7 = -dat_2d448;
  v2 = v6 - dat_2d1d0;
  v6 = v7;
  if (v7 <= v2)
    v6 = v2;
  v2 = v4 - dat_2d1d0;
  v4 = v7;
  if (v7 <= v2)
    v4 = v2;
  v5 = (dat_2d430 - dat_2d1d0 <= v5) ? dat_2d430 + -1 : dat_2d1d0 + v5; // branch-flip
  v3 = (dat_2d560 - dat_2d1d0 <= v3) ? dat_2d560 + -1 : dat_2d1d0 + v3; // branch-flip
  v9 = 0;
  if (dat_2d240)
    v9 = sub_8d30(dat_2d418,v6);
  sub_11b50();
  v12 = dat_2d5c0;
  fputs_unlocked("***************",dat_2d5c0);
  if (v9)
    sub_80a2(v12,v9);
  putc_unlocked(10,v12);
  sub_128c2(4);
  fputs_unlocked("*** ",v12);
  sub_7ff1((struct_20 *)0x2d360,v6,v5);
  fputs_unlocked(" ****",v12);
  sub_128c2(3);
  putc_unlocked(10,v12);
  if (v1 & 1) {
    v10 = a0;
    for (v7 = v6; v7 <= v5; v7 = v7 + 1) {
      sub_128c2(2);
      for (; (v10 && ((long)(v10[2] + v10[3]) <= v7)); v10 = (unsigned long *)*v10) {
      }
      v8 = " ";
      if ((v10) && ((long)v10[3] <= v7)) {
        v8 = (1 <= (long)v10[1]) ? "!" : "-"; // branch-flip
      }
      sub_124fe(v8,(struct_21 *)(dat_2d418 + v7 * 8),1);
      sub_128c2(3);
      if (*(char *)(*(long *)((v7 + 1) * 8 + dat_2d418) + -1) == '\n')
        putc_unlocked(10,v12);
    }
  }
  sub_128c2(4);
  fputs_unlocked("--- ",v12);
  sub_7ff1((struct_20 *)0x2d490,v4,v3);
  fputs_unlocked(" ----",v12);
  sub_128c2(3);
  putc_unlocked(10,v12);
  if (v1 & 2) {
    v11 = a0;
    for (v7 = v4; v7 <= v3; v7 = v7 + 1) {
      sub_128c2(1);
      for (; (v11 && ((long)(v11[1] + v11[4]) <= v7)); v11 = (unsigned long *)*v11) {
      }
      v8 = " ";
      if ((v11) && ((long)v11[4] <= v7)) {
        v8 = (1 <= (long)v11[2]) ? "!" : "+"; // branch-flip
      }
      sub_124fe(v8,(struct_21 *)(dat_2d548 + v7 * 8),1);
      sub_128c2(3);
      if (*(char *)(*(long *)((v7 + 1) * 8 + dat_2d548) + -1) == '\n')
        putc_unlocked(10,v12);
    }
    return;
  }
}

// Function: sub_863e @ 0x863e
void sub_863e(struct_20 *a0,long a1,long a2) // return-dupe, ternary
{
  char *v1; // rax
  long v2; // stack - 0x18
  long v3; // stack - 0x20
  
  sub_129d0(a0,a1,a2,&v3,&v2);
  if (v2 <= v3) {
    v1 = (v3 <= v2) ? "%ld" : "%ld,0"; // branch-flip
    fprintf(dat_2d5c0,v1,v2);
    return;
  }
  fprintf(dat_2d5c0,"%ld,%ld",v3,(v2 - v3) + 1);
}

// Function: sub_8710 @ 0x8710
void sub_8710(unsigned long *a0) // return-dupe, ternary x3
{
  long v1;
  long v10; // stack - 0x48
  unsigned long *v11; // stack - 0x40
  long v12; // stack - 0x38
  FILE *v13; // stack - 0x30
  struct_21 *v14; // stack - 0x28
  struct_21 *v15; // stack - 0x20
  struct_21 *v16; // stack - 0x18
  long v2;
  int v3; // edx
  long v4; // stack - 0x60
  long v5; // stack - 0x68
  long v6; // stack - 0x70
  long v7; // stack - 0x78
  long v8; // stack - 0x58
  long v9; // stack - 0x50
  
  if (!sub_12af4(a0,&v7,&v6,&v5,&v4))
    return;
  v8 = -dat_2d448;
  v2 = v7 - dat_2d1d0;
  v7 = v8;
  if (v8 <= v2)
    v7 = v2;
  v2 = v5 - dat_2d1d0;
  v5 = v8;
  if (v8 <= v2)
    v5 = v2;
  v6 = (dat_2d430 - dat_2d1d0 <= v6) ? dat_2d430 + -1 : dat_2d1d0 + v6; // branch-flip
  v4 = (dat_2d560 - dat_2d1d0 <= v4) ? dat_2d560 + -1 : dat_2d1d0 + v4; // branch-flip
  v12 = 0;
  if (dat_2d240)
    v12 = sub_8d30(dat_2d418,v7);
  sub_11b50();
  v13 = dat_2d5c0;
  sub_128c2(4);
  fputs_unlocked("@@ -",v13);
  sub_863e((struct_20 *)0x2d360,v7,v6);
  fputs_unlocked(" +",v13);
  sub_863e((struct_20 *)0x2d490,v5,v4);
  fputs_unlocked(" @@",v13);
  sub_128c2(3);
  if (v12)
    sub_80a2(v13,v12);
  putc_unlocked(10,v13);
  v8 = v7;
  v9 = v5;
  v11 = a0;
  while ((v8 <= v6 || (v9 <= v4))) {
    if ((v11) && ((long)v11[3] <= v8)) {
      v10 = v11[2];
      while (v2 = v10 + -1, v10) {
        v1 = v8 + 1;
        v15 = (struct_21 *)(v8 * 8 + dat_2d418);
        v8 = v1;
        v10 = v2;
        sub_128c2(2);
        putc_unlocked(0x2d,v13);
        if ((dat_2d2b0) && ((dat_2d2b1 != '\x01' || (*v15->field_0x0 != 10))))
          putc_unlocked(9,v13);
        sub_124fe(NULL,v15,1);
        sub_128c2(3);
        if (*(char *)(v15->field_0x8 + -1) == '\n')
          putc_unlocked(10,v13);
      }
      v10 = v11[1];
      while (v2 = v10 + -1, v10) {
        v1 = v9 + 1;
        v14 = (struct_21 *)(v9 * 8 + dat_2d548);
        v9 = v1;
        v10 = v2;
        sub_128c2(1);
        putc_unlocked(0x2b,v13);
        if ((dat_2d2b0) && ((dat_2d2b1 != '\x01' || (*v14->field_0x0 != 10))))
          putc_unlocked(9,v13);
        sub_124fe(NULL,v14,1);
        sub_128c2(3);
        if (*(char *)(v14->field_0x8 + -1) == '\n')
          putc_unlocked(10,v13);
      }
      v11 = (unsigned long *)*v11;
      v10 = 0xffffffffffffffff;
    }
    else {
      v2 = v8 + 1;
      v16 = (struct_21 *)(v8 * 8 + dat_2d418);
      v8 = v2;
      if ((dat_2d2b1 != '\x01') || (*v16->field_0x0 != 10)) {
        v3 = (dat_2d2b0) ? 9 : 0x20; // branch-flip
        putc_unlocked(v3,v13);
      }
      sub_124cf(NULL,v16);
      v9 += 1;
    }
  }
}

// Function: sub_8bbf @ 0x8bbf
long * sub_8bbf(long *a0)
{
  long *v1;
  long v2;
  long *v3; // stack - 0x40
  
  v1 = a0;
  while( true ) {
    v3 = v1;
    v1 = (long *)*v3;
    if ((v1) && (*(char *)&v1[5]))
      v2 = dat_2d1d0;
    else {
      v2 = dat_2d1d0 * 2 + 1;
    }
    if ((v1) && (v1[3] - (v3[2] + v3[3]) != v1[4] - (v3[1] + v3[4]))) break;
    if ((!v1) || (v2 <= v1[3] - (v3[2] + v3[3])))
      return v3;
  }
  abort(); // no-return
}

// Function: sub_8c9d @ 0x8c9d
void sub_8c9d(unsigned long *a0)
{
  char v1 [8];
  char v2 [8];
  char v3 [8];
  char v4 [8];
  unsigned long *v5; // stack - 0x40
  unsigned long *v6; // stack - 0x18
  
  v5 = a0;
  while (v5) {
    v6 = (unsigned long *)*v5;
    *v5 = 0;
    *(bool *)&v5[5] = sub_12af4(v5,v4,v3,v2,v1) == 0;
    *v5 = v6;
    v5 = v6;
  }
}

// Function: sub_8d30 @ 0x8d30
char * sub_8d30(long a0,long a1)
{
  long v1;
  char *v2;
  long v3;
  unsigned long v4; // rax
  long v5; // stack - 0x28
  
  v3 = dat_2d1b0;
  dat_2d1b0 = a1;
  v5 = a1;
  do {
    v1 = v5 + -1;
    if (v1 < v3) {
      if (dat_2d1b8 != 0x7fffffffffffffff)
        return *(char **)(a0 + dat_2d1b8 * 8);
      return NULL;
    }
    v2 = *(char **)(a0 + v1 * 8);
    v4 = (*(long *)(a0 + v5 * 8) - (long)v2) - 1;
    if (0x80000000 <= v4)
      v4 = 0x7fffffff;
    v5 = v1;
  } while (re_search((re_pattern_buffer *)0x2d220,v2,(int)v4,0,(int)v4,NULL) < 0);
  dat_2d1b8 = v1;
  return v2;
}

// Function: sub_8e44 @ 0x8e44
char * sub_8e44(char **a0,int a1)
{
  char *v1;
  char *v2; // rax
  int v3; // stack - 0x24
  unsigned long v4; // stack - 0x20
  char *v5; // stack - 0x18
  
  v4 = 1;
  for (v3 = 0; v3 < a1; v3 = v3 + 1) {
    v1 = a0[v3];
    v4 += sub_1a4f6(v1) + 1;
  }
  v2 = (char *)sub_1c945(v4);
  v5 = v2;
  for (v3 = 0; v3 < a1; v3 = v3 + 1) {
    *v5 = 0x20;
    v1 = a0[v3];
    v5 = (char *)sub_1a550(&v5[1],v1);
  }
  *v5 = 0;
  return v2;
}

// Function: sub_8f0e @ 0x8f0e
unsigned long sub_8f0e(void) // return-dupe
{
  if (dat_2d1ef)
    return 0x10000010;
  return 0x10000000;
}

// Function: main @ 0x8f2f
int main(int argc,char **argv,char **envp) // ternary x2, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in main
{
  char *v1;
  unsigned long v10; // rax
  unsigned long v11; // rax
  unsigned long v12; // rax
  unsigned char v13; // dl
  char *v14; // stack - 0x88
  long v15; // stack - 0x98
  long v16; // stack - 0x90
  int v17; // stack - 0xb0
  int v18; // stack - 0xac
  unsigned int v19; // stack - 0xa8
  char *v2;
  int v20; // stack - 0xa4
  unsigned long v21; // stack - 0x80
  unsigned long v22; // stack - 0x78
  unsigned long v23; // stack - 0x70
  char *v24; // stack - 0x68
  char *v25; // stack - 0x60
  unsigned long v26; // stack - 0x58
  unsigned long v27; // stack - 0x50
  unsigned long v28; // stack - 0x48
  unsigned long v29; // stack - 0x40
  bool v3;
  unsigned long v30; // stack - 0x38
  bool v4;
  unsigned char v5; // al
  unsigned int v6; // eax
  int v7; // eax
  unsigned long v8;
  unsigned long v9; // rax
  
  v17 = 0;
  v20 = -1;
  v21 = 0xffffffffffffffff;
  v4 = 0;
  v22 = 0;
  v3 = 0;
  v15 = 0;
  v16 = 0;
  dat_2d0c8 = 2;
  sub_19a53(*argv);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_13721(NULL);
  dat_2d600 = 0x2d220;
  dat_2d640 = 0x2d260;
  re_set_syntax(0x50a46);
  dat_2d340 = sub_139c7();
  dat_2d5c8 = 0;
  sub_1cfb9();
  do {
    v18 = getopt_long(argc,argv,"0123456789abBcC:dD:eEfF:hHiI:lL:nNpPqrsS:tTuU:vwW:x:X:yZ",(option *)0x2bf40,NULL);
    if (v18 == -1) {
      if (dat_2d1c4 == 1) {
        v25 = getenv("TERM");
        if ((v25) && (!strcmp(v25,"dumb")))
          dat_2d1c4 = 0;
      }
      if (!dat_2d1c0) {
        if (v3) {
          sub_a63e(2);
          if ((long)v21 <= -1)
            dat_2d1d0 = 3;
        }
        else {
          sub_a63e(1);
        }
      }
      if ((dat_2d1c0 != 2) || (sub_14cdf(2)))
        dat_2d350 = "%Y-%m-%d %H:%M:%S.%N %z";
      else {
        dat_2d350 = "%a %b %e %T %Y";
      }
      if (((0 <= (long)v21) && ((dat_2d1c0 == 2 || (dat_2d1c0 == 3)))) && (((long)dat_2d1d0 < (long)v21 || (((long)v21 < (long)dat_2d1d0 && (!v4))))))
        dat_2d1d0 = v21;
      if (!dat_2d2a8)
        dat_2d2a8 = 8;
      if (!v22)
        v22 = 0x82;
      if (dat_2d2a1) // branch-flip
        v26 = 1;
      else {
        v26 = dat_2d2a8;
      }
      v27 = v22;
      v28 = v26 + 3;
      v29 = (unsigned long)((unsigned int)v22 & (unsigned int)v28 & 1) + (v22 >> 1) + (v28 >> 1);
      v30 = v29 - v29 % v26;
      if ((4 <= v30) && (v30 < v22)) {
        dat_2d320 = v22 - v30;
        if (v30 - 3 <= v22 - v30)
          dat_2d320 = v30 - 3;
      }
      else {
        dat_2d320 = 0;
      }
      if (dat_2d320) // branch-flip
        dat_2d328 = v30;
      else {
        dat_2d328 = v22;
      }
      if ((long)dat_2d1e0 < (long)dat_2d1d0)
        dat_2d1e0 = dat_2d1d0;
      sub_a296((struct_0 *)0x2d5e0);
      sub_a296((struct_0 *)0x2d620);
      if (dat_2d1c0 == 7) {
        for (v19 = 0; v19 <= 2; v19 = v19 + 1) {
          if (!*(long *)((long)(int)v19 * 8 + 0x2d300))
            *(char **)((long)(int)v19 * 8 + 0x2d300) = "%l\n";
        }
        if (!dat_2d2e8) {
          if (dat_2d2f8) // branch-flip
            dat_2d2e8 = dat_2d2f8;
          else {
            dat_2d2e8 = "%<";
          }
        }
        if (!dat_2d2f0) {
          if (dat_2d2f8) // branch-flip
            dat_2d2f0 = dat_2d2f8;
          else {
            dat_2d2f0 = "%>";
          }
        }
        if (!dat_2d2e0)
          dat_2d2e0 = "%=";
        if (!dat_2d2f8)
          dat_2d2f8 = (char *)sub_12fa9(dat_2d2e8,dat_2d2f0,"");
      }
      if (dat_2d1c0 != 7) // branch-flip
        dat_2d1c8 = dat_2d31a || dat_2d1c0 != 8;
      else {
        dat_2d1c8 = !((*dat_2d2e0) && ((strcmp(dat_2d2e0,"%=") || (*dat_2d300))));
      }
      v13 = dat_2d1ec | dat_2d1ee;
      v5 = ((dat_2d620) || (dat_2d1e8));
      dat_2d1ed = (~(v5 | dat_2d2b2 | v13) & dat_2d2a0 & 1) != 0;
      dat_2d330 = sub_8e44(&argv[1],optind + -1);
      if (v15) { // branch-flip
        if (v16)
          sub_10966("--from-file and --to-file both specified"); // no-return
        for (; optind < argc; optind = optind + 1) {
          v7 = sub_a746(NULL,v15,argv[optind]);
          if (v17 < v7)
            v17 = v7;
        }
      }
      else if (v16) { // branch-flip
        for (; optind < argc; optind = optind + 1) {
          v7 = sub_a746(NULL,argv[optind],v16);
          if (v17 < v7)
            v17 = v7;
        }
      }
      else {
        if (argc - optind != 2) {
          if (2 > argc - optind)
            sub_a338("missing operand after \'%s\'",argv[(long)argc + -1]); // no-return
          sub_a338("extra operand \'%s\'",argv[(long)optind + 2]); // no-return
        }
        v1 = argv[(long)optind + 1];
        v2 = argv[optind];
        v17 = sub_a746(NULL,v2,v1);
      }
      sub_10bca();
      sub_a3ae();
      exit(v17); // no-return
    }
    switch(v18) {
      case 0:
      case 0x68:
      case 0x80:
      case 0x85:
        break;
      default:
        sub_a338(NULL,0); // no-return
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
        if (10 <= (unsigned int)(v20 - 0x30U)) // branch-flip
          v21 = (unsigned long)(v18 + -0x30);
        else {
          v21 = (0x666666666666666 <= (long)(v21 - (v18 <= 0x33))) ? 0x3fffffffffffffff : (long)(v18 + -0x30) + v21 * 10; // branch-flip
        }
        break;
      case 0x42:
        dat_2d1ec = 1;
        break;
      case 0x43:
      case 0x55:
        if (optarg) { // branch-flip
          v23 = strtoimax(optarg,&v14,10);
          if ((*v14) || ((long)v23 <= -1))
            sub_a338("invalid context length \'%s\'",optarg); // no-return
          if (0x4000000000000000 <= (long)v23)
            v23 = 0x3fffffffffffffff;
        }
        else {
          v23 = 3;
        }
        v6 = (v18 != 0x55) ? 2 : 3; // branch-flip
        sub_a63e(v6);
        if ((long)dat_2d1d0 < (long)v23)
          dat_2d1d0 = v23;
        v4 = 1;
        break;
      case 0x44:
        sub_a63e(7);
        v24 = (char *)sub_1c945(strlen(optarg) * 7 + 0x61);
        sprintf(v24,"%%=%c#ifndef %s\n%%<#endif /* ! %s */\n%c#ifdef %s\n%%>#endif /* %s */\n%c#ifndef %s\n%%<#else /* %s */\n%%>#endif /* %s */\n",0,optarg,optarg,0,optarg,optarg,0,optarg,optarg,optarg);
        for (v19 = 0; v19 <= 3; v19 = v19 + 1) {
          sub_a5af((long)(int)v19 * 8 + 0x2d2e0,v24,"-D");
          v8 = strlen(v24);
          v24 = &v24[v8 + 1];
        }
        break;
      case 0x45:
        if (dat_2d1e8 <= 3)
          dat_2d1e8 |= 1;
        break;
      case 0x46:
        sub_a107(0x2d5e0,optarg);
        break;
      case 0x48:
        dat_2d338 = 1;
        break;
      case 0x49:
        sub_a107(0x2d620,optarg);
        break;
      case 0x4c:
        if (dat_2d200) { // branch-flip
          if (dat_2d208)
            sub_10966("too many file label options"); // no-return
          dat_2d208 = optarg;
        }
        else {
          dat_2d200 = optarg;
        }
        break;
      case 0x4e:
        dat_2d648 = 1;
        break;
      case 0x50:
        dat_2d649 = 1;
        break;
      case 0x53:
        sub_a5af(0x2d2b8,optarg,"-S");
        break;
      case 0x54:
        dat_2d2b0 = 1;
        break;
      case 0x57:
        v23 = strtoimax(optarg,&v14,10);
        if (((long)v23 <= 0) || (*v14))
          sub_a338("invalid width \'%s\'",optarg); // no-return
        if (v22 != v23) {
          if (v22)
            sub_10966("conflicting width options"); // no-return
          v22 = v23;
        }
        break;
      case 0x58:
        v7 = sub_148b2(sub_142e9,dat_2d340,optarg,sub_8f0e(),10);
        if (v7)
          sub_10925(optarg); // no-return
        break;
      case 0x5a:
        if (dat_2d1e8 <= 3)
          dat_2d1e8 |= 2;
        break;
      case 0x61:
        dat_2d1d8 = 1;
        break;
      case 0x62:
        if (dat_2d1e8 <= 3)
          dat_2d1e8 = 4;
        break;
      case 99:
        sub_a63e(2);
        if ((long)dat_2d1d0 <= 2)
          dat_2d1d0 = 3;
        break;
      case 100:
        dat_2d348 = 1;
        break;
      case 0x65:
        sub_a63e(4);
        break;
      case 0x66:
        sub_a63e(5);
        break;
      case 0x69:
        dat_2d1ee = 1;
        break;
      case 0x6c:
        if (!s_25400[0])
          sub_a338("pagination not supported on this host",0); // no-return
        dat_2d2c0 = 1;
        signal(0x11,0);
        break;
      case 0x6e:
        sub_a63e(6);
        break;
      case 0x70:
        v3 = 1;
        sub_a107(0x2d5e0,"^[[:alpha:]$_]");
        break;
      case 0x71:
        dat_2d2a0 = 1;
        break;
      case 0x72:
        dat_2d5c9 = 1;
        break;
      case 0x73:
        dat_2d64a = 1;
        break;
      case 0x74:
        dat_2d2a1 = '\x01';
        break;
      case 0x75:
        sub_a63e(3);
        if ((long)dat_2d1d0 <= 2)
          dat_2d1d0 = 3;
        break;
      case 0x76:
        v9 = sub_1a014("Len Tower");
        v10 = sub_1a014("Richard Stallman");
        v11 = sub_1a014("David Hayes");
        v12 = sub_1a014("Mike Haertel");
        sub_1c71c(stdout,"diff","GNU diffutils",dat_2d0c0,sub_1a014("Paul Eggert"),v12,v11,v10,v9,0);
        sub_a3ae();
        return 0;
      case 0x77:
        dat_2d1e8 = 5;
        break;
      case 0x78:
        sub_142e9(dat_2d340,optarg,sub_8f0e());
        break;
      case 0x79:
        sub_a63e(8);
        break;
      case 0x81:
        sub_a5af(&v15,optarg,"--from-file");
        break;
      case 0x82:
        sub_a405();
        sub_a3ae();
        return 0;
      case 0x83:
        v23 = strtoimax(optarg,&v14,10);
        if ((*v14) || ((long)v23 <= -1))
          sub_a338("invalid horizon length \'%s\'",optarg); // no-return
        v8 = v23;
        if ((long)v23 <= (long)dat_2d1e0)
          v8 = dat_2d1e0;
        dat_2d1e0 = v8;
        break;
      case 0x84:
        dat_2d1ef = 1;
        break;
      case 0x86:
        dat_2d319 = 1;
        break;
      case 0x87:
        sub_a63e(7);
        for (v19 = 0; v19 <= 2; v19 = v19 + 1) {
          sub_a5af((long)(int)v19 * 8 + 0x2d300,optarg,"--line-format");
        }
        break;
      case 0x88:
        dat_2d1f0 = 1;
        break;
      case 0x89:
        dat_2d1ef = 0;
        break;
      case 0x8a:
        sub_a63e(1);
        break;
      case 0x8b:
        sub_a63e(8);
        dat_2d318 = 1;
        break;
      case 0x8c:
        dat_2d2b2 = 1;
        break;
      case 0x8d:
        dat_2d2b1 = 1;
        break;
      case 0x8e:
        dat_2d31a = '\x01';
        break;
      case 0x8f:
        v23 = strtoimax(optarg,&v14,10);
        if ((((long)v23 <= 0) || (0xfffffffffffffffd <= v23)) || (*v14))
          sub_a338("invalid tabsize \'%s\'",optarg); // no-return
        if (dat_2d2a8 != v23) {
          if (dat_2d2a8)
            sub_10966("conflicting tabsize options"); // no-return
          dat_2d2a8 = v23;
        }
        break;
      case 0x90:
        sub_a5af(&v16,optarg,"--to-file");
        break;
      case 0x91:
      case 0x92:
      case 0x93:
        sub_a63e(7);
        v18 -= 0x91;
        sub_a5af((long)v18 * 8 + 0x2d300,optarg,&"--unchanged-line-format"[(long)v18 * 0x18]);
        break;
      case 0x94:
      case 0x95:
      case 0x96:
      case 0x97:
        sub_a63e(7);
        v18 -= 0x94;
        sub_a5af((long)v18 * 8 + 0x2d2e0,optarg,&"--unchanged-group-format"[(long)v18 * 0x19]);
        break;
      case 0x98:
        sub_a682(optarg);
        break;
      case 0x99:
        sub_1140f(optarg);
        break;
      case 0x9a:
        dat_2d5c8 = 1;
      
    }
    v20 = v18;
  } while( true );
}

// Function: sub_a107 @ 0xa107
void sub_a107(struct_22 *a0,char *a1)
{
  long v1;
  unsigned char v2;
  unsigned long v3; // rax
  char *v4; // rax
  void *v5; // stack - 0x38
  long v6; // stack - 0x30
  unsigned long v7; // stack - 0x28
  
  v3 = strlen(a1);
  v4 = re_compile_pattern(a1,v3,a0->field_0x20);
  if (v4)
    error(2,0,"%s: %s",a1,v4); // no-return
  v5 = a0->field_0x0;
  v6 = a0->field_0x8;
  a0->field_0x18 = v5 != NULL;
  v2 = a0->field_0x18;
  a0->field_0x8 = (int)((unsigned int)v2 * 2) + v6 + v3;
  v7 = a0->field_0x10;
  if (v7 <= (unsigned long)a0->field_0x8) {
    if (!v7)
      v7 = 1;
    do {
      v7 <<= 1;
    } while (v7 <= (unsigned long)a0->field_0x8);
    a0->field_0x10 = v7;
    v5 = (void *)sub_1c991(v5,v7);
    a0->field_0x0 = v5;
  }
  if (v2) {
    v1 = v6 + 1;
    *(char *)(v6 + (long)v5) = 0x5c;
    v6 += 2;
    *(char *)(v1 + (long)v5) = 0x7c;
  }
  memcpy((void *)((long)v5 + v6),a1,v3 + 1);
}

// Function: sub_a296 @ 0xa296
void sub_a296(struct_0 *a0)
{
  long v1;
  char *v2; // rax
  
  if (a0->field_0x0) {
    v1 = a0->field_0x20;
    *(unsigned long *)(v1 + 0x20) = sub_1c945(0x100);
    if (a0->field_0x18) {
      v2 = re_compile_pattern((char *)a0->field_0x0,a0->field_0x8,(re_pattern_buffer *)a0->field_0x20);
      if (v2)
        error(2,0,"%s: %s",(char *)a0->field_0x0,v2); // no-return
    }
  }
}

// Function: sub_a338 @ 0xa338
void sub_a338(char *a0,unsigned long a1)
{
  char *v1;
  
  v1 = dat_3d8a8;
  if (a0)
    error(0,0,gettext(a0),a1); // no-return
  error(2,0,gettext("Try \'%s --help\' for more information."),v1); // no-return
}

// Function: sub_a3ae @ 0xa3ae
void sub_a3ae(void)
{
  if (ferror_unlocked(stdout))
    sub_10966("write failed"); // no-return
  if (fclose(stdout))
    sub_10925(gettext("standard output")); // no-return
}

// Function: sub_a405 @ 0xa405
void sub_a405(void) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_a405
{
  FILE *v1;
  int v2; // eax
  char *v3;
  long *v4; // stack - 0x30
  char *v5; // stack - 0x28
  
  v3 = dat_3d8a8;
  printf(gettext("Usage: %s [OPTION]... FILES\n"),v3);
  printf("%s\n\n",gettext("Compare FILES line by line."));
  v1 = stdout;
  fputs_unlocked(gettext("Mandatory arguments to long options are mandatory for short options too.\n"),v1);
  v4 = (long *)0x2c6e0;
  do {
    if (!*v4) {
      sub_1c800();
      return;
    }
    if (*(char *)*v4) { // branch-flip
      v3 = (char *)*v4;
      v5 = gettext(v3);
      while( true ) {
        v3 = strchr(v5,10);
        if (!v3) break;
        v2 = ((int)v3 + 1) - (int)v5;
        if (0x1000 <= v2)
          __assert_fail("msglen < 4096","diff.c",0x3f8,"usage"); // no-return
        printf("  %.*s",v2,v5);
        v5 = &v3[1];
      }
      v2 = ((*v5 != ' ') && (*v5 != '-'));
      printf(&"  %s\n"[v2 * 2],v5);
    }
    else {
      putchar_unlocked(10);
    }
    v4 = &v4[1];
  } while( true );
}

// Function: sub_a5af @ 0xa5af
void sub_a5af(long *a0,char *a1,char *a2)
{
  char *v1;
  
  if (*a0) {
    v1 = (char *)*a0;
    if (strcmp(v1,a1))
      error(0,0,gettext("conflicting %s option value \'%s\'"),a2,a1); // no-return
  }
  *a0 = (long)a1;
}

// Function: sub_a63e @ 0xa63e
void sub_a63e(int a0) // return-dupe
{
  if (a0 == dat_2d1c0)
    return;
  if (!dat_2d1c0) {
    dat_2d1c0 = a0;
    return;
  }
  sub_a338("conflicting output style options",0); // no-return
}

// Function: sub_a682 @ 0xa682
void sub_a682(char *a0)
{
  if ((!a0) || (!strcmp(a0,"auto"))) {
    dat_2d1c4 = 1;
    return;
  }
  if (!strcmp(a0,"always")) {
    dat_2d1c4 = 2;
    return;
  }
  if (!strcmp(a0,"never")) {
    dat_2d1c4 = 0;
    return;
  }
  sub_a338("invalid color \'%s\'",a0); // no-return
}

// Function: sub_a723 @ 0xa723
void sub_a723(long a0)
{
  sub_14c76((timespec *)(a0 + 0x58));
}

// Function: sub_a746 @ 0xa746
int sub_a746(void *a0,char *a1,char *a2) // ternary x2
{
  bool v1;
  int v10 [2]; // stack - 0x288
  stat v11 [2]; // stack - 0x278
  long v12; // stack - 0x148
  char *v13 [2]; // stack - 0x298
  char *v14; // stack - 0x310
  char *v15; // stack - 0x308
  int v16; // stack - 0x2f4
  char *v17; // stack - 0x2e0
  char *v18; // stack - 0x2d8
  char *v19; // stack - 0x280
  int v2;
  int v20; // stack - 0x158
  char *v21; // stack - 0x150
  long v22; // stack - 0x140
  long v23; // stack - 0x138
  unsigned int v24; // stack - 0x130
  int v25; // stack - 0x12c
  int v26; // stack - 0x128
  long v27; // stack - 0x120
  long v28; // stack - 0x118
  long v29; // stack - 0xf0
  int v3; // eax
  long v30; // stack - 0xe0
  void *v31; // stack - 0x28
  int v4;
  long v5; // rax
  unsigned long v6;
  char *v7;
  long v8;
  char *v9;
  
  v16 = 0;
  if ((((!a1) || (!a2)) && ((dat_2d649 != '\x01' || (!a2)))) && (dat_2d648 != '\x01')) {
    if (a1)
      a2 = a1;
    sub_109a8("Only in %s: %s\n",*(unsigned long *)((long)a0 + (long)(int)(unsigned int)(a1 == NULL) * 0x130 + 8),a2);
    return 1;
  }
  memset(v10,0,0x260);
  v31 = a0;
  v10[0] = (a1) ? -2 : -1; // branch-flip
  v20 = (a2) ? -2 : -1; // branch-flip
  v15 = a1;
  if (!a1)
    v15 = a2;
  v14 = a2;
  if (!a2)
    v14 = v15;
  if (a0) { // branch-flip
    v17 = (char *)sub_14ade(*(void **)((long)a0 + 8),v15,NULL);
    v19 = v17;
    v18 = (char *)sub_14ade(*(void **)((long)a0 + 0x138),v14,NULL);
    v21 = v18;
  }
  else {
    v17 = NULL;
    v18 = NULL;
    v19 = v15;
    v21 = v14;
  }
  for (v4 = 0; v4 <= 1; v4 = v4 + 1) {
    if (v10[(long)v4 * 0x4c] != -1) {
      if ((v4) && (v9 = *(char **)((long)v11 + (long)v4 * 0x130 + -8), !strcmp(v9,v19))) {
        v10[(long)v4 * 0x4c] = v10[0];
        v5 = (long)v4 * 0x130;
        *(unsigned long *)((long)v11 + v5) = v11[0]._0_8_;
        *(unsigned long *)((long)v11 + v5 + 8) = v11[0]._8_8_;
        *(unsigned long *)((long)v11 + v5 + 0x10) = v11[0]._16_8_;
        *(unsigned long *)((long)v11 + v5 + 0x18) = CONCAT44(v11[0]._28_4_,v11[0]._24_4_);
        *(unsigned long *)((long)v11 + v5 + 0x20) = CONCAT44(v11[0]._36_4_,v11[0]._32_4_);
        *(unsigned long *)((long)v11 + v5 + 0x28) = v11[0]._40_8_;
        *(unsigned long *)((long)v11 + v5 + 0x30) = v11[0]._48_8_;
        *(unsigned long *)((long)v11 + v5 + 0x38) = v11[0]._56_8_;
        *(unsigned long *)((long)v11 + v5 + 0x40) = v11[0]._64_8_;
        *(unsigned long *)((long)v11 + v5 + 0x48) = v11[0]._72_8_;
        *(unsigned long *)((long)v11 + v5 + 0x50) = v11[0]._80_8_;
        *(unsigned long *)((long)v11 + v5 + 0x58) = v11[0]._88_8_;
        *(unsigned long *)((long)v11 + v5 + 0x60) = v11[0]._96_8_;
        *(unsigned long *)((long)v11 + v5 + 0x68) = v11[0]._104_8_;
        *(unsigned long *)((long)v11 + v5 + 0x70) = v11[0]._112_8_;
        *(unsigned long *)((long)v11 + v5 + 0x78) = v11[0]._120_8_;
        *(unsigned long *)((long)v11 + v5 + 0x80) = v11[0]._128_8_;
        *(unsigned long *)((long)v11 + v5 + 0x88) = v11[0]._136_8_;
      }
      else {
        v9 = *(char **)((long)v11 + (long)v4 * 0x130 + -8);
        if (strcmp(v9,"-")) { // branch-flip
          if (dat_2d1f0) // branch-flip
            v2 = lstat(*(char **)((long)v11 + (long)v4 * 0x130 + -8),(stat *)((long)v11 + (long)v4 * 0x130));
          else {
            v2 = stat(*(char **)((long)v11 + (long)v4 * 0x130 + -8),(stat *)((long)v11 + (long)v4 * 0x130));
          }
          if (v2)
            v10[(long)v4 * 0x4c] = 0xfffffffd - *__errno_location();
        }
        else {
          v10[(long)v4 * 0x4c] = 0;
          if (!isatty(0))
            sub_131e6(0,0);
          if (fstat(0,(stat *)((long)v11 + (long)v4 * 0x130))) // branch-flip
            v10[(long)v4 * 0x4c] = 0xfffffffd - *__errno_location();
          else {
            if ((*(unsigned int *)((long)v11 + (long)v4 * 0x130 + 0x18) & 0xf000) == 0x8000) {
              v5 = lseek(0,0,1);
              if (0 <= v5) { // branch-flip
                v5 = *(long *)((long)v11 + (long)v4 * 0x130 + 0x30) - v5;
                v8 = 0;
                if (0 <= v5)
                  v8 = v5;
                *(long *)((long)v11 + (long)v4 * 0x130 + 0x30) = v8;
              }
              else {
                v10[(long)v4 * 0x4c] = 0xfffffffd - *__errno_location();
              }
            }
            sub_a723((long)v11 + (long)v4 * 0x130);
          }
        }
      }
    }
  }
  for (v4 = 0; v4 <= 1; v4 = v4 + 1) {
    if ((dat_2d648) || ((!v4 && (dat_2d649)))) {
      if (v10[(long)v4 * 0x4c] != -2) { // branch-flip
        if (((v10[(long)v4 * 0x4c] != -5) && (v10[(long)v4 * 0x4c] != -0xc)) || ((a0 || ((v10[(long)(1 - v4) * 0x4c] != -2 && (v10[(long)(1 - v4) * 0x4c]))))))
          v1 = 0;
        else {
          v1 = 1;
        }
      }
      else if ((((*(unsigned int *)((long)v11 + (long)v4 * 0x130 + 0x18) & 0xf000) != 0x8000) || (*(unsigned int *)((long)v11 + (long)v4 * 0x130 + 0x18) & 0x1ff)) || (*(long *)((long)v11 + (long)v4 * 0x130 + 0x30)))
        v1 = 0;
      else {
        v1 = 1;
      }
      if (v1)
        v10[(long)v4 * 0x4c] = -1;
    }
  }
  for (v4 = 0; v4 <= 1; v4 = v4 + 1) {
    if (v10[(long)v4 * 0x4c] == -1) {
      memset((void *)((long)v11 + (long)v4 * 0x130),0,0x90);
      *(unsigned int *)((long)v11 + (long)v4 * 0x130 + 0x18) = *(unsigned int *)((long)v11 + (long)(1 - v4) * 0x130 + 0x18);
    }
  }
  for (v4 = 0; v4 <= 1; v4 = v4 + 1) {
    v2 = v10[(long)v4 * 0x4c];
    if (0 <= 0xfffffffdU - v2) {
      *__errno_location() = 0xfffffffdU - v2;
      sub_108ec(*(unsigned long *)((long)v11 + (long)v4 * 0x130 + -8));
      v16 = 2;
    }
  }
  if (((!v16) && (!a0)) && (((v24 & 0xf000) == 0x4000) != ((v11[0]._24_4_ & 0xf000) == 0x4000))) {
    v3 = (unsigned int)((v11[0]._24_4_ & 0xf000) == 0x4000);
    v4 = 1 - v3;
    v9 = *(char **)((long)v11 + (long)v3 * 0x130 + -8);
    v6 = *(unsigned long *)((long)v11 + (long)v4 * 0x130 + -8);
    v17 = (char *)sub_c9bd(v6,(char *)sub_13111(v9));
    *(char **)((long)v11 + (long)v4 * 0x130 + -8) = v17;
    v7 = *(char **)((long)v11 + (long)v4 * 0x130 + -8);
    if (!strcmp(v9,"-"))
      sub_10966("cannot compare \'-\' to a directory"); // no-return
    if (dat_2d1f0) // branch-flip
      v4 = lstat(v7,(stat *)((long)v11 + (long)v4 * 0x130));
    else {
      v4 = stat(v7,(stat *)((long)v11 + (long)v4 * 0x130));
    }
    if (v4) {
      sub_108ec(v7);
      v16 = 2;
    }
  }
  if ((v16) || ((v10[0] == -1 && (v20 == -1)))) goto label_bcfe;
  if ((v10[0] != -1) && (v20 != -1)) {
    if (((v11[0]._8_8_ != v22) || (v11[0]._0_8_ != v12)) && (((((v11[0]._24_4_ & 0xf000) != 0x6000 || ((v24 & 0xf000) != 0x6000)) && (((v11[0]._24_4_ & 0xf000) != 0x2000 || ((v24 & 0xf000) != 0x2000)))) || (v11[0]._40_8_ != v27))))
      v1 = 0;
    else {
      v1 = 1;
    }
    if ((((((!v1) || (v11[0]._24_4_ != v24)) || (v11[0]._16_8_ != v23)) || ((v11[0]._28_4_ != v25 || (v11[0]._32_4_ != v26)))) || (v11[0]._48_8_ != v28)) || ((v11[0]._88_8_ != v29 || (v11[0]._104_8_ != v30)))) goto label_b57a;
    v1 = 1;
  }
  else {
label_b57a:
    v1 = 0;
  }
  if ((!v1) || (!dat_2d1c8)) {
    if ((v24 & 0xf000) != 0x4000 || (v11[0]._24_4_ & 0xf000) != 0x4000) { // branch-flip
      if (((v24 & 0xf000) != 0x4000 && (v11[0]._24_4_ & 0xf000) != 0x4000) && ((!a0 || ((((v11[0]._24_4_ & 0xf000) == 0x8000 || ((v11[0]._24_4_ & 0xf000) == 0xa000)) && (((v24 & 0xf000) == 0x8000 || ((v24 & 0xf000) == 0xa000)))))))) {
        if (((v11[0]._24_4_ & 0xf000) != 0xa000) && ((v24 & 0xf000) != 0xa000)) {
          if ((((dat_2d1ed) && ((v11[0]._24_4_ & 0xf000) == 0x8000)) && ((v24 & 0xf000) == 0x8000)) && (((v11[0]._48_8_ != v28 && (1 <= (long)v11[0]._48_8_)) && (1 <= v28)))) {
            if (dat_2d208) // branch-flip
              v9 = dat_2d208;
            else {
              v9 = v21;
            }
            if (dat_2d200) // branch-flip
              v7 = dat_2d200;
            else {
              v7 = v19;
            }
            sub_109a8("Files %s and %s differ\n",v7,v9);
            v16 = 1;
          }
          else {
            if (v10[0] == -2) {
              v10[0] = open(v19,0,0);
              if (v10[0] <= -1) {
                sub_108ec(v19);
                v16 = 2;
              }
            }
            if (v20 == -2) {
              if (v1)
                v20 = v10[0];
              else {
                v20 = open(v21,0,0);
                if (v20 <= -1) {
                  sub_108ec(v21);
                  v16 = 2;
                }
              }
            }
            if (!v16)
              v16 = sub_7206(v10);
            if ((0 <= v10[0]) && (close(v10[0]))) {
              sub_108ec(v19);
              v16 = 2;
            }
            if (((0 <= v20) && (v10[0] != v20)) && (close(v20))) {
              sub_108ec(v21);
              v16 = 2;
            }
          }
        }
        else {
          if (!dat_2d1f0)
            __assert_fail("no_dereference_symlinks","diff.c",0x549,"compare_files"); // no-return
          if (((v11[0]._24_4_ & 0xf000) != 0xa000) || ((v24 & 0xf000) != 0xa000)) {
            v5 = sub_14997(&v12);
            if (dat_2d208) // branch-flip
              v9 = dat_2d208;
            else {
              v9 = v21;
            }
            v6 = sub_14997(v11);
            if (dat_2d200) // branch-flip
              v7 = dat_2d200;
            else {
              v7 = v19;
            }
            sub_109e2("File %s is a %s while file %s is a %s\n",v7,v6,v9,v5);
            v16 = 1;
          }
          else {
            v13[0] = NULL;
            v13[1] = NULL;
            for (v4 = 0; v4 <= 1; v4 = v4 + 1) {
              v6 = *(unsigned long *)((long)v11 + (long)v4 * 0x130 + -8);
              v13[v4] = (char *)sub_1cf7b(v6);
              if (!v13[v4]) {
                sub_108ec(*(unsigned long *)((long)v11 + (long)v4 * 0x130 + -8));
                v16 = 2;
                break;
              }
            }
            if ((!v16) && (strcmp(v13[0],v13[1]))) {
              sub_109a8("Symbolic links %s and %s differ\n",v19,v21);
              v16 = 1;
            }
            for (v4 = 0; v4 <= 1; v4 = v4 + 1) {
              free(v13[v4]);
            }
          }
        }
      }
      else if ((v10[0] != -1) && (v20 != -1)) {
        v5 = sub_14997(&v12);
        if (dat_2d208) // branch-flip
          v9 = dat_2d208;
        else {
          v9 = v21;
        }
        v6 = sub_14997(v11);
        if (dat_2d200) // branch-flip
          v7 = dat_2d200;
        else {
          v7 = v19;
        }
        sub_109e2("File %s is a %s while file %s is a %s\n",v7,v6,v9,v5);
        v16 = 1;
      }
      else if (((v24 & 0xf000) != 0x4000 && (v11[0]._24_4_ & 0xf000) != 0x4000) || ((!dat_2d5c9 || ((!dat_2d648 && ((!dat_2d649 || (v10[0] != -1)))))))) {
        if (!a0)
          __assert_fail("parent","diff.c",0x52d,"compare_files"); // no-return
        sub_109a8("Only in %s: %s\n",*(unsigned long *)((long)a0 + (long)(int)(unsigned int)(v10[0] == -1) * 0x130 + 8),v15);
        v16 = 1;
      }
      else {
        v16 = sub_c21b(v10,sub_a746);
      }
    }
    else {
      if (dat_2d1c0 == 7)
        sub_10966("-D option not supported with directories"); // no-return
      if ((a0) && (dat_2d5c9 != '\x01'))
        sub_109a8("Common subdirectories: %s and %s\n",v19,v21);
      else {
        v16 = sub_c21b(v10,sub_a746);
      }
    }
  }
label_bcfe:
  if (v16) { // branch-flip
    if (fflush_unlocked(stdout))
      sub_10925(gettext("standard output")); // no-return
  }
  else if ((dat_2d64a) && ((v11[0]._24_4_ & 0xf000) != 0x4000)) {
    if (dat_2d208) // branch-flip
      v9 = dat_2d208;
    else {
      v9 = v21;
    }
    if (dat_2d200) // branch-flip
      v7 = dat_2d200;
    else {
      v7 = v19;
    }
    sub_109a8("Files %s and %s are identical\n",v7,v9);
  }
  free(v17);
  free(v18);
  return v16;
}

// Function: sub_bde2 @ 0xbde2
unsigned long sub_bde2(struct_11 *a0,unsigned long *a1) // return-dupe x3
{
  char *v1;
  unsigned long v10; // stack - 0x40
  long v11; // stack - 0x38
  int v2;
  char v3; // al
  DIR *v4; // rax
  dirent *v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rbx
  unsigned long v8; // stack - 0x50
  char *v9; // stack - 0x48
  
  a1[1] = 0;
  a1[2] = 0;
  v8 = 0;
  v9 = NULL;
  if (a0->field_0x0 != -1) {
    v4 = opendir(a0->field_0x8);
    if (!v4)
      return 0;
    v10 = 0x200;
    v11 = 0;
    v9 = (char *)sub_1c945(0x200);
    a1[2] = (unsigned long)v9;
    while( true ) {
      *__errno_location() = 0;
      v5 = readdir(v4);
      if (!v5) break;
      v1 = &v5->field_0x13;
      v6 = strlen(&v5->field_0x13) + 1;
      if (((*v1 != '.') || ((v5->field_0x14 && ((v5->field_0x14 != '.' || (v5->field_0x15)))))) && (v3 = sub_141f6(dat_2d340,v1), !v3)) {
        while (v10 < v6 + v11) {
          if (0x3fffffffffffffff <= v10)
            sub_1cf09(); // no-return
          v10 <<= 1;
          v9 = (char *)sub_1c991(v9,v10);
          a1[2] = (unsigned long)v9;
        }
        memcpy(&v9[v11],v1,v6);
        v11 += v6;
        v8 += 1;
      }
    }
    if (*__errno_location()) {
      v2 = *__errno_location();
      closedir(v4);
      *__errno_location() = v2;
      return 0;
    }
    if (closedir(v4))
      return 0;
  }
  if (v8 <= 0xffffffffffffffd) {
    v6 = sub_1c945((v8 + 1) * 8);
    a1[1] = v6;
    *a1 = v8;
    for (v7 = 0; v7 < v8; v7 = v7 + 1) {
      *(char **)(v7 * 8 + v6) = v9;
      v9 = &v9[strlen(v9) + 1];
    }
    *(unsigned long *)(v6 + v8 * 8) = 0;
    return 1;
  }
  sub_1cf09(); // no-return
}

// Function: sub_c084 @ 0xc084
int sub_c084(char *a0,char *a1) // ternary
{
  char *v1; // rax
  int v2; // stack - 0x1c
  
  *__errno_location() = 0;
  v2 = (dat_2d1ef) ? strcasecmp(a0,a1) : strcoll(a0,a1); // branch-flip
  if (*__errno_location()) {
    v1 = gettext("cannot compare file names \'%s\' and \'%s\'");
    error(0,*__errno_location(),v1,a0,a1); // no-return
  }
  return v2;
}

// Function: sub_c140 @ 0xc140
unsigned long sub_c140(char *a0,char *a1)
{
  unsigned int v1; // eax
  
  if ((dat_2d660) && ((v1 = sub_c084(a0,a1), v1 || (dat_2d1ef))))
    return (unsigned long)v1;
  return CONCAT44(dat_4,strcmp(a0,a1));
}

// Function: sub_c1a0 @ 0xc1a0
unsigned long sub_c1a0(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  unsigned long v4; // rax
  
  v1 = (char *)*a0;
  v2 = (char *)*a1;
  if (dat_2d660) { // branch-flip
    v3 = sub_c084(v1,v2);
    if (!v3) {
      v3 = strcmp(v1,v2); // return-dupe
      return v4;
    }
    v4 = (unsigned long)v3;
  }
  else {
    v3 = strcmp(v1,v2);
  }
  return v4;
}

// Function: sub_c21b @ 0xc21b
unsigned int sub_c21b(int *a0,void *a1)
{
  char *v1;
  long *v10 [9]; // stack - 0x68
  int v11; // stack - 0xa0
  int v12; // stack - 0x9c
  long *v13; // stack - 0x80
  long *v2;
  int v3;
  int v4;
  int v5;
  char v6; // al
  int v7; // eax
  struct_11 *v8;
  long v9;
  
  v11 = 0;
  if (((*a0 == -1) || (v6 = sub_c796(a0,0), v6)) && ((a0[0x4c] == -1 || (v6 = sub_c796(a0,1), v6)))) {
    v7 = (unsigned int)(*a0 == -1);
    v8 = (int *)&a0[(long)v7 * 0x4c + 2];
    v1 = *(char **)v8;
    v8 = (char *)gettext("%s: recursive directory loop");
    error(0,0,(char *)v8,v1); // no-return
  }
  for (v12 = 0; v12 <= 1; v12 = v12 + 1) {
    v8 = (struct_11 *)&a0[(long)v12 * 0x4c];
    v6 = sub_bde2(v8,&v10[(long)v12 * 3 + 2]);
    if (v6 != '\x01') {
      v8 = (int *)&a0[(long)v12 * 0x4c + 2];
      sub_108ec(*(unsigned long *)v8);
      v11 = 2;
    }
  }
  if (!v11) {
    v10[0] = v10[3];
    v10[1] = v10[6];
    dat_2d660 = 1;
    v7 = _setjmp((void *)0x2d680);
    if (v7)
      dat_2d660 = 0;
    for (v12 = 0; v12 <= 1; v12 = v12 + 1) {
      v8 = (long **)&v10[(long)v12 * 3 + 2];
      v2 = *(long **)v8;
      v8 = (long)(long)v12;
      qsort(v10[(long)v8],v2,8,sub_c1a0);
    }
    if ((dat_2d2b8) && (!*(long *)&a0[0x98])) {
      while ((*v10[0] && (v7 = sub_c140((char *)*v10[0],dat_2d2b8), v7 <= -1))) {
        v8 = (long *)&v10[0][1];
        v10[0] = (long *)v8;
      }
      while ((*v10[1] && (v7 = sub_c140((char *)*v10[1],dat_2d2b8), v7 <= -1))) {
        v8 = (long *)&v10[1][1];
        v10[1] = (long *)v8;
      }
    }
    while ((*v10[0] || (*v10[1]))) {
      if (*v10[0]) { // branch-flip
        if (*v10[1]) // branch-flip
          v7 = sub_c140((char *)*v10[0],(char *)*v10[1]);
        else {
          v7 = -1;
        }
      }
      else {
        v7 = 1;
      }
      v4 = v7;
      if (((!v7) && (dat_2d1ef)) && (v7 = strcmp((char *)*v10[0],(char *)*v10[1]), v7)) {
        v3 = v7 >> 0x1f;
        v7 = -v3;
        v5 = v7;
        v7 = v3 + 1;
        v8 = (long)(long)v7;
        v2 = v10[(long)v8];
        v8 = (long)(long)v5;
        v1 = (char *)*v10[(long)v8];
        v13 = v2;
        do {
          v13 = &v13[1];
          if ((!*v13) || (v7 = sub_c140((char *)*v13,v1), v7)) goto label_c676;
          v7 = strcmp((char *)*v13,v1);
        } while (v7 <= -1);
        if (!v7) {
          v8 = (unsigned long)((long)v13 - (long)v2);
          memmove(&v2[1],v2,(unsigned long)v8);
          *v2 = (long)v1;
        }
      }
label_c676:
      if (0 <= v4) { // branch-flip
        v2 = &v10[1][1];
        v9 = *v10[1];
        v10[1] = v2;
      }
      else {
        v9 = 0;
      }
      if (1 <= v4) // branch-flip
        v8 = NULL;
      else {
        v2 = &v10[0][1];
        v8 = (long)*v10[0];
        v10[0] = v2;
      }
      v7 = (*a1)(a0,v8,v9);
      if (v11 < v7)
        v11 = v7;
    }
  }
  for (v12 = 0; v12 <= 1; v12 = v12 + 1) {
    v8 = (long **)&v10[(long)v12 * 3 + 3];
    free(*(long **)v8);
    v8 = (long **)&v10[(long)v12 * 3 + 4];
    free(*(long **)v8);
  }
  v7 = v11;
  return v7;
}

// Function: sub_c796 @ 0xc796
unsigned long sub_c796(void *a0,int a1)
{
  bool v1;
  void *v2; // stack - 0x10
  
  v2 = a0;
  do {
    v2 = *(void **)((long)v2 + 0x260);
    if (!v2)
      return 0;
    if (((*(long *)((long)v2 + (long)a1 * 0x130 + 0x18) != *(long *)((long)a0 + (long)a1 * 0x130 + 0x18)) || (*(long *)((long)v2 + (long)a1 * 0x130 + 0x10) != *(long *)((long)a0 + (long)a1 * 0x130 + 0x10))) && (((((*(unsigned int *)((long)v2 + (long)a1 * 0x130 + 0x28) & 0xf000) != 0x6000 || ((*(unsigned int *)((long)a0 + (long)a1 * 0x130 + 0x28) & 0xf000) != 0x6000)) && (((*(unsigned int *)((long)v2 + (long)a1 * 0x130 + 0x28) & 0xf000) != 0x2000 || ((*(unsigned int *)((long)a0 + (long)a1 * 0x130 + 0x28) & 0xf000) != 0x2000)))) || (*(long *)((long)v2 + (long)a1 * 0x130 + 0x38) != *(long *)((long)a0 + (long)a1 * 0x130 + 0x38)))))
      v1 = 0;
    else {
      v1 = 1;
    }
  } while (!v1);
  return 1;
}

// Function: sub_c9bd @ 0xc9bd
unsigned long sub_c9bd(void *a0,char *a1)
{
  char *v1;
  unsigned long v2; // rax
  char v3 [8];
  unsigned int v4 [2]; // stack - 0x148
  char *v5; // stack - 0x180
  long *v6; // stack - 0x178
  long *v7; // stack - 0x160
  void *v8; // stack - 0x158
  void *v9; // stack - 0x140
  
  v7 = NULL;
  v8 = NULL;
  v5 = a1;
  if (dat_2d1ef) {
    v4[0] = 0;
    v9 = a0;
    if (sub_bde2(v4,v3)) {
      dat_2d660 = 1;
      if (!_setjmp((void *)0x2d680)) {
        for (v6 = v7; *v6; v6 = &v6[1]) {
          v1 = (char *)*v6;
          if (!sub_c140(v1,a1)) {
            v1 = (char *)*v6;
            if (!strcmp(v1,a1)) {
              v5 = (char *)*v6;
              break;
            }
            if (v5 == a1)
              v5 = (char *)*v6;
          }
        }
      }
    }
  }
  v2 = sub_14ade(a0,v5,NULL);
  free(v7);
  free(v8);
  return v2;
}

// Function: sub_cb85 @ 0xcb85
void sub_cb85(long a0)
{
  sub_1245b(a0,sub_12449,sub_cbb5);
}

// Function: sub_cbb5 @ 0xcbb5
void sub_cbb5(long *a0) // return-dupe x3
{
  bool v1;
  unsigned int v2; // eax
  long v3; // stack - 0x20
  long v4; // stack - 0x28
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x38
  long v7; // stack - 0x18
  
  v2 = sub_12af4(a0,&v6,&v5,&v4,&v3);
  if (!v2)
    return;
  sub_11b50();
  sub_12a37(0x2c,(struct_20 *)0x2d360,v6,v5);
  fputc_unlocked((int)*(char *)((unsigned long)v2 + 0x25780),dat_2d5c0);
  fputc_unlocked(10,dat_2d5c0);
  if (v2 == 1)
    return;
  v1 = 1;
  for (v7 = v4; v7 <= v3; v7 = v7 + 1) {
    if (!v1) {
      fputs_unlocked("a\n",dat_2d5c0);
      v1 = 1;
    }
    if ((**(char **)(v7 * 8 + dat_2d548) != '.') || (*(char *)(*(long *)(v7 * 8 + dat_2d548) + 1) != '\n'))
      sub_124cf("",(struct_21 *)(v7 * 8 + dat_2d548));
    else {
      fputs_unlocked("..\n.\ns/.//\n",dat_2d5c0);
      v1 = 0;
    }
  }
  if (!v1)
    return;
  fputs_unlocked(".\n",dat_2d5c0);
}

// Function: sub_cd6a @ 0xcd6a
void sub_cd6a(long a0)
{
  sub_1245b(a0,sub_12437,sub_cd9a);
}

// Function: sub_cd9a @ 0xcd9a
void sub_cd9a(long *a0) // return-dupe x2
{
  unsigned int v1; // eax
  long v2; // stack - 0x20
  long v3; // stack - 0x28
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x38
  long v6; // stack - 0x18
  
  v1 = sub_12af4(a0,&v5,&v4,&v3,&v2);
  if (!v1)
    return;
  sub_11b50();
  fputc_unlocked((int)*(char *)((unsigned long)v1 + 0x25780),dat_2d5c0);
  sub_12a37(0x20,(struct_20 *)0x2d360,v5,v4);
  fputc_unlocked(10,dat_2d5c0);
  if (v1 == 1)
    return;
  for (v6 = v3; v6 <= v2; v6 = v6 + 1) {
    sub_124cf("",(struct_21 *)(v6 * 8 + dat_2d548));
  }
  fputs_unlocked(".\n",dat_2d5c0);
}

// Function: sub_ceba @ 0xceba
void sub_ceba(long a0)
{
  sub_1245b(a0,sub_12437,sub_ceea);
}

// Function: sub_ceea @ 0xceea
void sub_ceea(long *a0) // return-dupe x2
{
  unsigned int v1; // eax
  long v10; // stack - 0x28
  long v11; // stack - 0x18
  long v2;
  long v3; // stack - 0x40
  long v4; // stack - 0x48
  unsigned long v5; // stack - 0x50
  unsigned long v6; // stack - 0x58
  long v7; // stack - 0x30
  long v8; // stack - 0x38
  long v9; // stack - 0x20
  
  v1 = sub_12af4(a0,&v6,&v5,&v4,&v3);
  if (!v1)
    return;
  sub_11b50();
  sub_129d0((struct_20 *)0x2d360,v6,v5,&v8,&v7);
  if (v1 & 1) {
    if (v8 <= v7) // branch-flip
      v2 = (v7 - v8) + 1;
    else {
      v2 = 1;
    }
    fprintf(dat_2d5c0,"d%ld %ld\n",v8,v2);
  }
  if (!(v1 & 2))
    return;
  sub_129d0((struct_20 *)0x2d490,v4,v3,&v10,&v9);
  if (v10 <= v9) // branch-flip
    v2 = (v9 - v10) + 1;
  else {
    v2 = 1;
  }
  fprintf(dat_2d5c0,"a%ld %ld\n",v7,v2);
  for (v11 = v4; v11 <= v3; v11 = v11 + 1) {
    sub_124cf("",(struct_21 *)(v11 * 8 + dat_2d548));
  }
}

// Function: sub_d076 @ 0xd076
void sub_d076(long a0) // return-dupe
{
  dat_2d748 = -dat_2d448;
  dat_2d750 = dat_2d748;
  sub_1245b(a0,sub_12437,sub_d11e);
  if ((dat_2d430 <= dat_2d748) && (dat_2d560 <= dat_2d750))
    return;
  sub_11b50();
  sub_d221(dat_2d2e0,dat_2d748,dat_2d430,dat_2d750,dat_2d560);
}

// Function: sub_d11e @ 0xd11e
void sub_d11e(long *a0) // return-dupe
{
  unsigned int v1; // eax
  long v2; // stack - 0x18
  long v3; // stack - 0x20
  long v4; // stack - 0x28
  long v5; // stack - 0x30
  
  v1 = sub_12af4(a0,&v5,&v4,&v3,&v2);
  if (!v1)
    return;
  sub_11b50();
  if ((dat_2d748 < v5) || (dat_2d750 < v3))
    sub_d221(dat_2d2e0,dat_2d748,v5,dat_2d750,v3);
  dat_2d748 = v4 + 1;
  dat_2d750 = v2 + 1;
  sub_d221(*(char **)((unsigned long)v1 * 8 + 0x2d2e0),v5,dat_2d748,v3,dat_2d750);
}

// Function: sub_d221 @ 0xd221
void sub_d221(char *a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  
  v1 = 0x2d360;
  v4 = 0x2d490;
  v2 = a1;
  v3 = a2;
  v5 = a3;
  v6 = a4;
  sub_d2bc(dat_2d5c0,a0,0,&v1);
}

// Function: sub_d2bc @ 0xd2bc
char * sub_d2bc(FILE *a0,char *a1,char a2,unsigned long *a3)
{
  char *v1;
  char *v2;
  char *v3; // stack - 0x68
  long v4 [3]; // stack - 0x48
  char v5;
  int v6; // stack - 0x6c
  FILE *v7; // stack - 0x60
  FILE *v8; // stack - 0x58
  char *v9; // stack - 0x50
  
label_d542:
  do {
    v5 = *a1;
    if ((v5 == a2) || (!v5))
      return a1;
    v9 = &a1[1];
    v2 = v9;
    if (v5 != '%') goto label_d52f;
    v2 = &a1[2];
    v5 = *v9;
    switch(v5) {
      case 0x25:
label_d52f:
        a1 = v2;
        if (a0)
          putc_unlocked((int)v5,a0);
        goto label_d542;
      default:
        a1 = (char *)sub_da03(a0,a1,0,0,a3);
        if (a1) goto label_d542;
label_d522:
        v5 = '%';
        v2 = v9;
        goto label_d52f;
      case 0x28:
        for (v6 = 0; v6 <= 1; v6 = v6 + 1) {
          if (10 <= (unsigned int)((int)*v2 - 0x30U)) { // branch-flip
            v5 = *v2;
            v4[v6] = sub_d577(a3,(int)v5);
            if (v4[v6] < 0) goto label_d522;
            v1 = &v2[1];
          }
          else {
            *__errno_location() = 0;
            v4[v6] = strtoimax(v2,&v3,10);
            if (*__errno_location()) goto label_d522;
            v1 = v3;
          }
          v2 = &v1[1];
          if (*v1 != "=?"[v6]) goto label_d522;
        }
        if (v4[0] != v4[1]) { // branch-flip
          v7 = NULL;
          v8 = a0;
        }
        else {
          v8 = NULL;
          v7 = a0;
        }
        a1 = (char *)sub_d2bc(v7,v2,0x3a,a3);
        if ((*a1) && (a1 = (char *)sub_d2bc(v8,&a1[1],0x29,a3), *a1))
          a1 = &a1[1];
        break;
      case 0x3c:
        sub_d6bd(a0,dat_2d308,a3);
        a1 = v2;
        break;
      case 0x3d:
        sub_d6bd(a0,dat_2d300,a3);
        a1 = v2;
        break;
      case 0x3e:
        goto label_d4cc;
      
    }
  } while( true );
label_d4cc:
  sub_d6bd(a0,dat_2d310,&a3[3]);
  a1 = v2;
  goto label_d542;
}

// Function: sub_d577 @ 0xd577
long sub_d577(unsigned long *a0,char a1) // early-return
{
  struct_20 *v1;
  long v2;
  char v3; // stack - 0x14
  unsigned long *v4; // stack - 0x10
  
  v3 = a1;
  v4 = a0;
  switch(a1) {
    case 0x45:
      v3 = 0x65;
      v4 = &a0[3];
      break;
    case 0x46:
      v3 = 0x66;
      v4 = &a0[3];
      break;
    case 0x4c:
      v3 = 0x6c;
      v4 = &a0[3];
      break;
    case 0x4d:
      v3 = 0x6d;
      v4 = &a0[3];
      break;
    case 0x4e:
      v3 = 0x6e;
      v4 = &a0[3];
    
  }
  switch(v3) {
    case 0x65:
      v2 = v4[1];
      v1 = (struct_20 *)*v4;
      v2 = sub_129a8(v1,v2) + -1;
      break;
    case 0x66:
      v2 = sub_129a8((struct_20 *)*v4,v4[1]);
      break;
    default:
      return -1;
    case 0x6c:
      v2 = v4[2];
      v1 = (struct_20 *)*v4;
      v2 = sub_129a8(v1,v2) + -1;
      break;
    case 0x6d:
      v2 = sub_129a8((struct_20 *)*v4,v4[2]);
      break;
    case 0x6e:
      v2 = v4[2] - v4[1];
    
  }
  return v2;
}

// Function: sub_d6bd @ 0xd6bd
void sub_d6bd(FILE *a0,char *a1,long *a2) // return-dupe x3
{
  char *v1;
  long v2;
  long v3;
  long v4;
  char *v5;
  char *v6;
  char v7;
  long v8; // stack - 0x50
  
  v2 = *a2;
  v3 = *(long *)(v2 + 0xb8);
  v8 = a2[1];
  v4 = a2[2];
  if (!a0)
    return;
  if ((dat_2d2a1 != '\x01') && (*a1 == '%')) {
    if ((a1[1] == 'l') && (((a1[2] == '\n' && (!a1[3])) && (v8 < v4)))) {
      fwrite_unlocked(*(void **)(v3 + v8 * 8),1,(*(long *)(v3 + v4 * 8) + (unsigned long)(*(char *)(*(long *)(v3 + v4 * 8) + -1) != '\n')) - *(long *)(v3 + v8 * 8),a0);
      return;
    }
    if ((a1[1] == 'L') && (!a1[2])) {
      fwrite_unlocked(*(void **)(v3 + v8 * 8),1,*(long *)(v3 + v4 * 8) - *(long *)(v3 + v8 * 8),a0);
      return;
    }
  }
  do {
    if (v4 <= v8)
      return;
    v5 = a1;
    while( true ) {
      v1 = &v5[1];
      v7 = *v5;
      if (!v7) break;
      v6 = v1;
      if (v7 != '%') { // branch-flip
label_d9c0:
        putc_unlocked((int)v7,a0); // crossjump-dupe
        v5 = v6;
      }
      else {
        v6 = &v5[2];
        v7 = *v1;
        if (v7 != 'l') { // branch-flip
          if ('m' <= v7) { // branch-flip
label_d989:
            v5 = (char *)sub_da03(a0,v5,v2,v8,0);
            if (!v5) {
              v7 = '%';
              v6 = v1;
              putc_unlocked((int)v7,a0);
              v5 = v6;
            }
          }
          else {
            if (v7 == '%') goto label_d9c0;
            if (v7 != 'L') goto label_d989;
            sub_1267d(*(unsigned char **)(v3 + v8 * 8),*(unsigned char **)(v3 + (v8 + 1) * 8),NULL,0);
            v5 = v6;
          }
        }
        else {
          sub_1267d(*(unsigned char **)(v3 + v8 * 8),(unsigned char *)(*(long *)(v3 + (v8 + 1) * 8) - (unsigned long)(*(char *)(*(long *)(v3 + (v8 + 1) * 8) + -1) == '\n')),NULL,0);
          v5 = v6;
        }
      }
    }
    v8 += 1;
  } while( true );
}

// Function: sub_da03 @ 0xda03
char * sub_da03(FILE *a0,void *a1,long a2,unsigned long a3,unsigned long a4) // early-return x5
{
  long v1;
  long v10; // stack - 0x70
  void *v11; // stack - 0x68
  FILE *v12; // stack - 0x60
  char v13; // stack - 0x4a
  char v14; // stack - 0x49
  char *v15; // stack - 0x48
  long v16; // stack - 0x40
  long v17; // stack - 0x38
  char *v18; // stack - 0x30
  unsigned long v19; // stack - 0x28
  char *v2;
  char *v20; // stack - 0x20
  char *v21; // stack - 0x18
  unsigned long v3; // rax
  char v4 [8];
  char v5; // stack - 0x4b
  char *v6; // rsp
  char *v7; // rsp
  unsigned long v8; // stack - 0x80
  unsigned long v9; // stack - 0x78
  
  v6 = v4;
  v15 = (char *)((long)a1 + 1);
  do {
    do {
      v2 = &v15[1];
      v13 = *v15;
      v15 = v2;
    } while (v13 == '-');
  } while ((v13 == '\'') || (v13 == '0'));
  while ((unsigned int)((int)v13 - 0x30U) <= 9) {
    v2 = &v15[1];
    v13 = *v15;
    v15 = v2;
  }
  if (v13 == '.') {
    do {
      v2 = &v15[1];
      v13 = *v15;
      v15 = v2;
    } while ((unsigned int)((int)v13 - 0x30U) <= 9);
  }
  v2 = &v15[1];
  v14 = *v15;
  if (0x21 <= (unsigned int)((int)v13 - 0x58U))
    return NULL;
  v3 = 1L << ((unsigned char)((int)v13 - 0x58U) & 0x3f);
  v8 = a4;
  v9 = a3;
  v10 = a2;
  v11 = a1;
  v12 = a0;
  v15 = v2;
  if (v3 & 0x100801001) { // branch-flip
    if (a2) { // branch-flip
      if (v14 != 'n')
        return NULL;
      v16 = sub_129a8(a2,a3);
    }
    else {
      v16 = sub_d577(a4,(int)v14);
      if (v16 <= -1)
        return NULL;
    }
    if (v12) {
      v17 = v16;
      v18 = &v15[0xfffffffffffffffe - (long)v11];
      v19 = 1;
      if (0xfa1 <= &v15[1 - (long)v11]) { // branch-flip
        v2 = (char *)sub_1cf49(&v15[1 - (long)v11]);
        v7 = v4;
      }
      else {
        v3 = ((unsigned long)&v15[0x37 - (long)v11] / 0x10) * 0x10;
        for (; v6 != &v4[-(v3 & 0xfffffffffffff000)]; v6 = &v6[-0x1000]) {
          *(unsigned long *)&v6[-8] = *(unsigned long *)&v6[-8];
        }
        v1 = -(unsigned long)((unsigned int)v3 & 0xfff);
        v7 = &v6[v1];
        if (v3 & 0xfff)
          *(unsigned long *)&v6[((unsigned long)((unsigned int)v3 & 0xfff) - 8) + v1] = *(unsigned long *)&v6[((unsigned long)((unsigned int)v3 & 0xfff) - 8) + v1];
        v2 = (char *)(((unsigned long)&v6[v1 + 0xf] & 0xfffffffffffffff0) + 0x1f & 0xffffffffffffffe0);
      }
      *(unsigned long *)&v7[-8] = 0xdce7;
      v20 = v2;
      v21 = &v2[(long)&v18[v19]];
      memcpy(v2,v11,(unsigned long)v18);
      *(unsigned long *)&v7[-8] = 0xdd0c;
      memcpy(&v20[(long)v18],"l",v19);
      v2 = &v21[1];
      *v21 = v13;
      *v2 = '\0';
      *(unsigned long *)&v7[-8] = 0xdd41;
      v21 = v2;
      fprintf(v12,v20,v17);
      *(unsigned long *)&v7[-8] = 0xdd4d;
      sub_168f3(v20);
    }
  }
  else {
    if (!(v3 & 0x800))
      return NULL;
    if (v14 != '\'')
      return NULL;
    v15 = (char *)sub_dd74(v2,&v5);
    if (!v15)
      return NULL;
    if (v12)
      putc_unlocked((int)v5,v12);
  }
  return v15;
}

// Function: sub_dd74 @ 0xdd74
char * sub_dd74(char *a0,char *a1) // early-return x4
{
  unsigned int v1; // eax
  char *v2;
  char *v3;
  char v4; // stack - 0x1e
  
  v4 = *a0;
  if (v4 != '\\') { // branch-flip
    if ((v4 <= '\\') && ((!v4 || (v4 == '\''))))
      return NULL;
    v3 = &a0[2];
    if (a0[1] != '\'')
      return NULL;
  }
  else {
    v4 = '\0';
    v2 = &a0[1];
    while( true ) {
      v3 = &v2[1];
      if (*v2 == '\'') break;
      v1 = (int)*v2 - 0x30;
      if (8 <= v1)
        return NULL;
      v4 = (char)v1 + v4 * '\b';
      v2 = v3;
    }
    if (((long)&v3[0xfffffffffffffffe - (long)a0] <= 0) || (4 <= (long)&v3[0xfffffffffffffffe - (long)a0]))
      return NULL;
  }
  *a1 = v4;
  return v3;
}

// Function: sub_de51 @ 0xde51
void sub_de51(struct_3 *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  if ((a1) && (a0->field_0x121 != '\x01')) {
    v1 = sub_1d1d3(a0->field_0x0,(void *)(a0->field_0xa0 + a0->field_0xb0),a1);
    if (v1 == 0xffffffffffffffff)
      sub_10925(a0->field_0x8); // no-return
    a0->field_0xb0 = a0->field_0xb0 + v1;
    a0->field_0x121 = v1 < a1;
  }
}

// Function: sub_df00 @ 0xdf00
unsigned long sub_df00(struct_4 *a0,char a1)
{
  int v1;
  unsigned long v2;
  int v3; // eax
  void *v4; // rax
  
  if (0 <= a0->field_0x0) { // branch-flip
    v2 = a0->field_0x48;
    a0->field_0xa8 = sub_1d2a2(8,v2,0x7fffffffffffffef);
    v2 = a0->field_0xa8;
    a0->field_0xa0 = (void *)sub_1c945(v2);
    if (a1 != '\x01') {
      v3 = sub_131e6(a0->field_0x0,0);
      sub_de51(a0,a0->field_0xa8);
      v2 = a0->field_0xb0;
      if (v3) {
        v1 = a0->field_0x0;
        if ((long)lseek(v1,-v2,1) <= -1)
          sub_10925(a0->field_0x8); // no-return
        sub_131e6(a0->field_0x0,v3);
        a0->field_0xb0 = 0;
        a0->field_0x121 = 0;
      }
      v4 = memchr(a0->field_0xa0,0,v2);
      return CONCAT71((undefined7)((unsigned long)v4 >> 8),v4 != NULL);
    }
  }
  else {
    a0->field_0xa8 = 8;
    v2 = a0->field_0xa8;
    a0->field_0xa0 = (void *)sub_1c945(v2);
  }
  a0->field_0xb0 = 0;
  a0->field_0x121 = 0;
  return 0;
}

// Function: sub_e09d @ 0xe09d
void sub_e09d(struct_5 *a0) // return-dupe x3
{
  unsigned long v1;
  void *v2;
  unsigned long v3; // rax
  
  if (a0->field_0x0 < 0)
    return;
  if ((a0->field_0x28 & 0xf000) == 0x8000) {
    v1 = a0->field_0x40;
    v3 = (v1 & 0xfffffffffffffff8) + 0x10;
    if (((a0->field_0x40 != v1) || (v3 < v1)) || (0x7fffffffffffffff <= v3))
      sub_1cf09(); // no-return
    if ((unsigned long)a0->field_0xa8 < v3) {
      a0->field_0xa8 = v3;
      v2 = a0->field_0xa0;
      a0->field_0xa0 = (void *)sub_1c991(v2,v3);
    }
    if (((unsigned long)a0->field_0xb0 <= v1) && (sub_de51(a0,(v1 - a0->field_0xb0) + 1), (unsigned long)a0->field_0xb0 <= v1))
      return;
  }
  sub_de51(a0,a0->field_0xa8 - a0->field_0xb0);
  if (!a0->field_0xb0)
    return;
  while( true ) {
    if (a0->field_0xb0 != a0->field_0xa8) {
      a0->field_0xa8 = a0->field_0xb0 + 0x10 & 0xfffffffffffffff8;
      v1 = a0->field_0xa8;
      v2 = a0->field_0xa0;
      a0->field_0xa0 = (void *)sub_1c991(v2,v1);
      return;
    }
    if (0x3ffffffffffffff8 <= (unsigned long)a0->field_0xa8) break;
    a0->field_0xa8 = a0->field_0xa8 * 2;
    v1 = a0->field_0xa8;
    v2 = a0->field_0xa0;
    a0->field_0xa0 = (void *)sub_1c991(v2,v1);
    sub_de51(a0,a0->field_0xa8 - a0->field_0xb0);
  }
  sub_1cf09(); // no-return
}

// Function: sub_e2f8 @ 0xe2f8
void sub_e2f8(void *a0) // ternary x3
{
  unsigned char v1;
  unsigned char *v10; // rax
  unsigned long v11;
  unsigned char *v12;
  long *v13; // rax
  bool v14; // zf
  unsigned char v15; // stack - 0xb1
  unsigned char *v16; // stack - 0xa8
  unsigned long v17; // stack - 0xa0
  unsigned long *v18; // stack - 0x98
  long v19; // stack - 0x90
  long v2;
  long v20; // stack - 0x88
  long v21; // stack - 0x80
  void *v22; // stack - 0x78
  long v23; // stack - 0x70
  unsigned long v24; // stack - 0x68
  unsigned long v25; // stack - 0x60
  unsigned long v26; // stack - 0x58
  unsigned long v27; // stack - 0x50
  unsigned char *v28; // stack - 0x48
  long v29; // stack - 0x40
  unsigned char *v3;
  unsigned long v4;
  unsigned char *v5;
  bool v6;
  unsigned int v7;
  char v8;
  unsigned char *v9;
  
  v16 = *(unsigned char **)((long)a0 + 0xe0);
  v19 = *(long *)((long)a0 + 0xb8);
  v20 = *(long *)((long)a0 + 0xd8);
  v21 = 0;
  v2 = *(long *)((long)a0 + 0xc0);
  v22 = (void *)sub_1c945(v20 << 3);
  v8 = dat_2d1ee;
  v7 = dat_2d1e8;
  v23 = dat_2d768;
  v24 = dat_2d770;
  v25 = dat_2d778;
  v3 = *(unsigned char **)((long)a0 + 0xf0);
  v10 = (unsigned char *)(*(long *)((long)a0 + 0xb0) + *(long *)((long)a0 + 0xa0));
  v14 = dat_2d1e8 != 0;
  v6 = dat_2d1ee != '\0';
  while (v9 = v16, v16 < v3) {
    v26 = 0;
    if (v7 == 5) {
      while( true ) {
        v12 = &v16[1];
        v1 = *v16;
        v16 = v12;
        if (v1 == 10) break;
        if (!(*(unsigned short *)((unsigned long)v1 * 2 + *(long *)__ctype_b_loc()) & 0x2000)) {
          v11 = (v8) ? (unsigned long)tolower((unsigned int)v1) : (unsigned long)v1; // branch-flip
          v26 = v11 + (v26 << 7 | v26 >> 0x39);
        }
      }
      goto label_e811;
    }
    if (6 <= v7) { // branch-flip
label_e783:
      if (v8) { // branch-flip
        while( true ) {
          v12 = &v16[1];
          v1 = *v16;
          v16 = v12;
          if (v1 == 10) break;
          v26 = (v26 << 7 | v26 >> 0x39) + (long)tolower((unsigned int)v1);
        }
      }
      else {
        while( true ) {
          v12 = &v16[1];
          v1 = *v16;
          v16 = v12;
          if (v1 == 10) break;
          v26 = (v26 << 7 | v26 >> 0x39) + (unsigned long)v1;
        }
      }
    }
    else if (4 <= v7) { // branch-flip
      if (v7 != 4) goto label_e783;
      while( true ) {
        v12 = &v16[1];
        v15 = *v16;
        v16 = v12;
        if (v15 == 10) break;
        if (*(unsigned short *)((unsigned long)v15 * 2 + *(long *)__ctype_b_loc()) & 0x2000) {
          do {
            v12 = &v16[1];
            v15 = *v16;
            v16 = v12;
            if (v15 == 10) goto label_e811;
          } while (*(unsigned short *)((unsigned long)v15 * 2 + *(long *)__ctype_b_loc()) & 0x2000);
          v26 = (v26 << 7 | v26 >> 0x39) + 0x20;
        }
        v11 = (v8) ? (unsigned long)tolower((unsigned int)v15) : (unsigned long)v15; // branch-flip
        v26 = v11 + (v26 << 7 | v26 >> 0x39);
      }
    }
    else {
      if (!v7) goto label_e783;
      v27 = 0;
      while( true ) {
        v12 = &v16[1];
        v15 = *v16;
        v16 = v12;
        if (v15 == 10) break;
        if ((v7 & 2) && (*(unsigned short *)((unsigned long)v15 * 2 + *(long *)__ctype_b_loc()) & 0x2000)) {
          v28 = v12;
          do {
            v16 = &v28[1];
            v1 = *v28;
            if (v1 == 10) {
              goto label_e811;
            }
            v28 = v16;
          } while (*(unsigned short *)((unsigned long)v1 * 2 + *(long *)__ctype_b_loc()) & 0x2000);
        }
        v29 = 1;
        if (v7 & 1) {
          if (v15 != 0xd) { // branch-flip
            if (0xe <= v15) { // branch-flip
label_e711:
              v27 += 1;
            }
            else if (v15 != 8) { // branch-flip
              if (v15 != 9) goto label_e711;
              v15 = 0x20;
              v29 = dat_2d2a8 - v27 % dat_2d2a8;
              v27 = (v27 <= v29 + v27) ? v29 + v27 : 0; // branch-flip
            }
            else {
              v27 -= v27 != 0;
            }
          }
          else {
            v27 = 0;
          }
        }
        if (v8)
          v15 = (unsigned char)tolower((unsigned int)v15);
        do {
          v26 = (v26 << 7 | v26 >> 0x39) + (unsigned long)v15;
          v29 -= 1;
          v16 = v12;
        } while (v29);
      }
    }
label_e811:
    v18 = (unsigned long *)((v26 % dat_2d760) * 8 + dat_2d758);
    v12 = &v16[0xffffffffffffffff - (long)v9];
    if ((((v16 == v10) && (*(char *)((long)a0 + 0x120))) && (dat_2d1c0 != 4)) && ((dat_2d1c0 != 5 && (v7 <= 1))))
      v18 = (unsigned long *)(dat_2d758 + -8);
    for (v17 = *v18; v17; v17 = *(unsigned long *)(v23 + v17 * 0x20)) {
      if (v26 == *(unsigned long *)(v23 + v17 * 0x20 + 8)) {
        v5 = *(unsigned char **)(v23 + v17 * 0x20 + 0x10);
        if (v12 != *(unsigned char **)(v23 + v17 * 0x20 + 0x18)) { // branch-flip
          if (v14) {
label_ea3f:
            if (sub_11f90(v5,v9) != '\x01') goto label_ea84;
          }
        }
        else {
          if (!memcmp(v5,v9,(unsigned long)v12)) goto label_ea84;
          if (v14 || v6) goto label_ea3f;
        }
      }
    }
    v11 = v24 + 1;
    v17 = v24;
    if (v24 == v25) {
      if (0x1ffffffffffffff <= v25)
        sub_1cf09(); // no-return
      v4 = v25 << 1;
      v23 = sub_1c991(v23,v25 << 6);
      v25 = v4;
    }
    *(unsigned long *)(v24 * 0x20 + v23) = *v18;
    *(unsigned long *)(v24 * 0x20 + v23 + 8) = v26;
    *(unsigned char **)(v24 * 0x20 + v23 + 0x10) = v9;
    *(unsigned char **)(v24 * 0x20 + v23 + 0x18) = v12;
    *v18 = v24;
    v24 = v11;
label_ea84:
    if (v21 == v20) {
      if (((0x2aaaaaaaaaaaaaa9 < v20) || (0xffffffffffffffe < (unsigned long)(v20 * 2 - v2))) || (0xfffffffffffffff <= (unsigned long)(v20 - v2)))
        sub_1cf09(); // no-return
      v20 = v20 * 2 - v2;
      v22 = (void *)sub_1c991(v22,v20 * 8);
      v19 = sub_1c991((void *)(v19 + v2 * 8),(v20 - v2) * 8) + v2 * -8;
    }
    *(unsigned char **)(v21 * 8 + v19) = v9;
    *(unsigned long *)(v21 * 8 + (long)v22) = v17;
    v21 += 1;
  }
  *(long *)((long)a0 + 200) = v21;
  v17 = 0;
  while( true ) {
    if (v21 == v20) {
      if (((0x2aaaaaaaaaaaaaa9 < v20) || (0xffffffffffffffe < (unsigned long)(v20 * 2 - v2))) || (0xfffffffffffffff <= (unsigned long)(v20 - v2)))
        sub_1cf09(); // no-return
      v20 = v20 * 2 - v2;
      v19 = sub_1c991((void *)(v19 + v2 * 8),(v20 - v2) * 8) + v2 * -8;
    }
    *(unsigned char **)(v21 * 8 + v19) = v16;
    if (v16 == v10) break;
    if ((dat_2d1d0 <= (long)v17) && (dat_2d1c8)) goto label_ed3e;
    v21 += 1;
    do {
      v3 = &v16[1];
      v1 = *v16;
      v16 = v3;
    } while (v1 != 10);
    v17 += 1;
  }
  if (((*(char *)((long)a0 + 0x120)) && (dat_2d1c0 != 4)) && (dat_2d1c0 != 5)) {
    v13 = (long *)(v19 + v21 * 8);
    *v13 = *v13 + -1;
  }
label_ed3e:
  *(long *)((long)a0 + 0xb8) = v19;
  *(long *)((long)a0 + 0xd0) = v21;
  *(long *)((long)a0 + 0xd8) = v20;
  *(void **)((long)a0 + 0xf8) = v22;
  dat_2d768 = v23;
  dat_2d778 = v25;
  dat_2d770 = v24;
  return;
}

// Function: sub_edad @ 0xedad
void sub_edad(void *a0) // return-dupe, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_edad
{
  char *v1;
  long v2;
  void *v3;
  int v4; // eax
  char *v5; // rax
  long v6; // stack - 0x30
  char *v7; // stack - 0x28
  char *v8; // stack - 0x20
  
  v6 = *(long *)((long)a0 + 0xb0);
  v3 = *(void **)((long)a0 + 0xa0);
  if (!v3)
    return;
  if (dat_2d2b2) {
    v5 = (char *)(v6 + (long)v3);
    *v5 = '\r';
    v7 = rawmemchr(v3,0xd);
    v8 = v7;
    while (v8 != v5) {
      v4 = !((*v8 != '\r') || (v8[1] != '\n'));
      v1 = &v7[1];
      *v7 = v8[v4];
      v8 = &(&v8[v4])[1];
      v7 = v1;
    }
    v6 -= (long)v5 - (long)v7;
  }
  if ((v6) && (*(char *)((long)v3 + v6 + -1) != '\n')) {
    v2 = v6 + 1;
    *(char *)(v6 + (long)v3) = 10;
    *(char *)((long)a0 + 0x120) = 1;
    v6 = v2;
  }
  memset((void *)(v6 + (long)v3),0,8);
  *(long *)((long)a0 + 0xb0) = v6;
}

// Function: sub_eef5 @ 0xeef5
long sub_eef5(long a0,unsigned long a1,unsigned long a2) // ternary
{
  a1 = (10 <= a0) ? a1 / (unsigned long)(a0 - 1U) : 0x20; // branch-flip
  a2 /= a1;
  if (!a2)
    a2 = 1;
  if (0x787878787878783 <= a2)
    a2 = 0x787878787878782;
  return a2 + 5;
}

// Function: sub_ef6a @ 0xef6a
void sub_ef6a(struct_6 *a0) // ternary, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_ef6a
{
  long *v1;
  unsigned long v10;
  unsigned long v11;
  bool v12; // al
  long *v13; // stack - 0xb8
  long *v14; // stack - 0xb0
  long *v15; // stack - 0xa8
  long *v16; // stack - 0xa0
  long *v17; // stack - 0x98
  void *v18; // stack - 0x90
  long v19; // stack - 0x88
  long *v2;
  unsigned long v20; // stack - 0x80
  unsigned long v21; // stack - 0x78
  long v22; // stack - 0x70
  long *v3;
  unsigned long v4;
  long v5;
  long *v6;
  int v7; // eax
  long *v8;
  long v9;
  
  sub_e09d(a0);
  sub_edad(a0);
  if (a0->field_0x0 != a0->field_0x130) { // branch-flip
    sub_e09d(&a0->field_0x130);
    sub_edad(&a0->field_0x130);
  }
  else {
    a0->field_0x1d0 = a0->field_0xa0;
    a0->field_0x1d8 = a0->field_0xa8;
    a0->field_0x1e0 = a0->field_0xb0;
    a0->field_0x250 = a0->field_0x120;
  }
  v2 = a0->field_0xa0;
  v3 = a0->field_0x1d0;
  v10 = a0->field_0xb0;
  v11 = a0->field_0x1e0;
  if (v2 != v3) { // branch-flip
    v13 = v2;
    v14 = v3;
    if (v11 <= v10) // branch-flip
      *(unsigned char *)(v11 + (long)v3) = ~*(unsigned char *)(v11 + (long)v2);
    else {
      *(unsigned char *)(v10 + (long)v2) = ~*(unsigned char *)(v10 + (long)v3);
    }
    while (*v13 == *v14) {
      v14 = &v14[1];
      v13 = &v13[1];
    }
    v16 = v14;
    for (v15 = v13; (char)*v15 == (char)*v16; v15 = (long *)((long)v15 + 1)) {
      v16 = (long *)((long)v16 + 1);
    }
    if (((dat_2d1c0 != 4) && (dat_2d1c0 != 5)) && ((long *)((long)v3 + (v11 - (unsigned char)a0->field_0x250)) < v16 != (long *)((long)v2 + (v10 - (unsigned char)a0->field_0x120)) < v15)) {
      v15 = (long *)((long)v15 + -1);
      v16 = (long *)((long)v16 + -1);
    }
  }
  else {
    v15 = (long *)((long)v3 + v11);
    v16 = v15;
  }
  v19 = dat_2d1e0;
  while ((v15 != v2 && ((*(char *)((long)v15 + -1) != '\n' || (v9 = v19 + -1, v12 = v19 != 0, v19 = v9, v12))))) {
    v15 = (long *)((long)v15 + -1);
    v16 = (long *)((long)v16 + -1);
  }
  a0->field_0xe0 = (long)v15;
  a0->field_0x210 = v16;
  v8 = (long *)(v10 + (long)v2);
  v16 = (long *)(v11 + (long)v3);
  if (((dat_2d1c0 == 4) || (dat_2d1c0 == 5)) || (v15 = v8, a0->field_0x120 == a0->field_0x250)) {
    v9 = (v11 <= v10) ? v10 - v11 : 0; // branch-flip
    v17 = (long *)(v9 + a0->field_0xe0);
    v1 = v8;
    v6 = v16;
    do {
      v16 = v6;
      v15 = v1;
      if (v15 == v17) goto label_f3ed;
      v1 = (long *)((long)v15 + -1);
      v6 = (long *)((long)v16 + -1);
    } while (*(char *)(long *)((long)v15 + -1) == *(char *)(long *)((long)v16 + -1));
    v17 = v15;
label_f3ed:
    if (((v2 != v15) && (*(char *)((long)v15 + -1) != '\n')) || ((v3 != v16 && (*(char *)((long)v16 + -1) != '\n'))))
      v7 = 1;
    else {
      v7 = 0;
    }
    v19 = dat_2d1e0 + v7;
    while ((v9 = v19 + -1, v19 && (v15 != v8))) {
      do {
        v1 = (long *)((long)v15 + 1);
        v5 = *v15;
        v15 = v1;
        v19 = v9;
      } while ((char)v5 != '\n');
    }
    v16 = (long *)((long)v16 + ((long)v15 - (long)v17));
  }
  a0->field_0xf0 = v15;
  a0->field_0x220 = v16;
  if ((((dat_2d1c8) && (!dat_2d240)) && ((long)dat_2d1d0 <= 0x1ffffffffffffffe)) && (dat_2d1d0 < v10)) {
    v9 = sub_eef5(0,0,(long)v15 - a0->field_0xe0);
    v21 = sub_eef5(0,0,(char *)((long)v2 + (v10 - (long)v15)));
    for (v22 = 1; v22 <= (long)dat_2d1d0; v22 = v22 << 1) {
    }
    if ((long)dat_2d1d0 <= (long)v21)
      v21 = dat_2d1d0;
    v21 += v22 + v9;
  }
  else {
    v22 = 0;
    v21 = sub_eef5(0,0,v10);
  }
  v20 = 0;
  v18 = (void *)sub_1c945(v21 << 3);
  v12 = (((dat_2d1c8 != '\x01') || (v15 != (long *)a0->field_0xe0)) || (v16 != a0->field_0x210));
  v15 = v2;
  if (v12) {
    v8 = (long *)a0->field_0xe0;
    while (v15 != v8) {
      v10 = v20 + 1;
      v20 &= v22 - 1U;
      if (v20 == v21) {
        if (0x7ffffffffffffff <= v21)
          sub_1cf09(); // no-return
        v4 = v21 * 2;
        v18 = (void *)sub_1c991(v18,v21 << 4);
        v21 = v4;
      }
      *(long **)(v20 * 8 + (long)v18) = v15;
      do {
        v1 = (long *)((long)v15 + 1);
        v9 = *v15;
        v15 = v1;
        v20 = v10;
      } while ((char)v9 != '\n');
    }
  }
  if ((v22) && ((long)dat_2d1d0 < (long)v20))
    v10 = dat_2d1d0;
  else {
    v10 = v20;
  }
  v9 = sub_eef5(v20,(long)v15 - (long)v2,(long)v16 - (long)a0->field_0x210);
  v11 = sub_eef5(v20,(long)v15 - (long)v2,(char *)((long)v3 + (v11 - (long)v16)));
  if ((long)dat_2d1d0 <= (long)v11)
    v11 = dat_2d1d0;
  v11 += v10 + v9;
  if (((long)v10 <= (long)v11) && (v11 <= 0xffffffffffffffe)) {
    v9 = sub_1c945(v11 * 8);
    if (v10 != v20) {
      for (v19 = 0; v19 < (long)v10; v19 = v19 + 1) {
        *(unsigned long *)(v9 + v19 * 8) = *(unsigned long *)((long)v18 + (v19 + (v20 - dat_2d1d0) & v22 - 1U) * 8);
      }
      for (v19 = 0; v19 < (long)v10; v19 = v19 + 1) {
        *(unsigned long *)((long)v18 + v19 * 8) = *(unsigned long *)(v9 + v19 * 8);
      }
    }
    for (v19 = 0; v19 < (long)v10; v19 = v19 + 1) {
      *(char **)(v9 + v19 * 8) = (char *)((long)v3 + (*(long *)((long)v18 + v19 * 8) - (long)v2));
    }
    a0->field_0xb8 = (void *)(v10 * 8 + (long)v18);
    a0->field_0x1e8 = v9 + v10 * 8;
    a0->field_0x1f0 = -v10;
    a0->field_0xc0 = a0->field_0x1f0;
    a0->field_0xd8 = v21 - v10;
    a0->field_0x208 = v11 - v10;
    a0->field_0x218 = v20;
    a0->field_0xe8 = a0->field_0x218;
    return;
  }
  sub_1cf09(); // no-return
}

// Function: sub_fa13 @ 0xfa13
unsigned long sub_fa13(int *a0,bool a1) // return-dupe
{
  bool v1;
  bool v2;
  long v3; // rax
  bool v4;
  int v5; // stack - 0xc
  
  v1 = dat_2d1d8 != '\0';
  v2 = sub_df00(a0,v1 || a1) != '\0';
  v4 = v2 || a1;
  if (*a0 != a0[0x4c]) // branch-flip
    v4 = sub_df00((struct_4 *)&a0[0x4c],a1 || (v1 || v2)) || v4;
  else {
    *(unsigned long *)&a0[0x74] = *(unsigned long *)&a0[0x28];
    *(unsigned long *)&a0[0x76] = *(unsigned long *)&a0[0x2a];
    *(unsigned long *)&a0[0x78] = *(unsigned long *)&a0[0x2c];
  }
  if (v4) {
    sub_131e6(*a0,0);
    sub_131e6(a0[0x4c],0);
    return 1;
  }
  sub_ef6a(a0);
  dat_2d778 = *(long *)&a0[0x82] + *(long *)&a0[0x36] + 1;
  if (0x3ffffffffffffff <= dat_2d778)
    sub_1cf09(); // no-return
  dat_2d768 = (void *)sub_1c945(dat_2d778 * 0x20);
  dat_2d770 = 1;
  for (v5 = 9; (unsigned long)(1L << ((unsigned char)v5 & 0x3f)) < (unsigned long)((long)dat_2d778 / 3); v5 = v5 + 1) {
  }
  dat_2d760 = (1L << ((unsigned char)v5 & 0x3f)) - (unsigned long)*(unsigned char *)((long)v5 + 0x25240);
  if (dat_2d760 <= 0xffffffffffffffe) {
    v3 = dat_2d760 + 1;
    dat_2d758 = sub_13030(v3 * 8) + 8;
    for (v5 = 0; v5 <= 1; v5 = v5 + 1) {
      sub_e2f8(&a0[(long)v5 * 0x4c]);
    }
    *(unsigned long *)&a0[0x96] = dat_2d770;
    *(unsigned long *)&a0[0x4a] = *(unsigned long *)&a0[0x96];
    free(dat_2d768);
    free((void *)(dat_2d758 + -8));
    return 0;
  }
  sub_1cf09(); // no-return
}

// Function: sub_fcfd @ 0xfcfd
void sub_fcfd(long a0)
{
  sub_1245b(a0,sub_12437,sub_fd2d);
}

// Function: sub_fd2d @ 0xfd2d
void sub_fd2d(long *a0) // return-dupe x2
{
  unsigned int v1; // eax
  long v2;
  long v3; // stack - 0x28
  long v4; // stack - 0x30
  long v5; // stack - 0x38
  long v6; // stack - 0x40
  
  v1 = sub_12af4(a0,&v6,&v5,&v4,&v3);
  if (!v1)
    return;
  sub_11b50();
  sub_128c2(4);
  sub_12a37(0x2c,(struct_20 *)0x2d360,v6,v5);
  fputc_unlocked((int)*(char *)((unsigned long)v1 + 0x25780),dat_2d5c0);
  sub_12a37(0x2c,(struct_20 *)0x2d490,v4,v3);
  sub_128c2(3);
  fputc_unlocked(10,dat_2d5c0);
  if (v1 & 1) {
    for (v2 = v6; v2 <= v5; v2 = v2 + 1) {
      sub_128c2(2);
      sub_124fe("<",(struct_21 *)(dat_2d418 + v2 * 8),1);
      sub_128c2(3);
      if (*(char *)(*(long *)((v2 + 1) * 8 + dat_2d418) + -1) == '\n')
        putc_unlocked(10,dat_2d5c0);
    }
  }
  if (v1 == 3)
    fputs_unlocked("---\n",dat_2d5c0);
  if (!(v1 & 2))
    return;
  for (v2 = v4; v2 <= v3; v2 = v2 + 1) {
    sub_128c2(1);
    sub_124fe(">",(struct_21 *)(dat_2d548 + v2 * 8),1);
    sub_128c2(3);
    if (*(char *)(*(long *)((v2 + 1) * 8 + dat_2d548) + -1) == '\n')
      putc_unlocked(10,dat_2d5c0);
  }
}

// Function: sub_ff70 @ 0xff70
void sub_ff70(long a0)
{
  sub_11b50();
  dat_2d780 = -dat_2d448;
  dat_2d788 = dat_2d780;
  sub_1245b(a0,sub_12437,sub_106e7);
  sub_1055a(dat_2d430,dat_2d560);
}

// Function: sub_ffdd @ 0xffdd
unsigned long sub_ffdd(unsigned long a0,unsigned long a1)
{
  unsigned long v1;
  FILE *v2;
  unsigned long v3; // stack - 0x30
  unsigned long v4; // stack - 0x20
  
  v2 = dat_2d5c0;
  v1 = dat_2d2a8;
  v3 = a0;
  if (dat_2d2a1 != '\x01') {
    v3 = a0;
    for (v4 = (a0 + dat_2d2a8) - a0 % dat_2d2a8; v4 <= a1; v4 = v4 + v1) {
      putc_unlocked(9,v2);
      v3 = v4;
    }
  }
  while (v1 = v3 + 1, v3 < a1) {
    putc_unlocked(0x20,v2);
    v3 = v1;
  }
  return a1;
}

// Function: sub_10090 @ 0x10090
unsigned long sub_10090(struct_19 *a0,unsigned long a1,unsigned long a2)
{
  char *v1;
  unsigned long v10;
  int v11; // stack - 0x74
  unsigned long v12; // stack - 0x70
  FILE *v13; // stack - 0x68
  char *v14; // stack - 0x60
  long v15; // stack - 0x58
  unsigned long v16; // stack - 0x50
  char v2;
  char *v3;
  bool v4;
  int v5; // eax
  unsigned long v6;
  unsigned long v7;
  unsigned long v8; // stack - 0x48
  int v9; // stack - 0x78
  
  v13 = dat_2d5c0;
  v3 = a0->field_0x8;
  v8 = 0;
  v7 = 0;
  v10 = 0;
  v1 = a0->field_0x0;
  do {
    while( true ) {
      v14 = v1;
      if (v3 <= v14)
        return v10;
      v1 = &v14[1];
      v2 = *v14;
      v5 = (int)v2;
      if (v5 <= 0x5f) break;
      if (0x1e <= (unsigned int)(v5 - 0x61U)) { // branch-flip
label_10288:
        v16 = sub_1d369(&v9,v14,(long)v3 - (long)v14,&v8);
        if ((v16) && (v16 <= 0xfffffffffffffffd)) {
          v11 = wcwidth(v9);
          if (1 <= v11)
            v7 += (long)v11;
          if (v7 <= a2) {
            fwrite_unlocked(v14,1,v16,stdout);
            v10 = v7;
          }
          v1 = &v14[v16];
        }
        else {
label_10310:
          if (v7 < a2)
            putc_unlocked((int)v2,v13);
        }
      }
      else {
label_1032d:
        v6 = v7 + 1;
        v4 = v7 < a2;
        v7 = v6;
        if (v4) {
          putc_unlocked((int)v2,v13);
          v10 = v6;
        }
      }
    }
    if (v5 <= 7) goto label_10288;
    switch(v5) {
      case 8:
        if ((v7) && (v7 = v7 - 1, v7 < a2)) {
          if (v10 <= v7) { // branch-flip
            for (; v10 < v7; v10 = v10 + 1) {
              putc_unlocked(0x20,v13);
            }
          }
          else {
            putc_unlocked((int)v2,v13);
            v10 = v7;
          }
        }
        break;
      case 9:
        v15 = dat_2d2a8 - v7 % dat_2d2a8;
        if (v7 == v10) {
          v6 = v15 + v10;
          v12 = v6;
          if (dat_2d2a1) { // branch-flip
            if (a2 < v6)
              v12 = a2;
            for (; v10 < v12; v10 = v10 + 1) {
              putc_unlocked(0x20,v13);
            }
          }
          else if (v6 < a2) {
            putc_unlocked((int)v2,v13);
            v10 = v6;
          }
        }
        v7 = v15 + v7;
        break;
      case 10:
        return v10;
      case 0xb:
      case 0xc:
        goto label_10310;
      case 0xd:
        putc_unlocked((int)v2,v13);
        sub_ffdd(0,a1);
        v7 = 0;
        v10 = 0;
        break;
      default:
        goto label_10288;
      case 0x20:
      case 0x21:
      case 0x22:
      case 0x23:
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
      case 0x38:
      case 0x39:
      case 0x3a:
      case 0x3b:
      case 0x3c:
      case 0x3d:
      case 0x3e:
      case 0x3f:
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
      case 0x5b:
      case 0x5c:
      case 0x5d:
      case 0x5e:
      case 0x5f:
        goto label_1032d;
      
    }
  } while( true );
}

// Function: sub_10388 @ 0x10388
void sub_10388(struct_19 *a0,char a1,struct_19 *a2) // return-dupe, ternary
{
  bool v1;
  unsigned long v2;
  unsigned long v3;
  FILE *v4;
  char v5; // stack - 0x44
  bool v6; // stack - 0x2a
  unsigned long v7; // stack - 0x28
  
  v4 = dat_2d5c0;
  v3 = dat_2d328;
  v2 = dat_2d320;
  v7 = 0;
  v6 = 0;
  v1 = 0;
  if (a1 != '<') { // branch-flip
    if (a1 == '>') {
      sub_128c2(1);
      v1 = 1;
    }
  }
  else {
    sub_128c2(2);
    v1 = 1;
  }
  if (a0) {
    v6 = a0->field_0x8[-1] == '\n';
    v7 = sub_10090(a0,0,v2);
  }
  if (a1 != ' ') {
    v7 = sub_ffdd(v7,(unsigned long)((v3 + v2) - 1U) >> 1) + 1;
    v5 = a1;
    if ((a1 == '|') && (v6 != (a2->field_0x8[-1] == '\n'))) {
      v5 = (v6) ? '/' : '\\'; // branch-flip
    }
    putc_unlocked((int)v5,v4);
  }
  if ((a2) && (v6 = a2->field_0x8[-1] == '\n' || v6, *a2->field_0x0 != '\n'))
    sub_10090(a2,sub_ffdd(v7,v3),v2);
  if (v6)
    putc_unlocked(10,v4);
  if (v1) {
    sub_128c2(3);
    return;
  }
}

// Function: sub_1055a @ 0x1055a
void sub_1055a(long a0,long a1)
{
  long v1;
  long v2;
  long v3; // stack - 0x28
  long v4; // stack - 0x20
  
  v3 = dat_2d780;
  v4 = dat_2d788;
  if ((dat_2d31a != '\x01') && ((dat_2d780 != a0 || (dat_2d788 != a1)))) {
    if (dat_2d318)
      fprintf(dat_2d5c0,"i%ld,%ld\n",a0 - dat_2d780,a1 - dat_2d788);
    if (dat_2d319 != '\x01') {
      while ((v3 != a0 && (v4 != a1))) {
        v2 = v4 + 1;
        v1 = v3 + 1;
        sub_10388((struct_19 *)(v3 * 8 + dat_2d418),0x20,(struct_19 *)(dat_2d548 + v4 * 8));
        v3 = v1;
        v4 = v2;
      }
      while (v4 != a1) {
        v2 = v4 + 1;
        sub_10388(NULL,0x29,(struct_19 *)(v4 * 8 + dat_2d548));
        v4 = v2;
      }
    }
    while (v3 != a0) {
      v2 = v3 + 1;
      sub_10388((struct_19 *)(v3 * 8 + dat_2d418),0x28,NULL);
      v3 = v2;
    }
  }
  dat_2d780 = a0;
  dat_2d788 = a1;
}

// Function: sub_106e7 @ 0x106e7
void sub_106e7(long *a0) // return-dupe x2, ternary
{
  int v1; // eax
  long v10; // stack - 0x28
  long v2;
  long v3; // stack - 0x38
  long v4; // stack - 0x40
  long v5; // stack - 0x48
  long v6; // stack - 0x50
  long v7;
  unsigned int v8; // stack - 0x54
  long v9; // stack - 0x30
  
  v8 = sub_12af4(a0,&v6,&v5,&v4,&v3);
  if (!v8)
    return;
  sub_1055a(v6,v4);
  if (dat_2d318) {
    v9 = (v5 - v6) + 1;
    v10 = (v3 - v4) + 1;
    fprintf(dat_2d5c0,"c%ld,%ld\n",v9,v10);
  }
  if (v8 == 3) {
    v2 = v6;
    for (v7 = v4; (v2 <= v5 && (v7 <= v3)); v7 = v7 + 1) {
      sub_10388((struct_19 *)(dat_2d418 + v2 * 8),0x7c,(struct_19 *)(v7 * 8 + dat_2d548));
      v2 += 1;
    }
    v1 = (v7 <= v3) ? 2 : 0; // branch-flip
    v8 = v1 + (unsigned int)(v2 <= v5);
    dat_2d780 = v2;
    dat_2d788 = v7;
    v6 = v2;
    v4 = v7;
  }
  if (v8 & 2) {
    for (v2 = v4; v2 <= v3; v2 = v2 + 1) {
      sub_10388(NULL,0x3e,(struct_19 *)(dat_2d548 + v2 * 8));
    }
    dat_2d788 = v2;
  }
  if (!(v8 & 1))
    return;
  for (v2 = v6; v2 <= v5; v2 = v2 + 1) {
    sub_10388((struct_19 *)(dat_2d418 + v2 * 8),0x3c,NULL);
  }
  dat_2d780 = v2;
}

// Function: sub_108ec @ 0x108ec
void sub_108ec(unsigned long a0)
{
  error(0,*__errno_location(),"%s",a0); // no-return
}

// Function: sub_10925 @ 0x10925
void sub_10925(char *a0)
{
  int v1;
  
  v1 = *__errno_location();
  sub_10bca();
  error(2,v1,"%s",a0); // no-return
}

// Function: sub_10966 @ 0x10966
void sub_10966(char *a0)
{
  sub_10bca();
  error(2,0,"%s",gettext(a0)); // no-return
}

// Function: sub_109a8 @ 0x109a8
void sub_109a8(char *a0,unsigned long a1,unsigned long a2)
{
  sub_109e2(a0,a1,a2,0,0);
}

// Function: sub_109e2 @ 0x109e2
void sub_109e2(char *a0,unsigned long a1,unsigned long a2,long a3,long a4) // return-dupe
{
  char *v1;
  unsigned long *v2; // rax
  char *v3 [3]; // stack - 0x68
  unsigned long v4 [5];
  int v5; // stack - 0x84
  unsigned long *v6; // stack - 0x80
  unsigned long v7; // stack - 0x78
  long v8; // stack - 0x50
  long v9; // stack - 0x48
  
  if (!dat_2d2c0) {
    if (dat_2d318)
      putchar_unlocked(0x20);
    printf(gettext(a0),a1,a2,a3,a4);
    return;
  }
  v7 = 8;
  v3[0] = a0;
  v3[1] = (char *)a1;
  v3[2] = (char *)a2;
  if (!a3)
    a3 = 0x2540f;
  v8 = a3;
  if (!a4)
    a4 = 0x2540f;
  v9 = a4;
  for (v5 = 0; v5 <= 4; v5 = v5 + 1) {
    v1 = v3[v5];
    v4[v5] = strlen(v1) + 1;
    v7 += v4[v5];
  }
  v2 = (unsigned long *)sub_1c945(v7);
  v6 = &v2[1];
  for (v5 = 0; v5 <= 4; v5 = v5 + 1) {
    memcpy(v6,v3[v5],v4[v5]);
    v6 = (unsigned long *)((long)v6 + v4[v5]);
  }
  *dat_2d020 = v2;
  *v2 = 0;
  dat_2d020 = v2;
}

// Function: sub_10bca @ 0x10bca
void sub_10bca(void)
{
  unsigned long *v1;
  long v2;
  char *v3;
  long v4;
  long v5;
  long v6;
  long v7 [5]; // stack - 0x58
  int v8; // stack - 0x6c
  unsigned long *v9; // stack - 0x68
  
  v9 = dat_2d7a0;
  while (v9) {
    v1 = (unsigned long *)*v9;
    for (v8 = 0; v6 = v7[4], v5 = v7[3], v4 = v7[2], v2 = v7[1], v8 <= 3; v8 = v8 + 1) {
      v2 = v7[v8];
      v3 = (char *)v7[v8];
      v7[v8 + 1] = strlen(v3) + 1 + v2;
    }
    printf(gettext((char *)&v9[1]),v2,v4,v5,v6);
    free(v9);
    v9 = v1;
  }
}

// Function: sub_10ccc @ 0x10ccc
void sub_10ccc(unsigned int a0) // return-dupe
{
  if (dat_2d840)
    return;
  dat_2d840 = a0;
}

// Function: sub_10ced @ 0x10ced
void sub_10ced(void) // return-dupe
{
  if (dat_2d840)
    return;
  dat_2d844 += 1;
}

// Function: sub_10d14 @ 0x10d14
void sub_10d14(void)
{
  sigset_t v1;
  int v2; // stack - 0xa0
  
  while ((dat_2d840 || (dat_2d844))) {
    sub_128c2(3);
    fflush_unlocked(stdout);
    sigprocmask(0,(sigset_t *)0x2d7c0,&v1);
    v2 = dat_2d840;
    if (dat_2d844) { // branch-flip
      dat_2d844 -= 1;
      v2 = 0x13;
    }
    else {
      signal(dat_2d840,0);
    }
    raise(v2);
    sigprocmask(2,&v1,NULL);
  }
}

// Function: sub_10e15 @ 0x10e15
void sub_10e15(void) // ternary
{
  int v1;
  void *v2 [17]; // stack - 0xa8
  int v3; // stack - 0xac
  undefined4 v4; // stack - 0x20
  
  sigemptyset((sigset_t *)0x2d7c0);
  for (v3 = 0; v3 <= 0xb; v3 = v3 + 1) {
    sigaction(*(int *)((long)v3 * 4 + 0x257c0),NULL,(sigaction *)v2);
    if (v2[0] != (void *)0x1)
      sigaddset((sigset_t *)0x2d7c0,*(int *)((long)v3 * 4 + 0x257c0));
  }
  v2[1] = (void *)dat_2d7c0;
  v2[2] = (void *)dat_2d7c8;
  v2[3] = (void *)dat_2d7d0;
  v2[4] = (void *)dat_2d7d8;
  v2[5] = (void *)dat_2d7e0;
  v2[6] = (void *)dat_2d7e8;
  v2[7] = (void *)dat_2d7f0;
  v2[8] = (void *)dat_2d7f8;
  v2[9] = (void *)dat_2d800;
  v2[10] = (void *)dat_2d808;
  v2[0xb] = (void *)dat_2d810;
  v2[0xc] = (void *)dat_2d818;
  v2[0xd] = (void *)dat_2d820;
  v2[0xe] = (void *)dat_2d828;
  v2[0xf] = (void *)dat_2d830;
  v2[0x10] = (void *)dat_2d838;
  v4 = 0x10000000;
  for (v3 = 0; v3 <= 0xb; v3 = v3 + 1) {
    v1 = *(int *)((long)v3 * 4 + 0x257c0);
    if (sigismember((void *)0x2d7c0,v1)) {
      v2[0] = (*(int *)((long)v3 * 4 + 0x257c0) != 0x14) ? sub_10ccc : sub_10ced; // branch-flip
      sigaction(*(int *)((long)v3 * 4 + 0x257c0),(sigaction *)v2,NULL);
    }
  }
}

// Function: sub_11052 @ 0x11052
unsigned long sub_11052(void)
{
  unsigned long v1; // rax
  
  v1 = (*(void *)((long)dat_25410 + 0x25410))(); // jump-as-call
  return v1;
}

// Function: sub_1140f @ 0x1140f
void sub_1140f(unsigned long a0)
{
  dat_2d868 = a0;
}

// Function: sub_11429 @ 0x11429
void sub_11429(void)
{
  char *v1;
  char v2 [3];
  long v3; // fs_offset
  unsigned long v4; // stack - 0x10
  
  v1 = dat_2d868;
  v4 = *(unsigned long *)(v3 + 0x28);
  if ((dat_2d868) && (*dat_2d868)) {
    strcpy(v2,"??");
    sub_1ced8(v1);
    (*(void *)((long)dat_255b4 + 0x255b0))(); // jump-as-call
    return;
  }
}

// Function: sub_117e9 @ 0x117e9
void sub_117e9(char a0) // return-dupe, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_117e9
{
  bool v1; // al
  int v2; // eax
  
  if (!dat_2d5c0)
    return;
  if (!dat_2d1c4)
    return;
  if (dat_2d5c8) { // branch-flip
label_11848:
    v1 = 1;
  }
  else {
    if (a0 != '\x01') {
      v2 = fileno(dat_2d5c0);
      if (isatty(v2)) goto label_11848;
    }
    v1 = 0;
  }
  dat_2d859 = ((dat_2d1c4 == 2) || ((dat_2d1c4 == 1 && (v1))));
  if (dat_2d859)
    sub_11429();
  if (!v1)
    return;
  sub_10e15();
}

// Function: sub_118ac @ 0x118ac
void sub_118ac(unsigned long a0,unsigned long a1,char a2)
{
  dat_2d848 = a0;
  dat_2d850 = a1;
  dat_2d858 = a2;
  dat_2d5c0 = 0;
}

// Function: sub_118ef @ 0x118ef
unsigned long sub_118ef(char a0) // early-return x9
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  
  v1 = (unsigned int)a0;
  v2 = (unsigned long)v1;
  if (0x23 <= (int)v1) { // branch-flip
    if (v1 == 0x5c)
      return 0x5c;
  }
  else if (7 <= (int)v1) {
    v1 -= 7;
    v2 = (unsigned long)v1;
    if (v1 <= 0x1b) {
      v2 = (long)*(int *)((unsigned long)v1 * 4 + 0x255cc) + 0x255cc;
      switch(v1) {
        case 0:
          return 0x61;
        case 1:
          return 0x62;
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
        case 0x1b:
          return 0x22;
        
      }
    }
  }
  return CONCAT71((undefined7)(v2 >> 8),a0 <= '\x1f');
}

// Function: sub_11984 @ 0x11984
unsigned char * sub_11984(unsigned char *a0) // ternary
{
  unsigned char *v1;
  unsigned char v2;
  bool v3;
  char v4; // al
  unsigned char v5; // al
  unsigned char *v6; // rax
  unsigned char *v7; // stack - 0x30
  long v8; // stack - 0x28
  unsigned char *v9; // stack - 0x20
  
  v8 = 0;
  v3 = 0;
  for (v7 = a0; *v7; v7 = &v7[1]) {
    if (*v7 != 0x20) { // branch-flip
      v4 = sub_118ef((int)(char)*v7);
      if (!v4) goto label_119ec;
      v8 = (v4 != '\x01') ? v8 + 1 : v8 + 3; // branch-flip
    }
    else {
      v3 = 1;
    }
label_119ec:
  }
  if ((v3) || (v8)) {
    v6 = (unsigned char *)sub_1c945(&v7[(v8 - (long)a0) + 3]);
    v9 = &v6[1];
    *v6 = 0x22;
    for (v7 = a0; *v7; v7 = &v7[1]) {
      v2 = *v7;
      v5 = sub_118ef((int)(char)v2);
      if (v5) { // branch-flip
        if (v5 != 1) { // branch-flip
          *v9 = 0x5c;
          v1 = &v9[2];
          v9[1] = v5;
          v9 = v1;
        }
        else {
          *v9 = 0x5c;
          v9[1] = (v2 >> 6) + 0x30;
          v9[2] = ((char)v2 >> 3 & 7U) + 0x30;
          v1 = &v9[4];
          v9[3] = (v2 & 7) + 0x30;
          v9 = v1;
        }
      }
      else {
        v1 = &v9[1];
        *v9 = v2;
        v9 = v1;
      }
    }
    *v9 = 0x22;
    v9[1] = 0;
    a0 = v6;
  }
  return a0;
}

// Function: sub_11b50 @ 0x11b50
void sub_11b50(void) // return-dupe x2, ternary
{
  int v1; // eax
  unsigned long v10; // stack - 0x20
  char *v2; // rax
  int v3; // stack - 0x50
  char *v4; // stack - 0x38
  void *v5; // stack - 0x48
  int v6; // stack - 0x4c
  void *v7; // stack - 0x40
  char *v8; // stack - 0x30
  char *v9; // stack - 0x28
  
  if (dat_2d5c0)
    return;
  v5 = (void *)sub_11984(dat_2d848);
  v7 = (void *)sub_11984(dat_2d850);
  v2 = (char *)sub_1d0f7("diff%s %s %s",dat_2d330,v5,v7);
  if (dat_2d2c0) { // branch-flip
    if (fflush_unlocked(stdout))
      sub_10925(gettext("write failed")); // no-return
    v4 = "/usr/bin/pr";
    v8 = "-h";
    v9 = v2;
    v10 = 0;
    if (pipe(&v3))
      sub_10925("pipe"); // no-return
    dat_2d870 = fork();
    if (dat_2d870 <= -1)
      sub_10925("fork"); // no-return
    if (!dat_2d870) {
      close(v6);
      if (v3) {
        if (dup2(v3,0) <= -1)
          sub_10925("dup2"); // no-return
        close(v3);
      }
      execv("/usr/bin/pr",&v4);
      v1 = (*__errno_location() != 2) ? 0x7e : 0x7f; // branch-flip
      _exit(v1); // no-return
    }
    close(v3);
    dat_2d5c0 = fdopen(v6,"w");
    if (!dat_2d5c0)
      sub_10925("fdopen"); // no-return
    sub_117e9(1);
  }
  else {
    dat_2d5c0 = stdout;
    sub_117e9(0);
    if (dat_2d858)
      printf("%s\n",v2);
  }
  free(v2);
  if (dat_2d1c0 != 2) { // branch-flip
    if (dat_2d1c0 == 3)
      sub_7e5c(0x2d360,&v5,1);
  }
  else {
    sub_7e5c(0x2d360,&v5,0);
  }
  if (v5 != dat_2d848)
    free(v5);
  if (v7 == dat_2d850)
    return;
  free(v7);
}

// Function: sub_11e1b @ 0x11e1b
void sub_11e1b(void) // ternary
{
  char *v1; // rax
  unsigned int v2; // stack - 0x1c
  int v3; // stack - 0x18
  unsigned int v4; // stack - 0x14
  
  if ((dat_2d5c0) && (dat_2d5c0 != stdout)) {
    v3 = 0;
    if (ferror_unlocked(dat_2d5c0))
      sub_10966("write failed"); // no-return
    if (fclose(dat_2d5c0))
      sub_10925(gettext("write failed")); // no-return
    if (waitpid(dat_2d870,(int *)&v2,0) <= -1)
      sub_10925("waitpid"); // no-return
    if ((v3) || (v2 & 0x7f))
      v4 = 0x7fffffff;
    else {
      v4 = (int)v2 >> 8 & 0xff;
    }
    if (v4) {
      if (v4 != 0x7e) { // branch-flip
        if (v4 != 0x7f) { // branch-flip
          v1 = (v4 != 0x7fffffff) ? "subsidiary program \'%s\' failed (exit status %d)" : "subsidiary program \'%s\' failed"; // branch-flip
        }
        else {
          v1 = "subsidiary program \'%s\' not found";
        }
      }
      else {
        v1 = "subsidiary program \'%s\' could not be invoked";
      }
      v1 = gettext(v1);
      error(2,v3,v1,"/usr/bin/pr",(unsigned long)v4); // no-return
    }
  }
  dat_2d5c0 = NULL;
}

// Function: sub_11f90 @ 0x11f90
unsigned long sub_11f90(unsigned char *a0,unsigned char *a1) // return-dupe x3, ternary
{
  unsigned int v1; // eax
  unsigned char *v10;
  unsigned char *v11;
  unsigned long v12; // stack - 0x48
  unsigned char *v13; // stack - 0x40
  unsigned char *v14; // stack - 0x38
  unsigned long v15; // stack - 0x30
  unsigned int v2; // eax
  long v3; // rax
  char v4; // bl
  unsigned char v5;
  char v6; // r12b
  unsigned char v7;
  unsigned char *v8;
  unsigned char *v9;
  
  v12 = 0;
  v8 = a0;
  v10 = a1;
label_11fbb:
  v9 = &v8[1];
  v5 = *v8;
  v1 = (unsigned int)v5;
  v11 = &v10[1];
  v7 = *v10;
  v2 = (unsigned int)v7;
  if (v5 == v7) goto label_123e3;
  switch(dat_2d1e8) {
    case 0:
      goto label_123b9;
    case 1:
label_122e2:
      if (((v5 == 0x20) && (v7 == 9)) || ((v5 == 9 && (v7 == 0x20)))) {
        v15 = v12;
        do {
          if ((char)v1 != ' ') { // branch-flip
            if ((char)v1 != '\t') goto label_1234b;
            v12 += dat_2d2a8 - v12 % dat_2d2a8;
          }
          else {
            v12 += 1;
          }
          v8 = &v9[1];
          v1 = (unsigned int)*v9;
          v9 = v8;
        } while( true );
      }
      break;
    case 2:
    case 3:
      if ((*(unsigned short *)((unsigned long)v1 * 2 + *(long *)__ctype_b_loc()) & 0x2000) && (*(unsigned short *)((unsigned long)v2 * 2 + *(long *)__ctype_b_loc()) & 0x2000)) {
        if (v5 != 10) {
          v13 = v9;
          while ((v5 = *v13, v5 != 10 && (*(unsigned short *)((unsigned long)v5 * 2 + *(long *)__ctype_b_loc()) & 0x2000))) {
            v13 = &v13[1];
          }
          if (v5 != 10) goto label_123b9;
        }
        if (v7 == 10)
          return 0;
        v14 = v11;
        while ((v5 = *v14, v5 != 10 && (*(unsigned short *)((unsigned long)v5 * 2 + *(long *)__ctype_b_loc()) & 0x2000))) {
          v14 = &v14[1];
        }
        if (v5 == 10)
          return 0;
      }
      else if (dat_2d1e8 != 2) goto label_122e2;
      break;
    case 4:
      v8 = v9;
      if (*(unsigned short *)((unsigned long)v5 * 2 + *(long *)__ctype_b_loc()) & 0x2000) {
        do {
          v9 = v8;
          if ((char)v1 == '\n') goto label_120dd;
          v1 = (unsigned int)*v9;
          v8 = &v9[1];
        } while (*(unsigned short *)((unsigned long)v1 * 2 + *(long *)__ctype_b_loc()) & 0x2000);
        v1 = 0x20;
      }
label_120dd:
      v8 = v11;
      if (*(unsigned short *)((unsigned long)v7 * 2 + *(long *)__ctype_b_loc()) & 0x2000) {
        do {
          v11 = v8;
          if ((char)v2 == '\n') goto label_12140;
          v2 = (unsigned int)*v11;
          v8 = &v11[1];
        } while (*(unsigned short *)((unsigned long)v2 * 2 + *(long *)__ctype_b_loc()) & 0x2000);
        v2 = 0x20;
      }
label_12140:
      v4 = (char)v1;
      v6 = (char)v2;
      if (v4 == v6) goto label_123b9;
      if ((((v6 != ' ') || (v4 == '\n')) || (v9 <= &a0[1])) || (!(*(unsigned short *)((unsigned long)v9[-2] * 2 + *(long *)__ctype_b_loc()) & 0x2000))) {
        if (((v4 != ' ') || (v6 == '\n')) || ((v11 <= &a1[1] || (!(*(unsigned short *)((unsigned long)v11[-2] * 2 + *(long *)__ctype_b_loc()) & 0x2000))))) goto label_123b9;
        v10 = &v11[-1];
        v8 = v9;
      }
      else {
        v8 = &v9[-1];
        v10 = v11;
      }
      goto label_11fbb;
    case 5:
      while ((*(unsigned short *)((unsigned long)v1 * 2 + *(long *)__ctype_b_loc()) & 0x2000 && ((char)v1 != '\n'))) {
        v8 = &v9[1];
        v1 = (unsigned int)*v9;
        v9 = v8;
      }
      while ((*(unsigned short *)((unsigned long)v2 * 2 + *(long *)__ctype_b_loc()) & 0x2000 && ((char)v2 != '\n'))) {
        v8 = &v11[1];
        v2 = (unsigned int)*v11;
        v11 = v8;
      }
    
  }
label_123b9:
  v5 = (unsigned char)v1;
  v7 = (unsigned char)v2;
  if (dat_2d1ee) {
    v5 = (unsigned char)tolower(v1);
    v7 = (unsigned char)tolower(v2);
  }
  if (v5 != v7)
    return 1;
label_123e3:
  if (v5 == 10)
    return 0;
  v3 = (v5 != 9) ? 1 : dat_2d2a8 - v12 % dat_2d2a8; // branch-flip
  v12 += v3;
  v8 = v9;
  v10 = v11;
  goto label_11fbb;
label_1234b:
  if ((char)v2 != ' ') { // branch-flip
    if ((char)v2 != '\t') goto label_12392;
    v15 += dat_2d2a8 - v15 % dat_2d2a8;
  }
  else {
    v15 += 1;
  }
  v8 = &v11[1];
  v2 = (unsigned int)*v11;
  v11 = v8;
  goto label_1234b;
label_12392:
  if (v12 != v15)
    return 1;
  goto label_123b9;
}

// Function: sub_12437 @ 0x12437
unsigned long sub_12437(unsigned long a0)
{
  return a0;
}

// Function: sub_12449 @ 0x12449
unsigned long sub_12449(unsigned long a0)
{
  return a0;
}

// Function: sub_1245b @ 0x1245b
void sub_1245b(long a0,void *a1,void *a2)
{
  long v1;
  long *v2; // rax
  long v3; // stack - 0x20
  
  v3 = a0;
  while (v3) {
    v2 = (long *)(*a1)(v3);
    v1 = *v2;
    *v2 = 0;
    (*a2)(v3);
    *v2 = v1;
    v3 = v1;
  }
}

// Function: sub_124cf @ 0x124cf
void sub_124cf(char *a0,struct_21 *a1)
{
  sub_124fe(a0,a1,0);
}

// Function: sub_124fe @ 0x124fe
void sub_124fe(char *a0,struct_21 *a1,bool a2) // ternary, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_124fe
{
  unsigned char *v1;
  long v2;
  FILE *v3;
  int v4; // eax
  char *v5; // stack - 0x38
  char *v6; // stack - 0x30
  char *v7; // stack - 0x28
  
  v3 = dat_2d5c0;
  v1 = a1->field_0x0;
  v2 = a1->field_0x8;
  v5 = NULL;
  if ((a0) && (*a0)) {
    v5 = (dat_2d2b0) ? "%s\t" : "%s "; // branch-flip
    v6 = v5;
    v7 = a0;
    if ((dat_2d2b1) && (*a1->field_0x0 == 10)) {
      v6 = "%s";
      v7 = &a0[*a0 == ' '];
    }
    fprintf(dat_2d5c0,v6,v7);
  }
  v4 = ((a2) && (*(char *)(v2 + -1) == '\n'));
  sub_1267d(v1,(unsigned char *)(v2 - v4),v5,a0);
  if (((!a0) || (*a0)) && (*(char *)(v2 + -1) != '\n')) {
    sub_128c2(3);
    fprintf(v3,"\n\\ %s\n",gettext("No newline at end of file"));
  }
}

// Function: sub_1267d @ 0x1267d
void sub_1267d(unsigned char *a0,unsigned char *a1,char *a2,unsigned long a3) // return-dupe x2
{
  unsigned char *v1;
  unsigned long v10; // stack - 0x48
  unsigned char v2;
  FILE *v3;
  unsigned long v4;
  unsigned long v5; // rax
  unsigned char *v6; // stack - 0x70
  unsigned long v7; // stack - 0x60
  long v8; // stack - 0x58
  long v9; // stack - 0x50
  
  v3 = dat_2d5c0;
  v4 = dat_2d2a8;
  v10 = 0x400;
  if (dat_2d2a1 != '\x01') {
    v7 = (long)a1 - (long)a0;
    v6 = a0;
    while( true ) {
      if (!v7)
        return;
      v4 = v10;
      if (v7 <= 0x400)
        v4 = v7;
      v5 = fwrite_unlocked(v6,1,v4,dat_2d5c0);
      if (v5 < v4) break;
      v6 = &v6[v5];
      v7 -= v5;
      sub_10d14();
    }
    return;
  }
  v5 = 0;
  v8 = 0;
label_1286f:
  do {
    while( true ) {
      while( true ) {
        if (a1 <= a0)
          return;
        v8 += 1;
        if (v8 == 0x400) {
          sub_10d14();
          v8 = 0;
        }
        v1 = &a0[1];
        v2 = *a0;
        a0 = v1;
        if (v2 != 0xd) break;
        putc_unlocked(0xd,v3);
        if (((a2) && (v1 < a1)) && (*v1 != 10))
          fprintf(v3,a2,a3);
        v5 = 0;
      }
      if (v2 <= 0xd) break;
label_12838:
      v5 += (*(unsigned short *)((unsigned long)v2 * 2 + *(long *)__ctype_b_loc()) & 0x4000) != 0;
      putc_unlocked((unsigned int)v2,v3);
    }
    if (v2 != 8) {
      if (v2 != 9) goto label_12838;
      v9 = v4 - v5 % v4;
      v5 = v9 + v5;
      do {
        putc_unlocked(0x20,v3);
        v9 -= 1;
      } while (v9);
      goto label_1286f;
    }
    if (v5) {
      v5 -= 1;
      putc_unlocked(8,v3);
    }
  } while( true );
}

// Function: sub_12889 @ 0x12889
void sub_12889(struct_16 *a0)
{
  fwrite_unlocked(a0->field_0x8,a0->field_0x0,1,dat_2d5c0);
}

// Function: sub_128c2 @ 0x128c2
void sub_128c2(int a0)
{
  if (a0 != 3)
    sub_10d14();
  if ((dat_2d859) && (a0 != dat_2d010)) {
    sub_12889((struct_16 *)0x2d040);
    switch(a0) {
      case 0:
        sub_12889((struct_16 *)0x2d080);
        break;
      case 1:
        sub_12889((struct_16 *)0x2d090);
        break;
      case 2:
        sub_12889((struct_16 *)0x2d0a0);
        break;
      case 3:
        sub_12889((struct_16 *)0x2d070);
        break;
      case 4:
        sub_12889((struct_16 *)0x2d0b0);
        break;
      default:
        abort(); // no-return
      
    }
    sub_12889((struct_16 *)0x2d050);
    dat_2d010 = a0;
  }
}

// Function: sub_129a8 @ 0x129a8
long sub_129a8(struct_20 *a0,long a1)
{
  return a1 + a0->field_0xe8 + 1;
}

// Function: sub_129d0 @ 0x129d0
void sub_129d0(struct_20 *a0,long a1,long a2,long *a3,long *a4)
{
  *a3 = sub_129a8(a0,a1 + -1) + 1;
  *a4 = sub_129a8(a0,a2 + 1) + -1;
}

// Function: sub_12a37 @ 0x12a37
void sub_12a37(char a0,struct_20 *a1,long a2,long a3) // return-dupe
{
  long v1; // stack - 0x18
  long v2; // stack - 0x20
  
  sub_129d0(a1,a2,a3,&v2,&v1);
  if (v2 < v1) {
    fprintf(dat_2d5c0,"%ld%c%ld",v2,a0,v1);
    return;
  }
  fprintf(dat_2d5c0,"%ld",v1);
}

// Function: sub_12af4 @ 0x12af4
unsigned char sub_12af4(long *a0,long *a1,long *a2,long *a3,long *a4) // early-return, ternary, warn: iteboolean: re-rolled 3 0/1 select diamond(s) to a boolean assignment in sub_12af4
{
  unsigned char *v1;
  char *v10; // rax
  unsigned char v11; // dl
  int v12; // edx
  long *v13; // stack - 0xa0
  long v14; // stack - 0x98
  long v15; // stack - 0x90
  long v16; // stack - 0x88
  unsigned char *v17; // stack - 0x80
  unsigned char *v18; // stack - 0x78
  long v2;
  long v3;
  bool v4;
  bool v5; // al
  bool v6; // al
  int v7; // eax
  long v8; // rax
  long v9; // rax
  
  v3 = dat_2d548;
  v2 = dat_2d418;
  v4 = ((dat_2d1ec) || (dat_2d280));
  v7 = (unsigned int)dat_2d1ec;
  v5 = ((dat_2d1ec) && (2 <= dat_2d1e8));
  v6 = ((v5) && (4 <= dat_2d1e8));
  v15 = 0;
  v14 = 0;
  *a1 = a0[3];
  *a3 = a0[4];
  v13 = a0;
  do {
    v8 = v13[2] + v13[3] + -1;
    v9 = v13[1] + v13[4] + -1;
    v14 += v13[2];
    v15 += v13[1];
    v16 = v13[3];
    while ((v16 <= v8 && (v4))) {
      v1 = *(unsigned char **)(v2 + v16 * 8);
      v10 = (char *)(*(long *)(v2 + (v16 + 1) * 8) + -1);
      v10 = &v10[*v10 != '\n'];
      v12 = (int)v10 - (int)v1;
      v17 = v1;
      if (v5) {
        for (; *v17 != 10; v17 = &v17[1]) {
          if (!(*(unsigned short *)((unsigned long)*v17 * 2 + *(long *)__ctype_b_loc()) & 0x2000)) {
            if (!v6)
              v17 = v1;
            break;
          }
        }
      }
      if (((long)v10 - (long)v17 != (long)(int)(v7 - 1)) && ((!dat_2d280 || (re_search((re_pattern_buffer *)0x2d260,(char *)v1,v12,0,v12,NULL) <= -1))))
        v4 = 0;
      v16 += 1;
    }
    v16 = v13[4];
    while ((v16 <= v9 && (v4))) {
      v1 = *(unsigned char **)(v3 + v16 * 8);
      v10 = (char *)(*(long *)(v3 + (v16 + 1) * 8) + -1);
      v10 = &v10[*v10 != '\n'];
      v12 = (int)v10 - (int)v1;
      v18 = v1;
      if (v5) {
        for (; *v18 != 10; v18 = &v18[1]) {
          if (!(*(unsigned short *)((unsigned long)*v18 * 2 + *(long *)__ctype_b_loc()) & 0x2000)) {
            if (!v6)
              v18 = v1;
            break;
          }
        }
      }
      if (((long)v10 - (long)v18 != (long)(int)(v7 - 1)) && ((!dat_2d280 || (re_search((re_pattern_buffer *)0x2d260,(char *)v1,v12,0,v12,NULL) <= -1))))
        v4 = 0;
      v16 += 1;
    }
    v13 = (long *)*v13;
    if (!v13) {
      *a2 = v8;
      *a4 = v9;
      if (v4)
        return 0;
      v11 = (v15) ? 2 : 0; // branch-flip
      return v14 != 0 | v11;
    }
  } while( true );
}

// Function: sub_12fa9 @ 0x12fa9
char * sub_12fa9(char *a0,char *a1,char *a2)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = strlen(a0);
  v2 = strlen(a1);
  v3 = (char *)sub_1c945(strlen(a2) + v1 + v2 + 1);
  sprintf(v3,"%s%s%s",a0,a1,a2);
  return v3;
}

// Function: sub_13030 @ 0x13030
void * sub_13030(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)sub_1c945(a0);
  memset(v1,0,a0);
  return v1;
}

// Function: sub_1306b @ 0x1306b
void sub_1306b(unsigned long *a0)
{
  unsigned long *v1; // stack - 0x30
  
  fflush_unlocked(stdout);
  for (v1 = a0; v1; v1 = (unsigned long *)*v1) {
    fprintf(stderr,"%3ld %3ld delete %ld insert %ld\n",v1[3],v1[4],v1[2],v1[1]);
  }
  fflush_unlocked(stderr);
}

// Function: sub_13111 @ 0x13111
char * sub_13111(char *a0)
{
  bool v1;
  char *v2; // stack - 0x18
  char *v3; // stack - 0x10
  
  v1 = 0;
  for (v2 = a0; *v2 == '/'; v2 = &v2[1]) {
  }
  for (v3 = v2; *v3; v3 = &v3[1]) {
    if (*v3 != '/') { // branch-flip
      if (!v1) goto label_13168;
      v2 = v3;
      v1 = 0;
    }
    else {
      v1 = 1;
    }
label_13168:
  }
  return v2;
}

// Function: sub_1317e @ 0x1317e
unsigned long sub_1317e(char *a0)
{
  unsigned long v1; // stack - 0x18
  
  for (v1 = strlen(a0); (2 <= v1 && (a0[v1 - 1] == '/')); v1 = v1 - 1) {
  }
  return v1;
}

// Function: sub_131d1 @ 0x131d1
unsigned long sub_131d1(void)
{
  return 0;
}

// Function: sub_131e6 @ 0x131e6
void sub_131e6(unsigned int a0,unsigned int a1)
{
  sub_131d1(a0,a1);
}

// Function: sub_13209 @ 0x13209
unsigned long sub_13209(int a0) // return-dupe
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

// Function: sub_13247 @ 0x13247
unsigned long sub_13247(int a0) // return-dupe
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

// Function: sub_13279 @ 0x13279
bool sub_13279(unsigned int a0)
{
  return a0 < 0x80;
}

// Function: sub_1329a @ 0x1329a
unsigned int sub_1329a(int a0) // return-dupe
{
  if ((a0 != 0x20) && (a0 != 9))
    return 0;
  return 1;
}

// Function: sub_132c2 @ 0x132c2
unsigned long sub_132c2(int a0) // return-dupe
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

// Function: sub_132ef @ 0x132ef
bool sub_132ef(int a0)
{
  return (unsigned int)(a0 - 0x30U) < 10;
}

// Function: sub_13313 @ 0x13313
bool sub_13313(int a0)
{
  return (unsigned int)(a0 - 0x21U) < 0x5e;
}

// Function: sub_13337 @ 0x13337
bool sub_13337(int a0)
{
  return (unsigned int)(a0 - 0x61U) < 0x1a;
}

// Function: sub_1335b @ 0x1335b
bool sub_1335b(int a0)
{
  return (unsigned int)(a0 - 0x20U) < 0x5f;
}

// Function: sub_1337f @ 0x1337f
unsigned long sub_1337f(int a0) // return-dupe, return-dupe x3
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

// Function: sub_133c9 @ 0x133c9
unsigned long sub_133c9(int a0) // return-dupe
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

// Function: sub_133f6 @ 0x133f6
bool sub_133f6(int a0)
{
  return (unsigned int)(a0 - 0x41U) < 0x1a;
}

// Function: sub_1341a @ 0x1341a
unsigned long sub_1341a(int a0) // return-dupe
{
  if (((unsigned int)(a0 - 0x30U) <= 0x36) && (0x7e0000007e03ffU >> ((unsigned char)(a0 - 0x30U) & 0x3f) & 1))
    return 1;
  return 0;
}

// Function: sub_13462 @ 0x13462
int sub_13462(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_13485 @ 0x13485
int sub_13485(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_134a8 @ 0x134a8
void sub_134a8(int a0)
{
  char *v1;
  unsigned long v2; // rax
  unsigned long v3; // rax
  void *v4; // rax
  char v5 [4104]; // stack - 0x1018
  long v6; // fs_offset
  char *v7; // stack - 0x1060
  unsigned long v8; // stack - 0x10
  
  v8 = *(unsigned long *)(v6 + 0x28);
  (*dat_3d880)(a0);
  if (a0) // branch-flip
    v1 = dat_3d888;
  else {
    v1 = dat_3d890;
  }
  v2 = strlen(dat_3d898);
  v3 = strlen(v1);
  if (0x1000 <= v3 + v2) { // branch-flip
    write(2,dat_3d898,v2);
    write(2,": (/.*)?",2);
    write(2,v1,v3);
    v5[0] = 10;
    v7 = (char *)0x1;
  }
  else {
    v4 = mempcpy(v5,dat_3d898,v2);
    v7 = mempcpy(mempcpy(v4,": (/.*)?",2),v1,v3);
    *v7 = 10;
    v7 = &v7[1 - (long)v5];
  }
  write(2,v5,(unsigned long)v7);
  if (!a0)
    _exit(dat_2d0c8); // no-return
  raise(a0);
  abort(); // no-return
}

// Function: sub_136b1 @ 0x136b1
void sub_136b1(void)
{
  return;
}

// Function: sub_136bf @ 0x136bf
unsigned long sub_136bf(unsigned long a0,int a1)
{
  if (!a1)
    return 0;
  sub_134a8(0xb); // no-return
}

// Function: sub_136eb @ 0x136eb
void sub_136eb(int a0)
{
  unsigned int v1; // eax
  
  if ((a0) && (!dat_3d8a0))
    v1 = 0xb;
  else {
    v1 = 0;
  }
  sub_134a8(v1); // no-return
}

// Function: sub_13721 @ 0x13721
void * sub_13721(void *a0) // return-dupe
{
  int v1; // eax
  void *v2; // rax
  
  if (a0) // branch-flip
    v2 = a0;
  else {
    v2 = sub_136b1;
  }
  dat_3d880 = v2;
  v2 = (char *)gettext("program error");
  dat_3d888 = (char *)v2;
  v2 = (char *)gettext("stack overflow");
  dat_3d890 = (char *)v2;
  v2 = (unsigned long)sub_14c65();
  dat_3d898 = v2;
  v1 = sub_1abf1(sub_136eb,0x2d880,0x10000);
  if (v1) {
    v2 = (int *)__errno_location();
    *(int *)v2 = 0x5f;
    v2 = (void *)0xffffffff;
    return v2;
  }
  v1 = sub_1ab41(sub_136bf);
  dat_3d8a0 = v1;
  v2 = NULL;
  return v2;
}

// Function: sub_137d4 @ 0x137d4
void sub_137d4(struct_24 *a0,unsigned long a1)
{
  struct_23 *v1; // rax
  
  v1 = (struct_23 *)sub_1c945(0x10);
  v1->field_0x8 = a1;
  v1->field_0x0 = a0->field_0x8;
  a0->field_0x8 = v1;
}

// Function: sub_13820 @ 0x13820
unsigned long sub_13820(char *a0,unsigned int a1) // return-dupe x3, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_13820
{
  char *v1;
  char v2;
  int v3; // eax
  unsigned long v4; // rdx
  char *v5; // stack - 0x10
  
  v5 = a0;
  do {
    while( true ) {
      v1 = &v5[1];
      v2 = *v5;
      v5 = v1;
      if (v2 != '}') break;
label_138d9:
      if (a1 & 0x8000000)
        return 1;
    }
    if (v2 <= '}') {
      if (v2 == '{') goto label_138d9;
      if (v2 <= '{') {
        if (v2 != '\\') { // branch-flip
          if (v2 <= '\\') {
            if (v2 == '[')
              return 1;
            if (v2 <= '[') {
              if (!v2)
                return 0;
              if ((('\0' <= v2) && (v2 <= '@')) && ('!' <= v2)) {
                v4 = 1L << (v2 - 0x21U & 0x3fU);
                if (v4 & 0x80000401) { // branch-flip
                  if ((a1 & 0x20) && (*v1 == '('))
                    return 1;
                }
                else {
                  if (v4 & 0x2180) goto label_138d9;
                  if (v4 & 0x40000200)
                    return 1;
                }
              }
            }
          }
        }
        else if (!(a1 & 0x8000000)) {
          v3 = !((a1 & 2) || (!*v1));
          v5 = &v1[v3];
        }
      }
    }
  } while( true );
}

// Function: sub_1395f @ 0x1395f
void sub_1395f(char *a0) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_1395f
{
  char *v1;
  char *v2;
  char v3;
  int v4; // eax
  char *v5; // stack - 0x20
  char *v6; // stack - 0x10
  
  v5 = a0;
  v6 = a0;
  do {
    v4 = !((*v6 != '\\') || (!v6[1]));
    v2 = &v6[v4];
    v6 = &v2[1];
    v1 = &v5[1];
    *v5 = *v2;
    v3 = *v5;
    v5 = v1;
  } while (v3);
}

// Function: sub_139c7 @ 0x139c7
void sub_139c7(void)
{
  sub_1cd69(0x10);
}

// Function: sub_139db @ 0x139db
void sub_139db(unsigned char *a0,unsigned long a1)
{
  sub_1535a(a0,a1);
}

// Function: sub_13a0c @ 0x13a0c
unsigned long sub_13a0c(char *a0,unsigned long a1)
{
  bool v1;
  unsigned int v10; // stack - 0x34
  char v2 [8];
  char v3 [4]; // stack - 0x58
  unsigned int v4; // stack - 0x9c
  unsigned long v5; // stack - 0x98
  char v6; // stack - 0x4c
  char *v7; // stack - 0x48
  long v8; // stack - 0x40
  unsigned int v9; // stack - 0x38
  
  v5 = 0;
  v3[0] = 0;
  v7 = a0;
  memset(v2,0,8);
  while( true ) {
    v6 = 0;
    sub_16d35(v3);
    if ((v9 != '\x01') || (v10))
      v1 = 1;
    else {
      v1 = 0;
    }
    if (!v1) break;
    if (v9) // branch-flip
      v4 = towlower(v10);
    else {
      v4 = (unsigned int)*v7;
    }
    v5 = (long)(int)v4 + v5 * 0x1f;
    v7 = &v7[v8];
  }
  return v5 % a1;
}

// Function: sub_13b74 @ 0x13b74
unsigned long sub_13b74(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = strcmp(a0,a1);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}

// Function: sub_13bb2 @ 0x13bb2
unsigned long sub_13bb2(unsigned char *a0,unsigned char *a1)
{
  unsigned long v1; // rax
  
  v1 = sub_1694e(a0,a1);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),(int)v1 == 0);
}

// Function: sub_13bf0 @ 0x13bf0
void sub_13bf0(void *a0)
{
  free(a0);
}

// Function: sub_13c0f @ 0x13c0f
void sub_13c0f(unsigned long *a0,int a1,unsigned int a2) // ternary x2
{
  struct_26 *v1; // rax
  void *v2; // rax
  void *v3; // rdx
  
  v1 = (struct_26 *)sub_1cd69(0x28);
  v1->field_0x8 = a1;
  v1->field_0xc = a2;
  if (!a1) {
    v3 = (a2 & 0x10) ? sub_13bb2 : sub_13b74; // branch-flip
    v2 = (a2 & 0x10) ? sub_13a0c : sub_139db; // branch-flip
    v1->field_0x10 = sub_15742(0,NULL,v2,v3,sub_13bf0);
  }
  v1->field_0x0 = *a0;
  *a0 = v1;
}

// Function: sub_13cc8 @ 0x13cc8
void sub_13cc8(void *a0)
{
  long v1; // stack - 0x10
  
  if (*(int *)((long)a0 + 8)) { // branch-flip
    if (*(int *)((long)a0 + 8) == 1) {
      for (v1 = 0; v1 < *(long *)((long)a0 + 0x20); v1 = v1 + 1) {
        if (*(unsigned int *)(v1 * 0x48 + *(long *)((long)a0 + 0x10)) & 0x8000000)
          regfree((re_pattern_buffer *)(v1 * 0x48 + *(long *)((long)a0 + 0x10) + 8));
      }
      free(*(void **)((long)a0 + 0x10));
    }
  }
  else {
    sub_159c2(*(unsigned long *)((long)a0 + 0x10));
  }
  free(a0);
}

// Function: sub_13d90 @ 0x13d90
void sub_13d90(struct_28 *a0)
{
  unsigned long *v1;
  unsigned long *v2; // stack - 0x28
  unsigned long *v3; // stack - 0x20
  
  v2 = a0->field_0x0;
  while (v2) {
    v1 = (unsigned long *)*v2;
    sub_13cc8(v2);
    v2 = v1;
  }
  v3 = a0->field_0x8;
  while (v3) {
    v1 = (unsigned long *)*v3;
    free((void *)v3[1]);
    free(v3);
    v3 = v1;
  }
  free(a0);
}

// Function: sub_13e26 @ 0x13e26
unsigned long sub_13e26(unsigned char *a0,unsigned char *a1,unsigned int a2)
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  unsigned char *v3; // rax
  char *v4; // rax
  unsigned int v5; // stack - 0x28
  unsigned char *v6; // stack - 0x20
  
  if (a2 & 8) { // branch-flip
    if (a2 & 0x10) { // branch-flip
      v3 = (unsigned char *)sub_1ced8(a1);
      v6 = v3;
      while( true ) {
        v4 = strchr((char *)v6,0x2f);
        if (v4)
          *v4 = '\0';
        v1 = sub_1694e(a0,v3);
        if ((!v4) || ((int)v1 <= 0)) break;
        v6 = (unsigned char *)&v4[1];
        *v4 = '/';
      }
      free(v3);
      v2 = (unsigned long)v1;
    }
    else {
      v2 = strlen((char *)a0);
      v5 = strncmp((char *)a0,(char *)a1,v2);
      if ((!v5) && (v5 = (unsigned int)(char)a1[v2], v5 == 0x2f))
        v5 = 0;
      v2 = (unsigned long)v5;
    }
  }
  else if (a2 & 0x10) // branch-flip
    v2 = sub_1694e(a0,a1);
  else {
    v2 = CONCAT44(dat_4,strcmp((char *)a0,(char *)a1));
  }
  return v2;
}

// Function: sub_13f61 @ 0x13f61
bool sub_13f61(unsigned long a0,char *a1,unsigned int a2)
{
  void *v1;
  bool v2; // stack - 0x19
  char *v3; // stack - 0x18
  
  if (a2 & 0x10000000) // branch-flip
    v1 = dat_2cfe0;
  else {
    v1 = sub_13e26;
  }
  v2 = (*v1)(a0,a1,a2) == 0;
  if (!(a2 & 0x40000000)) {
    v3 = a1;
    while ((*v3 && (v2 != 1))) {
      if ((*v3 == '/') && (v3[1] != '/'))
        v2 = (*v1)(a0,&v3[1],a2) == 0;
      v3 = &v3[1];
    }
  }
  return v2;
}

// Function: sub_14029 @ 0x14029
unsigned long sub_14029(struct_8 *a0,char *a1)
{
  unsigned int v1;
  unsigned long v2;
  unsigned long v3;
  unsigned int v4; // eax
  unsigned long v5; // rax
  bool v6; // zf
  
  v1 = a0->field_0x0;
  v4 = v1 & 0x8000000;
  if (v4) { // branch-flip
    v4 = regexec((re_pattern_buffer *)&a0->field_0x8,a1,0,NULL,0);
    v6 = v4 == 0;
    v3 = v5 >> 8;
    v5 = CONCAT71((undefined7)v3,v6);
  }
  else {
    v2 = a0->field_0x8;
    v5 = (unsigned long)(sub_13f61(v2,a1,v1) != '\0');
  }
  return v5;
}

// Function: sub_140a1 @ 0x140a1
unsigned long sub_140a1(void *a0,char *a1)
{
  long v1;
  long v2;
  long v3; // stack - 0x20
  
  v1 = *(long *)((long)a0 + 0x20);
  v2 = *(long *)((long)a0 + 0x10);
  v3 = 0;
  while( true ) {
    if (v1 <= v3)
      return 0;
    if (sub_14029((struct_8 *)(v3 * 0x48 + v2),a1)) break;
    v3 += 1;
  }
  return 1;
}

// Function: sub_14123 @ 0x14123
unsigned long sub_14123(void *a0,char *a1,char *a2) // return-dupe
{
  unsigned int v1;
  struct_10 *v2;
  char *v3; // rax
  char *v4; // stack - 0x38
  
  v1 = *(unsigned int *)((long)a0 + 0xc);
  v2 = *(struct_10 **)((long)a0 + 0x10);
  v4 = a1;
  while( true ) {
    strcpy(a2,v4);
    while( true ) {
      if (sub_1508d(v2,a2))
        return 1;
      if ((!(v1 & 8)) || (v3 = strrchr(a2,0x2f), !v3)) break;
      *v3 = '\0';
    }
    if (v1 & 0x40000000) break;
    v4 = strchr(v4,0x2f);
    if (v4)
      v4 = &v4[1];
    if (!v4)
      return 0;
  }
  return 0;
}

// Function: sub_141f6 @ 0x141f6
char sub_141f6(long *a0,char *a1) // early-return
{
  char v1; // al
  char *v2; // rax
  bool v3; // stack - 0x19
  long *v4; // stack - 0x18
  char *v5; // stack - 0x10
  
  v3 = 0;
  v5 = NULL;
  if (!*a0) {
    v1 = 0;
    return v1;
  }
  v4 = (long *)*a0;
  while( true ) {
    if ((int)v4[1]) // branch-flip
      v1 = sub_140a1(v4,a1);
    else {
      if (!v5) {
        v2 = (unsigned long)strlen(a1);
        v2 = (unsigned long)((long)v2 + 1);
        v2 = (char *)sub_1c945(v2);
        v5 = v2;
      }
      v1 = sub_14123(v4,a1,v5);
    }
    if (v1) {
      free(v5); // return-dupe
      v1 = ((*(unsigned int *)((long)v4 + 0xc) & 0x20000000) == 0) != v3;
      return v1;
    }
    if (!*v4) break;
    v4 = (long *)*v4;
  }
  v3 = 1;
  free(v5);
  v1 = ((*(unsigned int *)((long)v4 + 0xc) & 0x20000000) == 0) != v3;
  return v1;
}

// Function: sub_142e9 @ 0x142e9
void sub_142e9(long *a0,char *a1,unsigned int a2) // return-dupe
{
  long v1;
  unsigned long v10; // stack - 0x40
  void *v2;
  long v3;
  unsigned int v4; // eax
  long *v5; // rax
  unsigned int *v6; // rax
  char *v7; // rax
  char *v8; // stack - 0x68
  int v9; // stack - 0x4c
  
  if ((!(a2 & 0x18000000)) || (!sub_13820(a1,a2))) {
    if (((!*a0) || (*(int *)(*a0 + 8))) || ((*(unsigned int *)(*a0 + 0xc) ^ a2) & 0x60000018))
      sub_13c0f(a0,0,a2);
    v1 = *a0;
    v7 = (char *)sub_1ced8(a1);
    if ((a2 & 0x10000002) == 0x10000000)
      sub_1395f(v7);
    v2 = *(void **)(v1 + 0x10);
    if ((char *)sub_16483(v2,v7) == v7)
      return;
    free(v7);
    return;
  }
  if ((!*a0) || ((*(int *)(*a0 + 8) != 1 || ((*(unsigned int *)(*a0 + 0xc) ^ a2) & 0x20000000))))
    sub_13c0f(a0,1,a2);
  v1 = *a0;
  v5 = (long *)(v1 + 0x10);
  if (*(long *)(v1 + 0x20) == *(long *)(v1 + 0x18)) {
    v2 = (void *)*v5;
    *v5 = sub_1cbad(v2,(long *)(v1 + 0x18),1,-1,0x48);
  }
  v3 = *(long *)(v1 + 0x20);
  *(long *)(v1 + 0x20) = v3 + 1;
  v6 = (unsigned int *)(v3 * 0x48 + *v5);
  *v6 = a2;
  if (a2 & 0x8000000) {
    v4 = (int)a2 >> 3 & 2U | 9;
    if (a2 & 8) { // branch-flip
      for (v10 = strlen(a1); (1 <= (long)v10 && (a1[v10 - 1] == '/')); v10 = v10 - 1) {
      }
      if (v10) { // branch-flip
        v7 = (char *)sub_1c96b(v10 + 7);
        memcpy(v7,a1,v10);
        strcpy(&v7[v10],"(/.*)?");
        v9 = regcomp((re_pattern_buffer *)&v6[2],v7,v4);
        free(v7);
      }
      else {
        v9 = 1;
      }
    }
    else {
      v9 = regcomp((re_pattern_buffer *)&v6[2],a1,v4);
    }
    if (!v9)
      return;
    *(long *)(v1 + 0x20) = *(long *)(v1 + 0x20) + -1;
    return;
  }
  v8 = a1;
  if (a2 & 0x4000000) {
    v8 = (char *)sub_1ced8(a1);
    sub_137d4(a0,v8);
  }
  *(char **)&v6[2] = v8;
}

// Function: sub_14622 @ 0x14622
unsigned long sub_14622(void *a0,struct_24 *a1,FILE *a2,unsigned int a3,unsigned char a4,unsigned long a5) // return-dupe, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_14622
{
  long v1;
  unsigned char *v10; // stack - 0x18
  int v2; // eax
  long v3; // stack - 0x48
  int v4; // stack - 0x50
  unsigned char *v5; // stack - 0x40
  unsigned char *v6; // stack - 0x38
  unsigned char *v7; // stack - 0x30
  long v8; // stack - 0x28
  unsigned char *v9; // stack - 0x20
  
  v5 = NULL;
  v3 = 0;
  v8 = 0;
  v4 = 0;
  while (v2 = getc(a2), v2 != -1) {
    if (v8 == v3)
      v5 = (void *)sub_1cbad(v5,&v3,1,-1,1);
    v1 = v8 + 1;
    *(char *)((long)v5 + v8) = (char)v2;
    v8 = v1;
  }
  if (ferror(a2))
    v4 = *__errno_location();
  v5 = (unsigned char *)sub_1c9dc(v5,v8 + 1);
  v5[v8] = a4;
  v2 = ((v8) && (a4 != v5[v8 + -1]));
  v10 = &v5[v2 + v8];
  sub_137d4(a1,v5);
  v7 = v5;
  v6 = v5;
  do {
    if (v10 <= v6) {
      *__errno_location() = v4;
      if (v4)
        return 0xffffffff;
      return 0;
    }
    if (a4 == *v6) {
      v9 = v6;
      if (*(unsigned short *)((unsigned long)a4 * 2 + *(long *)__ctype_b_loc()) & 0x2000) { // branch-flip
        for (; v9 != v7; v9 = &v9[-1]) {
          if (!(*(unsigned short *)((unsigned long)v9[-1] * 2 + *(long *)__ctype_b_loc()) & 0x2000)) goto label_147fd;
        }
      }
      else {
label_147fd:
        *v9 = 0;
        (*a0)(a1,v7,a3,a5);
      }
      v7 = &v6[1];
    }
    v6 = &v6[1];
  } while( true );
}

// Function: sub_14871 @ 0x14871
void sub_14871(unsigned long a0,unsigned long a1,unsigned int a2,unsigned long *a3)
{
  (*(void *)*a3)(a0,a1,a2);
}

// Function: sub_148b2 @ 0x148b2
unsigned int sub_148b2(unsigned long a0,struct_24 *a1,char *a2,unsigned int a3,char a4) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_148b2
{
  unsigned long v1; // stack - 0x20
  char v2; // stack - 0x15
  unsigned int v3; // stack - 0x14
  FILE *v4; // stack - 0x10
  
  v2 = !((*a2 != '-') || (a2[1]));
  v3 = 0;
  v1 = a0;
  if (v2) // branch-flip
    v4 = stdin;
  else {
    v4 = fopen(a2,"re");
    if (!v4)
      return 0xffffffff;
  }
  v3 = sub_14622(sub_14871,a1,v4,a3,(int)a4,&v1);
  if ((v2 != '\x01') && (fclose(v4)))
    v3 = 0xffffffff;
  return v3;
}

// Function: sub_14997 @ 0x14997
void sub_14997(void *a0) // return-dupe x8
{
  if ((*(unsigned int *)((long)a0 + 0x18) & 0xf000) == 0x8000) {
    if (!*(long *)((long)a0 + 0x30)) {
      gettext("regular empty file");
      return;
    }
    gettext("regular file");
    return;
  }
  if ((*(unsigned int *)((long)a0 + 0x18) & 0xf000) == 0x4000) {
    gettext("directory");
    return;
  }
  if ((*(unsigned int *)((long)a0 + 0x18) & 0xf000) == 0xa000) {
    gettext("symbolic link");
    return;
  }
  if ((*(unsigned int *)((long)a0 + 0x18) & 0xf000) == 0x6000) {
    gettext("block special file");
    return;
  }
  if ((*(unsigned int *)((long)a0 + 0x18) & 0xf000) == 0x2000) {
    gettext("character special file");
    return;
  }
  if ((*(unsigned int *)((long)a0 + 0x18) & 0xf000) == 0x1000) {
    gettext("fifo");
    return;
  }
  if ((*(unsigned int *)((long)a0 + 0x18) & 0xf000) == 0xc000) {
    gettext("socket");
    return;
  }
  gettext("weird file");
}

// Function: sub_14ade @ 0x14ade
long sub_14ade(void *a0,char *a1,unsigned long *a2)
{
  long v1; // rax
  
  v1 = sub_14b23(a0,a1,a2);
  if (!v1)
    sub_1cf09(); // no-return
  return v1;
}

// Function: sub_14b23 @ 0x14b23
void * sub_14b23(void *a0,char *a1,unsigned long *a2) // early-return
{
  char *v1; // rax
  long v2; // rax
  unsigned long v3; // rax
  void *v4; // rax
  char v5; // stack - 0x39
  
  v1 = (char *)sub_13111(a0);
  v2 = sub_1317e(v1);
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

// Function: sub_14c65 @ 0x14c65
unsigned long sub_14c65(void)
{
  return __progname;
}

// Function: sub_14c76 @ 0x14c76
void sub_14c76(timespec *a0)
{
  clock_gettime(0,a0);
}

// Function: sub_14c9a @ 0x14c9a
undefined16 sub_14c9a(void)
{
  char v1 [16]; // stack - 0x28
  
  sub_14c76(v1);
  return v1._0_16_;
}

// Function: sub_14cdf @ 0x14cdf
unsigned int sub_14cdf(unsigned int a0) // return-dupe
{
  char v1 [264];
  
  if (sub_1a47d(a0,v1,0x101))
    return 0;
  if ((strcmp(v1,"C")) && (strcmp(v1,"POSIX")))
    return 1;
  return 0;
}

// Function: sub_14d89 @ 0x14d89
unsigned long sub_14d89(struct_33 *a0)
{
  return a0->field_0x10;
}

// Function: sub_14d9f @ 0x14d9f
unsigned long sub_14d9f(struct_34 *a0)
{
  return a0->field_0x18;
}

// Function: sub_14db5 @ 0x14db5
unsigned long sub_14db5(struct_35 *a0)
{
  return a0->field_0x20;
}

// Function: sub_14dcb @ 0x14dcb
unsigned long sub_14dcb(struct_36 *a0)
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

// Function: sub_14e4d @ 0x14e4d
unsigned long sub_14e4d(struct_36 *a0) // return-dupe
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

// Function: sub_14eeb @ 0x14eeb
void sub_14eeb(struct_35 *a0,FILE *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_14db5(a0);
  v2 = sub_14d89(a0);
  v3 = sub_14d9f(a0);
  v4 = sub_14dcb(a0);
  fprintf(a1,"# entries:         %lu\n",v1);
  fprintf(a1,"# buckets:         %lu\n",v2);
  fprintf(a1,"# buckets used:    %lu (%.2f%%)\n",v3,(dat_25928 * (double)v3) / (double)v2);
  fprintf(a1,"max bucket length: %lu\n",v4);
}

// Function: sub_15032 @ 0x15032
long sub_15032(struct_9 *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*a0->field_0x30)(a1,a0->field_0x10);
  if ((unsigned long)a0->field_0x10 <= v1)
    abort(); // no-return
  return v1 * 0x10 + a0->field_0x0;
}

// Function: sub_1508d @ 0x1508d
long sub_1508d(struct_10 *a0,char *a1) // return-dupe
{
  void *v1;
  long v2;
  long *v3; // stack - 0x18
  
  v3 = (long *)sub_15032(a0,a1);
  if (!*v3)
    return 0;
  while( true ) {
    if (!v3)
      return 0;
    if ((a1 == (char *)*v3) || (v1 = a0->field_0x38, v2 = *v3, (*v1)(a1,v2))) break;
    v3 = (long *)v3[1];
  }
  return *v3;
}

// Function: sub_15124 @ 0x15124
long sub_15124(struct_36 *a0) // early-return
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

// Function: sub_15184 @ 0x15184
long sub_15184(void *a0,long a1) // early-return
{
  long *v1; // stack - 0x18
  long *v2; // stack - 0x10
  
  v1 = (long *)sub_15032(a0,a1);
  v2 = v1;
  while ((a1 != *v2 || (!v2[1]))) {
    v2 = (long *)v2[1];
    if (!v2) {
      do {
        v1 = &v1[2];
        if (*(long **)((long)a0 + 8) <= v1)
          return 0;
      } while (!*v1);
      return *v1;
    }
  }
  return *(long *)v2[1];
}

// Function: sub_15222 @ 0x15222
unsigned long sub_15222(struct_36 *a0,long a1,unsigned long a2)
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

// Function: sub_152c2 @ 0x152c2
long sub_152c2(struct_36 *a0,void *a1,unsigned long a2)
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

// Function: sub_1535a @ 0x1535a
unsigned long sub_1535a(unsigned char *a0,unsigned long a1)
{
  unsigned char *v1; // stack - 0x20
  unsigned long v2; // stack - 0x10
  
  v2 = 0;
  for (v1 = a0; *v1; v1 = &v1[1]) {
    v2 = ((unsigned long)*v1 + v2 * 0x1f) % a1;
  }
  return v2;
}

// Function: sub_153b4 @ 0x153b4
unsigned long sub_153b4(unsigned long a0)
{
  unsigned long v1; // stack - 0x18
  unsigned long v2; // stack - 0x10
  
  v2 = 9;
  for (v1 = 3; (v2 < a0 && (a0 % v1)); v1 = v1 + 2) {
    v2 += (v1 + 1) * 4;
  }
  return CONCAT71((undefined7)(a0 % v1 >> 8),a0 % v1 != 0);
}

// Function: sub_15423 @ 0x15423
unsigned long sub_15423(unsigned long a0)
{
  unsigned long v1; // stack - 0x10
  
  v1 = a0;
  if (a0 <= 9)
    v1 = 10;
  v1 |= 1;
  while( true ) {
    if (v1 == 0xffffffffffffffff)
      return 0xffffffffffffffff;
    if (sub_153b4(v1) == '\x01') break;
    v1 += 2;
  }
  return v1;
}

// Function: sub_1546e @ 0x1546e
void sub_1546e(struct_37 *a0)
{
  a0->field_0x0 = 0;
  a0->field_0x4 = dat_25930;
  a0->field_0x8 = dat_25934;
  a0->field_0xc = dat_25938;
  a0->field_0x10 = 0;
}

// Function: sub_154c4 @ 0x154c4
unsigned long sub_154c4(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_1dc13(a0,3) % a1;
}

// Function: sub_154ff @ 0x154ff
unsigned long sub_154ff(long a0,long a1)
{
  return CONCAT71((undefined7)((unsigned long)a0 >> 8),a0 == a1);
}

// Function: sub_1551c @ 0x1551c
unsigned long sub_1551c(struct_25 *a0) // return-dupe x2
{
  float *v1;
  
  v1 = a0->field_0x28;
  if (v1 == (float *)0x258a0)
    return 1;
  if ((((dat_2593c < v1[2]) && (v1[2] < dat_25930 - dat_2593c)) && (dat_2593c + dat_25930 < v1[3])) && (((0.0 <= *v1 && (*v1 + dat_2593c < v1[1])) && ((v1[1] <= dat_25930 && (*v1 + dat_2593c < v1[2]))))))
    return 1;
  a0->field_0x28 = (float *)0x258a0;
  return 0;
}

// Function: sub_15629 @ 0x15629
unsigned long sub_15629(unsigned long a0,void *a1) // early-return x2
{
  unsigned long v1; // rax
  float v2; // xmm0_da
  unsigned long v3; // stack - 0x20
  
  v3 = a0;
  if (*(char *)((long)a1 + 0x10) != '\x01') {
    v2 = (float)a0 / *(float *)((long)a1 + 8);
    if (dat_25940 <= v2)
      return 0;
    if (dat_25944 <= v2)
      v3 = (long)(v2 - dat_25944) ^ 0x8000000000000000;
    else {
      v3 = (unsigned long)v2;
    }
  }
  v1 = sub_15423(v3);
  if (!((long)(v1 << 3) <= -1 || v1 >> 0x3d))
    return v1;
  return 0;
}

// Function: sub_15742 @ 0x15742
long * sub_15742(unsigned long a0,void *a1,void *a2,void *a3,long a4) // early-return x2
{
  unsigned long v1;
  long *v2; // rax
  void *v3; // stack - 0x38
  void *v4; // stack - 0x30
  void *v5; // stack - 0x28
  
  v4 = a2;
  if (!a2)
    v4 = sub_154c4;
  v3 = a3;
  if (!a3)
    v3 = sub_154ff;
  v2 = malloc(0x50);
  if (!v2)
    return NULL;
  v5 = a1;
  if (!a1)
    v5 = (void *)0x258a0;
  v2[5] = (long)v5;
  if (sub_1551c(v2) == '\x01') {
    v2[2] = sub_15629(a0,v5);
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

// Function: sub_158b7 @ 0x158b7
void sub_158b7(struct_38 *a0)
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

// Function: sub_159c2 @ 0x159c2
void sub_159c2(struct_27 *a0)
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

// Function: sub_15af1 @ 0x15af1
void * sub_15af1(struct_29 *a0)
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

// Function: sub_15b40 @ 0x15b40
void sub_15b40(struct_29 *a0,struct_23 *a1)
{
  a1->field_0x0 = 0;
  a1->field_0x8 = a0->field_0x48;
  a0->field_0x48 = a1;
}

// Function: sub_15b7a @ 0x15b7a
long sub_15b7a(struct_10 *a0,long a1,unsigned long *a2,bool a3) // early-return
{
  void *v1;
  long v2;
  struct_23 *v3;
  long v4;
  unsigned long v5;
  long *v6; // stack - 0x38
  
  v6 = (long *)sub_15032(a0,a1);
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
      v3 = (struct_23 *)v6[1];
      v6[1] = v3->field_0x8;
      sub_15b40(a0,v3);
    }
    return v2;
  }
  v2 = *v6;
  if (a3) {
    if (v6[1]) { // branch-flip
      v3 = (struct_23 *)v6[1];
      v4 = v3->field_0x8;
      *v6 = v3->field_0x0;
      v6[1] = v4;
      sub_15b40(a0,v3);
    }
    else {
      *v6 = 0;
    }
  }
  return v2;
}

// Function: sub_15d13 @ 0x15d13
unsigned long sub_15d13(struct_31 *a0,struct_36 *a1,bool a2)
{
  long *v1;
  long v2;
  struct_2 *v3; // rax
  struct_2 *v4; // rax
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
        v3 = (struct_2 *)sub_15032(a0,v2);
        v1 = (long *)v6[1];
        if (v3->field_0x0) { // branch-flip
          v6[1] = v3->field_0x8;
          v3->field_0x8 = (long)v6;
        }
        else {
          v3->field_0x0 = v2;
          a0->field_0x18 = a0->field_0x18 + 1;
          sub_15b40(a0,v6);
        }
        v6 = v1;
      }
      v2 = *v5;
      v5[1] = 0;
      if (!a2) {
        v3 = (struct_2 *)sub_15032(a0,v2);
        if (v3->field_0x0) { // branch-flip
          v4 = (struct_2 *)sub_15af1(a0);
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

// Function: sub_15ee7 @ 0x15ee7
unsigned long sub_15ee7(struct_32 *a0,unsigned long a1) // return-dupe x4
{
  void *v1; // stack - 0x68
  unsigned long v10; // stack - 0x20
  void *v2; // stack - 0x60
  unsigned long v3; // stack - 0x58
  unsigned long v4; // stack - 0x50
  unsigned long v5; // stack - 0x48
  void *v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  unsigned long v8; // stack - 0x30
  unsigned long v9; // stack - 0x28
  
  v3 = sub_15629(a1,a0->field_0x28);
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
  if (sub_15d13(&v1,a0,0)) {
    free(a0->field_0x0);
    a0->field_0x0 = v1;
    a0->field_0x8 = v2;
    a0->field_0x10 = v3;
    a0->field_0x18 = v4;
    a0->field_0x48 = v10;
    return 1;
  }
  a0->field_0x48 = v10;
  if ((sub_15d13(a0,&v1,1) == '\x01') && (sub_15d13(a0,&v1,0) == '\x01')) {
    free(v1);
    return 0;
  }
  abort(); // no-return
}

// Function: sub_16112 @ 0x16112
unsigned long sub_16112(void *a0,long a1,long *a2) // return-dupe x2, return-dupe, ternary x2
{
  unsigned long v1; // rax
  long *v2; // rax
  long *v3; // stack - 0x30
  float v4; // xmm0_da
  long v5; // stack - 0x28
  long v6; // stack - 0x20
  
  if (!a1)
    abort(); // no-return
  v5 = sub_15b7a(a0,a1,&v3,0);
  if (v5) {
    if (!a2)
      return 0;
    *a2 = v5;
    return 0;
  }
  if (((float)*(unsigned long *)((long)a0 + 0x10) * *(float *)(*(long *)((long)a0 + 0x28) + 8) < (float)*(unsigned long *)((long)a0 + 0x18)) && (sub_1551c(a0), (float)*(unsigned long *)((long)a0 + 0x10) * *(float *)(*(long *)((long)a0 + 0x28) + 8) < (float)*(unsigned long *)((long)a0 + 0x18))) {
    v6 = *(long *)((long)a0 + 0x28);
    v4 = (*(char *)(v6 + 0x10)) ? (float)*(unsigned long *)((long)a0 + 0x10) * *(float *)(v6 + 0xc) : *(float *)(v6 + 8) * (*(float *)(v6 + 0xc) * (float)*(unsigned long *)((long)a0 + 0x10)); // branch-flip
    if (dat_25940 <= v4)
      return 0xffffffff;
    v1 = (dat_25944 <= v4) ? (long)(v4 - dat_25944) ^ 0x8000000000000000 : (unsigned long)v4;
    if (sub_15ee7(a0,v1) != '\x01')
      return 0xffffffff;
    if (sub_15b7a(a0,a1,&v3,0))
      abort(); // no-return
  }
  if (!*v3) {
    *v3 = a1;
    *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + 1;
    *(long *)((long)a0 + 0x18) = *(long *)((long)a0 + 0x18) + 1;
    return 1;
  }
  v2 = (long *)sub_15af1(a0);
  if (v2) {
    *v2 = a1;
    v2[1] = v3[1];
    v3[1] = (long)v2;
    *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + 1;
    return 1;
  }
  return 0xffffffff;
}

// Function: sub_16483 @ 0x16483
unsigned long sub_16483(void *a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  
  v1 = sub_16112(a0,a1,&v3);
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

// Function: sub_164f3 @ 0x164f3
long sub_164f3(void *a0,long a1) // early-return, ternary x2
{
  char v1; // al
  long *v2; // stack - 0x40
  float v3; // xmm0_da
  void *v4; // stack - 0x38
  long v5; // stack - 0x30
  long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  void *v8; // stack - 0x18
  
  v5 = sub_15b7a(a0,a1,&v2,1);
  if (!v5)
    return 0;
  *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + -1;
  if (((!*v2) && (*(long *)((long)a0 + 0x18) = *(long *)((long)a0 + 0x18) + -1, (float)*(unsigned long *)((long)a0 + 0x18) < (float)*(unsigned long *)((long)a0 + 0x10) * **(float **)((long)a0 + 0x28))) && (sub_1551c(a0), (float)*(unsigned long *)((long)a0 + 0x18) < (float)*(unsigned long *)((long)a0 + 0x10) * **(float **)((long)a0 + 0x28))) {
    v6 = *(long *)((long)a0 + 0x28);
    if (*(char *)(v6 + 0x10)) { // branch-flip
      v3 = (float)*(unsigned long *)((long)a0 + 0x10) * *(float *)(v6 + 4);
      v7 = (dat_25944 <= v3) ? (long)(v3 - dat_25944) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    else {
      v3 = *(float *)(v6 + 8) * (*(float *)(v6 + 4) * (float)*(unsigned long *)((long)a0 + 0x10));
      v7 = (dat_25944 <= v3) ? (long)(v3 - dat_25944) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    v1 = sub_15ee7(a0,v7);
    if (v1 != '\x01') {
      v4 = *(void **)((long)a0 + 0x48);
      while (v4) {
        v8 = *(void **)((long)v4 + 8);
        free(v4);
        v4 = v8;
      }
      *(unsigned long *)((long)a0 + 0x48) = 0;
    }
  }
  return v5;
}

// Function: sub_167f9 @ 0x167f9
void sub_167f9(void *a0,long a1)
{
  sub_164f3(a0,a1);
}

// Function: sub_16822 @ 0x16822
long sub_16822(unsigned long a0) // early-return
{
  void *v1; // rax
  unsigned long v2; // rax
  long v3; // rax
  
  if ((a0 <= a0 + 0x8000000000000020) && (v1 = malloc(a0 + 0x20), v1)) {
    v2 = (long)v1 + 0x10U & 0xffffffffffffffe0;
    v3 = v2 + 0x10;
    *(char *)(v2 + 0xf) = (char)v3 - (char)v1;
    return v3;
  }
  return 0;
}

// Function: sub_168f3 @ 0x168f3
void sub_168f3(void *a0) // return-dupe
{
  if ((unsigned long)a0 & 0xf)
    abort(); // no-return
  if (!((unsigned long)a0 & 0x10))
    return;
  free((void *)((long)a0 - (unsigned long)*(unsigned char *)((long)a0 + -1)));
}

// Function: sub_1694e @ 0x1694e
int sub_1694e(unsigned char *a0,unsigned char *a1) // early-return, ternary x2
{
  bool v1;
  unsigned char *v10; // stack - 0xb8
  unsigned char *v11; // stack - 0xb0
  char v12; // stack - 0x9c
  unsigned char *v13; // stack - 0x98
  unsigned long v14; // stack - 0x90
  char v15; // stack - 0x88
  unsigned int v16; // stack - 0x84
  char v17; // stack - 0x5c
  unsigned char *v18; // stack - 0x58
  unsigned long v19; // stack - 0x50
  unsigned char v2;
  char v20; // stack - 0x48
  unsigned int v21; // stack - 0x44
  unsigned char v3;
  unsigned int v4; // eax
  int v5; // eax
  char v6 [8];
  char v7 [8];
  char v8 [4]; // stack - 0xa8
  char v9 [4]; // stack - 0x68
  
  if (a0 == a1)
    return 0;
  if (2 <= __ctype_get_mb_cur_max()) {
    v8[0] = 0;
    v13 = a0;
    memset(v6,0,8);
    v12 = 0;
    v9[0] = 0;
    v18 = a1;
    memset(v7,0,8);
    while( true ) {
      v17 = 0;
      sub_16d35(v8);
      if ((v15 != '\x01') || (v16))
        v1 = 1;
      else {
        v1 = 0;
      }
      if (!v1) break;
      sub_16d35(v9);
      if ((v20 != '\x01') || (v21))
        v1 = 1;
      else {
        v1 = 0;
      }
      if (!v1) break;
      if (v15) { // branch-flip
        if (v20) { // branch-flip
          v4 = towlower(v16);
          v5 = v4 - towlower(v21);
        }
        else {
          v5 = -1;
        }
      }
      else if (v20) // branch-flip
        v5 = 1;
      else if (v14 != v19) { // branch-flip
        if (v19 <= v14) { // branch-flip
          v5 = (0 <= memcmp(v13,v18,v19)) ? 1 : -1; // branch-flip
        }
        else {
          v5 = (1 <= memcmp(v13,v18,v14)) ? 1 : -1; // branch-flip
        }
      }
      else {
        v5 = memcmp(v13,v18,v14);
      }
      if (v5)
        return v5;
      v13 = &v13[v14];
      v12 = 0;
      v18 = &v18[v19];
    }
    sub_16d35(v8);
    if ((v15 != '\x01') || (v16))
      v1 = 1;
    else {
      v1 = 0;
    }
    if (v1)
      return 1;
    sub_16d35(v9);
    if ((v20 != '\x01') || (v21))
      v1 = 1;
    else {
      v1 = 0;
    }
    if (v1)
      return -1;
    return 0;
  }
  v10 = a0;
  v11 = a1;
  do {
    if (*(unsigned short *)((unsigned long)*v10 * 2 + *(long *)__ctype_b_loc()) & 0x100) { // branch-flip
      v2 = *v10;
      v2 = (unsigned char)tolower((unsigned int)v2);
    }
    else {
      v2 = *v10;
    }
    if (*(unsigned short *)((unsigned long)*v11 * 2 + *(long *)__ctype_b_loc()) & 0x100) { // branch-flip
      v3 = *v11;
      v3 = (unsigned char)tolower((unsigned int)v3);
    }
    else {
      v3 = *v11;
    }
    if (!v2) break;
    v10 = &v10[1];
    v11 = &v11[1];
  } while (v2 == v3);
  return (unsigned int)v2 - (unsigned int)v3;
}

// Function: sub_16d35 @ 0x16d35
void sub_16d35(struct_7 *a0)
{
  char v1;
  char *v2;
  unsigned long v3; // rax
  
  if (a0->field_0xc)
    return;
  if (!a0->field_0x0) {
    v1 = *a0->field_0x10;
    if (sub_1e296((int)v1)) {
      a0->field_0x18 = 1;
      a0->field_0x24 = (int)*a0->field_0x10;
      a0->field_0x20 = '\x01';
      goto label_16f3b;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x1[3]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    a0->field_0x0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v3 = sub_1bc00(a0->field_0x10,v3);
  v2 = a0->field_0x10;
  a0->field_0x18 = sub_1d369(&a0->field_0x24,v2,v3,(mbstate_t *)&a0->field_0x1[3]);
  if (a0->field_0x18 != -1) { // branch-flip
    if (a0->field_0x18 != -2) { // branch-flip
      if (!a0->field_0x18) {
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
      v2 = a0->field_0x10;
      a0->field_0x18 = strlen(v2);
      a0->field_0x20 = '\0';
    }
  }
  else {
    a0->field_0x18 = 1;
    a0->field_0x20 = '\0';
  }
label_16f3b:
  a0->field_0xc = '\x01';
  return;
}

// Function: sub_16f4c @ 0x16f4c
void sub_16f4c(void *a0,long a1)
{
  *(long *)((long)a0 + 0x10) = *(long *)((long)a0 + 0x10) + a1;
}

// Function: sub_16f76 @ 0x16f76
void sub_16f76(struct_41 *a0,struct_41 *a1)
{
  a0->field_0x0 = a1->field_0x0;
  if (a0->field_0x0) // branch-flip
    memcpy(&a0->field_0x1[3],&a1->field_0x1[3],8);
  else {
    memset(&a0->field_0x1[3],0,8);
  }
  a0->field_0xc = a1->field_0xc;
  sub_1e1f0((struct_39 *)&a0[1].field_0x1[2],(struct_40 *)&a1[1].field_0x1[2]);
}

// Function: sub_1700b @ 0x1700b
long sub_1700b(long a0,long a1,long a2)
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

// Function: sub_17063 @ 0x17063
long sub_17063(long a0,long a1,long a2)
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

// Function: sub_170bb @ 0x170bb
int sub_170bb(int a0,int a1)
{
  return (a0 - ((a0 - a1) + 0x17e) % 7) + 3;
}

// Function: sub_1710b @ 0x1710b
void sub_1710b(char *a0,long a1,char *a2,tm *a3,unsigned long a4,unsigned int a5)
{
  char v1; // stack - 0x11
  
  v1 = 0;
  sub_1718e(a0,a1,a2,a3,0,0,0xffffffffffffffff,&v1,a4,a5);
}

// Function: sub_1718e @ 0x1718e
long sub_1718e(char *a0,long a1,char *a2,tm *a3,char a4,int a5,int a6,unsigned long a7,unsigned long a8,int a9) // early-return x13, ternary x6, warn: iteboolean: re-rolled 3 0/1 select diamond(s) to a boolean assignment in sub_1718e
{
  char *v1;
  unsigned int v10; // eax
  int v11; // eax
  unsigned long v12; // rax
  unsigned long v13;
  unsigned long v14;
  char v15 [3]; // stack - 0x42d
  char v16 [2];
  char v17 [22];
  char v18 [1010];
  unsigned long v19; // stack - 0x468
  int v2;
  bool v20; // stack - 0x6c0
  bool v21; // stack - 0x6bd
  unsigned int v22; // stack - 0x6b8
  int v23; // stack - 0x6b4
  int v24; // stack - 0x6ac
  unsigned int v25; // stack - 0x6a8
  unsigned int v26; // stack - 0x6a4
  unsigned int v27; // stack - 0x6a0
  char v28; // stack - 0x69c
  int v29; // stack - 0x698
  unsigned int v3;
  int v30; // stack - 0x694
  int v31; // stack - 0x690
  int v32; // stack - 0x68c
  int v33; // stack - 0x688
  int v34; // stack - 0x684
  unsigned int v35; // stack - 0x680
  int v36; // stack - 0x67c
  char *v37; // stack - 0x640
  long v38; // stack - 0x638
  char *v39; // stack - 0x630
  bool v4;
  char *v40; // stack - 0x628
  char *v41; // stack - 0x620
  char *v42; // stack - 0x618
  unsigned long v43; // stack - 0x610
  char *v44; // stack - 0x608
  long v45; // stack - 0x600
  unsigned long v46; // stack - 0x460
  unsigned long v47; // stack - 0x458
  long v48; // stack - 0x450
  unsigned long v49; // stack - 0x448
  unsigned long v5;
  unsigned long v50; // stack - 0x440
  unsigned long v51; // stack - 0x438
  unsigned long v6;
  bool v7;
  char v8;
  int v9;
  
  v6 = a8;
  v5 = a7;
  v2 = *__errno_location();
  v22 = *(unsigned int *)&a3->field_0x8;
  v38 = 0;
  v37 = *(char **)&a3->field_0x30;
  if (!v37)
    v37 = "";
  if (0xd <= (int)v22) // branch-flip
    v22 -= 0xc;
  else if (!v22)
    v22 = 0xc;
  v39 = a0;
  v40 = a2;
  do {
    if (!*v40) {
      if ((v39) && (a1))
        *v39 = '\0';
      *__errno_location() = v2;
      return v38;
    }
    v23 = 0;
    v4 = 0;
    v7 = 0;
    v21 = a4;
    if (*v40 != '%') {
      v12 = (0 <= a6) ? (unsigned long)a6 : 0; // branch-flip
      v14 = v12;
      if (v12 <= 1)
        v14 = 1;
      if ((unsigned long)(a1 - v38) <= v14) {
        *__errno_location() = 0x22;
        return 0;
      }
      if (v39) {
        if (1 < v12) {
          memset(v39,0x20,v12 - 1);
          v39 = &v39[v12 - 1];
        }
        *v39 = *v40;
        v39 = &v39[1];
      }
      v38 += v14;
      goto label_199e6;
    }
    while( true ) {
      v40 = &v40[1];
      if (0x3c < (unsigned int)((int)*v40 - 0x23U)) break;
      v12 = 1L << ((unsigned char)((int)*v40 - 0x23U) & 0x3f);
      if (v12 & 0x1000000000002500) // branch-flip
        v23 = (int)*v40;
      else if (v12 & 1) // branch-flip
        v7 = 1;
      else {
        if (!(v12 & 0x800000000000000)) break;
        v21 = '\x01';
      }
    }
    if ((unsigned int)((int)*v40 - 0x30U) <= 9) {
      a6 = 0;
      do {
        v9 = (int)((long)a6 * 10);
        if (((long)v9 != (long)a6 * 10) || (a6 = *v40 + -0x30 + v9, SCARRY4(*v40 + -0x30,v9)))
          a6 = 0x7fffffff;
        v40 = &v40[1];
      } while ((unsigned int)((int)*v40 - 0x30U) <= 9);
    }
    if ((*v40 != 'E') && (*v40 != 'O'))
      v8 = '\0';
    else {
      v1 = &v40[1];
      v8 = *v40;
      v40 = v1;
    }
    v28 = *v40;
    switch(v28) {
      case 0:
        v40 = &v40[-1];
        break;
      case 0x25:
        if (v8) break;
        if ((v23 != 0x2d) && (0 <= a6))
          v12 = (unsigned long)a6;
        else {
          v12 = 0;
        }
        v14 = v12;
        if (v12 <= 1)
          v14 = 1;
        if ((unsigned long)(a1 - v38) <= v14) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (v39) {
          if (1 < v12) {
            v12 -= 1;
            if ((v23 != 0x30) && (v23 != 0x2b)) {
              memset(v39,0x20,v12);
              v39 = &v39[v12];
            }
            else {
              memset(v39,0x30,v12);
              v39 = &v39[v12];
            }
          }
          *v39 = *v40;
          v39 = &v39[1];
        }
        v38 += v14;
        goto label_199e6;
      case 0x3a:
        for (v43 = 1; v40[v43] == ':'; v43 = v43 + 1) {
        }
        if (v40[v43] == 'z') {
          v40 = &v40[v43];
          goto label_19554;
        }
        break;
      case 0x41:
        if (!v8) {
          if (v7)
            v21 = 1;
          v4 = 0;
          goto label_179ef;
        }
        break;
      case 0x42:
        if (v8 != 'E') {
          if (v7)
            v21 = 1;
          v4 = 0;
          goto label_179ef;
        }
        break;
      case 0x43:
        if (v8 == 'E') goto label_179ef;
        v20 = *(int *)&a3->field_0x14 < -0x76c;
        v26 = (int)((unsigned int)((*(unsigned int *)&a3->field_0x14 >> 0x1f & (unsigned int)!(bool)v20) != 0) * -99 + *(int *)&a3->field_0x14) / 100 + 0x13;
        v24 = 2;
label_17da1:
        if (!v23)
          v23 = a5;
        if (v23 != 0x2b) { // branch-flip
label_17dee:
          v7 = 0;
        }
        else {
          v10 = (v24 != 2) ? 9999 : 99; // branch-flip
          if ((v26 <= v10) && (a6 <= v24)) goto label_17dee;
          v7 = 1;
        }
        goto label_17e3a;
      case 0x44:
        if (!v8) {
          v41 = "%m/%d/%y";
          goto label_177db;
        }
        break;
      case 0x46:
        if (v8) break;
        if ((v23) || (0 <= a6)) {
          v29 = a6 + -6;
          if (v29 <= -1)
            v29 = 0;
        }
        else {
          v23 = 0x2b;
          v29 = 4;
        }
        v41 = "%Y-%m-%d";
        goto label_177e5;
      case 0x47:
      case 0x56:
      case 0x67:
        if (v8 != 'E') {
          v9 = (0 <= *(int *)&a3->field_0x14) ? -100 : 300; // branch-flip
          v10 = v9 + *(int *)&a3->field_0x14;
          v33 = 0;
          v34 = sub_170bb(*(unsigned int *)&a3->field_0x1c,*(unsigned int *)&a3->field_0x18);
          if (0 <= v34) { // branch-flip
            v9 = !((v10 & 3) || ((!((int)v10 % 100) && ((int)v10 % 400))));
            v9 = sub_170bb(*(int *)&a3->field_0x1c - (v9 + 0x16d),*(unsigned int *)&a3->field_0x18);
            if (0 <= v9) {
              v33 = 1;
              v34 = v9;
            }
          }
          else {
            v33 = -1;
            v3 = *(unsigned int *)&a3->field_0x18;
            v9 = *(int *)&a3->field_0x1c;
            v11 = !((v10 - 1 & 3) || ((!((int)(v10 - 1) % 100) && ((int)(v10 - 1) % 400))));
            v34 = sub_170bb(v11 + 0x16d + v9,v3);
          }
          if (*v40 != 'G') { // branch-flip
            if (*v40 != 'g') {
              v24 = 2;
              v25 = v34 / 7 + 1;
              goto label_17e18;
            }
            v26 = (v33 + *(int *)&a3->field_0x14 % 100) % 100;
            v24 = 2;
            v20 = 0;
            if ((int)v26 <= -1) {
              v26 = (0xfffff894U - v33 <= *(int *)&a3->field_0x14) ? v26 + 100 : -v26; // branch-flip
            }
          }
          else {
            v24 = 4;
            v20 = *(int *)&a3->field_0x14 < 0xfffff894U - v33;
            v26 = v33 + *(int *)&a3->field_0x14 + 0x76c;
          }
          goto label_17da1;
        }
        break;
      case 0x48:
        if (v8 != 'E') {
          v24 = 2;
          v25 = *(unsigned int *)&a3->field_0x8;
          goto label_17e18;
        }
        break;
      case 0x49:
        if (v8 != 'E') {
          v24 = 2;
          v25 = v22;
          goto label_17e18;
        }
        break;
      case 0x4d:
        if (v8 != 'E') {
          v24 = 2;
          v25 = *(unsigned int *)&a3->field_0x4;
          goto label_17e18;
        }
        break;
      case 0x4e:
        if (v8 == 'E') break;
        v30 = a9;
        if (a6 <= 0)
          a6 = 9;
        v31 = 9;
        for (; (a6 < v31 || ((2 <= v31 && (!(v30 % 10))))); v30 = v30 / 10) {
          v31 -= 1;
        }
        for (v32 = v31; 0 < v32; v32 = v32 + -1) {
          v17[v32 + -1] = (char)v30 + (char)(v30 / 10) * '\xf6' + '0';
          v30 /= 10;
        }
        if (!v23)
          v23 = 0x30;
        v12 = (unsigned long)v31;
        if ((unsigned long)(a1 - v38) <= v12) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (v39) {
          if (v21) // branch-flip
            sub_17063(v39,v17,v12);
          else {
            memcpy(v39,v17,v12);
          }
          v39 = &v39[v12];
        }
        if ((v23 != 0x2d) && (0 <= a6 - v31))
          v14 = (unsigned long)(a6 - v31);
        else {
          v14 = 0;
        }
        v13 = v14;
        if (!v14)
          v13 = 0;
        if (a1 - (v38 + v12) <= v13) {
          *__errno_location() = 0x22;
          return 0;
        }
        if ((v39) && (v14)) {
          if ((v23 != 0x30) && (v23 != 0x2b)) {
            memset(v39,0x20,v14);
            v39 = &v39[v14];
          }
          else {
            memset(v39,0x30,v14);
            v39 = &v39[v14];
          }
        }
        v38 = v38 + v12 + v13;
        goto label_199e6;
      case 0x50:
        v4 = 1;
        v28 = 'p';
        goto label_18a25;
      case 0x52:
        v41 = "%H:%M";
        goto label_177db;
      case 0x53:
        if (v8 != 'E') {
          v24 = 2;
          v25 = *(unsigned int *)a3;
          goto label_17e18;
        }
        break;
      case 0x54:
        v41 = "%H:%M:%S";
label_177db:
        v29 = -1;
label_177e5:
        v12 = sub_1718e(0,0xffffffffffffffff,v41,a3,(unsigned char)v21,v23,v29,v5,v6,a9);
        if ((v23 != 0x2d) && (0 <= a6))
          v14 = (unsigned long)a6;
        else {
          v14 = 0;
        }
        v13 = v14;
        if (v14 <= v12)
          v13 = v12;
        if ((unsigned long)(a1 - v38) <= v13) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (v39) {
          if (v12 < v14) {
            v14 -= v12;
            if ((v23 != 0x30) && (v23 != 0x2b)) {
              memset(v39,0x20,v14);
              v39 = &v39[v14];
            }
            else {
              memset(v39,0x30,v14);
              v39 = &v39[v14];
            }
          }
          sub_1718e(v39,a1 - v38,v41,a3,(unsigned char)v21,v23,v29,v5,v6,a9);
          v39 = &v39[v12];
        }
        v38 += v13;
        goto label_199e6;
      case 0x55:
        if (v8 != 'E') {
          v24 = 2;
          v25 = ((*(int *)&a3->field_0x1c - *(int *)&a3->field_0x18) + 7) / 7;
          goto label_17e18;
        }
        break;
      case 0x57:
        if (v8 != 'E') {
          v24 = 2;
          v25 = ((*(int *)&a3->field_0x1c - (*(int *)&a3->field_0x18 + 6) % 7) + 7) / 7;
          goto label_17e18;
        }
        break;
      case 0x58:
        goto label_17d15;
      case 0x59:
        if (v8 == 'E') goto label_179ef;
        if (v8 != 'O') {
          v24 = 4;
          v20 = *(int *)&a3->field_0x14 < -0x76c;
          v26 = *(int *)&a3->field_0x14 + 0x76c;
          goto label_17da1;
        }
        break;
      case 0x5a:
        if (v7)
          v21 = 0;
        v12 = strlen(v37);
        if ((v23 != 0x2d) && (0 <= a6))
          v14 = (unsigned long)a6;
        else {
          v14 = 0;
        }
        v13 = v14;
        if (v14 <= v12)
          v13 = v12;
        if ((unsigned long)(a1 - v38) <= v13) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (v39) {
          if (v12 < v14) {
            v14 -= v12;
            if ((v23 != 0x30) && (v23 != 0x2b)) {
              memset(v39,0x20,v14);
              v39 = &v39[v14];
            }
            else {
              memset(v39,0x30,v14);
              v39 = &v39[v14];
            }
          }
          if (v7)
            sub_1700b(v39,v37,v12);
          else if (v21) // branch-flip
            sub_17063(v39,v37,v12);
          else {
            memcpy(v39,v37,v12);
          }
          v39 = &v39[v12];
        }
        v38 += v13;
        goto label_199e6;
      case 0x61:
        if (!v8) {
          if (v7)
            v21 = 1;
          v4 = 0;
          goto label_179ef;
        }
        break;
      case 0x62:
      case 0x68:
        if (v7)
          v21 = 1;
        v4 = 0;
        if (v8 != 'E') goto label_179ef;
        break;
      case 99:
        goto label_17d15;
      case 100:
        if (v8 != 'E') {
          v24 = 2;
          v25 = *(unsigned int *)&a3->field_0xc;
          goto label_17e18;
        }
        break;
      case 0x65:
        if (v8 != 'E') {
          v25 = *(unsigned int *)&a3->field_0xc;
label_17e02:
          v24 = 2;
          if (!v23)
            v23 = 0x5f;
          goto label_17e18;
        }
        break;
      case 0x6a:
        if (v8 != 'E') {
          v24 = 3;
          v20 = *(int *)&a3->field_0x1c < -1;
          v26 = *(int *)&a3->field_0x1c + 1;
          goto label_17e33;
        }
        break;
      case 0x6b:
        if (v8 != 'E') {
          v25 = *(unsigned int *)&a3->field_0x8;
          goto label_17e02;
        }
        break;
      case 0x6c:
        if (v8 != 'E') {
          v25 = v22;
          goto label_17e02;
        }
        break;
      case 0x6d:
        if (v8 != 'E') {
          v24 = 2;
          v20 = *(int *)&a3->field_0x10 < -1;
          v26 = *(int *)&a3->field_0x10 + 1;
          goto label_17e33;
        }
        break;
      case 0x6e:
        if ((v23 != 0x2d) && (0 <= a6))
          v12 = (unsigned long)a6;
        else {
          v12 = 0;
        }
        v14 = v12;
        if (v12 <= 1)
          v14 = 1;
        if ((unsigned long)(a1 - v38) <= v14) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (v39) {
          if (1 < v12) {
            v12 -= 1;
            if ((v23 != 0x30) && (v23 != 0x2b)) {
              memset(v39,0x20,v12);
              v39 = &v39[v12];
            }
            else {
              memset(v39,0x30,v12);
              v39 = &v39[v12];
            }
          }
          *v39 = '\n';
          v39 = &v39[1];
        }
        v38 += v14;
        goto label_199e6;
      case 0x70:
label_18a25:
        if (v7) {
          v21 = 0;
          v4 = 1;
        }
        goto label_179ef;
      case 0x71:
        v24 = 1;
        v20 = 0;
        v26 = (*(int *)&a3->field_0x10 * 0xb >> 5) + 1;
        goto label_17e33;
      case 0x72:
        goto label_179ef;
      case 0x73:
        v19 = *(unsigned long *)a3;
        v46 = *(unsigned long *)&a3->field_0x8;
        v47 = *(unsigned long *)&a3->field_0x10;
        v49 = *(unsigned long *)&a3->field_0x20;
        v50 = *(unsigned long *)&a3->field_0x28;
        v51 = *(unsigned long *)&a3->field_0x30;
        v48._0_4_ = (unsigned int)*(unsigned long *)&a3->field_0x18;
        v48 = CONCAT44(0xffffffff,(unsigned int)v48);
        v45 = sub_1d983(v6,&v19);
        if (v48 <= -1) {
          *__errno_location() = 0x4b;
          return 0;
        }
        v42 = &v18[1];
        v20 = -(char)(v45 >> 0x3f);
        do {
          v8 = (char)v45 + (char)(v45 / 10) * '\xf6';
          v45 /= 10;
          if (v20)
            v8 = -v8;
          v42 = &v42[-1];
          *v42 = v8 + '0';
        } while (v45);
        v24 = 1;
        v7 = 0;
        goto label_17f1d;
      case 0x74:
        if ((v23 != 0x2d) && (0 <= a6))
          v12 = (unsigned long)a6;
        else {
          v12 = 0;
        }
        v14 = v12;
        if (v12 <= 1)
          v14 = 1;
        if ((unsigned long)(a1 - v38) <= v14) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (v39) {
          if (1 < v12) {
            v12 -= 1;
            if ((v23 != 0x30) && (v23 != 0x2b)) {
              memset(v39,0x20,v12);
              v39 = &v39[v12];
            }
            else {
              memset(v39,0x30,v12);
              v39 = &v39[v12];
            }
          }
          *v39 = '\t';
          v39 = &v39[1];
        }
        v38 += v14;
        goto label_199e6;
      case 0x75:
        v24 = 1;
        v25 = (*(int *)&a3->field_0x18 + 6) % 7 + 1;
label_17e18:
        v20 = (bool)(v25 >> 0x1f);
        v26 = v25;
label_17e33:
        v7 = 0;
label_17e3a:
        v27 = 0;
label_17e44:
        if ((v8 == 'O') && ((bool)v20 != 1)) goto label_179ef;
        v42 = &v18[1];
        if ((bool)v20)
          v26 = -v26;
        do {
          if (v27 & 1) {
            v42 = &v42[-1];
            *v42 = ':';
          }
          v27 = (int)v27 >> 1;
          v42 = &v42[-1];
          *v42 = (char)v26 + (char)(v26 / 10) * '\xf6' + '0';
          v26 /= 10;
        } while ((v26) || (v27));
label_17f1d:
        if (!v23)
          v23 = 0x30;
        if (a6 <= -1)
          a6 = v24;
        if (v20) // branch-flip
          v8 = '-';
        else {
          v8 = (v7) ? '+' : '\0';
        }
        v9 = ((int)v17 + 0x17) - (int)v42;
        v11 = (a6 - (unsigned int)(v8 != '\0')) - v9;
        if ((v23 == 0x2d) || (v11 <= 0))
          v11 = 0;
        if (v8) {
          if (v23 == 0x5f) {
            if (v39) {
              memset(v39,0x20,(long)v11);
              v39 = &v39[v11];
            }
            v38 += v11;
            a6 -= v11;
          }
          if ((unsigned long)(a1 - v38) <= 1) {
            *__errno_location() = 0x22;
            return 0;
          }
          if (v39) {
            *v39 = v8;
            v39 = &v39[1];
          }
          v38 += 1;
          a6 -= 1;
        }
        v12 = (unsigned long)v9;
        if ((v23 != 0x2d) && (0 <= a6))
          v14 = (unsigned long)a6;
        else {
          v14 = 0;
        }
        v13 = v14;
        if (v14 <= v12)
          v13 = v12;
        if ((unsigned long)(a1 - v38) <= v13) {
          *__errno_location() = 0x22;
          return 0;
        }
        if (v39) {
          if (v12 < v14) {
            v14 -= v12;
            if ((v23 != 0x30) && (v23 != 0x2b)) {
              memset(v39,0x20,v14);
              v39 = &v39[v14];
            }
            else {
              memset(v39,0x30,v14);
              v39 = &v39[v14];
            }
          }
          if (v21) // branch-flip
            sub_17063(v39,v42,v12);
          else {
            memcpy(v39,v42,v12);
          }
          v39 = &v39[v12];
        }
        v38 += v13;
        goto label_199e6;
      case 0x77:
        if (v8 != 'E') {
          v24 = 1;
          v25 = *(unsigned int *)&a3->field_0x18;
          goto label_17e18;
        }
        break;
      case 0x78:
label_17d15:
        if (v8 != 'O') goto label_179ef;
        break;
      case 0x79:
        if (v8 != 'E') {
          v35 = *(int *)&a3->field_0x14 % 100;
          if ((int)v35 <= -1) {
            v35 = (-0x76c <= *(int *)&a3->field_0x14) ? v35 + 100 : -v35; // branch-flip
          }
          v24 = 2;
          v20 = 0;
          v26 = v35;
          goto label_17da1;
        }
label_179ef:
        v15[0] = ' ';
        v44 = &v15[2];
        v15[1] = 0x25;
        if (v8) {
          v44 = v16;
          v15[2] = v8;
        }
        *v44 = v28;
        v44[1] = '\0';
        v12 = strftime(v17,0x400,v15,a3);
        if (v12) {
          v12 -= 1;
          if ((v23 != 0x2d) && (0 <= a6))
            v14 = (unsigned long)a6;
          else {
            v14 = 0;
          }
          v13 = v14;
          if (v14 <= v12)
            v13 = v12;
          if ((unsigned long)(a1 - v38) <= v13) {
            *__errno_location() = 0x22;
            return 0;
          }
          if (v39) {
            if (v12 < v14) {
              v14 -= v12;
              if ((v23 != 0x30) && (v23 != 0x2b)) {
                memset(v39,0x20,v14);
                v39 = &v39[v14];
              }
              else {
                memset(v39,0x30,v14);
                v39 = &v39[v14];
              }
            }
            if (v4)
              sub_1700b(v39,&v17[1],v12);
            else if (v21) // branch-flip
              sub_17063(v39,&v17[1],v12);
            else {
              memcpy(v39,&v17[1],v12);
            }
            v39 = &v39[v12];
          }
          v38 += v13;
        }
        goto label_199e6;
      case 0x7a:
        v43 = 0;
label_19554:
        if (*(int *)&a3->field_0x20 < 0) goto label_199e6;
        v9 = (int)*(unsigned long *)&a3->field_0x28;
        v20 = !((0 <= v9) && ((v9 || (*v37 != '-'))));
        v26 = v9 / 0xe10;
        v11 = (v9 / 0x3c) % 0x3c;
        if (v43 != 3) {
          if (v43 <= 3) {
            if (v43 == 2) goto label_19709;
            if (v43 <= 2) {
              if (!v43) {
                v24 = 5;
                v27 = 0;
                v26 = v11 + v26 * 100;
                goto label_17d95;
              }
              if (v43 == 1) goto label_196d8;
            }
          }
          break;
        }
        if (v9 % 0x3c) { // branch-flip
label_19709:
          v24 = 9;
          v27 = 0x14;
          v26 = v9 % 0x3c + v26 * 10000 + v11 * 100;
        }
        else if (v11) { // branch-flip
label_196d8:
          v24 = 6;
          v27 = 4;
          v26 = v11 + v26 * 100;
        }
        else {
          v24 = 3;
          v27 = 0;
        }
label_17d95:
        v7 = 1;
        goto label_17e44;
      
    }
    v36 = 1;
    while (v40[1 - v36] != '%') {
      v36 += 1;
    }
    v12 = (unsigned long)v36;
    if ((v23 != 0x2d) && (0 <= a6))
      v14 = (unsigned long)a6;
    else {
      v14 = 0;
    }
    v13 = v14;
    if (v14 <= v12)
      v13 = v12;
    if ((unsigned long)(a1 - v38) <= v13) {
      *__errno_location() = 0x22;
      return 0;
    }
    if (v39) {
      if (v12 < v14) {
        v14 -= v12;
        if ((v23 != 0x30) && (v23 != 0x2b)) {
          memset(v39,0x20,v14);
          v39 = &v39[v14];
        }
        else {
          memset(v39,0x30,v14);
          v39 = &v39[v14];
        }
      }
      if (v21) // branch-flip
        sub_17063(v39,&v40[1 - v36],v12);
      else {
        memcpy(v39,&v40[1 - v36],v12);
      }
      v39 = &v39[v12];
    }
    v38 += v13;
label_199e6:
    a6 = -1;
    v40 = &v40[1];
  } while( true );
}

// Function: sub_19a53 @ 0x19a53
void sub_19a53(char *a0)
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
  dat_3d8a8 = v2;
  program_invocation_name = v2;
}

// Function: sub_19b3e @ 0x19b3e
char sub_19b3e(char *a0,char *a1)
{
  bool v1;
  char *v10; // stack - 0xc0
  char v11; // stack - 0xb5
  char v12; // stack - 0x8c
  char *v13; // stack - 0x88
  long v14; // stack - 0x80
  unsigned int v15; // stack - 0x78
  unsigned int v16; // stack - 0x74
  long v17; // stack - 0x70
  unsigned long v18; // stack - 0x68
  unsigned long v19; // stack - 0x60
  bool v2;
  long v20; // stack - 0x50
  undefined3 v21; // stack - 0x4b
  char *v22; // stack - 0x48
  long v23; // stack - 0x40
  unsigned long v24; // stack - 0x38
  unsigned long v25; // stack - 0x30
  bool v3;
  char *v4; // rax
  char *v5; // rax
  unsigned long v6; // rax
  char v7 [8];
  char v8 [4]; // stack - 0x98
  char *v9; // stack - 0x58
  
  v4 = (char *)sub_1bc48(a1,2);
  v11 = 0;
  v10 = a0;
  while( true ) {
    while( true ) {
      if ((!*v10) || (v5 = (char *)sub_1f057(v10,v4), !v5)) {
        free(v4); // return-dupe
        return v11;
      }
      if (2 <= __ctype_get_mb_cur_max()) break;
      v3 = 1;
      if ((v10 < v5) && (*(unsigned short *)((unsigned long)(unsigned char)v5[-1] * 2 + *(long *)__ctype_b_loc()) & 8))
        v3 = 0;
      v6 = strlen(v4);
      v1 = 1;
      if ((v5[v6]) && (*(unsigned short *)((unsigned long)(unsigned char)v5[v6] * 2 + *(long *)__ctype_b_loc()) & 8))
        v1 = 0;
      if ((v3) && (v1)) {
        v11 = 1;
        free(v4);
        return v11;
      }
      if (!*v5) {
        free(v4);
        return v11;
      }
      v10 = &v5[1];
    }
    v13 = v10;
    v8[0] = 0;
    memset(v7,0,8);
    v12 = 0;
    v3 = 1;
    if (v13 < v5) {
      do {
        v12 = 0;
        sub_16d35(v8);
        if (((char)v15 != '\x01') || (v16))
          v1 = 1;
        else {
          v1 = 0;
        }
        if (!v1)
          abort(); // no-return
        v9 = v13;
        v20 = v14;
        v22 = (char *)CONCAT44(v16,v15);
        v23 = v17;
        v24 = v18;
        v25 = v19;
        v13 = &v13[v14];
        v12 = 0;
      } while (v13 < v5);
      if (((char)v15) && (iswalnum(v16)))
        v3 = 0;
    }
    v13 = v5;
    v8[0] = 0;
    memset(v7,0,8);
    v12 = 0;
    v22 = v4;
    v9 = (char *)((unsigned long)v9 & 0xffffffffffffff00);
    memset((void *)((long)&v9 + 4),0,8);
    v20 = (undefined5)(unsigned int)v20;
    while( true ) {
      sub_16d35(&v9);
      if (((char)v24 != '\x01') || (v24._4_4_))
        v1 = 1;
      else {
        v1 = 0;
      }
      if (!v1) break;
      sub_16d35(v8);
      if (((char)v15 != '\x01') || (v16))
        v1 = 1;
      else {
        v1 = 0;
      }
      if (!v1)
        abort(); // no-return
      v13 = &v13[v14];
      v12 = 0;
      v22 = &v22[v23];
      v20 = (undefined5)(unsigned int)v20;
    }
    v1 = 1;
    sub_16d35(v8);
    if (((char)v15 != '\x01') || (v16))
      v2 = 1;
    else {
      v2 = 0;
    }
    if (v2) {
      v9 = v13;
      v20 = v14;
      v22 = (char *)CONCAT44(v16,v15);
      v23 = v17;
      v24 = v18;
      v25 = v19;
      if (((char)v15) && (iswalnum(v16)))
        v1 = 0;
    }
    if ((v3) && (v1)) break;
    v13 = v5;
    v8[0] = 0;
    memset(v7,0,8);
    v12 = 0;
    sub_16d35(v8);
    if (((char)v15 != '\x01') || (v16))
      v3 = 1;
    else {
      v3 = 0;
    }
    if (!v3) {
      free(v4);
      return v11;
    }
    v10 = &v5[v14];
  }
  v11 = 1;
  free(v4);
  return v11;
}

// Function: sub_1a014 @ 0x1a014
char * sub_1a014(char *a0)
{
  char *v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = gettext(a0);
  if (v1 != a0) {
    if (sub_19b3e(v1,a0)) // branch-flip
      a0 = v1;
    else {
      v2 = strlen(v1);
      v3 = (char *)sub_1c945(strlen(a0) + v2 + 4);
      sprintf(v3,"%s (%s)",v1,a0);
      a0 = v3;
    }
  }
  return a0;
}

// Function: sub_1a0ba @ 0x1a0ba
char * sub_1a0ba(char *a0,char *a1)
{
  char *v1; // rax
  char *v2;
  unsigned long v3; // rax
  char *v4; // rax
  char *v5;
  char *v6; // stack - 0x70
  char *v7; // stack - 0x68
  char *v8; // stack - 0x60
  char *v9; // stack - 0x58
  
  v1 = gettext(a0);
  v2 = (char *)sub_1e16d();
  v6 = NULL;
  v7 = NULL;
  v9 = NULL;
  if (sub_1dd22(v2,"UTF-8")) { // branch-flip
    v6 = (char *)sub_1d0a6(a1,"UTF-8",v2);
    v8 = v6;
    v3 = strlen(v2);
    v4 = (char *)sub_1c945(v3 + 0xb);
    memcpy(v4,v2,v3);
    memcpy(&v4[v3],"//TRANSLIT",0xb);
    v2 = (char *)sub_1d0a6(a1,"UTF-8",v4);
    free(v4);
    if (v2) {
      if (strchr(v2,0x3f)) // branch-flip
        free(v2);
      else {
        v7 = v2;
        v9 = v2;
      }
    }
  }
  else {
    v8 = a1;
    v9 = a1;
  }
  if (v8) // branch-flip
    v2 = v8;
  else if (v9) // branch-flip
    v2 = v9;
  else {
    v2 = a0;
  }
  if (!strcmp(v1,a0)) {
    if ((v6) && (v6 != v2))
      free(v6);
    if ((v7) && (v7 != v2))
      free(v7);
    return v2;
  }
  if (((sub_19b3e(v1,a0)) || ((v8 && (sub_19b3e(v1,v8))))) || ((v9 && (sub_19b3e(v1,v9))))) {
    if (v6)
      free(v6);
    if (v7)
      free(v7);
    v5 = v1;
  }
  else {
    v3 = strlen(v1);
    v5 = (char *)sub_1c945(strlen(v2) + v3 + 4);
    sprintf(v5,"%s (%s)",v1,v2);
    if (v6)
      free(v6);
    if (v7)
      free(v7);
  }
  return v5;
}

// Function: sub_1a39d @ 0x1a39d
char * sub_1a39d(int a0)
{
  return setlocale(a0,NULL);
}

// Function: sub_1a3c5 @ 0x1a3c5
unsigned long sub_1a3c5(unsigned int a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_1a39d(a0);
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

// Function: sub_1a47d @ 0x1a47d
void sub_1a47d(unsigned int a0,char *a1,unsigned long a2)
{
  sub_1a3c5(a0,a1,a2);
}

// Function: sub_1a4ab @ 0x1a4ab
void sub_1a4ab(unsigned int a0)
{
  sub_1a39d(a0);
}

// Function: sub_1a4c6 @ 0x1a4c6
void sub_1a4c6(void)
{
  dat_3d8b0 = sub_1f7c5(NULL);
  sub_1f838(dat_3d8b0,1);
}

// Function: sub_1a4f6 @ 0x1a4f6
void sub_1a4f6(char *a0)
{
  long v1;
  
  if (!dat_3d8b0)
    sub_1a4c6();
  v1 = dat_3d8b0;
  sub_20bda(0,0,a0,strlen(a0),v1);
}

// Function: sub_1a550 @ 0x1a550
long sub_1a550(long a0,char *a1)
{
  long v1;
  unsigned long v2; // rax
  
  if (!dat_3d8b0)
    sub_1a4c6();
  v1 = dat_3d8b0;
  v2 = strlen(a1);
  return sub_20bda(a0,0xffffffffffffffff,a1,v2,v1) + a0;
}

// Function: sub_1a5b3 @ 0x1a5b3
void sub_1a5b3(char *a0)
{
  long v1;
  
  if (!dat_3d8b0)
    sub_1a4c6();
  v1 = dat_3d8b0;
  sub_20c82(a0,strlen(a0),v1);
}

// Function: sub_1a603 @ 0x1a603
char * sub_1a603(long *a0)
{
  char *v1;
  char *v2; // rax
  long *v3; // stack - 0x28
  unsigned long v4; // stack - 0x20
  char *v5; // stack - 0x18
  char *v6; // stack - 0x10
  
  if (*a0) { // branch-flip
    v4 = 0;
    v3 = a0;
    do {
      v1 = (char *)*v3;
      v4 += sub_1a4f6(v1) + 1;
      v3 = &v3[1];
    } while (*v3);
    v6 = (char *)sub_1c945(v4);
    v5 = v6;
    v3 = a0;
    while( true ) {
      v2 = (char *)sub_1a550(v5,(char *)*v3);
      v3 = &v3[1];
      if (!*v3) break;
      v5 = &v2[1];
      *v2 = 0x20;
    }
    *v2 = 0;
  }
  else {
    v6 = (char *)sub_1ced8("");
  }
  return v6;
}

// Function: sub_1a6d9 @ 0x1a6d9
void sub_1a6d9(void)
{
  return;
}

// Function: sub_1a6e4 @ 0x1a6e4
void sub_1a6e4(unsigned long a0) // return-dupe
{
  char v1 [8];
  long v2; // stack - 0x30
  
  if ((int)sub_1ba8f(a0,v1) <= -1)
    return;
  dat_3d8b8 = v2 + -1;
}

// Function: sub_1a740 @ 0x1a740
void sub_1a740(unsigned long a0,void *a1,void *a2) // return-dupe x2, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_1a740
{
  unsigned long v1;
  unsigned long v2;
  int v3;
  int v4; // eax
  unsigned int v5; // eax
  unsigned long v6; // stack - 0x38
  bool v7;
  long v8; // stack - 0x30
  void *v9; // stack - 0x28
  
  v1 = *(unsigned long *)((long)a1 + 0x10);
  if ((dat_3d8d8) && (v3 = (*dat_3d8d8)(v1,0), v3))
    return;
  if ((dat_3d8c0) && (v2 = *(unsigned long *)((long)a2 + 0xa0), dat_3d8b8)) {
    v3 = *__errno_location();
    v4 = sub_1ba8f(dat_3d8b8,&v6);
    *__errno_location() = v3;
    if (0 <= v4) {
      if (v6 <= v1) // branch-flip
        v7 = v1 <= (unsigned long)(v8 - 1U);
      else {
        v7 = (*v9)(v1,&v6) != 0;
      }
      if (v7) {
        v5 = ((dat_3d8c8 <= v2) && (v2 <= dat_3d8d0 + dat_3d8c8));
        (*dat_3d8c0)(v5,a2);
      }
    }
  }
  if ((dat_3d8d8) && (v3 = (*dat_3d8d8)(v1,1), v3))
    return;
  signal(0xb,0);
}

// Function: sub_1a8e3 @ 0x1a8e3
void sub_1a8e3(int a0)
{
  void *v1; // stack - 0xa8
  sigset_t v2;
  undefined4 v3; // stack - 0x20
  
  v1 = sub_1a740;
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
  v3 = 4;
  if (dat_3d8c0)
    v3 = 0x8000004;
  sigaction(a0,(sigaction *)&v1,NULL);
}

// Function: sub_1ab41 @ 0x1ab41
unsigned long sub_1ab41(unsigned long a0)
{
  dat_3d8d8 = a0;
  sub_1a8e3(0xb);
  return 0;
}

// Function: sub_1ab74 @ 0x1ab74
void sub_1ab74(void) // return-dupe
{
  dat_3d8d8 = 0;
  if (dat_3d8c0) {
    dat_3d8d8 = 0;
    return;
  }
  signal(0xb,0);
}

// Function: sub_1abb0 @ 0x1abb0
unsigned long sub_1abb0(void *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  sub_1a6d9();
  (*a0)(a1,a2,a3);
  return 1;
}

// Function: sub_1abf1 @ 0x1abf1
unsigned long sub_1abf1(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe x2
{
  unsigned long v1; // stack - 0x28
  unsigned int v2; // stack - 0x20
  unsigned long v3; // stack - 0x18
  
  if ((!dat_3d8b8) && (sub_1a6e4(&v1), !dat_3d8b8))
    return 0xffffffff;
  v2 = 0;
  dat_3d8c0 = a0;
  dat_3d8c8 = a1;
  dat_3d8d0 = a2;
  v1 = a1;
  v3 = a2;
  if (sigaltstack(&v1,NULL) <= -1)
    return 0xffffffff;
  sub_1a8e3(0xb);
  return 0;
}

// Function: sub_1acc3 @ 0x1acc3
void sub_1acc3(void) // return-dupe
{
  char v1 [8];
  unsigned int v2; // stack - 0x20
  
  dat_3d8c0 = 0;
  if (dat_3d8d8) // branch-flip
    sub_1a8e3(0xb);
  else {
    signal(0xb,0);
  }
  v2 = 2;
  if (0 <= sigaltstack(v1,NULL))
    return;
  perror("gnulib sigsegv (stackoverflow_deinstall_handler)");
}

// Function: sub_1ad60 @ 0x1ad60
bool sub_1ad60(long a0,struct_12 *a1)
{
  return (unsigned long)(a1->field_0x0 - a0) <= (unsigned long)(a1->field_0x0 - a1->field_0x18) >> 1;
}

// Function: sub_1ada1 @ 0x1ada1
unsigned long sub_1ada1(struct_13 *a0,char *a1) // return-dupe x3, return-dupe
{
  void *v1;
  long v2;
  long v3;
  int v4; // eax
  int v5; // stack - 0x20
  unsigned long v6; // stack - 0x18
  unsigned long v7; // stack - 0x10
  
  v5 = open(a1,0);
  if (v5 <= -1)
    return 0xffffffff;
  a0->field_0x0 = 0;
  a0->field_0x10 = 0;
  v6 = 0;
  a0->field_0x18 = &a0[1];
  v7 = 1;
  a0->field_0x20 = 0;
  a0->field_0x30 = 0;
  a0->field_0x38 = 0;
  do {
    if (0x104a <= v7) { // branch-flip
      v1 = a0->field_0x18;
      v4 = (int)read(v5,v1,v7);
      if ((0 <= v4) || (*__errno_location() != 4)) {
        if (1 <= v4) {
          if ((unsigned long)(long)(v4 + 0x1049) <= v7) {
            a0->field_0x8 = (long)v4;
            while( true ) {
              v2 = a0->field_0x8;
              v1 = a0->field_0x18;
              v3 = a0->field_0x8;
              v4 = (int)read(v5,(void *)((long)v1 + v3),v7 - v2);
              if ((v4 <= -1) && (*__errno_location() == 4)) goto label_1b050;
              if (v4 < 0) goto label_1b0a2;
              if (v7 - a0->field_0x8 < (unsigned long)(long)(v4 + 0x1049)) break;
              if (!v4) {
                close(v5);
                return 0;
              }
              a0->field_0x8 = a0->field_0x8 + (long)v4;
            }
          }
          goto label_1af50;
        }
label_1b0a2:
        close(v5);
        goto label_1b0af;
      }
    }
    else {
label_1af50:
      if (v6) { // branch-flip
        v7 <<= 1;
        if (!v7) goto label_1b0a2;
        if (a0->field_0x20)
          munmap((void *)a0->field_0x20,a0->field_0x28);
      }
      else {
        v6 = (unsigned long)getpagesize();
        for (v7 = v6; v7 <= 0x1049; v7 = v7 << 1) {
        }
      }
      a0->field_0x20 = mmap(0,v7,3,0x22,0xffffffff,0);
      if (a0->field_0x20 == -1) {
        close(v5);
        return 0xffffffff;
      }
      a0->field_0x28 = v7;
      a0->field_0x30 = a0->field_0x20;
      a0->field_0x38 = a0->field_0x30 + v7;
      a0->field_0x18 = (void *)a0->field_0x20;
    }
label_1b050:
    if ((long)lseek(v5,0,0) <= -1) {
      close(v5);
      v5 = open(a1,0);
      if (v5 < 0) {
label_1b0af:
        if (!a0->field_0x20)
          return 0xffffffff;
        munmap((void *)a0->field_0x20,a0->field_0x28);
        return 0xffffffff;
      }
    }
  } while( true );
}

// Function: sub_1b0de @ 0x1b0de
unsigned long sub_1b0de(struct_14 *a0) // early-return
{
  if (a0->field_0x0 == a0->field_0x8) {
    a0->field_0x10 = 1;
    return 0xffffffff;
  }
  return (unsigned long)*(unsigned char *)(a0->field_0x0 + a0->field_0x18);
}

// Function: sub_1b12a @ 0x1b12a
int sub_1b12a(struct_14 *a0)
{
  int v1; // eax
  
  v1 = sub_1b0de(a0);
  if (0 <= v1)
    a0->field_0x0 = a0->field_0x0 + 1;
  return v1;
}

// Function: sub_1b166 @ 0x1b166
unsigned long sub_1b166(struct_14 *a0,long *a1) // return-dupe
{
  int v1; // eax
  int v2; // stack - 0x18
  long v3; // stack - 0x10
  
  v3 = 0;
  v2 = 0;
  do {
    v1 = sub_1b0de(a0);
    if ((0x30 <= v1) && (v1 <= 0x39))
      v3 = (long)(v1 + -0x30) + v3 * 0x10;
    else if ((0x41 <= v1) && (v1 <= 0x46))
      v3 = (long)(v1 + -0x37) + v3 * 0x10;
    else {
      if ((v1 <= 0x60) || (0x66 < v1)) {
        if (!v2)
          return 0xffffffff;
        *a1 = v3;
        return 0;
      }
      v3 = (long)(v1 + -0x57) + v3 * 0x10;
    }
    sub_1b12a(a0);
    v2 += 1;
  } while( true );
}

// Function: sub_1b242 @ 0x1b242
void sub_1b242(void *a0) // return-dupe
{
  if (!*(long *)((long)a0 + 0x20))
    return;
  munmap(*(void **)((long)a0 + 0x20),*(unsigned long *)((long)a0 + 0x28));
}

// Function: sub_1b27d @ 0x1b27d
unsigned long sub_1b27d(struct_15 *a0) // return-dupe
{
  bool v1;
  int v2; // eax
  char v3 [48];
  unsigned long v4; // stack - 0x78
  long v5; // stack - 0x70
  unsigned long v6; // stack - 0x68
  unsigned long v7; // stack - 0x60
  unsigned long v8; // stack - 0x28
  unsigned long v9; // stack - 0x20
  
  if ((int)sub_1ada1(v3,"/proc/self/maps") < 0)
    return 0xffffffff;
  v6 = v8;
  v7 = v9;
  do {
    while( true ) {
      if ((((int)sub_1b166(v3,&v4) < 0) || (sub_1b12a(v3) != 0x2d)) || ((int)sub_1b166(v3,&v5) < 0)) {
        sub_1b242(v3); // return-dupe
        return 0;
      }
      do {
        v2 = sub_1b12a(v3);
        if ((v2 != -1) && (v2 != 10))
          v1 = 1;
        else {
          v1 = 0;
        }
      } while (v1);
      if ((v6 < v4) || ((unsigned long)(v5 - 1U) < v7 - 1)) break;
      if (((v4 < v6) && (v2 = sub_1ba02(a0,v4,v6), v2)) || ((v7 - 1 < (unsigned long)(v5 - 1U) && (v2 = sub_1ba02(a0,v7,v5), v2)))) {
        sub_1b242(v3);
        return 0;
      }
    }
    v2 = sub_1ba02(a0,v4,v5);
  } while (!v2);
  sub_1b242(v3);
  return 0;
}

// Function: sub_1b41e @ 0x1b41e
unsigned long sub_1b41e(struct_15 *a0) // return-dupe
{
  if (!sub_1b27d(a0))
    return 0;
  return 0xffffffff;
}

// Function: sub_1b451 @ 0x1b451
void sub_1b451(void)
{
  dat_3d8e0 = (long)getpagesize();
}

// Function: sub_1b46a @ 0x1b46a
unsigned int sub_1b46a(void *a0)
{
  unsigned int v1; // eax
  long v2;
  
  v1 = mincore(a0,dat_3d8e0,&v2);
  return ~v1 >> 0x1f;
}

// Function: sub_1b4c1 @ 0x1b4c1
unsigned long sub_1b4c1(unsigned long a0)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  long v3; // rcx
  char v4 [1032];
  unsigned long v5; // stack - 0x440
  unsigned long v6; // stack - 0x438
  
  v6 = 0x400;
  v5 = a0;
  while( true ) {
    if (!v5)
      return 0;
    if (v5 / dat_3d8e0 < v6)
      v6 = v5 / dat_3d8e0;
    v1 = dat_3d8e0 * v6;
    v3 = dat_3d8e0 * v6;
    if (mincore((void *)(v5 - v3),v1,v4) <= -1) break;
    v5 -= dat_3d8e0 * v6;
  }
  while (v6 != 1) {
    v1 = v6 + 1 >> 1;
    v2 = dat_3d8e0 * v1;
    v3 = dat_3d8e0 * v1;
    if (0 <= mincore((void *)(v5 - v3),v2,v4)) { // branch-flip
      v5 -= dat_3d8e0 * v1;
      v6 >>= 1;
    }
    else {
      v6 = v1;
    }
  }
  return v5;
}

// Function: sub_1b675 @ 0x1b675
void * sub_1b675(long a0)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char v3 [1032];
  void *v4; // stack - 0x440
  unsigned long v5; // stack - 0x438
  
  v5 = 0x400;
  v4 = (void *)(a0 + dat_3d8e0);
  while( true ) {
    if (!v4)
      return NULL;
    if ((unsigned long)-(long)v4 / dat_3d8e0 < v5)
      v5 = (unsigned long)-(long)v4 / dat_3d8e0;
    v1 = dat_3d8e0 * v5;
    if (mincore(v4,v1,v3) <= -1) break;
    v4 = (void *)((long)v4 + dat_3d8e0 * v5);
  }
  while (v5 != 1) {
    v1 = v5 + 1 >> 1;
    v2 = dat_3d8e0 * v1;
    if (0 <= mincore(v4,v2,v3)) { // branch-flip
      v4 = (void *)((long)v4 + dat_3d8e0 * v1);
      v5 >>= 1;
    }
    else {
      v5 = v1;
    }
  }
  return v4;
}

// Function: sub_1b80d @ 0x1b80d
unsigned long sub_1b80d(unsigned long a0,unsigned long a1) // return-dupe
{
  long v1; // rax
  unsigned long v2; // rax
  long v3; // rax
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  void *v6; // stack - 0x20
  
  v1 = dat_3d8e0 * (a0 / dat_3d8e0);
  v2 = (dat_3d8e0 * (a1 / dat_3d8e0 + 1) - v1) / dat_3d8e0;
  for (v4 = 1; v4 < v2; v4 = v4 << 1) {
  }
  do {
    v4 >>= 1;
    if (!v4)
      return 1;
    v3 = dat_3d8e0 * v4;
    v6 = (void *)(v3 + v1);
    for (v5 = v4; v5 < v2; v5 = v5 + v4 * 2) {
      if (sub_1b46a(v6))
        return 0;
      v6 = (void *)((long)v6 + v3 * 2);
    }
  } while( true );
}

// Function: sub_1b91e @ 0x1b91e
unsigned long sub_1b91e(unsigned long a0,long *a1) // early-return
{
  long v1;
  unsigned long v2; // rax
  
  v2 = a0 * 2 - *a1;
  if (a0 < v2)
    return 0;
  v1 = *a1;
  return sub_1b80d(v2,v1 - 1);
}

// Function: sub_1b97b @ 0x1b97b
unsigned long sub_1b97b(unsigned long a0,unsigned long *a1)
{
  unsigned long v1; // rax
  
  if (!dat_3d8e0)
    sub_1b451();
  v1 = dat_3d8e0 * (a0 / dat_3d8e0);
  *a1 = sub_1b4c1(v1);
  a1[1] = sub_1b675(v1);
  a1[2] = sub_1b91e;
  return 0;
}

// Function: sub_1ba02 @ 0x1ba02
unsigned long sub_1ba02(struct_15 *a0,unsigned long a1,unsigned long a2) // return-dupe
{
  if ((a1 <= a0->field_0x0) && (a0->field_0x0 <= a2 - 1)) {
    *a0->field_0x8 = a1;
    a0->field_0x8[1] = a2;
    a0->field_0x8[3] = a0->field_0x10;
    a0->field_0x18 = 0;
    return 1;
  }
  a0->field_0x10 = a2;
  return 0;
}

// Function: sub_1ba8f @ 0x1ba8f
unsigned long sub_1ba8f(unsigned long a0,unsigned long *a1) // early-return
{
  unsigned long v1; // stack - 0x38
  unsigned long *v2; // stack - 0x30
  unsigned long v3; // stack - 0x28
  int v4; // stack - 0x20
  
  v3 = 0;
  v4 = -1;
  v1 = a0;
  v2 = a1;
  sub_1b41e(&v1);
  if (!v4) {
    a1[2] = sub_1ad60;
    return 0;
  }
  return sub_1b97b(a0,a1);
}

// Function: sub_1bb23 @ 0x1bb23
unsigned long sub_1bb23(void *a0)
{
  return *(unsigned long *)((long)a0 + 0x50);
}

// Function: sub_1bb39 @ 0x1bb39
unsigned long sub_1bb39(void *a0)
{
  return *(unsigned long *)((long)a0 + 0x70);
}

// Function: sub_1bb4f @ 0x1bb4f
unsigned long sub_1bb4f(struct_17 *a0)
{
  return a0->field_0x60;
}

// Function: sub_1bb65 @ 0x1bb65
unsigned long sub_1bb65(void)
{
  return 0;
}

// Function: sub_1bb78 @ 0x1bb78
undefined16 sub_1bb78(void *a0)
{
  return *(char (*)[16])((long)a0 + 0x48);
}

// Function: sub_1bb92 @ 0x1bb92
undefined16 sub_1bb92(void *a0)
{
  return *(char (*)[16])((long)a0 + 0x68);
}

// Function: sub_1bbac @ 0x1bbac
undefined16 sub_1bbac(void *a0)
{
  return *(char (*)[16])((long)a0 + 0x58);
}

// Function: sub_1bbc6 @ 0x1bbc6
undefined16 sub_1bbc6(void)
{
  char v1 [16];
  
  v1._8_8_ = 0xffffffffffffffff;
  v1._0_8_ = 0xffffffffffffffff;
  return v1._0_16_;
}

// Function: sub_1bbec @ 0x1bbec
unsigned int sub_1bbec(unsigned int a0)
{
  return a0;
}

// Function: sub_1bc00 @ 0x1bc00
unsigned long sub_1bc00(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = memchr(a0,0,a1);
  if (v1)
    a1 = (long)v1 + (1U - (long)a0);
  return a1;
}

// Function: sub_1bc48 @ 0x1bc48
unsigned char * sub_1bc48(char *a0,int a1)
{
  unsigned long v1; // rax
  unsigned char *v10; // stack - 0x50
  long v11; // stack - 0x48
  char v12; // stack - 0x40
  unsigned int v13; // stack - 0x3c
  char v2 [8];
  unsigned char *v3; // stack - 0x68
  int v4; // stack - 0x84
  unsigned char *v5; // stack - 0x80
  unsigned char *v6; // stack - 0x78
  unsigned char *v7; // stack - 0x70
  char v8; // stack - 0x60
  char v9; // stack - 0x54
  
  v7 = (unsigned char *)strdup(a0);
  if (v7) {
    if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
      if (a1) {
        v10 = v7;
        v3 = &v7[strlen((char *)v7)];
        v8 = 0;
        memset(v2,0,8);
        while (((v9 = 0, v10 < v3 && (sub_1e2db(&v3), v12)) && (iswspace(v13)))) {
          v10 = &v10[v11];
        }
        v1 = strlen((char *)v10);
        memmove(v7,v10,v1 + 1);
      }
      if (a1 != 1) {
        v4 = 0;
        v10 = v7;
        v3 = &v7[strlen((char *)v7)];
        v8 = 0;
        memset(v2,0,8);
        for (; v9 = 0, v10 < v3; v10 = &v10[v11]) {
          sub_1e2db(&v3);
          if (((v4) || (!v12)) || (!iswspace(v13))) {
            if ((v4) || ((v12 == '\x01' && (iswspace(v13))))) {
              if ((v4 != 1) || ((v12 == '\x01' && (iswspace(v13))))) {
                if ((v4 != 1) || ((!v12 || (!iswspace(v13))))) {
                  if (((v4 != 2) || (!v12)) || (!iswspace(v13)))
                    v4 = 1;
                }
                else {
                  v4 = 2;
                  v5 = v10;
                }
              }
            }
            else {
              v4 = 1;
            }
          }
        }
        if (v4 == 2)
          *v5 = 0;
      }
    }
    else {
      if (a1) {
        v6 = v7;
        while ((*v6 && (*(unsigned short *)((unsigned long)*v6 * 2 + *(long *)__ctype_b_loc()) & 0x2000))) {
          v6 = &v6[1];
        }
        memmove(v7,v6,strlen((char *)v6) + 1);
      }
      if (a1 != 1) {
        v6 = &v7[strlen((char *)v7) - 1];
        while ((v7 <= v6 && (*(unsigned short *)((unsigned long)*v6 * 2 + *(long *)__ctype_b_loc()) & 0x2000))) {
          *v6 = 0;
          v6 = &v6[-1];
        }
      }
    }
    return v7;
  }
  sub_1cf09(); // no-return
}

// Function: sub_1bfa7 @ 0x1bfa7
void sub_1bfa7(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5) // return-dupe x10
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
  fprintf(a0,"Copyright %s %d Free Software Foundation, Inc.",gettext("(C)"),0x7e5);
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

// Function: sub_1c5a3 @ 0x1c5a3
void sub_1c5a3(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4)
{
  long v1; // stack - 0x10
  
  for (v1 = 0; a4[v1]; v1 = v1 + 1) {
  }
  sub_1bfa7(a0,a1,a2,a3,a4,v1);
}

// Function: sub_1c616 @ 0x1c616
void sub_1c616(FILE *a0,char *a1,char *a2,char *a3,struct_43 *a4)
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
  sub_1bfa7(a0,a1,a2,a3,v2,v3);
}

// Function: sub_1c71c @ 0x1c71c
void sub_1c71c(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,FILE *a8,char *a9,char *a10,char *a11,unsigned long a12,unsigned long a13)
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
  sub_1c616(a8,a9,a10,a11,&v3);
}

// Function: sub_1c800 @ 0x1c800
void sub_1c800(void)
{
  fputs_unlocked("\n",stdout);
  printf(gettext("Report bugs to: %s\n"),"bug-diffutils@gnu.org");
  printf(gettext("%s home page: <%s>\n"),"GNU diffutils","https://www.gnu.org/software/diffutils/");
  printf(gettext("General help using GNU software: <%s>\n"),"https://www.gnu.org/gethelp/");
}

// Function: sub_1c8a9 @ 0x1c8a9
void sub_1c8a9(unsigned long a0,unsigned long a1)
{
  sub_1ca0d(NULL,a0,a1);
}

// Function: sub_1c8d4 @ 0x1c8d4
void sub_1c8d4(void *a0,unsigned long a1,unsigned long a2)
{
  sub_1ca0d(a0,a1,a2);
}

// Function: sub_1c905 @ 0x1c905
void sub_1c905(unsigned long a0)
{
  sub_1c945(a0);
}

// Function: sub_1c923 @ 0x1c923
long sub_1c923(long a0)
{
  if (!a0)
    sub_1cf09(); // no-return
  return a0;
}

// Function: sub_1c945 @ 0x1c945
void sub_1c945(unsigned long a0)
{
  sub_1c923(malloc(a0));
}

// Function: sub_1c96b @ 0x1c96b
void sub_1c96b(unsigned long a0)
{
  sub_1c923(sub_1e0a0(a0));
}

// Function: sub_1c991 @ 0x1c991
void * sub_1c991(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = realloc(a0,a1);
  if ((!v1) && ((!a0 || (a1))))
    sub_1cf09(); // no-return
  return v1;
}

// Function: sub_1c9dc @ 0x1c9dc
void sub_1c9dc(void *a0,unsigned long a1)
{
  sub_1c923(sub_1e0be(a0,a1));
}

// Function: sub_1ca0d @ 0x1ca0d
void * sub_1ca0d(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = reallocarray(a0,a1,a2);
  if ((!v1) && ((!a0 || ((a1 && (a2))))))
    sub_1cf09(); // no-return
  return v1;
}

// Function: sub_1ca67 @ 0x1ca67
void sub_1ca67(void *a0,unsigned long a1,unsigned long a2)
{
  sub_1c923(sub_1e11e(a0,a1,a2));
}

// Function: sub_1caa0 @ 0x1caa0
void sub_1caa0(void *a0,unsigned long *a1)
{
  sub_1cace(a0,a1,1);
}

// Function: sub_1cace @ 0x1cace
unsigned long sub_1cace(void *a0,unsigned long *a1,unsigned long a2)
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
      sub_1cf09(); // no-return
  }
  else if (!v6) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v2 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0);
    v6 = v2 + (unsigned long)(v2 == 0);
  }
  v4 = sub_1ca0d(a0,v6,a2);
  *a1 = v6;
  return v4;
}

// Function: sub_1cbad @ 0x1cbad
unsigned long sub_1cbad(void *a0,long *a1,long a2,long a3,long a4) // ternary
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
    sub_1cf09(); // no-return
  v3 = sub_1c991(a0,v5);
  *a1 = v4;
  return v3;
}

// Function: sub_1cd69 @ 0x1cd69
void sub_1cd69(unsigned long a0)
{
  sub_1cdaf(a0,1);
}

// Function: sub_1cd8c @ 0x1cd8c
void sub_1cd8c(unsigned long a0)
{
  sub_1cde0(a0,1);
}

// Function: sub_1cdaf @ 0x1cdaf
void sub_1cdaf(unsigned long a0,unsigned long a1)
{
  sub_1c923(calloc(a0,a1));
}

// Function: sub_1cde0 @ 0x1cde0
void sub_1cde0(unsigned long a0,unsigned long a1)
{
  sub_1c923(sub_1e0f5(a0,a1));
}

// Function: sub_1ce11 @ 0x1ce11
void sub_1ce11(void *a0,unsigned long a1)
{
  memcpy((void *)sub_1c945(a1),a0,a1);
}

// Function: sub_1ce49 @ 0x1ce49
void sub_1ce49(void *a0,unsigned long a1)
{
  memcpy((void *)sub_1c96b(a1),a0,a1);
}

// Function: sub_1ce89 @ 0x1ce89
void sub_1ce89(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_1c96b(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1);
}

// Function: sub_1ced8 @ 0x1ced8
void sub_1ced8(char *a0)
{
  sub_1ce11(a0,strlen(a0) + 1);
}

// Function: sub_1cf09 @ 0x1cf09
void sub_1cf09(void)
{
  error(dat_2d0c8,0,"%s",gettext("memory exhausted")); // no-return
}

// Function: sub_1cf49 @ 0x1cf49
long sub_1cf49(unsigned long a0)
{
  long v1; // rax
  
  v1 = sub_16822(a0);
  if (!v1)
    sub_1cf09(); // no-return
  return v1;
}

// Function: sub_1cf7b @ 0x1cf7b
long sub_1cf7b(unsigned long a0)
{
  long v1; // rax
  
  v1 = sub_1db28(a0);
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_1cf09(); // no-return
  }
  return v1;
}

// Function: sub_1cfb9 @ 0x1cfb9
void sub_1cfb9(void)
{
  int v1; // eax
  
  v1 = sub_2173e();
  if (v1)
    error(dat_2d0c8,v1,gettext("standard file descriptors")); // no-return
}

// Function: sub_1cfff @ 0x1cfff
int sub_1cfff(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4)
{
  int v1; // eax
  
  v1 = sub_2180a(a0,a1,a2,a3,a4);
  if (v1 <= -1) {
    if (*__errno_location() == 0xc)
      sub_1cf09(); // no-return
  }
  return v1;
}

// Function: sub_1d05d @ 0x1d05d
long sub_1d05d(char *a0,unsigned long a1)
{
  long v1; // rax
  
  v1 = sub_21bd3(a0,a1);
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_1cf09(); // no-return
  }
  return v1;
}

// Function: sub_1d0a6 @ 0x1d0a6
long sub_1d0a6(char *a0,char *a1,char *a2)
{
  long v1; // rax
  
  v1 = sub_21ef6(a0,a1,a2);
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_1cf09(); // no-return
  }
  return v1;
}

// Function: sub_1d0f7 @ 0x1d0f7
unsigned long sub_1d0f7(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,char *a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
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
  return sub_22193(a8,&v3);
}

// Function: sub_1d1d3 @ 0x1d1d3
long sub_1d1d3(int a0,void *a1,long a2) // early-return
{
  unsigned long v1; // rax
  unsigned long v2;
  long v3; // rax
  void *v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  
  v5 = 0x7fffffffffffffff;
  v4 = a1;
  do {
    v1 = (a2 + (long)a1) - (long)v4;
    v2 = v5;
    if (v1 <= v5)
      v2 = v1;
    v3 = read(a0,v4,v2);
    if (1 <= v3) // branch-flip
      v4 = (void *)((long)v4 + v3);
    else {
      if (!v3) break;
      if ((*__errno_location() != 0x16) || (v2 < 0x80000000))
        return -1;
      v5 = 0x7fffffff;
    }
  } while (v4 < (void *)(a2 + (long)a1));
  return (long)v4 - (long)a1;
}

// Function: sub_1d2a2 @ 0x1d2a2
unsigned long sub_1d2a2(unsigned long a0,unsigned long a1,unsigned long a2) // early-return
{
  unsigned long v1; // rax
  unsigned long v2; // stack - 0x30
  unsigned long v3; // stack - 0x28
  
  if (a0) { // branch-flip
    if (a1) { // branch-flip
      v2 = a0;
      v3 = a1;
      while (v1 = v2 % v3, v1) {
        v2 = v3;
        v3 = v1;
      }
      v1 = (a0 / v3) * a1;
      if ((a2 >= v1) && (a0 / v3 == v1 / a1))
        a0 = v1;
    }
    else {
    }
  }
  else {
    if (!a1)
      return 0x2000;
    a0 = a1;
  }
  return a0;
}

// Function: sub_1d369 @ 0x1d369
unsigned long sub_1d369(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3) // early-return
{
  long v1;
  unsigned int *v2; // stack - 0x30
  unsigned long v3; // stack - 0x18
  
  v2 = a0;
  if (!a0)
    v2 = &v1;
  v3 = mbrtowc(v2,a1,a2,a3);
  if (((0xfffffffffffffffe <= v3) && (a2)) && (sub_14cdf(0) != '\x01')) {
    *v2 = (unsigned int)(unsigned char)*a1;
    return 1;
  }
  return v3;
}

// Function: sub_1d413 @ 0x1d413
void sub_1d413(void *a0,void *a1,unsigned long a2)
{
  memcpy(a0,a1,a2);
  *(char *)(a2 + (long)a0) = 0;
}

// Function: sub_1d453 @ 0x1d453
unsigned long * sub_1d453(char *a0) // ternary
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
      sub_1d413((void *)((long)v3 + 9),a0,v1);
  }
  return v3;
}

// Function: sub_1d508 @ 0x1d508
unsigned long sub_1d508(long *a0,struct_44 *a1) // return-dupe
{
  struct_44 *v1;
  long v2;
  long v3; // rax
  long *v4; // stack - 0x30
  char *v5; // stack - 0x20
  
  v5 = "";
  v1 = (struct_44 *)a1->field_0x30;
  if ((!v1) || ((a1 <= v1 && (v1 < &a1[1]))))
    return 1;
  if (v1->field_0x0[0]) {
    v5 = (char *)((long)a0 + 9);
    v4 = a0;
    while (strcmp(v5,(char *)v1)) {
      if ((!*v5) && ((v5 != (char *)((long)v4 + 9) || (!(char)v4[1])))) {
        v3 = strlen((char *)v1) + 1;
        if ((long)v4 + (0x80U - (long)v5) <= v3) { // branch-flip
          *v4 = sub_1d453(v1);
          v2 = *v4;
          if (!v2)
            return 0;
          *(char *)(v2 + 8) = 0;
          v5 = (char *)(v2 + 9);
        }
        else {
          sub_1d413(v5,v1,v3);
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

// Function: sub_1d6ac @ 0x1d6ac
void sub_1d6ac(unsigned long *a0) // return-dupe
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

// Function: sub_1d6f2 @ 0x1d6f2
void sub_1d6f2(void)
{
  getenv("TZ");
}

// Function: sub_1d70b @ 0x1d70b
void sub_1d70b(char *a0) // return-dupe
{
  if (a0) {
    setenv("TZ",a0,1);
    return;
  }
  unsetenv("TZ");
}

// Function: sub_1d750 @ 0x1d750
bool sub_1d750(void *a0) // ternary
{
  char *v1; // rax
  bool v2; // zf
  
  v1 = (*(char *)((long)a0 + 8)) ? (char *)((long)a0 + 9) : NULL; // branch-flip
  v2 = sub_1d70b(v1) == 0;
  if (v2)
    tzset();
  return v2;
}

// Function: sub_1d79a @ 0x1d79a
unsigned long * sub_1d79a(void *a0) // early-return x2
{
  int v1;
  char *v2; // rax
  unsigned long *v3; // rax
  bool v4;
  
  v2 = (char *)sub_1d6f2();
  if (v2) { // branch-flip
    if ((*(char *)((long)a0 + 8)) && (!strcmp((char *)((long)a0 + 9),v2)))
      v4 = 1;
    else {
      v4 = 0;
    }
  }
  else {
    v4 = *(char *)((long)a0 + 8) == '\0';
  }
  if (v4)
    return (unsigned long *)0x1;
  v3 = (unsigned long *)sub_1d453(v2);
  if ((v3) && (sub_1d750(a0) != '\x01')) {
    v1 = *__errno_location();
    sub_1d6ac(v3);
    *__errno_location() = v1;
    return NULL;
  }
  return v3;
}

// Function: sub_1d867 @ 0x1d867
char sub_1d867(unsigned long *a0) // early-return
{
  char v1; // al
  int v2; // stack - 0xc
  
  if (a0 == (unsigned long *)0x1)
    return '\x01';
  v2 = *__errno_location();
  v1 = sub_1d750(a0);
  if (v1 != '\x01')
    v2 = *__errno_location();
  sub_1d6ac(a0);
  *__errno_location() = v2;
  return v1;
}

// Function: sub_1d8d3 @ 0x1d8d3
struct_44 * sub_1d8d3(long *a0,void *a1,struct_44 *a2) // early-return, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_1d8d3
{
  bool v1; // al
  unsigned long *v2; // rax
  
  if (!a0)
    return (struct_44 *)gmtime_r(a1,(tm *)a2);
  v2 = (unsigned long *)sub_1d79a(a0);
  if (v2) {
    v1 = ((localtime_r(a1,(tm *)a2)) && (sub_1d508(a0,a2)));
    if ((sub_1d867(v2)) && (v1))
      return a2;
  }
  return NULL;
}

// Function: sub_1d983 @ 0x1d983
long sub_1d983(long *a0,tm *a1) // early-return, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_1d983
{
  bool v1; // al
  unsigned long *v2; // rax
  long v3; // rax
  tm v4; // stack - 0x58
  
  if (!a0)
    return timegm(a1);
  v2 = (unsigned long *)sub_1d79a(a0);
  if (v2) {
    v4._0_4_ = *(unsigned int *)a1;
    v4._4_4_ = *(unsigned int *)&a1->field_0x4;
    v4._8_4_ = *(unsigned int *)&a1->field_0x8;
    v4._12_4_ = *(unsigned int *)&a1->field_0xc;
    v4._16_4_ = *(unsigned int *)&a1->field_0x10;
    v4._20_4_ = *(unsigned int *)&a1->field_0x14;
    v4._28_4_ = -1;
    v4._32_4_ = *(unsigned int *)&a1->field_0x20;
    v3 = mktime(&v4);
    v1 = ((0 <= (int)v4._28_4_) && (sub_1d508(a0,&v4)));
    if ((sub_1d867(v2)) && (v1)) {
      *(unsigned long *)a1 = CONCAT44(v4._4_4_,v4._0_4_);
      *(unsigned long *)&a1->field_0x8 = CONCAT44(v4._12_4_,v4._8_4_);
      *(unsigned long *)&a1->field_0x10 = CONCAT44(v4._20_4_,v4._16_4_);
      *(unsigned long *)&a1->field_0x18 = CONCAT44(v4._28_4_,v4._24_4_);
      *(unsigned long *)&a1->field_0x20 = CONCAT44(v4._36_4_,v4._32_4_);
      *(unsigned long *)&a1->field_0x28 = v4._40_8_;
      *(unsigned long *)&a1->field_0x30 = v4._48_8_;
      return v3;
    }
  }
  return -1;
}

// Function: sub_1dae9 @ 0x1dae9
void sub_1dae9(int a0,char *a1,char *a2,unsigned long a3)
{
  if (a0 != -100)
    abort(); // no-return
  readlink(a1,a2,a3);
}

// Function: sub_1db28 @ 0x1db28
void sub_1db28(unsigned long a0)
{
  sub_1dfe9(0xffffff9c,a0,NULL,0,NULL,sub_1dae9);
}

// Function: sub_1db62 @ 0x1db62
unsigned long sub_1db62(unsigned long a0,unsigned char a1)
{
  return a0 << (a1 & 0x3f) | a0 >> 0x40 - (a1 & 0x3f);
}

// Function: sub_1db82 @ 0x1db82
unsigned long sub_1db82(unsigned long a0,unsigned char a1)
{
  return a0 >> (a1 & 0x3f) | a0 << 0x40 - (a1 & 0x3f);
}

// Function: sub_1dba2 @ 0x1dba2
unsigned int sub_1dba2(unsigned int a0,unsigned char a1)
{
  return a0 << (a1 & 0x1f) | a0 >> 0x20 - (a1 & 0x1f);
}

// Function: sub_1dbbe @ 0x1dbbe
unsigned int sub_1dbbe(unsigned int a0,unsigned char a1)
{
  return a0 >> (a1 & 0x1f) | a0 << 0x20 - (a1 & 0x1f);
}

// Function: sub_1dbda @ 0x1dbda
unsigned long sub_1dbda(unsigned long a0,unsigned char a1)
{
  return a0 >> (0x40 - a1 & 0x3f) | a0 << (a1 & 0x3f);
}

// Function: sub_1dc13 @ 0x1dc13
unsigned long sub_1dc13(unsigned long a0,unsigned char a1)
{
  return a0 << (0x40 - a1 & 0x3f) | a0 >> (a1 & 0x3f);
}

// Function: sub_1dc4c @ 0x1dc4c
unsigned int sub_1dc4c(unsigned short a0,unsigned char a1)
{
  return (unsigned int)(a0 >> (0x10 - a1 & 0x1f)) | (unsigned int)a0 << (a1 & 0x1f);
}

// Function: sub_1dc82 @ 0x1dc82
unsigned int sub_1dc82(unsigned short a0,unsigned char a1)
{
  return (unsigned int)a0 << (0x10 - a1 & 0x1f) | (unsigned int)(a0 >> (a1 & 0x1f));
}

// Function: sub_1dcb8 @ 0x1dcb8
unsigned int sub_1dcb8(unsigned char a0,unsigned char a1)
{
  return (unsigned int)(a0 >> (8 - a1 & 0x1f)) | (unsigned int)a0 << (a1 & 0x1f);
}

// Function: sub_1dced @ 0x1dced
unsigned int sub_1dced(unsigned char a0,unsigned char a1)
{
  return (unsigned int)a0 << (8 - a1 & 0x1f) | (unsigned int)(a0 >> (a1 & 0x1f));
}

// Function: sub_1dd22 @ 0x1dd22
int sub_1dd22(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned char v2; // al
  
  if (a0 == a1)
    return 0;
  do {
    v1 = sub_13462((unsigned char)*a0);
    v2 = sub_13462((unsigned char)*a1);
    if (!v1) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while (v1 == v2);
  return (unsigned int)v1 - (unsigned int)v2;
}

// Function: sub_1dd9c @ 0x1dd9c
void * sub_1dd9c(unsigned int a0,unsigned long a1,void *a2,unsigned long a3,unsigned long *a4,void *a5,void *a6) // early-return x3, return-dupe
{
  long v1;
  unsigned long v10; // stack - 0x38
  int v2;
  void *v3;
  long v4; // rax
  void *v5; // rax
  unsigned long *v6; // stack - 0x70
  unsigned long v7; // stack - 0x68
  void *v8; // stack - 0x60
  void *v9; // stack - 0x40
  
  v6 = a4;
  if (!a4)
    v6 = (unsigned long *)0x2c9e0;
  v7 = a3;
  v8 = a2;
  if (!a2) {
    v8 = a6;
    v7 = 0x400;
  }
  v9 = v8;
  v10 = v7;
  if (0x7fffffffffffffff <= v7)
    v10 = 0x7fffffffffffffff;
  do {
    if (!v9) {
label_1dfb9:
      if (v6[3])
        (*(void *)v6[3])(v10);
      *__errno_location() = 0xc;
      return NULL;
    }
    v4 = (*a5)(a0,a1,v9,v10);
    if (v4 <= -1) {
      if (v9 != v8) {
        v2 = *__errno_location();
        (*(void *)v6[2])(v9);
        *__errno_location() = v2;
        return NULL;
      }
      return NULL;
    }
    if (v4 < (long)v10) {
      v1 = v4 + 1;
      *(char *)((long)v9 + v4) = 0;
      if (v9 != a6) { // branch-flip
        if ((((v1 < (long)v10) && (v9 != v8)) && (v6[1])) && (v5 = (void *)(*(void *)v6[1])(v9,v1), v5))
          v9 = v5;
      }
      else {
        v5 = (void *)(*(void *)*v6)(v1);
        v10 = v1;
        if (!v5) goto label_1dfb9;
        v9 = memcpy(v5,v9,v1);
      }
      return v9;
    }
    if (v9 != v8)
      (*(void *)v6[2])(v9);
    if (0x3fffffffffffffff <= (long)v10) {
      *__errno_location() = 0x24;
      return NULL;
    }
    v10 = v10 * 2 + 1;
    v3 = (void *)*v6;
    v9 = (void *)(*v3)(v10);
  } while( true );
}

// Function: sub_1dfe9 @ 0x1dfe9
void sub_1dfe9(unsigned int a0,unsigned long a1,void *a2,unsigned long a3,unsigned long *a4,void *a5)
{
  char v1 [1032];
  
  sub_1dd9c(a0,a1,a2,a3,a4,a5,v1);
}

// Function: sub_1e086 @ 0x1e086
unsigned long sub_1e086(void)
{
  *__errno_location() = 0xc;
  return 0;
}

// Function: sub_1e0a0 @ 0x1e0a0
void sub_1e0a0(unsigned long a0)
{
  malloc(a0);
}

// Function: sub_1e0be @ 0x1e0be
void sub_1e0be(void *a0,unsigned long a1)
{
  realloc(a0,a1 == 0 | a1);
}

// Function: sub_1e0f5 @ 0x1e0f5
void sub_1e0f5(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1);
}

// Function: sub_1e11e @ 0x1e11e
void sub_1e11e(void *a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x20
  unsigned long v2; // stack - 0x18
  
  if ((!a1) || (v1 = a2, v2 = a1, !a2)) {
    v1 = 1;
    v2 = 1;
  }
  reallocarray(a0,v2,v1);
}

// Function: sub_1e16d @ 0x1e16d
char * sub_1e16d(void)
{
  char *v1; // stack - 0x10
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    v1 = "";
  if (!*v1)
    v1 = "ASCII";
  return v1;
}

// Function: sub_1e1b5 @ 0x1e1b5
unsigned int sub_1e1b5(unsigned int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (v1 <= -1) {
    v1 = iswcntrl(a0);
    v1 = (unsigned int)(v1 == 0);
  }
  return v1;
}

// Function: sub_1e1f0 @ 0x1e1f0
void sub_1e1f0(struct_39 *a0,struct_40 *a1) // return-dupe
{
  if ((struct_40 *)a1->field_0x0 != &a1[1]) // branch-flip
    a0->field_0x0 = a1->field_0x0;
  else {
    memcpy(&a0[1],&a1[1],a1->field_0x8);
    a0->field_0x0 = (long *)&a0[1];
  }
  a0->field_0x8 = a1->field_0x8;
  a0->field_0x10 = a1->field_0x10;
  if (!a0->field_0x10)
    return;
  a0->field_0x14 = a1->field_0x14;
}

// Function: sub_1e296 @ 0x1e296
bool sub_1e296(unsigned char a0)
{
  return (*(unsigned int *)((long)(int)(unsigned int)(a0 >> 5) * 4 + 0x26060) >> (a0 & 0x1f) & 1) != 0;
}

// Function: sub_1e2db @ 0x1e2db
void sub_1e2db(struct_42 *a0)
{
  char v1;
  long v2;
  char *v3;
  char *v4;
  
  if (a0->field_0x14)
    return;
  if (!a0->field_0x8) {
    v1 = *a0->field_0x18;
    if (sub_1e296((int)v1)) {
      a0->field_0x20 = 1;
      a0->field_0x2c = (int)*a0->field_0x18;
      a0->field_0x28 = 1;
      goto label_1e4e8;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x9[3]))
      __assert_fail("mbsinit (&iter->state)","mbiter.h",0x87,"mbiter_multi_next"); // no-return
    a0->field_0x8 = 1;
  }
  v2 = a0->field_0x0;
  v3 = a0->field_0x18;
  v4 = a0->field_0x18;
  a0->field_0x20 = sub_1d369(&a0->field_0x2c,v4,v2 - (long)v3,(mbstate_t *)&a0->field_0x9[3]);
  if (a0->field_0x20 != -1) { // branch-flip
    if (a0->field_0x20 != -2) { // branch-flip
      if (!a0->field_0x20) {
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
      a0->field_0x20 = a0->field_0x0 - (long)a0->field_0x18;
      a0->field_0x28 = 0;
    }
  }
  else {
    a0->field_0x20 = 1;
    a0->field_0x28 = 0;
  }
label_1e4e8:
  a0->field_0x14 = 1;
  return;
}

// Function: sub_1e4f5 @ 0x1e4f5
void sub_1e4f5(struct_12 *a0,long a1)
{
  a0->field_0x18 = a0->field_0x18 + a1;
  a0->field_0x0 = a0->field_0x0 + a1;
}

// Function: sub_1e534 @ 0x1e534
void sub_1e534(struct_45 *a0,struct_45 *a1)
{
  a0->field_0x0 = a1->field_0x0;
  a0->field_0x8 = a1->field_0x8;
  if (a0->field_0x8) // branch-flip
    memcpy(&a0->field_0x9[3],&a1->field_0x9[3],8);
  else {
    memset(&a0->field_0x9[3],0,8);
  }
  a0->field_0x14 = a1->field_0x14;
  sub_1e1f0((struct_39 *)&a0[1],(struct_40 *)&a1[1]);
}

// Function: sub_1e5da @ 0x1e5da
unsigned long sub_1e5da(char *a0,long a1,unsigned long a2,unsigned long *a3) // return-dupe
{
  long v1;
  unsigned long v10; // stack - 0x48
  long v11; // stack - 0x40
  unsigned long v12; // stack - 0x38
  char *v13; // stack - 0x30
  char *v14; // stack - 0x28
  unsigned long v15; // stack - 0x20
  unsigned long v16; // stack - 0x18
  unsigned long v2; // rax
  unsigned long *v3; // stack - 0x78
  unsigned long *v4; // rsp
  unsigned long *v5;
  unsigned long v6; // stack - 0x70
  long v7; // stack - 0x68
  char *v8; // stack - 0x60
  char v9; // stack - 0x49
  
  v4 = &v3;
  v5 = &v3;
  v8 = a0;
  v7 = a1;
  v6 = a2;
  v3 = a3;
  v15 = a2;
  if (0 <= (long)(a2 << 3) && !(a2 >> 0x3d)) { // branch-flip
    if (0xfa1 <= a2 << 3) { // branch-flip
      v2 = sub_16822(a2 << 3);
      v5 = &v3;
    }
    else {
      v2 = (((a2 + 4) * 8 + 0x16) / 0x10) * 0x10;
      for (; v4 != (unsigned long *)((long)&v3 - (v2 & 0xfffffffffffff000)); v4 = (char *)((long)v4 + -0x1000)) {
        *(unsigned long *)((long)v4 + -8) = *(unsigned long *)((long)v4 + -8);
      }
      v1 = -(unsigned long)((unsigned int)v2 & 0xfff);
      if (v2 & 0xfff)
        *(unsigned long *)((long)v4 + ((unsigned long)((unsigned int)v2 & 0xfff) - 8) + v1) = *(unsigned long *)((long)v4 + ((unsigned long)((unsigned int)v2 & 0xfff) - 8) + v1);
      v2 = ((unsigned long)((long)v4 + v1 + 0xf) & 0xfffffffffffffff0) + 0x1f & 0xffffffffffffffe0;
      v5 = (unsigned long *)((long)v4 + v1);
    }
  }
  else {
    v2 = 0;
  }
  if (!v2)
    return 0;
  *(unsigned long *)(v2 + 8) = 1;
  v11 = 0;
  for (v10 = 2; v10 < v15; v10 = v10 + 1) {
    v9 = *(char *)(v7 + (v10 - 1));
    for (; v9 != *(char *)(v11 + v7); v11 = v11 - *(long *)(v2 + v11 * 8)) {
      if (!v11) {
        *(unsigned long *)(v10 * 8 + v2) = v10;
        goto label_1e7cf;
      }
    }
    v11 += 1;
    *(unsigned long *)(v10 * 8 + v2) = v10 - v11;
label_1e7cf:
  }
  *v3 = 0;
  v12 = 0;
  v13 = v8;
  v14 = v8;
  do {
    while( true ) {
      if (!*v14) goto label_1e89a;
      if (*(char *)(v12 + v7) == *v14) break;
      if (v12) { // branch-flip
        v13 = &v13[*(long *)(v2 + v12 * 8)];
        v12 -= *(long *)(v2 + v12 * 8);
      }
      else {
        v13 = &v13[1];
        v14 = &v14[1];
      }
    }
    v12 += 1;
    v14 = &v14[1];
  } while (v12 != v15);
  *v3 = v13;
label_1e89a:
  *(unsigned long *)((long)v5 + -8) = 0x1e8a6;
  v16 = v2;
  sub_168f3(v2);
  return 1;
}

// Function: sub_1e8c1 @ 0x1e8c1
unsigned long sub_1e8c1(void *a0,void *a1,long *a2)
{
  void *v1;
  char v10 [4]; // stack - 0x98
  char *v11; // rsp
  char *v12;
  bool v13;
  long *v14; // stack - 0x110
  void *v15; // stack - 0x108
  void *v16; // stack - 0x100
  long v17; // stack - 0xf0
  unsigned long v18; // stack - 0xe8
  long v19; // stack - 0xe0
  void *v2;
  unsigned long v20; // stack - 0xd8
  long v21; // stack - 0xd0
  unsigned long v22; // stack - 0xc8
  unsigned long v23; // stack - 0xc0
  unsigned long v24; // stack - 0xb8
  long v25; // stack - 0xb0
  long v26; // stack - 0xa8
  unsigned long *v27; // stack - 0xa0
  char v28; // stack - 0x8c
  void *v29; // stack - 0x88
  unsigned long v3;
  long v30; // stack - 0x80
  char v31; // stack - 0x78
  int v32; // stack - 0x74
  char v33; // stack - 0x4c
  long v34; // stack - 0x40
  char v35; // stack - 0x38
  int v36; // stack - 0x34
  long v4;
  char v5 [8];
  char v6 [8];
  void *v7; // stack - 0x48
  char v8 [4]; // stack - 0x58
  char v9 [8];
  
  v11 = v5;
  v12 = v5;
  v14 = a2;
  v15 = a1;
  v16 = a0;
  v22 = sub_22704(a1);
  if (0 <= SUB168(ZEXT816(0x38) * ZEXT816(v22),0) && !SUB168(ZEXT816(0x38) * ZEXT816(v22),8)) { // branch-flip
    if (0xfa1 <= v22 * 0x38) { // branch-flip
      v3 = sub_16822(v22 * 0x38);
      v12 = v5;
    }
    else {
      v3 = ((v22 * 0x38 + 0x36) / 0x10) * 0x10;
      for (; v11 != &v5[-(v3 & 0xfffffffffffff000)]; v11 = &v11[-0x1000]) {
        *(unsigned long *)&v11[-8] = *(unsigned long *)&v11[-8];
      }
      v4 = -(unsigned long)((unsigned int)v3 & 0xfff);
      if (v3 & 0xfff)
        *(unsigned long *)&v11[((unsigned long)((unsigned int)v3 & 0xfff) - 8) + v4] = *(unsigned long *)&v11[((unsigned long)((unsigned int)v3 & 0xfff) - 8) + v4];
      v3 = ((unsigned long)&v11[v4 + 0xf] & 0xfffffffffffffff0) + 0x1f & 0xffffffffffffffe0;
      v12 = &v11[v4];
    }
  }
  else {
    v3 = 0;
  }
  if (!v3)
    return 0;
  v4 = v3 + v22 * 0x30;
  v17 = 0;
  v7 = v15;
  v8[0] = 0;
  *(unsigned long *)&v12[-8] = 0x1eab9;
  v23 = v3;
  v24 = v3;
  v25 = v4;
  v26 = v4;
  memset(v6,0,8);
  while( true ) {
    v33 = 0;
    *(unsigned long *)&v12[-8] = 0x1eb1a;
    sub_16d35(v8);
    if ((v35 != '\x01') || (v36))
      v13 = 1;
    else {
      v13 = 0;
    }
    if (!v13) break;
    *(unsigned long *)&v12[-8] = 0x1eaf3;
    sub_1e1f0((struct_39 *)(v24 + v17 * 0x30),&v7);
    v7 = (void *)(v34 + (long)v7);
    v17 += 1;
  }
  *(unsigned long *)(v26 + 8) = 1;
  v19 = 0;
  v18 = 2;
label_1ed25:
  if (v18 < v22) {
    v27 = (unsigned long *)(v24 + v18 * 0x30 + -0x30);
    do {
      if ((*(char *)&v27[2]) && (*(char *)(v24 + v19 * 0x30 + 0x10)))
        v13 = *(int *)((long)v27 + 0x14) == *(int *)(v24 + v19 * 0x30 + 0x14);
      else {
        if (v27[1] == *(long *)(v24 + v19 * 0x30 + 8)) {
          v3 = v27[1];
          v2 = *(void **)(v24 + v19 * 0x30);
          v1 = (void *)*v27;
          *(unsigned long *)&v12[-8] = 0x1ec7b;
          if (!memcmp(v1,v2,v3)) {
            v13 = 1;
            goto label_1ec8e;
          }
        }
        v13 = 0;
      }
label_1ec8e:
      if (v13) {
        v19 += 1;
        *(unsigned long *)(v18 * 8 + v26) = v18 - v19;
        goto label_1ed1d;
      }
      if (!v19) goto label_1ecd0;
      v19 -= *(long *)(v26 + v19 * 8);
    } while( true );
  }
  *v14 = 0;
  v20 = 0;
  v29 = v16;
  v10[0] = 0;
  *(unsigned long *)&v12[-8] = 0x1ed81;
  memset(v9,0,8);
  v28 = 0;
  v7 = v16;
  v8[0] = 0;
  *(unsigned long *)&v12[-8] = 0x1edb1;
  memset(v6,0,8);
  v33 = 0;
  do {
    *(unsigned long *)&v12[-8] = 0x1f007;
    sub_16d35(v8);
    if ((v35 != '\x01') || (v36))
      v13 = 1;
    else {
      v13 = 0;
    }
    if (!v13) goto label_1f02d;
    if ((*(char *)(v24 + v20 * 0x30 + 0x10)) && (v35))
      v13 = *(int *)(v24 + v20 * 0x30 + 0x14) == v36;
    else {
      if (*(long *)(v24 + v20 * 0x30 + 8) == v34) {
        v3 = *(unsigned long *)(v24 + v20 * 0x30 + 8);
        v2 = *(void **)(v24 + v20 * 0x30);
        *(unsigned long *)&v12[-8] = 0x1eea2;
        if (!memcmp(v2,v7,v3)) {
          v13 = 1;
          goto label_1eeb5;
        }
      }
      v13 = 0;
    }
label_1eeb5:
    if (v13) {
      v20 += 1;
      v7 = (void *)(v34 + (long)v7);
      v33 = 0;
      if (v20 == v22) {
        *v14 = (long)v29;
label_1f02d:
        *(unsigned long *)&v12[-8] = 0x1f03c;
        sub_168f3(v23);
        return 1;
      }
    }
    else if (v20) { // branch-flip
      v21 = *(long *)(v26 + v20 * 8);
      v20 -= v21;
      for (; v21; v21 = v21 + -1) {
        *(unsigned long *)&v12[-8] = 0x1ef4b;
        sub_16d35(v10);
        if ((v31 != '\x01') || (v32))
          v13 = 1;
        else {
          v13 = 0;
        }
        if (!v13) {
          *(unsigned long *)&v12[-8] = 0x1ef72; // no-return
          abort();
        }
        v29 = (void *)(v30 + (long)v29);
        v28 = 0;
      }
    }
    else {
      *(unsigned long *)&v12[-8] = 0x1efab;
      sub_16d35(v10);
      if ((v31 != '\x01') || (v32))
        v13 = 1;
      else {
        v13 = 0;
      }
      if (!v13) {
        *(unsigned long *)&v12[-8] = 0x1efd2; // no-return
        abort();
      }
      v29 = (void *)(v30 + (long)v29);
      v28 = 0;
      v7 = (void *)(v34 + (long)v7);
      v33 = 0;
    }
  } while( true );
label_1ecd0:
  *(unsigned long *)(v18 * 8 + v26) = v18;
label_1ed1d:
  v18 += 1;
  goto label_1ed25;
}

// Function: sub_1f057 @ 0x1f057
char * sub_1f057(char *a0,char *a1) // early-return, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_1f057
{
  char v1;
  char v10 [4]; // stack - 0xd8
  char v11 [4]; // stack - 0x118
  char *v12; // stack - 0x1b0
  char v13 [12];
  char v14 [8];
  char v15 [4]; // stack - 0x58
  bool v16;
  char *v17; // stack - 0x1c0
  unsigned long v18; // stack - 0x1a8
  unsigned long v19; // stack - 0x1a0
  bool v2;
  unsigned long v20; // stack - 0x198
  long v21; // stack - 0x190
  unsigned long v22; // stack - 0x188
  unsigned long v23; // stack - 0x180
  unsigned long v24; // stack - 0x178
  char *v25; // stack - 0x170
  char *v26; // stack - 0x168
  char *v27; // stack - 0x160
  char v28; // stack - 0x14c
  char *v29; // stack - 0x148
  char *v3;
  unsigned long v30; // stack - 0x140
  char v31; // stack - 0x138
  int v32; // stack - 0x134
  char v33; // stack - 0x10c
  char *v34; // stack - 0x108
  long v35; // stack - 0x100
  char v36; // stack - 0xf8
  int v37; // stack - 0xf4
  char v38; // stack - 0xcc
  char *v39; // stack - 0xc8
  char *v4;
  unsigned long v40; // stack - 0xc0
  char v41; // stack - 0xb8
  int v42; // stack - 0xb4
  char v43; // stack - 0x8c
  void *v44; // stack - 0x88
  unsigned long v45; // stack - 0x80
  char v46; // stack - 0x78
  int v47; // stack - 0x74
  char v48; // stack - 0x4c
  char *v49; // stack - 0x48
  unsigned long v5; // rax
  unsigned long v50; // stack - 0x40
  char v51; // stack - 0x38
  int v52; // stack - 0x34
  char v6 [8];
  char v7 [4]; // stack - 0x158
  char v8 [8];
  char v9 [8];
  
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
    v7[0] = 0;
    v29 = a1;
    memset(v6,0,8);
    v28 = 0;
    sub_16d35(v7);
    if ((v31 != '\x01') || (v32))
      v2 = 1;
    else {
      v2 = 0;
    }
    if (v2) {
      v2 = 1;
      v18 = 0;
      v19 = 0;
      v20 = 0;
      v11[0] = 0;
      v34 = a1;
      memset(v8,0,8);
      v33 = 0;
      v10[0] = 0;
      v39 = a0;
      memset(v9,0,8);
      do {
        v38 = 0;
        sub_16d35(v10);
        if ((v41 != '\x01') || (v42))
          v16 = 1;
        else {
          v16 = 0;
        }
        if (!v16)
          return NULL;
        if (((v2) && (10 <= v18)) && (v18 * 5 <= v19)) {
          for (v21 = v19 - v20; v21; v21 = v21 + -1) {
            sub_16d35(v11);
            if ((v36 != '\x01') || (v37))
              v16 = 1;
            else {
              v16 = 0;
            }
            if (!v16) break;
            v34 = &v34[v35];
            v33 = 0;
          }
          v20 = v19;
          sub_16d35(v11);
          if ((v36 != '\x01') || (v37))
            v16 = 1;
          else {
            v16 = 0;
          }
          if (!v16) {
            if (sub_1e8c1(a0,a1,&v12))
              return v12;
            v2 = 0;
          }
        }
        v18 += 1;
        v19 += 1;
        if ((v41) && (v31))
          v16 = v42 == v32;
        else {
          v16 = !((v40 != v30) || (memcmp(v39,v29,v40)));
        }
        if (v16) {
          memcpy(v13,v10,0x40);
          v44 = (void *)(v45 + (long)v44);
          v43 = 0;
          v15[0] = 0;
          v49 = a1;
          memset(v14,0,8);
          v48 = 0;
          sub_16d35(v15);
          if ((v51 != '\x01') || (v52))
            v16 = 1;
          else {
            v16 = 0;
          }
          if (!v16)
            abort(); // no-return
          v49 = &v49[v50];
          while( true ) {
            v48 = 0;
            sub_16d35(v15);
            if ((v51 != '\x01') || (v52))
              v16 = 1;
            else {
              v16 = 0;
            }
            if (!v16)
              return v39;
            sub_16d35(v13);
            if ((v46 != '\x01') || (v47))
              v16 = 1;
            else {
              v16 = 0;
            }
            if (!v16)
              return NULL;
            v19 += 1;
            if ((v46) && (v51))
              v16 = v47 != v52;
            else {
              v16 = ((v45 != v50) || (memcmp(v44,v49,v45)));
            }
            if (v16) break;
            v44 = (void *)(v45 + (long)v44);
            v43 = 0;
            v49 = &v49[v50];
          }
        }
        v39 = &v39[v40];
      } while( true );
    }
  }
  else if (*a1) {
    v2 = 1;
    v22 = 0;
    v23 = 0;
    v24 = 0;
    v1 = *a1;
    v17 = a0;
    v25 = a1;
    do {
      if (!*v17)
        return NULL;
      if (((v2) && (10 <= v22)) && (v22 * 5 <= v23)) {
        if (v25) {
          v5 = strnlen(v25,v23 - v24);
          v25 = &v25[v5];
          if (!*v25)
            v25 = NULL;
          v24 = v23;
        }
        if (!v25) {
          v5 = strlen(a1);
          if (sub_1e5da(v17,a1,v5,&v12))
            return v12;
          v2 = 0;
        }
      }
      v22 += 1;
      v23 += 1;
      v3 = v17;
      v4 = a1;
      if (v1 == *v17) {
        do {
          v27 = &v4[1];
          v26 = &v3[1];
          if (!*v27)
            return v17;
          if (!*v26)
            return NULL;
          v23 += 1;
          v3 = v26;
          v4 = v27;
        } while (*v26 == *v27);
      }
      v17 = &v17[1];
    } while( true );
  }
  return a0;
}

// Function: sub_1f7c5 @ 0x1f7c5
unsigned long sub_1f7c5(void *a0)
{
  int v1;
  unsigned long v2; // rax
  
  v1 = *__errno_location();
  if (!a0)
    a0 = (void *)0x3d900;
  v2 = sub_1ce11(a0,0x38);
  *__errno_location() = v1;
  return v2;
}

// Function: sub_1f814 @ 0x1f814
unsigned int sub_1f814(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x3d900;
  return *a0;
}

// Function: sub_1f838 @ 0x1f838
void sub_1f838(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x3d900;
  *a0 = a1;
}

// Function: sub_1f863 @ 0x1f863
unsigned int sub_1f863(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int v1; // eax
  unsigned int *v2; // rax
  
  if (!a0)
    a0 = 0x3d900;
  v2 = (unsigned int *)((unsigned long)(a1 >> 5) * 4 + a0 + 8);
  v1 = *v2 >> (a1 & 0x1f) & 1;
  *v2 = *v2 ^ (a2 & 1 ^ v1) << (a1 & 0x1f);
  return v1;
}

// Function: sub_1f8f3 @ 0x1f8f3
unsigned int sub_1f8f3(void *a0,unsigned int a1)
{
  unsigned int v1;
  void *v2; // stack - 0x20
  
  v2 = a0;
  if (!a0)
    v2 = (void *)0x3d900;
  v1 = *(unsigned int *)((long)v2 + 4);
  *(unsigned int *)((long)v2 + 4) = a1;
  return v1;
}

// Function: sub_1f92d @ 0x1f92d
void sub_1f92d(unsigned int *a0,long a1,long a2)
{
  unsigned int *v1; // stack - 0x10
  
  v1 = a0;
  if (!a0)
    v1 = (unsigned int *)0x3d900;
  *v1 = 10;
  if ((a1) && (a2)) {
    *(long *)&v1[10] = a1;
    *(long *)&v1[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_1f98f @ 0x1f98f
unsigned long * sub_1f98f(unsigned long *a0,unsigned int a1)
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

// Function: sub_1fa54 @ 0x1fa54
char * sub_1fa54(char *a0,int a1) // early-return x6
{
  char *v1; // rax
  char *v2; // rax
  
  v1 = gettext(a0);
  if (v1 != a0)
    return v1;
  v2 = (char *)sub_1e16d();
  if (!sub_1dd22(v2,"UTF-8")) {
    if (*a0 == '`')
      return "‘";
    return "’";
  }
  if (sub_1dd22(v2,"GB18030")) {
    if (a1 == 9)
      return "\"";
    return "\'";
  }
  if (*a0 == '`')
    return (char *)0x2619e;
  return (char *)0x261a2;
}

// Function: sub_1fb19 @ 0x1fb19
unsigned long sub_1fb19(long a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_1fbd3:
  switch(v19) {
    case 0:
      v14 = 0;
      break;
    case 1:
label_1fd27:
      v14 = 1;
      goto label_1fd2b;
    case 2:
label_1fd3a:
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
      goto label_1fd27;
    case 4:
label_1fd2b:
      if (!v14)
        v2 = 1;
      goto label_1fd3a;
    case 5:
label_1fc15:
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
      goto label_1fc15;
    case 7:
      v2 = 1;
      v14 = 0;
      break;
    case 8:
    case 9:
    case 10:
      if (v19 != 10) {
        v18 = (char *)sub_1fa54("`",v19);
        v17 = (char *)sub_1fa54("\'",v19);
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
label_209d9:
  if (v20 != 0xffffffffffffffff) // branch-flip
    v16 = v24 != v20;
  else {
    v16 = a2[v24] != '\0';
  }
  if (!v16) goto label_20a11;
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
      if (v14) goto label_20b55;
      v4 = 1;
    }
  }
  v22 = a2[v24];
  if (v22 == 0x7e) {
label_202f8:
    if (!v24) {
label_20303:
      v16 = 1;
label_20307:
      if (v19 != 2) goto label_2081e;
label_200e0:
      if (v14) goto label_20b55;
    }
    goto label_2081e;
  }
  if (0x7e < v22) {
label_203e4:
    if (v9 != 1) {
      memset(&v12,0,8);
      v29 = 0;
      v16 = 1;
      if (v20 == 0xffffffffffffffff)
        v20 = strlen(a2);
label_2045e:
      v31 = sub_1d369(&v13,&a2[v24 + v29],v20 - (v24 + v29),&v12);
      if (v31) {
        if (v31 != 0xffffffffffffffff) { // branch-flip
          if (v31 != 0xfffffffffffffffe) goto label_2050c;
          v16 = 0;
          for (; (v29 + v24 < v20 && (a2[v24 + v29])); v29 = v29 + 1) {
          }
        }
        else {
          v16 = 0;
        }
      }
      goto label_205bb;
    }
    v29 = 1;
    v16 = (*(unsigned short *)((unsigned long)v22 * 2 + *(long *)__ctype_b_loc()) & 0x4000) != 0;
label_205bb:
    if ((v29 <= 1) && ((!v2 || (v16)))) goto label_2081e;
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
        if (v14) goto label_20b55;
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
    goto label_20944;
  }
  if (v22 == 0x7d) {
label_202c6:
    if (v20 != 0xffffffffffffffff) // branch-flip
      v15 = v20 != 1;
    else {
      v15 = a2[1] != '\0';
    }
    if (v15) goto label_2081e;
    goto label_202f8;
  }
  if (0x7d < v22) goto label_203e4;
  if (v22 == 0x7c) goto label_20307;
  if (0x7c < v22) goto label_203e4;
  if (v22 == 0x7b) goto label_202c6;
  if (0x7b < v22) goto label_203e4;
  if (0x3f < v22) {
    if ((0x7a < v22) || (v22 <= 0x40)) goto label_203e4;
    v11 = 1L << (v22 + 0xbf & 0x3f);
    if (v11 & 0x3ffffff53ffffff) goto label_203db;
    if (v11 & 0xa4000000) goto label_20307;
    if (!(v11 & 0x8000000)) goto label_203e4;
    if (v19 == 2) {
      if (!v14) goto label_20944;
      goto label_20b55;
    }
    v23 = v22;
    if (((!v2) || (!v14)) || (!v28)) goto label_2029c;
    goto label_20944;
  }
  switch(v22) {
    case 0:
      if (!v2) {
        if (!(a5 & 1)) goto label_2081e;
        goto label_209d4;
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
        goto label_2081e;
      }
      goto label_20b55;
    default:
      goto label_203e4;
    case 7:
      v23 = 0x61;
      break;
    case 8:
      v23 = 0x62;
      break;
    case 9:
      v23 = 0x74;
      goto label_2029c;
    case 10:
      v23 = 0x6e;
      goto label_2029c;
    case 0xb:
      v23 = 0x76;
      break;
    case 0xc:
      v23 = 0x66;
      break;
    case 0xd:
      v23 = 0x72;
label_2029c:
      if ((v19 == 2) && (v14)) goto label_20b55;
      break;
    case 0x20:
      goto label_20303;
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
      goto label_20307;
    case 0x23:
      goto label_202f8;
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
label_203db:
      v16 = 1;
      goto label_2081e;
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
          goto label_2081e;
        }
        goto label_20b55;
      }
      goto label_2081e;
    case 0x3f:
      if (v19 == 2) goto label_200e0;
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
          goto label_2081e;
        }
        goto label_20b55;
      }
      goto label_2081e;
    
  }
  if (v2) {
    v22 = v23;
label_20889:
    if (v14) goto label_20b55;
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
label_2081e:
    if (((((v2) && (v19 != 2)) || (v14)) && ((v8 && (*(unsigned int *)(v8 + (unsigned long)(v22 >> 5) * 4) >> (v22 & 0x1f) & 1)))) || (v4)) goto label_20889;
  }
label_20944:
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
label_209d4:
  v24 += 1;
  goto label_209d9;
label_2050c:
  if ((v14) && (v19 == 2)) {
    for (v30 = 1; v30 < v31; v30 = v30 + 1) {
      if (((unsigned int)((int)a2[v24 + v29 + v30] - 0x5bU) <= 0x21) && (0x20000002bU >> ((unsigned char)((int)a2[v24 + v29 + v30] - 0x5bU) & 0x3f) & 1)) goto label_20b55;
    }
  }
  if (!iswprint(v13))
    v16 = 0;
  v29 += v31;
  if (mbsinit(&v12)) goto label_205bb;
  goto label_2045e;
label_20a11:
  if (((!v25) && (v19 == 2)) && (v14)) {
label_20b55:
    if ((v19 == 2) && (v2))
      v19 = 4;
    return sub_1fb19(a0,v21,a2,v20,v19,a5 & 0xfffffffd,0,v18,v17);
  }
  if (((v19 == 2) && (!v14)) && (v6)) {
    if (v7)
      return sub_1fb19(a0,v26,a2,v20,5,a5,v8,v18,v17);
    if ((!v21) && (v26)) {
      v21 = v26;
      v25 = 0;
      goto label_1fbd3;
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

// Function: sub_20bda @ 0x20bda
unsigned long sub_20bda(long a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  unsigned long v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x3d900;
  v1 = *__errno_location();
  v2 = sub_1fb19(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *__errno_location() = v1;
  return v2;
}

// Function: sub_20c82 @ 0x20c82
void sub_20c82(char *a0,unsigned long a1,unsigned int *a2)
{
  sub_20cb8(a0,a1,NULL,a2);
}

// Function: sub_20cb8 @ 0x20cb8
long sub_20cb8(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  unsigned int v2; // eax
  long v3; // rax
  long v4; // rax
  
  if (!a3)
    a3 = (unsigned int *)0x3d900;
  v1 = *__errno_location();
  v2 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_1fb19(0,0,a0,a1,*a3,v2,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = sub_1c905(v3 + 1U);
  sub_1fb19(v4,v3 + 1U,a0,a1,*a3,v2,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *__errno_location() = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_20de4 @ 0x20de4
void sub_20de4(void) // return-dupe
{
  void *v1;
  int v2; // stack - 0x14
  
  v1 = dat_2d150;
  for (v2 = 1; v2 < dat_2d0e0; v2 = v2 + 1) {
    free(*(void **)((long)v1 + (long)v2 * 0x10 + 8));
  }
  if (*(long *)((long)v1 + 8) != 0x3d940) {
    free(*(void **)((long)v1 + 8));
    dat_2d140 = 0x100;
    dat_2d148 = 0x3d940;
  }
  if (v1 == (void *)0x2d140) {
    dat_2d0e0 = 1;
    return;
  }
  free(v1);
  dat_2d0e0 = 1;
  dat_2d150 = (void *)0x2d140;
}

// Function: sub_20ea3 @ 0x20ea3
void * sub_20ea3(int a0,char *a1,unsigned long a2,struct_46 *a3)
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
  v6 = dat_2d150;
  if ((0 <= a0) && (a0 <= 0x7ffffffe)) {
    if (dat_2d0e0 <= a0) {
      v5 = dat_2d150 == (unsigned long *)0x2d140;
      v4 = (long)dat_2d0e0;
      if (v5)
        v3 = NULL;
      else {
        v3 = dat_2d150;
      }
      dat_2d150 = (unsigned long *)sub_1cbad(v3,&v4,(long)((a0 - dat_2d0e0) + 1),0x7fffffff,0x10);
      v6 = dat_2d150;
      if (v5) {
        *dat_2d150 = dat_2d140;
        dat_2d150[1] = dat_2d148;
      }
      memset(&dat_2d150[(long)dat_2d0e0 * 2],0,(v4 - dat_2d0e0) * 0x10);
      dat_2d0e0 = (int)v4;
    }
    v8 = v6[(long)a0 * 2];
    v7 = (void *)v6[(long)a0 * 2 + 1];
    v2 = a3->field_0x4 | 1;
    v9 = sub_1fb19(v7,v8,a1,a2,a3->field_0x0,v2,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v8 <= v9) {
      v8 = v9 + 1;
      v6[(long)a0 * 2] = v8;
      if (v7 != (void *)0x3d940)
        free(v7);
      v7 = (void *)sub_1c905(v8);
      v6[(long)a0 * 2 + 1] = v7;
      sub_1fb19(v7,v8,a1,a2,a3->field_0x0,v2,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *__errno_location() = v1;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_21152 @ 0x21152
void sub_21152(unsigned int a0,char *a1)
{
  sub_20ea3(a0,a1,0xffffffffffffffff,(struct_46 *)0x3d900);
}

// Function: sub_21186 @ 0x21186
void sub_21186(unsigned int a0,char *a1,unsigned long a2)
{
  sub_20ea3(a0,a1,a2,(struct_46 *)0x3d900);
}

// Function: sub_211b8 @ 0x211b8
void sub_211b8(char *a0)
{
  sub_21152(0,a0);
}

// Function: sub_211db @ 0x211db
void sub_211db(char *a0,unsigned long a1)
{
  sub_21186(0,a0,a1);
}

// Function: sub_21206 @ 0x21206
void sub_21206(unsigned int a0,unsigned int a1,char *a2)
{
  char v1 [56];
  
  sub_1f98f(v1,a1);
  sub_20ea3(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_2126e @ 0x2126e
void sub_2126e(unsigned int a0,unsigned int a1,char *a2,unsigned long a3)
{
  char v1 [56];
  
  sub_1f98f(v1,a1);
  sub_20ea3(a0,a2,a3,v1);
}

// Function: sub_212d4 @ 0x212d4
void sub_212d4(unsigned int a0,char *a1)
{
  sub_21206(0,a0,a1);
}

// Function: sub_212fc @ 0x212fc
void sub_212fc(unsigned int a0,char *a1,unsigned long a2)
{
  sub_2126e(0,a0,a1,a2);
}

// Function: sub_2132c @ 0x2132c
void sub_2132c(char *a0,unsigned long a1,char a2)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v1 = dat_3d900;
  v2 = dat_3d908;
  v3 = dat_3d910;
  v4 = dat_3d918;
  v5 = dat_3d920;
  v6 = dat_3d928;
  v7 = dat_3d930;
  sub_1f863(&v1,(int)a2,1);
  sub_20ea3(0,a0,a1,&v1);
}

// Function: sub_213e7 @ 0x213e7
void sub_213e7(char *a0,char a1)
{
  sub_2132c(a0,0xffffffffffffffff,(int)a1);
}

// Function: sub_21415 @ 0x21415
void sub_21415(char *a0)
{
  sub_213e7(a0,0x3a);
}

// Function: sub_21438 @ 0x21438
void sub_21438(char *a0,unsigned long a1)
{
  sub_2132c(a0,a1,0x3a);
}

// Function: sub_21466 @ 0x21466
void sub_21466(unsigned int a0,unsigned int a1,char *a2)
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
  sub_1f98f(&v1,a1);
  v2 = v1;
  v12 = v3;
  v13 = v4;
  v14 = v5;
  v15 = v6;
  v16 = v7;
  v17 = v8;
  sub_1f863(&v2,0x3a,1);
  sub_20ea3(v11,v9,0xffffffffffffffff,&v2);
}

// Function: sub_21528 @ 0x21528
void sub_21528(unsigned int a0,long a1,long a2,char *a3)
{
  sub_21562(a0,a1,a2,a3,0xffffffffffffffff);
}

// Function: sub_21562 @ 0x21562
void sub_21562(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v1 = dat_3d900;
  v2 = dat_3d908;
  v3 = dat_3d910;
  v4 = dat_3d918;
  v5 = dat_3d920;
  v6 = dat_3d928;
  v7 = dat_3d930;
  sub_1f92d(&v1,a1,a2);
  sub_20ea3(a0,a3,a4,&v1);
}

// Function: sub_21620 @ 0x21620
void sub_21620(long a0,long a1,char *a2)
{
  sub_21528(0,a0,a1,a2);
}

// Function: sub_21653 @ 0x21653
void sub_21653(long a0,long a1,char *a2,unsigned long a3)
{
  sub_21562(0,a0,a1,a2,a3);
}

// Function: sub_21691 @ 0x21691
void sub_21691(unsigned int a0,char *a1,unsigned long a2)
{
  sub_20ea3(a0,a1,a2,(struct_46 *)0x2d100);
}

// Function: sub_216c3 @ 0x216c3
void sub_216c3(char *a0,unsigned long a1)
{
  sub_21691(0,a0,a1);
}

// Function: sub_216ee @ 0x216ee
void sub_216ee(unsigned int a0,char *a1)
{
  sub_21691(a0,a1,0xffffffffffffffff);
}

// Function: sub_2171b @ 0x2171b
void sub_2171b(char *a0)
{
  sub_216ee(0,a0);
}

// Function: sub_2173e @ 0x2173e
int sub_2173e(void)
{
  int v1; // eax
  int v2; // stack - 0x18
  
  v2 = 0;
  do {
    if (3 <= v2)
      return 0;
    if ((int)sub_2225e(v2,1) <= -1) {
      if (v2) // branch-flip
        v1 = -1;
      else {
        v1 = open("/dev/full",1);
      }
      if (v1 <= -1)
        v1 = open("/dev/null",(unsigned int)(v2 == 0));
      if (v1 <= -1)
        return *__errno_location();
      if (3 <= v1) {
        close(v1);
        return 0;
      }
    }
    v2 += 1;
  } while( true );
}

// Function: sub_2180a @ 0x2180a
unsigned long sub_2180a(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4) // return-dupe x2, return-dupe
{
  char v1 [4104];
  char *v10; // stack - 0x1030
  long v11; // stack - 0x1028
  long v12; // stack - 0x1020
  char *v2; // stack - 0x1058
  char *v3; // stack - 0x1060
  long v4; // stack - 0x1068
  unsigned long v5; // stack - 0x1070
  char *v6; // stack - 0x1050
  char *v7; // stack - 0x1048
  long v8; // stack - 0x1040
  long v9; // stack - 0x1038
  
  iconv(a2,0,0,0,0);
  v7 = NULL;
  v5 = a0;
  v4 = a1;
  while (v4) {
    v3 = v1;
    v2 = (char *)0x1000;
    v8 = iconv(a2,&v5,&v4,&v3,&v2);
    if ((v8 == -1) && (*__errno_location() != 7)) {
      if (*__errno_location() != 0x16)
        return 0xffffffff;
      break;
    }
    v7 = &v3[(long)v7 - (long)v1];
  }
  v3 = v1;
  v2 = (char *)0x1000;
  v9 = iconv(a2,0,0,&v3,&v2);
  if (v9 == -1)
    return 0xffffffff;
  v7 = &v3[(long)v7 - (long)v1];
  if (!v7) {
    *a4 = 0;
    return 0;
  }
  v10 = v7;
  if ((*a3) && (v7 <= (char *)*a4))
    v6 = (char *)*a3;
  else {
    v6 = malloc((unsigned long)v7);
    if (!v6) {
      *__errno_location() = 0xc;
      return 0xffffffff;
    }
  }
  iconv(a2,0,0,0,0);
  v3 = v6;
  v2 = v10;
  v5 = a0;
  v4 = a1;
  do {
    if (!v4) goto label_21b1a;
    v11 = iconv(a2,&v5,&v4,&v3,&v2);
  } while (v11 != -1);
  if (*__errno_location() == 0x16) {
label_21b1a:
    v12 = iconv(a2,0,0,&v3,&v2);
    if (v12 != -1) {
      if (v2)
        abort(); // no-return
      *a3 = (long)v6;
      *a4 = (unsigned long)v10;
      return 0;
    }
  }
  if (v6 == (char *)*a3)
    return 0xffffffff;
  free(v6);
  return 0xffffffff;
}

// Function: sub_21bd3 @ 0x21bd3
char * sub_21bd3(char *a0,unsigned long a1) // return-dupe
{
  unsigned long v1;
  long v10; // stack - 0x60
  long v11; // stack - 0x58
  unsigned long v12; // stack - 0x50
  char *v13; // stack - 0x48
  long v14; // stack - 0x40
  unsigned long v15; // stack - 0x38
  long v16; // stack - 0x28
  unsigned long v17; // stack - 0x20
  char *v18; // stack - 0x18
  char *v2;
  long v3; // stack - 0x80
  char *v4; // stack - 0x88
  unsigned long v5; // stack - 0x90
  char *v6; // stack - 0x98
  char *v7; // stack - 0x78
  unsigned long v8; // stack - 0x70
  unsigned long v9; // stack - 0x68
  
  v6 = a0;
  v5 = strlen(a0);
  v8 = v5;
  v9 = 0xffffffff;
  if (v5 <= 0xfffffff)
    v8 = v5 << 4;
  v8 += 1;
  v7 = malloc(v8);
  if (!v7) {
    *__errno_location() = 0xc;
    return NULL;
  }
  iconv(a1,0,0,0,0);
  v4 = v7;
  v1 = v8;
  while( true ) {
    v3 = v1 - 1;
    v10 = iconv(a1,&v6,&v5,&v4,&v3);
    if ((v10 != -1) || (*__errno_location() == 0x16)) break;
    if (*__errno_location() != 7) {
      free(v7); // return-dupe
      return NULL;
    }
    v11 = (long)v4 - (long)v7;
    v12 = v8 * 2;
    if (v12 <= v8) {
      *__errno_location() = 0xc;
      free(v7);
      return NULL;
    }
    v13 = realloc(v7,v12);
    if (!v13) {
      *__errno_location() = 0xc;
      free(v7);
      return NULL;
    }
    v8 = v12;
    v4 = &v13[v11];
    v1 = v12 - v11;
    v7 = v13;
  }
  while( true ) {
    v14 = iconv(a1,0,0,&v4,&v3);
    if (v14 != -1) {
      v2 = &v4[1];
      *v4 = 0;
      v15 = (long)v2 - (long)v7;
      if ((v15 < v8) && (v4 = v2, v2 = realloc(v7,v15), v2))
        v7 = v2;
      return v7;
    }
    if (*__errno_location() != 7) {
      free(v7);
      return NULL;
    }
    v16 = (long)v4 - (long)v7;
    v17 = v8 * 2;
    if (v17 <= v8) break;
    v18 = realloc(v7,v17);
    if (!v18) {
      *__errno_location() = 0xc;
      free(v7);
      return NULL;
    }
    v8 = v17;
    v4 = &v18[v16];
    v3 = (v17 - v16) + -1;
    v7 = v18;
  }
  *__errno_location() = 0xc;
  free(v7);
  return NULL;
}

// Function: sub_21ef6 @ 0x21ef6
char * sub_21ef6(char *a0,char *a1,char *a2)
{
  int v1;
  char *v2; // rax
  long v3; // rax
  
  if ((*a0) && (sub_1dd22(a1,a2))) {
    v3 = iconv_open(a2,a1);
    if (v3 == -1)
      return NULL;
    v2 = (char *)sub_21bd3(a0,v3);
    if (v2) { // branch-flip
      if ((int)iconv_close(v3) <= -1) {
        free(v2);
        return NULL;
      }
    }
    else {
      v1 = *__errno_location();
      iconv_close(v3);
      *__errno_location() = v1;
    }
    return v2;
  }
  v2 = strdup(a0);
  if (!v2)
    *__errno_location() = 0xc;
  return v2;
}

// Function: sub_21fe9 @ 0x21fe9
char * sub_21fe9(long a0,unsigned long *a1) // early-return
{
  char *v1;
  unsigned long v10; // stack - 0x28
  unsigned long *v11; // stack - 0x20
  unsigned long *v2;
  unsigned long v3; // rax
  char *v4; // rax
  unsigned long *v5;
  unsigned long v6;
  unsigned long v7; // stack - 0x60
  long v8; // stack - 0x58
  char *v9; // stack - 0x50
  
  v7 = 0;
  v10 = *a1;
  v11 = (unsigned long *)a1[1];
  v6 = a1[2];
  for (v8 = a0; v8; v8 = v8 + -1) {
    if (0x30 <= (unsigned int)v10) { // branch-flip
      v5 = &v11[1];
      v2 = v11;
      v11 = v5;
    }
    else {
      v3 = v10 & 0xffffffff;
      v10 = (unsigned long)((unsigned int)v10 + 8);
      v2 = (unsigned long *)(v6 + v3);
    }
    v3 = strlen((char *)*v2);
    v7 = sub_227d4(v7,v3);
  }
  if ((v7 != 0xffffffffffffffff) && (v7 <= 0x7fffffff)) {
    v4 = (char *)sub_1c945(v7 + 1);
    v9 = v4;
    for (v8 = a0; v8; v8 = v8 + -1) {
      if (0x30 <= (unsigned int)*a1) { // branch-flip
        v5 = (unsigned long *)a1[1];
        a1[1] = (unsigned long)&v5[1];
      }
      else {
        v5 = (unsigned long *)((unsigned long)(unsigned int)*a1 + a1[2]);
        *(unsigned int *)a1 = (unsigned int)*a1 + 8;
      }
      v1 = (char *)*v5;
      v6 = strlen(v1);
      memcpy(v9,v1,v6);
      v9 = &v9[v6];
    }
    *v9 = 0;
    return v4;
  }
  *__errno_location() = 0x4b;
  return NULL;
}

// Function: sub_22193 @ 0x22193
unsigned long sub_22193(char *a0,unsigned long *a1)
{
  char *v1;
  char *v2; // stack - 0x28
  long v3; // stack - 0x20
  char *v4; // stack - 0x18
  
  v3 = 0;
  v4 = a0;
  while( true ) {
    if (!*v4)
      return sub_21fe9(v3,a1);
    v1 = v4;
    if ((*v4 != '%') || (v1 = &v4[1], v4[1] != 's')) break;
    v4 = &v4[2];
    v3 += 1;
  }
  v4 = v1;
  if (vasprintf(&v2,a0,a1) <= -1) {
    if (*__errno_location() != 0xc)
      return 0;
    sub_1cf09(); // no-return
  }
  return v2;
}

// Function: sub_2225e @ 0x2225e
int sub_2225e(int a0,int a1)
{
  unsigned long v1;
  unsigned long v2; // stack - 0xa8
  
  if (!a1)
    return sub_225d0(a0,(unsigned int)v2);
  if (a1 == 0x406)
    return sub_22601(a0,(unsigned int)v2);
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

// Function: sub_225d0 @ 0x225d0
int sub_225d0(int a0,unsigned int a1)
{
  return fcntl(a0,0,(unsigned long)a1);
}

// Function: sub_22601 @ 0x22601
int sub_22601(int a0,unsigned int a1)
{
  int v1;
  unsigned int v2; // eax
  int v3; // stack - 0x14
  
  if (0 <= dat_3da40) { // branch-flip
    v3 = fcntl(a0,0x406,(unsigned long)a1);
    if ((0 <= v3) || (*__errno_location() != 0x16))
      dat_3da40 = 1;
    else {
      v3 = sub_225d0(a0,a1);
      if (0 <= v3)
        dat_3da40 = -1;
    }
  }
  else {
    v3 = sub_225d0(a0,a1);
  }
  if ((0 <= v3) && (dat_3da40 == -1)) {
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

// Function: sub_22704 @ 0x22704
char * sub_22704(char *a0)
{
  bool v1;
  int v10; // stack - 0x34
  char *v2; // rax
  char v3 [8];
  char v4 [4]; // stack - 0x58
  unsigned long v5; // stack - 0x60
  char v6; // stack - 0x4c
  char *v7; // stack - 0x48
  long v8; // stack - 0x40
  char v9; // stack - 0x38
  
  v2 = (unsigned long)__ctype_get_mb_cur_max();
  if (2 <= v2) { // branch-flip
    v5 = 0;
    v4[0] = 0;
    v2 = v3;
    v7 = a0;
    memset(v2,0,8);
    while( true ) {
      v6 = 0;
      sub_16d35(v4);
      if ((v9 != '\x01') || (v10))
        v1 = 1;
      else {
        v1 = 0;
      }
      if (!v1) break;
      v5 += 1;
      v2 = &v7[v8];
      v7 = v2;
    }
    v2 = (char *)v5;
  }
  else {
    v2 = (unsigned long)strlen(a0);
  }
  return v2;
}

// Function: sub_227d4 @ 0x227d4
unsigned long sub_227d4(unsigned long a0,long a1) // early-return
{
  if (a0 <= a1 + a0)
    return a1 + a0;
  return 0xffffffffffffffff;
}

// Function: sub_2280c @ 0x2280c
void sub_2280c(unsigned long a0,long a1,long a2)
{
  sub_227d4(sub_227d4(a0,a1),a2);
}

// Function: sub_2284b @ 0x2284b
void sub_2284b(unsigned long a0,long a1,long a2,long a3)
{
  unsigned long v1; // rax
  
  v1 = sub_227d4(a0,a1);
  sub_227d4(sub_227d4(v1,a2),a3);
}

// Function: sub_228a0 @ 0x228a0
unsigned long sub_228a0(unsigned long a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = a1;
  if (a1 <= a0)
    v1 = a0;
  return v1;
}

// Function: _DT_FINI @ 0x228c4
void _DT_FINI(void)
{
  return;
}
