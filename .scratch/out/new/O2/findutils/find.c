// Function: _DT_INIT @ 0x6000
void _DT_INIT(void) // return-dupe
{
  if (!dat_37f38)
    return;
  (*dat_37f38)();
}

// Function: sub_6020 @ 0x6020
void sub_6020(void)
{
  (*dat_37aa0)(); // jump-as-call
}

// Function: free @ 0x6920
void free(void *a0)
{
  (*dat_37f70)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x6930
void __cxa_finalize(void)
{
  (*dat_37f88)(); // jump-as-call
}

// Function: strtod @ 0x6940
void strtod(void)
{
  (*dat_37fe8)(); // jump-as-call
}

// Function: ctime @ 0x6950
char * ctime(void *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37aa8)(); // jump-as-call
  return v1;
}

// Function: setmntent @ 0x6960
void * setmntent(char *a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_37ab0)(); // jump-as-call
  return v1;
}

// Function: chdir @ 0x6970
int chdir(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_37ab8)(); // jump-as-call
  return v1;
}

// Function: fileno @ 0x6980
int fileno(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_37ac0)(); // jump-as-call
  return v1;
}

// Function: strtoumax @ 0x6990
void strtoumax(void)
{
  (*dat_37ac8)(); // jump-as-call
}

// Function: endmntent @ 0x69a0
int endmntent(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_37ad0)(); // jump-as-call
  return v1;
}

// Function: __getdelim @ 0x69b0
long __getdelim(char **a0,void *a1,int a2,FILE *a3)
{
  long v1; // rax
  
  v1 = (*dat_37ad8)(); // jump-as-call
  return v1;
}

// Function: mktime @ 0x69c0
long mktime(tm *a0)
{
  long v1; // rax
  
  v1 = (*dat_37ae0)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x69d0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_37ae8)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x69e0
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_37af0)(); // jump-as-call
  return v1;
}

// Function: getgrnam @ 0x69f0
group * getgrnam(char *a0)
{
  group *v1; // rax
  
  v1 = (group *)(*dat_37af8)(); // jump-as-call
  return v1;
}

// Function: close @ 0x6a00
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_37b00)(); // jump-as-call
  return v1;
}

// Function: hasmntopt @ 0x6a10
char * hasmntopt(void *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37b08)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x6a20
void abort(void)
{
  (*dat_37b10)(); // jump-as-call
}

// Function: memchr @ 0x6a30
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_37b18)(); // jump-as-call
  return v1;
}

// Function: fstatfs @ 0x6a40
int fstatfs(int a0,statfs *a1)
{
  int v1; // eax
  
  v1 = (*dat_37b20)(); // jump-as-call
  return v1;
}

// Function: clock_gettime @ 0x6a50
int clock_gettime(int a0,timespec *a1)
{
  int v1; // eax
  
  v1 = (*dat_37b28)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x6a60
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37b30)(); // jump-as-call
  return v1;
}

// Function: __fprintf_chk @ 0x6a70
int __fprintf_chk(FILE *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_37b38)(); // jump-as-call
  return v1;
}

// Function: isatty @ 0x6a80
int isatty(int a0)
{
  int v1; // eax
  
  v1 = (*dat_37b40)(); // jump-as-call
  return v1;
}

// Function: uname @ 0x6a90
int uname(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_37b48)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x6aa0
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37b50)(); // jump-as-call
  return v1;
}

// Function: __isoc99_sscanf @ 0x6ab0
int __isoc99_sscanf(char *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_37b58)(); // jump-as-call
  return v1;
}

// Function: execvp @ 0x6ac0
int execvp(char *a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_37b60)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x6ad0
void exit(int a0)
{
  (*dat_37b68)(); // jump-as-call
}

// Function: __assert_fail @ 0x6ae0
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_37b70)(); // jump-as-call
}

// Function: fstat @ 0x6af0
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_37b78)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x6b00
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_37b80)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x6b10
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37b88)(); // jump-as-call
  return v1;
}

// Function: gettimeofday @ 0x6b20
int gettimeofday(timeval *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_37b90)(); // jump-as-call
  return v1;
}

// Function: openat @ 0x6b30
int openat(int a0,char *a1,int a2,...)
{
  int v1; // eax
  
  v1 = (*dat_37b98)(); // jump-as-call
  return v1;
}

// Function: strpbrk @ 0x6b40
char * strpbrk(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37ba0)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x6b50
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_37ba8)(); // jump-as-call
  return v1;
}

// Function: getmntent @ 0x6b60
void * getmntent(void *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_37bb0)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x6b70
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_37bb8)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x6b80
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_37bc0)(); // jump-as-call
  return v1;
}

// Function: freecon @ 0x6b90
void freecon(char *a0)
{
  (*dat_37bc8)(); // jump-as-call
}

// Function: fopen @ 0x6ba0
FILE * fopen(char *a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_37bd0)(); // jump-as-call
  return v1;
}

// Function: fchdir @ 0x6bb0
int fchdir(int a0)
{
  int v1; // eax
  
  v1 = (*dat_37bd8)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x6bc0
void _exit(int a0)
{
  (*dat_37be0)(); // jump-as-call
}

// Function: __memcpy_chk @ 0x6bd0
void * __memcpy_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_37be8)(); // jump-as-call
  return v1;
}

// Function: __cxa_atexit @ 0x6be0
void __cxa_atexit(void)
{
  (*dat_37bf0)(); // jump-as-call
}

// Function: sysconf @ 0x6bf0
long sysconf(int a0)
{
  long v1; // rax
  
  v1 = (*dat_37bf8)(); // jump-as-call
  return v1;
}

// Function: fgetfilecon @ 0x6c00
int fgetfilecon(int a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_37c00)(); // jump-as-call
  return v1;
}

// Function: tzset @ 0x6c10
void tzset(void)
{
  (*dat_37c08)(); // jump-as-call
}

// Function: getpwuid @ 0x6c20
passwd * getpwuid(unsigned int a0)
{
  passwd *v1; // rax
  
  v1 = (passwd *)(*dat_37c10)(); // jump-as-call
  return v1;
}

// Function: lsetfilecon @ 0x6c30
int lsetfilecon(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_37c18)(); // jump-as-call
  return v1;
}

// Function: re_compile_pattern @ 0x6c40
char * re_compile_pattern(char *a0,unsigned long a1,re_pattern_buffer *a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37c20)(); // jump-as-call
  return v1;
}

// Function: strnlen @ 0x6c50
unsigned long strnlen(char *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_37c28)(); // jump-as-call
  return v1;
}

// Function: is_selinux_enabled @ 0x6c60
int is_selinux_enabled(void)
{
  int v1; // eax
  
  v1 = (*dat_37c30)(); // jump-as-call
  return v1;
}

// Function: fnmatch @ 0x6c70
int fnmatch(char *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_37c38)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x6c80
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_37c40)(); // jump-as-call
  return v1;
}

// Function: ferror @ 0x6c90
int ferror(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_37c48)(); // jump-as-call
  return v1;
}

// Function: opendir @ 0x6ca0
DIR * opendir(char *a0)
{
  DIR *v1; // rax
  
  v1 = (DIR *)(*dat_37c50)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x6cb0
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_37c58)(); // jump-as-call
  return v1;
}

// Function: __vfprintf_chk @ 0x6cc0
int __vfprintf_chk(FILE *a0,int a1,char *a2,void *a3)
{
  int v1; // eax
  
  v1 = (*dat_37c60)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x6cd0
int __freading(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_37c68)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x6ce0
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_37c70)(); // jump-as-call
  return v1;
}

// Function: readdir @ 0x6cf0
dirent * readdir(DIR *a0)
{
  dirent *v1; // rax
  
  v1 = (dirent *)(*dat_37c78)(); // jump-as-call
  return v1;
}

// Function: sprintf @ 0x6d00
int sprintf(char *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_37c80)(); // jump-as-call
  return v1;
}

// Function: fdopen @ 0x6d10
FILE * fdopen(int a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_37c88)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x6d20
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37c90)(); // jump-as-call
  return v1;
}

// Function: strtok_r @ 0x6d30
char * strtok_r(char *a0,char *a1,char **a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37c98)(); // jump-as-call
  return v1;
}

// Function: timegm @ 0x6d40
long timegm(tm *a0)
{
  long v1; // rax
  
  v1 = (*dat_37ca0)(); // jump-as-call
  return v1;
}

// Function: poll @ 0x6d50
void poll(void)
{
  (*dat_37ca8)(); // jump-as-call
}

// Function: gmtime_r @ 0x6d60
tm * gmtime_r(void *a0,tm *a1)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_37cb0)(); // jump-as-call
  return v1;
}

// Function: strstr @ 0x6d70
char * strstr(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37cb8)(); // jump-as-call
  return v1;
}

// Function: __fpending @ 0x6d80
unsigned long __fpending(FILE *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_37cc0)(); // jump-as-call
  return v1;
}

// Function: abs @ 0x6d90
int abs(int a0)
{
  int v1; // eax
  
  v1 = (*dat_37cc8)(); // jump-as-call
  return v1;
}

// Function: strcat @ 0x6da0
char * strcat(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37cd0)(); // jump-as-call
  return v1;
}

// Function: memrchr @ 0x6db0
void * memrchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_37cd8)(); // jump-as-call
  return v1;
}

// Function: fputs @ 0x6dc0
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_37ce0)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x6dd0
void lseek(void)
{
  (*dat_37ce8)(); // jump-as-call
}

// Function: strtol @ 0x6de0
long strtol(char *a0,char **a1,int a2)
{
  long v1; // rax
  
  v1 = (*dat_37cf0)(); // jump-as-call
  return v1;
}

// Function: dirfd @ 0x6df0
int dirfd(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_37cf8)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x6e00
int iswcntrl(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_37d00)(); // jump-as-call
  return v1;
}

// Function: endpwent @ 0x6e10
void endpwent(void)
{
  (*dat_37d08)(); // jump-as-call
}

// Function: __strcpy_chk @ 0x6e20
char * __strcpy_chk(char *a0,char *a1,unsigned long a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37d10)(); // jump-as-call
  return v1;
}

// Function: putc @ 0x6e30
int putc(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_37d18)(); // jump-as-call
  return v1;
}

// Function: re_set_syntax @ 0x6e40
void re_set_syntax(void)
{
  (*dat_37d20)(); // jump-as-call
}

// Function: signal @ 0x6e50
void signal(void)
{
  (*dat_37d28)(); // jump-as-call
}

// Function: strspn @ 0x6e60
unsigned long strspn(char *a0,char *a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_37d30)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x6e70
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_37d38)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x6e80
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37d40)(); // jump-as-call
  return v1;
}

// Function: waitpid @ 0x6e90
int waitpid(int a0,int *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_37d48)(); // jump-as-call
  return v1;
}

// Function: setenv @ 0x6ea0
int setenv(char *a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_37d50)(); // jump-as-call
  return v1;
}

// Function: getenv @ 0x6eb0
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37d58)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x6ec0
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_37d60)(); // jump-as-call
  return v1;
}

// Function: __errno_location @ 0x6ed0
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_37d68)(); // jump-as-call
  return v1;
}

// Function: qsort @ 0x6ee0
void qsort(void)
{
  (*dat_37d70)(); // jump-as-call
}

// Function: strdup @ 0x6ef0
char * strdup(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37d78)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x6f00
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37d80)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x6f10
void __stack_chk_fail(void)
{
  (*dat_37d88)(); // jump-as-call
}

// Function: strcmp @ 0x6f20
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_37d90)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x6f30
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_37d98)(); // jump-as-call
  return v1;
}

// Function: getcwd @ 0x6f40
char * getcwd(char *a0,unsigned long a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37da0)(); // jump-as-call
  return v1;
}

// Function: getgrgid @ 0x6f50
group * getgrgid(unsigned int a0)
{
  group *v1; // rax
  
  v1 = (group *)(*dat_37da8)(); // jump-as-call
  return v1;
}

// Function: localeconv @ 0x6f60
lconv * localeconv(void)
{
  lconv *v1; // rax
  
  v1 = (lconv *)(*dat_37db0)(); // jump-as-call
  return v1;
}

// Function: strcpy @ 0x6f70
char * strcpy(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37db8)(); // jump-as-call
  return v1;
}

// Function: setfilecon @ 0x6f80
int setfilecon(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_37dc0)(); // jump-as-call
  return v1;
}

// Function: endgrent @ 0x6f90
void endgrent(void)
{
  (*dat_37dc8)(); // jump-as-call
}

// Function: rpmatch @ 0x6fa0
int rpmatch(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_37dd0)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x6fb0
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_37dd8)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x6fc0
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_37de0)(); // jump-as-call
  return v1;
}

// Function: feof @ 0x6fd0
int feof(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_37de8)(); // jump-as-call
  return v1;
}

// Function: faccessat @ 0x6fe0
int faccessat(int a0,char *a1,int a2,int a3)
{
  int v1; // eax
  
  v1 = (*dat_37df0)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x6ff0
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_37df8)(); // jump-as-call
  return v1;
}

// Function: strncpy @ 0x7000
char * strncpy(char *a0,char *a1,unsigned long a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37e00)(); // jump-as-call
  return v1;
}

// Function: getfilecon @ 0x7010
int getfilecon(char *a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_37e08)(); // jump-as-call
  return v1;
}

// Function: getrlimit @ 0x7020
void getrlimit(void)
{
  (*dat_37e10)(); // jump-as-call
}

// Function: difftime @ 0x7030
void difftime(void)
{
  (*dat_37e18)(); // jump-as-call
}

// Function: localtime_r @ 0x7040
tm * localtime_r(void *a0,tm *a1)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_37e20)(); // jump-as-call
  return v1;
}

// Function: fseeko @ 0x7050
int fseeko(FILE *a0,long a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_37e28)(); // jump-as-call
  return v1;
}

// Function: unsetenv @ 0x7060
int unsetenv(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_37e30)(); // jump-as-call
  return v1;
}

// Function: closedir @ 0x7070
int closedir(DIR *a0)
{
  int v1; // eax
  
  v1 = (*dat_37e38)(); // jump-as-call
  return v1;
}

// Function: __sprintf_chk @ 0x7080
int __sprintf_chk(char *a0,int a1,unsigned long a2,char *a3,...)
{
  int v1; // eax
  
  v1 = (*dat_37e40)(); // jump-as-call
  return v1;
}

// Function: __snprintf_chk @ 0x7090
int __snprintf_chk(char *a0,unsigned long a1,int a2,unsigned long a3,char *a4,...)
{
  int v1; // eax
  
  v1 = (*dat_37e48)(); // jump-as-call
  return v1;
}

// Function: access @ 0x70a0
int access(char *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_37e50)(); // jump-as-call
  return v1;
}

// Function: fork @ 0x70b0
int fork(void)
{
  int v1; // eax
  
  v1 = (*dat_37e58)(); // jump-as-call
  return v1;
}

// Function: modf @ 0x70c0
void modf(void)
{
  (*dat_37e60)(); // jump-as-call
}

// Function: unlinkat @ 0x70d0
int unlinkat(int a0,char *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_37e68)(); // jump-as-call
  return v1;
}

// Function: fwrite @ 0x70e0
unsigned long fwrite(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_37e70)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x70f0
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_37e78)(); // jump-as-call
  return v1;
}

// Function: lstat @ 0x7100
int lstat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_37e80)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x7110
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37e88)(); // jump-as-call
  return v1;
}

// Function: error @ 0x7120
void error(int a0,int a1,char *a2,...)
{
  (*dat_37e90)(); // jump-as-call
}

// Function: localtime @ 0x7130
tm * localtime(void *a0)
{
  tm *v1; // rax
  
  v1 = (tm *)(*dat_37e98)(); // jump-as-call
  return v1;
}

// Function: getdelim @ 0x7140
long getdelim(char **a0,void *a1,int a2,FILE *a3)
{
  long v1; // rax
  
  v1 = (*dat_37ea0)(); // jump-as-call
  return v1;
}

// Function: strftime @ 0x7150
unsigned long strftime(char *a0,unsigned long a1,char *a2,tm *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_37ea8)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x7160
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_37eb0)(); // jump-as-call
  return v1;
}

// Function: strtoul @ 0x7170
unsigned long strtoul(char *a0,char **a1,int a2)
{
  unsigned long v1; // rax
  
  v1 = (*dat_37eb8)(); // jump-as-call
  return v1;
}

// Function: fstatat @ 0x7180
int fstatat(int a0,char *a1,stat *a2,int a3)
{
  int v1; // eax
  
  v1 = (*dat_37ec0)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x7190
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_37ec8)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x71a0
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_37ed0)(); // jump-as-call
  return v1;
}

// Function: open @ 0x71b0
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_37ed8)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x71c0
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_37ee0)(); // jump-as-call
  return v1;
}

// Function: getpwnam @ 0x71d0
passwd * getpwnam(char *a0)
{
  passwd *v1; // rax
  
  v1 = (passwd *)(*dat_37ee8)(); // jump-as-call
  return v1;
}

// Function: lgetfilecon @ 0x71e0
int lgetfilecon(char *a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_37ef0)(); // jump-as-call
  return v1;
}

// Function: re_match @ 0x71f0
int re_match(re_pattern_buffer *a0,char *a1,int a2,int a3,void *a4)
{
  int v1; // eax
  
  v1 = (*dat_37ef8)(); // jump-as-call
  return v1;
}

// Function: strndup @ 0x7200
char * strndup(char *a0,unsigned long a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_37f00)(); // jump-as-call
  return v1;
}

// Function: time @ 0x7210
void time(void)
{
  (*dat_37f08)(); // jump-as-call
}

// Function: fflush @ 0x7220
int fflush(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_37f10)(); // jump-as-call
  return v1;
}

// Function: fdopendir @ 0x7230
DIR * fdopendir(int a0)
{
  DIR *v1; // rax
  
  v1 = (DIR *)(*dat_37f18)(); // jump-as-call
  return v1;
}

// Function: main @ 0x7240
int main(int argc,char **argv,char **envp)
{
  int v1; // eax
  int v2; // eax
  int v3; // eax
  void *v4; // rax
  char *v5;
  
  v5 = *argv;
  if (!v5)
    v5 = "find";
  sub_21e40(v5);
  sub_da20();
  dat_38618 = 0;
  dat_38604 = 0;
  dat_38608 = 0;
  dat_385f8 = 0xffffff9c;
  if (sub_16220())
    sub_15fd0();
  dat_38610 = sub_ea90("w");
  if (dat_38610) {
    sub_e630((struct_49 *)0x38620);
    setlocale(6,"");
    bindtextdomain("findutils","/usr/local/share/locale");
    textdomain("findutils");
    if (!sub_27a00(sub_17a40)) {
      v1 = sub_e180(argc,argv);
      if (dat_38660 & 2)
        dat_38670 = sub_dec0;
      if (dat_38660 & 0x80)
        __fprintf_chk(stderr,1,"cur_day_start = %s",ctime((void *)0x38648));
      v4 = (void *)sub_c9f0(argc,argv,v1);
      if (sub_8070(argc - v1,&argv[v1])) {
        sub_9e00(v4);
        sub_dab0();
      }
      return dat_38604;
    }
    v5 = dcgettext(NULL,"The atexit library function failed",5);
    error(1,*__errno_location(),v5);
    return v2;
  }
  v5 = dcgettext(NULL,"Failed to initialize shared-file hash table",5);
  error(1,*__errno_location(),v5);
  return v3;
}

// Function: sub_7420 @ 0x7420
void sub_7420(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_37fe0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_7450 @ 0x7450
void sub_7450(void)
{
  return;
}

// Function: _FINI_0 @ 0x74c0
void _FINI_0(void)
{
  if (!dat_38590) {
    if (dat_37f88)
      __cxa_finalize(dat_38008);
    sub_7450();
    dat_38590 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x7500
void _INIT_0(void)
{
  return;
}

// Function: sub_7510 @ 0x7510
char * sub_7510(int a0)
{
  switch(a0) {
    default:
      __sprintf_chk((char *)0x38598,1,0xe,"[%d]",a0);
      return (char *)0x38598;
    case 1:
      return "FTS_D";
    case 2:
      return "FTS_DC";
    case 3:
      return "FTS_DEFAULT";
    case 4:
      return "FTS_DNR";
    case 5:
      return "FTS_DOT";
    case 6:
      return "FTS_DP";
    case 7:
      return "FTS_ERR";
    case 8:
      return "FTS_F";
    case 9:
      return "FTS_INIT";
    case 10:
      return "FTS_NS";
    case 0xb:
      return "FTS_NSOK";
    case 0xc:
      return "FTS_SL";
    case 0xd:
      return "FTS_SLNONE";
    case 0xe:
      return "FTS_W";
    
  }
}

// Function: sub_7640 @ 0x7640
void sub_7640(void)
{
  __assert_fail("dir_fd == AT_FDCWD || dir_fd >= 0","ftsfind.c",0x6b,"inside_dir"); // no-return
}

// Function: sub_7670 @ 0x7670
void sub_7670(struct_4 *a0,struct_3 *a1) // return-dupe
{
  char v1;
  char *v10;
  unsigned int v11; // edx
  long v12;
  unsigned int v13;
  long v14; // stack - 0x168
  unsigned int v15; // stack - 0x16c
  char v16 [152];
  long v17; // stack - 0x160
  long v18; // stack - 0x158
  long v19; // stack - 0x150
  unsigned char v2;
  long v20; // stack - 0x148
  long v21; // stack - 0x140
  long v22; // stack - 0x138
  long v23; // stack - 0x130
  long v24; // stack - 0x128
  long v25; // stack - 0x120
  long v26; // stack - 0x118
  long v27; // stack - 0x110
  long v28; // stack - 0x108
  long v29; // stack - 0x100
  unsigned char v3;
  long v30; // stack - 0xf8
  long v31; // stack - 0xf0
  long v32; // stack - 0xe8
  long v33; // stack - 0xe0
  short v4;
  int v5;
  struct_2 *v6; // rax
  char *v7; // rax
  char *v8; // rax
  char *v9;
  
  if (dat_38660 & 4) {
    v9 = (char *)sub_239f0(2,dat_3868c,(char *)a1->field_0x30);
    v10 = (char *)sub_239f0(1,dat_3868c,a1->field_0x38);
    v5 = dat_38014;
    v12 = a1->field_0x58;
    v7 = (char *)sub_7510(a1->field_0x68);
    v8 = (char *)sub_239f0(0,dat_3868c,a1->field_0x38);
    __fprintf_chk(stderr,1,"consider_visiting (early): %s: fts_info=%-6s, fts_level=%2d, prev_depth=%d fts_path=%s, fts_accpath=%s\n",v8,v7,(int)v12,v5,v10,v9);
  }
  v4 = a1->field_0x68;
  if (v4 != 6) { // branch-flip
    v12 = a1->field_0x58;
    if ((v12 <= dat_38014) && (v13 = dat_38018, v12)) {
label_76e0:
      v5 = a0->field_0x2c;
      if (v13 & 0x200) goto label_76ea;
label_770b:
      v12 = a1->field_0x58;
      v4 = a1->field_0x68;
    }
    else if (dat_38018 & 0x200) goto label_7908;
label_7713:
    v17 = a1->field_0x78;
    dat_38014 = (unsigned int)v12;
    if (v4 == 7) {
      sub_e8c0(a1->field_0x40,a1->field_0x38); // return-dupe
      return;
    }
    if (v4 != 4) { // branch-flip
      if (v4 == 2) {
        if ((a1->field_0x88 & 0xf000) != 0xa000) {
          v12 = *(long *)(a1->field_0x0 + 0x48);
          v9 = *(char **)(a1->field_0x0 + 0x38);
          if (v12) { // branch-flip
            v10 = &v9[v12];
            v1 = *v10;
            *v10 = '\0';
            v9 = (char *)sub_239f0(1,dat_3868c,v9);
            *v10 = v1;
          }
          else {
            v9 = (char *)sub_239f0(1,dat_3868c,"");
          }
          v10 = (char *)sub_e8a0(0,a1->field_0x38);
          error(0,0,dcgettext(NULL,"File system loop detected; %s is part of the same file system loop as %s.",5),v10,v9);
          dat_38604 = 1;
          return;
        }
        v9 = (char *)sub_e8a0(0,a1->field_0x38);
        error(0,0,dcgettext(NULL,"Symbolic link %s is part of a loop in the directory hierarchy; we have already visited the directory to which it points.",5),v9);
        dat_38604 = 1;
        return;
      }
      if (v4 != 0xd) { // branch-flip
        if (v4 == 10) {
          if (!v12) {
            sub_e8c0(a1->field_0x40,a1->field_0x38);
            return;
          }
          v5 = (*dat_38670)(a1->field_0x30,v16);
          if ((v5) && (*__errno_location() == 0x28)) {
            sub_e8c0(0x28,a1->field_0x38); // return-dupe
            return;
          }
          sub_e8c0(a1->field_0x40,a1->field_0x38);
          v4 = a1->field_0x68;
        }
      }
      else {
        v5 = (*dat_38670)(a1->field_0x30,v16);
        if ((v5) && (*__errno_location() == 0x28)) {
          sub_e8c0(0x28,a1->field_0x38);
          return;
        }
        v4 = a1->field_0x68;
      }
    }
    else {
      sub_e8c0(a1->field_0x40,a1->field_0x38);
      if (dat_38620)
        return;
      v4 = a1->field_0x68;
    }
    if (2 <= (unsigned short)(v4 - 10U)) goto label_7761;
    if (dat_385e4)
      __assert_fail("!state.have_stat","ftsfind.c",0x17c,"consider_visiting"); // no-return
    if ((v4 != 0xb) && (dat_385e8))
      __assert_fail("ent->fts_info == FTS_NSOK || state.type == 0","ftsfind.c",0x17d,"consider_visiting"); // no-return
    v15 = dat_385e8;
    v13 = dat_385e8;
label_7a35:
    dat_385e0 = (unsigned int)a1->field_0x58;
    if (v13) goto label_7808;
    v4 = a1->field_0x68;
label_7a4b:
    if (((unsigned short)(v4 - 1U) < 2) || (v4 == 6)) goto label_7847;
    v5 = 0;
    v13 = 0;
    if (0 <= dat_38624) goto label_7862;
label_7882:
    v11 = (unsigned int)CONCAT71(dat_38661,dat_38660);
    if (a1->field_0x58 < (long)dat_38628) goto label_7934;
    if (dat_38660 & 4) goto label_7bf5;
  }
  else {
    if (dat_38018 & 0x200) {
label_7908:
      if (0 <= dat_38010) {
        close(dat_38010);
        dat_38010 = -1;
        v13 = dat_38018;
        goto label_76e0;
      }
      v5 = a0->field_0x2c;
label_76ea:
      if ((v5 != -100) && (v5 < 0))
        sub_7640(); // no-return
      dat_385f8 = v5;
      if (dat_38010 < 0) {
        if (v5 == -100) {
          dat_38010 = -100;
          v12 = a1->field_0x58;
          v4 = a1->field_0x68;
          goto label_7713;
        }
        dat_38010 = sub_17a00();
      }
      goto label_770b;
    }
    dat_38014 = (unsigned int)a1->field_0x58;
label_7761:
    v14 = a1->field_0x70;
    v17 = a1->field_0x78;
    v18 = a1->field_0x80;
    v19 = a1->field_0x88;
    dat_385e4 = 0x101;
    v20 = a1->field_0x90;
    v21 = a1->field_0x98;
    v22 = a1->field_0xa0;
    v23 = a1->field_0xa8;
    v24 = a1->field_0xb0;
    v25 = a1->field_0xb8;
    v26 = a1->field_0xc0;
    v27 = a1->field_0xc8;
    v28 = a1->field_0xd0;
    v29 = a1->field_0xd8;
    v30 = a1->field_0xe0;
    v31 = a1->field_0xe8;
    v32 = a1->field_0xf0;
    v33 = a1->field_0xf8;
    v15 = (unsigned int)v19;
    dat_385e8 = (unsigned int)v19;
    if (!(unsigned int)v19) {
      v9 = (char *)sub_239f0(0,dat_3868c,a1->field_0x38);
      error(0,0,dcgettext(NULL,"WARNING: file %s appears to have mode 0000",5),v9);
      v13 = v15;
      goto label_7a35;
    }
    dat_385e0 = (unsigned int)a1->field_0x58;
label_7808:
    v9 = a1->field_0x38;
    if (!sub_dfe0(&v15,v9,&a1[1],&v14,0))
      return;
    v4 = a1->field_0x68;
    if ((v15 & 0xf000) != 0x4000) goto label_7a4b;
label_7847:
    if (v4 == 0xb) {
      sub_1a750(a0,a1,1);
      return;
    }
    v5 = 1;
    if (0 <= dat_38624) { // branch-flip
label_7862:
      if (a1->field_0x58 < (long)dat_38624) goto label_786c;
      sub_1a750(a0,a1,4);
      v13 = (unsigned int)((long)dat_38624 < a1->field_0x58);
      v4 = a1->field_0x68;
    }
    else {
label_786c:
      v13 = 0;
    }
    if (v4 == 1) {
      if (!dat_38620) goto label_792d;
      goto label_7882;
    }
    if ((v4 != 6) || (!dat_38620)) goto label_7882;
label_792d:
    v11 = (unsigned int)CONCAT71(dat_38661,dat_38660);
label_7934:
    if (!(v11 & 4)) goto label_793d;
    v13 = 1;
label_7bf5:
    v3 = dat_385e5;
    v2 = dat_385e4;
    v9 = (char *)sub_7510((unsigned short)v4);
    v10 = (char *)sub_239f0(0,dat_3868c,a1->field_0x38);
    __fprintf_chk(stderr,1,"consider_visiting (late): %s: fts_info=%-6s, isdir=%d ignore=%d have_stat=%d have_type=%d \n",v10,v9,v5,v13,(unsigned int)v2,(unsigned int)v3);
    v4 = a1->field_0x68;
  }
  if (!v13) {
    dat_385f0 = a1->field_0x30;
    dat_385e4 = CONCAT11(dat_385e5,2 <= (unsigned short)(v4 - 10U));
    dat_385f8 = a0->field_0x2c;
    v6 = (struct_2 *)sub_b580();
    sub_e7b0(a1->field_0x38,&v14,v6);
    if (dat_38600)
      sub_1a750(a0,a1,4);
  }
label_793d:
  if (a1->field_0x68 == 6) {
    dat_38600 = 0;
    return;
  }
}

// Function: sub_7e10 @ 0x7e10
unsigned long sub_7e10(char *a0) // return-dupe
{
  int v1;
  unsigned long v10; // stack - 0x40
  struct_16 *v2; // rax
  int *v3; // rax
  long v4; // rax
  char *v5; // rax
  char *v6; // rax
  char *v7; // stack - 0x48
  unsigned int v8;
  int v9; // r13d
  
  dat_385fc = (unsigned int)strlen(a0);
  if ((dat_38018 & 0x200) && (dat_385f8 = 0xffffff9c, dat_38010 < 0))
    dat_38010 = -100;
  v10 = 0;
  if (dat_38668 != 1) { // branch-flip
    if (dat_38668 != 2) { // branch-flip
      v8 = dat_38018;
      if (!dat_38668) {
        dat_38018 |= 0x10;
        v8 = dat_38018;
      }
    }
    else {
      dat_38018 |= 0x11;
      v8 = dat_38018;
    }
  }
  else {
    dat_38018 |= 3;
    v8 = dat_38018;
  }
  if (dat_3862d) {
    dat_38018 = v8 | 0x40;
    v8 = dat_38018;
  }
  v9 = -0x80000000;
  v7 = a0;
  v2 = (struct_16 *)sub_199d0(&v7,v8,0);
  v3 = __errno_location();
  if (!v2) {
    v5 = (char *)sub_e8a0(0,a0);
    v6 = dcgettext(NULL,"cannot search %s",5);
    error(0,*v3,v6,v5);
    dat_38604 = 1;
    return 1;
  }
  while( true ) {
    *v3 = 0;
    v4 = sub_19ff0(v2);
    if (!v4) break;
    v1 = (int)*(unsigned long *)(v4 + 0x58);
    if ((dat_38608) && (v9 != v1)) {
      sub_d9f0();
      v1 = *(int *)(v4 + 0x58);
    }
    v9 = v1;
    dat_385e8 = *(int *)(v4 + 0x88);
    dat_38618 = 0;
    dat_385e4 = 0;
    dat_385e5 = dat_385e8 != 0;
    sub_7670(v2);
  }
  if (*v3) { // branch-flip
    a0 = (char *)sub_e8a0(0,a0);
    v5 = "failed to read file names from file system at or below %s";
  }
  else {
    if (!sub_19e00(v2))
      return 1;
    v5 = dcgettext(NULL,"failed to restore working directory after searching %s",5);
  }
  error(0,*v3,v5,a0);
  dat_38604 = 1;
  return 0;
}

// Function: sub_8070 @ 0x8070
unsigned long sub_8070(int a0,unsigned long *a1)
{
  FILE *v1;
  unsigned long v10; // rax
  int v11; // stack - 0x16c
  stat v12; // stack - 0x168
  unsigned short v13; // stack - 0x42
  stat v14; // stack - 0xd8
  unsigned long v15; // rax
  FILE *v16; // stack - 0x180
  int v2; // eax
  char *v3;
  struct_28 *v4; // rax
  char *v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  if ((1 <= a0) && (v3 = (char *)*a1, !sub_e120(v3,1))) {
    v16 = dat_38690;
    if (dat_38690) {
      v3 = (char *)sub_e8a0(0,(char *)*a1);
      error(0,0,dcgettext(NULL,"extra operand %s",5),v3);
      error(1,0,"%s",dcgettext(NULL,"file operands cannot be combined with -files0-from",5));
      return v10;
    }
    v3 = NULL;
    v4 = (struct_28 *)sub_17670(a1);
  }
  else {
    v1 = dat_38690;
    if (!dat_38690) {
      v13 = 0x2e;
      return (unsigned long)(unsigned int)sub_7e10(&v13);
    }
    if (strcmp((char *)dat_38690,"-")) { // branch-flip
      v3 = (char *)sub_e8a0(0,v1);
      v16 = fopen((char *)dat_38690,"r");
      if (!v16) {
        v5 = dcgettext(NULL,"cannot open %s for reading",5);
        error(1,*__errno_location(),v5,v3);
        return v8;
      }
      v2 = fileno(v16);
      if (v2 < 0)
        __assert_fail("fd >= 0","ftsfind.c",0x262,"process_all_startpoints"); // no-return
      if ((((dat_38698) && (!fstat(v2,&v12))) && (!fstat(0,&v14))) && ((v12._8_8_ == v14._8_8_ && (v12._0_8_ == v14._0_8_)))) {
        error(1,0,"%s: %s\n",dcgettext(NULL,"option -files0-from: standard input must not refer to the same file when combined with -ok, -okdir",5),v3);
        return v6;
      }
      sub_17990(v2,1);
    }
    else {
      if (dat_38698) {
        error(1,0,"%s\n",dcgettext(NULL,"option -files0-from reading from standard input cannot be combined with -ok, -okdir",5));
        return v9;
      }
      v3 = (char *)sub_e8a0(0,dcgettext(NULL,"(standard input)",5));
      v16 = stdin;
    }
    v4 = (struct_28 *)sub_176a0(v16);
  }
  if (!v4)
    sub_25f50(); // no-return, return-dupe
  v15 = 1;
  do {
    while( true ) {
      v5 = (char *)sub_176e0(v4,&v11);
      if (!v5) {
        if (v11 == 3) {
          sub_25f50();
        }
        if (v11 == 4) {
          v5 = dcgettext(NULL,"%s: read error",5);
          error(0,*__errno_location(),v5,v3);
          dat_38604 = 1;
          sub_177a0(v4);
          return 0;
        }
        if (v11 != 2)
          __assert_fail("!\"unexpected error code from argv_iter\"","ftsfind.c",0x2a3,"process_all_startpoints"); // no-return
        goto label_8176;
      }
      if (*v5) break;
      if (dat_38690) { // branch-flip
        v15 = sub_17780(v4);
        error(0,0,"%s:%lu: %s",v3,v15,dcgettext(NULL,"invalid zero-length file name",5));
      }
      else {
        error(0,2,"%s",(char *)sub_e8a0(0,v5));
      }
      dat_38604 = 1;
      v15 = 0;
    }
    if ((!dat_38690) && (sub_e120(v5,1))) goto label_8176;
    dat_385fc = (unsigned int)strlen(v5);
  } while (sub_7e10(v5));
  v15 = 0;
label_8176:
  sub_177a0(v4);
  if ((((char)v15) && (dat_38690)) && ((ferror(v16) || (sub_17d40(v16))))) {
    error(1,0,dcgettext(NULL,"error reading %s",5),v3);
    return v7;
  }
  return v15;
}

// Function: sub_8530 @ 0x8530
unsigned long sub_8530(unsigned int *a0)
{
  *a0 = dat_38018;
  return 1;
}

// Function: sub_8550 @ 0x8550
void sub_8550(char *a0,char *a1)
{
  char *v1; // rax
  
  sub_180c0(stdout);
  v1 = dcgettext(NULL,"< %s ... %s > ? ",5);
  if (0 <= __fprintf_chk(stderr,1,v1,a0,a1)) {
    sub_180c0(stderr);
    sub_268c0(); // tail-call
    return;
  }
  v1 = dcgettext(NULL,"Failed to write prompt for -ok",5);
  error(1,*__errno_location(),v1);
}

// Function: sub_85e0 @ 0x85e0
void sub_85e0(void)
{
  sub_dab0();
  exit(dat_38604); // no-return
}

// Function: sub_8600 @ 0x8600
unsigned long sub_8600(unsigned long a0,void *a1)
{
  if (dat_38620) {
    if (!dat_385e4)
      __assert_fail("state.have_stat","pred.c",0x355,"pred_prune"); // no-return
    if ((a1) && ((*(unsigned int *)((long)a1 + 0x18) & 0xf000) == 0x4000)) {
      dat_38600 = 1;
      return 1;
    }
  }
  return 1;
}

// Function: sub_8660 @ 0x8660
unsigned long sub_8660(char *a0,void *a1,unsigned int a2)
{
  char *v1;
  char *v2; // rax
  unsigned long v3; // r12
  
  v2 = (char *)sub_17650(dat_385f8,dat_385f0);
  if (v2) { // branch-flip
    v1 = *(char **)((long)a1 + 0x38);
    v3 = CONCAT71((undefined7)((unsigned long)a1 >> 8),fnmatch(v1,v2,(a2 & 0xff) << 4) == 0);
  }
  else {
    v3 = 0;
    sub_e8c0(*__errno_location(),a0);
    dat_38604 = 1;
  }
  free(v2);
  return v3 & 0xffffffff;
}

// Function: sub_86e0 @ 0x86e0
void sub_86e0(FILE *a0,void *a1)
{
  char *v1; // rcx
  char *v2; // r8
  char *v3; // r9
  
  sub_8870(a0,*(long **)((long)a1 + 0x110));
  v3 = "[need inum] ";
  if (!*(char *)((long)a1 + 0x1c))
    v3 = "";
  v2 = "[need type] ";
  if (!*(char *)((long)a1 + 0x1b))
    v2 = "";
  v1 = "[call stat] ";
  if (!*(char *)((long)a1 + 0x1a))
    v1 = "";
  __fprintf_chk(a0,1,"%s%s%s",v1,v2,v3);
  sub_b540(a0,a1);
  __fprintf_chk(a0,1," [est success rate %.4g] ",(double)*(float *)((long)a1 + 0x24));
  if (dat_38660 & 0x40) {
    __fprintf_chk(a0,1,"[real success rate %lu/%lu",*(unsigned long *)((long)a1 + 0x128),*(unsigned long *)((long)a1 + 0x120));
    if (*(unsigned long *)((long)a1 + 0x120)) // branch-flip
      __fprintf_chk(a0,1,"=%.4g] ",(double)*(unsigned long *)((long)a1 + 0x128) / (double)*(unsigned long *)((long)a1 + 0x120));
    else {
      __fprintf_chk(a0,1,"=_] ");
    }
  }
  sub_8870(a0,*(long **)((long)a1 + 0x118)); // tail-call
}

// Function: sub_8870 @ 0x8870
void sub_8870(FILE *a0,long *a1)
{
  if (!a1)
    return;
  while( true ) {
    if (((void *)*a1 != sub_8940) && ((void *)*a1 != sub_8990)) break;
    if (a1[0x22]) goto label_88be;
    a1 = (long *)a1[0x23];
    if (!a1)
      return;
  }
  if ((!a1[0x22]) && (!a1[0x23])) {
    sub_86e0(a0,a1); // tail-call
    return;
  }
label_88be:
  __fprintf_chk(a0,1,"%s"," ( ");
  sub_86e0(a0,a1);
  __fprintf_chk(a0,1,"%s"," ) "); // tail-call
  return;
}

// Function: sub_8940 @ 0x8940
void sub_8940(char *a0,void *a1,void *a2)
{
  struct_2 *v1;
  
  v1 = *(struct_2 **)((long)a2 + 0x110);
  if (v1) {
    if (sub_e7b0(a0,a1,v1))
      return;
  }
  sub_e7b0(a0,a1,*(struct_2 **)((long)a2 + 0x118)); // tail-call
}

// Function: sub_8990 @ 0x8990
void sub_8990(char *a0,void *a1,void *a2)
{
  struct_2 *v1;
  
  v1 = *(struct_2 **)((long)a2 + 0x110);
  if (v1) {
    if (!sub_e7b0(a0,a1,v1))
      return;
  }
  sub_e7b0(a0,a1,*(struct_2 **)((long)a2 + 0x118)); // tail-call
}

// Function: sub_89e0 @ 0x89e0
unsigned long sub_89e0(long a0,long a1,void *a2,int a3) // early-return x3
{
  int v1;
  long v2;
  unsigned long v3; // rax
  double v4; // xmm0_qa
  double v5; // xmm0_qa
  double v6; // xmm0_qa
  double v7; // xmm0_qa
  
  v1 = *(int *)((long)a2 + 0x3c);
  if (v1 == 1) {
    v2 = *(long *)((long)a2 + 0x48);
    if ((a0 == *(long *)((long)a2 + 0x40)) && (v2 == a1))
      return 0;
    v3 = difftime(a0,*(long *)((long)a2 + 0x40));
    return CONCAT71((undefined7)((unsigned long)v3 >> 8),(double)(a1 - v2) * dat_28950 + v4 < 0.0);
  }
  if (v1 == 2) {
    difftime(a0,*(unsigned long *)((long)a2 + 0x40));
    v7 = (double)(a1 - *(long *)((long)a2 + 0x48)) * dat_28950 + v6;
    if (v7 <= dat_28958)
      return 0;
    return (unsigned long)(v7 <= (double)a3);
  }
  if (!v1) {
    v2 = *(long *)((long)a2 + 0x48);
    if ((a0 == *(long *)((long)a2 + 0x40)) && (a1 == v2))
      return 0;
    v3 = difftime(a0,*(long *)((long)a2 + 0x40));
    return CONCAT71((undefined7)((unsigned long)v3 >> 8),0.0 <= (double)(a1 - v2) * dat_28950 + v5);
  }
  __assert_fail("0","pred.c",0x8a,"pred_timewindow"); // no-return
}

// Function: sub_8b10 @ 0x8b10
unsigned long sub_8b10(unsigned long a0,void *a1,void *a2)
{
  long v1;
  long v2;
  
  v1 = *(long *)((long)a1 + 0x50);
  v2 = *(long *)((long)a1 + 0x48);
  return sub_89e0(v2,v1,a2,0x3c); // tail-call
}

// Function: sub_8b30 @ 0x8b30
unsigned long sub_8b30(unsigned long a0,void *a1,void *a2)
{
  long v1;
  long v2;
  unsigned long v3; // rax
  double v4; // xmm0_qa
  
  if (*(int *)((long)a2 + 0x3c))
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0xa6,"pred_anewer"); // no-return
  v1 = *(long *)((long)a1 + 0x50);
  v2 = *(long *)((long)a2 + 0x48);
  if ((*(long *)((long)a1 + 0x48) == *(long *)((long)a2 + 0x40)) && (v1 == v2))
    return 0;
  v3 = difftime();
  return CONCAT71((undefined7)((unsigned long)v3 >> 8),0.0 <= (double)(v1 - v2) * dat_28950 + v4);
}

// Function: sub_8bc0 @ 0x8bc0
unsigned long sub_8bc0(unsigned long a0,void *a1,void *a2)
{
  long v1;
  long v2;
  
  v1 = *(long *)((long)a1 + 0x50);
  v2 = *(long *)((long)a1 + 0x48);
  return sub_89e0(v2,v1,a2,0x15180); // tail-call
}

// Function: sub_8be0 @ 0x8be0
unsigned long sub_8be0(void)
{
  return 1;
}

// Function: sub_8bf0 @ 0x8bf0
unsigned long sub_8bf0(unsigned long a0,void *a1,void *a2)
{
  long v1;
  long v2;
  
  v1 = *(long *)((long)a1 + 0x70);
  v2 = *(long *)((long)a1 + 0x68);
  return sub_89e0(v2,v1,a2,0x3c); // tail-call
}

// Function: sub_8c10 @ 0x8c10
unsigned long sub_8c10(unsigned long a0,void *a1,void *a2)
{
  long v1;
  long v2;
  unsigned long v3; // rax
  double v4; // xmm0_qa
  
  if (*(int *)((long)a2 + 0x3c))
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",199,"pred_cnewer"); // no-return
  v1 = *(long *)((long)a1 + 0x70);
  v2 = *(long *)((long)a2 + 0x48);
  if ((*(long *)((long)a1 + 0x68) == *(long *)((long)a2 + 0x40)) && (v1 == v2))
    return 0;
  v3 = difftime();
  return CONCAT71((undefined7)((unsigned long)v3 >> 8),0.0 <= (double)(v1 - v2) * dat_28950 + v4);
}

// Function: sub_8ca0 @ 0x8ca0
unsigned long sub_8ca0(char *a0,void *a1,void *a2)
{
  struct_2 *v1;
  
  if (*(struct_2 **)((long)a2 + 0x110))
    sub_e7b0(a0,a1,*(struct_2 **)((long)a2 + 0x110));
  v1 = *(struct_2 **)((long)a2 + 0x118);
  return sub_e7b0(a0,a1,v1); // tail-call
}

// Function: sub_8ce0 @ 0x8ce0
unsigned long sub_8ce0(unsigned long a0,void *a1,void *a2)
{
  long v1;
  long v2;
  
  v1 = *(long *)((long)a1 + 0x70);
  v2 = *(long *)((long)a1 + 0x68);
  return sub_89e0(v2,v1,a2,0x15180); // tail-call
}

// Function: sub_8d00 @ 0x8d00
char sub_8d00(char *a0,void *a1) // return-dupe
{
  int *v1; // rax
  char *v2;
  char *v3; // rax
  int v4; // r13d
  
  v2 = dat_385f0;
  if (!strcmp(dat_385f0,"."))
    return '\x01';
  v4 = 0;
  if (dat_385e4)
    v4 = (unsigned int)((*(unsigned int *)((long)a1 + 0x18) & 0xf000) == 0x4000) << 9;
  if (!unlinkat(dat_385f8,v2,v4))
    return '\x01';
  v1 = __errno_location();
  if (*v1 != 2) { // branch-flip
    if (((*v1 == 0x15) && (!v4)) && (!unlinkat(dat_385f8,dat_385f0,0x200)))
      return '\x01';
  }
  else if (dat_3862e) {
    *v1 = 0;
    return dat_3862e;
  }
  v2 = (char *)sub_e8a0(0,a0);
  v3 = dcgettext(NULL,"cannot delete %s",5);
  error(0,*v1,v3,v2);
  dat_38604 = 1;
  return '\0';
}

// Function: sub_8e20 @ 0x8e20
bool sub_8e20(char *a0,void *a1)
{
  unsigned int v1; // eax
  int v2; // eax
  int *v3; // rax
  DIR *v4; // rax
  dirent *v5; // rax
  char *v6; // rax
  bool v7; // r12b
  
  v1 = *(unsigned int *)((long)a1 + 0x18) & 0xf000;
  if (v1 != 0x4000) {
    if (v1 != 0x8000)
      return 0;
    return *(long *)((long)a1 + 0x30) == 0;
  }
  v3 = __errno_location();
  *v3 = 0;
  v2 = sub_1f080(dat_385f8,dat_385f0,0x90900);
  if (0 <= v2) {
    v4 = fdopendir(v2);
    if (!v4) {
      v6 = (char *)sub_e8a0(0,a0);
      error(0,*v3,"%s",v6);
      dat_38604 = 1;
      close(v2);
      return 0;
    }
    *v3 = 0;
    do {
      v5 = readdir(v4);
      if (!v5) {
        v7 = 1;
        goto label_8ed3;
      }
    } while ((v5->field_0x13 == '.') && ((!v5->field_0x14 || (*(short *)&v5->field_0x14 == 0x2e))));
    v7 = 0;
label_8ed3:
    if (*v3) {
      v6 = (char *)sub_e8a0(0,a0);
      error(0,*v3,"%s",v6);
      dat_38604 = 1;
      closedir(v4);
      return 0;
    }
    if (!closedir(v4))
      return v7;
  }
  v6 = (char *)sub_e8a0(0,a0);
  error(0,*v3,"%s",v6);
  dat_38604 = 1;
  return 0;
}

// Function: sub_8fb0 @ 0x8fb0
void sub_8fb0(char *a0)
{
  sub_a040(a0); // tail-call
}

// Function: sub_8fc0 @ 0x8fc0
void sub_8fc0(void)
{
  sub_a040(dat_385f0); // tail-call
}

// Function: sub_8fd0 @ 0x8fd0
unsigned long sub_8fd0(void)
{
  return 0;
}

// Function: sub_8fe0 @ 0x8fe0
unsigned long sub_8fe0(unsigned char *a0,void *a1,void *a2)
{
  sub_165e0(a0,dat_385f8,dat_385f0,a1,dat_38638,dat_3865c,(unsigned char)*(char *)((long)a2 + 0x28),*(FILE **)((long)a2 + 0x40));
  return 1;
}

// Function: sub_9020 @ 0x9020
unsigned long sub_9020(char *a0,unsigned long a1,void *a2)
{
  sub_16e80(*(FILE **)((long)a2 + 0x40),*(unsigned int **)((long)a2 + 0x58),(unsigned char)*(char *)((long)a2 + 0x50),"%s\n",a0);
  return 1;
}

// Function: sub_9060 @ 0x9060
unsigned long sub_9060(char *a0,unsigned long a1,void *a2)
{
  FILE *v1;
  
  v1 = *(FILE **)((long)a2 + 0x40);
  fputs(a0,v1);
  putc(0,v1);
  return 1;
}

// Function: sub_9090 @ 0x9090
unsigned int sub_9090(unsigned long a0,long *a1,void *a2)
{
  char *v1;
  unsigned int v2; // eax
  
  v1 = *(char **)((long)a2 + 0x38);
  v2 = strcmp((char *)sub_108e0(a1,a0),v1);
  return CONCAT31((undefined3)(v2 >> 8),v2 == 0);
}

// Function: sub_90c0 @ 0x90c0
unsigned long sub_90c0(unsigned long a0,void *a1,void *a2,unsigned long a3,unsigned long a4)
{
  int v1;
  undefined7 v2; // r8
  
  v1 = *(int *)((long)a2 + 0x38);
  v2 = (undefined7)((unsigned long)a4 >> 8);
  if (v1 == 1)
    return CONCAT71(v2,(unsigned long)*(unsigned int *)((long)a1 + 0x20) < *(unsigned long *)((long)a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)((long)a2 + 0x40) < (unsigned long)*(unsigned int *)((long)a1 + 0x20));
  }
  return CONCAT71(v2,(unsigned long)*(unsigned int *)((long)a1 + 0x20) == *(unsigned long *)((long)a2 + 0x40)) & 0xffffffff;
}

// Function: sub_9110 @ 0x9110
unsigned int sub_9110(unsigned long a0,void *a1,void *a2)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)((long)a1 + 0x20) >> 8),*(int *)((long)a2 + 0x38) == *(int *)((long)a1 + 0x20));
}

// Function: sub_9120 @ 0x9120
unsigned long sub_9120(char *a0,void *a1,void *a2)
{
  if ((*(unsigned int *)((long)a1 + 0x18) & 0xf000) != 0xa000)
    return 0;
  return sub_8660(a0,a2,1); // tail-call
}

// Function: sub_9150 @ 0x9150
int sub_9150(char *a0,unsigned long a1,void *a2)
{
  unsigned int v1;
  int v2;
  char *v3;
  unsigned int v4; // eax
  char *v5;
  
  v5 = *(char **)((long)a2 + 0x38);
  v5 = (char *)sub_17b90(a0);
  v3 = v5;
  sub_17d00(v5);
  v4 = fnmatch(v5,v5,0x10);
  v2 = v4;
  free(v3);
  v1 = v4 >> 8;
  v4 = CONCAT31((undefined3)v1,v2 == 0);
  return v4;
}

// Function: sub_9190 @ 0x9190
unsigned long sub_9190(unsigned long a0,void *a1,void *a2,unsigned long a3,unsigned long a4)
{
  int v1;
  undefined7 v2; // r8
  
  v1 = *(int *)((long)a2 + 0x38);
  v2 = (undefined7)((unsigned long)a4 >> 8);
  if (v1 == 1)
    return CONCAT71(v2,*(unsigned long *)((long)a1 + 8) < *(unsigned long *)((long)a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)((long)a2 + 0x40) < *(unsigned long *)((long)a1 + 8));
  }
  return CONCAT71(v2,*(long *)((long)a1 + 8) == *(long *)((long)a2 + 0x40)) & 0xffffffff;
}

// Function: sub_91e0 @ 0x91e0
unsigned int sub_91e0(char *a0,unsigned long a1,void *a2)
{
  unsigned int v1; // eax
  
  v1 = fnmatch(*(char **)((long)a2 + 0x38),a0,0x10);
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}

// Function: sub_9210 @ 0x9210
unsigned long sub_9210(unsigned long a0,void *a1,void *a2,unsigned long a3,unsigned long a4)
{
  int v1;
  undefined7 v2; // r8
  
  v1 = *(int *)((long)a2 + 0x38);
  v2 = (undefined7)((unsigned long)a4 >> 8);
  if (v1 == 1)
    return CONCAT71(v2,*(unsigned long *)((long)a1 + 0x10) < *(unsigned long *)((long)a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)((long)a2 + 0x40) < *(unsigned long *)((long)a1 + 0x10));
  }
  return CONCAT71(v2,*(long *)((long)a1 + 0x10) == *(long *)((long)a2 + 0x40)) & 0xffffffff;
}

// Function: sub_9260 @ 0x9260
unsigned long sub_9260(char *a0,void *a1,void *a2)
{
  if ((*(unsigned int *)((long)a1 + 0x18) & 0xf000) != 0xa000)
    return 0;
  return sub_8660(a0,a2,0); // tail-call
}

// Function: sub_9290 @ 0x9290
unsigned long sub_9290(unsigned char *a0,void *a1,void *a2)
{
  sub_165e0(a0,dat_385f8,dat_385f0,a1,dat_38638,dat_3865c,(unsigned char)*(char *)((long)a2 + 0x28),*(FILE **)((long)a2 + 0x40));
  return 1;
}

// Function: sub_92d0 @ 0x92d0
unsigned long sub_92d0(unsigned long a0,void *a1,void *a2)
{
  long v1;
  long v2;
  
  v1 = *(long *)((long)a1 + 0x60);
  v2 = *(long *)((long)a1 + 0x58);
  return sub_89e0(v2,v1,a2,0x3c); // tail-call
}

// Function: sub_92f0 @ 0x92f0
unsigned long sub_92f0(unsigned long a0,void *a1,void *a2)
{
  long v1;
  long v2;
  
  v1 = *(long *)((long)a1 + 0x60);
  v2 = *(long *)((long)a1 + 0x58);
  return sub_89e0(v2,v1,a2,0x15180); // tail-call
}

// Function: sub_9310 @ 0x9310
int sub_9310(char *a0,unsigned long a1,void *a2)
{
  unsigned int v1;
  int v2;
  char *v3;
  unsigned int v4; // eax
  char *v5;
  
  v5 = *(char **)((long)a2 + 0x38);
  v5 = (char *)sub_17b90(a0);
  v3 = v5;
  sub_17d00(v5);
  v4 = fnmatch(v5,v5,0);
  v2 = v4;
  free(v3);
  v1 = v4 >> 8;
  v4 = CONCAT31((undefined3)v1,v2 == 0);
  return v4;
}

// Function: sub_9350 @ 0x9350
unsigned int sub_9350(char *a0,void *a1,void *a2)
{
  struct_2 *v1;
  
  v1 = *(struct_2 **)((long)a2 + 0x118);
  return sub_e7b0(a0,a1,v1) ^ 1;
}

// Function: sub_9370 @ 0x9370
unsigned long sub_9370(unsigned long a0,void *a1,void *a2)
{
  long v1;
  long v2;
  unsigned long v3; // rax
  double v4; // xmm0_qa
  
  if (*(int *)((long)a2 + 0x3c))
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0x26d,"pred_newer"); // no-return
  v1 = *(long *)((long)a1 + 0x60);
  v2 = *(long *)((long)a2 + 0x48);
  if ((*(long *)((long)a1 + 0x58) == *(long *)((long)a2 + 0x40)) && (v1 == v2))
    return 0;
  v3 = difftime();
  return CONCAT71((undefined7)((unsigned long)v3 >> 8),0.0 <= (double)(v1 - v2) * dat_28950 + v4);
}

// Function: sub_9400 @ 0x9400
unsigned long sub_9400(char *a0,void *a1,void *a2) // early-return
{
  long v1;
  unsigned long v2; // rax
  long v3;
  long v4;
  double v5; // xmm0_qa
  
  if (*(int *)((long)a2 + 0x3c))
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0x277,"pred_newerXY"); // no-return
  switch(*(unsigned int *)((long)a2 + 0x38)) {
    case 0:
      v4 = *(long *)((long)a1 + 0x48);
      v3 = *(long *)((long)a1 + 0x50);
      break;
    case 1:
      v2 = sub_e8a0(0,a0);
      error(0,0,dcgettext(NULL,"WARNING: cannot determine birth time of file %s",5),v2);
      return 0;
    case 2:
      v4 = *(long *)((long)a1 + 0x68);
      v3 = *(long *)((long)a1 + 0x70);
      break;
    case 3:
      v4 = *(long *)((long)a1 + 0x58);
      v3 = *(long *)((long)a1 + 0x60);
      break;
    case 4:
      __assert_fail("pred_ptr->args.reftime.xval != XVAL_TIME","pred.c",0x27c,"pred_newerXY"); // no-return
    default:
      __assert_fail("collected","pred.c",0x29b,"pred_newerXY"); // no-return
    
  }
  v1 = *(long *)((long)a2 + 0x48);
  if ((*(long *)((long)a2 + 0x40) == v4) && (v3 == v1))
    return 0;
  v2 = difftime();
  return CONCAT71((undefined7)((unsigned long)v2 >> 8),0.0 <= (double)(v3 - v1) * dat_28950 + v5);
}

// Function: sub_9540 @ 0x9540
unsigned long sub_9540(unsigned long a0,void *a1)
{
  group *v1; // rax
  
  v1 = getgrgid(*(unsigned int *)((long)a1 + 0x20));
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 == NULL);
}

// Function: sub_9560 @ 0x9560
unsigned long sub_9560(unsigned long a0,void *a1)
{
  passwd *v1; // rax
  
  v1 = getpwuid(*(unsigned int *)((long)a1 + 0x1c));
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 == NULL);
}

// Function: sub_9580 @ 0x9580
unsigned long sub_9580(char *a0,unsigned long a1,struct_34 *a2)
{
  char *v1;
  
  v1 = *(char **)a2->field_0xe8;
  if (!sub_8550(v1,a0))
    return 0;
  return sub_a040(a0,a1,a2); // tail-call
}

// Function: sub_95d0 @ 0x95d0
unsigned long sub_95d0(char *a0,unsigned long a1,struct_34 *a2)
{
  char *v1;
  unsigned long v2; // rax
  
  v1 = *(char **)a2->field_0xe8;
  if (!sub_8550(v1,a0))
    return 0;
  v2 = sub_a040(dat_385f0,a1,a2); // tail-call
  return v2;
}

// Function: sub_9630 @ 0x9630
unsigned long sub_9630(void)
{
  return 1;
}

// Function: sub_9640 @ 0x9640
unsigned int sub_9640(char *a0,unsigned long a1,void *a2)
{
  unsigned int v1; // eax
  
  v1 = fnmatch(*(char **)((long)a2 + 0x38),a0,0);
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}

// Function: sub_9660 @ 0x9660
unsigned long sub_9660(unsigned long a0,void *a1,void *a2,unsigned long a3,unsigned long a4)
{
  unsigned int v1;
  unsigned int v2;
  int v3;
  undefined7 v4; // r8
  
  v1 = *(unsigned int *)((long)a1 + 0x18);
  v2 = *(unsigned int *)((long)a2 + (unsigned long)((v1 & 0xf000) == 0x4000) * 4 + 0x3c);
  v3 = *(int *)((long)a2 + 0x38);
  if (v3 == 1) {
    if (!v2)
      return 1;
    return (unsigned long)((v1 & v2) != 0);
  }
  v4 = (undefined7)((unsigned long)a4 >> 8);
  if (v3 != 2) {
    if (v3)
      abort(); // no-return
    return CONCAT71(v4,(v1 & v2) == v2) & 0xffffffff;
  }
  return CONCAT71(v4,(v1 & 0xfff) == v2) & 0xffffffff;
}

// Function: sub_96e0 @ 0x96e0
unsigned int sub_96e0(void)
{
  unsigned int v1; // eax
  
  v1 = faccessat(dat_385f8,dat_385f0,1,0);
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}

// Function: sub_9710 @ 0x9710
unsigned int sub_9710(void)
{
  unsigned int v1; // eax
  
  v1 = faccessat(dat_385f8,dat_385f0,4,0);
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}

// Function: sub_9740 @ 0x9740
unsigned int sub_9740(void)
{
  unsigned int v1; // eax
  
  v1 = faccessat(dat_385f8,dat_385f0,2,0);
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}

// Function: sub_9770 @ 0x9770
unsigned long sub_9770(char *a0,unsigned long a1,void *a2)
{
  sub_16e80(*(FILE **)((long)a2 + 0x40),*(unsigned int **)((long)a2 + 0x58),(unsigned char)*(char *)((long)a2 + 0x50),"%s\n",a0);
  return 1;
}

// Function: sub_97b0 @ 0x97b0
unsigned long sub_97b0(char *a0,unsigned long a1,void *a2)
{
  FILE *v1;
  
  v1 = *(FILE **)((long)a2 + 0x40);
  fputs(a0,v1);
  putc(0,v1);
  return 1;
}

// Function: sub_97e0 @ 0x97e0
unsigned long sub_97e0(char *a0,unsigned long a1,void *a2)
{
  int v1; // eax
  unsigned long v2; // rax
  
  v2 = strlen(a0);
  v1 = re_match(*(re_pattern_buffer **)((long)a2 + 0x38),a0,(int)v2,0,NULL);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),v1 == (int)v2);
}

// Function: sub_9820 @ 0x9820
unsigned long sub_9820(unsigned long a0,void *a1,void *a2)
{
  int v1;
  unsigned long v2; // rax
  undefined7 v3; // r8
  
  v1 = *(int *)((long)a2 + 0x38);
  v2 = (*(long *)((long)a1 + 0x30) / (long)*(int *)((long)a2 + 0x3c) + 1) - (unsigned long)(*(long *)((long)a1 + 0x30) % (long)*(int *)((long)a2 + 0x3c) == 0);
  v3 = (undefined7)((unsigned long)a1 >> 8);
  if (v1 == 1)
    return CONCAT71(v3,v2 < *(unsigned long *)((long)a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)((long)a2 + 0x40) < v2);
  }
  return CONCAT71(v3,*(unsigned long *)((long)a2 + 0x40) == v2) & 0xffffffff;
}

// Function: sub_9890 @ 0x9890
unsigned long sub_9890(char *a0,struct_12 *a1,void *a2)
{
  unsigned long v1; // rax
  
  if (((!a1->field_0x8) || (a1->field_0x8 == *(long *)((long)a2 + 0x38))) && (v1 = sub_d820(a0,dat_385f0,a1), !(int)v1)) {
    if (a1->field_0x8 != *(long *)((long)a2 + 0x38))
      return v1;
    return CONCAT71((undefined7)((unsigned long)*(long *)((long)a2 + 0x40) >> 8),a1->field_0x0 == *(long *)((long)a2 + 0x40));
  }
  return 0;
}

// Function: sub_98f0 @ 0x98f0
unsigned long sub_98f0(void)
{
  return 1;
}

// Function: sub_9900 @ 0x9900
char sub_9900(unsigned long a0,void *a1,long a2)
{
  unsigned int v1;
  long v2; // rcx
  char v3;
  
  if (!dat_385e5)
    __assert_fail("state.have_type","pred.c",0x3d1,"pred_type"); // no-return
  v3 = 0;
  if (dat_385e8) {
    v1 = dat_385e8;
    if (dat_385e4)
      v1 = *(unsigned int *)((long)a1 + 0x18);
    v1 &= 0xf000;
    if (v1 != 0x6000) { // branch-flip
      if (0x6001 <= v1) { // branch-flip
        if (v1 != 0xa000) { // branch-flip
          v2 = 6;
          if (v1 != 0xc000) {
            if (v1 != 0x8000)
              return 0;
            v2 = 3;
          }
        }
        else {
          v2 = 4;
        }
      }
      else if (v1 != 0x2000) { // branch-flip
        v2 = 2;
        if ((v1 != 0x4000) && (v2 = 5, v1 != 0x1000))
          return 0;
      }
      else {
        v2 = 1;
      }
    }
    else {
      v2 = 0;
    }
    v3 = *(char *)(a2 + 0x38 + v2);
  }
  return v3;
}

// Function: sub_99e0 @ 0x99e0
unsigned long sub_99e0(unsigned long a0,void *a1,void *a2,unsigned long a3,unsigned long a4)
{
  int v1;
  undefined7 v2; // r8
  
  v1 = *(int *)((long)a2 + 0x38);
  v2 = (undefined7)((unsigned long)a4 >> 8);
  if (v1 == 1)
    return CONCAT71(v2,(unsigned long)*(unsigned int *)((long)a1 + 0x1c) < *(unsigned long *)((long)a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)((long)a2 + 0x40) < (unsigned long)*(unsigned int *)((long)a1 + 0x1c));
  }
  return CONCAT71(v2,(unsigned long)*(unsigned int *)((long)a1 + 0x1c) == *(unsigned long *)((long)a2 + 0x40)) & 0xffffffff;
}

// Function: sub_9a30 @ 0x9a30
unsigned long sub_9a30(unsigned long a0,void *a1,void *a2)
{
  long v1;
  long v2;
  unsigned long v3; // rax
  long v4;
  long v5;
  double v6; // xmm0_qa
  
  v1 = *(long *)((long)a1 + 0x50);
  v4 = *(long *)((long)a1 + 0x70);
  v2 = *(long *)((long)a1 + 0x48);
  v5 = *(long *)((long)a1 + 0x68);
  if (v1 != v4 || v2 != v5) { // branch-flip
    difftime(v2,v5);
    if ((double)(v1 - v4) * dat_28950 + v6 < 0.0)
      return 0;
    v5 -= v2;
    v4 -= v1;
    if (v4 <= -1) {
      v4 += 1000000000;
      v5 -= 1;
    }
  }
  else {
    v4 = 0;
    v5 = 0;
  }
  v3 = sub_89e0(v5,v4,a2,0x15180); // tail-call
  return v3;
}

// Function: sub_9ae0 @ 0x9ae0
unsigned int sub_9ae0(unsigned long a0,void *a1,void *a2)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)((long)a1 + 0x1c) >> 8),*(int *)((long)a2 + 0x38) == *(int *)((long)a1 + 0x1c));
}

// Function: sub_9af0 @ 0x9af0
int * sub_9af0(char *a0,void *a1,long a2) // early-return, ternary
{
  char v1;
  int *v2;
  char v3; // al
  int v4; // eax
  int *v5; // rax
  void *v6; // rbx
  char v7 [152];
  
  v3 = sub_dfb0();
  v6 = (v3) ? sub_de70 : sub_ddb0; // branch-flip
  sub_d810(v7);
  v4 = (*v6)(dat_385f0,v7);
  if (v4) { // branch-flip
    v3 = sub_dfb0();
    v1 = v3;
    v5 = __errno_location();
    v2 = v5;
    if ((!v1) || (*v5 != 2)) {
      v5 = (char *)sub_e8a0(0,a0);
      error(0,*v2,"%s",(char *)v5);
      v5 = NULL;
      dat_38604 = 1;
      return v5;
    }
    v5 = (unsigned long)sub_9900(a0,a1,a2);
  }
  else {
    v5 = (unsigned long)sub_9900(a0,v7,a2);
  }
  return v5;
}

// Function: sub_9c00 @ 0x9c00
unsigned long sub_9c00(char *a0,unsigned long a1,void *a2)
{
  int v1; // eax
  unsigned long v2; // rax
  char *v3;
  char *v4; // stack - 0x38
  unsigned long v5; // r12
  unsigned long v6; // r12
  
  v1 = (*dat_38680)(dat_385f8,dat_385f0,&v4);
  if (0 <= v1) { // branch-flip
    v3 = *(char **)((long)a2 + 0x38);
    v6 = CONCAT71((undefined7)((unsigned long)v5 >> 8),fnmatch(v3,v4,0) == 0);
    freecon(v4);
  }
  else {
    v2 = sub_e8a0(0,a0);
    v3 = dcgettext(NULL,"getfilecon failed: %s",5);
    v6 = 0;
    error(0,*__errno_location(),v3,v2);
  }
  return v6 & 0xffffffff;
}

// Function: sub_9cd0 @ 0x9cd0
void sub_9cd0(FILE *a0,void *a1)
{
  char *v1;
  int v2; // eax
  char *v3;
  int v4; // edx
  char v5 [264];
  
  do {
    if (!a1) {
      __fprintf_chk(a0,1,"\n");
      return;
    }
    v3 = NULL;
    if (*(char **)((long)a1 + 8)) {
      __strcpy_chk(v5,*(char **)((long)a1 + 8),0x100);
      v2 = (int)strlen(v5);
      v4 = v2 + -1;
      if (0 <= v4) { // branch-flip
        v1 = &v5[v4];
        do {
          v3 = v1;
          if ((*v3 != ' ') && (*v3 != '\t')) {
            v3 = &v5[v4 + 1];
            break;
          }
          v4 -= 1;
          v1 = &v3[-1];
        } while (v4 != -1);
      }
      else {
        v3 = &v5[v2];
      }
      *v3 = '\0';
      v3 = v5;
    }
    __fprintf_chk(a0,1,"[%s] ",v3);
    a1 = *(void **)((long)a1 + 0x108);
  } while( true );
}

// Function: sub_9de0 @ 0x9de0
void sub_9de0(FILE *a0,void *a1)
{
  if (a1) {
    sub_86e0(a0,a1); // tail-call
    return;
  }
}

// Function: sub_9e00 @ 0x9e00
void sub_9e00(void *a0)
{
  if (!(dat_38660 & 0x40))
    return;
  __fprintf_chk(stderr,1,"Predicate success rates after completion:\n");
  if (a0)
    sub_86e0(stderr,a0);
  __fprintf_chk(stderr,1,"\n"); // tail-call
}

// Function: sub_9e60 @ 0x9e60
void sub_9e60(long *a0)
{
  void *v1;
  int *v2;
  
  if (!a0)
    return;
  do {
    v1 = (void *)*a0;
    if (!v1)
      __assert_fail("p->pred_func != NULL","pred.c",0x51e,"pred_sanity_check"); // no-return
    v2 = (int *)a0[0x26];
    if (!v2)
      __assert_fail("p->parser_entry != NULL","pred.c",0x521,"pred_sanity_check"); // no-return
    if ((v1 != *(void **)&v2[6]) && (*(void **)&v2[6]))
      __assert_fail("p->parser_entry->pred_func == p->pred_func","pred.c",0x52a,"pred_sanity_check"); // no-return
    switch(*v2) {
      case 0:
      case 2:
        if (!*v2)
          __assert_fail("p->parser_entry->type != ARG_OPTION","pred.c",0x53a,"pred_sanity_check"); // no-return
        __assert_fail("p->parser_entry->type != ARG_POSITIONAL_OPTION","pred.c",0x53b,"pred_sanity_check"); // no-return
      case 1:
      case 3:
      case 4:
      case 5:
        if (*(char *)((long)a0 + 0x19))
          __assert_fail("!p->no_default_print","pred.c",0x553,"pred_sanity_check"); // no-return
        if ((char)a0[3])
          __assert_fail("!p->side_effects","pred.c",0x554,"pred_sanity_check"); // no-return
        break;
      case 6:
        if (!(char)a0[3])
          __assert_fail("p->side_effects","pred.c",0x53f,"pred_sanity_check"); // no-return
        if (((v1 != sub_8600) && (v1 != sub_85e0)) && (!*(char *)((long)a0 + 0x19)))
          __assert_fail("p->no_default_print","pred.c",0x545,"pred_sanity_check"); // no-return
      
    }
    a0 = (long *)a0[0x21];
    if (!a0)
      return;
  } while( true );
}

// Function: sub_a040 @ 0xa040
unsigned long sub_a040(char *a0,unsigned long a1,struct_34 *a2)
{
  struct_7 *v1;
  long v10; // rbx
  char v11;
  bool v12; // zf
  char *v13; // stack - 0x50
  unsigned char v14; // stack - 0x41
  char *v15; // stack - 0x40
  struct_6 *v2;
  unsigned int v3;
  unsigned long v4; // rax
  char *v5; // rax
  char *v6;
  unsigned int *v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v14 = sub_e880(a2->field_0x0);
  v6 = dat_385f0;
  if (v14) { // branch-flip
    if (!a2->field_0xd0) {
      v5 = strchr(dat_385f0,0x2f);
      v3 = dat_385f8;
      if (v5) { // branch-flip
        v6 = (char *)sub_17c40(v6);
        v3 = dat_385f8;
        v7 = (unsigned int *)sub_25a50(0x10);
        a2->field_0xf8 = (long)v7;
        *(unsigned long *)&v7[2] = 0;
        *v7 = sub_1f080(v3,v6,0);
        if (*(int *)a2->field_0xf8 < 0) {
          free(v6);
label_a33f:
          v8 = sub_e8a0(0,a0);
          v6 = dcgettext(NULL,"Failed to save working directory in order to run a command on %s",5);
          error(1,*__errno_location(),v6,v8);
          return v9;
        }
        sub_17990(*(int *)a2->field_0xf8,1);
        free(v6);
        v6 = dat_385f0;
      }
      else {
        v7 = (unsigned int *)sub_25a50(0x10);
        a2->field_0xf8 = (long)v7;
        *(unsigned long *)&v7[2] = 0;
        *v7 = sub_1f080(v3,".",0);
        if (*(int *)a2->field_0xf8 < 0) goto label_a33f;
        sub_17990(*(int *)a2->field_0xf8,1);
        v6 = dat_385f0;
      }
    }
    a0 = (char *)sub_17b90(v6);
    v15 = a0;
    v11 = (*a0 != '/') * '\x02';
    v13 = NULL;
    if (*a0 != '/')
      v13 = "./";
  }
  else {
    if (a2->field_0xf8 != dat_385c0)
      __assert_fail("execp->wd_for_exec == initial_wd","exec.c",0x94,"impl_pred_exec"); // no-return
    v13 = NULL;
    v11 = '\0';
    v15 = NULL;
  }
  v4 = (unsigned long)a2->field_0x38;
  v2 = (struct_6 *)&a2->field_0x39[0x5f];
  v1 = (struct_7 *)&a2->field_0x39[7];
  if (a2->field_0x38) { // branch-flip
    sub_14fb0(v1,v2,a0,strlen(a0) + 1,v13,(unsigned char)v11,0);
    if (a2->field_0xd0)
      dat_38608 = 1;
    if (!v15) {
      return v4 & 0xffffffff; // return-dupe
    }
    if (!v14)
      __assert_fail("local","exec.c",0xd0,"impl_pred_exec"); // return-dupe, no-return
  }
  else {
    v10 = 0;
    if (1 <= a2->field_0xf0) {
      do {
        v4 = strlen(a0);
        v6 = *(char **)(a2->field_0xe8 + v10 * 8);
        v10 += 1;
        sub_15270(v1,v2,v6,strlen(v6),v13,(unsigned char)v11,a0,v4,0);
      } while ((int)v10 < a2->field_0xf0);
    }
    sub_15430(v1,v2);
    v12 = !(a2->field_0x100 >> 8 & 0xff) && !(a2->field_0x100 & 0x7f);
    v4 = CONCAT71((undefined7)((unsigned long)v10 >> 8),v12);
    if (!v14) {
      if (v15) {
        __assert_fail("local","exec.c",0xd0,"impl_pred_exec");
      }
      return v4 & 0xffffffff;
    }
    sub_242c0((struct_10 *)a2->field_0xf8);
    if (!v15) {
      return v4 & 0xffffffff;
    }
    v14 = v12;
  }
  free(v15);
  v4 = (unsigned long)v14;
  return v4 & 0xffffffff;
}

// Function: sub_a3a0 @ 0xa3a0
int * sub_a3a0(unsigned long a0,struct_53 *a1,unsigned long a2,char **a3) // return-dupe
{
  long v1;
  unsigned long v10; // stack - 0x3a
  char *v11;
  char *v12; // r12
  unsigned short v13; // stack - 0x32
  struct_9 *v2;
  int v3;
  int *v4;
  char v5; // al
  int v6; // eax
  int *v7; // rax
  unsigned int v8;
  unsigned long v9; // rbx
  
  if (dat_38660 & 0x20) {
    v9 = 0;
    __fprintf_chk(stderr,1,"DebugExec: launching process (argc=%lu):",a1->field_0x60 - 1);
    if (a1->field_0x60 != 1) {
      do {
        v1 = v9 * 8;
        v9 += 1;
        v7 = (unsigned long)sub_e8a0(0,*(char **)(a1->field_0x68 + v1));
        __fprintf_chk(stderr,1," %s",v7);
        v7 = (unsigned long)(a1->field_0x60 - 1);
      } while (v9 < v7);
    }
    __fprintf_chk(stderr,1,"\n");
  }
  sub_180c0(stdout);
  sub_180c0(stderr);
  if (dat_3801c) {
    dat_3801c = 0;
    signal(0x11,0);
  }
  v6 = fork();
  v3 = v6;
  if (v6 == -1) {
    v7 = (char *)dcgettext(NULL,"cannot fork",5);
    v11 = (char *)v7;
    v7 = __errno_location();
    error(1,*v7,v11);
    return v7;
  }
  if (v6) {
    while (v6 = waitpid(v3,(int *)&a1->field_0xc8,0), v6 == -1) {
      v7 = __errno_location();
      v4 = v7;
      if (*v7 != 4) {
        v7 = (char *)sub_e8a0(0,*a3);
        v11 = (char *)v7;
        v7 = (char *)dcgettext(NULL,"error waiting for %s",5);
        error(0,*v4,(char *)v7,v11);
        dat_38604 = 1;
        v7 = NULL;
        return v7;
      }
    }
    v8 = a1->field_0xc8;
    v5 = ((unsigned char)v8 & 0x7f) + 1;
    if ('\x02' <= v5) { // branch-flip
      v7 = (char *)sub_239f0(0,dat_3868c,*a3);
      v11 = (char *)v7;
      v7 = (char *)dcgettext(NULL,"%s terminated by signal %d",5);
      error(0,0,(char *)v7,v11,v8 & 0x7f);
    }
    else {
      v8 = v8 >> 8 & 0xff;
      if (dat_38660 & 0x20)
        __fprintf_chk(stderr,1,"DebugExec: process (PID=%ld) terminated with exit status: %d\n",(long)v3,v8);
      if (!v8) {
        v7 = (int *)0x1;
        return v7;
      }
    }
    if (!a1->field_0x0) {
      v7 = (int *)0x1;
      return v7;
    }
    v7 = (int *)0x1;
    dat_38604 = 1;
    return v7;
  }
  v2 = a1->field_0xc0;
  if (!v2)
    __assert_fail("NULL != execp->wd_for_exec","exec.c",0x141,"launch"); // no-return
  v7 = __errno_location();
  v4 = v7;
  if (a1->field_0xbc) { // branch-flip
    v10 = 0x6c756e2f7665642f;
    v13 = 0x6c;
    v6 = close(0);
    if (0 <= v6) {
      v6 = sub_18040(&v10,0);
      if (0 <= v6) goto label_a437;
      v7 = (unsigned long)sub_e8a0(0,&v10);
      error(0,*v4,"%s",v7);
      v6 = sub_24290(v2);
      goto label_a7ad;
    }
    v7 = (char *)dcgettext(NULL,"Cannot close standard input",5);
    error(0,*v4,(char *)v7);
    v6 = sub_24290(v2);
    if (!v6) {
      _exit(1); // return-dupe, no-return
    }
  }
  else {
label_a437:
    v6 = sub_24290(v2);
label_a7ad:
    if (!v6) {
      v5 = sub_16220();
      if (v5)
        sub_16240();
      v5 = sub_15970(a3);
      if (v5) // branch-flip
        *v4 = 7;
      else {
        execvp(*a3,a3);
      }
      v7 = (unsigned long)sub_e8a0(0,*a3);
      error(0,*v4,"%s",v7);
      _exit(1);
    }
  }
  if (0 <= v2->field_0x0) { // branch-flip
label_a614:
    v11 = "";
    v12 = "";
  }
  else {
    v11 = v2->field_0x8;
    v12 = ": ";
    if (!v11) goto label_a614;
  }
  v7 = (char *)dcgettext(NULL,"Failed to change directory%s%s",5);
  error(0,*v4,(char *)v7,v12,v11);
  _exit(1);
}

// Function: sub_a840 @ 0xa840
void sub_a840(void *a0,char a1)
{
  long v1;
  
  while( true ) {
    if ((a1) && (*(int *)((long)a0 + 0x10) != 3))
      __assert_fail("BI_OP == p->p_type","tree.c",0x4ce,"check_normalization"); // no-return
    v1 = *(long *)((long)a0 + 0x110);
    if (v1) {
      if (*(int *)(v1 + 0x10) != 3)
        __assert_fail("BI_OP == p->pred_left->p_type","tree.c",0x4d3,"check_normalization"); // no-return
      sub_a840(v1,0);
    }
    a0 = *(void **)((long)a0 + 0x118);
    if (!a0) break;
    a1 = 0;
  }
}

// Function: sub_a8c0 @ 0xa8c0
char * sub_a8c0(long *a0,int a1,void *a2)
{
  short v1;
  long v10;
  int v2;
  unsigned int v3;
  unsigned long v4;
  char *v5;
  char *v6; // rax
  long v7; // rdx
  long v8;
  long *v9;
  
  v10 = *a0;
  if (!v10) {
    v6 = dcgettext(NULL,"invalid expression",5);
    error(1,0,v6);
    return v6;
  }
  v9 = a0;
  switch(*(unsigned int *)(v10 + 0x10)) {
    case 0:
label_ab90:
      v6 = dcgettext(NULL,"invalid expression",5);
      error(1,0,v6);
label_abb4:
      v6 = dcgettext(NULL,"oops -- invalid expression type!",5);
      error(1,0,v6);
      return v6;
    case 1:
      v8 = *(long *)(v10 + 0x108);
      *a0 = v8;
      break;
    case 2:
label_a960:
      *v9 = *(long *)(v10 + 0x108);
      v6 = (unsigned long)sub_a8c0();
      *(char **)(v10 + 0x118) = v6;
      v8 = *a0;
      break;
    case 3:
      goto label_ab64;
    case 4:
      v8 = *(long *)(v10 + 0x108);
      if ((!v8) || (*(char *)(v8 + 0x29))) {
        v5 = *(char **)(v10 + 8);
        v6 = dcgettext(NULL,"invalid expression; expected to find a \')\' but didn\'t see one. Perhaps you need an extra predicate after \'%s\'",5);
        error(1,0,v6,v5);
        return v6;
      }
      v2 = *(int *)(v8 + 0x10);
      *a0 = v8;
      if (v2 != 5) {
label_aa00:
        v6 = (long)sub_a8c0();
        v10 = *a0;
        if ((!v10) || (*(int *)(v10 + 0x10) != 5)) {
          v6 = dcgettext(NULL,"invalid expression; I was expecting to find a \')\' somewhere but did not see one.",5);
          error(1,0,v6);
          return v6;
        }
        v8 = *(long *)(v10 + 0x108);
        *a0 = v8;
        v10 = (long)v6;
        goto label_a983;
      }
      if (*(char *)(v10 + 0x29)) {
        v10 = *(long *)(v8 + 8);
        v6 = dcgettext(NULL,"invalid expression: expected expression before closing parentheses \'%s\'.",5);
        v9 = (long *)0x1;
        error(1,0,v6,v10);
        goto label_a960;
      }
      goto label_ab40;
    case 5:
      if (!a2) {
        v5 = *(char **)(v10 + 8);
        v6 = dcgettext(NULL,"invalid expression: expected expression before closing parentheses \'%s\'.",5);
        error(1,0,v6,v5);
        return v6;
      }
      if (2 <= (unsigned int)(*(int *)((long)a2 + 0x10) - 2U)) { // branch-flip
        if (!*(char *)(v10 + 0x29)) {
          v6 = dcgettext(NULL,"invalid expression; you have too many \')\'",5);
          error(1,0,v6);
          goto label_aa00;
        }
      }
      else if (!*(char *)(v10 + 0x29)) {
        v5 = *(char **)((long)a2 + 8);
        v6 = dcgettext(NULL,"expected an expression between \'%s\' and \')\'",5);
        error(1,0,v6,v5);
        return v6;
      }
      v10 = *(long *)((long)a2 + 8);
      v6 = dcgettext(NULL,"expected an expression after \'%s\'",5);
      error(1,0,v6,v10);
      v8 = v7;
      goto label_aa70;
    default:
      goto label_abb4;
    
  }
label_a983:
  if ((!v8) || (a1 = (int)(short)a1, *(int *)(v8 + 0x14) <= a1)) {
    v6 = (char *)v10;
    return v6;
  }
label_aa70:
  v3 = *(unsigned int *)(v8 + 0x10);
  if (v3 != 5) {
    while (v3 != 4) {
      if (4 < v3) goto label_ab00;
      if (v3 != 3) break;
      *(long *)(v8 + 0x110) = v10;
      v10 = *a0;
      v1 = *(short *)(v10 + 0x14);
      *a0 = *(long *)(v10 + 0x108);
      v6 = (unsigned long)sub_a8c0(a0,(int)v1);
      *(char **)(v10 + 0x118) = v6;
      v8 = *a0;
      if (!v8) {
        v6 = (char *)v10;
        return v6;
      }
      if (*(int *)(v8 + 0x14) <= a1) {
        v6 = (char *)v10;
        return v6;
      }
      v3 = *(unsigned int *)(v8 + 0x10);
    }
    v6 = dcgettext(NULL,"invalid expression",5);
    error(1,0,v6);
label_ab00:
    if (v3 != 5) {
      v6 = dcgettext(NULL,"oops -- invalid expression type (%d)!",5);
      error(1,0,v6,v3);
      return v6;
    }
    if (v10) {
      v6 = (char *)v10;
      return v6;
    }
    v10 = 0;
  }
  v6 = dcgettext(NULL,"invalid expression",5);
  error(1,0,v6);
label_ab40:
  v6 = dcgettext(NULL,"invalid expression; empty parentheses are not allowed.",5);
  error(1,0,v6);
label_ab64:
  v4 = *(unsigned long *)(v10 + 8);
  v6 = dcgettext(NULL,"invalid expression; you have used a binary operator \'%s\' with nothing before it.",5);
  error(1,0,v6,v4);
  goto label_ab90;
}

// Function: sub_acd0 @ 0xacd0
void sub_acd0(unsigned long *a0,unsigned long *a1)
{
  unsigned long v1; // stack - 0x20
  unsigned long v2; // stack - 0x18
  
  v1 = *a0;
  v2 = *a1;
  memcmp(&v1,&v2,8);
}

// Function: sub_ad30 @ 0xad30
void sub_ad30(struct_42 *a0,int a1,unsigned long *a2)
{
  float v1;
  struct_41 *v2; // rax
  float v3;
  
  v2 = (struct_41 *)sub_25e50(0x138);
  v2->field_0x10 = 3;
  v2->field_0x14 = a1;
  v2->field_0x20 = 0;
  if (a1 != 2) { // branch-flip
    if (a1 != 3) {
      if (a1 != 1) {
        v2->field_0x118 = a0;
        *a2 = v2;
        return;
      }
      *a2 = v2;
      v2->field_0x8 = ",";
      v2->field_0x0 = sub_8ca0;
      *(unsigned int *)v2->field_0x24 = 0x3f800000;
      v2->field_0x118 = a0;
      return;
    }
    v1 = a0->field_0x24;
    v2->field_0x8 = "-a";
    v2->field_0x0 = sub_8990;
  }
  else {
    v1 = a0->field_0x24;
    v2->field_0x8 = "-o";
    v2->field_0x0 = sub_8940;
  }
  if (v1 <= dat_29578) {
    v3 = 0.0;
    if (0.0 <= v1)
      v3 = v1;
  }
  else {
    v3 = dat_29578;
  }
  v2->field_0x118 = a0;
  *(float *)v2->field_0x24 = v3;
  *a2 = v2;
}

// Function: sub_ae30 @ 0xae30
void sub_ae30(struct_39 *a0) // return-dupe
{
  char *v1;
  unsigned long v10;
  unsigned long *v11; // r14
  void *v12; // stack - 0x68
  unsigned int v13; // stack - 0x5c
  void *v2;
  void *v3;
  int v4; // eax
  unsigned int v5;
  unsigned long v6; // rcx
  void *v7; // stack - 0x48
  void *v8; // stack - 0x50
  unsigned long v9;
  
  if (!a0)
    return;
  sub_ae30(a0->field_0x118);
  sub_ae30(a0->field_0x110);
  v13 = 3;
  if ((!a0->field_0x1a) && (v13 = 1, !a0->field_0x1c))
    v13 = (unsigned int)a0->field_0x1b + (unsigned int)a0->field_0x1b;
  v12 = a0->field_0x0;
  if ((v12 != sub_8fb0) && (v12 != sub_8fc0)) {
    if (v12 != sub_ff90) { // branch-flip
      if (!dat_386b0) {
        v11 = (unsigned long *)0x380e0;
        qsort(0x380e0,0x3b,0x10,sub_acd0);
        do {
          v2 = (void *)v11[2];
          v3 = (void *)*v11;
          v8 = v2;
          v7 = v3;
          if (memcmp(&v8,&v7,8) < 0) {
            error(1,0,"failed to sort the costlookup array");
            return;
          }
          v8 = v3;
          v7 = v2;
          if (0 < memcmp(&v8,&v7,8))
            __assert_fail("result <= 0","tree.c",0x3f5,"check_sorted"); // no-return
          v11 = &v11[2];
        } while (v11 != (unsigned long *)0x38480);
        dat_386b0 = 1;
        v12 = a0->field_0x0;
      }
      v10 = 0;
      v9 = 0x3b;
      do {
        while( true ) {
          v6 = v10 + v9 >> 1;
          v8 = v12;
          v7 = *(void **)(v6 * 0x10 + 0x380e0);
          v4 = memcmp(&v8,&v7,8);
          if (0 <= v4) break;
          v9 = v6;
          if (v6 <= v10) goto label_af40;
        }
        if (!v4) {
          v5 = *(unsigned int *)(v6 * 0x10 + 0x380e8);
          goto label_afd3;
        }
        v10 = v6 + 1;
      } while (v10 < v9);
label_af40:
      v1 = a0->field_0x8;
      error(0,0,dcgettext(NULL,"warning: there is no entry in the predicate evaluation cost table for predicate %s; please report this as a bug",5),v1);
      v5 = 10;
    }
    else {
      v5 = a0->field_0x20;
    }
label_afd3:
    if (v5 <= v13)
      v5 = v13;
  }
  else {
    v5 = 8 - (unsigned int)a0->field_0x38;
  }
  a0->field_0x20 = v5;
}

// Function: sub_b0c0 @ 0xb0c0
unsigned int sub_b0c0(void *a0)
{
  long v1;
  long v2;
  long v3;
  unsigned int v4; // eax
  unsigned int v5;
  unsigned int v6;
  unsigned int v7;
  unsigned int v8;
  
  v5 = 0;
  v1 = *(long *)((long)a0 + 0x110);
  if (v1) {
    v2 = *(long *)(v1 + 0x110);
    if (v2) {
      v3 = *(long *)(v2 + 0x110);
      if (v3) {
        if (*(long *)(v3 + 0x110)) {
          v4 = sub_b0c0();
          v5 = v4;
        }
        v4 = 0;
        if (*(long *)(v3 + 0x118))
          v4 = sub_b0c0();
        if (v5 < *(unsigned int *)(v3 + 0x20))
          v5 = *(unsigned int *)(v3 + 0x20);
        if (v5 < v4)
          v5 = v4;
      }
      v3 = *(long *)(v2 + 0x118);
      v4 = 0;
      if (v3) {
        v6 = 0;
        if (*(long *)(v3 + 0x110)) {
          v4 = sub_b0c0();
          v6 = v4;
        }
        v4 = 0;
        if (*(long *)(v3 + 0x118))
          v4 = sub_b0c0();
        if (v6 < *(unsigned int *)(v3 + 0x20))
          v6 = *(unsigned int *)(v3 + 0x20);
        if (v4 <= v6)
          v4 = v6;
      }
      if (v5 < *(unsigned int *)(v2 + 0x20))
        v5 = *(unsigned int *)(v2 + 0x20);
      if (v5 < v4)
        v5 = v4;
    }
    v2 = *(long *)(v1 + 0x118);
    v6 = 0;
    if (v2) {
      v3 = *(long *)(v2 + 0x110);
      v7 = 0;
      if (v3) {
        v7 = 0;
        if (*(long *)(v3 + 0x110)) {
          v4 = sub_b0c0();
          v7 = v4;
        }
        v4 = 0;
        if (*(long *)(v3 + 0x118))
          v4 = sub_b0c0();
        if (v7 < *(unsigned int *)(v3 + 0x20))
          v7 = *(unsigned int *)(v3 + 0x20);
        if (v7 < v4)
          v7 = v4;
      }
      v3 = *(long *)(v2 + 0x118);
      v4 = 0;
      if (v3) {
        v6 = 0;
        if (*(long *)(v3 + 0x110)) {
          v4 = sub_b0c0();
          v6 = v4;
        }
        v4 = 0;
        if (*(long *)(v3 + 0x118))
          v4 = sub_b0c0();
        if (v6 < *(unsigned int *)(v3 + 0x20))
          v6 = *(unsigned int *)(v3 + 0x20);
        if (v4 <= v6)
          v4 = v6;
      }
      v6 = *(unsigned int *)(v2 + 0x20);
      if (*(unsigned int *)(v2 + 0x20) <= v7)
        v6 = v7;
      if (v6 < v4)
        v6 = v4;
    }
    if (v5 < *(unsigned int *)(v1 + 0x20))
      v5 = *(unsigned int *)(v1 + 0x20);
    if (v5 < v6)
      v5 = v6;
  }
  v1 = *(long *)((long)a0 + 0x118);
  v4 = 0;
  if (v1) {
    v2 = *(long *)(v1 + 0x110);
    v6 = 0;
    if (v2) {
      v3 = *(long *)(v2 + 0x110);
      if (v3) {
        if (*(long *)(v3 + 0x110)) {
          v4 = sub_b0c0();
          v6 = v4;
        }
        v4 = 0;
        if (*(long *)(v3 + 0x118))
          v4 = sub_b0c0();
        if (v6 < *(unsigned int *)(v3 + 0x20))
          v6 = *(unsigned int *)(v3 + 0x20);
        if (v6 < v4)
          v6 = v4;
      }
      v3 = *(long *)(v2 + 0x118);
      v7 = 0;
      if (v3) {
        v8 = 0;
        if (*(long *)(v3 + 0x110)) {
          v4 = sub_b0c0();
          v8 = v4;
        }
        v4 = 0;
        if (*(long *)(v3 + 0x118))
          v4 = sub_b0c0();
        v7 = *(unsigned int *)(v3 + 0x20);
        if (*(unsigned int *)(v3 + 0x20) <= v8)
          v7 = v8;
        if (v7 < v4)
          v7 = v4;
      }
      if (v6 < *(unsigned int *)(v2 + 0x20))
        v6 = *(unsigned int *)(v2 + 0x20);
      if (v6 < v7)
        v6 = v7;
    }
    v2 = *(long *)(v1 + 0x118);
    v7 = 0;
    if (v2) {
      v3 = *(long *)(v2 + 0x110);
      v8 = 0;
      if (v3) {
        if (*(long *)(v3 + 0x110)) {
          v4 = sub_b0c0();
          v8 = v4;
        }
        v4 = 0;
        if (*(long *)(v3 + 0x118))
          v4 = sub_b0c0();
        if (v8 < *(unsigned int *)(v3 + 0x20))
          v8 = *(unsigned int *)(v3 + 0x20);
        if (v8 < v4)
          v8 = v4;
      }
      v3 = *(long *)(v2 + 0x118);
      v4 = 0;
      if (v3) {
        v7 = 0;
        if (*(long *)(v3 + 0x110)) {
          v4 = sub_b0c0();
          v7 = v4;
        }
        v4 = 0;
        if (*(long *)(v3 + 0x118))
          v4 = sub_b0c0();
        if (v7 < *(unsigned int *)(v3 + 0x20))
          v7 = *(unsigned int *)(v3 + 0x20);
        if (v4 <= v7)
          v4 = v7;
      }
      v7 = *(unsigned int *)(v2 + 0x20);
      if (*(unsigned int *)(v2 + 0x20) <= v8)
        v7 = v8;
      if (v7 < v4)
        v7 = v4;
    }
    v4 = *(unsigned int *)(v1 + 0x20);
    if (*(unsigned int *)(v1 + 0x20) <= v6)
      v4 = v6;
    if (v4 < v7)
      v4 = v7;
  }
  if (v5 < *(unsigned int *)((long)a0 + 0x20))
    v5 = *(unsigned int *)((long)a0 + 0x20);
  if (v4 <= v5)
    v4 = v5;
  return v4;
}

// Function: sub_b430 @ 0xb430
unsigned long sub_b430(void *a0)
{
  long v1;
  unsigned long v2; // rax
  
  do {
    if (!a0)
      return 0;
    if (*(unsigned char *)((long)a0 + 0x18))
      return (unsigned long)*(unsigned char *)((long)a0 + 0x18);
    v1 = *(long *)((long)a0 + 0x110);
    if (v1) {
      if (*(unsigned char *)(v1 + 0x18))
        return (unsigned long)*(unsigned char *)(v1 + 0x18);
      v2 = sub_b430(*(unsigned long *)(v1 + 0x110));
      if ((char)v2)
        return v2;
      v2 = sub_b430(*(unsigned long *)(v1 + 0x118));
      if ((char)v2)
        return v2;
    }
    a0 = *(void **)((long)a0 + 0x118);
  } while( true );
}

// Function: sub_b4b0 @ 0xb4b0
unsigned long sub_b4b0(char *a0,unsigned char a1)
{
  char *v1;
  int v2; // eax
  unsigned long v3; // rbx
  
  if (!dat_386b8) {
    v2 = fnmatch(a0,".",(unsigned int)a1 << 4);
    return CONCAT71((undefined7)(CONCAT44(dat_4,v2) >> 8),v2 == 0);
  }
  v3 = 0;
  do {
    v1 = *(char **)(dat_386c0 + v3 * 8);
    if (!fnmatch(a0,v1,(unsigned int)a1 << 4))
      return 1;
    v3 += 1;
  } while (v3 < dat_386b8);
  return 0;
}

// Function: sub_b540 @ 0xb540
void sub_b540(FILE *a0,void *a1)
{
  if (*(char **)((long)a1 + 0x30)) {
    __fprintf_chk(a0,1,"%s %s",*(char **)((long)a1 + 8),*(char **)((long)a1 + 0x30)); // tail-call
    return;
  }
  __fprintf_chk(a0,1,"%s",*(char **)((long)a1 + 8)); // tail-call
}

// Function: sub_b580 @ 0xb580
unsigned long sub_b580(void)
{
  return dat_386d0;
}

// Function: sub_b590 @ 0xb590
float sub_b590(struct_40 *a0)
{
  unsigned int v1;
  long v2;
  long v3;
  void *v4;
  float v5;
  float v6;
  float v7;
  
  if (!a0)
    __assert_fail("NULL != p","tree.c",0x484,"calculate_derived_rates"); // no-return
  v2 = a0->field_0x118;
  if (v2)
    sub_b590(v2);
  v3 = a0->field_0x110;
  if (v3)
    sub_b590(v3);
  v1 = a0->field_0x10;
  if (v1 == 5)
    __assert_fail("p->p_type != CLOSE_PAREN","tree.c",0x48b,"calculate_derived_rates"); // no-return
  if (v1 == 4)
    __assert_fail("p->p_type != OPEN_PAREN","tree.c",0x48c,"calculate_derived_rates"); // no-return
  if (v1 == 2) {
    if (a0->field_0x0 != sub_9350)
      __assert_fail("pred_is (p, pred_negate)","tree.c",0x49c,"calculate_derived_rates"); // no-return
    if (v3)
      __assert_fail("NULL == p->pred_left","tree.c",0x49d,"calculate_derived_rates"); // no-return
    v6 = dat_29578 - *(float *)(v2 + 0x24);
    a0->field_0x24 = v6;
    return v6;
  }
  if (v1 <= 2) {
    if (v1) { // branch-flip
      if (v2)
        __assert_fail("NULL == p->pred_right","tree.c",0x496,"calculate_derived_rates"); // no-return
      if (v3)
        __assert_fail("NULL == p->pred_left","tree.c",0x497,"calculate_derived_rates"); // no-return
    }
    else {
      if (v2)
        __assert_fail("NULL == p->pred_right","tree.c",0x491,"calculate_derived_rates"); // no-return
      if (v3)
        __assert_fail("NULL == p->pred_left","tree.c",0x492,"calculate_derived_rates"); // no-return
    }
    return a0->field_0x24;
  }
  if (v1 != 3)
    __assert_fail("0","tree.c",0x4c0,"calculate_derived_rates"); // no-return
  v4 = a0->field_0x0;
  v6 = dat_29578;
  if (v4 != sub_8990) { // branch-flip
    if (v4 == sub_8ca0) goto label_b678;
    if (v4 != sub_8940)
      __assert_fail("0","tree.c",0x4b4,"calculate_derived_rates"); // no-return
    if (v2) // branch-flip
      v7 = *(float *)(v2 + 0x24);
    else {
      v7 = dat_29578;
    }
    v5 = dat_29578;
    if (v3)
      v5 = *(float *)(v3 + 0x24);
    v5 = v5 + v7;
  }
  else {
    if (v2) // branch-flip
      v5 = *(float *)(v2 + 0x24);
    else {
      v5 = dat_29578;
    }
    if (v3)
      v5 = v5 * *(float *)(v3 + 0x24);
  }
  if (v5 <= dat_29578) {
    v6 = 0.0;
    if (0.0 <= v5)
      v6 = v5;
  }
label_b678:
  a0->field_0x24 = v6;
  return v6;
}

// Function: sub_b8a0 @ 0xb8a0
void sub_b8a0(int *a0)
{
  long v1; // rax
  
  if (!*a0)
    __assert_fail("entry->type != ARG_OPTION","tree.c",0x5cd,"get_new_pred"); // no-return
  if (*a0 != 2) {
    v1 = sub_25e50(0x138);
    if (dat_386d8) // branch-flip
      *(long *)(dat_386c8 + 0x108) = v1;
    else {
      dat_386d8 = v1;
    }
    *(int **)(v1 + 0x130) = a0;
    *(unsigned short *)(v1 + 0x1a) = 0x101;
    *(unsigned long *)(v1 + 0x10) = 0;
    *(char **)(v1 + 0x30) = "ThisShouldBeSetToSomethingElse";
    *(char *)(v1 + 0x28) = dat_3862f;
    *(unsigned long *)(v1 + 0x20) = 0x3f8000000000000a;
    *(unsigned long *)(v1 + 0x128) = 0;
    *(unsigned long *)(v1 + 0x120) = 0;
    dat_386c8 = v1;
    return;
  }
  __assert_fail("entry->type != ARG_POSITIONAL_OPTION","tree.c",0x5ce,"get_new_pred"); // no-return
}

// Function: sub_b990 @ 0xb990
void sub_b990(int *a0) // return-dupe
{
  long v1; // rax
  
  v1 = sub_b8a0(a0);
  if (!v1)
    return;
  *(unsigned long *)(v1 + 0x30) = 0;
}

// Function: sub_b9b0 @ 0xb9b0
void sub_b9b0(int *a0,unsigned long a1)
{
  long v1;
  struct_44 *v2; // rax
  
  v1 = dat_386a0;
  if (!dat_386a0) {
    v1 = sub_14bc0("and");
    dat_386a0 = v1;
    if (!v1)
      __assert_fail("entry_and != NULL","tree.c",0x5f8,"get_new_pred_chk_op"); // no-return
  }
  if (dat_386c8) {
    if ((*(unsigned int *)(dat_386c8 + 0x10) & 0xfffffffb) != 1) { // branch-flip
      if (!*(unsigned int *)(dat_386c8 + 0x10)) {
        error(1,0,dcgettext(NULL,"oops -- invalid default insertion of and!",5));
        return;
      }
    }
    else {
      v2 = (struct_44 *)sub_b8a0(v1);
      v2->field_0x8 = "-a";
      v2->field_0x0 = sub_8990;
      v2->field_0x30 = 0;
      v2->field_0x38 = 0;
      v2->field_0x10 = 0x300000003;
      v2->field_0x18 = 0;
      v2->field_0x1c = 0;
    }
  }
  v1 = sub_b8a0(a0);
  *(unsigned long *)(v1 + 0x30) = a1;
  *(int **)(v1 + 0x130) = a0;
}

// Function: sub_bac0 @ 0xbac0
void sub_bac0(FILE *a0,void *a1,int a2)
{
  char v1;
  char *v10;
  int v11; // ecx
  short v2; // ax
  unsigned int v3; // eax
  long v4; // rax
  long v5;
  unsigned long v6; // rax
  char *v7;
  short v8;
  int *v9; // rdx
  
  if (!a1)
    return;
  do {
    v11 = 0;
    if (1 <= a2) {
      do {
        __fprintf_chk(a0,1,"    ");
        v11 += 1;
      } while (v11 != a2);
    }
    __fprintf_chk(a0,1,"pred=[");
    if (*(char **)((long)a1 + 0x30)) // branch-flip
      __fprintf_chk(a0,1,"%s %s",*(char **)((long)a1 + 8),*(char **)((long)a1 + 0x30));
    else {
      __fprintf_chk(a0,1,"%s",*(char **)((long)a1 + 8));
    }
    v10 = "no";
    v2 = (short)*(unsigned int *)((long)a1 + 0x14);
    if (v2) {
      v8 = 1;
      v11 = 0;
      v5 = 0x36100;
      while( true ) {
        v11 += 1;
        v4 = v5 + 0x10;
        if ((v2 == v8) || (v8 == -1)) break;
        v8 = *(short *)(v5 + 0x20);
        v5 = v4;
      }
      v10 = *(char **)((long)v11 * 0x10 + 0x36108);
    }
    v7 = "no";
    v2 = (short)*(unsigned int *)((long)a1 + 0x10);
    if (v2) {
      v8 = 1;
      v11 = 0;
      v5 = 0x36080;
      while( true ) {
        v11 += 1;
        v4 = v5 + 0x10;
        if ((v2 == v8) || (v8 == -1)) break;
        v8 = *(short *)(v5 + 0x20);
        v5 = v4;
      }
      v7 = *(char **)((long)v11 * 0x10 + 0x36088);
    }
    __fprintf_chk(a0,1,"] type=%s prec=%s",v7,v10);
    v10 = "";
    v9 = (int *)0x38020;
    if (!*(char *)((long)a1 + 0x18))
      v10 = "no ";
    v6 = 0;
    do {
      if (*(int *)((long)a1 + 0x20) == *v9) {
        v7 = *(char **)(v6 * 0x10 + 0x38028);
        goto label_bc5b;
      }
      v3 = (int)v6 + 1;
      v6 = (unsigned long)v3;
      v9 = &v9[4];
    } while (v3 != 0xb);
    v7 = "unknown";
label_bc5b:
    __fprintf_chk(a0,1," cost=%s est_success_rate=%#.4g %sside effects ",v7,(double)*(float *)((long)a1 + 0x24),v10);
    if (*(unsigned long *)((long)a1 + 0x18) & 0xffffff0000) {
      __fprintf_chk(a0,1,"Needs ");
      if (*(char *)((long)a1 + 0x1a)) { // branch-flip
        __fprintf_chk(a0,1,"stat");
        if (*(char *)((long)a1 + 0x1c)) { // branch-flip
          v10 = ",";
label_bdc5:
          __fprintf_chk(a0,1,"%sinode",v10);
          v1 = *(char *)((long)a1 + 0x1b);
        }
        else {
          v1 = *(char *)((long)a1 + 0x1b);
        }
        if (!v1) goto label_bc89;
        v10 = ",";
      }
      else {
        if (*(char *)((long)a1 + 0x1c)) {
          v10 = "";
          goto label_bdc5;
        }
        if (!*(char *)((long)a1 + 0x1b)) goto label_bc89;
        v10 = "";
      }
      __fprintf_chk(a0,1,"%stype",v10);
    }
label_bc89:
    v11 = 0;
    __fprintf_chk(a0,1,"\n");
    if (1 <= a2) {
      do {
        __fprintf_chk(a0,1,"    ");
        v11 += 1;
      } while (v11 != a2);
    }
    if (*(long *)((long)a1 + 0x110)) { // branch-flip
      __fprintf_chk(a0,1,"left:\n");
      sub_bac0(a0,*(unsigned long *)((long)a1 + 0x110),a2 + 1);
    }
    else {
      if (!*(long *)((long)a1 + 0x118)) {
        v10 = "no children.\n";
        __fprintf_chk(a0,1,v10); // return-dupe, tail-call
        return;
      }
      __fprintf_chk(a0,1,"no left.\n");
    }
    v11 = 0;
    if (1 <= a2) {
      do {
        __fprintf_chk(a0,1,"    ");
        v11 += 1;
      } while (v11 != a2);
    }
    if (!*(long *)((long)a1 + 0x118)) {
      v10 = "no right.\n";
      __fprintf_chk(a0,1,v10);
      return;
    }
    a2 += 1;
    __fprintf_chk(a0,1,"right:\n");
    a1 = *(void **)((long)a1 + 0x118);
    if (!a1)
      return;
  } while( true );
}

// Function: sub_bef0 @ 0xbef0
void sub_bef0(struct_12 *a0,long *a1) // return-dupe
{
  float v1;
  long v10;
  float v2;
  unsigned int v3;
  unsigned int v4;
  long v5;
  long v6;
  long v7;
  long v8;
  long v9;
  
  v10 = a0->field_0x0;
  if (!v10)
    return;
  if (dat_38660 & 8) { // branch-flip
    __fprintf_chk(stderr,1,"%s:\n","predlist before merge sort");
    sub_bac0(stderr,(void *)a0->field_0x0,2);
    v10 = a0->field_0x0;
    sub_b590(v10);
    if (!v10) {
      v8 = 0;
      v9 = 0;
      goto label_bfd3;
    }
  }
  else {
    sub_b590(v10);
  }
  v8 = 0;
  v9 = 0;
  do {
    v6 = *(long *)(v10 + 0x110);
    a0->field_0x0 = v6;
    *(unsigned long *)(v10 + 0x110) = 0;
    if (v8) {
      v3 = *(unsigned int *)(*(long *)(v10 + 0x118) + 0x20);
      v7 = v8;
      do {
        v4 = *(unsigned int *)(*(long *)(v7 + 0x118) + 0x20);
        if (v4 != v3) {
          if (v4 < v3) goto label_bfb1;
label_bf50:
          v5 = *(long *)(v7 + 0x110);
          *(long *)(v10 + 0x110) = v5;
          if (!v5)
            v9 = v10;
          *(long *)(v7 + 0x110) = v10;
          goto label_bf6f;
        }
        v1 = *(float *)(*(long *)(v7 + 0x118) + 0x24);
        v2 = *(float *)(*(long *)(v10 + 0x118) + 0x24);
        if (v1 == v2) goto label_bf50;
        if (*(int *)(v7 + 0x14) != 2) { // branch-flip
          if (v2 <= v1) goto label_bf50;
        }
        else if (v1 < v2) goto label_bf50;
label_bfb1:
        v7 = *(long *)(v7 + 0x110);
      } while (v7);
    }
    *(long *)(v10 + 0x110) = v8;
    v8 = v10;
    if (!v9)
      v9 = v10;
label_bf6f:
    v10 = v6;
  } while (v6);
label_bfd3:
  if (dat_38660 & 8) {
    __fprintf_chk(stderr,1,"%s:\n","predlist after merge sort");
    sub_bac0(stderr,v8,2);
  }
  sub_b590(v8);
  *(long *)(v9 + 0x110) = *a1;
  *a1 = v8;
  a0->field_0x8 = 0;
  a0->field_0x0 = 0;
}

// Function: sub_c0b0 @ 0xc0b0
void sub_c0b0(long a0,unsigned long a1,struct_12 *a2,unsigned long a3,long *a4)
{
  long v1;
  long v2; // rbx
  
  dat_386a8 = sub_bef0;
  sub_bef0(a2,a4);
  (*dat_386a8)(a3,a4);
  v1 = a0;
  do {
    v2 = v1 + 0x10;
    (*dat_386a8)(v1,a4);
    v1 = v2;
  } while (v2 != a0 + 0xb0);
}

// Function: sub_c110 @ 0xc110
unsigned long sub_c110(long *a0)
{
  int v1;
  long v10;
  char *v11;
  int *v12; // rdx
  int v13;
  long v14; // stack - 0x118
  long v15 [23];
  long v16; // stack - 0x108
  unsigned int v17;
  long v18;
  unsigned long v19;
  void *v2;
  long v20; // stack - 0x110
  long v21; // stack - 0x100
  char *v3;
  unsigned int v4;
  long *v5;
  long *v6; // rax
  unsigned long v7; // rax
  long v8;
  unsigned long v9; // rax
  
  v8 = *a0;
  v14 = 0;
  v20 = 0;
  v16 = 0;
  v21 = 0;
  if (!v8)
    return 0;
  v5 = v15;
  do {
    v5[1] = 0;
    v6 = &v5[2];
    *v5 = 0;
    v5 = v6;
  } while (v6 != &v15[0x16]);
  v10 = v8;
  v18 = *(long *)(v8 + 0x110);
  if (*(long *)(v8 + 0x110)) { // branch-flip
    for (; *(long *)(v18 + 0x110); v18 = *(long *)(v18 + 0x110)) {
      v10 = v18;
    }
    v17 = *(unsigned int *)(v10 + 0x14);
    v5 = (long *)(v10 + 0x110);
  }
  else {
    v17 = 3;
    v5 = a0;
    v18 = v8;
  }
  if (*(int *)(v18 + 0x10) != 3) { // branch-flip
    sub_ad30(v18,v17,v5);
    if (dat_38660 & 9) goto label_c448;
  }
  else {
    if (!(dat_38660 & 9)) goto label_c1e6;
label_c448:
    __fprintf_chk(stderr,1,"Normalized Eval Tree:\n");
    sub_bac0(stderr,(void *)*a0,0);
  }
  v8 = *a0;
  if (!v8) {
    v19 = 0;
    sub_c0b0(v15,0xb,&v16,&v14,a0); // return-dupe
    return v19;
  }
label_c1e6:
  v1 = *(int *)(v8 + 0x10);
  v13 = 0;
  if (v1 == 3)
    v13 = *(int *)(v8 + 0x14);
  v19 = 0;
  do {
    if ((v1 == 3) && (*(int *)(v8 + 0x14) != v13))
      v8 = sub_ad30(v8,v13,a0);
    v5 = *(long **)(v8 + 0x118);
    v4 = *(unsigned int *)&v5[2];
    v2 = (void *)*v5;
    if (v4 != 2) { // branch-flip
      if (2 < v4) {
        if (v4 != 3) {
          error(1,0,dcgettext(NULL,"oops -- invalid expression type!",5));
          return v9;
        }
        *(char *)&v5[3] = sub_c110(v8 + 0x118);
        v5 = *(long **)(v8 + 0x118);
        goto label_c3c0;
      }
      if (v13 == 1) goto label_c3c0;
      if ((char)v5[3]) goto label_c3c7;
      if ((((v2 != sub_9310 && v2 != sub_9640) && v2 != sub_9150) && (v2 != sub_91e0)) && ((((v2 == sub_9350 || v2 == sub_8990) || !dat_38688) || v2 == sub_8ca0 || ((v2 == sub_8940 || ((int)v5[4])))))) {
        if (v2 == sub_97e0) {
          *a0 = *(long *)(v8 + 0x110);
          *(long *)(v8 + 0x110) = v14;
          v14 = v8;
          if (!v20)
            v20 = v8;
          goto label_c3ec;
        }
        if ((2 <= dat_38688) && (((v4 = *(unsigned int *)&v5[4], v4 - 1 < 2 && (!*(char *)((long)v5 + 0x1a))) || (dat_38688 != 2)))) {
          if (dat_38660 & 8) {
            __fprintf_chk(stderr,1,"-O%d: categorising predicate ",(unsigned int)dat_38688);
            v11 = *(char **)(*(long *)(v8 + 0x118) + 0x30);
            v3 = *(char **)(*(long *)(v8 + 0x118) + 8);
            if (v11) // branch-flip
              __fprintf_chk(stderr,1,"%s %s",v3,v11);
            else {
              __fprintf_chk(stderr,1,"%s",v3);
            }
            v12 = (int *)0x38020;
            v7 = 0;
            do {
              if (*(int *)(*(long *)(v8 + 0x118) + 0x20) == *v12) {
                v11 = *(char **)(v7 * 0x10 + 0x38028);
                goto label_c617;
              }
              v4 = (int)v7 + 1;
              v7 = (unsigned long)v4;
              v12 = &v12[4];
            } while (v4 != 0xb);
            v11 = "unknown";
label_c617:
            __fprintf_chk(stderr,1," by cost (%s)\n",v11);
            v4 = *(unsigned int *)(*(long *)(v8 + 0x118) + 0x20);
          }
          v7 = (unsigned long)v4;
          v5 = &v15[v7 * 2];
          *a0 = *(long *)(v8 + 0x110);
          *(long *)(v8 + 0x110) = *v5;
          *v5 = v8;
          if (!v15[v7 * 2 + 1])
            v15[v7 * 2 + 1] = v8;
          goto label_c3ec;
        }
        goto label_c3e5;
      }
      if (dat_38660 & 8) {
        __fprintf_chk(stderr,1,"-O%d: promoting cheap predicate ",(unsigned int)dat_38688);
        v11 = *(char **)(*(long *)(v8 + 0x118) + 0x30);
        v3 = *(char **)(*(long *)(v8 + 0x118) + 8);
        if (v11) // branch-flip
          __fprintf_chk(stderr,1,"%s %s",v3,v11);
        else {
          __fprintf_chk(stderr,1,"%s",v3);
        }
        __fprintf_chk(stderr,1," into name_list\n");
      }
      *a0 = *(long *)(v8 + 0x110);
      *(long *)(v8 + 0x110) = v16;
      v16 = v8;
      if (!v21)
        v21 = v8;
    }
    else {
      *(char *)&v5[3] = sub_c110(&v5[0x23]);
      v5 = *(long **)(v8 + 0x118);
label_c3c0:
      if ((char)v5[3]) {
label_c3c7:
        v19 = 1;
        sub_c0b0(v15,0xb,&v16,&v14,a0);
      }
label_c3e5:
      a0 = (long *)(v8 + 0x110);
    }
label_c3ec:
    v8 = *a0;
    if (!v8) {
      sub_c0b0(v15,0xb,&v16,&v14,a0);
      return v19;
    }
    v1 = *(int *)(v8 + 0x10);
  } while( true );
}

// Function: sub_c690 @ 0xc690
long sub_c690(struct_43 *a0)
{
  float v1;
  char *v10;
  unsigned int v11;
  void *v12; // rdx
  unsigned int v13;
  void *v14;
  unsigned long v15;
  unsigned int v16;
  long v17;
  long v18;
  bool v19; // zf
  float v2;
  int v3;
  void *v4;
  void *v5;
  void *v6;
  char v7; // al
  unsigned int v8; // eax
  long v9; // rax
  
  if (!a0) {
    v9 = 0;
    return v9;
  }
  do {
    v11 = (unsigned int)dat_38660 & 8;
    v10 = "Not a binary operation";
    if (a0->field_0x10 == 3) {
      v9 = a0->field_0x110;
      v15 = dat_38660;
      do {
        v11 = (unsigned int)v15;
        if ((!v9) || (v18 = a0->field_0x118, !v18)) {
          v10 = "Doesn\'t have two arms";
          v11 &= 8;
          goto label_c6d8;
        }
        v17 = *(long *)(v9 + 0x118);
        if (!v17) {
          v10 = "Left arm has no child on RHS";
          v11 &= 8;
          goto label_c6d8;
        }
        if (*(char *)(v17 + 0x18)) {
label_c774:
          v11 &= 8;
          v10 = "Left subtree has side-effects";
          goto label_c6d8;
        }
        v7 = sub_b430(*(void **)(v17 + 0x110));
        v11 = (unsigned int)v15;
        if (v7) goto label_c774;
        v14 = *(void **)(v17 + 0x118);
        v7 = sub_b430(v14);
        v11 = (unsigned int)v15;
        if (v7) goto label_c774;
        if (*(char *)(v18 + 0x18)) {
label_c7ed:
          v11 &= 8;
          v10 = "Right subtree has side-effects";
          goto label_c6d8;
        }
        v4 = *(void **)(v18 + 0x110);
        v7 = sub_b430(v4);
        v11 = (unsigned int)v15;
        if (v7) goto label_c7ed;
        v5 = *(void **)(v18 + 0x118);
        v7 = sub_b430(v5);
        v11 = (unsigned int)v15;
        if (v7) goto label_c7ed;
        v16 = 0;
        if (v12) {
          v8 = sub_b0c0(v12);
          v16 = v8;
        }
        v8 = 0;
        if (v14)
          v8 = sub_b0c0(v14);
        if (v16 < *(unsigned int *)(v17 + 0x20))
          v16 = *(unsigned int *)(v17 + 0x20);
        if (v16 < v8)
          v16 = v8;
        v13 = 0;
        if (v4) {
          v8 = sub_b0c0(v4);
          v13 = v8;
        }
        v8 = 0;
        if (v5)
          v8 = sub_b0c0(v5);
        if (v13 < *(unsigned int *)(v18 + 0x20))
          v13 = *(unsigned int *)(v18 + 0x20);
        if (v13 < v8)
          v13 = v8;
        v11 = (unsigned int)v15 & 8;
        if ((int)v16 < (int)v13) {
          v10 = "efficient as-is";
          goto label_c6d8;
        }
        if (v16 == v13) {
          v1 = *(float *)(v17 + 0x24);
          v2 = *(float *)(v18 + 0x24);
          if (v15 & 8) { // branch-flip
            __fprintf_chk(stderr,1,"Success rates: l=%f, r=%f\n");
            v6 = a0->field_0x0;
            v11 = (unsigned int)dat_38660 & 8;
            v15 = dat_38660;
          }
          else {
            v6 = a0->field_0x0;
          }
          if (v6 != sub_8940) { // branch-flip
            if (v6 != sub_8990) {
              v10 = "Not \'AND\' or \'OR\'";
              goto label_c6d8;
            }
            if (v2 <= v1) {
              v10 = "Operation is AND; right success rate <= left";
              goto label_c6d8;
            }
          }
          else if (v1 <= v2) {
            v10 = "Operation is OR; right success rate >= left";
            goto label_c6d8;
          }
        }
        v19 = v11 != 0;
        v11 = 0;
        if (v19) {
          __fprintf_chk(stderr,1,"Performing arm swap on:\n");
          sub_bac0(stderr,a0,0);
          v11 = (unsigned int)dat_38660 & 8;
          v15 = dat_38660;
        }
        v9 = a0->field_0x110;
        v3 = a0->field_0x10;
        v18 = *(long *)(v9 + 0x118);
        *(long *)(v9 + 0x118) = a0->field_0x118;
        a0->field_0x118 = v18;
      } while (v3 == 3);
      v10 = "Not a binary operation";
    }
label_c6d8:
    if (v11) {
      __fprintf_chk(stderr,1,"Not an arm swap candidate (%s):\n",v10);
      sub_bac0(stderr,a0,0);
    }
    v7 = sub_c690(a0->field_0x110);
    if ((!v7) && (v7 = sub_c690(a0->field_0x118), !v7)) {
      v9 = 0;
      return v9;
    }
  } while( true );
}

// Function: sub_c9f0 @ 0xc9f0
int * sub_c9f0(int a0,long a1,int a2)
{
  long *v1;
  long *v10; // stack - 0x48
  unsigned long v11;
  int v12;
  bool v13; // zf
  long v14; // stack - 0x70
  char *v2;
  char *v3;
  int *v4;
  int *v5;
  char v6; // al
  int *v7; // rax
  int v8 [4]; // stack - 0x5c
  int v9; // stack - 0x4c
  
  v7 = (long)(long)a2;
  dat_386d8 = NULL;
  dat_386c0 = a1 + (long)v7 * 8;
  v8[0] = a0;
  v9 = a2;
  if (a2 < a0) {
    while (v6 = sub_e120(*(char **)(a1 + (long)v7 * 8),1), !v6) {
      dat_386b8 += 1;
      v9 += 1;
      if (v8[0] <= v9) break;
      v7 = (long)(long)v9;
    }
  }
  v7 = (int *)sub_14bc0("(");
  v4 = v7;
  v7 = (long)sub_14bc0(")");
  v14 = (long)v7;
  v7 = (int *)sub_14bc0("print");
  v5 = v7;
  if (!v4)
    __assert_fail("entry_open != NULL","tree.c",0x4f6,"build_expression_tree"); // no-return
  if (!v14)
    __assert_fail("entry_close != NULL","tree.c",0x4f7,"build_expression_tree"); // no-return
  if (!v7)
    __assert_fail("entry_print != NULL","tree.c",0x4f8,"build_expression_tree"); // no-return
  sub_12c60(v4,a1,v8);
  *(char **)(dat_386c8 + 8) = "(";
  *(char *)((long)dat_386d8 + 0x29) = 1;
  sub_14ba0(a1,v8[0]);
  sub_9e60(dat_386c8);
  if (v9 < v8[0]) {
    do {
      v7 = (long)(long)v9;
      dat_38618 = 0;
      v6 = sub_e120(*(char **)(a1 + (long)v7 * 8),0);
      v13 = v6 == '\0';
      v7 = (long)(long)v9;
      v2 = *(char **)(a1 + (long)v7 * 8);
      if (v13) {
        v7 = (char *)dcgettext(NULL,"paths must precede expression: `%s\'",5);
        error(0,0,(char *)v7,v2);
        v7 = (long)(long)v9;
        v2 = *(char **)(a1 + (long)v7 * 8);
        if (!access(v2,0)) {
          v2 = *(char **)(dat_386c8 + 8);
          v7 = (char *)dcgettext(NULL,"possible unquoted pattern after predicate `%s\'?",5);
          error(0,0,(char *)v7,v2);
        }
        exit(1); // no-return
      }
      v7 = (int *)sub_14bc0(v2);
      v4 = v7;
      if (!v7) {
        v7 = (char *)dcgettext(NULL,"unknown predicate `%s\'",5);
        error(1,0,(char *)v7,v2);
        return v7;
      }
      v12 = v9;
      if (*v7 != 4) {
        v12 = v9 + 1;
        v9 = v12;
      }
      v6 = (**(void **)&v7[4])(v7,a1,&v9);
      if (!v6) {
        v3 = *(char **)(a1 + (long)v9 * 8);
        if (!v3) {
          v7 = (char *)dcgettext(NULL,"missing argument to `%s\'",5);
          error(1,0,(char *)v7,v2);
          return v7;
        }
        if ((*v4 == 4) && (v9 == v12)) {
          v7 = (char *)dcgettext(NULL,"invalid predicate `%s\'",5);
          error(1,0,(char *)v7,v2);
          return v7;
        }
        v7 = (char *)dcgettext(NULL,"invalid argument `%s\' to `%s\'",5);
        error(1,0,(char *)v7,v3,v2);
        return v7;
      }
      v7 = NULL;
      *(char **)(dat_386c8 + 8) = v2;
      if (v9 != v12)
        v7 = *(int **)(a1 + (long)v12 * 8);
      *(int **)(dat_386c8 + 0x30) = v7;
      sub_9e60(dat_386c8);
      sub_9e60(dat_386d8);
    } while (v9 < v8[0]);
  }
  sub_14bb0(a1,v8[0],dat_386c8,dat_386d8);
  v1 = dat_386d8;
  if ((void *)dat_386d8[0x21]) { // branch-flip
    v6 = sub_e0e0((void *)dat_386d8[0x21]);
    if (v6) { // branch-flip
      sub_12a60(v14,a1,v8);
      *(char **)(dat_386c8 + 8) = ")";
      *(char *)(dat_386c8 + 0x29) = 1;
      sub_9e60(dat_386c8);
      sub_11e20(v5,a1,v8);
      *(char **)(dat_386c8 + 8) = "-print";
      *(char *)(dat_386c8 + 0x29) = 1;
      sub_9e60(dat_386c8);
      sub_9e60(dat_386d8);
    }
    else {
      v1 = &dat_386d8[0x21];
      v10 = dat_386d8;
      dat_386d8 = (long *)*v1;
      sub_9e60((long *)*v1);
      free(v10);
    }
  }
  else {
    v10 = dat_386d8;
    dat_386c8 = 0;
    dat_386d8 = NULL;
    free(v1);
    sub_11e20(v5,a1,v8);
    *(char **)(dat_386c8 + 8) = "-print";
    sub_9e60(dat_386c8);
    sub_9e60(dat_386d8);
  }
  if (dat_38660 & 9) {
    __fprintf_chk(stderr,1,"Predicate List:\n");
    sub_9cd0(stderr,dat_386d8);
  }
  sub_14a70(dat_386d8);
  sub_9e60(dat_386d8);
  v10 = dat_386d8;
  v7 = (unsigned long)sub_a8c0(&v10,0,NULL);
  dat_386d0 = v7;
  sub_b590(v7);
  if (!v10) {
    if (dat_38660 & 9) {
      __fprintf_chk(stderr,1,"Eval Tree:\n");
      sub_bac0(stderr,dat_386d0,0);
      v7 = (int *)dat_386d0;
    }
    sub_ae30(v7);
    sub_c110((long *)0x386d0);
    v11 = dat_386d0;
    sub_a840(dat_386d0,1);
    sub_c690(v11);
    v11 = dat_386d0;
    sub_a840(dat_386d0,1);
    if (dat_38660 & 9) {
      __fprintf_chk(stderr,1,"Optimized Eval Tree:\n");
      sub_bac0(stderr,dat_386d0,0);
      __fprintf_chk(stderr,1,"Optimized command line:\n");
      sub_9de0(stderr,dat_386d0);
      __fprintf_chk(stderr,1,"\n");
      v11 = dat_386d0;
    }
    v7 = (int *)v11;
    return v7;
  }
  if ((void *)*v10 == sub_8be0) {
    v7 = (char *)dcgettext(NULL,"you have too many \')\'",5);
    error(1,0,(char *)v7);
    return v7;
  }
  v2 = (char *)v10[1];
  if (v2) {
    v7 = (char *)dcgettext(NULL,"unexpected extra predicate \'%s\'",5);
    error(1,0,(char *)v7,v2);
    return v7;
  }
  v7 = (char *)dcgettext(NULL,"unexpected extra predicate",5);
  error(1,0,(char *)v7);
  return v7;
}

// Function: sub_d0f0 @ 0xd0f0
void sub_d0f0(struct_32 *a0) // return-dupe
{
  void *v1;
  
  v1 = a0->field_0x0;
  if (((v1 != sub_9020 && v1 != sub_ff90) && v1 != sub_8fe0) && (v1 != sub_9060))
    return;
  a0->field_0x40 = 0;
}

// Function: sub_d130 @ 0xd130
void sub_d130(long *a0) // return-dupe
{
  if (!a0)
    return;
  do {
    while( true ) {
      if (!dat_38608)
        __assert_fail("state.execdirs_outstanding","util.c",0x18c,"do_complete_pending_execdirs"); // no-return
      sub_d130(a0[0x22]);
      if (((((void *)*a0 == sub_8fc0) || ((void *)*a0 == sub_95d0)) && ((char)a0[7])) && ((int)a0[0x1a])) break;
label_d180:
      a0 = (long *)a0[0x23];
      if (!a0)
        return;
    }
    sub_159f0((struct_9 *)a0[0x1f],sub_d220,&a0[7]);
    if ((struct_10 *)a0[0x1f] == dat_385c0) goto label_d180;
    sub_242c0((struct_10 *)a0[0x1f]);
    free((void *)a0[0x1f]);
    a0[0x1f] = 0;
    a0 = (long *)a0[0x23];
    if (!a0)
      return;
  } while( true );
}

// Function: sub_d220 @ 0xd220
unsigned long sub_d220(long a0)
{
  sub_15430((void *)(a0 + 8),(struct_5 *)(a0 + 0x60));
  return 0;
}

// Function: sub_d240 @ 0xd240
void sub_d240(long *a0)
{
  if (!a0)
    return;
  do {
    sub_d240(a0[0x22]);
    if ((((void *)*a0 == sub_8fb0) && ((char)a0[7])) && ((int)a0[0x1a]))
      sub_15430(&a0[8],(struct_5 *)&a0[0x13]);
    a0 = (long *)a0[0x23];
  } while (a0);
}

// Function: sub_d2b0 @ 0xd2b0
void sub_d2b0(void)
{
  __assert_fail("entry->pred_func != NULL","util.c",0x7b,"insert_primary"); // no-return
}

// Function: sub_d2e0 @ 0xd2e0
void sub_d2e0(void *a0,void *a1)
{
  long v1;
  long v2;
  long v3;
  long v4;
  long v5;
  long v6;
  long v7;
  long v8;
  void *v9; // stack - 0x40
  
  v9 = a0;
  do {
    for (v1 = *(long *)((long)v9 + 0x110); v1; v1 = *(long *)(v1 + 0x118)) {
      for (v2 = *(long *)(v1 + 0x110); v2; v2 = *(long *)(v2 + 0x118)) {
        for (v3 = *(long *)(v2 + 0x110); v3; v3 = *(long *)(v3 + 0x118)) {
          for (v4 = *(long *)(v3 + 0x110); v4; v4 = *(long *)(v4 + 0x118)) {
            for (v5 = *(long *)(v4 + 0x110); v5; v5 = *(long *)(v5 + 0x118)) {
              for (v6 = *(long *)(v5 + 0x110); v6; v6 = *(long *)(v6 + 0x118)) {
                for (v7 = *(long *)(v6 + 0x110); v7; v7 = *(long *)(v7 + 0x118)) {
                  for (v8 = *(long *)(v7 + 0x110); v8; v8 = *(long *)(v8 + 0x118)) {
                    if (*(long *)(v8 + 0x110))
                      sub_d2e0(*(long *)(v8 + 0x110),a1);
                    (*a1)(v8);
                  }
                  (*a1)(v7);
                }
                (*a1)(v6);
              }
              (*a1)(v5);
            }
            (*a1)(v4);
          }
          (*a1)(v3);
        }
        (*a1)(v2);
      }
      (*a1)(v1);
    }
    (*a1)(v9);
    v9 = *(void **)((long)v9 + 0x118);
  } while (v9);
}

// Function: sub_d490 @ 0xd490
void sub_d490(struct_55 *a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1;
  struct_54 *v2; // rax
  
  v2 = (struct_54 *)sub_b9b0(a0,a2);
  v1 = a0->field_0x8;
  v2->field_0x0 = a1;
  v2->field_0x8 = v1;
  v2->field_0x38 = 0;
  v2->field_0x10 = 1;
}

// Function: sub_d4d0 @ 0xd4d0
void sub_d4d0(int *a0,unsigned long a1)
{
  long v1;
  long v2;
  struct_45 *v3; // rax
  
  v1 = *(long *)&a0[6];
  if (v1) {
    v3 = (struct_45 *)sub_b9b0(a0,a1);
    v2 = *(long *)&a0[2];
    v3->field_0x0 = v1;
    v3->field_0x8 = v2;
    v3->field_0x38 = 0;
    v3->field_0x10 = 1;
    return;
  }
  sub_d2b0(); // no-return
}

// Function: sub_d520 @ 0xd520
void sub_d520(int *a0)
{
  long v1;
  long v2;
  struct_45 *v3; // rax
  
  v1 = *(long *)&a0[6];
  if (v1) {
    v3 = (struct_45 *)sub_b9b0(a0,0);
    v2 = *(long *)&a0[2];
    v3->field_0x0 = v1;
    v3->field_0x8 = v2;
    v3->field_0x38 = 0;
    v3->field_0x10 = 1;
    return;
  }
  sub_d2b0(); // no-return
}

// Function: sub_d570 @ 0xd570
void sub_d570(int a0)
{
  FILE *v1;
  char *v2;
  long v3;
  unsigned long *v4; // rbx
  
  v2 = dat_38ed0;
  if (a0) {
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
    exit(a0); // no-return
  }
  v4 = (unsigned long *)0x36180;
  __fprintf_chk(stdout,1,dcgettext(NULL,"Usage: %s [-H] [-L] [-P] [-Olevel] [-D debugopts] [path...] [expression]\n",5),v2);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nDefault path is the current directory; default expression is -print.\nExpression may consist of: operators, options, tests, and actions.\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nOperators (decreasing precedence; -and is implicit where no others are given):\n      ( EXPR )   ! EXPR   -not EXPR   EXPR1 -a EXPR2   EXPR1 -and EXPR2\n      EXPR1 -o EXPR2   EXPR1 -or EXPR2   EXPR1 , EXPR2\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nPositional options (always true):\n      -daystart -follow -nowarn -regextype -warn\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nNormal options (always true, specified before other expressions):\n      -depth -files0-from FILE -maxdepth LEVELS -mindepth LEVELS\n       -mount -noleaf -xdev -ignore_readdir_race -noignore_readdir_race\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nTests (N can be +N or -N or N):\n      -amin N -anewer FILE -atime N -cmin N -cnewer FILE -context CONTEXT\n      -ctime N -empty -false -fstype TYPE -gid N -group NAME -ilname PATTERN\n      -iname PATTERN -inum N -iwholename PATTERN -iregex PATTERN\n      -links N -lname PATTERN -mmin N -mtime N -name PATTERN -newer FILE\n      -nouser -nogroup -path PATTERN -perm [-/]MODE -regex PATTERN\n      -readable -writable -executable\n      -wholename PATTERN -size N[bcwkMG] -true -type [bcdpflsD] -uid N\n      -used N -user NAME -xtype [bcdpfls]\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nActions:\n      -delete -print0 -printf FORMAT -fprintf FILE FORMAT -print \n      -fprint0 FILE -fprint FILE -ls -fls FILE -prune -quit\n      -exec COMMAND ; -exec COMMAND {} + -ok COMMAND ;\n      -execdir COMMAND ; -execdir COMMAND {} + -okdir COMMAND ;\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"\nOther common options:\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"      --help                   display this help and exit\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"      --version                output version information and exit\n\n",5),v1);
  v1 = stdout;
  fputs(dcgettext(NULL,"Valid arguments for -D:\n",5),v1);
  v3 = 0;
  do {
    if (v3) { // branch-flip
      __fprintf_chk(stdout,1,"%s%s",", ",*v4);
      v1 = stdout;
      if (v3 + 1 == 9) {
        fputs(dcgettext(NULL,"\nUse \'-D help\' for a description of the options, or see find(1)\n\n",5),v1);
        sub_175f0(stdout,dat_38ed0);
        exit(0); // no-return
      }
    }
    else {
      __fprintf_chk(stdout,1,"%s%s","",*v4);
    }
    v4 = &v4[3];
    v3 += 1;
  } while( true );
}

// Function: sub_d810 @ 0xd810
void sub_d810(void)
{
  return;
}

// Function: sub_d820 @ 0xd820
unsigned long sub_d820(char *a0,char *a1,struct_1 *a2) // return-dupe x3
{
  int v1;
  char *v2; // rax
  int *v3; // rax
  
  if (dat_385e4) {
    dat_385e4 = 0x101;
    dat_385e8 = a2->field_0x18;
    return 0;
  }
  v1 = (*dat_38670)(a1,a2);
  if (!v1) {
    if (a2->field_0x18) {
      dat_385e8 = a2->field_0x18;
      dat_385e4 = 0x101;
      return 0;
    }
    v2 = (char *)sub_239f0(0,dat_3868c,a1);
    error(0,0,dcgettext(NULL,"WARNING: file %s appears to have mode 0000",5),v2);
    dat_385e4 = 0x101;
    dat_385e8 = a2->field_0x18;
    dat_38604 = 1;
    return 0;
  }
  v3 = __errno_location();
  v1 = *v3;
  if ((dat_3862e) && (v1 == 2))
    return 0xffffffff;
  if (dat_38618) {
    dat_38618 = 1;
    return 0xffffffff;
  }
  error(0,v1,"%s",(char *)sub_239f0(0,dat_3868c,a0));
  dat_38604 = 1;
  dat_38618 = 1;
  return 0xffffffff;
}

// Function: sub_d920 @ 0xd920
unsigned long sub_d920(void) // return-dupe
{
  int v1; // eax
  char v2 [130];
  char v3 [262];
  float v4; // xmm1_da
  
  if (uname(v2))
    return 1;
  v4 = (float)(double)strtod(v3,0);
  v1 = strcmp("Linux",v2);
  if (v1) {
    v1 = strcmp("FreeBSD",v2);
    if (v1)
      return 1;
    return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),(float)s_2a2d3._1_4_ <= v4);
  }
  return CONCAT71((undefined7)(CONCAT44(dat_4,v1) >> 8),dat_2a2d0 <= v4);
}

// Function: sub_d9f0 @ 0xd9f0
void sub_d9f0(void)
{
  if (!dat_38608)
    return;
  sub_d130((long *)sub_b580());
  dat_38608 = 0;
}

// Function: sub_da20 @ 0xda20
void sub_da20(void)
{
  int v1; // eax
  char *v2; // rax
  char *v3;
  char *v4;
  
  dat_385c0 = (int *)sub_25a50(0x10);
  v1 = sub_24230(dat_385c0);
  if (!v1)
    return;
  v3 = "";
  v4 = "";
  if (*dat_385c0 < 0) {
    v3 = *(char **)&dat_385c0[2];
    v4 = ": ";
    if (!v3) {
      v3 = "";
      v4 = v3;
    }
  }
  v2 = dcgettext(NULL,"Failed to save initial working directory%s%s",5);
  error(1,*__errno_location(),v2,v4,v3);
}

// Function: sub_dab0 @ 0xdab0
void sub_dab0(void)
{
  void *v1;
  int v2;
  long *v3;
  char *v4; // rax
  char *v5;
  long *v6;
  char *v7;
  
  v3 = (long *)sub_b580();
  if (v3) { // branch-flip
    v6 = v3;
    do {
      if ((void *)v6[0x22])
        sub_d2e0((void *)v6[0x22],sub_d240);
      sub_d240(v6);
      v6 = (long *)v6[0x23];
    } while (v6);
    if (dat_38608) {
      sub_d130((long *)sub_b580());
      dat_38608 = '\0';
    }
    sub_eb20(dat_38610);
    do {
      if ((void *)v3[0x22])
        sub_d2e0((void *)v3[0x22],sub_d0f0);
      v1 = (void *)*v3;
      if (((v1 == sub_ff90 || v1 == sub_9020) || v1 == sub_8fe0) || (v1 == sub_9060))
        v3[8] = 0;
      v3 = (long *)v3[0x23];
    } while (v3);
  }
  else {
    sub_eb20(dat_38610);
  }
  v2 = sub_24290(dat_385c0);
  if (!v2) {
    sub_242c0(dat_385c0);
    free(dat_385c0);
    dat_385c0 = NULL;
    if (sub_16220()) {
      sub_16240();
      sub_161f0();
    }
    v2 = sub_180c0(stdout);
    if (v2 != -1)
      return;
    v2 = *__errno_location();
    error(0,v2,"%s",(char *)sub_239f0(0,dat_3868c,"standard output"));
    dat_38604 = 1;
    return;
  }
  v5 = "";
  v7 = "";
  if (*dat_385c0 < 0) {
    v5 = *(char **)&dat_385c0[2];
    v7 = ": ";
    if (!v5) {
      v5 = "";
      v7 = v5;
    }
  }
  v4 = dcgettext(NULL,"Failed to restore initial working directory%s%s",5);
  error(0,*__errno_location(),v4,v7,v5);
  _exit(1); // no-return
}

// Function: sub_dcd0 @ 0xdcd0
unsigned long sub_dcd0(char *a0,stat *a1)
{
  unsigned int v1; // eax
  int v2;
  
  if ((dat_385f8 <= -1) && (dat_385f8 != -100))
    __assert_fail("state.cwd_dir_fd >= 0","util.c",0x25d,"optionh_stat"); // no-return
  if (dat_385e0) {
    v2 = fstatat(dat_385f8,a0,a1,0x100); // tail-call
    return v2;
  }
  v1 = fstatat(dat_385f8,a0,a1,0);
  if (v1) {
    v2 = *__errno_location();
    if ((v2 == 2) || (v2 == 0x14)) {
      if (dat_38660 & 2)
        __fprintf_chk(stderr,1,"fallback_stat(): stat(%s) failed; falling back on lstat()\n",a0);
      v2 = fstatat(dat_385f8,a0,a1,0x100); // tail-call
      return v2;
    }
  }
  return (unsigned long)v1;
}

// Function: sub_ddb0 @ 0xddb0
unsigned long sub_ddb0(char *a0,stat *a1)
{
  unsigned int v1; // eax
  int v2;
  
  if ((dat_385f8 <= -1) && (dat_385f8 != -100))
    __assert_fail("state.cwd_dir_fd >= 0","util.c",0x27c,"optionl_stat"); // no-return
  v1 = fstatat(dat_385f8,a0,a1,0);
  if (v1) {
    v2 = *__errno_location();
    if ((v2 == 2) || (v2 == 0x14)) {
      if (dat_38660 & 2)
        __fprintf_chk(stderr,1,"fallback_stat(): stat(%s) failed; falling back on lstat()\n",a0);
      v2 = fstatat(dat_385f8,a0,a1,0x100); // tail-call
      return v2;
    }
  }
  return (unsigned long)v1;
}

// Function: sub_de70 @ 0xde70
void sub_de70(char *a0,stat *a1)
{
  if ((dat_385f8 <= -1) && (dat_385f8 != -100))
    __assert_fail("(state.cwd_dir_fd >= 0) || (state.cwd_dir_fd==AT_FDCWD)","util.c",0x28d,"optionp_stat"); // no-return
  fstatat(dat_385f8,a0,a1,0x100); // tail-call
}

// Function: sub_dec0 @ 0xdec0
void sub_dec0(char *a0,stat *a1)
{
  dat_386e0 += 1;
  __fprintf_chk(stderr,1,"debug_stat (%s)\n",a0);
  if (dat_38668 == 1) {
    sub_ddb0(a0,a1); // tail-call
    return;
  }
  if (dat_38668 == 2) {
    sub_dcd0(a0,a1); // tail-call
    return;
  }
  if (dat_38668)
    __assert_fail("0","util.c",0x2a5,"debug_stat"); // no-return
  if ((dat_385f8 <= -1) && (dat_385f8 != -100))
    __assert_fail("(state.cwd_dir_fd >= 0) || (state.cwd_dir_fd==AT_FDCWD)","util.c",0x28d,"optionp_stat"); // no-return
  fstatat(dat_385f8,a0,a1,0x100); // tail-call
}

// Function: sub_dfb0 @ 0xdfb0
unsigned int sub_dfb0(void) // early-return x2
{
  if (dat_38668 == 1)
    return 1;
  if (dat_38668 != 2)
    return 0;
  return CONCAT31((undefined3)((unsigned int)dat_385e0 >> 8),dat_385e0 == 0);
}

// Function: sub_dfe0 @ 0xdfe0
unsigned int sub_dfe0(unsigned int *a0,char *a1,char *a2,struct_1 *a3,unsigned int a4) // return-dupe
{
  unsigned int v1;
  
  v1 = *a0;
  if (v1) {
    if (((v1 & 0xf000) == 0xa000) && ((dat_38668 == 1 || ((dat_38668 == 2 && (!dat_385e0)))))) {
      if (sub_d820(a1,a2,a3))
        return 0;
      dat_385e8 = a3->field_0x18;
      *a0 = dat_385e8;
      dat_385e4 = CONCAT11(1,dat_385e4);
      return 1;
    }
    dat_385e4 = CONCAT11(1,dat_385e4);
    a3->field_0x18 = v1;
    dat_385e8 = v1;
    return 1;
  }
  if ((char)a4) {
    dat_385e8 = 0;
    dat_385e4 = 0;
    return a4;
  }
  if (sub_d820(a1,a2,a3))
    return 0;
  dat_385e8 = a3->field_0x18;
  *a0 = dat_385e8;
  dat_385e4 = CONCAT11(1,dat_385e4);
  return 1;
}

// Function: sub_e0e0 @ 0xe0e0
unsigned long sub_e0e0(void *a0)
{
  while( true ) {
    if (!a0)
      return 1;
    if (*(char *)((long)a0 + 0x19)) break;
    a0 = *(void **)((long)a0 + 0x108);
  }
  return 0;
}

// Function: sub_e120 @ 0xe120
unsigned long sub_e120(char *a0,unsigned int a1) // return-dupe
{
  undefined7 v1; // rax
  
  v1 = (undefined7)((unsigned long)((long)*(int *)((unsigned long)(unsigned char)(*a0 - 0x21) * 4 + 0x2a220) + 0x2a220) >> 8);
  switch(*a0) {
    case 0x21:
    case 0x28:
      return CONCAT71(v1,a0[1] == '\0');
    default:
      return 0;
    case 0x29:
    case 0x2c:
      goto label_e150;
    case 0x2d:
      return CONCAT71(v1,a0[1] != '\0');
    
  }
label_e150:
  if (!a0[1])
    return (unsigned long)(a1 ^ 1);
  return 0;
}

// Function: sub_e180 @ 0xe180
unsigned long sub_e180(int a0,long a1)
{
  unsigned char v1;
  unsigned long v10; // rax
  unsigned long v11; // rax
  unsigned long v12; // rax
  unsigned long v13; // rax
  char v14 [2]; // stack - 0x42
  char *v15; // stack - 0x50
  char *v16;
  char *v17;
  unsigned int v18; // r12d
  unsigned long *v19; // r12
  int v2;
  FILE *v3;
  long v4; // rax
  char *v5;
  int *v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v18 = 1;
  if (2 <= a0) {
    do {
      v4 = (long)(int)v18;
      v17 = *(char **)(a1 + v4 * 8);
      if (strcmp("-H",v17)) { // branch-flip
        if (strcmp("-L",v17)) { // branch-flip
          if (strcmp("-P",v17)) { // branch-flip
            if (!strcmp("--",v17)) {
              v18 += 1;
              break;
            }
            if (strcmp("-D",v17)) { // branch-flip
              if (strncmp("-O",v17,2)) break;
              v5 = &v17[2];
              v1 = v17[2];
              if (!v1) {
                error(1,0,dcgettext(NULL,"The -O option must be immediately followed by a decimal integer",5));
                return v13;
              }
              if (!(*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)v1 * 2) & 8)) {
                error(1,0,dcgettext(NULL,"Please specify a decimal number immediately after -O",5));
                return v12;
              }
              v6 = __errno_location();
              v2 = *v6;
              *v6 = 0;
              v7 = strtoul(v5,&v15,10);
              if (v7) { // branch-flip
                if (*v15) {
                  error(1,0,dcgettext(NULL,"Invalid optimisation level %s",5),v5); // return-dupe
                  return v10;
                }
                if (v7 == 0xffffffffffffffff) {
                  if (*v6) {
                    v17 = dcgettext(NULL,"Invalid optimisation level %s",5);
                    error(1,*v6,v17,v5);
                    return v9;
                  }
                  error(1,0,dcgettext(NULL,"Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate.",5),v7); // return-dupe
                  return v8;
                }
                if (0x10000 <= v7) {
                  error(1,0,dcgettext(NULL,"Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate.",5),v7);
                  return v8;
                }
              }
              else {
                if (v5 == v15) {
                  error(1,0,dcgettext(NULL,"Please specify a decimal number immediately after -O",5));
                  return v11;
                }
                if (*v15) {
                  error(1,0,dcgettext(NULL,"Invalid optimisation level %s",5),v5);
                  return v10;
                }
              }
              dat_38688 = (unsigned short)v7;
              *v6 = v2;
            }
            else {
              v18 += 1;
              if (a0 <= (int)v18) {
                v17 = "Missing argument after the -D option.";
label_e4bc:
                error(0,0,dcgettext(NULL,v17,5));
                sub_d570(1); // no-return
              }
              v15 = NULL;
              v14[0] = ',';
              v14[1] = '\0';
              v17 = *(char **)(a1 + 8 + v4 * 8);
              v5 = strtok_r(v17,v14,&v15);
              if (!v5) {
                v17 = "Empty argument to the -D option.";
                goto label_e4bc;
              }
              do {
                v16 = "exec";
                v4 = 0;
                v19 = (unsigned long *)0x36198;
                while (strcmp(v16,v5)) {
                  v4 += 1;
                  if (v4 == 9) {
                    v5 = (char *)sub_239f0(0,dat_3868c,v17);
                    error(0,0,dcgettext(NULL,"Ignoring unrecognised debug flag %s",5),v5);
                    goto label_e31a;
                  }
                  v16 = (char *)*v19;
                  v19 = &v19[3];
                }
                dat_38660 |= (long)*(int *)(v4 * 0x18 + 0x36188);
label_e31a:
                v5 = strtok_r(NULL,v14,&v15);
                v3 = stdout;
              } while (v5);
              if (dat_38660 & 0x10) {
                fputs(dcgettext(NULL,"Valid arguments for -D:\n",5),v3);
                v5 = "exec";
                v17 = "Show diagnostic information relating to -exec, -execdir, -ok and -okdir";
                for (v19 = (unsigned long *)0x36198; __fprintf_chk(stdout,1,"%-10s %s\n",v5,v17), v19 != (unsigned long *)0x36258; v19 = &v19[3]) {
                  v17 = (char *)v19[2];
                  v5 = (char *)*v19;
                }
                exit(0); // no-return
              }
            }
          }
          else {
            sub_14af0(0);
          }
        }
        else {
          sub_14af0(1);
        }
      }
      else {
        sub_14af0(2);
      }
      v18 += 1;
    } while ((int)v18 < a0);
  }
  return (unsigned long)v18;
}

// Function: sub_e630 @ 0xe630
void sub_e630(struct_49 *a0)
{
  char v1; // al
  int v2; // eax
  long v3; // rdx
  timeval v4; // stack - 0x28
  
  a0->field_0x11 = getenv("POSIXLY_CORRECT") != NULL;
  v1 = sub_d920();
  a0->field_0x5c = 0;
  a0->field_0x58 = v1;
  v2 = isatty(0);
  a0->field_0xf = 0;
  a0->field_0x10 = v2 != 0;
  if (a0->field_0x11)
    a0->field_0x10 = 0;
  *(unsigned long *)&a0->field_0x2[2] = 0xffffffffffffffff;
  a0->field_0x0 = 1;
  if (gettimeofday(&v4,NULL)) { // branch-flip
    v4._0_8_ = time(0);
    if (v4._0_8_ == -1)
      __assert_fail("t != (time_t)-1","util.c",0x3d2,"now"); // no-return
    v3 = 0;
  }
  else {
    v3 = v4._8_8_ * 1000;
  }
  a0->field_0x18 = v4._0_8_;
  a0->field_0x28 = v4._0_8_ + -0x15180;
  *(unsigned short *)a0->field_0xd = 0;
  a0->field_0x20 = v3;
  a0->field_0x30 = v3;
  a0->field_0x38 = 0;
  a0->field_0x3c = (-(unsigned int)(a0->field_0x11 == '\0') & 0x200) + 0x200;
  a0->field_0x40 = 0;
  a0->field_0x68 = 2;
  if (getenv("FIND_BLOCK_SIZE")) {
    error(1,0,dcgettext(NULL,"The environment variable FIND_BLOCK_SIZE is not supported, the only thing that affects the block size is the POSIXLY_CORRECT environment variable",5));
    return;
  }
  a0->field_0xc = 0;
  sub_14af0(0);
  a0->field_0x6c = 8;
  a0->field_0x70 = 0;
  a0->field_0x78 = 0;
}

// Function: sub_e7b0 @ 0xe7b0
unsigned long sub_e7b0(char *a0,void *a1,struct_2 *a2)
{
  int v1; // eax
  unsigned long v2; // rax
  
  a2->field_0x120 = a2->field_0x120 + 1;
  if ((((!(a2->field_0x18 & 0xffffff0000)) || ((((!*(char *)((long)&a2->field_0x18 + 2) || (dat_385e4)) && ((!*(char *)((long)&a2->field_0x18 + 3) || (dat_385e5)))) && ((!*(char *)((long)&a2->field_0x18 + 4) || (((*(long *)((long)a1 + 8) && (dat_385e5)) && ((*(unsigned int *)((long)a1 + 0x18) & 0xf000) != 0x4000)))))))) || (v1 = sub_d820(a0,dat_385f0,a1), !v1)) && (v2 = (*a2->field_0x0)(a0,a1,a2), (char)v2)) {
    a2->field_0x128 = a2->field_0x128 + 1;
    return v2;
  }
  return 0;
}

// Function: sub_e880 @ 0xe880
unsigned int sub_e880(void *a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == sub_95d0) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == sub_8fc0);
}

// Function: sub_e8a0 @ 0xe8a0
void sub_e8a0(unsigned long a0,char *a1)
{
  sub_239f0(a0,dat_3868c,a1); // tail-call
}

// Function: sub_e8c0 @ 0xe8c0
void sub_e8c0(int a0,char *a1)
{
  if (!dat_38618) {
    error(0,a0,"%s",(char *)sub_239f0(0,dat_3868c,a1));
    dat_38618 = 1;
    dat_38604 = 1;
    return;
  }
  dat_38618 = 1;
}

// Function: sub_e920 @ 0xe920
void sub_e920(int a0,char *a1)
{
  if (!dat_38618) {
    error(1,a0,"%s",(char *)sub_239f0(0,dat_3868c,a1));
    return;
  }
  dat_38618 = 1;
  abort(); // no-return
}

// Function: sub_e970 @ 0xe970
void sub_e970(int a0,char *a1)
{
  error(0,a0,"%s",(char *)sub_239f0(0,dat_3868c,a1));
  dat_38604 = 1;
}

// Function: sub_e9b0 @ 0xe9b0
void sub_e9b0(int a0,char *a1)
{
  dat_38618 = 0;
  error(1,a0,"%s",(char *)sub_239f0(0,dat_3868c,a1));
}

// Function: sub_ea00 @ 0xea00
unsigned long sub_ea00(struct_12 *a0,struct_12 *a1)
{
  if (a0->field_0x8 != a1->field_0x8)
    return 0;
  return CONCAT71((undefined7)((unsigned long)a1->field_0x0 >> 8),a0->field_0x0 == a1->field_0x0);
}

// Function: sub_ea30 @ 0xea30
unsigned long sub_ea30(struct_50 *a0,unsigned long a1)
{
  return (a0->field_0x0 ^ a0->field_0x8) % a1;
}

// Function: sub_ea50 @ 0xea50
void sub_ea50(void *a0)
{
  FILE *v1;
  int *v2; // rax
  
  v1 = *(FILE **)((long)a0 + 0x18);
  if (v1) {
    if (sub_17d40(v1)) {
      v2 = __errno_location();
      sub_e9b0(*v2,*(char **)((long)a0 + 0x10)); // no-return
    }
  }
  free(*(void **)((long)a0 + 0x10));
  free(a0); // tail-call
}

// Function: sub_ea90 @ 0xea90
unsigned long * sub_ea90(char *a0)
{
  unsigned long *v1; // rax
  char *v2; // rax
  long v3; // rax
  unsigned long *v4;
  
  v1 = malloc(0x10);
  v4 = v1;
  if (v1) {
    v2 = strdup(a0);
    *v1 = v2;
    if (v2) { // branch-flip
      v3 = sub_1b2f0(0xb,NULL,sub_ea30,sub_ea00,sub_ea50);
      v1[1] = v3;
      if (!v3) {
        free((void *)*v1);
        v4 = NULL;
        free(v1);
      }
    }
    else {
      v4 = NULL;
      free(v1);
    }
  }
  return v4;
}

// Function: sub_eb20 @ 0xeb20
void sub_eb20(struct_35 *a0)
{
  free(a0->field_0x0);
  sub_1b4b0(a0->field_0x8); // tail-call
}

// Function: sub_eb40 @ 0xeb40
unsigned long sub_eb40(struct_46 *a0,char *a1)
{
  struct_24 *v1;
  int v2;
  unsigned long *v3; // rax
  char *v4; // rax
  FILE *v5; // rax
  long v6; // rax
  int *v7; // rax
  stat v8; // stack - 0xc8
  unsigned long v9;
  
  v3 = malloc(0x20);
  if (v3) { // branch-flip
    v4 = strdup(a1);
    v3[2] = v4;
    if (v4) { // branch-flip
      v5 = (FILE *)sub_182a0(a1,a0->field_0x0);
      v3[3] = v5;
      if (v5) { // branch-flip
        v2 = fileno(v5);
        if (v2 < 0)
          __assert_fail("fd >= 0","sharefile.c",0xa6,"sharefile_fopen"); // no-return
        sub_17990(v2,1);
        if (0 <= fstat(v2,&v8)) { // branch-flip
          v1 = a0->field_0x8;
          *v3 = v8._0_8_;
          v3[1] = v8._8_8_;
          v6 = sub_1b060(v1,v3);
          if (v6) { // branch-flip
            sub_ea50(v3);
            v9 = *(unsigned long *)(v6 + 0x18);
          }
          else {
            v1 = a0->field_0x8;
            if (sub_1b950(v1,v3)) // branch-flip
              v9 = v3[3];
            else {
              v7 = __errno_location();
              v2 = *v7;
              sub_ea50(v3);
              *v7 = v2;
              v9 = 0;
            }
          }
        }
        else {
          v9 = 0;
          sub_ea50(v3);
        }
      }
      else {
        sub_ea50(v3);
        v9 = 0;
      }
    }
    else {
      free(v3);
      v9 = 0;
    }
  }
  else {
    v9 = 0;
  }
  return v9;
}

// Function: sub_ece0 @ 0xece0
void sub_ece0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,void *a8,char *a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13) // return-dupe
{
  FILE *v1;
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
  char v2; // al
  unsigned long v20; // stack - 0x18
  int *v3; // rax
  unsigned int v4; // stack - 0xd8
  char v5 [16];
  unsigned int v6; // stack - 0xd4
  char *v7; // stack - 0xd0
  char *v8; // stack - 0xc8
  unsigned long v9; // stack - 0xa8
  
  if (v2) {
    v13 = a0;
    v14 = a1;
    v15 = a2;
    v16 = a3;
    v17 = a4;
    v18 = a5;
    v19 = a6;
    v20 = a7;
  }
  v1 = *(FILE **)((long)a8 + 8);
  v7 = &Stack0000000000000008;
  v4 = 0x10;
  v8 = v5;
  v6 = 0x30;
  v9 = a10;
  v10 = a11;
  v11 = a12;
  v12 = a13;
  if (0 <= __vfprintf_chk(v1,1,a9,&v4))
    return;
  v3 = __errno_location();
  sub_e970(*v3,*(char **)((long)a8 + 0x10));
}

// Function: sub_edc0 @ 0xedc0
unsigned char * sub_edc0(long a0,long a1,int a2)
{
  unsigned int *v1;
  unsigned long v10; // rax
  tm *v11; // rax
  char *v12; // rax
  unsigned long v13; // rax
  char *v14;
  unsigned int v15; // edx
  unsigned int v16; // stack - 0x68
  long v17; // stack - 0xc8
  char v18 [8]; // stack - 0x74
  tm v19; // stack - 0xb8
  unsigned long v2;
  unsigned int v20; // stack - 0x6c
  unsigned char v21;
  long v22;
  long v23; // stack - 0xf0
  long v24; // stack - 0xe0
  long v25; // stack - 0xc0
  unsigned int v26; // stack - 0x64
  unsigned char v3;
  long v4;
  bool v5;
  bool v6;
  int v7;
  unsigned int v8;
  unsigned char *v9;
  
  v17 = a0;
  v25 = a1;
  if (a2 != 0x2b) { // branch-flip
    v18[1] = (char)a2;
    v18[0] = 0x25;
    v18[2] = 0;
    if ((0x19 <= (unsigned int)(a2 - 0x40U)) || (!(0x1180001UL >> ((unsigned long)(unsigned int)(a2 - 0x40U) & 0x3f) & 1))) {
      v16 &= 0xffffff00;
      v6 = 0;
      v5 = 0;
      v8 = 0;
    }
    else {
      v6 = 1;
      v5 = 1;
      v8 = __snprintf_chk((char *)&v16,0x20,1,0x20,".%09ld0",a1);
    }
    if (a2 == 0x40) goto label_ee40;
  }
  else {
    v18._0_8_ = 0x64252d6d252d5925;
    v20 = 0x54252b;
    v8 = __snprintf_chk((char *)&v16,0x20,1,0x20,".%09ld0",a1);
    v6 = 1;
  }
  v5 = v6;
  v11 = localtime(&v17);
  if (v11) {
    v10 = (unsigned long)(int)v8;
    v12 = (char *)sub_25a50(strlen(v18) + 2);
    *v12 = '_';
    memcpy(&v12[1],v18,strlen(v18) + 1);
    v19._8_8_ = *(unsigned long *)&v11->field_0x8;
    v19._16_8_ = *(unsigned long *)&v11->field_0x10;
    v19._24_8_ = *(unsigned long *)&v11->field_0x18;
    v19._0_4_ = SUB84(*(unsigned long *)v11,0);
    v19._32_8_ = *(unsigned long *)&v11->field_0x20;
    v19._40_8_ = *(unsigned long *)&v11->field_0x28;
    v19._48_8_ = *(unsigned long *)&v11->field_0x30;
    v7 = v19._0_4_ + 0xb;
    if (0xb <= (int)v19._0_4_)
      v7 = v19._0_4_ + -0xb;
    v19._4_4_ = SUB84((unsigned long)*(unsigned long *)v11 >> 0x20,0);
    v19._0_4_ = v7;
    v14 = (char *)dat_38b08;
    if (!dat_38b08) {
      dat_38b00 = 1;
      v14 = (char *)sub_25a50(1);
      dat_38b08 = (unsigned char *)v14;
    }
    while ((v13 = strftime(v14,dat_38b00,v12,v11), !v13 || (dat_38b00 <= v13))) {
      v14 = (char *)sub_25ca0(v14,(unsigned long *)0x38b00,1);
      dat_38b08 = (unsigned char *)v14;
    }
    v2 = v10 + 1 + v13;
    dat_38b08 = (unsigned char *)sub_25aa0(v14,v2);
    dat_38b00 = v2;
    v14 = (char *)sub_25a50(v2);
    strftime(v14,dat_38b00,v12,&v19);
    v9 = dat_38b08;
    v21 = *dat_38b08;
    if (v21) {
      v23 = 0;
      v22 = 0;
      v24 = 0;
      v6 = 0;
      do {
        v3 = v14[v22];
        if (!v3) {
          free(v12); // return-dupe
          free(v14);
          return &v9[1];
        }
        if (v3 != v21) {
          v4 = *(long *)__ctype_b_loc();
          if ((!(*(unsigned char *)(v4 + 1 + (unsigned long)v21 * 2) & 8)) || (!(*(unsigned char *)(v4 + 1 + (unsigned long)v3 * 2) & 8))) {
            free(v12);
            free(v14);
            return &v9[1];
          }
          if (v6) {
            if (v23 != v22 - v24) {
              free(v12);
              free(v14);
              return &v9[1];
            }
            v23 += 1;
          }
          else {
            v23 = 1;
            v6 = 1;
            v24 = v22;
          }
        }
        v22 += 1;
        v21 = v9[v22];
      } while (v21);
      if ((!v14[v22]) && (v23 == 2)) {
        v24 += 2;
        v1 = (unsigned int *)&v9[v24];
        if (!(*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)(unsigned char)*v1 * 2) & 8)) {
          if (v2 != v13 + 1 + v10)
            __assert_fail("end_of_seconds + ns_size + suffix_len == final_len","print.c",0x225,"do_time_format"); // no-return
          memmove(&v9[v10 + v24],v1,(v13 + 1) - v24);
          if (8 <= v8) { // branch-flip
            *(unsigned long *)v1 = CONCAT44(v26,v16);
            *(unsigned long *)((long)v1 + ((unsigned long)v8 - 8)) = *(unsigned long *)&v18[(unsigned long)v8 + 4];
            v22 = (long)v1 - ((unsigned long)&v1[2] & 0xfffffffffffffff8);
            v8 = v8 + (int)v22 & 0xfffffff8;
            if (8 <= v8) {
              v15 = 0;
              do {
                v10 = (unsigned long)v15;
                v15 += 8;
                *(unsigned long *)(((unsigned long)&v1[2] & 0xfffffffffffffff8) + v10) = *(unsigned long *)((long)&v16 + (v10 - v22));
              } while (v15 < v8);
            }
          }
          else if (v10 & 4) { // branch-flip
            *v1 = v16;
            *(unsigned int *)((long)v1 + ((unsigned long)v8 - 4)) = *(unsigned int *)((long)&v20 + (unsigned long)v8);
          }
          else if (v8) {
            *(unsigned char *)v1 = (unsigned char)v16;
            if (v10 & 2)
              *(unsigned short *)((long)v1 + ((unsigned long)v8 - 2)) = *(unsigned short *)((long)&v20 + (unsigned long)v8 + 2);
          }
        }
      }
    }
    free(v12);
    free(v14);
    return &v9[1];
  }
label_ee40:
  v22 = -v17;
  if (-v17 <= -1)
    v22 = v17;
  v9 = (unsigned char *)sub_1bb50(v22,(char *)0x38b21,0,1,1);
  if ((unsigned char *)0x38b21 <= v9) {
    if ((unsigned char *)0x38e37 <= v9)
      __assert_fail("p < (buf + (sizeof buf))","print.c",0x2ba,"format_date"); // no-return
    if (v17 <= -1) {
      v9[-1] = 0x2d;
      v9 = &v9[-1];
    }
    if (v5) {
      v10 = strlen((char *)v9);
      if ((unsigned char *)0x317 <= &v9[v10 - 0x38b20])
        __assert_fail("sizeof buf > used","print.c",0x2c7,"format_date"); // no-return
      v10 = 0x316 - (long)&v9[v10 - 0x38b20];
      if (v10 <= strlen((char *)&v16)) {
        error(0,0,"charsprinted=%ld but remaining=%lu: ns_buf=%s",(long)(int)v8,v10,(char *)&v16);
        if (v10 <= strlen((char *)&v16))
          __assert_fail("strlen (ns_buf) < remaining","print.c",0x2d0,"format_date"); // no-return
      }
      strcat((char *)v9,(char *)&v16);
    }
    return v9;
  }
  __assert_fail("p > buf","print.c",0x2b9,"format_date"); // no-return
}

// Function: sub_f400 @ 0xf400
unsigned long sub_f400(unsigned long a0,long a1) // early-return
{
  int v1;
  tm *v10; // rax
  unsigned long v11; // stack - 0x18
  long v12; // stack - 0x10
  int v2;
  int v3;
  int v4;
  int v5;
  int v6;
  int v7;
  char *v8;
  char *v9;
  
  v11 = a0;
  v12 = a1;
  v10 = localtime(&v11);
  if (!v10)
    return sub_edc0(v11,v12,0x40);
  v1 = *(int *)&v10->field_0x18;
  if (v1 < 0)
    __assert_fail("ptm->tm_wday >= 0","print.c",0x2ed,"ctime_format"); // no-return
  if (6 < v1)
    __assert_fail("ptm->tm_wday < 7","print.c",0x2ee,"ctime_format"); // no-return
  v2 = *(int *)&v10->field_0x10;
  if (v2 < 0)
    __assert_fail("ptm->tm_mon >= 0","print.c",0x2ef,"ctime_format"); // no-return
  if (0xb < v2)
    __assert_fail("ptm->tm_mon < 12","print.c",0x2f0,"ctime_format"); // no-return
  v3 = *(int *)&v10->field_0x8;
  if (v3 < 0)
    __assert_fail("ptm->tm_hour >= 0","print.c",0x2f1,"ctime_format"); // no-return
  if (0x17 < v3)
    __assert_fail("ptm->tm_hour < 24","print.c",0x2f2,"ctime_format"); // no-return
  v4 = *(int *)&v10->field_0x4;
  if (v4 <= 0x3b) {
    v5 = *(int *)v10;
    if (0x3d < v5)
      __assert_fail("ptm->tm_sec <= 61","print.c",0x2f4,"ctime_format"); // no-return
    v6 = *(int *)&v10->field_0x14;
    v7 = *(int *)&v10->field_0xc;
    v8 = *(char **)((long)v2 * 8 + 0x36260);
    v9 = *(char **)((long)v1 * 8 + 0x362c0);
    if (__snprintf_chk((char *)0x38700,0x400,1,0x400,"%3s %3s %2d %02d:%02d:%02d.%09ld0 %04d",v9,v8,v7,v3,v4,v5,v12,v6 + 0x76c) <= 0x3ff)
      return 0x38700;
    __assert_fail("nout < TIME_BUF_LEN","print.c",0x302,"ctime_format"); // no-return
  }
  __assert_fail("ptm->tm_min < 60","print.c",0x2f3,"ctime_format"); // no-return
}

// Function: sub_f620 @ 0xf620
void sub_f620(void *a0,struct_37 *a1,char *a2,struct_36 *a3) // return-dupe
{
  FILE *v1;
  char v10 [10];
  char *v11; // stack - 0x360
  stat v12; // stack - 0x358
  void *v13;
  unsigned long v14; // r8
  unsigned long v15;
  char v16; // stack - 0x2be
  unsigned int v2;
  int v3;
  int *v4; // rax
  passwd *v5; // rax
  char *v6;
  unsigned long v7; // rax
  unsigned long v8; // rcx
  char *v9;
  
  v3 = a1->field_0x0;
  if (v3 == 1) {
    v3 = a1->field_0x10;
    v1 = *(FILE **)((long)a0 + 8);
    v13 = a1->field_0x8;
    if (fwrite(v13,1,(long)v3,v1) < (unsigned long)(long)v3) {
      v4 = __errno_location();
      sub_e970(*v4,*(char **)((long)a0 + 0x10));
    }
    v1 = *(FILE **)((long)a0 + 8);
    if (!sub_180c0(v1))
      return;
    v4 = __errno_location(); // return-dupe
    sub_e970(*v4,*(char **)((long)a0 + 0x10)); // tail-call
    return;
  }
  if (v3 != 2) {
    if (v3)
      return;
label_f688:
    v3 = a1->field_0x10;
    v1 = *(FILE **)((long)a0 + 8);
    v13 = a1->field_0x8;
    if ((unsigned long)(long)v3 <= fwrite(v13,1,(long)v3,v1))
      return;
    v4 = __errno_location();
    sub_e970(*v4,*(char **)((long)a0 + 0x10));
    return;
  }
  switch(a1->field_0x4) {
    case 0:
      error(1,0,dcgettext(NULL,"error: %s at end of format string",5),"%");
      goto label_f65e;
    default:
      return;
    case 0x25:
      goto label_f688;
    case 0x44:
      v15 = a3->field_0x0;
      v14 = 1;
      v8 = 1;
      break;
    case 0x46:
      v9 = (char *)sub_108e0(a3,a2);
      goto label_f7c4;
    case 0x47:
      goto label_f79c;
    case 0x48:
      v9 = (char *)sub_25a50((long)(dat_385fc + 1));
      v15 = (unsigned long)dat_385fc;
      memcpy(v9,a2,v15);
      v9[v15] = '\0';
      sub_ece0(a0,a1->field_0x8,v9);
      free(v9); // return-dupe, tail-call
      return;
    case 0x4d:
      sub_18290(a3,v10);
      v16 = 0;
      sub_ece0(a0,a1->field_0x8,v10);
      return;
    case 0x50:
      v9 = "";
      if (1 <= (int)dat_385e0)
        v9 = &(&a2[dat_385fc])[a2[dat_385fc] == '/'];
label_f7c4:
      v3 = sub_16e80(*(FILE **)((long)a0 + 8),*(unsigned int **)((long)a0 + 0x20),(unsigned char)*(char *)((long)a0 + 0x18),a1->field_0x8,v9);
label_fa78:
      if (0 <= v3)
        return;
      v4 = __errno_location();
      sub_e970(*v4,*(char **)((long)a0 + 0x10));
      return;
    case 0x53:
      sub_ece0(a0,a1->field_0x8); // tail-call
      return;
    case 0x55:
      goto label_f752;
    case 0x59:
      v2 = a3->field_0x18 & 0xf000;
      if (v2 == 0xa000) {
        if (fstatat(dat_385f8,dat_385f0,&v12,0)) {
          v4 = __errno_location();
          v13 = a1->field_0x8;
          v3 = *v4;
          if ((v3 != 2) && (v3 != 0x14)) {
            if (v3 != 0x28) {
              sub_ece0(a0,v13,"?");
              v7 = sub_e8a0(0,a2);
              error(0,*v4,"%s",v7);
              return;
            }
            sub_ece0(a0,v13,"L");
            return;
          }
          sub_ece0(a0,v13,"N");
          return;
        }
        v9 = "f";
        v2 = v12._24_4_ & 0xf000;
        if (((v2 != 0x8000) && (v9 = "d", v2 != 0x4000)) && ((v9 = "l", v2 != 0xa000 && ((((v9 = "s", v2 != 0xc000 && (v9 = "b", v2 != 0x6000)) && (v9 = "c", v2 != 0x2000)) && (v9 = "U", v2 == 0x1000))))))
          v9 = "p";
        sub_ece0(a0,a1->field_0x8,v9); // return-dupe
        return;
      }
      v9 = "f";
      if ((v2 != 0x8000) && (v9 = "d", v2 != 0x4000)) goto label_f96d;
      goto label_f9c1;
    case 0x5a:
      v3 = (*dat_38680)(dat_385f8,dat_385f0,&v11);
      if (0 <= v3) {
        sub_ece0(a0,a1->field_0x8,v11);
        freecon(v11);
        return;
      }
label_f65e:
      sub_ece0(a0);
      v7 = sub_e8a0(0,a2);
      v9 = dcgettext(NULL,"getfilecon failed: %s",5);
      error(0,*__errno_location(),v9,v7);
      dat_38604 = 1;
      return;
    case 0x61:
      v9 = (char *)sub_f400(a3->field_0x48,a3->field_0x50);
      goto label_f9c1;
    case 0x62:
      v15 = a3->field_0x40;
      v14 = 0x200;
      v8 = 0x200;
      break;
    case 99:
      v9 = (char *)sub_f400(a3->field_0x68,a3->field_0x70);
      goto label_f9c1;
    case 100:
      v13 = a1->field_0x8;
      v2 = dat_385e0;
      sub_ece0(a0,v13,v2); // return-dupe, tail-call
      return;
    case 0x66:
      v9 = (char *)sub_17b90(a2);
label_fbce:
      v3 = sub_16e80(*(FILE **)((long)a0 + 8),*(unsigned int **)((long)a0 + 0x20),(unsigned char)*(char *)((long)a0 + 0x18),a1->field_0x8,v9);
      goto label_fb12;
    case 0x67:
      v5 = (passwd *)getgrgid(a3->field_0x20);
      if (v5) {
label_f9ef:
        *(char *)((long)a1->field_0x8 + (long)a1->field_0x10) = 0x73;
        v13 = a1->field_0x8;
        v9 = *(char **)v5;
        sub_ece0(a0,v13,v9); // return-dupe, tail-call
        return;
      }
      goto label_f79c;
    case 0x68:
      v9 = (char *)sub_25f30(a2);
      for (v6 = &v9[strlen(v9) - 1]; v9 <= v6; v6 = &v6[-1]) {
        if (*v6 != '/') {
          if ((v9 < v6) && (v6[1] == '/'))
            v6[1] = '\0';
          break;
        }
      }
      v6 = strrchr(v9,0x2f);
      if (v6) {
        *v6 = '\0';
        goto label_fbce;
      }
      v3 = sub_16e80(*(FILE **)((long)a0 + 8),*(unsigned int **)((long)a0 + 0x20),(unsigned char)*(char *)((long)a0 + 0x18),a1->field_0x8,".");
      goto label_fb12;
    case 0x69:
      v15 = a3->field_0x8;
      v14 = 1;
      v8 = 1;
      break;
    case 0x6b:
      v15 = a3->field_0x40;
      v14 = 0x400;
      v8 = 0x200;
      break;
    case 0x6c:
      if ((a3->field_0x18 & 0xf000) == 0xa000) {
        v9 = (char *)sub_17650(dat_385f8,dat_385f0);
        if (v9) goto label_fbce;
        sub_e8c0(*__errno_location(),a2);
        dat_38604 = 1;
      }
      v9 = NULL;
      v3 = sub_16e80(*(FILE **)((long)a0 + 8),*(unsigned int **)((long)a0 + 0x20),(unsigned char)*(char *)((long)a0 + 0x18),a1->field_0x8,"");
label_fb12:
      if (v3 <= -1) {
        v4 = __errno_location();
        sub_e970(*v4,*(char **)((long)a0 + 0x10));
      }
      free(v9);
      return;
    case 0x6d:
      v13 = a1->field_0x8;
      v2 = a3->field_0x18 & 0xfff;
      sub_ece0(a0,v13,v2);
      return;
    case 0x6e:
      v15 = a3->field_0x10;
      v14 = 1;
      v8 = 1;
      break;
    case 0x70:
      v3 = sub_16e80(*(FILE **)((long)a0 + 8),*(unsigned int **)((long)a0 + 0x20),(unsigned char)*(char *)((long)a0 + 0x18),a1->field_0x8,a2);
      goto label_fa78;
    case 0x73:
      v15 = a3->field_0x30;
      v14 = 1;
      v8 = 1;
      break;
    case 0x74:
      v9 = (char *)sub_f400(a3->field_0x58,a3->field_0x60);
      goto label_f9c1;
    case 0x75:
      v5 = getpwuid(a3->field_0x1c);
      if (v5) goto label_f9ef;
      v15 = (unsigned long)a3->field_0x1c; // return-dupe
      v14 = 1;
      v8 = 1;
      v9 = (char *)sub_1bb50(v15,v10,0,v8,v14);
      sub_ece0(a0,a1->field_0x8,v9);
      return;
    case 0x79:
      v9 = "f";
      v2 = a3->field_0x18 & 0xf000;
      if (((v2 != 0x8000) && (v9 = "d", v2 != 0x4000)) && (v9 = "l", v2 != 0xa000)) {
label_f96d:
        v9 = "s";
        if ((v2 != 0xc000) && (((v9 = "b", v2 != 0x6000 && (v9 = "c", v2 != 0x2000)) && (v9 = "U", v2 == 0x1000))))
          v9 = "p";
      }
label_f9c1:
      v13 = a1->field_0x8;
      sub_ece0(a0,v13,v9);
      return;
    
  }
label_f770:
  v9 = (char *)sub_1bb50(v15,v10,0,v8,v14);
  sub_ece0(a0,a1->field_0x8,v9);
  return;
label_f752:
  v15 = (unsigned long)a3->field_0x1c;
  v14 = 1;
  v8 = 1;
  goto label_f770;
label_f79c:
  v15 = (unsigned long)a3->field_0x20;
  v14 = 1;
  v8 = 1;
  goto label_f770;
}

// Function: sub_ff90 @ 0xff90
unsigned long sub_ff90(char *a0,struct_36 *a1,void *a2) // return-dupe
{
  char v1;
  unsigned long v2; // rax
  int *v3;
  unsigned long v4;
  unsigned long v5;
  
  v3 = *(int **)((long)a2 + 0x38);
  if (!v3)
    return 1;
  a2 = (void *)((long)a2 + 0x38);
  do {
    while ((*v3 != 2 || (!*(char *)((long)v3 + 5)))) {
      sub_f620(a2,v3,a0,a1);
      v3 = *(int **)&v3[6];
      if (!v3)
        return 1;
    }
    v1 = (char)v3[1];
    if (v1 != 'C') { // branch-flip
      if ('C' < v1) {
        if (v1 != 'T')
          __assert_fail("0","print.c",0x518,"pred_fprintf"); // no-return, return-dupe
        v4 = a1->field_0x60;
        v5 = a1->field_0x58;
        goto label_ffe6;
      }
      if (v1 == 'A') {
        v4 = a1->field_0x50;
        v5 = a1->field_0x48;
        goto label_ffe6;
      }
      if (v1 != 'B') {
        __assert_fail("0","print.c",0x518,"pred_fprintf");
      }
      v5 = 0xffffffffffffffff;
      v4 = 0xffffffffffffffff;
      if (*(char *)((long)v3 + 5) == '@') goto label_ffe6;
      sub_ece0(a2,*(unsigned long *)&v3[2],"");
    }
    else {
      v4 = a1->field_0x70;
      v5 = a1->field_0x68;
label_ffe6:
      v2 = sub_edc0(v5,v4);
      sub_ece0(a2,*(unsigned long *)&v3[2],v2);
    }
    v3 = *(int **)&v3[6];
    if (!v3)
      return 1;
  } while( true );
}

// Function: sub_100b0 @ 0x100b0
long sub_100b0(long *a0,char *a1,unsigned int a2,unsigned int a3,char a4,char a5,void *a6)
{
  long v1;
  unsigned int v2; // eax
  unsigned int *v3; // rax
  char *v4; // rax
  unsigned long v5; // r14
  char v6; // stack - 0x39
  
  if (a4 == '{')
    __assert_fail("format_char != \'{\'","print.c",0x4e,"make_segment"); // no-return
  if (a4 == '[')
    __assert_fail("format_char != \'[\'","print.c",0x4f,"make_segment"); // no-return
  if (a4 == '(')
    __assert_fail("format_char != \'(\'","print.c",0x50,"make_segment"); // no-return
  v5 = (unsigned long)(int)a2;
  v3 = (unsigned int *)sub_25a50(0x20);
  *a0 = (long)v3;
  *v3 = a3;
  *(char *)&v3[1] = a4;
  *(char *)((long)v3 + 5) = a5;
  v3[6] = 0;
  v3[7] = 0;
  v3[4] = a2;
  v4 = (char *)sub_25a50(v5 + 2);
  *(char **)&v3[2] = v4;
  v4 = &strncpy(v4,a1,v5)[v5];
  if (a3 < 2) {
    if (a4)
      __assert_fail("0 == format_char","print.c",0x61,"make_segment"); // no-return
    if (!a5) {
      v1 = *a0;
      *v4 = '\0';
      return v1 + 0x18;
    }
    __assert_fail("0 == aux_format_char","print.c",0x62,"make_segment"); // no-return
  }
  if (a3 != 2)
    __assert_fail("kind == KIND_FORMAT","print.c",0x69,"make_segment"); // no-return
  switch(a4) {
    case 0x25:
    case 100:
      v6 = a4;
      goto label_101f9;
    default:
      v2 = 0;
      break;
    case 0x41:
    case 0x42:
    case 0x43:
    case 0x44:
    case 0x46:
    case 0x47:
    case 0x4d:
    case 0x54:
    case 0x55:
    case 0x61:
    case 0x62:
    case 99:
    case 0x67:
    case 0x6b:
    case 0x6e:
    case 0x73:
    case 0x74:
    case 0x75:
      *v4 = 's';
      v4 = &v4[1];
      *(char *)((long)a6 + 0x1a) = 1;
      v2 = 3;
      break;
    case 0x48:
    case 0x50:
    case 0x66:
    case 0x68:
    case 0x70:
      v6 = 's';
label_101f9:
      *v4 = v6;
      v4[1] = '\0';
      return *a0 + 0x18; // return-dupe
    case 0x53:
      *v4 = 'g';
      v4 = &v4[1];
      *(char *)((long)a6 + 0x1a) = 1;
      v2 = 3;
      break;
    case 0x59:
      *v4 = 's';
      v4 = &v4[1];
      *(char *)((long)a6 + 0x1a) = 1;
      v2 = 2;
      break;
    case 0x5a:
      *v4 = 's';
      v2 = 5;
      v4 = &v4[1];
      break;
    case 0x69:
      *v4 = 's';
      v4 = &v4[1];
      *(char *)((long)a6 + 0x1c) = 1;
      v2 = 1;
      break;
    case 0x6c:
      *v4 = 's';
      v4 = &v4[1];
      *(char *)((long)a6 + 0x1a) = 1;
      v2 = 4;
      break;
    case 0x6d:
      *v4 = 'o';
      v4 = &v4[1];
      *(char *)((long)a6 + 0x1a) = 1;
      v2 = 3;
      break;
    case 0x79:
      *v4 = 's';
      v4 = &v4[1];
      *(char *)((long)a6 + 0x1b) = 1;
      v2 = 2;
    
  }
  *v4 = '\0';
  if (*(unsigned int *)((long)a6 + 0x20) < v2)
    *(unsigned int *)((long)a6 + 0x20) = v2;
  return *a0 + 0x18;
}

// Function: sub_10380 @ 0x10380
unsigned long sub_10380(unsigned long *a0,struct_55 *a1,unsigned char *a2) // return-dupe x2
{
  unsigned char v1;
  int v10;
  unsigned char *v11;
  int v12; // r13d
  unsigned char *v13;
  unsigned char *v14;
  long v15; // r15
  unsigned long v16; // rbp
  long *v17; // stack - 0x48
  unsigned long v2;
  void *v3; // rax
  unsigned char *v4;
  unsigned long v5; // rax
  unsigned long v6; // rax
  unsigned char v7;
  char *v8; // rsi
  long v9;
  
  v3 = (void *)sub_d490(a1,sub_ff90,a2);
  v2 = a0[1];
  *(unsigned long *)((long)v3 + 0x38) = *a0;
  *(unsigned long *)((long)v3 + 0x40) = v2;
  v2 = a0[3];
  *(unsigned long *)((long)v3 + 0x48) = a0[2];
  *(unsigned long *)((long)v3 + 0x50) = v2;
  v2 = a0[4];
  v17 = (long *)((long)v3 + 0x38);
  *(unsigned int *)((long)v3 + 0x18) = 0x101;
  *(unsigned long *)((long)v3 + 0x58) = v2;
  *(unsigned int *)((long)v3 + 0x20) = 0;
  *(unsigned long *)((long)v3 + 0x38) = 0;
  v7 = *a2;
  if (!v7)
    return 1;
  v13 = a2;
  do {
    while( true ) {
      v1 = a2[1];
      v16 = (unsigned long)v1;
      v12 = (int)v13;
      if (v7 != 0x5c) break;
      v10 = (int)a2;
      if (v1 == 99) {
        sub_100b0(v17,v13,v10 - v12,1,0,0,v3);
        if (!*(char *)((long)v3 + 0x1a))
          return 1;
        if (3 <= *(unsigned int *)((long)v3 + 0x20))
          return 1;
        *(unsigned int *)((long)v3 + 0x20) = 3;
        return 1;
      }
      v4 = &a2[1];
      if (v1) { // branch-flip
        if (8 <= (unsigned char)(v1 - 0x30)) { // branch-flip
          if ((0x1b <= (unsigned char)(v1 + 0xa4)) || (!"\\"[(unsigned char)(v1 + 0xa4)])) {
            v11 = &a2[2];
            error(0,0,dcgettext(NULL,"warning: unrecognized escape `\\%c\'",5),v1);
            a2 = v4;
            goto label_1040f;
          }
          *a2 = "\\"[(unsigned char)(v1 + 0xa4)];
          v11 = &a2[2];
          a2 = v4;
        }
        else {
          v7 = 0;
          v9 = 0;
          while( true ) {
            if (8 <= (unsigned char)((char)v16 - 0x30U)) break;
            v11 = &v4[1];
            v7 = (char)v16 + '\xd0' + v7 * '\b';
            v14 = v4;
            if (v9 + 1 == 3) goto label_10740;
            v16 = (unsigned long)*v11;
            v4 = v11;
            v9 += 1;
          }
          v14 = &a2[v9];
          v11 = v4;
label_10740:
          *a2 = v7;
          a2 = v14;
        }
      }
      else {
        error(0,0,dcgettext(NULL,"warning: escape `\\\' followed by nothing at all",5));
        v11 = v4;
      }
      v17 = (long *)sub_100b0(v17,v13,(v10 - v12) + 1,0,0,0,v3);
      v7 = a2[1];
      a2 = v11;
      v13 = v11;
      if (!v7) goto label_104b7;
    }
    if (v7 != 0x25) // branch-flip
      v11 = &a2[1];
    else {
      if (!v1) {
        error(1,0,dcgettext(NULL,"error: %s at end of format string",5),(char *)a2);
        return v5;
      }
      if (v1 != 0x25) { // branch-flip
        v9 = 0;
        while( true ) {
          v15 = v9 + 1;
          if ((!(char)v16) || (!strchr("-+ #",(int)(char)v16))) break;
          v16 = (unsigned long)a2[v9 + 2];
          v9 = v15;
        }
        v9 = *(long *)__ctype_b_loc();
        while (v7 = (unsigned char)v16, *(unsigned char *)(v9 + 1 + v16 * 2) & 8) {
          v15 += 1;
          v16 = (unsigned long)a2[v15];
        }
        v4 = &a2[v15];
        if (v7 == 0x2e) {
          v4 = &a2[v15 + 1];
          v7 = *v4;
          if (*(unsigned char *)(v9 + 1 + (unsigned long)v7 * 2) & 8) {
            v11 = &a2[v15 + 2];
            do {
              v4 = v11;
              v7 = *v4;
              v11 = &v4[1];
            } while (*(unsigned char *)(v9 + 1 + (unsigned long)v7 * 2) & 8);
          }
        }
        v16 = (unsigned long)(unsigned int)(int)(char)v7;
        a2 = v4;
        if (strchr("abcdDfFgGhHiklmMnpPsStuUyYZ%",(int)(char)v7)) { // branch-flip
          if (v7) {
            v10 = 0;
            goto label_1068d;
          }
          v16 = 0;
label_1056a:
          if (strchr("{[(",(int)v16)) {
            error(1,0,dcgettext(NULL,"error: the format directive `%%%c\' is reserved for future use",5),(char)v16); // return-dupe
            return v6;
          }
          v4 = &a2[1];
          v8 = "warning: unrecognized format directive `%%%c\'";
        }
        else {
label_10610:
          if (!strchr("ABCT",(int)v16)) goto label_1056a;
          v10 = (int)(char)a2[1];
          v4 = &a2[1];
          if (a2[1]) goto label_1068d;
          if (strchr("{[(",(int)v16)) {
            error(1,0,dcgettext(NULL,"error: the format directive `%%%c\' is reserved for future use",5),(char)v16);
            return v6;
          }
          v8 = "warning: format directive `%%%c\' should be followed by another character";
        }
        error(0,0,dcgettext(NULL,v8,5),v16);
        v17 = (long *)sub_100b0(v17,v13,(int)v4 - v12,0,0,0,v3);
        v11 = v4;
      }
      else {
        v4 = &a2[1];
        v16 = 0x25;
        a2 = v4;
        if (!strchr("abcdDfFgGhHiklmMnpPsStuUyYZ%",0x25)) goto label_10610;
        v16 = 0x25;
        v10 = 0;
label_1068d:
        v11 = &v4[1];
        v17 = (long *)sub_100b0(v17,v13,(int)a2 - v12,2,v16,v10,v3);
        a2 = v4;
      }
      v13 = v11;
    }
label_1040f:
    v7 = a2[1];
    a2 = v11;
  } while (v7);
label_104b7:
  if (v11 <= v13)
    return 1;
  sub_100b0(v17,v13,(int)v11 - (int)v13,0,0,0,v3);
  return 1;
}

// Function: sub_10870 @ 0x10870
long sub_10870(char a0)
{
  long v1;
  long v2;
  
  if (dat_38e40) {
    if ((dat_38e38 == '\x01') || (v2 = dat_38e40, !a0))
      return dat_38e40;
    do {
      v1 = *(long *)(v2 + 0x30);
      sub_1efb0(v2);
      v2 = v1;
    } while (v1);
    dat_38e40 = 0;
  }
  dat_38e40 = sub_1e550((unsigned char)a0);
  dat_38e38 = a0;
  return dat_38e40;
}

// Function: sub_108e0 @ 0x108e0
void sub_108e0(long *a0) // return-dupe
{
  char *v1;
  int v2; // eax
  long v3;
  long v4;
  long v5 [19]; // stack - 0xc8
  long v6;
  
  if (dat_38e58) {
    if ((dat_38e50) && (*a0 == dat_38e48))
      return;
    free(dat_38e58);
  }
  dat_38e48 = *a0;
  v3 = sub_10870(1);
  if (!v3) {
    error(1,0,dcgettext(NULL,"Cannot read mounted file system list",5));
    return;
  }
  v6 = 0;
  do {
    v1 = *(char **)(v3 + 0x18);
    if (strcmp(v1,"ignore")) {
      v4 = *(long *)(v3 + 0x20);
      if (*(long *)(v3 + 0x20) == -1) {
        sub_d810(v5);
        v2 = (*dat_38670)(*(unsigned long *)(v3 + 8),v5);
        if (v2) goto label_10970;
        *(long *)(v3 + 0x20) = v5[0];
        v4 = v5[0];
      }
      if (*a0 == v4)
        v6 = v3;
    }
label_10970:
    v3 = *(long *)(v3 + 0x30);
    if (!v3) {
      if (v6) {
        v1 = *(char **)(v6 + 0x18);
        dat_38e58 = (void *)sub_25f30(v1);
        dat_38e50 = 1;
        return;
      }
      dat_38e50 = 0;
      dat_38e58 = (unsigned long)sub_25f30(dcgettext(NULL,"unknown",5));
      return;
    }
  } while( true );
}

// Function: sub_10a50 @ 0x10a50
unsigned long sub_10a50(char *a0)
{
  char *v1;
  long v2;
  
  if (strcmp("afs",a0)) {
    v2 = sub_10870(0);
    if (v2) {
      while( true ) {
        v1 = *(char **)(v2 + 0x18);
        if (!strcmp(a0,v1)) break;
        v2 = *(long *)(v2 + 0x30);
        if (!v2)
          return 0;
      }
    }
  }
  return 1;
}

// Function: sub_10ac0 @ 0x10ac0
void * sub_10ac0(long *a0)
{
  long v1;
  void *v10;
  long v11; // stack - 0x100
  unsigned long v2;
  int v3; // eax
  void *v4; // rax
  unsigned long v5; // stack - 0xe0
  long v6 [19]; // stack - 0xd8
  long v7;
  long v8;
  void *v9;
  
  v5 = 0;
  v4 = (long)sub_1e550(0);
  v7 = (long)v4;
  if (v4) { // branch-flip
    v11 = 0;
    v8 = (long)v4;
    v9 = NULL;
    do {
      while( true ) {
        v1 = v11 + 1;
        v2 = v1 * 8;
        v4 = (void *)sub_15be0(v9,v2,&v5);
        v10 = v4;
        if (v4) break;
        v10 = NULL;
        free(v9);
        v8 = *(long *)(v8 + 0x30);
        v9 = v10;
        if (!v8) goto label_10b90;
      }
      v4 = *(void **)(v8 + 0x20);
      if (v4 != (void *)0xffffffffffffffff) { // branch-flip
label_10b36:
        *(void **)((long)v10 + (v2 - 8)) = v4;
        v11 = v1;
      }
      else {
        sub_d810(v6);
        v3 = (*dat_38670)(*(unsigned long *)(v8 + 8),v6);
        if (!v3) {
          *(long *)(v8 + 0x20) = v6[0];
          v4 = (void *)v6[0];
          goto label_10b36;
        }
      }
      v8 = *(long *)(v8 + 0x30);
      v9 = v10;
    } while (v8);
label_10b90:
    do {
      v8 = *(long *)(v7 + 0x30);
      sub_1efb0(v7);
      v7 = v8;
    } while (v8);
    if (v10)
      *a0 = v11;
  }
  else {
    v10 = NULL;
  }
  v4 = v10;
  return v4;
}

// Function: sub_10c30 @ 0x10c30
bool sub_10c30(unsigned long a0,long a1,int *a2)
{
  long v1;
  
  if (a1) {
    v1 = *(long *)(a1 + (long)*a2 * 8);
    if (v1) {
      *a2 = *a2 + 1;
      dat_38690 = v1;
    }
    return v1 != 0;
  }
  return 0;
}

// Function: sub_10c70 @ 0x10c70
unsigned long sub_10c70(int a0,char *a1,char **a2)
{
  return sub_24500(a0,a1,a2); // tail-call
}

// Function: sub_10c80 @ 0x10c80
void sub_10c80(void)
{
  char *v1; // rdx
  unsigned int v2; // stack - 0x14
  long v3; // fs_offset
  unsigned long v4; // stack - 0x10
  
  v4 = *(unsigned long *)(v3 + 0x28);
  sub_16370("find");
  __printf_chk(1,dcgettext(NULL,"Features enabled: ",5));
  __printf_chk(1,"D_TYPE ");
  v1 = "enabled";
  if (!dat_38678)
    v1 = "disabled";
  __printf_chk(1,"O_NOFOLLOW(%s) ",v1);
  __printf_chk(1,"LEAF_OPTIMISATION ");
  if (1 <= is_selinux_enabled())
    __printf_chk(1,"SELINUX ");
  v2 = 0;
  if (sub_8530(&v2)) {
    __printf_chk(1,"FTS(");
    if (v2 & 0x200)
      __printf_chk(1,"FTS_CWDFD");
    __printf_chk(1,") ");
  }
  __printf_chk(1,"CBO(level=%d) ",(unsigned int)dat_38688);
  __printf_chk(1,"\n");
  exit(0); // no-return
}

// Function: sub_10dd0 @ 0x10dd0
void sub_10dd0(void)
{
  sub_d570(0); // no-return
}

// Function: sub_10df0 @ 0x10df0
unsigned long sub_10df0(int *a0)
{
  long v1; // rax
  
  v1 = sub_d520(a0);
  *(unsigned short *)(v1 + 0x1a) = 0;
  *(unsigned int *)(v1 + 0x24) = 0x3f800000;
  return 1;
}

// Function: sub_10e20 @ 0x10e20
unsigned long sub_10e20(int *a0)
{
  long v1; // rax
  
  v1 = sub_d520(a0);
  *(unsigned int *)(v1 + 0x18) = 1;
  *(unsigned int *)(v1 + 0x24) = 0x3f800000;
  return 1;
}

// Function: sub_10e50 @ 0x10e50
unsigned long sub_10e50(int *a0)
{
  long v1; // rax
  
  v1 = sub_d520(a0);
  if (!dat_38620)
    *(unsigned short *)(v1 + 0x1a) = 0;
  *(unsigned short *)(v1 + 0x18) = 1;
  return 1;
}

// Function: sub_10e80 @ 0x10e80
unsigned long sub_10e80(int *a0)
{
  *(unsigned int *)(sub_d520(a0) + 0x24) = 0x3a83126f;
  return 1;
}

// Function: sub_10ea0 @ 0x10ea0
unsigned long sub_10ea0(int *a0)
{
  struct_56 *v1; // rax
  unsigned int v2;
  
  v1 = (struct_56 *)sub_d520(a0);
  v1->field_0x18 = 0;
  v2 = dat_2bc38;
  if (v1->field_0x0 != sub_96e0)
    v2 = dat_2bc3c;
  v1->field_0x24 = v2;
  return 1;
}

// Function: sub_10ee0 @ 0x10ee0
unsigned long sub_10ee0(int *a0)
{
  *(unsigned int *)(sub_d520(a0) + 0x24) = 0x3c23d70a;
  return 1;
}

// Function: sub_10f00 @ 0x10f00
unsigned long sub_10f00(int *a0)
{
  long v1; // rax
  
  v1 = sub_d520(a0);
  dat_38620 = 0;
  *(unsigned int *)(v1 + 0x18) = 0x101;
  *(unsigned int *)(v1 + 0x24) = 0x3f800000;
  return 1;
}

// Function: sub_10f30 @ 0x10f30
char * sub_10f30(long a0,int *a1,struct_55 *a2,void *a3) // return-dupe x2
{
  char v1;
  char *v2;
  long v3;
  char *v4; // rax
  char *v5; // r12
  float v6;
  
  v5 = "-xtype";
  if (a3 != sub_9af0)
    v5 = "-type";
  if (!a0) {
    v4 = NULL;
    return v4;
  }
  v2 = *(char **)(a0 + (long)*a1 * 8);
  if (!v2) {
    v4 = NULL;
    return v4;
  }
  *a1 = *a1 + 1;
  if (*v2) {
    v4 = (long)sub_d490(a2,a3,v2);
    v3 = (long)v4;
    *(unsigned int *)((long)v4 + 0x24) = 0;
    *(bool *)((long)v4 + 0x1a) = a3 == sub_9af0;
    *(bool *)((long)v4 + 0x1b) = a3 != sub_9af0;
    v1 = *v2;
    if (!v1) {
      v4 = (char *)0x1;
      return v4;
    }
    do {
      if (0x30 <= (unsigned char)(v1 + 0xbcU)) {
label_110d2:
        v4 = dcgettext(NULL,"Unknown argument to %s: %c",5);
        error(1,0,v4,v5,v1);
        return v4;
      }
      switch(v1 + 0xbcU) {
        case 0:
          v4 = dcgettext(NULL,"%s %c is not supported because Solaris doors are not supported on the platform find was compiled on.",5);
          error(1,0,v4,v5,0x44);
          goto label_1113d;
        default:
          goto label_110d2;
        case 0x1e:
          v4 = NULL;
          v6 = dat_2bc44;
          break;
        case 0x1f:
          v4 = (char *)0x1;
          v6 = dat_2bc48;
          break;
        case 0x20:
          v4 = (char *)0x2;
          v6 = dat_2bc4c;
          break;
        case 0x22:
          v4 = (char *)0x3;
          v6 = dat_2bc50;
          break;
        case 0x28:
          v4 = (char *)0x4;
          v6 = dat_2bc54;
          break;
        case 0x2c:
          v4 = (char *)0x5;
          v6 = dat_2bc58;
          break;
        case 0x2f:
          v4 = (char *)0x6;
          v6 = dat_2bc5c;
        
      }
      if (*(char *)(v3 + 0x38 + (long)v4)) {
label_1113d:
        v4 = dcgettext(NULL,"Duplicate file type \'%c\' in the argument list to %s.",5);
        error(1,0,v4,(unsigned long)(unsigned int)(int)v1,v5);
label_11167:
        v4 = dcgettext(NULL,"Must separate multiple arguments to %s using: \',\'",5);
        error(1,0,v4,v5);
        break;
      }
      *(float *)(v3 + 0x24) = v6 + *(float *)(v3 + 0x24);
      *(char *)(v3 + 0x38 + (long)v4) = 1;
      if (!v2[1]) {
        v4 = (char *)0x1;
        return v4;
      }
      if (v2[1] != ',') goto label_11167;
      v1 = v2[2];
      v2 = &v2[2];
    } while (v1);
    v4 = dcgettext(NULL,"Last file type in list argument to %s is missing, i.e., list is ending on: \',\'",5);
    error(1,0,v4,v5);
  }
  v4 = dcgettext(NULL,"Arguments to %s should contain at least one letter",5);
  error(1,0,v4,v5);
  return v4;
}

// Function: sub_111e0 @ 0x111e0
void sub_111e0(struct_55 *a0,long a1,int *a2)
{
  sub_10f30(a1,a2,a0,sub_9af0); // tail-call
}

// Function: sub_11200 @ 0x11200
void sub_11200(struct_55 *a0,long a1,int *a2)
{
  sub_10f30(a1,a2,a0,sub_9900); // tail-call
}

// Function: sub_11220 @ 0x11220
char * sub_11220(void)
{
  int v1; // eax
  char *v2; // rax
  
  if (dat_38e60) {
    v2 = (char *)0x1;
    return v2;
  }
  v1 = fnmatch("foo","foo",0);
  if (!v1) {
    v1 = fnmatch("Foo","foo",0);
    if (v1) {
      v1 = fnmatch("Foo","foo",0x10);
      if (!v1) {
        dat_38e60 = 1;
        v2 = (char *)0x1;
        return v2;
      }
    }
  }
  v2 = dcgettext(NULL,"sanity check of the fnmatch() library function failed.",5);
  error(1,0,v2);
  return v2;
}

// Function: sub_112c0 @ 0x112c0
unsigned long sub_112c0(struct_55 *a0,long a1,int *a2,char *a3,void *a4) // return-dupe
{
  char *v1;
  long v2; // rax
  char *v3; // rax
  
  sub_11220();
  if (!a1)
    return 0;
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1)
    return 0;
  *a2 = *a2 + 1;
  v2 = sub_d490(a0,a4,v1);
  *(unsigned short *)(v2 + 0x1a) = 0;
  *(char **)(v2 + 0x38) = v1;
  if (strpbrk(v1,"*?[")) // branch-flip
    *(unsigned int *)(v2 + 0x24) = dat_2bc60;
  else {
    *(unsigned int *)(v2 + 0x24) = dat_2bc64;
  }
  if ((((!dat_38631) && (v3 = strrchr(v1,0x2f), v3)) && (!v3[1])) && (!sub_b4b0(v1,a4 == sub_91e0))) {
    error(0,0,dcgettext(NULL,"warning: -%s %s will not match anything because it ends with /.",5),a3,v1);
    *(unsigned int *)(v2 + 0x24) = 0x322bcc77;
  }
  return 1;
}

// Function: sub_113f0 @ 0x113f0
unsigned long sub_113f0(struct_55 *a0,long a1,int *a2)
{
  return sub_112c0(a0,a1,a2); // tail-call
}

// Function: sub_11410 @ 0x11410
unsigned long sub_11410(struct_55 *a0,long a1,int *a2)
{
  return sub_112c0(a0,a1,a2); // tail-call
}

// Function: sub_11430 @ 0x11430
unsigned long sub_11430(struct_55 *a0,long a1,int *a2)
{
  return sub_112c0(a0,a1,a2); // tail-call
}

// Function: sub_11450 @ 0x11450
unsigned long sub_11450(struct_55 *a0,long a1,int *a2)
{
  return sub_112c0(a0,a1,a2); // tail-call
}

// Function: sub_11470 @ 0x11470
passwd * sub_11470(int *a0,long a1,int *a2)
{
  char *v1;
  unsigned int v2; // eax
  passwd *v3; // rax
  unsigned int v4;
  unsigned int v5;
  
  if (!a1) {
    v3 = NULL;
    return v3;
  }
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (v1) {
    v2 = *a2 + 1;
    *a2 = v2;
    v3 = getpwnam(v1);
    endpwent();
    if (v3) // branch-flip
      v4 = *(unsigned int *)&v3->field_0x10;
    else {
      v3 = (unsigned long)strspn(v1,"0123456789");
      if ((!v3) || (v1[(long)v3])) {
        if (*v1) {
          v3 = (char *)sub_239f0(0,dat_3868c,v1);
          v1 = (char *)v3;
          v3 = (char *)dcgettext(NULL,"%s is not the name of a known user",5);
          error(1,0,(char *)v3,v1);
          return v3;
        }
        v3 = (char *)dcgettext(NULL,"The argument to -user should not be empty",5);
        error(1,0,(char *)v3);
        return v3;
      }
      v2 = sub_17410(v1,dat_3868c);
      v4 = v2;
    }
    v3 = (long)sub_d4d0(a0,v1);
    *(unsigned int *)((long)v3 + 0x38) = v4;
    v5 = dat_2bc68;
    if (100 <= v4)
      v5 = dat_2bc38;
    *(unsigned int *)((long)v3 + 0x24) = v5;
    v3 = (passwd *)0x1;
    return v3;
  }
  v3 = NULL;
  return v3;
}

// Function: sub_115c0 @ 0x115c0
unsigned long sub_115c0(int *a0)
{
  *(unsigned int *)(sub_d4d0(a0,0) + 0x24) = 0x38d1b717;
  return 1;
}

// Function: sub_115e0 @ 0x115e0
unsigned long sub_115e0(int *a0,long a1,int *a2) // return-dupe
{
  char *v1;
  long v2; // rax
  unsigned int v3;
  
  sub_11220();
  if (!a1)
    return 0;
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1)
    return 0;
  *a2 = *a2 + 1;
  v2 = sub_d4d0(a0,v1);
  *(char **)(v2 + 0x38) = v1;
  v3 = dat_2bc6c;
  if (!strpbrk(v1,"*?["))
    v3 = dat_2bc70;
  *(unsigned int *)(v2 + 0x24) = v3;
  return 1;
}

// Function: sub_11690 @ 0x11690
unsigned long sub_11690(int *a0,long a1,int *a2) // return-dupe
{
  char *v1;
  long v2; // rax
  unsigned int v3;
  
  if (!a1)
    return 0;
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1)
    return 0;
  *a2 = *a2 + 1;
  v2 = sub_d4d0(a0,v1);
  *(char **)(v2 + 0x38) = v1;
  v3 = dat_2bc6c;
  if (!strpbrk(v1,"*?["))
    v3 = dat_2bc70;
  *(unsigned int *)(v2 + 0x24) = v3;
  return 1;
}

// Function: sub_11720 @ 0x11720
char * sub_11720(int *a0,long a1,int *a2)
{
  unsigned char *v1;
  unsigned long v10;
  unsigned int v11;
  unsigned char v2;
  unsigned char *v3;
  int v4; // eax
  char *v5; // rax
  unsigned char v6;
  long v7; // stack - 0x48
  char v8 [14];
  unsigned char v9 [2]; // stack - 0x32
  
  if ((!a1) || (v5 = (long)(long)*a2, v3 = *(unsigned char **)(a1 + (long)v5 * 8), !v3)) {
    v5 = NULL;
    return v5;
  }
  v5 = (unsigned long)strlen((char *)v3);
  if (!v4) {
    v5 = dcgettext(NULL,"invalid null argument to -size",5);
    error(1,0,v5);
    return v5;
  }
  v5 = (long)(long)v4;
  v1 = &v3[(long)v5 + -1];
  v6 = *v1;
  if (v6 != 0x62) { // branch-flip
    if ('b' < (char)v6) {
      if (v6 != 0x6b) { // branch-flip
        if (v6 != 0x77) { // branch-flip
          if (v6 != 99) {
            v5 = dcgettext(NULL,"invalid -size type `%c\'",5); // return-dupe
            error(1,0,v5,v6);
            return v5;
          }
          *v1 = 0;
          v10 = 1;
        }
        else {
          *v1 = 0;
          v10 = 2;
        }
      }
      else {
        *v1 = 0;
        v10 = 0x400;
      }
      goto label_117a9;
    }
    if (v6 == 0x47) {
      *v1 = 0;
      v10 = 0x40000000;
      goto label_117a9;
    }
    if ('H' <= (char)v6) {
      if (v6 != 0x4d) {
        v5 = dcgettext(NULL,"invalid -size type `%c\'",5);
        error(1,0,v5,v6);
        return v5;
      }
      *v1 = 0;
      v10 = 0x100000;
      goto label_117a9;
    }
    if (9 < (unsigned char)(v6 - 0x30)) {
      v5 = dcgettext(NULL,"invalid -size type `%c\'",5);
      error(1,0,v5,v6);
      return v5;
    }
    v2 = *v3;
    v10 = 0x200;
    v6 = 0;
  }
  else {
    *v1 = 0;
    v10 = 0x200;
label_117a9:
    v2 = *v3;
  }
  v11 = (unsigned int)v10;
  if (v2 != 0x2b) { // branch-flip
    if (v2 != 0x2d) { // branch-flip
      v4 = sub_26490(v3,v8,10,&v7,"");
      if (v4) {
        v9[1] = 0; // return-dupe
        v9[0] = v6;
        v5 = dcgettext(NULL,"Invalid argument `%s%s\' to -size",5);
        error(1,0,v5,(char *)v3,(char *)v9);
        return v5;
      }
      v5 = (long)sub_d4d0(a0,v3);
      *(unsigned int *)((long)v5 + 0x38) = 2;
      *(unsigned int *)((long)v5 + 0x3c) = v11;
      v11 = dat_2bc40;
    }
    else {
      v4 = sub_26490(&v3[1],v8,10,&v7,"");
      if (v4) {
        v9[1] = 0;
        v9[0] = v6;
        v5 = dcgettext(NULL,"Invalid argument `%s%s\' to -size",5);
        error(1,0,v5,(char *)v3,(char *)v9);
        return v5;
      }
      v5 = (long)sub_d4d0(a0,v3);
      *(unsigned int *)((long)v5 + 0x3c) = v11;
      *(unsigned int *)((long)v5 + 0x38) = 1;
      v11 = dat_2bc3c;
      if ((unsigned long)(v10 * v7) <= 0x5000)
        v11 = dat_2bc64;
    }
  }
  else {
    v4 = sub_26490(&v3[1],v8,10,&v7,"");
    if (v4) {
      v9[1] = 0;
      v9[0] = v6;
      v5 = dcgettext(NULL,"Invalid argument `%s%s\' to -size",5);
      error(1,0,v5,(char *)v3,(char *)v9);
      return v5;
    }
    v5 = (long)sub_d4d0(a0,v3);
    *(unsigned int *)((long)v5 + 0x3c) = v11;
    *(unsigned int *)((long)v5 + 0x38) = 0;
    v11 = dat_2bc64;
    if ((unsigned long)(v10 * v7) <= 0x5000)
      v11 = dat_2bc3c;
  }
  *(long *)((long)v5 + 0x40) = v7;
  *(unsigned short *)((long)v5 + 0x1a) = 1;
  *(unsigned int *)((long)v5 + 0x24) = v11;
  *a2 = *a2 + 1;
  v5 = (char *)0x1;
  return v5;
}

// Function: sub_11a40 @ 0x11a40
unsigned long sub_11a40(int *a0,long a1,int *a2)
{
  char *v1;
  int v2;
  int v3; // eax
  long v4; // rax
  long v5; // stack - 0x168
  stat v6; // stack - 0xd8
  unsigned int v7;
  long v8; // stack - 0x160
  unsigned int v9; // stack - 0x150
  
  sub_d810(&v5);
  if (!a1)
    return 0;
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1)
    return 0;
  *a2 = *a2 + 1;
  v2 = (*dat_38670)(v1,&v5);
  if (v2)
    sub_e920(*__errno_location(),v1); // return-dupe, no-return
  sub_d810(&v6);
  v7 = 0;
  if (!dat_38668) {
    if (dat_38678) // branch-flip
      v7 = 0x20000;
    else if ((v9 & 0xf000) == 0xa000) {
      v2 = -2;
      goto label_11b07;
    }
  }
  v2 = sub_160b0(v1,v7);
  if (0 <= v2) {
    if ((fstat(v2,&v6)) || (v3 = (*dat_38670)(v1,&v5), v3)) {
      sub_e920(*__errno_location(),v1);
    }
    if ((dat_38668) || (dat_38678)) {
      v5 = v6._0_8_;
      v8 = v6._8_8_;
    }
    else if ((((v9 & 0xf000) == 0xa000) || (v5 != v6._0_8_)) || (v8 != v6._8_8_)) {
      close(v2);
      v2 = -1;
    }
  }
label_11b07:
  v4 = sub_d4d0(a0,v1);
  *(int *)(v4 + 0x48) = v2;
  *(long *)(v4 + 0x38) = v8;
  *(unsigned int *)(v4 + 0x24) = 0x3c23d70a;
  *(long *)(v4 + 0x40) = v5;
  *(unsigned short *)(v4 + 0x1a) = 1;
  return 1;
}

// Function: sub_11c90 @ 0x11c90
void sub_11c90(char *a0,struct_47 *a1)
{
  int v1; // eax
  FILE *v2;
  char *v3; // rax
  bool v4; // al
  
  a1->field_0x0 = 0;
  a1->field_0x20 = sub_23640(NULL);
  if (strcmp(a0,"/dev/stderr")) { // branch-flip
    if (strcmp(a0,"/dev/stdout")) { // branch-flip
      v2 = (FILE *)sub_eb40(dat_38610,a0);
      a1->field_0x10 = a0;
      a1->field_0x8 = v2;
      if (!v2)
        sub_e9b0(*__errno_location(),a0); // no-return
    }
    else {
      a1->field_0x8 = stdout;
      v3 = dcgettext(NULL,"standard output",5);
      v2 = a1->field_0x8;
      a1->field_0x10 = v3;
    }
  }
  else {
    a1->field_0x8 = stderr;
    v3 = dcgettext(NULL,"standard error",5);
    v2 = a1->field_0x8;
    a1->field_0x10 = v3;
  }
  v1 = fileno(v2);
  v4 = 0;
  if (v1 != -1)
    v4 = isatty(v1) != 0;
  a1->field_0x18 = v4;
}

// Function: sub_11d80 @ 0x11d80
unsigned long sub_11d80(struct_55 *a0,long a1,int *a2) // early-return x2
{
  int v1;
  unsigned char *v2;
  unsigned long v3; // rax
  char v4 [40];
  
  if (!a1)
    return 0;
  v1 = *a2;
  v2 = *(unsigned char **)(a1 + (long)v1 * 8);
  if (!v2)
    return 0;
  *a2 = v1 + 1;
  sub_11c90("/dev/stdout",v4);
  v3 = sub_10380(v4,a0,v2);
  if (!(char)v3)
    *a2 = v1;
  return v3;
}

// Function: sub_11e20 @ 0x11e20
unsigned long sub_11e20(int *a0)
{
  long v1; // rax
  
  v1 = sub_d520(a0);
  *(unsigned int *)(v1 + 0x18) = 0x101;
  sub_11c90("/dev/stdout",(struct_47 *)(v1 + 0x38));
  return 1;
}

// Function: sub_11e50 @ 0x11e50
bool sub_11e50(int *a0,long a1,int *a2)
{
  char *v1;
  long v2; // rax
  
  if (a1) {
    v1 = *(char **)(a1 + (long)*a2 * 8);
    if (v1) {
      *a2 = *a2 + 1;
      v2 = sub_d4d0(a0,v1);
      sub_11c90(v1,(struct_47 *)(v2 + 0x38));
      *(unsigned int *)(v2 + 0x18) = 0x101;
      *(unsigned int *)(v2 + 0x24) = 0x3f800000;
    }
    return v1 != NULL;
  }
  return 0;
}

// Function: sub_11ec0 @ 0x11ec0
bool sub_11ec0(int *a0,long a1,int *a2)
{
  char *v1;
  long v2; // rax
  
  if (a1) {
    v1 = *(char **)(a1 + (long)*a2 * 8);
    if (v1) {
      *a2 = *a2 + 1;
      v2 = sub_d520(a0);
      sub_11c90(v1,(struct_47 *)(v2 + 0x38));
      *(unsigned int *)(v2 + 0x24) = 0x3f800000;
      *(unsigned short *)(v2 + 0x18) = 0x101;
    }
    return v1 != NULL;
  }
  return 0;
}

// Function: sub_11f30 @ 0x11f30
unsigned long sub_11f30(struct_55 *a0,long a1,int *a2)
{
  char *v1;
  unsigned char *v2;
  unsigned long v3; // rax
  char v4 [40];
  int v5;
  
  v5 = *a2;
  if (a1) {
    v1 = *(char **)(a1 + (long)v5 * 8);
    if ((v1) && (v2 = *(unsigned char **)(a1 + 8 + (long)v5 * 8), v2)) {
      *a2 = v5 + 2;
      sub_11c90(v1,v4);
      v5 = *a2;
      v3 = sub_10380(v4,a0,v2);
      if ((char)v3)
        return v3;
    }
  }
  *a2 = v5;
  return 0;
}

// Function: sub_11fd0 @ 0x11fd0
bool sub_11fd0(int *a0,long a1,int *a2)
{
  char *v1;
  long v2; // rax
  
  if (a1) {
    v1 = *(char **)(a1 + (long)*a2 * 8);
    if (v1) {
      *a2 = *a2 + 1;
      v2 = sub_d4d0(a0,v1);
      sub_11c90(v1,(struct_47 *)(v2 + 0x38));
      *(unsigned int *)(v2 + 0x18) = 0x101;
      *(unsigned int *)(v2 + 0x24) = 0x3f800000;
    }
    return v1 != NULL;
  }
  return 0;
}

// Function: sub_12040 @ 0x12040
char * sub_12040(int *a0,long a1,int *a2)
{
  unsigned char *v1;
  int v2;
  char *v3;
  int v4; // eax
  char *v5; // rax
  unsigned long v6; // rbx
  unsigned char *v7;
  unsigned int v8; // r14d
  unsigned int v9; // stack - 0x40
  
  if (!a1) {
    v5 = NULL;
    return v5;
  }
  v1 = *(unsigned char **)(a1 + (long)*a2 * 8);
  if (v1) {
    v4 = *a2 + 1;
    *a2 = v4;
    if (*v1 != 0x2d) { // branch-flip
      if (*v1 != 0x2f) { // branch-flip
        v8 = 2;
        v9 = dat_2bc40;
        v7 = v1;
      }
      else {
        v9 = dat_2bc74;
        v7 = &v1[1];
        v8 = 1;
      }
    }
    else {
      v7 = &v1[1];
      v8 = 0;
      v9 = dat_2bc38;
    }
    v5 = (char *)sub_1def0(v7);
    v3 = v5;
    if ((v5) && ((*v1 != 0x2b || (8 <= (unsigned char)(v1[1] - 0x30))))) {
      v4 = sub_1e310(0,0,0,v3,NULL);
      v2 = v4;
      v4 = sub_1e310(0,1,0,v3,NULL);
      v6 = CONCAT44(v4,v2);
      free(v3);
      if ((*v1 == 0x2f) && (!v4 && !v2)) {
        v8 = 0;
        v5 = dcgettext(NULL,"warning: you have specified a mode pattern %s (which is equivalent to /000). The meaning of -perm /000 has now been changed to be consistent with -perm -000; that is, while it used to match no files, it now matches all files.",5);
        error(0,0,v5,(char *)v1);
        v9 = dat_2bc78;
      }
      v5 = (long)sub_d4d0(a0,v1);
      *(unsigned int *)((long)v5 + 0x38) = v8;
      *(unsigned long *)((long)v5 + 0x3c) = v6;
      *(unsigned int *)((long)v5 + 0x24) = v9;
      v5 = (char *)0x1;
      return v5;
    }
    v5 = (char *)sub_239f0(0,dat_3868c,v1);
    v3 = v5;
    v5 = dcgettext(NULL,"invalid mode %s",5);
    error(1,0,v5,v3);
    return v5;
  }
  v5 = NULL;
  return v5;
}

// Function: sub_12230 @ 0x12230
unsigned long sub_12230(char *a0,void *a1,long a2,int *a3)
{
  struct_57 *v1;
  unsigned long v10; // rax
  long v11;
  char *v12; // rax
  char *v13; // rax
  unsigned long v14; // rax
  long v15; // rax
  unsigned long v16; // rax
  unsigned long v17; // rax
  unsigned long v18; // rax
  unsigned long v19; // rax
  struct_6 *v2;
  long *v20;
  unsigned long v21; // stack - 0x60
  long v22; // stack - 0x68
  char v23 [24];
  int v24;
  int v25; // ebp
  long *v26;
  long *v27;
  char *v28; // stack - 0xb8
  char v3;
  int v4;
  void *v5;
  bool v6;
  void *v7;
  void *v8; // rax
  char *v9;
  
  v5 = *(void **)((long)a1 + 0x18);
  if ((!a2) || (!*(long *)(a2 + (long)*a3 * 8)))
    return 0;
  v8 = (void *)sub_d490(a1,v5,"(some -exec* arguments)");
  *(unsigned int *)((long)v8 + 0x18) = 0x101;
  *(unsigned long *)((long)v8 + 0xf8) = 0;
  if (v5 != sub_9580 && v5 != sub_95d0) {
    *(char *)((long)v8 + 0xf4) = 0;
    v7 = sub_8fc0;
label_123d6:
    if (v5 != v7) {
      if (!dat_385c0)
        __assert_fail("NULL != initial_wd","parser.c",0xb79,"insert_exec_ok"); // no-return
      *(long *)((long)v8 + 0xf8) = dat_385c0;
      goto label_123f8;
    }
  }
  else {
    dat_38698 = 1;
    *(char *)((long)v8 + 0xf4) = 1;
    if (v5 != sub_8fc0) {
      v7 = sub_95d0;
      goto label_123d6;
    }
  }
  dat_3862e = 0;
  v9 = getenv("PATH");
  if (v9) {
    sub_17570(v9,":",1,&v22,&v21);
    do {
      if (!v21) {
        error(1,0,dcgettext(NULL,"The current directory is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove the current directory from your $PATH (that is, remove \".\", doubled colons, or leading or trailing colons)",5),a0); // return-dupe
        return v10;
      }
      v3 = v9[v22];
      if ((v21 == 1) && (v3 == '.')) {
        error(1,0,dcgettext(NULL,"The current directory is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove the current directory from your $PATH (that is, remove \".\", doubled colons, or leading or trailing colons)",5),a0);
        return v10;
      }
      if (v3 != '/') {
        v12 = strndup(&v9[v22],v21);
        if (!v12)
          v12 = &v9[v22];
        v9 = (char *)sub_e8a0(0,v12);
        error(1,0,dcgettext(NULL,"The relative path %s is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove that entry from $PATH",5),v9,a0);
        return v17;
      }
    } while (sub_17570(v9,":",0,&v22,&v21));
  }
label_123f8:
  *(char *)((long)v8 + 0x38) = 0;
  v4 = *a3;
  v11 = (long)v4 * 8;
  v27 = (long *)(a2 + v11);
  v9 = (char *)*v27;
  v24 = v4;
  if (v9) {
    v26 = (long *)(a2 + 8 + v11);
    v28 = NULL;
    v25 = 0;
    v6 = 0;
    v20 = v27;
    do {
      if (*v9 != ';') { // branch-flip
        if ((((*v9 == '+') && (v5 != sub_9580 && v5 != sub_95d0)) && (!v9[1])) && (v6)) {
          *(char *)((long)v8 + 0x38) = 1;
label_124de:
          if ((v4 != v24) && (*v20)) {
            if (*(char *)((long)v8 + 0x38)) {
              v9 = "dir";
              if (v5 != sub_8fc0)
                v9 = "";
              if (1 < v25) {
                error(1,0,dcgettext(NULL,"Only one instance of {} is supported with -exec%s ... +",5),v9);
                return v19;
              }
              if (strlen(v28) != 2) {
                __snprintf_chk(v23,0x13,1,0x13,"-exec%s ... {} +",v9);
                v9 = (char *)sub_239f0(2,dat_3868c,v28);
                v12 = (char *)sub_239f0(1,dat_3868c,"{}");
                v13 = (char *)sub_239f0(0,dat_3868c,v23);
                error(1,0,dcgettext(NULL,"In %s the %s must appear by itself, but you specified %s",5),v13,v12,v9);
                return v14;
              }
            }
            v1 = (struct_57 *)((long)v8 + 0x40);
            if ((unsigned int)(sub_156e0(v1,0x800) - 1) < 2) {
              error(1,0,dcgettext(NULL,"The environment is too large for exec().",5));
              return v18;
            }
            sub_15860(v1);
            v25 = v24 - v4;
            v2 = (struct_6 *)((long)v8 + 0x98);
            *(void **)((long)v8 + 0x80) = sub_a3a0;
            if (*(char *)((long)v8 + 0x38)) { // branch-flip
              *(unsigned long *)((long)v8 + 0xe8) = 0;
              *(unsigned long *)((long)v8 + 0x70) = 0;
              *(long *)((long)v8 + 0x78) = (long)(v25 + -1);
              *(unsigned long *)((long)v8 + 0x68) = 0;
              *(unsigned long *)((long)v8 + 0x88) = 0;
              *(unsigned long *)((long)v8 + 0x90) = 0;
              sub_15890(v1,v2,(long)v8 + 0x38);
              if (v4 < v24 + -1) {
                do {
                  v9 = (char *)*v27;
                  v27 = &v27[1];
                  sub_14fb0(v1,v2,v9,strlen(v9) + 1,NULL,0,1);
                } while (v27 != (long *)(a2 + 8 + ((unsigned long)(unsigned int)((v24 - v4) - 2) + (long)v4) * 8));
              }
            }
            else {
              *(int *)((long)v8 + 0xf0) = v25;
              *(char **)((long)v8 + 0x70) = "{}";
              *(unsigned long *)((long)v8 + 0x68) = strlen("{}");
              *(unsigned long *)((long)v8 + 0x88) = 0;
              *(unsigned long *)((long)v8 + 0x90) = 0;
              *(unsigned long *)((long)v8 + 0xe8) = sub_25a50((long)v25 << 3);
              sub_15890(v1,v2,(long)v8 + 0x38);
              v4 = *(int *)((long)v8 + 0xf0);
              if (1 <= v4) {
                v11 = *(long *)((long)v8 + 0xe8);
                v15 = 0;
                do {
                  *(long *)(v11 + v15 * 8) = v27[v15];
                  v15 += 1;
                } while (v15 != v4);
              }
            }
            if (*v20) {
              *a3 = v24 + 1;
              return 1;
            }
            *a3 = v24;
            return 1;
          }
          goto label_1260a;
        }
      }
      else if (!v9[1]) goto label_124de;
      if (sub_1d5d0(v9,"{}")) { // branch-flip
        v25 += 1;
        if ((!v24) && ((v5 == sub_8fc0 || (v5 == sub_95d0)))) {
          error(1,0,dcgettext(NULL,"You may not use {} within the utility name for -execdir and -okdir, because this is a potential security problem.",5));
          return v16;
        }
        v6 = 1;
        v28 = v9;
      }
      else {
        v6 = 0;
      }
      v9 = (char *)*v26;
      v24 += 1;
      v26 = &v26[1];
      v20 = &v20[1];
    } while (v9);
  }
label_1260a:
  *a3 = v24;
  free(v8);
  return 0;
}

// Function: sub_12920 @ 0x12920
void sub_12920(void *a0,long a1,int *a2)
{
  sub_12230("-okdir",a0,a1,a2); // tail-call
}

// Function: sub_12940 @ 0x12940
void sub_12940(void *a0,long a1,int *a2)
{
  sub_12230("-ok",a0,a1,a2); // tail-call
}

// Function: sub_12960 @ 0x12960
void sub_12960(void *a0,long a1,int *a2)
{
  sub_12230("-execdir",a0,a1,a2); // tail-call
}

// Function: sub_12980 @ 0x12980
void sub_12980(void *a0,long a1,int *a2)
{
  sub_12230("-exec",a0,a1,a2); // tail-call
}

// Function: sub_129a0 @ 0x129a0
unsigned long sub_129a0(void)
{
  struct_48 *v1; // rax
  
  v1 = (struct_48 *)sub_b990();
  v1->field_0x10 = 0x200000003;
  v1->field_0x0 = sub_8940;
  v1->field_0x1a = 0;
  return 1;
}

// Function: sub_129e0 @ 0x129e0
unsigned long sub_129e0(void)
{
  struct_48 *v1; // rax
  
  v1 = (struct_48 *)sub_b990();
  v1->field_0x10 = 0x300000003;
  v1->field_0x0 = sub_8990;
  v1->field_0x1a = 0;
  return 1;
}

// Function: sub_12a20 @ 0x12a20
unsigned long sub_12a20(void)
{
  struct_59 *v1; // rax
  
  v1 = (struct_59 *)sub_b990();
  v1->field_0x10 = 0x100000003;
  v1->field_0x0 = sub_8ca0;
  v1->field_0x1a = 0;
  v1->field_0x24 = 0x3f800000;
  return 1;
}

// Function: sub_12a60 @ 0x12a60
unsigned long sub_12a60(void)
{
  struct_48 *v1; // rax
  
  v1 = (struct_48 *)sub_b990();
  v1->field_0x10 = 5;
  v1->field_0x0 = sub_8be0;
  v1->field_0x1a = 0;
  return 1;
}

// Function: sub_12a90 @ 0x12a90
unsigned long sub_12a90(int *a0,long a1,int *a2)
{
  int v1; // eax
  unsigned int v2;
  group *v3; // rax
  long v4; // rax
  unsigned long v5; // rax
  unsigned long v6; // rax
  char *v7; // rax
  char *v8;
  unsigned long v9; // rax
  
  if (!a1)
    return 0;
  v8 = *(char **)(a1 + (long)*a2 * 8);
  if (!v8)
    return 0;
  *a2 = *a2 + 1;
  v3 = getgrnam(v8);
  endgrent();
  if (v3) // branch-flip
    v2 = *(unsigned int *)&v3->field_0x10;
  else {
    v1 = (int)strspn(v8,"0123456789");
    if (!v1) {
      if (*v8) {
        v8 = (char *)sub_239f0(0,dat_3868c,v8);
        error(1,0,dcgettext(NULL,"%s is not the name of an existing group",5),v8);
        return v5;
      }
      error(1,0,dcgettext(NULL,"argument to -group is empty, but should be a group name",5));
      return v6;
    }
    if (v8[v1]) {
      v7 = (char *)sub_239f0(1,dat_3868c,&v8[v1]);
      v8 = (char *)sub_239f0(0,dat_3868c,v8);
      error(1,0,dcgettext(NULL,"%s is not the name of an existing group and it does not look like a numeric group ID because it has the unexpected suffix %s",5),v8,v7);
      return v9;
    }
    v2 = sub_17410(v8);
  }
  v4 = sub_d4d0(a0,v8);
  *(unsigned int *)(v4 + 0x38) = v2;
  v2 = dat_2bc68;
  if (100 <= *(unsigned long *)(v4 + 0x40))
    v2 = dat_2bc38;
  *(unsigned int *)(v4 + 0x24) = v2;
  return 1;
}

// Function: sub_12c20 @ 0x12c20
unsigned long sub_12c20(int *a0)
{
  struct_48 *v1; // rax
  
  v1 = (struct_48 *)sub_b9b0(a0,0);
  v1->field_0x10 = 0x400000002;
  v1->field_0x0 = sub_9350;
  v1->field_0x1a = 0;
  return 1;
}

// Function: sub_12c60 @ 0x12c60
unsigned long sub_12c60(int *a0)
{
  struct_48 *v1; // rax
  
  v1 = (struct_48 *)sub_b9b0(a0,0);
  v1->field_0x10 = 4;
  v1->field_0x0 = sub_9630;
  v1->field_0x1a = 0;
  return 1;
}

// Function: sub_12ca0 @ 0x12ca0
int * sub_12ca0(char *a0,int *a1)
{
  int v1;
  char *v2;
  
  v2 = dat_38e70;
  v1 = *a1;
  if (v1 != 2) {
    if (v1 != 1) { // branch-flip
      if (v1) { // branch-flip
        if (!dat_38e70)
          dat_38e70 = a0;
      }
      else if (((dat_38e70) && (!dat_38631)) && (dat_38630))
        error(0,0,dcgettext(NULL,"warning: you have specified the global option %s after the argument %s, but global options are not positional, i.e., %s affects tests specified before it as well as those specified after it.  Please specify global options before other arguments.",5),a0,v2,a0);
    }
    else {
      a1 = NULL;
    }
  }
  return a1;
}

// Function: sub_12d30 @ 0x12d30
unsigned long sub_12d30(unsigned int a0,char *a1,char **a2,unsigned int a3)
{
  int v1;
  unsigned long v2; // rax
  
  v1 = *__errno_location();
  if ((v1 != 2) && (v1 != 0x14))
    return (unsigned long)a3;
  if (dat_38660 & 2)
    __fprintf_chk(stderr,1,"fallback_getfilecon(): getfilecon(%s) failed; falling back on lgetfilecon()\n",a1);
  v2 = sub_24500(a0,a1,a2); // tail-call
  return v2;
}

// Function: sub_12dc0 @ 0x12dc0
unsigned long sub_12dc0(unsigned long a0,char *a1,char **a2)
{
  int v1; // eax
  
  v1 = sub_242e0(a0,a1,a2);
  if (!v1)
    return 0;
  return sub_12d30(a0 & 0xffffffff,a1,a2,v1); // tail-call
}

// Function: sub_12e00 @ 0x12e00
unsigned long sub_12e00(unsigned long a0,char *a1,char **a2)
{
  int v1; // eax
  unsigned long v2; // rax
  
  if (dat_385e0) {
    v2 = sub_24500(a0,a1,a2); // tail-call
    return v2;
  }
  v1 = sub_242e0(a0,a1,a2);
  if (!v1)
    return 0;
  v2 = sub_12d30(a0 & 0xffffffff,a1,a2,v1); // tail-call
  return v2;
}

// Function: sub_12e60 @ 0x12e60
char * sub_12e60(int *a0,long a1,int *a2)
{
  int v1;
  unsigned long v2;
  char *v3; // rax
  
  if (!a1) {
    v3 = NULL;
    return v3;
  }
  v3 = (long)(long)*a2;
  if (*(long *)(a1 + (long)v3 * 8)) {
    if (1 <= is_selinux_enabled()) {
      v3 = (long)sub_d4d0(a0,0);
      v1 = *a2;
      *(unsigned int *)((long)v3 + 0x24) = 0x3c23d70a;
      v2 = *(unsigned long *)(a1 + (long)v1 * 8);
      *(char *)((long)v3 + 0x1a) = 0;
      *(unsigned long *)((long)v3 + 0x38) = v2;
      v3 = (char *)0x1;
      *a2 = *a2 + 1;
      return v3;
    }
    v3 = dcgettext(NULL,"invalid predicate -context: SELinux is not enabled.",5);
    error(1,0,v3);
    return v3;
  }
  v3 = NULL;
  return v3;
}

// Function: sub_12f00 @ 0x12f00
unsigned long sub_12f00(char *a0,char *a1,char *a2)
{
  char *v1; // rax
  char *v2; // rax
  char *v3; // rax
  
  if ((!dat_38631) && (dat_38630)) {
    if (strchr(a2,0x2f)) {
      v1 = (char *)sub_e8a0(2,a1);
      v2 = (char *)sub_e8a0(1,"/");
      v3 = (char *)sub_e8a0(0,a0);
      error(0,0,dcgettext(NULL,"warning: %s matches against basenames only, but the given pattern contains a directory separator (%s), thus the expression will evaluate to false all the time.  Did you mean %s?",5),v3,v2,v1);
    }
    return 1;
  }
  return 1;
}

// Function: sub_12fc0 @ 0x12fc0
unsigned int sub_12fc0(int *a0,long a1,int *a2)
{
  int v1;
  char *v2;
  unsigned int v3; // eax
  long v4; // rax
  unsigned int v5;
  
  if (!a1)
    return 0;
  v1 = *a2;
  v3 = 0;
  v2 = *(char **)(a1 + (long)v1 * 8);
  if (v2) {
    *a2 = v1 + 1;
    sub_11220();
    v3 = sub_12f00("-name","-wholename",v2);
    if ((char)v3) {
      v4 = sub_d4d0(a0,v2);
      *(unsigned short *)(v4 + 0x1a) = 0;
      *(char **)(v4 + 0x38) = v2;
      v5 = dat_2bc60;
      if (!strpbrk(v2,"*?["))
        v5 = dat_2bc64;
      *(unsigned int *)(v4 + 0x24) = v5;
      return v3;
    }
    *a2 = v1;
  }
  return v3;
}

// Function: sub_13090 @ 0x13090
unsigned int sub_13090(int *a0,long a1,int *a2) // return-dupe x2
{
  char *v1;
  unsigned int v2; // eax
  long v3; // rax
  unsigned int v4;
  
  sub_11220();
  if (!a1)
    return 0;
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1)
    return 0;
  *a2 = *a2 + 1;
  v2 = sub_12f00("-iname","-iwholename",v1);
  if (!(char)v2)
    return 0;
  v3 = sub_d4d0(a0,v1);
  *(unsigned short *)(v3 + 0x1a) = 0;
  *(char **)(v3 + 0x38) = v1;
  v4 = dat_2bc60;
  if (!strpbrk(v1,"*?["))
    v4 = dat_2bc64;
  *(unsigned int *)(v3 + 0x24) = v4;
  return v2;
}

// Function: sub_13150 @ 0x13150
unsigned long sub_13150(void) // ternary
{
  tm *v1; // rax
  long v2; // rdx
  
  if (dat_38658)
    return 1;
  dat_38648 += 0x15180;
  dat_38650 = 0;
  v1 = localtime((void *)0x38648);
  v2 = (v1) ? (long)(*(int *)&v1->field_0x4 * 0x3c + *(int *)v1 + *(int *)&v1->field_0x8 * 0xe10) : dat_38648 % 0x15180; // branch-flip
  dat_38658 = 1;
  dat_38648 -= v2;
  return 1;
}

// Function: sub_13200 @ 0x13200
unsigned long sub_13200(long a0,int *a1,struct_55 *a2,int a3)
{
  char *v1;
  long v2; // rax
  re_pattern_buffer *v3; // rax
  unsigned long v4; // rax
  char *v5; // rax
  unsigned long v6; // rax
  unsigned int v7;
  
  if (!a0)
    return 0;
  v1 = *(char **)(a0 + (long)*a1 * 8);
  if (v1) {
    *a1 = *a1 + 1;
    v2 = sub_d490(a2,sub_97e0,v1);
    *(unsigned short *)(v2 + 0x1a) = 0;
    v3 = (re_pattern_buffer *)sub_25a50(0x40);
    *(re_pattern_buffer **)(v2 + 0x38) = v3;
    *(unsigned long *)&v3->field_0x8 = 100;
    v4 = sub_25a50(100);
    *(unsigned long *)&v3->field_0x20 = 0;
    *(unsigned long *)v3 = v4;
    re_set_syntax((long)a3);
    *(long *)&v3->field_0x18 = (long)a3;
    *(unsigned long *)&v3->field_0x28 = 0;
    v5 = re_compile_pattern(v1,strlen(v1),v3);
    if (!v5) {
      v5 = strpbrk(v1,"*?[");
      v7 = dat_2bc60;
      if (!v5) {
        if (!strpbrk(v1,"."))
          v7 = dat_2bc64;
      }
      *(unsigned int *)(v2 + 0x24) = v7;
      return 1;
    }
    error(1,0,dcgettext(NULL,"failed to compile regular expression \'%s\': %s",5),v1,v5);
    return v6;
  }
  return 0;
}

// Function: sub_13360 @ 0x13360
void sub_13360(struct_55 *a0,long a1,int *a2)
{
  sub_13200(a1,a2,a0,dat_3867c); // tail-call
}

// Function: sub_13380 @ 0x13380
void sub_13380(struct_55 *a0,long a1,int *a2)
{
  sub_13200(a1,a2,a0,dat_3867c | 0x400000); // tail-call
}

// Function: sub_133b0 @ 0x133b0
unsigned long sub_133b0(void)
{
  int v1;
  long v2;
  int *v3;
  int *v4; // rax
  int v5; // edx
  
  v5 = 0;
  v3 = (int *)0x36d80;
  v2 = dat_38e68;
  if (!dat_38e68) {
    do {
      v2 = *(long *)&v3[2];
      v5 += 1;
      if (!v2) goto label_133c9;
      v4 = &v3[8];
      v1 = *v3;
      v3 = v4;
    } while (v1 != 1);
    v2 = (long)v5 * 0x20 + 0x36d60;
    dat_38e68 = v2;
  }
label_133c9:
  v2 = sub_d520(v2);
  *(unsigned short *)(v2 + 0x1a) = 0;
  *(unsigned int *)(v2 + 0x24) = 0x3f800000;
  return 1;
}

// Function: sub_13420 @ 0x13420
unsigned long sub_13420(unsigned long a0,long a1,int *a2) // return-dupe
{
  char *v1;
  unsigned long v2; // rax
  
  if (!a1)
    return 0;
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1)
    return 0;
  *a2 = *a2 + 1;
  dat_3867c = sub_17200(v1);
  v2 = sub_133b0(); // tail-call
  return v2;
}

// Function: sub_13460 @ 0x13460
char * sub_13460(unsigned long a0,long a1,int *a2,int *a3)
{
  char *v1;
  char *v2;
  int v3;
  char *v4; // rax
  unsigned int v5;
  
  v3 = *a2;
  v1 = *(char **)(a1 + -8 + (long)v3 * 8);
  if ((a1) && (v2 = *(char **)(a1 + (long)v3 * 8), v2)) {
    v3 += 1;
    *a2 = v3;
    v4 = (unsigned long)strspn(v2,"0123456789");
    v5 = dat_3868c;
    if ((1 <= v3) && (v4 = (long)(long)v3, !v2[(long)v4])) {
      v3 = sub_17410(v2,dat_3868c);
      *a3 = v3;
      if (0 <= v3) {
        v4 = (unsigned long)sub_133b0(); // tail-call
        return v4;
      }
      v5 = dat_3868c;
    }
    v4 = (char *)sub_239f0(0,v5,v2);
    v2 = v4;
    v4 = dcgettext(NULL,"Expected a positive decimal integer argument to %s, but got %s",5);
    error(1,0,v4,v1,v2);
    return v4;
  }
  v4 = NULL;
  return v4;
}

// Function: sub_13510 @ 0x13510
void sub_13510(void)
{
  sub_13460(); // tail-call
}

// Function: sub_13520 @ 0x13520
void sub_13520(void)
{
  sub_13460(); // tail-call
}

// Function: sub_13530 @ 0x13530
void sub_13530(void)
{
  if ((!dat_38631) && (dat_38630)) {
    error(0,0,dcgettext(NULL,"warning: the -d option is deprecated; please use -depth instead, because the latter is a POSIX-compliant feature.",5));
    dat_38620 = 0x100;
    sub_133b0(); // tail-call
    return;
  }
  dat_38620 = 0x100;
  sub_133b0(); // tail-call
}

// Function: sub_135a0 @ 0x135a0
void sub_135a0(void)
{
  dat_38620 = 0x100;
  sub_133b0(); // tail-call
}

// Function: sub_135c0 @ 0x135c0
void sub_135c0(void)
{
  dat_3862e = 1;
  sub_133b0(); // tail-call
}

// Function: sub_135d0 @ 0x135d0
void sub_135d0(void)
{
  dat_3862d = 1;
  sub_133b0(); // tail-call
}

// Function: sub_135e0 @ 0x135e0
void sub_135e0(void)
{
  dat_3862c = 1;
  sub_133b0(); // tail-call
}

// Function: sub_135f0 @ 0x135f0
void sub_135f0(void)
{
  dat_3862e = 0;
  sub_133b0(); // tail-call
}

// Function: sub_13600 @ 0x13600
void sub_13600(void)
{
  dat_38630 = 0;
  sub_133b0(); // tail-call
}

// Function: sub_13610 @ 0x13610
void sub_13610(void)
{
  dat_38630 = 1;
  sub_133b0(); // tail-call
}

// Function: sub_13620 @ 0x13620
unsigned long sub_13620(int *a0,long a1,int *a2,unsigned long a3,unsigned long a4,unsigned long a5)
{
  char v1;
  char v10 [6]; // stack - 0x46
  char v11 [72];
  bool v12; // r9b
  unsigned long v13; // r9
  float v14; // xmm0_da
  unsigned int v15;
  long v16; // stack - 0x90
  long v17; // stack - 0x88
  long v18; // stack - 0x80
  long v19; // stack - 0x78
  char v2;
  long v20; // stack - 0x70
  long v21; // stack - 0x68
  void *v3;
  int v4;
  long *v5; // rax
  unsigned long v6; // rax
  char *v7;
  unsigned long v8; // rax
  int *v9; // rax
  
  if (a1) {
    v4 = *a2;
    v7 = *(char **)(a1 + (long)v4 * 8);
    if ((v7) && (strlen(v7) == 8)) {
      builtin_strncpy(v10,"aBcmt",6);
      if (strncmp("-newer",v7,6))
        __assert_fail("0 == strncmp (\"-newer\", argv[*arg_ptr], 6)","parser.c",0x613,"parse_newerXY"); // no-return
      v1 = v7[6];
      v2 = v7[7];
      v12 = v1 == 'B' || v2 == 'B';
      v13 = CONCAT71((undefined7)((unsigned long)a5 >> 8),v12);
      if (v12) {
        error(0,0,dcgettext(NULL,"This system does not provide a way to find the birth time of a file.",5));
        v13 = 0;
      }
      else if (v1 != 't') {
        v13 = 0;
        if (strchr(v10,(int)v1)) {
          v13 = 0;
          if (strchr(v10,(int)v2)) {
            if (!*(long *)(a1 + 8 + (long)v4 * 8)) {
              v7 = (char *)sub_239f0(0,dat_3868c,v7);
              error(1,0,dcgettext(NULL,"The %s test needs an argument",5),v7);
              return v6;
            }
            *a2 = v4 + 1;
            v5 = (long *)sub_d4d0(a0);
            if (v1 != 'c') { // branch-flip
              if (v1 != 'm') { // branch-flip
                if (v1 != 'a') {
                  if (strchr(v10,(int)v1))
                    __assert_fail("0","parser.c",0x64c,"parse_newerXY"); // no-return
                  __assert_fail("strchr (validchars, x)","parser.c",0x64b,"parse_newerXY"); // no-return
                }
                v15 = 0;
              }
              else {
                v15 = 3;
              }
            }
            else {
              v15 = 2;
            }
            *(unsigned int *)&v5[7] = v15;
            if (v2 != 't') { // branch-flip
              sub_d810(v11);
              v4 = (*dat_38670)(*(unsigned long *)(a1 + (long)*a2 * 8),v11);
              if (v4) {
                v9 = __errno_location();
                sub_e920(*v9,*(char **)(a1 + (long)*a2 * 8)); // no-return
              }
              if (v2 != 'c') { // branch-flip
                if ('d' <= v2) { // branch-flip
                  if (v2 != 'm')
                    __assert_fail("0","parser.c",0x1f2,"get_stat_Ytime"); // no-return, return-dupe
                  v16 = v18;
                  v17 = v19;
                }
                else if (v2 != 'a') {
                  __assert_fail("0","parser.c",0x1f2,"get_stat_Ytime");
                }
              }
              else {
                v16 = v20;
                v17 = v21;
              }
              v5[8] = v16;
              v5[9] = v17;
            }
            else {
              v7 = *(char **)(a1 + (long)*a2 * 8);
              if (!sub_21de0((struct_12 *)&v5[8],v7,(long *)0x38638)) {
                v7 = (char *)sub_239f0(0,dat_3868c,*(char **)(a1 + (long)*a2 * 8));
                error(1,0,dcgettext(NULL,"I cannot figure out how to interpret %s as a date or time",5),v7);
                return v8;
              }
              v16 = v5[8];
            }
            *(unsigned int *)((long)v5 + 0x3c) = 0;
            v14 = (float)(int)((dat_38648 - v16) / 0x15180);
            v15 = dat_2bc40;
            if (((dat_2bc64 <= v14) && (v15 = dat_2bc74, dat_29578 <= v14)) && (v14 <= dat_2bc80))
              v15 = dat_2bc7c;
            v3 = (void *)*v5;
            *(unsigned int *)((long)v5 + 0x24) = v15;
            *a2 = *a2 + 1;
            if (!v3)
              __assert_fail("our_pred->pred_func != NULL","parser.c",0x66f,"parse_newerXY"); // no-return
            if (v3 != sub_9400)
              __assert_fail("our_pred->pred_func == pred_newerXY","parser.c",0x670,"parse_newerXY"); // no-return
            v13 = (unsigned long)*(unsigned char *)((long)v5 + 0x1a);
            if (!*(unsigned char *)((long)v5 + 0x1a))
              __assert_fail("our_pred->need_stat","parser.c",0x671,"parse_newerXY"); // no-return
          }
        }
      }
      return v13 & 0xffffffff; // return-dupe
    }
  }
  v13 = 0;
  return v13 & 0xffffffff;
}

// Function: sub_13b00 @ 0x13b00
unsigned long sub_13b00(int *a0)
{
  long v1; // rax
  
  v1 = sub_d520(a0);
  sub_11c90("/dev/stdout",(struct_47 *)(v1 + 0x38));
  *(unsigned int *)(v1 + 0x24) = 0x3f800000;
  *(unsigned short *)(v1 + 0x18) = 0x101;
  return 1;
}

// Function: sub_13b40 @ 0x13b40
unsigned long sub_13b40(int *a0)
{
  long v1; // rax
  
  v1 = sub_d4d0(a0,0);
  sub_11c90("/dev/stdout",(struct_47 *)(v1 + 0x38));
  *(unsigned int *)(v1 + 0x18) = 0x101;
  *(unsigned int *)(v1 + 0x24) = 0x3f800000;
  return 1;
}

// Function: sub_13b80 @ 0x13b80
void sub_13b80(void)
{
  dat_3862c = 1;
  dat_38680 = sub_12dc0;
  dat_38668 = 1;
  if (!(dat_38660 & 2)) {
    dat_38670 = sub_ddb0;
    sub_133b0(); // tail-call
    return;
  }
  dat_38670 = sub_dec0;
  sub_133b0(); // tail-call
}

// Function: sub_13be0 @ 0x13be0
unsigned long sub_13be0(double a0,char *a1,void *a2,long a3,long a4,char *a5)
{
  unsigned int v1; // eax
  long v2; // rax
  unsigned long v3; // rax
  long v4; // rdx
  double v5; // stack - 0x50
  double v6; // stack - 0x48
  double v7; // xmm0_qa
  
  if (*a1 != '+') { // branch-flip
    if (*a1 != '-') // branch-flip
      *(unsigned int *)((long)a2 + 4) = 2;
    else {
      *(unsigned int *)((long)a2 + 4) = 0;
      a1 = &a1[1];
    }
  }
  else {
    *(unsigned int *)((long)a2 + 4) = 1;
    a1 = &a1[1];
  }
  v1 = sub_25f90(a1,NULL,&v5,dat_37fe8);
  if ((char)v1) {
    v7 = (double)modf(a0 * v5,&v6) * dat_2bc88;
    if (dat_2bc88 <= v7)
      __assert_fail("nanosec < nanosec_per_sec","parser.c",0xc4d,"get_relative_timestamp"); // no-return
    v2 = (long)((double)a3 - v6);
    *(long *)((long)a2 + 8) = v2;
    if (a3 < v2 != v6 < 0.0) {
      error(1,0,a5,a1);
      return v3;
    }
    v4 = (long)((double)a4 - v7);
    if (v7 <= (double)a4)
      *(long *)((long)a2 + 0x10) = v4;
    else {
      *(long *)((long)a2 + 0x10) = v4 + 1000000000;
      *(long *)((long)a2 + 8) = v2 + -1;
    }
  }
  return (unsigned long)v1;
}

// Function: sub_13d80 @ 0x13d80
char * sub_13d80(int *a0,long a1,int *a2) // return-dupe
{
  char *v1;
  unsigned int v2; // eax
  char *v3; // rax
  unsigned long v4; // stack - 0x38
  unsigned long v5; // r12
  float v6; // xmm0_da
  unsigned int v7;
  long v8; // stack - 0x30
  unsigned long v9; // stack - 0x28
  
  if (!a1) {
    v3 = NULL;
    return v3;
  }
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1) {
    v3 = NULL;
    return v3;
  }
  v2 = *a2 + 1;
  *a2 = v2;
  v2 = sub_13be0(v1,&v4,0,0,"arithmetic overflow while converting %s days to a number of seconds");
  v5 = (unsigned long)v2;
  if (!(char)v2) {
    v3 = dcgettext(NULL,"Invalid argument %s to -used",5);
    error(1,0,v3,v1);
    return v3;
  }
  v3 = (long)sub_d4d0(a0,v1);
  *(unsigned long *)((long)v3 + 0x38) = v4;
  *(long *)((long)v3 + 0x40) = v8;
  *(unsigned long *)((long)v3 + 0x48) = v9;
  v6 = (float)(v8 / 0x15180);
  v7 = dat_2bc40;
  if (((dat_2bc64 <= v6) && (v7 = dat_2bc74, dat_29578 <= v6)) && (v6 <= dat_2bc80))
    v7 = dat_2bc7c;
  *(unsigned int *)((long)v3 + 0x24) = v7;
  v3 = (char *)v5;
  return v3;
}

// Function: sub_13ed0 @ 0x13ed0
long sub_13ed0(long a0,int *a1,int *a2) // return-dupe
{
  unsigned char *v1;
  long v2; // rax
  char *v3; // rcx
  int v4; // ebx
  unsigned long v5; // stack - 0x30
  char v6 [8];
  unsigned char *v7;
  char *v8; // r8
  
  if (!a0)
    return 0;
  v1 = *(unsigned char **)(a0 + (long)*a1 * 8);
  if (v1) {
    *a1 = *a1 + 1;
    if (*v1 != 0x2b) { // branch-flip
      if (*v1 != 0x2d) { // branch-flip
        v4 = 2;
        v7 = v1;
      }
      else {
        v7 = &v1[1];
        v4 = 1;
      }
    }
    else {
      v7 = &v1[1];
      v4 = 0;
    }
    if (sub_26490(v7,v6,10,&v5,""))
      return 0;
    v2 = sub_d4d0(a2,v1);
    *(int *)(v2 + 0x38) = v4;
    *(unsigned long *)(v2 + 0x40) = v5;
    if (dat_38660 & 1) {
      __fprintf_chk(stderr,1,"inserting %s\n",*(char **)(v2 + 8));
      v8 = " >";
      v3 = "gt";
      if (v4) {
        v8 = " =";
        if (v4 == 1)
          v8 = " <";
        v3 = "eq";
        if (v4 == 1)
          v3 = "lt";
      }
      __fprintf_chk(stderr,1,"    type: %s    %s  ",v3,v8);
      __fprintf_chk(stderr,1,"%lu\n",*(unsigned long *)(v2 + 0x40));
      return v2;
    }
    return v2;
  }
  return 0;
}

// Function: sub_14060 @ 0x14060
unsigned long sub_14060(int *a0,long a1,int *a2)
{
  long v1; // rax
  
  v1 = sub_13ed0(a1,a2,a0);
  if (!v1) {
    *a2 = *a2 + -1;
    return 0;
  }
  if (*(long *)(v1 + 0x40) != 1) {
    if (*(long *)(v1 + 0x40) != 2) {
      *(unsigned int *)(v1 + 0x24) = 0x3a83126f;
      return 1;
    }
    *(unsigned int *)(v1 + 0x24) = 0x3c23d70a;
    return 1;
  }
  *(unsigned int *)(v1 + 0x24) = 0x3f7d70a4;
  return 1;
}

// Function: sub_140d0 @ 0x140d0
unsigned long sub_140d0(int *a0,long a1,int *a2)
{
  long v1; // rax
  
  v1 = sub_13ed0(a1,a2,a0);
  if (v1) {
    *(unsigned int *)(v1 + 0x24) = 0x358637bd;
    *(unsigned short *)(v1 + 0x1a) = 0;
    *(char *)(v1 + 0x1c) = 1;
    return 1;
  }
  *a2 = *a2 + -1;
  return 0;
}

// Function: sub_14120 @ 0x14120
unsigned long sub_14120(int *a0,long a1,int *a2) // return-dupe
{
  long v1; // rax
  
  v1 = sub_13ed0(a1,a2,a0);
  if (!v1) {
    *a2 = *a2 + -1;
    return 0;
  }
  if (*(unsigned long *)(v1 + 0x40) <= 99) {
    *(unsigned int *)(v1 + 0x24) = dat_2bc68;
    return 1;
  }
  *(unsigned int *)(v1 + 0x24) = dat_2bc38;
  return 1;
}

// Function: sub_14180 @ 0x14180
unsigned long sub_14180(int *a0,long a1,int *a2) // return-dupe
{
  long v1; // rax
  
  v1 = sub_13ed0(a1,a2,a0);
  if (!v1) {
    *a2 = *a2 + -1;
    return 0;
  }
  if (*(unsigned long *)(v1 + 0x40) <= 99) {
    *(unsigned int *)(v1 + 0x24) = dat_2bc68;
    return 1;
  }
  *(unsigned int *)(v1 + 0x24) = dat_2bc38;
  return 1;
}

// Function: sub_141e0 @ 0x141e0
unsigned long sub_141e0(int *a0,long a1,int *a2) // return-dupe
{
  char *v1;
  int v2; // eax
  long v3; // rax
  char v4 [88];
  float v5; // xmm0_da
  unsigned int v6;
  long v7; // stack - 0x70
  unsigned long v8; // stack - 0x68
  
  sub_d810(v4);
  if (!a1)
    return 0;
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1)
    return 0;
  *a2 = *a2 + 1;
  v2 = (*dat_38670)(v1,v4);
  if (v2)
    sub_e920(*__errno_location(),v1); // no-return
  v3 = sub_d4d0(a0,v1);
  *(unsigned long *)(v3 + 0x48) = v8;
  *(long *)(v3 + 0x40) = v7;
  *(unsigned long *)(v3 + 0x38) = 3;
  v5 = (float)(int)((dat_38648 - v7) / 0x15180);
  v6 = dat_2bc40;
  if (((dat_2bc64 <= v5) && (v6 = dat_2bc74, dat_29578 <= v5)) && (v5 <= dat_2bc80))
    v6 = dat_2bc7c;
  *(unsigned int *)(v3 + 0x24) = v6;
  return 1;
}

// Function: sub_14340 @ 0x14340
unsigned long sub_14340(int *a0,long a1,int *a2) // return-dupe
{
  char *v1;
  int v2; // eax
  long v3; // rax
  char v4 [88];
  float v5; // xmm0_da
  unsigned int v6;
  long v7; // stack - 0x70
  unsigned long v8; // stack - 0x68
  
  sub_d810(v4);
  if (!a1)
    return 0;
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1)
    return 0;
  *a2 = *a2 + 1;
  v2 = (*dat_38670)(v1,v4);
  if (v2)
    sub_e920(*__errno_location(),v1); // no-return
  v3 = sub_d4d0(a0,v1);
  *(unsigned long *)(v3 + 0x48) = v8;
  *(long *)(v3 + 0x40) = v7;
  *(unsigned long *)(v3 + 0x38) = 0;
  v5 = (float)(int)((dat_38648 - v7) / 0x15180);
  v6 = dat_2bc40;
  if (((dat_2bc64 <= v5) && (v6 = dat_2bc74, dat_29578 <= v5)) && (v5 <= dat_2bc80))
    v6 = dat_2bc7c;
  *(unsigned int *)(v3 + 0x24) = v6;
  return 1;
}

// Function: sub_144a0 @ 0x144a0
unsigned long sub_144a0(int *a0,long a1,int *a2) // return-dupe
{
  char *v1;
  int v2; // eax
  long v3; // rax
  char v4 [88];
  float v5; // xmm0_da
  unsigned int v6;
  long v7; // stack - 0x70
  unsigned long v8; // stack - 0x68
  
  sub_d810(v4);
  if (!a1)
    return 0;
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1)
    return 0;
  *a2 = *a2 + 1;
  v2 = (*dat_38670)(v1,v4);
  if (v2)
    sub_e920(*__errno_location(),v1); // no-return
  v3 = sub_d4d0(a0,v1);
  *(unsigned long *)(v3 + 0x48) = v8;
  *(long *)(v3 + 0x40) = v7;
  *(unsigned long *)(v3 + 0x38) = 2;
  v5 = (float)(int)((dat_38648 - v7) / 0x15180);
  v6 = dat_2bc40;
  if (((dat_2bc64 <= v5) && (v6 = dat_2bc74, dat_29578 <= v5)) && (v5 <= dat_2bc80))
    v6 = dat_2bc7c;
  *(unsigned int *)(v3 + 0x24) = v6;
  return 1;
}

// Function: sub_14600 @ 0x14600
unsigned int sub_14600(int *a0,long a1,int *a2,unsigned int a3)
{
  int v1;
  unsigned int v2; // eax
  long v3;
  unsigned int v4; // stack - 0x48
  float v5; // xmm0_da
  unsigned int v6;
  unsigned int v7; // stack - 0x44
  long v8; // stack - 0x40
  unsigned long v9; // stack - 0x38
  
  if (a1) { // branch-flip
    v1 = *a2;
    v2 = 0;
    v3 = *(long *)(a1 + (long)v1 * 8);
    if (v3) {
      *a2 = v1 + 1;
      v4 = a3;
      v2 = sub_13be0(v3,&v4,dat_38648 + 0x15180,dat_38650,"arithmetic overflow while converting %s minutes to a number of seconds");
      if ((char)v2) { // branch-flip
        v3 = sub_d4d0(a0,v3);
        *(unsigned long *)(v3 + 0x38) = CONCAT44(v7,v4);
        *(long *)(v3 + 0x40) = v8;
        *(unsigned long *)(v3 + 0x48) = v9;
        v5 = (float)(int)((dat_38648 - v8) / 0x15180);
        v6 = dat_2bc40;
        if (((dat_2bc64 <= v5) && (v6 = dat_2bc74, dat_29578 <= v5)) && (v5 <= dat_2bc80))
          v6 = dat_2bc7c;
        *(unsigned int *)(v3 + 0x24) = v6;
      }
      else {
        *a2 = v1;
      }
    }
  }
  else {
    v2 = 0;
  }
  return v2;
}

// Function: sub_14750 @ 0x14750
unsigned int sub_14750(int *a0,long a1,int *a2)
{
  return sub_14600(a0,a1,a2); // tail-call
}

// Function: sub_14760 @ 0x14760
unsigned int sub_14760(int *a0,long a1,int *a2)
{
  return sub_14600(a0,a1,a2); // tail-call
}

// Function: sub_14770 @ 0x14770
unsigned int sub_14770(int *a0,long a1,int *a2)
{
  return sub_14600(a0,a1,a2); // tail-call
}

// Function: sub_14780 @ 0x14780
unsigned int sub_14780(int *a0,long a1,int *a2)
{
  int v1;
  int v10; // stack - 0x54
  long v11; // stack - 0x50
  unsigned long v12; // stack - 0x48
  unsigned int v2; // eax
  char *v3; // rax
  long v4;
  unsigned int v5; // stack - 0x58
  unsigned long v6; // stack - 0x60
  char *v7;
  float v8; // xmm0_da
  unsigned int v9;
  
  v3 = dcgettext(NULL,"arithmetic overflow while converting %s days to a number of seconds",5);
  if (a1) { // branch-flip
    v1 = *a2;
    v2 = 0;
    v7 = *(char **)(a1 + (long)v1 * 8);
    if (v7) {
      *a2 = v1 + 1;
      v4 = dat_38648;
      if (*v7 == '-')
        v4 = dat_38648 + 0x1517f;
      v2 = sub_13be0(v7,&v5,v4,dat_38650,v3);
      if ((char)v2) { // branch-flip
        v4 = sub_d4d0(a0,v7);
        *(unsigned long *)(v4 + 0x38) = CONCAT44(v10,v5);
        *(long *)(v4 + 0x40) = v11;
        *(unsigned long *)(v4 + 0x48) = v12;
        v8 = (float)(int)((dat_38648 - v11) / 0x15180);
        v9 = dat_2bc40;
        if (((dat_2bc64 <= v8) && (v9 = dat_2bc74, dat_29578 <= v8)) && (v8 <= dat_2bc80))
          v9 = dat_2bc7c;
        *(unsigned int *)(v4 + 0x24) = v9;
        if (dat_38660 & 1) {
          __fprintf_chk(stderr,1,"inserting %s\n",*(char **)(v4 + 8));
          if (v10) { // branch-flip
            if (v10 != 1) { // branch-flip
              v7 = " ?";
              v3 = "?";
              if (v10 == 2) {
                __fprintf_chk(stderr,1,"    type: %s    %s  ","eq",">=");
                v6 = *(unsigned long *)(v4 + 0x40);
                v3 = ctime(&v6);
                __fprintf_chk(stderr,1,"%lu %s",*(unsigned long *)(v4 + 0x40),v3);
                v6 = *(long *)(v4 + 0x40) + 0x15180;
                __fprintf_chk(stderr,1,"                 <  %lu %s",v6,ctime(&v6));
                return v2;
              }
            }
            else {
              v7 = " <";
              v3 = "lt";
            }
          }
          else {
            v7 = " >";
            v3 = "gt";
          }
          __fprintf_chk(stderr,1,"    type: %s    %s  ",v3,v7);
          v6 = *(unsigned long *)(v4 + 0x40);
          v3 = ctime(&v6);
          __fprintf_chk(stderr,1,"%lu %s",*(unsigned long *)(v4 + 0x40),v3);
        }
      }
      else {
        *a2 = v1;
      }
    }
  }
  else {
    v2 = 0;
  }
  return v2;
}

// Function: sub_14a70 @ 0x14a70
void sub_14a70(long *a0)
{
  void *v1;
  unsigned int v2; // eax
  
  if (a0) {
    v2 = 0;
    do {
      while( true ) {
        v1 = (void *)*a0;
        if (v1 != sub_8d00) break;
        a0 = (long *)a0[0x21];
        v2 |= 1;
        if (!a0) goto label_14aba;
      }
      a0 = (long *)a0[0x21];
      if (v1 == sub_8600)
        v2 |= 2;
    } while (a0);
label_14aba:
    if ((v2 == 3) && (!dat_38621)) {
      error(1,0,dcgettext(NULL,"The -delete action automatically turns on -depth, but -prune does nothing when -depth is in effect.  If you want to carry on anyway, just explicitly use the -depth option.",5));
      return;
    }
  }
}

// Function: sub_14af0 @ 0x14af0
void sub_14af0(int a0) // return-dupe
{
  if (a0 != 1) { // branch-flip
    if (a0 != 2) { // branch-flip
      if (!a0) {
        dat_38670 = sub_de70;
        dat_38680 = sub_10c70;
      }
    }
    else {
      dat_3862c = 1;
      dat_38670 = sub_dcd0;
      dat_38680 = sub_12e00;
    }
  }
  else {
    dat_3862c = 1;
    dat_38670 = sub_ddb0;
    dat_38680 = sub_12dc0;
  }
  dat_38668 = a0;
  if (!(dat_38660 & 2))
    return;
  dat_38670 = sub_dec0;
}

// Function: sub_14ba0 @ 0x14ba0
void sub_14ba0(void)
{
  dat_38e70 = 0;
}

// Function: sub_14bb0 @ 0x14bb0
void sub_14bb0(void)
{
  return;
}

// Function: sub_14bc0 @ 0x14bc0
unsigned long sub_14bc0(char *a0)
{
  char v1;
  long *v2; // rbx
  int v3; // ebp
  int *v4;
  char *v5;
  
  if (!strncmp("-newer",a0,6)) {
    v4 = (int *)0x37840;
    if (strlen(a0) == 8) {
      return sub_12ca0(a0,v4); // return-dupe, tail-call
    }
  }
  v1 = *a0;
  v2 = (long *)0x36d88;
  v5 = "!";
  v3 = 0;
  while( true ) {
    if (!strcmp(v5,&a0[v1 == '-'])) break;
    v5 = (char *)*v2;
    v2 = &v2[4];
    v3 += 1;
    if (!v5)
      return 0;
  }
  v4 = (int *)((long)v3 * 0x20 + 0x36d60);
  return sub_12ca0(a0,v4);
}

// Function: sub_14c70 @ 0x14c70
unsigned long sub_14c70(int *a0,long a1,int *a2) // return-dupe x3
{
  char *v1;
  long v2; // rax
  char *v3; // rax
  stat v4;
  undefined4 v5;
  
  if (!a1)
    return 0;
  v1 = *(char **)(a1 + (long)*a2 * 8);
  if (!v1)
    return 0;
  *a2 = *a2 + 1;
  if ((2 <= dat_38688) && (!sub_10a50(v1))) {
    if (dat_38660 & 8)
      __fprintf_chk(stderr,1,"-fstype %s can never succeed, substituting -false\n",v1);
    v2 = sub_d520((int *)sub_14bc0("false"));
    *(unsigned int *)(v2 + 0x18) = 0;
    *(unsigned int *)(v2 + 0x24) = 0;
    return 1;
  }
  v2 = sub_d4d0(a0,v1);
  *(char **)(v2 + 0x38) = v1;
  v5 = dat_29578;
  if (!stat("/",&v4)) {
    v3 = (char *)sub_108e0(&v4,"/");
    v5 = s_2bc83._1_4_;
    if (strcmp(v1,v3))
      v5 = dat_2bc74;
  }
  *(unsigned int *)(v2 + 0x24) = v5;
  return 1;
}

// Function: sub_14df0 @ 0x14df0
unsigned long sub_14df0(void)
{
  long v1; // rax
  
  v1 = sub_d520((int *)sub_14bc0("false"));
  *(unsigned int *)(v1 + 0x18) = 0;
  *(unsigned int *)(v1 + 0x24) = 0;
  return 1;
}

// Function: sub_14e30 @ 0x14e30
unsigned long sub_14e30(void)
{
  return 0;
}

// Function: sub_14e40 @ 0x14e40
unsigned long sub_14e40(char *a0,unsigned long a1) // early-return
{
  unsigned char *v1; // rax
  char *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // stack - 0x28
  char v5 [8];
  
  v1 = (unsigned char *)getenv(a0);
  if (!v1)
    return 0;
  if (!sub_26060(v1,v5,10,&v4,NULL))
    return (unsigned long)(v4 < a1);
  v2 = dcgettext(NULL,"Environment variable %s is not set to a valid decimal number",5);
  error(1,*__errno_location(),v2,a0);
  return v3;
}

// Function: sub_14ef0 @ 0x14ef0
unsigned long sub_14ef0(long a0,void *a1,bool a2,unsigned long a3) // early-return
{
  unsigned long v1;
  unsigned long v2;
  
  v2 = *(unsigned long *)((long)a1 + 0x40);
  if (a2) { // branch-flip
    if (v2 < a3) {
      *(unsigned long *)((long)a1 + 0x40) = a3;
      v2 = a3;
    }
    if ((v2) && (v2 < *(unsigned long *)((long)a1 + 0x48))) {
      v2 = *(unsigned long *)((long)a1 + 0x48) - v2;
      if (v2 != 1) {
        v1 = a3 + (v2 >> 1);
        goto label_14f1a;
      }
    }
    else {
      v1 = a3;
      if (a3 == 0xffffffffffffffff) goto label_14f1a;
    }
    v1 = a3 + 1;
  }
  else {
    if (a3 <= (unsigned long)(*(long *)((long)a1 + 0x48) - 1U))
      *(unsigned long *)((long)a1 + 0x48) = a3;
    if ((v2) && (v2 < *(unsigned long *)((long)a1 + 0x48))) {
      v2 = *(unsigned long *)((long)a1 + 0x48) - v2;
      v1 = a3 - (v2 >> 1);
      if (v2 == 1)
        v1 = a3 - 1;
    }
    else {
      v1 = a3 >> 1;
    }
  }
label_14f1a:
  v2 = a0 + 1U;
  if (a0 + 1U <= v1)
    v2 = v1;
  if (a0)
    v1 = v2;
  if (v1)
    return v1;
  return 1;
}

// Function: sub_14fb0 @ 0x14fb0
void sub_14fb0(struct_7 *a0,struct_6 *a1,char *a2,long a3,char *a4,long a5,int a6) // return-dupe x2
{
  unsigned long v1;
  void *v2;
  unsigned long v3;
  unsigned long v4;
  
  if (!a2)
    __assert_fail("arg != NULL","buildcmd.c",0x155,"bc_push_arg"); // no-return
  if (a2 != "do_not_care") { // branch-flip
    v1 = a1->field_0x20;
    if (a0->field_0x18 < a3 + a5 + v1) {
      if ((a6) || (a1->field_0x0 == a0->field_0x38)) {
        error(1,0,dcgettext(NULL,"cannot fit single argument within argument list size limit",5));
        return;
      }
      if ((a0->field_0x30) || ((a0->field_0x0 && ((a0->field_0x48 || (a0->field_0x50)))))) {
        error(1,0,dcgettext(NULL,"argument list too long",5));
        return;
      }
      sub_15430(a0,a1);
      v3 = a1->field_0x0;
label_15059:
      if (((a0->field_0x50) && (a0->field_0x50 == v3 - a0->field_0x38)) || (v3 == a0->field_0x20)) {
label_15077:
        sub_15430(a0,a1);
        goto label_15082;
      }
      goto label_1508d;
    }
    v3 = a1->field_0x0;
    if (!a6) goto label_15059;
    if (v3 == a0->field_0x20) goto label_15077;
    v2 = a1->field_0x8;
    v4 = a1->field_0x10;
    if (v4 <= v3) goto label_15190;
label_150b5:
    a1->field_0x0 = v3 + 1;
    *(unsigned long *)((long)v2 + v3 * 8) = a1->field_0x18 + v1;
    if (a4) {
      strcpy((char *)(v1 + a1->field_0x18),a4);
      v1 = a5 + a1->field_0x20;
      a1->field_0x20 = v1;
    }
    strcpy((char *)(v1 + a1->field_0x18),a2);
    v1 = a3 + a1->field_0x20;
    a1->field_0x20 = v1;
    if (a6) { // branch-flip
      if (a1->field_0x0 != a0->field_0x20) goto label_15140;
    }
    else if (((!a0->field_0x50) || (a0->field_0x50 != a1->field_0x0 - a0->field_0x38)) && (a1->field_0x0 != a0->field_0x20))
      return;
    sub_15430(a0,a1);
  }
  else {
label_15082:
    v3 = a1->field_0x0;
    if (!a6) {
label_1508d:
      a1->field_0x38 = 1;
    }
    v2 = a1->field_0x8;
    v4 = a1->field_0x10;
    if (v4 <= v3) {
label_15190:
      if (!v2) {
        a1->field_0x10 = 0x40;
        v2 = (void *)sub_25a50(0x200);
        v3 = a1->field_0x0;
        a1->field_0x8 = v2;
        goto label_150a8;
      }
      a1->field_0x10 = v4 * 2;
      v2 = (void *)sub_25aa0(v2,v4 << 4);
      v3 = a1->field_0x0;
      a1->field_0x8 = v2;
      if (a2 == "do_not_care") goto label_151b9;
label_150b1:
      v1 = a1->field_0x20;
      goto label_150b5;
    }
label_150a8:
    if (a2 != "do_not_care") goto label_150b1;
label_151b9:
    a1->field_0x0 = v3 + 1;
    *(unsigned long *)((long)v2 + v3 * 8) = 0;
  }
  if (!a6)
    return;
  v1 = a1->field_0x20;
label_15140:
  a1->field_0x28 = v1;
  return;
}

// Function: sub_15270 @ 0x15270
void sub_15270(void *a0,struct_6 *a1,char *a2,unsigned long a3,char *a4,long a5,char *a6,long a7,unsigned int a8)
{
  char *v1;
  char *v2;
  long v3;
  unsigned int v4;
  char *v5; // rax
  unsigned long v6; // rbx
  unsigned long v7;
  char *v8;
  
  v4 = a8;
  v3 = a7;
  v2 = a6;
  v7 = *(long *)((long)a0 + 0x18) - 1;
  if (!dat_38e78)
    dat_38e78 = (char *)sub_25a50(*(long *)((long)a0 + 0x18) + 1);
  v1 = dat_38e78;
  v8 = dat_38e78;
  do {
    while( true ) {
      v5 = (char *)sub_1d5d0(a2,*(char **)((long)a0 + 0x30));
      if (v5) break;
      if (v7 <= a3) {
label_153e4:
        if (*a2) {
          error(1,0,dcgettext(NULL,"command too long",5));
          return;
        }
        goto label_153a0;
      }
      v5 = &a2[a3];
      strncpy(v8,a2,a3);
      v7 -= a3;
      v8 = &v8[a3];
      a3 = 0;
      a2 = v5;
      if (!*v5) goto label_153a0;
    }
    v6 = (long)v5 - (long)a2;
    if (v7 <= v6) goto label_153e4;
    strncpy(v8,a2,v6);
    v8 = &v8[v6];
    if (v7 - v6 <= (unsigned long)(v3 + a5)) {
      a2 = v5;
      goto label_153e4;
    }
    v7 = (v7 - v6) - (v3 + a5);
    if (a4) {
      strcpy(v8,a4);
      v8 = &v8[a5];
    }
    strcpy(v8,v2);
    a2 = &v5[*(long *)((long)a0 + 0x28)];
    v8 = &v8[v3];
    a3 = (a3 - *(long *)((long)a0 + 0x28)) - v6;
  } while (*a2);
label_153a0:
  *v8 = '\0';
  a6._0_4_ = v4;
  sub_14fb0(a0,a1,v1,&v8[1 - (long)v1],NULL,0); // tail-call
  return;
}

// Function: sub_15430 @ 0x15430
void sub_15430(void *a0,struct_5 *a1)
{
  unsigned long v1;
  long v10; // r12
  unsigned long v11; // r12
  unsigned long v2;
  int v3; // eax
  void *v4; // rax
  unsigned long v5;
  unsigned long v6;
  unsigned long v7;
  unsigned long v8;
  unsigned long v9;
  
  sub_14fb0(a0,a1,"do_not_care",0,0,0,0);
  v8 = a1->field_0x0;
  if (!v8)
    __assert_fail("state->cmd_argc > 0","buildcmd.c",0x105,"bc_do_exec"); // no-return
  if (*(long *)((a1->field_0x8 - 8) + v8 * 8))
    __assert_fail("state->cmd_argv[state->cmd_argc-1] == NULL","buildcmd.c",0x106,"bc_do_exec"); // no-return
  v11 = 0;
  v4 = (void *)sub_25a50(v8 * 8 + 8);
  v8 = a1->field_0x0;
  v7 = v8;
  do {
    v1 = *(unsigned long *)((long)a0 + 0x38);
    if (v1) { // branch-flip
      v6 = a1->field_0x8;
      v5 = 0;
      do {
        v5 += 1;
        *(unsigned long *)((long)v4 + v5 * 8 + -8) = *(unsigned long *)((v6 - 8) + v5 * 8);
      } while (v1 != v5);
      v6 = v11 + v1;
      v5 = v1;
    }
    else {
      v5 = 0;
      v6 = v11;
    }
    if ((v5 < v7) && (v6 < v8)) {
      v2 = a1->field_0x8;
      do {
        v6 += 1;
        v5 += 1;
        *(unsigned long *)((long)v4 + v5 * 8 + -8) = *(unsigned long *)((v2 - 8) + v6 * 8);
        if (v8 <= v6) break;
      } while (v5 < v7);
      if (v5 < v1)
        __assert_fail("dst_pos >= ctl->initial_argc","buildcmd.c",0xf2,"copy_args"); // no-return
    }
    v8 = a1->field_0x30;
    *(unsigned long *)((long)v4 + v5 * 8) = 0;
    v3 = (**(void **)((long)a0 + 0x40))(a0,v8,v5 & 0xffffffff,v4);
    v9 = *(long *)((long)a0 + 0x38);
    if (v3) { // branch-flip
      v10 = v11 - v9;
      v7 = sub_14ef0(v9,a1,1,v7);
      v11 = v10 + v5;
    }
    else {
      if (v7 <= v9 + 1U) {
        error(1,0,dcgettext(NULL,"can\'t call exec() due to argument size restrictions",5));
        return;
      }
      v7 = sub_14ef0(v9,a1,0,v7);
    }
    v8 = a1->field_0x0;
    if (v8 - v9 <= v11 + 1) {
      free(v4);
      a1->field_0x0 = *(unsigned long *)((long)a0 + 0x38);
      a1->field_0x20 = a1->field_0x28;
      a1->field_0x38 = 0xffffffff00000000;
      return;
    }
  } while( true );
}

// Function: sub_15660 @ 0x15660
long sub_15660(void) // early-return
{
  long v1; // rax
  
  v1 = sysconf(0);
  if (1 <= v1)
    return v1;
  return 0x7fffffffffffffff;
}

// Function: sub_15690 @ 0x15690
long sub_15690(void)
{
  unsigned long *v1;
  char *v2;
  unsigned long *v3;
  long v4;
  
  v4 = 0;
  v2 = (char *)*environ;
  v3 = environ;
  while (v2) {
    v1 = &v3[1];
    v3 = &v3[1];
    v4 = v4 + 1 + strlen(v2);
    v2 = (char *)*v1;
  }
  return v4;
}

// Function: sub_156e0 @ 0x156e0
unsigned long sub_156e0(struct_57 *a0,unsigned long a1)
{
  unsigned long v1; // rax
  unsigned long v2;
  unsigned long v3;
  long v4; // rdx
  long *v5;
  char *v6;
  
  v6 = (char *)*environ;
  if (v6) { // branch-flip
    v2 = 0;
    v5 = environ;
    do {
      v1 = strlen(v6);
      v6 = (char *)v5[1];
      v5 = &v5[1];
      v2 = v2 + 1 + v1;
    } while (v6);
    a0->field_0x10 = 0x1000;
    v1 = sysconf(0);
    a0->field_0x0 = 0;
    if ((long)v1 <= 0)
      v1 = 0x7fffffffffffffff;
    a0->field_0x8 = v1;
    if (v1 < v2)
      return 1;
  }
  else {
    a0->field_0x10 = 0x1000;
    v1 = sysconf(0);
    if (0 < (long)v1) {
      a0->field_0x8 = v1;
      v2 = 0;
      a0->field_0x0 = 0;
      v3 = a1;
      goto label_1575d;
    }
    v1 = 0x7fffffffffffffff;
    a0->field_0x0 = 0;
    v2 = 0;
    a0->field_0x8 = 0x7fffffffffffffff;
  }
  v3 = a1 + v2;
label_1575d:
  if (v1 <= v3)
    return 2;
  v2 = (v1 - a1) - v2;
  a0->field_0x8 = v2;
  v4 = (v2 >> 3) - 2;
  a0->field_0x20 = v4;
  if (v4) {
    a0->field_0x18 = v2;
    a0->field_0x28 = 0;
    a0->field_0x30 = 0;
    a0->field_0x38 = 0;
    a0->field_0x40 = sub_14e30;
    a0->field_0x48 = 0;
    a0->field_0x50 = 0;
    return 0;
  }
  __assert_fail("ctl->max_arg_count > 0","buildcmd.c",0x206,"bc_init_controlinfo"); // no-return
}

// Function: sub_15860 @ 0x15860
void sub_15860(struct_57 *a0)
{
  unsigned long v1;
  
  v1 = a0->field_0x8;
  if ((0x20000 <= v1) && (v1 = a0->field_0x10, v1 <= 0x1ffff))
    v1 = 0x20000;
  a0->field_0x18 = v1;
}

// Function: sub_15890 @ 0x15890
void sub_15890(struct_57 *a0,unsigned long *a1,unsigned long a2)
{
  unsigned long v1;
  unsigned long v2; // rax
  
  v1 = a0->field_0x18;
  *a1 = 0;
  a1[4] = 0;
  a1[1] = 0;
  a1[2] = 0;
  a1[8] = 0;
  a1[9] = 0;
  if (v1 <= 0x7ffffffffffff7ff) {
    v2 = sub_25a50(v1 + 1);
    a1[5] = 0;
    a1[3] = v2;
    a1[4] = 0;
    a1[7] = 0xffffffff00000000;
    a1[6] = a2;
    return;
  }
  __assert_fail("ctl->arg_max <= (LONG_MAX - 2048L)","buildcmd.c",0x23c,"bc_init_state"); // no-return
}

// Function: sub_15940 @ 0x15940
void sub_15940(void *a0,struct_62 *a1)
{
  a1->field_0x0 = *(unsigned long *)((long)a0 + 0x38);
  a1->field_0x20 = a1->field_0x28;
  a1->field_0x38 = 0xffffffff00000000;
}

// Function: sub_15970 @ 0x15970
unsigned long sub_15970(char **a0)
{
  char *v1;
  unsigned long v2; // rax
  unsigned long v3;
  unsigned long v4;
  
  v4 = 0;
  v3 = 0;
  v1 = *a0;
  while (v1) {
    v3 += 1;
    v4 += strlen(v1);
    v1 = a0[v3];
  }
  if (sub_14e40("__GNU_FINDUTILS_EXEC_ARG_COUNT_LIMIT",v3))
    return 1;
  v2 = sub_14e40("__GNU_FINDUTILS_EXEC_ARG_LENGTH_LIMIT",v4);
  return CONCAT71((undefined7)((unsigned long)v2 >> 8),(int)v2 != 0);
}

// Function: sub_159f0 @ 0x159f0
int sub_159f0(struct_9 *a0,void *a1,unsigned long a2)
{
  int v1;
  int v2; // eax
  int *v3; // rax
  char v4 [24];
  
  if (sub_24230(v4))
    sub_1f000(*__errno_location()); // no-return
  if (!sub_24290(a0)) {
    v2 = (*a1)(a2);
    if (0 <= v2) { // branch-flip
      if (!sub_24290(v4)) {
        sub_242c0(v4);
        return v2;
      }
    }
    else {
      v3 = __errno_location();
      v1 = *v3;
      if (!sub_24290(v4)) {
        sub_242c0(v4);
        if (!v1)
          return v2;
        *v3 = v1;
        return v2;
      }
    }
  }
  sub_1f040(*__errno_location()); // no-return
}

// Function: sub_15ac0 @ 0x15ac0
unsigned long sub_15ac0(int a0,void *a1,unsigned long a2)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  char v4 [24];
  unsigned long v5; // r12
  
  if (a0 == -100)
    return (*a1)(a2); // jump-as-call
  if (sub_24230(v4))
    sub_1f000(*__errno_location()); // no-return
  if (fchdir(a0)) { // branch-flip
    v3 = __errno_location();
    v5 = 0xffffffff;
    v1 = *v3;
    sub_242c0(v4);
    *v3 = v1;
  }
  else {
    v2 = (*a1)(a2);
    v5 = (unsigned long)v2;
    if (0 <= (int)v2) { // branch-flip
      if (sub_24290(v4))
        sub_1f040(*__errno_location()); // return-dupe, no-return
      sub_242c0(v4);
    }
    else {
      v3 = __errno_location();
      v1 = *v3;
      if (sub_24290(v4)) {
        sub_1f040(*__errno_location());
      }
      sub_242c0(v4);
      if (v1)
        *v3 = v1;
    }
  }
  return v5;
}

// Function: sub_15be0 @ 0x15be0
void * sub_15be0(void *a0,unsigned long a1,unsigned long *a2)
{
  int v1;
  unsigned long v2;
  unsigned long v3;
  int *v4; // rax
  unsigned long v5;
  
  v4 = __errno_location();
  v1 = *v4;
  if (!a1)
    __assert_fail("wanted > 0u","extendbuf.c",0x47,"extendbuf"); // no-return
  v2 = *a2;
  v5 = 0x10;
  if (v2)
    v5 = v2;
  for (; (v3 = v5, v5 < a1 && (v3 = a1, v5 <= v5 * 2)); v5 = v5 * 2) {
  }
  if (v2) { // branch-flip
    if (v2 != v3) {
      *a2 = v3;
      a0 = realloc(a0,v3);
    }
  }
  else {
    if (a0)
      __assert_fail("NULL == existing","extendbuf.c",0x4f,"extendbuf"); // no-return
    *a2 = v3;
    a0 = malloc(v3);
  }
  if (!a0)
    return NULL;
  *v4 = v1;
  return a0;
}

// Function: sub_15cd0 @ 0x15cd0
void sub_15cd0(void *a0,unsigned long a1,unsigned long *a2)
{
  if (sub_15be0(a0,a1,a2))
    return;
  free(a0);
  sub_25f50(); // no-return
}

// Function: sub_15d00 @ 0x15d00
int sub_15d00(void) // return-dupe x2
{
  bool v1;
  int v2; // eax
  DIR *v3; // rax
  int *v4; // rax
  dirent *v5; // rax
  char *v6; // rax
  int v7;
  
  v3 = (DIR *)sub_17af0("/proc/self/fd");
  if (!v3)
    return -1;
  v1 = 0;
  v4 = __errno_location();
  v7 = -1;
  while( true ) {
    *v4 = 0;
    v5 = readdir(v3);
    if (!v5) break;
    if ((v5->field_0x13 != '.') || ((v5->field_0x14 && (*(short *)&v5->field_0x14 != 0x2e)))) {
      v1 = 1;
      v2 = sub_17410(&v5->field_0x13,0);
      if (v7 < v2)
        v7 = v2;
    }
  }
  if (!*v4) {
    closedir(v3);
    if (!v1)
      return -1;
    return v7;
  }
  v6 = (char *)sub_239f0(0,8,"/proc/self/fd");
  error(0,*v4,"%s",v6);
  closedir(v3);
  return -1;
}

// Function: sub_15de0 @ 0x15de0
unsigned long sub_15de0(unsigned long a0,struct_51 *a1) // return-dupe
{
  long v1;
  void *v2; // rax
  
  if (sub_17dd0(a0,1) & 1)
    return 0;
  v2 = (void *)sub_15be0(a1->field_0x0,a1->field_0x8 * 4 + 4,&a1[1]);
  if (!v2)
    return 0xffffffff;
  v1 = a1->field_0x8;
  a1->field_0x0 = v2;
  *(int *)((long)v2 + v1 * 4) = (int)a0;
  a1->field_0x8 = v1 + 1;
  return 0;
}

// Function: sub_15e60 @ 0x15e60
unsigned long sub_15e60(int a0,struct_38 *a1)
{
  int v1;
  unsigned long v2;
  
  if (sub_17dd0(a0,1) & 1)
    return 0;
  v2 = a1->field_0x10;
  if (v2 < a1->field_0x8) {
    do {
      v1 = *(int *)(a1->field_0x0 + v2 * 4);
      if (a0 <= v1) {
        if (a0 == v1)
          return 0;
        break;
      }
      v2 += 1;
      a1->field_0x10 = v2;
    } while (v2 != a1->field_0x8);
  }
  a1->field_0x18 = a0;
  return 0xffffffff;
}

// Function: sub_15ed0 @ 0x15ed0
void sub_15ed0(int a0,void *a1,unsigned long a2) // return-dupe
{
  int v1;
  int *v2;
  int *v3; // rax
  int v4;
  int v5; // ebx
  long v6;
  int v7 [128];
  
  if (a0 <= 0)
    return;
  v4 = 0;
  while( true ) {
    v5 = a0 - v4;
    if (0x41 <= v5)
      v5 = 0x40;
    v1 = v4 + v5;
    v2 = &v6;
    do {
      *v2 = v4;
      v4 += 1;
      *(unsigned short *)&v2[1] = 5;
      v3 = &v2[2];
      *(unsigned short *)((long)v2 + 6) = 0;
      v2 = v3;
    } while (v4 != v1);
    if (poll(&v6,(long)v5,0) == -1) break;
    v2 = &v6;
    do {
      if ((*(short *)((long)v2 + 6) != 0x20) && (v4 = *v2, (*a1)(v4,a2)))
        return;
      v2 = &v2[2];
    } while (v2 != &v7[(unsigned long)(unsigned int)(v5 - 1) * 2]);
    v4 = v1;
    if (a0 <= v1)
      return;
  }
}

// Function: sub_15fd0 @ 0x15fd0
void sub_15fd0(void)
{
  int v1; // eax
  long v2; // rax
  long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  
  v1 = sub_15d00();
  if (v1 < 0) {
    v2 = sysconf(4);
    if (v2 == -1)
      v2 = 0x14;
    v1 = (int)v2;
    if (!getrlimit(7,&v3)) {
      if (v3 == -1)
        v3 = v2;
      v1 = (int)v3;
    }
  }
  v3 = 0;
  v5 = 0;
  v4 = 0;
  sub_15ed0(v1 + (unsigned int)(v1 != 0x7fffffff),sub_15de0,&v3);
  dat_38e90 = v3;
  dat_38e88 = v4;
}

// Function: sub_160b0 @ 0x160b0
int sub_160b0(char *a0,unsigned int a1,unsigned long a2) // ternary
{
  unsigned char v1; // al
  int v2; // eax
  unsigned long v3; // r13
  
  v3 = (a1 & 0x40) ? a2 & 0xffffffff : 0; // branch-flip
  v1 = dat_38e81;
  if (!dat_38e81) {
    v2 = sub_18040("/",0x80000,a2);
    if (0 <= v2) { // branch-flip
      v1 = sub_17dd0(v2,1);
      close(v2);
      dat_38e80 = v1 & 1;
      dat_38e81 = 1;
    }
    else {
      dat_38e81 = 1;
      dat_38e80 = v1;
    }
  }
  v2 = sub_18040(a0,a1 | 0x80000,v3);
  if ((0 <= v2) && (!dat_38e80))
    sub_17990(v2,1);
  return v2;
}

// Function: sub_161f0 @ 0x161f0
void sub_161f0(void)
{
  free(dat_38e90);
  dat_38e90 = NULL;
  dat_38e88 = 0;
}

// Function: sub_16220 @ 0x16220
unsigned long sub_16220(void)
{
  char *v1; // rax
  
  v1 = getenv("GNU_FINDUTILS_FD_LEAK_CHECK");
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),v1 != NULL);
}

// Function: sub_16240 @ 0x16240
void sub_16240(void)
{
  unsigned long v1;
  long v2;
  int v3;
  long v4; // rax
  long v5; // stack - 0x58
  unsigned long v6; // stack - 0x50
  unsigned long v7; // stack - 0x48
  int v8; // stack - 0x40
  
  v2 = dat_38e90;
  v1 = dat_38e88;
  v3 = sub_15d00();
  if (v3 < 0) {
    v4 = sysconf(4);
    if (v4 == -1)
      v4 = 0x14;
    v3 = (int)v4;
    if (!getrlimit(7,&v5)) {
      if (v5 == -1)
        v5 = v4;
      v3 = (int)v5;
    }
  }
  v5 = v2;
  v6 = v1;
  v8 = -1;
  v7 = 0;
  sub_15ed0(v3 + (unsigned int)(v3 != 0x7fffffff),sub_15e60,&v5);
  v3 = v8;
  if (0 <= v8) {
    error(0,0,dcgettext(NULL,"File descriptor %d will leak; please report this as a bug, remembering to include a detailed description of the simplest way to reproduce this problem.",5),v3);
    __assert_fail("no_leaks","fdleak.c",0x18c,"complain_about_leaky_fds"); // no-return
  }
}

// Function: sub_16370 @ 0x16370
undefined16 sub_16370(char *a0)
{
  char v1 [16];
  char *v2; // rax
  char *v3; // rax
  
  sub_180c0(stderr);
  v2 = dcgettext(NULL,"Kevin Dalley",5);
  v3 = dcgettext(NULL,"James Youngman",5);
  sub_25880(stdout,a0,"GNU findutils",dat_38490,dcgettext(NULL,"Eric B. Decker",5),v3);
  v1._8_8_ = 0;
  v1._0_8_ = v2;
  return v1._0_16_;
}

// Function: sub_16400 @ 0x16400
unsigned long sub_16400(unsigned char *a0,FILE *a1) // return-dupe
{
  unsigned char v1;
  int v2; // eax
  
  v1 = *a0;
  if (!v1)
    return 1;
  do {
    a0 = &a0[1];
    if ('#' <= (char)v1) { // branch-flip
      if (v1 == 0x5c) {
        v2 = __fprintf_chk(a1,1,"\\\\");
        goto label_164c8;
      }
label_16468:
      if (0x5e <= (unsigned char)(v1 - 0x21)) {
        v2 = __fprintf_chk(a1,1,"\\%03o");
        goto label_164c8;
      }
      if (putc((unsigned int)v1,a1) == -1)
        return 0;
    }
    else {
      if ((char)v1 <= '\a') goto label_16468;
      switch(v1) {
        case 8:
          v2 = __fprintf_chk(a1,1,"\\b");
          break;
        case 9:
          v2 = __fprintf_chk(a1,1,"\\t");
          break;
        case 10:
          v2 = __fprintf_chk(a1,1,"\\n");
          break;
        default:
          goto label_16468;
        case 0xc:
          v2 = __fprintf_chk(a1,1,"\\f");
          break;
        case 0xd:
          v2 = __fprintf_chk(a1,1,"\\r");
          break;
        case 0x20:
          v2 = __fprintf_chk(a1,1,"\\ ");
          break;
        case 0x22:
          v2 = __fprintf_chk(a1,1,"\\\"");
        
      }
label_164c8:
      if (v2 <= -1)
        return 0;
    }
    v1 = *a0;
    if (!v1)
      return 1;
  } while( true );
}

// Function: sub_165e0 @ 0x165e0
void sub_165e0(unsigned char *a0,unsigned int a1,unsigned long a2,void *a3,long a4,unsigned int a5,int a6,FILE *a7)
{
  long v1;
  unsigned long v10; // rax
  int *v11; // rax
  unsigned char *v12; // rax
  char *v13; // rcx
  char *v14;
  char v15 [12];
  char v16 [664];
  char v17 [8];
  char v18 [256];
  char *v19; // rsp
  FILE *v2;
  char *v20; // rsp
  char *v21;
  char *v22;
  unsigned int v23;
  unsigned long v24;
  unsigned long v25; // stack - 0x420
  char *v26; // stack - 0x418
  unsigned long v27; // stack - 0x410
  unsigned int v28; // stack - 0x408
  unsigned int v29; // stack - 0x404
  char v3; // al
  long v30; // stack - 0x400
  unsigned char *v31; // stack - 0x3f8
  void *v32; // stack - 0x3f0
  int v4; // eax
  int v5;
  int v6; // eax
  unsigned long v7; // rax
  char *v8; // rax
  tm *v9; // rax
  
  v2 = a7;
  v19 = v17;
  v22 = v17;
  v21 = v17;
  v32 = (void *)CONCAT44(v32._4_4_,a5);
  v27 = a2;
  v29 = a1;
  v30 = a4;
  v31 = a0;
  sub_18110(*(unsigned int *)((long)a3 + 0x18),v15);
  v7 = sub_1bb50(*(unsigned long *)((long)a3 + 8),v16,0,1,1);
  v4 = __fprintf_chk(v2,1,"%*s",(unsigned long)dat_384b4,v7);
  if (v4 < 0) {
    v23 = 100;
    goto label_169b6;
  }
  if ((int)dat_384b4 < v4)
    dat_384b4 = v4;
  v5 = putc(0x20,v2);
  v28 = 0;
  if (v5 == -1)
    v28 = 0x96;
  v25 = (unsigned long)(int)v32;
  v26 = (char *)CONCAT71(v26._1_7_,v5 != -1);
  v7 = sub_1bb50(*(unsigned long *)((long)a3 + 0x40),v16,0,0x200,v25);
  v4 = __fprintf_chk(v2,1,"%*s",(unsigned long)dat_384b0,v7);
  if (v4 < 0) {
    v23 = 200;
    v21 = v17;
    goto label_169b6;
  }
  if ((int)dat_384b0 < v4)
    dat_384b0 = v4;
  v21 = v17;
  v23 = v28;
  if (!(char)v26) goto label_169b6;
  if (putc(0x20,v2) == -1) {
    v23 = 0xfa;
    v21 = v17;
    goto label_169b6;
  }
  if (fputs(v15,v2) < 0) {
    v23 = 0x113;
    v21 = v17;
    goto label_169b6;
  }
  v5 = __fprintf_chk(v2,1,"%*lu",dat_384ac,*(unsigned long *)((long)a3 + 0x10));
  if (v5 < 0) {
    v23 = 300;
    v21 = v17;
    goto label_169b6;
  }
  if (dat_384ac < v5)
    dat_384ac = v5;
  putc(0x20,v2);
  v8 = (char *)sub_1c9f0(*(unsigned int *)((long)a3 + 0x1c));
  if (v8) { // branch-flip
    v26 = v8;
    v6 = sub_1dec0(v8,0);
    v5 = dat_384a8;
    if (dat_384a8 < v6) {
      v5 = v6;
      dat_384a8 = v6;
    }
    v5 = __fprintf_chk(v2,1,"%-*s ",v5,v26);
    if (v5 < 0) {
      v23 = 400;
      v21 = v17;
      goto label_169b6;
    }
  }
  else {
    v5 = __fprintf_chk(v2,1,"%-8lu ",(unsigned long)*(unsigned int *)((long)a3 + 0x1c));
    if (dat_384a8 < v5)
      dat_384a8 = v5;
    if (v5 <= 0) {
      v23 = 0x1c2;
      v21 = v17;
      goto label_169b6;
    }
  }
  v8 = (char *)sub_1cb90(*(unsigned int *)((long)a3 + 0x20));
  if (v8) { // branch-flip
    v26 = v8;
    v6 = sub_1dec0(v8,0);
    v5 = dat_384a4;
    if (dat_384a4 < v6) {
      v5 = v6;
      dat_384a4 = v6;
    }
    v5 = __fprintf_chk(v2,1,"%-*s ",v5,v26);
    if (v5 < 0) {
      v23 = 500;
      v21 = v17;
      goto label_169b6;
    }
  }
  else {
    v5 = __fprintf_chk(v2,1,"%-*lu",dat_384a4,(unsigned long)*(unsigned int *)((long)a3 + 0x20));
    if (dat_384a4 < v5)
      dat_384a4 = v5;
    v23 = 0x226;
    v21 = v17;
    if (v5 < 0) goto label_169b6;
    if (putc(0x20,v2) == -1) {
      v23 = 0x20d;
      v21 = v17;
      goto label_169b6;
    }
  }
  if ((*(unsigned int *)((long)a3 + 0x18) & 0xb000) != 0x2000) { // branch-flip
    v24 = 1;
    if ((int)v32 <= -1)
      v24 = v25;
    v7 = sub_1bb50(*(unsigned long *)((long)a3 + 0x30),v16,0,1,v24);
    v4 = __fprintf_chk(v2,1,"%*s",(unsigned long)dat_38498,v7);
    if (v4 < 0) {
      v23 = 800;
      v21 = v17;
      goto label_169b6;
    }
    if ((int)dat_38498 < v4)
      dat_38498 = v4;
  }
  else {
    v4 = __fprintf_chk(v2,1,"%*lu",(unsigned long)dat_384a0,(unsigned long)((unsigned int)((unsigned long)*(unsigned long *)((long)a3 + 0x28) >> 8) & 0xfff | (unsigned int)((unsigned long)*(unsigned long *)((long)a3 + 0x28) >> 0x20) & 0xfffff000));
    v23 = 600;
    v21 = v17;
    if (v4 < 0) goto label_169b6;
    if ((int)dat_384a0 < v4)
      dat_384a0 = v4;
    if (__fprintf_chk(v2,1,", ") < 0) {
      v23 = 0x271;
      v21 = v17;
      goto label_169b6;
    }
    v4 = __fprintf_chk(v2,1,"%*lu",(unsigned long)dat_3849c,(unsigned long)((unsigned int)*(unsigned long *)((long)a3 + 0x28) & 0xff | (unsigned int)((*(unsigned long *)((long)a3 + 0x28) >> 0x14) << 8)));
    v23 = 0x28a;
    v21 = v17;
    if (v4 < 0) goto label_169b6;
    if ((int)dat_3849c < v4)
      dat_3849c = v4;
  }
  if (putc(0x20,v2) == -1) {
    v23 = 0x352;
    v21 = v17;
    goto label_169b6;
  }
  v9 = localtime((void *)((long)a3 + 0x58));
  if (v9) { // branch-flip
    v8 = "%b %e  %Y";
    if ((v30 + -0xed4e00 <= *(long *)((long)a3 + 0x58)) && (v8 = "%b %e %H:%M", v30 + 0xe10 < *(long *)((long)a3 + 0x58)))
      v8 = "%b %e  %Y";
    v14 = v18;
    v24 = 0x100;
    v32 = a3;
    while( true ) {
      *(unsigned long *)&v19[-8] = 0x1694b;
      v10 = strftime(v14,v24,v8,v9);
      a3 = v32;
      if (v10) break;
      v24 *= 2;
      v13 = &v19[-(v24 + 0x17 & 0xfffffffffffff000)];
      v21 = v19;
      v22 = v19;
      while (v19 != v13) {
        v21 = &v22[-0x1000];
        v20 = &v22[-0x1000];
        *(unsigned long *)&v22[-8] = *(unsigned long *)&v22[-8];
        v19 = &v22[-0x1000];
        v22 = v20;
      }
      v10 = (unsigned long)((unsigned int)(v24 + 0x17) & 0xff0);
      v1 = -v10;
      v19 = &v21[v1];
      if (v10)
        *(unsigned long *)&v21[-8] = *(unsigned long *)&v21[-8];
      v14 = (char *)((unsigned long)&v21[v1 + 0xf] & 0xfffffffffffffff0);
    }
    *(unsigned long *)&v19[-8] = 0x16a63;
    v23 = 900;
    v21 = v19;
    v22 = v19;
    if (__fprintf_chk(v2,1,"%s ",v14) < 0) goto label_169b6;
  }
  else {
    v24 = *(unsigned long *)((long)a3 + 0x58);
    if (0 <= (long)v24) { // branch-flip
      v8 = (char *)sub_1bb50(v24,v16,0,1,1);
      v22 = v17;
      if (__fprintf_chk(v2,1,"%*s ",0xc,v8) <= -1) {
        v23 = 0x44c;
        v21 = v17;
        goto label_169b6;
      }
    }
    else {
      v8 = (char *)sub_1bb50(-v24,v16,0,1,1);
      v5 = 0xc - (int)strlen(v8);
      if (v5 <= -1)
        v5 = 0;
      if (__fprintf_chk(v2,1,"%*s%s ",v5,"-",v8) <= -1) {
        v23 = 1000;
        v21 = v17;
        goto label_169b6;
      }
    }
  }
  v21 = v22;
  if (a6) { // branch-flip
    *(unsigned long *)&v22[-8] = 0x16a95;
    if (0 <= __fprintf_chk(v2,1,"%s",(char *)v31)) {
label_16abb:
      if ((*(unsigned int *)((long)a3 + 0x18) & 0xf000) == 0xa000) {
        *(unsigned long *)&v22[-8] = 0x16bf2;
        v12 = (unsigned char *)sub_17650(v29,v27);
        if (v12) {
          *(unsigned long *)&v22[-8] = 0x16c0d;
          if (fputs(" -> ",v2) <= -1) {
            *(unsigned long *)&v22[-8] = 0x16c1d;
            free(v12);
            v23 = 0x514;
            goto label_169b6;
          }
          if (a6) { // branch-flip
            *(unsigned long *)&v22[-8] = 0x16d8c;
            if (0 <= __fprintf_chk(v2,1,"%s",(char *)v12)) {
label_16e65:
              *(unsigned long *)&v22[-8] = 0x16e6d;
              free(v12);
              goto label_16ace;
            }
          }
          else {
            *(unsigned long *)&v22[-8] = 0x16e5d;
            if (sub_16400(v12,v2)) goto label_16e65;
          }
          *(unsigned long *)&v22[-8] = 0x16d9c;
          free(v12);
          v23 = 0x546;
          goto label_169b6;
        }
        *(unsigned long *)&v22[-8] = 0x16e2d;
        v5 = *__errno_location();
        *(unsigned long *)&v22[-8] = 0x16e46;
        error(0,v5,"%s",(char *)v31);
        *(unsigned long *)&v22[-8] = 0x16e4d;
        free(NULL);
      }
label_16ace:
      *(unsigned long *)&v22[-8] = 0x16adb;
      if (putc(10,v2) != -1)
        return;
      v23 = 0x578;
      goto label_169b6;
    }
  }
  else {
    *(unsigned long *)&v22[-8] = 0x16ab7;
    v3 = sub_16400(v31,v2);
    if (v3) goto label_16abb;
  }
  v23 = 0x4b0;
label_169b6:
  v32 = (void *)CONCAT44(v32._4_4_,v23);
  *(unsigned long *)&v21[-8] = 0x169d0;
  v8 = dcgettext(NULL,"Failed to write output (at stage %d)",5);
  *(unsigned long *)&v21[-8] = 0x169d8;
  v11 = __errno_location();
  v5 = *v11;
  *(unsigned long *)&v21[-8] = 0x169ef;
  error(1,v5,v8,(unsigned long)v32 & 0xffffffff);
  return;
}

// Function: sub_16e80 @ 0x16e80
unsigned long sub_16e80(FILE *a0,unsigned int *a1,bool a2,char *a3,char *a4)
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  char *v3; // rax
  char v4 [8200];
  
  if (a2) {
    v2 = sub_23760(v4,0x2000,a4,0xffffffffffffffff,a1);
    if (0x2000 <= v2) { // branch-flip
      v3 = (char *)sub_25a50(v2 + 1);
      sub_23760(v3,v2 + 1,a4,0xffffffffffffffff,a1);
      v3[sub_17010(v3,v2)] = '\0';
      v1 = __fprintf_chk(a0,1,a3,v3);
      free(v3);
    }
    else {
      v4[sub_17010(v4,v2)] = 0;
      v1 = __fprintf_chk(a0,1,a3,v4);
    }
    return (unsigned long)v1;
  }
  return __fprintf_chk(a0,1,a3,a4); // tail-call
}

// Function: sub_17010 @ 0x17010
long * sub_17010(char *a0,long a1) // early-return
{
  char v1;
  char *v10;
  char *v11;
  char *v12; // stack - 0x68
  int v2; // eax
  long *v3; // rax
  long v4;
  long v5; // rdx
  char *v6; // rbx
  mbstate_t v7; // stack - 0x48
  int v8; // stack - 0x4c
  char *v9;
  
  v6 = &a0[a1];
  v3 = (unsigned long)__ctype_get_mb_cur_max();
  if (2 <= v3) { // branch-flip
    if (v6 <= a0) {
      v3 = NULL;
      return v3;
    }
    v9 = a0;
    v11 = a0;
    do {
      v1 = *v11;
      v10 = &v9[1];
      if ('`' <= v1) { // branch-flip
        if (0x1e <= (unsigned char)(v1 + 0x9fU)) goto label_1709a;
label_1714c:
        *v9 = v1;
        v11 = &v11[1];
      }
      else {
        if ('@' < v1) goto label_1714c;
        if ('$' <= v1) { // branch-flip
          if ((unsigned char)(v1 - 0x25U) <= 0x1a) goto label_1714c;
        }
        else if ('\x1f' < v1) goto label_1714c;
label_1709a:
        v7 = 0;
        while (v3 = (long)sub_1cd30(&v8,v11,(long)v6 - (long)v11,&v7), v3 != (long *)0xffffffffffffffff) {
          if (v3 == (long *)0xfffffffffffffffe) {
            v11 = v6;
            goto label_171a4;
          }
          v4 = 1;
          if (v3)
            v4 = (long)v3;
          v2 = wcwidth(v8);
          if (0 <= v2) { // branch-flip
            v5 = 0;
            do {
              v9[v5] = v11[v5];
              v5 += 1;
            } while (v4 != v5);
            v10 = &v9[v4];
          }
          else {
            *v9 = '?';
          }
          v11 = &v11[v4];
          v2 = mbsinit(&v7);
          if (v2) goto label_17157;
          v9 = v10;
          v10 = &v10[1];
        }
        v11 = &v11[1];
label_171a4:
        *v9 = '?';
      }
label_17157:
      v9 = v10;
    } while (v11 < v6);
    v3 = (long)((long)v10 - (long)a0);
  }
  else {
    if (a0 < v6) {
      v3 = __ctype_b_loc();
      v12 = a0;
      do {
        if (!(*(unsigned char *)(*v3 + 1 + (unsigned long)(unsigned char)*v12 * 2) & 0x40))
          *v12 = '?';
        v12 = &v12[1];
      } while (v12 != v6);
    }
    v3 = (long *)a1;
  }
  return v3;
}

// Function: sub_17200 @ 0x17200
unsigned long sub_17200(char *a0)
{
  char *v1;
  char *v10;
  int v2;
  char *v3; // rax
  unsigned long v4; // rax
  unsigned long v5; // rax
  unsigned int v6; // ebx
  unsigned long v7; // rbx
  unsigned long *v8; // r12
  long v9;
  
  v10 = "findutils-default";
  v9 = 0;
  v8 = (unsigned long *)0x36310;
  v7 = 0;
  while( true ) {
    if (!strcmp(v10,a0))
      return (unsigned long)*(unsigned int *)(v7 * 0x10 + 0x3630c);
    v6 = (int)v7 + 1;
    v7 = (unsigned long)v6;
    v10 = (char *)sub_24210(v10);
    v9 = v9 + 2 + strlen(v10);
    if (v6 == 0xd) break;
    v10 = (char *)*v8;
    v8 = &v8[2];
  }
  v8 = (unsigned long *)0x36300;
  v3 = (char *)sub_25a50(v9 + 1);
  v10 = v3;
  v2 = 0;
  do {
    if (v2) { // branch-flip
      v10[2] = '\0';
      v1 = (char *)*v8;
      v10[0] = ',';
      v10[1] = ' ';
      v4 = sub_24210(v1);
      v10 = &(&v10[2])[sprintf(&v10[2],"%s",v4)];
      if (v2 + 1 == 0xd) {
        v10 = (char *)sub_24210(a0);
        error(1,0,dcgettext(NULL,"Unknown regular expression type %s; valid types are %s.",5),v10,v3);
        return v5;
      }
    }
    else {
      v4 = sub_24210((char *)*v8);
      v10 = &v10[sprintf(v10,"%s",v4)];
    }
    v8 = &v8[2];
    v2 += 1;
  } while( true );
}

// Function: sub_17350 @ 0x17350
unsigned long sub_17350(unsigned int a0) // early-return
{
  if (0xd <= a0)
    return 0;
  return *(unsigned long *)((unsigned long)a0 * 0x10 + 0x36300);
}

// Function: sub_17370 @ 0x17370
unsigned int sub_17370(unsigned int a0)
{
  if (a0 <= 0xc)
    return *(unsigned int *)((unsigned long)a0 * 0x10 + 0x3630c);
  return 0xffffffff;
}

// Function: sub_173a0 @ 0x173a0
unsigned int sub_173a0(unsigned int a0) // early-return
{
  if (0xd <= a0)
    return 0;
  return *(unsigned int *)((unsigned long)a0 * 0x10 + 0x36308);
}

// Function: sub_173c0 @ 0x173c0
unsigned int sub_173c0(unsigned int a0,unsigned int a1)
{
  unsigned int v1; // eax
  unsigned int v2;
  int *v3;
  int *v4; // rdx
  
  if ((a0 <= 0xc) && (a0)) {
    v2 = 1;
    v1 = 0;
    v3 = (int *)0x3630c;
    while( true ) {
      if ((v2 & a1) && (*v3 == *(int *)((unsigned long)a0 * 0x10 + 0x3630c)))
        return v1;
      v1 += 1;
      v4 = &v3[4];
      if (a0 == v1) break;
      v2 = v3[3];
      v3 = v4;
    }
  }
  return 0xffffffff;
}

// Function: sub_17410 @ 0x17410
void sub_17410(char *a0,unsigned int a1)
{
  int *v1; // rax
  unsigned long v2;
  long v3; // rax
  char *v4; // rax
  char *v5; // rax
  char *v6; // rax
  char *v7; // stack - 0x38
  
  v1 = __errno_location();
  *v1 = 0;
  v2 = strtol(a0,&v7,10);
  if (0xfffffffffffffffe <= v2 + 0x7fffffffffffffffU) {
    if (*v1 == 0x22) {
      error(1,0x22,"%s",a0);
      goto label_174c5;
    }
    error(1,*v1,"%s",a0);
    v2 = v3;
  }
  if (!(v2 + 0x80000000U >> 0x20)) {
    if (*v7) {
      v4 = (char *)sub_239f0(1,a1,a0);
      v5 = (char *)sub_239f0(0,a1,v7);
      v6 = dcgettext(NULL,"Unexpected suffix %s on %s",5);
      error(1,*v1,v6,v5,v4);
      return;
    }
    if (v7 == a0) {
      v4 = (char *)sub_239f0(0,a1,v7);
      v5 = dcgettext(NULL,"Expected an integer: %s",5);
      error(1,*v1,v5,v4);
      return;
    }
    return;
  }
label_174c5:
  *v1 = 0x22;
  error(1,0x22,"%s",a0);
  return;
}

// Function: sub_17570 @ 0x17570
unsigned long sub_17570(char *a0,char *a1,bool a2,long *a3,unsigned long *a4)
{
  char v1;
  unsigned long v2; // rax
  long v3; // rax
  char *v4; // rax
  
  if (a2) { // branch-flip
    *a3 = 0;
    *a4 = 0;
    v1 = *a1;
  }
  else {
    v3 = *a4 + *a3;
    *a3 = v3;
    if (!a0[v3])
      return 0;
    *a3 = v3 + 1;
    v1 = *a1;
    a0 = &a0[v3 + 1];
  }
  if ((v1) && (v4 = strpbrk(a0,a1), v4))
    v2 = (long)v4 - (long)a0;
  else {
    v2 = strlen(a0);
  }
  *a4 = v2;
  return 1;
}

// Function: sub_175f0 @ 0x175f0
void sub_175f0(FILE *a0,char *a1)
{
  __fprintf_chk(a0,1,dcgettext(NULL,"Please see also the documentation at %s.\nYou can report (and track progress on fixing) bugs in the \"%s\"\nprogram via the %s bug-reporting page at\n%s or, if\nyou have no web access, by sending email to <%s>.\n",5),"http://www.gnu.org/software/findutils/",a1,"GNU findutils","https://savannah.gnu.org/bugs/?group=findutils","bug-findutils@gnu.org");
}

// Function: sub_17650 @ 0x17650
char * sub_17650(unsigned long a0,unsigned long a1)
{
  char *v1; // rax
  
  v1 = sub_177c0(a0,a1,NULL,0,NULL,dat_37f90); // tail-call
  return v1;
}

// Function: sub_17670 @ 0x17670
void sub_17670(unsigned long a0) // return-dupe
{
  struct_27 *v1; // rax
  
  v1 = malloc(0x30);
  if (!v1)
    return;
  v1->field_0x0 = 0;
  v1->field_0x20 = a0;
  v1->field_0x28 = a0;
}

// Function: sub_176a0 @ 0x176a0
void sub_176a0(unsigned long a0) // return-dupe
{
  unsigned long *v1; // rax
  
  v1 = malloc(0x30);
  if (!v1)
    return;
  *v1 = a0;
  v1[2] = 0;
  v1[3] = 0;
  v1[1] = 0;
  v1[4] = 0;
}

// Function: sub_176e0 @ 0x176e0
long sub_176e0(struct_28 *a0,int *a1) // early-return x2
{
  FILE *v1;
  long *v2;
  long v3;
  
  v1 = a0->field_0x0;
  if (!v1) {
    v2 = a0->field_0x28;
    v3 = *v2;
    if (!v3) {
      *a1 = 2;
      return 0;
    }
    *a1 = 1;
    a0->field_0x28 = &v2[1];
    return v3;
  }
  if (0 <= getdelim((char **)&a0->field_0x10,a0->field_0x18,0,v1)) {
    *a1 = 1;
    a0->field_0x8 = a0->field_0x8 + 1;
    return a0->field_0x10;
  }
  v1 = a0->field_0x0;
  *a1 = (-(unsigned int)(feof(v1) == 0) & 2) + 2;
  return 0;
}

// Function: sub_17780 @ 0x17780
long sub_17780(struct_29 *a0)
{
  if (a0->field_0x0)
    return a0->field_0x8;
  return a0->field_0x28 - a0->field_0x20 >> 3;
}

// Function: sub_177a0 @ 0x177a0
void sub_177a0(struct_30 *a0)
{
  if (a0->field_0x0)
    free(a0->field_0x10);
  free(a0); // tail-call
}

// Function: sub_177c0 @ 0x177c0
char * sub_177c0(unsigned int a0,unsigned long a1,char *a2,unsigned long a3,unsigned long *a4,void *a5)
{
  int v1;
  long v2; // rax
  char *v3;
  int *v4; // rax
  char *v5; // rax
  void *v6; // rax
  char v7 [1032];
  long v8;
  long v9;
  
  if (!a4)
    a4 = (unsigned long *)0x37860;
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
    if (v2 < v9) {
      v3[v2] = 0;
      v8 = v2 + 1;
      if (v3 != v7) {
        if (v9 <= v8)
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
    if (0x3ffffffffffffffe < v9) {
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

// Function: sub_17990 @ 0x17990
int sub_17990(unsigned long a0,bool a1)
{
  unsigned int v1;
  int v2; // eax
  int v3; // r8d
  
  v2 = sub_17dd0(a0,1,0);
  if (0 <= v2) { // branch-flip
    v1 = v2 & 0xfffffffe;
    if (a1)
      v1 = v2 | 1;
    v3 = 0;
    if (v1 != v2)
      v3 = -(unsigned int)(sub_17dd0(a0 & 0xffffffff,2) == -1);
  }
  else {
    v3 = -1;
  }
  return v3;
}

// Function: sub_17a00 @ 0x17a00
void sub_17a00(unsigned long a0)
{
  sub_17dd0(a0,0x406,0); // tail-call
}

// Function: sub_17a20 @ 0x17a20
void sub_17a20(unsigned long a0)
{
  dat_38ea0 = a0;
}

// Function: sub_17a30 @ 0x17a30
void sub_17a30(char a0)
{
  dat_38e98 = a0;
}

// Function: sub_17a40 @ 0x17a40
void sub_17a40(void)
{
  int v1; // eax
  int *v2; // rax
  char *v3; // rax
  char *v4; // rax
  
  v1 = sub_27370(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!dat_38e98) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (dat_38ea0) { // branch-flip
        v4 = (char *)sub_23d80(dat_38ea0);
        error(0,*v2,"%s: %s",v4,v3);
      }
      else {
        error(0,*v2,"%s",v3);
      }
      _exit(dat_384b8); // return-dupe, no-return
    }
  }
  v1 = sub_27370(stderr);
  if (!v1)
    return;
  _exit(dat_384b8);
}

// Function: sub_17af0 @ 0x17af0
DIR * sub_17af0(char *a0)
{
  unsigned int v1; // eax
  int v2; // eax
  DIR *v3; // rax
  int *v4; // rax
  DIR *v5; // rax
  int v6;
  
  v3 = opendir(a0);
  if (v3) {
    v1 = dirfd(v3);
    if (v1 <= 2) {
      v2 = sub_17dd0(v1,0x406,3);
      v4 = __errno_location();
      if (0 <= v2) { // branch-flip
        v5 = fdopendir(v2);
        v6 = *v4;
        if (!v5)
          close(v2);
      }
      else {
        v6 = *v4;
        v5 = NULL;
      }
      closedir(v3);
      *v4 = v6;
      return v5;
    }
  }
  return v3;
}

// Function: sub_17b90 @ 0x17b90
void sub_17b90(char *a0)
{
  char *v1; // rax
  long v2; // rax
  unsigned long v3; // rax
  void *v4; // rax
  
  v1 = (char *)sub_26f10(a0);
  if (*v1) { // branch-flip
    v2 = sub_26f70(v1);
    v3 = v2 + (unsigned long)(v1[v2] == '/');
    a0 = v1;
  }
  else {
    v3 = sub_26f70(a0);
  }
  v4 = (void *)sub_25a70(v3 + 1);
  *(char *)((long)memcpy(v4,a0,v3) + v3) = 0;
}

// Function: sub_17c00 @ 0x17c00
void sub_17c00(char *a0)
{
  char v1;
  long v2;
  unsigned long v3;
  
  v1 = *a0;
  v3 = sub_26f10(a0) - (long)a0;
  do {
    if ((v1 == '/') >= v3) break;
    v2 = v3 - 1;
    v3 -= 1;
  } while (a0[v2] == '/');
}

// Function: sub_17c40 @ 0x17c40
char * sub_17c40(char *a0)
{
  unsigned long v1;
  char v2;
  void *v3; // rax
  char *v4; // rax
  unsigned long v5;
  
  v2 = *a0;
  v5 = sub_26f10(a0) - (long)a0;
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
label_17cae:
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
      goto label_17cae;
    }
    v5 = v1;
  } while( true );
}

// Function: sub_17d00 @ 0x17d00
bool sub_17d00(char *a0)
{
  char v1;
  char *v2;
  long v3; // rax
  
  v2 = (char *)sub_26f10(a0);
  if (!*v2)
    v2 = a0;
  v3 = sub_26f70(v2);
  v1 = v2[v3];
  v2[v3] = '\0';
  return v1 != '\0';
}

// Function: sub_17d40 @ 0x17d40
unsigned long sub_17d40(FILE *a0) // early-return
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
    if (sub_180c0(a0)) {
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

// Function: sub_17dd0 @ 0x17dd0
int sub_17dd0(int a0,int a1,unsigned long a2)
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
    if (0 <= dat_38ea8) { // branch-flip
      v2 = fcntl(a0,0x406,a2 & 0xffffffff);
      if ((0 <= v2) || (*__errno_location() != 0x16)) {
        dat_38ea8 = 1;
        return v2;
      }
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      dat_38ea8 = -1;
    }
    else {
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      if (dat_38ea8 != -1)
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

// Function: sub_18040 @ 0x18040
void sub_18040(char *a0,unsigned int a1,unsigned long a2)
{
  unsigned long v1; // rdx
  
  v1 = 0;
  if (a1 & 0x40)
    v1 = a2 & 0xffffffff;
  sub_252e0(open(a0,a1,v1));
}

// Function: sub_180c0 @ 0x180c0
void sub_180c0(FILE *a0)
{
  if (a0) {
    if ((__freading(a0)) && (*(unsigned int *)a0 & 0x100)) {
      sub_18340(a0,0,1);
      fflush(a0); // tail-call
      return;
    }
  }
  fflush(a0); // tail-call
}

// Function: sub_18110 @ 0x18110
void sub_18110(unsigned int a0,struct_31 *a1) // ternary x2
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

// Function: sub_18290 @ 0x18290
void sub_18290(void *a0,struct_31 *a1)
{
  sub_18110(*(unsigned int *)((long)a0 + 0x18),a1); // tail-call
}

// Function: sub_182a0 @ 0x182a0
FILE * sub_182a0(char *a0,char *a1)
{
  int v1;
  unsigned int v2; // eax
  int v3;
  FILE *v4; // rax
  FILE *v5;
  int *v6; // rax
  
  v4 = fopen(a0,a1);
  v5 = v4;
  if ((v4) && (v2 = fileno(v4), v2 <= 2)) {
    v3 = sub_252c0(v2);
    if (0 <= v3) { // branch-flip
      if ((!sub_17d40(v4)) && (v5 = fdopen(v3,a1), v5))
        return v5;
      v6 = __errno_location();
      v1 = *v6;
      close(v3);
      *v6 = v1;
      v5 = NULL;
    }
    else {
      v6 = __errno_location();
      v5 = NULL;
      v3 = *v6;
      sub_17d40(v4);
      *v6 = v3;
    }
  }
  return v5;
}

// Function: sub_18340 @ 0x18340
int sub_18340(FILE *a0,long a1,int a2) // return-dupe
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

// Function: sub_183c0 @ 0x183c0
unsigned long sub_183c0(struct_12 *a0,struct_12 *a1)
{
  if (a0->field_0x8 != a1->field_0x8)
    return 0;
  return CONCAT71((undefined7)((unsigned long)a1->field_0x0 >> 8),a0->field_0x0 == a1->field_0x0);
}

// Function: sub_183f0 @ 0x183f0
unsigned long sub_183f0(void *a0,unsigned long a1)
{
  return *(unsigned long *)((long)a0 + 8) % a1;
}

// Function: sub_18410 @ 0x18410
unsigned long sub_18410(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}

// Function: sub_18420 @ 0x18420
unsigned long sub_18420(long *a0,long *a1)
{
  return CONCAT71((undefined7)((unsigned long)*a1 >> 8),*a0 == *a1);
}

// Function: sub_18430 @ 0x18430
int sub_18430(long *a0,long *a1)
{
  unsigned long v1;
  unsigned long v2;
  
  v1 = *(unsigned long *)(*a1 + 0x78);
  v2 = *(unsigned long *)(*a0 + 0x78);
  return (unsigned int)(v2 >= v1 && v2 != v1) - (unsigned int)(v2 < v1);
}

// Function: sub_18450 @ 0x18450
void * sub_18450(void *a0,void *a1,unsigned long a2)
{
  unsigned long v1;
  long *v2;
  unsigned long v3;
  long *v4;
  void *v5; // rax
  long v6; // rcx
  long *v7;
  void *v8;
  
  v3 = *(unsigned long *)((long)a0 + 0x40);
  v4 = *(long **)((long)a0 + 0x10);
  v7 = v4;
  if (a2 <= *(unsigned long *)((long)a0 + 0x38)) {
label_184ab:
    while (a1) {
      *v4 = (long)a1;
      v4 = &v4[1];
      a1 = *(void **)((long)a1 + 0x10);
    }
    qsort(v7,a2,8,v3);
    v4 = *(long **)((long)a0 + 0x10);
    v8 = (void *)*v4;
    v5 = v8;
    v6 = a2 - 1;
    v7 = v4;
    if (v6) {
      while( true ) {
        v2 = &v7[1];
        v7 = &v7[1];
        *(long *)((long)v8 + 0x10) = *v2;
        v6 -= 1;
        if (!v6) break;
        v8 = (void *)*v7;
      }
      v8 = (void *)v4[a2 - 1];
    }
    *(unsigned long *)((long)v8 + 0x10) = 0;
    return v5;
  }
  v1 = a2 + 0x28;
  *(unsigned long *)((long)a0 + 0x38) = v1;
  if (!(v1 >> 0x3d)) {
    v4 = realloc(v4,v1 * 8);
    if (v4) {
      *(long **)((long)a0 + 0x10) = v4;
      v7 = v4;
      goto label_184ab;
    }
    v4 = *(long **)((long)a0 + 0x10);
  }
  free(v4);
  *(unsigned long *)((long)a0 + 0x10) = 0;
  *(unsigned long *)((long)a0 + 0x38) = 0;
  return a1;
}

// Function: sub_18560 @ 0x18560
void * sub_18560(struct_11 *a0,void *a1,unsigned long a2)
{
  unsigned long v1;
  void *v2; // rax
  
  v2 = malloc(a2 + 0x108 & 0xfffffffffffffff8);
  if (v2) {
    memcpy((void *)((long)v2 + 0x100),a1,a2);
    v1 = a0->field_0x20;
    *(char *)((long)v2 + a2 + 0x100) = 0;
    *(unsigned long *)((long)v2 + 0x60) = a2;
    *(struct_11 **)((long)v2 + 0x50) = a0;
    *(unsigned long *)((long)v2 + 0x38) = v1;
    *(unsigned int *)((long)v2 + 0x40) = 0;
    *(unsigned long *)((long)v2 + 0x18) = 0;
    *(unsigned int *)((long)v2 + 0x6a) = 0x30000;
    *(unsigned long *)((long)v2 + 0x20) = 0;
    *(unsigned long *)((long)v2 + 0x28) = 0;
  }
  return v2;
}

// Function: sub_18600 @ 0x18600
unsigned long sub_18600(void *a0,int a1)
{
  long v1;
  struct_24 *v2;
  struct_25 *v3; // rax
  struct_25 *v4; // rax
  unsigned long v5 [2]; // stack - 0xb8
  statfs v6; // stack - 0xa8
  
  v1 = *(long *)((long)a0 + 0x50);
  v2 = *(struct_24 **)(v1 + 0x50);
  if (!(*(unsigned char *)(v1 + 0x49) & 2))
    return 0;
  if (!v2) {
    v2 = (struct_24 *)sub_1b2f0(0xd,NULL,sub_18410,sub_18420,dat_37f70);
    *(struct_24 **)(v1 + 0x50) = v2;
    if (!v2) goto label_1868e;
  }
  v5[0] = *(unsigned long *)((long)a0 + 0x70);
  v1 = sub_1b060(v2,v5);
  if (v1)
    return *(unsigned long *)(v1 + 8);
label_1868e:
  if ((0 <= a1) && (!fstatfs(a1,&v6))) {
    if ((v2) && (v3 = malloc(0x10), v3)) {
      v3->field_0x0 = *(unsigned long *)((long)a0 + 0x70);
      v3->field_0x8 = v6._0_8_;
      v4 = (struct_25 *)sub_1b950(v2,v3);
      if (v4) { // branch-flip
        if (v3 != v4)
          abort(); // no-return
      }
      else {
        free(v3);
      }
    }
    return v6._0_8_;
  }
  return 0;
}

// Function: sub_18730 @ 0x18730
void sub_18730(void *a0,int a1,bool a2)
{
  int v1;
  
  v1 = *(int *)((long)a0 + 0x2c);
  if ((v1 == a1) && (v1 != -100))
    abort(); // no-return
  if (a2) { // branch-flip
    v1 = sub_1c970((void *)((long)a0 + 0x60));
    if (0 <= v1) {
      close(v1);
      goto label_18774;
    }
  }
  else if ((!(*(unsigned char *)((long)a0 + 0x48) & 4)) && (0 <= v1)) {
    close(v1);
label_18774:
    *(int *)((long)a0 + 0x2c) = a1;
    return;
  }
  *(int *)((long)a0 + 0x2c) = a1;
}

// Function: sub_18790 @ 0x18790
unsigned long sub_18790(void *a0,long a1)
{
  void *v1; // rax
  unsigned long v2; // rsi
  
  v2 = a1 + 0x100U + *(unsigned long *)((long)a0 + 0x30);
  if (CARRY8(a1 + 0x100U,*(unsigned long *)((long)a0 + 0x30))) {
    free(*(void **)((long)a0 + 0x20));
    *(unsigned long *)((long)a0 + 0x20) = 0;
    *__errno_location() = 0x24;
    return 0;
  }
  *(unsigned long *)((long)a0 + 0x30) = v2;
  v1 = realloc(*(void **)((long)a0 + 0x20),v2);
  if (v1) {
    *(void **)((long)a0 + 0x20) = v1;
    return 1;
  }
  free(*(void **)((long)a0 + 0x20));
  *(unsigned long *)((long)a0 + 0x20) = 0;
  return 0;
}

// Function: sub_18800 @ 0x18800
unsigned long sub_18800(void *a0)
{
  unsigned long v1; // rax
  void *v2; // rax
  
  if (*(unsigned int *)((long)a0 + 0x48) & 0x102) {
    v1 = sub_1b2f0(0x1f,NULL,sub_183f0,sub_183c0,dat_37f70);
    *(long *)((long)a0 + 0x58) = v1;
    return CONCAT71((undefined7)(v1 >> 8),v1 != 0);
  }
  v2 = malloc(0x20);
  *(void **)((long)a0 + 0x58) = v2;
  if (v2) {
    sub_273e0(v2);
    return 1;
  }
  return 0;
}

// Function: sub_18870 @ 0x18870
unsigned long sub_18870(void *a0,struct_22 *a1) // return-dupe x2
{
  void *v1;
  struct_23 *v2; // rax
  struct_23 *v3; // rax
  unsigned long v4;
  
  if (!(*(unsigned int *)((long)a0 + 0x48) & 0x102)) {
    v4 = sub_27400(*(struct_21 **)((long)a0 + 0x58),&a1->field_0x70);
    if (!(char)v4)
      return 1;
    a1->field_0x0 = a1;
    a1->field_0x68 = 2;
    return v4;
  }
  v2 = malloc(0x18);
  if (!v2)
    return 0;
  v4 = a1->field_0x70;
  v1 = *(void **)((long)a0 + 0x58);
  v2->field_0x10 = a1;
  *(unsigned long *)v2->field_0x0 = v4;
  *(unsigned long *)((long)v2->field_0x0 + 8) = a1->field_0x78;
  v3 = (struct_23 *)sub_1b950(v1,v2);
  if (v2 == v3)
    return 1;
  free(v2);
  if (v3) {
    v4 = v3->field_0x10;
    a1->field_0x68 = 2;
    a1->field_0x0 = v4;
    return 1;
  }
  return 0;
}

// Function: sub_18920 @ 0x18920
void sub_18920(void *a0,void *a1)
{
  long v1;
  long *v2;
  long v3;
  void *v4; // rax
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  
  if (*(unsigned int *)((long)a0 + 0x48) & 0x102) { // branch-flip
    v5 = *(unsigned long *)((long)a1 + 0x70);
    v6 = *(unsigned long *)((long)a1 + 0x78);
    v4 = (void *)sub_1b9b0(*(void **)((long)a0 + 0x58),&v5);
    if (!v4)
      abort(); // no-return, return-dupe
    free(v4);
  }
  else {
    v1 = *(long *)((long)a1 + 8);
    if ((v1) && (0 <= *(long *)(v1 + 0x58))) {
      v2 = *(long **)((long)a0 + 0x58);
      if (!v2[2]) {
        abort();
      }
      if ((*v2 == *(long *)((long)a1 + 0x78)) && (v2[1] == *(long *)((long)a1 + 0x70))) {
        v3 = *(long *)(v1 + 0x78);
        v2[1] = *(long *)(v1 + 0x70);
        *v2 = v3;
      }
    }
  }
}

// Function: sub_189e0 @ 0x189e0
int sub_189e0(int a0,unsigned long a1,void *a2,bool a3) // early-return x3
{
  stat *v1;
  unsigned long *v2;
  char *v3;
  unsigned int v4; // eax
  int v5;
  int *v6; // rax
  unsigned long v7; // rcx
  unsigned long *v8;
  
  v1 = (stat *)((long)a2 + 0x70);
  if ((((*(long *)((long)a2 + 0x58)) || (!(a1 & 1))) && (!(a1 & 2))) && (!a3)) {
    v3 = *(char **)((long)a2 + 0x30);
    if (!fstatat(a0,v3,v1,0x100)) {
label_18a62:
      v4 = *(unsigned int *)((long)a2 + 0x88) & 0xf000;
      if (v4 != 0x4000) { // branch-flip
        if (v4 == 0xa000)
          return 0xc;
        v5 = (unsigned int)(v4 == 0x8000) * 5 + 3;
      }
      else {
        if (*(char *)((long)a2 + 0x100) != '.')
          return 1;
        if ((*(char *)((long)a2 + 0x101)) && ((*(unsigned int *)((long)a2 + 0x100) & 0xffff00) != 0x2e00))
          return 1;
        v5 = (-(unsigned int)(*(long *)((long)a2 + 0x58) == 0) & 0xfffffffc) + 5;
      }
      return v5;
    }
    v5 = *__errno_location();
  }
  else {
    v3 = *(char **)((long)a2 + 0x30);
    if (!stat(v3,v1)) goto label_18a62;
    v6 = __errno_location();
    v5 = *v6;
    if (v5 == 2) {
      v3 = *(char **)((long)a2 + 0x30);
      if (!lstat(v3,v1)) {
        *v6 = 0;
        return 0xd;
      }
      v5 = *v6;
    }
  }
  *(int *)((long)a2 + 0x40) = v5;
  v8 = (unsigned long *)((long)a2 + 0x78U & 0xfffffffffffffff8);
  *(unsigned long *)((long)a2 + 0x70) = 0;
  *(unsigned long *)((long)a2 + 0xf8) = 0;
  v7 = (unsigned long)(((int)v1 - (int)v8) + 0x90U >> 3);
  while (v7) {
    v2 = &v8[1];
    *v8 = 0;
    v7 -= 1;
    v8 = v2;
  }
  return 10;
}

// Function: sub_18b40 @ 0x18b40
bool sub_18b40(void *a0)
{
  int v1;
  bool v2;
  
  if (*(unsigned int *)((long)a0 + 0x48) & 4) // branch-flip
    v2 = 0;
  else if (*(unsigned int *)((long)a0 + 0x48) & 0x200) { // branch-flip
    sub_18730(a0,0xffffff9c,1);
    v2 = 0;
  }
  else {
    v1 = *(int *)((long)a0 + 0x28);
    v2 = fchdir(v1) != 0;
  }
  while( true ) {
    if (sub_1c960((void *)((long)a0 + 0x60))) break;
    v1 = sub_1c9b0((void *)((long)a0 + 0x60));
    if (0 <= v1)
      close(v1);
  }
  return v2;
}

// Function: sub_18bd0 @ 0x18bd0
unsigned int sub_18bd0(void *a0,void *a1,int a2,char *a3) // return-dupe
{
  unsigned int v1;
  unsigned char v10; // stack - 0xdd
  int v2;
  int v3;
  unsigned char v4; // al
  int v5; // eax
  int *v6; // rax
  unsigned int v7;
  stat v8; // stack - 0xd8
  int v9;
  
  v9 = a2;
  if ((a3) && (v5 = strcmp(a3,".."), !v5)) {
    v7 = *(unsigned int *)((long)a0 + 0x48);
    if (v7 & 4) {
label_18d80:
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
      if (!(v7 & 2)) goto label_18d5f;
    }
    else {
      if (!(v7 & 0x200)) {
        v10 = 1;
        v7 = (v7 & 0x10) << 0xd | 0x90900;
        goto label_18de4;
      }
      v4 = sub_1c960((void *)((long)a0 + 0x60));
      if (v4) {
        v1 = *(unsigned int *)((long)a0 + 0x2c);
        v7 = (v7 & 0x10) << 0xd | 0x90900;
        v10 = v4;
        goto label_18c75;
      }
      v5 = sub_1c9b0((void *)((long)a0 + 0x60));
      v7 = *(unsigned int *)((long)a0 + 0x48);
      if (v5 < 0) {
        v10 = 1;
        v5 = v7;
        goto label_18e74;
      }
      v10 = 1;
      a2 = v5;
      v9 = v5;
      if (!(v7 & 2)) goto label_18ce0;
    }
label_18ca0:
    v5 = fstat(v9,&v8);
    if (!v5) {
      if ((*(long *)((long)a1 + 0x70) == v8._0_8_) && (*(long *)((long)a1 + 0x78) == v8._8_8_)) {
        v7 = *(unsigned int *)((long)a0 + 0x48);
        goto label_18ce0;
      }
      v6 = __errno_location();
      *v6 = 2;
    }
    v3 = -1;
  }
  else {
    v7 = *(unsigned int *)((long)a0 + 0x48);
    if (v7 & 4) goto label_18d80;
    if (0 <= a2) // branch-flip
      v10 = 0;
    else {
      v10 = 0;
      v5 = v7;
label_18e74:
      v5 &= 0x200;
      v1 = *(unsigned int *)((long)a0 + 0x2c);
      v7 = (v7 & 0x10) << 0xd | 0x90900;
      if (v5) { // branch-flip
label_18c75:
        v5 = sub_1f080(v1,a3,v7);
      }
      else {
label_18de4:
        v5 = sub_18040(a3,v7);
      }
      if (v5 < 0) {
        v5 = -1;
        return v5;
      }
      v7 = *(unsigned int *)((long)a0 + 0x48);
      v9 = v5;
    }
    if (v7 & 2) goto label_18ca0;
    if (a3) {
label_18d5f:
      v5 = strcmp(a3,"..");
      if (!v5) goto label_18ca0;
    }
label_18ce0:
    if (v7 & 0x200) {
      sub_18730(a0,v9,v10 ^ 1);
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

// Function: sub_18ed0 @ 0x18ed0
void * sub_18ed0(unsigned long *a0,int a1)
{
  unsigned char v1;
  unsigned long v10; // rax
  void *v11;
  void *v12;
  long v13;
  long v14;
  void *v15;
  unsigned int v16;
  unsigned long v17;
  int v18; // stack - 0x44
  char *v19;
  DIR *v2;
  DIR *v20;
  unsigned long v21;
  unsigned long v22;
  bool v23;
  unsigned long v24; // stack - 0xa0
  void *v25; // stack - 0x98
  void *v26; // stack - 0x90
  char *v27; // stack - 0x70
  unsigned long v28; // stack - 0x68
  unsigned long v29; // stack - 0x60
  bool v3;
  bool v30; // stack - 0x4b
  char v4; // al
  unsigned int v5; // eax
  unsigned int v6;
  int v7;
  int *v8; // rax
  dirent *v9; // rax
  
  v15 = (void *)*a0;
  v2 = *(DIR **)((long)v15 + 0x18);
  if (v2) { // branch-flip
    v18 = dirfd(v2);
    if (v18 <= -1) {
      closedir(*(DIR **)((long)v15 + 0x18));
      *(unsigned long *)((long)v15 + 0x18) = 0;
      if (a1 == 3) {
        *(unsigned short *)((long)v15 + 0x68) = 4;
        *(int *)((long)v15 + 0x40) = *__errno_location();
        return (void *)0;
      }
      return (void *)0;
    }
    if (a0[8]) { // branch-flip
      v28 = 0xffffffffffffffff;
label_18f32:
      v23 = 1;
      v16 = *(unsigned int *)&a0[9];
    }
    else {
      v28 = 100000;
      v16 = *(unsigned int *)&a0[9];
      v23 = 1;
    }
  }
  else {
    v16 = *(unsigned int *)&a0[9];
    v7 = 0;
    if ((v16 & 0x10) && (v7 = 0x20000, v16 & 1))
      v7 = (unsigned int)(*(long *)((long)v15 + 0x58) != 0) << 0x11;
    v6 = 0xffffff9c;
    if ((v16 & 0x204) == 0x200)
      v6 = *(unsigned int *)((long)a0 + 0x2c);
    v14 = sub_1f100(v6,*(char **)((long)v15 + 0x30),v7,&v18);
    *(long *)((long)v15 + 0x18) = v14;
    if (!v14) {
      if (a1 == 3) {
        *(unsigned short *)((long)v15 + 0x68) = 4;
        *(int *)((long)v15 + 0x40) = *__errno_location();
        return NULL;
      }
      return (void *)0;
    }
    v16 = *(unsigned int *)&a0[9];
    if (*(short *)((long)v15 + 0x68) != 0xb) { // branch-flip
      if (v16 & 0x100) {
        sub_18920(a0,v15);
        sub_189e0(*(unsigned int *)((long)a0 + 0x2c),*(unsigned int *)&a0[9],v15,0);
        if (!sub_18870(a0,v15)) {
          *__errno_location() = 0xc;
          return NULL;
        }
      }
    }
    else {
      v6 = *(unsigned int *)((long)a0 + 0x2c);
      *(unsigned short *)((long)v15 + 0x68) = sub_189e0(v6,v16,v15,0);
    }
    v28 = (unsigned long)(-(unsigned int)(a0[8] == 0) & 0x186a1) - 1;
    if (a1 == 2) {
label_19760:
      v23 = 0;
      v16 = *(unsigned int *)&a0[9];
      goto label_18f3b;
    }
    v7 = v18;
    if (((*(unsigned int *)&a0[9] & 0x38) != 0x18) || (*(long *)((long)v15 + 0x80) != 2)) {
label_1940a:
      v23 = a1 == 3;
      v3 = 1;
    }
    else {
      v14 = sub_18600(v15,v18);
      v7 = v18;
      if (v14 == 0x9fa0) goto label_1940a;
      if (v14 <= 0x9fa0) {
        if ((v14) && (v14 != 0x6969)) goto label_195a1;
        goto label_1940a;
      }
      if ((v14 == 0x5346414f) || (v14 == 0xff534d42)) goto label_1940a;
label_195a1:
      if (a1 != 3) goto label_19760;
      v3 = 0;
      v23 = 1;
    }
    if (*(unsigned char *)((long)a0 + 0x49) & 2) {
      v7 = sub_17dd0(v7,0x406,3);
      v18 = v7;
    }
    if ((0 <= v7) && (!sub_18bd0(a0,v15,v7,NULL))) goto label_18f32;
    if ((v23) && (v3))
      *(int *)((long)v15 + 0x40) = *__errno_location();
    *(unsigned short *)((long)v15 + 0x6a) = *(unsigned short *)((long)v15 + 0x6a) | 1;
    closedir(*(DIR **)((long)v15 + 0x18));
    v16 = *(unsigned int *)&a0[9];
    *(unsigned long *)((long)v15 + 0x18) = 0;
    if ((v16 & 0x200) && (0 <= v18)) {
      close(v18);
      v16 = *(unsigned int *)&a0[9];
    }
    *(unsigned long *)((long)v15 + 0x18) = 0;
    v23 = 0;
  }
label_18f3b:
  v22 = *(unsigned long *)((long)v15 + 0x48);
  v24 = v22;
  v29 = v22 - 1;
  if (*(char *)(*(long *)((long)v15 + 0x38) + -1 + v22) != '/') {
    v24 = v22 + 1;
    v29 = v22;
  }
  v27 = NULL;
  if (v16 & 4) {
    v27 = &((char *)(v29 + a0[4]))[1];
    *(char *)(v29 + a0[4]) = 0x2f;
  }
  v20 = *(DIR **)((long)v15 + 0x18);
  v14 = *(long *)((long)v15 + 0x58);
  v22 = a0[6] - v24;
  if (v20) { // branch-flip
    v8 = __errno_location();
    v21 = 0;
    v3 = 0;
    v30 = 0;
    v25 = NULL;
    v26 = NULL;
    do {
      while( true ) {
        *v8 = 0;
        v9 = readdir(v20);
        if (!v9) {
          if (*v8) {
            *(int *)((long)v15 + 0x40) = *v8;
            *(unsigned short *)((long)v15 + 0x68) = (-(unsigned short)(!v2 && !v21) & 0xfffd) + 7;
          }
          if (*(DIR **)((long)v15 + 0x18)) {
            closedir(*(DIR **)((long)v15 + 0x18));
            *(unsigned long *)((long)v15 + 0x18) = 0;
          }
          goto label_1960e;
        }
        if (((*(unsigned char *)&a0[9] & 0x20) || (v9->field_0x13 != '.')) || ((v9->field_0x14 && (*(short *)&v9->field_0x14 != 0x2e)))) break;
label_19004:
        v20 = *(DIR **)((long)v15 + 0x18);
        if (!v20) goto label_1960e;
      }
      v10 = strlen(&v9->field_0x13);
      v11 = (void *)sub_18560(a0,&v9->field_0x13,v10);
      if (!v11) {
label_194a8:
        v7 = *v8;
        free(v11);
        while (v26) {
          v11 = *(void **)((long)v26 + 0x10);
          if (*(DIR **)((long)v26 + 0x18))
            closedir(*(DIR **)((long)v26 + 0x18));
          free(v26);
          v26 = v11;
        }
        closedir(*(DIR **)((long)v15 + 0x18));
        *(unsigned long *)((long)v15 + 0x18) = 0;
        *(unsigned short *)((long)v15 + 0x68) = 7;
        *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
        *v8 = v7;
        return NULL;
      }
      if (v22 <= v10) {
        v13 = a0[4];
        v4 = sub_18790(a0,v29 + 2 + v10);
        if (!v4) goto label_194a8;
        if (a0[4] != v13) {
          v19 = (char *)(a0[4] + v24);
          if (!(*(unsigned char *)&a0[9] & 4))
            v19 = v27;
          v30 = v4;
          v27 = v19;
        }
        v22 = a0[6] - v24;
      }
      if (CARRY8(v10,v24)) {
        free(v11);
        while (v26) {
          v11 = *(void **)((long)v26 + 0x10);
          if (*(DIR **)((long)v26 + 0x18))
            closedir(*(DIR **)((long)v26 + 0x18));
          free(v26);
          v26 = v11;
        }
        closedir(*(DIR **)((long)v15 + 0x18));
        *(unsigned long *)((long)v15 + 0x18) = 0;
        *(unsigned short *)((long)v15 + 0x68) = 7;
        *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
        *v8 = 0x24;
        return NULL;
      }
      v16 = *(unsigned int *)&a0[9];
      *(long *)((long)v11 + 0x58) = v14 + 1;
      v17 = *a0;
      *(unsigned long *)((long)v11 + 0x48) = v10 + v24;
      *(unsigned long *)((long)v11 + 8) = v17;
      *(unsigned long *)((long)v11 + 0x78) = *(unsigned long *)v9;
      if (v16 & 4) { // branch-flip
        *(unsigned long *)((long)v11 + 0x30) = *(unsigned long *)((long)v11 + 0x38);
        memmove(v27,(void *)((long)v11 + 0x100),*(long *)((long)v11 + 0x60) + 1);
        v16 = *(unsigned int *)&a0[9];
      }
      else {
        *(void **)((long)v11 + 0x30) = (void *)((long)v11 + 0x100);
      }
      if ((a0[8]) && (!(v16 & 0x400))) {
        v6 = *(unsigned int *)((long)a0 + 0x2c);
        *(unsigned short *)((long)v11 + 0x68) = sub_189e0(v6,v16,v11,0);
      }
      else {
        v1 = v9->field_0x12;
        v5 = (unsigned int)v1 - 1;
        if ((v16 & 8) && (v1 & 0xfb)) {
          if ((v16 & 0x10) || (v1 != 10)) {
            *(unsigned short *)((long)v11 + 0x68) = 0xb;
            if (0xc <= v5) {
              v6 = 0;
              v17 = 1;
              goto label_190fc;
            }
            v17 = 1;
          }
          else {
            *(unsigned short *)((long)v11 + 0x68) = 0xb;
label_19268:
            v17 = 2;
          }
          v6 = *(unsigned int *)((unsigned long)v5 * 4 + 0x2c380);
        }
        else {
          *(unsigned short *)((long)v11 + 0x68) = 0xb;
          if (v5 <= 0xb) goto label_19268;
          v6 = 0;
          v17 = 2;
        }
label_190fc:
        *(unsigned int *)((long)v11 + 0x88) = v6;
        *(unsigned long *)((long)v11 + 0xa0) = v17;
      }
      *(unsigned long *)((long)v11 + 0x10) = 0;
      if (v26) // branch-flip
        *(void **)((long)v25 + 0x10) = v11;
      else {
        v26 = v11;
      }
      if (v21 != 10000) {
        v21 += 1;
        if (v21 < v28) {
          v25 = v11;
          goto label_19004;
        }
        break;
      }
      if (!a0[8]) {
        v13 = sub_18600(v15,v18);
        if (((v13 == 0x1021994) || (v13 == 0xff534d42)) || (v13 == 0x6969)) {
          v25 = v11;
          v21 = 0x2711;
          v3 = 0;
          goto label_19004;
        }
        v3 = 1;
      }
      v21 = 0x2711;
      v20 = *(DIR **)((long)v15 + 0x18);
      v25 = v11;
    } while (v20);
label_1960e:
    if (v30) {
      v13 = a0[4];
      for (v14 = a0[1]; v14; v14 = *(long *)(v14 + 0x10)) {
        if (*(long *)(v14 + 0x30) != v14 + 0x100)
          *(long *)(v14 + 0x30) = (*(long *)(v14 + 0x30) - *(long *)(v14 + 0x38)) + v13;
        *(long *)(v14 + 0x38) = v13;
      }
      v14 = *(long *)((long)v26 + 0x58);
      v11 = v26;
      while (0 <= v14) {
        if (*(long *)((long)v11 + 0x30) != (long)v11 + 0x100)
          *(long *)((long)v11 + 0x30) = (*(long *)((long)v11 + 0x30) - *(long *)((long)v11 + 0x38)) + v13;
        v12 = *(void **)((long)v11 + 0x10);
        *(long *)((long)v11 + 0x38) = v13;
        if (!v12)
          v12 = *(void **)((long)v11 + 8);
        v14 = *(long *)((long)v12 + 0x58);
        v11 = v12;
      }
    }
    if (*(unsigned char *)&a0[9] & 4) {
      if ((a0[6] == v24) || (!v21)) goto label_1962b;
label_19631:
      *v27 = 0;
    }
    if ((!v2) && (v23)) {
      if ((a1 != 1) && (v21)) goto label_1965c;
      goto label_196b0;
    }
  }
  else {
    if (*(unsigned char *)&a0[9] & 4) {
      v3 = 0;
      v26 = NULL;
      v21 = 0;
label_1962b:
      v27 = &v27[-1];
      goto label_19631;
    }
    v26 = NULL;
    if (!(bool)(v23 & v2 == NULL)) goto label_196d5;
    v3 = 0;
    v21 = 0;
label_196b0:
    if (*(long *)((long)v15 + 0x58)) // branch-flip
      v7 = sub_18bd0(a0,*(void **)((long)v15 + 8),0xffffffff,"..");
    else {
      v7 = sub_18b40(a0);
    }
    if (v7) {
      *(unsigned short *)((long)v15 + 0x68) = 7;
      *(unsigned int *)&a0[9] = *(unsigned int *)&a0[9] | 0x2000;
      while (v26) {
        v15 = *(void **)((long)v26 + 0x10);
        if (*(DIR **)((long)v26 + 0x18))
          closedir(*(DIR **)((long)v26 + 0x18));
        free(v26);
        v26 = v15;
      }
      return (void *)0;
    }
  }
  if (!v21) {
label_196d5:
    if (((a1 == 3) && (*(short *)((long)v15 + 0x68) != 4)) && (*(short *)((long)v15 + 0x68) != 7))
      *(unsigned short *)((long)v15 + 0x68) = 6;
    while (v26) {
      v15 = *(void **)((long)v26 + 0x10);
      if (*(DIR **)((long)v26 + 0x18))
        closedir(*(DIR **)((long)v26 + 0x18));
      free(v26);
      v26 = v15;
    }
    return NULL;
  }
label_1965c:
  if (v3) {
    a0[8] = sub_18430;
    v15 = (void *)sub_18450(a0,v26,v21);
    a0[8] = 0;
    return v15;
  }
  if (!a0[8])
    return v26;
  if (v21 == 1)
    return v26;
  return (void *)sub_18450(a0,v26,v21);
}

// Function: sub_199d0 @ 0x199d0
long * sub_199d0(long *a0,unsigned long a1,long a2)
{
  unsigned int v1;
  void *v10;
  unsigned int v11;
  unsigned long v12;
  char *v13;
  long v14; // r13
  long *v15;
  void *v16; // stack - 0x50
  unsigned char v17; // stack - 0x42
  void *v18; // stack - 0x40
  unsigned int v2;
  void *v3;
  unsigned int v4;
  int v5; // eax
  long *v6; // rax
  unsigned long v7; // rax
  void *v8;
  long v9;
  
  v11 = (unsigned int)a1;
  if (((a1 & 0xfffff000) || ((v11 & 0x204) == 0x204)) || (!(a1 & 0x12))) {
    *__errno_location() = 0x16;
    return NULL;
  }
  v6 = calloc(1,0x80);
  if (!v6)
    return NULL;
  v6[8] = a2;
  v13 = (char *)*a0;
  *(unsigned int *)((long)v6 + 0x2c) = 0xffffff9c;
  v4 = v11 & 0xfffffdff | 4;
  if (!(a1 & 2))
    v4 = v11;
  *(unsigned int *)&v6[9] = v4;
  if (v13) { // branch-flip
    v12 = 0;
    v15 = a0;
    do {
      v7 = strlen(v13);
      if (v12 < v7)
        v12 = v7;
      v13 = (char *)v15[1];
      v15 = &v15[1];
    } while (v13);
    v12 += 1;
    if (v12 <= 0xfff)
      v12 = 0x1000;
  }
  else {
    v12 = 0x1000;
  }
  v17 = sub_18790(v6,v12);
  if (!v17) {
    free(v6); // return-dupe
    return NULL;
  }
  if (*a0) { // branch-flip
    v16 = (void *)sub_18560(v6,"",0);
    if (v16) {
      *(unsigned long *)((long)v16 + 0x58) = 0xffffffffffffffff;
      v13 = (char *)*a0;
      goto label_19ad3;
    }
  }
  else {
    v16 = NULL;
    v13 = NULL;
label_19ad3:
    if (a2)
      v17 = (unsigned char)(*(unsigned int *)&v6[9] >> 10) & 1;
    if (v13) { // branch-flip
      v18 = NULL;
      v10 = NULL;
      v9 = 0;
label_19b8d:
      do {
        v14 = v9;
        v3 = v18;
        v12 = strlen(v13);
        if (((3 <= v12) && (((unsigned char)(a1 >> 0xb) ^ 1) & 1)) && (v13[v12 - 1] == '/')) {
          do {
            if (v13[v12 - 2] != '/') break;
            v12 -= 1;
          } while (v12 != 1);
        }
        v8 = (void *)sub_18560(v6,v13,v12);
        if (!v8) goto label_19c1d;
        *(unsigned long *)((long)v8 + 0x58) = 0;
        *(void **)((long)v8 + 8) = v16;
        *(long *)((long)v8 + 0x30) = (long)v8 + 0x100;
        if ((v10) && (v17)) {
          *(unsigned long *)((long)v8 + 0xa0) = 2;
          *(unsigned short *)((long)v8 + 0x68) = 0xb;
          if (a2) { // branch-flip
label_19b75:
            *(void **)((long)v8 + 0x10) = v10;
          }
          else {
            *(unsigned long *)((long)v8 + 0x10) = 0;
label_19d28:
            v18 = v8;
            *(void **)((long)v3 + 0x10) = v8;
            v8 = v10;
          }
          v13 = (char *)a0[v14 + 1];
          v10 = v8;
          v9 = v14 + 1;
          if (!v13) break;
          goto label_19b8d;
        }
        v1 = *(unsigned int *)&v6[9];
        v2 = *(unsigned int *)((long)v6 + 0x2c);
        *(unsigned short *)((long)v8 + 0x68) = sub_189e0(v2,v1,v8,0);
        if (a2) goto label_19b75;
        *(unsigned long *)((long)v8 + 0x10) = 0;
        if (v10) goto label_19d28;
        v13 = (char *)a0[v14 + 1];
        v10 = v8;
        v9 = v14 + 1;
        v18 = v8;
      } while (v13);
      if ((a2) && (2 <= v14 + 1U))
        v8 = (void *)sub_18450(v6,v8,v14 + 1U);
      v9 = sub_18560(v6,"",0);
      *v6 = v9;
      v10 = v8;
      if (v9) {
        *(void **)(v9 + 0x10) = v8;
        *(unsigned short *)(v9 + 0x68) = 9;
        *(unsigned long *)(v9 + 0x58) = 1;
        if (sub_18800(v6)) {
label_19cf7:
          v11 = *(unsigned int *)&v6[9];
          if (!(v11 & 0x204)) {
            if (v11 & 0x200) // branch-flip
              v5 = sub_1f080(*(unsigned int *)((long)v6 + 0x2c),".");
            else {
              v5 = sub_18040(".",(v11 & 0x10) << 0xd | 0x90900);
            }
            *(int *)&v6[5] = v5;
            if (v5 <= -1)
              *(unsigned int *)&v6[9] = *(unsigned int *)&v6[9] | 4;
          }
          sub_1c940((struct_14 *)&v6[0xc],0xffffffff);
          return v6;
        }
      }
label_19c1d:
      while (v10) {
        v3 = *(void **)((long)v10 + 0x10);
        if (*(DIR **)((long)v10 + 0x18))
          closedir(*(DIR **)((long)v10 + 0x18));
        free(v10);
        v10 = v3;
      }
    }
    else {
      v9 = sub_18560(v6,"",0);
      *v6 = v9;
      if (v9) {
        *(unsigned long *)(v9 + 0x10) = 0;
        *(unsigned short *)(v9 + 0x68) = 9;
        *(unsigned long *)(v9 + 0x58) = 1;
        if (sub_18800(v6)) goto label_19cf7;
      }
    }
    free(v16);
  }
  free((void *)v6[4]);
  free(v6);
  return NULL;
}

// Function: sub_19e00 @ 0x19e00
unsigned int sub_19e00(struct_16 *a0) // return-dupe
{
  long v1;
  void *v2;
  struct_15 *v3;
  int v4; // eax
  int *v5; // rax
  void *v6;
  int v7;
  
  v6 = a0->field_0x0;
  if (v6) {
    if (*(long *)((long)v6 + 0x58) > -1) {
      do {
        v2 = *(void **)((long)v6 + 0x10);
        if (v2) { // branch-flip
          free(v6);
          v1 = *(long *)((long)v2 + 0x58);
          v6 = v2;
        }
        else {
          v2 = *(void **)((long)v6 + 8);
          free(v6);
          v1 = *(long *)((long)v2 + 0x58);
          v6 = v2;
        }
      } while (v1 >= 0);
    }
    free(v6);
  }
  v6 = a0->field_0x8;
  while (v6) {
    v2 = *(void **)((long)v6 + 0x10);
    if (*(DIR **)((long)v6 + 0x18))
      closedir(*(DIR **)((long)v6 + 0x18));
    free(v6);
    v6 = v2;
  }
  free(a0->field_0x10);
  free(a0->field_0x20);
  if (a0->field_0x48 & 0x200) { // branch-flip
    v7 = a0->field_0x2c;
    if ((0 <= v7) && (close(v7))) {
      v7 = *__errno_location();
      goto label_19ebc;
    }
  }
  else if (!(a0->field_0x48 & 4)) {
    v7 = a0->field_0x28;
    if (fchdir(v7)) { // branch-flip
      v5 = __errno_location();
      v7 = *v5;
      v4 = close(a0->field_0x28);
      if ((v7) || (!v4)) goto label_19ebc;
    }
    else {
      v7 = a0->field_0x28;
      if (!close(v7)) goto label_19eb9;
      v5 = __errno_location();
    }
    v7 = *v5;
    goto label_19ebc;
  }
label_19eb9:
  v7 = 0;
label_19ebc:
  while (!sub_1c960(&a0[1])) {
    v4 = sub_1c9b0(&a0[1]);
    if (0 <= v4)
      close(v4);
  }
  if (a0->field_0x50)
    sub_1b4b0(a0->field_0x50);
  v3 = a0->field_0x58;
  if (a0->field_0x48 & 0x102) { // branch-flip
    if (v3)
      sub_1b4b0(v3);
  }
  else {
    free(v3);
  }
  free(a0);
  if (!v7)
    return 0;
  *__errno_location() = v7;
  return 0xffffffff;
}

// Function: sub_19ff0 @ 0x19ff0
void * sub_19ff0(struct_26 *a0)
{
  short v1;
  void *v10;
  char *v11; // rax
  unsigned long v12; // rax
  void *v13;
  char *v14; // rdi
  long v15;
  unsigned int v2;
  unsigned int v3;
  char *v4;
  void *v5;
  struct_15 *v6;
  long v7;
  short v8;
  int v9;
  
  v10 = a0->field_0x0;
  if (!v10)
    return NULL;
  v2 = a0->field_0x48;
  if (v2 & 0x2000)
    return NULL;
  v8 = *(short *)((long)v10 + 0x6c);
  *(unsigned short *)((long)v10 + 0x6c) = 3;
  if (v8 == 1) {
    v3 = a0->field_0x2c;
    *(unsigned short *)((long)v10 + 0x68) = sub_189e0(v3,v2,v10,0);
    return v10;
  }
  v1 = *(short *)((long)v10 + 0x68);
  if (v8 != 2) { // branch-flip
    if (v1 != 1) { // branch-flip
label_1a078:
      do {
        v13 = *(void **)((long)v10 + 0x10);
        if (!v13) {
          v13 = *(void **)((long)v10 + 8);
          if (*(long *)((long)v13 + 0x18)) {
            v7 = *(long *)((long)v13 + 0x48);
            a0->field_0x0 = v13;
            *(char *)(a0->field_0x20 + v7) = 0;
            v13 = (void *)sub_18ed0(a0,3);
            if (v13) {
              free(v10);
              goto label_1a30a;
            }
            if (a0->field_0x48 & 0x2000)
              return NULL;
            v13 = *(void **)((long)v10 + 8);
          }
          a0->field_0x0 = v13;
          free(v10);
          if (*(long *)((long)v13 + 0x58) == -1) {
            free(v13);
            *__errno_location() = 0;
            a0->field_0x0 = NULL;
            return NULL;
          }
          if (*(short *)((long)v13 + 0x68) != 0xb) {
            *(char *)(a0->field_0x20 + *(long *)((long)v13 + 0x48)) = 0;
            if (*(long *)((long)v13 + 0x58)) { // branch-flip
              if (*(unsigned short *)((long)v13 + 0x6a) & 2) {
                v9 = *(int *)((long)v13 + 0x44);
                if (!(a0->field_0x48 & 4)) {
                  if (a0->field_0x48 & 0x200) { // branch-flip
                    sub_18730(a0,v9,1);
                    v9 = *(int *)((long)v13 + 0x44);
                  }
                  else {
                    if (fchdir(v9)) {
                      *(int *)((long)v13 + 0x40) = *__errno_location();
                      a0->field_0x48 = a0->field_0x48 | 0x2000;
                    }
                    v9 = *(int *)((long)v13 + 0x44);
                  }
                }
                close(v9);
                goto label_1a0e3;
              }
              if (*(unsigned short *)((long)v13 + 0x6a) & 1) goto label_1a0e3;
              v9 = sub_18bd0(a0,*(void **)((long)v13 + 8),0xffffffff,"..");
            }
            else {
              v9 = sub_18b40(a0);
            }
            if (v9) {
              *(int *)((long)v13 + 0x40) = *__errno_location();
              a0->field_0x48 = a0->field_0x48 | 0x2000;
            }
label_1a0e3:
            if (*(short *)((long)v13 + 0x68) != 2) {
              if (*(int *)((long)v13 + 0x40)) // branch-flip
                *(unsigned short *)((long)v13 + 0x68) = 7;
              else {
                *(unsigned short *)((long)v13 + 0x68) = 6;
                sub_18920(a0,v13);
              }
            }
            if (!(a0->field_0x48 & 0x2000))
              return v13;
            return NULL;
          }
          abort(); // no-return, return-dupe
        }
        a0->field_0x0 = v13;
        free(v10);
        v10 = v13;
        if (!*(long *)((long)v13 + 0x58)) {
          if (sub_18b40(a0)) {
            a0->field_0x48 = a0->field_0x48 | 0x2000;
            return NULL;
          }
          v6 = a0->field_0x58;
          if (a0->field_0x48 & 0x102) { // branch-flip
            if (v6)
              sub_1b4b0(v6);
          }
          else {
            free(v6);
          }
          v5 = (void *)a0->field_0x20;
          v4 = (char *)((long)v13 + 0x100);
          *(long *)((long)v13 + 0x48) = *(long *)((long)v13 + 0x60);
          memmove(v5,v4,*(long *)((long)v13 + 0x60) + 1);
          v11 = strrchr(v4,0x2f);
          if ((v11) && ((v4 != v11 || (*(char *)((long)v13 + 0x101))))) {
            v12 = strlen(&v11[1]);
            memmove(v4,&v11[1],v12 + 1);
            *(unsigned long *)((long)v13 + 0x60) = v12;
          }
          v7 = a0->field_0x20;
          *(long *)((long)v13 + 0x38) = v7;
          *(long *)((long)v13 + 0x30) = v7;
          sub_18800(a0);
          v8 = *(short *)((long)v13 + 0x68);
          goto label_1a34b;
        }
      } while (*(short *)((long)v13 + 0x6c) == 4);
      if (*(short *)((long)v13 + 0x6c) == 2) {
        v8 = sub_189e0(a0->field_0x2c,a0->field_0x48,v13,1);
        *(short *)((long)v13 + 0x68) = v8;
        if ((v8 == 1) && (v2 = a0->field_0x48, !(v2 & 4))) {
          if (v2 & 0x200) // branch-flip
            v9 = sub_1f080(a0->field_0x2c,".");
          else {
            v9 = sub_18040(".",(v2 & 0x10) << 0xd | 0x90900);
          }
          *(int *)((long)v13 + 0x44) = v9;
          if (0 <= v9) // branch-flip
            *(unsigned short *)((long)v13 + 0x6a) = *(unsigned short *)((long)v13 + 0x6a) | 2;
          else {
            v9 = *__errno_location();
            *(unsigned short *)((long)v13 + 0x68) = 7;
            *(int *)((long)v13 + 0x40) = v9;
          }
        }
        *(unsigned short *)((long)v13 + 0x6c) = 3;
      }
    }
    else {
      if (v8 == 4) goto label_1a1e2;
label_1a147:
      if ((v2 & 0x40) && (*(long *)((long)v10 + 0x70) != a0->field_0x18)) {
label_1a1e2:
        if (*(unsigned char *)((long)v10 + 0x6a) & 2)
          close(*(int *)((long)v10 + 0x44));
        v13 = a0->field_0x8;
        if (v13) {
          do {
            v5 = *(void **)((long)v13 + 0x10);
            if (*(DIR **)((long)v13 + 0x18))
              closedir(*(DIR **)((long)v13 + 0x18));
            free(v13);
            v13 = v5;
          } while (v5);
          a0->field_0x8 = NULL;
        }
        *(unsigned short *)((long)v10 + 0x68) = 6;
        sub_18920(a0,v10);
        return v10;
      }
      if (a0->field_0x8) { // branch-flip
        if (v2 & 0x1000) {
          a0->field_0x48 = v2 & 0xffffefff;
          v13 = a0->field_0x8;
          do {
            v5 = *(void **)((long)v13 + 0x10);
            if (*(DIR **)((long)v13 + 0x18))
              closedir(*(DIR **)((long)v13 + 0x18));
            free(v13);
            v13 = v5;
          } while (v5);
          a0->field_0x8 = NULL;
          goto label_1a442;
        }
        v4 = *(char **)((long)v10 + 0x30);
        if (sub_18bd0(a0,v10,0xffffffff,v4)) { // branch-flip
          v9 = *__errno_location();
          *(unsigned short *)((long)v10 + 0x6a) = *(unsigned short *)((long)v10 + 0x6a) | 1;
          *(int *)((long)v10 + 0x40) = v9;
          v13 = a0->field_0x8;
          if (v13) {
            v10 = v13;
            do {
              *(unsigned long *)((long)v10 + 0x30) = *(unsigned long *)(*(long *)((long)v10 + 8) + 0x30);
              v10 = *(void **)((long)v10 + 0x10);
            } while (v10);
          }
        }
        else {
          v13 = a0->field_0x8;
        }
      }
      else {
label_1a442:
        v13 = (void *)sub_18ed0(a0,3);
        a0->field_0x8 = v13;
        if (!v13) {
          if (!(a0->field_0x48 & 0x2000)) {
            if ((*(int *)((long)v10 + 0x40)) && (*(short *)((long)v10 + 0x68) != 4))
              *(unsigned short *)((long)v10 + 0x68) = 7;
            sub_18920(a0,v10);
            return v10;
          }
          return NULL;
        }
      }
      a0->field_0x8 = NULL;
    }
label_1a30a:
    v7 = *(long *)(*(long *)((long)v13 + 8) + 0x48);
    v15 = v7 + -1;
    if (*(char *)(*(long *)(*(long *)((long)v13 + 8) + 0x38) + -1 + v7) != '/')
      v15 = v7;
    v14 = (char *)(v15 + a0->field_0x20);
    *v14 = 0x2f;
    memmove(&v14[1],(void *)((long)v13 + 0x100),*(long *)((long)v13 + 0x60) + 1);
    v8 = *(short *)((long)v13 + 0x68);
    v10 = v13;
label_1a34b:
    a0->field_0x0 = v10;
    if (v8 == 0xb) goto label_1a3c8;
  }
  else {
    if (2 <= (unsigned short)(v1 - 0xcU)) {
      if (v1 == 1) goto label_1a147;
      goto label_1a078;
    }
    v8 = sub_189e0(a0->field_0x2c,v2,v10,1);
    *(short *)((long)v10 + 0x68) = v8;
    if (v8 == 1) {
      v2 = a0->field_0x48;
      if (v2 & 4) {
        a0->field_0x0 = v10;
        goto label_1a35f;
      }
      if (v2 & 0x200) // branch-flip
        v9 = sub_1f080(a0->field_0x2c,".");
      else {
        v9 = sub_18040(".",(v2 & 0x10) << 0xd | 0x90900);
      }
      *(int *)((long)v10 + 0x44) = v9;
      if (v9 < 0) {
        v9 = *__errno_location();
        *(unsigned short *)((long)v10 + 0x68) = 7;
        *(int *)((long)v10 + 0x40) = v9;
        a0->field_0x0 = v10;
        return v10;
      }
      *(unsigned short *)((long)v10 + 0x6a) = *(unsigned short *)((long)v10 + 0x6a) | 2;
      v8 = *(short *)((long)v10 + 0x68);
      goto label_1a34b;
    }
    a0->field_0x0 = v10;
    if (v8 != 0xb)
      return v10;
label_1a3c8:
    if (*(long *)((long)v10 + 0xa0) != 2) {
      if (*(long *)((long)v10 + 0xa0) == 1)
        return v10;
      abort();
    }
    v8 = sub_189e0(a0->field_0x2c,a0->field_0x48,v10,0);
    *(short *)((long)v10 + 0x68) = v8;
  }
  if (v8 != 1)
    return v10;
label_1a35f:
  if (!*(long *)((long)v10 + 0x58))
    a0->field_0x18 = *(long *)((long)v10 + 0x70);
  if (!sub_18870(a0,v10)) {
    v10 = NULL;
    *__errno_location() = 0xc;
  }
  return v10;
}

// Function: sub_1a750 @ 0x1a750
unsigned long sub_1a750(unsigned long a0,void *a1,unsigned int a2)
{
  if (a2 <= 4) {
    *(short *)((long)a1 + 0x6c) = (short)a2;
    return 0;
  }
  *__errno_location() = 0x16;
  return 1;
}

// Function: sub_1a780 @ 0x1a780
int * sub_1a780(struct_63 *a0,unsigned int a1) // early-return x2
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
  v2 = a0->field_0x0;
  *v7 = 0;
  if (a0->field_0x48 & 0x2000) {
    v7 = NULL;
    return v7;
  }
  if (*(short *)(v2 + 0x68) == 9) {
    v7 = *(void **)(v2 + 0x10);
    return v7;
  }
  if (*(short *)(v2 + 0x68) != 1) {
    v7 = NULL;
    return v7;
  }
  v4 = a0->field_0x8;
  while (v4) {
    v3 = *(void **)((long)v4 + 0x10);
    if (*(DIR **)((long)v4 + 0x18))
      closedir(*(DIR **)((long)v4 + 0x18));
    free(v4);
    v4 = v3;
  }
  v8 = 1;
  if (a1 == 0x1000) {
    a0->field_0x48 = a0->field_0x48 | 0x1000;
    v8 = 2;
  }
  if (((*(long *)(v2 + 0x58)) || (**(char **)(v2 + 0x30) == '/')) || (v1 = a0->field_0x48, v1 & 4)) {
    v7 = (void *)sub_18ed0(a0,v8);
    a0->field_0x8 = v7;
  }
  else {
    if (v1 & 0x200) { // branch-flip
      v6 = sub_1f080(a0->field_0x2c,".");
      v9 = v6;
    }
    else {
      v6 = sub_18040(".",(v1 & 0x10) << 0xd | 0x90900);
      v9 = v6;
    }
    if (v9 < 0) {
      a0->field_0x8 = NULL;
      v7 = NULL;
      return v7;
    }
    v7 = (void *)sub_18ed0(a0,v8);
    a0->field_0x8 = v7;
    if (a0->field_0x48 & 0x200) // branch-flip
      sub_18730(a0,v9,1);
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
    v7 = a0->field_0x8;
  }
  return v7;
}

// Function: sub_1a980 @ 0x1a980
unsigned long sub_1a980(unsigned long a0,unsigned long a1)
{
  return (a0 >> 3 | a0 << 0x3d) % a1;
}

// Function: sub_1a9a0 @ 0x1a9a0
bool sub_1a9a0(long a0,long a1)
{
  return a1 == a0;
}

// Function: sub_1a9b0 @ 0x1a9b0
unsigned long sub_1a9b0(struct_13 *a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = a0->field_0x28;
  if (v2 == (float *)0x2c420)
    return 1;
  v1 = v2[2];
  if ((((dat_2bc64 < v1) && (v1 < dat_2bc3c)) && (dat_2c434 < v2[3])) && (dat_2c440 <= *v2)) {
    v3 = *v2 + dat_2bc64;
    if (((v3 < v2[1]) && (v2[1] <= dat_29578)) && (v3 < v1))
      return 1;
  }
  a0->field_0x28 = (float *)0x2c420;
  return 0;
}

// Function: sub_1aa40 @ 0x1aa40
unsigned long sub_1aa40(struct_17 *a0,long a1,unsigned long *a2,bool a3) // early-return
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

// Function: sub_1ab50 @ 0x1ab50
unsigned long sub_1ab50(float a0,unsigned long a1,bool a2) // ternary
{
  unsigned long v1; // rax
  unsigned long v2; // rcx
  unsigned long v3; // rsi
  long v4; // rdi
  unsigned long v5;
  
  if (!a2) {
    a0 = (float)a1 / a0;
    if (dat_2c438 <= a0) {
      v1 = 0;
      return v1;
    }
    a1 = (a0 < dat_2c43c) ? (unsigned long)a0 : (long)(a0 - dat_2c43c) ^ 0x8000000000000000;
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
          if (v5 <= v3) goto label_1ac25;
          v4 += 8;
        } while (v5 % v2);
      }
    }
    else {
      v2 = 3;
label_1ac25:
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

// Function: sub_1ac90 @ 0x1ac90
unsigned long sub_1ac90(struct_18 *a0,struct_64 *a1,bool a2) // return-dupe x2
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
label_1acc6:
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
        goto label_1acc6;
      }
    }
    v6 = &v6[2];
    if (a1->field_0x8 <= v6)
      return 1;
  } while( true );
}

// Function: sub_1ae00 @ 0x1ae00
unsigned long sub_1ae00(void *a0)
{
  return *(unsigned long *)((long)a0 + 0x10);
}

// Function: sub_1ae10 @ 0x1ae10
unsigned long sub_1ae10(void *a0)
{
  return *(unsigned long *)((long)a0 + 0x18);
}

// Function: sub_1ae20 @ 0x1ae20
unsigned long sub_1ae20(void *a0)
{
  return *(unsigned long *)((long)a0 + 0x20);
}

// Function: sub_1ae30 @ 0x1ae30
unsigned long sub_1ae30(struct_64 *a0)
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

// Function: sub_1ae90 @ 0x1ae90
bool sub_1ae90(struct_64 *a0)
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
label_1aeb0:
      v2 = &v2[2];
    }
    v1 = v2[1];
    v4 += 1;
    v3 += 1;
    if (!v1) goto label_1aeb0;
    do {
      v1 = *(long *)(v1 + 8);
      v3 += 1;
    } while (v1);
    v2 = &v2[2];
  } while( true );
}

// Function: sub_1af00 @ 0x1af00
void sub_1af00(unsigned long *a0,FILE *a1)
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
      if ((long *)a0[1] <= v4) goto label_1af6d;
    }
    v5 = 1;
    for (v3 = v4[1]; v3; v3 = *(long *)(v3 + 8)) {
      v5 += 1;
    }
    if (v6 < v5)
      v6 = v5;
  }
label_1af6d:
  __fprintf_chk(a1,1,"# entries:         %lu\n",a0[4]);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v2,((double)v2 * dat_2c440) / (double)v1);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v6); // tail-call
  return;
}

// Function: sub_1b060 @ 0x1b060
long sub_1b060(struct_24 *a0,long a1) // return-dupe
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

// Function: sub_1b0d0 @ 0x1b0d0
long sub_1b0d0(struct_64 *a0)
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

// Function: sub_1b120 @ 0x1b120
long sub_1b120(struct_65 *a0,long a1)
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

// Function: sub_1b1b0 @ 0x1b1b0
void sub_1b1b0(struct_64 *a0,long a1,unsigned long a2)
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

// Function: sub_1b200 @ 0x1b200
long sub_1b200(struct_64 *a0,void *a1,unsigned long a2)
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

// Function: sub_1b280 @ 0x1b280
unsigned long sub_1b280(unsigned char *a0,unsigned long a1)
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

// Function: sub_1b2c0 @ 0x1b2c0
void sub_1b2c0(struct_66 *a0)
{
  a0->field_0x10 = 0;
  a0->field_0x0 = 0x3f80000000000000;
  a0->field_0x8 = 0x3fb4fdf43f4ccccd;
}

// Function: sub_1b2f0 @ 0x1b2f0
unsigned long * sub_1b2f0(unsigned long a0,void *a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_1a980;
  if (!a3)
    a3 = sub_1a9a0;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = (void *)0x2c420;
  v1[5] = a1;
  if (sub_1a9b0(v1)) {
    v2 = sub_1ab50(*(float *)((long)a1 + 8),a0,(unsigned char)*(char *)((long)a1 + 0x10));
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

// Function: sub_1b400 @ 0x1b400
void sub_1b400(struct_67 *a0)
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
        if (a0->field_0x8 <= v5) goto label_1b49b;
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
label_1b49b:
  a0->field_0x18 = 0;
  a0->field_0x20 = 0;
  return;
}

// Function: sub_1b4b0 @ 0x1b4b0
void sub_1b4b0(struct_15 *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = a0->field_0x0;
  v3 = a0->field_0x8;
  if ((a0->field_0x40) && (a0->field_0x20)) {
    if (v3 <= v4) goto label_1b55c;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_1b51f;
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
label_1b51f:
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
label_1b55c:
  v2 = a0->field_0x48;
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free(a0->field_0x0);
  free(a0); // tail-call
}

// Function: sub_1b5a0 @ 0x1b5a0
unsigned int sub_1b5a0(struct_20 *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_1ab50(*(float *)(a0->field_0x28 + 8),a1,(unsigned char)*(char *)(a0->field_0x28 + 0x10));
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
  v1 = sub_1ac90(&v2,a0,0);
  if (!(char)v1) {
    a0->field_0x48 = v11;
    if ((sub_1ac90(a0,&v2,1)) && (sub_1ac90(a0,&v2,0))) {
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

// Function: sub_1b720 @ 0x1b720
long * sub_1b720(void *a0,long a1,long *a2) // return-dupe, ternary
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
  v2 = (long)sub_1aa40(a0,a1,&v3,0);
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
    if ((long)v2 > -1) goto label_1b7af;
label_1b836:
    v5 = (float)v2;
  }
  else {
    v2 = *(long **)((long)a0 + 0x10);
    v6 = (float)v4;
    if (0 > (long)v2) goto label_1b836;
label_1b7af:
    v5 = (float)(long)v2;
  }
  if (*(float *)(*(long *)((long)a0 + 0x28) + 8) * v5 < v6) {
    sub_1a9b0(a0);
    v1 = *(long *)((long)a0 + 0x28);
    if (v5 * *(float *)(v1 + 8) < v6) {
      v5 = v5 * *(float *)(v1 + 0xc);
      if (!*(char *)(v1 + 0x10))
        v5 = v5 * *(float *)(v1 + 8);
      if (dat_2c438 <= v5) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v4 = (dat_2c43c <= v5) ? (long)(v5 - dat_2c43c) ^ 0x8000000000000000 : (unsigned long)v5;
      if (!sub_1b5a0(a0,v4)) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v2 = (long)sub_1aa40(a0,a1,&v3,0);
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

// Function: sub_1b950 @ 0x1b950
unsigned long sub_1b950(void *a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_1b720(a0,a1,&v3);
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

// Function: sub_1b9b0 @ 0x1b9b0
long sub_1b9b0(void *a0,long a1) // ternary x2
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
  
  v5 = sub_1aa40(a0,a1,&v7,1);
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
      sub_1a9b0(a0);
      v2 = *(float **)((long)a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_2c43c <= v8) ? (long)(v8 - dat_2c43c) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_1b5a0(a0,v6)) {
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

// Function: sub_1bb40 @ 0x1bb40
long sub_1bb40(void *a0)
{
  return sub_1b9b0(a0); // tail-call
}

// Function: sub_1bb50 @ 0x1bb50
char * sub_1bb50(unsigned long a0,char *a1,unsigned int a2,unsigned long a3,unsigned long a4) // ternary
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
        if (a2 & 0x10) goto label_1bdcc;
label_1c1db:
        v21 = -1;
label_1bddf:
        if (v33 != 1) goto label_1bdef;
        if (5 < (int)((((unsigned int)v15 & 1) + v25 != 0) + v20)) goto label_1bdff;
        goto label_1be18;
      }
    }
label_1bc34:
    v30 = (long double)(long)a3;
    if ((long)a3 < 0)
      v30 = v30 + (long double)dat_2c438;
    if (0 <= (long)a4) { // branch-flip
      v30 = v30 / (long double)(long)a4;
      if ((long)a0 > -1) goto label_1bc71;
label_1bf20:
      v30 = ((long double)(long)a0 + (long double)dat_2c438) * v30;
      if (!(a2 & 0x10)) goto label_1bf68;
label_1bc7e:
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
      v30 = v30 / ((long double)(long)a4 + (long double)dat_2c438);
      if (0 > (long)a0) goto label_1bf20;
label_1bc71:
      v30 = (long double)(long)a0 * v30;
      if (a2 & 0x10) goto label_1bc7e;
label_1bf68:
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
    if ((!a3) || (v17 = a4 / a3, a4 % a3)) goto label_1bc34;
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
    if (!(a2 & 0x10)) goto label_1c1db;
label_1bdcc:
    v20 = (unsigned int)v16;
    v15 = SUB168(v8._0_16_,0);
    v22 = (unsigned long)v11;
    v21 = 0;
    v17 = v15;
    if (v15 < v22) goto label_1bddf;
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
      if (v21 == 8) goto label_1bddf;
    }
    if (10 <= v15) goto label_1bddf;
    v17 = v15;
    if (v33 != 1) { // branch-flip
      if ((!v33) && (v25)) goto label_1c567;
label_1c5dc:
      if (!v20) goto label_1c5e4;
      v10 = (char)v16 + '0';
    }
    else {
      if (3 > (v20 & 1) + v25) goto label_1c5dc;
label_1c567:
      if (v20 != 9) // branch-flip
        v10 = (char)v16 + '1';
      else {
        v17 = v15 + 1;
        if (v15 == 9) goto label_1c5aa;
        v25 = 0;
        v15 = v17;
label_1c5e4:
        if (a2 & 8) goto label_1c5ac;
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
label_1c5aa:
    v25 = 0;
    v15 = v17;
label_1c5ac:
    v20 = 0;
    if (v33 == 1) goto label_1be18;
label_1bdef:
    if (v33) { // branch-flip
label_1be18:
      v29 = v24;
      do {
        v29 = &v29[-1];
        *v29 = (char)v15 + (char)(v15 / 10) * '\xf6' + '0';
        v3 = 10 <= v15;
        v15 /= 10;
      } while (v3);
    }
    else {
      if (1 > (int)(v25 + v20)) goto label_1be18;
label_1bdff:
      v15 += 1;
      if (((!(a2 & 0x10)) || (v11 != v15)) || (v21 == 8)) goto label_1be18;
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
  if (!(a2 & 0x80)) goto label_1c07f;
  if (v21 != -1) { // branch-flip
label_1c06c:
    if (!(a2 & 0x100) && !v21) goto label_1c07f;
    if (a2 & 0x40) {
label_1c229:
      v32 = &a1[0x288];
      a1[0x287] = ' ';
    }
    if (v21) goto label_1c248;
    v28 = v32;
    if (!(a2 & 0x100)) goto label_1c07f;
  }
  else {
    if (a4 < 2) {
      v21 = 0;
      goto label_1c06c;
    }
    v21 = 1;
    v14 = 1;
    do {
      v14 *= v11;
      if (a4 <= v14) break;
      v21 += 1;
    } while (v21 != 8);
    if (a2 & 0x40) goto label_1c229;
label_1c248:
    v28 = &v32[1];
    if ((v19) || (v21 != 1)) {
      *v32 = *(char *)((long)v21 + 0x2c490);
      if (!(a2 & 0x100)) {
label_1c53b:
        v32 = v28;
        goto label_1c07f;
      }
      if (v19) {
        v32[1] = 'i';
        v28 = &v32[2];
      }
    }
    else {
      *v32 = 'k';
      if (!(a2 & 0x100)) goto label_1c53b;
    }
  }
  *v28 = 'B';
  v32 = &v28[1];
label_1c07f:
  *v32 = '\0';
  return v29;
}

// Function: sub_1c750 @ 0x1c750
unsigned long sub_1c750(unsigned char *a0,unsigned int *a1,unsigned long *a2) // early-return x4
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
  v3 = sub_26980(a0,(long *)0x363d0,(void *)0x2c488,4);
  if (0 <= v3) {
    *a2 = 1;
    *a1 = v7 | *(unsigned int *)((long)v3 * 4 + 0x2c488);
    return 0;
  }
  v4 = sub_26490(a0,&v6,0,a2,"eEgGkKmMpPtTyYzZ0");
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

// Function: sub_1c940 @ 0x1c940
void sub_1c940(struct_14 *a0,unsigned int a1)
{
  *(unsigned long *)a0->field_0x14 = 0;
  a0->field_0x1c = 1;
  a0->field_0x0 = a1;
  a0->field_0x4 = a1;
  a0->field_0x8 = a1;
  a0->field_0xc = a1;
  a0->field_0x10 = a1;
}

// Function: sub_1c960 @ 0x1c960
char sub_1c960(void *a0)
{
  return *(char *)((long)a0 + 0x1c);
}

// Function: sub_1c970 @ 0x1c970
unsigned int sub_1c970(void *a0,unsigned int a1)
{
  unsigned int v1;
  unsigned int v2; // eax
  unsigned int v3; // edx
  
  v2 = (unsigned int)(*(unsigned char *)((long)a0 + 0x1c) ^ 1);
  v3 = *(int *)((long)a0 + 0x14) + v2 & 3;
  v1 = *(unsigned int *)((long)a0 + (unsigned long)v3 * 4);
  *(unsigned int *)((long)a0 + (unsigned long)v3 * 4) = a1;
  *(unsigned int *)((long)a0 + 0x14) = v3;
  if (*(unsigned int *)((long)a0 + 0x18) == v3)
    *(unsigned int *)((long)a0 + 0x18) = v2 + *(unsigned int *)((long)a0 + 0x18) & 3;
  *(char *)((long)a0 + 0x1c) = 0;
  return v1;
}

// Function: sub_1c9b0 @ 0x1c9b0
unsigned int sub_1c9b0(void *a0)
{
  unsigned int v1;
  unsigned int v2;
  
  if (*(char *)((long)a0 + 0x1c))
    abort(); // no-return
  v1 = *(unsigned int *)((long)a0 + 0x14);
  v2 = *(unsigned int *)((long)a0 + (unsigned long)v1 * 4);
  *(unsigned int *)((long)a0 + (unsigned long)v1 * 4) = *(unsigned int *)((long)a0 + 0x10);
  if (v1 != *(unsigned int *)((long)a0 + 0x18)) {
    *(unsigned int *)((long)a0 + 0x14) = v1 + 3 & 3;
    return v2;
  }
  *(char *)((long)a0 + 0x1c) = 1;
  return v2;
}

// Function: sub_1c9f0 @ 0x1c9f0
unsigned int * sub_1c9f0(unsigned int a0) // early-return
{
  passwd *v1; // rax
  unsigned int *v2;
  char *v3;
  
  for (v2 = dat_38ec8; v2; v2 = *(unsigned int **)&v2[2]) {
    if (*v2 == a0) goto label_1ca1d;
  }
  v3 = "";
  v1 = getpwuid(a0);
  if (v1)
    v3 = *(char **)v1;
  v2 = (unsigned int *)sub_25a50(strlen(v3) + 0x18 & 0xfffffffffffffff8);
  *v2 = a0;
  strcpy((char *)&v2[4],v3);
  *(unsigned int **)&v2[2] = dat_38ec8;
  dat_38ec8 = v2;
label_1ca1d:
  if ((char)v2[4])
    return &v2[4];
  return NULL;
}

// Function: sub_1ca80 @ 0x1ca80
unsigned int * sub_1ca80(char *a0)
{
  char v1;
  passwd *v2; // rax
  unsigned int *v3;
  long v4;
  
  if (dat_38ec8) {
    v1 = *a0;
    v3 = dat_38ec8;
    do {
      if ((*(char *)&v3[4] == v1) && (!strcmp((char *)&v3[4],a0)))
        return v3;
      v3 = *(unsigned int **)&v3[2];
    } while (v3);
  }
  if (dat_38ec0) {
    v1 = *a0;
    v4 = (long)dat_38ec0;
    do {
      if ((*(char *)(v4 + 0x10) == v1) && (!strcmp((char *)(v4 + 0x10),a0)))
        return NULL;
      v4 = *(long *)(v4 + 8);
    } while (v4);
  }
  v2 = getpwnam(a0);
  v3 = (unsigned int *)sub_25a50(strlen(a0) + 0x18 & 0xfffffffffffffff8);
  strcpy((char *)&v3[4],a0);
  v4 = (long)dat_38ec0;
  if (v2) {
    *v3 = *(unsigned int *)&v2->field_0x10;
    *(unsigned int **)&v3[2] = dat_38ec8;
    dat_38ec8 = v3;
    return v3;
  }
  dat_38ec0 = v3;
  *(long *)&v3[2] = v4;
  return NULL;
}

// Function: sub_1cb90 @ 0x1cb90
unsigned int * sub_1cb90(unsigned int a0) // early-return
{
  group *v1; // rax
  unsigned int *v2;
  char *v3;
  
  for (v2 = dat_38eb8; v2; v2 = *(unsigned int **)&v2[2]) {
    if (*v2 == a0) goto label_1cbbd;
  }
  v3 = "";
  v1 = getgrgid(a0);
  if (v1)
    v3 = *(char **)v1;
  v2 = (unsigned int *)sub_25a50(strlen(v3) + 0x18 & 0xfffffffffffffff8);
  *v2 = a0;
  strcpy((char *)&v2[4],v3);
  *(unsigned int **)&v2[2] = dat_38eb8;
  dat_38eb8 = v2;
label_1cbbd:
  if ((char)v2[4])
    return &v2[4];
  return NULL;
}

// Function: sub_1cc20 @ 0x1cc20
unsigned int * sub_1cc20(char *a0)
{
  char v1;
  group *v2; // rax
  unsigned int *v3;
  long v4;
  
  if (dat_38eb8) {
    v1 = *a0;
    v3 = dat_38eb8;
    do {
      if ((*(char *)&v3[4] == v1) && (!strcmp((char *)&v3[4],a0)))
        return v3;
      v3 = *(unsigned int **)&v3[2];
    } while (v3);
  }
  if (dat_38eb0) {
    v1 = *a0;
    v4 = (long)dat_38eb0;
    do {
      if ((*(char *)(v4 + 0x10) == v1) && (!strcmp((char *)(v4 + 0x10),a0)))
        return NULL;
      v4 = *(long *)(v4 + 8);
    } while (v4);
  }
  v2 = getgrnam(a0);
  v3 = (unsigned int *)sub_25a50(strlen(a0) + 0x18 & 0xfffffffffffffff8);
  strcpy((char *)&v3[4],a0);
  v4 = (long)dat_38eb0;
  if (v2) {
    *v3 = *(unsigned int *)&v2->field_0x10;
    *(unsigned int **)&v3[2] = dat_38eb8;
    dat_38eb8 = v3;
    return v3;
  }
  dat_38eb0 = v3;
  *(long *)&v3[2] = v4;
  return NULL;
}

// Function: sub_1cd30 @ 0x1cd30
unsigned long sub_1cd30(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_274f0(0))) {
    v1 = 1;
    *a0 = (unsigned int)(unsigned char)*a1;
  }
  return v1;
}

// Function: sub_1cdc0 @ 0x1cdc0
unsigned long sub_1cdc0(char *a0,long a1,unsigned long a2,unsigned long *a3)
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
      v9 = sub_275c0(v9);
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
              goto label_1ceb8;
            }
            v3 -= *(long *)(v9 + v3 * 8);
            v2 = *(char *)(a1 + v3);
          }
          v3 += 1;
          *(unsigned long *)(v9 + v1 * 8) = v1 - v3;
label_1ceb8:
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
      *(unsigned long *)&v8[-8] = 0x1cf0e;
      sub_27610(v9);
      return 1;
    }
  }
  return 0;
}

// Function: sub_1cf60 @ 0x1cf60
void sub_1cf60(struct_33 *a0)
{
  unsigned char v1;
  unsigned char *v2;
  unsigned long v3; // rax
  long v4; // rax
  
  if (!a0->field_0x0) {
    v1 = *a0->field_0x10;
    if (*(unsigned int *)((unsigned long)(v1 >> 5) * 4 + 0x2d9e0) >> (v1 & 0x1f) & 1) {
      a0->field_0x18 = 1;
      v1 = *a0->field_0x10;
      a0->field_0x20 = '\x01';
      a0->field_0x24 = (int)(char)v1;
      goto label_1cfa1;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x1[3]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    a0->field_0x0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v2 = a0->field_0x10;
  v4 = sub_1cd30(&a0->field_0x24,v2,sub_24ce0(v2,v3),(mbstate_t *)&a0->field_0x1[3]);
  a0->field_0x18 = v4;
  if (v4 == -1) {
    a0->field_0x18 = 1;
    a0->field_0x20 = '\0';
    a0->field_0xc = '\x01';
    return;
  }
  if (v4 == -2) {
    v3 = strlen((char *)a0->field_0x10);
    a0->field_0x20 = '\0';
    a0->field_0x18 = v3;
    a0->field_0xc = '\x01';
    return;
  }
  if (!v4) {
    a0->field_0x18 = 1;
    if (*a0->field_0x10)
      __assert_fail("*iter->cur.ptr == \'\\0\'","mbuiter.h",0xab,"mbuiter_multi_next"); // no-return
    if (a0->field_0x24)
      __assert_fail("iter->cur.wc == 0","mbuiter.h",0xac,"mbuiter_multi_next"); // no-return
  }
  a0->field_0x20 = '\x01';
  if (mbsinit((mbstate_t *)&a0->field_0x1[3])) {
    a0->field_0x0 = '\0';
    a0->field_0xc = '\x01';
    return;
  }
label_1cfa1:
  a0->field_0xc = '\x01';
  return;
}

// Function: sub_1d0d0 @ 0x1d0d0
char sub_1d0d0(void *a0,char *a1,unsigned long *a2)
{
  char v1;
  void *v10; // stack - 0xf8
  char v11 [32];
  char v12 [4]; // stack - 0x88
  char v13 [4]; // stack - 0xc8
  void **v14;
  char *v15; // rsp
  void **v16; // rsp
  void **v17;
  char *v18;
  unsigned long v19;
  void *v2;
  unsigned long *v20;
  unsigned long *v21; // r13
  long v22;
  char *v23;
  unsigned long *v24; // stack - 0xf0
  char v25; // stack - 0xe1
  void *v26; // stack - 0xe0
  char *v27; // stack - 0xd8
  char *v28; // stack - 0xd0
  unsigned long v29; // stack - 0xc4
  void *v3;
  char v30; // stack - 0xbc
  void *v31; // stack - 0xb8
  long v32; // stack - 0xb0
  char v33; // stack - 0xa8
  int v34; // stack - 0xa4
  unsigned long v35; // stack - 0x84
  char v36; // stack - 0x7c
  char *v37; // stack - 0x78
  unsigned long v38; // stack - 0x70
  char v39; // stack - 0x68
  char v4 [16];
  int v40; // stack - 0x64
  unsigned long *v5;
  void *v6;
  char v7; // al
  unsigned int v8; // eax
  unsigned long *v9;
  
  v16 = &v10;
  v14 = &v10;
  v17 = &v10;
  v10 = a0;
  v24 = a2;
  v9 = (void *)sub_276f0(a1);
  v26 = v9;
  v4 = ZEXT816(v9);
  v9 = SUB168(ZEXT816(0x38) * v4,0);
  if ((0 <= (long)v9) && (!SUB168(ZEXT816(0x38) * v4,8))) {
    v19 = (long)v26 * 0x38;
    if (0xfa1 <= v19) { // branch-flip
      v9 = (unsigned long *)sub_275c0(v19);
      v20 = v9;
      v6 = v26;
    }
    else {
      v17 = &v10;
      while (v16 != (void **)((long)&v10 - (v19 + 0x36 & 0xfffffffffffff000))) {
        v14 = (char *)((long)v17 + -0x1000);
        v15 = (char *)((long)v17 + -0x1000);
        *(unsigned long *)((long)v17 + -8) = *(unsigned long *)((long)v17 + -8);
        v16 = (char *)((long)v17 + -0x1000);
        v17 = (void **)v15;
      }
      v8 = (unsigned int)(v19 + 0x36) & 0xff0;
      v9 = (unsigned long)(unsigned long)v8;
      v17 = (char *)((long)v14 + -(long)v9);
      if (v9)
        *(unsigned long *)((long)v14 + -8) = *(unsigned long *)((long)v14 + -8);
      v20 = (unsigned long *)(((unsigned long)((long)v14 + -(long)v9 + 0xf) & 0xfffffffffffffff0) + 0x1f & 0xffffffffffffffe0);
      v6 = v26;
    }
    if (v20) {
      v9 = &v20[3];
      v12[0] = 0;
      v28 = v12;
      v35 = 0;
      v26 = v6;
      v37 = a1;
      do {
        v36 = 0;
        *(unsigned long *)((long)v17 + -8) = 0x1d26f;
        sub_1cf60(v28);
        v19 = v38;
        v25 = v39;
        if (v39) { // branch-flip
          if (!v40) goto label_1d2a0;
          if (v37 != v11) { // branch-flip
            v9[-3] = v37;
            v9[-2] = v38;
            *(char *)&v9[-1] = 1;
          }
          else {
            v27 = v37;
            *(unsigned long *)((long)v17 + -8) = 0x1d5ab;
            memcpy(v9,v11,v38);
            v9[-3] = v9;
            v9[-2] = v19;
            *(char *)&v9[-1] = v25;
            v37 = v27;
          }
          *(int *)((long)v9 + -4) = v40;
        }
        else if (v37 != v11) { // branch-flip
          v9[-3] = v37;
          v9[-2] = v38;
          *(char *)&v9[-1] = 0;
        }
        else {
          v27 = v37;
          *(unsigned long *)((long)v17 + -8) = 0x1d573;
          memcpy(v9,v11,v38);
          v9[-3] = v9;
          v9[-2] = v19;
          *(char *)&v9[-1] = 0;
          v37 = v27;
        }
        v37 = &v37[v19];
        v9 = &v9[6];
      } while( true );
    }
  }
  v7 = '\0';
  return v7;
label_1d530:
  v20[(long)((long)v6 * 6 + (long)v27)] = v27;
label_1d33b:
  v27 = (void *)((long)v27 + 1);
  v5 = v21;
  if (v26 == v27) goto label_1d35b;
  goto label_1d2d0;
label_1d4de:
  v27 = v23;
  if ((v33) && (!v34)) {
label_1d58c:
    *(unsigned long *)((long)v17 + -8) = 0x1d591; // no-return
    abort();
  }
  v30 = '\0';
  v31 = (void *)((long)v31 + v32);
  v22 -= 1;
  if (!v22) goto label_1d520;
label_1d509:
  *(unsigned long *)((long)v17 + -8) = 0x1d511;
  sub_1cf60(v13);
  v23 = v27;
  goto label_1d4de;
label_1d520:
  v1 = v36;
  goto label_1d3b0;
label_1d2a0:
  v22 = 0;
  v20[(long)v6 * 6 + 1] = 1;
  v27 = (void *)0x2;
  v5 = v20;
  if ((void *)0x3 <= v26) {
label_1d2d0:
    v21 = &v5[6];
    v1 = *(char *)&v5[8];
    do {
      v9 = &v20[v22 * 6];
      if ((v1) && (*(char *)&v9[2])) {
        if (*(int *)((long)v5 + 0x44) == *(int *)((long)v9 + 0x14)) {
label_1d326:
          v22 += 1;
          v9 = (long)((long)v27 - v22);
          v20[(long)((long)v6 * 6 + (long)v27)] = v9;
          goto label_1d33b;
        }
      }
      else {
        v19 = v5[7];
        if (v19 == v9[1]) {
          v2 = (void *)*v9;
          v3 = (void *)*v21;
          *(unsigned long *)((long)v17 + -8) = 0x1d322;
          v8 = memcmp(v3,v2,v19);
          if (!v8) goto label_1d326;
        }
      }
      if (!v22) goto label_1d530;
      v22 -= v20[(long)v6 * 6 + v22];
    } while( true );
  }
label_1d35b:
  v12[0] = 0;
  v23 = NULL;
  v13[0] = 0;
  *v24 = 0;
  v29 = 0;
  v30 = '\0';
  v35 = 0;
  v36 = '\0';
  v31 = v10;
  v37 = v10;
  v7 = 0;
  v1 = v7;
label_1d3b0:
  v7 = v1;
  if (!v7) goto label_1d3ff;
  if (!v39) goto label_1d411;
label_1d3ba:
  if (v40) {
    v9 = &v20[(long)v23 * 6];
    if (!*(char *)&v9[2]) goto label_1d41c;
    if (*(int *)((long)v9 + 0x14) != v40) goto label_1d426;
    v18 = v37;
    v19 = v38;
label_1d3e3:
    v37 = (void *)((long)v18 + v19);
    v36 = '\0';
    v23 = (void *)((long)v23 + 1);
    if (v26 != v23) {
label_1d3ff:
      do {
        *(unsigned long *)((long)v17 + -8) = 0x1d40b;
        sub_1cf60(v28);
        if (v39) goto label_1d3ba;
label_1d411:
        v9 = &v20[(long)v23 * 6];
label_1d41c:
        v19 = v38;
        if (v9[1] == v38) {
          v2 = (void *)*v9;
          v27 = v37;
          *(unsigned long *)((long)v17 + -8) = 0x1d496;
          v8 = memcmp(v2,v37,v38);
          v18 = v27;
          if (!v8) goto label_1d3e3;
        }
label_1d426:
        if (v23) {
          v22 = v20[(long)((long)v6 * 6 + (long)v23)];
          v9 = (void *)((long)v23 - v22);
          v1 = v36;
          v27 = (char *)v9;
          if (!v22) goto label_1d3b0;
          v23 = (char *)v9;
          if (!v30) goto label_1d509;
          goto label_1d4de;
        }
        if (!v30) {
          *(unsigned long *)((long)v17 + -8) = 0x1d440;
          sub_1cf60(v13);
        }
        if ((v33) && (!v34)) goto label_1d58c;
        v30 = '\0';
        v31 = (void *)((long)v31 + v32);
        v36 = '\0';
        v37 = (void *)((long)v37 + v38);
      } while( true );
    }
    *v24 = (unsigned long)v31;
  }
  *(unsigned long *)((long)v17 + -8) = 0x1d559;
  sub_27610(v20);
  v7 = v25;
  return v7;
}

// Function: sub_1d5d0 @ 0x1d5d0
char * sub_1d5d0(char *a0,char *a1) // early-return, return-dupe
{
  char *v1;
  char *v10; // stack - 0x190
  char v11 [4]; // stack - 0x188
  char v12 [4]; // stack - 0x88
  unsigned int v13; // stack - 0x108
  unsigned long v14; // stack - 0xc8
  char v15 [4]; // stack - 0x148
  unsigned long v16;
  long v17;
  unsigned long v18; // stack - 0x1e0
  unsigned char v19; // stack - 0x1d1
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
  char v43 [5]; // stack - 0xc0
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
    sub_1cf60(v11);
    if ((!v25) || (v26)) {
      v19 = 1;
      v15[0] = 0;
      v27 = 0;
      v28 = '\0';
      v13 &= 0xffffff00;
      v34 = 0;
      v33 = 0;
      v18 = 0;
      v20 = 0;
      v8 = 0;
      v29 = a1;
      v36 = a0;
      do {
        v35 &= 0xffffff00;
        sub_1cf60(&v13);
        if ((char)v38) { // branch-flip
          if (!v39)
            return NULL;
          v4 = 10 <= v18 & v19;
          if (v4) {
            if (v18 * 5 <= v8) { // branch-flip
label_1db50:
              v17 = v8 - v20;
              if (v17) { // branch-flip
                if (!v28) goto label_1db9d;
                while ((!v31 || (v32))) {
                  v28 = '\0';
                  v29 = &v29[v30];
                  v17 -= 1;
                  if (!v17) goto label_1dbd0;
label_1db9d:
                  sub_1cf60(v15);
                }
                if (v28) goto label_1dc60;
label_1dbd0:
                sub_1cf60(v15);
              }
              else if (!v28) goto label_1dbd0;
              v19 = v31;
              if (v31) { // branch-flip
                if (!v32) {
label_1dc60:
                  v19 = sub_1d0d0(a0,a1,&v10);
                  if (v19)
                    return v10;
                }
              }
              else {
                v19 = 1;
              }
              v20 = v8;
              if (!(char)v38) goto label_1d926;
            }
            else {
              v19 = v4;
            }
          }
          v9 = v8 + 1;
          if (!v25) goto label_1d926;
          if (v39 == v26) goto label_1d966;
label_1d8b8:
          v6 = v36;
          v16 = v37;
        }
        else {
          v4 = 10 <= v18 & v19;
          if (v4) { // branch-flip
            if (v18 * 5 <= v8) goto label_1db50;
            v19 = v4;
          }
          else {
          }
label_1d926:
          v16 = v37;
          v6 = v36;
          v9 = v8 + 1;
          if (v37 == v24) {
            if (!memcmp(v36,v23,v37)) {
label_1d966:
              v9 = v8 + 1;
              v14 = CONCAT44(v33,v13);
              v12[0] = 0;
              v47 = CONCAT44(v39,v38);
              v52 = 0;
              v46 = v37;
              v45 = &v36[v37];
              v44 = (undefined3)(v35 >> 8);
              v43[4] = 0;
              v43._0_4_ = v34;
              v51 = 0;
              v48 = v40;
              v49 = v41;
              v50 = v42;
              v53 = a1;
              sub_1cf60(v12);
              if ((v55) && (!v56))
                abort(); // no-return
              v53 = &v53[v54];
              do {
                v9 += 1;
                v52 = 0;
                sub_1cf60(v12);
                if ((v55) && (!v56))
                  return v36;
                if (!v43[4])
                  sub_1cf60(&v14);
                v16 = v54;
                v1 = v53;
                v8 = v46;
                v6 = v45;
                if ((char)v47) { // branch-flip
                  if (!v47._4_4_)
                    return NULL;
                  if (!v55) goto label_1dade;
                  if (v47._4_4_ != v56) goto label_1d8b8;
                }
                else {
label_1dade:
                  if ((v46 != v54) || (memcmp(v45,v53,v46))) goto label_1d8b8;
                }
                v45 = &v6[v8];
                v43[4] = 0;
                v53 = &v1[v16];
              } while( true );
            }
          }
        }
        v18 += 1;
        v36 = &v6[v16];
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
      v16 = 0;
      v9 = 0;
      v8 = 0;
      v5 = 1;
      v6 = a1;
      do {
        v16 += 1;
        v7 = v9 + 1;
        v1 = &a0[1];
        if (v2 == v3) {
          v3 = a1[1];
          if (!v3)
            return a0;
          while( true ) {
            v17 = v7 - v9;
            if (!a0[v17])
              return NULL;
            v7 += 1;
            if (a0[v17] != v3) break;
            v3 = a1[v7 - v9];
            if (!v3)
              return a0;
          }
        }
        v9 = v7;
        if (!*v1)
          return NULL;
        v4 = 10 <= v16 & (unsigned char)v5;
        if (v4) {
          if (v16 * 5 <= v9) { // branch-flip
            if (v6) {
              v6 = &v6[strnlen(v6,v9 - v8)];
              v8 = v9;
              if (*v6) goto label_1d6fd;
            }
            v5 = sub_1cdc0(v1,a1,strlen(a1),&v10);
            if ((char)v5)
              return v10;
            v6 = NULL;
          }
          else {
label_1d6fd:
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

// Function: sub_1dca0 @ 0x1dca0
int sub_1dca0(char *a0,long a1,unsigned int a2)
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
          if (0x1e <= (unsigned char)(v2 + 0x9fU)) goto label_1dd24;
label_1ddd3:
          a0 = &a0[1];
          v10 += 1;
        }
        else {
          if ('@' < v2) goto label_1ddd3;
          if ('#' < v2) {
            if (0x1b <= (unsigned char)(v2 - 0x25U)) goto label_1dd24;
            goto label_1ddd3;
          }
          if ('\x1f' < v2) goto label_1ddd3;
label_1dd24:
          v8 = 0;
          do {
            v5 = sub_1cd30(&v9,a0,(long)v1 - (long)a0,&v8);
            if (v5 == -1) {
              if (a2 & 1)
                return -1;
              goto label_1ddd3;
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
label_1de59:
          if (v10 == 0x7fffffff)
            return 0x7fffffff;
          v10 += 1;
        }
        else {
          if (a2 & 2)
            return -1;
          if (!(v3 & 2)) goto label_1de59;
        }
      } while (v1 != a0);
    }
  }
  return v10;
}

// Function: sub_1dec0 @ 0x1dec0
void sub_1dec0(char *a0,unsigned int a1)
{
  sub_1dca0(a0,strlen(a0),a1); // tail-call
}

// Function: sub_1def0 @ 0x1def0
unsigned short * sub_1def0(unsigned char *a0)
{
  char v1;
  long v10;
  unsigned short *v11; // rdi
  unsigned int v12;
  long v13;
  bool v14;
  unsigned char v2;
  unsigned int v3; // eax
  unsigned short *v4; // rax
  unsigned int v5;
  unsigned int v6;
  unsigned char *v7;
  char v8; // bl
  unsigned long v9;
  
  v2 = *a0;
  v5 = (unsigned int)(char)v2;
  if ((unsigned char)(v2 - 0x30) <= 7) {
    v12 = 0;
    v7 = a0;
    do {
      v12 = (v5 - 0x30) + v12 * 8;
      v7 = &v7[1];
      if (0x1000 <= v12)
        return NULL;
      v2 = *v7;
      v5 = (unsigned int)(char)v2;
    } while ((unsigned char)(v2 - 0x30) <= 7);
    if (v2)
      return NULL;
    v5 = v12 & 0xc00 | 0x3ff;
    if (5 <= (long)v7 - (long)a0)
      v5 = 0xfff;
    v4 = (unsigned short *)sub_25a50(0x20);
    *v4 = 0x13d;
    *(unsigned int *)&v4[2] = 0xfff;
    *(unsigned int *)&v4[4] = v12;
    *(unsigned int *)&v4[6] = v5;
    *(char *)((long)v4 + 0x11) = 0;
    return v4;
  }
  v9 = 1;
  v7 = a0;
  while (v2) {
    if ((unsigned char)v5 <= 0x3d)
      v9 = (v9 + 1) - (unsigned long)((0x2000280000000000U >> ((unsigned char)v5 & 0x3f) & 1) == 0);
    v2 = v7[1];
    v5 = (unsigned int)v2;
    v7 = &v7[1];
  }
  v4 = (unsigned short *)sub_25b90(v9,0x10);
  v10 = 0;
label_1df5e:
  v2 = *a0;
  v12 = 0;
  v1 = v2 + 0x99;
  v14 = v2 == 0x67;
  v5 = 0;
  if (v14) goto label_1e09d;
  do {
    v12 = (unsigned int)v2;
    if (v14 || SBORROW1(v2,'g') != v1 < '\0') {
      if (v2 != 0x61) goto label_1df7e;
      v12 = 0xfff;
      a0 = &a0[1];
    }
    else if (v2 != 0x6f) { // branch-flip
      if (v2 != 0x75) {
        free(v4); // return-dupe
        return NULL;
      }
      v12 = v5 | 0x9c0;
      a0 = &a0[1];
    }
    else {
      v12 = v5 | 0x207;
      a0 = &a0[1];
    }
    while( true ) {
      v2 = *a0;
      v1 = v2 + 0x99;
      v14 = v1 == '\0';
      v5 = v12;
      if (!v14) break;
label_1e09d:
      v12 |= 0x438;
      a0 = &a0[1];
    }
  } while( true );
label_1df7e:
  if (((char)v2 <= 'a') && (((v2 & 0xef) == 0x2d || (v2 == 0x2b)))) {
    v2 = a0[1];
    v13 = v10 + 1;
    v11 = &v4[v10 * 8];
    v1 = v2 + 0x91;
    v14 = 0;
    v7 = a0;
    if (v2 == 0x6f) goto label_1e020;
label_1dfc0:
    v3 = (unsigned int)(char)v2;
    a0 = &v7[1];
    if (v14 || SBORROW1(v2,'o') != v1 < '\0') {
      if ('8' <= (char)v2) { // branch-flip
        if (v2 != 0x67) goto label_1e0b0;
        v3 = (unsigned int)v7[2];
        a0 = &v7[2];
        v6 = 0x38;
        v8 = 3;
      }
      else {
        if ('0' <= (char)v2) {
          v6 = 0;
          goto label_1e0da;
        }
        v8 = 1;
        v6 = 0;
      }
    }
    else {
      if (v2 != 0x75) {
label_1e0b0:
        v8 = 1;
        v6 = 0;
        do {
          switch(v3 - 0x58 & 0xff) {
            case 0:
              v8 = 2;
              break;
            default:
              goto label_1dfd9;
            case 0x1a:
              v6 |= 0x124;
              break;
            case 0x1b:
              v6 |= 0xc00;
              break;
            case 0x1c:
              v6 |= 0x200;
              break;
            case 0x1f:
              v6 |= 0x92;
              break;
            case 0x20:
              v6 |= 0x49;
            
          }
          a0 = &a0[1];
          v3 = (unsigned int)*a0;
        } while( true );
      }
      v3 = (unsigned int)v7[2];
      a0 = &v7[2];
      v6 = 0x1c0;
      v8 = 3;
    }
label_1dfd9:
    do {
      v2 = (unsigned char)v3;
      *(char *)v11 = (char)v12;
      *(char *)((long)v11 + 1) = v8;
      *(unsigned int *)&v11[2] = v5;
      *(unsigned int *)&v11[4] = v6;
      if (v5)
        v6 &= v5;
      v10 = v13;
      v12 = v3;
label_1dff4:
      *(unsigned int *)&v11[6] = v6;
      v13 = v10 + 1;
      v11 = &v11[8];
      if (((v2 & 0xef) != 0x2d) && (v2 != 0x2b)) {
        if (v2 != 0x2c) {
          if (!v2) {
            *(char *)((long)v4 + v10 * 0x10 + 1) = 0;
            return v4;
          }
          break;
        }
        a0 = &a0[1];
        goto label_1df5e;
      }
      v2 = a0[1];
      v1 = v2 + 0x91;
      v14 = v1 == '\0';
      v7 = a0;
      if (!v14) goto label_1dfc0;
label_1e020:
      v3 = (unsigned int)a0[2];
      a0 = &a0[2];
      v6 = 7;
      v8 = 3;
    } while( true );
  }
  free(v4);
  return NULL;
  while( true ) {
    v2 = *a0;
    v3 = (unsigned int)(char)v2;
    if (8 <= (unsigned char)(v2 - 0x30)) break;
label_1e0da:
    v6 = (v3 - 0x30) + v6 * 8;
    a0 = &a0[1];
    if (0xfff < v6) {
      free(v4);
      return NULL;
    }
  }
  if ((!v5) && ((!v2 || (v2 == 0x2c)))) {
    *(char *)v11 = (char)v12;
    v5 = 0xfff;
    *(unsigned int *)&v11[4] = v6;
    v6 = 0xfff;
    *(char *)((long)v11 + 1) = 1;
    *(unsigned int *)&v11[2] = 0xfff;
    v10 = v13;
    v12 = v3;
    goto label_1dff4;
  }
label_1df80:
  free(v4);
  return NULL;
}

// Function: sub_1e290 @ 0x1e290
struct_68 * sub_1e290(char *a0) // early-return
{
  struct_68 *v1; // rax
  stat v2; // stack - 0xa8
  
  if (stat(a0,&v2))
    return NULL;
  v1 = (struct_68 *)sub_25a50(0x20);
  v1->field_0x0 = 0x13d;
  v1->field_0x4 = 0xfff;
  v1->field_0x8 = v2._24_4_;
  v1->field_0xc = 0xfff;
  v1->field_0x11 = 0;
  return v1;
}

// Function: sub_1e310 @ 0x1e310
unsigned int sub_1e310(unsigned int a0,bool a1,unsigned int a2,char *a3,unsigned int *a4)
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  unsigned int v4; // esi
  char v5;
  unsigned int v6;
  unsigned int v7;
  
  a0 &= 0xfff;
  v5 = a3[1];
  if (!v5) {
    if (a4)
      *a4 = 0;
    return a0;
  }
  v7 = 0;
  do {
    v1 = *(unsigned int *)&a3[4];
    v2 = *(unsigned int *)&a3[8];
    if (a1) { // branch-flip
      v4 = *(unsigned int *)&a3[0xc] | 0xfffff3ff;
      v3 = ~*(unsigned int *)&a3[0xc] & 0xc00;
      if (v5 != '\x02') goto label_1e391;
label_1e445:
      v2 |= 0x49;
      v4 = ~v3;
    }
    else if (v5 != '\x02') { // branch-flip
      v4 = 0xffffffff;
      v3 = 0;
label_1e391:
      if (v5 == '\x03') {
        v2 &= a0;
        v6 = -(unsigned int)((v2 & 0x124) != 0) & 0x124;
        if (v2 & 0x92)
          v6 |= 0x92;
        if (v2 & 0x49)
          v6 |= 0x49;
        v2 |= v6;
      }
    }
    else {
      if (a0 & 0x49) {
        v3 = 0;
        goto label_1e445;
      }
      v4 = 0xffffffff;
      v3 = 0;
    }
    v5 = *a3;
    if (v1) { // branch-flip
      v2 = v2 & v4 & v1;
      if (v5 == '-') {
label_1e458:
        v7 |= v2;
        a0 &= ~v2;
        goto label_1e360;
      }
      if (v5 != '=') goto label_1e356;
      v3 |= ~v1;
      v4 = ~v3;
label_1e3ef:
      v5 = a3[0x11];
      a0 = v3 & a0 | v2;
      v7 |= v4 & 0xfff;
    }
    else {
      v2 = v2 & v4 & ~a2;
      if (v5 == '-') goto label_1e458;
      if (v5 == '=') goto label_1e3ef;
label_1e356:
      if (v5 == '+') {
        v7 |= v2;
        a0 |= v2;
      }
label_1e360:
      v5 = a3[0x11];
    }
    a3 = &a3[0x10];
    if (!v5) {
      if (a4)
        *a4 = v7;
      return a0;
    }
  } while( true );
}

// Function: sub_1e4b0 @ 0x1e4b0
void sub_1e4b0(char *a0)
{
  unsigned long v1;
  unsigned long v2; // rax
  char *v3;
  unsigned long v4;
  
  v2 = strlen(a0);
  if (v2 == 0xffffffffffffffff)
    return;
  v4 = 0;
  v3 = a0;
  do {
    v1 = v4 + 1;
    if ((((a0[v4] != '\\') || (v2 + 1 <= v4 + 4)) || (4 <= (unsigned char)(a0[v1] - 0x30U))) || ((8 <= (unsigned char)(a0[v4 + 2] - 0x30U) || (8 <= (unsigned char)(a0[v4 + 3] - 0x30U))))) {
      *v3 = a0[v4];
      v4 = v1;
      if (v2 + 1 <= v1)
        return;
    }
    else {
      *v3 = (a0[v4 + 3] - 0x30U) + ((a0[v4 + 2] - 0x30U) + (a0[v1] - 0x30U) * '\b') * '\b';
      v4 += 4;
    }
    v3 = &v3[1];
  } while( true );
}

// Function: sub_1e550 @ 0x1e550
unsigned long * sub_1e550(void)
{
  int v1;
  unsigned long *v10;
  unsigned long v11; // rax
  unsigned long *v12; // stack - 0x58
  unsigned long v13; // stack - 0x48
  char *v14; // stack - 0x50
  unsigned int v15; // stack - 0x60
  unsigned int v16; // stack - 0x64
  int v17; // stack - 0x5c
  char *v18;
  bool v19; // cl
  FILE *v2; // rax
  bool v20; // r12b
  unsigned long *v21; // stack - 0x88
  char *v3;
  char *v4; // rax
  char *v5;
  char *v6; // rax
  unsigned long *v7;
  int *v8; // rax
  void *v9; // rax
  
  v2 = fopen("/proc/self/mountinfo","re");
  if (v2) { // branch-flip
    v21 = &v12;
    v14 = NULL;
    v13 = 0;
    while (__getdelim(&v14,&v13,10,v2) != -1) {
      if ((unsigned int)(__isoc99_sscanf(v14,"%*u %*u %u:%u %n",&v16,&v15,&v17) - 2U) <= 1) {
        v18 = &v14[v17];
        v3 = strchr(v18,0x20);
        if (v3) {
          *v3 = '\0';
          v3 = &v3[1];
          v4 = strchr(v3,0x20);
          if (v4) {
            *v4 = '\0';
            v4 = strstr(&v4[1]," - ");
            if (v4) {
              v4 = &v4[3];
              v5 = strchr(v4,0x20);
              if (v5) {
                *v5 = '\0';
                v5 = &v5[1];
                v6 = strchr(v5,0x20);
                if (v6) {
                  *v6 = '\0';
                  sub_1e4b0(v5);
                  sub_1e4b0(v3);
                  sub_1e4b0(v18);
                  sub_1e4b0(v4);
                  v7 = (unsigned long *)sub_25a50(0x38);
                  *v7 = sub_25f30(v5);
                  v19 = 1;
                  v7[1] = sub_25f30(v3);
                  v7[2] = sub_25f30(v18);
                  v3 = (char *)sub_25f30(v4);
                  v7[3] = v3;
                  *(unsigned char *)&v7[5] = *(unsigned char *)&v7[5] | 4;
                  v7[4] = ((unsigned long)v15 & 0xffffff00) << 0xc | ((unsigned long)v16 & 0xfffff000) << 0x20 | (unsigned long)((v16 & 0xfff) << 8) | (unsigned long)(unsigned char)v15;
                  if ((((((strcmp(v3,"autofs")) && (strcmp(v3,"proc"))) && (strcmp(v3,"subfs"))) && ((strcmp(v3,"debugfs") && (strcmp(v3,"devpts"))))) && (((strcmp(v3,"fusectl") && ((strcmp(v3,"fuse.portal") && (strcmp(v3,"mqueue"))))) && (strcmp(v3,"rpc_pipefs"))))) && ((((strcmp(v3,"sysfs") && (strcmp(v3,"devfs"))) && (strcmp(v3,"kernfs"))) && (strcmp(v3,"ignore")))))
                    v19 = strcmp(v3,"none") == 0;
                  v18 = (char *)*v7;
                  *(unsigned char *)&v7[5] = *(unsigned char *)&v7[5] & 0xfe | v19;
                  v19 = 1;
                  if (!strchr(v18,0x3a)) {
                    if ((*v18 != '/') || (v18[1] != '/')) {
label_1e9cb:
                      v19 = 1;
                      if (strcmp(v3,"acfs")) {
                        v19 = 1;
                        if (strcmp(v3,"afs")) {
                          v19 = 1;
                          if (strcmp(v3,"coda")) {
                            v19 = 1;
                            if (strcmp(v3,"auristorfs")) {
                              v19 = 1;
                              if (strcmp(v3,"fhgfs")) {
                                v19 = 1;
                                if (strcmp(v3,"gpfs")) {
                                  v19 = 1;
                                  if (strcmp(v3,"ibrix")) {
                                    v19 = 1;
                                    if (strcmp(v3,"ocfs2")) {
                                      v19 = 1;
                                      if (strcmp(v3,"vxfs"))
                                        v19 = strcmp("-hosts",v18) == 0;
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                    else {
                      v19 = 1;
                      if (strcmp(v3,"smbfs")) {
                        v19 = 1;
                        if (strcmp(v3,"smb3")) {
                          v19 = 1;
                          if (strcmp(v3,"cifs")) goto label_1e9cb;
                        }
                      }
                    }
                  }
                  *(unsigned char *)&v7[5] = *(unsigned char *)&v7[5] & 0xfd | v19 * '\x02';
                  *v21 = v7;
                  v21 = &v7[6];
                }
              }
            }
          }
        }
      }
    }
    free(v14);
    if (*(unsigned char *)v2 & 0x20) {
      v8 = __errno_location();
      v1 = *v8;
      sub_17d40(v2);
      *v8 = v1;
      goto label_1eb37;
    }
    if (sub_17d40(v2) != -1) goto label_1e816;
  }
  else {
    v9 = setmntent("/etc/mtab","r");
    if (!v9)
      return NULL;
    v21 = &v12;
    while (v7 = getmntent(v9), v7) {
      v19 = 1;
      v18 = hasmntopt(v7,"bind");
      v10 = (unsigned long *)sub_25a50(0x38);
      v11 = sub_25f30((char *)*v7);
      v3 = (char *)v7[1];
      *v10 = v11;
      v11 = sub_25f30(v3);
      v10[2] = 0;
      v3 = (char *)v7[2];
      v10[1] = v11;
      v3 = (char *)sub_25f30(v3);
      *(unsigned char *)&v10[5] = *(unsigned char *)&v10[5] | 4;
      v10[3] = v3;
      if (((((strcmp(v3,"autofs")) && (strcmp(v3,"proc"))) && (strcmp(v3,"subfs"))) && (((strcmp(v3,"debugfs") && (strcmp(v3,"devpts"))) && ((strcmp(v3,"fusectl") && ((strcmp(v3,"fuse.portal") && (strcmp(v3,"mqueue"))))))))) && ((strcmp(v3,"rpc_pipefs") && ((((strcmp(v3,"sysfs") && (strcmp(v3,"devfs"))) && (strcmp(v3,"kernfs"))) && (strcmp(v3,"ignore"))))))) {
        v1 = strcmp(v3,"none");
        v19 = !v18 && !v1;
      }
      v18 = (char *)*v10;
      v20 = 1;
      *(unsigned char *)&v10[5] = *(unsigned char *)&v10[5] & 0xfe | v19;
      if ((!strchr(v18,0x3a)) && (((*v18 != '/' || (v18[1] != '/')) || ((strcmp(v3,"smbfs") && ((strcmp(v3,"smb3") && (strcmp(v3,"cifs"))))))))) {
        v20 = 1;
        if (((strcmp(v3,"acfs")) && (((strcmp(v3,"afs") && (strcmp(v3,"coda"))) && (strcmp(v3,"auristorfs"))))) && (((strcmp(v3,"fhgfs") && (strcmp(v3,"gpfs"))) && ((strcmp(v3,"ibrix") && ((strcmp(v3,"ocfs2") && (strcmp(v3,"vxfs")))))))))
          v20 = strcmp("-hosts",v18) == 0;
      }
      v10[4] = 0xffffffffffffffff;
      *(unsigned char *)&v10[5] = *(unsigned char *)&v10[5] & 0xfd | v20 * '\x02';
      *v21 = v10;
      v21 = &v10[6];
    }
    if (endmntent(v9)) {
label_1e816:
      *v21 = 0;
      return v12;
    }
  }
  v8 = __errno_location();
  v1 = *v8;
label_1eb37:
  *v21 = 0;
  v7 = v12;
  while (v7) {
    v10 = (unsigned long *)v7[6];
    v12 = v7;
    free((void *)*v7);
    free((void *)v7[1]);
    free((void *)v7[2]);
    if (*(unsigned char *)&v7[5] & 4)
      free((void *)v7[3]);
    free(v7);
    v7 = v10;
  }
  *v8 = v1;
  return NULL;
}

// Function: sub_1efb0 @ 0x1efb0
void sub_1efb0(struct_52 *a0)
{
  free(a0->field_0x0);
  free(a0->field_0x8);
  free(a0->field_0x10);
  if (!(a0->field_0x28 & 4)) {
    free(a0); // tail-call
    return;
  }
  free(a0->field_0x18);
  free(a0); // tail-call
}

// Function: sub_1f000 @ 0x1f000
void sub_1f000(int a0)
{
  error(dat_384b8,a0,dcgettext(NULL,"unable to record current working directory",5));
  abort(); // no-return
}

// Function: sub_1f040 @ 0x1f040
void sub_1f040(int a0)
{
  error(dat_384b8,a0,dcgettext(NULL,"failed to return to initial working directory",5));
  abort(); // no-return
}

// Function: sub_1f080 @ 0x1f080
void sub_1f080(int a0,char *a1,unsigned int a2,unsigned long a3)
{
  unsigned long v1; // rcx
  
  v1 = 0;
  if (a2 & 0x40)
    v1 = a3 & 0xffffffff;
  sub_252e0(openat(a0,a1,a2,v1));
}

// Function: sub_1f100 @ 0x1f100
DIR * sub_1f100(unsigned long a0,char *a1,unsigned int a2,int *a3)
{
  int v1;
  int v2; // eax
  DIR *v3; // rax
  int *v4; // rax
  
  v3 = NULL;
  v2 = sub_1f080(a0,a1,a2 | 0x90900);
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

// Function: sub_1f160 @ 0x1f160
long sub_1f160(void *a0)
{
  long v1; // rax
  long v2;
  unsigned long v3; // stack + 0x8
  long v4; // stack + 0x10
  long v5; // stack + 0x18
  
  if (((*(long *)((long)a0 + 0xa8)) && (!*(long *)((long)a0 + 0x30))) && (!*(char *)((long)a0 + 0xa1))) {
    if ((*(long *)((long)a0 + 200)) || (3 <= v5)) {
      v1 = v5;
      *(char *)((long)a0 + 0xd8) = 1;
      *(long *)((long)a0 + 0x30) = v5;
      *(unsigned long *)((long)a0 + 0x20) = v3;
      *(long *)((long)a0 + 0x28) = v4;
      return v1;
    }
    *(unsigned long *)((long)a0 + 200) = 1;
  }
  else {
    if (5 <= v5) {
      v1 = *(long *)((long)a0 + 0xa8) + 1;
      *(long *)((long)a0 + 0xa8) = v1;
      *(long *)((long)a0 + 0x30) = v5 + -4;
      *(long *)((long)a0 + 0x40) = v4 % 100;
      v1 = v4 * 0x346dc5d63886594b;
      *(long *)((long)a0 + 0x38) = (v4 / 100) % 100;
      *(long *)((long)a0 + 0x28) = v4 / 10000;
      return v1;
    }
    *(long *)((long)a0 + 200) = *(long *)((long)a0 + 200) + 1;
    if (3 <= v5) {
      v2 = v4 / 100;
      v1 = v4 % 100;
      goto label_1f2a4;
    }
  }
  v1 = 0;
  v2 = v4;
label_1f2a4:
  *(long *)((long)a0 + 0x48) = v2;
  *(long *)((long)a0 + 0x50) = v1;
  *(unsigned long *)((long)a0 + 0x58) = 0;
  *(unsigned long *)((long)a0 + 0x60) = 0;
  *(unsigned int *)((long)a0 + 0x1c) = 2;
  return v1;
}

// Function: sub_1f2e0 @ 0x1f2e0
unsigned int sub_1f2e0(unsigned int *a0,unsigned int *a1) // early-return
{
  unsigned int v1; // eax
  
  if ((int)a1[6] <= -1)
    return 0;
  v1 = *a0 ^ *a1 | a0[1] ^ a1[1] | a0[2] ^ a1[2] | a0[3] ^ a1[3] | a0[4] ^ a1[4] | a0[5] ^ a1[5];
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0);
}

// Function: sub_1f320 @ 0x1f320
void sub_1f320(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,char *a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13)
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
  v7 = a9;
  v8 = a10;
  v9 = a11;
  v10 = a12;
  v11 = a13;
  fputs("date: ",stderr);
  v5 = &Stack0000000000000008;
  v2 = 8;
  v6 = v3;
  v4 = 0x30;
  __vfprintf_chk(stderr,1,a8,&v2);
}

// Function: sub_1f400 @ 0x1f400
long sub_1f400(void *a0,char *a1)
{
  unsigned long *v1;
  char *v2;
  long v3;
  
  v3 = 0x368a0;
  v2 = "GMT";
  do {
    if (!strcmp(a1,v2))
      return v3;
    v2 = *(char **)(v3 + 0x10);
    v3 += 0x10;
  } while (v2);
  v3 = (long)a0 + 0xe0;
  v2 = *(char **)((long)a0 + 0xe0);
  while (v2) {
    if (!strcmp(a1,v2))
      return v3;
    v1 = (unsigned long *)(v3 + 0x10);
    v3 += 0x10;
    v2 = (char *)*v1;
  }
  v3 = 0x365a0;
  v2 = "WET";
  do {
    if (!strcmp(a1,v2))
      return v3;
    v2 = *(char **)(v3 + 0x10);
    v3 += 0x10;
  } while (v2);
  return 0;
}

// Function: sub_1f4d0 @ 0x1f4d0
unsigned long sub_1f4d0(struct_1 *a0,bool a1,long a2,long a3,long a4) // return-dupe
{
  long v1;
  bool v2;
  
  if (3 <= a3) { // branch-flip
    if (0 <= a4) goto label_1f4e5;
    v1 = a2 / 100;
    a4 = a2 % 100;
  }
  else {
    if (0 <= a4) {
label_1f4e5:
      v1 = a2 * 0x3c;
      if (a1) { // branch-flip
        v2 = SBORROW8(v1,a4);
        a4 = -a4;
      }
      else {
        v2 = SCARRY8(v1,a4);
      }
      a4 = v1 + a4;
      if (SEXT816(v1) != SEXT816(a2) * SEXT816(0x3c) || v2)
        return 0;
      goto label_1f501;
    }
    a4 = 0;
    v1 = a2;
  }
  a4 += v1 * 0x3c;
label_1f501:
  if (0xb41 <= a4 + 0x5a0U)
    return 0;
  a0->field_0x18 = (int)a4 * 0x3c;
  return 1;
}

// Function: sub_1f590 @ 0x1f590
unsigned long sub_1f590(struct_60 *a0) // ternary
{
  char v1;
  long v10;
  long v11;
  long v12;
  int v13;
  long v14;
  unsigned char *v15;
  long v16;
  unsigned long v17; // rax
  unsigned int v18; // ecx
  char v19;
  long v2;
  unsigned int v20;
  int v21;
  char *v22;
  long *v23;
  long *v24;
  char v25 [20]; // stack - 0x79
  long v26 [139];
  char v27 [12];
  char v28 [25]; // stack - 0x59
  unsigned char *v29;
  long v3;
  unsigned char *v30;
  char *v31;
  char *v32;
  int v33; // edi
  unsigned char v34;
  int v35;
  long v36;
  long v37; // r8
  int v38; // r11d
  unsigned int v39;
  long v4;
  bool v40;
  bool v41; // of
  bool v42; // of
  bool v43; // of
  bool v44; // of
  bool v45; // of
  bool v46; // of
  long v47; // stack - 0x5b0
  long v48; // stack - 0x5a8
  long v49; // stack - 0x5a0
  long v5;
  long v50; // stack - 0x598
  long v51; // stack - 0x590
  long v52; // stack - 0x580
  long v53; // stack - 0x578
  long v54; // stack - 0x568
  long v55; // stack - 0x560
  long v56; // stack - 0x558
  int v57; // stack - 0x540
  long v58; // stack - 0x518
  long v59; // stack - 0x500
  long v6;
  long v60; // stack - 0x4f8
  long v61; // stack - 0x4f0
  long v62; // stack - 0x4e8
  long v7;
  long v8;
  long v9;
  
  v35 = 0;
  v39 = 0xfffffffe;
  v18 = 0x26;
  v25[1] = 0;
  v57 = 0;
  v23 = v26;
  v22 = &v25[1];
label_1f601:
  if (v18 != 0xffffffa3) {
    if (v39 == 0xfffffffe) {
      v29 = a0->field_0x0;
      v39 = (unsigned int)*v29;
label_1fb47:
      do {
        v19 = (char)v39;
        if ('\x0e' <= v19) { // branch-flip
          if (v19 == ' ') {
label_1fcb1:
            v29 = &v29[1];
            a0->field_0x0 = v29;
            v39 = (unsigned int)*v29;
            goto label_1fb47;
          }
        }
        else if ('\b' < v19) goto label_1fcb1;
        v34 = v19 - 0x2bU & 0xfd;
        if (10 <= v39 - 0x30) { // branch-flip
          if (v34) {
            if ('[' <= v19) { // branch-flip
              if ((unsigned char)(v19 + 0x9fU) <= 0x19) {
label_1fddb:
                v30 = (unsigned char *)&v28[1];
                v15 = v30;
                goto label_1fe08;
              }
            }
            else if ('@' < v19) goto label_1fddb;
            if (v19 != '(') {
              v21 = 0;
              a0->field_0x0 = &v29[1];
              v39 = (unsigned int)*v29;
              if (!*v29) goto label_1f643;
              goto label_1fce0;
            }
            v14 = 0;
            do {
              v30 = v29;
              v29 = &v30[1];
              a0->field_0x0 = v29;
              v34 = *v30;
              if (!v34) goto label_1f968;
              v14 = (v34 != 0x28) ? v14 - (unsigned long)(v34 == 0x29) : v14 + 1; // branch-flip
            } while (v14);
            v39 = (unsigned int)v30[1];
            goto label_1fb47;
          }
        }
        else if (v34) {
          v13 = 0;
          break;
        }
        v13 = (unsigned int)(v19 != '-') * 2 + -1;
        do {
          while( true ) {
            v29 = &v29[1];
            a0->field_0x0 = v29;
            v34 = *v29;
            v39 = (unsigned int)v34;
            if ('\r' < (char)v34) break;
            if ((char)v34 <= '\b') goto label_1fba1;
          }
        } while (v34 == 0x20);
label_1fba1:
      } while (9 < (unsigned int)v34 - 0x30);
      v14 = 0;
label_1fbb5:
      v21 = v39 - 0x30;
      if (v13 == -1)
        v21 = 0x30 - v39;
      v37 = v14 + v21;
      if (!SCARRY8(v14,(long)v21)) {
        v39 = (unsigned int)v29[1];
        v30 = &v29[1];
        if (10 <= v39 - 0x30) {
          if (((v29[1] & 0xfd) != 0x2c) || (v20 = (int)(char)v29[2] - 0x30, 10 <= v20)) {
            v53 = (long)v30 - (long)a0->field_0x0;
            a0->field_0x0 = v30;
            v58 = CONCAT71(v58._1_7_,(unsigned char)((unsigned int)v13 >> 0x1f));
            v21 = 0x14 - (unsigned int)(v13 != 0);
            v39 = 0x113 - (unsigned int)(v13 != 0);
            v52 = v37;
          }
          else {
            v38 = (int)(char)v29[3];
            v29 = &v29[3];
            v33 = 8;
            v21 = v38;
            do {
              v39 = v38 - 0x30;
              v20 *= 10;
              if (v39 <= 9) {
                v38 = (int)(char)v29[1];
                v20 += v39;
                v29 = &v29[1];
                v39 = v38 - 0x30;
                v21 = v38;
              }
              v33 -= 1;
            } while (v33);
            if (v13 != -1) {
label_1fc51:
              while (v39 <= 9) {
                v30 = &v29[1];
                v29 = &v29[1];
                v39 = (int)(char)*v30 - 0x30;
              }
              if ((0 <= v13) || (!v20)) {
                a0->field_0x0 = v29;
                v21 = 0x16 - (unsigned int)(v13 != 0);
                v39 = 0x115 - (unsigned int)(v13 != 0);
                v52 = (long)(int)v20;
                v58 = v37;
                goto label_1fcee;
              }
label_1fc74:
              if (!SCARRY8(v37,-1)) {
                a0->field_0x0 = v29;
                v39 = 0x114;
                v21 = 0x15;
                v52 = (long)(int)(1000000000 - v20);
                v58 = v37 + -1;
                goto label_1fcee;
              }
              goto label_20d06;
            }
            while (v39 <= 9) {
              if ((char)v21 != '0') {
                v20 += 1;
                v39 = (int)(char)*v29 - 0x30;
                goto label_1fc51;
              }
              v21 = (int)(char)v29[1];
              v29 = &v29[1];
              v39 = v21 - 0x30;
            }
            if (v20) goto label_1fc74;
            a0->field_0x0 = v29;
            v21 = 0x15;
            v39 = 0x114;
            v52 = 0;
            v58 = v37;
          }
          goto label_1fcee;
        }
        v14 = v37 * 10;
        if (SEXT816(v14) == SEXT816(v37) * SEXT816(10)) goto label_1fd16;
      }
      goto label_1fcd6;
    }
    goto label_1f617;
  }
  goto label_1f6f0;
label_1fe08:
  do {
    v29 = &v29[1];
    if (v15 < &v28[0x14]) {
      *v15 = (unsigned char)v39;
      v15 = &v15[1];
    }
    a0->field_0x0 = v29;
    v34 = *v29;
    v39 = (unsigned int)v34;
    if ('[' <= (char)v34) { // branch-flip
      if ((unsigned char)(v34 + 0x9f) <= 0x19) goto label_1fe08;
    }
    else if ('A' <= (char)v34) goto label_1fe08;
  } while (v34 == 0x2e);
  *v15 = 0;
  v29 = v30;
  v34 = v28[1];
  while (v34) {
    if ((unsigned int)v34 - 0x61 <= 0x19)
      v34 -= 0x20;
    v15 = &v29[1];
    *v29 = v34;
    v34 = *v15;
    v29 = v15;
  }
  v31 = "AM";
  v14 = 0x36ca0;
  do {
    if (!strcmp((char *)v30,v31)) goto label_1fed3;
    v31 = *(char **)(v14 + 0x10);
    v14 += 0x10;
  } while (v31);
  v17 = strlen((char *)v30);
  v40 = 1;
  if ((v17 != 3) && (v40 = 0, v17 == 4))
    v40 = v28[4] == '.';
  v14 = 0x36b00;
  v31 = "JANUARY";
  do {
    if (v40)
      v13 = strncmp((char *)v30,v31,3);
    else {
      v13 = strcmp((char *)v30,v31);
    }
    if (!v13) goto label_1fed3;
    v31 = *(char **)(v14 + 0x10);
    v14 += 0x10;
  } while (v31);
  v14 = sub_1f400(a0,v30);
  if (v14) {
label_20ccc:
    v58 = (long)*(int *)(v14 + 0xc);
    v39 = *(unsigned int *)(v14 + 8);
    goto label_1f617;
  }
  v31 = "YEAR";
  if (!strcmp((char *)v30,"DST")) {
    v58 = 0;
    v39 = 0x103;
    goto label_1fce0;
  }
  v14 = 0x36a40;
  do {
    if (!strcmp((char *)v30,v31)) {
      v58 = (long)*(int *)(v14 + 0xc);
      v39 = *(unsigned int *)(v14 + 8);
      goto label_1f617;
    }
    v31 = *(char **)(v14 + 0x10);
    v14 += 0x10;
  } while (v31);
  v31 = &v28[v17];
  v40 = 0;
  if (*v31 == 'S') {
    v14 = 0x36a40;
    v32 = "YEAR";
    *v31 = '\0';
    do {
      if (!strcmp((char *)v30,v32)) goto label_20f63;
      v32 = *(char **)(v14 + 0x10);
      v14 += 0x10;
    } while (v32);
    *v31 = 'S';
  }
  v14 = 0x368e0;
  v31 = "TOMORROW";
  do {
    if (!strcmp((char *)v30,v31)) goto label_20f63;
    v31 = *(char **)(v14 + 0x10);
    v14 += 0x10;
  } while (v31);
  v29 = v30;
  v15 = v30;
  v34 = v28[1];
  if (v17 != 1) { // branch-flip
    if (!v28[1]) {
label_1fcd6:
      v39 = 0x3f;
      goto label_1fce0;
    }
label_2106a:
    do {
      if (v34 != 0x2e) // branch-flip
        v29 = &v29[1];
      else {
        v40 = 1;
      }
      v34 = v15[1];
      v15 = &v15[1];
      *v29 = v34;
    } while (v34);
    if ((v40) && (v14 = sub_1f400(a0,v30), v14)) {
      v58 = (long)*(int *)(v14 + 0xc);
      v39 = *(unsigned int *)(v14 + 8);
      goto label_1f617;
    }
  }
  else {
    v14 = 0x36400;
    v31 = "A";
    do {
      if (*v31 == v28[1]) goto label_20ccc;
      v31 = *(char **)(v14 + 0x10);
      v14 += 0x10;
    } while ((unsigned char *)v31);
    if (v28[1]) goto label_2106a;
  }
label_20d06:
  v21 = 2;
  v39 = 0x3f;
  goto label_1fcee;
label_1fed3:
  v58 = (long)*(int *)(v14 + 0xc);
  v39 = *(unsigned int *)(v14 + 8);
  goto label_1f617;
label_20f63:
  v58 = (long)*(int *)(v14 + 0xc);
  v39 = *(unsigned int *)(v14 + 8);
label_1f617:
  if (1 <= (int)v39) { // branch-flip
    if (v39 == 0x100) {
      v39 = 0x101;
      goto label_1f743;
    }
    if (0x116 <= (int)v39) { // branch-flip
      v18 += 2;
      v21 = 2;
    }
    else {
label_1fce0:
      v21 = (int)*(char *)((long)(int)v39 + 0x2cde0);
label_1fcee:
      v18 += v21;
    }
  }
  else {
label_1f968:
    v21 = 0;
    v39 = 0;
  }
label_1f643:
  if ((0x71 <= v18) || (*(char *)((long)(int)v18 + 0x2cba0) != v21)) {
label_1f6f0:
    v14 = (long)v35;
    v35 = (int)*(char *)(v14 + 0x2cce0);
    if (!*(char *)(v14 + 0x2cce0)) {
      if (v57 == 3) {
        if (0 < (int)v39) {
          v18 = (unsigned int)*(char *)(v14 + 0x2cd60);
          v39 = 0xfffffffe;
          goto label_1f743;
        }
        if (!v39)
          return 1;
      }
      v18 = (unsigned int)*(char *)(v14 + 0x2cd60);
label_1f743:
      do {
        if (((v18 != 0xffffffa3) && (v18 = v18 + 1, v18 <= 0x70)) && (*(char *)((long)(int)v18 + 0x2cba0) == '\x01')) {
          v19 = *(char *)((long)(int)v18 + 0x2cc20);
          v35 = (int)v19;
          if ('\x01' <= v19) goto label_1f774;
        }
        if (v22 == &v25[1])
          return 1;
        v31 = &v22[-1];
        v22 = &v22[-1];
        v23 = &v23[-7];
        v18 = (unsigned int)*(char *)((long)*v31 + 0x2cd60);
      } while( true );
    }
  }
  else {
    v19 = *(char *)((long)(int)v18 + 0x2cc20);
    v35 = (int)v19;
    if ('\x01' <= v19) {
      v39 = 0xfffffffe;
      v23[0xd] = v62;
      v57 = v57 + -1 + (unsigned int)(v57 == 0);
      v23[0xb] = v60;
      v23[0xc] = v61;
      v24 = &v23[7];
      *v24 = v58;
      v23[8] = v52;
      v23[9] = v53;
      v23[10] = v59;
      goto label_1f932;
    }
    v35 = -v35;
  }
  v37 = (long)v35;
  v19 = *(char *)(v37 + 0x2cae0);
  v24 = &v23[(long)(1 - (int)v19) * 7];
  v47 = v24[1];
  v14 = *v24;
  v51 = v24[2];
  v48 = v24[3];
  v50 = v24[4];
  v49 = v24[5];
  v13 = (int)v24[6];
  switch(v35) {
    case 4:
      v36 = *v23;
      v16 = v23[1];
      v31 = "number of seconds";
      a0->field_0xa0 = 1;
      a0->field_0x58 = v36;
      a0->field_0x60 = v16;
      goto label_1f870;
    case 7:
      a0->field_0xc8 = a0->field_0xc8 + 1;
      v31 = "datetime";
      a0->field_0xa8 = a0->field_0xa8 + 1;
      goto label_1f870;
    case 8:
      a0->field_0xc8 = a0->field_0xc8 + 1;
      v31 = "time";
      goto label_1f870;
    case 9:
      a0->field_0xb8 = a0->field_0xb8 + 1;
      v31 = "local_zone";
      goto label_1f870;
    case 10:
      a0->field_0xd0 = a0->field_0xd0 + 1;
      v31 = "zone";
      goto label_1f870;
    case 0xb:
      a0->field_0xa8 = a0->field_0xa8 + 1;
      v31 = "date";
      goto label_1f870;
    case 0xc:
      a0->field_0xb0 = a0->field_0xb0 + 1;
      v31 = "day";
      goto label_1f870;
    case 0xd:
      v31 = "relative";
      goto label_1f870;
    case 0xe:
      v31 = "number";
      goto label_1f870;
    case 0xf:
      v31 = "hybrid";
      goto label_1f870;
    case 0x12:
      a0->field_0x50 = 0;
      v36 = v23[-6];
      a0->field_0x58 = 0;
      a0->field_0x48 = v36;
      v36 = *v23;
      a0->field_0x60 = 0;
      a0->field_0x1c = (int)v36;
      break;
    case 0x13:
      v36 = v23[-6];
      v16 = v23[-0x14];
      a0->field_0x58 = 0;
      a0->field_0x60 = 0;
      a0->field_0x50 = v36;
      v36 = *v23;
      a0->field_0x48 = v16;
      a0->field_0x1c = (int)v36;
      break;
    case 0x14:
      v36 = v23[-0x14];
      v16 = v23[-0x22];
      a0->field_0x58 = v23[-7];
      v2 = v23[-6];
      a0->field_0x48 = v16;
      a0->field_0x60 = (long)(int)v2;
      v16 = *v23;
      a0->field_0x50 = v36;
      a0->field_0x1c = (int)v16;
      break;
    case 0x16:
      a0->field_0x50 = 0;
      v36 = v23[-6];
      a0->field_0x58 = 0;
      a0->field_0x48 = v36;
      a0->field_0x60 = 0;
      a0->field_0x1c = 2;
      break;
    case 0x17:
      v36 = v23[-6];
      v16 = v23[-0x14];
      a0->field_0x58 = 0;
      a0->field_0x60 = 0;
      a0->field_0x48 = v16;
      a0->field_0x50 = v36;
      a0->field_0x1c = 2;
      break;
    case 0x18:
      v36 = v23[-7];
      v16 = v23[-0x14];
      a0->field_0x1c = 2;
      v2 = v23[-0x22];
      a0->field_0x58 = v36;
      v36 = v23[-6];
      a0->field_0x48 = v2;
      a0->field_0x50 = v16;
      a0->field_0x60 = (long)(int)v36;
      break;
    case 0x1b:
      v36 = v23[-5];
      v16 = v23[-6];
      a0->field_0xd0 = a0->field_0xd0 + 1;
      v1 = *(char *)&v23[-7];
      v2 = *v23;
      if (!sub_1f4d0(a0,(unsigned char)v1,v16,v36,v2))
        return 1;
      break;
    case 0x1c:
      a0->field_0x14 = (int)*v23;
      break;
    case 0x1d:
      a0->field_0xc0 = a0->field_0xc0 + 1;
      a0->field_0x14 = 1;
      break;
    case 0x1e:
      a0->field_0x18 = (int)*v23;
      break;
    case 0x1f:
      a0->field_0x18 = 0xffff9d90;
      break;
    case 0x20:
      a0->field_0x18 = (int)v23[-7];
      goto label_20450;
    case 0x21:
      a0->field_0x18 = 0xffff9d90;
label_20450:
      v12 = v23[6];
      v35 = a0->field_0x98;
      a0->field_0x98 = (int)v12 + a0->field_0x98;
      v36 = a0->field_0x90;
      v16 = v23[5];
      a0->field_0x90 = v36 + v23[5];
      v2 = a0->field_0x88;
      v3 = v23[4];
      a0->field_0x88 = v2 + v23[4];
      v4 = a0->field_0x80;
      v5 = v23[3];
      a0->field_0x80 = v4 + v23[3];
      v6 = a0->field_0x78;
      v7 = v23[2];
      a0->field_0x78 = v6 + v23[2];
      v8 = a0->field_0x70;
      v9 = v23[1];
      a0->field_0x70 = v8 + v23[1];
      v10 = a0->field_0x68;
      v11 = *v23;
      a0->field_0x68 = v10 + *v23;
      if ((((((SCARRY8(v36,v16) || SCARRY4((int)v12,v35)) || SCARRY8(v2,v3)) || SCARRY8(v4,v5)) || SCARRY8(v6,v7)) || SCARRY8(v8,v9)) || (SCARRY8(v10,v11)))
        return 1;
      a0->field_0xa1 = 1;
      v31 = "relative";
label_1f870:
      dcgettext(NULL,v31,5);
      break;
    case 0x22:
      v36 = v23[-5];
      v16 = v23[-6];
      v1 = *(char *)&v23[-7];
      v2 = *v23;
      if (!sub_1f4d0(a0,(unsigned char)v1,v16,v36,v2))
        return 1;
      v35 = a0->field_0x18;
      v36 = v23[-0xe];
      v16 = (long)v35 + v23[-0xe];
      v21 = (int)v16;
      a0->field_0x18 = v21;
      if (v16 != v21 || SCARRY8((long)v35,v36))
        return 1;
      break;
    case 0x23:
      a0->field_0x18 = (int)*v23 + 0xe10;
      break;
    case 0x24:
      a0->field_0x18 = (int)v23[-7] + 0xe10;
      break;
    case 0x25:
      a0->field_0x8 = 0;
      a0->field_0x10 = (int)*v23;
      break;
    case 0x26:
      a0->field_0x8 = 0;
      a0->field_0x10 = (int)v23[-7];
      break;
    case 0x27:
      a0->field_0xdf = 1;
      a0->field_0x8 = v23[-7];
      a0->field_0x10 = (int)*v23;
      break;
    case 0x28:
      a0->field_0xdf = 1;
      a0->field_0x8 = v23[-6];
      a0->field_0x10 = (int)*v23;
      break;
    case 0x29:
      a0->field_0x38 = v23[-0xd];
      a0->field_0x40 = v23[1];
      break;
    case 0x2a:
      if (4 <= v23[-0x1a]) { // branch-flip
        a0->field_0x38 = v23[-0xd];
        v2 = v23[-0x1b];
        v36 = v23[-0x1a];
        v16 = v23[1];
        a0->field_0x20 = v23[-0x1c];
        a0->field_0x28 = v2;
        a0->field_0x30 = v36;
        a0->field_0x40 = v16;
      }
      else {
        v36 = v23[-0x1b];
        v16 = *v23;
        v2 = v23[1];
        a0->field_0x40 = v23[-0xd];
        a0->field_0x38 = v36;
        a0->field_0x20 = v16;
        a0->field_0x28 = v2;
        a0->field_0x30 = v23[2];
      }
      break;
    case 0x2b:
      a0->field_0x40 = v23[-0xd];
      a0->field_0x38 = v23[-7];
      v36 = v23[1];
      a0->field_0x28 = -v36;
      goto label_207f7;
    case 0x2c:
      a0->field_0x38 = v23[-0xe];
      v36 = v23[-6];
      a0->field_0x40 = -v36;
      if (v36 == -0x8000000000000000)
        return 1;
      v36 = v23[1];
      a0->field_0x28 = -v36;
label_207f7:
      if (v36 == -0x8000000000000000)
        return 1;
label_207fd:
      a0->field_0x30 = v23[2];
      break;
    case 0x2d:
      a0->field_0x38 = v23[-7];
      a0->field_0x40 = v23[1];
      break;
    case 0x2e:
      v16 = *v23;
      v2 = v23[1];
      a0->field_0x38 = v23[-0x15];
      v36 = v23[-0xd];
      a0->field_0x20 = v16;
      a0->field_0x28 = v2;
      a0->field_0x40 = v36;
      goto label_207fd;
    case 0x2f:
      a0->field_0x40 = v23[-6];
      a0->field_0x38 = *v23;
      break;
    case 0x30:
      v16 = *v23;
      v2 = v23[1];
      a0->field_0x40 = v23[-0xd];
      v36 = v23[-7];
      a0->field_0x20 = v16;
      a0->field_0x28 = v2;
      a0->field_0x38 = v36;
      a0->field_0x30 = v23[2];
      break;
    case 0x32:
      v16 = v23[-0xe];
      v2 = v23[-0xd];
      a0->field_0x30 = v23[-0xc];
      v36 = v23[-6];
      a0->field_0x20 = v16;
      a0->field_0x28 = v2;
      a0->field_0x38 = -v36;
      if (v36 == -0x8000000000000000)
        return 1;
      v36 = v23[1];
      a0->field_0x40 = -v36;
      if (v36 == -0x8000000000000000)
        return 1;
      break;
    case 0x33:
      v36 = v23[-2];
      v16 = v23[-7];
      v2 = v23[-6];
      v3 = v23[-5];
      v4 = v23[-4];
      v5 = v23[-3];
      v54 = a0->field_0x70;
      v35 = (int)v23[-1];
      v6 = a0->field_0x68;
      v21 = a0->field_0x98;
      v7 = a0->field_0x90;
      v56 = a0->field_0x80;
      v8 = a0->field_0x88;
      v55 = a0->field_0x78;
      if (0 <= (int)*v23) // branch-flip
        v40 = (((((SCARRY8(v7,v36) || SCARRY4(v21,v35)) || SCARRY8(v8,v5)) || SCARRY8(v56,v4)) || SCARRY8(v55,v3)) || SCARRY8(v54,v2)) || SCARRY8(v6,v16);
      else {
        v41 = SBORROW4(v21,v35);
        v35 = -v35;
        v42 = SBORROW8(v7,v36);
        v36 = -v36;
        v43 = SBORROW8(v8,v5);
        v5 = -v5;
        v44 = SBORROW8(v56,v4);
        v4 = -v4;
        v45 = SBORROW8(v55,v3);
        v3 = -v3;
        v46 = SBORROW8(v54,v2);
        v2 = -v2;
        v40 = SBORROW8(v6,v16);
        v16 = -v16;
        v40 = (((((v42 || v41) || v43) || v44) || v45) || v46) || v40;
      }
      v56 += v4;
      v55 += v3;
      v54 += v2;
      a0->field_0x98 = v21 + v35;
      a0->field_0x90 = v7 + v36;
      a0->field_0x80 = v56;
      a0->field_0x88 = v8 + v5;
      a0->field_0x78 = v55;
      a0->field_0x68 = v6 + v16;
      a0->field_0x70 = v54;
      if (v40)
        return 1;
      goto label_20142;
    case 0x34:
    case 0x35:
      v12 = v23[6];
      v35 = a0->field_0x98;
      a0->field_0x98 = (int)v12 + a0->field_0x98;
      v36 = a0->field_0x90;
      v16 = v23[5];
      a0->field_0x90 = v36 + v23[5];
      v2 = a0->field_0x88;
      v3 = v23[4];
      a0->field_0x88 = v2 + v23[4];
      v4 = a0->field_0x80;
      v5 = v23[3];
      a0->field_0x80 = v4 + v23[3];
      v6 = a0->field_0x78;
      v7 = v23[2];
      a0->field_0x78 = v6 + v23[2];
      v8 = a0->field_0x70;
      v9 = v23[1];
      a0->field_0x70 = v8 + v23[1];
      v10 = a0->field_0x68;
      v11 = *v23;
      a0->field_0x68 = v10 + *v23;
      if (((((SCARRY8(v36,v16) || SCARRY4((int)v12,v35)) || SCARRY8(v2,v3)) || SCARRY8(v4,v5)) || SCARRY8(v6,v7)) || SCARRY8(v8,v9))
        return 1;
      if (SCARRY8(v10,v11))
        return 1;
label_20142:
      a0->field_0xa1 = 1;
      break;
    case 0x36:
      v14 = v23[-7];
      goto label_1feec;
    case 0x37:
    case 0x4b:
      v14 = v23[-6];
label_1feec:
      v13 = 0;
      v49 = 0;
      v50 = 0;
      v48 = 0;
      v51 = 0;
      v47 = 0;
      break;
    case 0x38:
      v13 = 0;
      v14 = 1;
      v49 = 0;
      v50 = 0;
      v48 = 0;
      v51 = 0;
      v47 = 0;
      break;
    case 0x39:
      v47 = v23[-7];
      goto label_20001;
    case 0x3a:
    case 0x4c:
      v47 = v23[-6];
label_20001:
      v14 = 0;
      v13 = 0;
      v49 = 0;
      v50 = 0;
      v48 = 0;
      v51 = 0;
      break;
    case 0x3b:
      v13 = 0;
      v14 = 0;
      v49 = 0;
      v50 = 0;
      v48 = 0;
      v51 = 0;
      v47 = 1;
      break;
    case 0x3c:
      v14 = v23[-7];
      goto label_1ff2a;
    case 0x3d:
    case 0x4d:
      v14 = v23[-6];
label_1ff2a:
      v51 = v14 * *v23;
      if (SEXT816(v51) != SEXT816(v14) * SEXT816(*v23))
        return 1;
      v49 = 0;
      v50 = 0;
      v48 = 0;
      v47 = 0;
      v13 = 0;
      v14 = 0;
      break;
    case 0x3e:
    case 0x51:
      v51 = *v23;
      v14 = 0;
      v13 = 0;
      v49 = 0;
      v50 = 0;
      v48 = 0;
      v47 = 0;
      break;
    case 0x3f:
      v48 = v23[-7];
      goto label_20154;
    case 0x40:
    case 0x4e:
      v48 = v23[-6];
label_20154:
      v14 = 0;
      v13 = 0;
      v49 = 0;
      v50 = 0;
      v51 = 0;
      v47 = 0;
      break;
    case 0x41:
      v13 = 0;
      v14 = 0;
      v49 = 0;
      v50 = 0;
      v48 = 1;
      v51 = 0;
      v47 = 0;
      break;
    case 0x42:
      v50 = v23[-7];
      goto label_1ff76;
    case 0x43:
    case 0x4f:
      v50 = v23[-6];
label_1ff76:
      v14 = 0;
      v13 = 0;
      v49 = 0;
      v48 = 0;
      v51 = 0;
      v47 = 0;
      break;
    case 0x44:
      v13 = 0;
      v14 = 0;
      v49 = 0;
      v50 = 1;
      v48 = 0;
      v51 = 0;
      v47 = 0;
      break;
    case 0x45:
      v49 = v23[-7];
      goto label_2003e;
    case 0x46:
    case 0x50:
      v49 = v23[-6];
label_2003e:
      v13 = 0;
      goto label_2004b;
    case 0x47:
    case 0x48:
      v49 = v23[-7];
      v13 = (int)v23[-6];
      goto label_2004b;
    case 0x49:
      v13 = 0;
      v49 = 1;
label_2004b:
      v50 = 0;
      v14 = 0;
      v48 = 0;
      v51 = 0;
      v47 = 0;
      break;
    case 0x55:
    case 0x57:
      v47 = 0;
      v14 = v23[1];
      break;
    case 0x58:
      sub_1f160(a0);
      break;
    case 0x59:
      sub_1f160(a0);
      v12 = v23[6];
      v35 = a0->field_0x98;
      a0->field_0x98 = (int)v12 + a0->field_0x98;
      v36 = a0->field_0x90;
      v16 = v23[5];
      a0->field_0x90 = v36 + v23[5];
      v2 = a0->field_0x88;
      v3 = v23[4];
      a0->field_0x88 = v2 + v23[4];
      v4 = a0->field_0x80;
      v5 = v23[3];
      a0->field_0x80 = v4 + v23[3];
      v6 = a0->field_0x78;
      v7 = v23[2];
      a0->field_0x78 = v6 + v23[2];
      v8 = a0->field_0x70;
      v9 = v23[1];
      a0->field_0x70 = v8 + v23[1];
      v10 = a0->field_0x68;
      v11 = *v23;
      a0->field_0x68 = v10 + *v23;
      if (((((SCARRY8(v36,v16) || SCARRY4((int)v12,v35)) || SCARRY8(v2,v3)) || SCARRY8(v4,v5)) || SCARRY8(v6,v7)) || SCARRY8(v8,v9))
        return 1;
      if (SCARRY8(v10,v11))
        return 1;
      a0->field_0xa1 = 1;
      break;
    case 0x5a:
      v14 = -1;
      break;
    case 0x5b:
      v14 = v23[1];
    
  }
  v36 = (long)v19;
  v23[v36 * -7 + 8] = v47;
  v24 = &v23[v36 * -7 + 7];
  v22 = &v22[-v36];
  v23[v36 * -7 + 7] = v14;
  v23[v36 * -7 + 9] = v51;
  v23[v36 * -7 + 10] = v48;
  v23[v36 * -7 + 0xb] = v50;
  v23[v36 * -7 + 0xc] = v49;
  *(int *)&v23[v36 * -7 + 0xd] = v13;
  v14 = (long)(*(char *)(v37 + 0x2cb40) + -0x1c);
  v18 = (int)*(char *)(v14 + 0x2ccc0) + (int)*v22;
  if ((0x71 <= v18) || (*v22 != *(char *)((long)(int)v18 + 0x2cba0))) {
    v19 = *(char *)(v14 + 0x2cca0);
    v35 = (int)v19;
  }
  else {
    v19 = *(char *)((long)(int)v18 + 0x2cc20);
    v35 = (int)v19;
  }
label_1f932:
  v22[1] = v19;
  v22 = &v22[1];
  if (v27 <= v22)
    return 2;
  if (v35 == 0xc)
    return 0;
  v18 = (unsigned int)*(char *)((long)v35 + 0x2cd60);
  v23 = v24;
  goto label_1f601;
label_1fd16:
  v29 = v30;
  goto label_1fbb5;
label_1f774:
  v24 = &v23[7];
  v57 = 3;
  v23[0xd] = v62;
  *v24 = v58;
  v23[8] = v52;
  v23[9] = v53;
  v23[10] = v59;
  v23[0xb] = v60;
  v23[0xc] = v61;
  goto label_1f932;
}

// Function: sub_210e0 @ 0x210e0
unsigned int sub_210e0(struct_12 *a0,char *a1,long *a2,long *a3)
{
  char *v1;
  long v10;
  long *v11; // rax
  long v12;
  long v13; // rax
  long *v14;
  char *v15;
  long v16; // rax
  int v17; // edx
  long v18; // rdx
  char *v19;
  char v2;
  int v20; // stack - 0x258
  unsigned int v21; // stack - 0x218
  long v22; // stack - 0x2b0
  char *v23; // stack - 0x1d8
  unsigned long v24; // stack - 0x298
  char v25 [29];
  unsigned short v26; // stack - 0xc8
  char v27 [104];
  long v28 [2];
  long v29;
  int v3;
  unsigned long v30;
  char *v31;
  int v32; // eax
  bool v33;
  char *v34; // stack - 0x328
  int v35; // stack - 0x320
  unsigned int v36; // stack - 0x30c
  unsigned int v37; // stack - 0x300
  int v38; // stack - 0x2fc
  unsigned int v39; // stack - 0x290
  unsigned long v4;
  int v40; // stack - 0x28c
  int v41; // stack - 0x288
  int v42; // stack - 0x284
  int v43; // stack - 0x280
  int v44; // stack - 0x278
  long v45; // stack - 0x270
  int v46; // stack - 0x254
  int v47; // stack - 0x250
  int v48; // stack - 0x24c
  int v49; // stack - 0x248
  char v5;
  int v50; // stack - 0x244
  int v51; // stack - 0x238
  char *v52; // stack - 0x228
  unsigned int v53; // stack - 0x214
  unsigned int v54; // stack - 0x210
  int v55; // stack - 0x20c
  int v56; // stack - 0x208
  int v57; // stack - 0x204
  int v58; // stack - 0x1f8
  char *v59; // stack - 0x1e8
  unsigned int v6; // eax
  long v60; // stack - 0x1d0
  int v61; // stack - 0x1c8
  int v62; // stack - 0x1c4
  int v63; // stack - 0x1c0
  int v64; // stack - 0x1bc
  unsigned long v65; // stack - 0x1b0
  long v66; // stack - 0x1a8
  long v67; // stack - 0x1a0
  long v68; // stack - 0x198
  unsigned long v69; // stack - 0x190
  int v7;
  long v70; // stack - 0x188
  long v71; // stack - 0x180
  long v72; // stack - 0x178
  long v73; // stack - 0x170
  long v74; // stack - 0x168
  long v75; // stack - 0x160
  long v76; // stack - 0x158
  long v77; // stack - 0x150
  long v78; // stack - 0x148
  int v79; // stack - 0x140
  int v8;
  unsigned short v80; // stack - 0x138
  unsigned long v81; // stack - 0x130
  unsigned long v82; // stack - 0x128
  long v83; // stack - 0x120
  unsigned long v84; // stack - 0x118
  unsigned long v85; // stack - 0x110
  long v86; // stack - 0x108
  unsigned long v87; // stack - 0x100
  char *v88; // stack - 0xf8
  unsigned int v89; // stack - 0xf0
  int v9; // eax
  int v90; // stack - 0xec
  char *v91; // stack - 0xe8
  unsigned int v92; // stack - 0xe0
  int v93; // stack - 0xdc
  unsigned long v94; // stack - 0xd8
  char v95; // stack - 0xc6
  
  if (!a2) {
    a2 = v28;
    sub_27490(a2);
  }
  v12 = a2[1];
  v10 = *a2;
  do {
    v2 = *a1;
    if ('\x0e' <= v2) { // branch-flip
      if (v2 != ' ') goto label_2113d;
    }
    else if (v2 <= '\b') goto label_2113d;
    a1 = &a1[1];
  } while( true );
label_2113d:
  if (!strncmp(a1,"TZ=\"",4)) {
    v2 = a1[4];
    v1 = &a1[4];
    if (!v2) goto label_211a8;
    v29 = 1;
    v15 = v1;
    v5 = v2;
label_2118b:
    if (v5 == '\\') {
      v31 = &v15[1];
      v19 = &v15[1];
      v15 = v31;
      if ((*v19 != '\\') && (*v19 != '\"')) goto label_211a8;
label_2117b:
      v31 = &v15[1];
      v5 = v15[1];
      v29 += 1;
      v15 = v31;
      if (!v5) goto label_211a8;
      goto label_2118b;
    }
    if (v5 != '\"') goto label_2117b;
    v34 = NULL;
    v15 = v27;
    v31 = v15;
    if (v29 <= 100) goto label_21a9d;
    v15 = malloc(v29);
    v34 = v15;
    if (!v15) {
      v34 = NULL; // return-dupe
      v6 = 0;
      free(v34); // return-dupe
      return v6;
    }
    v2 = a1[4];
    v31 = v34;
label_21a9d:
    while (a1 = v1, v2 != '\"') {
      v19 = &v15[1];
      a1 = &a1[v2 == '\\'];
      v2 = a1[1];
      *v15 = *a1;
      v1 = &a1[1];
      v15 = v19;
    }
    *v15 = '\0';
    v14 = (long *)sub_24db0(v31);
    if (!v14) {
      v6 = 0;
      free(v34);
      return v6;
    }
    do {
      while( true ) {
        a1 = &a1[1];
        v2 = *a1;
        if (v2 <= '\r') break;
        if (v2 != ' ') goto label_21aeb;
      }
    } while ('\t' <= v2);
label_21aeb:
    if (sub_250c0(v14,a2,&v20)) goto label_211cf;
    goto label_21b07;
  }
label_211a8:
  if (!sub_250c0(a3,a2,&v20)) {
    v34 = NULL;
    v6 = 0;
    free(v34);
    return v6;
  }
  v34 = NULL;
  v14 = a3;
label_211cf:
  v35 = (int)v12;
  v72 = (long)v35;
  v66 = 0;
  if (!*a1)
    a1 = "0";
  v64 = 2;
  v32 = 0x76a700;
  v65 = (long)v50 + 0x76c;
  v44 = v51;
  v73 = 0;
  v67 = (long)(v49 + 1);
  v74 = 0;
  v68 = (long)v48;
  v75 = 0;
  v69 = (unsigned long)v47;
  v76 = 0;
  v70 = (long)v46;
  v77 = 0;
  v71 = (long)v20;
  v78 = 0;
  v79 = 0;
  v80 = 0;
  v81 = 0;
  v82 = 0;
  v83 = 0;
  v84 = 0;
  v85 = 0;
  v86 = 0;
  v87 = 0;
  v88 = v52;
  v89 = 0x10d;
  v90 = v51;
  v91 = NULL;
  v23 = a1;
  do {
    v22 = v32 + v10;
    if (SCARRY8((long)v32,v10)) {
label_21b8e:
      if (((v52) && (v91)) && (!strcmp(v52,v91))) {
        v90 = -1;
        v91 = NULL;
      }
      break;
    }
    if (((sub_250c0(v14,&v22,&v21)) && (v59)) && (v51 != v58)) {
      v91 = v59;
      v92 = 0x10d;
      v93 = v58;
      v94 = 0;
      goto label_21b8e;
    }
    v32 += 0x76a700;
  } while (v32 != 0x1da9c00);
  v6 = 0;
  v32 = sub_1f590(&v23);
  v12 = v86;
  v4 = v82;
  v29 = v81;
  if (!v32) {
    v6 = (unsigned int)(unsigned char)v80;
    if ((unsigned char)v80) { // branch-flip
      a0->field_0x0 = v71;
      a0->field_0x8 = v72;
    }
    else {
      v18 = v85 | v81 | v82;
      if ((long)(v83 + v86 | v84 | v18) <= 1) {
        if (0 <= (long)v65) { // branch-flip
          v30 = v65;
          if (v66 == 2) {
            v10 = 2000;
            if (0x45 <= (long)v65)
              v10 = 0x76c;
            v30 = v65 + v10;
          }
          v33 = 0;
          v10 = v30 - 0x76c;
          if (0x76c <= v30) { // branch-flip
            v33 = 0;
            if (v10 < 0) {
label_21be7:
              v33 = 1;
            }
          }
          else if (0 <= v10) goto label_21be7;
          v32 = (int)v10;
          if (v10 != v32)
            v33 = 1;
        }
        else {
          v32 = (int)(0xfffffffffffff894 - v65);
          v33 = 0xfffffffffffff894 - v65 != (long)v32 || SBORROW8(-0x76c,v65);
        }
        v42 = v32;
        if (!v33) {
          v17 = (int)(v67 + -1);
          v6 = 0;
          v41 = v17;
          if (v67 + -1 == (long)v17 && !SCARRY8(v67,-1)) {
            v9 = (int)v68;
            v36 = (unsigned int)(v68 != v9);
            v40 = v9;
            if (v68 == v9) {
              if ((v85) || ((!v81 && !v82 && (v80._1_1_)))) {
                v30 = v69;
                if (v64) { // branch-flip
                  if (v64 != 1) { // branch-flip
                    if (0x18 <= v69) {
label_21cce:
                      v39 = 0xffffffff;
                      v6 = 0;
                      goto label_213fd;
                    }
                  }
                  else {
                    if (0xb <= v69 - 1) {
                      v39 = 0xc;
                      goto label_21cc8;
                    }
                    v30 = (unsigned long)((int)v69 + 0xc);
                  }
label_21c5a:
                  v39 = (unsigned int)v30;
                }
                else {
                  v39 = 0;
                  if (v69 - 1 <= 10) goto label_21c5a;
label_21cc8:
                  if (v69 != 0xc) goto label_21cce;
                }
                v37 = (unsigned int)v70;
                v36 = (unsigned int)v71;
                v24 = CONCAT44(v37,v36);
                v35 = v39;
              }
              else {
                v24 = 0;
                v39 = 0;
                v72 = 0;
                v35 = 0;
                v37 = 0;
              }
              if (v18)
                v44 = -1;
              if (v83) { // branch-flip
                v38 = v62;
                v44 = v38;
              }
              else {
                v38 = v44;
              }
              v21 = v36;
              v53 = v37;
              v54 = v35;
              v43 = -1;
              v58 = v38;
              v55 = v9;
              v56 = v17;
              v57 = v32;
              v10 = sub_25170(v14,&v24);
              v6 = sub_1f2e0(&v21,&v24);
              v8 = v63;
              if ((char)v6) { // branch-flip
label_218a3:
                if ((v29) || (!v4)) {
label_218b1:
                  if ((v73 || v74) || v75) { // branch-flip
                    v10 = v42 + v73;
                    v32 = (int)v10;
                    if ((v10 == v32) && (!SCARRY8((long)v42,v73))) {
                      v10 = v41 + v74;
                      v17 = (int)v10;
                      if ((v10 == v17) && (!SCARRY8((long)v41,v74))) {
                        v10 = v40 + v75;
                        v9 = (int)v10;
                        if ((v10 == v9) && (!SCARRY8((long)v40,v75))) {
                          v6 = 0;
                          v39 = v35;
                          v24 = CONCAT44(v37,v36);
                          v44 = v38;
                          v40 = v9;
                          v41 = v17;
                          v42 = v32;
                          v10 = sub_25170(v14,&v24);
                          if (v10 == -1) goto label_213fd;
                          goto label_219b0;
                        }
                      }
                    }
                  }
                  else {
label_219b0:
                    if (v12) {
                      v12 = v63 - v45;
                      v33 = SBORROW8(v10,v12);
                      v10 -= v12;
                      v6 = 0;
                      if (SBORROW8((long)v63,v45) || v33) goto label_213fd;
                    }
                    v12 = v76 * 0xe10;
                    if ((((SEXT816(v12) == SEXT816(v76) * SEXT416(0xe10)) && (!SCARRY8(v10,v12))) && (v13 = v77 * 0x3c, SEXT816(v13) == SEXT816(v77) * SEXT816(0x3c))) && ((v16 = v10 + v12 + v13, !SCARRY8(v10 + v12,v13) && (v12 = v16 + v78, !SCARRY8(v16,v78))))) {
                      v13 = ((v79 + v72) % 1000000000 + 1000000000) % 1000000000;
                      v10 = (long)(int)(((v79 + v72) - v13) / 1000000000);
                      if (!SCARRY8(v12,v10)) {
                        v6 = 1;
                        a0->field_0x0 = v12 + v10;
                        a0->field_0x8 = v13;
                        goto label_213fd;
                      }
                    }
                  }
label_21b07:
                  v6 = 0;
                }
                else {
                  if (1 <= v60)
                    v60 -= (unsigned long)(v43 != v61);
                  v10 = v60 * 7;
                  if ((SEXT816(v10) != SEXT816(v60) * SEXT816(7)) || (v13 = (long)(((v61 - v43) + 7) % 7), v16 = v13 + v10, SCARRY8(v13,v10))) goto label_21b07;
                  v13 = (long)v40;
                  v10 = v16 + v13;
                  v40 = (int)v10;
                  v6 = 0;
                  if (v10 == v40 && !SCARRY8(v16,v13)) {
                    v44 = -1;
                    v10 = sub_25170(v14,&v24);
                    if (v10 != -1) goto label_218b1;
                  }
                }
              }
              else if (v12) {
                v26 = 0x5858;
                v95 = 0x58;
                v3 = v63 >> 0x1f;
                v7 = v63 / 0xe10;
                v7 = __sprintf_chk(v25,1,0x1b,"%c%02d",((unsigned char)v3 & 2) + 0x2b,abs(v7));
                v8 = abs(v8 % 0xe10);
                if (v8) {
                  v10 = (long)v7;
                  v25[v10] = ':';
                  v2 = (char)((v8 / 0x3c) / 10);
                  v25[v10 + 1] = v2 + '0';
                  v7 = v8 % 0x3c;
                  v1 = &v25[3];
                  v25[v10 + 2] = (char)(v8 / 0x3c) + v2 * '\xf6' + '0';
                  if (v7) {
                    v25[v10 + 3] = ':';
                    v1 = &v25[6];
                    v2 = (char)(v7 / 10);
                    v25[v10 + 4] = v2 + '0';
                    v25[v10 + 5] = (char)v7 + v2 * '\xf6' + '0';
                  }
                  v1[v10] = '\0';
                }
                v11 = (long *)sub_24db0(&v26);
                if (v11) {
                  v43 = -1;
                  v24 = CONCAT44(v37,v36);
                  v39 = v35;
                  v44 = v38;
                  v40 = v9;
                  v41 = v17;
                  v42 = v32;
                  v10 = sub_25170(v11,&v24);
                  v6 = sub_1f2e0(&v21,&v24);
                  sub_25080(v11);
                  if ((char)v6) goto label_218a3;
                }
              }
            }
          }
        }
      }
    }
  }
label_213fd:
  if (v14 != a3)
    sub_25080(v14);
  free(v34);
  return v6;
}

// Function: sub_21de0 @ 0x21de0
unsigned int sub_21de0(struct_12 *a0,char *a1,long *a2)
{
  unsigned int v1; // eax
  long *v2; // rax
  
  v1 = 0;
  v2 = (long *)sub_24db0(getenv("TZ"));
  if (v2) {
    v1 = sub_210e0(a0,a1,a2,v2);
    sub_25080(v2);
  }
  return v1;
}

// Function: sub_21e40 @ 0x21e40
void sub_21e40(char *a0)
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
          program_invocation_short_name = a0;
        }
      }
    }
    dat_38ed0 = a0;
    __progname_full = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_21ee0 @ 0x21ee0
char * sub_21ee0(char *a0,int a1) // return-dupe x3
{
  unsigned char *v1; // rax
  
  v1 = (unsigned char *)sub_27580();
  if ((*v1 & 0xdf) != 0x55) { // branch-flip
    if (((((*v1 & 0xdf) == 0x47) && ((v1[1] & 0xdf) == 0x42)) && (v1[2] == 0x31)) && (((v1[3] == 0x38 && (v1[4] == 0x30)) && ((v1[5] == 0x33 && ((v1[6] == 0x30 && (!v1[7])))))))) {
      if (*a0 == '`')
        return (char *)0x2cf43;
      return (char *)0x2cf40;
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

// Function: sub_21fc0 @ 0x21fc0
long * sub_21fc0(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_22030:
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
label_224d6:
      v2 = 0;
      goto label_22490;
    case 2:
      if (!v18) {
        v2 = 0;
        goto label_225f1;
      }
      v25 = 1;
      v2 = 0;
      v13 = 0;
      v30 = 1;
      v31 = "\'";
      break;
    case 3:
      v2 = 1;
label_22490:
      v25 = 1;
      v13 = 0;
      a4 = 2;
      v30 = 1;
      v31 = "\'";
      break;
    case 4:
      if (!v18) {
        v2 = 1;
        goto label_225f1;
      }
      goto label_224d6;
    case 5:
      if (!v18) goto label_22431;
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
          v10 = (char *)sub_21ee0("`",a4);
          v36 = (char *)v10;
        }
        v10 = (char *)dcgettext(NULL,"\'",5);
        v35 = (char *)v10;
        if (v10 == (long *)0x2a438) {
          v10 = (char *)sub_21ee0("\'",a4);
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
label_2211b:
  do {
label_2212e:
    v23 = 0;
    a1 = v14;
label_22138:
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
          if ((a3 < v23 + v30) || (v9 = memcmp(v12,v31,v30), v9)) goto label_22aa0;
          if (!v25) {
            v8 = *v12;
            v19 = (unsigned long)v8;
            v21 = v19;
            if ((char)v8 <= '?') {
              switch(v8) {
                case 0:
                  goto label_2269c;
                default:
                  goto label_22373;
                case 7:
                  goto label_22687;
                case 8:
                  goto label_22656;
                case 9:
                  goto label_22723;
                case 10:
                  goto label_2267d;
                case 0xb:
                  goto label_22750;
                case 0xc:
                  goto label_2263b;
                case 0xd:
                  goto label_2258d;
                case 0x20:
                  goto label_2275d;
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
                  goto label_22326;
                case 0x23:
                  goto label_22700;
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
                  goto label_22258;
                case 0x27:
                  goto label_22572;
                case 0x3f:
                  goto label_22531;
                
              }
            }
            v5 = v22;
            if ((char)v8 <= 'z') {
              if (v8 == 0x40) goto label_22373;
              v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
              if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_22258;
              if (!((unsigned long)v10 & 0xa4000000)) goto label_22b44;
              goto label_22a88;
            }
            if (v8 == 0x7d) goto label_22c10;
            if ((char)v8 <= '}') {
              v21 = 0x7b;
              if (v8 == 0x7b) goto label_22a6a;
              v21 = 0x7c;
              if (v8 == 0x7c) goto label_22a88;
              goto label_22373;
            }
            if (v8 != 0x7e) goto label_22373;
label_22bc4:
            if (!v23) {
              v8 = a4 == 2;
              v21 = 0x7e;
              v26 = v3;
              goto label_223e9;
            }
            v21 = 0x7e;
            goto label_22bd2;
          }
          goto label_2235c;
        }
label_22aa0:
        v8 = *v12;
        v19 = (unsigned long)v8;
        v21 = v19;
        if ('?' < (char)v8) {
          if ('z' < (char)v8) {
            if (v8 != 0x7d) { // branch-flip
              if ('~' <= (char)v8) {
                v22 = 0;
                if (v8 == 0x7e) goto label_22bc4;
                goto label_22370;
              }
              v21 = 0x7b;
              v22 = 0;
              if (v8 != 0x7b) {
                v21 = 0x7c;
                v22 = 0;
                v5 = 0;
                if (v8 != 0x7c) goto label_22373;
                goto label_22a88;
              }
            }
            else {
              v21 = 0x7d;
              v22 = 0;
            }
label_22a6a:
            if (a3 == 0xffffffffffffffff) goto label_22c1f;
label_22a78:
            v5 = v22;
            if (a3 == 1) goto label_22705;
label_22a88:
            v8 = a4 == 2;
            v26 = v3;
            v24 = 0;
            v22 = v5;
            goto label_223e9;
          }
          if (v8 == 0x40) goto label_22370;
          v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
          v22 = 0;
          if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_22258;
          v22 = 0;
          v5 = 0;
          if ((unsigned long)v10 & 0xa4000000) goto label_22a88;
label_22b44:
          if ((char)v19 == '\\') {
            if (((bool)(v2 & v25)) && (v30)) goto label_22b6a;
            v17 = 0x5c;
            goto label_22592;
          }
label_22373:
          v17 = (unsigned char)v19;
          if (v33 != 1) goto label_228f3;
label_22380:
          v10 = __ctype_b_loc();
          v21 = 1;
          v26 = (*(unsigned char *)(*v10 + 1 + v19 * 2) & 0x40) == 0;
          v27 = (bool)(v26 & v2);
          v26 = !v26;
          goto label_223ce;
        }
        switch(v8) {
          case 0:
            goto label_2268e;
          default:
label_22370:
            v22 = 0;
            goto label_22373;
          case 7:
label_22687:
            v17 = 0x61;
            goto label_22667;
          case 8:
label_22656:
            v17 = 0x62;
            goto label_22667;
          case 9:
            v22 = 0;
label_22723:
            v21 = 9;
            v17 = 0x74;
            goto label_22730;
          case 10:
label_2267d:
            v17 = 0x6e;
            break;
          case 0xb:
label_22750:
            v17 = 0x76;
            break;
          case 0xc:
label_2263b:
            v17 = 0x66;
label_22667:
            if (v25) {
              v2 = 1;
              goto label_225bf;
            }
            v24 = 0;
            goto label_222de;
          case 0xd:
label_2258d:
            v17 = 0x72;
            break;
          case 0x20:
            v22 = 0;
label_2275d:
            v21 = 0x20;
            goto label_22258;
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
            goto label_22329;
          case 0x23:
            v21 = 0x23;
            v22 = 0;
            goto label_22705;
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
label_22258:
            v8 = 0;
            goto label_22268;
          case 0x27:
            v22 = 0;
            goto label_22572;
          case 0x3f:
            v22 = 0;
            goto label_22531;
          
        }
        goto label_22592;
      }
      v17 = *v12;
      v19 = (unsigned long)v17;
      if ('?' < (char)v17) {
        if ('{' <= (char)v17) {
          if (v17 == 0x7d) {
label_22c10:
            v21 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_22a78;
label_22c1f:
            v5 = v22;
            if (a2[1]) goto label_22a88;
            goto label_22705;
          }
          if ((char)v17 <= '}') {
            v21 = 0x7b;
            if (v17 != 0x7b) {
              if (v17 == 0x7c) goto label_22326;
              goto label_22373;
            }
            goto label_22a6a;
          }
          v21 = 0x7e;
          if (v17 == 0x7e) goto label_22705;
          if (v33 == 1) goto label_22380;
label_228f3:
          v15 = 0;
          if (a3 == 0xffffffffffffffff) {
            v10 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v10;
          }
          v21 = 0;
          do {
            v19 = v23 + v21;
            v10 = (long)sub_1cd30(&v16,&a2[v19],a3 - v19,&v15);
            v7 = (long)v10;
            v24 = v26;
            if (!v10) break;
            if (v10 == (long *)0xffffffffffffffff) {
              v24 = 0;
              goto label_22e5b;
            }
            if (v10 == (long *)0xfffffffffffffffe) {
              v10 = (long *)v19;
              if (v19 >= a3) goto label_231df;
              goto label_231d5;
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
                if ((v8 <= 0x21) && (0x20000002bU >> (v19 & 0x3f) & 1)) goto label_22a2d;
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
          goto label_22e5b;
        }
        if (v17 == 0x40) goto label_22373;
        v10 = (unsigned long)(1L << (v17 + 0xbf & 0x3f));
        if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_223e0;
        if ((unsigned long)v10 & 0xa4000000) goto label_22326;
        if (v17 != 0x5c) goto label_22373;
        if (a4 == 2) {
          if (!v25) goto label_22b6a;
          goto label_22353;
        }
        if (!(bool)(v2 & v25 & v30 != 0)) {
          v21 = 0x5c;
          v17 = 0x5c;
          v22 = 0;
          goto label_22730;
        }
label_22b6a:
        v23 += 1;
        v24 = 0;
        v17 = 0x5c;
        v8 = v20;
        goto label_22b80;
      }
      switch(v17) {
        case 0:
          if (!v2) {
            v22 = 0;
            v21 = 0;
            if (v34 & 1) goto label_22802;
            goto label_2273b;
          }
label_2268e:
          if (!v25) {
            v22 = 0;
label_2269c:
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
                goto label_22413;
              }
label_22fa5:
              a0[v14] = 0x5c;
              v13 = v14;
              v20 = (bool)v8;
            }
            else if (v13 < a1) {
              v8 = v20;
              goto label_22fa5;
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
label_22fdc:
              v17 = (unsigned char)v21;
              v24 = 0;
              v13 = v14;
              v8 = v26;
              if (!v2 || v27) goto label_22413;
              goto label_22268;
            }
            v17 = 0x30;
            v24 = 0;
            v13 = v14;
            v8 = v26;
            goto label_22413;
          }
          if (a4 != 2) goto label_2235c;
          goto label_225cb;
        default:
          goto label_22373;
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
          goto label_227bf;
        case 10:
          v21 = 10;
          v17 = 0x6e;
          goto label_227bf;
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
label_227bf:
          v8 = a4 == 2 & v25;
          v22 = 0;
          if (!(bool)v8) goto label_22730;
label_22341:
          a4 = 2;
label_22353:
          if (!v28) goto label_2235c;
          goto label_225cb;
        case 0x20:
          v19 = 0x20;
          goto label_22329;
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
label_22326:
          v26 = 0;
label_22329:
          v21 = v19;
          v24 = v26;
          if ((a4 == 2) && (v25)) goto label_22341;
          goto label_223e7;
        case 0x23:
label_22700:
          v21 = 0x23;
label_22705:
          if (!v23) {
            v19 = v21;
            goto label_22329;
          }
label_22bd2:
          v8 = a4 == 2;
          v26 = v3;
          v24 = 0;
          goto label_223e9;
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
          goto label_223e0;
        case 0x27:
label_22572:
          v24 = v26;
          if (a4 != 2) {
            v8 = 0;
            v21 = 0x27;
            goto label_223e9;
          }
          if (!v25) {
            if (a1) { // branch-flip
              v14 = 0;
              v21 = a1;
              if (v32) goto label_22ffb;
            }
            else {
label_22ffb:
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
            goto label_22413;
          }
          goto label_22353;
        case 0x3f:
label_22531:
          if (a4 == 2) {
            if (v25) goto label_22353;
            v8 = 0;
            v17 = 0x3f;
            v24 = 0;
            goto label_22413;
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
                goto label_223e9;
              }
              if (v25) goto label_2235c;
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
              goto label_22fdc;
            }
          }
          v8 = 0;
          v21 = 0x3f;
          v26 = v3;
          v24 = 0;
          goto label_223e9;
        
      }
label_22730:
      if (!v2) {
label_2273b:
        v17 = (unsigned char)v21;
        v24 = 0;
        v8 = 0;
        v26 = v3;
        if (!v25) goto label_22410;
        goto label_22268;
      }
label_22592:
      v24 = 0;
      goto label_225a3;
    }
    if (!(bool)(v13 == 0 & v25 & a4 == 2)) {
      v8 = a4 == 2 & (v25 ^ 1U);
      v25 = (bool)(v25 ^ 1U);
      if ((!(bool)v8) || (v25 = (bool)v8, !v3)) {
label_2329b:
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
        if (a1 || !v32) goto label_2329b;
        v23 = v32;
        goto label_22d2b;
      }
      v13 = 0;
      a4 = 5;
      v10 = (unsigned long)__ctype_get_mb_cur_max();
      v30 = 1;
      v33 = (unsigned long)v10;
      v31 = "\"";
      if (!(v34 & 2)) goto label_2334b;
      v3 = 0;
      v20 = 0;
      v32 = 0;
      v2 = v4;
      v25 = v4;
      goto label_2212e;
    }
label_22a2d:
    if (v2) {
label_225cb:
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
label_225f1:
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
label_22d2b:
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
label_231d5:
    if (!a2[(long)v10]) break;
  }
label_231df:
  v24 = 0;
label_22e5b:
  v19 = (unsigned long)v17;
  v26 = v24;
  if (2 <= v21) {
label_22e65:
    v26 = 0;
    v21 += v23;
    v14 = v23;
    do {
      v17 = (unsigned char)v19;
      if (v27) {
        v28 = a4 == 2;
        if (v25) goto label_22353;
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
        if (v21 <= v23) goto label_222f1;
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
          goto label_22b80;
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
label_223ce:
  if (v27) {
    v24 = 0;
    v27 = v2;
    goto label_22e65;
  }
label_223e0:
  v21 = v19;
  v24 = v26;
label_223e7:
  v8 = a4 == 2;
  v26 = v3;
label_223e9:
  v17 = (unsigned char)v21;
  v27 = (bool)v8 == 0;
  v3 = v26;
  v8 = 0;
  if ((v27 && v2) || (v8 = 0, v25)) {
label_22268:
    v17 = (unsigned char)v21;
    v13 = v14;
    if (!v29) goto label_22413;
    if (!(*(unsigned int *)(v29 + (v21 >> 5) * 4) >> (v17 & 0x1f) & 1)) goto label_22413;
  }
  else {
label_22410:
    v8 = 0;
    v3 = v26;
label_22413:
    if (!v22) {
      v8 ^= 1;
      v23 += 1;
      v8 &= v20;
label_22b80:
      if ((bool)v8) {
        if (v13 < a1)
          a0[v13] = 0x27;
        if (v13 + 1 < a1)
          a0[v13 + 1] = 0x27;
        v20 = 0;
        v13 += 2;
      }
      goto label_222f1;
    }
  }
label_225a3:
  if (v25) {
label_225bf:
    if ((bool)(v2 & a4 == 2)) goto label_225cb;
label_2235c:
    v34 &= 0xfffffffd;
    v29 = 0;
    goto label_22030;
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
label_222de:
  if (v13 < a1)
    a0[v13] = 0x5c;
  v13 += 1;
  v23 += 1;
label_222f1:
  if (v13 < a1)
    a0[v13] = v17;
  v13 += 1;
  if (!v24)
    v4 = 0;
  goto label_22138;
label_22802:
  v23 += 1;
  goto label_22138;
label_2334b:
  a1 = v32;
label_22431:
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
  goto label_2211b;
}

// Function: sub_23450 @ 0x23450
char * sub_23450(unsigned int a0,char *a1,unsigned long a2,struct_0 *a3)
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
    v5 = dat_38500;
    if (dat_384f8 <= (int)a0) {
      v10 = (long)dat_384f8;
      v8 = (long)(int)((a0 - dat_384f8) + 1);
      if (dat_38500 != (unsigned long *)0x38510) // branch-flip
        v5 = (unsigned long *)sub_25d30(dat_38500,&v10,v8,0x7fffffff,0x10);
      else {
        v5 = (unsigned long *)sub_25d30(NULL,&v10,v8,0x7fffffff,0x10);
        *v5 = dat_38510;
        v5[1] = dat_38518;
      }
      dat_38500 = v5;
      memset(&v5[(long)dat_384f8 * 2],0,(v10 - dat_384f8) * 0x10);
      dat_384f8 = (int)v10;
    }
    v1 = &v5[(long)(int)a0 * 2];
    v4 = *v1;
    v7 = (char *)v1[1];
    v9 = a3->field_0x4 | 1;
    v6 = sub_21fc0(v7,v4,a1,a2,a3->field_0x0,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v4 <= v6) {
      v6 += 1;
      *v1 = v6;
      if (v7 != (char *)0x38ee0)
        free(v7);
      v7 = (char *)sub_25a90(v6);
      v3 = a3->field_0x0;
      v1[1] = (unsigned long)v7;
      sub_21fc0(v7,v6,a1,a2,v3,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *v11 = v2;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_23640 @ 0x23640
void sub_23640(void *a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = (void *)0x38fe0;
  sub_25e90(a0,0x38);
  *v2 = v1;
}

// Function: sub_23680 @ 0x23680
unsigned int sub_23680(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x38fe0;
  return *a0;
}

// Function: sub_236a0 @ 0x236a0
void sub_236a0(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x38fe0;
  *a0 = a1;
}

// Function: sub_236c0 @ 0x236c0
unsigned int sub_236c0(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0x38fe0;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_23700 @ 0x23700
unsigned int sub_23700(void *a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = (void *)0x38fe0;
  v1 = *(unsigned int *)((long)a0 + 4);
  *(unsigned int *)((long)a0 + 4) = a1;
  return v1;
}

// Function: sub_23720 @ 0x23720
void sub_23720(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0x38fe0;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_23760 @ 0x23760
void sub_23760(char *a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x38fe0;
  v2 = __errno_location();
  v1 = *v2;
  sub_21fc0(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_237e0 @ 0x237e0
char * sub_237e0(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  char *v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x38fe0;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_21fc0(NULL,0,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = (char *)sub_25a90(v3 + 1U);
  sub_21fc0(v4,v3 + 1U,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_238d0 @ 0x238d0
char * sub_238d0(char *a0,unsigned long a1,unsigned int *a2)
{
  return sub_237e0(a0,a1,NULL,a2); // tail-call
}

// Function: sub_238e0 @ 0x238e0
void sub_238e0(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_38500;
  if (2 <= dat_384f8) {
    v4 = (unsigned long *)((long)dat_38500 + 0x18);
    v1 = (unsigned long *)((long)dat_38500 + (unsigned long)(unsigned int)(dat_384f8 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0x38ee0) {
    free(*(void **)((long)v3 + 8));
    dat_38518 = 0x38ee0;
    dat_38510 = 0x100;
  }
  if (v3 == (void *)0x38510) {
    dat_384f8 = 1;
    return;
  }
  free(v3);
  dat_384f8 = 1;
  dat_38500 = (void *)0x38510;
}

// Function: sub_23980 @ 0x23980
char * sub_23980(unsigned int a0,char *a1)
{
  return sub_23450(a0,a1,0xffffffffffffffff,(struct_0 *)0x38fe0); // tail-call
}

// Function: sub_239a0 @ 0x239a0
char * sub_239a0(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_23450(a0,a1,a2); // tail-call
}

// Function: sub_239b0 @ 0x239b0
char * sub_239b0(char *a0)
{
  return sub_23450(0,a0,0xffffffffffffffff,(struct_0 *)0x38fe0); // tail-call
}

// Function: sub_239d0 @ 0x239d0
char * sub_239d0(char *a0,unsigned long a1)
{
  return sub_23450(0,a0,a1,(struct_0 *)0x38fe0); // tail-call
}

// Function: sub_239f0 @ 0x239f0
void sub_239f0(unsigned int a0,int a1,char *a2)
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
    sub_23450(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_23a90 @ 0x23a90
void sub_23a90(unsigned int a0,int a1,char *a2,unsigned long a3)
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
    sub_23450(a0,a2,a3,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_23b20 @ 0x23b20
void sub_23b20(int a0,char *a1)
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
    sub_23450(0,a1,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_23bb0 @ 0x23bb0
void sub_23bb0(int a0,char *a1,unsigned long a2)
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
    sub_23450(0,a1,a2,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_23c40 @ 0x23c40
void sub_23c40(char *a0,unsigned long a1,unsigned char a2)
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
  
  v3 = dat_38fe0;
  v4 = dat_38fe8;
  v9 = dat_39010;
  v5 = dat_38ff0;
  v6 = dat_38ff8;
  v7 = dat_39000;
  v8 = dat_39008;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a2 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a2 & 0x1f)) & 1) << (a2 & 0x1f) ^ v2;
  sub_23450(0,a0,a1,&v3);
}

// Function: sub_23ce0 @ 0x23ce0
void sub_23ce0(char *a0,unsigned char a1)
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
  
  v3 = dat_38fe0;
  v4 = dat_38fe8;
  v9 = dat_39010;
  v5 = dat_38ff0;
  v6 = dat_38ff8;
  v7 = dat_39000;
  v8 = dat_39008;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a1 & 0x1f)) & 1) << (a1 & 0x1f) ^ v2;
  sub_23450(0,a0,0xffffffffffffffff,&v3);
}

// Function: sub_23d80 @ 0x23d80
void sub_23d80(char *a0)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_38fe0;
  v3 = (unsigned int)((unsigned long)dat_38fe8 >> 0x20);
  v8 = dat_39010;
  v4 = dat_38ff0;
  v5 = dat_38ff8;
  v6 = dat_39000;
  v7 = dat_39008;
  v2 = (unsigned int)dat_38fe8;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_23450(0,a0,0xffffffffffffffff,&v1);
}

// Function: sub_23e10 @ 0x23e10
void sub_23e10(char *a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_38fe0;
  v3 = (unsigned int)((unsigned long)dat_38fe8 >> 0x20);
  v8 = dat_39010;
  v4 = dat_38ff0;
  v5 = dat_38ff8;
  v6 = dat_39000;
  v7 = dat_39008;
  v2 = (unsigned int)dat_38fe8;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_23450(0,a0,a1,&v1);
}

// Function: sub_23ea0 @ 0x23ea0
void sub_23ea0(unsigned int a0,int a1,char *a2)
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
    sub_23450(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_23f40 @ 0x23f40
void sub_23f40(unsigned int a0,long a1,long a2,char *a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_38fe8;
  v4 = dat_38ff0;
  v5 = dat_38ff8;
  v8 = dat_39010;
  v2 = (unsigned int)((unsigned long)dat_38fe0 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_39000;
  v7 = dat_39008;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_23450(a0,a3,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_23fe0 @ 0x23fe0
void sub_23fe0(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_38fe8;
  v4 = dat_38ff0;
  v5 = dat_38ff8;
  v8 = dat_39010;
  v2 = (unsigned int)((unsigned long)dat_38fe0 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_39000;
  v7 = dat_39008;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_23450(a0,a3,a4,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_24080 @ 0x24080
void sub_24080(long a0,long a1,char *a2)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_38fe8;
  v4 = dat_38ff0;
  v5 = dat_38ff8;
  v8 = dat_39010;
  v2 = (unsigned int)((unsigned long)dat_38fe0 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_39000;
  v7 = dat_39008;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_23450(0,a2,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_24120 @ 0x24120
void sub_24120(long a0,long a1,char *a2,unsigned long a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_38fe8;
  v4 = dat_38ff0;
  v5 = dat_38ff8;
  v8 = dat_39010;
  v2 = (unsigned int)((unsigned long)dat_38fe0 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_39000;
  v7 = dat_39008;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_23450(0,a2,a3,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_241c0 @ 0x241c0
char * sub_241c0(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_23450(a0,a1,a2); // tail-call
}

// Function: sub_241d0 @ 0x241d0
char * sub_241d0(char *a0,unsigned long a1)
{
  return sub_23450(0,a0,a1,(struct_0 *)0x384c0); // tail-call
}

// Function: sub_241f0 @ 0x241f0
char * sub_241f0(unsigned int a0,char *a1)
{
  return sub_23450(a0,a1,0xffffffffffffffff,(struct_0 *)0x384c0); // tail-call
}

// Function: sub_24210 @ 0x24210
char * sub_24210(char *a0)
{
  return sub_23450(0,a0,0xffffffffffffffff,(struct_0 *)0x384c0); // tail-call
}

// Function: sub_24230 @ 0x24230
int sub_24230(struct_8 *a0)
{
  int v1; // eax
  char *v2; // rax
  
  a0->field_0x8 = 0;
  v1 = sub_18040(".",0x80000);
  a0->field_0x0 = v1;
  if (0 <= v1)
    return 0;
  v2 = getcwd(NULL,0);
  a0->field_0x8 = v2;
  return -(unsigned int)(v2 == NULL);
}

// Function: sub_24290 @ 0x24290
void sub_24290(struct_9 *a0)
{
  if (0 <= a0->field_0x0) {
    fchdir(a0->field_0x0); // tail-call
    return;
  }
  sub_26fd0(a0->field_0x8); // tail-call
}

// Function: sub_242c0 @ 0x242c0
void sub_242c0(struct_10 *a0)
{
  if (0 <= a0->field_0x0)
    close(a0->field_0x0);
  free(a0->field_0x8); // tail-call
}

// Function: sub_242e0 @ 0x242e0
unsigned long sub_242e0(int a0,char *a1,char **a2)
{
  int v1;
  unsigned int v2; // eax
  unsigned int v3;
  char *v4; // rax
  unsigned int *v5; // rax
  int *v6; // rax
  char v7 [4040];
  int v8 [4]; // stack - 0x1018
  unsigned long v9; // r15
  
  if ((a0 == -100) || (*a1 == '/'))
    return sub_24b60(a1,a2); // tail-call
  v4 = (char *)sub_26db0(v7,a0,a1);
  if (v4) {
    v2 = sub_24b60(v4,a2);
    v5 = (unsigned int *)__errno_location();
    v3 = *v5;
    if (v4 != v7)
      free(v4);
    if (v2 != 0xffffffff)
      return (unsigned long)v2;
    if (0x27 <= v3) { // branch-flip
      if (v3 != 0x5f) goto label_243f9;
    }
    else if (0xffffffbfffefdff9U >> ((unsigned long)v3 & 0x3f) & 1) {
label_243f9:
      *v5 = v3;
      return 0xffffffff;
    }
  }
  if (sub_24230(v8))
    sub_1f000(*__errno_location()); // no-return
  if ((0 <= a0) && (v8[0] == a0)) {
    v9 = 0xffffffff;
    sub_242c0(v8);
    *__errno_location() = 9;
  }
  else if (fchdir(a0)) { // branch-flip
    v6 = __errno_location();
    v9 = 0xffffffff;
    v1 = *v6;
    sub_242c0(v8);
    *v6 = v1;
  }
  else {
    v3 = sub_24b60(a1,a2);
    v9 = (unsigned long)v3;
    if (v3 != 0xffffffff) { // branch-flip
      if (sub_24290(v8))
        sub_1f040(*__errno_location()); // return-dupe, no-return
      sub_242c0(v8);
    }
    else {
      v6 = __errno_location();
      v1 = *v6;
      if (sub_24290(v8)) {
        sub_1f040(*__errno_location());
      }
      sub_242c0(v8);
      if (v1)
        *v6 = v1;
    }
  }
  return v9;
}

// Function: sub_24500 @ 0x24500
unsigned long sub_24500(int a0,char *a1,char **a2)
{
  int v1;
  unsigned int v2; // eax
  unsigned int v3;
  char *v4; // rax
  unsigned int *v5; // rax
  int *v6; // rax
  char v7 [4040];
  int v8 [4]; // stack - 0x1018
  unsigned long v9; // r15
  
  if ((a0 == -100) || (*a1 == '/'))
    return sub_24be0(a1,a2); // tail-call
  v4 = (char *)sub_26db0(v7,a0,a1);
  if (v4) {
    v2 = sub_24be0(v4,a2);
    v5 = (unsigned int *)__errno_location();
    v3 = *v5;
    if (v4 != v7)
      free(v4);
    if (v2 != 0xffffffff)
      return (unsigned long)v2;
    if (0x27 <= v3) { // branch-flip
      if (v3 != 0x5f) goto label_24619;
    }
    else if (0xffffffbfffefdff9U >> ((unsigned long)v3 & 0x3f) & 1) {
label_24619:
      *v5 = v3;
      return 0xffffffff;
    }
  }
  if (sub_24230(v8))
    sub_1f000(*__errno_location()); // no-return
  if ((0 <= a0) && (v8[0] == a0)) {
    v9 = 0xffffffff;
    sub_242c0(v8);
    *__errno_location() = 9;
  }
  else if (fchdir(a0)) { // branch-flip
    v6 = __errno_location();
    v9 = 0xffffffff;
    v1 = *v6;
    sub_242c0(v8);
    *v6 = v1;
  }
  else {
    v3 = sub_24be0(a1,a2);
    v9 = (unsigned long)v3;
    if (v3 != 0xffffffff) { // branch-flip
      if (sub_24290(v8))
        sub_1f040(*__errno_location()); // return-dupe, no-return
      sub_242c0(v8);
    }
    else {
      v6 = __errno_location();
      v1 = *v6;
      if (sub_24290(v8)) {
        sub_1f040(*__errno_location());
      }
      sub_242c0(v8);
      if (v1)
        *v6 = v1;
    }
  }
  return v9;
}

// Function: sub_24720 @ 0x24720
unsigned long sub_24720(int a0,char *a1,char *a2)
{
  unsigned int v1;
  int v2;
  unsigned int v3; // eax
  char *v4; // rax
  unsigned int *v5; // rax
  int *v6; // rax
  char v7 [4040];
  int v8 [4]; // stack - 0x1018
  
  if ((a0 == -100) || (*a1 == '/'))
    return setfilecon(a1,a2); // tail-call
  v4 = (char *)sub_26db0(v7,a0,a1);
  if (!v4) goto label_247d8;
  v3 = setfilecon(v4,a2);
  v5 = (unsigned int *)__errno_location();
  v1 = *v5;
  if (v4 != v7)
    free(v4);
  if (v3 != 0xffffffff) {
    return (unsigned long)v3; // return-dupe
  }
  if (0x27 <= v1) { // branch-flip
    if (v1 != 0x5f) goto label_248d9;
  }
  else if (0xffffffbfffefdff9U >> ((unsigned long)v1 & 0x3f) & 1) {
label_248d9:
    *v5 = v1;
    return (unsigned long)v3;
  }
label_247d8:
  if (sub_24230(v8))
    sub_1f000(*__errno_location()); // no-return
  if ((0 <= a0) && (v8[0] == a0)) {
    v3 = 0xffffffff;
    sub_242c0(v8);
    *__errno_location() = 9;
  }
  else if (fchdir(a0)) { // branch-flip
    v6 = __errno_location();
    v3 = 0xffffffff;
    v2 = *v6;
    sub_242c0(v8);
    *v6 = v2;
  }
  else {
    v3 = setfilecon(a1,a2);
    if (v3 != 0xffffffff) { // branch-flip
      if (sub_24290(v8))
        sub_1f040(*__errno_location()); // return-dupe, no-return
      sub_242c0(v8);
    }
    else {
      v6 = __errno_location();
      v2 = *v6;
      if (sub_24290(v8)) {
        sub_1f040(*__errno_location());
      }
      sub_242c0(v8);
      if (v2)
        *v6 = v2;
    }
  }
  return (unsigned long)v3;
}

// Function: sub_24940 @ 0x24940
unsigned long sub_24940(int a0,char *a1,char *a2)
{
  unsigned int v1;
  int v2;
  unsigned int v3; // eax
  char *v4; // rax
  unsigned int *v5; // rax
  int *v6; // rax
  char v7 [4040];
  int v8 [4]; // stack - 0x1018
  
  if ((a0 == -100) || (*a1 == '/'))
    return lsetfilecon(a1,a2); // tail-call
  v4 = (char *)sub_26db0(v7,a0,a1);
  if (!v4) goto label_249f8;
  v3 = lsetfilecon(v4,a2);
  v5 = (unsigned int *)__errno_location();
  v1 = *v5;
  if (v4 != v7)
    free(v4);
  if (v3 != 0xffffffff) {
    return (unsigned long)v3; // return-dupe
  }
  if (0x27 <= v1) { // branch-flip
    if (v1 != 0x5f) goto label_24af9;
  }
  else if (0xffffffbfffefdff9U >> ((unsigned long)v1 & 0x3f) & 1) {
label_24af9:
    *v5 = v1;
    return (unsigned long)v3;
  }
label_249f8:
  if (sub_24230(v8))
    sub_1f000(*__errno_location()); // no-return
  if ((0 <= a0) && (v8[0] == a0)) {
    v3 = 0xffffffff;
    sub_242c0(v8);
    *__errno_location() = 9;
  }
  else if (fchdir(a0)) { // branch-flip
    v6 = __errno_location();
    v3 = 0xffffffff;
    v2 = *v6;
    sub_242c0(v8);
    *v6 = v2;
  }
  else {
    v3 = lsetfilecon(a1,a2);
    if (v3 != 0xffffffff) { // branch-flip
      if (sub_24290(v8))
        sub_1f040(*__errno_location()); // return-dupe, no-return
      sub_242c0(v8);
    }
    else {
      v6 = __errno_location();
      v2 = *v6;
      if (sub_24290(v8)) {
        sub_1f040(*__errno_location());
      }
      sub_242c0(v8);
      if (v2)
        *v6 = v2;
    }
  }
  return (unsigned long)v3;
}

// Function: sub_24b60 @ 0x24b60
int sub_24b60(char *a0,char **a1)
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

// Function: sub_24be0 @ 0x24be0
int sub_24be0(char *a0,char **a1)
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

// Function: sub_24c60 @ 0x24c60
int sub_24c60(int a0,char **a1)
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

// Function: sub_24ce0 @ 0x24ce0
void * sub_24ce0(void *a0,unsigned long a1)
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

// Function: sub_24d20 @ 0x24d20
unsigned long sub_24d20(long *a0)
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

// Function: sub_24db0 @ 0x24db0
unsigned long * sub_24db0(char *a0)
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

// Function: sub_24e50 @ 0x24e50
unsigned long sub_24e50(long *a0,struct_61 *a1) // return-dupe
{
  struct_61 *v1;
  long *v2;
  int v3; // eax
  unsigned long v4; // rax
  long v5; // rax
  char *v6;
  
  v1 = (struct_61 *)a1->field_0x30;
  if (!v1)
    return 1;
  if ((a1 <= v1) && (v1 < &a1[1]))
    return 1;
  v6 = (char *)((long)a0 + 9);
  if (!v1->field_0x0[0]) {
    v6 = "";
label_24ee8:
    a1->field_0x30 = v6;
    return 1;
  }
  do {
    v3 = strcmp(v6,(char *)v1);
    while( true ) {
      if (!v3) goto label_24ee8;
      if ((!*v6) && ((v6 != (char *)((long)a0 + 9) || (!(char)a0[1])))) {
        v4 = strlen((char *)v1);
        if ((long)a0 + (0x80U - (long)v6) <= (long)(v4 + 1)) { // branch-flip
          v5 = sub_24db0(v1);
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
        goto label_24ee8;
      }
      v6 = &v6[strlen(v6) + 1];
      if ((*v6) || (v2 = (long *)*a0, !v2)) break;
      v6 = (char *)((long)v2 + 9);
      v3 = strcmp(v6,(char *)v1);
      a0 = v2;
    }
  } while( true );
}

// Function: sub_24f80 @ 0x24f80
long * sub_24f80(void *a0)
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
  v4 = (long *)sub_24db0(v3);
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

// Function: sub_25080 @ 0x25080
void sub_25080(long *a0) // return-dupe
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

// Function: sub_250c0 @ 0x250c0
struct_61 * sub_250c0(long *a0,void *a1,struct_61 *a2) // return-dupe x3
{
  long *v1; // rax
  
  if (!a0)
    return (struct_61 *)gmtime_r(a1,(tm *)a2); // tail-call
  v1 = (long *)sub_24f80(a0);
  if (!v1)
    return NULL;
  if ((!localtime_r(a1,(tm *)a2)) || (!sub_24e50(a0,a2))) {
    if (v1 == (long *)0x1)
      return NULL;
    sub_24d20(v1);
    return NULL;
  }
  if ((v1 != (long *)0x1) && (!sub_24d20(v1)))
    return NULL;
  return a2;
}

// Function: sub_25170 @ 0x25170
long sub_25170(long *a0,tm *a1) // return-dupe x3
{
  long *v1; // rax
  long v2; // rax
  tm v3; // stack - 0x68
  
  if (!a0)
    return timegm(a1); // tail-call
  v1 = (long *)sub_24f80(a0);
  if (!v1)
    return -1;
  v3._0_8_ = *(unsigned long *)a1;
  v3._28_4_ = -1;
  v3._8_8_ = *(unsigned long *)&a1->field_0x8;
  v3._16_8_ = *(unsigned long *)&a1->field_0x10;
  v3._32_4_ = *(unsigned int *)&a1->field_0x20;
  v2 = mktime(&v3);
  if (((int)v3._28_4_ <= -1) || (!sub_24e50(a0,&v3))) {
    if (v1 == (long *)0x1)
      return -1;
    sub_24d20(v1);
    return -1;
  }
  if ((v1 != (long *)0x1) && (!sub_24d20(v1)))
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

// Function: sub_252c0 @ 0x252c0
void sub_252c0(unsigned long a0)
{
  sub_17dd0(a0,0,3); // tail-call
}

// Function: sub_252e0 @ 0x252e0
unsigned int sub_252e0(unsigned int a0)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  
  if (3 <= a0)
    return a0;
  v2 = sub_252c0(a0);
  v3 = __errno_location();
  v1 = *v3;
  close(a0);
  *v3 = v1;
  return v2;
}

// Function: sub_25340 @ 0x25340
long sub_25340(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5)
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
  fputs("\n",a0);
  __fprintf_chk(a0,1,dcgettext(NULL,"License GPLv3+: GNU GPL version 3 or later <%s>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n",5),"https://gnu.org/licenses/gpl.html");
  fputs("\n",a0);
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
label_2546d:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_254a6:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0x2d600) + 0x2d600;
  switch(a5) {
    case 0:
      goto label_254a6;
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
      goto label_2546d;
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

// Function: sub_257c0 @ 0x257c0
void sub_257c0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_25340(); // tail-call
}

// Function: sub_257e0 @ 0x257e0
void sub_257e0(FILE *a0,char *a1,char *a2,char *a3,struct_69 *a4)
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
  sub_25340(a0,a1,a2,a3);
}

// Function: sub_25880 @ 0x25880
void sub_25880(FILE *a0,char *a1,char *a2,unsigned long a3,unsigned long a4,unsigned long a5)
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
  sub_25340(a0,a1,a2);
}

// Function: sub_25960 @ 0x25960
void sub_25960(void)
{
  fputs("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-findutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU findutils","http://www.gnu.org/software/findutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_25a10 @ 0x25a10
void sub_25a10(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_25f50(); // no-return
}

// Function: sub_25a50 @ 0x25a50
void sub_25a50(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_25f50(); // no-return
}

// Function: sub_25a70 @ 0x25a70
void sub_25a70(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_25f50(); // no-return
}

// Function: sub_25a90 @ 0x25a90
void sub_25a90(void)
{
  sub_25a50(); // tail-call
}

// Function: sub_25aa0 @ 0x25aa0
void sub_25aa0(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_25f50(); // no-return
}

// Function: sub_25ae0 @ 0x25ae0
void sub_25ae0(void *a0,unsigned long a1)
{
  if (realloc(a0,a1 | a1 == 0))
    return;
  sub_25f50(); // no-return
}

// Function: sub_25b10 @ 0x25b10
void sub_25b10(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_25f50(); // no-return
}

// Function: sub_25b50 @ 0x25b50
void sub_25b50(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!a1) || (!a2)) {
    a1 = 1;
    a2 = 1;
  }
  if (reallocarray(a0,a1,a2))
    return;
  sub_25f50(); // no-return
}

// Function: sub_25b90 @ 0x25b90
void sub_25b90(unsigned long a0,unsigned long a1)
{
  if (reallocarray(NULL,a0,a1))
    return;
  sub_25f50(); // no-return
}

// Function: sub_25bc0 @ 0x25bc0
void sub_25bc0(unsigned long a0,unsigned long a1)
{
  if ((!a0) || (!a1)) {
    a0 = 1;
    a1 = 1;
  }
  if (!reallocarray(NULL,a0,a1))
    sub_25f50(); // no-return
}

// Function: sub_25c10 @ 0x25c10
void sub_25c10(void *a0,unsigned long *a1)
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
  sub_25f50(); // no-return
}

// Function: sub_25ca0 @ 0x25ca0
void sub_25ca0(char *a0,unsigned long *a1,unsigned long a2)
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
  sub_25f50(); // no-return
}

// Function: sub_25d30 @ 0x25d30
void sub_25d30(void *a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_25dee;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_25dee:
  if (!a0)
    *a1 = 0;
  if (((a2 <= v2 - v1) || ((v2 = v1 + a2, !SCARRY8(v1,a2) && (((v2 <= a3 || (a3 <= -1)) && (v3 = v2 * a4, SEXT816(v3) == SEXT816(v2) * SEXT816(a4))))))) && ((realloc(a0,v3) || ((a0 && (!v3)))))) {
    *a1 = v2;
    return;
  }
  sub_25f50(); // no-return
}

// Function: sub_25e30 @ 0x25e30
void sub_25e30(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_25f50(); // no-return
}

// Function: sub_25e50 @ 0x25e50
void sub_25e50(unsigned long a0)
{
  sub_25e30(a0,1); // tail-call
}

// Function: sub_25e60 @ 0x25e60
void sub_25e60(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_25f50(); // no-return
}

// Function: sub_25e80 @ 0x25e80
void sub_25e80(unsigned long a0)
{
  sub_25e60(a0,1); // tail-call
}

// Function: sub_25e90 @ 0x25e90
void sub_25e90(void *a0,unsigned long a1)
{
  memcpy((void *)sub_25a50(a1),a0,a1); // tail-call
}

// Function: sub_25ec0 @ 0x25ec0
void sub_25ec0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_25a70(a1),a0,a1); // tail-call
}

// Function: sub_25ef0 @ 0x25ef0
void sub_25ef0(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_25a70(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_25f30 @ 0x25f30
void sub_25f30(char *a0)
{
  sub_25e90(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_25f50 @ 0x25f50
void sub_25f50(void)
{
  error(dat_384b8,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_25f90 @ 0x25f90
bool sub_25f90(char *a0,unsigned long *a1,double *a2,void *a3)
{
  int *v1; // rax
  char *v2; // stack - 0x38
  bool v3; // r8b
  double v4; // xmm0_qa
  
  v1 = __errno_location();
  *v1 = 0;
  v4 = (double)(*a3)(a0,&v2);
  if (v2 != a0) { // branch-flip
    if (!a1) {
      v3 = 0;
      if ((*v2) || (v3 = 1, v4 == dat_28958)) goto label_26005;
label_25ff3:
      v3 = *v1 != 0x22;
      goto label_25ffc;
    }
    v3 = 1;
    if (v4 != dat_28958) goto label_25ff3;
  }
  else {
    v3 = 0;
label_25ffc:
    if (!a1) goto label_26005;
  }
  *a1 = v2;
label_26005:
  *a2 = v4;
  return v3;
}

// Function: sub_26060 @ 0x26060
unsigned int sub_26060(unsigned char *a0,char **a1,unsigned int a2,unsigned long *a3,char *a4)
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
    __assert_fail("0 <= strtol_base && strtol_base <= 36","xstrtol.c",0x55,"xstrtoul"); // no-return
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
  v7 = strtoul((char *)a0,a1,a2);
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
    if (!a4) goto label_2611d;
    v9 = *v6;
    if (!v9) goto label_2611d;
    v10 = v7;
    if (!strchr(a4,(int)(char)v9)) goto label_26125;
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
label_26125:
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
      goto label_26324;
    case 5:
    case 0x25:
      if ((SUB168(ZEXT816(v10) * ZEXT816(v8),8)) || (v2 = ZEXT816(SUB168(ZEXT816(v10) * ZEXT816(v8),0)) * ZEXT816(v8), v7 = SUB168(v2,0), SUB168(v2,8))) goto label_262e9;
label_262b7:
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
      if (SUB168(ZEXT816(v10) * ZEXT816(v8),8)) goto label_262e9;
      break;
    case 0xb:
    case 0x2b:
      v7 = SUB168(ZEXT816(v10) * ZEXT816(v8),0);
      if (!SUB168(ZEXT816(v10) * ZEXT816(v8),8)) goto label_262b7;
label_262e9:
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
      goto label_26324;
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
      goto label_26324;
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
      goto label_26324;
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
label_26324:
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
  *a1 = (char *)&v6[v13];
  if (v6[v13])
    v14 |= 2;
label_2611d:
  *a3 = v7;
  return v14;
}

// Function: sub_26490 @ 0x26490
unsigned int sub_26490(unsigned char *a0,long *a1,unsigned int a2,unsigned long *a3,char *a4)
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
    __assert_fail("0 <= strtol_base && strtol_base <= 36","xstrtol.c",0x55,"xstrtoumax"); // no-return
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
    if (!a4) goto label_2654d;
    v9 = *v6;
    if (!v9) goto label_2654d;
    v10 = v7;
    if (!strchr(a4,(int)(char)v9)) goto label_26555;
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
label_26555:
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
      goto label_26754;
    case 5:
    case 0x25:
      if ((SUB168(ZEXT816(v10) * ZEXT816(v8),8)) || (v2 = ZEXT816(SUB168(ZEXT816(v10) * ZEXT816(v8),0)) * ZEXT816(v8), v7 = SUB168(v2,0), SUB168(v2,8))) goto label_26719;
label_266e7:
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
      if (SUB168(ZEXT816(v10) * ZEXT816(v8),8)) goto label_26719;
      break;
    case 0xb:
    case 0x2b:
      v7 = SUB168(ZEXT816(v10) * ZEXT816(v8),0);
      if (!SUB168(ZEXT816(v10) * ZEXT816(v8),8)) goto label_266e7;
label_26719:
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
      goto label_26754;
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
      goto label_26754;
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
      goto label_26754;
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
label_26754:
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
label_2654d:
  *a3 = v7;
  return v14;
}

// Function: sub_268c0 @ 0x268c0
bool sub_268c0(void)
{
  long v1; // rax
  unsigned long v2; // stack - 0x18
  char *v3; // stack - 0x20
  bool v4;
  
  v4 = 0;
  v3 = NULL;
  v2 = 0;
  v1 = __getdelim(&v3,&v2,10,stdin);
  if (1 <= v1) {
    if (v3[v1 + -1] == '\n')
      v3[v1 + -1] = '\0';
    v4 = 0 < rpmatch(v3);
  }
  free(v3);
  return v4;
}

// Function: sub_26960 @ 0x26960
void sub_26960(void)
{
  exit(dat_384b8); // no-return
}

// Function: sub_26980 @ 0x26980
unsigned long sub_26980(char *a0,long *a1,void *a2,unsigned long a3)
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

// Function: sub_26a90 @ 0x26a90
long sub_26a90(char *a0,long *a1) // return-dupe
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

// Function: sub_26af0 @ 0x26af0
void sub_26af0(char *a0,char *a1,long a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (a2 != -1) // branch-flip
    v1 = dcgettext(NULL,"ambiguous argument %s for %s",5);
  else {
    v1 = dcgettext(NULL,"invalid argument %s for %s",5);
  }
  v2 = sub_241f0(1,a0);
  error(0,0,v1,sub_239f0(0,8,a1),v2); // tail-call
}

// Function: sub_26b80 @ 0x26b80
void sub_26b80(unsigned long *a0,void *a1,unsigned long a2)
{
  char *v1;
  FILE *v2;
  long v3;
  void *v4;
  
  v2 = stderr;
  v4 = NULL;
  fputs(dcgettext(NULL,"Valid arguments are:",5),v2);
  v3 = 0;
  v1 = (char *)*a0;
  while (v1) {
    if ((v3) && (!memcmp(v4,a1,a2)))
      __fprintf_chk(stderr,1,", %s",(char *)sub_24210(v1));
    else {
      __fprintf_chk(stderr,1,"\n  - %s",sub_24210(v1));
      v4 = a1;
    }
    v3 += 1;
    a1 = (void *)((long)a1 + a2);
    v1 = (char *)a0[v3];
  }
  putc(10,stderr); // tail-call
}

// Function: sub_26c80 @ 0x26c80
long sub_26c80(char *a0,char *a1,long *a2,void *a3,unsigned long a4,void *a5,char a6) // early-return
{
  char *v1;
  long v2;
  
  if (a6) { // branch-flip
    v2 = sub_26980(a1,a2,a3,a4);
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
  sub_26af0(a0,a1,v2);
  sub_26b80(a2,a3,a4);
  (*a5)();
  return -1;
}

// Function: sub_26d50 @ 0x26d50
long sub_26d50(void *a0,long *a1,void *a2,unsigned long a3) // return-dupe
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

// Function: sub_26db0 @ 0x26db0
char * sub_26db0(char *a0,int a1,char *a2) // early-return, return-dupe
{
  unsigned long v1;
  int v2; // eax
  char v3 [40];
  
  if (!*a2) {
    *a0 = '\0';
    return a0;
  }
  if (!dat_39018) {
    v2 = open("/proc/self/fd",0x90900);
    if (v2 <= -1) {
      dat_39018 = 0xffffffff;
      return NULL;
    }
    __sprintf_chk(v3,1,0x20,"/proc/self/fd/%d/../fd",v2);
    dat_39018 = (-(unsigned int)(access(v3,0) == 0) & 2) - 1;
    close(v2);
  }
  if (dat_39018 <= -1)
    return NULL;
  v1 = strlen(a2) + 0x1b;
  if ((0xfc1 <= v1) && (a0 = malloc(v1), !a0))
    return NULL;
  strcpy(&a0[__sprintf_chk(a0,1,0xffffffffffffffff,"/proc/self/fd/%d/",a1)],a2);
  return a0;
}

// Function: sub_26f10 @ 0x26f10
char * sub_26f10(char *a0)
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

// Function: sub_26f70 @ 0x26f70
void sub_26f70(char *a0)
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

// Function: sub_26fa0 @ 0x26fa0
void sub_26fa0(void)
{
  __assert_fail("! close_fail","chdir-long.c",0x40,"cdb_free"); // no-return
}

// Function: sub_26fd0 @ 0x26fd0
int sub_26fd0(char *a0)
{
  int v1;
  int v2;
  int v3; // eax
  unsigned long v4; // rax
  unsigned long v5; // rax
  char *v6; // rax
  char *v7;
  int *v8; // stack - 0x40
  
  v1 = chdir(a0);
  if (!v1)
    return 0;
  v8 = __errno_location();
  if (*v8 != 0x24)
    return v1;
  v4 = strlen(a0);
  if (!v4)
    __assert_fail("0 < len","chdir-long.c",0x7e,"chdir_long"); // no-return
  if (v4 <= 0xfff)
    __assert_fail("4096 <= len","chdir-long.c",0x7f,"chdir_long"); // no-return
  v5 = strspn(a0,"/");
  if (v5 != 2) { // branch-flip
    v1 = -100;
    v7 = a0;
    if (v5) {
      v1 = openat(-100,"/",0x10900);
      if (v1 <= -1) goto label_272c0;
      v7 = &a0[v5];
    }
  }
  else {
    v6 = memchr(&a0[3],0x2f,v4 - 3);
    if (!v6)
      return -1;
    *v6 = 0;
    v1 = openat(-100,a0,0x10900);
    *v6 = 0x2f;
    if (v1 < 0) {
label_272c0:
      v2 = *v8;
      goto label_27133;
    }
    v7 = &(&v6[1])[strspn(&v6[1],"/")];
  }
  if (*v7 == '/')
    __assert_fail("*dir != \'/\'","chdir-long.c",0xa2,"chdir_long"); // no-return
  a0 = &a0[v4];
  if (a0 < v7)
    __assert_fail("dir <= dir_end","chdir-long.c",0xa3,"chdir_long"); // no-return
  if (0x1000 <= (long)a0 - (long)v7) { // branch-flip
    do {
      v6 = memrchr(v7,0x2f,0x1000);
      if (!v6) {
        *v8 = 0x24;
        return -1;
      }
      *v6 = 0;
      if (0xfff < (long)v6 - (long)v7)
        __assert_fail("slash - dir < 4096","chdir-long.c",0xb3,"chdir_long"); // no-return
      v2 = openat(v1,v7,0x10900);
      if (v2 < 0) {
        *v6 = 0x2f;
        goto label_27123;
      }
      if ((0 <= v1) && (close(v1))) {
        sub_26fa0(); // no-return, return-dupe
      }
      *v6 = 0x2f;
      v7 = &(&v6[1])[strspn(&v6[1],"/")];
      v1 = v2;
    } while ((long)a0 - (long)v7 > 0xfff);
  }
  else {
  }
  if (a0 <= v7) { // branch-flip
    if (!fchdir(v1)) {
      if (v1 <= -1)
        return 0;
label_27227:
      if (!close(v1))
        return 0;
      sub_26fa0();
    }
label_27123:
    v2 = *v8;
    if (v1 <= -1) goto label_27133;
  }
  else {
    v3 = openat(v1,v7,0x10900);
    if (v3 < 0) goto label_27123;
    if ((0 <= v1) && (close(v1))) {
      sub_26fa0();
    }
    v1 = v3;
    if (!fchdir(v3)) goto label_27227;
    v2 = *v8;
  }
  if (close(v1))
    sub_26fa0();
label_27133:
  *v8 = v2;
  return -1;
}

// Function: sub_27370 @ 0x27370
unsigned long sub_27370(FILE *a0) // early-return x2, return-dupe
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // rax
  
  v2 = __fpending(a0);
  v1 = ferror(a0);
  v3 = sub_17d40(a0);
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

// Function: sub_273e0 @ 0x273e0
void sub_273e0(void *a0)
{
  *(unsigned long *)((long)a0 + 0x10) = 0;
  *(unsigned int *)((long)a0 + 0x18) = 0x95f616;
}

// Function: sub_27400 @ 0x27400
unsigned long sub_27400(struct_21 *a0,struct_12 *a1)
{
  unsigned long v1;
  unsigned long v2;
  long v3;
  long v4;
  
  if (a0->field_0x18 != 0x95f616)
    __assert_fail("state->magic == 9827862","cycle-check.c",0x3c,"cycle_check"); // no-return
  v2 = a0->field_0x10;
  v3 = a1->field_0x8;
  if (v2) { // branch-flip
    if ((a0->field_0x0 == v3) && (a1->field_0x0 == a0->field_0x8))
      return 1;
    v1 = v2 + 1;
    a0->field_0x10 = v1;
    if (v2 & v1)
      return 0;
    if (!v1)
      return 1;
  }
  else {
    a0->field_0x10 = 1;
  }
  v4 = a1->field_0x0;
  a0->field_0x0 = v3;
  a0->field_0x8 = v4;
  return 0;
}

// Function: sub_27490 @ 0x27490
void sub_27490(timespec *a0)
{
  clock_gettime(0,a0); // tail-call
}

// Function: sub_274a0 @ 0x274a0
timespec sub_274a0(void)
{
  timespec v1; // stack - 0x28
  
  clock_gettime(0,&v1);
  return v1;
}

// Function: sub_274f0 @ 0x274f0
bool sub_274f0(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_27940(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_27580 @ 0x27580
char * sub_27580(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_275c0 @ 0x275c0
long sub_275c0(unsigned long a0)
{
  void *v1; // rax
  long v2; // rax
  long v3; // rdx
  unsigned long v4; // r8
  
  v4 = a0 + 0x20;
  if ((0 <= (long)v4) && (a0 <= v4)) {
    v1 = malloc(v4);
    if (v1) {
      v3 = (((long)v1 + 0x10U & 0xffffffffffffffe0) - (long)v1) + 0x10;
      v2 = (long)v1 + v3;
      *(char *)(v2 + -1) = (char)v3;
      return v2;
    }
    return 0;
  }
  return 0;
}

// Function: sub_27610 @ 0x27610
void sub_27610(void *a0)
{
  if ((unsigned long)a0 & 0xf)
    abort(); // no-return
  if (!((unsigned long)a0 & 0x10))
    return;
  free((void *)((long)a0 - (unsigned long)*(unsigned char *)((long)a0 + -1))); // tail-call
}

// Function: sub_27640 @ 0x27640
int sub_27640(unsigned int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (0 <= v1)
    return v1;
  return (unsigned long)(iswcntrl(a0) == 0);
}

// Function: sub_27670 @ 0x27670
void sub_27670(struct_70 *a0,struct_71 *a1) // return-dupe
{
  char v1;
  long v2;
  struct_71 *v3;
  
  v3 = (struct_71 *)a1->field_0x0;
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

// Function: sub_276d0 @ 0x276d0
unsigned int sub_276d0(unsigned char a0)
{
  return *(unsigned int *)((unsigned long)(a0 >> 5) * 4 + 0x2d9e0) >> (a0 & 0x1f) & 1;
}

// Function: sub_276f0 @ 0x276f0
unsigned long sub_276f0(unsigned char *a0)
{
  bool v1;
  char v10; // stack - 0x58
  unsigned char *v2;
  unsigned long v3; // rax
  mbstate_t v4; // stack - 0x74
  int v5 [9]; // stack - 0x54
  unsigned long v6; // r12
  char v7; // stack - 0x6c
  unsigned char *v8; // stack - 0x68
  unsigned long v9; // stack - 0x60
  
  if (__ctype_get_mb_cur_max() <= 1)
    return strlen((char *)a0); // tail-call
  v6 = 0;
  v1 = 0;
  v4 = 0;
  v8 = a0;
  do {
    v7 = 0;
    if (v1) {
label_277cc:
      v3 = __ctype_get_mb_cur_max();
      v2 = v8;
      v9 = sub_1cd30(v5,v2,sub_24ce0(v8,v3),&v4);
      if (v9 != 0xffffffffffffffff) { // branch-flip
        if (v9 != 0xfffffffffffffffe) {
          if (!v9) {
            v9 = 1;
            if (*v8)
              __assert_fail("*iter->cur.ptr == \'\\0\'","mbuiter.h",0xab,"mbuiter_multi_next"); // no-return
            if (v5[0])
              __assert_fail("iter->cur.wc == 0","mbuiter.h",0xac,"mbuiter_multi_next"); // no-return
          }
          v10 = 1;
          if (mbsinit(&v4))
            v1 = 0;
          goto label_277a8;
        }
        v9 = strlen((char *)v8);
        v10 = 0;
      }
      else {
        v9 = 1;
        v10 = 0;
      }
    }
    else {
      if (!(*(unsigned int *)((unsigned long)(*v8 >> 5) * 4 + 0x2d9e0) >> (*v8 & 0x1f) & 1)) {
        if (!mbsinit(&v4))
          __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
        v1 = 1;
        goto label_277cc;
      }
      v9 = 1;
      v5[0] = (int)(char)*v8;
      v10 = 1;
label_277a8:
      if (!v5[0])
        return v6;
    }
    v8 = &v8[v9];
    v6 += 1;
  } while( true );
}

// Function: sub_27940 @ 0x27940
unsigned long sub_27940(int a0,char *a1,unsigned long a2) // return-dupe x2
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

// Function: sub_279f0 @ 0x279f0
void sub_279f0(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_27a00 @ 0x27a00
void sub_27a00(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_38008); // tail-call
}

// Function: _DT_FINI @ 0x27a14
void _DT_FINI(void)
{
  return;
}
