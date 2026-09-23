// Function: _DT_INIT @ 0x4000
void _DT_INIT(void) // return-dupe
{
  if (!dat_29fd8)
    return;
  (*dat_29fd8)();
}

// Function: sub_4020 @ 0x4020
void sub_4020(void)
{
  (*dat_29ba8)(); // jump-as-call
}

// Function: free @ 0x4840
void free(void *a0)
{
  (*dat_29fb8)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x4850
void __cxa_finalize(void)
{
  (*dat_29fe8)(); // jump-as-call
}

// Function: __ctype_toupper_loc @ 0x4860
void * __ctype_toupper_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_29bb0)(); // jump-as-call
  return v1;
}

// Function: getenv @ 0x4870
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_29bb8)(); // jump-as-call
  return v1;
}

// Function: __snprintf_chk @ 0x4880
int __snprintf_chk(char *a0,unsigned long a1,int a2,unsigned long a3,char *a4,...)
{
  int v1; // eax
  
  v1 = (*dat_29bc0)(); // jump-as-call
  return v1;
}

// Function: raise @ 0x4890
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_29bc8)(); // jump-as-call
  return v1;
}

// Function: strtoimax @ 0x48a0
void strtoimax(void)
{
  (*dat_29bd0)(); // jump-as-call
}

// Function: __vfprintf_chk @ 0x48b0
int __vfprintf_chk(void *a0,int a1,char *a2,void *a3)
{
  int v1; // eax
  
  v1 = (*dat_29bd8)(); // jump-as-call
  return v1;
}

// Function: __mempcpy_chk @ 0x48c0
void * __mempcpy_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_29be0)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x48d0
void abort(void)
{
  (*dat_29be8)(); // jump-as-call
}

// Function: re_match @ 0x48e0
void re_match(void)
{
  (*dat_29bf0)(); // jump-as-call
}

// Function: __errno_location @ 0x48f0
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_29bf8)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x4900
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_29c00)(); // jump-as-call
  return v1;
}

// Function: splice @ 0x4910
void splice(void)
{
  (*dat_29c08)(); // jump-as-call
}

// Function: _exit @ 0x4920
void _exit(int a0)
{
  (*dat_29c10)(); // jump-as-call
}

// Function: strcpy @ 0x4930
char * strcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_29c18)(); // jump-as-call
  return v1;
}

// Function: __fpending @ 0x4940
unsigned long __fpending(void *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_29c20)(); // jump-as-call
  return v1;
}

// Function: toupper @ 0x4950
int toupper(int a0)
{
  int v1; // eax
  
  v1 = (*dat_29c28)(); // jump-as-call
  return v1;
}

// Function: puts @ 0x4960
int puts(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_29c30)(); // jump-as-call
  return v1;
}

// Function: ferror @ 0x4970
int ferror(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_29c38)(); // jump-as-call
  return v1;
}

// Function: qsort @ 0x4980
void qsort(void)
{
  (*dat_29c40)(); // jump-as-call
}

// Function: isatty @ 0x4990
int isatty(int a0)
{
  int v1; // eax
  
  v1 = (*dat_29c48)(); // jump-as-call
  return v1;
}

// Function: sigaction @ 0x49a0
int sigaction(int a0,void *a1,void *a2)
{
  int v1; // eax
  
  v1 = (*dat_29c50)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x49b0
void iswcntrl(void)
{
  (*dat_29c58)(); // jump-as-call
}

// Function: reallocarray @ 0x49c0
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_29c60)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x49d0
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_29c68)(); // jump-as-call
  return v1;
}

// Function: clearerr_unlocked @ 0x49e0
void clearerr_unlocked(void *a0)
{
  (*dat_29c70)(); // jump-as-call
}

// Function: write @ 0x49f0
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_29c78)(); // jump-as-call
  return v1;
}

// Function: fread_unlocked @ 0x4a00
void fread_unlocked(void)
{
  (*dat_29c80)(); // jump-as-call
}

// Function: textdomain @ 0x4a10
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_29c88)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x4a20
int fclose(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_29c90)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x4a30
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_29c98)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x4a40
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_29ca0)(); // jump-as-call
  return v1;
}

// Function: regfree @ 0x4a50
void regfree(void)
{
  (*dat_29ca8)(); // jump-as-call
}

// Function: __ctype_get_mb_cur_max @ 0x4a60
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_29cb0)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x4a70
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_29cb8)(); // jump-as-call
  return v1;
}

// Function: openat @ 0x4a80
int openat(int a0,char *a1,int a2,...)
{
  int v1; // eax
  
  v1 = (*dat_29cc0)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x4a90
void __stack_chk_fail(void)
{
  (*dat_29cc8)(); // jump-as-call
}

// Function: mmap @ 0x4aa0
void mmap(void)
{
  (*dat_29cd0)(); // jump-as-call
}

// Function: getopt_long @ 0x4ab0
int getopt_long(int a0,char **a1,char *a2,void *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_29cd8)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x4ac0
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_29ce0)(); // jump-as-call
  return v1;
}

// Function: pcre2_jit_stack_assign_8 @ 0x4ad0
void pcre2_jit_stack_assign_8(void)
{
  (*dat_29ce8)(); // jump-as-call
}

// Function: strchr @ 0x4ae0
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_29cf0)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x4af0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_29cf8)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x4b00
void lseek(void)
{
  (*dat_29d00)(); // jump-as-call
}

// Function: pcre2_compile_context_free_8 @ 0x4b10
void pcre2_compile_context_free_8(void)
{
  (*dat_29d08)(); // jump-as-call
}

// Function: __assert_fail @ 0x4b20
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_29d10)(); // jump-as-call
}

// Function: fputs @ 0x4b30
int fputs(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_29d18)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x4b40
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_29d20)(); // jump-as-call
  return v1;
}

// Function: pcre2_set_compile_extra_options_8 @ 0x4b50
void pcre2_set_compile_extra_options_8(void)
{
  (*dat_29d28)(); // jump-as-call
}

// Function: close @ 0x4b60
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_29d30)(); // jump-as-call
  return v1;
}

// Function: pcre2_set_character_tables_8 @ 0x4b70
void pcre2_set_character_tables_8(void)
{
  (*dat_29d38)(); // jump-as-call
}

// Function: closedir @ 0x4b80
int closedir(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_29d40)(); // jump-as-call
  return v1;
}

// Function: pcre2_general_context_create_8 @ 0x4b90
void pcre2_general_context_create_8(void)
{
  (*dat_29d48)(); // jump-as-call
}

// Function: regcomp @ 0x4ba0
void regcomp(void)
{
  (*dat_29d50)(); // jump-as-call
}

// Function: memchr @ 0x4bb0
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_29d58)(); // jump-as-call
  return v1;
}

// Function: read @ 0x4bc0
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_29d60)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x4bd0
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_29d68)(); // jump-as-call
  return v1;
}

// Function: pcre2_jit_stack_free_8 @ 0x4be0
void pcre2_jit_stack_free_8(void)
{
  (*dat_29d70)(); // jump-as-call
}

// Function: fputs_unlocked @ 0x4bf0
int fputs_unlocked(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_29d78)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x4c00
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_29d80)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x4c10
int ferror_unlocked(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_29d88)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x4c20
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_29d90)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x4c30
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_29d98)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x4c40
void signal(void)
{
  (*dat_29da0)(); // jump-as-call
}

// Function: dirfd @ 0x4c50
int dirfd(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_29da8)(); // jump-as-call
  return v1;
}

// Function: sigemptyset @ 0x4c60
int sigemptyset(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_29db0)(); // jump-as-call
  return v1;
}

// Function: pcre2_get_ovector_pointer_8 @ 0x4c70
void pcre2_get_ovector_pointer_8(void)
{
  (*dat_29db8)(); // jump-as-call
}

// Function: stat @ 0x4c80
int stat(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_29dc0)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x4c90
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_29dc8)(); // jump-as-call
  return v1;
}

// Function: putchar_unlocked @ 0x4ca0
int putchar_unlocked(int a0)
{
  int v1; // eax
  
  v1 = (*dat_29dd0)(); // jump-as-call
  return v1;
}

// Function: readdir @ 0x4cb0
void * readdir(void *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_29dd8)(); // jump-as-call
  return v1;
}

// Function: pcre2_set_depth_limit_8 @ 0x4cc0
void pcre2_set_depth_limit_8(void)
{
  (*dat_29de0)(); // jump-as-call
}

// Function: mincore @ 0x4cd0
void mincore(void)
{
  (*dat_29de8)(); // jump-as-call
}

// Function: wcwidth @ 0x4ce0
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_29df0)(); // jump-as-call
  return v1;
}

// Function: putc_unlocked @ 0x4cf0
int putc_unlocked(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_29df8)(); // jump-as-call
  return v1;
}

// Function: re_set_syntax @ 0x4d00
void re_set_syntax(void)
{
  (*dat_29e00)(); // jump-as-call
}

// Function: iswalnum @ 0x4d10
void iswalnum(void)
{
  (*dat_29e08)(); // jump-as-call
}

// Function: malloc @ 0x4d20
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_29e10)(); // jump-as-call
  return v1;
}

// Function: wcrtomb @ 0x4d30
void wcrtomb(void)
{
  (*dat_29e18)(); // jump-as-call
}

// Function: nl_langinfo @ 0x4d40
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_29e20)(); // jump-as-call
  return v1;
}

// Function: pcre2_jit_stack_create_8 @ 0x4d50
void pcre2_jit_stack_create_8(void)
{
  (*dat_29e28)(); // jump-as-call
}

// Function: pcre2_get_startchar_8 @ 0x4d60
void pcre2_get_startchar_8(void)
{
  (*dat_29e30)(); // jump-as-call
}

// Function: regexec @ 0x4d70
void regexec(void)
{
  (*dat_29e38)(); // jump-as-call
}

// Function: strcoll @ 0x4d80
int strcoll(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_29e40)(); // jump-as-call
  return v1;
}

// Function: re_search @ 0x4d90
void re_search(void)
{
  (*dat_29e48)(); // jump-as-call
}

// Function: pcre2_get_error_message_8 @ 0x4da0
void pcre2_get_error_message_8(void)
{
  (*dat_29e50)(); // jump-as-call
}

// Function: fchdir @ 0x4db0
int fchdir(int a0)
{
  int v1; // eax
  
  v1 = (*dat_29e58)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x4dc0
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_29e60)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x4dd0
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_29e68)(); // jump-as-call
  return v1;
}

// Function: munmap @ 0x4de0
int munmap(void *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = (*dat_29e70)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x4df0
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_29e78)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x4e00
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_29e80)(); // jump-as-call
  return v1;
}

// Function: iswspace @ 0x4e10
int iswspace(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_29e88)(); // jump-as-call
  return v1;
}

// Function: pcre2_maketables_8 @ 0x4e20
void pcre2_maketables_8(void)
{
  (*dat_29e90)(); // jump-as-call
}

// Function: mempcpy @ 0x4e30
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_29e98)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x4e40
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_29ea0)(); // jump-as-call
  return v1;
}

// Function: error @ 0x4e50
void error(int a0,int a1,char *a2,...)
{
  (*dat_29ea8)(); // jump-as-call
}

// Function: memrchr @ 0x4e60
void memrchr(void)
{
  (*dat_29eb0)(); // jump-as-call
}

// Function: pcre2_match_8 @ 0x4e70
void pcre2_match_8(void)
{
  (*dat_29eb8)(); // jump-as-call
}

// Function: open @ 0x4e80
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_29ec0)(); // jump-as-call
  return v1;
}

// Function: pcre2_compile_context_create_8 @ 0x4e90
void pcre2_compile_context_create_8(void)
{
  (*dat_29ec8)(); // jump-as-call
}

// Function: fopen @ 0x4ea0
void * fopen(char *a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_29ed0)(); // jump-as-call
  return v1;
}

// Function: perror @ 0x4eb0
void perror(char *a0)
{
  (*dat_29ed8)(); // jump-as-call
}

// Function: sysconf @ 0x4ec0
long sysconf(int a0)
{
  long v1; // rax
  
  v1 = (*dat_29ee0)(); // jump-as-call
  return v1;
}

// Function: towlower @ 0x4ed0
unsigned int towlower(unsigned int a0)
{
  unsigned int v1; // eax
  
  v1 = (*dat_29ee8)(); // jump-as-call
  return v1;
}

// Function: wctob @ 0x4ee0
void wctob(void)
{
  (*dat_29ef0)(); // jump-as-call
}

// Function: fdopendir @ 0x4ef0
void * fdopendir(int a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_29ef8)(); // jump-as-call
  return v1;
}

// Function: towupper @ 0x4f00
void towupper(void)
{
  (*dat_29f00)(); // jump-as-call
}

// Function: fstatfs @ 0x4f10
void fstatfs(void)
{
  (*dat_29f08)(); // jump-as-call
}

// Function: __cxa_atexit @ 0x4f20
void __cxa_atexit(void)
{
  (*dat_29f10)(); // jump-as-call
}

// Function: pcre2_match_context_create_8 @ 0x4f30
void pcre2_match_context_create_8(void)
{
  (*dat_29f18)(); // jump-as-call
}

// Function: getpagesize @ 0x4f40
int getpagesize(void)
{
  int v1; // eax
  
  v1 = (*dat_29f20)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x4f50
void exit(int a0)
{
  (*dat_29f28)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x4f60
int __fprintf_chk(void *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_29f30)(); // jump-as-call
  return v1;
}

// Function: pcre2_match_data_create_from_pattern_8 @ 0x4f70
void pcre2_match_data_create_from_pattern_8(void)
{
  (*dat_29f38)(); // jump-as-call
}

// Function: pcre2_config_8 @ 0x4f80
void pcre2_config_8(void)
{
  (*dat_29f40)(); // jump-as-call
}

// Function: re_compile_pattern @ 0x4f90
void re_compile_pattern(void)
{
  (*dat_29f48)(); // jump-as-call
}

// Function: pcre2_jit_compile_8 @ 0x4fa0
void pcre2_jit_compile_8(void)
{
  (*dat_29f50)(); // jump-as-call
}

// Function: fflush_unlocked @ 0x4fb0
int fflush_unlocked(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_29f58)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x4fc0
int mbsinit(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_29f60)(); // jump-as-call
  return v1;
}

// Function: sigaltstack @ 0x4fd0
void sigaltstack(void)
{
  (*dat_29f68)(); // jump-as-call
}

// Function: iswprint @ 0x4fe0
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_29f70)(); // jump-as-call
  return v1;
}

// Function: pcre2_compile_8 @ 0x4ff0
void pcre2_compile_8(void)
{
  (*dat_29f78)(); // jump-as-call
}

// Function: fstat @ 0x5000
int fstat(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_29f80)(); // jump-as-call
  return v1;
}

// Function: getc @ 0x5010
int getc(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_29f88)(); // jump-as-call
  return v1;
}

// Function: sigaddset @ 0x5020
int sigaddset(void *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_29f90)(); // jump-as-call
  return v1;
}

// Function: strstr @ 0x5030
char * strstr(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_29f98)(); // jump-as-call
  return v1;
}

// Function: fstatat @ 0x5040
int fstatat(int a0,char *a1,void *a2,int a3)
{
  int v1; // eax
  
  v1 = (*dat_29fa0)(); // jump-as-call
  return v1;
}

// Function: __ctype_tolower_loc @ 0x5050
void * __ctype_tolower_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_29fa8)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x5060
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_29fb0)(); // jump-as-call
  return v1;
}

// Function: sub_5070 @ 0x5070
undefined16 sub_5070(void)
{
  char v1 [16];
  unsigned long v2; // rax
  
  *__errno_location() = 0xc;
  v1._8_8_ = 0;
  v1._0_8_ = v2;
  return v1._0_16_ << 0x40;
}

// Function: sub_5090 @ 0x5090
unsigned long sub_5090(int a0,long a1) // early-return x2
{
  long *v1;
  int *v10; // rax
  unsigned long *v11; // rax
  char *v12;
  unsigned long v13; // rax
  unsigned long v14; // rax
  unsigned long v15; // rax
  unsigned long v16; // rax
  long v17;
  long *v18;
  long v19; // stack - 0x178
  unsigned int *v2;
  long v20; // stack - 0x180
  long v21; // stack - 0x188
  char v22 [8];
  char v23; // stack - 0x42
  long v24; // stack - 0x168
  long v25; // stack - 0xd8
  unsigned int v26; // eax
  unsigned int *v27;
  long v28; // rax
  int v29;
  unsigned long v3;
  unsigned long v30;
  long v31;
  char *v32;
  char *v33; // stack - 0x1c8
  int v34; // stack - 0x1c0
  int v35; // stack - 0x1bc
  unsigned int v36; // stack - 0x1a8
  int v37; // stack - 0x1a4
  long v38; // stack - 0x160
  unsigned int v39; // stack - 0x150
  char v4 [16];
  long v40; // stack - 0xd0
  char v41; // stack - 0x43
  char v42; // stack - 0x41
  bool v5;
  char v6;
  int v7; // eax
  unsigned int v8; // eax
  void *v9;
  
  dat_2a684 = 10;
  v21 = 0;
  v20 = 0;
  dat_2a080 = 2;
  dat_2a804 = 0xffffffff;
  dat_2a7e0 = 0x7fffffffffffffff;
  dat_2a7f8 = -1;
  dat_2a7f0 = -1;
  v19 = -1;
  dat_2a8fd = '\0';
  setlocale(6,"");
  bindtextdomain("grep","/usr/local/share/locale");
  textdomain("grep");
  sub_145d0(0x2a180);
  sub_1e1d0(sub_7a80);
  sub_dd40(0);
  dat_2a8c8 = sub_18320(0,0,sub_73d0,sub_7420,0);
  if (!dat_2a8c8)
    sub_1d4b0(); // no-return
  v36 = 0;
  v35 = 0;
  v34 = -1;
  v33 = NULL;
label_51d0:
  do {
    v29 = optind;
    v7 = sub_7aa0(a0,a1,&v19);
    v12 = optarg;
    if (v7 == -1) {
      if (dat_2a905) {
        sub_1cf80(stdout,sub_179e0(),"GNU grep","3.8",0);
        puts(dcgettext(NULL,"Written by Mike Haertel and others; see\n<https://git.sv.gnu.org/cgit/grep.git/tree/AUTHORS>.",5));
        return 0;
      }
      if (dat_2a908)
        sub_a440(0); // no-return
      if (v33) { // branch-flip
        if (!v21) {
          dat_2a686 = 0;
          dat_2a685 = '\0';
          dat_2a802 ^= 1;
          v21 = 1;
          *v33 = '\n';
        }
      }
      else {
        v31 = (long)optind;
        if (a0 <= optind) {
          sub_a440(2); // no-return, return-dupe
        }
        optind += 1;
        v12 = *(char **)(a1 + v31 * 8);
        v30 = 0;
        if ((v34 != 2) && (*v12 == '\\'))
          v30 = (unsigned long)(v12[1] == '-');
        v33 = (char *)sub_1d490(&v12[v30]);
        dat_2a8d8 = v33;
        v30 = strlen(v33);
        v33[v30] = '\n';
        v21 = sub_7bd0(v33,0,v30 + 1,"");
      }
      v31 = v21 + -1;
      v21 = v31;
      sub_184e0(dat_2a8c8);
      if (!dat_2a7a1) {
        v5 = 0;
        if (!fstat(1,&v24)) {
          if ((v39 & 0xf000) != 0x8000) { // branch-flip
            if ((v39 & 0xf000) == 0x2000) {
              if (((stat("/dev/null",&v25)) || (v38 != v40)) || (v24 != v25))
                v5 = 1;
              else {
                dat_2a7a0 = '\x01';
              }
            }
          }
          else {
            v17 = 0x24;
            v18 = &v24;
            v27 = (unsigned int *)0x2a920;
            while (v17) {
              v2 = &v27[1];
              v1 = (long *)((long)v18 + 4);
              *v27 = (int)*v18;
              v17 -= 1;
              v18 = v1;
              v27 = v2;
            }
          }
        }
        if (dat_2a7a0) { // branch-flip
label_5952:
          dat_2a80c = 0;
label_595c:
          dat_2a7e8 = 0;
          dat_2a7a2 = 1;
        }
        else if (dat_2a80c) goto label_595c;
        dat_2a803 = dat_2a7e8 | dat_2a7a2;
        if (dat_2a7f0 < 0)
          dat_2a7f0 = v19;
        if (dat_2a7f8 < 0)
          dat_2a7f8 = v19;
        if (((!dat_2a7e0) || (((!v31 && (dat_2a802)) && ((!dat_2a685 && (!dat_2a686)))))) && (dat_2a80c != 2))
          return 1;
        if (dat_2a900 == 2) {
          if ((!v5) || (!sub_1d920()))
            dat_2a900 = 0;
          else {
            dat_2a900 = (unsigned int)(isatty(1) != 0);
          }
        }
        sub_1d960();
        if (dat_2a900) {
          v12 = getenv("GREP_COLOR");
          if (v12) {
            v6 = *v12;
            if (v6) {
              v32 = v12;
              while( true ) {
                if (v6 != ';') {
                  if (!sub_da90((int)v6)) goto label_5feb;
                }
                v6 = v32[1];
                if (!v6) break;
                v32 = &v32[1];
              }
              dat_2a060 = v12;
              dat_2a068 = v12;
            }
          }
label_5feb:
          sub_7970();
          if ((dat_2a068 == v12) || (dat_2a060 == v12))
            error(0,0,dcgettext(NULL,"warning: GREP_COLOR=\'%s\' is deprecated; use GREP_COLORS=\'mt=%s\'",5),v12,v12);
        }
        sub_7520();
        v30 = 0;
        if (0 <= v34)
          v30 = (long)v34;
        if (v34 <= 2) {
          if (dat_2a687)
            sub_7d30();
          if (v34 != 2) { // branch-flip
            if (2 <= dat_2a8d0)
              v30 = (unsigned long)(int)sub_7fb0(v30 & 0xffffffff,v33,&v21);
          }
          else {
            if (dat_2a180) { // branch-flip
              v26 = 1;
              if ((!sub_7730(v33,v31)) && (v26 = 0, dat_2a687))
                v26 = (sub_7f20(v33,v31) ^ 1) & 0xff;
            }
            else {
              v26 = 0;
              if (dat_2a8d0 == 1)
                v26 = (unsigned int)dat_2a686;
            }
            v30 = 2;
            if (v26 & 1) {
              v30 = 0;
              sub_a6b0(0x2a8d8,&v21);
              v33 = dat_2a8d8;
            }
          }
        }
        v31 = v30 * 0x20;
        dat_2a878 = *(void **)(v31 + 0x29798);
        dat_2a870 = (**(void **)(v31 + 0x29790))(v33,v21,(long)*(int *)(v31 + 0x2978c),dat_2a8fd || dat_2a900);
        v23 = dat_2a684;
        v41 = 0;
        v42 = 0;
        v31 = (*dat_2a878)(dat_2a870,&v23,1,v22,0);
        dat_2a820 = (v31 == 0) == (bool)dat_2a802;
        a0 -= optind;
        if ((v36) || (2 <= a0))
          dat_2a808 = ~v36 >> 0x1f;
        else {
          dat_2a808 = -(unsigned int)(dat_2a018 == 3);
        }
        v30 = sysconf(0x1e);
        if (v30 - 1 <= 0x3ffffffffffffffa) {
          v4._8_8_ = 0;
          v4._0_8_ = v30;
          v31 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x18000)) % v4._0_16_,0);
          dat_2a858 = 0x18000;
          if (v31)
            dat_2a858 = (v30 + 0x18000) - v31;
          dat_2a858 = v30 + 8 + dat_2a858;
          dat_2a838 = v30;
          dat_2a860 = sub_1d110();
          if ((dat_2a014 & 2) && (!dat_2a884))
            dat_2a884 = 1;
          if (1 <= a0) { // branch-flip
            v26 = 1;
            v18 = (long *)(a1 + (long)optind * 8);
            v31 = *v18;
            do {
              v8 = sub_a3f0(v31);
              v31 = v18[1];
              v18 = &v18[1];
              v26 &= v8;
            } while (v31);
          }
          else {
            if ((dat_2a018 != 3) || (v35 <= 0))
              v11 = (unsigned long *)0x28f60;
            else {
              dat_2a88a = 1;
              v11 = (unsigned long *)0x28f70;
            }
            v3 = *v11;
            v26 = (unsigned int)(unsigned char)sub_a3f0(v3);
          }
          if (dat_2a889)
            return 2;
          return (unsigned long)v26;
        }
        abort(); // no-return, return-dupe
      }
      v5 = 0;
      goto label_5952;
    }
    switch(v7) {
      case 0:
      case 0x55:
        goto label_51d0;
      default:
        sub_a440(2);
      case 0x41:
        sub_7900(optarg,0x2a7f0);
        goto label_51d0;
      case 0x42:
        sub_7900(optarg,0x2a7f8);
        goto label_51d0;
      case 0x43:
        sub_7900(optarg,&v19);
        goto label_51d0;
      case 0x44:
        v29 = strcmp(optarg,"read");
        if (v29) { // branch-flip
          if (strcmp(v12,"skip")) {
            error(2,0,dcgettext(NULL,"unknown devices method",5));
            return v16;
          }
          dat_2a884 = 2;
        }
        else {
          dat_2a884 = 1;
        }
        goto label_51d0;
      case 0x45:
        v34 = sub_77b0("egrep",v34);
        goto label_51d0;
      case 0x46:
        v34 = sub_77b0("fgrep",v34);
        goto label_51d0;
      case 0x47:
        v34 = sub_77b0("grep",v34);
        goto label_51d0;
      case 0x48:
        v36 = 1;
        goto label_51d0;
      case 0x49:
label_52c3:
        dat_2a810 = 2;
        goto label_51d0;
      case 0x4c:
        dat_2a80c = 2;
        goto label_51d0;
      case 0x50:
        v34 = sub_77b0("perl",v34);
        goto label_51d0;
      case 0x52:
        dat_2a014 = 0x30a;
        goto label_5276;
      case 0x54:
        dat_2a8fc = 1;
        goto label_51d0;
      case 0x56:
        dat_2a905 = '\x01';
        goto label_51d0;
      case 0x58:
        v34 = sub_77b0(optarg,v34);
        goto label_51d0;
      case 0x5a:
        dat_2a804 = 0;
        goto label_51d0;
      case 0x61:
label_529d:
        dat_2a810 = 1;
        goto label_51d0;
      case 0x62:
        dat_2a800 = 1;
        goto label_51d0;
      case 99:
        goto label_54f0;
      case 100:
        v31 = sub_d900("--directories",optarg,0x28f80,0x20788,4,dat_2a078,1);
        dat_2a018 = *(int *)(v31 * 4 + 0x20788);
        if (dat_2a018 != 3)
          v29 = v35;
        v35 = v29;
        goto label_51d0;
      case 0x65:
        v30 = strlen(optarg);
        v17 = v21;
        v31 = v30 + 1 + (v21 - v20);
        if (1 <= v31) {
          dat_2a8d8 = (char *)sub_1d2b0(v33,&v20,v31,0xffffffffffffffff,1);
          v33 = dat_2a8d8;
          v12 = optarg;
        }
        *(char *)mempcpy(&v33[v17],v12,v30) = 10;
        v21 = sub_7bd0(v33,v17,v30 + 1 + v17,"");
        goto label_51d0;
      case 0x66:
        v29 = strcmp(optarg,"-");
        v9 = stdin;
        if ((v29) && (v9 = fopen(v12,"r"), !v9)) {
          error(2,*__errno_location(),"%s",optarg);
          return v14;
        }
        v31 = v21;
        v17 = v21;
        while( true ) {
          v28 = (v17 - v20) + 2;
          if (1 <= v28) {
            v33 = (char *)sub_1d2b0(v33,&v20,v28,0xffffffffffffffff,1);
            dat_2a8d8 = v33;
          }
          v28 = fread_unlocked(&v33[v17],1,v20 - (v17 + 1),v9);
          if (!v28) break;
          v17 += v28;
        }
        v10 = __errno_location();
        v37 = *v10;
        if (ferror_unlocked(v9)) { // branch-flip
label_5751:
          if (v37) {
            error(2,v37,"%s",optarg);
            return v15;
          }
        }
        else if (stdin != v9) { // branch-flip
          if (fclose(v9)) {
            v37 = *v10;
            goto label_5751;
          }
        }
        else {
          clearerr_unlocked(v9);
        }
        if ((v17 != v31) && (v33[v17 + -1] != '\n')) {
          v33[v17] = '\n';
          v17 += 1;
        }
        v21 = sub_7bd0(v33,v31,v17,optarg);
        goto label_51d0;
      case 0x68:
        v36 = 0xffffffff;
        goto label_51d0;
      case 0x69:
      case 0x79:
        dat_2a687 = '\x01';
        goto label_51d0;
      case 0x6c:
        dat_2a80c = 1;
        goto label_51d0;
      case 0x6d:
        v26 = sub_1d560(optarg,0,10,0x2a7e0,"");
        if (2 <= v26) {
          error(2,0,dcgettext(NULL,"invalid max count",5));
label_54f0:
          dat_2a7e8 = 1;
        }
        goto label_51d0;
      case 0x6e:
        dat_2a801 = 1;
        goto label_51d0;
      case 0x6f:
        dat_2a8fd = '\x01';
        goto label_51d0;
      case 0x71:
        dat_2a7a1 = '\x01';
        dat_2a080 = 0;
        goto label_51d0;
      case 0x72:
label_5276:
        dat_2a018 = 3;
        v35 = v29;
        goto label_51d0;
      case 0x73:
        dat_2a904 = 1;
        goto label_51d0;
      case 0x75:
        error(0,0,dcgettext(NULL,"warning: --unix-byte-offsets (-u) is obsolete",5));
        goto label_51d0;
      case 0x76:
        dat_2a802 = 1;
        goto label_51d0;
      case 0x77:
        sub_ca30();
        dat_2a686 = 1;
        goto label_51d0;
      case 0x78:
        dat_2a685 = '\x01';
        goto label_51d0;
      case 0x7a:
        dat_2a684 = 0;
        goto label_51d0;
      case 0x80:
        v29 = strcmp(optarg,"binary");
        if (!v29) {
          dat_2a810 = 0;
          goto label_51d0;
        }
        if (!strcmp(v12,"text")) goto label_529d;
        if (strcmp(v12,"without-match")) {
          error(2,0,dcgettext(NULL,"unknown binary-files type",5));
          abort();
        }
        goto label_52c3;
      case 0x81:
        if (optarg) {
          v29 = sub_ddf0(optarg,"always");
          if (((!v29) || (!sub_ddf0(v12,"yes"))) || (!sub_ddf0(v12,"force"))) {
            dat_2a900 = 1;
            goto label_51d0;
          }
          if (((!sub_ddf0(v12,"never")) || (!sub_ddf0(v12,"no"))) || (!sub_ddf0(v12,"none"))) {
            dat_2a900 = 0;
            goto label_51d0;
          }
          if (((sub_ddf0(v12,"auto")) && (sub_ddf0(v12,"tty"))) && (sub_ddf0(v12,"if-tty"))) {
            dat_2a908 = 1;
            goto label_51d0;
          }
        }
        dat_2a900 = 2;
        goto label_51d0;
      case 0x82:
        sub_147b0(optarg);
        v31 = dat_2a8a0;
        if (!dat_2a8a0) {
          v31 = sub_14d20();
          dat_2a8a0 = v31;
        }
        sub_14fb0(v31,optarg,sub_76d0(0));
        v31 = dat_2a8a8;
        if (!dat_2a8a8) {
          v31 = sub_14d20();
          dat_2a8a8 = v31;
        }
        sub_14fb0(v31,optarg,sub_76d0(1));
        break;
      case 0x83:
      case 0x86:
        v31 = 0;
        while( true ) {
          v17 = *(long *)(v31 * 8 + 0x2a8b0);
          if (!v17) {
            v17 = sub_14d20(v31);
            *(long *)(v31 * 8 + 0x2a8b0) = v17;
          }
          sub_14fb0(v17,optarg,sub_76d0(v31) | (unsigned int)(v7 == 0x86) << 0x1d);
          if (v31 == 1) break;
          v31 = 1;
        }
        break;
      case 0x84:
        v31 = 0;
        while( true ) {
          if (!*(long *)(v31 * 8 + 0x2a8b0))
            *(unsigned long *)(v31 * 8 + 0x2a8b0) = sub_14d20(v31);
          sub_76d0(v31);
          if (sub_15440(sub_14fb0)) {
            error(2,*__errno_location(),"%s",optarg);
            return v13;
          }
          if (v31 == 1) break;
          v31 = 1;
        }
        break;
      case 0x85:
        dat_2a070 = optarg;
        break;
      case 0x87:
        dat_2a7d8 = 1;
        break;
      case 0x88:
        dat_2a7d0 = optarg;
        break;
      case 0x89:
        dat_2a687 = '\0';
      
    }
  } while( true );
}

// Function: sub_61f0 @ 0x61f0
void sub_61f0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_29fc0)(sub_5090,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_6220 @ 0x6220
void sub_6220(void)
{
  return;
}

// Function: _FINI_0 @ 0x6290
void _FINI_0(void)
{
  if (!dat_2a168) {
    if (dat_29fe8)
      __cxa_finalize(dat_2a008);
    sub_6220();
    dat_2a168 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x62d0
void _INIT_0(void)
{
  return;
}

// Function: sub_62e0 @ 0x62e0
unsigned long sub_62e0(void *a0,long a1,char a2) // early-return x2
{
  void *v1;
  
  v1 = (void *)((long)a0 + a1 + -1);
  if (a1 + -1 <= -1)
    return 0;
  while( true ) {
    a0 = memchr(a0,0x5c,(long)v1 - (long)a0);
    if (!a0)
      return 0;
    if ((unsigned char)(*(char *)((long)a0 + 1) - 0x31U) <= 8) break;
    if (((int)*(char *)((long)a0 + 1) == (-(unsigned int)(a2 == '\0') & 0x24) + 0x5c) && (a0 = (void *)((long)a0 + 1), v1 == a0))
      return 0;
    a0 = (void *)((long)a0 + 1);
  }
  return 1;
}

// Function: sub_6350 @ 0x6350
void sub_6350(unsigned long *a0)
{
  char *v1;
  long v10; // r13
  char v2;
  unsigned char v3;
  char v4;
  char *v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rax
  char *v8; // rax
  void *v9; // rax
  
  v5 = (char *)sub_13730(a0[1]);
  if (v5) {
    v6 = sub_ca70(0);
    v2 = *v5;
    v1 = &v5[3];
    *a0 = v6;
    if (v2) { // branch-flip
      a0[7] = a0[7] + 1;
      v7 = strlen(v1);
      v10 = (unsigned char)v5[1] + v7 + (unsigned long)(unsigned char)v5[2];
      v8 = (char *)sub_1d110(v10);
      v4 = dat_2a684;
      *v8 = dat_2a684;
      v3 = v5[1];
      *(unsigned char *)&a0[8] = *(unsigned char *)&a0[8] | v3;
      v9 = memcpy(&v8[v3],v1,v7);
      if (v5[2])
        *(char *)((long)v9 + v7) = v4;
      sub_bab0(*a0,v8,v10);
      free(v8);
    }
    else {
      sub_bab0(v6,v1,strlen(v1));
    }
    sub_bfa0(*a0);
    sub_13ef0(v5); // tail-call
    return;
  }
}

// Function: sub_6460 @ 0x6460
unsigned int sub_6460(long a0,unsigned long a1,unsigned long a2,long a3,long a4,unsigned long a5,unsigned int a6) // return-dupe
{
  unsigned int v1;
  void *v10; // stack - 0x68
  unsigned long v11; // stack - 0x60
  unsigned long v12; // stack - 0x58
  unsigned long v13; // stack - 0x50
  long v2; // rax
  char *v3; // rax
  unsigned long *v4; // rbx
  unsigned long v5; // stack - 0x88
  unsigned long v6; // stack - 0x90
  unsigned long v7; // stack - 0x80
  unsigned long v8; // stack - 0x78
  unsigned long v9; // stack - 0x70
  
  v1 = a6;
  v5 = 0;
  v7 = 0;
  if ((dat_2a687) || ((char)a6)) {
    v10 = NULL;
    v11 = 0;
    if ((char)a6) {
      re_set_syntax(a5 | 0x2000000);
      v2 = re_compile_pattern(a1,a2,&v5);
      if (!v2) {
        regfree(&v5);
        return v1;
      }
      goto label_6507;
    }
  }
  else {
    v10 = (void *)sub_1d110(0x100);
  }
  v11 = 0;
  re_set_syntax(a5);
  v2 = re_compile_pattern(a1,a2,&v5);
  if (!v2) {
    v4 = (unsigned long *)(a3 * 0x40 + *(long *)(a0 + 0x10));
    *v4 = v5;
    v4[1] = v7;
    v4[2] = v8;
    v4[3] = v9;
    v4[4] = v10;
    v4[5] = v11;
    v4[6] = v12;
    v4[7] = v13;
    return 1;
  }
label_6507:
  free(v10);
  if ((a4 != -1) && (v3 = (char *)sub_89e0(a4,&v6), *v3)) {
    error(0,0,"%s:%td: %s",v3,v6,v2);
    return 0;
  }
  error(0,0,"%s",v2);
  return 0;
}

// Function: sub_6630 @ 0x6630
void sub_6630(unsigned long a0)
{
  error(2,0,"%s",a0);
}

// Function: sub_6660 @ 0x6660
void sub_6660(unsigned long a0)
{
  error(0,0,dcgettext(NULL,"warning: %s",5),a0); // tail-call
}

// Function: sub_6690 @ 0x6690
long sub_6690(char *a0,unsigned long a1,unsigned long a2,char a3)
{
  bool v1;
  long v10; // stack - 0x50
  long v11; // stack - 0x48
  unsigned long v12;
  char *v13;
  long v14; // r14
  long v15; // r13
  void *v16; // stack - 0xa8
  unsigned long v17; // stack - 0xa0
  char *v18; // stack - 0x98
  char *v19; // stack - 0x90
  unsigned char v2; // al
  char *v20; // stack - 0x78
  unsigned long v21; // stack - 0x70
  unsigned int v3;
  long v4; // rax
  unsigned long v5;
  char *v6; // rax
  long v7;
  long v8;
  unsigned char v9; // bl
  
  v4 = sub_1d390(1,0x48);
  v5 = sub_13f00();
  *(unsigned long *)(v4 + 8) = v5;
  if (dat_2a687)
    a2 |= 0x400000;
  sub_142e0(v5,0x2a180,a2,(unsigned int)(dat_2a684 == '\0') * 2 | (unsigned int)a2 & 0x10 | 0x2c);
  v2 = dat_2a180 ^ 1 | dat_2a182;
  v6 = &a0[a1];
  v7 = sub_1d0f0(0x40);
  *(unsigned long *)(v4 + 0x18) = 0;
  v10 = 1;
  *(long *)(v4 + 0x10) = v7 + 0x40;
  v11 = 0;
  v16 = rawmemchr(a0,10);
  v15 = 0;
  v14 = (long)v16 - (long)a0;
  v3 = sub_62e0(a0,v14,v2);
  v7 = 0;
  v1 = 0;
  v17 = 0;
  v19 = NULL;
  v8 = 1;
  v13 = a0;
  v18 = a0;
label_6829:
  v7 = (v7 - v8) + 2;
  if (1 <= v7) {
    v8 = *(long *)(v4 + 0x10);
    *(long *)(v4 + 0x10) = sub_1d2b0(v8 + -0x40,&v10,v7,0xffffffffffffffff,0x40) + 0x40;
  }
  re_set_syntax(a2);
  v5 = *(unsigned long *)(v4 + 0x18);
  if (!sub_6460(v4,v13,v14,v5,v15,a2,(v3 ^ 1) & 0xff))
    v1 = 1;
  v13 = (char *)((long)v16 + 1);
  v15 += 1;
  if ((char)v3) {
    *(long *)(v4 + 0x18) = *(long *)(v4 + 0x18) + 1;
    if (v6 < v13) goto label_6976;
    v16 = rawmemchr(v13,10);
    v14 = (long)v16 - (long)v13;
    v3 = sub_62e0(v13,v14,v2);
    v18 = v13;
label_681f:
    v7 = *(long *)(v4 + 0x18);
    v8 = v10;
    goto label_6829;
  }
  if (v6 >= v13) {
    v16 = rawmemchr(v13,10);
    v14 = (long)v16 - (long)v13;
    v3 = sub_62e0(v13,v14,v2);
    v9 = v18 < v13 & (unsigned char)v3;
    if (v9) { // branch-flip
      v12 = (long)v13 - (long)v18;
      v7 = (v17 - v11) + v12;
      if (1 <= v7)
        v19 = (char *)sub_1d2b0(v19,&v11,v7,0xffffffffffffffff,1);
      memcpy(&v19[v17],v18,v12);
      v17 += v12;
      v3 = (unsigned int)v9;
    }
    else {
    }
    goto label_681f;
  }
  v13 = v18;
label_6976:
  if (v1)
    exit(2); // no-return
  if (v13 <= v6) {
    if (v13 <= a0) { // branch-flip
      v17 = a1;
      v19 = a0;
    }
    else {
      v12 = v17 + ((long)v6 - (long)v13);
      v19 = (char *)sub_1d180(v19,v12);
      memcpy(&v19[v17],v13,(long)v6 - (long)v13);
      v17 = v12;
    }
  }
  if (!dat_2a686) {
    v13 = NULL;
    v20 = a0;
    v21 = a1;
    if (!dat_2a685) goto label_69c3;
  }
  v13 = (char *)sub_1d110(a1 + 0x2d);
  if (dat_2a685) { // branch-flip
    if (a2 & 0x2000) { // branch-flip
      *(unsigned short *)v13 = dat_1f097;
      v13[2] = dat_1f099;
      v12 = strlen(v13);
      v7 = v12 + a1;
      memcpy(&v13[v12],a0,a1);
      v6 = ")$";
    }
    else {
      *(unsigned int *)v13 = dat_1f09a;
      v12 = strlen(v13);
      v7 = v12 + a1;
      memcpy(&v13[v12],a0,a1);
      v6 = "\\)$";
    }
  }
  else if (a2 & 0x2000) { // branch-flip
    *(unsigned long *)v13 = s_1f060._0_8_;
    *(unsigned long *)&v13[8] = s_1f060._8_8_;
    *(unsigned short *)&v13[0x10] = s_1f060._16_2_;
    v13[0x12] = s_1f060[0x12];
    v12 = strlen(v13);
    v7 = v12 + a1;
    memcpy(&v13[v12],a0,a1);
    v6 = ")([^[:alnum:]_]|$)";
  }
  else {
    *(unsigned long *)v13 = s_1f080._0_8_;
    *(unsigned long *)&v13[8] = s_1f080._8_8_;
    *(unsigned int *)&v13[0x10] = s_1f080._16_4_;
    *(unsigned short *)&v13[0x14] = s_1f080._20_2_;
    v13[0x16] = s_1f080[0x16];
    v12 = strlen(v13);
    v7 = v12 + a1;
    memcpy(&v13[v12],a0,a1);
    v6 = "\\)\\([^[:alnum:]_]\\|$\\)";
  }
  v6 = strcpy(&v13[v7],v6);
  v20 = v13;
  v21 = strlen(v6) + v7;
label_69c3:
  sub_13340(v20,v21,*(unsigned long *)(v4 + 8));
  sub_6350(v4);
  sub_14250(0,0,*(unsigned long *)(v4 + 8),1);
  if (v19) {
    if ((a3) || (v5 = *(unsigned long *)(v4 + 8), !sub_13480(v5))) {
      *(long *)(v4 + 0x10) = *(long *)(v4 + 0x10) + -0x40;
      *(long *)(v4 + 0x18) = *(long *)(v4 + 0x18) + 1;
      if (!sub_6460(v4,v19,v17,0,0xffffffffffffffff,a2,0))
        abort(); // no-return
    }
    if (v20 != v19)
      free(v19);
  }
  free(v13);
  return v4;
}

// Function: sub_6c90 @ 0x6c90
long sub_6c90(long *a0,void *a1,long a2,long *a3,void *a4) // return-dupe
{
  long *v1;
  long v10;
  void *v11; // rax
  void *v12;
  int v13;
  void *v14; // rbx
  long v15 [3]; // stack - 0x58
  long v16; // stack - 0x60
  char v17; // stack - 0x69
  void *v18; // stack - 0x68
  int v19; // esi
  unsigned char *v2;
  long v20;
  void *v21;
  void *v22;
  bool v23; // zf
  void *v24; // stack - 0xf8
  void *v25; // stack - 0xe0
  long v26; // stack - 0xd8
  void *v27; // stack - 0xb0
  long v28; // stack - 0xa0
  long v3;
  char v4;
  char v5; // al
  int v6;
  long v7; // rax
  long v8;
  long v9;
  
  v4 = dat_2a684;
  v14 = (void *)(a2 + (long)a1);
  v8 = a0[1];
  v7 = sub_13460(v8);
  v5 = sub_13470(v8);
  if (v14 <= a1)
    return -1;
  v25 = a1;
  v18 = a1;
  do {
    while (a4) {
      v22 = v25;
      v25 = v14;
      v27 = a4;
label_6f59:
      v28 = (long)v25 - (long)v22;
      if (0x80000000 < v28)
        sub_1d4b0(); // no-return, return-dupe
      if (1 <= a0[3]) {
        v1 = &a0[4];
        v26 = 0;
        v24 = v25;
        v12 = (void *)((long)v25 - 1);
        v10 = (long)v25 - (long)v27;
        v8 = 0;
        v19 = (int)v22;
        do {
          while( true ) {
            v9 = v8 * 0x40;
            v20 = a0[2] + v9;
            *(unsigned char *)(v20 + 0x38) = (dat_2a684 == '\n') << 7 | *(unsigned char *)(v20 + 0x38) & 0x3f;
            v6 = re_search(v20,v22,(int)v28 + -1,(int)v27 - v19,(int)v10 + -1,v1);
            if (v6 < -1) {
              sub_1d4b0();
            }
            if (v6 != -1) break;
label_70a8:
            v8 += 1;
            if (a0[3] <= v8) goto label_7150;
          }
          v21 = (void *)((long)v6 + (long)v22);
          if (v24 < v21) goto label_70a8;
          v20 = (long)(*(int *)a0[6] - v6);
          if (a4) {
            if (dat_2a686) {
              if (!dat_2a685) goto label_700d;
              v23 = v10 + -1 != v20;
              v20 = v10;
              v21 = v27;
              if (v23) goto label_70a8;
            }
label_719a:
            if (v24 <= v21) { // branch-flip
              if (v24 == v21) {
                if (v20 <= v26)
                  v20 = v26;
                v26 = v20;
              }
            }
            else {
              v24 = v21;
              v26 = v20;
            }
            goto label_70a8;
          }
          if (!dat_2a685) {
            if (dat_2a686) {
label_700d:
              do {
                if ((!sub_cc90((long)v21 + v20,v12)) && (!sub_cca0(v22,v21,v12))) {
                  if (a4) goto label_719a;
                  goto label_7210;
                }
                v6 = (int)v21; // branch-flip
                if (1 <= v20) {
                  v3 = a0[2];
                  v2 = (unsigned char *)(v9 + v3 + 0x38);
                  *v2 = *v2 | 0x40;
                  v13 = re_match(v9 + v3,v22,(v6 + -1 + (int)v20) - (int)v27,v6 - v19,v1);
                  if (v13 < -1) {
                    sub_1d4b0();
                  }
                  if (v13 <= 0) goto label_7042;
                }
                else {
label_7042:
                  if (v12 == v21) break;
                  v20 = a0[2];
                  v2 = (unsigned char *)(v9 + v20 + 0x38);
                  *v2 = *v2 & 0xbf;
                  v6 = re_search(v9 + v20,v22,(int)v28 + -1,(v6 + 1) - v19,((int)v25 - (v6 + 1)) + -1,v1);
                  if (v6 < 0) {
                    if (v6 != -1) {
                      sub_1d4b0();
                    }
                    goto label_70a8;
                  }
                  v13 = *(int *)a0[6] - v6;
                  v21 = (void *)((long)v22 + (long)v6);
                }
                v20 = (long)v13;
              } while (v21 <= v24);
              goto label_70a8;
            }
            goto label_7210;
          }
          if (v20 == v10 + -1) goto label_7210;
          v8 += 1;
        } while (v8 < a0[3]);
label_7150:
        if (v24 < v25) {
          v28 = v26;
          v22 = v24;
          goto label_7210;
        }
      }
label_7162:
      if (v14 <= v25)
        return 0xffffffffffffffff;
    }
    v17 = '\0';
    v16 = 0;
    v12 = v14;
    if (*a0) {
      v8 = sub_c870(*a0,(long)v25 - (unsigned long)*(unsigned char *)&a0[8],(unsigned long)*(unsigned char *)&a0[8] + ((long)v14 - (long)v25),v15,1);
      if (v8 <= -1)
        return v8;
      v21 = (void *)((long)v25 + v8);
      v19 = (int)v4;
      v9 = memrchr(a1,v19,(long)v21 - (long)a1);
      v10 = v15[0];
      v22 = (void *)(v9 + 1);
      if (!v9)
        v22 = a1;
      v9 = a0[7];
      if ((v5 == '\x01') && (v9 <= v15[0])) {
        v10 = (long)v21 - (long)v22;
        if (v10 <= 0xf)
          v10 = 0x10;
        if (v8 >> 2 <= v10) { // branch-flip
          if ((long)v14 - (long)v25 >> 2 <= v10) // branch-flip
            v25 = v22;
          else {
            v12 = (void *)((long)rawmemchr((void *)((long)v25 + v10 * 4),v19) + 1);
            v25 = v22;
          }
        }
        else {
          v12 = (void *)((long)rawmemchr(v21,v19) + 1);
          v25 = v22;
        }
        goto label_6e20;
      }
      v12 = (void *)((long)rawmemchr(v21,v19) + 1);
      if (v9 <= v10) {
        v25 = v22;
        goto label_6e20;
      }
      v25 = v12;
      if ((dat_2a180 == '\x01') && (!dat_2a182)) {
        if (v18 < v22)
          v18 = v22;
        if (!sub_caf0(&v18,0,v21,v14)) goto label_73ab;
        v21 = v18;
        goto label_6eb5;
      }
label_73ab:
      v28 = (long)v25 - (long)v22;
label_7210:
      *a3 = v28;
      return (long)v22 - (long)a1;
    }
label_6e20:
    if (!v7) {
      v21 = v25;
      v22 = v25;
      v25 = v12;
label_6eb5:
      v12 = (void *)sub_13450(a0[1],v21,v25,0,&v16,&v17);
      if ((!v12) || (v25 == v12)) goto label_7162;
      if (v16)
        v22 = (void *)(memrchr(a1,(int)v4,(long)v12 - (long)a1) + 1);
      v25 = (void *)((long)rawmemchr(v12,(int)v4) + 1);
      if (v17) {
        v27 = v22;
        goto label_6f59;
      }
      goto label_73ab;
    }
    v11 = (void *)sub_13450(v7,v25,v12,0,&v16,0);
    if ((v11) && (v12 != v11)) {
      if (v16)
        v25 = (void *)(memrchr(a1,(int)v4,(long)v11 - (long)a1) + 1);
      v16 = 0;
      v21 = v25;
      v22 = v25;
      v25 = (void *)((long)rawmemchr(v11,(int)v4) + 1);
      goto label_6eb5;
    }
    v25 = v12;
    if (v14 <= v12)
      return 0xffffffffffffffff;
  } while( true );
}

// Function: sub_73d0 @ 0x73d0
unsigned long sub_73d0(long a0,unsigned long a1)
{
  unsigned char v1;
  unsigned long v2;
  unsigned char *v3;
  
  v3 = (unsigned char *)(dat_2a8d8 + -1 + a0);
  v2 = 0x3938a0565d85cb03;
  v1 = *v3;
  while (v1 != 10) {
    v3 = &v3[1];
    v2 = v2 * 0x21 ^ (unsigned long)v1;
    v1 = *v3;
  }
  return v2 % a1;
}

// Function: sub_7420 @ 0x7420
unsigned long sub_7420(long a0,long a1) // return-dupe
{
  char *v1;
  char *v2;
  char *v3;
  long v4; // rax
  long v5;
  char v6;
  
  v1 = (char *)(dat_2a8d8 + -1 + a0);
  v2 = (char *)(dat_2a8d8 + -1 + a1);
  v6 = *v1;
  if (v6 != *v2)
    return 0;
  v5 = 1;
  do {
    if (v6 == '\n')
      return 1;
    v6 = v1[v5];
    v4 = v5 + 1;
    v3 = &v2[v5];
    v5 = v4;
  } while (v6 == *v3);
  return 0;
}

// Function: sub_7470 @ 0x7470
void sub_7470(void)
{
  dat_2a060 = dat_2a068;
}

// Function: sub_7490 @ 0x7490
void sub_7490(void)
{
  dat_2a900 = 0xffffffff;
}

// Function: sub_74a0 @ 0x74a0
void sub_74a0(void)
{
  dat_2a028 = 0x1f09e;
  dat_2a020 = 0x1f0a4;
}

// Function: sub_74d0 @ 0x74d0
unsigned int sub_74d0(unsigned int a0,unsigned long a1,unsigned long a2,unsigned int a3)
{
  unsigned long v1; // rax
  unsigned int v2; // edx
  
  v2 = a0 & 0xf000;
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),(a0 & 0xb000) == 0x2000) | CONCAT31((undefined3)((unsigned int)a3 >> 8),v2 == 0xc000) | CONCAT31((undefined3)(v2 >> 8),v2 == 0x1000);
}

// Function: sub_7500 @ 0x7500
unsigned char sub_7500(unsigned char a0) // early-return
{
  if (dat_2a884 == 2)
    return 1;
  return dat_2a884 == 0 & (a0 ^ 1);
}

// Function: sub_7520 @ 0x7520
void sub_7520(void)
{
  unsigned int v1;
  unsigned int v2; // eax
  long v3; // rdx
  unsigned int v4; // esi
  
  v3 = 1;
  v2 = 1;
  v4 = 0;
  do {
    while ((*(char *)(v3 + 0x2a183) == '\x01' || (v1 = v2, v4 & 0xff & (unsigned int)v3))) {
      v3 += 1;
      if (v3 == 0x100) {
        dat_2a868 = (unsigned long)(v4 & 0xff) * 0x101010101010101;
        return;
      }
    }
    do {
      v2 = v1;
      v1 = v2 * 2;
    } while ((int)(v2 * 2) <= (int)(unsigned int)v3);
    v3 += 1;
    v4 |= v2;
  } while (v3 != 0x100);
  dat_2a868 = (unsigned long)(v4 & 0xff) * 0x101010101010101;
  return;
}

// Function: sub_7590 @ 0x7590
unsigned long * sub_7590(unsigned long *a0)
{
  char *v1;
  unsigned long v2;
  unsigned long v3;
  unsigned char v4; // al
  
  v3 = dat_2a868;
  while( true ) {
    if (!((unsigned long)a0 & 7)) {
      if (v3 & *a0) // branch-flip
        v4 = sub_c9d0((int)(char)*a0);
      else {
        do {
          a0 = &a0[1];
        } while (!(v3 & *a0));
        v4 = sub_c9d0((int)(char)*a0);
      }
      while (!(v4 & v3)) {
        v1 = (char *)((long)a0 + 1);
        a0 = (unsigned long *)((long)a0 + 1);
        v4 = sub_c9d0((int)*v1);
      }
      return a0;
    }
    v2 = *a0;
    if ((unsigned char)sub_c9d0((int)(char)v2) & v3) break;
    a0 = (unsigned long *)((long)a0 + 1);
  }
  return a0;
}

// Function: sub_7630 @ 0x7630
unsigned long sub_7630(char *a0,long a1) // return-dupe
{
  char *v1;
  long v2; // rax
  unsigned long v3; // stack - 0x28
  
  if (!dat_2a868)
    return 0;
  v1 = &a0[a1];
  v3 = 0;
  *v1 = 0xff;
  while( true ) {
    a0 = (char *)sub_7590(a0);
    if (v1 <= a0)
      return 0;
    v2 = sub_c9e0(a0,(long)v1 - (long)a0,&v3);
    if (v2 < 0) break;
    a0 = &a0[v2];
  }
  return 1;
}

// Function: sub_76d0 @ 0x76d0
int sub_76d0(char a0)
{
  return (-(unsigned int)(a0 == '\0') & 0x40000000) + 0x10000000;
}

// Function: sub_76f0 @ 0x76f0
unsigned long sub_76f0(char *a0,long a1) // return-dupe
{
  char *v1; // rsi
  
  v1 = &a0[a1];
  if (v1 <= a0)
    return 1;
  do {
    if (*a0)
      return 0;
    a0 = &a0[1];
  } while (a0 != v1);
  return 1;
}

// Function: sub_7720 @ 0x7720
unsigned int sub_7720(int a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x28) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == 0x1f);
}

// Function: sub_7730 @ 0x7730
unsigned long sub_7730(long a0,long a1) // return-dupe
{
  long v1; // rax
  long v2; // rbx
  unsigned long v3; // stack - 0x38
  
  v3 = 0;
  if (a1 <= 0)
    return 0;
  v2 = 0;
  do {
    v1 = sub_c9f0(a0 + v2,a1 - v2,&v3);
    if (v1 <= -1)
      return 1;
    v2 += v1;
  } while (v2 < a1);
  return 0;
}

// Function: sub_77b0 @ 0x77b0
char * sub_77b0(char *a0,int a1)
{
  char *v1; // rax
  unsigned long v2; // rbx
  
  v2 = 0;
  while( true ) {
    if (!strcmp(a0,(char *)(v2 * 0x20 + 0x29780))) break;
    v2 += 1;
    if (v2 == 7) {
      v1 = dcgettext(NULL,"invalid matcher %s",5);
      error(2,0,v1,a0);
      return v1;
    }
  }
  if ((0 <= a1) && ((int)v2 != a1)) {
    v1 = dcgettext(NULL,"conflicting matchers specified",5);
    error(2,0,v1);
    return v1;
  }
  v1 = (char *)(v2 & 0xffffffff);
  return v1;
}

// Function: sub_7860 @ 0x7860
void sub_7860(long a0,long a1)
{
  if (!SCARRY8(a0,a1))
    return;
  error(2,0,dcgettext(NULL,"input is too large to count",5));
}

// Function: sub_7890 @ 0x7890
unsigned long sub_7890(char *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  a0[a1] = '\0';
  v1 = strlen(a0);
  return CONCAT71((undefined7)(v1 >> 8),a1 != v1);
}

// Function: sub_78b0 @ 0x78b0
void sub_78b0(char *a0,char *a1,char a2)
{
  unsigned long v1; // rax
  char *v2;
  
  if (a2) {
    while( true ) {
      *a1 = '\0';
      v1 = strlen(a0);
      *a1 = a2;
      v2 = &a0[v1];
      if (a1 == &a0[v1]) break;
      do {
        a0 = &v2[1];
        *v2 = a2;
        v2 = a0;
      } while (!*a0);
    }
    return;
  }
}

// Function: sub_7900 @ 0x7900
void sub_7900(unsigned long a0,long *a1)
{
  if (((unsigned int)sub_1d560(a0,0,10,a1,"") <= 1) && (0 <= *a1))
    return;
  error(2,0,"%s: %s",a0,dcgettext(NULL,"invalid context length argument",5));
}

// Function: sub_7970 @ 0x7970
void sub_7970(void)
{
  char v1;
  char *v2;
  char *v3;
  long v4; // rbx
  char *v5; // r12
  char *v6;
  
  v2 = getenv("GREP_COLORS");
  if ((v2) && (*v2)) {
    v3 = (char *)sub_1d490(v2);
    v6 = NULL;
    v2 = v3;
label_79dc:
    do {
      while( true ) {
        v5 = &v2[1];
        v1 = *v2;
        if ((v1 != ':') && (v1)) break;
        *v2 = '\0';
        v4 = 0x29600;
        v2 = "mt";
        do {
          if (!strcmp(v2,v3)) break;
          v2 = *(char **)(v4 + 0x18);
          v4 += 0x18;
        } while (v2);
        if ((v6) && (*(unsigned long **)(v4 + 8)))
          **(unsigned long **)(v4 + 8) = v6;
        if (*(void **)(v4 + 0x10))
          (**(void **)(v4 + 0x10))();
        if (!v1)
          return;
        v6 = NULL;
        v3 = v5;
        v2 = v5;
      }
      if (v1 == '=') {
        if (v3 == v2)
          return;
        if (v6)
          return;
        *v2 = '\0';
        v6 = v5;
        v2 = v5;
        goto label_79dc;
      }
      v2 = v5;
    } while (((!v6) || (v1 == ';')) || (sub_da90((int)v1)));
  }
}

// Function: sub_7a80 @ 0x7a80
void sub_7a80(void)
{
  if (dat_2a8c0)
    return;
  sub_de70(); // tail-call
}

// Function: sub_7aa0 @ 0x7aa0
int sub_7aa0(int a0,char **a1,unsigned long a2)
{
  int v1;
  char v2; // al
  int v3; // eax
  char *v4;
  char v5 [21]; // stack - 0x68
  char v6 [3];
  char v7; // stack - 0x81
  char *v8; // stack - 0x80
  
  v7 = '\0';
  v8 = v5;
  do {
    v1 = optind;
    v3 = getopt_long(a0,a1,"0123456789A:B:C:D:EFGHIPTUVX:abcd:e:f:hiLlm:noqRrsuvwxyZz",(void *)0x28fa0,NULL);
    v2 = sub_da90(v3);
    if (!v2) {
      if (v8 != v5) {
label_7b75:
        *v8 = '\0';
        sub_7900(v5,a2);
      }
      return v3;
    }
    if ((dat_2a010 != v1) || (!v7))
      v4 = v5;
    else {
      v4 = &v8[-(unsigned long)(v5[0] == '0')];
      if (&v8[-(unsigned long)(v5[0] == '0')] == v6) {
        builtin_strncpy(&v8[-(unsigned long)(v5[0] == '0')],"...",4);
        v8 = &v6[3];
        goto label_7b75;
      }
    }
    v8 = &v4[1];
    *v4 = (char)v3;
    v7 = v2;
    dat_2a010 = v1;
  } while( true );
}

// Function: sub_7bd0 @ 0x7bd0
long sub_7bd0(long a0,long a1,long a2,long a3)
{
  void *v1;
  long v2;
  long *v3;
  int v4; // eax
  long v5;
  void *v6;
  long v7; // rbp
  void *v8;
  int v9;
  
  v6 = (void *)(a0 + a1);
  if (v6 < (void *)(a2 + a0)) {
    v7 = 1;
    v8 = v6;
    v9 = 0;
    do {
      v1 = (void *)((long)rawmemchr(v8,10) + 1);
      memmove(v6,v8,(long)v1 - (long)v8);
      a1 = (long)v6 - a0;
      v4 = sub_18750(dat_2a8c8,a1 + 1,0);
      if (v4) {
        if (v4 < 0)
          sub_1d4b0(); // no-return
        v6 = (void *)((long)v6 + ((long)v1 - (long)v8));
        if (!v9) {
          v5 = dat_2a8f0;
          if (dat_2a8e0 == dat_2a8e8) {
            v5 = sub_1d2b0(dat_2a8f0,0x2a8e8,1,0xffffffffffffffff,0x18);
            dat_2a8f0 = v5;
          }
          v2 = dat_2a8e0 + 1;
          v3 = (long *)(v5 + dat_2a8e0 * 0x18);
          v3[1] = a3;
          *v3 = dat_2a8d0;
          v3[2] = v7;
          dat_2a8e0 = v2;
        }
        dat_2a8d0 += 1;
        a1 = (long)v6 - a0;
      }
      v7 += 1;
      v8 = v1;
      v9 = v4;
    } while (v1 < (void *)(a2 + a0));
  }
  return a1;
}

// Function: sub_7d30 @ 0x7d30
void sub_7d30(void)
{
  unsigned int v1;
  char v2; // al
  int v3; // eax
  char v4 [128];
  unsigned long v5; // stack - 0xe0
  char v6 [24];
  int *v7; // r13
  char *v8; // r14
  char *v9;
  
  v8 = (char *)0x2a6a0;
  v7 = (int *)0x2a284;
  do {
    if (*v7 != -1) {
      v3 = sub_146b0(*v7,v4);
      if (1 <= v3) {
        v9 = &v4[(long)v3 * 4];
        do {
          v1 = *(unsigned int *)&v9[-4];
          v5 = 0;
          if (wcrtomb(v6,v1,&v5) != 1) {
            v2 = 0xff;
            goto label_7dfc;
          }
          v9 = &v9[-4];
        } while (v9 != &v4[(unsigned long)(unsigned int)(v3 - 1) * -4 + (long)v3 * 4 + -4]);
      }
      v2 = 1;
label_7dfc:
      *v8 = v2;
    }
    v7 = &v7[1];
    v8 = &v8[1];
    if (v7 == (int *)0x2a684)
      return;
  } while( true );
}

// Function: sub_7e40 @ 0x7e40
unsigned long sub_7e40(unsigned char *a0,unsigned long a1,unsigned long a2)
{
  long *v1; // rax
  unsigned int v2; // stack - 0xbc
  char v3 [136];
  unsigned long v4;
  unsigned long v5; // rax
  
  if (*(int *)((unsigned long)*a0 * 4 + 0x2a284) != -1) // branch-flip
    v5 = (unsigned long)*(char *)((unsigned long)*a0 + 0x2a6a0);
  else {
    v5 = sub_18c40(&v2,a0,a1,a2);
    if ((v5 <= 0x10) && (!sub_146b0(v2,v3))) {
      v4 = v5;
      do {
        v4 -= 1;
        if ((long)v4 <= 0)
          return v5;
        v1 = __ctype_toupper_loc();
      } while ((unsigned int)a0[v4] == *(unsigned int *)(*v1 + (unsigned long)a0[v4] * 4));
    }
    v5 = 0xffffffffffffffff;
  }
  return v5;
}

// Function: sub_7f20 @ 0x7f20
unsigned long sub_7f20(long a0,long a1) // return-dupe
{
  int v1; // eax
  unsigned long v2; // stack - 0x38
  long v3; // r13
  
  v2 = 0;
  if (a1 <= 0)
    return 1;
  v3 = 0;
  do {
    v1 = sub_7e40(a0 + v3,a1 - v3,&v2);
    if (v1 <= -1)
      return 0;
    v3 += v1;
  } while (v3 < a1);
  return 1;
}

// Function: sub_7fb0 @ 0x7fb0
int sub_7fb0(int a0,unsigned char *a1,unsigned long *a2)
{
  unsigned char v1;
  void *v2; // rax
  unsigned long v3;
  void *v4;
  unsigned long v5; // stack - 0x48
  unsigned long v6;
  unsigned char *v7; // r15
  unsigned char *v8;
  
  v6 = *a2;
  v2 = (void *)sub_1d110(v6 + 1);
  v5 = 0;
  v4 = v2;
  v8 = a1;
  while (v6) {
    v1 = *v8;
    if (v1 == 0x5b) {
      free(v2); // return-dupe
      return a0;
    }
    if ('\\' <= (char)v1) { // branch-flip
      if (v1 != 0x5c) { // branch-flip
        if (v1 == 0x5e) {
          free(v2);
          return a0;
        }
        if ((unsigned char)(v1 + 0x85) <= 1) goto label_80e9;
      }
      else if (2 <= (long)v6) {
        v1 = v8[1];
        if ('=' <= (char)v1) { // branch-flip
          if ((unsigned char)(v1 - 0x3e) <= 0x3e) {
            v3 = 1L << (v1 - 0x3e & 0x3f);
            if (v3 & 0x220001402200011) {
              free(v2);
              return a0;
            }
            v3 &= 0x6000000000000002;
label_81a9:
            if ((v3) && (!a0)) {
              free(v2);
              return a0;
            }
          }
        }
        else if ('\n' <= (char)v1) {
          v3 = 1L << (v1 & 0x3f);
          if (!(v3 & 0x13fe008000000400)) {
            v3 &= 0xb0000000000;
            goto label_81a9;
          }
          free(v2);
          return a0;
        }
        v8 = &v8[1];
        v6 -= 1;
      }
    }
    else if ('/' <= (char)v1) { // branch-flip
      if (v1 == 0x3f) {
label_80e9:
        if (a0) {
          free(v2);
          return a0;
        }
      }
    }
    else if ('$' <= (char)v1) {
      v3 = 1L << (v1 & 0x3f);
      if (v3 & 0x441000000000) {
        free(v2);
        return a0;
      }
      if (v3 & 0x90000000000) goto label_80e9;
    }
    if (dat_2a687) // branch-flip
      v3 = sub_7e40(v8,v6,&v5);
    else {
      v3 = sub_c9f0(v8,v6,&v5);
    }
    if ((long)v3 < 0) {
      free(v2);
      return a0;
    }
    v7 = &v8[v3];
    v4 = mempcpy(v4,v8,v3);
    v6 -= v3;
    v8 = v7;
  }
  a0 = 2;
  v6 = (long)v4 - (long)v2;
  if (*a2 != v6) {
    *a2 = v6;
    *(char *)mempcpy(a1,v2,v6) = 10;
  }
  free(v2);
  return a0;
}

// Function: sub_81e0 @ 0x81e0
unsigned long sub_81e0(unsigned long a0,unsigned char a1,unsigned int a2)
{
  long v1;
  unsigned long v2; // rax
  
  if ((char)a2) { // branch-flip
    if (dat_2a018 == 4)
      return (unsigned long)a2;
    if ((a1) && (dat_2a88a))
      return 0;
    v1 = *(long *)((unsigned long)a1 * 8 + 0x2a8a0);
  }
  else {
    v1 = *(long *)((unsigned long)a1 * 8 + 0x2a8b0);
  }
  if (!v1)
    return 0;
  v2 = sub_14ef0(v1,a0); // tail-call
  return v2;
}

// Function: sub_8250 @ 0x8250
void sub_8250(void *a0)
{
  void *v1;
  long v2; // rbp
  int v3; // r12d
  
  if (a0 <= dat_2a7c0) // branch-flip
    v2 = 0;
  else {
    v3 = (int)dat_2a684;
    v2 = 0;
    v1 = dat_2a7c0;
    while( true ) {
      v1 = memchr(v1,v3,(long)a0 - (long)v1);
      if (!v1) break;
      v1 = (void *)((long)v1 + 1);
      v2 += 1;
      if (a0 <= v1) break;
    }
  }
  dat_2a7c0 = a0;
  dat_2a818 = sub_7860(dat_2a818,v2);
}

// Function: sub_82c0 @ 0x82c0
void sub_82c0(char *a0)
{
  if (!*a0)
    return;
  sub_1d970(dat_2a028,a0); // tail-call
}

// Function: sub_82e0 @ 0x82e0
void sub_82e0(void)
{
  if (!dat_2a900)
    return;
  sub_82c0(); // tail-call
}

// Function: sub_8300 @ 0x8300
void sub_8300(char *a0) // return-dupe
{
  int v1; // eax
  
  v1 = fputs_unlocked(a0,stdout);
  if (0 <= v1)
    return;
  dat_2a8c0 = *__errno_location();
}

// Function: sub_8330 @ 0x8330
void sub_8330(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,char *a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13) // return-dupe
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
  int v2; // eax
  unsigned long v20; // stack - 0x18
  unsigned int v3; // stack - 0xd8
  char v4 [8];
  unsigned int v5; // stack - 0xd4
  char *v6; // stack - 0xd0
  char *v7; // stack - 0xc8
  unsigned long v8; // stack - 0xb0
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
  v6 = &Stack0000000000000008;
  v3 = 8;
  v7 = v4;
  v5 = 0x30;
  v8 = a9;
  v9 = a10;
  v10 = a11;
  v11 = a12;
  v12 = a13;
  v2 = __vfprintf_chk(stdout,1,a8,&v3);
  if (0 <= v2)
    return;
  dat_2a8c0 = *__errno_location();
}

// Function: sub_8410 @ 0x8410
void sub_8410(int a0) // return-dupe
{
  if (0 <= putchar_unlocked(a0))
    return;
  dat_2a8c0 = *__errno_location();
}

// Function: sub_8430 @ 0x8430
void sub_8430(void) // return-dupe
{
  int v1; // eax
  
  v1 = fflush_unlocked(stdout);
  if (!v1)
    return;
  dat_2a8c0 = *__errno_location();
}

// Function: sub_8460 @ 0x8460
void sub_8460(void *a0,unsigned long a1) // return-dupe
{
  unsigned long v1; // rax
  
  v1 = fwrite_unlocked(a0,1,a1,stdout);
  if (a1 == v1)
    return;
  dat_2a8c0 = *__errno_location();
}

// Function: sub_8490 @ 0x8490
int * sub_8490(unsigned int a0)
{
  unsigned long v1;
  int *v2; // rax
  int *v3;
  bool v4; // zf
  
  if (((a0 & 0xf000) == 0x1000) && (dat_2a7a0)) {
    v2 = (int *)splice(0,0,1,0,0x18000,1);
    v3 = v2;
    v4 = v2 != NULL;
    if (0 <= (long)v2) { // branch-flip
      v2 = NULL;
      if (v4) {
        do {
          v2 = (int *)splice(0,0,1,0,0x18000,1);
          v3 = v2;
        } while (0 < (long)v2);
      }
    }
    else {
      v2 = __errno_location();
      if (*v2 == 0x16) goto label_84ae;
    }
    v1 = (unsigned long)v2 >> 8;
    v2 = (unsigned long)CONCAT71((undefined7)v1,v3 == NULL);
    return v2;
  }
label_84ae:
  do {
    v2 = (long)sub_1baf0(0,dat_2a860,dat_2a858);
    if (!v2) {
      v2 = (int *)0x1;
      return v2;
    }
  } while (v2 != (int *)0xffffffffffffffff);
  v2 = NULL;
  return v2;
}

// Function: sub_8550 @ 0x8550
bool sub_8550(unsigned int a0)
{
  return (a0 & 0xf000) == 0x8000;
}

// Function: sub_8560 @ 0x8560
unsigned char sub_8560(unsigned long a0,long a1)
{
  unsigned long v1;
  long v10; // rdx
  unsigned long *v11;
  long v12;
  long v13;
  long v14;
  unsigned long v15;
  unsigned int v16;
  unsigned int v2;
  unsigned char v3; // al
  char v4; // al
  unsigned long v5; // rax
  long v6;
  long v7; // rax
  void *v8;
  unsigned long v9; // rax
  
  v8 = dat_2a860;
  v15 = dat_2a838;
  v12 = dat_2a838 + 8;
  v13 = dat_2a858;
  v11 = dat_2a840;
  if ((long)dat_2a860 + (dat_2a858 - (long)dat_2a840) < v12) {
    v1 = a0 + 1;
    v5 = ~a0;
    v13 = ((dat_2a838 + a0) - dat_2a858) + v12;
    if (1 <= v13) { // branch-flip
      v16 = *(unsigned int *)(a1 + 0x18);
      v14 = -1;
      v6 = a1;
      if ((((sub_8550(v16)) && (v6 = *(long *)(v6 + 0x30) - dat_2a830, 0 <= v6)) && (v7 = v12 + a0 + v6, !SCARRY8(v12 + a0,v6))) && (v14 = v10 + v13, v10 + v13 < v7))
        v14 = v7;
      v8 = (void *)sub_1d2b0(0,0x2a858,v13,v14,1);
      v15 = dat_2a838;
      v11 = (unsigned long *)((long)v8 + v1);
      if ((unsigned long)v11 % dat_2a838) { // branch-flip
        v11 = (unsigned long *)(((dat_2a838 + 1 + a0) - (unsigned long)v11 % dat_2a838) + (long)v8);
        memmove((void *)(v5 + (long)v11),(void *)((long)dat_2a840 + v5),v1);
      }
      else {
        memmove(v8,(void *)(v5 + (long)dat_2a840),v1);
      }
      free(dat_2a860);
      v13 = dat_2a858;
      dat_2a860 = v8;
    }
    else {
      v11 = (unsigned long *)((long)dat_2a860 + v1);
      if ((unsigned long)v11 % dat_2a838) { // branch-flip
        v11 = (unsigned long *)(((dat_2a838 + 1 + a0) - (unsigned long)v11 % dat_2a838) + (long)dat_2a860);
        memmove((void *)(v5 + (long)v11),(void *)((long)dat_2a840 + v5),v1);
        v13 = dat_2a858;
      }
      else {
        memmove(dat_2a860,(void *)((long)dat_2a840 + v5),v1);
        v13 = dat_2a858;
      }
    }
  }
  dat_2a848 = (long)v11 - a0;
  v12 = (long)v8 + ((v13 + -8) - (long)v11);
  do {
    v16 = dat_2a850;
label_86eb:
    v13 = sub_1baf0(v16,v11,v12 - v12 % (long)v15);
    if (v13 == -1) {
      v3 = 0;
label_8704:
      *v11 = 0;
      dat_2a840 = v11;
      return v3;
    }
    dat_2a830 += v13;
    v3 = dat_2a821 ^ 1 | v13 == 0;
    if (v3) {
      v11 = (unsigned long *)((long)v11 + v13);
      goto label_8704;
    }
    v4 = sub_76f0(v11,v13);
    if (!v4) {
      v11 = (unsigned long *)((long)v11 + v13);
      v3 = 1;
      goto label_8704;
    }
    v9 = sub_7860(dat_2a818,v13);
    v16 = dat_2a850;
    v13 = dat_2a830;
    dat_2a818 = v9;
  } while (dat_2a880);
  v6 = lseek(dat_2a850,dat_2a830,3);
  if ((0 <= v6) || (((*__errno_location() == 6 && (v2 = *(unsigned int *)(a1 + 0x18), sub_8550(v2))) && ((v13 < *(long *)(a1 + 0x30) && (v6 = lseek(v16,0,2), 0 <= v6)))))) {
    dat_2a818 = sub_7860(v9,v6 - v13);
    v16 = dat_2a850;
    dat_2a830 = v6;
  }
  else {
    dat_2a880 = '\x01';
  }
  goto label_86eb;
}

// Function: sub_8890 @ 0x8890
void sub_8890(char a0)
{
  if (!a0)
    return;
  sub_1d990(dat_2a020); // tail-call
}

// Function: sub_88b0 @ 0x88b0
unsigned long sub_88b0(unsigned long a0,unsigned long a1,char *a2)
{
  char v1;
  long v2;
  unsigned long v3;
  
  v3 = a1;
  if (a1 > a0) {
    v2 = 0;
    if (*(char *)(a1 - 1) != dat_2a684) // branch-flip
      v1 = *(char *)(a1 - 1);
    else {
      v3 = a1 - 1;
      if (a0 >= v3) goto label_88e8;
      v2 = 1;
      v1 = *(char *)(a1 - 2);
    }
    if (v1 == '\r')
      v3 = a1 - (v2 + 1);
  }
label_88e8:
  if (1 <= (long)(v3 - a0)) {
    sub_82c0(a2);
    sub_8460(a0,v3 - a0);
    sub_8890(*a2);
    a0 = v3;
  }
  return a0;
}

// Function: sub_8950 @ 0x8950
void sub_8950(char *a0)
{
  if (!dat_2a900)
    return;
  sub_8890(*a0); // tail-call
}

// Function: sub_8970 @ 0x8970
void sub_8970(unsigned long a0,unsigned long a1)
{
  sub_82e0(a1);
  sub_8330("%*ld",dat_2a8f8,a0);
  sub_8950(a1); // tail-call
}

// Function: sub_89b0 @ 0x89b0
void sub_89b0(char a0)
{
  sub_82e0(dat_2a040);
  sub_8410((int)a0);
  sub_8950(dat_2a040); // tail-call
}

// Function: sub_89e0 @ 0x89e0
long sub_89e0(long a0,long *a1)
{
  long v1; // rax
  long *v2;
  long v3; // rdx
  
  v2 = dat_2a8f0;
  if (2 <= dat_2a8e0) {
    v1 = 0x18;
    v3 = 1;
    while (*(long *)((long)dat_2a8f0 + v1) <= a0) {
      v3 += 1;
      v1 += 0x18;
      if (v3 == dat_2a8e0) {
        v2 = &dat_2a8f0[v3 * 3 + -3];
        *a1 = (a0 - *v2) + v2[2];
        return v2[1];
      }
    }
    v2 = (long *)((long)dat_2a8f0 + v1 + -0x18);
  }
  *a1 = (a0 - *v2) + v2[2];
  return v2[1];
}

// Function: sub_8a50 @ 0x8a50
void sub_8a50(void)
{
  if (dat_2a890)
    return;
  dat_2a890 = dcgettext(NULL,"(standard input)",5);
}

// Function: sub_8a90 @ 0x8a90
void sub_8a90(void)
{
  sub_82e0(dat_2a058);
  sub_8300(sub_8a50());
  sub_8950(dat_2a058); // tail-call
}

// Function: sub_8ac0 @ 0x8ac0
char sub_8ac0(long a0,long a1,unsigned long a2,char a3) // early-return
{
  char v1;
  bool v2;
  bool v3;
  char v4;
  unsigned long v5;
  
  if (dat_2a810 != 1) {
    v1 = *(char *)(a0 + a1);
    v4 = sub_7630(a0,a1);
    *(char *)(a0 + a1) = v1;
    if (v4) {
      dat_2a888 = 1;
      return '\0';
    }
  }
  if (dat_2a808) {
    sub_8a90();
    if (dat_2a804) // branch-flip
      sub_89b0((int)a3);
    else {
      sub_8410(0);
    }
  }
  if (dat_2a801) {
    v5 = dat_2a818;
    if (dat_2a7c0 < a2) {
      sub_8250(a0);
      v5 = sub_7860(dat_2a818,1);
      dat_2a7c0 = a2;
      dat_2a818 = v5;
    }
    sub_8970(v5,dat_2a050);
    sub_89b0((int)a3);
  }
  if (dat_2a800) {
    v5 = sub_7860(dat_2a7c8,a0 - dat_2a848);
    sub_8970(v5,dat_2a048);
    sub_89b0((int)a3);
  }
  if (!dat_2a8fc)
    return '\x01';
  v2 = dat_2a801 || dat_2a800;
  v3 = dat_2a808 != 0;
  v4 = dat_2a8fc;
  if ((v2 || v3) && a1) {
    sub_8410(9);
    v4 = (v2 || v3) && a1;
  }
  return v4;
}

// Function: sub_8c60 @ 0x8c60
unsigned long sub_8c60(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  unsigned long v1;
  char v2; // al
  long v3; // rax
  long v4; // stack - 0x48
  unsigned long v5;
  unsigned long v6;
  unsigned long v7;
  
  if (a1 <= a0) { // branch-flip
    if (!dat_2a8fd)
      return a0;
  }
  else {
    v5 = 0;
    v7 = a0;
    do {
      while( true ) {
        v3 = (*dat_2a878)(dat_2a870,a0,a1 - a0,&v4,v7);
        if ((v3 < 0) || (v1 = a0 + v3, a1 == v1)) goto label_8d78;
        if (!v4) break;
        if (dat_2a8fd) { // branch-flip
          v2 = sub_8ac0(v1,v4,a1,(-(dat_2a802 == '\0') & 0xdU) + 0x2d);
          if (!v2)
            return 0;
        }
        else {
          sub_82c0(a2);
          v6 = v5;
          if (!v5)
            v6 = v7;
          v5 = 0;
          sub_8460(v6,v1 - v6);
        }
        sub_82e0(a3);
        sub_8460(v1,v4);
        sub_8950(a3);
        if (dat_2a8fd)
          sub_8410((int)dat_2a684);
        v7 = v1 + v4;
        if (a1 <= v7) goto label_8d78;
      }
      v4 = 1;
      if (!v5)
        v5 = v7;
      v7 = v1 + 1;
    } while (v7 < a1);
label_8d78:
    if (!dat_2a8fd) {
      if (!v5)
        return v7;
      return v5;
    }
  }
  return a1;
}

// Function: sub_8e20 @ 0x8e20
void sub_8e20(unsigned long a0,unsigned long a1,char a2)
{
  int v1;
  unsigned char v2;
  char *v3;
  unsigned char v4;
  char *v5;
  bool v6; // zf
  
  v4 = dat_2a8fd;
  if ((!dat_2a8fd) && (v4 = dat_2a8fd, !sub_8ac0(a0,(a1 - a0) + -1,a1,(int)a2)))
    return;
  v6 = a2 == ':';
  v2 = (v6 ^ dat_2a802) & dat_2a8fd;
  if (dat_2a900) { // branch-flip
    v5 = dat_2a038;
    if (v6 == (bool)((unsigned char)((unsigned int)dat_2a900 >> 0x1f) & dat_2a802))
      v5 = dat_2a030;
    v3 = dat_2a068;
    if (a2 != ':')
      v3 = dat_2a060;
    if (v2) goto label_8f00;
    v2 = dat_2a8fd;
    if (*v5) { // branch-flip
      if (v6 == (bool)dat_2a802) goto label_8f20;
      if (*v3) goto label_8f00;
    }
    else {
      if (!*v3) goto label_8e6c;
      if (v6 != (bool)dat_2a802) goto label_8f00;
label_8f20:
      if (v2) goto label_8e88;
      if (!*v5) {
        v4 = 0;
        goto label_8e6c;
      }
    }
    a0 = sub_88b0(a0,a1,v5);
    v4 = dat_2a8fd;
  }
  else if (v2) {
    v3 = NULL;
    v5 = NULL;
label_8f00:
    a0 = sub_8c60(a0,a1,v5,v3);
    if (!a0)
      return;
    v2 = dat_2a8fd;
    goto label_8f20;
  }
label_8e6c:
  if ((a0 < a1) && (v4 != 1))
    sub_8460(a0,a1 - a0);
label_8e88:
  if (dat_2a7d8)
    sub_8430();
  v1 = dat_2a8c0;
  if (!dat_2a8c0) {
    dat_2a7b8 = a1;
    return;
  }
  error(2,v1,dcgettext(NULL,"write error",5));
}

// Function: sub_8ff0 @ 0x8ff0
void sub_8ff0(void *a0) // return-dupe x2
{
  void *v1;
  void *v2; // rax
  
  if (!dat_2a7b8)
    dat_2a7b8 = dat_2a848;
  if (dat_2a7a8 <= 0)
    return;
  do {
    v1 = dat_2a7b8;
    if (a0 <= dat_2a7b8)
      return;
    v2 = rawmemchr(dat_2a7b8,(int)dat_2a684);
    sub_8e20(v1,(long)v2 + 1,0x2d);
    dat_2a7a8 -= 1;
  } while (1 <= dat_2a7a8);
}

// Function: sub_9070 @ 0x9070
void sub_9070(void *a0,void *a1)
{
  char *v1;
  unsigned long v2; // rax
  void *v3;
  void *v4;
  void *v5;
  int v6; // r12d
  long v7;
  long v8; // r14
  char v9;
  
  v9 = dat_2a684;
  v6 = (int)dat_2a684;
  v4 = a0;
  if ((dat_2a803) || ((1 <= (long)dat_2a7a8 && (sub_8ff0(a0), dat_2a803)))) {
    if (!dat_2a802) {
      dat_2a688 = 1; // return-dupe
      dat_2a7a8 = 0;
      dat_2a7b0 -= 1;
      dat_2a828 = (long)a1 + (dat_2a830 - dat_2a840);
      return;
    }
    v9 = dat_2a802;
  }
  else {
    v3 = dat_2a7b8;
    if (!dat_2a7b8)
      v3 = dat_2a848;
    v2 = 0;
    if (1 <= (long)dat_2a7f8) { // branch-flip
      do {
        while (v5 = v4, v4 > v3) {
          do {
            v4 = (void *)((long)v5 - 1);
            v1 = (char *)((long)v5 - 2);
            v5 = v4;
          } while (*v1 != v9);
          v2 += 1;
          if (v2 == dat_2a7f8) goto label_9219;
        }
        v2 += 1;
      } while (v2 != dat_2a7f8);
label_9219:
      if (((dat_2a688) && (dat_2a7b8 != v4)) && (dat_2a070)) {
label_9107:
        sub_82e0(dat_2a040);
        sub_8300(dat_2a070);
        sub_8950(dat_2a040);
        sub_8410(10);
      }
      if (v4 < a0) {
        do {
          v3 = (void *)((long)rawmemchr(v4,(int)v9) + 1);
          sub_8e20(v4,v3,0x2d);
          v4 = v3;
        } while (v3 < a0);
      }
    }
    else if ((((0 <= (long)(dat_2a7f8 & dat_2a7f0)) && (dat_2a7b8 != a0)) && (dat_2a688)) && (dat_2a070)) goto label_9107;
    v9 = dat_2a803;
    if (!dat_2a802) {
      if (dat_2a803) {
        dat_2a688 = 1;
        dat_2a7a8 = 0;
        dat_2a7b0 -= 1;
        dat_2a828 = (long)a1 + (dat_2a830 - dat_2a840);
        return;
      }
      sub_8e20(a0,a1,0x3a);
      dat_2a7b0 -= 1;
      v4 = a1;
      v9 = dat_2a803;
      goto label_91bc;
    }
  }
  v8 = 0;
  v7 = dat_2a7b0;
  if (v4 < a1) {
    for (; v8 < v7; v8 = v8 + 1) {
      v3 = (void *)((long)rawmemchr(v4,v6) + 1);
      if (!v9) {
        sub_8e20(v4,v3,0x3a);
        v7 = dat_2a7b0;
        v9 = dat_2a803;
      }
      v4 = v3;
      if (a1 <= v3) {
        dat_2a7b0 = v7 - (v8 + 1);
        goto label_91bc;
      }
    }
    dat_2a7b0 = v7 - v8;
  }
label_91bc:
  dat_2a7a8 = 0;
  if (!v9) {
    dat_2a7a8 = dat_2a7f0;
    if ((long)dat_2a7f0 <= -1)
      dat_2a7a8 = 0;
  }
  dat_2a688 = 1;
  dat_2a828 = (long)v4 + (dat_2a830 - dat_2a840);
}

// Function: sub_9350 @ 0x9350
long sub_9350(unsigned long a0,unsigned long a1) // early-return
{
  unsigned long v1;
  long v2;
  long v3; // rax
  long v4; // stack - 0x38
  unsigned long v5;
  unsigned long v6;
  int v7;
  
  v2 = dat_2a7b0;
  if (a1 <= a0)
    return 0;
  do {
    v3 = (*dat_2a878)(dat_2a870,a0,a1 - a0,&v4,0);
    if (0 <= v3) { // branch-flip
      v1 = a0 + v3;
      v5 = v4 + v1;
      if (!dat_2a802) {
        if (a1 == v1) {
          return v2 - dat_2a7b0; // return-dupe
        }
        v6 = v5;
        a0 = v1;
        goto label_93a9;
      }
      if (a0 < v1) {
        v6 = v1;
        goto label_93a9;
      }
    }
    else {
      if (!dat_2a802) {
        return v2 - dat_2a7b0;
      }
      v4 = 0;
      if (a1 <= a0) break;
      v6 = a1;
      v5 = a1;
label_93a9:
      sub_9070(a0,v6);
      if ((!dat_2a7b0) || (dat_2a7a2)) {
        if (dat_2a7a1) {
          v7 = 0;
          if (dat_2a889)
            v7 = dat_2a080;
          exit(v7); // no-return
        }
        return v2 - dat_2a7b0;
      }
    }
    a0 = v5;
  } while (v5 < a1);
  return v2 - dat_2a7b0;
}

// Function: sub_94a0 @ 0x94a0
void sub_94a0(int a0)
{
  if (dat_2a904) {
    dat_2a889 = 1;
    return;
  }
  error(0,a0,"%s",sub_8a50());
  dat_2a889 = 1;
}

// Function: sub_94e0 @ 0x94e0
unsigned long sub_94e0(long a0,unsigned long a1,long a2)
{
  unsigned int v1;
  long v2; // rax
  long v3; // rax
  long v4; // rdx
  
  if (dat_2a881)
    return 0;
  v1 = *(unsigned int *)(a2 + 0x18);
  if ((((sub_8550(v1)) && (a0 < *(long *)(v4 + 0x30))) && (((int)a1 || (a0 = lseek(0,0,1), 0 <= a0)))) && (v2 = lseek(a1 & 0xffffffff,a0,4), 0 <= v2)) {
    v3 = lseek(a1 & 0xffffffff,a0,0);
    if (v3 < 0)
      v3 = sub_94a0(*__errno_location());
    return CONCAT71((undefined7)((unsigned long)v3 >> 8),v2 < *(long *)(a2 + 0x30));
  }
  return 0;
}

// Function: sub_95a0 @ 0x95a0
void sub_95a0(int a0,long a1,char a2)
{
  unsigned int v1;
  long v2; // rax
  
  if (a0)
    return;
  if (dat_2a7b0) { // branch-flip
    if ((!a2) && (((dat_2a881 || (((long)lseek(0,0,2) <= -1 && (*__errno_location() != 0x16)))) && (v1 = *(unsigned int *)(a1 + 0x18), !sub_8490(v1))))) {
      sub_94a0(*__errno_location()); // return-dupe, tail-call
      return;
    }
  }
  else if (((dat_2a830 != dat_2a828) && (!dat_2a881)) && (v2 = lseek(0,dat_2a828,0), v2 <= -1)) {
    sub_94a0(*__errno_location());
    return;
  }
}

// Function: sub_9630 @ 0x9630
unsigned long sub_9630(int a0) // return-dupe
{
  int v1;
  
  dat_2a840 = dat_2a860 + 1;
  if (dat_2a840 % dat_2a838)
    dat_2a840 = dat_2a860 + ((dat_2a838 + 1) - dat_2a840 % dat_2a838);
  dat_2a848 = dat_2a840;
  *(char *)(dat_2a840 - 1) = dat_2a684;
  dat_2a850 = a0;
  if (a0) {
    dat_2a830 = 0;
    dat_2a881 = 0;
    dat_2a880 = 0;
    return 1;
  }
  dat_2a830 = lseek(0,0,1);
  dat_2a880 = (unsigned char)((unsigned long)dat_2a830 >> 0x3f);
  if (0 <= dat_2a830) {
    dat_2a881 = dat_2a880;
    return 1;
  }
  dat_2a881 = dat_2a880;
  v1 = *__errno_location();
  if (v1 == 0x1d) {
    dat_2a830 = 0;
    return 1;
  }
  sub_94a0(v1);
  return 0;
}

// Function: sub_9710 @ 0x9710
long sub_9710(unsigned long a0,long a1,char *a2) // return-dupe
{
  unsigned int v1;
  int v10;
  long v11; // rbp
  char *v12;
  char *v13;
  char *v14;
  char *v15;
  long v16; // stack - 0x70
  long v17; // stack - 0x68
  char v18; // stack - 0x59
  long v19; // stack - 0x58
  char v2;
  char v3;
  char v4;
  char v5;
  unsigned int v6; // eax
  long v7;
  unsigned long v8; // rax
  unsigned long v9; // rdx
  
  v4 = dat_2a803;
  v3 = dat_2a7a2;
  v2 = dat_2a684;
  if (!sub_9630(a0))
    return 0;
  dat_2a7c8 = 0;
  dat_2a7b8 = NULL;
  dat_2a7b0 = dat_2a7e0;
  dat_2a821 = v2 == '\0' & dat_2a820;
  dat_2a818 = 0;
  dat_2a828 = 0;
  dat_2a7a8 = 0;
  dat_2a888 = '\0';
  v6 = sub_8560(0,a1);
  v9 = (unsigned long)v6;
  if (!(char)v6) {
    sub_94a0(*__errno_location());
    return 0;
  }
  dat_2a8f8 = 0;
  if (dat_2a8fc) {
    v7 = 0x7fffffffffffffff;
    v1 = *(unsigned int *)(a1 + 0x18);
    if ((sub_8550(v1)) && (v7 = *(long *)(a1 + 0x30), dat_2a801))
      v7 += (unsigned long)(v7 != 0x7fffffffffffffff);
    dat_2a8f8 = 0;
    do {
      dat_2a8f8 += 1;
      v7 /= 10;
    } while (v7);
  }
  v18 = '\0';
  v9 &= 0xffffffff;
  v7 = 0;
  v17 = -1;
  v19 = 0;
  v16 = 0;
  do {
    v12 = dat_2a848;
    v15 = dat_2a840;
    v10 = dat_2a810;
    if (v17 <= -1) {
      if (v2) {
        if (dat_2a810 != 1) {
          v11 = (long)dat_2a840 - (long)dat_2a848;
          v5 = sub_7890(dat_2a848,v11);
          if (v5) { // branch-flip
label_9b0d:
            if (v10 == 2)
              return 0;
            if (!dat_2a7e8) {
              dat_2a803 = '\x01';
              dat_2a7a2 = '\x01';
            }
            dat_2a821 = dat_2a820;
            v17 = v19;
            v12 = dat_2a848;
            v15 = dat_2a840;
            v18 = v2;
          }
          else if ((char)v9) {
            if (sub_94e0(v11,(int)a0,a1)) {
              v10 = dat_2a810;
              goto label_9b0d;
            }
            v12 = dat_2a848;
            v15 = dat_2a840;
          }
        }
      }
    }
    if (dat_2a7b8)
      dat_2a7b8 = v12;
    v13 = &v12[v7];
    dat_2a7c0 = v12;
    if (v13 == v15) {
      *a2 = 1;
      if (v16) {
        dat_2a840 = &v15[1];
        *v15 = v2;
        if (dat_2a7b0)
          v19 += sub_9350(&v12[v7 - v16]);
        if (dat_2a7a8)
          sub_8ff0(dat_2a840);
      }
      goto label_99f0;
    }
    sub_78b0(v13,v15,(int)v18);
    v5 = v13[-1];
    v13[-1] = v2;
    v7 = memrchr(&v13[-1],(int)v2,&v15[1 - (long)v13]);
    v13[-1] = v5;
    v14 = &v13[-v16];
    v12 = (char *)(v7 + 1);
    if (v13 != v12) { // branch-flip
      v16 = (long)v15 - (long)v12;
      if (v14 < v12) {
        if (dat_2a7b0) { // branch-flip
          v19 += sub_9350(v14,v12);
          if (dat_2a7a8) goto label_991a;
          if (!dat_2a7b0) goto label_99f0;
        }
        else {
          if (!dat_2a7a8) goto label_99f0;
label_991a:
          sub_8ff0(v12);
          if (!dat_2a7b0 && !dat_2a7a8) goto label_99f0;
        }
        if (dat_2a7a2) {
          v7 = 0;
          if (0 <= v17)
            v7 = v17;
          if (v7 < v19) goto label_99f0;
        }
      }
    }
    else {
      v16 = (long)v15 - (long)v14;
      v12 = v14;
    }
    if (1 <= dat_2a7f8) {
      v7 = 0;
      do {
        if (v12 <= dat_2a848) break;
        if (v12 == dat_2a7b8) goto label_99af;
        v7 += 1;
        v13 = v12;
        do {
          v12 = &v13[-1];
          v14 = &v13[-2];
          v13 = v12;
        } while (*v14 != v2);
      } while (v7 != dat_2a7f8);
    }
    if (v12 != dat_2a7b8)
      dat_2a7b8 = NULL;
label_99af:
    v7 = (long)v15 - (long)v12;
    if (dat_2a800)
      dat_2a7c8 = sub_7860(dat_2a7c8,&dat_2a840[-v7 - (long)dat_2a848]);
    if (dat_2a801)
      sub_8250(v12);
    v9 = 0;
  } while (sub_8560(v7,a1));
  sub_94a0(*__errno_location());
label_99f0:
  if (dat_2a810) {
    dat_2a7a2 = v3;
    dat_2a803 = v4;
    return v19;
  }
  if (v4) {
    dat_2a7a2 = v3;
    dat_2a803 = v4;
    return v19;
  }
  if (!dat_2a888) {
    if (v17 <= -1) {
      dat_2a7a2 = v3;
      dat_2a803 = v4;
      return v19;
    }
    if (v19 <= v17) {
      dat_2a7a2 = v3;
      dat_2a803 = v4;
      return v19;
    }
  }
  dat_2a7a2 = v3;
  dat_2a803 = v4;
  v8 = sub_8a50();
  error(0,0,dcgettext(NULL,"%s: binary file matches",5),v8);
  return v19;
}

// Function: sub_9c70 @ 0x9c70
unsigned long sub_9c70(int a0,unsigned char a1)
{
  char v1; // al
  unsigned long v10; // r13
  unsigned long v11; // r13
  unsigned long v12; // stack - 0xd0
  long v13; // stack - 0xc0
  unsigned int v14; // stack - 0xb0
  long v2; // rax
  long v3; // rax
  int *v4; // rax
  unsigned long v5; // rax
  long v6; // stack - 0xc8
  char v7; // stack - 0xd9
  unsigned long v8; // stack - 0xd8
  unsigned int v9; // r12d
  
  v7 = 0;
  if (fstat(a0,&v6)) { // branch-flip
    v11 = 1;
    sub_94a0(*__errno_location());
label_9d70:
    if (!a0) {
      return v11 & 0xffffffff; // return-dupe
    }
  }
  else {
    if (!a0) {
      if ((int)dat_2a808 < 0)
        dat_2a808 = (unsigned int)((v14 & 0xf000) == 0x4000);
label_9cca:
      if ((((dat_2a803) || (dat_2a80c)) || (dat_2a7e0 <= 1)) || ((((v14 & 0xf000) != 0x8000 || (v13 != dat_2a928)) || (v6 != dat_2a920)))) {
        v2 = sub_9710(a0,&v6,&v7);
        if (dat_2a7e8) {
          if (dat_2a808) {
            sub_8a90();
            if (dat_2a804) // branch-flip
              sub_89b0(0x3a);
            else {
              sub_8410(0);
            }
          }
          sub_8330("%ld\n",v2);
          if (dat_2a7d8)
            sub_8430();
        }
        v11 = CONCAT71((undefined7)((unsigned long)v10 >> 8),v2 == 0);
        if (dat_2a80c) { // branch-flip
          if (dat_2a80c == (v2 == 0) + 1) {
            sub_8a90();
            sub_8410(dat_2a804 & 10);
            if (dat_2a7d8)
              sub_8430();
          }
        }
        else {
          sub_95a0(a0,&v6,v7);
        }
      }
      else {
        if (!dat_2a904) {
          v5 = sub_8a50();
          error(0,0,dcgettext(NULL,"%s: input file is also the output",5),v5);
        }
        dat_2a889 = 1;
        v11 = 1;
      }
      goto label_9d70;
    }
    if (((!sub_7500(a1)) || (v1 = sub_74d0(v14), !v1)) && ((!a1 || (v1 = sub_81e0(dat_2a890,1,(v14 & 0xf000) == 0x4000), !v1)))) {
      if (0 <= (int)dat_2a808) { // branch-flip
        if (dat_2a018 == 3) goto label_9ed9;
label_9e01:
        if ((dat_2a018 == 4) && ((v14 & 0xf000) == 0x4000)) goto label_9e36;
      }
      else {
        dat_2a808 = (unsigned int)((v14 & 0xf000) == 0x4000);
        if (dat_2a018 != 3) goto label_9e01;
label_9ed9:
        if ((v14 & 0xf000) == 0x4000) {
          v9 = ~(unsigned int)(a1 ^ 1) & dat_2a014;
          if (close(a0))
            sub_94a0(*__errno_location());
          v12 = 0;
          v8 = dat_2a890;
          v2 = sub_16c90(&v8,v9,0);
          if (!v2)
            sub_1d4b0(); // no-return
          v11 = 1;
          while (v3 = sub_171f0(v2), v3) {
            v11 = (unsigned long)((unsigned int)v11 & sub_a180(v2,v3,a1));
          }
          v4 = __errno_location();
          if (*v4)
            sub_94a0(*v4);
          if (sub_17000(v2))
            sub_94a0(*v4);
          return v11 & 0xffffffff;
        }
      }
      if (((dat_2a884 != 2) && ((dat_2a884 || (a1)))) || (v1 = sub_74d0(v14), !v1)) goto label_9cca;
    }
label_9e36:
    v11 = 1;
  }
  if (close(a0))
    sub_94a0(*__errno_location());
  return v11 & 0xffffffff;
}

// Function: sub_a0f0 @ 0xa0f0
unsigned long sub_a0f0(unsigned int a0,unsigned long a1,char a2,char a3)
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // r8
  
  v3 = (unsigned long)a0;
  sub_7500(a3);
  if (a2) { // branch-flip
    v1 = sub_19570(v3 & 0xffffffff);
    if (v1 < 0) {
      sub_94a0(*__errno_location());
      return 1;
    }
  }
  else {
    v1 = sub_19570(v3 & 0xffffffff);
    if (v1 <= -1) {
      v2 = sub_7720(*__errno_location());
      if ((char)v2)
        return v2;
      sub_94a0();
      return 1;
    }
  }
  return sub_9c70(v1,a3); // tail-call
}

// Function: sub_a180 @ 0xa180
unsigned long sub_a180(long a0,long a1,unsigned int a2) // return-dupe x3
{
  short v1;
  int v2;
  char *v3;
  long v4;
  unsigned int v5; // eax
  unsigned long v6; // rax
  unsigned int v7; // eax
  char v8 [24];
  int v9; // stack - 0xc0
  
  v1 = *(short *)(a1 + 0x68);
  a2 = *(long *)(a1 + 0x58) == 0 & a2;
  if (v1 == 6)
    return 1;
  if (!(char)a2) {
    v7 = sub_81e0(a1 + 0x100,0,(unsigned short)(v1 - 1U) < 2 || v1 == 4);
    if ((char)v7) {
      sub_17830(a0,a1,4);
      return (unsigned long)v7;
    }
    v1 = *(short *)(a1 + 0x68);
  }
  dat_2a890 = *(long *)(a1 + 0x38);
  if ((dat_2a88a) && (*(char *)(dat_2a890 + 1)))
    dat_2a890 += 2;
  v4 = dat_2a890;
  v7 = 1;
  if (!(*(unsigned int *)(a0 + 0x48) & 2))
    v7 = *(unsigned int *)(a0 + 0x48) & a2;
  switch(v1) {
    default:
      abort(); // no-return
    case 1:
      if (dat_2a018 == 3)
        return 1;
      sub_17830(a0,a1,4);
      break;
    case 2:
      if (dat_2a904)
        return 1;
      error(0,0,dcgettext(NULL,"%s: warning: recursive directory loop",5),v4);
      return 1;
    case 3:
    case 0xb:
      v5 = sub_7500(a2);
      if ((char)v5) {
        v2 = *(int *)(a1 + 0x88);
        if (!*(int *)(a1 + 0x88)) {
          v3 = *(char **)(a1 + 0x30);
          v2 = *(int *)(a0 + 0x2c);
          if (fstatat(v2,v3,v8,v7 << 8 ^ 0x100)) {
            sub_94a0(*__errno_location());
            return (unsigned long)v5;
          }
          v2 = v9;
        }
        if (sub_74d0(v2))
          return (unsigned long)v5;
      }
      break;
    case 4:
    case 7:
    case 10:
      sub_94a0(*(unsigned int *)(a1 + 0x40));
      return 1;
    case 8:
    case 0xd:
      break;
    case 0xc:
    case 0xe:
      return 1;
    
  }
  v6 = sub_a0f0(*(unsigned int *)(a0 + 0x2c),*(unsigned long *)(a1 + 0x30),v7,a2); // tail-call
  return v6;
}

// Function: sub_a3f0 @ 0xa3f0
void sub_a3f0(char *a0)
{
  if (!strcmp(a0,"-")) {
    dat_2a890 = (char *)dat_2a7d0;
    sub_9c70(0,1); // tail-call
    return;
  }
  dat_2a890 = a0;
  sub_a0f0(0xffffff9c,a0,1,1); // tail-call
}

// Function: sub_a440 @ 0xa440
void sub_a440(int a0)
{
  unsigned long v1; // rax
  
  v1 = sub_179e0();
  if (a0) { // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Usage: %s [OPTION]... PATTERNS [FILE]...\n",5),v1);
    v1 = sub_179e0();
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v1);
  }
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... PATTERNS [FILE]...\n",5),v1);
    __printf_chk(1,dcgettext(NULL,"Search for PATTERNS in each FILE.\n",5));
    v1 = sub_179e0();
    __printf_chk(1,dcgettext(NULL,"Example: %s -i \'hello world\' menu.h main.c\nPATTERNS can contain multiple patterns separated by newlines.\n\nPattern selection and interpretation:\n",5),v1);
    __printf_chk(1,dcgettext(NULL,"  -E, --extended-regexp     PATTERNS are extended regular expressions\n  -F, --fixed-strings       PATTERNS are strings\n  -G, --basic-regexp        PATTERNS are basic regular expressions\n  -P, --perl-regexp         PATTERNS are Perl regular expressions\n",5));
    __printf_chk(1,dcgettext(NULL,"  -e, --regexp=PATTERNS     use PATTERNS for matching\n  -f, --file=FILE           take PATTERNS from FILE\n  -i, --ignore-case         ignore case distinctions in patterns and data\n      --no-ignore-case      do not ignore case distinctions (default)\n  -w, --word-regexp         match only whole words\n  -x, --line-regexp         match only whole lines\n  -z, --null-data           a data line ends in 0 byte, not newline\n",5));
    __printf_chk(1,dcgettext(NULL,"\nMiscellaneous:\n  -s, --no-messages         suppress error messages\n  -v, --invert-match        select non-matching lines\n  -V, --version             display version information and exit\n      --help                display this help text and exit\n",5));
    __printf_chk(1,dcgettext(NULL,"\nOutput control:\n  -m, --max-count=NUM       stop after NUM selected lines\n  -b, --byte-offset         print the byte offset with output lines\n  -n, --line-number         print line number with output lines\n      --line-buffered       flush output on every line\n  -H, --with-filename       print file name with output lines\n  -h, --no-filename         suppress the file name prefix on output\n      --label=LABEL         use LABEL as the standard input file name prefix\n",5));
    __printf_chk(1,dcgettext(NULL,"  -o, --only-matching       show only nonempty parts of lines that match\n  -q, --quiet, --silent     suppress all normal output\n      --binary-files=TYPE   assume that binary files are TYPE;\n                            TYPE is \'binary\', \'text\', or \'without-match\'\n  -a, --text                equivalent to --binary-files=text\n",5));
    __printf_chk(1,dcgettext(NULL,"  -I                        equivalent to --binary-files=without-match\n  -d, --directories=ACTION  how to handle directories;\n                            ACTION is \'read\', \'recurse\', or \'skip\'\n  -D, --devices=ACTION      how to handle devices, FIFOs and sockets;\n                            ACTION is \'read\' or \'skip\'\n  -r, --recursive           like --directories=recurse\n  -R, --dereference-recursive  likewise, but follow all symlinks\n",5));
    __printf_chk(1,dcgettext(NULL,"      --include=GLOB        search only files that match GLOB (a file pattern)\n      --exclude=GLOB        skip files that match GLOB\n      --exclude-from=FILE   skip files that match any file pattern from FILE\n      --exclude-dir=GLOB    skip directories that match GLOB\n",5));
    __printf_chk(1,dcgettext(NULL,"  -L, --files-without-match  print only names of FILEs with no selected lines\n  -l, --files-with-matches  print only names of FILEs with selected lines\n  -c, --count               print only a count of selected lines per FILE\n  -T, --initial-tab         make tabs line up (if needed)\n  -Z, --null                print 0 byte after FILE name\n",5));
    __printf_chk(1,dcgettext(NULL,"\nContext control:\n  -B, --before-context=NUM  print NUM lines of leading context\n  -A, --after-context=NUM   print NUM lines of trailing context\n  -C, --context=NUM         print NUM lines of output context\n",5));
    __printf_chk(1,dcgettext(NULL,"  -NUM                      same as --context=NUM\n      --group-separator=SEP  print SEP on line between matches with context\n      --no-group-separator  do not print separator for matches with context\n      --color[=WHEN],\n      --colour[=WHEN]       use markers to highlight the matching strings;\n                            WHEN is \'always\', \'never\', or \'auto\'\n  -U, --binary              do not strip CR characters at EOL (MSDOS/Windows)\n\n",5));
    __printf_chk(1,dcgettext(NULL,"When FILE is \'-\', read standard input.  With no FILE, read \'.\' if\nrecursive, \'-\' otherwise.  With fewer than two FILEs, assume -h.\nExit status is 0 if any line is selected, 1 otherwise;\nif any error occurs and -q is not given, the exit status is 2.\n",5));
    sub_1d030();
  }
  exit(a0); // no-return
}

// Function: sub_a6b0 @ 0xa6b0
void sub_a6b0(unsigned long *a0,unsigned long *a1)
{
  unsigned char v1;
  unsigned char *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // stack - 0x48
  unsigned char *v5;
  unsigned long v6; // r14
  unsigned long v7;
  unsigned char *v8; // stack - 0x60
  
  v7 = *a1;
  v5 = (unsigned char *)*a0;
  v4 = 0;
  v2 = (unsigned char *)sub_1d210(v7 + 1,2);
  v8 = v2;
  if (v7) { // branch-flip
    do {
      while (v3 = sub_c9f0(v5,v7,&v4), v3 == 0xffffffffffffffff) {
        v4 = 0;
label_a767:
        v1 = *v5;
        if (((unsigned char)((unsigned int)v1 - 0x24) <= 0x3a) && (0x580000000000441U >> ((unsigned long)((unsigned int)v1 - 0x24) & 0x3f) & 1)) {
          *v8 = 0x5c;
          v1 = *v5;
          v8 = &v8[1];
        }
        v7 -= 1;
        *v8 = v1;
        v8 = &v8[1];
        v5 = &v5[1];
        if (!v7) goto label_a7ab;
      }
      if (v3 == 1) goto label_a767;
      v6 = 0;
      if (v3 != 0xfffffffffffffffe) {
        v6 = v7 - v3;
        v7 = v3;
      }
      v8 = mempcpy(v8,v5,v7);
      v5 = &v5[v7];
      v7 = v6;
    } while (v6);
label_a7ab:
    v7 = (long)v8 - (long)v2;
  }
  else {
    v7 = 0;
  }
  *v8 = 10;
  free((void *)*a0);
  *a0 = v2;
  *a1 = v7;
}

// Function: sub_a820 @ 0xa820
void sub_a820(char *a0,long a1)
{
  char v1;
  unsigned long v2; // rax
  void *v3; // rax
  char *v4; // rax
  unsigned long v5; // rax
  unsigned long *v6; // rax
  unsigned long v7; // stack - 0x48
  unsigned long v8; // r13
  char *v9;
  
  v4 = NULL;
  v7 = 0;
  v2 = sub_ca70(1);
  v9 = a0;
  do {
    v3 = rawmemchr(v9,10);
    v8 = (long)v3 - (long)v9;
    if (dat_2a685) {
      if ((v9 <= a0) || (dat_2a684 != '\n')) {
        if ((long)v7 <= (long)(v8 + 1)) {
          free(v4);
          v7 = v8;
          v4 = (char *)sub_1d2b0(0,&v7,2,0xffffffffffffffff,1);
          *v4 = dat_2a684;
        }
        v1 = dat_2a684;
        memcpy(&v4[1],v9,v8);
        v4[v8 + 1] = v1;
        v9 = v4;
      }
      else {
        v9 = &v9[-1];
      }
      v8 += 2;
    }
    sub_bab0(v2,v9,v8);
    v9 = (char *)((long)v3 + 1U);
  } while ((char *)((long)v3 + 1U) <= &a0[a1]);
  free(v4);
  v5 = sub_bf90(v2);
  sub_bfa0(v2);
  v6 = (unsigned long *)sub_1d0f0(0x28);
  *v6 = v2;
  v6[2] = a0;
  v6[1] = v5;
  v6[3] = a1;
  v6[4] = 0;
}

// Function: sub_a9c0 @ 0xa9c0
long sub_a9c0(unsigned long *a0,void *a1,long a2,long *a3,void *a4) // return-dupe
{
  unsigned long v1;
  unsigned int v10; // edx
  void *v11;
  char v12 [16];
  char v13 [16];
  long v14; // stack - 0x80
  void *v15; // stack - 0x88
  void *v16;
  void *v17;
  long v18;
  long v19; // stack - 0x68
  unsigned long v2;
  long v20; // stack - 0x48
  unsigned long v3;
  char v4;
  unsigned int v5; // eax
  long v6;
  long v7; // rax
  unsigned char v8; // cl
  unsigned char v9; // cl
  
  v4 = dat_2a684;
  v16 = (void *)((long)a1 + a2);
  v1 = *a0;
  v5 = (unsigned int)dat_2a685;
  v8 = dat_2a180 & ((dat_2a182 | dat_2a685) ^ 1);
  v9 = v8 | dat_2a686;
  v10 = v5 ^ 1;
  v11 = a1;
  if (a4)
    v11 = a4;
  if (v16 < v11)
    return -1;
  v15 = v11;
  while( true ) {
    v6 = sub_c870(v1,(long)v11 - (unsigned long)v5,(long)v16 + ((unsigned long)v5 - (long)v11),v12,(v9 | a4 != NULL) & v10);
    if (v6 <= -1)
      return -1;
    v6 = (long)v11 + v6;
    v14 = 0;
    v18 = v19 - (int)((unsigned int)dat_2a685 * 2);
    if ((!v8) || (!sub_caf0(&v15,&v14,v6,v16))) break;
    v11 = v15;
    if (v16 < v15)
      return -1;
label_ac3c:
    v5 = (unsigned int)dat_2a685;
  }
  v11 = v15;
  if ((dat_2a686 == 1) || (!a4)) {
    if (!dat_2a685) {
      if (!dat_2a686) {
        v11 = (void *)(v6 + v18);
label_acfd:
        if (v11 < v16)
          v16 = (void *)((long)rawmemchr(v11,(int)v4) + 1);
label_ad17:
        v18 = memrchr(a1,(int)v4,v6 - (long)a1);
        if (v18) { // branch-flip
          v6 = (v18 + 1) - (long)a1;
          a1 = (void *)(v18 + 1);
        }
        else {
          v6 = 0;
        }
        v18 = (long)v16 - (long)a1;
        goto label_acc8;
      }
      if (v14) { // branch-flip
        if (v14 <= 0) goto label_ab44;
        v7 = sub_cc90(v6 - v14,v16);
      }
      else {
        v7 = memrchr(v15,(int)v4,v6 - (long)v15);
        if (v7) {
          v11 = (void *)(v7 + 1);
          v15 = v11;
        }
label_ab44:
        v7 = sub_cca0(v11,v6,v16);
      }
      if (!v7) {
        while( true ) {
          v11 = (void *)(v6 + v18);
          if (!sub_cc90(v11,v16)) {
            if (!a4) goto label_acfd;
            goto label_acc0;
          }
          if ((!a4) && (!dat_2a180)) break;
          if ((!v18) || (sub_c870(v1,v6,v18 + -1,v13,1))) goto label_ac20;
          v18 = v20;
        }
        if (!a0[4]) {
          sub_a6b0(&a0[2],&a0[3]);
          v2 = a0[3];
          v3 = a0[2];
          a0[4] = sub_6690(v3,v2,0x10a46,0);
        }
        v17 = v16;
        if (v11 < v16)
          v17 = (void *)((long)rawmemchr(v11,(int)v4) + 1);
        v2 = a0[4];
        if (0 <= (long)sub_6c90(v2,v6,(long)v17 - v6,a3,0)) {
          v16 = v17;
          goto label_ad17;
        }
        v6 = (long)v17 - 1;
      }
label_ac20:
      v15 = (void *)(v6 + sub_cc80(v6,v16));
      v11 = (void *)((long)v15 + 1);
      if (v16 < (void *)((long)v15 + 1))
        return -1;
      goto label_ac3c;
    }
    v18 += (unsigned long)(a4 == NULL);
  }
label_acc0:
  v6 -= (long)a1;
label_acc8:
  *a3 = v18;
  return v6;
}

// Function: sub_adc0 @ 0xadc0
void sub_adc0(char a0)
{
  sub_c9d0((int)a0); // tail-call
}

// Function: sub_add0 @ 0xadd0
unsigned int sub_add0(long a0,unsigned int a1)
{
  if (a0)
    return (unsigned int)*(unsigned char *)(a0 + (unsigned long)(unsigned char)sub_adc0((int)(char)a1));
  return a1;
}

// Function: sub_ae00 @ 0xae00
void sub_ae00(unsigned long *a0,long *a1)
{
  long v1;
  
  if (a0) {
    sub_ae00(*a0,a1);
    sub_ae00(a0[1],a1);
    v1 = a0[2];
    *(long *)(*a1 + 0x18) = v1;
    *a1 = v1;
    return;
  }
}

// Function: sub_ae50 @ 0xae50
void sub_ae50(unsigned long *a0,long a1,unsigned long a2,char a3)
{
  unsigned long *v1;
  unsigned char *v2;
  unsigned long *v3;
  long *v4;
  long *v5;
  unsigned long *v6;
  
  if (!a0)
    return;
  sub_ae50(*a0,a1,a2,a3);
  sub_ae50(a0[1],a1,a2,a3);
  do {
    if (!a1) {
      *(unsigned long *)(a0[2] + 0x20) = a2;
      return;
    }
    v6 = *(unsigned long **)(a1 + 8);
    if (v6) {
      do {
        v2 = (unsigned char *)&v6[3];
        if (*(unsigned char *)&a0[3] == *v2) {
          v4 = (long *)a0[2];
          v5 = (long *)v6[2];
          v4[4] = (long)v5;
          if (a3)
            return;
          if (!*v5)
            return;
          if (*v4)
            return;
          *v4 = -1;
          return;
        }
        v3 = (unsigned long *)*v6;
        v1 = &v6[1];
        v6 = v3;
        if (*v2 <= *(unsigned char *)&a0[3])
          v6 = (unsigned long *)*v1;
      } while (v6);
    }
    a1 = *(long *)(a1 + 0x20);
  } while( true );
}

// Function: sub_af10 @ 0xaf10
void sub_af10(unsigned long *a0,long a1,long a2) // return-dupe
{
  if (!a0)
    return;
  sub_af10(*a0,a1,a2);
  sub_af10(a0[1],a1,a2);
  if ((long)(unsigned long)*(unsigned char *)(a2 + (unsigned long)*(unsigned char *)&a0[3]) <= a1)
    return;
  *(unsigned char *)(a2 + (unsigned long)*(unsigned char *)&a0[3]) = (unsigned char)a1;
}

// Function: sub_af60 @ 0xaf60
unsigned long sub_af60(unsigned long *a0,unsigned long *a1) // return-dupe
{
  unsigned long *v1;
  unsigned char *v2;
  unsigned long *v3;
  unsigned long v4; // rax
  
  if (!a1)
    return 1;
  v4 = sub_af60(a0,*a1);
  if (((char)v4) && (v4 = sub_af60(a0,a1[1]), (char)v4)) {
    if (!a0)
      return 0;
    do {
      v2 = (unsigned char *)&a0[3];
      if (*(unsigned char *)&a1[3] == *v2)
        return v4;
      v3 = (unsigned long *)*a0;
      v1 = &a0[1];
      a0 = v3;
      if (*v2 <= *(unsigned char *)&a1[3])
        a0 = (unsigned long *)*v1;
    } while (a0);
    return 0;
  }
  return v4;
}

// Function: sub_afe0 @ 0xafe0
void sub_afe0(unsigned long *a0,long a1)
{
  if (a0) {
    sub_afe0(*a0,a1);
    sub_afe0(a0[1],a1);
    *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&a0[3] * 8) = a0[2];
    return;
  }
}

// Function: sub_b030 @ 0xb030
unsigned long sub_b030(unsigned long *a0,unsigned long a1,long a2,long a3,unsigned long a4,char a5,char a6,long a7,long a8)
{
  char v1;
  char v2;
  unsigned char v3; // al
  unsigned long v4; // rax
  long v5;
  unsigned long v6;
  long v7;
  long v8; // stack - 0x58
  
  v2 = a6;
  v6 = *a0;
  v8 = 0;
  v5 = a3;
  do {
    v3 = sub_add0(a4,(int)*(char *)(v6 - 2));
    if (v3 != v2) { // branch-flip
      v5 = 0;
      v7 = 2;
    }
    else {
      v7 = 2;
      do {
        v7 += 1;
        if (v5 < v7) {
          v7 = v5 + 1 + v8;
          goto label_b124;
        }
        v3 = sub_add0(a4,(int)*(char *)(v6 + -v7));
        v1 = v3;
        v3 = sub_add0(a4,(int)*(char *)(a2 + -v7));
      } while (v1 == v3);
label_b167:
      v5 = v7 * 8 + -0x10;
    }
    v5 = *(long *)(*(long *)(a8 + 0x978) + v5);
    v6 += v5;
    if (a1 < v6) goto label_b197;
    v1 = *(char *)(v6 - 1);
    v3 = sub_add0(a4,(int)v1);
    if (v3 != a5) {
      if (a7) {
        v3 = sub_adc0((int)v1);
        v4 = (unsigned long)v3;
        v4 = (unsigned long)*(unsigned char *)(a7 + v4);
        v6 += v4;
      }
label_b197:
      v4 = 0;
label_b199:
      *a0 = v6;
      return v4;
    }
    v8 = v7 + -1;
  } while( true );
label_b124:
  if (a3 < v7) {
    v6 -= a3;
    v4 = 1;
    goto label_b199;
  }
  v3 = sub_add0(a4,(int)*(char *)(v6 + -v7));
  v1 = v3;
  v3 = sub_add0(a4,(int)*(char *)(a2 + -v7));
  if (v1 != v3) goto label_b167;
  v7 += 1;
  goto label_b124;
}

// Function: sub_b1c0 @ 0xb1c0
char * sub_b1c0(char *a0,long a1,long a2) // return-dupe x2
{
  char *v1;
  char v2;
  int v3;
  unsigned int v4;
  
  v1 = &a0[a1];
  v3 = *(int *)(a2 + 0x98c);
  if (0 <= v3) { // branch-flip
    while( true ) {
      if (v1 <= a0)
        return NULL;
      v2 = *a0;
      if (*(long *)(a2 + 0x170 + (unsigned long)(unsigned char)sub_adc0((int)v2) * 8)) break;
      a0 = &a0[1];
      if (!((unsigned long)a0 & 0xf)) {
        v4 = *(unsigned int *)(a2 + 0x988);
        return (char *)sub_190f0(a0,v4,v3,(long)v1 - (long)a0); // tail-call
      }
    }
  }
  else {
    if (v1 <= a0)
      return NULL;
    while (v2 = *a0, !*(long *)(a2 + 0x170 + (unsigned long)(unsigned char)sub_adc0((int)v2) * 8)) {
      a0 = &a0[1];
      if (v1 == a0)
        return NULL;
    }
  }
  return a0;
}

// Function: sub_b290 @ 0xb290
long sub_b290(long a0,char *a1,long a2,long *a3,char a4) // return-dupe x5
{
  unsigned char *v1;
  char *v10;
  unsigned long *v11;
  long v12;
  char *v13; // rbp
  long *v14;
  char *v15;
  unsigned long v16;
  long *v17;
  char v2;
  int v3;
  unsigned long v4;
  unsigned long *v5;
  unsigned long *v6;
  long v7;
  long *v8;
  unsigned char v9; // al
  
  if (a2 < *(long *)(a0 + 0x68))
    return -1;
  v14 = *(long **)(a0 + 0x60);
  v4 = *(unsigned long *)(a0 + 0x980);
  v12 = *v14;
  v13 = &a1[a2];
  v17 = v14;
  v10 = a1;
  if (!v12) {
    v3 = *(int *)(a0 + 0x988);
    v15 = a1;
label_b2e6:
    v10 = v15;
    if (0 <= v3) { // branch-flip
      v10 = (char *)sub_b1c0(v15,(long)v13 - (long)v15,a0);
      if (!v10)
        return -1;
      v2 = *v10;
      v10 = &v10[1];
      v14 = *(long **)(a0 + 0x170 + (unsigned long)(unsigned char)sub_add0(v4,(int)v2) * 8);
    }
    else {
      while( true ) {
        v2 = *v10;
        v10 = &v10[1];
        v14 = *(long **)(a0 + 0x170 + (unsigned long)(unsigned char)sub_add0(v4,(int)v2) * 8);
        if (v14) break;
        if (v13 <= v10)
          return -1;
      }
    }
    v12 = *v14;
    v17 = v14;
    if (!v12) {
      if (v13 <= v10)
        return -1;
      do {
        v15 = &v10[1];
        v9 = sub_add0(v4,(int)*v10);
        v11 = (unsigned long *)v14[1];
        while (v1 = (unsigned char *)&v11[3], *v1 != v9) {
          v5 = (unsigned long *)*v11;
          v6 = &v11[1];
          v11 = v5;
          if (*v1 <= v9)
            v11 = (unsigned long *)*v6;
          if (!v11) {
            v14 = (long *)v14[4];
            if (!v14) {
              v14 = *(long **)(a0 + 0x170 + (unsigned long)v9 * 8);
              if (v14) goto label_b375;
              if (v13 <= v15)
                return -1;
              goto label_b2e6;
            }
            v12 = *v14;
            v17 = v14;
            if (v12) goto label_b3a7;
            v11 = (unsigned long *)v14[1];
          }
        }
        v14 = (long *)v11[2];
label_b375:
        v12 = *v14;
        v17 = v14;
        v10 = v15;
        if (v12) break;
        if (v13 == v15)
          return -1;
      } while( true );
    }
  }
label_b3a7:
  while (v12 <= -1) {
    v14 = (long *)v14[4];
    v12 = *v14;
  }
  v12 = v14[5];
  v16 = (long)v10 - v12;
  if ((!a4) || (v13 <= v10)) {
label_b434:
    v7 = *v14;
    a3[2] = v12;
    *a3 = v7 >> 1;
    a3[1] = v16 - (long)a1;
    return v16 - (long)a1;
  }
  do {
    v9 = sub_add0(v4,(int)*v10);
label_b3e8:
    v11 = (unsigned long *)v17[1];
label_b3ef:
    if (!v11) goto label_b490;
    v1 = (unsigned char *)&v11[3];
    if (*v1 != v9) {
      v6 = (unsigned long *)*v11;
      v5 = &v11[1];
      v11 = v6;
      if (*v1 <= v9)
        v11 = (unsigned long *)*v5;
      goto label_b3ef;
    }
    v17 = (long *)v11[2];
    v10 = &v10[1];
    v12 = *v17;
    v8 = v17;
    if (v12) {
      while (v12 <= -1) {
        v12 = *(long *)v8[4];
        v8 = (long *)v8[4];
      }
      if ((unsigned long)((long)v10 - v8[5]) <= v16) {
        v14 = v8;
        v16 = (long)v10 - v8[5];
      }
    }
    if (v13 == v10) {
      v12 = v14[5];
      goto label_b434;
    }
  } while( true );
label_b490:
  v17 = (long *)v17[4];
  v12 = v14[5];
  if ((!v17) || (v17[5] < v12)) goto label_b434;
  goto label_b3e8;
}

// Function: sub_b530 @ 0xb530
void sub_b530(void)
{
  sub_b290(); // tail-call
}

// Function: sub_b540 @ 0xb540
long sub_b540(long a0,long a1,long a2) // return-dupe x2
{
  long v1;
  unsigned long v10;
  unsigned long v11; // stack - 0x48
  unsigned long v12; // r14
  unsigned long v13;
  char v2;
  char v3;
  char v4;
  unsigned char v5;
  unsigned long v6;
  long v7; // rax
  long v8;
  long v9;
  
  v8 = *(long *)(a0 + 0x68);
  if (!v8)
    return 0;
  v6 = *(unsigned long *)(a0 + 0x980);
  if (a2 < v8)
    return -1;
  if (v8 == 1) {
    v8 = sub_b1c0(a1,a2,a0);
    if (!v8)
      return -1;
    return v8 - a1;
  }
  v1 = a0 + 0x70;
  v13 = a1 + v8;
  v7 = *(long *)(a0 + 0x970) + v8;
  v2 = *(char *)(a0 + 0x988);
  v3 = *(char *)(a0 + 0x990);
  v11 = v13;
  if ((a2 <= v8 * 0xc) || (SEXT816(v8 * 0xc) != SEXT816(v8) * SEXT816(0xc))) {
label_b5f3:
    a2 += a1;
    v4 = *(char *)(v13 - 1);
    v12 = (unsigned long)*(unsigned char *)(a0 + 0x70 + (unsigned long)(unsigned char)sub_adc0((int)v4));
    do {
      v11 = v13;
      if ((long)(a2 - v11) < (long)v12)
        return -1;
      while( true ) {
        v13 = v11 + v12;
        v4 = *(char *)(v13 - 1);
        v11 = v13;
        v5 = *(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
        v12 = (unsigned long)v5;
        if (v5) break;
        if (sub_b030(&v11,a2,v7,v8,v6,(int)v2,(int)v3,0,a0)) {
          return v11 - a1;
        }
        if ((long)(a2 - v11) <= -1)
          return -1;
      }
    } while( true );
  }
  v12 = a2 + v8 * -0xb + a1;
label_b6df:
  do {
    if (v12 < v13) goto label_b5f3;
    v4 = *(char *)(v13 - 1);
    v9 = v13 + *(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
    v4 = *(char *)(v9 + -1);
    v5 = *(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
    v10 = v9 + (unsigned long)v5;
    v11 = v10;
    if (v5) {
      v4 = *(char *)(v10 - 1);
      v9 = v10 + *(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
      v4 = *(char *)(v9 + -1);
      v9 += (unsigned long)*(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
      v4 = *(char *)(v9 + -1);
      v5 = *(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
      v10 = v9 + (unsigned long)v5;
      v11 = v10;
      if (v5) {
        v4 = *(char *)(v10 - 1);
        v9 = v10 + *(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
        v4 = *(char *)(v9 + -1);
        v9 += (unsigned long)*(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
        v4 = *(char *)(v9 + -1);
        v5 = *(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
        v10 = v9 + (unsigned long)v5;
        v11 = v10;
        if (v5) {
          v4 = *(char *)(v10 - 1);
          v9 = v10 + *(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
          v4 = *(char *)(v9 + -1);
          v11 = v9 + (unsigned long)*(unsigned char *)(v1 + (unsigned long)(unsigned char)sub_adc0((int)v4));
          v9 = v11 - v13;
          v13 = v11;
          if (0x80 <= v9) goto label_b6df;
          v9 = sub_b1c0(v11 - 1,(a1 + a2) - (v11 - 1),a0);
          if (!v9)
            return -1;
          v13 = v9 + 1;
          v11 = v13;
          if (v12 <= v13) goto label_b5f3;
        }
      }
    }
    v13 = v11;
    if (sub_b030(&v11,v12,v7,v8,v6,(int)v2,(int)v3,v1,a0))
      return v11 - a1;
  } while( true );
}

// Function: sub_b930 @ 0xb930
void sub_b930(long a0,unsigned long a1,unsigned long a2,unsigned long *a3)
{
  unsigned long v1;
  unsigned long v2; // rax
  
  v2 = sub_b540();
  v1 = *(unsigned long *)(a0 + 0x68);
  *a3 = 0;
  a3[1] = v2;
  a3[2] = v1;
}

// Function: sub_b960 @ 0xb960
long sub_b960(unsigned long a0)
{
  long v1; // rax
  unsigned long *v2; // rax
  unsigned long v3; // rax
  unsigned long v4;
  unsigned long *v5;
  
  v1 = sub_1d0f0(0x9a0);
  _obstack_begin(v1,0,0,sub_1d0f0,dat_29fb8);
  v4 = *(unsigned long *)(v1 + 0x20);
  *(unsigned long *)(v1 + 0x58) = 0;
  if (0x40 <= v4 - *(long *)(v1 + 0x18)) { // branch-flip
    v5 = *(unsigned long **)(v1 + 0x10);
    v2 = (unsigned long *)(*(long *)(v1 + 0x18) + 0x40);
    if (v2 != v5) goto label_b9ca;
  }
  else {
    _obstack_newchunk(v1,0x40);
    v5 = *(unsigned long **)(v1 + 0x10);
    v4 = *(unsigned long *)(v1 + 0x20);
    v2 = (unsigned long *)(*(long *)(v1 + 0x18) + 0x40);
    if (v2 != v5) goto label_b9ca;
  }
  *(unsigned char *)(v1 + 0x50) = *(unsigned char *)(v1 + 0x50) | 2;
label_b9ca:
  *(unsigned long **)(v1 + 0x60) = v5;
  v3 = (long)v2 + *(unsigned long *)(v1 + 0x30) & ~*(unsigned long *)(v1 + 0x30);
  if (v3 - *(long *)(v1 + 8) <= v4 - *(long *)(v1 + 8))
    v4 = v3;
  *(unsigned long *)(v1 + 0x18) = v4;
  *(unsigned long *)(v1 + 0x10) = v4;
  *v5 = 0;
  v5[1] = 0;
  v5[2] = 0;
  v5[3] = 0;
  v5[4] = 0;
  v5[5] = 0;
  v5[6] = 0;
  *(unsigned long *)(v1 + 0x68) = 0x7fffffffffffffff;
  *(unsigned long *)(v1 + 0x980) = a0;
  *(void **)(v1 + 0x998) = sub_b530;
  *(unsigned long *)(v1 + 0x970) = 0;
  return v1;
}

// Function: sub_bab0 @ 0xbab0
void sub_bab0(long a0,unsigned char *a1,long a2) // return-dupe
{
  unsigned char v1;
  long *v10;
  long *v11;
  long v12;
  int v13 [12]; // stack - 0xd8
  long *v14 [13]; // stack - 0xa8
  unsigned char v15;
  unsigned long v16;
  int v17;
  long v18;
  unsigned long *v19;
  void *v2;
  unsigned long v20;
  unsigned long v21; // stack - 0xf8
  unsigned long v22; // stack - 0xf0
  long v3;
  long *v4;
  long v5;
  long v6;
  long *v7;
  char v8;
  unsigned long *v9; // rax
  
  v2 = *(void **)(a0 + 0x998);
  v3 = *(long *)(a0 + 0x980);
  v7 = *(long **)(a0 + 0x60);
  if (v2 == sub_b930)
    a1 = &a1[a2];
label_bb05:
  do {
    while( true ) {
      if (!a2) {
        v3 = *(long *)(a0 + 0x58);
        if (!*v7)
          *v7 = v3 * 2 + 1;
        *(long *)(a0 + 0x58) = v3 + 1;
        if (*(long *)(a0 + 0x68) <= v7[5])
          return;
        *(long *)(a0 + 0x68) = v7[5];
        return;
      }
      a2 -= 1;
      if (v2 != sub_b930) { // branch-flip
        v15 = *a1;
        a1 = &a1[1];
      }
      else {
        v15 = a1[-1];
        a1 = &a1[-1];
      }
      if (v3)
        v15 = *(unsigned char *)(v3 + (unsigned long)v15);
      v10 = (long *)v7[1];
      v13[0] = 0;
      v14[0] = &v7[1];
      v12 = 1;
      if (!v10) break;
      while (v1 = *(unsigned char *)&v10[3], v1 != v15) {
        v14[v12] = v10;
        v18 = v12 + 1;
        if (v15 <= v1) { // branch-flip
          v10 = (long *)*v10;
          v13[v12] = 0;
        }
        else {
          v10 = (long *)v10[1];
          v13[v12] = 1;
        }
        if (!v10) goto label_bb9c;
        v12 = v18;
      }
      v7 = (long *)v10[2];
    }
    v12 = 0;
label_bb9c:
    v16 = *(unsigned long *)(a0 + 0x20);
    if (0x20 <= v16 - *(long *)(a0 + 0x18)) { // branch-flip
      v19 = *(unsigned long **)(a0 + 0x10);
      v9 = (unsigned long *)(*(long *)(a0 + 0x18) + 0x20);
      if (v9 == v19) {
label_bea7:
        *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 2;
      }
    }
    else {
      _obstack_newchunk(a0,0x20);
      v19 = *(unsigned long **)(a0 + 0x10);
      v16 = *(unsigned long *)(a0 + 0x20);
      v9 = (unsigned long *)(*(long *)(a0 + 0x18) + 0x20);
      if (v9 == v19) goto label_bea7;
    }
    v20 = *(unsigned long *)(a0 + 0x30);
    v22 = ~v20;
    v10 = (long *)((long)v9 + v20 & v22);
    v18 = *(long *)(a0 + 8);
    v21 = v16 - v18;
    if ((unsigned long)((long)v10 - v18) <= v21) { // branch-flip
      *(long **)(a0 + 0x18) = v10;
      *(long **)(a0 + 0x10) = v10;
      *v19 = 0;
      v19[1] = 0;
      if (v16 - (long)v10 <= 0x3f) goto label_bc0f;
      v11 = &v10[8];
      if (v11 == v10) goto label_be4f;
    }
    else {
      *(unsigned long *)(a0 + 0x18) = v16;
      *(unsigned long *)(a0 + 0x10) = v16;
      *v19 = 0;
      v19[1] = 0;
label_bc0f:
      _obstack_newchunk(a0,0x40);
      v20 = *(unsigned long *)(a0 + 0x30);
      v16 = *(unsigned long *)(a0 + 0x20);
      v22 = ~v20;
      v11 = (long *)(*(long *)(a0 + 0x18) + 0x40);
      v18 = *(long *)(a0 + 8);
      v21 = v16 - v18;
      v10 = *(long **)(a0 + 0x10);
      if (v11 == v10) {
label_be4f:
        *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 2;
      }
    }
    v22 = (long)v11 + v20 & v22;
    v17 = v13[v12];
    if (v22 - v18 <= v21)
      v16 = v22;
    *(unsigned long *)(a0 + 0x18) = v16;
    v18 = v7[5];
    *(unsigned long *)(a0 + 0x10) = v16;
    v19[2] = v10;
    v10[5] = v18 + 1;
    v11 = v14[v12];
    *v10 = 0;
    v10[1] = 0;
    v10[2] = (long)v7;
    v10[3] = 0;
    v10[4] = 0;
    v10[6] = 0;
    *(unsigned char *)&v19[3] = v15;
    *(char *)((long)v19 + 0x19) = 0;
    if (v17) // branch-flip
      v11[1] = (long)v19;
    else {
      *v11 = (long)v19;
    }
    v7 = v10;
  } while (!v12);
  while( true ) {
    v10 = v14[v12];
    v8 = *(char *)((long)v10 + 0x19);
    if (v8) break;
    *(unsigned char *)((long)v10 + 0x19) = -(v17 == 0) | 1;
    v12 -= 1;
    if (!v12) goto label_bb05;
    v17 = v13[v12];
  }
  if (v17) { // branch-flip
    if (v17 != 1) goto label_bb05;
    v8 += '\x01';
    *(char *)((long)v10 + 0x19) = v8;
  }
  else {
    v8 += '\xff';
    *(char *)((long)v10 + 0x19) = v8;
  }
  if (v8) {
    if (v8 != '\xfe') { // branch-flip
      if (v8 != '\x02')
        abort(); // no-return, return-dupe
      if (v13[v12 + 1]) { // branch-flip
        if (v13[v12 + 1] != 1) {
          abort();
        }
        v11 = (long *)v10[1];
        v18 = *v11;
        *v11 = (long)v10;
        *(char *)((long)v10 + 0x19) = 0;
        v10[1] = v18;
        *(char *)((long)v11 + 0x19) = 0;
      }
      else {
        v4 = (long *)v10[1];
        v11 = (long *)*v4;
        v18 = v11[1];
        v5 = *v11;
        *v11 = (long)v10;
        v10[1] = v5;
        v11[1] = (long)v4;
        *v4 = v18;
        *(char *)((long)v10 + 0x19) = -(*(char *)((long)v11 + 0x19) == '\x01');
        *(bool *)((long)v4 + 0x19) = *(char *)((long)v11 + 0x19) == '\xff';
        *(char *)((long)v11 + 0x19) = 0;
      }
    }
    else if (v13[v12 + 1]) { // branch-flip
      if (v13[v12 + 1] != 1) {
        abort();
      }
      v18 = *v10;
      v11 = *(long **)(v18 + 8);
      v5 = v11[1];
      v6 = *v11;
      *v11 = v18;
      *(long *)(v18 + 8) = v6;
      v11[1] = (long)v10;
      *v10 = v5;
      *(char *)(v18 + 0x19) = -(*(char *)((long)v11 + 0x19) == '\x01');
      *(bool *)((long)v10 + 0x19) = *(char *)((long)v11 + 0x19) == '\xff';
      *(char *)((long)v11 + 0x19) = 0;
    }
    else {
      v11 = (long *)*v10;
      v18 = v11[1];
      v11[1] = (long)v10;
      *(char *)((long)v10 + 0x19) = 0;
      *v10 = v18;
      *(char *)((long)v11 + 0x19) = 0;
    }
    if (v13[v12 + -1]) // branch-flip
      v14[v12 + -1][1] = (long)v11;
    else {
      *v14[v12 + -1] = (long)v11;
    }
  }
  goto label_bb05;
}

// Function: sub_bf90 @ 0xbf90
unsigned long sub_bf90(long a0)
{
  return *(unsigned long *)(a0 + 0x58);
}

// Function: sub_bfa0 @ 0xbfa0
void sub_bfa0(unsigned long *a0) // return-dupe
{
  unsigned long *v1;
  unsigned char v10; // al
  unsigned long v11;
  long v12; // rax
  unsigned long *v13;
  unsigned long v14;
  unsigned long v15;
  long v16;
  long v17;
  unsigned long *v18;
  long *v19;
  unsigned long *v2;
  unsigned long v20;
  unsigned long v21; // rdx
  unsigned long v22; // rdx
  unsigned long v23 [33];
  long *v24; // stack - 0x950
  unsigned long v25 [256];
  long v26;
  unsigned long *v27;
  long *v28;
  long **v29;
  long v3;
  unsigned long *v30;
  long v31;
  unsigned int v32; // r13d
  unsigned int v33;
  unsigned int v34;
  unsigned int v35; // stack - 0x96c
  unsigned long v4;
  char v5 [16];
  char v6 [16];
  char v7 [16];
  char v8 [16];
  char v9;
  
  v3 = a0[0x130];
  v30 = v23;
  if (!v3)
    v30 = &a0[0xe];
  v31 = a0[0xb];
  if (v31 == 1) {
    v29 = &v24;
    v19 = (long *)a0[0xc];
    v28 = v19;
    v24 = v19;
    for (; v19; v19 = (long *)v19[3]) {
      sub_ae00(v19[1],v29);
    }
    v11 = a0[4];
    v14 = a0[0xd];
    if (v14 <= v11 - a0[3]) { // branch-flip
      v26 = a0[2];
      v17 = a0[3] + v14;
      if (v17 == v26) {
label_c855:
        *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] | 2;
      }
    }
    else {
      _obstack_newchunk(a0,v14);
      v26 = a0[2];
      v11 = a0[4];
      v28 = (long *)a0[0xc];
      v17 = a0[3] + v14;
      v14 = a0[0xd];
      if (v17 == v26) goto label_c855;
    }
    a0[0x12e] = v26;
    v15 = v17 + a0[6] & ~a0[6];
    if (v15 - a0[1] <= v11 - a0[1])
      v11 = v15;
    a0[3] = v11;
    a0[2] = v11;
    if (1 <= (long)v14) {
      v17 = 0;
      while( true ) {
        *(char *)(v26 + v17) = *(char *)(v28[1] + 0x18);
        v17 += 1;
        v28 = (long *)v28[3];
        if ((long)a0[0xd] <= v17) break;
        v26 = a0[0x12e];
      }
    }
    v18 = (unsigned long *)sub_b960(a0[0x130]);
    v20 = a0[0xd];
    v4 = a0[0x12e];
    v18[0x133] = sub_b930;
    sub_bab0(v18,v4,v20);
    _obstack_free(a0,0);
    v26 = 0x134;
    v13 = v18;
    v27 = a0;
    while (v26) {
      v2 = &v27[1];
      v1 = &v13[1];
      *v27 = *v13;
      v26 -= 1;
      v13 = v1;
      v27 = v2;
    }
    free(v18);
    v11 = a0[0xd];
    if (0x100 <= (long)v11)
      v11 = 0xff;
    v11 &= 0xff;
    v7._8_8_ = 0;
    v7._0_8_ = v11;
    *v30 = SUB168(v7._0_16_ * ZEXT816(0x101010101010101),0);
    v26 = v11 * 0x101010101010101 + SUB168(v7._0_16_ * ZEXT816(0x101010101010101),8);
    v30[1] = v26;
    v30[3] = v26;
    v30[2] = v11 * 0x101010101010101;
    v30[5] = v26;
    v30[7] = v26;
    v30[9] = v26;
    v30[4] = v11 * 0x101010101010101;
    v30[0xb] = v26;
    v30[0xd] = v26;
    v30[0xf] = v26;
    v30[6] = v11 * 0x101010101010101;
    v30[0x11] = v26;
    v30[0x13] = v26;
    v30[0x15] = v26;
    v30[8] = v11 * 0x101010101010101;
    v30[0x17] = v26;
    v30[0x19] = v26;
    v30[0x1b] = v26;
    v30[10] = v11 * 0x101010101010101;
    v30[0x1d] = v26;
    v30[0xc] = v11 * 0x101010101010101;
    v30[0xe] = v11 * 0x101010101010101;
    v30[0x10] = v11 * 0x101010101010101;
    v30[0x12] = v11 * 0x101010101010101;
    v30[0x14] = v11 * 0x101010101010101;
    v30[0x16] = v11 * 0x101010101010101;
    v30[0x18] = v11 * 0x101010101010101;
    v30[0x1a] = v11 * 0x101010101010101;
    v30[0x1c] = v11 * 0x101010101010101;
    v8._8_8_ = 0;
    v8._0_8_ = v11;
    v20 = SUB168(v8._0_16_ * ZEXT816(0x101010101010101),8);
    v30[0x1e] = SUB168(v8._0_16_ * ZEXT816(0x101010101010101),0);
    v30[0x1f] = v26;
    v19 = (long *)a0[0xc];
    if (v19) goto label_c164;
    v26 = 0;
    v24 = v19;
label_c3d0:
    while (v26 = *(long *)(v26 + 0x18), v26) {
      while( true ) {
        v17 = *(long *)(*(long *)(v26 + 0x10) + 0x38);
        v16 = *(long *)(v26 + 0x38);
        if (v17 < *(long *)(v26 + 0x38)) {
          *(long *)(v26 + 0x38) = v17;
          v16 = v17;
        }
        if (*(long *)(v26 + 0x30) <= v16) break;
        *(long *)(v26 + 0x30) = v16;
        v26 = *(long *)(v26 + 0x18);
        if (!v26) goto label_c233;
      }
    }
  }
  else {
    v11 = a0[0xd];
    if (0x100 <= (long)v11)
      v11 = 0xff;
    v11 &= 0xff;
    v5._8_8_ = 0;
    v5._0_8_ = v11;
    *v30 = SUB168(v5._0_16_ * ZEXT816(0x101010101010101),0);
    v26 = v11 * 0x101010101010101 + SUB168(v5._0_16_ * ZEXT816(0x101010101010101),8);
    v30[1] = v26;
    v30[3] = v26;
    v30[2] = v11 * 0x101010101010101;
    v30[5] = v26;
    v30[7] = v26;
    v30[9] = v26;
    v30[4] = v11 * 0x101010101010101;
    v30[0xb] = v26;
    v30[0xd] = v26;
    v30[0xf] = v26;
    v30[6] = v11 * 0x101010101010101;
    v30[0x11] = v26;
    v30[0x13] = v26;
    v30[0x15] = v26;
    v30[8] = v11 * 0x101010101010101;
    v30[0x17] = v26;
    v30[0x19] = v26;
    v30[0x1b] = v26;
    v30[10] = v11 * 0x101010101010101;
    v30[0x1d] = v26;
    v30[0xc] = v11 * 0x101010101010101;
    v30[0xe] = v11 * 0x101010101010101;
    v30[0x10] = v11 * 0x101010101010101;
    v30[0x12] = v11 * 0x101010101010101;
    v30[0x14] = v11 * 0x101010101010101;
    v30[0x16] = v11 * 0x101010101010101;
    v30[0x18] = v11 * 0x101010101010101;
    v30[0x1a] = v11 * 0x101010101010101;
    v30[0x1c] = v11 * 0x101010101010101;
    v6._8_8_ = 0;
    v6._0_8_ = v11;
    v20 = SUB168(v6._0_16_ * ZEXT816(0x101010101010101),8);
    v30[0x1e] = SUB168(v6._0_16_ * ZEXT816(0x101010101010101),0);
    v30[0x1f] = v26;
    v19 = (long *)a0[0xc];
    v24 = v19;
    if (v19) {
label_c164:
      v29 = &v24;
      v24 = v19;
      do {
        while( true ) {
          v26 = v19[1];
          sub_ae00(v26,v29,v20,v31 == 1);
          sub_af10(v26,v19[5],v30);
          v26 = v19[1];
          sub_ae50(v26,v19[4],a0[0xc]);
          if (v31 == 1) break;
label_c170:
          v19 = (long *)v19[3];
          v20 = v21;
          if (!v19) goto label_c227;
        }
        v17 = a0[0xd];
        v16 = v19[4];
        v19[6] = v17;
        v19[7] = v17;
        if (!v16) goto label_c170;
        v17 = *v19;
        do {
          v9 = sub_af60(*(unsigned long *)(v16 + 8),v26);
          if ((!v9) && (v12 = v19[5] - *(long *)(v16 + 0x28), v12 < *(long *)(v16 + 0x30)))
            *(long *)(v16 + 0x30) = v12;
          if ((v17) && (v12 = v19[5] - *(long *)(v16 + 0x28), v12 < *(long *)(v16 + 0x38)))
            *(long *)(v16 + 0x38) = v12;
          v16 = *(long *)(v16 + 0x20);
        } while (v16);
        v19 = (long *)v19[3];
        v20 = v22;
      } while (v19);
label_c227:
      if (v31 == 1) {
        v26 = a0[0xc];
        goto label_c3d0;
      }
    }
  }
label_c233:
  v13 = &a0[0x2e];
  if (v3)
    v13 = v25;
  v26 = 0;
  *v13 = 0;
  v13[0xff] = 0;
  v11 = (unsigned long)(((int)v13 - (int)(unsigned long *)((unsigned long)&v13[1] & 0xfffffffffffffff8)) + 0x800U >> 3);
  v27 = (unsigned long *)((unsigned long)&v13[1] & 0xfffffffffffffff8);
  while (v11) {
    v18 = &v27[1];
    *v27 = 0;
    v11 -= 1;
    v27 = v18;
  }
  sub_afe0(*(unsigned long *)(a0[0xc] + 8),v13);
  v35 = 0xffffffff;
  v33 = 0xfffffffe;
  do {
    while( true ) {
      v32 = (unsigned int)v26;
      if (v3) { // branch-flip
        v10 = sub_adc0((int)*(char *)(v3 + v26));
        v34 = (unsigned int)v10;
        v17 = v13[v10];
        a0[v26 + 0x2e] = v17;
      }
      else {
        v17 = a0[v26 + 0x2e];
        v34 = v32;
      }
      if (v17) break;
label_c2f0:
      v34 = v33;
      v26 += 1;
      v33 = v34;
      if (v26 == 0x100) goto label_c32d;
    }
    if (v33 != 0xfffffffe) {
      if (v34 != v33) { // branch-flip
        if ((v34 != v32) || (v33 != v35))
          v33 = 0xffffffff;
        else {
          v35 = v32;
        }
      }
      else {
        if (v33 != v35)
          v32 = 0xffffffff;
        v35 = v32;
      }
      goto label_c2f0;
    }
    v26 += 1;
    v33 = v34;
    v35 = v32;
  } while (v26 != 0x100);
label_c32d:
  *(unsigned int *)&a0[0x131] = v34;
  *(unsigned int *)((long)a0 + 0x98c) = v35;
  if (v31 != 1) goto label_c34a;
  v11 = a0[4];
  v14 = a0[0xd];
  if (v14 <= v11 - a0[3]) { // branch-flip
    v31 = a0[2];
    v26 = a0[3] + v14;
    v15 = v14;
    if (v26 == v31) {
label_c821:
      *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] | 2;
    }
  }
  else {
    _obstack_newchunk(a0);
    v31 = a0[2];
    v11 = a0[4];
    v15 = a0[0xd];
    v26 = a0[3] + v14;
    if (v26 == v31) goto label_c821;
  }
  a0[0x12e] = v31;
  v14 = v26 + a0[6] & ~a0[6];
  if (v14 - a0[1] <= v11 - a0[1])
    v11 = v14;
  a0[3] = v11;
  a0[2] = v11;
  v26 = a0[0xc];
  if (1 <= (long)v15) {
    while( true ) {
      v15 -= 1;
      *(char *)(v31 + v15) = *(char *)(*(long *)(v26 + 8) + 0x18);
      v26 = *(long *)(v26 + 0x18);
      if (!v15) break;
      v31 = a0[0x12e];
    }
    v31 = a0[0xd];
    if (2 <= v31) {
      v14 = a0[4];
      v26 = a0[3];
      v11 = v31 * 8 - 8;
      if (v14 - v26 < v11) {
        _obstack_newchunk(a0,v11);
        v26 = a0[3];
        v14 = a0[4];
        v31 = a0[0xd];
      }
      v17 = a0[2];
      if (v26 + v11 == v17)
        *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] | 2;
      a0[0x12f] = v17;
      v11 = v26 + v11 + a0[6] & ~a0[6];
      if (v11 - a0[1] <= v14 - a0[1])
        v14 = v11;
      a0[3] = v14;
      a0[2] = v14;
      v26 = *(long *)(a0[0xc] + 0x18);
      if (2 <= v31) {
        v16 = 0;
        do {
          v13 = (unsigned long *)(v26 + 0x30);
          v26 = *(long *)(v26 + 0x18);
          *(unsigned long *)(v17 + v16 * 8) = *v13;
          v31 = a0[0xd];
          v16 += 1;
        } while (v16 < v31 + -1);
      }
      v9 = *(char *)(a0[0x12e] + -2 + v31);
      *(char *)&a0[0x132] = sub_add0(v3,(int)v9);
    }
  }
label_c34a:
  v31 = 0;
  if (v3) {
    do {
      v9 = *(char *)(v3 + v31);
      *(char *)((long)a0 + v31 + 0x70) = *(char *)((long)v30 + (unsigned long)(unsigned char)sub_adc0((int)v9));
      v31 += 1;
    } while (v31 != 0x100);
    return;
  }
}

// Function: sub_c870 @ 0xc870
void sub_c870(long a0)
{
  (**(void **)(a0 + 0x998))(); // jump-as-call
}

// Function: sub_c890 @ 0xc890
void sub_c890(void *a0)
{
  _obstack_free(a0,0);
  free(a0); // tail-call
}

// Function: sub_c8b0 @ 0xc8b0
unsigned long sub_c8b0(int a0)
{
  unsigned long v1; // rax
  
  if (a0 == 0x5f)
    return 1;
  v1 = iswalnum();
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),(int)v1 != 0);
}

// Function: sub_c8e0 @ 0xc8e0
long sub_c8e0(long a0,long a1,char a2)
{
  unsigned long v1; // rax
  long v2; // rax
  unsigned long v3; // stack - 0x48
  unsigned int v4; // stack - 0x4c
  long v5; // r12
  
  v5 = 0;
  v3 = 0;
  do {
    while( true ) {
      if (a1 - a0 <= v5)
        return v5;
      v1 = (unsigned long)*(unsigned char *)(a0 + v5);
      if (!*(char *)(v1 + 0x2a9c0)) break;
      v5 += 1;
      if (!a2)
        return v5;
    }
    if (*(char *)(v1 + 0x2a183) != '\xfe')
      return v5;
    v4 = 0;
    v2 = sub_18c40(&v4,(unsigned char *)(a0 + v5),(a1 - a0) - v5,&v3);
    if (!sub_c8b0(v4))
      return v5;
    v5 = v5 + v2 + (unsigned long)(v2 == 0);
  } while (a2);
  return v5;
}

// Function: sub_c9d0 @ 0xc9d0
unsigned int sub_c9d0(unsigned int a0)
{
  return a0;
}

// Function: sub_c9e0 @ 0xc9e0
void sub_c9e0(void)
{
  sub_18c10(); // tail-call
}

// Function: sub_c9f0 @ 0xc9f0
void sub_c9f0(char *a0)
{
  char v1;
  long v2; // rcx
  
  v1 = *a0;
  v2 = 0x2a180;
  if (*(char *)(v2 + 3 + (unsigned long)(unsigned char)sub_c9d0((int)v1)) != '\xfe')
    return;
  sub_c9e0(a0); // tail-call
}

// Function: sub_ca30 @ 0xca30
void sub_ca30(void)
{
  unsigned int v1;
  unsigned int *v2; // rbx
  char *v3;
  char *v4; // rbp
  
  v2 = (unsigned int *)0x2a284;
  v3 = (char *)0x2a9c0;
  do {
    v1 = *v2;
    v2 = &v2[1];
    v4 = &v3[1];
    *v3 = sub_c8b0(v1);
    v3 = v4;
  } while (v2 != (unsigned int *)0x2a684);
}

// Function: sub_ca70 @ 0xca70
void sub_ca70(char a0)
{
  long v1; // rax
  long v2; // rbx
  
  if (dat_2a687) {
    if ((__ctype_get_mb_cur_max() == 1) || (a0)) {
      v2 = 0;
      v1 = sub_1d110(0x100);
      do {
        *(char *)(v1 + v2) = (char)toupper((int)v2);
        v2 += 1;
      } while (v2 != 0x100);
      sub_b960(v1); // tail-call
      return;
    }
  }
  sub_b960(0); // tail-call
}

// Function: sub_caf0 @ 0xcaf0
long sub_caf0(unsigned long *a0,long *a1,char *a2,long a3) // early-return, return-dupe
{
  unsigned int v1; // eax
  long v2;
  char *v3;
  unsigned long v4; // stack - 0x48
  char *v5; // r12
  char *v6;
  
  v6 = (char *)*a0;
  if (v6 < a2) {
    if (dat_2a182) { // branch-flip
      if (((int)*a2 & 0xc0U) != 0x80) {
label_cb95:
        *a0 = a2;
        return 0;
      }
      v1 = (unsigned int)a2[-1];
      if ((v1 & 0xc0) != 0x80) // branch-flip
        v2 = 1;
      else {
        v1 = (unsigned int)a2[-2];
        if ((v1 & 0xc0) != 0x80) // branch-flip
          v2 = 2;
        else {
          v1 = (unsigned int)a2[-3];
          if ((v1 & 0xc0) == 0x80) goto label_cb95;
          v2 = 3;
        }
      }
      if ((int)(~v1 & 0xff) >> (7U - (char)v2 & 0x1fU)) goto label_cb95;
      v4 = 0;
      v5 = &a2[-v2];
      v2 = sub_c9e0(v5,a3 - (long)v5,&v4);
      v3 = a2;
      if (0 <= v2) {
        v3 = &v5[v2];
        v6 = v5;
      }
    }
    else {
      v4 = 0;
      while( true ) {
        v2 = sub_c9f0(v6,a3 - (long)v6,&v4);
        if (0 <= v2) // branch-flip
          v3 = &v6[v2];
        else {
          v4 = 0;
          v2 = 1;
          v3 = &v6[1];
        }
        if (a2 <= v3) break;
        v6 = v3;
      }
      if (a1)
        *a1 = v2;
    }
    *a0 = v3;
    if (v3 == a2)
      return 0;
  }
  return (long)a2 - (long)v6;
}

// Function: sub_cc80 @ 0xcc80
void sub_cc80(unsigned long a0,unsigned long a1)
{
  sub_c8e0(a0,a1,1); // tail-call
}

// Function: sub_cc90 @ 0xcc90
void sub_cc90(unsigned long a0,unsigned long a1)
{
  sub_c8e0(a0,a1,0); // tail-call
}

// Function: sub_cca0 @ 0xcca0
unsigned long sub_cca0(long a0,long a1,unsigned long a2) // early-return
{
  unsigned long v1; // rax
  long v2; // stack - 0x28
  
  if (a0 == a1)
    return 0;
  if ((dat_2a180) && (!(~(*(unsigned char *)(a1 + -1) >> 7) & dat_2a182))) {
    v2 = a0;
    v1 = sub_cc90((a1 + -1) - sub_caf0(&v2,0,a1 + -1,a2),a2); // tail-call
    return v1;
  }
  return (unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)(a1 + -1) + 0x2a9c0);
}

// Function: sub_cd60 @ 0xcd60
void sub_cd60(void *a0)
{
  free(a0); // tail-call
}

// Function: sub_cd70 @ 0xcd70
void sub_cd70(long a0)
{
  if (0 <= a0) {
    sub_1d110(); // tail-call
    return;
  }
  sub_1d4b0(); // no-return
}

// Function: sub_cd90 @ 0xcd90
int sub_cd90(unsigned long *a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int a4)
{
  int v1;
  long v2;
  long v3;
  int v4; // stack - 0x44
  bool v5; // sf
  unsigned long v6; // stack - 0x60
  
  while( true ) {
    while (v1 = pcre2_match_8(a0[1],a1,a2,a3,a4,a0[3],a0[2]), v1 == -0x2e) {
      v2 = a0[5];
      if (0x4000000000000000 <= v2)
        return -0x2e;
      a0[5] = v2 * 2;
      pcre2_jit_stack_free_8(a0[4]);
      v2 = pcre2_jit_stack_create_8(v2,v2 * 2,*a0);
      a0[4] = v2;
      if (!v2)
        sub_1d4b0(); // no-return
      v3 = a0[2];
      if (!v3) {
        v3 = pcre2_match_context_create_8(*a0);
        v2 = a0[4];
        a0[2] = v3;
      }
      v6 = 0xcec2;
      pcre2_jit_stack_assign_8(v3,0,v2);
    }
    if (v1 != -0x35)
      return v1;
    pcre2_config_8(7,&v4,v6);
    v5 = v4 <= -1;
    v4 *= 2;
    if (v5) break;
    v2 = a0[2];
    v1 = v4;
    if (!v2) {
      v2 = pcre2_match_context_create_8(*a0);
      a0[2] = v2;
      v1 = v4;
    }
    v6 = 0xcdf8;
    pcre2_set_depth_limit_8(v2,v1);
  }
  return 0xffffffcb;
}

// Function: sub_cef0 @ 0xcef0
unsigned long sub_cef0(void)
{
  return 0;
}

// Function: sub_cf00 @ 0xcf00
unsigned long * sub_cf00(char *a0,unsigned long a1)
{
  unsigned long *v1; // rax
  unsigned long *v10; // rax
  int v11; // stack - 0x154
  char v12 [8];
  char v13 [264];
  unsigned int v14; // r13d
  bool v15; // cf
  unsigned long v16; // stack - 0x168
  char *v17; // stack - 0x160
  unsigned long v2; // rax
  unsigned long v3; // rax
  long v4; // rax
  unsigned long v5; // rax
  char *v6; // rax
  unsigned long *v7; // rax
  unsigned long *v8; // rax
  unsigned long *v9; // rax
  
  v15 = dat_2a687 == '\0';
  v1 = (unsigned long *)sub_1d110(0x38);
  v14 = (-(unsigned int)v15 & 0xfffffff8) + 0x18;
  v2 = pcre2_general_context_create_8(sub_cd70,sub_cd60,0);
  *v1 = v2;
  v3 = pcre2_compile_context_create_8(v2);
  if (dat_2a180) {
    if (!dat_2a182) {
      error(2,0,dcgettext(NULL,"-P supports only unibyte and UTF-8 locales",5));
      return v8;
    }
    v14 |= 0x4080000;
  }
  if (&a0[a1] != rawmemchr(a0,10)) {
    error(2,0,dcgettext(NULL,"the -P option only supports a single pattern",5));
    return v10;
  }
  v16 = a1;
  if (dat_2a685) { // branch-flip
    pcre2_set_compile_extra_options_8(v3,8);
    v17 = NULL;
  }
  else {
    v17 = NULL;
    if (dat_2a686) {
      v16 = a1 + 0x11;
      v17 = (char *)sub_1d110(v16);
      *(unsigned long *)v17 = s_20990._0_8_;
      *(unsigned short *)&v17[8] = s_20990._8_2_;
      v6 = mempcpy(&v17[10],a0,a1);
      *(unsigned int *)v6 = s_20988._0_4_;
      *(unsigned short *)&v6[4] = s_20988._4_2_;
      v6[6] = s_20988[6];
      a0 = v17;
    }
  }
  pcre2_set_character_tables_8(v3,pcre2_maketables_8(v2));
  v4 = pcre2_compile_8(a0,v16,v14,&v11,v12,v3);
  v1[1] = v4;
  if (!v4) {
    pcre2_get_error_message_8(v11,v13,0x100);
    error(2,0,"%s",v13);
    return v9;
  }
  free(v17);
  pcre2_compile_context_free_8(v3);
  v1[2] = 0;
  v2 = pcre2_match_data_create_from_pattern_8(v1[1],v2);
  v1[3] = v2;
  v5 = pcre2_jit_compile_8(v1[1],1);
  v11 = (int)v5;
  if ((v11 + 0x30U <= 0x30) && (!(0xfffefffffffffff6U >> ((unsigned long)(v11 + 0x30U) & 0x3f) & 1))) {
    v1[4] = 0;
    v1[5] = 0x8000;
    *(unsigned int *)&v1[6] = sub_cd90(v1,"",0,0,1);
    *(unsigned int *)((long)v1 + 0x34) = sub_cd90(v1,"",0,0,0);
    return v1;
  }
  error(2,0,dcgettext(NULL,"JIT internal error: %d",5),v5 & 0xffffffff);
  return v7;
}

// Function: sub_d220 @ 0xd220
long sub_d220(long a0,char *a1,long a2,long *a3,char *a4)
{
  char v1;
  char *v10;
  unsigned int v11; // r13d
  unsigned long v12; // r14
  char *v13; // stack - 0x60
  int v2;
  long *v3; // rax
  char *v4;
  long v5; // rax
  unsigned long v6; // rax
  long v7; // rax
  char *v8;
  long v9; // rbp
  
  v8 = a1;
  if (a4)
    v8 = a4;
  v12 = (unsigned long)(v8[-1] == dat_2a684);
  v3 = (long *)pcre2_get_ovector_pointer_8(*(unsigned long *)(a0 + 0x18));
  v13 = a1;
  do {
    v4 = rawmemchr(v8,(int)dat_2a684);
    v10 = v13;
    while( true ) {
      while (v1 = *v8, *(char *)((unsigned long)(unsigned char)sub_c9d0((int)v1) + 0x2a183) == '\xff') {
        v8 = &v8[1];
        v12 = 0;
        v10 = v8;
      }
      v9 = (long)v8 - (long)v10;
      if (v8 == v4) break;
      v11 = (unsigned int)v12 ^ 1;
      v2 = sub_cd90(a0,v10,(long)v4 - (long)v10,v9,v11);
      if (!sub_cef0()) goto label_d41b;
      v5 = pcre2_get_startchar_8(*(unsigned long *)(a0 + 0x18));
      if (v9 <= v5) { // branch-flip
        if (v5) // branch-flip
          v2 = sub_cd90(a0,v10,v5,v9,v11 | 0x40000002);
        else {
          v2 = *(int *)(a0 + 0x30 + v12 * 4);
          *v3 = 0;
          v3[1] = 0;
        }
        if (v2 != -1) goto label_d39e;
        v8 = &v10[v5 + 1];
        v12 = 0;
        v10 = v8;
      }
      else {
        v10 = &v10[v5 + 1];
      }
    }
    v2 = *(int *)(a0 + 0x30 + v12 * 4);
    v3[1] = v9;
    *v3 = v9;
label_d41b:
    if (v2 != -1) {
label_d39e:
      if (1 <= v2) {
        v4 = &v4[1];
        if (a4) {
          v13 = &v10[*v3];
          v4 = &v10[v3[1]];
        }
        *a3 = (long)v4 - (long)v13;
        return (long)v13 - (long)a1;
      }
      if ((v2 < -0x2d) && (-0x3f <= v2)) {
        switch(v2) {
          case 0xffffffc1:
            v6 = sub_8a50();
            error(2,0,dcgettext(NULL,"%s: exceeded PCRE\'s heap limit",5),v6);
            goto label_d4ff;
          case 0xffffffcc:
label_d58c:
            v6 = sub_8a50();
            error(2,0,dcgettext(NULL,"%s: PCRE detected recurse loop",5),v6);
            goto label_d5bb;
          case 0xffffffcb:
            v6 = sub_8a50();
            error(2,0,dcgettext(NULL,"%s: exceeded PCRE\'s nested backtracking limit",5),v6);
          case 0xffffffd0:
label_d55d:
            v6 = sub_8a50(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
            error(2,0,dcgettext(NULL,"%s: memory exhausted",5),v6);
            goto label_d58c;
          case 0xffffffd1:
label_d52e:
            v6 = sub_8a50();
            error(2,0,dcgettext(NULL,"%s: exceeded PCRE\'s backtracking limit",5),v6);
            goto label_d55d;
          case 0xffffffd2:
label_d4ff:
            v6 = sub_8a50();
            error(2,0,dcgettext(NULL,"%s: exhausted PCRE JIT stack",5),v6);
            goto label_d52e;
          
        }
      }
      v6 = sub_8a50();
      error(2,0,dcgettext(NULL,"%s: internal PCRE error: %d",5),v6,v2);
      return v7;
    }
    v12 = 1;
    v8 = &v4[1];
    if (&a1[a2] <= v8)
      return -1;
    v13 = v8;
  } while( true );
}

// Function: sub_d5f0 @ 0xd5f0
void sub_d5f0(void)
{
  sub_a440(1); // tail-call, no-return
}

// Function: sub_d600 @ 0xd600
unsigned long sub_d600(char *a0,long *a1,void *a2,unsigned long a3)
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

// Function: sub_d710 @ 0xd710
long sub_d710(char *a0,long *a1) // return-dupe
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

// Function: sub_d770 @ 0xd770
void sub_d770(unsigned long a0,unsigned long a1,long a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (a2 != -1) // branch-flip
    v1 = dcgettext(NULL,"ambiguous argument %s for %s",5);
  else {
    v1 = dcgettext(NULL,"invalid argument %s for %s",5);
  }
  v2 = sub_1bad0(1,a0);
  error(0,0,v1,sub_1b750(0,8,a1),v2); // tail-call
}

// Function: sub_d800 @ 0xd800
void sub_d800(long *a0,void *a1,unsigned long a2)
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
      __fprintf_chk(stderr,1,", %s",sub_1bae0(v1));
    else {
      __fprintf_chk(stderr,1,"\n  - %s",sub_1bae0(v1));
      v4 = a1;
    }
    v3 += 1;
    a1 = (void *)((long)a1 + a2);
    v1 = a0[v3];
  }
  putc_unlocked(10,stderr); // tail-call
}

// Function: sub_d900 @ 0xd900
long sub_d900(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,void *a5,char a6) // early-return
{
  long v1; // rax
  
  if (a6) // branch-flip
    v1 = sub_d600(a1,a2,a3,a4);
  else {
    v1 = sub_d710(a1,a2);
  }
  if (0 <= v1)
    return v1;
  sub_d770(a0,a1,v1);
  sub_d800(a2,a3,a4);
  (*a5)();
  return -1;
}

// Function: sub_d990 @ 0xd990
long sub_d990(void *a0,long *a1,void *a2,unsigned long a3) // return-dupe
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

// Function: sub_d9f0 @ 0xd9f0
unsigned long sub_d9f0(int a0) // early-return
{
  unsigned long v1; // rax
  
  if (0x5a < a0)
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),(unsigned int)(a0 - 0x61U) <= 0x19);
  if (0x41 <= a0)
    return 1;
  return (unsigned long)((unsigned int)(a0 - 0x30U) <= 9);
}

// Function: sub_da20 @ 0xda20
bool sub_da20(int a0)
{
  if (a0 <= 0x5a)
    return 0x40 < a0;
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_da40 @ 0xda40
bool sub_da40(unsigned int a0)
{
  return a0 <= 0x7f;
}

// Function: sub_da50 @ 0xda50
unsigned int sub_da50(int a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x20) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == 9);
}

// Function: sub_da70 @ 0xda70
unsigned long sub_da70(unsigned int a0)
{
  unsigned long v1; // rax
  
  if ((int)a0 <= 0x1f)
    return (unsigned long)(~a0 >> 0x1f);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x7f);
}

// Function: sub_da90 @ 0xda90
bool sub_da90(int a0)
{
  return (unsigned int)(a0 - 0x30U) <= 9;
}

// Function: sub_daa0 @ 0xdaa0
bool sub_daa0(int a0)
{
  return (unsigned int)(a0 - 0x21U) <= 0x5d;
}

// Function: sub_dab0 @ 0xdab0
bool sub_dab0(int a0)
{
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_dac0 @ 0xdac0
bool sub_dac0(int a0)
{
  return (unsigned int)(a0 - 0x20U) <= 0x5e;
}

// Function: sub_dad0 @ 0xdad0
unsigned long sub_dad0(int a0)
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

// Function: sub_db20 @ 0xdb20
bool sub_db20(int a0)
{
  if (a0 <= 0xd)
    return 8 < a0;
  return a0 == 0x20;
}

// Function: sub_db40 @ 0xdb40
bool sub_db40(int a0)
{
  return (unsigned int)(a0 - 0x41U) <= 0x19;
}

// Function: sub_db50 @ 0xdb50
unsigned int sub_db50(int a0)
{
  if ((unsigned int)(a0 - 0x30U) <= 0x36)
    return (unsigned int)(0x7e0000007e03ff >> ((unsigned char)(a0 - 0x30U) & 0x3f)) & 1;
  return 0;
}

// Function: sub_db80 @ 0xdb80
int sub_db80(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_dba0 @ 0xdba0
int sub_dba0(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_dbc0 @ 0xdbc0
void sub_dbc0(void)
{
  return;
}

// Function: sub_dbd0 @ 0xdbd0
void sub_dbd0(int a0)
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
  (*dat_2aae0)();
  if (a0) // branch-flip
    v6 = dat_2aad8;
  else {
    v6 = dat_2aad0;
  }
  v1 = strlen(dat_2aac8);
  v2 = strlen(v6);
  if (0x1000 <= v1 + v2) { // branch-flip
    write(2,dat_2aac8,v1);
    write(2,": unbalanced [",2);
    write(2,v6,v2);
    v5[0] = 10;
    v4 = (char *)0x1;
  }
  else {
    v3 = __mempcpy_chk(v5,dat_2aac8,v1,0x1002);
    *(unsigned short *)v3 = s_20a11._0_2_;
    v4 = mempcpy(&v3[2],v6,v2);
    *v4 = 10;
    v4 = &v4[1 - (long)v5];
  }
  write(2,v5,(unsigned long)v4);
  if (!a0)
    _exit(dat_2a080); // no-return
  raise(a0);
  abort(); // no-return
}

// Function: sub_dcf0 @ 0xdcf0
unsigned long sub_dcf0(unsigned long a0,int a1)
{
  if (!a1)
    return 0;
  sub_dbd0(0xb); // no-return
}

// Function: sub_dd10 @ 0xdd10
void sub_dd10(unsigned long a0)
{
  if ((int)a0)
    a0 = (unsigned long)(-(unsigned int)(dat_2aac0 == 0) & 0xb);
  sub_dbd0(a0); // no-return
}

// Function: sub_dd40 @ 0xdd40
unsigned long sub_dd40(void *a0) // return-dupe
{
  if (!a0)
    a0 = sub_dbc0;
  dat_2aae0 = a0;
  dat_2aad8 = dcgettext(NULL,"program error",5);
  dat_2aad0 = dcgettext(NULL,"stack overflow",5);
  dat_2aac8 = sub_179e0();
  if (!sub_1bf40(sub_dd10,0x2ab00,0x10000)) {
    dat_2aac0 = sub_1bec0(sub_dcf0);
    return 0;
  }
  *__errno_location() = 0x5f;
  return 0xffffffff;
}

// Function: sub_ddf0 @ 0xddf0
int sub_ddf0(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned int v2; // eax
  
  if (a0 == a1)
    return 0;
  do {
    v2 = sub_db80(*a0);
    v1 = sub_db80(*a1);
    if (!(unsigned char)v2) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while ((unsigned char)v2 == v1);
  return (v2 & 0xff) - (unsigned int)v1;
}

// Function: sub_de50 @ 0xde50
void sub_de50(unsigned long a0)
{
  dat_3ab08 = a0;
}

// Function: sub_de60 @ 0xde60
void sub_de60(char a0)
{
  dat_3ab00 = a0;
}

// Function: sub_de70 @ 0xde70
void sub_de70(void)
{
  int v1; // eax
  int *v2; // rax
  char *v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_1dad0(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!dat_3ab00) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (dat_3ab08) { // branch-flip
        v4 = sub_1b900(dat_3ab08);
        error(0,*v2,"%s: %s",v4,v3);
      }
      else {
        error(0,*v2,"%s",v3);
      }
      _exit(dat_2a080); // return-dupe, no-return
    }
  }
  v1 = sub_1dad0(stderr);
  if (!v1)
    return;
  _exit(dat_2a080);
}

// Function: sub_df20 @ 0xdf20
bool sub_df20(char a0)
{
  return (unsigned char)(a0 - 0x30U) <= 9;
}

// Function: sub_df30 @ 0xdf30
unsigned int sub_df30(unsigned int a0)
{
  return a0;
}

// Function: sub_df40 @ 0xdf40
unsigned int sub_df40(int a0)
{
  return a0 >> 6 & 7;
}

// Function: sub_df50 @ 0xdf50
unsigned int sub_df50(int a0)
{
  return a0 >> 3 & 7;
}

// Function: sub_df60 @ 0xdf60
unsigned int sub_df60(unsigned int a0)
{
  return a0 & 7;
}

// Function: sub_df70 @ 0xdf70
unsigned int sub_df70(unsigned long a0,unsigned int a1,unsigned long a2)
{
  unsigned int v1;
  unsigned int v2; // eax
  unsigned int v3;
  unsigned long v4; // rdx
  unsigned long v5; // rdx
  
  v3 = (unsigned int)a2 & 1;
  if (a2 & 1) {
    v2 = sub_df60();
    a2 = v4;
    v3 = v2;
  }
  if (a2 & 2) {
    v2 = sub_df50();
    v3 |= v2;
    a2 = v5;
  }
  if (a2 & 4) {
    v2 = sub_df40();
    v3 |= v2;
  }
  v1 = v2 >> 8;
  v2 = CONCAT31((undefined3)v1,(a1 & v3) != 0);
  return v2;
}

// Function: sub_dfa0 @ 0xdfa0
unsigned int sub_dfa0(unsigned int a0)
{
  a0 = (int)a0 >> 2 ^ a0;
  return CONCAT31((undefined3)(a0 >> 8),(a0 & 0x49) != 0);
}

// Function: sub_dfb0 @ 0xdfb0
unsigned int sub_dfb0(unsigned int a0)
{
  a0 = (int)a0 >> 1 ^ a0;
  return CONCAT31((undefined3)(a0 >> 8),(a0 & 0x49) != 0);
}

// Function: sub_dfc0 @ 0xdfc0
unsigned int sub_dfc0(unsigned int a0,long a1)
{
  return (unsigned int)(*(unsigned long *)(a1 + (unsigned long)(a0 >> 6) * 8) >> ((unsigned char)a0 & 0x3f)) & 1;
}

// Function: sub_dfe0 @ 0xdfe0
void sub_dfe0(unsigned int a0,long a1)
{
  unsigned long *v1;
  
  v1 = (unsigned long *)(a1 + (unsigned long)(a0 >> 6) * 8);
  *v1 = *v1 | 1L << ((unsigned char)a0 & 0x3f);
}

// Function: sub_e000 @ 0xe000
void sub_e000(unsigned int a0,long a1)
{
  unsigned long *v1;
  unsigned char v2;
  
  v2 = (unsigned char)a0 & 0x3f;
  v1 = (unsigned long *)(a1 + (unsigned long)(a0 >> 6) * 8);
  *v1 = *v1 & (-2L << v2 | 0xfffffffffffffffeU >> 0x40 - v2);
}

// Function: sub_e020 @ 0xe020
void sub_e020(unsigned long *a0)
{
  *a0 = 0xffffffffffffffff;
  a0[1] = 0xffffffffffffffff;
  a0[2] = 0xffffffffffffffff;
  a0[3] = 0xffffffffffffffff;
}

// Function: sub_e040 @ 0xe040
void sub_e040(unsigned long *a0)
{
  unsigned long *v1;
  
  v1 = &a0[4];
  do {
    *a0 = ~*a0;
    a0 = &a0[1];
  } while (a0 != v1);
}

// Function: sub_e060 @ 0xe060
bool sub_e060(long a0,long a1)
{
  unsigned long *v1;
  unsigned long *v2;
  long v3; // rax
  unsigned long v4; // rcx
  
  v3 = 0;
  v4 = 0;
  do {
    v1 = (unsigned long *)(a0 + v3);
    v2 = (unsigned long *)(a1 + v3);
    v3 += 8;
    v4 |= *v1 ^ *v2;
  } while (v3 != 0x20);
  return v4 == 0;
}

// Function: sub_e080 @ 0xe080
unsigned long sub_e080(unsigned long *a0)
{
  unsigned long v1; // rax
  
  v1 = *a0 | a0[1] | a0[2] | a0[3];
  return CONCAT71((undefined7)(v1 >> 8),v1 == 0);
}

// Function: sub_e0a0 @ 0xe0a0
unsigned int sub_e0a0(long a0,long *a1) // return-dupe
{
  long v1;
  long v2;
  long *v3;
  long *v4; // rax
  long v5;
  long v6;
  long v7;
  
  v1 = a1[1];
  if (v1 <= 0)
    return 0;
  v2 = *a1;
  v5 = 0;
  v7 = v1;
  while( true ) {
    while( true ) {
      v6 = v7 + v5 >> 1;
      v3 = (long *)(v6 * 0x10 + v2);
      if (a0 <= *v3) break;
      v5 = v6 + 1;
      if (v7 <= v5)
        return 0;
    }
    if (*v3 == a0) break;
    v7 = v6;
    if (v6 <= v5)
      return 0;
  }
  v5 = v3[1];
  if (v6 + 1 < v1) {
    do {
      v4 = &v3[2];
      *v3 = v3[2];
      v3[1] = v3[3];
      v3 = v4;
    } while (v4 != (long *)(v2 + -0x10 + v1 * 0x10));
    v6 = ((v1 + -2) - v6) + v6 + 1;
  }
  a1[1] = v6;
  return (int)v5;
}

// Function: sub_e140 @ 0xe140
unsigned int sub_e140(long a0,long a1)
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  unsigned int v4;
  long v5; // rcx
  
  v5 = 0;
  v4 = 0;
  do {
    v1 = *(unsigned long *)(a1 + v5);
    v2 = *(unsigned long *)(a0 + 0x3f0 + v5);
    if (v1 & v2)
      v4 |= 4;
    v3 = *(unsigned long *)(a0 + 0x3d0 + v5);
    if (v1 & v3)
      v4 |= 2;
    if (~(v2 | v3) & v1)
      v4 |= 1;
    v5 += 8;
  } while (v5 != 0x20);
  return v4;
}

// Function: sub_e1a0 @ 0xe1a0
int sub_e1a0(long *a0,long *a1)
{
  return (unsigned int)(*a1 < *a0) - (unsigned int)(*a0 < *a1);
}

// Function: sub_e1c0 @ 0xe1c0
unsigned long sub_e1c0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,char *a5)
{
  *a5 = 1;
  return a1;
}

// Function: sub_e1d0 @ 0xe1d0
unsigned short sub_e1d0(long a0,unsigned char a1)
{
  if (*(int *)(a0 + 0x51c + (unsigned long)a1 * 4) != -1)
    return *(unsigned short *)(*(long *)__ctype_b_loc() + (unsigned long)a1 * 2) >> 3 & 1 | (unsigned short)(a1 == 0x5f);
  return 0;
}

// Function: sub_e210 @ 0xe210
int sub_e210(long a0,char a1)
{
  if ((*(char *)(a0 + 0x1ce) == a1) && (!(*(unsigned char *)(a0 + 0x1c8) & 1)))
    return 4;
  return (unsigned char)sub_e1d0(a0,a1) + 1;
}

// Function: sub_e250 @ 0xe250
void sub_e250(char (*a0)[16])
{
  *a0 = 0;
  a0[1] = 0;
}

// Function: sub_e260 @ 0xe260
long sub_e260(int *a0,unsigned char *a1,unsigned long a2,long a3)
{
  int v1;
  long v2; // rax
  int v3; // stack - 0x24
  
  v1 = *(int *)(a3 + 0x51c + (unsigned long)*a1 * 4);
  if (v1 == -1) {
    v2 = sub_18c40(&v3,a1,a2,a3 + 0x110);
    if ((unsigned long)(v2 - 1U) <= 0xfffffffffffffffc) {
      *a0 = v3;
      return v2;
    }
    *(unsigned long *)(a3 + 0x110) = 0;
  }
  *a0 = v1;
  return 1;
}

// Function: sub_e300 @ 0xe300
unsigned long sub_e300(long a0)
{
  char v1;
  int v2;
  int v3; // eax
  unsigned long v4; // rax
  char *v5;
  long v6;
  long v7; // rdx
  
  v3 = sub_e260(a0 + 0x48,*(unsigned long *)(a0 + 0x20),*(unsigned long *)(a0 + 0x28),a0);
  v2 = v3;
  v5 = *(char **)(a0 + 0x20);
  v6 = (long)v3;
  v4 = 0xffffffff;
  if (v2 == 1) {
    v1 = *v5;
    v4 = (unsigned long)(unsigned char)sub_df30((int)v1);
    v6 = v7;
  }
  *(long *)(a0 + 0x28) = *(long *)(a0 + 0x28) - v6;
  *(char **)(a0 + 0x20) = &v5[v6];
  return v4;
}

// Function: sub_e350 @ 0xe350
void sub_e350(long a0)
{
  if (*(long *)(a0 + 0x28)) {
    sub_e300(a0); // tail-call
    return;
  }
  sub_6630(dcgettext(NULL,"unbalanced [",5)); // no-return
}

// Function: sub_e380 @ 0xe380
unsigned char * sub_e380(long a0,unsigned char *a1,unsigned char *a2,long a3)
{
  char v1 [4];
  unsigned char *v2;
  
  v2 = a1;
  if ((!*(char *)(a0 + 0x2cf + (unsigned long)*a1)) && (v2 = a2, a2 < a1)) {
    do {
      v2 = &v2[(int)sub_e260(v1,v2,a3 - (long)v2,a0)];
    } while (v2 < a1);
  }
  return v2;
}

// Function: sub_e410 @ 0xe410
void sub_e410(long a0,unsigned long *a1,char *a2)
{
  *a1 = *(unsigned long *)(a0 + 0x20);
  a1[1] = *(unsigned long *)(a0 + 0x28);
  *(char **)(a0 + 0x20) = a2;
  *(unsigned long *)(a0 + 0x28) = strlen(a2);
}

// Function: sub_e440 @ 0xe440
unsigned int sub_e440(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = strcmp(a0,a1);
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}

// Function: sub_e460 @ 0xe460
long sub_e460(unsigned long a0)
{
  long *v1; // rbx
  int v2; // ebp
  char *v3;
  
  v3 = "alpha";
  v2 = 0;
  v1 = (long *)0x29878;
  do {
    if (sub_e440(a0,v3))
      return (long)v2 * 0x18 + 0x29860;
    v3 = (char *)*v1;
    v1 = &v1[3];
    v2 += 1;
  } while (v3);
  return 0;
}

// Function: sub_e4c0 @ 0xe4c0
bool sub_e4c0(unsigned long a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = wctob();
  if (0 <= v1)
    sub_dfe0(v1,a1);
  return 0 <= v1;
}

// Function: sub_e4e0 @ 0xe4e0
unsigned long sub_e4e0(unsigned long a0,long a1,long *a2)
{
  if (a1 < *a2)
    return a0;
  return sub_1d2b0(a0,a2,1); // tail-call
}

// Function: sub_e500 @ 0xe500
long sub_e500(long *a0,unsigned long *a1)
{
  unsigned long *v1;
  unsigned long v2;
  unsigned long v3;
  char v4; // al
  long v5;
  long v6;
  long v7;
  long v8; // r12
  long v9; // r13
  
  v5 = a0[1];
  v6 = *a0;
  if (1 <= v5) { // branch-flip
    v8 = 0;
    v7 = v6;
    do {
      v4 = sub_e060(a1,v6);
      if (v4)
        return v8;
      v8 += 1;
      v6 += 0x20;
    } while (v5 != v8);
    v9 = v5 << 5;
    v6 = v7;
  }
  else {
    v9 = 0;
    v8 = 0;
  }
  v5 = sub_e4e0(v6,v5,&a0[2],0x7ffffffffffffeeb,0x20);
  v2 = *a1;
  v3 = a1[1];
  a0[1] = a0[1] + 1;
  *a0 = v5;
  *(unsigned long *)(v5 + v9) = v2;
  ((unsigned long *)(v5 + v9))[1] = v3;
  v2 = a1[3];
  v1 = (unsigned long *)(v5 + 0x10 + v9);
  *v1 = a1[2];
  v1[1] = v2;
  return v8;
}

// Function: sub_e5c0 @ 0xe5c0
void sub_e5c0(long a0,long a1,long *a2)
{
  unsigned int *v1;
  long v10;
  long v2;
  long v3;
  long v4; // rax
  long v5;
  long v6;
  long *v7; // rax
  long *v8;
  long *v9; // rdx
  
  v2 = a2[1];
  v5 = *a2;
  if (1 <= v2) { // branch-flip
    v10 = 0;
    v6 = v2;
    do {
      while( true ) {
        v4 = v10 + v6 >> 1;
        v7 = (long *)(v4 * 0x10 + v5);
        v3 = *v7;
        if (v3 < a0) break;
        if (v3 == a0) {
          v1 = (unsigned int *)&v7[1];
          *v1 = *v1 | (unsigned int)a1;
          return;
        }
        v6 = v4;
        if (v4 <= v10) goto label_e61a;
      }
      v10 = v4 + 1;
    } while (v10 < v6);
label_e61a:
    v5 = sub_e4e0(v5,v2,&a2[2],0xffffffffffffffff,0x10);
    *a2 = v5;
    if (v2 <= v10) // branch-flip
      v7 = (long *)(v5 + v10 * 0x10);
    else {
      v7 = (long *)(v5 + v10 * 0x10);
      v8 = (long *)(v5 + v2 * 0x10);
      do {
        v9 = &v8[-2];
        *v8 = v8[-2];
        v8[1] = v8[-1];
        v8 = v9;
      } while (v7 != v9);
    }
  }
  else {
    v7 = (long *)sub_e4e0(v5,v2,&a2[2],0xffffffffffffffff,0x10);
    *a2 = (long)v7;
  }
  *v7 = a0;
  v7[1] = a1;
  a2[1] = a2[1] + 1;
}

// Function: sub_e6c0 @ 0xe6c0
void sub_e6c0(unsigned long a0,unsigned long a1,long *a2)
{
  long v1; // rax
  unsigned long *v2; // rax
  
  v1 = sub_e4e0(*a2,a2[1],&a2[2],0xffffffffffffffff,0x10);
  *a2 = v1;
  v2 = (unsigned long *)(v1 + a2[1] * 0x10);
  a2[1] = a2[1] + 1;
  *v2 = a0;
  v2[1] = a1;
}

// Function: sub_e710 @ 0xe710
void sub_e710(int a0,unsigned long a1)
{
  int v1; // eax
  int v2;
  int v3; // ebx
  
  v1 = toupper(a0);
  v2 = 0;
  do {
    while (v1 != toupper(v2)) {
      v2 += 1;
      if (v2 == 0x100)
        return;
    }
    v3 = v2 + 1;
    sub_dfe0(v2,a1);
    v2 = v3;
  } while (v3 != 0x100);
}

// Function: sub_e760 @ 0xe760
long sub_e760(long a0) // early-return, ternary
{
  long v1;
  char *v10;
  unsigned char v11; // dl
  unsigned long v12;
  void *v13;
  char v14 [32];
  int v15 [36]; // stack - 0xf8
  char v16 [40]; // stack - 0x68
  int v17;
  unsigned long v18;
  int v19;
  char v2;
  int v20;
  long v21;
  unsigned char v22; // r12b
  int v23; // stack - 0x134
  char v24; // stack - 0x12d
  unsigned int v25; // stack - 0x11c
  unsigned char v3;
  unsigned int v4; // eax
  unsigned int v5; // eax
  int v6; // eax
  unsigned long v7;
  unsigned long v8;
  long v9; // rax
  
  *(unsigned long *)(a0 + 0x68) = 0;
  sub_e250(v14);
  v4 = sub_e350(a0);
  v24 = '\x01';
  v25 = v4;
  if (v4 == 0x5e) {
    v4 = sub_e350(a0);
    v24 = *(char *)(a0 + 0x419);
  }
  v23 = *(int *)(a0 + 0x48);
  v22 = v4 == 0x3a;
  v18 = (unsigned long)v4;
  do {
    v22 &= 0xfd;
    v4 = (unsigned int)v18;
    if (v4 != 0x5b) { // branch-flip
      if ((v4 != 0x5c) || (!(*(unsigned char *)(a0 + 0x1c0) & 1))) {
label_e7ee:
        v5 = sub_e350(a0);
        v19 = *(int *)(a0 + 0x48);
        if (v5 == 0x2d) goto label_e802;
label_e97a:
        v18 = (unsigned long)v4;
        v12 = (unsigned long)v5;
        v17 = v23;
label_e87d:
        v23 = v19;
        v3 = ((int)v18 != 0x3a) * '\x02' + 2;
        v19 = v23;
        goto label_e88a;
      }
      v4 = sub_e350(a0);
      v18 = (unsigned long)v4;
      v23 = *(int *)(a0 + 0x48);
      v5 = sub_e350(a0);
      v19 = *(int *)(a0 + 0x48);
      if (v5 != 0x2d) goto label_e97a;
label_e802:
      v6 = sub_e350(a0);
      v20 = *(int *)(a0 + 0x48);
      v4 = (unsigned int)v18;
      v17 = v23;
      if (v6 != 0x5b) { // branch-flip
        if (v6 == 0x5d) {
          v10 = *(char **)(a0 + 0x20);
label_ec2c:
          *(long *)(a0 + 0x28) = *(long *)(a0 + 0x28) + 1;
          *(char **)(a0 + 0x20) = &v10[-1];
          v12 = 0x2d;
          goto label_e87d;
        }
        if ((v6 == 0x5c) && (*(unsigned char *)(a0 + 0x1c0) & 1)) {
          v6 = sub_e350(a0);
          v20 = *(int *)(a0 + 0x48);
        }
      }
      else {
        v10 = *(char **)(a0 + 0x20);
        if (*v10 == '.') {
          v24 = '\0';
          goto label_ec2c;
        }
      }
      v22 |= 8;
      v5 = sub_e350(a0);
      v19 = *(int *)(a0 + 0x48);
      if ((v20 == v23) && (v23 != -1)) {
        v12 = (unsigned long)v5;
        goto label_e87d;
      }
      v23 = v19;
      if (*(char *)(a0 + 0x419)) { // branch-flip
label_ed04:
        v21 = (long)(int)v4 * 2;
        while (v12 = v18, (int)v4 <= v6) {
          while ((*(char *)(a0 + 0x1cd) && (*(unsigned short *)(*(long *)__ctype_b_loc() + v21) & 0x400))) {
            v4 = (int)v12 + 1;
            v21 += 2;
            sub_e710(v12,v14);
            v12 = (unsigned long)v4;
            if (v6 < (int)v4) goto label_ed5f;
          }
          v4 = (int)v12 + 1;
          v18 = (unsigned long)v4;
          v21 += 2;
          sub_dfe0(v12,v14);
        }
label_ed5f:
        v12 = (unsigned long)v5;
      }
      else {
        sub_df20((int)(char)v18);
        if (v11 & sub_df20((int)(char)v6)) goto label_ed04;
        v12 = (unsigned long)v5;
        v24 = '\0';
      }
    }
    else {
      v5 = sub_e350(a0);
      v19 = *(int *)(a0 + 0x48);
      v17 = v23;
      if (v5 != 0x3a) { // branch-flip
        if ((v5 == 0x2e) || (v5 == 0x3d)) {
label_eb10:
          v19 = 0;
          do {
            v7 = sub_e350(a0);
            v21 = *(long *)(a0 + 0x28);
            v2 = v16[0];
label_eb25:
            v16[0] = v2;
            if ((!v21) || ((v5 == (unsigned int)v7 && (**(char **)(a0 + 0x20) == ']')))) {
              v16[v19] = '\0';
              sub_e350(a0);
              if (v5 != 0x3a) { // branch-flip
label_ebab:
                v24 = '\0';
              }
              else {
                v10 = v16;
                if (*(char *)(a0 + 0x1cd)) {
                  v10 = "alpha";
                  if (!sub_e440(v16,"upper")) {
                    v10 = "alpha";
                    if (!sub_e440(v16,"lower"))
                      v10 = v16;
                  }
                }
                v21 = sub_e460(v10);
                if (!v21)
                  sub_6630(dcgettext(NULL,"invalid character class",5)); // no-return
                if ((*(char *)(a0 + 0x418)) && (!*(char *)(v21 + 0x10))) goto label_ebab;
                v19 = 0;
                do {
                  v13 = *(void **)(v21 + 8);
                  if ((*v13)(v19))
                    sub_dfe0(v19,v14);
                  v19 += 1;
                } while (v19 != 0x100);
              }
              v22 |= 8;
              v12 = (unsigned long)(unsigned int)sub_e350(a0);
              v23 = *(int *)(a0 + 0x48);
              goto label_e8a8;
            }
            if (v19 <= 0x1f) goto label_eb39;
            v16[0] = '\0';
          } while( true );
        }
        if (v5 == 0x100) goto label_e7ee;
        if (v5 == 0x2d) goto label_e802;
        v12 = (unsigned long)v5;
        v18 = 0x5b;
        v3 = 4;
      }
      else {
        if (*(unsigned char *)(a0 + 0x1c0) & 4) goto label_eb10;
        v18 = 0x5b;
        v12 = 0x3a;
        v3 = 4;
      }
label_e88a:
      v23 = v19;
      v22 |= v3;
      if (*(char *)(a0 + 0x418)) { // branch-flip
        if (v17 != -1) { // branch-flip
          if (*(char *)(a0 + 0x1cd)) { // branch-flip
            v19 = sub_146b0(v17,&v15[1]) + 1;
            v15[0] = v17;
            if (v19 <= 0) goto label_e8a8;
          }
          else {
            v19 = 1;
          }
          v21 = 0;
          v15[0] = v17;
          while( true ) {
            if (!sub_e4c0(v17,v14)) {
              v9 = sub_e4e0(*(unsigned long *)(a0 + 0x60),*(unsigned long *)(a0 + 0x68),a0 + 0x70,0xffffffffffffffff,4);
              v1 = *(long *)(a0 + 0x68);
              *(long *)(a0 + 0x60) = v9;
              *(long *)(a0 + 0x68) = v1 + 1;
              *(int *)(v9 + v1 * 4) = v15[v21];
            }
            if (v19 <= (int)v21 + 1) break;
            v21 += 1;
            v17 = v15[v21];
          }
        }
        else {
          v24 = '\0';
        }
      }
      else {
        v8 = v18;
        if (*(char *)(a0 + 0x1cd)) {
          v8 = (unsigned long)(int)v18;
          if (*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + v8 * 2) & 4) {
            sub_e710(v18,v14);
            goto label_e8a8;
          }
        }
        sub_dfe0(v8 & 0xffffffff,v14);
      }
    }
label_e8a8:
    v18 = v12;
    if ((int)v12 == 0x5d) {
      if (v22 == 7) {
        v13 = (*(unsigned char *)(a0 + 0x1c8) & 4) ? sub_6630 : sub_6660; // branch-flip
        (*v13)(dcgettext(NULL,"character class syntax is [[:space:]], not [:space:]",5));
      }
      if (!v24)
        return 0x112;
      if (*(char *)(a0 + 0x418)) { // branch-flip
        if ((v25 == 0x5e) || (*(long *)(a0 + 0x68))) {
          *(bool *)(a0 + 0x58) = v25 == 0x5e;
          v7 = 0xffffffffffffffff;
          if (!sub_e080(v14))
            v7 = sub_e500(a0,v14);
          *(unsigned long *)(a0 + 0x50) = v7;
          return 0x113;
        }
      }
      else if ((v25 == 0x5e) && (sub_e040(v14), *(unsigned char *)(a0 + 0x1c1) & 1))
        sub_e000(10,v14);
      return sub_e500(a0,v14) + 0x114;
    }
  } while( true );
label_eb39:
  v21 = (long)v19;
  v19 += 1;
  v16[v21] = (char)v7;
  v7 = sub_e350(a0);
  v21 = *(long *)(a0 + 0x28);
  v2 = v16[0];
  goto label_eb25;
}

// Function: sub_ee50 @ 0xee50
void sub_ee50(long a0,unsigned long a1,char a2) // return-dupe x2
{
  long v1;
  long v2;
  unsigned long v3; // rax
  
  v1 = *(long *)(a0 + 0x98);
  if (*(long *)(a0 + 0xa0) != v1) { // branch-flip
label_ee71:
    if (*(char *)(a0 + 0x418)) {
      *(char *)(*(long *)(a0 + 0x118) + v1) = a2;
      v2 = *(long *)(a0 + 0x90);
      goto label_ee90;
    }
    v2 = *(long *)(a0 + 0x90);
  }
  else {
    v2 = sub_1d2b0(*(unsigned long *)(a0 + 0x90),a0 + 0xa0,1,0xffffffffffffffff,8);
    *(long *)(a0 + 0x90) = v2;
    if (*(char *)(a0 + 0x418)) {
      v3 = sub_1d1a0(*(unsigned long *)(a0 + 0x118),*(unsigned long *)(a0 + 0xa0),1);
      v1 = *(long *)(a0 + 0x98);
      *(unsigned long *)(a0 + 0x118) = v3;
      goto label_ee71;
    }
label_ee90:
    v1 = *(long *)(a0 + 0x98);
  }
  *(long *)(a0 + 0x98) = v1 + 1;
  *(unsigned long *)(v2 + v1 * 8) = a1;
  switch(a1) {
    case 0x100:
      *(char *)(a0 + 0xc1) = 1;
      goto label_eed7;
    case 0x101:
    case 0x102:
    case 0x103:
      return;
    case 0x105:
    case 0x106:
      *(long *)(a0 + 0x88) = *(long *)(a0 + 0x88) + -1;
      return;
    case 0x10c:
    case 0x10d:
    case 0x10e:
    case 0x10f:
    case 0x110:
    case 0x111:
      *(char *)(a0 + 0xc1) = 1;
      break;
    case 0x112:
      *(char *)(a0 + 0xc0) = 0;
    
  }
  *(long *)(a0 + 0xb0) = *(long *)(a0 + 0xb0) + 1;
label_eed7:
  v1 = *(long *)(a0 + 0x88) + 1;
  *(long *)(a0 + 0x88) = v1;
  if (*(long *)(a0 + 0xa8) < v1) {
    *(long *)(a0 + 0xa8) = v1;
    return;
  }
}

// Function: sub_efb0 @ 0xefb0
void sub_efb0(long a0,long a1,long a2) // return-dupe x2
{
  unsigned long *v1;
  char *v2;
  long v3;
  
  if (*(char *)(a0 + 0x418)) {
    if (a2 <= 0)
      return;
    a2 += a1;
    do {
      v2 = (char *)(*(long *)(a0 + 0x118) + a1);
      v3 = a1 * 8;
      a1 += 1;
      sub_ee50(a0,*(unsigned long *)(*(long *)(a0 + 0x90) + v3),(int)*v2);
    } while (a2 != a1);
    return;
  }
  a1 *= 8;
  v3 = 0;
  if (a2 <= 0)
    return;
  do {
    v3 += 1;
    v1 = (unsigned long *)(*(long *)(a0 + 0x90) + a1);
    a1 += 8;
    sub_ee50(a0,*v1,3);
  } while (a2 != v3);
}

// Function: sub_f060 @ 0xf060
void sub_f060(long a0,long a1) // return-dupe
{
  long v1;
  long v2; // rbx
  
  while( true ) {
    if ((!*(char *)(a0 + 0x418)) || (a1 != 0x113)) {
      sub_ee50(a0,a1,3); // return-dupe, tail-call
      return;
    }
    if (1 <= *(long *)(a0 + 0x68)) break;
    *(unsigned long *)(a0 + 0x68) = 0;
    if (*(long *)(a0 + 0x50) == -1)
      return;
    a1 = *(long *)(a0 + 0x50) + 0x114;
  }
  v2 = 1;
  sub_f140(a0,**(unsigned int **)(a0 + 0x60));
  if (2 <= *(long *)(a0 + 0x68)) {
    do {
      v1 = v2 * 4;
      v2 += 1;
      sub_f140(a0,*(unsigned int *)(*(long *)(a0 + 0x60) + v1));
      sub_f060(a0,0x106);
    } while (v2 < *(long *)(a0 + 0x68));
  }
  *(unsigned long *)(a0 + 0x68) = 0;
  if (*(long *)(a0 + 0x50) == -1)
    return;
  sub_f060(a0,*(long *)(a0 + 0x50) + 0x114);
  a1 = 0x106;
  sub_ee50(a0,a1,3);
  return;
}

// Function: sub_f140 @ 0xf140
void sub_f140(unsigned long a0,unsigned long a1) // return-dupe x3
{
  char *v1;
  int v2; // eax
  long v3; // rax
  int v4; // ebx
  char v5 [24]; // stack - 0x48
  unsigned long v6; // stack - 0x50
  
  v6 = 0;
  v3 = wcrtomb(v5,a1,&v6);
  if (v3 == -1) {
    v5[0] = 0;
    sub_ee50(a0,0,3);
    return;
  }
  v2 = (int)v3;
  if (v2 != 1) {
    sub_ee50(a0,v5[0],1);
    if (v2 <= 1)
      return;
    v3 = 1;
    do {
      v4 = (int)v3;
      v1 = &v5[v3];
      v3 += 1;
      sub_ee50(a0,*v1,(v2 + -1 == v4) * '\x02');
      sub_f060(a0,0x105);
    } while ((unsigned long)(unsigned int)(v2 - 2) + 2 != v3);
    return;
  }
  sub_ee50(a0,v5[0],3);
}

// Function: sub_f250 @ 0xf250
void sub_f250(long a0)
{
  unsigned long v1; // stack - 0x48
  int v2; // ebp
  long v3;
  unsigned long v4;
  unsigned long v5; // stack - 0x40
  unsigned long v6; // stack - 0x38
  unsigned long v7; // stack - 0x30
  
  v3 = *(long *)(a0 + 200);
  if (!v3) {
    v4 = *(unsigned long *)(a0 + 0x1c0);
    v1 = 0xffffffffffffffff;
    v5 = 0xffffffffffffffff;
    v6 = 0;
    v7 = 0;
    if (!(v4 & 0x40))
      sub_e000(10,&v1);
    if (v4 & 0x80)
      sub_e000(0);
    v3 = 1;
    *(long *)(a0 + 200) = sub_e500(a0) + 0x114;
    do {
      *(long *)(a0 + 200 + v3 * 8) = sub_e500(a0,v3 * 0x20 + 0x20f60) + 0x114;
      v3 += 1;
    } while (v3 != 9);
    v3 = *(long *)(a0 + 200);
  }
  v2 = 3;
  sub_f060(a0,v3);
  sub_f060(a0,*(unsigned long *)(a0 + 0xd0));
  sub_f060(a0,0xe0);
  sub_f060(a0,*(unsigned long *)(a0 + 0xe0));
  sub_f060(a0,0x105);
  sub_f060(a0,0x106);
  sub_f060(a0,0xed);
  sub_f060(a0,*(unsigned long *)(a0 + 0xf0));
  sub_f060(a0,0x105);
  sub_f060(a0,0x106);
  sub_f060(a0,*(unsigned long *)(a0 + 0xe8));
  sub_f060(a0,0xf0);
  sub_f060(a0,*(unsigned long *)(a0 + 0xf8));
  sub_f060(a0,0x105);
  sub_f060(a0,0x106);
  sub_f060(a0,0xf4);
  sub_f060(a0,*(unsigned long *)(a0 + 0x108));
  sub_f060(a0,0x105);
  sub_f060(a0,0x106);
  sub_f060(a0,*(unsigned long *)(a0 + 0x100));
  do {
    sub_f060(a0,*(unsigned long *)(a0 + 0xd8));
    sub_f060(a0,0x105);
    sub_f060(a0,0x106);
    v2 -= 1;
  } while (v2);
}

// Function: sub_f4a0 @ 0xf4a0
void sub_f4a0(long a0) // return-dupe x2
{
  char v1;
  long v2;
  unsigned long *v3; // rax
  unsigned long v4;
  long v5;
  long v6; // stack - 0x38
  long v7;
  long v8;
  
  v5 = *(long *)(a0 + 0x160);
  v6 = 0;
  if (*(long *)(a0 + 0x130) <= v5)
    return;
  v7 = *(long *)(a0 + 0x170);
  if (v7) {
    v7 -= 0x10;
    v6 = v5 + 2;
  }
  v3 = (unsigned long *)sub_1d2b0(v7,&v6,*(long *)(a0 + 0x130) - v5,0xffffffffffffffff,8);
  v2 = v6;
  v4 = *(unsigned long *)(a0 + 0x178);
  v3[1] = 0;
  v7 = v6 + -2;
  *v3 = 0;
  *(unsigned long **)(a0 + 0x170) = &v3[2];
  *(long *)(a0 + 0x160) = v7;
  v4 = sub_1d1a0(v4,v7,8);
  *(unsigned long *)(a0 + 0x178) = v4;
  v4 = sub_1d1a0(*(unsigned long *)(a0 + 0x180),v7,1);
  *(unsigned long *)(a0 + 0x180) = v4;
  v4 = sub_1d1a0(*(unsigned long *)(a0 + 0x188),v7,8);
  *(unsigned long *)(a0 + 0x188) = v4;
  if (*(char *)(a0 + 0x418)) {
    v8 = *(long *)(a0 + 0x1b0);
    if (v8)
      v8 -= 0x10;
    v3 = (unsigned long *)sub_1d1a0(v8,v6,8);
    if (!v5) {
      v3[1] = 0;
      *v3 = 0;
    }
    *(unsigned long **)(a0 + 0x1b0) = &v3[2];
  }
  if (v7 <= v5)
    return;
  v7 = *(long *)(a0 + 0x170);
  v8 = *(long *)(a0 + 0x178);
  v5 *= 8;
  v1 = *(char *)(a0 + 0x418);
  do {
    *(unsigned long *)(v7 + v5) = 0;
    *(unsigned long *)(v8 + v5) = 0;
    if (v1)
      *(unsigned long *)(*(long *)(a0 + 0x1b0) + v5) = 0;
    v5 += 8;
  } while (v2 * 8 + -0x10 != v5);
}

// Function: sub_f650 @ 0xf650
void sub_f650(unsigned long *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = sub_1d210(a1,0x10);
  a0[2] = a1;
  *a0 = v1;
  a0[1] = 0;
}

// Function: sub_f680 @ 0xf680
void sub_f680(long *a0,long *a1,unsigned int a2,long *a3)
{
  unsigned int v1;
  long v2;
  long *v3; // rax
  long v4; // rcx
  long v5;
  long *v6; // rax
  long v7;
  unsigned int v8;
  
  if (a3[2] - a0[1] < a1[1]) {
    free((void *)*a3);
    v5 = a1[1];
    a3[2] = a0[1];
    *a3 = sub_1d2b0(0,&a3[2],v5,0xffffffffffffffff,0x10);
  }
  a3[1] = 0;
  v5 = a1[1];
  v7 = 0;
  v4 = 0;
label_f6c0:
  v2 = a0[1];
  do {
    if (v2 <= v4) { // branch-flip
      if (v5 <= v7)
        return;
      v3 = (long *)(v7 * 0x10 + *a1);
    }
    else {
      v8 = 0;
      v6 = (long *)(v4 * 0x10 + *a0);
      v2 = *v6;
      if (v5 <= v7) goto label_f729;
      v3 = (long *)(v7 * 0x10 + *a1);
      if (v2 <= *v3) goto label_f774;
    }
    v8 = *(unsigned int *)&v3[1];
    if (a2 & v8) {
      v5 = a3[1];
      v6 = (long *)(v5 * 0x10 + *a3);
      *v6 = *v3;
      a3[1] = v5 + 1;
      *(unsigned int *)&v6[1] = a2 & v8;
      v5 = a1[1];
    }
    v7 += 1;
    v2 = a0[1];
  } while( true );
label_f774:
  if (*v3 == v2) {
    v7 += 1;
    v8 = *(unsigned int *)&v3[1] & a2;
  }
label_f729:
  v5 = a3[1];
  v1 = *(unsigned int *)&v6[1];
  v4 += 1;
  v6 = (long *)(v5 * 0x10 + *a3);
  *v6 = v2;
  a3[1] = v5 + 1;
  v5 = a1[1];
  *(unsigned int *)&v6[1] = v1 | v8;
  goto label_f6c0;
}

// Function: sub_f7d0 @ 0xf7d0
void sub_f7d0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_f680(a0,a1,0xffffffff,a2); // tail-call
}

// Function: sub_f7e0 @ 0xf7e0
void sub_f7e0(long a0)
{
  long v1;
  long v2; // rbx
  long v3;
  
  free(*(void **)(a0 + 0x118));
  free(*(void **)(a0 + 0x60));
  free(*(void **)(a0 + 0x198));
  v3 = *(long *)(a0 + 0x1b0);
  if (v3) {
    if (0 <= *(long *)(a0 + 0x160)) {
      v2 = -1;
      do {
        v1 = v2 * 8;
        v2 += 1;
        free(*(void **)(v3 + v1));
        v3 = *(long *)(a0 + 0x1b0);
      } while (v2 < *(long *)(a0 + 0x160));
    }
    free((void *)(v3 + -0x10)); // tail-call
    return;
  }
}

// Function: sub_f870 @ 0xf870
void sub_f870(unsigned long *a0)
{
  unsigned long *v1;
  void *v2;
  
  v2 = (void *)*a0;
  while (v2) {
    free(v2);
    v1 = &a0[1];
    a0 = &a0[1];
    v2 = (void *)*v1;
  }
}

// Function: sub_f8a0 @ 0xf8a0
void sub_f8a0(unsigned long *a0)
{
  sub_f870(*a0);
  *(unsigned long *)*a0 = 0;
  *(char *)a0[3] = 0;
  *(char *)a0[2] = 0;
  *(char *)a0[1] = 0;
  *(unsigned short *)&a0[4] = 0;
}

// Function: sub_f8e0 @ 0xf8e0
void sub_f8e0(unsigned long *a0)
{
  sub_f870(*a0);
  free((void *)*a0);
  free((void *)a0[1]);
  free((void *)a0[2]);
  free((void *)a0[3]);
  free(a0); // tail-call
}

// Function: sub_f920 @ 0xf920
void sub_f920(unsigned long *a0,unsigned long *a1)
{
  long v1;
  unsigned long v2; // rax
  
  v1 = a0[1];
  if (v1 <= (long)a1[2]) // branch-flip
    a1[1] = v1;
  else {
    free((void *)*a1);
    v2 = sub_1d2b0(0,&a1[2],a0[1] - a1[2],0xffffffffffffffff,0x10);
    v1 = a0[1];
    *a1 = v2;
    a1[1] = v1;
  }
  if (!v1)
    return;
  memcpy((void *)*a1,(void *)*a0,v1 << 4); // tail-call
}

// Function: sub_f9a0 @ 0xf9a0
long sub_f9a0(long a0,long *a1,unsigned int a2)
{
  unsigned long *v1;
  unsigned long v10; // rbx
  unsigned long v11; // rsi
  unsigned int v12;
  long v13; // r9
  unsigned int v14;
  long *v15;
  long v16; // r12
  char v17; // sil
  unsigned short v18;
  long v19; // stack - 0x40
  unsigned long v2;
  long v3;
  char v4; // al
  long *v5;
  long *v6; // rax
  long v7;
  unsigned long v8; // rcx
  unsigned long *v9;
  
  v11 = (unsigned long)a2;
  v17 = (char)a2;
  v1 = *(unsigned long **)(a0 + 0x128);
  v2 = a1[1];
  v3 = *(long *)(a0 + 0x130);
  if (1 <= (long)v2) { // branch-flip
    v15 = (long *)*a1;
    v10 = 0;
    v5 = v15;
    do {
      v6 = &v5[2];
      v10 ^= (unsigned long)*(unsigned int *)&v5[1] + *v5;
      v5 = v6;
    } while (&v15[v2 * 2] != v6);
    if (1 <= v3) goto label_fa0c;
    v19 = 0;
    v16 = 0;
  }
  else {
    v10 = 0;
    if (v3 <= 0) {
      v19 = 0;
      v18 = 0;
      v16 = 0;
      goto label_fa78;
    }
label_fa0c:
    v16 = 0;
    v9 = v1;
    do {
      if (((*v9 == v10) && (v9[2] == v2)) && ((unsigned char)v9[4] == a2)) {
        if (1 <= (long)v2) { // branch-flip
          v8 = 0;
          while ((v7 = v8 * 0x10, *(int *)(*a1 + 8 + v7) == *(int *)(v9[1] + 8 + v7) && (*(long *)(*a1 + v7) == *(long *)(v9[1] + v7)))) {
            v8 += 1;
            if (v8 == v2)
              return v16;
          }
        }
        else {
          v8 = 0;
        }
        if (v2 == v8)
          return v16;
      }
      v16 += 1;
      v9 = &v9[9];
    } while (v16 < v3);
    v19 = v16 * 0x48;
    if ((long)v2 <= 0) {
      v18 = 0;
      goto label_fa78;
    }
    v15 = (long *)*a1;
  }
  v7 = *(long *)(a0 + 0x150);
  v13 = 0;
  v14 = 0;
  do {
    while( true ) {
      v17 = (char)v11;
      v12 = *(unsigned int *)(v7 + v15[v13 * 2] * 4);
      if (v12) break;
      if (*(long *)(*(long *)(a0 + 0x90) + v15[v13 * 2] * 8) == 0x112)
        v14 = 0x1ff;
      v18 = (unsigned short)v14;
      v13 += 1;
      if ((long)v2 <= v13) goto label_fa78;
    }
    v4 = sub_df70();
    v17 = (char)v11;
    v12 |= v14;
    if (v4)
      v14 = v12;
    v18 = (unsigned short)v14;
    v13 += 1;
  } while (v13 < (long)v2);
label_fa78:
  v7 = sub_e4e0(v1,v3,a0 + 0x138,0xffffffffffffffff,0x48);
  v3 = a1[1];
  *(long *)(a0 + 0x128) = v7;
  *(unsigned long *)(v7 + v19) = v10;
  sub_f650(&((unsigned long *)(v7 + v19))[1],v3);
  sub_f920(a1,*(long *)(a0 + 0x128) + v19 + 8);
  v19 = *(long *)(a0 + 0x128) + v19;
  *(char *)(v19 + 0x20) = v17;
  *(unsigned short *)(v19 + 0x22) = v18;
  *(unsigned long *)(v19 + 0x30) = 0;
  *(unsigned long *)(v19 + 0x28) = 0;
  *(unsigned long *)(v19 + 0x40) = 0xffffffffffffffff;
  *(long *)(a0 + 0x130) = *(long *)(a0 + 0x130) + 1;
  return v16;
}

// Function: sub_fc10 @ 0xfc10
void sub_fc10(unsigned long a0,long *a1,unsigned long a2) // return-dupe
{
  unsigned long *v1; // rax
  long v2;
  long v3; // rbx
  
  if (4 <= a1[1]) {
    sub_f7d0(a1,a0);
    sub_f920(a2,a0); // tail-call
    return;
  }
  v2 = 0;
  if (a1[1] <= 0)
    return;
  do {
    v3 = v2 + 1;
    v1 = (unsigned long *)(v2 * 0x10 + *a1);
    sub_e5c0(*v1,v1[1],a0);
    v2 = v3;
  } while (v3 < a1[1]);
}

// Function: sub_fca0 @ 0xfca0
void sub_fca0(long a0,long a1,long a2,unsigned long a3)
{
  long *v1;
  long *v10;
  long v11; // rbx
  long *v12;
  long v13;
  long v14;
  long v15; // r15
  unsigned char *v2;
  unsigned char v3;
  unsigned int v4;
  long v5;
  long v6;
  long v7;
  long *v8;
  unsigned int *v9; // rax
  
  v5 = *(long *)(a0 + 0x140);
  v1 = (long *)(v5 + a1 * 0x18);
  if (1 <= v1[1]) { // branch-flip
    v11 = 0;
    v13 = 0;
    do {
      while( true ) {
        v10 = (long *)*v1;
        v8 = &v10[v11 * 2];
        v4 = *(unsigned int *)&v8[1];
        if (v4) break;
label_fd2f:
        v11 += 1;
        if (v11 >= v1[1]) goto label_fd83;
      }
      v6 = *v8;
      v14 = *(long *)(*(long *)(a0 + 0x90) + v6 * 8);
      if (v14 > -1) {
        v2 = (unsigned char *)(a2 + v6);
        if (((a1 != v6) && (v3 = *v2, !(v3 & 6))) && (v13)) {
          v15 = 0;
          v12 = v10;
          do {
            v7 = *v12;
            if (((a1 != v7) && (*(unsigned int *)&v12[1] == v4)) && ((!(*(unsigned char *)(a2 + v7) & 6) && ((v14 == *(long *)(*(long *)(a0 + 0x90) + v7 * 8) && (!((*(unsigned char *)(a2 + v7) ^ v3) & 1))))))) {
              v14 = v5 + v6 * 0x18;
              if (v3 & 1)
                sub_e0a0(v6,v14);
              sub_fc10(v5 + v7 * 0x18,v14,a3);
              if (v15 < v13) goto label_fd2f;
              v10 = (long *)*v1;
              v8 = &v10[v11 * 2];
              break;
            }
            v15 += 1;
            v12 = &v12[2];
          } while (v15 != v13);
        }
        v6 = v8[1];
        v14 = v13 + 1;
        v10[v13 * 2] = *v8;
        (&v10[v13 * 2])[1] = v6;
        *v2 = *v2 | 0x10;
        v13 = v14;
        goto label_fd2f;
      }
      v9 = (unsigned int *)(a1 * 4 + *(long *)(a0 + 0x150));
      v11 += 1;
      *v9 = *v9 | v4;
    } while (v1[1] > v11);
  }
  else {
    v13 = 0;
  }
label_fd83:
  v1[1] = v13;
  return;
}

// Function: sub_fe80 @ 0xfe80
void sub_fe80(unsigned long a0,unsigned long a1,unsigned long a2,unsigned int a3,unsigned long a4)
{
  a3 &= sub_e0a0(a1,a0);
  if (!a3)
    return;
  sub_f920(a0,a4);
  sub_f680(a4,a2,a3,a0); // tail-call
}

// Function: sub_fee0 @ 0xfee0
void sub_fee0(long a0,long *a1)
{
  long v1;
  long *v2;
  long v3;
  void *v4 [3]; // stack - 0x58
  long *v5;
  long v6; // r15
  unsigned int v7; // stack - 0x74
  
  v6 = 0;
  sub_f650(v4,*(unsigned long *)(a0 + 0xb0));
  v5 = a1;
  if (1 <= *(long *)(a0 + 0x98)) {
    do {
      v3 = *(long *)(a0 + 0x140);
      v2 = (long *)((long)v5 + (v3 - (long)a1));
      if (1 <= v2[1]) {
        switch(*(unsigned long *)(*(long *)(a0 + 0x90) + v6 * 8)) {
          case 0x100:
            v7 = 0x1ff;
            break;
          default:
            goto label_10051;
          case 0x10c:
            v7 = 0x124;
            break;
          case 0x10d:
            v7 = 0x1c0;
            break;
          case 0x10e:
            v7 = 0x28;
            break;
          case 0x10f:
            v7 = 0x82;
            break;
          case 0x110:
            v7 = 0xaa;
            break;
          case 0x111:
            v7 = 0x155;
          
        }
        sub_e0a0(v6,v2);
        if (1 <= v5[1]) {
          v1 = 0;
          while( true ) {
            sub_fe80(v3 + *(long *)(v1 * 0x10 + *v5) * 0x18,v6,v2,v7,v4);
            if (v5[1] <= v1 + 1) break;
            v3 = *(long *)(a0 + 0x140);
            v2 = (long *)((long)v5 + (v3 - (long)a1));
            v1 += 1;
          }
          v2 = (long *)((long)v5 + (*(long *)(a0 + 0x140) - (long)a1));
        }
        if (1 <= v2[1]) {
          v3 = 0;
          do {
            v1 = v3 * 0x10;
            v3 += 1;
            sub_fe80(&a1[*(long *)(v1 + *v2) * 3],v6,v5,0x1ff,v4);
            v2 = (long *)((long)v5 + (*(long *)(a0 + 0x140) - (long)a1));
          } while (v3 < v2[1]);
        }
      }
label_10051:
      v6 += 1;
      v5 = &v5[3];
    } while (v6 < *(long *)(a0 + 0x98));
  }
  free(v4[0]);
}

// Function: sub_100f0 @ 0x100f0
void sub_100f0(long a0)
{
  unsigned long *v1;
  void *v10; // rax
  long v11;
  long v12;
  unsigned long *v13; // rax
  long v14; // rax
  long v15; // r14
  unsigned int v2;
  long *v3;
  unsigned long v4;
  unsigned long *v5; // rax
  void *v6; // rax
  void *v7; // rax
  void *v8; // rax
  long *v9;
  
  v5 = (unsigned long *)sub_1d210(*(unsigned long *)(a0 + 0x98),8);
  v12 = *(long *)(a0 + 0x98);
  *v5 = 0;
  v13 = &v5[1];
  if (2 <= v12) {
    do {
      *v13 = 0xffffffffffffffff;
      v13 = &v13[1];
    } while (&v5[v12] != v13);
  }
  v10 = NULL;
  v6 = (void *)sub_1d210(*(unsigned long *)(a0 + 0xb0),8);
  v7 = (void *)sub_1d210(*(unsigned long *)(a0 + 0xb0),0x18);
  v8 = (void *)sub_1d210(*(unsigned long *)(a0 + 0xb0),4);
  if (*(char *)(a0 + 0x418))
    v10 = (void *)sub_1d210(*(unsigned long *)(a0 + 0xb0),1);
  v12 = *(long *)(a0 + 0x140);
  if (1 <= *(long *)(a0 + 0x98)) { // branch-flip
    v14 = 0;
    v15 = 1;
    do {
      v11 = v5[v14];
      v13 = (unsigned long *)(v12 + v14 * 0x18);
      if (0 <= v11) { // branch-flip
        *(unsigned long *)((long)v6 + v11 * 8) = *(unsigned long *)(*(long *)(a0 + 0x90) + v14 * 8);
        v4 = v13[1];
        v1 = (unsigned long *)((long)v7 + v11 * 0x18);
        *v1 = *v13;
        v1[1] = v4;
        v1[2] = v13[2];
        *(unsigned int *)((long)v8 + v11 * 4) = *(unsigned int *)(*(long *)(a0 + 0x150) + v14 * 4);
        if (v10)
          *(char *)((long)v10 + v11) = *(char *)(*(long *)(a0 + 0x118) + v14);
        v12 = v13[1];
        v3 = (long *)*v13;
        if (1 <= v12) {
          v11 = 0;
          v9 = v3;
          do {
            v12 = v5[*v9];
            if (v12 == -1) {
              v5[*v9] = v15;
              v15 += 1;
              v12 = v5[*v9];
            }
            *v9 = v12;
            v12 = v13[1];
            v11 += 1;
            v9 = &v9[2];
          } while (v11 < v12);
        }
        qsort(v3,v12,0x10,sub_e1a0);
        v12 = *(long *)(a0 + 0x140);
      }
      else {
        free((void *)*v13);
        v12 = *(long *)(a0 + 0x140);
        v13 = (unsigned long *)(v14 * 0x18 + v12);
        *v13 = 0;
        v13[1] = 0;
      }
      v14 += 1;
    } while (v14 < *(long *)(a0 + 0x98));
  }
  else {
    v15 = 1;
  }
  v14 = 0;
  while( true ) {
    v11 = v14 * 0x18;
    *(unsigned long *)(*(long *)(a0 + 0x90) + v14 * 8) = *(unsigned long *)((long)v6 + v14 * 8);
    v4 = ((unsigned long *)((long)v7 + v11))[1];
    v2 = *(unsigned int *)((long)v8 + v14 * 4);
    *(unsigned long *)(v12 + v11) = *(unsigned long *)((long)v7 + v11);
    ((unsigned long *)(v12 + v11))[1] = v4;
    *(unsigned long *)(v12 + 0x10 + v11) = *(unsigned long *)((long)v7 + v11 + 0x10);
    *(unsigned int *)(*(long *)(a0 + 0x150) + v14 * 4) = v2;
    if (v10)
      *(char *)(*(long *)(a0 + 0x118) + v14) = *(char *)((long)v10 + v14);
    v14 += 1;
    if (v14 == v15) break;
    v12 = *(long *)(a0 + 0x140);
  }
  *(long *)(a0 + 0xb0) = v14;
  *(long *)(a0 + 0x98) = v14;
  free(v6);
  free(v7);
  free(v8);
  free(v10);
  free(v5); // tail-call
}

// Function: sub_103f0 @ 0x103f0
void sub_103f0(long a0)
{
  long v1;
  long *v10;
  unsigned char *v2; // rax
  long v3; // rax
  unsigned long v4;
  unsigned char v5; // cl
  long v6;
  unsigned char *v7;
  void *v8 [3]; // stack - 0x48
  long v9; // rsi
  
  v2 = (unsigned char *)sub_1d3e0(*(unsigned long *)(a0 + 0x98));
  v6 = *(long *)(a0 + 0x98);
  if (1 <= v6) {
    v10 = *(long **)(a0 + 0x140);
    v9 = 0;
    do {
      v3 = 0;
      if (0 < v10[1]) {
        do {
          while( true ) {
            v1 = *(long *)(*v10 + v3 * 0x10);
            if (v1 != v9) break;
            v2[v9] = v2[v9] | 1;
            v3 += 1;
            if (v10[1] <= v3) goto label_10488;
          }
          v7 = &v2[v1];
          if (v9 <= v1) { // branch-flip
            v5 = *v7 & 8;
            *v7 = v5;
            v7 = &v2[*(long *)(*v10 + v3 * 0x10)];
            if (v5) // branch-flip
              *v7 = *v7 | 4;
            else {
              *v7 = *v7 | 8;
            }
          }
          else {
            *v7 = *v7 | 2;
          }
          v3 += 1;
        } while (v3 < v10[1]);
      }
label_10488:
      v9 += 1;
      v10 = &v10[3];
    } while (v9 != v6);
  }
  v4 = *(unsigned long *)(a0 + 0xb0);
  *v2 = *v2 | 0x10;
  v6 = 0;
  sub_f650(v8,v4);
  v4 = sub_1d3c0(*(unsigned long *)(a0 + 0x98),4);
  *(unsigned long *)(a0 + 0x150) = v4;
  if (0 < *(long *)(a0 + 0x98)) {
    do {
      while (!(v2[v6] & 0x10)) {
        v6 += 1;
        if (*(long *)(a0 + 0x98) <= v6) goto label_1050a;
      }
      sub_fca0(a0,v6,v2,v8);
      v6 += 1;
    } while (v6 < *(long *)(a0 + 0x98));
  }
label_1050a:
  sub_100f0(a0);
  free(v8[0]);
  free(v2);
  return;
}

// Function: sub_10570 @ 0x10570
unsigned long * sub_10570(unsigned long a0,unsigned long a1)
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  
  v1 = (unsigned long *)sub_1d0f0(0x30);
  *v1 = sub_1d3b0(8);
  v1[1] = sub_1d3e0(a1);
  v1[2] = sub_1d3e0(a1);
  v2 = sub_1d3e0(a1);
  v1[5] = a0;
  v1[3] = v2;
  *(unsigned short *)&v1[4] = 0;
  return v1;
}

// Function: sub_105e0 @ 0x105e0
long * sub_105e0(long *a0,char *a1)
{
  long *v1;
  long v2;
  long v3;
  char *v4;
  char *v5;
  long v6;
  long v7; // r15
  
  v5 = (char *)*a0;
  if (v5) { // branch-flip
    v2 = 0;
    v4 = v5;
    do {
      if (strstr(v4,a1)) {
        free(a1);
        return a0;
      }
      v2 += 1;
      v4 = (char *)a0[v2];
    } while (v4);
    v7 = 0;
    v1 = a0;
    do {
      while( true ) {
        if (strstr(a1,v5)) break;
        v7 += 1;
        v1 = &a0[v7];
        v5 = (char *)*v1;
        v3 = v2;
        if (!v5) goto label_106a9;
      }
      v3 = v2 + -1;
      free(v5);
      v6 = v3 * 8;
      if (v3 == v7) {
        v3 = v2 + 1;
        v2 = v6 + 8;
        goto label_106b9;
      }
      *v1 = a0[v3];
      v1 = &a0[v7];
      a0[v3] = 0;
      v5 = (char *)*v1;
      v2 = v3;
    } while (v5);
label_106a9:
    v6 = v3 * 8;
    v3 += 2;
    v2 = v6 + 8;
  }
  else {
    v2 = 8;
    v6 = 0;
    v3 = 2;
  }
label_106b9:
  v1 = (long *)sub_1d1a0(a0,v3,8);
  *(char **)((long)v1 + v6) = a1;
  *(unsigned long *)((long)v1 + v2) = 0;
  return v1;
}

// Function: sub_10710 @ 0x10710
unsigned long sub_10710(unsigned long a0,long *a1)
{
  long v1;
  
  v1 = *a1;
  if (v1) {
    do {
      a1 = &a1[1];
      a0 = sub_105e0(a0,sub_1d490(v1));
      v1 = *a1;
    } while (v1);
  }
  return a0;
}

// Function: sub_10760 @ 0x10760
char * sub_10760(char *a0,char *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  
  v1 = strlen(a1);
  if (v1) {
    v2 = strlen(a0);
    a0 = (char *)sub_1d180(a0,v1 + 1 + v2);
    memcpy(&a0[v2],a1,v1 + 1);
  }
  return a0;
}

// Function: sub_107c0 @ 0x107c0
void sub_107c0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_105e0(a0,sub_1d450(a1,a2)); // tail-call
}

// Function: sub_107e0 @ 0x107e0
unsigned long sub_107e0(char *a0,char *a1)
{
  char v1;
  char v2;
  char v3;
  unsigned long v4;
  char *v5;
  long v6;
  long v7;
  
  v4 = sub_1d3b0(8);
  v1 = *a0;
  while (v1) {
    v5 = strchr(a1,(int)v1);
    if (v5) {
      v2 = a0[1];
      v7 = 0;
      do {
        v6 = 1;
        v3 = v2;
        while ((v3 && (v5[v6] == v3))) {
          v6 += 1;
          v3 = a0[v6];
        }
        if (v7 < v6)
          v7 = v6;
        v5 = strchr(&v5[1],(int)v1);
      } while (v5);
      v4 = sub_107c0(v4,a0,v7);
    }
    v5 = &a0[1];
    a0 = &a0[1];
    v1 = *v5;
  }
  return v4;
}

// Function: sub_108a0 @ 0x108a0
unsigned long sub_108a0(long *a0,long *a1)
{
  long v1;
  unsigned long v2;
  void *v3; // rax
  long *v4;
  
  v2 = sub_1d3b0(8);
  v1 = *a0;
  while (v1) {
    v1 = *a1;
    v4 = a1;
    while (v1) {
      v4 = &v4[1];
      v3 = (void *)sub_107e0(*a0);
      v2 = sub_10710(v2,v3);
      sub_f870(v3);
      free(v3);
      v1 = *v4;
    }
    a0 = &a0[1];
    v1 = *a0;
  }
  return v2;
}

// Function: sub_10930 @ 0x10930
unsigned int sub_10930(long a0,long *a1,long a2)
{
  long *v1;
  long v2;
  unsigned int v3; // eax
  
  if (1 <= a2) {
    v3 = 0;
    v1 = &a1[a2 * 2];
    do {
      v2 = *a1;
      a1 = &a1[2];
      v3 |= *(unsigned int *)(*(long *)(a0 + 0x158) + v2 * 4);
    } while (v1 != a1);
    return v3;
  }
  return 0;
}

// Function: sub_10970 @ 0x10970
void sub_10970(long a0,char a1)
{
  unsigned char v1;
  unsigned long *v10; // rax
  long *v11; // rcx
  long v12; // rdx
  long v13;
  long v14;
  long *v15;
  long *v16; // rbx
  void *v17 [3]; // stack - 0x58
  long *v18; // stack - 0x78
  char *v19;
  unsigned int v2;
  char *v20;
  long v21;
  long v22;
  unsigned long *v23; // rdi
  unsigned int *v24;
  long v25; // r8
  unsigned char v26;
  unsigned int *v27;
  long v28; // rbp
  long *v29; // stack - 0xb8
  void *v3;
  long *v30; // stack - 0xa8
  unsigned long *v31; // stack - 0xa0
  long *v32; // stack - 0x98
  char *v33; // stack - 0x90
  long v34; // stack - 0x70
  unsigned int v4; // eax
  int v5; // eax
  unsigned long v6;
  unsigned int *v7; // rax
  long v8; // rax
  unsigned long *v9;
  
  v32 = (long *)sub_1d210(*(unsigned long *)(a0 + 0xb0),0x20);
  v29 = &v32[*(long *)(a0 + 0xb0) * 2];
  v33 = (char *)sub_1d210(*(unsigned long *)(a0 + 0xa8),0x18);
  sub_f060(a0,0x105);
  *(char *)(a0 + 0x148) = a1;
  v13 = *(long *)(a0 + 0x98);
  sub_f650(v17,*(unsigned long *)(a0 + 0xb0));
  v6 = sub_1d3c0(v13,0x18);
  *(unsigned long *)(a0 + 0x140) = v6;
  if (*(char *)(a0 + 0xc1)) { // branch-flip
    v31 = (unsigned long *)sub_1d3c0(v13,0x18);
    if (0 < v13) goto label_10a42;
    sub_fee0(a0,v31);
label_10ba6:
    free(v31);
    sub_103f0(a0);
    sub_f650(&v18,1);
    sub_e6c0(0,0x1ff,&v18);
    v7 = (unsigned int *)sub_1d3c0(v13,4);
    *(unsigned int **)(a0 + 0x158) = v7;
    if (v13 <= 0) goto label_10c80;
  }
  else {
    v31 = NULL;
    if (v13 <= 0) {
      sub_103f0(a0);
      sub_f650(&v18,1);
      sub_e6c0(0,0x1ff,&v18);
      *(unsigned long *)(a0 + 0x158) = sub_1d3c0(v13,4);
      goto label_10c80;
    }
label_10a42:
    v28 = 0;
    v30 = v32;
    v19 = v33;
    do {
      v22 = *(long *)(*(long *)(a0 + 0x90) + v28 * 8);
      v20 = v19;
      switch(v22) {
        case 0x100:
          *v19 = 1;
          v20 = &v19[0x18];
          *(unsigned long *)&v19[0x10] = 0;
          *(unsigned long *)&v19[8] = 0;
          break;
        case 0x101:
label_10d30:
          v19[-0x18] = 1;
          break;
        case 0x102:
        case 0x103:
          v15 = &v29[*(long *)&v19[-8] * -2];
          v34 = *(long *)&v19[-0x10];
          v11 = &v30[v34 * -2];
          if ((*(char *)(a0 + 0xc1)) && (v11 < v30)) {
            v18 = v15;
            v34 = *(long *)&v19[-8];
            do {
              v22 = *v11;
              v11 = &v11[2];
              sub_fc10(&v31[v22 * 3],&v18,v17);
            } while (v11 < v30);
            v34 = *(long *)&v19[-0x10];
            v15 = &v29[*(long *)&v19[-8] * -2];
            v11 = &v30[v34 * -2];
          }
          v18 = v11;
          if (v15 < v29) {
            do {
              v22 = *v15;
              v15 = &v15[2];
              sub_fc10(*(long *)(a0 + 0x140) + v22 * 0x18,&v18,v17);
            } while (v15 < v29);
          }
          if (*(long *)(*(long *)(a0 + 0x90) + v28 * 8) != 0x103) goto label_10d30;
          break;
        default:
          *(unsigned long *)&v19[0x10] = 1;
          *v19 = v22 == 0x112;
          *v29 = v28;
          v11 = &v30[2];
          v15 = &v29[2];
          *v30 = v28;
          *(unsigned long *)&v19[8] = 1;
          *(unsigned int *)&v29[1] = 0x1ff;
          *(unsigned int *)&v30[1] = 0x1ff;
          v20 = &v19[0x18];
          v29 = v15;
          v30 = v11;
          break;
        case 0x105:
          v22 = *(long *)&v19[-0x10];
          v21 = *(long *)&v19[-8];
          v14 = *(long *)&v19[-0x20];
          v8 = v21 << 4;
          v25 = v14 << 4;
          v15 = &v30[v22 * -2];
          if ((v31) && (v18 = &v29[(v21 + v14) * -2], v15 < v30)) {
            v34 = v14;
            do {
              v22 = *v15;
              v15 = &v15[2];
              sub_fc10(&v31[v22 * 3],&v18,v17);
            } while (v15 < v30);
            v22 = *(long *)&v19[-0x10];
            v21 = *(long *)&v19[-8];
            v14 = *(long *)&v19[-0x20];
            v15 = &v30[v22 * -2];
            v25 = v14 << 4;
            v8 = v21 << 4;
          }
          v11 = (long *)((long)v29 - v8);
          v16 = (long *)((long)v11 - v25);
          v18 = v15;
          v34 = v22;
          if (v16 < v11) {
            do {
              v22 = *v16;
              v16 = &v16[2];
              sub_fc10(*(long *)(a0 + 0x140) + v22 * 0x18,&v18,v17);
            } while (v16 < v11);
            v22 = *(long *)&v19[-0x10];
            v21 = *(long *)&v19[-8];
            v14 = *(long *)&v19[-0x20];
          }
          v1 = v19[-0x30];
          if (v1) { // branch-flip
            v26 = v19[-0x18];
            *(long *)&v19[-0x28] = *(long *)&v19[-0x28] + v22;
            if (!v26) goto label_10edc;
label_10e51:
            v21 += v14;
          }
          else {
            v26 = v19[-0x18];
            v30 = &v30[v22 * -2];
            if (v26) goto label_10e51;
label_10edc:
            v9 = (unsigned long *)((long)v29 - (v21 * 0x10 + v14 * 0x10));
            v23 = &v9[v21 * 2];
            if (1 <= v21) {
              do {
                v6 = (&v9[v14 * 2])[1];
                v10 = &v9[2];
                *v9 = v9[v14 * 2];
                v9[1] = v6;
                v9 = v10;
              } while (v23 != v10);
            }
            v29 = &v29[v14 * -2];
          }
          *(long *)&v19[-0x20] = v21;
          v19[-0x30] = v1 & v26;
          v20 = &v19[-0x18];
          break;
        case 0x106:
          *(long *)&v19[-0x28] = *(long *)&v19[-0x28] + *(long *)&v19[-0x10];
          *(long *)&v19[-0x20] = *(long *)&v19[-0x20] + *(long *)&v19[-8];
          v19[-0x30] = v19[-0x30] | v19[-0x18];
          v20 = &v19[-0x18];
        
      }
      v28 += 1;
      v19 = v20;
    } while (v28 < v13);
    if (v31) {
      v28 = 0;
      sub_fee0(a0,v31);
      v9 = v31;
      do {
        v3 = (void *)*v9;
        v28 += 1;
        v9 = &v9[3];
        free(v3);
      } while (v13 != v28);
      goto label_10ba6;
    }
    sub_103f0(a0);
    sub_f650(&v18,1);
    sub_e6c0(0,0x1ff,&v18);
    v7 = (unsigned int *)sub_1d3c0(v13,4);
    *(unsigned int **)(a0 + 0x158) = v7;
  }
  v24 = *(unsigned int **)(a0 + 0x150);
  v15 = *(long **)(a0 + 0x140);
  v27 = &v7[v13];
  do {
    v2 = *v24;
    if (sub_dfa0(v2))
      *v7 = *v7 | 4;
    if (sub_dfb0())
      *v7 = *v7 | 2;
    if (1 <= v15[1]) {
      v13 = *v15;
      v28 = v15[1] * 0x10 + v13;
      do {
        v2 = *(unsigned int *)(v13 + 8);
        if (sub_dfa0(v2))
          *v7 = *v7 | 4;
        if (sub_dfb0())
          *v7 = *v7 | 2;
        v13 = v12 + 0x10;
      } while (v13 != v28);
    }
    v7 = &v7[1];
    v24 = &v24[1];
    v15 = &v15[3];
  } while (v27 != v7);
label_10c80:
  v4 = sub_10930(a0,v18,v34);
  if (v4 & 4)
    sub_f9a0(a0,&v18,4);
  v5 = sub_f9a0(a0,&v18,v4 ^ 7);
  *(int *)(a0 + 0x16c) = v5;
  *(long *)(a0 + 400) = (long)v5;
  if (v4 & 2)
    v5 = sub_f9a0(a0,&v18,2);
  *(unsigned int *)(a0 + 0x168) = 0;
  *(int *)(a0 + 0x16c) = v5 + 1;
  free(v32);
  free(v33);
  free(v17[0]);
  free(v18);
}

// Function: sub_11120 @ 0x11120
long sub_11120(long a0,long a1)
{
  long v1;
  long v2;
  
  v2 = 0;
  while( true ) {
    for (; v1 = *(long *)(a0 + -8 + a1 * 8), 0x103 < v1; a1 = (a1 + -1) - v1) {
      if (2 <= (unsigned long)(v1 - 0x105U)) {
        return v2 + 1;
      }
      v1 = sub_11120(a0,a1 + -1);
      v2 = v2 + 1 + v1;
    }
    if (v1 <= 0x100) break;
    a1 -= 1;
    v2 += 1;
  }
  return v2 + 1;
}

// Function: sub_11190 @ 0x11190
void sub_11190(long a0,unsigned long a1,unsigned long a2)
{
  *(unsigned long *)(a0 + 0x20) = a1;
  *(unsigned long *)(a0 + 0x28) = a2;
}

// Function: sub_111a0 @ 0x111a0
long sub_111a0(long a0) // early-return
{
  int v1;
  int v10; // edx
  unsigned long v11; // stack - 0xd8
  char v12 [32];
  char v13 [104];
  int v14;
  unsigned long v15;
  unsigned char v16; // r12b
  bool v17;
  unsigned long v18; // stack - 0xd0
  char v2;
  unsigned int v3;
  unsigned long v4;
  long v5;
  char *v6;
  char *v7; // rax
  int v8; // edx
  int v9;
  
  v9 = 2;
  v16 = 0;
label_111d7:
  if (!*(long *)(a0 + 0x28)) {
    *(unsigned long *)(a0 + 0x30) = 0xffffffffffffffff;
    return -1;
  }
  v4 = sub_e300(a0);
  v1 = (int)v4;
  switch(v1) {
    case 10:
      if (!(*(unsigned char *)(a0 + 0x1c1) & 8)) goto label_11200;
      if (!v16) goto label_11233;
      goto label_11212;
    default:
      goto label_11200;
    case 0x24:
      if ((!v16) && (((((v15 = *(unsigned long *)(a0 + 0x1c0), v15 & 8 || (v5 = *(long *)(a0 + 0x28), !v5)) || ((v3 = (unsigned int)(v15 >> 0xd) ^ 1, (long)(unsigned long)(v3 & 1) < v5 && ((*(char **)(a0 + 0x20))[(unsigned long)(**(char **)(a0 + 0x20) == '\\') & (unsigned long)(v3 & 1)] == ')')))) || ((v3 = (unsigned int)(v15 >> 0xf) ^ 1, (long)(unsigned long)(v3 & 1) < v5 && ((*(char **)(a0 + 0x20))[(unsigned long)(**(char **)(a0 + 0x20) == '\\') & (unsigned long)(v3 & 1)] == '|')))) || ((v15 & 0x800 && ((1 <= v5 && (**(char **)(a0 + 0x20) == '\n')))))))) goto label_11233;
      goto label_11212;
    case 0x27:
      if (!v16) goto label_11212;
      if (!(*(unsigned char *)(a0 + 0x1c2) & 8)) {
label_11233:
        *(unsigned long *)(a0 + 0x30) = 0x10d;
        return 0x10d;
      }
      goto label_11205;
    case 0x28:
      if ((((unsigned char)(*(unsigned long *)(a0 + 0x1c0) >> 0xd) ^ 1) & 1) == v16) {
        *(long *)(a0 + 0x38) = *(long *)(a0 + 0x38) + 1;
        *(char *)(a0 + 0x78) = 1;
        *(unsigned long *)(a0 + 0x30) = 0x107;
        return 0x107;
      }
      goto label_11212;
    case 0x29:
      if (((((unsigned char)(*(unsigned long *)(a0 + 0x1c0) >> 0xd) ^ 1) & 1) == v16) && ((*(long *)(a0 + 0x38) || (!(*(unsigned long *)(a0 + 0x1c0) & 0x20000))))) {
        *(char *)(a0 + 0x78) = 0;
        *(long *)(a0 + 0x38) = *(long *)(a0 + 0x38) + -1;
        *(unsigned long *)(a0 + 0x30) = 0x108;
        return 0x108;
      }
      goto label_11212;
    case 0x2a:
      if (v16) goto label_11212;
      if (*(char *)(a0 + 0x78)) {
        if (!(*(unsigned char *)(a0 + 0x1c0) & 0x10)) goto label_11212;
        if (*(unsigned char *)(a0 + 0x1c8) & 0x10)
          sub_6660(dcgettext(NULL,"* at start of expression",5));
      }
      *(unsigned long *)(a0 + 0x30) = 0x102;
      return 0x102;
    case 0x2b:
      v15 = *(unsigned long *)(a0 + 0x1c0);
      if (v15 & 0x400) goto label_11200;
      if (((unsigned char)(v15 >> 1) & 1) != v16) goto label_11212;
      if (*(char *)(a0 + 0x78)) {
        if (!(v15 & 0x10)) goto label_11200;
        if (*(unsigned char *)(a0 + 0x1c8) & 0x20)
          sub_6660(dcgettext(NULL,"+ at start of expression",5));
      }
      *(unsigned long *)(a0 + 0x30) = 0x103;
      return 0x103;
    case 0x2e:
      if (!v16) {
        if (*(long *)(a0 + 0x18) < 0) {
          sub_e020(v12);
          v4 = *(unsigned long *)(a0 + 0x1c0);
          if (!(v4 & 0x40))
            sub_e000(10,v12);
          if (v4 & 0x80)
            sub_e000(0,v12);
          if (*(char *)(a0 + 0x418)) {
            v4 = 0;
            do {
              if (*(int *)(a0 + 0x51c + v4 * 4) == -1)
                sub_e000(v4 & 0xffffffff,v12);
              v4 += 1;
            } while (v4 != 0x100);
          }
          *(unsigned long *)(a0 + 0x18) = sub_e500(a0,v12);
        }
        *(char *)(a0 + 0x78) = 0;
        v5 = 0x10a;
        if (!*(char *)(a0 + 0x418))
          v5 = *(long *)(a0 + 0x18) + 0x114;
        goto label_1122f;
      }
      goto label_11212;
    case 0x31:
    case 0x32:
    case 0x33:
    case 0x34:
    case 0x35:
    case 0x36:
    case 0x37:
    case 0x38:
    case 0x39:
      if (!v16) goto label_11212;
      if (!(*(unsigned char *)(a0 + 0x1c1) & 0x40)) {
        *(char *)(a0 + 0x78) = 0;
        *(unsigned long *)(a0 + 0x30) = 0x112;
        return 0x112;
      }
      goto label_11205;
    case 0x3c:
      if (!v16) goto label_11212;
      if (!(*(unsigned char *)(a0 + 0x1c2) & 8)) {
        *(unsigned long *)(a0 + 0x30) = 0x10e;
        return 0x10e;
      }
      goto label_11205;
    case 0x3e:
      if (!v16) goto label_11212;
      if (!(*(unsigned char *)(a0 + 0x1c2) & 8)) {
        *(unsigned long *)(a0 + 0x30) = 0x10f;
        return 0x10f;
      }
      goto label_11205;
    case 0x3f:
      v15 = *(unsigned long *)(a0 + 0x1c0);
      if (v15 & 0x400) goto label_11200;
      if (((unsigned char)(v15 >> 1) & 1) != v16) goto label_11212;
      if (*(char *)(a0 + 0x78)) {
        if (!(v15 & 0x10)) goto label_11200;
        if (*(unsigned char *)(a0 + 0x1c8) & 0x20)
          sub_6660(dcgettext(NULL,"? at start of expression",5));
      }
      *(unsigned long *)(a0 + 0x30) = 0x101;
      return 0x101;
    case 0x42:
      if (!v16) goto label_11212;
      if (!(*(unsigned char *)(a0 + 0x1c2) & 8)) {
        *(unsigned long *)(a0 + 0x30) = 0x111;
        return 0x111;
      }
      goto label_11205;
    case 0x53:
    case 0x73:
      if (!v16) goto label_11212;
      v15 = (unsigned long)((unsigned int)*(unsigned long *)(a0 + 0x1c0) & 0x80000);
      if (*(unsigned long *)(a0 + 0x1c0) & 0x80000) goto label_11205;
      if (*(char *)(a0 + 0x418)) {
        v17 = v1 == 0x73;
        v6 = "^[:space:]]";
label_11233:
        sub_e410(a0,&v11,&v6[v17]);
        v5 = sub_e760(a0);
        *(long *)(a0 + 0x30) = v5;
        sub_11190(a0,v11,v18);
        *(char *)(a0 + 0x78) = 0;
        return v5;
      }
      sub_e250(v12);
      v5 = *(long *)__ctype_b_loc();
      do {
        if (*(unsigned char *)(v5 + 1 + v15 * 2) & 0x20)
          sub_dfe0(v15 & 0xffffffff,v12);
        v15 += 1;
      } while (v15 != 0x100);
      if (v1 != 0x53) goto label_11233;
      break;
    case 0x57:
    case 0x77:
      if (!v16) goto label_11212;
      v15 = (unsigned long)((unsigned int)*(unsigned long *)(a0 + 0x1c0) & 0x80000);
      if (*(unsigned long *)(a0 + 0x1c0) & 0x80000) goto label_11205;
      if (*(char *)(a0 + 0x418)) {
        v17 = v1 == 0x77;
        v6 = "^_[:alnum:]]";
        goto label_11233;
      }
      sub_e250(v12);
      do {
        if (*(char *)(a0 + 0x1cf + v15) == '\x02')
          sub_dfe0(v15 & 0xffffffff,v12);
        v15 += 1;
      } while (v15 != 0x100);
      if (v1 != 0x57) goto label_11233;
      break;
    case 0x5b:
      if (v16) goto label_11212;
      *(char *)(a0 + 0x78) = 0;
      v5 = sub_e760(a0);
      goto label_1122f;
    case 0x5c:
      if (!v16) goto label_11416;
      goto label_11212;
    case 0x5d:
    case 0x7d:
      goto label_11212;
    case 0x5e:
      if ((!v16) && (((*(unsigned char *)(a0 + 0x1c0) & 8 || ((unsigned long)(*(long *)(a0 + 0x30) - 0x106U) <= 1)) || (*(long *)(a0 + 0x30) == -1)))) goto label_11233;
      goto label_11212;
    case 0x60:
      if (!v16) goto label_11212;
      if (!(*(unsigned char *)(a0 + 0x1c2) & 8)) {
label_11233:
        *(unsigned long *)(a0 + 0x30) = 0x10c;
        return 0x10c;
      }
      goto label_11205;
    case 0x62:
      if (!v16) goto label_11212;
      if (!(*(unsigned char *)(a0 + 0x1c2) & 8)) {
        *(unsigned long *)(a0 + 0x30) = 0x110;
        return 0x110;
      }
      goto label_11205;
    case 0x7b:
      v15 = *(unsigned long *)(a0 + 0x1c0);
      if (!(v15 & 0x200)) goto label_11200;
      if ((((unsigned char)(v15 >> 0xc) ^ 1) & 1) != v16) goto label_11212;
      v6 = *(char **)(a0 + 0x20);
      v9 = -1;
      *(unsigned long *)(a0 + 0x40) = 0xffffffffffffffff;
      v7 = &v6[*(long *)(a0 + 0x28)];
      goto label_115fd;
    case 0x7c:
      if (*(unsigned long *)(a0 + 0x1c0) & 0x400) goto label_11200;
      if ((((unsigned char)(*(unsigned long *)(a0 + 0x1c0) >> 0xf) ^ 1) & 1) == v16) {
label_11233:
        *(char *)(a0 + 0x78) = 1;
        *(unsigned long *)(a0 + 0x30) = 0x106;
        return 0x106;
      }
      goto label_11212;
    
  }
  sub_e040(v12);
label_11233:
  *(char *)(a0 + 0x78) = 0;
  v5 = sub_e500(a0,v12) + 0x114;
  *(long *)(a0 + 0x30) = v5;
  return v5;
label_115fd:
  if (v6 == v7) goto label_11af4;
  v14 = (int)*v6;
  v2 = sub_df20(v14,v14,v9);
  if (!v2) {
    if ((char)v14 == ',') {
      if (v8 <= -1)
        *(unsigned int *)(a0 + 0x40) = 0;
      goto label_11b2e;
    }
    *(int *)(a0 + 0x44) = v8;
    goto label_11907;
  }
  if ((0 <= v8) && (v14 = v14 + v8 * 10, 0x8031 <= v14))
    v14 = 0x8030;
  v9 = v14 + -0x30;
  v6 = &v6[1];
  *(int *)(a0 + 0x40) = v9;
  goto label_115fd;
label_11af4:
  if (!v16) goto label_11a0c;
  goto label_11926;
label_11b2e:
  v6 = &v6[1];
  if (v7 == v6) goto label_11b75;
  v2 = *v6;
  if (!sub_df20((int)v2)) goto label_11907;
  v9 = v10;
  if ((0 <= *(int *)(a0 + 0x44)) && (v9 = v10 + *(int *)(a0 + 0x44) * 10, 0x8031 <= v9))
    v9 = 0x8030;
  *(int *)(a0 + 0x44) = v9 + -0x30;
  goto label_11b2e;
label_11416:
  if (!*(long *)(a0 + 0x28))
    sub_6630(dcgettext(NULL,"unfinished \\ escape",5)); // no-return
  v16 = 1;
  if (v9 == 1)
    abort(); // no-return
  v9 = 1;
  goto label_111d7;
label_11907:
  if (v16) { // branch-flip
    if (v6 != v7) {
      v2 = *v6;
      v6 = &v6[1];
      if (v2 != '\\') goto label_11926;
      goto label_11a0c;
    }
label_11b75:
    v6 = v7;
label_11926:
    v3 = (unsigned int)v15;
    if (v15 & 0x200000) goto label_11212;
    if (!*(char *)(a0 + 0x78)) {
      sub_6630(dcgettext(NULL,"invalid content of \\{\\}",5)); // return-dupe, no-return
    }
    v17 = 1;
  }
  else {
label_11a0c:
    v3 = (unsigned int)v15;
    if (v6 == v7) goto label_11b75;
    v2 = *v6;
    v6 = &v6[1];
    if (v2 != '}') goto label_11926;
    if ((*(int *)(a0 + 0x40) < 0) || ((v9 = *(int *)(a0 + 0x44), v9 < *(int *)(a0 + 0x40) && (0 <= v9)))) goto label_11926;
    if (!*(char *)(a0 + 0x78)) goto label_11966;
    v17 = 0;
  }
  if (v3 & 0x10) {
    if (*(unsigned char *)(a0 + 0x1c8) & 0x20)
      sub_6660(dcgettext(NULL,"{...} at start of expression",5));
    if (!v17) {
      v9 = *(int *)(a0 + 0x44);
label_11966:
      if (0x8000 > v9) {
        *(char **)(a0 + 0x20) = v6;
        *(long *)(a0 + 0x28) = (long)v7 - (long)v6;
        *(char *)(a0 + 0x78) = 0;
        *(unsigned long *)(a0 + 0x30) = 0x104;
        return 0x104;
      }
      sub_6630(dcgettext(NULL,"regular expression too big",5)); // no-return
    }
    sub_6630(dcgettext(NULL,"invalid content of \\{\\}",5));
  }
label_11200:
  if (v16) {
label_11205:
    if (*(unsigned char *)(a0 + 0x1c8) & 8) {
      v3 = *(unsigned int *)(a0 + 0x48);
      if (iswprint(v3)) { // branch-flip
        v3 = *(unsigned int *)(a0 + 0x48);
        if (iswspace(v3)) // branch-flip
          v6 = dcgettext(NULL,"stray \\ before white space",5);
        else {
          v3 = *(unsigned int *)(a0 + 0x48);
          v6 = v13;
          v7 = dcgettext(NULL,"stray \\ before %lc",5);
          if (100 <= (unsigned int)__snprintf_chk(v6,100,1,100,v7,(unsigned long)v3))
            v6 = dcgettext(NULL,"stray \\",5);
        }
      }
      else {
        v6 = dcgettext(NULL,"stray \\ before unprintable character",5);
      }
      sub_6660(v6);
    }
  }
label_11212:
  *(char *)(a0 + 0x78) = 0;
  if (*(char *)(a0 + 0x418)) {
    *(unsigned long *)(a0 + 0x30) = 0x109;
    return 0x109;
  }
  v5 = (long)v1;
  if (*(char *)(a0 + 0x1cd)) {
    v5 = (long)v1;
    if (*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + v5 * 2) & 4) {
      sub_e250(v12,v12);
      sub_e710(v4 & 0xffffffff);
      v5 = sub_e500(a0,v12) + 0x114;
      *(long *)(a0 + 0x30) = v5;
      return v5;
    }
  }
label_1122f:
  *(long *)(a0 + 0x30) = v5;
  return v5;
}

// Function: sub_11c60 @ 0x11c60
void sub_11c60(long a0) // return-dupe
{
  sub_11fc0(a0);
  if (*(long *)(a0 + 0x80) != 0x106)
    return;
  do {
    *(unsigned long *)(a0 + 0x80) = sub_111a0(a0);
    sub_11fc0(a0);
    sub_f060(a0,0x106);
  } while (*(long *)(a0 + 0x80) == 0x106);
}

// Function: sub_11cc0 @ 0x11cc0
void sub_11cc0(long a0)
{
  unsigned int v1;
  int v2; // eax
  unsigned int *v3; // rbx
  unsigned int v4 [34];
  unsigned long v5;
  
  v3 = v4;
  v5 = *(unsigned long *)(a0 + 0x80);
  if ((0x100 <= v5) && ((long)v5 <= 0x109)) {
    if (v5 != 0x109) {
      if (v5 != 0x107) {
        sub_f060(a0,0x100); // tail-call
        return;
      }
      *(unsigned long *)(a0 + 0x80) = sub_111a0(a0);
      sub_11c60(a0);
      if (*(long *)(a0 + 0x80) != 0x108)
        sub_6630(dcgettext(NULL,"unbalanced (",5)); // no-return
      goto label_11d14;
    }
    if (*(int *)(a0 + 0x48) != -1) {
      sub_f140(a0,*(int *)(a0 + 0x48));
      if ((*(char *)(a0 + 0x1cd)) && (v2 = sub_146b0(*(unsigned int *)(a0 + 0x48),v4), 1 <= v2)) {
        do {
          v1 = *v3;
          v3 = &v3[1];
          sub_f140(a0,v1);
          sub_f060(a0,0x106);
        } while (&v4[(unsigned long)(unsigned int)(v2 - 1) + 1] != v3);
      }
      goto label_11d14;
    }
    v5 = 0x112;
  }
  else if ((v5 == 0x10a) && (*(char *)(a0 + 0x41a))) {
    sub_f250(a0);
    goto label_11d14;
  }
  sub_f060(a0,v5);
label_11d14:
  *(unsigned long *)(a0 + 0x80) = sub_111a0(a0);
  return;
}

// Function: sub_11e50 @ 0x11e50
void sub_11e50(long a0)
{
  int v1;
  long v2;
  long v3; // rax
  int v4; // ebp
  
  sub_11cc0(a0);
  v2 = *(long *)(a0 + 0x80);
  do {
    while( true ) {
      if (4 <= (unsigned long)(v2 - 0x101U))
        return;
      if (v2 == 0x104) break;
      sub_f060(a0,v2);
label_11e87:
      v2 = sub_111a0(a0);
      *(long *)(a0 + 0x80) = v2;
    }
    v2 = *(long *)(a0 + 0x98);
    v3 = sub_11120(*(unsigned long *)(a0 + 0x90),v2);
    v2 -= v3;
    if (*(long *)(a0 + 0x40)) {
      if (*(int *)(a0 + 0x44) < 0)
        sub_f060(a0,0x103);
      v1 = *(int *)(a0 + 0x40);
      if (!v1) {
        sub_f060(a0,0x101);
        v1 = *(int *)(a0 + 0x40);
      }
      v4 = 1;
      if (v1 <= 1) goto label_11f83;
      do {
        v4 += 1;
        sub_efb0(a0,v2,v3);
        sub_f060(a0,0x105);
      } while (v4 < *(int *)(a0 + 0x40));
      if (v4 < *(int *)(a0 + 0x44)) {
        do {
          v4 += 1;
          sub_efb0(a0,v2,v3);
          sub_f060(a0,0x101);
          sub_f060(a0,0x105);
label_11f83:
        } while (v4 < *(int *)(a0 + 0x44));
      }
      goto label_11e87;
    }
    *(long *)(a0 + 0x98) = v2;
    *(unsigned long *)(a0 + 0x80) = sub_111a0(a0);
    sub_11e50(a0);
    v2 = *(long *)(a0 + 0x80);
  } while( true );
}

// Function: sub_11fc0 @ 0x11fc0
void sub_11fc0(long a0) // return-dupe x2
{
  long v1;
  
  sub_11e50(a0);
  v1 = *(long *)(a0 + 0x80);
  if (!(v1 - 0x106U & 0xfffffffffffffffdU))
    return;
  do {
    if (v1 <= -1)
      return;
    sub_11e50(a0);
    sub_f060(a0,0x105);
    v1 = *(long *)(a0 + 0x80);
  } while (v1 - 0x106U & 0xfffffffffffffffdU);
}

// Function: sub_12020 @ 0x12020
void sub_12020(unsigned int a0,unsigned int a1,long a2,long a3)
{
  sub_df70(*(unsigned short *)(a3 + 0x22 + a2 * 0x48),a0,a1); // tail-call
}

// Function: sub_12040 @ 0x12040
unsigned long sub_12040(long a0,long a1)
{
  return CONCAT71((undefined7)((unsigned long)(a0 * 9) >> 8),*(short *)(a1 + 0x22 + a0 * 0x48) != 0);
}

// Function: sub_12050 @ 0x12050
unsigned long sub_12050(long a0,long *a1,unsigned char a2)
{
  unsigned long *v1;
  unsigned long *v10;
  long v11;
  long v12;
  int v13;
  unsigned long v14;
  long v15; // rdx
  unsigned long v16; // rdx
  long v17; // rbx
  unsigned long v18 [4];
  unsigned long v19 [4]; // stack - 0x68
  char v2;
  void *v20; // stack - 0xe8
  void *v21; // stack - 0xc8
  void *v22 [4]; // stack - 0xa8
  long v23;
  long v24;
  long v25;
  long v26; // stack - 0xe0
  long v27; // stack - 0xc0
  char v3;
  unsigned int v4; // eax
  unsigned int v5; // eax
  unsigned long *v6;
  long *v7;
  unsigned long *v8;
  unsigned long *v9;
  
  v11 = a0;
  v3 = sub_12040(a0,a1[0x25]);
  v24 = a1[0x2e];
  v12 = v24;
  if (v3)
    v12 = a1[0x2f];
  v6 = (unsigned long *)(v12 + v11 * 8);
  v10 = (unsigned long *)*v6;
  if (!v10) {
    if (0x400 <= (int)a1[0x2d]) { // branch-flip
      v12 = (long)*(int *)((long)a1 + 0x16c);
      if (v12 < a1[0x2c]) {
        do {
          free(*(void **)(v24 + v12 * 8));
          free(*(void **)(a1[0x2f] + v12 * 8));
          *(unsigned long *)(a1[0x2f] + v12 * 8) = 0;
          v24 = a1[0x2e];
          *(unsigned long *)(v24 + v12 * 8) = 0;
          v12 += 1;
        } while (v12 < a1[0x2c]);
      }
      v13 = 1;
    }
    else {
      v13 = (int)a1[0x2d] + 1;
    }
    *(int *)&a1[0x2d] = v13;
    v10 = (unsigned long *)sub_1d0f0(0x800);
    *v6 = v10;
    v6 = v10;
    do {
      *v6 = 0xfffffffffffffffe;
      v6 = &v6[1];
    } while (&v10[0x100] != v6);
  }
  *(char *)(a1[0x30] + a0) = 0;
  v12 = a1[0x25];
  v24 = a0 * 0x48;
  v14 = (unsigned long)*(unsigned char *)(v12 + 0x20 + a0 * 0x48);
  v11 = a0;
  v3 = sub_12020(v14,4,a0,v12);
  if (v3) {
    *(unsigned char *)(a1[0x30] + v11) = *(unsigned char *)(a1[0x30] + v11) | 4;
    v12 = a1[0x25];
    v14 = (unsigned long)*(unsigned char *)(v12 + 0x20 + v24);
  }
  v11 = a0;
  v3 = sub_12020(v14 & 0xffffffff,2,a0,v12);
  if (v3) {
    *(unsigned char *)(a1[0x30] + v11) = *(unsigned char *)(a1[0x30] + v11) | 2;
    v12 = a1[0x25];
    v14 = (unsigned long)*(unsigned char *)(v12 + 0x20 + v24);
  }
  v11 = a0;
  if (sub_12020(v14 & 0xffffffff,1,a0,v12))
    *(unsigned char *)(a1[0x30] + v11) = *(unsigned char *)(a1[0x30] + v11) | 1;
  sub_f650(&v20,a1[0x16]);
  if (1 <= *(long *)(a1[0x25] + 0x10 + v24)) {
    v12 = a1[0x25] + v24;
    v11 = a1[0x28];
    v25 = 0;
    do {
      v7 = (long *)(v11 + *(long *)(*(long *)(v12 + 8) + v25 * 0x10) * 0x18);
      v23 = 0;
      if (1 <= v7[1]) {
        do {
          v17 = v23 + 1;
          v6 = (unsigned long *)(v23 * 0x10 + *v7);
          sub_e5c0(*v6,v6[1],&v20);
          v12 = a1[0x25] + v24;
          v11 = a1[0x28];
          v7 = (long *)(v11 + *(long *)(*(long *)(v12 + 8) + v25 * 0x10) * 0x18);
          v23 = v17;
        } while (v17 < v7[1]);
      }
      v25 += 1;
    } while (v25 < *(long *)(v12 + 0x10));
  }
  sub_f650(&v21,a1[0x16]);
  v12 = 0;
  sub_e020(v18);
  if (1 <= v26) {
    do {
      v7 = (long *)(v12 * 0x10 + (long)v20);
      v11 = *v7;
      v25 = v7[1];
      v13 = (int)v7[1];
      v14 = *(unsigned long *)(a1[0x12] + v11 * 8);
      if (0x100 <= v14) { // branch-flip
        if (0x114 <= (long)v14) { // branch-flip
          v23 = v14 * 0x20 + *a1;
          v19[0] = *(unsigned long *)(v23 + -0x2280);
          v19[1] = *(unsigned long *)(v23 + -0x2278);
          v19[2] = *(unsigned long *)(v23 + -0x2270);
          v19[3] = *(unsigned long *)(v23 + -0x2268);
          v3 = sub_dfc0(a2,v19);
          if (!v3) goto label_12551;
label_122fa:
          if (v13 != 0x1ff) { // branch-flip
label_12303:
            v2 = *(char *)(a1[0x25] + 0x20 + v24);
            if (!sub_df70(v13,v2,4)) {
              v8 = (unsigned long *)&a1[0x7e];
              v9 = v19;
              do {
                v14 = *v8;
                v8 = &v8[1];
                *v9 = *v9 & ~v14;
                v9 = &v9[1];
              } while (&v19[4] != v9);
            }
            if (!sub_df70(v13)) {
              v8 = (unsigned long *)&a1[0x7a];
              v9 = v19;
              do {
                v14 = *v8;
                v8 = &v8[1];
                *v9 = *v9 & ~v14;
                v9 = &v9[1];
              } while (&v19[4] != v9);
            }
            if (!sub_df70(v13)) {
              v8 = (unsigned long *)&a1[0x7e];
              v9 = v19;
              do {
                v1 = &v8[-4];
                v14 = *v8;
                v8 = &v8[1];
                *v9 = *v9 & (*v1 | v14);
                v9 = &v9[1];
              } while (&v19[4] != v9);
            }
            if (!sub_e080(v19)) {
              if (sub_dfc0(a2,v19)) goto label_12374;
              goto label_12561;
            }
          }
          else {
label_1237d:
            v9 = v19;
            v8 = v18;
            do {
              *v8 = *v8 & *v9;
              v8 = &v8[1];
              v9 = &v9[1];
            } while (&v18[4] != v8);
            sub_e6c0(v11,v25,&v21);
          }
        }
        else if (v14 == 0x10a) {
          v8 = (unsigned long *)(a1[3] * 0x20 + *a1);
          v19[0] = *v8;
          v19[1] = v8[1];
          v19[2] = v8[2];
          v19[3] = v8[3];
          v3 = sub_dfc0(a2,v19,1);
          v23 = v24 + a1[0x25];
          v2 = *(char *)(v23 + 0x20);
          if (sub_df70(v13,v2,v16)) {
            if (!*(long *)(v23 + 0x30)) {
              sub_f650(v23 + 0x28,1);
              v23 = v24 + a1[0x25];
            }
            sub_e5c0(v11,v25,v23 + 0x28);
          }
          if (v13 != 0x1ff) goto label_12303;
label_12374:
          if (v3) goto label_1237d;
          goto label_12561;
        }
      }
      else {
        sub_e250(v19,v19);
        sub_dfe0(v14 & 0xffffffff);
        if (v14 == a2) {
          v3 = '\x01';
          goto label_122fa;
        }
label_12551:
        v3 = '\0';
        if (v13 != 0x1ff) goto label_12303;
label_12561:
        v9 = v19;
        v8 = v18;
        do {
          v14 = *v9;
          v9 = &v9[1];
          *v8 = *v8 & ~v14;
          v8 = &v8[1];
        } while (&v18[4] != v8);
      }
      v12 += 1;
    } while (v12 < v26);
  }
  sub_f650(v22,a1[0x16]);
  if (1 <= v27) { // branch-flip
    if ((char)a1[0x29]) {
      if ((char)a1[0x83]) {
        v24 = 0;
        do {
          v12 = v24 + 1;
          if (!(*(unsigned char *)(a1[0x23] + *(long *)((long)v21 + v24 * 0x10)) & 1)) goto label_123fa;
          v24 = v12;
        } while (v27 != v12);
      }
      sub_fc10(&v21,a1[0x25] + 8,v22);
    }
label_123fa:
    v4 = sub_e140(a1,v18);
    v24 = -1;
    v5 = sub_10930();
    if (~v5 & v4)
      v24 = sub_f9a0(a1,&v21,v5 ^ 7);
    v12 = v24;
    if (v4 & v5 & 4)
      v12 = sub_f9a0(a1,&v21,4);
    v11 = v24;
    if (v4 & v5 & 2)
      v11 = sub_f9a0(a1,&v21,2);
    sub_f4a0(a1);
  }
  else if ((char)a1[0x29]) { // branch-flip
    v24 = a1[0x32];
    v12 = 0;
    v11 = (long)(*(int *)((long)a1 + 0x16c) + -1);
  }
  else {
    v11 = -1;
    v12 = -1;
    v24 = -1;
  }
  v14 = 0;
  do {
    if (sub_dfc0(v14 & 0xffffffff,v18)) {
      v3 = *(char *)((long)a1 + v15 + 0x1cf);
      if (v3 != '\x02') { // branch-flip
        v25 = v12;
        if (v3 != '\x04')
          v25 = v24;
        v10[v15] = v25;
      }
      else {
        v10[v15] = v11;
      }
    }
    v14 = v15 + 1;
  } while (v14 != 0x100);
  free(v21);
  free(v20);
  free(v22[0]);
  v14 = (unsigned long)*(unsigned char *)((long)a1 + 0x1ce);
  v3 = sub_dfc0(v14,v18);
  if (v3) {
    *(unsigned long *)(a1[0x31] + a0 * 8) = v10[v14];
    v10[v14] = 0xffffffffffffffff;
  }
  return v10[a2];
}

// Function: sub_12880 @ 0x12880
unsigned char * sub_12880(long a0,long a1,long *a2)
{
  unsigned char *v1; // rax
  long v2;
  
  v2 = *(long *)(*(long *)(a0 + 0x170) + a1 * 8);
  v1 = (unsigned char *)*a2;
  if (!v2) {
    v2 = *(long *)(*(long *)(a0 + 0x178) + a1 * 8);
    if (!v2) {
      sub_12050(a1,a0,*v1);
      v2 = *(long *)(*(long *)(a0 + 0x170) + a1 * 8);
      if ((!v2) && (v2 = *(long *)(*(long *)(a0 + 0x178) + a1 * 8), !v2))
        __assert_fail("t","dfa.c",0xcce,"transit_state_singlebyte"); // no-return
      v1 = (unsigned char *)*a2;
    }
  }
  if (*(long *)(v2 + (unsigned long)*v1 * 8) == -2) {
    sub_12050(a1,a0,(unsigned long)*v1);
    v1 = (unsigned char *)*a2;
  }
  *a2 = (long)&v1[1];
  v1 = (unsigned long)(unsigned long)*v1;
  v1 = (unsigned char *)*(unsigned long *)(v2 + (long)v1 * 8);
  return v1;
}

// Function: sub_12950 @ 0x12950
long sub_12950(long a0,long a1,long *a2,long a3)
{
  long *v1;
  long v10;
  int v11; // ebx
  int v12; // stack - 0x34
  long v13;
  long v14;
  unsigned long v2;
  unsigned long v3;
  int v4; // eax
  long v5;
  long v6;
  unsigned long *v7;
  long v8;
  unsigned long *v9;
  
  v4 = sub_e260(&v12,*a2,a3 - *a2,a0);
  *(unsigned long *)(a0 + 0x1a0) = 0;
  v5 = a1;
  if (1 <= v4) {
    v11 = 0;
    do {
      v11 += 1;
      v5 = sub_12880(a0,v5,a2);
      if (v4 == v11) {
        v4 = 0;
        goto label_129cf;
      }
    } while (*(int *)(a0 + 0x16c) <= v5);
    v4 -= v11;
  }
label_129cf:
  *a2 = *a2 + (long)v4;
  if (v12 != -1) {
    v8 = *(long *)(a0 + 0x128);
    v6 = *(long *)(a0 + 0x1b0);
    a1 *= 0x48;
    v13 = v8 + a1;
    if (*(long *)(v13 + 0x40) < 0) {
      v10 = *(long *)(a0 + 0x1b8);
      v14 = v10 + 1;
      if (0x400 <= v10) {
        if (0 <= *(long *)(a0 + 0x160)) {
          v13 = -1;
          do {
            free(*(void **)(v6 + v13 * 8));
            v6 = *(long *)(a0 + 0x1b0);
            *(unsigned long *)(v6 + v13 * 8) = 0;
            v13 += 1;
          } while (v13 < *(long *)(a0 + 0x160));
          v8 = *(long *)(a0 + 0x128);
          v13 = v8 + a1;
        }
        v14 = *(long *)(a0 + 0x130);
        if (1 <= v14) {
          v9 = (unsigned long *)(v8 + 0x40);
          do {
            *v9 = 0xffffffffffffffff;
            v9 = &v9[9];
          } while ((unsigned long *)(v8 + 0x40 + v14 * 0x48) != v9);
        }
        v14 = 1;
        v10 = 0;
      }
      *(long *)(a0 + 0x1b8) = v14;
      *(long *)(v13 + 0x40) = v10;
    }
    v13 = v5 * 8;
    v1 = (long *)(v6 + v13);
    v6 = *v1;
    if (v6) { // branch-flip
      v14 = v8 + a1;
      v6 = *(long *)(v6 + *(long *)(v14 + 0x40) * 8);
      if (0 <= v6)
        return v6;
    }
    else {
      *v1 = sub_1d0f0(0x2000);
      v7 = *(unsigned long **)(*(long *)(a0 + 0x1b0) + v5 * 8);
      v9 = &v7[0x400];
      do {
        *v7 = 0xffffffffffffffff;
        v7 = &v7[1];
      } while (v9 != v7);
      v8 = *(long *)(a0 + 0x128);
      v14 = v8 + a1;
    }
    v6 = a0 + 0x198;
    if (v5 != -1) // branch-flip
      sub_f7d0(v14 + 0x28,v8 + 8 + v5 * 0x48,v6);
    else {
      sub_f920(v14 + 0x28,v6);
    }
    v2 = *(unsigned long *)(a0 + 0x1a0);
    v3 = *(unsigned long *)(a0 + 0x198);
    v5 = sub_f9a0(a0,v6,sub_10930(a0,v3,v2) ^ 7);
    sub_f4a0(a0);
    *(long *)(*(long *)(*(long *)(a0 + 0x1b0) + v13) + *(long *)(*(long *)(a0 + 0x128) + 0x40 + a1) * 8) = v5;
  }
  return v5;
}

// Function: sub_12c00 @ 0x12c00
unsigned char * sub_12c00(long a0,unsigned char *a1,unsigned char *a2,char a3,long *a4,char a5)
{
  unsigned long *v1;
  long v10;
  unsigned char *v11; // stack - 0x48
  long v12;
  long v13;
  long v14;
  long v15;
  long v16; // stack - 0x78
  unsigned char v2;
  unsigned char v3;
  char v4;
  char v5;
  unsigned char *v6;
  unsigned char *v7;
  long v8;
  unsigned long *v9;
  
  if (0x400 <= *(long *)(a0 + 0x130)) { // branch-flip
    v13 = (long)*(int *)(a0 + 0x16c);
    if (v13 < *(long *)(a0 + 0x130)) {
      do {
        v8 = v13 + 1;
        free(*(void **)(*(long *)(a0 + 0x128) + 8 + v13 * 0x48));
        free(*(void **)(*(long *)(a0 + 0x128) + 0x28 + v13 * 0x48));
        v13 = v8;
      } while (v8 < *(long *)(a0 + 0x130));
      v13 = (long)*(int *)(a0 + 0x16c);
    }
    v8 = *(long *)(a0 + 0x170);
    *(long *)(a0 + 0x130) = v13;
    v13 = *(long *)(a0 + 0x160);
    if (v8) {
      if (1 <= v13) {
        v14 = 0;
        do {
          free(*(void **)(v8 + v14 * 8));
          free(*(void **)(*(long *)(a0 + 0x178) + v14 * 8));
          v8 = *(long *)(a0 + 0x170);
          *(unsigned long *)(*(long *)(a0 + 0x178) + v14 * 8) = 0;
          v13 = *(long *)(a0 + 0x160);
          *(unsigned long *)(v8 + v14 * 8) = 0;
          v14 += 1;
        } while (v14 < v13);
      }
      *(unsigned int *)(a0 + 0x168) = 0;
    }
    if ((*(char *)(a0 + 0x418)) && (v8 = *(long *)(a0 + 0x1b0), v8)) {
      if (0 <= v13) {
        v14 = -1;
        do {
          free(*(void **)(v8 + v14 * 8));
          v8 = *(long *)(a0 + 0x1b0);
          v13 = *(long *)(a0 + 0x160);
          *(unsigned long *)(v8 + v14 * 8) = 0;
          v14 += 1;
        } while (v14 < v13);
      }
      if (1 <= *(int *)(a0 + 0x16c)) {
        v9 = (unsigned long *)(*(long *)(a0 + 0x128) + 0x40);
        v1 = &v9[(long)*(int *)(a0 + 0x16c) * 9];
        do {
          *v9 = 0xffffffffffffffff;
          v9 = &v9[9];
        } while (v1 != v9);
      }
      *(unsigned long *)(a0 + 0x1b8) = 0;
    }
  }
  else {
    v13 = *(long *)(a0 + 0x160);
  }
  if (!v13)
    sub_f4a0(a0);
  v13 = *(long *)(a0 + 0x170);
  v2 = *(unsigned char *)(a0 + 0x1ce);
  v3 = *a2;
  *a2 = v2;
  v11 = a1;
  if ((a5) && (*(unsigned long *)(a0 + 0x110) = 0, !*(long *)(a0 + 0x1a8))) {
    sub_f650(a0 + 0x198,*(unsigned long *)(a0 + 0xb0));
    v6 = v11;
  }
  else {
    v6 = a1;
  }
  v16 = 0;
  v8 = 0;
  v14 = 0;
label_12cb0:
  while( true ) {
    v12 = *(long *)(v13 + v8 * 8);
    v10 = v8;
    v15 = v8;
    v7 = v6;
    if (!v12) break;
label_12cc0:
    v14 = v8;
    if (*(int *)(a0 + 0x16c) <= v14) {
      if (!a5) {
        v7 = &v6[1];
        v10 = *(long *)(v12 + (unsigned long)*v6 * 8);
        v8 = *(long *)(v13 + v10 * 8);
        v14 = v10;
        if (v8) goto label_12edf;
        v14 = v15;
        v11 = v7;
        break;
      }
label_12d10:
      if (((*(long *)(*(long *)(a0 + 0x128) + 0x30 + v14 * 0x48)) && (*(int *)(a0 + 0x51c + (unsigned long)*v6 * 4) == -1)) && (v10 = v14, v6 < a2)) goto label_130a8;
      v7 = &v6[1];
      v10 = *(long *)(v12 + (unsigned long)*v6 * 8);
      v12 = *(long *)(v13 + v10 * 8);
      v8 = v10;
      v15 = v10;
      v6 = v7;
      v11 = v7;
      if (!v12) break;
      goto label_12cc0;
    }
    if (a5) {
      if (!*(long *)(*(long *)(a0 + 0x128) + 0x30 + v14 * 0x48)) {
        if (v14 == *(long *)(v12 + (unsigned long)*v6 * 8)) goto label_12f10;
      }
label_12cf6:
      v6 = (unsigned char *)sub_e380(a0,v6,a1,a2);
      v11 = v6;
      a1 = v6;
      goto label_12d10;
    }
    if (v14 == *(long *)(v12 + (unsigned long)*v6 * 8)) {
label_12f10:
      do {
        v6 = &v6[1];
      } while (*(long *)(v12 + (unsigned long)*v6 * 8) == v14);
      v11 = v6;
      if (a5) goto label_12cf6;
    }
    v7 = &v6[1];
    v10 = *(long *)(v12 + (unsigned long)*v6 * 8);
    v8 = *(long *)(v13 + v10 * 8);
    if (!v8) {
      v11 = v7;
      break;
    }
    v14 = v10;
    if (*(long *)(v8 + (unsigned long)v6[1] * 8) == v10) {
      v6 = &v6[2];
      do {
        v7 = v6;
        v6 = &v7[1];
      } while (*(long *)(v8 + (unsigned long)*v7 * 8) == v10);
    }
label_12edf:
    v6 = &v7[1];
    v8 = *(long *)(v8 + (unsigned long)*v7 * 8);
    v11 = v6;
  }
  if (0 > v10) {
    if (v10 != -2) { // branch-flip
      if (((a2 < v7) || (v7[-1] != v2)) || (v8 = *(long *)(*(long *)(a0 + 0x188) + v14 * 8), v8 < 0)) {
        v7 = NULL;
label_12fd7:
        if (a4)
          *a4 = *a4 + v16;
        *a2 = v3;
        return v7;
      }
      v16 += 1;
      v6 = v7;
      a1 = v7;
      if (!a3) {
        v8 = 0;
        v4 = *(char *)(a0 + 0x1cf + (unsigned long)v2);
        if (v4 != '\x04') {
          if (v4 != '\x02') // branch-flip
            v8 = *(long *)(a0 + 400);
          else {
            v8 = (long)(*(int *)(a0 + 0x16c) + -1);
          }
        }
      }
    }
    else {
      v8 = sub_12050(v14,a0,v7[-1]);
      v13 = *(long *)(a0 + 0x170);
      v6 = v11;
    }
    goto label_12cb0;
  }
  v8 = *(long *)(*(long *)(a0 + 0x178) + v10 * 8);
  if (!v8) {
    sub_12050(v10,a0,(unsigned long)*v7);
    v13 = *(long *)(a0 + 0x170);
    v8 = v10;
    v6 = v11;
    goto label_12cb0;
  }
  if ((*(unsigned char *)(*(long *)(a0 + 0x180) + v10) & *(unsigned char *)(a0 + 0x1cf + (unsigned long)*v7)) || ((a2 == v7 && (v5 = *(char *)(*(long *)(a0 + 0x128) + 0x20 + v10 * 0x48), sub_12020(v5,4,v10))))) goto label_12fd7;
  if (a5) {
    if (v10 < *(int *)(a0 + 0x16c)) {
      v7 = (unsigned char *)sub_e380(a0,v7,a1,a2);
      v11 = v7;
      a1 = v7;
    }
    if (((*(long *)(*(long *)(a0 + 0x128) + v10 * 0x48 + 0x30)) && (*(int *)(a0 + 0x51c + (unsigned long)*v7 * 4) == -1)) && (v7 < a2)) {
label_130a8:
      v8 = sub_12950(a0,v10,&v11,a2);
      v13 = *(long *)(a0 + 0x170);
      v6 = v11;
      a1 = v11;
      v14 = v10;
      goto label_12cb0;
    }
    v8 = *(long *)(*(long *)(a0 + 0x178) + v10 * 8);
  }
  v8 = *(long *)(v8 + (unsigned long)*v7 * 8);
  v6 = &v7[1];
  v14 = v10;
  v11 = &v7[1];
  goto label_12cb0;
}

// Function: sub_13310 @ 0x13310
void sub_13310(void)
{
  sub_12c00(); // tail-call
}

// Function: sub_13320 @ 0x13320
void sub_13320(void)
{
  sub_12c00(); // tail-call
}

// Function: sub_13340 @ 0x13340
void sub_13340(unsigned long a0,unsigned long a1,long a2)
{
  *(unsigned long *)(a2 + 0x20) = a0;
  *(unsigned long *)(a2 + 0x28) = a1;
  *(unsigned long *)(a2 + 0x30) = 0xffffffffffffffff;
  *(char *)(a2 + 0x78) = 1;
  if (!*(char *)(a2 + 0x1cc))
    sub_6630(dcgettext(NULL,"no syntax specified",5)); // no-return
  if (!*(long *)(a2 + 0xb8))
    sub_f060(a2,0x10b);
  *(unsigned long *)(a2 + 0x80) = sub_111a0(a2);
  *(unsigned long *)(a2 + 0x88) = *(unsigned long *)(a2 + 0xa8);
  sub_11c60(a2);
  if (*(long *)(a2 + 0x80) == -1) {
    sub_f060(a2,~*(unsigned long *)(a2 + 0xb8));
    sub_f060(a2,0x105);
    if (!*(long *)(a2 + 0xb8)) {
      *(unsigned long *)(a2 + 0xb8) = 1;
      return;
    }
    sub_f060(a2,0x106);
    *(long *)(a2 + 0xb8) = *(long *)(a2 + 0xb8) + 1;
    return;
  }
  sub_6630(dcgettext(NULL,"unbalanced )",5)); // no-return
}

// Function: sub_13450 @ 0x13450
void sub_13450(long a0)
{
  (**(void **)(a0 + 0x410))(); // jump-as-call
}

// Function: sub_13460 @ 0x13460
unsigned long sub_13460(long a0)
{
  return *(unsigned long *)(a0 + 0x120);
}

// Function: sub_13470 @ 0x13470
char sub_13470(long a0)
{
  return *(char *)(a0 + 0xc0);
}

// Function: sub_13480 @ 0x13480
unsigned long sub_13480(long a0) // return-dupe
{
  long v1;
  long v2; // rdx
  
  if (*(long *)(a0 + 0x98) <= 0)
    return 1;
  v2 = 0;
  do {
    v1 = *(long *)(*(long *)(a0 + 0x90) + v2 * 8);
    if (0x112 <= v1) { // branch-flip
      if ((unsigned long)(v1 - 0x112U) <= 1)
        return 0;
    }
    else if ((0x10d < v1) && (*(char *)(a0 + 0x418)))
      return 0;
    v2 += 1;
    if (v2 == *(long *)(a0 + 0x98))
      return 1;
  } while( true );
}

// Function: sub_134e0 @ 0x134e0
void sub_134e0(unsigned long *a0)
{
  long v1; // rbp
  long v2;
  void *v3;
  
  free((void *)*a0);
  free((void *)a0[0x12]);
  if (*(char *)&a0[0x83])
    sub_f7e0(a0);
  free((void *)a0[0x2a]);
  free((void *)a0[0x2b]);
  v2 = 0;
  if (1 <= (long)a0[0x26]) {
    do {
      v1 = v2 + 1;
      free(*(void **)(a0[0x25] + 8 + v2 * 0x48));
      free(*(void **)(a0[0x25] + 0x28 + v2 * 0x48));
      v2 = v1;
    } while (v1 < (long)a0[0x26]);
  }
  free((void *)a0[0x25]);
  v3 = (void *)a0[0x28];
  if (v3) {
    if (1 <= (long)a0[0x13]) {
      v2 = 0;
      do {
        v1 = v2 + 1;
        free(*(void **)((long)v3 + v2 * 0x18));
        v3 = (void *)a0[0x28];
        v2 = v1;
      } while (v1 < (long)a0[0x13]);
    }
    free(v3);
  }
  v2 = a0[0x2e];
  if (v2) {
    if (1 <= (long)a0[0x2c]) {
      v1 = 0;
      do {
        free(*(void **)(v2 + v1 * 8));
        v2 = v1 * 8;
        v1 += 1;
        free(*(void **)(a0[0x2f] + v2));
        v2 = a0[0x2e];
      } while (v1 < (long)a0[0x2c]);
    }
    free((void *)(v2 + -0x10));
    free((void *)a0[0x2f]);
    free((void *)a0[0x31]);
    free((void *)a0[0x30]);
  }
  if (a0[0x24]) {
    sub_134e0(a0[0x24]);
    free((void *)a0[0x24]); // tail-call
    return;
  }
}

// Function: sub_13670 @ 0x13670
void sub_13670(long a0)
{
  long v1;
  long v2; // rdx
  char v3;
  
  if (!*(char *)(a0 + 0x41a))
    return;
  if (1 <= *(long *)(a0 + 0x98)) {
    v2 = 0;
    v3 = '\0';
    do {
      v1 = *(long *)(*(long *)(a0 + 0x90) + v2 * 8);
      if (v1 != 0x112) { // branch-flip
        if (v1 == 0x113)
          return;
        if (v1 == 0x10a)
          abort(); // no-return
      }
      else {
        v3 = *(char *)(a0 + 0x41a);
      }
      v2 += 1;
    } while (v2 != *(long *)(a0 + 0x98));
    if (v3) goto label_136c1;
  }
  if (*(long *)(a0 + 0x120)) {
    sub_134e0(*(long *)(a0 + 0x120));
    free(*(void **)(a0 + 0x120));
    *(unsigned long *)(a0 + 0x120) = 0;
  }
label_136c1:
  sub_f7e0(a0);
  *(char *)(a0 + 0x418) = 0;
  *(void **)(a0 + 0x410) = sub_13310;
  *(char *)(a0 + 0xc0) = 1;
  return;
}

// Function: sub_13730 @ 0x13730
char * sub_13730(long *a0)
{
  unsigned long *v1;
  unsigned long v10;
  unsigned long v11; // rax
  unsigned long v12; // rax
  unsigned long v13;
  unsigned long v14;
  char *v15;
  void *v16; // rax
  long v17;
  long v18;
  unsigned long v19; // rcx
  unsigned long v2;
  char *v20;
  char v21;
  char v22;
  long v23;
  char *v24;
  long v25; // rsi
  long v26;
  long v27;
  char v28;
  long *v29;
  bool v3;
  int v30;
  int v31; // r14d
  long v32;
  bool v4;
  unsigned char v5; // al
  unsigned char v6;
  char v7;
  int v8;
  unsigned long *v9;
  
  v5 = (*(unsigned char *)&a0[0x83] ^ 1) & *(unsigned char *)((long)a0 + 0x1cd);
  if (a0[0x13] <= 2)
    return NULL;
  v4 = 0;
  v9 = NULL;
  v23 = 1;
  v3 = 0;
label_13780:
  v17 = a0[0x12];
  v18 = v23 * 8;
  v27 = *(long *)(v17 + v23 * 8);
  if (0x113 < v27) {
    v26 = *a0 + -0x2280 + v27 * 0x20;
    v30 = 0;
    do {
      v31 = v30 + 1;
      if (sub_dfc0(v30,v26)) {
        v27 = (long)v30;
        if (v31 != 0x100) goto label_138b8;
        goto label_13908;
      }
      v30 = v31;
    } while (v31 != 0x100);
label_137d0:
    v9 = (unsigned long *)sub_10570(v9,2);
    v26 = v23 + 2;
    goto label_137e8;
  }
  if (0x100 <= v27) {
    switch(v27) {
      case 0x100:
      case 0x10a:
      case 0x10e:
      case 0x10f:
      case 0x110:
      case 0x111:
      case 0x112:
      case 0x113:
        goto label_137d0;
      case 0x101:
      case 0x102:
        sub_f8a0(v9);
        v26 = v23 + 2;
        break;
      case 0x103:
        v26 = v23 + 2;
        *(char *)v9[3] = 0;
        break;
      default:
        goto label_13908;
      case 0x105:
        v1 = (unsigned long *)v9[5];
        v14 = sub_10710(*v1,*v9);
        *v1 = v14;
        if ((*(char *)v1[2]) && (v20 = (char *)v9[1], *v20)) {
          v10 = strlen((char *)v1[2]);
          v11 = strlen(v20);
          v16 = (void *)sub_1d110(v10 + 1 + v11);
          memcpy((void *)((long)v16 + v10),(void *)v9[1],v11 + 1);
          v16 = memcpy(v16,(void *)v1[2],v10);
          v14 = *v1;
          *v1 = sub_105e0(v14,v16);
        }
        if (*(char *)v1[3]) {
          v14 = v9[1];
          v2 = v1[1];
          v1[1] = sub_10760(v2,v14);
        }
        if (!*(char *)v9[3])
          *(char *)v1[2] = 0;
        v14 = sub_10760(v1[2],v9[2]);
        v20 = (char *)v1[3];
        v1[2] = v14;
        if (((*v20) || (*(char *)&v1[4])) && ((v24 = (char *)v9[3], *v24 || (*(char *)((long)v9 + 0x21))))) {
          v1[3] = sub_10760(v20,v24);
          v21 = *(char *)((long)v9 + 0x21);
        }
        else {
          *v20 = '\0';
          v21 = 0;
          *(char *)&v1[4] = 0;
        }
        *(char *)((long)v1 + 0x21) = v21;
        sub_f8e0(v9);
        v26 = v23 + 2;
        v9 = v1;
        break;
      case 0x106:
        v1 = (unsigned long *)v9[5];
        v14 = v9[3];
        v15 = (char *)v1[3];
        if (sub_e440(v15,v14)) { // branch-flip
          *(unsigned char *)&v1[4] = *(unsigned char *)&v1[4] & *(unsigned char *)&v9[4];
          v6 = *(unsigned char *)((long)v1 + 0x21) & *(unsigned char *)((long)v9 + 0x21);
        }
        else {
          *v15 = 0;
          *(char *)&v1[4] = 0;
          v6 = 0;
        }
        *(unsigned char *)((long)v1 + 0x21) = v6;
        v20 = (char *)v1[1];
        v22 = *v20;
        if (v22) {
          v24 = (char *)v9[1];
          do {
            if (*v24 != v22) break;
            v22 = v20[1];
            v20 = &v20[1];
            v24 = &v24[1];
          } while (v22);
        }
        *v20 = '\0';
        v20 = (char *)v1[2];
        v11 = strlen(v20);
        v24 = (char *)v9[2];
        v12 = strlen(v24);
        v10 = v11;
        if ((long)v12 <= (long)v11)
          v10 = v12;
        if (1 <= (long)v10) {
          v19 = 0;
          v13 = v11;
          do {
            if (v20[v13 - 1] != v24[(v13 - 1) + (v12 - v11)]) {
              if (v19) goto label_13b1a;
              goto label_13b42;
            }
            v19 += 1;
            v13 -= 1;
          } while (v10 != v19);
          v13 = v11 - v19;
label_13b1a:
          v18 = 0;
          while( true ) {
            v20[v18] = v20[v13 + v18];
            v18 += 1;
            if ((long)v19 <= v18) break;
            v20 = (char *)v1[2];
          }
          v20 = (char *)(v19 + v1[2]);
        }
label_13b42:
        *v20 = '\0';
        v14 = sub_108a0(*v1,*v9);
        sub_f870(*v1);
        free((void *)*v1);
        *v1 = v14;
        sub_f8e0(v9);
        v26 = v23 + 2;
        v9 = v1;
        break;
      case 0x107:
      case 0x108:
        __assert_fail("!\"neither LPAREN nor RPAREN may appear here\"","dfa.c",0x1035,"dfamust"); // no-return
      case 0x10c:
        v9 = (unsigned long *)sub_10570(v9,2);
        v3 = 1;
        v26 = v23 + 2;
        *(char *)&v9[4] = 1;
        break;
      case 0x10d:
        v9 = (unsigned long *)sub_10570(v9,2);
        v4 = 1;
        v26 = v23 + 2;
        *(char *)((long)v9 + 0x21) = 1;
      
    }
    goto label_137e8;
  }
  if (v27 == -1) {
    if (v9[5])
      __assert_fail("!mp->prev","dfa.c",0x107d,"dfamust"); // no-return
    v29 = (long *)*v9;
    v24 = "";
    v20 = (char *)*v29;
    if (v20) goto label_13cc0;
    v14 = v9[3];
    v15 = NULL;
    if (!sub_e440("",v14)) goto label_13820;
    v22 = '\0';
    goto label_13d58;
  }
  if (v27) goto label_13908;
  goto label_13813;
label_138b8:
  do {
    v8 = v31;
    if (sub_dfc0(v31,v26)) {
      if (!v5) goto label_137d0;
      v8 = toupper(v8);
      if (v8 != toupper(v30)) goto label_137d0;
    }
    v31 += 1;
  } while (v31 != 0x100);
  v17 = a0[0x12];
label_13908:
  v26 = v23 + 2;
  v25 = 2;
  v32 = v26;
  if ((*(long *)(v17 + 8 + v18) == 0x105) && (v26 < a0[0x13] + -1)) {
    while ((v32 == v23 || ((unsigned long)(*(long *)(v17 + v32 * 8) - 1U) <= 0xfe))) {
      if ((*(long *)(v17 + 8 + v32 * 8) != 0x105) || (v32 = v32 + 2, a0[0x13] + -1 <= v32)) break;
    }
    v25 = (v32 - v23 >> 1) + 1;
  }
  v9 = (unsigned long *)sub_10570(v9,v25);
  v21 = (char)v27;
  if (v5)
    v21 = (char)toupper((int)v27);
  *(char *)v9[2] = v21;
  *(char *)v9[1] = v21;
  *(char *)v9[3] = v21;
  if (v32 <= v26) { // branch-flip
    v17 = 1;
    v18 = 1;
  }
  else {
    v27 = 1;
    v10 = (unsigned long)((v32 - v23) - 3U) >> 1;
    v17 = v10 + 2;
    do {
      v18 += 0x10;
      v14 = *(unsigned long *)(a0[0x12] + v18);
      v21 = (char)v14;
      if (v5)
        v21 = (char)toupper((int)v14);
      *(char *)(v9[2] + v27) = v21;
      *(char *)(v9[1] + v27) = v21;
      *(char *)(v9[3] + v27) = v21;
      v27 += 1;
    } while (v17 != v27);
    v18 = v10 * 2;
    v27 = v18 + v26;
    v26 = v23 + 4 + v18;
    v18 = v17;
    v23 = v27;
  }
  *(char *)(v9[2] + v17) = 0;
  *(char *)(v9[1] + v17) = 0;
  *(char *)(v9[3] + v17) = 0;
  v14 = v9[3];
  v2 = *v9;
  *v9 = sub_107c0(v2,v14,v18);
label_137e8:
  v23 += 1;
  if (a0[0x13] <= v26) goto label_13813;
  goto label_13780;
label_13813:
  v15 = NULL;
  if (!v9)
    return NULL;
  goto label_13820;
label_13cc0:
  do {
    v29 = &v29[1];
    v10 = strlen(v20);
    if (strlen(v24) < v10)
      v24 = v20;
    v20 = (char *)*v29;
  } while (v20);
  v7 = sub_e440(v24,v9[3]);
  v22 = *v24;
  if (v7) { // branch-flip
label_13d58:
    v7 = *(char *)&v9[4];
    v21 = *(char *)((long)v9 + 0x21);
    if (((!v3) || (v28 = 0, v7)) && (v28 = 1, v4))
      v28 = v21;
  }
  else {
    v21 = 0;
    v7 = '\0';
    v28 = 0;
  }
  if (v22) { // branch-flip
    v15 = (char *)sub_1d0f0(strlen(v24) + 4);
    *v15 = v28;
    v15[1] = v7;
    v15[2] = v21;
    strcpy(&v15[3],v24);
  }
  else {
    v15 = NULL;
  }
label_13820:
  do {
    v1 = (unsigned long *)v9[5];
    sub_f8e0(v9);
    v9 = v1;
  } while (v1);
  return v15;
}

// Function: sub_13ef0 @ 0x13ef0
void sub_13ef0(void *a0)
{
  free(a0); // tail-call
}

// Function: sub_13f00 @ 0x13f00
void sub_13f00(void)
{
  sub_1d0f0(0x920); // tail-call
}

// Function: sub_13f10 @ 0x13f10
void sub_13f10(unsigned long *a0)
{
  long v1;
  void *v10; // rax
  long v11; // rax
  long v12;
  long v13; // rbx
  char v14 [40];
  unsigned long *v15;
  unsigned long *v16;
  long v17;
  long v18;
  unsigned long *v2;
  unsigned long *v3;
  unsigned long v4;
  unsigned long v5;
  bool v6;
  bool v7;
  bool v8;
  unsigned long *v9; // rax
  
  v9 = (unsigned long *)sub_13f00();
  v12 = 0x124;
  v15 = a0;
  v16 = v9;
  while (v12) {
    v3 = &v16[1];
    v2 = &v15[1];
    *v16 = *v15;
    v12 -= 1;
    v15 = v2;
    v16 = v3;
  }
  *(char *)&v9[0x83] = 0;
  v9[0x82] = sub_13310;
  v9[0x23] = 0;
  v9[0x24] = 0;
  v9[0x25] = 0;
  v9[0x26] = 0;
  v9[0x2a] = 0;
  v9[0x2b] = 0;
  v9[0x28] = 0;
  v9[0x2c] = 0;
  v9[0x2e] = 0;
  v9[0x2f] = 0;
  v9[0x30] = 0;
  v9[0x31] = 0;
  v10 = (void *)sub_1d210(v9[2],0x20);
  v12 = a0[1];
  *v9 = v10;
  if (v12)
    memcpy(v10,(void *)*a0,v12 << 5);
  v4 = a0[0x13];
  v9[0x12] = sub_1d210(v4,0x10);
  v12 = a0[0x13];
  v9[0x14] = v12 * 2;
  if (1 <= v12) { // branch-flip
    v12 = a0[0x12];
    v13 = 0;
    v7 = 0;
    v17 = 0;
    v8 = 0;
    do {
      while( true ) {
        v6 = v8;
        v5 = *(unsigned long *)(v12 + v13 * 8);
        v18 = v17 + 1;
        v1 = v17 * 8;
        if ((long)v5 <= 0x111) break;
        if (2 <= v5 - 0x112) {
label_14050:
          *(unsigned long *)(v9[0x12] + v1) = v5;
          if (v5 <= 0xff || 0x113 < (long)v5)
            v7 = v5 <= 0xff || 0x113 < (long)v5;
          goto label_14076;
        }
label_140c0:
        v18 = v17 + 2;
        sub_e020(v14,v14);
        v11 = sub_e500(v9);
        v17 = v9[0x12];
        v6 = 1;
        v12 = a0[0x12];
        *(long *)(v17 + v1) = v11 + 0x114;
        *(unsigned long *)(v17 + 8 + v1) = 0x102;
        v13 = v13 + (unsigned long)((unsigned long)(*(long *)(v12 + 8 + v13 * 8) - 0x101U) < 3) + 1;
        v17 = v18;
        v8 = 1;
        if ((long)a0[0x13] <= v13) goto label_14134;
      }
      if ((long)v5 <= 0x10d) {
        if (v5 != 0x10a) goto label_14050;
        goto label_140c0;
      }
      if (*(char *)&a0[0x83]) // branch-flip
        *(unsigned long *)(v9[0x12] + v1) = 0x100;
      else {
        *(unsigned long *)(v9[0x12] + v1) = v5;
      }
label_14076:
      v13 += 1;
      v17 = v18;
      v8 = v6;
    } while (v13 < (long)a0[0x13]);
label_14134:
    v9[0x13] = v18;
    if ((v7) && ((v6 || (*(char *)&a0[0x83])))) {
      a0[0x24] = v9;
      return;
    }
  }
  else {
    v9[0x13] = 0;
  }
  sub_134e0(v9);
  free(v9); // tail-call
}

// Function: sub_14250 @ 0x14250
void sub_14250(long a0,unsigned long a1,long a2,char a3)
{
  if (a0)
    sub_13340();
  sub_13f10(a2);
  if (sub_13480(a2)) { // branch-flip
    sub_13670();
    sub_10970(a2,a3);
  }
  else {
    *(void **)(a2 + 0x410) = sub_e1c0;
  }
  if (*(long *)(a2 + 0x120)) {
    *(char *)(a2 + 0xc0) = 1;
    sub_10970(*(long *)(a2 + 0x120),a3); // tail-call
    return;
  }
}

// Function: sub_142e0 @ 0x142e0
void sub_142e0(unsigned long *a0,unsigned char *a1,unsigned long a2,unsigned int a3) // ternary
{
  unsigned char *v1;
  unsigned int v10; // r14d
  bool v11; // al
  unsigned long *v2;
  unsigned char v3;
  char v4; // al
  unsigned long v5; // rcx
  void *v6; // rcx
  long v7; // rcx
  unsigned int v8; // ebp
  unsigned long *v9;
  
  v8 = 0xffffff80;
  *a0 = 0;
  a0[0x81] = 0;
  v5 = (unsigned long)(((int)a0 - (int)(unsigned long *)((unsigned long)&a0[1] & 0xfffffffffffffff8)) + 0x410U >> 3);
  v9 = (unsigned long *)((unsigned long)&a0[1] & 0xfffffffffffffff8);
  while (v5) {
    v2 = &v9[1];
    *v9 = 0;
    v5 -= 1;
    v9 = v2;
  }
  v6 = sub_13320;
  v3 = *a1;
  if (!v3)
    v6 = sub_13310;
  a0[0x82] = v6;
  v7 = 0xa0;
  v9 = &a0[0x83];
  while (v7) {
    v2 = &v9[1];
    v1 = &a1[8];
    *v9 = *(unsigned long *)a1;
    v7 -= 1;
    a1 = v1;
    v9 = v2;
  }
  *(unsigned int *)v9 = *(unsigned int *)a1;
  *(unsigned char *)&a0[0x18] = v3 ^ 1;
  a0[3] = 0xffffffffffffffff;
  *(char *)((long)a0 + 0x1cc) = 1;
  *(unsigned char *)((long)a0 + 0x1cd) = (unsigned char)(a2 >> 0x16) & 1;
  a0[0x38] = a2;
  *(unsigned int *)&a0[0x39] = a3;
  *(unsigned char *)((long)a0 + 0x1ce) = -((a3 & 2) == 0) & 10;
  do {
    v10 = v8 & 0xff;
    v5 = (unsigned long)v10;
    v4 = sub_e210(a0,v5);
    *(char *)((long)a0 + v5 + 0x1cf) = v4;
    if (v4 != '\x02') { // branch-flip
      if (v4 == '\x04')
        sub_dfe0(v5,&a0[0x7e]);
    }
    else {
      sub_dfe0(v10,&a0[0x7a]);
    }
    v11 = (*(char *)((long)a0 + 0x41a)) ? (v8 & 0xc0) != 0x80 : strchr("\n\r./",v10) != NULL; // branch-flip
    v8 += 1;
    *(bool *)((long)a0 + v5 + 0x2cf) = v11;
  } while (v8 != 0x80);
}

// Function: sub_14440 @ 0x14440
void sub_14440(unsigned long *a0,long a1)
{
  unsigned long *v1;
  unsigned long *v2;
  unsigned long v3;
  unsigned long v4; // rcx
  long v5; // rcx
  unsigned long *v6;
  unsigned long *v7;
  
  *a0 = 0;
  a0[0x37] = 0;
  v4 = (unsigned long)(((int)a0 - (int)(unsigned long *)((unsigned long)&a0[1] & 0xfffffffffffffff8)) + 0x1c0U >> 3);
  v6 = (unsigned long *)((unsigned long)&a0[1] & 0xfffffffffffffff8);
  while (v4) {
    v7 = &v6[1];
    *v6 = 0;
    v4 -= 1;
    v6 = v7;
  }
  a0[3] = 0xffffffffffffffff;
  *(char *)&a0[0x18] = *(char *)(a1 + 0xc0);
  v3 = *(unsigned long *)(a1 + 0x410);
  v5 = 0x4a;
  v6 = (unsigned long *)(a1 + 0x1c0);
  v7 = &a0[0x38];
  while (v5) {
    v2 = &v7[1];
    v1 = &v6[1];
    *v7 = *v6;
    v5 -= 1;
    v6 = v1;
    v7 = v2;
  }
  a0[0x82] = v3;
  v5 = 0xa0;
  v6 = (unsigned long *)(a1 + 0x418);
  v7 = &a0[0x83];
  while (v5) {
    v2 = &v7[1];
    v1 = &v6[1];
    *v7 = *v6;
    v5 -= 1;
    v6 = v1;
    v7 = v2;
  }
  *(unsigned int *)v7 = *(unsigned int *)v6;
}

// Function: sub_144d0 @ 0x144d0
bool sub_144d0(void)
{
  unsigned long v1; // stack - 0x18
  int v2; // stack - 0x1c
  
  v1 = 0;
  return sub_18c40(&v2,0x21080,2,&v1) == 2 && v2 == 0x100;
}

// Function: sub_14540 @ 0x14540
unsigned int sub_14540(unsigned int a0) // early-return x2
{
  int v1; // ebx
  char v2 [3]; // stack - 0x34
  char v3; // stack - 0x31
  
  if ((char)a0)
    return 0;
  v1 = 0;
  do {
    v2[0] = (char)v1;
    v2[2] = v2[0] + '\x01';
    v3 = 0;
    v2[1] = 0;
    if (0 <= strcoll(v2,&v2[2]))
      return a0;
    v1 += 1;
  } while (v1 != 0xff);
  return 1;
}

// Function: sub_145d0 @ 0x145d0
void sub_145d0(long a0)
{
  bool v1;
  unsigned long v10; // r15
  unsigned int v2;
  unsigned long v3; // rax
  unsigned long v4; // rcx
  unsigned long v5;
  unsigned long v6; // stack - 0x48
  char v7; // stack - 0x4d
  unsigned int v8; // stack - 0x4c
  unsigned int v9; // r15d
  
  v10 = 0xffffff80;
  v1 = 2 <= __ctype_get_mb_cur_max();
  *(bool *)a0 = v1;
  *(char *)(a0 + 1) = sub_14540(v1);
  *(char *)(a0 + 2) = sub_144d0();
  do {
    v7 = (char)v10;
    v6 = 0;
    v3 = sub_18c40(&v8,&v7,1,&v6);
    v4 = v10 & 0xff;
    v5 = 1;
    if (v3)
      v5 = v3;
    v2 = 0xffffffff;
    if (v3 <= 1)
      v2 = v8;
    v9 = (int)v10 + 1;
    v10 = (unsigned long)v9;
    *(char *)(a0 + 3 + v4) = (char)v5;
    *(unsigned int *)(a0 + 0x104 + v4 * 4) = v2;
  } while (v9 != 0x80);
}

// Function: sub_146b0 @ 0x146b0
unsigned int sub_146b0(unsigned long a0,unsigned int *a1)
{
  unsigned int v1; // eax
  unsigned int v2; // eax
  unsigned int v3; // ebx
  short *v4; // rbp
  unsigned int v5; // r12d
  unsigned int v6; // stack - 0x44
  
  v1 = towupper(a0);
  v2 = towlower(v1);
  v5 = (unsigned int)a0;
  if (v5 != v1)
    *a1 = v1;
  v6 = (unsigned int)(v5 != v1);
  if (((v1 != v2) && (v5 != v2)) && (towupper(v2) == v1)) {
    a1[(int)v6] = v2;
    v6 += 1;
  }
  v4 = (short *)0x210a2;
  v3 = 0xb5;
  while( true ) {
    if (((v2 != v3 && v1 != v3) && (v5 != v3)) && (towupper(v3) == v1)) {
      a1[(int)v6] = v3;
      v6 += 1;
    }
    if (v4 == (short *)"re") break;
    v3 = (unsigned int)*v4;
    v4 = &v4[1];
  }
  return v6;
}

// Function: sub_147b0 @ 0x147b0
bool sub_147b0(char *a0)
{
  char v1;
  char *v2;
  
  v2 = (char *)sub_1d9a0(a0);
  if (!*v2)
    v2 = a0;
  v1 = v2[sub_1da00(v2)];
  v2[sub_1da00(v2)] = '\0';
  return v1 != '\0';
}

// Function: sub_147f0 @ 0x147f0
void sub_147f0(char *a0)
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

// Function: sub_14830 @ 0x14830
void sub_14830(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long *a3)
{
  (*(void *)*a3)(); // jump-as-call
}

// Function: sub_14840 @ 0x14840
void sub_14840(void *a0)
{
  free(a0); // tail-call
}

// Function: sub_14850 @ 0x14850
void sub_14850(void *a0)
{
  unsigned int *v1;
  long v2; // rbx
  void *v3;
  
  if (!*(int *)((long)a0 + 8)) {
    sub_184e0(*(unsigned long *)((long)a0 + 0x10));
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
          regfree(&v1[2]);
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

// Function: sub_148e0 @ 0x148e0
unsigned int sub_148e0(void)
{
  int v1; // eax
  
  v1 = sub_18cd0();
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}

// Function: sub_14900 @ 0x14900
unsigned int sub_14900(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = strcmp(a0,a1);
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}

// Function: sub_14920 @ 0x14920
unsigned long sub_14920(char *a0,char *a1,unsigned int a2) // early-return
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
    return sub_18cd0(a0,a1); // tail-call
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
  v5 = (char *)sub_1d490(a1);
  v6 = v5;
  while (v6 = strchr(v6,0x2f), v6) {
    *v6 = '\0';
    v2 = sub_18cd0(a0,v5);
    if ((int)v2 <= 0) {
      free(v5); // return-dupe
      return (unsigned long)v2;
    }
    *v6 = '/';
    v6 = &v6[1];
  }
  v2 = sub_18cd0(a0,v5);
  free(v5);
  return (unsigned long)v2;
}

// Function: sub_14a10 @ 0x14a10
void sub_14a10(unsigned long *a0,int a1,unsigned int a2)
{
  unsigned long v1;
  unsigned long *v2; // rax
  void *v3; // rcx
  void *v4; // rdx
  bool v5; // zf
  
  v2 = (unsigned long *)sub_1d3b0(0x28);
  *(int *)&v2[1] = a1;
  *(unsigned int *)((long)v2 + 0xc) = a2;
  if (!a1) {
    v5 = (a2 & 0x10) == 0;
    v3 = sub_148e0;
    if (v5)
      v3 = sub_14900;
    v4 = sub_14aa0;
    if (v5)
      v4 = sub_14a90;
    v2[2] = sub_18320(0,0,v4,v3,sub_14840);
  }
  v1 = *a0;
  *a0 = v2;
  *v2 = v1;
}

// Function: sub_14a90 @ 0x14a90
void sub_14a90(void)
{
  sub_182b0(); // tail-call
}

// Function: sub_14aa0 @ 0x14aa0
unsigned long sub_14aa0(char *a0,unsigned long a1)
{
  unsigned int v1; // eax
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
    sub_18f20(v2);
    if ((v8) && (!v9)) break;
    if (v8) // branch-flip
      v1 = towlower(v9);
    else {
      v1 = (unsigned int)*v6;
    }
    v6 = &v6[v7];
    v3 = (long)(int)v1 + v3 * 0x1f;
  }
  return v3 % a1;
}

// Function: sub_14b70 @ 0x14b70
char * sub_14b70(unsigned int a0,unsigned long a1,char *a2,char *a3)
{
  char *v1; // rax
  
  while( true ) {
    strcpy(a3,a2);
    v1 = (long)sub_180c0(a1,a3);
    while( true ) {
      if (v1) {
        v1 = (char *)0x1;
        return v1;
      }
      if ((!(a0 & 8)) || (v1 = strrchr(a3,0x2f), !v1)) break;
      *v1 = '\0';
      v1 = (long)sub_180c0(a1,a3);
    }
    if ((a0 & 0x40000000) || (v1 = strchr(a2,0x2f), !v1)) break;
    a2 = &v1[1];
  }
  v1 = NULL;
  return v1;
}

// Function: sub_14c20 @ 0x14c20
void sub_14c20(long a0,unsigned long a1)
{
  unsigned long v1;
  unsigned long *v2; // rax
  
  v2 = (unsigned long *)sub_1d0f0(0x10);
  v1 = *(unsigned long *)(a0 + 8);
  v2[1] = a1;
  *v2 = v1;
  *(unsigned long **)(a0 + 8) = v2;
}

// Function: sub_14c50 @ 0x14c50
unsigned long sub_14c50(char *a0,unsigned int a1) // return-dupe x2
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
          if (!(v1 + 0x85U & 0xfd)) goto label_14ce0;
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
label_14ce0:
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

// Function: sub_14d20 @ 0x14d20
void sub_14d20(void)
{
  sub_1d3b0(0x10); // tail-call
}

// Function: sub_14d30 @ 0x14d30
void sub_14d30(unsigned long *a0)
{
  unsigned long *v1;
  unsigned long *v2;
  
  v2 = (unsigned long *)*a0;
  while (v2) {
    v1 = (unsigned long *)*v2;
    sub_14850(v2);
    v2 = v1;
  }
  v2 = (unsigned long *)a0[1];
  while (v2) {
    v1 = (unsigned long *)*v2;
    free((void *)v2[1]);
    free(v2);
    v2 = v1;
  }
  free(a0); // tail-call
}

// Function: sub_14d90 @ 0x14d90
unsigned long sub_14d90(unsigned long a0,char *a1,unsigned int a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // rax
  char v2;
  char v3;
  char *v4;
  unsigned long v5; // r8
  char *v6;
  void *v7;
  
  v7 = sub_14920;
  if (a2 & 0x10000000)
    v7 = dat_29fd0;
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

// Function: sub_14e50 @ 0x14e50
unsigned int sub_14e50(unsigned int *a0,unsigned long a1)
{
  unsigned long v1;
  int v2; // eax
  
  if (!(*a0 & 0x8000000)) {
    v1 = *(unsigned long *)&a0[2];
    return sub_14d90(v1); // tail-call
  }
  v2 = regexec(&a0[2],a1,0,0,0);
  return CONCAT31((undefined3)((unsigned int)v2 >> 8),v2 == 0);
}

// Function: sub_14e90 @ 0x14e90
unsigned long sub_14e90(long a0,long a1,unsigned long a2)
{
  unsigned long v1; // rax
  long v2; // r13
  
  if (a1 <= 0)
    return 0;
  v2 = 0;
  do {
    v1 = sub_14e50(a0,a2);
    if ((char)v1)
      return v1;
    v2 += 1;
    a0 += 0x48;
  } while (v2 != a1);
  return 0;
}

// Function: sub_14ef0 @ 0x14ef0
unsigned int sub_14ef0(long *a0,char *a1)
{
  unsigned int v1;
  long v2;
  long v3;
  void *v4; // rax
  long *v5;
  unsigned int v6; // r12d
  
  if (!(long *)*a0)
    return 0;
  v4 = NULL;
  v5 = (long *)*a0;
  do {
    if (!(int)v5[1]) {
      if (!v4)
        v4 = (void *)sub_1d0f0(strlen(a1) + 1);
      v2 = v5[2];
      v1 = *(unsigned int *)((long)v5 + 0xc);
      if (!sub_14b70(v1,v2,a1,v4)) goto label_14f2b;
      v6 = 0; // return-dupe
      free(v4); // return-dupe
      return (*(unsigned int *)((long)v5 + 0xc) >> 0x1d ^ 1) & 1 ^ v6;
    }
    v2 = v5[4];
    v3 = v5[2];
    if (sub_14e90(v3,v2,a1)) {
      v6 = 0;
      free(v4);
      return (*(unsigned int *)((long)v5 + 0xc) >> 0x1d ^ 1) & 1 ^ v6;
    }
label_14f2b:
    if (!(long *)*v5) {
      v6 = 1;
      free(v4);
      return (*(unsigned int *)((long)v5 + 0xc) >> 0x1d ^ 1) & 1 ^ v6;
    }
    v5 = (long *)*v5;
  } while( true );
}

// Function: sub_14fb0 @ 0x14fb0
void sub_14fb0(long *a0,char *a1,unsigned int a2) // return-dupe x2
{
  unsigned int *v1;
  unsigned int v10; // r14d
  long v11;
  unsigned long v2;
  int v3; // eax
  unsigned long v4; // rax
  void *v5; // rax
  long v6;
  unsigned int *v7; // rbx
  unsigned long v8;
  long v9;
  
  v9 = *a0;
  if ((!(a2 & 0x18000000)) || (!sub_14c50(a1,a2))) {
    if (((!v9) || (*(int *)(v9 + 8))) || ((*(unsigned int *)(v9 + 0xc) ^ a2) & 0x60000018)) {
      sub_14a10(a0,0,a2);
      v9 = *a0;
    }
    v5 = (void *)sub_1d490(a1);
    if ((a2 & 0x10000002) == 0x10000000)
      sub_147f0(v5);
    v2 = *(unsigned long *)(v9 + 0x10);
    if (v5 == (void *)sub_18940(v2,v5))
      return;
    free(v5); // tail-call
    return;
  }
  if ((!v9) || ((*(int *)(v9 + 8) != 1 || ((*(unsigned int *)(v9 + 0xc) ^ a2) & 0x20000000)))) {
    sub_14a10(a0,1,a2);
    v9 = *a0;
  }
  v11 = *(long *)(v9 + 0x20);
  v6 = *(long *)(v9 + 0x10);
  if (v11 == *(long *)(v9 + 0x18)) {
    v6 = sub_1d2b0(v6,v9 + 0x18,1,0xffffffffffffffff,0x48);
    v11 = *(long *)(v9 + 0x20);
    *(long *)(v9 + 0x10) = v6;
  }
  *(long *)(v9 + 0x20) = v11 + 1;
  v1 = (unsigned int *)(v6 + v11 * 0x48);
  *v1 = a2;
  if (!(a2 & 0x8000000)) {
    if (a2 & 0x4000000) {
      a1 = (char *)sub_1d490(a1);
      sub_14c20(a0,a1);
    }
    *(char **)&v1[2] = a1;
    return;
  }
  v10 = (int)a2 >> 3 & 2U | 9;
  if (a2 & 8) { // branch-flip
    v4 = strlen(a1);
    if (0 < (long)v4) {
      do {
        v8 = v4;
        if (a1[v4 - 1] != '/') goto label_15091;
        v4 -= 1;
      } while (v4);
      goto label_150f1;
    }
    v8 = v4;
    if (!v4) goto label_150f1;
label_15091:
    v5 = (void *)sub_1d110(v8 + 7);
    memcpy(v5,a1,v4);
    v7 = (unsigned int *)(v4 + (long)v5);
    *(unsigned short *)&v7[1] = 0x3f29;
    *v7 = 0x2a2e2f28;
    *(char *)((long)v7 + 6) = 0;
    v3 = regcomp(&v1[2],v5,v10);
    free(v5);
  }
  else {
    v3 = regcomp(&v1[2],a1,v10);
  }
  if (!v3)
    return;
  v11 = *(long *)(v9 + 0x20) + -1;
label_150f1:
  *(long *)(v9 + 0x20) = v11;
  return;
}

// Function: sub_15250 @ 0x15250
int sub_15250(void *a0,unsigned long a1,void *a2,unsigned int a3,unsigned char a4,unsigned long a5)
{
  int v1; // eax
  int *v10; // stack - 0x58
  int v11; // stack - 0x50
  long v2;
  unsigned char *v3;
  long v4; // stack - 0x48
  unsigned char *v5; // rbp
  unsigned char *v6; // r14
  long v7; // r15
  unsigned char *v8;
  int v9; // stack - 0x5c
  
  v7 = 0;
  v2 = 0;
  v4 = 0;
  while( true ) {
    v1 = getc(a2);
    if (v1 == -1) break;
    if (v4 == v7)
      v2 = sub_1d2b0(v2,&v4,1,0xffffffffffffffff,1);
    *(char *)(v2 + v7) = (char)v1;
    v7 += 1;
  }
  v9 = ferror(a2);
  v10 = __errno_location();
  v11 = 0;
  if (v9) {
    v9 = *v10;
    v11 = -(unsigned int)(v9 != 0);
  }
  v3 = (unsigned char *)sub_1d180(v2,v7 + 1);
  v3[v7] = a4;
  if (v7) { // branch-flip
    v5 = &v3[(unsigned long)(v3[v7 + -1] != a4) + v7];
    sub_14c20(a1,v3);
    if (v3 < v5) {
      v8 = v3;
      do {
        v6 = &v8[1];
        if (*v8 == a4) {
          v2 = *(long *)__ctype_b_loc();
          if (*(unsigned short *)(v2 + (unsigned long)a4 * 2) & 0x2000) { // branch-flip
            for (; v3 != v8; v8 = &v8[-1]) {
              if (!(*(unsigned char *)(v2 + 1 + (unsigned long)v8[-1] * 2) & 0x20)) goto label_153ed;
            }
          }
          else {
label_153ed:
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
    sub_14c20(a1,v3);
  }
  *v10 = v9;
  return v11;
}

// Function: sub_15440 @ 0x15440
unsigned int sub_15440(unsigned long a0,unsigned long a1,char *a2,unsigned int a3,char a4) // return-dupe
{
  int v1;
  unsigned int v2; // eax
  void *v3; // rax
  int *v4; // rax
  unsigned long v5 [2]; // stack - 0x30
  
  v5[0] = a0;
  if (!strcmp(a2,"-")) {
    v2 = sub_15250(sub_14830,a1,stdin,a3,(int)a4,v5);
    return v2;
  }
  v3 = fopen(a2,"re");
  if (!v3)
    return 0xffffffff;
  v2 = sub_15250(sub_14830,a1,v3,a3,(int)a4,v5);
  v4 = __errno_location();
  v1 = *v4;
  if (fclose(v3))
    return 0xffffffff;
  *v4 = v1;
  return v2;
}

// Function: sub_15520 @ 0x15520
unsigned long sub_15520(long *a0,long *a1)
{
  if (a0[1] != a1[1])
    return 0;
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_15550 @ 0x15550
unsigned long sub_15550(long a0,unsigned long a1)
{
  return *(unsigned long *)(a0 + 8) % a1;
}

// Function: sub_15570 @ 0x15570
unsigned long sub_15570(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_15580 @ 0x15580
unsigned long sub_15580(long *a0,long *a1)
{
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_15590 @ 0x15590
int sub_15590(long *a0,long *a1)
{
  unsigned long v1;
  unsigned long v2;
  
  v1 = *(unsigned long *)(*a1 + 0x78);
  v2 = *(unsigned long *)(*a0 + 0x78);
  return (unsigned int)(v2 >= v1 && v2 != v1) - (unsigned int)(v2 < v1);
}

// Function: sub_155b0 @ 0x155b0
void sub_155b0(long a0,int a1)
{
  unsigned int v1;
  
  v1 = 0;
  if ((unsigned int)(a1 - 1U) <= 0xb)
    v1 = *(unsigned int *)((unsigned long)(unsigned int)(a1 - 1U) * 4 + 0x210e0);
  *(unsigned int *)(a0 + 0x18) = v1;
}

// Function: sub_155d0 @ 0x155d0
long sub_155d0(long *a0)
{
  unsigned long v1; // rax
  unsigned long v2;
  char *v3;
  
  v3 = (char *)*a0;
  if (v3) {
    v2 = 0;
    do {
      v1 = strlen(v3);
      if (v2 < v1)
        v2 = v1;
      v3 = (char *)a0[1];
      a0 = &a0[1];
    } while (v3);
    return v2 + 1;
  }
  return 1;
}

// Function: sub_15620 @ 0x15620
unsigned long sub_15620(long a0,long a1)
{
  void *v1; // rax
  unsigned long v2; // rsi
  
  v2 = a1 + 0x100U + *(unsigned long *)(a0 + 0x30);
  if (CARRY8(a1 + 0x100U,*(unsigned long *)(a0 + 0x30))) {
    free(*(void **)(a0 + 0x20));
    *(unsigned long *)(a0 + 0x20) = 0;
    *__errno_location() = 0x24;
    return 0;
  }
  *(unsigned long *)(a0 + 0x30) = v2;
  v1 = realloc(*(void **)(a0 + 0x20),v2);
  if (v1) {
    *(void **)(a0 + 0x20) = v1;
    return 1;
  }
  free(*(void **)(a0 + 0x20));
  *(unsigned long *)(a0 + 0x20) = 0;
  return 0;
}

// Function: sub_15690 @ 0x15690
void sub_15690(long a0,unsigned char a1)
{
  if (*(short *)(a0 + 0x68) == 0xb) {
    *(unsigned long *)(a0 + 0xa0) = (unsigned long)a1 + 1;
    return;
  }
  abort(); // no-return
}

// Function: sub_156b0 @ 0x156b0
int sub_156b0(long a0,long a1,char a2) // early-return x3
{
  void *v1;
  unsigned long *v2;
  char *v3;
  unsigned int v4; // eax
  int v5;
  int *v6; // rax
  unsigned long v7; // rcx
  unsigned long *v8;
  
  v1 = (void *)(a1 + 0x70);
  if ((*(unsigned int *)(a0 + 0x48) & 2) || (((*(unsigned int *)(a0 + 0x48) & 1 && (!*(long *)(a1 + 0x58))) || (a2)))) {
    v3 = *(char **)(a1 + 0x30);
    v5 = *(int *)(a0 + 0x2c);
    if (fstatat(v5,v3,v1,0) < 0) {
      v6 = __errno_location();
      v5 = *v6;
      if (v5 == 2) {
        v3 = *(char **)(a1 + 0x30);
        v5 = *(int *)(a0 + 0x2c);
        if (0 <= fstatat(v5,v3,v1,0x100)) {
          *v6 = 0;
          return 0xd;
        }
        v5 = *v6;
      }
      goto label_1574b;
    }
  }
  else {
    v3 = *(char **)(a1 + 0x30);
    v5 = *(int *)(a0 + 0x2c);
    if (fstatat(v5,v3,v1,0x100) <= -1) {
      v5 = *__errno_location();
label_1574b:
      *(int *)(a1 + 0x40) = v5;
      v8 = (unsigned long *)(a1 + 0x78U & 0xfffffffffffffff8);
      *(unsigned long *)(a1 + 0x70) = 0;
      *(unsigned long *)(a1 + 0xf8) = 0;
      v7 = (unsigned long)(((int)v1 - (int)v8) + 0x90U >> 3);
      while (v7) {
        v2 = &v8[1];
        *v8 = 0;
        v7 -= 1;
        v8 = v2;
      }
      return 10;
    }
  }
  v4 = *(unsigned int *)(a1 + 0x88) & 0xf000;
  if (v4 != 0x4000) { // branch-flip
    if (v4 == 0xa000)
      return 0xc;
    v5 = (unsigned int)(v4 == 0x8000) * 5 + 3;
  }
  else {
    if (*(char *)(a1 + 0x100) != '.')
      return 1;
    if ((*(char *)(a1 + 0x101)) && ((*(unsigned int *)(a1 + 0x100) & 0xffff00) != 0x2e00))
      return 1;
    v5 = (-(unsigned int)(*(long *)(a1 + 0x58) == 0) & 0xfffffffc) + 5;
  }
  return v5;
}

// Function: sub_15860 @ 0x15860
long sub_15860(long a0,long a1,unsigned long a2)
{
  unsigned long v1;
  long *v2;
  unsigned long v3;
  long *v4;
  long v5; // rax
  long v6; // rcx
  long *v7;
  long v8;
  
  v3 = *(unsigned long *)(a0 + 0x40);
  v4 = *(long **)(a0 + 0x10);
  v7 = v4;
  if (a2 <= *(unsigned long *)(a0 + 0x38)) {
label_158bb:
    while (a1) {
      *v4 = a1;
      v4 = &v4[1];
      a1 = *(long *)(a1 + 0x10);
    }
    qsort(v7,a2,8,v3);
    v4 = *(long **)(a0 + 0x10);
    v8 = *v4;
    v5 = v8;
    v6 = a2 - 1;
    v7 = v4;
    if (v6) {
      while( true ) {
        v2 = &v7[1];
        v7 = &v7[1];
        *(long *)(v8 + 0x10) = *v2;
        v6 -= 1;
        if (!v6) break;
        v8 = *v7;
      }
      v8 = v4[a2 - 1];
    }
    *(unsigned long *)(v8 + 0x10) = 0;
    return v5;
  }
  v1 = a2 + 0x28;
  *(unsigned long *)(a0 + 0x38) = v1;
  if (!(v1 >> 0x3d)) {
    v4 = realloc(v4,v1 * 8);
    if (v4) {
      *(long **)(a0 + 0x10) = v4;
      v7 = v4;
      goto label_158bb;
    }
    v4 = *(long **)(a0 + 0x10);
  }
  free(v4);
  *(unsigned long *)(a0 + 0x10) = 0;
  *(unsigned long *)(a0 + 0x38) = 0;
  return a1;
}

// Function: sub_15970 @ 0x15970
void * sub_15970(long a0,void *a1,unsigned long a2)
{
  unsigned long v1;
  void *v2; // rax
  
  v2 = malloc(a2 + 0x108 & 0xfffffffffffffff8);
  if (v2) {
    memcpy((void *)((long)v2 + 0x100),a1,a2);
    v1 = *(unsigned long *)(a0 + 0x20);
    *(char *)((long)v2 + a2 + 0x100) = 0;
    *(unsigned long *)((long)v2 + 0x60) = a2;
    *(long *)((long)v2 + 0x50) = a0;
    *(unsigned long *)((long)v2 + 0x38) = v1;
    *(unsigned int *)((long)v2 + 0x40) = 0;
    *(unsigned long *)((long)v2 + 0x18) = 0;
    *(unsigned int *)((long)v2 + 0x6a) = 0x30000;
    *(unsigned long *)((long)v2 + 0x20) = 0;
    *(unsigned long *)((long)v2 + 0x28) = 0;
  }
  return v2;
}

// Function: sub_15a10 @ 0x15a10
unsigned long sub_15a10(long a0)
{
  long v1; // rax
  void *v2; // rax
  
  if (*(unsigned int *)(a0 + 0x48) & 0x102) {
    v1 = sub_18320(0x1f,0,sub_15550,sub_15520,dat_29fb8);
    *(long *)(a0 + 0x58) = v1;
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 != 0);
  }
  v2 = malloc(0x20);
  *(void **)(a0 + 0x58) = v2;
  if (v2) {
    sub_1db50(v2);
    return 1;
  }
  return 0;
}

// Function: sub_15a80 @ 0x15a80
void sub_15a80(void *a0)
{
  void *v1;
  
  if (a0) {
    do {
      v1 = *(void **)((long)a0 + 0x10);
      if (*(void **)((long)a0 + 0x18))
        closedir(*(void **)((long)a0 + 0x18));
      free(a0);
      a0 = v1;
    } while (v1);
    return;
  }
}

// Function: sub_15ad0 @ 0x15ad0
void sub_15ad0(unsigned long a0)
{
  int v1; // eax
  
  while( true ) {
    if (sub_18b60(a0)) break;
    v1 = sub_18bb0(a0);
    if (0 <= v1)
      close(v1);
  }
}

// Function: sub_15b10 @ 0x15b10
unsigned long sub_15b10(long a0,int a1)
{
  long v1;
  long v2;
  unsigned long *v3; // rax
  unsigned long *v4; // rax
  unsigned long v5 [2]; // stack - 0xb8
  unsigned long v6 [15]; // stack - 0xa8
  
  v1 = *(long *)(a0 + 0x50);
  v2 = *(long *)(v1 + 0x50);
  if (!(*(unsigned char *)(v1 + 0x49) & 2))
    return 0;
  if (!v2) {
    v2 = sub_18320(0xd,0,sub_15570,sub_15580,dat_29fb8);
    *(long *)(v1 + 0x50) = v2;
    if (!v2) goto label_15b9e;
  }
  v5[0] = *(unsigned long *)(a0 + 0x70);
  v1 = sub_180c0(v2,v5);
  if (v1)
    return *(unsigned long *)(v1 + 8);
label_15b9e:
  if ((0 <= a1) && (!fstatfs(a1,v6))) {
    if ((v2) && (v3 = malloc(0x10), v3)) {
      *v3 = *(unsigned long *)(a0 + 0x70);
      v3[1] = v6[0];
      v4 = (unsigned long *)sub_18940(v2,v3);
      if (v4) { // branch-flip
        if (v3 != v4)
          abort(); // no-return
      }
      else {
        free(v3);
      }
    }
    return v6[0];
  }
  return 0;
}

// Function: sub_15c40 @ 0x15c40
bool sub_15c40(void)
{
  long v1; // rax
  bool v2; // r8b
  
  v1 = sub_15b10();
  v2 = 0;
  if (v1 != 0x9fa0) {
    if (0x9fa1 <= v1) { // branch-flip
      if (v1 != 0x5346414f)
        return v1 != 0xff534d42;
    }
    else if (v1)
      v2 = v1 != 0x6969;
  }
  return v2;
}

// Function: sub_15c90 @ 0x15c90
unsigned long sub_15c90(void)
{
  long v1; // rax
  
  v1 = sub_15b10();
  if ((v1 != 0x1021994) && (v1 != 0xff534d42))
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 != 0x6969);
  return 0;
}

// Function: sub_15cd0 @ 0x15cd0
void sub_15cd0(long a0,long a1)
{
  char *v1;
  void *v2;
  unsigned long v3;
  char *v4; // rax
  unsigned long v5; // rax
  
  v1 = (char *)(a1 + 0x100);
  v2 = *(void **)(a0 + 0x20);
  *(long *)(a1 + 0x48) = *(long *)(a1 + 0x60);
  memmove(v2,v1,*(long *)(a1 + 0x60) + 1);
  v4 = strrchr(v1,0x2f);
  if ((v4) && ((v1 != v4 || (*(char *)(a1 + 0x101))))) {
    v5 = strlen(&v4[1]);
    memmove(v1,&v4[1],v5 + 1);
    *(unsigned long *)(a1 + 0x60) = v5;
  }
  v3 = *(unsigned long *)(a0 + 0x20);
  *(unsigned long *)(a1 + 0x38) = v3;
  *(unsigned long *)(a1 + 0x30) = v3;
}

// Function: sub_15d60 @ 0x15d60
unsigned long sub_15d60(long a0,unsigned long *a1) // return-dupe x2
{
  unsigned long v1;
  unsigned long *v2; // rax
  unsigned long *v3; // rax
  unsigned long v4;
  
  if (!(*(unsigned int *)(a0 + 0x48) & 0x102)) {
    v4 = sub_1db70(*(unsigned long *)(a0 + 0x58),&a1[0xe]);
    if (!(char)v4)
      return 1;
    *a1 = a1;
    *(unsigned short *)&a1[0xd] = 2;
    return v4;
  }
  v2 = malloc(0x18);
  if (!v2)
    return 0;
  v4 = a1[0xe];
  v1 = *(unsigned long *)(a0 + 0x58);
  v2[2] = a1;
  *v2 = v4;
  v2[1] = a1[0xf];
  v3 = (unsigned long *)sub_18940(v1,v2);
  if (v2 == v3)
    return 1;
  free(v2);
  if (v3) {
    v4 = v3[2];
    *(unsigned short *)&a1[0xd] = 2;
    *a1 = v4;
    return 1;
  }
  return 0;
}

// Function: sub_15e10 @ 0x15e10
void sub_15e10(long a0,int a1,char a2)
{
  int v1;
  
  v1 = *(int *)(a0 + 0x2c);
  if ((v1 == a1) && (v1 != -100))
    abort(); // no-return
  if (a2) { // branch-flip
    v1 = sub_18b70(a0 + 0x60);
    if (0 <= v1) {
      close(v1);
      goto label_15e54;
    }
  }
  else if ((!(*(unsigned char *)(a0 + 0x48) & 4)) && (0 <= v1)) {
    close(v1);
label_15e54:
    *(int *)(a0 + 0x2c) = a1;
    return;
  }
  *(int *)(a0 + 0x2c) = a1;
}

// Function: sub_15e70 @ 0x15e70
bool sub_15e70(long a0)
{
  int v1;
  bool v2;
  
  if (*(unsigned int *)(a0 + 0x48) & 4) // branch-flip
    v2 = 0;
  else if (*(unsigned int *)(a0 + 0x48) & 0x200) { // branch-flip
    sub_15e10(a0,0xffffff9c,1);
    v2 = 0;
  }
  else {
    v1 = *(int *)(a0 + 0x28);
    v2 = fchdir(v1) != 0;
  }
  sub_15ad0(a0 + 0x60);
  return v2;
}

// Function: sub_15ee0 @ 0x15ee0
void sub_15ee0(long a0,long a1)
{
  long v1;
  long *v2;
  long v3;
  void *v4; // rax
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  
  if (*(unsigned int *)(a0 + 0x48) & 0x102) { // branch-flip
    v5 = *(unsigned long *)(a1 + 0x70);
    v6 = *(unsigned long *)(a1 + 0x78);
    v4 = (void *)sub_189a0(*(unsigned long *)(a0 + 0x58),&v5);
    if (!v4)
      abort(); // no-return, return-dupe
    free(v4);
  }
  else {
    v1 = *(long *)(a1 + 8);
    if ((v1) && (0 <= *(long *)(v1 + 0x58))) {
      v2 = *(long **)(a0 + 0x58);
      if (!v2[2]) {
        abort();
      }
      if ((*v2 == *(long *)(a1 + 0x78)) && (v2[1] == *(long *)(a1 + 0x70))) {
        v3 = *(long *)(v1 + 0x78);
        v2[1] = *(long *)(v1 + 0x70);
        *v2 = v3;
      }
    }
  }
}

// Function: sub_15fa0 @ 0x15fa0
void sub_15fa0(long a0,long a1,long a2)
{
  long v1;
  
  for (; a0; a0 = *(long *)(a0 + 0x10)) {
    if (*(long *)(a0 + 0x30) != a0 + 0x100)
      *(long *)(a0 + 0x30) = (*(long *)(a0 + 0x30) - *(long *)(a0 + 0x38)) + a1;
    *(long *)(a0 + 0x38) = a1;
  }
  do {
    if (*(long *)(a2 + 0x58) <= -1)
      return;
    while( true ) {
      if (*(long *)(a2 + 0x30) != a2 + 0x100)
        *(long *)(a2 + 0x30) = (*(long *)(a2 + 0x30) - *(long *)(a2 + 0x38)) + a1;
      v1 = *(long *)(a2 + 0x10);
      *(long *)(a2 + 0x38) = a1;
      if (!v1) break;
      a2 = v1;
      if (*(long *)(v1 + 0x58) <= -1)
        return;
    }
    a2 = *(long *)(a2 + 8);
  } while( true );
}

// Function: sub_16020 @ 0x16020
void sub_16020(unsigned long a0,unsigned int a1,unsigned long a2)
{
  if (a1 & 0x200) {
    sub_19570(a0,a2); // tail-call
    return;
  }
  sub_1de60(a2,(a1 & 0x10) << 0xd | 0x90900); // tail-call
}

// Function: sub_16060 @ 0x16060
int sub_16060(long a0,long a1,int a2,char *a3) // return-dupe
{
  int v1;
  int v2;
  int v3;
  int *v4; // rax
  long v5; // stack - 0xd8
  unsigned int v6;
  unsigned char v7; // stack - 0xdd
  long v8; // stack - 0xd0
  
  v1 = a2;
  if ((a3) && (!strcmp(a3,".."))) {
    v6 = *(unsigned int *)(a0 + 0x48);
    if (v6 & 4) {
label_161f0:
      if ((v6 & 0x200) && (0 <= a2)) {
        close(a2);
        return 0;
      }
      return 0;
    }
    if (a2 <= -1) {
      if (!(v6 & 0x200)) {
label_160d7:
        v7 = 1;
        goto label_160dc;
      }
      v7 = sub_18b60(a0 + 0x60);
      if (v7) goto label_160dc;
      v3 = sub_18bb0(a0 + 0x60);
      v6 = *(unsigned int *)(a0 + 0x48);
      if (v3 < 0) goto label_160d7;
      v7 = 1;
      a2 = v3;
      v1 = v3;
      if (v6 & 2) goto label_16108;
      goto label_16140;
    }
    v7 = 1;
    if (!(v6 & 2)) goto label_161cd;
label_16108:
    v3 = a2;
    if (!fstat(v1,&v5)) {
      if ((*(long *)(a1 + 0x70) == v5) && (*(long *)(a1 + 0x78) == v8)) {
        v6 = *(unsigned int *)(a0 + 0x48);
        goto label_16140;
      }
      *__errno_location() = 2;
    }
    v2 = -1;
  }
  else {
    v6 = *(unsigned int *)(a0 + 0x48);
    if (v6 & 4) goto label_161f0;
    if (0 <= a2) // branch-flip
      v7 = 0;
    else {
      v7 = 0;
label_160dc:
      v1 = sub_16020(*(unsigned int *)(a0 + 0x2c),v6,a3);
      if (v1 < 0)
        return -1;
      v6 = *(unsigned int *)(a0 + 0x48);
    }
    if (v6 & 2) goto label_16108;
    v3 = a2;
    if (a3) {
label_161cd:
      v3 = a2;
      if (!strcmp(a3,"..")) goto label_16108;
    }
label_16140:
    if (v6 & 0x200) {
      sub_15e10(a0,v1,v7 ^ 1);
      return 0;
    }
    v2 = fchdir(v1);
  }
  if (0 <= v3)
    return v2;
  v4 = __errno_location();
  v3 = *v4;
  close(v1);
  *v4 = v3;
  return v2;
}

// Function: sub_16310 @ 0x16310
void * sub_16310(long *a0,int a1) // return-dupe
{
  long v1;
  unsigned long v10;
  void *v11; // rax
  long v12;
  unsigned int v13;
  int v14; // stack - 0x44
  char *v15;
  unsigned int v16;
  void *v17;
  unsigned long v18;
  unsigned long v19;
  void *v2;
  bool v20;
  unsigned long v21; // stack - 0xa0
  void *v22; // stack - 0x98
  void *v23; // stack - 0x90
  char *v24; // stack - 0x70
  unsigned long v25; // stack - 0x68
  unsigned long v26; // stack - 0x60
  char v27; // stack - 0x4b
  char v28; // stack - 0x4a
  long v3;
  bool v4;
  char v5; // al
  int v6; // eax
  int v7;
  int *v8; // rax
  unsigned long *v9; // rax
  
  v1 = *a0;
  v2 = *(void **)(v1 + 0x18);
  if (v2) { // branch-flip
    v14 = dirfd(v2);
    if (v14 < 0) {
      closedir(*(void **)(v1 + 0x18));
      *(unsigned long *)(v1 + 0x18) = 0;
      if (a1 == 3) {
        *(unsigned short *)(v1 + 0x68) = 4;
        *(int *)(v1 + 0x40) = *__errno_location();
        return NULL;
      }
      return NULL;
    }
    if (a0[8]) { // branch-flip
      v25 = 0xffffffffffffffff;
label_16372:
      v20 = 1;
      v13 = *(unsigned int *)&a0[9];
    }
    else {
      v25 = 100000;
      v13 = *(unsigned int *)&a0[9];
      v20 = 1;
    }
  }
  else {
    v13 = *(unsigned int *)&a0[9];
    v7 = 0;
    if ((v13 & 0x10) && (v7 = 0x20000, v13 & 1))
      v7 = (unsigned int)(*(long *)(v1 + 0x58) != 0) << 0x11;
    v16 = 0xffffff9c;
    if ((v13 & 0x204) == 0x200)
      v16 = *(unsigned int *)((long)a0 + 0x2c);
    v12 = sub_195f0(v16,*(unsigned long *)(v1 + 0x30),v7,&v14);
    *(long *)(v1 + 0x18) = v12;
    if (!v12) {
      if (a1 == 3) {
        *(unsigned short *)(v1 + 0x68) = 4;
        *(int *)(v1 + 0x40) = *__errno_location();
        return NULL;
      }
      return NULL;
    }
    if (*(short *)(v1 + 0x68) != 0xb) { // branch-flip
      if (*(unsigned char *)((long)a0 + 0x49) & 1) {
        sub_15ee0(a0,v1);
        sub_156b0(a0,v1,0);
        if (!sub_15d60(a0,v1)) {
          *__errno_location() = 0xc;
          return NULL;
        }
      }
    }
    else {
      *(unsigned short *)(v1 + 0x68) = sub_156b0(a0,v1,0);
    }
    v25 = (unsigned long)(-(unsigned int)(a0[8] == 0) & 0x186a1) - 1;
    if (a1 == 2) {
label_16a20:
      v20 = 0;
      v13 = *(unsigned int *)&a0[9];
      goto label_1637b;
    }
    v7 = v14;
    if (((*(unsigned int *)&a0[9] & 0x38) != 0x18) || (*(long *)(v1 + 0x80) != 2)) {
label_16757:
      v20 = a1 == 3;
      v4 = 1;
    }
    else {
      v6 = sub_15c40(v1,v14);
      v7 = v14;
      if (!v6) goto label_16757;
      if (a1 != 3) goto label_16a20;
      v4 = 0;
      v20 = 1;
    }
    if (*(unsigned char *)((long)a0 + 0x49) & 2) {
      v7 = sub_1dd10(v7,0x406,3);
      v14 = v7;
    }
    if ((0 <= v7) && (!sub_16060(a0,v1,v7,0))) goto label_16372;
    if ((v20) && (v4))
      *(int *)(v1 + 0x40) = *__errno_location();
    *(unsigned short *)(v1 + 0x6a) = *(unsigned short *)(v1 + 0x6a) | 1;
    closedir(*(void **)(v1 + 0x18));
    v13 = *(unsigned int *)&a0[9];
    *(unsigned long *)(v1 + 0x18) = 0;
    if ((v13 & 0x200) && (0 <= v14)) {
      close(v14);
      v13 = *(unsigned int *)&a0[9];
    }
    *(unsigned long *)(v1 + 0x18) = 0;
    v20 = 0;
  }
label_1637b:
  v19 = *(unsigned long *)(v1 + 0x48);
  v21 = v19;
  v26 = v19 - 1;
  if (*(char *)(*(long *)(v1 + 0x38) + -1 + v19) != '/') {
    v21 = v19 + 1;
    v26 = v19;
  }
  v24 = NULL;
  if (v13 & 4) {
    v24 = &((char *)(v26 + a0[4]))[1];
    *(char *)(v26 + a0[4]) = 0x2f;
  }
  v17 = *(void **)(v1 + 0x18);
  v12 = *(long *)(v1 + 0x58);
  v19 = a0[6] - v21;
  if (v17) { // branch-flip
    v8 = __errno_location();
    v18 = 0;
    v28 = '\0';
    v27 = '\0';
    v22 = NULL;
    v23 = NULL;
    do {
      while( true ) {
        *v8 = 0;
        v9 = readdir(v17);
        if (!v9) {
          if (*v8) {
            *(int *)(v1 + 0x40) = *v8;
            *(unsigned short *)(v1 + 0x68) = (-(unsigned short)(!v2 && !v18) & 0xfffd) + 7;
          }
          if (*(void **)(v1 + 0x18)) {
            closedir(*(void **)(v1 + 0x18));
            *(unsigned long *)(v1 + 0x18) = 0;
          }
          goto label_168d4;
        }
        if (((*(unsigned char *)&a0[9] & 0x20) || (*(char *)((long)v9 + 0x13) != '.')) || ((*(char *)((long)v9 + 0x14) && (*(short *)((long)v9 + 0x14) != 0x2e)))) break;
label_16443:
        v17 = *(void **)(v1 + 0x18);
        if (!v17) goto label_16590;
      }
      v10 = strlen((char *)((long)v9 + 0x13));
      v11 = (void *)sub_15970(a0,(char *)((long)v9 + 0x13),v10);
      if (!v11) {
label_167d0:
        v7 = *v8;
        free(v11);
        sub_15a80(v23);
        closedir(*(void **)(v1 + 0x18));
        *(unsigned long *)(v1 + 0x18) = 0;
        *(unsigned short *)(v1 + 0x68) = 7;
        *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
        *v8 = v7;
        return NULL;
      }
      if (v19 <= v10) {
        v3 = a0[4];
        v5 = sub_15620(a0,v26 + 2 + v10);
        if (!v5) goto label_167d0;
        if (a0[4] != v3) {
          v15 = (char *)(a0[4] + v21);
          if (!(*(unsigned char *)&a0[9] & 4))
            v15 = v24;
          v27 = v5;
          v24 = v15;
        }
        v19 = a0[6] - v21;
      }
      if (CARRY8(v10,v21)) {
        free(v11);
        sub_15a80(v23);
        closedir(*(void **)(v1 + 0x18));
        *(unsigned long *)(v1 + 0x18) = 0;
        *(unsigned short *)(v1 + 0x68) = 7;
        *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
        *v8 = 0x24;
        return NULL;
      }
      *(long *)((long)v11 + 0x58) = v12 + 1;
      v3 = *a0;
      *(unsigned long *)((long)v11 + 0x48) = v10 + v21;
      *(long *)((long)v11 + 8) = v3;
      *(unsigned long *)((long)v11 + 0x78) = *v9;
      v13 = *(unsigned int *)&a0[9];
      if (v13 & 4) { // branch-flip
        *(unsigned long *)((long)v11 + 0x30) = *(unsigned long *)((long)v11 + 0x38);
        memmove(v24,(void *)((long)v11 + 0x100),*(long *)((long)v11 + 0x60) + 1);
        v13 = *(unsigned int *)&a0[9];
      }
      else {
        *(void **)((long)v11 + 0x30) = (void *)((long)v11 + 0x100);
      }
      if ((a0[8]) && (!(v13 & 0x400)))
        *(unsigned short *)((long)v11 + 0x68) = sub_156b0(a0,v11,0);
      else {
        v10 = 1;
        if ((v13 & 8) && (*(unsigned char *)((long)v9 + 0x12) & 0xfb))
          v10 = (unsigned long)((v13 >> 4 & 1 | (unsigned int)(*(unsigned char *)((long)v9 + 0x12) != 10)) ^ 1);
        *(unsigned short *)((long)v11 + 0x68) = 0xb;
        sub_155b0((long)v11 + 0x70);
        sub_15690(v11,v10 & 0xffffffff);
      }
      *(unsigned long *)((long)v11 + 0x10) = 0;
      if (v23) // branch-flip
        *(void **)((long)v22 + 0x10) = v11;
      else {
        v23 = v11;
      }
      v10 = v18 + 1;
      if (v18 != 10000) {
        v18 = v10;
        if (v10 < v25) {
          v22 = v11;
          goto label_16443;
        }
        goto label_168d4;
      }
      if (!a0[8])
        v28 = sub_15c90(v1,v14);
      v18 = 0x2711;
      v17 = *(void **)(v1 + 0x18);
      v22 = v11;
    } while (v17);
label_16590:
label_168d4:
    if (v27)
      sub_15fa0(a0[1],a0[4],v23);
    if (*(unsigned char *)&a0[9] & 4) {
      if ((a0[6] == v21) || (!v18)) goto label_16904;
label_1690a:
      *v24 = 0;
    }
    if ((v2) || (!v20)) {
label_169b0:
      if (!v18) goto label_169b5;
    }
    else if ((a1 == 1) || (!v18)) {
label_16990:
      if (*(long *)(v1 + 0x58)) // branch-flip
        v7 = sub_16060(a0,*(unsigned long *)(v1 + 8),0xffffffff,"..");
      else {
        v7 = sub_15e70(a0);
      }
      if (v7) {
        *(unsigned short *)(v1 + 0x68) = 7;
        *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
        sub_15a80(v23);
        return NULL;
      }
      goto label_169b0;
    }
    if (v28) { // branch-flip
      a0[8] = (long)sub_15590;
      v23 = (void *)sub_15860(a0,v23,v18);
      a0[8] = 0;
    }
    else if ((a0[8]) && (v18 != 1))
      v23 = (void *)sub_15860(a0,v23,v18);
  }
  else {
    if (*(unsigned char *)&a0[9] & 4) {
      v23 = NULL;
      v18 = 0;
      v28 = '\0';
label_16904:
      v24 = &v24[-1];
      goto label_1690a;
    }
    v23 = NULL;
    if ((bool)(v20 & v2 == NULL)) {
      v28 = '\0';
      v18 = 0;
      goto label_16990;
    }
label_169b5:
    if (((a1 == 3) && (*(short *)(v1 + 0x68) != 4)) && (*(short *)(v1 + 0x68) != 7))
      *(unsigned short *)(v1 + 0x68) = 6;
    sub_15a80(v23);
    v23 = NULL;
  }
  return v23;
}

// Function: sub_16c60 @ 0x16c60
void sub_16c60(unsigned int a0,void *a1)
{
  if (!(a0 & 0x102)) {
    free(a1); // tail-call
    return;
  }
  if (a1) {
    sub_184e0(a1); // tail-call
    return;
  }
}

// Function: sub_16c90 @ 0x16c90
long * sub_16c90(long *a0,unsigned int a1,long a2)
{
  unsigned int v1;
  char *v10;
  void *v11; // stack - 0x50
  unsigned char v12; // stack - 0x42
  long v13; // stack - 0x40
  int v2; // eax
  long *v3; // rax
  unsigned long v4; // rax
  long v5;
  unsigned long v6;
  long v7;
  long v8;
  long v9; // r13
  
  if (((a1 & 0xfffff000) || ((a1 & 0x204) == 0x204)) || (!(a1 & 0x12))) {
    *__errno_location() = 0x16;
    return NULL;
  }
  v3 = calloc(1,0x80);
  if (!v3)
    return NULL;
  v3[8] = a2;
  *(unsigned int *)((long)v3 + 0x2c) = 0xffffff9c;
  v1 = a1 & 0xfffffdff | 4;
  if (!(a1 & 2))
    v1 = a1;
  *(unsigned int *)&v3[9] = v1;
  v4 = sub_155d0(a0);
  v6 = 0x1000;
  if (0x1000 <= v4)
    v6 = v4;
  v12 = sub_15620(v3,v6);
  if (!v12) {
    free(v3); // return-dupe
    return NULL;
  }
  if (*a0) { // branch-flip
    v11 = (void *)sub_15970(v3,"",0);
    if (v11) {
      *(unsigned long *)((long)v11 + 0x58) = 0xffffffffffffffff;
      v10 = (char *)*a0;
      if (a2) goto label_16d79;
label_16d88:
      if (!v10) goto label_16fcd;
      v13 = 0;
      v8 = 0;
      v7 = 0;
label_16e2f:
      do {
        v9 = v7;
        v7 = v13;
        v6 = strlen(v10);
        if (((3 <= v6) && (((unsigned char)(a1 >> 0xb) ^ 1) & 1)) && (v10[v6 - 1] == '/')) {
          do {
            if (v10[v6 - 2] != '/') break;
            v6 -= 1;
          } while (v6 != 1);
        }
        v5 = sub_15970(v3,v10,v6);
        if (!v5) goto label_16ea0;
        *(unsigned long *)(v5 + 0x58) = 0;
        *(void **)(v5 + 8) = v11;
        *(long *)(v5 + 0x30) = v5 + 0x100;
        if ((v8) && (v12)) {
          *(unsigned short *)(v5 + 0x68) = 0xb;
          sub_15690(v5,1);
          if (a2) { // branch-flip
label_16e17:
            *(long *)(v5 + 0x10) = v8;
          }
          else {
            *(unsigned long *)(v5 + 0x10) = 0;
label_16fa8:
            v13 = v5;
            *(long *)(v7 + 0x10) = v5;
            v5 = v8;
          }
          v10 = (char *)a0[v9 + 1];
          v8 = v5;
          v7 = v9 + 1;
          if (!v10) break;
          goto label_16e2f;
        }
        *(unsigned short *)(v5 + 0x68) = sub_156b0(v3,v5,0);
        if (a2) goto label_16e17;
        *(unsigned long *)(v5 + 0x10) = 0;
        if (v8) goto label_16fa8;
        v10 = (char *)a0[v9 + 1];
        v8 = v5;
        v7 = v9 + 1;
        v13 = v5;
      } while (v10);
      if ((a2) && (2 <= v9 + 1U))
        v5 = sub_15860(v3,v5,v9 + 1U);
      goto label_16f34;
    }
  }
  else {
    v11 = NULL;
    if (a2) {
      v10 = NULL;
label_16d79:
      v12 = (unsigned char)(*(unsigned int *)&v3[9] >> 10) & 1;
      goto label_16d88;
    }
label_16fcd:
    v5 = 0;
label_16f34:
    v7 = sub_15970(v3,"",0);
    *v3 = v7;
    v8 = v5;
    if (v7) {
      *(long *)(v7 + 0x10) = v5;
      *(unsigned short *)(v7 + 0x68) = 9;
      *(unsigned long *)(v7 + 0x58) = 1;
      if (sub_15a10(v3)) {
        if (!(*(unsigned int *)&v3[9] & 0x204)) {
          v2 = sub_16020(*(unsigned int *)((long)v3 + 0x2c),*(unsigned int *)&v3[9],".");
          *(int *)&v3[5] = v2;
          if (v2 <= -1)
            *(unsigned int *)&v3[9] = *(unsigned int *)&v3[9] | 4;
        }
        sub_18b40(&v3[0xc],0xffffffff);
        return v3;
      }
    }
label_16ea0:
    sub_15a80(v8);
    free(v11);
  }
  free((void *)v3[4]);
  free(v3);
  return NULL;
}

// Function: sub_17000 @ 0x17000
unsigned int sub_17000(long *a0) // return-dupe
{
  long *v1;
  void *v2;
  int *v3; // rax
  void *v4;
  long v5;
  int v6;
  
  v4 = (void *)*a0;
  if (v4) {
    if (*(long *)((long)v4 + 0x58) > -1) {
      do {
        v2 = *(void **)((long)v4 + 0x10);
        if (v2) { // branch-flip
          free(v4);
          v5 = *(long *)((long)v2 + 0x58);
          v4 = v2;
        }
        else {
          v2 = *(void **)((long)v4 + 8);
          free(v4);
          v5 = *(long *)((long)v2 + 0x58);
          v4 = v2;
        }
      } while (v5 >= 0);
    }
    free(v4);
  }
  if (a0[1])
    sub_15a80(a0[1]);
  v1 = &a0[0xc];
  free((void *)a0[2]);
  free((void *)a0[4]);
  if (*(unsigned int *)&a0[9] & 0x200) { // branch-flip
    v6 = *(int *)((long)a0 + 0x2c);
    if ((v6 <= -1) || (!close(v6))) goto label_17097;
    v6 = *__errno_location();
label_1714d:
    sub_15ad0(v1);
    v5 = a0[10];
    if (v5) goto label_170ac;
  }
  else {
    if (!(*(unsigned int *)&a0[9] & 4)) {
      v5 = a0[5];
      if (fchdir((int)v5)) {
        v3 = __errno_location();
        v5 = a0[5];
        v6 = *v3;
        if (!close((int)v5)) goto label_1714d;
        if (!v6) goto label_17108;
        sub_15ad0(v1);
        v5 = a0[10];
        if (!v5) {
          sub_16c60((int)a0[9],a0[0xb]);
          free(a0);
          goto label_170d1;
        }
        goto label_170ac;
      }
      v5 = a0[5];
      if (close((int)v5)) {
        v3 = __errno_location();
label_17108:
        v6 = *v3;
        goto label_1714d;
      }
    }
label_17097:
    v6 = 0;
    sub_15ad0(v1);
    v5 = a0[10];
    if (!v5) {
      sub_16c60((int)a0[9],a0[0xb]);
      free(a0);
      return 0;
    }
label_170ac:
    sub_184e0(v5);
  }
  sub_16c60((int)a0[9],a0[0xb]);
  free(a0);
  if (!v6)
    return 0;
  v3 = __errno_location();
label_170d1:
  *v3 = v6;
  return 0xffffffff;
}

// Function: sub_171f0 @ 0x171f0
void * sub_171f0(long *a0)
{
  short v1;
  unsigned int v2;
  unsigned long v3;
  long v4;
  short v5;
  int v6;
  void *v7;
  void *v8;
  long v9;
  
  v7 = (void *)*a0;
  if (!v7)
    return NULL;
  v2 = *(unsigned int *)&a0[9];
  if (v2 & 0x2000)
    return NULL;
  v5 = *(short *)((long)v7 + 0x6c);
  *(unsigned short *)((long)v7 + 0x6c) = 3;
  if (v5 == 1) {
    *(unsigned short *)((long)v7 + 0x68) = sub_156b0(a0,v7,0);
    return v7;
  }
  v1 = *(short *)((long)v7 + 0x68);
  if (v5 != 2) { // branch-flip
    if (v1 != 1) { // branch-flip
label_17280:
      do {
        v8 = *(void **)((long)v7 + 0x10);
        if (!v8) {
          v8 = *(void **)((long)v7 + 8);
          if (*(long *)((long)v8 + 0x18)) {
            v4 = *(long *)((long)v8 + 0x48);
            *a0 = (long)v8;
            *(char *)(a0[4] + v4) = 0;
            v8 = (void *)sub_16310(a0,3);
            if (v8) {
              free(v7);
              goto label_174c2;
            }
            if (*(unsigned char *)((long)a0 + 0x49) & 0x20)
              return NULL;
            v8 = *(void **)((long)v7 + 8);
          }
          *a0 = (long)v8;
          free(v7);
          if (*(long *)((long)v8 + 0x58) == -1) {
            free(v8);
            *__errno_location() = 0;
            *a0 = 0;
            return NULL;
          }
          if (*(short *)((long)v8 + 0x68) != 0xb) {
            *(char *)(a0[4] + *(long *)((long)v8 + 0x48)) = 0;
            if (*(long *)((long)v8 + 0x58)) { // branch-flip
              if (*(unsigned short *)((long)v8 + 0x6a) & 2) {
                v6 = *(int *)((long)v8 + 0x44);
                if (!(*(unsigned int *)&a0[9] & 4)) {
                  if (*(unsigned int *)&a0[9] & 0x200) { // branch-flip
                    sub_15e10(a0,v6,1);
                    v6 = *(int *)((long)v8 + 0x44);
                  }
                  else {
                    if (fchdir(v6)) {
                      *(int *)((long)v8 + 0x40) = *__errno_location();
                      *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
                    }
                    v6 = *(int *)((long)v8 + 0x44);
                  }
                }
                close(v6);
                goto label_172eb;
              }
              if (*(unsigned short *)((long)v8 + 0x6a) & 1) goto label_172eb;
              v6 = sub_16060(a0,*(unsigned long *)((long)v8 + 8),0xffffffff,"..");
            }
            else {
              v6 = sub_15e70(a0);
            }
            if (v6) {
              *(int *)((long)v8 + 0x40) = *__errno_location();
              *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
            }
label_172eb:
            if (*(short *)((long)v8 + 0x68) != 2) {
              if (*(int *)((long)v8 + 0x40)) // branch-flip
                *(unsigned short *)((long)v8 + 0x68) = 7;
              else {
                *(unsigned short *)((long)v8 + 0x68) = 6;
                sub_15ee0(a0,v8);
              }
            }
            if (!(*(unsigned char *)((long)a0 + 0x49) & 0x20))
              return v8;
            return NULL;
          }
          abort(); // no-return, return-dupe
        }
        *a0 = (long)v8;
        free(v7);
        v7 = v8;
        if (!*(long *)((long)v8 + 0x58)) {
          if (sub_15e70(a0)) {
            *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
            return NULL;
          }
          sub_16c60((int)a0[9],a0[0xb]);
          sub_15cd0(a0,v8);
          sub_15a10(a0);
          v5 = *(short *)((long)v8 + 0x68);
          goto label_17465;
        }
      } while (*(short *)((long)v8 + 0x6c) == 4);
      if (*(short *)((long)v8 + 0x6c) == 2) {
        v5 = sub_156b0(a0,v8,1);
        *(short *)((long)v8 + 0x68) = v5;
        if ((v5 == 1) && (!(*(unsigned int *)&a0[9] & 4))) {
          v6 = sub_16020(*(unsigned int *)((long)a0 + 0x2c),*(unsigned int *)&a0[9],".");
          *(int *)((long)v8 + 0x44) = v6;
          if (0 <= v6) // branch-flip
            *(unsigned short *)((long)v8 + 0x6a) = *(unsigned short *)((long)v8 + 0x6a) | 2;
          else {
            v6 = *__errno_location();
            *(unsigned short *)((long)v8 + 0x68) = 7;
            *(int *)((long)v8 + 0x40) = v6;
          }
        }
        *(unsigned short *)((long)v8 + 0x6c) = 3;
      }
    }
    else {
      if (v5 == 4) goto label_173ea;
label_1734f:
      if ((v2 & 0x40) && (*(long *)((long)v7 + 0x70) != a0[3])) {
label_173ea:
        if (*(unsigned char *)((long)v7 + 0x6a) & 2)
          close(*(int *)((long)v7 + 0x44));
        if (a0[1]) {
          sub_15a80(a0[1]);
          a0[1] = 0;
        }
        *(unsigned short *)((long)v7 + 0x68) = 6;
        sub_15ee0(a0,v7);
        return v7;
      }
      if (a0[1]) { // branch-flip
        if (v2 & 0x1000) {
          *(unsigned int *)&a0[9] = v2 & 0xffffefff;
          sub_15a80(a0[1]);
          a0[1] = 0;
          goto label_17560;
        }
        v3 = *(unsigned long *)((long)v7 + 0x30);
        if (sub_16060(a0,v7,0xffffffff,v3)) { // branch-flip
          v6 = *__errno_location();
          *(unsigned short *)((long)v7 + 0x6a) = *(unsigned short *)((long)v7 + 0x6a) | 1;
          *(int *)((long)v7 + 0x40) = v6;
          v8 = (void *)a0[1];
          if (v8) {
            v7 = v8;
            do {
              *(unsigned long *)((long)v7 + 0x30) = *(unsigned long *)(*(long *)((long)v7 + 8) + 0x30);
              v7 = *(void **)((long)v7 + 0x10);
            } while (v7);
          }
        }
        else {
          v8 = (void *)a0[1];
        }
      }
      else {
label_17560:
        v8 = (void *)sub_16310(a0,3);
        a0[1] = (long)v8;
        if (!v8) {
          if (!(*(unsigned char *)((long)a0 + 0x49) & 0x20)) {
            if ((*(int *)((long)v7 + 0x40)) && (*(short *)((long)v7 + 0x68) != 4))
              *(unsigned short *)((long)v7 + 0x68) = 7;
            sub_15ee0(a0,v7);
            return v7;
          }
          return NULL;
        }
      }
      a0[1] = 0;
    }
label_174c2:
    v4 = *(long *)(*(long *)((long)v8 + 8) + 0x48);
    v9 = v4 + -1;
    if (*(char *)(*(long *)(*(long *)((long)v8 + 8) + 0x38) + -1 + v4) != '/')
      v9 = v4;
    v4 = a0[4];
    *(char *)(v9 + v4) = 0x2f;
    memmove(&((char *)(v9 + v4))[1],(void *)((long)v8 + 0x100),*(long *)((long)v8 + 0x60) + 1);
    v5 = *(short *)((long)v8 + 0x68);
    *a0 = (long)v8;
    v7 = v8;
label_1746d:
    if (v5 == 0xb) goto label_17511;
  }
  else {
    if (2 <= (unsigned short)(v1 - 0xcU)) {
      if (v1 == 1) goto label_1734f;
      goto label_17280;
    }
    v5 = sub_156b0(a0,v7,1);
    *(short *)((long)v7 + 0x68) = v5;
    if (v5 == 1) {
      if (*(unsigned int *)&a0[9] & 4) {
        *a0 = (long)v7;
        goto label_1747d;
      }
      v6 = sub_16020(*(unsigned int *)((long)a0 + 0x2c),*(unsigned int *)&a0[9],".");
      *(int *)((long)v7 + 0x44) = v6;
      if (v6 < 0) {
        v6 = *__errno_location();
        *(unsigned short *)((long)v7 + 0x68) = 7;
        *(int *)((long)v7 + 0x40) = v6;
        *a0 = (long)v7;
        return v7;
      }
      *(unsigned short *)((long)v7 + 0x6a) = *(unsigned short *)((long)v7 + 0x6a) | 2;
      v5 = *(short *)((long)v7 + 0x68);
label_17465:
      *a0 = (long)v7;
      goto label_1746d;
    }
    *a0 = (long)v7;
    if (v5 != 0xb)
      return v7;
label_17511:
    if (*(long *)((long)v7 + 0xa0) != 2) {
      if (*(long *)((long)v7 + 0xa0) == 1)
        return v7;
      abort();
    }
    v5 = sub_156b0(a0,v7,0);
    *(short *)((long)v7 + 0x68) = v5;
  }
  if (v5 != 1)
    return v7;
label_1747d:
  if (!*(long *)((long)v7 + 0x58))
    a0[3] = *(long *)((long)v7 + 0x70);
  if (!sub_15d60(a0,v7)) {
    v7 = NULL;
    *__errno_location() = 0xc;
  }
  return v7;
}

// Function: sub_17830 @ 0x17830
unsigned long sub_17830(unsigned long a0,long a1,unsigned int a2)
{
  if (a2 <= 4) {
    *(short *)(a1 + 0x6c) = (short)a2;
    return 0;
  }
  *__errno_location() = 0x16;
  return 1;
}

// Function: sub_17860 @ 0x17860
int * sub_17860(long *a0,unsigned int a1) // early-return x2
{
  long v1;
  int v2;
  int *v3;
  int v4;
  int *v5; // rax
  unsigned long v6; // r14
  
  v5 = __errno_location();
  v3 = v5;
  if (a1 & 0xffffefff) {
    *v5 = 0x16;
    v5 = NULL;
    return v5;
  }
  v1 = *a0;
  *v5 = 0;
  if (*(unsigned char *)((long)a0 + 0x49) & 0x20) {
    v5 = NULL;
    return v5;
  }
  if (*(short *)(v1 + 0x68) == 9) {
    v5 = (int *)*(long *)(v1 + 0x10);
    return v5;
  }
  if (*(short *)(v1 + 0x68) != 1) {
    v5 = NULL;
    return v5;
  }
  if (a0[1])
    sub_15a80(a0[1]);
  v6 = 1;
  if (a1 == 0x1000) {
    *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x1000;
    v6 = 2;
  }
  if (((*(long *)(v1 + 0x58)) || (**(char **)(v1 + 0x30) == '/')) || (*(unsigned int *)&a0[9] & 4)) {
    v5 = (long)sub_16310(a0,v6);
    a0[1] = (long)v5;
  }
  else {
    v4 = sub_16020(*(unsigned int *)((long)a0 + 0x2c),*(unsigned int *)&a0[9],".");
    v2 = v4;
    if (v4 < 0) {
      a0[1] = 0;
      v5 = NULL;
      return v5;
    }
    v5 = (long)sub_16310(a0,v6);
    a0[1] = (long)v5;
    if (*(unsigned char *)((long)a0 + 0x49) & 2) // branch-flip
      sub_15e10(a0,v2,1);
    else {
      v4 = fchdir(v2);
      if (v4) {
        v4 = *v3;
        close(v2);
        *v3 = v4;
        v5 = NULL;
        return v5;
      }
      close(v2);
    }
    v5 = (long)a0[1];
  }
  return v5;
}

// Function: sub_179e0 @ 0x179e0
unsigned long sub_179e0(void)
{
  return __progname;
}

// Function: sub_179f0 @ 0x179f0
unsigned long sub_179f0(unsigned long a0)
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

// Function: sub_17a60 @ 0x17a60
unsigned long sub_17a60(unsigned long a0)
{
  char v1; // al
  unsigned long v2; // rax
  unsigned long v3;
  
  v2 = 10;
  if (10 <= a0)
    v2 = a0;
  v3 = v2 | 1;
  while ((v3 != 0xffffffffffffffff && (v1 = sub_179f0(v3), !v1))) {
    v3 += 2;
  }
  v2 = v3;
  return v2;
}

// Function: sub_17aa0 @ 0x17aa0
unsigned long sub_17aa0(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_1da80(a0,3) % a1;
}

// Function: sub_17ac0 @ 0x17ac0
bool sub_17ac0(long a0,long a1)
{
  return a1 == a0;
}

// Function: sub_17ad0 @ 0x17ad0
unsigned long sub_17ad0(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0x21180)
    return 1;
  v1 = v2[2];
  if ((((dat_21194 < v1) && (v1 < dat_21198)) && (dat_2119c < v2[3])) && (dat_211b0 <= *v2)) {
    v3 = *v2 + dat_21194;
    if (((v3 < v2[1]) && (v2[1] <= dat_211a0)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0x21180;
  return 0;
}

// Function: sub_17b60 @ 0x17b60
void sub_17b60(long a0,unsigned long *a1)
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(a0 + 0x48);
  *a1 = 0;
  a1[1] = v1;
  *(unsigned long **)(a0 + 0x48) = a1;
}

// Function: sub_17b80 @ 0x17b80
long sub_17b80(long *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*(void *)a0[6])(a1,a0[2]);
  if (v1 < (unsigned long)a0[2])
    return v1 * 0x10 + *a0;
  abort(); // no-return
}

// Function: sub_17bb0 @ 0x17bb0
long sub_17bb0(long a0,long a1,unsigned long *a2,char a3) // early-return
{
  void *v1;
  long *v2;
  long v3;
  long *v4;
  long v5;
  
  v4 = (long *)sub_17b80(a0,a1);
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
      sub_17b60(a0);
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
      sub_17b60(a0);
      return v5;
    }
    *v4 = 0;
  }
  return v5;
}

// Function: sub_17cb0 @ 0x17cb0
void sub_17cb0(long a0)
{
  if (*(long *)(a0 + 0x48)) {
    *(unsigned long *)(a0 + 0x48) = *(unsigned long *)(*(long *)(a0 + 0x48) + 8);
    return;
  }
  malloc(0x10); // tail-call
}

// Function: sub_17ce0 @ 0x17ce0
unsigned long sub_17ce0(long a0,unsigned long *a1,char a2) // return-dupe x2
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
label_17d08:
      v5 = &v5[2];
      if ((long *)a1[1] <= v5)
        return 1;
    }
    v3 = (long *)v5[1];
    if ((long *)v5[1]) {
      do {
        while( true ) {
          v6 = *v3;
          v2 = (long *)sub_17b80(a0,v6);
          v4 = (long *)v3[1];
          if (*v2) break;
          *v2 = v6;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
          sub_17b60(a0);
          v3 = v4;
          if (!v4) goto label_17d78;
        }
        v3[1] = v2[1];
        v2[1] = (long)v3;
        v3 = v4;
      } while (v4);
label_17d78:
      v6 = *v5;
    }
    v5[1] = 0;
    if (a2) goto label_17d08;
    v3 = (long *)sub_17b80(a0,v6);
    if (*v3) { // branch-flip
      v4 = (long *)sub_17cb0(a0);
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

// Function: sub_17e10 @ 0x17e10
unsigned long sub_17e10(float a0,unsigned long a1,char a2) // early-return, ternary
{
  unsigned long v1; // rax
  
  if (!a2) {
    a0 = (float)a1 / a0;
    if (dat_211a4 <= a0)
      return 0;
    a1 = (a0 < dat_211a8) ? (unsigned long)a0 : (long)(a0 - dat_211a8) ^ 0x8000000000000000;
  }
  v1 = sub_17a60(a1);
  if ((!(v1 >> 0x3c & 1)) && (!(v1 >> 0x3d)))
    return v1;
  return 0;
}

// Function: sub_17ea0 @ 0x17ea0
unsigned long sub_17ea0(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}

// Function: sub_17eb0 @ 0x17eb0
unsigned long sub_17eb0(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}

// Function: sub_17ec0 @ 0x17ec0
unsigned long sub_17ec0(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}

// Function: sub_17ed0 @ 0x17ed0
unsigned long sub_17ed0(unsigned long *a0)
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

// Function: sub_17f30 @ 0x17f30
bool sub_17f30(unsigned long *a0)
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
label_17f50:
      v2 = &v2[2];
    }
    v1 = v2[1];
    v4 += 1;
    v3 += 1;
    if (!v1) goto label_17f50;
    do {
      v1 = *(long *)(v1 + 8);
      v3 += 1;
    } while (v1);
    v2 = &v2[2];
  } while( true );
}

// Function: sub_17fa0 @ 0x17fa0
void sub_17fa0(unsigned long a0,void *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_17ec0(a0);
  v2 = sub_17ea0();
  v3 = sub_17eb0();
  v4 = sub_17ed0();
  __fprintf_chk(a1,1,"# entries:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v2);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v3);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v4); // tail-call
}

// Function: sub_180c0 @ 0x180c0
long sub_180c0(long a0,long a1)
{
  void *v1;
  long *v2;
  long v3;
  
  v2 = (long *)sub_17b80(a0,a1);
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

// Function: sub_18120 @ 0x18120
long sub_18120(unsigned long *a0)
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

// Function: sub_18170 @ 0x18170
long sub_18170(long a0,long a1)
{
  long v1;
  long *v2; // rax
  long *v3;
  
  v2 = (long *)sub_17b80(a0,a1);
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

// Function: sub_181e0 @ 0x181e0
void sub_181e0(unsigned long *a0,long a1,unsigned long a2)
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

// Function: sub_18230 @ 0x18230
long sub_18230(unsigned long *a0,void *a1,unsigned long a2)
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

// Function: sub_182b0 @ 0x182b0
unsigned long sub_182b0(unsigned char *a0,unsigned long a1)
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

// Function: sub_182f0 @ 0x182f0
void sub_182f0(unsigned long *a0)
{
  *(char *)&a0[2] = 0;
  *a0 = 0x3f80000000000000;
  a0[1] = 0x3fb4fdf43f4ccccd;
}

// Function: sub_18320 @ 0x18320
unsigned long * sub_18320(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_17aa0;
  if (!a3)
    a3 = sub_17ac0;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = 0x21180;
  v1[5] = a1;
  if (sub_17ad0(v1)) {
    v2 = sub_17e10(*(unsigned int *)(a1 + 8),a0,*(char *)(a1 + 0x10));
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

// Function: sub_18430 @ 0x18430
void sub_18430(unsigned long *a0)
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
        if ((long *)a0[1] <= v5) goto label_184cb;
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
label_184cb:
  a0[3] = 0;
  a0[4] = 0;
  return;
}

// Function: sub_184e0 @ 0x184e0
void sub_184e0(unsigned long *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = (long *)*a0;
  v3 = (long *)a0[1];
  if ((a0[8]) && (a0[4])) {
    if (v3 <= v4) goto label_1858c;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_1854f;
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
label_1854f:
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
label_1858c:
  v2 = (void *)a0[9];
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free((void *)*a0);
  free(a0); // tail-call
}

// Function: sub_185d0 @ 0x185d0
unsigned int sub_185d0(unsigned long *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_17e10(*(unsigned int *)(a0[5] + 8),a1,*(char *)(a0[5] + 0x10));
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
  v1 = sub_17ce0(&v2,a0,0);
  if (!(char)v1) {
    a0[9] = v11;
    if ((sub_17ce0(a0,&v2,1)) && (sub_17ce0(a0,&v2,0))) {
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

// Function: sub_18750 @ 0x18750
unsigned long sub_18750(long a0,long a1,long *a2) // ternary
{
  long v1;
  long *v2; // rax
  long *v3; // stack - 0x38
  unsigned long v4; // rsi
  float v5; // xmm4_da
  float v6; // xmm5_da
  
  if (a1) {
    v1 = sub_17bb0(a0,a1,&v3,0);
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
      sub_17ad0(a0);
      v1 = *(long *)(a0 + 0x28);
      if (v5 * *(float *)(v1 + 8) < v6) {
        v5 = v5 * *(float *)(v1 + 0xc);
        if (!*(char *)(v1 + 0x10))
          v5 = v5 * *(float *)(v1 + 8);
        if (dat_211a4 <= v5)
          return 0xffffffff;
        v4 = (dat_211a8 <= v5) ? (long)(v5 - dat_211a8) ^ 0x8000000000000000 : (unsigned long)v5;
        if (!sub_185d0(a0,v4))
          return 0xffffffff;
        if (sub_17bb0(a0,a1,&v3,0)) {
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
    v2 = (long *)sub_17cb0(a0);
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

// Function: sub_18940 @ 0x18940
unsigned long sub_18940(unsigned long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_18750(a0,a1,&v3);
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

// Function: sub_189a0 @ 0x189a0
long sub_189a0(long a0,unsigned long a1) // ternary x2
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
  
  v5 = sub_17bb0(a0,a1,&v7,1);
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
      sub_17ad0(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_211a8 <= v8) ? (long)(v8 - dat_211a8) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_185d0(a0,v6)) {
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

// Function: sub_18b30 @ 0x18b30
void sub_18b30(void)
{
  sub_189a0(); // tail-call
}

// Function: sub_18b40 @ 0x18b40
void sub_18b40(unsigned int *a0,unsigned int a1)
{
  *(unsigned long *)&a0[5] = 0;
  *(char *)&a0[7] = 1;
  *a0 = a1;
  a0[1] = a1;
  a0[2] = a1;
  a0[3] = a1;
  a0[4] = a1;
}

// Function: sub_18b60 @ 0x18b60
char sub_18b60(long a0)
{
  return *(char *)(a0 + 0x1c);
}

// Function: sub_18b70 @ 0x18b70
unsigned int sub_18b70(long a0,unsigned int a1)
{
  unsigned int v1;
  unsigned int v2; // eax
  unsigned int v3; // edx
  
  v2 = (unsigned int)(*(unsigned char *)(a0 + 0x1c) ^ 1);
  v3 = *(int *)(a0 + 0x14) + v2 & 3;
  v1 = *(unsigned int *)(a0 + (unsigned long)v3 * 4);
  *(unsigned int *)(a0 + (unsigned long)v3 * 4) = a1;
  *(unsigned int *)(a0 + 0x14) = v3;
  if (*(unsigned int *)(a0 + 0x18) == v3)
    *(unsigned int *)(a0 + 0x18) = v2 + *(unsigned int *)(a0 + 0x18) & 3;
  *(char *)(a0 + 0x1c) = 0;
  return v1;
}

// Function: sub_18bb0 @ 0x18bb0
unsigned int sub_18bb0(long a0)
{
  unsigned int v1;
  unsigned int v2;
  char v3; // al
  
  v3 = sub_18b60(a0);
  if (v3)
    abort(); // no-return
  v1 = *(unsigned int *)(a0 + 0x14);
  v2 = *(unsigned int *)(a0 + (unsigned long)v1 * 4);
  *(unsigned int *)(a0 + (unsigned long)v1 * 4) = *(unsigned int *)(a0 + 0x10);
  if (v1 != *(unsigned int *)(a0 + 0x18)) {
    *(unsigned int *)(a0 + 0x14) = v1 + 3 & 3;
    return v2;
  }
  *(char *)(a0 + 0x1c) = 1;
  return v2;
}

// Function: sub_18c10 @ 0x18c10
void sub_18c10(unsigned long a0,unsigned long a1,long a2)
{
  if (!a2)
    a2 = 0x3ab10;
  sub_18c40(0,a0,a1,a2); // tail-call
}

// Function: sub_18c40 @ 0x18c40
unsigned long sub_18c40(unsigned int *a0,unsigned char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,(char *)a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_1dee0(0))) {
    v1 = 1;
    *a0 = (unsigned int)*a1;
  }
  return v1;
}

// Function: sub_18cd0 @ 0x18cd0
unsigned long sub_18cd0(unsigned char *a0,unsigned char *a1) // early-return x3
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
  unsigned int v4;
  int v5; // eax
  unsigned char v6;
  char v7 [4]; // stack - 0xb8
  char v8 [4]; // stack - 0x78
  unsigned long v9; // stack - 0xb4
  
  if (a1 == a0)
    return 0;
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
    v7[0] = 0;
    v9 = 0;
    v8[0] = 0;
    v15 = 0;
    v11 = a0;
    v17 = a1;
    while( true ) {
      v10 = 0;
      v16 = 0;
      sub_18f20(v7);
      if ((v13) && (!v14)) break;
      sub_18f20(v8);
      if (v19) { // branch-flip
        if (!v20) break;
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
          if (v12 < v18) {
            if (memcmp(v11,v17,v12) <= 0)
              return 0xffffffff;
            return 1;
          }
          if (0 <= memcmp(v11,v17,v18))
            return 1;
          return 0xffffffff;
        }
        v5 = memcmp(v11,v17,v12);
        if (v5)
          return CONCAT44(dat_4,v5);
      }
      v11 = &v11[v12];
      v17 = &v17[v18];
    }
    sub_18f20(v7);
    if ((!v13) || (v14))
      return 1;
    sub_18f20(v8);
    if ((!v19) || (v4 = 0, v20))
      return 0xffffffff;
  }
  else {
    v2 = *(long *)__ctype_b_loc();
    do {
      v6 = *a0;
      if (*(unsigned char *)(v2 + 1 + (unsigned long)v6 * 2) & 1)
        v6 = *(unsigned char *)(*(long *)__ctype_tolower_loc() + (unsigned long)v6 * 4);
      v1 = *a1;
      if (*(unsigned char *)(v2 + 1 + (unsigned long)v1 * 2) & 1)
        v1 = *(unsigned char *)(*(long *)__ctype_tolower_loc() + (unsigned long)v1 * 4);
      if (!v6)
        return (unsigned long)-(unsigned int)v1;
      a0 = &a0[1];
      a1 = &a1[1];
    } while (v6 == v1);
    v4 = (unsigned int)v6 - (unsigned int)v1;
  }
  return (unsigned long)v4;
}

// Function: sub_18f20 @ 0x18f20
void sub_18f20(char *a0)
{
  char v1;
  char *v2;
  unsigned long v3;
  unsigned long v4; // rax
  long v5; // rax
  
  if (a0[0xc])
    return;
  if (!*a0) {
    v2 = *(char **)&a0[0x10];
    v1 = *v2;
    if (sub_1e0b0((int)v1)) {
      a0[0x18] = '\x01';
      a0[0x19] = '\0';
      a0[0x1a] = '\0';
      a0[0x1b] = '\0';
      a0[0x1c] = '\0';
      a0[0x1d] = '\0';
      a0[0x1e] = '\0';
      a0[0x1f] = '\0';
      v1 = *v2;
      a0[0x20] = '\x01';
      *(int *)&a0[0x24] = (int)v1;
      goto label_18f62;
    }
    if (!mbsinit(&a0[4]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    *a0 = '\x01';
  }
  v4 = __ctype_get_mb_cur_max();
  v3 = *(unsigned long *)&a0[0x10];
  v5 = sub_18c40(&a0[0x24],v3,sub_1c9a0(v3,v4),&a0[4]);
  *(long *)&a0[0x18] = v5;
  if (v5 == -1) {
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
  if (v5 != -2) { // branch-flip
    if (!v5) {
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
    if (mbsinit(&a0[4]))
      *a0 = '\0';
  }
  else {
    v4 = strlen(*(char **)&a0[0x10]);
    a0[0x20] = '\0';
    *(unsigned long *)&a0[0x18] = v4;
  }
label_18f62:
  a0[0xc] = '\x01';
  return;
}

// Function: sub_190a0 @ 0x190a0
void sub_190a0(long a0,long a1)
{
  *(long *)(a0 + 0x10) = *(long *)(a0 + 0x10) + a1;
}

// Function: sub_190b0 @ 0x190b0
void sub_190b0(char *a0,char *a1)
{
  char v1;
  
  v1 = *a1;
  *a0 = v1;
  if (v1) // branch-flip
    *(unsigned long *)&a0[4] = *(unsigned long *)&a1[4];
  else {
    a0[4] = '\0';
    a0[5] = '\0';
    a0[6] = '\0';
    a0[7] = '\0';
    a0[8] = '\0';
    a0[9] = '\0';
    a0[10] = '\0';
    a0[0xb] = '\0';
  }
  a0[0xc] = a1[0xc];
  sub_1e050(&a0[0x10],&a1[0x10]); // tail-call
}

// Function: sub_190f0 @ 0x190f0
unsigned long * sub_190f0(unsigned long *a0,unsigned char a1,unsigned char a2,unsigned long a3) // early-return, return-dupe
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long *v3; // rdx
  unsigned long v4; // rsi
  unsigned long v5; // rsi
  
  if (a1 == a2)
    return memchr(a0,(unsigned int)a1,a3); // tail-call
  for (; (a3 && ((unsigned long)a0 & 7)); a0 = (unsigned long *)((long)a0 + 1)) {
    if (((unsigned char)*a0 == a2) || ((unsigned char)*a0 == a1))
      return a0;
    a3 -= 1;
  }
  v4 = (long)(int)(unsigned int)CONCAT11(a1,a1) << 0x10 | (long)(int)(unsigned int)CONCAT11(a1,a1);
  v1 = (long)(int)(unsigned int)CONCAT11(a2,a2) | (long)(int)(unsigned int)CONCAT11(a2,a2) << 0x10;
  for (; 8 <= a3; a3 = a3 - 8) {
    v5 = *a0 ^ (v4 << 0x20 | v4);
    v2 = *a0 ^ (v1 << 0x20 | v1);
    if ((~v2 & v2 + 0xfefefefefefefeff | v5 + 0xfefefefefefefeff & ~v5) & 0x8080808080808080) goto label_191d6;
    a0 = &a0[1];
  }
  if (!a3)
    return NULL;
label_191d6:
  v3 = (unsigned long *)(a3 + (long)a0);
  while( true ) {
    if ((unsigned char)*a0 == a1)
      return a0;
    if ((unsigned char)*a0 == a2) break;
    a0 = (unsigned long *)((long)a0 + 1);
    if (a0 == v3)
      return NULL;
  }
  return a0;
}

// Function: sub_19230 @ 0x19230
void sub_19230(long a0,unsigned long a1)
{
  if (*(unsigned char *)(a0 + 0x50) & 1) {
    (**(void **)(a0 + 0x38))(*(unsigned long *)(a0 + 0x48)); // jump-as-call
    return;
  }
  (**(void **)(a0 + 0x38))(a1); // jump-as-call
}

// Function: sub_19250 @ 0x19250
void sub_19250(long a0,unsigned long a1)
{
  if (*(unsigned char *)(a0 + 0x50) & 1) {
    (**(void **)(a0 + 0x40))(*(unsigned long *)(a0 + 0x48)); // jump-as-call
    return;
  }
  (**(void **)(a0 + 0x40))(a1); // jump-as-call
}

// Function: sub_19270 @ 0x19270
unsigned long sub_19270(long *a0,long a1,long a2)
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
  v1 = (long *)sub_19230(a0);
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

// Function: sub_19300 @ 0x19300
void sub_19300(void)
{
  __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"memory exhausted",5));
  exit(dat_2a080); // no-return
}

// Function: _obstack_begin @ 0x19350
void _obstack_begin(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) & 0xfe;
  *(unsigned long *)(a0 + 0x38) = a3;
  *(unsigned long *)(a0 + 0x40) = a4;
  sub_19270(); // tail-call
}

// Function: _obstack_begin_1 @ 0x19370
void _obstack_begin_1(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5)
{
  *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 1;
  *(unsigned long *)(a0 + 0x38) = a3;
  *(unsigned long *)(a0 + 0x40) = a4;
  *(unsigned long *)(a0 + 0x48) = a5;
  sub_19270(); // tail-call
}

// Function: _obstack_newchunk @ 0x19390
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
    v4 = (unsigned long *)sub_19230(a0);
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
        sub_19250(a0,v2);
      }
      a0[2] = (unsigned long)v7;
      a0[3] = (long)v7 + v6;
      *(unsigned char *)&a0[10] = (unsigned char)a0[10] & 0xfd;
      return;
    }
  }
  (*obstack_alloc_failed_handler)(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: _obstack_allocated_p @ 0x19490
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

// Function: _obstack_free @ 0x194d0
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
      sub_19250(a0);
      *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 2;
    } while (v2);
  }
  if (!a1)
    return;
  abort(); // no-return
}

// Function: _obstack_memory_used @ 0x19540
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

// Function: sub_19570 @ 0x19570
void sub_19570(int a0,char *a1,unsigned int a2,unsigned long a3)
{
  unsigned long v1; // rcx
  
  v1 = 0;
  if (a2 & 0x40)
    v1 = a3 & 0xffffffff;
  sub_1c9e0(openat(a0,a1,a2,v1));
}

// Function: sub_195f0 @ 0x195f0
void * sub_195f0(unsigned long a0,unsigned long a1,unsigned int a2,int *a3)
{
  int v1;
  int v2; // eax
  void *v3; // rax
  int *v4; // rax
  
  v3 = NULL;
  v2 = sub_19570(a0,a1,a2 | 0x90900);
  if (0 <= v2) {
    v3 = fdopendir(v2);
    if (v3) // branch-flip
      *a3 = v2;
    else {
      v4 = __errno_location();
      v1 = *v4;
      close(v2);
      *v4 = v1;
    }
  }
  return v3;
}

// Function: sub_19650 @ 0x19650
int * sub_19650(int *a0,int a1)
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

// Function: sub_196a0 @ 0x196a0
bool sub_196a0(long a0,long a1)
{
  return sub_ddf0(a0 + 9,a1 + 9) == 0;
}

// Function: sub_196c0 @ 0x196c0
unsigned long sub_196c0(long a0,unsigned long a1,unsigned char a2)
{
  unsigned char v1;
  
  if (sub_db40((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 8) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 8);
  }
  if (v1 == a2) {
    if (a2)
      return sub_196a0(a0,a1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_19720 @ 0x19720
unsigned long sub_19720(long a0,unsigned long a1,unsigned char a2,char a3)
{
  unsigned char v1;
  
  if (sub_db40((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 7) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 7);
  }
  if (v1 == a2) {
    if (a2)
      return sub_196c0(a0,a1,(int)a3); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_197a0 @ 0x197a0
unsigned long sub_197a0(long a0,unsigned long a1,unsigned char a2,char a3,char a4)
{
  unsigned char v1;
  
  if (sub_db40((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 6) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 6);
  }
  if (v1 == a2) {
    if (a2)
      return sub_19720(a0,a1,(int)a3,(int)a4); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_19810 @ 0x19810
unsigned long sub_19810(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5)
{
  unsigned char v1;
  
  if (sub_db40((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 5) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 5);
  }
  if (v1 == a2) {
    if (a2)
      return sub_197a0(a0,a1,(int)a3,(int)a4,(int)a5); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_198b0 @ 0x198b0
unsigned long sub_198b0(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6)
{
  unsigned int v1;
  unsigned char v2;
  
  v1 = a6;
  if (sub_db40((int)(char)a2)) // branch-flip
    v2 = *(unsigned char *)(a0 + 4) & 0xdf;
  else {
    v2 = *(unsigned char *)(a0 + 4);
  }
  if (v2 == a2) {
    if (a2)
      return sub_19810(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_19950 @ 0x19950
unsigned long sub_19950(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,int a6,unsigned int a7)
{
  unsigned int v1;
  unsigned int v2;
  unsigned char v3;
  
  v2 = a7;
  v1 = a6;
  if (sub_db40((int)(char)a2)) // branch-flip
    v3 = *(unsigned char *)(a0 + 3) & 0xdf;
  else {
    v3 = *(unsigned char *)(a0 + 3);
  }
  if (v3 == a2) {
    if (a2) {
      a6 = (int)(char)v2;
      return sub_198b0(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_19a10 @ 0x19a10
unsigned long sub_19a10(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,unsigned int a8)
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  unsigned char v4;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_db40((int)(char)a2)) // branch-flip
    v4 = *(unsigned char *)(a0 + 2) & 0xdf;
  else {
    v4 = *(unsigned char *)(a0 + 2);
  }
  if (v4 == a2) {
    if (a2) {
      a7 = (int)(char)v3;
      a6 = (int)(char)v2;
      return sub_19950(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_19ad0 @ 0x19ad0
unsigned long sub_19ad0(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,int a8,unsigned int a9)
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
  if (sub_db40((int)(char)a2)) // branch-flip
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
      return sub_19a10(a0,a1,(int)a3,(int)a4,(int)a5,(int)v6); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_19bb0 @ 0x19bb0
unsigned long sub_19bb0(unsigned char *a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6,unsigned int a7,unsigned int a8) // early-return
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_db40((int)(char)a2)) { // branch-flip
    if (a2 != (*a0 & 0xdf))
      return 0;
  }
  else if (a2 != *a0)
    return 0;
  return sub_19ad0(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1,(int)(char)v2,(int)(char)v3,0,0);
}

// Function: sub_19c70 @ 0x19c70
char * sub_19c70(char *a0,int a1) // return-dupe, ternary x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = dcgettext(NULL,a0,5);
  if (a0 == v1) {
    v2 = sub_1dfe0();
    if (sub_19bb0(v2,"UTF-8",0x55,0x54,0x46,0x2d,0x38,0,0)) { // branch-flip
      v1 = (*v1 != '`') ? "’" : "‘"; // branch-flip
    }
    else {
      if (!sub_19bb0(v2,"GB18030",0x47,0x42,0x31,0x38,0x30,0x33,0x30)) {
        if (a1 == 9)
          return "\"";
        return "\'";
      }
      v1 = (*v1 != '`') ? (char *)0x21227 : (char *)0x2122c; // branch-flip
    }
  }
  return v1;
}

// Function: sub_19da0 @ 0x19da0
long * sub_19da0(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_19e10:
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
label_1a296:
      v2 = 0;
      goto label_1a250;
    case 2:
      if (!v17) {
        v2 = 0;
        goto label_1a3b1;
      }
      v24 = 1;
      v2 = 0;
      v12 = 0;
      v30 = 1;
      v31 = "\'";
      break;
    case 3:
      v2 = 1;
label_1a250:
      v24 = 1;
      v12 = 0;
      a4 = 2;
      v30 = 1;
      v31 = "\'";
      break;
    case 4:
      if (!v17) {
        v2 = 1;
        goto label_1a3b1;
      }
      goto label_1a296;
    case 5:
      if (!v17) goto label_1a1f1;
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
        v9 = (char *)sub_19c70("`",a4);
        v36 = (char *)v9;
        v9 = (char *)sub_19c70("\'",a4);
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
label_19ed3:
  do {
label_19ee6:
    v23 = 0;
    a1 = v13;
label_19ef0:
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
          if ((a3 < v23 + v30) || (v8 = memcmp(v11,v31,v30), v8)) goto label_1a860;
          if (!v24) {
            v7 = *v11;
            v18 = (unsigned long)v7;
            v21 = v18;
            if ((char)v7 <= '?') {
              switch(v7) {
                case 0:
                  goto label_1a45c;
                default:
                  goto label_1a133;
                case 7:
                  goto label_1a420;
                case 8:
                  goto label_1a405;
                case 9:
                  goto label_1a4e3;
                case 10:
                  goto label_1a3fb;
                case 0xb:
                  goto label_1a510;
                case 0xc:
                  goto label_1a447;
                case 0xd:
                  goto label_1a34d;
                case 0x20:
                  goto label_1a51d;
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
                  goto label_1a0e6;
                case 0x23:
                  goto label_1a4c0;
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
                  goto label_1a018;
                case 0x27:
                  goto label_1a332;
                case 0x3f:
                  goto label_1a2f1;
                
              }
            }
            if ('{' > (char)v7) {
              if (v7 == 0x40) goto label_1a133;
              v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
              if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_1a018;
              if (!((unsigned long)v9 & 0xa4000000)) goto label_1a904;
              goto label_1a848;
            }
            if (v7 == 0x7d) goto label_1a9d0;
            if ((char)v7 <= '}') {
              v21 = 0x7b;
              if (v7 == 0x7b) goto label_1a82a;
              v21 = 0x7c;
              if (v7 == 0x7c) goto label_1a848;
              goto label_1a133;
            }
            if (v7 != 0x7e) goto label_1a133;
label_1a984:
            if (!v23) {
              v7 = a4 == 2;
              v21 = 0x7e;
              v26 = v3;
              goto label_1a1a9;
            }
            v21 = 0x7e;
            goto label_1a992;
          }
          goto label_1a11c;
        }
label_1a860:
        v7 = *v11;
        v18 = (unsigned long)v7;
        v21 = v18;
        if ('?' < (char)v7) {
          if ('{' <= (char)v7) { // branch-flip
            if (v7 != 0x7d) { // branch-flip
              if ('~' <= (char)v7) {
                v22 = 0;
                if (v7 == 0x7e) goto label_1a984;
                goto label_1a130;
              }
              v21 = 0x7b;
              v25 = 0;
              if (v7 != 0x7b) {
                v21 = 0x7c;
                v25 = 0;
                v22 = 0;
                if (v7 == 0x7c) goto label_1a848;
                goto label_1a133;
              }
            }
            else {
              v21 = 0x7d;
              v25 = 0;
            }
label_1a82a:
            if (a3 == 0xffffffffffffffff) goto label_1a9df;
label_1a838:
            v22 = v25;
            if (a3 == 1) goto label_1a4c5;
          }
          else {
            if (v7 == 0x40) goto label_1a130;
            v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
            v22 = 0;
            if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_1a018;
            v25 = 0;
            v22 = 0;
            if (!((unsigned long)v9 & 0xa4000000)) {
label_1a904:
              if ((char)v18 != '\\') goto label_1a133;
              if (((bool)(v2 & v24)) && (v30)) goto label_1a92a;
              v16 = 0x5c;
              goto label_1a352;
            }
          }
label_1a848:
          v7 = a4 == 2;
          v26 = v3;
          v25 = 0;
          goto label_1a1a9;
        }
        switch(v7) {
          case 0:
            goto label_1a44e;
          default:
label_1a130:
            v25 = 0;
label_1a133:
            v16 = (unsigned char)v18;
            v22 = v25;
            if (v33 != 1) goto label_1a6b3;
label_1a140:
            v9 = __ctype_b_loc();
            v21 = 1;
            v26 = (*(unsigned char *)(*v9 + 1 + v18 * 2) & 0x40) == 0;
            v27 = (bool)(v26 & v2);
            v26 = !v26;
            v22 = v25;
            goto label_1a18e;
          case 7:
label_1a420:
            v16 = 0x61;
            if (!v24) goto label_1a418;
            goto label_1a37f;
          case 8:
label_1a405:
            v16 = 0x62;
            goto label_1a40a;
          case 9:
            v25 = 0;
label_1a4e3:
            v21 = 9;
            v16 = 0x74;
            goto label_1a4f0;
          case 10:
label_1a3fb:
            v16 = 0x6e;
            break;
          case 0xb:
label_1a510:
            v16 = 0x76;
            break;
          case 0xc:
label_1a447:
            v16 = 0x66;
label_1a40a:
            if (v24) goto label_1a37f;
label_1a418:
            v25 = 0;
            goto label_1a09e;
          case 0xd:
label_1a34d:
            v16 = 0x72;
            break;
          case 0x20:
            v22 = 0;
label_1a51d:
            v21 = 0x20;
            goto label_1a018;
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
            goto label_1a0e9;
          case 0x23:
            v21 = 0x23;
            v25 = 0;
            goto label_1a4c5;
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
label_1a018:
            v7 = 0;
            goto label_1a028;
          case 0x27:
            v22 = 0;
            goto label_1a332;
          case 0x3f:
            v22 = 0;
            goto label_1a2f1;
          
        }
        goto label_1a352;
      }
      v16 = *v11;
      v18 = (unsigned long)v16;
      if ('?' < (char)v16) {
        if ('{' <= (char)v16) {
          if (v16 == 0x7d) {
label_1a9d0:
            v21 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_1a838;
label_1a9df:
            v22 = v25;
            if (a2[1]) goto label_1a848;
            goto label_1a4c5;
          }
          if ((char)v16 <= '}') {
            v21 = 0x7b;
            if (v16 != 0x7b) {
              if (v16 == 0x7c) goto label_1a0e6;
              goto label_1a133;
            }
            goto label_1a82a;
          }
          v21 = 0x7e;
          if (v16 == 0x7e) goto label_1a4c5;
          if (v33 == 1) goto label_1a140;
label_1a6b3:
          v14 = 0;
          if (a3 == 0xffffffffffffffff) {
            v9 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v9;
          }
          v21 = 0;
          do {
            v18 = v23 + v21;
            v9 = (long)sub_18c40(&v15,&a2[v18],a3 - v18,&v14);
            v6 = (long)v9;
            v25 = v26;
            if (!v9) break;
            if (v9 == (long *)0xffffffffffffffff) {
              v25 = 0;
              goto label_1ac1b;
            }
            if (v9 == (long *)0xfffffffffffffffe) {
              v9 = (long *)v18;
              if (a3 > v18) goto label_1af95;
              goto label_1af9f;
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
                if ((v7 <= 0x21) && (0x20000002bU >> (v18 & 0x3f) & 1)) goto label_1a7ed;
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
          goto label_1ac1b;
        }
        if (v16 == 0x40) goto label_1a133;
        v9 = (unsigned long)(1L << (v16 + 0xbf & 0x3f));
        if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_1a1a0;
        if ((unsigned long)v9 & 0xa4000000) goto label_1a0e6;
        if (v16 != 0x5c) goto label_1a133;
        if (a4 == 2) {
          if (!v24) goto label_1a92a;
          goto label_1a113;
        }
        if (!(bool)(v2 & v24 & v30 != 0)) {
          v21 = 0x5c;
          v16 = 0x5c;
          v25 = 0;
          goto label_1a4f0;
        }
label_1a92a:
        v23 += 1;
        v25 = 0;
        v16 = 0x5c;
        v7 = v20;
        goto label_1a940;
      }
      switch(v16) {
        case 0:
          if (!v2) {
            v22 = 0;
            v21 = 0;
            if (v34 & 1) goto label_1a5c2;
            goto label_1a4fb;
          }
label_1a44e:
          if (!v24) {
            v22 = 0;
label_1a45c:
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
                goto label_1a1d3;
              }
label_1ad65:
              a0[v13] = 0x5c;
              v12 = v13;
              v20 = (bool)v7;
            }
            else if (v12 < a1) {
              v7 = v20;
              goto label_1ad65;
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
label_1ad9c:
              v16 = (unsigned char)v21;
              v25 = 0;
              v12 = v13;
              v7 = v26;
              if (!v2 || v27) goto label_1a1d3;
              goto label_1a028;
            }
            v16 = 0x30;
            v25 = 0;
            v12 = v13;
            v7 = v26;
            goto label_1a1d3;
          }
          if (a4 != 2) goto label_1a11c;
          goto label_1a38b;
        default:
          goto label_1a133;
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
          goto label_1a57f;
        case 10:
          v21 = 10;
          v16 = 0x6e;
          goto label_1a57f;
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
label_1a57f:
          v7 = a4 == 2 & v24;
          v25 = 0;
          if (!(bool)v7) goto label_1a4f0;
label_1a101:
          a4 = 2;
label_1a113:
          if (!v28) goto label_1a11c;
          goto label_1a38b;
        case 0x20:
          v18 = 0x20;
          goto label_1a0e9;
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
label_1a0e6:
          v26 = 0;
          goto label_1a0e9;
        case 0x23:
label_1a4c0:
          v21 = 0x23;
label_1a4c5:
          v22 = v25;
          if (v23) {
label_1a992:
            v7 = a4 == 2;
            v26 = v3;
            v25 = 0;
            goto label_1a1a9;
          }
          v18 = v21;
label_1a0e9:
          v21 = v18;
          v22 = v25;
          v25 = v26;
          if ((a4 == 2) && (v24)) goto label_1a101;
          goto label_1a1a7;
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
          goto label_1a1a0;
        case 0x27:
label_1a332:
          v25 = v26;
          if (a4 != 2) {
            v7 = 0;
            v21 = 0x27;
            goto label_1a1a9;
          }
          if (!v24) {
            if (a1) { // branch-flip
              v13 = 0;
              v18 = a1;
              if (v32) goto label_1adbb;
            }
            else {
label_1adbb:
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
            goto label_1a1d3;
          }
          goto label_1a113;
        case 0x3f:
label_1a2f1:
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
                  goto label_1a1a9;
                }
                if (v24) goto label_1a11c;
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
                goto label_1ad9c;
              }
            }
            v7 = 0;
            v21 = 0x3f;
            v26 = v3;
            v25 = 0;
            goto label_1a1a9;
          }
          if (v24) goto label_1a113;
          v7 = 0;
          v16 = 0x3f;
          v25 = 0;
          goto label_1a1d3;
        
      }
label_1a4f0:
      v22 = v25;
      if (!v2) {
label_1a4fb:
        v16 = (unsigned char)v21;
        v25 = 0;
        v7 = 0;
        v26 = v3;
        if (!v24) goto label_1a1d0;
        goto label_1a028;
      }
label_1a352:
      v25 = 0;
      goto label_1a363;
    }
    if (!(bool)(v12 == 0 & v24 & a4 == 2)) {
      v7 = a4 == 2 & (v24 ^ 1U);
      v24 = (bool)(v24 ^ 1U);
      if ((!(bool)v7) || (v24 = (bool)v7, !v3)) {
label_1b01f:
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
        if (a1 || !v32) goto label_1b01f;
        v23 = v32;
        goto label_1aaeb;
      }
      v12 = 0;
      a4 = 5;
      v9 = (unsigned long)__ctype_get_mb_cur_max();
      v30 = 1;
      v33 = (unsigned long)v9;
      v31 = "\"";
      if (!(v34 & 2)) goto label_1b0cf;
      v3 = 0;
      v20 = 0;
      v32 = 0;
      v2 = v4;
      v24 = v4;
      goto label_19ee6;
    }
label_1a7ed:
    if (v2) {
label_1a38b:
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
label_1a3b1:
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
label_1aaeb:
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
label_1af95:
    if (!a2[(long)v9]) break;
  }
label_1af9f:
  v25 = 0;
label_1ac1b:
  v18 = (unsigned long)v16;
  v26 = v25;
  if (2 <= v21) {
label_1ac25:
    v19 = 0;
    v21 += v23;
    v13 = v23;
    do {
      v16 = (unsigned char)v18;
      if (v27) {
        v28 = a4 == 2;
        if (v24) goto label_1a113;
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
        if (v21 <= v23) goto label_1a0b1;
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
          goto label_1a940;
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
label_1a18e:
  if (v27) {
    v25 = 0;
    v27 = v2;
    goto label_1ac25;
  }
label_1a1a0:
  v21 = v18;
  v25 = v26;
label_1a1a7:
  v7 = a4 == 2;
  v26 = v3;
label_1a1a9:
  v16 = (unsigned char)v21;
  v27 = (bool)v7 == 0;
  v3 = v26;
  v7 = 0;
  if ((v27 && v2) || (v7 = 0, v24)) {
label_1a028:
    v16 = (unsigned char)v21;
    v12 = v13;
    if (!v29) goto label_1a1d3;
    if (!(*(unsigned int *)(v29 + (v21 >> 5) * 4) >> (v16 & 0x1f) & 1)) goto label_1a1d3;
  }
  else {
label_1a1d0:
    v7 = 0;
    v3 = v26;
label_1a1d3:
    if (!v22) {
      v7 ^= 1;
      v23 += 1;
      v7 &= v20;
label_1a940:
      if ((bool)v7) {
        if (v12 < a1)
          a0[v12] = 0x27;
        if (v12 + 1 < a1)
          a0[v12 + 1] = 0x27;
        v20 = 0;
        v12 += 2;
      }
      goto label_1a0b1;
    }
  }
label_1a363:
  if (v24) {
    v24 = v2;
label_1a37f:
    if ((bool)(v24 & a4 == 2)) goto label_1a38b;
label_1a11c:
    v34 &= 0xfffffffd;
    v29 = 0;
    goto label_19e10;
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
label_1a09e:
  if (v12 < a1)
    a0[v12] = 0x5c;
  v12 += 1;
  v23 += 1;
label_1a0b1:
  if (v12 < a1)
    a0[v12] = v16;
  v12 += 1;
  if (!v25)
    v4 = 0;
  goto label_19ef0;
label_1a5c2:
  v23 += 1;
  goto label_19ef0;
label_1b0cf:
  a1 = v32;
label_1a1f1:
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
  goto label_19ed3;
}

// Function: sub_1b1d0 @ 0x1b1d0
void * sub_1b1d0(unsigned int a0,unsigned long a1,unsigned long a2,unsigned int *a3)
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
    v6 = dat_2a0e0;
    if (dat_2a0d8 <= (int)a0) {
      v10 = (long)dat_2a0d8;
      v9 = (long)(int)((a0 - dat_2a0d8) + 1);
      if (dat_2a0e0 != (unsigned long *)0x2a0f0) // branch-flip
        v6 = (unsigned long *)sub_1d2b0(dat_2a0e0,&v10,v9,0x7fffffff,0x10);
      else {
        v6 = (unsigned long *)sub_1d2b0(0,&v10,v9,0x7fffffff,0x10);
        *v6 = dat_2a0f0;
        v6[1] = dat_2a0f8;
      }
      dat_2a0e0 = v6;
      memset(&v6[(long)dat_2a0d8 * 2],0,(v10 - dat_2a0d8) * 0x10);
      dat_2a0d8 = (int)v10;
    }
    v1 = &v6[(long)(int)a0 * 2];
    v3 = a3[1];
    v5 = *v1;
    v8 = (void *)v1[1];
    v7 = sub_19da0(v8,v5,a1,a2,*a3,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    if (v5 <= v7) {
      v7 += 1;
      *v1 = v7;
      if (v8 != (void *)0x3ab20)
        free(v8);
      v8 = (void *)sub_1d130(v7);
      v4 = *a3;
      v1[1] = (unsigned long)v8;
      sub_19da0(v8,v7,a1,a2,v4,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    }
    *v11 = v2;
    return v8;
  }
  abort(); // no-return
}

// Function: sub_1b3c0 @ 0x1b3c0
void sub_1b3c0(long a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = 0x3ac20;
  sub_1d3f0(a0,0x38);
  *v2 = v1;
}

// Function: sub_1b400 @ 0x1b400
unsigned int sub_1b400(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x3ac20;
  return *a0;
}

// Function: sub_1b420 @ 0x1b420
void sub_1b420(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x3ac20;
  *a0 = a1;
}

// Function: sub_1b440 @ 0x1b440
unsigned int sub_1b440(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0x3ac20;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_1b480 @ 0x1b480
unsigned int sub_1b480(long a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = 0x3ac20;
  v1 = *(unsigned int *)(a0 + 4);
  *(unsigned int *)(a0 + 4) = a1;
  return v1;
}

// Function: sub_1b4a0 @ 0x1b4a0
void sub_1b4a0(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0x3ac20;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_1b4e0 @ 0x1b4e0
void sub_1b4e0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x3ac20;
  v2 = __errno_location();
  v1 = *v2;
  sub_19da0(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(unsigned long *)&a4[10],*(unsigned long *)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_1b560 @ 0x1b560
unsigned long sub_1b560(unsigned long a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x3ac20;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_19da0(0,0,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  v4 = sub_1d130(v3 + 1);
  sub_19da0(v4,v3 + 1,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_1b650 @ 0x1b650
void sub_1b650(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_1b560(a0,a1,0,a2); // tail-call
}

// Function: sub_1b660 @ 0x1b660
void sub_1b660(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_2a0e0;
  if (2 <= dat_2a0d8) {
    v4 = (unsigned long *)((long)dat_2a0e0 + 0x18);
    v1 = (unsigned long *)((long)dat_2a0e0 + (unsigned long)(unsigned int)(dat_2a0d8 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0x3ab20) {
    free(*(void **)((long)v3 + 8));
    dat_2a0f8 = 0x3ab20;
    dat_2a0f0 = 0x100;
  }
  if (v3 == (void *)0x2a0f0) {
    dat_2a0d8 = 1;
    return;
  }
  free(v3);
  dat_2a0d8 = 1;
  dat_2a0e0 = (void *)0x2a0f0;
}

// Function: sub_1b700 @ 0x1b700
void sub_1b700(unsigned long a0,unsigned long a1)
{
  sub_1b1d0(a0,a1,0xffffffffffffffff,0x3ac20); // tail-call
}

// Function: sub_1b720 @ 0x1b720
void sub_1b720(void)
{
  sub_1b1d0(); // tail-call
}

// Function: sub_1b730 @ 0x1b730
void sub_1b730(unsigned long a0)
{
  sub_1b700(0,a0); // tail-call
}

// Function: sub_1b740 @ 0x1b740
void sub_1b740(unsigned long a0,unsigned long a1)
{
  sub_1b720(0,a0,a1); // tail-call
}

// Function: sub_1b750 @ 0x1b750
void sub_1b750(unsigned int a0,unsigned long a1,unsigned long a2)
{
  char v1 [56];
  
  sub_19650(v1);
  sub_1b1d0(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_1b7b0 @ 0x1b7b0
void sub_1b7b0(unsigned int a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  char v1 [56];
  
  sub_19650(v1);
  sub_1b1d0(a0,a2,a3,v1);
}

// Function: sub_1b820 @ 0x1b820
void sub_1b820(unsigned int a0,unsigned long a1)
{
  sub_1b750(0,a0,a1); // tail-call
}

// Function: sub_1b830 @ 0x1b830
void sub_1b830(unsigned int a0,unsigned long a1,unsigned long a2)
{
  sub_1b7b0(0,a0,a1,a2); // tail-call
}

// Function: sub_1b850 @ 0x1b850
void sub_1b850(unsigned long a0,unsigned long a1,char a2,unsigned long a3)
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
  v1 = dat_3ac20;
  v3 = dat_3ac28;
  v4 = dat_3ac30;
  v5 = dat_3ac38;
  v8 = dat_3ac50;
  v6 = dat_3ac40;
  v7 = dat_3ac48;
  sub_1b440(&v1,(int)a2,1,a3,a0,a1);
  sub_1b1d0(0,a0,a1,v2);
}

// Function: sub_1b8e0 @ 0x1b8e0
void sub_1b8e0(unsigned long a0,char a1)
{
  sub_1b850(a0,0xffffffffffffffff,(int)a1); // tail-call
}

// Function: sub_1b900 @ 0x1b900
void sub_1b900(unsigned long a0)
{
  sub_1b8e0(a0,0x3a); // tail-call
}

// Function: sub_1b910 @ 0x1b910
void sub_1b910(unsigned long a0,unsigned long a1)
{
  sub_1b850(a0,a1,0x3a); // tail-call
}

// Function: sub_1b920 @ 0x1b920
void sub_1b920(unsigned int a0,unsigned long a1,unsigned long a2)
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
  
  sub_19650(&v1);
  v3 = &v2;
  v2 = v1;
  v10 = v4;
  v15 = v9;
  v11 = v5;
  v12 = v6;
  v13 = v7;
  v14 = v8;
  sub_1b440(v3,0x3a,1);
  sub_1b1d0(a0,a2,0xffffffffffffffff,v3);
}

// Function: sub_1b9c0 @ 0x1b9c0
void sub_1b9c0(unsigned int a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x68
  unsigned long v2; // stack - 0x60
  unsigned long v3; // stack - 0x58
  unsigned long v4; // stack - 0x50
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  
  v1 = dat_3ac20;
  v2 = dat_3ac28;
  v7 = dat_3ac50;
  v3 = dat_3ac30;
  v4 = dat_3ac38;
  v5 = dat_3ac40;
  v6 = dat_3ac48;
  sub_1b4a0(&v1);
  sub_1b1d0(a0,a3,a4,&v1);
}

// Function: sub_1ba60 @ 0x1ba60
void sub_1ba60(void)
{
  sub_1b9c0(); // tail-call
}

// Function: sub_1ba70 @ 0x1ba70
void sub_1ba70(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_1ba60(0,a0,a1,a2); // tail-call
}

// Function: sub_1ba90 @ 0x1ba90
void sub_1ba90(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  sub_1b9c0(0,a0,a1,a2,a3); // tail-call
}

// Function: sub_1bab0 @ 0x1bab0
void sub_1bab0(void)
{
  sub_1b1d0(); // tail-call
}

// Function: sub_1bac0 @ 0x1bac0
void sub_1bac0(unsigned long a0,unsigned long a1)
{
  sub_1bab0(0,a0,a1); // tail-call
}

// Function: sub_1bad0 @ 0x1bad0
void sub_1bad0(unsigned long a0,unsigned long a1)
{
  sub_1bab0(a0,a1,0xffffffffffffffff); // tail-call
}

// Function: sub_1bae0 @ 0x1bae0
void sub_1bae0(unsigned long a0)
{
  sub_1bad0(0,a0); // tail-call
}

// Function: sub_1baf0 @ 0x1baf0
long sub_1baf0(int a0,void *a1,unsigned long a2)
{
  int v1;
  long v2; // rax
  
  while( true ) {
    do {
      v2 = read(a0,a1,a2);
      if (0 <= v2)
        return v2;
      v1 = *__errno_location();
    } while (v1 == 4);
    if (a2 <= 0x7ff00000) break;
    if (v1 != 0x16)
      return v2;
    a2 = 0x7ff00000;
  }
  return v2;
}

// Function: sub_1bb60 @ 0x1bb60
void sub_1bb60(unsigned long a0) // return-dupe
{
  char v1 [8];
  long v2; // stack - 0x30
  
  if ((int)sub_1c900(a0,v1) <= -1)
    return;
  dat_3ac78 = v2 + -1;
}

// Function: sub_1bbb0 @ 0x1bbb0
void sub_1bbb0(unsigned long a0,long a1,long a2)
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
  if (dat_3ac58) { // branch-flip
    v4 = (*dat_3ac58)(v1,0);
    if (v4)
      return;
    if (dat_3ac70) goto label_1bbfe;
  }
  else {
    if (!dat_3ac70) {
      signal(0xb,0); // return-dupe, tail-call
      return;
    }
label_1bbfe:
    v3 = dat_3ac78;
    v2 = *(unsigned long *)(a2 + 0xa0);
    if (dat_3ac78) {
      v6 = __errno_location();
      v4 = *v6;
      v5 = sub_1c900(v3,&v7);
      *v6 = v4;
      if (0 <= v5) {
        if (v7 <= v1) { // branch-flip
          if (v1 <= (unsigned long)(v8 - 1U)) goto label_1bce5;
        }
        else if ((*v9)(v1,&v7)) {
label_1bce5:
          (*dat_3ac70)(dat_3ac68 <= v2 && v2 <= dat_3ac68 + dat_3ac60,a2);
        }
      }
    }
  }
  if ((dat_3ac58) && (v4 = (*dat_3ac58)(v1,1), v4))
    return;
  signal(0xb,0);
  return;
}

// Function: sub_1bd40 @ 0x1bd40
void sub_1bd40(void)
{
  void *v1; // stack - 0xa8
  char v2 [128];
  int v3; // stack - 0x20
  
  v1 = sub_1bbb0;
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
  v3 = (-(unsigned int)(dat_3ac70 == 0) & 0xf8000000) + 0x8000004;
  sigaction(0xb,&v1,NULL);
}

// Function: sub_1bec0 @ 0x1bec0
unsigned long sub_1bec0(unsigned long a0)
{
  dat_3ac58 = a0;
  sub_1bd40();
  return 0;
}

// Function: sub_1bee0 @ 0x1bee0
void sub_1bee0(void)
{
  dat_3ac58 = 0;
  if (dat_3ac70)
    return;
  signal(0xb,0); // tail-call
}

// Function: sub_1bf10 @ 0x1bf10
unsigned long sub_1bf10(void *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  (*a0)(a1,a2,a3);
  return 1;
}

// Function: sub_1bf40 @ 0x1bf40
unsigned long sub_1bf40(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  
  if ((!dat_3ac78) && (sub_1bb60(&v1), !dat_3ac78))
    return 0xffffffff;
  v2 = 0;
  dat_3ac60 = a2;
  dat_3ac68 = a1;
  dat_3ac70 = a0;
  v1 = a1;
  v3 = a2;
  if ((int)sigaltstack(&v1,0) <= -1)
    return 0xffffffff;
  sub_1bd40();
  return 0;
}

// Function: sub_1bff0 @ 0x1bff0
void sub_1bff0(void) // return-dupe
{
  char v1 [8];
  unsigned int v2; // stack - 0x20
  
  dat_3ac70 = 0;
  if (dat_3ac58) // branch-flip
    sub_1bd40();
  else {
    signal(0xb,0);
  }
  v2 = 2;
  if (0 <= (int)sigaltstack(v1,0))
    return;
  perror("gnulib sigsegv (stackoverflow_deinstall_handler)");
}

// Function: sub_1c080 @ 0x1c080
bool sub_1c080(long a0,long *a1)
{
  return (unsigned long)(*a1 - a0) <= (unsigned long)(*a1 - a1[3]) >> 1;
}

// Function: sub_1c0a0 @ 0x1c0a0
unsigned long sub_1c0a0(long *a0)
{
  if (*a0 != a0[1])
    return (unsigned long)*(unsigned char *)(a0[3] + *a0);
  *(unsigned int *)&a0[2] = 1;
  return 0xffffffff;
}

// Function: sub_1c0c0 @ 0x1c0c0
void sub_1c0c0(long *a0) // return-dupe
{
  if ((int)sub_1c0a0() <= -1)
    return;
  *a0 = *a0 + 1;
}

// Function: sub_1c0d0 @ 0x1c0d0
unsigned long sub_1c0d0(unsigned long a0,long *a1)
{
  int v1; // eax
  long v2; // rcx
  int v3; // esi
  
  v2 = 0;
  v3 = 0;
  do {
    v1 = sub_1c0a0();
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
    sub_1c0c0();
    v3 += 1;
  } while( true );
}

// Function: sub_1c140 @ 0x1c140
unsigned long sub_1c140(unsigned long *a0,unsigned long a1,unsigned long a2)
{
  unsigned long *v1;
  unsigned long v2;
  
  if ((a1 <= *a0) && (*a0 <= a2 - 1)) {
    v1 = (unsigned long *)a0[1];
    v1[1] = a2;
    v2 = a0[2];
    *v1 = a1;
    v1[3] = v2;
    *(unsigned int *)&a0[3] = 0;
    return 1;
  }
  a0[2] = a2;
  return 0;
}

// Function: sub_1c180 @ 0x1c180
void sub_1c180(void)
{
  dat_3ac80 = (long)getpagesize();
}

// Function: sub_1c1a0 @ 0x1c1a0
void sub_1c1a0(long a0)
{
  if (*(void **)(a0 + 0x20)) {
    munmap(*(void **)(a0 + 0x20),*(unsigned long *)(a0 + 0x28)); // tail-call
    return;
  }
}

// Function: sub_1c1c0 @ 0x1c1c0
unsigned int sub_1c1c0(unsigned long a0)
{
  unsigned int v1; // eax
  long v2;
  
  v1 = mincore(a0,dat_3ac80,&v2);
  return ~v1 >> 0x1f;
}

// Function: sub_1c210 @ 0x1c210
unsigned long sub_1c210(unsigned long a0,unsigned long a1) // return-dupe
{
  unsigned long v1;
  long v2; // rax
  unsigned long v3; // rax
  unsigned long v4;
  long v5;
  unsigned long v6; // r14
  unsigned long v7;
  
  v1 = dat_3ac80;
  v2 = (a0 / dat_3ac80) * dat_3ac80;
  v3 = ((a1 / dat_3ac80 + 1) * dat_3ac80 - v2) / dat_3ac80;
  if (v3 <= 1)
    return 1;
  v4 = 1;
  do {
    v4 *= 2;
  } while (v4 < v3);
  do {
    v6 = v4 >> 1;
    v5 = v2 + v1 * v6;
    if (v6 < v3) {
      v7 = v6;
      do {
        if (sub_1c1c0(v5))
          return 0;
        v7 += v4 & 0xfffffffffffffffe;
        v5 += v1 * v6 * 2;
      } while (v7 < v3);
    }
    v4 = v6;
  } while (2 <= v6);
  return 1;
}

// Function: sub_1c300 @ 0x1c300
unsigned long sub_1c300(unsigned long a0,long *a1)
{
  long v1;
  unsigned long v2; // rdi
  
  v1 = *a1;
  v2 = a0 * 2 - v1;
  if (a0 < v2)
    return 0;
  return sub_1c210(v2,v1 + -1); // tail-call
}

// Function: sub_1c330 @ 0x1c330
unsigned long sub_1c330(unsigned long a0)
{
  unsigned long v1;
  unsigned long v2;
  char v3 [1032];
  long v4; // rsi
  unsigned long v5; // r14
  unsigned long v6; // r15
  
  v2 = 0x400;
  while( true ) {
    v1 = dat_3ac80;
    if (!a0)
      return 0;
    if (a0 / dat_3ac80 < v2)
      v2 = a0 / dat_3ac80;
    v4 = dat_3ac80 * v2;
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

// Function: sub_1c410 @ 0x1c410
long sub_1c410(long a0) // return-dupe
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  char v4 [1032];
  
  v1 = dat_3ac80;
  a0 += dat_3ac80;
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

// Function: sub_1c510 @ 0x1c510
unsigned long sub_1c510(unsigned long a0,unsigned long *a1)
{
  long v1; // rbp
  
  if (!dat_3ac80)
    sub_1c180();
  v1 = (a0 / dat_3ac80) * dat_3ac80;
  *a1 = sub_1c330(v1);
  a1[1] = sub_1c410(v1);
  a1[2] = sub_1c300;
  return 0;
}

// Function: sub_1c580 @ 0x1c580
unsigned long sub_1c580(unsigned long *a0) // return-dupe x2
{
  void *v1;
  int v2; // eax
  int v3; // eax
  long v4;
  unsigned long v5; // rax
  unsigned long v6; // r12
  unsigned long v7;
  
  v2 = open("/proc/self/maps",0);
  if (v2 <= -1)
    return 0xffffffff;
  *a0 = 0;
  v7 = 1;
  *(unsigned int *)&a0[2] = 0;
  v6 = 0;
  a0[3] = &a0[8];
  a0[4] = 0;
  a0[6] = 0;
  a0[7] = 0;
  do {
    if (0x104a <= v7) { // branch-flip
      v1 = (void *)a0[3];
      v3 = (int)read(v2,v1,v7);
      if (0 <= v3) {
        if (!v3) goto label_1c74c;
        if (v7 < (unsigned long)(long)(v3 + 0x1049)) goto label_1c61f;
        v4 = (long)v3;
        while( true ) {
          a0[1] = v4;
          v5 = read(v2,(void *)(v4 + a0[3]),v7 - v4);
          v3 = (int)v5;
          if (v3 <= -1) break;
          if (v7 - a0[1] < (unsigned long)(long)(v3 + 0x1049)) goto label_1c61f;
          if (!v3) {
            close(v2);
            return v5 & 0xffffffff;
          }
          v4 = (long)v3 + a0[1];
        }
      }
      if (*__errno_location() != 4) {
label_1c74c:
        close(v2);
        v1 = (void *)a0[4];
        goto label_1c75a;
      }
    }
    else {
label_1c61f:
      if (v6) { // branch-flip
        v7 *= 2;
        if (!v7) goto label_1c74c;
        if ((void *)a0[4])
          munmap((void *)a0[4],a0[5]);
      }
      else {
        v6 = (unsigned long)getpagesize();
        for (v7 = v6; v7 <= 0x1049; v7 = v7 * 2) {
        }
      }
      v4 = mmap(0,v7,3,0x22,0xffffffff,0);
      a0[4] = v4;
      if (v4 == -1) {
        close(v2);
        return 0xffffffff;
      }
      a0[5] = v7;
      a0[6] = v4;
      a0[7] = v7 + v4;
      a0[3] = v4;
    }
    if ((long)lseek(v2,0,0) <= -1) {
      close(v2);
      v2 = open("/proc/self/maps",0);
      if (v2 <= -1) {
        v1 = (void *)a0[4];
label_1c75a:
        if (!v1)
          return 0xffffffff;
        munmap(v1,a0[5]);
        return 0xffffffff;
      }
    }
  } while( true );
}

// Function: sub_1c7c0 @ 0x1c7c0
unsigned long sub_1c7c0(unsigned long a0) // return-dupe
{
  unsigned long v1;
  unsigned long v10; // stack - 0x48
  long v11; // stack - 0x40
  int v2; // eax
  unsigned long v3; // stack - 0x88
  char v4 [48];
  long v5; // stack - 0x80
  long v6;
  unsigned long v7;
  unsigned long *v8;
  long *v9;
  
  v8 = &v3;
  if ((int)sub_1c580(v4) <= -1)
    return 0xffffffff;
  v9 = &v5;
  v7 = v10;
label_1c810:
  do {
    v2 = sub_1c0d0(v4,v8);
    if (((v2 < 0) || (sub_1c0c0() != 0x2d)) || (v2 = sub_1c0d0(v9), v2 < 0)) {
      sub_1c1a0(v4); // return-dupe
      return 0;
    }
    do {
      v2 = sub_1c0c0(v4);
      if (v2 == -1) break;
    } while (v2 != 10);
    if (v3 <= v7) { // branch-flip
      v1 = v5 - 1;
      if (v1 < (unsigned long)(v11 - 1U)) goto label_1c8b0;
      v6 = v5;
      if ((v3 < v7) && (v2 = sub_1c140(a0,v3,v7), v2)) {
        sub_1c1a0(v4);
        return 0;
      }
      if (v1 <= (unsigned long)(v11 - 1U)) goto label_1c810;
      v2 = sub_1c140(a0,v11,v6);
    }
    else {
label_1c8b0:
      v2 = sub_1c140(a0,v3,v5);
    }
    if (v2) {
      sub_1c1a0(v4);
      return 0;
    }
  } while( true );
}

// Function: sub_1c8f0 @ 0x1c8f0
void sub_1c8f0(void)
{
  sub_1c7c0(); // tail-call
}

// Function: sub_1c900 @ 0x1c900
unsigned long sub_1c900(unsigned long a0,long a1)
{
  unsigned long v1; // stack - 0x48
  long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  int v4; // stack - 0x30
  
  v3 = 0;
  v4 = -1;
  v1 = a0;
  v2 = a1;
  sub_1c8f0(&v1);
  if (!v4) {
    *(void **)(a1 + 0x10) = sub_1c080;
    return 0;
  }
  return sub_1c510(a0,a1); // tail-call
}

// Function: sub_1c9a0 @ 0x1c9a0
void * sub_1c9a0(void *a0,unsigned long a1)
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

// Function: sub_1c9e0 @ 0x1c9e0
unsigned int sub_1c9e0(unsigned int a0)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  
  if (3 <= a0)
    return a0;
  v2 = sub_1e1b0();
  v3 = __errno_location();
  v1 = *v3;
  close(a0);
  *v3 = v1;
  return v2;
}

// Function: sub_1ca40 @ 0x1ca40
long sub_1ca40(void *a0,long a1,unsigned long a2,unsigned long a3,unsigned long *a4,unsigned long a5)
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
label_1cb6d:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_1cba6:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0x21970) + 0x21970;
  switch(a5) {
    case 0:
      goto label_1cba6;
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
      goto label_1cb6d;
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

// Function: sub_1cec0 @ 0x1cec0
void sub_1cec0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_1ca40(); // tail-call
}

// Function: sub_1cee0 @ 0x1cee0
void sub_1cee0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
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
  sub_1ca40(a0,a1,a2,a3);
}

// Function: sub_1cf80 @ 0x1cf80
void sub_1cf80(void)
{
  sub_1cee0();
}

// Function: sub_1d030 @ 0x1d030
void sub_1d030(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-grep@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU grep","https://www.gnu.org/software/grep/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_1d0e0 @ 0x1d0e0
long sub_1d0e0(long a0)
{
  if (a0)
    return a0;
  sub_1d4b0(); // no-return
}

// Function: sub_1d0f0 @ 0x1d0f0
void sub_1d0f0(unsigned long a0)
{
  sub_1d0e0(malloc(a0)); // tail-call
}

// Function: sub_1d110 @ 0x1d110
void sub_1d110(void)
{
  sub_1d0e0(sub_1df70()); // tail-call
}

// Function: sub_1d130 @ 0x1d130
void sub_1d130(void)
{
  sub_1d0f0(); // tail-call
}

// Function: sub_1d140 @ 0x1d140
void sub_1d140(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_1d4b0(); // no-return
}

// Function: sub_1d180 @ 0x1d180
void sub_1d180(void)
{
  sub_1d0e0(sub_1df80()); // tail-call
}

// Function: sub_1d1a0 @ 0x1d1a0
void sub_1d1a0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_1d4b0(); // no-return
}

// Function: sub_1d1e0 @ 0x1d1e0
void sub_1d1e0(void)
{
  sub_1d1a0(); // tail-call
}

// Function: sub_1d1f0 @ 0x1d1f0
void sub_1d1f0(void)
{
  sub_1d0e0(sub_1dfb0()); // tail-call
}

// Function: sub_1d210 @ 0x1d210
void sub_1d210(unsigned long a0,unsigned long a1)
{
  sub_1d1a0(0,a0,a1); // tail-call
}

// Function: sub_1d220 @ 0x1d220
void sub_1d220(unsigned long a0,unsigned long a1)
{
  sub_1d1f0(0,a0,a1); // tail-call
}

// Function: sub_1d230 @ 0x1d230
void sub_1d230(long a0,unsigned long *a1,unsigned long a2)
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
      sub_1d4b0(); // no-return
  }
  else if (!v3) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v3 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0) + (unsigned long)(0x81 <= a2);
  }
  sub_1d1a0(a0,v3,a2);
  *a1 = v3;
}

// Function: sub_1d2a0 @ 0x1d2a0
void sub_1d2a0(unsigned long a0,unsigned long a1)
{
  sub_1d230(a0,a1,1); // tail-call
}

// Function: sub_1d2b0 @ 0x1d2b0
void sub_1d2b0(long a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_1d356;
    v3 = 0x80;
  }
  v2 = v3 / a4;
  v3 -= v3 % a4;
label_1d356:
  if (!a0)
    *a1 = 0;
  if ((v2 - v1 < a2) && ((v2 = v1 + a2, SCARRY8(v1,a2) || (((a3 < v2 && (0 <= a3)) || (v3 = v2 * a4, SEXT816(v3) != SEXT816(v2) * SEXT816(a4)))))))
    sub_1d4b0(a0,v3); // no-return
  sub_1d140(a0,v3);
  *a1 = v2;
}

// Function: sub_1d390 @ 0x1d390
void sub_1d390(unsigned long a0,unsigned long a1)
{
  sub_1d0e0(calloc(a0,a1)); // tail-call
}

// Function: sub_1d3b0 @ 0x1d3b0
void sub_1d3b0(unsigned long a0)
{
  sub_1d390(a0,1); // tail-call
}

// Function: sub_1d3c0 @ 0x1d3c0
void sub_1d3c0(void)
{
  sub_1d0e0(sub_1dfa0()); // tail-call
}

// Function: sub_1d3e0 @ 0x1d3e0
void sub_1d3e0(unsigned long a0)
{
  sub_1d3c0(a0,1); // tail-call
}

// Function: sub_1d3f0 @ 0x1d3f0
void sub_1d3f0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_1d0f0(a1),a0,a1); // tail-call
}

// Function: sub_1d420 @ 0x1d420
void sub_1d420(void *a0,unsigned long a1)
{
  memcpy((void *)sub_1d110(a1),a0,a1); // tail-call
}

// Function: sub_1d450 @ 0x1d450
void sub_1d450(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_1d110(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_1d490 @ 0x1d490
void sub_1d490(char *a0)
{
  sub_1d3f0(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_1d4b0 @ 0x1d4b0
void sub_1d4b0(void)
{
  error(dat_2a080,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_1d4f0 @ 0x1d4f0
unsigned long sub_1d4f0(long *a0,int a1)
{
  long v1;
  unsigned long v2; // rax
  long v3; // rsi
  
  v1 = *a0;
  v3 = a1 * v1;
  if (SEXT816(v3) != SEXT816((long)a1) * SEXT816(v1)) { // branch-flip
    v3 = -0x8000000000000000;
    if (0 <= v1)
      v3 = 0x7fffffffffffffff;
    v2 = 1;
  }
  else {
    v2 = 0;
  }
  *a0 = v3;
  return v2;
}

// Function: sub_1d530 @ 0x1d530
unsigned int sub_1d530(unsigned long a0,unsigned long a1,int a2)
{
  unsigned int v1; // eax
  unsigned int v2; // r8d
  
  a1 &= 0xffffffff;
  v2 = 0;
  do {
    a2 -= 1;
    v1 = sub_1d4f0(a0,a1 & 0xffffffff);
    v2 |= v1;
  } while (a2);
  return v2;
}

// Function: sub_1d560 @ 0x1d560
unsigned int sub_1d560(unsigned char *a0,unsigned long *a1,unsigned int a2,unsigned long *a3,char *a4)
{
  unsigned char v1;
  unsigned int v10;
  unsigned char v2;
  unsigned char *v3;
  unsigned int v4; // eax
  int *v5; // rax
  long v6;
  unsigned long v7; // stack - 0x48
  int v8; // ecx
  unsigned long v9; // r8
  
  if (0x25 <= a2)
    __assert_fail("0 <= strtol_base && strtol_base <= 36","xstrtol.c",0x55,"xstrtoimax"); // no-return
  if (!a1)
    a1 = &v6;
  v5 = __errno_location();
  *v5 = 0;
  v7 = strtoimax(a0,a1,a2);
  v3 = (unsigned char *)*a1;
  if (v3 != a0) { // branch-flip
    if (*v5) { // branch-flip
      if (*v5 != 0x22)
        return 4;
      v10 = 1;
    }
    else {
      v10 = 0;
    }
    if ((!a4) || (v1 = *v3, !v1)) goto label_1d5ec;
    v8 = (int)(char)v1;
  }
  else {
    if (!a4)
      return 4;
    v1 = *v3;
    if (!v1)
      return 4;
    v8 = (int)(char)v1;
    if (!strchr(a4,v8))
      return 4;
    v7 = 1;
    v10 = 0;
  }
  if (strchr(a4,v8)) {
    v4 = (unsigned int)v1 - 0x45;
    v8 = 1;
    v9 = 0x400;
    if (((unsigned char)v4 <= 0x2f) && (0x814400308945U >> ((unsigned long)v4 & 0x3f) & 1)) {
      v9 = 0x400;
      v8 = 1;
      if (strchr(a4,0x30)) {
        v2 = v3[1];
        if (v2 != 0x44) { // branch-flip
          if (v2 != 0x69) { // branch-flip
            v8 = (v2 == 0x42) + 1;
            v9 = 0x400;
            if (v2 == 0x42)
              v9 = 1000;
          }
          else {
            v8 = (v3[2] == 0x42) + 1 + (unsigned int)(v3[2] == 0x42);
          }
        }
        else {
          v8 = 2;
          v9 = 1000;
        }
      }
    }
    switch((unsigned int)v1 - 0x42 & 0xff) {
      case 0:
        v10 |= sub_1d4f0(&v7,0x400);
        break;
      default:
        goto label_1d5ef;
      case 3:
        v10 |= sub_1d530(&v7,v9,6);
        break;
      case 5:
      case 0x25:
        v10 |= sub_1d530(&v7,v9,3);
        break;
      case 9:
      case 0x29:
        v10 |= sub_1d530(&v7,v9,1);
        break;
      case 0xb:
      case 0x2b:
        v10 |= sub_1d530(&v7,v9,2);
        break;
      case 0xe:
        v10 |= sub_1d530(&v7,v9,5);
        break;
      case 0x12:
      case 0x32:
        v10 |= sub_1d530(&v7,v9,4);
        break;
      case 0x17:
        v10 |= sub_1d530(&v7,v9,8);
        break;
      case 0x18:
        v10 |= sub_1d530(&v7,v9,7);
        break;
      case 0x20:
        v10 |= sub_1d4f0(&v7,0x200);
        break;
      case 0x21:
        break;
      case 0x35:
        v10 |= sub_1d4f0(&v7,2);
      
    }
    *a1 = &v3[v8];
    if (v3[v8])
      v10 |= 2;
label_1d5ec:
    *a3 = v7;
    return v10;
  }
label_1d5ef:
  *a3 = v7;
  return v10 | 2;
}

// Function: sub_1d920 @ 0x1d920
bool sub_1d920(void) // early-return
{
  char *v1; // rax
  
  v1 = getenv("TERM");
  if (!v1)
    return 0;
  return strcmp(v1,"dumb") != 0;
}

// Function: sub_1d960 @ 0x1d960
void sub_1d960(void)
{
  return;
}

// Function: sub_1d970 @ 0x1d970
void sub_1d970(char *a0,unsigned long a1)
{
  __printf_chk(1,a0,a1); // tail-call
}

// Function: sub_1d990 @ 0x1d990
void sub_1d990(char *a0)
{
  fputs(a0,stdout); // tail-call
}

// Function: sub_1d9a0 @ 0x1d9a0
char * sub_1d9a0(char *a0)
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

// Function: sub_1da00 @ 0x1da00
void sub_1da00(char *a0)
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

// Function: sub_1da30 @ 0x1da30
unsigned long sub_1da30(unsigned long a0,unsigned char a1)
{
  return a0 << (a1 & 0x3f) | a0 >> 0x40 - (a1 & 0x3f);
}

// Function: sub_1da40 @ 0x1da40
unsigned long sub_1da40(unsigned long a0,unsigned char a1)
{
  return a0 >> (a1 & 0x3f) | a0 << 0x40 - (a1 & 0x3f);
}

// Function: sub_1da50 @ 0x1da50
unsigned int sub_1da50(unsigned int a0,unsigned char a1)
{
  return a0 << (a1 & 0x1f) | a0 >> 0x20 - (a1 & 0x1f);
}

// Function: sub_1da60 @ 0x1da60
unsigned int sub_1da60(unsigned int a0,unsigned char a1)
{
  return a0 >> (a1 & 0x1f) | a0 << 0x20 - (a1 & 0x1f);
}

// Function: sub_1da70 @ 0x1da70
void sub_1da70(void)
{
  sub_1da30(); // tail-call
}

// Function: sub_1da80 @ 0x1da80
void sub_1da80(void)
{
  sub_1da40(); // tail-call
}

// Function: sub_1da90 @ 0x1da90
unsigned int sub_1da90(unsigned int a0,unsigned char a1)
{
  return CONCAT22((short)((unsigned int)a0 >> 0x10),(unsigned short)a0 << (a1 & 0xf) | (unsigned short)a0 >> 0x10 - (a1 & 0xf));
}

// Function: sub_1daa0 @ 0x1daa0
unsigned int sub_1daa0(unsigned int a0,unsigned char a1)
{
  return CONCAT22((short)((unsigned int)a0 >> 0x10),(unsigned short)a0 >> (a1 & 0xf) | (unsigned short)a0 << 0x10 - (a1 & 0xf));
}

// Function: sub_1dab0 @ 0x1dab0
unsigned int sub_1dab0(unsigned int a0,unsigned char a1)
{
  return CONCAT31((undefined3)((unsigned int)a0 >> 8),(unsigned char)a0 << (a1 & 7) | (unsigned char)a0 >> 8 - (a1 & 7));
}

// Function: sub_1dac0 @ 0x1dac0
unsigned int sub_1dac0(unsigned int a0,unsigned char a1)
{
  return CONCAT31((undefined3)((unsigned int)a0 >> 8),(unsigned char)a0 >> (a1 & 7) | (unsigned char)a0 << 8 - (a1 & 7));
}

// Function: sub_1dad0 @ 0x1dad0
unsigned long sub_1dad0(void *a0) // early-return x2, return-dupe
{
  int v1; // eax
  int v2; // eax
  unsigned long v3; // rax
  
  v3 = __fpending(a0);
  v1 = ferror_unlocked(a0);
  v2 = fclose(a0);
  if (v1) {
    if (v2)
      return 0xffffffff;
    *__errno_location() = 0;
    return 0xffffffff;
  }
  if (v2) {
    if (v3)
      return 0xffffffff;
    return (unsigned long)-(unsigned int)(*__errno_location() != 9);
  }
  return CONCAT44(dat_4,v2);
}

// Function: sub_1db40 @ 0x1db40
unsigned long sub_1db40(unsigned long a0)
{
  return CONCAT71((undefined7)(a0 - 1 >> 8),(a0 - 1 & a0) == 0);
}

// Function: sub_1db50 @ 0x1db50
void sub_1db50(long a0)
{
  *(unsigned long *)(a0 + 0x10) = 0;
  *(unsigned int *)(a0 + 0x18) = 0x95f616;
}

// Function: sub_1db70 @ 0x1db70
unsigned long sub_1db70(long *a0,long *a1) // return-dupe x3
{
  char v1; // al
  long v2;
  long *v3; // rdx
  long *v4; // rdx
  long *v5;
  long v6;
  
  if ((int)a0[3] != 0x95f616)
    __assert_fail("state->magic == 9827862","cycle-check.c",0x3c,"cycle_check"); // no-return
  if (a0[2]) { // branch-flip
    v6 = a1[1];
    if ((v6 == *a0) && (*a1 == a0[1]))
      return 1;
    v2 = a0[2] + 1;
    a0[2] = v2;
    v1 = sub_1db40(v2);
    if (!v1)
      return 0;
    v5 = v3;
    if (!v2)
      return 1;
  }
  else {
    a0[2] = 1;
    if (!sub_1db40(1))
      return 0;
    v6 = a1[1];
    v5 = v4;
  }
  v2 = *a1;
  *v5 = v6;
  v5[1] = v2;
  return 0;
}

// Function: sub_1dc20 @ 0x1dc20
void sub_1dc20(int a0,unsigned long a1)
{
  fcntl(a0,0,a1 & 0xffffffff); // tail-call
}

// Function: sub_1dc30 @ 0x1dc30
int sub_1dc30(unsigned long a0,unsigned long a1)
{
  int v1;
  int v2; // eax
  unsigned int v3; // eax
  int *v4; // rax
  
  if (0 <= dat_3ac88) { // branch-flip
    v2 = fcntl((int)a0,0x406,a1 & 0xffffffff);
    if ((0 <= v2) || (*__errno_location() != 0x16)) {
      dat_3ac88 = 1;
      return v2;
    }
    v2 = sub_1dc20(a0 & 0xffffffff,a1 & 0xffffffff);
    if (v2 < 0)
      return v2;
    dat_3ac88 = -1;
  }
  else {
    v2 = sub_1dc20(a0,a1);
    if (v2 < 0)
      return v2;
    if (dat_3ac88 != -1)
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

// Function: sub_1dd10 @ 0x1dd10
void sub_1dd10(int a0,int a1,unsigned int a2)
{
  unsigned long v1;
  unsigned long v2; // stack - 0x28
  
  v2 = CONCAT44(v2._4_4_,a2);
  if (!a1) {
    sub_1dc20(a0,a2);
    return;
  }
  if (a1 == 0x406) {
    sub_1dc30(a0,a2);
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

// Function: sub_1de60 @ 0x1de60
void sub_1de60(char *a0,unsigned int a1,unsigned long a2)
{
  unsigned long v1; // rdx
  
  v1 = 0;
  if (a1 & 0x40)
    v1 = a2 & 0xffffffff;
  sub_1c9e0(open(a0,a1,v1));
}

// Function: sub_1dee0 @ 0x1dee0
bool sub_1dee0(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_1e190(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_1df70 @ 0x1df70
void sub_1df70(unsigned long a0)
{
  malloc(a0); // tail-call
}

// Function: sub_1df80 @ 0x1df80
void sub_1df80(void *a0,unsigned long a1)
{
  realloc(a0,a1 | a1 == 0); // tail-call
}

// Function: sub_1dfa0 @ 0x1dfa0
void sub_1dfa0(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1); // tail-call
}

// Function: sub_1dfb0 @ 0x1dfb0
void sub_1dfb0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((a1) && (a2)) {
    reallocarray(a0,a1,a2); // tail-call
    return;
  }
  reallocarray(a0,1,1); // tail-call
}

// Function: sub_1dfe0 @ 0x1dfe0
char * sub_1dfe0(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_1e020 @ 0x1e020
int sub_1e020(int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (0 <= v1)
    return v1;
  return (unsigned long)(iswcntrl(a0) == 0);
}

// Function: sub_1e050 @ 0x1e050
void sub_1e050(long *a0,long *a1) // return-dupe
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

// Function: sub_1e0b0 @ 0x1e0b0
unsigned int sub_1e0b0(unsigned char a0)
{
  return *(unsigned int *)((unsigned long)(a0 >> 5) * 4 + 0x21b40) >> (a0 & 0x1f) & 1;
}

// Function: sub_1e0d0 @ 0x1e0d0
void sub_1e0d0(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_1e0e0 @ 0x1e0e0
unsigned long sub_1e0e0(unsigned long a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_1e0d0();
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

// Function: sub_1e190 @ 0x1e190
void sub_1e190(void)
{
  sub_1e0e0(); // tail-call
}

// Function: sub_1e1a0 @ 0x1e1a0
void sub_1e1a0(void)
{
  sub_1e0d0(); // tail-call
}

// Function: sub_1e1b0 @ 0x1e1b0
void sub_1e1b0(unsigned long a0)
{
  sub_1dd10(a0,0,3); // tail-call
}

// Function: sub_1e1d0 @ 0x1e1d0
void sub_1e1d0(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_2a008); // tail-call
}

// Function: _DT_FINI @ 0x1e1e4
void _DT_FINI(void)
{
  return;
}
