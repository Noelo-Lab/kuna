// Function: _DT_INIT @ 0x4000
void _DT_INIT(void) // return-dupe
{
  if (!dat_2afd8)
    return;
  (*dat_2afd8)();
}

// Function: sub_4020 @ 0x4020
void sub_4020(void)
{
  (*dat_2abc8)(); // jump-as-call
}

// Function: free @ 0x4800
void free(void *a0)
{
  (*dat_2afb8)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x4810
void __cxa_finalize(void)
{
  (*dat_2afe8)(); // jump-as-call
}

// Function: __ctype_toupper_loc @ 0x4820
void * __ctype_toupper_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2abd0)(); // jump-as-call
  return v1;
}

// Function: getenv @ 0x4830
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2abd8)(); // jump-as-call
  return v1;
}

// Function: __snprintf_chk @ 0x4840
int __snprintf_chk(char *a0,unsigned long a1,int a2,unsigned long a3,char *a4,...)
{
  int v1; // eax
  
  v1 = (*dat_2abe0)(); // jump-as-call
  return v1;
}

// Function: raise @ 0x4850
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_2abe8)(); // jump-as-call
  return v1;
}

// Function: strtoimax @ 0x4860
void strtoimax(void)
{
  (*dat_2abf0)(); // jump-as-call
}

// Function: __vfprintf_chk @ 0x4870
int __vfprintf_chk(void *a0,int a1,char *a2,void *a3)
{
  int v1; // eax
  
  v1 = (*dat_2abf8)(); // jump-as-call
  return v1;
}

// Function: __mempcpy_chk @ 0x4880
void * __mempcpy_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2ac00)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x4890
void abort(void)
{
  (*dat_2ac08)(); // jump-as-call
}

// Function: re_match @ 0x48a0
void re_match(void)
{
  (*dat_2ac10)(); // jump-as-call
}

// Function: __errno_location @ 0x48b0
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_2ac18)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x48c0
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_2ac20)(); // jump-as-call
  return v1;
}

// Function: splice @ 0x48d0
void splice(void)
{
  (*dat_2ac28)(); // jump-as-call
}

// Function: _exit @ 0x48e0
void _exit(int a0)
{
  (*dat_2ac30)(); // jump-as-call
}

// Function: strcpy @ 0x48f0
char * strcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2ac38)(); // jump-as-call
  return v1;
}

// Function: __fpending @ 0x4900
unsigned long __fpending(void *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_2ac40)(); // jump-as-call
  return v1;
}

// Function: puts @ 0x4910
int puts(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_2ac48)(); // jump-as-call
  return v1;
}

// Function: ferror @ 0x4920
int ferror(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_2ac50)(); // jump-as-call
  return v1;
}

// Function: qsort @ 0x4930
void qsort(void)
{
  (*dat_2ac58)(); // jump-as-call
}

// Function: isatty @ 0x4940
int isatty(int a0)
{
  int v1; // eax
  
  v1 = (*dat_2ac60)(); // jump-as-call
  return v1;
}

// Function: sigaction @ 0x4950
int sigaction(int a0,void *a1,void *a2)
{
  int v1; // eax
  
  v1 = (*dat_2ac68)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x4960
void iswcntrl(void)
{
  (*dat_2ac70)(); // jump-as-call
}

// Function: reallocarray @ 0x4970
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2ac78)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x4980
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_2ac80)(); // jump-as-call
  return v1;
}

// Function: clearerr_unlocked @ 0x4990
void clearerr_unlocked(void *a0)
{
  (*dat_2ac88)(); // jump-as-call
}

// Function: write @ 0x49a0
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_2ac90)(); // jump-as-call
  return v1;
}

// Function: fread_unlocked @ 0x49b0
void fread_unlocked(void)
{
  (*dat_2ac98)(); // jump-as-call
}

// Function: textdomain @ 0x49c0
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2aca0)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x49d0
int fclose(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_2aca8)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x49e0
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2acb0)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x49f0
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2acb8)(); // jump-as-call
  return v1;
}

// Function: regfree @ 0x4a00
void regfree(void)
{
  (*dat_2acc0)(); // jump-as-call
}

// Function: __ctype_get_mb_cur_max @ 0x4a10
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_2acc8)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x4a20
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_2acd0)(); // jump-as-call
  return v1;
}

// Function: openat @ 0x4a30
int openat(int a0,char *a1,int a2,...)
{
  int v1; // eax
  
  v1 = (*dat_2acd8)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x4a40
void __stack_chk_fail(void)
{
  (*dat_2ace0)(); // jump-as-call
}

// Function: mmap @ 0x4a50
void mmap(void)
{
  (*dat_2ace8)(); // jump-as-call
}

// Function: getopt_long @ 0x4a60
int getopt_long(int a0,char **a1,char *a2,void *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_2acf0)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x4a70
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_2acf8)(); // jump-as-call
  return v1;
}

// Function: pcre2_jit_stack_assign_8 @ 0x4a80
void pcre2_jit_stack_assign_8(void)
{
  (*dat_2ad00)(); // jump-as-call
}

// Function: strchr @ 0x4a90
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2ad08)(); // jump-as-call
  return v1;
}

// Function: __overflow @ 0x4aa0
int __overflow(void *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_2ad10)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x4ab0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2ad18)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x4ac0
void lseek(void)
{
  (*dat_2ad20)(); // jump-as-call
}

// Function: pcre2_compile_context_free_8 @ 0x4ad0
void pcre2_compile_context_free_8(void)
{
  (*dat_2ad28)(); // jump-as-call
}

// Function: __assert_fail @ 0x4ae0
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_2ad30)(); // jump-as-call
}

// Function: fputs @ 0x4af0
int fputs(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_2ad38)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x4b00
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2ad40)(); // jump-as-call
  return v1;
}

// Function: pcre2_set_compile_extra_options_8 @ 0x4b10
void pcre2_set_compile_extra_options_8(void)
{
  (*dat_2ad48)(); // jump-as-call
}

// Function: close @ 0x4b20
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_2ad50)(); // jump-as-call
  return v1;
}

// Function: pcre2_set_character_tables_8 @ 0x4b30
void pcre2_set_character_tables_8(void)
{
  (*dat_2ad58)(); // jump-as-call
}

// Function: closedir @ 0x4b40
int closedir(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_2ad60)(); // jump-as-call
  return v1;
}

// Function: pcre2_general_context_create_8 @ 0x4b50
void pcre2_general_context_create_8(void)
{
  (*dat_2ad68)(); // jump-as-call
}

// Function: regcomp @ 0x4b60
void regcomp(void)
{
  (*dat_2ad70)(); // jump-as-call
}

// Function: memchr @ 0x4b70
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2ad78)(); // jump-as-call
  return v1;
}

// Function: read @ 0x4b80
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_2ad80)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x4b90
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_2ad88)(); // jump-as-call
  return v1;
}

// Function: pcre2_jit_stack_free_8 @ 0x4ba0
void pcre2_jit_stack_free_8(void)
{
  (*dat_2ad90)(); // jump-as-call
}

// Function: fputs_unlocked @ 0x4bb0
int fputs_unlocked(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_2ad98)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x4bc0
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2ada0)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x4bd0
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2ada8)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x4be0
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_2adb0)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x4bf0
void signal(void)
{
  (*dat_2adb8)(); // jump-as-call
}

// Function: dirfd @ 0x4c00
int dirfd(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_2adc0)(); // jump-as-call
  return v1;
}

// Function: sigemptyset @ 0x4c10
int sigemptyset(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_2adc8)(); // jump-as-call
  return v1;
}

// Function: pcre2_get_ovector_pointer_8 @ 0x4c20
void pcre2_get_ovector_pointer_8(void)
{
  (*dat_2add0)(); // jump-as-call
}

// Function: stat @ 0x4c30
int stat(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_2add8)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x4c40
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2ade0)(); // jump-as-call
  return v1;
}

// Function: readdir @ 0x4c50
void * readdir(void *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2ade8)(); // jump-as-call
  return v1;
}

// Function: pcre2_set_depth_limit_8 @ 0x4c60
void pcre2_set_depth_limit_8(void)
{
  (*dat_2adf0)(); // jump-as-call
}

// Function: mincore @ 0x4c70
void mincore(void)
{
  (*dat_2adf8)(); // jump-as-call
}

// Function: wcwidth @ 0x4c80
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_2ae00)(); // jump-as-call
  return v1;
}

// Function: re_set_syntax @ 0x4c90
void re_set_syntax(void)
{
  (*dat_2ae08)(); // jump-as-call
}

// Function: iswalnum @ 0x4ca0
void iswalnum(void)
{
  (*dat_2ae10)(); // jump-as-call
}

// Function: malloc @ 0x4cb0
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2ae18)(); // jump-as-call
  return v1;
}

// Function: wcrtomb @ 0x4cc0
void wcrtomb(void)
{
  (*dat_2ae20)(); // jump-as-call
}

// Function: nl_langinfo @ 0x4cd0
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2ae28)(); // jump-as-call
  return v1;
}

// Function: pcre2_jit_stack_create_8 @ 0x4ce0
void pcre2_jit_stack_create_8(void)
{
  (*dat_2ae30)(); // jump-as-call
}

// Function: regexec @ 0x4cf0
void regexec(void)
{
  (*dat_2ae38)(); // jump-as-call
}

// Function: strcoll @ 0x4d00
int strcoll(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_2ae40)(); // jump-as-call
  return v1;
}

// Function: re_search @ 0x4d10
void re_search(void)
{
  (*dat_2ae48)(); // jump-as-call
}

// Function: pcre2_get_error_message_8 @ 0x4d20
void pcre2_get_error_message_8(void)
{
  (*dat_2ae50)(); // jump-as-call
}

// Function: fchdir @ 0x4d30
int fchdir(int a0)
{
  int v1; // eax
  
  v1 = (*dat_2ae58)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x4d40
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_2ae60)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x4d50
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2ae68)(); // jump-as-call
  return v1;
}

// Function: munmap @ 0x4d60
int munmap(void *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = (*dat_2ae70)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x4d70
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2ae78)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x4d80
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_2ae80)(); // jump-as-call
  return v1;
}

// Function: iswspace @ 0x4d90
int iswspace(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_2ae88)(); // jump-as-call
  return v1;
}

// Function: pcre2_maketables_8 @ 0x4da0
void pcre2_maketables_8(void)
{
  (*dat_2ae90)(); // jump-as-call
}

// Function: mempcpy @ 0x4db0
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2ae98)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x4dc0
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2aea0)(); // jump-as-call
  return v1;
}

// Function: error @ 0x4dd0
void error(int a0,int a1,char *a2,...)
{
  (*dat_2aea8)(); // jump-as-call
}

// Function: memrchr @ 0x4de0
void memrchr(void)
{
  (*dat_2aeb0)(); // jump-as-call
}

// Function: pcre2_match_8 @ 0x4df0
void pcre2_match_8(void)
{
  (*dat_2aeb8)(); // jump-as-call
}

// Function: open @ 0x4e00
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_2aec0)(); // jump-as-call
  return v1;
}

// Function: pcre2_compile_context_create_8 @ 0x4e10
void pcre2_compile_context_create_8(void)
{
  (*dat_2aec8)(); // jump-as-call
}

// Function: fopen @ 0x4e20
void * fopen(char *a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2aed0)(); // jump-as-call
  return v1;
}

// Function: perror @ 0x4e30
void perror(char *a0)
{
  (*dat_2aed8)(); // jump-as-call
}

// Function: sysconf @ 0x4e40
long sysconf(int a0)
{
  long v1; // rax
  
  v1 = (*dat_2aee0)(); // jump-as-call
  return v1;
}

// Function: towlower @ 0x4e50
unsigned int towlower(unsigned int a0)
{
  unsigned int v1; // eax
  
  v1 = (*dat_2aee8)(); // jump-as-call
  return v1;
}

// Function: wctob @ 0x4e60
void wctob(void)
{
  (*dat_2aef0)(); // jump-as-call
}

// Function: fdopendir @ 0x4e70
void * fdopendir(int a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2aef8)(); // jump-as-call
  return v1;
}

// Function: towupper @ 0x4e80
void towupper(void)
{
  (*dat_2af00)(); // jump-as-call
}

// Function: fstatfs @ 0x4e90
void fstatfs(void)
{
  (*dat_2af08)(); // jump-as-call
}

// Function: __cxa_atexit @ 0x4ea0
void __cxa_atexit(void)
{
  (*dat_2af10)(); // jump-as-call
}

// Function: pcre2_match_context_create_8 @ 0x4eb0
void pcre2_match_context_create_8(void)
{
  (*dat_2af18)(); // jump-as-call
}

// Function: getpagesize @ 0x4ec0
int getpagesize(void)
{
  int v1; // eax
  
  v1 = (*dat_2af20)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x4ed0
void exit(int a0)
{
  (*dat_2af28)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x4ee0
int __fprintf_chk(void *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_2af30)(); // jump-as-call
  return v1;
}

// Function: pcre2_match_data_create_from_pattern_8 @ 0x4ef0
void pcre2_match_data_create_from_pattern_8(void)
{
  (*dat_2af38)(); // jump-as-call
}

// Function: pcre2_config_8 @ 0x4f00
void pcre2_config_8(void)
{
  (*dat_2af40)(); // jump-as-call
}

// Function: re_compile_pattern @ 0x4f10
void re_compile_pattern(void)
{
  (*dat_2af48)(); // jump-as-call
}

// Function: pcre2_jit_compile_8 @ 0x4f20
void pcre2_jit_compile_8(void)
{
  (*dat_2af50)(); // jump-as-call
}

// Function: fflush_unlocked @ 0x4f30
int fflush_unlocked(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_2af58)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x4f40
int mbsinit(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_2af60)(); // jump-as-call
  return v1;
}

// Function: sigaltstack @ 0x4f50
void sigaltstack(void)
{
  (*dat_2af68)(); // jump-as-call
}

// Function: iswprint @ 0x4f60
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_2af70)(); // jump-as-call
  return v1;
}

// Function: pcre2_compile_8 @ 0x4f70
void pcre2_compile_8(void)
{
  (*dat_2af78)(); // jump-as-call
}

// Function: fstat @ 0x4f80
int fstat(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_2af80)(); // jump-as-call
  return v1;
}

// Function: getc @ 0x4f90
int getc(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_2af88)(); // jump-as-call
  return v1;
}

// Function: sigaddset @ 0x4fa0
int sigaddset(void *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_2af90)(); // jump-as-call
  return v1;
}

// Function: strstr @ 0x4fb0
char * strstr(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_2af98)(); // jump-as-call
  return v1;
}

// Function: fstatat @ 0x4fc0
int fstatat(int a0,char *a1,void *a2,int a3)
{
  int v1; // eax
  
  v1 = (*dat_2afa0)(); // jump-as-call
  return v1;
}

// Function: __ctype_tolower_loc @ 0x4fd0
void * __ctype_tolower_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2afa8)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x4fe0
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_2afb0)(); // jump-as-call
  return v1;
}

// Function: sub_4ff0 @ 0x4ff0
unsigned long sub_4ff0(int a0,char **a1) // early-return
{
  long *v1;
  int v10;
  int v11;
  char *v12;
  void *v13; // rax
  unsigned long v14;
  void *v15;
  char *v16;
  char *v17;
  long v18; // rax
  unsigned long v19; // rax
  unsigned int *v2;
  unsigned long v20; // rax
  long v21;
  long *v22;
  char *v23;
  long v24; // stack - 0x1a8
  char v25; // stack - 0x68
  long v26; // stack - 0x1a0
  char v27 [3];
  char v28 [8];
  char v29; // stack - 0x67
  char v3;
  long v30; // stack - 0x188
  long *v31; // stack - 0x1b0
  long v32 [18]; // stack - 0xf8
  unsigned long v33; // stack - 0x190
  char v34 [4];
  unsigned int v35; // eax
  unsigned int *v36;
  unsigned int v37; // eax
  long *v38;
  char *v39; // r12
  unsigned int v4;
  int *v40; // rax
  char *v41; // r14
  unsigned char *v42; // r15
  char *v43;
  unsigned char *v44;
  int v45; // stack - 0x1f8
  int v46; // stack - 0x1e4
  unsigned char *v47; // stack - 0x1e0
  long *v48; // stack - 0x1d8
  int v49; // stack - 0x1d0
  char v5 [16];
  unsigned int v50; // stack - 0x1cc
  long v51; // stack - 0x180
  unsigned int v52; // stack - 0x170
  char v53; // stack - 0x66
  bool v6;
  unsigned int v7;
  unsigned char v8;
  char v9; // al
  
  dat_2b684 = 10;
  v31 = NULL;
  v24 = 0;
  dat_2b080 = 2;
  dat_2b804 = 0xffffffff;
  dat_2b7e0 = 0x7fffffffffffffff;
  dat_2b7f8 = -1;
  dat_2b7f0 = -1;
  v26 = -1;
  dat_2b8fd = '\0';
  setlocale(6,"");
  bindtextdomain("grep","/usr/local/share/locale");
  textdomain("grep");
  sub_16210(0x2b180);
  sub_20120(sub_7a00);
  sub_f6a0(0);
  dat_2b8c8 = sub_1a2d0(0,0,sub_7850,sub_78a0,0);
  if (!dat_2b8c8)
    sub_1f410(); // no-return
  v48 = &v24;
  v50 = 0;
  v49 = 0;
  v46 = -1;
  v47 = NULL;
label_5148:
  v10 = optind;
  v6 = 0;
  v16 = &v25;
  while( true ) {
    v11 = optind;
    v45 = getopt_long(a0,a1,"0123456789A:B:C:D:EFGHIPTUVX:abcd:e:f:hiLlm:noqRrsuvwxyZz",(void *)0x29fc0,NULL);
    if ((unsigned int)(v45 - 0x30U) > 9) break;
    if ((dat_2b010 != v11) || (!v6))
      v12 = &v25;
    else {
      v12 = &v16[-(unsigned long)(v25 == '0')];
      if (v12 == v27) {
        builtin_strncpy(&v16[-(unsigned long)(v25 == '0')],"...",4);
        v16 = &v27[3];
        goto label_51b5;
      }
    }
    dat_2b010 = v11;
    v16 = &v12[1];
    v6 = 1;
    *v12 = (char)v45;
  }
  if (v16 != &v25) {
label_51b5:
    *v16 = '\0';
    sub_7d10(&v25,&v26);
  }
  v16 = optarg;
  if (v45 != -1) {
    switch(v45) {
      case 0:
      case 0x55:
        break;
      default:
        sub_a170(2); // no-return, return-dupe
      case 0x41:
        sub_7d10(optarg,0x2b7f0);
        break;
      case 0x42:
        sub_7d10(optarg,0x2b7f8);
        break;
      case 0x43:
        sub_7d10(optarg,&v26);
        break;
      case 0x44:
        v10 = strcmp(optarg,"read");
        if (v10) { // branch-flip
label_5a31:
          if (strcmp(v16,"skip")) {
            error(2,0,dcgettext(NULL,"unknown devices method",5));
            goto label_666b;
          }
          dat_2b884 = 2;
        }
        else {
          dat_2b884 = 1;
        }
        break;
      case 0x45:
        v46 = sub_7950("egrep",v46);
        break;
      case 0x46:
        v46 = sub_7950("fgrep",v46);
        break;
      case 0x47:
        v46 = sub_7950("grep",v46);
        break;
      case 0x48:
        v50 = 1;
        break;
      case 0x49:
label_52b7:
        dat_2b810 = 2;
        break;
      case 0x4c:
        dat_2b80c = 2;
        break;
      case 0x50:
        v46 = sub_7950("perl",v46);
        break;
      case 0x52:
        dat_2b014 = 0x30a;
        goto label_5267;
      case 0x54:
        dat_2b8fc = 1;
        break;
      case 0x56:
        dat_2b905 = '\x01';
        break;
      case 0x58:
        v46 = sub_7950(optarg,v46);
        break;
      case 0x5a:
        dat_2b804 = 0;
        break;
      case 0x61:
label_5291:
        dat_2b810 = 1;
        break;
      case 0x62:
        dat_2b800 = 1;
        break;
      case 99:
        dat_2b7e8 = 1;
        break;
      case 100:
        v21 = sub_f3f0("--directories",optarg,0x29fa0,0x22788,4,dat_2b078,1);
        dat_2b018 = *(int *)(v21 * 4 + 0x22788);
        if (dat_2b018 != 3)
          v10 = v49;
        v49 = v10;
        break;
      case 0x65:
        v14 = strlen(optarg);
        v22 = v31;
        v21 = (long)v31 + ((v14 + 1) - v24);
        if (1 <= v21) {
          dat_2b8d8 = (unsigned char *)sub_1f1f0(v47,v48,v21,0xffffffffffffffff,1);
          v47 = dat_2b8d8;
          v16 = optarg;
        }
        *(char *)mempcpy(&v47[(long)v22],v16,v14) = 10;
        v31 = (long *)sub_7a20(v47,v22,v14 + 1 + (long)v22,"");
        break;
      case 0x66:
        v10 = strcmp(optarg,"-");
        if (v10) { // branch-flip
          v44 = fopen(v16,"r");
          if (!v44) {
            error(2,*__errno_location(),"%s",optarg);
            v16 = NULL;
            goto label_5a31;
          }
        }
        else {
          v44 = stdin;
        }
        v22 = v31;
        v38 = v31;
        while( true ) {
          v21 = (long)v38 + (2U - v24);
          if (1 <= v21) {
            v47 = (unsigned char *)sub_1f1f0(v47,v48,v21,0xffffffffffffffff,1);
            dat_2b8d8 = v47;
          }
          v21 = fread_unlocked(&v47[(long)v38],1,v24 - (long)(long *)((long)v38 + 1U),v44);
          if (!v21) break;
          v38 = (long *)((long)v38 + v21);
        }
        v40 = __errno_location();
        v10 = *v40;
        if (*v44 & 0x20) { // branch-flip
label_5749:
          if (v10) {
            error(2,v10,"%s",optarg);
            return v19;
          }
        }
        else if (stdin != v44) { // branch-flip
          if (fclose(v44)) {
            v10 = *v40;
            goto label_5749;
          }
        }
        else {
          clearerr_unlocked(v44);
        }
        if ((v38 != v22) && ((&v47[-1])[(long)v38] != 10)) {
          v47[(long)v38] = 10;
          v38 = (long *)((long)v38 + 1U);
        }
        v31 = (long *)sub_7a20(v47,v22,v38,optarg);
        break;
      case 0x68:
        v50 = 0xffffffff;
        break;
      case 0x69:
      case 0x79:
        dat_2b687 = '\x01';
        break;
      case 0x6c:
        goto label_5660;
      case 0x6d:
        v37 = sub_1f450(optarg,0,10,0x2b7e0,"");
        if (2 <= v37) {
          error(2,0,dcgettext(NULL,"invalid max count",5));
label_5660:
          dat_2b80c = 1;
        }
        break;
      case 0x6e:
        dat_2b801 = 1;
        break;
      case 0x6f:
        dat_2b8fd = '\x01';
        break;
      case 0x71:
        goto label_5540;
      case 0x72:
label_5267:
        dat_2b018 = 3;
        v49 = v10;
        break;
      case 0x73:
        dat_2b904 = 1;
        break;
      case 0x75:
        error(0,0,dcgettext(NULL,"warning: --unix-byte-offsets (-u) is obsolete",5));
        break;
      case 0x76:
        dat_2b802 = 1;
        break;
      case 0x77:
        sub_e570();
        dat_2b686 = 1;
        break;
      case 0x78:
        dat_2b685 = '\x01';
        break;
      case 0x7a:
        dat_2b684 = 0;
        break;
      case 0x80:
        v10 = strcmp(optarg,"binary");
        if (!v10) {
          dat_2b810 = 0;
          break;
        }
        if (!strcmp(v16,"text")) goto label_5291;
        if (strcmp(v16,"without-match")) {
          error(2,0,dcgettext(NULL,"unknown binary-files type",5));
          return v20;
        }
        goto label_52b7;
      case 0x81:
        if (optarg) {
          v10 = sub_f750(optarg,"always");
          if (((!v10) || (!sub_f750(v16,"yes"))) || (!sub_f750(v16,"force"))) {
            dat_2b900 = 1;
            break;
          }
          if (((!sub_f750(v16,"never")) || (!sub_f750(v16,"no"))) || (!sub_f750(v16,"none"))) {
            dat_2b900 = 0;
            break;
          }
          if (((sub_f750(v16,"auto")) && (sub_f750(v16,"tty"))) && (sub_f750(v16,"if-tty"))) {
            dat_2b908 = 1;
            break;
          }
        }
        dat_2b900 = 2;
        break;
      case 0x82:
        sub_16480(optarg);
        v21 = dat_2b8a0;
        if (!dat_2b8a0) {
          v21 = sub_169d0();
          dat_2b8a0 = v21;
        }
        sub_16d40(v21,optarg,0x50000000);
        v21 = dat_2b8a8;
        if (!dat_2b8a8) {
          v21 = sub_169d0();
          dat_2b8a8 = v21;
        }
        sub_16d40(v21,optarg,0x10000000);
        break;
      case 0x83:
      case 0x86:
        v21 = dat_2b8b0;
        if (!dat_2b8b0) {
          v21 = sub_169d0();
          dat_2b8b0 = v21;
        }
        v37 = (unsigned int)(v45 == 0x86) << 0x1d;
        sub_16d40(v21,optarg,v37 | 0x50000000);
        v21 = dat_2b8b8;
        if (!dat_2b8b8) {
          v21 = sub_169d0();
          dat_2b8b8 = v21;
        }
        sub_16d40(v21,optarg,v37 | 0x10000000);
        break;
      case 0x84:
        v21 = dat_2b8b0;
        if (!dat_2b8b0) {
          v21 = sub_169d0();
          dat_2b8b0 = v21;
        }
        v10 = sub_17290(sub_16d40,v21,optarg,0x50000000,10);
        if (!v10) {
          v21 = dat_2b8b8;
          if (!dat_2b8b8) {
            v21 = sub_169d0();
            dat_2b8b8 = v21;
          }
          v10 = sub_17290(sub_16d40,v21,optarg,0x10000000,10);
          if (!v10) break;
        }
        error(2,*__errno_location(),"%s",optarg);
label_5540:
        dat_2b7a1 = '\x01';
        dat_2b080 = 0;
        break;
      case 0x85:
        dat_2b070 = optarg;
        break;
      case 0x87:
        dat_2b7d8 = 1;
        break;
      case 0x88:
        dat_2b7d0 = optarg;
        break;
      case 0x89:
        dat_2b687 = '\0';
      
    }
    goto label_5148;
  }
  if (dat_2b905) {
    sub_1ed40(stdout,sub_19950(),"GNU grep","3.8",0);
    puts(dcgettext(NULL,"Written by Mike Haertel and others; see\n<https://git.sv.gnu.org/cgit/grep.git/tree/AUTHORS>.",5));
    return 0;
  }
  if (dat_2b908)
    sub_a170(0); // no-return
  if (v47) { // branch-flip
    if (!v31) {
      dat_2b686 = 0;
      dat_2b685 = '\0';
      dat_2b802 ^= 1;
      v31 = (long *)0x1;
      *v47 = 10;
    }
  }
  else {
    v21 = (long)optind;
    if (a0 <= optind) {
      sub_a170(2);
    }
    optind += 1;
    v14 = 0;
    v16 = a1[v21];
    if ((v46 != 2) && (*v16 == '\\'))
      v14 = (unsigned long)(v16[1] == '-');
    v47 = (unsigned char *)sub_1f3f0(&v16[v14]);
    dat_2b8d8 = v47;
    v14 = strlen((char *)v47);
    v47[v14] = 10;
    v31 = (long *)sub_7a20(v47,0,v14 + 1,"");
  }
  v22 = v31;
  v48 = (long *)((long)v31 - 1);
  v31 = v48;
  sub_1a490(dat_2b8c8);
  if (dat_2b7a1) {
    v6 = 0;
    goto label_5965;
  }
  v6 = 0;
  if (!fstat(1,&v30)) {
    if ((v52 & 0xf000) != 0x8000) { // branch-flip
      if ((v52 & 0xf000) == 0x2000) {
        if (((stat("/dev/null",v32)) || (v51 != v32[1])) || (v30 != v32[0]))
          v6 = 1;
        else {
          dat_2b7a0 = '\x01';
        }
      }
    }
    else {
      v21 = 0x24;
      v38 = &v30;
      v36 = (unsigned int *)0x2b920;
      while (v21) {
        v2 = &v36[1];
        v1 = (long *)((long)v38 + 4);
        *v36 = (int)*v38;
        v21 -= 1;
        v38 = v1;
        v36 = v2;
      }
    }
  }
  if (dat_2b7a0) { // branch-flip
label_5965:
    dat_2b80c = 0;
label_596f:
    dat_2b7e8 = 0;
    dat_2b7a2 = 1;
  }
  else if (dat_2b80c) goto label_596f;
  dat_2b803 = dat_2b7e8 | dat_2b7a2;
  if (dat_2b7f0 < 0)
    dat_2b7f0 = v26;
  if (dat_2b7f8 < 0)
    dat_2b7f8 = v26;
  if (((!dat_2b7e0) || (((!v48 && (dat_2b802)) && ((!dat_2b685 && (!dat_2b686)))))) && (dat_2b80c != 2))
    return 1;
  if (dat_2b900 == 2) {
    if ((!v6) || (!sub_1fa50()))
      dat_2b900 = 0;
    else {
      dat_2b900 = (unsigned int)(isatty(1) != 0);
    }
  }
  sub_1fa90();
  if (dat_2b900) {
    v16 = getenv("GREP_COLOR");
    if (v16) {
      if (*v16) {
        v12 = v16;
        while( true ) {
          if (*v12 != ';') {
            if (9 < (unsigned int)((int)*v12 - 0x30U)) goto label_62a1;
          }
          if (!v12[1]) break;
          v12 = &v12[1];
        }
        dat_2b060 = v16;
        dat_2b068 = v16;
      }
    }
label_62a1:
    v12 = getenv("GREP_COLORS");
    if ((v12) && (*v12)) {
      v17 = (char *)sub_1f3f0(v12);
      v23 = NULL;
      v12 = v17;
label_62d6:
      do {
        do {
          do {
            while( true ) {
              v39 = &v12[1];
              v3 = *v12;
              if ((v3 != ':') && (v3)) break;
              *v12 = '\0';
              v21 = 0x2a620;
              v12 = "mt";
              do {
                if (!strcmp(v12,v17)) break;
                v12 = *(char **)(v21 + 0x18);
                v21 += 0x18;
              } while (v12);
              if ((*(unsigned long **)(v21 + 8)) && (v23))
                **(unsigned long **)(v21 + 8) = v23;
              if (*(void **)(v21 + 0x10))
                (**(void **)(v21 + 0x10))();
              if (!v3) goto label_631c;
              v23 = NULL;
              v17 = v39;
              v12 = v39;
            }
            if (v3 == '=') {
              if ((v17 == v12) || (v23)) goto label_631c;
              *v12 = '\0';
              v23 = v39;
              v12 = v39;
              goto label_62d6;
            }
            v12 = v39;
          } while (!v23);
        } while (v3 == ';');
      } while ((unsigned int)((int)v3 - 0x30U) <= 9);
    }
label_631c:
    if ((dat_2b068 == v16) || (dat_2b060 == v16))
      error(0,0,dcgettext(NULL,"warning: GREP_COLOR=\'%s\' is deprecated; use GREP_COLORS=\'mt=%s\'",5),v16,v16);
  }
  v21 = 1;
  v37 = 1;
  v35 = 0;
  do {
    while (*(char *)(v21 + 0x2b183) == '\x01') {
label_5aa0:
      v21 += 1;
      if (v21 == 0x100) goto label_5ad9;
    }
    v7 = v37;
    if (v35 & 0xff & (unsigned int)v21) goto label_5aa0;
    do {
      v37 = v7;
      v7 = v37 * 2;
    } while ((int)(v37 * 2) <= (int)(unsigned int)v21);
    v21 += 1;
    v35 |= v37;
  } while (v21 != 0x100);
label_5ad9:
  dat_2b868 = (unsigned long)(v35 & 0xff) * 0x101010101010101;
  v10 = 0;
  if (0 <= v46)
    v10 = v46;
  if (3 <= v46) goto label_5d5b;
  if (dat_2b687) {
    v40 = (int *)0x2b284;
    v41 = (char *)0x2b6a0;
    do {
      if (*v40 != -1) {
        v11 = sub_16380(*v40,v32);
        if (1 <= v11) {
          v43 = (char *)((long)v32 + (long)v11 * 4);
          do {
            v4 = *(unsigned int *)&v43[-4];
            v33 = 0;
            if (wcrtomb(&v25,v4,&v33) != 1) {
              v9 = 0xff;
              goto label_619c;
            }
            v43 = &v43[-4];
          } while (&v34[(unsigned long)(unsigned int)(v11 - 1) * -4 + (long)v11 * 4] != v43);
        }
        v9 = 1;
label_619c:
        *v41 = v9;
      }
      v40 = &v40[1];
      v41 = &v41[1];
    } while (v40 != (int *)0x2b684);
  }
  if (v46 != 2) {
    if (2 <= dat_2b8d0) {
      v13 = (void *)sub_1ef30(v22);
      v32[0] = 0;
      v22 = v48;
      v15 = v13;
      v44 = v47;
      while (v22) {
        v8 = *v44;
        if (v8 == 0x5b) goto label_5d53;
        if ('\\' <= (char)v8) { // branch-flip
          if (v8 != 0x5c) { // branch-flip
            if (v8 == 0x5e) goto label_5d53;
            if ((unsigned char)(v8 + 0x85) <= 1) goto label_61c8;
          }
          else if (2 <= (long)v22) {
            v8 = v44[1];
            if ('=' <= (char)v8) { // branch-flip
              if ((unsigned char)(v8 - 0x3e) <= 0x3e) {
                v14 = 1L << (v8 - 0x3e & 0x3f);
                if (!(v14 & 0x220001402200011)) {
                  v14 &= 0x6000000000000002;
label_6596:
                  if ((!v14) || (v46 == 1)) goto label_622b;
                  v10 = 0;
                }
                goto label_5d53;
              }
            }
            else if ('\n' <= (char)v8) {
              v14 = 1L << (v8 & 0x3f);
              if (!(v14 & 0x13fe008000000400)) {
                v14 &= 0xb0000000000;
                goto label_6596;
              }
              goto label_5d53;
            }
label_622b:
            v44 = &v44[1];
            v22 = (long *)((long)v22 - 1);
          }
        }
        else if ('/' <= (char)v8) { // branch-flip
          if (v8 == 0x3f) {
label_61c8:
            if (v46 == 1) goto label_5d53;
          }
        }
        else if ('$' <= (char)v8) {
          v14 = 1L << (v8 & 0x3f);
          if (v14 & 0x441000000000) goto label_5d53;
          if (v14 & 0x90000000000) goto label_61c8;
        }
        if (dat_2b687) { // branch-flip
          if (*(int *)((unsigned long)*v44 * 4 + 0x2b284) != -1) // branch-flip
            v14 = (unsigned long)*(char *)((unsigned long)*v44 + 0x2b6a0);
          else {
            v14 = sub_7d80(v44,v22,v32);
          }
        }
        else {
          v14 = sub_e540(v44,v22,v32);
        }
        if ((long)v14 < 0) goto label_5d53;
        v42 = &v44[v14];
        v15 = mempcpy(v15,v44,v14);
        v22 = (long *)((long)v22 - v14);
        v44 = v42;
      }
      v10 = 2;
      v22 = (long *)((long)v15 - (long)v13);
      if (v48 != v22) {
        v31 = v22;
        *(char *)mempcpy(v47,v13,(unsigned long)v22) = 10;
      }
label_5d53:
      free(v13);
    }
    goto label_5d5b;
  }
  if (dat_2b180) { // branch-flip
    v32[0] = 0;
    for (v21 = 0; v21 < (long)v48; v21 = v21 + v14) {
      v3 = *(char *)((unsigned long)v47[v21] + 0x2b183);
      v14 = (unsigned long)v3;
      if (((v3 != '\xfe') || (v14 = sub_1ac90(&v47[v21],(long)v48 - v21,v32), 0x11 <= v14)) && ((long)v14 < 0)) goto label_663d;
    }
    if (dat_2b687) {
label_666b:
      v32[0] = 0;
      for (v21 = 0; v21 < (long)v48; v21 = v21 + v18) {
        v14 = (unsigned long)v47[v21];
        if (*(int *)(v14 * 4 + 0x2b284) != -1) // branch-flip
          v18 = (long)*(char *)(v14 + 0x2b6a0);
        else {
          v18 = sub_7d80(&v47[v21],(long)v48 - v21,v32);
        }
        if (v18 < 0) goto label_663d;
      }
    }
    v8 = 0;
  }
  else {
    v8 = 0;
    if (dat_2b8d0 == 1)
      v8 = dat_2b686;
  }
label_60b9:
  v10 = 2;
  if (v8 & 1) {
    sub_a3e0(0x2b8d8,&v31);
    v47 = dat_2b8d8;
    v10 = 0;
  }
label_5d5b:
  v21 = (long)v10 * 0x20;
  dat_2b878 = *(void **)(v21 + 0x2a7b8);
  dat_2b870 = (**(void **)(v21 + 0x2a7b0))(v47,v31,(long)*(int *)(v21 + 0x2a7ac),dat_2b8fd || dat_2b900);
  v29 = dat_2b684;
  v25 = '\0';
  v53 = 0;
  v21 = (*dat_2b878)(dat_2b870,&v29,1,v28,0);
  dat_2b820 = (v21 == 0) == (bool)dat_2b802;
  a0 -= optind;
  if ((v50) || (2 <= a0))
    dat_2b808 = ~v50 >> 0x1f;
  else {
    dat_2b808 = -(unsigned int)(dat_2b018 == 3);
  }
  v14 = sysconf(0x1e);
  if (0x3ffffffffffffffa < v14 - 1)
    abort(); // no-return
  v5._8_8_ = 0;
  v5._0_8_ = v14;
  v21 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x18000)) % v5._0_16_,0);
  dat_2b858 = 0x18000;
  if (v21)
    dat_2b858 = (v14 + 0x18000) - v21;
  dat_2b858 = v14 + 8 + dat_2b858;
  dat_2b838 = v14;
  dat_2b860 = sub_1ef30(dat_2b858);
  if ((dat_2b014 & 2) && (!dat_2b884))
    dat_2b884 = 1;
  if (1 <= a0) // branch-flip
    a1 = &a1[optind];
  else if ((dat_2b018 != 3) || (v49 <= 0))
    a1 = (char **)0x29f80;
  else {
    dat_2b88a = 1;
    a1 = (char **)0x29f90;
  }
  v16 = *a1;
  v37 = 1;
  do {
    while( true ) {
      a1 = &a1[1];
      if (!strcmp(v16,"-")) break;
      dat_2b890 = v16;
      v35 = sub_a0f0(0xffffff9c,v16,1,1);
      v16 = *a1;
      v37 &= v35;
      if (!v16) goto label_5f29;
    }
    dat_2b890 = dat_2b7d0;
    v35 = sub_9130(0,1);
    v16 = *a1;
    v37 &= v35;
  } while (v16);
label_5f29:
  if (dat_2b889)
    return 2;
  return (unsigned long)v37;
label_663d:
  v8 = 1;
  goto label_60b9;
}

// Function: sub_6750 @ 0x6750
void sub_6750(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_2afc0)(sub_4ff0,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_6780 @ 0x6780
void sub_6780(void)
{
  return;
}

// Function: _FINI_0 @ 0x67f0
void _FINI_0(void)
{
  if (!dat_2b168) {
    if (dat_2afe8)
      __cxa_finalize(dat_2b008);
    sub_6780();
    dat_2b168 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x6830
void _INIT_0(void)
{
  return;
}

// Function: sub_6840 @ 0x6840
unsigned int sub_6840(long a0,unsigned long a1,unsigned long a2,long a3,long a4,unsigned long a5,unsigned int a6) // return-dupe
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
  if ((dat_2b687) || ((char)a6)) {
    v10 = NULL;
    v11 = 0;
    if ((char)a6) {
      re_set_syntax(a5 | 0x2000000);
      v2 = re_compile_pattern(a1,a2,&v5);
      if (!v2) {
        regfree(&v5);
        return v1;
      }
      goto label_68e7;
    }
  }
  else {
    v10 = (void *)sub_1ef30(0x100);
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
label_68e7:
  free(v10);
  if ((a4 != -1) && (v3 = (char *)sub_9080(a4,&v6), *v3)) {
    error(0,0,"%s:%td: %s",v3,v6,v2);
    return 0;
  }
  error(0,0,"%s",v2);
  return 0;
}

// Function: sub_6a10 @ 0x6a10
void sub_6a10(unsigned long a0)
{
  error(2,0,"%s",a0);
}

// Function: sub_6a40 @ 0x6a40
void sub_6a40(unsigned long a0)
{
  error(0,0,dcgettext(NULL,"warning: %s",5),a0); // tail-call
}

// Function: sub_6a70 @ 0x6a70
unsigned long * sub_6a70(char *a0,unsigned long a1,unsigned long a2,char a3)
{
  char v1;
  long v10; // rax
  char *v11;
  unsigned long v12;
  char *v13; // rax
  void *v14; // rax
  long v15; // rdx
  long v16; // stack - 0x50
  long v17; // stack - 0x48
  char *v18;
  long v19;
  unsigned char v2;
  unsigned long v20; // stack - 0xb0
  void *v21; // stack - 0xa0
  char *v22; // stack - 0x90
  unsigned long v23; // stack - 0x88
  char *v24; // stack - 0x80
  char *v25; // stack - 0x70
  unsigned long v26; // stack - 0x68
  long v3;
  bool v4;
  bool v5;
  char v6;
  unsigned long *v7; // rax
  unsigned long v8;
  char *v9; // rax
  
  v7 = (unsigned long *)sub_1f2f0(1,0x48);
  v8 = sub_15a90();
  v7[1] = v8;
  if (dat_2b687)
    a2 |= 0x400000;
  sub_15fa0(v8,0x2b180,a2,(unsigned int)(dat_2b684 == '\0') * 2 | (unsigned int)a2 & 0x10 | 0x2c);
  v6 = dat_2b182;
  v1 = dat_2b180;
  v9 = &a0[a1];
  v5 = 0;
  v7[3] = 0;
  v7[2] = sub_1ef10(0x40) + 0x40;
  v19 = 0;
  v16 = 1;
  v17 = 0;
  v23 = 0;
  v24 = NULL;
  v18 = a0;
  v22 = a0;
  do {
    v21 = rawmemchr(v18,10);
    v10 = (long)v21 - (long)v18;
    v15 = v10 + -1;
    if (0 <= v15) {
      v11 = v18;
      while (v11 = memchr(v11,0x5c,(long)&v18[v15] - (long)v11), v11) {
        if ((unsigned char)(v11[1] - 0x31U) <= 8) {
          if (v18 <= v22) { // branch-flip
            v20 = 0;
            v4 = 1;
          }
          else {
            v12 = (long)v18 - (long)v22;
            v15 = (v23 - v17) + v12;
            if (1 <= v15)
              v24 = (char *)sub_1f1f0(v24,&v17,v15,0xffffffffffffffff,1);
            memcpy(&v24[v23],v22,v12);
            v23 += v12;
            v4 = 1;
            v20 = 0;
          }
          goto label_6bfa;
        }
        if (((int)v11[1] == (-(unsigned int)(v1 == '\x01' && !v6) & 0x24) + 0x5c) && (v11 = &v11[1], v11 == &v18[v15])) break;
        v11 = &v11[1];
      }
    }
    v20 = 1;
    v4 = 0;
label_6bfa:
    v15 = (v7[3] - v16) + 2;
    if (1 <= v15) {
      v3 = v7[2];
      v7[2] = sub_1f1f0(v3 + -0x40,&v16,v15,0xffffffffffffffff,0x40) + 0x40;
    }
    re_set_syntax(a2);
    v8 = v7[3];
    if (!sub_6840(v7,v18,v10,v8,v19,a2,v20))
      v5 = 1;
    v18 = (char *)((long)v21 + 1);
    v19 += 1;
    if (v4) {
      v7[3] = v7[3] + 1;
      v22 = v18;
    }
    if (v9 < v18) {
      if (v5)
        exit(2); // no-return
      if (v22 <= v9) {
        if (v22 <= a0) { // branch-flip
          v23 = a1;
          v24 = a0;
        }
        else {
          v12 = v23 + ((long)v9 - (long)v22);
          v24 = (char *)sub_1efa0(v24,v12);
          memcpy(&v24[v23],v22,(long)v9 - (long)v22);
          v23 = v12;
        }
      }
      if ((dat_2b686) || (v18 = NULL, v25 = a0, v26 = a1, dat_2b685)) {
        v18 = (char *)sub_1ef30(a1 + 0x2d);
        if (dat_2b685) { // branch-flip
          if (a2 & 0x2000) { // branch-flip
            *(unsigned short *)v18 = dat_21097;
            v18[2] = dat_21099;
            v12 = strlen(v18);
            v19 = v12 + a1;
            memcpy(&v18[v12],a0,a1);
            v9 = ")$";
          }
          else {
            *(unsigned int *)v18 = dat_2109a;
            v12 = strlen(v18);
            v19 = v12 + a1;
            memcpy(&v18[v12],a0,a1);
            v9 = "\\)$";
          }
        }
        else if (a2 & 0x2000) { // branch-flip
          *(unsigned long *)v18 = s_21060._0_8_;
          *(unsigned long *)&v18[8] = s_21060._8_8_;
          *(unsigned short *)&v18[0x10] = s_21060._16_2_;
          v18[0x12] = s_21060[0x12];
          v12 = strlen(v18);
          v19 = v12 + a1;
          memcpy(&v18[v12],a0,a1);
          v9 = ")([^[:alnum:]_]|$)";
        }
        else {
          *(unsigned long *)v18 = s_21080._0_8_;
          *(unsigned long *)&v18[8] = s_21080._8_8_;
          *(unsigned int *)&v18[0x10] = s_21080._16_4_;
          *(unsigned short *)&v18[0x14] = s_21080._20_2_;
          v18[0x16] = s_21080[0x16];
          v12 = strlen(v18);
          v19 = v12 + a1;
          memcpy(&v18[v12],a0,a1);
          v9 = "\\)\\([^[:alnum:]_]\\|$\\)";
        }
        v9 = strcpy(&v18[v19],v9);
        v25 = v18;
        v26 = strlen(v9) + v19;
      }
      sub_14cf0(v25,v26,v7[1]);
      v9 = (char *)sub_15020(v7[1]);
      if (v9) {
        v11 = &v9[3];
        v8 = sub_e5c0(0);
        v1 = *v9;
        *v7 = v8;
        if (v1) { // branch-flip
          v7[7] = v7[7] + 1;
          v12 = strlen(v11);
          v19 = (unsigned char)v9[1] + v12 + (unsigned long)(unsigned char)v9[2];
          v13 = (char *)sub_1ef30(v19);
          v1 = dat_2b684;
          *v13 = dat_2b684;
          v2 = v9[1];
          *(unsigned char *)&v7[8] = *(unsigned char *)&v7[8] | v2;
          v14 = memcpy(&v13[v2],v11,v12);
          if (v9[2])
            *(char *)((long)v14 + v12) = v1;
          sub_c9d0(*v7,v13,v19);
          free(v13);
        }
        else {
          sub_c9d0(v8,v11,strlen(v11));
        }
        sub_cec0(*v7);
        sub_15a80(v9);
      }
      sub_15aa0(0,0,v7[1],1);
      if (v24) {
        if ((a3) || (v8 = v7[1], !sub_14e30(v8))) {
          v7[2] = v7[2] + -0x40;
          v7[3] = v7[3] + 1;
          if (!sub_6840(v7,v24,v23,0,0xffffffffffffffff,a2,0))
            abort(); // no-return
        }
        if (v25 != v24)
          free(v24);
      }
      free(v18);
      return v7;
    }
  } while( true );
}

// Function: sub_7110 @ 0x7110
long sub_7110(long *a0,void *a1,long a2,long *a3,void *a4) // return-dupe
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
  
  v4 = dat_2b684;
  v14 = (void *)(a2 + (long)a1);
  v8 = a0[1];
  v7 = sub_14e10(v8);
  v5 = sub_14e20(v8);
  if (v14 <= a1)
    return -1;
  v25 = a1;
  v18 = a1;
  do {
    while (a4) {
      v22 = v25;
      v25 = v14;
      v27 = a4;
label_73d9:
      v28 = (long)v25 - (long)v22;
      if (0x80000000 < v28)
        sub_1f410(); // no-return, return-dupe
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
            *(unsigned char *)(v20 + 0x38) = (dat_2b684 == '\n') << 7 | *(unsigned char *)(v20 + 0x38) & 0x3f;
            v6 = re_search(v20,v22,(int)v28 + -1,(int)v27 - v19,(int)v10 + -1,v1);
            if (v6 < -1) {
              sub_1f410();
            }
            if (v6 != -1) break;
label_7528:
            v8 += 1;
            if (a0[3] <= v8) goto label_75d0;
          }
          v21 = (void *)((long)v6 + (long)v22);
          if (v24 < v21) goto label_7528;
          v20 = (long)(*(int *)a0[6] - v6);
          if (a4) {
            if (dat_2b686) {
              if (!dat_2b685) goto label_748d;
              v23 = v10 + -1 != v20;
              v20 = v10;
              v21 = v27;
              if (v23) goto label_7528;
            }
label_761a:
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
            goto label_7528;
          }
          if (!dat_2b685) {
            if (dat_2b686) {
label_748d:
              do {
                if ((!sub_e820((long)v21 + v20,v12)) && (!sub_e830(v22,v21,v12))) {
                  if (a4) goto label_761a;
                  goto label_7690;
                }
                v6 = (int)v21; // branch-flip
                if (1 <= v20) {
                  v3 = a0[2];
                  v2 = (unsigned char *)(v9 + v3 + 0x38);
                  *v2 = *v2 | 0x40;
                  v13 = re_match(v9 + v3,v22,(v6 + -1 + (int)v20) - (int)v27,v6 - v19,v1);
                  if (v13 < -1) {
                    sub_1f410();
                  }
                  if (v13 <= 0) goto label_74c2;
                }
                else {
label_74c2:
                  if (v12 == v21) break;
                  v20 = a0[2];
                  v2 = (unsigned char *)(v9 + v20 + 0x38);
                  *v2 = *v2 & 0xbf;
                  v6 = re_search(v9 + v20,v22,(int)v28 + -1,(v6 + 1) - v19,((int)v25 - (v6 + 1)) + -1,v1);
                  if (v6 < 0) {
                    if (v6 != -1) {
                      sub_1f410();
                    }
                    goto label_7528;
                  }
                  v13 = *(int *)a0[6] - v6;
                  v21 = (void *)((long)v22 + (long)v6);
                }
                v20 = (long)v13;
              } while (v21 <= v24);
              goto label_7528;
            }
            goto label_7690;
          }
          if (v20 == v10 + -1) goto label_7690;
          v8 += 1;
        } while (v8 < a0[3]);
label_75d0:
        if (v24 < v25) {
          v28 = v26;
          v22 = v24;
          goto label_7690;
        }
      }
label_75e2:
      if (v14 <= v25)
        return 0xffffffffffffffff;
    }
    v17 = '\0';
    v16 = 0;
    v12 = v14;
    if (*a0) {
      v8 = sub_e3e0(*a0,(long)v25 - (unsigned long)*(unsigned char *)&a0[8],(unsigned long)*(unsigned char *)&a0[8] + ((long)v14 - (long)v25),v15,1);
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
        goto label_72a0;
      }
      v12 = (void *)((long)rawmemchr(v21,v19) + 1);
      if (v9 <= v10) {
        v25 = v22;
        goto label_72a0;
      }
      v25 = v12;
      if ((dat_2b180 == '\x01') && (!dat_2b182)) {
        if (v18 < v22)
          v18 = v22;
        if (!sub_e640(&v18,0,v21,v14)) goto label_782b;
        v21 = v18;
        goto label_7335;
      }
label_782b:
      v28 = (long)v25 - (long)v22;
label_7690:
      *a3 = v28;
      return (long)v22 - (long)a1;
    }
label_72a0:
    if (!v7) {
      v21 = v25;
      v22 = v25;
      v25 = v12;
label_7335:
      v12 = (void *)sub_14e00(a0[1],v21,v25,0,&v16,&v17);
      if ((!v12) || (v25 == v12)) goto label_75e2;
      if (v16)
        v22 = (void *)(memrchr(a1,(int)v4,(long)v12 - (long)a1) + 1);
      v25 = (void *)((long)rawmemchr(v12,(int)v4) + 1);
      if (v17) {
        v27 = v22;
        goto label_73d9;
      }
      goto label_782b;
    }
    v11 = (void *)sub_14e00(v7,v25,v12,0,&v16,0);
    if ((v11) && (v12 != v11)) {
      if (v16)
        v25 = (void *)(memrchr(a1,(int)v4,(long)v11 - (long)a1) + 1);
      v16 = 0;
      v21 = v25;
      v22 = v25;
      v25 = (void *)((long)rawmemchr(v11,(int)v4) + 1);
      goto label_7335;
    }
    v25 = v12;
    if (v14 <= v12)
      return 0xffffffffffffffff;
  } while( true );
}

// Function: sub_7850 @ 0x7850
unsigned long sub_7850(long a0,unsigned long a1)
{
  unsigned char v1;
  unsigned long v2;
  unsigned char *v3;
  
  v3 = (unsigned char *)(dat_2b8d8 + -1 + a0);
  v2 = 0x3938a0565d85cb03;
  v1 = *v3;
  while (v1 != 10) {
    v3 = &v3[1];
    v2 = v2 * 0x21 ^ (unsigned long)v1;
    v1 = *v3;
  }
  return v2 % a1;
}

// Function: sub_78a0 @ 0x78a0
unsigned long sub_78a0(long a0,long a1) // return-dupe
{
  char *v1;
  char *v2;
  char *v3;
  long v4; // rax
  long v5;
  char v6;
  
  v1 = (char *)(dat_2b8d8 + -1 + a0);
  v2 = (char *)(dat_2b8d8 + -1 + a1);
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

// Function: sub_78f0 @ 0x78f0
void sub_78f0(void)
{
  dat_2b060 = dat_2b068;
}

// Function: sub_7910 @ 0x7910
void sub_7910(void)
{
  dat_2b900 = 0xffffffff;
}

// Function: sub_7920 @ 0x7920
void sub_7920(void)
{
  dat_2b028 = 0x2109e;
  dat_2b020 = 0x210a4;
}

// Function: sub_7950 @ 0x7950
char * sub_7950(char *a0,int a1)
{
  char *v1; // rax
  unsigned long v2; // rbx
  
  v2 = 0;
  while( true ) {
    if (!strcmp(a0,(char *)(v2 * 0x20 + 0x2a7a0))) break;
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

// Function: sub_7a00 @ 0x7a00
void sub_7a00(void)
{
  if (dat_2b8c0)
    return;
  sub_f7e0(); // tail-call
}

// Function: sub_7a20 @ 0x7a20
long sub_7a20(long a0,long a1,long a2,long a3)
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
      v4 = sub_1a700(dat_2b8c8,a1 + 1,0);
      if (v4) {
        if (v4 < 0)
          sub_1f410(); // no-return
        v6 = (void *)((long)v6 + ((long)v1 - (long)v8));
        if (!v9) {
          v5 = dat_2b8f0;
          if (dat_2b8e0 == dat_2b8e8) {
            v5 = sub_1f1f0(dat_2b8f0,0x2b8e8,1,0xffffffffffffffff,0x18);
            dat_2b8f0 = v5;
          }
          v2 = dat_2b8e0 + 1;
          v3 = (long *)(v5 + dat_2b8e0 * 0x18);
          v3[1] = a3;
          *v3 = dat_2b8d0;
          v3[2] = v7;
          dat_2b8e0 = v2;
        }
        dat_2b8d0 += 1;
        a1 = (long)v6 - a0;
      }
      v7 += 1;
      v8 = v1;
      v9 = v4;
    } while (v1 < (void *)(a2 + a0));
  }
  return a1;
}

// Function: sub_7b80 @ 0x7b80
void sub_7b80(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,char *a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13) // return-dupe
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
  dat_2b8c0 = *__errno_location();
}

// Function: sub_7c60 @ 0x7c60
void sub_7c60(void)
{
  error(2,0,dcgettext(NULL,"input is too large to count",5));
}

// Function: sub_7c90 @ 0x7c90
void sub_7c90(void *a0) // return-dupe
{
  long v1;
  void *v2;
  long v3; // rbx
  int v4; // r12d
  
  v1 = dat_2b818;
  if (a0 <= dat_2b7c0) {
    dat_2b7c0 = a0;
    return;
  }
  v4 = (int)dat_2b684;
  v3 = 0;
  v2 = dat_2b7c0;
  do {
    v2 = memchr(v2,v4,(long)a0 - (long)v2);
    if (!v2) break;
    v2 = (void *)((long)v2 + 1);
    v3 += 1;
  } while (v2 < a0);
  if (!SCARRY8(v3,v1)) {
    dat_2b7c0 = a0;
    dat_2b818 = v3 + v1;
    return;
  }
  sub_7c60(); // no-return
}

// Function: sub_7d10 @ 0x7d10
void sub_7d10(unsigned long a0,long *a1)
{
  if (((unsigned int)sub_1f450(a0,0,10,a1,"") <= 1) && (0 <= *a1))
    return;
  error(2,0,"%s: %s",a0,dcgettext(NULL,"invalid context length argument",5));
}

// Function: sub_7d80 @ 0x7d80
unsigned long sub_7d80(long a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // rax
  long *v2; // rax
  unsigned int v3; // stack - 0xbc
  char v4 [136];
  unsigned long v5;
  
  v1 = sub_1acc0(&v3,a0,a1,a2);
  if ((v1 <= 0x10) && (!sub_16380(v3,v4))) {
    v5 = v1;
    do {
      v5 -= 1;
      if ((long)v5 <= 0)
        return v1;
      v2 = __ctype_toupper_loc();
    } while (*(unsigned int *)(*v2 + (unsigned long)*(unsigned char *)(a0 + v5) * 4) == (unsigned int)*(unsigned char *)(a0 + v5));
  }
  return 0xffffffffffffffff;
}

// Function: sub_7e40 @ 0x7e40
void sub_7e40(void *a0,unsigned long a1) // return-dupe
{
  unsigned long v1; // rax
  
  v1 = fwrite_unlocked(a0,1,a1,stdout);
  if (a1 == v1)
    return;
  dat_2b8c0 = *__errno_location();
}

// Function: sub_7e70 @ 0x7e70
void sub_7e70(unsigned char a0) // return-dupe
{
  unsigned char *v1;
  int v2; // eax
  
  v1 = *(unsigned char **)((long)stdout + 0x28);
  if (v1 < *(unsigned char **)((long)stdout + 0x30)) {
    *(unsigned char **)((long)stdout + 0x28) = &v1[1];
    *v1 = a0;
    return;
  }
  v2 = __overflow(stdout,(unsigned int)a0);
  if (0 <= v2)
    return;
  dat_2b8c0 = *__errno_location();
}

// Function: sub_7ec0 @ 0x7ec0
void sub_7ec0(char a0)
{
  if ((dat_2b900) && (*dat_2b040))
    sub_1faa0(dat_2b028);
  sub_7e70((int)a0);
  if ((dat_2b900) && (*dat_2b040)) {
    sub_1fac0(dat_2b020); // tail-call
    return;
  }
}

// Function: sub_7f20 @ 0x7f20
void sub_7f20(unsigned long a0,char *a1)
{
  if ((dat_2b900) && (*a1))
    sub_1faa0(dat_2b028,a1,a0);
  sub_7b80("%*ld",dat_2b8f8,a0);
  if ((dat_2b900) && (*a1)) {
    sub_1fac0(dat_2b020); // tail-call
    return;
  }
}

// Function: sub_7fa0 @ 0x7fa0
void sub_7fa0(void)
{
  int v1; // eax
  char *v2;
  
  v2 = dat_2b890;
  if ((dat_2b900) && (*dat_2b058)) {
    sub_1faa0(dat_2b028);
    v2 = dat_2b890;
  }
  dat_2b890 = v2;
  if (!v2) {
    v2 = dcgettext(NULL,"(standard input)",5);
    dat_2b890 = v2;
  }
  v1 = fputs_unlocked(v2,stdout);
  if (v1 <= -1)
    dat_2b8c0 = *__errno_location();
  if ((dat_2b900) && (*dat_2b058)) {
    sub_1fac0(dat_2b020); // tail-call
    return;
  }
}

// Function: sub_8050 @ 0x8050
char sub_8050(unsigned long *a0,long a1,unsigned long a2,char a3) // early-return
{
  unsigned long *v1;
  unsigned long *v10;
  bool v2;
  bool v3;
  unsigned long v4;
  char v5; // al
  unsigned long v6; // rax
  unsigned long v7;
  unsigned long v8; // stack - 0x48
  long v9;
  
  if (dat_2b810 != 1) {
    v1 = (unsigned long *)((long)a0 + a1);
    v4 = *v1;
    if (dat_2b868) {
      *(unsigned char *)v1 = 0xff;
      v8 = 0;
      v7 = dat_2b868;
      v10 = a0;
      do {
        for (; (unsigned long)v10 & 7; v10 = (unsigned long *)((long)v10 + 1)) {
          v6 = (unsigned long)(unsigned char)*v10;
          if (v6 & v7) goto label_81ff;
        }
        for (; v6 = v7 & *v10, !v6; v10 = &v10[1]) {
        }
        while (v6 = (unsigned long)(unsigned char)*v10, !(v6 & v7)) {
          v10 = (unsigned long *)((long)v10 + 1);
        }
label_81ff:
        if (v1 <= v10) {
          *(unsigned char *)v1 = (unsigned char)v4;
          break;
        }
        v6 = sub_1ac90(v10,(long)v1 - (long)v10,&v8);
        if ((0x11 <= v6) && ((long)v6 <= -1)) {
          *(unsigned char *)v1 = (unsigned char)v4;
          v5 = '\0';
          dat_2b888 = 1;
          return v5;
        }
        v10 = (unsigned long *)((long)v10 + v6);
        v7 = dat_2b868;
      } while( true );
    }
  }
  if (dat_2b808) {
    sub_7fa0();
    if (dat_2b804) // branch-flip
      sub_7ec0((int)a3);
    else {
      sub_7e70(0);
    }
  }
  if (dat_2b801) {
    v9 = dat_2b818;
    if (dat_2b7c0 < a2) {
      sub_7c90(a0);
      v9 = dat_2b818 + 1;
      if (SCARRY8(dat_2b818,1)) {
        sub_7c60(v9); // no-return, return-dupe
      }
      dat_2b7c0 = a2;
      dat_2b818 = v9;
    }
    sub_7f20(v9,dat_2b050);
    sub_7ec0((int)a3);
  }
  if (dat_2b800) {
    v9 = ((long)a0 - dat_2b848) + dat_2b7c8;
    if (SCARRY8((long)a0 - dat_2b848,dat_2b7c8))
      sub_7c60(v9);
    sub_7f20(v9,dat_2b048);
    sub_7ec0((int)a3);
  }
  if (!dat_2b8fc) {
    v5 = '\x01';
    return v5;
  }
  v2 = dat_2b801 || dat_2b800;
  v3 = dat_2b808 != 0;
  v5 = dat_2b8fc;
  if ((v2 || v3) && a1) {
    sub_7e70(9);
    v5 = (v2 || v3) && a1;
  }
  return v5;
}

// Function: sub_82a0 @ 0x82a0
void sub_82a0(unsigned long a0,unsigned long a1,char a2)
{
  unsigned long v1;
  unsigned long v10;
  bool v11; // zf
  char *v12; // stack - 0x58
  char *v13; // stack - 0x50
  unsigned char v2;
  char v3;
  int v4;
  long v5;
  unsigned long v6;
  unsigned char v7; // cl
  long v8; // stack - 0x48
  unsigned long v9;
  
  if ((!dat_2b8fd) && (!sub_8050(a0,(a1 - a0) + -1,a1,(int)a2)))
    return;
  v11 = a2 == ':';
  v7 = (v11 ^ dat_2b802) & dat_2b8fd;
  v2 = dat_2b8fd;
  if (dat_2b900) { // branch-flip
    v13 = dat_2b038;
    if (v11 == (bool)((unsigned char)((unsigned int)dat_2b900 >> 0x1f) & dat_2b802))
      v13 = dat_2b030;
    v12 = dat_2b068;
    if (a2 != ':')
      v12 = dat_2b060;
    if (v7) goto label_84c2;
    v9 = a0;
    if (!*v13) {
      if (!*v12) goto label_82fd;
      if (v11 == (bool)dat_2b802) goto label_83d0;
label_83b0:
      if (a0 < a1) goto label_84cb;
      goto label_83c0;
    }
    if (v11 == (bool)dat_2b802) goto label_83d0;
    if (*v12) goto label_83b0;
label_83f7:
    a0 = a1;
    if (v9 < a1) {
      v6 = 0;
      if (*(char *)(a1 - 1) == dat_2b684) {
        a0 = a1 - 1;
        if (a0 <= v9) goto label_8430;
        v6 = 1;
      }
      if (*(char *)(a1 + ~v6) == '\r')
        a0 = a1 - (v6 + 1);
    }
label_8430:
    if (1 <= (long)(a0 - v9)) { // branch-flip
      sub_1faa0(dat_2b028,v13);
      sub_7e40(v9,a0 - v9);
      if (*v13)
        sub_1fac0(dat_2b020);
      v2 = dat_2b8fd;
    }
    else {
      a0 = v9; // crossjump-dupe
      v2 = dat_2b8fd;
    }
  }
  else if (v7) {
    v12 = NULL;
    v13 = NULL;
label_84c2:
    if (a1 <= a0) { // branch-flip
label_86b0:
      a0 = a1;
label_83c0:
      if (!a0)
        return;
      v9 = a0;
      v2 = dat_2b8fd;
label_83d0:
      if (v2) goto label_8318;
      v3 = *v13;
    }
    else {
label_84cb:
      v9 = 0;
      v6 = a0;
      do {
        v5 = (*dat_2b878)(dat_2b870,a0,a1 - a0,&v8,v6);
        if ((v5 < 0) || (v1 = a0 + v5, a1 == v1)) break;
        if (v8) { // branch-flip
          if (dat_2b8fd) { // branch-flip
            v3 = sub_8050(v1,v8,a1,(-(dat_2b802 == 0) & 0xdU) + 0x2d);
            if (!v3)
              return;
          }
          else {
            if (*v13)
              sub_1faa0(dat_2b028,v13);
            v10 = v9;
            if (!v9)
              v10 = v6;
            v9 = 0;
            sub_7e40(v10,v1 - v10);
          }
          if ((dat_2b900) && (*v12))
            sub_1faa0(dat_2b028,v12);
          sub_7e40(v1,v8);
          if ((dat_2b900) && (*v12))
            sub_1fac0(dat_2b020);
          if (dat_2b8fd)
            sub_7e70((int)dat_2b684);
          v5 = v8;
        }
        else {
          v5 = 1;
          v8 = 1;
          if (!v9)
            v9 = v6;
        }
        v6 = v1 + v5;
      } while (v6 < a1);
      if (dat_2b8fd) goto label_86b0;
      a0 = v6;
      if (!v9) goto label_83c0;
      v3 = *v13;
    }
    if (v3) goto label_83f7;
    a0 = v9;
    v2 = dat_2b8fd;
  }
label_82fd:
  if ((a0 < a1) && (v2 != 1))
    sub_7e40(a0,a1 - a0);
label_8318:
  if ((dat_2b7d8) && (v4 = fflush_unlocked(stdout), v4)) {
    v4 = *__errno_location();
    dat_2b8c0 = v4;
  }
  else {
    v4 = dat_2b8c0;
  }
  if (!v4) {
    dat_2b7b8 = a1;
    return;
  }
  error(2,v4,dcgettext(NULL,"write error",5));
}

// Function: sub_8710 @ 0x8710
void sub_8710(void *a0) // return-dupe x2
{
  void *v1;
  void *v2; // rax
  
  if (!dat_2b7b8)
    dat_2b7b8 = dat_2b848;
  if (dat_2b7a8 <= 0)
    return;
  do {
    v1 = dat_2b7b8;
    if (a0 <= dat_2b7b8)
      return;
    v2 = rawmemchr(dat_2b7b8,(int)dat_2b684);
    sub_82a0(v1,(long)v2 + 1,0x2d);
    dat_2b7a8 -= 1;
  } while (1 <= dat_2b7a8);
}

// Function: sub_8790 @ 0x8790
long sub_8790(void *a0,void *a1) // early-return
{
  char v1;
  char *v10;
  char v11;
  void *v12;
  long v13;
  void *v14; // stack - 0x70
  long v2;
  int v3;
  long v4; // rax
  void *v5;
  unsigned long v6; // rax
  void *v7;
  void *v8;
  long v9; // stack - 0x48
  
  v2 = dat_2b7b0;
  if (a1 <= a0)
    return 0;
  do {
    v4 = (*dat_2b878)(dat_2b870,a0,(long)a1 - (long)a0,&v9,0);
    v1 = dat_2b684;
    v13 = dat_2b7b0;
    if (0 <= v4) { // branch-flip
      v14 = (void *)((long)a0 + v4);
      v5 = (void *)(v9 + (long)v14);
      if (!dat_2b802) {
        if (a1 != v14) {
          if (!dat_2b803) {
            v7 = v14;
            v14 = v5;
            goto label_8847;
          }
          v14 = v5;
label_8a78:
          dat_2b7b0 -= 1;
          dat_2b7a8 = 0;
          dat_2b828 = (long)v14 + (dat_2b830 - dat_2b840);
          goto label_89b3;
        }
        return v2 - v13; // return-dupe
      }
      if (a0 < v14) {
        goto label_88dc;
      }
    }
    else {
      if ((!dat_2b802) || (v9 = 0, a1 <= a0)) {
        return v2 - v13;
      }
      v5 = a1;
      v14 = a1;
label_88dc:
      if (dat_2b803) { // branch-flip
label_88f3:
        v11 = dat_2b802;
        if (!dat_2b802) goto label_8a78;
label_8911:
        v4 = 0;
        v13 = dat_2b7b0;
        if (a0 < v14) {
          for (; v4 < v13; v4 = v4 + 1) {
            v7 = (void *)((long)rawmemchr(a0,(int)v1) + 1);
            if (!v11) {
              sub_82a0(a0,v7,0x3a);
              v13 = dat_2b7b0;
              v11 = dat_2b803;
            }
            a0 = v7;
            if (v14 <= v7) {
              dat_2b7b0 = v13 - (v4 + 1);
              goto label_8984;
            }
          }
          dat_2b7b0 = v13 - v4;
        }
      }
      else {
        v7 = a0;
label_8847:
        a0 = v7;
        if ((0 < (long)dat_2b7a8) && (sub_8710(v7), dat_2b803)) goto label_88f3;
        v8 = dat_2b7b8;
        if (!dat_2b7b8)
          v8 = dat_2b848;
        v6 = 0;
        v12 = v7; // branch-flip
        if (1 <= (long)dat_2b7f8) {
          do {
            a0 = v12;
            if (v8 < v12) {
              do {
                a0 = (void *)((long)v12 - 1);
                if (v1 == *(char *)((long)v12 - 2)) break;
                a0 = (void *)((long)v12 - 2);
                v10 = (char *)((long)v12 - 3);
                v12 = a0;
              } while (v1 != *v10);
            }
            v6 += 1;
            v12 = a0;
          } while (v6 != dat_2b7f8);
          if (((dat_2b688) && (dat_2b7b8 != a0)) && (dat_2b070)) {
label_8b46:
            v10 = dat_2b070;
            if ((dat_2b900) && (*dat_2b040)) {
              sub_1faa0(dat_2b028);
              v10 = dat_2b070;
            }
            v3 = fputs_unlocked(v10,stdout);
            if (v3 <= -1)
              dat_2b8c0 = *__errno_location();
            if ((dat_2b900) && (*dat_2b040)) {
              sub_1fac0(dat_2b020);
              sub_7e70(10);
            }
            else {
              sub_7e70(10);
            }
          }
          if (a0 < v7) {
            do {
              v8 = (void *)((long)rawmemchr(a0,(int)v1) + 1);
              sub_82a0(a0,v8,0x2d);
              a0 = v8;
            } while (v8 < v7);
          }
        }
        else if ((((0 <= (long)(dat_2b7f8 & dat_2b7f0)) && (dat_2b7b8 != v7)) && (dat_2b688)) && (dat_2b070)) goto label_8b46;
        v11 = dat_2b803;
        if (dat_2b802) goto label_8911;
        if (dat_2b803) goto label_8a78;
        sub_82a0(v7,v14,0x3a);
        dat_2b7b0 -= 1;
        a0 = v14;
        v11 = dat_2b803;
      }
label_8984:
      dat_2b828 = (long)a0 + (dat_2b830 - dat_2b840);
      dat_2b7a8 = 0;
      if (!v11) {
        dat_2b7a8 = dat_2b7f0;
        if ((long)dat_2b7f0 <= -1)
          dat_2b7a8 = 0;
      }
label_89b3:
      dat_2b688 = '\x01';
      if ((!dat_2b7b0) || (dat_2b7a2)) {
        v13 = dat_2b7b0;
        if (dat_2b7a1) {
          v3 = 0;
          if (dat_2b889)
            v3 = dat_2b080;
          exit(v3); // no-return
        }
        return v2 - v13;
      }
    }
    a0 = v5;
    v13 = dat_2b7b0;
    if (a1 <= v5)
      return v2 - v13;
  } while( true );
}

// Function: sub_8cb0 @ 0x8cb0
void sub_8cb0(int a0)
{
  char *v1;
  
  if (dat_2b904) {
    dat_2b889 = 1;
    return;
  }
  v1 = dat_2b890;
  if (!dat_2b890) {
    v1 = dcgettext(NULL,"(standard input)",5);
    dat_2b890 = v1;
  }
  error(0,a0,"%s",v1);
  dat_2b889 = 1;
}

// Function: sub_8d20 @ 0x8d20
unsigned long sub_8d20(unsigned long a0,long a1)
{
  unsigned char v1; // al
  char *v10;
  long v11; // rsi
  long v12;
  char *v13;
  unsigned long v14;
  unsigned int v15;
  unsigned long v16; // r15
  void *v2;
  unsigned long v3;
  unsigned long v4; // rax
  unsigned long v5; // rax
  long v6;
  char *v7;
  long v8; // rbx
  long v9;
  
  v2 = dat_2b860;
  v14 = dat_2b838;
  v9 = dat_2b838 + 8;
  v6 = dat_2b858;
  v13 = dat_2b840;
  if ((long)dat_2b860 + (dat_2b858 - (long)dat_2b840) < v9) {
    v3 = a0 + 1;
    v16 = ~a0;
    v6 = ((dat_2b838 + a0) - dat_2b858) + v9;
    if (1 <= v6) { // branch-flip
      v12 = -1;
      if (((((*(unsigned int *)(a1 + 0x18) & 0xf000) == 0x8000) && (v8 = *(long *)(a1 + 0x30) - dat_2b830, 0 <= v8)) && (v11 = v8 + v9 + a0, !SCARRY8(v8,v9 + a0))) && (v12 = dat_2b858 + v6, dat_2b858 + v6 < v11))
        v12 = v11;
      v2 = (void *)sub_1f1f0(0,0x2b858,v6,v12,1);
      v14 = dat_2b838;
      v13 = (char *)((long)v2 + v3);
      if ((unsigned long)v13 % dat_2b838) { // branch-flip
        v13 = (char *)(((dat_2b838 + 1 + a0) - (unsigned long)v13 % dat_2b838) + (long)v2);
        memmove(&v13[v16],&dat_2b840[v16],v3);
      }
      else {
        memmove(v2,&dat_2b840[v16],v3);
      }
      free(dat_2b860);
      v6 = dat_2b858;
      dat_2b860 = v2;
    }
    else {
      v13 = (char *)((long)dat_2b860 + v3);
      if ((unsigned long)v13 % dat_2b838) { // branch-flip
        v13 = (char *)(((dat_2b838 + 1 + a0) - (unsigned long)v13 % dat_2b838) + (long)dat_2b860);
        memmove(&v13[v16],&dat_2b840[v16],v3);
        v6 = dat_2b858;
      }
      else {
        memmove(dat_2b860,&dat_2b840[v16],v3);
        v6 = dat_2b858;
      }
    }
  }
  v9 = (long)v2 + ((v6 + -8) - (long)v13);
  dat_2b848 = (long)v13 - a0;
  v15 = dat_2b850;
  while (v6 = sub_1da00(v15,v13,v9 - v9 % (long)v14), v15 = dat_2b850, v6 != -1) {
    v10 = &v13[v6];
    v12 = dat_2b830 + v6;
    v1 = dat_2b821 ^ 1 | v6 == 0;
    v3 = (unsigned long)v1;
    dat_2b830 = v12;
    if (v1) goto label_8ea3;
    if (v13 < v10) {
      v7 = v13;
      do {
        if (*v7) {
          v3 = 1;
          goto label_8ea3;
        }
        v7 = &v7[1];
      } while (v10 != v7);
    }
    v8 = v6 + dat_2b818;
    if (SCARRY8(v6,dat_2b818)) {
      error(2,0,dcgettext(NULL,"input is too large to count",5));
      return v4;
    }
    dat_2b818 = v8;
    if (!dat_2b880) {
      v6 = lseek(dat_2b850,v12,3);
      if ((0 <= v6) || (((*__errno_location() == 6 && ((*(unsigned int *)(a1 + 0x18) & 0xf000) == 0x8000)) && ((v12 < *(long *)(a1 + 0x30) && (v6 = lseek(v15,0,2), 0 <= v6)))))) {
        if (SCARRY8(v6 - v12,v8)) {
          error(2,0,dcgettext(NULL,"input is too large to count",5));
          return v5;
        }
        dat_2b818 = (v6 - v12) + v8;
        dat_2b830 = v6;
      }
      else {
        dat_2b880 = '\x01';
      }
    }
  }
  v3 = 0;
  v10 = v13;
label_8ea3:
  v10[0] = '\0';
  v10[1] = '\0';
  v10[2] = '\0';
  v10[3] = '\0';
  v10[4] = '\0';
  v10[5] = '\0';
  v10[6] = '\0';
  v10[7] = '\0';
  dat_2b840 = v10;
  return v3;
}

// Function: sub_9080 @ 0x9080
long sub_9080(long a0,long *a1)
{
  long v1; // rax
  long *v2;
  long v3; // rdx
  
  v2 = dat_2b8f0;
  if (2 <= dat_2b8e0) {
    v1 = 0x18;
    v3 = 1;
    while (*(long *)((long)dat_2b8f0 + v1) <= a0) {
      v3 += 1;
      v1 += 0x18;
      if (v3 == dat_2b8e0) {
        v2 = &dat_2b8f0[v3 * 3 + -3];
        *a1 = (a0 - *v2) + v2[2];
        return v2[1];
      }
    }
    v2 = (long *)((long)dat_2b8f0 + v1 + -0x18);
  }
  *a1 = (a0 - *v2) + v2[2];
  return v2[1];
}

// Function: sub_90f0 @ 0x90f0
void sub_90f0(void)
{
  if (dat_2b890)
    return;
  dat_2b890 = dcgettext(NULL,"(standard input)",5);
}

// Function: sub_9130 @ 0x9130
unsigned long sub_9130(int a0,unsigned char a1)
{
  char *v1;
  unsigned long v10; // rax
  unsigned long v11; // rax
  long v12;
  char *v13;
  long v14; // rax
  int *v15; // rax
  unsigned long v16;
  unsigned long v17; // rax
  long v18; // stack - 0x168
  char *v19; // stack - 0x178
  char *v2;
  char v20 [24];
  char v21;
  unsigned int v22; // edx
  unsigned long v23; // rax
  long v24;
  unsigned char v25; // bl
  char v26;
  char *v27;
  char *v28;
  long v29; // stack - 0x1b8
  unsigned int v3;
  long v30; // stack - 0x1b0
  long v31; // stack - 0x1a8
  char v32; // stack - 0x17b
  unsigned long v33; // stack - 0x170
  long v34; // stack - 0x160
  unsigned int v35; // stack - 0x150
  long v36; // stack - 0x138
  unsigned int v37; // stack - 0xc0
  char v4;
  char v5;
  char v6;
  short v7;
  int v8;
  unsigned int v9;
  
  if (fstat(a0,&v18)) { // branch-flip
    v11 = 1;
    sub_8cb0(*__errno_location());
label_98d2:
    if (!a0) {
      return v11 & 0xffffffff; // return-dupe
    }
label_9454:
    if (!close(a0)) {
      return v11 & 0xffffffff;
    }
  }
  else {
    if (a0) {
      if (dat_2b884 != 2) { // branch-flip
        if ((dat_2b884) || (a1 == 1)) {
label_94a4:
          if (a1) {
            if ((v35 & 0xf000) != 0x4000) // branch-flip
              v12 = 0x2b8b0;
            else {
              if (dat_2b018 == 4) goto label_944e;
              v12 = 0x2b8a0;
              if (dat_2b88a) goto label_94e0;
            }
            if ((*(long *)(v12 + 8)) && (v6 = sub_16b90(*(long *)(v12 + 8),dat_2b890), v6)) goto label_944e;
          }
        }
        else if (((v35 & 0xb000) == 0x2000 || (v35 & 0xf000) == 0xc000) || ((v35 & 0xf000) == 0x1000)) goto label_944e;
label_94e0:
        if (0 <= (int)dat_2b808) { // branch-flip
          if (dat_2b018 == 3) goto label_9a0e;
label_94fd:
          if ((dat_2b018 == 4) && ((v35 & 0xf000) == 0x4000)) goto label_944e;
        }
        else {
          dat_2b808 = (unsigned int)((v35 & 0xf000) == 0x4000);
          if (dat_2b018 != 3) goto label_94fd;
label_9a0e:
          if ((v35 & 0xf000) == 0x4000) {
            v22 = ~(unsigned int)(a1 ^ 1) & dat_2b014;
            if (close(a0))
              sub_8cb0(*__errno_location());
            v33 = 0;
            v19 = dat_2b890;
            v12 = sub_189c0(&v19,v22,0);
            if (!v12)
              sub_1f410(); // no-return
            v11 = 1;
label_9a8d:
            do {
              v14 = sub_18fd0(v12);
              if (!v14) {
                v15 = __errno_location();
                if (*v15)
                  sub_8cb0(*v15);
                if (sub_18de0(v12))
                  sub_8cb0(*v15);
                return v11 & 0xffffffff;
              }
              v7 = *(short *)(v14 + 0x68);
              v25 = *(long *)(v14 + 0x58) == 0 & a1;
            } while (v7 == 6);
            if (!v25) {
              if (((unsigned short)(v7 - 1U) < 2) || (v24 = dat_2b8b0, v7 == 4)) {
                if (dat_2b018 != 4) {
                  v24 = dat_2b8a0;
                  goto label_9f2d;
                }
label_9f64:
                sub_19720(v12,v14,4);
                goto label_9a8d;
              }
label_9f2d:
              if (v24) {
                if (sub_16b90(v24,v14 + 0x100)) goto label_9f64;
                v7 = *(short *)(v14 + 0x68);
              }
            }
            dat_2b890 = *(char **)(v14 + 0x38);
            if ((dat_2b88a) && (dat_2b890[1]))
              dat_2b890 = &dat_2b890[2];
            v13 = dat_2b890;
            v22 = 1;
            if (!(*(unsigned int *)(v12 + 0x48) & 2))
              v22 = *(unsigned int *)(v12 + 0x48) & (unsigned int)v25;
            switch(v7) {
              default:
                goto label_9f9b;
              case 1:
                if (dat_2b018 == 3) goto label_9a8d;
                sub_19720(v12,v14,4);
                break;
              case 2:
                if (!dat_2b904)
                  error(0,0,dcgettext(NULL,"%s: warning: recursive directory loop",5),v13);
                goto label_9a8d;
              case 3:
              case 0xb:
                if ((dat_2b884 == 2) || ((!v25 && (!dat_2b884)))) {
                  v9 = *(unsigned int *)(v14 + 0x88);
                  if (!*(unsigned int *)(v14 + 0x88)) {
                    v13 = *(char **)(v14 + 0x30);
                    v8 = *(int *)(v12 + 0x2c);
                    v9 = v37;
                    if (fstatat(v8,v13,v20,v22 << 8 ^ 0x100)) {
                      sub_8cb0(*__errno_location());
                      goto label_9a8d;
                    }
                  }
                  if (((v9 & 0xb000) == 0x2000 || (v9 & 0xf000) == 0xc000) || ((v9 & 0xf000) == 0x1000)) goto label_9a8d;
                }
                break;
              case 4:
              case 7:
              case 10:
                sub_8cb0(*(unsigned int *)(v14 + 0x40));
                goto label_9a8d;
              case 8:
              case 0xd:
                break;
              case 0xc:
              case 0xe:
                goto label_9a8d;
              
            }
            v16 = *(unsigned long *)(v14 + 0x30);
            v3 = *(unsigned int *)(v12 + 0x2c);
            v11 = (unsigned long)((unsigned int)v11 & sub_a0f0(v3,v16,v22,v25));
            goto label_9a8d;
          }
        }
        if (((dat_2b884 != 2) && ((dat_2b884 || (a1)))) || (((v35 & 0xb000) != 0x2000 && (v35 & 0xf000) != 0xc000 && ((v35 & 0xf000) != 0x1000)))) goto label_9191;
      }
      else if (((v35 & 0xb000) != 0x2000 && (v35 & 0xf000) != 0xc000) && ((v35 & 0xf000) != 0x1000)) goto label_94a4;
label_944e:
      v11 = 1;
      goto label_9454;
    }
    if ((int)dat_2b808 < 0)
      dat_2b808 = (unsigned int)((v35 & 0xf000) == 0x4000);
label_9191:
    v5 = dat_2b803;
    v4 = dat_2b7a2;
    v6 = dat_2b684;
    if ((((!dat_2b803) && (!dat_2b80c)) && (2 <= dat_2b7e0)) && ((((v35 & 0xf000) == 0x8000 && (v34 == dat_2b928)) && (v18 == dat_2b920)))) {
      if (!dat_2b904) {
        v16 = sub_90f0();
        error(0,0,dcgettext(NULL,"%s: input file is also the output",5),v16);
      }
      dat_2b889 = 1;
      v11 = 1;
      goto label_98d2;
    }
    dat_2b840 = (char *)(dat_2b860 + 1);
    if ((unsigned long)dat_2b840 % dat_2b838)
      dat_2b840 = (char *)(dat_2b860 + ((dat_2b838 + 1) - (unsigned long)dat_2b840 % dat_2b838));
    dat_2b840[-1] = dat_2b684;
    dat_2b848 = dat_2b840;
    dat_2b850 = a0;
    if (a0) { // branch-flip
      dat_2b830 = 0;
      dat_2b881 = 0;
      dat_2b880 = 0;
label_9228:
      dat_2b7c8 = NULL;
      dat_2b7b8 = NULL;
      dat_2b7b0 = dat_2b7e0;
      dat_2b821 = v6 == '\0' & dat_2b820;
      dat_2b818 = 0;
      dat_2b828 = 0;
      dat_2b7a8 = 0;
      dat_2b888 = '\0';
      v32 = sub_8d20(0,&v18);
      if (v32) { // branch-flip
        dat_2b8f8 = 0;
        if (dat_2b8fc) {
          v12 = 0x7fffffffffffffff;
          if (((v35 & 0xf000) == 0x8000) && (v12 = v36, dat_2b801))
            v12 = v36 + (unsigned long)(v36 != 0x7fffffffffffffff);
          dat_2b8f8 = 0;
          do {
            dat_2b8f8 += 1;
            v12 /= 10;
          } while (v12);
        }
        v30 = -1;
        v29 = 0;
        v31 = 0;
        v12 = 0;
        v21 = '\0';
        v26 = v32;
        do {
          v28 = dat_2b848;
          v13 = dat_2b840;
          v8 = dat_2b810;
          v25 = (unsigned char)((unsigned long)v30 >> 0x3f) & v6 != '\0';
          v11 = (unsigned long)v25;
          if (v25) {
            if (dat_2b810 != 1) {
              *dat_2b840 = '\0';
              v23 = (long)dat_2b840 - (long)dat_2b848;
              v10 = strlen(dat_2b848);
              if (v23 != v10) { // branch-flip
label_9355:
                if (v8 == 2) {
                  v32 = '\0';
                  v31 = 0;
                  goto label_96ef;
                }
                if (!dat_2b7e8) {
                  dat_2b803 = '\x01';
                  dat_2b7a2 = '\x01';
                }
                dat_2b821 = dat_2b820;
                v30 = v31;
                v28 = dat_2b848;
                v13 = dat_2b840;
                v21 = v6;
              }
              else if (v26) {
                if (!dat_2b881) {
                  if ((v35 & 0xf000) == 0x8000) {
                    if ((long)v23 < v36) {
                      if (!a0) {
                        v23 = lseek(0,0,1);
                        if (0 > (long)v23) goto label_939e;
                      }
                      v14 = lseek(a0,v23,4);
                      if (0 <= v14) {
                        if ((long)lseek(a0,v23,0) < 0)
                          sub_8cb0(*__errno_location());
                        if (v14 < v36) {
                          v8 = dat_2b810;
                          goto label_9355;
                        }
                        v28 = dat_2b848;
                        v13 = dat_2b840;
                      }
                    }
                  }
                }
              }
            }
          }
label_939e:
          if (dat_2b7b8)
            dat_2b7b8 = v28;
          v2 = &v28[v12];
          dat_2b7c0 = v28;
          if (v2 == v13) {
            if (v29) {
              dat_2b840 = &v13[1];
              *v13 = v6;
              if (dat_2b7b0)
                v31 += sub_8790(&v28[v12 - v29]);
              if (dat_2b7a8)
                sub_8710(dat_2b840);
            }
            goto label_96b9;
          }
          if (v21) {
            v27 = v2;
            while( true ) {
              *v13 = '\0';
              v11 = strlen(v27);
              *v13 = v21;
              v28 = &v27[v11];
              if (&v27[v11] == v13) break;
              do {
                v27 = &v28[1];
                *v28 = v21;
                v28 = v27;
              } while (!*v27);
            }
          }
          v26 = v2[-1];
          v2[-1] = v6;
          v12 = memrchr(&v2[-1],(int)v6,&v13[1 - (long)v2]);
          v2[-1] = v26;
          v27 = &v2[-v29];
          v28 = (char *)(v12 + 1);
          if (v2 != v28) { // branch-flip
            v29 = (long)v13 - (long)v28;
            if (v27 < v28) {
              if (!dat_2b7b0) {
                if (dat_2b7a8) goto label_95c2;
label_97dd:
                v32 = '\0';
                goto label_96b9;
              }
              v31 += sub_8790(v27,v28);
              if (dat_2b7a8) { // branch-flip
label_95c2:
                sub_8710(v28);
                if (!dat_2b7b0 && !dat_2b7a8) goto label_97dd;
              }
              else if (!dat_2b7b0) goto label_97dd;
              if (dat_2b7a2) {
                v12 = 0;
                if (0 <= v30)
                  v12 = v30;
                if (v12 < v31) goto label_97dd;
              }
            }
          }
          else {
            v29 = (long)v13 - (long)v27;
            v28 = v27;
          }
          if (1 <= dat_2b7f8) {
            v12 = 0;
            do {
              if (v28 <= dat_2b848) break;
              if (v28 == dat_2b7b8) goto label_964f;
              v12 += 1;
              v27 = v28;
              do {
                v28 = &v27[-1];
                v1 = &v27[-2];
                v27 = v28;
              } while (v6 != *v1);
            } while (v12 != dat_2b7f8);
          }
          if (v28 != dat_2b7b8)
            dat_2b7b8 = NULL;
label_964f:
          v12 = (long)v13 - (long)v28;
          if (dat_2b800) {
            v13 = &(&dat_2b840[-v12 - (long)dat_2b848])[(long)dat_2b7c8];
            if (SCARRY8((long)&dat_2b840[-v12 - (long)dat_2b848],(long)dat_2b7c8)) {
              error(2,0,dcgettext(NULL,"input is too large to count",5));
              return v17;
            }
            dat_2b7c8 = v13;
          }
          if (dat_2b801)
            sub_7c90(v28);
          v26 = '\0';
        } while (sub_8d20(v12,&v18));
        sub_8cb0(*__errno_location());
        v32 = '\0';
label_96b9:
        v11 = CONCAT71((undefined7)((unsigned long)v2 >> 8),v31 == 0);
        dat_2b7a2 = v4;
        dat_2b803 = v5;
        if (((!dat_2b810) && (!v5)) && ((dat_2b888 || ((0 <= v30 && (v30 < v31)))))) {
          v13 = dat_2b890;
          if (!dat_2b890) {
            v13 = dcgettext(NULL,"(standard input)",5);
            dat_2b890 = v13;
          }
          error(0,0,dcgettext(NULL,"%s: binary file matches",5),v13);
        }
      }
      else {
        v11 = 1;
        sub_8cb0(*__errno_location());
        v31 = 0;
      }
    }
    else {
      dat_2b830 = lseek(0,0,1);
      dat_2b880 = (unsigned char)((unsigned long)dat_2b830 >> 0x3f);
      dat_2b881 = dat_2b880;
      if (0 <= dat_2b830) goto label_9228;
      v8 = *__errno_location();
      if (v8 == 0x1d) {
        dat_2b830 = 0;
        goto label_9228;
      }
      sub_8cb0(v8);
      v11 = 1;
      v32 = '\0';
      v31 = 0;
    }
label_96ef:
    if (dat_2b7e8) {
      if (dat_2b808) {
        sub_7fa0();
        if (dat_2b804) // branch-flip
          sub_7ec0(0x3a);
        else {
          sub_7e70(0);
        }
      }
      sub_7b80("%ld\n",v31);
      if ((dat_2b7d8) && (v8 = fflush_unlocked(stdout), v8))
        dat_2b8c0 = *__errno_location();
    }
    if (dat_2b80c) {
      if (dat_2b80c == (v31 == 0) + 1) {
        sub_7fa0();
        sub_7e70(dat_2b804 & 10);
        if ((dat_2b7d8) && (v8 = fflush_unlocked(stdout), v8))
          dat_2b8c0 = *__errno_location();
      }
      goto label_98d2;
    }
    if (a0) goto label_9454;
    if (!dat_2b7b0) {
      if (((dat_2b830 == dat_2b828) || (dat_2b881)) || (v12 = lseek(0,dat_2b828,0), 0 <= v12)) {
        return v11 & 0xffffffff;
      }
      sub_8cb0(*__errno_location()); // return-dupe
      return v11 & 0xffffffff;
    }
    if ((v32) || ((!dat_2b881 && ((0 <= (long)lseek(0,0,2) || (*__errno_location() == 0x16)))))) {
      return v11 & 0xffffffff;
    }
    if (((v35 & 0xf000) != 0x1000) || (!dat_2b7a0)) {
label_9b90:
      do {
        v12 = sub_1da00(0,dat_2b860,dat_2b858);
        if (!v12) {
          return v11 & 0xffffffff;
        }
      } while (v12 != -1);
    }
    else {
      v12 = splice(0,0,1,0,0x18000,1);
      if (0 <= v12) { // branch-flip
        if (!v12) {
          return v11 & 0xffffffff;
        }
        do {
          v12 = splice(0,0,1,0,0x18000,1);
        } while (0 < v12);
        if (!v12) {
          return v11 & 0xffffffff;
        }
      }
      else if (*__errno_location() == 0x16) goto label_9b90;
    }
  }
  sub_8cb0(*__errno_location());
  return v11 & 0xffffffff;
label_9f9b:
  abort(); // no-return
}

// Function: sub_a0f0 @ 0xa0f0
unsigned long sub_a0f0(unsigned long a0,unsigned long a1,char a2,char a3) // early-return
{
  int v1;
  unsigned long v2; // rax
  int *v3; // rax
  unsigned int v4; // edx
  
  v4 = (-(unsigned int)(a2 == '\0') & 0x20000) + 0x100;
  if ((dat_2b884 == 2) || ((!dat_2b884 && (a3 != '\x01'))))
    v4 |= 0x800;
  v1 = sub_1b5d0(a0,a1,v4);
  if (v1 <= -1) {
    v3 = __errno_location();
    v1 = *v3;
    if ((!a2) && (v1 == 0x28 || v1 == 0x1f))
      return CONCAT71((undefined7)((unsigned long)v3 >> 8),v1 == 0x28 || v1 == 0x1f);
    sub_8cb0(v1);
    return 1;
  }
  v2 = sub_9130(v1,a3); // tail-call
  return v2;
}

// Function: sub_a170 @ 0xa170
void sub_a170(int a0)
{
  unsigned long v1; // rax
  
  v1 = sub_19950();
  if (a0) { // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Usage: %s [OPTION]... PATTERNS [FILE]...\n",5),v1);
    v1 = sub_19950();
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v1);
  }
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... PATTERNS [FILE]...\n",5),v1);
    __printf_chk(1,dcgettext(NULL,"Search for PATTERNS in each FILE.\n",5));
    v1 = sub_19950();
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
    sub_1ee20();
  }
  exit(a0); // no-return
}

// Function: sub_a3e0 @ 0xa3e0
void sub_a3e0(unsigned long *a0,unsigned long *a1)
{
  unsigned long v1;
  unsigned char v2;
  unsigned char *v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // stack - 0x48
  unsigned char *v6;
  unsigned long v7;
  unsigned long v8;
  unsigned char *v9; // stack - 0x60
  
  v8 = *a1;
  v6 = (unsigned char *)*a0;
  v5 = 0;
  v3 = (unsigned char *)sub_1f050(v8 + 1,2);
  v9 = v3;
  if (v8) { // branch-flip
    do {
      while( true ) {
        v4 = (unsigned long)*(char *)((unsigned long)*v6 + 0x2b183);
        if (*(char *)((unsigned long)*v6 + 0x2b183) == '\xfe')
          v4 = sub_1ac90(v6,v8,&v5);
        if (v4 != 0xffffffffffffffff) break;
        v5 = 0;
label_a49c:
        v2 = *v6;
        if (((unsigned char)((unsigned int)v2 - 0x24) <= 0x3a) && (0x580000000000441U >> ((unsigned long)((unsigned int)v2 - 0x24) & 0x3f) & 1)) {
          *v9 = 0x5c;
          v2 = *v6;
          v9 = &v9[1];
        }
        v8 -= 1;
        *v9 = v2;
        v9 = &v9[1];
        v6 = &v6[1];
        if (!v8) goto label_a4de;
      }
      if (v4 == 1) goto label_a49c;
      v7 = v8;
      v1 = 0;
      if (v4 != 0xfffffffffffffffe) {
        v7 = v4;
        v1 = v8 - v4;
      }
      v8 = v1;
      v9 = mempcpy(v9,v6,v7);
      v6 = &v6[v7];
    } while (v8);
label_a4de:
    v8 = (long)v9 - (long)v3;
  }
  else {
    v8 = 0;
  }
  *v9 = 10;
  free((void *)*a0);
  *a0 = v3;
  *a1 = v8;
}

// Function: sub_a570 @ 0xa570
void sub_a570(char *a0,long a1)
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
  v2 = sub_e5c0(1);
  v9 = a0;
  do {
    v3 = rawmemchr(v9,10);
    v8 = (long)v3 - (long)v9;
    if (dat_2b685) {
      if ((v9 <= a0) || (dat_2b684 != '\n')) {
        if ((long)v7 <= (long)(v8 + 1)) {
          free(v4);
          v7 = v8;
          v4 = (char *)sub_1f1f0(0,&v7,2,0xffffffffffffffff,1);
          *v4 = dat_2b684;
        }
        v1 = dat_2b684;
        memcpy(&v4[1],v9,v8);
        v4[v8 + 1] = v1;
        v9 = v4;
      }
      else {
        v9 = &v9[-1];
      }
      v8 += 2;
    }
    sub_c9d0(v2,v9,v8);
    v9 = (char *)((long)v3 + 1U);
  } while ((char *)((long)v3 + 1U) <= &a0[a1]);
  free(v4);
  v5 = sub_ceb0(v2);
  sub_cec0(v2);
  v6 = (unsigned long *)sub_1ef10(0x28);
  *v6 = v2;
  v6[2] = a0;
  v6[1] = v5;
  v6[3] = a1;
  v6[4] = 0;
}

// Function: sub_a710 @ 0xa710
long sub_a710(unsigned long *a0,void *a1,long a2,long *a3,void *a4) // return-dupe
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
  
  v4 = dat_2b684;
  v16 = (void *)((long)a1 + a2);
  v1 = *a0;
  v5 = (unsigned int)dat_2b685;
  v8 = dat_2b180 & ((dat_2b182 | dat_2b685) ^ 1);
  v9 = v8 | dat_2b686;
  v10 = v5 ^ 1;
  v11 = a1;
  if (a4)
    v11 = a4;
  if (v16 < v11)
    return -1;
  v15 = v11;
  while( true ) {
    v6 = sub_e3e0(v1,(long)v11 - (unsigned long)v5,(long)v16 + ((unsigned long)v5 - (long)v11),v12,(v9 | a4 != NULL) & v10);
    if (v6 <= -1)
      return -1;
    v6 = (long)v11 + v6;
    v14 = 0;
    v18 = v19 - (int)((unsigned int)dat_2b685 * 2);
    if ((!v8) || (!sub_e640(&v15,&v14,v6,v16))) break;
    v11 = v15;
    if (v16 < v15)
      return -1;
label_a98c:
    v5 = (unsigned int)dat_2b685;
  }
  v11 = v15;
  if ((dat_2b686 == 1) || (!a4)) {
    if (!dat_2b685) {
      if (!dat_2b686) {
        v11 = (void *)(v6 + v18);
label_aa4d:
        if (v11 < v16)
          v16 = (void *)((long)rawmemchr(v11,(int)v4) + 1);
label_aa67:
        v18 = memrchr(a1,(int)v4,v6 - (long)a1);
        if (v18) { // branch-flip
          v6 = (v18 + 1) - (long)a1;
          a1 = (void *)(v18 + 1);
        }
        else {
          v6 = 0;
        }
        v18 = (long)v16 - (long)a1;
        goto label_aa18;
      }
      if (v14) { // branch-flip
        if (v14 <= 0) goto label_a894;
        v7 = sub_e820(v6 - v14,v16);
      }
      else {
        v7 = memrchr(v15,(int)v4,v6 - (long)v15);
        if (v7) {
          v11 = (void *)(v7 + 1);
          v15 = v11;
        }
label_a894:
        v7 = sub_e830(v11,v6,v16);
      }
      if (!v7) {
        while( true ) {
          v11 = (void *)(v6 + v18);
          if (!sub_e820(v11,v16)) {
            if (!a4) goto label_aa4d;
            goto label_aa10;
          }
          if ((!a4) && (!dat_2b180)) break;
          if ((!v18) || (sub_e3e0(v1,v6,v18 + -1,v13,1))) goto label_a970;
          v18 = v20;
        }
        if (!a0[4]) {
          sub_a3e0(&a0[2],&a0[3]);
          v2 = a0[3];
          v3 = a0[2];
          a0[4] = sub_6a70(v3,v2,0x10a46,0);
        }
        v17 = v16;
        if (v11 < v16)
          v17 = (void *)((long)rawmemchr(v11,(int)v4) + 1);
        v2 = a0[4];
        if (0 <= (long)sub_7110(v2,v6,(long)v17 - v6,a3,0)) {
          v16 = v17;
          goto label_aa67;
        }
        v6 = (long)v17 - 1;
      }
label_a970:
      v15 = (void *)(v6 + sub_e810(v6,v16));
      v11 = (void *)((long)v15 + 1);
      if (v16 < (void *)((long)v15 + 1))
        return -1;
      goto label_a98c;
    }
    v18 += (unsigned long)(a4 == NULL);
  }
label_aa10:
  v6 -= (long)a1;
label_aa18:
  *a3 = v18;
  return v6;
}

// Function: sub_ab10 @ 0xab10
void sub_ab10(unsigned long *a0,long a1,unsigned long a2,char a3)
{
  unsigned long *v1;
  unsigned char *v2;
  unsigned long *v3;
  long *v4;
  long *v5;
  unsigned long *v6;
  
  if (!a0)
    return;
  sub_ab10(*a0,a1,a2,a3);
  sub_ab10(a0[1],a1,a2,a3);
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

// Function: sub_abd0 @ 0xabd0
unsigned long sub_abd0(unsigned long *a0,unsigned long *a1) // return-dupe
{
  unsigned long *v1;
  unsigned char *v2;
  unsigned long *v3;
  unsigned long v4; // rax
  
  if (!a1)
    return 1;
  v4 = sub_abd0(a0,*a1);
  if (((char)v4) && (v4 = sub_abd0(a0,a1[1]), (char)v4)) {
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

// Function: sub_ac50 @ 0xac50
unsigned char * sub_ac50(unsigned char *a0,long a1,long a2) // return-dupe
{
  unsigned char *v1;
  int v2;
  unsigned int v3;
  
  v2 = *(int *)(a2 + 0x98c);
  v1 = &a0[a1];
  if (0 <= v2) { // branch-flip
    while( true ) {
      if (v1 <= a0)
        return NULL;
      if (*(long *)(a2 + 0x170 + (unsigned long)*a0 * 8)) break;
      a0 = &a0[1];
      if (!((unsigned long)a0 & 0xf)) {
        v3 = *(unsigned int *)(a2 + 0x988);
        return (unsigned char *)sub_1b140(a0,v3,v2,(long)v1 - (long)a0); // tail-call
      }
    }
  }
  else {
    if (v1 <= a0)
      return NULL;
    while (!*(long *)(a2 + 0x170 + (unsigned long)*a0 * 8)) {
      a0 = &a0[1];
      if (v1 == a0)
        return NULL;
    }
  }
  return a0;
}

// Function: sub_ace0 @ 0xace0
long sub_ace0(long a0,unsigned char *a1,long a2,long *a3,char a4) // return-dupe x4
{
  unsigned char *v1;
  long v10;
  unsigned long v11; // rsi
  long *v12;
  long *v13;
  unsigned char *v14;
  unsigned char *v15; // r13
  int v2;
  long v3;
  unsigned long *v4;
  unsigned long *v5;
  long *v6;
  unsigned char *v7;
  unsigned long *v8;
  unsigned long v9;
  
  if (a2 < *(long *)(a0 + 0x68))
    return -1;
  v15 = &a1[a2];
  v3 = *(long *)(a0 + 0x980);
  v12 = *(long **)(a0 + 0x60);
  v10 = *v12;
  v14 = a1;
  v13 = v12;
  if (!v10) {
    v2 = *(int *)(a0 + 0x988);
    v7 = a1;
    while( true ) {
      if (0 <= v2) { // branch-flip
        v7 = (unsigned char *)sub_ac50(v7,(long)v15 - (long)v7,a0);
        if (!v7)
          return -1;
        v14 = &v7[1];
        v9 = (unsigned long)*v7;
        if (v3)
          v9 = (unsigned long)*(unsigned char *)(v3 + v9);
        v12 = *(long **)(a0 + 0x170 + v9 * 8);
      }
      else {
        while( true ) {
          v14 = &v7[1];
          v9 = (unsigned long)*v7;
          if (v3)
            v9 = (unsigned long)*(unsigned char *)(v3 + v9);
          v12 = *(long **)(a0 + 0x170 + v9 * 8);
          if (v12) break;
          v7 = v14;
          if (v15 <= v14)
            return -1;
        }
      }
      v10 = *v12;
      v13 = v12;
      if (v10) break;
      if (v15 <= v14)
        return -1;
      while( true ) {
        v7 = &v14[1];
        v9 = (unsigned long)*v14;
        if (v3)
          v9 = (unsigned long)*(unsigned char *)(v3 + v9);
        while( true ) {
          v8 = (unsigned long *)v12[1];
          do {
            v1 = (unsigned char *)&v8[3];
            if (*v1 == (unsigned char)v9) {
              v12 = (long *)v8[2];
              goto label_adbd;
            }
            v4 = (unsigned long *)*v8;
            v5 = &v8[1];
            v8 = v4;
            if (*v1 <= (unsigned char)v9)
              v8 = (unsigned long *)*v5;
          } while (v8);
          v12 = (long *)v12[4];
          if (!v12) break;
          v10 = *v12;
          v13 = v12;
          if (v10) goto label_aded;
        }
        v12 = *(long **)(a0 + 0x170 + v9 * 8);
        if (!v12) break;
label_adbd:
        v10 = *v12;
        v14 = v7;
        v13 = v12;
        if (v10) goto label_aded;
        if (v15 == v7)
          return -1;
      }
      if (v15 <= v7)
        return -1;
    }
  }
label_aded:
  while (v10 <= -1) {
    v12 = (long *)v12[4];
    v10 = *v12;
  }
  v10 = v12[5];
  v9 = (long)v14 - v10;
  if ((!a4) || (v15 <= v14)) {
label_ae61:
    v3 = *v12;
    a3[2] = v10;
    a3[1] = v9 - (long)a1;
    *a3 = v3 >> 1;
    return v9 - (long)a1;
  }
  do {
    v11 = (unsigned long)*v14;
    if (v3)
      v11 = (unsigned long)*(unsigned char *)(v3 + v11);
label_ae20:
    v8 = (unsigned long *)v13[1];
label_ae27:
    if (!v8) goto label_aeb8;
    v7 = (unsigned char *)&v8[3];
    if (*v7 != (unsigned char)v11) {
      v5 = (unsigned long *)*v8;
      v4 = &v8[1];
      v8 = v5;
      if (*v7 <= (unsigned char)v11)
        v8 = (unsigned long *)*v4;
      goto label_ae27;
    }
    v13 = (long *)v8[2];
    v14 = &v14[1];
    v10 = *v13;
    v6 = v13;
    if (v10) {
      while (v10 <= -1) {
        v10 = *(long *)v6[4];
        v6 = (long *)v6[4];
      }
      if ((unsigned long)((long)v14 - v6[5]) <= v9) {
        v9 = (long)v14 - v6[5];
        v12 = v6;
      }
    }
    if (v15 == v14) {
      v10 = v12[5];
      goto label_ae61;
    }
  } while( true );
label_aeb8:
  v13 = (long *)v13[4];
  v10 = v12[5];
  if ((!v13) || (v13[5] < v10)) goto label_ae61;
  goto label_ae20;
}

// Function: sub_af60 @ 0xaf60
void sub_af60(void)
{
  sub_ace0(); // tail-call
}

// Function: sub_af70 @ 0xaf70
long sub_af70(long a0,long a1,long a2) // return-dupe
{
  long v1;
  long v10; // rbp
  unsigned char v11;
  long v12; // rdi
  long v13;
  unsigned long v14; // rcx
  unsigned long v15;
  unsigned char v2;
  unsigned char v3;
  long v4;
  long v5;
  long v6;
  long v7;
  unsigned char v8;
  unsigned long v9;
  
  v7 = *(long *)(a0 + 0x68);
  if (!v7)
    return 0;
  v4 = *(long *)(a0 + 0x980);
  if (a2 < v7)
    return -1;
  if (v7 == 1) {
    v7 = sub_ac50(a1,a2,a0);
    if (v7)
      return v7 - a1;
    return -1;
  }
  v1 = a0 + 0x70;
  v2 = *(unsigned char *)(a0 + 0x988);
  v3 = *(unsigned char *)(a0 + 0x990);
  v10 = *(long *)(a0 + 0x970) + v7;
  v9 = a1 + v7;
  if ((SEXT816(v7 * 0xc) == SEXT816(v7) * SEXT816(0xc)) && (v7 * 0xc < a2)) {
    v14 = a2 + v7 * -0xb + a1;
    v15 = v9;
    if (v9 <= v14) {
label_b168:
      v6 = *(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v15 - 1)) + v15;
      v8 = *(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v6 + -1));
      v9 = v6 + (unsigned long)v8;
      if (((!v8) || (v6 = *(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v9 - 1)) + v9, v6 = v6 + (unsigned long)*(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v6 + -1)), v8 = *(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v6 + -1)), v9 = (unsigned long)v8 + v6, !v8)) || (v6 = *(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v9 - 1)) + v9, v6 = v6 + (unsigned long)*(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v6 + -1)), v8 = *(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v6 + -1)), v9 = (unsigned long)v8 + v6, !v8)) {
label_b260:
        v6 = 0;
        v13 = v7;
label_b2ae:
        v8 = *(unsigned char *)(v9 - 2);
        if (v4)
          v8 = *(unsigned char *)(v4 + (unsigned long)v8);
        if (v3 == v8) {
          v12 = -3;
          v5 = 2;
          do {
            v5 += 1;
            if (v13 < v5) {
              v5 = v13 + v6 + 1;
              if (v7 < v5) {
                return (v9 - v7) - a1;
              }
              v15 = ~(v13 + v6);
              goto label_b321;
            }
            v8 = *(unsigned char *)(v9 + v12);
            v11 = *(unsigned char *)(v10 + v12);
            if (v4) {
              v8 = *(unsigned char *)(v4 + (unsigned long)v8);
              v11 = *(unsigned char *)(v4 + (unsigned long)v11);
            }
            v12 -= 1;
          } while (v11 == v8);
          goto label_b347;
        }
        v6 = 0;
        v5 = 2;
        goto label_b277;
      }
      v6 = v9 + *(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v9 - 1));
      v9 = v6 + (unsigned long)*(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v6 + -1));
      if (0x7f < (long)(v9 - v15)) goto label_b378;
      v6 = sub_ac50(v9 - 1,(a1 + a2) - (v9 - 1),a0);
      if (!v6)
        return -1;
      v9 = v6 + 1;
      if (v9 < v14) goto label_b260;
      goto label_affe;
    }
  }
label_affe:
  v15 = (unsigned long)*(unsigned char *)(a0 + 0x70 + (unsigned long)*(unsigned char *)(v9 - 1));
label_b010:
  do {
    if ((long)((a2 + a1) - v9) < (long)v15)
      return -1;
    v9 += v15;
    v8 = *(unsigned char *)(v1 + (unsigned long)*(unsigned char *)(v9 - 1));
    v15 = (unsigned long)v8;
  } while (v8);
  v6 = 0;
  v13 = v7;
label_b071:
  v8 = *(unsigned char *)(v9 - 2);
  if (v4)
    v8 = *(unsigned char *)(v4 + (unsigned long)v8);
  if (v3 == v8) {
    v12 = -3;
    v5 = 2;
    do {
      v5 += 1;
      if (v13 < v5) {
        v5 = v13 + v6 + 1;
        if (v7 < v5) {
          return (v9 - v7) - a1;
        }
        v14 = ~(v13 + v6);
        goto label_b0e9;
      }
      v8 = *(unsigned char *)(v9 + v12);
      v11 = *(unsigned char *)(v10 + v12);
      if (v4) {
        v8 = *(unsigned char *)(v4 + (unsigned long)v8);
        v11 = *(unsigned char *)(v4 + (unsigned long)v11);
      }
      v12 -= 1;
    } while (v11 == v8);
    goto label_b10f;
  }
  v6 = 0;
  v5 = 2;
  goto label_b047;
label_b321:
  v8 = *(unsigned char *)(v9 + v15);
  v11 = *(unsigned char *)(v10 + v15);
  if (v4) {
    v8 = *(unsigned char *)(v4 + (unsigned long)v8);
    v11 = *(unsigned char *)(v4 + (unsigned long)v11);
  }
  if (v11 != v8) goto label_b33e;
  v5 += 1;
  v15 -= 1;
  if (v7 < v5) {
    return (v9 - v7) - a1;
  }
  goto label_b321;
label_b33e:
  if (v7 < v5)
    return (v9 - v7) - a1;
label_b347:
  v6 = v5 * 8 + -0x10;
label_b277:
  v13 = *(long *)(*(long *)(a0 + 0x978) + v6);
  v9 += v13;
  v8 = *(unsigned char *)(v9 - 1);
  v15 = (unsigned long)v8;
  if (v14 < v9) goto label_affe;
  if (v4)
    v8 = *(unsigned char *)(v4 + (unsigned long)v8);
  if (v2 != v8) goto label_b370;
  v6 = v5 + -1;
  goto label_b2ae;
label_b370:
  v9 = *(unsigned char *)(v1 + v15) + v9;
label_b378:
  v15 = v9;
  if (v14 < v9) goto label_affe;
  goto label_b168;
label_b0e9:
  v8 = *(unsigned char *)(v9 + v14);
  v11 = *(unsigned char *)(v10 + v14);
  if (v4) {
    v8 = *(unsigned char *)(v4 + (unsigned long)v8);
    v11 = *(unsigned char *)(v4 + (unsigned long)v11);
  }
  if (v11 != v8) goto label_b106;
  v5 += 1;
  v14 -= 1;
  if (v7 < v5) {
    return (v9 - v7) - a1;
  }
  goto label_b0e9;
label_b106:
  if (v7 < v5)
    return (v9 - v7) - a1;
label_b10f:
  v6 = v5 * 8 + -0x10;
label_b047:
  v13 = *(long *)(*(long *)(a0 + 0x978) + v6);
  v9 += v13;
  if ((unsigned long)(a2 + a1) < v9) goto label_b010;
  v8 = *(unsigned char *)(v9 - 1);
  if (v4)
    v8 = *(unsigned char *)(v4 + (unsigned long)v8);
  if (v2 != v8) goto label_b010;
  v6 = v5 + -1;
  goto label_b071;
}

// Function: sub_b3f0 @ 0xb3f0
void sub_b3f0(long a0,unsigned long a1,unsigned long a2,unsigned long *a3)
{
  unsigned long v1;
  unsigned long v2; // rax
  
  v2 = sub_af70();
  v1 = *(unsigned long *)(a0 + 0x68);
  *a3 = 0;
  a3[1] = v2;
  a3[2] = v1;
}

// Function: sub_b420 @ 0xb420
void sub_b420(long *a0,long *a1)
{
  long *v1;
  long *v2;
  long *v3;
  long *v4;
  long v5;
  long v6;
  
  v1 = (long *)*a0;
  if (v1) {
    v2 = (long *)*v1;
    if (v2) {
      v3 = (long *)*v2;
      if (v3) {
        v4 = (long *)*v3;
        if (v4) {
          if (*v4)
            sub_b420(*v4,a1);
          if (v4[1])
            sub_b420(v4[1],a1);
          v6 = v4[2];
          *(long *)(*a1 + 0x18) = v6;
          *a1 = v6;
        }
        v4 = (long *)v3[1];
        if (v4) { // branch-flip
          if (*v4)
            sub_b420(*v4,a1);
          if (v4[1])
            sub_b420(v4[1],a1);
          v6 = v4[2];
          *(long *)(*a1 + 0x18) = v6;
          *a1 = v6;
        }
        else {
          v6 = *a1;
        }
        v5 = v3[2];
        *(long *)(v6 + 0x18) = v5;
        *a1 = v5;
      }
      v3 = (long *)v2[1];
      if (v3) { // branch-flip
        v4 = (long *)*v3;
        if (v4) {
          if (*v4)
            sub_b420(*v4,a1);
          if (v4[1])
            sub_b420(v4[1],a1);
          v6 = v4[2];
          *(long *)(*a1 + 0x18) = v6;
          *a1 = v6;
        }
        v4 = (long *)v3[1];
        if (v4) { // branch-flip
          if (*v4)
            sub_b420(*v4,a1);
          if (v4[1])
            sub_b420(v4[1],a1);
          v5 = v4[2];
          *(long *)(*a1 + 0x18) = v5;
          *a1 = v5;
        }
        else {
          v5 = *a1;
        }
        v6 = v3[2];
        *(long *)(v5 + 0x18) = v6;
        *a1 = v6;
      }
      else {
        v6 = *a1;
      }
      v5 = v2[2];
      *(long *)(v6 + 0x18) = v5;
      *a1 = v5;
    }
    v2 = (long *)v1[1];
    if (!v2) {
      v6 = v1[2];
      v1 = (long *)a0[1];
      *(long *)(*a1 + 0x18) = v6;
      *a1 = v6;
      goto label_b91e;
    }
    v3 = (long *)*v2;
    if (v3) {
      v4 = (long *)*v3;
      if (v4) {
        if (*v4)
          sub_b420(*v4,a1);
        if (v4[1])
          sub_b420(v4[1],a1);
        v6 = v4[2];
        *(long *)(*a1 + 0x18) = v6;
        *a1 = v6;
      }
      v4 = (long *)v3[1];
      if (v4) { // branch-flip
        if (*v4)
          sub_b420(*v4,a1);
        if (v4[1])
          sub_b420(v4[1],a1);
        v6 = v4[2];
        *(long *)(*a1 + 0x18) = v6;
        *a1 = v6;
      }
      else {
        v6 = *a1;
      }
      v5 = v3[2];
      *(long *)(v6 + 0x18) = v5;
      *a1 = v5;
    }
    v3 = (long *)v2[1];
    if (v3) { // branch-flip
      v4 = (long *)*v3;
      if (v4) {
        if (*v4)
          sub_b420(*v4,a1);
        if (v4[1])
          sub_b420(v4[1],a1);
        v6 = v4[2];
        *(long *)(*a1 + 0x18) = v6;
        *a1 = v6;
      }
      v4 = (long *)v3[1];
      if (v4) { // branch-flip
        if (*v4)
          sub_b420(*v4,a1);
        if (v4[1])
          sub_b420(v4[1],a1);
        v5 = v4[2];
        *(long *)(*a1 + 0x18) = v5;
        *a1 = v5;
      }
      else {
        v5 = *a1;
      }
      v6 = v3[2];
      *(long *)(v5 + 0x18) = v6;
      *a1 = v6;
    }
    else {
      v6 = *a1;
    }
    v5 = v2[2];
    *(long *)(v6 + 0x18) = v5;
    *a1 = v5;
    v6 = v1[2];
    *(long *)(v5 + 0x18) = v6;
    *a1 = v6;
  }
  v1 = (long *)a0[1];
label_b91e:
  if (v1) { // branch-flip
    v2 = (long *)*v1;
    if (v2) {
      v3 = (long *)*v2;
      if (v3) {
        v4 = (long *)*v3;
        if (v4) {
          if (*v4)
            sub_b420(*v4,a1);
          if (v4[1])
            sub_b420(v4[1],a1);
          v6 = v4[2];
          *(long *)(*a1 + 0x18) = v6;
          *a1 = v6;
        }
        v4 = (long *)v3[1];
        if (v4) { // branch-flip
          if (*v4)
            sub_b420(*v4,a1);
          if (v4[1])
            sub_b420(v4[1],a1);
          v6 = v4[2];
          *(long *)(*a1 + 0x18) = v6;
          *a1 = v6;
        }
        else {
          v6 = *a1;
        }
        v5 = v3[2];
        *(long *)(v6 + 0x18) = v5;
        *a1 = v5;
      }
      v3 = (long *)v2[1];
      if (v3) { // branch-flip
        v4 = (long *)*v3;
        if (v4) {
          if (*v4)
            sub_b420(*v4,a1);
          if (v4[1])
            sub_b420(v4[1],a1);
          v6 = v4[2];
          *(long *)(*a1 + 0x18) = v6;
          *a1 = v6;
        }
        v4 = (long *)v3[1];
        if (v4) { // branch-flip
          if (*v4)
            sub_b420(*v4,a1);
          if (v4[1])
            sub_b420(v4[1],a1);
          v5 = v4[2];
          *(long *)(*a1 + 0x18) = v5;
          *a1 = v5;
        }
        else {
          v5 = *a1;
        }
        v6 = v3[2];
        *(long *)(v5 + 0x18) = v6;
        *a1 = v6;
      }
      else {
        v6 = *a1;
      }
      v5 = v2[2];
      *(long *)(v6 + 0x18) = v5;
      *a1 = v5;
    }
    v2 = (long *)v1[1];
    if (v2) { // branch-flip
      v3 = (long *)*v2;
      if (v3) {
        v4 = (long *)*v3;
        if (v4) {
          if (*v4)
            sub_b420(*v4,a1);
          if (v4[1])
            sub_b420(v4[1],a1);
          v6 = v4[2];
          *(long *)(*a1 + 0x18) = v6;
          *a1 = v6;
        }
        v4 = (long *)v3[1];
        if (v4) { // branch-flip
          if (*v4)
            sub_b420(*v4,a1);
          if (v4[1])
            sub_b420(v4[1],a1);
          v6 = v4[2];
          *(long *)(*a1 + 0x18) = v6;
          *a1 = v6;
        }
        else {
          v6 = *a1;
        }
        v5 = v3[2];
        *(long *)(v6 + 0x18) = v5;
        *a1 = v5;
      }
      v3 = (long *)v2[1];
      if (v3) { // branch-flip
        v4 = (long *)*v3;
        if (v4) {
          if (*v4)
            sub_b420(*v4,a1);
          if (v4[1])
            sub_b420(v4[1],a1);
          v6 = v4[2];
          *(long *)(*a1 + 0x18) = v6;
          *a1 = v6;
        }
        v4 = (long *)v3[1];
        if (v4) { // branch-flip
          if (*v4)
            sub_b420(*v4,a1);
          if (v4[1])
            sub_b420(v4[1],a1);
          v5 = v4[2];
          *(long *)(*a1 + 0x18) = v5;
          *a1 = v5;
        }
        else {
          v5 = *a1;
        }
        v6 = v3[2];
        *(long *)(v5 + 0x18) = v6;
        *a1 = v6;
      }
      else {
        v6 = *a1;
      }
      v5 = v2[2];
      *(long *)(v6 + 0x18) = v5;
      *a1 = v5;
    }
    else {
      v5 = *a1;
    }
    v6 = v1[2];
    *(long *)(v5 + 0x18) = v6;
    *a1 = v6;
  }
  else {
    v6 = *a1;
  }
  v5 = a0[2];
  *(long *)(v6 + 0x18) = v5;
  *a1 = v5;
}

// Function: sub_ba10 @ 0xba10
void sub_ba10(long *a0,long a1)
{
  long *v1;
  long *v2;
  long *v3;
  unsigned long *v4;
  long *v5;
  
  if (a0) {
    v1 = (long *)*a0;
    if (v1) {
      v2 = (long *)*v1;
      if (v2) {
        v3 = (long *)*v2;
        if (v3) {
          v4 = (unsigned long *)*v3;
          if (v4) {
            sub_ba10(*v4);
            sub_ba10(v4[1],a1);
            *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
          }
          v5 = (long *)v3[1];
          if (v5) {
            v4 = (unsigned long *)*v5;
            if (v4) {
              sub_ba10(*v4,a1);
              sub_ba10(v4[1],a1);
              *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
            }
            sub_ba10(v5[1],a1);
            *(long *)(a1 + (unsigned long)*(unsigned char *)&v5[3] * 8) = v5[2];
          }
          *(long *)(a1 + (unsigned long)*(unsigned char *)&v3[3] * 8) = v3[2];
        }
        v3 = (long *)v2[1];
        if (v3) {
          v5 = (long *)*v3;
          if (v5) {
            v4 = (unsigned long *)*v5;
            if (v4) {
              sub_ba10(*v4,a1);
              sub_ba10(v4[1],a1);
              *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
            }
            v4 = (unsigned long *)v5[1];
            if (v4) {
              sub_ba10(*v4,a1);
              sub_ba10(v4[1],a1);
              *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
            }
            *(long *)(a1 + (unsigned long)*(unsigned char *)&v5[3] * 8) = v5[2];
          }
          v4 = (unsigned long *)v3[1];
          if (v4) {
            sub_ba10(*v4,a1);
            sub_ba10(v4[1],a1);
            *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
          }
          *(long *)(a1 + (unsigned long)*(unsigned char *)&v3[3] * 8) = v3[2];
        }
        *(long *)(a1 + (unsigned long)*(unsigned char *)&v2[3] * 8) = v2[2];
      }
      v2 = (long *)v1[1];
      if (v2) {
        v3 = (long *)*v2;
        if (v3) {
          v4 = (unsigned long *)*v3;
          if (v4) {
            sub_ba10(*v4,a1);
            sub_ba10(v4[1],a1);
            *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
          }
          v4 = (unsigned long *)v3[1];
          if (v4) {
            sub_ba10(*v4,a1);
            sub_ba10(v4[1],a1);
            *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
          }
          *(long *)(a1 + (unsigned long)*(unsigned char *)&v3[3] * 8) = v3[2];
        }
        v3 = (long *)v2[1];
        if (v3) {
          v5 = (long *)*v3;
          if (v5) {
            v4 = (unsigned long *)*v5;
            if (v4) {
              sub_ba10(*v4,a1);
              sub_ba10(v4[1],a1);
              *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
            }
            v4 = (unsigned long *)v5[1];
            if (v4) {
              sub_ba10(*v4,a1);
              sub_ba10(v4[1],a1);
              *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
            }
            *(long *)(a1 + (unsigned long)*(unsigned char *)&v5[3] * 8) = v5[2];
          }
          v4 = (unsigned long *)v3[1];
          if (v4) {
            sub_ba10(*v4,a1);
            sub_ba10(v4[1],a1);
            *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
          }
          *(long *)(a1 + (unsigned long)*(unsigned char *)&v3[3] * 8) = v3[2];
        }
        *(long *)(a1 + (unsigned long)*(unsigned char *)&v2[3] * 8) = v2[2];
      }
      *(long *)(a1 + (unsigned long)*(unsigned char *)&v1[3] * 8) = v1[2];
    }
    v1 = (long *)a0[1];
    if (v1) {
      v2 = (long *)*v1;
      if (v2) {
        v3 = (long *)*v2;
        if (v3) {
          v4 = (unsigned long *)*v3;
          if (v4) {
            sub_ba10(*v4,a1);
            sub_ba10(v4[1],a1);
            *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
          }
          v4 = (unsigned long *)v3[1];
          if (v4) {
            sub_ba10(*v4,a1);
            sub_ba10(v4[1],a1);
            *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
          }
          *(long *)(a1 + (unsigned long)*(unsigned char *)&v3[3] * 8) = v3[2];
        }
        v3 = (long *)v2[1];
        if (v3) {
          v5 = (long *)*v3;
          if (v5) {
            v4 = (unsigned long *)*v5;
            if (v4) {
              sub_ba10(*v4,a1);
              sub_ba10(v4[1],a1);
              *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
            }
            v4 = (unsigned long *)v5[1];
            if (v4) {
              sub_ba10(*v4,a1);
              sub_ba10(v4[1],a1);
              *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
            }
            *(long *)(a1 + (unsigned long)*(unsigned char *)&v5[3] * 8) = v5[2];
          }
          v5 = (long *)v3[1];
          if (v5) {
            v4 = (unsigned long *)*v5;
            if (v4) {
              sub_ba10(*v4,a1);
              sub_ba10(v4[1],a1);
              *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
            }
            v4 = (unsigned long *)v5[1];
            if (v4) {
              sub_ba10(*v4,a1);
              sub_ba10(v4[1],a1);
              *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
            }
            *(long *)(a1 + (unsigned long)*(unsigned char *)&v5[3] * 8) = v5[2];
          }
          *(long *)(a1 + (unsigned long)*(unsigned char *)&v3[3] * 8) = v3[2];
        }
        *(long *)(a1 + (unsigned long)*(unsigned char *)&v2[3] * 8) = v2[2];
      }
      v2 = (long *)v1[1];
      if (v2) {
        v3 = (long *)*v2;
        if (v3) {
          v5 = (long *)*v3;
          if (v5) {
            v4 = (unsigned long *)*v5;
            if (v4) {
              sub_ba10(*v4,a1);
              sub_ba10(v4[1],a1);
              *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
            }
            v4 = (unsigned long *)v5[1];
            if (v4) {
              sub_ba10(*v4,a1);
              sub_ba10(v4[1],a1);
              *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
            }
            *(long *)(a1 + (unsigned long)*(unsigned char *)&v5[3] * 8) = v5[2];
          }
          v4 = (unsigned long *)v3[1];
          if (v4) {
            sub_ba10(*v4,a1);
            sub_ba10(v4[1],a1);
            *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
          }
          *(long *)(a1 + (unsigned long)*(unsigned char *)&v3[3] * 8) = v3[2];
        }
        v3 = (long *)v2[1];
        if (v3) {
          v4 = (unsigned long *)*v3;
          if (v4) {
            sub_ba10(*v4,a1);
            sub_ba10(v4[1],a1);
            *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
          }
          v5 = (long *)v3[1];
          if (v5) {
            v4 = (unsigned long *)*v5;
            if (v4) {
              sub_ba10(*v4,a1);
              sub_ba10(v4[1],a1);
              *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
            }
            v4 = (unsigned long *)v5[1];
            if (v4) {
              sub_ba10(*v4,a1);
              sub_ba10(v4[1],a1);
              *(unsigned long *)(a1 + (unsigned long)*(unsigned char *)&v4[3] * 8) = v4[2];
            }
            *(long *)(a1 + (unsigned long)*(unsigned char *)&v5[3] * 8) = v5[2];
          }
          *(long *)(a1 + (unsigned long)*(unsigned char *)&v3[3] * 8) = v3[2];
        }
        *(long *)(a1 + (unsigned long)*(unsigned char *)&v2[3] * 8) = v2[2];
      }
      *(long *)(a1 + (unsigned long)*(unsigned char *)&v1[3] * 8) = v1[2];
    }
    *(long *)(a1 + (unsigned long)*(unsigned char *)&a0[3] * 8) = a0[2];
    return;
  }
}

// Function: sub_c100 @ 0xc100
void sub_c100(long *a0,long a1,long a2) // return-dupe
{
  long *v1;
  long *v2;
  long *v3;
  unsigned long *v4;
  long *v5;
  unsigned char v6; // bl
  
  if (!a0)
    return;
  v1 = (long *)*a0;
  v6 = (unsigned char)a1;
  if (v1) {
    v2 = (long *)*v1;
    if (v2) {
      v3 = (long *)*v2;
      if (v3) {
        v4 = (unsigned long *)*v3;
        if (v4) {
          sub_c100(*v4);
          sub_c100(v4[1],a1,a2);
          if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2))
            *(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2) = v6;
        }
        v4 = (unsigned long *)v3[1];
        if (v4) {
          sub_c100(*v4,a1,a2);
          sub_c100(v4[1],a1,a2);
          if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2))
            *(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2) = v6;
        }
        if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v3[3] + a2))
          *(unsigned char *)((unsigned long)*(unsigned char *)&v3[3] + a2) = v6;
      }
      v3 = (long *)v2[1];
      if (v3) {
        v4 = (unsigned long *)*v3;
        if (v4) {
          sub_c100(*v4,a1,a2);
          sub_c100(v4[1],a1,a2);
          if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2))
            *(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2) = v6;
        }
        v4 = (unsigned long *)v3[1];
        if (v4) {
          sub_c100(*v4,a1,a2);
          sub_c100(v4[1],a1,a2);
          if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2))
            *(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2) = v6;
        }
        if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v3[3] + a2))
          *(unsigned char *)((unsigned long)*(unsigned char *)&v3[3] + a2) = v6;
      }
      if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v2[3] + a2))
        *(unsigned char *)((unsigned long)*(unsigned char *)&v2[3] + a2) = v6;
    }
    v2 = (long *)v1[1];
    if (v2) {
      v3 = (long *)*v2;
      if (v3) {
        v4 = (unsigned long *)*v3;
        if (v4) {
          sub_c100(*v4,a1,a2);
          sub_c100(v4[1],a1,a2);
          if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2))
            *(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2) = v6;
        }
        v4 = (unsigned long *)v3[1];
        if (v4) {
          sub_c100(*v4,a1,a2);
          sub_c100(v4[1],a1,a2);
          if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2))
            *(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2) = v6;
        }
        if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v3[3] + a2))
          *(unsigned char *)((unsigned long)*(unsigned char *)&v3[3] + a2) = v6;
      }
      v3 = (long *)v2[1];
      if (v3) {
        v4 = (unsigned long *)*v3;
        if (v4) {
          sub_c100(*v4,a1,a2);
          sub_c100(v4[1],a1,a2);
          if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2))
            *(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2) = v6;
        }
        v4 = (unsigned long *)v3[1];
        if (v4) {
          sub_c100(*v4,a1,a2);
          sub_c100(v4[1],a1,a2);
          if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2))
            *(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2) = v6;
        }
        if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v3[3] + a2))
          *(unsigned char *)((unsigned long)*(unsigned char *)&v3[3] + a2) = v6;
      }
      if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v2[3] + a2))
        *(unsigned char *)((unsigned long)*(unsigned char *)&v2[3] + a2) = v6;
    }
    if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v1[3] + a2))
      *(unsigned char *)((unsigned long)*(unsigned char *)&v1[3] + a2) = v6;
  }
  v1 = (long *)a0[1];
  if (v1) {
    v2 = (long *)*v1;
    if (v2) {
      v3 = (long *)*v2;
      if (v3) {
        v4 = (unsigned long *)*v3;
        if (v4) {
          sub_c100(*v4,a1,a2);
          sub_c100(v4[1],a1,a2);
          if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2))
            *(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2) = v6;
        }
        v4 = (unsigned long *)v3[1];
        if (v4) {
          sub_c100(*v4,a1,a2);
          sub_c100(v4[1],a1,a2);
          if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2))
            *(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2) = v6;
        }
        if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v3[3] + a2))
          *(unsigned char *)((unsigned long)*(unsigned char *)&v3[3] + a2) = v6;
      }
      v3 = (long *)v2[1];
      if (v3) {
        v5 = (long *)*v3;
        if (v5) {
          v4 = (unsigned long *)*v5;
          if (v4) {
            sub_c100(*v4,a1,a2);
            sub_c100(v4[1],a1,a2);
            if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2))
              *(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2) = v6;
          }
          v4 = (unsigned long *)v5[1];
          if (v4) {
            sub_c100(*v4,a1,a2);
            sub_c100(v4[1],a1,a2);
            if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2))
              *(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2) = v6;
          }
          if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v5[3] + a2))
            *(unsigned char *)((unsigned long)*(unsigned char *)&v5[3] + a2) = v6;
        }
        v5 = (long *)v3[1];
        if (v5) {
          v4 = (unsigned long *)*v5;
          if (v4) {
            sub_c100(*v4,a1,a2);
            sub_c100(v4[1],a1,a2);
            if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2))
              *(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2) = v6;
          }
          v4 = (unsigned long *)v5[1];
          if (v4) {
            sub_c100(*v4,a1,a2);
            sub_c100(v4[1],a1,a2);
            if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2))
              *(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2) = v6;
          }
          if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v5[3] + a2))
            *(unsigned char *)((unsigned long)*(unsigned char *)&v5[3] + a2) = v6;
        }
        if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v3[3] + a2))
          *(unsigned char *)((unsigned long)*(unsigned char *)&v3[3] + a2) = v6;
      }
      if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v2[3] + a2))
        *(unsigned char *)((unsigned long)*(unsigned char *)&v2[3] + a2) = v6;
    }
    v2 = (long *)v1[1];
    if (v2) {
      v3 = (long *)*v2;
      if (v3) {
        v4 = (unsigned long *)*v3;
        if (v4) {
          sub_c100(*v4,a1,a2);
          sub_c100(v4[1],a1,a2);
          if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2))
            *(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2) = v6;
        }
        v4 = (unsigned long *)v3[1];
        if (v4) {
          sub_c100(*v4,a1,a2);
          sub_c100(v4[1],a1,a2);
          if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2))
            *(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2) = v6;
        }
        if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v3[3] + a2))
          *(unsigned char *)((unsigned long)*(unsigned char *)&v3[3] + a2) = v6;
      }
      v3 = (long *)v2[1];
      if (v3) {
        v4 = (unsigned long *)*v3;
        if (v4) {
          sub_c100(*v4,a1,a2);
          sub_c100(v4[1],a1,a2);
          if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2))
            *(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2) = v6;
        }
        v4 = (unsigned long *)v3[1];
        if (v4) {
          sub_c100(*v4,a1,a2);
          sub_c100(v4[1],a1,a2);
          if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2))
            *(unsigned char *)((unsigned long)*(unsigned char *)&v4[3] + a2) = v6;
        }
        if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v3[3] + a2))
          *(unsigned char *)((unsigned long)*(unsigned char *)&v3[3] + a2) = v6;
      }
      if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v2[3] + a2))
        *(unsigned char *)((unsigned long)*(unsigned char *)&v2[3] + a2) = v6;
    }
    if (a1 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v1[3] + a2))
      *(unsigned char *)((unsigned long)*(unsigned char *)&v1[3] + a2) = v6;
  }
  if ((long)(unsigned long)*(unsigned char *)(a2 + (unsigned long)*(unsigned char *)&a0[3]) <= a1)
    return;
  *(unsigned char *)(a2 + (unsigned long)*(unsigned char *)&a0[3]) = v6;
}

// Function: sub_c880 @ 0xc880
long sub_c880(unsigned long a0)
{
  long v1; // rax
  unsigned long *v2; // rax
  unsigned long v3; // rax
  unsigned long v4;
  unsigned long *v5;
  
  v1 = sub_1ef10(0x9a0);
  _obstack_begin(v1,0,0,sub_1ef10,dat_2afb8);
  v4 = *(unsigned long *)(v1 + 0x20);
  *(unsigned long *)(v1 + 0x58) = 0;
  if (0x40 <= v4 - *(long *)(v1 + 0x18)) { // branch-flip
    v5 = *(unsigned long **)(v1 + 0x10);
    v2 = (unsigned long *)(*(long *)(v1 + 0x18) + 0x40);
    if (v2 != v5) goto label_c8ea;
  }
  else {
    _obstack_newchunk(v1,0x40);
    v5 = *(unsigned long **)(v1 + 0x10);
    v4 = *(unsigned long *)(v1 + 0x20);
    v2 = (unsigned long *)(*(long *)(v1 + 0x18) + 0x40);
    if (v2 != v5) goto label_c8ea;
  }
  *(unsigned char *)(v1 + 0x50) = *(unsigned char *)(v1 + 0x50) | 2;
label_c8ea:
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
  *(void **)(v1 + 0x998) = sub_af60;
  *(unsigned long *)(v1 + 0x970) = 0;
  return v1;
}

// Function: sub_c9d0 @ 0xc9d0
void sub_c9d0(long a0,unsigned char *a1,long a2) // return-dupe
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
  if (v2 == sub_b3f0)
    a1 = &a1[a2];
label_ca25:
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
      if (v2 != sub_b3f0) { // branch-flip
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
        if (!v10) goto label_cabc;
        v12 = v18;
      }
      v7 = (long *)v10[2];
    }
    v12 = 0;
label_cabc:
    v16 = *(unsigned long *)(a0 + 0x20);
    if (0x20 <= v16 - *(long *)(a0 + 0x18)) { // branch-flip
      v19 = *(unsigned long **)(a0 + 0x10);
      v9 = (unsigned long *)(*(long *)(a0 + 0x18) + 0x20);
      if (v9 == v19) {
label_cdc7:
        *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 2;
      }
    }
    else {
      _obstack_newchunk(a0,0x20);
      v19 = *(unsigned long **)(a0 + 0x10);
      v16 = *(unsigned long *)(a0 + 0x20);
      v9 = (unsigned long *)(*(long *)(a0 + 0x18) + 0x20);
      if (v9 == v19) goto label_cdc7;
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
      if (v16 - (long)v10 <= 0x3f) goto label_cb2f;
      v11 = &v10[8];
      if (v11 == v10) goto label_cd6f;
    }
    else {
      *(unsigned long *)(a0 + 0x18) = v16;
      *(unsigned long *)(a0 + 0x10) = v16;
      *v19 = 0;
      v19[1] = 0;
label_cb2f:
      _obstack_newchunk(a0,0x40);
      v20 = *(unsigned long *)(a0 + 0x30);
      v16 = *(unsigned long *)(a0 + 0x20);
      v22 = ~v20;
      v11 = (long *)(*(long *)(a0 + 0x18) + 0x40);
      v18 = *(long *)(a0 + 8);
      v21 = v16 - v18;
      v10 = *(long **)(a0 + 0x10);
      if (v11 == v10) {
label_cd6f:
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
    if (!v12) goto label_ca25;
    v17 = v13[v12];
  }
  if (v17) { // branch-flip
    if (v17 != 1) goto label_ca25;
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
  goto label_ca25;
}

// Function: sub_ceb0 @ 0xceb0
unsigned long sub_ceb0(long a0)
{
  return *(unsigned long *)(a0 + 0x58);
}

// Function: sub_cec0 @ 0xcec0
void sub_cec0(unsigned long *a0) // return-dupe
{
  unsigned long *v1;
  char v10; // al
  unsigned int v11;
  unsigned long v12;
  long *v13;
  long *v14;
  long v15; // rax
  long v16;
  long v17;
  unsigned long *v18;
  unsigned long v19;
  unsigned long *v2;
  long v20;
  long *v21;
  unsigned long v22;
  unsigned long v23; // rcx
  long *v24;
  long *v25; // rdx
  long *v26; // rdx
  long *v27; // rdx
  long *v28; // rdx
  long *v29;
  long v3;
  unsigned long v30 [33];
  long *v31; // stack - 0x950
  unsigned long v32 [256];
  unsigned int v33;
  unsigned int v34;
  long v35;
  long *v36;
  unsigned long *v37;
  unsigned long *v38;
  long *v39;
  unsigned long v4;
  long *v40;
  unsigned int v41; // r10d
  long **v42;
  unsigned long *v43;
  unsigned char v44;
  unsigned long v5;
  char v6 [16];
  char v7 [16];
  char v8 [16];
  char v9 [16];
  
  v3 = a0[0x130];
  v43 = v30;
  if (!v3)
    v43 = &a0[0xe];
  v16 = a0[0xb];
  if (v16 == 1) {
    v42 = &v31;
    v24 = (long *)a0[0xc];
    v39 = v24;
    v31 = v24;
    for (; v24; v24 = (long *)v24[3]) {
      v40 = (long *)v24[1];
      if (v40) {
        v21 = (long *)*v40;
        if (v21) {
          if (*v21)
            sub_b420(*v21,v42);
          if (v21[1])
            sub_b420(v21[1],v42);
          v21 = (long *)v21[2];
          v31[3] = (long)v21;
          v31 = v21;
        }
        v21 = (long *)v40[1];
        v29 = v31;
        if (v21) {
          if (*v21)
            sub_b420(*v21,v42);
          if (v21[1])
            sub_b420(v21[1],v42);
          v29 = (long *)v21[2];
          v31[3] = (long)v29;
        }
        v31 = (long *)v40[2];
        v29[3] = (long)v31;
      }
    }
    v12 = a0[4];
    v22 = a0[0xd];
    if (v22 <= v12 - a0[3]) { // branch-flip
      v35 = a0[2];
      v17 = a0[3] + v22;
      if (v17 == v35) {
label_e3d2:
        *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] | 2;
      }
    }
    else {
      _obstack_newchunk(a0,v22);
      v35 = a0[2];
      v12 = a0[4];
      v39 = (long *)a0[0xc];
      v17 = a0[3] + v22;
      v22 = a0[0xd];
      if (v17 == v35) goto label_e3d2;
    }
    a0[0x12e] = v35;
    v19 = v17 + a0[6] & ~a0[6];
    if (v19 - a0[1] <= v12 - a0[1])
      v12 = v19;
    a0[3] = v12;
    a0[2] = v12;
    if (1 <= (long)v22) {
      v17 = 0;
      while( true ) {
        *(char *)(v35 + v17) = *(char *)(v39[1] + 0x18);
        v17 += 1;
        v39 = (long *)v39[3];
        if ((long)a0[0xd] <= v17) break;
        v35 = a0[0x12e];
      }
    }
    v18 = (unsigned long *)sub_c880(a0[0x130]);
    v4 = a0[0xd];
    v5 = a0[0x12e];
    v18[0x133] = sub_b3f0;
    sub_c9d0(v18,v5,v4);
    _obstack_free(a0,0);
    v35 = 0x134;
    v37 = v18;
    v38 = a0;
    while (v35) {
      v2 = &v38[1];
      v1 = &v37[1];
      *v38 = *v37;
      v35 -= 1;
      v37 = v1;
      v38 = v2;
    }
    free(v18);
    v12 = a0[0xd];
    if (0x100 <= (long)v12)
      v12 = 0xff;
    v12 &= 0xff;
    v8._8_8_ = 0;
    v8._0_8_ = v12;
    *v43 = SUB168(v8._0_16_ * ZEXT816(0x101010101010101),0);
    v35 = v12 * 0x101010101010101 + SUB168(v8._0_16_ * ZEXT816(0x101010101010101),8);
    v43[1] = v35;
    v43[3] = v35;
    v43[2] = v12 * 0x101010101010101;
    v43[5] = v35;
    v43[7] = v35;
    v43[9] = v35;
    v43[4] = v12 * 0x101010101010101;
    v43[0xb] = v35;
    v43[0xd] = v35;
    v43[0xf] = v35;
    v43[6] = v12 * 0x101010101010101;
    v43[0x11] = v35;
    v43[0x13] = v35;
    v43[0x15] = v35;
    v43[8] = v12 * 0x101010101010101;
    v43[0x17] = v35;
    v43[0x19] = v35;
    v43[0x1b] = v35;
    v43[10] = v12 * 0x101010101010101;
    v43[0x1d] = v35;
    v43[0xc] = v12 * 0x101010101010101;
    v43[0xe] = v12 * 0x101010101010101;
    v43[0x10] = v12 * 0x101010101010101;
    v43[0x12] = v12 * 0x101010101010101;
    v43[0x14] = v12 * 0x101010101010101;
    v43[0x16] = v12 * 0x101010101010101;
    v43[0x18] = v12 * 0x101010101010101;
    v43[0x1a] = v12 * 0x101010101010101;
    v43[0x1c] = v12 * 0x101010101010101;
    v9._8_8_ = 0;
    v9._0_8_ = v12;
    v24 = SUB168(v9._0_16_ * ZEXT816(0x101010101010101),8);
    v43[0x1e] = SUB168(v9._0_16_ * ZEXT816(0x101010101010101),0);
    v43[0x1f] = v35;
    v39 = (long *)a0[0xc];
    if (v39) goto label_d084;
    v35 = 0;
    v31 = v39;
label_e1bb:
    for (v35 = *(long *)(v35 + 0x18); v35; v35 = *(long *)(v35 + 0x18)) {
      v17 = *(long *)(*(long *)(v35 + 0x10) + 0x38);
      v20 = *(long *)(v35 + 0x38);
      if (v17 < *(long *)(v35 + 0x38)) {
        *(long *)(v35 + 0x38) = v17;
        v20 = v17;
      }
      if (v20 < *(long *)(v35 + 0x30))
        *(long *)(v35 + 0x30) = v20;
    }
  }
  else {
    v12 = a0[0xd];
    if (0x100 <= (long)v12)
      v12 = 0xff;
    v12 &= 0xff;
    v6._8_8_ = 0;
    v6._0_8_ = v12;
    *v43 = SUB168(v6._0_16_ * ZEXT816(0x101010101010101),0);
    v35 = v12 * 0x101010101010101 + SUB168(v6._0_16_ * ZEXT816(0x101010101010101),8);
    v43[1] = v35;
    v43[3] = v35;
    v43[2] = v12 * 0x101010101010101;
    v43[5] = v35;
    v43[7] = v35;
    v43[9] = v35;
    v43[4] = v12 * 0x101010101010101;
    v43[0xb] = v35;
    v43[0xd] = v35;
    v43[0xf] = v35;
    v43[6] = v12 * 0x101010101010101;
    v43[0x11] = v35;
    v43[0x13] = v35;
    v43[0x15] = v35;
    v43[8] = v12 * 0x101010101010101;
    v43[0x17] = v35;
    v43[0x19] = v35;
    v43[0x1b] = v35;
    v43[10] = v12 * 0x101010101010101;
    v43[0x1d] = v35;
    v43[0xc] = v12 * 0x101010101010101;
    v43[0xe] = v12 * 0x101010101010101;
    v43[0x10] = v12 * 0x101010101010101;
    v43[0x12] = v12 * 0x101010101010101;
    v43[0x14] = v12 * 0x101010101010101;
    v43[0x16] = v12 * 0x101010101010101;
    v43[0x18] = v12 * 0x101010101010101;
    v43[0x1a] = v12 * 0x101010101010101;
    v43[0x1c] = v12 * 0x101010101010101;
    v7._8_8_ = 0;
    v7._0_8_ = v12;
    v24 = SUB168(v7._0_16_ * ZEXT816(0x101010101010101),8);
    v43[0x1e] = SUB168(v7._0_16_ * ZEXT816(0x101010101010101),0);
    v43[0x1f] = v35;
    v39 = (long *)a0[0xc];
    v31 = v39;
    if (v39) {
label_d084:
      v31 = v39;
label_dc63:
      do {
        v37 = (unsigned long *)v39[1];
        if (v37) {
          v40 = (long *)*v37;
          if (v40) { // branch-flip
            v21 = (long *)*v40;
            if (v21) {
              v29 = (long *)*v21;
              if (v29) {
                v13 = (long *)*v29;
                if (v13) {
                  if (*v13) {
                    sub_b420(*v13,&v31,v24);
                    v24 = v25;
                  }
                  if (v13[1])
                    sub_b420(v13[1],&v31,v24);
                  v24 = (long *)v13[2];
                  v31[3] = (long)v24;
                  v31 = v24;
                }
                v24 = (long *)v29[1];
                v13 = v31;
                if (v24) {
                  if (*v24)
                    sub_b420(*v24,&v31);
                  if (v24[1])
                    sub_b420(v24[1],&v31);
                  v13 = (long *)v24[2];
                  v31[3] = (long)v13;
                }
                v31 = (long *)v29[2];
                v13[3] = (long)v31;
              }
              v24 = (long *)v21[1];
              if (v24) {
                v29 = (long *)*v24;
                if (v29) {
                  if (*v29)
                    sub_b420(*v29,&v31);
                  if (v29[1])
                    sub_b420(v29[1],&v31);
                  v29 = (long *)v29[2];
                  v31[3] = (long)v29;
                  v31 = v29;
                }
                v29 = (long *)v24[1];
                v13 = v31;
                if (v29) {
                  if (*v29)
                    sub_b420(*v29,&v31);
                  if (v29[1])
                    sub_b420(v29[1],&v31);
                  v13 = (long *)v29[2];
                  v31[3] = (long)v13;
                }
                v31 = (long *)v24[2];
                v13[3] = (long)v31;
              }
              v24 = (long *)v21[2];
              v31[3] = (long)v24;
              v31 = v24;
            }
            v21 = (long *)v40[1];
            v24 = v31;
            if (v21) {
              v24 = (long *)*v21;
              if (v24) {
                v29 = (long *)*v24;
                if (v29) {
                  if (*v29)
                    sub_b420(*v29,&v31);
                  if (v29[1])
                    sub_b420(v29[1],&v31);
                  v29 = (long *)v29[2];
                  v31[3] = (long)v29;
                  v31 = v29;
                }
                v29 = (long *)v24[1];
                v13 = v31;
                if (v29) {
                  if (*v29)
                    sub_b420(*v29,&v31);
                  if (v29[1])
                    sub_b420(v29[1],&v31);
                  v13 = (long *)v29[2];
                  v31[3] = (long)v13;
                }
                v31 = (long *)v24[2];
                v13[3] = (long)v31;
              }
              v24 = (long *)v21[1];
              if (v24) {
                v29 = (long *)*v24;
                if (v29) {
                  if (*v29)
                    sub_b420(*v29,&v31);
                  if (v29[1])
                    sub_b420(v29[1],&v31);
                  v29 = (long *)v29[2];
                  v31[3] = (long)v29;
                  v31 = v29;
                }
                v29 = (long *)v24[1];
                v13 = v31;
                if (v29) {
                  if (*v29)
                    sub_b420(*v29,&v31);
                  if (v29[1])
                    sub_b420(v29[1],&v31);
                  v13 = (long *)v29[2];
                  v31[3] = (long)v13;
                }
                v31 = (long *)v24[2];
                v13[3] = (long)v31;
              }
              v24 = (long *)v21[2];
              v31[3] = (long)v24;
            }
            v21 = (long *)v40[2];
            v29 = (long *)v37[1];
            v24[3] = (long)v21;
            v31 = v21;
            if (v29) { // branch-flip
label_d327:
              v21 = (long *)*v29;
              if (v21) {
                v13 = (long *)*v21;
                if (v13) {
                  v14 = (long *)*v13;
                  if (v14) {
                    if (*v14) {
                      sub_b420(*v14,&v31,v24);
                      v24 = v26;
                    }
                    if (v14[1])
                      sub_b420(v14[1],&v31,v24);
                    v24 = (long *)v14[2];
                    v31[3] = (long)v24;
                    v31 = v24;
                  }
                  v24 = (long *)v13[1];
                  v14 = v31;
                  if (v24) {
                    if (*v24)
                      sub_b420(*v24,&v31);
                    if (v24[1])
                      sub_b420(v24[1],&v31);
                    v14 = (long *)v24[2];
                    v31[3] = (long)v14;
                  }
                  v31 = (long *)v13[2];
                  v14[3] = (long)v31;
                }
                v24 = (long *)v21[1];
                if (v24) {
                  v13 = (long *)*v24;
                  if (v13) {
                    if (*v13)
                      sub_b420(*v13,&v31);
                    if (v13[1])
                      sub_b420(v13[1],&v31);
                    v13 = (long *)v13[2];
                    v31[3] = (long)v13;
                    v31 = v13;
                  }
                  v13 = (long *)v24[1];
                  v14 = v31;
                  if (v13) {
                    if (*v13)
                      sub_b420(*v13,&v31);
                    if (v13[1])
                      sub_b420(v13[1],&v31);
                    v14 = (long *)v13[2];
                    v31[3] = (long)v14;
                  }
                  v31 = (long *)v24[2];
                  v14[3] = (long)v31;
                }
                v24 = (long *)v21[2];
                v31[3] = (long)v24;
                v31 = v24;
              }
              v24 = (long *)v29[1];
              if (v24) {
                v13 = (long *)*v24;
                if (v13) {
                  v14 = (long *)*v13;
                  if (v14) {
                    if (*v14)
                      sub_b420(*v14,&v31);
                    if (v14[1])
                      sub_b420(v14[1],&v31);
                    v14 = (long *)v14[2];
                    v31[3] = (long)v14;
                    v31 = v14;
                  }
                  v14 = (long *)v13[1];
                  if (v14) {
                    if (*v14)
                      sub_b420(*v14,&v31);
                    if (v14[1])
                      sub_b420(v14[1],&v31);
                    v14 = (long *)v14[2];
                    v31[3] = (long)v14;
                    v31 = v14;
                  }
                  v13 = (long *)v13[2];
                  v31[3] = (long)v13;
                  v31 = v13;
                }
                v13 = (long *)v24[1];
                v14 = v31;
                if (v13) {
                  v14 = (long *)*v13;
                  if (v14) {
                    if (*v14)
                      sub_b420(*v14,&v31);
                    if (v14[1])
                      sub_b420(v14[1],&v31);
                    v14 = (long *)v14[2];
                    v31[3] = (long)v14;
                    v31 = v14;
                  }
                  v14 = (long *)v13[1];
                  v36 = v31;
                  if (v14) {
                    if (*v14)
                      sub_b420(*v14,&v31);
                    if (v14[1])
                      sub_b420(v14[1],&v31);
                    v36 = (long *)v14[2];
                    v31[3] = (long)v36;
                  }
                  v14 = (long *)v13[2];
                  v36[3] = (long)v14;
                }
                v31 = (long *)v24[2];
                v14[3] = (long)v31;
              }
              v17 = v29[2];
              v35 = v39[5];
              v31[3] = v17;
              v31 = (long *)v37[2];
              *(long **)(v17 + 0x18) = v31;
              if (v40) goto label_d613;
            }
            else {
              v31 = (long *)v37[2];
              v35 = v39[5];
              v21[3] = (long)v31;
label_d613:
              v24 = (long *)*v40;
              v44 = (unsigned char)v35;
              if (v24) {
                v21 = (long *)*v24;
                if (v21) {
                  v38 = (unsigned long *)*v21;
                  if (v38) {
                    sub_c100(*v38,v35,v43);
                    sub_c100(v38[1],v35,v43);
                    if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                  }
                  v38 = (unsigned long *)v21[1];
                  if (v38) {
                    sub_c100(*v38,v35,v43);
                    sub_c100(v38[1],v35,v43);
                    if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                  }
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v21[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v21[3] + (long)v43) = v44;
                }
                v21 = (long *)v24[1];
                if (v21) {
                  v38 = (unsigned long *)*v21;
                  if (v38) {
                    sub_c100(*v38,v35,v43);
                    sub_c100(v38[1],v35,v43);
                    if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                  }
                  v38 = (unsigned long *)v21[1];
                  if (v38) {
                    sub_c100(*v38,v35,v43);
                    sub_c100(v38[1],v35,v43);
                    if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                  }
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v21[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v21[3] + (long)v43) = v44;
                }
                if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v24[3] + (long)v43))
                  *(unsigned char *)((unsigned long)*(unsigned char *)&v24[3] + (long)v43) = v44;
              }
              v24 = (long *)v40[1];
              if (v24) {
                v21 = (long *)*v24;
                if (v21) {
                  v38 = (unsigned long *)*v21;
                  if (v38) {
                    sub_c100(*v38,v35,v43);
                    sub_c100(v38[1],v35,v43);
                    if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                  }
                  v38 = (unsigned long *)v21[1];
                  if (v38) {
                    sub_c100(*v38,v35,v43);
                    sub_c100(v38[1],v35,v43);
                    if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                  }
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v21[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v21[3] + (long)v43) = v44;
                }
                v21 = (long *)v24[1];
                if (v21) {
                  v38 = (unsigned long *)*v21;
                  if (v38) {
                    sub_c100(*v38,v35,v43);
                    sub_c100(v38[1],v35,v43);
                    if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                  }
                  v38 = (unsigned long *)v21[1];
                  if (v38) {
                    sub_c100(*v38,v35,v43);
                    sub_c100(v38[1],v35,v43);
                    if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                      *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                  }
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v21[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v21[3] + (long)v43) = v44;
                }
                if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v24[3] + (long)v43))
                  *(unsigned char *)((unsigned long)*(unsigned char *)&v24[3] + (long)v43) = v44;
              }
              if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v40[3] + (long)v43))
                *(unsigned char *)((unsigned long)*(unsigned char *)&v40[3] + (long)v43) = v44;
              v29 = (long *)v37[1];
              if (!v29) goto label_dc21;
              v21 = (long *)*v29;
            }
            v44 = (unsigned char)v35;
            if (v21) {
              v24 = (long *)*v21;
              if (v24) {
                v38 = (unsigned long *)*v24;
                if (v38) {
                  sub_c100(*v38,v35,v43);
                  sub_c100(v38[1],v35,v43);
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                }
                v38 = (unsigned long *)v24[1];
                if (v38) {
                  sub_c100(*v38,v35,v43);
                  sub_c100(v38[1],v35,v43);
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                }
                if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v24[3] + (long)v43))
                  *(unsigned char *)((unsigned long)*(unsigned char *)&v24[3] + (long)v43) = v44;
              }
              v24 = (long *)v21[1];
              if (v24) {
                v38 = (unsigned long *)*v24;
                if (v38) {
                  sub_c100(*v38,v35,v43);
                  sub_c100(v38[1],v35,v43);
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                }
                v38 = (unsigned long *)v24[1];
                if (v38) {
                  sub_c100(*v38,v35,v43);
                  sub_c100(v38[1],v35,v43);
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                }
                if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v24[3] + (long)v43))
                  *(unsigned char *)((unsigned long)*(unsigned char *)&v24[3] + (long)v43) = v44;
              }
              if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v21[3] + (long)v43))
                *(unsigned char *)((unsigned long)*(unsigned char *)&v21[3] + (long)v43) = v44;
            }
            v24 = (long *)v29[1];
            if (v24) {
              v40 = (long *)*v24;
              if (v40) {
                v38 = (unsigned long *)*v40;
                if (v38) {
                  sub_c100(*v38,v35,v43);
                  sub_c100(v38[1],v35,v43);
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                }
                v38 = (unsigned long *)v40[1];
                if (v38) {
                  sub_c100(*v38,v35,v43);
                  sub_c100(v38[1],v35,v43);
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                }
                if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v40[3] + (long)v43))
                  *(unsigned char *)((unsigned long)*(unsigned char *)&v40[3] + (long)v43) = v44;
              }
              v40 = (long *)v24[1];
              if (v40) {
                v38 = (unsigned long *)*v40;
                if (v38) {
                  sub_c100(*v38,v35,v43);
                  sub_c100(v38[1],v35,v43);
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                }
                v38 = (unsigned long *)v40[1];
                if (v38) {
                  sub_c100(*v38,v35,v43);
                  sub_c100(v38[1],v35,v43);
                  if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43))
                    *(unsigned char *)((unsigned long)*(unsigned char *)&v38[3] + (long)v43) = v44;
                }
                if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v40[3] + (long)v43))
                  *(unsigned char *)((unsigned long)*(unsigned char *)&v40[3] + (long)v43) = v44;
              }
              if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v24[3] + (long)v43))
                *(unsigned char *)((unsigned long)*(unsigned char *)&v24[3] + (long)v43) = v44;
            }
            if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v29[3] + (long)v43))
              *(unsigned char *)((unsigned long)*(unsigned char *)&v29[3] + (long)v43) = v44;
          }
          else {
            v29 = (long *)v37[1];
            if (v29) goto label_d327;
            v24 = (long *)v37[2];
            v35 = v39[5];
            v31[3] = (long)v24;
            v31 = v24;
          }
label_dc21:
          if (v35 < (long)(unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)&v37[3] + (long)v43))
            *(unsigned char *)((unsigned long)*(unsigned char *)&v37[3] + (long)v43) = (unsigned char)v35;
          v37 = (unsigned long *)v39[1];
        }
        v35 = a0[0xc];
        sub_ab10(v37,v39[4],v35,v16 == 1);
        if (v16 == 1) {
          v17 = a0[0xd];
          v20 = v39[4];
          v39[6] = v17;
          v39[7] = v17;
          if (v20) {
            v17 = *v39;
            do {
              v10 = sub_abd0(*(unsigned long *)(v20 + 8),v37);
              if ((!v10) && (v15 = v39[5] - *(long *)(v20 + 0x28), v15 < *(long *)(v20 + 0x30)))
                *(long *)(v20 + 0x30) = v15;
              if ((v17) && (v15 = v39[5] - *(long *)(v20 + 0x28), v15 < *(long *)(v20 + 0x38)))
                *(long *)(v20 + 0x38) = v15;
              v20 = *(long *)(v20 + 0x20);
            } while (v20);
            v39 = (long *)v39[3];
            v24 = v28;
            if (!v39) break;
            goto label_dc63;
          }
        }
        v39 = (long *)v39[3];
        v24 = v27;
      } while (v39);
      if (v16 == 1) goto label_e1bb;
    }
  }
  v12 = 0xffffffff;
  v37 = v32;
  if (!v3)
    v37 = &a0[0x2e];
  v41 = 0xffffffff;
  *v37 = 0;
  v37[0xff] = 0;
  v22 = (unsigned long)(((int)v37 - (int)(unsigned long *)((unsigned long)&v37[1] & 0xfffffffffffffff8)) + 0x800U >> 3);
  v38 = (unsigned long *)((unsigned long)&v37[1] & 0xfffffffffffffff8);
  while (v22) {
    v18 = &v38[1];
    *v38 = 0;
    v22 -= 1;
    v38 = v18;
  }
  sub_ba10(*(unsigned long *)(a0[0xc] + 8),v37);
  v22 = 0;
  v33 = 0xfffffffe;
  do {
    while( true ) {
      v11 = (unsigned int)v22;
      v19 = v22 & 0xffffffff;
      if (v3) { // branch-flip
        v23 = (unsigned long)*(unsigned char *)(v3 + v22);
        v35 = v37[v23];
        a0[v22 + 0x2e] = v35;
      }
      else {
        v35 = a0[v22 + 0x2e];
        v23 = v22 & 0xffffffff;
      }
      v34 = v33;
      if (v35) break;
label_ddd0:
      v11 = (unsigned int)v12;
      v22 += 1;
      v33 = v34;
      if (v22 == 0x100) goto label_de21;
    }
    v34 = (unsigned int)v23;
    if (v33 != 0xfffffffe) {
      if (v34 != v33) { // branch-flip
        if ((v34 != v11) || (v33 != (unsigned int)v12))
          v34 = 0xffffffff;
        else {
          v12 = v19;
          v34 = v33;
        }
      }
      else {
        if (v33 != (unsigned int)v12)
          v11 = v41;
        v12 = (unsigned long)v11;
        v34 = v33;
      }
      goto label_ddd0;
    }
    v22 += 1;
    v12 = v19;
    v33 = v34;
  } while (v22 != 0x100);
label_de21:
  *(unsigned int *)&a0[0x131] = v34;
  *(unsigned int *)((long)a0 + 0x98c) = v11;
  if (v16 != 1) goto label_de3a;
  v12 = a0[4];
  v22 = a0[0xd];
  if (v22 <= v12 - a0[3]) { // branch-flip
    v16 = a0[2];
    v35 = a0[3] + v22;
    v19 = v22;
    if (v35 == v16) {
label_e39e:
      *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] | 2;
    }
  }
  else {
    _obstack_newchunk(a0);
    v16 = a0[2];
    v12 = a0[4];
    v19 = a0[0xd];
    v35 = a0[3] + v22;
    if (v35 == v16) goto label_e39e;
  }
  a0[0x12e] = v16;
  v22 = v35 + a0[6] & ~a0[6];
  if (v22 - a0[1] <= v12 - a0[1])
    v12 = v22;
  a0[3] = v12;
  a0[2] = v12;
  v35 = a0[0xc];
  if (1 <= (long)v19) {
    while( true ) {
      v19 -= 1;
      *(char *)(v16 + v19) = *(char *)(*(long *)(v35 + 8) + 0x18);
      v35 = *(long *)(v35 + 0x18);
      if (!v19) break;
      v16 = a0[0x12e];
    }
    v16 = a0[0xd];
    if (2 <= v16) {
      v22 = a0[4];
      v35 = a0[3];
      v12 = v16 * 8 - 8;
      if (v22 - v35 < v12) {
        _obstack_newchunk(a0,v12);
        v35 = a0[3];
        v22 = a0[4];
        v16 = a0[0xd];
      }
      v17 = a0[2];
      if (v35 + v12 == v17)
        *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] | 2;
      a0[0x12f] = v17;
      v12 = v35 + v12 + a0[6] & ~a0[6];
      if (v12 - a0[1] <= v22 - a0[1])
        v22 = v12;
      a0[3] = v22;
      a0[2] = v22;
      v35 = *(long *)(a0[0xc] + 0x18);
      if (2 <= v16) {
        v20 = 0;
        do {
          v37 = (unsigned long *)(v35 + 0x30);
          v35 = *(long *)(v35 + 0x18);
          *(unsigned long *)(v17 + v20 * 8) = *v37;
          v16 = a0[0xd];
          v20 += 1;
        } while (v20 < v16 + -1);
      }
      v44 = *(unsigned char *)(a0[0x12e] + -2 + v16);
      if (v3)
        v44 = *(unsigned char *)(v3 + (unsigned long)v44);
      *(unsigned char *)&a0[0x132] = v44;
    }
  }
label_de3a:
  v16 = 0;
  if (v3) {
    do {
      *(char *)((long)a0 + v16 + 0x70) = *(char *)((long)v43 + (unsigned long)*(unsigned char *)(v3 + v16));
      v16 += 1;
    } while (v16 != 0x100);
    return;
  }
}

// Function: sub_e3e0 @ 0xe3e0
void sub_e3e0(long a0)
{
  (**(void **)(a0 + 0x998))(); // jump-as-call
}

// Function: sub_e400 @ 0xe400
void sub_e400(void *a0)
{
  _obstack_free(a0,0);
  free(a0); // tail-call
}

// Function: sub_e420 @ 0xe420
long sub_e420(long a0,long a1,char a2)
{
  unsigned long v1; // rax
  long v2; // rax
  int v3; // stack - 0x4c
  unsigned long v4; // stack - 0x48
  long v5; // r14
  
  v5 = 0;
  v4 = 0;
  do {
    if (a1 - a0 <= v5)
      return v5;
    v1 = (unsigned long)*(unsigned char *)(a0 + v5);
    if (*(char *)(v1 + 0x2b9c0)) // branch-flip
      v5 += 1;
    else {
      if (*(char *)(v1 + 0x2b183) != '\xfe')
        return v5;
      v3 = 0;
      v2 = sub_1acc0(&v3,(unsigned char *)(a0 + v5),(a1 - a0) - v5,&v4);
      if ((v3 != 0x5f) && (!iswalnum()))
        return v5;
      v5 = v5 + v2 + (unsigned long)(v2 == 0);
    }
    if (!a2)
      return v5;
  } while( true );
}

// Function: sub_e520 @ 0xe520
unsigned int sub_e520(unsigned int a0)
{
  return a0;
}

// Function: sub_e530 @ 0xe530
void sub_e530(void)
{
  sub_1ac90(); // tail-call
}

// Function: sub_e540 @ 0xe540
void sub_e540(unsigned char *a0)
{
  if (*(char *)((unsigned long)*a0 + 0x2b183) != '\xfe')
    return;
  sub_1ac90(); // tail-call
}

// Function: sub_e570 @ 0xe570
void sub_e570(void)
{
  int *v1; // rbx
  char *v2; // rbp
  bool v3; // al
  
  v2 = (char *)0x2b9c0;
  v1 = (int *)0x2b284;
  do {
    v3 = 1;
    if (*v1 != 0x5f)
      v3 = iswalnum() != 0;
    v1 = &v1[1];
    *v2 = v3;
    v2 = &v2[1];
  } while (v1 != (int *)0x2b684);
}

// Function: sub_e5c0 @ 0xe5c0
void sub_e5c0(char a0)
{
  long v1;
  long v2; // rax
  long v3; // rax
  
  if (!dat_2b687) {
    sub_c880(0); // tail-call
    return;
  }
  if ((__ctype_get_mb_cur_max() != 1) && (!a0)) {
    sub_c880(0); // tail-call
    return;
  }
  v2 = sub_1ef30(0x100);
  v1 = *(long *)__ctype_toupper_loc();
  v3 = 0;
  do {
    *(char *)(v2 + v3) = (char)*(unsigned int *)(v1 + v3 * 4);
    v3 += 1;
  } while (v3 != 0x100);
  sub_c880(v2); // tail-call
}

// Function: sub_e640 @ 0xe640
long sub_e640(unsigned long *a0,unsigned long *a1,unsigned char *a2,long a3) // early-return, return-dupe
{
  unsigned int v1; // eax
  unsigned char *v2;
  unsigned long v3; // rax
  unsigned long v4;
  long v5; // rdx
  unsigned char *v6;
  unsigned long v7; // stack - 0x48
  unsigned char *v8; // r14
  
  v6 = (unsigned char *)*a0;
  if (v6 < a2) {
    if (dat_2b182) { // branch-flip
      if (((int)(char)*a2 & 0xc0U) != 0x80) {
label_e6ea:
        *a0 = a2;
        return 0;
      }
      v1 = (unsigned int)(char)a2[-1];
      if ((v1 & 0xc0) != 0x80) // branch-flip
        v5 = 1;
      else {
        v1 = (unsigned int)(char)a2[-2];
        if ((v1 & 0xc0) != 0x80) // branch-flip
          v5 = 2;
        else {
          v1 = (unsigned int)(char)a2[-3];
          if ((v1 & 0xc0) == 0x80) goto label_e6ea;
          v5 = 3;
        }
      }
      if ((int)(~v1 & 0xff) >> (7U - (char)v5 & 0x1fU)) goto label_e6ea;
      v7 = 0;
      v8 = &a2[-v5];
      v3 = sub_1ac90(v8,a3 - (long)v8,&v7);
      if ((v3 <= 0x10) || (v2 = a2, 0 <= (long)v3)) {
        v2 = &v8[v3];
        v6 = v8;
      }
    }
    else {
      v7 = 0;
      while( true ) {
        v3 = (unsigned long)*(char *)((unsigned long)*v6 + 0x2b183);
        if (((*(char *)((unsigned long)*v6 + 0x2b183) != '\xfe') || (v3 = sub_1ac90(v6,a3 - (long)v6,&v7), v4 = v3, 0x11 <= v3)) && (v4 = v3, (long)v3 <= -1)) {
          v7 = 0;
          v4 = 1;
          v2 = &v6[1];
        }
        else {
          v2 = &v6[v3];
        }
        if (a2 <= v2) break;
        v6 = v2;
      }
      if (a1)
        *a1 = v4;
    }
    *a0 = v2;
    if (v2 == a2)
      return 0;
  }
  return (long)a2 - (long)v6;
}

// Function: sub_e810 @ 0xe810
void sub_e810(unsigned long a0,unsigned long a1)
{
  sub_e420(a0,a1,1); // tail-call
}

// Function: sub_e820 @ 0xe820
void sub_e820(unsigned long a0,unsigned long a1)
{
  sub_e420(a0,a1,0); // tail-call
}

// Function: sub_e830 @ 0xe830
unsigned long sub_e830(long a0,long a1,unsigned long a2) // early-return
{
  unsigned long v1; // rax
  long v2; // stack - 0x28
  
  if (a0 == a1)
    return 0;
  if ((dat_2b180) && (!(~(*(unsigned char *)(a1 + -1) >> 7) & dat_2b182))) {
    v2 = a0;
    v1 = sub_e420((a1 + -1) - sub_e640(&v2,0,a1 + -1,a2),a2,0); // tail-call
    return v1;
  }
  return (unsigned long)*(unsigned char *)((unsigned long)*(unsigned char *)(a1 + -1) + 0x2b9c0);
}

// Function: sub_e8f0 @ 0xe8f0
void sub_e8f0(void *a0)
{
  free(a0); // tail-call
}

// Function: sub_e900 @ 0xe900
void sub_e900(long a0)
{
  if (0 <= a0) {
    sub_1ef30(); // tail-call
    return;
  }
  sub_1f410(); // no-return
}

// Function: sub_e920 @ 0xe920
int sub_e920(unsigned long *a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int a4)
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
        sub_1f410(); // no-return
      v3 = a0[2];
      if (!v3) {
        v3 = pcre2_match_context_create_8(*a0);
        v2 = a0[4];
        a0[2] = v3;
      }
      v6 = 0xea52;
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
    v6 = 0xe988;
    pcre2_set_depth_limit_8(v2,v1);
  }
  return 0xffffffcb;
}

// Function: sub_ea80 @ 0xea80
unsigned long * sub_ea80(char *a0,unsigned long a1)
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
  
  v15 = dat_2b687 == '\0';
  v1 = (unsigned long *)sub_1ef30(0x38);
  v14 = (-(unsigned int)v15 & 0xfffffff8) + 0x18;
  v2 = pcre2_general_context_create_8(sub_e900,sub_e8f0,0);
  *v1 = v2;
  v3 = pcre2_compile_context_create_8(v2);
  if (dat_2b180) {
    if (!dat_2b182) {
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
  if (dat_2b685) { // branch-flip
    pcre2_set_compile_extra_options_8(v3,8);
    v17 = NULL;
  }
  else {
    v17 = NULL;
    if (dat_2b686) {
      v16 = a1 + 0x11;
      v17 = (char *)sub_1ef30(v16);
      *(unsigned long *)v17 = s_22990._0_8_;
      *(unsigned short *)&v17[8] = s_22990._8_2_;
      v6 = mempcpy(&v17[10],a0,a1);
      *(unsigned int *)v6 = s_22988._0_4_;
      *(unsigned short *)&v6[4] = s_22988._4_2_;
      v6[6] = s_22988[6];
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
    *(unsigned int *)&v1[6] = sub_e920(v1,"",0,0,1);
    *(unsigned int *)((long)v1 + 0x34) = sub_e920(v1,"",0,0,0);
    return v1;
  }
  error(2,0,dcgettext(NULL,"JIT internal error: %d",5),v5 & 0xffffffff);
  return v7;
}

// Function: sub_eda0 @ 0xeda0
unsigned char * sub_eda0(long a0,unsigned char *a1,long a2,long *a3,unsigned char *a4)
{
  int v1;
  unsigned char *v10; // stack - 0x68
  long *v2;
  unsigned long v3;
  unsigned char *v4; // rax
  long v5; // rcx
  unsigned char *v6;
  unsigned int v7; // ebp
  unsigned char *v8;
  unsigned char *v9;
  
  v6 = a1;
  if (a4)
    v6 = a4;
  v7 = (unsigned int)(v6[-1] == dat_2b684);
  v4 = (long *)pcre2_get_ovector_pointer_8(*(unsigned long *)(a0 + 0x18));
  v2 = (long *)v4;
  v10 = a1;
  while( true ) {
    v4 = rawmemchr(v6,(int)(char)dat_2b684);
    v8 = v4;
    v4 = (unsigned long)(unsigned long)*v6;
    if (*(char *)((long)v4 + 0x2b183) != '\xff') { // branch-flip
      v5 = (long)v6 - (long)v10;
      v9 = v10;
    }
    else {
      do {
        v4 = (unsigned long)(unsigned long)v6[1];
        v6 = &v6[1];
      } while (*(char *)((long)v4 + 0x2b183) == '\xff');
      v5 = 0;
      v7 = 0;
      v9 = v6;
    }
    if (v8 != v6) // branch-flip
      v1 = sub_e920(a0,v9,(long)v8 - (long)v9,v5,v7 ^ 1);
    else {
      v1 = *(int *)(a0 + 0x30 + (unsigned long)v7 * 4);
      v2[1] = v5;
      *v2 = v5;
    }
    if (v1 != -1) break;
    v6 = &v8[1];
    v4 = &a1[a2];
    v7 = 1;
    if (v4 <= v6) {
      v4 = (unsigned char *)0xffffffffffffffff;
      return v4;
    }
    v10 = v6;
  }
  if (1 <= v1) {
    v8 = &v8[1];
    if (a4) {
      v10 = &v9[*v2];
      v8 = &v9[v2[1]];
    }
    v4 = (long)((long)v10 - (long)a1);
    *a3 = (long)v8 - (long)v10;
    return v4;
  }
  if ((v1 < -0x2d) && (-0x3f <= v1)) {
    switch(v1) {
      case 0xffffffcb:
label_f044:
        v4 = (unsigned long)sub_90f0();
        v3 = v4;
        v4 = (char *)dcgettext(NULL,"%s: exceeded PCRE\'s nested backtracking limit",5);
        error(2,0,(char *)v4,v3);
        goto label_f073;
      case 0xffffffc1:
        v4 = (unsigned long)sub_90f0();
        v3 = v4;
        v4 = (char *)dcgettext(NULL,"%s: exceeded PCRE\'s heap limit",5);
        error(2,0,(char *)v4,v3);
      case 0xffffffcc:
label_f015:
        v4 = (unsigned long)sub_90f0(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
        v3 = v4;
        v4 = (char *)dcgettext(NULL,"%s: PCRE detected recurse loop",5);
        error(2,0,(char *)v4,v3);
        goto label_f044;
      case 0xffffffd0:
label_efe6:
        v4 = (unsigned long)sub_90f0();
        v3 = v4;
        v4 = (char *)dcgettext(NULL,"%s: memory exhausted",5);
        error(2,0,(char *)v4,v3);
        goto label_f015;
      case 0xffffffd1:
label_efb7:
        v4 = (unsigned long)sub_90f0();
        v3 = v4;
        v4 = (char *)dcgettext(NULL,"%s: exceeded PCRE\'s backtracking limit",5);
        error(2,0,(char *)v4,v3);
        goto label_efe6;
      case 0xffffffd2:
        v4 = (unsigned long)sub_90f0();
        v3 = v4;
        v4 = (char *)dcgettext(NULL,"%s: exhausted PCRE JIT stack",5);
        error(2,0,(char *)v4,v3);
        goto label_efb7;
      
    }
  }
  v4 = (unsigned long)sub_90f0();
  v3 = v4;
  v4 = (char *)dcgettext(NULL,"%s: internal PCRE error: %d",5);
  error(2,0,(char *)v4,v3,v1);
  return v4;
}

// Function: sub_f0b0 @ 0xf0b0
void sub_f0b0(void)
{
  sub_a170(1); // tail-call, no-return
}

// Function: sub_f0c0 @ 0xf0c0
unsigned long sub_f0c0(char *a0,long *a1,void *a2,unsigned long a3)
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

// Function: sub_f1d0 @ 0xf1d0
long sub_f1d0(char *a0,long *a1) // return-dupe
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

// Function: sub_f230 @ 0xf230
void sub_f230(unsigned long a0,unsigned long a1,long a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (a2 != -1) // branch-flip
    v1 = dcgettext(NULL,"ambiguous argument %s for %s",5);
  else {
    v1 = dcgettext(NULL,"invalid argument %s for %s",5);
  }
  v2 = sub_1d9c0(1,a0);
  error(0,0,v1,sub_1d1c0(0,8,a1),v2); // tail-call
}

// Function: sub_f2c0 @ 0xf2c0
void sub_f2c0(long *a0,void *a1,unsigned long a2)
{
  long v1;
  char *v2;
  void *v3;
  long v4;
  void *v5;
  
  v3 = stderr;
  v5 = NULL;
  fputs_unlocked(dcgettext(NULL,"Valid arguments are:",5),v3);
  v4 = 0;
  v1 = *a0;
  while (v1) {
    if ((v4) && (!memcmp(v5,a1,a2)))
      __fprintf_chk(stderr,1,", %s",sub_1d9e0(v1));
    else {
      __fprintf_chk(stderr,1,"\n  - %s",sub_1d9e0(v1));
      v5 = a1;
    }
    v4 += 1;
    a1 = (void *)((long)a1 + a2);
    v1 = a0[v4];
  }
  v2 = *(char **)((long)stderr + 0x28);
  if (v2 < *(char **)((long)stderr + 0x30)) {
    *(char **)((long)stderr + 0x28) = &v2[1];
    *v2 = 10;
    return;
  }
  __overflow(stderr,10); // tail-call
}

// Function: sub_f3f0 @ 0xf3f0
long sub_f3f0(unsigned long a0,char *a1,unsigned long *a2,unsigned long a3,unsigned long a4,void *a5,char a6) // early-return
{
  char *v1;
  long v2;
  
  if (a6) { // branch-flip
    v2 = sub_f0c0(a1,a2,a3,a4);
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
  sub_f230(a0,a1,v2);
  sub_f2c0(a2,a3,a4);
  (*a5)();
  return -1;
}

// Function: sub_f4c0 @ 0xf4c0
long sub_f4c0(void *a0,long *a1,void *a2,unsigned long a3) // return-dupe
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

// Function: sub_f520 @ 0xf520
void sub_f520(void)
{
  return;
}

// Function: sub_f530 @ 0xf530
void sub_f530(int a0)
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
  (*dat_2bae0)();
  if (a0) // branch-flip
    v6 = dat_2bad8;
  else {
    v6 = dat_2bad0;
  }
  v1 = strlen(dat_2bac8);
  v2 = strlen(v6);
  if (0x1000 <= v1 + v2) { // branch-flip
    write(2,dat_2bac8,v1);
    write(2,": ",2);
    write(2,v6,v2);
    v5[0] = 10;
    v4 = (char *)0x1;
  }
  else {
    v3 = __mempcpy_chk(v5,dat_2bac8,v1,0x1002);
    *(unsigned short *)v3 = s_22a11._0_2_;
    v4 = mempcpy(&v3[2],v6,v2);
    *v4 = 10;
    v4 = &v4[1 - (long)v5];
  }
  write(2,v5,(unsigned long)v4);
  if (!a0)
    _exit(dat_2b080); // no-return
  raise(a0);
  abort(); // no-return
}

// Function: sub_f650 @ 0xf650
unsigned long sub_f650(unsigned long a0,int a1)
{
  if (!a1)
    return 0;
  sub_f530(0xb); // no-return
}

// Function: sub_f670 @ 0xf670
void sub_f670(unsigned long a0)
{
  if ((int)a0)
    a0 = (unsigned long)(-(unsigned int)(dat_2bac0 == 0) & 0xb);
  sub_f530(a0); // no-return
}

// Function: sub_f6a0 @ 0xf6a0
unsigned long sub_f6a0(void *a0) // return-dupe
{
  if (!a0)
    a0 = sub_f520;
  dat_2bae0 = a0;
  dat_2bad8 = dcgettext(NULL,"program error",5);
  dat_2bad0 = dcgettext(NULL,"stack overflow",5);
  dat_2bac8 = sub_19950();
  if (!sub_1de00(sub_f670,0x2bb00,0x10000)) {
    dat_2bac0 = sub_1dd80(sub_f650);
    return 0;
  }
  *__errno_location() = 0x5f;
  return 0xffffffff;
}

// Function: sub_f750 @ 0xf750
int sub_f750(long a0,long a1)
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
label_f76d:
        v5 += 0x20;
        v2 += 0x20;
      }
      if (!v1) break;
    }
    else {
      v1 += 0x20;
      v4 += 0x20;
      if (v5 - 0x41 <= 0x19) goto label_f76d;
    }
    v3 += 1;
  } while (v4 == v2);
  return v1 - v5;
}

// Function: sub_f7c0 @ 0xf7c0
void sub_f7c0(unsigned long a0)
{
  dat_3bb08 = a0;
}

// Function: sub_f7d0 @ 0xf7d0
void sub_f7d0(char a0)
{
  dat_3bb00 = a0;
}

// Function: sub_f7e0 @ 0xf7e0
void sub_f7e0(void)
{
  int v1; // eax
  int *v2; // rax
  char *v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_1fb60(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!dat_3bb00) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (dat_3bb08) { // branch-flip
        v4 = sub_1d550(dat_3bb08);
        error(0,*v2,"%s: %s",v4,v3);
      }
      else {
        error(0,*v2,"%s",v3);
      }
      _exit(dat_2b080); // return-dupe, no-return
    }
  }
  v1 = sub_1fb60(stderr);
  if (!v1)
    return;
  _exit(dat_2b080);
}

// Function: sub_f890 @ 0xf890
unsigned int sub_f890(long a0,long *a1) // return-dupe
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

// Function: sub_f930 @ 0xf930
int sub_f930(long *a0,long *a1)
{
  return (unsigned int)(*a1 < *a0) - (unsigned int)(*a0 < *a1);
}

// Function: sub_f950 @ 0xf950
unsigned long sub_f950(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,char *a5)
{
  *a5 = 1;
  return a1;
}

// Function: sub_f960 @ 0xf960
void sub_f960(int a0,long a1)
{
  unsigned long *v1;
  long v2;
  unsigned long v3; // rcx
  
  v2 = *(long *)__ctype_toupper_loc();
  if (a0 + 0x80U <= 0x17f)
    a0 = *(int *)(v2 + (long)a0 * 4);
  v3 = 0;
  do {
    if (a0 == *(int *)(v2 + v3 * 4)) {
      v1 = (unsigned long *)(a1 + (v3 >> 6 & 0x3ffffff) * 8);
      *v1 = *v1 | 1L << ((unsigned char)v3 & 0x3f);
    }
    v3 += 1;
  } while (v3 != 0x100);
}

// Function: sub_f9c0 @ 0xf9c0
void sub_f9c0(long a0,unsigned long a1,char a2) // return-dupe x2
{
  long v1;
  long v2;
  unsigned long v3; // rax
  
  v1 = *(long *)(a0 + 0x98);
  if (*(long *)(a0 + 0xa0) != v1) { // branch-flip
label_f9e1:
    if (*(char *)(a0 + 0x418)) {
      *(char *)(*(long *)(a0 + 0x118) + v1) = a2;
      v2 = *(long *)(a0 + 0x90);
      goto label_fa00;
    }
    v2 = *(long *)(a0 + 0x90);
  }
  else {
    v2 = sub_1f1f0(*(unsigned long *)(a0 + 0x90),a0 + 0xa0,1,0xffffffffffffffff,8);
    *(long *)(a0 + 0x90) = v2;
    if (*(char *)(a0 + 0x418)) {
      v3 = sub_1efd0(*(unsigned long *)(a0 + 0x118),*(unsigned long *)(a0 + 0xa0),1);
      v1 = *(long *)(a0 + 0x98);
      *(unsigned long *)(a0 + 0x118) = v3;
      goto label_f9e1;
    }
label_fa00:
    v1 = *(long *)(a0 + 0x98);
  }
  *(long *)(a0 + 0x98) = v1 + 1;
  *(unsigned long *)(v2 + v1 * 8) = a1;
  switch(a1) {
    case 0x100:
      *(char *)(a0 + 0xc1) = 1;
      goto label_fa47;
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
label_fa47:
  v1 = *(long *)(a0 + 0x88) + 1;
  *(long *)(a0 + 0x88) = v1;
  if (*(long *)(a0 + 0xa8) < v1) {
    *(long *)(a0 + 0xa8) = v1;
    return;
  }
}

// Function: sub_fb20 @ 0xfb20
void sub_fb20(long a0,long a1) // return-dupe
{
  long v1;
  long v2; // rbx
  
  while( true ) {
    if ((!*(char *)(a0 + 0x418)) || (a1 != 0x113)) {
      sub_f9c0(a0,a1,3); // return-dupe, tail-call
      return;
    }
    if (1 <= *(long *)(a0 + 0x68)) break;
    *(unsigned long *)(a0 + 0x68) = 0;
    if (*(long *)(a0 + 0x50) == -1)
      return;
    a1 = *(long *)(a0 + 0x50) + 0x114;
  }
  v2 = 1;
  sub_fc00(a0,**(unsigned int **)(a0 + 0x60));
  if (2 <= *(long *)(a0 + 0x68)) {
    do {
      v1 = v2 * 4;
      v2 += 1;
      sub_fc00(a0,*(unsigned int *)(*(long *)(a0 + 0x60) + v1));
      sub_fb20(a0,0x106);
    } while (v2 < *(long *)(a0 + 0x68));
  }
  *(unsigned long *)(a0 + 0x68) = 0;
  if (*(long *)(a0 + 0x50) == -1)
    return;
  sub_fb20(a0,*(long *)(a0 + 0x50) + 0x114);
  a1 = 0x106;
  sub_f9c0(a0,a1,3);
  return;
}

// Function: sub_fc00 @ 0xfc00
void sub_fc00(unsigned long a0,unsigned long a1) // return-dupe x3
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
    sub_f9c0(a0,0,3);
    return;
  }
  v2 = (int)v3;
  if (v2 != 1) {
    sub_f9c0(a0,v5[0],1);
    if (v2 <= 1)
      return;
    v3 = 1;
    do {
      v4 = (int)v3;
      v1 = &v5[v3];
      v3 += 1;
      sub_f9c0(a0,*v1,(v2 + -1 == v4) * '\x02');
      sub_fb20(a0,0x105);
    } while ((unsigned long)(unsigned int)(v2 - 2) + 2 != v3);
    return;
  }
  sub_f9c0(a0,v5[0],3);
}

// Function: sub_fd10 @ 0xfd10
void sub_fd10(long a0) // return-dupe x2
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
  v3 = (unsigned long *)sub_1f1f0(v7,&v6,*(long *)(a0 + 0x130) - v5,0xffffffffffffffff,8);
  v2 = v6;
  v4 = *(unsigned long *)(a0 + 0x178);
  v3[1] = 0;
  v7 = v6 + -2;
  *v3 = 0;
  *(unsigned long **)(a0 + 0x170) = &v3[2];
  *(long *)(a0 + 0x160) = v7;
  v4 = sub_1efd0(v4,v7,8);
  *(unsigned long *)(a0 + 0x178) = v4;
  v4 = sub_1efd0(*(unsigned long *)(a0 + 0x180),v7,1);
  *(unsigned long *)(a0 + 0x180) = v4;
  v4 = sub_1efd0(*(unsigned long *)(a0 + 0x188),v7,8);
  *(unsigned long *)(a0 + 0x188) = v4;
  if (*(char *)(a0 + 0x418)) {
    v8 = *(long *)(a0 + 0x1b0);
    if (v8)
      v8 -= 0x10;
    v3 = (unsigned long *)sub_1efd0(v8,v6,8);
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

// Function: sub_fec0 @ 0xfec0
void sub_fec0(long *a0,long *a1,unsigned int a2,long *a3)
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
    *a3 = sub_1f1f0(0,&a3[2],v5,0xffffffffffffffff,0x10);
  }
  a3[1] = 0;
  v5 = a1[1];
  v7 = 0;
  v4 = 0;
label_ff00:
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
      if (v5 <= v7) goto label_ff69;
      v3 = (long *)(v7 * 0x10 + *a1);
      if (v2 <= *v3) goto label_ffb4;
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
label_ffb4:
  if (*v3 == v2) {
    v7 += 1;
    v8 = *(unsigned int *)&v3[1] & a2;
  }
label_ff69:
  v5 = a3[1];
  v1 = *(unsigned int *)&v6[1];
  v4 += 1;
  v6 = (long *)(v5 * 0x10 + *a3);
  *v6 = v2;
  a3[1] = v5 + 1;
  v5 = a1[1];
  *(unsigned int *)&v6[1] = v1 | v8;
  goto label_ff00;
}

// Function: sub_10010 @ 0x10010
void sub_10010(unsigned long *a0,unsigned long *a1)
{
  long v1;
  unsigned long v2; // rax
  
  v1 = a0[1];
  if (v1 <= (long)a1[2]) // branch-flip
    a1[1] = v1;
  else {
    free((void *)*a1);
    v2 = sub_1f1f0(0,&a1[2],a0[1] - a1[2],0xffffffffffffffff,0x10);
    v1 = a0[1];
    *a1 = v2;
    a1[1] = v1;
  }
  if (!v1)
    return;
  memcpy((void *)*a1,(void *)*a0,v1 << 4); // tail-call
}

// Function: sub_10090 @ 0x10090
unsigned long * sub_10090(unsigned long a0,unsigned long a1)
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  
  v1 = (unsigned long *)sub_1ef10(0x30);
  *v1 = sub_1f310(8);
  v1[1] = sub_1f340(a1);
  v1[2] = sub_1f340(a1);
  v2 = sub_1f340(a1);
  v1[5] = a0;
  v1[3] = v2;
  *(unsigned short *)&v1[4] = 0;
  return v1;
}

// Function: sub_10100 @ 0x10100
long * sub_10100(long *a0,char *a1)
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
        if (!v5) goto label_101c9;
      }
      v3 = v2 + -1;
      free(v5);
      v6 = v3 * 8;
      if (v3 == v7) {
        v3 = v2 + 1;
        v2 = v6 + 8;
        goto label_101d9;
      }
      *v1 = a0[v3];
      v1 = &a0[v7];
      a0[v3] = 0;
      v5 = (char *)*v1;
      v2 = v3;
    } while (v5);
label_101c9:
    v6 = v3 * 8;
    v3 += 2;
    v2 = v6 + 8;
  }
  else {
    v2 = 8;
    v6 = 0;
    v3 = 2;
  }
label_101d9:
  v1 = (long *)sub_1efd0(a0,v3,8);
  *(char **)((long)v1 + v6) = a1;
  *(unsigned long *)((long)v1 + v2) = 0;
  return v1;
}

// Function: sub_10230 @ 0x10230
char * sub_10230(char *a0,char *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  
  v1 = strlen(a1);
  if (v1) {
    v2 = strlen(a0);
    a0 = (char *)sub_1efa0(a0,v1 + 1 + v2);
    memcpy(&a0[v2],a1,v1 + 1);
  }
  return a0;
}

// Function: sub_10290 @ 0x10290
long sub_10290(long a0,long *a1,unsigned int a2)
{
  unsigned int v1;
  unsigned long v10; // rax
  unsigned long v11;
  long *v12;
  long v13; // r12
  unsigned short v14;
  unsigned int v15;
  long v16; // stack - 0x50
  long v2;
  unsigned int v3; // eax
  long *v4;
  long *v5; // rax
  unsigned long *v6;
  unsigned long v7; // rcx
  unsigned long *v8;
  long v9;
  
  v11 = a1[1];
  v6 = *(unsigned long **)(a0 + 0x128);
  v2 = *(long *)(a0 + 0x130);
  if (1 <= (long)v11) { // branch-flip
    v12 = (long *)*a1;
    v10 = 0;
    v4 = v12;
    do {
      v5 = &v4[2];
      v10 ^= (unsigned long)*(unsigned int *)&v4[1] + *v4;
      v4 = v5;
    } while (&v12[v11 * 2] != v5);
    if (1 <= v2) goto label_102f4;
    v16 = 0;
    v13 = 0;
  }
  else {
    v10 = 0;
    if (v2 <= 0) {
      v16 = 0;
      v14 = 0;
      v13 = 0;
      goto label_1040b;
    }
label_102f4:
    v13 = 0;
    v8 = v6;
    do {
      if (((*v8 == v10) && (v8[2] == v11)) && ((unsigned char)v8[4] == a2)) {
        if (1 <= (long)v11) { // branch-flip
          v7 = 0;
          while ((v9 = v7 * 0x10, *(int *)(*a1 + 8 + v9) == *(int *)(v8[1] + 8 + v9) && (*(long *)(*a1 + v9) == *(long *)(v8[1] + v9)))) {
            v7 += 1;
            if (v7 == v11)
              return v13;
          }
        }
        else {
          v7 = 0;
        }
        if (v11 == v7)
          return v13;
      }
      v13 += 1;
      v8 = &v8[9];
    } while (v13 < v2);
    v16 = v13 * 0x48;
    if ((long)v11 <= 0) {
      v14 = 0;
      goto label_1040b;
    }
    v12 = (long *)*a1;
  }
  v9 = 0;
  v15 = 0;
  do {
    while( true ) {
      v1 = *(unsigned int *)(*(long *)(a0 + 0x150) + v12[v9 * 2] * 4);
      if (v1) break;
      if (*(long *)(*(long *)(a0 + 0x90) + v12[v9 * 2] * 8) == 0x112)
        v15 = 0x1ff;
      v14 = (unsigned short)v15;
      v9 += 1;
      if ((long)v11 <= v9) goto label_1040b;
    }
    v3 = v1 | v15;
    if (((int)v1 >> 3 | (int)v1 >> 6 | v1) & a2 & 7)
      v15 = v3;
    v14 = (unsigned short)v15;
    v9 += 1;
  } while (v9 < (long)v11);
label_1040b:
  if (*(long *)(a0 + 0x138) <= v2) {
    v6 = (unsigned long *)sub_1f1f0(v6,a0 + 0x138,1,0xffffffffffffffff,0x48);
    v11 = a1[1];
  }
  *(unsigned long **)(a0 + 0x128) = v6;
  v6 = (unsigned long *)((long)v6 + v16);
  *v6 = v10;
  v10 = sub_1f050(v11,0x10);
  v2 = *(long *)(a0 + 0x128);
  v6[1] = v10;
  v6[3] = v11;
  v6[2] = 0;
  sub_10010(a1,v2 + v16 + 8);
  v16 = *(long *)(a0 + 0x128) + v16;
  *(char *)(v16 + 0x20) = (char)a2;
  *(unsigned short *)(v16 + 0x22) = v14;
  *(unsigned long *)(v16 + 0x30) = 0;
  *(unsigned long *)(v16 + 0x28) = 0;
  *(unsigned long *)(v16 + 0x40) = 0xffffffffffffffff;
  *(long *)(a0 + 0x130) = *(long *)(a0 + 0x130) + 1;
  return v13;
}

// Function: sub_10530 @ 0x10530
void sub_10530(long a0,long a1,long a2) // return-dupe x2
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
      sub_f9c0(a0,*(unsigned long *)(*(long *)(a0 + 0x90) + v3),(int)*v2);
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
    sub_f9c0(a0,*v1,3);
  } while (a2 != v3);
}

// Function: sub_105e0 @ 0x105e0
void sub_105e0(long a0)
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

// Function: sub_10670 @ 0x10670
long sub_10670(long a0,long a1)
{
  long v1;
  long v2;
  
  v2 = 0;
  while( true ) {
    for (; v1 = *(long *)(a0 + -8 + a1 * 8), 0x103 < v1; a1 = (a1 + -1) - v1) {
      if (2 <= (unsigned long)(v1 - 0x105U)) {
        return v2 + 1;
      }
      v1 = sub_10670(a0,a1 + -1);
      v2 = v2 + 1 + v1;
    }
    if (v1 <= 0x100) break;
    a1 -= 1;
    v2 += 1;
  }
  return v2 + 1;
}

// Function: sub_106e0 @ 0x106e0
void sub_106e0(long a0,long a1,long *a2)
{
  long v1;
  long v2; // rax
  long v3;
  long *v4;
  long *v5;
  long v6;
  long *v7;
  long *v8; // rbp
  long v9;
  
  v1 = a2[1];
  v5 = (long *)*a2;
  v3 = v1; // branch-flip
  if (1 <= v1) {
    v6 = 0;
    v9 = v1;
    do {
      while( true ) {
        v2 = v9 + v6 >> 1;
        v4 = &v5[v2 * 2];
        if (*v4 < a0) break;
        if (*v4 == a0) {
          *(unsigned int *)&v4[1] = *(unsigned int *)&v4[1] | (unsigned int)a1;
          return;
        }
        v9 = v2;
        if (v2 <= v6) goto label_1073d;
      }
      v6 = v2 + 1;
    } while (v6 < v9);
label_1073d:
    v9 = v6 << 4;
    v4 = v5;
    if (a2[2] <= v1) goto label_107a4;
  }
  else {
    if (v1 < a2[2]) goto label_10782;
    v9 = 0;
    v6 = 0;
label_107a4:
    v4 = (long *)sub_1f1f0(v5,&a2[2],1,0xffffffffffffffff,0x10);
    v3 = a2[1];
  }
  *a2 = (long)v4;
  v5 = (long *)((long)v4 + v9);
  if (v6 < v1) {
    v7 = &v4[v1 * 2];
    do {
      v8 = &v7[-2];
      *v7 = v7[-2];
      v7[1] = v7[-1];
      v7 = v8;
    } while (&v4[v6 * 2] != v8);
  }
label_10782:
  *v5 = a0;
  v5[1] = a1;
  a2[1] = v3 + 1;
  return;
}

// Function: sub_10800 @ 0x10800
void sub_10800(unsigned long a0,long *a1,unsigned long a2) // return-dupe
{
  unsigned long *v1; // rax
  long v2;
  long v3; // rbx
  
  if (4 <= a1[1]) {
    sub_fec0(a1,a0,0xffffffff,a2);
    sub_10010(a2,a0); // tail-call
    return;
  }
  v2 = 0;
  if (a1[1] <= 0)
    return;
  do {
    v3 = v2 + 1;
    v1 = (unsigned long *)(v2 * 0x10 + *a1);
    sub_106e0(*v1,v1[1],a0);
    v2 = v3;
  } while (v3 < a1[1]);
}

// Function: sub_10890 @ 0x10890
long sub_10890(long *a0,unsigned long *a1)
{
  unsigned long *v1;
  long v10;
  long v11; // r12
  unsigned long *v2;
  unsigned long *v3;
  unsigned long v4;
  unsigned long v5;
  long v6; // rax
  long v7;
  unsigned long v8; // rcx
  long v9;
  
  v10 = a0[1];
  v7 = *a0;
  if (1 <= v10) { // branch-flip
    v11 = 0;
    v9 = v7;
    do {
      v6 = 0;
      v8 = 0;
      do {
        v1 = (unsigned long *)((long)a1 + v6);
        v2 = (unsigned long *)(v9 + v6);
        v6 += 8;
        v8 |= *v1 ^ *v2;
      } while (v6 != 0x20);
      if (!v8)
        return v11;
      v11 += 1;
      v9 += 0x20;
    } while (v10 != v11);
    v9 = v10 << 5;
  }
  else {
    v9 = 0;
    v11 = 0;
  }
  if (a0[2] <= v10) {
    v7 = sub_1f1f0(v7,&a0[2],1,0x7ffffffffffffeeb,0x20);
    v10 = a0[1];
  }
  v4 = *a1;
  v5 = a1[1];
  *a0 = v7;
  a0[1] = v10 + 1;
  *(unsigned long *)(v7 + v9) = v4;
  ((unsigned long *)(v7 + v9))[1] = v5;
  v4 = a1[3];
  v3 = (unsigned long *)(v7 + 0x10 + v9);
  *v3 = a1[2];
  v3[1] = v4;
  return v11;
}

// Function: sub_10960 @ 0x10960
void sub_10960(unsigned long *a0)
{
  long *v1;
  void *v2;
  
  v1 = (long *)*a0;
  v2 = (void *)*v1;
  if (v2) {
    do {
      free(v2);
      v2 = (void *)v1[1];
      v1 = &v1[1];
    } while (v2);
    v1 = (long *)*a0;
  }
  free(v1);
  free((void *)a0[1]);
  free((void *)a0[2]);
  free((void *)a0[3]);
  free(a0); // tail-call
}

// Function: sub_109d0 @ 0x109d0
unsigned long sub_109d0(long a0,long *a1,unsigned char a2)
{
  long *v1;
  unsigned long v10;
  unsigned long *v11;
  unsigned int v12; // ecx
  unsigned long *v13;
  long v14;
  unsigned int v15;
  unsigned int v16; // edx
  unsigned int v17; // edx
  int v18; // edx
  unsigned long *v19;
  unsigned char v2;
  unsigned long *v20;
  unsigned long v21;
  void *v22; // stack - 0xe8
  char v23 [16]; // stack - 0x68
  unsigned long v24 [4]; // stack - 0x88
  long v25 [2]; // stack - 0xb8
  long v26;
  long *v27; // stack - 0xc8
  void *v28; // stack - 0xa8
  long v29;
  char v3;
  long v30;
  long v31; // r14
  unsigned char v32;
  unsigned int v33; // r15d
  long v34;
  unsigned long v35;
  long v36; // stack - 0xe0
  long v37; // stack - 0xd8
  long v38; // stack - 0xc0
  unsigned long v39; // stack - 0xa0
  unsigned short v4;
  long v40; // stack - 0x98
  char v41 [16]; // stack - 0x58
  unsigned long v5;
  long *v6;
  char (*v7)[16];
  unsigned long *v8;
  long v9;
  
  v29 = a0 * 0x48;
  v30 = a1[0x2e];
  v9 = v30;
  if (*(short *)(a1[0x25] + 0x22 + a0 * 0x48))
    v9 = a1[0x2f];
  v20 = (unsigned long *)(v9 + a0 * 8);
  v11 = (unsigned long *)*v20;
  if (!v11) {
    if (0x400 <= (int)a1[0x2d]) { // branch-flip
      v9 = (long)*(int *)((long)a1 + 0x16c);
      if (v9 < a1[0x2c]) {
        do {
          free(*(void **)(v30 + v9 * 8));
          free(*(void **)(a1[0x2f] + v9 * 8));
          *(unsigned long *)(a1[0x2f] + v9 * 8) = 0;
          v30 = a1[0x2e];
          *(unsigned long *)(v30 + v9 * 8) = 0;
          v9 += 1;
        } while (v9 < a1[0x2c]);
      }
      v18 = 1;
    }
    else {
      v18 = (int)a1[0x2d] + 1;
    }
    *(int *)&a1[0x2d] = v18;
    v11 = (unsigned long *)sub_1ef10(0x800);
    *v20 = v11;
    v20 = v11;
    do {
      *v20 = 0xfffffffffffffffe;
      v20 = &v20[1];
    } while (&v11[0x100] != v20);
  }
  *(char *)(a1[0x30] + a0) = 0;
  v32 = *(unsigned char *)(a1[0x25] + v29 + 0x20);
  v4 = *(unsigned short *)(a1[0x25] + v29 + 0x22);
  v15 = (unsigned int)v4;
  if ((int)v15 >> 6 & (unsigned int)v32 & 7) {
    *(unsigned char *)(a1[0x30] + a0) = *(unsigned char *)(a1[0x30] + a0) | 4;
    v32 = *(unsigned char *)(a1[0x25] + v29 + 0x20);
    v4 = *(unsigned short *)(a1[0x25] + v29 + 0x22);
    v15 = (unsigned int)v4;
  }
  if ((int)(unsigned int)v4 >> 3 & (unsigned int)v32 & 7) {
    *(unsigned char *)(a0 + a1[0x30]) = *(unsigned char *)(a0 + a1[0x30]) | 2;
    v32 = *(unsigned char *)(a1[0x25] + v29 + 0x20);
    v15 = (unsigned int)*(unsigned short *)(a1[0x25] + v29 + 0x22);
  }
  if (v15 & 7 & (unsigned int)v32)
    *(unsigned char *)(a0 + a1[0x30]) = *(unsigned char *)(a0 + a1[0x30]) | 1;
  v30 = a1[0x16];
  v22 = (void *)sub_1f050(v30,0x10);
  v36 = 0;
  v37 = v30;
  if (1 <= *(long *)(a1[0x25] + 0x10 + v29)) {
    v30 = a1[0x25] + v29;
    v9 = a1[0x28];
    v34 = 0;
    do {
      v6 = (long *)(v9 + *(long *)(*(long *)(v30 + 8) + v34 * 0x10) * 0x18);
      v14 = 0;
      if (1 <= v6[1]) {
        do {
          v31 = v14 + 1;
          v20 = (unsigned long *)(v14 * 0x10 + *v6);
          sub_106e0(*v20,v20[1],&v22);
          v30 = a1[0x25] + v29;
          v9 = a1[0x28];
          v6 = (long *)(v9 + *(long *)(*(long *)(v30 + 8) + v34 * 0x10) * 0x18);
          v14 = v31;
        } while (v31 < v6[1]);
      }
      v34 += 1;
    } while (v34 < *(long *)(v30 + 0x10));
  }
  v30 = a1[0x16];
  v27 = (long *)sub_1f050(v30,0x10);
  v38 = 0;
  v24[0] = 0xffffffffffffffff;
  v24[1] = 0xffffffffffffffff;
  v24[2] = 0xffffffffffffffff;
  v24[3] = 0xffffffffffffffff;
  v25[0] = v30; // branch-flip
  if (1 <= v36) {
    v30 = 0;
    v21 = 0xffffffffffffffff;
    v10 = (unsigned long)(a2 >> 6);
    do {
      v6 = (long *)(v30 * 0x10 + (long)v22);
      v9 = *v6;
      v34 = v6[1];
      v15 = *(unsigned int *)&v6[1];
      v35 = *(unsigned long *)(a1[0x12] + v9 * 8);
      if (0x100 <= v35) { // branch-flip
        if (0x114 <= (long)v35) { // branch-flip
          v34 = v35 * 0x20 + *a1;
          v7 = (char (*)[16])(v34 + -0x2280);
          v23._0_16_ = *v7;
          v41._0_16_ = *(char (*)[16])(v34 + -0x2270);
          v35 = *(unsigned long *)&v23[v10 * 8] >> (a2 & 0x3f);
          v32 = (unsigned char)v35 & 1;
          if (!(v35 & 1)) goto label_10d4e;
          v35 = *(unsigned long *)*v7;
          if (v15 == 0x1ff) goto label_10cde;
label_10f90:
          v2 = *(unsigned char *)(a1[0x25] + 0x20 + v29);
          if (!((int)v15 >> 6 & (unsigned int)v2 & 7)) {
            v8 = (unsigned long *)v23;
            v13 = (unsigned long *)&a1[0x7e];
            do {
              v35 = *v13;
              v13 = &v13[1];
              *v8 = *v8 & ~v35;
              v8 = &v8[1];
            } while (&v26 != v8);
          }
          if (!((int)v15 >> 3 & (unsigned int)v2 & 7)) {
            v8 = (unsigned long *)v23;
            v13 = (unsigned long *)&a1[0x7a];
            do {
              v35 = *v13;
              v13 = &v13[1];
              *v8 = *v8 & ~v35;
              v8 = &v8[1];
            } while (&v26 != v8);
          }
          if (!(v2 & v15 & 7)) {
            v8 = (unsigned long *)v23;
            v13 = (unsigned long *)&a1[0x7e];
            do {
              v19 = &v13[-4];
              v35 = *v13;
              v13 = &v13[1];
              *v8 = *v8 & (*v19 | v35);
              v8 = &v8[1];
            } while (&v26 != v8);
          }
          if (((v23._8_8_ || v23._0_8_) || v41._0_8_) || v41._8_8_) {
            v35 = v23._0_8_;
            if (!(*(unsigned long *)&v23[v10 * 8] >> (a2 & 0x3f) & 1)) goto label_10d66;
label_11008:
            v35 = v23._0_8_;
            if (!v32) goto label_10d66;
            goto label_10cde;
          }
        }
        else if (v35 == 0x10a) {
          v7 = (char (*)[16])(a1[3] * 0x20 + *a1);
          v23._0_16_ = *v7;
          v41._0_16_ = v7[1];
          v32 = (unsigned char)(*(unsigned long *)&v23[v10 * 8] >> (a2 & 0x3f)) & 1;
          v14 = a1[0x25] + v29;
          if (*(unsigned char *)(v14 + 0x20) & v15 & 7) {
            if (!*(long *)(v14 + 0x30)) {
              *(unsigned long *)(v14 + 0x28) = sub_1f050(1,0x10);
              *(unsigned long *)(v14 + 0x38) = 1;
              *(unsigned long *)(v14 + 0x30) = 0;
              v14 = a1[0x25] + v29;
            }
            sub_106e0(v9,v34,v14 + 0x28);
          }
          if (v15 != 0x1ff) goto label_10f90;
          goto label_11008;
        }
      }
      else {
        v23._0_16_ = 0;
        v41._0_16_ = 0;
        *(long *)&v23[(v35 >> 6 & 0x3ffffff) * 8] = 1L << ((unsigned char)v35 & 0x3f);
        if (v35 != a2) { // branch-flip
label_10d4e:
          v32 = 0;
          v35 = v23._0_8_;
          if (v15 != 0x1ff) goto label_10f90;
label_10d66:
          v13 = (unsigned long *)v23;
          v8 = v24;
          while( true ) {
            v19 = &v8[1];
            v13 = &v13[1];
            *v8 = ~v35 & v21;
            if ((unsigned long *)v23 == v19) break;
            v21 = *v19;
            v35 = *v13;
            v8 = v19;
          }
        }
        else {
          v23._0_8_ = 0;
          v32 = 1;
          v35 = v23._0_8_;
          if (v15 != 0x1ff) goto label_10f90;
label_10cde:
          v13 = (unsigned long *)v23;
          v8 = v24;
          while( true ) {
            v19 = &v8[1];
            v13 = &v13[1];
            *v8 = v21 & v35;
            if ((unsigned long *)v23 == v19) break;
            v21 = *v19;
            v35 = *v13;
            v8 = v19;
          }
          if (v25[0] <= v38)
            v27 = (long *)sub_1f1f0(v27,v25,1,0xffffffffffffffff,0x10);
          v34 = v38 + 1;
          v27[v38 * 2] = v9;
          *(unsigned int *)&(&v27[v38 * 2])[1] = v15;
          v38 = v34;
        }
        v21 = v24[0];
      }
      v30 += 1;
    } while (v30 < v36);
  }
  else {
    v21 = 0xffffffffffffffff;
  }
  v29 = a1[0x16];
  v28 = (void *)sub_1f050(v29,0x10);
  v39 = 0;
  v40 = v29;
  if (v38 <= 0) {
    if ((char)a1[0x29]) { // branch-flip
      v29 = a1[0x32];
      v9 = 0;
      v30 = (long)(*(int *)((long)a1 + 0x16c) + -1);
    }
    else {
      v30 = -1;
      v9 = -1;
      v29 = -1;
    }
    goto label_11249;
  }
  v6 = v27;
  v29 = v38;
  if ((char)a1[0x29]) {
    if ((char)a1[0x83]) {
      v30 = 0;
      do {
        v9 = v30 + 1;
        if (!(*(unsigned char *)(a1[0x23] + v27[v30 * 2]) & 1)) goto label_10e85;
        v30 = v9;
      } while (v38 != v9);
    }
    sub_10800(&v27,a1[0x25] + 8,&v28);
    v6 = v27;
    v29 = v38;
  }
label_10e85:
  v15 = 0;
  v30 = 0;
  v10 = v21;
  while( true ) {
    v35 = *(unsigned long *)((long)a1 + v30 + 0x3f0);
    if (v35 & v10)
      v15 |= 4;
    v5 = *(unsigned long *)((long)a1 + v30 + 0x3d0);
    if (v5 & v10)
      v15 |= 2;
    if (~(v35 | v5) & v10)
      v15 |= 1;
    if (v30 + 8 == 0x20) break;
    v10 = *(unsigned long *)((long)v24 + v30 + 8);
    v30 += 8;
  }
  if (1 <= v29) { // branch-flip
    v16 = 0;
    v1 = &v6[v29 * 2];
    do {
      v29 = *v6;
      v6 = &v6[2];
      v16 |= *(unsigned int *)(a1[0x2b] + v29 * 4);
    } while (v1 != v6);
    v29 = -1;
    v17 = v16 ^ 7;
    v33 = v16 & v15 & 2;
    v12 = v16 & v15 & 4;
    if (~v16 & v15) {
label_1142c:
      v29 = sub_10290(a1,&v27,v17);
    }
    v9 = v29;
    if (v12)
      v9 = sub_10290(a1,&v27,4);
    v30 = v29;
    if (v33)
      v30 = sub_10290(a1,&v27,2);
  }
  else {
    if (v15) {
      v33 = 0;
      v12 = 0;
      v17 = 7;
      goto label_1142c;
    }
    v29 = -1;
    v9 = -1;
    v30 = -1;
  }
  sub_fd10(a1);
label_11249:
  v10 = 0;
  while( true ) {
    if (v21 >> (v10 & 0x3f) & 1) {
      v3 = *(char *)((long)a1 + v10 + 0x1cf);
      if (v3 != '\x02') { // branch-flip
        v34 = v9;
        if (v3 != '\x04')
          v34 = v29;
        v11[v10] = v34;
      }
      else {
        v11[v10] = v30;
      }
    }
    if (v10 == 0xff) break;
    v10 += 1;
    v21 = v24[v10 >> 6 & 0x3ffffff];
  }
  free(v27);
  free(v22);
  free(v28);
  v32 = *(unsigned char *)((long)a1 + 0x1ce);
  if (v24[v32 >> 6] >> (v32 & 0x3f) & 1) {
    *(unsigned long *)(a1[0x31] + a0 * 8) = v11[v32];
    v11[v32] = 0xffffffffffffffff;
  }
  return v11[a2];
}

// Function: sub_11480 @ 0x11480
void sub_11480(long a0,char a1)
{
  unsigned char v1;
  unsigned long *v10; // rax
  void *v11;
  void *v12; // rax
  long *v13;
  unsigned long *v14;
  void *v15; // rax
  unsigned char v16;
  long v17;
  unsigned int v18;
  char *v19;
  unsigned int v2;
  char *v20;
  unsigned char *v21;
  long *v22; // stack - 0x98
  void *v23; // stack - 0x78
  void *v24; // stack - 0x58
  long v25;
  long v26;
  long v27;
  long v28;
  unsigned int *v29;
  long v3;
  long *v30;
  unsigned long *v31;
  unsigned int *v32;
  long *v33;
  long v34; // r15
  long v35; // fs_offset
  unsigned char *v36; // stack - 0xf8
  void *v37; // stack - 0xf0
  long *v38; // stack - 0xe8
  long *v39; // stack - 0xd0
  long v4;
  long *v40; // stack - 0xc0
  char *v41; // stack - 0xb8
  long **v42; // stack - 0xb0
  long v43; // stack - 0x90
  unsigned long v44; // stack - 0x88
  unsigned long v45; // stack - 0x70
  unsigned long v46; // stack - 0x68
  unsigned long v47; // stack - 0x50
  unsigned long v48; // stack - 0x48
  long v49; // stack - 0x40
  int v5; // eax
  unsigned long v6;
  long *v7;
  long v8;
  unsigned int *v9; // rax
  
  v49 = *(long *)(v35 + 0x28);
  v40 = (long *)sub_1f050(*(unsigned long *)(a0 + 0xb0),0x20);
  v36 = (long *)&v40[*(long *)(a0 + 0xb0) * 2];
  v41 = (char *)sub_1f050(*(unsigned long *)(a0 + 0xa8),0x18);
  sub_fb20(a0,0x105);
  *(char *)(a0 + 0x148) = a1;
  v6 = *(unsigned long *)(a0 + 0xb0);
  v3 = *(long *)(a0 + 0x98);
  v23 = (void *)sub_1f050(v6,0x10);
  v45 = 0;
  v46 = v6;
  v6 = sub_1f320(v3,0x18);
  *(unsigned long *)(a0 + 0x140) = v6;
  if (*(char *)(a0 + 0xc1)) { // branch-flip
    v39 = (long *)sub_1f320(v3,0x18);
    if (0 < v3) goto label_1156b;
    v6 = *(unsigned long *)(a0 + 0xb0);
    v24 = (void *)sub_1f050(v6,0x10);
    v47 = 0;
    v48 = v6; // branch-flip
    if (1 <= *(long *)(a0 + 0x98)) {
label_116dc:
      v47 = 0;
      v25 = 0;
      v13 = v39;
      do {
        v28 = *(long *)(a0 + 0x140);
        v7 = (long *)((long)v13 + (v28 - (long)v39));
        if (1 <= v7[1]) {
          switch(*(unsigned long *)(*(long *)(a0 + 0x90) + v25 * 8)) {
            case 0x100:
              v38._0_4_ = 0x1ff;
              break;
            default:
              goto label_118fd;
            case 0x10c:
              v38._0_4_ = 0x124;
              break;
            case 0x10d:
              v38._0_4_ = 0x1c0;
              break;
            case 0x10e:
              v38._0_4_ = 0x28;
              break;
            case 0x10f:
              v38._0_4_ = 0x82;
              break;
            case 0x110:
              v38._0_4_ = 0xaa;
              break;
            case 0x111:
              v38._0_4_ = 0x155;
            
          }
          sub_f890(v25,v7);
          v33 = v7;
          if (1 <= v13[1]) {
            v27 = 0;
            do {
              while( true ) {
                v17 = v28 + *(long *)(v27 * 0x10 + *v13) * 0x18;
                v18 = sub_f890(v25,v17) & (unsigned int)v38;
                if (!v18) break;
                v27 += 1;
                sub_10010(v17,&v24);
                sub_fec0(&v24,v7,v18,v17);
                v28 = *(long *)(a0 + 0x140);
                v7 = (long *)((long)v13 + (v28 - (long)v39));
                if (v13[1] <= v27) goto label_122ae;
              }
              v27 += 1;
            } while (v27 < v13[1]);
label_122ae:
            v33 = v7;
          }
          if (1 <= v33[1]) {
            v28 = 0;
            do {
              v7 = &v39[*(long *)(v28 * 0x10 + *v7) * 3];
              v18 = sub_f890(v25,v7);
              if (v18 & 0x1ff) {
                sub_10010(v7,&v24);
                sub_fec0(&v24,v13,v18 & 0x1ff,v7);
              }
              v28 += 1;
              v7 = (long *)((long)v13 + (*(long *)(a0 + 0x140) - (long)v39));
            } while (v28 < v7[1]);
          }
        }
label_118fd:
        v25 += 1;
        v13 = &v13[3];
      } while (v25 < *(long *)(a0 + 0x98));
      free(v24);
      if (v3 > 0) goto label_1192e;
    }
    else {
      free(v24);
    }
  }
  else {
    v39 = NULL;
    if (v3 <= 0) goto label_11960;
label_1156b:
    v25 = 0;
    v38 = v40;
    v19 = v41;
    do {
      v28 = *(long *)(*(long *)(a0 + 0x90) + v25 * 8);
      v20 = v19;
      switch(v28) {
        case 0x100:
          *v19 = 1;
          v20 = &v19[0x18];
          *(unsigned long *)&v19[0x10] = 0;
          *(unsigned long *)&v19[8] = 0;
          break;
        case 0x101:
label_11758:
          v19[-0x18] = 1;
          break;
        case 0x102:
        case 0x103:
          v7 = (long *)((long)v36 + *(long *)&v19[-8] * -2 * 8);
          v43 = *(long *)&v19[-0x10];
          v13 = &v38[v43 * -2];
          if ((*(char *)(a0 + 0xc1)) && (v13 < v38)) {
            v22 = v7;
            v43 = *(long *)&v19[-8];
            do {
              v28 = *v13;
              v13 = &v13[2];
              sub_10800(&v39[v28 * 3],&v22,&v23);
            } while (v13 < v38);
            v43 = *(long *)&v19[-0x10];
            v7 = (long *)((long)v36 + *(long *)&v19[-8] * -2 * 8);
            v13 = &v38[v43 * -2];
          }
          v22 = v13;
          if (v7 < v36) {
            do {
              v28 = *v7;
              v7 = &v7[2];
              sub_10800(*(long *)(a0 + 0x140) + v28 * 0x18,&v22,&v23);
            } while (v7 < v36);
          }
          if (*(long *)(*(long *)(a0 + 0x90) + v25 * 8) != 0x103) goto label_11758;
          break;
        default:
          *(unsigned long *)&v19[0x10] = 1;
          *v19 = v28 == 0x112;
          *(long *)v36 = v25;
          v7 = &v38[2];
          v13 = (long *)((long)v36 + 0x10);
          *v38 = v25;
          *(unsigned long *)&v19[8] = 1;
          *(unsigned int *)((long)v36 + 8) = 0x1ff;
          *(unsigned int *)&v38[1] = 0x1ff;
          v20 = &v19[0x18];
          v36 = (unsigned char *)v13;
          v38 = v7;
          break;
        case 0x105:
          v28 = *(long *)&v19[-0x10];
          v27 = *(long *)&v19[-8];
          v17 = *(long *)&v19[-0x20];
          v8 = v27 << 4;
          v26 = v17 << 4;
          v13 = &v38[v28 * -2];
          if ((v39) && (v22 = (long *)((long)v36 + (v27 + v17) * -2 * 8), v13 < v38)) {
            v43 = v17;
            do {
              v28 = *v13;
              v13 = &v13[2];
              sub_10800(&v39[v28 * 3],&v22,&v23);
            } while (v13 < v38);
            v28 = *(long *)&v19[-0x10];
            v27 = *(long *)&v19[-8];
            v17 = *(long *)&v19[-0x20];
            v13 = &v38[v28 * -2];
            v26 = v17 << 4;
            v8 = v27 << 4;
          }
          v7 = (long *)((long)v36 - v8);
          v33 = (long *)((long)v7 - v26);
          v22 = v13;
          v43 = v28;
          if (v33 < v7) {
            do {
              v28 = *v33;
              v33 = &v33[2];
              sub_10800(*(long *)(a0 + 0x140) + v28 * 0x18,&v22,&v23);
            } while (v33 < v7);
            v28 = *(long *)&v19[-0x10];
            v27 = *(long *)&v19[-8];
            v17 = *(long *)&v19[-0x20];
          }
          v16 = v19[-0x30];
          if (v16) { // branch-flip
            v1 = v19[-0x18];
            *(long *)&v19[-0x28] = *(long *)&v19[-0x28] + v28;
            if (!v1) goto label_1212f;
label_11888:
            v27 += v17;
          }
          else {
            v1 = v19[-0x18];
            v38 = &v38[v28 * -2];
            if (v1) goto label_11888;
label_1212f:
            v10 = (unsigned long *)((long)v36 - (v27 * 0x10 + v17 * 0x10));
            v31 = v10;
            if (1 <= v27) {
              do {
                v6 = (&v31[v17 * 2])[1];
                v14 = &v31[2];
                *v31 = v31[v17 * 2];
                v31[1] = v6;
                v31 = v14;
              } while (&v10[v27 * 2] != v14);
            }
            v36 = (long *)((long)v36 + v17 * -2 * 8);
          }
          *(long *)&v19[-0x20] = v27;
          v19[-0x30] = v16 & v1;
          v20 = &v19[-0x18];
          break;
        case 0x106:
          *(long *)&v19[-0x28] = *(long *)&v19[-0x28] + *(long *)&v19[-0x10];
          *(long *)&v19[-0x20] = *(long *)&v19[-0x20] + *(long *)&v19[-8];
          v19[-0x30] = v19[-0x30] | v19[-0x18];
          v20 = &v19[-0x18];
        
      }
      v25 += 1;
      v19 = v20;
    } while (v25 < v3);
    if (!v39) goto label_11960;
    v6 = *(unsigned long *)(a0 + 0xb0);
    v24 = (void *)sub_1f050(v6,0x10);
    v47 = 0;
    v48 = v6;
    if (*(long *)(a0 + 0x98) > 0) goto label_116dc;
    free(v24);
label_1192e:
    v25 = 0;
    v13 = v39;
    do {
      v11 = (void *)*v13;
      v25 += 1;
      v13 = &v13[3];
      free(v11);
    } while (v3 != v25);
  }
  free(v39);
label_11960:
  v42 = &v22;
  v36 = (unsigned char *)sub_1f340(*(unsigned long *)(a0 + 0x98));
  v25 = *(long *)(a0 + 0x98);
  if (1 <= v25) {
    v13 = *(long **)(a0 + 0x140);
    v28 = 0;
    do {
      v27 = 0;
      if (0 < v13[1]) {
        do {
          while( true ) {
            v17 = *(long *)(*v13 + v27 * 0x10);
            if (v17 != v28) break;
            v36[v28] = v36[v28] | 1;
            v27 += 1;
            if (v13[1] <= v27) goto label_119d8;
          }
          v21 = &v36[v17];
          if (v28 <= v17) { // branch-flip
            v16 = *v21 & 8;
            *v21 = v16;
            v21 = &v36[*(long *)(*v13 + v27 * 0x10)];
            if (v16) // branch-flip
              *v21 = *v21 | 4;
            else {
              *v21 = *v21 | 8;
            }
          }
          else {
            *v21 = *v21 | 2;
          }
          v27 += 1;
        } while (v27 < v13[1]);
      }
label_119d8:
      v28 += 1;
      v13 = &v13[3];
    } while (v28 != v25);
  }
  *v36 = *v36 | 0x10;
  v6 = *(unsigned long *)(a0 + 0xb0);
  v24 = (void *)sub_1f050(v6,0x10);
  v47 = 0;
  v48 = v6;
  v6 = sub_1f320(*(unsigned long *)(a0 + 0x98),4);
  v25 = *(long *)(a0 + 0x98);
  *(unsigned long *)(a0 + 0x150) = v6;
  if (1 <= v25) {
    v28 = 0;
    do {
      while (!(v36[v28] & 0x10)) {
        v28 += 1;
        if (v28 >= v25) goto label_11b82;
      }
      v27 = *(long *)(a0 + 0x140);
      v13 = (long *)(v27 + v28 * 0x18);
      if (1 <= v13[1]) { // branch-flip
        v17 = 0;
        v25 = 0;
        do {
          while( true ) {
            v33 = (long *)*v13;
            v7 = &v33[v25 * 2];
            v18 = *(unsigned int *)&v7[1];
            if (v18) break;
label_11afe:
            v25 += 1;
            if (v13[1] <= v25) goto label_11b50;
          }
          v8 = *v7;
          v26 = *(long *)(*(long *)(a0 + 0x90) + v8 * 8);
          if (0 <= v26) {
            v21 = &v36[v8];
            if (((v8 != v28) && (v16 = *v21, !(v16 & 6))) && (v17)) {
              v34 = 0;
              v30 = v33;
              do {
                v4 = *v30;
                if (((v4 != v28) && (v18 == *(unsigned int *)&v30[1])) && ((!(v36[v4] & 6) && ((v26 == *(long *)(*(long *)(a0 + 0x90) + v4 * 8) && (!((v36[v4] ^ v16) & 1))))))) {
                  v26 = v27 + v8 * 0x18;
                  if (v16 & 1)
                    sub_f890(v8,v26);
                  sub_10800(v27 + v4 * 0x18,v26,&v24);
                  if (v34 < v17) goto label_11afe;
                  v33 = (long *)*v13;
                  v7 = &v33[v25 * 2];
                  break;
                }
                v34 += 1;
                v30 = &v30[2];
              } while (v34 != v17);
            }
            v8 = v7[1];
            v26 = v17 + 1;
            v33[v17 * 2] = *v7;
            (&v33[v17 * 2])[1] = v8;
            *v21 = *v21 | 0x10;
            v17 = v26;
            goto label_11afe;
          }
          v9 = (unsigned int *)(v28 * 4 + *(long *)(a0 + 0x150));
          v25 += 1;
          *v9 = *v9 | v18;
        } while (v25 < v13[1]);
label_11b50:
        v25 = *(long *)(a0 + 0x98);
      }
      else {
        v17 = 0;
      }
      v28 += 1;
      v13[1] = v17;
    } while (v25 > v28);
  }
label_11b82:
  v10 = (unsigned long *)sub_1f050(v25,8);
  v25 = *(long *)(a0 + 0x98);
  *v10 = 0;
  v31 = &v10[1];
  if (2 <= v25) {
    do {
      *v31 = 0xffffffffffffffff;
      v31 = &v31[1];
    } while (&v10[v25] != v31);
  }
  v11 = (void *)sub_1f050(*(unsigned long *)(a0 + 0xb0),8);
  v37 = (void *)sub_1f050(*(unsigned long *)(a0 + 0xb0),0x18);
  v12 = (void *)sub_1f050(*(unsigned long *)(a0 + 0xb0),4);
  v15 = NULL;
  if (*(char *)(a0 + 0x418))
    v15 = (void *)sub_1f050(*(unsigned long *)(a0 + 0xb0),1);
  v25 = *(long *)(a0 + 0x140);
  v28 = 1;
  if (1 <= *(long *)(a0 + 0x98)) {
    v27 = 0;
    do {
      v17 = v10[v27];
      v31 = (unsigned long *)(v25 + v27 * 0x18);
      if (0 <= v17) { // branch-flip
        *(unsigned long *)((long)v11 + v17 * 8) = *(unsigned long *)(*(long *)(a0 + 0x90) + v27 * 8);
        v6 = v31[1];
        v14 = (unsigned long *)((long)v37 + v17 * 0x18);
        *v14 = *v31;
        v14[1] = v6;
        v14[2] = v31[2];
        *(unsigned int *)((long)v12 + v17 * 4) = *(unsigned int *)(*(long *)(a0 + 0x150) + v27 * 4);
        if (v15)
          *(char *)((long)v15 + v17) = *(char *)(*(long *)(a0 + 0x118) + v27);
        v25 = v31[1];
        v13 = (long *)*v31;
        if (1 <= v25) {
          v17 = 0;
          v7 = v13;
          do {
            v25 = v10[*v7];
            if (v25 == -1) {
              v10[*v7] = v28;
              v28 += 1;
              v25 = v10[*v7];
            }
            *v7 = v25;
            v25 = v31[1];
            v17 += 1;
            v7 = &v7[2];
          } while (v17 < v25);
        }
        qsort(v13,v25,0x10,sub_f930);
        v25 = *(long *)(a0 + 0x140);
      }
      else {
        free((void *)*v31);
        v25 = *(long *)(a0 + 0x140);
        v31 = (unsigned long *)(v27 * 0x18 + v25);
        *v31 = 0;
        v31[1] = 0;
      }
      v27 += 1;
    } while (v27 < *(long *)(a0 + 0x98));
  }
  v27 = 0;
  while( true ) {
    v17 = v27 * 0x18;
    *(unsigned long *)(*(long *)(a0 + 0x90) + v27 * 8) = *(unsigned long *)((long)v11 + v27 * 8);
    v6 = ((unsigned long *)((long)v37 + v17))[1];
    v2 = *(unsigned int *)((long)v12 + v27 * 4);
    *(unsigned long *)(v25 + v17) = *(unsigned long *)((long)v37 + v17);
    ((unsigned long *)(v25 + v17))[1] = v6;
    *(unsigned long *)(v25 + 0x10 + v17) = *(unsigned long *)((long)v37 + v17 + 0x10);
    *(unsigned int *)(*(long *)(a0 + 0x150) + v27 * 4) = v2;
    if (v15)
      *(char *)(*(long *)(a0 + 0x118) + v27) = *(char *)((long)v15 + v27);
    v27 += 1;
    if (v28 == v27) break;
    v25 = *(long *)(a0 + 0x140);
  }
  *(long *)(a0 + 0xb0) = v28;
  *(long *)(a0 + 0x98) = v28;
  free(v11);
  free(v37);
  free(v12);
  free(v15);
  free(v10);
  free(v24);
  free(v36);
  v22 = (long *)sub_1f050(1,0x10);
  v44 = 1;
  *v22 = 0;
  *(unsigned int *)&v22[1] = 0x1ff;
  v43 = 1;
  v9 = (unsigned int *)sub_1f320(v3,4);
  *(unsigned int **)(a0 + 0x158) = v9;
  if (1 <= v3) {
    v32 = *(unsigned int **)(a0 + 0x150);
    v13 = *(long **)(a0 + 0x140);
    v29 = v9;
    do {
      v18 = *v32;
      if (((int)v18 >> 2 ^ v18) & 0x49) {
        *v29 = *v29 | 4;
        v18 = *v32;
      }
      if ((v18 ^ (int)v18 >> 1) & 0x49)
        *v29 = *v29 | 2;
      if (1 <= v13[1]) {
        v25 = *v13;
        v28 = v13[1] * 0x10 + v25;
        do {
          v18 = *(unsigned int *)(v25 + 8);
          if (((int)v18 >> 2 ^ v18) & 0x49) {
            *v29 = *v29 | 4;
            v18 = *(unsigned int *)(v25 + 8);
          }
          if ((v18 ^ (int)v18 >> 1) & 0x49)
            *v29 = *v29 | 2;
          v25 += 0x10;
        } while (v28 != v25);
      }
      v29 = &v29[1];
      v32 = &v32[1];
      v13 = &v13[3];
    } while (&v9[v3] != v29);
  }
  if (1 <= v43) { // branch-flip
    v18 = 0;
    v13 = v22;
    do {
      v3 = *v13;
      v13 = &v13[2];
      v18 |= v9[v3];
    } while (v13 != &v22[v43 * 2]);
    if (v18 & 4)
      sub_10290(a0,v42,4);
    v5 = sub_10290(a0,v42,v18 ^ 7);
    *(int *)(a0 + 0x16c) = v5;
    *(long *)(a0 + 400) = (long)v5;
    if (v18 & 2)
      v5 = sub_10290(a0,v42,2);
  }
  else {
    v5 = sub_10290(a0,v42,7);
    *(long *)(a0 + 400) = (long)v5;
  }
  *(unsigned int *)(a0 + 0x168) = 0;
  *(int *)(a0 + 0x16c) = v5 + 1;
  free(v40);
  free(v41);
  free(v23);
  free(v22);
  if (v49 != *(long *)(v35 + 0x28))
    __stack_chk_fail(); // no-return
}

// Function: sub_12550 @ 0x12550
char * sub_12550(long a0)
{
  int v1;
  char *v2; // rax
  long v3;
  long v4;
  unsigned int v5; // stack - 0x24
  unsigned char *v6;
  unsigned long v7; // r8
  
  v4 = *(long *)(a0 + 0x28);
  if (!v4) {
    v2 = dcgettext(NULL,"unbalanced [",5);
    sub_6a10(v2); // no-return
  }
  v6 = *(unsigned char **)(a0 + 0x20);
  v2 = (unsigned long)(unsigned long)*v6;
  v1 = *(int *)(a0 + 0x51c + (long)v2 * 4);
  if (v1 != -1) { // branch-flip
label_12589:
    *(int *)(a0 + 0x48) = v1;
  }
  else {
    v2 = (long)sub_1acc0(&v5,v6,v4,a0 + 0x110);
    if (0xfffffffffffffffd <= (unsigned long)((long)v2 - 1U)) {
      *(unsigned long *)(a0 + 0x110) = 0;
      v6 = *(unsigned char **)(a0 + 0x20);
      v4 = *(long *)(a0 + 0x28);
      goto label_12589;
    }
    v6 = *(unsigned char **)(a0 + 0x20);
    *(unsigned int *)(a0 + 0x48) = v5;
    v4 = *(long *)(a0 + 0x28);
    if ((int)v2 != 1) {
      v2 = (long)(long)(int)v2;
      v7 = 0xffffffff;
      v3 = (long)v2;
      goto label_1259a;
    }
  }
  v7 = (unsigned long)*v6;
  v2 = (char *)0x1;
  v3 = 1;
label_1259a:
  *(unsigned char **)(a0 + 0x20) = &v6[(long)v2];
  *(long *)(a0 + 0x28) = v4 - v3;
  v2 = (char *)v7;
  return v2;
}

// Function: sub_12650 @ 0x12650
long sub_12650(long a0) // early-return, ternary
{
  char v1;
  unsigned long *v10;
  unsigned long v11;
  unsigned long v12;
  void *v13;
  char v14 [16]; // stack - 0x118
  int v15; // stack - 0xf8
  char v16 [40]; // stack - 0x68
  char *v17;
  int v18;
  unsigned int v19; // r12d
  unsigned char v2;
  long *v20; // r12
  int v21;
  int v22;
  long v23;
  unsigned char v24; // bl
  int v25; // stack - 0x12c
  char v26; // stack - 0x125
  unsigned int v27; // stack - 0x120
  char v28 [16]; // stack - 0x108
  unsigned int v3; // eax
  int v4; // eax
  unsigned long v5; // rax
  unsigned long v6;
  long v7;
  long v8;
  char *v9;
  
  *(unsigned long *)(a0 + 0x68) = 0;
  v14._0_16_ = 0;
  v28._0_16_ = 0;
  v3 = sub_12550(a0);
  v26 = '\x01';
  v27 = v3;
  if (v3 == 0x5e) {
    v3 = sub_12550(a0);
    v26 = *(char *)(a0 + 0x419);
  }
  v25 = *(int *)(a0 + 0x48);
  v24 = v3 == 0x3a;
  v11 = (unsigned long)v3;
  do {
    v24 &= 0xfd;
    if ((int)v11 != 0x5b) { // branch-flip
      if (((int)v11 == 0x5c) && (*(unsigned char *)(a0 + 0x1c0) & 1)) {
        v11 = (unsigned long)(unsigned int)sub_12550(a0);
        v25 = *(int *)(a0 + 0x48);
      }
label_126d7:
      v5 = sub_12550(a0);
      v22 = *(int *)(a0 + 0x48);
      if ((int)v5 != 0x2d) {
        v5 &= 0xffffffff;
        v18 = v25;
label_1275f:
        v25 = v22;
        v2 = ((int)v11 != 0x3a) * '\x02' + 2;
        v22 = v25;
        goto label_1276b;
      }
label_126ec:
      v4 = sub_12550(a0);
      v21 = *(int *)(a0 + 0x48);
      v18 = v25;
      if (v4 != 0x5b) { // branch-flip
        if (v4 == 0x5d) {
          v9 = *(char **)(a0 + 0x20);
label_12b34:
          *(long *)(a0 + 0x28) = *(long *)(a0 + 0x28) + 1;
          *(char **)(a0 + 0x20) = &v9[-1];
          v5 = 0x2d;
          goto label_1275f;
        }
        if ((v4 == 0x5c) && (*(unsigned char *)(a0 + 0x1c0) & 1)) {
          v4 = sub_12550(a0);
          v21 = *(int *)(a0 + 0x48);
        }
      }
      else {
        v9 = *(char **)(a0 + 0x20);
        if (*v9 == '.') {
          v26 = '\0';
          goto label_12b34;
        }
      }
      v24 |= 8;
      v3 = sub_12550(a0);
      v22 = *(int *)(a0 + 0x48);
      v5 = (unsigned long)v3;
      if ((v21 == v25) && (v25 != -1)) {
        goto label_1275f;
      }
      v25 = v22;
      if ((*(char *)(a0 + 0x419)) || ((unsigned char)((char)v4 - 0x30U) <= 9 && (unsigned char)((char)v11 - 0x30U) <= 9)) {
        if ((int)v11 <= v4) {
          v1 = *(char *)(a0 + 0x1cd);
          v23 = (long)(int)v11 * 2;
          do {
            while ((!v1 || (!(*(unsigned short *)(*(long *)__ctype_b_loc() + v23) & 0x400)))) {
              v19 = (int)v11 + 1;
              v23 += 2;
              *(unsigned long *)&v14[(v11 >> 6) * 8] = *(unsigned long *)&v14[(v11 >> 6) * 8] | 1L << ((unsigned char)v11 & 0x3f);
              v11 = (unsigned long)v19;
              if (v4 < (int)v19) goto label_12cd7;
            }
            v19 = (int)v11 + 1;
            v23 += 2;
            sub_f960(v11,v14);
            v11 = (unsigned long)v19;
          } while ((int)v19 <= v4);
        }
label_12cd7:
        v5 = (unsigned long)v3;
      }
      else {
        v26 = '\0';
      }
    }
    else {
      v3 = sub_12550(a0);
      v22 = *(int *)(a0 + 0x48);
      v18 = v25;
      if (v3 != 0x3a) {
        if ((v3 == 0x2e) || (v3 == 0x3d)) goto label_129f0;
        if (v3 == 0x100) goto label_126d7;
        if (v3 != 0x2d) {
          v5 = (unsigned long)v3;
          v11 = 0x5b;
          v2 = 4;
          goto label_1276b;
        }
        goto label_126ec;
      }
      if (*(unsigned char *)(a0 + 0x1c0) & 4) {
label_129f0:
        v22 = 0;
        do {
          v12 = sub_12550(a0);
          v23 = *(long *)(a0 + 0x28);
          v1 = v16[0];
label_12a05:
          v16[0] = v1;
          if ((!v23) || ((v3 == (unsigned int)v12 && (**(char **)(a0 + 0x20) == ']')))) {
            v16[v22] = '\0';
            sub_12550(a0);
            if (v3 != 0x3a) goto label_12ace;
            v9 = v16;
            if (*(char *)(a0 + 0x1cd)) {
              v9 = "alpha";
              if ((strcmp(v16,"upper")) && (strcmp(v16,"lower")))
                v9 = v16;
            }
            v20 = (long *)0x2a898;
            v22 = 0;
            v17 = "alpha";
            while (strcmp(v9,v17)) {
              v17 = (char *)*v20;
              v20 = &v20[3];
              v22 += 1;
              if (!v17)
                sub_6a10(dcgettext(NULL,"invalid character class",5)); // no-return
            }
            v11 = 0;
            if ((*(char *)(a0 + 0x418)) && (!*(char *)((long)v22 * 0x18 + 0x2a890))) {
label_12ace:
              v26 = '\0';
            }
            else {
              v13 = *(void **)((long)v22 * 0x18 + 0x2a888);
              do {
                if ((*v13)(v11))
                  *(unsigned long *)&v14[(v11 >> 6) * 8] = *(unsigned long *)&v14[(v11 >> 6) * 8] | 1L << ((unsigned char)v11 & 0x3f);
                v3 = (int)v11 + 1;
                v11 = (unsigned long)v3;
              } while (v3 != 0x100);
            }
            v24 |= 8;
            v5 = (unsigned long)(unsigned int)sub_12550(a0);
            v25 = *(int *)(a0 + 0x48);
            goto label_12789;
          }
          if (v22 <= 0x1f) goto label_12a1a;
          v16[0] = '\0';
        } while( true );
      }
      v11 = 0x5b;
      v5 = 0x3a;
      v2 = 4;
label_1276b:
      v25 = v22;
      v24 |= v2;
      if (*(char *)(a0 + 0x418)) { // branch-flip
        if (v18 != -1) { // branch-flip
          if (*(char *)(a0 + 0x1cd)) { // branch-flip
            v22 = sub_16380(v18,(long)&v15 + 4) + 1;
            v15._0_4_ = v18;
            if (v22 <= 0) goto label_12789;
          }
          else {
            v22 = 1;
            v15._0_4_ = v18;
          }
          v23 = 0;
          while( true ) {
            v3 = wctob(v18);
            if (0 <= (int)v3) // branch-flip
              *(unsigned long *)&v14[(unsigned long)(v3 >> 6) * 8] = *(unsigned long *)&v14[(unsigned long)(v3 >> 6) * 8] | 1L << ((unsigned char)v3 & 0x3f);
            else {
              v8 = *(long *)(a0 + 0x68);
              v7 = *(long *)(a0 + 0x60);
              if (*(long *)(a0 + 0x70) <= v8) {
                v7 = sub_1f1f0(v7,a0 + 0x70,1,0xffffffffffffffff,4);
                v8 = *(long *)(a0 + 0x68);
              }
              *(long *)(a0 + 0x60) = v7;
              *(long *)(a0 + 0x68) = v8 + 1;
              *(unsigned int *)(v7 + v8 * 4) = *(unsigned int *)((long)&v15 + v23 * 4);
            }
            v23 += 1;
            if (v22 <= (int)v23) break;
            v18 = *(int *)((long)&v15 + v23 * 4);
          }
        }
        else {
          v26 = '\0';
        }
      }
      else {
        v6 = v11;
        if (*(char *)(a0 + 0x1cd)) {
          v6 = (unsigned long)(int)v11;
          if (*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + v6 * 2) & 4) {
            sub_f960(v11,v14);
            goto label_12789;
          }
        }
        *(unsigned long *)&v14[(v6 >> 6 & 0x3ffffff) * 8] = *(unsigned long *)&v14[(v6 >> 6 & 0x3ffffff) * 8] | 1L << ((unsigned char)v6 & 0x3f);
      }
    }
label_12789:
    v11 = v5;
    if ((int)v5 == 0x5d) {
      if (v24 == 7) {
        v13 = (*(unsigned char *)(a0 + 0x1c8) & 4) ? sub_6a10 : sub_6a40; // branch-flip
        (*v13)(dcgettext(NULL,"character class syntax is [[:space:]], not [:space:]",5));
      }
      if (!v26)
        return 0x112;
      if (*(char *)(a0 + 0x418)) { // branch-flip
        if ((v27 == 0x5e) || (*(long *)(a0 + 0x68))) {
          v12 = 0xffffffffffffffff;
          *(bool *)(a0 + 0x58) = v27 == 0x5e;
          if (((v14._0_8_ || v14._8_8_) || v28._0_8_) || v28._8_8_)
            v12 = sub_10890(a0,v14);
          *(unsigned long *)(a0 + 0x50) = v12;
          return 0x113;
        }
      }
      else {
        v10 = (unsigned long *)v14;
        if (v27 == 0x5e) {
          do {
            *v10 = ~*v10;
            v10 = &v10[1];
          } while (&v15 != v10);
          if (*(unsigned char *)(a0 + 0x1c1) & 1)
            v14._0_8_ &= 0xfffffffffffffbff;
        }
      }
      return sub_10890(a0,v14) + 0x114;
    }
  } while( true );
label_12a1a:
  v23 = (long)v22;
  v22 += 1;
  v16[v23] = (char)v12;
  v12 = sub_12550(a0);
  v23 = *(long *)(a0 + 0x28);
  v1 = v16[0];
  goto label_12a05;
}

// Function: sub_12df0 @ 0x12df0
long sub_12df0(long a0) // early-return
{
  unsigned char v1;
  unsigned long v10;
  unsigned char *v11;
  char v12 [16]; // stack - 0xc8
  unsigned int v13; // stack - 0xcc
  unsigned long v14 [13];
  unsigned char *v15;
  unsigned char v16; // r13b
  bool v17;
  char v18 [16]; // stack - 0xb8
  unsigned int v2;
  int v3;
  unsigned int v4; // eax
  int v5;
  long v6;
  long v7; // rax
  unsigned long *v8;
  char *v9; // rax
  
  v5 = 2;
  v16 = 0;
label_12e27:
  v6 = *(long *)(a0 + 0x28);
  if (!v6) {
    *(unsigned long *)(a0 + 0x30) = 0xffffffffffffffff;
    return -1;
  }
  v15 = *(unsigned char **)(a0 + 0x20);
  v3 = *(int *)(a0 + 0x51c + (unsigned long)*v15 * 4);
  if (v3 != -1) { // branch-flip
label_12e4b:
    *(int *)(a0 + 0x48) = v3;
  }
  else {
    v7 = sub_1acc0(&v13,v15,v6,a0 + 0x110);
    if (0xfffffffffffffffd <= (unsigned long)(v7 - 1U)) {
      *(unsigned long *)(a0 + 0x110) = 0;
      v15 = *(unsigned char **)(a0 + 0x20);
      v6 = *(long *)(a0 + 0x28);
      goto label_12e4b;
    }
    v6 = *(long *)(a0 + 0x28);
    v15 = *(unsigned char **)(a0 + 0x20);
    *(unsigned int *)(a0 + 0x48) = v13;
    v3 = (int)v7;
    if (v3 != 1) {
      v4 = 0xffffffff;
      *(unsigned char **)(a0 + 0x20) = &v15[v3];
      *(long *)(a0 + 0x28) = v6 - v3;
      goto label_12e90;
    }
  }
  v1 = *v15;
  v4 = (unsigned int)v1;
  v11 = &v15[1];
  v6 -= 1;
  *(unsigned char **)(a0 + 0x20) = v11;
  *(long *)(a0 + 0x28) = v6;
  switch(v4 - 10 & 0xff) {
    case 0:
      if (!(*(unsigned char *)(a0 + 0x1c1) & 8)) {
        v4 = 10;
        goto label_12e90;
      }
      if (v16) {
        v4 = 10;
        goto label_12ea2;
      }
      break;
    default:
      goto label_12e90;
    case 0x1a:
      if (v16) // branch-flip
        v4 = 0x24;
      else {
        v10 = *(unsigned long *)(a0 + 0x1c0);
        if (((((v10 & 8) || (!v6)) || ((v4 = (unsigned int)(v10 >> 0xd) ^ 1, (long)(unsigned long)(v4 & 1) < v6 && (v11[(unsigned long)(v15[1] == 0x5c) & (unsigned long)(v4 & 1)] == 0x29)))) || ((v4 = (unsigned int)(v10 >> 0xf) ^ 1, (long)(unsigned long)(v4 & 1) < v6 && (v11[(unsigned long)(v15[1] == 0x5c) & (unsigned long)(v4 & 1)] == 0x7c)))) || ((v4 = 0x24, v10 & 0x800 && ((1 <= v6 && (v15[1] == 10)))))) goto label_12ec3;
      }
      goto label_12ea2;
    case 0x1d:
      if (!v16) {
        v4 = 0x27;
        goto label_12ea2;
      }
      if (!(*(unsigned char *)(a0 + 0x1c2) & 8)) {
label_12ec3:
        *(unsigned long *)(a0 + 0x30) = 0x10d;
        return 0x10d;
      }
      v4 = 0x27;
      goto label_12e95;
    case 0x1e:
      v4 = 0x28;
      if ((((unsigned char)(*(unsigned long *)(a0 + 0x1c0) >> 0xd) ^ 1) & 1) == v16) {
        *(long *)(a0 + 0x38) = *(long *)(a0 + 0x38) + 1;
        *(char *)(a0 + 0x78) = 1;
        *(unsigned long *)(a0 + 0x30) = 0x107;
        return 0x107;
      }
      goto label_12ea2;
    case 0x1f:
      if (((((unsigned char)(*(unsigned long *)(a0 + 0x1c0) >> 0xd) ^ 1) & 1) == v16) && ((*(long *)(a0 + 0x38) || (!(*(unsigned long *)(a0 + 0x1c0) & 0x20000))))) {
        *(char *)(a0 + 0x78) = 0;
        *(long *)(a0 + 0x38) = *(long *)(a0 + 0x38) + -1;
        *(unsigned long *)(a0 + 0x30) = 0x108;
        return 0x108;
      }
      v4 = 0x29;
      goto label_12ea2;
    case 0x20:
      if (!v16) {
        if (!*(char *)(a0 + 0x78)) {
label_12ec3:
          *(unsigned long *)(a0 + 0x30) = 0x102;
          return 0x102;
        }
        if (*(unsigned char *)(a0 + 0x1c0) & 0x10) {
          if (*(unsigned char *)(a0 + 0x1c8) & 0x10)
            sub_6a40(dcgettext(NULL,"* at start of expression",5));
          goto label_12ec3;
        }
      }
      v4 = 0x2a;
      goto label_12ea2;
    case 0x21:
      v10 = *(unsigned long *)(a0 + 0x1c0);
      if (!(v10 & 0x400)) {
        if (((unsigned char)(v10 >> 1) & 1) != v16) {
          v4 = 0x2b;
          goto label_12ea2;
        }
        if (!*(char *)(a0 + 0x78)) {
label_12ec3:
          *(unsigned long *)(a0 + 0x30) = 0x103;
          return 0x103;
        }
        if (v10 & 0x10) {
          if (*(unsigned char *)(a0 + 0x1c8) & 0x20)
            sub_6a40(dcgettext(NULL,"+ at start of expression",5));
          goto label_12ec3;
        }
      }
      v4 = 0x2b;
      goto label_12e90;
    case 0x24:
      if (!v16) {
        if (*(long *)(a0 + 0x18) < 0) {
          v18._8_8_ = 0xffffffffffffffff;
          v18._0_8_ = 0xffffffffffffffff;
          v10 = -(unsigned long)((*(unsigned long *)(a0 + 0x1c0) & 0x40) == 0) & 0xfffffffffffffc00;
          v12._0_8_ = v10 - 1;
          if (*(unsigned long *)(a0 + 0x1c0) & 0x80)
            v12._0_8_ = v10 - 2;
          v12._8_8_ = 0xffffffffffffffff;
          if (*(char *)(a0 + 0x418)) {
            v10 = 0;
            do {
              if (*(int *)(a0 + 0x51c + v10 * 4) == -1)
                *(unsigned long *)&v12[(v10 >> 6 & 0x3ffffff) * 8] = *(unsigned long *)&v12[(v10 >> 6 & 0x3ffffff) * 8] & ~(1L << ((unsigned char)v10 & 0x3f));
              v10 += 1;
            } while (v10 != 0x100);
          }
          *(unsigned long *)(a0 + 0x18) = sub_10890(a0,v12);
        }
        *(char *)(a0 + 0x78) = 0;
        v6 = 0x10a;
        if (!*(char *)(a0 + 0x418))
          v6 = *(long *)(a0 + 0x18) + 0x114;
        goto label_12ebf;
      }
      v4 = 0x2e;
      goto label_12ea2;
    case 0x27:
    case 0x28:
    case 0x29:
    case 0x2a:
    case 0x2b:
    case 0x2c:
    case 0x2d:
    case 0x2e:
    case 0x2f:
      if (!v16) goto label_12ea2;
      if (!(*(unsigned char *)(a0 + 0x1c1) & 0x40)) {
        *(char *)(a0 + 0x78) = 0;
        *(unsigned long *)(a0 + 0x30) = 0x112;
        return 0x112;
      }
      goto label_12e95;
    case 0x32:
      if (!v16) {
        v4 = 0x3c;
        goto label_12ea2;
      }
      if (!(*(unsigned char *)(a0 + 0x1c2) & 8)) {
        *(unsigned long *)(a0 + 0x30) = 0x10e;
        return 0x10e;
      }
      v4 = 0x3c;
      goto label_12e95;
    case 0x34:
      if (!v16) {
        v4 = 0x3e;
        goto label_12ea2;
      }
      if (!(*(unsigned char *)(a0 + 0x1c2) & 8)) {
        *(unsigned long *)(a0 + 0x30) = 0x10f;
        return 0x10f;
      }
      v4 = 0x3e;
      goto label_12e95;
    case 0x35:
      v10 = *(unsigned long *)(a0 + 0x1c0);
      if (!(v10 & 0x400)) {
        if (((unsigned char)(v10 >> 1) & 1) != v16) {
          v4 = 0x3f;
          goto label_12ea2;
        }
        if (!*(char *)(a0 + 0x78)) {
label_12ec3:
          *(unsigned long *)(a0 + 0x30) = 0x101;
          return 0x101;
        }
        if (v10 & 0x10) {
          if (*(unsigned char *)(a0 + 0x1c8) & 0x20)
            sub_6a40(dcgettext(NULL,"? at start of expression",5));
          goto label_12ec3;
        }
      }
      v4 = 0x3f;
      goto label_12e90;
    case 0x38:
      if (!v16) {
        v4 = 0x42;
        goto label_12ea2;
      }
      if (!(*(unsigned char *)(a0 + 0x1c2) & 8)) {
        *(unsigned long *)(a0 + 0x30) = 0x111;
        return 0x111;
      }
      v4 = 0x42;
      goto label_12e95;
    case 0x49:
    case 0x69:
      if (!v16) goto label_12ea2;
      v10 = (unsigned long)((unsigned int)*(unsigned long *)(a0 + 0x1c0) & 0x80000);
      if (!(*(unsigned long *)(a0 + 0x1c0) & 0x80000)) {
        if (!*(char *)(a0 + 0x418)) {
          v12._0_16_ = 0;
          v18._0_16_ = 0;
          v6 = *(long *)__ctype_b_loc();
          do {
            if (*(unsigned char *)(v6 + 1 + v10 * 2) & 0x20)
              *(unsigned long *)&v12[(v10 >> 6 & 0x3ffffff) * 8] = *(unsigned long *)&v12[(v10 >> 6 & 0x3ffffff) * 8] | 1L << ((unsigned char)v10 & 0x3f);
            v10 += 1;
          } while (v10 != 0x100);
          v8 = (unsigned long *)v12;
          if (v4 == 0x53) {
            do {
              *v8 = ~*v8;
              v8 = &v8[1];
            } while (v8 != v14);
          }
label_12ec3:
          *(char *)(a0 + 0x78) = 0;
          v6 = sub_10890(a0) + 0x114;
          *(long *)(a0 + 0x30) = v6;
          return v6;
        }
        v17 = v4 == 0x73;
        v9 = "^[:space:]]";
        goto label_12ec3;
      }
      goto label_12e95;
    case 0x4d:
    case 0x6d:
      if (!v16) goto label_12ea2;
      v10 = (unsigned long)((unsigned int)*(unsigned long *)(a0 + 0x1c0) & 0x80000);
      if (!(*(unsigned long *)(a0 + 0x1c0) & 0x80000)) {
        if (!*(char *)(a0 + 0x418)) {
          v12._0_16_ = 0;
          v18._0_16_ = 0;
          do {
            if (*(char *)(a0 + 0x1cf + v10) == '\x02')
              *(unsigned long *)&v12[(v10 >> 6 & 0x3ffffff) * 8] = *(unsigned long *)&v12[(v10 >> 6 & 0x3ffffff) * 8] | 1L << ((unsigned char)v10 & 0x3f);
            v10 += 1;
          } while (v10 != 0x100);
          v8 = (unsigned long *)v12;
          if (v1 == 0x57) {
            do {
              *v8 = ~*v8;
              v8 = &v8[1];
            } while (v14 != v8);
          }
          goto label_12ec3;
        }
        v17 = v4 == 0x77;
        v9 = "^_[:alnum:]]";
label_12ec3:
        *(char **)(a0 + 0x20) = &v9[v17];
        *(unsigned long *)(a0 + 0x28) = strlen(&v9[v17]);
        v7 = sub_12650(a0);
        *(long *)(a0 + 0x28) = v6;
        *(long *)(a0 + 0x30) = v7;
        *(unsigned char **)(a0 + 0x20) = v11;
        *(char *)(a0 + 0x78) = 0;
        return v7;
      }
      goto label_12e95;
    case 0x51:
      v4 = 0x5b;
      if (v16) goto label_12ea2;
      *(char *)(a0 + 0x78) = 0;
      v6 = sub_12650(a0);
      goto label_12ebf;
    case 0x52:
      if (!v16) goto label_132a2;
      v4 = 0x5c;
      goto label_12ea2;
    case 0x53:
    case 0x73:
      goto label_12ea2;
    case 0x54:
      if (v16) // branch-flip
        v4 = 0x5e;
      else if (((*(unsigned char *)(a0 + 0x1c0) & 8) || ((unsigned long)(*(long *)(a0 + 0x30) - 0x106U) <= 1)) || (v4 = 0x5e, *(long *)(a0 + 0x30) == -1)) goto label_12ec3;
      goto label_12ea2;
    case 0x56:
      if (!v16) {
        v4 = 0x60;
        goto label_12ea2;
      }
      if (!(*(unsigned char *)(a0 + 0x1c2) & 8)) {
label_12ec3:
        *(unsigned long *)(a0 + 0x30) = 0x10c;
        return 0x10c;
      }
      v4 = 0x60;
      goto label_12e95;
    case 0x58:
      if (!v16) {
        v4 = 0x62;
        goto label_12ea2;
      }
      if (!(*(unsigned char *)(a0 + 0x1c2) & 8)) {
        *(unsigned long *)(a0 + 0x30) = 0x110;
        return 0x110;
      }
      v4 = 0x62;
      goto label_12e95;
    case 0x71:
      v10 = *(unsigned long *)(a0 + 0x1c0);
      if (!(v10 & 0x200)) goto label_136f0;
      if ((((unsigned char)(v10 >> 0xc) ^ 1) & 1) != v16) goto label_13700;
      v15 = &v11[v6];
      *(unsigned long *)(a0 + 0x40) = 0xffffffffffffffff;
      goto label_134d6;
    case 0x72:
      if (*(unsigned long *)(a0 + 0x1c0) & 0x400) {
        v4 = 0x7c;
        goto label_12e90;
      }
      v4 = 0x7c;
      if ((((unsigned char)(*(unsigned long *)(a0 + 0x1c0) >> 0xf) ^ 1) & 1) != v16) goto label_12ea2;
    
  }
  *(char *)(a0 + 0x78) = 1;
  *(unsigned long *)(a0 + 0x30) = 0x106;
  return 0x106;
label_134d6:
  if (v15 == v11) goto label_13971;
  v1 = *v11;
  v5 = (int)(char)v1;
  if (10 <= (unsigned char)(v1 - 0x30)) {
    if (v15 != v11) {
      if (v1 == 0x2c) {
        if (*(int *)(a0 + 0x40) <= -1)
          *(unsigned int *)(a0 + 0x40) = 0;
        goto label_139bd;
      }
      *(int *)(a0 + 0x44) = *(int *)(a0 + 0x40);
      if (!v16) goto label_13938;
      goto label_13539;
    }
    goto label_13971;
  }
  if ((0 <= *(int *)(a0 + 0x40)) && (v5 = v5 + *(int *)(a0 + 0x40) * 10, 0x8031 <= v5))
    v5 = 0x8030;
  v11 = &v11[1];
  *(int *)(a0 + 0x40) = v5 + -0x30;
  goto label_134d6;
label_139bd:
  v11 = &v11[1];
  if (v15 == v11) goto label_13986;
  v5 = (int)(char)*v11;
  if (10 <= (unsigned char)(*v11 - 0x30)) {
    if (!v16) goto label_13981;
    if (v15 != v11) goto label_13539;
    goto label_13986;
  }
  if ((0 <= *(int *)(a0 + 0x44)) && (v5 = v5 + *(int *)(a0 + 0x44) * 10, 0x8031 <= v5))
    v5 = 0x8030;
  *(int *)(a0 + 0x44) = v5 + -0x30;
  goto label_139bd;
label_13539:
  v1 = *v11;
  v11 = &v11[1];
  if (v1 == 0x5c) goto label_13981;
label_13546:
  if (v10 & 0x200000) {
label_13700:
    v4 = 0x7b;
    goto label_12ea2;
  }
  if (!*(char *)(a0 + 0x78)) {
    sub_6a10(dcgettext(NULL,"invalid content of \\{\\}",5)); // return-dupe, no-return
  }
  v17 = 1;
  goto label_13562;
label_132a2:
  if (!v6)
    sub_6a10(dcgettext(NULL,"unfinished \\ escape",5)); // no-return
  v16 = 1;
  if (v5 == 1)
    abort(); // no-return
  v5 = 1;
  goto label_12e27;
label_13971:
  if (v16) {
    goto label_13546;
  }
label_13981:
  if (v11 == v15) {
label_13986:
    v11 = v15;
    goto label_13546;
  }
label_13938:
  v1 = *v11;
  v11 = &v11[1];
  if (((v1 != 0x7d) || (*(int *)(a0 + 0x40) < 0)) || ((v5 = *(int *)(a0 + 0x44), 0 <= v5 && (v5 < *(int *)(a0 + 0x40))))) goto label_13546;
  if (!*(char *)(a0 + 0x78)) goto label_13584;
  v17 = 0;
label_13562:
  if (v10 & 0x10) {
    if (*(unsigned char *)(a0 + 0x1c8) & 0x20)
      sub_6a40(dcgettext(NULL,"{...} at start of expression",5));
    if (!v17) {
      v5 = *(int *)(a0 + 0x44);
label_13584:
      if (0x8000 > v5) {
        *(unsigned char **)(a0 + 0x20) = v11;
        *(long *)(a0 + 0x28) = (long)v15 - (long)v11;
        *(char *)(a0 + 0x78) = 0;
        *(unsigned long *)(a0 + 0x30) = 0x104;
        return 0x104;
      }
      sub_6a10(dcgettext(NULL,"regular expression too big",5)); // no-return
    }
    sub_6a10(dcgettext(NULL,"invalid content of \\{\\}",5));
  }
label_136f0:
  v4 = 0x7b;
label_12e90:
  if (v16) {
label_12e95:
    if (*(unsigned char *)(a0 + 0x1c8) & 8) {
      v2 = *(unsigned int *)(a0 + 0x48);
      if (iswprint(v2)) { // branch-flip
        v2 = *(unsigned int *)(a0 + 0x48);
        if (iswspace(v2)) // branch-flip
          v8 = (unsigned long *)dcgettext(NULL,"stray \\ before white space",5);
        else {
          v2 = *(unsigned int *)(a0 + 0x48);
          v8 = v14;
          v9 = dcgettext(NULL,"stray \\ before %lc",5);
          if (100 <= (unsigned int)__snprintf_chk((char *)v8,100,1,100,v9,(unsigned long)v2))
            v8 = (unsigned long *)dcgettext(NULL,"stray \\",5);
        }
      }
      else {
        v8 = (unsigned long *)dcgettext(NULL,"stray \\ before unprintable character",5);
      }
      sub_6a40(v8);
    }
  }
label_12ea2:
  *(char *)(a0 + 0x78) = 0;
  if (*(char *)(a0 + 0x418)) {
    *(unsigned long *)(a0 + 0x30) = 0x109;
    return 0x109;
  }
  v6 = (long)(int)v4;
  if (*(char *)(a0 + 0x1cd)) {
    v6 = (long)(int)v4;
    if (*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + v6 * 2) & 4) {
      v12._0_16_ = 0;
      v18._0_16_ = 0;
      sub_f960(v4,v12);
      v6 = sub_10890(a0,v12) + 0x114;
      *(long *)(a0 + 0x30) = v6;
      return v6;
    }
  }
label_12ebf:
  *(long *)(a0 + 0x30) = v6;
  return v6;
}

// Function: sub_13af0 @ 0x13af0
void sub_13af0(long a0)
{
  long v1;
  
  sub_13bf0(a0);
  v1 = *(long *)(a0 + 0x80);
  while ((v1 - 0x106U & 0xfffffffffffffffdU && (0 <= v1))) {
    sub_13bf0(a0);
    sub_fb20(a0,0x105);
    v1 = *(long *)(a0 + 0x80);
  }
  while (v1 == 0x106) {
    *(unsigned long *)(a0 + 0x80) = sub_12df0(a0);
    sub_13bf0(a0);
    v1 = *(long *)(a0 + 0x80);
    while ((v1 - 0x106U & 0xfffffffffffffffdU && (0 <= v1))) {
      sub_13bf0(a0);
      sub_fb20(a0,0x105);
      v1 = *(long *)(a0 + 0x80);
    }
    sub_fb20(a0,0x106);
    v1 = *(long *)(a0 + 0x80);
  }
}

// Function: sub_13bf0 @ 0x13bf0
void sub_13bf0(long a0)
{
  unsigned int v1;
  unsigned long v10; // stack - 0xa8
  unsigned long v11; // stack - 0xa0
  unsigned long v2;
  long v3; // rax
  unsigned long v4; // stack - 0xb8
  int v5; // ebp
  int v6;
  unsigned long *v7; // rsp
  long v8;
  unsigned long v9; // stack - 0xb0
  
  v7 = &v4;
  v2 = *(unsigned long *)(a0 + 0x80);
  if ((0x100 <= v2) && ((long)v2 <= 0x109)) {
    if (v2 == 0x109) {
      if (*(int *)(a0 + 0x48) != -1) {
        sub_fc00(a0,*(int *)(a0 + 0x48));
        if (!*(char *)(a0 + 0x1cd)) goto label_13c58;
        v6 = sub_16380(*(unsigned int *)(a0 + 0x48),&v4);
        if (v6 <= 0) goto label_13c58;
        do {
          v1 = *(unsigned int *)v7;
          v7 = (unsigned int *)((long)v7 + 4);
          sub_fc00(a0,v1);
          sub_fb20(a0,0x106);
          if ((unsigned long *)((long)&v4 + (unsigned long)(unsigned int)(v6 - 1) * 4 + 4) == v7) goto label_13c58;
        } while( true );
      }
      sub_fb20(a0,0x112);
      goto label_13c58;
    }
    if (v2 != 0x107) {
      sub_fb20(a0,0x100);
      v2 = *(unsigned long *)(a0 + 0x80);
      goto label_13c75;
    }
    *(unsigned long *)(a0 + 0x80) = sub_12df0(a0);
    sub_13af0(a0);
    if (*(long *)(a0 + 0x80) == 0x108) goto label_13c58;
    sub_6a10(dcgettext(NULL,"unbalanced (",5)); // no-return
  }
  if ((v2 == 0x10a) && (*(char *)(a0 + 0x41a))) {
    v8 = *(long *)(a0 + 200);
    if (!v8) {
      v4 = 0xffffffffffffffff;
      v9 = 0xffffffffffffffff;
      v10 = 0;
      v11 = 0;
      if (!(*(unsigned long *)(a0 + 0x1c0) & 0x40))
        v4 = 0xfffffffffffffbff;
      if (*(unsigned long *)(a0 + 0x1c0) & 0x80)
        v4 &= 0xfffffffffffffffe;
      v8 = 1;
      *(long *)(a0 + 200) = sub_10890(a0,&v4) + 0x114;
      do {
        *(long *)(a0 + 200 + v8 * 8) = sub_10890(a0,v8 * 0x20 + 0x22d20) + 0x114;
        v8 += 1;
      } while (v8 != 9);
      v8 = *(long *)(a0 + 200);
    }
    v6 = 3;
    sub_fb20(a0,v8);
    sub_fb20(a0,*(unsigned long *)(a0 + 0xd0));
    sub_fb20(a0,0xe0);
    sub_fb20(a0,*(unsigned long *)(a0 + 0xe0));
    sub_fb20(a0,0x105);
    sub_fb20(a0,0x106);
    sub_fb20(a0,0xed);
    sub_fb20(a0,*(unsigned long *)(a0 + 0xf0));
    sub_fb20(a0,0x105);
    sub_fb20(a0,0x106);
    sub_fb20(a0,*(unsigned long *)(a0 + 0xe8));
    sub_fb20(a0,0xf0);
    sub_fb20(a0,*(unsigned long *)(a0 + 0xf8));
    sub_fb20(a0,0x105);
    sub_fb20(a0,0x106);
    sub_fb20(a0,0xf4);
    sub_fb20(a0,*(unsigned long *)(a0 + 0x108));
    sub_fb20(a0,0x105);
    sub_fb20(a0,0x106);
    sub_fb20(a0,*(unsigned long *)(a0 + 0x100));
    do {
      sub_fb20(a0,*(unsigned long *)(a0 + 0xd8));
      sub_fb20(a0,0x105);
      sub_fb20(a0,0x106);
      v6 -= 1;
      if (!v6) goto label_13c58;
    } while( true );
  }
label_13c50:
  sub_fb20(a0,v2);
label_13c58:
  do {
    v2 = sub_12df0(a0);
    *(unsigned long *)(a0 + 0x80) = v2;
label_13c75:
    while( true ) {
      if (4 <= v2 - 0x101)
        return;
      if (v2 != 0x104) goto label_13c50;
      v8 = *(long *)(a0 + 0x98);
      v3 = sub_10670(*(unsigned long *)(a0 + 0x90),v8);
      v8 -= v3;
      if (*(long *)(a0 + 0x40)) break;
      *(long *)(a0 + 0x98) = v8;
      *(unsigned long *)(a0 + 0x80) = sub_12df0(a0);
      sub_13bf0(a0);
      v2 = *(unsigned long *)(a0 + 0x80);
    }
    if (*(int *)(a0 + 0x44) < 0)
      sub_fb20(a0,0x103);
    v6 = *(int *)(a0 + 0x40);
    if (!v6) {
      sub_fb20(a0,0x101);
      v6 = *(int *)(a0 + 0x40);
    }
    v5 = 1;
    if (2 <= v6) {
      do {
        v5 += 1;
        sub_10530(a0,v8,v3);
        sub_fb20(a0,0x105);
      } while (v5 < *(int *)(a0 + 0x40));
    }
    if (v5 < *(int *)(a0 + 0x44)) {
      do {
        v5 += 1;
        sub_10530(a0,v8,v3);
        sub_fb20(a0,0x101);
        sub_fb20(a0,0x105);
      } while (v5 < *(int *)(a0 + 0x44));
    }
  } while( true );
}

// Function: sub_14090 @ 0x14090
long sub_14090(long a0,long a1,long *a2,long a3)
{
  unsigned char *v1;
  int v10; // stack - 0x44
  unsigned char *v11;
  long *v12; // rsi
  unsigned long v13;
  long v14;
  int v15; // eax
  long v16;
  int v17; // r15d
  int v18; // stack - 0x54
  long *v2;
  long v3;
  long v4;
  unsigned long *v5;
  long v6;
  unsigned int v7; // edx
  unsigned long *v8;
  long v9;
  
  v11 = (unsigned char *)*a2;
  v18 = *(int *)(a0 + 0x51c + (unsigned long)*v11 * 4);
  v3 = a1;
  if (v18 != -1) { // branch-flip
    *(unsigned long *)(a0 + 0x1a0) = 0;
    v15 = 1;
  }
  else {
    v16 = sub_1acc0(&v10,v11,a3 - (long)v11,a0 + 0x110);
    if (0xfffffffffffffffd <= (unsigned long)(v16 - 1U)) { // branch-flip
      *(unsigned long *)(a0 + 0x110) = 0;
      v15 = 1;
      v11 = (unsigned char *)*a2;
      *(unsigned long *)(a0 + 0x1a0) = 0;
    }
    else {
      v18 = v10;
      v11 = (unsigned char *)*a2;
      v15 = (int)v16;
      *(unsigned long *)(a0 + 0x1a0) = 0;
      if (v15 <= 0) goto label_141bb;
    }
  }
  v17 = 0;
  do {
    v16 = *(long *)(*(long *)(a0 + 0x170) + v3 * 8);
    if ((v16) || (v16 = *(long *)(*(long *)(a0 + 0x178) + v3 * 8), v16)) {
      v13 = (unsigned long)*v11;
      v4 = *(long *)(v16 + v13 * 8);
      v1 = v11;
    }
    else {
      sub_109d0(v3,a0,*v11);
      v16 = *(long *)(*(long *)(a0 + 0x170) + v3 * 8);
      if ((!v16) && (v16 = *(long *)(*(long *)(a0 + 0x178) + v3 * 8), !v16))
        __assert_fail("t","dfa.c",0xcce,"transit_state_singlebyte"); // no-return
      v1 = (unsigned char *)*a2;
      v13 = (unsigned long)*v1;
      v4 = *(long *)(v16 + v13 * 8);
    }
    if (v4 == -2) {
      sub_109d0(v3,a0,v13);
      v1 = (unsigned char *)*a2;
    }
    v11 = &v1[1];
    v17 += 1;
    *a2 = (long)v11;
    v3 = *(long *)(v16 + (unsigned long)*v1 * 8);
  } while ((v17 < v15) && (*(int *)(a0 + 0x16c) <= v3));
  v15 -= v17;
label_141bb:
  *a2 = (long)&v11[v15];
  if (v18 != -1) {
    v6 = *(long *)(a0 + 0x128);
    v4 = *(long *)(a0 + 0x1b0);
    a1 *= 0x48;
    v16 = v6 + a1;
    if (*(long *)(v16 + 0x40) < 0) {
      v9 = *(long *)(a0 + 0x1b8);
      v14 = v9 + 1;
      if (0x400 <= v9) {
        if (0 <= *(long *)(a0 + 0x160)) {
          v16 = -1;
          do {
            free(*(void **)(v4 + v16 * 8));
            v4 = *(long *)(a0 + 0x1b0);
            *(unsigned long *)(v4 + v16 * 8) = 0;
            v16 += 1;
          } while (v16 < *(long *)(a0 + 0x160));
          v6 = *(long *)(a0 + 0x128);
          v16 = v6 + a1;
        }
        v14 = *(long *)(a0 + 0x130);
        if (1 <= v14) {
          v8 = (unsigned long *)(v6 + 0x40);
          do {
            *v8 = 0xffffffffffffffff;
            v8 = &v8[9];
          } while ((unsigned long *)(v6 + 0x40 + v14 * 0x48) != v8);
        }
        v14 = 1;
        v9 = 0;
      }
      *(long *)(a0 + 0x1b8) = v14;
      *(long *)(v16 + 0x40) = v9;
    }
    v16 = v3 * 8;
    v2 = (long *)(v4 + v16);
    v4 = *v2;
    if (v4) { // branch-flip
      v14 = v6 + a1;
      v4 = *(long *)(v4 + *(long *)(v14 + 0x40) * 8);
      if (0 <= v4)
        return v4;
    }
    else {
      *v2 = sub_1ef10(0x2000);
      v5 = *(unsigned long **)(*(long *)(a0 + 0x1b0) + v3 * 8);
      v8 = &v5[0x400];
      do {
        *v5 = 0xffffffffffffffff;
        v5 = &v5[1];
      } while (v8 != v5);
      v6 = *(long *)(a0 + 0x128);
      v14 = v6 + a1;
    }
    v4 = a0 + 0x198;
    if (v3 != -1) // branch-flip
      sub_fec0(v14 + 0x28,v6 + 8 + v3 * 0x48,0xffffffff,v4);
    else {
      sub_10010(v14 + 0x28,v4);
    }
    v2 = *(long **)(a0 + 0x198);
    if (1 <= *(long *)(a0 + 0x1a0)) { // branch-flip
      v7 = 0;
      v12 = &v2[*(long *)(a0 + 0x1a0) * 2];
      do {
        v3 = *v2;
        v2 = &v2[2];
        v7 |= *(unsigned int *)(*(long *)(a0 + 0x158) + v3 * 4);
      } while (v12 != v2);
      v7 ^= 7;
    }
    else {
      v7 = 7;
    }
    v3 = sub_10290(a0,v4,v7);
    sub_fd10(a0);
    *(long *)(*(long *)(*(long *)(a0 + 0x1b0) + v16) + *(long *)(*(long *)(a0 + 0x128) + 0x40 + a1) * 8) = v3;
  }
  return v3;
}

// Function: sub_144a0 @ 0x144a0
unsigned char * sub_144a0(long a0,unsigned char *a1,unsigned char *a2,char a3,long *a4,char a5)
{
  unsigned long *v1;
  long v10;
  unsigned long v11; // rax
  long v12;
  unsigned long *v13;
  unsigned char *v14;
  unsigned char *v15; // stack - 0x48
  char v16 [4];
  long v17;
  long v18; // stack - 0xa0
  unsigned char v2;
  unsigned char v3;
  char v4;
  unsigned long v5;
  unsigned char *v6;
  unsigned long v7; // rax
  long v8;
  long v9;
  
  if (0x400 <= *(long *)(a0 + 0x130)) { // branch-flip
    v17 = (long)*(int *)(a0 + 0x16c);
    if (v17 < *(long *)(a0 + 0x130)) {
      do {
        v9 = v17 + 1;
        free(*(void **)(*(long *)(a0 + 0x128) + 8 + v17 * 0x48));
        free(*(void **)(*(long *)(a0 + 0x128) + 0x28 + v17 * 0x48));
        v17 = v9;
      } while (v9 < *(long *)(a0 + 0x130));
      v17 = (long)*(int *)(a0 + 0x16c);
    }
    v9 = *(long *)(a0 + 0x170);
    *(long *)(a0 + 0x130) = v17;
    v17 = *(long *)(a0 + 0x160);
    if (v9) {
      if (1 <= v17) {
        v12 = 0;
        do {
          free(*(void **)(v9 + v12 * 8));
          free(*(void **)(*(long *)(a0 + 0x178) + v12 * 8));
          v9 = *(long *)(a0 + 0x170);
          *(unsigned long *)(*(long *)(a0 + 0x178) + v12 * 8) = 0;
          v17 = *(long *)(a0 + 0x160);
          *(unsigned long *)(v9 + v12 * 8) = 0;
          v12 += 1;
        } while (v12 < v17);
      }
      *(unsigned int *)(a0 + 0x168) = 0;
    }
    if ((*(char *)(a0 + 0x418)) && (v9 = *(long *)(a0 + 0x1b0), v9)) {
      if (0 <= v17) {
        v12 = -1;
        do {
          free(*(void **)(v9 + v12 * 8));
          v9 = *(long *)(a0 + 0x1b0);
          v17 = *(long *)(a0 + 0x160);
          *(unsigned long *)(v9 + v12 * 8) = 0;
          v12 += 1;
        } while (v12 < v17);
      }
      if (1 <= *(int *)(a0 + 0x16c)) {
        v13 = (unsigned long *)(*(long *)(a0 + 0x128) + 0x40);
        v1 = &v13[(long)*(int *)(a0 + 0x16c) * 9];
        do {
          *v13 = 0xffffffffffffffff;
          v13 = &v13[9];
        } while (v1 != v13);
      }
      *(unsigned long *)(a0 + 0x1b8) = 0;
    }
  }
  else {
    v17 = *(long *)(a0 + 0x160);
  }
  if (!v17)
    sub_fd10(a0);
  v17 = *(long *)(a0 + 0x170);
  v2 = *(unsigned char *)(a0 + 0x1ce);
  v3 = *a2;
  *a2 = v2;
  v15 = a1;
  if ((a5) && (*(unsigned long *)(a0 + 0x110) = 0, !*(long *)(a0 + 0x1a8))) {
    v5 = *(unsigned long *)(a0 + 0xb0);
    v11 = sub_1f050(v5,0x10);
    *(unsigned long *)(a0 + 0x1a8) = v5;
    *(unsigned long *)(a0 + 0x198) = v11;
    *(unsigned long *)(a0 + 0x1a0) = 0;
  }
  v13 = (unsigned long *)(a0 + 0x110);
  v18 = 0;
  v12 = 0;
  v9 = 0;
  v14 = a1;
label_14550:
  do {
    while( true ) {
      v10 = *(long *)(v17 + v9 * 8);
      if (!v10) break;
      v7 = (unsigned long)*a1;
      v12 = v9; // branch-flip
      if (*(int *)(a0 + 0x16c) <= v9) {
        if (a5) goto label_14781;
        v10 = *(long *)(v10 + v7 * 8);
        v15 = &a1[1];
        v9 = *(long *)(v17 + v10 * 8);
        if (!v9) {
          v9 = v10;
          a1 = v15;
          break;
        }
        v9 = *(long *)(v9 + (unsigned long)a1[1] * 8);
        v12 = v10;
label_14863:
        a1 = &v15[1];
        v15 = a1;
      }
      else {
        if (!a5) {
          if (v9 == *(long *)(v10 + v7 * 8)) goto label_14670;
label_14690:
          v10 = *(long *)(v10 + v7 * 8);
          v15 = &a1[1];
          v8 = *(long *)(v17 + v10 * 8);
          if (v8) {
            v9 = *(long *)(v8 + (unsigned long)a1[1] * 8);
            v12 = v10;
            if (v10 == v9) {
              v6 = &a1[2];
              do {
                v15 = v6;
                v9 = *(long *)(v8 + (unsigned long)*v15 * 8);
                v6 = &v15[1];
              } while (v9 == v10);
            }
            goto label_14863;
          }
          v9 = v10;
          a1 = v15;
          break;
        }
        if ((!*(long *)(*(long *)(a0 + 0x128) + 0x30 + v9 * 0x48)) && (v9 == *(long *)(v10 + v7 * 8))) {
label_14670:
          do {
            a1 = &a1[1];
            v7 = (unsigned long)*a1;
          } while (*(long *)(v10 + v7 * 8) == v9);
          v15 = a1;
          if (!a5) goto label_14690;
        }
        v7 = (unsigned long)*a1;
        if (!*(char *)(a0 + 0x2cf + v7)) {
          if (v14 < a1) {
            do {
              while (*(int *)(a0 + 0x51c + (unsigned long)*v14 * 4) != -1) {
label_145e0:
                v14 = &v14[1];
                if (v14 >= a1) goto label_14630;
              }
              v8 = sub_1acc0(v16,v14,(long)a2 - (long)v14,v13);
              if ((unsigned long)(v8 - 1U) > 0xfffffffffffffffc) {
                *v13 = 0;
                goto label_145e0;
              }
              v14 = &v14[(int)v8];
            } while (a1 > v14);
label_14630:
          }
          a1 = v14;
          v7 = (unsigned long)*a1;
        }
        v14 = a1;
        v15 = a1;
label_14781:
        if (((*(long *)(*(long *)(a0 + 0x128) + 0x30 + v9 * 0x48)) && (*(int *)(a0 + 0x51c + v7 * 4) == -1)) && (a1 < a2)) {
label_14a18:
          v10 = sub_14090(a0,v9,&v15,a2);
          v17 = *(long *)(a0 + 0x170);
          v12 = v9;
          v9 = v10;
          a1 = v15;
          v14 = v15;
        }
        else {
          a1 = &a1[1];
          v9 = *(long *)(v10 + v7 * 8);
          v15 = a1;
        }
      }
    }
    if (0 > v9) {
      if (v9 != -2) { // branch-flip
        if (((a2 < a1) || (a1[-1] != v2)) || (v9 = *(long *)(*(long *)(a0 + 0x188) + v12 * 8), v9 < 0)) {
          a1 = NULL;
label_14a77:
          if (a4)
            *a4 = *a4 + v18;
          *a2 = v3;
          return a1;
        }
        v18 += 1;
        v14 = a1;
        if (!a3) {
          v4 = *(char *)(a0 + 0x1cf + (unsigned long)v2);
          v9 = 0;
          if (v4 != '\x04') {
            if (v4 != '\x02') // branch-flip
              v9 = *(long *)(a0 + 400);
            else {
              v9 = (long)(*(int *)(a0 + 0x16c) + -1);
            }
          }
        }
      }
      else {
        v9 = sub_109d0(v12,a0,a1[-1]);
        v17 = *(long *)(a0 + 0x170);
      }
      goto label_14550;
    }
    v7 = (unsigned long)*a1;
    v10 = *(long *)(*(long *)(a0 + 0x178) + v9 * 8);
    if (v10) { // branch-flip
      v12 = (long)(int)(unsigned int)*a1;
      if (*(unsigned char *)(*(long *)(a0 + 0x180) + v9) & *(unsigned char *)(a0 + 0x1cf + v12)) goto label_14a77;
      if ((a2 == a1) && (v8 = *(long *)(a0 + 0x128) + v9 * 0x48, (int)(unsigned int)*(unsigned short *)(v8 + 0x22) >> 6 & (unsigned int)*(unsigned char *)(v8 + 0x20) & 7)) {
        a1 = a2;
        goto label_14a77;
      }
      if (a5) {
        if (v9 < *(int *)(a0 + 0x16c)) {
          if (!*(char *)(a0 + 0x2cf + v12)) {
            for (; v14 < a1; v14 = &v14[1]) {
              while (*(int *)(a0 + 0x51c + (unsigned long)*v14 * 4) == -1) {
                v12 = sub_1acc0(v16,v14,(long)a2 - (long)v14,v13);
                if (0xfffffffffffffffd <= (unsigned long)(v12 - 1U)) {
                  *v13 = 0;
                  break;
                }
                v14 = &v14[(int)v12];
                if (a1 <= v14) goto label_14a02;
              }
            }
label_14a02:
            v7 = (unsigned long)*v14;
            a1 = v14;
          }
          v14 = a1;
          v15 = a1;
        }
        if (((*(long *)(*(long *)(a0 + 0x128) + v9 * 0x48 + 0x30)) && (*(int *)(a0 + 0x51c + v7 * 4) == -1)) && (a1 < a2)) goto label_14a18;
        v10 = *(long *)(*(long *)(a0 + 0x178) + v9 * 8);
      }
      a1 = &a1[1];
      v12 = v9;
      v9 = *(long *)(v10 + v7 * 8);
      v15 = a1;
    }
    else {
      sub_109d0(v9,a0,v7);
      v17 = *(long *)(a0 + 0x170);
    }
  } while( true );
}

// Function: sub_14cc0 @ 0x14cc0
void sub_14cc0(void)
{
  sub_144a0(); // tail-call
}

// Function: sub_14cd0 @ 0x14cd0
void sub_14cd0(void)
{
  sub_144a0(); // tail-call
}

// Function: sub_14cf0 @ 0x14cf0
void sub_14cf0(unsigned long a0,unsigned long a1,long a2)
{
  *(unsigned long *)(a2 + 0x20) = a0;
  *(unsigned long *)(a2 + 0x28) = a1;
  *(unsigned long *)(a2 + 0x30) = 0xffffffffffffffff;
  *(char *)(a2 + 0x78) = 1;
  if (!*(char *)(a2 + 0x1cc))
    sub_6a10(dcgettext(NULL,"no syntax specified",5)); // no-return
  if (!*(long *)(a2 + 0xb8))
    sub_fb20(a2,0x10b);
  *(unsigned long *)(a2 + 0x80) = sub_12df0(a2);
  *(unsigned long *)(a2 + 0x88) = *(unsigned long *)(a2 + 0xa8);
  sub_13af0(a2);
  if (*(long *)(a2 + 0x80) == -1) {
    sub_fb20(a2,~*(unsigned long *)(a2 + 0xb8));
    sub_fb20(a2,0x105);
    if (!*(long *)(a2 + 0xb8)) {
      *(unsigned long *)(a2 + 0xb8) = 1;
      return;
    }
    sub_fb20(a2,0x106);
    *(long *)(a2 + 0xb8) = *(long *)(a2 + 0xb8) + 1;
    return;
  }
  sub_6a10(dcgettext(NULL,"unbalanced )",5)); // no-return
}

// Function: sub_14e00 @ 0x14e00
void sub_14e00(long a0)
{
  (**(void **)(a0 + 0x410))(); // jump-as-call
}

// Function: sub_14e10 @ 0x14e10
unsigned long sub_14e10(long a0)
{
  return *(unsigned long *)(a0 + 0x120);
}

// Function: sub_14e20 @ 0x14e20
char sub_14e20(long a0)
{
  return *(char *)(a0 + 0xc0);
}

// Function: sub_14e30 @ 0x14e30
unsigned long sub_14e30(long a0) // return-dupe
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

// Function: sub_14e90 @ 0x14e90
void sub_14e90(unsigned long *a0)
{
  long v1; // rbp
  long v2;
  void *v3;
  
  free((void *)*a0);
  free((void *)a0[0x12]);
  if (*(char *)&a0[0x83])
    sub_105e0(a0);
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
    sub_14e90(a0[0x24]);
    free((void *)a0[0x24]); // tail-call
    return;
  }
}

// Function: sub_15020 @ 0x15020
char * sub_15020(long *a0)
{
  char v1;
  unsigned long v10;
  unsigned long v11;
  unsigned long v12;
  long *v13; // rax
  char *v14; // rax
  unsigned long v15;
  unsigned long *v16;
  void *v17;
  long v18;
  unsigned long v19; // rcx
  unsigned long *v2;
  char *v20;
  char v21;
  char v22;
  long *v23;
  unsigned long v24;
  long *v25;
  unsigned long *v26;
  long v27;
  long v28;
  unsigned int v29; // r12d
  bool v3;
  int v30;
  long v31;
  char *v32;
  char v33;
  long v34;
  long v35;
  unsigned long *v36; // stack - 0x80
  long v37; // stack - 0x68
  bool v4;
  unsigned char v5; // al
  unsigned char v6; // al
  char v7;
  int v8;
  unsigned long v9;
  
  v18 = a0[0x13];
  v5 = (*(unsigned char *)&a0[0x83] ^ 1) & *(unsigned char *)((long)a0 + 0x1cd);
  v36 = NULL;
  if (v18 <= 2)
    return NULL;
  v4 = 0;
  v3 = 0;
  v37 = 1;
  do {
    v35 = a0[0x12];
    v24 = *(unsigned long *)(v35 + v37 * 8);
    v27 = v37 * 8;
    if (0x113 < (long)v24) {
      v31 = *a0 + -0x2280 + v24 * 0x20;
      v9 = 0;
      do {
        v24 = (unsigned long)(int)v9;
        v29 = (int)v9 + 1;
        if (*(unsigned long *)(v31 + (v9 >> 6) * 8) >> (v24 & 0x3f) & 1) {
          if (v29 == 0x100) goto label_155bb;
          v10 = (unsigned long)v29;
          v34 = v24 * 4;
          goto label_15553;
        }
        v9 = (unsigned long)v29;
      } while (v29 != 0x100);
label_150d1:
      v36 = (unsigned long *)sub_10090(v36,2);
      v31 = v37 + 2;
      goto label_150ee;
    }
    if (0x100 > (long)v24) {
      if (v24 == 0xffffffffffffffff) {
        if (v36[5])
          __assert_fail("!mp->prev","dfa.c",0x107d,"dfamust"); // no-return
        v23 = (long *)*v36;
        v32 = "";
        v20 = (char *)*v23;
        if (v20) goto label_1583f;
        v20 = (char *)v36[3];
        v26 = v36;
        if (strcmp("",v20)) goto label_15130;
        v21 = '\0';
        goto label_158dd;
      }
      if (v24) {
label_155bb:
        v28 = 2;
        v31 = v37 + 2;
        v34 = v31;
        if ((*(long *)(v35 + 8 + v27) == 0x105) && (v31 < v18 + -1)) {
          while ((v34 == v37 || ((unsigned long)(*(long *)(v35 + v34 * 8) - 1U) <= 0xfe))) {
            if ((*(long *)(v35 + 8 + v34 * 8) != 0x105) || (v34 = v34 + 2, v18 + -1 <= v34)) break;
          }
          v28 = (v34 - v37 >> 1) + 1;
        }
        v36 = (unsigned long *)sub_10090(v36,v28);
        v7 = (char)v24;
        if ((v5) && ((int)v24 + 0x80U <= 0x17f))
          v7 = (char)*(unsigned int *)(*(long *)__ctype_toupper_loc() + (long)(int)v24 * 4);
        *(char *)v36[2] = v7;
        *(char *)v36[1] = v7;
        *(char *)v36[3] = v7;
        if (v34 <= v31) { // branch-flip
          v18 = 1;
          v27 = 1;
        }
        else {
          v35 = 1;
          v24 = (unsigned long)((v34 - v37) - 3U) >> 1;
          v18 = v24 + 2;
          do {
            v27 += 0x10;
            v7 = (char)*(unsigned long *)(a0[0x12] + v27);
            if ((v5) && (v30 = (int)*(unsigned long *)(a0[0x12] + v27), v30 + 0x80U <= 0x17f))
              v7 = (char)*(unsigned int *)(*(long *)__ctype_toupper_loc() + (long)v30 * 4);
            *(char *)(v36[2] + v35) = v7;
            *(char *)(v36[1] + v35) = v7;
            *(char *)(v36[3] + v35) = v7;
            v35 += 1;
          } while (v18 != v35);
          v34 = v24 * 2;
          v35 = v37 + 4;
          v37 = v31 + v34;
          v27 = v18;
          v31 = v35 + v34;
        }
        *(char *)(v36[2] + v18) = 0;
        *(char *)(v36[1] + v18) = 0;
        *(char *)(v36[3] + v18) = 0;
        v12 = *v36;
        v15 = sub_1f3b0(v36[3],v27);
        *v36 = sub_10100(v12,v15);
        goto label_150ee;
      }
      break;
    }
    switch(v24) {
      case 0x100:
      case 0x10a:
      case 0x10e:
      case 0x10f:
      case 0x110:
      case 0x111:
      case 0x112:
      case 0x113:
        goto label_150d1;
      case 0x101:
      case 0x102:
        v23 = (long *)*v36;
        v17 = (void *)*v23;
        if (v17) {
          do {
            free(v17);
            v17 = (void *)v23[1];
            v23 = &v23[1];
          } while (v17);
          v23 = (long *)*v36;
        }
        *v23 = 0;
        *(char *)v36[3] = 0;
        *(char *)v36[2] = 0;
        *(char *)v36[1] = 0;
        *(unsigned short *)&v36[4] = 0;
        v31 = v37 + 2;
        break;
      case 0x103:
        *(char *)v36[3] = 0;
        v31 = v37 + 2;
        break;
      default:
        goto label_155bb;
      case 0x105:
        v23 = (long *)*v36;
        v26 = (unsigned long *)v36[5];
        v12 = *v26;
        v18 = *v23;
        while (v18) {
          v23 = &v23[1];
          v12 = sub_10100(v12,sub_1f3f0(v18));
          v18 = *v23;
        }
        *v26 = v12;
        if ((*(char *)v26[2]) && (v20 = (char *)v36[1], *v20)) {
          v24 = strlen((char *)v26[2]);
          v9 = strlen(v20);
          v17 = (void *)sub_1ef30(v24 + 1 + v9);
          memcpy((void *)((long)v17 + v24),(void *)v36[1],v9 + 1);
          memcpy(v17,(void *)v26[2],v24);
          v12 = *v26;
          *v26 = sub_10100(v12,v17);
        }
        if (*(char *)v26[3]) {
          v12 = v26[1];
          v15 = v36[1];
          v26[1] = sub_10230(v12,v15);
        }
        if (!*(char *)v36[3])
          *(char *)v26[2] = 0;
        v12 = sub_10230(v26[2],v36[2]);
        v20 = (char *)v26[3];
        v26[2] = v12;
        if (((*v20) || (*(char *)&v26[4])) && ((v32 = (char *)v36[3], *v32 || (*(char *)((long)v36 + 0x21))))) {
          v26[3] = sub_10230(v20,v32);
          v7 = *(char *)((long)v36 + 0x21);
        }
        else {
          *v20 = '\0';
          v7 = 0;
          *(char *)&v26[4] = 0;
        }
        *(char *)((long)v26 + 0x21) = v7;
        goto label_157f5;
      case 0x106:
        v26 = (unsigned long *)v36[5];
        v20 = (char *)v36[3];
        v32 = (char *)v26[3];
        if (strcmp(v32,v20)) { // branch-flip
          *v32 = '\0';
          *(char *)&v26[4] = 0;
          v6 = 0;
        }
        else {
          *(unsigned char *)&v26[4] = *(unsigned char *)&v26[4] & *(unsigned char *)&v36[4];
          v6 = *(unsigned char *)((long)v26 + 0x21) & *(unsigned char *)((long)v36 + 0x21);
        }
        *(unsigned char *)((long)v26 + 0x21) = v6;
        v20 = (char *)v26[1];
        v21 = *v20;
        if (v21) {
          v32 = (char *)v36[1];
          do {
            if (*v32 != v21) break;
            v21 = v20[1];
            v20 = &v20[1];
            v32 = &v32[1];
          } while (v21);
        }
        *v20 = '\0';
        v20 = (char *)v26[2];
        v9 = strlen(v20);
        v32 = (char *)v36[2];
        v10 = strlen(v32);
        v24 = v9;
        if ((long)v10 <= (long)v9)
          v24 = v10;
        if (1 <= (long)v24) {
          v19 = 0;
          v11 = v9;
          do {
            if (v20[v11 - 1] != v32[(v11 - 1) + (v10 - v9)]) {
              if (!v19) goto label_1532f;
              goto label_15301;
            }
            v19 += 1;
            v11 -= 1;
          } while (v24 != v19);
          v11 = v9 - v19;
label_15301:
          v18 = 0;
          while( true ) {
            v20[v18] = v20[v11 + v18];
            v18 += 1;
            if ((long)v19 <= v18) break;
            v20 = (char *)v26[2];
          }
          v20 = (char *)(v19 + v26[2]);
        }
label_1532f:
        *v20 = '\0';
        v16 = (unsigned long *)*v36;
        v23 = (long *)*v26;
        v12 = sub_1f310(8);
        v18 = *v23;
        while (v18) {
          v20 = (char *)*v16;
          v2 = v16;
          while (v2 = &v2[1], v20) {
            v32 = (char *)*v23;
            v13 = (long *)sub_1f310(8);
            v21 = *v32;
            if (v21) {
              do {
                v14 = strchr(v20,(int)v21);
                if (v14) {
                  v33 = v32[1];
                  v18 = 0;
                  do {
                    v27 = 1;
                    v1 = v33;
                    while ((v1 && (v14[v27] == v1))) {
                      v27 += 1;
                      v1 = v32[v27];
                    }
                    if (v18 < v27)
                      v18 = v27;
                    v14 = strchr(&v14[1],(int)v21);
                  } while (v14);
                  v13 = (long *)sub_10100(v13,sub_1f3b0(v32,v18));
                }
                v21 = v32[1];
                v32 = &v32[1];
              } while (v21);
            }
            v18 = *v13;
            if (v18) {
              v25 = v13;
              do {
                v25 = &v25[1];
                v12 = sub_10100(v12,sub_1f3f0(v18));
                v18 = *v25;
              } while (v18);
              v17 = (void *)*v13;
              if (v17) {
                v25 = v13;
                do {
                  free(v17);
                  v17 = (void *)v25[1];
                  v25 = &v25[1];
                } while (v17);
              }
            }
            free(v13);
            v20 = (char *)*v2;
          }
          v23 = &v23[1];
          v18 = *v23;
        }
        v23 = (long *)*v26;
        v17 = (void *)*v23;
        if (v17) {
          do {
            free(v17);
            v17 = (void *)v23[1];
            v23 = &v23[1];
          } while (v17);
          v23 = (long *)*v26;
        }
        free(v23);
        *v26 = v12;
label_157f5:
        sub_10960(v36);
        v31 = v37 + 2;
        v36 = v26;
        break;
      case 0x107:
      case 0x108:
        __assert_fail("!\"neither LPAREN nor RPAREN may appear here\"","dfa.c",0x1035,"dfamust"); // no-return
      case 0x10c:
        v36 = (unsigned long *)sub_10090(v36,2);
        v3 = 1;
        *(char *)&v36[4] = 1;
        v31 = v37 + 2;
        break;
      case 0x10d:
        v36 = (unsigned long *)sub_10090(v36,2);
        v4 = 1;
        *(char *)((long)v36 + 0x21) = 1;
        v31 = v37 + 2;
      
    }
label_150ee:
    v37 += 1;
    v18 = a0[0x13];
  } while (v18 > v31);
  v26 = v36;
  if (!v36)
    return NULL;
label_15130:
  v36 = NULL;
  v16 = v36;
label_15139:
  do {
    v36 = v16;
    v2 = (unsigned long *)v26[5];
    sub_10960(v26);
    v26 = v2;
    v16 = v36;
  } while (v2);
  return (char *)v36;
label_15553:
  v11 = v10;
  v34 += 4;
  v30 = (int)v11;
  if (*(unsigned long *)(v31 + (v11 >> 6) * 8) >> (v11 & 0x3f) & 1) {
    if (!v5) goto label_150d1;
    v28 = *(long *)__ctype_toupper_loc();
    v8 = v30;
    if ((int)v9 + 0x81U <= 0x17f)
      v8 = *(int *)(v28 + v34);
    if (*(int *)(v28 + v24 * 4) != v8) goto label_150d1;
  }
  if (v30 == 0xff) goto label_155bb;
  v10 = (unsigned long)(v30 + 1);
  v9 = v11;
  goto label_15553;
label_1583f:
  do {
    v23 = &v23[1];
    v24 = strlen(v20);
    if (strlen(v32) < v24)
      v32 = v20;
    v20 = (char *)*v23;
  } while (v20);
  v20 = (char *)v36[3];
  v21 = *v32;
  if (strcmp(v32,v20)) { // branch-flip
    v7 = 0;
    v22 = 0;
    v33 = '\0';
  }
  else {
label_158dd:
    v33 = *(char *)&v36[4];
    v7 = *(char *)((long)v36 + 0x21);
    if (((!v3) || (v22 = 0, v33)) && (v22 = 1, v4))
      v22 = v7;
  }
  v16 = NULL;
  if (v21) {
    v16 = (char *)sub_1ef10(strlen(v32) + 4);
    *(char *)v16 = v22;
    *(char *)((long)v16 + 1) = v33;
    *(char *)((long)v16 + 2) = v7;
    strcpy((char *)((long)v16 + 3),v32);
  }
  v26 = v36;
  goto label_15139;
}

// Function: sub_15a80 @ 0x15a80
void sub_15a80(void *a0)
{
  free(a0); // tail-call
}

// Function: sub_15a90 @ 0x15a90
void sub_15a90(void)
{
  sub_1ef10(0x920); // tail-call
}

// Function: sub_15aa0 @ 0x15aa0
void sub_15aa0(long a0,unsigned long a1,unsigned long *a2,char a3)
{
  unsigned long *v1;
  long v10;
  long v11;
  long v12; // rbx
  unsigned long v13; // stack - 0x68
  unsigned long *v14;
  unsigned long *v15;
  char v16;
  long v17;
  long v18;
  unsigned long v19; // stack - 0x60
  unsigned long *v2;
  unsigned long v20; // stack - 0x58
  unsigned long v21; // stack - 0x50
  unsigned long v3;
  unsigned long v4;
  bool v5;
  bool v6;
  unsigned long *v7; // rax
  void *v8; // rax
  long v9; // rax
  
  if (a0)
    sub_14cf0(a0,a1,a2);
  v7 = (unsigned long *)sub_15a90();
  v11 = 0x124;
  v14 = a2;
  v15 = v7;
  while (v11) {
    v2 = &v15[1];
    v1 = &v14[1];
    *v15 = *v14;
    v11 -= 1;
    v14 = v1;
    v15 = v2;
  }
  *(char *)&v7[0x83] = 0;
  v7[0x82] = sub_14cc0;
  v7[0x23] = 0;
  v7[0x24] = 0;
  v7[0x25] = 0;
  v7[0x26] = 0;
  v7[0x2a] = 0;
  v7[0x2b] = 0;
  v7[0x28] = 0;
  v7[0x2c] = 0;
  v7[0x2e] = 0;
  v7[0x2f] = 0;
  v7[0x30] = 0;
  v7[0x31] = 0;
  v8 = (void *)sub_1f050(v7[2],0x20);
  v11 = a2[1];
  *v7 = v8;
  if (v11)
    memcpy(v8,(void *)*a2,v11 << 5);
  v3 = a2[0x13];
  v7[0x12] = sub_1f050(v3,0x10);
  v11 = a2[0x13];
  v7[0x14] = v11 * 2;
  if (1 <= v11) { // branch-flip
    v6 = 0;
    v5 = 0;
    v11 = a2[0x12];
    v12 = 0;
    v17 = 0;
    do {
      v4 = *(unsigned long *)(v11 + v12 * 8);
      v18 = v17 + 1;
      v10 = v17 * 8;
      if (0x112 <= (long)v4) { // branch-flip
        if (v4 - 0x112 <= 1) goto label_15c6b;
label_15bf8:
        *(unsigned long *)(v7[0x12] + v10) = v4;
        if (v4 <= 0xff || 0x113 < (long)v4)
          v6 = v4 <= 0xff || 0x113 < (long)v4;
      }
      else if (0x10e <= (long)v4) { // branch-flip
        if (*(char *)&a2[0x83]) // branch-flip
          *(unsigned long *)(v7[0x12] + v10) = 0x100;
        else {
          *(unsigned long *)(v7[0x12] + v10) = v4;
        }
      }
      else {
        if (v4 != 0x10a) goto label_15bf8;
label_15c6b:
        v18 = v17 + 2;
        v13 = 0xffffffffffffffff;
        v19 = 0xffffffffffffffff;
        v20 = 0xffffffffffffffff;
        v21 = 0xffffffffffffffff;
        v9 = sub_10890(v7,&v13);
        v5 = 1;
        v17 = v7[0x12];
        v11 = a2[0x12];
        *(long *)(v17 + v10) = v9 + 0x114;
        *(unsigned long *)(v17 + 8 + v10) = 0x102;
        v12 += (unsigned long)((unsigned long)(*(long *)(v11 + 8 + v12 * 8) - 0x101U) < 3);
      }
      v10 = a2[0x13];
      v12 += 1;
      v17 = v18;
    } while (v12 < v10);
    v7[0x13] = v18;
    if ((!v6) || ((!v5 && (!*(char *)&a2[0x83])))) goto label_15eaf;
    a2[0x24] = v7;
  }
  else {
    v7[0x13] = 0;
label_15eaf:
    sub_14e90(v7);
    free(v7);
    v10 = a2[0x13];
  }
  if (1 <= v10) { // branch-flip
    v11 = 0;
    do {
      v17 = *(long *)(a2[0x12] + v11 * 8);
      if (0x112 <= v17) { // branch-flip
        if ((unsigned long)(v17 - 0x112U) <= 1) goto label_15e31;
      }
      else if ((0x10d < v17) && (*(char *)&a2[0x83])) {
label_15e31:
        v11 = a2[0x24];
        a2[0x82] = sub_f950;
        goto label_15e4b;
      }
      v11 += 1;
    } while (v11 != v10);
    if (!*(char *)((long)a2 + 0x41a)) goto label_15dde;
    v16 = '\0';
    v11 = 0;
    do {
      v17 = *(long *)(a2[0x12] + v11 * 8);
      if (v17 != 0x112) { // branch-flip
        if (v17 == 0x113) goto label_15dde;
        if (v17 == 0x10a)
          abort(); // no-return
      }
      else {
        v16 = *(char *)((long)a2 + 0x41a);
      }
      v11 += 1;
    } while (v11 != v10);
    if (!v16) goto label_15f5c;
  }
  else {
    if (!*(char *)((long)a2 + 0x41a)) goto label_15dde;
label_15f5c:
    if (a2[0x24]) {
      sub_14e90(a2[0x24]);
      free((void *)a2[0x24]);
      a2[0x24] = 0;
    }
  }
  sub_105e0(a2);
  *(char *)&a2[0x83] = 0;
  a2[0x82] = sub_14cc0;
  *(char *)&a2[0x18] = 1;
label_15dde:
  sub_11480(a2,a3);
  v11 = a2[0x24];
label_15e4b:
  if (!v11)
    return;
  *(char *)&a2[0x18] = 1;
  sub_11480(v11,a3); // tail-call
}

// Function: sub_15fa0 @ 0x15fa0
void sub_15fa0(unsigned long *a0,unsigned char *a1,unsigned long a2,unsigned int a3)
{
  unsigned char *v1;
  unsigned long *v10;
  unsigned int v11; // r14d
  bool v12; // al
  unsigned long *v2;
  char v3;
  unsigned char v4;
  unsigned long v5; // rcx
  void *v6; // rcx
  long v7; // rcx
  unsigned char v8; // bl
  unsigned int v9; // ebx
  
  v9 = 0xffffff80;
  *a0 = 0;
  a0[0x81] = 0;
  v5 = (unsigned long)(((int)a0 - (int)(unsigned long *)((unsigned long)&a0[1] & 0xfffffffffffffff8)) + 0x410U >> 3);
  v10 = (unsigned long *)((unsigned long)&a0[1] & 0xfffffffffffffff8);
  while (v5) {
    v2 = &v10[1];
    *v10 = 0;
    v5 -= 1;
    v10 = v2;
  }
  v6 = sub_14cd0;
  v4 = *a1;
  if (!v4)
    v6 = sub_14cc0;
  a0[0x82] = v6;
  v7 = 0xa0;
  v10 = &a0[0x83];
  while (v7) {
    v2 = &v10[1];
    v1 = &a1[8];
    *v10 = *(unsigned long *)a1;
    v7 -= 1;
    a1 = v1;
    v10 = v2;
  }
  v3 = *(char *)((long)a0 + 0x41a);
  *(unsigned int *)v10 = *(unsigned int *)a1;
  *(unsigned char *)&a0[0x18] = v4 ^ 1;
  *(unsigned int *)&a0[0x39] = a3;
  a0[3] = 0xffffffffffffffff;
  *(unsigned char *)((long)a0 + 0x1cd) = (unsigned char)(a2 >> 0x16) & 1;
  *(char *)((long)a0 + 0x1cc) = 1;
  a0[0x38] = a2;
  v4 = -((a3 & 2) == 0) & 10;
  *(unsigned char *)((long)a0 + 0x1ce) = v4;
  do {
    v8 = (unsigned char)v9;
    v11 = v9 & 0xff;
    v7 = (long)(int)v11;
    if ((v4 != v8) || (a3 & 1)) {
      if (*(int *)((long)a0 + v7 * 4 + 0x51c) != -1) {
        if ((*(unsigned char *)(*(long *)__ctype_b_loc() + ((unsigned long)v9 & 0xff) * 2) & 8) || (v8 == 0x5f)) {
          *(char *)((long)a0 + v7 + 0x1cf) = 2;
          a0[(unsigned long)(v11 >> 6) + 0x7a] = a0[(unsigned long)(v11 >> 6) + 0x7a] | 1L << (v8 & 0x3f);
          goto label_16109;
        }
      }
      *(char *)((long)a0 + v7 + 0x1cf) = 1;
    }
    else {
      *(char *)((long)a0 + v7 + 0x1cf) = 4;
      a0[(unsigned long)(v11 >> 6) + 0x7e] = a0[(unsigned long)(v11 >> 6) + 0x7e] | 1L << (v8 & 0x3f);
    }
label_16109:
    if (v3) // branch-flip
      v12 = (v9 & 0xc0) != 0x80;
    else {
      v12 = strchr("\n\r./",v11) != NULL;
    }
    v9 += 1;
    *(bool *)((long)a0 + v7 + 0x2cf) = v12;
    if (v9 == 0x80)
      return;
  } while( true );
}

// Function: sub_16180 @ 0x16180
void sub_16180(unsigned long *a0,long a1)
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

// Function: sub_16210 @ 0x16210
void sub_16210(long a0)
{
  char v1; // al
  int v10;
  unsigned int v11; // r15d
  char v12; // stack - 0x41
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rcx
  unsigned long v5;
  char v6 [3]; // stack - 0x44
  unsigned long v7; // stack - 0x4c
  int v8; // stack - 0x50
  char v9; // stack - 0x51
  
  v2 = __ctype_get_mb_cur_max();
  v1 = 0;
  *(bool *)a0 = 2 <= v2;
  if (v2 <= 1) {
    v10 = 0;
    do {
      v6[2] = (char)v10;
      v6[0] = v6[2] + '\x01';
      v6[1] = 0;
      v12 = 0;
      if (0 <= strcoll(&v6[2],v6)) {
        v1 = 0;
        goto label_16296;
      }
      v10 += 1;
    } while (v10 != 0xff);
    v1 = 1;
  }
label_16296:
  *(char *)(a0 + 1) = v1;
  v7 = 0;
  *(bool *)(a0 + 2) = sub_1acc0(&v8,0x23075,2,&v7) == 2 && v8 == 0x100;
  v2 = 0xffffff80;
  do {
    v9 = (char)v2;
    v7 = 0;
    v3 = sub_1acc0(&v8,&v9,1,&v7);
    v4 = v2 & 0xff;
    v5 = 1;
    if (v3)
      v5 = v3;
    v10 = -1;
    if (v3 <= 1)
      v10 = v8;
    v11 = (int)v2 + 1;
    v2 = (unsigned long)v11;
    *(char *)(a0 + 3 + v4) = (char)v5;
    *(int *)(a0 + 0x104 + v4 * 4) = v10;
  } while (v11 != 0x80);
}

// Function: sub_16380 @ 0x16380
unsigned int sub_16380(unsigned long a0,unsigned int *a1)
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
  v4 = (short *)0x23082;
  v3 = 0xb5;
  while( true ) {
    if (((v2 != v3 && v1 != v3) && (v5 != v3)) && (towupper(v3) == v1)) {
      a1[(int)v6] = v3;
      v6 += 1;
    }
    if (v4 == (short *)"mbuiter.h") break;
    v3 = (unsigned int)*v4;
    v4 = &v4[1];
  }
  return v6;
}

// Function: sub_16480 @ 0x16480
bool sub_16480(char *a0)
{
  char v1;
  char *v2;
  
  v2 = (char *)sub_1fad0(a0);
  if (!*v2)
    v2 = a0;
  v1 = v2[sub_1fb30(v2)];
  v2[sub_1fb30(v2)] = '\0';
  return v1 != '\0';
}

// Function: sub_164c0 @ 0x164c0
void sub_164c0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long *a3)
{
  (*(void *)*a3)(); // jump-as-call
}

// Function: sub_164d0 @ 0x164d0
void sub_164d0(void *a0)
{
  free(a0); // tail-call
}

// Function: sub_164e0 @ 0x164e0
unsigned int sub_164e0(void)
{
  int v1; // eax
  
  v1 = sub_1aec0();
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}

// Function: sub_16500 @ 0x16500
unsigned int sub_16500(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = strcmp(a0,a1);
  return CONCAT31((undefined3)((unsigned int)v1 >> 8),v1 == 0);
}

// Function: sub_16520 @ 0x16520
void sub_16520(void)
{
  sub_1a260(); // tail-call
}

// Function: sub_16530 @ 0x16530
unsigned long sub_16530(char *a0,char *a1,unsigned int a2) // early-return
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
    return sub_1aec0(a0,a1); // tail-call
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
  v5 = (char *)sub_1f3f0(a1);
  v6 = v5;
  while (v6 = strchr(v6,0x2f), v6) {
    *v6 = '\0';
    v2 = sub_1aec0(a0,v5);
    if ((int)v2 <= 0) {
      free(v5); // return-dupe
      return (unsigned long)v2;
    }
    *v6 = '/';
    v6 = &v6[1];
  }
  v2 = sub_1aec0(a0,v5);
  free(v5);
  return (unsigned long)v2;
}

// Function: sub_16620 @ 0x16620
unsigned long sub_16620(unsigned char *a0,unsigned long a1)
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
label_166f3:
      v3 = __ctype_get_mb_cur_max();
      v7 = v10;
      v4 = sub_1e760(v10,v3);
      v11 = sub_1acc0(v6,v7,v4,&v5);
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
          if (!mbsinit(&v5)) goto label_166a4;
          v1 = 0;
          if (v2) goto label_166ad;
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
      if (!(*(unsigned int *)((unsigned long)(*v10 >> 5) * 4 + 0x23200) >> (*v10 & 0x1f) & 1)) {
        if (!mbsinit(&v5))
          __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
        v1 = 1;
        goto label_166f3;
      }
      v11 = 1;
      v2 = (unsigned int)(char)*v10;
      v12 = '\x01';
      v6[0] = v2;
label_166a4:
      if (!v2)
        return v8 % a1;
label_166ad:
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

// Function: sub_168d0 @ 0x168d0
void sub_168d0(long a0,unsigned long a1)
{
  unsigned long v1;
  unsigned long *v2; // rax
  
  v2 = (unsigned long *)sub_1ef10(0x10);
  v1 = *(unsigned long *)(a0 + 8);
  v2[1] = a1;
  *v2 = v1;
  *(unsigned long **)(a0 + 8) = v2;
}

// Function: sub_16900 @ 0x16900
unsigned long sub_16900(char *a0,unsigned int a1) // return-dupe x2
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
          if (!(v1 + 0x85U & 0xfd)) goto label_16990;
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
label_16990:
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

// Function: sub_169d0 @ 0x169d0
void sub_169d0(void)
{
  sub_1f310(0x10); // tail-call
}

// Function: sub_169e0 @ 0x169e0
void sub_169e0(unsigned long *a0)
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
              if (v7[4] <= v5) goto label_16a61;
            }
            v5 += 1;
            regfree(&v1[2]);
            v6 = (void *)v7[2];
          } while (v5 < v7[4]);
        }
label_16a61:
        free(v6);
      }
      free(v7);
      v7 = v2;
      if (!v2) goto label_16a76;
    }
    sub_1a490(v7[2]);
    free(v7);
  }
label_16a76:
  v4 = (unsigned long *)a0[1];
  while (v4) {
    v3 = (unsigned long *)*v4;
    free((void *)v4[1]);
    free(v4);
    v4 = v3;
  }
  free(a0); // tail-call
}

// Function: sub_16ad0 @ 0x16ad0
unsigned long sub_16ad0(unsigned long a0,char *a1,unsigned int a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // rax
  char v2;
  char v3;
  char *v4;
  unsigned long v5; // r8
  char *v6;
  void *v7;
  
  v7 = sub_16530;
  if (a2 & 0x10000000)
    v7 = dat_2afd0;
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

// Function: sub_16b90 @ 0x16b90
unsigned int sub_16b90(long *a0,char *a1)
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
            if (sub_16ad0(v4,a1)) {
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
        v8 = (char *)sub_1ef10(strlen(a1) + 1);
      v9 = *(unsigned int *)((long)a0 + 0xc);
      v2 = a0[2];
      v7 = a1;
      while( true ) {
        strcpy(v8,v7);
        v5 = sub_1a040(v2,v8);
        while( true ) {
          if (v5) {
            v9 = 0;
            free(v8);
            return (*(unsigned int *)((long)a0 + 0xc) >> 0x1d ^ 1) & 1 ^ v9;
          }
          if ((!(v9 & 8)) || (v6 = strrchr(v8,0x2f), !v6)) break;
          *v6 = '\0';
          v5 = sub_1a040(v2,v8);
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

// Function: sub_16d40 @ 0x16d40
void sub_16d40(long *a0,char *a1,unsigned int a2) // return-dupe x2
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
  if ((!(a2 & 0x18000000)) || (!sub_16900(a1,a2))) {
    if (((!v4) || (*(int *)&v4[1])) || ((*(unsigned int *)((long)v4 + 0xc) ^ a2) & 0x60000018)) {
      v4 = (long *)sub_1f310(0x28);
      v10 = sub_164e0;
      v13 = sub_16620;
      *(unsigned int *)&v4[1] = 0;
      *(unsigned int *)((long)v4 + 0xc) = a2;
      if (!(a2 & 0x10)) {
        v10 = sub_16500;
        v13 = sub_16520;
      }
      v4[2] = sub_1a2d0(0,0,v13,v10,sub_164d0);
      v17 = *a0;
      *a0 = (long)v4;
      *v4 = v17;
    }
    v7 = (char *)sub_1f3f0(a1);
    if ((a2 & 0x10000002) != 0x10000000) {
label_16f1b:
      v17 = v4[2];
      if (v7 == (char *)sub_1a930(v17,v7))
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
        if (!v12) goto label_16f1b;
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
    v4 = (long *)sub_1f310(0x28);
    *(unsigned int *)&v4[1] = 1;
    *(unsigned int *)((long)v4 + 0xc) = a2;
    v17 = *a0;
    *a0 = (long)v4;
    *v4 = v17;
  }
  v17 = v4[4];
  v8 = v4[2];
  if (v17 == v4[3]) {
    v8 = sub_1f1f0(v8,&v4[3],1,0xffffffffffffffff,0x48);
    v17 = v4[4];
    v4[2] = v8;
  }
  v4[4] = v17 + 1;
  v1 = (unsigned int *)(v8 + v17 * 0x48);
  *v1 = a2;
  if (!(a2 & 0x8000000)) {
    if (a2 & 0x4000000) {
      a1 = (char *)sub_1f3f0(a1);
      v4 = (long *)sub_1ef10(0x10);
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
        if (a1[v5 - 1] != '/') goto label_16e21;
        v5 -= 1;
      } while (v5);
      goto label_16e76;
    }
    v15 = v5;
    if (!v5) goto label_16e76;
label_16e21:
    v6 = (void *)sub_1ef30(v15 + 7);
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
label_16e76:
  v4[4] = v17;
  return;
}

// Function: sub_17080 @ 0x17080
int sub_17080(void *a0,long a1,void *a2,unsigned int a3,unsigned char a4,unsigned long a5)
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
      v3 = sub_1f1f0(v3,&v6,1,0xffffffffffffffff,1);
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
  v4 = (unsigned char *)sub_1efa0(v3,v9 + 1);
  v4[v9] = a4;
  if (v9) { // branch-flip
    v7 = &v4[(unsigned long)(v4[v9 + -1] != a4) + v9];
    v5 = (unsigned long *)sub_1ef10(0x10);
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
              if (!(*(unsigned char *)(v3 + 1 + (unsigned long)v10[-1] * 2) & 0x20)) goto label_1723d;
            }
          }
          else {
label_1723d:
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
    v5 = (unsigned long *)sub_1ef10(0x10);
    v1 = *(unsigned long *)(a1 + 8);
    v5[1] = v4;
    *v5 = v1;
    *(unsigned long **)(a1 + 8) = v5;
  }
  *v12 = v11;
  return v13;
}

// Function: sub_17290 @ 0x17290
unsigned int sub_17290(unsigned long a0,unsigned long a1,char *a2,unsigned int a3,char a4) // return-dupe
{
  int v1;
  unsigned int v2; // eax
  void *v3; // rax
  int *v4; // rax
  unsigned long v5 [2]; // stack - 0x30
  
  v5[0] = a0;
  if (!strcmp(a2,"-")) {
    v2 = sub_17080(sub_164c0,a1,stdin,a3,(int)a4,v5);
    return v2;
  }
  v3 = fopen(a2,"re");
  if (!v3)
    return 0xffffffff;
  v2 = sub_17080(sub_164c0,a1,v3,a3,(int)a4,v5);
  v4 = __errno_location();
  v1 = *v4;
  if (fclose(v3))
    return 0xffffffff;
  *v4 = v1;
  return v2;
}

// Function: sub_17370 @ 0x17370
unsigned long sub_17370(long *a0,long *a1)
{
  if (a0[1] != a1[1])
    return 0;
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_173a0 @ 0x173a0
unsigned long sub_173a0(long a0,unsigned long a1)
{
  return *(unsigned long *)(a0 + 8) % a1;
}

// Function: sub_173c0 @ 0x173c0
unsigned long sub_173c0(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_173d0 @ 0x173d0
unsigned long sub_173d0(long *a0,long *a1)
{
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_173e0 @ 0x173e0
int sub_173e0(long *a0,long *a1)
{
  unsigned long v1;
  unsigned long v2;
  
  v1 = *(unsigned long *)(*a1 + 0x78);
  v2 = *(unsigned long *)(*a0 + 0x78);
  return (unsigned int)(v2 >= v1 && v2 != v1) - (unsigned int)(v2 < v1);
}

// Function: sub_17400 @ 0x17400
int sub_17400(long a0,long a1,char a2) // early-return x3
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
      goto label_1749b;
    }
  }
  else {
    v3 = *(char **)(a1 + 0x30);
    v5 = *(int *)(a0 + 0x2c);
    if (fstatat(v5,v3,v1,0x100) <= -1) {
      v5 = *__errno_location();
label_1749b:
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

// Function: sub_175b0 @ 0x175b0
long sub_175b0(long a0,long a1,unsigned long a2)
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
label_1760b:
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
      goto label_1760b;
    }
    v4 = *(long **)(a0 + 0x10);
  }
  free(v4);
  *(unsigned long *)(a0 + 0x10) = 0;
  *(unsigned long *)(a0 + 0x38) = 0;
  return a1;
}

// Function: sub_176c0 @ 0x176c0
void * sub_176c0(long a0,void *a1,unsigned long a2)
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

// Function: sub_17760 @ 0x17760
unsigned long sub_17760(long a0,int a1)
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
    v2 = sub_1a2d0(0xd,0,sub_173c0,sub_173d0,dat_2afb8);
    *(long *)(v1 + 0x50) = v2;
    if (!v2) goto label_177ee;
  }
  v5[0] = *(unsigned long *)(a0 + 0x70);
  v1 = sub_1a040(v2,v5);
  if (v1)
    return *(unsigned long *)(v1 + 8);
label_177ee:
  if ((0 <= a1) && (!fstatfs(a1,v6))) {
    if ((v2) && (v3 = malloc(0x10), v3)) {
      *v3 = *(unsigned long *)(a0 + 0x70);
      v3[1] = v6[0];
      v4 = (unsigned long *)sub_1a930(v2,v3);
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

// Function: sub_17890 @ 0x17890
void sub_17890(long a0,int a1,char a2)
{
  int v1;
  
  v1 = *(int *)(a0 + 0x2c);
  if ((v1 == a1) && (v1 != -100))
    abort(); // no-return
  if (a2) { // branch-flip
    v1 = sub_1ab60(a0 + 0x60);
    if (0 <= v1) {
      close(v1);
      goto label_178d4;
    }
  }
  else if ((!(*(unsigned char *)(a0 + 0x48) & 4)) && (0 <= v1)) {
    close(v1);
label_178d4:
    *(int *)(a0 + 0x2c) = a1;
    return;
  }
  *(int *)(a0 + 0x2c) = a1;
}

// Function: sub_178f0 @ 0x178f0
unsigned long sub_178f0(long a0,long a1)
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

// Function: sub_17960 @ 0x17960
unsigned long sub_17960(long a0)
{
  long v1; // rax
  void *v2; // rax
  
  if (*(unsigned int *)(a0 + 0x48) & 0x102) {
    v1 = sub_1a2d0(0x1f,0,sub_173a0,sub_17370,dat_2afb8);
    *(long *)(a0 + 0x58) = v1;
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 != 0);
  }
  v2 = malloc(0x20);
  *(void **)(a0 + 0x58) = v2;
  if (v2) {
    sub_1fbd0(v2);
    return 1;
  }
  return 0;
}

// Function: sub_179d0 @ 0x179d0
unsigned long sub_179d0(long a0,unsigned long *a1) // return-dupe x2
{
  unsigned long v1;
  unsigned long *v2; // rax
  unsigned long *v3; // rax
  unsigned long v4;
  
  if (!(*(unsigned int *)(a0 + 0x48) & 0x102)) {
    v4 = sub_1fbf0(*(unsigned long *)(a0 + 0x58),&a1[0xe]);
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
  v3 = (unsigned long *)sub_1a930(v1,v2);
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

// Function: sub_17a80 @ 0x17a80
void sub_17a80(long a0,long a1)
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
    v4 = (void *)sub_1a990(*(unsigned long *)(a0 + 0x58),&v5);
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

// Function: sub_17b40 @ 0x17b40
bool sub_17b40(long a0)
{
  int v1;
  bool v2;
  
  if (*(unsigned int *)(a0 + 0x48) & 4) // branch-flip
    v2 = 0;
  else if (*(unsigned int *)(a0 + 0x48) & 0x200) { // branch-flip
    sub_17890(a0,0xffffff9c,1);
    v2 = 0;
  }
  else {
    v1 = *(int *)(a0 + 0x28);
    v2 = fchdir(v1) != 0;
  }
  while( true ) {
    if (sub_1ab50(a0 + 0x60)) break;
    v1 = sub_1aba0(a0 + 0x60);
    if (0 <= v1)
      close(v1);
  }
  return v2;
}

// Function: sub_17bd0 @ 0x17bd0
unsigned int sub_17bd0(long a0,long a1,int a2,char *a3) // return-dupe
{
  unsigned int v1;
  unsigned char v10; // stack - 0xdd
  long v11; // stack - 0xd0
  int v2;
  int v3;
  unsigned char v4; // al
  unsigned int v5; // eax
  int *v6; // rax
  unsigned int v7;
  long v8; // stack - 0xd8
  int v9;
  
  v9 = a2;
  if ((a3) && (v5 = strcmp(a3,".."), !v5)) {
    v7 = *(unsigned int *)(a0 + 0x48);
    if (v7 & 4) {
label_17d80:
      if ((v7 & 0x200) && (0 <= a2)) {
        close(a2);
        v5 = 0;
        return v5;
      }
      v5 = 0;
      return v5;
    }
    if (0 <= a2) { // branch-flip
      v10 = 1;
      if (!(v7 & 2)) goto label_17d5f;
    }
    else {
      if (!(v7 & 0x200)) {
        v10 = 1;
        v7 = (v7 & 0x10) << 0xd | 0x90900;
        goto label_17de4;
      }
      v4 = sub_1ab50(a0 + 0x60);
      if (v4) {
        v1 = *(unsigned int *)(a0 + 0x2c);
        v7 = (v7 & 0x10) << 0xd | 0x90900;
        v10 = v4;
        goto label_17c75;
      }
      v5 = sub_1aba0(a0 + 0x60);
      v7 = *(unsigned int *)(a0 + 0x48);
      if ((int)v5 < 0) {
        v10 = 1;
        v5 = v7;
        goto label_17e74;
      }
      v10 = 1;
      a2 = v5;
      v9 = v5;
      if (!(v7 & 2)) goto label_17ce0;
    }
label_17ca0:
    v5 = fstat(v9,&v8);
    if (!v5) {
      if ((*(long *)(a1 + 0x70) == v8) && (*(long *)(a1 + 0x78) == v11)) {
        v7 = *(unsigned int *)(a0 + 0x48);
        goto label_17ce0;
      }
      v6 = __errno_location();
      *v6 = 2;
    }
    v3 = -1;
  }
  else {
    v7 = *(unsigned int *)(a0 + 0x48);
    if (v7 & 4) goto label_17d80;
    if (0 <= a2) // branch-flip
      v10 = 0;
    else {
      v10 = 0;
      v5 = v7;
label_17e74:
      v5 &= 0x200;
      v1 = *(unsigned int *)(a0 + 0x2c);
      v7 = (v7 & 0x10) << 0xd | 0x90900;
      if (v5) { // branch-flip
label_17c75:
        v5 = sub_1b5d0(v1,a3,v7);
      }
      else {
label_17de4:
        v5 = sub_1fef0(a3,v7);
      }
      if ((int)v5 < 0) {
        v5 = -1;
        return v5;
      }
      v7 = *(unsigned int *)(a0 + 0x48);
      v9 = v5;
    }
    if (v7 & 2) goto label_17ca0;
    if (a3) {
label_17d5f:
      v5 = strcmp(a3,"..");
      if (!v5) goto label_17ca0;
    }
label_17ce0:
    if (v7 & 0x200) {
      sub_17890(a0,v9,v10 ^ 1);
      v5 = 0;
      return v5;
    }
    v5 = fchdir(v9);
    v3 = v5;
  }
  if (0 <= a2) {
    v5 = v3;
    return v5;
  }
  v6 = __errno_location();
  v2 = *v6;
  close(v9);
  *v6 = v2;
  v5 = v3;
  return v5;
}

// Function: sub_17ed0 @ 0x17ed0
void * sub_17ed0(long *a0,int a1)
{
  unsigned char v1;
  unsigned long v10; // rax
  void *v11;
  long v12;
  long v13;
  void *v14;
  unsigned int v15;
  unsigned long v16; // rdx
  int v17; // stack - 0x44
  char *v18;
  void *v19;
  long v2;
  unsigned long v20;
  unsigned long v21;
  bool v22;
  unsigned long v23; // stack - 0xa0
  void *v24; // stack - 0x98
  void *v25; // stack - 0x90
  char *v26; // stack - 0x70
  unsigned long v27; // stack - 0x68
  unsigned long v28; // stack - 0x60
  char v29; // stack - 0x4b
  bool v3;
  char v4; // al
  unsigned int v5; // eax
  unsigned int v6;
  int v7;
  int *v8; // rax
  unsigned long *v9; // rax
  
  v2 = *a0;
  v14 = *(void **)(v2 + 0x18);
  if (v14) { // branch-flip
    v17 = dirfd(v14);
    if (v17 <= -1) {
      closedir(*(void **)(v2 + 0x18));
      *(unsigned long *)(v2 + 0x18) = 0;
      if (a1 == 3) {
        *(unsigned short *)(v2 + 0x68) = 4;
        *(int *)(v2 + 0x40) = *__errno_location();
        return (void *)0;
      }
      return (void *)0;
    }
    if (a0[8]) { // branch-flip
      v27 = 0xffffffffffffffff;
label_17f32:
      v22 = 1;
      v15 = *(unsigned int *)&a0[9];
    }
    else {
      v27 = 100000;
      v15 = *(unsigned int *)&a0[9];
      v22 = 1;
    }
  }
  else {
    v15 = *(unsigned int *)&a0[9];
    v7 = 0;
    if ((v15 & 0x10) && (v7 = 0x20000, v15 & 1))
      v7 = (unsigned int)(*(long *)(v2 + 0x58) != 0) << 0x11;
    v6 = 0xffffff9c;
    if ((v15 & 0x204) == 0x200)
      v6 = *(unsigned int *)((long)a0 + 0x2c);
    v13 = sub_1b650(v6,*(unsigned long *)(v2 + 0x30),v7,&v17);
    *(long *)(v2 + 0x18) = v13;
    if (!v13) {
      if (a1 == 3) {
        *(unsigned short *)(v2 + 0x68) = 4;
        *(int *)(v2 + 0x40) = *__errno_location();
        return NULL;
      }
      return (void *)0;
    }
    if (*(short *)(v2 + 0x68) != 0xb) { // branch-flip
      if (*(unsigned char *)((long)a0 + 0x49) & 1) {
        sub_17a80(a0,v2);
        sub_17400(a0,v2,0);
        if (!sub_179d0(a0,v2)) {
          *__errno_location() = 0xc;
          return NULL;
        }
      }
    }
    else {
      *(unsigned short *)(v2 + 0x68) = sub_17400(a0,v2,0);
    }
    v27 = (unsigned long)(-(unsigned int)(a0[8] == 0) & 0x186a1) - 1;
    if (a1 == 2) {
label_18760:
      v22 = 0;
      v15 = *(unsigned int *)&a0[9];
      goto label_17f3b;
    }
    v7 = v17;
    if (((*(unsigned int *)&a0[9] & 0x38) != 0x18) || (*(long *)(v2 + 0x80) != 2)) {
label_18405:
      v22 = a1 == 3;
      v3 = 1;
    }
    else {
      v13 = sub_17760(v2,v17);
      v7 = v17;
      if (v13 == 0x9fa0) goto label_18405;
      if (v13 <= 0x9fa0) {
        if ((v13) && (v13 != 0x6969)) goto label_185a1;
        goto label_18405;
      }
      if ((v13 == 0x5346414f) || (v13 == 0xff534d42)) goto label_18405;
label_185a1:
      if (a1 != 3) goto label_18760;
      v3 = 0;
      v22 = 1;
    }
    if (*(unsigned char *)((long)a0 + 0x49) & 2) {
      v7 = sub_1fc80(v7,0x406,3);
      v17 = v7;
    }
    if ((0 <= v7) && (!sub_17bd0(a0,v2,v7,0))) goto label_17f32;
    if ((v22) && (v3))
      *(int *)(v2 + 0x40) = *__errno_location();
    *(unsigned short *)(v2 + 0x6a) = *(unsigned short *)(v2 + 0x6a) | 1;
    closedir(*(void **)(v2 + 0x18));
    v15 = *(unsigned int *)&a0[9];
    *(unsigned long *)(v2 + 0x18) = 0;
    if ((v15 & 0x200) && (0 <= v17)) {
      close(v17);
      v15 = *(unsigned int *)&a0[9];
    }
    *(unsigned long *)(v2 + 0x18) = 0;
    v22 = 0;
  }
label_17f3b:
  v21 = *(unsigned long *)(v2 + 0x48);
  v23 = v21;
  v28 = v21 - 1;
  if (*(char *)(*(long *)(v2 + 0x38) + -1 + v21) != '/') {
    v23 = v21 + 1;
    v28 = v21;
  }
  v26 = NULL;
  if (v15 & 4) {
    v26 = &((char *)(v28 + a0[4]))[1];
    *(char *)(v28 + a0[4]) = 0x2f;
  }
  v19 = *(void **)(v2 + 0x18);
  v13 = *(long *)(v2 + 0x58);
  v21 = a0[6] - v23;
  if (v19) { // branch-flip
    v8 = __errno_location();
    v20 = 0;
    v3 = 0;
    v29 = '\0';
    v24 = NULL;
    v25 = NULL;
    do {
      while( true ) {
        *v8 = 0;
        v9 = readdir(v19);
        if (!v9) {
          if (*v8) {
            *(int *)(v2 + 0x40) = *v8;
            *(unsigned short *)(v2 + 0x68) = (-(unsigned short)(!v14 && !v20) & 0xfffd) + 7;
          }
          if (*(void **)(v2 + 0x18)) {
            closedir(*(void **)(v2 + 0x18));
            *(unsigned long *)(v2 + 0x18) = 0;
          }
          goto label_1860e;
        }
        if (((*(unsigned char *)&a0[9] & 0x20) || (*(char *)((long)v9 + 0x13) != '.')) || ((*(char *)((long)v9 + 0x14) && (*(short *)((long)v9 + 0x14) != 0x2e)))) break;
label_18004:
        v19 = *(void **)(v2 + 0x18);
        if (!v19) goto label_1860e;
      }
      v10 = strlen((char *)((long)v9 + 0x13));
      v11 = (void *)sub_176c0(a0,(char *)((long)v9 + 0x13),v10);
      if (!v11) {
label_184a8:
        v7 = *v8;
        free(v11);
        while (v25) {
          v14 = *(void **)((long)v25 + 0x10);
          if (*(void **)((long)v25 + 0x18))
            closedir(*(void **)((long)v25 + 0x18));
          free(v25);
          v25 = v14;
        }
        closedir(*(void **)(v2 + 0x18));
        *(unsigned long *)(v2 + 0x18) = 0;
        *(unsigned short *)(v2 + 0x68) = 7;
        *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
        *v8 = v7;
        return NULL;
      }
      if (v21 <= v10) {
        v12 = a0[4];
        v4 = sub_178f0(a0,v28 + 2 + v10);
        if (!v4) goto label_184a8;
        if (a0[4] != v12) {
          v18 = (char *)(a0[4] + v23);
          if (!(*(unsigned char *)&a0[9] & 4))
            v18 = v26;
          v29 = v4;
          v26 = v18;
        }
        v21 = a0[6] - v23;
      }
      if (CARRY8(v10,v23)) {
        free(v11);
        while (v25) {
          v14 = *(void **)((long)v25 + 0x10);
          if (*(void **)((long)v25 + 0x18))
            closedir(*(void **)((long)v25 + 0x18));
          free(v25);
          v25 = v14;
        }
        closedir(*(void **)(v2 + 0x18));
        *(unsigned long *)(v2 + 0x18) = 0;
        *(unsigned short *)(v2 + 0x68) = 7;
        *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
        *v8 = 0x24;
        return NULL;
      }
      *(long *)((long)v11 + 0x58) = v13 + 1;
      v12 = *a0;
      *(unsigned long *)((long)v11 + 0x48) = v10 + v23;
      v15 = *(unsigned int *)&a0[9];
      *(long *)((long)v11 + 8) = v12;
      *(unsigned long *)((long)v11 + 0x78) = *v9;
      if (v15 & 4) { // branch-flip
        *(unsigned long *)((long)v11 + 0x30) = *(unsigned long *)((long)v11 + 0x38);
        memmove(v26,(void *)((long)v11 + 0x100),*(long *)((long)v11 + 0x60) + 1);
        v15 = *(unsigned int *)&a0[9];
      }
      else {
        *(void **)((long)v11 + 0x30) = (void *)((long)v11 + 0x100);
      }
      if ((a0[8]) && (!(v15 & 0x400)))
        *(unsigned short *)((long)v11 + 0x68) = sub_17400(a0,v11,0);
      else {
        v1 = *(unsigned char *)((long)v9 + 0x12);
        v5 = (unsigned int)v1 - 1;
        if ((v15 & 8) && (v1 & 0xfb)) {
          if ((v15 & 0x10) || (v1 != 10)) {
            *(unsigned short *)((long)v11 + 0x68) = 0xb;
            if (0xc <= v5) {
              v6 = 0;
              v16 = 1;
              goto label_180f8;
            }
            v16 = 1;
          }
          else {
            *(unsigned short *)((long)v11 + 0x68) = 0xb;
label_18268:
            v16 = 2;
          }
          v6 = *(unsigned int *)((unsigned long)v5 * 4 + 0x23120);
        }
        else {
          *(unsigned short *)((long)v11 + 0x68) = 0xb;
          if (v5 <= 0xb) goto label_18268;
          v6 = 0;
          v16 = 2;
        }
label_180f8:
        *(unsigned int *)((long)v11 + 0x88) = v6;
        *(unsigned long *)((long)v11 + 0xa0) = v16;
      }
      *(unsigned long *)((long)v11 + 0x10) = 0;
      if (v25) // branch-flip
        *(void **)((long)v24 + 0x10) = v11;
      else {
        v25 = v11;
      }
      if (v20 != 10000) {
        v20 += 1;
        if (v20 < v27) {
          v24 = v11;
          goto label_18004;
        }
        break;
      }
      if (!a0[8]) {
        v12 = sub_17760(v2,v17);
        if (((v12 == 0x1021994) || (v12 == 0xff534d42)) || (v12 == 0x6969)) {
          v24 = v11;
          v20 = 0x2711;
          v3 = 0;
          goto label_18004;
        }
        v3 = 1;
      }
      v20 = 0x2711;
      v19 = *(void **)(v2 + 0x18);
      v24 = v11;
    } while (v19);
label_1860e:
    if (v29) {
      v12 = a0[4];
      for (v13 = a0[1]; v13; v13 = *(long *)(v13 + 0x10)) {
        if (*(long *)(v13 + 0x30) != v13 + 0x100)
          *(long *)(v13 + 0x30) = (*(long *)(v13 + 0x30) - *(long *)(v13 + 0x38)) + v12;
        *(long *)(v13 + 0x38) = v12;
      }
      v13 = *(long *)((long)v25 + 0x58);
      v19 = v25;
      while (0 <= v13) {
        if (*(long *)((long)v19 + 0x30) != (long)v19 + 0x100)
          *(long *)((long)v19 + 0x30) = (*(long *)((long)v19 + 0x30) - *(long *)((long)v19 + 0x38)) + v12;
        v11 = *(void **)((long)v19 + 0x10);
        *(long *)((long)v19 + 0x38) = v12;
        if (!v11)
          v11 = *(void **)((long)v19 + 8);
        v13 = *(long *)((long)v11 + 0x58);
        v19 = v11;
      }
    }
    if (*(unsigned char *)&a0[9] & 4) {
      if ((a0[6] == v23) || (!v20)) goto label_1862b;
label_18631:
      *v26 = 0;
    }
    if ((!v14) && (v22)) {
      if ((a1 != 1) && (v20)) goto label_1865c;
      goto label_186b0;
    }
  }
  else {
    if (*(unsigned char *)&a0[9] & 4) {
      v3 = 0;
      v25 = NULL;
      v20 = 0;
label_1862b:
      v26 = &v26[-1];
      goto label_18631;
    }
    v25 = NULL;
    if (!(bool)(v22 & v14 == NULL)) goto label_186d5;
    v3 = 0;
    v20 = 0;
label_186b0:
    if (*(long *)(v2 + 0x58)) // branch-flip
      v7 = sub_17bd0(a0,*(unsigned long *)(v2 + 8),0xffffffff,"..");
    else {
      v7 = sub_17b40(a0);
    }
    if (v7) {
      *(unsigned short *)(v2 + 0x68) = 7;
      *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
      while (v25) {
        v14 = *(void **)((long)v25 + 0x10);
        if (*(void **)((long)v25 + 0x18))
          closedir(*(void **)((long)v25 + 0x18));
        free(v25);
        v25 = v14;
      }
      return (void *)0;
    }
  }
  if (!v20) {
label_186d5:
    if (((a1 == 3) && (*(short *)(v2 + 0x68) != 7)) && (*(short *)(v2 + 0x68) != 4))
      *(unsigned short *)(v2 + 0x68) = 6;
    while (v25) {
      v14 = *(void **)((long)v25 + 0x10);
      if (*(void **)((long)v25 + 0x18))
        closedir(*(void **)((long)v25 + 0x18));
      free(v25);
      v25 = v14;
    }
    return NULL;
  }
label_1865c:
  if (v3) {
    a0[8] = (long)sub_173e0;
    v14 = (void *)sub_175b0(a0,v25,v20);
    a0[8] = 0;
    return v14;
  }
  if (!a0[8])
    return v25;
  if (v20 == 1)
    return v25;
  return (void *)sub_175b0(a0,v25,v20);
}

// Function: sub_189c0 @ 0x189c0
long * sub_189c0(long *a0,unsigned long a1,long a2)
{
  void *v1;
  unsigned long v10;
  char *v11;
  long v12; // r13
  long *v13;
  void *v14; // stack - 0x50
  unsigned char v15; // stack - 0x42
  void *v16; // stack - 0x40
  unsigned int v2;
  int v3; // eax
  long *v4; // rax
  unsigned long v5; // rax
  void *v6;
  long v7;
  void *v8;
  unsigned int v9;
  
  v9 = (unsigned int)a1;
  if (((a1 & 0xfffff000) || ((v9 & 0x204) == 0x204)) || (!(a1 & 0x12))) {
    *__errno_location() = 0x16;
    return NULL;
  }
  v4 = calloc(1,0x80);
  if (!v4)
    return NULL;
  v4[8] = a2;
  v11 = (char *)*a0;
  *(unsigned int *)((long)v4 + 0x2c) = 0xffffff9c;
  v2 = v9 & 0xfffffdff | 4;
  if (!(a1 & 2))
    v2 = v9;
  *(unsigned int *)&v4[9] = v2;
  if (v11) { // branch-flip
    v10 = 0;
    v13 = a0;
    do {
      v5 = strlen(v11);
      if (v10 < v5)
        v10 = v5;
      v11 = (char *)v13[1];
      v13 = &v13[1];
    } while (v11);
    v10 += 1;
    if (v10 <= 0xfff)
      v10 = 0x1000;
  }
  else {
    v10 = 0x1000;
  }
  v15 = sub_178f0(v4,v10);
  if (!v15) {
    free(v4); // return-dupe
    return NULL;
  }
  if (*a0) { // branch-flip
    v14 = (void *)sub_176c0(v4,"",0);
    if (v14) {
      *(unsigned long *)((long)v14 + 0x58) = 0xffffffffffffffff;
      v11 = (char *)*a0;
      goto label_18ac3;
    }
  }
  else {
    v14 = NULL;
    v11 = NULL;
label_18ac3:
    if (a2)
      v15 = (unsigned char)(*(unsigned int *)&v4[9] >> 10) & 1;
    if (v11) { // branch-flip
      v16 = NULL;
      v8 = NULL;
      v7 = 0;
label_18b7d:
      do {
        v12 = v7;
        v1 = v16;
        v10 = strlen(v11);
        if (((3 <= v10) && (((unsigned char)(a1 >> 0xb) ^ 1) & 1)) && (v11[v10 - 1] == '/')) {
          do {
            if (v11[v10 - 2] != '/') break;
            v10 -= 1;
          } while (v10 != 1);
        }
        v6 = (void *)sub_176c0(v4,v11,v10);
        if (!v6) goto label_18c0d;
        *(unsigned long *)((long)v6 + 0x58) = 0;
        *(void **)((long)v6 + 8) = v14;
        *(long *)((long)v6 + 0x30) = (long)v6 + 0x100;
        if ((v8) && (v15)) {
          *(unsigned long *)((long)v6 + 0xa0) = 2;
          *(unsigned short *)((long)v6 + 0x68) = 0xb;
          if (a2) { // branch-flip
label_18b65:
            *(void **)((long)v6 + 0x10) = v8;
          }
          else {
            *(unsigned long *)((long)v6 + 0x10) = 0;
label_18d08:
            v16 = v6;
            *(void **)((long)v1 + 0x10) = v6;
            v6 = v8;
          }
          v11 = (char *)a0[v12 + 1];
          v8 = v6;
          v7 = v12 + 1;
          if (!v11) break;
          goto label_18b7d;
        }
        *(unsigned short *)((long)v6 + 0x68) = sub_17400(v4,v6,0);
        if (a2) goto label_18b65;
        *(unsigned long *)((long)v6 + 0x10) = 0;
        if (v8) goto label_18d08;
        v11 = (char *)a0[v12 + 1];
        v8 = v6;
        v7 = v12 + 1;
        v16 = v6;
      } while (v11);
      if ((a2) && (2 <= v12 + 1U))
        v6 = (void *)sub_175b0(v4,v6,v12 + 1U);
      v7 = sub_176c0(v4,"",0);
      *v4 = v7;
      v8 = v6;
      if (v7) {
        *(void **)(v7 + 0x10) = v6;
        *(unsigned short *)(v7 + 0x68) = 9;
        *(unsigned long *)(v7 + 0x58) = 1;
        if (sub_17960(v4)) {
label_18cdf:
          v9 = *(unsigned int *)&v4[9];
          if (!(v9 & 0x204)) {
            if (v9 & 0x200) // branch-flip
              v3 = sub_1b5d0(*(unsigned int *)((long)v4 + 0x2c),".");
            else {
              v3 = sub_1fef0(".",(v9 & 0x10) << 0xd | 0x90900);
            }
            *(int *)&v4[5] = v3;
            if (v3 <= -1)
              *(unsigned int *)&v4[9] = *(unsigned int *)&v4[9] | 4;
          }
          sub_1ab30(&v4[0xc],0xffffffff);
          return v4;
        }
      }
label_18c0d:
      while (v8) {
        v1 = *(void **)((long)v8 + 0x10);
        if (*(void **)((long)v8 + 0x18))
          closedir(*(void **)((long)v8 + 0x18));
        free(v8);
        v8 = v1;
      }
    }
    else {
      v7 = sub_176c0(v4,"",0);
      *v4 = v7;
      if (v7) {
        *(unsigned long *)(v7 + 0x10) = 0;
        *(unsigned short *)(v7 + 0x68) = 9;
        *(unsigned long *)(v7 + 0x58) = 1;
        if (sub_17960(v4)) goto label_18cdf;
      }
    }
    free(v14);
  }
  free((void *)v4[4]);
  free(v4);
  return NULL;
}

// Function: sub_18de0 @ 0x18de0
unsigned int sub_18de0(long *a0) // return-dupe
{
  void *v1;
  long v2;
  int v3; // eax
  int *v4; // rax
  void *v5;
  int v6;
  
  v5 = (void *)*a0;
  if (v5) {
    if (*(long *)((long)v5 + 0x58) > -1) {
      do {
        v1 = *(void **)((long)v5 + 0x10);
        if (v1) { // branch-flip
          free(v5);
          v2 = *(long *)((long)v1 + 0x58);
          v5 = v1;
        }
        else {
          v1 = *(void **)((long)v5 + 8);
          free(v5);
          v2 = *(long *)((long)v1 + 0x58);
          v5 = v1;
        }
      } while (v2 >= 0);
    }
    free(v5);
  }
  v5 = (void *)a0[1];
  while (v5) {
    v1 = *(void **)((long)v5 + 0x10);
    if (*(void **)((long)v5 + 0x18))
      closedir(*(void **)((long)v5 + 0x18));
    free(v5);
    v5 = v1;
  }
  free((void *)a0[2]);
  free((void *)a0[4]);
  if (*(unsigned int *)&a0[9] & 0x200) { // branch-flip
    v6 = *(int *)((long)a0 + 0x2c);
    if ((0 <= v6) && (close(v6))) {
      v6 = *__errno_location();
      goto label_18e9c;
    }
  }
  else if (!(*(unsigned int *)&a0[9] & 4)) {
    v2 = a0[5];
    if (fchdir((int)v2)) { // branch-flip
      v4 = __errno_location();
      v6 = *v4;
      v3 = close((int)a0[5]);
      if ((v6) || (!v3)) goto label_18e9c;
    }
    else {
      v2 = a0[5];
      if (!close((int)v2)) goto label_18e99;
      v4 = __errno_location();
    }
    v6 = *v4;
    goto label_18e9c;
  }
label_18e99:
  v6 = 0;
label_18e9c:
  while (!sub_1ab50(&a0[0xc])) {
    v3 = sub_1aba0(&a0[0xc]);
    if (0 <= v3)
      close(v3);
  }
  if (a0[10])
    sub_1a490(a0[10]);
  v5 = (void *)a0[0xb];
  if (*(unsigned int *)&a0[9] & 0x102) { // branch-flip
    if (v5)
      sub_1a490(v5);
  }
  else {
    free(v5);
  }
  free(a0);
  if (!v6)
    return 0;
  *__errno_location() = v6;
  return 0xffffffff;
}

// Function: sub_18fd0 @ 0x18fd0
void * sub_18fd0(long *a0)
{
  char *v1;
  char *v10; // rax
  unsigned long v11; // rax
  void *v12;
  long v13;
  short v2;
  unsigned int v3;
  unsigned long v4;
  void *v5;
  long v6;
  short v7;
  int v8;
  void *v9;
  
  v9 = (void *)*a0;
  if (!v9)
    return NULL;
  v3 = *(unsigned int *)&a0[9];
  if (v3 & 0x2000)
    return NULL;
  v7 = *(short *)((long)v9 + 0x6c);
  *(unsigned short *)((long)v9 + 0x6c) = 3;
  if (v7 == 1) {
    *(unsigned short *)((long)v9 + 0x68) = sub_17400(a0,v9,0);
    return v9;
  }
  v2 = *(short *)((long)v9 + 0x68);
  if (v7 != 2) { // branch-flip
    if (v2 != 1) { // branch-flip
label_19058:
      do {
        v12 = *(void **)((long)v9 + 0x10);
        if (!v12) {
          v12 = *(void **)((long)v9 + 8);
          if (*(long *)((long)v12 + 0x18)) {
            v6 = *(long *)((long)v12 + 0x48);
            *a0 = (long)v12;
            *(char *)(a0[4] + v6) = 0;
            v12 = (void *)sub_17ed0(a0,3);
            if (v12) {
              free(v9);
              goto label_192ea;
            }
            if (*(unsigned char *)((long)a0 + 0x49) & 0x20)
              return NULL;
            v12 = *(void **)((long)v9 + 8);
          }
          *a0 = (long)v12;
          free(v9);
          if (*(long *)((long)v12 + 0x58) == -1) {
            free(v12);
            *__errno_location() = 0;
            *a0 = 0;
            return NULL;
          }
          if (*(short *)((long)v12 + 0x68) != 0xb) {
            *(char *)(a0[4] + *(long *)((long)v12 + 0x48)) = 0;
            if (*(long *)((long)v12 + 0x58)) { // branch-flip
              if (*(unsigned short *)((long)v12 + 0x6a) & 2) {
                v8 = *(int *)((long)v12 + 0x44);
                if (!(*(unsigned int *)&a0[9] & 4)) {
                  if (*(unsigned int *)&a0[9] & 0x200) { // branch-flip
                    sub_17890(a0,v8,1);
                    v8 = *(int *)((long)v12 + 0x44);
                  }
                  else {
                    if (fchdir(v8)) {
                      *(int *)((long)v12 + 0x40) = *__errno_location();
                      *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
                    }
                    v8 = *(int *)((long)v12 + 0x44);
                  }
                }
                close(v8);
                goto label_190c3;
              }
              if (*(unsigned short *)((long)v12 + 0x6a) & 1) goto label_190c3;
              v8 = sub_17bd0(a0,*(unsigned long *)((long)v12 + 8),0xffffffff,"..");
            }
            else {
              v8 = sub_17b40(a0);
            }
            if (v8) {
              *(int *)((long)v12 + 0x40) = *__errno_location();
              *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
            }
label_190c3:
            if (*(short *)((long)v12 + 0x68) != 2) {
              if (*(int *)((long)v12 + 0x40)) // branch-flip
                *(unsigned short *)((long)v12 + 0x68) = 7;
              else {
                *(unsigned short *)((long)v12 + 0x68) = 6;
                sub_17a80(a0,v12);
              }
            }
            if (!(*(unsigned char *)((long)a0 + 0x49) & 0x20))
              return v12;
            return NULL;
          }
          abort(); // no-return, return-dupe
        }
        *a0 = (long)v12;
        free(v9);
        v9 = v12;
        if (!*(long *)((long)v12 + 0x58)) {
          if (sub_17b40(a0)) {
            *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
            return NULL;
          }
          v5 = (void *)a0[0xb];
          if (*(unsigned int *)&a0[9] & 0x102) { // branch-flip
            if (v5)
              sub_1a490(v5);
          }
          else {
            free(v5);
          }
          v5 = (void *)a0[4];
          v1 = (char *)((long)v12 + 0x100);
          *(long *)((long)v12 + 0x48) = *(long *)((long)v12 + 0x60);
          memmove(v5,v1,*(long *)((long)v12 + 0x60) + 1);
          v10 = strrchr(v1,0x2f);
          if ((v10) && ((v1 != v10 || (*(char *)((long)v12 + 0x101))))) {
            v11 = strlen(&v10[1]);
            memmove(v1,&v10[1],v11 + 1);
            *(unsigned long *)((long)v12 + 0x60) = v11;
          }
          v6 = a0[4];
          *(long *)((long)v12 + 0x38) = v6;
          *(long *)((long)v12 + 0x30) = v6;
          sub_17960(a0);
          v7 = *(short *)((long)v12 + 0x68);
          goto label_1932b;
        }
      } while (*(short *)((long)v12 + 0x6c) == 4);
      if (*(short *)((long)v12 + 0x6c) == 2) {
        v7 = sub_17400(a0,v12,1);
        *(short *)((long)v12 + 0x68) = v7;
        if ((v7 == 1) && (v3 = *(unsigned int *)&a0[9], !(v3 & 4))) {
          if (v3 & 0x200) // branch-flip
            v8 = sub_1b5d0(*(unsigned int *)((long)a0 + 0x2c),".");
          else {
            v8 = sub_1fef0(".",(v3 & 0x10) << 0xd | 0x90900);
          }
          *(int *)((long)v12 + 0x44) = v8;
          if (0 <= v8) // branch-flip
            *(unsigned short *)((long)v12 + 0x6a) = *(unsigned short *)((long)v12 + 0x6a) | 2;
          else {
            v8 = *__errno_location();
            *(unsigned short *)((long)v12 + 0x68) = 7;
            *(int *)((long)v12 + 0x40) = v8;
          }
        }
        *(unsigned short *)((long)v12 + 0x6c) = 3;
      }
    }
    else {
      if (v7 == 4) goto label_191c2;
label_19127:
      if ((v3 & 0x40) && (*(long *)((long)v9 + 0x70) != a0[3])) {
label_191c2:
        if (*(unsigned char *)((long)v9 + 0x6a) & 2)
          close(*(int *)((long)v9 + 0x44));
        v12 = (void *)a0[1];
        if (v12) {
          do {
            v5 = *(void **)((long)v12 + 0x10);
            if (*(void **)((long)v12 + 0x18))
              closedir(*(void **)((long)v12 + 0x18));
            free(v12);
            v12 = v5;
          } while (v5);
          a0[1] = 0;
        }
        *(unsigned short *)((long)v9 + 0x68) = 6;
        sub_17a80(a0,v9);
        return v9;
      }
      if ((void *)a0[1]) { // branch-flip
        if (v3 & 0x1000) {
          *(unsigned int *)&a0[9] = v3 & 0xffffefff;
          v12 = (void *)a0[1];
          do {
            v5 = *(void **)((long)v12 + 0x10);
            if (*(void **)((long)v12 + 0x18))
              closedir(*(void **)((long)v12 + 0x18));
            free(v12);
            v12 = v5;
          } while (v5);
          a0[1] = 0;
          goto label_1941a;
        }
        v4 = *(unsigned long *)((long)v9 + 0x30);
        if (sub_17bd0(a0,v9,0xffffffff,v4)) { // branch-flip
          v8 = *__errno_location();
          *(unsigned short *)((long)v9 + 0x6a) = *(unsigned short *)((long)v9 + 0x6a) | 1;
          *(int *)((long)v9 + 0x40) = v8;
          v12 = (void *)a0[1];
          if (v12) {
            v9 = v12;
            do {
              *(unsigned long *)((long)v9 + 0x30) = *(unsigned long *)(*(long *)((long)v9 + 8) + 0x30);
              v9 = *(void **)((long)v9 + 0x10);
            } while (v9);
          }
        }
        else {
          v12 = (void *)a0[1];
        }
      }
      else {
label_1941a:
        v12 = (void *)sub_17ed0(a0,3);
        a0[1] = (long)v12;
        if (!v12) {
          if (!(*(unsigned char *)((long)a0 + 0x49) & 0x20)) {
            if ((*(int *)((long)v9 + 0x40)) && (*(short *)((long)v9 + 0x68) != 4))
              *(unsigned short *)((long)v9 + 0x68) = 7;
            sub_17a80(a0,v9);
            return v9;
          }
          return NULL;
        }
      }
      a0[1] = 0;
    }
label_192ea:
    v6 = *(long *)(*(long *)((long)v12 + 8) + 0x48);
    v13 = v6 + -1;
    if (*(char *)(*(long *)(*(long *)((long)v12 + 8) + 0x38) + -1 + v6) != '/')
      v13 = v6;
    v6 = a0[4];
    *(char *)(v13 + v6) = 0x2f;
    memmove(&((char *)(v13 + v6))[1],(void *)((long)v12 + 0x100),*(long *)((long)v12 + 0x60) + 1);
    v7 = *(short *)((long)v12 + 0x68);
    v9 = v12;
label_1932b:
    *a0 = (long)v9;
    if (v7 == 0xb) goto label_193a8;
  }
  else {
    if (2 <= (unsigned short)(v2 - 0xcU)) {
      if (v2 == 1) goto label_19127;
      goto label_19058;
    }
    v7 = sub_17400(a0,v9,1);
    *(short *)((long)v9 + 0x68) = v7;
    if (v7 == 1) {
      v3 = *(unsigned int *)&a0[9];
      if (v3 & 4) {
        *a0 = (long)v9;
        goto label_1933f;
      }
      if (v3 & 0x200) // branch-flip
        v8 = sub_1b5d0(*(unsigned int *)((long)a0 + 0x2c),".");
      else {
        v8 = sub_1fef0(".",(v3 & 0x10) << 0xd | 0x90900);
      }
      *(int *)((long)v9 + 0x44) = v8;
      if (v8 < 0) {
        v8 = *__errno_location();
        *(unsigned short *)((long)v9 + 0x68) = 7;
        *(int *)((long)v9 + 0x40) = v8;
        *a0 = (long)v9;
        return v9;
      }
      *(unsigned short *)((long)v9 + 0x6a) = *(unsigned short *)((long)v9 + 0x6a) | 2;
      v7 = *(short *)((long)v9 + 0x68);
      goto label_1932b;
    }
    *a0 = (long)v9;
    if (v7 != 0xb)
      return v9;
label_193a8:
    if (*(long *)((long)v9 + 0xa0) != 2) {
      if (*(long *)((long)v9 + 0xa0) == 1)
        return v9;
      abort();
    }
    v7 = sub_17400(a0,v9,0);
    *(short *)((long)v9 + 0x68) = v7;
  }
  if (v7 != 1)
    return v9;
label_1933f:
  if (!*(long *)((long)v9 + 0x58))
    a0[3] = *(long *)((long)v9 + 0x70);
  if (!sub_179d0(a0,v9)) {
    v9 = NULL;
    *__errno_location() = 0xc;
  }
  return v9;
}

// Function: sub_19720 @ 0x19720
unsigned long sub_19720(unsigned long a0,long a1,unsigned int a2)
{
  if (a2 <= 4) {
    *(short *)(a1 + 0x6c) = (short)a2;
    return 0;
  }
  *__errno_location() = 0x16;
  return 1;
}

// Function: sub_19750 @ 0x19750
int * sub_19750(long *a0,unsigned int a1) // early-return x2
{
  unsigned int v1;
  long v2;
  void *v3;
  void *v4;
  int *v5;
  int v6;
  int *v7; // rax
  unsigned long v8; // r12
  int v9;
  
  v7 = __errno_location();
  v5 = v7;
  if (a1 & 0xffffefff) {
    *v7 = 0x16;
    v7 = NULL;
    return v7;
  }
  v2 = *a0;
  *v7 = 0;
  if (*(unsigned char *)((long)a0 + 0x49) & 0x20) {
    v7 = NULL;
    return v7;
  }
  if (*(short *)(v2 + 0x68) == 9) {
    v7 = (int *)*(long *)(v2 + 0x10);
    return v7;
  }
  if (*(short *)(v2 + 0x68) != 1) {
    v7 = NULL;
    return v7;
  }
  v4 = (void *)a0[1];
  while (v4) {
    v3 = *(void **)((long)v4 + 0x10);
    if (*(void **)((long)v4 + 0x18))
      closedir(*(void **)((long)v4 + 0x18));
    free(v4);
    v4 = v3;
  }
  v8 = 1;
  if (a1 == 0x1000) {
    *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x1000;
    v8 = 2;
  }
  if (((*(long *)(v2 + 0x58)) || (**(char **)(v2 + 0x30) == '/')) || (v1 = *(unsigned int *)&a0[9], v1 & 4)) {
    v7 = (long)sub_17ed0(a0,v8);
    a0[1] = (long)v7;
  }
  else {
    if (v1 & 0x200) { // branch-flip
      v6 = sub_1b5d0(*(unsigned int *)((long)a0 + 0x2c),".");
      v9 = v6;
    }
    else {
      v6 = sub_1fef0(".",(v1 & 0x10) << 0xd | 0x90900);
      v9 = v6;
    }
    if (v9 < 0) {
      a0[1] = 0;
      v7 = NULL;
      return v7;
    }
    v7 = (long)sub_17ed0(a0,v8);
    a0[1] = (long)v7;
    if (*(unsigned char *)((long)a0 + 0x49) & 2) // branch-flip
      sub_17890(a0,v9,1);
    else {
      v6 = fchdir(v9);
      if (v6) {
        v6 = *v5;
        close(v9);
        *v5 = v6;
        v7 = NULL;
        return v7;
      }
      close(v9);
    }
    v7 = (long)a0[1];
  }
  return v7;
}

// Function: sub_19950 @ 0x19950
unsigned long sub_19950(void)
{
  return __progname;
}

// Function: sub_19960 @ 0x19960
unsigned long sub_19960(unsigned long a0,unsigned long a1)
{
  return (a0 >> 3 | a0 << 0x3d) % a1;
}

// Function: sub_19980 @ 0x19980
bool sub_19980(long a0,long a1)
{
  return a1 == a0;
}

// Function: sub_19990 @ 0x19990
unsigned long sub_19990(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0x231c0)
    return 1;
  v1 = v2[2];
  if ((((dat_231d4 < v1) && (v1 < dat_231d8)) && (dat_231dc < v2[3])) && (dat_231f0 <= *v2)) {
    v3 = *v2 + dat_231d4;
    if (((v3 < v2[1]) && (v2[1] <= dat_231e0)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0x231c0;
  return 0;
}

// Function: sub_19a20 @ 0x19a20
unsigned long sub_19a20(long *a0,long a1,unsigned long *a2,char a3) // early-return
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

// Function: sub_19b30 @ 0x19b30
unsigned long sub_19b30(float a0,unsigned long a1,char a2) // ternary
{
  char v1 [16];
  unsigned long v2; // rax
  unsigned long v3; // rcx
  unsigned long v4; // rsi
  long v5; // rdi
  unsigned long v6;
  
  if (!a2) {
    a0 = (float)a1 / a0;
    if (dat_231e4 <= a0) {
      v2 = 0;
      return v2;
    }
    a1 = (a0 < dat_231e8) ? (unsigned long)a0 : (long)(a0 - dat_231e8) ^ 0x8000000000000000;
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
          if (v6 <= v4) goto label_19c05;
          v5 += 8;
        } while (v6 % v3);
      }
    }
    else {
      v3 = 3;
label_19c05:
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

// Function: sub_19c70 @ 0x19c70
unsigned long sub_19c70(long *a0,unsigned long *a1,char a2) // return-dupe x2
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
label_19ca6:
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
        goto label_19ca6;
      }
    }
    v6 = &v6[2];
    if ((long *)a1[1] <= v6)
      return 1;
  } while( true );
}

// Function: sub_19de0 @ 0x19de0
unsigned long sub_19de0(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}

// Function: sub_19df0 @ 0x19df0
unsigned long sub_19df0(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}

// Function: sub_19e00 @ 0x19e00
unsigned long sub_19e00(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}

// Function: sub_19e10 @ 0x19e10
unsigned long sub_19e10(unsigned long *a0)
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

// Function: sub_19e70 @ 0x19e70
bool sub_19e70(unsigned long *a0)
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
label_19e90:
      v2 = &v2[2];
    }
    v1 = v2[1];
    v4 += 1;
    v3 += 1;
    if (!v1) goto label_19e90;
    do {
      v1 = *(long *)(v1 + 8);
      v3 += 1;
    } while (v1);
    v2 = &v2[2];
  } while( true );
}

// Function: sub_19ee0 @ 0x19ee0
void sub_19ee0(unsigned long *a0,void *a1)
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
      if ((long *)a0[1] <= v4) goto label_19f4d;
    }
    v5 = 1;
    for (v3 = v4[1]; v3; v3 = *(long *)(v3 + 8)) {
      v5 += 1;
    }
    if (v6 < v5)
      v6 = v5;
  }
label_19f4d:
  __fprintf_chk(a1,1,"# entries:         %lu\n",a0[4]);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v2);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v6); // tail-call
  return;
}

// Function: sub_1a040 @ 0x1a040
long sub_1a040(long *a0,long a1) // return-dupe
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

// Function: sub_1a0b0 @ 0x1a0b0
long sub_1a0b0(unsigned long *a0)
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

// Function: sub_1a100 @ 0x1a100
long sub_1a100(long *a0,long a1)
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

// Function: sub_1a190 @ 0x1a190
void sub_1a190(unsigned long *a0,long a1,unsigned long a2)
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

// Function: sub_1a1e0 @ 0x1a1e0
long sub_1a1e0(unsigned long *a0,void *a1,unsigned long a2)
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

// Function: sub_1a260 @ 0x1a260
unsigned long sub_1a260(unsigned char *a0,unsigned long a1)
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

// Function: sub_1a2a0 @ 0x1a2a0
void sub_1a2a0(unsigned long *a0)
{
  *(char *)&a0[2] = 0;
  *a0 = 0x3f80000000000000;
  a0[1] = 0x3fb4fdf43f4ccccd;
}

// Function: sub_1a2d0 @ 0x1a2d0
unsigned long * sub_1a2d0(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_19960;
  if (!a3)
    a3 = sub_19980;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = 0x231c0;
  v1[5] = a1;
  if (sub_19990(v1)) {
    v2 = sub_19b30(*(unsigned int *)(a1 + 8),a0,*(char *)(a1 + 0x10));
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

// Function: sub_1a3e0 @ 0x1a3e0
void sub_1a3e0(unsigned long *a0)
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
        if ((long *)a0[1] <= v5) goto label_1a47b;
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
label_1a47b:
  a0[3] = 0;
  a0[4] = 0;
  return;
}

// Function: sub_1a490 @ 0x1a490
void sub_1a490(unsigned long *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = (long *)*a0;
  v3 = (long *)a0[1];
  if ((a0[8]) && (a0[4])) {
    if (v3 <= v4) goto label_1a53c;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_1a4ff;
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
label_1a4ff:
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
label_1a53c:
  v2 = (void *)a0[9];
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free((void *)*a0);
  free(a0); // tail-call
}

// Function: sub_1a580 @ 0x1a580
unsigned int sub_1a580(unsigned long *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_19b30(*(unsigned int *)(a0[5] + 8),a1,*(char *)(a0[5] + 0x10));
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
  v1 = sub_19c70(&v2,a0,0);
  if (!(char)v1) {
    a0[9] = v11;
    if ((sub_19c70(a0,&v2,1)) && (sub_19c70(a0,&v2,0))) {
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

// Function: sub_1a700 @ 0x1a700
long * sub_1a700(long a0,long a1,long *a2) // return-dupe, ternary
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
  v2 = (long)sub_19a20(a0,a1,&v3,0);
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
    if ((long)v2 > -1) goto label_1a78f;
label_1a816:
    v5 = (float)v2;
  }
  else {
    v2 = *(long **)(a0 + 0x10);
    v6 = (float)v4;
    if (0 > (long)v2) goto label_1a816;
label_1a78f:
    v5 = (float)(long)v2;
  }
  if (*(float *)(*(long *)(a0 + 0x28) + 8) * v5 < v6) {
    sub_19990(a0);
    v1 = *(long *)(a0 + 0x28);
    if (v5 * *(float *)(v1 + 8) < v6) {
      v5 = v5 * *(float *)(v1 + 0xc);
      if (!*(char *)(v1 + 0x10))
        v5 = v5 * *(float *)(v1 + 8);
      if (dat_231e4 <= v5) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v4 = (dat_231e8 <= v5) ? (long)(v5 - dat_231e8) ^ 0x8000000000000000 : (unsigned long)v5;
      if (!sub_1a580(a0,v4)) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v2 = (long)sub_19a20(a0,a1,&v3,0);
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

// Function: sub_1a930 @ 0x1a930
unsigned long sub_1a930(unsigned long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_1a700(a0,a1,&v3);
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

// Function: sub_1a990 @ 0x1a990
long sub_1a990(long a0,unsigned long a1) // ternary x2
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
  
  v5 = sub_19a20(a0,a1,&v7,1);
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
      sub_19990(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_231e8 <= v8) ? (long)(v8 - dat_231e8) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_1a580(a0,v6)) {
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

// Function: sub_1ab20 @ 0x1ab20
void sub_1ab20(void)
{
  sub_1a990(); // tail-call
}

// Function: sub_1ab30 @ 0x1ab30
void sub_1ab30(unsigned int *a0,unsigned int a1)
{
  *(unsigned long *)&a0[5] = 0;
  *(char *)&a0[7] = 1;
  *a0 = a1;
  a0[1] = a1;
  a0[2] = a1;
  a0[3] = a1;
  a0[4] = a1;
}

// Function: sub_1ab50 @ 0x1ab50
char sub_1ab50(long a0)
{
  return *(char *)(a0 + 0x1c);
}

// Function: sub_1ab60 @ 0x1ab60
unsigned int sub_1ab60(long a0,unsigned int a1)
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

// Function: sub_1aba0 @ 0x1aba0
unsigned int sub_1aba0(long a0)
{
  unsigned int v1;
  unsigned int v2;
  
  if (*(char *)(a0 + 0x1c))
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

// Function: sub_1abe0 @ 0x1abe0
int sub_1abe0(int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (0 <= v1)
    return v1;
  return (unsigned long)(iswcntrl(a0) == 0);
}

// Function: sub_1ac10 @ 0x1ac10
void sub_1ac10(long *a0,long *a1) // return-dupe
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

// Function: sub_1ac70 @ 0x1ac70
unsigned int sub_1ac70(unsigned char a0)
{
  return *(unsigned int *)((unsigned long)(a0 >> 5) * 4 + 0x23200) >> (a0 & 0x1f) & 1;
}

// Function: sub_1ac90 @ 0x1ac90
void sub_1ac90(unsigned long a0,unsigned long a1,long a2)
{
  if (!a2)
    a2 = 0x3bb10;
  sub_1acc0(0,a0,a1,a2); // tail-call
}

// Function: sub_1acc0 @ 0x1acc0
unsigned long sub_1acc0(unsigned int *a0,unsigned char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,(char *)a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_1ff70(0))) {
    v1 = 1;
    *a0 = (unsigned int)*a1;
  }
  return v1;
}

// Function: sub_1ad50 @ 0x1ad50
void sub_1ad50(char *a0)
{
  unsigned char v1;
  unsigned long v2;
  unsigned long v3; // rax
  long v4; // rax
  
  if (!*a0) {
    v1 = **(unsigned char **)&a0[0x10];
    if (*(unsigned int *)((unsigned long)(v1 >> 5) * 4 + 0x23200) >> (v1 & 0x1f) & 1) {
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
      goto label_1ad91;
    }
    if (!mbsinit(&a0[4]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    *a0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v2 = *(unsigned long *)&a0[0x10];
  v4 = sub_1acc0(&a0[0x24],v2,sub_1e760(v2,v3),&a0[4]);
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
label_1ad91:
  a0[0xc] = '\x01';
  return;
}

// Function: sub_1aec0 @ 0x1aec0
unsigned long sub_1aec0(unsigned char *a0,unsigned char *a1) // early-return
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
    sub_1ad50(v7);
    if ((v13) && (!v14)) break;
    if (!v16)
      sub_1ad50(v8);
    if (v19) { // branch-flip
      if (!v20) {
        if (!v10) goto label_1b0a7;
        goto label_1b0af;
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
  if (v10) goto label_1b0ca;
label_1b0a7:
  sub_1ad50(v7);
label_1b0af:
  if ((!v13) || (v14))
    return 1;
label_1b0ca:
  if (!v16)
    sub_1ad50(v8);
  if ((v19) && (!v20))
    return 0;
  return 0xffffffff;
}

// Function: sub_1b140 @ 0x1b140
unsigned long * sub_1b140(unsigned long *a0,unsigned char a1,unsigned char a2,unsigned long a3) // early-return, return-dupe
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
    if ((~v2 & v2 + 0xfefefefefefefeff | v5 + 0xfefefefefefefeff & ~v5) & 0x8080808080808080) goto label_1b226;
    a0 = &a0[1];
  }
  if (!a3)
    return NULL;
label_1b226:
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

// Function: sub_1b280 @ 0x1b280
void sub_1b280(void)
{
  __fprintf_chk(stderr,1,"%s\n",dcgettext(NULL,"memory exhausted",5));
  exit(dat_2b080); // no-return
}

// Function: sub_1b2d0 @ 0x1b2d0
long * sub_1b2d0(long *a0,long a1,long a2) // ternary
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
  v1 = (*(unsigned char *)&a0[10] & 1) ? (long *)(*(void *)a0[7])(a0[9]) : (long *)(*(void *)a0[7])(a1); // branch-flip
  a0[1] = (long)v1;
  if (v1) {
    v2 = (long)v1 + v3 + 0x10 & -a2;
    a0[2] = v2;
    a0[3] = v2;
    v3 = *a0;
    *v1 = v3 + (long)v1;
    a0[4] = v3 + (long)v1;
    v1[1] = 0;
    v1 = (long *)0x1;
    *(unsigned char *)&a0[10] = *(unsigned char *)&a0[10] & 0xf9;
    return v1;
  }
  (*obstack_alloc_failed_handler)(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: _obstack_begin @ 0x1b370
void _obstack_begin(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) & 0xfe;
  *(unsigned long *)(a0 + 0x38) = a3;
  *(unsigned long *)(a0 + 0x40) = a4;
  sub_1b2d0(); // tail-call
}

// Function: _obstack_begin_1 @ 0x1b390
void _obstack_begin_1(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5)
{
  *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 1;
  *(unsigned long *)(a0 + 0x38) = a3;
  *(unsigned long *)(a0 + 0x40) = a4;
  *(unsigned long *)(a0 + 0x48) = a5;
  sub_1b2d0(); // tail-call
}

// Function: _obstack_newchunk @ 0x1b3b0
void _obstack_newchunk(unsigned long *a0,unsigned long a1) // ternary
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
    v4 = (a0[10] & 1) ? (unsigned long *)(*(void *)a0[7])(a0[9]) : (unsigned long *)(*(void *)a0[7])(v5); // branch-flip
    if (v4) {
      a0[1] = (unsigned long)v4;
      v3 = (void *)a0[2];
      v4[1] = v2;
      a0[4] = (long)v4 + v5;
      *v4 = (long)v4 + v5;
      v7 = (void *)((long)v4 + a0[6] + 0x10 & ~a0[6]);
      memcpy(v7,v3,v6);
      v1 = a0[10];
      if ((!(v1 & 2)) && (a0[2] == (~a0[6] & v2 + 0x10 + a0[6]))) {
        v4[1] = *(unsigned long *)(v2 + 8);
        if (v1 & 1) // branch-flip
          (*(void *)a0[8])(a0[9],v2);
        else {
          (*(void *)a0[8])(v2);
        }
      }
      a0[2] = (unsigned long)v7;
      a0[3] = (long)v7 + v6;
      *(unsigned char *)&a0[10] = (unsigned char)a0[10] & 0xfd;
      return;
    }
  }
  (*obstack_alloc_failed_handler)(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: _obstack_allocated_p @ 0x1b4e0
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

// Function: _obstack_free @ 0x1b520
void _obstack_free(long a0,unsigned long *a1) // return-dupe
{
  unsigned long *v1;
  unsigned long *v2;
  
  v2 = *(unsigned long **)(a0 + 8);
  if (*(unsigned long **)(a0 + 8)) {
    while( true ) {
      if ((v2 < a1) && (v1 = (unsigned long *)*v2, a1 <= v1)) {
        *(unsigned long **)(a0 + 0x18) = a1;
        *(unsigned long **)(a0 + 0x10) = a1;
        *(unsigned long **)(a0 + 0x20) = v1;
        *(unsigned long **)(a0 + 8) = v2;
        return;
      }
      v1 = (unsigned long *)v2[1];
      if (*(unsigned char *)(a0 + 0x50) & 1) { // branch-flip
        (**(void **)(a0 + 0x40))(*(unsigned long *)(a0 + 0x48));
        *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 2;
      }
      else {
        (**(void **)(a0 + 0x40))(v2);
        *(unsigned char *)(a0 + 0x50) = *(unsigned char *)(a0 + 0x50) | 2;
      }
      if (!v1) break;
      v2 = v1;
    }
  }
  if (!a1)
    return;
  abort(); // no-return
}

// Function: _obstack_memory_used @ 0x1b5a0
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

// Function: sub_1b5d0 @ 0x1b5d0
void sub_1b5d0(int a0,char *a1,unsigned int a2,unsigned long a3)
{
  unsigned long v1; // rcx
  
  v1 = 0;
  if (a2 & 0x40)
    v1 = a3 & 0xffffffff;
  sub_1e7a0(openat(a0,a1,a2,v1));
}

// Function: sub_1b650 @ 0x1b650
void * sub_1b650(unsigned long a0,unsigned long a1,unsigned int a2,int *a3)
{
  int v1;
  int v2; // eax
  void *v3; // rax
  int *v4; // rax
  
  v3 = NULL;
  v2 = sub_1b5d0(a0,a1,a2 | 0x90900);
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

// Function: sub_1b6b0 @ 0x1b6b0
char * sub_1b6b0(char *a0,int a1) // return-dupe x3
{
  unsigned char *v1; // rax
  
  v1 = (unsigned char *)sub_20000();
  if ((*v1 & 0xdf) != 0x55) { // branch-flip
    if (((((*v1 & 0xdf) == 0x47) && ((v1[1] & 0xdf) == 0x42)) && (v1[2] == 0x31)) && (((v1[3] == 0x38 && (v1[4] == 0x30)) && ((v1[5] == 0x33 && ((v1[6] == 0x30 && (!v1[7])))))))) {
      if (*a0 == '`')
        return (char *)0x2323c;
      return (char *)0x23237;
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

// Function: sub_1b790 @ 0x1b790
long * sub_1b790(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_1b800:
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
label_1bca6:
      v2 = 0;
      goto label_1bc60;
    case 2:
      if (!v18) {
        v2 = 0;
        goto label_1bdc1;
      }
      v26 = 1;
      v2 = 0;
      v13 = 0;
      v31 = 1;
      v32 = "\'";
      break;
    case 3:
      v2 = 1;
label_1bc60:
      v26 = 1;
      v13 = 0;
      a4 = 2;
      v31 = 1;
      v32 = "\'";
      break;
    case 4:
      if (!v18) {
        v2 = 1;
        goto label_1bdc1;
      }
      goto label_1bca6;
    case 5:
      if (!v18) goto label_1bc01;
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
          v10 = (char *)sub_1b6b0("`",a4);
          v37 = (char *)v10;
        }
        v10 = (char *)dcgettext(NULL,"\'",5);
        v36 = (char *)v10;
        if (v10 == (long *)0x230dc) {
          v10 = (char *)sub_1b6b0("\'",a4);
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
label_1b8eb:
  do {
label_1b8fe:
    v24 = 0;
    a1 = v14;
label_1b908:
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
          if ((a3 < v24 + v31) || (v9 = memcmp(v12,v32,v31), v9)) goto label_1c270;
          if (!v26) {
            v8 = *v12;
            v19 = (unsigned long)v8;
            v22 = v19;
            if ((char)v8 <= '?') {
              switch(v8) {
                case 0:
                  goto label_1be6c;
                default:
                  goto label_1bb43;
                case 7:
                  goto label_1be57;
                case 8:
                  goto label_1be26;
                case 9:
                  goto label_1bef3;
                case 10:
                  goto label_1be4d;
                case 0xb:
                  goto label_1bf20;
                case 0xc:
                  goto label_1be0b;
                case 0xd:
                  goto label_1bd5d;
                case 0x20:
                  goto label_1bf2d;
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
                  goto label_1baf6;
                case 0x23:
                  goto label_1bed0;
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
                  goto label_1ba28;
                case 0x27:
                  goto label_1bd42;
                case 0x3f:
                  goto label_1bd01;
                
              }
            }
            v5 = v23;
            if ((char)v8 <= 'z') {
              if (v8 == 0x40) goto label_1bb43;
              v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
              if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_1ba28;
              if (!((unsigned long)v10 & 0xa4000000)) goto label_1c314;
              goto label_1c258;
            }
            if (v8 == 0x7d) goto label_1c3e0;
            if ((char)v8 <= '}') {
              v22 = 0x7b;
              if (v8 == 0x7b) goto label_1c23a;
              v22 = 0x7c;
              if (v8 == 0x7c) goto label_1c258;
              goto label_1bb43;
            }
            if (v8 != 0x7e) goto label_1bb43;
label_1c394:
            if (!v24) {
              v8 = a4 == 2;
              v22 = 0x7e;
              v27 = v3;
              goto label_1bbb9;
            }
            v22 = 0x7e;
            goto label_1c3a2;
          }
          goto label_1bb2c;
        }
label_1c270:
        v8 = *v12;
        v19 = (unsigned long)v8;
        v22 = v19;
        if ('?' < (char)v8) {
          if ('z' < (char)v8) {
            if (v8 != 0x7d) { // branch-flip
              if ('~' <= (char)v8) {
                v23 = 0;
                if (v8 == 0x7e) goto label_1c394;
                goto label_1bb40;
              }
              v22 = 0x7b;
              v23 = 0;
              if (v8 != 0x7b) {
                v22 = 0x7c;
                v23 = 0;
                v5 = 0;
                if (v8 != 0x7c) goto label_1bb43;
                goto label_1c258;
              }
            }
            else {
              v22 = 0x7d;
              v23 = 0;
            }
label_1c23a:
            if (a3 == 0xffffffffffffffff) goto label_1c3ef;
label_1c248:
            v5 = v23;
            if (a3 == 1) goto label_1bed5;
label_1c258:
            v8 = a4 == 2;
            v27 = v3;
            v25 = 0;
            v23 = v5;
            goto label_1bbb9;
          }
          if (v8 == 0x40) goto label_1bb40;
          v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
          v23 = 0;
          if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_1ba28;
          v23 = 0;
          v5 = 0;
          if ((unsigned long)v10 & 0xa4000000) goto label_1c258;
label_1c314:
          if ((char)v19 == '\\') {
            if (((bool)(v2 & v26)) && (v31)) goto label_1c33a;
            v17 = 0x5c;
            goto label_1bd62;
          }
label_1bb43:
          v17 = (unsigned char)v19;
          if (v34 != 1) goto label_1c0c3;
label_1bb50:
          v10 = __ctype_b_loc();
          v22 = 1;
          v27 = (*(unsigned char *)(*v10 + 1 + v19 * 2) & 0x40) == 0;
          v28 = (bool)(v27 & v2);
          v27 = !v27;
          goto label_1bb9e;
        }
        switch(v8) {
          case 0:
            goto label_1be5e;
          default:
label_1bb40:
            v23 = 0;
            goto label_1bb43;
          case 7:
label_1be57:
            v17 = 0x61;
            goto label_1be37;
          case 8:
label_1be26:
            v17 = 0x62;
            goto label_1be37;
          case 9:
            v23 = 0;
label_1bef3:
            v22 = 9;
            v17 = 0x74;
            goto label_1bf00;
          case 10:
label_1be4d:
            v17 = 0x6e;
            break;
          case 0xb:
label_1bf20:
            v17 = 0x76;
            break;
          case 0xc:
label_1be0b:
            v17 = 0x66;
label_1be37:
            if (v26) {
              v2 = 1;
              goto label_1bd8f;
            }
            v25 = 0;
            goto label_1baae;
          case 0xd:
label_1bd5d:
            v17 = 0x72;
            break;
          case 0x20:
            v23 = 0;
label_1bf2d:
            v22 = 0x20;
            goto label_1ba28;
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
            goto label_1baf9;
          case 0x23:
            v22 = 0x23;
            v23 = 0;
            goto label_1bed5;
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
label_1ba28:
            v8 = 0;
            goto label_1ba38;
          case 0x27:
            v23 = 0;
            goto label_1bd42;
          case 0x3f:
            v23 = 0;
            goto label_1bd01;
          
        }
        goto label_1bd62;
      }
      v17 = *v12;
      v19 = (unsigned long)v17;
      if ('?' < (char)v17) {
        if ('{' <= (char)v17) {
          if (v17 == 0x7d) {
label_1c3e0:
            v22 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_1c248;
label_1c3ef:
            v5 = v23;
            if (a2[1]) goto label_1c258;
            goto label_1bed5;
          }
          if ((char)v17 <= '}') {
            v22 = 0x7b;
            if (v17 != 0x7b) {
              if (v17 == 0x7c) goto label_1baf6;
              goto label_1bb43;
            }
            goto label_1c23a;
          }
          v22 = 0x7e;
          if (v17 == 0x7e) goto label_1bed5;
          if (v34 == 1) goto label_1bb50;
label_1c0c3:
          v15 = 0;
          if (a3 == 0xffffffffffffffff) {
            v10 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v10;
          }
          v22 = 0;
          do {
            v19 = v24 + v22;
            v10 = (long)sub_1acc0(&v16,&a2[v19],a3 - v19,&v15);
            v7 = (long)v10;
            v25 = v27;
            if (!v10) break;
            if (v10 == (long *)0xffffffffffffffff) {
              v25 = 0;
              goto label_1c62b;
            }
            if (v10 == (long *)0xfffffffffffffffe) {
              v10 = (long *)v19;
              if (v19 >= a3) goto label_1c9af;
              goto label_1c9a5;
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
                if ((v8 <= 0x21) && (0x20000002bU >> (v19 & 0x3f) & 1)) goto label_1c1fd;
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
          goto label_1c62b;
        }
        if (v17 == 0x40) goto label_1bb43;
        v10 = (unsigned long)(1L << (v17 + 0xbf & 0x3f));
        if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_1bbb0;
        if ((unsigned long)v10 & 0xa4000000) goto label_1baf6;
        if (v17 != 0x5c) goto label_1bb43;
        if (a4 == 2) {
          if (!v26) goto label_1c33a;
          goto label_1bb23;
        }
        if (!(bool)(v2 & v26 & v31 != 0)) {
          v22 = 0x5c;
          v17 = 0x5c;
          v23 = 0;
          goto label_1bf00;
        }
label_1c33a:
        v24 += 1;
        v25 = 0;
        v17 = 0x5c;
        v8 = v21;
        goto label_1c350;
      }
      switch(v17) {
        case 0:
          if (!v2) {
            v23 = 0;
            v22 = 0;
            if (v35 & 1) goto label_1bfd2;
            goto label_1bf0b;
          }
label_1be5e:
          if (!v26) {
            v23 = 0;
label_1be6c:
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
                goto label_1bbe3;
              }
label_1c775:
              a0[v14] = 0x5c;
              v13 = v14;
              v21 = (bool)v8;
            }
            else if (v13 < a1) {
              v8 = v21;
              goto label_1c775;
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
label_1c7ac:
              v17 = (unsigned char)v22;
              v25 = 0;
              v13 = v14;
              v8 = v27;
              if (!v2 || v28) goto label_1bbe3;
              goto label_1ba38;
            }
            v17 = 0x30;
            v25 = 0;
            v13 = v14;
            v8 = v27;
            goto label_1bbe3;
          }
          if (a4 != 2) goto label_1bb2c;
          goto label_1bd9b;
        default:
          goto label_1bb43;
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
          goto label_1bf8f;
        case 10:
          v22 = 10;
          v17 = 0x6e;
          goto label_1bf8f;
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
label_1bf8f:
          v8 = a4 == 2 & v26;
          v23 = 0;
          if (!(bool)v8) goto label_1bf00;
label_1bb11:
          a4 = 2;
label_1bb23:
          if (!v29) goto label_1bb2c;
          goto label_1bd9b;
        case 0x20:
          v19 = 0x20;
          goto label_1baf9;
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
label_1baf6:
          v27 = 0;
label_1baf9:
          v22 = v19;
          v25 = v27;
          if ((a4 == 2) && (v26)) goto label_1bb11;
          goto label_1bbb7;
        case 0x23:
label_1bed0:
          v22 = 0x23;
label_1bed5:
          if (!v24) {
            v19 = v22;
            goto label_1baf9;
          }
label_1c3a2:
          v8 = a4 == 2;
          v27 = v3;
          v25 = 0;
          goto label_1bbb9;
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
          goto label_1bbb0;
        case 0x27:
label_1bd42:
          v25 = v27;
          if (a4 != 2) {
            v8 = 0;
            v22 = 0x27;
            goto label_1bbb9;
          }
          if (!v26) {
            if (a1) { // branch-flip
              v14 = 0;
              v22 = a1;
              if (v33) goto label_1c7cb;
            }
            else {
label_1c7cb:
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
            goto label_1bbe3;
          }
          goto label_1bb23;
        case 0x3f:
label_1bd01:
          if (a4 == 2) {
            if (v26) goto label_1bb23;
            v8 = 0;
            v17 = 0x3f;
            v25 = 0;
            goto label_1bbe3;
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
                goto label_1bbb9;
              }
              if (v26) goto label_1bb2c;
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
              goto label_1c7ac;
            }
          }
          v8 = 0;
          v22 = 0x3f;
          v27 = v3;
          v25 = 0;
          goto label_1bbb9;
        
      }
label_1bf00:
      if (!v2) {
label_1bf0b:
        v17 = (unsigned char)v22;
        v25 = 0;
        v8 = 0;
        v27 = v3;
        if (!v26) goto label_1bbe0;
        goto label_1ba38;
      }
label_1bd62:
      v25 = 0;
      goto label_1bd73;
    }
    if (!(bool)(v13 == 0 & v26 & a4 == 2)) {
      v8 = a4 == 2 & (v26 ^ 1U);
      v26 = (bool)(v26 ^ 1U);
      if ((!(bool)v8) || (v26 = (bool)v8, !v3)) {
label_1ca6b:
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
        if (a1 || !v33) goto label_1ca6b;
        v24 = v33;
        goto label_1c4fb;
      }
      v13 = 0;
      a4 = 5;
      v10 = (unsigned long)__ctype_get_mb_cur_max();
      v31 = 1;
      v34 = (unsigned long)v10;
      v32 = "\"";
      if (!(v35 & 2)) goto label_1cb1b;
      v3 = 0;
      v21 = 0;
      v33 = 0;
      v2 = v4;
      v26 = v4;
      goto label_1b8fe;
    }
label_1c1fd:
    if (v2) {
label_1bd9b:
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
label_1bdc1:
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
label_1c4fb:
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
label_1c9a5:
    if (!a2[(long)v10]) break;
  }
label_1c9af:
  v25 = 0;
label_1c62b:
  v19 = (unsigned long)v17;
  v27 = v25;
  if (2 <= v22) {
label_1c635:
    v20 = 0;
    v22 += v24;
    v14 = v24;
    do {
      v17 = (unsigned char)v19;
      if (v28) {
        v29 = a4 == 2;
        if (v26) goto label_1bb23;
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
        if (v22 <= v24) goto label_1bac1;
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
          goto label_1c350;
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
label_1bb9e:
  if (v28) {
    v25 = 0;
    v28 = v2;
    goto label_1c635;
  }
label_1bbb0:
  v22 = v19;
  v25 = v27;
label_1bbb7:
  v8 = a4 == 2;
  v27 = v3;
label_1bbb9:
  v17 = (unsigned char)v22;
  v28 = (bool)v8 == 0;
  v3 = v27;
  v8 = 0;
  if ((v28 && v2) || (v8 = 0, v26)) {
label_1ba38:
    v17 = (unsigned char)v22;
    v13 = v14;
    if (!v30) goto label_1bbe3;
    if (!(*(unsigned int *)(v30 + (v22 >> 5) * 4) >> (v17 & 0x1f) & 1)) goto label_1bbe3;
  }
  else {
label_1bbe0:
    v8 = 0;
    v3 = v27;
label_1bbe3:
    if (!v23) {
      v8 ^= 1;
      v24 += 1;
      v8 &= v21;
label_1c350:
      if ((bool)v8) {
        if (v13 < a1)
          a0[v13] = 0x27;
        if (v13 + 1 < a1)
          a0[v13 + 1] = 0x27;
        v21 = 0;
        v13 += 2;
      }
      goto label_1bac1;
    }
  }
label_1bd73:
  if (v26) {
label_1bd8f:
    if ((bool)(v2 & a4 == 2)) goto label_1bd9b;
label_1bb2c:
    v35 &= 0xfffffffd;
    v30 = 0;
    goto label_1b800;
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
label_1baae:
  if (v13 < a1)
    a0[v13] = 0x5c;
  v13 += 1;
  v24 += 1;
label_1bac1:
  if (v13 < a1)
    a0[v13] = v17;
  v13 += 1;
  if (!v25)
    v4 = 0;
  goto label_1b908;
label_1bfd2:
  v24 += 1;
  goto label_1b908;
label_1cb1b:
  a1 = v33;
label_1bc01:
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
  goto label_1b8eb;
}

// Function: sub_1cc20 @ 0x1cc20
void * sub_1cc20(unsigned int a0,unsigned long a1,unsigned long a2,unsigned int *a3)
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
    v6 = dat_2b0e0;
    if (dat_2b0d8 <= (int)a0) {
      v10 = (long)dat_2b0d8;
      v9 = (long)(int)((a0 - dat_2b0d8) + 1);
      if (dat_2b0e0 != (unsigned long *)0x2b0f0) // branch-flip
        v6 = (unsigned long *)sub_1f1f0(dat_2b0e0,&v10,v9,0x7fffffff,0x10);
      else {
        v6 = (unsigned long *)sub_1f1f0(0,&v10,v9,0x7fffffff,0x10);
        *v6 = dat_2b0f0;
        v6[1] = dat_2b0f8;
      }
      dat_2b0e0 = v6;
      memset(&v6[(long)dat_2b0d8 * 2],0,(v10 - dat_2b0d8) * 0x10);
      dat_2b0d8 = (int)v10;
    }
    v1 = &v6[(long)(int)a0 * 2];
    v3 = a3[1];
    v5 = *v1;
    v8 = (void *)v1[1];
    v7 = sub_1b790(v8,v5,a1,a2,*a3,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    if (v5 <= v7) {
      v7 += 1;
      *v1 = v7;
      if (v8 != (void *)0x3bb20)
        free(v8);
      v8 = (void *)sub_1ef50(v7);
      v4 = *a3;
      v1[1] = (unsigned long)v8;
      sub_1b790(v8,v7,a1,a2,v4,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    }
    *v11 = v2;
    return v8;
  }
  abort(); // no-return
}

// Function: sub_1ce10 @ 0x1ce10
void sub_1ce10(long a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = 0x3bc20;
  sub_1f350(a0,0x38);
  *v2 = v1;
}

// Function: sub_1ce50 @ 0x1ce50
unsigned int sub_1ce50(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x3bc20;
  return *a0;
}

// Function: sub_1ce70 @ 0x1ce70
void sub_1ce70(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x3bc20;
  *a0 = a1;
}

// Function: sub_1ce90 @ 0x1ce90
unsigned int sub_1ce90(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0x3bc20;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_1ced0 @ 0x1ced0
unsigned int sub_1ced0(long a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = 0x3bc20;
  v1 = *(unsigned int *)(a0 + 4);
  *(unsigned int *)(a0 + 4) = a1;
  return v1;
}

// Function: sub_1cef0 @ 0x1cef0
void sub_1cef0(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0x3bc20;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_1cf30 @ 0x1cf30
void sub_1cf30(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x3bc20;
  v2 = __errno_location();
  v1 = *v2;
  sub_1b790(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(unsigned long *)&a4[10],*(unsigned long *)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_1cfb0 @ 0x1cfb0
unsigned long sub_1cfb0(unsigned long a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x3bc20;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_1b790(0,0,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  v4 = sub_1ef50(v3 + 1);
  sub_1b790(v4,v3 + 1,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_1d0a0 @ 0x1d0a0
void sub_1d0a0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_1cfb0(a0,a1,0,a2); // tail-call
}

// Function: sub_1d0b0 @ 0x1d0b0
void sub_1d0b0(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_2b0e0;
  if (2 <= dat_2b0d8) {
    v4 = (unsigned long *)((long)dat_2b0e0 + 0x18);
    v1 = (unsigned long *)((long)dat_2b0e0 + (unsigned long)(unsigned int)(dat_2b0d8 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0x3bb20) {
    free(*(void **)((long)v3 + 8));
    dat_2b0f8 = 0x3bb20;
    dat_2b0f0 = 0x100;
  }
  if (v3 == (void *)0x2b0f0) {
    dat_2b0d8 = 1;
    return;
  }
  free(v3);
  dat_2b0d8 = 1;
  dat_2b0e0 = (void *)0x2b0f0;
}

// Function: sub_1d150 @ 0x1d150
void sub_1d150(unsigned long a0,unsigned long a1)
{
  sub_1cc20(a0,a1,0xffffffffffffffff,0x3bc20); // tail-call
}

// Function: sub_1d170 @ 0x1d170
void sub_1d170(void)
{
  sub_1cc20(); // tail-call
}

// Function: sub_1d180 @ 0x1d180
void sub_1d180(unsigned long a0)
{
  sub_1cc20(0,a0,0xffffffffffffffff,0x3bc20); // tail-call
}

// Function: sub_1d1a0 @ 0x1d1a0
void sub_1d1a0(unsigned long a0,unsigned long a1)
{
  sub_1cc20(0,a0,a1,0x3bc20); // tail-call
}

// Function: sub_1d1c0 @ 0x1d1c0
void sub_1d1c0(unsigned long a0,int a1,unsigned long a2)
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
    sub_1cc20(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_1d260 @ 0x1d260
void sub_1d260(unsigned long a0,int a1,unsigned long a2,unsigned long a3)
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
    sub_1cc20(a0,a2,a3,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_1d2f0 @ 0x1d2f0
void sub_1d2f0(int a0,unsigned long a1)
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
    sub_1cc20(0,a1,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_1d380 @ 0x1d380
void sub_1d380(int a0,unsigned long a1,unsigned long a2)
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
    sub_1cc20(0,a1,a2,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_1d410 @ 0x1d410
void sub_1d410(unsigned long a0,unsigned long a1,unsigned char a2)
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
  
  v3 = dat_3bc20;
  v4 = dat_3bc28;
  v9 = dat_3bc50;
  v5 = dat_3bc30;
  v6 = dat_3bc38;
  v7 = dat_3bc40;
  v8 = dat_3bc48;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a2 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a2 & 0x1f)) & 1) << (a2 & 0x1f) ^ v2;
  sub_1cc20(0,a0,a1,&v3);
}

// Function: sub_1d4b0 @ 0x1d4b0
void sub_1d4b0(unsigned long a0,unsigned char a1)
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
  
  v3 = dat_3bc20;
  v4 = dat_3bc28;
  v9 = dat_3bc50;
  v5 = dat_3bc30;
  v6 = dat_3bc38;
  v7 = dat_3bc40;
  v8 = dat_3bc48;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a1 & 0x1f)) & 1) << (a1 & 0x1f) ^ v2;
  sub_1cc20(0,a0,0xffffffffffffffff,&v3);
}

// Function: sub_1d550 @ 0x1d550
void sub_1d550(unsigned long a0)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_3bc20;
  v3 = (unsigned int)((unsigned long)dat_3bc28 >> 0x20);
  v8 = dat_3bc50;
  v4 = dat_3bc30;
  v5 = dat_3bc38;
  v6 = dat_3bc40;
  v7 = dat_3bc48;
  v2 = (unsigned int)dat_3bc28;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_1cc20(0,a0,0xffffffffffffffff,&v1);
}

// Function: sub_1d5e0 @ 0x1d5e0
void sub_1d5e0(unsigned long a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_3bc20;
  v3 = (unsigned int)((unsigned long)dat_3bc28 >> 0x20);
  v8 = dat_3bc50;
  v4 = dat_3bc30;
  v5 = dat_3bc38;
  v6 = dat_3bc40;
  v7 = dat_3bc48;
  v2 = (unsigned int)dat_3bc28;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_1cc20(0,a0,a1,&v1);
}

// Function: sub_1d670 @ 0x1d670
void sub_1d670(unsigned long a0,int a1,unsigned long a2)
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
    sub_1cc20(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_1d710 @ 0x1d710
void sub_1d710(unsigned long a0,long a1,long a2,unsigned long a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_3bc28;
  v4 = dat_3bc30;
  v5 = dat_3bc38;
  v8 = dat_3bc50;
  v2 = (unsigned int)((unsigned long)dat_3bc20 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_3bc40;
  v7 = dat_3bc48;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_1cc20(a0,a3,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_1d7b0 @ 0x1d7b0
void sub_1d7b0(unsigned int a0,long a1,long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_3bc28;
  v4 = dat_3bc30;
  v5 = dat_3bc38;
  v8 = dat_3bc50;
  v2 = (unsigned int)((unsigned long)dat_3bc20 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_3bc40;
  v7 = dat_3bc48;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_1cc20(a0,a3,a4,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_1d850 @ 0x1d850
void sub_1d850(long a0,long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_3bc28;
  v4 = dat_3bc30;
  v5 = dat_3bc38;
  v8 = dat_3bc50;
  v2 = (unsigned int)((unsigned long)dat_3bc20 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_3bc40;
  v7 = dat_3bc48;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_1cc20(0,a2,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_1d8f0 @ 0x1d8f0
void sub_1d8f0(long a0,long a1,unsigned long a2,unsigned long a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_3bc28;
  v4 = dat_3bc30;
  v5 = dat_3bc38;
  v8 = dat_3bc50;
  v2 = (unsigned int)((unsigned long)dat_3bc20 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_3bc40;
  v7 = dat_3bc48;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_1cc20(0,a2,a3,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_1d990 @ 0x1d990
void sub_1d990(void)
{
  sub_1cc20(); // tail-call
}

// Function: sub_1d9a0 @ 0x1d9a0
void sub_1d9a0(unsigned long a0,unsigned long a1)
{
  sub_1cc20(0,a0,a1,0x2b0a0); // tail-call
}

// Function: sub_1d9c0 @ 0x1d9c0
void sub_1d9c0(unsigned long a0,unsigned long a1)
{
  sub_1cc20(a0,a1,0xffffffffffffffff,0x2b0a0); // tail-call
}

// Function: sub_1d9e0 @ 0x1d9e0
void sub_1d9e0(unsigned long a0)
{
  sub_1cc20(0,a0,0xffffffffffffffff,0x2b0a0); // tail-call
}

// Function: sub_1da00 @ 0x1da00
long sub_1da00(int a0,void *a1,unsigned long a2)
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

// Function: sub_1da70 @ 0x1da70
void sub_1da70(unsigned long a0,long a1,long a2)
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
  if (dat_3bc58) { // branch-flip
    v4 = (*dat_3bc58)(v1,0);
    if (v4)
      return;
    if (dat_3bc70) goto label_1dabe;
  }
  else {
    if (!dat_3bc70) {
      signal(0xb,0); // return-dupe, tail-call
      return;
    }
label_1dabe:
    v3 = dat_3bc78;
    v2 = *(unsigned long *)(a2 + 0xa0);
    if (dat_3bc78) {
      v6 = __errno_location();
      v4 = *v6;
      v5 = sub_1e340(v3,&v7);
      *v6 = v4;
      if (0 <= v5) {
        if (v7 <= v1) { // branch-flip
          if (v1 <= (unsigned long)(v8 - 1U)) goto label_1dba5;
        }
        else if ((*v9)(v1,&v7)) {
label_1dba5:
          (*dat_3bc70)(dat_3bc68 <= v2 && v2 <= dat_3bc68 + dat_3bc60,a2);
        }
      }
    }
  }
  if ((dat_3bc58) && (v4 = (*dat_3bc58)(v1,1), v4))
    return;
  signal(0xb,0);
  return;
}

// Function: sub_1dc00 @ 0x1dc00
void sub_1dc00(void)
{
  void *v1; // stack - 0xa8
  char v2 [128];
  int v3; // stack - 0x20
  
  v1 = sub_1da70;
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
  v3 = (-(unsigned int)(dat_3bc70 == 0) & 0xf8000000) + 0x8000004;
  sigaction(0xb,&v1,NULL);
}

// Function: sub_1dd80 @ 0x1dd80
unsigned long sub_1dd80(unsigned long a0)
{
  dat_3bc58 = a0;
  sub_1dc00();
  return 0;
}

// Function: sub_1dda0 @ 0x1dda0
void sub_1dda0(void)
{
  dat_3bc58 = 0;
  if (dat_3bc70)
    return;
  signal(0xb,0); // tail-call
}

// Function: sub_1ddd0 @ 0x1ddd0
unsigned long sub_1ddd0(void *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  (*a0)(a1,a2,a3);
  return 1;
}

// Function: sub_1de00 @ 0x1de00
long sub_1de00(unsigned long a0,unsigned long a1,unsigned long a2)
{
  int v1; // eax
  long v2; // rax
  unsigned long v3; // stack - 0x58
  char v4 [4];
  unsigned int v5; // stack - 0x50
  unsigned int v6; // stack - 0x4c
  unsigned long v7; // stack - 0x48
  
  if (!dat_3bc78) {
    v1 = sub_1e340(v4,&v3);
    if (0 <= v1) { // branch-flip
      v2 = CONCAT44(v6,v5) + -1;
      dat_3bc78 = v2;
    }
    else {
      v2 = dat_3bc78;
    }
    if (!v2) {
      v2 = 0xffffffff;
      return v2;
    }
  }
  v5 = 0;
  dat_3bc60 = a2;
  dat_3bc68 = a1;
  dat_3bc70 = a0;
  v3 = a1;
  v7 = a2;
  v1 = sigaltstack(&v3,0);
  if (v1 <= -1) {
    v2 = 0xffffffff;
    return v2;
  }
  sub_1dc00();
  v2 = 0;
  return v2;
}

// Function: sub_1dee0 @ 0x1dee0
void sub_1dee0(void) // return-dupe
{
  char v1 [8];
  unsigned int v2; // stack - 0x20
  
  dat_3bc70 = 0;
  if (dat_3bc58) // branch-flip
    sub_1dc00();
  else {
    signal(0xb,0);
  }
  v2 = 2;
  if (0 <= (int)sigaltstack(v1,0))
    return;
  perror("gnulib sigsegv (stackoverflow_deinstall_handler)");
}

// Function: sub_1df70 @ 0x1df70
bool sub_1df70(long a0,long *a1)
{
  return (unsigned long)(*a1 - a0) <= (unsigned long)(*a1 - a1[3]) >> 1;
}

// Function: sub_1df90 @ 0x1df90
unsigned long sub_1df90(long *a0,long *a1)
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
        if (5 < (unsigned int)v1 - 0x61) goto label_1e010;
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
label_1e010:
  if (v4) {
    *a1 = v3;
    return 0;
  }
  return 0xffffffff;
}

// Function: sub_1e020 @ 0x1e020
unsigned long sub_1e020(unsigned long a0,long *a1) // return-dupe
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
  
  v1 = dat_3bc80;
  v3 = a0 * 2 - *a1;
  if (a0 < v3)
    return 0;
  v4 = (v3 / dat_3bc80) * dat_3bc80;
  v5 = (((unsigned long)(*a1 - 1U) / dat_3bc80 + 1) * dat_3bc80 - v4) / dat_3bc80;
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

// Function: sub_1e160 @ 0x1e160
unsigned long sub_1e160(unsigned long a0)
{
  unsigned long v1;
  unsigned long v2;
  char v3 [1032];
  long v4; // rsi
  unsigned long v5; // r14
  unsigned long v6; // r15
  
  v2 = 0x400;
  while( true ) {
    v1 = dat_3bc80;
    if (!a0)
      return 0;
    if (a0 / dat_3bc80 < v2)
      v2 = a0 / dat_3bc80;
    v4 = dat_3bc80 * v2;
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

// Function: sub_1e240 @ 0x1e240
long sub_1e240(long a0) // return-dupe
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  char v4 [1032];
  
  v1 = dat_3bc80;
  a0 += dat_3bc80;
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

// Function: sub_1e340 @ 0x1e340
unsigned long sub_1e340(char *a0,unsigned long *a1)
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
            if (!v5) goto label_1e568;
            if (v12 < (unsigned long)(long)(v5 + 0x1049)) goto label_1e400;
            v6 = (long)v5;
            while( true ) {
              v5 = (int)read(v4,&v16[v6],v12 - v6);
              if (v5 <= -1) break;
              if (v12 - v6 < (unsigned long)(long)(v5 + 0x1049)) goto label_1e400;
              if (!v5) {
                close(v4);
                v10 = NULL;
                goto label_1e5d0;
              }
              v6 = v5 + v6;
            }
          }
          if (*__errno_location() != 4) {
label_1e568:
            close(v4);
            goto label_1e4a0;
          }
        }
        else {
label_1e400:
          if (v11) { // branch-flip
            v12 *= 2;
            if (!v12) goto label_1e568;
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
            goto label_1e4b4;
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
label_1e4a0:
    if (v17)
      munmap(v17,v18);
  }
label_1e4b4:
  v12 = dat_3bc80;
  if (!dat_3bc80) {
    v12 = (unsigned long)getpagesize();
    dat_3bc80 = v12;
  }
  v6 = ((unsigned long)a0 / v12) * v12;
  *a1 = sub_1e160(v6);
  a1[1] = sub_1e240(v6);
  v7 = sub_1e020;
label_1e6b3:
  a1[2] = v7;
  return 0;
label_1e5d0:
  if (((((int)sub_1df90() < 0) || (v15 == v6)) || (v2 = &v16[v15], v15 = v15 + 1, *v2 != '-')) || ((int)sub_1df90() < 0)) goto label_1e4a0;
  do {
    if (v6 == v15) break;
    v2 = &v16[v15];
    v15 += 1;
  } while (*v2 != '\n');
  v8 = v14;
  if (v13 <= v19) { // branch-flip
    v1 = &v14[-1];
    if (v1 < &v20[-1]) goto label_1e680;
    v3 = v10;
    if ((v13 < v19) && ((v3 = v19, v13 <= a0 && (a0 <= &v19[-1])))) {
      v14 = v19;
      goto label_1e68e;
    }
    v10 = v3;
    if (v1 <= &v20[-1]) // branch-flip
      v8 = v10;
    else if ((v20 <= a0) && (a0 <= v1)) {
      v13 = v20;
      goto label_1e68e;
    }
  }
  else {
label_1e680:
    if ((v13 <= a0) && (a0 <= &v14[-1])) {
label_1e68e:
      *a1 = v13;
      a1[1] = v14;
      a1[3] = v10;
      if (v17)
        munmap(v17,v18);
      v7 = sub_1df70;
      goto label_1e6b3;
    }
  }
  v10 = v8;
  goto label_1e5d0;
}

// Function: sub_1e760 @ 0x1e760
void * sub_1e760(void *a0,unsigned long a1)
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

// Function: sub_1e7a0 @ 0x1e7a0
unsigned int sub_1e7a0(unsigned int a0)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  
  if (3 <= a0)
    return a0;
  v2 = sub_20100();
  v3 = __errno_location();
  v1 = *v3;
  close(a0);
  *v3 = v1;
  return v2;
}

// Function: sub_1e800 @ 0x1e800
long sub_1e800(void *a0,long a1,unsigned long a2,unsigned long a3,unsigned long *a4,unsigned long a5)
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
label_1e92d:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_1e966:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0x23970) + 0x23970;
  switch(a5) {
    case 0:
      goto label_1e966;
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
      goto label_1e92d;
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

// Function: sub_1ec80 @ 0x1ec80
void sub_1ec80(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_1e800(); // tail-call
}

// Function: sub_1eca0 @ 0x1eca0
void sub_1eca0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
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
  sub_1e800(a0,a1,a2,a3);
}

// Function: sub_1ed40 @ 0x1ed40
void sub_1ed40(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5)
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
  sub_1e800(a0,a1,a2);
}

// Function: sub_1ee20 @ 0x1ee20
void sub_1ee20(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-grep@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU grep","https://www.gnu.org/software/grep/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_1eed0 @ 0x1eed0
void sub_1eed0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_1f410(); // no-return
}

// Function: sub_1ef10 @ 0x1ef10
void sub_1ef10(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_1f410(); // no-return
}

// Function: sub_1ef30 @ 0x1ef30
void sub_1ef30(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_1f410(); // no-return
}

// Function: sub_1ef50 @ 0x1ef50
void sub_1ef50(void)
{
  sub_1ef10(); // tail-call
}

// Function: sub_1ef60 @ 0x1ef60
void sub_1ef60(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_1f410(); // no-return
}

// Function: sub_1efa0 @ 0x1efa0
void sub_1efa0(void *a0,unsigned long a1)
{
  if (realloc(a0,a1 | a1 == 0))
    return;
  sub_1f410(); // no-return
}

// Function: sub_1efd0 @ 0x1efd0
void sub_1efd0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_1f410(); // no-return
}

// Function: sub_1f010 @ 0x1f010
void sub_1f010(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!a1) || (!a2)) {
    a1 = 1;
    a2 = 1;
  }
  if (reallocarray(a0,a1,a2))
    return;
  sub_1f410(); // no-return
}

// Function: sub_1f050 @ 0x1f050
void sub_1f050(unsigned long a0,unsigned long a1)
{
  if (reallocarray(NULL,a0,a1))
    return;
  sub_1f410(); // no-return
}

// Function: sub_1f080 @ 0x1f080
void sub_1f080(unsigned long a0,unsigned long a1)
{
  if ((!a0) || (!a1)) {
    a0 = 1;
    a1 = 1;
  }
  if (!reallocarray(NULL,a0,a1))
    sub_1f410(); // no-return
}

// Function: sub_1f0d0 @ 0x1f0d0
void sub_1f0d0(void *a0,unsigned long *a1)
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
  sub_1f410(); // no-return
}

// Function: sub_1f160 @ 0x1f160
void sub_1f160(void *a0,unsigned long *a1,unsigned long a2)
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
  sub_1f410(); // no-return
}

// Function: sub_1f1f0 @ 0x1f1f0
void sub_1f1f0(void *a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= (long)v3) goto label_1f2ae;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_1f2ae:
  if (!a0)
    *a1 = 0;
  if (((a2 <= v2 - v1) || ((v2 = v1 + a2, !SCARRY8(v1,a2) && (((v2 <= a3 || (a3 <= -1)) && (v3 = v2 * a4, SEXT816((long)v3) == SEXT816(v2) * SEXT816(a4))))))) && ((realloc(a0,v3) || ((a0 && (!v3)))))) {
    *a1 = v2;
    return;
  }
  sub_1f410(); // no-return
}

// Function: sub_1f2f0 @ 0x1f2f0
void sub_1f2f0(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_1f410(); // no-return
}

// Function: sub_1f310 @ 0x1f310
void sub_1f310(unsigned long a0)
{
  sub_1f2f0(a0,1); // tail-call
}

// Function: sub_1f320 @ 0x1f320
void sub_1f320(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_1f410(); // no-return
}

// Function: sub_1f340 @ 0x1f340
void sub_1f340(unsigned long a0)
{
  sub_1f320(a0,1); // tail-call
}

// Function: sub_1f350 @ 0x1f350
void sub_1f350(void *a0,unsigned long a1)
{
  memcpy((void *)sub_1ef10(a1),a0,a1); // tail-call
}

// Function: sub_1f380 @ 0x1f380
void sub_1f380(void *a0,unsigned long a1)
{
  memcpy((void *)sub_1ef30(a1),a0,a1); // tail-call
}

// Function: sub_1f3b0 @ 0x1f3b0
void sub_1f3b0(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_1ef30(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_1f3f0 @ 0x1f3f0
void sub_1f3f0(char *a0)
{
  sub_1f350(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_1f410 @ 0x1f410
void sub_1f410(void)
{
  error(dat_2b080,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_1f450 @ 0x1f450
unsigned int sub_1f450(unsigned char *a0,long *a1,unsigned int a2,long *a3,char *a4) // ternary x6
{
  unsigned char v1;
  long v10;
  unsigned char v11; // r8b
  unsigned int v12; // edi
  int v13; // esi
  long v14; // r9
  unsigned int v15;
  unsigned char v2;
  unsigned char *v3;
  int *v4; // rax
  long v5;
  long v6; // rax
  long v7;
  int v8; // ecx
  long v9; // rcx
  
  if (0x25 <= a2)
    __assert_fail("0 <= strtol_base && strtol_base <= 36","xstrtol.c",0x55,"xstrtoimax"); // no-return
  if (!a1)
    a1 = &v10;
  v4 = __errno_location();
  *v4 = 0;
  v5 = strtoimax(a0,a1,a2);
  v3 = (unsigned char *)*a1;
  if (v3 != a0) { // branch-flip
    if (*v4) { // branch-flip
      if (*v4 != 0x22)
        return 4;
      v15 = 1;
    }
    else {
      v15 = 0;
    }
    if (!a4) goto label_1f4db;
    v1 = *v3;
    if (!v1) goto label_1f4db;
    if (!strchr(a4,(int)(char)v1)) goto label_1f4e2;
  }
  else {
    if (!a4)
      return 4;
    v1 = *v3;
    if (!v1)
      return 4;
    v15 = 0;
    v5 = 1;
    if (!strchr(a4,(int)(char)v1))
      return 4;
  }
  v12 = (unsigned int)v1 - 0x45;
  v13 = 1;
  v8 = 0x400;
  v11 = (unsigned char)v12;
  if ((v11 <= 0x2f) && (0x814400308945U >> ((unsigned long)v12 & 0x3f) & 1)) {
    v8 = 0x400;
    v13 = 1;
    if (!strchr(a4,0x30)) goto label_1f58c;
    v2 = v3[1];
    if (v2 == 0x44) {
label_1f938:
      v13 = 2;
      v8 = 1000;
      goto label_1f58c;
    }
    if (v2 == 0x69) {
      v13 = (v3[2] == 0x42) + 1 + (unsigned int)(v3[2] == 0x42);
      goto label_1f58c;
    }
    if (v2 == 0x42) goto label_1f938;
    switch(v11) {
      case 0:
        v14 = 1;
        v8 = 0x400;
        goto label_1f7bd;
      default:
        goto label_1f4e2;
      case 2:
      case 0x22:
        v14 = 1;
        v8 = 0x400;
        goto label_1f6fb;
      case 6:
      case 0x26:
        v14 = 1;
        v8 = 0x400;
        goto label_1f6d8;
      case 8:
      case 0x28:
        v14 = 1;
        v8 = 0x400;
        goto label_1f6b8;
      case 0xb:
        v14 = 1;
        v8 = 0x400;
        goto label_1f77b;
      case 0xf:
      case 0x2f:
        v14 = 1;
        v8 = 0x400;
        goto label_1f65e;
      case 0x14:
        v14 = 1;
        v8 = 0x400;
        goto label_1f845;
      case 0x15:
        v14 = 1;
        v8 = 0x400;
        goto label_1f805;
      case 0x1d:
        v14 = 1;
        goto label_1f885;
      case 0x1e:
        v14 = 1;
      
    }
    goto label_1f69b;
  }
label_1f58c:
  switch((unsigned int)v1 - 0x42 & 0xff) {
    case 0:
      v7 = v5 * 0x400;
      v14 = (long)v13;
      if (SEXT816(v7) == SEXT816(v5) * SEXT416(0x400)) goto label_1f6d0;
label_1f746:
      v14 = (long)v13;
      if (v5 > -1) goto label_1f6ec;
label_1f757:
      v15 = 1;
      v5 = -0x8000000000000000;
      goto label_1f69b;
    default:
label_1f4e2:
      *a3 = v5;
      return v15 | 2;
    case 3:
      v14 = (long)v13;
label_1f7bd:
      v13 = 6;
      v12 = 0;
      v7 = v5;
      do {
        v5 = v7 * v8;
        if (SEXT816(v5) != SEXT816(v7) * SEXT816((long)v8)) {
          v12 = 1;
          v5 = (0 <= v7) ? 0x7fffffffffffffff : -0x8000000000000000; // branch-flip
        }
        v13 -= 1;
        v7 = v5;
      } while (v13);
      break;
    case 5:
    case 0x25:
      v14 = (long)v13;
label_1f6fb:
      v13 = 3;
      v12 = 0;
      v7 = v5;
      do {
        v5 = v7 * v8;
        if (SEXT816(v5) != SEXT816(v7) * SEXT816((long)v8)) {
          v12 = 1;
          v5 = (0 <= v7) ? 0x7fffffffffffffff : -0x8000000000000000; // branch-flip
        }
        v13 -= 1;
        v7 = v5;
      } while (v13);
      break;
    case 9:
    case 0x29:
      v14 = (long)v13;
label_1f6d8:
      v7 = v8 * v5;
      if (SEXT816(v7) == SEXT816((long)v8) * SEXT816(v5)) goto label_1f6d0;
label_1f6e1:
      if (0 > v5) goto label_1f757;
label_1f6ec:
      v15 = 1;
      v5 = 0x7fffffffffffffff;
      goto label_1f69b;
    case 0xb:
    case 0x2b:
      v14 = (long)v13;
label_1f6b8:
      v6 = (long)v8;
      v9 = v5 * v6;
      if (SEXT816(v9) != SEXT816(v5) * SEXT816(v6)) goto label_1f6e1;
      v7 = v6 * v9;
      if (SEXT816(v7) == SEXT816(v6) * SEXT816(v9)) goto label_1f6d0;
      v15 = 1;
      v5 = -0x8000000000000000;
      if (0 <= v9)
        v5 = 0x7fffffffffffffff;
      goto label_1f69b;
    case 0xe:
      v14 = (long)v13;
label_1f77b:
      v13 = 5;
      v12 = 0;
      v7 = v5;
      do {
        v5 = v7 * v8;
        if (SEXT816(v5) != SEXT816(v7) * SEXT816((long)v8)) {
          v12 = 1;
          v5 = (0 <= v7) ? 0x7fffffffffffffff : -0x8000000000000000; // branch-flip
        }
        v13 -= 1;
        v7 = v5;
      } while (v13);
      break;
    case 0x12:
    case 0x32:
      v14 = (long)v13;
label_1f65e:
      v13 = 4;
      v12 = 0;
      v7 = v5;
      do {
        v5 = v7 * v8;
        if (SEXT816(v5) != SEXT816(v7) * SEXT816((long)v8)) {
          v12 = 1;
          v5 = (0 <= v7) ? 0x7fffffffffffffff : -0x8000000000000000; // branch-flip
        }
        v13 -= 1;
        v7 = v5;
      } while (v13);
      break;
    case 0x17:
      v14 = (long)v13;
label_1f845:
      v13 = 8;
      v12 = 0;
      v7 = v5;
      do {
        v5 = v7 * v8;
        if (SEXT816(v5) != SEXT816(v7) * SEXT816((long)v8)) {
          v12 = 1;
          v5 = (0 <= v7) ? 0x7fffffffffffffff : -0x8000000000000000; // branch-flip
        }
        v13 -= 1;
        v7 = v5;
      } while (v13);
      break;
    case 0x18:
      v14 = (long)v13;
label_1f805:
      v13 = 7;
      v12 = 0;
      v7 = v5;
      do {
        v5 = v7 * v8;
        if (SEXT816(v5) != SEXT816(v7) * SEXT816((long)v8)) {
          v12 = 1;
          v5 = (0 <= v7) ? 0x7fffffffffffffff : -0x8000000000000000; // branch-flip
        }
        v13 -= 1;
        v7 = v5;
      } while (v13);
      break;
    case 0x20:
      v14 = (long)v13;
label_1f885:
      v7 = v5 * 0x200;
      if (SEXT816(v7) != SEXT816(v5) * SEXT416(0x200)) goto label_1f6e1;
      goto label_1f6d0;
    case 0x21:
      v14 = (long)v13;
      goto label_1f69b;
    case 0x35:
      v7 = v5 * 2;
      v14 = (long)v13;
      if (SEXT816(v7) != SEXT816(v5) * SEXT816(2)) goto label_1f746;
label_1f6d0:
      v5 = v7;
      goto label_1f69b;
    
  }
  v15 |= v12;
label_1f69b:
  *a1 = (long)&v3[v14];
  if (v3[v14])
    v15 |= 2;
label_1f4db:
  *a3 = v5;
  return v15;
}

// Function: sub_1fa50 @ 0x1fa50
bool sub_1fa50(void) // early-return
{
  char *v1; // rax
  
  v1 = getenv("TERM");
  if (!v1)
    return 0;
  return strcmp(v1,"dumb") != 0;
}

// Function: sub_1fa90 @ 0x1fa90
void sub_1fa90(void)
{
  return;
}

// Function: sub_1faa0 @ 0x1faa0
void sub_1faa0(char *a0,unsigned long a1)
{
  __printf_chk(1,a0,a1); // tail-call
}

// Function: sub_1fac0 @ 0x1fac0
void sub_1fac0(char *a0)
{
  fputs(a0,stdout); // tail-call
}

// Function: sub_1fad0 @ 0x1fad0
char * sub_1fad0(char *a0)
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

// Function: sub_1fb30 @ 0x1fb30
void sub_1fb30(char *a0)
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

// Function: sub_1fb60 @ 0x1fb60
unsigned long sub_1fb60(unsigned int *a0) // early-return x2, return-dupe
{
  unsigned int v1;
  int v2; // eax
  unsigned long v3; // rax
  
  v3 = __fpending(a0);
  v1 = *a0;
  v2 = fclose(a0);
  if (v1 & 0x20) {
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

// Function: sub_1fbd0 @ 0x1fbd0
void sub_1fbd0(long a0)
{
  *(unsigned long *)(a0 + 0x10) = 0;
  *(unsigned int *)(a0 + 0x18) = 0x95f616;
}

// Function: sub_1fbf0 @ 0x1fbf0
unsigned long sub_1fbf0(long *a0,long *a1)
{
  unsigned long v1;
  unsigned long v2;
  long v3;
  long v4;
  
  if ((int)a0[3] != 0x95f616)
    __assert_fail("state->magic == 9827862","cycle-check.c",0x3c,"cycle_check"); // no-return
  v2 = a0[2];
  v3 = a1[1];
  if (v2) { // branch-flip
    if ((*a0 == v3) && (*a1 == a0[1]))
      return 1;
    v1 = v2 + 1;
    a0[2] = v1;
    if (v2 & v1)
      return 0;
    if (!v1)
      return 1;
  }
  else {
    a0[2] = 1;
  }
  v4 = *a1;
  *a0 = v3;
  a0[1] = v4;
  return 0;
}

// Function: sub_1fc80 @ 0x1fc80
int sub_1fc80(int a0,int a1,unsigned long a2)
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
    if (0 <= dat_3bc88) { // branch-flip
      v2 = fcntl(a0,0x406,a2 & 0xffffffff);
      if ((0 <= v2) || (*__errno_location() != 0x16)) {
        dat_3bc88 = 1;
        return v2;
      }
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      dat_3bc88 = -1;
    }
    else {
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      if (dat_3bc88 != -1)
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

// Function: sub_1fef0 @ 0x1fef0
void sub_1fef0(char *a0,unsigned int a1,unsigned long a2)
{
  unsigned long v1; // rdx
  
  v1 = 0;
  if (a1 & 0x40)
    v1 = a2 & 0xffffffff;
  sub_1e7a0(open(a0,a1,v1));
}

// Function: sub_1ff70 @ 0x1ff70
bool sub_1ff70(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_20040(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_20000 @ 0x20000
char * sub_20000(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_20040 @ 0x20040
unsigned long sub_20040(int a0,char *a1,unsigned long a2) // return-dupe x2
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

// Function: sub_200f0 @ 0x200f0
void sub_200f0(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_20100 @ 0x20100
void sub_20100(unsigned long a0)
{
  sub_1fc80(a0,0,3); // tail-call
}

// Function: sub_20120 @ 0x20120
void sub_20120(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_2b008); // tail-call
}

// Function: _DT_FINI @ 0x20134
void _DT_FINI(void)
{
  return;
}
