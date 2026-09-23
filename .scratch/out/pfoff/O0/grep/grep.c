// Function: _DT_INIT @ 0x4000
void _DT_INIT(void) // return-dupe
{
  if (!dat_32fe8)
    return;
  (*dat_32fe8)();
}

// Function: sub_4020 @ 0x4020
void sub_4020(void)
{
  (*dat_32bc8)(); // jump-as-call
}

// Function: free @ 0x4820
void free(void *a0)
{
  (*dat_32fc8)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x4830
void __cxa_finalize(void)
{
  (*dat_32ff8)(); // jump-as-call
}

// Function: getenv @ 0x4840
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_32bd0)(); // jump-as-call
  return v1;
}

// Function: raise @ 0x4850
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_32bd8)(); // jump-as-call
  return v1;
}

// Function: strtoimax @ 0x4860
void strtoimax(void)
{
  (*dat_32be0)(); // jump-as-call
}

// Function: abort @ 0x4870
void abort(void)
{
  (*dat_32be8)(); // jump-as-call
}

// Function: re_match @ 0x4880
int re_match(re_pattern_buffer *a0,char *a1,int a2,int a3,void *a4)
{
  int v1; // eax
  
  v1 = (*dat_32bf0)(); // jump-as-call
  return v1;
}

// Function: __errno_location @ 0x4890
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_32bf8)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x48a0
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_32c00)(); // jump-as-call
  return v1;
}

// Function: splice @ 0x48b0
long splice(int a0,void *a1,int a2,void *a3,unsigned long a4,unsigned int a5)
{
  long v1; // rax
  
  v1 = (*dat_32c08)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x48c0
void _exit(int a0)
{
  (*dat_32c10)(); // jump-as-call
}

// Function: strcpy @ 0x48d0
char * strcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_32c18)(); // jump-as-call
  return v1;
}

// Function: __fpending @ 0x48e0
unsigned long __fpending(FILE *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_32c20)(); // jump-as-call
  return v1;
}

// Function: toupper @ 0x48f0
int toupper(int a0)
{
  int v1; // eax
  
  v1 = (*dat_32c28)(); // jump-as-call
  return v1;
}

// Function: puts @ 0x4900
int puts(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_32c30)(); // jump-as-call
  return v1;
}

// Function: ferror @ 0x4910
int ferror(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_32c38)(); // jump-as-call
  return v1;
}

// Function: qsort @ 0x4920
void qsort(void)
{
  (*dat_32c40)(); // jump-as-call
}

// Function: isatty @ 0x4930
int isatty(int a0)
{
  int v1; // eax
  
  v1 = (*dat_32c48)(); // jump-as-call
  return v1;
}

// Function: sigaction @ 0x4940
int sigaction(int a0,sigaction *a1,sigaction *a2)
{
  int v1; // eax
  
  v1 = (*dat_32c50)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x4950
int iswcntrl(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_32c58)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x4960
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_32c60)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x4970
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_32c68)(); // jump-as-call
  return v1;
}

// Function: clearerr_unlocked @ 0x4980
void clearerr_unlocked(FILE *a0)
{
  (*dat_32c70)(); // jump-as-call
}

// Function: write @ 0x4990
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_32c78)(); // jump-as-call
  return v1;
}

// Function: fread_unlocked @ 0x49a0
unsigned long fread_unlocked(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_32c80)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x49b0
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_32c88)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x49c0
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_32c90)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x49d0
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_32c98)(); // jump-as-call
  return v1;
}

// Function: regfree @ 0x49e0
void regfree(re_pattern_buffer *a0)
{
  (*dat_32ca0)(); // jump-as-call
}

// Function: __ctype_get_mb_cur_max @ 0x49f0
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_32ca8)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x4a00
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_32cb0)(); // jump-as-call
  return v1;
}

// Function: openat @ 0x4a10
int openat(int a0,char *a1,int a2,...)
{
  int v1; // eax
  
  v1 = (*dat_32cb8)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x4a20
void __stack_chk_fail(void)
{
  (*dat_32cc0)(); // jump-as-call
}

// Function: mmap @ 0x4a30
void mmap(void)
{
  (*dat_32cc8)(); // jump-as-call
}

// Function: getopt_long @ 0x4a40
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_32cd0)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x4a50
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_32cd8)(); // jump-as-call
  return v1;
}

// Function: gettext @ 0x4a60
char * gettext(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_32ce0)(); // jump-as-call
  return v1;
}

// Function: pcre2_jit_stack_assign_8 @ 0x4a70
void pcre2_jit_stack_assign_8(void)
{
  (*dat_32ce8)(); // jump-as-call
}

// Function: strchr @ 0x4a80
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_32cf0)(); // jump-as-call
  return v1;
}

// Function: printf @ 0x4a90
int printf(char *a0,...)
{
  int v1; // eax
  
  v1 = (*dat_32cf8)(); // jump-as-call
  return v1;
}

// Function: snprintf @ 0x4aa0
int snprintf(char *a0,unsigned long a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_32d00)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x4ab0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_32d08)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x4ac0
void lseek(void)
{
  (*dat_32d10)(); // jump-as-call
}

// Function: pcre2_compile_context_free_8 @ 0x4ad0
void pcre2_compile_context_free_8(void)
{
  (*dat_32d18)(); // jump-as-call
}

// Function: __assert_fail @ 0x4ae0
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_32d20)(); // jump-as-call
}

// Function: fputs @ 0x4af0
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_32d28)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x4b00
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_32d30)(); // jump-as-call
  return v1;
}

// Function: pcre2_set_compile_extra_options_8 @ 0x4b10
void pcre2_set_compile_extra_options_8(void)
{
  (*dat_32d38)(); // jump-as-call
}

// Function: close @ 0x4b20
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_32d40)(); // jump-as-call
  return v1;
}

// Function: pcre2_set_character_tables_8 @ 0x4b30
void pcre2_set_character_tables_8(void)
{
  (*dat_32d48)(); // jump-as-call
}

// Function: closedir @ 0x4b40
int closedir(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_32d50)(); // jump-as-call
  return v1;
}

// Function: pcre2_general_context_create_8 @ 0x4b50
void pcre2_general_context_create_8(void)
{
  (*dat_32d58)(); // jump-as-call
}

// Function: regcomp @ 0x4b60
int regcomp(re_pattern_buffer *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_32d60)(); // jump-as-call
  return v1;
}

// Function: memchr @ 0x4b70
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_32d68)(); // jump-as-call
  return v1;
}

// Function: read @ 0x4b80
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_32d70)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x4b90
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_32d78)(); // jump-as-call
  return v1;
}

// Function: pcre2_jit_stack_free_8 @ 0x4ba0
void pcre2_jit_stack_free_8(void)
{
  (*dat_32d80)(); // jump-as-call
}

// Function: fputs_unlocked @ 0x4bb0
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_32d88)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x4bc0
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_32d90)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x4bd0
int ferror_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_32d98)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x4be0
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_32da0)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x4bf0
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_32da8)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x4c00
void signal(void)
{
  (*dat_32db0)(); // jump-as-call
}

// Function: dirfd @ 0x4c10
int dirfd(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_32db8)(); // jump-as-call
  return v1;
}

// Function: fprintf @ 0x4c20
int fprintf(FILE *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_32dc0)(); // jump-as-call
  return v1;
}

// Function: sigemptyset @ 0x4c30
int sigemptyset(sigset_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_32dc8)(); // jump-as-call
  return v1;
}

// Function: pcre2_get_ovector_pointer_8 @ 0x4c40
void pcre2_get_ovector_pointer_8(void)
{
  (*dat_32dd0)(); // jump-as-call
}

// Function: stat @ 0x4c50
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_32dd8)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x4c60
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_32de0)(); // jump-as-call
  return v1;
}

// Function: putchar_unlocked @ 0x4c70
int putchar_unlocked(int a0)
{
  int v1; // eax
  
  v1 = (*dat_32de8)(); // jump-as-call
  return v1;
}

// Function: readdir @ 0x4c80
dirent * readdir(DIR *a0)
{
  dirent *v1; // rax
  
  v1 = (dirent *)(*dat_32df0)(); // jump-as-call
  return v1;
}

// Function: pcre2_set_depth_limit_8 @ 0x4c90
void pcre2_set_depth_limit_8(void)
{
  (*dat_32df8)(); // jump-as-call
}

// Function: mincore @ 0x4ca0
int mincore(void *a0,unsigned long a1,char *a2)
{
  int v1; // eax
  
  v1 = (*dat_32e00)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x4cb0
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_32e08)(); // jump-as-call
  return v1;
}

// Function: tolower @ 0x4cc0
int tolower(int a0)
{
  int v1; // eax
  
  v1 = (*dat_32e10)(); // jump-as-call
  return v1;
}

// Function: putc_unlocked @ 0x4cd0
int putc_unlocked(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_32e18)(); // jump-as-call
  return v1;
}

// Function: re_set_syntax @ 0x4ce0
void re_set_syntax(void)
{
  (*dat_32e20)(); // jump-as-call
}

// Function: iswalnum @ 0x4cf0
int iswalnum(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_32e28)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x4d00
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_32e30)(); // jump-as-call
  return v1;
}

// Function: wcrtomb @ 0x4d10
unsigned long wcrtomb(char *a0,int a1,mbstate_t *a2)
{
  unsigned long v1; // rax
  
  v1 = (*dat_32e38)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x4d20
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_32e40)(); // jump-as-call
  return v1;
}

// Function: pcre2_jit_stack_create_8 @ 0x4d30
void pcre2_jit_stack_create_8(void)
{
  (*dat_32e48)(); // jump-as-call
}

// Function: pcre2_get_startchar_8 @ 0x4d40
void pcre2_get_startchar_8(void)
{
  (*dat_32e50)(); // jump-as-call
}

// Function: regexec @ 0x4d50
int regexec(re_pattern_buffer *a0,char *a1,unsigned long a2,void *a3,int a4)
{
  int v1; // eax
  
  v1 = (*dat_32e58)(); // jump-as-call
  return v1;
}

// Function: strcoll @ 0x4d60
int strcoll(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_32e60)(); // jump-as-call
  return v1;
}

// Function: re_search @ 0x4d70
int re_search(re_pattern_buffer *a0,char *a1,int a2,int a3,int a4,void *a5)
{
  int v1; // eax
  
  v1 = (*dat_32e68)(); // jump-as-call
  return v1;
}

// Function: pcre2_get_error_message_8 @ 0x4d80
void pcre2_get_error_message_8(void)
{
  (*dat_32e70)(); // jump-as-call
}

// Function: fchdir @ 0x4d90
int fchdir(int a0)
{
  int v1; // eax
  
  v1 = (*dat_32e78)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x4da0
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_32e80)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x4db0
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_32e88)(); // jump-as-call
  return v1;
}

// Function: munmap @ 0x4dc0
int munmap(void *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = (*dat_32e90)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x4dd0
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_32e98)(); // jump-as-call
  return v1;
}

// Function: iswspace @ 0x4de0
int iswspace(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_32ea0)(); // jump-as-call
  return v1;
}

// Function: pcre2_maketables_8 @ 0x4df0
void pcre2_maketables_8(void)
{
  (*dat_32ea8)(); // jump-as-call
}

// Function: mempcpy @ 0x4e00
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_32eb0)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x4e10
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_32eb8)(); // jump-as-call
  return v1;
}

// Function: error @ 0x4e20
void error(int a0,int a1,char *a2,...)
{
  (*dat_32ec0)(); // jump-as-call
}

// Function: memrchr @ 0x4e30
void * memrchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_32ec8)(); // jump-as-call
  return v1;
}

// Function: pcre2_match_8 @ 0x4e40
void pcre2_match_8(void)
{
  (*dat_32ed0)(); // jump-as-call
}

// Function: open @ 0x4e50
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_32ed8)(); // jump-as-call
  return v1;
}

// Function: pcre2_compile_context_create_8 @ 0x4e60
void pcre2_compile_context_create_8(void)
{
  (*dat_32ee0)(); // jump-as-call
}

// Function: fopen @ 0x4e70
FILE * fopen(char *a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_32ee8)(); // jump-as-call
  return v1;
}

// Function: perror @ 0x4e80
void perror(char *a0)
{
  (*dat_32ef0)(); // jump-as-call
}

// Function: sysconf @ 0x4e90
long sysconf(int a0)
{
  long v1; // rax
  
  v1 = (*dat_32ef8)(); // jump-as-call
  return v1;
}

// Function: towlower @ 0x4ea0
unsigned int towlower(unsigned int a0)
{
  unsigned int v1; // eax
  
  v1 = (*dat_32f00)(); // jump-as-call
  return v1;
}

// Function: wctob @ 0x4eb0
int wctob(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_32f08)(); // jump-as-call
  return v1;
}

// Function: fdopendir @ 0x4ec0
DIR * fdopendir(int a0)
{
  DIR *v1; // rax
  
  v1 = (DIR *)(*dat_32f10)(); // jump-as-call
  return v1;
}

// Function: vfprintf @ 0x4ed0
int vfprintf(FILE *a0,char *a1,void *a2)
{
  int v1; // eax
  
  v1 = (*dat_32f18)(); // jump-as-call
  return v1;
}

// Function: towupper @ 0x4ee0
unsigned int towupper(unsigned int a0)
{
  unsigned int v1; // eax
  
  v1 = (*dat_32f20)(); // jump-as-call
  return v1;
}

// Function: fstatfs @ 0x4ef0
int fstatfs(int a0,statfs *a1)
{
  int v1; // eax
  
  v1 = (*dat_32f28)(); // jump-as-call
  return v1;
}

// Function: __cxa_atexit @ 0x4f00
void __cxa_atexit(void)
{
  (*dat_32f30)(); // jump-as-call
}

// Function: pcre2_match_context_create_8 @ 0x4f10
void pcre2_match_context_create_8(void)
{
  (*dat_32f38)(); // jump-as-call
}

// Function: getpagesize @ 0x4f20
int getpagesize(void)
{
  int v1; // eax
  
  v1 = (*dat_32f40)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x4f30
void exit(int a0)
{
  (*dat_32f48)(); // jump-as-call
}

// Function: pcre2_match_data_create_from_pattern_8 @ 0x4f40
void pcre2_match_data_create_from_pattern_8(void)
{
  (*dat_32f50)(); // jump-as-call
}

// Function: pcre2_config_8 @ 0x4f50
void pcre2_config_8(void)
{
  (*dat_32f58)(); // jump-as-call
}

// Function: re_compile_pattern @ 0x4f60
char * re_compile_pattern(char *a0,unsigned long a1,re_pattern_buffer *a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_32f60)(); // jump-as-call
  return v1;
}

// Function: pcre2_jit_compile_8 @ 0x4f70
void pcre2_jit_compile_8(void)
{
  (*dat_32f68)(); // jump-as-call
}

// Function: fflush_unlocked @ 0x4f80
int fflush_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_32f70)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x4f90
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_32f78)(); // jump-as-call
  return v1;
}

// Function: sigaltstack @ 0x4fa0
int sigaltstack(void *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_32f80)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x4fb0
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_32f88)(); // jump-as-call
  return v1;
}

// Function: pcre2_compile_8 @ 0x4fc0
void pcre2_compile_8(void)
{
  (*dat_32f90)(); // jump-as-call
}

// Function: fstat @ 0x4fd0
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_32f98)(); // jump-as-call
  return v1;
}

// Function: getc @ 0x4fe0
int getc(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_32fa0)(); // jump-as-call
  return v1;
}

// Function: sigaddset @ 0x4ff0
int sigaddset(sigset_t *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_32fa8)(); // jump-as-call
  return v1;
}

// Function: strstr @ 0x5000
char * strstr(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_32fb0)(); // jump-as-call
  return v1;
}

// Function: fstatat @ 0x5010
int fstatat(int a0,char *a1,stat *a2,int a3)
{
  int v1; // eax
  
  v1 = (*dat_32fb8)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x5020
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_32fc0)(); // jump-as-call
  return v1;
}

// Function: sub_5030 @ 0x5030
void sub_5030(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_32fd0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_5060 @ 0x5060
void sub_5060(void)
{
  return;
}

// Function: _FINI_0 @ 0x50d0
void _FINI_0(void)
{
  if (!dat_33168) {
    if (dat_32ff8)
      __cxa_finalize(dat_33008);
    sub_5060();
    dat_33168 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x5110
void _INIT_0(void)
{
  return;
}

// Function: sub_5119 @ 0x5119
void sub_5119(void)
{
  return;
}

// Function: sub_512c @ 0x512c
void sub_512c(void)
{
  return;
}

// Function: sub_513f @ 0x513f
void sub_513f(char *a0)
{
  error(2,0,"%s",a0);
}

// Function: sub_5174 @ 0x5174
void sub_5174(char *a0)
{
  error(0,0,gettext("warning: %s"),a0);
}

// Function: sub_51b4 @ 0x51b4
void sub_51b4(struct_26 *a0) // return-dupe
{
  char v1;
  char *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  unsigned char *v5; // rax
  
  v2 = (char *)sub_1ae2f(a0->field_0x8);
  if (!v2)
    return;
  a0->field_0x0 = (void *)sub_f7c8(0);
  if (*v2) { // branch-flip
    a0->field_0x38 = a0->field_0x38 + 1;
    v3 = strlen(&v2[3]);
    v4 = (unsigned long)(unsigned char)v2[2] + (unsigned char)v2[1] + v3;
    v5 = (unsigned char *)sub_26b10(v4);
    *v5 = dat_33183;
    v1 = v2[1];
    a0->field_0x40 = v2[1] || a0->field_0x40;
    memcpy(&v5[(unsigned char)v1],&v2[3],v3);
    if (v2[2])
      (&v5[(unsigned char)v1])[v3] = dat_33183;
    sub_cacb(a0->field_0x0,v5,v4);
    free(v5);
  }
  else {
    v3 = strlen(&v2[3]);
    sub_cacb(a0->field_0x0,&v2[3],v3);
  }
  sub_d9b5(a0->field_0x0);
  sub_1bb28(v2);
}

// Function: sub_5354 @ 0x5354
unsigned long sub_5354(void *a0,long a1,bool a2) // return-dupe, ternary
{
  int v1; // eax
  void *v2; // rax
  void *v3; // stack - 0x18
  
  v1 = (a2) ? 0x5c : 0x80; // branch-flip
  if (a1 + -1 <= -1)
    return 0;
  v2 = (void *)((long)a0 + a1 + -1);
  v3 = a0;
  while( true ) {
    v3 = memchr(v3,0x5c,(long)v2 - (long)v3);
    if (!v3)
      return 0;
    if (('1' <= *(char *)((long)v3 + 1)) && (*(char *)((long)v3 + 1) <= '9')) break;
    if ((v1 == *(char *)((long)v3 + 1)) && (v3 = (void *)((long)v3 + 1), v3 == v2))
      return 0;
    v3 = (void *)((long)v3 + 1);
  }
  return 1;
}

// Function: sub_5428 @ 0x5428
unsigned long sub_5428(long a0,char *a1,unsigned long a2,long a3,long a4,unsigned long a5,unsigned int a6) // return-dupe, return-dupe x2, ternary x2
{
  char v1; // al
  undefined8 v10; // stack - 0x30
  unsigned long *v2; // rax
  char v3 [40]; // stack - 0x68
  long v4 [2]; // stack - 0x90
  char *v5; // stack - 0x80
  char *v6; // stack - 0x78
  long v7; // stack - 0x70
  undefined8 v8; // stack - 0x40
  undefined8 v9; // stack - 0x38
  
  v1 = (char)a6;
  v3._0_8_ = 0;
  v3._8_8_ = 0;
  v4[1] = 0xff;
  v3._32_8_ = (dat_33180 || v1) ? NULL : (void *)sub_26b10(0x100); // branch-flip
  v8 = 0;
  if (v1) // branch-flip
    re_set_syntax(a5 | 0x2000000);
  else {
    re_set_syntax(a5);
  }
  v5 = re_compile_pattern(a1,a2,(re_pattern_buffer *)v3);
  if (v5) {
    free(v3._32_8_);
    v6 = (0 <= a4) ? (char *)sub_6a2e(a4,v4) : ""; // branch-flip
    if (!*v6) {
      error(0,0,"%s",v5);
      return 0;
    }
    v7 = v4[0];
    error(0,0,"%s:%td: %s",v6,v4[0],v5);
    return 0;
  }
  if (v1) {
    regfree((re_pattern_buffer *)v3);
    return 1;
  }
  v2 = (unsigned long *)(a3 * 0x40 + *(long *)(a0 + 0x10));
  *v2 = v3._0_8_;
  v2[1] = v3._8_8_;
  v2[2] = v3._16_8_;
  v2[3] = v3._24_8_;
  v2[4] = v3._32_8_;
  v2[5] = v8;
  v2[6] = v9;
  v2[7] = v10;
  return 1;
}

// Function: sub_5661 @ 0x5661
long sub_5661(char *a0,unsigned long a1,unsigned long a2,bool a3) // ternary x5
{
  long v1;
  long v10; // stack - 0xa8
  bool v11;
  unsigned long v12; // stack - 0xd0
  unsigned long v13; // stack - 0xc8
  char *v14; // stack - 0xc0
  char *v15; // stack - 0x98
  char *v16; // stack - 0x90
  char *v17; // stack - 0x88
  char *v18; // stack - 0x80
  unsigned long v19; // stack - 0x78
  bool v2;
  long v20; // stack - 0x70
  long v21; // stack - 0x68
  char *v22; // stack - 0x60
  void *v23; // stack - 0x58
  unsigned long v24; // stack - 0x50
  unsigned long v25; // stack - 0x48
  long v26; // stack - 0x40
  long v27; // stack - 0x38
  unsigned long v28; // stack - 0x30
  unsigned long v29; // stack - 0x28
  bool v3;
  char *v30; // stack - 0x20
  unsigned long v31; // stack - 0x18
  char v4; // al
  char v5; // al
  unsigned int v6; // eax
  unsigned long v7; // rax
  char *v8; // rax
  long v9; // stack - 0xa0
  
  v21 = sub_26fc8(1,0x48);
  v7 = sub_1bb47();
  *(unsigned long *)(v21 + 8) = v7;
  v12 = a2;
  if (dat_33180)
    v12 = a2 | 0x400000;
  v6 = (dat_33183) ? 0 : 2; // branch-flip
  sub_1bb5b(*(void **)(v21 + 8),(char *)0x331a0,v12,v6 | (unsigned int)v12 & 0x10 | 0x2c);
  v11 = dat_331a2 != '\0';
  v2 = dat_331a0 != '\x01';
  v22 = &a0[a1];
  v3 = 0;
  v16 = a0;
  v7 = sub_26aea(0x40);
  *(unsigned long *)(v21 + 0x10) = v7;
  *(long *)(v21 + 0x10) = *(long *)(v21 + 0x10) + 0x40;
  *(unsigned long *)(v21 + 0x18) = 0;
  v10 = 1;
  v18 = NULL;
  v19 = 0;
  v9 = 0;
  v20 = 0;
  v17 = a0;
  do {
    v23 = rawmemchr(v16,10);
    v24 = (long)v23 - (long)v16;
    v4 = sub_5354(v16,v24,v11 || v2);
    if ((v4) && (v17 < v16)) {
      v25 = (long)v16 - (long)v17;
      v26 = v25 + (v19 - v9);
      if (1 <= v26)
        v18 = (char *)sub_26dc6(v18,&v9,v26,-1,1);
      memcpy(&v18[v19],v17,v25);
      v19 += v25;
    }
    v27 = (*(long *)(v21 + 0x18) - v10) + 2;
    if (1 <= v27) {
      v7 = sub_26dc6((void *)(*(long *)(v21 + 0x10) + -0x40),&v10,v27,-1,0x40);
      *(unsigned long *)(v21 + 0x10) = v7;
      *(long *)(v21 + 0x10) = *(long *)(v21 + 0x10) + 0x40;
    }
    re_set_syntax(v12);
    v5 = sub_5428(v21,v16,v24,*(long *)(v21 + 0x18),v20,v12,v4 == '\0');
    if (v5 != '\x01')
      v3 = 1;
    v16 = (char *)((long)v23 + 1);
    v20 += 1;
    if (v4) {
      *(long *)(v21 + 0x18) = *(long *)(v21 + 0x18) + 1;
      v17 = v16;
    }
  } while (v16 <= v22);
  if (!v3) {
    if (v17 <= v22) {
      if (v17 <= a0) { // branch-flip
        v18 = a0;
        v19 = a1;
      }
      else {
        v28 = (long)v22 - (long)v17;
        v18 = (char *)sub_26b9f(v18,v19 + v28);
        memcpy(&v18[v19],v17,v28);
        v19 += v28;
      }
    }
    if ((dat_33181) || (dat_33182)) {
      v11 = (v12 & 0x2000) != 0;
      v29 = 0x2d;
      v30 = (char *)sub_26b10(a1 + 0x2d);
      if (dat_33182) { // branch-flip
        v8 = (v11) ? "^(" : "^\\(";
      }
      else {
        v8 = (v11) ? "(^|[^[:alnum:]_])(" : "\\(^\\|[^[:alnum:]_]\\)\\(";
      }
      strcpy(v30,v8);
      v31 = strlen(v30);
      memcpy(&v30[v31],a0,a1);
      v31 += a1;
      if (dat_33182) { // branch-flip
        v8 = (v11) ? ")$" : "\\)$";
      }
      else {
        v8 = (v11) ? ")([^[:alnum:]_]|$)" : "\\)\\([^[:alnum:]_]\\|$\\)";
      }
      strcpy(&v30[v31],v8);
      v8 = &v30[v31];
      v13 = strlen(v8) + v31;
      v15 = v30;
      v14 = v30;
      v31 = v13;
    }
    else {
      v15 = NULL;
      v13 = a1;
      v14 = a0;
    }
    sub_14b84(v14,v13,*(long *)(v21 + 8));
    sub_51b4(v21);
    sub_1a490(0,0,*(long *)(v21 + 8),1);
    if (v18) {
      if ((a3) || (v1 = *(long *)(v21 + 8), sub_19e86(v1) != '\x01')) {
        *(long *)(v21 + 0x10) = *(long *)(v21 + 0x10) + -0x40;
        *(long *)(v21 + 0x18) = *(long *)(v21 + 0x18) + 1;
        v4 = sub_5428(v21,v18,v19,0,-1,v12,0);
        if (v4 != '\x01')
          abort(); // no-return
      }
      if (v18 != v14)
        free(v18);
    }
    free(v15);
    return v21;
  }
  exit(2); // no-return
}

// Function: sub_5d23 @ 0x5d23
long sub_5d23(long *a0,char *a1,long a2,long *a3,char *a4)
{
  void *v1; // rax
  char v10; // stack - 0xbc
  char v11; // stack - 0xbb
  char v12; // stack - 0xba
  char v13; // stack - 0xb9
  int v14; // stack - 0xb8
  int v15; // stack - 0xb4
  char *v16; // stack - 0xa0
  char *v17; // stack - 0x98
  char *v18; // stack - 0x90
  char *v19; // stack - 0x88
  char *v2;
  char *v20; // stack - 0x80
  char *v21; // stack - 0x78
  char *v22; // stack - 0x70
  long v23; // stack - 0x68
  char *v24; // stack - 0x60
  long *v25; // stack - 0x58
  long v26; // stack - 0x50
  char *v27; // stack - 0x48
  long v28; // stack - 0x40
  char *v29; // stack - 0x38
  unsigned long v3; // rax
  char *v30; // stack - 0x30
  long v4;
  char *v5; // rax
  int v6; // ecx
  long v7 [3]; // stack - 0x28
  char *v8; // stack - 0xb0
  long v9; // stack - 0xa8
  
  v12 = dat_33183;
  v4 = a0[1];
  v25 = a0;
  v26 = sub_19da3(v4);
  v4 = v25[1];
  v13 = sub_19dbc(v4);
  v27 = &a1[a2];
  v8 = a1;
  v16 = a1;
  v17 = a1;
  do {
    if (v27 <= v17)
      return -1;
    v17 = v27;
    if (a4) { // branch-flip
      v18 = a4;
label_6255:
      if (0x80000001 <= (long)v17 - (long)v16)
        sub_27122(); // no-return
      v20 = v17;
      v22 = NULL;
      for (v23 = 0; v23 < v25[3]; v23 = v23 + 1) {
        v4 = v23 * 0x40 + v25[2];
        *(unsigned char *)(v4 + 0x38) = *(unsigned char *)(v4 + 0x38) & 0xbf;
        v4 = v23 * 0x40 + v25[2];
        *(unsigned char *)(v4 + 0x38) = *(unsigned char *)(v4 + 0x38) & 0x7f | (dat_33183 == '\n') << 7;
        v15 = re_search((re_pattern_buffer *)(v25[2] + v23 * 0x40),v16,((int)v17 - (int)v16) + -1,(int)v18 - (int)v16,((int)v17 - (int)v18) + -1,&v25[4]);
        if (v15 <= -2)
          sub_27122(); // no-return
        if (0 <= v15) {
          v21 = (char *)(long)(*(int *)v25[6] - v15);
          v19 = &v16[v15];
          if (v19 <= v20) {
            if ((!a4) || (dat_33181 == '\x01')) {
              if (((dat_33182 == '\x01') || (dat_33181 == '\x01')) && ((!dat_33182 || (v21 != &v17[0xffffffffffffffff - (long)v18])))) {
                if ((dat_33182 != '\x01') && (dat_33181)) {
                  while (v19 <= v20) {
                    v14 = 0;
                    v2 = &v17[-1];
                    v5 = &v19[(long)v21];
                    if ((!sub_fbe0(v5,v2)) && (v2 = &v17[-1], !sub_fc0e(v16,v19,v2))) goto label_6645;
                    if (1 <= (long)v21) {
                      v21 = &v21[-1];
                      v4 = v23 * 0x40 + v25[2];
                      *(unsigned char *)(v4 + 0x38) = *(unsigned char *)(v4 + 0x38) | 0x40;
                      v14 = re_match((re_pattern_buffer *)(v25[2] + v23 * 0x40),v16,((int)v19 + (int)v21) - (int)v18,(int)v19 - (int)v16,&v25[4]);
                      if (v14 <= -2)
                        sub_27122(); // no-return
                    }
                    if (1 <= v14) // branch-flip
                      v21 = (char *)(long)v14;
                    else {
                      if (v19 == &v17[-1]) break;
                      v19 = &v19[1];
                      v4 = v23 * 0x40 + v25[2];
                      *(unsigned char *)(v4 + 0x38) = *(unsigned char *)(v4 + 0x38) & 0xbf;
                      v15 = re_search((re_pattern_buffer *)(v25[2] + v23 * 0x40),v16,((int)v17 - (int)v16) + -1,(int)v19 - (int)v16,((int)v17 - (int)v19) + -1,&v25[4]);
                      if (v15 <= -1) {
                        if (v15 <= -2)
                          sub_27122(); // no-return
                        break;
                      }
                      v21 = (char *)(long)(*(int *)v25[6] - v15);
                      v19 = &v16[v15];
                    }
                  }
                }
                goto label_6687;
              }
              v19 = v18;
              v21 = &v17[-(long)v18];
            }
label_6645:
            if (!a4) goto label_66f6;
            if ((v19 < v20) || ((v19 == v20 && ((long)v22 < (long)v21)))) {
              v20 = v19;
              v22 = v21;
            }
          }
        }
label_6687:
      }
      if (v20 < v17) {
        v16 = v20;
        v21 = v22;
        goto label_6708;
      }
    }
    else {
      v24 = v16;
      v9 = 0;
      v11 = '\0';
      v10 = '\0';
      if (*v25) {
        v28 = sub_f585(*v25,(long)v16 - (unsigned long)*(unsigned char *)&v25[8],&v27[(unsigned long)*(unsigned char *)&v25[8] - (long)v16],v7,1);
        if (v28 <= -1)
          return v28;
        v19 = &v16[v28];
        v29 = v16;
        v1 = memrchr(a1,(int)v12,(long)v19 - (long)a1);
        if (v1) // branch-flip
          v16 = (char *)((long)v1 + 1);
        else {
          v16 = a1;
        }
        v11 = v7[0] < v25[7];
        v24 = v16;
        if (((bool)v11) || (v13 != '\x01')) {
label_5f74:
          v6 = (int)v12;
          v17 = (char *)((long)rawmemchr(v19,v6) + 1);
        }
        else {
          v4 = 0x10;
          if (0x10 <= (long)v19 - (long)v16)
            v4 = (long)v19 - (long)v16;
          if (v4 < (long)v19 - (long)v29 >> 2) goto label_5f74;
          v4 = 0x10;
          if (0x10 <= (long)v19 - (long)v16)
            v4 = (long)v19 - (long)v16;
          if ((long)v27 - (long)v29 >> 2 <= v4) // branch-flip
            v17 = v27;
          else {
            v6 = (int)v12;
            v4 = (long)v19 - (long)v16;
            if (v4 <= 0xf)
              v4 = 0x10;
            v2 = &v29[v4 * 4];
            v17 = (char *)((long)rawmemchr(v2,v6) + 1);
          }
        }
        if (!v11) goto label_6088;
        if (!dat_331a2 && dat_331a0 == '\x01') {
          if (v8 < v16)
            v8 = v16;
          if (sub_f84a(&v8,NULL,v19,v27)) {
            v24 = v8;
            goto label_6088;
          }
        }
label_66f6:
        v21 = &v17[-(long)v16];
label_6708:
        *a3 = (long)v21;
        return (long)v16 - (long)a1;
      }
label_6088:
      if ((v26) && (v11 != '\x01')) {
        v30 = (char *)sub_19d4d(v26,v24,v17,0,&v9,0);
        if ((v30) && (v30 != v17)) {
          if (v9) {
            v3 = (long)v30 - (long)a1;
            v6 = (int)v12;
            v16 = (char *)((long)memrchr(a1,v6,v3) + 1);
            v24 = v16;
          }
          v6 = (int)v12;
          v17 = (char *)((long)rawmemchr(v30,v6) + 1);
          v9 = 0;
          goto label_616c;
        }
      }
      else {
label_616c:
        v30 = (char *)sub_19d4d(v25[1],v24,v17,0,&v9,&v10);
        if ((v30) && (v30 != v17)) {
          if (v9) {
            v3 = (long)v30 - (long)a1;
            v6 = (int)v12;
            v16 = (char *)((long)memrchr(a1,v6,v3) + 1);
          }
          v6 = (int)v12;
          v17 = (char *)((long)rawmemchr(v30,v6) + 1);
          if (v10 == '\x01') {
            v18 = v16;
            goto label_6255;
          }
          goto label_66f6;
        }
      }
    }
    v16 = v17;
  } while( true );
}

// Function: sub_673a @ 0x673a
void sub_673a(void)
{
  return;
}

// Function: sub_674d @ 0x674d
void sub_674d(void)
{
  return;
}

// Function: sub_6760 @ 0x6760
unsigned long sub_6760(long a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x38
  unsigned char *v2; // stack - 0x30
  
  v1 = 0x3938a0565d85cb03;
  for (v2 = (unsigned char *)(a0 + -1 + dat_33780); *v2 != 10; v2 = &v2[1]) {
    v1 = (unsigned long)*v2 ^ v1 * 0x21;
  }
  return v1 % a1;
}

// Function: sub_67f8 @ 0x67f8
unsigned long sub_67f8(long a0,long a1)
{
  char *v1; // stack - 0x28
  char *v2; // stack - 0x20
  
  v1 = (char *)(a0 + -1 + dat_33780);
  v2 = (char *)(a1 + -1 + dat_33780);
  while( true ) {
    if (*v1 != *v2)
      return 0;
    if (*v1 == '\n') break;
    v1 = &v1[1];
    v2 = &v2[1];
  }
  return 1;
}

// Function: sub_687b @ 0x687b
long sub_687b(long a0,long a1,long a2,long a3)
{
  long v1;
  int v2; // eax
  unsigned long v3; // rax
  long *v4; // rax
  int v5; // stack - 0x68
  void *v6; // stack - 0x60
  long v7; // stack - 0x58
  void *v8; // stack - 0x50
  
  v6 = (void *)(a0 + a1);
  v7 = 1;
  v5 = 0;
  v8 = (void *)(a0 + a1);
  do {
    if ((void *)(a0 + a2) <= v8)
      return (long)v6 - a0;
    v3 = (long)rawmemchr(v8,10) + (1U - (long)v8);
    memmove(v6,v8,v3);
    v2 = sub_212a6(dat_33790,(long)v6 + (1U - a0),NULL);
    if (v2) {
      if (v2 <= -1)
        sub_27122(); // no-return
      v6 = (void *)((long)v6 + v3);
      if (!v5) {
        if (dat_33778 == dat_33770)
          dat_33768 = sub_26dc6(dat_33768,(long *)0x33770,1,-1,0x18);
        v1 = dat_33778 + 1;
        v4 = (long *)(dat_33778 * 0x18 + dat_33768);
        *v4 = dat_33788;
        v4[1] = a3;
        v4[2] = v7;
        dat_33778 = v1;
      }
      dat_33788 += 1;
    }
    v5 = v2;
    v7 += 1;
    v8 = (void *)((long)v8 + v3);
  } while( true );
}

// Function: sub_6a2e @ 0x6a2e
unsigned long sub_6a2e(long a0,long *a1)
{
  long v1; // stack - 0x10
  
  for (v1 = 1; (v1 < dat_33778 && (*(long *)(v1 * 0x18 + dat_33768) <= a0)); v1 = v1 + 1) {
  }
  *a1 = (a0 - *(long *)(v1 * 0x18 + -0x18 + dat_33768)) + *(long *)(v1 * 0x18 + dat_33768 + -8);
  return *(unsigned long *)(v1 * 0x18 + dat_33768 + -0x10);
}

// Function: sub_6b00 @ 0x6b00
void sub_6b00(void)
{
  return;
}

// Function: sub_6b0b @ 0x6b0b
void sub_6b0b(void)
{
  return;
}

// Function: sub_6b1e @ 0x6b1e
void sub_6b1e(char *a0) // return-dupe
{
  if (!*a0)
    return;
  sub_276e4(dat_33068,a0);
}

// Function: sub_6b52 @ 0x6b52
void sub_6b52(char *a0) // return-dupe
{
  if (!*a0)
    return;
  sub_27713(dat_33070);
}

// Function: sub_6b7f @ 0x6b7f
void sub_6b7f(char *a0) // return-dupe
{
  if (!dat_33758)
    return;
  sub_6b1e(a0);
}

// Function: sub_6ba8 @ 0x6ba8
void sub_6ba8(char *a0) // return-dupe
{
  if (!dat_33758)
    return;
  sub_6b52(a0);
}

// Function: sub_6bd1 @ 0x6bd1
void sub_6bd1(void)
{
  dat_33030 = dat_33028;
}

// Function: sub_6bea @ 0x6bea
void sub_6bea(void)
{
  dat_33758 = 0xffffffff;
}

// Function: sub_6bff @ 0x6bff
void sub_6bff(void)
{
  dat_33068 = 0x290e3;
  dat_33070 = 0x290e9;
}

// Function: sub_6c26 @ 0x6c26
void sub_6c26(int a0) // return-dupe
{
  if (0 <= putchar_unlocked(a0))
    return;
  dat_33798 = *__errno_location();
}

// Function: sub_6c53 @ 0x6c53
void sub_6c53(char *a0) // return-dupe
{
  if (0 <= fputs_unlocked(a0,stdout))
    return;
  dat_33798 = *__errno_location();
}

// Function: sub_6c8d @ 0x6c8d
void sub_6c8d(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,char *a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13) // return-dupe
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
  if (0 <= vfprintf(stdout,a8,&v3))
    return;
  dat_33798 = *__errno_location();
}

// Function: sub_6d74 @ 0x6d74
void sub_6d74(void *a0,unsigned long a1,unsigned long a2) // return-dupe
{
  if (fwrite_unlocked(a0,a1,a2,stdout) == a2)
    return;
  dat_33798 = *__errno_location();
}

// Function: sub_6dc0 @ 0x6dc0
void sub_6dc0(void) // return-dupe
{
  if (!fflush_unlocked(stdout))
    return;
  dat_33798 = *__errno_location();
}

// Function: sub_6deb @ 0x6deb
unsigned int sub_6deb(unsigned int a0) // return-dupe
{
  if (((((a0 & 0xf000) != 0x2000) && ((a0 & 0xf000) != 0x6000)) && ((a0 & 0xf000) != 0xc000)) && ((a0 & 0xf000) != 0x1000))
    return 0;
  return 1;
}

// Function: sub_6e43 @ 0x6e43
unsigned int sub_6e43(unsigned char a0) // return-dupe
{
  if ((dat_337cc != 2) && (!((a0 ^ 1) & dat_337cc == 0)))
    return 0;
  return 1;
}

// Function: sub_6e8a @ 0x6e8a
unsigned int sub_6e8a(long a0)
{
  unsigned int v1; // eax
  
  v1 = *(unsigned int *)(a0 + 0x18) & 0xf000;
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0x8000);
}

// Function: sub_6eac @ 0x6eac
char * sub_6eac(void)
{
  if (!dat_337c0)
    dat_337c0 = gettext("(standard input)");
  return dat_337c0;
}

// Function: sub_6edf @ 0x6edf
void sub_6edf(int a0) // return-dupe
{
  if (dat_33755 == '\x01') {
    dat_337c9 = 1;
    return;
  }
  error(0,a0,"%s",(char *)sub_6eac());
  dat_337c9 = 1;
}

// Function: sub_6f2c @ 0x6f2c
void sub_6f2c(void) // return-dupe
{
  if (dat_33798)
    return;
  sub_1143e();
}

// Function: sub_6f46 @ 0x6f46
void sub_6f46(void)
{
  unsigned char v1; // stack - 0x11
  int v2; // stack - 0x10
  unsigned int v3; // stack - 0xc
  
  v1 = 0;
  v2 = 1;
  for (v3 = 1; (int)v3 <= 0xff; v3 = v3 + 1) {
    if (!(v1 & v3) && *(char *)((long)(int)v3 + 0x331a3) != '\x01') {
      for (; v2 * 2 <= (int)v3; v2 = v2 << 1) {
      }
      v1 |= (unsigned char)v2;
    }
  }
  dat_337e8 = (unsigned long)v1 * (SUB168(ZEXT816(0xffffffffffffffff) * ZEXT816(0x8080808080808081),8) >> 7);
}

// Function: sub_6fdd @ 0x6fdd
unsigned long * sub_6fdd(unsigned long *a0)
{
  unsigned long v1;
  unsigned long *v2; // stack - 0x20
  unsigned long *v3; // stack - 0x18
  
  v2 = a0;
  while( true ) {
    if (!((unsigned long)v2 & 7)) {
      for (v3 = v2; !(dat_337e8 & *v3); v3 = &v3[1]) {
      }
      v2 = v3;
      while (v1 = *v2, !(dat_337e8 & (unsigned char)sub_f64d((int)(char)v1))) {
        v2 = (unsigned long *)((long)v2 + 1);
      }
      return v2;
    }
    v1 = *v2;
    if (dat_337e8 & (unsigned char)sub_f64d((int)(char)v1)) break;
    v2 = (unsigned long *)((long)v2 + 1);
  }
  return v2;
}

// Function: sub_7098 @ 0x7098
unsigned long sub_7098(unsigned long *a0,long a1) // return-dupe
{
  long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  unsigned long *v4; // stack - 0x28
  
  if (!dat_337e8)
    return 0;
  v3 = 0;
  *(char *)((long)a0 + a1) = 0xff;
  v4 = a0;
  while( true ) {
    v2 = sub_6fdd(v4);
    if ((unsigned long)((long)a0 + a1) <= v2)
      return 0;
    v1 = sub_f686(v2,(long)a0 + (a1 - v2),&v3);
    if (v1 <= -1) break;
    v4 = (unsigned long *)(v2 + v1);
  }
  return 1;
}

// Function: sub_716c @ 0x716c
unsigned long sub_716c(char *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  a0[a1] = '\0';
  v1 = strlen(a0);
  return CONCAT71((undefined7)(v1 >> 8),v1 != a1);
}

// Function: sub_71a6 @ 0x71a6
unsigned long sub_71a6(long a0,int a1,long a2)
{
  long v1; // rax
  long v2; // stack - 0x18
  
  if (((dat_337d0 != '\x01') && (sub_6e8a(a2))) && (a0 < *(long *)(a2 + 0x30))) {
    v2 = a0;
    if (!a1) {
      v2 = lseek(0,0,1);
      if (v2 <= -1)
        return 0;
    }
    v1 = lseek(a1,v2,4);
    if (0 <= v1) {
      if ((long)lseek(a1,v2,0) <= -1)
        sub_6edf(*__errno_location());
      if (v1 < *(long *)(a2 + 0x30))
        return 1;
    }
  }
  return 0;
}

// Function: sub_728f @ 0x728f
void sub_728f(char *a0,long *a1)
{
  if (((unsigned int)sub_27281(a0,NULL,10,a1,(char *)0x290d2) <= 1) && (0 <= *a1))
    return;
  error(2,0,"%s: %s",a0,gettext("invalid context length argument"));
}

// Function: sub_7316 @ 0x7316
unsigned long sub_7316(bool a0) // return-dupe
{
  if (a0)
    return 0x10000000;
  return 0x50000000;
}

// Function: sub_7337 @ 0x7337
unsigned int sub_7337(char *a0,unsigned char a1,char a2) // return-dupe
{
  long *v1;
  long v2; // stack - 0x10
  
  if (a2 != '\x01') // branch-flip
    v2 = 0x337a0;
  else {
    if (dat_33010 == 4)
      return 1;
    if ((a1) && (dat_337c8))
      return 0;
    v2 = 0x337b0;
  }
  if ((*(long *)(v2 + (unsigned long)a1 * 8)) && (v1 = *(long **)(v2 + (unsigned long)a1 * 8), sub_1cbb6(v1,a0)))
    return 1;
  return 0;
}

// Function: sub_73f5 @ 0x73f5
long sub_73f5(long a0,long a1)
{
  long v1; // rax
  
  if (!SCARRY8(a1,a0))
    return a1 + a0;
  error(2,0,gettext("input is too large to count"));
  return v1;
}

// Function: sub_747d @ 0x747d
unsigned long sub_747d(char *a0,long a1)
{
  char *v1; // stack - 0x10
  
  v1 = a0;
  while( true ) {
    if (&a0[a1] <= v1)
      return 1;
    if (*v1) break;
    v1 = &v1[1];
  }
  return 0;
}

// Function: sub_74c6 @ 0x74c6
unsigned long sub_74c6(int a0) // return-dupe, ternary
{
  dat_33808 = ((dat_337f0 + 1U) % dat_33818) ? (dat_33818 - (dat_337f0 + 1U) % dat_33818) + 1 + dat_337f0 : dat_337f0 + 1; // branch-flip
  *(char *)(dat_33808 + -1) = dat_33183;
  dat_33800 = a0;
  dat_33810 = dat_33808;
  if (a0) // branch-flip
    dat_33820 = 0;
  else {
    dat_33820 = lseek(0,0,1);
  }
  dat_337d0 = -(char)((long)dat_33820 >> 0x3f);
  if (!dat_337d0) {
    dat_337d1 = dat_337d0;
    return 1;
  }
  dat_337d1 = dat_337d0;
  if (*__errno_location() == 0x1d) {
    dat_33820 = 0;
    return 1;
  }
  sub_6edf(*__errno_location());
  return 0;
}

// Function: sub_7608 @ 0x7608
char sub_7608(long a0,long a1)
{
  long v1;
  long v10; // stack - 0x50
  long v11; // stack - 0x48
  long v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  long v5; // rax
  char v6; // stack - 0x71
  long v7; // stack - 0x68
  void *v8; // stack - 0x60
  long v9; // stack - 0x58
  
  v1 = dat_33818 + 8;
  if (v1 <= (long)dat_337f0 + (dat_337f8 - (long)dat_33810)) // branch-flip
    v7 = (long)dat_33810;
  else {
    v5 = v1 + ((a0 + dat_33818) - dat_337f8);
    if (1 <= v5) { // branch-flip
      v9 = -1;
      if (((sub_6e8a(a1)) && (v2 = *(long *)(a1 + 0x30) - dat_33820, 0 <= v2)) && (v3 = v2 + a0 + v1, !SCARRY8(v2,a0 + v1))) {
        v9 = v3;
        if (v3 <= dat_337f8 + v5)
          v9 = dat_337f8 + v5;
      }
      v8 = (void *)sub_26dc6(NULL,(long *)0x337f8,v5,v9,1);
    }
    else {
      v8 = dat_337f0;
    }
    v1 = a0;
    if ((unsigned long)((long)v8 + a0 + 1) % dat_33818)
      v1 = a0 + (dat_33818 - (unsigned long)((long)v8 + a0 + 1) % dat_33818);
    v7 = (long)v8 + v1 + 1;
    v4 = a0 + 1;
    memmove((void *)(v7 - v4),(void *)((long)dat_33810 - v4),v4);
    if (1 <= v5) {
      free(dat_337f0);
      dat_337f0 = v8;
    }
  }
  dat_33808 = v7 - a0;
  sub_6b00();
  v1 = (long)dat_337f0 + ((dat_337f8 + -8) - v7);
  v5 = v1 % (long)dat_33818;
  v6 = 1;
  while( true ) {
    v10 = sub_24c3e(dat_33800,v7,v1 - v5);
    if (v10 == -1) {
      v10 = 0;
      v6 = 0;
    }
    dat_33820 = v10 + dat_33820;
    if ((dat_33830 != '\x01' || !v10) || (sub_747d(v7,v10) != '\x01')) break;
    dat_33838 = sub_73f5(dat_33838,v10);
    if (dat_337d1 != '\x01') {
      v11 = lseek(dat_33800,dat_33820,3);
      if (((v11 <= -1) && (*__errno_location() == 6)) && ((sub_6e8a(a1) && (dat_33820 < *(long *)(a1 + 0x30)))))
        v11 = lseek(dat_33800,0,2);
      if (0 <= v11) { // branch-flip
        dat_33838 = sub_73f5(dat_33838,v11 - dat_33820);
        dat_33820 = v11;
      }
      else {
        dat_337d1 = '\x01';
      }
    }
  }
  dat_33810 = (void *)(v7 + v10);
  memset(dat_33810,0,8);
  sub_6b0b((long)dat_33810 + 8,(dat_337f8 - ((long)dat_33810 - (long)dat_337f0)) + -8);
  return v6;
}

// Function: sub_7a8d @ 0x7a8d
void sub_7a8d(void *a0)
{
  long v1; // stack - 0x18
  void *v2; // stack - 0x10
  
  v1 = 0;
  v2 = dat_33890;
  while ((v2 < a0 && (v2 = memchr(v2,(int)dat_33183,(long)a0 - (long)v2), v2))) {
    v1 += 1;
    v2 = (void *)((long)v2 + 1);
  }
  dat_33838 = sub_73f5(dat_33838,v1);
  dat_33890 = a0;
}

// Function: sub_7b22 @ 0x7b22
void sub_7b22(void)
{
  sub_6b7f(dat_33038);
  sub_6c53((char *)sub_6eac());
  sub_6ba8(dat_33038);
}

// Function: sub_7b58 @ 0x7b58
void sub_7b58(char a0)
{
  sub_6b7f(dat_33050);
  sub_6c26((int)a0);
  sub_6ba8(dat_33050);
}

// Function: sub_7b95 @ 0x7b95
void sub_7b95(unsigned long a0,char *a1)
{
  sub_6b7f(a1);
  sub_6c8d("%*ld",dat_33760,a0);
  sub_6ba8(a1);
}

// Function: sub_7be4 @ 0x7be4
unsigned long sub_7be4(unsigned long *a0,long a1,unsigned long a2,char a3)
{
  char v1;
  char v2; // al
  unsigned long v3; // rax
  
  if (dat_33840 != 1) {
    v1 = *(char *)((long)a0 + a1);
    v2 = sub_7098(a0,a1);
    *(char *)(a1 + (long)a0) = v1;
    if (v2) {
      dat_337ca = 1;
      return 0;
    }
  }
  if (dat_33848) {
    sub_7b22();
    if (dat_3384c) // branch-flip
      sub_7b58((int)a3);
    else {
      sub_6c26(0);
    }
  }
  if (dat_33852) {
    if (dat_33890 < a2) {
      sub_7a8d(a0);
      dat_33838 = sub_73f5(dat_33838,1);
      dat_33890 = a2;
    }
    sub_7b95(dat_33838,dat_33040);
    sub_7b58((int)a3);
  }
  if (dat_33853) {
    v3 = sub_73f5(dat_33888,(long)a0 - dat_33808);
    sub_7b95(v3,dat_33048);
    sub_7b58((int)a3);
  }
  if (((dat_3375d) && (dat_33853 || (dat_33852 || dat_33848))) && (a1))
    sub_6c26(9);
  return 1;
}

// Function: sub_7d91 @ 0x7d91
unsigned long * sub_7d91(unsigned long *a0,unsigned long *a1,char *a2,char *a3) // ternary
{
  char v1; // al
  char v2; // al
  long v3; // stack - 0x38
  unsigned long *v4; // stack - 0x30
  unsigned long *v5; // stack - 0x28
  long v6; // stack - 0x20
  unsigned long *v7; // stack - 0x18
  
  v5 = NULL;
  v4 = a0;
  do {
    if (((a1 <= v4) || (v6 = (*dat_337d8)(dat_337e0,a0,(long)a1 - (long)a0,&v3,v4), v6 <= -1)) || (v7 = (unsigned long *)((long)a0 + v6), v7 == a1)) {
      if (dat_3375c) // branch-flip
        v4 = a1;
      else if (v5)
        v4 = v5;
      return v4;
    }
    if (v3) { // branch-flip
      if (dat_3375c) { // branch-flip
        v1 = (dat_33851) ? 0x2d : 0x3a; // branch-flip
        v2 = sub_7be4(v7,v3,a1,(unsigned char)v1);
        if (v2 != '\x01')
          return NULL;
      }
      else {
        sub_6b1e(a2);
        if (v5) {
          v4 = v5;
          v5 = NULL;
        }
        sub_6d74(v4,1,(long)v7 - (long)v4);
      }
      sub_6b7f(a3);
      sub_6d74(v7,1,v3);
      sub_6ba8(a3);
      if (dat_3375c)
        sub_6c26((int)dat_33183);
    }
    else {
      v3 = 1;
      if (!v5)
        v5 = v4;
    }
    v4 = (unsigned long *)((long)v7 + v3);
  } while( true );
}

// Function: sub_7f87 @ 0x7f87
unsigned long sub_7f87(unsigned long a0,unsigned long a1,char *a2) // warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_7f87
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x20
  
  v1 = !((a1 <= a0) || (*(char *)(a1 - 1) != dat_33183));
  v2 = (unsigned long)v1;
  v1 = !((a1 - v2 <= a0) || (*(char *)(a1 + ~v2) != '\r'));
  v2 = (a1 - (v2 + (long)v1)) - a0;
  v3 = a0;
  if (1 <= (long)v2) {
    sub_6b1e(a2);
    sub_6d74(a0,1,v2);
    v3 = a0 + v2;
    sub_6b52(a2);
  }
  return v3;
}

// Function: sub_806d @ 0x806d
void sub_806d(unsigned long a0,unsigned long a1,char a2) // return-dupe x2
{
  unsigned char v1;
  bool v2; // zf
  unsigned long v3; // stack - 0x30
  char *v4; // stack - 0x18
  char *v5; // stack - 0x10
  
  if ((dat_3375c != '\x01') && (sub_7be4(a0,(a1 - a0) + -1,a1,(int)a2) != '\x01'))
    return;
  v2 = (bool)dat_33851 != (a2 == ':');
  if (dat_33758) { // branch-flip
    if ((dat_33851) && (dat_33758 <= -1))
      v1 = 1;
    else {
      v1 = 0;
    }
    if ((bool)(v1 ^ a2 == ':'))
      v4 = dat_33058;
    else {
      v4 = dat_33060;
    }
    if (a2 != ':') // branch-flip
      v5 = dat_33030;
    else {
      v5 = dat_33028;
    }
  }
  else {
    v5 = NULL;
    v4 = NULL;
  }
  v3 = a0;
  if (((dat_3375c) && (v2)) || ((dat_33758 && ((*v4 || (*v5)))))) {
    if ((v2) && ((dat_3375c || (*v5)))) {
      v3 = sub_7d91(a0,a1,v4,v5);
      if (!v3)
        return;
    }
    if ((dat_3375c != '\x01') && (*v4))
      v3 = sub_7f87(v3,a1,v4);
  }
  if ((dat_3375c != '\x01') && (v3 < a1))
    sub_6d74(v3,1,a1 - v3);
  if (dat_33878)
    sub_6dc0();
  if (dat_33798) {
    error(2,dat_33798,gettext("write error"));
    return;
  }
  dat_33898 = a1;
}

// Function: sub_828e @ 0x828e
void sub_828e(void *a0)
{
  void *v1; // rax
  
  if (!dat_33898)
    dat_33898 = dat_33808;
  while ((1 <= dat_338a8 && (dat_33898 < a0))) {
    v1 = rawmemchr(dat_33898,(int)dat_33183);
    sub_806d(dat_33898,(long)v1 + 1,0x2d);
    dat_338a8 -= 1;
  }
}

// Function: sub_8326 @ 0x8326
void sub_8326(void *a0,void *a1)
{
  void *v1;
  char v2;
  void *v3;
  char *v4; // rax
  void *v5; // stack - 0x38
  long v6; // stack - 0x30
  long v7; // stack - 0x28
  
  v2 = dat_33183;
  if ((dat_33850 != '\x01') && (1 <= dat_338a8))
    sub_828e(a0);
  v5 = a0;
  if (dat_33850 != '\x01') {
    if (dat_33898) // branch-flip
      v3 = dat_33898;
    else {
      v3 = dat_33808;
    }
    for (v6 = 0; v6 < dat_33858; v6 = v6 + 1) {
      if (v3 < v5) {
        do {
          v1 = (void *)((long)v5 - 1);
          v4 = (char *)((long)v5 - 2);
          v5 = v1;
        } while (v2 != *v4);
      }
    }
    if ((((0 <= dat_33858) || (0 <= dat_33860)) && (dat_339c0)) && ((v5 != dat_33898 && (dat_33020)))) {
      sub_6b7f(dat_33050);
      sub_6c53(dat_33020);
      sub_6ba8(dat_33050);
      sub_6c26(10);
    }
    while (v5 < a0) {
      v3 = (void *)((long)rawmemchr(v5,(int)v2) + 1);
      sub_806d(v5,v3,0x2d);
      v5 = v3;
    }
  }
  if (dat_33851) { // branch-flip
    v7 = 0;
    while ((v5 < a1 && (v7 < dat_338a0))) {
      v3 = (void *)((long)rawmemchr(v5,(int)v2) + 1);
      if (dat_33850 != '\x01')
        sub_806d(v5,v3,0x3a);
      v7 += 1;
      v5 = v3;
    }
  }
  else {
    if (dat_33850 != '\x01')
      sub_806d(a0,a1,0x3a);
    v7 = 1;
    v5 = a1;
  }
  dat_33828 = dat_33820 - (dat_33810 - (long)v5);
  if (dat_33850) // branch-flip
    dat_338a8 = 0;
  else {
    dat_338a8 = dat_33860;
    if (dat_33860 <= -1)
      dat_338a8 = 0;
  }
  dat_339c0 = 1;
  dat_338a0 -= v7;
}

// Function: sub_85c1 @ 0x85c1
void sub_85c1(char *a0,char *a1,char a2) // return-dupe
{
  char *v1;
  char *v2; // stack - 0x10
  
  v2 = a0;
  if (!a2)
    return;
  while( true ) {
    *a1 = '\0';
    v2 = &v2[strlen(v2)];
    *a1 = a2;
    if (v2 == a1) break;
    do {
      v1 = &v2[1];
      *v2 = a2;
      v2 = v1;
    } while (!*v1);
  }
}

// Function: sub_862e @ 0x862e
long sub_862e(unsigned long a0,unsigned long a1)
{
  int v1;
  long v2; // stack - 0x50
  unsigned long v3; // stack - 0x48
  long v4; // stack - 0x40
  long v5; // stack - 0x38
  unsigned long v6; // stack - 0x30
  unsigned long v7; // stack - 0x28
  unsigned long v8; // stack - 0x20
  unsigned long v9; // stack - 0x18
  
  v5 = dat_338a0;
  v3 = a0;
  do {
    if (a1 <= v3)
      return v5 - dat_338a0; // return-dupe
    v4 = (*dat_337d8)(dat_337e0,v3,a1 - v3,&v2,0);
    if (v4 <= -1) {
      if (dat_33851 != '\x01') {
        return v5 - dat_338a0;
      }
      v4 = a1 - v3;
      v2 = 0;
    }
    v6 = v3 + v4;
    v7 = v6 + v2;
    if ((dat_33851 != '\x01') && (v6 == a1)) {
      return v5 - dat_338a0;
    }
    if ((dat_33851 != '\x01') || (v3 < v6)) {
      if (dat_33851) { // branch-flip
        v9 = v6;
        v8 = v3;
      }
      else {
        v9 = v7;
        v8 = v6;
      }
      sub_8326(v8,v9);
      if ((!dat_338a0) || (dat_338b0)) {
        if (dat_338b1) {
          if (dat_337c9) // branch-flip
            v1 = dat_33080;
          else {
            v1 = 0;
          }
          exit(v1); // no-return
        }
        return v5 - dat_338a0;
      }
    }
    v3 = v7;
  } while( true );
}

// Function: sub_87e9 @ 0x87e9
long sub_87e9(unsigned int a0,long a1,char *a2) // early-return x3, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_87e9
{
  bool v1;
  char v10; // stack - 0x5e
  long v11; // stack - 0x58
  long v12; // stack - 0x50
  long v13; // stack - 0x48
  char *v14; // stack - 0x40
  char *v15; // stack - 0x38
  char *v16; // stack - 0x30
  long v17; // stack - 0x28
  long v18; // stack - 0x20
  char v2;
  char v3;
  char v4;
  char v5;
  int v6; // eax
  long v7;
  char *v8;
  char *v9; // rax
  
  v4 = dat_338b0;
  v3 = dat_33850;
  v2 = dat_33183;
  v10 = '\0';
  v17 = -1;
  if (sub_74c6(a0,a1) != '\x01')
    return 0;
  dat_33888 = 0;
  dat_33898 = NULL;
  dat_33838 = 0;
  dat_338a0 = dat_33870;
  dat_33828 = 0;
  dat_338a8 = 0;
  dat_33830 = ((dat_33831) && (!v2));
  dat_337ca = '\0';
  v11 = 0;
  v13 = 0;
  v14 = NULL;
  if (sub_7608(0,a1) != '\x01') {
    sub_6edf(*__errno_location());
    return 0;
  }
  dat_33760 = 0;
  if (dat_3375d) {
    if (sub_6e8a(a1)) // branch-flip
      v18 = *(long *)(a1 + 0x30);
    else {
      v18 = 0x7fffffffffffffff;
    }
    v6 = ((dat_33852) && (v18 != 0x7fffffffffffffff));
    v18 += v6;
    do {
      dat_33760 += 1;
      v18 /= 10;
    } while (v18);
  }
  v1 = 1;
  while( true ) {
    if ((((v17 <= -1) && (v2)) && (dat_33840 != 1)) && ((v5 = sub_716c(dat_33808,(long)dat_33810 - (long)dat_33808), v5 || ((v1 && (v7 = (long)dat_33810 - (long)dat_33808, sub_71a6(v7,a0,a1))))))) {
      if (dat_33840 == 2)
        return 0;
      if (dat_33868 != '\x01') {
        dat_33850 = '\x01';
        dat_338b0 = '\x01';
      }
      v17 = v11;
      v10 = v2;
      dat_33830 = dat_33831;
    }
    dat_33890 = dat_33808;
    if (dat_33898)
      dat_33898 = dat_33808;
    v8 = &dat_33808[(long)v14];
    if (v8 == dat_33810) {
      *a2 = 1;
      if (v13) {
        v8 = &dat_33810[1];
        *dat_33810 = v2;
        dat_33810 = v8;
        if (dat_338a0) {
          v9 = &dat_33808[(long)v14 - v13];
          v11 += sub_862e(v9,v8);
        }
        if (dat_338a8)
          sub_828e(dat_33810);
      }
      goto label_8d91;
    }
    sub_85c1(v8,dat_33810,(int)v10);
    v5 = v8[-1];
    v8[-1] = v2;
    v9 = &dat_33810[1 - (long)v8];
    v16 = (char *)((long)memrchr(&v8[-1],(int)v2,(unsigned long)v9) + 1);
    v8[-1] = v5;
    if (v16 == v8)
      v16 = &v8[-v13];
    v7 = -v13;
    v13 = (long)dat_33810 - (long)v16;
    if (&v8[v7] < v16) {
      if (dat_338a0)
        v11 += sub_862e(&v8[v7],v16);
      if (dat_338a8)
        sub_828e(v16);
      if ((!dat_338a0) && (!dat_338a8)) goto label_8d91;
      if (dat_338b0) {
        v7 = v17;
        if (v17 <= -1)
          v7 = 0;
        if (v7 < v11) goto label_8d91;
      }
    }
    v12 = 0;
    v15 = v16;
    while (((v12 < dat_33858 && (dat_33808 < v15)) && (v15 != dat_33898))) {
      v12 += 1;
      do {
        v8 = &v15[-1];
        v9 = &v15[-2];
        v15 = v8;
      } while (v2 != *v9);
    }
    if (v15 != dat_33898)
      dat_33898 = NULL;
    v14 = &v16[v13 - (long)v15];
    if (dat_33853)
      dat_33888 = sub_73f5(dat_33888,&dat_33810[-(long)v14 - (long)dat_33808]);
    if (dat_33852)
      sub_7a8d(v15);
    if (sub_7608(v14,a1) != '\x01') break;
    v1 = 0;
  }
  sub_6edf(*__errno_location());
label_8d91:
  dat_33850 = v3;
  dat_338b0 = v4;
  if (((!dat_33840) && (v3 != '\x01')) && ((dat_337ca || ((0 <= v17 && (v17 < v11)))))) {
    v8 = (char *)sub_6eac();
    error(0,0,gettext("%s: binary file matches"),v8);
  }
  return v11;
}

// Function: sub_8e14 @ 0x8e14
unsigned long sub_8e14(long a0,long a1,unsigned char a2) // return-dupe, ternary, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_8e14
{
  int v1;
  stat *v10; // stack - 0xc0
  char *v2;
  long v3;
  bool v4; // al
  unsigned int v5;
  int v6; // eax
  unsigned long v7; // rax
  stat v8;
  bool v9; // zf
  
  v9 = (*(long *)(a1 + 0x58) == 0 & a2) != 0;
  if (*(short *)(a1 + 0x68) == 6)
    return 1;
  if (!v9) {
    v5 = !(((*(short *)(a1 + 0x68) != 1) && (*(short *)(a1 + 0x68) != 2)) && (*(short *)(a1 + 0x68) != 4));
    if (sub_7337((char *)(a1 + 0x100),0,v5)) {
      sub_1e92f(a0,a1,4);
      return 1;
    }
  }
  dat_337c0 = *(long *)(a1 + 0x38);
  if ((dat_337c8) && (*(char *)(dat_337c0 + 1)))
    dat_337c0 += 2;
  v3 = dat_337c0;
  v4 = ((*(unsigned int *)(a0 + 0x48) & 2) || ((*(unsigned int *)(a0 + 0x48) & 1 && (v9))));
  switch(*(unsigned short *)(a1 + 0x68)) {
    default:
      abort(); // no-return
    case 1:
      if (dat_33010 == 3)
        return 1;
      sub_1e92f(a0,a1,4);
      break;
    case 2:
      if (dat_33755 == '\x01')
        return 1;
      error(0,0,gettext("%s: warning: recursive directory loop"),v3);
      return 1;
    case 3:
    case 0xb:
      if (sub_6e43(v9)) {
        v10 = (stat *)(a1 + 0x70);
        if (!*(int *)(a1 + 0x88)) {
          v6 = (v4) ? 0 : 0x100;
          v2 = *(char **)(a1 + 0x30);
          v1 = *(int *)(a0 + 0x2c);
          if (fstatat(v1,v2,&v8,v6)) {
            sub_6edf(*__errno_location());
            return 1;
          }
          v10 = &v8;
        }
        v5 = *(unsigned int *)&v10->field_0x18;
        if (sub_6deb(v5))
          return 1;
      }
      break;
    case 4:
    case 7:
    case 10:
      sub_6edf(*(unsigned int *)(a1 + 0x40));
      return 1;
    case 8:
    case 0xd:
      break;
    case 0xc:
    case 0xe:
      return 1;
    
  }
  v7 = sub_91bc(*(unsigned int *)(a0 + 0x2c),*(unsigned long *)(a1 + 0x30),v4,v9);
  return v7;
}

// Function: sub_9197 @ 0x9197
unsigned long sub_9197(int a0) // return-dupe
{
  if ((a0 != 0x28) && (a0 != 0x1f))
    return 0;
  return 1;
}

// Function: sub_91bc @ 0x91bc
unsigned long sub_91bc(unsigned int a0,char *a1,bool a2,char a3) // ternary x2
{
  unsigned int v1; // eax
  int v2;
  unsigned int v3; // ebx
  
  v3 = (a2) ? 0x100 : 0x20100; // branch-flip
  v1 = (sub_6e43((unsigned char)a3)) ? 0x800 : 0; // branch-flip
  v2 = sub_22af5(a0,a1,v1 | v3);
  if (v2 <= -1) {
    if (!a2) {
      v2 = *__errno_location();
      if (sub_9197(v2) == '\x01')
        return 1;
    }
    sub_6edf(*__errno_location());
    return 1;
  }
  return sub_9456(v2,(unsigned char)a3);
}

// Function: sub_9272 @ 0x9272
unsigned long sub_9272(int a0,long a1) // early-return x2
{
  unsigned int v1;
  unsigned long v2; // rax
  long v3; // rax
  unsigned long v4; // stack - 0x10
  
  if (((*(unsigned int *)(a1 + 0x18) & 0xf000) == 0x1000) && (dat_338b2)) {
    v2 = splice(a0,NULL,1,NULL,0x18000,1);
    v4 = v2;
    if ((long)v2 <= -1) {
      v1 = *__errno_location();
      v2 = (unsigned long)v1;
      if (v1 == 0x16) goto label_9331;
    }
    while (0 < (long)v4) {
      v2 = splice(a0,NULL,1,NULL,0x18000,1);
      v4 = v2;
    }
    return CONCAT71((undefined7)(v2 >> 8),v4 == 0);
  }
label_9331:
  do {
    v3 = sub_24c3e(a0,dat_337f0,dat_337f8);
    if (!v3)
      return 1;
  } while (v3 != -1);
  return 0;
}

// Function: sub_9361 @ 0x9361
void sub_9361(int a0,long a1,char a2) // return-dupe
{
  bool v1;
  
  if (a0)
    return;
  if (dat_338a0) { // branch-flip
    if ((a2 != '\x01') && (((dat_337d0 || (((long)lseek(0,0,2) <= -1 && (*__errno_location() != 0x16)))) && (sub_9272(0,a1) != '\x01'))))
      v1 = 1;
    else {
      v1 = 0;
    }
  }
  else if (((dat_33820 != dat_33828) && (dat_337d0 != '\x01')) && ((long)lseek(0,dat_33828,0) <= -1))
    v1 = 1;
  else {
    v1 = 0;
  }
  if (v1) {
    sub_6edf(*__errno_location());
    return;
  }
}

// Function: sub_9456 @ 0x9456
unsigned char sub_9456(int a0,unsigned char a1) // ternary
{
  char v1; // al
  long v10; // stack - 0xe0
  long v11; // stack - 0xd8
  long v12; // stack - 0xd0
  unsigned long v13; // stack - 0xc0
  unsigned char v2; // al
  int v3; // eax
  char *v4; // rax
  stat v5; // stack - 0xb8
  unsigned long v6; // stack - 0xc8
  char v7; // stack - 0xe6
  unsigned char v8; // stack - 0xe5
  unsigned int v9; // stack - 0xe4
  
  v8 = 1;
  v7 = 0;
  if (fstat(a0,&v5)) // branch-flip
    sub_6edf(*__errno_location());
  else if ((((!a0) || (!sub_6e43(a1))) || (v1 = sub_6deb(v5._24_4_), !v1)) && (((!a0 || (!a1)) || (v1 = sub_7337(dat_337c0,1,(v5._24_4_ & 0xf000) == 0x4000), !v1)))) {
    if ((int)dat_33848 <= -1)
      dat_33848 = (unsigned int)((v5._24_4_ & 0xf000) == 0x4000);
    if (((a0) && (dat_33010 == 3)) && ((v5._24_4_ & 0xf000) == 0x4000)) {
      v9 = dat_33014 & ~(unsigned int)(a1 ^ 1);
      if (close(a0))
        sub_6edf(*__errno_location());
      v6 = dat_337c0;
      v13 = 0;
      v10 = sub_1d975(&v6,v9,0);
      if (v10) {
        while( true ) {
          v11 = sub_1e2b9(v10);
          if (!v11) break;
          v2 = sub_8e14(v10,v11,a1);
          v8 = (v2 & v8) != 0;
        }
        if (*__errno_location())
          sub_6edf(*__errno_location());
        v3 = sub_1ddff(v10);
        if (!v3)
          return v8;
        sub_6edf(*__errno_location());
        return v8;
      }
      sub_27122(); // no-return
    }
    if ((!a0) || (((dat_33010 != 4 || ((v5._24_4_ & 0xf000) != 0x4000)) && (((dat_337cc != 2 && ((dat_337cc || (a1 == 1)))) || (v1 = sub_6deb(v5._24_4_), !v1)))))) {
      if (((((dat_33850 != '\x01') && (!dat_33844)) && (2 <= dat_33870)) && (((v5._24_4_ & 0xf000) == 0x8000 && (v5._8_8_ == dat_336c8)))) && (v5._0_8_ == dat_336c0)) {
        if (dat_33755 != '\x01') {
          v4 = (char *)sub_6eac();
          error(0,0,gettext("%s: input file is also the output"),v4);
        }
        dat_337c9 = 1;
      }
      else {
        v12 = sub_87e9(a0,&v5,&v7);
        if (dat_33868) {
          if (dat_33848) {
            sub_7b22();
            if (dat_3384c) // branch-flip
              sub_7b58(0x3a);
            else {
              sub_6c26(0);
            }
          }
          sub_6c8d("%ld\n",v12);
          if (dat_33878)
            sub_6dc0();
        }
        v8 = v12 == 0;
        if (dat_33844) { // branch-flip
          v3 = ((bool)v8) ? 2 : 1;
          if (v3 == dat_33844) {
            sub_7b22();
            sub_6c26(dat_3384c & 10);
            if (dat_33878)
              sub_6dc0();
          }
        }
        else {
          sub_9361(a0,&v5,(unsigned char)v7);
        }
      }
    }
  }
  if ((a0) && (close(a0)))
    sub_6edf(*__errno_location());
  return v8;
}

// Function: sub_992b @ 0x992b
void sub_992b(char *a0) // return-dupe
{
  if (!strcmp(a0,"-")) {
    dat_337c0 = (char *)dat_33880;
    if (dat_338b3)
      sub_2716d(0,0);
    sub_9456(0,1);
    return;
  }
  dat_337c0 = a0;
  sub_91bc(0xffffff9c,a0,1,1);
}

// Function: sub_99b6 @ 0x99b6
void sub_99b6(int a0)
{
  char *v1; // rax
  
  if (a0) { // branch-flip
    v1 = (char *)sub_1ff0c();
    fprintf(stderr,gettext("Usage: %s [OPTION]... PATTERNS [FILE]...\n"),v1);
    v1 = (char *)sub_1ff0c();
    fprintf(stderr,gettext("Try \'%s --help\' for more information.\n"),v1);
  }
  else {
    v1 = (char *)sub_1ff0c();
    printf(gettext("Usage: %s [OPTION]... PATTERNS [FILE]...\n"),v1);
    printf(gettext("Search for PATTERNS in each FILE.\n"));
    v1 = (char *)sub_1ff0c();
    printf(gettext("Example: %s -i \'hello world\' menu.h main.c\nPATTERNS can contain multiple patterns separated by newlines.\n\nPattern selection and interpretation:\n"),v1);
    printf(gettext("  -E, --extended-regexp     PATTERNS are extended regular expressions\n  -F, --fixed-strings       PATTERNS are strings\n  -G, --basic-regexp        PATTERNS are basic regular expressions\n  -P, --perl-regexp         PATTERNS are Perl regular expressions\n"));
    printf(gettext("  -e, --regexp=PATTERNS     use PATTERNS for matching\n  -f, --file=FILE           take PATTERNS from FILE\n  -i, --ignore-case         ignore case distinctions in patterns and data\n      --no-ignore-case      do not ignore case distinctions (default)\n  -w, --word-regexp         match only whole words\n  -x, --line-regexp         match only whole lines\n  -z, --null-data           a data line ends in 0 byte, not newline\n"));
    printf(gettext("\nMiscellaneous:\n  -s, --no-messages         suppress error messages\n  -v, --invert-match        select non-matching lines\n  -V, --version             display version information and exit\n      --help                display this help text and exit\n"));
    printf(gettext("\nOutput control:\n  -m, --max-count=NUM       stop after NUM selected lines\n  -b, --byte-offset         print the byte offset with output lines\n  -n, --line-number         print line number with output lines\n      --line-buffered       flush output on every line\n  -H, --with-filename       print file name with output lines\n  -h, --no-filename         suppress the file name prefix on output\n      --label=LABEL         use LABEL as the standard input file name prefix\n"));
    printf(gettext("  -o, --only-matching       show only nonempty parts of lines that match\n  -q, --quiet, --silent     suppress all normal output\n      --binary-files=TYPE   assume that binary files are TYPE;\n                            TYPE is \'binary\', \'text\', or \'without-match\'\n  -a, --text                equivalent to --binary-files=text\n"));
    printf(gettext("  -I                        equivalent to --binary-files=without-match\n  -d, --directories=ACTION  how to handle directories;\n                            ACTION is \'read\', \'recurse\', or \'skip\'\n  -D, --devices=ACTION      how to handle devices, FIFOs and sockets;\n                            ACTION is \'read\' or \'skip\'\n  -r, --recursive           like --directories=recurse\n  -R, --dereference-recursive  likewise, but follow all symlinks\n"));
    printf(gettext("      --include=GLOB        search only files that match GLOB (a file pattern)\n      --exclude=GLOB        skip files that match GLOB\n      --exclude-from=FILE   skip files that match any file pattern from FILE\n      --exclude-dir=GLOB    skip directories that match GLOB\n"));
    printf(gettext("  -L, --files-without-match  print only names of FILEs with no selected lines\n  -l, --files-with-matches  print only names of FILEs with selected lines\n  -c, --count               print only a count of selected lines per FILE\n  -T, --initial-tab         make tabs line up (if needed)\n  -Z, --null                print 0 byte after FILE name\n"));
    printf(gettext("\nContext control:\n  -B, --before-context=NUM  print NUM lines of leading context\n  -A, --after-context=NUM   print NUM lines of trailing context\n  -C, --context=NUM         print NUM lines of output context\n"));
    printf(gettext("  -NUM                      same as --context=NUM\n      --group-separator=SEP  print SEP on line between matches with context\n      --no-group-separator  do not print separator for matches with context\n      --color[=WHEN],\n      --colour[=WHEN]       use markers to highlight the matching strings;\n                            WHEN is \'always\', \'never\', or \'auto\'\n  -U, --binary              do not strip CR characters at EOL (MSDOS/Windows)\n\n"));
    printf(gettext("When FILE is \'-\', read standard input.  With no FILE, read \'.\' if\nrecursive, \'-\' otherwise.  With fewer than two FILEs, assume -h.\nExit status is 0 if any line is selected, 1 otherwise;\nif any error occurs and -q is not given, the exit status is 2.\n"));
    sub_269ee();
  }
  exit(a0); // no-return
}

// Function: sub_9be6 @ 0x9be6
unsigned long sub_9be6(char *a0,unsigned int a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned int v3; // stack - 0xc
  
  v3 = 0;
  while( true ) {
    if (7 <= v3) {
      error(2,0,gettext("invalid matcher %s"),a0);
      return v2;
    }
    if (!strcmp(a0,(char *)((long)(int)v3 * 0x20 + 0x327a0))) break;
    v3 += 1;
  }
  if ((0 <= (int)a1) && (a1 != v3)) {
    error(2,0,gettext("conflicting matchers specified"));
    return v1;
  }
  return (unsigned long)v3;
}

// Function: sub_9c9f @ 0x9c9f
int sub_9c9f(int a0,char **a1,long *a2)
{
  char *v1;
  bool v2;
  int v3;
  int v4; // eax
  char v5 [21]; // stack - 0x38
  char v6 [19];
  char *v7; // stack - 0x40
  
  v2 = 0;
  v7 = v5;
  do {
    v3 = optind;
    v4 = getopt_long(a0,a1,"0123456789A:B:C:D:EFGHIPTUVX:abcd:e:f:hiLlm:noqRrsuvwxyZz",(option *)0x320a0,NULL);
    if (sub_10eab(v4) != '\x01') {
label_9da2:
      if (v7 != v5) {
        *v7 = '\0';
        sub_728f(v5,a2);
      }
      return v4;
    }
    if ((v3 == dat_33018) && (v2))
      v7 = &v7[-(unsigned long)(v5[0] == '0')];
    else {
      v7 = v5;
    }
    if (v7 == v6) {
      strcpy(v7,"...");
      v7 = &v7[3];
      goto label_9da2;
    }
    v1 = &v7[1];
    *v7 = (char)v4;
    v2 = 1;
    dat_33018 = v3;
    v7 = v1;
  } while( true );
}

// Function: sub_9ddf @ 0x9ddf
void sub_9ddf(void)
{
  char v1;
  char *v2;
  char *v3; // stack - 0x30
  char *v4; // stack - 0x28
  char *v5; // stack - 0x20
  long *v6; // stack - 0x18
  
  v2 = getenv("GREP_COLORS");
  if ((!v2) || (!*v2))
    return;
  v3 = (char *)sub_270f1(v2);
  v4 = v3;
  v5 = NULL;
label_9e38:
  do {
    for (; (*v3 == ':' || (!*v3)); v3 = &v3[1]) {
      v1 = *v3;
      *v3 = '\0';
      v6 = (long *)0x31f80;
      while ((*v6 && (v2 = (char *)*v6, strcmp(v2,v4)))) {
        v6 = &v6[3];
      }
      if ((v6[1]) && (v5))
        *(char **)v6[1] = v5;
      if (v6[2])
        (*(void *)v6[2])();
      if (!v1)
        return;
      v5 = NULL;
      v4 = &v3[1];
label_9f6a:
    }
    if (*v3 != '=') {
      if (v5) {
        if ((*v3 != ';') && (v1 = *v3, !sub_10eab((int)v1)))
          return;
        goto label_9f6a;
      }
      v3 = &v3[1];
      goto label_9e38;
    }
    if (v3 == v4)
      return;
    if (v5)
      return;
    v5 = &v3[1];
    *v3 = '\0';
    v3 = v5;
  } while( true );
}

// Function: sub_9f7b @ 0x9f7b
unsigned long sub_9f7b(char *a0,long a1)
{
  long v1; // rax
  unsigned long v2; // stack - 0x18
  long v3; // stack - 0x28
  
  v2 = 0;
  v3 = 0;
  while( true ) {
    if (a1 <= v3)
      return 0;
    v1 = sub_f6da(&a0[v3],a1 - v3,&v2);
    if (v1 <= -1) break;
    v3 += v1;
  }
  return 1;
}

// Function: sub_a011 @ 0xa011
void sub_a011(void)
{
  int v1;
  int v2 [32];
  mbstate_t v3; // stack - 0xb0
  char v4 [24];
  int v5; // stack - 0xc0
  char v6; // stack - 0xbc
  int v7; // stack - 0xb8
  
  v5 = 0;
  do {
    if (0x100 <= v5)
      return;
    v1 = *(int *)(((long)v5 + 0x40) * 4 + 0x331a4);
    if (v1 != -1) {
      v6 = 1;
      v7 = sub_1c01a(v1,v2);
      do {
        v7 -= 1;
        if (v7 <= -1) goto label_a0e9;
        v3 = 0;
        v1 = v2[v7];
      } while (wcrtomb(v4,v1,&v3) == 1);
      v6 = 0xff;
label_a0e9:
      *(char *)((long)v5 + 0x338c0) = v6;
    }
    v5 += 1;
  } while( true );
}

// Function: sub_a134 @ 0xa134
unsigned long sub_a134(unsigned char *a0,unsigned long a1,mbstate_t *a2) // early-return x3
{
  unsigned char v1;
  unsigned int v2; // stack - 0xac
  char v3 [136];
  unsigned long v4; // stack - 0xa8
  unsigned long v5; // stack - 0xa0
  
  if (*(int *)(((long)(int)(unsigned int)*a0 + 0x40) * 4 + 0x331a4) != -1)
    return (long)*(char *)((long)(int)(unsigned int)*a0 + 0x338c0);
  v5 = sub_21b80(&v2,a0,a1,a2);
  if (0x11 <= v5)
    return 0xffffffffffffffff;
  if (sub_1c01a(v2,v3))
    return 0xffffffffffffffff;
  v4 = v5;
  do {
    v4 -= 1;
    if ((long)v4 <= 0)
      return v5;
    v1 = a0[v4];
  } while (toupper((unsigned int)v1) == (unsigned int)v1);
  return 0xffffffffffffffff;
}

// Function: sub_a29a @ 0xa29a
unsigned long sub_a29a(char *a0,long a1)
{
  int v1; // eax
  unsigned long v2; // stack - 0x18
  long v3; // stack - 0x20
  
  v2 = 0;
  v3 = 0;
  while( true ) {
    if (a1 <= v3)
      return 1;
    v1 = sub_a134(&a0[v3],a1 - v3,&v2);
    if (v1 <= -1) break;
    v3 += v1;
  }
  return 0;
}

// Function: sub_a32f @ 0xa32f
void sub_a32f(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2; // rax
  unsigned long v3; // stack - 0x18
  unsigned long v4; // stack - 0x40
  char *v5; // stack - 0x38
  char *v6; // stack - 0x30
  unsigned long v7; // stack - 0x28
  
  v4 = *a1;
  v5 = (char *)*a0;
  v3 = 0;
  v2 = (char *)sub_26c63(v4 + 1,2);
  v6 = v2;
  do {
    if (!v4) {
      *v6 = '\n';
      free((void *)*a0);
      *a0 = v2;
      *a1 = (long)v6 - (long)v2;
      return;
    }
    v7 = sub_f6da(v5,v4,&v3);
    if (v7 != 1) { // branch-flip
      if ((long)v7 <= 1) {
        if (v7 != 0xfffffffffffffffe) { // branch-flip
          if (v7 == 0xffffffffffffffff) {
            memset(&v3,0,8);
            v7 = 1;
            goto label_a412;
          }
        }
        else {
          v7 = v4;
        }
      }
      v6 = mempcpy(v6,v5,v7);
    }
    else {
label_a412:
      if (((unsigned int)((int)*v5 - 0x24U) <= 0x3a) && (0x580000000000441U >> ((unsigned char)((int)*v5 - 0x24U) & 0x3f) & 1)) {
        v1 = &v6[1];
        *v6 = '\\';
        v6 = v1;
      }
      v1 = &v6[1];
      *v6 = *v5;
      v6 = v1;
    }
    v5 = &v5[v7];
    v4 -= v7;
  } while( true );
}

// Function: sub_a4d3 @ 0xa4d3
int sub_a4d3(int a0,unsigned char *a1,unsigned long *a2)
{
  unsigned char v1;
  void *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // stack - 0x18
  int v5; // stack - 0x4c
  unsigned long v6; // stack - 0x48
  void *v7; // stack - 0x40
  unsigned char *v8; // stack - 0x38
  
  v6 = *a2;
  v2 = (void *)sub_26b10(v6 + 1);
  v7 = v2;
  v4 = 0;
  v8 = a1;
  for (; v6; v6 = v6 - v3) {
    v1 = *v8;
    v5 = a0;
    if ((char)v1 <= '|') {
      if ('{' <= (char)v1) goto label_a5e5;
      if (v1 == 0x5e) {
        free(v2); // return-dupe
        return v5;
      }
      if ((char)v1 <= '^') {
        if (v1 != 0x5c) { // branch-flip
          if ((char)v1 <= '\\') {
            if (v1 == 0x5b) {
              free(v2);
              return v5;
            }
            if ((char)v1 <= '[') {
              if ('/' <= (char)v1) { // branch-flip
                if (v1 == 0x3f) {
label_a5e5:
                  if (a0) {
                    free(v2);
                    return v5;
                  }
                }
              }
              else if ('$' <= (char)v1) {
                v3 = 1L << (v1 & 0x3f);
                if (v3 & 0x441000000000) {
                  free(v2);
                  return v5;
                }
                if (v3 & 0x90000000000) goto label_a5e5;
              }
            }
          }
        }
        else if (2 <= (long)v6) {
          v1 = v8[1];
          if ('=' <= (char)v1) { // branch-flip
            if (((char)v1 <= '|') && ('>' <= (char)v1)) {
              v3 = 1L << (v1 - 0x3e & 0x3f);
              if (!(v3 & 0x220001402200011)) {
                v3 &= 0x6000000000000002;
                goto label_a65d;
              }
              free(v2);
              return v5;
            }
          }
          else if ('\n' <= (char)v1) {
            v3 = 1L << (v1 & 0x3f);
            if (v3 & 0x13fe008000000400) {
              free(v2);
              return v5;
            }
            v3 &= 0xb0000000000;
label_a65d:
            if ((v3) && (!a0)) {
              free(v2);
              return v5;
            }
          }
          v8 = &v8[1];
          v6 -= 1;
        }
      }
    }
    if (dat_33180) // branch-flip
      v3 = sub_a134(v8,v6,&v4);
    else {
      v3 = sub_f6da(v8,v6,&v4);
    }
    if ((long)v3 < 0) {
      free(v2);
      return v5;
    }
    v7 = mempcpy(v7,v8,v3);
    v8 = &v8[v3];
  }
  if (*a2 != (long)v7 - (long)v2) {
    *a2 = (long)v7 - (long)v2;
    *(char *)mempcpy(a1,v2,(long)v7 - (long)v2) = 10;
  }
  v5 = 2;
  free(v2);
  return v5;
}

// Function: main @ 0xa7d1
int main(int argc,char **argv,char **envp) // ternary x3, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in main
{
  char **v1;
  int v10; // eax
  char *v11;
  unsigned long v12; // rax
  long v13;
  long v14; // stack - 0x1c8
  long v15; // stack - 0x1d0
  stat v16; // stack - 0x148
  stat v17; // stack - 0xb8
  long v18; // stack - 0x1d8
  char v19; // stack - 0x22
  char v2 [16];
  char v20 [8];
  bool v21; // stack - 0x202
  int v22; // stack - 0x200
  int v23; // stack - 0x1fc
  unsigned int v24; // stack - 0x1f8
  int v25; // stack - 0x1f4
  int v26; // stack - 0x1f0
  int v27; // stack - 0x1ec
  int v28; // stack - 0x1e8
  char *v29; // stack - 0x1b8
  char v3 [16];
  FILE *v30; // stack - 0x1b0
  long v31; // stack - 0x1a8
  char *v32; // stack - 0x1a0
  char **v33; // stack - 0x198
  char *v34; // stack - 0x190
  unsigned long v35; // stack - 0x188
  char *v36; // stack - 0x180
  unsigned long v37; // stack - 0x178
  long v38; // stack - 0x170
  unsigned long v39; // stack - 0x168
  bool v4;
  unsigned long v40; // stack - 0x160
  long v41; // stack - 0x158
  char *v42; // stack - 0x150
  char v43; // stack - 0x23
  char v44; // stack - 0x21
  unsigned char v5; // al
  char v6;
  unsigned int v7; // eax
  unsigned int v8; // eax
  int v9;
  
  v29 = NULL;
  v18 = 0;
  v15 = 0;
  v22 = -1;
  dat_33080 = 2;
  v24 = 0;
  dat_33183 = 10;
  dat_3384c = 0xffffffff;
  dat_33870 = 0x7fffffffffffffff;
  dat_33858 = -1;
  dat_33860 = -1;
  v14 = -1;
  dat_3375c = '\0';
  setlocale(6,(char *)0x290d2);
  bindtextdomain("grep","/usr/local/share/locale");
  textdomain("grep");
  sub_1bf18((char *)0x331a0);
  sub_285f0(sub_6f2c);
  sub_112dd(NULL);
  v23 = 0;
  dat_33790 = sub_208d6(0,0,sub_6760,sub_67f8,0);
  if (!dat_33790)
    sub_27122(); // no-return
label_b57f:
  do {
    v9 = optind;
    v10 = sub_9c9f(argc,argv,&v14);
    if (v10 == -1) {
      if (dat_33754) {
        sub_2690a(stdout,sub_1ff0c(),"GNU grep","3.8",0);
        puts(gettext("Written by Mike Haertel and others; see\n<https://git.sv.gnu.org/cgit/grep.git/tree/AUTHORS>."));
        return 0;
      }
      if (dat_33750)
        sub_99b6(0); // no-return
      if (v29) { // branch-flip
        if (!v18) {
          dat_33851 = dat_33851 != '\x01';
          dat_33181 = '\0';
          dat_33182 = '\0';
          v18 = 1;
          *v29 = '\n';
        }
      }
      else {
        if (argc <= optind)
          sub_99b6(2); // no-return
        v9 = optind + 1;
        v34 = argv[optind];
        v5 = (((v22 != 2) && (*v34 == '\\')) && (v34[1] == '-'));
        optind = v9;
        dat_33780 = (char *)sub_270f1(&v34[v5]);
        v29 = dat_33780;
        v12 = strlen(dat_33780);
        v35 = v12;
        v29[v12] = '\n';
        v18 = sub_687b(v29,0,v12 + 1,0x290d2);
      }
      v18 -= 1;
      sub_20b56(dat_33790);
      v4 = 0;
      if ((dat_338b1 != '\x01') && (!fstat(1,&v16))) {
        if ((v16._24_4_ & 0xf000) != 0x8000) { // branch-flip
          if ((v16._24_4_ & 0xf000) == 0x2000) {
            if (((stat("/dev/null",&v17)) || (v16._8_8_ != v17._8_8_)) || (v16._0_8_ != v17._0_8_))
              v4 = 1;
            else {
              dat_338b2 = '\x01';
            }
          }
        }
        else {
          dat_336c0 = v16._0_8_;
          dat_336c8 = v16._8_8_;
          dat_336d8 = CONCAT44(v16._28_4_,v16._24_4_);
          dat_336d0 = v16._16_8_;
          dat_336e0 = v16._32_8_;
          dat_336e8 = v16._40_8_;
          dat_336f0 = v16._48_8_;
          dat_336f8 = v16._56_8_;
          dat_33700 = v16._64_8_;
          dat_33708 = v16._72_8_;
          dat_33710 = v16._80_8_;
          dat_33718 = v16._88_8_;
          dat_33720 = v16._96_8_;
          dat_33728 = v16._104_8_;
          dat_33730 = v16._112_8_;
          dat_33738 = v16._120_8_;
          dat_33740 = v16._128_8_;
          dat_33748 = v16._136_8_;
        }
      }
      if (dat_338b2 || dat_338b1) { // branch-flip
        dat_33844 = 0;
label_b9b9:
        dat_33868 = '\0';
        dat_338b0 = '\x01';
      }
      else if (dat_33844) goto label_b9b9;
      dat_33850 = dat_338b0 || dat_33868;
      if (dat_33860 <= -1)
        dat_33860 = v14;
      if (dat_33858 <= -1)
        dat_33858 = v14;
      if (((!dat_33870) || ((((!v18 && (dat_33851)) && (dat_33182 != '\x01')) && (dat_33181 != '\x01')))) && (dat_33844 != 2))
        return 1;
      if (dat_33758 == 2) {
        dat_33758 = !(((!v4) || (!sub_2768b())) || (!isatty(1)));
      }
      sub_276d9();
      if (!dat_33758) goto label_bbb8;
      v36 = getenv("GREP_COLOR");
      if ((!v36) || (!*v36)) goto label_bb59;
      v32 = v36;
      goto label_bb33;
    }
    switch(v10) {
      case 0:
      case 0x55:
        goto label_b57f;
      default:
        sub_99b6(2); // no-return
      case 0x41:
        sub_728f(optarg,(long *)0x33860);
        goto label_b57f;
      case 0x42:
        sub_728f(optarg,(long *)0x33858);
        goto label_b57f;
      case 0x43:
        sub_728f(optarg,&v14);
        goto label_b57f;
      case 0x44:
        if (!strcmp(optarg,"read")) {
          dat_337cc = 1;
          goto label_b57f;
        }
        if (!strcmp(optarg,"skip")) {
          dat_337cc = 2;
          goto label_b57f;
        }
        error(2,0,gettext("unknown devices method"));
        goto label_aa4b;
      case 0x45:
label_aa4b:
        v22 = sub_9be6("egrep",v22);
        goto label_b57f;
      case 0x46:
        v22 = sub_9be6("fgrep",v22);
        break;
      case 0x47:
        v22 = sub_9be6("grep",v22);
        break;
      case 0x48:
        v24 = 1;
        break;
      case 0x49:
        dat_33840 = 2;
        break;
      case 0x4c:
        dat_33844 = 2;
        break;
      case 0x50:
        v22 = sub_9be6("perl",v22);
        break;
      case 0x52:
        dat_33014 = 0x30a;
        goto label_b073;
      case 0x71:
        dat_338b1 = '\x01';
        dat_33080 = 0;
        break;
      case 0x54:
        dat_3375d = 1;
        break;
      case 0x56:
        dat_33754 = '\x01';
        break;
      case 0x58:
        v22 = sub_9be6(optarg,v22);
        break;
      case 0x5a:
        dat_3384c = 0;
        break;
      case 0x61:
        dat_33840 = 1;
        break;
      case 0x62:
        dat_33853 = 1;
        break;
      case 99:
        dat_33868 = '\x01';
        break;
      case 100:
        dat_33010 = *(int *)(sub_10c86("--directories",optarg,0x32700,0x293a8,4,dat_33078,1) * 4 + 0x293a8);
        if (dat_33010 == 3)
          v23 = v9;
        break;
      case 0x65:
        v40 = strlen(optarg);
        v41 = v40 + (v18 - v15) + 1;
        if (1 <= v41) {
          dat_33780 = (char *)sub_26dc6(v29,&v15,v41,-1,1);
          v29 = dat_33780;
        }
        v42 = mempcpy(&v29[v18],optarg,v40);
        *v42 = 10;
        v13 = v40 + v18;
        v18 = sub_687b(v29,v18,v13 + 1,0x290d2);
        break;
      case 0x66:
        if (strcmp(optarg,"-")) { // branch-flip
          v11 = (dat_338b3) ? "rb" : "r"; // branch-flip
          v30 = fopen(optarg,v11);
          v11 = optarg;
          if (!v30)
            error(2,*__errno_location(),"%s",v11);
        }
        else {
          if (dat_338b3)
            sub_2716d(0,0);
          v30 = stdin;
        }
        v31 = v18;
        while( true ) {
          v38 = (v31 - v15) + 2;
          if (1 <= v38) {
            dat_33780 = (char *)sub_26dc6(v29,&v15,v38,-1,1);
            v29 = dat_33780;
          }
          v39 = fread_unlocked(&v29[v31],1,v15 - (v31 + 1),v30);
          if (!v39) break;
          v31 += v39;
        }
        v25 = *__errno_location();
        if (!ferror_unlocked(v30)) {
          v25 = 0;
          if (v30 != stdin) { // branch-flip
            if (fclose(v30))
              v25 = *__errno_location();
          }
          else {
            clearerr_unlocked(v30);
          }
        }
        if (v25)
          error(2,v25,"%s",optarg);
        if ((v31 != v18) && (v29[v31 + -1] != '\n')) {
          v13 = v31 + 1;
          v29[v31] = '\n';
          v31 = v13;
        }
        v18 = sub_687b(v29,v18,v31,optarg);
        break;
      case 0x68:
        v24 = 0xffffffff;
        break;
      case 0x69:
      case 0x79:
        dat_33180 = '\x01';
        break;
      case 0x6c:
        dat_33844 = 1;
        break;
      case 0x6d:
        v7 = sub_27281(optarg,NULL,10,(unsigned long *)0x33870,(char *)0x290d2);
        if (2 <= v7)
          error(2,0,gettext("invalid max count"));
        break;
      case 0x6e:
        dat_33852 = 1;
        break;
      case 0x6f:
        dat_3375c = '\x01';
        break;
      case 0x72:
label_b073:
        dat_33010 = 3;
        v23 = v9;
        break;
      case 0x73:
        dat_33755 = 1;
        break;
      case 0x75:
        error(0,0,gettext("warning: --unix-byte-offsets (-u) is obsolete"));
        break;
      case 0x76:
        dat_33851 = '\x01';
        break;
      case 0x77:
        sub_f770();
        dat_33181 = '\x01';
        break;
      case 0x78:
        dat_33182 = '\x01';
        break;
      case 0x7a:
        dat_33183 = 0;
        break;
      case 0x80:
        if (strcmp(optarg,"binary")) { // branch-flip
          if (strcmp(optarg,"text")) { // branch-flip
            if (strcmp(optarg,"without-match")) {
              error(2,0,gettext("unknown binary-files type"));
              goto label_b188;
            }
            dat_33840 = 2;
          }
          else {
            dat_33840 = 1;
          }
        }
        else {
          dat_33840 = 0;
        }
        break;
      case 0x81:
label_b188:
        if (optarg) { // branch-flip
          v9 = sub_11390(optarg,"always");
          if (((v9) && (v9 = sub_11390(optarg,"yes"), v9)) && (v9 = sub_11390(optarg,"force"), v9)) {
            v9 = sub_11390(optarg,"never");
            if (((v9) && (v9 = sub_11390(optarg,"no"), v9)) && (v9 = sub_11390(optarg,"none"), v9)) {
              v9 = sub_11390(optarg,"auto");
              if (((v9) && (v9 = sub_11390(optarg,"tty"), v9)) && (v9 = sub_11390(optarg,"if-tty"), v9))
                dat_33750 = 1;
              else {
                dat_33758 = 2;
              }
            }
            else {
              dat_33758 = 0;
            }
          }
          else {
            dat_33758 = 1;
          }
        }
        else {
          dat_33758 = 2;
        }
        break;
      case 0x82:
        sub_1c12b(optarg);
        for (v28 = 0; v28 <= 1; v28 = v28 + 1) {
          if (!*(long *)((long)v28 * 8 + 0x337b0))
            *(unsigned long *)((long)v28 * 8 + 0x337b0) = sub_1c384();
          v8 = sub_7316(v28 != 0);
          sub_1cca9(*(unsigned long *)((long)v28 * 8 + 0x337b0),optarg,v8);
        }
        break;
      case 0x83:
      case 0x86:
        for (v26 = 0; v26 <= 1; v26 = v26 + 1) {
          if (!*(long *)((long)v26 * 8 + 0x337a0))
            *(unsigned long *)((long)v26 * 8 + 0x337a0) = sub_1c384();
          v7 = (v10 != 0x86) ? 0 : 0x20000000; // branch-flip
          sub_1cca9(*(unsigned long *)((long)v26 * 8 + 0x337a0),optarg,v7 | sub_7316(v26 != 0));
        }
        break;
      case 0x84:
        for (v27 = 0; v27 <= 1; v27 = v27 + 1) {
          if (!*(long *)((long)v27 * 8 + 0x337a0))
            *(unsigned long *)((long)v27 * 8 + 0x337a0) = sub_1c384();
          v8 = sub_7316(v27 != 0);
          v9 = sub_1d272(sub_1cca9,*(unsigned long *)((long)v27 * 8 + 0x337a0),optarg,v8,10);
          v11 = optarg;
          if (v9)
            error(2,*__errno_location(),"%s",v11);
        }
        break;
      case 0x85:
        dat_33020 = optarg;
        break;
      case 0x87:
        dat_33878 = 1;
        break;
      case 0x88:
        dat_33880 = optarg;
        break;
      case 0x89:
        dat_33180 = '\0';
      
    }
  } while( true );
label_bb33:
  if ((*v32 != ';') && (v6 = *v32, !sub_10eab((int)v6))) goto label_bb59;
  if (!v32[1]) {
    dat_33030 = v36;
    dat_33028 = v36;
label_bb59:
    sub_9ddf();
    if ((v36 == dat_33028) || (v36 == dat_33030))
      error(0,0,gettext("warning: GREP_COLOR=\'%s\' is deprecated; use GREP_COLORS=\'mt=%s\'"),v36,v36);
label_bbb8:
    sub_6f46();
    if (v22 <= -1)
      v22 = 0;
    if (((v22 == 2) || (v22 == 1)) || (!v22)) {
      if (dat_33180)
        sub_a011();
      if (v22 != 2) { // branch-flip
        if (2 <= dat_33788)
          v22 = sub_a4d3(v22,v29,&v18);
      }
      else {
        if (dat_331a0 != '\x01') { // branch-flip
          if ((dat_33788 != 1) || (!dat_33181))
            v4 = 0;
          else {
            v4 = 1;
          }
        }
        else {
          v6 = sub_9f7b(v29,v18);
          if ((v6) || ((dat_33180 && (v6 = sub_a29a(v29,v18), v6 != '\x01'))))
            v4 = 1;
          else {
            v4 = 0;
          }
        }
        if (v4) {
          sub_a32f((unsigned long *)0x33780,&v18);
          v29 = dat_33780;
          v22 = 0;
        }
      }
    }
    dat_337d8 = *(void **)((long)v22 * 0x20 + 0x327b8);
    dat_337e0 = (**(void **)((long)v22 * 0x20 + 0x327b0))(v29,v18,(long)*(int *)((long)v22 * 0x20 + 0x327ac),dat_33758 || dat_3375c);
    v43 = 0;
    v19 = dat_33183;
    v44 = 0;
    v13 = (*dat_337d8)(dat_337e0,&v19,1,v20,0);
    dat_33831 = (v13 == 0) == (bool)dat_33851;
    argc -= optind;
    if ((v24) || (2 <= argc))
      dat_33848 = ~v24 >> 0x1f;
    else {
      dat_33848 = -(unsigned int)(dat_33010 == 3);
    }
    if (dat_338b3)
      sub_2716d(1,0);
    v37 = sysconf(0x1e);
    if ((1 <= (long)v37) && ((long)v37 <= 0x3ffffffffffffffb)) {
      v2._8_8_ = 0;
      v2._0_8_ = v37;
      if (SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x18000)) % v2._0_16_,0)) { // branch-flip
        v3._8_8_ = 0;
        v3._0_8_ = v37;
        v13 = (v37 - SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x18000)) % v3._0_16_,0)) + 0x18000;
      }
      else {
        v13 = 0x18000;
      }
      dat_337f8 = v37 + v13 + 8;
      dat_33818 = v37;
      dat_337f0 = sub_26b10(dat_337f8);
      if ((dat_33014 & 2) && (!dat_337cc))
        dat_337cc = 1;
      if (1 <= argc) // branch-flip
        v33 = &argv[optind];
      else if ((dat_33010 != 3) || (v23 <= 0))
        v33 = (char **)0x32730;
      else {
        v33 = (char **)0x32720;
        dat_337c8 = 1;
      }
      v21 = 1;
      do {
        v1 = &v33[1];
        v11 = *v33;
        v33 = v1;
        v21 = (sub_992b(v11) & v21) != 0;
      } while (*v33);
      v7 = (dat_337c9) ? 2 : (unsigned int)v21; // branch-flip
      return v7;
    }
    abort(); // no-return
  }
  v32 = &v32[1];
  goto label_bb33;
}

// Function: sub_c019 @ 0xc019
void sub_c019(void)
{
  return;
}

// Function: sub_c02c @ 0xc02c
void sub_c02c(void)
{
  return;
}

// Function: sub_c03f @ 0xc03f
unsigned long * sub_c03f(char *a0,long a1)
{
  unsigned long *v1; // rax
  unsigned long v2; // stack - 0x50
  char *v3; // stack - 0x48
  char *v4; // stack - 0x40
  unsigned long v5; // stack - 0x38
  unsigned long v6; // stack - 0x30
  void *v7; // stack - 0x28
  unsigned long v8; // stack - 0x20
  
  v3 = NULL;
  v2 = 0;
  v6 = sub_f7c8(1);
  v4 = a0;
  do {
    v7 = rawmemchr(v4,10);
    v5 = (long)v7 - (long)v4;
    if (dat_33182) {
      if ((dat_33183 != '\n') || (v4 <= a0)) {
        if ((long)v2 <= (long)(v5 + 1)) {
          free(v3);
          v2 = v5;
          v3 = (char *)sub_26dc6(NULL,&v2,2,-1,1);
          *v3 = dat_33183;
        }
        memcpy(&v3[1],v4,v5);
        v3[v5 + 1] = dat_33183;
        v4 = v3;
      }
      else {
        v4 = &v4[-1];
      }
      v5 += 2;
    }
    sub_cacb(v6,v4,v5);
    v4 = (char *)((long)v7 + 1);
  } while (v4 <= &a0[a1]);
  free(v3);
  v8 = sub_d633(v6);
  sub_d9b5(v6);
  v1 = (unsigned long *)sub_26aea(0x28);
  *v1 = v6;
  v1[1] = v8;
  v1[2] = a0;
  v1[3] = a1;
  v1[4] = 0;
  return v1;
}

// Function: sub_c23c @ 0xc23c
long sub_c23c(long *a0,void *a1,long a2,long *a3,void *a4)
{
  long *v1;
  void *v10; // stack - 0x90
  char v11 [16];
  bool v12; // zf
  void *v13; // stack - 0x80
  long v14; // stack - 0x78
  long v15; // stack - 0x70
  long *v16; // stack - 0x68
  long v17; // stack - 0x60
  long v18; // stack - 0x58
  void *v19; // stack - 0x50
  char v2;
  long v20; // stack - 0x38
  long v21; // stack - 0x18
  unsigned char v3; // al
  long v4; // rax
  unsigned char v5; // dl
  char *v6; // rdx
  void *v7; // rax
  char v8 [16];
  long v9; // stack - 0x88
  
  v2 = dat_33183;
  v17 = *a0;
  v12 = (unsigned char)((dat_33182 ^ 1) & dat_331a0 & (dat_331a2 ^ 1)) != 0;
  v5 = (v12 || a4) | dat_33181;
  v3 = dat_33182 ^ 1;
  if (a4) // branch-flip
    v10 = a4;
  else {
    v10 = a1;
  }
  v13 = v10;
  v16 = a0;
  do {
    if (((void *)((long)a1 + a2) < v13) || (v18 = sub_f585(v17,(long)v13 - (unsigned long)dat_33182,(long)a1 + (unsigned long)dat_33182 + (a2 - (long)v13),v8,(v3 & v5) != 0), v18 < 0))
      return -1;
    v15 = v20 - (int)((unsigned int)dat_33182 * 2);
    v9 = 0;
    if ((v12) && (v6 = (char *)(v18 + (long)v13), sub_f84a(&v10,&v9,v6,a2 + (long)a1)))
      v13 = (void *)((long)v10 + -1);
    else {
      v13 = (void *)((long)v13 + v18);
      if ((dat_33181 ^ 1) & a4 != NULL) {
label_c7f0:
        *a3 = v15;
        return (long)v13 - (long)a1;
      }
      if (dat_33182) {
        v15 += (unsigned long)(a4 == NULL);
        goto label_c7f0;
      }
      if (dat_33181 != 1) {
label_c73e:
        if ((unsigned long)((long)a1 + a2) <= (unsigned long)(v15 + (long)v13)) // branch-flip
          v14 = (long)a1 + a2;
        else {
          v7 = (void *)((long)v13 + v15);
          v14 = (long)rawmemchr(v7,(int)v2) + 1;
        }
label_c79b:
        v7 = memrchr(a1,(int)v2,(long)v13 - (long)a1);
        if (v7) // branch-flip
          v13 = (void *)((long)v7 + 1);
        else {
          v13 = a1;
        }
        v15 = v14 - (long)v13;
        goto label_c7f0;
      }
      if (!v9) {
        v19 = memrchr(v10,(int)v2,(long)v13 - (long)v10);
        if (v19)
          v10 = (void *)((long)v19 + 1);
      }
      if (1 <= v9) // branch-flip
        v4 = sub_fbe0((long)v13 - v9,a2 + (long)a1);
      else {
        v4 = sub_fc0e(v10,v13,a2 + (long)a1);
      }
      if (!v4) {
        while( true ) {
          v4 = (long)v13 + v15;
          if (!sub_fbe0(v4,a2 + (long)a1)) {
            if (!a4) goto label_c73e;
            goto label_c7f0;
          }
          if ((!a4) && (dat_331a0 != 1)) break;
          if (!v15) goto label_c6dc;
          v15 -= 1;
          if (sub_f585(v17,v13,v15,v11,1)) goto label_c6dc;
          v15 = v21;
        }
        if (!v16[4]) {
          sub_a32f(&v16[2],&v16[3]);
          v4 = sub_5661((char *)v16[2],v16[3],0x10a46,0);
          v16[4] = v4;
        }
        if ((unsigned long)((long)a1 + a2) <= (unsigned long)(v15 + (long)v13)) // branch-flip
          v14 = (long)a1 + a2;
        else {
          v7 = (void *)((long)v13 + v15);
          v14 = (long)rawmemchr(v7,(int)v2) + 1;
        }
        v4 = v14 - (long)v13;
        v1 = (long *)v16[4];
        if (0 <= (long)sub_5d23(v1,v13,v4,a3,NULL)) goto label_c79b;
        v13 = (void *)(v14 + -1);
      }
label_c6dc:
      v4 = sub_fbb2(v13,a2 + (long)a1);
      v10 = (void *)((long)v13 + v4);
      v13 = v10;
    }
    v13 = (void *)((long)v13 + 1);
  } while( true );
}

// Function: sub_c81f @ 0xc81f
void sub_c81f(void)
{
  return;
}

// Function: sub_c832 @ 0xc832
void sub_c832(void)
{
  return;
}

// Function: sub_c845 @ 0xc845
void sub_c845(char a0)
{
  sub_f64d((int)a0);
}

// Function: sub_c863 @ 0xc863
char sub_c863(long a0,char a1)
{
  if (a0)
    a1 = *(char *)(a0 + (unsigned long)(unsigned char)sub_c845((int)a1));
  return a1;
}

// Function: sub_c89b @ 0xc89b
void * sub_c89b(unsigned long a0)
{
  long v1;
  void *v2; // rax
  
  v2 = (void *)sub_26aea(0x9a0);
  _obstack_begin(v2,0,0,sub_26aea,dat_32fc8);
  *(unsigned long *)((long)v2 + 0x58) = 0;
  if ((unsigned long)(*(long *)((long)v2 + 0x20) - *(long *)((long)v2 + 0x18)) < 0x40)
    _obstack_newchunk(v2,0x40);
  *(long *)((long)v2 + 0x18) = *(long *)((long)v2 + 0x18) + 0x40;
  v1 = *(long *)((long)v2 + 0x10);
  if (v1 == *(long *)((long)v2 + 0x18))
    *(unsigned char *)((long)v2 + 0x50) = *(unsigned char *)((long)v2 + 0x50) | 2;
  *(unsigned long *)((long)v2 + 0x18) = ~*(unsigned long *)((long)v2 + 0x30) & *(long *)((long)v2 + 0x18) + *(long *)((long)v2 + 0x30);
  if ((unsigned long)(*(long *)((long)v2 + 0x20) - *(long *)((long)v2 + 8)) < (unsigned long)(*(long *)((long)v2 + 0x18) - *(long *)((long)v2 + 8)))
    *(unsigned long *)((long)v2 + 0x18) = *(unsigned long *)((long)v2 + 0x20);
  *(unsigned long *)((long)v2 + 0x10) = *(unsigned long *)((long)v2 + 0x18);
  *(long *)((long)v2 + 0x60) = v1;
  **(unsigned long **)((long)v2 + 0x60) = 0;
  *(unsigned long *)(*(long *)((long)v2 + 0x60) + 8) = 0;
  *(unsigned long *)(*(long *)((long)v2 + 0x60) + 0x10) = 0;
  *(unsigned long *)(*(long *)((long)v2 + 0x60) + 0x18) = 0;
  *(unsigned long *)(*(long *)((long)v2 + 0x60) + 0x20) = 0;
  *(unsigned long *)(*(long *)((long)v2 + 0x60) + 0x28) = 0;
  *(unsigned long *)(*(long *)((long)v2 + 0x60) + 0x30) = 0;
  *(unsigned long *)((long)v2 + 0x68) = 0x7fffffffffffffff;
  *(unsigned long *)((long)v2 + 0x970) = 0;
  *(unsigned long *)((long)v2 + 0x980) = a0;
  *(void **)((long)v2 + 0x998) = sub_f50c;
  return v2;
}

// Function: sub_cacb @ 0xcacb
void sub_cacb(void *a0,unsigned char *a1,long a2) // return-dupe, ternary x2
{
  unsigned char *v1;
  char v10; // dl
  long *v11 [13]; // stack - 0x78
  int v12 [12]; // stack - 0xa8
  bool v13; // zf
  long v14; // stack - 0x170
  unsigned char *v15; // stack - 0x168
  long *v16; // stack - 0x150
  long *v17; // stack - 0x148
  long v18; // stack - 0x140
  long *v19; // stack - 0x138
  long v2;
  long v3;
  long v4;
  long *v5;
  long v6;
  long *v7;
  long v8;
  unsigned char v9;
  
  v16 = *(long **)((long)a0 + 0x60);
  v3 = *(long *)((long)a0 + 0x980);
  v13 = *(void **)((long)a0 + 0x998) == sub_f061;
  v14 = a2;
  v15 = a1;
  if (v13) {
    v15 = &a1[a2];
    v14 = a2;
  }
  do {
    v2 = v14 + -1;
    if (!v14) {
      if (!*v16)
        *v16 = *(long *)((long)a0 + 0x58) * 2 + 1;
      *(long *)((long)a0 + 0x58) = *(long *)((long)a0 + 0x58) + 1;
      if (*(long *)((long)a0 + 0x68) > v16[5]) {
        *(long *)((long)a0 + 0x68) = v16[5];
        return;
      }
      return;
    }
    if (v13) {
      v15 = &v15[-1];
      v9 = *v15;
    }
    else {
      v1 = &v15[1];
      v9 = *v15;
      v15 = v1;
    }
    if (v3)
      v9 = *(unsigned char *)(v3 + (unsigned long)v9);
    v17 = (long *)v16[1];
    v11[0] = &v16[1];
    v12[0] = 0;
    v18 = 1;
    while ((v17 && (v9 != *(unsigned char *)&v17[3]))) {
      v11[v18] = v17;
      if (*(unsigned char *)&v17[3] <= v9) { // branch-flip
        v4 = v18 + 1;
        v12[v18] = 1;
        v17 = (long *)v17[1];
        v18 = v4;
      }
      else {
        v4 = v18 + 1;
        v12[v18] = 0;
        v17 = (long *)*v17;
        v18 = v4;
      }
    }
    if (!v17) {
      if ((unsigned long)(*(long *)((long)a0 + 0x20) - *(long *)((long)a0 + 0x18)) < 0x20)
        _obstack_newchunk(a0,0x20);
      *(long *)((long)a0 + 0x18) = *(long *)((long)a0 + 0x18) + 0x20;
      v17 = *(long **)((long)a0 + 0x10);
      if (v17 == *(long **)((long)a0 + 0x18))
        *(unsigned char *)((long)a0 + 0x50) = *(unsigned char *)((long)a0 + 0x50) | 2;
      *(unsigned long *)((long)a0 + 0x18) = ~*(unsigned long *)((long)a0 + 0x30) & *(long *)((long)a0 + 0x18) + *(long *)((long)a0 + 0x30);
      if ((unsigned long)(*(long *)((long)a0 + 0x20) - *(long *)((long)a0 + 8)) < (unsigned long)(*(long *)((long)a0 + 0x18) - *(long *)((long)a0 + 8)))
        *(unsigned long *)((long)a0 + 0x18) = *(unsigned long *)((long)a0 + 0x20);
      *(unsigned long *)((long)a0 + 0x10) = *(unsigned long *)((long)a0 + 0x18);
      *v17 = 0;
      v17[1] = 0;
      if ((unsigned long)(*(long *)((long)a0 + 0x20) - *(long *)((long)a0 + 0x18)) < 0x40)
        _obstack_newchunk(a0,0x40);
      *(long *)((long)a0 + 0x18) = *(long *)((long)a0 + 0x18) + 0x40;
      v4 = *(long *)((long)a0 + 0x10);
      if (v4 == *(long *)((long)a0 + 0x18))
        *(unsigned char *)((long)a0 + 0x50) = *(unsigned char *)((long)a0 + 0x50) | 2;
      *(unsigned long *)((long)a0 + 0x18) = ~*(unsigned long *)((long)a0 + 0x30) & *(long *)((long)a0 + 0x18) + *(long *)((long)a0 + 0x30);
      if ((unsigned long)(*(long *)((long)a0 + 0x20) - *(long *)((long)a0 + 8)) < (unsigned long)(*(long *)((long)a0 + 0x18) - *(long *)((long)a0 + 8)))
        *(unsigned long *)((long)a0 + 0x18) = *(unsigned long *)((long)a0 + 0x20);
      *(unsigned long *)((long)a0 + 0x10) = *(unsigned long *)((long)a0 + 0x18);
      v17[2] = v4;
      *(unsigned long *)v17[2] = 0;
      *(unsigned long *)(v17[2] + 8) = 0;
      *(long **)(v17[2] + 0x10) = v16;
      *(unsigned long *)(v17[2] + 0x18) = 0;
      *(unsigned long *)(v17[2] + 0x20) = 0;
      *(long *)(v17[2] + 0x28) = v16[5] + 1;
      *(unsigned long *)(v17[2] + 0x30) = 0;
      *(unsigned char *)&v17[3] = v9;
      *(char *)((long)v17 + 0x19) = 0;
      v18 -= 1;
      if (v12[v18]) // branch-flip
        v11[v18][1] = (long)v17;
      else {
        *v11[v18] = (long)v17;
      }
      while ((v18 && (!*(char *)((long)v11[v18] + 0x19)))) {
        if (v12[v18]) // branch-flip
          *(char *)((long)v11[v18] + 0x19) = *(char *)((long)v11[v18] + 0x19) + '\x01';
        else {
          *(char *)((long)v11[v18] + 0x19) = *(char *)((long)v11[v18] + 0x19) + '\xff';
        }
        v18 -= 1;
      }
      if (v18) {
        if (v12[v18]) { // branch-flip
label_d19a:
          if (v12[v18] == 1) {
            v5 = v11[v18];
            *(char *)((long)v5 + 0x19) = *(char *)((long)v5 + 0x19) + '\x01';
            if (*(char *)((long)v5 + 0x19)) goto label_d1d3;
          }
        }
        else {
          v5 = v11[v18];
          *(char *)((long)v5 + 0x19) = *(char *)((long)v5 + 0x19) + '\xff';
          if (!*(char *)((long)v5 + 0x19)) goto label_d19a;
label_d1d3:
          if (*(char *)((long)v11[v18] + 0x19) != '\xfe') { // branch-flip
            if (*(char *)((long)v11[v18] + 0x19) != '\x02')
              abort(); // no-return
            if (v12[v18 + 1]) { // branch-flip
              if (v12[v18 + 1] != 1)
                abort(); // no-return
              v5 = v11[v18];
              v19 = (long *)v5[1];
              v4 = *v19;
              *v19 = (long)v5;
              v5[1] = v4;
              *(char *)((long)v5 + 0x19) = 0;
              *(char *)((long)v19 + 0x19) = *(char *)((long)v5 + 0x19);
            }
            else {
              v5 = v11[v18];
              v7 = (long *)v5[1];
              v19 = (long *)*v7;
              v4 = *v19;
              v8 = v19[1];
              *v19 = (long)v5;
              v5[1] = v4;
              v19[1] = (long)v7;
              *v7 = v8;
              v10 = (*(char *)((long)v19 + 0x19) != '\x01') ? 0 : 0xff; // branch-flip
              *(char *)((long)v5 + 0x19) = v10;
              *(bool *)((long)v7 + 0x19) = *(char *)((long)v19 + 0x19) == '\xff';
              *(char *)((long)v19 + 0x19) = 0;
            }
          }
          else if (v12[v18 + 1]) { // branch-flip
            if (v12[v18 + 1] != 1)
              abort(); // no-return
            v5 = v11[v18];
            v4 = *v5;
            v19 = *(long **)(v4 + 8);
            v8 = v19[1];
            v6 = *v19;
            *v19 = v4;
            *(long *)(v4 + 8) = v6;
            v19[1] = (long)v5;
            *v5 = v8;
            v10 = (*(char *)((long)v19 + 0x19) != '\x01') ? 0 : 0xff; // branch-flip
            *(char *)(v4 + 0x19) = v10;
            *(bool *)((long)v5 + 0x19) = *(char *)((long)v19 + 0x19) == '\xff';
            *(char *)((long)v19 + 0x19) = 0;
          }
          else {
            v5 = v11[v18];
            v19 = (long *)*v5;
            v4 = v19[1];
            v19[1] = (long)v5;
            *v5 = v4;
            *(char *)((long)v5 + 0x19) = 0;
            *(char *)((long)v19 + 0x19) = *(char *)((long)v5 + 0x19);
          }
          if (v12[v18 + -1]) // branch-flip
            v11[v18 + -1][1] = (long)v19;
          else {
            *v11[v18 + -1] = (long)v19;
          }
        }
      }
    }
    v16 = (long *)v17[2];
    v14 = v2;
  } while( true );
}

// Function: sub_d633 @ 0xd633
unsigned long sub_d633(long a0)
{
  return *(unsigned long *)(a0 + 0x58);
}

// Function: sub_d649 @ 0xd649
void sub_d649(unsigned long *a0,long *a1) // return-dupe
{
  long v1;
  
  if (!a0)
    return;
  sub_d649(*a0,a1);
  sub_d649(a0[1],a1);
  v1 = *a1;
  *(unsigned long *)(v1 + 0x18) = a0[2];
  *a1 = *(long *)(v1 + 0x18);
}

// Function: sub_d6b4 @ 0xd6b4
void sub_d6b4(struct_21 *a0,long a1,unsigned long a2,char a3) // return-dupe, ternary
{
  long v1; // stack - 0x28
  unsigned long *v2; // stack - 0x10
  
  if (!a0)
    return;
  sub_d6b4(a0->field_0x0,a1,a2,(unsigned char)a3);
  sub_d6b4(a0->field_0x8,a1,a2,(unsigned char)a3);
  v1 = a1;
  while( true ) {
    if (!v1) {
      *(unsigned long *)(a0->field_0x10 + 0x20) = a2;
      return;
    }
    v2 = *(unsigned long **)(v1 + 8);
    while ((v2 && (a0->field_0x18 != *(char *)&v2[3]))) {
      v2 = (*(unsigned char *)&v2[3] <= (unsigned char)a0->field_0x18) ? (unsigned long *)v2[1] : (unsigned long *)*v2; // branch-flip
    }
    if (v2) break;
    v1 = *(long *)(v1 + 0x20);
  }
  *(unsigned long *)(a0->field_0x10 + 0x20) = v2[2];
  if (a3 == '\x01')
    return;
  if (!*(long *)v2[2])
    return;
  if (*(long *)a0->field_0x10)
    return;
  *(unsigned long *)a0->field_0x10 = 0xffffffffffffffff;
}

// Function: sub_d7f4 @ 0xd7f4
void sub_d7f4(struct_23 *a0,long a1,long a2) // return-dupe x2
{
  if (!a0)
    return;
  sub_d7f4(a0->field_0x0,a1,a2);
  sub_d7f4(a0->field_0x8,a1,a2);
  if ((long)(unsigned long)*(unsigned char *)(a2 + (unsigned long)a0->field_0x18) <= a1)
    return;
  *(char *)(a2 + (unsigned long)a0->field_0x18) = (char)a1;
}

// Function: sub_d883 @ 0xd883
unsigned long sub_d883(unsigned long *a0,struct_21 *a1) // early-return x3
{
  unsigned long v1;
  unsigned int v2; // eax
  unsigned long *v3;
  unsigned long *v4; // stack - 0x10
  
  if (!a1)
    return 1;
  v1 = a1->field_0x0;
  if (sub_d883(a0,v1) != '\x01')
    return 0;
  v1 = a1->field_0x8;
  v2 = sub_d883(a0,v1) ^ 1;
  v3 = (unsigned long *)(unsigned long)v2;
  v4 = a0;
  if ((char)v2)
    return 0;
  while ((v4 && (v3 = NULL, a1->field_0x18 != *(char *)&v4[3]))) {
    if (*(unsigned char *)&v4[3] <= (unsigned char)a1->field_0x18) { // branch-flip
      v3 = (unsigned long *)v4[1];
      v4 = v3;
    }
    else {
      v3 = (unsigned long *)*v4;
      v4 = v3;
    }
  }
  return CONCAT71((undefined7)((unsigned long)v3 >> 8),v4 != NULL);
}

// Function: sub_d943 @ 0xd943
void sub_d943(struct_23 *a0,long a1) // return-dupe
{
  if (!a0)
    return;
  sub_d943(a0->field_0x0,a1);
  sub_d943(a0->field_0x8,a1);
  *(unsigned long *)((unsigned long)a0->field_0x18 * 8 + a1) = a0->field_0x10;
}

// Function: sub_d9b5 @ 0xd9b5
void sub_d9b5(unsigned long *a0) // return-dupe
{
  unsigned long *v1;
  unsigned long *v10;
  unsigned long *v11;
  bool v12; // zf
  unsigned int v13; // stack - 0xa1c
  unsigned int v14; // stack - 0xa18
  unsigned int v15; // stack - 0xa14
  unsigned int v16; // stack - 0xa10
  int v17; // stack - 0xa0c
  long *v18; // stack - 0xa00
  long v19; // stack - 0x9f8
  unsigned long *v2;
  long v20; // stack - 0x9f0
  long v21; // stack - 0x9e8
  long v22; // stack - 0x9e0
  long v23; // stack - 0x9d8
  unsigned long *v24; // stack - 0x9d0
  unsigned long *v25; // stack - 0x9c8
  unsigned long *v26; // stack - 0x9c0
  unsigned long v27; // stack - 0x9b8
  unsigned long *v28; // stack - 0x9b0
  unsigned long *v29; // stack - 0x9a8
  char v3;
  long v30; // stack - 0x9a0
  unsigned long *v31; // stack - 0x998
  unsigned long *v32; // stack - 0x990
  unsigned long v33; // stack - 0x988
  unsigned long *v34; // stack - 0x980
  unsigned long *v35; // stack - 0x978
  unsigned long *v36; // stack - 0x970
  unsigned long v37; // stack - 0x968
  unsigned long *v38; // stack - 0x960
  unsigned long *v39; // stack - 0x958
  struct_21 *v4;
  long v40; // stack - 0x950
  unsigned long *v41; // stack - 0x948
  unsigned long *v42; // stack - 0x940
  unsigned long v43; // stack - 0x938
  unsigned long *v44; // stack - 0x930
  unsigned long *v45; // stack - 0x928
  long v46; // stack - 0x920
  char v5; // al
  long v6;
  unsigned long v7 [33];
  long *v8; // stack - 0xa08
  unsigned long v9 [256];
  
  v23 = a0[0x130];
  if (v23) // branch-flip
    v24 = v7;
  else {
    v24 = &a0[0xe];
  }
  v12 = a0[0xb] == 1;
  if (v12) {
    v8 = (long *)a0[0xc];
    for (v18 = v8; v18; v18 = (long *)v18[3]) {
      sub_d649((unsigned long *)v18[1],&v8);
    }
    v27 = a0[0xd];
    v25 = a0;
    v26 = a0;
    v28 = a0;
    if ((unsigned long)(a0[4] - a0[3]) < v27)
      _obstack_newchunk((obstack *)a0,v27);
    v26[3] = v26[3] + v27;
    v29 = v25;
    v30 = v25[2];
    if (v30 == v25[3])
      *(unsigned char *)&v25[10] = *(unsigned char *)&v25[10] | 2;
    v25[3] = ~v25[6] & v25[3] + v25[6];
    if ((unsigned long)(v25[4] - v25[1]) < (unsigned long)(v25[3] - v25[1]))
      v25[3] = v25[4];
    v25[2] = v25[3];
    a0[0x12e] = v30;
    v18 = (long)a0[0xc];
    for (v19 = 0; v19 < (long)a0[0xd]; v19 = v19 + 1) {
      *(char *)(v19 + a0[0x12e]) = *(char *)(*(long *)((long)v18 + 8) + 0x18);
      v18 = *(long **)((long)v18 + 0x18);
    }
    v31 = (unsigned long *)sub_c89b(a0[0x130]);
    v31[0x133] = sub_f061;
    sub_cacb(v31,(unsigned char *)a0[0x12e],a0[0xd]);
    v33 = 0;
    v32 = a0;
    _obstack_free((obstack *)a0,NULL);
    v6 = 0x134;
    v10 = v31;
    v11 = a0;
    while (v6) {
      v2 = &v11[1];
      v1 = &v10[1];
      *v11 = *v10;
      v6 -= 1;
      v10 = v1;
      v11 = v2;
    }
    free(v31);
  }
  v6 = a0[0xd];
  if (0x100 <= v6)
    v6 = 0xff;
  memset(v24,(int)v6,0x100);
  v8 = (long *)a0[0xc];
  for (v18 = v8; v18; v18 = (long *)v18[3]) {
    sub_d649((unsigned long *)v18[1],&v8);
    sub_d7f4((struct_23 *)v18[1],v18[5],v24);
    sub_d6b4((struct_21 *)v18[1],v18[4],a0[0xc],v12);
    if (v12) {
      v18[6] = a0[0xd];
      v18[7] = a0[0xd];
      for (v20 = v18[4]; v20; v20 = *(long *)(v20 + 0x20)) {
        v4 = (struct_21 *)v18[1];
        v10 = *(unsigned long **)(v20 + 8);
        if ((sub_d883(v10,v4) != '\x01') && (v18[5] - *(long *)(v20 + 0x28) < *(long *)(v20 + 0x30)))
          *(long *)(v20 + 0x30) = v18[5] - *(long *)(v20 + 0x28);
        if ((*v18) && (v18[5] - *(long *)(v20 + 0x28) < *(long *)(v20 + 0x38)))
          *(long *)(v20 + 0x38) = v18[5] - *(long *)(v20 + 0x28);
      }
    }
  }
  if (v12) {
    for (v18 = *(long **)(a0[0xc] + 0x18); v18; v18 = (long *)v18[3]) {
      if (*(long *)(v18[2] + 0x38) < v18[7])
        v18[7] = *(long *)(v18[2] + 0x38);
      if (v18[7] < v18[6])
        v18[6] = v18[7];
    }
  }
  if (v23) // branch-flip
    v34 = v9;
  else {
    v34 = &a0[0x2e];
  }
  memset(v34,0,0x800);
  sub_d943(*(struct_23 **)(a0[0xc] + 8),v34);
  v13 = 0xfffffffe;
  v14 = 0xffffffff;
  for (v15 = 0; (int)v15 <= 0xff; v15 = v15 + 1) {
    v16 = v15;
    if (v23) {
      v3 = *(char *)(v23 + (int)v15);
      v16 = (unsigned int)(unsigned char)sub_c845((int)v3);
      a0[(long)(int)v15 + 0x2e] = v34[(int)v16];
    }
    if (a0[(long)(int)v15 + 0x2e]) {
      if (-1 <= (int)v13) { // branch-flip
        if (v13 != v16) { // branch-flip
          if ((v15 != v16) || (v13 != v14))
            v13 = 0xffffffff;
          else {
            v14 = v15;
          }
        }
        else if (v14 != v16) // branch-flip
          v14 = 0xffffffff;
        else {
          v14 = v15;
        }
      }
      else {
        v13 = v16;
        v14 = v15;
      }
    }
  }
  *(unsigned int *)&a0[0x131] = v13;
  *(unsigned int *)((long)a0 + 0x98c) = v14;
  if (v12) {
    v37 = a0[0xd];
    v35 = a0;
    v36 = a0;
    v38 = a0;
    if ((unsigned long)(a0[4] - a0[3]) < v37)
      _obstack_newchunk((obstack *)a0,v37);
    v36[3] = v36[3] + v37;
    v39 = v35;
    v40 = v35[2];
    if (v40 == v35[3])
      *(unsigned char *)&v35[10] = *(unsigned char *)&v35[10] | 2;
    v35[3] = ~v35[6] & v35[3] + v35[6];
    if ((unsigned long)(v35[4] - v35[1]) < (unsigned long)(v35[3] - v35[1]))
      v35[3] = v35[4];
    v35[2] = v35[3];
    a0[0x12e] = v40;
    v18 = (long *)a0[0xc];
    for (v21 = a0[0xd]; 0 < v21; v21 = v21 + -1) {
      *(char *)(v21 + -1 + a0[0x12e]) = *(char *)(v18[1] + 0x18);
      v18 = (long *)v18[3];
    }
    if (2 <= (long)a0[0xd]) {
      v43 = (a0[0xd] + -1) * 8;
      v41 = a0;
      v42 = a0;
      v44 = a0;
      if ((unsigned long)(a0[4] - a0[3]) < v43)
        _obstack_newchunk((obstack *)a0,v43);
      v42[3] = v42[3] + v43;
      v45 = v41;
      v46 = v41[2];
      if (v46 == v41[3])
        *(unsigned char *)&v41[10] = *(unsigned char *)&v41[10] | 2;
      v41[3] = ~v41[6] & v41[3] + v41[6];
      if ((unsigned long)(v41[4] - v41[1]) < (unsigned long)(v41[3] - v41[1]))
        v41[3] = v41[4];
      v41[2] = v41[3];
      a0[0x12f] = v46;
      v18 = *(long **)(a0[0xc] + 0x18);
      for (v22 = 0; v22 < a0[0xd] + -1; v22 = v22 + 1) {
        *(long *)(a0[0x12f] + v22 * 8) = v18[6];
        v18 = (long *)v18[3];
      }
      v5 = sub_c863(v23,(int)*(char *)(a0[0xd] + -2 + a0[0x12e]));
      *(char *)&a0[0x132] = v5;
    }
  }
  if (v23) {
    for (v17 = 0; v17 <= 0xff; v17 = v17 + 1) {
      v3 = *(char *)(v23 + v17);
      *(char *)((long)a0 + (long)v17 + 0x70) = *(char *)((long)v24 + (unsigned long)(unsigned char)sub_c845((int)v3));
    }
    return;
  }
}

// Function: sub_e78b @ 0xe78b
unsigned long sub_e78b(unsigned long *a0,unsigned long a1,long a2,long a3,long a4,char a5,unsigned int a6,long a7,long a8) // return-dupe
{
  char v1;
  char v2;
  char v3; // al
  unsigned long v4; // stack - 0x38
  long v5; // stack - 0x30
  long v6; // stack - 0x28
  long v7; // stack - 0x20
  
  v2 = (char)a6;
  v4 = *a0;
  v6 = 0;
  v5 = a3;
  do {
    v7 = 2;
    v1 = *(char *)(v4 - 2);
    if (v2 == sub_c863(a4,(int)v1)) {
      do {
        v7 += 1;
        if (v5 < v7) break;
        v3 = sub_c863(a4,(int)*(char *)(v4 - v7));
        v1 = *(char *)(a2 - v7);
      } while (v3 == sub_c863(a4,(int)v1));
      if (v5 < v7) {
        v7 = v6 + v5;
        do {
          v7 += 1;
          if (a3 < v7) break;
          v3 = sub_c863(a4,(int)*(char *)(v4 - v7));
          v1 = *(char *)(a2 - v7);
        } while (v3 == sub_c863(a4,(int)v1));
        if (a3 < v7) {
          *a0 = v4 - a3;
          return 1;
        }
      }
    }
    v5 = *(long *)(v7 * 8 + -0x10 + *(long *)(a8 + 0x978));
    v4 += v5;
    if (a1 < v4) goto label_e9ac;
    v1 = *(char *)(v4 - 1);
    if (a5 != sub_c863(a4,(int)v1)) {
      if (a7) {
        v2 = *(char *)(v4 - 1);
        v4 += *(unsigned char *)(a7 + (unsigned long)(unsigned char)sub_c845((int)v2));
      }
label_e9ac:
      *a0 = v4;
      return 0;
    }
    v6 = v7 + -1;
  } while( true );
}

// Function: sub_e9c2 @ 0xe9c2
char * sub_e9c2(char *a0,long a1,long a2) // early-return
{
  char v1;
  unsigned int v2;
  unsigned int v3;
  char *v4; // rax
  char *v5; // stack - 0x30
  
  v4 = &a0[a1];
  v5 = a0;
  if (0 <= *(int *)(a2 + 0x98c)) {
    while( true ) {
      if (v4 <= v5)
        return NULL;
      v1 = *v5;
      if (*(long *)(a2 + ((long)(int)(unsigned int)(unsigned char)sub_c845((int)v1) + 0x2e) * 8)) break;
      v5 = &v5[1];
      if (!((unsigned long)v5 % 0x10)) {
        v2 = *(unsigned int *)(a2 + 0x98c);
        v3 = *(unsigned int *)(a2 + 0x988);
        return (char *)sub_222e7(v5,v3,v2,(long)v4 - (long)v5);
      }
    }
    return v5;
  }
  while( true ) {
    if (v4 <= v5)
      return NULL;
    v1 = *v5;
    if (*(long *)(a2 + ((long)(int)(unsigned int)(unsigned char)sub_c845((int)v1) + 0x2e) * 8)) break;
    v5 = &v5[1];
  }
  return v5;
}

// Function: sub_eae1 @ 0xeae1
long sub_eae1(long a0,long a1,long a2) // early-return x3
{
  char v1;
  long v10; // stack - 0x38
  long v11; // stack - 0x30
  long v12; // stack - 0x28
  unsigned long v13; // stack - 0x20
  unsigned long v14; // stack - 0x18
  char v2; // al
  char v3;
  int v4; // eax
  long v5; // rax
  unsigned long v6; // stack - 0x50
  unsigned int v7; // stack - 0x58
  long v8; // stack - 0x48
  long v9; // stack - 0x40
  
  v9 = *(long *)(a0 + 0x68);
  v10 = *(long *)(a0 + 0x980);
  if (!v9)
    return 0;
  if (a2 < v9)
    return -1;
  if (v9 != 1) { // branch-flip
    v11 = a0 + 0x70;
    v12 = v9 + *(long *)(a0 + 0x970);
    v6 = a1 + v9;
    v2 = (char)*(unsigned int *)(a0 + 0x988);
    v1 = *(char *)(a0 + 0x990);
    v8 = v9 * 0xc;
    if ((SEXT816(v8) != SEXT816(v9) * SEXT816(0xc)) || (a2 <= v8)) {
label_ef6e:
      v13 = a1 + a2;
      v3 = *(char *)(v6 - 1);
      v7 = (unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3));
      do {
        if ((long)(v13 - v6) < (long)(int)v7)
          return -1;
        v6 = (long)(int)v7 + v6;
        v3 = *(char *)(v6 - 1);
        v7 = (unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3));
      } while ((v7) || (v3 = sub_e78b(&v6,v13,v12,v9,v10,(int)v2,(int)v1,0,a0), !v3));
      v5 = v6 - a1;
    }
    else {
      v13 = a1 + a2 + v9 * -0xb;
label_ef5d:
      do {
        if (v13 < v6) goto label_ef6e;
        v14 = v6;
        v3 = *(char *)(v6 - 1);
        v6 = (long)(int)(unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3)) + v6;
        v3 = *(char *)(v6 - 1);
        v4 = (unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3));
        v6 = (long)v4 + v6;
        if (v4) {
          v3 = *(char *)(v6 - 1);
          v6 = (long)(int)(unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3)) + v6;
          v3 = *(char *)(v6 - 1);
          v6 = (long)(int)(unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3)) + v6;
          v3 = *(char *)(v6 - 1);
          v4 = (unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3));
          v6 = (long)v4 + v6;
          if (v4) {
            v3 = *(char *)(v6 - 1);
            v6 = (long)(int)(unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3)) + v6;
            v3 = *(char *)(v6 - 1);
            v6 = (long)(int)(unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3)) + v6;
            v3 = *(char *)(v6 - 1);
            v4 = (unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3));
            v6 = (long)v4 + v6;
            if (v4) {
              v3 = *(char *)(v6 - 1);
              v6 = (long)(int)(unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3)) + v6;
              v3 = *(char *)(v6 - 1);
              v6 = (long)(int)(unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3)) + v6;
              if (0x80 <= (long)(v6 - v14)) goto label_ef5d;
              v6 -= 1;
              v5 = sub_e9c2(v6,(a1 + a2) - v6,a0);
              if (!v5)
                return -1;
              v6 = v5 + 1;
              if (v13 <= v6) goto label_ef6e;
            }
          }
        }
        v3 = sub_e78b(&v6,v13,v12,v9,v10,(int)v2,(int)v1,v11,a0);
      } while (!v3);
      v5 = v6 - a1;
    }
  }
  else {
    v5 = sub_e9c2(a1,a2,a0);
    if (!v5)
      return -1;
    v5 -= a1;
  }
  return v5;
}

// Function: sub_f061 @ 0xf061
unsigned long sub_f061(long a0,long a1,long a2,unsigned long *a3)
{
  unsigned long v1; // rax
  
  if (*(long *)(a0 + 0x980)) // branch-flip
    v1 = sub_eae1(a0,a1,a2);
  else {
    v1 = sub_eae1(a0,a1,a2);
  }
  *a3 = 0;
  a3[1] = v1;
  a3[2] = *(unsigned long *)(a0 + 0x68);
  return v1;
}

// Function: sub_f0f4 @ 0xf0f4
long sub_f0f4(long a0,char *a1,long a2,long *a3,bool a4) // early-return x5, ternary x2
{
  char v1;
  unsigned long v10; // stack - 0x38
  unsigned long *v11; // stack - 0x30
  long *v12; // stack - 0x28
  int v2;
  long v3;
  unsigned char v4; // al
  char *v5; // rax
  char *v6;
  long *v7; // stack - 0x50
  long *v8; // stack - 0x48
  char *v9; // stack - 0x40
  
  if (a2 < *(long *)(a0 + 0x68))
    return -1;
  v3 = *(long *)(a0 + 0x980);
  v7 = *(long **)(a0 + 0x60);
  v5 = &a1[a2];
  v9 = a1;
  if (!*v7) {
    v2 = *(int *)(a0 + 0x988);
    v9 = a1;
label_f17b:
    if (0 <= v2) { // branch-flip
      v6 = (char *)sub_e9c2(v9,(long)v5 - (long)v9,a0);
      if (!v6)
        return -1;
      v1 = *v6;
      v7 = *(long **)(a0 + ((long)(int)(unsigned int)(unsigned char)sub_c863(v3,(int)v1) + 0x2e) * 8);
      v9 = &v6[1];
    }
    else {
      while( true ) {
        v6 = &v9[1];
        v1 = *v9;
        v7 = *(long **)(a0 + ((long)(int)(unsigned int)(unsigned char)sub_c863(v3,(int)v1) + 0x2e) * 8);
        v9 = v6;
        if (v7) break;
        if (v5 <= v6)
          return -1;
      }
    }
label_f24b:
    do {
      if (*v7) break;
      if (v5 <= v9)
        return -1;
      v6 = &v9[1];
      v4 = sub_c863(v3,(int)*v9);
      v11 = (unsigned long *)v7[1];
      while (v4 != *(unsigned char *)&v11[3]) {
        v11 = (*(unsigned char *)&v11[3] <= v4) ? (unsigned long *)v11[1] : (unsigned long *)*v11; // branch-flip
        if (!v11) {
          v7 = (long *)v7[4];
          if (!v7) {
            v7 = *(long **)(a0 + ((long)(int)(unsigned int)v4 + 0x2e) * 8);
            v9 = v6;
            if (v7) goto label_f24b;
            if (v5 <= v6)
              return -1;
            goto label_f17b;
          }
          if (*v7) goto label_f366;
          v11 = (unsigned long *)v7[1];
        }
      }
      v7 = (long *)v11[2];
      v9 = v6;
    } while( true );
  }
label_f366:
  for (v8 = v7; *v8 < 0; v8 = (long *)v8[4]) {
  }
  v10 = (long)v9 - v8[5];
  if (a4) {
    while (v9 < v5) {
      v6 = &v9[1];
      v4 = sub_c863(v3,(int)*v9);
      do {
        v11 = (unsigned long *)v7[1];
        while ((v11 && (v4 != *(unsigned char *)&v11[3]))) {
          v11 = (*(unsigned char *)&v11[3] <= v4) ? (unsigned long *)v11[1] : (unsigned long *)*v11; // branch-flip
        }
      } while (((!v11) && (v7 = (long *)v7[4], v7)) && (v8[5] <= v7[5]));
      if (!v11) break;
      v7 = (long *)v11[2];
      v9 = v6;
      if (*v7) {
        for (v12 = v7; *v12 < 0; v12 = (long *)v12[4]) {
        }
        if ((unsigned long)((long)v6 - v12[5]) <= v10) {
          v8 = v12;
          v10 = (long)v6 - v12[5];
        }
      }
    }
  }
  *a3 = *v8 >> 1;
  a3[1] = v10 - (long)a1;
  a3[2] = v8[5];
  return v10 - (long)a1;
}

// Function: sub_f50c @ 0xf50c
void sub_f50c(long a0,char *a1,long a2,long *a3,char a4) // return-dupe
{
  if (*(long *)(a0 + 0x980)) {
    sub_f0f4(a0,a1,a2,a3,(unsigned char)a4);
    return;
  }
  sub_f0f4(a0,a1,a2,a3,(unsigned char)a4);
}

// Function: sub_f585 @ 0xf585
void sub_f585(long a0,unsigned long a1,unsigned long a2,unsigned long a3,char a4)
{
  (**(void **)(a0 + 0x998))(a0,a1,a2,a3,(unsigned char)a4);
}

// Function: sub_f5d1 @ 0xf5d1
void sub_f5d1(obstack *a0)
{
  _obstack_free(a0,NULL);
  free(a0);
}

// Function: sub_f64d @ 0xf64d
char sub_f64d(char a0)
{
  return a0;
}

// Function: sub_f660 @ 0xf660
void sub_f660(void)
{
  return;
}

// Function: sub_f673 @ 0xf673
void sub_f673(void)
{
  return;
}

// Function: sub_f686 @ 0xf686
unsigned long sub_f686(char *a0,unsigned long a1,mbstate_t *a2)
{
  return sub_21b3b(a0,a1,a2);
}

// Function: sub_f6da @ 0xf6da
long sub_f6da(char *a0,unsigned long a1,mbstate_t *a2)
{
  unsigned char v1; // al
  int v2; // eax
  long v3; // rax
  
  v2 = (int)*a0;
  v1 = sub_f64d(v2);
  v2 = (unsigned int)v1;
  v3 = (long)v2;
  if (*(char *)(v3 + 0x331a3) != '\xfe') // branch-flip
    v3 = (long)*(char *)(v3 + 0x331a3);
  else {
    v3 = sub_f686(a0,a1,a2);
  }
  return v3;
}

// Function: sub_f73c @ 0xf73c
unsigned int sub_f73c(unsigned int a0)
{
  if ((a0 != 0x5f) && (!iswalnum(a0)))
    return 0;
  return 1;
}

// Function: sub_f770 @ 0xf770
void sub_f770(void)
{
  unsigned int v1;
  int v2; // stack - 0xc
  
  for (v2 = 0; v2 <= 0xff; v2 = v2 + 1) {
    v1 = *(unsigned int *)(((long)v2 + 0x40) * 4 + 0x331a4);
    *(char *)((long)v2 + 0x339e0) = sub_f73c(v1);
  }
}

// Function: sub_f7c8 @ 0xf7c8
void sub_f7c8(bool a0)
{
  int v1; // stack - 0x14
  long v2; // stack - 0x10
  
  v2 = 0;
  if (dat_33180) {
    if ((__ctype_get_mb_cur_max() == 1) || (a0)) {
      v2 = sub_26b10(0x100);
      for (v1 = 0; v1 <= 0xff; v1 = v1 + 1) {
        *(char *)(v2 + v1) = (char)toupper(v1);
      }
    }
  }
  sub_c89b(v2);
}

// Function: sub_f84a @ 0xf84a
long sub_f84a(unsigned long *a0,long *a1,char *a2,long a3) // early-return
{
  long v1; // rax
  unsigned long v2; // stack - 0x18
  int v3; // stack - 0x3c
  char *v4; // stack - 0x38
  char *v5; // stack - 0x30
  long v6; // stack - 0x28
  
  v5 = (char *)*a0;
  if (v5 < a2) {
    if (dat_331a2) { // branch-flip
      v4 = a2;
      if (((int)*a2 & 0xc0U) == 0x80) {
        for (v3 = 1; v3 <= 3; v3 = v3 + 1) {
          if (((int)a2[-v3] & 0xc0U) != 0x80) {
            if (!((int)(~(int)a2[-v3] & 0xffU) >> (7U - (char)v3 & 0x1fU))) {
              v2 = 0;
              v1 = sub_f686(&a2[-(long)v3],a3 - ((long)a2 - (long)v3),&v2);
              if (0 <= v1) {
                v5 = &a2[-(long)v3];
                v4 = &v5[v1];
              }
            }
            break;
          }
        }
      }
    }
    else {
      v2 = 0;
      v4 = v5;
      do {
        v6 = sub_f6da(v4,a3 - (long)v4,&v2);
        if (v6 <= -1) {
          v6 = 1;
          memset(&v2,0,8);
        }
        v5 = v4;
        v4 = &v4[v6];
      } while (v4 < a2);
      if (a1)
        *a1 = v6;
    }
    *a0 = (unsigned long)v4;
    if (v4 == a2)
      return 0;
  }
  return (long)a2 - (long)v5;
}

// Function: sub_fa82 @ 0xfa82
long sub_fa82(long a0,long a1,char a2)
{
  char *v1;
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  unsigned int v4; // stack - 0x2c
  long v5; // stack - 0x28
  long v6; // stack - 0x20
  
  v5 = 0;
  v3 = 0;
  do {
    if (a1 - a0 <= v5)
      return v5;
    if (*(char *)((long)(int)(unsigned int)*(unsigned char *)(a0 + v5) + 0x339e0)) // branch-flip
      v5 += 1;
    else {
      if (*(char *)((long)(int)(unsigned int)*(unsigned char *)(a0 + v5) + 0x331a3) != '\xfe')
        return v5;
      v4 = 0;
      v2 = (a1 - a0) - v5;
      v1 = (char *)(v5 + a0);
      v6 = sub_21b80(&v4,v1,v2,&v3);
      if (sub_f73c(v4) != '\x01')
        return v5;
      v5 += (unsigned long)(v6 == 0) + v6;
    }
    if (a2 != '\x01')
      return v5;
  } while( true );
}

// Function: sub_fbb2 @ 0xfbb2
void sub_fbb2(long a0,long a1)
{
  sub_fa82(a0,a1,1);
}

// Function: sub_fbe0 @ 0xfbe0
void sub_fbe0(long a0,long a1)
{
  sub_fa82(a0,a1,0);
}

// Function: sub_fc0e @ 0xfc0e
unsigned long sub_fc0e(long a0,long a1,long a2) // early-return
{
  unsigned char *v1;
  unsigned char v2; // al
  unsigned long v3; // rax
  long v4; // stack - 0x18
  
  if (a0 == a1) {
    v3 = 0;
    return v3;
  }
  v1 = (unsigned char *)(a1 + -1);
  if ((dat_331a0 != '\x01') || (v2 = *v1 >> 7, v2 = ~v2, v2 = v2 & dat_331a2, v2)) {
    v3 = (long)(int)(unsigned int)*v1;
    v3 = (unsigned long)*(unsigned char *)(v3 + 0x339e0);
  }
  else {
    v4 = a0;
    v3 = sub_f84a(&v4,NULL,v1,a2);
    v3 = sub_fbe0((long)v1 - v3,a2);
  }
  return v3;
}

// Function: sub_fce9 @ 0xfce9
void sub_fce9(void)
{
  return;
}

// Function: sub_fcfc @ 0xfcfc
void sub_fcfc(void)
{
  return;
}

// Function: sub_fd0f @ 0xfd0f
void sub_fd0f(unsigned long a0)
{
  if ((long)a0 <= -1)
    sub_27122(); // no-return
  sub_26b10(a0);
}

// Function: sub_fd3f @ 0xfd3f
void sub_fd3f(void *a0)
{
  free(a0);
}

// Function: sub_fd62 @ 0xfd62
int sub_fd62(unsigned long *a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int a4)
{
  unsigned long v1;
  int v2;
  int v3 [3]; // stack - 0x34
  long v4; // stack - 0x28
  long v5; // stack - 0x20
  unsigned long v6; // stack - 0x18
  
  while( true ) {
    while( true ) {
      v3[1] = 0x2000;
      v4 = 0x7fffffffffffffff;
      v3[2] = pcre2_match_8(a0[1],a1,a2,a3,a4,a0[3],a0[2]);
      if ((v3[2] != -0x2e) || (v4 / 2 < (long)a0[5])) break;
      v5 = a0[5];
      a0[5] = v5 * 2;
      v6 = a0[5];
      pcre2_jit_stack_free_8(a0[4]);
      v1 = *a0;
      a0[4] = pcre2_jit_stack_create_8(v5,v6,v1);
      if (!a0[4])
        sub_27122(); // no-return
      if (!a0[2]) {
        v1 = *a0;
        a0[2] = pcre2_match_context_create_8(v1);
      }
      pcre2_jit_stack_assign_8(a0[2],0,a0[4]);
    }
    if (v3[2] != -0x35)
      return v3[2];
    pcre2_config_8(7,v3);
    v2 = v3[0] * 2;
    if (v3[0] <= -1) break;
    v3[0] = v2;
    if (!a0[2]) {
      v1 = *a0;
      a0[2] = pcre2_match_context_create_8(v1);
    }
    pcre2_set_depth_limit_8(a0[2],v3[0]);
  }
  return v3[2];
}

// Function: sub_ff6e @ 0xff6e
unsigned long sub_ff6e(void)
{
  return 0;
}

// Function: sub_ff80 @ 0xff80
unsigned long * sub_ff80(void *a0,unsigned long a1) // ternary
{
  int v1; // eax
  char v10 [264];
  unsigned long v11; // stack - 0x188
  void *v12; // stack - 0x180
  unsigned int v13; // stack - 0x174
  void *v14; // stack - 0x168
  void *v15; // stack - 0x160
  unsigned long *v16; // stack - 0x158
  unsigned long v17; // stack - 0x150
  unsigned long v18; // stack - 0x148
  unsigned long v19; // stack - 0x140
  unsigned int v2; // eax
  void *v20; // stack - 0x138
  void *v21; // stack - 0x130
  unsigned long *v3; // rax
  unsigned long *v4; // rax
  unsigned long v5; // rax
  unsigned long *v6; // rax
  unsigned long *v7; // rax
  char v8 [8];
  int v9; // stack - 0x178
  
  v13 = (dat_33180) ? 0x18 : 0x10; // branch-flip
  v15 = (void *)((long)a0 + a1);
  v16 = (unsigned long *)sub_26b10(0x38);
  *v16 = pcre2_general_context_create_8(sub_fd0f,sub_fd3f,0);
  v17 = *v16;
  v18 = pcre2_compile_context_create_8(v17);
  if (dat_331a0) {
    if (dat_331a2 != '\x01') {
      error(2,0,gettext("-P supports only unibyte and UTF-8 locales"));
      return v3;
    }
    v13 |= 0x4080000;
  }
  if (v15 == rawmemchr(a0,10)) {
    v14 = NULL;
    v11 = a1;
    v12 = a0;
    if (dat_33182) // branch-flip
      pcre2_set_compile_extra_options_8(v18,8);
    else if (dat_33181) {
      v19 = a1 + 0x11;
      v14 = (void *)sub_26b10(v19);
      v20 = v14;
      v21 = mempcpy(v14,"(?<!\\w)(?:)(?!\\w)invalid argument %s for %s",10);
      v21 = mempcpy(v21,a0,a1);
      memcpy(v21,")(?!\\w)invalid argument %s for %s",7);
      v12 = v20;
      v11 = v19;
    }
    v5 = pcre2_maketables_8(v17);
    pcre2_set_character_tables_8(v18,v5);
    v5 = pcre2_compile_8(v12,v11,v13,&v9,v8,v18);
    v16[1] = v5;
    if (v16[1]) {
      free(v14);
      pcre2_compile_context_free_8(v18);
      v16[2] = 0;
      v5 = pcre2_match_data_create_from_pattern_8(v16[1],v17);
      v16[3] = v5;
      v1 = pcre2_jit_compile_8(v16[1],1);
      v9 = v1;
      if (((v1) && (v1 != -0x2d)) && (v1 != -0x30)) {
        error(2,0,gettext("JIT internal error: %d"),v1);
        return v7;
      }
      v16[4] = 0;
      v16[5] = 0x8000;
      v2 = sub_fd62(v16,0x2a907,0,0,1);
      *(unsigned int *)&v16[6] = v2;
      v2 = sub_fd62(v16,0x2a907,0,0,0);
      *(unsigned int *)((long)v16 + 0x34) = v2;
      return v16;
    }
    pcre2_get_error_message_8(v9,v10,0x100);
    error(2,0,"%s",v10);
    return v6;
  }
  error(2,0,gettext("the -P option only supports a single pattern"));
  return v4;
}

// Function: sub_10409 @ 0x10409
long sub_10409(unsigned long *a0,char *a1,long a2,long *a3,char *a4) // early-return
{
  char v1;
  unsigned int v10; // stack - 0x7c
  char *v11; // stack - 0x78
  char *v12; // stack - 0x70
  char *v13; // stack - 0x68
  char *v14; // stack - 0x60
  char *v15; // stack - 0x58
  long *v2; // rax
  char *v3; // rax
  long v4; // rax
  long v5; // rax
  unsigned long v6; // rax
  long v7; // rax
  bool v8; // stack - 0x81
  int v9; // stack - 0x80
  
  if (a4) // branch-flip
    v11 = a4;
  else {
    v11 = a1;
  }
  v8 = v11[-1] == dat_33183;
  v2 = (long *)pcre2_get_ovector_pointer_8(a0[3]);
  v12 = a1;
  v13 = a1;
  do {
    v3 = rawmemchr(v11,(int)dat_33183);
    while( true ) {
      while (v1 = *v11, *(char *)((long)(int)(unsigned int)(unsigned char)sub_f64d((int)v1) + 0x331a3) == '\xff') {
        v11 = &v11[1];
        v8 = 0;
        v13 = v11;
      }
      v4 = (long)v11 - (long)v13;
      if (v11 == v3) break;
      v10 = (unsigned int)(v8 != 1);
      v9 = sub_fd62(a0,v13,(long)v3 - (long)v13,v4,v10);
      if (sub_ff6e(v9) != '\x01') goto label_10655;
      v5 = pcre2_get_startchar_8(a0[3]);
      if (v4 <= v5) {
        if (v5) // branch-flip
          v9 = sub_fd62(a0,v13,v5,v4,v10 | 0x40000002);
        else {
          *v2 = 0;
          v2[1] = 0;
          v9 = *(int *)((long)a0 + ((long)(int)(unsigned int)v8 + 0xc) * 4);
        }
        if (v9 != -1) goto label_10655;
        v11 = &v13[v5 + 1];
        v8 = 0;
      }
      v13 = &v13[v5 + 1];
    }
    v2[1] = v4;
    *v2 = v2[1];
    v9 = *(int *)((long)a0 + ((long)(int)(unsigned int)v8 + 0xc) * 4);
label_10655:
    if (v9 != -1) break;
    v8 = 1;
    v11 = &v3[1];
    v12 = v11;
    v13 = v11;
  } while (v11 < &a1[a2]);
  if (1 <= v9) {
    v14 = &v13[*v2];
    v15 = &v13[v2[1]];
    if (!a4) {
      v14 = v12;
      v15 = &v3[1];
    }
    *a3 = (long)v15 - (long)v14;
    return (long)v14 - (long)a1;
  }
  if (-0x2d <= v9) { // branch-flip
    if (v9 == -1)
      return -1;
  }
  else if (-0x3f <= v9) {
    switch(v9) {
      case 0xffffffcb:
label_10787:
        v6 = sub_6eac();
        error(2,0,gettext("%s: exceeded PCRE\'s nested backtracking limit"),v6);
        goto label_107b8;
      case 0xffffffcc:
label_107b8:
        v6 = sub_6eac();
        error(2,0,gettext("%s: PCRE detected recurse loop"),v6);
        goto label_107e9;
      case 0xffffffc1:
label_107e9:
        v6 = sub_6eac();
        error(2,0,gettext("%s: exceeded PCRE\'s heap limit"),v6);
        break;
      case 0xffffffd0:
        v6 = sub_6eac();
        error(2,0,gettext("%s: memory exhausted"),v6);
        goto label_10725;
      case 0xffffffd1:
label_10756:
        v6 = sub_6eac();
        error(2,0,gettext("%s: exceeded PCRE\'s backtracking limit"),v6);
        goto label_10787;
      case 0xffffffd2:
label_10725:
        v6 = sub_6eac();
        error(2,0,gettext("%s: exhausted PCRE JIT stack"),v6);
        goto label_10756;
      
    }
  }
  v3 = (char *)sub_6eac();
  error(2,0,gettext("%s: internal PCRE error: %d"),v3,v9);
  return v7;
}

// Function: sub_108d8 @ 0x108d8
void sub_108d8(void)
{
  sub_99b6(1); // no-return
}

// Function: sub_108ed @ 0x108ed
long sub_108ed(char *a0,long a1,long a2,unsigned long a3) // early-return
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

// Function: sub_10a20 @ 0x10a20
long sub_10a20(char *a0,long a1)
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

// Function: sub_10a96 @ 0x10a96
void sub_10a96(char *a0,char *a1,long a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (a2 != -1) // branch-flip
    v1 = gettext("ambiguous argument %s for %s");
  else {
    v1 = gettext("invalid argument %s for %s");
  }
  v2 = sub_24bee(1,a0);
  error(0,0,v1,sub_24706(0,8,a1),v2);
}

// Function: sub_10b2f @ 0x10b2f
void sub_10b2f(long a0,long a1,unsigned long a2)
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
      if (memcmp(v4,(void *)(a2 * v3 + a1),a2)) goto label_10bb0;
      v1 = *(char **)(a0 + v3 * 8);
      fprintf(stderr,", %s",(char *)sub_24c1b(v1));
    }
    else {
label_10bb0:
      v1 = *(char **)(a0 + v3 * 8);
      fprintf(stderr,"\n  - %s",(char *)sub_24c1b(v1));
      v4 = (void *)(a1 + a2 * v3);
    }
    v3 += 1;
  } while( true );
}

// Function: sub_10c86 @ 0x10c86
long sub_10c86(char *a0,char *a1,long a2,long a3,unsigned long a4,void *a5,unsigned int a6) // early-return, ternary
{
  long v1; // stack - 0x10
  
  v1 = ((char)a6) ? sub_108ed(a1,a2,a3,a4) : sub_10a20(a1,a2); // branch-flip
  if (0 <= v1)
    return v1;
  sub_10a96(a0,a1,v1);
  sub_10b2f(a2,a3,a4);
  (*a5)();
  return -1;
}

// Function: sub_10d35 @ 0x10d35
unsigned long sub_10d35(void *a0,long a1,long a2,unsigned long a3)
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

// Function: sub_10dc5 @ 0x10dc5
unsigned long sub_10dc5(int a0) // return-dupe
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

// Function: sub_10e03 @ 0x10e03
unsigned long sub_10e03(int a0) // return-dupe
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

// Function: sub_10e35 @ 0x10e35
bool sub_10e35(unsigned int a0)
{
  return a0 < 0x80;
}

// Function: sub_10e56 @ 0x10e56
unsigned int sub_10e56(int a0) // return-dupe
{
  if ((a0 != 0x20) && (a0 != 9))
    return 0;
  return 1;
}

// Function: sub_10e7e @ 0x10e7e
unsigned long sub_10e7e(int a0) // return-dupe
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

// Function: sub_10eab @ 0x10eab
bool sub_10eab(int a0)
{
  return (unsigned int)(a0 - 0x30U) < 10;
}

// Function: sub_10ecf @ 0x10ecf
bool sub_10ecf(int a0)
{
  return (unsigned int)(a0 - 0x21U) < 0x5e;
}

// Function: sub_10ef3 @ 0x10ef3
bool sub_10ef3(int a0)
{
  return (unsigned int)(a0 - 0x61U) < 0x1a;
}

// Function: sub_10f17 @ 0x10f17
bool sub_10f17(int a0)
{
  return (unsigned int)(a0 - 0x20U) < 0x5f;
}

// Function: sub_10f3b @ 0x10f3b
unsigned long sub_10f3b(int a0) // return-dupe, return-dupe x3
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

// Function: sub_10f85 @ 0x10f85
unsigned long sub_10f85(int a0) // return-dupe
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

// Function: sub_10fb2 @ 0x10fb2
bool sub_10fb2(int a0)
{
  return (unsigned int)(a0 - 0x41U) < 0x1a;
}

// Function: sub_10fd6 @ 0x10fd6
unsigned long sub_10fd6(int a0) // return-dupe
{
  if (((unsigned int)(a0 - 0x30U) <= 0x36) && (0x7e0000007e03ffU >> ((unsigned char)(a0 - 0x30U) & 0x3f) & 1))
    return 1;
  return 0;
}

// Function: sub_1101e @ 0x1101e
int sub_1101e(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_11041 @ 0x11041
int sub_11041(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_11064 @ 0x11064
void sub_11064(int a0)
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
  (*dat_43ae0)(a0);
  if (a0) // branch-flip
    v1 = dat_43ae8;
  else {
    v1 = dat_43af0;
  }
  v2 = strlen(dat_43af8);
  v3 = strlen(v1);
  if (0x1000 <= v3 + v2) { // branch-flip
    write(2,dat_43af8,v2);
    write(2,": write error",2);
    write(2,v1,v3);
    v5[0] = 10;
    v7 = (char *)0x1;
  }
  else {
    v4 = mempcpy(v5,dat_43af8,v2);
    v7 = mempcpy(mempcpy(v4,": write error",2),v1,v3);
    *v7 = 10;
    v7 = &v7[1 - (long)v5];
  }
  write(2,v5,(unsigned long)v7);
  if (!a0)
    _exit(dat_33080); // no-return
  raise(a0);
  abort(); // no-return
}

// Function: sub_1126d @ 0x1126d
void sub_1126d(void)
{
  return;
}

// Function: sub_1127b @ 0x1127b
unsigned long sub_1127b(unsigned long a0,int a1)
{
  if (!a1)
    return 0;
  sub_11064(0xb); // no-return
}

// Function: sub_112a7 @ 0x112a7
void sub_112a7(int a0)
{
  unsigned int v1; // eax
  
  if ((a0) && (!dat_43b00))
    v1 = 0xb;
  else {
    v1 = 0;
  }
  sub_11064(v1); // no-return
}

// Function: sub_112dd @ 0x112dd
void * sub_112dd(void *a0) // return-dupe
{
  int v1; // eax
  void *v2; // rax
  
  if (a0) // branch-flip
    v2 = a0;
  else {
    v2 = sub_1126d;
  }
  dat_43ae0 = v2;
  v2 = (char *)gettext("program error");
  dat_43ae8 = (char *)v2;
  v2 = (char *)gettext("stack overflow");
  dat_43af0 = (char *)v2;
  v2 = (unsigned long)sub_1ff0c();
  dat_43af8 = v2;
  v1 = sub_251ca(sub_112a7,0x33ae0,0x10000);
  if (v1) {
    v2 = (int *)__errno_location();
    *(int *)v2 = 0x5f;
    v2 = (void *)0xffffffff;
    return v2;
  }
  v1 = sub_2511a(sub_1127b);
  dat_43b00 = v1;
  v2 = NULL;
  return v2;
}

// Function: sub_11390 @ 0x11390
int sub_11390(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned char v2; // al
  
  if (a0 == a1)
    return 0;
  do {
    v1 = sub_1101e((unsigned char)*a0);
    v2 = sub_1101e((unsigned char)*a1);
    if (!v1) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while (v1 == v2);
  return (unsigned int)v1 - (unsigned int)v2;
}

// Function: sub_1140a @ 0x1140a
void sub_1140a(unsigned long a0)
{
  dat_43b08 = a0;
}

// Function: sub_11424 @ 0x11424
void sub_11424(char a0)
{
  dat_43b10 = a0;
}

// Function: sub_1143e @ 0x1143e
void sub_1143e(void)
{
  int v1; // eax
  char *v2; // rax
  char *v3; // rax
  
  v1 = sub_279f4(stdout);
  if (v1) {
    if (dat_43b10 == '\x01') {
      if (*__errno_location() == 0x20) goto label_1150f;
    }
    v2 = gettext("write error");
    if (dat_43b08) { // branch-flip
      v3 = (char *)sub_24915(dat_43b08);
      error(0,*__errno_location(),"%s: %s",v3,v2);
    }
    else {
      error(0,*__errno_location(),"%s",v2);
    }
    _exit(dat_33080); // no-return
  }
label_1150f:
  v1 = sub_279f4(stderr);
  if (v1)
    _exit(dat_33080); // no-return
}

// Function: sub_11536 @ 0x11536
unsigned long sub_11536(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = strcmp(a0,a1);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}

// Function: sub_11564 @ 0x11564
unsigned int sub_11564(char a0) // return-dupe
{
  if (('0' <= a0) && (a0 <= '9'))
    return 1;
  return 0;
}

// Function: sub_1158e @ 0x1158e
char sub_1158e(char a0)
{
  return a0;
}

// Function: sub_115a1 @ 0x115a1
unsigned int sub_115a1(int a0)
{
  return a0 >> 6 & 7;
}

// Function: sub_115b7 @ 0x115b7
unsigned int sub_115b7(int a0)
{
  return a0 >> 3 & 7;
}

// Function: sub_115cd @ 0x115cd
unsigned int sub_115cd(unsigned int a0)
{
  return a0 & 7;
}

// Function: sub_115e0 @ 0x115e0
unsigned int sub_115e0(unsigned int a0,unsigned int a1,unsigned int a2)
{
  unsigned int v1; // eax
  unsigned int v2; // eax
  unsigned int v3; // eax
  
  if (a2 & 1) // branch-flip
    v1 = sub_115cd(a0);
  else {
    v1 = 0;
  }
  if (a2 & 2) // branch-flip
    v2 = sub_115b7(a0);
  else {
    v2 = 0;
  }
  if (a2 & 4) // branch-flip
    v3 = sub_115a1(a0);
  else {
    v3 = 0;
  }
  a1 = (v3 | v1 | v2) & a1;
  return CONCAT31((undefined3)(a1 >> 8),a1 != 0);
}

// Function: sub_1165b @ 0x1165b
bool sub_1165b(unsigned int a0)
{
  return (((int)a0 >> 2 ^ a0) & 0x49) != 0;
}

// Function: sub_11679 @ 0x11679
bool sub_11679(unsigned int a0)
{
  return (((int)a0 >> 1 ^ a0) & 0x49) != 0;
}

// Function: sub_11696 @ 0x11696
unsigned short sub_11696(long a0,long a1)
{
  short v1;
  
  v1 = *(short *)(a0 * 0x48 + *(long *)(a1 + 0x128) + 0x22);
  return CONCAT11((char)((unsigned short)v1 >> 8),v1 != 0);
}

// Function: sub_116d2 @ 0x116d2
void sub_116d2(unsigned int a0,unsigned int a1,long a2,long a3)
{
  sub_115e0(*(unsigned short *)(a2 * 0x48 + *(long *)(a3 + 0x128) + 0x22),a0,a1);
}

// Function: sub_11724 @ 0x11724
unsigned long sub_11724(int *a0,char *a1,unsigned long a2,long a3)
{
  int v1; // stack - 0x20
  int v2; // stack - 0x1c
  unsigned long v3; // stack - 0x18
  
  v2 = *(int *)(a3 + 0xc + ((long)(int)(unsigned int)(unsigned char)*a1 + 0x144) * 4);
  if (v2 == -1) {
    v3 = sub_21b80(&v1,a1,a2,(mbstate_t *)(a3 + 0x110));
    if ((v3) && (v3 <= 0xfffffffffffffffd)) {
      *a0 = v1;
      return v3;
    }
    memset((void *)(a3 + 0x110),0,8);
  }
  *a0 = v2;
  return 1;
}

// Function: sub_117fa @ 0x117fa
bool sub_117fa(unsigned int a0,long a1)
{
  return (*(unsigned long *)(a1 + (unsigned long)(a0 >> 6) * 8) >> ((unsigned char)a0 & 0x3f) & 1) != 0;
}

// Function: sub_11834 @ 0x11834
void sub_11834(unsigned int a0,long a1)
{
  *(unsigned long *)(a1 + (unsigned long)(a0 >> 6) * 8) = *(unsigned long *)(a1 + (unsigned long)(a0 >> 6) * 8) | 1L << ((unsigned char)a0 & 0x3f);
}

// Function: sub_11882 @ 0x11882
void sub_11882(unsigned int a0,long a1)
{
  *(unsigned long *)(a1 + (unsigned long)(a0 >> 6) * 8) = *(unsigned long *)(a1 + (unsigned long)(a0 >> 6) * 8) & ~(1L << ((unsigned char)a0 & 0x3f));
}

// Function: sub_118d3 @ 0x118d3
void sub_118d3(void *a0)
{
  memset(a0,0,0x20);
}

// Function: sub_118fc @ 0x118fc
void sub_118fc(long a0)
{
  int v1; // stack - 0xc
  
  for (v1 = 0; v1 <= 3; v1 = v1 + 1) {
    *(unsigned long *)(a0 + (long)v1 * 8) = 0xffffffffffffffff;
  }
}

// Function: sub_11934 @ 0x11934
void sub_11934(long a0)
{
  int v1; // stack - 0xc
  
  for (v1 = 0; v1 <= 3; v1 = v1 + 1) {
    *(unsigned long *)(a0 + (long)v1 * 8) = ~*(unsigned long *)(a0 + (long)v1 * 8);
  }
}

// Function: sub_11983 @ 0x11983
unsigned long sub_11983(long a0,long a1)
{
  unsigned long v1;
  unsigned long v2; // rax
  int v3; // stack - 0x14
  unsigned long v4; // stack - 0x10
  
  v4 = 0;
  for (v3 = 0; v3 <= 3; v3 = v3 + 1) {
    v2 = *(unsigned long *)(a1 + (long)v3 * 8) ^ *(unsigned long *)(a0 + (long)v3 * 8);
    v4 |= v2;
  }
  v1 = v2 >> 8;
  v2 = CONCAT71((undefined7)v1,v4 == 0);
  return v2;
}

// Function: sub_119db @ 0x119db
unsigned long sub_119db(long a0)
{
  unsigned long v1;
  unsigned long v2; // rax
  int v3; // stack - 0x14
  unsigned long v4; // stack - 0x10
  
  v4 = 0;
  for (v3 = 0; v3 <= 3; v3 = v3 + 1) {
    v2 = *(unsigned long *)(a0 + (long)v3 * 8);
    v4 |= v2;
  }
  v1 = v2 >> 8;
  v2 = CONCAT71((undefined7)v1,v4 == 0);
  return v2;
}

// Function: sub_11a1e @ 0x11a1e
void * sub_11a1e(void *a0,long a1,long *a2,long a3,long a4)
{
  void *v1; // rax
  
  if (*a2 <= a1) // branch-flip
    v1 = (void *)sub_26dc6(a0,a2,1,a3,a4);
  else {
    v1 = a0;
  }
  return v1;
}

// Function: sub_11a76 @ 0x11a76
long sub_11a76(struct_27 *a0,unsigned long *a1)
{
  unsigned long *v1;
  long v2;
  void *v3;
  unsigned long v4;
  long v5; // stack - 0x10
  
  v5 = 0;
  while( true ) {
    if (a0->field_0x8 <= v5) {
      v2 = a0->field_0x8;
      v3 = a0->field_0x0;
      a0->field_0x0 = (void *)sub_11a1e(v3,v2,&a0[1],0x7ffffffffffffeeb,0x20);
      a0->field_0x8 = a0->field_0x8 + 1;
      v1 = (unsigned long *)((long)a0->field_0x0 + v5 * 0x20);
      v4 = a1[1];
      *v1 = *a1;
      v1[1] = v4;
      v4 = a1[3];
      v1[2] = a1[2];
      v1[3] = v4;
      return v5;
    }
    v3 = a0->field_0x0;
    if (sub_11983(a1,(void *)((long)v3 + v5 * 0x20))) break;
    v5 += 1;
  }
  return v5;
}

// Function: sub_11b61 @ 0x11b61
unsigned int sub_11b61(long a0,unsigned char a1)
{
  if ((*(int *)(a0 + 0xc + ((long)(int)(unsigned int)a1 + 0x144) * 4) != -1) && ((*(unsigned short *)((unsigned long)a1 * 2 + *(long *)__ctype_b_loc()) & 8 || (a1 == 0x5f))))
    return 1;
  return 0;
}

// Function: sub_11bc7 @ 0x11bc7
unsigned long sub_11bc7(long a0,char a1) // return-dupe x2
{
  if ((a1 == *(char *)(a0 + 0x1ce)) && (!(*(unsigned int *)(a0 + 0x1c8) & 1)))
    return 4;
  if (sub_11b61(a0,(unsigned char)a1))
    return 2;
  return 1;
}

// Function: sub_11c28 @ 0x11c28
bool sub_11c28(unsigned int a0,long a1)
{
  int v1; // eax
  
  v1 = wctob(a0);
  if (0 <= v1)
    sub_11834(v1,a1);
  return 0 <= v1;
}

// Function: sub_11c6d @ 0x11c6d
void sub_11c6d(int a0,long a1)
{
  int v1; // eax
  int v2; // stack - 0x10
  
  v1 = toupper(a0);
  for (v2 = 0; v2 <= 0xff; v2 = v2 + 1) {
    if (v1 == toupper(v2))
      sub_11834(v2,a1);
  }
}

// Function: sub_11cc7 @ 0x11cc7
unsigned int sub_11cc7(long a0)
{
  char v1;
  int v2; // eax
  unsigned int v3; // eax
  
  v2 = sub_11724((int *)(a0 + 0x48),*(char **)(a0 + 0x20),*(unsigned long *)(a0 + 0x28),a0);
  if (v2 != 1) // branch-flip
    v3 = 0xffffffff;
  else {
    v1 = **(char **)(a0 + 0x20);
    v3 = (unsigned int)(unsigned char)sub_1158e((int)v1);
  }
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + (long)v2;
  *(long *)(a0 + 0x28) = *(long *)(a0 + 0x28) - (long)v2;
  return v3;
}

// Function: sub_11d5f @ 0x11d5f
void sub_11d5f(long a0)
{
  if (!*(long *)(a0 + 0x28))
    sub_513f(gettext("unbalanced [")); // no-return
  sub_11cc7(a0);
}

// Function: sub_11da1 @ 0x11da1
long sub_11da1(char *a0)
{
  char *v1;
  int v2; // stack - 0xc
  
  v2 = 0;
  while( true ) {
    if (!*(long *)((long)v2 * 0x18 + 0x32880))
      return 0;
    v1 = *(char **)((long)v2 * 0x18 + 0x32880);
    if (sub_11536(a0,v1)) break;
    v2 += 1;
  }
  return (long)v2 * 0x18 + 0x32880;
}

// Function: sub_11e3e @ 0x11e3e
long sub_11e3e(long a0) // early-return x2, ternary x3
{
  unsigned int v1;
  char v10 [32];
  char v11 [40]; // stack - 0x48
  char v12 [140];
  bool v13; // zf
  unsigned char v14;
  char v15; // stack - 0x13a
  int v16; // stack - 0x134
  int v17; // stack - 0x130
  int v18; // stack - 0x12c
  int v19; // stack - 0x128
  void *v2;
  int v20; // stack - 0x124
  int v21; // stack - 0x120
  int v22; // stack - 0x11c
  int v23; // stack - 0x118
  int v24; // stack - 0x114
  int v25; // stack - 0x110
  int v26; // stack - 0xd8
  unsigned char v3; // al
  int v4; // eax
  int v5;
  char *v6;
  long v7;
  unsigned long v8; // rax
  void *v9;
  
  v15 = '\x01';
  *(unsigned long *)(a0 + 0x68) = 0;
  sub_118d3(v10);
  v16 = sub_11d5f(a0);
  v13 = v16 == 0x5e;
  if (v13) {
    v16 = sub_11d5f(a0);
    v15 = *(char *)(a0 + 0x419);
  }
  v17 = *(int *)(a0 + 0x48);
  v14 = v16 == 0x3a;
  do {
    v18 = 0x100;
    v14 &= 0xfd;
    if (v16 != 0x5b) { // branch-flip
label_1217c:
      if ((v16 == 0x5c) && (*(unsigned long *)(a0 + 0x1c0) & 1)) {
        v16 = sub_11d5f(a0);
        v17 = *(int *)(a0 + 0x48);
      }
      if (v18 == 0x100) {
        v18 = sub_11d5f(a0);
        v19 = *(int *)(a0 + 0x48);
      }
      if (v18 == 0x2d) {
        v22 = sub_11d5f(a0);
        v23 = *(int *)(a0 + 0x48);
        if ((v22 == 0x5b) && (**(char **)(a0 + 0x20) == '.')) {
          v15 = '\0';
          v22 = 0x5d;
        }
        if (v22 != 0x5d) { // branch-flip
          if ((v22 == 0x5c) && (*(unsigned long *)(a0 + 0x1c0) & 1)) {
            v22 = sub_11d5f(a0);
            v23 = *(int *)(a0 + 0x48);
          }
          v14 |= 8;
          v18 = sub_11d5f(a0);
          v19 = *(int *)(a0 + 0x48);
          if ((v17 != v23) || (v17 == -1)) {
            if (!*(char *)(a0 + 0x419)) {
              v3 = sub_11564((int)(char)v16);
              if (!(sub_11564((int)(char)v22) & v3)) {
                v15 = '\0';
                goto label_125c8;
              }
            }
            for (v24 = v16; v24 <= v22; v24 = v24 + 1) {
              if ((*(char *)(a0 + 0x1cd)) && (*(unsigned short *)((long)v24 * 2 + *(long *)__ctype_b_loc()) & 0x400))
                sub_11c6d(v24,v10);
              else {
                sub_11834(v24,v10);
              }
            }
            goto label_125c8;
          }
        }
        else {
          *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + -1;
          *(long *)(a0 + 0x28) = *(long *)(a0 + 0x28) + 1;
        }
      }
      v3 = (v16 != 0x3a) ? 4 : 2; // branch-flip
      v14 |= v3;
      if (*(char *)(a0 + 0x418) != '\x01') { // branch-flip
        if ((*(char *)(a0 + 0x1cd)) && (*(unsigned short *)((long)v16 * 2 + *(long *)__ctype_b_loc()) & 0x400))
          sub_11c6d(v16,v10);
        else {
          sub_11834(v16,v10);
        }
      }
      else if (v17 != -1) { // branch-flip
        v5 = (*(char *)(a0 + 0x1cd)) ? sub_1c01a(v17,v12) + 1 : 1; // branch-flip
        v26 = v17;
        for (v25 = 0; v25 < v5; v25 = v25 + 1) {
          v1 = *(unsigned int *)&v12[(long)v25 * 4 + -4];
          if (sub_11c28(v1,v10) != '\x01') {
            v7 = *(long *)(a0 + 0x68);
            v2 = *(void **)(a0 + 0x60);
            *(unsigned long *)(a0 + 0x60) = sub_11a1e(v2,v7,(long *)(a0 + 0x70),-1,4);
            v7 = *(long *)(a0 + 0x68);
            *(long *)(a0 + 0x68) = v7 + 1;
            *(unsigned int *)(*(long *)(a0 + 0x60) + v7 * 4) = *(unsigned int *)&v12[(long)v25 * 4 + -4];
          }
        }
      }
      else {
        v15 = '\0';
      }
    }
    else {
      v18 = sub_11d5f(a0);
      v19 = *(int *)(a0 + 0x48);
      if ((((v18 != 0x3a) || (!(*(unsigned long *)(a0 + 0x1c0) & 4))) && (v18 != 0x2e)) && (v18 != 0x3d)) goto label_1217c;
      v20 = 0;
      while ((v4 = sub_11d5f(a0), *(long *)(a0 + 0x28) && ((v4 != v18 || (**(char **)(a0 + 0x20) != ']'))))) {
        if (0x20 <= v20) // branch-flip
          v11[0] = '\0';
        else {
          v5 = v20 + 1;
          v11[v20] = (char)v4;
          v20 = v5;
        }
      }
      v11[v20] = '\0';
      sub_11d5f(a0);
      if (v18 != 0x3a) // branch-flip
        v15 = '\0';
      else {
        if ((*(char *)(a0 + 0x1cd)) && ((sub_11536(v11,"upper") || (sub_11536(v11,"lower")))))
          v6 = "alpha";
        else {
          v6 = v11;
        }
        v7 = sub_11da1(v6);
        if (!v7)
          sub_513f(gettext("invalid character class")); // no-return
        if ((*(char *)(a0 + 0x418)) && (*(char *)(v7 + 0x10) != '\x01'))
          v15 = '\0';
        else {
          for (v21 = 0; v21 <= 0xff; v21 = v21 + 1) {
            v9 = *(void **)(v7 + 8);
            if ((*v9)(v21))
              sub_11834(v21,v10);
          }
        }
      }
      v14 |= 8;
      v18 = sub_11d5f(a0);
      v19 = *(int *)(a0 + 0x48);
    }
label_125c8:
    v17 = v19;
    v16 = v18;
    if (v18 == 0x5d) {
      if (v14 == 7) {
        v9 = (*(unsigned int *)(a0 + 0x1c8) & 4) ? sub_513f : sub_5174; // branch-flip
        (*v9)(gettext("character class syntax is [[:space:]], not [:space:]"));
      }
      if (v15 != '\x01')
        return 0x112;
      if ((*(char *)(a0 + 0x418)) && ((v13 || (*(long *)(a0 + 0x68))))) {
        *(bool *)(a0 + 0x58) = v13;
        if (sub_119db(v10)) // branch-flip
          v8 = 0xffffffffffffffff;
        else {
          v8 = sub_11a76(a0,v10);
        }
        *(unsigned long *)(a0 + 0x50) = v8;
        return 0x113;
      }
      if ((v13) && (sub_11934(v10), *(unsigned long *)(a0 + 0x1c0) & 0x100))
        sub_11882(10,v10);
      return sub_11a76(a0,v10) + 0x114;
    }
  } while( true );
}

// Function: sub_1274c @ 0x1274c
void sub_1274c(long a0,struct_1 *a1,char *a2)
{
  a1->field_0x0 = *(unsigned long *)(a0 + 0x20);
  a1->field_0x8 = *(unsigned long *)(a0 + 0x28);
  *(char **)(a0 + 0x20) = a2;
  *(unsigned long *)(a0 + 0x28) = strlen(a2);
}

// Function: sub_127a9 @ 0x127a9
void sub_127a9(long a0,struct_1 *a1)
{
  *(unsigned long *)(a0 + 0x20) = a1->field_0x0;
  *(unsigned long *)(a0 + 0x28) = a1->field_0x8;
}

// Function: sub_127db @ 0x127db
unsigned long sub_127db(long a0) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_127db
{
  char *v1;
  char v10 [16];
  char v11 [104];
  bool v12; // stack - 0xea
  int v13; // stack - 0xe8
  int v14; // stack - 0xe4
  int v15; // stack - 0xe0
  int v16; // stack - 0xdc
  int v17; // stack - 0xd8
  char *v18; // stack - 0xd0
  char *v19; // stack - 0xc8
  char v2;
  bool v3; // al
  int v4; // eax
  unsigned int v5;
  char *v6; // rax
  unsigned long v7;
  long v8;
  char v9 [32];
  
  v12 = 0;
  v13 = 0;
label_13bcb:
  if (2 <= v13)
    abort(); // no-return
  if (!*(long *)(a0 + 0x28)) {
    *(unsigned long *)(a0 + 0x30) = 0xffffffffffffffff;
    return *(unsigned long *)(a0 + 0x30);
  }
  v17 = sub_11cc7(a0);
  switch(v17) {
    case 10:
      if (!(*(unsigned long *)(a0 + 0x1c0) & 0x800)) goto label_1395f;
      if (!v12) {
        *(char *)(a0 + 0x78) = 1;
        *(unsigned long *)(a0 + 0x30) = 0x106;
        return *(unsigned long *)(a0 + 0x30);
      }
      goto label_13ace;
    default:
      goto label_1395f;
    case 0x24:
      if ((!v12) && (((((*(unsigned long *)(a0 + 0x1c0) & 8 || (!*(long *)(a0 + 0x28))) || (((long)(unsigned long)((*(unsigned long *)(a0 + 0x1c0) & 0x2000) == 0) < *(long *)(a0 + 0x28) && (*(char *)((unsigned long)(**(char **)(a0 + 0x20) == '\\' && !(*(unsigned long *)(a0 + 0x1c0) & 0x2000)) + *(long *)(a0 + 0x20)) == ')')))) || (((long)(unsigned long)((*(unsigned long *)(a0 + 0x1c0) & 0x8000) == 0) < *(long *)(a0 + 0x28) && (*(char *)((unsigned long)(**(char **)(a0 + 0x20) == '\\' && !(*(unsigned long *)(a0 + 0x1c0) & 0x8000)) + *(long *)(a0 + 0x20)) == '|')))) || ((*(unsigned long *)(a0 + 0x1c0) & 0x800 && ((1 <= *(long *)(a0 + 0x28) && (**(char **)(a0 + 0x20) == '\n')))))))) {
        *(unsigned long *)(a0 + 0x30) = 0x10d;
        return *(unsigned long *)(a0 + 0x30);
      }
      goto label_13ace;
    case 0x27:
      if (v12 != 1) goto label_13ace;
      if (!(*(unsigned long *)(a0 + 0x1c0) & 0x80000)) {
        *(unsigned long *)(a0 + 0x30) = 0x10d;
        return *(unsigned long *)(a0 + 0x30);
      }
      goto label_1398c;
    case 0x28:
      if (v12 == ((*(unsigned long *)(a0 + 0x1c0) & 0x2000) == 0)) {
        *(long *)(a0 + 0x38) = *(long *)(a0 + 0x38) + 1;
        *(char *)(a0 + 0x78) = 1;
        *(unsigned long *)(a0 + 0x30) = 0x107;
        return *(unsigned long *)(a0 + 0x30);
      }
      goto label_13ace;
    case 0x29:
      if ((v12 == ((*(unsigned long *)(a0 + 0x1c0) & 0x2000) == 0)) && ((*(long *)(a0 + 0x38) || (!(*(unsigned long *)(a0 + 0x1c0) & 0x20000))))) {
        *(long *)(a0 + 0x38) = *(long *)(a0 + 0x38) + -1;
        *(char *)(a0 + 0x78) = 0;
        *(unsigned long *)(a0 + 0x30) = 0x108;
        return *(unsigned long *)(a0 + 0x30);
      }
      goto label_13ace;
    case 0x2a:
      if (v12) goto label_13ace;
      if (*(char *)(a0 + 0x78)) {
        if (!(*(unsigned long *)(a0 + 0x1c0) & 0x10)) goto label_1395f;
        if (*(unsigned int *)(a0 + 0x1c8) & 0x10)
          sub_5174(gettext("* at start of expression"));
      }
      *(unsigned long *)(a0 + 0x30) = 0x102;
      return *(unsigned long *)(a0 + 0x30);
    case 0x2b:
      if (*(unsigned long *)(a0 + 0x1c0) & 0x400) goto label_1395f;
      if (v12 != ((*(unsigned long *)(a0 + 0x1c0) & 2) != 0)) goto label_13ace;
      if (*(char *)(a0 + 0x78)) {
        if (!(*(unsigned long *)(a0 + 0x1c0) & 0x10)) goto label_1395f;
        if (*(unsigned int *)(a0 + 0x1c8) & 0x20)
          sub_5174(gettext("+ at start of expression"));
      }
      *(unsigned long *)(a0 + 0x30) = 0x103;
      return *(unsigned long *)(a0 + 0x30);
    case 0x2e:
      if (!v12) {
        if (*(long *)(a0 + 0x18) <= -1) {
          sub_118fc(v9);
          if (!(*(unsigned long *)(a0 + 0x1c0) & 0x40))
            sub_11882(10,v9);
          if (*(unsigned long *)(a0 + 0x1c0) & 0x80)
            sub_11882(0,v9);
          if (*(char *)(a0 + 0x418)) {
            for (v14 = 0; v14 <= 0xff; v14 = v14 + 1) {
              if (*(int *)(a0 + 0xc + ((long)v14 + 0x144) * 4) == -1)
                sub_11882(v14,v9);
            }
          }
          *(unsigned long *)(a0 + 0x18) = sub_11a76(a0,v9);
        }
        *(char *)(a0 + 0x78) = 0;
        if (*(char *)(a0 + 0x418)) // branch-flip
          v8 = 0x10a;
        else {
          v8 = *(long *)(a0 + 0x18) + 0x114;
        }
        *(long *)(a0 + 0x30) = v8;
        return *(unsigned long *)(a0 + 0x30);
      }
      goto label_13ace;
    case 0x31:
    case 0x32:
    case 0x33:
    case 0x34:
    case 0x35:
    case 0x36:
    case 0x37:
    case 0x38:
    case 0x39:
      if (v12 != 1) goto label_13ace;
      if (!(*(unsigned long *)(a0 + 0x1c0) & 0x4000)) {
        *(char *)(a0 + 0x78) = 0;
        *(unsigned long *)(a0 + 0x30) = 0x112;
        return *(unsigned long *)(a0 + 0x30);
      }
      goto label_1398c;
    case 0x3c:
      if (v12 != 1) goto label_13ace;
      if (!(*(unsigned long *)(a0 + 0x1c0) & 0x80000)) {
        *(unsigned long *)(a0 + 0x30) = 0x10e;
        return *(unsigned long *)(a0 + 0x30);
      }
      goto label_1398c;
    case 0x3e:
      if (v12 != 1) goto label_13ace;
      if (!(*(unsigned long *)(a0 + 0x1c0) & 0x80000)) {
        *(unsigned long *)(a0 + 0x30) = 0x10f;
        return *(unsigned long *)(a0 + 0x30);
      }
      goto label_1398c;
    case 0x3f:
      if (*(unsigned long *)(a0 + 0x1c0) & 0x400) goto label_1395f;
      if (v12 != ((*(unsigned long *)(a0 + 0x1c0) & 2) != 0)) goto label_13ace;
      if (*(char *)(a0 + 0x78)) {
        if (!(*(unsigned long *)(a0 + 0x1c0) & 0x10)) goto label_1395f;
        if (*(unsigned int *)(a0 + 0x1c8) & 0x20)
          sub_5174(gettext("? at start of expression"));
      }
      *(unsigned long *)(a0 + 0x30) = 0x101;
      return *(unsigned long *)(a0 + 0x30);
    case 0x42:
      if (v12 != 1) goto label_13ace;
      if (!(*(unsigned long *)(a0 + 0x1c0) & 0x80000)) {
        *(unsigned long *)(a0 + 0x30) = 0x111;
        return *(unsigned long *)(a0 + 0x30);
      }
      goto label_1398c;
    case 0x53:
    case 0x73:
      if (v12 != 1) goto label_13ace;
      if (!(*(unsigned long *)(a0 + 0x1c0) & 0x80000)) {
        if (*(char *)(a0 + 0x418) == '\x01') {
          sub_1274c(a0,v10,&"^[:space:]]"[(int)(unsigned int)(v17 == 0x73)]);
          *(unsigned long *)(a0 + 0x30) = sub_11e3e(a0);
          sub_127a9(a0,v10);
          *(char *)(a0 + 0x78) = 0;
          return *(unsigned long *)(a0 + 0x30);
        }
        sub_118d3(v9);
        for (v15 = 0; v15 <= 0xff; v15 = v15 + 1) {
          if (*(unsigned short *)((long)v15 * 2 + *(long *)__ctype_b_loc()) & 0x2000)
            sub_11834(v15,v9);
        }
        if (v17 == 0x53)
          sub_11934(v9);
        *(char *)(a0 + 0x78) = 0;
        *(long *)(a0 + 0x30) = sub_11a76(a0,v9) + 0x114;
        return *(unsigned long *)(a0 + 0x30);
      }
      goto label_1398c;
    case 0x57:
    case 0x77:
      if (v12 != 1) goto label_13ace;
      if (!(*(unsigned long *)(a0 + 0x1c0) & 0x80000)) {
        if (*(char *)(a0 + 0x418) == '\x01') {
          sub_1274c(a0,v10,&"^_[:alnum:]]"[(int)(unsigned int)(v17 == 0x77)]);
          *(unsigned long *)(a0 + 0x30) = sub_11e3e(a0);
          sub_127a9(a0,v10);
          *(char *)(a0 + 0x78) = 0;
          return *(unsigned long *)(a0 + 0x30);
        }
        sub_118d3(v9);
        for (v16 = 0; v16 <= 0xff; v16 = v16 + 1) {
          if (*(char *)(a0 + 0x1cf + (long)v16) == '\x02')
            sub_11834(v16,v9);
        }
        if (v17 == 0x57)
          sub_11934(v9);
        *(char *)(a0 + 0x78) = 0;
        *(long *)(a0 + 0x30) = sub_11a76(a0,v9) + 0x114;
        return *(unsigned long *)(a0 + 0x30);
      }
      goto label_1398c;
    case 0x5b:
      if (!v12) {
        *(char *)(a0 + 0x78) = 0;
        *(unsigned long *)(a0 + 0x30) = sub_11e3e(a0);
        return *(unsigned long *)(a0 + 0x30);
      }
      goto label_13ace;
    case 0x5c:
      if (v12) goto label_13ace;
      if (!*(long *)(a0 + 0x28))
        sub_513f(gettext("unfinished \\ escape")); // no-return
      v12 = 1;
      v13 += 1;
      goto label_13bcb;
    case 0x5d:
    case 0x7d:
      goto label_13ace;
    case 0x5e:
      if ((!v12) && ((((*(unsigned long *)(a0 + 0x1c0) & 8 || (*(long *)(a0 + 0x30) == -1)) || (*(long *)(a0 + 0x30) == 0x107)) || (*(long *)(a0 + 0x30) == 0x106)))) {
        *(unsigned long *)(a0 + 0x30) = 0x10c;
        return *(unsigned long *)(a0 + 0x30);
      }
      goto label_13ace;
    case 0x60:
      if (v12 != 1) goto label_13ace;
      if (!(*(unsigned long *)(a0 + 0x1c0) & 0x80000)) {
        *(unsigned long *)(a0 + 0x30) = 0x10c;
        return *(unsigned long *)(a0 + 0x30);
      }
      goto label_1398c;
    case 0x62:
      if (v12 != 1) goto label_13ace;
      if (!(*(unsigned long *)(a0 + 0x1c0) & 0x80000)) {
        *(unsigned long *)(a0 + 0x30) = 0x110;
        return *(unsigned long *)(a0 + 0x30);
      }
      goto label_1398c;
    case 0x7b:
      goto label_12eeb;
    case 0x7c:
      if (*(unsigned long *)(a0 + 0x1c0) & 0x400) goto label_1395f;
      if (v12 == ((*(unsigned long *)(a0 + 0x1c0) & 0x8000) == 0)) {
        *(char *)(a0 + 0x78) = 1;
        *(unsigned long *)(a0 + 0x30) = 0x106;
        return *(unsigned long *)(a0 + 0x30);
      }
      goto label_13ace;
    
  }
label_12eeb:
  if (!(*(unsigned long *)(a0 + 0x1c0) & 0x200)) goto label_1395f;
  if (v12 != ((*(unsigned long *)(a0 + 0x1c0) & 0x1000) == 0)) goto label_13ace;
  v18 = *(char **)(a0 + 0x20);
  v6 = &v18[*(long *)(a0 + 0x28)];
  *(unsigned int *)(a0 + 0x44) = 0xffffffff;
  *(unsigned int *)(a0 + 0x40) = *(unsigned int *)(a0 + 0x44);
  while ((v18 != v6 && (v2 = *v18, sub_11564((int)v2)))) {
    if (0 <= *(int *)(a0 + 0x40)) { // branch-flip
      v4 = (int)*v18 + *(int *)(a0 + 0x40) * 10;
      if (0x8031 <= v4)
        v4 = 0x8030;
    }
    else {
      v4 = (int)*v18;
    }
    *(int *)(a0 + 0x40) = v4 + -0x30;
    v18 = &v18[1];
  }
  if (v18 != v6) {
    if (*v18 != ',') // branch-flip
      *(unsigned int *)(a0 + 0x44) = *(unsigned int *)(a0 + 0x40);
    else {
      if (*(int *)(a0 + 0x40) <= -1)
        *(unsigned int *)(a0 + 0x40) = 0;
      while ((v18 = &v18[1], v18 != v6 && (v2 = *v18, sub_11564((int)v2)))) {
        if (0 <= *(int *)(a0 + 0x44)) { // branch-flip
          v4 = (int)*v18 + *(int *)(a0 + 0x44) * 10;
          if (0x8031 <= v4)
            v4 = 0x8030;
        }
        else {
          v4 = (int)*v18;
        }
        *(int *)(a0 + 0x44) = v4 + -0x30;
      }
    }
  }
  v3 = ((((v12) && ((v18 == v6 || (v1 = &v18[1], v2 = *v18, v18 = v1, v2 != '\\')))) || (v18 == v6)) || (((v1 = &v18[1], v2 = *v18, v18 = v1, v2 != '}' || (*(int *)(a0 + 0x40) <= -1)) || ((0 <= *(int *)(a0 + 0x44) && (*(int *)(a0 + 0x44) < *(int *)(a0 + 0x40)))))));
  if ((!v3) || (!(*(unsigned long *)(a0 + 0x1c0) & 0x200000))) {
    if (*(char *)(a0 + 0x78)) {
      if (!(*(unsigned long *)(a0 + 0x1c0) & 0x10)) {
label_1395f:
        if (v12 != 1) goto label_13ace;
label_1398c:
        if (*(unsigned int *)(a0 + 0x1c8) & 8) {
          v5 = *(unsigned int *)(a0 + 0x48);
          if (iswprint(v5)) { // branch-flip
            v5 = *(unsigned int *)(a0 + 0x48);
            if (iswspace(v5)) // branch-flip
              v19 = gettext("stray \\ before white space");
            else {
              v5 = *(unsigned int *)(a0 + 0x48);
              v5 = snprintf(v11,100,gettext("stray \\ before %lc"),v5);
              if ((0 <= (int)v5) && (v5 <= 99))
                v19 = v11;
              else {
                v19 = gettext("stray \\");
              }
            }
          }
          else {
            v19 = gettext("stray \\ before unprintable character");
          }
          sub_5174(v19);
        }
        goto label_13ace;
      }
      if (*(unsigned int *)(a0 + 0x1c8) & 0x20)
        sub_5174(gettext("{...} at start of expression"));
    }
    if (v3)
      sub_513f(gettext("invalid content of \\{\\}")); // no-return
    if (0x8000 <= *(int *)(a0 + 0x44))
      sub_513f(gettext("regular expression too big")); // no-return
    *(char **)(a0 + 0x20) = v18;
    *(long *)(a0 + 0x28) = (long)v6 - (long)v18;
    *(char *)(a0 + 0x78) = 0;
    *(unsigned long *)(a0 + 0x30) = 0x104;
    v7 = *(unsigned long *)(a0 + 0x30);
  }
  else {
label_13ace:
    *(char *)(a0 + 0x78) = 0;
    if (*(char *)(a0 + 0x418)) { // branch-flip
      *(unsigned long *)(a0 + 0x30) = 0x109;
      v7 = *(unsigned long *)(a0 + 0x30);
    }
    else if ((*(char *)(a0 + 0x1cd)) && (*(unsigned short *)((long)v17 * 2 + *(long *)__ctype_b_loc()) & 0x400)) {
      sub_118d3(v9);
      sub_11c6d(v17,v9);
      *(long *)(a0 + 0x30) = sub_11a76(a0,v9) + 0x114;
      v7 = *(unsigned long *)(a0 + 0x30);
    }
    else {
      *(long *)(a0 + 0x30) = (long)v17;
      v7 = *(unsigned long *)(a0 + 0x30);
    }
  }
  return v7;
}

// Function: sub_13bf7 @ 0x13bf7
void sub_13bf7(long a0,unsigned long a1,char a2) // return-dupe x3
{
  void *v1;
  unsigned long v2;
  long v3;
  
  if (*(long *)(a0 + 0xa0) == *(long *)(a0 + 0x98)) {
    v1 = *(void **)(a0 + 0x90);
    *(unsigned long *)(a0 + 0x90) = sub_26dc6(v1,(long *)(a0 + 0xa0),1,-1,8);
    if (*(char *)(a0 + 0x418)) {
      v2 = *(unsigned long *)(a0 + 0xa0);
      v1 = *(void **)(a0 + 0x118);
      *(unsigned long *)(a0 + 0x118) = sub_26bd0(v1,v2,1);
    }
  }
  if (*(char *)(a0 + 0x418))
    *(char *)(*(long *)(a0 + 0x118) + *(long *)(a0 + 0x98)) = a2;
  v3 = *(long *)(a0 + 0x98);
  *(long *)(a0 + 0x98) = v3 + 1;
  *(unsigned long *)(*(long *)(a0 + 0x90) + v3 * 8) = a1;
  switch(a1) {
    case 0x100:
      *(char *)(a0 + 0xc1) = 1;
      goto label_13d9a;
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
label_13d9a:
  *(long *)(a0 + 0x88) = *(long *)(a0 + 0x88) + 1;
  if (*(long *)(a0 + 0xa8) < *(long *)(a0 + 0x88)) {
    *(unsigned long *)(a0 + 0xa8) = *(unsigned long *)(a0 + 0x88);
    return;
  }
}

// Function: sub_13dee @ 0x13dee
void sub_13dee(long a0,long a1)
{
  bool v1;
  long v2; // stack - 0x10
  
  if ((*(char *)(a0 + 0x418)) && (a1 == 0x113)) {
    v1 = 0;
    for (v2 = 0; v2 < *(long *)(a0 + 0x68); v2 = v2 + 1) {
      sub_13ef3(a0,*(unsigned int *)(v2 * 4 + *(long *)(a0 + 0x60)));
      if (v1)
        sub_13dee(a0,0x106);
      v1 = 1;
    }
    *(unsigned long *)(a0 + 0x68) = 0;
    if (*(long *)(a0 + 0x50) != -1) {
      sub_13dee(a0,*(long *)(a0 + 0x50) + 0x114);
      if (v1)
        sub_13dee(a0,0x106);
    }
  }
  else {
    sub_13bf7(a0,a1,3);
  }
}

// Function: sub_13ef3 @ 0x13ef3
void sub_13ef3(long a0,int a1) // ternary x2
{
  unsigned long v1; // rax
  unsigned long v2; // rdx
  mbstate_t v3; // stack - 0x30
  char v4 [24]; // stack - 0x28
  int v5; // stack - 0x40
  int v6; // stack - 0x3c
  
  v3 = 0;
  v1 = wcrtomb(v4,a1,&v3);
  if (v1 != 0xffffffffffffffff) // branch-flip
    v5 = (int)v1;
  else {
    v5 = 1;
    v4[0] = '\0';
  }
  v2 = (v5 != 1) ? 1 : 3; // branch-flip
  sub_13bf7(a0,(unsigned char)v4[0],v2);
  for (v6 = 1; v6 < v5; v6 = v6 + 1) {
    v2 = (v6 != v5 + -1) ? 0 : 2; // branch-flip
    sub_13bf7(a0,(unsigned char)v4[v6],v2);
    sub_13dee(a0,0x105);
  }
}

// Function: sub_13fe9 @ 0x13fe9
void sub_13fe9(long a0)
{
  unsigned long v1; // stack - 0x38
  unsigned int v2; // stack - 0x40
  int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  
  if (!*(long *)(a0 + 200)) {
    v1 = dat_2b020;
    v4 = dat_2b028;
    v5 = dat_2b030;
    v6 = dat_2b038;
    if (!(*(unsigned long *)(a0 + 0x1c0) & 0x40))
      sub_11882(10,&v1);
    if (*(unsigned long *)(a0 + 0x1c0) & 0x80)
      sub_11882(0,&v1);
    *(long *)(a0 + 200) = sub_11a76(a0,&v1) + 0x114;
    for (v2 = 1; v2 <= 8; v2 = v2 + 1) {
      *(long *)(a0 + 8 + ((long)(int)v2 + 0x18) * 8) = sub_11a76(a0,(unsigned long *)((long)(int)v2 * 0x20 + 0x2b020)) + 0x114;
    }
  }
  sub_13dee(a0,*(long *)(a0 + 200));
  sub_13dee(a0,*(long *)(a0 + 0xd0));
  sub_13dee(a0,0xe0);
  sub_13dee(a0,*(long *)(a0 + 0xe0));
  sub_13dee(a0,0x105);
  sub_13dee(a0,0x106);
  sub_13dee(a0,0xed);
  sub_13dee(a0,*(long *)(a0 + 0xf0));
  sub_13dee(a0,0x105);
  sub_13dee(a0,0x106);
  sub_13dee(a0,*(long *)(a0 + 0xe8));
  sub_13dee(a0,0xf0);
  sub_13dee(a0,*(long *)(a0 + 0xf8));
  sub_13dee(a0,0x105);
  sub_13dee(a0,0x106);
  sub_13dee(a0,0xf4);
  sub_13dee(a0,*(long *)(a0 + 0x108));
  sub_13dee(a0,0x105);
  sub_13dee(a0,0x106);
  sub_13dee(a0,*(long *)(a0 + 0x100));
  for (v3 = 0; v3 <= 2; v3 = v3 + 1) {
    sub_13dee(a0,*(long *)(a0 + 0xd8));
    sub_13dee(a0,0x105);
    sub_13dee(a0,0x106);
  }
}

// Function: sub_1430d @ 0x1430d
void sub_1430d(long a0) // return-dupe x3
{
  int v1; // eax
  unsigned int v2 [34];
  int v3; // stack - 0xa0
  
  if ((((((0 <= *(long *)(a0 + 0x80)) && (*(long *)(a0 + 0x80) <= 0xff)) || (0x113 < *(long *)(a0 + 0x80))) || (((*(long *)(a0 + 0x80) == 0x10b || (*(long *)(a0 + 0x80) == 0x112)) || ((*(long *)(a0 + 0x80) == 0x10c || ((*(long *)(a0 + 0x80) == 0x10d || (*(long *)(a0 + 0x80) == 0x10e)))))))) || (*(long *)(a0 + 0x80) == 0x10f)) || ((((*(long *)(a0 + 0x80) == 0x110 || (*(long *)(a0 + 0x80) == 0x111)) || (*(long *)(a0 + 0x80) == 0x10a)) || (*(long *)(a0 + 0x80) == 0x113)))) {
    if ((*(long *)(a0 + 0x80) != 0x10a) || (!*(char *)(a0 + 0x41a)))
      sub_13dee(a0,*(long *)(a0 + 0x80));
    else {
      sub_13fe9(a0);
    }
    *(unsigned long *)(a0 + 0x80) = sub_127db(a0);
    return;
  }
  if (*(long *)(a0 + 0x80) == 0x109) {
    if (*(int *)(a0 + 0x48) != -1) { // branch-flip
      sub_13ef3(a0,*(unsigned int *)(a0 + 0x48));
      if (*(char *)(a0 + 0x1cd)) {
        v1 = sub_1c01a(*(unsigned int *)(a0 + 0x48),v2);
        for (v3 = 0; v3 < v1; v3 = v3 + 1) {
          sub_13ef3(a0,v2[v3]);
          sub_13dee(a0,0x106);
        }
      }
    }
    else {
      sub_13dee(a0,0x112);
    }
    *(unsigned long *)(a0 + 0x80) = sub_127db(a0);
    return;
  }
  if (*(long *)(a0 + 0x80) == 0x107) {
    *(unsigned long *)(a0 + 0x80) = sub_127db(a0);
    sub_14b1b(a0);
    if (*(long *)(a0 + 0x80) == 0x108) {
      *(unsigned long *)(a0 + 0x80) = sub_127db(a0);
      return;
    }
    sub_513f(gettext("unbalanced (")); // no-return
  }
  sub_13dee(a0,0x100);
}

// Function: sub_146a2 @ 0x146a2
long sub_146a2(long a0,long a1) // early-return, return-dupe
{
  long v1;
  
  v1 = *(long *)(a1 * 8 + -8 + *(long *)(a0 + 0x90));
  if (0x103 < v1) {
    if (2 <= (unsigned long)(v1 - 0x105U))
      return 1;
    v1 = sub_146a2(a0,a1 + -1);
    return sub_146a2(a0,(a1 + -1) - v1) + v1 + 1;
  }
  if (v1 <= 0x100)
    return 1;
  return sub_146a2(a0,a1 + -1) + 1;
}

// Function: sub_14760 @ 0x14760
void sub_14760(long a0,long a1,long a2) // return-dupe
{
  long v1; // stack - 0x18
  long v2; // stack - 0x10
  
  if (*(char *)(a0 + 0x418)) {
    for (v1 = 0; v1 < a2; v1 = v1 + 1) {
      sub_13bf7(a0,*(unsigned long *)((v1 + a1) * 8 + *(long *)(a0 + 0x90)),(int)*(char *)(v1 + a1 + *(long *)(a0 + 0x118)));
    }
    return;
  }
  for (v2 = 0; v2 < a2; v2 = v2 + 1) {
    sub_13bf7(a0,*(unsigned long *)((v2 + a1) * 8 + *(long *)(a0 + 0x90)),3);
  }
}

// Function: sub_14840 @ 0x14840
void sub_14840(long a0)
{
  long v1;
  long v2;
  int v3; // stack - 0x2c
  
  sub_1430d(a0);
  while (((*(long *)(a0 + 0x80) == 0x101 || (*(long *)(a0 + 0x80) == 0x102)) || ((*(long *)(a0 + 0x80) == 0x103 || (*(long *)(a0 + 0x80) == 0x104))))) {
    if ((*(long *)(a0 + 0x80) != 0x104) || ((!*(int *)(a0 + 0x40) && (!*(int *)(a0 + 0x44))))) {
      if (*(long *)(a0 + 0x80) != 0x104) { // branch-flip
        sub_13dee(a0,*(long *)(a0 + 0x80));
        *(unsigned long *)(a0 + 0x80) = sub_127db(a0);
      }
      else {
        v1 = *(long *)(a0 + 0x98);
        v2 = *(long *)(a0 + 0x98);
        *(long *)(a0 + 0x98) = v1 - sub_146a2(a0,v2);
        *(unsigned long *)(a0 + 0x80) = sub_127db(a0);
        sub_14840(a0);
      }
    }
    else {
      v1 = sub_146a2(a0,*(long *)(a0 + 0x98));
      v2 = *(long *)(a0 + 0x98) - v1;
      if (*(int *)(a0 + 0x44) <= -1)
        sub_13dee(a0,0x103);
      if (!*(int *)(a0 + 0x40))
        sub_13dee(a0,0x101);
      for (v3 = 1; v3 < *(int *)(a0 + 0x40); v3 = v3 + 1) {
        sub_14760(a0,v2,v1);
        sub_13dee(a0,0x105);
      }
      for (; v3 < *(int *)(a0 + 0x44); v3 = v3 + 1) {
        sub_14760(a0,v2,v1);
        sub_13dee(a0,0x101);
        sub_13dee(a0,0x105);
      }
      *(unsigned long *)(a0 + 0x80) = sub_127db(a0);
    }
  }
}

// Function: sub_14aa7 @ 0x14aa7
void sub_14aa7(long a0)
{
  sub_14840(a0);
  while (((*(long *)(a0 + 0x80) != 0x108 && (*(long *)(a0 + 0x80) != 0x106)) && (0 <= *(long *)(a0 + 0x80)))) {
    sub_14840(a0);
    sub_13dee(a0,0x105);
  }
}

// Function: sub_14b1b @ 0x14b1b
void sub_14b1b(long a0)
{
  sub_14aa7(a0);
  while (*(long *)(a0 + 0x80) == 0x106) {
    *(unsigned long *)(a0 + 0x80) = sub_127db(a0);
    sub_14aa7(a0);
    sub_13dee(a0,0x106);
  }
}

// Function: sub_14b84 @ 0x14b84
void sub_14b84(unsigned long a0,unsigned long a1,long a2)
{
  *(unsigned long *)(a2 + 0x20) = a0;
  *(unsigned long *)(a2 + 0x28) = a1;
  *(unsigned long *)(a2 + 0x30) = 0xffffffffffffffff;
  *(char *)(a2 + 0x78) = 1;
  if (*(char *)(a2 + 0x1cc) != '\x01')
    sub_513f(gettext("no syntax specified")); // no-return
  if (!*(long *)(a2 + 0xb8))
    sub_13dee(a2,0x10b);
  *(unsigned long *)(a2 + 0x80) = sub_127db(a2);
  *(unsigned long *)(a2 + 0x88) = *(unsigned long *)(a2 + 0xa8);
  sub_14b1b(a2);
  if (*(long *)(a2 + 0x80) != -1)
    sub_513f(gettext("unbalanced )")); // no-return
  sub_13dee(a2,~*(unsigned long *)(a2 + 0xb8));
  sub_13dee(a2,0x105);
  if (*(long *)(a2 + 0xb8))
    sub_13dee(a2,0x106);
  *(long *)(a2 + 0xb8) = *(long *)(a2 + 0xb8) + 1;
}

// Function: sub_14ce2 @ 0x14ce2
void sub_14ce2(struct_28 *a0,unsigned long *a1) // return-dupe
{
  long v1;
  long v2;
  
  if ((long)a1[2] < (long)a0->field_0x8) {
    free((void *)*a1);
    v1 = a0->field_0x8;
    v2 = a1[2];
    *a1 = sub_26dc6(NULL,&a1[2],v1 - v2,-1,0x10);
  }
  a1[1] = a0->field_0x8;
  if (!a0->field_0x8)
    return;
  memcpy((void *)*a1,a0->field_0x0,a0->field_0x8 << 4);
}

// Function: sub_14d9e @ 0x14d9e
void sub_14d9e(unsigned long *a0,unsigned long a1)
{
  *a0 = sub_26c63(a1,0x10);
  a0[2] = a1;
  a0[1] = 0;
}

// Function: sub_14de8 @ 0x14de8
void sub_14de8(long a0,long a1,struct_27 *a2)
{
  long *v1;
  long v10; // stack - 0x30
  void *v2;
  unsigned long v3;
  long v4; // rax
  unsigned long *v5; // rcx
  unsigned long *v6; // rdx
  unsigned int v7; // stack - 0x50
  long v8; // stack - 0x40
  long v9; // stack - 0x38
  
  v10 = a2->field_0x8;
  v8 = 0;
  v9 = v10;
  while( true ) {
    while( true ) {
      if (v9 <= v8) {
        v2 = a2->field_0x0;
        a2->field_0x0 = (void *)sub_11a1e(v2,v10,&a2[1],-1,0x10);
        for (; v8 < v10; v10 = v10 + -1) {
          v6 = (unsigned long *)((long)a2->field_0x0 + v10 * 0x10 + -0x10);
          v5 = (unsigned long *)((long)a2->field_0x0 + v10 * 0x10);
          v3 = v6[1];
          *v5 = *v6;
          v5[1] = v3;
        }
        v1 = (long *)((long)a2->field_0x0 + v8 * 0x10);
        *v1 = a0;
        v1[1] = a1;
        a2->field_0x8 = a2->field_0x8 + 1;
        return;
      }
      v4 = v9 + v8 >> 1;
      if (a0 <= *(long *)(v4 * 0x10 + (long)a2->field_0x0)) break;
      v8 = v4 + 1;
    }
    if (*(long *)(v4 * 0x10 + (long)a2->field_0x0) == a0) break;
    v9 = v4;
  }
  v7 = (unsigned int)a1;
  *(unsigned int *)((long)a2->field_0x0 + v4 * 0x10 + 8) = v7 | *(unsigned int *)((long)a2->field_0x0 + v4 * 0x10 + 8);
}

// Function: sub_14f8b @ 0x14f8b
void sub_14f8b(unsigned long a0,unsigned long a1,struct_27 *a2)
{
  unsigned long *v1;
  long v2;
  void *v3;
  
  v2 = a2->field_0x8;
  v3 = a2->field_0x0;
  a2->field_0x0 = (void *)sub_11a1e(v3,v2,&a2[1],-1,0x10);
  v2 = a2->field_0x8;
  a2->field_0x8 = v2 + 1;
  v1 = (unsigned long *)((long)a2->field_0x0 + v2 * 0x10);
  *v1 = a0;
  v1[1] = a1;
}

// Function: sub_15021 @ 0x15021
void sub_15021(struct_3 *a0,struct_3 *a1,unsigned int a2,long *a3)
{
  unsigned int v1;
  long v2;
  long v3;
  unsigned int v4;
  long v5; // stack - 0x18
  long v6; // stack - 0x10
  
  v5 = 0;
  v6 = 0;
  if (a3[2] - a0->field_0x8 < a1->field_0x8) {
    free((void *)*a3);
    a3[2] = a0->field_0x8;
    v2 = a1->field_0x8;
    *a3 = sub_26dc6(NULL,&a3[2],v2,-1,0x10);
  }
  a3[1] = 0;
  while ((v5 < a0->field_0x8 || (v6 < a1->field_0x8))) {
    if ((a1->field_0x8 <= v6) || ((v5 < a0->field_0x8 && (*(long *)(v5 * 0x10 + a0->field_0x0) <= *(long *)(v6 * 0x10 + a1->field_0x0))))) {
      if (((a0->field_0x8 <= v5) || (a1->field_0x8 <= v6)) || (*(long *)(v5 * 0x10 + a0->field_0x0) != *(long *)(v6 * 0x10 + a1->field_0x0)))
        v4 = 0;
      else {
        v2 = v6 + 1;
        v4 = *(unsigned int *)(v6 * 0x10 + a1->field_0x0 + 8) & a2;
        v6 = v2;
      }
      *(unsigned long *)(*a3 + a3[1] * 0x10) = *(unsigned long *)(a0->field_0x0 + v5 * 0x10);
      v2 = v5 + 1;
      v1 = *(unsigned int *)(v5 * 0x10 + a0->field_0x0 + 8);
      v3 = a3[1];
      a3[1] = v3 + 1;
      *(unsigned int *)(*a3 + v3 * 0x10 + 8) = v1 | v4;
      v5 = v2;
    }
    else {
      if (*(unsigned int *)(v6 * 0x10 + a1->field_0x0 + 8) & a2) {
        *(unsigned long *)(*a3 + a3[1] * 0x10) = *(unsigned long *)(a1->field_0x0 + v6 * 0x10);
        v4 = *(unsigned int *)(v6 * 0x10 + a1->field_0x0 + 8);
        v2 = a3[1];
        a3[1] = v2 + 1;
        *(unsigned int *)(*a3 + v2 * 0x10 + 8) = v4 & a2;
      }
      v6 += 1;
    }
  }
}

// Function: sub_152c9 @ 0x152c9
void sub_152c9(struct_3 *a0,struct_3 *a1,long *a2)
{
  sub_15021(a0,a1,0xffffffff,a2);
}

// Function: sub_15300 @ 0x15300
void sub_15300(struct_27 *a0,struct_3 *a1,struct_28 *a2) // return-dupe
{
  long *v1; // rax
  long v2; // stack - 0x10
  
  if (a1->field_0x8 <= 3) {
    for (v2 = 0; v2 < a1->field_0x8; v2 = v2 + 1) {
      v1 = (long *)(v2 * 0x10 + a1->field_0x0);
      sub_14de8(*v1,v1[1],a0);
    }
    return;
  }
  sub_152c9(a1,a0,a2);
  sub_14ce2(a2,a0);
}

// Function: sub_1539a @ 0x1539a
unsigned int sub_1539a(long a0,struct_3 *a1)
{
  unsigned int v1;
  long v2;
  unsigned long v3;
  unsigned long *v4; // rcx
  unsigned long *v5; // rdx
  long v6; // stack - 0x30
  long v7; // stack - 0x28
  long v8; // stack - 0x20
  
  v2 = a1->field_0x8;
  v6 = 0;
  v7 = v2;
  while( true ) {
    while( true ) {
      if (v7 <= v6)
        return 0;
      v8 = v7 + v6 >> 1;
      if (a0 <= *(long *)(v8 * 0x10 + a1->field_0x0)) break;
      v6 = v8 + 1;
    }
    if (a0 == *(long *)(v8 * 0x10 + a1->field_0x0)) break;
    v7 = v8;
  }
  v1 = *(unsigned int *)(v8 * 0x10 + a1->field_0x0 + 8);
  for (; v8 + 1 < v2; v8 = v8 + 1) {
    v5 = (unsigned long *)(a1->field_0x0 + (v8 + 1) * 0x10);
    v4 = (unsigned long *)(a1->field_0x0 + v8 * 0x10);
    v3 = v5[1];
    *v4 = *v5;
    v4[1] = v3;
  }
  a1->field_0x8 = v8;
  return v1;
}

// Function: sub_154bd @ 0x154bd
void sub_154bd(struct_28 *a0,long a1,struct_3 *a2,unsigned int a3,struct_3 *a4) // return-dupe
{
  a3 = sub_1539a(a1,a0) & a3;
  if (!a3)
    return;
  sub_14ce2(a0,a4);
  sub_15021(a4,a2,a3,a0);
}

// Function: sub_15528 @ 0x15528
long sub_15528(long a0,struct_28 *a1,unsigned int a2)
{
  unsigned int v1;
  long v2;
  void *v3;
  unsigned int v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v5 = 0;
  v4 = 0;
  for (v6 = 0; v6 < (long)a1->field_0x8; v6 = v6 + 1) {
    v5 ^= *(long *)(v6 * 0x10 + (long)a1->field_0x0) + (unsigned long)*(unsigned int *)((long)a1->field_0x0 + v6 * 0x10 + 8);
  }
  v6 = 0;
  do {
    if (*(long *)(a0 + 0x130) <= v6) {
      for (v8 = 0; v8 < (long)a1->field_0x8; v8 = v8 + 1) {
        v1 = *(unsigned int *)(*(long *)(v8 * 0x10 + (long)a1->field_0x0) * 4 + *(long *)(a0 + 0x150));
        if (v1) { // branch-flip
          if (sub_115e0(v1,a2,7))
            v4 |= v1;
        }
        else if (*(long *)(*(long *)(v8 * 0x10 + (long)a1->field_0x0) * 8 + *(long *)(a0 + 0x90)) == 0x112)
          v4 = 0x1ff;
      }
      v2 = *(long *)(a0 + 0x130);
      v3 = *(void **)(a0 + 0x128);
      *(unsigned long *)(a0 + 0x128) = sub_11a1e(v3,v2,(long *)(a0 + 0x138),-1,0x48);
      *(unsigned long *)(*(long *)(a0 + 0x128) + v6 * 0x48) = v5;
      sub_14d9e((unsigned long *)(v6 * 0x48 + *(long *)(a0 + 0x128) + 8),a1->field_0x8);
      sub_14ce2(a1,(unsigned long *)(v6 * 0x48 + *(long *)(a0 + 0x128) + 8));
      *(char *)(v6 * 0x48 + *(long *)(a0 + 0x128) + 0x20) = (char)a2;
      *(short *)(v6 * 0x48 + *(long *)(a0 + 0x128) + 0x22) = (short)v4;
      *(unsigned long *)(v6 * 0x48 + *(long *)(a0 + 0x128) + 0x30) = 0;
      *(unsigned long *)(v6 * 0x48 + *(long *)(a0 + 0x128) + 0x28) = 0;
      *(unsigned long *)(v6 * 0x48 + *(long *)(a0 + 0x128) + 0x40) = 0xffffffffffffffff;
      *(long *)(a0 + 0x130) = *(long *)(a0 + 0x130) + 1;
      return v6;
    }
    if (((v5 == *(unsigned long *)(v6 * 0x48 + *(long *)(a0 + 0x128))) && (a1->field_0x8 == *(long *)(v6 * 0x48 + *(long *)(a0 + 0x128) + 0x10))) && (a2 == *(unsigned char *)(v6 * 0x48 + *(long *)(a0 + 0x128) + 0x20))) {
      for (v7 = 0; ((v7 < (long)a1->field_0x8 && (*(int *)((long)a1->field_0x0 + v7 * 0x10 + 8) == *(int *)(v7 * 0x10 + *(long *)(v6 * 0x48 + *(long *)(a0 + 0x128) + 8) + 8))) && (*(long *)(v7 * 0x10 + (long)a1->field_0x0) == *(long *)(v7 * 0x10 + *(long *)(v6 * 0x48 + *(long *)(a0 + 0x128) + 8)))); v7 = v7 + 1) {
      }
      if (v7 == a1->field_0x8)
        return v6;
    }
    v6 += 1;
  } while( true );
}

// Function: sub_15999 @ 0x15999
void sub_15999(long a0,long a1)
{
  void *v1 [3]; // stack - 0x28
  unsigned int v2; // stack - 0x44
  long v3; // stack - 0x40
  long v4; // stack - 0x38
  long v5; // stack - 0x30
  
  sub_14d9e(v1,*(unsigned long *)(a0 + 0xb0));
  v3 = 0;
  do {
    if (*(long *)(a0 + 0x98) <= v3) {
      free(v1[0]);
      return;
    }
    if (1 <= *(long *)(v3 * 0x18 + *(long *)(a0 + 0x140) + 8)) {
      switch(*(unsigned long *)(v3 * 8 + *(long *)(a0 + 0x90))) {
        case 0x100:
          v2 = 0x1ff;
          break;
        default:
          goto label_15c47;
        case 0x10c:
          v2 = 0x124;
          break;
        case 0x10d:
          v2 = 0x1c0;
          break;
        case 0x10e:
          v2 = 0x28;
          break;
        case 0x10f:
          v2 = 0x82;
          break;
        case 0x110:
          v2 = 0xaa;
          break;
        case 0x111:
          v2 = 0x155;
        
      }
      sub_1539a(v3,*(long *)(a0 + 0x140) + v3 * 0x18);
      for (v4 = 0; v4 < *(long *)(a1 + v3 * 0x18 + 8); v4 = v4 + 1) {
        sub_154bd(*(long *)(a0 + 0x140) + *(long *)(v4 * 0x10 + *(long *)(a1 + v3 * 0x18)) * 0x18,v3,*(long *)(a0 + 0x140) + v3 * 0x18,v2,v1);
      }
      for (v5 = 0; v5 < *(long *)(v3 * 0x18 + *(long *)(a0 + 0x140) + 8); v5 = v5 + 1) {
        sub_154bd(*(long *)(v5 * 0x10 + *(long *)(v3 * 0x18 + *(long *)(a0 + 0x140))) * 0x18 + a1,v3,v3 * 0x18 + a1,0x1ff,v1);
      }
    }
label_15c47:
    v3 += 1;
  } while( true );
}

// Function: sub_15c84 @ 0x15c84
unsigned int sub_15c84(long a0,long a1)
{
  unsigned int v1; // stack - 0x10
  int v2; // stack - 0xc
  
  v1 = 0;
  for (v2 = 0; v2 <= 3; v2 = v2 + 1) {
    if (*(unsigned long *)(a0 + ((long)v2 + 0x7e) * 8) & *(unsigned long *)(a1 + (long)v2 * 8))
      v1 |= 4;
    if (*(unsigned long *)(a0 + ((long)v2 + 0x7a) * 8) & *(unsigned long *)(a1 + (long)v2 * 8))
      v1 |= 2;
    if (~(*(unsigned long *)(a0 + ((long)v2 + 0x7e) * 8) | *(unsigned long *)(a0 + ((long)v2 + 0x7a) * 8)) & *(unsigned long *)(a1 + (long)v2 * 8))
      v1 |= 1;
  }
  return v1;
}

// Function: sub_15d56 @ 0x15d56
unsigned int sub_15d56(long a0,struct_3 *a1)
{
  unsigned int v1; // stack - 0x14
  long v2; // stack - 0x10
  
  v1 = 0;
  for (v2 = 0; v2 < a1->field_0x8; v2 = v2 + 1) {
    v1 |= *(unsigned int *)(*(long *)(v2 * 0x10 + a1->field_0x0) * 4 + *(long *)(a0 + 0x158));
  }
  return v1;
}

// Function: sub_15dbb @ 0x15dbb
void sub_15dbb(long a0,long a1,long a2,struct_28 *a3)
{
  long v1;
  long v10; // stack - 0x28
  unsigned long *v2;
  int v3;
  long v4;
  long v5;
  unsigned long v6;
  unsigned long *v7; // rcx
  long v8; // stack - 0x38
  long v9; // stack - 0x30
  
  v4 = *(long *)(a0 + 0x140);
  v8 = 0;
  v9 = 0;
  do {
    if (*(long *)(v4 + a1 * 0x18 + 8) <= v9) {
      *(long *)(a1 * 0x18 + v4 + 8) = v8;
      return;
    }
    v5 = *(long *)(v9 * 0x10 + *(long *)(v4 + a1 * 0x18));
    v3 = *(int *)(v9 * 0x10 + *(long *)(v4 + a1 * 0x18) + 8);
    if (v3) {
      if (0 <= *(long *)(*(long *)(v9 * 0x10 + *(long *)(v4 + a1 * 0x18)) * 8 + *(long *)(a0 + 0x90))) { // branch-flip
        if ((v5 != a1) && (!(*(unsigned char *)(a2 + v5) & 6))) {
          for (v10 = 0; v10 < v8; v10 = v10 + 1) {
            v1 = *(long *)(v10 * 0x10 + *(long *)(v4 + a1 * 0x18));
            if ((((v1 != a1) && (v3 == *(int *)(v10 * 0x10 + *(long *)(v4 + a1 * 0x18) + 8))) && (!(*(unsigned char *)(a2 + v1) & 6))) && ((*(long *)(v5 * 8 + *(long *)(a0 + 0x90)) == *(long *)(v1 * 8 + *(long *)(a0 + 0x90)) && (!((*(unsigned char *)(a2 + v1) ^ *(unsigned char *)(a2 + v5)) & 1))))) {
              if (*(unsigned char *)(a2 + v5) & 1)
                sub_1539a(v5,(struct_3 *)(v5 * 0x18 + v4));
              sub_15300((struct_27 *)(v1 * 0x18 + v4),(struct_3 *)(v5 * 0x18 + v4),a3);
              break;
            }
          }
          if (v10 < v8) goto label_16184;
        }
        v2 = (unsigned long *)(*(long *)(v4 + a1 * 0x18) + v9 * 0x10);
        v1 = v8 + 1;
        v7 = (unsigned long *)(*(long *)(v4 + a1 * 0x18) + v8 * 0x10);
        v6 = v2[1];
        *v7 = *v2;
        v7[1] = v6;
        *(unsigned char *)(a2 + v5) = *(unsigned char *)(a2 + v5) | 0x10;
        v8 = v1;
      }
      else {
        *(unsigned int *)(a1 * 4 + *(long *)(a0 + 0x150)) = *(unsigned int *)(a1 * 4 + *(long *)(a0 + 0x150)) | *(unsigned int *)(v9 * 0x10 + *(long *)(v4 + a1 * 0x18) + 8);
      }
    }
label_16184:
    v9 += 1;
  } while( true );
}

// Function: sub_161d8 @ 0x161d8
int sub_161d8(long *a0,long *a1)
{
  return (unsigned int)(*a1 < *a0) - (unsigned int)(*a0 < *a1);
}

// Function: sub_1622c @ 0x1622c
void sub_1622c(long a0) // ternary
{
  long v1;
  long v10; // stack - 0x58
  long v11; // stack - 0x50
  long v12; // stack - 0x48
  long v13; // stack - 0x40
  long v14; // stack - 0x38
  unsigned long *v2;
  unsigned long v3;
  unsigned long *v4; // rax
  void *v5; // rax
  void *v6; // rax
  void *v7; // rax
  void *v8; // rax
  unsigned long *v9;
  
  v4 = (unsigned long *)sub_26c63(*(unsigned long *)(a0 + 0x98),8);
  v10 = 1;
  *v4 = 0;
  for (v11 = 1; v11 < *(long *)(a0 + 0x98); v11 = v11 + 1) {
    v4[v11] = 0xffffffffffffffff;
  }
  v5 = (void *)sub_26c63(*(unsigned long *)(a0 + 0xb0),8);
  v6 = (void *)sub_26c63(*(unsigned long *)(a0 + 0xb0),0x18);
  v7 = (void *)sub_26c63(*(unsigned long *)(a0 + 0xb0),4);
  v8 = (*(char *)(a0 + 0x418)) ? (void *)sub_26c63(*(unsigned long *)(a0 + 0xb0),1) : NULL; // branch-flip
  for (v12 = 0; v12 < *(long *)(a0 + 0x98); v12 = v12 + 1) {
    if (0 <= (long)v4[v12]) { // branch-flip
      *(unsigned long *)((long)v5 + v4[v12] * 8) = *(unsigned long *)(v12 * 8 + *(long *)(a0 + 0x90));
      v2 = (unsigned long *)(*(long *)(a0 + 0x140) + v12 * 0x18);
      v9 = (unsigned long *)(v4[v12] * 0x18 + (long)v6);
      v3 = v2[1];
      *v9 = *v2;
      v9[1] = v3;
      v9[2] = v2[2];
      *(unsigned int *)((long)v7 + v4[v12] * 4) = *(unsigned int *)(v12 * 4 + *(long *)(a0 + 0x150));
      if (v8)
        *(char *)((long)v8 + v4[v12]) = *(char *)(v12 + *(long *)(a0 + 0x118));
      for (v13 = 0; v13 < *(long *)(v12 * 0x18 + *(long *)(a0 + 0x140) + 8); v13 = v13 + 1) {
        if (v4[*(long *)(v13 * 0x10 + *(long *)(v12 * 0x18 + *(long *)(a0 + 0x140)))] == -1) {
          v1 = v10 + 1;
          v4[*(long *)(v13 * 0x10 + *(long *)(v12 * 0x18 + *(long *)(a0 + 0x140)))] = v10;
          v10 = v1;
        }
        *(unsigned long *)(*(long *)(v12 * 0x18 + *(long *)(a0 + 0x140)) + v13 * 0x10) = v4[*(long *)(v13 * 0x10 + *(long *)(v12 * 0x18 + *(long *)(a0 + 0x140)))];
      }
      qsort(*(unsigned long *)(v12 * 0x18 + *(long *)(a0 + 0x140)),*(unsigned long *)(v12 * 0x18 + *(long *)(a0 + 0x140) + 8),0x10,sub_161d8);
    }
    else {
      free(*(void **)(v12 * 0x18 + *(long *)(a0 + 0x140)));
      *(unsigned long *)(v12 * 0x18 + *(long *)(a0 + 0x140)) = 0;
      *(unsigned long *)(v12 * 0x18 + *(long *)(a0 + 0x140) + 8) = 0;
    }
  }
  for (v14 = 0; v14 < v10; v14 = v14 + 1) {
    *(unsigned long *)(*(long *)(a0 + 0x90) + v14 * 8) = *(unsigned long *)(v14 * 8 + (long)v5);
    v2 = (unsigned long *)(v14 * 0x18 + (long)v6);
    v9 = (unsigned long *)(*(long *)(a0 + 0x140) + v14 * 0x18);
    v3 = v2[1];
    *v9 = *v2;
    v9[1] = v3;
    v9[2] = v2[2];
    *(unsigned int *)(*(long *)(a0 + 0x150) + v14 * 4) = *(unsigned int *)(v14 * 4 + (long)v7);
    if (v8)
      *(char *)(*(long *)(a0 + 0x118) + v14) = *(char *)(v14 + (long)v8);
  }
  *(long *)(a0 + 0xb0) = v10;
  *(unsigned long *)(a0 + 0x98) = *(unsigned long *)(a0 + 0xb0);
  free(v5);
  free(v6);
  free(v7);
  free(v8);
  free(v4);
}

// Function: sub_1681a @ 0x1681a
void sub_1681a(long a0)
{
  long v1;
  unsigned long v2;
  unsigned char *v3; // rax
  void *v4 [3]; // stack - 0x28
  long v5; // stack - 0x50
  long v6; // stack - 0x48
  long v7; // stack - 0x40
  
  v3 = (unsigned char *)sub_26fa5(*(unsigned long *)(a0 + 0x98));
  for (v5 = 0; v5 < *(long *)(a0 + 0x98); v5 = v5 + 1) {
    for (v6 = 0; v6 < *(long *)(v5 * 0x18 + *(long *)(a0 + 0x140) + 8); v6 = v6 + 1) {
      if (v5 != *(long *)(v6 * 0x10 + *(long *)(v5 * 0x18 + *(long *)(a0 + 0x140)))) { // branch-flip
        if (v5 <= *(long *)(v6 * 0x10 + *(long *)(v5 * 0x18 + *(long *)(a0 + 0x140)))) { // branch-flip
          v1 = *(long *)(v6 * 0x10 + *(long *)(v5 * 0x18 + *(long *)(a0 + 0x140)));
          v3[v1] = v3[*(long *)(v6 * 0x10 + *(long *)(v5 * 0x18 + *(long *)(a0 + 0x140)))] & 8;
          if (v3[v1]) // branch-flip
            v3[*(long *)(v6 * 0x10 + *(long *)(v5 * 0x18 + *(long *)(a0 + 0x140)))] = v3[*(long *)(v6 * 0x10 + *(long *)(v5 * 0x18 + *(long *)(a0 + 0x140)))] | 4;
          else {
            v3[*(long *)(v6 * 0x10 + *(long *)(v5 * 0x18 + *(long *)(a0 + 0x140)))] = v3[*(long *)(v6 * 0x10 + *(long *)(v5 * 0x18 + *(long *)(a0 + 0x140)))] | 8;
          }
        }
        else {
          v3[*(long *)(v6 * 0x10 + *(long *)(v5 * 0x18 + *(long *)(a0 + 0x140)))] = v3[*(long *)(v6 * 0x10 + *(long *)(v5 * 0x18 + *(long *)(a0 + 0x140)))] | 2;
        }
      }
      else {
        v3[*(long *)(v6 * 0x10 + *(long *)(v5 * 0x18 + *(long *)(a0 + 0x140)))] = v3[*(long *)(v6 * 0x10 + *(long *)(v5 * 0x18 + *(long *)(a0 + 0x140)))] | 1;
      }
    }
  }
  *v3 = *v3 | 0x10;
  sub_14d9e(v4,*(unsigned long *)(a0 + 0xb0));
  v2 = *(unsigned long *)(a0 + 0x98);
  *(unsigned long *)(a0 + 0x150) = sub_26ff9(v2,4);
  for (v7 = 0; v7 < *(long *)(a0 + 0x98); v7 = v7 + 1) {
    if (v3[v7] & 0x10)
      sub_15dbb(a0,v7,v3,v4);
  }
  sub_1622c(a0);
  free(v4[0]);
  free(v3);
}

// Function: sub_16c9f @ 0x16c9f
void sub_16c9f(long a0,char a1) // ternary
{
  unsigned int v1;
  void *v10 [3]; // stack - 0x28
  long *v11; // stack - 0x48
  long *v12; // stack - 0xe8
  long *v13; // stack - 0xe0
  char *v14; // stack - 0xd8
  long v15; // stack - 0xd0
  long *v16; // stack - 0xc8
  long *v17; // stack - 0xc0
  long *v18; // stack - 0xb8
  long *v19; // stack - 0xb0
  long v2;
  long v20; // stack - 0xa8
  long v21; // stack - 0xa0
  long v22; // stack - 0x98
  long v23; // stack - 0x90
  unsigned int v24; // stack - 0x4c
  unsigned long v25; // stack - 0x40
  long v3;
  long v4;
  long v5;
  unsigned int v6; // eax
  long *v7; // rax
  char *v8; // rax
  void *v9; // rax
  
  v7 = (long *)sub_26c63(*(unsigned long *)(a0 + 0xb0),0x20);
  v12 = v7;
  v13 = &v7[*(long *)(a0 + 0xb0) * 2];
  v8 = (char *)sub_26c63(*(unsigned long *)(a0 + 0xa8),0x18);
  v14 = v8;
  sub_13dee(a0,0x105);
  v2 = *(unsigned long *)(a0 + 0x98);
  *(char *)(a0 + 0x148) = a1;
  sub_14d9e(v10,*(unsigned long *)(a0 + 0xb0));
  *(unsigned long *)(a0 + 0x140) = sub_26ff9(v2,0x18);
  v9 = (*(char *)(a0 + 0xc1)) ? (void *)sub_26ff9(v2,0x18) : NULL; // branch-flip
  for (v15 = 0; v15 < v2; v15 = v15 + 1) {
    switch(*(unsigned long *)(v15 * 8 + *(long *)(a0 + 0x90))) {
      case 0x100:
        *v14 = 1;
        *(unsigned long *)&v14[0x10] = 0;
        *(unsigned long *)&v14[8] = *(unsigned long *)&v14[0x10];
        v14 = &v14[0x18];
        break;
      case 0x101:
label_17012:
        if (*(long *)(v15 * 8 + *(long *)(a0 + 0x90)) != 0x103)
          v14[-0x18] = 1;
        break;
      case 0x102:
      case 0x103:
        if (*(char *)(a0 + 0xc1)) {
          v11 = &v13[*(long *)&v14[-8] * -2];
          v25 = *(unsigned long *)&v14[-8];
          for (v16 = &v12[*(long *)&v14[-0x10] * -2]; v16 < v12; v16 = &v16[2]) {
            sub_15300((struct_27 *)(*v16 * 0x18 + (long)v9),&v11,v10);
          }
        }
        v11 = &v12[*(long *)&v14[-0x10] * -2];
        v25 = *(unsigned long *)&v14[-0x10];
        for (v17 = &v13[*(long *)&v14[-8] * -2]; v17 < v13; v17 = &v17[2]) {
          sub_15300((struct_27 *)(*(long *)(a0 + 0x140) + *v17 * 0x18),&v11,v10);
        }
        goto label_17012;
      default:
        *v14 = *(long *)(v15 * 8 + *(long *)(a0 + 0x90)) == 0x112;
        *(unsigned long *)&v14[0x10] = 1;
        *(unsigned long *)&v14[8] = *(unsigned long *)&v14[0x10];
        v14 = &v14[0x18];
        *v13 = v15;
        *v12 = *v13;
        *(unsigned int *)&v13[1] = 0x1ff;
        *(unsigned int *)&v12[1] = *(unsigned int *)&v13[1];
        v12 = &v12[2];
        v13 = &v13[2];
        break;
      case 0x105:
        if (v9) {
          v25 = *(unsigned long *)&v14[-0x20];
          v11 = &v13[*(long *)&v14[-0x20] * -2 + *(long *)&v14[-8] * -2];
          for (v18 = &v12[*(long *)&v14[-0x10] * -2]; v18 < v12; v18 = &v18[2]) {
            sub_15300((struct_27 *)(*v18 * 0x18 + (long)v9),&v11,v10);
          }
        }
        v25 = *(unsigned long *)&v14[-0x10];
        v11 = &v12[*(long *)&v14[-0x10] * -2];
        v3 = *(long *)&v14[-8];
        for (v19 = &(&v13[v3 * -2])[*(long *)&v14[-0x20] * -2]; v19 < &v13[v3 * -2]; v19 = &v19[2]) {
          sub_15300((struct_27 *)(*(long *)(a0 + 0x140) + *v19 * 0x18),&v11,v10);
        }
        if (v14[-0x30]) // branch-flip
          *(long *)&v14[-0x28] = *(long *)&v14[-0x10] + *(long *)&v14[-0x28];
        else {
          v12 = &v12[*(long *)&v14[-0x10] * -2];
        }
        if (v14[-0x18]) // branch-flip
          *(long *)&v14[-0x20] = *(long *)&v14[-8] + *(long *)&v14[-0x20];
        else {
          v3 = *(long *)&v14[-8];
          v4 = *(long *)&v14[-0x20];
          for (v20 = 0; v20 < *(long *)&v14[-8]; v20 = v20 + 1) {
            v5 = (&v13[v4 * -2 + v3 * -2 + (v20 + *(long *)&v14[-0x20]) * 2])[1];
            v13[v4 * -2 + v3 * -2 + v20 * 2] = v13[v4 * -2 + v3 * -2 + (v20 + *(long *)&v14[-0x20]) * 2];
            (&v13[v4 * -2 + v3 * -2 + v20 * 2])[1] = v5;
          }
          v13 = &v13[*(long *)&v14[-0x20] * -2];
          *(unsigned long *)&v14[-0x20] = *(unsigned long *)&v14[-8];
        }
        v14[-0x30] = (v14[-0x30] & v14[-0x18]) != 0;
        v14 = &v14[-0x18];
        break;
      case 0x106:
        *(long *)&v14[-0x28] = *(long *)&v14[-0x10] + *(long *)&v14[-0x28];
        *(long *)&v14[-0x20] = *(long *)&v14[-8] + *(long *)&v14[-0x20];
        v14[-0x30] = v14[-0x18] || v14[-0x30];
        v14 = &v14[-0x18];
      
    }
  }
  if (v9) {
    sub_15999(a0,v9);
    for (v21 = 0; v21 < v2; v21 = v21 + 1) {
      free(*(void **)((long)v9 + v21 * 0x18));
    }
    free(v9);
  }
  sub_1681a(a0);
  sub_14d9e(&v11,1);
  sub_14f8b(0,CONCAT44(v24,0x1ff),&v11);
  *(unsigned long *)(a0 + 0x158) = sub_26ff9(v2,4);
  for (v22 = 0; v22 < v2; v22 = v22 + 1) {
    v1 = *(unsigned int *)(v22 * 4 + *(long *)(a0 + 0x150));
    if (sub_1165b(v1))
      *(unsigned int *)(v22 * 4 + *(long *)(a0 + 0x158)) = *(unsigned int *)(v22 * 4 + *(long *)(a0 + 0x158)) | 4;
    v1 = *(unsigned int *)(v22 * 4 + *(long *)(a0 + 0x150));
    if (sub_11679(v1))
      *(unsigned int *)(v22 * 4 + *(long *)(a0 + 0x158)) = *(unsigned int *)(v22 * 4 + *(long *)(a0 + 0x158)) | 2;
    for (v23 = 0; v23 < *(long *)(v22 * 0x18 + *(long *)(a0 + 0x140) + 8); v23 = v23 + 1) {
      v1 = *(unsigned int *)(v23 * 0x10 + *(long *)(v22 * 0x18 + *(long *)(a0 + 0x140)) + 8);
      if (sub_1165b(v1))
        *(unsigned int *)(v22 * 4 + *(long *)(a0 + 0x158)) = *(unsigned int *)(v22 * 4 + *(long *)(a0 + 0x158)) | 4;
      v1 = *(unsigned int *)(v23 * 0x10 + *(long *)(v22 * 0x18 + *(long *)(a0 + 0x140)) + 8);
      if (sub_11679(v1))
        *(unsigned int *)(v22 * 4 + *(long *)(a0 + 0x158)) = *(unsigned int *)(v22 * 4 + *(long *)(a0 + 0x158)) | 2;
    }
  }
  v6 = sub_15d56(a0,&v11);
  if (v6 & 4)
    sub_15528(a0,&v11,4);
  *(unsigned int *)(a0 + 0x16c) = sub_15528(a0,&v11,v6 ^ 7);
  *(long *)(a0 + 400) = (long)*(int *)(a0 + 0x16c);
  if (v6 & 2)
    *(unsigned int *)(a0 + 0x16c) = sub_15528(a0,&v11,2);
  *(int *)(a0 + 0x16c) = *(int *)(a0 + 0x16c) + 1;
  *(unsigned int *)(a0 + 0x168) = 0;
  free(v7);
  free(v8);
  free(v10[0]);
  free(v11);
}

// Function: sub_179b7 @ 0x179b7
void sub_179b7(long a0) // return-dupe, ternary x3
{
  void *v1;
  unsigned long *v2; // rax
  long v3; // stack - 0x30
  long v4; // stack - 0x28
  unsigned long *v5; // stack - 0x20
  unsigned long v6; // stack - 0x18
  
  v4 = *(long *)(a0 + 0x160);
  if (*(long *)(a0 + 0x130) <= v4)
    return;
  v5 = (*(long *)(a0 + 0x170)) ? (void *)(*(long *)(a0 + 0x170) + -0x10) : NULL; // branch-flip
  v3 = (v5) ? *(long *)(a0 + 0x160) + 2 : 0; // branch-flip
  v5 = (unsigned long *)sub_26dc6(v5,&v3,*(long *)(a0 + 0x130) - v4,-1,8);
  v5[1] = 0;
  *v5 = v5[1];
  *(unsigned long **)(a0 + 0x170) = &v5[2];
  *(long *)(a0 + 0x160) = v3 + -2;
  v6 = *(unsigned long *)(a0 + 0x160);
  v1 = *(void **)(a0 + 0x178);
  *(unsigned long *)(a0 + 0x178) = sub_26bd0(v1,v6,8);
  v1 = *(void **)(a0 + 0x180);
  *(unsigned long *)(a0 + 0x180) = sub_26bd0(v1,v6,1);
  v1 = *(void **)(a0 + 0x188);
  *(unsigned long *)(a0 + 0x188) = sub_26bd0(v1,v6,8);
  if (*(char *)(a0 + 0x418)) {
    v5 = (*(long *)(a0 + 0x1b0)) ? (void *)(*(long *)(a0 + 0x1b0) + -0x10) : NULL; // branch-flip
    v2 = (unsigned long *)sub_26bd0(v5,v3,8);
    if (!v4) {
      v2[1] = 0;
      *v2 = v2[1];
    }
    *(unsigned long **)(a0 + 0x1b0) = &v2[2];
  }
  for (; v4 < (long)v6; v4 = v4 + 1) {
    *(unsigned long *)(v4 * 8 + *(long *)(a0 + 0x170)) = 0;
    *(unsigned long *)(v4 * 8 + *(long *)(a0 + 0x178)) = 0;
    if (*(char *)(a0 + 0x418))
      *(unsigned long *)(v4 * 8 + *(long *)(a0 + 0x1b0)) = 0;
  }
}

// Function: sub_17c61 @ 0x17c61
unsigned long sub_17c61(long a0,long *a1,unsigned char a2) // ternary x4
{
  unsigned long *v1;
  void *v10; // stack - 0x98
  unsigned long v11 [4];
  unsigned long v12 [5]; // stack - 0x38
  void *v13; // stack - 0x78
  unsigned long v14; // rdi
  bool v15;
  bool v16; // stack - 0x12d
  int v17; // stack - 0x12c
  int v18; // stack - 0x128
  int v19; // stack - 0x124
  char v2;
  int v20; // stack - 0x120
  int v21; // stack - 0x11c
  int v22; // stack - 0x118
  int v23; // stack - 0x114
  long v24; // stack - 0x108
  long v25; // stack - 0x100
  long v26; // stack - 0xf8
  long v27; // stack - 0xf0
  long v28; // stack - 0xe8
  long v29; // stack - 0xe0
  char v3;
  long v30; // stack - 0xd8
  long v31; // stack - 0xd0
  long v32; // stack - 0xc8
  long v33; // stack - 0xb0
  long v34; // stack - 0x90
  unsigned long v35; // stack - 0x70
  unsigned int v4; // eax
  unsigned int v5; // eax
  long v6;
  long *v7; // rax
  unsigned long *v8; // rax
  void *v9; // stack - 0xb8
  
  v6 = (sub_11696(a0,a1)) ? a1[0x2f] : a1[0x2e]; // branch-flip
  v7 = (long *)(v6 + a0 * 8);
  v27 = *v7;
  if (!v27) {
    if (0x400 <= (int)a1[0x2d]) {
      for (v28 = (long)*(int *)((long)a1 + 0x16c); v28 < a1[0x2c]; v28 = v28 + 1) {
        free(*(void **)(v28 * 8 + a1[0x2e]));
        free(*(void **)(v28 * 8 + a1[0x2f]));
        v8 = (unsigned long *)(v28 * 8 + a1[0x2f]);
        *v8 = 0;
        *(unsigned long *)(v28 * 8 + a1[0x2e]) = *v8;
      }
      *(unsigned int *)&a1[0x2d] = 0;
    }
    *(int *)&a1[0x2d] = (int)a1[0x2d] + 1;
    v27 = sub_26aea(0x800);
    *v7 = v27;
    for (v17 = 0; v17 <= 0xff; v17 = v17 + 1) {
      *(unsigned long *)(v27 + (long)v17 * 8) = 0xfffffffffffffffe;
    }
  }
  *(char *)(a0 + a1[0x30]) = 0;
  v2 = *(char *)(a0 * 0x48 + a1[0x25] + 0x20);
  if (sub_116d2((unsigned char)v2,4,a0,a1))
    *(unsigned char *)(a0 + a1[0x30]) = *(unsigned char *)(a0 + a1[0x30]) | 4;
  v2 = *(char *)(a0 * 0x48 + a1[0x25] + 0x20);
  if (sub_116d2((unsigned char)v2,2,a0,a1))
    *(unsigned char *)(a0 + a1[0x30]) = *(unsigned char *)(a0 + a1[0x30]) | 2;
  v2 = *(char *)(a0 * 0x48 + a1[0x25] + 0x20);
  if (sub_116d2((unsigned char)v2,1,a0,a1))
    *(unsigned char *)(a0 + a1[0x30]) = *(unsigned char *)(a0 + a1[0x30]) | 1;
  sub_14d9e(&v9,a1[0x16]);
  for (v29 = 0; v29 < *(long *)(a0 * 0x48 + a1[0x25] + 0x10); v29 = v29 + 1) {
    for (v30 = 0; v30 < *(long *)(*(long *)(v29 * 0x10 + *(long *)(a0 * 0x48 + a1[0x25] + 8)) * 0x18 + a1[0x28] + 8); v30 = v30 + 1) {
      v7 = (long *)(v30 * 0x10 + *(long *)(*(long *)(v29 * 0x10 + *(long *)(a0 * 0x48 + a1[0x25] + 8)) * 0x18 + a1[0x28]));
      sub_14de8(*v7,v7[1],&v9);
    }
  }
  sub_14d9e(&v10,a1[0x16]);
  sub_118fc(v11);
  v31 = 0;
  do {
    if (v33 <= v31) {
      sub_14d9e(&v13,a1[0x16]);
      if (1 <= v34) { // branch-flip
        if ((char)a1[0x29]) {
          v16 = (char)a1[0x83] == '\0';
          if (!v16) {
            v16 = 1;
            for (v32 = 0; (v16 && (v32 < v34)); v32 = v32 + 1) {
              v16 = (*(unsigned char *)(*(long *)(v32 * 0x10 + (long)v10) + a1[0x23]) & v16) != 0;
            }
          }
          if (v16)
            sub_15300(&v10,(struct_3 *)(a1[0x25] + 8),&v13);
        }
        v4 = sub_15c84(a1,v11);
        v5 = sub_15d56(a1,&v10);
        v24 = (~v5 & v4) ? sub_15528(a1,&v10,v5 ^ 7) : -1; // branch-flip
        v25 = (v5 & v4 & 4) ? sub_15528(a1,&v10,4) : v24; // branch-flip
        v26 = (v5 & v4 & 2) ? sub_15528(a1,&v10,2) : v24; // branch-flip
        sub_179b7(a1);
      }
      else if ((char)a1[0x29]) { // branch-flip
        v25 = 0;
        v26 = (long)(*(int *)((long)a1 + 0x16c) + -1);
        v24 = a1[0x32];
      }
      else {
        v25 = -1;
        v26 = -1;
        v24 = -1;
      }
      for (v23 = 0; v23 <= 0xff; v23 = v23 + 1) {
        if (sub_117fa(v23,v11)) {
          v3 = *(char *)((long)a1 + (long)v23 + 0x1cf);
          if (v3 != '\x02') { // branch-flip
            if (v3 != '\x04') // branch-flip
              *(long *)((long)v23 * 8 + v27) = v24;
            else {
              *(long *)((long)v23 * 8 + v27) = v25;
            }
          }
          else {
            *(long *)((long)v23 * 8 + v27) = v26;
          }
        }
      }
      free(v10);
      free(v9);
      free(v13);
      v2 = *(char *)((long)a1 + 0x1ce);
      if (sub_117fa((unsigned char)v2,v11)) {
        *(unsigned long *)(a1[0x31] + a0 * 8) = *(unsigned long *)((unsigned long)*(unsigned char *)((long)a1 + 0x1ce) * 8 + v27);
        *(unsigned long *)(v27 + (unsigned long)*(unsigned char *)((long)a1 + 0x1ce) * 8) = 0xffffffffffffffff;
      }
      return *(unsigned long *)(v27 + (unsigned long)a2 * 8);
    }
    v7 = (long *)(v31 * 0x10 + (long)v9);
    v35 = v7[1];
    v13 = (void *)*v7;
    v15 = 0;
    if ((0 <= *(long *)((long)v13 * 8 + a1[0x12])) && (*(long *)((long)v13 * 8 + a1[0x12]) <= 0xff)) {
      sub_118d3(v12);
      sub_11834(*(unsigned long *)((long)v13 * 8 + a1[0x12]) & 0xffffffff,v12);
      if (*(unsigned long *)((long)v13 * 8 + a1[0x12]) == (unsigned long)a2)
        v15 = 1;
label_184ca:
      if ((int)v35 != 0x1ff) {
        v2 = *(char *)(a0 * 0x48 + a1[0x25] + 0x20);
        v14 = v35 & 0xffffffff;
        if (sub_115e0(v14,(unsigned char)v2,4) != '\x01') {
          for (v18 = 0; v18 <= 3; v18 = v18 + 1) {
            v12[v18] = v12[v18] & ~a1[(long)v18 + 0x7e];
          }
        }
        v2 = *(char *)(a0 * 0x48 + a1[0x25] + 0x20);
        v14 = v35 & 0xffffffff;
        if (sub_115e0(v14,(unsigned char)v2,2) != '\x01') {
          for (v19 = 0; v19 <= 3; v19 = v19 + 1) {
            v12[v19] = v12[v19] & ~a1[(long)v19 + 0x7a];
          }
        }
        v2 = *(char *)(a0 * 0x48 + a1[0x25] + 0x20);
        v14 = v35 & 0xffffffff;
        if (sub_115e0(v14,(unsigned char)v2,1) != '\x01') {
          for (v20 = 0; v20 <= 3; v20 = v20 + 1) {
            v12[v20] = v12[v20] & (a1[(long)v20 + 0x7e] | a1[(long)v20 + 0x7a]);
          }
        }
        if (sub_119db(v12)) goto label_187bb;
        if (sub_117fa(a2,v12) != '\x01')
          v15 = 0;
      }
      if (v15) {
        for (v21 = 0; v21 <= 3; v21 = v21 + 1) {
          v11[v21] = v11[v21] & v12[v21];
        }
        sub_14f8b(v13,v35,&v10);
      }
      else {
        for (v22 = 0; v22 <= 3; v22 = v22 + 1) {
          v11[v22] = v11[v22] & ~v12[v22];
        }
      }
    }
    else {
      if (0x114 <= *(long *)((long)v13 * 8 + a1[0x12])) {
        v1 = (unsigned long *)(*a1 + *(long *)((long)v13 * 8 + a1[0x12]) * 0x20 + -0x2280);
        v12[0] = *v1;
        v12[1] = v1[1];
        v12[2] = v1[2];
        v12[3] = v1[3];
        if (sub_117fa(a2,v12))
          v15 = 1;
        goto label_184ca;
      }
      if (*(long *)((long)v13 * 8 + a1[0x12]) == 0x10a) {
        v1 = (unsigned long *)(*a1 + a1[3] * 0x20);
        v12[0] = *v1;
        v12[1] = v1[1];
        v12[2] = v1[2];
        v12[3] = v1[3];
        v15 = sub_117fa(a2,v12) != '\0';
        v2 = *(char *)(a0 * 0x48 + a1[0x25] + 0x20);
        v14 = v35 & 0xffffffff;
        if (sub_115e0(v14,(unsigned char)v2,1)) {
          if (!*(long *)(a0 * 0x48 + a1[0x25] + 0x30))
            sub_14d9e((unsigned long *)(a0 * 0x48 + a1[0x25] + 0x28),1);
          sub_14de8(v13,v35,(struct_27 *)(a0 * 0x48 + a1[0x25] + 0x28));
        }
        goto label_184ca;
      }
    }
label_187bb:
    v31 += 1;
  } while( true );
}

// Function: sub_18c54 @ 0x18c54
unsigned long sub_18c54(long *a0,long a1,long *a2)
{
  unsigned char *v1;
  long v2; // stack - 0x10
  
  if (*(long *)(a1 * 8 + a0[0x2e])) // branch-flip
    v2 = *(long *)(a1 * 8 + a0[0x2e]);
  else if (*(long *)(a1 * 8 + a0[0x2f])) // branch-flip
    v2 = *(long *)(a1 * 8 + a0[0x2f]);
  else {
    sub_17c61(a1,a0,(unsigned char)*(char *)*a2);
    if (*(long *)(a1 * 8 + a0[0x2e])) // branch-flip
      v2 = *(long *)(a1 * 8 + a0[0x2e]);
    else {
      v2 = *(long *)(a1 * 8 + a0[0x2f]);
      if (!v2)
        __assert_fail("t","dfa.c",0xcce,"transit_state_singlebyte"); // no-return
    }
  }
  if (*(long *)(v2 + (unsigned long)*(unsigned char *)*a2 * 8) == -2)
    sub_17c61(a1,a0,(unsigned char)*(char *)*a2);
  v1 = (unsigned char *)*a2;
  *a2 = (long)&v1[1];
  return *(unsigned long *)(v2 + (unsigned long)*v1 * 8);
}

// Function: sub_18e06 @ 0x18e06
long sub_18e06(long a0,long a1,long *a2,long a3)
{
  long v1;
  long v10; // stack - 0x40
  long v11; // stack - 0x38
  long v12; // stack - 0x30
  long v13; // stack - 0x28
  char *v2;
  unsigned int v3; // eax
  int v4; // stack - 0x54
  long v5; // stack - 0x68
  int v6; // stack - 0x50
  int v7; // stack - 0x4c
  int v8; // stack - 0x48
  unsigned int v9; // stack - 0x44
  
  v1 = *a2;
  v2 = (char *)*a2;
  v8 = sub_11724(&v4,v2,a3 - v1,a0);
  *(unsigned long *)(a0 + 0x1a0) = 0;
  v6 = 0;
  v12 = a1;
  for (v5 = a1; (v6 < v8 && ((!v6 || (*(int *)(a0 + 0x16c) <= v5)))); v5 = sub_18c54(a0,v5,a2)) {
    v6 += 1;
  }
  *a2 = *a2 + (long)(v8 - v6);
  if (v4 != -1) {
    if (*(long *)(v12 * 0x48 + *(long *)(a0 + 0x128) + 0x40) <= -1) {
      if (0x400 <= *(long *)(a0 + 0x1b8)) {
        for (v10 = -1; v10 < *(long *)(a0 + 0x160); v10 = v10 + 1) {
          free(*(void **)(v10 * 8 + *(long *)(a0 + 0x1b0)));
          *(unsigned long *)(v10 * 8 + *(long *)(a0 + 0x1b0)) = 0;
        }
        for (v11 = 0; v11 < *(long *)(a0 + 0x130); v11 = v11 + 1) {
          *(unsigned long *)(v11 * 0x48 + *(long *)(a0 + 0x128) + 0x40) = 0xffffffffffffffff;
        }
        *(unsigned long *)(a0 + 0x1b8) = 0;
      }
      v1 = *(long *)(a0 + 0x1b8);
      *(long *)(a0 + 0x1b8) = v1 + 1;
      *(long *)(v12 * 0x48 + *(long *)(a0 + 0x128) + 0x40) = v1;
    }
    if (*(long *)(v5 * 8 + *(long *)(a0 + 0x1b0))) { // branch-flip
      if (0 <= *(long *)(*(long *)(v12 * 0x48 + *(long *)(a0 + 0x128) + 0x40) * 8 + *(long *)(v5 * 8 + *(long *)(a0 + 0x1b0))))
        return *(long *)(*(long *)(v12 * 0x48 + *(long *)(a0 + 0x128) + 0x40) * 8 + *(long *)(v5 * 8 + *(long *)(a0 + 0x1b0)));
    }
    else {
      v1 = *(long *)(a0 + 0x1b0);
      *(unsigned long *)(v1 + v5 * 8) = sub_26aea(0x2000);
      for (v7 = 0; v7 <= 0x3ff; v7 = v7 + 1) {
        *(unsigned long *)((long)v7 * 8 + *(long *)(v5 * 8 + *(long *)(a0 + 0x1b0))) = 0xffffffffffffffff;
      }
    }
    if (v5 != -1) // branch-flip
      sub_152c9((struct_3 *)(v12 * 0x48 + *(long *)(a0 + 0x128) + 0x28),(struct_3 *)(v5 * 0x48 + *(long *)(a0 + 0x128) + 8),(long *)(a0 + 0x198));
    else {
      sub_14ce2((struct_28 *)(v12 * 0x48 + *(long *)(a0 + 0x128) + 0x28),(unsigned long *)(a0 + 0x198));
    }
    v3 = sub_15d56(a0,(struct_3 *)(a0 + 0x198));
    v9 = v3;
    v13 = sub_15528(a0,(struct_28 *)(a0 + 0x198),v3 ^ 7);
    sub_179b7(a0);
    *(long *)(*(long *)(v5 * 8 + *(long *)(a0 + 0x1b0)) + *(long *)(v12 * 0x48 + *(long *)(a0 + 0x128) + 0x40) * 8) = v13;
    v5 = v13;
  }
  return v5;
}

// Function: sub_1929e @ 0x1929e
unsigned char * sub_1929e(long a0,unsigned char *a1,unsigned char *a2,long a3)
{
  int v1; // eax
  unsigned char *v2; // rax
  char v3 [4];
  unsigned char *v4; // stack - 0x30
  
  v1 = (unsigned int)*a1;
  v2 = (long)(long)v1;
  v4 = a2;
  if (*(char *)(a0 + 0x2cf + (long)v2)) // branch-flip
    v2 = a1;
  else {
    for (; v4 < a1; v4 = &v4[(long)v2]) {
      v2 = (unsigned long)(a3 - (long)v4);
      v1 = sub_11724(v3,v4,v2,a0);
      v2 = (long)(long)v1;
    }
    v2 = v4;
  }
  return v2;
}

// Function: sub_19334 @ 0x19334
unsigned char * sub_19334(long a0,unsigned char *a1,unsigned char *a2,bool a3,long *a4,char a5)
{
  unsigned char *v1;
  long v10; // stack - 0x68
  long v11; // stack - 0x60
  long v12; // stack - 0x58
  long v13; // stack - 0x50
  long v14; // stack - 0x48
  long v15; // stack - 0x40
  unsigned char *v16; // stack - 0x38
  long v17; // stack - 0x30
  long v18; // stack - 0x28
  long v19; // stack - 0x20
  unsigned char v2;
  long v20; // stack - 0x18
  unsigned char v3;
  unsigned char v4;
  long v5;
  long v6;
  char v7; // al
  unsigned long *v8; // rax
  unsigned char *v9; // stack - 0x70
  
  if (0x400 <= *(long *)(a0 + 0x130)) {
    for (v10 = (long)*(int *)(a0 + 0x16c); v10 < *(long *)(a0 + 0x130); v10 = v10 + 1) {
      free(*(void **)(v10 * 0x48 + *(long *)(a0 + 0x128) + 8));
      free(*(void **)(v10 * 0x48 + *(long *)(a0 + 0x128) + 0x28));
    }
    *(long *)(a0 + 0x130) = (long)*(int *)(a0 + 0x16c);
    if (*(long *)(a0 + 0x170)) {
      for (v11 = 0; v11 < *(long *)(a0 + 0x160); v11 = v11 + 1) {
        free(*(void **)(v11 * 8 + *(long *)(a0 + 0x170)));
        free(*(void **)(v11 * 8 + *(long *)(a0 + 0x178)));
        v8 = (unsigned long *)(v11 * 8 + *(long *)(a0 + 0x178));
        *v8 = 0;
        *(unsigned long *)(v11 * 8 + *(long *)(a0 + 0x170)) = *v8;
      }
      *(unsigned int *)(a0 + 0x168) = 0;
    }
    if ((*(char *)(a0 + 0x418)) && (*(long *)(a0 + 0x1b0))) {
      for (v12 = -1; v12 < *(long *)(a0 + 0x160); v12 = v12 + 1) {
        free(*(void **)(v12 * 8 + *(long *)(a0 + 0x1b0)));
        *(unsigned long *)(v12 * 8 + *(long *)(a0 + 0x1b0)) = 0;
      }
      for (v13 = 0; v13 < *(int *)(a0 + 0x16c); v13 = v13 + 1) {
        *(unsigned long *)(v13 * 0x48 + *(long *)(a0 + 0x128) + 0x40) = 0xffffffffffffffff;
      }
      *(unsigned long *)(a0 + 0x1b8) = 0;
    }
  }
  if (!*(long *)(a0 + 0x160))
    sub_179b7(a0);
  v14 = 0;
  v15 = 0;
  v17 = *(long *)(a0 + 0x170);
  v2 = *(unsigned char *)(a0 + 0x1ce);
  v3 = *a2;
  *a2 = v2;
  v9 = a1;
  v16 = a1;
  if ((a5) && (memset((void *)(a0 + 0x110),0,8), !*(long *)(a0 + 0x1a8)))
    sub_14d9e((unsigned long *)(a0 + 0x198),*(unsigned long *)(a0 + 0xb0));
  v18 = 0;
label_198ee:
  do {
    while (v19 = *(long *)(v17 + v14 * 8), v19) {
      if (v14 < *(int *)(a0 + 0x16c)) {
        if ((a5 != '\x01') || (!*(long *)(v14 * 0x48 + *(long *)(a0 + 0x128) + 0x30))) {
          while (v14 == *(long *)(v19 + (unsigned long)*v9 * 8)) {
            v9 = &v9[1];
          }
        }
        if (a5) {
          v16 = (unsigned char *)sub_1929e(a0,v9,v16,a2);
          v9 = v16;
        }
      }
      if (a5) { // branch-flip
        v15 = v14;
        if (((*(long *)(v14 * 0x48 + *(long *)(a0 + 0x128) + 0x30)) && (*(int *)(a0 + 0xc + ((long)(int)(unsigned int)*v9 + 0x144) * 4) == -1)) && (v9 < a2)) {
          v14 = sub_18e06(a0,v14,&v9,a2);
          v16 = v9;
          v17 = *(long *)(a0 + 0x170);
        }
        else {
          v1 = &v9[1];
          v14 = *(long *)(v19 + (unsigned long)*v9 * 8);
          v9 = v1;
        }
      }
      else {
        v1 = &v9[1];
        v5 = *(long *)(v19 + (unsigned long)*v9 * 8);
        v6 = *(long *)(v17 + v5 * 8);
        v9 = v1;
        if (!v6) {
          v20 = v14;
          v15 = v14;
          v19 = 0;
          v14 = v5;
          break;
        }
        if (v14 < *(int *)(a0 + 0x16c)) {
          while (v5 == *(long *)(v6 + (unsigned long)*v9 * 8)) {
            v9 = &v9[1];
          }
        }
        v1 = &v9[1];
        v14 = *(long *)(v6 + (unsigned long)*v9 * 8);
        v9 = v1;
        v15 = v5;
      }
    }
    if (0 <= v14) {
      if (*(long *)(v14 * 8 + *(long *)(a0 + 0x178))) { // branch-flip
        if ((*(unsigned char *)(a0 + 0x1cf + (long)(int)(unsigned int)*v9) & *(unsigned char *)(v14 + *(long *)(a0 + 0x180))) || ((a2 == v9 && (v7 = sub_116d2((unsigned char)*(char *)(v14 * 0x48 + *(long *)(a0 + 0x128) + 0x20),4,v14,a0), v7)))) goto label_19c3b;
        if ((a5) && (v14 < *(int *)(a0 + 0x16c))) {
          v16 = (unsigned char *)sub_1929e(a0,v9,v16,a2);
          v9 = v16;
        }
        v15 = v14;
        if ((((a5 != '\x01') || (!*(long *)(v14 * 0x48 + *(long *)(a0 + 0x128) + 0x30))) || (*(int *)(a0 + 0xc + ((long)(int)(unsigned int)*v9 + 0x144) * 4) != -1)) || (a2 <= v9)) {
          v1 = &v9[1];
          v14 = *(long *)((unsigned long)*v9 * 8 + *(long *)(v14 * 8 + *(long *)(a0 + 0x178)));
          v9 = v1;
        }
        else {
          v14 = sub_18e06(a0,v14,&v9,a2);
          v16 = v9;
          v17 = *(long *)(a0 + 0x170);
        }
      }
      else {
        sub_17c61(v14,a0,*v9);
        v17 = *(long *)(a0 + 0x170);
      }
      goto label_198ee;
    }
    if (v14 != -2) { // branch-flip
      if (((a2 < v9) || (v2 != v9[-1])) || (*(long *)(v15 * 8 + *(long *)(a0 + 0x188)) < 0)) {
        v9 = NULL;
label_19c3b:
        if (a4)
          *a4 = *a4 + v18;
        *a2 = v3;
        return v9;
      }
      v18 += 1;
      v16 = v9;
      if (a3) // branch-flip
        v14 = *(long *)(v15 * 8 + *(long *)(a0 + 0x188));
      else if (*(char *)(a0 + 0x1cf + (long)(int)(unsigned int)v2) != '\x04') { // branch-flip
        if (*(char *)(a0 + 0x1cf + (long)(int)(unsigned int)v2) != '\x02') // branch-flip
          v14 = *(long *)(a0 + 400);
        else {
          v14 = (long)(*(int *)(a0 + 0x16c) + -1);
        }
      }
      else {
        v14 = 0;
      }
    }
    else {
      v4 = v9[-1];
      v14 = sub_17c61(v15,a0,v4);
      v17 = *(long *)(a0 + 0x170);
    }
  } while( true );
}

// Function: sub_19c87 @ 0x19c87
void sub_19c87(long a0,unsigned char *a1,unsigned char *a2,char a3,long *a4,unsigned long a5)
{
  sub_19334(a0,a1,a2,(unsigned char)a3,a4,1,a5);
}

// Function: sub_19cd3 @ 0x19cd3
void sub_19cd3(long a0,unsigned char *a1,unsigned char *a2,char a3,long *a4,unsigned long a5)
{
  sub_19334(a0,a1,a2,(unsigned char)a3,a4,0,a5);
}

// Function: sub_19d1f @ 0x19d1f
unsigned long sub_19d1f(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,char *a5)
{
  *a5 = 1;
  return a1;
}

// Function: sub_19d4d @ 0x19d4d
void sub_19d4d(long a0,char *a1,char *a2,char a3,unsigned long a4,unsigned long a5)
{
  (**(void **)(a0 + 0x410))(a0,a1,a2,(unsigned char)a3,a4,a5);
}

// Function: sub_19da3 @ 0x19da3
unsigned long sub_19da3(long a0)
{
  return *(unsigned long *)(a0 + 0x120);
}

// Function: sub_19dbc @ 0x19dbc
char sub_19dbc(long a0)
{
  return *(char *)(a0 + 0xc0);
}

// Function: sub_19dd5 @ 0x19dd5
void sub_19dd5(long a0) // return-dupe
{
  long v1; // stack - 0x10
  
  free(*(void **)(a0 + 0x118));
  free(*(void **)(a0 + 0x60));
  free(*(void **)(a0 + 0x198));
  if (!*(long *)(a0 + 0x1b0))
    return;
  for (v1 = -1; v1 < *(long *)(a0 + 0x160); v1 = v1 + 1) {
    free(*(void **)(v1 * 8 + *(long *)(a0 + 0x1b0)));
  }
  free((void *)(*(long *)(a0 + 0x1b0) + -0x10));
}

// Function: sub_19e86 @ 0x19e86
unsigned long sub_19e86(long a0) // return-dupe
{
  long v1;
  long v2; // stack - 0x10
  
  v2 = 0;
  do {
    if (*(long *)(a0 + 0x98) <= v2)
      return 1;
    v1 = *(long *)(v2 * 8 + *(long *)(a0 + 0x90));
    if (0x112 <= v1) { // branch-flip
      if ((unsigned long)(v1 - 0x112U) <= 1)
        return 0;
    }
    else if ((0x10e <= v1) && (*(char *)(a0 + 0x418) == '\x01'))
      return 0;
    v2 += 1;
  } while( true );
}

// Function: sub_19f0c @ 0x19f0c
void sub_19f0c(long a0) // return-dupe x2
{
  long v1;
  bool v2;
  long v3; // stack - 0x10
  
  if (*(char *)(a0 + 0x41a) != '\x01')
    return;
  v2 = 0;
  v3 = 0;
  while( true ) {
    if (*(long *)(a0 + 0x98) <= v3) {
      if ((!v2) && (*(long *)(a0 + 0x120))) {
        sub_1a550(*(struct_29 **)(a0 + 0x120));
        free(*(void **)(a0 + 0x120));
        *(unsigned long *)(a0 + 0x120) = 0;
      }
      sub_19dd5(a0);
      *(char *)(a0 + 0x418) = 0;
      *(void **)(a0 + 0x410) = sub_19cd3;
      *(char *)(a0 + 0xc0) = 1;
      return;
    }
    v1 = *(long *)(v3 * 8 + *(long *)(a0 + 0x90));
    if (v1 == 0x113) break;
    if (v1 <= 0x113) {
      if (v1 == 0x10a)
        abort(); // no-return
      if (v1 == 0x112)
        v2 = 1;
    }
    v3 += 1;
  }
}

// Function: sub_1a02d @ 0x1a02d
void sub_1a02d(unsigned long *a0)
{
  unsigned long *v1;
  unsigned long *v10;
  unsigned long *v11;
  long v12; // stack - 0x50
  long v13; // stack - 0x48
  unsigned long *v2;
  unsigned long v3;
  bool v4;
  bool v5;
  unsigned long *v6; // rax
  long v7; // rax
  long v8;
  char v9 [40];
  
  v6 = (unsigned long *)sub_1bb47();
  v8 = 0x124;
  v10 = a0;
  v11 = v6;
  while (v8) {
    v2 = &v11[1];
    v1 = &v10[1];
    *v11 = *v10;
    v8 -= 1;
    v10 = v1;
    v11 = v2;
  }
  *(char *)&v6[0x83] = 0;
  v6[0x82] = sub_19cd3;
  v6[0x23] = 0;
  v6[0x24] = 0;
  v6[0x25] = 0;
  v6[0x26] = 0;
  v6[0x2a] = 0;
  v6[0x2b] = 0;
  v6[0x28] = 0;
  v6[0x2c] = 0;
  v6[0x2e] = 0;
  v6[0x2f] = 0;
  v6[0x30] = 0;
  v6[0x31] = 0;
  v3 = v6[2];
  *v6 = sub_26c63(v3,0x20);
  if (a0[1])
    memcpy((void *)*v6,(void *)*a0,a0[1] << 5);
  v3 = a0[0x13];
  v6[0x12] = sub_26c63(v3,0x10);
  v6[0x14] = a0[0x13] * 2;
  v4 = 0;
  v5 = 0;
  v12 = 0;
  for (v13 = 0; v13 < (long)a0[0x13]; v13 = v13 + 1) {
    v8 = *(long *)(v13 * 8 + a0[0x12]);
    if (0x114 <= v8) { // branch-flip
label_1a36b:
      v8 = v12 + 1;
      *(unsigned long *)(v6[0x12] + v12 * 8) = *(unsigned long *)(a0[0x12] + v13 * 8);
      v12 = v8;
      if (((0 <= *(long *)(v13 * 8 + a0[0x12])) && (*(long *)(v13 * 8 + a0[0x12]) <= 0xff)) || (0x114 <= *(long *)(v13 * 8 + a0[0x12])))
        v5 = 1;
    }
    else if ((0x112 <= v8) || (v8 == 0x10a)) {
      sub_118fc(v9);
      v7 = sub_11a76(v6,v9);
      v8 = v12 + 1;
      *(long *)(v12 * 8 + v6[0x12]) = v7 + 0x114;
      v12 += 2;
      *(unsigned long *)(v8 * 8 + v6[0x12]) = 0x102;
      if (((*(long *)((v13 + 1) * 8 + a0[0x12]) == 0x101) || (*(long *)((v13 + 1) * 8 + a0[0x12]) == 0x102)) || (*(long *)((v13 + 1) * 8 + a0[0x12]) == 0x103))
        v13 += 1;
      v4 = 1;
    }
    else {
      if (((v8 < 0x10a) || (4 <= (unsigned long)(v8 - 0x10eU))) || (!*(char *)&a0[0x83])) goto label_1a36b;
      v8 = v12 + 1;
      *(unsigned long *)(v12 * 8 + v6[0x12]) = 0x100;
      v12 = v8;
    }
  }
  v6[0x13] = v12;
  if ((v5) && ((v4 || (*(char *)&a0[0x83]))))
    a0[0x24] = v6;
  else {
    sub_1a550(v6);
    free(v6);
  }
}

// Function: sub_1a490 @ 0x1a490
void sub_1a490(long a0,unsigned long a1,long a2,char a3) // return-dupe
{
  if (a0)
    sub_14b84(a0,a1,a2);
  sub_1a02d(a2);
  if (sub_19e86(a2)) { // branch-flip
    sub_19f0c(a2);
    sub_16c9f(a2,(unsigned char)a3);
  }
  else {
    *(void **)(a2 + 0x410) = sub_19d1f;
  }
  if (!*(long *)(a2 + 0x120))
    return;
  *(char *)(a2 + 0xc0) = 1;
  sub_16c9f(*(long *)(a2 + 0x120),(unsigned char)a3);
}

// Function: sub_1a550 @ 0x1a550
void sub_1a550(struct_29 *a0) // return-dupe
{
  long v1; // stack - 0x20
  long v2; // stack - 0x18
  long v3; // stack - 0x10
  
  free(a0->field_0x0);
  free(a0->field_0x90);
  if (a0->field_0x418)
    sub_19dd5(a0);
  free(a0->field_0x150);
  free(a0->field_0x158);
  for (v1 = 0; v1 < a0->field_0x130; v1 = v1 + 1) {
    free(*(void **)((long)a0->field_0x128 + v1 * 0x48 + 8));
    free(*(void **)((long)a0->field_0x128 + v1 * 0x48 + 0x28));
  }
  free(a0->field_0x128);
  if (a0->field_0x140) {
    for (v2 = 0; v2 < a0->field_0x98; v2 = v2 + 1) {
      free(*(void **)(v2 * 0x18 + a0->field_0x140));
    }
    free((void *)a0->field_0x140);
  }
  if (a0->field_0x170) {
    for (v3 = 0; v3 < a0->field_0x160; v3 = v3 + 1) {
      free(*(void **)(v3 * 8 + a0->field_0x170));
      free(*(void **)(v3 * 8 + (long)a0->field_0x178));
    }
    free((void *)(a0->field_0x170 + -0x10));
    free(a0->field_0x178);
    free(a0->field_0x188);
    free(a0->field_0x180);
  }
  if (!a0->field_0x120)
    return;
  sub_1a550(a0->field_0x120);
  free((void *)a0->field_0x120);
}

// Function: sub_1a7ba @ 0x1a7ba
char * sub_1a7ba(char *a0,char *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  
  v1 = strlen(a1);
  if (v1) {
    v2 = strlen(a0);
    a0 = (char *)sub_26b9f(a0,v1 + v2 + 1);
    memcpy(&a0[v2],a1,v1 + 1);
  }
  return a0;
}

// Function: sub_1a848 @ 0x1a848
void sub_1a848(long *a0)
{
  long *v1;
  long *v2; // stack - 0x10
  
  v2 = a0;
  while (*v2) {
    v1 = &v2[1];
    free((void *)*v2);
    v2 = v1;
  }
}

// Function: sub_1a881 @ 0x1a881
void * sub_1a881(void *a0,char *a1)
{
  char *v1;
  void *v2; // rax
  long v3; // stack - 0x18
  long v4; // stack - 0x10
  
  for (v3 = 0; *(long *)((long)a0 + v3 * 8); v3 = v3 + 1) {
    v1 = *(char **)((long)a0 + v3 * 8);
    if (strstr(v1,a1)) {
      free(a1);
      return a0;
    }
  }
  v4 = 0;
  while (*(long *)((long)a0 + v4 * 8)) {
    v1 = *(char **)((long)a0 + v4 * 8);
    if (strstr(a1,v1)) { // branch-flip
      free(*(void **)((long)a0 + v4 * 8));
      v3 -= 1;
      if (v3 == v4) break;
      *(unsigned long *)((long)a0 + v4 * 8) = *(unsigned long *)((long)a0 + v3 * 8);
      *(unsigned long *)((long)a0 + v3 * 8) = 0;
    }
    else {
      v4 += 1;
    }
  }
  v2 = (void *)sub_26bd0(a0,v3 + 2,8);
  *(char **)(v3 * 8 + (long)v2) = a1;
  *(unsigned long *)((long)v2 + (v3 + 1) * 8) = 0;
  return v2;
}

// Function: sub_1aa32 @ 0x1aa32
void sub_1aa32(void *a0,void *a1,unsigned long a2)
{
  sub_1a881(a0,(char *)sub_270a2(a1,a2));
}

// Function: sub_1aa71 @ 0x1aa71
void * sub_1aa71(char *a0,char *a1)
{
  char v1;
  void *v2; // stack - 0x30
  char *v3; // stack - 0x28
  unsigned long v4; // stack - 0x20
  char *v5; // stack - 0x18
  unsigned long v6; // stack - 0x10
  
  v2 = (void *)sub_26f82(8);
  for (v3 = a0; *v3; v3 = &v3[1]) {
    v4 = 0;
    v1 = *v3;
    for (v5 = strchr(a1,(int)v1); v5; v5 = strchr(&v5[1],(int)v1)) {
      for (v6 = 1; (v3[v6] && (v3[v6] == v5[v6])); v6 = v6 + 1) {
      }
      if ((long)v4 < (long)v6)
        v4 = v6;
      v1 = *v3;
    }
    if (v4)
      v2 = (void *)sub_1aa32(v2,v3,v4);
  }
  return v2;
}

// Function: sub_1ab7d @ 0x1ab7d
void * sub_1ab7d(void *a0,long *a1)
{
  char *v1; // rax
  long *v2; // stack - 0x18
  void *v3; // stack - 0x10
  
  v3 = a0;
  for (v2 = a1; *v2; v2 = &v2[1]) {
    v1 = (char *)sub_270f1((char *)*v2);
    v3 = (void *)sub_1a881(v3,v1);
  }
  return v3;
}

// Function: sub_1abcf @ 0x1abcf
void * sub_1abcf(long a0,long a1)
{
  long *v1; // rax
  void *v2; // stack - 0x28
  long v3; // stack - 0x20
  long v4; // stack - 0x18
  
  v2 = (void *)sub_26f82(8);
  for (v3 = 0; *(long *)(a0 + v3 * 8); v3 = v3 + 1) {
    for (v4 = 0; *(long *)(a1 + v4 * 8); v4 = v4 + 1) {
      v1 = (long *)sub_1aa71(*(char **)(a0 + v3 * 8),*(char **)(a1 + v4 * 8));
      v2 = (void *)sub_1ab7d(v2,v1);
      sub_1a848(v1);
      free(v1);
    }
  }
  return v2;
}

// Function: sub_1acc0 @ 0x1acc0
struct_24 * sub_1acc0(unsigned long a0,unsigned long a1)
{
  struct_24 *v1; // rax
  
  v1 = (struct_24 *)sub_26aea(0x30);
  v1->field_0x0 = sub_26f82(8);
  v1->field_0x8 = sub_26fa5(a1);
  v1->field_0x10 = sub_26fa5(a1);
  v1->field_0x18 = sub_26fa5(a1);
  v1->field_0x20 = 0;
  v1->field_0x21 = 0;
  v1->field_0x28 = a0;
  return v1;
}

// Function: sub_1ad5d @ 0x1ad5d
void sub_1ad5d(struct_30 *a0)
{
  char *v1;
  char *v2;
  
  sub_1a848(a0->field_0x0);
  *a0->field_0x0 = 0;
  v1 = a0->field_0x18;
  *v1 = 0;
  v2 = a0->field_0x10;
  *v2 = *v1;
  *a0->field_0x8 = *v2;
  a0->field_0x20 = 0;
  a0->field_0x21 = 0;
}

// Function: sub_1adc2 @ 0x1adc2
void sub_1adc2(unsigned long *a0)
{
  sub_1a848((long *)*a0);
  free((void *)*a0);
  free((void *)a0[1]);
  free((void *)a0[2]);
  free((void *)a0[3]);
  free(a0);
}

// Function: sub_1ae2f @ 0x1ae2f
char * sub_1ae2f(struct_25 *a0) // ternary x2
{
  char *v1;
  bool v10;
  int v11;
  char v12; // al
  int v13; // eax
  unsigned long v14; // rax
  unsigned long v15; // rax
  char *v16;
  long v17;
  bool v18; // zf
  char v19; // stack - 0xda
  char *v2;
  char v20; // stack - 0xd9
  char v21; // stack - 0xd8
  int v22; // stack - 0xd4
  long *v23; // stack - 0xd0
  char *v24; // stack - 0xc8
  long v25; // stack - 0xc0
  long v26; // stack - 0xb8
  long v27; // stack - 0xb0
  unsigned long v28; // stack - 0xa8
  long v29; // stack - 0xa0
  char *v3;
  long v30; // stack - 0x98
  long v31; // stack - 0x90
  unsigned long v32; // stack - 0x88
  char *v33; // stack - 0x80
  unsigned long v4;
  long v5;
  void *v6;
  void *v7;
  long *v8;
  bool v9;
  
  v23 = NULL;
  v24 = "";
  v19 = 0;
  v20 = 0;
  v21 = 0;
  v9 = 0;
  v10 = 0;
  v18 = ((a0->field_0x418 ^ 1) & a0->field_0x1cd) == 0;
  v25 = 1;
  do {
    if (a0->field_0x98 <= v25 + 1) {
label_1ba6d:
      v33 = NULL;
      if (*v24) {
        v33 = (char *)sub_26aea(strlen(v24) + 4);
        *v33 = v19;
        v33[1] = v20;
        v33[2] = v21;
        strcpy(&v33[3],v24);
      }
      while (v23) {
        v8 = (long *)v23[5];
        sub_1adc2(v23);
        v23 = v8;
      }
      return v33;
    }
    v26 = *(long *)(v25 * 8 + a0->field_0x90);
    if (0x114 <= v26) { // branch-flip
label_1b66b:
      if (0x114 <= v26) { // branch-flip
        v17 = v26 * 0x20 + -0x2280 + a0->field_0x0;
        v22 = 0;
        while ((v11 = v22, v22 <= 0xff && (!sub_117fa(v22,v17)))) {
          v22 += 1;
        }
        if (0x100 <= v22) // branch-flip
          v23 = (long *)sub_1acc0(v23,2);
        else {
          v26 = (long)v22;
          do {
            do {
              v22 += 1;
              if (0x100 <= v22) goto label_1b76f;
            } while (!sub_117fa(v22,v17));
            if (v18) break;
            v13 = toupper(v22);
          } while (v13 == toupper(v11));
label_1b76f:
          if (0xff < v22) goto label_1b79b;
          v23 = (long *)sub_1acc0(v23,2);
        }
      }
      else {
label_1b79b:
        v31 = v25 + 2;
        if (*(long *)((v25 + 1) * 8 + a0->field_0x90) == 0x105) {
          while ((v31 < a0->field_0x98 + -1 && (((v31 == v25 || ((1 <= *(long *)(v31 * 8 + a0->field_0x90) && (*(long *)(v31 * 8 + a0->field_0x90) <= 0xff)))) && (*(long *)((v31 + 1) * 8 + a0->field_0x90) == 0x105))))) {
            v31 += 2;
          }
        }
        v23 = (long *)sub_1acc0(v23,(v31 - v25 >> 1) + 1);
        v12 = (v18) ? (char)v26 : (char)toupper((int)v26);
        v2 = (char *)v23[2];
        *v2 = v12;
        v3 = (char *)v23[1];
        *v3 = *v2;
        *(char *)v23[3] = *v3;
        v32 = 1;
        while (v25 + 2 < v31) {
          v25 += 2;
          v4 = *(unsigned long *)(v25 * 8 + a0->field_0x90);
          v12 = (v18) ? (char)v4 : (char)toupper((int)v4);
          v17 = v23[2];
          *(char *)(v17 + v32) = v12;
          v5 = v23[1];
          *(char *)(v32 + v5) = *(char *)(v17 + v32);
          *(char *)(v32 + v23[3]) = *(char *)(v32 + v5);
          v32 += 1;
        }
        v17 = v23[2];
        *(char *)(v17 + v32) = 0;
        v5 = v23[1];
        *(char *)(v32 + v5) = *(char *)(v17 + v32);
        *(char *)(v32 + v23[3]) = *(char *)(v32 + v5);
        v6 = (void *)v23[3];
        v7 = (void *)*v23;
        *v23 = sub_1aa32(v7,v6,v32);
      }
    }
    else {
      if (v26 <= 0xff) {
        if (v26 != -1) { // branch-flip
          if (v26) goto label_1b66b;
        }
        else {
          if (v23[5])
            __assert_fail("!mp->prev","dfa.c",0x107d,"dfamust"); // no-return
          for (v30 = 0; *(long *)(v30 * 8 + *v23); v30 = v30 + 1) {
            v15 = strlen(*(char **)(v30 * 8 + *v23));
            if (strlen(v24) < v15)
              v24 = *(char **)(v30 * 8 + *v23);
          }
          v16 = (char *)v23[3];
          if (sub_11536(v24,v16)) {
            if (((!v9) || ((char)v23[4])) && ((!v10 || (*(char *)((long)v23 + 0x21)))))
              v19 = 1;
            v20 = *(char *)&v23[4];
            v21 = *(char *)((long)v23 + 0x21);
          }
        }
        goto label_1ba6d;
      }
      switch(v26) {
        case 0x100:
        case 0x10a:
        case 0x10e:
        case 0x10f:
        case 0x110:
        case 0x111:
        case 0x112:
        case 0x113:
          v23 = (long *)sub_1acc0(v23,2);
          break;
        case 0x101:
        case 0x102:
          sub_1ad5d(v23);
          break;
        case 0x103:
          *(char *)v23[3] = 0;
          break;
        default:
          goto label_1b66b;
        case 0x105:
          v8 = (long *)v23[5];
          v17 = *v23;
          v5 = *v8;
          *v8 = sub_1ab7d(v5,v17);
          if ((*(char *)v8[2]) && (*(char *)v23[1])) {
            v15 = strlen((char *)v8[2]);
            v14 = strlen((char *)v23[1]);
            v16 = (char *)sub_26b10(v14 + v15 + 1);
            memcpy(&v16[v15],(void *)v23[1],v14 + 1);
            memcpy(v16,(void *)v8[2],v15);
            v6 = (void *)*v8;
            *v8 = sub_1a881(v6,v16);
          }
          if (*(char *)v8[3]) {
            v17 = v23[1];
            v5 = v8[1];
            v8[1] = sub_1a7ba(v5,v17);
          }
          if (!*(char *)v23[3])
            *(char *)v8[2] = 0;
          v17 = v23[2];
          v5 = v8[2];
          v8[2] = sub_1a7ba(v5,v17);
          if (((*(char *)v8[3]) || ((char)v8[4])) && ((*(char *)v23[3] || (*(char *)((long)v23 + 0x21))))) {
            v17 = v23[3];
            v5 = v8[3];
            v8[3] = sub_1a7ba(v5,v17);
            *(char *)((long)v8 + 0x21) = *(char *)((long)v23 + 0x21);
          }
          else {
            *(char *)v8[3] = 0;
            *(char *)&v8[4] = 0;
            *(char *)((long)v8 + 0x21) = 0;
          }
          sub_1adc2(v23);
          v23 = v8;
          break;
        case 0x106:
          v8 = (long *)v23[5];
          v16 = (char *)v23[3];
          v1 = (char *)v8[3];
          if (sub_11536(v1,v16)) { // branch-flip
            *(bool *)&v8[4] = (*(unsigned char *)&v23[4] & *(unsigned char *)&v8[4]) != 0;
            *(bool *)((long)v8 + 0x21) = (*(unsigned char *)((long)v23 + 0x21) & *(unsigned char *)((long)v8 + 0x21)) != 0;
          }
          else {
            *(char *)v8[3] = 0;
            *(char *)&v8[4] = 0;
            *(char *)((long)v8 + 0x21) = 0;
          }
          for (v29 = 0; (*(char *)(v29 + v8[1]) && (*(char *)(v29 + v8[1]) == *(char *)(v29 + v23[1]))); v29 = v29 + 1) {
          }
          *(char *)(v29 + v8[1]) = 0;
          v15 = strlen((char *)v8[2]);
          v14 = strlen((char *)v23[2]);
          v28 = v15;
          if ((long)v14 < (long)v15)
            v28 = v14;
          v29 = 0;
          while ((v29 < (long)v28 && (*(char *)((v15 - v29) + -1 + v8[2]) == *(char *)((v14 - v29) + -1 + v23[2])))) {
            v29 += 1;
          }
          for (v27 = 0; v27 < v29; v27 = v27 + 1) {
            *(char *)(v8[2] + v27) = *(char *)(v8[2] + v27 + (v15 - v29));
          }
          *(char *)(v27 + v8[2]) = 0;
          v17 = sub_1abcf(*v8,*v23);
          sub_1a848((long *)*v8);
          free((void *)*v8);
          *v8 = v17;
          sub_1adc2(v23);
          v23 = v8;
          break;
        case 0x107:
        case 0x108:
          __assert_fail("!\"neither LPAREN nor RPAREN may appear here\"","dfa.c",0x1035,"dfamust"); // no-return
        case 0x10c:
          v23 = (long *)sub_1acc0(v23,2);
          *(char *)&v23[4] = 1;
          v9 = 1;
          break;
        case 0x10d:
          v23 = (long *)sub_1acc0(v23,2);
          *(char *)((long)v23 + 0x21) = 1;
          v10 = 1;
        
      }
    }
    v25 += 1;
  } while( true );
}

// Function: sub_1bb28 @ 0x1bb28
void sub_1bb28(char *a0)
{
  free(a0);
}

// Function: sub_1bb47 @ 0x1bb47
void sub_1bb47(void)
{
  sub_26aea(0x920);
}

// Function: sub_1bb5b @ 0x1bb5b
void sub_1bb5b(void *a0,char *a1,unsigned long a2,unsigned int a3) // ternary x3
{
  char *v1;
  unsigned long *v2;
  char v3;
  char v4; // al
  void *v5; // rax
  long v6; // rcx
  unsigned long *v7;
  bool v8; // al
  unsigned int v9; // stack - 0xc
  
  memset(a0,0,0x410);
  v5 = (*a1) ? sub_19c87 : sub_19cd3; // branch-flip
  *(void **)((long)a0 + 0x410) = v5;
  v6 = 0xa0;
  v7 = (unsigned long *)((long)a0 + 0x418);
  while (v6) {
    v2 = &v7[1];
    v1 = &a1[8];
    *v7 = *(unsigned long *)a1;
    v6 -= 1;
    a1 = v1;
    v7 = v2;
  }
  *(unsigned int *)v7 = *(unsigned int *)a1;
  *(bool *)((long)a0 + 0xc0) = *(char *)((long)a0 + 0x418) == '\0';
  *(unsigned long *)((long)a0 + 0x18) = 0xffffffffffffffff;
  *(char *)((long)a0 + 0x1cc) = 1;
  *(bool *)((long)a0 + 0x1cd) = (a2 & 0x400000) != 0;
  v4 = (a3 & 2) ? 0 : 10; // branch-flip
  *(char *)((long)a0 + 0x1ce) = v4;
  *(unsigned long *)((long)a0 + 0x1c0) = a2;
  *(unsigned int *)((long)a0 + 0x1c8) = a3;
  for (v9 = 0xffffff80; (int)v9 <= 0x7f; v9 = v9 + 1) {
    v4 = (char)v9;
    *(char *)((long)a0 + (long)(int)(v9 & 0xff) + 0x1cf) = sub_11bc7(a0,(unsigned char)v4);
    v3 = *(char *)((long)a0 + (long)(int)(v9 & 0xff) + 0x1cf);
    if (v3 != '\x02') { // branch-flip
      if (v3 == '\x04')
        sub_11834((unsigned char)v4,(long)a0 + 0x3f0);
    }
    else {
      sub_11834((unsigned char)v4,(long)a0 + 0x3d0);
    }
    v8 = (*(char *)((long)a0 + 0x41a)) ? (v9 & 0xc0) != 0x80 : strchr("\n\r./",v9 & 0xff) != NULL; // branch-flip
    *(bool *)((long)a0 + (long)(int)(v9 & 0xff) + 0x2cf) = v8;
  }
}

// Function: sub_1bd5a @ 0x1bd5a
void sub_1bd5a(void *a0,long a1)
{
  unsigned long *v1;
  unsigned long *v2;
  long v3; // rcx
  unsigned long *v4;
  unsigned long *v5;
  
  memset(a0,0,0x1c0);
  *(unsigned long *)((long)a0 + 0x18) = 0xffffffffffffffff;
  *(char *)((long)a0 + 0xc0) = *(char *)(a1 + 0xc0);
  v3 = 0x4a;
  v4 = (unsigned long *)(a1 + 0x1c0);
  v5 = (unsigned long *)((long)a0 + 0x1c0);
  while (v3) {
    v2 = &v5[1];
    v1 = &v4[1];
    *v5 = *v4;
    v3 -= 1;
    v4 = v1;
    v5 = v2;
  }
  *(unsigned long *)((long)a0 + 0x410) = *(unsigned long *)(a1 + 0x410);
  v3 = 0xa0;
  v4 = (unsigned long *)(a1 + 0x418);
  v5 = (unsigned long *)((long)a0 + 0x418);
  while (v3) {
    v2 = &v5[1];
    v1 = &v4[1];
    *v5 = *v4;
    v3 -= 1;
    v4 = v1;
    v5 = v2;
  }
  *(unsigned int *)v5 = *(unsigned int *)v4;
}

// Function: sub_1be0e @ 0x1be0e
unsigned int sub_1be0e(void) // return-dupe
{
  unsigned long v1; // stack - 0x18
  int v2; // stack - 0x1c
  
  v1 = 0;
  if ((sub_21b80(&v2,"Ā",2,&v1) == 2) && (v2 == 0x100))
    return 1;
  return 0;
}

// Function: sub_1be85 @ 0x1be85
unsigned long sub_1be85(bool a0) // return-dupe
{
  char v1 [3]; // stack - 0x14
  int v2; // stack - 0x18
  char v3; // stack - 0x11
  
  if (a0)
    return 0;
  v2 = 0;
  while( true ) {
    if (0xff <= v2)
      return 1;
    v1[0] = (char)v2;
    v1[2] = v1[0] + '\x01';
    v3 = 0;
    v1[1] = 0;
    if (0 <= strcoll(v1,&v1[2])) break;
    v2 += 1;
  }
  return 0;
}

// Function: sub_1bf18 @ 0x1bf18
void sub_1bf18(char *a0)
{
  char v1;
  unsigned int v2;
  unsigned long v3; // stack - 0x18
  unsigned char v4; // stack - 0x2a
  unsigned int v5; // stack - 0x28
  unsigned char v6; // stack - 0x29
  int v7; // stack - 0x24
  unsigned long v8; // stack - 0x20
  
  *a0 = 2 <= __ctype_get_mb_cur_max();
  v1 = *a0;
  a0[1] = sub_1be85((unsigned char)v1);
  a0[2] = sub_1be0e();
  for (v7 = -0x80; v7 <= 0x7f; v7 = v7 + 1) {
    v4 = (unsigned char)v7;
    v3 = 0;
    v6 = v4;
    v8 = sub_21b80(&v5,&v4,1,&v3);
    if (2 <= v8) // branch-flip
      v1 = (char)v8;
    else {
      v1 = 1;
    }
    a0[(long)(int)(unsigned int)v6 + 3] = v1;
    if (2 <= v8) // branch-flip
      v2 = 0xffffffff;
    else {
      v2 = v5;
    }
    *(unsigned int *)&a0[((long)(int)(unsigned int)v6 + 0x40) * 4 + 4] = v2;
  }
}

// Function: sub_1c01a @ 0x1c01a
unsigned int sub_1c01a(unsigned int a0,unsigned int *a1)
{
  unsigned int v1;
  unsigned int v2; // eax
  unsigned int v3; // eax
  unsigned int v4;
  unsigned int v5; // stack - 0x1c
  unsigned int v6; // stack - 0x18
  
  v2 = towupper(a0);
  v3 = towlower(v2);
  if (v2 != a0)
    *a1 = v2;
  v6 = (unsigned int)(v2 != a0);
  if ((v3 != v2) && (v3 != a0)) {
    if (v2 == towupper(v3)) {
      v4 = v6 + 1;
      a1[(int)v6] = v3;
      v6 = v4;
    }
  }
  for (v5 = 0; v5 <= 0x12; v5 = v5 + 1) {
    v4 = (unsigned int)*(short *)((long)(int)v5 * 2 + 0x2b1a0);
    if (((v4 != v3) && (v4 != v2)) && (v4 != a0)) {
      if (v2 == towupper(v4)) {
        v1 = v6 + 1;
        a1[(int)v6] = v4;
        v6 = v1;
      }
    }
  }
  return v6;
}

// Function: sub_1c12b @ 0x1c12b
bool sub_1c12b(char *a0)
{
  char v1;
  long v2; // rax
  char *v3; // stack - 0x18
  
  v3 = (char *)sub_2773c(a0);
  if (!*v3)
    v3 = a0;
  v2 = sub_277a9(v3);
  v1 = v3[v2];
  v3[v2] = '\0';
  return v1 != '\0';
}

// Function: sub_1c191 @ 0x1c191
void sub_1c191(long a0,unsigned long a1)
{
  struct_1 *v1; // rax
  
  v1 = (struct_1 *)sub_26aea(0x10);
  v1->field_0x8 = a1;
  v1->field_0x0 = *(unsigned long *)(a0 + 8);
  *(struct_1 **)(a0 + 8) = v1;
}

// Function: sub_1c1dd @ 0x1c1dd
unsigned long sub_1c1dd(char *a0,unsigned int a1) // return-dupe x3, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_1c1dd
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
label_1c296:
      if (a1 & 0x8000000)
        return 1;
    }
    if (v2 <= '}') {
      if (v2 == '{') goto label_1c296;
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
                  if (v4 & 0x2180) goto label_1c296;
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

// Function: sub_1c31c @ 0x1c31c
void sub_1c31c(char *a0) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_1c31c
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

// Function: sub_1c384 @ 0x1c384
void sub_1c384(void)
{
  sub_26f82(0x10);
}

// Function: sub_1c398 @ 0x1c398
void sub_1c398(unsigned char *a0,unsigned long a1)
{
  sub_204ee(a0,a1);
}

// Function: sub_1c3c9 @ 0x1c3c9
unsigned long sub_1c3c9(char *a0,unsigned long a1)
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
    sub_22011(v3);
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

// Function: sub_1c531 @ 0x1c531
unsigned long sub_1c531(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = strcmp(a0,a1);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == 0);
}

// Function: sub_1c56f @ 0x1c56f
unsigned long sub_1c56f(unsigned char *a0,unsigned char *a1)
{
  unsigned long v1; // rax
  
  v1 = sub_21c2a(a0,a1);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),(int)v1 == 0);
}

// Function: sub_1c5ad @ 0x1c5ad
void sub_1c5ad(void *a0)
{
  free(a0);
}

// Function: sub_1c5cc @ 0x1c5cc
void sub_1c5cc(unsigned long *a0,int a1,unsigned int a2) // ternary x2
{
  struct_31 *v1; // rax
  void *v2; // rax
  void *v3; // rdx
  
  v1 = (struct_31 *)sub_26f82(0x28);
  v1->field_0x8 = a1;
  v1->field_0xc = a2;
  if (!a1) {
    v3 = (a2 & 0x10) ? sub_1c56f : sub_1c531; // branch-flip
    v2 = (a2 & 0x10) ? sub_1c3c9 : sub_1c398; // branch-flip
    v1->field_0x10 = sub_208d6(0,0,v2,v3,sub_1c5ad);
  }
  v1->field_0x0 = *a0;
  *a0 = v1;
}

// Function: sub_1c688 @ 0x1c688
void sub_1c688(void *a0)
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
    sub_20b56(*(unsigned long *)((long)a0 + 0x10));
  }
  free(a0);
}

// Function: sub_1c750 @ 0x1c750
void sub_1c750(struct_32 *a0)
{
  unsigned long *v1;
  unsigned long *v2; // stack - 0x28
  unsigned long *v3; // stack - 0x20
  
  v2 = a0->field_0x0;
  while (v2) {
    v1 = (unsigned long *)*v2;
    sub_1c688(v2);
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

// Function: sub_1c7e6 @ 0x1c7e6
unsigned long sub_1c7e6(unsigned char *a0,unsigned char *a1,unsigned int a2)
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  unsigned char *v3; // rax
  char *v4; // rax
  unsigned int v5; // stack - 0x28
  unsigned char *v6; // stack - 0x20
  
  if (a2 & 8) { // branch-flip
    if (a2 & 0x10) { // branch-flip
      v3 = (unsigned char *)sub_270f1(a1);
      v6 = v3;
      while( true ) {
        v4 = strchr((char *)v6,0x2f);
        if (v4)
          *v4 = '\0';
        v1 = sub_21c2a(a0,v3);
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
    v2 = sub_21c2a(a0,a1);
  else {
    v2 = CONCAT44(dat_4,strcmp((char *)a0,(char *)a1));
  }
  return v2;
}

// Function: sub_1c921 @ 0x1c921
bool sub_1c921(unsigned long a0,char *a1,unsigned int a2)
{
  void *v1;
  bool v2; // stack - 0x19
  char *v3; // stack - 0x18
  
  if (a2 & 0x10000000) // branch-flip
    v1 = dat_32fe0;
  else {
    v1 = sub_1c7e6;
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

// Function: sub_1c9e9 @ 0x1c9e9
unsigned long sub_1c9e9(struct_7 *a0,char *a1)
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
    v5 = (unsigned long)(sub_1c921(v2,a1,v1) != '\0');
  }
  return v5;
}

// Function: sub_1ca61 @ 0x1ca61
unsigned long sub_1ca61(long a0,char *a1)
{
  long v1;
  long v2;
  long v3; // stack - 0x20
  
  v1 = *(long *)(a0 + 0x20);
  v2 = *(long *)(a0 + 0x10);
  v3 = 0;
  while( true ) {
    if (v1 <= v3)
      return 0;
    if (sub_1c9e9((struct_7 *)(v3 * 0x48 + v2),a1)) break;
    v3 += 1;
  }
  return 1;
}

// Function: sub_1cae3 @ 0x1cae3
unsigned long sub_1cae3(long a0,char *a1,char *a2) // return-dupe
{
  unsigned int v1;
  long v2;
  char *v3; // rax
  char *v4; // stack - 0x38
  
  v1 = *(unsigned int *)(a0 + 0xc);
  v2 = *(long *)(a0 + 0x10);
  v4 = a1;
  while( true ) {
    strcpy(a2,v4);
    while( true ) {
      if (sub_20221(v2,a2))
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

// Function: sub_1cbb6 @ 0x1cbb6
char sub_1cbb6(long *a0,char *a1) // early-return
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
      v1 = sub_1ca61(v4,a1);
    else {
      if (!v5) {
        v2 = (unsigned long)strlen(a1);
        v2 = (unsigned long)((long)v2 + 1);
        v2 = (char *)sub_26aea(v2);
        v5 = v2;
      }
      v1 = sub_1cae3(v4,a1,v5);
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

// Function: sub_1cca9 @ 0x1cca9
void sub_1cca9(long *a0,char *a1,unsigned int a2) // return-dupe
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
  
  if ((!(a2 & 0x18000000)) || (!sub_1c1dd(a1,a2))) {
    if (((!*a0) || (*(int *)(*a0 + 8))) || ((*(unsigned int *)(*a0 + 0xc) ^ a2) & 0x60000018))
      sub_1c5cc(a0,0,a2);
    v1 = *a0;
    v7 = (char *)sub_270f1(a1);
    if ((a2 & 0x10000002) == 0x10000000)
      sub_1c31c(v7);
    v1 = *(long *)(v1 + 0x10);
    if ((char *)sub_21617(v1,v7) == v7)
      return;
    free(v7);
    return;
  }
  if ((!*a0) || ((*(int *)(*a0 + 8) != 1 || ((*(unsigned int *)(*a0 + 0xc) ^ a2) & 0x20000000))))
    sub_1c5cc(a0,1,a2);
  v1 = *a0;
  v5 = (long *)(v1 + 0x10);
  if (*(long *)(v1 + 0x20) == *(long *)(v1 + 0x18)) {
    v2 = (void *)*v5;
    *v5 = sub_26dc6(v2,(long *)(v1 + 0x18),1,-1,0x48);
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
        v7 = (char *)sub_26b10(v10 + 7);
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
    v8 = (char *)sub_270f1(a1);
    sub_1c191(a0,v8);
  }
  *(char **)&v6[2] = v8;
}

// Function: sub_1cfe2 @ 0x1cfe2
unsigned long sub_1cfe2(void *a0,long a1,FILE *a2,unsigned int a3,unsigned char a4,unsigned long a5) // return-dupe, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_1cfe2
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
      v5 = (void *)sub_26dc6(v5,&v3,1,-1,1);
    v1 = v8 + 1;
    *(char *)((long)v5 + v8) = (char)v2;
    v8 = v1;
  }
  if (ferror(a2))
    v4 = *__errno_location();
  v5 = (unsigned char *)sub_26b9f(v5,v8 + 1);
  v5[v8] = a4;
  v2 = ((v8) && (a4 != v5[v8 + -1]));
  v10 = &v5[v2 + v8];
  sub_1c191(a1,v5);
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
          if (!(*(unsigned short *)((unsigned long)v9[-1] * 2 + *(long *)__ctype_b_loc()) & 0x2000)) goto label_1d1bd;
        }
      }
      else {
label_1d1bd:
        *v9 = 0;
        (*a0)(a1,v7,a3,a5);
      }
      v7 = &v6[1];
    }
    v6 = &v6[1];
  } while( true );
}

// Function: sub_1d231 @ 0x1d231
void sub_1d231(unsigned long a0,unsigned long a1,unsigned int a2,unsigned long *a3)
{
  (*(void *)*a3)(a0,a1,a2);
}

// Function: sub_1d272 @ 0x1d272
FILE * sub_1d272(unsigned long a0,long a1,char *a2,unsigned int a3,char a4) // early-return x2
{
  unsigned int v1; // eax
  FILE *v2; // rax
  unsigned long v3; // stack - 0x20
  unsigned int v4; // stack - 0x18
  int v5; // stack - 0x14
  FILE *v6; // stack - 0x10
  
  v3 = a0;
  v1 = strcmp(a2,"-");
  if (v1) { // branch-flip
    v2 = fopen(a2,"re");
    v6 = v2;
    if (!v2) {
      v2 = (FILE *)0xffffffff;
      return v2;
    }
    v1 = sub_1cfe2(sub_1d231,a1,v2,a3,(int)a4,&v3);
    v4 = v1;
    v2 = (int *)__errno_location();
    v5 = *(int *)v2;
    v1 = fclose(v6);
    if (v1) {
      v2 = (FILE *)0xffffffff;
      return v2;
    }
    v2 = (int *)__errno_location();
    *(int *)v2 = v5;
    v2 = (unsigned long)(unsigned long)v4;
  }
  else {
    v2 = (unsigned long)sub_1cfe2(sub_1d231,a1,stdin,a3,(int)a4,&v3);
  }
  return v2;
}

// Function: sub_1d366 @ 0x1d366
unsigned int sub_1d366(struct_3 *a0,struct_3 *a1) // return-dupe
{
  if ((a0->field_0x8 == a1->field_0x8) && (a0->field_0x0 == a1->field_0x0))
    return 1;
  return 0;
}

// Function: sub_1d3bf @ 0x1d3bf
unsigned long sub_1d3bf(long a0,unsigned long a1)
{
  return *(unsigned long *)(a0 + 8) % a1;
}

// Function: sub_1d3ed @ 0x1d3ed
unsigned long sub_1d3ed(long a0) // return-dupe x2, return-dupe
{
  if (!(*(unsigned int *)(a0 + 0x48) & 0x102)) {
    *(void **)(a0 + 0x58) = malloc(0x20);
    if (*(long *)(a0 + 0x58)) {
      sub_27aa3(*(long *)(a0 + 0x58));
      return 1;
    }
    return 0;
  }
  *(unsigned long *)(a0 + 0x58) = sub_208d6(0x1f,0,sub_1d3bf,sub_1d366,dat_32fc8);
  if (*(long *)(a0 + 0x58))
    return 1;
  return 0;
}

// Function: sub_1d499 @ 0x1d499
unsigned long sub_1d499(long a0,struct_11 *a1) // return-dupe x3
{
  struct_10 *v1;
  struct_12 *v2; // rax
  struct_12 *v3; // rax
  
  if (!(*(unsigned int *)(a0 + 0x48) & 0x102)) {
    v1 = *(struct_10 **)(a0 + 0x58);
    if (!sub_27ac9(v1,&a1->field_0x70))
      return 1;
    a1->field_0x0 = a1;
    a1->field_0x68 = 2;
    return 1;
  }
  v2 = malloc(0x18);
  if (!v2)
    return 0;
  *(unsigned long *)v2->field_0x0 = a1->field_0x70;
  *(unsigned long *)((long)v2->field_0x0 + 8) = a1->field_0x78;
  v2->field_0x10 = a1;
  v3 = (struct_12 *)sub_21617(*(long *)(a0 + 0x58),v2);
  if (v3 == v2)
    return 1;
  free(v2);
  if (v3) {
    a1->field_0x0 = v3->field_0x10;
    a1->field_0x68 = 2;
    return 1;
  }
  return 0;
}

// Function: sub_1d5ab @ 0x1d5ab
void sub_1d5ab(long a0,long a1)
{
  long v1;
  void *v2; // rax
  long v3; // stack - 0x28
  unsigned long v4; // stack - 0x20
  
  if (*(unsigned int *)(a0 + 0x48) & 0x102) { // branch-flip
    v3 = *(long *)(a1 + 0x70);
    v4 = *(unsigned long *)(a1 + 0x78);
    v2 = (void *)sub_21687(*(long *)(a0 + 0x58),&v3);
    if (!v2)
      abort(); // no-return
    free(v2);
  }
  else {
    v1 = *(long *)(a1 + 8);
    if ((v1) && (0 <= *(long *)(v1 + 0x58))) {
      if (!*(long *)(*(long *)(a0 + 0x58) + 0x10))
        abort(); // no-return
      if ((**(long **)(a0 + 0x58) == *(long *)(a1 + 0x78)) && (*(long *)(*(long *)(a0 + 0x58) + 8) == *(long *)(a1 + 0x70))) {
        *(unsigned long *)(*(long *)(a0 + 0x58) + 8) = *(unsigned long *)(v1 + 0x70);
        **(unsigned long **)(a0 + 0x58) = *(unsigned long *)(v1 + 0x78);
      }
    }
  }
}

// Function: sub_1d6dd @ 0x1d6dd
void sub_1d6dd(long a0) // return-dupe
{
  if (!(*(unsigned int *)(a0 + 0x48) & 0x102)) {
    free(*(void **)(a0 + 0x58));
    return;
  }
  if (*(long *)(a0 + 0x58)) {
    sub_20b56(*(struct_8 **)(a0 + 0x58));
    return;
  }
}

// Function: sub_1d72f @ 0x1d72f
void sub_1d72f(long a0)
{
  int v1; // eax
  
  while( true ) {
    if (sub_21a13(a0) == '\x01') break;
    v1 = sub_21ab5(a0);
    if (0 <= v1)
      close(v1);
  }
}

// Function: sub_1d777 @ 0x1d777
void sub_1d777(long a0,bool a1) // ternary
{
  unsigned long v1; // rdx
  
  if (*(short *)(a0 + 0x68) != 0xb)
    abort(); // no-return
  v1 = (a1) ? 2 : 1; // branch-flip
  *(unsigned long *)(a0 + 0xa0) = v1;
}

// Function: sub_1d7bf @ 0x1d7bf
void sub_1d7bf(long a0,int a1,bool a2)
{
  int v1;
  
  v1 = *(int *)(a0 + 0x2c);
  if ((v1 == a1) && (v1 != -100))
    abort(); // no-return
  if (a2) { // branch-flip
    v1 = sub_21a29(a0 + 0x60,v1);
    if (0 <= v1)
      close(v1);
  }
  else if ((!(*(unsigned int *)(a0 + 0x48) & 4)) && (0 <= v1))
    close(v1);
  *(int *)(a0 + 0x2c) = a1;
}

// Function: sub_1d84f @ 0x1d84f
unsigned int sub_1d84f(long a0)
{
  unsigned int v1;
  int v2;
  
  if (!(*(unsigned int *)(a0 + 0x48) & 4)) {
    if (*(unsigned int *)(a0 + 0x48) & 0x200) { // branch-flip
      if (*(unsigned int *)(a0 + 0x48) & 0x200) // branch-flip
        v1 = 0xffffff9c;
      else {
        v1 = *(unsigned int *)(a0 + 0x28);
      }
      sub_1d7bf(a0,v1,1);
    }
    else {
      if (*(unsigned int *)(a0 + 0x48) & 0x200) // branch-flip
        v2 = -100;
      else {
        v2 = *(int *)(a0 + 0x28);
      }
      if (fchdir(v2)) {
        v1 = 1;
        sub_1d72f(a0 + 0x60); // return-dupe
        return v1;
      }
    }
  }
  v1 = 0;
  sub_1d72f(a0 + 0x60);
  return v1;
}

// Function: sub_1d8fd @ 0x1d8fd
unsigned int sub_1d8fd(long a0,char *a1)
{
  unsigned int v1; // eax
  unsigned int v2; // eax
  
  v1 = (*(unsigned int *)(a0 + 0x48) & 0x10) << 0xd | 0x90900;
  if (*(unsigned int *)(a0 + 0x48) & 0x200) // branch-flip
    v2 = sub_22af5(*(unsigned int *)(a0 + 0x2c),a1,v1);
  else {
    v2 = sub_28056(a1,v1);
  }
  return v2;
}

// Function: sub_1d975 @ 0x1d975
long * sub_1d975(long *a0,unsigned int a1,long a2) // early-return x5
{
  bool v1; // al
  long *v2; // rax
  unsigned long v3; // rax
  long v4; // rax
  long v5;
  long *v6; // stack - 0x70
  void *v7; // stack - 0x60
  long v8; // stack - 0x58
  unsigned long v9; // stack - 0x50
  
  v7 = NULL;
  v8 = 0;
  if (a1 & 0xfffff000) {
    *__errno_location() = 0x16;
    return NULL;
  }
  if ((a1 & 4) && (a1 & 0x200)) {
    *__errno_location() = 0x16;
    return NULL;
  }
  if (!(a1 & 0x12)) {
    *__errno_location() = 0x16;
    return NULL;
  }
  v2 = calloc(1,0x80);
  if (!v2)
    return NULL;
  v2[8] = a2;
  *(unsigned int *)&v2[9] = a1;
  if (*(unsigned int *)&v2[9] & 2) {
    *(unsigned int *)&v2[9] = *(unsigned int *)&v2[9] | 4;
    *(unsigned int *)&v2[9] = *(unsigned int *)&v2[9] & 0xfffffdff;
  }
  *(unsigned int *)((long)v2 + 0x2c) = 0xffffff9c;
  v3 = sub_1fbe3(a0);
  if (v3 <= 0xfff)
    v3 = 0x1000;
  if (sub_1f9ff(v2,v3) != '\x01') {
    free(v2); // return-dupe
    return NULL;
  }
  if (*a0) { // branch-flip
    v7 = (void *)sub_1f908(v2,(void *)0x2b1d4,0);
    if (v7) {
      *(unsigned long *)((long)v7 + 0x58) = 0xffffffffffffffff;
      goto label_1dad4;
    }
  }
  else {
label_1dad4:
    if ((a2) && (!(*(unsigned int *)&v2[9] & 0x400)))
      v1 = 0;
    else {
      v1 = 1;
    }
    v5 = 0;
    v3 = 0;
    for (v6 = a0; *v6; v6 = &v6[1]) {
      v9 = strlen((char *)*v6);
      if (((!(a1 & 0x800)) && (3 <= v9)) && (*(char *)(*v6 + (v9 - 1)) == '/')) {
        for (; (2 <= v9 && (*(char *)(*v6 + (v9 - 2)) == '/')); v9 = v9 - 1) {
        }
      }
      v4 = sub_1f908(v2,(void *)*v6,v9);
      if (!v4) goto label_1dcfe;
      *(unsigned long *)(v4 + 0x58) = 0;
      *(void **)(v4 + 8) = v7;
      *(long *)(v4 + 0x30) = v4 + 0x100;
      if ((v1) && (v5)) {
        *(unsigned short *)(v4 + 0x68) = 0xb;
        sub_1d777(v4,1);
      }
      else {
        *(unsigned short *)(v4 + 0x68) = sub_1f5cd(v2,v4,0);
      }
      if (a2) { // branch-flip
        *(long *)(v4 + 0x10) = v5;
        v5 = v4;
      }
      else {
        *(unsigned long *)(v4 + 0x10) = 0;
        if (v5) { // branch-flip
          *(long *)(v8 + 0x10) = v4;
          v8 = v4;
        }
        else {
          v8 = v4;
          v5 = v4;
        }
      }
      v3 += 1;
    }
    if ((a2) && (2 <= v3))
      v5 = sub_1f79a(v2,v5,v3);
    *v2 = sub_1f908(v2,(void *)0x2b1d4,0);
    if (*v2) {
      *(long *)(*v2 + 0x10) = v5;
      *(unsigned short *)(*v2 + 0x68) = 9;
      *(unsigned long *)(*v2 + 0x58) = 1;
      if (sub_1d3ed(v2) == '\x01') {
        if ((!(*(unsigned int *)&v2[9] & 4)) && (!(*(unsigned int *)&v2[9] & 0x200))) {
          *(unsigned int *)&v2[5] = sub_1d8fd(v2,".");
          if ((int)v2[5] <= -1)
            *(unsigned int *)&v2[9] = *(unsigned int *)&v2[9] | 4;
        }
        sub_219b6(&v2[0xc],0xffffffff);
        return v2;
      }
    }
label_1dcfe:
    sub_1f9be(v5);
    free(v7);
  }
  free((void *)v2[4]);
  free(v2);
  return NULL;
}

// Function: sub_1dd40 @ 0x1dd40
void sub_1dd40(struct_14 *a0,long a1)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  *(unsigned long *)(a1 + 0x48) = *(unsigned long *)(a1 + 0x60);
  memmove((void *)a0->field_0x20,(void *)(a1 + 0x100),*(long *)(a1 + 0x48) + 1);
  v1 = strrchr((char *)(a1 + 0x100),0x2f);
  if ((v1) && ((v1 != (char *)(a1 + 0x100) || (v1[1])))) {
    v2 = strlen(&v1[1]);
    memmove((void *)(a1 + 0x100),&v1[1],v2 + 1);
    *(unsigned long *)(a1 + 0x60) = v2;
  }
  *(long *)(a1 + 0x38) = a0->field_0x20;
  *(unsigned long *)(a1 + 0x30) = *(unsigned long *)(a1 + 0x38);
}

// Function: sub_1ddff @ 0x1ddff
unsigned long sub_1ddff(struct_9 *a0) // return-dupe, ternary
{
  int v1;
  void *v2;
  void *v3;
  int v4; // stack - 0x1c
  
  v4 = 0;
  if (a0->field_0x0) {
    v3 = (void *)a0->field_0x0;
    while (0 <= *(long *)((long)v3 + 0x58)) {
      v2 = (*(long *)((long)v3 + 0x10)) ? *(void **)((long)v3 + 0x10) : *(void **)((long)v3 + 8); // branch-flip
      free(v3);
      v3 = v2;
    }
    free(v3);
  }
  if (a0->field_0x8)
    sub_1f9be((void *)a0->field_0x8);
  free(a0->field_0x10);
  free(a0->field_0x20);
  if (a0->field_0x48 & 0x200) { // branch-flip
    if ((0 <= a0->field_0x2c) && (v1 = a0->field_0x2c, close(v1)))
      v4 = *__errno_location();
  }
  else if (!(a0->field_0x48 & 4)) {
    v1 = a0->field_0x28;
    if (fchdir(v1))
      v4 = *__errno_location();
    v1 = a0->field_0x28;
    if ((close(v1)) && (!v4))
      v4 = *__errno_location();
  }
  sub_1d72f(&a0[1].field_0x8);
  if (a0->field_0x50)
    sub_20b56((struct_8 *)a0->field_0x50);
  sub_1d6dd(a0);
  free(a0);
  if (v4) {
    *__errno_location() = v4;
    return 0xffffffff;
  }
  return 0;
}

// Function: sub_1df8c @ 0x1df8c
unsigned long sub_1df8c(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_1dfc1 @ 0x1dfc1
unsigned long sub_1dfc1(long *a0,long *a1)
{
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_1dff7 @ 0x1dff7
unsigned long sub_1dff7(long a0,int a1) // early-return x2
{
  long v1;
  struct_1 *v2; // rax
  struct_1 *v3; // rax
  unsigned long v4 [2]; // stack - 0x98
  statfs v5; // stack - 0x88
  long v6; // stack - 0xb8
  
  v1 = *(long *)(a0 + 0x50);
  v6 = *(long *)(v1 + 0x50);
  if (!(*(unsigned int *)(v1 + 0x48) & 0x200))
    return 0;
  if (!v6) {
    *(unsigned long *)(v1 + 0x50) = sub_208d6(0xd,0,sub_1df8c,sub_1dfc1,dat_32fc8);
    v6 = *(long *)(v1 + 0x50);
  }
  if (v6) {
    v4[0] = *(unsigned long *)(a0 + 0x70);
    v1 = sub_20221(v6,v4);
    if (v1)
      return *(unsigned long *)(v1 + 8);
  }
  if ((0 <= a1) && (!fstatfs(a1,&v5))) {
    if ((v6) && (v2 = malloc(0x10), v2)) {
      v2->field_0x0 = *(unsigned long *)(a0 + 0x70);
      v2->field_0x8 = v5._0_8_;
      v3 = (struct_1 *)sub_21617(v6,v2);
      if (v3) { // branch-flip
        if (v3 != v2)
          abort(); // no-return
      }
      else {
        free(v2);
      }
    }
    return v5._0_8_;
  }
  return 0;
}

// Function: sub_1e1f0 @ 0x1e1f0
unsigned long sub_1e1f0(long a0,unsigned int a1) // early-return
{
  long v1; // rax
  
  v1 = sub_1dff7(a0,a1);
  if ((v1 != 0xff534d42) && ((0xff534d43 <= v1 || ((v1 != 0x6969 && (v1 != 0x1021994))))))
    return 1;
  return 0;
}

// Function: sub_1e246 @ 0x1e246
unsigned long sub_1e246(long a0,unsigned int a1) // early-return
{
  long v1; // rax
  
  v1 = sub_1dff7(a0,a1);
  if ((v1 != 0xff534d42) && ((0xff534d43 <= v1 || ((v1 != 0x5346414f && ((0x53464150 <= v1 || ((v1 != 0x9fa0 && ((0x9fa1 <= v1 || ((v1 && (v1 != 0x6969))))))))))))))
    return 1;
  return 0;
}

// Function: sub_1e2b9 @ 0x1e2b9
void * sub_1e2b9(struct_14 *a0) // early-return x7, ternary
{
  char *v1;
  short v2;
  int v3;
  char *v4;
  unsigned short v5; // ax
  void *v6;
  long v7;
  void *v8;
  bool v9;
  
  if ((!a0->field_0x0) || (a0->field_0x48 & 0x2000))
    return NULL;
  v6 = (void *)a0->field_0x0;
  v2 = *(short *)((long)v6 + 0x6c);
  *(unsigned short *)((long)v6 + 0x6c) = 3;
  if (v2 == 1) {
    *(unsigned short *)((long)v6 + 0x68) = sub_1f5cd(a0,v6,0);
    return v6;
  }
  if ((v2 != 2) || ((*(short *)((long)v6 + 0x68) != 0xc && (*(short *)((long)v6 + 0x68) != 0xd)))) {
    v8 = v6;
    if (*(short *)((long)v6 + 0x68) != 1) { // branch-flip
      do {
        if ((!*(long *)((long)v8 + 0x10)) && (*(long *)(*(long *)((long)v8 + 8) + 0x18))) {
          v7 = *(long *)((long)v8 + 8);
          a0->field_0x0 = v7;
          *(char *)(*(long *)(v7 + 0x48) + a0->field_0x20) = 0;
          v6 = (void *)sub_1ec09(a0,3);
          if (!v6) {
            if (a0->field_0x48 & 0x2000)
              return NULL;
label_1e789:
            v6 = *(void **)((long)v8 + 8);
            a0->field_0x0 = (long)v6;
            free(v8);
            if (*(long *)((long)v6 + 0x58) == -1) {
              free(v6);
              *__errno_location() = 0;
              a0->field_0x0 = 0;
              return NULL;
            }
            if (*(short *)((long)v6 + 0x68) != 0xb) {
              *(char *)(*(long *)((long)v6 + 0x48) + a0->field_0x20) = 0;
              if (*(long *)((long)v6 + 0x58)) { // branch-flip
                if (*(unsigned short *)((long)v6 + 0x6a) & 2) { // branch-flip
                  if (!(a0->field_0x48 & 4)) {
                    if (a0->field_0x48 & 0x200) { // branch-flip
                      sub_1d7bf(a0,*(unsigned int *)((long)v6 + 0x44),1);
                      v9 = 0;
                    }
                    else {
                      v3 = *(int *)((long)v6 + 0x44);
                      v9 = fchdir(v3) != 0;
                    }
                    if (v9) {
                      *(int *)((long)v6 + 0x40) = *__errno_location();
                      a0->field_0x48 = a0->field_0x48 | 0x2000;
                    }
                  }
                  close(*(int *)((long)v6 + 0x44));
                }
                else if ((!(*(unsigned short *)((long)v6 + 0x6a) & 1)) && (v7 = *(long *)((long)v6 + 8), sub_1fc3d(a0,v7,0xffffffff,".."))) {
                  *(int *)((long)v6 + 0x40) = *__errno_location();
                  a0->field_0x48 = a0->field_0x48 | 0x2000;
                }
              }
              else if (sub_1d84f(a0)) {
                *(int *)((long)v6 + 0x40) = *__errno_location();
                a0->field_0x48 = a0->field_0x48 | 0x2000;
              }
              if (*(short *)((long)v6 + 0x68) != 2) {
                v5 = (*(int *)((long)v6 + 0x40)) ? 7 : 6; // branch-flip
                *(unsigned short *)((long)v6 + 0x68) = v5;
                if (!*(int *)((long)v6 + 0x40))
                  sub_1d5ab(a0,v6);
              }
              if (!(a0->field_0x48 & 0x2000))
                return v6;
              return NULL;
            }
            abort(); // no-return
          }
          free(v8);
          goto label_1e693;
        }
        v6 = *(void **)((long)v8 + 0x10);
        if (!v6) goto label_1e789;
        a0->field_0x0 = (long)v6;
        free(v8);
        if (!*(long *)((long)v6 + 0x58)) {
          if (sub_1d84f(a0)) {
            a0->field_0x48 = a0->field_0x48 | 0x2000;
            return NULL;
          }
          sub_1d6dd(a0);
          sub_1dd40(a0,v6);
          sub_1d3ed(a0);
          goto label_1e6f7;
        }
        v8 = v6;
      } while (*(short *)((long)v6 + 0x6c) == 4);
      if (*(short *)((long)v6 + 0x6c) == 2) {
        *(unsigned short *)((long)v6 + 0x68) = sub_1f5cd(a0,v6,1);
        if ((*(short *)((long)v6 + 0x68) == 1) && (!(a0->field_0x48 & 4))) {
          *(unsigned int *)((long)v6 + 0x44) = sub_1d8fd(a0,".");
          if (0 <= *(int *)((long)v6 + 0x44)) // branch-flip
            *(unsigned short *)((long)v6 + 0x6a) = *(unsigned short *)((long)v6 + 0x6a) | 2;
          else {
            *(int *)((long)v6 + 0x40) = *__errno_location();
            *(unsigned short *)((long)v6 + 0x68) = 7;
          }
        }
        *(unsigned short *)((long)v6 + 0x6c) = 3;
      }
    }
    else {
      if ((v2 == 4) || ((a0->field_0x48 & 0x40 && (*(long *)((long)v6 + 0x70) != a0->field_0x18)))) {
        if (*(unsigned short *)((long)v6 + 0x6a) & 2)
          close(*(int *)((long)v6 + 0x44));
        if (a0->field_0x8) {
          sub_1f9be((void *)a0->field_0x8);
          a0->field_0x8 = 0;
        }
        *(unsigned short *)((long)v6 + 0x68) = 6;
        sub_1d5ab(a0,v6);
        return v6;
      }
      if ((a0->field_0x8) && (a0->field_0x48 & 0x1000)) {
        a0->field_0x48 = a0->field_0x48 & 0xffffefff;
        sub_1f9be((void *)a0->field_0x8);
        a0->field_0x8 = 0;
      }
      if (a0->field_0x8) { // branch-flip
        v4 = *(char **)((long)v6 + 0x30);
        if (sub_1fc3d(a0,v6,0xffffffff,v4)) {
          *(int *)((long)v6 + 0x40) = *__errno_location();
          *(unsigned short *)((long)v6 + 0x6a) = *(unsigned short *)((long)v6 + 0x6a) | 1;
          for (v7 = a0->field_0x8; v7; v7 = *(long *)(v7 + 0x10)) {
            *(unsigned long *)(v7 + 0x30) = *(unsigned long *)(*(long *)(v7 + 8) + 0x30);
          }
        }
      }
      else {
        a0->field_0x8 = sub_1ec09(a0,3);
        if (!a0->field_0x8) {
          if (!(a0->field_0x48 & 0x2000)) {
            if ((*(int *)((long)v6 + 0x40)) && (*(short *)((long)v6 + 0x68) != 4))
              *(unsigned short *)((long)v6 + 0x68) = 7;
            sub_1d5ab(a0,v6);
            return v6;
          }
          return NULL;
        }
      }
      v6 = (void *)a0->field_0x8;
      a0->field_0x8 = 0;
    }
label_1e693:
    if (*(char *)(*(long *)(*(long *)((long)v6 + 8) + 0x48) + -1 + *(long *)(*(long *)((long)v6 + 8) + 0x38)) != '/') // branch-flip
      v7 = *(long *)(*(long *)((long)v6 + 8) + 0x48);
    else {
      v7 = *(long *)(*(long *)((long)v6 + 8) + 0x48) + -1;
    }
    v1 = (char *)(a0->field_0x20 + v7);
    *v1 = 0x2f;
    memmove(&v1[1],(void *)((long)v6 + 0x100),*(long *)((long)v6 + 0x60) + 1);
  }
  else {
    *(unsigned short *)((long)v6 + 0x68) = sub_1f5cd(a0,v6,1);
    if ((*(short *)((long)v6 + 0x68) == 1) && (!(a0->field_0x48 & 4))) {
      *(unsigned int *)((long)v6 + 0x44) = sub_1d8fd(a0,".");
      if (0 <= *(int *)((long)v6 + 0x44)) // branch-flip
        *(unsigned short *)((long)v6 + 0x6a) = *(unsigned short *)((long)v6 + 0x6a) | 2;
      else {
        *(int *)((long)v6 + 0x40) = *__errno_location();
        *(unsigned short *)((long)v6 + 0x68) = 7;
      }
    }
  }
label_1e6f7:
  a0->field_0x0 = (long)v6;
  if (*(short *)((long)v6 + 0x68) == 0xb) {
    if (*(long *)((long)v6 + 0xa0) != 2) { // branch-flip
      if (*(long *)((long)v6 + 0xa0) != 1)
        abort(); // no-return
    }
    else {
      *(unsigned short *)((long)v6 + 0x68) = sub_1f5cd(a0,v6,0);
    }
  }
  if (*(short *)((long)v6 + 0x68) == 1) {
    if (!*(long *)((long)v6 + 0x58))
      a0->field_0x18 = *(long *)((long)v6 + 0x70);
    if (sub_1d499(a0,v6) != '\x01') {
      *__errno_location() = 0xc;
      return NULL;
    }
  }
  return v6;
}

// Function: sub_1e92f @ 0x1e92f
unsigned long sub_1e92f(unsigned long a0,long a1,int a2) // return-dupe
{
  if ((((a2) && (a2 != 1)) && (a2 != 2)) && ((a2 != 3 && (a2 != 4)))) {
    *__errno_location() = 0x16;
    return 1;
  }
  *(short *)(a1 + 0x6c) = (short)a2;
  return 0;
}

// Function: sub_1e98a @ 0x1e98a
int * sub_1e98a(struct_14 *a0,int a1) // early-return x4
{
  int v1;
  long v2;
  int v3;
  int v4; // eax
  int *v5; // rax
  unsigned int v6; // stack - 0x2c
  
  if ((a1) && (a1 != 0x1000)) {
    v5 = __errno_location();
    *v5 = 0x16;
    v5 = NULL;
    return v5;
  }
  v2 = a0->field_0x0;
  v5 = __errno_location();
  *v5 = 0;
  v4 = a0->field_0x48 & 0x2000;
  if (v4) {
    v5 = NULL;
    return v5;
  }
  if (*(short *)(v2 + 0x68) != 9) { // branch-flip
    if (*(short *)(v2 + 0x68) != 1) {
      v5 = NULL;
      return v5;
    }
    if (a0->field_0x8)
      sub_1f9be((void *)a0->field_0x8);
    if (a1 != 0x1000) // branch-flip
      v6 = 1;
    else {
      v4 = a0->field_0x48 | 0x1000;
      a0->field_0x48 = v4;
      v6 = 2;
    }
    if (((*(long *)(v2 + 0x58)) || (**(char **)(v2 + 0x30) == '/')) || (v4 = a0->field_0x48 & 4, v4)) {
      v5 = (long)sub_1ec09(a0,v6);
      a0->field_0x8 = (long)v5;
      v5 = (long)a0->field_0x8;
    }
    else {
      v4 = sub_1d8fd(a0,".");
      v3 = v4;
      if (v4 <= -1) {
        a0->field_0x8 = 0;
        v5 = NULL;
        return v5;
      }
      v5 = (long)sub_1ec09(a0,v6);
      a0->field_0x8 = (long)v5;
      v4 = a0->field_0x48 & 0x200;
      if (v4) // branch-flip
        sub_1d7bf(a0,v3,1);
      else {
        v4 = fchdir(v3);
        if (v4) {
          v5 = __errno_location();
          v1 = *v5;
          close(v3);
          v5 = __errno_location();
          *v5 = v1;
          v5 = NULL;
          return v5;
        }
        close(v3);
      }
      v5 = (long)a0->field_0x8;
    }
  }
  else {
    v5 = *(int **)(v2 + 0x10);
  }
  return v5;
}

// Function: sub_1eb29 @ 0x1eb29
int sub_1eb29(long *a0,long *a1)
{
  return (unsigned int)(*(unsigned long *)(*a1 + 0x78) < *(unsigned long *)(*a0 + 0x78)) - (unsigned int)(*(unsigned long *)(*a0 + 0x78) < *(unsigned long *)(*a1 + 0x78));
}

// Function: sub_1eb7d @ 0x1eb7d
void sub_1eb7d(long a0,unsigned int a1)
{
  unsigned int v1; // stack - 0xc
  
  switch(a1) {
    default:
      v1 = 0;
      break;
    case 1:
      v1 = 0x1000;
      break;
    case 2:
      v1 = 0x2000;
      break;
    case 4:
      v1 = 0x4000;
      break;
    case 6:
      v1 = 0x6000;
      break;
    case 8:
      v1 = 0x8000;
      break;
    case 10:
      v1 = 0xa000;
      break;
    case 0xc:
      v1 = 0xc000;
    
  }
  *(unsigned int *)(a0 + 0x18) = v1;
}

// Function: sub_1ec09 @ 0x1ec09
void * sub_1ec09(struct_13 *a0,int a1) // early-return, return-dupe x2, ternary, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_1ec09
{
  bool v1;
  void *v10; // rax
  int v11; // stack - 0x98
  unsigned long v12; // rax
  void *v13;
  unsigned long v14; // r14
  bool v15; // zf
  bool v16; // stack - 0x9b
  int v17; // stack - 0x94
  void *v18; // stack - 0x90
  unsigned long v19; // stack - 0x88
  char v2; // al
  char *v20; // stack - 0x80
  long v21; // stack - 0x78
  DIR *v22; // stack - 0x70
  unsigned long v23; // stack - 0x68
  unsigned long v24; // stack - 0x60
  long v25; // stack - 0x58
  dirent *v26; // stack - 0x50
  unsigned long v27; // stack - 0x48
  long v28; // stack - 0x40
  unsigned long v29; // stack - 0x38
  bool v3;
  bool v4; // al
  unsigned short v5; // ax
  unsigned int v6;
  int v7; // eax
  long v8;
  char *v9; // rax
  
  v21 = a0->field_0x0;
  v15 = *(long *)(v21 + 0x18) != 0;
  v16 = 0;
  if (v15) {
    v22 = *(DIR **)(v21 + 0x18);
    v11 = dirfd(v22);
    if (v11 <= -1) {
      closedir(*(DIR **)(v21 + 0x18));
      *(unsigned long *)(v21 + 0x18) = 0;
      if (a1 != 3)
        return NULL;
      *(unsigned short *)(v21 + 0x68) = 4;
      *(int *)(v21 + 0x40) = *__errno_location();
      return NULL;
    }
  }
  else {
    if ((a0->field_0x48 & 0x10) && ((!(a0->field_0x48 & 1) || (*(long *)(v21 + 0x58)))))
      v12 = 0x20000;
    else {
      v12 = 0;
    }
    if ((a0->field_0x48 & 4) || (!(a0->field_0x48 & 0x200)))
      v6 = 0xffffff9c;
    else {
      v6 = a0->field_0x2c;
    }
    v12 = sub_22c33(v6,*(char **)(v21 + 0x30),v12,&v11);
    *(unsigned long *)(v21 + 0x18) = v12;
    if (!*(long *)(v21 + 0x18)) {
      if (a1 != 3)
        return NULL;
      *(unsigned short *)(v21 + 0x68) = 4;
      *(int *)(v21 + 0x40) = *__errno_location();
      return NULL;
    }
    if (*(short *)(v21 + 0x68) != 0xb) { // branch-flip
      if (a0->field_0x48 & 0x100) {
        sub_1d5ab(a0,v21);
        sub_1f5cd(a0,v21,0);
        v2 = sub_1d499(a0,v21);
        if (v2 != '\x01') {
          *__errno_location() = 0xc;
          return NULL;
        }
      }
    }
    else {
      v5 = sub_1f5cd(a0,v21,0);
      *(unsigned short *)(v21 + 0x68) = v5;
    }
  }
  v23 = (a0->field_0x40) ? 0xffffffffffffffff : 100000; // branch-flip
  if (v15)
    v1 = 1;
  else {
    v3 = ((a1 != 2) && ((((!(a0->field_0x48 & 8) || (!(a0->field_0x48 & 0x10))) || (a0->field_0x48 & 0x20)) || ((*(long *)(v21 + 0x80) != 2 || (v7 = sub_1e246(v21,v11), !v7))))));
    if ((v3) || (v1 = 0, a1 == 3)) {
      if (a0->field_0x48 & 0x200)
        v11 = sub_27bb0(v11,0x406,3);
      if ((0 <= v11) && (v7 = sub_1fc3d(a0,v21,v11,NULL), !v7))
        v1 = 1;
      else {
        if ((v3) && (a1 == 3))
          *(int *)(v21 + 0x40) = *__errno_location();
        *(unsigned short *)(v21 + 0x6a) = *(unsigned short *)(v21 + 0x6a) | 1;
        v1 = 0;
        closedir(*(DIR **)(v21 + 0x18));
        *(unsigned long *)(v21 + 0x18) = 0;
        if ((a0->field_0x48 & 0x200) && (0 <= v11))
          close(v11);
        *(unsigned long *)(v21 + 0x18) = 0;
      }
    }
  }
  if (*(char *)(*(long *)(v21 + 0x48) + -1 + *(long *)(v21 + 0x38)) != '/') // branch-flip
    v8 = *(long *)(v21 + 0x48);
  else {
    v8 = *(long *)(v21 + 0x48) + -1;
  }
  if (a0->field_0x48 & 4) { // branch-flip
    v9 = (char *)(v8 + a0->field_0x20);
    v20 = &v9[1];
    *v9 = 0x2f;
  }
  else {
    v20 = NULL;
  }
  v24 = v8 + 1;
  v19 = a0->field_0x30 - v24;
  v25 = *(long *)(v21 + 0x58) + 1;
  v3 = 0;
  v13 = NULL;
  v18 = NULL;
  v14 = 0;
  do {
    do {
      if (!*(long *)(v21 + 0x18)) {
label_1f419:
        if (*(long *)(v21 + 0x18)) {
          closedir(*(DIR **)(v21 + 0x18));
          *(unsigned long *)(v21 + 0x18) = 0;
        }
        goto label_1f448;
      }
      *__errno_location() = 0;
      v26 = readdir(*(DIR **)(v21 + 0x18));
      if (!v26) {
        if (*__errno_location()) {
          *(int *)(v21 + 0x40) = *__errno_location();
          if ((v15) || (v14))
            v5 = 7;
          else {
            v5 = 4;
          }
          *(unsigned short *)(v21 + 0x68) = v5;
        }
        goto label_1f419;
      }
    } while (((!(a0->field_0x48 & 0x20)) && (v26->field_0x13 == '.')) && ((!v26->field_0x14 || ((v26->field_0x14 == '.' && (!v26->field_0x15))))));
    v27 = strlen(&v26->field_0x13);
    v10 = (void *)sub_1f908(a0,&v26->field_0x13,v27);
    if (!v10) {
label_1f16a:
      v17 = *__errno_location();
      free(v10);
      sub_1f9be(v13);
      closedir(*(DIR **)(v21 + 0x18));
      *(unsigned long *)(v21 + 0x18) = 0;
      *(unsigned short *)(v21 + 0x68) = 7;
      a0->field_0x48 = a0->field_0x48 | 0x2000;
      *__errno_location() = v17;
      return NULL;
    }
    if (v19 <= v27) {
      v28 = a0->field_0x20;
      v8 = v24 + v27;
      if (sub_1f9ff(a0,v8 + 1) != '\x01') goto label_1f16a;
      if ((v28 != a0->field_0x20) && (v3 = 1, a0->field_0x48 & 4))
        v20 = (char *)(v24 + a0->field_0x20);
      v19 = a0->field_0x30 - v24;
    }
    v29 = v27 + v24;
    if (v29 < v24) {
      free(v10);
      sub_1f9be(v13);
      closedir(*(DIR **)(v21 + 0x18));
      *(unsigned long *)(v21 + 0x18) = 0;
      *(unsigned short *)(v21 + 0x68) = 7;
      a0->field_0x48 = a0->field_0x48 | 0x2000;
      *__errno_location() = 0x24;
      return NULL;
    }
    *(long *)((long)v10 + 0x58) = v25;
    *(long *)((long)v10 + 8) = a0->field_0x0;
    *(unsigned long *)((long)v10 + 0x48) = v29;
    *(unsigned long *)((long)v10 + 0x78) = *(unsigned long *)v26;
    if (a0->field_0x48 & 4) { // branch-flip
      *(unsigned long *)((long)v10 + 0x30) = *(unsigned long *)((long)v10 + 0x38);
      memmove(v20,(void *)((long)v10 + 0x100),*(long *)((long)v10 + 0x60) + 1);
    }
    else {
      *(long *)((long)v10 + 0x30) = (long)v10 + 0x100;
    }
    if ((a0->field_0x40) && (!(a0->field_0x48 & 0x400)))
      *(unsigned short *)((long)v10 + 0x68) = sub_1f5cd(a0,v10,0);
    else {
      v4 = (((a0->field_0x48 & 8) && ((v26->field_0x12 && (v26->field_0x12 != '\x04')))) && ((a0->field_0x48 & 0x10 || (v26->field_0x12 != '\n'))));
      *(unsigned short *)((long)v10 + 0x68) = 0xb;
      sub_1eb7d((long)v10 + 0x70,(unsigned char)v26->field_0x12);
      sub_1d777(v10,!v4);
    }
    *(unsigned long *)((long)v10 + 0x10) = 0;
    if (v13) // branch-flip
      *(void **)((long)v18 + 0x10) = v10;
    else {
      v13 = v10;
    }
    v18 = v10;
    if ((v14 == 10000) && (!a0->field_0x40))
      v16 = sub_1e1f0(v21,v11);
    v14 += 1;
  } while (v14 < v23);
label_1f448:
  if (v3)
    sub_1fad0(a0,v13);
  if (a0->field_0x48 & 4) {
    if ((v24 == a0->field_0x30) || (!v14))
      v20 = &v20[-1];
    *v20 = 0;
  }
  if (((!v15) && (v1)) && ((a1 == 1 || (!v14)))) {
    if (*(long *)(v21 + 0x58)) // branch-flip
      v7 = sub_1fc3d(a0,*(long *)(v21 + 8),0xffffffff,"..");
    else {
      v7 = sub_1d84f(a0);
    }
    if (v7) {
      *(unsigned short *)(v21 + 0x68) = 7;
      a0->field_0x48 = a0->field_0x48 | 0x2000;
      sub_1f9be(v13);
      return NULL;
    }
  }
  if (v14) {
    if (v16) {
      a0->field_0x40 = (long)sub_1eb29;
      v13 = (void *)sub_1f79a(a0,v13,v14);
      a0->field_0x40 = 0;
    }
    if ((a0->field_0x40) && (2 <= v14))
      v13 = (void *)sub_1f79a(a0,v13,v14);
    return v13;
  }
  if (((a1 == 3) && (*(short *)(v21 + 0x68) != 4)) && (*(short *)(v21 + 0x68) != 7))
    *(unsigned short *)(v21 + 0x68) = 6;
  sub_1f9be(v13);
  return NULL;
}

// Function: sub_1f5cd @ 0x1f5cd
unsigned long sub_1f5cd(long a0,long a1,bool a2) // early-return x7, ternary
{
  stat *v1;
  int v2;
  char *v3;
  int v4;
  bool v5; // stack - 0x34
  
  v1 = (stat *)(a1 + 0x70);
  if ((*(unsigned int *)(a0 + 0x48) & 2) || ((v5 = a2, *(unsigned int *)(a0 + 0x48) & 1 && (!*(long *)(a1 + 0x58)))))
    v5 = 1;
  v4 = (v5) ? 0 : 0x100; // branch-flip
  v3 = *(char **)(a1 + 0x30);
  v2 = *(int *)(a0 + 0x2c);
  if (fstatat(v2,v3,v1,v4) <= -1) {
    if (((v5) && (*__errno_location() == 2)) && (v3 = *(char **)(a1 + 0x30), v4 = *(int *)(a0 + 0x2c), 0 <= fstatat(v4,v3,v1,0x100))) {
      *__errno_location() = 0;
      return 0xd;
    }
    *(int *)(a1 + 0x40) = *__errno_location();
    memset(v1,0,0x90);
    return 10;
  }
  if ((*(unsigned int *)(a1 + 0x88) & 0xf000) == 0x4000) {
    if ((*(char *)(a1 + 0x100) == '.') && ((!*(char *)(a1 + 0x101) || ((*(char *)(a1 + 0x101) == '.' && (!*(char *)(a1 + 0x102))))))) {
      if (!*(long *)(a1 + 0x58))
        return 1;
      return 5;
    }
    return 1;
  }
  if ((*(unsigned int *)(a1 + 0x88) & 0xf000) == 0xa000)
    return 0xc;
  if ((*(unsigned int *)(a1 + 0x88) & 0xf000) == 0x8000)
    return 8;
  return 3;
}

// Function: sub_1f755 @ 0x1f755
void sub_1f755(long *a0,unsigned long a1)
{
  (**(void **)(*(long *)(*a0 + 0x50) + 0x40))(a0,a1);
}

// Function: sub_1f79a @ 0x1f79a
long sub_1f79a(long a0,long a1,unsigned long a2)
{
  long *v1;
  unsigned long v2;
  long v3;
  void *v4; // rax
  long *v5;
  
  v2 = *(unsigned long *)(a0 + 0x40);
  if (*(unsigned long *)(a0 + 0x38) < a2) {
    *(unsigned long *)(a0 + 0x38) = a2 + 0x28;
    if ((0x2000000000000000 <= *(unsigned long *)(a0 + 0x38)) || (v4 = realloc(*(void **)(a0 + 0x10),*(long *)(a0 + 0x38) * 8), !v4)) {
      free(*(void **)(a0 + 0x10));
      *(unsigned long *)(a0 + 0x10) = 0;
      *(unsigned long *)(a0 + 0x38) = 0;
      return a1;
    }
    *(void **)(a0 + 0x10) = v4;
  }
  v5 = *(long **)(a0 + 0x10);
  while (a1) {
    v1 = &v5[1];
    *v5 = a1;
    a1 = *(long *)(a1 + 0x10);
    v5 = v1;
  }
  qsort(*(unsigned long *)(a0 + 0x10),a2,8,v2);
  v5 = *(long **)(a0 + 0x10);
  v3 = *v5;
  while (a2 = a2 - 1, a2) {
    *(long *)(*v5 + 0x10) = v5[1];
    v5 = &v5[1];
  }
  *(unsigned long *)(*v5 + 0x10) = 0;
  return v3;
}

// Function: sub_1f908 @ 0x1f908
void * sub_1f908(long a0,void *a1,unsigned long a2) // early-return
{
  void *v1; // rax
  
  v1 = malloc(a2 + 0x108 & 0xfffffffffffffff8);
  if (!v1)
    return NULL;
  memcpy((void *)((long)v1 + 0x100),a1,a2);
  *(char *)((long)v1 + a2 + 0x100) = 0;
  *(unsigned long *)((long)v1 + 0x60) = a2;
  *(long *)((long)v1 + 0x50) = a0;
  *(unsigned long *)((long)v1 + 0x38) = *(unsigned long *)(a0 + 0x20);
  *(unsigned int *)((long)v1 + 0x40) = 0;
  *(unsigned long *)((long)v1 + 0x18) = 0;
  *(unsigned short *)((long)v1 + 0x6a) = 0;
  *(unsigned short *)((long)v1 + 0x6c) = 3;
  *(unsigned long *)((long)v1 + 0x20) = 0;
  *(unsigned long *)((long)v1 + 0x28) = 0;
  return v1;
}

// Function: sub_1f9be @ 0x1f9be
void sub_1f9be(void *a0)
{
  void *v1;
  
  while (a0) {
    v1 = *(void **)((long)a0 + 0x10);
    if (*(long *)((long)a0 + 0x18))
      closedir(*(DIR **)((long)a0 + 0x18));
    free(a0);
    a0 = v1;
  }
}

// Function: sub_1f9ff @ 0x1f9ff
unsigned long sub_1f9ff(long a0,long a1) // return-dupe x2
{
  unsigned long v1; // rax
  void *v2; // rax
  
  v1 = a1 + *(long *)(a0 + 0x30) + 0x100;
  if (v1 < *(unsigned long *)(a0 + 0x30)) {
    free(*(void **)(a0 + 0x20));
    *(unsigned long *)(a0 + 0x20) = 0;
    *__errno_location() = 0x24;
    return 0;
  }
  *(unsigned long *)(a0 + 0x30) = v1;
  v2 = realloc(*(void **)(a0 + 0x20),*(unsigned long *)(a0 + 0x30));
  if (!v2) {
    free(*(void **)(a0 + 0x20));
    *(unsigned long *)(a0 + 0x20) = 0;
    return 0;
  }
  *(void **)(a0 + 0x20) = v2;
  return 1;
}

// Function: sub_1fad0 @ 0x1fad0
void sub_1fad0(long a0,long a1) // ternary
{
  long v1;
  long v2; // stack - 0x18
  
  v1 = *(long *)(a0 + 0x20);
  for (v2 = *(long *)(a0 + 8); v2; v2 = *(long *)(v2 + 0x10)) {
    if (*(long *)(v2 + 0x30) != v2 + 0x100)
      *(long *)(v2 + 0x30) = (*(long *)(v2 + 0x30) - *(long *)(v2 + 0x38)) + v1;
    *(long *)(v2 + 0x38) = v1;
  }
  v2 = a1;
  while (0 <= *(long *)(v2 + 0x58)) {
    if (*(long *)(v2 + 0x30) != v2 + 0x100)
      *(long *)(v2 + 0x30) = (*(long *)(v2 + 0x30) - *(long *)(v2 + 0x38)) + v1;
    *(long *)(v2 + 0x38) = v1;
    v2 = (*(long *)(v2 + 0x10)) ? *(long *)(v2 + 0x10) : *(long *)(v2 + 8); // branch-flip
  }
}

// Function: sub_1fbe3 @ 0x1fbe3
long sub_1fbe3(long *a0)
{
  unsigned long v1; // rax
  long *v2; // stack - 0x20
  unsigned long v3; // stack - 0x18
  
  v3 = 0;
  for (v2 = a0; *v2; v2 = &v2[1]) {
    v1 = strlen((char *)*v2);
    if (v3 < v1)
      v3 = v1;
  }
  return v3 + 1;
}

// Function: sub_1fc3d @ 0x1fc3d
int sub_1fc3d(long a0,long a1,int a2,char *a3) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_1fc3d
{
  bool v1; // al
  int v2;
  stat v3; // stack - 0xa8
  char *v4; // stack - 0xe8
  int v5; // stack - 0xdc
  int v6; // stack - 0xb8
  int v7; // stack - 0xb4
  
  v1 = ((a3) && (!strcmp(a3,"..")));
  if (*(unsigned int *)(a0 + 0x48) & 4) {
    if ((*(unsigned int *)(a0 + 0x48) & 0x200) && (0 <= a2))
      close(a2);
    return 0;
  }
  v4 = a3;
  v5 = a2;
  if ((((a2 <= -1) && (v1)) && (*(unsigned int *)(a0 + 0x48) & 0x200)) && ((sub_21a13(a0 + 0x60) != '\x01' && (v2 = sub_21ab5(a0 + 0x60), 0 <= v2)))) {
    v5 = v2;
    v4 = NULL;
  }
  v7 = v5;
  if (v5 <= -1) {
    v7 = sub_1d8fd(a0,v4);
    if (v7 <= -1)
      return -1;
  }
  if ((*(unsigned int *)(a0 + 0x48) & 2) || ((v4 && (!strcmp(v4,".."))))) {
    if (fstat(v7,&v3)) {
      v6 = -1;
      goto label_1fec0;
    }
    if ((*(long *)(a1 + 0x70) != v3._0_8_) || (*(long *)(a1 + 0x78) != v3._8_8_)) {
      *__errno_location() = 2;
      v6 = -1;
      goto label_1fec0;
    }
  }
  if (*(unsigned int *)(a0 + 0x48) & 0x200) {
    sub_1d7bf(a0,v7,!v1);
    return 0;
  }
  v6 = fchdir(v7);
label_1fec0:
  if (v5 <= -1) {
    v2 = *__errno_location();
    close(v7);
    *__errno_location() = v2;
  }
  return v6;
}

// Function: sub_1ff0c @ 0x1ff0c
unsigned long sub_1ff0c(void)
{
  return __progname;
}

// Function: sub_1ff1d @ 0x1ff1d
unsigned long sub_1ff1d(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}

// Function: sub_1ff33 @ 0x1ff33
unsigned long sub_1ff33(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}

// Function: sub_1ff49 @ 0x1ff49
unsigned long sub_1ff49(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}

// Function: sub_1ff5f @ 0x1ff5f
unsigned long sub_1ff5f(struct_33 *a0)
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

// Function: sub_1ffe1 @ 0x1ffe1
unsigned long sub_1ffe1(struct_33 *a0) // return-dupe
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

// Function: sub_2007f @ 0x2007f
void sub_2007f(unsigned long a0,FILE *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_1ff49(a0);
  v2 = sub_1ff1d(a0);
  v3 = sub_1ff33(a0);
  v4 = sub_1ff5f(a0);
  fprintf(a1,"# entries:         %lu\n",v1);
  fprintf(a1,"# buckets:         %lu\n",v2);
  fprintf(a1,"# buckets used:    %lu (%.2f%%)\n",v3,(dat_2b298 * (double)v3) / (double)v2);
  fprintf(a1,"max bucket length: %lu\n",v4);
}

// Function: sub_201c6 @ 0x201c6
long sub_201c6(struct_0 *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*a0->field_0x30)(a1,a0->field_0x10);
  if ((unsigned long)a0->field_0x10 <= v1)
    abort(); // no-return
  return v1 * 0x10 + a0->field_0x0;
}

// Function: sub_20221 @ 0x20221
long sub_20221(long a0,long a1) // return-dupe
{
  void *v1;
  long v2;
  long *v3; // stack - 0x18
  
  v3 = (long *)sub_201c6(a0,a1);
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

// Function: sub_202b8 @ 0x202b8
long sub_202b8(struct_33 *a0) // early-return
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

// Function: sub_20318 @ 0x20318
long sub_20318(long a0,long a1) // early-return
{
  long *v1; // stack - 0x18
  long *v2; // stack - 0x10
  
  v1 = (long *)sub_201c6(a0,a1);
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

// Function: sub_203b6 @ 0x203b6
unsigned long sub_203b6(struct_33 *a0,long a1,unsigned long a2)
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

// Function: sub_20456 @ 0x20456
long sub_20456(struct_33 *a0,void *a1,unsigned long a2)
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

// Function: sub_204ee @ 0x204ee
unsigned long sub_204ee(unsigned char *a0,unsigned long a1)
{
  unsigned char *v1; // stack - 0x20
  unsigned long v2; // stack - 0x10
  
  v2 = 0;
  for (v1 = a0; *v1; v1 = &v1[1]) {
    v2 = ((unsigned long)*v1 + v2 * 0x1f) % a1;
  }
  return v2;
}

// Function: sub_20548 @ 0x20548
unsigned long sub_20548(unsigned long a0)
{
  unsigned long v1; // stack - 0x18
  unsigned long v2; // stack - 0x10
  
  v2 = 9;
  for (v1 = 3; (v2 < a0 && (a0 % v1)); v1 = v1 + 2) {
    v2 += (v1 + 1) * 4;
  }
  return CONCAT71((undefined7)(a0 % v1 >> 8),a0 % v1 != 0);
}

// Function: sub_205b7 @ 0x205b7
unsigned long sub_205b7(unsigned long a0)
{
  unsigned long v1; // stack - 0x10
  
  v1 = a0;
  if (a0 <= 9)
    v1 = 10;
  v1 |= 1;
  while( true ) {
    if (v1 == 0xffffffffffffffff)
      return 0xffffffffffffffff;
    if (sub_20548(v1) == '\x01') break;
    v1 += 2;
  }
  return v1;
}

// Function: sub_20602 @ 0x20602
void sub_20602(struct_34 *a0)
{
  a0->field_0x0 = 0;
  a0->field_0x4 = dat_2b2a0;
  a0->field_0x8 = dat_2b2a4;
  a0->field_0xc = dat_2b2a8;
  a0->field_0x10 = 0;
}

// Function: sub_20658 @ 0x20658
unsigned long sub_20658(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_278e5(a0,3) % a1;
}

// Function: sub_20693 @ 0x20693
unsigned long sub_20693(long a0,long a1)
{
  return CONCAT71((undefined7)((unsigned long)a0 >> 8),a0 == a1);
}

// Function: sub_206b0 @ 0x206b0
unsigned long sub_206b0(long a0) // return-dupe x2
{
  float *v1;
  
  v1 = *(float **)(a0 + 0x28);
  if (v1 == (float *)0x2b210)
    return 1;
  if ((((dat_2b2ac < v1[2]) && (v1[2] < dat_2b2a0 - dat_2b2ac)) && (dat_2b2ac + dat_2b2a0 < v1[3])) && (((0.0 <= *v1 && (*v1 + dat_2b2ac < v1[1])) && ((v1[1] <= dat_2b2a0 && (*v1 + dat_2b2ac < v1[2]))))))
    return 1;
  *(unsigned long *)(a0 + 0x28) = 0x2b210;
  return 0;
}

// Function: sub_207bd @ 0x207bd
unsigned long sub_207bd(unsigned long a0,long a1) // early-return x2
{
  unsigned long v1; // rax
  float v2; // xmm0_da
  unsigned long v3; // stack - 0x20
  
  v3 = a0;
  if (*(char *)(a1 + 0x10) != '\x01') {
    v2 = (float)a0 / *(float *)(a1 + 8);
    if (dat_2b2b0 <= v2)
      return 0;
    if (dat_2b2b4 <= v2)
      v3 = (long)(v2 - dat_2b2b4) ^ 0x8000000000000000;
    else {
      v3 = (unsigned long)v2;
    }
  }
  v1 = sub_205b7(v3);
  if (!((long)(v1 << 3) <= -1 || v1 >> 0x3d))
    return v1;
  return 0;
}

// Function: sub_208d6 @ 0x208d6
long * sub_208d6(unsigned long a0,long a1,void *a2,void *a3,long a4) // early-return x2
{
  unsigned long v1;
  long *v2; // rax
  void *v3; // stack - 0x38
  void *v4; // stack - 0x30
  long v5; // stack - 0x28
  
  v4 = a2;
  if (!a2)
    v4 = sub_20658;
  v3 = a3;
  if (!a3)
    v3 = sub_20693;
  v2 = malloc(0x50);
  if (!v2)
    return NULL;
  v5 = a1;
  if (!a1)
    v5 = 0x2b210;
  v2[5] = v5;
  if (sub_206b0(v2) == '\x01') {
    v2[2] = sub_207bd(a0,v5);
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

// Function: sub_20a4b @ 0x20a4b
void sub_20a4b(struct_35 *a0)
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

// Function: sub_20b56 @ 0x20b56
void sub_20b56(struct_8 *a0)
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

// Function: sub_20c85 @ 0x20c85
void * sub_20c85(long a0)
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

// Function: sub_20cd4 @ 0x20cd4
void sub_20cd4(long a0,struct_1 *a1)
{
  a1->field_0x0 = 0;
  a1->field_0x8 = *(unsigned long *)(a0 + 0x48);
  *(struct_1 **)(a0 + 0x48) = a1;
}

// Function: sub_20d0e @ 0x20d0e
long sub_20d0e(long a0,long a1,unsigned long *a2,bool a3) // early-return
{
  void *v1;
  long v2;
  struct_1 *v3;
  long v4;
  unsigned long v5;
  long *v6; // stack - 0x38
  
  v6 = (long *)sub_201c6(a0,a1);
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
      v3 = (struct_1 *)v6[1];
      v6[1] = v3->field_0x8;
      sub_20cd4(a0,v3);
    }
    return v2;
  }
  v2 = *v6;
  if (a3) {
    if (v6[1]) { // branch-flip
      v3 = (struct_1 *)v6[1];
      v4 = v3->field_0x8;
      *v6 = v3->field_0x0;
      v6[1] = v4;
      sub_20cd4(a0,v3);
    }
    else {
      *v6 = 0;
    }
  }
  return v2;
}

// Function: sub_20ea7 @ 0x20ea7
unsigned long sub_20ea7(long a0,struct_33 *a1,bool a2)
{
  long *v1;
  long v2;
  struct_3 *v3; // rax
  struct_3 *v4; // rax
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
        v3 = (struct_3 *)sub_201c6(a0,v2);
        v1 = (long *)v6[1];
        if (v3->field_0x0) { // branch-flip
          v6[1] = v3->field_0x8;
          v3->field_0x8 = (long)v6;
        }
        else {
          v3->field_0x0 = v2;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
          sub_20cd4(a0,v6);
        }
        v6 = v1;
      }
      v2 = *v5;
      v5[1] = 0;
      if (!a2) {
        v3 = (struct_3 *)sub_201c6(a0,v2);
        if (v3->field_0x0) { // branch-flip
          v4 = (struct_3 *)sub_20c85(a0);
          if (!v4)
            return 0;
          v4->field_0x0 = v2;
          v4->field_0x8 = v3->field_0x8;
          v3->field_0x8 = (long)v4;
        }
        else {
          v3->field_0x0 = v2;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
        }
        *v5 = 0;
        a1->field_0x18 = a1->field_0x18 + -1;
      }
    }
    v5 = &v5[2];
  } while( true );
}

// Function: sub_2107b @ 0x2107b
unsigned long sub_2107b(struct_4 *a0,unsigned long a1) // return-dupe x4
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
  
  v3 = sub_207bd(a1,a0->field_0x28);
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
  if (sub_20ea7(&v1,a0,0)) {
    free(a0->field_0x0);
    a0->field_0x0 = v1;
    a0->field_0x8 = v2;
    a0->field_0x10 = v3;
    a0->field_0x18 = v4;
    a0->field_0x48 = v10;
    return 1;
  }
  a0->field_0x48 = v10;
  if ((sub_20ea7(a0,&v1,1) == '\x01') && (sub_20ea7(a0,&v1,0) == '\x01')) {
    free(v1);
    return 0;
  }
  abort(); // no-return
}

// Function: sub_212a6 @ 0x212a6
unsigned long sub_212a6(long a0,long a1,long *a2) // return-dupe x2, return-dupe, ternary x2
{
  unsigned long v1; // rax
  long *v2; // rax
  long *v3; // stack - 0x30
  float v4; // xmm0_da
  long v5; // stack - 0x28
  long v6; // stack - 0x20
  
  if (!a1)
    abort(); // no-return
  v5 = sub_20d0e(a0,a1,&v3,0);
  if (v5) {
    if (!a2)
      return 0;
    *a2 = v5;
    return 0;
  }
  if (((float)*(unsigned long *)(a0 + 0x10) * *(float *)(*(long *)(a0 + 0x28) + 8) < (float)*(unsigned long *)(a0 + 0x18)) && (sub_206b0(a0), (float)*(unsigned long *)(a0 + 0x10) * *(float *)(*(long *)(a0 + 0x28) + 8) < (float)*(unsigned long *)(a0 + 0x18))) {
    v6 = *(long *)(a0 + 0x28);
    v4 = (*(char *)(v6 + 0x10)) ? (float)*(unsigned long *)(a0 + 0x10) * *(float *)(v6 + 0xc) : *(float *)(v6 + 8) * (*(float *)(v6 + 0xc) * (float)*(unsigned long *)(a0 + 0x10)); // branch-flip
    if (dat_2b2b0 <= v4)
      return 0xffffffff;
    v1 = (dat_2b2b4 <= v4) ? (long)(v4 - dat_2b2b4) ^ 0x8000000000000000 : (unsigned long)v4;
    if (sub_2107b(a0,v1) != '\x01')
      return 0xffffffff;
    if (sub_20d0e(a0,a1,&v3,0))
      abort(); // no-return
  }
  if (!*v3) {
    *v3 = a1;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
    return 1;
  }
  v2 = (long *)sub_20c85(a0);
  if (v2) {
    *v2 = a1;
    v2[1] = v3[1];
    v3[1] = (long)v2;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    return 1;
  }
  return 0xffffffff;
}

// Function: sub_21617 @ 0x21617
unsigned long sub_21617(long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  
  v1 = sub_212a6(a0,a1,&v3);
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

// Function: sub_21687 @ 0x21687
long sub_21687(long a0,long a1) // early-return, ternary x2
{
  char v1; // al
  long *v2; // stack - 0x40
  float v3; // xmm0_da
  void *v4; // stack - 0x38
  long v5; // stack - 0x30
  long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  void *v8; // stack - 0x18
  
  v5 = sub_20d0e(a0,a1,&v2,1);
  if (!v5)
    return 0;
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + -1;
  if (((!*v2) && (*(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + -1, (float)*(unsigned long *)(a0 + 0x18) < (float)*(unsigned long *)(a0 + 0x10) * **(float **)(a0 + 0x28))) && (sub_206b0(a0), (float)*(unsigned long *)(a0 + 0x18) < (float)*(unsigned long *)(a0 + 0x10) * **(float **)(a0 + 0x28))) {
    v6 = *(long *)(a0 + 0x28);
    if (*(char *)(v6 + 0x10)) { // branch-flip
      v3 = (float)*(unsigned long *)(a0 + 0x10) * *(float *)(v6 + 4);
      v7 = (dat_2b2b4 <= v3) ? (long)(v3 - dat_2b2b4) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    else {
      v3 = *(float *)(v6 + 8) * (*(float *)(v6 + 4) * (float)*(unsigned long *)(a0 + 0x10));
      v7 = (dat_2b2b4 <= v3) ? (long)(v3 - dat_2b2b4) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    v1 = sub_2107b(a0,v7);
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

// Function: sub_2198d @ 0x2198d
void sub_2198d(long a0,long a1)
{
  sub_21687(a0,a1);
}

// Function: sub_219b6 @ 0x219b6
void sub_219b6(long a0,unsigned int a1)
{
  int v1; // stack - 0xc
  
  *(char *)(a0 + 0x1c) = 1;
  *(unsigned int *)(a0 + 0x14) = 0;
  *(unsigned int *)(a0 + 0x18) = 0;
  for (v1 = 0; v1 <= 3; v1 = v1 + 1) {
    *(unsigned int *)(a0 + (long)v1 * 4) = a1;
  }
  *(unsigned int *)(a0 + 0x10) = a1;
}

// Function: sub_21a13 @ 0x21a13
char sub_21a13(long a0)
{
  return *(char *)(a0 + 0x1c);
}

// Function: sub_21a29 @ 0x21a29
unsigned int sub_21a29(long a0,unsigned int a1)
{
  unsigned int v1;
  unsigned int v2; // eax
  
  v2 = (unsigned int)(*(unsigned char *)(a0 + 0x1c) ^ 1) + *(int *)(a0 + 0x14) & 3;
  v1 = *(unsigned int *)(a0 + (unsigned long)v2 * 4);
  *(unsigned int *)(a0 + (unsigned long)v2 * 4) = a1;
  *(unsigned int *)(a0 + 0x14) = v2;
  if (v2 == *(unsigned int *)(a0 + 0x18))
    *(unsigned int *)(a0 + 0x18) = (unsigned int)(*(unsigned char *)(a0 + 0x1c) ^ 1) + *(int *)(a0 + 0x18) & 3;
  *(char *)(a0 + 0x1c) = 0;
  return v1;
}

// Function: sub_21ab5 @ 0x21ab5
unsigned int sub_21ab5(long a0)
{
  unsigned int v1;
  
  if (sub_21a13(a0))
    abort(); // no-return
  v1 = *(unsigned int *)(a0 + (unsigned long)*(unsigned int *)(a0 + 0x14) * 4);
  *(unsigned int *)(a0 + (unsigned long)*(unsigned int *)(a0 + 0x14) * 4) = *(unsigned int *)(a0 + 0x10);
  if (*(int *)(a0 + 0x14) != *(int *)(a0 + 0x18)) // branch-flip
    *(unsigned int *)(a0 + 0x14) = *(int *)(a0 + 0x14) + 3U & 3;
  else {
    *(char *)(a0 + 0x1c) = 1;
  }
  return v1;
}

// Function: sub_21b3b @ 0x21b3b
void sub_21b3b(char *a0,unsigned long a1,mbstate_t *a2)
{
  mbstate_t *v1; // stack - 0x20
  
  v1 = a2;
  if (!a2)
    v1 = (mbstate_t *)0x43b18;
  sub_21b80(NULL,a0,a1,v1);
}

// Function: sub_21b80 @ 0x21b80
unsigned long sub_21b80(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3) // early-return
{
  long v1;
  unsigned int *v2; // stack - 0x30
  unsigned long v3; // stack - 0x18
  
  v2 = a0;
  if (!a0)
    v2 = &v1;
  v3 = mbrtowc(v2,a1,a2,a3);
  if (((0xfffffffffffffffe <= v3) && (a2)) && (sub_28192(0) != '\x01')) {
    *v2 = (unsigned int)(unsigned char)*a1;
    return 1;
  }
  return v3;
}

// Function: sub_21c2a @ 0x21c2a
int sub_21c2a(unsigned char *a0,unsigned char *a1) // early-return, ternary x2
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
      sub_22011(v8);
      if ((v15 != '\x01') || (v16))
        v1 = 1;
      else {
        v1 = 0;
      }
      if (!v1) break;
      sub_22011(v9);
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
    sub_22011(v8);
    if ((v15 != '\x01') || (v16))
      v1 = 1;
    else {
      v1 = 0;
    }
    if (v1)
      return 1;
    sub_22011(v9);
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

// Function: sub_22011 @ 0x22011
void sub_22011(struct_6 *a0)
{
  char v1;
  char *v2;
  unsigned long v3; // rax
  
  if (a0->field_0xc)
    return;
  if (!a0->field_0x0) {
    v1 = *a0->field_0x10;
    if (sub_2844c((int)v1)) {
      a0->field_0x18 = 1;
      a0->field_0x24 = (int)*a0->field_0x10;
      a0->field_0x20 = '\x01';
      goto label_22217;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x1[3]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    a0->field_0x0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v3 = sub_260fc(a0->field_0x10,v3);
  v2 = a0->field_0x10;
  a0->field_0x18 = sub_21b80(&a0->field_0x24,v2,v3,(mbstate_t *)&a0->field_0x1[3]);
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
label_22217:
  a0->field_0xc = '\x01';
  return;
}

// Function: sub_22228 @ 0x22228
void sub_22228(long a0,long a1)
{
  *(long *)(a0 + 0x10) = *(long *)(a0 + 0x10) + a1;
}

// Function: sub_22252 @ 0x22252
void sub_22252(struct_38 *a0,struct_38 *a1)
{
  a0->field_0x0 = a1->field_0x0;
  if (a0->field_0x0) // branch-flip
    memcpy(&a0->field_0x1[3],&a1->field_0x1[3],8);
  else {
    memset(&a0->field_0x1[3],0,8);
  }
  a0->field_0xc = a1->field_0xc;
  sub_283a6((struct_36 *)&a0[1].field_0x1[2],(struct_37 *)&a1[1].field_0x1[2]);
}

// Function: sub_222e7 @ 0x222e7
unsigned long * sub_222e7(unsigned long *a0,unsigned char a1,unsigned char a2,unsigned long a3) // early-return
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // stack - 0x70
  unsigned long *v6; // stack - 0x48
  unsigned long *v7; // stack - 0x40
  unsigned long *v8; // stack - 0x38
  
  if (a1 != a2) { // branch-flip
    v5 = a3;
    for (v7 = a0; (v5 && ((unsigned long)v7 & 7)); v7 = (unsigned long *)((long)v7 + 1)) {
      if ((a1 == (unsigned char)*v7) || (a2 == (unsigned char)*v7))
        return v7;
      v5 -= 1;
    }
    v8 = v7;
    v1 = (long)(int)(unsigned int)CONCAT11(a1,a1) | (long)(int)(unsigned int)CONCAT11(a1,a1) << 0x10;
    v2 = (long)(int)(unsigned int)CONCAT11(a2,a2) | (long)(int)(unsigned int)CONCAT11(a2,a2) << 0x10;
    while ((8 <= v5 && (v3 = *v8 ^ (v1 | v1 << 0x20), v4 = *v8 ^ (v2 | v2 << 0x20), !((v3 + 0xfefefefefefefeff & ~v3 | ~v4 & v4 + 0xfefefefefefefeff) & 0x8080808080808080)))) {
      v8 = &v8[1];
      v5 -= 8;
    }
    v6 = v8;
    while( true ) {
      if (!v5)
        return NULL;
      if ((a1 == (unsigned char)*v6) || (a2 == (unsigned char)*v6)) break;
      v5 -= 1;
      v6 = (unsigned long *)((long)v6 + 1);
    }
  }
  else {
    v6 = memchr(a0,(unsigned int)a1,a3);
  }
  return v6;
}

// Function: sub_224b1 @ 0x224b1
void sub_224b1(long a0,unsigned long a1) // return-dupe
{
  if (*(unsigned char *)(a0 + 0x50) & 1) {
    (**(void **)(a0 + 0x38))(*(unsigned long *)(a0 + 0x48),a1);
    return;
  }
  (**(void **)(a0 + 0x38))(a1);
}

// Function: sub_22505 @ 0x22505
void sub_22505(long a0,unsigned long a1) // return-dupe
{
  if (*(unsigned char *)(a0 + 0x50) & 1) {
    (**(void **)(a0 + 0x40))(*(unsigned long *)(a0 + 0x48),a1);
    return;
  }
  (**(void **)(a0 + 0x40))(a1);
}

// Function: sub_2255a @ 0x2255a
unsigned long sub_2255a(struct_20 *a0,long a1,long a2)
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
  a0->field_0x8 = (long *)sub_224b1(a0,v1);
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

// Function: _obstack_begin @ 0x22680
int _obstack_begin(obstack *a0,unsigned long a1,unsigned long a2,void *a3,void *a4)
{
  unsigned char v1; // dl
  
  *(void **)&a0->field_0x38 = a3;
  *(void **)&a0->field_0x40 = a4;
  v1 = a0->field_0x50 & 0xfe;
  a0->field_0x50 = v1;
  return sub_2255a(a0,a1,a2);
}

// Function: _obstack_begin_1 @ 0x226df
int _obstack_begin_1(obstack *a0,unsigned long a1,unsigned long a2,void *a3,void *a4,void *a5)
{
  unsigned char v1; // dl
  
  *(void **)&a0->field_0x38 = a3;
  *(void **)&a0->field_0x40 = a4;
  *(void **)&a0->field_0x48 = a5;
  v1 = a0->field_0x50 | 1;
  a0->field_0x50 = v1;
  return sub_2255a(a0,a1,a2);
}

// Function: _obstack_newchunk @ 0x2274e
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
    v5 = (unsigned long *)sub_224b1(a0,v6);
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
    sub_22505(a0,v1);
  }
  *(void **)&a0->field_0x10 = v4;
  *(unsigned long *)&a0->field_0x18 = *(long *)&a0->field_0x10 + v2;
  a0->field_0x50 = a0->field_0x50 & 0xfd;
}

// Function: _obstack_allocated_p @ 0x22945
bool _obstack_allocated_p(long a0,unsigned long *a1)
{
  unsigned long *v1; // stack - 0x18
  
  for (v1 = *(unsigned long **)(a0 + 8); (v1 && ((a1 <= v1 || ((unsigned long *)*v1 < a1)))); v1 = (unsigned long *)v1[1]) {
  }
  return v1 != NULL;
}

// Function: _obstack_free @ 0x229a2
void _obstack_free(obstack *a0,void *a1) // return-dupe
{
  unsigned long *v1;
  unsigned long *v2; // stack - 0x18
  
  v2 = *(unsigned long **)&a0->field_0x8;
  while ((v2 && ((a1 <= v2 || ((void *)*v2 < a1))))) {
    v1 = (unsigned long *)v2[1];
    sub_22505(a0,v2);
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

// Function: _obstack_memory_used @ 0x22a66
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

// Function: sub_22ab0 @ 0x22ab0
void sub_22ab0(void)
{
  fprintf(stderr,"%s\n",gettext("memory exhausted"));
  exit(dat_33080); // no-return
}

// Function: sub_22af5 @ 0x22af5
void sub_22af5(int a0,char *a1,unsigned int a2)
{
  unsigned int v1; // stack - 0xdc
  unsigned int v2; // stack - 0xa0
  
  v1 = 0;
  if (a2 & 0x40)
    v1 = v2;
  sub_26144(openat(a0,a1,a2,(unsigned long)v1));
}

// Function: sub_22c33 @ 0x22c33
DIR * sub_22c33(unsigned int a0,char *a1,unsigned int a2,int *a3) // early-return
{
  int v1;
  int v2; // eax
  DIR *v3; // rax
  
  v2 = sub_22af5(a0,a1,a2 | 0x90900);
  if (v2 <= -1)
    return NULL;
  v3 = fdopendir(v2);
  if (v3) // branch-flip
    *a3 = v2;
  else {
    v1 = *__errno_location();
    close(v2);
    *__errno_location() = v1;
  }
  return v3;
}

// Function: sub_22cc5 @ 0x22cc5
unsigned long sub_22cc5(void *a0)
{
  int v1;
  unsigned long v2; // rax
  
  v1 = *__errno_location();
  if (!a0)
    a0 = (void *)0x43b20;
  v2 = sub_2702a(a0,0x38);
  *__errno_location() = v1;
  return v2;
}

// Function: sub_22d14 @ 0x22d14
unsigned int sub_22d14(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x43b20;
  return *a0;
}

// Function: sub_22d38 @ 0x22d38
void sub_22d38(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x43b20;
  *a0 = a1;
}

// Function: sub_22d63 @ 0x22d63
unsigned int sub_22d63(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int v1; // eax
  unsigned int *v2; // rax
  
  if (!a0)
    a0 = 0x43b20;
  v2 = (unsigned int *)((unsigned long)(a1 >> 5) * 4 + a0 + 8);
  v1 = *v2 >> (a1 & 0x1f) & 1;
  *v2 = *v2 ^ (a2 & 1 ^ v1) << (a1 & 0x1f);
  return v1;
}

// Function: sub_22df3 @ 0x22df3
unsigned int sub_22df3(long a0,unsigned int a1)
{
  unsigned int v1;
  long v2; // stack - 0x20
  
  v2 = a0;
  if (!a0)
    v2 = 0x43b20;
  v1 = *(unsigned int *)(v2 + 4);
  *(unsigned int *)(v2 + 4) = a1;
  return v1;
}

// Function: sub_22e2d @ 0x22e2d
void sub_22e2d(unsigned int *a0,long a1,long a2)
{
  unsigned int *v1; // stack - 0x10
  
  v1 = a0;
  if (!a0)
    v1 = (unsigned int *)0x43b20;
  *v1 = 10;
  if ((a1) && (a2)) {
    *(long *)&v1[10] = a1;
    *(long *)&v1[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_22e8f @ 0x22e8f
unsigned long * sub_22e8f(unsigned long *a0,unsigned int a1)
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

// Function: sub_22f54 @ 0x22f54
char * sub_22f54(char *a0,int a1) // early-return x6
{
  char *v1; // rax
  char *v2; // rax
  
  v1 = gettext(a0);
  if (v1 != a0)
    return v1;
  v2 = (char *)sub_28323();
  if (!sub_11390(v2,"UTF-8")) {
    if (*a0 == '`')
      return "‘";
    return "’";
  }
  if (sub_11390(v2,"GB18030")) {
    if (a1 == 9)
      return "\"";
    return "\'";
  }
  if (*a0 == '`')
    return (char *)0x2b3de;
  return (char *)0x2b3e2;
}

// Function: sub_23019 @ 0x23019
unsigned long sub_23019(long a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_230d3:
  switch(v19) {
    case 0:
      v14 = 0;
      break;
    case 1:
label_23227:
      v14 = 1;
      goto label_2322b;
    case 2:
label_2323a:
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
      goto label_23227;
    case 4:
label_2322b:
      if (!v14)
        v2 = 1;
      goto label_2323a;
    case 5:
label_23115:
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
      goto label_23115;
    case 7:
      v2 = 1;
      v14 = 0;
      break;
    case 8:
    case 9:
    case 10:
      if (v19 != 10) {
        v18 = (char *)sub_22f54("`",v19);
        v17 = (char *)sub_22f54("\'",v19);
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
label_23ed9:
  if (v20 != 0xffffffffffffffff) // branch-flip
    v16 = v24 != v20;
  else {
    v16 = a2[v24] != '\0';
  }
  if (!v16) goto label_23f11;
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
      if (v14) goto label_24055;
      v4 = 1;
    }
  }
  v22 = a2[v24];
  if (v22 == 0x7e) {
label_237f8:
    if (!v24) {
label_23803:
      v16 = 1;
label_23807:
      if (v19 != 2) goto label_23d1e;
label_235e0:
      if (v14) goto label_24055;
    }
    goto label_23d1e;
  }
  if (0x7e < v22) {
label_238e4:
    if (v9 != 1) {
      memset(&v12,0,8);
      v29 = 0;
      v16 = 1;
      if (v20 == 0xffffffffffffffff)
        v20 = strlen(a2);
label_2395e:
      v31 = sub_21b80(&v13,&a2[v24 + v29],v20 - (v24 + v29),&v12);
      if (v31) {
        if (v31 != 0xffffffffffffffff) { // branch-flip
          if (v31 != 0xfffffffffffffffe) goto label_23a0c;
          v16 = 0;
          for (; (v29 + v24 < v20 && (a2[v24 + v29])); v29 = v29 + 1) {
          }
        }
        else {
          v16 = 0;
        }
      }
      goto label_23abb;
    }
    v29 = 1;
    v16 = (*(unsigned short *)((unsigned long)v22 * 2 + *(long *)__ctype_b_loc()) & 0x4000) != 0;
label_23abb:
    if ((v29 <= 1) && ((!v2 || (v16)))) goto label_23d1e;
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
        if (v14) goto label_24055;
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
    goto label_23e44;
  }
  if (v22 == 0x7d) {
label_237c6:
    if (v20 != 0xffffffffffffffff) // branch-flip
      v15 = v20 != 1;
    else {
      v15 = a2[1] != '\0';
    }
    if (v15) goto label_23d1e;
    goto label_237f8;
  }
  if (0x7d < v22) goto label_238e4;
  if (v22 == 0x7c) goto label_23807;
  if (0x7c < v22) goto label_238e4;
  if (v22 == 0x7b) goto label_237c6;
  if (0x7b < v22) goto label_238e4;
  if (0x3f < v22) {
    if ((0x7a < v22) || (v22 <= 0x40)) goto label_238e4;
    v11 = 1L << (v22 + 0xbf & 0x3f);
    if (v11 & 0x3ffffff53ffffff) goto label_238db;
    if (v11 & 0xa4000000) goto label_23807;
    if (!(v11 & 0x8000000)) goto label_238e4;
    if (v19 == 2) {
      if (!v14) goto label_23e44;
      goto label_24055;
    }
    v23 = v22;
    if (((!v2) || (!v14)) || (!v28)) goto label_2379c;
    goto label_23e44;
  }
  switch(v22) {
    case 0:
      if (!v2) {
        if (!(a5 & 1)) goto label_23d1e;
        goto label_23ed4;
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
        goto label_23d1e;
      }
      goto label_24055;
    default:
      goto label_238e4;
    case 7:
      v23 = 0x61;
      break;
    case 8:
      v23 = 0x62;
      break;
    case 9:
      v23 = 0x74;
      goto label_2379c;
    case 10:
      v23 = 0x6e;
      goto label_2379c;
    case 0xb:
      v23 = 0x76;
      break;
    case 0xc:
      v23 = 0x66;
      break;
    case 0xd:
      v23 = 0x72;
label_2379c:
      if ((v19 == 2) && (v14)) goto label_24055;
      break;
    case 0x20:
      goto label_23803;
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
      goto label_23807;
    case 0x23:
      goto label_237f8;
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
label_238db:
      v16 = 1;
      goto label_23d1e;
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
          goto label_23d1e;
        }
        goto label_24055;
      }
      goto label_23d1e;
    case 0x3f:
      if (v19 == 2) goto label_235e0;
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
          goto label_23d1e;
        }
        goto label_24055;
      }
      goto label_23d1e;
    
  }
  if (v2) {
    v22 = v23;
label_23d89:
    if (v14) goto label_24055;
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
label_23d1e:
    if (((((v2) && (v19 != 2)) || (v14)) && ((v8 && (*(unsigned int *)(v8 + (unsigned long)(v22 >> 5) * 4) >> (v22 & 0x1f) & 1)))) || (v4)) goto label_23d89;
  }
label_23e44:
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
label_23ed4:
  v24 += 1;
  goto label_23ed9;
label_23a0c:
  if ((v14) && (v19 == 2)) {
    for (v30 = 1; v30 < v31; v30 = v30 + 1) {
      if (((unsigned int)((int)a2[v24 + v29 + v30] - 0x5bU) <= 0x21) && (0x20000002bU >> ((unsigned char)((int)a2[v24 + v29 + v30] - 0x5bU) & 0x3f) & 1)) goto label_24055;
    }
  }
  if (!iswprint(v13))
    v16 = 0;
  v29 += v31;
  if (mbsinit(&v12)) goto label_23abb;
  goto label_2395e;
label_23f11:
  if (((!v25) && (v19 == 2)) && (v14)) {
label_24055:
    if ((v19 == 2) && (v2))
      v19 = 4;
    return sub_23019(a0,v21,a2,v20,v19,a5 & 0xfffffffd,0,v18,v17);
  }
  if (((v19 == 2) && (!v14)) && (v6)) {
    if (v7)
      return sub_23019(a0,v26,a2,v20,5,a5,v8,v18,v17);
    if ((!v21) && (v26)) {
      v21 = v26;
      v25 = 0;
      goto label_230d3;
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

// Function: sub_240da @ 0x240da
unsigned long sub_240da(long a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  unsigned long v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x43b20;
  v1 = *__errno_location();
  v2 = sub_23019(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *__errno_location() = v1;
  return v2;
}

// Function: sub_24182 @ 0x24182
void sub_24182(char *a0,unsigned long a1,unsigned int *a2)
{
  sub_241b8(a0,a1,NULL,a2);
}

// Function: sub_241b8 @ 0x241b8
long sub_241b8(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  unsigned int v2; // eax
  long v3; // rax
  long v4; // rax
  
  if (!a3)
    a3 = (unsigned int *)0x43b20;
  v1 = *__errno_location();
  v2 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_23019(0,0,a0,a1,*a3,v2,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = sub_26b36(v3 + 1U);
  sub_23019(v4,v3 + 1U,a0,a1,*a3,v2,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *__errno_location() = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_242e4 @ 0x242e4
void sub_242e4(void) // return-dupe
{
  void *v1;
  int v2; // stack - 0x14
  
  v1 = dat_33110;
  for (v2 = 1; v2 < dat_330a0; v2 = v2 + 1) {
    free(*(void **)((long)v1 + (long)v2 * 0x10 + 8));
  }
  if (*(long *)((long)v1 + 8) != 0x43b60) {
    free(*(void **)((long)v1 + 8));
    dat_33100 = 0x100;
    dat_33108 = 0x43b60;
  }
  if (v1 == (void *)0x33100) {
    dat_330a0 = 1;
    return;
  }
  free(v1);
  dat_330a0 = 1;
  dat_33110 = (void *)0x33100;
}

// Function: sub_243a3 @ 0x243a3
void * sub_243a3(int a0,char *a1,unsigned long a2,struct_5 *a3)
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
  v6 = dat_33110;
  if ((0 <= a0) && (a0 <= 0x7ffffffe)) {
    if (dat_330a0 <= a0) {
      v5 = dat_33110 == (unsigned long *)0x33100;
      v4 = (long)dat_330a0;
      if (v5)
        v3 = NULL;
      else {
        v3 = dat_33110;
      }
      dat_33110 = (unsigned long *)sub_26dc6(v3,&v4,(long)((a0 - dat_330a0) + 1),0x7fffffff,0x10);
      v6 = dat_33110;
      if (v5) {
        *dat_33110 = dat_33100;
        dat_33110[1] = dat_33108;
      }
      memset(&dat_33110[(long)dat_330a0 * 2],0,(v4 - dat_330a0) * 0x10);
      dat_330a0 = (int)v4;
    }
    v8 = v6[(long)a0 * 2];
    v7 = (void *)v6[(long)a0 * 2 + 1];
    v2 = a3->field_0x4 | 1;
    v9 = sub_23019(v7,v8,a1,a2,a3->field_0x0,v2,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v8 <= v9) {
      v8 = v9 + 1;
      v6[(long)a0 * 2] = v8;
      if (v7 != (void *)0x43b60)
        free(v7);
      v7 = (void *)sub_26b36(v8);
      v6[(long)a0 * 2 + 1] = v7;
      sub_23019(v7,v8,a1,a2,a3->field_0x0,v2,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *__errno_location() = v1;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_24652 @ 0x24652
void sub_24652(unsigned int a0,char *a1)
{
  sub_243a3(a0,a1,0xffffffffffffffff,(struct_5 *)0x43b20);
}

// Function: sub_24686 @ 0x24686
void sub_24686(unsigned int a0,char *a1,unsigned long a2)
{
  sub_243a3(a0,a1,a2,(struct_5 *)0x43b20);
}

// Function: sub_246b8 @ 0x246b8
void sub_246b8(char *a0)
{
  sub_24652(0,a0);
}

// Function: sub_246db @ 0x246db
void sub_246db(char *a0,unsigned long a1)
{
  sub_24686(0,a0,a1);
}

// Function: sub_24706 @ 0x24706
void sub_24706(unsigned int a0,unsigned int a1,char *a2)
{
  char v1 [56];
  
  sub_22e8f(v1,a1);
  sub_243a3(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_2476e @ 0x2476e
void sub_2476e(unsigned int a0,unsigned int a1,char *a2,unsigned long a3)
{
  char v1 [56];
  
  sub_22e8f(v1,a1);
  sub_243a3(a0,a2,a3,v1);
}

// Function: sub_247d4 @ 0x247d4
void sub_247d4(unsigned int a0,char *a1)
{
  sub_24706(0,a0,a1);
}

// Function: sub_247fc @ 0x247fc
void sub_247fc(unsigned int a0,char *a1,unsigned long a2)
{
  sub_2476e(0,a0,a1,a2);
}

// Function: sub_2482c @ 0x2482c
void sub_2482c(char *a0,unsigned long a1,char a2)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v1 = dat_43b20;
  v2 = dat_43b28;
  v3 = dat_43b30;
  v4 = dat_43b38;
  v5 = dat_43b40;
  v6 = dat_43b48;
  v7 = dat_43b50;
  sub_22d63(&v1,(int)a2,1);
  sub_243a3(0,a0,a1,&v1);
}

// Function: sub_248e7 @ 0x248e7
void sub_248e7(char *a0,char a1)
{
  sub_2482c(a0,0xffffffffffffffff,(int)a1);
}

// Function: sub_24915 @ 0x24915
void sub_24915(char *a0)
{
  sub_248e7(a0,0x3a);
}

// Function: sub_24938 @ 0x24938
void sub_24938(char *a0,unsigned long a1)
{
  sub_2482c(a0,a1,0x3a);
}

// Function: sub_24966 @ 0x24966
void sub_24966(unsigned int a0,unsigned int a1,char *a2)
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
  sub_22e8f(&v1,a1);
  v2 = v1;
  v12 = v3;
  v13 = v4;
  v14 = v5;
  v15 = v6;
  v16 = v7;
  v17 = v8;
  sub_22d63(&v2,0x3a,1);
  sub_243a3(v11,v9,0xffffffffffffffff,&v2);
}

// Function: sub_24a28 @ 0x24a28
void sub_24a28(unsigned int a0,long a1,long a2,char *a3)
{
  sub_24a62(a0,a1,a2,a3,0xffffffffffffffff);
}

// Function: sub_24a62 @ 0x24a62
void sub_24a62(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v1 = dat_43b20;
  v2 = dat_43b28;
  v3 = dat_43b30;
  v4 = dat_43b38;
  v5 = dat_43b40;
  v6 = dat_43b48;
  v7 = dat_43b50;
  sub_22e2d(&v1,a1,a2);
  sub_243a3(a0,a3,a4,&v1);
}

// Function: sub_24b20 @ 0x24b20
void sub_24b20(long a0,long a1,char *a2)
{
  sub_24a28(0,a0,a1,a2);
}

// Function: sub_24b53 @ 0x24b53
void sub_24b53(long a0,long a1,char *a2,unsigned long a3)
{
  sub_24a62(0,a0,a1,a2,a3);
}

// Function: sub_24b91 @ 0x24b91
void sub_24b91(unsigned int a0,char *a1,unsigned long a2)
{
  sub_243a3(a0,a1,a2,(struct_5 *)0x330c0);
}

// Function: sub_24bc3 @ 0x24bc3
void sub_24bc3(char *a0,unsigned long a1)
{
  sub_24b91(0,a0,a1);
}

// Function: sub_24bee @ 0x24bee
void sub_24bee(unsigned int a0,char *a1)
{
  sub_24b91(a0,a1,0xffffffffffffffff);
}

// Function: sub_24c1b @ 0x24c1b
void sub_24c1b(char *a0)
{
  sub_24bee(0,a0);
}

// Function: sub_24c3e @ 0x24c3e
long sub_24c3e(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  unsigned long v2; // stack - 0x30
  
  v2 = a2;
  while( true ) {
    do {
      v1 = read(a0,a1,v2);
      if (0 <= v1)
        return v1;
    } while (*__errno_location() == 4);
    if (*__errno_location() != 0x16) break;
    if (v2 <= 0x7ff00000)
      return v1;
    v2 = 0x7ff00000;
  }
  return v1;
}

// Function: sub_24cb2 @ 0x24cb2
void sub_24cb2(void)
{
  return;
}

// Function: sub_24cbd @ 0x24cbd
void sub_24cbd(unsigned long a0) // return-dupe
{
  char v1 [8];
  long v2; // stack - 0x30
  
  if ((int)sub_26068(a0,v1) <= -1)
    return;
  dat_43c60 = v2 + -1;
}

// Function: sub_24d19 @ 0x24d19
void sub_24d19(unsigned long a0,long a1,long a2) // return-dupe x2, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_24d19
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
  
  v1 = *(unsigned long *)(a1 + 0x10);
  if ((dat_43c80) && (v3 = (*dat_43c80)(v1,0), v3))
    return;
  if ((dat_43c68) && (v2 = *(unsigned long *)(a2 + 0xa0), dat_43c60)) {
    v3 = *__errno_location();
    v4 = sub_26068(dat_43c60,&v6);
    *__errno_location() = v3;
    if (0 <= v4) {
      if (v6 <= v1) // branch-flip
        v7 = v1 <= (unsigned long)(v8 - 1U);
      else {
        v7 = (*v9)(v1,&v6) != 0;
      }
      if (v7) {
        v5 = ((dat_43c70 <= v2) && (v2 <= dat_43c78 + dat_43c70));
        (*dat_43c68)(v5,a2);
      }
    }
  }
  if ((dat_43c80) && (v3 = (*dat_43c80)(v1,1), v3))
    return;
  signal(0xb,0);
}

// Function: sub_24ebc @ 0x24ebc
void sub_24ebc(int a0)
{
  void *v1; // stack - 0xa8
  sigset_t v2;
  undefined4 v3; // stack - 0x20
  
  v1 = sub_24d19;
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
  if (dat_43c68)
    v3 = 0x8000004;
  sigaction(a0,(sigaction *)&v1,NULL);
}

// Function: sub_2511a @ 0x2511a
unsigned long sub_2511a(unsigned long a0)
{
  dat_43c80 = a0;
  sub_24ebc(0xb);
  return 0;
}

// Function: sub_2514d @ 0x2514d
void sub_2514d(void) // return-dupe
{
  dat_43c80 = 0;
  if (dat_43c68) {
    dat_43c80 = 0;
    return;
  }
  signal(0xb,0);
}

// Function: sub_25189 @ 0x25189
unsigned long sub_25189(void *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  sub_24cb2();
  (*a0)(a1,a2,a3);
  return 1;
}

// Function: sub_251ca @ 0x251ca
unsigned long sub_251ca(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe x2
{
  unsigned long v1; // stack - 0x28
  unsigned int v2; // stack - 0x20
  unsigned long v3; // stack - 0x18
  
  if ((!dat_43c60) && (sub_24cbd(&v1), !dat_43c60))
    return 0xffffffff;
  v2 = 0;
  dat_43c68 = a0;
  dat_43c70 = a1;
  dat_43c78 = a2;
  v1 = a1;
  v3 = a2;
  if (sigaltstack(&v1,NULL) <= -1)
    return 0xffffffff;
  sub_24ebc(0xb);
  return 0;
}

// Function: sub_2529c @ 0x2529c
void sub_2529c(void) // return-dupe
{
  char v1 [8];
  unsigned int v2; // stack - 0x20
  
  dat_43c68 = 0;
  if (dat_43c80) // branch-flip
    sub_24ebc(0xb);
  else {
    signal(0xb,0);
  }
  v2 = 2;
  if (0 <= sigaltstack(v1,NULL))
    return;
  perror("gnulib sigsegv (stackoverflow_deinstall_handler)");
}

// Function: sub_25339 @ 0x25339
bool sub_25339(long a0,struct_15 *a1)
{
  return (unsigned long)(a1->field_0x0 - a0) <= (unsigned long)(a1->field_0x0 - a1->field_0x18) >> 1;
}

// Function: sub_2537a @ 0x2537a
unsigned long sub_2537a(struct_16 *a0,char *a1) // return-dupe x3, return-dupe
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
              if ((v4 <= -1) && (*__errno_location() == 4)) goto label_25629;
              if (v4 < 0) goto label_2567b;
              if (v7 - a0->field_0x8 < (unsigned long)(long)(v4 + 0x1049)) break;
              if (!v4) {
                close(v5);
                return 0;
              }
              a0->field_0x8 = a0->field_0x8 + (long)v4;
            }
          }
          goto label_25529;
        }
label_2567b:
        close(v5);
        goto label_25688;
      }
    }
    else {
label_25529:
      if (v6) { // branch-flip
        v7 <<= 1;
        if (!v7) goto label_2567b;
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
label_25629:
    if ((long)lseek(v5,0,0) <= -1) {
      close(v5);
      v5 = open(a1,0);
      if (v5 < 0) {
label_25688:
        if (!a0->field_0x20)
          return 0xffffffff;
        munmap((void *)a0->field_0x20,a0->field_0x28);
        return 0xffffffff;
      }
    }
  } while( true );
}

// Function: sub_256b7 @ 0x256b7
unsigned long sub_256b7(struct_17 *a0) // early-return
{
  if (a0->field_0x0 == a0->field_0x8) {
    a0->field_0x10 = 1;
    return 0xffffffff;
  }
  return (unsigned long)*(unsigned char *)(a0->field_0x0 + a0->field_0x18);
}

// Function: sub_25703 @ 0x25703
int sub_25703(struct_17 *a0)
{
  int v1; // eax
  
  v1 = sub_256b7(a0);
  if (0 <= v1)
    a0->field_0x0 = a0->field_0x0 + 1;
  return v1;
}

// Function: sub_2573f @ 0x2573f
unsigned long sub_2573f(struct_17 *a0,long *a1) // return-dupe
{
  int v1; // eax
  int v2; // stack - 0x18
  long v3; // stack - 0x10
  
  v3 = 0;
  v2 = 0;
  do {
    v1 = sub_256b7(a0);
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
    sub_25703(a0);
    v2 += 1;
  } while( true );
}

// Function: sub_2581b @ 0x2581b
void sub_2581b(long a0) // return-dupe
{
  if (!*(long *)(a0 + 0x20))
    return;
  munmap(*(void **)(a0 + 0x20),*(unsigned long *)(a0 + 0x28));
}

// Function: sub_25856 @ 0x25856
unsigned long sub_25856(struct_18 *a0) // return-dupe
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
  
  if ((int)sub_2537a(v3,"/proc/self/maps") < 0)
    return 0xffffffff;
  v6 = v8;
  v7 = v9;
  do {
    while( true ) {
      if ((((int)sub_2573f(v3,&v4) < 0) || (sub_25703(v3) != 0x2d)) || ((int)sub_2573f(v3,&v5) < 0)) {
        sub_2581b(v3); // return-dupe
        return 0;
      }
      do {
        v2 = sub_25703(v3);
        if ((v2 != -1) && (v2 != 10))
          v1 = 1;
        else {
          v1 = 0;
        }
      } while (v1);
      if ((v6 < v4) || ((unsigned long)(v5 - 1U) < v7 - 1)) break;
      if (((v4 < v6) && (v2 = sub_25fdb(a0,v4,v6), v2)) || ((v7 - 1 < (unsigned long)(v5 - 1U) && (v2 = sub_25fdb(a0,v7,v5), v2)))) {
        sub_2581b(v3);
        return 0;
      }
    }
    v2 = sub_25fdb(a0,v4,v5);
  } while (!v2);
  sub_2581b(v3);
  return 0;
}

// Function: sub_259f7 @ 0x259f7
unsigned long sub_259f7(struct_18 *a0) // return-dupe
{
  if (!sub_25856(a0))
    return 0;
  return 0xffffffff;
}

// Function: sub_25a2a @ 0x25a2a
void sub_25a2a(void)
{
  dat_43c88 = (long)getpagesize();
}

// Function: sub_25a43 @ 0x25a43
unsigned int sub_25a43(void *a0)
{
  unsigned int v1; // eax
  long v2;
  
  v1 = mincore(a0,dat_43c88,&v2);
  return ~v1 >> 0x1f;
}

// Function: sub_25a9a @ 0x25a9a
unsigned long sub_25a9a(unsigned long a0)
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
    if (v5 / dat_43c88 < v6)
      v6 = v5 / dat_43c88;
    v1 = dat_43c88 * v6;
    v3 = dat_43c88 * v6;
    if (mincore((void *)(v5 - v3),v1,v4) <= -1) break;
    v5 -= dat_43c88 * v6;
  }
  while (v6 != 1) {
    v1 = v6 + 1 >> 1;
    v2 = dat_43c88 * v1;
    v3 = dat_43c88 * v1;
    if (0 <= mincore((void *)(v5 - v3),v2,v4)) { // branch-flip
      v5 -= dat_43c88 * v1;
      v6 >>= 1;
    }
    else {
      v6 = v1;
    }
  }
  return v5;
}

// Function: sub_25c4e @ 0x25c4e
void * sub_25c4e(long a0)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char v3 [1032];
  void *v4; // stack - 0x440
  unsigned long v5; // stack - 0x438
  
  v5 = 0x400;
  v4 = (void *)(a0 + dat_43c88);
  while( true ) {
    if (!v4)
      return NULL;
    if ((unsigned long)-(long)v4 / dat_43c88 < v5)
      v5 = (unsigned long)-(long)v4 / dat_43c88;
    v1 = dat_43c88 * v5;
    if (mincore(v4,v1,v3) <= -1) break;
    v4 = (void *)((long)v4 + dat_43c88 * v5);
  }
  while (v5 != 1) {
    v1 = v5 + 1 >> 1;
    v2 = dat_43c88 * v1;
    if (0 <= mincore(v4,v2,v3)) { // branch-flip
      v4 = (void *)((long)v4 + dat_43c88 * v1);
      v5 >>= 1;
    }
    else {
      v5 = v1;
    }
  }
  return v4;
}

// Function: sub_25de6 @ 0x25de6
unsigned long sub_25de6(unsigned long a0,unsigned long a1) // return-dupe
{
  long v1; // rax
  unsigned long v2; // rax
  long v3; // rax
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  void *v6; // stack - 0x20
  
  v1 = dat_43c88 * (a0 / dat_43c88);
  v2 = (dat_43c88 * (a1 / dat_43c88 + 1) - v1) / dat_43c88;
  for (v4 = 1; v4 < v2; v4 = v4 << 1) {
  }
  do {
    v4 >>= 1;
    if (!v4)
      return 1;
    v3 = dat_43c88 * v4;
    v6 = (void *)(v3 + v1);
    for (v5 = v4; v5 < v2; v5 = v5 + v4 * 2) {
      if (sub_25a43(v6))
        return 0;
      v6 = (void *)((long)v6 + v3 * 2);
    }
  } while( true );
}

// Function: sub_25ef7 @ 0x25ef7
unsigned long sub_25ef7(unsigned long a0,long *a1) // early-return
{
  long v1;
  unsigned long v2; // rax
  
  v2 = a0 * 2 - *a1;
  if (a0 < v2)
    return 0;
  v1 = *a1;
  return sub_25de6(v2,v1 - 1);
}

// Function: sub_25f54 @ 0x25f54
unsigned long sub_25f54(unsigned long a0,unsigned long *a1)
{
  unsigned long v1; // rax
  
  if (!dat_43c88)
    sub_25a2a();
  v1 = dat_43c88 * (a0 / dat_43c88);
  *a1 = sub_25a9a(v1);
  a1[1] = sub_25c4e(v1);
  a1[2] = sub_25ef7;
  return 0;
}

// Function: sub_25fdb @ 0x25fdb
unsigned long sub_25fdb(struct_18 *a0,unsigned long a1,unsigned long a2) // return-dupe
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

// Function: sub_26068 @ 0x26068
unsigned long sub_26068(unsigned long a0,unsigned long *a1) // early-return
{
  unsigned long v1; // stack - 0x38
  unsigned long *v2; // stack - 0x30
  unsigned long v3; // stack - 0x28
  int v4; // stack - 0x20
  
  v3 = 0;
  v4 = -1;
  v1 = a0;
  v2 = a1;
  sub_259f7(&v1);
  if (!v4) {
    a1[2] = sub_25339;
    return 0;
  }
  return sub_25f54(a0,a1);
}

// Function: sub_260fc @ 0x260fc
unsigned long sub_260fc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = memchr(a0,0,a1);
  if (v1)
    a1 = (long)v1 + (1U - (long)a0);
  return a1;
}

// Function: sub_26144 @ 0x26144
int sub_26144(int a0)
{
  int v1;
  int v2; // stack - 0x1c
  
  v2 = a0;
  if ((0 <= a0) && (a0 <= 2)) {
    v2 = sub_285ba(a0);
    v1 = *__errno_location();
    close(a0);
    *__errno_location() = v1;
  }
  return v2;
}

// Function: sub_26195 @ 0x26195
void sub_26195(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5) // return-dupe x10
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

// Function: sub_26791 @ 0x26791
void sub_26791(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4)
{
  long v1; // stack - 0x10
  
  for (v1 = 0; a4[v1]; v1 = v1 + 1) {
  }
  sub_26195(a0,a1,a2,a3,a4,v1);
}

// Function: sub_26804 @ 0x26804
void sub_26804(FILE *a0,char *a1,char *a2,char *a3,struct_19 *a4)
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
  sub_26195(a0,a1,a2,a3,v2,v3);
}

// Function: sub_2690a @ 0x2690a
void sub_2690a(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,FILE *a8,char *a9,char *a10,char *a11,unsigned long a12,unsigned long a13)
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
  sub_26804(a8,a9,a10,a11,&v3);
}

// Function: sub_269ee @ 0x269ee
void sub_269ee(void)
{
  fputs_unlocked("\n",stdout);
  printf(gettext("Report bugs to: %s\n"),"bug-grep@gnu.org");
  printf(gettext("%s home page: <%s>\n"),"GNU grep","https://www.gnu.org/software/grep/");
  printf(gettext("General help using GNU software: <%s>\n"),"https://www.gnu.org/gethelp/");
}

// Function: sub_26a97 @ 0x26a97
void sub_26a97(void *a0,unsigned long a1,unsigned long a2)
{
  sub_26bd0(a0,a1,a2);
}

// Function: sub_26ac8 @ 0x26ac8
long sub_26ac8(long a0)
{
  if (!a0)
    sub_27122(); // no-return
  return a0;
}

// Function: sub_26aea @ 0x26aea
void sub_26aea(unsigned long a0)
{
  sub_26ac8(malloc(a0));
}

// Function: sub_26b10 @ 0x26b10
void sub_26b10(unsigned long a0)
{
  sub_26ac8(sub_28256(a0));
}

// Function: sub_26b36 @ 0x26b36
void sub_26b36(unsigned long a0)
{
  sub_26aea(a0);
}

// Function: sub_26b54 @ 0x26b54
void * sub_26b54(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = realloc(a0,a1);
  if ((!v1) && ((!a0 || (a1))))
    sub_27122(); // no-return
  return v1;
}

// Function: sub_26b9f @ 0x26b9f
void sub_26b9f(void *a0,unsigned long a1)
{
  sub_26ac8(sub_28274(a0,a1));
}

// Function: sub_26bd0 @ 0x26bd0
void * sub_26bd0(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = reallocarray(a0,a1,a2);
  if ((!v1) && ((!a0 || ((a1 && (a2))))))
    sub_27122(); // no-return
  return v1;
}

// Function: sub_26c2a @ 0x26c2a
void sub_26c2a(void *a0,unsigned long a1,unsigned long a2)
{
  sub_26ac8(sub_282d4(a0,a1,a2));
}

// Function: sub_26c63 @ 0x26c63
void sub_26c63(unsigned long a0,unsigned long a1)
{
  sub_26bd0(NULL,a0,a1);
}

// Function: sub_26c8e @ 0x26c8e
void sub_26c8e(unsigned long a0,unsigned long a1)
{
  sub_26c2a(NULL,a0,a1);
}

// Function: sub_26cb9 @ 0x26cb9
void sub_26cb9(void *a0,unsigned long *a1)
{
  sub_26ce7(a0,a1,1);
}

// Function: sub_26ce7 @ 0x26ce7
unsigned long sub_26ce7(void *a0,unsigned long *a1,unsigned long a2)
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
      sub_27122(); // no-return
  }
  else if (!v6) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v2 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0);
    v6 = v2 + (unsigned long)(v2 == 0);
  }
  v4 = sub_26bd0(a0,v6,a2);
  *a1 = v6;
  return v4;
}

// Function: sub_26dc6 @ 0x26dc6
unsigned long sub_26dc6(void *a0,long *a1,long a2,long a3,long a4) // ternary
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
    sub_27122(); // no-return
  v3 = sub_26b54(a0,v5);
  *a1 = v4;
  return v3;
}

// Function: sub_26f82 @ 0x26f82
void sub_26f82(unsigned long a0)
{
  sub_26fc8(a0,1);
}

// Function: sub_26fa5 @ 0x26fa5
void sub_26fa5(unsigned long a0)
{
  sub_26ff9(a0,1);
}

// Function: sub_26fc8 @ 0x26fc8
void sub_26fc8(unsigned long a0,unsigned long a1)
{
  sub_26ac8(calloc(a0,a1));
}

// Function: sub_26ff9 @ 0x26ff9
void sub_26ff9(unsigned long a0,unsigned long a1)
{
  sub_26ac8(sub_282ab(a0,a1));
}

// Function: sub_2702a @ 0x2702a
void sub_2702a(void *a0,unsigned long a1)
{
  memcpy((void *)sub_26aea(a1),a0,a1);
}

// Function: sub_27062 @ 0x27062
void sub_27062(void *a0,unsigned long a1)
{
  memcpy((void *)sub_26b10(a1),a0,a1);
}

// Function: sub_270a2 @ 0x270a2
void sub_270a2(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_26b10(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1);
}

// Function: sub_270f1 @ 0x270f1
void sub_270f1(char *a0)
{
  sub_2702a(a0,strlen(a0) + 1);
}

// Function: sub_27122 @ 0x27122
void sub_27122(void)
{
  error(dat_33080,0,"%s",gettext("memory exhausted"));
  abort(); // no-return
}

// Function: sub_27162 @ 0x27162
void sub_27162(void)
{
  return;
}

// Function: sub_2716d @ 0x2716d
void sub_2716d(unsigned int a0,unsigned int a1) // return-dupe
{
  if (0 <= (int)sub_27811(a0,a1))
    return;
  sub_27162();
}

// Function: sub_2719a @ 0x2719a
unsigned long sub_2719a(long *a0,int a1) // return-dupe, ternary
{
  long v1; // rax
  
  v1 = (long)a1 * *a0;
  if (SEXT816(v1) == SEXT816((long)a1) * SEXT816(*a0)) {
    *a0 = v1;
    return 0;
  }
  v1 = (0 <= *a0) ? 0x7fffffffffffffff : -0x8000000000000000; // branch-flip
  *a0 = v1;
  return 1;
}

// Function: sub_2723c @ 0x2723c
unsigned int sub_2723c(long *a0,unsigned int a1,int a2)
{
  int v1;
  int v2; // stack - 0x28
  unsigned int v3; // stack - 0xc
  
  v3 = 0;
  v2 = a2;
  while (v1 = v2 + -1, v2) {
    v3 |= sub_2719a(a0,a1);
    v2 = v1;
  }
  return v3;
}

// Function: sub_27281 @ 0x27281
unsigned int sub_27281(long a0,long *a1,int a2,unsigned long *a3,char *a4)
{
  char v1;
  long v2;
  unsigned long v3; // stack - 0x20
  unsigned int v4; // stack - 0x38
  unsigned int v5; // stack - 0x34
  int v6; // stack - 0x30
  unsigned int v7; // stack - 0x2c
  long *v8; // stack - 0x18
  
  v4 = 0;
  if ((a2 < 0) || (0x25 <= a2))
    __assert_fail("0 <= strtol_base && strtol_base <= 36","xstrtol.c",0x55,"xstrtoimax"); // no-return
  if (!a1)
    a1 = &v2;
  v8 = a1;
  *__errno_location() = 0;
  v3 = strtoimax(a0,v8,a2);
  if (a0 != *v8) { // branch-flip
    if (*__errno_location()) {
      if (*__errno_location() != 0x22)
        return 4;
      v4 = 1;
    }
  }
  else {
    if (((!a4) || (!*(char *)*v8)) || (v1 = *(char *)*v8, !strchr(a4,(int)v1)))
      return 4;
    v3 = 1;
  }
  if (!a4) {
    *a3 = v3;
    return v4;
  }
  if (*(char *)*v8) {
    v5 = 0x400;
    v6 = 1;
    v1 = *(char *)*v8;
    if (!strchr(a4,(int)v1)) {
      *a3 = v3;
      return v4 | 2;
    }
    if ((((unsigned int)((int)*(char *)*v8 - 0x45U) <= 0x2f) && (0x814400308945U >> ((unsigned char)((int)*(char *)*v8 - 0x45U) & 0x3f) & 1)) && (strchr(a4,0x30))) {
      v1 = *(char *)(*v8 + 1);
      if (v1 != 'i') { // branch-flip
        if ((v1 <= 'i') && ((v1 == 'B' || (v1 == 'D')))) {
          v5 = 1000;
          v6 = 2;
        }
      }
      else if (*(char *)(*v8 + 2) == 'B')
        v6 = 3;
    }
    switch(*(char *)*v8) {
      case 0x42:
        v7 = sub_2719a(&v3,0x400);
        break;
      default:
        *a3 = v3;
        return v4 | 2;
      case 0x45:
        v7 = sub_2723c(&v3,v5,6);
        break;
      case 0x47:
      case 0x67:
        v7 = sub_2723c(&v3,v5,3);
        break;
      case 0x4b:
      case 0x6b:
        v7 = sub_2723c(&v3,v5,1);
        break;
      case 0x4d:
      case 0x6d:
        v7 = sub_2723c(&v3,v5,2);
        break;
      case 0x50:
        v7 = sub_2723c(&v3,v5,5);
        break;
      case 0x54:
      case 0x74:
        v7 = sub_2723c(&v3,v5,4);
        break;
      case 0x59:
        v7 = sub_2723c(&v3,v5,8);
        break;
      case 0x5a:
        v7 = sub_2723c(&v3,v5,7);
        break;
      case 0x62:
        v7 = sub_2719a(&v3,0x200);
        break;
      case 99:
        v7 = 0;
        break;
      case 0x77:
        v7 = sub_2719a(&v3,2);
      
    }
    v4 |= v7;
    *v8 = *v8 + (long)v6;
    if (*(char *)*v8)
      v4 |= 2;
  }
  *a3 = v3;
  return v4;
}

// Function: sub_2768b @ 0x2768b
unsigned long sub_2768b(void) // return-dupe
{
  char *v1; // rax
  
  v1 = getenv("TERM");
  if ((v1) && (strcmp(v1,"dumb")))
    return 1;
  return 0;
}

// Function: sub_276d9 @ 0x276d9
void sub_276d9(void)
{
  return;
}

// Function: sub_276e4 @ 0x276e4
void sub_276e4(char *a0,char *a1)
{
  printf(a0,a1);
}

// Function: sub_27713 @ 0x27713
void sub_27713(char *a0)
{
  fputs(a0,stdout);
}

// Function: sub_2773c @ 0x2773c
char * sub_2773c(char *a0)
{
  bool v1;
  char *v2; // stack - 0x18
  char *v3; // stack - 0x10
  
  v1 = 0;
  for (v2 = a0; *v2 == '/'; v2 = &v2[1]) {
  }
  for (v3 = v2; *v3; v3 = &v3[1]) {
    if (*v3 != '/') { // branch-flip
      if (!v1) goto label_27793;
      v2 = v3;
      v1 = 0;
    }
    else {
      v1 = 1;
    }
label_27793:
  }
  return v2;
}

// Function: sub_277a9 @ 0x277a9
unsigned long sub_277a9(char *a0)
{
  unsigned long v1; // stack - 0x18
  
  for (v1 = strlen(a0); (2 <= v1 && (a0[v1 - 1] == '/')); v1 = v1 - 1) {
  }
  return v1;
}

// Function: sub_277fc @ 0x277fc
unsigned long sub_277fc(void)
{
  return 0;
}

// Function: sub_27811 @ 0x27811
void sub_27811(unsigned int a0,unsigned int a1)
{
  sub_277fc(a0,a1);
}

// Function: sub_27834 @ 0x27834
unsigned long sub_27834(unsigned long a0,unsigned char a1)
{
  return a0 << (a1 & 0x3f) | a0 >> 0x40 - (a1 & 0x3f);
}

// Function: sub_27854 @ 0x27854
unsigned long sub_27854(unsigned long a0,unsigned char a1)
{
  return a0 >> (a1 & 0x3f) | a0 << 0x40 - (a1 & 0x3f);
}

// Function: sub_27874 @ 0x27874
unsigned int sub_27874(unsigned int a0,unsigned char a1)
{
  return a0 << (a1 & 0x1f) | a0 >> 0x20 - (a1 & 0x1f);
}

// Function: sub_27890 @ 0x27890
unsigned int sub_27890(unsigned int a0,unsigned char a1)
{
  return a0 >> (a1 & 0x1f) | a0 << 0x20 - (a1 & 0x1f);
}

// Function: sub_278ac @ 0x278ac
unsigned long sub_278ac(unsigned long a0,unsigned char a1)
{
  return a0 >> (0x40 - a1 & 0x3f) | a0 << (a1 & 0x3f);
}

// Function: sub_278e5 @ 0x278e5
unsigned long sub_278e5(unsigned long a0,unsigned char a1)
{
  return a0 << (0x40 - a1 & 0x3f) | a0 >> (a1 & 0x3f);
}

// Function: sub_2791e @ 0x2791e
unsigned int sub_2791e(unsigned short a0,unsigned char a1)
{
  return (unsigned int)(a0 >> (0x10 - a1 & 0x1f)) | (unsigned int)a0 << (a1 & 0x1f);
}

// Function: sub_27954 @ 0x27954
unsigned int sub_27954(unsigned short a0,unsigned char a1)
{
  return (unsigned int)a0 << (0x10 - a1 & 0x1f) | (unsigned int)(a0 >> (a1 & 0x1f));
}

// Function: sub_2798a @ 0x2798a
unsigned int sub_2798a(unsigned char a0,unsigned char a1)
{
  return (unsigned int)(a0 >> (8 - a1 & 0x1f)) | (unsigned int)a0 << (a1 & 0x1f);
}

// Function: sub_279bf @ 0x279bf
unsigned int sub_279bf(unsigned char a0,unsigned char a1)
{
  return (unsigned int)a0 << (8 - a1 & 0x1f) | (unsigned int)(a0 >> (a1 & 0x1f));
}

// Function: sub_279f4 @ 0x279f4
unsigned long sub_279f4(FILE *a0)
{
  int v1; // eax
  unsigned long v2; // rax
  bool v3; // zf
  
  v2 = __fpending(a0);
  v1 = ferror_unlocked(a0);
  v3 = fclose(a0) == 0;
  if ((!v1) && ((v3 || ((!v2 && (*__errno_location() == 9))))))
    return 0;
  if (v3)
    *__errno_location() = 0;
  return 0xffffffff;
}

// Function: sub_27a83 @ 0x27a83
unsigned long sub_27a83(unsigned long a0)
{
  a0 = a0 - 1 & a0;
  return CONCAT71((undefined7)(a0 >> 8),a0 == 0);
}

// Function: sub_27aa3 @ 0x27aa3
void sub_27aa3(long a0)
{
  *(unsigned long *)(a0 + 0x10) = 0;
  *(unsigned int *)(a0 + 0x18) = 0x95f616;
}

// Function: sub_27ac9 @ 0x27ac9
unsigned long sub_27ac9(struct_10 *a0,struct_3 *a1) // return-dupe
{
  unsigned long v1;
  
  if (a0->field_0x18 != 0x95f616)
    __assert_fail("state->magic == 9827862","cycle-check.c",0x3c,"cycle_check"); // no-return
  if (((a0->field_0x10) && (a1->field_0x8 == a0->field_0x0)) && (a1->field_0x0 == a0->field_0x8))
    return 1;
  a0->field_0x10 = a0->field_0x10 + 1;
  v1 = a0->field_0x10;
  if (!sub_27a83(v1))
    return 0;
  if (a0->field_0x10) {
    a0->field_0x8 = a1->field_0x0;
    a0->field_0x0 = a1->field_0x8;
    return 0;
  }
  return 1;
}

// Function: sub_27bb0 @ 0x27bb0
int sub_27bb0(int a0,int a1)
{
  unsigned long v1;
  unsigned long v2; // stack - 0xa8
  
  if (!a1)
    return sub_27f22(a0,(unsigned int)v2);
  if (a1 == 0x406)
    return sub_27f53(a0,(unsigned int)v2);
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

// Function: sub_27f22 @ 0x27f22
int sub_27f22(int a0,unsigned int a1)
{
  return fcntl(a0,0,(unsigned long)a1);
}

// Function: sub_27f53 @ 0x27f53
int sub_27f53(int a0,unsigned int a1)
{
  int v1;
  unsigned int v2; // eax
  int v3; // stack - 0x14
  
  if (0 <= dat_43c90) { // branch-flip
    v3 = fcntl(a0,0x406,(unsigned long)a1);
    if ((0 <= v3) || (*__errno_location() != 0x16))
      dat_43c90 = 1;
    else {
      v3 = sub_27f22(a0,a1);
      if (0 <= v3)
        dat_43c90 = -1;
    }
  }
  else {
    v3 = sub_27f22(a0,a1);
  }
  if ((0 <= v3) && (dat_43c90 == -1)) {
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

// Function: sub_28056 @ 0x28056
void sub_28056(char *a0,unsigned int a1)
{
  unsigned int v1; // stack - 0xdc
  unsigned int v2; // stack - 0xa8
  
  v1 = 0;
  if (a1 & 0x40)
    v1 = v2;
  sub_26144(open(a0,a1,(unsigned long)v1));
}

// Function: sub_28192 @ 0x28192
unsigned int sub_28192(unsigned int a0) // return-dupe
{
  char v1 [264];
  
  if (sub_28571(a0,v1,0x101))
    return 0;
  if ((strcmp(v1,"C")) && (strcmp(v1,"POSIX")))
    return 1;
  return 0;
}

// Function: sub_2823c @ 0x2823c
unsigned long sub_2823c(void)
{
  *__errno_location() = 0xc;
  return 0;
}

// Function: sub_28256 @ 0x28256
void sub_28256(unsigned long a0)
{
  malloc(a0);
}

// Function: sub_28274 @ 0x28274
void sub_28274(void *a0,unsigned long a1)
{
  realloc(a0,a1 == 0 | a1);
}

// Function: sub_282ab @ 0x282ab
void sub_282ab(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1);
}

// Function: sub_282d4 @ 0x282d4
void sub_282d4(void *a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x20
  unsigned long v2; // stack - 0x18
  
  if ((!a1) || (v1 = a2, v2 = a1, !a2)) {
    v1 = 1;
    v2 = 1;
  }
  reallocarray(a0,v2,v1);
}

// Function: sub_28323 @ 0x28323
char * sub_28323(void)
{
  char *v1; // stack - 0x10
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    v1 = "";
  if (!*v1)
    v1 = "ASCII";
  return v1;
}

// Function: sub_2836b @ 0x2836b
unsigned int sub_2836b(unsigned int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (v1 <= -1) {
    v1 = iswcntrl(a0);
    v1 = (unsigned int)(v1 == 0);
  }
  return v1;
}

// Function: sub_283a6 @ 0x283a6
void sub_283a6(struct_36 *a0,struct_37 *a1) // return-dupe
{
  if ((struct_37 *)a1->field_0x0 != &a1[1]) // branch-flip
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

// Function: sub_2844c @ 0x2844c
bool sub_2844c(unsigned char a0)
{
  return (*(unsigned int *)((long)(int)(unsigned int)(a0 >> 5) * 4 + 0x2ba80) >> (a0 & 0x1f) & 1) != 0;
}

// Function: sub_28491 @ 0x28491
char * sub_28491(int a0)
{
  return setlocale(a0,NULL);
}

// Function: sub_284b9 @ 0x284b9
unsigned long sub_284b9(unsigned int a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_28491(a0);
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

// Function: sub_28571 @ 0x28571
void sub_28571(unsigned int a0,char *a1,unsigned long a2)
{
  sub_284b9(a0,a1,a2);
}

// Function: sub_2859f @ 0x2859f
void sub_2859f(unsigned int a0)
{
  sub_28491(a0);
}

// Function: sub_285ba @ 0x285ba
void sub_285ba(unsigned int a0)
{
  sub_27bb0(a0,0,3);
}

// Function: sub_285f0 @ 0x285f0
void sub_285f0(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_33008); // tail-call
}

// Function: _DT_FINI @ 0x28604
void _DT_FINI(void)
{
  return;
}
