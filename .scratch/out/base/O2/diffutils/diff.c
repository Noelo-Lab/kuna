// Function: _DT_INIT @ 0x4000
void _DT_INIT(void) // return-dupe
{
  if (!dat_25fd8)
    return;
  (*dat_25fd8)();
}

// Function: sub_4020 @ 0x4020
void sub_4020(void)
{
  (*dat_25c00)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x47a0
void __cxa_finalize(void)
{
  (*dat_25fe8)(); // jump-as-call
}

// Function: __ctype_toupper_loc @ 0x47b0
void * __ctype_toupper_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25c08)(); // jump-as-call
  return v1;
}

// Function: getenv @ 0x47c0
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25c10)(); // jump-as-call
  return v1;
}

// Function: sigprocmask @ 0x47d0
int sigprocmask(int a0,void *a1,void *a2)
{
  int v1; // eax
  
  v1 = (*dat_25c18)(); // jump-as-call
  return v1;
}

// Function: raise @ 0x47e0
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_25c20)(); // jump-as-call
  return v1;
}

// Function: free @ 0x47f0
void free(void *a0)
{
  (*dat_25c28)(); // jump-as-call
}

// Function: strtoimax @ 0x4800
void strtoimax(void)
{
  (*dat_25c30)(); // jump-as-call
}

// Function: strcasecmp @ 0x4810
int strcasecmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_25c38)(); // jump-as-call
  return v1;
}

// Function: localtime @ 0x4820
void * localtime(void *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25c40)(); // jump-as-call
  return v1;
}

// Function: __mempcpy_chk @ 0x4830
void * __mempcpy_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25c48)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x4840
void abort(void)
{
  (*dat_25c50)(); // jump-as-call
}

// Function: __errno_location @ 0x4850
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_25c58)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x4860
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_25c60)(); // jump-as-call
  return v1;
}

// Function: localtime_r @ 0x4870
void * localtime_r(void *a0,void *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25c68)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x4880
void _exit(int a0)
{
  (*dat_25c70)(); // jump-as-call
}

// Function: strcpy @ 0x4890
char * strcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25c78)(); // jump-as-call
  return v1;
}

// Function: iconv @ 0x48a0
void iconv(void)
{
  (*dat_25c80)(); // jump-as-call
}

// Function: ferror @ 0x48b0
int ferror(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_25c88)(); // jump-as-call
  return v1;
}

// Function: qsort @ 0x48c0
void qsort(void)
{
  (*dat_25c90)(); // jump-as-call
}

// Function: isatty @ 0x48d0
int isatty(int a0)
{
  int v1; // eax
  
  v1 = (*dat_25c98)(); // jump-as-call
  return v1;
}

// Function: sigaction @ 0x48e0
int sigaction(int a0,void *a1,void *a2)
{
  int v1; // eax
  
  v1 = (*dat_25ca0)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x48f0
void iswcntrl(void)
{
  (*dat_25ca8)(); // jump-as-call
}

// Function: reallocarray @ 0x4900
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25cb0)(); // jump-as-call
  return v1;
}

// Function: readlink @ 0x4910
long readlink(char *a0,char *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_25cb8)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x4920
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_25cc0)(); // jump-as-call
  return v1;
}

// Function: clock_gettime @ 0x4930
int clock_gettime(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_25cc8)(); // jump-as-call
  return v1;
}

// Function: setenv @ 0x4940
int setenv(char *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_25cd0)(); // jump-as-call
  return v1;
}

// Function: write @ 0x4950
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_25cd8)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x4960
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25ce0)(); // jump-as-call
  return v1;
}

// Function: __open_2 @ 0x4970
void __open_2(void)
{
  (*dat_25ce8)(); // jump-as-call
}

// Function: fclose @ 0x4980
int fclose(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_25cf0)(); // jump-as-call
  return v1;
}

// Function: opendir @ 0x4990
void * opendir(char *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25cf8)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x49a0
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25d00)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x49b0
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25d08)(); // jump-as-call
  return v1;
}

// Function: regfree @ 0x49c0
void regfree(void)
{
  (*dat_25d10)(); // jump-as-call
}

// Function: __ctype_get_mb_cur_max @ 0x49d0
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_25d18)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x49e0
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_25d20)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x49f0
void __stack_chk_fail(void)
{
  (*dat_25d28)(); // jump-as-call
}

// Function: mmap @ 0x4a00
void mmap(void)
{
  (*dat_25d30)(); // jump-as-call
}

// Function: getopt_long @ 0x4a10
int getopt_long(int a0,char **a1,char *a2,void *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_25d38)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x4a20
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_25d40)(); // jump-as-call
  return v1;
}

// Function: dup2 @ 0x4a30
int dup2(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_25d48)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x4a40
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25d50)(); // jump-as-call
  return v1;
}

// Function: __overflow @ 0x4a50
int __overflow(void *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_25d58)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x4a60
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25d60)(); // jump-as-call
  return v1;
}

// Function: gmtime_r @ 0x4a70
void gmtime_r(void)
{
  (*dat_25d68)(); // jump-as-call
}

// Function: lseek @ 0x4a80
void lseek(void)
{
  (*dat_25d70)(); // jump-as-call
}

// Function: __assert_fail @ 0x4a90
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_25d78)(); // jump-as-call
}

// Function: fputs @ 0x4aa0
int fputs(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_25d80)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x4ab0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25d88)(); // jump-as-call
  return v1;
}

// Function: strnlen @ 0x4ac0
unsigned long strnlen(char *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_25d90)(); // jump-as-call
  return v1;
}

// Function: close @ 0x4ad0
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_25d98)(); // jump-as-call
  return v1;
}

// Function: pipe @ 0x4ae0
int pipe(int *a0)
{
  int v1; // eax
  
  v1 = (*dat_25da0)(); // jump-as-call
  return v1;
}

// Function: closedir @ 0x4af0
int closedir(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_25da8)(); // jump-as-call
  return v1;
}

// Function: regcomp @ 0x4b00
void regcomp(void)
{
  (*dat_25db0)(); // jump-as-call
}

// Function: memchr @ 0x4b10
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25db8)(); // jump-as-call
  return v1;
}

// Function: read @ 0x4b20
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_25dc0)(); // jump-as-call
  return v1;
}

// Function: lstat @ 0x4b30
int lstat(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_25dc8)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x4b40
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_25dd0)(); // jump-as-call
  return v1;
}

// Function: _setjmp @ 0x4b50
void _setjmp(void)
{
  (*dat_25dd8)(); // jump-as-call
}

// Function: fputs_unlocked @ 0x4b60
int fputs_unlocked(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_25de0)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x4b70
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25de8)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x4b80
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25df0)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x4b90
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_25df8)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x4ba0
void signal(void)
{
  (*dat_25e00)(); // jump-as-call
}

// Function: sigemptyset @ 0x4bb0
int sigemptyset(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_25e08)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x4bc0
int stat(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_25e10)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x4bd0
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25e18)(); // jump-as-call
  return v1;
}

// Function: tzset @ 0x4be0
void tzset(void)
{
  (*dat_25e20)(); // jump-as-call
}

// Function: fileno @ 0x4bf0
int fileno(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_25e28)(); // jump-as-call
  return v1;
}

// Function: readdir @ 0x4c00
void * readdir(void *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25e30)(); // jump-as-call
  return v1;
}

// Function: mincore @ 0x4c10
void mincore(void)
{
  (*dat_25e38)(); // jump-as-call
}

// Function: wcwidth @ 0x4c20
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_25e40)(); // jump-as-call
  return v1;
}

// Function: re_set_syntax @ 0x4c30
void re_set_syntax(void)
{
  (*dat_25e48)(); // jump-as-call
}

// Function: iswalnum @ 0x4c40
void iswalnum(void)
{
  (*dat_25e50)(); // jump-as-call
}

// Function: malloc @ 0x4c50
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25e58)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x4c60
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25e60)(); // jump-as-call
  return v1;
}

// Function: __vasprintf_chk @ 0x4c70
int __vasprintf_chk(char **a0,int a1,char *a2,void *a3)
{
  int v1; // eax
  
  v1 = (*dat_25e68)(); // jump-as-call
  return v1;
}

// Function: regexec @ 0x4c80
void regexec(void)
{
  (*dat_25e70)(); // jump-as-call
}

// Function: strcoll @ 0x4c90
int strcoll(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_25e78)(); // jump-as-call
  return v1;
}

// Function: mktime @ 0x4ca0
void mktime(void)
{
  (*dat_25e80)(); // jump-as-call
}

// Function: re_search @ 0x4cb0
void re_search(void)
{
  (*dat_25e88)(); // jump-as-call
}

// Function: fwrite_unlocked @ 0x4cc0
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_25e90)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x4cd0
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25e98)(); // jump-as-call
  return v1;
}

// Function: fdopen @ 0x4ce0
void * fdopen(int a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25ea0)(); // jump-as-call
  return v1;
}

// Function: munmap @ 0x4cf0
int munmap(void *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = (*dat_25ea8)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x4d00
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25eb0)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x4d10
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_25eb8)(); // jump-as-call
  return v1;
}

// Function: iconv_close @ 0x4d20
void iconv_close(void)
{
  (*dat_25ec0)(); // jump-as-call
}

// Function: timegm @ 0x4d30
void timegm(void)
{
  (*dat_25ec8)(); // jump-as-call
}

// Function: iswspace @ 0x4d40
int iswspace(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_25ed0)(); // jump-as-call
  return v1;
}

// Function: strftime @ 0x4d50
unsigned long strftime(char *a0,unsigned long a1,char *a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_25ed8)(); // jump-as-call
  return v1;
}

// Function: mempcpy @ 0x4d60
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25ee0)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x4d70
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25ee8)(); // jump-as-call
  return v1;
}

// Function: error @ 0x4d80
void error(int a0,int a1,char *a2,...)
{
  (*dat_25ef0)(); // jump-as-call
}

// Function: waitpid @ 0x4d90
int waitpid(int a0,int *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_25ef8)(); // jump-as-call
  return v1;
}

// Function: open @ 0x4da0
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_25f00)(); // jump-as-call
  return v1;
}

// Function: fopen @ 0x4db0
void * fopen(char *a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25f08)(); // jump-as-call
  return v1;
}

// Function: perror @ 0x4dc0
void perror(char *a0)
{
  (*dat_25f10)(); // jump-as-call
}

// Function: towlower @ 0x4dd0
unsigned int towlower(unsigned int a0)
{
  unsigned int v1; // eax
  
  v1 = (*dat_25f18)(); // jump-as-call
  return v1;
}

// Function: unsetenv @ 0x4de0
int unsetenv(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_25f20)(); // jump-as-call
  return v1;
}

// Function: getpagesize @ 0x4df0
int getpagesize(void)
{
  int v1; // eax
  
  v1 = (*dat_25f28)(); // jump-as-call
  return v1;
}

// Function: sigismember @ 0x4e00
void sigismember(void)
{
  (*dat_25f30)(); // jump-as-call
}

// Function: exit @ 0x4e10
void exit(int a0)
{
  (*dat_25f38)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x4e20
int __fprintf_chk(void *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_25f40)(); // jump-as-call
  return v1;
}

// Function: re_compile_pattern @ 0x4e30
void re_compile_pattern(void)
{
  (*dat_25f48)(); // jump-as-call
}

// Function: fflush_unlocked @ 0x4e40
int fflush_unlocked(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_25f50)(); // jump-as-call
  return v1;
}

// Function: strdup @ 0x4e50
char * strdup(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_25f58)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x4e60
int mbsinit(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_25f60)(); // jump-as-call
  return v1;
}

// Function: sigaltstack @ 0x4e70
void sigaltstack(void)
{
  (*dat_25f68)(); // jump-as-call
}

// Function: iswprint @ 0x4e80
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_25f70)(); // jump-as-call
  return v1;
}

// Function: fstat @ 0x4e90
int fstat(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_25f78)(); // jump-as-call
  return v1;
}

// Function: getc @ 0x4ea0
int getc(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_25f80)(); // jump-as-call
  return v1;
}

// Function: sigaddset @ 0x4eb0
int sigaddset(void *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_25f88)(); // jump-as-call
  return v1;
}

// Function: execv @ 0x4ec0
void execv(void)
{
  (*dat_25f90)(); // jump-as-call
}

// Function: fork @ 0x4ed0
int fork(void)
{
  int v1; // eax
  
  v1 = (*dat_25f98)(); // jump-as-call
  return v1;
}

// Function: __ctype_tolower_loc @ 0x4ee0
void * __ctype_tolower_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25fa0)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x4ef0
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_25fa8)(); // jump-as-call
  return v1;
}

// Function: iconv_open @ 0x4f00
void iconv_open(void)
{
  (*dat_25fb0)(); // jump-as-call
}

// Function: __sprintf_chk @ 0x4f10
int __sprintf_chk(char *a0,int a1,unsigned long a2,char *a3,...)
{
  int v1; // eax
  
  v1 = (*dat_25fb8)(); // jump-as-call
  return v1;
}

// Function: sub_4f20 @ 0x4f20
unsigned long sub_4f20(int a0,char **a1)
{
  bool v1;
  unsigned long v10; // rax
  unsigned long v11; // rax
  unsigned long v12; // rax
  char *v13;
  char *v14;
  unsigned long v15; // rcx
  unsigned long v16; // rdx
  long v17;
  char *v18; // stack - 0x48
  long v19; // stack - 0x58
  void *v2;
  long v20; // stack - 0x50
  int v21; // ebp
  long v22;
  char *v23;
  unsigned long v24;
  char **v25;
  char **v26;
  long v27;
  unsigned long v28; // stack - 0x88
  long v29; // stack - 0x80
  bool v3;
  int v4; // eax
  unsigned int v5; // eax
  int v6;
  char *v7; // rax
  unsigned long v8;
  unsigned long v9;
  
  v22 = (long)a0;
  v19 = 0;
  v20 = 0;
  dat_260b0 = 2;
  sub_14480(*a1);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_fd50(0);
  dat_26600 = 0x26500;
  dat_265c0 = 0x264c0;
  re_set_syntax(0x50a46);
  dat_26410 = sub_10310();
  dat_26180 = 0;
  sub_16f50();
  v3 = 0;
  v28 = 0;
  v1 = 0;
  v29 = -1;
  v6 = -1;
label_5020:
  v21 = (int)v22;
  v4 = getopt_long(v21,a1,"0123456789abBcC:dD:eEfF:hHiI:lL:nNpPqrsS:tTuU:vwW:x:X:yZ",(void *)0x25180,NULL);
  v9 = dat_36860;
  v23 = optarg;
  if (v4 != -1) {
    switch(v4) {
      case 0:
      case 0x68:
      case 0x80:
      case 0x85:
        break;
      default:
        sub_8e40(0,0); // no-return
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
        if (10 <= (unsigned int)(v6 - 0x30U)) { // branch-flip
          v29 = (long)(v4 + -0x30);
          v6 = v4;
        }
        else if (0x666666666666666 <= (long)(v29 - (unsigned long)(v4 <= 0x33))) { // branch-flip
          v29 = 0x3fffffffffffffff;
          v6 = v4;
        }
        else {
          v29 = (long)(v4 + -0x30) + v29 * 10;
          v6 = v4;
        }
        goto label_5020;
      case 0x42:
        dat_26554 = 1;
        v6 = v4;
        goto label_5020;
      case 0x43:
      case 0x55:
        if (optarg) { // branch-flip
          v17 = strtoimax(optarg,&v18,10);
          if ((*v18) || (v17 < 0))
            sub_8e40("invalid context length \'%s\'",optarg); // no-return
          v27 = 0x3fffffffffffffff;
          if (v17 <= 0x3fffffffffffffff)
            v27 = v17;
        }
        else {
          v27 = 3;
        }
        sub_8f60((v4 == 0x55) + '\x02');
        if (v27 <= dat_26570) { // branch-flip
          v1 = 1;
          v6 = v4;
        }
        else {
          v1 = 1;
          dat_26570 = v27;
          v6 = v4;
        }
        goto label_5020;
      case 0x44:
        sub_8f60(7);
        v8 = strlen(optarg);
        v23 = (char *)sub_16a40(v8 * 7 + 0x61);
        __sprintf_chk(v23,1,0xffffffffffffffff,"%%=%c#ifndef %s\n%%<#endif /* ! %s */\n%c#ifdef %s\n%%>#endif /* %s */\n%c#ifndef %s\n%%<#else /* %s */\n%%>#endif /* %s */\n",0,optarg,optarg,0,optarg,optarg,0,optarg,optarg,optarg);
        v22 = 0x26460;
        do {
          v27 = v22 + 8;
          sub_8f00(v22,v23,"-D");
          v23 = &v23[strlen(v23) + 1];
          v22 = v27;
        } while (v27 != 0x26480);
        v22 = (long)v21;
        v6 = v4;
        goto label_5020;
      case 0x45:
        v6 = v4;
        if (dat_26558 <= 3)
          dat_26558 |= 1;
        goto label_5020;
      case 0x46:
        sub_8d60(0x265e0,optarg);
        v6 = v4;
        goto label_5020;
      case 0x48:
        dat_26418 = 1;
        v6 = v4;
        goto label_5020;
      case 0x49:
        sub_8d60(0x265a0,optarg);
        v6 = v4;
        goto label_5020;
      case 0x4c:
        if (dat_26540) { // branch-flip
          if (dat_26548)
            sub_e7b0("too many file label options"); // no-return
          dat_26548 = optarg;
          v6 = v4;
        }
        else {
          dat_26540 = optarg;
          v6 = v4;
        }
        goto label_5020;
      case 0x4e:
        dat_26586 = 1;
        v6 = v4;
        goto label_5020;
      case 0x50:
        dat_26585 = 1;
        v6 = v4;
        goto label_5020;
      case 0x53:
        sub_8f00(0x26488,optarg,"-S");
        v6 = v4;
        goto label_5020;
      case 0x54:
        dat_26492 = 1;
        v6 = v4;
        goto label_5020;
      case 0x57:
        v8 = strtoimax(optarg,&v18,10);
        if (((long)v8 <= 0) || (*v18))
          sub_8e40("invalid width \'%s\'",optarg); // no-return
        if (v8 != v28) {
          if (v28)
            sub_e7b0("conflicting width options"); // no-return
          v28 = v8;
          v6 = v4;
          goto label_5020;
        }
        break;
      case 0x58:
        v6 = sub_10bd0(sub_10680,dat_26410,optarg,(-(unsigned int)(dat_26551 == '\0') & 0xfffffff0) + 0x10000010,10);
        if (v6)
          sub_e770(optarg); // no-return
        break;
      case 0x5a:
        v6 = v4;
        if (dat_26558 <= 3)
          dat_26558 |= 2;
        goto label_5020;
      case 0x61:
        dat_26568 = 1;
        v6 = v4;
        goto label_5020;
      case 0x62:
        v6 = v4;
        if (dat_26558 <= 3)
          dat_26558 = 4;
        goto label_5020;
      case 99:
        v9 = 2;
        goto label_5968;
      case 100:
        dat_26408 = 1;
        v6 = v4;
        goto label_5020;
      case 0x65:
        sub_8f60(4);
        v6 = v4;
        goto label_5020;
      case 0x66:
        sub_8f60(5);
        v6 = v4;
        goto label_5020;
      case 0x69:
        dat_26552 = 1;
        v6 = v4;
        goto label_5020;
      case 0x6c:
        if (!s_1e558[0])
          sub_8e40("pagination not supported on this host",0); // no-return
        dat_26480 = 1;
        signal(0x11,0);
        v6 = v4;
        goto label_5020;
      case 0x6e:
        sub_8f60(6);
        v6 = v4;
        goto label_5020;
      case 0x70:
        sub_8d60(0x265e0,"^[[:alpha:]$_]");
        v3 = 1;
        v6 = v4;
        goto label_5020;
      case 0x71:
        dat_264a1 = 1;
        v6 = v4;
        goto label_5020;
      case 0x72:
        dat_26608 = 1;
        v6 = v4;
        goto label_5020;
      case 0x73:
        dat_26584 = 1;
        v6 = v4;
        goto label_5020;
      case 0x74:
        dat_264a0 = '\x01';
        v6 = v4;
        goto label_5020;
      case 0x75:
        v9 = 3;
label_5968:
        sub_8f60(v9);
        v6 = v4;
        if (dat_26570 <= 2)
          dat_26570 = 3;
        goto label_5020;
      case 0x76:
        v9 = sub_14920("Len Tower");
        v10 = sub_14920("Richard Stallman");
        v11 = sub_14920("David Hayes");
        v12 = sub_14920("Mike Haertel");
        sub_16840(stdout,"diff","GNU diffutils",dat_260a8,sub_14920("Paul Eggert"),v12,v11,v10,v9,0);
        sub_9df0();
        return 0;
      case 0x77:
        dat_26558 = 5;
        v6 = v4;
        goto label_5020;
      case 0x78:
        sub_10680(dat_26410,optarg,(-(unsigned int)(dat_26551 == '\0') & 0xfffffff0) + 0x10000010);
        v6 = v4;
        goto label_5020;
      case 0x79:
        sub_8f60(8);
        v6 = v4;
        goto label_5020;
      case 0x81:
        sub_8f00(&v19,optarg,"--from-file");
        v6 = v4;
        goto label_5020;
      case 0x82:
        __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... FILES\n",5),v9);
        __printf_chk(1,"%s\n\n",dcgettext(NULL,"Compare FILES line by line.",5));
        v2 = stdout;
        v22 = 0x24f40;
        fputs_unlocked(dcgettext(NULL,"Mandatory arguments to long options are mandatory for short options too.\n",5),v2);
        v23 = "    --normal                  output a normal diff (the default)";
        do {
          if (*v23) { // branch-flip
            v23 = dcgettext(NULL,v23,5);
            while (v7 = strchr(v23,10), v7) {
              v8 = (long)&v7[1] - (long)v23;
              if (0xfff < (int)v8)
                __assert_fail("msglen < 4096","diff.c",0x3f8,"usage"); // no-return
              __printf_chk(1,"  %.*s",v8 & 0xffffffff,v23);
              v23 = &v7[1];
            }
            __printf_chk(1,&"  %s\n"[(int)(((unsigned int)(*v23 != '-') & (unsigned int)CONCAT71((undefined7)((unsigned long)v16 >> 8),*v23 != ' ')) * 2)],v23);
            v23 = *(char **)(v22 + 8);
          }
          else {
            v14 = *(char **)((long)stdout + 0x28);
            if (*(char **)((long)stdout + 0x30) <= v14) // branch-flip
              __overflow(stdout,10);
            else {
              *(char **)((long)stdout + 0x28) = &v14[1];
              *v14 = 10;
            }
            v23 = *(char **)(v22 + 8);
          }
          v22 += 8;
        } while (v23);
        sub_16920();
        sub_9df0();
        return 0;
      case 0x83:
        v27 = strtoimax(optarg,&v18,10);
        if ((*v18) || (v27 < 0))
          sub_8e40("invalid horizon length \'%s\'",optarg); // no-return
        if (dat_26560 < v27)
          dat_26560 = v27;
        v6 = v4;
        goto label_5020;
      case 0x84:
        dat_26551 = '\x01';
        v6 = v4;
        goto label_5020;
      case 0x86:
        dat_26439 = 1;
        v6 = v4;
        goto label_5020;
      case 0x87:
        sub_8f60(7);
        v27 = 0x26440;
        do {
          v17 = v27 + 8;
          sub_8f00(v27,optarg,"--line-format");
          v27 = v17;
        } while (v17 != 0x26458);
        v6 = v4;
        goto label_5020;
      case 0x88:
        dat_26550 = 1;
        v6 = v4;
        goto label_5020;
      case 0x89:
        dat_26551 = '\0';
        v6 = v4;
        goto label_5020;
      case 0x8a:
        sub_8f60(1);
        v6 = v4;
        goto label_5020;
      case 0x8b:
        sub_8f60(8);
        dat_2643a = 1;
        v6 = v4;
        goto label_5020;
      case 0x8c:
        dat_26490 = 1;
        v6 = v4;
        goto label_5020;
      case 0x8d:
        dat_26491 = 1;
        v6 = v4;
        goto label_5020;
      case 0x8e:
        dat_26438 = 1;
        v6 = v4;
        goto label_5020;
      case 0x8f:
        v8 = strtoimax(optarg,&v18,10);
        if ((((long)v8 <= 0) || (0xfffffffffffffffd <= v8)) || (*v18))
          sub_8e40("invalid tabsize \'%s\'",optarg); // no-return
        if (v8 != dat_26498) {
          if (dat_26498)
            sub_e7b0("conflicting tabsize options"); // no-return
          dat_26498 = v8;
          v6 = v4;
          goto label_5020;
        }
        break;
      case 0x90:
        sub_8f00(&v20,optarg,"--to-file");
        v6 = v4;
        goto label_5020;
      case 0x91:
      case 0x92:
      case 0x93:
        sub_8f60(7);
        v6 = v4 + -0x91;
        sub_8f00((long)v6 * 8 + 0x26440,optarg,&"--unchanged-line-format"[(long)v6 * 0x18]);
        goto label_5020;
      case 0x94:
      case 0x95:
      case 0x96:
      case 0x97:
        sub_8f60(7);
        v6 = v4 + -0x94;
        sub_8f00((long)v6 * 8 + 0x26460,optarg,&"--unchanged-group-format"[(long)v6 * 0x19]);
        goto label_5020;
      case 0x98:
        if ((optarg) && (v6 = strcmp(optarg,"auto"), v6)) {
          if (strcmp(v23,"always")) { // branch-flip
            if (strcmp(v23,"never"))
              sub_8e40("invalid color \'%s\'",v23); // no-return
            dat_2657c = 0;
            v6 = v4;
          }
          else {
            dat_2657c = 2;
            v6 = v4;
          }
        }
        else {
          dat_2657c = 1;
          v6 = v4;
        }
        goto label_5020;
      case 0x99:
        sub_e7f0(optarg);
        v6 = v4;
        goto label_5020;
      case 0x9a:
        goto label_5729;
      
    }
    v6 = v4;
    goto label_5020;
  }
  if (((dat_2657c == 1) && (v23 = getenv("TERM"), v23)) && (!strcmp(v23,"dumb")))
    dat_2657c = 0;
  if (dat_26580) { // branch-flip
label_5d2d:
    if ((dat_26580 != 2) || (sub_10f10(2)))
      dat_26400 = "%Y-%m-%d %H:%M:%S.%N %z";
    else {
      dat_26400 = "%a %b %e %T %Y";
    }
    if (0 <= v29) { // branch-flip
label_5b3f:
      v27 = dat_26570;
      if (((unsigned int)(dat_26580 - 2U) < 2) && ((dat_26570 < v29 || ((v29 != dat_26570 && (!v1)))))) {
        dat_26570 = v29;
        v27 = v29;
      }
    }
    else {
label_5d54:
      v27 = dat_26570;
    }
  }
  else {
    if (!v3) {
      sub_8f60(1);
      goto label_5d2d;
    }
    sub_8f60(2);
    if (0 <= v29) {
      if ((dat_26580 != 2) || (sub_10f10(2)))
        dat_26400 = "%Y-%m-%d %H:%M:%S.%N %z";
      else {
        dat_26400 = "%a %b %e %T %Y";
      }
      goto label_5b3f;
    }
    dat_26570 = 3;
    if (dat_26580 == 2) {
      if (sub_10f10(2)) {
        dat_26400 = "%Y-%m-%d %H:%M:%S.%N %z";
        v27 = dat_26570;
        goto label_5b58;
      }
      dat_26400 = "%a %b %e %T %Y";
      goto label_5d54;
    }
    dat_26400 = "%Y-%m-%d %H:%M:%S.%N %z";
    v27 = 3;
  }
label_5b58:
  if (!dat_26498)
    dat_26498 = 8;
  if (v28) { // branch-flip
    v8 = 1;
    if (!dat_264a0) {
label_5b88:
      v8 = dat_26498;
    }
    v15 = (v28 >> 1) + (v8 + 3 >> 1) + (unsigned long)((unsigned int)(v8 + 3) & (unsigned int)v28 & 1);
    v24 = v15 - v15 % v8;
    if ((4 <= v24) && (v24 < v28)) {
      dat_26430 = (v15 % v8 - v15) + v28;
      if (v24 - 3 < dat_26430)
        dat_26430 = v24 - 3;
      if (!dat_26430)
        v24 = v28;
      v28 = v24;
    }
    else {
      dat_26430 = 0;
    }
  }
  else {
    if (!dat_264a0) {
      v28 = 0x82;
      goto label_5b88;
    }
    v28 = 0x43;
    dat_26430 = 0x3f;
  }
  dat_26428 = v28;
  if (dat_26560 < v27)
    dat_26560 = v27;
  sub_8ea0(0x265e0);
  sub_8ea0(0x265a0);
  if (dat_26580 != 7) {
label_5c36:
    dat_26578 = dat_26580 != 8 | dat_26438;
    goto label_5c42;
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
label_604f:
      dat_26478 = (char *)sub_fa30(dat_26468,dat_26470,"");
      if (dat_26580 != 7) goto label_5c36;
    }
    v23 = dat_26460;
    v3 = 1;
    if (*dat_26460) goto label_5f6e;
  }
  else {
    v23 = "%=";
    dat_26460 = "%=";
    if (!dat_26478) goto label_604f;
label_5f6e:
    v3 = 0;
    if (!strcmp(v23,"%="))
      v3 = *dat_26440 == '\0';
  }
  dat_26578 = v3;
label_5c42:
  v25 = &a1[1];
  dat_26553 = ~(dat_26554 | dat_26552 | dat_26490 | (dat_265a0 || dat_26558)) & dat_264a1 & 1;
  if (2 <= optind) { // branch-flip
    v5 = optind - 2;
    v27 = 1;
    v26 = v25;
    do {
      v23 = *v26;
      v26 = &v26[1];
      v27 = v27 + 1 + sub_14d70(v23);
    } while (&a1[(unsigned long)v5 + 2] != v26);
    v13 = (char *)sub_16a40(v27);
    v14 = v13;
    do {
      *v14 = 0x20;
      v23 = *v25;
      v25 = &v25[1];
      v14 = (char *)sub_14de0(&v14[1],v23);
    } while (&a1[(unsigned long)v5 + 2] != v25);
  }
  else {
    v14 = (char *)sub_16a40(1);
    v13 = v14;
  }
  v17 = v20;
  v27 = v19;
  *v14 = 0;
  dat_26420 = v13;
  if (v19) { // branch-flip
    if (v20)
      sub_e7b0("--from-file and --to-file both specified"); // no-return
    v6 = 0;
    for (; optind < v21; optind = optind + 1) {
      v4 = sub_8f90(0,v27,a1[optind]);
      if (v6 < v4)
        v6 = v4;
    }
  }
  else {
    v27 = (long)optind;
    if (v20) { // branch-flip
      v6 = 0;
      while ((int)v27 < v21) {
        v4 = sub_8f90(0,a1[v27],v17);
        if (v6 < v4)
          v6 = v4;
        optind += 1;
        v27 = (long)optind;
      }
    }
    else {
      if (v21 - optind != 2) {
        if (2 > v21 - optind)
          sub_8e40("missing operand after \'%s\'",a1[v22 + -1]); // no-return
        sub_8e40("extra operand \'%s\'",a1[v27 + 2]); // no-return
      }
      v6 = sub_8f90(0,a1[v27],a1[v27 + 1]);
    }
  }
  sub_e670();
  sub_9df0();
  exit(v6); // no-return
label_5729:
  dat_26180 = 1;
  v6 = v4;
  goto label_5020;
}

// Function: sub_6270 @ 0x6270
void sub_6270(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_25fc0)(sub_4f20,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_62a0 @ 0x62a0
void sub_62a0(void)
{
  return;
}

// Function: _FINI_0 @ 0x6310
void _FINI_0(void)
{
  if (!dat_26168) {
    if (dat_25fe8)
      __cxa_finalize(dat_26008);
    sub_62a0();
    dat_26168 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x6350
void _INIT_0(void)
{
  return;
}

// Function: sub_6360 @ 0x6360
unsigned long sub_6360(long a0,long a1,long a2,long a3,unsigned char a4,long *a5) // early-return x2, return-dupe
{
  long *v1;
  long v10; // rcx
  long v11; // rcx
  long v12;
  long v13;
  unsigned char v14;
  long v15;
  long v16;
  long v17; // rdx
  long v18;
  long v19;
  long *v2;
  long v20; // rbp
  long v21;
  long v22;
  long v23;
  long v24;
  long v25;
  bool v26;
  long v27; // r11
  long v28;
  long v29;
  long v3;
  long v30;
  long v31; // stack - 0x110
  long v32; // stack - 0x108
  long v33; // stack - 0x100
  long v34; // stack - 0xf8
  long v35; // stack - 0xf0
  long v36; // stack - 0xd8
  unsigned char v37; // stack - 0xb3
  long v38; // stack - 0xb0
  long v39; // stack - 0xa8
  long v4;
  long v40; // stack - 0x58
  bool v5;
  unsigned char v6;
  long v7;
  unsigned long v8; // rax
  long v9;
  
  v3 = a5[1];
  v4 = *a5;
  v34 = a0;
  v35 = a2;
  v37 = a4;
  do {
    if (a1 <= v34) goto label_6650;
label_63ad:
    if (a3 <= v35) goto label_6650;
    if (*(long *)(v4 + v34 * 8) != *(long *)(v3 + v35 * 8)) {
      while( true ) {
        v28 = a1;
        v30 = a3;
        if (*(long *)(v4 + -8 + a1 * 8) != *(long *)(v3 + -8 + a3 * 8)) break;
        a1 -= 1;
        a3 -= 1;
        if (a1 <= v34) goto label_6650;
        if (a3 <= v35) goto label_6650;
      }
label_6416:
      v16 = v35;
      v22 = v34;
      if (v35 == v30) {
        if (v28 > v34) {
          do {
            v3 = v34 * 8;
            v34 += 1;
            *(char *)(dat_262b8 + *(long *)(dat_262a8 + v3)) = 1;
          } while (v34 != v28);
          return 0;
        }
        return 0;
      }
      v36 = 1;
      v9 = v34 - v30;
      v7 = a5[2];
      v15 = a5[3];
      v23 = *a5;
      v10 = v28 - v35;
      v13 = a5[1];
      v11 = v34 - v35;
      v29 = v28 - v30;
      *(long *)(v7 + v11 * 8) = v34;
      v6 = (char)v11 - (char)v29;
      *(long *)(v15 + v29 * 8) = v28;
      v27 = 2 - v29;
      v12 = v11;
      v31 = v29;
      v32 = v11;
      if (v11 <= v9) goto label_6852;
label_64e5:
      v12 -= 1;
      *(unsigned long *)(v7 + -8 + v12 * 8) = 0xffffffffffffffff;
      if (v10 <= v32) goto label_6866;
label_6502:
      *(unsigned long *)(v7 + 0x10 + v32 * 8) = 0xffffffffffffffff;
      v18 = v32 + 1;
      v32 = v18;
      if (v18 < v12) goto label_687a;
label_6522:
      v26 = 0;
      do {
        v24 = *(long *)(v7 + -8 + v18 * 8);
        v25 = *(long *)(v7 + 8 + v18 * 8);
        if (v25 <= v24)
          v25 = v24 + 1;
        v19 = v25 - v18;
        v5 = v19 < v30;
        v24 = v25;
        if (v5 && v25 < v28) {
          while (*(long *)(v23 + v24 * 8) == *(long *)(v13 + v19 * 8)) {
            v24 += 1;
            v19 += 1;
            if ((v28 <= v24) || (v30 <= v19)) break;
          }
          if (0x15 <= v24 - v25)
            v26 = v5 && v25 < v28;
        }
        *(long *)(v7 + v18 * 8) = v24;
        v14 = (v29 <= v18 && v18 <= v31) & v6 & 1;
        if ((v14) && (*(long *)(v15 + v18 * 8) <= v24)) goto label_65bb;
        v18 -= 2;
        if (v18 < v12) {
          if (v29 <= v9) goto label_6890;
label_66e0:
          v29 -= 1;
          *(unsigned long *)(v15 + -8 + v29 * 8) = 0x7fffffffffffffff;
          do {
            if (v10 <= v31) // branch-flip
              v31 -= 1;
            else {
              *(unsigned long *)(v15 + 0x10 + v31 * 8) = 0x7fffffffffffffff;
              v31 += 1;
            }
            if (v29 <= v31) {
              v18 = v31;
              do {
                v24 = *(long *)(v15 + 8 + v18 * 8);
                v25 = *(long *)(v15 + -8 + v18 * 8);
                if (v24 <= v25)
                  v25 = v24 + -1;
                v19 = v25 - v18;
                v5 = v35 < v19;
                v24 = v25;
                if (v34 < v25 && v5) {
                  while (*(long *)(v23 + -8 + v24 * 8) == *(long *)(v13 + -8 + v19 * 8)) {
                    v24 -= 1;
                    v19 -= 1;
                    if ((v24 <= v34) || (v19 <= v35)) break;
                  }
                  if (0x15 <= v25 - v24)
                    v26 = v34 < v25 && v5;
                }
                *(long *)(v15 + v18 * 8) = v24;
                v14 = (v18 <= v32 && v12 <= v18) & ~v6 & 1;
                if ((v14) && (v24 <= *(long *)(v7 + v18 * 8))) goto label_65bb;
                v18 -= 2;
              } while (v29 <= v18);
            }
            if (!v37) {
              v14 = 200 < v36 & *(unsigned char *)&a5[4] & v26;
              if (v14) {
                if (v12 <= v32) {
                  v24 = 0;
                  v18 = v32;
                  do {
                    while( true ) {
                      v20 = v18 + -2;
                      v19 = *(long *)(v7 + 0x10 + v20 * 8);
                      v25 = (2U - v11) + v20;
                      v17 = (v19 - v34) * 2 - v25;
                      v21 = -v25;
                      if (-v25 <= -1)
                        v21 = v25;
                      v25 = (v21 + v36) * 0xc;
                      if (v25 < v24)
                        v25 = v24;
                      if ((((v17 <= v25) || (v19 <= v34 + 0x13)) || (v28 <= v19)) || (((v25 = v19 - v18, v25 <= v35 + 0x13 || (v30 <= v25)) || (*(long *)(v13 + -8 + v25 * 8) != *(long *)(v23 + (v19 + -1) * 8))))) break;
                      v18 = 0;
                      do {
                        if (*(long *)(v23 + (v19 + -1) * 8 + -8 + v18) != *(long *)(v13 + v25 * 8 + -0x10 + v18)) goto label_6930;
                        v18 -= 8;
                      } while (v18 != -0x98);
                      v38 = v19;
                      v39 = v25;
                      if (v20 < v12) goto label_6d24;
                      v24 = v17;
                      v18 = v20;
                    }
label_6930:
                    v18 = v20;
                  } while (v12 <= v20);
                  if (v24) {
label_6d24:
                    v15 = (v30 + v28) - (v39 + v38);
                    v7 = (v39 + v38) - (v34 + v35);
                    v37 = v14;
                    v14 = 0;
                    goto label_65e3;
                  }
                }
                if (v29 <= v31) {
                  v33 = 0;
                  v18 = v31;
label_6a9c:
                  do {
                    v21 = v18 + -2;
                    v24 = v27 + v21;
                    v25 = *(long *)(v15 + 0x10 + v21 * 8);
                    v19 = -v24;
                    if (-v24 <= -1)
                      v19 = v24;
                    v24 += (v28 - v25) * 2;
                    if ((((v19 + v36) * 0xc < v24) && (v33 < v24)) && ((v34 < v25 && (((v19 = v25 - v18, v25 < v28 + -0x13 && (v35 < v19)) && (v19 < v30 + -0x13)))))) {
                      v1 = (long *)(v23 + v25 * 8);
                      v2 = (long *)(v13 + v19 * 8);
                      if (*v2 == *v1) {
                        v18 = 1;
                        do {
                          if (v1[v18] != v2[v18]) goto label_6af8;
                          v18 += 1;
                        } while (v18 != 0x14);
                        v18 = v21;
                        v33 = v24;
                        v38 = v25;
                        v39 = v19;
                        if (v21 < v29) goto label_6b1f;
                        goto label_6a9c;
                      }
                    }
label_6af8:
                    v18 = v21;
                  } while (v29 <= v21);
                  if (v33) {
label_6b1f:
                    v15 = (v30 + v28) - (v39 + v38);
                    v7 = (v39 + v38) - (v34 + v35);
                    goto label_65e3;
                  }
                }
              }
              if (v36 < a5[5]) goto label_6841;
              v23 = -1;
              if (v12 <= v32) {
                do {
                  v13 = *(long *)(v7 + v32 * 8);
                  if (v28 <= v13)
                    v13 = v28;
                  v9 = v13 - v32;
                  if (v30 < v13 - v32) {
                    v13 = v30 + v32;
                    v9 = v30;
                  }
                  if (v23 < v9 + v13) {
                    v23 = v9 + v13;
                    v40 = v13;
                  }
                  v32 -= 2;
                } while (v12 <= v32);
              }
              v13 = 0x7fffffffffffffff;
              if (v29 <= v31) {
                do {
                  v7 = *(long *)(v15 + v31 * 8);
                  if (v7 <= v34)
                    v7 = v34;
                  v12 = v7 - v31;
                  if (v7 - v31 < v35) {
                    v7 = v35 + v31;
                    v12 = v35;
                  }
                  if (v12 + v7 < v13) {
                    v13 = v12 + v7;
                    v38 = v7;
                  }
                  v31 -= 2;
                } while (v29 <= v31);
              }
              v7 = v23 - (v34 + v35);
              v15 = (v30 + v28) - v13;
              if (v7 <= v15) { // branch-flip
                v39 = v13 - v38;
                v7 = v13 - (v34 + v35);
                v14 = 1;
              }
              else {
                v15 = (v30 + v28) - v23;
                v37 = 1;
                v39 = v23 - v40;
                v38 = v40;
                v14 = 0;
              }
              goto label_65e3;
            }
label_6841:
            v36 += 1;
            if (v9 < v12) goto label_64e5;
label_6852:
            v12 += 1;
            if (v32 < v10) goto label_6502;
label_6866:
            v18 = v32 + -1;
            v32 = v18;
            if (v12 <= v18) goto label_6522;
label_687a:
            v26 = 0;
            if (v9 < v29) goto label_66e0;
label_6890:
            v29 += 1;
          } while( true );
        }
      } while( true );
    }
    v34 += 1;
    v35 += 1;
  } while( true );
label_65bb:
  v15 = (v30 + v28) - (v19 + v24);
  v7 = (v19 + v24) - (v34 + v35);
  v37 = v14;
  v38 = v24;
  v39 = v19;
label_65e3:
  if (v7 <= v15) { // branch-flip
    v35 = v39;
    v34 = v38;
    a1 = v28;
    a3 = v30;
    v28 = v38;
    v30 = v39;
    v6 = v14;
    v14 = v37;
  }
  else {
    v16 = v39;
    a1 = v38;
    a3 = v39;
    v22 = v38;
    v6 = v37;
  }
  v37 = v6;
  v8 = sub_6360(v22,v28,v16,v30,v14,a5);
  if ((char)v8)
    return v8;
  if (v34 < a1) goto label_63ad;
label_6650:
  v28 = a1;
  v30 = a3;
  if (v34 == a1) {
    if (a3 > v35) {
      do {
        v3 = v35 * 8;
        v35 += 1;
        *(char *)(dat_263e8 + *(long *)(dat_263d8 + v3)) = 1;
      } while (v35 != a3);
      return 0;
    }
    return 0;
  }
  goto label_6416;
}

// Function: sub_6d90 @ 0x6d90
void sub_6d90(unsigned long a0,long a1)
{
  long v1;
  char *v2; // rax
  long v3;
  
  v1 = dat_26548;
  if (!dat_26548)
    v1 = *(long *)(a1 + 0x138);
  v3 = dat_26540;
  if (!dat_26540)
    v3 = *(long *)(a1 + 8);
  if (dat_264a1) // branch-flip
    v2 = dcgettext(NULL,"Files %s and %s differ\n",5);
  else {
    v2 = dcgettext(NULL,"Binary files %s and %s differ\n",5);
  }
  sub_e660(v2,v3,v1); // tail-call
}

// Function: sub_6e20 @ 0x6e20
char sub_6e20(int *a0)
{
  int *v1;
  long v10;
  void *v11;
  long v12;
  void *v13; // rax
  long v14;
  long v15;
  long *v16;
  unsigned long *v17;
  char *v18;
  long v19;
  char v2;
  long v20;
  unsigned long v21;
  void *v22;
  long v23;
  unsigned long *v24;
  long v25;
  int *v26;
  long v27;
  long v28;
  void *v29 [3]; // stack - 0x58
  unsigned long v3;
  unsigned long v30; // stack - 0x98
  char v31 [8];
  char v32 [8];
  char v33 [8];
  char v34 [8];
  unsigned long v35;
  void *v36;
  char *v37;
  void **v38;
  long v39; // r13
  unsigned long v4;
  char *v40;
  long v41;
  bool v42; // zf
  char v43;
  long v44; // stack - 0xf0
  unsigned long *v45; // stack - 0xe8
  void *v46; // stack - 0xe0
  unsigned long v47; // stack - 0x90
  long v48; // stack - 0x88
  long v49; // stack - 0x80
  unsigned long *v5;
  char v50; // stack - 0x78
  long v51; // stack - 0x70
  long v52; // stack - 0x60
  char v6;
  int v7; // eax
  unsigned long v8;
  unsigned long v9;
  
  v6 = sub_bbc0(a0,dat_26553);
  if (!v6) {
    v46 = (void *)sub_fab0(*(long *)&a0[0x7e] + *(long *)&a0[0x32] + 4);
    *(long *)&a0[0x46] = (long)v46 + 1;
    *(long *)&a0[0x92] = (long)v46 + *(long *)&a0[0x32] + 3;
    v10 = sub_16a40((*(long *)&a0[0x32] + *(long *)&a0[0x7e]) * 0x10);
    *(long *)&a0[0x40] = v10;
    v10 += *(long *)&a0[0x32] * 8;
    *(long *)&a0[0x42] = v10;
    v10 += *(long *)&a0[0x32] * 8;
    *(long *)&a0[0x8c] = v10;
    *(long *)&a0[0x8e] = v10 + *(long *)&a0[0x7e] * 8;
    v11 = (void *)sub_fab0(*(long *)&a0[0x4a] << 4);
    v29[0] = v11;
    v22 = (void *)((long)v11 + *(long *)&a0[0x4a] * 8);
    if (1 <= *(long *)&a0[0x32]) {
      v10 = *(long *)&a0[0x3e];
      v12 = 0;
      do {
        v20 = v12 * 8;
        v12 += 1;
        v16 = (long *)((long)v11 + *(long *)(v10 + v20) * 8);
        *v16 = *v16 + 1;
      } while (v12 < *(long *)&a0[0x32]);
    }
    v10 = *(long *)&a0[0x7e];
    if (1 <= v10) {
      v12 = *(long *)&a0[0x8a];
      v20 = 0;
      do {
        v10 = v20 * 8;
        v20 += 1;
        v16 = (long *)((long)v22 + *(long *)(v12 + v10) * 8);
        *v16 = *v16 + 1;
        v10 = *(long *)&a0[0x7e];
      } while (v20 < v10);
    }
    v24 = &v28;
    v29[1] = v22;
    v13 = (void *)sub_fab0(v10 + *(long *)&a0[0x32]);
    v38 = v29;
    v8 = *(unsigned long *)&a0[0x32];
    v52 = (long)v13 + v8;
    v16 = (long *)&a0[0x3e];
    v36 = v13;
    v17 = v24;
    do {
      v10 = *v16;
      v21 = v8 >> 8;
      if (v21) { // branch-flip
        v35 = 5;
        do {
          v35 *= 2;
          v21 >>= 2;
        } while (v21);
label_7184:
        v21 = 0;
        do {
          while (v12 = *(long *)(v10 + v21 * 8), !v12) {
label_718d:
            v21 += 1;
            if (v21 == v8) goto label_71bb;
          }
          v4 = *(unsigned long *)((long)v22 + v12 * 8);
          if (!v4) {
            *(char *)((long)v36 + v21) = 1;
            goto label_718d;
          }
          if (v35 >= v4) goto label_718d;
          *(char *)((long)v36 + v21) = 2;
          v21 += 1;
        } while (v21 != v8);
      }
      else if (v8) {
        v35 = 5;
        goto label_7184;
      }
label_71bb:
      v17 = &v17[1];
      if ((long *)&a0[0xd6] == &v16[0x26]) goto label_71cf;
      v22 = *v38;
      v8 = v16[0x20];
      v38 = &v38[-1];
      v36 = (void *)*v17;
      v16 = &v16[0x26];
    } while( true );
  }
  if ((((*(long *)&a0[0x10] != *(long *)&a0[0x5c] && 1 <= *(long *)&a0[0x10]) && (1 <= *(long *)&a0[0x5c])) && ((*a0 <= -1 || ((a0[10] & 0xf000U) == 0x8000)))) && ((a0[0x4c] <= -1 || ((a0[0x56] & 0xf000U) == 0x8000)))) {
label_6f97:
    v43 = 1;
    sub_6d90(1,a0);
    v22 = *(void **)&a0[0x28];
    v11 = *(void **)&a0[0x74];
  }
  else {
    v22 = *(void **)&a0[0x28];
    if (*a0 != a0[0x4c]) { // branch-flip
      v3 = *(unsigned long *)&a0[0x5e];
      v9 = *(unsigned long *)&a0[0x12];
      v8 = sub_17190(8,sub_17190(v9,v3,0x7ffffffffffffffe),0x7ffffffffffffffe);
      v9 = sub_16a90(v22,v8);
      v3 = *(unsigned long *)&a0[0x74];
      *(unsigned long *)&a0[0x28] = v9;
      *(unsigned long *)&a0[0x74] = sub_16a90(v3,v8);
      while( true ) {
        if (0 <= *a0)
          sub_b8a0(a0,v8 - *(long *)&a0[0x2c]);
        if (0 <= a0[0x4c])
          sub_b8a0(&a0[0x4c],v8 - *(long *)&a0[0x78]);
        v21 = *(unsigned long *)&a0[0x2c];
        if (v21 != *(unsigned long *)&a0[0x78]) goto label_6f97;
        v22 = *(void **)&a0[0x28];
        v11 = *(void **)&a0[0x74];
        if (memcmp(v22,v11,v21)) goto label_6f97;
        if (v21 != v8) break;
        a0[0x78] = 0;
        a0[0x79] = 0;
        a0[0x2c] = 0;
        a0[0x2d] = 0;
      }
      v43 = 0;
    }
    else {
      v11 = *(void **)&a0[0x74];
      v43 = 0;
    }
  }
label_6fc4:
  if (v22 != v11) {
    free(v22);
    v11 = *(void **)&a0[0x74];
  }
  free(v11);
  return v43;
label_71cf:
  v44 = 0;
  v22 = v13;
  v45 = v24;
  do {
    v10 = *(long *)((long)a0 + v44 + 200);
    v12 = 0;
    if (0 < v10) {
      do {
        while( true ) {
          v18 = (char *)((long)v22 + v12);
          if (*v18 != '\x02') break;
          v12 += 1;
          *v18 = '\0';
          if (v10 <= v12) goto label_722b;
        }
        if (*v18) {
          v25 = 0;
          v20 = v12;
          while ((v20 = v20 + 1, v20 < v10 && (*(char *)((long)v22 + v20)))) {
            v25 += (unsigned long)(*(char *)((long)v22 + v20) == '\x02');
          }
          if (v20 <= v12) { // branch-flip
            v27 = v20 - v12;
            if (v25 * 4 <= v27) {
label_7380:
              v20 = v27 >> 4;
              if (v20 <= 0) goto label_74b4;
              v25 = 1;
              do {
                v25 *= 2;
                v20 >>= 2;
              } while (v20);
              v20 = v12 + -1 + v27;
label_73a3:
              v19 = 0;
              v14 = 0;
              do {
                while (v40 = (char *)(v12 + v19 + (long)v22), *v40 != '\x02') {
                  v41 = 0;
label_73ad:
                  v19 += 1;
                  v14 = v41;
                  if (v19 >= v27) goto label_73e2;
                }
                v41 = v14 + 1;
                if (v14 == v25) {
                  v19 -= v41;
                  goto label_73ad;
                }
                if (v25 >= v14) goto label_73ad;
                v19 += 1;
                *v40 = '\0';
                v14 = v41;
              } while (v27 > v19);
label_73e2:
              v12 = 0;
              v40 = v18;
              do {
                while( true ) {
                  v6 = *v40;
                  if ((8 <= (long)v40 - (long)v18) && (v6 == '\x01')) goto label_7433;
                  if (v6 != '\x02') break;
                  *v40 = '\0';
                  v40 = &v40[1];
                  v12 = 0;
                  if (&v18[v27] == v40) goto label_7433;
                }
                if (v6) { // branch-flip
                  v12 += 1;
                  if (v12 == 3) break;
                }
                else {
                  v12 = 0;
                }
                v40 = &v40[1];
              } while (&v18[v27] != v40);
label_7433:
              v18 = (char *)((long)v22 + (v20 - v27));
              v12 = 0;
              v40 = (char *)((long)v22 + v20);
              do {
                while( true ) {
                  if ((8 <= ((long)v22 + v20) - (long)v40) && (*v40 == '\x01')) goto label_748d;
                  if (*v40 != '\x02') break;
                  *v40 = '\0';
                  v40 = &v40[-1];
                  v12 = 0;
                  if (v18 == v40) goto label_748d;
                }
                if (*v40) { // branch-flip
                  v12 += 1;
                  if (v12 == 3) break;
                }
                else {
                  v12 = 0;
                }
                v40 = &v40[-1];
              } while (v18 != v40);
label_748d:
              v12 = v20; // crossjump-dupe
            }
          }
          else {
            do {
              if (*(char *)((long)v22 + v20 + -1) != '\x02') {
                v27 = v20 - v12;
                if (v25 * 4 <= v27) goto label_7380;
                if (v12 < v20) {
                  v18 = (char *)((long)v22 + v20 + -1);
                  do {
                    if (*v18 == '\x02')
                      *v18 = '\0';
                    v18 = &v18[-1];
                  } while ((char *)((long)v22 + v12 + -1) != v18);
                }
                goto label_7208;
              }
              v20 -= 1;
              v25 -= 1;
              *(char *)((long)v22 + v20) = 0;
            } while (v12 != v20);
            v27 = 0;
            if (v25 <= 0) {
label_74b4:
              v20 = v12 + -1 + v27;
              if (1 <= v27) {
                v25 = 1;
                goto label_73a3;
              }
              v12 = v20;
            }
          }
        }
label_7208:
        v12 += 1;
      } while (v12 < v10);
    }
label_722b:
    v44 += 0x130;
    v45 = &v45[1];
    if (v44 == 0x260) goto label_724b;
    v22 = (void *)*v45;
  } while( true );
label_724b:
  v16 = (long *)&a0[0x32];
  v22 = v13;
  do {
    v10 = *v16;
    v12 = 0;
    if (1 <= v10) {
      v20 = 0;
      do {
        while ((!dat_26408 && (*(char *)((long)v22 + v20)))) {
          *(char *)(v16[10] + v20) = 1;
          v20 += 1;
          if (v10 == v20) goto label_72b8;
        }
        v25 = v12 * 8;
        *(unsigned long *)(v16[7] + v12 * 8) = *(unsigned long *)(v16[6] + v20 * 8);
        v12 += 1;
        *(long *)(v16[8] + v25) = v20;
        v20 += 1;
      } while (v10 != v20);
    }
label_72b8:
    v16[9] = v12;
    v16 = &v16[0x26];
    v24 = &v24[1];
    if ((long *)&a0[0xca] == v16) goto label_74db;
    v22 = (void *)*v24;
  } while( true );
label_74db:
  free(v13);
  free(v11);
  v30 = *(unsigned long *)&a0[0x40];
  v25 = *(long *)&a0[0x90] + *(long *)&a0[0x44] + 3;
  v47 = *(unsigned long *)&a0[0x8c];
  v20 = sub_16a40(v25 * 0x10);
  v12 = *(long *)&a0[0x90];
  v10 = v12 * 8 + 8;
  v48 = v20 + v10;
  v49 = v20 + v10 + v25 * 8;
  v50 = dat_26418;
  if (v25) { // branch-flip
    v51 = 1;
    do {
      v51 *= 2;
      v25 >>= 2;
    } while (v25);
    if (v51 <= 0xfff)
      v51 = 0x1000;
  }
  else {
    v51 = 0x1000;
  }
  v20 = 0;
  v10 = 0x26;
  v26 = a0;
  v24 = (unsigned long *)0x261a0;
  while (v10) {
    v17 = &v24[1];
    v1 = &v26[2];
    *v24 = *(unsigned long *)v26;
    v10 -= 1;
    v26 = v1;
    v24 = v17;
  }
  v10 = 0x26;
  v26 = &a0[0x4c];
  while (v10) {
    v17 = &v24[1];
    v1 = &v26[2];
    *v24 = *(unsigned long *)v26;
    v10 -= 1;
    v26 = v1;
    v24 = v17;
  }
  sub_6360(0,*(unsigned long *)&a0[0x44],0,v12,dat_26408,&v30);
  free((void *)(*(long *)&a0[0x90] * -8 + -8 + v48));
label_75e0:
  v10 = *(long *)((long)a0 + v20 + 0x118);
  v19 = 0;
  v14 = 0;
  v12 = *(long *)((long)a0 + v20 + 0xf8);
  v25 = *(long *)((long)a0 + v20 + 200);
  v27 = *(long *)((long)a0 + (0x248U - v20));
label_760f:
  v42 = v25 == v14;
  v41 = v14;
  if (v25 <= v14) goto label_7765;
label_7618:
  v23 = v41;
  if (!*(char *)(v10 + v41)) goto label_7921;
  do {
    v14 = v23;
    do {
      v14 += 1;
    } while (*(char *)(v10 + v14));
    for (; *(char *)(v27 + v19); v19 = v19 + 1) {
    }
    do {
      v39 = v14 - v23;
      while ((v23 && (*(long *)(v12 + -8 + v23 * 8) == *(long *)(v12 + -8 + v14 * 8)))) {
        v14 -= 1;
        *(char *)(v10 + v23 + -1) = 1;
        *(char *)(v10 + v14) = 0;
        v6 = *(char *)(v10 + -2 + v23);
        v23 -= 1;
        while (v6) {
          v23 -= 1;
          v6 = *(char *)(v10 + -1 + v23);
        }
        do {
          v19 -= 1;
        } while (*(char *)(v27 + v19));
      }
      v41 = v14;
      if (!*(char *)(v27 + -1 + v19))
        v41 = v25;
      while ((v14 != v25 && (*(long *)(v12 + v23 * 8) == *(long *)(v12 + v14 * 8)))) {
        v23 += 1;
        v15 = v14 + 1;
        *(char *)(v10 + -1 + v23) = 0;
        *(char *)(v10 + v14) = 1;
        if (!*(char *)(v10 + v15)) goto label_7703;
        do {
          v15 += 1;
        } while (*(char *)(v10 + v15));
        v19 += 1;
        v6 = *(char *)(v27 + v19);
        while (v6) {
          v41 = v15;
label_7703:
          v19 += 1;
          v6 = *(char *)(v27 + v19);
        }
        v14 = v15;
      }
    } while (v39 != v14 - v23);
    if (v14 <= v41) goto label_760f;
    v37 = (char *)(v10 + -1 + v14);
    do {
      v37[v23 - v14] = 1;
      *v37 = 0;
      do {
        v19 -= 1;
      } while (*(char *)(v27 + v19));
      v37 = &v37[-1];
    } while ((char *)(v10 + -1 + v41) != v37);
    v42 = v25 == v41;
    if (!v42 && v41 <= v25) goto label_7618;
label_7765:
    v23 = v41;
  } while (!v42);
  v20 += 0x130;
  if (v20 != 0x260) goto label_75e0;
  v10 = *(long *)&a0[0x46];
  v12 = *(long *)&a0[0x92];
  v8 = *(unsigned long *)&a0[0x32];
  v21 = *(unsigned long *)&a0[0x7e];
  if (dat_26580 != 4) { // branch-flip
    if ((long)(v8 & v21) < 0) {
label_7b50:
      if ((dat_26554) || (v17 = NULL, dat_264e0)) goto label_7b69;
      goto label_7c25;
    }
    v24 = NULL;
    do {
      v6 = *(char *)(v10 + -1 + v8);
      v2 = *(char *)(v12 + -1 + v21);
      v17 = v24;
      if (v6 || v2) {
        if (v6) { // branch-flip
          v35 = v8;
          do {
            v35 -= 1;
          } while (*(char *)(v10 + -1 + v35));
          v20 = v8 - v35;
          v8 = v35;
        }
        else {
          v20 = 0;
        }
        if (v2) { // branch-flip
          v35 = v21;
          do {
            v35 -= 1;
          } while (*(char *)(v12 + -1 + v35));
          v25 = v21 - v35;
          v21 = v35;
        }
        else {
          v25 = 0;
        }
        v17 = (unsigned long *)sub_16a40(0x30);
        *v17 = v24;
        v17[3] = v8;
        v17[4] = v21;
        v17[1] = v25;
        v17[2] = v20;
      }
      v21 -= 1;
      v8 -= 1;
      v24 = v17;
    } while ((0 <= (long)v8) || (0 <= (long)v21));
  }
  else {
    if (((long)v8 <= 0) && ((long)v21 <= 0)) goto label_7b50;
    v25 = 0;
    v20 = 0;
    v24 = NULL;
    do {
      v17 = v24;
      v27 = v20;
      v14 = v25;
      if (*(char *)(v10 + v20) || *(char *)(v12 + v25)) {
        if (*(char *)(v10 + v20)) { // branch-flip
          do {
            v27 += 1;
          } while (*(char *)(v10 + v27));
          v19 = v27 - v20;
        }
        else {
          v19 = 0;
        }
        if (*(char *)(v12 + v25)) { // branch-flip
          do {
            v14 += 1;
          } while (*(char *)(v12 + v14));
          v41 = v14 - v25;
        }
        else {
          v41 = 0;
        }
        v17 = (unsigned long *)sub_16a40(0x30);
        v17[3] = v20;
        v17[4] = v25;
        *v17 = v24;
        v17[1] = v41;
        v17[2] = v19;
      }
      v20 = v27 + 1;
      v25 = v14 + 1;
      v24 = v17;
    } while ((v20 < (long)v8) || (v25 < (long)v21));
  }
  if ((!dat_26554) && (!dat_264e0)) {
label_7c25:
    v43 = v17 != NULL;
label_786c:
    if (dat_264a1) { // branch-flip
      if ((bool)v43)
        sub_6d90(1,a0);
    }
    else {
      if (!(bool)v43) {
label_7b7c:
        v43 = 0;
        if (dat_26578) goto label_788b;
      }
      v10 = dat_26548;
      if (!dat_26548)
        v10 = *(long *)&a0[0x4e];
      v12 = dat_26540;
      if (!dat_26540)
        v12 = *(long *)&a0[2];
      sub_e800(v12,v10,*(long *)&a0[0x98] != 0);
      switch(dat_26580) {
        default:
          abort(); // no-return
        case 1:
          sub_cff0(v17);
          break;
        case 2:
          sub_8c10(v17,0);
          break;
        case 3:
          sub_8c10(v17,1);
          break;
        case 4:
          sub_ac20(v17);
          break;
        case 5:
          sub_ac40(v17);
          break;
        case 6:
          sub_ac60(v17);
          break;
        case 7:
          sub_b6c0(v17);
          break;
        case 8:
          sub_da50(v17);
        
      }
      sub_ead0();
    }
label_788b:
    free(*(void **)&a0[0x40]);
    free(v46);
    free(*(void **)&a0[0x3e]);
    free((void *)(*(long *)&a0[0x2e] + *(long *)&a0[0x30] * 8));
    free(*(void **)&a0[0x8a]);
    free((void *)(*(long *)&a0[0x7a] + *(long *)&a0[0x7c] * 8));
    while (v17) {
      v24 = (unsigned long *)*v17;
      free(v17);
      v17 = v24;
    }
    if ((unsigned int)(dat_26580 - 4U) < 2) {
      v16 = (long *)0x26540;
      v26 = &a0[0x48];
      do {
        if ((char)*v26) {
          v18 = dcgettext(NULL,"No newline at end of file",5);
          v10 = *v16;
          if (!v10)
            v10 = *(long *)&v26[-0x46];
          error(0,0,"%s: %s\n",v10,v18);
          v43 = 2;
        }
        v16 = &v16[1];
        v26 = &v26[0x4c];
      } while (v16 != (long *)0x26550);
    }
    v22 = *(void **)&a0[0x28];
    v11 = *(void **)&a0[0x74];
    goto label_6fc4;
  }
  if (!v17) {
label_7b69:
    v17 = NULL;
    v43 = 0;
    if (!dat_264a1) goto label_7b7c;
    goto label_788b;
  }
  v24 = v17;
  do {
    v16 = (long *)sub_f060(v24);
    v5 = (unsigned long *)*v16;
    *v16 = 0;
    v7 = sub_f670(v24,v31,v34,v33,v32);
    *v16 = (long)v5;
    if (!v5) break;
    v24 = v5;
  } while (!v7);
  v43 = v7 != 0;
  goto label_786c;
label_7921:
  do {
    v19 += 1;
  } while (*(char *)(v27 + -1 + v19));
  v14 = v41 + 1;
  goto label_760f;
}

// Function: sub_7cf0 @ 0x7cf0
void sub_7cf0(unsigned long a0,long a1,unsigned long a2,long a3)
{
  char *v1;
  unsigned long v2;
  void *v3; // rax
  long v4; // rax
  char v5 [56];
  
  sub_f510(0);
  if (a3) // branch-flip
    __fprintf_chk(dat_26188,1,"%s %s",a0,a3);
  else {
    v3 = localtime((void *)(a1 + 0x68));
    v2 = *(unsigned long *)(a1 + 0x70);
    if ((!v3) || (v4 = sub_14460(v5,0x2b,dat_26400,v3,0,v2 & 0xffffffff), !v4))
      __sprintf_chk(v5,1,0x2b,"%ld.%.9d",*(unsigned long *)(a1 + 0x68),v2 & 0xffffffff);
    __fprintf_chk(dat_26188,1,"%s %s\t%s",a0,a2,v5);
  }
  sub_f510(3);
  v1 = *(char **)((long)dat_26188 + 0x28);
  if (*(char **)((long)dat_26188 + 0x30) <= v1) {
    __overflow(dat_26188,10); // tail-call
    return;
  }
  *(char **)((long)dat_26188 + 0x28) = &v1[1];
  *v1 = 10;
}

// Function: sub_7e50 @ 0x7e50
unsigned long sub_7e50(long a0,long a1)
{
  long v1;
  unsigned long v2; // r8
  unsigned long v3;
  
  v1 = dat_26178;
  dat_26178 = a1;
  do {
    a1 -= 1;
    if (a1 < v1) {
      v3 = 0;
      if (dat_26170 != 0x7fffffffffffffff)
        v3 = *(unsigned long *)(a0 + dat_26170 * 8);
      return v3;
    }
    v3 = *(unsigned long *)(a0 + a1 * 8);
    v2 = ~v3 + *(long *)(a0 + 8 + a1 * 8);
    if (0x80000000 <= v2)
      v2 = 0x7fffffff;
  } while ((int)re_search(0x26500,v3,v2 & 0xffffffff,0,v2,0) <= -1);
  dat_26170 = a1;
  return v3;
}

// Function: sub_7f00 @ 0x7f00
void sub_7f00(void *a0,char *a1)
{
  char v1;
  char *v2;
  int v3; // ecx
  int v4;
  char *v5;
  char *v6;
  
  v2 = *(char **)((long)a0 + 0x28);
  if (*(char **)((long)a0 + 0x30) <= v2) // branch-flip
    __overflow(a0,0x20);
  else {
    *(char **)((long)a0 + 0x28) = &v2[1];
    *v2 = 0x20;
  }
  v3 = 0;
  v5 = a1;
  do {
    v1 = *v5;
    if ('\x0e' <= v1) { // branch-flip
      if (v1 != ' ') goto label_7f3d;
    }
    else if ((v1 <= '\b') || (v1 == '\n')) goto label_7f3d;
    v5 = &v5[1];
    v3 += 1;
  } while( true );
label_7f3d:
  v6 = &a1[(long)v3 + 1];
  v4 = v3;
  while ((v1 != '\n' && (v4 = v4 + 1, v4 <= v3 + 0x27))) {
    v1 = *v6;
    v6 = &v6[1];
  }
  if (v3 < v4) {
    a1 = &a1[(long)v4 + -1];
    do {
      v1 = *a1;
      if ('\x0e' <= v1) { // branch-flip
        if (v1 != ' ') break;
      }
      else if (v1 <= '\b') break;
      v4 -= 1;
      a1 = &a1[-1];
    } while (v3 != v4);
  }
  fwrite_unlocked(v5,1,(long)(v4 - v3),a0); // tail-call
}

// Function: sub_7fe0 @ 0x7fe0
void sub_7fe0(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  long v1; // stack - 0x20
  long v2; // stack - 0x18
  
  sub_f5e0(a0,a1,a2,&v1,&v2);
  if (v2 <= v1) {
    __fprintf_chk(dat_26188,1,"%ld",v2);
    return;
  }
  __fprintf_chk(dat_26188,1,"%ld,%ld");
}

// Function: sub_8070 @ 0x8070
void sub_8070(unsigned long *a0)
{
  char *v1;
  char *v10;
  unsigned long *v11;
  bool v2;
  unsigned int v3; // eax
  long v4;
  long v5; // stack - 0x50
  long v6; // stack - 0x58
  long v7; // stack - 0x60
  long v8; // stack - 0x48
  void *v9;
  
  v3 = sub_f670(a0,&v7,&v6,&v5,&v8);
  if (v3) {
    v4 = -dat_26288;
    v7 -= dat_26570;
    if (v7 < v4)
      v7 = v4;
    v5 -= dat_26570;
    if (v5 < v4)
      v5 = v4;
    v4 = dat_26570 + v6;
    v2 = v6 < dat_26270 - dat_26570;
    v6 = dat_26270 + -1;
    if (v2)
      v6 = v4;
    v4 = dat_26570 + v8;
    v2 = v8 < dat_263a0 - dat_26570;
    v8 = dat_263a0 + -1;
    if (v2)
      v8 = v4;
    if (dat_26520) { // branch-flip
      v4 = sub_7e50(dat_26258);
      sub_e830();
      v9 = dat_26188;
      fputs_unlocked("***************",dat_26188);
      if (v4)
        sub_7f00(v9,v4);
    }
    else {
      sub_e830();
      v9 = dat_26188;
      fputs_unlocked("***************",dat_26188);
    }
    v1 = *(char **)((long)v9 + 0x28);
    if (*(char **)((long)v9 + 0x30) <= v1) // branch-flip
      __overflow(v9,10);
    else {
      *(char **)((long)v9 + 0x28) = &v1[1];
      *v1 = 10;
    }
    sub_f510(4);
    fputs_unlocked("*** ",v9);
    sub_7fe0(0x261a0,v7,v6);
    fputs_unlocked(" ****",v9);
    sub_f510(3);
    v1 = *(char **)((long)v9 + 0x28);
    if (*(char **)((long)v9 + 0x30) <= v1) // branch-flip
      __overflow(v9,10);
    else {
      *(char **)((long)v9 + 0x28) = &v1[1];
      *v1 = 10;
    }
    if ((v3 & 1) && (v7 <= v6)) {
      v11 = a0;
      v4 = v7;
      do {
        sub_f510(2);
        for (; v11; v11 = (unsigned long *)*v11) {
          if (v4 < (long)(v11[2] + v11[3])) {
            if ((long)v11[3] <= v4) {
              v10 = "-";
              if (1 <= (long)v11[1])
                v10 = "!";
              goto label_8274;
            }
            break;
          }
        }
        v10 = " ";
label_8274:
        sub_f330(v10,dat_26258 + v4 * 8,1);
        sub_f510(3);
        if (*(char *)(*(long *)(dat_26258 + 8 + v4 * 8) + -1) == '\n') {
          v1 = *(char **)((long)v9 + 0x28);
          if (*(char **)((long)v9 + 0x30) <= v1) // branch-flip
            __overflow(v9,10);
          else {
            *(char **)((long)v9 + 0x28) = &v1[1];
            *v1 = 10;
          }
        }
        v4 += 1;
      } while (v4 <= v6);
    }
    sub_f510(4);
    fputs_unlocked("--- ",v9);
    sub_7fe0(0x262d0,v5,v8);
    fputs_unlocked(" ----",v9);
    sub_f510(3);
    v1 = *(char **)((long)v9 + 0x28);
    if (*(char **)((long)v9 + 0x30) <= v1) // branch-flip
      __overflow(v9,10);
    else {
      *(char **)((long)v9 + 0x28) = &v1[1];
      *v1 = 10;
    }
    if ((v3 & 2) && (v5 <= v8)) {
      v4 = v5;
      do {
        sub_f510(1);
        for (; a0; a0 = (unsigned long *)*a0) {
          if (v4 < (long)(a0[1] + a0[4])) {
            if ((long)a0[4] <= v4) {
              v10 = "+";
              if (1 <= (long)a0[2])
                v10 = "!";
              goto label_839d;
            }
            break;
          }
        }
        v10 = " ";
label_839d:
        sub_f330(v10,dat_26388 + v4 * 8,1);
        sub_f510(3);
        if (*(char *)(*(long *)(dat_26388 + 8 + v4 * 8) + -1) == '\n') {
          v1 = *(char **)((long)v9 + 0x28);
          if (*(char **)((long)v9 + 0x30) <= v1) // branch-flip
            __overflow(v9,10);
          else {
            *(char **)((long)v9 + 0x28) = &v1[1];
            *v1 = 10;
          }
        }
        v4 += 1;
      } while (v4 <= v8);
    }
  }
}

// Function: sub_84f0 @ 0x84f0
void sub_84f0(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  char *v1; // rdx
  long v2; // stack - 0x20
  long v3; // stack - 0x18
  
  sub_f5e0(a0,a1,a2,&v2,&v3);
  if (v3 <= v2) {
    v1 = "%ld,0";
    if (v2 <= v3)
      v1 = "%ld";
    __fprintf_chk(dat_26188,1,v1);
    return;
  }
  __fprintf_chk(dat_26188,1,"%ld,%ld",v2,(v3 - v2) + 1);
}

// Function: sub_8590 @ 0x8590
void sub_8590(unsigned long *a0) // return-dupe
{
  long v1;
  long v10; // stack - 0x48
  unsigned long *v11;
  int v12; // esi
  long v13;
  void *v14;
  long v15; // stack - 0x78
  char *v2;
  bool v3;
  long v4;
  long v5;
  long v6;
  long v7; // stack - 0x50
  long v8; // stack - 0x58
  long v9; // stack - 0x60
  
  if (!sub_f670(a0,&v9,&v8,&v7,&v10))
    return;
  v9 -= dat_26570;
  v5 = -dat_26288;
  if (v9 < v5)
    v9 = v5;
  v7 -= dat_26570;
  if (v7 < v5)
    v7 = v5;
  v5 = dat_26570 + v8;
  v3 = v8 < dat_26270 - dat_26570;
  v8 = dat_26270 + -1;
  if (v3)
    v8 = v5;
  v5 = dat_26570 + v10;
  v3 = v10 < dat_263a0 - dat_26570;
  v10 = dat_263a0 + -1;
  if (v3)
    v10 = v5;
  if (dat_26520) { // branch-flip
    v5 = sub_7e50(dat_26258);
    sub_e830();
    v14 = dat_26188;
    sub_f510(4);
    fputs_unlocked("@@ -",v14);
    sub_84f0(0x261a0,v9,v8);
    fputs_unlocked(" +",v14);
    sub_84f0(0x262d0,v7,v10);
    fputs_unlocked(" @@",v14);
    sub_f510(3);
    if (v5)
      sub_7f00(v14,v5);
  }
  else {
    sub_e830();
    v14 = dat_26188;
    sub_f510(4);
    fputs_unlocked("@@ -",v14);
    sub_84f0(0x261a0,v9,v8);
    fputs_unlocked(" +",v14);
    sub_84f0(0x262d0,v7,v10);
    fputs_unlocked(" @@",v14);
    sub_f510(3);
  }
  v2 = *(char **)((long)v14 + 0x28);
  if (*(char **)((long)v14 + 0x30) <= v2) // branch-flip
    __overflow(v14,10);
  else {
    *(char **)((long)v14 + 0x28) = &v2[1];
    *v2 = 10;
  }
  v15 = v7;
  v5 = v9;
  do {
    if ((v8 < v5) && (v10 < v15))
      return;
    while ((!a0 || (v5 < (long)a0[3]))) {
      v1 = v5 + 1;
      v11 = (unsigned long *)(dat_26258 + v5 * 8);
      if ((!dat_26491) || (*(char *)*v11 != '\n')) {
        v2 = *(char **)((long)v14 + 0x28);
        v12 = (-(unsigned int)(dat_26492 == '\0') & 0x17) + 9;
        if (*(char **)((long)v14 + 0x30) <= v2) // branch-flip
          __overflow(v14,v12);
        else {
          *(char **)((long)v14 + 0x28) = &v2[1];
          *v2 = (char)v12;
        }
      }
      sub_f500(0,v11);
      v15 += 1;
      v5 = v1;
      if ((v8 < v1) && (v10 < v15))
        return;
    }
    v1 = a0[2];
    if (v1) {
      v13 = v5 * 8;
      v6 = v1;
      do {
        while( true ) {
          v4 = dat_26258;
          sub_f510(2);
          v2 = *(char **)((long)v14 + 0x28);
          v11 = (unsigned long *)(v4 + v13);
          if (*(char **)((long)v14 + 0x30) <= v2) // branch-flip
            __overflow(v14,0x2d);
          else {
            *(char **)((long)v14 + 0x28) = &v2[1];
            *v2 = 0x2d;
          }
          if ((dat_26492) && ((!dat_26491 || (*(char *)*v11 != '\n')))) {
            v2 = *(char **)((long)v14 + 0x28);
            if (*(char **)((long)v14 + 0x30) <= v2) // branch-flip
              __overflow(v14,9);
            else {
              *(char **)((long)v14 + 0x28) = &v2[1];
              *v2 = 9;
            }
          }
          sub_f330(0,v11,1);
          sub_f510(3);
          if (*(char *)(v11[1] + -1) == '\n') break;
label_8880:
          v13 += 8;
          v6 -= 1;
          if (!v6) goto label_893d;
        }
        v2 = *(char **)((long)v14 + 0x28);
        if (*(char **)((long)v14 + 0x30) <= v2) {
          __overflow(v14,10);
          goto label_8880;
        }
        v13 += 8;
        *(char **)((long)v14 + 0x28) = &v2[1];
        *v2 = 10;
        v6 -= 1;
      } while (v6);
label_893d:
      v5 += v1;
    }
    v1 = a0[1];
    if (v1) {
      v13 = v15 * 8;
      v6 = v1;
      do {
        while( true ) {
          v4 = dat_26388;
          sub_f510(1);
          v2 = *(char **)((long)v14 + 0x28);
          v11 = (unsigned long *)(v4 + v13);
          if (*(char **)((long)v14 + 0x30) <= v2) // branch-flip
            __overflow(v14,0x2b);
          else {
            *(char **)((long)v14 + 0x28) = &v2[1];
            *v2 = 0x2b;
          }
          if ((dat_26492) && ((!dat_26491 || (*(char *)*v11 != '\n')))) {
            v2 = *(char **)((long)v14 + 0x28);
            if (*(char **)((long)v14 + 0x30) <= v2) // branch-flip
              __overflow(v14,9);
            else {
              *(char **)((long)v14 + 0x28) = &v2[1];
              *v2 = 9;
            }
          }
          sub_f330(0,v11,1);
          sub_f510(3);
          if (*(char *)(v11[1] + -1) == '\n') break;
label_8970:
          v13 += 8;
          v6 -= 1;
          if (!v6) goto label_8a2d;
        }
        v2 = *(char **)((long)v14 + 0x28);
        if (*(char **)((long)v14 + 0x30) <= v2) {
          __overflow(v14,10);
          goto label_8970;
        }
        v13 += 8;
        *(char **)((long)v14 + 0x28) = &v2[1];
        *v2 = 10;
        v6 -= 1;
      } while (v6);
label_8a2d:
      v15 += v1;
    }
    a0 = (unsigned long *)*a0;
  } while( true );
}

// Function: sub_8b10 @ 0x8b10
long * sub_8b10(long *a0)
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

// Function: sub_8b80 @ 0x8b80
void sub_8b80(long a0,unsigned long *a1,char a2)
{
  if (a2) {
    sub_7cf0("---",a0,*a1,dat_26540);
    sub_7cf0("+++",a0 + 0x130,a1[1],dat_26548); // tail-call
    return;
  }
  sub_7cf0("***",a0,*a1,dat_26540);
  sub_7cf0("---",a0 + 0x130,a1[1],dat_26548); // tail-call
}

// Function: sub_8c10 @ 0x8c10
void sub_8c10(unsigned long *a0,char a1) // ternary
{
  unsigned long *v1;
  int v2; // eax
  void *v3; // rdx
  unsigned long *v4;
  char v5 [8];
  char v6 [8];
  char v7 [8];
  char v8 [8];
  
  if ((dat_26554) || (v4 = a0, dat_264e0)) {
    if (a0) {
      v4 = a0;
      do {
        v1 = (unsigned long *)*v4;
        *v4 = 0;
        v2 = sub_f670(v4,v8,v7,v6,v5);
        *v4 = v1;
        *(bool *)&v4[5] = v2 == 0;
        v4 = v1;
      } while (v1);
    }
  }
  else {
    for (; v4; v4 = (unsigned long *)*v4) {
      *(char *)&v4[5] = 0;
      v4 = (unsigned long *)*v4;
      if (!v4) break;
      *(char *)&v4[5] = 0;
    }
  }
  dat_26178 = -dat_26288;
  dat_26170 = 0x7fffffffffffffff;
  v3 = (a1) ? sub_8590 : sub_8070; // branch-flip
  sub_f080(a0,sub_8b10,v3); // tail-call
}

// Function: sub_8d60 @ 0x8d60
void sub_8d60(long *a0,char *a1)
{
  unsigned long v1; // rax
  long v2;
  long v3;
  unsigned long v4; // rdx
  unsigned long v5;
  long v6;
  
  v1 = strlen(a1);
  v2 = re_compile_pattern(a1,v1,a0[4]);
  if (v2) {
    error(2,0,"%s: %s",a1,v2); // tail-call
    return;
  }
  v2 = *a0;
  v6 = a0[1];
  v5 = a0[2];
  *(bool *)&a0[3] = v2 != 0;
  v4 = (unsigned long)((unsigned int)(v2 != 0) * 2) + v6 + v1;
  a0[1] = v4;
  v3 = v2;
  if (v5 <= v4) {
    if (!v5)
      v5 = 1;
    do {
      v5 *= 2;
    } while (v5 <= v4);
    a0[2] = v5;
    v3 = sub_16a90(v2);
    *a0 = v3;
  }
  if (v2) {
    v2 = v6 + 1;
    *(char *)(v3 + v6) = 0x5c;
    v6 += 2;
    *(char *)(v3 + v2) = 0x7c;
  }
  memcpy((void *)(v3 + v6),a1,v1 + 1); // tail-call
}

// Function: sub_8e40 @ 0x8e40
void sub_8e40(char *a0,unsigned long a1)
{
  unsigned long v1;
  
  if (a0)
    error(0,0,dcgettext(NULL,a0,5),a1);
  v1 = dat_36860;
  error(2,0,dcgettext(NULL,"Try \'%s --help\' for more information.",5),v1);
}

// Function: sub_8ea0 @ 0x8ea0
long sub_8ea0(long *a0,unsigned long a1,unsigned long a2)
{
  long v1;
  long v2;
  long v3; // rax
  
  if (!*a0)
    return v3;
  v1 = a0[4];
  v3 = sub_16a40(0x100);
  v2 = a0[3];
  *(long *)(v1 + 0x20) = v3;
  if ((char)v2) {
    v3 = re_compile_pattern(*a0,a0[1],a0[4]);
    if (v3) {
      error(2,0,"%s: %s",*a0,v3);
      return v3;
    }
  }
  v3 = a2;
  return v3;
}

// Function: sub_8f00 @ 0x8f00
void sub_8f00(unsigned long *a0,char *a1,unsigned long a2)
{
  char *v1;
  
  v1 = (char *)*a0;
  if (v1) {
    if (strcmp(v1,a1)) {
      error(0,0,dcgettext(NULL,"conflicting %s option value \'%s\'",5),a2,a1);
      sub_8e40(0,0); // no-return
    }
  }
  *a0 = a1;
}

// Function: sub_8f60 @ 0x8f60
void sub_8f60(int a0) // return-dupe
{
  if (dat_26580 == a0)
    return;
  if (!dat_26580) {
    dat_26580 = a0;
    return;
  }
  sub_8e40("conflicting output style options",0); // no-return
}

// Function: sub_8f90 @ 0x8f90
unsigned int sub_8f90(long a0,char *a1,char *a2)
{
  long *v1;
  int v10 [2]; // stack - 0x2a8
  long v11; // stack - 0x240
  long v12 [3]; // stack - 0x298
  long v13 [3]; // stack - 0x168
  int *v14;
  char *v15;
  long *v16;
  unsigned int v17; // r9d
  char *v18;
  int v19; // ebx
  unsigned long v2;
  bool v20;
  bool v21;
  char *v22; // stack - 0x2c0
  char *v23; // stack - 0x2a0
  unsigned int v24; // stack - 0x280
  int v25; // stack - 0x27c
  unsigned int v26; // stack - 0x278
  long v27; // stack - 0x270
  long v28; // stack - 0x268
  unsigned long v29; // stack - 0x260
  unsigned long v3;
  unsigned long v30; // stack - 0x258
  unsigned long v31; // stack - 0x250
  unsigned long v32; // stack - 0x248
  unsigned long v33; // stack - 0x238
  long v34; // stack - 0x230
  unsigned long v35; // stack - 0x228
  unsigned long v36; // stack - 0x220
  unsigned long v37; // stack - 0x218
  unsigned long v38; // stack - 0x210
  int v39; // stack - 0x178
  unsigned char v4; // al
  char *v40; // stack - 0x170
  unsigned long v41; // stack - 0x150
  unsigned long v42; // stack - 0x148
  long v43; // stack - 0x140
  long v44; // stack - 0x138
  unsigned long v45; // stack - 0x130
  unsigned long v46; // stack - 0x128
  unsigned long v47; // stack - 0x120
  unsigned long v48; // stack - 0x118
  long v49; // stack - 0x110
  unsigned int v5; // eax
  unsigned long v50; // stack - 0x108
  long v51; // stack - 0x100
  unsigned long v52; // stack - 0xf8
  unsigned long v53; // stack - 0xf0
  unsigned long v54; // stack - 0xe8
  unsigned long v55; // stack - 0xe0
  long v56; // stack - 0x48
  int *v6;
  long v7; // rcx
  int v8;
  char *v9;
  
  if ((!a1) || (!a2)) {
    if ((dat_26585 == '\x01') && (a2)) {
      v56 = a0;
      v7 = 0x4c;
      v6 = v10;
      while (v7) {
        v14 = &v6[2];
        v6[0] = 0;
        v6[1] = 0;
        v7 -= 1;
        v6 = v14;
      }
      if (a1) { // branch-flip
        v10[0] = -2;
        v39 = 0xfffffffe;
      }
      else {
        v10[0] = -1;
label_9698:
        v10[0] = -1;
        v39 = -2;
        if (!a2) goto label_9688;
      }
      v39 = -2;
      a1 = a2;
      goto label_9070;
    }
    if (!dat_26586) {
      v18 = a1;
      if (!a1)
        v18 = a2;
      v6 = (unsigned long)(unsigned long)(a1 == NULL);
      v6 = (long)((long)v6 * 0x130);
      sub_e660("Only in %s: %s\n",*(unsigned long *)(a0 + 8 + (long)v6),v18);
      v5 = 1;
      return v5;
    }
    v56 = a0;
    v7 = 0x4c;
    v6 = v10;
    while (v7) {
      v14 = &v6[2];
      v6[0] = 0;
      v6[1] = 0;
      v7 -= 1;
      v6 = v14;
    }
    if (!a1) {
      v10[0] = -1;
      if (a2) goto label_9698;
      v39 = -1;
label_9688:
      v10[0] = -1;
      a1 = NULL;
      goto label_9070;
    }
    v10[0] = -2;
    if (!a2) {
      v39 = -1;
      a2 = a1;
      goto label_9070;
    }
  }
  else {
    v56 = a0;
    v7 = 0x4c;
    v6 = v10;
    while (v7) {
      v14 = &v6[2];
      v6[0] = 0;
      v6[1] = 0;
      v7 -= 1;
      v6 = v14;
    }
    v10[0] = -2;
  }
  v10[0] = -2;
  v39 = -2;
label_9070:
  v56 = a0;
  if (a0) { // branch-flip
    v6 = (char *)sub_10d80(*(unsigned long *)(a0 + 8),a1,0);
    v18 = (char *)v6;
    v23 = (char *)v6;
    v6 = (char *)sub_10d80(*(unsigned long *)(a0 + 0x138),a2,0);
    v22 = (char *)v6;
  }
  else {
    v23 = a1;
    v18 = NULL;
    v22 = NULL;
    v6 = (int *)a2;
  }
  v40 = (char *)v6;
  v16 = &v11;
  v20 = 0;
  do {
    if ((int)v16[-0xd] != -1) {
      if ((!v20) || (v5 = strcmp(v40,v23), v5)) {
        v9 = (char *)v16[-0xc];
        v5 = strcmp(v9,"-");
        v1 = &v16[-0xb];
        if (v5) { // branch-flip
          if (dat_26550) // branch-flip
            v5 = lstat(v9,v1);
          else {
            v5 = stat(v9,v1);
          }
          if (v5) {
label_9390:
            v6 = __errno_location();
            v5 = 0xfffffffd - *v6;
            *(unsigned int *)&v16[-0xd] = v5;
          }
        }
        else {
          *(unsigned int *)&v16[-0xd] = 0;
          isatty(0);
          v5 = fstat(0,v1);
          if (v5) goto label_9390;
          v5 = *(unsigned int *)&v16[-8] & 0xf000;
          if (v5 == 0x8000) {
            v6 = (long)lseek(0,0,1);
            if (0 <= (long)v6) { // branch-flip
              v16[-5] = v16[-5] - (long)v6;
              v6 = (int *)v16[-5];
              if (v16[-5] <= -1)
                v6 = (int *)0;
              v16[-5] = (long)v6;
            }
            else {
              v6 = __errno_location();
              v5 = 0xfffffffd - *v6;
              *(unsigned int *)&v16[-0xd] = v5;
            }
          }
          sub_10eb0(v16);
        }
      }
      else {
        v41 = CONCAT44(v25,v24);
        v42 = (unsigned long)v26;
        v13[0] = v12[0];
        v13[1] = v12[1];
        v39 = v10[0];
        v13[2] = v12[2];
        v43 = v27;
        v44 = v28;
        v45 = v29;
        v46 = v30;
        v47 = v31;
        v48 = v32;
        v49 = v11;
        v50 = v33;
        v51 = v34;
        v52 = v35;
        v53 = v36;
        v54 = v37;
        v55 = v38;
      }
    }
    v16 = &v16[0x26];
    if (v20) goto label_9188;
    v20 = 1;
  } while( true );
label_9188:
  v19 = 0;
  v6 = v10;
  v14 = v10;
label_91a0:
  v19 += 1;
  if (!dat_26586) {
    if (v19 != 1) goto label_91b7;
    if (dat_26585) goto label_92e0;
    goto label_932a;
  }
label_92e0:
  v8 = *v6;
  if (v8 != -2) { // branch-flip
    if (((v8 == -5 || v8 == -0xc) && (!a0)) && (!(v14[0x4c] + 2U & 0xfffffffd))) {
label_931a:
      *v6 = -1;
      if (v19 != 2) goto label_932a;
label_91c1:
      if (v10[0] != -1) {
        v8 = 0xfffffffd - v10[0];
        if (v39 != -1) { // branch-flip
          if (v8 < 0) goto label_93d7;
        }
        else {
          v7 = 0x12;
          v16 = v13;
          while (v7) {
            v1 = &v16[1];
            *v16 = 0;
            v7 -= 1;
            v16 = v1;
          }
          v41 = CONCAT44(v41._4_4_,v24);
          if (v8 <= -1) goto label_93e5;
        }
        v6 = __errno_location();
        *v6 = v8;
        sub_e490(v23);
        v8 = 0xfffffffd - v39;
        if (0 <= v8) {
label_9219:
          *v6 = v8;
          sub_e490(v40);
        }
        goto label_9228;
      }
      v7 = 0x12;
      v16 = v12;
      while (v7) {
        v1 = &v16[1];
        *v16 = 0;
        v7 -= 1;
        v16 = v1;
      }
      v24 = (unsigned int)v41;
      if (v39 != -1) { // branch-flip
label_93d7:
        v5 = 0xfffffffd - v39;
        v8 = v5;
        if (0 <= (int)v5) {
          v6 = __errno_location();
          goto label_9219;
        }
      }
      else {
        v7 = 0x12;
        v16 = v13;
        while (v7) {
          v1 = &v16[1];
          *v16 = 0;
          v7 -= 1;
          v16 = v1;
        }
      }
label_93e5:
      v8 = v10[0];
      if (!a0) {
        v5 = v24 & 0xf000;
        v20 = v5 == 0x4000;
        if (v20 != (((unsigned int)v41 & 0xf000) == 0x4000)) {
          v6 = (unsigned long)(unsigned long)v20;
          v2 = (unsigned long)v6;
          v5 = (unsigned int)!v20;
          v7 = (long)(int)v5;
          v9 = (char *)v12[v2 * 0x26 + -1];
          v6 = (unsigned long)sub_fb40(v9);
          v6 = (char *)sub_a660(v12[v7 * 0x26 + -1],v6);
          v18 = (char *)v6;
          v12[v7 * 0x26 + -1] = (long)v6;
          v5 = strcmp(v9,"-");
          if (!v5)
            sub_e7b0("cannot compare \'-\' to a directory"); // no-return
          if (dat_26550) // branch-flip
            v5 = lstat(v18,&v12[v7 * 0x26]);
          else {
            v5 = stat(v18,&v12[v7 * 0x26]);
          }
          v4 = v5 != 0;
          v8 = v10[0];
          if ((bool)v4) {
            sub_e490(v18);
            goto label_9228;
          }
        }
      }
      if (v8 != -1) { // branch-flip
        if (v39 != -1) { // branch-flip
          if ((v12[1] == v13[1]) && (v12[0] == v13[0])) {
label_984b:
            if (v24 == (unsigned int)v41) {
              if (((((v12[2] != v13[2]) || (v25 != v41._4_4_)) || (v26 != (unsigned int)v42)) || ((v28 != v44 || (v11 != v49)))) || (v34 != v51)) {
                v20 = 0;
                v5 = (unsigned int)v41 & 0xf000;
              }
              else {
                if (dat_26578) goto label_9730;
                v20 = 1;
                v5 = (unsigned int)v41 & 0xf000;
              }
              goto label_96ca;
            }
            goto label_96c0;
          }
          v5 = v24 & 0xf000;
          v17 = (unsigned int)v41 & 0xf000;
          if (v5 != 0x6000) { // branch-flip
            v20 = 0;
            if (v5 != 0x2000) goto label_96ca;
            if (v17 == 0x2000) goto label_990d;
          }
          else if (v17 == 0x6000) {
label_990d:
            if (v43 == v27) goto label_984b;
          }
          if ((v5 == 0x4000) || (v20 = 0, v17 == 0x4000)) {
label_9940:
            if (v39 == -1) goto label_9949;
label_98a0:
            v19 = 1;
            v6 = (unsigned long)sub_10c90(v13);
            v3 = v6;
            v9 = dat_26548;
            if (!dat_26548)
              v9 = v40;
            v6 = (unsigned long)sub_10c90(v12);
            v15 = dat_26540;
            if (!dat_26540)
              v15 = v23;
            sub_e4c0("File %s is a %s while file %s is a %s\n",v15,v6,v9,v3);
            goto label_9228;
          }
label_955c:
          v17 = (unsigned int)v41 & 0xf000;
          if ((a0) && (((v5 & 0xffffdfff) != 0x8000 || (((unsigned int)v41 & 0xd000) != 0x8000)))) {
            if ((v8 == -1) || (v39 == -1)) goto label_9598;
            goto label_98a0;
          }
          if ((v5 == 0xa000) || (v17 == 0xa000)) {
            if (!dat_26550)
              __assert_fail("no_dereference_symlinks","diff.c",0x549,"compare_files"); // no-return
            if ((v5 != 0xa000) || (v17 != 0xa000)) goto label_98a0;
            v6 = (char *)sub_16f20(v23);
            v9 = (char *)v6;
            if (v6) { // branch-flip
              v6 = (char *)sub_16f20(v40);
              v15 = (char *)v6;
              v8 = 1;
              if (v6) {
                v5 = strcmp(v9,(char *)v6);
                if (!v5) {
                  free(v9);
                  free(v15);
                  goto label_9730;
                }
                v19 = 1;
                sub_e660("Symbolic links %s and %s differ\n",v23,v40);
                free(v9);
                free(v15);
                goto label_9228;
              }
            }
            else {
              v8 = 0;
            }
            v6 = (long)(long)v8;
            sub_e490(v12[(long)v6 * 0x26 + -1]);
            free(v9);
            free(NULL);
            goto label_9228;
          }
          v4 = v5 == 0x8000 & dat_26553;
          if ((((v4) && (v17 == 0x8000)) && (v28 != v44 && 1 <= v28)) && (1 <= v44)) {
            v9 = dat_26548;
            if (!dat_26548)
              v9 = v40;
            v15 = dat_26540;
            if (!dat_26540)
              v15 = v23;
            v19 = 1;
            sub_e660("Files %s and %s differ\n",v15,v9);
            goto label_9228;
          }
          if (v8 == -2) {
            v5 = open(v23,0,0);
            v10[0] = v5;
            if (0 <= (int)v5) goto label_9a07;
            sub_e490(v23);
            if (v39 == -2) {
              v21 = 1;
              goto label_9a17;
            }
label_9a32:
            if (0 <= v10[0]) {
              v8 = 2;
              goto label_9b75;
            }
            if (v39 < 0) goto label_9228;
label_9a4d:
            v5 = close(v39);
            if (v5) {
label_9a5a:
              sub_e490(v40);
            }
            goto label_9228;
          }
label_9a07:
          v21 = 0;
          if (v39 == -2) {
label_9a17:
            if (v20)
              v39 = v10[0];
            else {
              v5 = open(v40,0,0);
              v39 = v5;
              if ((int)v5 <= -1) {
                sub_e490(v40);
                goto label_9a32;
              }
            }
            if (v21) goto label_9a32;
          }
          v5 = sub_6e20(v10);
          v8 = v5;
          if (0 <= v10[0]) {
label_9b75:
            v5 = close(v10[0]);
            if (v5) {
              sub_e490(v23);
              if ((v39 <= -1) || (v10[0] == v39)) goto label_9228;
              goto label_9a4d;
            }
          }
          v5 = v8;
          if ((0 <= v39) && (v39 != v10[0])) {
            v5 = close(v39);
            v5 = v8;
            if (v5) goto label_9a5a;
          }
label_9728:
          v19 = v5;
          if (!v19) goto label_9730;
        }
        else {
label_96c0:
          v20 = 0;
          v5 = v24 & 0xf000;
label_96ca:
          v21 = ((unsigned int)v41 & 0xf000) != 0x4000;
          if (v5 == 0x4000) {
            if (v21) goto label_9931;
            if (dat_26580 == 7)
              sub_e7b0("-D option not supported with directories"); // no-return
            if ((a0) && (!dat_26608)) {
              sub_e660("Common subdirectories: %s and %s\n",v23,v40);
              goto label_9730;
            }
label_9717:
            v5 = sub_a230(v10,sub_8f90);
            goto label_9728;
          }
          if (v21) goto label_955c;
label_9931:
          if (v8 != -1) goto label_9940;
label_9949:
          if ((dat_26608) && ((dat_26586 || ((dat_26585 && (v8 == -1)))))) goto label_9717;
          if (!a0)
            __assert_fail("parent","diff.c",0x52d,"compare_files"); // no-return
label_9598:
          v19 = 1;
          v6 = (unsigned long)(unsigned long)(v8 == -1);
          v6 = (long)((long)v6 * 0x130);
          sub_e660("Only in %s: %s\n",*(unsigned long *)(a0 + 8 + (long)v6),a1);
        }
label_9228:
        v5 = fflush_unlocked(stdout);
        if (v5) {
          v6 = (char *)dcgettext(NULL,"standard output",5);
          sub_e770(v6); // no-return
        }
      }
      else {
        if (v39 != -1) {
          v20 = 0;
          v5 = v24 & 0xf000;
          goto label_96ca;
        }
label_9730:
        v19 = 0;
        if ((dat_26584) && (v5 = v24 & 0xf000, v5 != 0x4000)) {
          v9 = dat_26548;
          if (!dat_26548)
            v9 = v40;
          v15 = dat_26540;
          if (!dat_26540)
            v15 = v23;
          sub_e660("Files %s and %s are identical\n",v15,v9);
        }
      }
      free(v18);
      free(v22);
      v5 = v19;
      return v5;
    }
  }
  else if ((((v6[10] & 0xf000U) == 0x8000) && (!(v6[10] & 0x1ffU))) && (!*(long *)&v6[0x10])) goto label_931a;
label_91b7:
  if (v19 == 2) goto label_91c1;
label_932a:
  v6 = &v6[0x4c];
  v14 = &v14[-0x4c];
  goto label_91a0;
}

// Function: sub_9df0 @ 0x9df0
void sub_9df0(void)
{
  int v1; // eax
  
  if (*stdout & 0x20)
    sub_e7b0("write failed"); // no-return
  v1 = fclose(stdout);
  if (!v1)
    return;
  sub_e770(dcgettext(NULL,"standard output",5)); // no-return
}

// Function: sub_9e40 @ 0x9e40
unsigned long sub_9e40(long a0,int a1) // return-dupe x2
{
  long v1;
  long v2; // rcx
  unsigned int v3; // edx
  
  v1 = *(long *)(a0 + 0x260);
  if (!v1)
    return 0;
  v2 = (long)a1 * 0x130;
  a0 += v2;
  do {
    if ((*(long *)(v1 + v2 + 0x18) == *(long *)(a0 + 0x18)) && (*(long *)(v1 + v2 + 0x10) == *(long *)(a0 + 0x10)))
      return 1;
    v3 = *(unsigned int *)(v1 + 0x28 + v2) & 0xf000;
    if (v3 != 0x6000) { // branch-flip
      if ((v3 != 0x2000) || ((*(unsigned int *)(a0 + 0x28) & 0xf000) != 0x2000)) goto label_9e8a;
      if (*(long *)(v1 + 0x38 + v2) == *(long *)(a0 + 0x38))
        return 1;
label_9ecc:
      v1 = *(long *)(v1 + 0x260);
    }
    else {
      if ((*(unsigned int *)(a0 + 0x28) & 0xf000) == 0x6000) {
        if (*(long *)(v1 + 0x38 + v2) == *(long *)(a0 + 0x38))
          return 1;
        goto label_9ecc;
      }
label_9e8a:
      v1 = *(long *)(v1 + 0x260);
    }
    if (!v1)
      return 0;
  } while( true );
}

// Function: sub_9f10 @ 0x9f10
void sub_9f10(char *a0,char *a1)
{
  int *v1; // rax
  
  if (dat_266e8) {
    v1 = __errno_location();
    *v1 = 0;
    if (dat_26551) {
      strcasecmp(a0,a1); // tail-call
      return;
    }
    if (strcoll(a0,a1))
      return;
  }
  strcmp(a0,a1); // tail-call
}

// Function: sub_9f90 @ 0x9f90
unsigned long sub_9f90(int *a0,unsigned long *a1) // return-dupe x2
{
  char *v1;
  unsigned long v10; // rbx
  unsigned long *v11; // rbp
  unsigned long v12;
  unsigned long v13; // stack - 0x48
  unsigned long v14; // stack - 0x40
  int v2;
  char v3; // al
  void *v4; // rax
  char *v5;
  int *v6; // rax
  void *v7; // rax
  unsigned long v8; // rax
  unsigned long *v9; // rax
  
  v2 = *a0;
  a1[1] = 0;
  a1[2] = 0;
  if (v2 != -1) { // branch-flip
    v4 = opendir(*(char **)&a0[2]);
    if (!v4)
      return 0;
    v12 = 0x200;
    v5 = (char *)sub_16a40(0x200);
    a1[2] = (unsigned long)v5;
    v6 = __errno_location();
    v13 = 0;
    v14 = 0;
label_a010:
    *v6 = 0;
    v7 = readdir(v4);
    if (v7) {
      v1 = (char *)((long)v7 + 0x13);
      v8 = strlen(v1);
      if (*(char *)((long)v7 + 0x13) == '.') goto label_a048;
      goto label_a058;
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
      sub_16ec0(); // no-return, return-dupe
    v9 = (unsigned long *)sub_16a40(v14 * 8 + 8);
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
    v9 = (unsigned long *)sub_16a40(8);
    *a1 = 0;
    a1[1] = (unsigned long)v9;
  }
  *v9 = 0;
  return 1;
label_a048:
  if ((*(char *)((long)v7 + 0x14)) && ((*(char *)((long)v7 + 0x14) != '.' || (*(char *)((long)v7 + 0x15))))) {
label_a058:
    v3 = sub_104d0(dat_26410,v1);
    if (!v3) {
      v10 = v13 + v8 + 1;
      while (v12 < v10) {
        if (0x3ffffffffffffffe < v12) {
          sub_16ec0();
        }
        v12 *= 2;
        v5 = (char *)sub_16a90(v5,v12);
        a1[2] = (unsigned long)v5;
      }
      memcpy(&v5[v13],v1,v8 + 1);
      v14 += 1;
      v13 = v10;
    }
  }
  goto label_a010;
}

// Function: sub_a1c0 @ 0xa1c0
void sub_a1c0(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  int v3; // eax
  int *v4; // rax
  
  v1 = (char *)*a0;
  v2 = (char *)*a1;
  if (dat_266e8) {
    v4 = __errno_location();
    *v4 = 0;
    if (dat_26551) // branch-flip
      v3 = strcasecmp(v1,v2);
    else {
      v3 = strcoll(v1,v2);
    }
    if (v3)
      return;
  }
  strcmp(v1,v2); // tail-call
}

// Function: sub_a230 @ 0xa230
int sub_a230(int *a0,void *a1) // early-return
{
  unsigned long v1;
  long *v10;
  int v11; // stack - 0x8c
  void *v12; // stack - 0x68
  long *v13; // stack - 0x58
  void *v14; // stack - 0x50
  long *v2;
  char *v3;
  int v4;
  long v5;
  long *v6 [4]; // stack - 0x88
  unsigned long v7; // stack - 0x60
  char *v8;
  long v9;
  
  v4 = *a0;
  v11 = 0;
  if (((v4 == -1) || (sub_9e40(a0,0))) && ((a0[0x4c] == -1 || (sub_9e40(a0,1))))) {
    v1 = *(unsigned long *)&a0[(unsigned long)(v4 == -1) * 0x4c + 2];
    error(0,0,dcgettext(NULL,"%s: recursive directory loop",5),v1);
    return 2;
  }
  if (!sub_9f90(a0,&v6[2])) {
    sub_e490(*(unsigned long *)&a0[2]);
    v11 = 2;
  }
  if (sub_9f90(&a0[0x4c],&v7)) { // branch-flip
    if (!v11) {
      dat_266e8 = 1;
      v6[0] = v6[3];
      v6[1] = v13;
      if (_setjmp(0x26620))
        dat_266e8 = 0;
      qsort(v6[0],v6[2],8,sub_a1c0);
      qsort(v6[1],v7,8,sub_a1c0);
      v9 = dat_26488;
      if ((dat_26488) && (!*(long *)&a0[0x98])) {
        while ((*v6[0] && (v5 = *v6[0], (int)sub_9f10(v5,v9) <= -1))) {
          v6[0] = &v6[0][1];
        }
        while ((*v6[1] && (v5 = *v6[1], (int)sub_9f10(v5,v9) <= -1))) {
          v6[1] = &v6[1][1];
        }
      }
      while ((*v6[0] || (*v6[1]))) {
        if (*v6[0]) { // branch-flip
          if (*v6[1]) { // branch-flip
            v4 = sub_9f10(*v6[0],*v6[1]);
            if (v4) {
              if (0 <= v4) {
                v2 = &v6[1][1];
                v5 = *v6[1];
                v9 = 0;
                v6[1] = v2;
                goto label_a465;
              }
              goto label_a4ab;
            }
            if ((dat_26551) && (v4 = strcmp((char *)*v6[0],(char *)*v6[1]), v4)) {
              v2 = v6[(v4 >> 0x1f) + 1];
              v8 = (char *)v2[1];
              v3 = (char *)*v6[(unsigned int)-(v4 >> 0x1f)];
              v10 = &v2[1];
              if (v8) {
                while (!sub_9f10(v8,v3)) {
                  v4 = strcmp(v8,v3);
                  if (0 <= v4) {
                    if (!v4) {
                      memmove(&v2[1],v2,(long)v10 - (long)v2);
                      *v2 = (long)v3;
                    }
                    break;
                  }
                  v8 = (char *)v10[1];
                  v10 = &v10[1];
                  if (!v8) break;
                }
              }
            }
            v2 = &v6[1][1];
            v5 = *v6[1];
            v6[1] = v2;
          }
          else {
label_a4ab:
            v5 = 0;
          }
          v2 = &v6[0][1];
          v9 = *v6[0];
          v6[0] = v2;
        }
        else {
          v2 = &v6[1][1];
          v5 = *v6[1];
          v9 = 0;
          v6[1] = v2;
        }
label_a465:
        v4 = (*a1)(a0,v9,v5);
        if (v11 < v4)
          v11 = v4;
      }
    }
  }
  else {
    sub_e490(*(unsigned long *)&a0[0x4e]);
    v11 = 2;
  }
  free(v6[3]);
  free(v12);
  free(v13);
  free(v14);
  return v11;
}

// Function: sub_a660 @ 0xa660
unsigned long sub_a660(unsigned long a0,char *a1)
{
  unsigned long *v1;
  void *v10; // stack - 0x168
  unsigned long v11; // stack - 0x150
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
    if (sub_9f90(v6,v5)) {
      dat_266e8 = 1;
      if (_setjmp(0x26620)) // branch-flip
        v8 = a1;
      else {
        v2 = (char *)*v9;
        v3 = a1;
        v7 = v9;
        while (v8 = v3, v2) {
          if (!sub_9f10(v2,a1)) {
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
  v4 = sub_10d80(a0,v8,0);
  free(v9);
  free(v10);
  return v4;
}

// Function: sub_a7c0 @ 0xa7c0
void sub_a7c0(unsigned long a0) // return-dupe x2
{
  long v1;
  unsigned long v10; // stack - 0x50
  long v11; // stack - 0x38
  unsigned char v2;
  unsigned char *v3;
  char *v4;
  char *v5;
  unsigned int v6; // eax
  long v7;
  long v8; // stack - 0x40
  unsigned long v9; // stack - 0x48
  
  v6 = sub_f670(a0,&v10,&v9,&v8,&v11);
  if (!v6)
    return;
  sub_e830();
  sub_f610(0x2c,0x261a0,v10,v9);
  v2 = *(unsigned char *)((unsigned long)v6 + 0x1e550);
  v3 = *(unsigned char **)((long)dat_26188 + 0x28);
  if (*(unsigned char **)((long)dat_26188 + 0x30) <= v3) // branch-flip
    __overflow(dat_26188,(unsigned int)v2);
  else {
    *(unsigned char **)((long)dat_26188 + 0x28) = &v3[1];
    *v3 = v2;
  }
  v4 = *(char **)((long)dat_26188 + 0x28);
  if (*(char **)((long)dat_26188 + 0x30) <= v4) // branch-flip
    __overflow(dat_26188,10);
  else {
    *(char **)((long)dat_26188 + 0x28) = &v4[1];
    *v4 = 10;
  }
  if (v6 == 1)
    return;
  v7 = v8;
  if (v8 <= v11) {
    do {
      while( true ) {
        v1 = v7 * 8;
        v7 += 1;
        v5 = *(char **)(dat_26388 + v1);
        if ((*v5 != '.') || (v5[1] != '\n')) break;
        fputs_unlocked("..\n.\ns/.//\n",dat_26188);
        if (v11 < v7)
          return;
        fputs_unlocked("a\n",dat_26188);
      }
      sub_f500("");
    } while (v7 <= v11);
  }
  fputs_unlocked(".\n",dat_26188);
}

// Function: sub_a960 @ 0xa960
void sub_a960(unsigned long a0) // return-dupe x2
{
  unsigned char v1;
  unsigned char *v2;
  char *v3;
  unsigned int v4; // eax
  long v5; // rbx
  long v6; // stack - 0x30
  unsigned long v7; // stack - 0x38
  unsigned long v8; // stack - 0x40
  long v9; // stack - 0x28
  
  v4 = sub_f670(a0,&v8,&v7,&v6,&v9);
  if (!v4)
    return;
  sub_e830();
  v1 = *(unsigned char *)((unsigned long)v4 + 0x1e550);
  v2 = *(unsigned char **)((long)dat_26188 + 0x28);
  if (*(unsigned char **)((long)dat_26188 + 0x30) <= v2) // branch-flip
    __overflow(dat_26188,(unsigned int)v1);
  else {
    *(unsigned char **)((long)dat_26188 + 0x28) = &v2[1];
    *v2 = v1;
  }
  sub_f610(0x20,0x261a0,v8,v7);
  v3 = *(char **)((long)dat_26188 + 0x28);
  if (*(char **)((long)dat_26188 + 0x30) <= v3) // branch-flip
    __overflow(dat_26188,10);
  else {
    *(char **)((long)dat_26188 + 0x28) = &v3[1];
    *v3 = 10;
  }
  if (v4 == 1)
    return;
  if (v6 <= v9) {
    do {
      v5 = v6 + 1;
      sub_f500("",dat_26388 + v6 * 8);
      v6 = v5;
    } while (v5 <= v9);
  }
  fputs_unlocked(".\n",dat_26188);
}

// Function: sub_aab0 @ 0xaab0
void sub_aab0(unsigned long a0) // return-dupe x3
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
  
  v1 = sub_f670(a0,&v5,&v4,&v3,&v6);
  if (!v1)
    return;
  sub_e830();
  sub_f5e0(0x261a0,v5,v4,&v7,&v8);
  if (v1 & 1) {
    v2 = 1;
    if (v7 <= v8)
      v2 = (v8 - v7) + 1;
    __fprintf_chk(dat_26188,1,"d%ld %ld\n",v7,v2);
  }
  if (!(v1 & 2))
    return;
  sub_f5e0(0x262d0,v3,v6,&v9,&v10);
  v2 = 1;
  if (v9 <= v10)
    v2 = (v10 - v9) + 1;
  __fprintf_chk(dat_26188,1,"a%ld %ld\n",v8,v2);
  if (v6 < v3)
    return;
  do {
    v2 = v3 + 1;
    sub_f500("",dat_26388 + v3 * 8);
    v3 = v2;
  } while (v2 <= v6);
}

// Function: sub_ac20 @ 0xac20
void sub_ac20(unsigned long a0)
{
  sub_f080(a0,sub_f070,sub_a7c0); // tail-call
}

// Function: sub_ac40 @ 0xac40
void sub_ac40(unsigned long a0)
{
  sub_f080(a0,sub_f060,sub_a960); // tail-call
}

// Function: sub_ac60 @ 0xac60
void sub_ac60(unsigned long a0)
{
  sub_f080(a0,sub_f060,sub_aab0); // tail-call
}

// Function: sub_ac80 @ 0xac80
long sub_ac80(unsigned long *a0,char a1) // early-return
{
  unsigned long v1;
  unsigned long v2;
  
  switch(a1) {
    case 0x45:
      a0 = &a0[3];
      break;
    case 0x46:
      a0 = &a0[3];
label_acf4:
      v1 = a0[1];
      v2 = *a0;
      return sub_f5d0(v2,v1); // tail-call
    default:
      switch(a1) {
        case 0x65:
          break;
        case 0x66:
          goto label_acf4;
        default:
          return -1;
        case 0x6c:
          goto label_ad0c;
        case 0x6d:
          goto label_ad2c;
        case 0x6e:
          goto label_ad44;
        
      }
    case 0x4c:
      a0 = &a0[3];
label_ad0c:
      v1 = a0[2];
      v2 = *a0;
      return sub_f5d0(v2,v1) + -1;
    case 0x4d:
      a0 = &a0[3];
label_ad2c:
      v1 = a0[2];
      v2 = *a0;
      return sub_f5d0(v2,v1); // tail-call
    case 0x4e:
      a0 = &a0[3];
label_ad44:
      return a0[2] - a0[1];
    
  }
  v1 = a0[1];
  v2 = *a0;
  return sub_f5d0(v2,v1) + -1;
}

// Function: sub_ad70 @ 0xad70
unsigned char * sub_ad70(void *a0,void *a1,long a2,unsigned long a3,unsigned long a4) // return-dupe x6
{
  unsigned char v1;
  char *v10; // rax
  char v11 [8];
  char *v12;
  char *v13; // rsp
  char *v14; // rsp
  char *v15; // rsp
  unsigned long v16; // rdi
  unsigned char v17;
  unsigned char *v18; // stack - 0x60
  unsigned char *v19; // stack - 0x58
  long v2;
  unsigned char *v20; // stack - 0x50
  char *v3;
  unsigned char v4; // al
  unsigned int v5;
  unsigned int v6;
  unsigned char *v7;
  unsigned char *v8;
  unsigned char *v9;
  
  v9 = (unsigned char *)((long)a1 + 1);
  v12 = v11;
  v14 = v11;
  v15 = v11;
  do {
    v17 = *v9;
    v6 = (unsigned int)v17;
    v9 = &v9[1];
    if (0x31 <= v17) {
      if ((unsigned int)((int)(char)v17 - 0x30U) <= 9) goto label_ae30;
      goto label_add0;
    }
  } while (!(0xfffedf7fffffffffU >> ((unsigned long)v17 & 0x3f) & 1));
  v5 = (int)(char)v17;
  while (v5 - 0x30 <= 9) {
label_ae30:
    v17 = *v9;
    v9 = &v9[1];
    v6 = (unsigned int)(char)v17;
    v5 = v6;
  }
  if (v17 != 0x2e) { // branch-flip
label_add0:
    v17 = (unsigned char)v6;
    v6 -= 0x58;
    v1 = *v9;
    v4 = (unsigned char)v6;
  }
  else {
    do {
      v17 = *v9;
      v9 = &v9[1];
    } while ((unsigned int)((int)(char)v17 - 0x30U) <= 9);
    v6 = (int)(char)v17 - 0x58;
    v1 = *v9;
    v4 = (unsigned char)v6;
  }
  if (0x21 <= v4)
    return NULL;
  if (0x100801001U >> ((unsigned long)v6 & 0x3f) & 1) {
    if (a2) { // branch-flip
      if (v1 != 0x6e)
        return (unsigned char *)0;
      v19 = (long)sub_f5d0(a2,a3);
    }
    else {
      v19 = (long)sub_ac80(a4,(int)(char)v1);
      if ((long)v19 <= -1)
        return (unsigned char *)0;
    }
    v9 = &v9[1];
    if (!a0)
      return v9;
    v18 = &v9[0xfffffffffffffffe - (long)a1];
    v20 = &v9[0xffffffffffffffff - (long)a1];
    if (0xfa1 <= &v9[1 - (long)a1]) { // branch-flip
      v10 = (char *)sub_16f00();
      v8 = v18;
    }
    else {
      v3 = v11;
      while (v14 != &v11[-((unsigned long)&v9[0x37 - (long)a1] & 0xfffffffffffff000)]) {
        v12 = &v3[-0x1000];
        v13 = &v3[-0x1000];
        *(unsigned long *)&v3[-8] = *(unsigned long *)&v3[-8];
        v14 = &v3[-0x1000];
        v3 = v13;
      }
      v16 = (unsigned long)((unsigned int)&v9[0x37 - (long)a1] & 0xff0);
      v2 = -v16;
      v15 = &v12[v2];
      if (v16)
        *(unsigned long *)&v12[-8] = *(unsigned long *)&v12[-8];
      v10 = (char *)(((unsigned long)&v12[v2 + 0xf] & 0xfffffffffffffff0) + 0x1f & 0xffffffffffffffe0);
      v8 = v18;
    }
    v18 = v19;
    v19 = (unsigned char *)&v10[(long)v20];
    v20 = v8;
    *(unsigned long *)&v15[-8] = 0xaf86;
    memcpy(v10,a1,(unsigned long)v8,v15[-8]);
    v10[(long)v20] = 'l';
    *v19 = v17;
    v19[1] = 0;
    *(unsigned long *)&v15[-8] = 0xafb0;
    __fprintf_chk(a0,1,v10,v18);
    *(unsigned long *)&v15[-8] = 0xafb8;
    sub_121c0(v10);
    return v9;
  }
  if ((char)v6 != '\v')
    return NULL;
  if (v1 != 0x27)
    return (unsigned char *)0;
  v17 = v9[1];
  v6 = (unsigned int)v17;
  if (v17 == 0x27)
    return NULL;
  if (v17 != 0x5c) { // branch-flip
    if (!v17)
      return NULL;
    v8 = &v9[3];
    if (v9[2] != 0x27)
      return (unsigned char *)0;
  }
  else {
    v17 = v9[2];
    v8 = &v9[3];
    if (v17 != 0x27) { // branch-flip
      v6 = 0;
      v7 = v8;
      while( true ) {
        if (8 <= (unsigned int)((int)(char)v17 - 0x30U))
          return (unsigned char *)0;
        v8 = &v7[1];
        v6 = ((int)(char)v17 - 0x30U) + v6 * 8;
        v17 = *v7;
        if (v17 == 0x27) break;
        v7 = v8;
      }
    }
    else {
      v6 = 0;
      v7 = &v9[2];
    }
    if (3 <= &v7[0xfffffffffffffffd - (long)v9])
      return (unsigned char *)0;
  }
  if (!a0)
    return v8;
  v12 = *(char **)((long)a0 + 0x28);
  if (v12 < *(char **)((long)a0 + 0x30)) {
    *(char **)((long)a0 + 0x28) = &v12[1];
    *v12 = (char)v6;
    return v8;
  }
  __overflow(a0,v6 & 0xff);
  return v8;
}

// Function: sub_b070 @ 0xb070
void sub_b070(void *a0,char *a1,long *a2) // return-dupe x3
{
  char v1;
  long v10;
  void *v11;
  char *v12;
  long v13;
  long v2;
  long v3;
  long v4;
  char *v5;
  char *v6;
  unsigned long v7; // rax
  char v8;
  unsigned int v9; // esi
  
  v2 = *a2;
  v13 = a2[1];
  v3 = *(long *)(v2 + 0xb8);
  v4 = a2[2];
  if (!a0)
    return;
  if ((!dat_264a0) && (*a1 == '%')) {
    if (a1[1] != 'l') { // branch-flip
      if ((a1[1] == 'L') && (!a1[2])) {
        v11 = *(void **)(v3 + v13 * 8);
        v7 = *(long *)(v3 + v4 * 8) - (long)v11;
        fwrite_unlocked(v11,1,v7,a0); // return-dupe, tail-call
        return;
      }
    }
    else if ((a1[2] == '\n') && (!a1[3])) {
      if (v4 <= v13)
        return;
      v11 = *(void **)(v3 + v13 * 8);
      v2 = *(long *)(v3 + v4 * 8);
      v7 = (v2 + (unsigned long)(*(char *)(v2 + -1) != '\n')) - (long)v11;
      fwrite_unlocked(v11,1,v7,a0);
      return;
    }
  }
  if (v4 <= v13)
    return;
  do {
    v8 = *a1;
    v12 = a1;
label_b0e8:
    v5 = v12;
    if (v8) {
      do {
        v12 = &v5[1];
        if (v8 != '%') { // branch-flip
          v9 = (unsigned int)v8;
label_b0f6:
          v5 = *(char **)((long)a0 + 0x28);
          if (*(char **)((long)a0 + 0x30) <= v5) goto label_b178;
label_b102:
          *(char **)((long)a0 + 0x28) = &v5[1];
          *v5 = v8;
          v6 = v12;
        }
        else {
          v1 = v5[1];
          v6 = &v5[2];
          if (v1 != 'L') { // branch-flip
            if (v1 != 'l') {
              if (v1 == '%') {
                v9 = 0x25;
                v12 = v6;
                goto label_b0f6;
              }
              v6 = (char *)sub_ad70(a0,v5,v2,v13,0);
              if (v6) goto label_b10d;
              v5 = *(char **)((long)a0 + 0x28);
              v9 = 0x25;
              if (v5 < *(char **)((long)a0 + 0x30)) goto label_b102;
              goto label_b178;
            }
            v10 = *(long *)(v3 + 8 + v13 * 8);
            v10 -= (unsigned long)(*(char *)(v10 + -1) == '\n');
          }
          else {
            v10 = *(long *)(v3 + 8 + v13 * 8);
          }
          sub_f0e0(*(unsigned long *)(v3 + v13 * 8),v10,0,0);
        }
label_b10d:
        v8 = *v6;
        v5 = v6;
        if (!v8) break;
      } while( true );
    }
    v13 += 1;
    if (v4 == v13)
      return;
  } while( true );
label_b178:
  __overflow(a0,v9 & 0xff);
  v8 = *v12;
  goto label_b0e8;
}

// Function: sub_b2a0 @ 0xb2a0
void sub_b2a0(void *a0,unsigned char *a1,unsigned char a2,long a3)
{
  unsigned char *v1;
  unsigned char v10;
  unsigned char *v11; // stack - 0x60
  long v12 [3]; // stack - 0x58
  unsigned long v13;
  void *v14;
  void *v2;
  unsigned char *v3;
  int *v4; // rax
  unsigned char *v5;
  char *v6; // rax
  long v7; // rax
  long v8;
  unsigned char v9;
  
  v9 = *a1;
  if ((v9 == a2) || (!v9))
    return;
  do {
    v1 = &a1[1];
    v10 = v9;
    if (v9 != 0x25) {
label_b2ee:
      v3 = v1;
      if (a0) {
        v1 = *(unsigned char **)((long)a0 + 0x28);
        if (*(unsigned char **)((long)a0 + 0x30) <= v1) {
          __overflow(a0,(unsigned int)v10);
          v9 = *v3;
          goto label_b313;
        }
        *(unsigned char **)((long)a0 + 0x28) = &v1[1];
        *v1 = v10;
      }
      v9 = *v3;
      goto label_b313;
    }
    v10 = a1[1];
    v3 = &a1[2];
    v8 = a3;
    switch(v10) {
      case 0x25:
        v1 = v3;
        goto label_b2ee;
      default:
        v3 = (unsigned char *)sub_ad70(a0,a1,0,0,a3);
        v10 = v9;
        if (!v3) goto label_b2ee;
        v9 = *v3;
        goto label_b313;
      case 0x28:
        v8 = 0;
        while( true ) {
          if (10 <= (unsigned int)((int)(char)*v3 - 0x30U)) { // branch-flip
            v7 = sub_ac80(a3);
            v12[v8] = v7;
            v10 = v9;
            if (v7 < 0) goto label_b2ee;
            v5 = &v3[1];
          }
          else {
            v4 = __errno_location();
            *v4 = 0;
            v12[v8] = strtoimax(v3,&v11,10);
            v10 = v9;
            if (*v4) goto label_b2ee;
            v5 = v11;
          }
          v3 = &v5[1];
          v10 = v9;
          if (*v5 != "=?"[v8]) goto label_b2ee;
          if (v8 == 1) break;
          v8 = 1;
        }
        if (v12[0] != v12[1]) { // branch-flip
          v14 = a0;
          v2 = NULL;
        }
        else {
          v14 = NULL;
          v2 = a0;
        }
        v6 = (char *)sub_b2a0(v2,v3,0x3a,a3);
        if (!*v6)
          return;
        v6 = (char *)sub_b2a0(v14,&v6[1],0x29,a3);
        if (!*v6)
          return;
        v9 = v6[1];
        v3 = (unsigned char *)&v6[1];
        goto label_b313;
      case 0x3c:
        v13 = dat_26448;
        break;
      case 0x3d:
        v13 = dat_26440;
        break;
      case 0x3e:
        v8 = a3 + 0x18;
        v13 = dat_26450;
      
    }
    sub_b070(a0,v13,v8);
    v9 = a1[2];
label_b313:
    if (!v9)
      return;
    a1 = v3;
    if (a2 == v9)
      return;
  } while( true );
}

// Function: sub_b560 @ 0xb560
void sub_b560(unsigned long a0) // return-dupe
{
  unsigned int v1; // eax
  long v2; // stack - 0x88
  long v3; // stack - 0x78
  long v4; // stack - 0x80
  long v5 [3]; // stack - 0x70
  long v6; // stack - 0x58
  unsigned long v7; // stack - 0x50
  long v8; // stack - 0x48
  long v9; // stack - 0x40
  
  v1 = sub_f670(a0,&v2,&v4,&v3,v5);
  if (!v1)
    return;
  sub_e830();
  if ((dat_266f8 < v2) || (dat_266f0 < v3)) {
    v9 = v3;
    v6 = v2;
    v8 = dat_266f0;
    v5[1] = 0x261a0;
    v5[2] = dat_266f8;
    v7 = 0x262d0;
    sub_b2a0(dat_26188,dat_26460,0,&v5[1]);
  }
  dat_266f8 = v4 + 1;
  v5[1] = 0x261a0;
  dat_266f0 = v5[0] + 1;
  v7 = 0x262d0;
  v6 = dat_266f8;
  v9 = dat_266f0;
  sub_b2a0(dat_26188,*(unsigned long *)((unsigned long)v1 * 8 + 0x26460),0,&v5[1]);
}

// Function: sub_b6c0 @ 0xb6c0
void sub_b6c0(unsigned long a0) // return-dupe
{
  unsigned long v1; // stack - 0x48
  long v2; // stack - 0x40
  long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  long v5; // stack - 0x28
  long v6; // stack - 0x20
  
  dat_266f0 = -dat_26288;
  dat_266f8 = dat_266f0;
  sub_f080(a0,sub_f060,sub_b560);
  if ((dat_26270 <= dat_266f8) && (dat_263a0 <= dat_266f0))
    return;
  sub_e830();
  v1 = 0x261a0;
  v2 = dat_266f8;
  v3 = dat_26270;
  v4 = 0x262d0;
  v5 = dat_266f0;
  v6 = dat_263a0;
  sub_b2a0(dat_26188,dat_26460,0,&v1);
}

// Function: sub_b7b0 @ 0xb7b0
void sub_b7b0(long a0) // return-dupe
{
  char *v1;
  char v2;
  unsigned long *v3;
  char *v4; // rax
  char *v5;
  char *v6;
  long v7;
  unsigned long *v8;
  
  v3 = *(unsigned long **)(a0 + 0xa0);
  if (!v3)
    return;
  v7 = *(long *)(a0 + 0xb0);
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
          if (v1 == v4) goto label_b840;
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
label_b840:
    v7 -= (long)v1 - (long)v6;
  }
  v8 = v3;
  if ((v7) && (v8 = (unsigned long *)((long)v3 + v7), *(char *)((long)v3 + v7 + -1) != '\n')) {
    *(char *)v8 = 10;
    v7 += 1;
    *(char *)(a0 + 0x120) = 1;
    v8 = (unsigned long *)((long)v3 + v7);
  }
  *v8 = 0;
  *(long *)(a0 + 0xb0) = v7;
}

// Function: sub_b8a0 @ 0xb8a0
void sub_b8a0(unsigned int *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  if (!a1)
    return;
  if (*(char *)((long)a0 + 0x121))
    return;
  v1 = sub_170e0(*a0,*(long *)&a0[0x2c] + *(long *)&a0[0x28],a1);
  if (v1 != 0xffffffffffffffff) {
    *(unsigned long *)&a0[0x2c] = *(long *)&a0[0x2c] + v1;
    *(bool *)((long)a0 + 0x121) = v1 < a1;
    return;
  }
  sub_e770(*(unsigned long *)&a0[2]); // no-return
}

// Function: sub_b920 @ 0xb920
unsigned long sub_b920(int *a0,char a1)
{
  unsigned long v1; // rax
  void *v2; // rax
  
  if (0 <= *a0) { // branch-flip
    v1 = sub_17190(8,*(unsigned long *)&a0[0x12],0x7fffffffffffffef);
    *(unsigned long *)&a0[0x2a] = v1;
    *(unsigned long *)&a0[0x28] = sub_16a40(v1);
    if (!a1) {
      sub_b8a0(a0,*(unsigned long *)&a0[0x2a]);
      v2 = memchr(*(void **)&a0[0x28],0,*(unsigned long *)&a0[0x2c]);
      return CONCAT71((undefined7)((unsigned long)v2 >> 8),v2 != NULL);
    }
  }
  else {
    a0[0x2a] = 8;
    a0[0x2b] = 0;
    *(unsigned long *)&a0[0x28] = sub_16a40(8);
  }
  a0[0x2c] = 0;
  a0[0x2d] = 0;
  *(char *)((long)a0 + 0x121) = 0;
  return 0;
}

// Function: sub_b9e0 @ 0xb9e0
void sub_b9e0(int *a0) // return-dupe x2
{
  unsigned long v1;
  long v2; // rax
  unsigned long v3; // rax
  unsigned long v4;
  unsigned long v5;
  unsigned long v6; // r12
  
  if (*a0 < 0)
    return;
  if ((a0[10] & 0xf000U) != 0x8000) // branch-flip
    v4 = *(unsigned long *)&a0[0x2c];
  else {
    v5 = *(unsigned long *)&a0[0x10];
    v4 = (v5 & 0xfffffffffffffff8) + 0x10;
    if ((v4 < v5) || (0x7ffffffffffffffe < v4))
      sub_16ec0(); // no-return, return-dupe
    if (v4 <= *(unsigned long *)&a0[0x2a]) // branch-flip
      v4 = *(unsigned long *)&a0[0x2c];
    else {
      *(unsigned long *)&a0[0x2a] = v4;
      v1 = *(unsigned long *)&a0[0x28];
      *(unsigned long *)&a0[0x28] = sub_16a90(v1);
      v4 = *(unsigned long *)&a0[0x2c];
    }
    if (v4 <= v5) {
      sub_b8a0(a0,(v5 - v4) + 1);
      v4 = *(unsigned long *)&a0[0x2c];
      if (v4 <= v5)
        return;
    }
  }
  sub_b8a0(a0,*(long *)&a0[0x2a] - v4);
  v4 = *(unsigned long *)&a0[0x2c];
  if (!v4)
    return;
  v5 = *(unsigned long *)&a0[0x2a];
  if (v4 == v5) {
    do {
      do {
        if (0x3ffffffffffffff7 < v5) {
          sub_16ec0();
        }
        *(unsigned long *)&a0[0x2a] = v5 * 2;
        v2 = sub_16a90(*(unsigned long *)&a0[0x28]);
        v5 = *(unsigned long *)&a0[0x2a];
        v4 = *(unsigned long *)&a0[0x2c];
        *(long *)&a0[0x28] = v2;
        v6 = v5 - v4;
      } while (!v6);
      if (!*(char *)((long)a0 + 0x121)) {
        v3 = sub_170e0(*a0,v2 + v4,v6);
        if (v3 == 0xffffffffffffffff)
          sub_e770(*(unsigned long *)&a0[2]); // no-return
        v5 = *(unsigned long *)&a0[0x2a];
        v4 = *(long *)&a0[0x2c] + v3;
        *(unsigned long *)&a0[0x2c] = v4;
        *(bool *)((long)a0 + 0x121) = v3 < v6;
      }
    } while (v4 == v5);
  }
  v1 = *(unsigned long *)&a0[0x28];
  *(unsigned long *)&a0[0x2a] = v4 + 0x10 & 0xfffffffffffffff8;
  *(unsigned long *)&a0[0x28] = sub_16a90(v1);
}

// Function: sub_bbc0 @ 0xbbc0
unsigned char sub_bbc0(int *a0,unsigned int a1)
{
  unsigned char *v1;
  unsigned char *v10;
  long *v11; // rax
  int v12; // ecx
  unsigned char *v13;
  unsigned char *v14;
  unsigned int v15; // ebx
  unsigned char *v16;
  unsigned char *v17;
  unsigned char *v18;
  unsigned long *v19;
  unsigned long *v2;
  unsigned long v20;
  unsigned long v21;
  unsigned long v22; // rax
  unsigned long v23;
  unsigned char *v24;
  unsigned long v25;
  unsigned long v26;
  char v27; // r13b
  unsigned char *v28;
  unsigned long v29;
  void *v3;
  unsigned char v30;
  unsigned char *v31;
  bool v32; // cf
  bool v33; // zf
  unsigned long v34; // stack - 0xe0
  unsigned long v35; // stack - 0xd8
  unsigned long *v36; // stack - 0xd0
  unsigned long v37; // stack - 0xc0
  void *v38; // stack - 0xb8
  unsigned long v39; // stack - 0xb0
  char v4;
  long v40; // stack - 0xa0
  long v41; // stack - 0x98
  unsigned long v42; // stack - 0x90
  unsigned long v43; // stack - 0x88
  long *v44; // stack - 0x60
  unsigned int v5;
  long v6;
  unsigned long v7; // rax
  long v8;
  long v9;
  
  v15 = (unsigned int)dat_26568;
  v5 = sub_b920(a0,(v15 | a1) & 0xff);
  v30 = (unsigned char)(v5 | a1);
  if (*a0 != a0[0x4c]) // branch-flip
    v30 |= sub_b920(&a0[0x4c],(v15 | a1 | v5 | a1) & 0xff);
  else {
    *(unsigned long *)&a0[0x74] = *(unsigned long *)&a0[0x28];
    *(unsigned long *)&a0[0x76] = *(unsigned long *)&a0[0x2a];
    *(unsigned long *)&a0[0x78] = *(unsigned long *)&a0[0x2c];
  }
  if (v30)
    return v30;
  sub_b9e0(a0);
  sub_b7b0(a0);
  if (*a0 != a0[0x4c]) { // branch-flip
    sub_b9e0(&a0[0x4c]);
    sub_b7b0(&a0[0x4c]);
    v28 = *(unsigned char **)&a0[0x28];
    v31 = *(unsigned char **)&a0[0x74];
    v21 = *(unsigned long *)&a0[0x2c];
    v25 = *(unsigned long *)&a0[0x78];
    if (v28 == v31) goto label_bcf5;
    v10 = &v28[v21];
    v24 = &v31[v25];
    if (v25 <= v21) // branch-flip
      *v24 = ~v28[v25];
    else {
      *v10 = ~v31[v21];
    }
    v16 = v28;
    v13 = v31;
    if (*(long *)v31 == *(long *)v28) {
      do {
        v16 = &v16[8];
        v13 = &v13[8];
      } while (*(long *)v16 == *(long *)v13);
    }
    if (*v13 == *v16) {
      do {
        v16 = &v16[1];
        v13 = &v13[1];
      } while (*v16 == *v13);
    }
    v5 = dat_26580 - 4;
    v8 = dat_26560;
    if ((2 <= v5) && (&v28[v21 - *(unsigned char *)&a0[0x48]] < v16 != &v31[v25 - *(unsigned char *)&a0[0x94]] < v13)) {
      v16 = &v16[-1];
      v13 = &v13[-1];
    }
  }
  else {
    v21 = *(unsigned long *)&a0[0x2c];
    v31 = *(unsigned char **)&a0[0x28];
    *(unsigned long *)&a0[0x76] = *(unsigned long *)&a0[0x2a];
    *(unsigned char **)&a0[0x74] = v31;
    *(unsigned long *)&a0[0x78] = v21;
    *(char *)&a0[0x94] = (char)a0[0x48];
    v25 = v21;
label_bcf5:
    v16 = &v31[v25];
    v10 = &v31[v21];
    v5 = dat_26580 - 4;
    v13 = v16;
    v8 = dat_26560;
    v28 = v31;
    v24 = v16;
  }
  while (v16 != v28) {
    while (v16[-1] != 10) {
      v16 = &v16[-1];
      v13 = &v13[-1];
      if (v16 == v28) goto label_bd54;
    }
    v6 = v8 + -1;
    if (!v8) break;
    v13 = &v13[-1];
    v16 = &v16[-1];
    v8 = v6;
  }
label_bd54:
  *(unsigned char **)&a0[0x38] = v16;
  *(unsigned char **)&a0[0x84] = v13;
  v13 = v24;
  if ((v5 < 2) || (v17 = v10, (char)a0[0x48] == (char)a0[0x94])) {
    v14 = v10;
    v17 = &v16[v21 - v25];
    if (v21 < v25)
      v17 = v16;
    while (v14 != v17) {
      v18 = &v14[-1];
      v1 = &v13[-1];
      if (v14[-1] != v13[-1]) break;
      v13 = v1;
      v14 = v18;
    }
    if ((v14 != v28) && (v14[-1] != 10))
      v8 = dat_26560 + 1;
    else {
      v8 = dat_26560;
      if (v13 != v31)
        v8 = dat_26560 + (unsigned long)(v13[-1] != 10);
    }
    v6 = v8 + -1;
    v17 = v14;
    if ((v8) && (v18 = v14, v14 != v10)) {
      do {
        do {
          v17 = &v18[1];
          v30 = *v18;
          v18 = v17;
        } while (v30 != 10);
        v33 = v6 != 0;
        v6 -= 1;
      } while (v17 != v10 && v33);
      v13 = &v13[(long)v17 - (long)v14];
    }
  }
  *(unsigned char **)&a0[0x3c] = v17;
  *(unsigned char **)&a0[0x88] = v13;
  if ((((dat_26578) && (!dat_26520)) && ((long)dat_26570 <= 0x1ffffffffffffffe)) && (dat_26570 < v21)) {
    v8 = 1;
    v20 = (unsigned long)((long)v17 - (long)v16) >> 5;
    if (!v20)
      v20 = 1;
    if (0x787878787878783 <= v20)
      v20 = 0x787878787878782;
    v29 = (unsigned long)((long)v10 - (long)v17) >> 5;
    if (!v29)
      v29 = 1;
    if (0x787878787878783 <= v29)
      v29 = 0x787878787878782;
    v25 = 0;
    if (0 < (long)dat_26570) {
      do {
        v8 *= 2;
      } while (v8 <= (long)dat_26570);
      v25 = v8 - 1;
    }
    v21 = dat_26570;
    if ((long)(v29 + 5) < (long)dat_26570)
      v21 = v29 + 5;
    v21 = v20 + 5 + v8 + v21;
    v20 = v21;
  }
  else {
    v21 >>= 5;
    v25 = 0xffffffffffffffff;
    if (!v21)
      v21 = 1;
    if (0x787878787878783 <= v21)
      v21 = 0x787878787878782;
    v8 = 0;
    v21 += 5;
    v20 = v21;
  }
  v6 = sub_16a40(v21 * 8);
  v10 = *(unsigned char **)&a0[0x38];
  if (((dat_26578) && (v17 == v10)) && (v16 = *(unsigned char **)&a0[0x84], v13 == v16)) {
    v10 = v28;
label_cbe8:
    if (v8) {
      v26 = 0;
label_bfa9:
      v21 = dat_26570;
      v29 = v26;
      if ((long)dat_26570 <= (long)v26)
        v29 = dat_26570;
      goto label_bfb3;
    }
    v29 = 0;
    v23 = 0x20;
    v22 = (long)v13 - (long)v16;
    v26 = 0;
    v21 = dat_26570;
  }
  else {
    if (v28 == v10) {
      v16 = *(unsigned char **)&a0[0x84];
      goto label_cbe8;
    }
    v16 = v28;
    v21 = 0;
    v29 = v20;
    do {
      v26 = v21 + 1;
      v20 = v29;
      if ((v21 & v25) == v29) {
        if (0x7fffffffffffffe < v29) {
          sub_16ec0(); // no-return, return-dupe
        }
        v20 = v29 * 2;
        v6 = sub_16a90(v6,v29 << 4);
      }
      *(unsigned char **)(v6 + (v21 & v25) * 8) = v16;
      v17 = v16;
      do {
        v16 = &v17[1];
        v30 = *v17;
        v17 = v16;
      } while (v30 != 10);
      v21 = v26;
      v29 = v20;
    } while (v16 != v10);
    v16 = *(unsigned char **)&a0[0x84];
    if (v8) goto label_bfa9;
    v21 = dat_26570;
    v29 = v26;
label_bfb3:
    v23 = 0x20;
    v22 = (long)v13 - (long)v16;
    if (10 <= (long)v26)
      v23 = (unsigned long)((long)v10 - (long)v28) / (v26 - 1);
  }
  v7 = (unsigned long)((long)v24 - (long)v13) / v23;
  if (!v7)
    v7 = 1;
  if (0x787878787878783 <= v7)
    v7 = 0x787878787878782;
  if ((long)(v7 + 5) <= (long)v21)
    v21 = v7 + 5;
  v22 /= v23;
  if (!v22)
    v22 = 1;
  if (0x787878787878783 <= v22)
    v22 = 0x787878787878782;
  v21 += v29 + 5 + v22;
  if ((long)v21 < (long)v29 || 0xffffffffffffffe < v21) {
    sub_16ec0();
  }
  v8 = sub_16a40(v21 * 8);
  if (v29 != v26) { // branch-flip
    if (1 <= (long)v29) {
      v22 = v26 - dat_26570;
      v23 = v29 + v22;
      v9 = v22 * -8;
      do {
        *(unsigned long *)(v8 + v9 + v22 * 8) = *(unsigned long *)(v6 + (v25 & v22) * 8);
        v22 += 1;
      } while (v23 != v22);
      v25 = 0;
      do {
        *(unsigned long *)(v6 + v25 * 8) = *(unsigned long *)(v8 + v25 * 8);
        v25 += 1;
      } while (v29 != v25);
label_c0d0:
      v9 = 0;
      do {
        *(unsigned char **)(v8 + v9 * 8) = &v31[*(long *)(v6 + v9 * 8) - (long)v28];
        v9 += 1;
      } while (v9 < (long)v29);
    }
  }
  else if (v29) goto label_c0d0;
  *(unsigned long *)&a0[0x7a] = v29 * 8 + v8;
  *(unsigned long *)&a0[0x2e] = v6 + v29 * 8;
  *(unsigned long *)&a0[0x7c] = -v29;
  *(unsigned long *)&a0[0x30] = -v29;
  dat_26700 = (v20 - v29) + 1 + (v21 - v29);
  *(unsigned long *)&a0[0x36] = v20 - v29;
  *(unsigned long *)&a0[0x82] = v21 - v29;
  *(unsigned long *)&a0[0x86] = v26;
  *(unsigned long *)&a0[0x3a] = v26;
  if (dat_26700 > 0x3fffffffffffffe)
    sub_16ec0();
  dat_26710 = (void *)sub_16a40(dat_26700 * 0x20);
  dat_26708 = 1;
  if (0x201 <= (unsigned long)((long)dat_26700 / 3)) { // branch-flip
    v12 = 9;
    do {
      v12 += 1;
      dat_26718 = 1L << ((unsigned char)v12 & 0x3f);
    } while (dat_26718 < (unsigned long)((long)dat_26700 / 3));
    dat_26718 -= *(unsigned char *)((long)v12 + 0x1e1a0);
    if (0xffffffffffffffe < dat_26718) {
      sub_16ec0();
    }
    v8 = dat_26718 * 8 + 8;
  }
  else {
    dat_26718 = 0x1fd;
    v8 = 0xff0;
  }
  dat_26720 = sub_fab0(v8) + 8;
  v44 = (long *)&a0[0x28];
  do {
    v40 = v44[3];
    v8 = v44[4];
    v31 = (unsigned char *)v44[8];
    v42 = v44[7];
    v41 = sub_16a40(v42 * 8);
    v5 = dat_26558;
    v4 = dat_26552;
    v28 = (unsigned char *)v44[10];
    v38 = dat_26710;
    v37 = dat_26708;
    v43 = dat_26700;
    v10 = (unsigned char *)(v44[2] + *v44);
    v33 = dat_26558 != 0;
    if (v31 < v28) {
      v15 = dat_26558 & 1;
      v30 = *v31;
      v35 = 0;
      v24 = v31;
      v21 = dat_26700;
      v25 = dat_26498;
      do {
        v20 = (unsigned long)v30;
        v31 = &v24[1];
        dat_26498 = v25;
        if (v5 != 4) { // branch-flip
          if (5 <= v5) { // branch-flip
            if (v5 != 5) { // branch-flip
label_c810:
              if (v4) { // branch-flip
                if (v30 == 10) goto label_c8d1;
                v29 = 0;
                v16 = v31;
                do {
                  v31 = &v16[1];
                  v6 = v20 * 4;
                  v30 = *v16;
                  v20 = (unsigned long)v30;
                  v29 = (v29 << 7 | v29 >> 0x39) + (long)*(int *)(*(long *)__ctype_tolower_loc() + v6);
                  v16 = v31;
                } while (v30 != 10);
                v16 = &v31[0xffffffffffffffff - (long)v24];
              }
              else {
                v29 = 0;
                v16 = NULL;
                v13 = v31;
                if (v30 != 10) {
                  do {
                    v31 = &v13[1];
                    v29 = (v29 << 7 | v29 >> 0x39) + v20;
                    v30 = *v13;
                    v20 = (unsigned long)v30;
                    v13 = v31;
                  } while (v30 != 10);
                  v16 = &v31[0xffffffffffffffff - (long)v24];
                }
              }
            }
            else {
              if (v30 == 10) goto label_c8d1;
              v29 = 0;
              v6 = *(long *)__ctype_b_loc();
              do {
                if (!(*(unsigned char *)(v6 + 1 + v20 * 2) & 0x20)) {
                  if (v4)
                    v20 = (unsigned long)*(int *)(*(long *)__ctype_tolower_loc() + v20 * 4);
                  v29 = (v29 << 7 | v29 >> 0x39) + v20;
                }
                v30 = *v31;
                v20 = (unsigned long)v30;
                v31 = &v31[1];
              } while (v30 != 10);
              v16 = &v31[0xffffffffffffffff - (long)v24];
            }
          }
          else {
            if (v5 - 1 > 2) goto label_c810;
            if (v30 == 10) goto label_c8d1;
            v29 = 0;
            v23 = 0;
            v26 = v20;
            v16 = v31;
            do {
              if (v5 & 2) {
                v6 = *(long *)__ctype_b_loc();
                if (*(unsigned char *)(v6 + 1 + v20 * 2) & 0x20) {
                  v31 = v16;
                  do {
                    v30 = *v31;
                    v31 = &v31[1];
                    if (v30 == 10) {
                      v16 = &v31[0xffffffffffffffff - (long)v24];
                      goto label_c3d0;
                    }
                  } while (*(unsigned char *)(v6 + 1 + (unsigned long)v30 * 2) & 0x20);
                }
              }
              if (v15) { // branch-flip
                v27 = (char)v26;
                if (v27 != '\t') { // branch-flip
                  if (v27 != '\r') { // branch-flip
                    if (v27 == '\b') {
                      v20 = 8;
                      v23 = (v23 - 1) + (unsigned long)(v23 == 0);
                      goto label_c6a0;
                    }
                    v23 += 1;
                    v26 = 1;
                  }
                  else {
                    v20 = 0xd;
                    v26 = 1;
                    v23 = 0;
                  }
                }
                else {
                  v20 = 0x20;
                  v26 = v25 - v23 % v25;
                  v32 = CARRY8(v23,v26);
                  v23 += v26;
                  if (v32)
                    v23 = 0;
                }
              }
              else {
label_c6a0:
                v26 = 1;
              }
              if (v4)
                v20 = (unsigned long)*(unsigned char *)(*(long *)__ctype_tolower_loc() + v20 * 4);
              do {
                v29 = (v29 << 7 | v29 >> 0x39) + v20;
                v26 -= 1;
              } while (v26);
              v30 = *v16;
              v20 = (unsigned long)v30;
              v31 = &v16[1];
              v26 = v20;
              v16 = v31;
            } while (v30 != 10);
            v16 = &v31[0xffffffffffffffff - (long)v24];
          }
        }
        else {
          if (v30 != 10) {
            v29 = 0;
            v6 = *(long *)__ctype_b_loc();
            do {
              v16 = v31;
              if (*(unsigned char *)(v6 + 1 + v20 * 2) & 0x20) {
                v30 = *v31;
                while( true ) {
                  v31 = &v31[1];
                  if (v30 == 10) goto label_c549;
                  v20 = (unsigned long)v30;
                  if (!(*(unsigned char *)(v6 + 1 + v20 * 2) & 0x20)) break;
                  v30 = *v31;
                }
                v29 = (v29 << 7 | v29 >> 0x39) + 0x20;
                v16 = v31;
              }
              if (v4)
                v20 = (unsigned long)*(int *)(*(long *)__ctype_tolower_loc() + v20 * 4);
              v31 = &v16[1];
              v29 = (v29 << 7 | v29 >> 0x39) + v20;
              v20 = (unsigned long)*v16;
            } while (*v16 != 10);
label_c549:
            v16 = &v31[0xffffffffffffffff - (long)v24];
            goto label_c3d0;
          }
label_c8d1:
          v16 = NULL;
          v29 = 0;
        }
label_c3d0:
        v19 = (unsigned long *)(dat_26720 + (v29 % dat_26718) * 8);
        v36 = v19;
        if (((v10 == v31) && ((char)v44[0x10])) && ((2 <= (unsigned int)(dat_26580 - 4U) && (v36 = (unsigned long *)(dat_26720 + -8), 2 <= v5))))
          v36 = v19;
        v39 = *v36;
        v43 = v21;
        if (v39) {
          v34 = v39;
label_c432:
          do {
            v19 = (unsigned long *)(v34 * 0x20 + (long)v38);
            if (v29 == v19[1]) {
              v3 = (void *)v19[2];
              if ((unsigned char *)v19[3] != v16) { // branch-flip
                if (!v5) goto label_c420;
              }
              else {
                if (!memcmp(v3,v24,(unsigned long)v16)) goto label_c477;
                if (!(v33 || v4)) {
                  v34 = *v19;
                  if (!v34) break;
                  goto label_c432;
                }
              }
              if (!sub_ec00(v3,v24)) goto label_c477;
            }
label_c420:
            v34 = *v19;
          } while (v34);
        }
        if (v21 == v37) {
          if (0x1fffffffffffffe < v21) {
            sub_16ec0();
          }
          v43 = v21 * 2;
          v38 = (void *)sub_16a90(v38,v21 << 6);
          v39 = *v36;
        }
        v19 = (unsigned long *)(v37 * 0x20 + (long)v38);
        *v19 = v39;
        v19[1] = v29;
        v19[2] = (unsigned long)v24;
        v19[3] = (unsigned long)v16;
        *v36 = v37;
        v34 = v37;
        v37 += 1;
label_c477:
        if (v35 == v42) {
          if (((0x2aaaaaaaaaaaaaa9 < (long)v35) || (v42 = v35 * 2 - v8, 0xffffffffffffffe < v42)) || (0xffffffffffffffe < v35 - v8)) {
            sub_16ec0();
          }
          v41 = sub_16a90(v41,v42 * 8);
          v40 = sub_16a90(v40 + v8 * 8,(v42 - v8) * 8) + v8 * -8;
        }
        *(unsigned char **)(v40 + v35 * 8) = v24;
        *(unsigned long *)(v41 + v35 * 8) = v34;
        v35 += 1;
        if (v28 <= v31) goto label_c8e7;
        v30 = *v31;
        v24 = v31;
        v21 = v43;
        v25 = dat_26498;
      } while( true );
    }
    v35 = 0;
label_c8e7:
    v6 = 0;
    v44[5] = v35;
    while( true ) {
      if (v42 != v35) { // branch-flip
        v2 = (unsigned long *)(v40 + v35 * 8);
        *v2 = v31;
      }
      else {
        if (((0x2aaaaaaaaaaaaaa9 < (long)v42) || (v21 = v42 * 2 - v8, 0xffffffffffffffe < v21)) || (0xffffffffffffffe < v42 - v8)) {
          sub_16ec0();
        }
        v40 = sub_16a90(v40 + v8 * 8,(v21 - v8) * 8) + v8 * -8;
        v2 = (unsigned long *)(v40 + v35 * 8);
        *v2 = v31;
        v42 = v21;
      }
      if (v10 == v31) break;
      if (((long)dat_26570 <= v6) && (dat_26578)) goto label_c9fc;
      v35 += 1;
      v28 = v31;
      do {
        v31 = &v28[1];
        v30 = *v28;
        v28 = v31;
      } while (v30 != 10);
      v6 += 1;
    }
    if (((char)v44[0x10]) && (2 <= (unsigned int)(dat_26580 - 4U)))
      *v2 = &v10[-1];
label_c9fc:
    dat_26700 = v43;
    v44[3] = v40;
    v11 = &v44[0x26];
    dat_26708 = v37;
    v44[6] = v35;
    v44[7] = v42;
    v44[0xb] = v41;
    dat_26710 = v38;
    v44 = v11;
    if ((long *)&a0[0xc0] == v11) {
      *(unsigned long *)&a0[0x96] = v37;
      *(unsigned long *)&a0[0x4a] = v37;
      free(v38);
      free((void *)(dat_26720 + -8));
      return 0;
    }
  } while( true );
}

// Function: sub_cd40 @ 0xcd40
void sub_cd40(unsigned long a0) // return-dupe x2
{
  unsigned char v1;
  long v10;
  long v11; // rsi
  unsigned char *v2;
  char *v3;
  unsigned int v4; // eax
  long v5;
  long v6; // stack - 0x40
  long v7; // stack - 0x48
  long v8; // stack - 0x50
  long v9; // stack - 0x38
  
  v4 = sub_f670(a0,&v8,&v7,&v6,&v9);
  if (!v4)
    return;
  sub_e830();
  sub_f510(4);
  sub_f610(0x2c,0x261a0,v8,v7);
  v1 = *(unsigned char *)((unsigned long)v4 + 0x1e550);
  v2 = *(unsigned char **)((long)dat_26188 + 0x28);
  if (*(unsigned char **)((long)dat_26188 + 0x30) <= v2) // branch-flip
    __overflow(dat_26188,(unsigned int)v1);
  else {
    *(unsigned char **)((long)dat_26188 + 0x28) = &v2[1];
    *v2 = v1;
  }
  sub_f610(0x2c,0x262d0,v6,v9);
  sub_f510(3);
  v3 = *(char **)((long)dat_26188 + 0x28);
  if (*(char **)((long)dat_26188 + 0x30) <= v3) // branch-flip
    __overflow(dat_26188,10);
  else {
    *(char **)((long)dat_26188 + 0x28) = &v3[1];
    *v3 = 10;
  }
  if ((v4 & 1) && (v8 <= v7)) {
    v5 = v8 * 8;
    v10 = v8;
    do {
      while( true ) {
        sub_f510(2);
        v11 = dat_26258 + v5;
        v5 += 8;
        sub_f330("<",v11,1);
        sub_f510(3);
        if (*(char *)(*(long *)(dat_26258 + v5) + -1) == '\n') break;
label_ce70:
        v10 += 1;
        if (v7 < v10) goto label_cef0;
      }
      v3 = *(char **)((long)dat_26188 + 0x28);
      if (*(char **)((long)dat_26188 + 0x30) <= v3) {
        __overflow(dat_26188,10);
        goto label_ce70;
      }
      v10 += 1;
      *(char **)((long)dat_26188 + 0x28) = &v3[1];
      *v3 = 10;
    } while (v10 <= v7);
  }
label_cef0:
  if (v4 != 3) { // branch-flip
    if (!(v4 & 2))
      return;
  }
  else {
    fputs_unlocked("---\n",dat_26188);
  }
  if (v9 < v6)
    return;
  v5 = v6 * 8;
  v10 = v6;
  do {
    sub_f510(1);
    v11 = dat_26388 + v5;
    v5 += 8;
    sub_f330(">",v11,1);
    sub_f510(3);
    if (*(char *)(*(long *)(dat_26388 + v5) + -1) == '\n') {
      v3 = *(char **)((long)dat_26188 + 0x28);
      if (*(char **)((long)dat_26188 + 0x30) <= v3) // branch-flip
        __overflow(dat_26188,10);
      else {
        *(char **)((long)dat_26188 + 0x28) = &v3[1];
        *v3 = 10;
      }
    }
    v10 += 1;
  } while (v10 <= v9);
}

// Function: sub_cff0 @ 0xcff0
void sub_cff0(unsigned long a0)
{
  sub_f080(a0,sub_f060,sub_cd40); // tail-call
}

// Function: sub_d010 @ 0xd010
unsigned long sub_d010(unsigned long a0,unsigned long a1)
{
  char *v1;
  unsigned long v2;
  void *v3;
  unsigned long v4;
  unsigned long v5;
  
  v4 = dat_26498;
  v3 = dat_26188;
  if (!dat_264a0) {
    v5 = (dat_26498 + a0) - a0 % dat_26498;
    while (v2 = v5, v2 <= a1) {
      v1 = *(char **)((long)v3 + 0x28);
      if (*(char **)((long)v3 + 0x30) <= v1) // branch-flip
        __overflow(v3,9);
      else {
        *(char **)((long)v3 + 0x28) = &v1[1];
        *v1 = 9;
      }
      v5 = v4 + v2;
      a0 = v2;
    }
  }
  if (a0 < a1) {
    do {
      a0 += 1;
      v1 = *(char **)((long)v3 + 0x28);
      if (*(char **)((long)v3 + 0x30) <= v1) // branch-flip
        __overflow(v3,0x20);
      else {
        *(char **)((long)v3 + 0x28) = &v1[1];
        *v1 = 0x20;
      }
    } while (a1 != a0);
  }
  return a1;
}

// Function: sub_d0d0 @ 0xd0d0
unsigned long sub_d0d0(unsigned long *a0,unsigned long a1,unsigned long a2)
{
  unsigned char v1;
  unsigned char *v10;
  unsigned char *v11;
  unsigned long v12;
  unsigned long v13;
  unsigned char *v2;
  char *v3;
  void *v4;
  int v5; // eax
  unsigned long v6;
  long v7; // rcx
  unsigned long v8; // stack - 0x48
  int v9; // stack - 0x4c
  
  v4 = dat_26188;
  v6 = 0;
  v2 = (unsigned char *)a0[1];
  v8 = 0;
  if ((unsigned char *)*a0 < v2) {
    v10 = (unsigned char *)*a0;
    v12 = v6;
    v13 = 0;
    do {
      while( true ) {
        v1 = *v10;
        v11 = &v10[1];
        if ('_' < (char)v1) break;
        if ('\b' <= (char)v1) { // branch-flip
          switch(v1) {
            case 8:
              if (v13) {
                v13 -= 1;
                if (v13 < a2) {
                  if (v12 <= v13) { // branch-flip
                    if (v12 < v13) {
                      do {
                        v3 = *(char **)((long)v4 + 0x28);
                        if (*(char **)((long)v4 + 0x30) <= v3) // branch-flip
                          __overflow(v4,0x20);
                        else {
                          *(char **)((long)v4 + 0x28) = &v3[1];
                          *v3 = 0x20;
                        }
                        v12 += 1;
                      } while (v13 != v12);
                    }
                  }
                  else {
                    v3 = *(char **)((long)v4 + 0x28);
                    v12 = v13;
                    if (*(char **)((long)v4 + 0x30) <= v3) // branch-flip
                      __overflow(v4,8);
                    else {
                      *(char **)((long)v4 + 0x28) = &v3[1];
                      *v3 = 8;
                    }
                  }
                }
              }
              break;
            case 9:
              v7 = dat_26498 - v13 % dat_26498;
              if (v12 == v13) {
                v6 = v7 + v12;
                if (dat_264a0) { // branch-flip
                  if (a2 <= v6)
                    v6 = a2;
                  if (v12 < v6) {
                    do {
                      v3 = *(char **)((long)v4 + 0x28);
                      if (*(char **)((long)v4 + 0x30) <= v3) // branch-flip
                        __overflow(v4,0x20);
                      else {
                        *(char **)((long)v4 + 0x28) = &v3[1];
                        *v3 = 0x20;
                      }
                      v12 += 1;
                    } while (v12 != v6);
                  }
                }
                else if (v6 < a2) {
                  v3 = *(char **)((long)v4 + 0x28);
                  v12 = v6;
                  if (*(char **)((long)v4 + 0x30) <= v3) // branch-flip
                    __overflow(v4,9);
                  else {
                    *(char **)((long)v4 + 0x28) = &v3[1];
                    *v3 = 9;
                  }
                }
              }
              v13 += v7;
              break;
            case 10:
              return v12;
            case 0xb:
            case 0xc:
              goto label_d1fd;
            case 0xd:
              v3 = *(char **)((long)v4 + 0x28);
              if (*(char **)((long)v4 + 0x30) <= v3) // branch-flip
                __overflow(v4,0xd);
              else {
                *(char **)((long)v4 + 0x28) = &v3[1];
                *v3 = 0xd;
              }
              v13 = 0;
              sub_d010(0,a1);
              v12 = 0;
              break;
            default:
              goto label_d1c0;
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
              goto label_d15f;
            
          }
        }
        else {
label_d1c0:
          v6 = sub_17200(&v9,v10,(long)v2 - (long)v10,&v8);
          if (0xfffffffffffffffd <= v6 - 1) { // branch-flip
label_d1fd:
            if (v13 < a2) {
              v10 = *(unsigned char **)((long)v4 + 0x28);
              if (*(unsigned char **)((long)v4 + 0x30) <= v10) // branch-flip
                __overflow(v4,(unsigned int)v1);
              else {
                *(unsigned char **)((long)v4 + 0x28) = &v10[1];
                *v10 = v1;
              }
            }
          }
          else {
            v5 = wcwidth(v9);
            if (1 <= v5)
              v13 = (long)v5 + v13;
            if (v13 <= a2) {
              fwrite_unlocked(v10,1,v6,stdout);
              v12 = v13;
            }
            v11 = &v10[v6];
          }
        }
label_d220:
        v10 = v11;
        if (v2 <= v11)
          return v12;
      }
      if (0x1d < (unsigned char)(v1 + 0x9f)) goto label_d1c0;
label_d15f:
      v6 = v13 + 1;
      if (a2 <= v13) {
        v13 = v6;
        goto label_d220;
      }
      v10 = *(unsigned char **)((long)v4 + 0x28);
      v12 = v6;
      if (*(unsigned char **)((long)v4 + 0x30) <= v10) {
        __overflow(v4,(unsigned int)v1);
        v13 = v6;
        goto label_d220;
      }
      *(unsigned char **)((long)v4 + 0x28) = &v10[1];
      *v10 = v1;
      v10 = v11;
      v13 = v6;
    } while (v11 < v2);
  }
  return v6;
}

// Function: sub_d500 @ 0xd500
void sub_d500(long a0,char a1,unsigned long *a2)
{
  char *v1;
  char v10; // stack - 0x39
  char *v2;
  bool v3;
  void *v4;
  long v5;
  long v6;
  long v7;
  int v8; // esi
  bool v9;
  
  v6 = dat_26430;
  v5 = dat_26428;
  v4 = dat_26188;
  v10 = a1;
  if (a1 != '<') { // branch-flip
    if (a1 == '>') {
      sub_f510(1);
      goto label_d67a;
    }
    v3 = 0;
    if (a0) goto label_d556;
    v9 = 0;
    v7 = 0;
label_d5f8:
    if (a1 == ' ') goto label_d57b;
    v7 = sub_d010(v7,(unsigned long)(v6 + -1 + v5) >> 1) + 1;
    if (a1 != '|') goto label_d6a2;
    v8 = 0x7c;
    if ((*(char *)(a2[1] + -1) == '\n') != v9) {
      v8 = (-(unsigned int)(v9 == 0) & 0x2d) + 0x2f;
      v10 = (-(v9 == 0) & 0x2dU) + 0x2f;
    }
  }
  else {
    sub_f510(2);
label_d67a:
    v3 = 1;
    if (a0) {
label_d556:
      v9 = *(char *)(*(long *)(a0 + 8) + -1) == '\n';
      v7 = sub_d0d0(a0,0,v6);
      goto label_d5f8;
    }
    v9 = 0;
    v7 = sub_d010(0,(unsigned long)(v6 + -1 + v5) >> 1) + 1;
label_d6a2:
    v8 = (int)a1;
  }
  v2 = *(char **)((long)v4 + 0x28);
  if (*(char **)((long)v4 + 0x30) <= v2) // branch-flip
    __overflow(v4,v8);
  else {
    *(char **)((long)v4 + 0x28) = &v2[1];
    *v2 = v10;
  }
label_d57b:
  if ((a2) && (v9 = (bool)(v9 | *(char *)(a2[1] + -1) == '\n'), *(char *)*a2 != '\n'))
    sub_d0d0(a2,sub_d010(v7,v5),v6);
  if (v9) {
    v1 = *(char **)((long)v4 + 0x28);
    if (*(char **)((long)v4 + 0x30) <= v1) // branch-flip
      __overflow(v4,10);
    else {
      *(char **)((long)v4 + 0x28) = &v1[1];
      *v1 = 10;
    }
  }
  if (v3) {
    sub_f510(3); // tail-call
    return;
  }
}

// Function: sub_d700 @ 0xd700
void sub_d700(long a0,long a1) // return-dupe x3
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
  if (dat_26439) goto label_d81b;
  if ((v4) && (v5)) {
    while( true ) {
      v2 += 1;
      v3 += 1;
      sub_d500(dat_26258 + -8 + v3 * 8,0x20,dat_26388 + -8 + v2 * 8);
      if (a0 == v3) break;
      if (a1 == v2) goto label_d81b;
    }
  }
  if (a1 == v2) goto label_d81b;
  do {
    v1 = v2 * 8;
    v2 += 1;
    sub_d500(0,0x29,dat_26388 + v1);
  } while (a1 != v2);
  if (a0 == v3) {
    dat_26728 = a1;
    dat_26730 = a0;
    return;
  }
  do {
    v2 = v3 * 8;
    v3 += 1;
    sub_d500(dat_26258 + v2,0x28,0);
label_d81b:
  } while (a0 != v3);
  dat_26728 = a1;
  dat_26730 = a0;
}

// Function: sub_d860 @ 0xd860
void sub_d860(unsigned long a0) // return-dupe x2, return-dupe
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
  
  v1 = sub_f670(a0,&v7,&v6,&v5,&v8);
  if (!v1)
    return;
  sub_d700(v7,v5);
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
        if (v9 > v8) goto label_d9cf;
        v3 = v9 * 8;
        v9 += 1;
        v2 = v4 * 8;
        v4 += 1;
        sub_d500(dat_26258 + v2,0x7c,dat_26388 + v3);
      } while (v6 >= v4);
      v3 = v8;
      v2 = v6;
      v7 = v4;
      v5 = v9;
    }
label_d9cf:
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
        sub_d500(0,0x3e,dat_26388 + v2 * 8);
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
      sub_d500(dat_26258 + v2,0x3c,0);
    } while (v3 <= v6);
    dat_26730 = v3;
    return;
  }
  dat_26730 = v7;
}

// Function: sub_da50 @ 0xda50
void sub_da50(unsigned long a0)
{
  sub_e830();
  dat_26728 = -dat_26288;
  dat_26730 = dat_26728;
  sub_f080(a0,sub_f060,sub_d860);
  sub_d700(dat_26270,dat_263a0); // tail-call
}

// Function: sub_daa0 @ 0xdaa0
void sub_daa0(unsigned int a0) // return-dupe
{
  if (dat_26774)
    return;
  dat_26774 = a0;
}

// Function: sub_dac0 @ 0xdac0
unsigned int sub_dac0(long *a0,long *a1,unsigned int a2,long *a3)
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
  if (v5 == '\\') goto label_db26;
label_dafa:
  if (v13 || SBORROW1(v5,'\\') != v6 < '\0') {
    if (v5 != '=') { // branch-flip
      if ((v5 <= '=') && ((!v5 || (v5 == ':')))) {
        v2 = 1;
label_db5e:
        *a0 = v11;
        *a1 = (long)v3;
        *a3 = v10;
        return v2;
      }
    }
    else {
      v2 = a2;
      if ((char)a2) goto label_db5e;
    }
  }
  else if (v5 == '^') {
    v1 = v3[1];
    if ((unsigned char)(v1 - 0x40) <= 0x3e) {
      v3 = &v3[2];
      v9 += 1;
      v8 = v7 + 1;
      *(unsigned char *)(v7 + -1) = v1 & 0x1f;
      goto label_db16;
    }
    v3 = &v3[1];
    if (v1 == 0x3f) {
      v9 += 1;
      v8 = v7 + 1;
      *(char *)(v7 + -1) = 0x7f;
      goto label_db16;
    }
    v2 = 0;
    goto label_db5e;
  }
  v3 = &v3[1];
  v8 = v7;
label_db0b:
  *(char *)(v8 + -1) = v5;
  v9 += 1;
  v8 += 1;
label_db16:
  do {
    v5 = *v3;
    v11 = v8 + -1;
    v10 = v9 + -1;
    v6 = v5 + '\xa4';
    v13 = v6 == '\0';
    v4 = v3;
    v7 = v8;
    if (!v13) goto label_dafa;
label_db26:
    v6 = v4[1];
    v3 = &v4[2];
    if (!v6) {
      v2 = 0;
      goto label_db5e;
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
        if (8 <= (unsigned char)(v6 - 0x30U)) goto label_db0b;
        do {
          v3 = &v3[1];
          v5 = v6 + '\xd0' + v5 * '\b';
          v6 = *v3;
        } while ((unsigned char)(v6 - 0x30U) <= 7);
        *(char *)(v8 + -1) = v5;
        v9 += 1;
        v8 += 1;
        goto label_db16;
      default:
        goto label_dba0;
      case 0xf:
        v6 = '\x7f';
        break;
      case 0x28:
      case 0x48:
        goto label_dc37;
      case 0x2f:
        v6 = ' ';
        break;
      case 0x31:
        v6 = '\a';
        goto label_dba0;
      case 0x32:
        v6 = '\b';
        goto label_dba0;
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
label_dba0:
      
    }
    *(char *)(v8 + -1) = v6;
    v9 += 1;
    v8 += 1;
  } while( true );
label_dc37:
  v6 = v4[2];
  v12 = &v4[3];
  v5 = '\0';
  v3 = &v4[2];
  if ('F' < v6) goto label_dc82;
  do {
    if ('A' <= v6) { // branch-flip
      v5 = v6 + '\xc9' + v5 * '\x10';
      v3 = v12;
    }
    else {
      if (10 <= (unsigned char)(v6 - 0x30U)) goto label_db0b;
      v5 = v6 + '\xd0' + v5 * '\x10';
      v3 = v12;
    }
    while( true ) {
      v6 = *v3;
      v12 = &v3[1];
      if (v6 <= 'F') break;
label_dc82:
      if (6 <= (unsigned char)(v6 + 0x9fU)) goto label_db0b;
      v5 = v6 + '\xa9' + v5 * '\x10';
      v3 = v12;
    }
  } while( true );
}

// Function: sub_dd30 @ 0xdd30
void sub_dd30(char a0) // return-dupe
{
  char v1;
  void *v10; // stack - 0xe8
  int *v11; // rbp
  char *v12;
  bool v13;
  unsigned long v14; // stack - 0xe0
  unsigned long v15; // stack - 0xd8
  unsigned long v16; // stack - 0xd0
  unsigned long v17; // stack - 0xc8
  unsigned long v18; // stack - 0xc0
  unsigned long v19; // stack - 0xb8
  int v2;
  unsigned long v20; // stack - 0xb0
  unsigned long v21; // stack - 0xa8
  unsigned long v22; // stack - 0xa0
  unsigned long v23; // stack - 0x98
  unsigned long v24; // stack - 0x90
  unsigned long v25; // stack - 0x88
  unsigned long v26; // stack - 0x80
  unsigned long v27; // stack - 0x78
  unsigned long v28; // stack - 0x70
  unsigned long v29; // stack - 0x68
  void *v3;
  unsigned int v30; // stack - 0x60
  char v31; // stack - 0x41
  void *v4;
  long v5; // rcx
  int *v6; // rbx
  unsigned short v7; // stack - 0x43
  char *v8; // stack - 0xf8
  void *v9; // stack - 0xf0
  
  if (!dat_26188)
    return;
  if (!dat_2657c)
    return;
  if (dat_26180) { // branch-flip
    v13 = 1;
label_dd84:
    if (dat_2657c == 2) goto label_dd9f;
    if ((dat_2657c == 1) && (v13)) {
      v13 = 1;
      goto label_dd9f;
    }
  }
  else {
    if (!a0) {
      v2 = fileno(dat_26188);
      v13 = isatty(v2) != 0;
      goto label_dd84;
    }
    if (dat_2657c != 2) {
      dat_26758 = 0;
      return;
    }
    v13 = 0;
label_dd9f:
    v12 = dat_26748;
    dat_26758 = 1;
    v8 = dat_26748;
    if ((!dat_26748) || (!*dat_26748)) goto label_df18;
    v31 = 0;
    v7 = 0x3f3f;
    v3 = (void *)sub_16ea0(dat_26748);
    v9 = v3;
label_ddf0:
    while (v1 = *v12, v1 == '*') {
      v4 = (void *)sub_16a40(0x28);
      *(void **)((long)v4 + 0x20) = dat_26750;
      *(void **)((long)v4 + 8) = v9;
      dat_26750 = v4;
      v8 = &v12[1];
      if ((!sub_dac0(&v9,&v8,1,v4)) || (v12 = &v8[1], v1 = *v8, v8 = v12, v1 != '=')) goto label_deca;
      *(void **)((long)v4 + 0x18) = v9;
      if (!sub_dac0(&v9,&v8,0,(long)v4 + 0x10)) goto label_deca;
label_e14b:
      v12 = v8;
    }
    if (v1 == ':') {
      v12 = &v12[1];
      v8 = v12;
      goto label_ddf0;
    }
    if (!v1) goto label_df18;
    v7 = CONCAT11(v7._1_1_,v1);
    v8 = &v12[1];
    if (v12[1]) {
      v7 = CONCAT11(v12[1],v1);
      v8 = &v12[3];
      if (v12[2] == '=') {
        v5 = 0;
        v12 = "lc";
        while (strcmp((char *)&v7,v12)) {
          v5 += 1;
          v12 = *(char **)(v5 * 8 + 0x25920);
          if (!v12) goto label_dea6;
        }
        v5 = (long)(int)v5 * 0x10;
        *(void **)(v5 + 0x26028) = v9;
        if (sub_dac0(&v9,&v8,0,v5 + 0x26020)) goto label_e14b;
label_dea6:
        error(0,0,dcgettext(NULL,"unrecognized prefix: %s",5),&v7);
      }
    }
label_deca:
    error(0,0,dcgettext(NULL,"unparsable value for --palette",5));
    free(v3);
    v3 = dat_26750;
    while (v3) {
      v4 = *(void **)((long)v3 + 0x20);
      free(v3);
      v3 = v4;
    }
  }
  dat_26758 = 0;
label_df18:
  if (!v13)
    return;
  v2 = 0x14;
  v6 = (int *)0x1e524;
  sigemptyset((void *)0x26780);
  v11 = (int *)0x1e524;
  while( true ) {
    sigaction(v2,NULL,&v10);
    if (v10 != (void *)0x1)
      sigaddset((void *)0x26780,v2);
    if (v11 == (int *)0x1e550) break;
    v2 = *v11;
    v11 = &v11[1];
  }
  v30 = 0x10000000;
  v2 = 0x14;
  v14 = dat_26780;
  v15 = dat_26788;
  v16 = dat_26790;
  v17 = dat_26798;
  v18 = dat_267a0;
  v19 = dat_267a8;
  v20 = dat_267b0;
  v21 = dat_267b8;
  v22 = dat_267c0;
  v23 = dat_267c8;
  v24 = dat_267d0;
  v25 = dat_267d8;
  v26 = dat_267e0;
  v27 = dat_267e8;
  v28 = dat_267f0;
  v29 = dat_267f8;
  while( true ) {
    if (sigismember(0x26780,v2)) {
      v10 = sub_daa0;
      if (v2 == 0x14)
        v10 = sub_e1a0;
      sigaction(v2,&v10,NULL);
    }
    if (v6 == (int *)0x1e550) break;
    v2 = *v6;
    v6 = &v6[1];
  }
}

// Function: sub_e180 @ 0xe180
void sub_e180(unsigned long *a0)
{
  fwrite_unlocked((void *)a0[1],*a0,1,dat_26188); // tail-call
}

// Function: sub_e1a0 @ 0xe1a0
void sub_e1a0(void) // return-dupe
{
  if (dat_26774)
    return;
  dat_26770 += 1;
}

// Function: sub_e1c0 @ 0xe1c0
void sub_e1c0(void)
{
  char v1 [136];
  int v2;
  
  while ((dat_26774 || (dat_26770))) {
    if ((dat_26758) && (dat_26010 != 3)) {
      sub_e180(0x26020);
      sub_e180(0x26050);
      sub_e180(0x26030);
      dat_26010 = 3;
    }
    fflush_unlocked(stdout);
    sigprocmask(0,(void *)0x26780,v1);
    v2 = dat_26774;
    if (dat_26770) { // branch-flip
      dat_26770 -= 1;
      v2 = 0x13;
    }
    else {
      signal(dat_26774,0);
    }
    raise(v2);
    sigprocmask(2,v1,NULL);
  }
}

// Function: sub_e2e0 @ 0xe2e0
unsigned char * sub_e2e0(unsigned char *a0)
{
  bool v1;
  unsigned char *v2;
  unsigned char v3;
  unsigned char *v4; // rax
  unsigned char *v5; // rdx
  long v6; // rsi
  
  v3 = *a0;
  if (v3) {
    v6 = 0;
    v1 = 0;
    v2 = a0;
label_e314:
    do {
      v4 = v2;
      if (v3 != 0x20) { // branch-flip
        if (v3 != 0x22) {
          if ('#' <= (char)v3) { // branch-flip
            if (v3 != 0x5c) {
              v3 = v4[1];
              v2 = &v4[1];
              if (!v3) break;
              goto label_e314;
            }
          }
          else if (7 <= (unsigned char)(v3 - 7)) {
            if ((char)v3 <= '\x1f')
              v6 += 3;
            goto label_e308;
          }
        }
        v6 += 1;
      }
      else {
        v1 = 1;
      }
label_e308:
      v3 = v4[1];
      v2 = &v4[1];
    } while (v3);
    if ((v6) || (v1)) {
      v4 = (unsigned char *)sub_16a40(&v4[v6 + (4U - (long)a0)]);
      *v4 = 0x22;
      v3 = *a0;
      v2 = &v4[1];
      do {
        if (!v3) {
          v2[0] = 0x22;
          v2[1] = 0;
          return v4;
        }
        if ('\"' < (char)v3) {
          if (v3 == 0x5c) goto label_e416;
label_e484:
          *v2 = v3;
          v5 = &v2[1];
          goto label_e3d9;
        }
        if ((char)v3 <= '\x06') {
label_e3a8:
          if ('\x1f' < (char)v3) goto label_e484;
          *v2 = 0x5c;
          v5 = &v2[4];
          v2[1] = (v3 >> 6) + 0x30;
          v2[3] = (v3 & 7) + 0x30;
          v2[2] = ((char)v3 >> 3 & 7U) + 0x30;
          goto label_e3d9;
        }
        switch(v3) {
          case 7:
            *v2 = 0x5c;
            v5 = &v2[2];
            v2[1] = 0x61;
            break;
          case 8:
            v3 = 0x62;
            goto label_e416;
          case 9:
            v3 = 0x74;
            goto label_e416;
          case 10:
            *v2 = 0x5c;
            v5 = &v2[2];
            v2[1] = 0x6e;
            break;
          case 0xb:
            *v2 = 0x5c;
            v5 = &v2[2];
            v2[1] = 0x76;
            break;
          case 0xc:
            *v2 = 0x5c;
            v5 = &v2[2];
            v2[1] = 0x66;
            break;
          case 0xd:
            v3 = 0x72;
            goto label_e416;
          default:
            goto label_e3a8;
          case 0x22:
label_e416:
            *v2 = 0x5c;
            v5 = &v2[2];
            v2[1] = v3;
          
        }
label_e3d9:
        v3 = a0[1];
        a0 = &a0[1];
        v2 = v5;
      } while( true );
    }
  }
  return a0;
}

// Function: sub_e490 @ 0xe490
void sub_e490(unsigned long a0)
{
  error(0,*__errno_location(),"%s",a0); // tail-call
}

// Function: sub_e4c0 @ 0xe4c0
void sub_e4c0(char *a0,long a1,unsigned long a2,char *a3,char *a4)
{
  char *v1;
  long v10 [2]; // stack - 0x80
  char *v11;
  long v12;
  char *v13; // stack - 0x70
  char *v14; // stack - 0x68
  unsigned long v2;
  unsigned long *v3;
  long v4; // rax
  unsigned long *v5; // rax
  void *v6; // rax
  unsigned long *v7;
  long v8;
  unsigned long v9 [5];
  
  if (dat_26480) {
    v12 = 8;
    if (!a3)
      a3 = "";
    if (!a4)
      a4 = "";
    v8 = 0;
    v11 = a0;
    v10[0] = a1;
    v10[1] = a2;
    v13 = a3;
    v14 = a4;
    while( true ) {
      v4 = strlen(v11) + 1;
      *(long *)((long)v9 + v8) = v4;
      v12 += v4;
      if (v8 + 8 == 0x28) break;
      v11 = *(char **)((long)v10 + v8);
      v8 += 8;
    }
    v5 = (unsigned long *)sub_16a40(v12);
    v7 = &v5[1];
    v12 = 0;
    while( true ) {
      v2 = *(unsigned long *)((long)v9 + v12);
      v6 = memcpy(v7,a0,v2);
      v3 = dat_260a0;
      v7 = (unsigned long *)((long)v6 + v2);
      if (v12 + 8 == 0x28) break;
      a0 = *(char **)((long)v10 + v12);
      v12 += 8;
    }
    dat_260a0 = v5;
    *v3 = v5;
    *v5 = 0;
    return;
  }
  if (dat_2643a) {
    v1 = *(char **)((long)stdout + 0x28);
    if (*(char **)((long)stdout + 0x30) <= v1) // branch-flip
      __overflow(stdout,0x20);
    else {
      *(char **)((long)stdout + 0x28) = &v1[1];
      *v1 = 0x20;
    }
  }
  __printf_chk(1,dcgettext(NULL,a0,5),a1,a2,a3,a4); // tail-call
}

// Function: sub_e660 @ 0xe660
void sub_e660(void)
{
  sub_e4c0(); // tail-call
}

// Function: sub_e670 @ 0xe670
void sub_e670(void) // return-dupe
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

// Function: sub_e770 @ 0xe770
void sub_e770(unsigned long a0)
{
  int v1;
  
  v1 = *__errno_location();
  sub_e670();
  error(2,v1,"%s",a0);
}

// Function: sub_e7b0 @ 0xe7b0
void sub_e7b0(char *a0)
{
  sub_e670();
  error(2,0,"%s",dcgettext(NULL,a0,5));
}

// Function: sub_e7f0 @ 0xe7f0
void sub_e7f0(unsigned long a0)
{
  dat_26748 = a0;
}

// Function: sub_e800 @ 0xe800
void sub_e800(unsigned long a0,unsigned long a1,char a2)
{
  dat_26768 = a0;
  dat_26760 = a1;
  dat_26759 = a2;
  dat_26188 = 0;
}

// Function: sub_e830 @ 0xe830
void sub_e830(void) // return-dupe x2
{
  int v1; // eax
  unsigned long v10; // stack - 0x30
  void *v2; // rax
  void *v3; // stack - 0x58
  int v4; // stack - 0x60
  char *v5; // stack - 0x48
  int v6; // stack - 0x5c
  void *v7; // stack - 0x50
  char *v8; // stack - 0x40
  void *v9; // stack - 0x38
  
  if (dat_26188)
    return;
  v3 = (void *)sub_e2e0(dat_26768);
  v7 = (void *)sub_e2e0(dat_26760);
  v2 = (void *)sub_17020("diff%s %s %s",dat_26420,v3,v7);
  if (dat_26480) { // branch-flip
    v1 = fflush_unlocked(stdout);
    if (v1)
      sub_e770(dcgettext(NULL,"write failed",5)); // no-return
    v9 = v2;
    v5 = "/usr/bin/pr";
    v8 = "-h";
    v10 = 0;
    if (pipe(&v4))
      sub_e770("pipe"); // no-return
    dat_26740 = fork();
    if (dat_26740 < 0)
      sub_e770("fork"); // no-return
    if (!dat_26740) {
      close(v6);
      if (v4) {
        v1 = dup2(v4,0);
        if (v1 < 0)
          sub_e770("dup2"); // no-return
        close(v4);
      }
      execv("/usr/bin/pr",&v5);
      _exit((*__errno_location() == 2) + 0x7e); // no-return
    }
    close(v4);
    dat_26188 = fdopen(v6,"w");
    if (!dat_26188)
      sub_e770("fdopen"); // no-return
    sub_dd30(1);
  }
  else {
    dat_26188 = stdout;
    sub_dd30(0);
    if (dat_26759)
      __printf_chk(1,"%s\n",v2);
  }
  free(v2);
  if (dat_26580 != 2) { // branch-flip
    if (dat_26580 == 3)
      sub_8b80(0x261a0,&v3,1);
  }
  else {
    sub_8b80(0x261a0,&v3,0);
  }
  if (v3 != dat_26768)
    free(v3);
  if (v7 == dat_26760)
    return;
  free(v7);
}

// Function: sub_ead0 @ 0xead0
void sub_ead0(void)
{
  int v1; // eax
  unsigned char v2; // ah
  unsigned int v3;
  unsigned int v4; // stack - 0x14
  char *v5; // rsi
  
  if ((!dat_26188) || (dat_26188 == stdout)) {
    dat_26188 = NULL;
    return;
  }
  if (*dat_26188 & 0x20)
    sub_e7b0("write failed"); // no-return
  v1 = fclose(dat_26188);
  if (!v1) {
    v1 = waitpid(dat_26740,(int *)&v4,0);
    if (v1 < 0)
      sub_e770("waitpid"); // no-return
    if (v4 & 0x7f) { // branch-flip
      v5 = "subsidiary program \'%s\' failed";
      v3 = 0x7fffffff;
    }
    else {
      v2 = (unsigned char)(v4 >> 8);
      if (!v2) {
        dat_26188 = NULL;
        return;
      }
      v5 = "subsidiary program \'%s\' could not be invoked";
      v3 = (unsigned int)v2;
      if ((v2 != 0x7e) && (v5 = "subsidiary program \'%s\' failed (exit status %d)", v3 = (unsigned int)v2, v2 == 0x7f))
        v5 = "subsidiary program \'%s\' not found";
    }
    error(2,0,dcgettext(NULL,v5,5),"/usr/bin/pr",(unsigned long)v3);
    return;
  }
  sub_e770(dcgettext(NULL,"write failed",5)); // no-return
}

// Function: sub_ec00 @ 0xec00
unsigned long sub_ec00(unsigned char *a0,unsigned char *a1)
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
label_ec50:
  do {
    v8 = *v18;
    v10 = *v16;
    v12 = (unsigned long)v10;
    v14 = &v18[1];
    v13 = &v16[1];
    if (v8 == v10) goto label_ed3a;
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
                if (!(*(unsigned char *)(v7 + 1 + (unsigned long)v2 * 2) & 0x20)) goto label_ed08;
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
          else if (v5 != 2) goto label_edf0;
        }
      }
      else {
label_edf0:
        if ((v8 == 0x20) && (v10 == 9)) {
label_ee11:
          v15 = v19;
          do {
            if (v8 != 0x20) { // branch-flip
              if (v8 != 9) goto label_ee6c;
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
          goto label_ee11;
        }
      }
      goto label_ed08;
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
      if (v17) goto label_ef8f;
label_ef65:
      if ((unsigned char)v12 == v8) goto label_ed08;
    }
    else if (v17) {
label_ef8f:
      while (v8 = (unsigned char)v15, (char)v12 != '\n') {
        v12 = (unsigned long)*v13;
        v16 = &v13[1];
        if (!(*(unsigned char *)(v7 + 1 + v12 * 2) & 0x20)) {
          v12 = 0x20;
          goto label_ef65;
        }
        v13 = v16;
      }
      goto label_ed08;
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
      if (*(unsigned char *)(v7 + 1 + (unsigned long)v13[-2] * 2) & 0x20) goto label_ec50;
    }
  }
label_ed08:
  if (v4) {
    v7 = *(long *)__ctype_tolower_loc();
    v8 = *(unsigned char *)(v7 + (unsigned long)v8 * 4);
    v12 = (unsigned long)*(unsigned char *)(v7 + v12 * 4);
  }
  v10 = (unsigned char)v12;
  if (v8 != v10)
    return 1;
label_ed3a:
  if (v10 == 10)
    return 0;
  v7 = 1;
  if (v10 == 9)
    v7 = v3 - v19 % v3;
  v19 += v7;
  v16 = v13;
  v18 = v14;
  goto label_ec50;
label_ee6c:
  if ((char)v12 != ' ') { // branch-flip
    if ((char)v12 != '\t') goto label_efa0;
    v19 = (v3 + v19) - v19 % v3;
  }
  else {
    v19 += 1;
  }
  v12 = (unsigned long)*v13;
  v13 = &v13[1];
  goto label_ee6c;
label_efa0:
  if (v15 != v19)
    return 1;
  goto label_ed08;
}

// Function: sub_f060 @ 0xf060
unsigned long sub_f060(unsigned long a0)
{
  return a0;
}

// Function: sub_f070 @ 0xf070
unsigned long sub_f070(unsigned long a0)
{
  return a0;
}

// Function: sub_f080 @ 0xf080
void sub_f080(long a0,void *a1,void *a2)
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

// Function: sub_f0e0 @ 0xf0e0
void sub_f0e0(unsigned char *a0,unsigned char *a1,char *a2,unsigned long a3) // return-dupe x2
{
  unsigned char v1;
  unsigned char *v2;
  char *v3;
  void *v4;
  unsigned long v5; // rax
  unsigned long v6;
  long v7; // rcx
  unsigned long v8;
  long v9; // r14
  
  v6 = dat_26498;
  v4 = dat_26188;
  if (!dat_264a0) {
    v6 = (long)a1 - (long)a0;
    dat_264a0 = 0;
    while( true ) {
      if (!v6)
        return;
      v8 = 0x400;
      if (v6 <= 0x400)
        v8 = v6;
      v5 = fwrite_unlocked(a0,1,v8,dat_26188);
      if (v5 < v8) break;
      v6 -= v5;
      a0 = &a0[v5];
      sub_e1c0();
    }
    return;
  }
  v8 = 0;
  v9 = 1;
  if (a1 <= a0)
    return;
label_f1a0:
  v1 = *a0;
  a0 = &a0[1];
  if (v1 == 9) goto label_f23a;
  do {
    if (v1 != 0xd) { // branch-flip
      if (v1 != 8) { // branch-flip
        v8 += *(unsigned short *)(*(long *)__ctype_b_loc() + (unsigned long)v1 * 2) >> 0xe & 1;
        v2 = *(unsigned char **)((long)v4 + 0x28);
        if (*(unsigned char **)((long)v4 + 0x30) <= v2) { // branch-flip
label_f310:
          __overflow(v4,(unsigned int)v1);
        }
        else {
          *(unsigned char **)((long)v4 + 0x28) = &v2[1];
          *v2 = v1;
        }
      }
      else {
        if (!v8) goto label_f300;
        v3 = *(char **)((long)v4 + 0x28);
        v8 -= 1;
        if (*(char **)((long)v4 + 0x30) <= v3) goto label_f310;
        *(char **)((long)v4 + 0x28) = &v3[1];
        *v3 = 8;
      }
    }
    else {
      v3 = *(char **)((long)v4 + 0x28);
      if (*(char **)((long)v4 + 0x30) <= v3) // branch-flip
        __overflow(v4,0xd);
      else {
        *(char **)((long)v4 + 0x28) = &v3[1];
        *v3 = 0xd;
      }
      if ((a2) && (a0 < a1)) {
        v8 = 0;
        if (*a0 == 10) goto label_f213;
        __fprintf_chk(v4,1,a2,a3);
        goto label_f213;
      }
label_f300:
      v8 = 0;
    }
    while( true ) {
      if (a1 == a0)
        return;
label_f213:
      v9 += 1;
      if (v9 != 0x400) goto label_f1a0;
      sub_e1c0();
      v1 = *a0;
      a0 = &a0[1];
      v9 = 0;
      if (v1 != 9) break;
label_f23a:
      v7 = v6 - v8 % v6;
      v8 += v7;
      do {
        v3 = *(char **)((long)v4 + 0x28);
        if (*(char **)((long)v4 + 0x30) <= v3) // branch-flip
          __overflow(v4,0x20);
        else {
          *(char **)((long)v4 + 0x28) = &v3[1];
          *v3 = 0x20;
        }
        v7 -= 1;
      } while (v7);
    }
  } while( true );
}

// Function: sub_f330 @ 0xf330
void sub_f330(char *a0,unsigned long *a1,char a2)
{
  char v1;
  char *v2;
  long v3;
  void *v4;
  char *v5;
  char *v6;
  char *v7; // r13
  
  v4 = dat_26188;
  v2 = (char *)*a1;
  v3 = a1[1];
  if (a0) { // branch-flip
    v7 = NULL;
    if (*a0) {
      v7 = "%s\t";
      if (!dat_26492)
        v7 = "%s ";
      v5 = a0;
      v6 = v7;
      if ((dat_26491) && (*v2 == '\n')) {
        v6 = "%s";
        v5 = &a0[*a0 == ' '];
      }
      __fprintf_chk(dat_26188,1,v6,v5);
    }
    if (a2) { // branch-flip
      sub_f0e0(v2,v3 - (unsigned long)(*(char *)(v3 + -1) == '\n'),v7,a0);
      v1 = *a0;
    }
    else {
      sub_f0e0(v2,v3,v7,a0);
      v1 = *a0;
    }
    if (!v1)
      return;
  }
  else if (a2) // branch-flip
    sub_f0e0(v2,v3 - (unsigned long)(*(char *)(v3 + -1) == '\n'),0,0);
  else {
    sub_f0e0(v2,v3,0,0);
  }
  if (*(char *)(v3 + -1) == '\n')
    return;
  if ((dat_26758) && (dat_26010 != 3)) {
    sub_e180(0x26020);
    sub_e180(0x26050);
    sub_e180(0x26030);
    dat_26010 = 3;
  }
  __fprintf_chk(v4,1,"\n\\ %s\n",dcgettext(NULL,"No newline at end of file",5)); // tail-call
}

// Function: sub_f500 @ 0xf500
void sub_f500(unsigned long a0,unsigned long a1)
{
  sub_f330(a0,a1,0); // tail-call
}

// Function: sub_f510 @ 0xf510
void sub_f510(int a0)
{
  if (a0 != 3)
    sub_e1c0();
  if ((dat_26758) && (dat_26010 != a0)) {
    sub_e180(0x26020);
    switch(a0) {
      case 0:
        sub_e180(0x26060);
        break;
      case 1:
        sub_e180(0x26070);
        break;
      case 2:
        sub_e180(0x26080);
        break;
      case 3:
        sub_e180(0x26050);
        break;
      case 4:
        sub_e180(0x26090);
        break;
      default:
        abort(); // no-return
      
    }
    sub_e180(0x26030);
    dat_26010 = a0;
    return;
  }
}

// Function: sub_f5d0 @ 0xf5d0
long sub_f5d0(long a0,long a1)
{
  return a1 + *(long *)(a0 + 0xe8) + 1;
}

// Function: sub_f5e0 @ 0xf5e0
void sub_f5e0(long a0,long a1,long a2,long *a3,long *a4)
{
  *a3 = *(long *)(a0 + 0xe8) + 1 + a1;
  *a4 = *(long *)(a0 + 0xe8) + 1 + a2;
}

// Function: sub_f610 @ 0xf610
void sub_f610(char a0,long a1,long a2,long a3)
{
  a2 = *(long *)(a1 + 0xe8) + 1 + a2;
  a3 = *(long *)(a1 + 0xe8) + 1 + a3;
  if (a2 < a3) {
    __fprintf_chk(dat_26188,1,"%ld%c%ld",a2,(unsigned long)(unsigned int)(int)a0); // tail-call
    return;
  }
  __fprintf_chk(dat_26188,1,"%ld",a3); // tail-call
}

// Function: sub_f670 @ 0xf670
unsigned char sub_f670(long *a0,long *a1,long *a2,long *a3,long *a4)
{
  long v1;
  unsigned char *v10;
  unsigned char v11;
  long v12;
  unsigned long v13;
  unsigned long v14; // r8
  long v15;
  unsigned char v16; // stack - 0x69
  int v17; // stack - 0x64
  long v18; // stack - 0x60
  long v19; // stack - 0x58
  long v2;
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
      v17 = 0;
      v6 = 1;
      v5 = 3 < dat_26558;
    }
    else {
      v17 = 0;
      v6 = 0;
      v5 = 0;
    }
  }
  else {
    v17 = -1;
    v5 = 0;
    v9 = (unsigned int)(dat_264e0 != 0);
    v6 = 0;
  }
  v19 = 0;
  v18 = 0;
  *a1 = a0[3];
  *a3 = a0[4];
  do {
    v12 = a0[3];
    v15 = a0[4];
    v1 = a0[2] + -1 + v12;
    v19 += a0[1];
    v18 += a0[2];
    v2 = v15 + -1 + a0[1];
    v11 = v12 <= v1 & (unsigned char)v9;
    if (v11) { // branch-flip
      do {
        v15 = *(long *)(v7 + 8 + v12 * 8);
        v4 = *(unsigned char **)(v7 + v12 * 8);
        v15 = v15 + -1 + (unsigned long)(*(char *)(v15 + -1) != '\n');
        v14 = v15 - (long)v4;
        v13 = v14;
        if ((v6) && (v3 = *v4, v3 != 10)) {
          v10 = v4;
          do {
            if (!(*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)v3 * 2) & 0x20)) {
              v13 = v15 - (long)v10;
              if (!v5)
                v13 = v14;
              goto label_f7b2;
            }
            v3 = v10[1];
            v10 = &v10[1];
          } while (v3 != 10);
          v13 = v15 - (long)v10;
        }
label_f7b2:
        if ((long)v17 != v13) { // branch-flip
          if (!dat_264e0) {
            v9 = 0;
            goto label_f7cb;
          }
          v9 = (unsigned int)~re_search(0x264c0,v4,v14 & 0xffffffff,0,v14,0) >> 0x1f;
        }
        else {
          v9 = (unsigned int)v11;
        }
        v12 += 1;
      } while ((v12 <= v1) && ((unsigned char)v9));
      v15 = a0[4];
      v16 = v15 <= v2 & (unsigned char)v9;
    }
    else {
      v16 = v15 <= v2 & (unsigned char)v9;
    }
    if (v16) {
      do {
        v12 = *(long *)(v8 + 8 + v15 * 8);
        v4 = *(unsigned char **)(v8 + v15 * 8);
        v12 = v12 + -1 + (unsigned long)(*(char *)(v12 + -1) != '\n');
        v14 = v12 - (long)v4;
        v13 = v14;
        if ((v6) && (v11 = *v4, v11 != 10)) {
          v10 = v4;
          do {
            if (!(*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)v11 * 2) & 0x20)) {
              v13 = v12 - (long)v10;
              if (!v5)
                v13 = v14;
              goto label_f935;
            }
            v11 = v10[1];
            v10 = &v10[1];
          } while (v11 != 10);
          v13 = v12 - (long)v10;
        }
label_f935:
        if ((long)v17 != v13) { // branch-flip
          if (!dat_264e0) {
            v9 = 0;
            break;
          }
          v9 = (unsigned int)~re_search(0x264c0,v4,v14 & 0xffffffff,0,v14,0) >> 0x1f;
        }
        else {
          v9 = (unsigned int)v16;
        }
        v15 += 1;
        if ((v15 > v2) || (!(char)v9)) break;
      } while( true );
    }
label_f7cb:
    a0 = (long *)*a0;
    if (!a0) {
      v11 = 0;
      *a2 = v1;
      *a4 = v2;
      if (!(char)v9)
        v11 = (v19 != 0) * '\x02' | v18 != 0;
      return v11;
    }
  } while( true );
}

// Function: sub_fa30 @ 0xfa30
undefined16 sub_fa30(char *a0,char *a1,char *a2)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char v3 [16];
  
  v1 = strlen(a0);
  v2 = strlen(a1);
  v3._0_8_ = (char *)sub_16a40(v1 + v2 + 1 + strlen(a2));
  __sprintf_chk(v3._0_8_,1,0xffffffffffffffff,"%s%s%s",a0,a1,a2);
  v3._8_8_ = 0xfa6a;
  return v3._0_16_;
}

// Function: sub_fab0 @ 0xfab0
void sub_fab0(unsigned long a0)
{
  memset((void *)sub_16a40(),0,a0);
}

// Function: sub_fad0 @ 0xfad0
void sub_fad0(unsigned long *a0)
{
  fflush_unlocked(stdout);
  for (; a0; a0 = (unsigned long *)*a0) {
    __fprintf_chk(stderr,1,"%3ld %3ld delete %ld insert %ld\n",a0[3],a0[4],a0[2],a0[1]);
  }
  fflush_unlocked(stderr); // tail-call
}

// Function: sub_fb40 @ 0xfb40
char * sub_fb40(char *a0)
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

// Function: sub_fba0 @ 0xfba0
void sub_fba0(char *a0)
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

// Function: sub_fbd0 @ 0xfbd0
void sub_fbd0(void)
{
  return;
}

// Function: sub_fbe0 @ 0xfbe0
void sub_fbe0(int a0)
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
    write(2,": mbuiter.h",2);
    write(2,v6,v2);
    v5[0] = 10;
    v4 = (char *)0x1;
  }
  else {
    v3 = __mempcpy_chk(v5,dat_26828,v1,0x1002);
    *(unsigned short *)v3 = s_1e6f2._0_2_;
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

// Function: sub_fd00 @ 0xfd00
unsigned long sub_fd00(unsigned long a0,int a1)
{
  if (!a1)
    return 0;
  sub_fbe0(0xb); // no-return
}

// Function: sub_fd20 @ 0xfd20
void sub_fd20(unsigned long a0)
{
  if ((int)a0)
    a0 = (unsigned long)(-(unsigned int)(dat_26820 == 0) & 0xb);
  sub_fbe0(a0); // no-return
}

// Function: sub_fd50 @ 0xfd50
unsigned long sub_fd50(void *a0) // return-dupe
{
  if (!a0)
    a0 = sub_fbd0;
  dat_26840 = a0;
  dat_26838 = dcgettext(NULL,"program error",5);
  dat_26830 = dcgettext(NULL,"stack overflow",5);
  dat_26828 = sub_10ea0();
  if (!sub_15390(sub_fd20,0x26860,0x10000)) {
    dat_26820 = sub_15310(sub_fd00);
    return 0;
  }
  *__errno_location() = 0x5f;
  return 0xffffffff;
}

// Function: sub_fe00 @ 0xfe00
void sub_fe00(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long *a3)
{
  (*(void *)*a3)(); // jump-as-call
}

// Function: sub_fe10 @ 0xfe10
void sub_fe10(void *a0)
{
  free(a0); // tail-call
}

// Function: sub_fe20 @ 0xfe20
unsigned int sub_fe20(void)
{
  int v1; // eax
  
  v1 = sub_12410();
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}

// Function: sub_fe40 @ 0xfe40
unsigned int sub_fe40(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = strcmp(a0,a1);
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}

// Function: sub_fe60 @ 0xfe60
void sub_fe60(void)
{
  sub_118a0(); // tail-call
}

// Function: sub_fe70 @ 0xfe70
unsigned long sub_fe70(char *a0,char *a1,unsigned int a2) // early-return
{
  int v1; // eax
  unsigned int v2; // eax
  unsigned long v3; // rax
  unsigned long v4; // rax
  char *v5; // rax
  char *v6;
  
  if (!(a2 & 8)) {
    if (!(a2 & 0x10))
      return strcmp(a0,a1); // tail-call
    return sub_12410(a0,a1); // tail-call
  }
  if (!(a2 & 0x10)) {
    v3 = strlen(a0);
    v1 = strncmp(a0,a1,v3);
    v4 = CONCAT44(dat_4,v1);
    if (!v1) {
      if ((int)a1[v3] == 0x2f)
        return 0;
      v4 = (unsigned long)(unsigned int)(int)a1[v3];
    }
    return v4;
  }
  v5 = (char *)sub_16ea0(a1);
  v6 = v5;
  while (v6 = strchr(v6,0x2f), v6) {
    *v6 = '\0';
    v2 = sub_12410(a0,v5);
    if ((int)v2 <= 0) {
      free(v5); // return-dupe
      return (unsigned long)v2;
    }
    *v6 = '/';
    v6 = &v6[1];
  }
  v2 = sub_12410(a0,v5);
  free(v5);
  return (unsigned long)v2;
}

// Function: sub_ff60 @ 0xff60
unsigned long sub_ff60(unsigned char *a0,unsigned long a1)
{
  bool v1;
  unsigned char *v10; // stack - 0x78
  unsigned long v11; // stack - 0x70
  char v12; // stack - 0x68
  unsigned int v2;
  unsigned long v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // stack - 0x84
  unsigned int v6 [9]; // stack - 0x64
  unsigned char *v7;
  unsigned long v8;
  char v9; // stack - 0x7c
  
  v8 = 0;
  v1 = 0;
  v5 = 0;
  v10 = a0;
  do {
    v9 = 0;
    if (v1) {
label_10033:
      v3 = __ctype_get_mb_cur_max();
      v7 = v10;
      v4 = sub_15cf0(v10,v3);
      v11 = sub_17200(v6,v7,v4,&v5);
      v2 = v6[0];
      v7 = v10;
      if (v11 != 0xffffffffffffffff) { // branch-flip
        if (v11 != 0xfffffffffffffffe) {
          if (!v11) {
            v11 = 1;
            if (*v10)
              __assert_fail("*iter->cur.ptr == \'\\0\'","mbuiter.h",0xab,"mbuiter_multi_next"); // no-return
            if (v6[0])
              __assert_fail("iter->cur.wc == 0","mbuiter.h",0xac,"mbuiter_multi_next"); // no-return
          }
          v12 = '\x01';
          if (!mbsinit(&v5)) goto label_ffe4;
          v1 = 0;
          if (v2) goto label_ffed;
          return v8 % a1; // return-dupe
        }
        v11 = strlen((char *)v10);
        v12 = '\0';
        v2 = v6[0];
      }
      else {
        v12 = '\0';
        v11 = 1;
      }
    }
    else {
      if (!(*(unsigned int *)((unsigned long)(*v10 >> 5) * 4 + 0x1e8a0) >> (*v10 & 0x1f) & 1)) {
        if (!mbsinit(&v5))
          __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
        v1 = 1;
        goto label_10033;
      }
      v11 = 1;
      v2 = (unsigned int)(char)*v10;
      v12 = '\x01';
      v6[0] = v2;
label_ffe4:
      if (!v2)
        return v8 % a1;
label_ffed:
      v7 = v10;
    }
    v9 = 1;
    if (v12) // branch-flip
      v2 = towlower(v2);
    else {
      v2 = (unsigned int)(char)*v7;
      v10 = v7;
    }
    v10 = &v10[v11];
    v8 = (long)(int)v2 + v8 * 0x1f;
  } while( true );
}

// Function: sub_10210 @ 0x10210
void sub_10210(long a0,unsigned long a1)
{
  unsigned long v1;
  unsigned long *v2; // rax
  
  v2 = (unsigned long *)sub_16a40(0x10);
  v1 = *(unsigned long *)(a0 + 8);
  v2[1] = a1;
  *v2 = v1;
  *(unsigned long **)(a0 + 8) = v2;
}

// Function: sub_10240 @ 0x10240
unsigned long sub_10240(char *a0,unsigned int a1) // return-dupe x2
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
          if (!(v1 + 0x85U & 0xfd)) goto label_102d0;
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
label_102d0:
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

// Function: sub_10310 @ 0x10310
void sub_10310(void)
{
  sub_16dc0(0x10); // tail-call
}

// Function: sub_10320 @ 0x10320
void sub_10320(unsigned long *a0)
{
  unsigned int *v1;
  long *v2;
  unsigned long *v3;
  unsigned long *v4;
  long v5; // rbx
  void *v6;
  long *v7;
  
  v2 = (long *)*a0;
  while (v7 = v2, v2) {
    while( true ) {
      v2 = (long *)*v7;
      if (!(int)v7[1]) break;
      if ((int)v7[1] == 1) {
        v6 = (void *)v7[2];
        if (1 <= v7[4]) {
          v5 = 0;
          do {
            while (v1 = (unsigned int *)((long)v6 + v5 * 0x48), !(*v1 & 0x8000000)) {
              v5 += 1;
              if (v7[4] <= v5) goto label_103a1;
            }
            v5 += 1;
            regfree(&v1[2]);
            v6 = (void *)v7[2];
          } while (v5 < v7[4]);
        }
label_103a1:
        free(v6);
      }
      free(v7);
      v7 = v2;
      if (!v2) goto label_103b6;
    }
    sub_11ad0(v7[2]);
    free(v7);
  }
label_103b6:
  v4 = (unsigned long *)a0[1];
  while (v4) {
    v3 = (unsigned long *)*v4;
    free((void *)v4[1]);
    free(v4);
    v4 = v3;
  }
  free(a0); // tail-call
}

// Function: sub_10410 @ 0x10410
unsigned long sub_10410(unsigned long a0,char *a1,unsigned int a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // rax
  char v2;
  char v3;
  char *v4;
  unsigned long v5; // r8
  char *v6;
  void *v7;
  
  v7 = sub_fe70;
  if (a2 & 0x10000000)
    v7 = dat_25fd0;
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

// Function: sub_104d0 @ 0x104d0
unsigned int sub_104d0(long *a0,char *a1)
{
  int v1;
  unsigned long *v10; // rbx
  long v2;
  long *v3;
  unsigned long v4;
  long v5; // rax
  char *v6; // rax
  char *v7;
  char *v8;
  unsigned int v9;
  
  a0 = (long *)*a0;
  if (!a0)
    return 0;
  v1 = (int)a0[1];
  v8 = NULL;
  while( true ) {
    if (v1) { // branch-flip
      v2 = a0[4];
      if (1 <= v2) {
        v10 = (unsigned long *)(a0[2] + 8);
        v5 = 0;
        do {
          if (*(unsigned int *)&v10[-1] & 0x8000000) { // branch-flip
            if (!regexec(v10,a1,0,0,0)) {
              v9 = 0; // return-dupe
              free(v8); // return-dupe
              return (*(unsigned int *)((long)a0 + 0xc) >> 0x1d ^ 1) & 1 ^ v9;
            }
          }
          else {
            v4 = *v10;
            if (sub_10410(v4,a1)) {
              v9 = 0;
              free(v8);
              return (*(unsigned int *)((long)a0 + 0xc) >> 0x1d ^ 1) & 1 ^ v9;
            }
          }
          v5 += 1;
          v10 = &v10[9];
        } while (v2 != v5);
      }
    }
    else {
      if (!v8)
        v8 = (char *)sub_16a40(strlen(a1) + 1);
      v9 = *(unsigned int *)((long)a0 + 0xc);
      v2 = a0[2];
      v7 = a1;
      while( true ) {
        strcpy(v8,v7);
        v5 = sub_11680(v2,v8);
        while( true ) {
          if (v5) {
            v9 = 0;
            free(v8);
            return (*(unsigned int *)((long)a0 + 0xc) >> 0x1d ^ 1) & 1 ^ v9;
          }
          if ((!(v9 & 8)) || (v6 = strrchr(v8,0x2f), !v6)) break;
          *v6 = '\0';
          v5 = sub_11680(v2,v8);
        }
        if ((v9 & 0x40000000) || (v7 = strchr(v7,0x2f), !v7)) break;
        v7 = &v7[1];
      }
    }
    v3 = (long *)*a0;
    if (!v3) break;
    v1 = *(int *)&v3[1];
    a0 = v3;
  }
  v9 = 1;
  free(v8);
  return (*(unsigned int *)((long)a0 + 0xc) >> 0x1d ^ 1) & 1 ^ v9;
  v9 = 0;
  free(v8);
  return (*(unsigned int *)((long)a0 + 0xc) >> 0x1d ^ 1) & 1 ^ v9;
}

// Function: sub_10680 @ 0x10680
void sub_10680(long *a0,char *a1,unsigned int a2) // return-dupe x2
{
  unsigned int *v1;
  void *v10;
  char *v11;
  char v12;
  void *v13; // rdx
  unsigned int *v14; // rbx
  unsigned long v15;
  unsigned int v16; // r12d
  long v17;
  char *v2;
  int v3; // eax
  long *v4;
  unsigned long v5; // rax
  void *v6; // rax
  char *v7; // rax
  long v8;
  char *v9;
  
  v4 = (long *)*a0;
  if ((!(a2 & 0x18000000)) || (!sub_10240(a1,a2))) {
    if (((!v4) || (*(int *)&v4[1])) || ((*(unsigned int *)((long)v4 + 0xc) ^ a2) & 0x60000018)) {
      v4 = (long *)sub_16dc0(0x28);
      v10 = sub_fe20;
      v13 = sub_ff60;
      *(unsigned int *)&v4[1] = 0;
      *(unsigned int *)((long)v4 + 0xc) = a2;
      if (!(a2 & 0x10)) {
        v10 = sub_fe40;
        v13 = sub_fe60;
      }
      v4[2] = sub_11910(0,0,v13,v10,sub_fe10);
      v17 = *a0;
      *a0 = (long)v4;
      *v4 = v17;
    }
    v7 = (char *)sub_16ea0(a1);
    if ((a2 & 0x10000002) != 0x10000000) {
label_1085b:
      v17 = v4[2];
      if (v7 == (char *)sub_11f70(v17,v7))
        return;
      free(v7); // tail-call
      return;
    }
    v9 = v7;
    v11 = v7;
    do {
      v12 = *v11;
      v2 = v9;
      while (v12 != '\\') {
        v9 = &v2[1];
        v11 = &v11[1];
        *v2 = v12;
        if (!v12) goto label_1085b;
        v12 = *v11;
        v2 = v9;
      }
      v9 = &v11[1];
      v12 = '\\';
      if (*v9) {
        v11 = &v11[1];
        v12 = *v9;
      }
      v9 = &v2[1];
      v11 = &v11[1];
      *v2 = v12;
    } while( true );
  }
  if ((!v4) || ((*(int *)&v4[1] != 1 || ((*(unsigned int *)((long)v4 + 0xc) ^ a2) & 0x20000000)))) {
    v4 = (long *)sub_16dc0(0x28);
    *(unsigned int *)&v4[1] = 1;
    *(unsigned int *)((long)v4 + 0xc) = a2;
    v17 = *a0;
    *a0 = (long)v4;
    *v4 = v17;
  }
  v17 = v4[4];
  v8 = v4[2];
  if (v17 == v4[3]) {
    v8 = sub_16ca0(v8,&v4[3],1,0xffffffffffffffff,0x48);
    v17 = v4[4];
    v4[2] = v8;
  }
  v4[4] = v17 + 1;
  v1 = (unsigned int *)(v8 + v17 * 0x48);
  *v1 = a2;
  if (!(a2 & 0x8000000)) {
    if (a2 & 0x4000000) {
      a1 = (char *)sub_16ea0(a1);
      v4 = (long *)sub_16a40(0x10);
      v17 = a0[1];
      v4[1] = (long)a1;
      *v4 = v17;
      a0[1] = (long)v4;
    }
    *(char **)&v1[2] = a1;
    return;
  }
  v16 = (int)a2 >> 3 & 2U | 9;
  if (a2 & 8) { // branch-flip
    v5 = strlen(a1);
    if (0 < (long)v5) {
      do {
        v15 = v5;
        if (a1[v5 - 1] != '/') goto label_10761;
        v5 -= 1;
      } while (v5);
      goto label_107b6;
    }
    v15 = v5;
    if (!v5) goto label_107b6;
label_10761:
    v6 = (void *)sub_16a70(v15 + 7);
    memcpy(v6,a1,v5);
    v14 = (unsigned int *)(v5 + (long)v6);
    *(unsigned short *)&v14[1] = 0x3f29;
    *v14 = 0x2a2e2f28;
    *(char *)((long)v14 + 6) = 0;
    v3 = regcomp(&v1[2],v6,v16);
    free(v6);
  }
  else {
    v3 = regcomp(&v1[2],a1,v16);
  }
  if (!v3)
    return;
  v17 = v4[4] + -1;
label_107b6:
  v4[4] = v17;
  return;
}

// Function: sub_109c0 @ 0x109c0
int sub_109c0(void *a0,long a1,void *a2,unsigned int a3,unsigned char a4,unsigned long a5)
{
  unsigned long v1;
  unsigned char *v10;
  int v11; // stack - 0x5c
  int *v12; // stack - 0x58
  int v13; // stack - 0x50
  int v2; // eax
  long v3;
  unsigned char *v4;
  unsigned long *v5; // rax
  long v6; // stack - 0x48
  unsigned char *v7; // rbp
  unsigned char *v8; // r14
  long v9; // r15
  
  v9 = 0;
  v3 = 0;
  v6 = 0;
  while( true ) {
    v2 = getc(a2);
    if (v2 == -1) break;
    if (v6 == v9)
      v3 = sub_16ca0(v3,&v6,1,0xffffffffffffffff,1);
    *(char *)(v3 + v9) = (char)v2;
    v9 += 1;
  }
  v11 = ferror(a2);
  v12 = __errno_location();
  v13 = 0;
  if (v11) {
    v11 = *v12;
    v13 = -(unsigned int)(v11 != 0);
  }
  v4 = (unsigned char *)sub_16ad0(v3,v9 + 1);
  v4[v9] = a4;
  if (v9) { // branch-flip
    v7 = &v4[(unsigned long)(v4[v9 + -1] != a4) + v9];
    v5 = (unsigned long *)sub_16a40(0x10);
    v1 = *(unsigned long *)(a1 + 8);
    v5[1] = v4;
    *v5 = v1;
    *(unsigned long **)(a1 + 8) = v5;
    if (v4 < v7) {
      v10 = v4;
      do {
        v8 = &v10[1];
        if (*v10 == a4) {
          v3 = *(long *)__ctype_b_loc();
          if (*(unsigned short *)(v3 + (unsigned long)a4 * 2) & 0x2000) { // branch-flip
            for (; v4 != v10; v10 = &v10[-1]) {
              if (!(*(unsigned char *)(v3 + 1 + (unsigned long)v10[-1] * 2) & 0x20)) goto label_10b7d;
            }
          }
          else {
label_10b7d:
            *v10 = 0;
            (*a0)(a1,v4,a3,a5);
          }
          v4 = v8;
        }
        v10 = v8;
      } while (v7 != v8);
    }
  }
  else {
    v5 = (unsigned long *)sub_16a40(0x10);
    v1 = *(unsigned long *)(a1 + 8);
    v5[1] = v4;
    *v5 = v1;
    *(unsigned long **)(a1 + 8) = v5;
  }
  *v12 = v11;
  return v13;
}

// Function: sub_10bd0 @ 0x10bd0
unsigned int sub_10bd0(unsigned long a0,unsigned long a1,char *a2,unsigned long a3,char a4) // return-dupe
{
  unsigned int v1; // eax
  void *v2; // rax
  unsigned long v3; // stack - 0x20
  
  v3 = a0;
  if ((*a2 == '-') && (!a2[1])) {
    v1 = sub_109c0(sub_fe00,a1,stdin,a3,(int)a4,&v3);
    return v1;
  }
  v2 = fopen(a2,"re");
  if (!v2)
    return 0xffffffff;
  v1 = sub_109c0(sub_fe00,a1,v2,a3 & 0xffffffff,(int)a4,&v3);
  if (fclose(v2))
    return 0xffffffff;
  return v1;
}

// Function: sub_10c90 @ 0x10c90
void sub_10c90(long a0)
{
  unsigned int v1; // eax
  
  v1 = *(unsigned int *)(a0 + 0x18) & 0xf000;
  if (v1 == 0x8000) {
    if (!*(long *)(a0 + 0x30)) {
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

// Function: sub_10d80 @ 0x10d80
void sub_10d80(void)
{
  if (sub_10da0())
    return;
  sub_16ec0(); // no-return
}

// Function: sub_10da0 @ 0x10da0
void * sub_10da0(void *a0,char *a1,unsigned long *a2)
{
  unsigned long v1;
  long v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  void *v5; // rax
  char *v6; // rax
  unsigned long v7; // rbx
  char v8; // stack - 0x41
  
  v2 = sub_fb40(a0);
  v3 = sub_fba0(v2);
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

// Function: sub_10ea0 @ 0x10ea0
unsigned long sub_10ea0(void)
{
  return __progname;
}

// Function: sub_10eb0 @ 0x10eb0
void sub_10eb0(void *a0)
{
  clock_gettime(0,a0); // tail-call
}

// Function: sub_10ec0 @ 0x10ec0
undefined16 sub_10ec0(void)
{
  char v1 [16]; // stack - 0x28
  
  clock_gettime(0,v1);
  return v1._0_16_;
}

// Function: sub_10f10 @ 0x10f10
bool sub_10f10(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_14cb0(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_10fa0 @ 0x10fa0
unsigned long sub_10fa0(unsigned long a0,unsigned long a1)
{
  return (a0 >> 3 | a0 << 0x3d) % a1;
}

// Function: sub_10fc0 @ 0x10fc0
bool sub_10fc0(long a0,long a1)
{
  return a1 == a0;
}

// Function: sub_10fd0 @ 0x10fd0
unsigned long sub_10fd0(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0x1e860)
    return 1;
  v1 = v2[2];
  if ((((dat_1e874 < v1) && (v1 < dat_1e878)) && (dat_1e87c < v2[3])) && (dat_1e890 <= *v2)) {
    v3 = *v2 + dat_1e874;
    if (((v3 < v2[1]) && (v2[1] <= dat_1e880)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0x1e860;
  return 0;
}

// Function: sub_11060 @ 0x11060
unsigned long sub_11060(long *a0,long a1,unsigned long *a2,char a3) // early-return
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

// Function: sub_11170 @ 0x11170
unsigned long sub_11170(float a0,unsigned long a1,char a2) // ternary
{
  char v1 [16];
  unsigned long v2; // rax
  unsigned long v3; // rcx
  unsigned long v4; // rsi
  long v5; // rdi
  unsigned long v6;
  
  if (!a2) {
    a0 = (float)a1 / a0;
    if (dat_1e884 <= a0) {
      v2 = 0;
      return v2;
    }
    a1 = (a0 < dat_1e888) ? (unsigned long)a0 : (long)(a0 - dat_1e888) ^ 0x8000000000000000;
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
          if (v6 <= v4) goto label_11245;
          v5 += 8;
        } while (v6 % v3);
      }
    }
    else {
      v3 = 3;
label_11245:
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

// Function: sub_112b0 @ 0x112b0
unsigned long sub_112b0(long *a0,unsigned long *a1,char a2) // return-dupe x2
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
label_112e6:
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
        goto label_112e6;
      }
    }
    v6 = &v6[2];
    if ((long *)a1[1] <= v6)
      return 1;
  } while( true );
}

// Function: sub_11420 @ 0x11420
unsigned long sub_11420(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}

// Function: sub_11430 @ 0x11430
unsigned long sub_11430(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}

// Function: sub_11440 @ 0x11440
unsigned long sub_11440(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}

// Function: sub_11450 @ 0x11450
unsigned long sub_11450(unsigned long *a0)
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

// Function: sub_114b0 @ 0x114b0
bool sub_114b0(unsigned long *a0)
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
label_114d0:
      v2 = &v2[2];
    }
    v1 = v2[1];
    v4 += 1;
    v3 += 1;
    if (!v1) goto label_114d0;
    do {
      v1 = *(long *)(v1 + 8);
      v3 += 1;
    } while (v1);
    v2 = &v2[2];
  } while( true );
}

// Function: sub_11520 @ 0x11520
void sub_11520(unsigned long *a0,void *a1)
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
      if ((long *)a0[1] <= v4) goto label_1158d;
    }
    v5 = 1;
    for (v3 = v4[1]; v3; v3 = *(long *)(v3 + 8)) {
      v5 += 1;
    }
    if (v6 < v5)
      v6 = v5;
  }
label_1158d:
  __fprintf_chk(a1,1,"# entries:         %lu\n",a0[4]);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v2);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v6); // tail-call
  return;
}

// Function: sub_11680 @ 0x11680
long sub_11680(long *a0,long a1) // return-dupe
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

// Function: sub_116f0 @ 0x116f0
long sub_116f0(unsigned long *a0)
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

// Function: sub_11740 @ 0x11740
long sub_11740(long *a0,long a1)
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

// Function: sub_117d0 @ 0x117d0
void sub_117d0(unsigned long *a0,long a1,unsigned long a2)
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

// Function: sub_11820 @ 0x11820
long sub_11820(unsigned long *a0,void *a1,unsigned long a2)
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

// Function: sub_118a0 @ 0x118a0
unsigned long sub_118a0(unsigned char *a0,unsigned long a1)
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

// Function: sub_118e0 @ 0x118e0
void sub_118e0(unsigned long *a0)
{
  *(char *)&a0[2] = 0;
  *a0 = 0x3f80000000000000;
  a0[1] = 0x3fb4fdf43f4ccccd;
}

// Function: sub_11910 @ 0x11910
unsigned long * sub_11910(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_10fa0;
  if (!a3)
    a3 = sub_10fc0;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = 0x1e860;
  v1[5] = a1;
  if (sub_10fd0(v1)) {
    v2 = sub_11170(*(unsigned int *)(a1 + 8),a0,*(char *)(a1 + 0x10));
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

// Function: sub_11a20 @ 0x11a20
void sub_11a20(unsigned long *a0)
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
        if ((long *)a0[1] <= v5) goto label_11abb;
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
label_11abb:
  a0[3] = 0;
  a0[4] = 0;
  return;
}

// Function: sub_11ad0 @ 0x11ad0
void sub_11ad0(unsigned long *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = (long *)*a0;
  v3 = (long *)a0[1];
  if ((a0[8]) && (a0[4])) {
    if (v3 <= v4) goto label_11b7c;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_11b3f;
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
label_11b3f:
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
label_11b7c:
  v2 = (void *)a0[9];
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free((void *)*a0);
  free(a0); // tail-call
}

// Function: sub_11bc0 @ 0x11bc0
unsigned int sub_11bc0(unsigned long *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_11170(*(unsigned int *)(a0[5] + 8),a1,*(char *)(a0[5] + 0x10));
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
  v1 = sub_112b0(&v2,a0,0);
  if (!(char)v1) {
    a0[9] = v11;
    if ((sub_112b0(a0,&v2,1)) && (sub_112b0(a0,&v2,0))) {
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

// Function: sub_11d40 @ 0x11d40
long * sub_11d40(long a0,long a1,long *a2) // return-dupe, ternary
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
  v2 = (long)sub_11060(a0,a1,&v3,0);
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
    if ((long)v2 > -1) goto label_11dcf;
label_11e56:
    v5 = (float)v2;
  }
  else {
    v2 = *(long **)(a0 + 0x10);
    v6 = (float)v4;
    if (0 > (long)v2) goto label_11e56;
label_11dcf:
    v5 = (float)(long)v2;
  }
  if (*(float *)(*(long *)(a0 + 0x28) + 8) * v5 < v6) {
    sub_10fd0(a0);
    v1 = *(long *)(a0 + 0x28);
    if (v5 * *(float *)(v1 + 8) < v6) {
      v5 = v5 * *(float *)(v1 + 0xc);
      if (!*(char *)(v1 + 0x10))
        v5 = v5 * *(float *)(v1 + 8);
      if (dat_1e884 <= v5) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v4 = (dat_1e888 <= v5) ? (long)(v5 - dat_1e888) ^ 0x8000000000000000 : (unsigned long)v5;
      if (!sub_11bc0(a0,v4)) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v2 = (long)sub_11060(a0,a1,&v3,0);
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

// Function: sub_11f70 @ 0x11f70
unsigned long sub_11f70(unsigned long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_11d40(a0,a1,&v3);
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

// Function: sub_11fd0 @ 0x11fd0
long sub_11fd0(long a0,unsigned long a1) // ternary x2
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
  
  v5 = sub_11060(a0,a1,&v7,1);
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
      sub_10fd0(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_1e888 <= v8) ? (long)(v8 - dat_1e888) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_11bc0(a0,v6)) {
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

// Function: sub_12160 @ 0x12160
void sub_12160(void)
{
  sub_11fd0(); // tail-call
}

// Function: sub_12170 @ 0x12170
long sub_12170(unsigned long a0)
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

// Function: sub_121c0 @ 0x121c0
void sub_121c0(unsigned long a0)
{
  if (a0 & 0xf)
    abort(); // no-return
  if (!(a0 & 0x10))
    return;
  free((void *)(a0 - *(unsigned char *)(a0 - 1))); // tail-call
}

// Function: sub_121f0 @ 0x121f0
int sub_121f0(int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (0 <= v1)
    return v1;
  return (unsigned long)(iswcntrl(a0) == 0);
}

// Function: sub_12220 @ 0x12220
void sub_12220(long *a0,long *a1) // return-dupe
{
  long v1;
  long *v2;
  
  v2 = (long *)*a1;
  if (v2 == &a1[3])
    v2 = memcpy(&a0[3],&a1[3],a1[1]);
  v1 = a1[1];
  *a0 = (long)v2;
  a0[1] = v1;
  v1 = a1[2];
  *(char *)&a0[2] = (char)v1;
  if (!(char)v1)
    return;
  *(unsigned int *)((long)a0 + 0x14) = *(unsigned int *)((long)a1 + 0x14);
}

// Function: sub_12280 @ 0x12280
unsigned int sub_12280(unsigned char a0)
{
  return *(unsigned int *)((unsigned long)(a0 >> 5) * 4 + 0x1e8a0) >> (a0 & 0x1f) & 1;
}

// Function: sub_122a0 @ 0x122a0
void sub_122a0(char *a0)
{
  unsigned char v1;
  unsigned long v2;
  unsigned long v3; // rax
  long v4; // rax
  
  if (!*a0) {
    v1 = **(unsigned char **)&a0[0x10];
    if (*(unsigned int *)((unsigned long)(v1 >> 5) * 4 + 0x1e8a0) >> (v1 & 0x1f) & 1) {
      a0[0x18] = '\x01';
      a0[0x19] = '\0';
      a0[0x1a] = '\0';
      a0[0x1b] = '\0';
      a0[0x1c] = '\0';
      a0[0x1d] = '\0';
      a0[0x1e] = '\0';
      a0[0x1f] = '\0';
      v1 = **(unsigned char **)&a0[0x10];
      a0[0x20] = '\x01';
      *(int *)&a0[0x24] = (int)(char)v1;
      goto label_122e1;
    }
    if (!mbsinit(&a0[4]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    *a0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v2 = *(unsigned long *)&a0[0x10];
  v4 = sub_17200(&a0[0x24],v2,sub_15cf0(v2,v3),&a0[4]);
  *(long *)&a0[0x18] = v4;
  if (v4 == -1) {
    a0[0x18] = '\x01';
    a0[0x19] = '\0';
    a0[0x1a] = '\0';
    a0[0x1b] = '\0';
    a0[0x1c] = '\0';
    a0[0x1d] = '\0';
    a0[0x1e] = '\0';
    a0[0x1f] = '\0';
    a0[0x20] = '\0';
    a0[0xc] = '\x01';
    return;
  }
  if (v4 == -2) {
    v3 = strlen(*(char **)&a0[0x10]);
    a0[0x20] = '\0';
    *(unsigned long *)&a0[0x18] = v3;
    a0[0xc] = '\x01';
    return;
  }
  if (!v4) {
    a0[0x18] = '\x01';
    a0[0x19] = '\0';
    a0[0x1a] = '\0';
    a0[0x1b] = '\0';
    a0[0x1c] = '\0';
    a0[0x1d] = '\0';
    a0[0x1e] = '\0';
    a0[0x1f] = '\0';
    if (**(char **)&a0[0x10])
      __assert_fail("*iter->cur.ptr == \'\\0\'","mbuiter.h",0xab,"mbuiter_multi_next"); // no-return
    if (*(int *)&a0[0x24])
      __assert_fail("iter->cur.wc == 0","mbuiter.h",0xac,"mbuiter_multi_next"); // no-return
  }
  a0[0x20] = '\x01';
  if (mbsinit(&a0[4])) {
    *a0 = '\0';
    a0[0xc] = '\x01';
    return;
  }
label_122e1:
  a0[0xc] = '\x01';
  return;
}

// Function: sub_12410 @ 0x12410
unsigned long sub_12410(unsigned char *a0,unsigned char *a1) // early-return
{
  unsigned char v1;
  char v10; // stack - 0xac
  unsigned char *v11; // stack - 0xa8
  unsigned long v12; // stack - 0xa0
  char v13; // stack - 0x98
  unsigned int v14; // stack - 0x94
  unsigned long v15; // stack - 0x74
  char v16; // stack - 0x6c
  unsigned char *v17; // stack - 0x68
  unsigned long v18; // stack - 0x60
  char v19; // stack - 0x58
  long v2;
  unsigned int v20; // stack - 0x54
  unsigned int v3; // eax
  unsigned int v4; // eax
  int v5; // eax
  unsigned char v6;
  char v7 [4]; // stack - 0xb8
  char v8 [4]; // stack - 0x78
  unsigned long v9; // stack - 0xb4
  
  if (a1 == a0)
    return 0;
  if (__ctype_get_mb_cur_max() <= 1) {
    v2 = *(long *)__ctype_b_loc();
    while( true ) {
      v6 = *a0;
      if (*(unsigned char *)(v2 + 1 + (unsigned long)v6 * 2) & 1)
        v6 = *(unsigned char *)(*(long *)__ctype_tolower_loc() + (unsigned long)v6 * 4);
      v1 = *a1;
      if (*(unsigned char *)(v2 + 1 + (unsigned long)v1 * 2) & 1)
        v1 = *(unsigned char *)(*(long *)__ctype_tolower_loc() + (unsigned long)v1 * 4);
      if (!v6) break;
      a0 = &a0[1];
      a1 = &a1[1];
      if (v6 != v1)
        return (unsigned long)((unsigned int)v6 - (unsigned int)v1);
    }
    return (unsigned long)-(unsigned int)v1;
  }
  v7[0] = 0;
  v9 = 0;
  v8[0] = 0;
  v15 = 0;
  v11 = a0;
  v17 = a1;
  while( true ) {
    v10 = '\0';
    v16 = '\0';
    sub_122a0(v7);
    if ((v13) && (!v14)) break;
    if (!v16)
      sub_122a0(v8);
    if (v19) { // branch-flip
      if (!v20) {
        if (!v10) goto label_125f7;
        goto label_125ff;
      }
      if (!v13)
        return 1;
      v3 = towlower(v14);
      v4 = towlower(v20);
      v3 -= v4;
      if (v3)
        return (unsigned long)v3;
    }
    else {
      if (v13)
        return 0xffffffff;
      if (v12 != v18) {
        if (v18 <= v12) {
          if (0 <= memcmp(v11,v17,v18))
            return 1;
          return 0xffffffff;
        }
        if (memcmp(v11,v17,v12) <= 0)
          return 0xffffffff;
        return 1;
      }
      v5 = memcmp(v11,v17,v12);
      if (v5)
        return CONCAT44(dat_4,v5);
    }
    v11 = &v11[v12];
    v17 = &v17[v18];
  }
  if (v10) goto label_1261a;
label_125f7:
  sub_122a0(v7);
label_125ff:
  if ((!v13) || (v14))
    return 1;
label_1261a:
  if (!v16)
    sub_122a0(v8);
  if ((v19) && (!v20))
    return 0;
  return 0xffffffff;
}

// Function: sub_12690 @ 0x12690
long sub_12690(char *a0,long a1,char *a2,unsigned int *a3,char a4,unsigned int a5,int a6,unsigned long a7,int a8) // ternary
{
  int v1;
  unsigned char v10;
  int v11; // ecx
  long v12;
  char v13;
  unsigned int v14;
  unsigned long v15;
  unsigned long v16; // rdx
  unsigned long v17;
  char v18;
  long v19;
  unsigned int v2;
  unsigned long v20; // stack - 0x450
  unsigned long v21; // stack - 0x448
  char v22 [1010];
  unsigned long v23; // stack - 0x488
  int v24;
  unsigned long v25;
  int v26;
  unsigned int v27;
  char *v28;
  char v29;
  long v3;
  unsigned int v30;
  int v31;
  char *v32;
  unsigned int v33;
  long v34; // r13
  char *v35;
  unsigned int v36;
  bool v37;
  unsigned long v38; // stack - 0x4c0
  unsigned long v39; // stack - 0x4b8
  unsigned long v4;
  unsigned long v40; // stack - 0x480
  unsigned long v41; // stack - 0x478
  long v42; // stack - 0x470
  unsigned long v43; // stack - 0x468
  unsigned long v44; // stack - 0x460
  unsigned long v45; // stack - 0x458
  char v5;
  bool v6;
  unsigned int v7; // eax
  int *v8; // rax
  long *v9; // rax
  
  v4 = a7;
  v25 = (unsigned long)a6;
  v8 = __errno_location();
  v1 = *v8;
  v36 = a3[2];
  v32 = *(char **)&a3[0xc];
  if (!v32)
    v32 = "";
  if (0xd <= (int)v36) // branch-flip
    v36 -= 0xc;
  else if (!v36)
    v36 = 0xc;
  v5 = *a2;
  v34 = 0;
  if (!v5) {
label_12794:
    if ((a0) && (a1))
      *a0 = '\0';
    *v8 = v1;
    return v34;
  }
label_12732:
  if (v5 != '%') {
    v15 = 0;
    if (0 <= (int)v25)
      v15 = v25;
    v17 = 1;
    if (v15)
      v17 = v15;
    if ((unsigned long)(a1 - v34) <= v17) goto label_127b8;
    if (a0) {
      v35 = a0;
      if (1 < (int)v25) {
        v35 = &a0[v15 - 1];
        memset(a0,0x20,v15 - 1);
      }
      a0 = &v35[1];
      *v35 = *a2;
    }
    v34 += v17;
    goto label_1277b;
  }
  v33 = 0;
  v14 = 0;
  v5 = a4;
  while( true ) {
    v35 = &a2[1];
    v30 = (unsigned int)*v35;
    a2 = &a2[1];
    v10 = *v35 - 0x23;
    if (0x3d <= v10) break;
    v15 = 1L << (v10 & 0x3f);
    if (v15 & 0x1000000000002500) // branch-flip
      v33 = v30;
    else if (v10 != 0x3b) { // branch-flip
      if (!(v15 & 1)) break;
      v14 = (unsigned int)v15 & 1;
    }
    else {
      v5 = '\x01';
    }
  }
  v27 = v30;
  if (v30 - 0x30 <= 9) {
    v25 = 0;
    do {
      v19 = (long)(int)v25 * 10;
      v24 = (int)v19;
      if ((v24 != v19) || (v25 = (unsigned long)(unsigned int)(v24 + *a2 + -0x30), SCARRY4(v24,*a2 + -0x30)))
        v25 = 0x7fffffff;
      v30 = (unsigned int)a2[1];
      a2 = &a2[1];
      v27 = v30;
    } while (v30 - 0x30 <= 9);
  }
  v13 = (char)v30;
  v29 = (char)v14;
  v24 = (int)v25;
  v14 = v36;
  if ((v13 == 'E') || (v13 == 'O')) {
    v13 = a2[1];
    a2 = &a2[1];
    v35 = a2;
    switch(v13) {
      case 0:
        goto label_12d4e;
      default:
label_12900:
        goto label_12903;
      case 0x25:
        if (!v27) goto label_13ac5;
        v25 = 1;
        goto label_1292e;
      case 0x3a:
        goto label_12ee0;
      case 0x41:
        if (!v27) goto label_12ead;
        break;
      case 0x42:
        if (v27 != 0x45) goto label_12e8e;
        break;
      case 0x43:
        if (v27 != 0x45) goto label_1393a;
        goto label_12bd1;
      case 0x44:
        if (!v27) goto label_13921;
        break;
      case 0x46:
        if (!v27) goto label_1346e;
        break;
      case 0x47:
      case 0x56:
      case 0x67:
        if (v27 != 0x45) goto label_12a3a;
        goto label_12903;
      case 0x48:
        if (v27 != 0x45) goto label_13906;
        break;
      case 0x49:
        if (v27 != 0x45) goto label_138ed;
        break;
      case 0x4d:
        if (v27 != 0x45) goto label_138d0;
        break;
      case 0x4e:
        if (v27 != 0x45) goto label_1372b;
        break;
      case 0x50:
        goto label_1371e;
      case 0x52:
        goto label_13705;
      case 0x53:
        if (v27 != 0x45) goto label_136eb;
        break;
      case 0x54:
        goto label_136d2;
      case 0x55:
        if (v27 != 0x45) goto label_13699;
        break;
      case 0x57:
        if (v27 != 0x45) goto label_1362f;
        break;
      case 0x58:
        if (v27 != 0x4f) goto label_1361e;
        break;
      case 0x59:
        if (v27 == 0x45) goto label_12bd1;
        if (v27 != 0x4f) goto label_135bf;
        break;
      case 0x5a:
        goto label_13362;
      case 0x61:
        if (!v27) goto label_12fd4;
        break;
      case 0x62:
      case 0x68:
        goto label_128e0;
      case 99:
        if (v27 != 0x4f) goto label_139b1;
        break;
      case 100:
        if (v27 != 0x45) goto label_12e74;
        break;
      case 0x65:
        if (v27 != 0x45) goto label_12e60;
        break;
      case 0x6a:
        if (v27 != 0x45) goto label_12e2e;
        break;
      case 0x6b:
        if (v27 != 0x45) goto label_12dee;
        break;
      case 0x6c:
        if (v27 != 0x45) goto label_130cd;
        break;
      case 0x6d:
        if (v27 != 0x45) goto label_1309b;
        break;
      case 0x6e:
        goto label_1301a;
      case 0x70:
        goto label_12ff3;
      case 0x71:
        v13 = 'q';
        if (v27 != 0x4f) goto label_130de;
        goto label_12bd1;
      case 0x72:
        goto label_12d25;
      case 0x73:
        goto label_13167;
      case 0x74:
        goto label_12d6e;
      case 0x75:
        goto label_13a73;
      case 0x77:
        if (v27 != 0x45) goto label_13a56;
        break;
      case 0x78:
        if (v27 != 0x4f) goto label_13a45;
        break;
      case 0x79:
        if (v27 != 0x45) goto label_139c2;
        goto label_12bd1;
      case 0x7a:
        goto label_13463;
      
    }
label_12910:
    a2 = v35;
    do {
      a2 = &a2[-1];
    } while (*a2 != '%');
    v25 = (unsigned long)(((int)v35 + 1) - (int)a2);
label_1292e:
    v17 = v25;
    if ((0 <= v24) && (v33 != 0x2d)) {
      v15 = (unsigned long)v24;
      if (v25 <= v15)
        v17 = v15;
    }
    else {
      v15 = 0;
    }
    if ((unsigned long)(a1 - v34) <= v17) goto label_127b8;
    if (a0) {
      v28 = a0;
      if (v25 < v15) {
        v15 -= v25;
        v28 = &a0[v15];
        if ((v33 != 0x30) && (v33 != 0x2b))
          memset(a0,0x20,v15);
        else {
          memset(a0,0x30,v15);
        }
      }
      if (v5) { // branch-flip
        v9 = __ctype_toupper_loc();
        v15 = v25;
        do {
          v15 -= 1;
          v28[v15] = (char)*(unsigned int *)(*v9 + (unsigned long)(unsigned char)a2[v15] * 4);
        } while (v15);
      }
      else {
        memcpy(v28,a2,v25);
      }
      a0 = &v28[v25];
    }
    goto label_12d07;
  }
  switch(v30 & 0xff) {
    case 0:
label_12d4e:
      v13 = a2[-1];
      a2 = &a2[-1];
label_12d56:
      v35 = a2;
      if (v13 != '%') goto label_12910;
      v25 = 1;
      goto label_1292e;
    default:
      goto label_12903;
    case 0x25:
label_13ac5:
      v5 = '%';
      if ((v33 != 0x2d) && (0 <= v24)) {
        v15 = (unsigned long)v24;
        v25 = 1;
        if (v15)
          v25 = v15;
        if ((unsigned long)(a1 - v34) <= v25) goto label_127b8;
        if (a0) {
          if (2 <= v24) {
            v15 -= 1;
            v35 = &a0[v15];
            if ((v33 != 0x30) && (v33 != 0x2b)) {
              memset(a0,0x20,v15);
              v5 = *a2;
              a0 = v35;
            }
            else {
              memset(a0,0x30,v15);
              v5 = *a2;
              a0 = v35;
            }
          }
label_13b31:
          *a0 = v5;
          a0 = &a0[1];
        }
      }
      else {
        if ((unsigned long)(a1 - v34) <= 1) goto label_127b8;
        v25 = 1;
        if (a0) goto label_13b31;
      }
      goto label_12de1;
    case 0x3a:
      v27 = 0;
label_12ee0:
      v35 = &a2[1];
      v25 = 1;
      v29 = a2[1];
      while (v29 == ':') {
        v25 += 1;
        v35 = &a2[v25];
        v29 = a2[v25];
      }
      v13 = ':';
      if (v29 == 'z') goto label_12f03;
      goto label_12903;
    case 0x41:
label_12ead:
      v13 = 'A';
      if (v29)
        v5 = v29;
      goto label_12ec2;
    case 0x42:
      v27 = 0;
label_12e8e:
      v13 = 'B';
      if (v29)
        v5 = v29;
      v18 = '\0';
      goto label_12d27;
    case 0x43:
      v27 = 0;
label_1393a:
      v30 = a3[5];
      v39._0_1_ = (int)v30 < -0x76c;
      v14 = (int)((-(unsigned int)(v30 + 0x76c < 0x76c) & 0xffffff9d) + v30) / 100 + 0x13;
      v6 = -0x76c <= (int)v30;
      if (!v33)
        v33 = a5;
      if (v33 == 0x2b) {
        v13 = 'C';
        v31 = 2;
        v30 = 99;
        goto label_14038;
      }
      v37 = 0;
      v13 = 'C';
      v31 = 2;
      goto label_12bb9;
    case 0x44:
label_13921:
      v31 = -1;
      v35 = "%m/%d/%y";
      goto label_13490;
    case 0x46:
label_1346e:
      if ((0 <= v24) || (v33)) {
        v35 = "%Y-%m-%d";
        v31 = 0;
        if (0 <= v24 + -6)
          v31 = v24 + -6;
        goto label_13490;
      }
      v35 = "%Y-%m-%d";
      v25 = sub_12690(0,0xffffffffffffffff,"%Y-%m-%d",a3,v5,0x2b,4,v4,a8);
      v15 = 0;
      v33 = 0x2b;
      v38 = v25;
      v31 = 4;
      goto label_13503;
    case 0x47:
    case 0x56:
    case 0x67:
      v27 = 0;
label_12a3a:
      v30 = a3[5];
      v14 = a3[7];
      v2 = a3[6];
      v31 = (v14 - v2) + 0x17e;
      v7 = (v30 - 100) + ((int)v30 >> 0x1f & 400U);
      v26 = (v14 - v31) + 3 + (v31 / 7) * 7;
      if (0 <= v26) { // branch-flip
        v31 = 0x16d;
        if (!(v7 & 3)) {
          v31 = 0x16e;
          v7 = v7 * -0x3d70a3d7 + 0x51eb850;
          if ((v7 >> 2 | v30 * 0x40000000) <= 0x28f5c28)
            v31 = ((v7 >> 4 | (v30 - 100) * -0x70000000) < 0xa3d70b) + 0x16d;
        }
        v11 = ((v14 - v31) - v2) + 0x17e;
        v31 = ((v14 - v31) - v11) + 3 + (v11 / 7) * 7;
        if (0 <= v31)
          v26 = v31;
        v11 = (v31 >> 0x1f) + 1;
      }
      else {
        v7 -= 1;
        v31 = 0x16d;
        if ((!(v7 & 3)) && (v31 = 0x16e, (v7 * -0x3d70a3d7 + 0x51eb850 >> 2 | v7 * 0x40000000) <= 0x28f5c28))
          v31 = 0x16e - (unsigned int)((int)v7 % 400 != 0);
        v11 = -1;
        v26 = ((v14 + v31) - v2) + 0x17e;
        v26 = ((v14 + v31) - v26) + 3 + (v26 / 7) * 7;
      }
      if (v13 != 'G') {
        if (v13 == 'g') {
          v14 = ((int)v30 % 100 + v11) % 100;
          if ((int)v14 < 0) {
            v14 = (0xfffff894U - v11 <= (int)v30) ? v14 + 100 : -v14; // branch-flip
          }
          v29 = v13;
          if (v33) goto label_13a14;
          v33 = a5;
          if (a5 != 0x2b) goto label_12ba4;
          goto label_13a1a;
        }
        v31 = 2;
        v14 = v26 / 7 + 1;
        goto label_12e0d;
      }
      v14 = v30 + 0x76c + v11;
      v39._0_1_ = (int)v30 < 0xfffff894U - v11;
      if (v33) goto label_14023;
      if (a5 == 0x2b) {
        v31 = 4;
        v30 = 9999;
        goto label_14038;
      }
      v37 = 0;
      v31 = 4;
      v6 = 0xfffff894U - v11 <= (int)v30;
      v33 = a5;
      goto label_12bb9;
    case 0x48:
      v27 = 0;
label_13906:
      v13 = 'H';
      v31 = 2;
      v14 = a3[2];
      break;
    case 0x49:
      v27 = 0;
label_138ed:
      v13 = 'I';
      v31 = 2;
      break;
    case 0x4d:
      v27 = 0;
label_138d0:
      v13 = 'M';
      v31 = 2;
      v14 = a3[1];
      break;
    case 0x4e:
label_1372b:
      v26 = 9;
      v31 = a8;
      if (v24 <= 0)
        v24 = 9;
      do {
        if (v26 <= v24) {
          if (v26 == 1) {
            v25 = 1;
            v15 = 1;
            goto label_1379c;
          }
          if (v31 % 10) goto label_1378e;
        }
        v26 -= 1;
        v31 /= 10;
      } while( true );
    case 0x50:
      v27 = 0;
label_1371e:
      v18 = '\x01';
      goto label_12ff5;
    case 0x52:
label_13705:
      v31 = -1;
      v35 = "%H:%M";
      goto label_13490;
    case 0x53:
      v27 = 0;
label_136eb:
      v13 = 'S';
      v31 = 2;
      v14 = *a3;
      break;
    case 0x54:
label_136d2:
      v31 = -1;
      v35 = "%H:%M:%S";
label_13490:
      v25 = sub_12690(0,0xffffffffffffffff,v35,a3,v5,v33,v31,v4,a8);
      if ((v33 != 0x2d) && (0 <= v24)) {
        v15 = (unsigned long)v24;
        v38 = v15;
        if (v15 <= v25)
          v38 = v25;
      }
      else {
        v38 = v25;
        v15 = 0;
      }
label_13503:
      if ((unsigned long)(a1 - v34) <= v38) goto label_127b8;
      if (a0) {
        v28 = a0;
        if (v25 < v15) {
          v15 -= v25;
          v28 = &a0[v15];
          if ((v33 != 0x30) && (v33 != 0x2b))
            memset(a0,0x20,v15);
          else {
            memset(a0,0x30,v15);
          }
        }
        sub_12690(v28,a1 - v34,v35,a3,v5,v33,v31,v4,a8);
        a0 = &v28[v25];
      }
      v34 += v38;
      goto label_1277b;
    case 0x55:
      v27 = 0;
label_13699:
      v13 = 'U';
      v31 = 2;
      v14 = (int)((a3[7] - a3[6]) + 7) / 7;
      break;
    case 0x57:
      v27 = 0;
label_1362f:
      v31 = 2;
      v13 = 'W';
      v14 = (int)((((int)(a3[6] + 6) / 7) * 7 - (a3[6] + 6)) + 7 + a3[7]) / 7;
      break;
    case 0x58:
      v27 = 0;
label_1361e:
      v13 = 'X';
      v18 = '\0';
      goto label_12d27;
    case 0x59:
      v27 = 0;
label_135bf:
      v13 = 'Y';
      v39._0_1_ = (int)a3[5] < -0x76c;
      v14 = a3[5] + 0x76c;
      if (v33) {
label_14023:
        v31 = 4;
        v30 = 9999;
        if (v33 != 0x2b) {
          v37 = 0;
          v6 = (bool)(v39._0_1_ ^ 1);
          goto label_12bb9;
        }
label_14038:
        if (v14 <= v30) goto label_13a25;
        v37 = 1;
        v33 = 0x2b;
        v6 = (bool)(v39._0_1_ ^ 1);
        goto label_12bb9;
      }
      if (a5 == 0x2b) {
        v13 = 'Y';
        v31 = 4;
        v30 = 9999;
        goto label_14038;
      }
      v37 = 0;
      v30 = 0;
      v31 = 4;
      v33 = a5;
      goto label_13608;
    case 0x5a:
label_13362:
      if (v29)
        v5 = '\0';
      v25 = strlen(v32);
      if ((v33 != 0x2d) && (0 <= v24)) {
        v15 = (unsigned long)v24;
        v39 = v15;
        if (v15 <= v25)
          v39 = v25;
      }
      else {
        v39 = v25;
        v15 = 0;
      }
      if ((unsigned long)(a1 - v34) <= v39) goto label_127b8;
      if (a0) {
        v35 = a0;
        if (v25 < v15) {
          v15 -= v25;
          v35 = &a0[v15];
          if ((v33 != 0x30) && (v33 != 0x2b))
            memset(a0,0x20,v15);
          else {
            memset(a0,0x30,v15);
          }
        }
        if (v29) { // branch-flip
          if (v25) {
            v9 = __ctype_tolower_loc();
            v15 = v25;
            do {
              v15 -= 1;
              v35[v15] = (char)*(unsigned int *)(*v9 + (unsigned long)(unsigned char)v32[v15] * 4);
            } while (v15);
          }
        }
        else if (v5) { // branch-flip
          if (v25) {
            v9 = __ctype_toupper_loc();
            v15 = v25;
            do {
              v15 -= 1;
              v35[v15] = (char)*(unsigned int *)(*v9 + (unsigned long)(unsigned char)v32[v15] * 4);
            } while (v15);
          }
        }
        else {
          memcpy(v35,v32,v25);
        }
        a0 = &v35[v25];
      }
      v34 += v39;
      goto label_1277b;
    case 0x61:
label_12fd4:
      v13 = 'a';
      if (v29)
        v5 = v29;
label_12ec2:
      v18 = '\0';
      v35 = (char *)((long)&v20 + 5);
      goto label_12bf0;
    case 0x62:
    case 0x68:
      v27 = 0;
label_128e0:
      if (v29)
        v5 = v29;
      if (v27 == 0x45) goto label_12900;
      goto label_12d25;
    case 99:
      v27 = 0;
label_139b1:
      v13 = 'c';
      v18 = '\0';
      goto label_12d27;
    case 100:
      v27 = 0;
label_12e74:
      v13 = 'd';
      v31 = 2;
      v14 = a3[3];
      break;
    case 0x65:
      v27 = 0;
label_12e60:
      v13 = 'e';
      v14 = a3[3];
      goto label_12dfb;
    case 0x6a:
      v27 = 0;
label_12e2e:
      v37 = 0;
      v13 = 'j';
      v31 = 3;
      v30 = a3[7];
      v14 = v30 + 1;
      v39._0_1_ = (int)v30 < -1;
      v6 = -1 <= (int)v30;
      v30 = 0;
      goto label_12bc0;
    case 0x6b:
      v27 = 0;
label_12dee:
      v13 = 'k';
      v14 = a3[2];
      goto label_12dfb;
    case 0x6c:
      v27 = 0;
label_130cd:
      v13 = 'l';
label_12dfb:
      v31 = 2;
      if (!v33)
        v33 = 0x5f;
      break;
    case 0x6d:
      v27 = 0;
label_1309b:
      v37 = 0;
      v13 = 'm';
      v31 = 2;
      v30 = a3[4];
      v14 = v30 + 1;
      v39._0_1_ = (int)v30 < -1;
      v6 = -1 <= (int)v30;
      v30 = 0;
      goto label_12bc0;
    case 0x6e:
label_1301a:
      if ((v33 != 0x2d) && (0 <= v24)) {
        v15 = (unsigned long)v24;
        v25 = 1;
        if (v15)
          v25 = v15;
        if ((unsigned long)(a1 - v34) <= v25) goto label_127b8;
        if (a0) {
          if (2 <= v24) {
            v15 -= 1;
            v35 = &a0[v15];
            if ((v33 != 0x30) && (v33 != 0x2b)) {
              memset(a0,0x20,v15);
              a0 = v35;
            }
            else {
              memset(a0,0x30,v15);
              a0 = v35;
            }
          }
label_13089:
          *a0 = '\n';
          a0 = &a0[1];
        }
      }
      else {
        if ((unsigned long)(a1 - v34) <= 1) goto label_127b8;
        v25 = 1;
        if (a0) goto label_13089;
      }
      goto label_12de1;
    case 0x70:
      v27 = 0;
label_12ff3:
      v18 = '\0';
label_12ff5:
      v13 = 'p';
      if (v29) {
        v5 = '\0';
        v18 = v29;
      }
      goto label_12d27;
    case 0x71:
label_130de:
      v37 = 0;
      v31 = 1;
      v30 = 0;
      v39._0_1_ = 0;
      v14 = ((int)(a3[4] * 0xb) >> 5) + 1;
      goto label_13108;
    case 0x72:
      v27 = 0;
label_12d25:
      v18 = '\0';
      goto label_12d27;
    case 0x73:
label_13167:
      v43 = *(unsigned long *)&a3[8];
      v44 = *(unsigned long *)&a3[10];
      v23 = *(unsigned long *)a3;
      v40 = *(unsigned long *)&a3[2];
      v41 = *(unsigned long *)&a3[4];
      v45 = *(unsigned long *)&a3[0xc];
      v42._0_4_ = (unsigned int)*(unsigned long *)&a3[6];
      v42 = CONCAT44(0xffffffff,(unsigned int)v42);
      v19 = sub_176e0(v4,&v23);
      if (v42 <= -1) {
        *v8 = 0x4b;
        return 0;
      }
      v39._0_1_ = SUB81((unsigned long)v19 >> 0x3f,0);
      v35 = &v22[1];
      v12 = v19;
      do {
        v3 = v12 / 10;
        v13 = (char)v12 + (char)v3 * '\xf6';
        if (v19 <= -1)
          v13 = -v13;
        v35 = &v35[-1];
        *v35 = v13 + '0';
        v12 = v3;
      } while (v3);
      v37 = 0;
      v31 = 1;
      goto label_13258;
    case 0x74:
label_12d6e:
      if ((v33 != 0x2d) && (0 <= v24)) {
        v15 = (unsigned long)v24;
        v25 = 1;
        if (v15)
          v25 = v15;
        if ((unsigned long)(a1 - v34) <= v25) goto label_127b8;
        if (a0) {
          if (2 <= v24) {
            v15 -= 1;
            v35 = &a0[v15];
            if ((v33 != 0x30) && (v33 != 0x2b)) {
              memset(a0,0x20,v15);
              a0 = v35;
            }
            else {
              memset(a0,0x30,v15);
              a0 = v35;
            }
          }
label_12dd9:
          *a0 = '\t';
          a0 = &a0[1];
        }
      }
      else {
        if ((unsigned long)(a1 - v34) <= 1) goto label_127b8;
        v25 = 1;
        if (a0) goto label_12dd9;
      }
label_12de1:
      v34 += v25;
      goto label_1277b;
    case 0x75:
      v27 = 0;
label_13a73:
      v13 = 'u';
      v31 = 1;
      v14 = (int)(a3[6] + 6) % 7 + 1;
      break;
    case 0x77:
      v27 = 0;
label_13a56:
      v13 = 'w';
      v31 = 1;
      v14 = a3[6];
      break;
    case 0x78:
      v27 = 0;
label_13a45:
      v13 = 'x';
      v18 = '\0';
label_12d27:
      if (v27) goto label_12be0;
      v35 = (char *)((long)&v20 + 5);
      goto label_12bf0;
    case 0x79:
      v27 = 0;
label_139c2:
      v30 = (int)a3[5] % 100;
      v14 = v30;
      if ((int)v30 <= -1) {
        v14 = v30 + 100;
        if ((int)a3[5] <= -0x76d)
          v14 = -v30;
      }
      v13 = 'y';
      v29 = v13;
      if (!v33) {
        v13 = 'y';
        v33 = a5;
        v29 = 'y';
      }
label_13a14:
      if (v33 != 0x2b) { // branch-flip
label_12ba4:
        v37 = 0;
        v6 = 1;
        v31 = 2;
        v39._0_1_ = 0;
      }
      else {
label_13a1a:
        v13 = v29;
        v39._0_1_ = 0;
        v31 = 2;
label_13a25:
        v37 = v31 < v24;
        v6 = (bool)(v39._0_1_ ^ 1);
        v33 = 0x2b;
      }
label_12bb9:
      v30 = 0;
      goto label_12bc0;
    case 0x7a:
      v27 = 0;
label_13463:
      v25 = 0;
      v35 = a2;
label_12f03:
      a2 = v35;
      if ((int)a3[8] < 0) goto label_1277b;
      v39._0_1_ = 1;
      v26 = (int)*(unsigned long *)&a3[10];
      if ((0 <= v26) && (v39._0_1_ = 0, !v26))
        v39._0_1_ = *v32 == '-';
      v14 = v26 / 0xe10;
      v11 = (v26 / 0x3c) % 0x3c;
      if (v25 != 2) { // branch-flip
        if (3 <= v25) { // branch-flip
          if (v25 != 3) {
            v13 = *a2;
            goto label_12d56;
          }
          if (v26 % 0x3c) goto label_141d0;
          if (v11) goto label_12fae;
          v37 = 1;
          v31 = 3;
          v6 = (bool)(v39._0_1_ ^ 1);
          v30 = 0;
        }
        else if (v25) { // branch-flip
label_12fae:
          v37 = 1;
          v31 = 6;
          v6 = (bool)(v39._0_1_ ^ 1);
          v14 = v14 * 100 + v11;
          v30 = 4;
        }
        else {
          v37 = 1;
          v31 = 5;
          v6 = (bool)(v39._0_1_ ^ 1);
          v14 = v14 * 100 + v11;
          v30 = 0;
        }
      }
      else {
label_141d0:
        v37 = 1;
        v31 = 9;
        v6 = (bool)(v39._0_1_ ^ 1);
        v30 = 0x14;
        v14 = v14 * 10000 + v11 * 100 + v26 % 0x3c;
      }
      goto label_12bc0;
    
  }
label_12e0d:
  v37 = 0;
  v30 = 0;
  v10 = (unsigned char)(v14 >> 0x18);
  v39._0_1_ = (bool)(v10 >> 7);
  v6 = (bool)((unsigned char)~v10 >> 7);
label_12bc0:
  if (v27 != 0x4f) { // branch-flip
label_13608:
    if (v39._0_1_)
      v14 = -v14;
label_13108:
    v25 = (unsigned long)v14;
    v35 = &v22[1];
    while( true ) {
      v28 = v35;
      if (v30 & 1) {
        v35[-1] = ':';
        v28 = &v35[-1];
      }
      v30 = (int)v30 >> 1;
      v35 = &v28[-1];
      v28[-1] = (char)v25 + (char)(v25 / 10) * '\xf6' + '0';
      if (((unsigned int)v25 <= 9) && (!v30)) break;
      v25 /= 10;
    }
label_13258:
    v14 = v33;
    if (!v33)
      v14 = 0x30;
    if (v24 <= -1)
      v24 = v31;
    v31 = ((int)v22 + 1) - (int)v35;
    if (v39._0_1_) { // branch-flip
      v13 = '-';
label_13d15:
      v26 = v24 + -1;
      v11 = v26 - v31;
      if ((v11 <= 0) || (v33 == 0x2d))
        v11 = 0;
      if (v14 != 0x5f) { // branch-flip
        if ((unsigned long)(a1 - v34) <= 1) goto label_127b8;
        if (a0) goto label_13d46;
label_13d4d:
        v34 += 1;
        v24 = v26;
        if (v14 != 0x2d) goto label_13d5a;
label_13d8e:
        v25 = (unsigned long)v31;
        v15 = 0;
      }
      else {
        v25 = (unsigned long)v11;
        v34 += v25;
        if (a0) {
          memset(a0,0x20,v25);
          a0 = &a0[v25];
          if ((unsigned long)(a1 - v34) <= 1) goto label_127b8;
          v26 = (v24 - v11) + -1;
label_13d46:
          *a0 = v13;
          a0 = &a0[1];
          goto label_13d4d;
        }
        if ((unsigned long)(a1 - v34) <= 1) goto label_127b8;
        v34 += 1;
        v24 = (v24 - v11) + -1;
label_13d5a:
        v25 = (unsigned long)v31;
        v15 = 0;
        if (0 <= v24) goto label_132a0;
      }
    }
    else {
      if (v37) {
        v13 = '+';
        goto label_13d15;
      }
      if (((v31 >= v24) || (v33 == 0x2d)) && (v14 == 0x2d)) goto label_13d8e;
label_132a0:
      v15 = (unsigned long)v24;
      v25 = (long)v31;
      if ((unsigned long)(long)v31 <= v15)
        v25 = v15;
    }
    v17 = (unsigned long)v31;
    if ((unsigned long)(a1 - v34) <= v25) goto label_127b8;
    if (a0) {
      v28 = a0;
      if (v17 < v15) {
        v15 -= v17;
        v28 = &a0[v15];
        if ((v14 != 0x30) && (v14 != 0x2b))
          memset(a0,0x20,v15);
        else {
          memset(a0,0x30,v15);
        }
      }
      if (v5) { // branch-flip
        if (v17) {
          v9 = __ctype_toupper_loc();
          v15 = v17;
          do {
            v15 -= 1;
            v28[v15] = (char)*(unsigned int *)(*v9 + (unsigned long)(unsigned char)v35[v15] * 4);
          } while (v15);
        }
      }
      else {
        memcpy(v28,v35,v17);
      }
      a0 = &v28[v17];
    }
    v34 += v25;
  }
  else {
    v27 = 0x4f;
    if (!v6) goto label_13608;
label_12bd1:
    v18 = '\0';
label_12be0:
    v20._5_1_ = (char)v27;
    v35 = (char *)((long)&v20 + 6);
label_12bf0:
    v20._3_2_ = 0x2520;
    *v35 = v13;
    v35[1] = '\0';
    v25 = strftime((char *)&v21,0x400,(char *)((long)&v20 + 3),a3);
    if (v25) {
      v15 = v25 - 1;
      if ((0 <= v24) && (v33 != 0x2d)) {
        v16 = (unsigned long)v24;
        v17 = v16;
        if (v16 <= v15)
          v17 = v15;
      }
      else {
        v16 = 0;
        v17 = v15;
      }
      if ((unsigned long)(a1 - v34) <= v17) goto label_127b8;
      v35 = a2;
      if (a0) {
        v28 = a0;
        if (v15 < v16) {
          v16 -= v15;
          v28 = &a0[v16];
          if ((v33 != 0x30) && (v33 != 0x2b))
            memset(a0,0x20,v16);
          else {
            memset(a0,0x30,v16);
          }
        }
        if (v18) { // branch-flip
          v19 = v25 - 2;
          if (v15) {
            v9 = __ctype_tolower_loc();
            do {
              v28[v19] = (char)*(unsigned int *)(*v9 + (unsigned long)*(unsigned char *)((long)&v21 + v19 + 1) * 4);
              v37 = v19 != 0;
              v19 -= 1;
            } while (v37);
          }
        }
        else if (v5) { // branch-flip
          v19 = v25 - 2;
          if (v15) {
            v9 = __ctype_toupper_loc();
            do {
              v28[v19] = (char)*(unsigned int *)(*v9 + (unsigned long)*(unsigned char *)((long)&v21 + v19 + 1) * 4);
              v37 = v19 != 0;
              v19 -= 1;
            } while (v37);
          }
        }
        else {
          memcpy(v28,(void *)((long)&v21 + 1),v15);
        }
        a0 = &v28[v15];
      }
label_12d07:
      a2 = v35;
      v34 += v17;
    }
  }
  goto label_1277b;
label_12903:
  v35 = a2;
  goto label_12910;
label_1378e:
  v15 = (unsigned long)v26;
  v25 = v15;
  if (v26) { // branch-flip
label_1379c:
    v35 = (char *)((long)&v21 + v15);
    do {
      v35 = &v35[-1];
      *v35 = (char)v31 + (char)(v31 / 10) * '\xf6' + '0';
      v31 /= 10;
    } while ((char *)((long)&v20 + (v15 - (unsigned int)(v26 - 1)) + 7) != v35);
  }
  else {
    v25 = 0;
  }
  if (!v33)
    v33 = 0x30;
  if ((unsigned long)(a1 - v34) > v25) {
    if (a0) {
      if (v5) { // branch-flip
        if (v25) {
          v9 = __ctype_toupper_loc();
          v15 = v25;
          do {
            v17 = v15 - 1;
            a0[v17] = (char)*(unsigned int *)(*v9 + (unsigned long)*(unsigned char *)((long)&v20 + v15 + 7) * 4);
            v15 = v17;
          } while (v17);
        }
      }
      else if (8 <= v25) { // branch-flip
        *(unsigned long *)a0 = CONCAT44(v21._4_4_,(unsigned int)v21);
        *(unsigned long *)&a0[v25 - 8] = *(unsigned long *)((long)&v20 + v25);
        v19 = (long)a0 - ((unsigned long)&a0[8] & 0xfffffffffffffff8);
        v15 = v19 + v25 & 0xfffffffffffffff8;
        if (8 <= v15) {
          v17 = 0;
          do {
            *(unsigned long *)(((unsigned long)&a0[8] & 0xfffffffffffffff8) + v17) = *(unsigned long *)((long)&v21 + (v17 - v19));
            v17 += 8;
          } while (v17 < v15);
        }
      }
      else if (v25 & 4) { // branch-flip
        *(unsigned int *)a0 = (unsigned int)v21;
        *(unsigned int *)&a0[v25 - 4] = *(unsigned int *)((long)&v20 + v25 + 4);
      }
      else if (v25) {
        *a0 = (char)v21;
        if (v25 & 2)
          *(unsigned short *)&a0[v25 - 2] = *(unsigned short *)((long)&v20 + v25 + 6);
      }
      a0 = &a0[v25];
    }
    v34 += v25;
    if ((v33 != 0x2d) && (0 <= v24 - v26)) {
      v25 = (unsigned long)(v24 - v26);
      if ((unsigned long)(a1 - v34) <= v25) goto label_127b8;
      if (a0) { // branch-flip
        if (v25) {
          v35 = &a0[v25];
          v34 += v25;
          if ((v33 != 0x30) && (v33 != 0x2b)) {
            memset(a0,0x20,v25);
            a0 = v35;
          }
          else {
            memset(a0,0x30,v25);
            a0 = v35;
          }
        }
      }
      else {
        v34 += v25;
      }
    }
    else if (a1 == v34) goto label_127b8;
label_1277b:
    v5 = a2[1];
    a2 = &a2[1];
    v25 = 0xffffffffffffffff;
    if (!v5) goto label_12794;
    goto label_12732;
  }
label_127b8:
  *v8 = 0x22;
  return 0;
}

// Function: sub_14460 @ 0x14460
void sub_14460(void)
{
  sub_12690();
}

// Function: sub_14480 @ 0x14480
void sub_14480(char *a0)
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

// Function: sub_14520 @ 0x14520
void sub_14520(char *a0)
{
  unsigned char v1;
  unsigned long v2;
  unsigned long v3; // rax
  long v4; // rax
  
  if (!*a0) {
    v1 = **(unsigned char **)&a0[0x10];
    if (*(unsigned int *)((unsigned long)(v1 >> 5) * 4 + 0x1e8a0) >> (v1 & 0x1f) & 1) {
      a0[0x18] = '\x01';
      a0[0x19] = '\0';
      a0[0x1a] = '\0';
      a0[0x1b] = '\0';
      a0[0x1c] = '\0';
      a0[0x1d] = '\0';
      a0[0x1e] = '\0';
      a0[0x1f] = '\0';
      v1 = **(unsigned char **)&a0[0x10];
      a0[0x20] = '\x01';
      *(int *)&a0[0x24] = (int)(char)v1;
      goto label_14561;
    }
    if (!mbsinit(&a0[4]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    *a0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v2 = *(unsigned long *)&a0[0x10];
  v4 = sub_17200(&a0[0x24],v2,sub_15cf0(v2,v3),&a0[4]);
  *(long *)&a0[0x18] = v4;
  if (v4 == -1) {
    a0[0x18] = '\x01';
    a0[0x19] = '\0';
    a0[0x1a] = '\0';
    a0[0x1b] = '\0';
    a0[0x1c] = '\0';
    a0[0x1d] = '\0';
    a0[0x1e] = '\0';
    a0[0x1f] = '\0';
    a0[0x20] = '\0';
    a0[0xc] = '\x01';
    return;
  }
  if (v4 == -2) {
    v3 = strlen(*(char **)&a0[0x10]);
    a0[0x20] = '\0';
    *(unsigned long *)&a0[0x18] = v3;
    a0[0xc] = '\x01';
    return;
  }
  if (!v4) {
    a0[0x18] = '\x01';
    a0[0x19] = '\0';
    a0[0x1a] = '\0';
    a0[0x1b] = '\0';
    a0[0x1c] = '\0';
    a0[0x1d] = '\0';
    a0[0x1e] = '\0';
    a0[0x1f] = '\0';
    if (**(char **)&a0[0x10])
      __assert_fail("*iter->cur.ptr == \'\\0\'","mbuiter.h",0xab,"mbuiter_multi_next"); // no-return
    if (*(int *)&a0[0x24])
      __assert_fail("iter->cur.wc == 0","mbuiter.h",0xac,"mbuiter_multi_next"); // no-return
  }
  a0[0x20] = '\x01';
  if (mbsinit(&a0[4])) {
    *a0 = '\0';
    a0[0xc] = '\x01';
    return;
  }
label_14561:
  a0[0xc] = '\x01';
  return;
}

// Function: sub_14690 @ 0x14690
unsigned long sub_14690(char *a0,unsigned long a1)
{
  char v1;
  char v10; // stack - 0xac
  char *v11; // stack - 0xa8
  long v12; // stack - 0xa0
  char v13; // stack - 0x98
  int v14; // stack - 0x94
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
  
  v3 = (char *)sub_15d60(a1,2);
  v1 = *a0;
  while ((v1 && (v5 = (char *)sub_182a0(a0,v3), v5))) {
    if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
      v6[0] = 0;
      v9 = 0;
      v11 = a0;
      if (v5 <= a0) { // branch-flip
label_147c1:
        v8 = (unsigned char *)0x1;
      }
      else {
        do {
          v10 = '\0';
          sub_14520(v6);
          if ((v13) && (!v14)) {
            abort(); // no-return, return-dupe
          }
          v11 = &v11[v12];
          v10 = 0;
        } while (v11 < v5);
        if (!v13) goto label_147c1;
        v8 = (unsigned char *)CONCAT71((undefined7)((unsigned long)v8 >> 8),iswalnum() == 0);
      }
      v11 = v5;
      v6[0] = 0;
      v9 = 0;
      v17 = v3;
      v7[0] = 0;
      v15 = 0;
      while( true ) {
        v10 = '\0';
        v16 = 0;
        sub_14520(v7);
        if ((v19) && (!v20)) break;
        if (!v10)
          sub_14520(v6);
        if ((v13) && (!v14))
          abort();
        v11 = &v11[v12];
        v17 = &v17[v18];
      }
      if (!v10)
        sub_14520(v6);
      if (((((v13) && (!v14)) || (!v13)) || (!iswalnum())) && ((char)v8)) {
        free(v3); // return-dupe
        return (unsigned long)v8 & 0xffffffff;
      }
      v11 = v5;
      v6[0] = 0;
      v9 = 0;
      v10 = '\0';
      sub_14520(v6);
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

// Function: sub_14920 @ 0x14920
char * sub_14920(char *a0)
{
  char *v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = dcgettext(NULL,a0,5);
  if (a0 != v1) {
    if (!sub_14690(v1,a0)) {
      v2 = strlen(v1);
      v3 = (char *)sub_16a40(v2 + 4 + strlen(a0));
      __sprintf_chk(v3,1,0xffffffffffffffff,"%s (%s)",v1,a0);
      return v3;
    }
  }
  return v1;
}

// Function: sub_149b0 @ 0x149b0
char * sub_149b0(char *a0,char *a1)
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
  void *v8; // rax
  unsigned long *v9; // rbx
  
  v3 = dcgettext(NULL,a0,5);
  v4 = (char *)sub_17ab0();
  v6 = v3;
  if (sub_17870(v4,"UTF-8")) { // branch-flip
    v7 = (char *)sub_16ff0(a1,"UTF-8",v4);
    v5 = strlen(v4);
    v8 = memcpy((void *)sub_16a40(v5 + 0xb),v4,v5);
    v9 = (unsigned long *)(v5 + (long)v8);
    *v9 = 0x4c534e4152542f2f;
    *(unsigned short *)&v9[1] = 0x5449;
    *(char *)((long)v9 + 10) = 0;
    a1 = (char *)sub_16ff0(a1,"UTF-8",v8);
    free(v8);
    v11 = v7;
    if (a1) { // branch-flip
      if (strchr(a1,0x3f)) {
        free(a1);
        goto label_14bb3;
      }
      v4 = a1;
      if (!v7) {
        v10 = a1;
        goto label_14a1a;
      }
      if (!strcmp(v3,a0)) {
        if (v7 == a1)
          return v7;
        free(a1);
        return v7;
      }
    }
    else {
label_14bb3:
      if (!v7) {
        v11 = NULL;
        a1 = NULL;
        v10 = a0;
        v4 = NULL;
        goto label_14a1a;
      }
      if (!strcmp(v3,a0))
        return v7;
      a1 = NULL;
      v4 = NULL;
    }
    if (!sub_14690(v3,a0)) {
      a0 = v7;
label_14a3f:
      if (((!v11) || (!sub_14690(v3,v11))) && ((!a1 || (!sub_14690(v3,a1))))) goto label_14abc;
      if (!v7) goto label_14a6d;
    }
    free(v7);
  }
  else {
    v2 = strcmp(v3,a0);
    if (a1) {
      v10 = a1;
      v4 = NULL;
      v11 = a1;
label_14a1a:
      if (!strcmp(v3,a0))
        return v10;
      v1 = sub_14690(v3,a0);
      v7 = NULL;
      a0 = v10;
      if (v1) goto label_14a6d;
      goto label_14a3f;
    }
    if (!v2)
      return a0;
    if (sub_14690(v3,a0))
      return v3;
    v4 = NULL;
    v7 = NULL;
label_14abc:
    v5 = strlen(v3);
    v6 = (char *)sub_16a40(v5 + 4 + strlen(a0));
    __sprintf_chk(v6,1,0xffffffffffffffff,"%s (%s)",v3,a0);
    if (v7)
      free(v7);
  }
label_14a6d:
  if (v4)
    free(v4);
  return v6;
}

// Function: sub_14cb0 @ 0x14cb0
unsigned long sub_14cb0(int a0,char *a1,unsigned long a2) // return-dupe x2
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

// Function: sub_14d60 @ 0x14d60
void sub_14d60(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_14d70 @ 0x14d70
void sub_14d70(char *a0)
{
  long v1;
  
  if (!dat_36868) {
    dat_36868 = sub_1a0d0(0);
    sub_1a130(dat_36868,1);
  }
  v1 = dat_36868;
  sub_1a1f0(0,0,a0,strlen(a0),v1); // tail-call
}

// Function: sub_14de0 @ 0x14de0
long sub_14de0(long a0,char *a1)
{
  long v1;
  unsigned long v2; // rax
  
  if (!dat_36868) {
    dat_36868 = sub_1a0d0(0);
    sub_1a130(dat_36868,1);
  }
  v1 = dat_36868;
  v2 = strlen(a1);
  return sub_1a1f0(a0,0xffffffffffffffff,a1,v2,v1) + a0;
}

// Function: sub_14e50 @ 0x14e50
void sub_14e50(char *a0)
{
  long v1;
  
  if (!dat_36868) {
    dat_36868 = sub_1a0d0(0);
    sub_1a130(dat_36868,1);
  }
  v1 = dat_36868;
  sub_1a360(a0,strlen(a0),v1); // tail-call
}

// Function: sub_14ec0 @ 0x14ec0
char * sub_14ec0(long *a0)
{
  long v1;
  char *v2; // rax
  long v3;
  char *v4;
  char *v5;
  long *v6;
  
  v5 = (char *)*a0;
  if (v5) {
    v3 = 0;
    v6 = a0;
    do {
      if (!dat_36868) {
        dat_36868 = sub_1a0d0(0);
        sub_1a130(dat_36868,1);
      }
      v1 = dat_36868;
      v6 = &v6[1];
      v1 = sub_1a1f0(0,0,v5,strlen(v5),v1);
      v5 = (char *)*v6;
      v3 = v3 + 1 + v1;
    } while (v5);
    v2 = (char *)sub_16a40(v3);
    v5 = (char *)*a0;
    v4 = v2;
    while( true ) {
      if (!dat_36868) {
        dat_36868 = sub_1a0d0(0);
        sub_1a130(dat_36868,1);
      }
      v3 = dat_36868;
      a0 = &a0[1];
      v3 = sub_1a1f0(v4,0xffffffffffffffff,v5,strlen(v5),v3);
      v5 = (char *)*a0;
      v4 = &v4[v3];
      if (!v5) break;
      *v4 = 0x20;
      v4 = &v4[1];
    }
    *v4 = 0;
    return v2;
  }
  v4 = (char *)sub_16ea0(""); // tail-call
  return v4;
}

// Function: sub_15000 @ 0x15000
void sub_15000(unsigned long a0,long a1,long a2)
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
  
  v1 = *(unsigned long *)(a1 + 0x10);
  if (dat_36870) { // branch-flip
    v4 = (*dat_36870)(v1,0);
    if (v4)
      return;
    if (dat_36888) goto label_1504e;
  }
  else {
    if (!dat_36888) {
      signal(0xb,0); // return-dupe, tail-call
      return;
    }
label_1504e:
    v3 = dat_36890;
    v2 = *(unsigned long *)(a2 + 0xa0);
    if (dat_36890) {
      v6 = __errno_location();
      v4 = *v6;
      v5 = sub_158d0(v3,&v7);
      *v6 = v4;
      if (0 <= v5) {
        if (v7 <= v1) { // branch-flip
          if (v1 <= (unsigned long)(v8 - 1U)) goto label_15135;
        }
        else if ((*v9)(v1,&v7)) {
label_15135:
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

// Function: sub_15190 @ 0x15190
void sub_15190(void)
{
  void *v1; // stack - 0xa8
  char v2 [128];
  int v3; // stack - 0x20
  
  v1 = sub_15000;
  sigemptyset(v2);
  sigaddset(v2,1);
  sigaddset(v2,2);
  sigaddset(v2,3);
  sigaddset(v2,0xd);
  sigaddset(v2,0xe);
  sigaddset(v2,0xf);
  sigaddset(v2,10);
  sigaddset(v2,0xc);
  sigaddset(v2,0x11);
  sigaddset(v2,0x11);
  sigaddset(v2,0x17);
  sigaddset(v2,0x1d);
  sigaddset(v2,0x1d);
  sigaddset(v2,0x18);
  sigaddset(v2,0x19);
  sigaddset(v2,0x1a);
  sigaddset(v2,0x1b);
  sigaddset(v2,0x1e);
  sigaddset(v2,0x1c);
  v3 = (-(unsigned int)(dat_36888 == 0) & 0xf8000000) + 0x8000004;
  sigaction(0xb,&v1,NULL);
}

// Function: sub_15310 @ 0x15310
unsigned long sub_15310(unsigned long a0)
{
  dat_36870 = a0;
  sub_15190();
  return 0;
}

// Function: sub_15330 @ 0x15330
void sub_15330(void)
{
  dat_36870 = 0;
  if (dat_36888)
    return;
  signal(0xb,0); // tail-call
}

// Function: sub_15360 @ 0x15360
unsigned long sub_15360(void *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  (*a0)(a1,a2,a3);
  return 1;
}

// Function: sub_15390 @ 0x15390
long sub_15390(unsigned long a0,unsigned long a1,unsigned long a2)
{
  int v1; // eax
  long v2; // rax
  unsigned long v3; // stack - 0x58
  char v4 [4];
  unsigned int v5; // stack - 0x50
  unsigned int v6; // stack - 0x4c
  unsigned long v7; // stack - 0x48
  
  if (!dat_36890) {
    v1 = sub_158d0(v4,&v3);
    if (0 <= v1) { // branch-flip
      v2 = CONCAT44(v6,v5) + -1;
      dat_36890 = v2;
    }
    else {
      v2 = dat_36890;
    }
    if (!v2) {
      v2 = 0xffffffff;
      return v2;
    }
  }
  v5 = 0;
  dat_36878 = a2;
  dat_36880 = a1;
  dat_36888 = a0;
  v3 = a1;
  v7 = a2;
  v1 = sigaltstack(&v3,0);
  if (v1 <= -1) {
    v2 = 0xffffffff;
    return v2;
  }
  sub_15190();
  v2 = 0;
  return v2;
}

// Function: sub_15470 @ 0x15470
void sub_15470(void) // return-dupe
{
  char v1 [8];
  unsigned int v2; // stack - 0x20
  
  dat_36888 = 0;
  if (dat_36870) // branch-flip
    sub_15190();
  else {
    signal(0xb,0);
  }
  v2 = 2;
  if (0 <= (int)sigaltstack(v1,0))
    return;
  perror("gnulib sigsegv (stackoverflow_deinstall_handler)");
}

// Function: sub_15500 @ 0x15500
bool sub_15500(long a0,long *a1)
{
  return (unsigned long)(*a1 - a0) <= (unsigned long)(*a1 - a1[3]) >> 1;
}

// Function: sub_15520 @ 0x15520
unsigned long sub_15520(long *a0,long *a1)
{
  unsigned char v1;
  long v2;
  long v3; // rcx
  int v4; // esi
  
  v4 = 0;
  v3 = 0;
  while (v2 = *a0, v2 != a0[1]) {
    v1 = *(unsigned char *)(a0[3] + v2);
    if (10 <= (unsigned int)v1 - 0x30) { // branch-flip
      if (6 <= (unsigned int)v1 - 0x41) { // branch-flip
        if (5 < (unsigned int)v1 - 0x61) goto label_155a0;
        v3 = v3 * 0x10 + (long)(int)((unsigned int)v1 - 0x57);
      }
      else {
        v3 = v3 * 0x10 + (long)(int)((unsigned int)v1 - 0x37);
      }
    }
    else {
      v3 = v3 * 0x10 + (long)(int)((unsigned int)v1 - 0x30);
    }
    v4 += 1;
    *a0 = v2 + 1;
  }
  *(unsigned int *)&a0[2] = 1;
label_155a0:
  if (v4) {
    *a1 = v3;
    return 0;
  }
  return 0xffffffff;
}

// Function: sub_155b0 @ 0x155b0
unsigned long sub_155b0(unsigned long a0,long *a1) // return-dupe
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  long v4; // rax
  unsigned long v5; // rax
  long v6;
  long v7;
  unsigned long v8;
  unsigned long v9; // stack - 0x58
  
  v1 = dat_36898;
  v3 = a0 * 2 - *a1;
  if (a0 < v3)
    return 0;
  v4 = (v3 / dat_36898) * dat_36898;
  v5 = (((unsigned long)(*a1 - 1U) / dat_36898 + 1) * dat_36898 - v4) / dat_36898;
  v3 = 1;
  if (v5 <= 1)
    return 1;
  do {
    v9 = v3;
    v3 = v9 * 2;
  } while (v3 < v5);
  v9 &= 0x7fffffffffffffff;
  while( true ) {
    v2 = v9;
    v7 = v4 + v9 * v1;
    if (v9 < v5) {
      v8 = v9;
      do {
        if (0 <= (int)mincore(v7,v1,&v6))
          return 0;
        v8 += v3 & 0xfffffffffffffffe;
        v7 += v9 * v1 * 2;
      } while (v8 < v5);
    }
    if (v9 < 2) break;
    v9 >>= 1;
    v3 = v2;
  }
  return 1;
}

// Function: sub_156f0 @ 0x156f0
unsigned long sub_156f0(unsigned long a0)
{
  unsigned long v1;
  unsigned long v2;
  char v3 [1032];
  long v4; // rsi
  unsigned long v5; // r14
  unsigned long v6; // r15
  
  v2 = 0x400;
  while( true ) {
    v1 = dat_36898;
    if (!a0)
      return 0;
    if (a0 / dat_36898 < v2)
      v2 = a0 / dat_36898;
    v4 = dat_36898 * v2;
    v5 = a0 - v4;
    if ((int)mincore(v5,v4,v3) < 0) break;
    a0 = v5;
  }
  do {
    if (v2 == 1)
      return a0;
    while( true ) {
      v5 = v2 + 1 >> 1;
      v4 = v1 * v5;
      v6 = a0 - v4;
      if ((int)mincore(v6,v4,v3) < 0) break;
      v2 >>= 1;
      a0 = v6;
      if (v2 == 1)
        return v6;
    }
    v2 = v5;
  } while( true );
}

// Function: sub_157d0 @ 0x157d0
long sub_157d0(long a0) // return-dupe
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  char v4 [1032];
  
  v1 = dat_36898;
  a0 += dat_36898;
  if (!a0)
    return 0;
  v2 = 0x400;
  while( true ) {
    if ((unsigned long)-a0 / v1 < v2)
      v2 = (unsigned long)-a0 / v1;
    if ((int)mincore(a0,v1 * v2,v4) <= -1) break;
    a0 += v1 * v2;
    if (!a0)
      return 0;
  }
  do {
    v3 = v2;
    if (v2 == 1)
      return a0;
    while( true ) {
      v2 = v3 + 1 >> 1;
      if ((int)mincore(a0,v1 * v2,v4) < 0) break;
      v3 >>= 1;
      a0 += v1 * v2;
      if (v3 == 1)
        return a0;
    }
  } while( true );
}

// Function: sub_158d0 @ 0x158d0
unsigned long sub_158d0(char *a0,unsigned long *a1)
{
  char *v1;
  char *v10;
  unsigned long v11; // r12
  unsigned long v12;
  char *v13; // stack - 0x98
  char *v14; // stack - 0x90
  long v15; // stack - 0x88
  char *v16; // stack - 0x70
  char *v17; // stack - 0x68
  unsigned long v18; // stack - 0x60
  char *v19; // stack - 0x58
  char *v2;
  char *v20; // stack - 0x50
  char *v3;
  int v4; // eax
  int v5; // eax
  long v6;
  void *v7; // rax
  char *v8;
  char v9 [8];
  
  v4 = open("/proc/self/maps",0);
  if (0 <= v4) {
    v16 = v9;
    v11 = 0;
    v15 = 0;
    v12 = 1;
    v17 = NULL;
    v19 = NULL;
    v20 = NULL;
    do {
      do {
        if (0x104a <= v12) { // branch-flip
          v5 = (int)read(v4,v16,v12);
          if (0 <= v5) {
            if (!v5) goto label_15af8;
            if (v12 < (unsigned long)(long)(v5 + 0x1049)) goto label_15990;
            v6 = (long)v5;
            while( true ) {
              v5 = (int)read(v4,&v16[v6],v12 - v6);
              if (v5 <= -1) break;
              if (v12 - v6 < (unsigned long)(long)(v5 + 0x1049)) goto label_15990;
              if (!v5) {
                close(v4);
                v10 = NULL;
                goto label_15b60;
              }
              v6 = v5 + v6;
            }
          }
          if (*__errno_location() != 4) {
label_15af8:
            close(v4);
            goto label_15a30;
          }
        }
        else {
label_15990:
          if (v11) { // branch-flip
            v12 *= 2;
            if (!v12) goto label_15af8;
            if (v17)
              munmap(v17,v18);
          }
          else {
            v11 = (unsigned long)getpagesize();
            for (v12 = v11; v12 <= 0x1049; v12 = v12 * 2) {
            }
          }
          v17 = (char *)mmap(0,v12,3,0x22,0xffffffff,0);
          if (v17 == (char *)0xffffffffffffffff) {
            close(v4);
            goto label_15a44;
          }
          v20 = &v17[v12];
          v19 = v17;
          v16 = v17;
          v18 = v12;
        }
      } while (0 <= (long)lseek(v4,0,0));
      close(v4);
      v4 = open("/proc/self/maps",0);
    } while (0 <= v4);
label_15a30:
    if (v17)
      munmap(v17,v18);
  }
label_15a44:
  v12 = dat_36898;
  if (!dat_36898) {
    v12 = (unsigned long)getpagesize();
    dat_36898 = v12;
  }
  v6 = ((unsigned long)a0 / v12) * v12;
  *a1 = sub_156f0(v6);
  a1[1] = sub_157d0(v6);
  v7 = sub_155b0;
label_15c43:
  a1[2] = v7;
  return 0;
label_15b60:
  if (((((int)sub_15520() < 0) || (v15 == v6)) || (v2 = &v16[v15], v15 = v15 + 1, *v2 != '-')) || ((int)sub_15520() < 0)) goto label_15a30;
  do {
    if (v6 == v15) break;
    v2 = &v16[v15];
    v15 += 1;
  } while (*v2 != '\n');
  v8 = v14;
  if (v13 <= v19) { // branch-flip
    v1 = &v14[-1];
    if (v1 < &v20[-1]) goto label_15c10;
    v3 = v10;
    if ((v13 < v19) && ((v3 = v19, v13 <= a0 && (a0 <= &v19[-1])))) {
      v14 = v19;
      goto label_15c1e;
    }
    v10 = v3;
    if (v1 <= &v20[-1]) // branch-flip
      v8 = v10;
    else if ((v20 <= a0) && (a0 <= v1)) {
      v13 = v20;
      goto label_15c1e;
    }
  }
  else {
label_15c10:
    if ((v13 <= a0) && (a0 <= &v14[-1])) {
label_15c1e:
      *a1 = v13;
      a1[1] = v14;
      a1[3] = v10;
      if (v17)
        munmap(v17,v18);
      v7 = sub_15500;
      goto label_15c43;
    }
  }
  v10 = v8;
  goto label_15b60;
}

// Function: sub_15cf0 @ 0x15cf0
void * sub_15cf0(void *a0,unsigned long a1)
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

// Function: sub_15d30 @ 0x15d30
void sub_15d30(void)
{
  __assert_fail("iter->cur.wc == 0","mbiter.h",0xa3,"mbiter_multi_next"); // no-return
}

// Function: sub_15d60 @ 0x15d60
unsigned char * sub_15d60(char *a0,int a1)
{
  unsigned char v1;
  unsigned char *v10;
  unsigned char *v11; // stack - 0x90
  char v12; // stack - 0x74
  unsigned char *v13; // stack - 0x70
  long v14; // stack - 0x68
  char v15; // stack - 0x60
  long v2;
  bool v3;
  int v4; // eax
  int v5; // eax
  unsigned char *v6; // rax
  unsigned long v7; // stack - 0x7c
  unsigned int v8 [7]; // stack - 0x5c
  unsigned char *v9;
  
  v6 = (unsigned char *)strdup(a0);
  if (!v6)
    sub_16ec0(); // no-return
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
    if (a1) {
      v13 = v6;
      v3 = 0;
      v10 = &v6[strlen((char *)v6)];
      v7 = 0;
      v9 = v6;
      while( true ) {
        v12 = 0;
        if (v9 >= v10) break;
        v12 = 0;
        if (v3) {
label_161b4:
          v14 = sub_17200(v8,v9,(long)v10 - (long)v9,&v7);
          if (v14 != -1) { // branch-flip
            if (v14 != -2) {
              if (!v14) {
                v14 = 1;
                if (*v13) {
                  v14 = 1; // return-dupe
                  __assert_fail("*iter->cur.ptr == \'\\0\'","mbiter.h",0xa2,"mbiter_multi_next"); // no-return
                }
                if (v8[0]) {
                  v14 = 1;
                  sub_15d30(); // no-return, return-dupe
                }
              }
              v15 = '\x01';
              if (mbsinit(&v7))
                v3 = 0;
              goto label_16175;
            }
            v14 = (long)v10 - (long)v13;
            v15 = '\0';
            v12 = 1;
          }
          else {
            v14 = 1;
            v15 = '\0';
            v12 = 1;
          }
label_15f53:
          v9 = v13;
          break;
        }
        if (!(*(unsigned int *)((unsigned long)(*v9 >> 5) * 4 + 0x1e8a0) >> (*v9 & 0x1f) & 1)) {
          if (mbsinit(&v7)) {
            v3 = 1;
            goto label_161b4;
          }
          __assert_fail("mbsinit (&iter->state)","mbiter.h",0x87,"mbiter_multi_next"); // return-dupe, no-return
        }
        v14 = 1;
        v8[0] = (unsigned int)(char)*v9;
        v15 = '\x01';
label_16175:
        v12 = 1;
        v5 = iswspace(v8[0]);
        if (!v5) goto label_15f53;
        v9 = &v13[v14];
        v13 = v9;
      }
      memmove(v6,v9,strlen((char *)v9) + 1);
      if (a1 == 1)
        return v6;
    }
    v13 = v6;
    v5 = 0;
    v3 = 0;
    v10 = &v6[strlen((char *)v6)];
    v7 = 0;
    v9 = v6;
    if (v6 < v10) {
      do {
        v12 = 0;
        if (v3) {
label_15eca:
          v14 = sub_17200(v8,v9,(long)v10 - (long)v9,&v7);
          if (v14 != -1) { // branch-flip
            if (v14 != -2) { // branch-flip
              if (!v14) {
                v14 = 1;
                if (*v13) {
                  v14 = 1;
                  __assert_fail("*iter->cur.ptr == \'\\0\'","mbiter.h",0xa2,"mbiter_multi_next");
                }
                if (v8[0]) {
                  v14 = 1;
                  sub_15d30();
                }
              }
              v15 = '\x01';
              if (mbsinit(&v7))
                v3 = 0;
            }
            else {
              v14 = (long)v10 - (long)v13;
              v15 = '\0';
            }
          }
          else {
            v14 = 1;
            v15 = '\0';
          }
        }
        else {
          if (!(*(unsigned int *)((unsigned long)(*v9 >> 5) * 4 + 0x1e8a0) >> (*v9 & 0x1f) & 1)) {
            if (!mbsinit(&v7))
              __assert_fail("mbsinit (&iter->state)","mbiter.h",0x87,"mbiter_multi_next");
            v3 = 1;
            goto label_15eca;
          }
          v14 = 1;
          v8[0] = (unsigned int)(char)*v9;
          v15 = '\x01';
        }
        v12 = 1;
        if (v5) { // branch-flip
          if (v5 != 1) { // branch-flip
            if ((v5 != 2) || (!v15)) goto label_15dfb;
            v4 = iswspace(v8[0]);
            if (!v4)
              v5 = 1;
          }
          else if ((((v15) && (v4 = iswspace(v8[0]), v4)) && (v15)) && (v4 = iswspace(v8[0]), v4)) {
            v11 = v13;
            v5 = 2;
          }
        }
        else if (v15) { // branch-flip
          v4 = iswspace(v8[0]);
          if (!v4) {
            if (!v15) goto label_15dfb;
            v5 = 1;
            iswspace(v8[0]);
          }
        }
        else {
label_15dfb:
          v5 = 1;
        }
        v9 = &v13[v14];
        v13 = v9;
      } while (v9 < v10);
      if (v5 == 2)
        *v11 = 0;
    }
  }
  else {
    if (a1) {
      v1 = *v6;
      v10 = v6;
      if (v1) {
        do {
          if (!(*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)v1 * 2) & 0x20)) break;
          v1 = v10[1];
          v10 = &v10[1];
        } while (v1);
      }
      memmove(v6,v10,strlen((char *)v10) + 1);
      if (a1 == 1)
        return v6;
    }
    v10 = &v6[strlen((char *)v6) - 1];
    if (v6 <= v10) {
      v2 = *(long *)__ctype_b_loc();
      do {
        if (!(*(unsigned char *)(v2 + 1 + (unsigned long)*v10 * 2) & 0x20))
          return v6;
        *v10 = 0;
        v10 = &v10[-1];
      } while (v6 <= v10);
    }
  }
  return v6;
}

// Function: sub_16300 @ 0x16300
long sub_16300(void *a0,long a1,unsigned long a2,unsigned long a3,unsigned long *a4,unsigned long a5)
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
label_1642d:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_16466:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0x1f0d8) + 0x1f0d8;
  switch(a5) {
    case 0:
      goto label_16466;
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
      goto label_1642d;
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

// Function: sub_16780 @ 0x16780
void sub_16780(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_16300(); // tail-call
}

// Function: sub_167a0 @ 0x167a0
void sub_167a0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
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
  sub_16300(a0,a1,a2,a3);
}

// Function: sub_16840 @ 0x16840
void sub_16840(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5)
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
  sub_16300(a0,a1,a2);
}

// Function: sub_16920 @ 0x16920
void sub_16920(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-diffutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU diffutils","https://www.gnu.org/software/diffutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_169d0 @ 0x169d0
void sub_169d0(unsigned long a0,unsigned long a1)
{
  if (reallocarray(NULL,a0,a1))
    return;
  sub_16ec0(); // no-return
}

// Function: sub_16a00 @ 0x16a00
void sub_16a00(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_16ec0(); // no-return
}

// Function: sub_16a40 @ 0x16a40
void sub_16a40(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_16ec0(); // no-return
}

// Function: sub_16a60 @ 0x16a60
void sub_16a60(void)
{
  sub_16a40(); // tail-call
}

// Function: sub_16a70 @ 0x16a70
void sub_16a70(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_16ec0(); // no-return
}

// Function: sub_16a90 @ 0x16a90
void sub_16a90(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_16ec0(); // no-return
}

// Function: sub_16ad0 @ 0x16ad0
void sub_16ad0(void *a0,unsigned long a1)
{
  if (realloc(a0,a1 | a1 == 0))
    return;
  sub_16ec0(); // no-return
}

// Function: sub_16b00 @ 0x16b00
void sub_16b00(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_16ec0(); // no-return
}

// Function: sub_16b40 @ 0x16b40
void sub_16b40(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!a1) || (!a2)) {
    a1 = 1;
    a2 = 1;
  }
  if (reallocarray(a0,a1,a2))
    return;
  sub_16ec0(); // no-return
}

// Function: sub_16b80 @ 0x16b80
void sub_16b80(void *a0,unsigned long *a1)
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
  sub_16ec0(); // no-return
}

// Function: sub_16c10 @ 0x16c10
void sub_16c10(void *a0,unsigned long *a1,unsigned long a2)
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
  sub_16ec0(); // no-return
}

// Function: sub_16ca0 @ 0x16ca0
void sub_16ca0(void *a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= (long)v3) goto label_16d5e;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_16d5e:
  if (!a0)
    *a1 = 0;
  if (((a2 <= v2 - v1) || ((v2 = v1 + a2, !SCARRY8(v1,a2) && (((v2 <= a3 || (a3 <= -1)) && (v3 = v2 * a4, SEXT816((long)v3) == SEXT816(v2) * SEXT816(a4))))))) && ((realloc(a0,v3) || ((a0 && (!v3)))))) {
    *a1 = v2;
    return;
  }
  sub_16ec0(); // no-return
}

// Function: sub_16da0 @ 0x16da0
void sub_16da0(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_16ec0(); // no-return
}

// Function: sub_16dc0 @ 0x16dc0
void sub_16dc0(unsigned long a0)
{
  sub_16da0(a0,1); // tail-call
}

// Function: sub_16dd0 @ 0x16dd0
void sub_16dd0(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_16ec0(); // no-return
}

// Function: sub_16df0 @ 0x16df0
void sub_16df0(unsigned long a0)
{
  sub_16dd0(a0,1); // tail-call
}

// Function: sub_16e00 @ 0x16e00
void sub_16e00(void *a0,unsigned long a1)
{
  memcpy((void *)sub_16a40(a1),a0,a1); // tail-call
}

// Function: sub_16e30 @ 0x16e30
void sub_16e30(void *a0,unsigned long a1)
{
  memcpy((void *)sub_16a70(a1),a0,a1); // tail-call
}

// Function: sub_16e60 @ 0x16e60
void sub_16e60(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_16a70(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_16ea0 @ 0x16ea0
void sub_16ea0(char *a0)
{
  sub_16e00(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_16ec0 @ 0x16ec0
void sub_16ec0(void)
{
  error(dat_260b0,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_16f00 @ 0x16f00
void sub_16f00(void)
{
  if (sub_12170())
    return;
  sub_16ec0(); // no-return
}

// Function: sub_16f20 @ 0x16f20
long sub_16f20(void)
{
  long v1; // rax
  
  v1 = sub_17850();
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_16ec0(); // no-return
  }
  return v1;
}

// Function: sub_16f50 @ 0x16f50
void sub_16f50(void)
{
  int v1; // eax
  
  v1 = sub_1acc0();
  if (!v1)
    return;
  error(dat_260b0,v1,dcgettext(NULL,"standard file descriptors",5)); // tail-call
}

// Function: sub_16f90 @ 0x16f90
int sub_16f90(void)
{
  int v1; // eax
  
  v1 = sub_1ad70();
  if (v1 < 0) {
    if (*__errno_location() == 0xc)
      sub_16ec0(); // no-return
  }
  return v1;
}

// Function: sub_16fc0 @ 0x16fc0
long sub_16fc0(void)
{
  long v1; // rax
  
  v1 = sub_1aff0();
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_16ec0(); // no-return
  }
  return v1;
}

// Function: sub_16ff0 @ 0x16ff0
long sub_16ff0(void)
{
  long v1; // rax
  
  v1 = sub_1b240();
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_16ec0(); // no-return
  }
  return v1;
}

// Function: sub_17020 @ 0x17020
void sub_17020(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,unsigned long a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
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
  sub_1b460(a8,&v2);
}

// Function: sub_170e0 @ 0x170e0
long sub_170e0(int a0,void *a1,long a2)
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

// Function: sub_17190 @ 0x17190
unsigned long sub_17190(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [16];
  char v2 [16];
  char v3 [16];
  char v4 [16];
  unsigned long v5;
  unsigned long v6; // rdx
  unsigned long v7;
  
  if (a0) {
    if (a1) {
      v5 = a0;
      v7 = a1;
      while (v6 = v5 % v7, v6) {
        v5 = v7;
        v7 = v6;
      }
      v1._8_8_ = 0;
      v1._0_8_ = v7;
      v2._8_8_ = 0;
      v2._0_8_ = a0;
      v3._8_8_ = 0;
      v3._0_8_ = a1;
      v4._8_8_ = 0;
      v4._0_8_ = SUB168(v2._0_16_ / v1._0_16_,0);
      v5 = SUB168(v3._0_16_ * v4._0_16_,0);
      if ((v5 <= a2) && (!SUB168(v3._0_16_ * v4._0_16_,8)))
        a0 = v5;
    }
    return a0;
  }
  v5 = 0x2000;
  if (a1)
    v5 = a1;
  return v5;
}

// Function: sub_17200 @ 0x17200
unsigned long sub_17200(unsigned int *a0,unsigned char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,(char *)a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_10f10(0))) {
    v1 = 1;
    *a0 = (unsigned int)*a1;
  }
  return v1;
}

// Function: sub_17290 @ 0x17290
unsigned long sub_17290(long *a0)
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

// Function: sub_17320 @ 0x17320
unsigned long * sub_17320(char *a0)
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

// Function: sub_173c0 @ 0x173c0
unsigned long sub_173c0(long *a0,char *a1) // return-dupe
{
  char *v1;
  long *v2;
  int v3; // eax
  unsigned long v4; // rax
  long v5; // rax
  char *v6;
  
  v1 = *(char **)&a1[0x30];
  if (!v1)
    return 1;
  if ((a1 <= v1) && (v1 < &a1[0x38]))
    return 1;
  v6 = (char *)((long)a0 + 9);
  if (!*v1) {
    v6 = "";
label_17458:
    *(char **)&a1[0x30] = v6;
    return 1;
  }
  do {
    v3 = strcmp(v6,v1);
    while( true ) {
      if (!v3) goto label_17458;
      if ((!*v6) && ((v6 != (char *)((long)a0 + 9) || (!(char)a0[1])))) {
        v4 = strlen(v1);
        if ((long)a0 + (0x80U - (long)v6) <= (long)(v4 + 1)) { // branch-flip
          v5 = sub_17320(v1);
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
        goto label_17458;
      }
      v6 = &v6[strlen(v6) + 1];
      if ((*v6) || (v2 = (long *)*a0, !v2)) break;
      v6 = (char *)((long)v2 + 9);
      v3 = strcmp(v6,v1);
      a0 = v2;
    }
  } while( true );
}

// Function: sub_174f0 @ 0x174f0
long * sub_174f0(long a0)
{
  long *v1;
  int v2;
  char *v3; // rax
  long *v4;
  int *v5; // rax
  
  v3 = getenv("TZ");
  if (v3) { // branch-flip
    if ((*(char *)(a0 + 8)) && (!strcmp((char *)(a0 + 9),v3)))
      return (long *)0x1;
  }
  else if (!*(char *)(a0 + 8))
    return (long *)0x1;
  v4 = (long *)sub_17320(v3);
  if (v4) {
    if (*(char *)(a0 + 8)) // branch-flip
      v2 = setenv("TZ",(char *)(a0 + 9),1);
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

// Function: sub_175f0 @ 0x175f0
void sub_175f0(long *a0) // return-dupe
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

// Function: sub_17630 @ 0x17630
void * sub_17630(long a0,void *a1,void *a2) // return-dupe x3
{
  long v1; // rax
  
  if (!a0)
    return (void *)gmtime_r(a1,a2); // tail-call
  v1 = sub_174f0(a0);
  if (!v1)
    return NULL;
  if ((!localtime_r(a1,a2)) || (!sub_173c0(a0,a2))) {
    if (v1 == 1)
      return NULL;
    sub_17290(v1);
    return NULL;
  }
  if ((v1 != 1) && (!sub_17290(v1)))
    return NULL;
  return a2;
}

// Function: sub_176e0 @ 0x176e0
unsigned long sub_176e0(long a0,unsigned long *a1) // return-dupe x3
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
    return timegm(a1); // tail-call
  v1 = sub_174f0(a0);
  if (!v1)
    return 0xffffffffffffffff;
  v3 = *a1;
  v7 = -1;
  v4 = a1[1];
  v5 = a1[2];
  v8 = *(unsigned int *)&a1[4];
  v2 = mktime(&v3);
  if ((v7 <= -1) || (!sub_173c0(a0,&v3))) {
    if (v1 == 1)
      return 0xffffffffffffffff;
    sub_17290(v1);
    return 0xffffffffffffffff;
  }
  if ((v1 != 1) && (!sub_17290(v1)))
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

// Function: sub_17830 @ 0x17830
void sub_17830(int a0,char *a1,char *a2,unsigned long a3)
{
  if (a0 == -100) {
    readlink(a1,a2,a3); // tail-call
    return;
  }
  abort(); // no-return
}

// Function: sub_17850 @ 0x17850
void sub_17850(unsigned long a0)
{
  sub_178e0(0xffffff9c,a0,0,0,0,sub_17830); // tail-call
}

// Function: sub_17870 @ 0x17870
int sub_17870(long a0,long a1)
{
  unsigned int v1;
  unsigned char v2;
  long v3; // rdx
  unsigned char v4;
  unsigned int v5;
  
  if (a0 == a1)
    return 0;
  v3 = 0;
  do {
    v4 = *(unsigned char *)(a0 + v3);
    v1 = (unsigned int)v4;
    v2 = *(unsigned char *)(a1 + v3);
    v5 = (unsigned int)v2;
    if (0x1a <= v1 - 0x41) { // branch-flip
      if (v5 - 0x41 <= 0x19) {
label_1788d:
        v5 += 0x20;
        v2 += 0x20;
      }
      if (!v1) break;
    }
    else {
      v1 += 0x20;
      v4 += 0x20;
      if (v5 - 0x41 <= 0x19) goto label_1788d;
    }
    v3 += 1;
  } while (v4 == v2);
  return v1 - v5;
}

// Function: sub_178e0 @ 0x178e0
char * sub_178e0(unsigned int a0,unsigned long a1,char *a2,unsigned long a3,unsigned long *a4,void *a5)
{
  int v1;
  long v2; // rax
  char *v3;
  int *v4; // rax
  char *v5; // rax
  void *v6; // rax
  char v7 [1032];
  unsigned long v8;
  unsigned long v9;
  
  if (!a4)
    a4 = (unsigned long *)0x259e0;
  if (a2) { // branch-flip
    v9 = 0x7fffffffffffffff;
    if (a3 <= 0x7fffffffffffffff)
      v9 = a3;
  }
  else {
    v9 = 0x400;
    a2 = v7;
  }
  v3 = a2;
  do {
    v2 = (*a5)(a0,a1,v3,v9);
    if (v2 < 0) {
      if (a2 == v3)
        return NULL;
      v4 = __errno_location();
      v1 = *v4;
      (*(void *)a4[2])(v3);
      *v4 = v1;
      return NULL;
    }
    if (v2 < (long)v9) {
      v3[v2] = 0;
      v8 = v2 + 1;
      if (v3 != v7) {
        if ((long)v9 <= (long)v8)
          return v3;
        if (a2 == v3)
          return v3;
        if (!(void *)a4[1])
          return v3;
        v5 = (char *)(*(void *)a4[1])(v3,v8);
        if (v5)
          return v5;
        return v3;
      }
      v6 = (void *)(*(void *)*a4)(v8);
      if (v6)
        return memcpy(v6,v3,v8);
      break;
    }
    if (a2 != v3)
      (*(void *)a4[2])(v3);
    if (0x3ffffffffffffffe < (long)v9) {
      *__errno_location() = 0x24;
      return NULL;
    }
    v8 = v9 * 2 + 1;
    v3 = (char *)(*(void *)*a4)(v8);
    v9 = v8;
  } while (v3);
  if ((void *)a4[3])
    (*(void *)a4[3])(v8);
  *__errno_location() = 0xc;
  return NULL;
}

// Function: sub_17ab0 @ 0x17ab0
char * sub_17ab0(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_17af0 @ 0x17af0
unsigned long sub_17af0(char *a0,long a1,unsigned long a2,unsigned long *a3)
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
      v9 = sub_12170(v9);
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
              goto label_17be8;
            }
            v3 -= *(long *)(v9 + v3 * 8);
            v2 = *(char *)(a1 + v3);
          }
          v3 += 1;
          *(unsigned long *)(v9 + v1 * 8) = v1 - v3;
label_17be8:
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
      *(unsigned long *)&v8[-8] = 0x17c3e;
      sub_121c0(v9);
      return 1;
    }
  }
  return 0;
}

// Function: sub_17c90 @ 0x17c90
void sub_17c90(char *a0)
{
  unsigned char v1;
  unsigned long v2;
  unsigned long v3; // rax
  long v4; // rax
  
  if (!*a0) {
    v1 = **(unsigned char **)&a0[0x10];
    if (*(unsigned int *)((unsigned long)(v1 >> 5) * 4 + 0x1e8a0) >> (v1 & 0x1f) & 1) {
      a0[0x18] = '\x01';
      a0[0x19] = '\0';
      a0[0x1a] = '\0';
      a0[0x1b] = '\0';
      a0[0x1c] = '\0';
      a0[0x1d] = '\0';
      a0[0x1e] = '\0';
      a0[0x1f] = '\0';
      v1 = **(unsigned char **)&a0[0x10];
      a0[0x20] = '\x01';
      *(int *)&a0[0x24] = (int)(char)v1;
      goto label_17cd1;
    }
    if (!mbsinit(&a0[4]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    *a0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v2 = *(unsigned long *)&a0[0x10];
  v4 = sub_17200(&a0[0x24],v2,sub_15cf0(v2,v3),&a0[4]);
  *(long *)&a0[0x18] = v4;
  if (v4 == -1) {
    a0[0x18] = '\x01';
    a0[0x19] = '\0';
    a0[0x1a] = '\0';
    a0[0x1b] = '\0';
    a0[0x1c] = '\0';
    a0[0x1d] = '\0';
    a0[0x1e] = '\0';
    a0[0x1f] = '\0';
    a0[0x20] = '\0';
    a0[0xc] = '\x01';
    return;
  }
  if (v4 == -2) {
    v3 = strlen(*(char **)&a0[0x10]);
    a0[0x20] = '\0';
    *(unsigned long *)&a0[0x18] = v3;
    a0[0xc] = '\x01';
    return;
  }
  if (!v4) {
    a0[0x18] = '\x01';
    a0[0x19] = '\0';
    a0[0x1a] = '\0';
    a0[0x1b] = '\0';
    a0[0x1c] = '\0';
    a0[0x1d] = '\0';
    a0[0x1e] = '\0';
    a0[0x1f] = '\0';
    if (**(char **)&a0[0x10])
      __assert_fail("*iter->cur.ptr == \'\\0\'","mbuiter.h",0xab,"mbuiter_multi_next"); // no-return
    if (*(int *)&a0[0x24])
      __assert_fail("iter->cur.wc == 0","mbuiter.h",0xac,"mbuiter_multi_next"); // no-return
  }
  a0[0x20] = '\x01';
  if (mbsinit(&a0[4])) {
    *a0 = '\0';
    a0[0xc] = '\x01';
    return;
  }
label_17cd1:
  a0[0xc] = '\x01';
  return;
}

// Function: sub_17e00 @ 0x17e00
char sub_17e00(void *a0,char *a1,unsigned long *a2)
{
  char v1;
  char v10 [32];
  char v11 [4]; // stack - 0x88
  char v12 [4]; // stack - 0xc8
  void **v13;
  char *v14; // rsp
  void **v15; // rsp
  void **v16;
  char *v17;
  unsigned long v18;
  unsigned long *v19;
  void *v2;
  unsigned long *v20; // r13
  long v21;
  void *v22;
  unsigned long *v23; // stack - 0xf0
  void *v24; // stack - 0xe8
  char v25; // stack - 0xd9
  void *v26; // stack - 0xd8
  char *v27; // stack - 0xd0
  unsigned long v28; // stack - 0xc4
  char v29; // stack - 0xbc
  char v3 [16];
  void *v30; // stack - 0xb8
  long v31; // stack - 0xb0
  char v32; // stack - 0xa8
  int v33; // stack - 0xa4
  unsigned long v34; // stack - 0x84
  char v35; // stack - 0x7c
  char *v36; // stack - 0x78
  unsigned long v37; // stack - 0x70
  char v38; // stack - 0x68
  int v39; // stack - 0x64
  unsigned long *v4;
  void *v5;
  char v6; // al
  unsigned int v7; // eax
  unsigned long *v8;
  void *v9; // stack - 0xf8
  
  v15 = &v9;
  v13 = &v9;
  v16 = &v9;
  v9 = a0;
  v23 = a2;
  v8 = (void *)sub_1b790(a1);
  v24 = v8;
  v3._8_8_ = 0;
  v3._0_8_ = v8;
  v8 = SUB168(ZEXT816(0x38) * v3._0_16_,0);
  if ((0 <= (long)v8) && (!SUB168(ZEXT816(0x38) * v3._0_16_,8))) {
    v18 = (long)v24 * 0x38;
    if (0xfa1 <= v18) { // branch-flip
      v8 = (unsigned long *)sub_12170(v18);
      v19 = v8;
      v5 = v24;
    }
    else {
      v16 = &v9;
      while (v15 != (void **)((long)&v9 - (v18 + 0x36 & 0xfffffffffffff000))) {
        v13 = (char *)((long)v16 + -0x1000);
        v14 = (char *)((long)v16 + -0x1000);
        *(unsigned long *)((long)v16 + -8) = *(unsigned long *)((long)v16 + -8);
        v15 = (char *)((long)v16 + -0x1000);
        v16 = (void **)v14;
      }
      v7 = (unsigned int)(v18 + 0x36) & 0xff0;
      v8 = (unsigned long)(unsigned long)v7;
      v16 = (char *)((long)v13 + -(long)v8);
      if (v8)
        *(unsigned long *)((long)v13 + -8) = *(unsigned long *)((long)v13 + -8);
      v19 = (unsigned long *)(((unsigned long)((long)v13 + -(long)v8 + 0xf) & 0xfffffffffffffff0) + 0x1f & 0xffffffffffffffe0);
      v5 = v24;
    }
    if (v19) {
      v8 = &v19[3];
      v11[0] = 0;
      v27 = v11;
      v34 = 0;
      v24 = v5;
      v36 = a1;
      while( true ) {
        v35 = 0;
        *(unsigned long *)((long)v16 + -8) = 0x17f8e;
        sub_17c90(v27);
        v25 = v38;
        if ((v38) && (!v39)) break;
        if (v36 != v10) // branch-flip
          v8[-3] = v36;
        else {
          *(unsigned long *)((long)v16 + -8) = 0x17fbb;
          memcpy(v8,v10,v37,*(char *)((long)v16 + -8));
          v8[-3] = v8;
        }
        v8[-2] = v37;
        *(char *)&v8[-1] = v38;
        if (v38)
          *(int *)((long)v8 + -4) = v39;
        v36 = &v36[v37];
        v8 = &v8[6];
      }
      v21 = 0;
      v19[(long)v5 * 6 + 1] = 1;
      v26 = (void *)0x2;
      v4 = v19;
      if ((void *)0x3 <= v24) {
label_18000:
        v20 = &v4[6];
        v1 = *(char *)&v4[8];
        do {
          v8 = &v19[v21 * 6];
          if ((v1) && (*(char *)&v8[2])) {
            if (*(int *)((long)v4 + 0x44) == *(int *)((long)v8 + 0x14)) {
label_18056:
              v21 += 1;
              v8 = (long)((long)v26 - v21);
              v19[(long)((long)v5 * 6 + (long)v26)] = v8;
              goto label_1806b;
            }
          }
          else {
            v18 = v4[7];
            if (v18 == v8[1]) {
              v22 = (void *)*v8;
              v2 = (void *)*v20;
              *(unsigned long *)((long)v16 + -8) = 0x18052;
              v7 = memcmp(v2,v22,v18,*(char *)((long)v16 + -8));
              if (!v7) goto label_18056;
            }
          }
          if (!v21) goto label_18260;
          v21 -= v19[(long)v5 * 6 + v21];
        } while( true );
      }
label_1808b:
      v11[0] = 0;
      v22 = NULL;
      v12[0] = 0;
      *v23 = 0;
      v28 = 0;
      v29 = '\0';
      v34 = 0;
      v35 = '\0';
      v30 = v9;
      v36 = v9;
      v6 = '\0';
      v1 = v6;
label_180e0:
      v6 = v1;
      if (!v6) goto label_1812f;
      if (!v38) goto label_18141;
label_180ea:
      if (v39) {
        v8 = &v19[(long)v22 * 6];
        if (!*(char *)&v8[2]) goto label_1814c;
        if (*(int *)((long)v8 + 0x14) != v39) goto label_18156;
        v17 = v36;
        v18 = v37;
label_18113:
        v36 = (void *)((long)v17 + v18);
        v35 = '\0';
        v22 = (void *)((long)v22 + 1);
        if (v24 != v22) {
label_1812f:
          do {
            *(unsigned long *)((long)v16 + -8) = 0x1813b;
            sub_17c90(v27);
            if (v38) goto label_180ea;
label_18141:
            v8 = &v19[(long)v22 * 6];
label_1814c:
            v18 = v37;
            if (v8[1] == v37) {
              v2 = (void *)*v8;
              v26 = v36;
              *(unsigned long *)((long)v16 + -8) = 0x181c6;
              v7 = memcmp(v2,v36,v37,*(char *)((long)v16 + -8));
              v17 = v26;
              if (!v7) goto label_18113;
            }
label_18156:
            if (v22) {
              v21 = v19[(long)((long)v5 * 6 + (long)v22)];
              v8 = (void *)((long)v22 - v21);
              v1 = v35;
              v26 = v8;
              if (!v21) goto label_180e0;
              v22 = v8;
              if (!v29) goto label_18235;
              goto label_1820e;
            }
            if (!v29) {
              *(unsigned long *)((long)v16 + -8) = 0x18170;
              sub_17c90(v12);
            }
            if ((v32) && (!v33)) goto label_1828e;
            v29 = '\0';
            v30 = (void *)((long)v30 + v31);
            v35 = '\0';
            v36 = (void *)((long)v36 + v37);
          } while( true );
        }
        *v23 = (unsigned long)v30;
      }
      *(unsigned long *)((long)v16 + -8) = 0x18289;
      sub_121c0(v19);
      v6 = v25;
      return v6;
    }
  }
  v6 = '\0';
  return v6;
label_18260:
  v19[(long)((long)v5 * 6 + (long)v26)] = v26;
label_1806b:
  v26 = (void *)((long)v26 + 1);
  v4 = v20;
  if (v24 == v26) goto label_1808b;
  goto label_18000;
label_1820e:
  v26 = v22;
  if ((v32) && (!v33)) {
label_1828e:
    *(unsigned long *)((long)v16 + -8) = 0x18293; // no-return
    abort(*(char *)((long)v16 + -8));
  }
  v29 = '\0';
  v30 = (void *)((long)v30 + v31);
  v21 -= 1;
  if (!v21) goto label_18250;
label_18235:
  *(unsigned long *)((long)v16 + -8) = 0x1823d;
  sub_17c90(v12);
  v22 = v26;
  goto label_1820e;
label_18250:
  v1 = v35;
  goto label_180e0;
}

// Function: sub_182a0 @ 0x182a0
char * sub_182a0(char *a0,char *a1) // early-return, return-dupe
{
  char *v1;
  unsigned long v10; // rdx
  char *v11; // stack - 0x190
  char v12 [4]; // stack - 0x188
  char v13 [4]; // stack - 0x88
  unsigned int v14; // stack - 0x108
  unsigned long v15; // stack - 0xc8
  char v16 [4]; // stack - 0x148
  unsigned long v17;
  long v18;
  unsigned long v19; // stack - 0x1e0
  char v2;
  unsigned char v20; // stack - 0x1d1
  unsigned long v21; // stack - 0x1a8
  unsigned long v22; // stack - 0x184
  char v23; // stack - 0x17c
  char *v24; // stack - 0x178
  unsigned long v25; // stack - 0x170
  char v26; // stack - 0x168
  int v27; // stack - 0x164
  unsigned long v28; // stack - 0x144
  char v29; // stack - 0x13c
  char v3;
  char *v30; // stack - 0x138
  long v31; // stack - 0x130
  unsigned char v32; // stack - 0x128
  int v33; // stack - 0x124
  unsigned int v34; // stack - 0x104
  unsigned int v35; // stack - 0x100
  unsigned int v36; // stack - 0xfc
  char *v37; // stack - 0xf8
  unsigned long v38; // stack - 0xf0
  unsigned int v39; // stack - 0xe8
  unsigned char v4; // al
  int v40; // stack - 0xe4
  unsigned long v41; // stack - 0xe0
  unsigned long v42; // stack - 0xd8
  unsigned long v43; // stack - 0xd0
  char v44 [5]; // stack - 0xc0
  undefined3 v45; // stack - 0xbb
  char *v46; // stack - 0xb8
  unsigned long v47; // stack - 0xb0
  unsigned long v48; // stack - 0xa8
  unsigned long v49; // stack - 0xa0
  unsigned int v5; // eax
  unsigned long v50; // stack - 0x98
  unsigned long v51; // stack - 0x90
  unsigned long v52; // stack - 0x84
  char v53; // stack - 0x7c
  char *v54; // stack - 0x78
  unsigned long v55; // stack - 0x70
  char v56; // stack - 0x68
  int v57; // stack - 0x64
  int v6; // eax
  char *v7;
  unsigned long v8;
  unsigned long v9;
  
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
    v12[0] = 0;
    v22 = 0;
    v23 = 0;
    v24 = a1;
    sub_17c90(v12);
    if ((!v26) || (v27)) {
      v20 = 1;
      v16[0] = 0;
      v28 = 0;
      v29 = '\0';
      v14 &= 0xffffff00;
      v35 = 0;
      v34 = 0;
      v19 = 0;
      v21 = 0;
      v9 = 0;
      v30 = a1;
      v37 = a0;
      do {
        v36 &= 0xffffff00;
        sub_17c90(&v14);
        if ((char)v39) { // branch-flip
          if (!v40)
            return NULL;
          v4 = 10 <= v19 & v20;
          if (v4) {
            if (v19 * 5 <= v9) { // branch-flip
label_18820:
              v18 = v9 - v21;
              if (v18) { // branch-flip
                if (!v29) goto label_1886d;
                while ((!v32 || (v33))) {
                  v29 = '\0';
                  v30 = &v30[v31];
                  v18 -= 1;
                  if (!v18) goto label_188a0;
label_1886d:
                  sub_17c90(v16);
                }
                if (v29) goto label_18930;
label_188a0:
                sub_17c90(v16);
              }
              else if (!v29) goto label_188a0;
              v20 = v32;
              if (v32) { // branch-flip
                if (!v33) {
label_18930:
                  v20 = sub_17e00(a0,a1,&v11);
                  if (v20)
                    return v11;
                }
              }
              else {
                v20 = 1;
              }
              v21 = v9;
              if (!(char)v39) goto label_185f6;
            }
            else {
              v20 = v4;
            }
          }
          v10 = v9 + 1;
          if (!v26) goto label_185f6;
          if (v40 == v27) goto label_18636;
label_18588:
          v7 = v37;
          v17 = v38;
        }
        else {
          v4 = 10 <= v19 & v20;
          if (v4) { // branch-flip
            if (v19 * 5 <= v9) goto label_18820;
            v20 = v4;
          }
          else {
          }
label_185f6:
          v17 = v38;
          v7 = v37;
          v10 = v9 + 1;
          if (v38 == v25) {
            v6 = memcmp(v37,v24,v38);
            if (!v6) {
label_18636:
              v10 = v9 + 1;
              v15 = CONCAT44(v34,v14);
              v13[0] = 0;
              v48 = CONCAT44(v40,v39);
              v53 = 0;
              v47 = v38;
              v46 = &v37[v38];
              v45 = (undefined3)(v36 >> 8);
              v44[4] = 0;
              v44._0_4_ = v35;
              v52 = 0;
              v49 = v41;
              v50 = v42;
              v51 = v43;
              v54 = a1;
              sub_17c90(v13);
              if ((v56) && (!v57))
                abort(); // no-return
              v54 = &v54[v55];
              do {
                v10 += 1;
                v53 = 0;
                sub_17c90(v13);
                if ((v56) && (!v57))
                  return v37;
                if (!v44[4])
                  sub_17c90(&v15);
                v17 = v55;
                v1 = v54;
                v9 = v47;
                v7 = v46;
                if ((char)v48) { // branch-flip
                  if (!v48._4_4_)
                    return NULL;
                  if (!v56) goto label_187ae;
                  if (v48._4_4_ != v57) goto label_18588;
                }
                else {
label_187ae:
                  if ((v47 != v55) || (v6 = memcmp(v46,v54,v47), v6)) goto label_18588;
                }
                v46 = &v7[v9];
                v44[4] = 0;
                v54 = &v1[v17];
              } while( true );
            }
          }
        }
        v19 += 1;
        v37 = &v7[v17];
        v9 = v10;
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
      v10 = 0;
      v9 = 0;
      v5 = 1;
      v7 = a1;
      do {
        v17 += 1;
        v8 = v10 + 1;
        v1 = &a0[1];
        if (v2 == v3) {
          v3 = a1[1];
          if (!v3)
            return a0;
          while( true ) {
            v18 = v8 - v10;
            if (!a0[v18])
              return NULL;
            v8 += 1;
            if (a0[v18] != v3) break;
            v3 = a1[v8 - v10];
            if (!v3)
              return a0;
          }
        }
        v10 = v8;
        if (!*v1)
          return NULL;
        v4 = 10 <= v17 & (unsigned char)v5;
        if (v4) {
          if (v17 * 5 <= v10) { // branch-flip
            if (v7) {
              v7 = &v7[strnlen(v7,v10 - v9)];
              v9 = v10;
              if (*v7) goto label_183cd;
            }
            v5 = sub_17af0(v1,a1,strlen(a1),&v11);
            if ((char)v5)
              return v11;
            v7 = NULL;
          }
          else {
label_183cd:
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

// Function: sub_18970 @ 0x18970
char * sub_18970(char *a0,int a1) // return-dupe x3
{
  unsigned char *v1; // rax
  
  v1 = (unsigned char *)sub_17ab0();
  if ((*v1 & 0xdf) != 0x55) { // branch-flip
    if (((((*v1 & 0xdf) == 0x47) && ((v1[1] & 0xdf) == 0x42)) && (v1[2] == 0x31)) && (((v1[3] == 0x38 && (v1[4] == 0x30)) && ((v1[5] == 0x33 && ((v1[6] == 0x30 && (!v1[7])))))))) {
      if (*a0 == '`')
        return (char *)0x1f18c;
      return (char *)0x1f187;
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

// Function: sub_18a50 @ 0x18a50
long * sub_18a50(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_18ac0:
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
label_18f66:
      v2 = 0;
      goto label_18f20;
    case 2:
      if (!v18) {
        v2 = 0;
        goto label_19081;
      }
      v26 = 1;
      v2 = 0;
      v13 = 0;
      v31 = 1;
      v32 = "\'";
      break;
    case 3:
      v2 = 1;
label_18f20:
      v26 = 1;
      v13 = 0;
      a4 = 2;
      v31 = 1;
      v32 = "\'";
      break;
    case 4:
      if (!v18) {
        v2 = 1;
        goto label_19081;
      }
      goto label_18f66;
    case 5:
      if (!v18) goto label_18ec1;
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
          v10 = (char *)sub_18970("`",a4);
          v37 = (char *)v10;
        }
        v10 = (char *)dcgettext(NULL,"\'",5);
        v36 = (char *)v10;
        if (v10 == (long *)0x1e72a) {
          v10 = (char *)sub_18970("\'",a4);
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
label_18bab:
  do {
label_18bbe:
    v24 = 0;
    a1 = v14;
label_18bc8:
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
          if ((a3 < v24 + v31) || (v9 = memcmp(v12,v32,v31), v9)) goto label_19530;
          if (!v26) {
            v8 = *v12;
            v19 = (unsigned long)v8;
            v22 = v19;
            if ((char)v8 <= '?') {
              switch(v8) {
                case 0:
                  goto label_1912c;
                default:
                  goto label_18e03;
                case 7:
                  goto label_19117;
                case 8:
                  goto label_190e6;
                case 9:
                  goto label_191b3;
                case 10:
                  goto label_1910d;
                case 0xb:
                  goto label_191e0;
                case 0xc:
                  goto label_190cb;
                case 0xd:
                  goto label_1901d;
                case 0x20:
                  goto label_191ed;
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
                  goto label_18db6;
                case 0x23:
                  goto label_19190;
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
                  goto label_18ce8;
                case 0x27:
                  goto label_19002;
                case 0x3f:
                  goto label_18fc1;
                
              }
            }
            v5 = v23;
            if ((char)v8 <= 'z') {
              if (v8 == 0x40) goto label_18e03;
              v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
              if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_18ce8;
              if (!((unsigned long)v10 & 0xa4000000)) goto label_195d4;
              goto label_19518;
            }
            if (v8 == 0x7d) goto label_196a0;
            if ((char)v8 <= '}') {
              v22 = 0x7b;
              if (v8 == 0x7b) goto label_194fa;
              v22 = 0x7c;
              if (v8 == 0x7c) goto label_19518;
              goto label_18e03;
            }
            if (v8 != 0x7e) goto label_18e03;
label_19654:
            if (!v24) {
              v8 = a4 == 2;
              v22 = 0x7e;
              v27 = v3;
              goto label_18e79;
            }
            v22 = 0x7e;
            goto label_19662;
          }
          goto label_18dec;
        }
label_19530:
        v8 = *v12;
        v19 = (unsigned long)v8;
        v22 = v19;
        if ('?' < (char)v8) {
          if ('z' < (char)v8) {
            if (v8 != 0x7d) { // branch-flip
              if ('~' <= (char)v8) {
                v23 = 0;
                if (v8 == 0x7e) goto label_19654;
                goto label_18e00;
              }
              v22 = 0x7b;
              v23 = 0;
              if (v8 != 0x7b) {
                v22 = 0x7c;
                v23 = 0;
                v5 = 0;
                if (v8 != 0x7c) goto label_18e03;
                goto label_19518;
              }
            }
            else {
              v22 = 0x7d;
              v23 = 0;
            }
label_194fa:
            if (a3 == 0xffffffffffffffff) goto label_196af;
label_19508:
            v5 = v23;
            if (a3 == 1) goto label_19195;
label_19518:
            v8 = a4 == 2;
            v27 = v3;
            v25 = 0;
            v23 = v5;
            goto label_18e79;
          }
          if (v8 == 0x40) goto label_18e00;
          v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
          v23 = 0;
          if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_18ce8;
          v23 = 0;
          v5 = 0;
          if ((unsigned long)v10 & 0xa4000000) goto label_19518;
label_195d4:
          if ((char)v19 == '\\') {
            if (((bool)(v2 & v26)) && (v31)) goto label_195fa;
            v17 = 0x5c;
            goto label_19022;
          }
label_18e03:
          v17 = (unsigned char)v19;
          if (v34 != 1) goto label_19383;
label_18e10:
          v10 = __ctype_b_loc();
          v22 = 1;
          v27 = (*(unsigned char *)(*v10 + 1 + v19 * 2) & 0x40) == 0;
          v28 = (bool)(v27 & v2);
          v27 = !v27;
          goto label_18e5e;
        }
        switch(v8) {
          case 0:
            goto label_1911e;
          default:
label_18e00:
            v23 = 0;
            goto label_18e03;
          case 7:
label_19117:
            v17 = 0x61;
            goto label_190f7;
          case 8:
label_190e6:
            v17 = 0x62;
            goto label_190f7;
          case 9:
            v23 = 0;
label_191b3:
            v22 = 9;
            v17 = 0x74;
            goto label_191c0;
          case 10:
label_1910d:
            v17 = 0x6e;
            break;
          case 0xb:
label_191e0:
            v17 = 0x76;
            break;
          case 0xc:
label_190cb:
            v17 = 0x66;
label_190f7:
            if (v26) {
              v2 = 1;
              goto label_1904f;
            }
            v25 = 0;
            goto label_18d6e;
          case 0xd:
label_1901d:
            v17 = 0x72;
            break;
          case 0x20:
            v23 = 0;
label_191ed:
            v22 = 0x20;
            goto label_18ce8;
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
            goto label_18db9;
          case 0x23:
            v22 = 0x23;
            v23 = 0;
            goto label_19195;
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
label_18ce8:
            v8 = 0;
            goto label_18cf8;
          case 0x27:
            v23 = 0;
            goto label_19002;
          case 0x3f:
            v23 = 0;
            goto label_18fc1;
          
        }
        goto label_19022;
      }
      v17 = *v12;
      v19 = (unsigned long)v17;
      if ('?' < (char)v17) {
        if ('{' <= (char)v17) {
          if (v17 == 0x7d) {
label_196a0:
            v22 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_19508;
label_196af:
            v5 = v23;
            if (a2[1]) goto label_19518;
            goto label_19195;
          }
          if ((char)v17 <= '}') {
            v22 = 0x7b;
            if (v17 != 0x7b) {
              if (v17 == 0x7c) goto label_18db6;
              goto label_18e03;
            }
            goto label_194fa;
          }
          v22 = 0x7e;
          if (v17 == 0x7e) goto label_19195;
          if (v34 == 1) goto label_18e10;
label_19383:
          v15 = 0;
          if (a3 == 0xffffffffffffffff) {
            v10 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v10;
          }
          v22 = 0;
          do {
            v19 = v24 + v22;
            v10 = (long)sub_17200(&v16,&a2[v19],a3 - v19,&v15);
            v7 = (long)v10;
            v25 = v27;
            if (!v10) break;
            if (v10 == (long *)0xffffffffffffffff) {
              v25 = 0;
              goto label_198eb;
            }
            if (v10 == (long *)0xfffffffffffffffe) {
              v10 = (long *)v19;
              if (v19 >= a3) goto label_19c6f;
              goto label_19c65;
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
                if ((v8 <= 0x21) && (0x20000002bU >> (v19 & 0x3f) & 1)) goto label_194bd;
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
          goto label_198eb;
        }
        if (v17 == 0x40) goto label_18e03;
        v10 = (unsigned long)(1L << (v17 + 0xbf & 0x3f));
        if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_18e70;
        if ((unsigned long)v10 & 0xa4000000) goto label_18db6;
        if (v17 != 0x5c) goto label_18e03;
        if (a4 == 2) {
          if (!v26) goto label_195fa;
          goto label_18de3;
        }
        if (!(bool)(v2 & v26 & v31 != 0)) {
          v22 = 0x5c;
          v17 = 0x5c;
          v23 = 0;
          goto label_191c0;
        }
label_195fa:
        v24 += 1;
        v25 = 0;
        v17 = 0x5c;
        v8 = v21;
        goto label_19610;
      }
      switch(v17) {
        case 0:
          if (!v2) {
            v23 = 0;
            v22 = 0;
            if (v35 & 1) goto label_19292;
            goto label_191cb;
          }
label_1911e:
          if (!v26) {
            v23 = 0;
label_1912c:
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
                goto label_18ea3;
              }
label_19a35:
              a0[v14] = 0x5c;
              v13 = v14;
              v21 = (bool)v8;
            }
            else if (v13 < a1) {
              v8 = v21;
              goto label_19a35;
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
label_19a6c:
              v17 = (unsigned char)v22;
              v25 = 0;
              v13 = v14;
              v8 = v27;
              if (!v2 || v28) goto label_18ea3;
              goto label_18cf8;
            }
            v17 = 0x30;
            v25 = 0;
            v13 = v14;
            v8 = v27;
            goto label_18ea3;
          }
          if (a4 != 2) goto label_18dec;
          goto label_1905b;
        default:
          goto label_18e03;
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
          goto label_1924f;
        case 10:
          v22 = 10;
          v17 = 0x6e;
          goto label_1924f;
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
label_1924f:
          v8 = a4 == 2 & v26;
          v23 = 0;
          if (!(bool)v8) goto label_191c0;
label_18dd1:
          a4 = 2;
label_18de3:
          if (!v29) goto label_18dec;
          goto label_1905b;
        case 0x20:
          v19 = 0x20;
          goto label_18db9;
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
label_18db6:
          v27 = 0;
label_18db9:
          v22 = v19;
          v25 = v27;
          if ((a4 == 2) && (v26)) goto label_18dd1;
          goto label_18e77;
        case 0x23:
label_19190:
          v22 = 0x23;
label_19195:
          if (!v24) {
            v19 = v22;
            goto label_18db9;
          }
label_19662:
          v8 = a4 == 2;
          v27 = v3;
          v25 = 0;
          goto label_18e79;
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
          goto label_18e70;
        case 0x27:
label_19002:
          v25 = v27;
          if (a4 != 2) {
            v8 = 0;
            v22 = 0x27;
            goto label_18e79;
          }
          if (!v26) {
            if (a1) { // branch-flip
              v14 = 0;
              v22 = a1;
              if (v33) goto label_19a8b;
            }
            else {
label_19a8b:
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
            goto label_18ea3;
          }
          goto label_18de3;
        case 0x3f:
label_18fc1:
          if (a4 == 2) {
            if (v26) goto label_18de3;
            v8 = 0;
            v17 = 0x3f;
            v25 = 0;
            goto label_18ea3;
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
                goto label_18e79;
              }
              if (v26) goto label_18dec;
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
              goto label_19a6c;
            }
          }
          v8 = 0;
          v22 = 0x3f;
          v27 = v3;
          v25 = 0;
          goto label_18e79;
        
      }
label_191c0:
      if (!v2) {
label_191cb:
        v17 = (unsigned char)v22;
        v25 = 0;
        v8 = 0;
        v27 = v3;
        if (!v26) goto label_18ea0;
        goto label_18cf8;
      }
label_19022:
      v25 = 0;
      goto label_19033;
    }
    if (!(bool)(v13 == 0 & v26 & a4 == 2)) {
      v8 = a4 == 2 & (v26 ^ 1U);
      v26 = (bool)(v26 ^ 1U);
      if ((!(bool)v8) || (v26 = (bool)v8, !v3)) {
label_19d2b:
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
        if (a1 || !v33) goto label_19d2b;
        v24 = v33;
        goto label_197bb;
      }
      v13 = 0;
      a4 = 5;
      v10 = (unsigned long)__ctype_get_mb_cur_max();
      v31 = 1;
      v34 = (unsigned long)v10;
      v32 = "\"";
      if (!(v35 & 2)) goto label_19ddb;
      v3 = 0;
      v21 = 0;
      v33 = 0;
      v2 = v4;
      v26 = v4;
      goto label_18bbe;
    }
label_194bd:
    if (v2) {
label_1905b:
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
label_19081:
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
label_197bb:
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
label_19c65:
    if (!a2[(long)v10]) break;
  }
label_19c6f:
  v25 = 0;
label_198eb:
  v19 = (unsigned long)v17;
  v27 = v25;
  if (2 <= v22) {
label_198f5:
    v20 = 0;
    v22 += v24;
    v14 = v24;
    do {
      v17 = (unsigned char)v19;
      if (v28) {
        v29 = a4 == 2;
        if (v26) goto label_18de3;
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
        if (v22 <= v24) goto label_18d81;
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
          goto label_19610;
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
label_18e5e:
  if (v28) {
    v25 = 0;
    v28 = v2;
    goto label_198f5;
  }
label_18e70:
  v22 = v19;
  v25 = v27;
label_18e77:
  v8 = a4 == 2;
  v27 = v3;
label_18e79:
  v17 = (unsigned char)v22;
  v28 = (bool)v8 == 0;
  v3 = v27;
  v8 = 0;
  if ((v28 && v2) || (v8 = 0, v26)) {
label_18cf8:
    v17 = (unsigned char)v22;
    v13 = v14;
    if (!v30) goto label_18ea3;
    if (!(*(unsigned int *)(v30 + (v22 >> 5) * 4) >> (v17 & 0x1f) & 1)) goto label_18ea3;
  }
  else {
label_18ea0:
    v8 = 0;
    v3 = v27;
label_18ea3:
    if (!v23) {
      v8 ^= 1;
      v24 += 1;
      v8 &= v21;
label_19610:
      if ((bool)v8) {
        if (v13 < a1)
          a0[v13] = 0x27;
        if (v13 + 1 < a1)
          a0[v13 + 1] = 0x27;
        v21 = 0;
        v13 += 2;
      }
      goto label_18d81;
    }
  }
label_19033:
  if (v26) {
label_1904f:
    if ((bool)(v2 & a4 == 2)) goto label_1905b;
label_18dec:
    v35 &= 0xfffffffd;
    v30 = 0;
    goto label_18ac0;
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
label_18d6e:
  if (v13 < a1)
    a0[v13] = 0x5c;
  v13 += 1;
  v24 += 1;
label_18d81:
  if (v13 < a1)
    a0[v13] = v17;
  v13 += 1;
  if (!v25)
    v4 = 0;
  goto label_18bc8;
label_19292:
  v24 += 1;
  goto label_18bc8;
label_19ddb:
  a1 = v33;
label_18ec1:
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
  goto label_18bab;
}

// Function: sub_19ee0 @ 0x19ee0
void * sub_19ee0(unsigned int a0,unsigned long a1,unsigned long a2,unsigned int *a3)
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
    v6 = dat_26100;
    if (dat_260f8 <= (int)a0) {
      v10 = (long)dat_260f8;
      v9 = (long)(int)((a0 - dat_260f8) + 1);
      if (dat_26100 != (unsigned long *)0x26110) // branch-flip
        v6 = (unsigned long *)sub_16ca0(dat_26100,&v10,v9,0x7fffffff,0x10);
      else {
        v6 = (unsigned long *)sub_16ca0(0,&v10,v9,0x7fffffff,0x10);
        *v6 = dat_26110;
        v6[1] = dat_26118;
      }
      dat_26100 = v6;
      memset(&v6[(long)dat_260f8 * 2],0,(v10 - dat_260f8) * 0x10);
      dat_260f8 = (int)v10;
    }
    v1 = &v6[(long)(int)a0 * 2];
    v3 = a3[1];
    v5 = *v1;
    v8 = (void *)v1[1];
    v7 = sub_18a50(v8,v5,a1,a2,*a3,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    if (v5 <= v7) {
      v7 += 1;
      *v1 = v7;
      if (v8 != (void *)0x368a0)
        free(v8);
      v8 = (void *)sub_16a40(v7);
      v4 = *a3;
      v1[1] = (unsigned long)v8;
      sub_18a50(v8,v7,a1,a2,v4,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    }
    *v11 = v2;
    return v8;
  }
  abort(); // no-return
}

// Function: sub_1a0d0 @ 0x1a0d0
void sub_1a0d0(long a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = 0x369a0;
  sub_16e00(a0,0x38);
  *v2 = v1;
}

// Function: sub_1a110 @ 0x1a110
unsigned int sub_1a110(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x369a0;
  return *a0;
}

// Function: sub_1a130 @ 0x1a130
void sub_1a130(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x369a0;
  *a0 = a1;
}

// Function: sub_1a150 @ 0x1a150
unsigned int sub_1a150(long a0,unsigned char a1,unsigned int a2)
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

// Function: sub_1a190 @ 0x1a190
unsigned int sub_1a190(long a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = 0x369a0;
  v1 = *(unsigned int *)(a0 + 4);
  *(unsigned int *)(a0 + 4) = a1;
  return v1;
}

// Function: sub_1a1b0 @ 0x1a1b0
void sub_1a1b0(unsigned int *a0,long a1,long a2)
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

// Function: sub_1a1f0 @ 0x1a1f0
void sub_1a1f0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x369a0;
  v2 = __errno_location();
  v1 = *v2;
  sub_18a50(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(unsigned long *)&a4[10],*(unsigned long *)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_1a270 @ 0x1a270
unsigned long sub_1a270(unsigned long a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x369a0;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_18a50(0,0,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  v4 = sub_16a40(v3 + 1);
  sub_18a50(v4,v3 + 1,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_1a360 @ 0x1a360
void sub_1a360(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_1a270(a0,a1,0,a2); // tail-call
}

// Function: sub_1a370 @ 0x1a370
void sub_1a370(void) // return-dupe
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

// Function: sub_1a410 @ 0x1a410
void sub_1a410(unsigned long a0,unsigned long a1)
{
  sub_19ee0(a0,a1,0xffffffffffffffff,0x369a0); // tail-call
}

// Function: sub_1a430 @ 0x1a430
void sub_1a430(void)
{
  sub_19ee0(); // tail-call
}

// Function: sub_1a440 @ 0x1a440
void sub_1a440(unsigned long a0)
{
  sub_19ee0(0,a0,0xffffffffffffffff,0x369a0); // tail-call
}

// Function: sub_1a460 @ 0x1a460
void sub_1a460(unsigned long a0,unsigned long a1)
{
  sub_19ee0(0,a0,a1,0x369a0); // tail-call
}

// Function: sub_1a480 @ 0x1a480
void sub_1a480(unsigned long a0,int a1,unsigned long a2)
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
    sub_19ee0(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_1a520 @ 0x1a520
void sub_1a520(unsigned long a0,int a1,unsigned long a2,unsigned long a3)
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
    sub_19ee0(a0,a2,a3,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_1a5b0 @ 0x1a5b0
void sub_1a5b0(int a0,unsigned long a1)
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
    sub_19ee0(0,a1,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_1a640 @ 0x1a640
void sub_1a640(int a0,unsigned long a1,unsigned long a2)
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
    sub_19ee0(0,a1,a2,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_1a6d0 @ 0x1a6d0
void sub_1a6d0(unsigned long a0,unsigned long a1,unsigned char a2)
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
  
  v3 = dat_369a0;
  v4 = dat_369a8;
  v9 = dat_369d0;
  v5 = dat_369b0;
  v6 = dat_369b8;
  v7 = dat_369c0;
  v8 = dat_369c8;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a2 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a2 & 0x1f)) & 1) << (a2 & 0x1f) ^ v2;
  sub_19ee0(0,a0,a1,&v3);
}

// Function: sub_1a770 @ 0x1a770
void sub_1a770(unsigned long a0,unsigned char a1)
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
  
  v3 = dat_369a0;
  v4 = dat_369a8;
  v9 = dat_369d0;
  v5 = dat_369b0;
  v6 = dat_369b8;
  v7 = dat_369c0;
  v8 = dat_369c8;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a1 & 0x1f)) & 1) << (a1 & 0x1f) ^ v2;
  sub_19ee0(0,a0,0xffffffffffffffff,&v3);
}

// Function: sub_1a810 @ 0x1a810
void sub_1a810(unsigned long a0)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_369a0;
  v3 = (unsigned int)((unsigned long)dat_369a8 >> 0x20);
  v8 = dat_369d0;
  v4 = dat_369b0;
  v5 = dat_369b8;
  v6 = dat_369c0;
  v7 = dat_369c8;
  v2 = (unsigned int)dat_369a8;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_19ee0(0,a0,0xffffffffffffffff,&v1);
}

// Function: sub_1a8a0 @ 0x1a8a0
void sub_1a8a0(unsigned long a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_369a0;
  v3 = (unsigned int)((unsigned long)dat_369a8 >> 0x20);
  v8 = dat_369d0;
  v4 = dat_369b0;
  v5 = dat_369b8;
  v6 = dat_369c0;
  v7 = dat_369c8;
  v2 = (unsigned int)dat_369a8;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_19ee0(0,a0,a1,&v1);
}

// Function: sub_1a930 @ 0x1a930
void sub_1a930(unsigned long a0,int a1,unsigned long a2)
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
    sub_19ee0(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_1a9d0 @ 0x1a9d0
void sub_1a9d0(unsigned long a0,long a1,long a2,unsigned long a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_369a8;
  v4 = dat_369b0;
  v5 = dat_369b8;
  v8 = dat_369d0;
  v2 = (unsigned int)((unsigned long)dat_369a0 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_369c0;
  v7 = dat_369c8;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_19ee0(a0,a3,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_1aa70 @ 0x1aa70
void sub_1aa70(unsigned int a0,long a1,long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_369a8;
  v4 = dat_369b0;
  v5 = dat_369b8;
  v8 = dat_369d0;
  v2 = (unsigned int)((unsigned long)dat_369a0 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_369c0;
  v7 = dat_369c8;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_19ee0(a0,a3,a4,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_1ab10 @ 0x1ab10
void sub_1ab10(long a0,long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_369a8;
  v4 = dat_369b0;
  v5 = dat_369b8;
  v8 = dat_369d0;
  v2 = (unsigned int)((unsigned long)dat_369a0 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_369c0;
  v7 = dat_369c8;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_19ee0(0,a2,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_1abb0 @ 0x1abb0
void sub_1abb0(long a0,long a1,unsigned long a2,unsigned long a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_369a8;
  v4 = dat_369b0;
  v5 = dat_369b8;
  v8 = dat_369d0;
  v2 = (unsigned int)((unsigned long)dat_369a0 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_369c0;
  v7 = dat_369c8;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_19ee0(0,a2,a3,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_1ac50 @ 0x1ac50
void sub_1ac50(void)
{
  sub_19ee0(); // tail-call
}

// Function: sub_1ac60 @ 0x1ac60
void sub_1ac60(unsigned long a0,unsigned long a1)
{
  sub_19ee0(0,a0,a1,0x260c0); // tail-call
}

// Function: sub_1ac80 @ 0x1ac80
void sub_1ac80(unsigned long a0,unsigned long a1)
{
  sub_19ee0(a0,a1,0xffffffffffffffff,0x260c0); // tail-call
}

// Function: sub_1aca0 @ 0x1aca0
void sub_1aca0(unsigned long a0)
{
  sub_19ee0(0,a0,0xffffffffffffffff,0x260c0); // tail-call
}

// Function: sub_1acc0 @ 0x1acc0
int sub_1acc0(void) // early-return x2
{
  int v1; // eax
  int v2; // ebx
  int v3; // ebp
  
  v3 = 1;
  v2 = 0;
  do {
    v1 = sub_1b520(v2,1);
    if (0 <= v1) { // branch-flip
label_1acf3:
      if (v3 == 3) {
        v1 = 0;
        return v1;
      }
    }
    else {
      if ((v2) || (v1 = __open_2("/dev/full",1), v1 < 0)) {
        v1 = __open_2("/dev/null",v2 == 0);
        if (v1 < 0) {
          v1 = *__errno_location();
          return v1;
        }
        if (3 <= v1) {
          close(v1); // return-dupe
          v1 = 0;
          return v1;
        }
        goto label_1acf3;
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

// Function: sub_1ad70 @ 0x1ad70
unsigned long sub_1ad70(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4) // return-dupe
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
label_1af12:
      if (iconv(a2,0,0,&v5,&v4) != -1) {
        if (v4)
          abort(); // no-return
        *a3 = (long)v3;
        *a4 = (unsigned long)v2;
        return 0;
      }
label_1afbe:
      if ((char *)*a3 != v3) {
        free(v3);
        return 0xffffffff;
      }
      return 0xffffffff;
    }
    if (iconv(a2,&v7,&v8,&v5,&v4) == -1) {
      if (*__errno_location() != 0x16) goto label_1afbe;
      goto label_1af12;
    }
    a1 = v8;
  } while( true );
}

// Function: sub_1aff0 @ 0x1aff0
char * sub_1aff0(char *a0,unsigned long a1) // return-dupe
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
    if (*v4 == 0x16) goto label_1b18d;
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
label_1b18d:
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

// Function: sub_1b240 @ 0x1b240
char * sub_1b240(char *a0,unsigned long a1,unsigned long a2)
{
  int v1;
  char *v2; // rax
  long v3; // rax
  int *v4; // rax
  
  if ((!*a0) || (!sub_17870(a1,a2))) {
    v2 = strdup(a0);
    if (!v2)
      *__errno_location() = 0xc;
    return v2;
  }
  v3 = iconv_open(a2,a1);
  if (v3 == -1)
    return NULL;
  v2 = (char *)sub_1aff0(a0,v3);
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

// Function: sub_1b300 @ 0x1b300
char * sub_1b300(long a0,unsigned long *a1)
{
  unsigned int v1;
  bool v10; // cf
  unsigned long v11; // stack - 0x58
  unsigned long *v12; // stack - 0x50
  char *v2;
  unsigned long v3; // rax
  char *v4;
  unsigned long v5;
  unsigned long *v6;
  unsigned long v7; // rbx
  long v8;
  char *v9;
  
  v11 = *a1;
  v12 = (unsigned long *)a1[1];
  v5 = a1[2];
  if (a0) { // branch-flip
    v7 = 0;
    v8 = a0;
    do {
      v3 = v11 & 0xffffffff;
      if (0x30 <= (unsigned int)v11) { // branch-flip
        v6 = v12;
        v12 = &v12[1];
      }
      else {
        v11 = (unsigned long)((unsigned int)v11 + 8);
        v6 = (unsigned long *)(v3 + v5);
      }
      v3 = strlen((char *)*v6);
      v10 = CARRY8(v7,v3);
      v7 += v3;
      if (v10)
        v7 = 0xffffffffffffffff;
      v8 -= 1;
    } while (v8);
    if (0x80000000 <= v7) {
      *__errno_location() = 0x4b;
      return NULL;
    }
    v4 = (char *)sub_16a40(v7 + 1);
    v9 = v4;
    do {
      v1 = (unsigned int)*a1;
      if (0x30 <= v1) { // branch-flip
        v6 = (unsigned long *)a1[1];
        a1[1] = (unsigned long)&v6[1];
      }
      else {
        v6 = (unsigned long *)((unsigned long)v1 + a1[2]);
        *(unsigned int *)a1 = v1 + 8;
      }
      v2 = (char *)*v6;
      v5 = strlen(v2);
      memcpy(v9,v2,v5);
      v9 = &v9[v5];
      a0 -= 1;
    } while (a0);
  }
  else {
    v9 = (char *)sub_16a40(1);
    v4 = v9;
  }
  *v9 = 0;
  return v4;
}

// Function: sub_1b460 @ 0x1b460
unsigned long sub_1b460(char *a0,void *a1) // early-return
{
  char v1;
  char *v2; // stack - 0x18
  long v3;
  
  v1 = *a0;
  v3 = 0;
  while( true ) {
    if (!v1)
      return sub_1b300(v3,a1); // tail-call
    if ((v1 != '%') || (a0[v3 * 2 + 1] != 's')) break;
    v3 += 1;
    v1 = a0[v3 * 2];
  }
  if (0 <= __vasprintf_chk(&v2,1,a0,a1))
    return v2;
  if (*__errno_location() != 0xc)
    return 0;
  sub_16ec0(); // no-return
}

// Function: sub_1b520 @ 0x1b520
int sub_1b520(int a0,int a1,unsigned long a2)
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
    if (0 <= dat_369d8) { // branch-flip
      v2 = fcntl(a0,0x406,a2 & 0xffffffff);
      if ((0 <= v2) || (*__errno_location() != 0x16)) {
        dat_369d8 = 1;
        return v2;
      }
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      dat_369d8 = -1;
    }
    else {
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      if (dat_369d8 != -1)
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

// Function: sub_1b790 @ 0x1b790
unsigned long sub_1b790(unsigned char *a0)
{
  bool v1;
  unsigned long v10; // stack - 0x60
  char v11; // stack - 0x58
  unsigned char *v2;
  unsigned long v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // stack - 0x74
  int v6 [9]; // stack - 0x54
  unsigned long v7; // r12
  char v8; // stack - 0x6c
  unsigned char *v9; // stack - 0x68
  
  if (__ctype_get_mb_cur_max() <= 1)
    return strlen((char *)a0); // tail-call
  v7 = 0;
  v1 = 0;
  v5 = 0;
  v9 = a0;
  do {
    v8 = 0;
    if (v1) {
label_1b86c:
      v3 = __ctype_get_mb_cur_max();
      v2 = v9;
      v4 = sub_15cf0(v9,v3);
      v10 = sub_17200(v6,v2,v4,&v5);
      if (v10 != 0xffffffffffffffff) { // branch-flip
        if (v10 != 0xfffffffffffffffe) {
          if (!v10) {
            v10 = 1;
            if (*v9)
              __assert_fail("*iter->cur.ptr == \'\\0\'","mbuiter.h",0xab,"mbuiter_multi_next"); // no-return
            if (v6[0])
              __assert_fail("iter->cur.wc == 0","mbuiter.h",0xac,"mbuiter_multi_next"); // no-return
          }
          v11 = 1;
          if (mbsinit(&v5))
            v1 = 0;
          goto label_1b848;
        }
        v10 = strlen((char *)v9);
        v11 = 0;
      }
      else {
        v10 = 1;
        v11 = 0;
      }
    }
    else {
      if (!(*(unsigned int *)((unsigned long)(*v9 >> 5) * 4 + 0x1e8a0) >> (*v9 & 0x1f) & 1)) {
        if (!mbsinit(&v5))
          __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
        v1 = 1;
        goto label_1b86c;
      }
      v10 = 1;
      v6[0] = (int)(char)*v9;
      v11 = 1;
label_1b848:
      if (!v6[0])
        return v7;
    }
    v9 = &v9[v10];
    v7 += 1;
  } while( true );
}

// Function: _DT_FINI @ 0x1b9dc
void _DT_FINI(void)
{
  return;
}
